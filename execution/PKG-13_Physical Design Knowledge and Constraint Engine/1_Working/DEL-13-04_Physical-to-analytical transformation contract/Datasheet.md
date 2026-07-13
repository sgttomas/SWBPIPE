# Datasheet: DEL-13-04 Physical-to-analytical transformation contract

<!-- D41-R5-T7-PDU055-CURRENTNESS -->
## D-41 R5 T7 PDU-055 current declaration

Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.

Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

PDU-055 cited claim(s): `DEL-13-04-DECL-002`.

**Generated:** 2026-05-03
**Status:** Evidence refresh applied 2026-06-07
**Source posture:** Current implementation evidence is cited where available; unsupported particulars remain marked `TBD` or `ASSUMPTION`.

## Identification

| Field | Value | Source |
|---|---|---|
| Deliverable ID | DEL-13-04 | `_CONTEXT.md` |
| Name | Physical-to-analytical transformation contract | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row DEL-13-04 |
| Package ID | PKG-13 | `_CONTEXT.md` |
| Package name | Physical Design Knowledge and Constraint Engine | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` PKG-13 |
| Deliverable type | BACKEND_FEATURE_SLICE | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row DEL-13-04 |
| Scope coverage | SOW-066 | `_CONTEXT.md`; `docs/_Registers/ScopeLedger.csv` row SOW-066 |
| Objective support | OBJ-014 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` objective mapping |
| Context envelope | L, WATCH | `_CONTEXT.md`; `docs/_Registers/ContextBudgetQA.csv` row DEL-13-04 |

## Attributes

