---
doc_id: OPS-VALIDATION-MANUAL-CASE-NL-NONCONVERGENCE-LIMIT-ORIGINAL
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

# NL-NONCONVERGENCE-LIMIT-ORIGINAL

| Item | Record |
|---|---|
| Suite | Nonlinear support regression suite (`DEL-09-03`), crate `validation/benchmarks/nonlinear/` |
| Evidence class | Mechanics verification (nonlinear supports) |
| Evidence state | `DRAFT_EVIDENCE` |
| Independent reference | `validation/hand_calcs/nonlinear/unresolved_nonconvergence.md` |
| Provenance | Invented public-original fixture data only (recorded in the reference note and fixture metadata) |

## Test Purpose And Problem Statement

Invented non-convergence fixture for a support that changes state at the
configured iteration limit.

All inputs are invented or user-entered fixture data; nothing is copied from
protected standards, commercial software examples, or proprietary data.

## Input Model

The machine-readable input model is the fixture constructor `unresolved_nonconvergence_fixture()` (fixture id `NL-NONCONVERGENCE-LIMIT-ORIGINAL`) in `validation/benchmarks/nonlinear/src/lib.rs`, carrying explicit fixture-local units and public-original provenance metadata. The reference note records the same invented inputs with the longhand derivation.

## Expected Result And Independent Reference

The expected values and their derivation are recorded in the independent
reference note `validation/hand_calcs/nonlinear/unresolved_nonconvergence.md` and mirrored in the fixture's expected-value
slots. The reference derivation uses elementary open mechanics only.

## Software Result And Reproduction

The measured-vs-reference comparison executes inside the named suite
test(s) `nonconvergence_fixture_reports_failure_diagnostic`, `expected_reports_preserve_warning_and_failure_statuses`, which run the current in-repo solver path on the fixture
and assert agreement with the reference expectations.

Reproduction (from `projects/chirality-piping`):

```bash
cargo test --manifest-path validation/benchmarks/nonlinear/Cargo.toml nonconvergence_fixture_reports_failure_diagnostic
cargo test --manifest-path validation/benchmarks/nonlinear/Cargo.toml expected_reports_preserve_warning_and_failure_statuses
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

This case verifies diagnostic behavior, not numeric agreement: the fixture must fail to converge within the iteration cap and must report a single `NonConvergence` failure diagnostic (message records `did not converge after 4 iterations`). No success state is substituted. The fixture's `tolerance_policy` slots remain unresolved (`TBD` per `DEC-024`/`DEC-026`).

## Pass/Fail

`PASS` (expected-diagnostic case) at the recorded run: the fixture failed to converge exactly as the reference predicts and reported the expected `NonConvergence` failure diagnostic. This is software-quality evidence only.

## Solver Version

The in-repo OpenPipeStress solver and benchmark crates at the commit recorded
in `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/_run_records/WORKING_ITEMS_RUN_2026-07-10_TP-E2-VALMANUAL-001.md`; toolchain rustc 1.92.0 / cargo 1.92.0. No packaged release version
exists; release labels remain `TBD`.

## Boundary

This case page documents software verification evidence for an invented
public-original fixture. It is not a release-readiness, professional
approval, certification, sealing, authentication, or code-compliance record,
and it does not settle any `TBD` threshold or governed tolerance decision.
