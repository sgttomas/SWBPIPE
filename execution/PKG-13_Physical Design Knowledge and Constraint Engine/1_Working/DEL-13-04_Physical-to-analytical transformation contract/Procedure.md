# Procedure: DEL-13-04 Physical-to-analytical transformation contract

**Generated:** 2026-05-03
**Status:** Evidence refresh applied 2026-06-07
**Source posture:** Operational steps describe how to produce/use the contract without inventing unsupported implementation details.

## Purpose

This procedure defines the conservative workflow for checking and extending the DEL-13-04 physical-to-analytical transformation contract. The implemented workflow keeps the physical model as source of truth, derives a solver-ready analytical representation, records transformation diagnostics, and preserves traceability without introducing protected data, hidden defaults, public API expansion, GUI/runtime behavior, external prover claims, or professional compliance claims.

## Prerequisites

1. Confirm current deliverable context:
   - `_CONTEXT.md` identifies DEL-13-04, PKG-13, BACKEND_FEATURE_SLICE, SOW-066, OBJ-014.
   - `_STATUS.md` is in a state that permits setup drafting before overwriting local production documents.
2. Confirm governing references:
   - `_REFERENCES.md`
   - `execution/_Decomposition/SOFTWARE_DECOMP.md`
   - `docs/_Registers/Deliverables.csv`
   - `docs/_Registers/ScopeLedger.csv`
   - `docs/_Registers/ContextBudgetQA.csv`
   - `docs/CONTRACT.md`
   - `docs/SPEC.md`
   - `docs/TYPES.md`
   - `docs/IP_AND_DATA_BOUNDARY.md`
   - `INIT.md`
3. Treat `Dependencies.csv` as the approved DAG-006 mirror/evidence surface. Preserve approved rows as ACTIVE; do not retire, delete, or reclassify them during this setup workflow.
4. Use only source-cleared or invented/public-permissive data in examples and tests. Suspected protected content must be quarantined and escalated under project policy.
5. Inspect current implementation and test evidence:
   - `core/model_transform/physical_to_analytical/contract.py`
   - `core/model_transform/physical_to_analytical/_solver_boundary_adapter.py`
   - `fixtures/domain/invented_physical_source_of_truth_model.json`
   - `tests/test_physical_to_analytical_transform.py`
   - `tests/test_analytical_solver_boundary_adapter.py`
6. Treat current DEL-13-02 and DEL-13-03 four-document kits, `MEMORY.md` files, and TASK run records `TASK_RUN_2026-06-07_1127.md` / `TASK_RUN_2026-06-07_1133.md` as read-only upstream evidence for this refresh.

## Steps

### 1. Establish Transform Boundary

1. Identify the physical model as the source-of-truth input.
2. Identify the analytical model as a derived solver-ready output with `model_role = analytical_solver_model` and `source_model_ref` pointing back to the physical model.
3. Confirm the default target mechanics boundary is the 3D centerline/frame model.
4. Verify supported current physical-record coverage: nodes, straight-pipe elements, supported component metadata when referenced by valid elements, materials, sections, supports, load cases, combinations, diagnostics, unresolved assumptions, and traceability links.
5. Record broader physical-record coverage as `TBD` unless current implementation/tests explicitly define its handling.

### 2. Define Contract Inputs

1. Declare required input surfaces from source-grounded context:
   - canonical model/domain schema;
   - design knowledge and provenance;
   - constraint entities and constraint validation;
   - support/restraint semantics;
   - primitive load semantics;
   - diagnostics/result-envelope conventions.
2. For each unit-bearing input, require explicit unit metadata unless the field is explicitly dimensionless.
3. For each input whose source, provenance, or units are missing, define a diagnostic/warning path instead of a silent default.
4. For current implementation evidence, verify schema-shaped input/output fields used or produced by `contract.py`, including `id`, `model_role`, `coordinate_system`, `nodes`, `elements`, `components`, `materials`, `sections`, `supports`, `load_cases`, `combinations`, `design_knowledge_refs`, `constraint_refs`, `diagnostics`, `unresolved_assumptions`, and `traceability_links`.
5. Leave unimplemented schema references, runtime service envelopes, external prover behavior, and field-level scalar traceability `TBD` unless fixed by accessible implementation evidence.

### 3. Define Contract Outputs

