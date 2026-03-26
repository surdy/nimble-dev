# What If: Each Command Specifies Where Context Is Inserted

## The Question

Today, the context is always appended to the **end** of the user's raw input:
```
effectiveInput = rawInput + " " + context
```

What if, instead, each command YAML could declare **where** the context should be inserted within its phrase — using an explicit placeholder like `{context}`?

---

## How It Works Today

The context is a global modifier on the **input side**. The matching engine computes one `effectiveInput` and tests it against all command phrases. The command YAML has no awareness of context at all.

```yaml
# Command knows nothing about context
phrase: open reddit
title: Open Reddit
```

```
/ctx set reddit
open          → effective: "open reddit" → matches "open reddit" ✅
```

---

## What Would Change

### The core idea

Commands would use a `{context}` token inside their `phrase` to declare where the active context gets inserted:

```yaml
phrase: open {context}
title: Open {context}
```

When a context is set, the engine expands the phrase: `open {context}` with context `reddit` → `open reddit`. The user's raw input is then matched against the **expanded phrase**, not the original template.

### Matching logic inversion

| Aspect | Today | Proposed |
|--------|-------|----------|
| What gets transformed | The user's input (once) | Every command phrase (once per context change) |
| Commands involved | Zero — they're passive | Each one declares its own context slot |
| Global vs per-command | Global rule | Per-command opt-in |

---

## Design Options Within This Model

### Option 1: `{context}` as a phrase-level placeholder

```yaml
phrase: open {context} issues
title: Open {context} Issues
```

Context `github` → expanded phrase: `open github issues`
User types `open issues` → doesn't match (wrong words)
User types `open github issues` → matches ✅

**Problem:** This doesn't actually help the user type *less*. The user still has to type the full expanded phrase. The context doesn't reduce keystrokes — it just changes what phrase the command presents.

### Option 2: `{context}` as an invisible gap-filler

The `{context}` tokens are matched silently — the user never types them. The engine strips `{context}` words from the phrase when matching and only checks that the remaining words appear in the input.

```yaml
phrase: open {context} issues
```

Context `github` → expanded phrase: `open github issues`
Matching treats `{context}` as auto-filled → user only needs to type `open issues`

**Problem:** Now matching uses partial word sequences, similar to the "any position" approach. It works for this case, but the rules about what the user must type vs. what is auto-filled become non-obvious.

### Option 3: `{context}` as both a filter and a gap-filler (recommended if pursued)

- When no context is set, `{context}` acts as a wildcard — any word matches that position
- When a context is set, that position is locked to the context value and the user doesn't need to type it

```yaml
phrase: open {context} issues
```

No context → user types `open github issues` → matches (github fills the wildcard)
Context `github` → user types `open issues` → matches (github auto-fills the slot)
Context `github` → user types `open gitlab issues` → no match (gitlab ≠ github)

This is the most powerful option but also the most complex.

---

## Key Findings

### 1. Loss of the "universal modifier" mental model

Today's context is beautifully simple: *set it once, everything narrows.* It works the same way for every command without the command author doing anything. With per-command `{context}` placement:

- Commands **without** `{context}` in their phrase would be **unaffected** by the active context
- Only commands that **opt in** get context behaviour
- The user must trust that command authors placed `{context}` where it makes sense

This shifts context from a **system-level input modifier** to a **per-command template feature**. That's a fundamentally different paradigm.

### 2. Command authoring becomes harder

Today, writing a command is dead simple — pick a phrase, pick an action. The author doesn't think about context at all. With `{context}`:

- Authors must decide: "Does this command benefit from context? If so, where does the context go?"
- Getting it wrong (or forgetting it) means the command silently ignores contexts
- New users copying example configs may not understand why some commands respond to context and others don't

### 3. The title field inherits the same problem

If the phrase has `{context}`, should the title also expand? Today the title is a static label. With context expansion, it becomes a template:

```yaml
phrase: open {context} issues
title: Open {context} Issues
```

Context `github` → title shows "Open github Issues" (note the casing mismatch). Should the engine title-case the context? What about multi-word contexts? This is solvable but adds edge cases.

### 4. Parameter interaction is cleaner

One genuine upside: `{param}` and `{context}` have clearly separated roles.

```yaml
phrase: search {context}
action:
  type: open_url
  config:
    url: https://www.{context}.com/search?q={param}
```

Context `google`, user types `search svelte` → opens `google.com/search?q=svelte`
Context `github`, user types `search nimble` → opens `github.com/search?q=nimble`

This is actually compelling — you could write **one generic search command** that routes to different engines based on the context. Today you need separate `search google`, `search github`, `search npm` commands.

### 5. Backward compatibility concern

Every existing command has no `{context}` in its phrase. Under this model, **all existing commands would stop responding to contexts.** That's a breaking change. You'd need a migration strategy:

- **Implicit append** as fallback: if a command has no `{context}`, behave as today (append to end)
- **Explicit only**: require all context-aware commands to have `{context}` (breaks existing setups)

The implicit fallback is the safe choice but means two different context mechanisms coexist, which is confusing.

### 6. Context value now leaks into config semantics

Today the context is runtime-only — it affects input matching but never touches config files. With `{context}` in URLs and text:

```yaml
action:
  type: open_url
  config:
    url: https://www.{context}.com
```

The context value is now injected into **action config**. This is powerful but means the context value needs sanitisation (URL encoding, escaping). It also means a malicious or accidental context value could produce unexpected URLs — a minor security surface.

---

## Opinion

**Mixed — the per-command approach is well-motivated but may not be worth the complexity.**

**The good:** Placing `{context}` explicitly in the phrase is honest, predictable at the per-command level, and unlocks a genuinely useful pattern — generic commands that route based on context (e.g., one `search {context}` command instead of N separate search commands). The parameter/context separation is cleaner than today's suffix model.

**The bad:** It kills the "universal modifier" mental model that makes today's context so easy to explain. It adds authoring burden ("where do I put `{context}`?"), creates a backward compatibility problem, and requires either a breaking migration or two coexisting context mechanisms.

**The alternative I'd recommend:** If the goal is "write one search command that works for Google, GitHub, and npm based on context" — that's better served by a **parameterised URL with context** feature (`url: https://www.{context}.com/search?q={param}`) without changing how phrase matching works. Keep context as the global append-to-end modifier for matching, but allow `{context}` substitution in action config values (URL, text, etc.). You get the power without touching the matching engine.

**Bottom line:** The phrase-level `{context}` placeholder adds complexity to the part of Nimble that should stay simplest (phrase matching). But `{context}` in **action config values** (URL, paste text) is a clean, valuable feature that doesn't affect matching at all.
