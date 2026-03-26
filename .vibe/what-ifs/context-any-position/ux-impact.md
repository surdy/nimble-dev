# UX Impact: Context at Any Position

## What changes from the user's daily perspective

### 1. Phrase authoring becomes less constrained (benefit)

Users no longer need to think about word order when designing phrases. They can write `open github issues` naturally and rely on the context to fill `github` regardless of where it appears. This removes a subtle design constraint that non-power-users might find surprising.

### 2. Match results become less predictable (cost)

With the current system, a user can always predict what will match: take what I typed, add the context to the end, and that's what matches. With any-position matching, the user must reason about multiple possible insertions. When two commands share the same words in different orders, the user cannot predict which one wins.

**Example of confusion:**

- Commands: `open github issues`, `open issues tracker`
- Context: `issues`
- User types: `open github`
- Effective tokens: `open`, `github`, `issues`

Does this match `open github issues` (exact positional match) or `open issues tracker` (has all tokens via bag-of-words but wrong meaning)? The answer depends on the strategy, and users shouldn't need to know which strategy is in play.

### 3. Debugging becomes harder

When a command doesn't match, users currently check: "does my input + context equal a substring of the phrase?" With any-position matching, the debugging question becomes: "is there any permutation of my input + context that matches?" — a much harder thing to reason about.

### 4. The context chip becomes less informative

The context chip currently shows the active context (e.g., `reddit`), and users know it will be tacked onto the end. With any-position matching, seeing `github` in the chip doesn't tell the user *where* it will be inserted. The chip goes from informative to ambiguous.

### 5. Param commands become tricky

For `open_url` commands with `{param}`, the user currently has a clear flow:

1. Type the phrase: `search google`
2. Keep typing after a space: the rest is the search query
3. If context is active, it appends to step 2

With any-position matching, the distinction between "phrase completion" and "param text" blurs. If the user types `search rust` with context `google`, does `google` complete the phrase (making `rust` the param) or does `rust` become part of the phrase attempt? The engine must make invisible decisions.

### 6. No config or YAML changes needed

The YAML schema itself wouldn't change — phrases, titles, and actions stay the same. The change is entirely in the match algorithm. This is good (no migration burden) but also means there's no opt-in/opt-out mechanism per command.

---

## Workflow scenarios

### Scenario A: Power user with well-organized phrases _(no benefit)_

A user who already orders phrases with context-friendly word order (discriminator at the end) gets zero benefit. Their phrases already work perfectly with append-to-end.

### Scenario B: New user writing phrases "naturally" _(small benefit)_

A user who writes `open github issues` instead of `open issues github` would find that contexts "just work" without needing to learn the authoring convention. However, they'd also encounter confusing matches as their command set grows.

### Scenario C: User with overlapping phrases _(negative experience)_

A user with commands like `open github`, `open github issues`, and `search github repos` would find that context `github` matches in unpredictable ways depending on what they type. The current system's predictability is lost.

---

## Summary

| Aspect | Current (append) | Any-position |
|--------|------------------|-------------|
| Mental model | 1 rule, trivially predictable | Multiple strategies, hard to predict |
| Phrase authoring | Must put context-word last | Free word order |
| Param extraction | Clean boundary | Ambiguous |
| Debugging | Easy | Hard |
| False positives | None | Possible |
| Config migration | N/A | None needed |
| Context chip clarity | High | Lower |
