# Config Examples: Context at Any Position

## Before (current — append to end)

### Phrase design that works with today's context

```yaml
# commands/open-github-issues.yaml
# Phrase ends with "github" so context "github" completes it
phrase: open issues github
title: Open GitHub Issues
action:
  type: open_url
  config:
    url: https://github.com/issues
```

```yaml
# commands/paste-team-emails.yaml
# Phrase ends with "team" so context "team" completes it
phrase: paste emails team
title: Paste Team Emails
action:
  type: paste_text
  config:
    text: "alice@example.com, bob@example.com"
```

**Usage (current model):**

```
/ctx set github
open issues       → effective: "open issues github" → matches ✅
```

```
/ctx set team
paste emails      → effective: "paste emails team" → matches ✅
```

### Phrase design that does NOT work with today's context

```yaml
# These phrases have the discriminating word in the middle:
phrase: open github issues      # context "github" won't help
phrase: paste team emails       # context "team" won't help
```

```
/ctx set github
open issues       → effective: "open issues github" → no match ❌
                    (because "open github issues" ≠ "open issues github")
```

---

## After (hypothetical — context at any position)

### Strategy A: Combinatorial Insertion

The engine tries inserting the context at every word boundary.

```yaml
# This phrase now works with context regardless of word position:
phrase: open github issues
title: Open GitHub Issues
action:
  type: open_url
  config:
    url: https://github.com/issues
```

```
/ctx set github
open issues       → tries: "github open issues", "open github issues", "open issues github"
                  → "open github issues" matches ✅
```

**BUT — ambiguity example:**

```yaml
# Command A
phrase: open github issues
title: Open GitHub Issues

# Command B  
phrase: open issues github
title: Open Issues on GitHub
```

```
/ctx set github
open issues       → matches BOTH commands — which one wins?
```

### Strategy B: Bag-of-Words

All tokens must be present, order ignored.

```yaml
phrase: open github issues
title: Open GitHub Issues
action:
  type: open_url
  config:
    url: https://github.com/issues
```

```
/ctx set github
open issues       → tokens: {open, issues, github} — all in phrase ✅
issues open       → tokens: {issues, open, github} — all in phrase ✅ (order lost!)
```

**Problem:** `issues open` also matches, which is confusing.

### Parameter extraction ambiguity

```yaml
phrase: search google
title: Search Google
action:
  type: open_url
  config:
    url: https://www.google.com/search?q={param}
```

**Current (clear):**
```
/ctx set rust programming
search google     → effective: "search google rust programming"
                  → phrase match: "search google"
                  → param: "rust programming" (everything after phrase) ✅
```

**Any-position (ambiguous):**
```
/ctx set google
search rust       → combinatorial: "google search rust", "search google rust", "search rust google"
                  → "search google rust" matches phrase "search google"
                  → param: "rust" ← but the user typed "search rust", so they might expect
                    "rust" to be the param and "google" to be part of the phrase...
                    or they might expect "search rust google" to match something else entirely.
```

The engine must decide: does the context fill a phrase gap, or does it become part of the param? This ambiguity doesn't exist in the current model.

---

## Recommendation

Users can avoid the need for any-position context by adopting a **simple authoring convention**: put the word you'd use as a context at the **end** of the phrase.

```yaml
# Instead of:
phrase: open github issues     # ❌ context "github" can't reach the middle

# Write:
phrase: open issues github     # ✅ context "github" appends naturally
```

This convention costs nothing and preserves the dead-simple matching rule.
