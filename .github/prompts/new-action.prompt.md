---
agent: agent
description: Implement a new built-in action type for Context Actions end-to-end — Rust backend, frontend, tests, docs, example config, and commit.
tools:
  - editFiles
  - runCommands
  - codebase
---

You are implementing a new built-in action type for the Context Actions launcher.
Work through the checklist below **in order**, completing each step fully before moving to the next.
Never skip a step. After each numbered step, confirm it is done before proceeding.

---

## Step 0 — Clarify requirements (STOP HERE first)

Before writing any code, ensure you have clear answers to:

1. **Action type name** — the `type` string used in YAML (e.g. `open_url`, `paste_text`). Must be snake_case.
2. **Config fields** — what fields does the YAML `config:` block contain? What are their types and defaults?
3. **Behaviour** — exactly what does the launcher do when the user presses Enter on this command?
4. **Argument support** — does this action accept a typed suffix (arg mode: `none` / `optional` / `required`)?
5. **Classification** — is this a **basic** action (self-contained, no extra files) or **advanced** (requires scripts, config files, or technical setup)?
6. **Result action** — if the action produces output (like `script_action`), which built-in does it pipe into?

If any of these are unclear, ask the user before proceeding.

---

## Step 1 — Rust backend (`src-tauri/src/commands.rs`)

Read the current `commands.rs` before making any changes.

- Add a new `<ActionName>Config` struct with `serde` derives.
- Add the new variant to the `Action` enum: `<ActionName> { config: <ActionName>Config }`.
- Handle the new variant in `load_from_dir` / any deserialisation logic.
- Add a new Tauri command function in `src-tauri/src/lib.rs` if user-triggered execution is needed.
- All new public Rust functions must have `#[cfg(test)]` unit tests in the same file:
  - Happy path (valid YAML parses correctly)
  - Edge cases (missing optional fields use defaults, invalid values return errors)
  - Security: reject path traversal (`../`) if the action involves file or script references

## Step 2 — Run tests

```bash
cargo test --manifest-path src-tauri/Cargo.toml
```

All tests must pass before continuing.

## Step 3 — Frontend types (`src/lib/types.ts`)

- Add a `<ActionName>Config` TypeScript interface mirroring the Rust struct.
- Add the new variant to the `Action` discriminated union: `| { type: "<action_name>"; config: <ActionName>Config }`.

## Step 4 — Frontend execution (`src/routes/+page.svelte`)

Read the current `+page.svelte` before editing.

- Add a new `else if (cmd.action.type === "<action_name>")` branch in `executeCommand()`.
- Wire up the Tauri `invoke()` call.
- Handle window dismiss / focus restore as appropriate for the action's UX (refer to how `paste_text` or `copy_text` does it).
- If the action has argument support, extract the suffix from `effectiveInput` using the existing pattern.

## Step 5 — Register the Tauri command

In `src-tauri/src/lib.rs`, add the new command function to the `.invoke_handler(tauri::generate_handler![...])` call.

## Step 6 — Run tests again

```bash
cargo test --manifest-path src-tauri/Cargo.toml
```

Confirm all tests still pass.

## Step 7 — Example config

- Create `example-config/commands/examples/<action-name>-example.yaml` with a minimal working command using the new action type.
- If the action requires additional files (lists, scripts), add them to `example-config/lists/` or `example-config/scripts/` as appropriate.
- Update `example-config/README.md`: add a row to the examples table documenting the new entry.
- Copy the same files to `~/Library/Application Support/ContextActions/` so the running app can exercise them immediately.

## Step 8 — Documentation

Determine the classification from Step 0:

**Basic action** → create `docs/using/basic/<action-name>.md`
- Cover: YAML schema (all fields), a minimal example, parameter behaviour if any, platform notes if any.
- Add a row to `docs/using/basic/README.md` linking to the new page.

**Advanced action** → create `docs/using/advanced/<action-name>.md`
- Cover: YAML schema, when to use this vs simpler alternatives, a minimal example, argument modes if applicable, security considerations.
- Add a row to `docs/using/advanced/README.md` linking to the new page.

Also update:
- `docs/using/configuring-commands.md` — add the new action type to the full YAML schema reference.
- `docs/using/README.md` — update the Actions table description if the classification changes the summary line.
- `README.md` — add the new action to the Built-in actions list in the Features section.

## Step 9 — Roadmap

In `docs/roadmap.md`:
- If the action was a planned roadmap item, mark it `[x]` and update its description to match the final implementation.
- If it was not on the roadmap, no change needed.

## Step 10 — Development plan

Add a new Stage entry to `docs/development-plan.md`:
- Add a row to the summary table at the bottom.
- Append a `## Stage N — <Title> ✅` section describing goal, tasks, and done-when criteria.

## Step 11 — Commit and push

```bash
cargo test --manifest-path src-tauri/Cargo.toml
git add -A
git commit -m "Stage N: add <action_name> action — <one-line summary>"
git push
```

Confirm all 56+ tests pass before committing.

---

## Reference: existing action pattern

When in doubt, follow the pattern of `script_action` for complex actions or `copy_text` for simple ones. Read those implementations in `commands.rs`, `lib.rs`, `types.ts`, and `+page.svelte` before writing new code.
