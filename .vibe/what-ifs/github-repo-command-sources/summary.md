# What If: GitHub Repos as Command Sources

## The Question

What if Nimble could pull commands and scripts from one or more GitHub repositories — public or private — so users could subscribe to curated command collections maintained by others (or by themselves across machines)?

---

## How It Works Today

Commands are **local files only**. They live in `~/Library/Application Support/Nimble/commands/` and are discovered by recursively walking the directory. There is no concept of a remote source, package registry, or sync mechanism.

Sharing commands today means:
1. Manually copy YAML + script files into the local config directory
2. Use the `example-config/` directory in the Nimble repo as a reference
3. Deploy the Copilot authoring skill to generate commands on-demand

---

## Design Space

### What would "pulling from a repo" mean?

There are three fundamentally different approaches:

**Approach A — Git Clone/Pull into a local subdirectory**
Nimble clones each configured repo into a managed subdirectory (e.g., `commands/_repos/<owner>-<repo>/`) and periodically pulls updates. The existing file watcher picks up changes naturally.

**Approach B — GitHub API download**
Nimble uses the GitHub Contents API (or Releases API) to download specific files/directories from repos, copying them into the local config. No git dependency required.

**Approach C — Symlink / Mount**
The user manually clones repos and Nimble is configured to watch additional directories outside the default config path. No GitHub integration in the app itself.

---

## Proposed Config Format

A new `sources` section in `settings.yaml`:

```yaml
# settings.yaml
hotkey: Super+Space
show_context_chip: true
allow_duplicates: true
allow_external_paths: true

# Remote command sources — synced on launch and periodically
sources:
  - repo: surdy/nimble-commands          # public repo — no token needed
    path: commands/                       # subdirectory within the repo (default: root)
    branch: main                         # optional (default: main)
    auto_update: true                    # pull on launch + every N hours (default: true)

  - repo: mycompany/team-commands        # private repo — requires token
    path: nimble/                         # only sync this subdirectory
    branch: main
    token_env: GITHUB_TOKEN              # env var name holding the PAT
    auto_update: true

  - repo: surdy/personal-commands        # personal dotfiles-style repo
    auto_update: false                   # manual sync only (via /sync command)
```

### Where synced commands land

```
commands/
  _sources/                              # managed by Nimble — do not edit
    surdy-nimble-commands/               # <owner>-<repo>
      commands/
        open-hackernews.yaml
        search-duckduckgo.yaml
        show-bookmarks/
          show-bookmarks.yaml
          bookmarks.tsv
    mycompany-team-commands/
      nimble/
        paste-standup.yaml
        team-links/
          team-links.yaml
          team-links.tsv
    surdy-personal-commands/
      open-notes.yaml
      paste-snippets/
        ...
  examples/                              # user's own commands (untouched)
    open-github.yaml
    ...
```

The `_sources/` directory is managed by Nimble. Files inside it are **read-only from the user's perspective** — local edits would be overwritten on the next sync.

---

## Key Findings

### 1. Security is the dominant concern

This is the single most impactful aspect. Today, every script in `commands/` runs with the user's privileges. Adding remote repos as command sources means:

- **Arbitrary code execution from the internet.** A public repo could contain a script that `rm -rf ~` or exfiltrates data. The user must trust every repo they subscribe to.
- **Supply chain attacks.** A previously-trusted repo could push a malicious update. If `auto_update: true`, the user's machine runs the new code without review.
- **Private repo token exposure.** The PAT stored in an env var grants Nimble access to read from the repo — but if the token has broader scopes (common with classic PATs), it could be a credential leak vector if Nimble's process is compromised.

**Mitigations that would be needed:**
- Script allowlisting or approval flow on first sync and on updates
- Checksum/hash pinning for known-good versions
- A "review changes" UI before applying updates
- Option to allow only YAML commands (no scripts) from remote sources
- Fine-grained token scoping guidance in docs

### 2. The git dependency question

**If Nimble shells out to `git`:** Simple to implement, leverages existing auth (SSH keys, credential helpers). But it adds a hard dependency — users without git installed can't use the feature. Also, shelling out to git in a GUI app has UX implications (progress feedback, error handling, credential prompts).

**If Nimble uses the GitHub API:** No git dependency, works on any machine. But it requires: an HTTP client in the Rust backend, pagination handling, rate limit management (60 req/hr unauthenticated, 5000/hr with token), and the API doesn't give you a clean "sync this directory" primitive — you'd need to walk the tree endpoint recursively.

