# Owner route direction — solver engine and validation first (2026-09-25)

HELP_HUMAN (ROOT), second resumed session of 2026-09-25, after PR905 merged at `23aad15d6fd08638c6fa6a017b8aa128dd36d02a`.

## Exact owner messages

The owner asked:

> When you resume does the work graph clearly have the solver engine and validation regime plotted? Unless the UI work helps advance the solver engine and validation regime I want to delay development and testing into focused tranches later. Do you see that being supported by your work graph?

ROOT answered that the graph did not yet support this well. UI-MILESTONE and LIVE-FINAL were CLOSE prerequisites, there was no ordered solver route over the open finding groups, and the current route was scattered across checkpoint sections. ROOT proposed a records-only refresh:

- record the direction;
- move the UI, live-control, visual, profile, D-72 and A2 work into a named deferred successor, keeping their preserved checkpoints, and remove them from this undertaking's CLOSE prerequisites;
- put a short current route at the top of the graph, with the solver engine as ordered tranches over the open groups, each with its dependencies and the validation nodes that qualify it;
- obtain an independent review and merge the refresh as its own small PR.

The owner replied:

> Yes we need to refresh the work graph. You may proceed.

## ROOT application

- **Direction.** Solver-engine and validation work leads. UI development and testing move to later focused tranches, except where a UI change is needed to advance a solver or validation tranche. In that case it stays inside that tranche and is limited to what the tranche needs.
- **Interpretation (ROOT).** The deferred UI and live-control work is not abandoned, cancelled or closed. It moves to a named successor, `UI-SUCCESSOR`, with its checkpoints, rulings, activations and unperformed actual-human witnesses unchanged. Three original finding groups are primarily authoring-workflow or UI work: M23 (library-to-model assignment), M25 (authoring speed) and M38 (authoring/numbering/history inconsistencies). They move open to that successor. They are not counted as closed, and they are not closed by this transfer.
- **Scope.** This record changes sequencing and this undertaking's closure prerequisites only. It changes no accepted scope, hold, lifecycle, protected criterion, release authority or earlier owner decision. It adds no permission to populate material or component libraries or code rules.
- **Validation.** Each solver tranche is paired with the validation nodes that qualify it. This makes the static validation evidence part of this undertaking's route, which is ROOT's reading of "the solver engine and validation regime plotted". Licensed external runs, agent-controlled campaigns, dynamics, physical-measurement and design-search nodes keep their own conditions.
- **Execution.** The owner's earlier pause stands for new implementation work. This refresh is the authorized records work; the load-state tranche resumes when the owner directs.
