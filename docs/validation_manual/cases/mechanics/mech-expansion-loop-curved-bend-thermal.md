---
doc_id: OPS-VALIDATION-MANUAL-CASE-MECH-EXPANSION-LOOP-CURVED-BEND-THERMAL
doc_kind: governance.validation_manual_case
status: draft_evidence
created: 2026-07-10
generated_by: docs/validation_manual/cases/generate_validation_case_pages.py
refs:
  - rel: implements
    to: DEL-09-04
  - rel: evidences
    to: DEL-09-01
---

# MECH-EXPANSION-LOOP-CURVED-BEND-THERMAL

| Item | Record |
|---|---|
| Suite | Mechanics benchmark suite (`DEL-09-01`), crate `validation/benchmarks/mechanics/` |
| Evidence class | Mechanics verification |
| Evidence state | `DRAFT_EVIDENCE` |
| Independent reference | `validation/hand_calcs/mechanics/expansion_loop_curved_bend_thermal.md` |
| Provenance | Invented public-original fixture data only (recorded in the reference note and fixture metadata) |

## Test Purpose And Problem Statement

Expansion-loop (L-bend) thermal-bending benchmark with a hand-calculated
known-flexibility reference, authored as the independent reference required by
the D-34 exit-evidence bar (`DEC-070`, packet
`execution/_Coordination/_DECISIONS/D-34_bend_flexibility_stiffness_realization.md`
§5). A plane L-shaped anchor-to-anchor loop with a quarter-circle elbow is
solved by the force method under uniform temperature rise, with the elbow's
in-plane bending flexibility multiplied by a user-entered factor `k`. The
document provides anchor reactions, a tangent-point displacement, and a
k-sweep monotonicity statement for `k in {1, 5, 10, 20}`.

All inputs are invented or user-entered fixture data; nothing is copied from
protected standards, commercial software examples, or proprietary data.

This case is the D-34 / `DEC-070` exit-evidence comparison for the curved-bend macro element, including the k-sweep monotonicity evidence for the user-entered in-plane flexibility factors `k in {1, 5, 10, 20}`. The factor `k` is an opaque user-entered value; no flexibility-factor equation, catalog value, SIF, or default is used.

## Input Model

The machine-readable input model is the fixture constructor `expansion_loop_curved_bend_thermal_fixture()` (fixture id `MECH-EXPANSION-LOOP-CURVED-BEND-THERMAL`) in `validation/benchmarks/mechanics/src/lib.rs`, carrying explicit fixture-local units and public-original provenance metadata. The reference note records the same invented inputs with the longhand derivation.

## Expected Result And Independent Reference

The expected values and their derivation are recorded in the independent
reference note `validation/hand_calcs/mechanics/expansion_loop_curved_bend_thermal.md` and mirrored in the fixture's expected-value
slots. The reference derivation uses elementary open mechanics only.

## Software Result And Reproduction

The measured-vs-reference comparison executes inside the named suite
test(s) `expansion_loop_fixture_matches_witness_reference_table`, `expansion_loop_free_expansion_identity_is_stress_free`, `expansion_loop_t2_x_displacement_is_k_independent_free_shortening`, `expansion_loop_k_sweep_reactions_decrease_strictly_monotonically`, `expansion_loop_reactions_satisfy_whole_body_equilibrium`, `expansion_loop_stiffer_elbow_forces_larger_departure_from_free_growth`, which run the current in-repo solver path on the fixture
and assert agreement with the reference expectations.

Reproduction (from `projects/chirality-piping`):

```bash
cargo test --manifest-path validation/benchmarks/mechanics/Cargo.toml expansion_loop_fixture_matches_witness_reference_table
cargo test --manifest-path validation/benchmarks/mechanics/Cargo.toml expansion_loop_free_expansion_identity_is_stress_free
cargo test --manifest-path validation/benchmarks/mechanics/Cargo.toml expansion_loop_t2_x_displacement_is_k_independent_free_shortening
cargo test --manifest-path validation/benchmarks/mechanics/Cargo.toml expansion_loop_k_sweep_reactions_decrease_strictly_monotonically
cargo test --manifest-path validation/benchmarks/mechanics/Cargo.toml expansion_loop_reactions_satisfy_whole_body_equilibrium
cargo test --manifest-path validation/benchmarks/mechanics/Cargo.toml expansion_loop_stiffer_elbow_forces_larger_departure_from_free_growth
```

Recorded run: 2026-07-10, toolchain rustc 1.92.0 / cargo 1.92.0; suite
result `ok. 30 passed; 0 failed; 0 ignored` with the named test(s) passing. The full
suite output and the exact commit are recorded in the run record
`execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/_run_records/WORKING_ITEMS_RUN_2026-07-10_TP-E2-VALMANUAL-001.md`.

Headless-runner binding: the `DEC-065` `openpipestress-runner run-benchmark`
verb accepts a schema-first benchmark request but currently exits `1` with
the structured blocking diagnostic
`HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD` because
benchmark payload binding remains future bounded `DEL-10-05` work. Until that
binding lands, the exact reproduction command for this case is the crate test
above; the runner-reproducible external path is recorded in
[headless_runner_reproduction.md](../../headless_runner_reproduction.md).

## Tolerance

The witness-table comparison uses the measured, reason-annotated fixture-local constants in `validation/benchmarks/mechanics/src/lib.rs`: reaction and displacement relative tolerance `5.0e-7` (limiting factor: residual axial flexibility of the boosted bending-only comparison model, measured max deviation `5.8e-8` relative at axial-rigidity boost `1.0e5`; `u_y(T2)` measured max `4.5e-8`; `u_x(T2)` measured max `6.0e-8`), whole-body equilibrium absolute floors `1.0e-3` N / `1.0e-3` N-m (measured max residuals `2.9e-5` N and `7.1e-5` N-m), and free-expansion self-check floors `1.0e-8` m / `1.0e-2` N. The comparison replicates the witness bending-only-flexibility assumption per the witness verification appendix. The fixture's `tolerance_policy` slots remain unresolved: adoption into the governed `DEC-024`/`DEC-026` tolerance record remains `TBD` and is a separate governance step.

## Pass/Fail

`PASS` at the recorded run above: every named test asserted the measured-vs-reference expectations and completed with `ok`. This is software-quality evidence only.

## Solver Version

The in-repo OpenPipeStress solver and benchmark crates at the commit recorded
in `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/_run_records/WORKING_ITEMS_RUN_2026-07-10_TP-E2-VALMANUAL-001.md`; toolchain rustc 1.92.0 / cargo 1.92.0. No packaged release version
exists; release labels remain `TBD`.

## Boundary

This case page documents software verification evidence for an invented
public-original fixture. It is development verification and screening
evidence (claims registry BS-VALID, DEC-081), and it does not settle any
`TBD` threshold or governed tolerance decision.
