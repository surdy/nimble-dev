# Config Examples: Static List as TSV

## Current State — YAML list file

### Directory layout
```
commands/show-team-emails/
  show-team-emails.yaml     ← command YAML
  team-emails.yaml          ← list file (YAML array)
```

### Command YAML
```yaml
phrase: team emails
title: Team email addresses
action:
  type: static_list
  config:
    list: team-emails           # resolves to team-emails.yaml
    item_action: paste_text
```

### List file (`team-emails.yaml`)
```yaml
# Team email addresses
- title: Alice Smith
  subtext: alice@example.com

- title: Bob Jones
  subtext: bob@example.com

- title: Carol White
  subtext: carol@example.com
```

**Lines:** 10 (with blanks/comments)  
**Syntax to learn:** YAML arrays, `title:` / `subtext:` keys, indentation rules

---

## Option A — TSV list file (exclusive)

### Directory layout
```
commands/show-team-emails/
  show-team-emails.yaml     ← command YAML (unchanged)
  team-emails.tsv            ← list file (TSV)
```

### Command YAML — unchanged content, resolution changes
```yaml
phrase: team emails
title: Team email addresses
action:
  type: static_list
  config:
    list: team-emails           # now resolves to team-emails.tsv
    item_action: paste_text
```

### List file (`team-emails.tsv`)
```
# Team email addresses
Alice Smith	alice@example.com
Bob Jones	bob@example.com
Carol White
```

**Lines:** 4 (with comment)  
**Syntax to learn:** tabs between columns, `#` for comments

### Notes
- `Carol White` has no tab → subtext is `None`, title is used as the action value
- The `list:` field still uses the name without extension; resolver tries `.tsv` instead of `.yaml`

---

## Option D — Support both (detect by extension)

### Directory layout — user chooses format per list
```
commands/
  show-team-emails/
    show-team-emails.yaml     ← command YAML
    team-emails.tsv            ← TSV list (user chose TSV)
  show-shortcuts/
    show-shortcuts.yaml        ← command YAML
    shortcuts.yaml             ← YAML list (user prefers YAML)
```

### Command YAML — unchanged
```yaml
# list: team-emails
# Resolver tries: team-emails.tsv, then team-emails.yaml, then team-emails.yml
list: team-emails
```

### Resolution order
1. `<name>.tsv` — if exists, parse as TSV
2. `<name>.yaml` — if exists, parse as YAML
3. `<name>.yml` — if exists, parse as YAML
4. Error — file not found

---

## With `${VAR}` substitution

### Current (YAML)
```yaml
list: ${NIMBLE_CONFIG_DIR}/shared-lists/team-emails
# resolves to: /Users/you/.../Nimble/shared-lists/team-emails.yaml
```

### TSV (Option A)
```yaml
list: ${NIMBLE_CONFIG_DIR}/shared-lists/team-emails
# resolves to: /Users/you/.../Nimble/shared-lists/team-emails.tsv
```

### Dual support (Option D)
```yaml
list: ${NIMBLE_CONFIG_DIR}/shared-lists/team-emails
# tries: ...team-emails.tsv, then ...team-emails.yaml
```

---

## Comparison: A 20-item list

### YAML (current) — 60 lines
```yaml
- title: Item 1
  subtext: value1

- title: Item 2
  subtext: value2

# ... 18 more items ...

- title: Item 20
  subtext: value20
```

### TSV — 20 lines
```
Item 1	value1
Item 2	value2
# ... 18 more items ...
Item 20	value20
```

**60% fewer lines for the same data.**

---

## Spreadsheet copy-paste workflow

### Step 1: User has data in a spreadsheet

| Name | Email |
|------|-------|
| Alice Smith | alice@example.com |
| Bob Jones | bob@example.com |

### Step 2a: YAML (current) — manual reformat required
Select cells → paste into editor → manually wrap each row in `- title: ... subtext: ...` syntax

### Step 2b: TSV — direct paste
Select cells → Cmd+C → paste into `.tsv` file → done (spreadsheets copy as TSV by default)
