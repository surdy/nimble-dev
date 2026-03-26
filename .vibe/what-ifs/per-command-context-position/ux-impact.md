# UX Impact: Per-Command Context Position

## What changes from the user's daily perspective

### 1. Authoring burden increases

Today, writing a command YAML never involves thinking about context. Authors pick a phrase and an action — done. With `{context}` as a phrase feature, every command becomes a decision point:

- "Should this command respond to context?"
- "Where in the phrase should `{context}` go?"
- "What happens when no context is set — does `{context}` become a wildcard, or does the command hide?"

New users will forget to add `{context}` and wonder why their command ignores the active context.

### 2. Discovery and browsing changes

Today with no context set, the user types `search` and sees all search commands: `search google`, `search github`, `search npm`. Each is a distinct result.

With `{context}` and no context set, `search {context}` either:
- Shows as one collapsed result with the literal text `{context}` visible ← ugly
- Shows as multiple expanded results (one per… what? the engine doesn't know what values `{context}` can take)
- Hides entirely until a context is set ← discoverability loss

This is a fundamental discoverability problem. The current model preserves all commands as always-visible; the proposed model hides context-dependent commands when context is unset.

### 3. Context becomes a command-level feature rather than a system-level feature

**Today's mental model (1 sentence):**
> "Context appends a word to everything I type."

**Proposed mental model (paragraph):**
> "Context fills in `{context}` slots in my commands. Commands without `{context}` may or may not respond to context (depending on fallback behaviour). When no context is set, commands with `{context}` might show with placeholder text or might be hidden. The context value also gets substituted into URLs and text fields if the author included `{context}` there."

The proposed model is more **powerful** but requires substantially more explanation.

### 4. The "set context once, everything narrows" workflow partially breaks

Today:
```
/ctx set github
open                → narrows to "open github"
search              → narrows to "search github"  
paste               → narrows to "paste ... github" (probably no match, which is fine)
```

With per-command `{context}`:
```
/ctx set github
open                → matches "open {context}" if it exists, i.e. "open github"
search              → matches "search {context}" if it exists  
paste greeting      → matches "paste greeting" (no {context} → ignores the context entirely)
team emails         → "team emails" (no {context} → ignores context)
```

The narrowing behaviour is inconsistent across commands. Some respond, some don't. Users must know which commands have `{context}` and which don't.

### 5. Generic commands unlock a powerful pattern

The most compelling use case: **one command that works for many targets.**

Instead of maintaining 5 separate "open X" commands:
```
open-github.yaml, open-reddit.yaml, open-google.yaml, open-slack.yaml, open-notes.yaml
```

One file:
```yaml
phrase: open {context}
title: Open {context}
action:
  type: open_url
  config:
    url: https://www.{context}.com
```

For users with many similar commands, this is a real reduction in config files. But it only works when all targets follow the same URL pattern.

### 6. Context chip meaning changes

Today the chip shows the active context and the user knows exactly what it does — "this word is glued to the end of my input."

With per-command placement, the chip still shows the value, but its *effect* depends on which command is being matched. The chip goes from "this is what happens" to "this is a value that some commands will use."

---

## Workflow comparison

### Scenario: Research session across multiple sites

**Today:**
```
/ctx set react hooks
search google          → opens google search for "react hooks"
search github          → opens github search for "react hooks"
search npm             → opens npm search for "react hooks"
```
Context is the **param** (search query). Three commands exist. Works perfectly.

**Proposed:**
```yaml
# One command
phrase: search {context}
action:
  type: open_url
  config:
    url: https://www.{context}.com/search?q={param}
```
```
/ctx set google
search react hooks     → opens google search for "react hooks"
```
Context is the **target** (search engine), and `{param}` is the query. One command exists. Also works, but the context means something different — it's the engine, not the query.

These are **two different conceptual uses** of context, and the per-command model encourages the second one (context-as-routing) over the first (context-as-param). Neither is wrong, but they're incompatible — you can't use context for both within the same session.

---

## Summary

| Aspect | Today (append) | Per-command {context} |
|--------|----------------|-----------------------|
| Authoring effort | None (context-unaware) | Must decide placement per command |
| Mental model | 1 sentence | 1 paragraph |
| Discoverability | All commands always visible | Context-dependent commands may be hidden |
| Narrowing consistency | Every command is affected | Only opt-in commands are affected |
| Power/flexibility | Lower | Higher (generic commands) |
| Backward compatibility | N/A | Breaking without fallback |
| Config file count | One per target | One generic per pattern |
| Context chip clarity | High (one rule) | Medium (depends on command) |
