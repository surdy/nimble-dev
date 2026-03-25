# Nimble Agent Conventions

Rules and workflows that apply to all Nimble Copilot agents.

## Canonical Spec

All schema knowledge lives in `nimble-spec.yaml` (same directory as this file).
**Always read `nimble-spec.yaml` before answering schema questions or generating
command/script files.** Do not rely on memorised schema — the spec file is the
single source of truth and is updated as the app evolves.

## File Placement

- Command YAML files go inside `commands/` under the config directory.
- One command per `.yaml` file.
- Commands that reference a script or list file get their own subdirectory
  (co-located pattern — see `co_location` in the spec).
- Simple commands (open_url, paste_text, copy_text) can live as standalone
  `.yaml` files at any nesting depth under `commands/`.

## Reserved Phrases

- Phrases starting with `/` are reserved for built-in commands.
- Never create command files with a `/`-prefixed phrase.

## Testing

- After creating or editing a command, verify the YAML is valid.
- After writing a script, run it in the terminal to confirm output format.
- Validate JSON output with `python3 -m json.tool` or equivalent.

## Agent Boundaries

| Agent | Owns | Does NOT touch |
|-------|------|----------------|
| `@nimble-command` | Command YAML, list TSV, env.yaml | Scripts, Rust code, frontend code |
| `@nimble-script` | Script files (sh, py, js, ps1) | Command YAML, env.yaml, settings.yaml, Rust/frontend code |

When a task spans both domains, `@nimble-command` delegates script work to
`@nimble-script` with a clear description of the required output format,
arguments, and action context.

## Updating the Spec

When the Nimble command schema, settings schema, script interface, or
environment variable API changes:

1. Update `nimble-spec.yaml` to reflect the change.
2. Add a changelog entry at the bottom of the spec with the date and
   a one-line description of what changed.
3. Bump `spec_version` for any of these changes:
   - A field is removed or renamed
   - A field's type or semantics change
   - A new action type is added
   - A new environment variable is added
   - A new config field is added to any action or settings
   - The script interface changes (output format, timeout, arg modes)
4. When `spec_version` is bumped, update the `spec_version` field in
   **every** agent file (`nimble-command.agent.md`, `nimble-script.agent.md`)
   to match. Review each agent's instructions for accuracy against the
   new spec before updating their version number.
