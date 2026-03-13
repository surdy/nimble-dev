# Config Directory — Hybrid B+D Approaches
_Date: 2026-03-13_

Follows on from `config-directory-layout.md`. The question: can we get the intuitive co-location of Option B **and** the portable bundle model of Option D in a single design?

---

## The tension

Option B's strength is that it requires **no new mental model** — files just live near each other.
Option D's strength is that a bundle is a **discrete, portable unit** you can zip up, share, or install.
These pull in slightly different directions: B is file-proximity, D is explicit grouping with a boundary.

---

## Approach 1 — Hierarchical resolution (lightweight, no explicit bundle concept)

Extend Option B's fallback chain. When resolving a list or script reference, the loader walks *up* from the command file's directory toward the `commands/` root before falling back to the global `lists/`/`scripts/` dirs.

```
commands/
  work/
    open-jira.yaml        # script: search.sh
    team-data.yaml        # list co-located (Option B style)
    scripts/
      search.sh           # scoped to work/ naturally
lists/                    # global fallback still works
scripts/                  # global fallback still works
```

Resolution order for `script: search.sh` when loading `commands/work/open-jira.yaml`:
1. `commands/work/search.sh` (same dir)
2. `commands/work/scripts/search.sh` (scripts subdir within grouping dir)
3. `scripts/search.sh` (global)

Users end up with bundle-like groupings just by organising files into subdirectories — no new concepts to learn.

### Tradeoffs

| Pro/Con | Note |
|---------|------|
| Pro | Fully backwards-compatible; zero migration |
| Pro | No new concepts — "files near each other work" |
| Pro | Gradually adoptable — start with global dirs, co-locate later |
| Con | Not truly exportable — no defined boundary; sharing means "copy this subdir and hope it's complete" |
| Con | Resolution order is implicit; debugging a wrong list/script being loaded requires understanding the hierarchy |
| Con | Mixes executables into the `commands/` tree — weakens the current clean security boundary |

---

## Approach 2 — Manifest-based bundles (explicit opt-in)

Introduce a lightweight `bundle.yaml` marker file. A directory containing `bundle.yaml` is a bundle — list and script references within it resolve locally first. Without the marker, a directory behaves as plain Option B (list co-location only, scripts still in global `scripts/`).

```
commands/
  team-email.yaml           # loose command, no bundle
  team-emails.yaml          # co-located list (Option B)
  work/
    bundle.yaml             # marker: this dir is a bundle
    open-jira.yaml
    jira-issues.yaml        # list, resolved locally because bundle
    search.sh               # script allowed here only because of bundle.yaml
```

`bundle.yaml` can start as an empty file or carry metadata (name, author, version) when a sharing ecosystem is added later.

### Tradeoffs

| Pro/Con | Note |
|---------|------|
| Pro | Clear boundary — "this directory is a unit" is declared, not inferred |
| Pro | Security: scripts inside `commands/` only permitted where explicitly declared via bundle marker |
| Pro | Extensible: `bundle.yaml` can gain metadata later without breaking existing setups |
| Pro | Option B co-location still works for unmarked dirs — two models coexist |
| Con | New concept to explain; non-technical users creating one command don't want to think about bundles |
| Con | An empty marker file feels like ceremony for no immediate gain unless sharing matters to the user |
| Con | Need to decide: can a bundle reference global lists/scripts as a fallback, or is it self-contained only? |

---

## Approach 3 — Parallel `bundles/` top-level directory (clean separation)

Keep `commands/` YAML-only (with Option B co-location for lists). Add a sibling `bundles/` top-level directory where each subdirectory is a self-contained unit with its own commands, lists, and scripts.

```
ContextActions/
  settings.yaml
  commands/             # loose commands (YAML + co-located list YAML)
    team-email.yaml
    team-emails.yaml
  bundles/              # self-contained units
    work/
      open-jira.yaml
      jira-issues.yaml
      search.sh
    personal/
      ...
  lists/                # global shared lists (backwards-compat)
  scripts/              # global shared scripts (backwards-compat)
```

### Tradeoffs

| Pro/Con | Note |
|---------|------|
| Pro | Cleanest security separation: `commands/` stays inert; `bundles/` is the only place executables live alongside YAML |
| Pro | Conceptually distinct: "loose commands" vs "packaged units" is a meaningful distinction |
| Pro | Sharing story is simple: zip a `bundles/work/` directory |
| Con | Users must decide upfront: "commands/ or bundles/?" — the choice isn't always obvious |
| Con | Adds another root-level directory, making the config root more complex |
| Con | `lists/` and `scripts/` become somewhat redundant alongside `bundles/` |
| Con | The two-paradigm question is a recurring confusion source; the distinction blurs over time |

---

## Overall take

- **If sharing/distribution is not a near-term goal:** Approach 1 gets most of the intuitive benefit with the least new surface area. Cost: slightly blurrier security boundary.
- **If portable bundles matter (even informally):** Approach 2 is the most principled path — opt-in boundary, security story stays clean, `bundle.yaml` can stay empty until metadata is needed.
- **Approach 3** is compelling but probably premature. The two-paradigm "commands/ vs bundles/" question is a recurring user confusion source, and `lists/`/`scripts/` end up in an awkward position once `bundles/` exists.
