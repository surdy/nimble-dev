# What If: Static list files use TSV instead of YAML

## The Problem

Static list files currently use YAML format — the same format as command files. This creates two issues:

1. **Parsing confusion** — Nimble's file walker tries to parse list YAML files as commands, producing log errors (covered in the [filename convention what-if](../command-filename-convention/summary.md)).
2. **Authoring overhead** — YAML list syntax (`- title: ... subtext: ...`) is verbose for what is essentially a two-column table. Users coming from spreadsheets or text files may find it unfamiliar.

TSV (tab-separated values) is the simplest possible tabular format — one item per line, tab between columns. This analysis explores whether switching to TSV would be a net positive.

## Options Explored

### Option A — TSV with implicit columns (`title<TAB>subtext`)

List files use `.tsv` extension. Each line is one item. Tab separates title from subtext. Lines without a tab have title only (subtext is null). Empty lines and `#`-prefixed lines are ignored.

```tsv
Alice Smith	alice@example.com
Bob Jones	bob@example.com
Carol White
```

**Pros:**
- Extremely simple to author — paste from a spreadsheet, write in any text editor
- No YAML syntax to learn (no `- title:`, no indentation rules)
- Different file extension (`.tsv`) means the YAML command loader naturally skips list files — **solves the parse-error noise problem for free**
- Smaller file size for large lists
- Easy to generate from scripts, `cut`, `awk`, etc.
- Copy-paste from Excel/Google Sheets produces TSV by default

**Cons:**
- **Breaking change** — existing list files must be converted from YAML to TSV
- Tabs are invisible in most editors — users might accidentally use spaces
- No support for multi-line values (titles/subtexts with newlines)
- Less structured than YAML — no room for future fields without a format redesign
- Two different formats in the config dir (YAML for commands, TSV for lists) — cognitive load

### Option B — TSV with header row

Same as Option A but requires a header row:

```tsv
title	subtext
Alice Smith	alice@example.com
Bob Jones	bob@example.com
```

**Pros:**
- Self-documenting — the header tells you what each column is
- Easier to validate and gives better error messages

**Cons:**
- Header is boilerplate for a two-column format that never changes
- Extra line in every file
- Must detect/skip the header during parsing

### Option C — Keep YAML, but change extension to `.list.yaml`

Keep the current YAML format but use a `.list.yaml` extension so the command loader can skip list files by extension.

**Pros:**
- No format change — existing content works after a simple rename
- Solves the parse-error noise problem
- YAML remains the only format users need to learn

**Cons:**
- Still verbose YAML syntax for tabular data
- Rename still required (breaking change, but minimal)
- `list:` field resolution needs to append `.list.yaml` instead of `.yaml`

### Option D — Support both YAML and TSV (detect by extension)

If the resolved file ends in `.tsv`, parse as TSV. If it ends in `.yaml`/`.yml`, parse as YAML. Users choose whichever they prefer.

**Pros:**
- No breaking change — existing YAML lists keep working
- Users who prefer TSV can adopt it for new lists
- Gradual migration path

**Cons:**
- Two parsers to maintain
- Inconsistent config dir ("why are some lists YAML and some TSV?")
- Documentation must cover both formats
- More code surface area for bugs

## Config Impact

### Current YAML format
```yaml
# team-emails.yaml
- title: Alice Smith
  subtext: alice@example.com

- title: Bob Jones
  subtext: bob@example.com

- title: Carol White
  subtext: carol@example.com
```

### Proposed TSV format (Option A)
```
# team-emails.tsv
Alice Smith	alice@example.com
Bob Jones	bob@example.com
Carol White
```

### Command YAML change
```yaml
# Before
action:
  type: static_list
  config:
    list: team-emails        # resolves to team-emails.yaml

# After (Option A — extension changes)
action:
  type: static_list
  config:
    list: team-emails        # resolves to team-emails.tsv
```

### Dynamic list scripts — no change
`dynamic_list` scripts already output **JSON**, not YAML. TSV would only affect static list data files. The script interface is unaffected.

## UX Impact

### Authoring experience

| Task | YAML (current) | TSV (proposed) |
|------|----------------|----------------|
| Create a 5-item list | Write 10+ lines of YAML with precise indentation | Write 5 lines, paste from spreadsheet |
| Add an item | Add 2-3 YAML lines with correct indent | Add 1 line |
| Copy from spreadsheet | Manually reformat into YAML objects | Paste directly (Excel copies as TSV) |
| Edit in a text editor | YAML syntax highlighting helps | Tabs may be invisible depending on editor |
| Include comments | `#` comments are valid YAML | Would need explicit `#` skip logic |
| Multi-line values | Supported via YAML block scalars | Not supported |
| Future extensibility | Add new fields freely | Locked to 2 columns without a redesign |

### Common user profiles

**Spreadsheet users:** TSV is a clear win — they can paste directly from Excel/Sheets.

**Developer users:** YAML is familiar, but they'd likely appreciate the simplicity. Most dev tooling already formats TSV well.

**Power users with large lists:** TSV is dramatically more compact — a 100-item list goes from ~400 lines of YAML to 100 lines of TSV.

## Opinion

**This is a mixed proposition — I'd lean toward Option D (support both) if doing it at all.**

TSV is genuinely better for the *authoring* experience of list files. The current YAML format is overkill for what is essentially a two-column table. YAML's `- title: ... subtext: ...` syntax adds real friction, especially for non-technical users who just want to maintain a list of contacts, shortcuts, or snippets.

However, **switching exclusively to TSV (Option A) has meaningful downsides:**

1. **Breaking change** for a format that works today
2. **Invisible tabs** are a real footgun — a single space instead of a tab silently breaks the item, and many editors don't make tabs visually obvious
3. **Extensibility ceiling** — if list items ever need a third field (e.g., an icon, a category, a per-item action override), TSV becomes awkward. YAML handles this gracefully.
4. **Two formats** in the config dir adds cognitive load ("commands are YAML, lists are TSV" is one more thing to remember)

The **parse-error noise problem** (the original motivation) is better solved by the approaches in the [filename convention analysis](../command-filename-convention/summary.md) — skip `env.yaml` by name and downgrade parse failures to debug-level logging. That eliminates the noise without changing any file formats.

If the real goal is **easier list authoring**, Option D (support both, detect by extension) would let users choose TSV when it's convenient while keeping YAML as the default, documented format. But even then, I'd question whether the added complexity is worth it — the number of users who maintain large static lists is likely small, and they're likely technical enough to write YAML comfortably.

**Bottom line:** TSV solves a real authoring friction, but the parse-error noise problem has a simpler fix that doesn't require a format change. I'd only pursue TSV if user feedback shows that list authoring is a genuine pain point, and then via Option D (dual support) rather than a hard switch.
