---
doc_id: OPS-VALIDATION-MANUAL-CASE-NL-FRICTION-STICK-SLIDE-ORIGINAL
doc_kind: governance.validation_manual_case
status: draft_evidence
created: 2026-07-10
generated_by: docs/validation_manual/cases/generate_validation_case_pages.py
refs:
  - rel: implements
    to: DEL-09-04
  - rel: evidences
    to: DEL-09-03
---

# NL-FRICTION-STICK-SLIDE-ORIGINAL

| Item | Record |
|---|---|
| Suite | Nonlinear support regression suite (`DEL-09-03`), crate `validation/benchmarks/nonlinear/` |
| Evidence class | Mechanics verification (nonlinear supports) |
| Evidence state | `DRAFT_EVIDENCE` |
| Independent reference | `validation/hand_calcs/nonlinear/friction_transition.md` |
| Provenance | Invented public-original fixture data only (recorded in the reference note and fixture metadata) |

## Test Purpose And Problem Statement

Invented friction fixture for a pair of supports: one remains sticking and one
remains sliding under explicit normal and tangential trial reactions.

All inputs are invented or user-entered fixture data; nothing is copied from
protected standards, commercial software examples, or proprietary data.

## Input Model

The machine-readable input model is the fixture constructor `friction_transition_fixture()` (fixture id `NL-FRICTION-STICK-SLIDE-ORIGINAL`) in `validation/benchmarks/nonlinear/src/lib.rs`, carrying explicit fixture-local units and public-original provenance metadata. The reference note records the same invented inputs with the longhand derivation.

## Expected Result And Independent Reference

The expected values and their derivation are recorded in the independent
reference note `validation/hand_calcs/nonlinear/friction_transition.md` and mirrored in the fixture's expected-value
slots. The reference derivation uses elementary open mechanics only.

## Software Result And Reproduction

The measured-vs-reference comparison executes inside the named suite
test(s) `fixtures_are_public_original_and_unit_aware`, `active_set_gap_lift_off_and_friction_outcomes_are_deterministic`, which run the current in-repo solver path on the fixture
and assert agreement with the reference expectations.

Reproduction (from `projects/chirality-piping`):

```bash
cargo test --manifest-path validation/benchmarks/nonlinear/Cargo.toml fixtures_are_public_original_and_unit_aware
cargo test --manifest-path validation/benchmarks/nonlinear/Cargo.toml active_set_gap_lift_off_and_friction_outcomes_are_deterministic
```

Recorded run: 2026-07-10, toolchain rustc 1.92.0 / cargo 1.92.0; suite
result `ok. 19 passed; 0 failed; 0 ignored` with the named test(s) passing. The full
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

This classifier-level case asserts exact deterministic expected outcomes (support state, reaction, and diagnostic expectations) — no numeric tolerance band applies. The governed `DEC-046` convergence-tolerance record applies to the assembled global loop, not to this classifier-level case, and the fixture's `tolerance_policy` slots remain unresolved (`TBD` per `DEC-024`/`DEC-026`).

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
