# Config Examples: Bundled Scripts

## Current Directory Layout

```
Nimble/
  settings.yaml
  commands/
    examples/
      dynamic-list-example.yaml     ← command YAML
      script-action-example.yaml
  lists/
    team-emails.yaml
  scripts/
    hello.sh                        ← script lives here, separate from command
    timestamp.sh
    uuid.sh
    team-emails.sh
```

### Current command YAML
```yaml
# commands/examples/dynamic-list-example.yaml
phrase: say hello
title: Say hello (dynamic list example)
action:
  type: dynamic_list
  config:
    script: hello.sh        # resolves to scripts/hello.sh (flat lookup, no paths)
    arg: optional
    item_action: paste_text
```

### Current script reference rule
- `script:` value must be a **plain filename** (no `/`, `\`, `..`)
- Always resolves to `<config_dir>/scripts/<filename>`

---

## Proposed: Co-located Layout (Option A — relative script)

```
Nimble/
  settings.yaml
  commands/
    examples/
      dynamic-list-example.yaml
      script-action-example.yaml
    say-hello/                       ← "command package" directory
      command.yaml                   ← command YAML
      hello.sh                       ← script lives next to it
    paste-timestamp/
      command.yaml
      timestamp.sh
  lists/
    team-emails.yaml
  scripts/                           ← still exists for shared/global scripts
    uuid.sh
    team-emails.sh
```

### Co-located command YAML
```yaml
# commands/say-hello/command.yaml
phrase: say hello
title: Say hello (dynamic list example)
action:
  type: dynamic_list
  config:
    script: ./hello.sh          # "./" prefix means "relative to this YAML file"
    arg: optional
    item_action: paste_text
```

### Global script reference (unchanged)
```yaml
# commands/paste-timestamp/command.yaml
phrase: paste timestamp
title: Paste current date/time
action:
  type: script_action
  config:
    script: timestamp.sh        # no "./" prefix → resolves to scripts/timestamp.sh
    arg: none
    result_action: paste_text
```

---

## Proposed: Co-located Layout (Option B — implicit convention)

Instead of a `./` prefix, use a naming convention: if a file with the script name exists in the same directory as the command YAML, use it; otherwise fall back to `scripts/`.

```yaml
# commands/say-hello/command.yaml
phrase: say hello
title: Say hello
action:
  type: dynamic_list
  config:
    script: hello.sh           # same as before — but Nimble first checks
                                # commands/say-hello/hello.sh, then scripts/hello.sh
    arg: optional
    item_action: paste_text
```

### Pros of Option B
- Zero config change — existing YAMLs work unchanged
- No new syntax to learn

### Cons of Option B
- **Ambiguous resolution** — "is it using the local one or the global one?" Users can't tell from the YAML alone
- **Shadow risk** — accidentally placing a file named `hello.sh` in a command directory would silently override the global one

---

## Proposed: Co-located Layout (Option C — explicit `script_path` field)

A new field instead of overloading `script:`.

```yaml
# commands/say-hello/command.yaml
phrase: say hello
title: Say hello
action:
  type: dynamic_list
  config:
    script_file: hello.sh      # new field: resolves relative to this YAML file
    arg: optional
    item_action: paste_text
```

Validation: `script` and `script_file` are mutually exclusive (like the proposed `list`/`items` pattern for static lists).

---

## Side-by-Side Comparison of All Options

| Aspect | Current | Option A (`./` prefix) | Option B (implicit) | Option C (`script_file`) |
|--------|---------|----------------------|-------------------|----------------------|
| Config syntax | `script: name.sh` | `script: ./name.sh` | `script: name.sh` | `script_file: name.sh` |
| Explicit? | Yes | Yes | No — ambiguous | Yes |
| Breaking change? | n/a | No | No | No |
| Shadow risk | None | None | Yes | None |
| Learning curve | None | Minimal — `./` is familiar | None | Minimal — new field name |

**Recommendation**: Option A (`./` prefix) — explicit, backwards-compatible, no new field names, familiar convention.

---

## Shared Script Scenario

Two commands need the same script (e.g., `team-emails.sh`):

### Current (works fine)
```
scripts/team-emails.sh        ← one copy
commands/show-team-emails.yaml    → script: team-emails.sh
commands/paste-team-emails.yaml   → script: team-emails.sh
```

### Co-located (awkward)
```
commands/show-team-emails/
  command.yaml                → script: ./team-emails.sh
  team-emails.sh              ← copy #1
commands/paste-team-emails/
  command.yaml                → script: ./team-emails.sh
  team-emails.sh              ← copy #2 (duplication!)
```

**Better**: share via `scripts/`, co-locate only when the script is unique to one command.

```
commands/show-team-emails.yaml   → script: team-emails.sh   (global)
commands/say-hello/
  command.yaml                   → script: ./hello.sh        (local)
  hello.sh
scripts/
  team-emails.sh                 ← shared scripts stay here
```
