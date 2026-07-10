# WORKING_ITEMS Run Record - TP-PMM-P2-COMPLEMENTARITY-001

Date: 2026-07-10
Agent: WORKING_ITEMS (claude worker; owner-adopted tranche, branch
`claude/piping-p2-nonlinear-repair`)
Deliverable: DEL-04-04 - Nonlinear support active-set solver
Package: PKG-04 - Solver Core and Numerical Methods
Tranche: TP-PMM-P2-COMPLEMENTARITY-001
Target stage: R5 / physical-model mechanics program P2 clause 1
(`plans/PLAN_2026-07-09_physical_model_mechanics.md`; ruling `DEC-067` via
the D-35 ruling `execution/_Coordination/_DECISIONS/D-35_RULING_2026-07-09.md`)

## Scope

DEC-067 clause 1: replace the reaction-only (one-way, lift-off) and
displacement-only (gap) classification tests in `classify_support_state`
with the state-switched complementarity test, so re-engagement and lift-off
transitions cannot be missed and converged results do not depend on seeded
`initial_states`.

## Files Touched

- `core/solver/nonlinear_supports/{README.md, src/lib.rs}`
- `validation/benchmarks/nonlinear/src/lib.rs` (classifier-level fixture
  reconciliation)
- `validation/hand_calcs/nonlinear/{active_set_one_way.md, gap_closure.md, unresolved_nonconvergence.md}`
- `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-04_Nonlinear support active-set solver/{Specification.md, Procedure.md, MEMORY.md}`

## Implemented Evidence

- `classify_support_state(support, trial, prior_state)`: engaged supports
  (prior `Active`, including closed gaps) classify on trial reaction sign
  (one-way/lift-off keep their activation-sense test; a closed gap bears
  only against its closing direction, so a pulling/zero reaction releases
  it); released supports classify on trial displacement penetration toward
  the bearing side (one-way/lift-off) or the explicit clearance (gap). The
  friction stick/slip test is unchanged, and the deterministic
  sliding-persistence anti-chatter rule in
  `classify_iteration_support_state` is kept (no conflict for
  transition-path solves; the seeded-sliding interaction is recorded under
  TP-PMM-P2-FRICTION-001).
- Crate tests: engaged/released one-way classification, penetration
  re-engagement (including the unseeded-prior deterministic released
  branch), closed-gap bearing-sign persistence/release, plus the existing
  gap/lift-off/friction/nonconvergence coverage updated to the
  state-switched trials.
- Reconciled classifier-level invented fixtures whose trials encoded the
  old tests: `NL-ACTIVE-ONE-WAY-ORIGINAL` (re-engages from a penetrating
  `-0.02 mm` trial displacement; a released DOF has identically zero trial
  reaction, so the old reaction-based trial was unobservable in a real
  loop), `NL-GAP-CLOSURE-ORIGINAL` (closed gap persists under a `-2.0 N`
  bearing reaction), `NL-NONCONVERGENCE-LIMIT-ORIGINAL` (penetrating
  `0.002 rad` rotational trial at the iteration cap). Hand-calc witnesses
  updated to match.

## Checks

- `cargo test` `core/solver/nonlinear_supports`: 19 passed.
- `cargo test` `core/solver/nonlinear_integration`: 14 passed.
- `cargo test` `core/product_physics`: 56 passed.
- `cargo test` `validation/benchmarks/nonlinear`: 19 passed.
- `python3 -m pytest -q tests`: 369 passed; practitioner-harness pytest:
  263 passed, 1 skipped.
- `cargo fmt --check` clean on touched crates.
- Repo-wide harness `self-check` exit 0.
- `DEC-025` sweep recorded with this tranche set (shared sweep summary
  committed separately).

## Boundaries And Residuals

- The classifier consumes user-entered support behavior and trial facts
  only; no defaults, no protected standards content, no catalog values.
- An unseeded prior state deterministically takes the released branch;
  in-loop callers always seed explicit initial states.
- DEC-046 class-tiered tolerance policy mechanism unchanged; unmeasured
  axes stay `TBD`.
- No lifecycle transition, no release-readiness, professional,
  certification, sealing, authentication, or code-compliance claim.
