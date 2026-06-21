# WORKING_ITEMS Run Record - TP-R4-D17-PACKET-001

**Date:** 2026-06-21
**Agent:** WORKING_ITEMS
**Persona:** deliverable-scoped content production for OpenPipeStress
**Tranche:** `TP-R4-D17-PACKET-001`
**Scope:** D-17 sparse-solver live-path adoption decision-packet preparation
**Lifecycle posture:** packet/evidence update only; no deliverable issuance,
release readiness, professional approval, certification, sealing,
authentication, or code-compliance claim.

## Objective

Prepare the human-ruling packet for decision `D-17`: whether the accepted
`DEC-023` in-repo sparse solver (`core/solver/sparse_direct`) should be bound
into the live R4 `frame_kernel` / `product_physics` / `nonlinear_integration`
solve path now, and if so whether it is default or an explicit evidence lane.

## Authority And Inputs

- Required WORKING_ITEMS intake was re-read for this run:
  `{REPO_ROOT}/agents/AGENT_WORKING_ITEMS.md`, project `AGENTS.md`, and
  `execution/_Coordination/NEXT_INSTANCE_PROMPT.md`.
- Active project root:
  `/Users/ryan/.codex/worktrees/546c/chirality/projects/chirality-piping`.
- Active coordination basis: current R4 target stage (`DEC-048`) and the
  completion plan row D-17 / Phase D row D7.
- Prior governing sparse decision: `DEC-023` accepts the in-repo skyline/profile
  direct solver strategy; D-17 does not re-open solver-library selection.

## Work Performed

- Added
  `execution/_Coordination/_DECISIONS/D-17_sparse_solver_live_path_adoption.md`.
- Updated
  `execution/_Coordination/_DECISIONS/_REGISTER.md` row D-17 from
  `NOT_PREPARED` to `AWAITING_RULING`.
- Updated
  `plans/PLAN_2026-06-17_prd_completion.md` so D-17 and Phase D row D7 point to
  the prepared packet and record the advisory Option B recommendation.
- Updated `docs/PLAN.md` to avoid re-selecting D-17 packet preparation as the
  next unblocked item.
- Updated DEL-04-05 `MEMORY.md` to distinguish the resolved DEC-023 sparse
  strategy from the still-open live solve-path adoption question.

## Evidence Basis

- `core/solver/sparse_direct` implements the accepted DEC-023 solver strategy.
- `core/solver/performance_harness` measures the sparse path alongside dense and
  still records live solve-path adoption as `TBD`.
- `core/solver/nonlinear_integration` and `core/product_physics` still call
  `solve_dense` in the live path.
- PRD §20 requires sparse matrix methods for large models; PRD §22.5 R4 exit
  criteria do not explicitly name sparse adoption.

## Validation

- `git diff --check` passed.
- `cargo test --manifest-path core/solver/sparse_direct/Cargo.toml` passed
  18/18 tests.
- `cargo test --manifest-path core/solver/performance_harness/Cargo.toml`
  passed 18/18 tests.
- Full DEC-025 five-surface evidence sweep passed:
  `validation/evidence/sweeps/SWEEP_20260621T105725Z_54f581a0b912-dirty.json`.
  Surfaces passed: cargo crate sweep (33 manifests), repository pytest
  (362/362), desktop Vitest (407/407), desktop Playwright (18/18 plus dist
  smoke 1/1), and desktop production build. The Vite large-chunk warning is the
  existing warning posture; it did not fail the build.

## Boundaries Preserved

- No source behavior, schema, solver logic, dependency graph authority,
  lifecycle state, or release gate was changed.
- No protected standards text/tables, private project data, code-specific
  allowables, vendor catalog values, professional approval, certification,
  sealing, authentication, or code-compliance claims were introduced.

## Residuals And Next Step

- D-17 is awaiting human ruling.
- D-15 is also awaiting human ruling for the D5 spring-hanger scope.
- If D-17 is ruled, the next D7 implementation tranche should follow the
  accepted option. Under the advisory Option B, that tranche is a live sparse
  evidence lane with dense default retained and profile-direct assembly left as
  follow-on work.
