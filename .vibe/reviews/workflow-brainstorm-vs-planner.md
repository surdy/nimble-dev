# Reusable Brainstorm + Dev Plan Workflow
_Date: 2026-03-13_

The question: how to make the "brainstorm → file away → update roadmap/dev plan" workflow repeatable.

---

## Conclusion

Two custom VS Code chat modes (`.github/chatmodes/*.chatmode.md`) is the right fit:

- **Explore** — open-ended analysis, no roadmap/dev plan writes, outputs to `.vibe/reviews/` only
- **Planner** — triggered after a decision is made, converts a filed review into roadmap checkboxes and dev plan stages

The filed review in `.vibe/reviews/` is the formal handoff artifact between the two modes.

---

## Why two modes, not one

| Concern | Explore | Planner |
|---------|---------|---------|
| File writes | Only `.vibe/reviews/` | Only `docs/roadmap.md`, `.vibe/development-plan.md` |
| Tone | Exploratory, no commitment | Prescriptive, concrete tasks |
| Input | A question or problem | A filed review + a decision |
| Trigger | Any time | Only after a decision is made |

A single agent shifts modes at "I've decided on X" — but you lose enforcement. The agent can start updating the roadmap mid-exploration before you're ready.

---

## Alternatives considered

- **Single agent with explicit phases** — less overhead, but no constraint enforcement. Not recommended.
- **Prompt templates** (`.github/prompts/*.prompt.md`) — lighter weight, no tool restrictions; useful as a reminder of what to ask but not an enforced workflow.
- **Two custom chat modes** — native VS Code Copilot mechanism; each mode gets its own system prompt and declared tool set. Best fit.

---

## Names

- `Explore` and `Planner` preferred over "Brainstorm"/"Dev plan"
  - `Explore` = open-ended thinking, no commitments, files away if useful
  - `Planner` = decision made, convert to structured work
