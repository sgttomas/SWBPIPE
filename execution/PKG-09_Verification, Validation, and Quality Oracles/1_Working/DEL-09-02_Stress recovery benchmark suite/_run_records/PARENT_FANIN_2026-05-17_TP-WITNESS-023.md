---
doc_id: PARENT-FANIN-TP-WITNESS-023
doc_kind: execution.parent_fanin
status: success
created: 2026-05-17
task_id: TP-WITNESS-023
---

# PARENT FAN-IN - TP-WITNESS-023 Formal OpenMath Hand-Calc Witness Pilot

## Fan-In Summary

- `TP-WITNESS-023A` / `DEL-09-04`: validation manual and strategy now define
  formal hand-calc witnesses as authoritative machine-readable evidence with
  deterministic human-facing renderings.
- `TP-WITNESS-023B` / `DEL-09-02`: added the TP-PHYS-015 /
  TP-STRESS-016 OpenMath-style witness, validator/renderer, generated
  Markdown, generated Strict Content MathML, and focused tests.
- `TP-WITNESS-023C` / `DEL-08-04`: confirmed existing result-export values are
  sufficient for witness comparison without schema or exporter changes.

## Validation

- `python3 validation/witness/tools/witness_validator.py --write-generated --check-generated` passed.
- `python3 tests/test_calculation_witness.py` passed.
- `python3 tests/test_results_schema.py` passed.
- `cargo test --manifest-path validation/benchmarks/stress/Cargo.toml` passed:
  17 tests.
- `cargo test --manifest-path core/reporting/result_export/Cargo.toml` passed:
  12 tests.

## Review Notes

- The witness JSON is the authoritative source. Generated Markdown and MathML
  are checked for deterministic reproducibility from the same machine-readable
  artifact.
- The validation-local phrasebook permits only `arith1.plus`, `arith1.minus`,
  `arith1.times`, `arith1.divide`, `arith1.power`, and `nums1.pi` for this
  pilot.
- Negative tests cover unsupported OpenMath symbols, dimension mismatch, stale
  generated Markdown, tampered formula output, missing provenance, and OPS
  result mismatch.
- The comparator reads existing result-export JSON and does not call production
  section-property, stress-recovery, or solver implementation code.

## Boundary

No lifecycle/status file, dependency register, DAG file, blocker queue, review
disposition, candidate row, release record, acceptance record, public API/CLI/
GUI/report/persistence behavior, protected standards content, private or
proprietary data, allowables, SIF/flexibility data, fatigue/design-code check,
professional reliance claim, code-compliance claim, release statement, or
human-acceptance statement was changed or introduced.
