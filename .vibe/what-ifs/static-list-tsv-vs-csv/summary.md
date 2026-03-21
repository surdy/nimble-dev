# What If: TSV vs CSV for Static List Files (Breaking Change Accepted)

## Constraints
- Breaking change is **accepted** — no backward compatibility concern
- **Human editability** is the top priority
- List items have exactly **two fields**: title (required) and subtext (optional)
- The format also solves the parse-error noise problem for free (different extension → loader skips it)

## Head-to-Head: TSV vs CSV

### Editing experience

| Scenario | TSV | CSV |
|----------|-----|-----|
| Write by hand in a text editor | Tab key once between columns | Must remember quoting rules |
| Title contains a comma | Just works — commas aren't special | Must quote the field: `"Smith, Alice",alice@example.com` |
| Title contains a tab | Extremely rare (never in practice) | Just works — tabs aren't special |
| Title contains a quote | Just works | Must escape: `"She said ""hi""",value` |
| Paste from spreadsheet | Direct paste — spreadsheets copy as TSV | Need to export/save as CSV first |
| Copy from launcher docs/chat | Tabs survive copy-paste in most editors | Commas are fine, but quoting may not survive |
| Edit in Excel/Sheets | Open directly, saves back cleanly | Opens directly, but quoting/escaping on save can mangle data |
| Visibility of separator | **Invisible** — tabs look like whitespace | **Visible** — commas are obvious |
| Accidental wrong separator | Space instead of tab → silent failure | Period instead of comma → rare, but possible |

### The critical tradeoff

**TSV wins on simplicity** — no quoting rules, no escaping, no edge cases. A TSV file is literally "type title, press tab, type subtext, press enter." There is nothing else to learn.

**CSV wins on visibility** — you can *see* the comma. Tabs are invisible in almost every text editor. A user looking at a TSV file cannot tell if the separator is a tab or spaces without toggling "show whitespace" in their editor.

### Real-world data characteristics

For Nimble list items specifically:

| Field | Likely contains commas? | Likely contains tabs? | Likely contains quotes? |
|-------|------------------------|----------------------|------------------------|
| `title` (display name) | **Yes** — `"Smith, Alice"`, `"Copy, paste, done"` | No | Rarely |
| `subtext` (email, URL, value) | **Yes** — URLs with query params (`?a=1,2`), multi-value strings | No | Rarely |

**Commas appear naturally in both fields.** This is the strongest argument against CSV — the moment a title or subtext contains a comma, the user must learn quoting rules:

```csv
# CSV — title has a comma → must quote
"Smith, Alice",alice@example.com
Bob Jones,bob@example.com
```

```tsv
# TSV — title has a comma → just works
Smith, Alice	alice@example.com
Bob Jones	bob@example.com
```

### The invisible-tab problem

The main argument against TSV is that tabs are invisible. In practice:

1. **Most modern editors render tabs distinctly** — VS Code shows tab stops with indentation guides, and users can enable "Render Whitespace: all" to see them explicitly.
2. **If a user uses spaces instead of a tab**, the entire line becomes the title (subtext is null). This is a **silent failure** — the item appears but selecting it pastes the wrong value. This is the worst-case scenario.
3. **Mitigation**: Nimble could log a warning when a list item contains no tab but has spaces that look like they might be a separator (heuristic). Or the documentation can emphasize "press Tab, not Space."

### Format comparison for the same data

**5-item contact list:**

#### YAML (current)
```yaml
- title: Alice Smith
  subtext: alice@example.com

- title: Bob Jones
  subtext: bob@example.com

- title: Carol White, MD
  subtext: carol@example.com

- title: Dave Lee
  subtext: dave@example.com

- title: Eve Park

```
15 lines, must know YAML array syntax and indentation.

#### CSV
```csv
Alice Smith,alice@example.com
Bob Jones,bob@example.com
"Carol White, MD",carol@example.com
Dave Lee,dave@example.com
Eve Park
```
5 lines, but Carol's entry requires quoting because her title has a comma.

#### TSV
```
Alice Smith	alice@example.com
Bob Jones	bob@example.com
Carol White, MD	carol@example.com
Dave Lee	dave@example.com
Eve Park
```
5 lines, no quoting needed for Carol, commas are just text.

## Opinion

**TSV is the better choice**, and it's not close.

The invisible-tab concern is real but manageable. The comma-in-data concern with CSV is **not** manageable — it forces users to learn quoting rules (`"field with, comma"`) and escaping (`""` for literal quotes), which is exactly the kind of syntax friction we're trying to eliminate by moving away from YAML.

The whole point of this change is to make lists trivially editable by humans. CSV's quoting rules undermine that goal. A user who puts `Smith, Alice` in a CSV file without quoting it gets **silently broken data** (three fields instead of two) — which is arguably worse than TSV's invisible-tab problem (where at least you get the full text as the title, just missing the subtext split).

TSV also wins the spreadsheet workflow: select cells in Excel/Sheets → Cmd+C → paste into `.tsv` file → done. CSV requires an export step.

**If I were designing this from scratch with human editability as the primary goal, I'd pick TSV with these mitigations:**

1. **Use `.tsv` extension** — makes the format obvious and lets the command loader skip these files naturally
2. **Support `#` comments** — users expect them in config files
3. **Log a hint** if a list item has no tab but contains multiple space-separated words — catches the spaces-instead-of-tab mistake
4. **Document it clearly** — "press Tab between title and value, not Space"
