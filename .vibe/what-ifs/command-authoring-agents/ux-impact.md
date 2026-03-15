# UX Impact: AI Agents for Command Authoring

## User Workflow With Two Agents

### Scenario 1: Simple command (basic user)

> User: "@nimble-command I want a command that opens my company's Jira board"

The command agent asks what phrase they want, generates the YAML, and writes it to the commands directory. No script needed — no delegation.

**Result:** One YAML file, one interaction, done.

### Scenario 2: Script-backed command (power user)

> User: "@nimble-command I want a command that searches my contacts CSV file and lets me paste the email"

The command agent:
1. Generates the command YAML (`dynamic_list` with `arg: optional`, `item_action: paste_text`)
2. Delegates to `@nimble-script` as a subagent: "Write a shell script that reads a CSV, filters by $1, and outputs JSON array with title=name, subtext=email"
3. Returns both files to the user

**Result:** One YAML file + one script, single user interaction.

### Scenario 3: Script debugging (developer)

> User: "@nimble-script My contacts.sh script returns empty output — help me debug it"

The script agent reads the script, checks for common issues (wrong output format, missing shebang, no execute permission, argument handling), and fixes it. No YAML knowledge needed.

**Result:** Fixed script, focused interaction.

### Scenario 4: Static list creation

> User: "@nimble-command I want a dropdown of our team's Slack channels that copies the channel ID when I pick one"

The command agent generates the command YAML (`static_list` with `item_action: copy_text`) and the list YAML file. No script involved at all.

**Result:** Two YAML files (command + list), one interaction.

## What Changes for Users

### Daily workflow
- Users get a natural-language interface for creating commands instead of reading docs and writing YAML manually
- The agent knows the full schema, validates field combinations, and prevents common mistakes (e.g., using `item_action` on `open_url`, forgetting `arg` on `dynamic_list`)
- Script generation handles platform differences automatically (sh vs PowerShell, JSON formatting)

### Learning curve
- Users don't need to learn the YAML schema or script output format upfront — they describe what they want and get working files
- The agent can explain what it generated, serving as interactive documentation

### What doesn't change
- File structure stays the same (YAML + optional script/list, co-located)
- Manual editing is still fully supported — agents are additive, not mandatory
- All existing commands continue working unchanged

## Agent Interaction Model

```
User ──→ @nimble-command ──→ Writes YAML
              │
              ├──→ (if script needed) ──→ @nimble-script (subagent) ──→ Writes script
              │
              └──→ (if list needed) ──→ Writes list YAML directly (no subagent needed)

User ──→ @nimble-script ──→ Writes/debugs script directly
```

## Edge Cases

| Situation | Handling |
|-----------|----------|
| User asks @nimble-script to create a full command | Script agent should say "I only write scripts — use @nimble-command for the full command setup" |
| User asks @nimble-command for a complex multi-step pipeline | Command agent creates the YAML, delegates script writing, handles env.yaml if needed |
| User wants `${VAR}` external paths | Command agent handles this — it's YAML config, not script logic |
| User wants to debug env variable injection | Could go either way — command agent for env.yaml/inline env config, script agent for reading NIMBLE_* vars in the script |
