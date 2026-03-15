# Profiles — UX Impact

## What changes from the user's daily workflow

### Setting up profiles

**Option A (directory-based):**
1. User creates `profiles/work/commands/` and `profiles/home/commands/` directories
2. User moves existing command files into the appropriate profile directories
3. Commands that should be available everywhere go in `profiles/default/commands/` (or `profiles/shared/commands/`)
4. Restart or wait for live reload

Migration effort: **high** — every existing user must reorganise.

**Option B (tag-based) — recommended:**
1. User adds `profiles: [work]` to command files that should be work-only
2. User adds `profiles: [home]` to command files that should be home-only  
3. Commands without a `profiles:` field continue working everywhere — zero migration for existing commands
4. Live reload picks up the tags immediately

Migration effort: **zero** — fully backward-compatible.

---

### Switching profiles

Both options use the same built-in commands:

```
/profile set work       → switch to "work" profile
/profile reset          → clear profile, load all commands
/profile show           → display current profile
```

After switching:
- Commands are reloaded immediately (filtered/scoped)
- The profile persists across restarts (stored in settings.yaml or localStorage)
- Context remains independent — you can have profile `work` with context `backend-team`

**New UX element:** a profile indicator could appear in the launcher bar (similar to the context chip), showing the active profile name. Unlike the context chip, this would be more passive — profiles change less frequently than contexts.

---

### Daily usage after setup

**No functional change to how you type commands.** Once the profile is active, the launcher shows only the commands available in that profile. The typing, matching, and execution experience is identical.

The key daily workflow change: **fewer commands in the results list.** With 50 commands split across work/home, each profile might show only 30. This means less noise, faster matching, and more focused results.

---

### Overlap with contexts

Users will naturally ask: "What's the difference?"

| Aspect | Context | Profile |
|--------|---------|---------|
| What it does | Appends a suffix to matching | Filters which commands are loaded |
| Granularity | Per-keystroke matching refinement | Entire command set swap |
| Typical lifetime | Minutes to hours | Hours to days |
| User action | `/ctx set reddit` | `/profile set work` |
| Effect on command files | None — all commands stay loaded | Commands without the matching profile tag are hidden |
| Persistence | localStorage (survives restart) | settings.yaml (survives restart) |

**They compose:** Profile = "which commands exist", Context = "how matching works within those commands."

---

### Edge cases and potential confusion

1. **"I tagged a command with `profiles: [work]` but I'm on the `home` profile — where did it go?"**
   - The command is silently excluded from loading. No error, no warning. This is the intended behaviour but may surprise first-time profile users.
   - Mitigation: `/profile show` makes the active profile visible; the profile chip (if added) provides constant awareness.

2. **"I have no active profile — do tagged commands load?"**
   - **Option B recommended behaviour:** when no profile is active (default state), ALL commands load regardless of tags. This preserves backward compatibility and the "zero config" experience.
   - Alternative: only untagged commands load. This is cleaner but breaks existing users who add a `profiles:` tag to one file and suddenly lose other tagged commands.

3. **"Can I be on two profiles at once?"**
   - Recommend: **no.** Single active profile only. Multi-profile (`active_profile: [work, freelance]`) adds combinatorial complexity and confusing overlap. Users who need commands from two profiles should tag those commands with both profiles.

4. **"What about profile-specific hotkeys or settings?"**
   - Recommend: **defer.** Start with profile = command filter only. Same hotkey, same UI settings, same `allow_external_paths`. If users request profile-specific settings later, add `settings.yaml` inside profile directories (Option A) or a `profiles:` section in the root `settings.yaml` (Option B).

5. **"Scheduled profile switching (e.g. 'work' M–F 9am–5pm)"**
   - Recommend: **defer to a later stage.** Manual switching covers 90% of the use case. Scheduling adds cron-like complexity, timezone handling, and "what if I'm typing when the switch happens?" edge cases.
