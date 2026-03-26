# Config Examples: Skip Context Append with Parameter

The proposal requires **no changes to the YAML command schema** — it's a purely frontend matching behavior change. However, it changes how existing commands *behave* when context is active and a parameter is typed.

---

## Commands Unaffected (No Parameters)

These commands have no `{param}` or argument mechanism. Context only affects phrase matching, which works identically under both behaviors.

```yaml
# paste-greeting.yaml — context scopes matching, no param involved
phrase: "paste greeting"
title: "Paste a friendly greeting"
action:
  type: paste_text
  config:
    text: "Hello! Thanks for reaching out."
```

```yaml
# copy-email.yaml — same: context scopes, no param
phrase: "copy email"
title: "Copy my email address"
action:
  type: copy_text
  config:
    text: "me@example.com"
```

---

## open_url with {param} — The Most Affected Action Type

### Before (current behavior)

```yaml
# search-google.yaml
phrase: "search google"
title: "Search Google"
action:
  type: open_url
  config:
    url: "https://www.google.com/search?q={param}"
```

| Context | User types | effectiveInput | Param value |
|---------|-----------|---------------|-------------|
| `rust` | `search google` | `search google rust` | `rust` ✅ |
| `rust` | `search google borrowing` | `search google borrowing rust` | `borrowing rust` ← context appended |
| (none) | `search google cats` | `search google cats` | `cats` |

### After (proposed behavior)

| Context | User types | effectiveInput | Param value |
|---------|-----------|---------------|-------------|
| `rust` | `search google` | `search google rust` | `rust` ✅ (same) |
| `rust` | `search google borrowing` | `search google borrowing` | `borrowing` ← context dropped |
| (none) | `search google cats` | `search google cats` | `cats` (same) |

**Key difference:** Row 2 — the explicit param no longer includes context. Clean for "scoping" use cases, but loses the "combine context + refinement" capability.

---

## dynamic_list with arg: required — Cleaner Script Arguments

### Before (current behavior)

```yaml
# show-team.yaml
phrase: "show team"
title: "Show team members"
action:
  type: dynamic_list
  config:
    script: team-search.sh
    arg: required
    item_action: copy_text
```

| Context | User types | Script receives arg | Script receives NIMBLE_CONTEXT |
|---------|-----------|-------------------|-------------------------------|
| `work` | `show team john` | `john work` ← polluted | `work` |
| (none) | `show team john` | `john` | (empty) |

### After (proposed behavior)

| Context | User types | Script receives arg | Script receives NIMBLE_CONTEXT |
|---------|-----------|-------------------|-------------------------------|
| `work` | `show team john` | `john` ← clean | `work` |
| (none) | `show team john` | `john` | (empty) |

**Improvement:** The script gets exactly what the user typed. If it needs context, it reads `NIMBLE_CONTEXT` from the environment explicitly.

---

## dynamic_list with arg: optional — Subtle Edge Case

```yaml
phrase: "show users"
title: "Show all users"
action:
  type: dynamic_list
  config:
    script: list-users.sh
    arg: optional
    item_action: paste_text
```

| Context | User types | Current arg | Proposed arg |
|---------|-----------|-------------|-------------|
| `eng` | `show users` | `eng` (context leaked as arg) | `eng` (same — exact phrase → context appended) |
| `eng` | `show users j` | `j eng` | `j` |
| (none) | `show users j` | `j` | `j` |

**Note:** Row 1 shows that the existing "context leaks as optional arg" behavior persists under the proposal when the user types only the exact phrase. This is a **separate pre-existing issue** — the proposal doesn't fix it, but also doesn't make it worse.

---

## static_list — Typically Unaffected

Static lists require an **exact phrase match** to expand. Users rarely type beyond the phrase for a static list (there's no param mechanism). Context helps complete the phrase, which works the same under both behaviors.

```yaml
# paste-team-emails.yaml
phrase: "paste team emails"
title: "Paste team emails"
action:
  type: static_list
  config:
    list: emails
    item_action: paste_text
```

| Context | User types | effectiveInput | Match? |
|---------|-----------|---------------|--------|
| `emails` | `paste team` | `paste team emails` | Exact match ✅ |
| `emails` | `paste team e` | `paste team e emails` | No match ❌ (same under both) |

---

## Contextualized Phrases — No Regression

Commands whose phrase already incorporates a "context word" (e.g., "open slack work") continue to work because the raw input won't trigger param detection for them.

```yaml
# open-slack-work.yaml
phrase: "open slack work"
title: "Open Slack (work workspace)"
action:
  type: open_url
  config:
    url: "https://work.slack.com"
```

| Context | User types | Raw param check | Context appended? | effectiveInput |
|---------|-----------|----------------|-------------------|---------------|
| `work` | `open slack` | "open slack" doesn't start with any `phrase + " "` | Yes | `open slack work` → matches ✅ |
| `work` | `open slack w` | "open slack w" starts with "open slack " (if "open slack" is also a command) | No | `open slack w` → partial matches `open slack work` ✅ |