**If Nimble embeds libgit2 (via git2 crate):** No shell dependency, full git operations in-process. But libgit2 is a large dependency, adds build complexity, and SSH auth is finicky to configure correctly cross-platform.

### 3. Conflict resolution with local commands

If a remote repo defines `phrase: open github` and the user also has a local `open-github.yaml` with the same phrase, who wins?

Possible rules:
- **Local always wins** — remote commands are lower priority (safest, least surprising)
- **First-loaded wins** — depends on scan order (fragile)
- **Configurable per source** — add a `priority` field (powerful but complex)

The `allow_duplicates` setting already controls this for local files. Remote sources would need to integrate with the same dedup system.

### 4. Update frequency and offline behaviour

- On launch: sync all sources (with a timeout so app startup isn't blocked)
- Periodically: sync every N hours in the background
- Offline: use the last-synced local copy (the `_sources/` directory persists)
- Manual: a `/sync` built-in command to trigger an immediate pull

The app must work fully offline — remote sources are a convenience, not a dependency.

### 5. Script executability on sync

On macOS/Linux, synced script files need `+x` permission. The GitHub API doesn't preserve Unix permissions. Git clone does (if the repo committed with the executable bit). This means:
- API approach: Nimble must `chmod +x` every script file after download, or maintain a manifest
- Git approach: permissions are preserved automatically

### 6. Private repo authentication

GitHub PATs (fine-grained or classic) are the standard approach. The config points to an **env var name** rather than storing the token directly in YAML (which would be a credential leak if settings.yaml is shared or backed up).

But this raises the question: where does the user set this env var? Options:
- Shell profile (`~/.zshrc`, `~/.bashrc`) — only works if Nimble is launched from a terminal
- `env.yaml` — **no**, this would expose the token to every script
- A dedicated `secrets.yaml` or keychain integration — more secure but more complex
- macOS Keychain / Windows Credential Store / Linux Secret Service — ideal but platform-specific

### 7. Discoverability and community

This feature implicitly creates a **package ecosystem**. Once repos can be subscribed to, people will want:
- A registry or directory of popular command repos
- A standard repo structure convention (README, manifest, categories)
- Versioning (semver tags? branches?)
- A way to preview what a repo contains before subscribing

This is a significant scope expansion beyond "pull files from GitHub."

### 8. The simpler alternative: documented git workflow

Instead of building GitHub integration into Nimble, document a workflow:

```bash
# Clone a command repo into your commands directory
cd ~/Library/Application\ Support/Nimble/commands/
git clone https://github.com/surdy/nimble-commands.git community

# Pull updates whenever you want
cd community && git pull
```

The file watcher already picks up changes. No new code needed. The user gets the same result with full control over when updates happen. Private repos work via SSH keys or credential helpers that git already manages.

The downside: it requires git knowledge and manual maintenance. But it avoids every security and UX complexity of built-in sync.

---

## Opinion

**The motivation is strong but the built-in implementation is premature.**

Sharing commands via repos is a natural and valuable pattern — it's essentially "dotfiles for your launcher." The use cases are real:
- Teams sharing company-specific commands (Jira links, Slack channels, standup templates)
- Community curating useful commands (search engines, dev tools, productivity snippets)
- Individuals syncing their own commands across machines

However, building GitHub repo sync into Nimble itself introduces:
- A significant security surface (remote code execution via scripts)
- A hard dependency on GitHub (or a generic git host abstraction)
- Auth complexity (tokens, keychains, credential stores)
- An implicit package ecosystem that needs governance

**My recommendation: start with the "documented git workflow" (Approach C) and build toward Approach A incrementally.**

1. **Phase 1 (now):** Document how to `git clone` repos into `commands/` subdirectories. The file watcher already handles it. Add a convention for "command repos" (standard README, directory layout).

2. **Phase 2 (later):** Add a `sources` config in `settings.yaml` and a `/sync` command that runs `git pull` on configured directories. This is lightweight — Nimble just orchestrates git operations the user already understands.

3. **Phase 3 (much later):** If demand warrants it, add GitHub API integration for token-based private repo access, auto-update scheduling, and a review UI for incoming changes. This is where the security investment becomes necessary.

Phase 1 costs nothing, validates the use case, and establishes conventions. Phase 2 is a small convenience layer. Phase 3 is a product feature that should only be built if Phases 1-2 prove the demand.
