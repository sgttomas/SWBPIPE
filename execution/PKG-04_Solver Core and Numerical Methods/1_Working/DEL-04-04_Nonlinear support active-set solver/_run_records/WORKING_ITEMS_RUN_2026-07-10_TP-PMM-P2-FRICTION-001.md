# WORKING_ITEMS Run Record - TP-PMM-P2-FRICTION-001

Date: 2026-07-10
Agent: WORKING_ITEMS (claude worker; owner-adopted tranche, branch
`claude/piping-p2-nonlinear-repair`)
Deliverable: DEL-04-04 - Nonlinear support active-set solver
Package: PKG-04 - Solver Core and Numerical Methods
Tranche: TP-PMM-P2-FRICTION-001
Target stage: R5 / physical-model mechanics program P2 clauses 2-3
(`plans/PLAN_2026-07-09_physical_model_mechanics.md`; ruling `DEC-067` via
the D-35 ruling `execution/_Coordination/_DECISIONS/D-35_RULING_2026-07-09.md`)

## Scope

DEC-067 clause 2 (bounded +/- mu*N Coulomb sliding force instead of full
DOF release, normal reaction from the current iterate) and clause 3
(transition hand-calc witnesses and matching assembled fixtures for
re-engagement of a lifted one-way support, lift-off of a closed gap, and
sliding with the bounded friction force).

## Files Touched

- `core/solver/nonlinear_integration/{README.md, src/lib.rs}`
- `core/product_physics/src/lib.rs` (test expectations for bounded sliding)
- `fixtures/product_preview/invented_mechanics_result.json` (regenerated
  canned envelope; deltas confined to nonlinear-loop rows)
- `tests/product_preview/test_product_preview_service.py` (canned-envelope
  value assertions)
- `validation/benchmarks/nonlinear/src/lib.rs`
- `validation/hand_calcs/nonlinear/{assembled_one_way_reengagement.md, assembled_gap_lift_off.md, assembled_friction_bounded_sliding.md, assembled_friction_sliding.md, assembled_multi_support_friction_gap_acceptance.md, assembled_multi_support_derived_normal_gap_acceptance.md, assembled_multi_support_derived_normal_rotational_acceptance.md, assembled_multi_support_four_class_acceptance.md, README.md}`
- `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-04_Nonlinear support active-set solver/MEMORY.md`

## Implemented Evidence

- Bounded sliding force: `prescribed_displacement_for_state` still leaves a
  sliding DOF free, but the loop now applies an
  `AppliedSlidingFrictionForce` of magnitude `mu * |N|` opposing the prior
  iterate's motion (displacement sign, else impending-motion sense opposite
  the restrained tangential reaction). `N` comes from the current iterate's
  evidence through the existing `friction_normal_for_support` surface
  (explicit input or the named derived-normal support reaction). Applied
  forces are recorded per iteration on `NonlinearFrameIteration`.
- Support reactions are reported against the caller's base force, so the
  bounded force appears as the support's tangential reaction (`-mu*N`)
  while free-DOF force/moment/work residuals stay measured against the
  solved system (exact 0.0 limits still hold on the fixture seed).
- Seed-independence: a sliding state seeded before any solved iterate
  defers convergence one iteration so the bound is applied before the loop
  can converge; test
  `sliding_friction_support_applies_bounded_force_independent_of_seed`
  proves sticking-seeded and sliding-seeded runs converge to identical
  displacements/reactions ((10 - 0.3*10) N drive -> 0.07 mm, reaction
  -3 N).
- Clause 3 witnesses + fixtures (invented inputs only, DEC-046 class-tiered
  tighten-only policy refs; policy mechanism unchanged):
  - `assembled_one_way_reengagement.md` /
    `NL-ASSEMBLED-ONE-WAY-REENGAGE-ORIGINAL` (re-engagement of a lifted
    one-way support from a penetrating -0.1 mm free displacement).
  - `assembled_gap_lift_off.md` / `NL-ASSEMBLED-GAP-LIFT-OFF-ORIGINAL`
    (lift-off of a closed gap under a +3 N pulling contact reaction).
  - `assembled_friction_bounded_sliding.md` /
    `NL-ASSEMBLED-FRICTION-BOUNDED-SLIDE-ORIGINAL` (seeded sliding,
    bounded force -3 N, net 5 N drive -> 0.05 mm, reaction -3 N).
  - `dec_067_transition_fixtures_witness_state_switched_and_bounded_outcomes`
    asserts the hand-calc displacement/reaction values through the
    assembled loop; fixtures registered in `assembled_fixture_inventory`
    and the hand-calc README inventory.
- Reconciled existing sliding fixtures to the bounded mechanics: invented
  normal-reaction evidence retuned (explicit 10 N -> 20 N; derived-normal
  coefficient 0.03 -> 0.06) so bounded net drives stay exactly
  representable under the accepted 0.0 free-DOF residual limits
  (tighten-only preserved; observed deltas shrink). Product-preview canned
  envelope regenerated: deltas confined to nonlinear-loop rows
  (iteration-count 1 -> 2; sliding support uz-reaction 0 -> +0.4901 N;
  displacement magnitude bounded; derived normal 49.010116 -> 48.952652 N
  through the changed load path).

## Checks

- `cargo test` `core/solver/nonlinear_supports`: 19 passed.
- `cargo test` `core/solver/nonlinear_integration`: 14 passed.
- `cargo test` `core/product_physics`: 56 passed.
- `cargo test` `validation/benchmarks/nonlinear`: 19 passed;
  `validation/benchmarks/mechanics`: 27 passed (sanity).
- `python3 -m pytest -q tests`: 369 passed; practitioner-harness pytest:
  263 passed, 1 skipped.
- Dense/sparse parity oracle green on touched solve paths (dense-scrutiny
  parity tests and curved-bend parity row within 1.0e-9).
- `cargo fmt --check` clean on touched crates.
- Repo-wide harness `self-check` exit 0.
- `DEC-025` sweep on the final content commit recorded as its own commit.

## Boundaries And Residuals

- **No path/load-step friction history model** (explicitly out of scope per
  DEC-067): the bound is a single-iterate Coulomb limit from
  current-iterate evidence.
- **Anti-chatter interaction residual**: sliding direction comes from the
  prior iterate only, so a sliding support whose bound exceeds the actual
  drive can oscillate direction and reach the visible nonconvergence cap
  instead of re-sticking; this fails loudly (diagnostic), never silently.
- Sliding-force magnitude is not itself a convergence residual axis; the
  governed residual remains the active-set state-change count.
- A sliding seed at `max_iterations == 1` returns `converged=false` after
  the deferred first iterate without a nonconvergence diagnostic (residual
  is zero at the cap); recorded corner, not silent.
- Arc interior stations remain open under DEC-070.
- No lifecycle transition, no release-readiness, professional,
  certification, sealing, authentication, or code-compliance claim.
