# Config Examples: Data Directory for script_action

## The Use Case

An external application (say, a CRM tool) exports a contacts JSON file daily to `~/Data/crm/contacts.json`. You want a Nimble `dynamic_list` command that runs a script to parse and display those contacts. Today the script must hard-code `~/Data/crm/` or rely on an env var you set up yourself. With a `data_dir` field, the command YAML tells the script exactly where to look.

---

## Current: Script Hard-codes the Path

### `commands/search-contacts.yaml`
```yaml
phrase: search contacts
title: Search CRM contacts
action:
  type: dynamic_list
  config:
    script: crm-contacts.sh
    arg: optional
    item_action: paste_text
```

### `scripts/crm-contacts.sh`
```sh
#!/bin/sh
# Hard-coded path — breaks if user has data elsewhere
DATA_FILE="$HOME/Data/crm/contacts.json"
QUERY="$1"

if [ ! -f "$DATA_FILE" ]; then
  echo '[]'
  exit 0
fi

if [ -z "$QUERY" ]; then
  cat "$DATA_FILE"
else
  python3 -c "
import json, sys
with open('$DATA_FILE') as f:
    items = json.load(f)
q = sys.argv[1].lower()
print(json.dumps([i for i in items if q in i['title'].lower()]))
" "$QUERY"
fi
```

**Problem:** The path `~/Data/crm/` is baked into the script. If another user has their CRM exports at `~/Documents/crm-export/`, they must edit the script.

---

## Proposed: `data_dir` Field

### `commands/search-contacts.yaml`
```yaml
phrase: search contacts
title: Search CRM contacts
action:
  type: dynamic_list
  config:
    script: crm-contacts.sh
    arg: optional
    item_action: paste_text
    data_dir: ~/Data/crm          # NEW — passed to script as NIMBLE_DATA_DIR
```

### `scripts/crm-contacts.sh`
```sh
#!/bin/sh
# Uses the directory provided by the command config
DATA_FILE="$NIMBLE_DATA_DIR/contacts.json"    # env var set by Nimble
QUERY="$1"

if [ ! -f "$DATA_FILE" ]; then
  echo '[]'
  exit 0
fi

if [ -z "$QUERY" ]; then
  cat "$DATA_FILE"
else
  python3 -c "
import json, sys
with open(sys.argv[2]) as f:
    items = json.load(f)
q = sys.argv[1].lower()
print(json.dumps([i for i in items if q in i['title'].lower()]))
" "$QUERY" "$DATA_FILE"
fi
```

**Now portable:** A different user changes only the YAML — the script stays untouched:
```yaml
    data_dir: ~/Documents/crm-export
```

---

## Full Proposed Schema

### `dynamic_list` config
```yaml
action:
  type: dynamic_list
  config:
    script: <string>
    arg: none | optional | required
    item_action: paste_text | copy_text | open_url
    data_dir: <string>              # NEW — optional, path within ~/
```

### `script_action` config
```yaml
action:
  type: script_action
  config:
    script: <string>
    arg: none | optional | required
    result_action: open_url | paste_text | copy_text
    prefix: <string>
    suffix: <string>
    data_dir: <string>              # NEW — optional, path within ~/
```

---

## How Nimble Processes `data_dir`

| Step | What happens |
|------|-------------|
| 1. Parse | Read `data_dir` from YAML (optional — omit = no env var set) |
| 2. Expand | Replace leading `~` or `~/` with the user's `$HOME` |
| 3. Validate | Reject if the resolved path is not within `$HOME` |
| 4. Validate | Reject if the resolved path does not exist or is not a directory |
| 5. Pass | Set `NIMBLE_DATA_DIR=<resolved absolute path>` in the script's environment |

### Validation error examples
```yaml
# REJECTED — outside $HOME
data_dir: /etc/secrets

# REJECTED — path traversal
data_dir: ~/../../etc/passwd

# REJECTED — directory doesn't exist (warning at load time)
data_dir: ~/Data/nonexistent

# VALID
data_dir: ~/Data/crm
data_dir: ~/Documents/exports
data_dir: ~/Downloads
```

---

## More Examples

### script_action — paste latest build number
```yaml
phrase: paste build number
title: Paste latest CI build number
action:
  type: script_action
  config:
    script: get-build-number.sh
    arg: none
    result_action: paste_text
    data_dir: ~/ci-artifacts        # script reads build-info.json from here
```

### dynamic_list — browse log files
```yaml
phrase: search logs
title: Search application logs
action:
  type: dynamic_list
  config:
    script: search-logs.sh
    arg: required
    item_action: copy_text
    data_dir: ~/Library/Logs/MyApp
```

### No data_dir (unchanged — works exactly as before)
```yaml
phrase: paste timestamp
title: Paste current date/time
action:
  type: script_action
  config:
    script: timestamp.sh
    arg: none
    result_action: paste_text
    # no data_dir — script doesn't need external data
```

---

## Alternative Considered: Generic `env` Map

Instead of a single `data_dir`, allow arbitrary environment variables:

```yaml
action:
  type: script_action
  config:
    script: crm-contacts.sh
    env:
      DATA_DIR: ~/Data/crm
      API_KEY: abc123
      LOG_LEVEL: debug
```

### Why this was not recommended

| Factor | `data_dir` only | Generic `env` |
|--------|----------------|---------------|
| Simplicity | One field, one purpose | Open-ended — "what can I put here?" |
| Security | Easy to validate (must be a dir within ~/) | Must validate every value? Or treat as opaque? |
| Abuse potential | Low — it's a directory path | High — users could pass secrets, tokens, paths to anywhere |
| Learnability | "data_dir is where your data lives" | "env is a bag of strings — read the script to know what it expects" |
| Nimble philosophy | Fits — simple, opinionated | Doesn't fit — too configurable |

A generic `env` is a future option if demand exists, but `data_dir` alone covers the primary use case.
