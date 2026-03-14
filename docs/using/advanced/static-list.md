# Static List

Displays a named list of items inline in the launcher the moment the typed phrase matches exactly. No `Enter` is needed to expand the list — it appears as soon as the full phrase is typed. By default, selecting an item simply dismisses the launcher. You can optionally configure an action to be performed on the selected item's value.

---

## Directory layout

A `static_list` command requires two files — the command YAML and the list YAML — living together in the same subdirectory of `commands/`:

```
commands/
  show-team-emails/
    show-team-emails.yaml      ← command YAML
    team-emails.yaml           ← list file (referenced by name)
```

The command YAML references the list file by name (without the `.yaml` extension):

```yaml
action:
  type: static_list
  config:
    list: team-emails          # resolves to team-emails.yaml in the same directory
```

---

## List file format

Each list file is a YAML array of items with a required `title` and an optional `subtext`.

**`commands/show-team-emails/team-emails.yaml`:**
```yaml
# Team email addresses
- title: Alice Smith
  subtext: alice@example.com

- title: Bob Jones
  subtext: bob@example.com

- title: Carol White
  subtext: carol@example.com
```

| Field | Required | Notes |
|-------|----------|-------|
| `title` | ✅ | Displayed as the result title; used as the paste value if `subtext` is absent |
| `subtext` | No | Secondary display line; also the value used when the item is selected |

Blank lines between items and `#` comments are valid YAML and are encouraged for readability.

---

## Command YAML

```yaml
# Minimal — selecting an item just dismisses the launcher
phrase: team emails
title: Team email addresses
action:
  type: static_list
  config:
    list: team-emails

# With paste action — selecting an item pastes its subtext
phrase: paste snippet
title: Pick a text snippet
action:
  type: static_list
  config:
    list: snippets
    item_action: paste_text

# With copy action — selecting an item copies its subtext to clipboard
phrase: copy link
title: Pick a link to copy
action:
  type: static_list
  config:
    list: links
    item_action: copy_text

# With open_url — the item's subtext must be a valid URL
phrase: open bookmark
title: Pick a bookmark to open
action:
  type: static_list
  config:
    list: bookmarks
    item_action: open_url
```

| `item_action` value | What happens on selection |
|---------------------|---------------------------|
| *(absent)* | Launcher dismisses; no further action |
| `paste_text` | Item's `subtext` (or `title`) is pasted into the previously focused app |
| `copy_text` | Item's `subtext` (or `title`) is copied to the clipboard |
| `open_url` | Item's `subtext` (or `title`) is opened as a URL in the default browser |

> **Note:** `subtext` serves a dual purpose — it's the secondary hint shown in the UI *and* the value used when the item is acted on. If `subtext` is absent, `title` is used instead.

---

## Behaviour

| Phase | What happens |
|-------|--------------|
| Partial match (e.g. `team`) | Command appears as a single result row, like any other command |
| Exact phrase match (`team emails`) | List items replace the result row immediately — no `Enter` required |
| Backspace past exact match | List collapses; standard partial-match results return |
| Select an item (`Enter` / click) | Performs the configured `item_action`, or dismisses if none is set |
