---
run-id: WORKING_ITEMS_RUN_2026-06-21_TP-R4-D9-ASSEMBLEDSEED-001
timestamp: 2026-06-21T01:29:31-06:00
run-status: SUCCESS
control-surface: INLINE
scope-path: /Users/ryan/.codex/worktrees/546c/chirality/projects/chirality-piping/execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-03_Nonlinear support regression suite
task-profile: validation-qa
task-skill: NONE
resolved-skill-path: NONE
resolved-skill-version: UNKNOWN
resolved-task-profile-requirement: NONE
companion-files: []
allowed-tools:
  - unrestricted
write-authorization: ALLOWED_WRITE_TARGETS
runtime-overrides: {}
---

## Requested Tasks

- Continue Phase D/R4 work after D5 was blocked by `D-15` and D7 by `D-17`.
- Extend D9 nonlinear validation evidence beyond classifier-only fixtures by adding an assembled global-loop seed that exercises the `core/solver/nonlinear_integration` path landed under D6.
- Preserve `DEC-046` visibility: no measured release convergence values or silent public defaults are introduced; unmeasured entries remain `TBD`.

## Expected Outputs

- Assembled nonlinear validation seed fixtures in `validation/benchmarks/nonlinear/`.
- Public-original hand-calculation notes in `validation/hand_calcs/nonlinear/`.
- Focused pytest coverage proving the assembled fixtures, public provenance notes, boundary wording, and `TolerancePolicyTbd` remain visible.
- Deliverable-local memory and run-record evidence.
- Completion-plan and coordination updates that mark this as partial D9 progress only.

## Tools Used

- zsh `git`
- zsh `sed`
- zsh `rg`
- zsh `find`
- zsh `date`
- zsh `cargo`
- zsh `python3`
- `apply_patch`

## Write Authorization

ALLOWED_WRITE_TARGETS: `validation/benchmarks/nonlinear/**`, `validation/hand_calcs/nonlinear/**`, `tests/test_nonlinear_support_regression.py`, `validation/evidence/sweeps/**`, DEL-09-03 `MEMORY.md`, DEL-09-03 `_run_records/**`, `plans/PLAN_2026-06-17_prd_completion.md`, `plans/PLAN_COMPLETION_LOG.md`, `docs/PLAN.md`, `execution/_Coordination/_COORDINATION.md`, and `execution/_Coordination/NEXT_INSTANCE_PROMPT.md`.

## Outputs Produced

- Added assembled nonlinear benchmark fixtures for one-way support deactivation, gap closure, and lift-off release using `solve_active_set_frame`.
- Added the `open_pipe_stress_frame_kernel` and `open_pipe_stress_nonlinear_integration` benchmark dependencies and removed the stale linear-support dependency.
- Added `assembled_one_way_deactivation.md`, `assembled_gap_closure.md`, and `assembled_lift_off.md` hand-calculation notes.
- Updated the nonlinear hand-calculation index and focused pytest readiness checks for the assembled fixture note set.
- Recorded this run and updated DEL-09-03 memory.
- Recorded the five-surface evidence sweep summary at `validation/evidence/sweeps/SWEEP_20260621T073226Z_4cb593a09376-dirty.json`.

## Validation Evidence

- `cargo fmt --manifest-path validation/benchmarks/nonlinear/Cargo.toml -- --check`: passed.
- `cargo test --manifest-path validation/benchmarks/nonlinear/Cargo.toml`: passed; 7 unit tests and 0 doc tests.
- `python3 -m pytest -q tests/test_nonlinear_support_regression.py`: passed; 8 tests.
- `git diff --check`: passed.
- `python3 tools/release/run_evidence_sweep.py --execute`: passed all five surfaces; summary `validation/evidence/sweeps/SWEEP_20260621T073226Z_4cb593a09376-dirty.json`.

## Dependency Notes

- `D-15` remains `AWAITING_RULING` and continues to gate D5 spring-hanger scope.
- `D-17` remains `NOT_PREPARED` and continues to gate D7 sparse live-path adoption timing.
- `D-16`, `D-18`, `D-19`, and `D-23` are ruled by `DEC-044`, `DEC-045`, `DEC-046`, and `DEC-048`.
- Current status discovery during the run reported `CHECKING=8`, `IN_PROGRESS=92`, and `ISSUED=1`; this run did not modify lifecycle state.

## Residual Scope

- Friction normal-force model integration remains open.
- Measured class-tiered convergence values remain `TBD` under `DEC-046`.
- Sparse live-path adoption remains gated by `D-17`.
- Broader live-solver coverage, the PRD section 16.2 branch-assembly benchmark, component provenance in the rendered report path, and the R4 exit evidence package remain open.
- This run does not make a release, professional approval, certification, sealing, authentication, code-compliance, lifecycle, or R4 exit claim.
