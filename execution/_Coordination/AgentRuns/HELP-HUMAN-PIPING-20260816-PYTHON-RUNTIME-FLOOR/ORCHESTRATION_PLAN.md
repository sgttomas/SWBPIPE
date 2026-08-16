# ORCHESTRATION PLAN — Python Runtime Floor

- RunID: `HELP-HUMAN-PIPING-20260816-PYTHON-RUNTIME-FLOOR`
- Parent: `HELP_HUMAN`
- Manager: `WORKING_ITEMS`
- Bounded integration scope: `TM-PIP-027` / DEC-025 sweep development environment
- Posture: `SINGLE_MANAGER_DIRECT`
- Selection authority: HELP_HUMAN owner-directed activation of `TM-PIP-027`
- Source/base commit: `65735390590e500dbbea6b63a4a79ba42944bf6d`
- Branch: `codex/piping-python-runtime-floor`
- Plan version: `1`

## HELP_HUMAN Selection

Implement the existing `OPEN` Task Management row `TM-PIP-027`: require
Python 3.11 or newer before the DEC-025 sweep probes prerequisites or executes
surface 1, and replace every direct development-dependency range with an exact
compatible pin. Preserve the sweep's schema/version and its capability,
offline, and `CARGO_TARGET_DIR` behavior.

## Completed CHANGE Branch Setup Brief

Create one clean task branch named `codex/piping-python-runtime-floor` from
verified basis `65735390590e500dbbea6b63a4a79ba42944bf6d`; make no product
edits. Return branch, base, and cleanliness to HELP_HUMAN/WORKING_ITEMS.

## Completed CHANGE Branch Setup Return

- Branch: `codex/piping-python-runtime-floor`
- Base and current `HEAD`: `65735390590e500dbbea6b63a4a79ba42944bf6d`
- Initial worktree state: clean
- Product edits: none
- Disposition: ready for bounded WORKING_ITEMS execution

## Sealed WORKING_ITEMS Brief

- Objective: make the DEC-025 Python environment explicit and deterministic.
- Declared reads: root/project instructions, committed standing plan,
  `TM-PIP-027`, `requirements-dev.txt`, sweep tool/tests, software workflow
  profile, and validation surfaces required by the activation.
- Allowed product writes: `requirements-dev.txt`,
  `tools/release/run_evidence_sweep.py`, `tests/test_evidence_sweep.py`, and
  `docs/BUILD_AND_RELEASE.md` only if user-facing floor documentation proves
  necessary.
- Allowed closeout writes: this AgentRuns package and one DEC-025 sweep JSON;
  receipt remains owned by CHANGE.
- Exclusions: `core/**`, desktop product source, registers, DAGs, policy,
  foreign loops, lifecycle state, commits, pushes, PRs, merges, and receipt.
- Acceptance: focused tests; full Piping pytest; offline requirements resolve
  if cache permits; redirected-`CARGO_TARGET_DIR` sandbox-only DEC-025 sweep;
  diff check; self-check; practitioner-harness pytest; applicable validators;
  exact rerun instructions for CHANGE's clean-commit host sweep.
- Escalation: stop for any scope expansion, new normative choice, or protected
  path requirement; otherwise record environmental blockers without stalling
  independent work.

## Model Attribution

HELP_HUMAN selection and CHANGE setup were supplied by the parent run.
WORKING_ITEMS executes with the inherited Codex GPT-5 capability; the exact
runtime model identifier is not exposed to this manager. No override or
mid-run substitution is declared.
