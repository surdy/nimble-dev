# Example Config Directory

This directory mirrors the layout of the Nimble config directory on disk. Copy its contents into your own config directory to get started with a full set of working examples.

**Config directory location (macOS):**
```
~/Library/Application Support/Nimble/
```

## Structure

```
example-config/
├── settings.yaml           # application settings (hotkey, chip, dedup)
├── env.yaml                # global user-defined environment variables for scripts
└── commands/
    └── examples/
        ├── open-google.yaml
        ├── open-github.yaml
        ├── open-reddit.yaml
        ├── open-slack.yaml
        ├── open-notes.yaml
        ├── search-google.yaml
        ├── paste-email.yaml
        ├── paste-greeting.yaml
        ├── copy-email.yaml
        ├── show-team-emails/         # static_list — command + list co-located
        │   ├── show-team-emails.yaml
        │   └── team-emails.yaml
        ├── say-hello/                # dynamic_list — command + script co-located
        │   ├── say-hello.yaml
        │   └── hello.sh
        ├── paste-timestamp/          # script_action — command + script co-located
        │   ├── paste-timestamp.yaml
        │   └── timestamp.sh
        ├── copy-uuid/
        │   ├── copy-uuid.yaml
        │   └── uuid.sh
        ├── open-morning-sites/
        │   ├── open-morning-sites.yaml
        │   └── morning-sites.sh
        ├── paste-team-emails/
        │   ├── paste-team-emails.yaml
        │   └── team-emails.sh
        ├── paste-team-emails-as-task/
        │   ├── paste-team-emails-as-task.yaml
        │   └── team-emails.sh
        └── show-user-env/            # user-defined env demo (global + sidecar + inline)
            ├── show-user-env.yaml
            ├── env.yaml              # sidecar env vars for this command
            └── user-env.sh
```

## Examples covered

| Command phrase | Action type | What it does |
|---|---|---|
| `open google` | `open_url` | Opens Google in the browser |
| `open github` | `open_url` | Opens GitHub in the browser |
| `open reddit` | `open_url` | Opens Reddit in the browser |
| `open slack` | `open_url` | Opens Slack via deep link |
| `open notes` | `open_url` | Opens an Obsidian vault via deep link |
| `open morning sites` | `script_action` | Opens GitHub, HN, and Reddit simultaneously |
| `search google <query>` | `open_url` | Searches Google with a typed query |
| `paste email` | `paste_text` | Pastes a static email address |
| `paste greeting` | `paste_text` | Pastes a multi-line greeting template |
| `paste team emails` | `script_action` | Pastes all team emails, one per line |
| `paste team emails tasks` | `script_action` | Pastes emails as Markdown task list items |
| `copy email` | `copy_text` | Copies a static email address to clipboard |
| `copy uuid` | `script_action` | Copies a fresh UUID to clipboard |
| `team emails` | `static_list` | Shows pickable list of team email addresses |
| `say hello` | `dynamic_list` | Shows a filterable list of greetings |
| `paste timestamp` | `script_action` | Pastes the current date/time |
| `show user env` | `dynamic_list` | Shows user-defined env vars (global + sidecar + inline demo) |
