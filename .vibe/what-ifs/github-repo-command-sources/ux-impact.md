# UX Impact: GitHub Repos as Command Sources

## What changes from the user's daily perspective

### 1. Command discovery goes from "write everything yourself" to "subscribe and go"

Today, every command in Nimble is either seeded from examples or hand-written by the user. With repo sources, a user could subscribe to a curated collection and immediately have dozens of useful commands — search engines, dev tools, text snippets — without writing any YAML.

This is the biggest UX win: **reducing time-to-value for new users** by orders of magnitude.

### 2. A new trust decision is introduced

Every repo subscription is a trust decision. The user is saying: "I trust this repo's author to run code on my machine." Today, users only run code they wrote themselves.

For YAML-only commands (`open_url`, `paste_text`, `copy_text`, `static_list`), the risk is low — these can only open URLs, paste text, or show lists. For commands with scripts (`dynamic_list`, `script_action`), the risk is full code execution.

Users need to understand this distinction. A clear UX indicator (like a warning during first sync) would be important:

```
⚠ surdy/nimble-commands contains 3 scripts:
  - bookmarks/bookmarks.sh
  - dev-tools/timestamp.sh  
  - dev-tools/uuid.sh
Allow these scripts to run? [Allow all / Review each / Deny scripts]
```

### 3. Commands may appear and disappear unexpectedly

With `auto_update: true`, a repo owner could push an update that:
- Changes a phrase the user relies on
- Removes a command entirely
- Adds a command that conflicts with a local one

This is the "npm left-pad" problem at a smaller scale. The user's muscle memory could break after an auto-update.

**Mitigation:** The `_sources/` directory is a local cache. If the remote disappears, the last-synced version persists. But phrase changes are harder to detect.

### 4. The commands list gets longer

A user subscribed to 3 repos with 20 commands each suddenly has 60+ commands. Typing a short prefix like `open` could show dozens of matches from different sources. This affects:

- **Scan time** — more results to look through
- **Noise** — remote commands the user rarely uses clutter the list
- **Disambiguation** — multiple commands with similar phrases from different sources

The current substring matching doesn't have a "source" or "priority" dimension. Adding one would help:
- Local commands could be visually prioritized (sorted first, or marked with a local icon)
- Remote commands could show their source repo as subtext

### 5. Debugging "where did this command come from?" becomes a question

Today, every command is a file the user placed. With sources, the user might type a phrase and wonder: "which repo did this come from? why does it do *that*?" 

A source attribution in the UI (e.g., shown as subtext or accessible via a detail view) would help:

```
Search DuckDuckGo                     ← title
surdy/nimble-commands                 ← source attribution
```

### 6. Offline experience is unchanged (if designed correctly)

If synced commands are cached locally in `_sources/`, going offline just means no updates. All previously synced commands continue to work. This is the right design — Nimble should never depend on network connectivity.

### 7. Env vars and secrets don't transfer

A remote command that uses `${JIRA_BASE_URL}` won't work unless the subscriber also sets that env var in their own `env.yaml`. Repos would need to document required env vars:

```yaml
# manifest.yaml or README.md
required_env:
  JIRA_BASE_URL: "Your company's Jira URL (e.g., https://acme.atlassian.net)"
  SLACK_TEAM: "Your Slack workspace name"
```

Without this, subscribers get broken commands with cryptic errors.

### 8. The "edit → see change" loop is broken for remote commands

Today, editing a YAML file in `commands/` triggers a 300ms reload. But `_sources/` files are read-only (overwritten on sync). If a user wants to tweak a remote command, they must:

1. Copy the YAML to their own `commands/` directory
2. Edit the copy
3. Deal with the duplicate phrase (if `allow_duplicates: false`)

This is a familiar "fork" workflow, but it's friction compared to the current "edit anything" model.

---

## Workflow scenarios

### Scenario A: New user onboarding

**Today:**
1. Install Nimble → see 10 seeded examples
2. Want more? Write YAML by hand or use the Copilot skill

**With sources:**
1. Install Nimble → see 10 seeded examples
2. Open settings → add `sources: [{repo: surdy/nimble-commands}]`
3. Restart (or `/sync`) → 30 new commands appear instantly
4. Start using them immediately

**Impact:** Dramatically faster time-to-value. The second step could even be part of onboarding.

### Scenario B: Team collaboration

**Today:**
1. Team shares commands via Slack/email/wiki
2. Each team member manually copies YAML files
3. Updates require re-sharing and re-copying

**With sources:**
1. Team maintains a private GitHub repo with company commands
2. Each member adds one line to `settings.yaml`
3. Updates happen automatically on app launch

**Impact:** Real team collaboration. The "team-commands" repo becomes a shared resource.

### Scenario C: Cross-machine sync (personal)

**Today:**
1. User has commands on work laptop and home desktop
2. Manually copies files between machines (or uses Dropbox/iCloud for the config dir)

**With sources:**
1. User puts personal commands in a private GitHub repo
2. Both machines point to the same source
3. Push from one machine → auto-syncs to the other

**Impact:** Eliminates the manual sync problem. The private repo acts as a sync server.

### Scenario D: Malicious repo (threat scenario)

1. User subscribes to a popular public command repo
2. Repo owner pushes an update containing a script that exfiltrates `~/.ssh/`
3. With `auto_update: true`, Nimble syncs the update on next launch
4. User runs a `dynamic_list` command → script executes → keys stolen

**Impact:** This is the worst-case scenario. It's no different from installing a malicious npm package, but the attack surface is the user's full home directory.

---

## Summary

| Aspect | Today (local only) | With repo sources |
|--------|-------------------|-------------------|
| Time to value | Slow (write everything) | Fast (subscribe) |
| Trust model | Self-authored only | Trust repo authors |
| Update mechanism | Manual file edits | Auto or manual sync |
| Command count | User-controlled | Can grow quickly |
| Source attribution | Always known (local) | Needs UI support |
| Offline behaviour | Always works | Works (cached) |
| Team sharing | Manual copy | Real collaboration |
| Cross-machine sync | External tools | Built-in |
| Security surface | Minimal | Significant |
| Debugging | Straightforward | "Where did this come from?" |
| Editability | Full control | Fork-to-edit |