| Attribute | Current value | Source / note |
|---|---|---|
| Primary function | Derive a solver-ready analytical model from the physical model. | SOW-066 in `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| Determinism requirement | Transformation is deterministic. | SOW-066 in `_CONTEXT.md`; `docs/_Registers/ScopeLedger.csv` row SOW-066 |
| Warning obligation | Transformation warnings are recorded when physical design data cannot be represented analytically. | SOW-066 in `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEL-13-04 row |
| Traceability obligation | Transformation traceability is part of OBJ-014 and DEL-13-04 decomposition context. | `execution/_Decomposition/SOFTWARE_DECOMP.md` OBJ-014 and DEL-13-04 |
| Physical model role | Physical source-of-truth model anchors editable design data. | `docs/SPEC.md` section 3; `docs/TYPES.md` model registry |
| Analytical model role | Solver-ready idealization / analysis basis derived from the physical model. | `execution/_Decomposition/SOFTWARE_DECOMP.md` glossary row "Analytical Model" |
| Target mechanics boundary | Primary global analysis model is a 3D centerline/frame model. | `docs/CONTRACT.md` OPS-K-MECH-1; `INIT.md` project principles |
| Unit handling | Unit-bearing physical values crossing boundaries require explicit unit metadata. | `docs/SPEC.md` section 4; `docs/CONTRACT.md` OPS-K-UNIT-1 |
| Missing required values | Missing solve-required values are explicit findings, not silent defaults. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md` missing-data warning classes |
| Protected-data boundary | Public artifacts must not bundle protected standards data, tables, or proprietary values. | `docs/CONTRACT.md` OPS-K-IP-1; `docs/IP_AND_DATA_BOUNDARY.md` |
| Professional boundary | Software must not claim certification, sealing, approval, authentication, or code compliance. | `docs/CONTRACT.md` OPS-K-AUTH-1; `INIT.md` |
| Transform implementation path | `core/model_transform/physical_to_analytical/contract.py` | Current implementation evidence and `MEMORY.md` |
| Internal solver-boundary adapter path | `core/model_transform/physical_to_analytical/_solver_boundary_adapter.py` | Current implementation evidence and `tests/test_analytical_solver_boundary_adapter.py` |
| Canonical physical fixture | `fixtures/domain/invented_physical_source_of_truth_model.json` | Current fixture evidence and `tests/test_physical_to_analytical_transform.py` |
| Focused transform tests | `tests/test_physical_to_analytical_transform.py` | Current transform contract, diagnostic, traceability, and fixture tests |
| Focused adapter tests | `tests/test_analytical_solver_boundary_adapter.py` | Current internal solver-boundary adapter tests |
| Current transform result shape | `TransformResult` exposes `analytical_model`, `diagnostics`, `traceability_links`, and `has_blocking_findings`. | `core/model_transform/physical_to_analytical/contract.py` |
| Current transform settings | Default `TransformSettings` uses `analytical_model_id = ANALYTICAL-DERIVED` and `contract_version = DEL-13-04-0.1`. | `core/model_transform/physical_to_analytical/contract.py` |
| Exact transform-loss taxonomy | TBD | Implemented `PTA-*` and `ASBA-*` diagnostics are current code/test evidence, not a final release taxonomy. |
| Exact dependency versions | TBD | `_CONTEXT.md` Architecture Basis Injection "Still TBD". |

## Conditions

### Upstream Inputs

The local `Dependencies.csv` is an approved DAG-006 mirror/evidence surface. It lists these ACTIVE upstream dependencies for DEL-13-04:

| Target | Relationship summary | Source |
|---|---|---|
| DEL-00-01 Architecture decision record baseline | Architecture-basis predecessor. | `Dependencies.csv` row DAG-002-E0660 |
| DEL-00-02 Repository and module boundary architecture | Architecture-basis predecessor. | `Dependencies.csv` row DAG-002-E0661 |
| DEL-00-03 Application service command-query-job model | Architecture-basis predecessor. | `Dependencies.csv` row DAG-002-E0662 |
| DEL-00-04 Persistence and schema versioning architecture | Architecture-basis predecessor. | `Dependencies.csv` row DAG-002-E0663 |
| DEL-00-06 Diagnostics, warning, and result-envelope contract | Architecture-basis predecessor. | `Dependencies.csv` row DAG-002-E0664 |
| DEL-00-07 API boundary and adapter contract map | Architecture-basis predecessor. | `Dependencies.csv` row DAG-002-E0665 |
| DEL-00-08 Layered software test and acceptance strategy | Architecture-basis predecessor. | `Dependencies.csv` row DAG-002-E0666 |
| DEL-02-01 Canonical domain model schema | Physical-to-analytical transform starts from the canonical physical/domain model. | `Dependencies.csv` row DAG-002-E0772 |
| DEL-13-01 Design knowledge schema and provenance model | Transform consumes design-knowledge schema. | `Dependencies.csv` row DAG-002-E0773 |
| DEL-13-02 Constraint entity and provenance model | Transform consumes constraint entities. | `Dependencies.csv` row DAG-002-E0774 |
| DEL-13-03 Constraint validation engine | Transform consumes constraint validation. | `Dependencies.csv` row DAG-002-E0775 |
| DEL-04-01 3D frame stiffness kernel | Transform targets the frame kernel model. | `Dependencies.csv` row DAG-002-E0776 |
| DEL-04-03 Linear support and restraint models | Transform needs support/boundary-condition modeling. | `Dependencies.csv` row DAG-002-E0777 |
| DEL-05-01 Primitive load case engine | Transform needs primitive load semantics. | `Dependencies.csv` row DAG-002-E0778 |

### Boundary Conditions

| Condition | Value |
|---|---|
| Engineering standards text availability | Not locally available for this deliverable; do not derive clause-level requirements. |
| PRD v0.2 source availability | Referenced by SOW-066 but not present in `_REFERENCES.md` as a local source; PRD-derived particulars remain `TBD`. |
| Public data policy | No protected standards text, copied formulas, protected tables, protected examples, proprietary commercial data, or code-specific public defaults. |
| Solver acceptance policy | Deterministic mechanics tests are required before release; code-compliance acceptance remains outside software authority. |
| Constraint/provenance upstream | Current DEL-13-02 evidence is `schemas/constraint.schema.json` and `tests/test_constraint_schema.py`; transform consumption beyond copied `constraint_refs` remains broader integration `TBD`. |
| Constraint-validation upstream | Current DEL-13-03 evidence is `core/constraints/validation/engine.py` and `tests/test_constraint_validation.py`; direct runtime validator invocation by the transform remains `TBD`. |
| GUI/runtime/API integration | Current transform and adapter are Python implementation/test evidence only; no GUI, runtime command, public API, or external prover workflow is implemented here. These surfaces remain `TBD`. |
| Persistence/handoff readiness | Current analytical output carries empty `analysis_run_refs`, `comparison_refs`, `handoff_package_refs`, and `external_reference_refs`; persisted package and handoff readiness remain outside this slice and remain `TBD`. |

## Construction

The deliverable has materialized as a provider-neutral Python transform contract, an internal solver-boundary DTO adapter, a canonical invented fixture, and focused tests. These are implementation evidence only; lifecycle acceptance, release thresholds, professional reliance, and code-compliance claims remain outside software authority.

| Construct | Required / expected content | Status |
|---|---|---|
| Transform contract | Inputs, outputs, deterministic behavior, traceability, warning/diagnostic behavior, and unsupported-data handling for physical-to-analytical conversion. | Implemented at `core/model_transform/physical_to_analytical/contract.py`. |
| Analytical model output | Derived `analytical_solver_model` with `source_model_ref`, centerline/frame arrays for nodes/elements/materials/sections/supports/load cases, diagnostics, assumptions, and traceability links. | Implemented for current schema-shaped records; broader physical-record coverage, runtime result envelopes, persistence, and handoff readiness remain `TBD`. |
| Warning/diagnostic records | Deterministic `PTA-*` diagnostics for missing source/model role/coordinate data, missing fields, missing or `TBD` unit metadata, noncanonical dimensions, unsupported component/support/element data, and unresolved references. | Implemented for current contract coverage; final transform-loss taxonomy and release thresholds remain `TBD`. |
| Internal solver-boundary adapter | Deterministic DTO boundary for nodes, straight-pipe connectivity, property bindings, support targets, load-case records/applications, adapter DTO identity/hash/source-chain records, and adapter diagnostics. | Implemented at `core/model_transform/physical_to_analytical/_solver_boundary_adapter.py`; not a public API, GUI/runtime path, external prover path, or final solver acceptance record. |
| Traceability records | Physical-to-analytical `traceability_links` plus adapter DTO `source_chain`, `payload_hash_ref`, `result_trace_anchor`, and `solver_input_trace_anchor` for emitted load applications. | Implemented at object/DTO level; field-level scalar traceability and full runtime result trace-chain production remain `TBD`. |
| Tests | Transform warning/diagnostic tests and internal adapter tests. | Implemented in `tests/test_physical_to_analytical_transform.py` and `tests/test_analytical_solver_boundary_adapter.py` using invented/public-permissive fixture data. |

## References

- `_CONTEXT.md` - deliverable identity, scope, architecture-basis injection, and accepted decomposition reference.
- `_REFERENCES.md` - governing reference list and source boundary.
- `_DEPENDENCIES.md` and `Dependencies.csv` - approved DAG-006 mirror/evidence surface.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` - revision 0.7 package, scope, objective, and deliverable entries.
- `docs/_Registers/Deliverables.csv` - row DEL-13-04.
- `docs/_Registers/ScopeLedger.csv` - row SOW-066.
- `docs/_Registers/ContextBudgetQA.csv` - row DEL-13-04.
- `docs/CONTRACT.md` - invariant catalog, especially OPS-K-IP-1, OPS-K-DATA-2, OPS-K-AUTH-1, OPS-K-MECH-1, OPS-K-UNIT-1, OPS-K-SOLVER-1.
- `docs/SPEC.md` - physical model source-of-truth, unit contract, and missing-data warning classes.
- `docs/TYPES.md` - model, model role, traceability, diagnostic, and mechanics-boundary registry meanings.
- `docs/IP_AND_DATA_BOUNDARY.md` - protected-content handling policy.
- `INIT.md` - project principles and stop rules.
- `core/model_transform/physical_to_analytical/contract.py` - implemented transform contract.
- `core/model_transform/physical_to_analytical/_solver_boundary_adapter.py` - internal solver-boundary DTO adapter.
- `fixtures/domain/invented_physical_source_of_truth_model.json` - canonical invented physical source-of-truth fixture.
- `tests/test_physical_to_analytical_transform.py` - focused transform tests.
- `tests/test_analytical_solver_boundary_adapter.py` - focused adapter tests.
- Current DEL-13-02 and DEL-13-03 four-document kits, `MEMORY.md` files, and 2026-06-07 TASK run records - upstream evidence refresh context.

## D-41 R5 T2B PDU-047 Evidence State

| Surface | Available evidence | Held residual |
|---|---|---|
| 3D centerline/frame target | Deterministic transform and section-property preservation tests; bounded TP-PHYS-015 section-property oracle binding. | Independent validation of broader mechanics suitability. |
| Component field scalar | Paired paths on valid geometry quantity copy-through. | Runtime result-envelope continuation has no accepted producer/home. |
| PDU-036 trace-gap witness | Invented unsupported component is omitted with linked `ASSUMPTION_WARNING`; invalid/omitted scalar path is absent. | Project-owned verification only; broader field/runtime trace validation remains open. |
