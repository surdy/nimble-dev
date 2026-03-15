# Review: User-Defined Variables for Scripts
_Date: 2026-03-14_

## Overview

User-defined variables should complement built-in `NIMBLE_*` variables by giving users a simple way to inject their own values (team IDs, project keys, base URLs, email addresses) into script environments.

---

## Global user-defined variables

Recommended location: `Nimble/env.yaml`.

```yaml
WORK_EMAIL: alice@example.com
JIRA_BASE_URL: https://mycompany.atlassian.net
TEAM_SLACK_CHANNEL: C0123456789
```

### Why `env.yaml` over `settings.yaml`

- Keeps concerns separate: `settings.yaml` is app behavior; `env.yaml` is script data.
- Easier to explain and discover for users familiar with `.env` workflows.
- Simple implementation: parse as `HashMap<String, String>`.
- Optional file: if missing, no user variables are injected.

---

## Command-scoped user-defined variables

### Simplified sidecar rule (recommended)

Only consider `env.yaml` in the same directory as the command YAML (`source_dir`).

- No directory walking.
- No multi-level merge complexity.
- Consistent with co-located scripts/lists behavior.

Shared variables for sibling commands:

```text
commands/
└── jira/
    ├── env.yaml
    ├── create-ticket.yaml
    ├── create-ticket.sh
    ├── close-ticket.yaml
    └── close-ticket.sh
```

Isolated variables per command:

```text
commands/
└── jira/
    ├── create-ticket/
    │   ├── env.yaml
    │   ├── create-ticket.yaml
    │   └── create-ticket.sh
    └── close-ticket/
        ├── env.yaml
        ├── close-ticket.yaml
        └── close-ticket.sh
```

### Optional inline override in command YAML

```yaml
phrase: create ticket
title: Create Jira ticket
env:
  TICKET_TYPE: story
action:
  type: script_action
  config:
    script: create-ticket.sh
```

Use inline `env:` for command-specific overrides only.

---

## Precedence order (lowest -> highest)

```text
System environment
NIMBLE_* built-ins
Global Nimble/env.yaml
source_dir/env.yaml
Command inline env:
```

### Guardrails

- Built-in `NIMBLE_*` keys remain non-overridable by user-defined variables.
- User-defined keys should be validated to reject empty names and reserved prefixes (`NIMBLE_`).

---

## Open questions

- Should `dynamic_list` and `script_action` both support inline `env:` equally (recommended), or should scope be limited initially?
- Should there be a warning when global and command-scoped variables collide, even though override order is deterministic?
