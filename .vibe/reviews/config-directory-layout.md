# Config Directory Layout — Review
_Date: 2026-03-13_

## Current layout

```
ContextActions/
  settings.yaml
  commands/
    examples/
      …
    (user files & subdirs)
  lists/
    team-emails.yaml
  scripts/
    hello.sh
    uuid.sh
```

---

## What works well

**`commands/` as a dedicated watched directory** is solid. Recursive watch + subdirectory freedom gives users a natural file-system organisation model and the 300 ms hot-reload feels native.

**Separation of commands from lists and scripts** is correct conceptually — a command YAML is a declaration, a list file is data, a script is executable code. Mixing them in one directory would make the watcher logic messier and permissions harder to reason about.

---

## What feels off

### 1. `lists/` is a sibling of `commands/`, but it's just data referenced *by* commands

A list file only exists in relation to the command that names it. A user creating a `static_list` command must remember to put the list in a completely separate directory. There's no co-location signal. Compare:

```
# Current — two files in unrelated dirs
commands/snippets/team-email.yaml   # action.config.list: team-emails
lists/team-emails.yaml
```

If a user renames or deletes the list, there's nothing in the command dir to tell them it's missing. The split also makes backup/export fragile — it's easy to copy `commands/` and forget `lists/`.

### 2. `scripts/` has a flat namespace, but `commands/` supports subdirectories

Users can organise commands into `commands/work/`, `commands/personal/` etc., but scripts must all live flat in `scripts/`. A user with 20 scripts ends up with a cluttered flat directory while their commands are neatly grouped.

### 3. `settings.yaml` at the root is fine for now, but doesn't scale

When themes, contexts, or plugin settings are added, the root will accumulate more files. Most launchers (Alfred, Raycast) put settings inside a dedicated subdirectory or use a structured config format with sections.

---

## Options

### Option A — Keep current layout (status quo)

```
ContextActions/
  settings.yaml
  commands/
  lists/
  scripts/
```

**Pros:** Already implemented; users on early versions won't need migration; conceptually clean separation by type.
**Cons:** `lists/` co-location problem; flat `scripts/`; settings at root doesn't scale; backup/export requires knowing about 3 separate dirs.

---

### Option B — Co-locate lists inside `commands/`, allow scripts subdirectories *(recommended)*

```
ContextActions/
  settings.yaml
  commands/
    team-email.yaml          # references list: team-emails
    team-emails.yaml         # the list data, lives next to the command
    work/
      jira.yaml
  scripts/
    work/
      jira-search.sh         # scripts/ mirrors commands/ structure
```

The loader looks for list files relative to the command file's own directory first, falling back to `lists/` for backwards compatibility. Scripts continue to use flat name resolution but subdirectories are permitted.

**Pros:** Commands and their data travel together; easier backup; more intuitive for non-technical users; mirrors how VS Code tasks and snippets co-locate; fully backwards-compatible — no migration needed for existing `lists/` files.
**Cons:** A `.yaml` file in `commands/` now has two meanings (command or list data) — the watcher needs to distinguish them by schema presence; breaks the current clean type separation.

---

### Option C — Single `commands/` directory, embed everything

```
ContextActions/
  settings.yaml
  commands/
    work/
      jira.yaml
      jira-list.yaml
      jira-search.sh
```

Scripts and lists live inside the `commands/` tree.

**Pros:** One directory to back up; natural co-location; mirrors how Alfred workflow bundles work.
**Cons:** Mixes executable code and data in the watched directory — security boundary becomes harder to enforce (anything executable in `commands/` could be run); watcher must filter carefully; harder to explain to users what goes where.

---

### Option D — `packages/` or `bundles/` model (future-oriented)

```
ContextActions/
  settings.yaml
  packages/
    work/
      commands.yaml
      list-data.yaml
      search.sh
    personal/
      commands.yaml
```

A "package" is a self-contained unit that can be shared, exported, or installed from a registry.

**Pros:** Most scalable; enables community sharing; maps closely to how Raycast extensions work.
**Cons:** Significant implementation change; adds conceptual overhead for non-technical users who just want to add one command; premature unless a package/plugin ecosystem is planned.

---

## Recommendation

**Option B before v1.0**, specifically:

1. Allow list files to live next to the command file that references them — look in the command file's own directory first, fall back to `lists/` for backwards compatibility. Zero migration needed for existing setups.
2. Allow `scripts/` to mirror subdirectory structure — the current flat name resolution still works; subdirs are just permitted.
3. Keep `lists/` and `scripts/` as the canonical locations for shared/reusable assets referenced by multiple commands.

This resolves the co-location problem without the security concerns of Option C or the conceptual weight of Option D, and is fully backwards-compatible.

The `settings.yaml`-at-root issue is low priority until a second settings concern appears.
