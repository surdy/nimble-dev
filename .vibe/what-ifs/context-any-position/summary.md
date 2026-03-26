# What If: Context Applied at Any Position in the Phrase

## The Question

Currently, the active context is always **appended to the end** of the user's raw input:

```
effectiveInput = rawInput + " " + context
```

What if the context could be matched at **any position** within a command phrase — beginning, middle, or end — rather than only the last position?

---

## How It Works Today

The rule is simple: `effectiveInput = rawInput + " " + context`.

| Context   | User types      | Effective input          | Matches phrase…       |
|-----------|-----------------|--------------------------|-----------------------|
| `reddit`  | `open`          | `open reddit`            | `open reddit` ✅       |
| `google`  | `search`        | `search google`          | `search google` ✅     |
| `github`  | `open`          | `open github`            | `open github` ✅       |
| `github`  | `open issues`   | `open issues github`     | `open github issues` ❌ |
| `team`    | `paste emails`  | `paste emails team`      | `paste team emails` ❌  |

The last two rows reveal the limitation: if the context value belongs in the **middle** of a phrase, the current append-only model cannot match it.

---

## What Would Change

### Three possible matching strategies

**Strategy A — Combinatorial Insertion**
Try inserting the context tokens at every word boundary in the raw input and check if any insertion yields a phrase match.

- `"open issues"` + context `"github"` → try `"github open issues"`, `"open github issues"`, `"open issues github"` → second one hits `"open github issues"`.

**Strategy B — Bag-of-Words Matching**
Treat both the effective input and the phrase as sets of tokens. A phrase matches if *all* tokens from `rawInput + context` appear in the phrase (regardless of order).

**Strategy C — Ordered Subsequence with Gap-Fill**
The raw input tokens must appear in order within the phrase, and the context fills any gaps between them.

---

## Key Findings

### 1. Ambiguity explosions

With Strategy A (combinatorial), a 3-word input + 2-word context has **10 possible insertion points**. The matching engine must check each one against every loaded command. For typical command sets (<500) this is still fast, but the logic is dramatically more complex than one string append.

With Strategy B (bag-of-words), order is completely lost. This creates **false positive matches**: if you have both `"open github issues"` and `"open issues github"` (unlikely but legal), they become indistinguishable. More realistically, phrases like `"paste team log"` and `"paste log team"` would both match the same input, making results unpredictable.

### 2. Parameter extraction becomes ambiguous

Today, param extraction is trivial:

```
param = effectiveInput.slice(phrase.length + 1)
```

Everything after the matched phrase is the `{param}`. If the context can appear in the middle of the phrase, there is no longer a clean boundary between "phrase tokens" and "param tokens." Consider:

- Phrase: `search google`
- Context: `google`
- User types: `search typescript generics`

Today: `effectiveInput = "search typescript generics google"` → matches because `typed.startsWith("search google ")` is false… actually it **doesn't** match today, which is correct — the context is a phrase-narrower, not a param injector when the phrase is already fully typed.

With any-position matching: should `google` fill the second word of `search google`, making the param `"typescript generics"`? Or should the whole thing be a bag-of-words match with no clear param? The answer depends on the strategy, and each requires heuristics that may surprise users.

### 3. The mental model gets much harder

The current model is one sentence: *"the context is glued to the end of what you type."* Every user can predict what will match. With any-position matching, the user must mentally model: "my context words will be slotted in wherever they fit best across all my commands." That's hard to predict, especially when commands share words.

### 4. The real use case is narrow

The scenarios where mid-position context helps are phrases with structure like `<verb> <context> <object>`. But Nimble's phrase design already encourages putting the most specific/discriminating word **last** (e.g., `open reddit`, `search google`, `paste greeting`). The context naturally fills the discriminator slot at the end.

The cases where mid-position would help — `open github issues`, `paste team emails` — can be solved by restructuring the phrase to `open issues github` or `paste emails team`, which is arguably clearer anyway since the last word is the narrowing context.

---

## Opinion

**Negative — this change adds significant complexity for very little practical gain.**

The append-to-end model is elegant precisely because it maps to a single, predictable rule. Any-position matching introduces ambiguity in matching, ambiguity in parameter extraction, and a harder mental model — all to serve a narrow set of phrases that users can easily restructure to work with the current system.

The core design philosophy of Nimble is that phrases are short, predictable, and user-authored. The user controls the word order. If a context should complete a phrase, the user simply puts the context-able word at the end when they write the YAML.

If there's a genuine need for richer mid-phrase matching, a better approach might be **phrase aliases** or **phrase templates with named slots** — both of which are more explicit and predictable than implicit any-position insertion.

**Recommendation:** Keep the current append-to-end model. Document a "phrase authoring tip" advising users to put context-target words at the end of their phrases.
