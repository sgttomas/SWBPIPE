---
doc_id: OPS-VALIDATION-MANUAL-CASE-STRESS-PRESSURE-MEMBRANE-ORIGINAL
doc_kind: governance.validation_manual_case
status: draft_evidence
created: 2026-07-10
generated_by: docs/validation_manual/cases/generate_validation_case_pages.py
refs:
  - rel: implements
    to: DEL-09-04
  - rel: evidences
    to: DEL-09-02
---

# STRESS-PRESSURE-MEMBRANE-ORIGINAL

| Item | Record |
|---|---|
| Suite | Stress recovery benchmark suite (`DEL-09-02`), crate `validation/benchmarks/stress/` |
| Evidence class | Mechanics verification (stress recovery) |
| Evidence state | `DRAFT_EVIDENCE` |
| Independent reference | `validation/hand_calcs/stress/pressure_membrane.md` |
| Provenance | Invented public-original fixture data only (recorded in the reference note and fixture metadata) |

## Test Purpose And Problem Statement

Verify thin-wall pressure membrane stress recovery from explicit user-supplied pressure basis inputs, within the mechanics-only stress recovery boundary.

All inputs are invented or user-entered fixture data; nothing is copied from
protected standards, commercial software examples, or proprietary data.

## Input Model

The machine-readable input model is the fixture constructor `pressure_membrane_fixture()` (fixture id `STRESS-PRESSURE-MEMBRANE-ORIGINAL`) in `validation/benchmarks/stress/src/lib.rs`, carrying explicit fixture-local units and public-original provenance metadata. The reference note records the same invented inputs with the longhand derivation.

## Expected Result And Independent Reference

The expected values and their derivation are recorded in the independent
reference note `validation/hand_calcs/stress/pressure_membrane.md` and mirrored in the fixture's expected-value
slots. The reference derivation uses elementary open mechanics only.

## Software Result And Reproduction

The measured-vs-reference comparison executes inside the named suite
test(s) `recovers_pressure_membrane_fixture`, which run the current in-repo solver path on the fixture
and assert agreement with the reference expectations.

Reproduction (from `projects/chirality-piping`):

```bash
cargo test --manifest-path validation/benchmarks/stress/Cargo.toml recovers_pressure_membrane_fixture
```

Recorded run: 2026-07-10, toolchain rustc 1.92.0 / cargo 1.92.0; suite
result `ok. 22 passed; 0 failed; 0 ignored` with the named test(s) passing. The full
suite output and the exact commit are recorded in the run record
`execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/_run_records/WORKING_ITEMS_RUN_2026-07-10_TP-E2-VALMANUAL-001.md`.

Headless-runner status: PR #287 bound `run-benchmark` and `run-regression`
for the committed `DEL-10-05` benchmark/regression payload families. That
bounded evidence does not establish a runner payload or per-case runner
execution for this page, so the exact reproduction command here remains the
crate test above. `export-results` is the only structured runner verb that
remains a stub. The bound runner path and its exact witnessed cases are recorded
in
[headless_runner_reproduction.md](../../headless_runner_reproduction.md).

## Tolerance

Numeric agreement assertions in this suite use the fixture-local absolute assertion epsilon `INTERNAL_ASSERTION_EPSILON = 1.0e-9` recorded in `validation/benchmarks/stress/src/lib.rs`; structural and blocking-diagnostic expectations are asserted exactly. The fixture's `tolerance_policy` slots remain unresolved: the governed per-quantity-kind tolerance record required by `DEC-024`/`DEC-026` remains `TBD`, and this page neither tightens nor loosens any governed value.

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
