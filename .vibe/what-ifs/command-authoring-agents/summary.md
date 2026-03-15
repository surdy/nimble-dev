# What If: AI Agents for Nimble Command Authoring

## Question

Should we create one or multiple VS Code agents to help users write Nimble commands, scripts, and lists? Should there be a routing agent that delegates to specialists?

## Recommendation: Two agents (not three)

A **router agent is unnecessary**. VS Code's agent system already handles discovery and delegation via `description` keywords — adding a third "dispatcher" agent just adds a layer of indirection that burns tokens without value. The user already picks the agent from the agent selector, and parent agents already route to subagents based on description matching.

**Two agents** cover the entire surface area cleanly:

### 1. `nimble-command` — Command Author (user-invocable)

The primary agent. Handles **all six action types** — basic (`open_url`, `paste_text`, `copy_text`) and advanced (`static_list`, `dynamic_list`, `script_action`). This is the agent users invoke directly.

**Why not split basic/advanced?** Because:
- The YAML schema is the same structure for all types — `phrase`, `title`, `action.type`, `action.config`. Splitting by complexity creates two agents with 90% shared knowledge.
- Most user requests naturally blend levels: "I want a command that searches Jira and pastes the ticket URL" spans basic (open_url concepts) and advanced (script_action) in one sentence.
- The command author can handle simple requests in 3 lines of YAML and complex requests with scripts — the difficulty scales with the user's intent, not the agent's specialization.

### 2. `nimble-script` — Script Writer (user-invocable, also subagent)

A focused agent that **only writes scripts** (shell, Python, Node.js, PowerShell). It knows the output format (JSON array or plain text), the `NIMBLE_*` environment variables, the 5-second timeout, the argument-passing convention, and platform differences.

**Why separate this?** Because:
- Script writing is a fundamentally different task than YAML authoring. It requires knowledge of shell syntax, JSON formatting, error handling, platform differences (sh vs PowerShell), and the `NIMBLE_*` env API.
- The `nimble-command` agent can invoke `nimble-script` as a subagent when a request requires both a YAML file and a script.
- Users can also invoke `nimble-script` directly when they already have a command YAML but want to improve or debug their script.

## Why not a router?

| Approach | Pros | Cons |
|----------|------|------|
| **Router → specialists** | Clear separation | Extra agent burns tokens just to route; user must start with the router instead of the right agent; VS Code already does description-based routing |
| **Two peer agents** | Direct invocation; subagent delegation built-in; no wasted tokens | User must pick between two agents (but descriptions make this obvious) |
| **Single omnibus agent** | One entry point | Too much context crammed into one agent; script-writing instructions dilute YAML-authoring focus; agent body becomes enormous |

The router pattern is useful when you have 5+ specialists and the user can't easily tell which to pick. With two agents — one for YAML, one for scripts — the distinction is immediately obvious and the agent picker descriptions are sufficient.

## Opinion

**Strongly positive on two agents.** This maps perfectly to the mental model Nimble users already have: commands (YAML) and scripts (executables). The command agent handles the "what" (what should happen), the script agent handles the "how" (how to produce the output). The subagent pattern lets the command agent seamlessly delegate script writing without the user needing to switch contexts.

A router agent would add complexity without value. It would feel like calling a receptionist who just transfers you — annoying when there are only two departments.
