# WORKING_ITEMS Run Record - TP-R4-D6-LOOPCORE-001

Date: 2026-06-21
Agent: WORKING_ITEMS
Target stage: R4 / Phase D
Tranche: TP-R4-D6-LOOPCORE-001
Decision basis: `DEC-044` and `DEC-046`

## Objective

Land the first D6 assembled nonlinear loop-core slice under `DEC-044`: bridge
DEL-04-01 frame assembly/linear solve with DEL-04-04 nonlinear support
active-set classification without moving either existing crate boundary.

This is a partial D6 landing. It creates the core dense integration owner and
tests invented mechanics cases, but it does not bind the loop into
`product_physics`, the desktop app, result envelopes, report rendering, sparse
live-path solves, or R4 exit validation cases.

## Changes

- Added `core/solver/nonlinear_integration`, package
  `open_pipe_stress_nonlinear_integration`.
- Implemented `solve_active_set_frame`, which:
  - assembles frame stiffness through `frame_kernel`;
  - applies base restraints plus active nonlinear restraints;
  - solves the dense prescribed-displacement reduced system;
  - reconstructs full displacements and reactions;
  - feeds trial displacement/reaction facts to
    `evaluate_active_set_iteration`;
  - stops on active-set convergence or the explicit iteration cap.
- Added explicit convergence-control inputs with `policy_ref`,
  `ConvergencePolicyStatus`, residual tolerance, absolute residual floor, and
  max iteration cap.
- Preserved `TolerancePolicyTbd` visibility for `TBD` convergence-policy
  entries under `DEC-046`; no public defaults or measured class-tiered values
  were invented.
- Re-pointed DEL-04-04 documentation so the assembled loop is owned by
  `core/solver/nonlinear_integration`; DEL-04-04 remains the per-iteration
  classifier/state oracle.
- Cleaned stale D-16/D-18/D-19 decision-packet status text to match the ruled
  `DEC-044`/`DEC-045`/`DEC-046` register state.
- Updated `docs/SPEC.md`, `docs/TYPES.md`, `docs/PLAN.md`,
  `plans/PLAN_2026-06-17_prd_completion.md`, `_COORDINATION.md`, and
  `NEXT_INSTANCE_PROMPT.md` so the next handoff sees D6 as partially landed
  rather than unbuilt.

## Validation

Passed:

- `cargo fmt --manifest-path core/solver/nonlinear_integration/Cargo.toml --check`
- `cargo test --manifest-path core/solver/nonlinear_integration/Cargo.toml`
  - 7 unit tests; 0 doctests
- `cargo test --manifest-path core/solver/nonlinear_supports/Cargo.toml`
  - 16 unit tests; 0 doctests
- `cargo test --manifest-path core/solver/frame_kernel/Cargo.toml`
  - 34 unit tests; 0 doctests
- `git diff --check`

## Boundaries

- Dense loop-core mechanics only; no sparse live-path binding.
- No product/app/result-envelope/report integration.
- No measured class-tiered convergence values; unmeasured entries remain `TBD`
  under `DEC-046`.
- No pressure-thrust load generation, component macro-element expansion beyond
  prior D4 evidence, spring-hanger behavior, or D9 validation packet.
- No protected standards content, proprietary catalog value, private data,
  network/telemetry feature, public default, release-readiness claim,
  professional approval, certification, sealing, authentication, or
  code-compliance claim.

## Residuals

- Continue D6 by binding the loop into `product_physics` / result envelopes /
  app-facing preview only after selecting a scoped tranche.
- Draft D9 assembled nonlinear validation cases from the loop-core path and use
  those runs to seed measured `DEC-046` class entries where justified.
- Keep D7 sparse live-path adoption gated by `D-17`.
- Keep D5 spring-hanger behavior gated by `D-15`.
