---
run-id: WORKING_ITEMS_RUN_2026-06-21_TP-R4-D9-FRICTIONSEED-001
timestamp: 2026-06-21T01:43:54-06:00
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

- Continue Phase D/R4 D6/D9 nonlinear residual work after D5 remained gated by
  `D-15` and D7 by `D-17`.
- Add assembled friction validation coverage without claiming the governed
  derived normal-force model is complete.
- Surface explicit friction normal-reaction input evidence in the product
  mechanics result envelope while excluding that input evidence from load-case
  combination algebra.

## Expected Outputs

- A public-original assembled friction sticking validation fixture in
  `validation/benchmarks/nonlinear/`.
- A public-original hand-calculation note under
  `validation/hand_calcs/nonlinear/`.
- Product-preview result-envelope coverage proving explicit normal-reaction
  input evidence is visible and not algebraically combined.
- Deliverable-local memory and run-record evidence.
- Completion-plan and coordination updates marking this as partial D6/D9
  progress only.

## Tools Used

- zsh `git`
- zsh `sed`
- zsh `rg`
- zsh `date`
- zsh `cargo`
- zsh `python3`
- `apply_patch`

## Write Authorization

ALLOWED_WRITE_TARGETS: `validation/benchmarks/nonlinear/**`,
`validation/hand_calcs/nonlinear/**`,
`tests/test_nonlinear_support_regression.py`, `core/product_physics/**`,
`validation/evidence/sweeps/**`, DEL-09-03 `MEMORY.md`, DEL-09-03
`_run_records/**`, `plans/PLAN_2026-06-17_prd_completion.md`,
`plans/PLAN_COMPLETION_LOG.md`, `docs/PLAN.md`, and
`execution/_Coordination/NEXT_INSTANCE_PROMPT.md`.

## Outputs Produced

- Added `NL-ASSEMBLED-FRICTION-STICK-ORIGINAL`, an assembled dense active-set
  friction fixture that stays `Sticking` using explicit invented normal
  evidence.
- Added `validation/hand_calcs/nonlinear/assembled_friction_sticking.md` and
  indexed it in the nonlinear hand-calculation README and focused pytest
  requirements.
- Updated product physics to emit
  `nonlinear_support_friction_normal_reaction_input` result rows for explicit
  friction normal evidence and skip those evidence rows in combination algebra.
- Added a product-preview regression test for an invented friction support with
  explicit normal input.
- Recorded this run and updated the completion-plan handoff surfaces.

## Validation Evidence

- `cargo fmt --manifest-path validation/benchmarks/nonlinear/Cargo.toml -- --check`: passed.
- `cargo fmt --manifest-path core/product_physics/Cargo.toml -- --check`: passed.
- `cargo test --manifest-path validation/benchmarks/nonlinear/Cargo.toml`: passed; 7 unit tests and 0 doc tests.
- `python3 -m pytest -q tests/test_nonlinear_support_regression.py`: passed; 8 tests.
- `cargo test --manifest-path core/product_physics/Cargo.toml`: passed; 36 unit tests and 0 doc tests.
- `git diff --check`: passed.
- `python3 tools/release/run_evidence_sweep.py --execute`: passed all five
  surfaces; summary
  `validation/evidence/sweeps/SWEEP_20260621T074615Z_a83ced203fac-dirty.json`.

## Dependency Notes

- `D-15` remains `AWAITING_RULING` and continues to gate D5 spring-hanger
  scope.
- `D-17` remains `NOT_PREPARED` and continues to gate D7 sparse live-path
  adoption timing.
- `D-16`, `D-18`, `D-19`, and `D-23` are ruled by `DEC-044`, `DEC-045`,
  `DEC-046`, and `DEC-048`.
- Current status discovery during the run reported `CHECKING=8`,
  `IN_PROGRESS=92`, and `ISSUED=1`; this run did not modify lifecycle state.

## Residual Scope

- The derived friction normal-force model remains open; this run uses explicit
  invented normal-reaction evidence only.
- Sliding friction assembled/product validation remains open.
- Measured class-tiered convergence values remain `TBD` under `DEC-046`.
- Sparse live-path adoption remains gated by `D-17`.
- Broader live-solver coverage, the PRD section 16.2 branch-assembly
  benchmark, component provenance in the rendered report path, and the R4 exit
  evidence package remain open.
- This run does not make a release, professional approval, certification,
  sealing, authentication, code-compliance, lifecycle, or R4 exit claim.
