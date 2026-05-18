---
doc_id: TASK-RUN-TP-WITNESS-023B
doc_kind: execution.task_run
status: success
created: 2026-05-17
deliverable_id: DEL-09-02
package_id: PKG-09
task_id: TP-WITNESS-023B
---

# TASK RUN - TP-WITNESS-023B Formal OpenMath Hand-Calc Witness Pilot

## Input Echo

- DeliverablePath:
  `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-02_Stress recovery benchmark suite`
- Objective: add a machine-readable OpenMath-style formal hand-calc witness for
  TP-PHYS-015 / TP-STRESS-016 and validate it against existing result-export
  evidence.
- Required local state read: `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`,
  `_DEPENDENCIES.md`, `MEMORY.md`, current TP-PHYS-015 hand-calc, stress
  benchmark source, and TP-SECTION-021 result-export fixture.

## Write Scope Used

- `validation/witness/**`
- `validation/hand_calcs/stress/**`
- `tests/test_calculation_witness.py`
- this deliverable `MEMORY.md`
- this deliverable `_run_records/**`

## Outputs

- Added strict witness schema:
  `validation/witness/schemas/openmath_calculation_witness.schema.json`.
- Added repo-local OpenMath content-dictionary metadata:
  `validation/witness/content_dictionaries/ops_mechanics_cd.json`.
- Added authoritative witness fixture:
  `validation/witness/fixtures/tp_phys_015_section_property_stress_witness.json`.
- Added deterministic validator/renderer:
  `validation/witness/tools/witness_validator.py`.
- Added generated human/interchange renderings:
  `validation/hand_calcs/stress/generated/tp_phys_015_section_property_stress_witness.md`;
  `validation/witness/generated/tp_phys_015_section_property_stress_witness.mathml`.
- Added focused direct-run tests:
  `tests/test_calculation_witness.py`.
- Updated stress hand-calc inventory and TP-PHYS-015 prose note to point to
  the formal witness.

## Validation

- `python3 validation/witness/tools/witness_validator.py --write-generated --check-generated` passed.
- `python3 tests/test_calculation_witness.py` passed.
- `python3 tests/test_results_schema.py` passed.
- `cargo test --manifest-path validation/benchmarks/stress/Cargo.toml` passed:
  17 tests.
- `cargo test --manifest-path core/reporting/result_export/Cargo.toml` passed:
  12 tests.

## Boundary

The witness evaluator is validation-local and interprets only the approved
OpenMath arithmetic phrasebook for this pilot. It does not call production
solver, section-property, or stress-recovery implementation code, and it does
not introduce protected standards content, allowables, SIF/flexibility values,
fatigue criteria, code-compliance decisions, release claims, certification,
sealing, approval, authentication, or professional reliance conclusions.
