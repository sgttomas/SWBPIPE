---
doc_id: TASK-RUN-TP-RESULT-017
doc_kind: deliverable.task_run
status: completed
created: 2026-05-17
task: TP-RESULT-017 Result Envelope Serialization Alignment
deliverable_id: DEL-08-04
package_id: PKG-08
---

# TASK RUN - TP-RESULT-017 Result Envelope Serialization Alignment

## Request

Align the result-export crate test surface with the schema wrapper and the
result-boundary vocabulary accepted in TP-RESULT-016. Add an invented serialized
fixture for `MECH-TP-PHYS-015-CANONICAL-SOLVE-RESULT-ENVELOPE`.

## Loaded Truth Set

- DEL-08-04 deliverable-local truth set from TP-RESULT-016.
- `schemas/results.schema.yaml`.
- `core/reporting/result_export`.
- `tests/test_results_schema.py`.
- TP-PHYS-015A mechanics result-envelope evidence and TP-VERIFY-012A gap
  triage.

## Changes

- Added `result_export_document` serialization in
  `core/reporting/result_export` to emit the schema root wrapper, nested
  `solver_version`, export-format status, downstream-use constants,
  reproducibility refs, diagnostics, result sets, and quantity values.
- Added crate-side per-value diagnostics on `QuantityResult` and serialized
  them when present.
- Added invented fixture
  `fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json`
  for `MECH-TP-PHYS-015-CANONICAL-SOLVE-RESULT-ENVELOPE`.
- Added Rust fixture comparison proving the crate serialization output matches
  the invented serialized fixture.
- Updated `tests/test_results_schema.py` to structurally inspect the TP-PHYS-015
  serialized fixture and validate it against `schemas/results.schema.yaml` when
  the optional JSON Schema dependency is available.

## Validation

- `python3 tests/test_results_schema.py` - passed, including JSON Schema
  validation in this environment.
- `cargo test --manifest-path core/reporting/result_export/Cargo.toml` - passed
  11 tests.
- `cargo test --manifest-path validation/benchmarks/mechanics/Cargo.toml tp_phys_015a_canonical_payload_builds_result_boundary_evidence` - passed.

## Boundaries Preserved

- No solver behavior, public CLI/API transport, GUI/report rendering,
  persistence behavior, rule-pack behavior, allowables, design-code checks,
  release record, acceptance record, lifecycle state, dependency register,
  DAG/blocker file, or review disposition was changed.
- Serialization remains a schema-first result-export crate boundary. It does
  not define final public transport, package scripts, process policy, external
  formats, release tolerance, professional approval, certification, sealing,
  authentication, or code compliance.

## Remaining Follow-Up

- TP-RUNNER-013 should use the accepted serialized fixture to prove headless
  result-envelope reference and optional full-payload validation.
- Cross-deliverable rulings remain needed for per-value multi-hop trace-chain
  ownership, checksum/canonicalization ownership, shared diagnostic vocabulary,
  and governed stress section input ownership.
