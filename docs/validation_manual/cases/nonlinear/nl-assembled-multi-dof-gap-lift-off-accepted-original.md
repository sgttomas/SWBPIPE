---
doc_id: OPS-VALIDATION-MANUAL-CASE-NL-ASSEMBLED-MULTI-DOF-GAP-LIFT-OFF-ACCEPTED-ORIGINAL
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

# NL-ASSEMBLED-MULTI-DOF-GAP-LIFT-OFF-ACCEPTED-ORIGINAL

| Item | Record |
|---|---|
| Suite | Nonlinear support regression suite (`DEL-09-03`), crate `validation/benchmarks/nonlinear/` |
| Evidence class | Mechanics verification (nonlinear supports) |
| Evidence state | `DRAFT_EVIDENCE` |
| Independent reference | `validation/hand_calcs/nonlinear/assembled_multi_support_gap_lift_off_acceptance.md` |
| Provenance | Invented public-original fixture data only (recorded in the reference note and fixture metadata) |

## Test Purpose And Problem Statement

This note documents `NL-ASSEMBLED-MULTI-DOF-GAP-LIFT-OFF-ACCEPTED-ORIGINAL`,
an invented assembled nonlinear validation fixture that exercises lift-off
release on `Ux` and gap closure on `Uy` in one frame solve. It is a narrow
non-seed acceptance companion in the multi-support validation fixture set.

All inputs are invented or user-entered fixture data; nothing is copied from
protected standards, commercial software examples, or proprietary data.

## Input Model

The machine-readable input model is the fixture constructor `assembled_multi_dof_gap_lift_off_acceptance_fixture()` (fixture id `NL-ASSEMBLED-MULTI-DOF-GAP-LIFT-OFF-ACCEPTED-ORIGINAL`) in `validation/benchmarks/nonlinear/src/lib.rs`, carrying explicit fixture-local units and public-original provenance metadata. The reference note records the same invented inputs with the longhand derivation.

## Expected Result And Independent Reference

The expected values and their derivation are recorded in the independent
reference note `validation/hand_calcs/nonlinear/assembled_multi_support_gap_lift_off_acceptance.md` and mirrored in the fixture's expected-value
slots. The reference derivation uses elementary open mechanics only.

## Software Result And Reproduction

The measured-vs-reference comparison executes inside the named suite
test(s) `multisupport_acceptance_inventory_uses_narrow_dec_046_policy`, which run the current in-repo solver path on the fixture
and assert agreement with the reference expectations.

Reproduction (from `projects/chirality-piping`):

```bash
cargo test --manifest-path validation/benchmarks/nonlinear/Cargo.toml multisupport_acceptance_inventory_uses_narrow_dec_046_policy
```

Recorded run: 2026-07-10, toolchain rustc 1.92.0 / cargo 1.92.0; suite
result `ok. 19 passed; 0 failed; 0 ignored` with the named test(s) passing. The full
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

Convergence is judged under the narrow governed `DEC-046` multi-support records (`DEC-046-CV-B-multisupport-active-set-count-validation-v1` and companions in `validation/benchmarks/nonlinear/multisupport_*.dec046.json`): active-set changed-support-count residual with relative tolerance `0.0`, absolute floor `0.0`, max-iteration cap `4`, plus the multi-support free-DOF force/moment, free-DOF work, general-energy, and displacement/reaction-delta records — scoped to the accepted public-original multi-DOF / multi-support fixture set only. Fixture-local overrides may only tighten; loosening is a new governance event. Broader release/production thresholds remain `TBD`.

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
