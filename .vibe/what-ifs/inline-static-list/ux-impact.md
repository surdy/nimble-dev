# UX Impact: Inline Static Lists

## What Changes from the User's Perspective

### Creating a new list command

| Step | Current (external file) | With inline support |
|------|------------------------|-------------------|
| 1 | Create `commands/my-command.yaml` | Create `commands/my-command.yaml` |
| 2 | Create `lists/my-list.yaml` | _(done — items are in the command file)_ |
| 3 | Reference list name in command config | _(not needed)_ |

**Net effect**: One file instead of two for simple, single-use lists.

### Editing list data

| Scenario | Current | With inline |
|----------|---------|-------------|
| Small list (3–5 items) | Open a second file | Edit in place — data is right below the config |
| Large list (20+ items) | Dedicated file keeps things clean | Inline would clutter the command file — use external |
| Shared list (used by 2+ commands) | Edit one file, all commands update | Must use external `list:` reference (inline would mean duplicating) |

### Discoverability

| Aspect | Current | With inline |
|--------|---------|-------------|
| "Where are all my lists?" | `lists/` directory — clear, browsable | Some in `lists/`, some embedded in command files — scattered |
| "What lists does this command use?" | Must cross-reference the `list:` name | Self-evident — data is right there |

### Onboarding

- **Simpler getting-started story**: "Create one YAML file, put your items in it, done"
- **No need to explain `lists/` directory** for the first tutorial — can introduce it later as an "advanced: sharing lists" topic
- **Copy-paste friendly**: A single file can be shared in a Discord message, gist, or docs example

### Migration

- **Zero migration needed** — this is purely additive
- Existing configs with `list:` references continue to work unchanged
- Users adopt inline only when creating new commands, at their own pace

### The `lists/` directory

- Remains fully functional and documented
- Becomes **optional** rather than mandatory for all static list commands
- Still recommended for: shared lists, large data sets, separation of concerns

## Risk: Inconsistency Over Time

If a user mixes inline and external lists freely, their config directory could become harder to reason about — "is the data in the command file or in `lists/`?" This is a minor concern that documentation can address with a simple guideline:

> **Rule of thumb**: Use inline `items` for small, single-use lists. Use an external `list:` file when the list is large or shared across multiple commands.
