# TASK RUN - TP-VERIFY-012A Result Export Integration Gap Triage

## Identity

- Task: TP-VERIFY-012A result-export integration gap triage
- Requested by: WORKING_ITEMS orchestrator
- Agent posture: canonical TASK
- DeliverableID: DEL-08-04
- PackageID: PKG-08
- ScopeItems: SOW-046
- Objectives: OBJ-007; OBJ-009
- Timestamp: 2026-05-17 13:03 MDT

## Git State

- Initial `git status --short --branch`: `## main...origin/main`
- TASK writes are limited to this run record and DEL-08-04 `MEMORY.md`.

## Required Local Reads

Read before acting:

- `_CONTEXT.md`
- `_STATUS.md`
- `_REFERENCES.md`
- `_DEPENDENCIES.md`
- `Dependencies.csv`
- `MEMORY.md`
- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`

Inspected read-only implementation and coordination evidence:

- `schemas/results.schema.yaml`
- `core/reporting/result_export/src/lib.rs`
- `tests/test_results_schema.py`
- `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-04_Result export format/_run_records/TASK_RUN_2026-05-17_124230_TP-PHYS-015D1.md`
- `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-01_Mechanics benchmark suite/_run_records/TASK_RUN_2026-05-17_TP-PHYS-015A.md`
- `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-01_Mechanics benchmark suite/_run_records/PARENT_FANIN_2026-05-17_1248_TP-PHYS-015.md`
- `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-05_Headless CLI and structured I-O analysis runner/_run_records/TASK_RUN_2026-05-17_124145_TP-PHYS-015D2.md`
- `validation/hand_calcs/mechanics/tp_phys_015a_canonical_solve_result_envelope.md`
- `docs/_Registers/Deliverables.csv`
- `docs/_Registers/ScopeLedger.csv`
- `execution/_DAG/_LATEST.md`
- `execution/_Coordination/DEV-001_BLOCKER_QUEUE.md`

## Triage Matrix

| Gap | Classification | Recommended next owner | Rationale | Non-goals |
|---|---|---|---|---|
| Load-vector vocabulary | `READY_FOR_SCHEMA_TRANCHE` | `DEL-08-04` primary, with `DEL-09-01` evidence input | TP-PHYS-015A proves load-vector force and moment evidence can be represented generically, but `schemas/results.schema.yaml` has no first-class result-set type or metadata vocabulary for assembled nodal vector entries, global DOF names, or nodal equivalent force/moment components. This is a result-export vocabulary gap, not a solver behavior gap. | Do not add solver assembly behavior, public runtime commands, CLI/API surface, or load expansion rules. |
| Station-resultant set typing | `READY_FOR_SCHEMA_TRANCHE` | `DEL-08-04` primary, with `DEL-09-01` and `DEL-09-02` evidence input | Existing metadata supports component, coordinate system, location, basis, and sign convention, and TP-PHYS-015A/C use that path. The gap is that `ResultSet.set_type` lacks a dedicated `station_resultants` value, so station resultants are currently carried as generic `mechanics` quantities. | Do not add allowables, design-code categories, stress indices, fatigue checks, or professional/code-compliance wording. |
| Per-value multi-hop trace chains | `NEEDS_CROSS_DELIVERABLE_RULING` | Human/WORKING_ITEMS ruling between `DEL-13-04` trace-source ownership and `DEL-08-04` export-carrier ownership | The desired chain spans physical source, analytical model, adapter DTO, solver input, and result evidence. DEL-08-04 can carry refs, but the semantics of source-to-adapter trace links are owned by the transformation contract. A schema tranche should wait until ownership and minimal chain shape are assigned. | Do not invent hidden defaults, duplicate physical-to-analytical trace semantics inside result export alone, or mutate dependency/DAG records in this TASK. |
| Crate/schema shape mismatch | `READY_FOR_RUNTIME_TEST_TRANCHE` | `DEL-08-04` | The schema has root wrapper fields and optional per-value diagnostics that the Rust crate does not mirror. The crate validates bounded in-memory envelopes and emits envelope-level diagnostics. Because the schema vocabulary already exists, the next work should be a result-export crate/test alignment tranche, not a new schema-decision tranche. | Do not change schemas in this triage; do not add public API/CLI/report/persistence behavior. |
| Displacement/rotation dimension mapping | `READY_FOR_RUNTIME_TEST_TRANCHE` | `DEL-09-01` evidence producer with `DEL-08-04` export validation | Accepted PKG-02/result-export dimensions use `length` for displacement values and `angle` for rotations. TP-PHYS-014 mechanics benchmark records still contain local labels such as `displacement` and `rotation`, while TP-PHYS-015A manually maps exported evidence to `length` and `angle`. The next step is a focused mapping/test tranche proving that benchmark-local labels are normalized before result-export validation. | Do not reopen PKG-02 dimension vocabulary, add alternate schema dimensions, or silently accept unit/dimension omissions. |

## Ranked Next-Work Recommendation

1. `DEL-08-04` schema tranche for load-vector evidence and station-resultant set vocabulary, using TP-PHYS-015A/C records as fixtures.
2. `DEL-08-04` runtime/test tranche to align `core/reporting/result_export` with existing schema wrapper/per-value diagnostic fields.
3. Cross-deliverable ruling on trace-chain ownership between `DEL-13-04` and `DEL-08-04`; only then draft a minimal source-link schema tranche.
4. `DEL-09-01` plus `DEL-08-04` runtime/test tranche for explicit displacement/rotation dimension normalization to `length`/`angle`.

## Boundary Closeout

This TASK records triage evidence only. It did not edit schemas, Rust code, tests, lifecycle state, dependency registers, DAG/blocker files, review dispositions, release records, acceptance records, public runtime/API/CLI/report/persistence behavior, or professional/code-compliance surfaces.
