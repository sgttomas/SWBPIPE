# ORCHESTRATION PLAN — Dirty Sweep Output

- RunID: `HELP-HUMAN-PIPING-20260816-DIRTY-SWEEP-OUTPUT`
- Parent: `HELP_HUMAN`
- Manager and integration owner: `WORKING_ITEMS`
- Selection authority: `AGENT_0`
- Posture: `SERIAL`
- Branch: `codex/piping-dirty-sweep-output`
- Verified base: `44903bc69cf56d4ca794fe9629f26793a82bf1b3`
- Plan version: `1`

## HELP_HUMAN Selection

Implement the owner-ratified `TM-PIP-042` forward tool behavior: a DEC-025
sweep whose final Git state is dirty must not write into the canonical
`validation/evidence/sweeps/` directory by default. Preserve clean-tree
defaults, explicit non-canonical `--output-dir` behavior, and schema 2. Add a
clearly named opt-in for an intentional dirty canonical summary. Historical
evidence remains immutable.

## Sealed WORKING_ITEMS Brief

- Node: `N1` only; no Agent 2 dispatch and no parallel work.
- Allowed product writes:
  `projects/chirality-piping/tools/release/run_evidence_sweep.py` and
  `projects/chirality-piping/tests/test_evidence_sweep.py`.
- Allowed coordination writes: this named AgentRuns package only.
- Shared-surface writes: deferred to WORKING_ITEMS fan-in.
- Exclusions: historical sweep evidence, receipts, registers, deliverable
  state, foreign paths, commits, pushes, PRs, and merges.
- Acceptance: dirty default routes outside canonical evidence and writes
  nothing there; clean default remains canonical; explicit dirty-canonical
  opt-in works; explicit safe `--output-dir` remains compatible; summary
  schema remains 2; focused/full applicable tests, diff check, self-check, and
  practitioner-harness checks pass or have a precise environmental blocker.
- Gates: path containment; fail-safe Git-state handling; no historical
  evidence mutation; product tests; repository checks; manager fan-in.

## Model Attribution

HELP_HUMAN selection and CHANGE branch setup were supplied by the parent run.
WORKING_ITEMS executes with the inherited Codex GPT-5 capability; the exact
runtime model identifier is not exposed. No override or substitution is
declared.
