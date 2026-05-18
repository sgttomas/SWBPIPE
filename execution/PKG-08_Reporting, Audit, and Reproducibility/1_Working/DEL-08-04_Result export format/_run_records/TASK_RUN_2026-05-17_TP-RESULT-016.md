---
doc_id: TASK-RUN-TP-RESULT-016
doc_kind: deliverable.task_run
status: completed
created: 2026-05-17
task: TP-RESULT-016 Result Export Vocabulary
deliverable_id: DEL-08-04
package_id: PKG-08
---

# TASK RUN - TP-RESULT-016 Result Export Vocabulary

## Request

Resolve the TP-VERIFY-012 result-export vocabulary findings by adding governed
result-export vocabulary for assembled load-vector evidence and station
resultant result sets. Do not add solver behavior, report rendering, CLI/API
behavior, persistence behavior, rule checks, allowables, or professional/code
compliance wording.

## Loaded Truth Set

- `AGENTS.md`
- `agents/AGENT_WORKING_ITEMS.md`
- `agents/AGENT_TASK.md`
- `docs/CONTRACT.md`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `docs/IP_AND_DATA_BOUNDARY.md`
- `execution/_Coordination/_COORDINATION.md`
- `execution/_DAG/_LATEST.md`
- `execution/_Coordination/DEV-001_BLOCKER_QUEUE.md`
- `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-04_Result export format/_CONTEXT.md`
- `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-04_Result export format/_STATUS.md`
- `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-04_Result export format/_REFERENCES.md`
- `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-04_Result export format/_DEPENDENCIES.md`
- `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-04_Result export format/Dependencies.csv`
- `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-04_Result export format/MEMORY.md`
- Primary deliverable artifacts: `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`
- TP-PHYS-015 / TP-VERIFY-012 evidence records for DEL-08-04 and the canonical mechanics result envelope note.

## Changes

- Added `ResultSetType` to `schemas/results.schema.yaml` with first-class
  `load_vector_evidence` and `station_resultants` result-set categories.
- Added result metadata vocabulary for assembled solver load-vector evidence:
  nodal force/moment components, node location, and
  `assembled_solver_load_vector` basis.
- Added Rust `ResultSetType` vocabulary and validation for unsupported result
  set types in `core/reporting/result_export`.
- Added Rust tests proving load-vector and station-resultant result-set
  evidence validate through the result-export crate, and proving the prior
  ad-hoc `mechanics_solve_result_boundary` set type is now blocking.
- Updated the TP-PHYS-015A mechanics benchmark envelope fixture to use the
  governed `mechanics` result-set type so stricter result-export validation
  remains compatible with existing benchmark evidence.
- Updated `tests/test_results_schema.py` to assert the new schema vocabulary.

## Validation

- `python3 tests/test_results_schema.py` - passed.
- `cargo test --manifest-path core/reporting/result_export/Cargo.toml` - passed
  10 tests.
- `cargo test --manifest-path validation/benchmarks/mechanics/Cargo.toml tp_phys_015a_canonical_payload_builds_result_boundary_evidence` - passed.

## Boundaries Preserved

- No solver behavior, public runtime command, CLI/API surface, GUI/report
  rendering, persistence behavior, rule-pack behavior, allowables, design-code
  checks, fatigue checks, release record, acceptance record, lifecycle state,
  dependency register, DAG/blocker file, or review disposition was changed.
- The vocabulary classifies result-boundary evidence only. It does not make a
  release, professional, certification, sealing, approval, authentication, or
  code-compliance claim.

## Remaining Follow-Up

- TP-RESULT-017 should align serialized full-envelope fixture behavior with
  the accepted vocabulary and crate/schema wrapper expectations.
- TP-RUNNER-013 should consume the accepted DEL-08-04 shape for headless
  full-envelope validation.
- Per-value multi-hop trace chains, checksum/canonicalization ownership,
  shared diagnostic vocabulary, and governed stress section input ownership
  remain cross-deliverable ruling topics.
