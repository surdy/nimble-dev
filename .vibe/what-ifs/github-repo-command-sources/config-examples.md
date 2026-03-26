# Config Examples: GitHub Repos as Command Sources

---

## Before (current — local files only)

### settings.yaml

```yaml
hotkey: Super+Space
show_context_chip: true
allow_duplicates: true
allow_external_paths: true
```

### Directory layout

```
~/Library/Application Support/Nimble/
  settings.yaml
  env.yaml
  commands/
    examples/
      open-github.yaml
      search-google.yaml
      paste-greeting.yaml
      ...
```

### Sharing commands today

Manual copy:
```bash
# Copy someone's command into your config
cp ~/Downloads/search-duckduckgo.yaml \
   ~/Library/Application\ Support/Nimble/commands/
```

Or clone into a subdirectory:
```bash
cd ~/Library/Application\ Support/Nimble/commands/
git clone https://github.com/someone/nimble-commands.git community
```

---

## After (hypothetical — repo sources in settings.yaml)

### settings.yaml — with sources

```yaml
hotkey: Super+Space
show_context_chip: true
allow_duplicates: true
allow_external_paths: true

# Remote command sources
sources:
  # Public community repo — no auth needed
  - repo: surdy/nimble-commands
    path: commands/
    branch: main
    auto_update: true

  # Company private repo — PAT from env var
  - repo: acme-corp/nimble-team-commands
    path: nimble/
    branch: main
    token_env: GITHUB_TOKEN
    auto_update: true

  # Personal commands across machines — manual sync only
  - repo: jane/dotfiles
    path: nimble-commands/
    branch: main
    auto_update: false
```

### Full source schema

```yaml
sources:
  - repo: <owner>/<repo>            # required — GitHub owner/repo
    path: <string>                   # optional — subdirectory in repo (default: root)
    branch: <string>                 # optional — branch name (default: main)
    token_env: <string>              # optional — env var holding GitHub PAT (for private repos)
    auto_update: true | false        # optional — sync on launch + periodically (default: true)
    scripts: allow | deny            # optional — whether to sync script files (default: allow)
    priority: <integer>              # optional — conflict resolution order (default: 0, local always highest)
```

### Directory layout — after sync

```
~/Library/Application Support/Nimble/
  settings.yaml
  env.yaml
  commands/
    _sources/                         # ← managed by Nimble, read-only
      surdy-nimble-commands/
        commands/
          search-duckduckgo.yaml
          open-hackernews.yaml
          show-bookmarks/
            show-bookmarks.yaml
            bookmarks.tsv
      acme-corp-nimble-team-commands/
        nimble/
          paste-standup.yaml
          open-jira.yaml
          team-links/
            team-links.yaml
            links.tsv
            links.sh
      jane-dotfiles/
        nimble-commands/
          open-notes.yaml
          paste-signatures/
            paste-signatures.yaml
            signatures.tsv
    examples/                         # ← user's own commands (untouched)
      open-github.yaml
      search-google.yaml
      ...
```

### New built-in command

```
/sync                  # Pull all sources now
/sync surdy/nimble-commands   # Pull a specific source
```

---

## Command repo convention (proposed standard)

A "Nimble command repo" would follow this layout:

```
nimble-commands/
  README.md              # What's in this collection
  manifest.yaml          # Optional — metadata for the repo
  commands/
    open-hackernews.yaml
    search-duckduckgo.yaml
    show-bookmarks/
      show-bookmarks.yaml
      bookmarks.tsv
    dev-tools/
      paste-timestamp/
        paste-timestamp.yaml
        timestamp.sh
```

### manifest.yaml (optional — for future discoverability)

```yaml
name: Nimble Community Commands
description: A curated set of useful commands for developers
author: surdy
version: 1.0.0
categories:
  - search
  - developer-tools
  - productivity
min_nimble_version: 0.2.0
```

---

## Security-conscious variant: scripts denied

For users who want remote commands but not remote code execution:

```yaml
sources:
  - repo: surdy/nimble-commands
    path: commands/
    scripts: deny           # ← only sync YAML, TSV, and other data files
    auto_update: true
```

This would sync:
- `*.yaml` / `*.yml` command files ✅
- `*.tsv` list files ✅
- `*.sh`, `*.py`, `*.js`, `*.ps1` scripts ❌ (skipped)

Commands that reference scripts would fail gracefully ("script not found"), so the user knows what's missing and can decide to manually allow it.

---

## Phase 1 alternative: documented git workflow (no code changes)

### settings.yaml — unchanged

```yaml
hotkey: Super+Space
show_context_chip: true
allow_duplicates: true
allow_external_paths: true
```

### User workflow

```bash
# Subscribe to a public command repo
cd ~/Library/Application\ Support/Nimble/commands/
git clone https://github.com/surdy/nimble-commands.git community-surdy

# Subscribe to a private company repo
git clone git@github.com:acme-corp/nimble-team-commands.git team

# Update all repos
cd ~/Library/Application\ Support/Nimble/commands/
for dir in */; do
  [ -d "$dir/.git" ] && (cd "$dir" && git pull --ff-only)
done
```

### Result

```
commands/
  community-surdy/                    # ← git-managed
    .git/
    commands/
      search-duckduckgo.yaml
      ...
  team/                               # ← git-managed
    .git/
    nimble/
      paste-standup.yaml
      ...
  examples/                           # ← user's own
    open-github.yaml
    ...
```

The file watcher picks up all changes. No Nimble code changes needed. Works today.

### Nimble command to automate the pull

A user could even write a Nimble command to update their repos:

```yaml
# commands/sync-repos/sync-repos.yaml
phrase: sync repos
title: Pull latest commands from all repos
action:
  type: script_action
  config:
    script: sync.sh
    arg: none
    result_action: paste_text
```

```bash
#!/bin/sh
# commands/sync-repos/sync.sh
cd ~/Library/Application\ Support/Nimble/commands/
updated=0
for dir in */; do
  if [ -d "$dir/.git" ]; then
    (cd "$dir" && git pull --ff-only 2>/dev/null && updated=$((updated+1)))
  fi
done
echo "Updated $updated repo(s)"
```
