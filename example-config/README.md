# Example Config Directory

This directory mirrors the layout of the Ctx config directory on disk. Copy its contents into your own config directory to get started with a full set of working examples.

**Config directory location (macOS):**
```
~/Library/Application Support/com.ctx.launcher/
```

## Structure

```
example-config/
├── commands/
│   └── examples/          # one YAML file per command
│       ├── open-google.yaml
│       ├── open-github.yaml
│       ├── open-reddit.yaml
│       ├── open-slack.yaml
│       ├── open-notes.yaml
│       ├── open-morning-sites.yaml
│       ├── search-google.yaml
│       ├── paste-email.yaml
│       ├── paste-greeting.yaml
│       ├── paste-team-emails.yaml
│       ├── paste-team-emails-as-task.yaml
│       ├── copy-email.yaml
│       ├── copy-uuid.yaml
│       ├── show-team-emails.yaml
│       ├── dynamic-list-example.yaml
│       └── script-action-example.yaml
├── lists/
│   └── team-emails.yaml   # used by show-team-emails.yaml
└── scripts/
    ├── hello.sh            # dynamic_list — filterable greeting list
    ├── timestamp.sh        # script_action — outputs current date/time
    ├── uuid.sh             # script_action — generates a random UUID
    ├── team-emails.sh      # script_action — returns list of email addresses
    └── morning-sites.sh    # script_action — returns list of URLs to open
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
