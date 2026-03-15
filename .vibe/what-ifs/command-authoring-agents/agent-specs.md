# Agent Specifications: Nimble Command Authoring

## Agent 1: `nimble-command.agent.md`

```yaml
---
description: "Create, edit, or debug Nimble launcher commands. Use when: writing YAML command files, configuring open_url paste_text copy_text static_list dynamic_list script_action commands, setting up env.yaml variables, creating list files, configuring contexts, troubleshooting command matching."
tools: [read, edit, search, agent]
---
```

### Knowledge the agent needs

- Full YAML schema (all 6 action types with every config field)
- Config directory structure (platform paths, commands/ layout, co-location rules)
- env.yaml layering (global → sidecar → inline, NIMBLE_ reserved prefix)
- ${VAR} substitution in script/list paths
- settings.yaml fields and their effects
- Contexts (ctx set/reset, effective input matching)
- Phrase rules (no / prefix, uniqueness, partial matching behavior)
- File naming conventions (one command per .yaml file, subdirectory organization)
- When to delegate to @nimble-script for script generation

### What it should NOT do

- Write script logic (delegate to nimble-script)
- Modify Rust source or frontend code
- Change settings.yaml without user confirmation

---

## Agent 2: `nimble-script.agent.md`

```yaml
---
description: "Write, debug, or improve scripts for Nimble dynamic_list and script_action commands. Use when: writing shell scripts, Python scripts, Node.js scripts, PowerShell scripts for Nimble, debugging script output format, fixing JSON output, handling NIMBLE_* environment variables, script timeout issues, argument handling."
tools: [read, edit, search, execute]
---
```

### Knowledge the agent needs

- Output format: JSON array `[{"title":"...","subtext":"..."}]` or plain text
- Argument passing: `$1` (sh), `sys.argv[1]` (Python), `process.argv[2]` (Node.js)
- Built-in NIMBLE_* variables: NIMBLE_CONTEXT, NIMBLE_PHRASE, NIMBLE_CONFIG_DIR, NIMBLE_COMMAND_DIR, NIMBLE_OS, NIMBLE_VERSION
- User-defined env vars (available as regular env vars in the script)
- 5-second timeout constraint
- Platform differences: sh/bash on macOS/Linux, PowerShell on Windows
- Script must be executable (chmod +x on Unix)
- stderr is logged but never shown to user
- Security: scripts produce output only, cannot trigger system actions
- Co-location: script lives next to its command YAML (or via ${VAR} external path)

### What it should NOT do

- Write or modify YAML command files (that's nimble-command's job)
- Modify env.yaml or settings.yaml
- Run scripts with elevated privileges or modify system state

---

## Tool Allocation Rationale

| Agent | Tools | Why |
|-------|-------|-----|
| `nimble-command` | `read, edit, search, agent` | Reads existing commands, writes YAML, searches for patterns, delegates to nimble-script |
| `nimble-script` | `read, edit, search, execute` | Reads scripts, writes scripts, searches for patterns, runs scripts to test output |

Key difference: command agent gets `agent` (for subagent delegation), script agent gets `execute` (for testing scripts in terminal). Neither gets the other's distinctive tool.
