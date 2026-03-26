# Config Examples: Per-Command Context Position

---

## Before (current model — context appended to input)

### Basic open URL

```yaml
phrase: open reddit
title: Open Reddit
action:
  type: open_url
  config:
    url: https://www.reddit.com
```

```
/ctx set reddit → user types "open" → effective "open reddit" → match ✅
```

### Search with param

```yaml
phrase: search google
title: Search Google for…
action:
  type: open_url
  config:
    url: https://www.google.com/search?q={param}
```

```
/ctx set rust programming → user types "search google" → effective "search google rust programming" → param "rust programming" ✅
```

### Multiple search engines require multiple commands

```yaml
# search-google.yaml
phrase: search google
title: Search Google for…
action:
  type: open_url
  config:
    url: https://www.google.com/search?q={param}
```

```yaml
# search-github.yaml
phrase: search github
title: Search GitHub for…
action:
  type: open_url
  config:
    url: https://github.com/search?q={param}
```

```yaml
# search-npm.yaml
phrase: search npm
title: Search npm for…
action:
  type: open_url
  config:
    url: https://www.npmjs.com/search?q={param}
```

Each engine is a separate YAML file. Context `google` narrows to the Google one.

---

## After (hypothetical — {context} placeholder in phrase)

### Basic open URL (opt-in context)

```yaml
phrase: open {context}
title: Open {context}
action:
  type: open_url
  config:
    url: https://www.{context}.com
```

```
/ctx set reddit → user types "open" → phrase expands to "open reddit" → match ✅
/ctx set github → user types "open" → phrase expands to "open github" → match ✅
```

One command replaces `open-reddit.yaml`, `open-github.yaml`, `open-google.yaml`, etc.

**But:** `url: https://www.{context}.com` only works for sites that match their domain name. `open slack` → `https://www.slack.com` works; `open notes` → `https://www.notes.com` doesn't.

### Mid-phrase context

```yaml
phrase: open {context} issues
title: Open {context} Issues
action:
  type: open_url
  config:
    url: https://{context}.com/issues
```

```
/ctx set github → user types "open issues" → phrase expands to "open github issues" → match ✅
```

This is the power case — context fills a slot in the middle of the phrase.

### Generic search command (the star use case)

```yaml
phrase: search {context}
title: Search {context}
action:
  type: open_url
  config:
    url: https://www.{context}.com/search?q={param}
```

```
/ctx set google → user types "search svelte" → phrase expands to "search google" → param "svelte"
                 → opens https://www.google.com/search?q=svelte ✅

/ctx set github → user types "search nimble" → phrase expands to "search github" → param "nimble"
                 → opens https://www.github.com/search?q=nimble ✅
```

**Three commands replaced by one.** This is compelling… but only for sites where the domain pattern holds.

### Commands without {context} — unaffected

```yaml
phrase: paste greeting
title: Paste polite greeting
action:
  type: paste_text
  config:
    text: |
      Hi,
      Thank you for reaching out.
```

No `{context}` in phrase → context is ignored for this command. Today this command would be affected by context (the effective input would be `"paste greeting reddit"`, which still matches because `"paste greeting"` is a substring). **Under the new model, this command loses context awareness.**

### Backward compatibility — fallback mode

To avoid breaking existing commands, a fallback rule could be:

> If a command has no `{context}` in its phrase, apply the old behaviour (append to end).

```yaml
# Old command (no {context}) — still works via append fallback
phrase: open reddit
title: Open Reddit
action:
  type: open_url
  config:
    url: https://www.reddit.com
```

```
/ctx set reddit → user types "open" → effective "open reddit" → match ✅ (via fallback)
```

But now there are **two different context mechanisms** running simultaneously, which is confusing.

---

## Alternative Recommendation: {context} in Action Config Only

Keep phrase matching as-is (append to end). Allow `{context}` substitution only in action config values:

```yaml
phrase: search google
title: Search Google for…
action:
  type: open_url
  config:
    url: https://www.google.com/search?q={param}
```

```yaml
phrase: paste greeting
title: Paste contextual greeting
action:
  type: paste_text
  config:
    text: "Hello from the {context} team!"
```

```
/ctx set engineering → user types "paste greeting" → pastes "Hello from the engineering team!" ✅
```

Phrase matching doesn't change. Context substitution only happens in `url:`, `text:`, and `prefix:`/`suffix:` fields. Scripts already get `NIMBLE_CONTEXT` as an env var. This is strictly additive — no matching changes, no backward breaks.