1. Specify the derived analytical model output at the contract level.
2. Specify warning/diagnostic output for non-representable physical data, missing solve-required data, weak provenance, unresolved assumptions, or protected/private-data risk where applicable.
3. Specify traceability links from physical inputs to:
   - analytical output records;
   - warnings/diagnostics;
   - omissions;
   - assumptions;
   - unresolved `TBD` items.
4. For the internal adapter, specify DTO outputs only as internal solver-boundary evidence: nodes, straight-pipe connectivity, property bindings, support targets, load-case records/applications, adapter DTO identity/hash/source-chain records, load-case diagnostics, and diagnostics.
5. Do not encode compliance, approval, certification, external-prover success, release readiness, or professional reliance status in transform or adapter outputs.

### 4. Define Determinism Rules

1. Define deterministic behavior in terms of equivalent input model, units, transform contract version, and configuration.
2. Require repeatable analytical output and repeatable warning/diagnostic output for the same basis.
3. Verify current stable record ordering, stable diagnostic ordering, stable traceability ordering, and adapter DTO hash evidence (`sha256` over sorted JSON payloads with `JCS` vocabulary) where implemented.
4. Record release-grade canonicalization and broader audit-manifest policy as `TBD` unless fixed by a later governed decision.

### 5. Define Warning Tests

1. Create tests for transformation warnings required by SOW-066.
2. Include negative cases for at least:
   - missing solve-required physical input;
   - physical data that lacks a solver-ready analytical representation;
   - missing or ambiguous unit metadata on a unit-bearing value.
3. Use invented or permissive fixtures only.
4. Verify warnings remain diagnostic and do not become compliance or professional acceptance claims.
5. For the current implementation, run:
   - `python3 tests/test_physical_to_analytical_transform.py`
   - `python3 tests/test_analytical_solver_boundary_adapter.py`
   - `python3 -m pytest -q tests/test_physical_to_analytical_transform.py tests/test_analytical_solver_boundary_adapter.py`

### 6. Review Data Boundary

1. Scan the contract and tests for protected standards text, protected tables, code-specific defaults, copied formulas, proprietary catalog values, owner standards, or private project data.
2. Mark uncertain content `TBD` or `protected_suspected` according to policy.
3. Escalate suspected protected content; do not normalize it into public examples.

## Verification

| Check | Expected evidence |
|---|---|
| Four-doc consistency | `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` use the same DEL ID, package, scope, objective, and terminology. |
| Requirement coverage | Specification requirements trace to SOW-066, OBJ-014, project invariants, or local DAG mirror rows. |
| Determinism | Tests or planned tests cover repeated transform behavior. |
| Warning behavior | Tests or planned tests cover warnings for non-representable physical data. |
| Traceability | Contract records physical-to-analytical links or explicit unresolved gaps. |
| Unit safety | Missing/ambiguous unit metadata produces findings, not defaults. |
| Data boundary | No protected/private/proprietary engineering content is introduced. |
| Professional boundary | Outputs avoid certification, approval, compliance, or professional reliance claims. |
| Internal adapter boundary | Adapter accepts only strict `analytical_solver_model` mappings and reports DTO/load/orientation/property failures as diagnostics. |
| Dependency preservation | Existing local dependency rows remain unchanged during evidence refresh; historical row IDs remain audit evidence under the current DAG-006 coordination basis. |

## Records

Maintain the following deliverable-local records:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `_STATUS.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_run_records/**`

Check the following repo-level implementation and test evidence when verifying this deliverable:

- `core/model_transform/physical_to_analytical/contract.py`
- `core/model_transform/physical_to_analytical/_solver_boundary_adapter.py`
- `fixtures/domain/invented_physical_source_of_truth_model.json`
- `tests/test_physical_to_analytical_transform.py`
- `tests/test_analytical_solver_boundary_adapter.py`

Implementation records are no longer `TBD` for the current transform, internal adapter, canonical fixture, and focused tests listed above. Remaining `TBD` records include final transform-loss taxonomy, release thresholds, external prover behavior, GUI/runtime/API integration, persisted/handoff readiness, broader physical-record coverage, human acceptance, and professional/code-compliance boundaries.

## D-41 R5 T2B PDU-047 Check

Treat section-property numeric evidence as evidence for the carried section-property values only. Do not use it to infer broader 3D frame suitability, shell/solid equivalence, release readiness, or professional acceptance. Record that missing independent basis as a held residual.

For PDU-023, require paired source/target scalar paths only after quantity metadata passes current validation. Do not synthesize a result-envelope continuation.
