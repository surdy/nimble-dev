# Config Examples: Inline Static Lists

## Before (current) — Two Files Required

### `commands/show-team-emails.yaml`
```yaml
phrase: team emails
title: Team email addresses
action:
  type: static_list
  config:
    list: team-emails
    item_action: paste_text
```

### `lists/team-emails.yaml`
```yaml
- title: Alice Smith
  subtext: alice@example.com

- title: Bob Jones
  subtext: bob@example.com

- title: Carol White
  subtext: carol@example.com
```

---

## After (inline option) — Single File

### `commands/show-team-emails.yaml`
```yaml
phrase: team emails
title: Team email addresses
action:
  type: static_list
  config:
    item_action: paste_text
    items:
      - title: Alice Smith
        subtext: alice@example.com
      - title: Bob Jones
        subtext: bob@example.com
      - title: Carol White
        subtext: carol@example.com
```

---

## After (external file — unchanged)

The existing `list:` reference continues to work identically:

```yaml
phrase: team emails
title: Team email addresses
action:
  type: static_list
  config:
    list: team-emails
    item_action: paste_text
```

---

## Validation Rules

```yaml
# VALID: inline items
config:
  items:
    - title: Foo
  item_action: paste_text

# VALID: external list
config:
  list: my-list
  item_action: paste_text

# INVALID: both provided — should produce a load-time error
config:
  list: my-list
  items:
    - title: Foo
  item_action: paste_text

# INVALID: neither provided — should produce a load-time error
config:
  item_action: paste_text
```

---

## More Examples

### Minimal inline list (no item action)
```yaml
phrase: project links
title: Project links
action:
  type: static_list
  config:
    items:
      - title: GitHub Repo
        subtext: https://github.com/example/project
      - title: CI Dashboard
        subtext: https://ci.example.com
      - title: Docs Site
        subtext: https://docs.example.com
```

### Inline list with open_url action
```yaml
phrase: open bookmark
title: Open a bookmark
action:
  type: static_list
  config:
    item_action: open_url
    items:
      - title: GitHub
        subtext: https://github.com
      - title: Hacker News
        subtext: https://news.ycombinator.com
      - title: Reddit
        subtext: https://reddit.com
```

### Inline list with copy_text action
```yaml
phrase: copy snippet
title: Copy a code snippet
action:
  type: static_list
  config:
    item_action: copy_text
    items:
      - title: Console log
        subtext: "console.log('debug:', )"
      - title: TODO comment
        subtext: "// TODO: "
```

### When external file still makes sense
Two commands share the same list data:

```yaml
# commands/show-team-emails.yaml
phrase: team emails
title: Show team emails
action:
  type: static_list
  config:
    list: team-emails

# commands/paste-team-email.yaml
phrase: paste team email
title: Paste a team email address
action:
  type: static_list
  config:
    list: team-emails
    item_action: paste_text
```

Both reference `lists/team-emails.yaml` — updating that one file updates both commands.
