# UX Impact: Skip Context Append When Parameter Is Typed

## What Changes for the User

### The One-Sentence Summary

If you type text beyond a command phrase while a context is active, the context no longer contaminates your parameter — your typed text is the entire parameter.

---

## Detailed Workflow Walkthrough

### Workflow 1: Context as Phrase Scoping (Unchanged)

This is the primary context use case and is **completely unaffected**.

1. `/ctx set reddit`
2. Type `open` → effectiveInput = `"open reddit"` → matches `open reddit`
3. Press Enter → opens reddit.com

**Why unchanged:** The raw input `"open"` doesn't start with any command `phrase + " "`, so no param is detected, and context is still appended.

### Workflow 2: Context as Search Topic — Bare Phrase (Unchanged)

1. `/ctx set typescript generics`
2. Type `search google` → effectiveInput = `"search google typescript generics"` → param = `"typescript generics"`
3. Press Enter → searches Google for "typescript generics"

**Why unchanged:** Exact phrase match, no trailing text, context appended as before.

### Workflow 3: Context as Search Topic — With Refinement (CHANGED)

1. `/ctx set rust`
2. Type `search google borrowing`

| | effectiveInput | Param |
|---|---|---|
| **Current** | `search google borrowing rust` | `borrowing rust` |
| **Proposed** | `search google borrowing` | `borrowing` |

The context is no longer combined with the user's explicit query. The user must either:
- Type the full query themselves: `search google borrowing rust`
- Or use context without additional terms: `search google` (just gets `rust`)

**This is a capability loss.** Users who set context as a persistent topic and then add refinement terms lose the combined search.

### Workflow 4: Context Leaking into Parameters (FIXED)

1. `/ctx set work` (used to scope commands to work-related ones)
2. Type `search google weekend plans`

| | Param |
|---|---|
| **Current** | `weekend plans work` ← "work" is nonsensical here |
| **Proposed** | `weekend plans` ← clean |

This is the primary motivation for the proposal. When context is **scoping** (not topic), having it in params is wrong.

### Workflow 5: Dynamic List Script Arguments (IMPROVED)

Commands with `dynamic_list` or `script_action` that accept arguments:

1. `/ctx set prod` (scoping to production commands)
2. Type `show logs api-errors`

| | Script arg |
|---|---|
| **Current** | `api-errors prod` |
| **Proposed** | `api-errors` + env `NIMBLE_CONTEXT=prod` |

The script gets a clean argument and can use the context from the environment variable if needed. This is architecturally cleaner — it separates "what the user asked for" from "what scope they're in."

### Workflow 6: Static List Phrase Completion (Unchanged)

1. `/ctx set emails`
2. Type `paste team` → effectiveInput = `"paste team emails"` → exact match → list expands

No change. Static list matching requires exact phrases, and the proposal only suppresses context when a *parameter* is detected, which requires text beyond the full phrase.

---

## The Behavioral Cliff

The proposal creates a sharp boundary:

| User types | Context behavior |
|---|---|
| `search google` (exact phrase) | Context **appended** → becomes param |
| `search google c` (one extra char) | Context **dropped** → only typed text is param |

One character changes the behavior completely. This could be disorienting:

- User sets context `rust`, types `search goo` → effectiveInput = `"search goo rust"` (context appended, partial match)
- User continues to `search google` → effectiveInput = `"search google rust"` (context appended, exact match, param = `"rust"`)
- User adds a space: `search google ` → effectiveInput = `"search google "` (context DROPPED because trailing space triggers param detection)

Wait — does a trailing space count? `"search google ".startsWith("search google ")` → **yes**. So even a trailing space would suppress context. The user would need to **not** type a trailing space to keep context-as-param behavior. This is extremely fragile.

**Mitigation:** The two-pass check could require a non-empty suffix (i.e., `rawInput.startsWith(phrase + " ") && rawInput.slice(phrase.length + 1).trim() !== ""`). This avoids the trailing-space trap but adds implementation nuance.

---

## Impact on the Subtext Display

In the results list, each row shows context-aware subtext:

```svelte
{#if isParamMode}
  {cmd.phrase}<span class="param-hint"> → {paramText}</span>
{:else}
  {hl.before}<mark>{hl.match}</mark>{hl.after}
{/if}
```

Currently:
- `search google cats` with ctx `work` → subtext: `search google → cats work`

Proposed:
- `search google cats` with ctx `work` → subtext: `search google → cats`

The subtext becomes more honest — it shows exactly what will be used as the parameter. This is a **positive UX signal**.

---

## Who Benefits, Who Loses

### Benefits
- Users who use context for **command scoping** (the "work" context to see only work commands) and then type params into parameterized commands
- Script authors receiving arguments via `dynamic_list` / `script_action` — clean inputs
- Mental model simplicity: "context = which commands I see, params = what I explicitly type"

### Loses
- Users who use context as a **persistent search topic** and want to refine with additional terms
- This is a real workflow (documented in the contexts guide) but likely a minority use case

### Unaffected
- Users with no active context (majority of the time)
- Users who type only bare phrases with context
- Static list expansion workflows
- Built-in `/ctx` commands

---

## Alternative Approaches

### Option A: Per-Command `context_param` Flag
Add a YAML field to let command authors control whether context should be included in the parameter:

```yaml
phrase: "search google"
title: "Search Google"
context_param: true   # context is always included in {param}
action:
  type: open_url
  config:
    url: "https://www.google.com/search?q={param}"
```

**Pro:** No global behavior change, authors opt in.  
**Con:** More config complexity, every search command needs the flag.

### Option B: Explicit Param Separator
Reserve a character (e.g., `--`) to mark the start of explicit params:

```
search google -- cats    # "cats" is the param, context ignored
search google cats       # "cats" + context = full param
```

**Pro:** User controls when context is included.  
**Con:** Unnatural typing experience for a launcher, adds syntax.

### Option C: Context Position Control (per-command)
A `context` field in YAML that controls where/whether context is applied:

```yaml
phrase: "search google"
context: param          # context goes to param
# vs
phrase: "open slack"
context: suffix         # context appended to phrase (default)
```

**Pro:** Full control.  
**Con:** Significant spec complexity.

### Option D: Accept Current Behavior + Document the Quirk
Keep context always appended. Document that when an explicit param is typed, the context is included. Scripts can strip it using `NIMBLE_CONTEXT`.

**Pro:** No code change, no new edge cases.  
**Con:** The "cats work" surprise persists.
