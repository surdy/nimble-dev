# What If: Skip Context Append When User Types a Parameter

## The Question

Currently, `effectiveInput = rawInput + " " + context` is applied unconditionally (when context is active, input is non-empty, and doesn't start with `/`). This means if a user types text **beyond** a matched command phrase — i.e., a parameter — the context is still appended, polluting the parameter value.

Example: command `search google` with context `work`. User types `search google cats`. Today, `effectiveInput` = `"search google cats work"`, so the parameter becomes `"cats work"` instead of just `"cats"`.

**Proposal:** Only append context when the raw input is a bare phrase or partial phrase. If the user has typed text beyond a matched command phrase (a parameter), skip the context append.

---

## Key Finding: The Chicken-and-Egg Problem

At the point of computing `effectiveInput`, we **don't yet know which command will match**. Matching happens in `filtered`, which *depends on* `effectiveInput`. So we can't check "is the trailing text a parameter?" without first knowing the matched command.

### A Two-Pass Approach Would Be Required

1. **Pass 1:** Scan all commands against the raw input to check if `rawInput.startsWith(phrase + " ")` for any command phrase → if yes, trailing text exists → user is in "param mode"
2. **Pass 2:** Compute `effectiveInput` based on the result: skip context if param detected, append context otherwise

This is implementable but adds a full scan of all commands before filtering — O(n) with string comparisons, acceptable for performance but adds conceptual complexity.

---

## The Core Design Tension

Context currently serves **two conflicting purposes**:

| Purpose | Example | Needs context in param? |
|---------|---------|------------------------|
| **Phrase scoping** — narrow which commands match | Context `work`, "open slack" → matches "open slack work" | No |
| **Parameter injection** — context becomes the search query | Context `rust programming`, "search google" → searches "rust programming" | Yes |

The proposal cleanly fixes purpose #1 but **partially breaks** purpose #2 when the user combines context with an explicit typed parameter.

---

## Scenario Analysis

| Scenario | Current behavior | Proposed behavior | Better? |
|----------|-----------------|-------------------|---------|
| Type `open` with ctx `reddit` | `"open reddit"` → matches `open reddit` ✅ | Same (no param detected) ✅ | Tie |
| Type `search google` with ctx `rust` | param = `"rust"` ✅ | Same (exact phrase, no param) ✅ | Tie |
| Type `search google cats` with ctx `work` | param = `"cats work"` 😬 | param = `"cats"` ✅ | **Proposed** |
| Type `search google generics` with ctx `rust` (wanting "generics rust") | param = `"generics rust"` ✅ | param = `"generics"` (context lost) ❌ | **Current** |
| Type `open slack` with ctx `work` (phrase "open slack work" exists) | `"open slack work"` → exact match ✅ | Same (no param) ✅ | Tie |
| Type `show team john` with ctx `work` (dynamic_list, arg:required) | script gets `"john work"` 😬 | script gets `"john"`, env has `NIMBLE_CONTEXT=work` ✅ | **Proposed** |
| Type `team` with ctx `emails` (static_list "team emails") | `"team emails"` → exact match ✅ | Same (no param detected for raw "team") ✅ | Tie |

---

## The "Topic-Locked Searches" Regression

The docs describe this workflow:

> `/ctx set typescript generics` → type `search google` → searches "typescript generics"

This works under both current and proposed behaviors (bare phrase → context appended).

**But the refinement workflow breaks:**

> `/ctx set rust` → type `search google borrowing` → current searches "borrowing rust" → proposed searches just "borrowing"

Under the proposal, there's no way to **combine** an explicit query with the context value. The context either fills the entire param (bare phrase) or is ignored completely (phrase + typed text). This binary behavior eliminates the ability to refine a context-based search.

---

## Opinion: Mixed — Cleaner Model but Creates a UX Gap

**Positives:**
- Eliminates the most common surprise (context leaking into parameters)
- Cleaner mental model: "context helps you find commands, parameters are what you explicitly type"
- Scripts/URLs get exactly the argument the user typed — no hidden suffix
- The script still receives `NIMBLE_CONTEXT` as an env var, so scripts that *want* context can access it explicitly

**Negatives:**
- Breaks the documented "topic-locked search + refinement" workflow — this is a real capability loss
- Creates a sharp behavioral cliff: one extra character after a phrase completely changes how context behaves
- The two-pass implementation adds complexity — the current single-expression `effectiveInput` is elegant
- May surprise users who set context expecting it to always influence params

**Verdict:** The proposal improves the common case (context-as-scope) but sacrifices a genuine (if niche) capability (context-as-combined-param). If context is primarily used for **scoping which commands appear**, the proposal is a net positive. If users rely on context as a **persistent search term**, the regression is significant.

A possible middle ground: let the context still be appended, but give the command YAML a way to opt out (e.g., `context_as_param: false`), or let the per-command `context` field control this. This avoids a global behavioral change while letting authors control the interaction.

---

## Files

- [summary.md](summary.md) — this file
- [config-examples.md](config-examples.md) — before/after YAML config examples (no config changes needed for the proposal, but shows affected command types)
- [ux-impact.md](ux-impact.md) — detailed UX walkthrough of all affected workflows
