# Static List (Updated Documentation — What-If Scenario)

> **Note:** This is a hypothetical version of the static list docs reflecting inline list support. It is not the actual project documentation.

---

Displays a named list of items inline in the launcher the moment the typed phrase matches exactly. No `Enter` is needed to expand the list — it appears as soon as the full phrase is typed. By default, selecting an item simply dismisses the launcher. You can optionally configure an action to be performed on the selected item's value.

---

## Defining list items

You can define list items in two ways: **inline** in the command YAML, or in a **separate list file**. Choose whichever fits your use case.

### Option A: Inline items

Place items directly in the command file under `config.items`. Best for small, single-use lists.

```yaml
phrase: project links
title: Project links
action:
  type: static_list
  config:
    item_action: open_url
    items:
      - title: GitHub Repo
        subtext: https://github.com/example/project
      - title: CI Dashboard
        subtext: https://ci.example.com
      - title: Docs Site
        subtext: https://docs.example.com
```

### Option B: External list file

Store items in a separate YAML file inside the `lists/` subdirectory of your config directory (see [Config Directory](../config-directory.md)). Best for large lists or lists shared across multiple commands.

**`lists/team-emails.yaml`:**
```yaml
# Team email addresses
- title: Alice Smith
  subtext: alice@example.com

- title: Bob Jones
  subtext: bob@example.com

- title: Carol White
  subtext: carol@example.com
```

**`commands/show-team-emails.yaml`:**
```yaml
phrase: team emails
title: Team email addresses
action:
  type: static_list
  config:
    list: team-emails
    item_action: paste_text
```

> **Tip:** Use inline `items` for small, single-use lists. Use an external `list:` file when the list is large or shared across multiple commands.

### Validation

You must provide exactly one of `items` or `list` — not both, and not neither. Nimble will reject the command at load time if this rule is violated.

---

## Item format

Each item has a required `title` and an optional `subtext`, regardless of whether it is defined inline or in an external file.

| Field | Required | Notes |
|-------|----------|-------|
| `title` | ✅ | Displayed as the result title; used as the paste/copy/open value if `subtext` is absent |
| `subtext` | No | Secondary display line; also the value used when the item is selected |

---

## Command YAML — full schema

```yaml
phrase: <string>
title: <string>
action:
  type: static_list
  config:
    # Exactly one of these two:
    items: [...]            # inline list of {title, subtext} items
    list: <string>          # name of list file in lists/ (without .yaml extension)

    # Optional:
    item_action: paste_text | copy_text | open_url
```

### Examples

```yaml
# Inline items, paste on select
phrase: paste snippet
title: Pick a text snippet
action:
  type: static_list
  config:
    item_action: paste_text
    items:
      - title: Greeting
        subtext: "Hello, thanks for reaching out!"
      - title: Sign-off
        subtext: "Best regards, Jane"

# External file, copy on select
phrase: copy link
title: Pick a link to copy
action:
  type: static_list
  config:
    list: links
    item_action: copy_text

# Inline items, no action (just display)
phrase: team roster
title: Team roster
action:
  type: static_list
  config:
    items:
      - title: Alice Smith
        subtext: Engineering
      - title: Bob Jones
        subtext: Design
```

---

## `item_action` reference

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
