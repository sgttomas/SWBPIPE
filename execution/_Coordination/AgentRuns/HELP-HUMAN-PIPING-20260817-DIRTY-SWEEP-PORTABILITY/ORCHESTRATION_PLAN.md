# ORCHESTRATION PLAN — Dirty Sweep Record Portability

- RunID: `HELP-HUMAN-PIPING-20260817-DIRTY-SWEEP-PORTABILITY`
- Parent: `HELP_HUMAN`
- Manager and integration owner: `WORKING_ITEMS`
- Selection authority: `AGENT_0`
- Posture: `SERIAL`
- Branch: `codex/piping-dirty-sweep-output`
- Accepted predecessor commit: `6ec3117c09891039dba3b9d78603c737be3f54ba`
- Plan version: `1`

## Selection

Repository self-check exposed two tranche-local machine-path findings in the
accepted N1 AgentRuns package. Execute exactly N2 after N1: replace only those
machine-specific environment references with portable wording while preserving
the HELP_HUMAN attribution and exact focused/full test results.

## Sealed WORKING_ITEMS Brief

- Node: `N2` only; serial dependency on accepted `N1`.
- Allowed existing-record writes:
  `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260816-DIRTY-SWEEP-OUTPUT/MANAGER_RETURN.md`
  and
  `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260816-DIRTY-SWEEP-OUTPUT/WORK_GRAPH_V2.json`.
- Allowed new writes: this named N2 AgentRuns package only.
- Exclusions: product, tests, evidence, receipts, registers, deliverable state,
  Git operations, and all other paths.
- Acceptance: both cited machine-path strings are absent; portable phrasing
  retains HELP_HUMAN attribution plus `38 passed in 2.53s` and
  `576 passed in 15.49s`; self-check no longer reports the two tranche-local
  findings; JSON parses; containment and diff checks pass.
- Escalation: report any newly selectable consequence before expanding scope;
  do not manufacture N3.

## Model Attribution

HELP_HUMAN supplied N2 selection and N1 acceptance. WORKING_ITEMS executes with
the inherited Codex GPT-5 capability; the exact runtime model identifier is not
exposed. No override or substitution is declared.
