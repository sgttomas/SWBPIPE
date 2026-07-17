---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-13-04
package_id: PKG-13
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@2770fda4c63c98ee9f18cffbafd14c9aa59f497f
project_scope_refs: [SOW-066]
package_objective_refs: [OBJ-014]
---

# Scope of Work — DEL-13-04

## Purpose and Objective Traceability

This candidate defines `DEL-13-04` in service of project scope [SOW-066] and package objectives [OBJ-014].

- **OUT-001** — A deterministic physical-to-analytical transformation contract that derives solver-ready analytical models and records traceable warnings, omissions, assumptions, and source-to-target links.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-13-04 Physical-to-analytical transformation contract

> #### Datasheet: DEL-13-04 Physical-to-analytical transformation contract
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-13-04-DECL-002`.
>
> **Generated:** 2026-05-03
> **Status:** Evidence refresh applied 2026-06-07
> **Source posture:** Current implementation evidence is cited where available; unsupported particulars remain marked `TBD` or `ASSUMPTION`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value | Source |
> |---|---|---|
> | Deliverable ID | DEL-13-04 | `_CONTEXT.md` |
> | Name | Physical-to-analytical transformation contract | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row DEL-13-04 |
> | Package ID | PKG-13 | `_CONTEXT.md` |
> | Package name | Physical Design Knowledge and Constraint Engine | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` PKG-13 |
> | Deliverable type | BACKEND_FEATURE_SLICE | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row DEL-13-04 |
> | Scope coverage | SOW-066 | `_CONTEXT.md`; `docs/_Registers/ScopeLedger.csv` row SOW-066 |
> | Objective support | OBJ-014 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` objective mapping |
> | Context envelope | L, WATCH | `_CONTEXT.md`; `docs/_Registers/ContextBudgetQA.csv` row DEL-13-04 |
>

### CLM-004 — Attributes

> ##### Attributes
>
> | Attribute | Current value | Source / note |
> |---|---|---|
> | Primary function | Derive a solver-ready analytical model from the physical model. | SOW-066 in `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` |
> | Determinism requirement | Transformation is deterministic. | SOW-066 in `_CONTEXT.md`; `docs/_Registers/ScopeLedger.csv` row SOW-066 |
> | Warning obligation | Transformation warnings are recorded when physical design data cannot be represented analytically. | SOW-066 in `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEL-13-04 row |
> | Traceability obligation | Transformation traceability is part of OBJ-014 and DEL-13-04 decomposition context. | `execution/_Decomposition/SOFTWARE_DECOMP.md` OBJ-014 and DEL-13-04 |
> | Physical model role | Physical source-of-truth model anchors editable design data. | `docs/SPEC.md` section 3; `docs/TYPES.md` model registry |
> | Analytical model role | Solver-ready idealization / analysis basis derived from the physical model. | `execution/_Decomposition/SOFTWARE_DECOMP.md` glossary row "Analytical Model" |
> | Target mechanics boundary | Primary global analysis model is a 3D centerline/frame model. | `docs/CONTRACT.md` OPS-K-MECH-1; `INIT.md` project principles |
> | Unit handling | Unit-bearing physical values crossing boundaries require explicit unit metadata. | `docs/SPEC.md` section 4; `docs/CONTRACT.md` OPS-K-UNIT-1 |
> | Missing required values | Missing solve-required values are explicit findings, not silent defaults. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md` missing-data warning classes |
> | Protected-data boundary | Public artifacts must not bundle protected standards data, tables, or proprietary values. | `docs/CONTRACT.md` OPS-K-IP-1; `docs/IP_AND_DATA_BOUNDARY.md` |
> | Professional boundary | Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). | `docs/CONTRACT.md` OPS-K-AUTH-1; `INIT.md` |
> | Transform implementation path | `core/model_transform/physical_to_analytical/contract.py` | Current implementation evidence and `MEMORY.md` |
> | Internal solver-boundary adapter path | `core/model_transform/physical_to_analytical/_solver_boundary_adapter.py` | Current implementation evidence and `tests/test_analytical_solver_boundary_adapter.py` |
> | Canonical physical fixture | `fixtures/domain/invented_physical_source_of_truth_model.json` | Current fixture evidence and `tests/test_physical_to_analytical_transform.py` |
> | Focused transform tests | `tests/test_physical_to_analytical_transform.py` | Current transform contract, diagnostic, traceability, and fixture tests |
> | Focused adapter tests | `tests/test_analytical_solver_boundary_adapter.py` | Current internal solver-boundary adapter tests |
> | Current transform result shape | `TransformResult` exposes `analytical_model`, `diagnostics`, `traceability_links`, and `has_blocking_findings`. | `core/model_transform/physical_to_analytical/contract.py` |
> | Current transform settings | Default `TransformSettings` uses `analytical_model_id = ANALYTICAL-DERIVED` and `contract_version = DEL-13-04-0.1`. | `core/model_transform/physical_to_analytical/contract.py` |
> | Exact transform-loss taxonomy | TBD | Implemented `PTA-*` and `ASBA-*` diagnostics are current code/test evidence, not a final release taxonomy. |
> | Exact dependency versions | TBD | `_CONTEXT.md` Architecture Basis Injection "Still TBD". |
>

### CLM-005 — Conditions

> ##### Conditions
>

### CLM-006 — Upstream Inputs

> ###### Upstream Inputs
>
> The local `Dependencies.csv` is an approved DAG-006 mirror/evidence surface. It lists these ACTIVE upstream dependencies for DEL-13-04:
>
> | Target | Relationship summary | Source |
> |---|---|---|
> | DEL-00-01 Architecture decision record baseline | Architecture-basis predecessor. | `Dependencies.csv` row DAG-002-E0660 |
> | DEL-00-02 Repository and module boundary architecture | Architecture-basis predecessor. | `Dependencies.csv` row DAG-002-E0661 |
> | DEL-00-03 Application service command-query-job model | Architecture-basis predecessor. | `Dependencies.csv` row DAG-002-E0662 |
> | DEL-00-04 Persistence and schema versioning architecture | Architecture-basis predecessor. | `Dependencies.csv` row DAG-002-E0663 |
> | DEL-00-06 Diagnostics, warning, and result-envelope contract | Architecture-basis predecessor. | `Dependencies.csv` row DAG-002-E0664 |
> | DEL-00-07 API boundary and adapter contract map | Architecture-basis predecessor. | `Dependencies.csv` row DAG-002-E0665 |
> | DEL-00-08 Layered software test and acceptance strategy | Architecture-basis predecessor. | `Dependencies.csv` row DAG-002-E0666 |
> | DEL-02-01 Canonical domain model schema | Physical-to-analytical transform starts from the canonical physical/domain model. | `Dependencies.csv` row DAG-002-E0772 |
> | DEL-13-01 Design knowledge schema and provenance model | Transform consumes design-knowledge schema. | `Dependencies.csv` row DAG-002-E0773 |
> | DEL-13-02 Constraint entity and provenance model | Transform consumes constraint entities. | `Dependencies.csv` row DAG-002-E0774 |
> | DEL-13-03 Constraint validation engine | Transform consumes constraint validation. | `Dependencies.csv` row DAG-002-E0775 |
> | DEL-04-01 3D frame stiffness kernel | Transform targets the frame kernel model. | `Dependencies.csv` row DAG-002-E0776 |
> | DEL-04-03 Linear support and restraint models | Transform needs support/boundary-condition modeling. | `Dependencies.csv` row DAG-002-E0777 |
> | DEL-05-01 Primitive load case engine | Transform needs primitive load semantics. | `Dependencies.csv` row DAG-002-E0778 |
>

### CLM-007 — Boundary Conditions

> ###### Boundary Conditions
>
> | Condition | Value |
> |---|---|
> | Engineering standards text availability | Not locally available for this deliverable; do not derive clause-level requirements. |
> | PRD v0.2 source availability | Referenced by SOW-066 but not present in `_REFERENCES.md` as a local source; PRD-derived particulars remain `TBD`. |
> | Public data policy | No protected standards text, copied formulas, protected tables, protected examples, proprietary commercial data, or code-specific public defaults. |
> | Solver acceptance policy | Deterministic mechanics tests are required before release; code-compliance acceptance remains outside software authority. |
> | Constraint/provenance upstream | Current DEL-13-02 evidence is `schemas/constraint.schema.json` and `tests/test_constraint_schema.py`; transform consumption beyond copied `constraint_refs` remains broader integration `TBD`. |
> | Constraint-validation upstream | Current DEL-13-03 evidence is `core/constraints/validation/engine.py` and `tests/test_constraint_validation.py`; direct runtime validator invocation by the transform remains `TBD`. |
> | GUI/runtime/API integration | Current transform and adapter are Python implementation/test evidence only; no GUI, runtime command, public API, or external prover workflow is implemented here. These surfaces remain `TBD`. |
> | Persistence/handoff readiness | Current analytical output carries empty `analysis_run_refs`, `comparison_refs`, `handoff_package_refs`, and `external_reference_refs`; persisted package and handoff readiness remain outside this slice and remain `TBD`. |
>

### CLM-008 — Construction

> ##### Construction
>
> The deliverable has materialized as a provider-neutral Python transform contract, an internal solver-boundary DTO adapter, a canonical invented fixture, and focused tests. These are implementation evidence only; lifecycle acceptance, release thresholds, professional reliance, and code-compliance claims remain outside software authority.
>
> | Construct | Required / expected content | Status |
> |---|---|---|
> | Transform contract | Inputs, outputs, deterministic behavior, traceability, warning/diagnostic behavior, and unsupported-data handling for physical-to-analytical conversion. | Implemented at `core/model_transform/physical_to_analytical/contract.py`. |
> | Analytical model output | Derived `analytical_solver_model` with `source_model_ref`, centerline/frame arrays for nodes/elements/materials/sections/supports/load cases, diagnostics, assumptions, and traceability links. | Implemented for current schema-shaped records; broader physical-record coverage, runtime result envelopes, persistence, and handoff readiness remain `TBD`. |
> | Warning/diagnostic records | Deterministic `PTA-*` diagnostics for missing source/model role/coordinate data, missing fields, missing or `TBD` unit metadata, noncanonical dimensions, unsupported component/support/element data, and unresolved references. | Implemented for current contract coverage; final transform-loss taxonomy and release thresholds remain `TBD`. |
> | Internal solver-boundary adapter | Deterministic DTO boundary for nodes, straight-pipe connectivity, property bindings, support targets, load-case records/applications, adapter DTO identity/hash/source-chain records, and adapter diagnostics. | Implemented at `core/model_transform/physical_to_analytical/_solver_boundary_adapter.py`; not a public API, GUI/runtime path, external prover path, or final solver acceptance record. |
> | Traceability records | Physical-to-analytical `traceability_links` plus adapter DTO `source_chain`, `payload_hash_ref`, `result_trace_anchor`, and `solver_input_trace_anchor` for emitted load applications. | Implemented at object/DTO level; field-level scalar traceability and full runtime result trace-chain production remain `TBD`. |
> | Tests | Transform warning/diagnostic tests and internal adapter tests. | Implemented in `tests/test_physical_to_analytical_transform.py` and `tests/test_analytical_solver_boundary_adapter.py` using invented/public-permissive fixture data. |
>

### CLM-009 — References

> ##### References
>
> - `_CONTEXT.md` - deliverable identity, scope, architecture-basis injection, and accepted decomposition reference.
> - `_REFERENCES.md` - governing reference list and source boundary.
> - `_DEPENDENCIES.md` and `Dependencies.csv` - approved DAG-006 mirror/evidence surface.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` - revision 0.7 package, scope, objective, and deliverable entries.
> - `docs/_Registers/Deliverables.csv` - row DEL-13-04.
> - `docs/_Registers/ScopeLedger.csv` - row SOW-066.
> - `docs/_Registers/ContextBudgetQA.csv` - row DEL-13-04.
> - `docs/CONTRACT.md` - invariant catalog, especially OPS-K-IP-1, OPS-K-DATA-2, OPS-K-AUTH-1, OPS-K-MECH-1, OPS-K-UNIT-1, OPS-K-SOLVER-1.
> - `docs/SPEC.md` - physical model source-of-truth, unit contract, and missing-data warning classes.
> - `docs/TYPES.md` - model, model role, traceability, diagnostic, and mechanics-boundary registry meanings.
> - `docs/IP_AND_DATA_BOUNDARY.md` - protected-content handling policy.
> - `INIT.md` - project principles and stop rules.
> - `core/model_transform/physical_to_analytical/contract.py` - implemented transform contract.
> - `core/model_transform/physical_to_analytical/_solver_boundary_adapter.py` - internal solver-boundary DTO adapter.
> - `fixtures/domain/invented_physical_source_of_truth_model.json` - canonical invented physical source-of-truth fixture.
> - `tests/test_physical_to_analytical_transform.py` - focused transform tests.
> - `tests/test_analytical_solver_boundary_adapter.py` - focused adapter tests.
> - Current DEL-13-02 and DEL-13-03 four-document kits, `MEMORY.md` files, and 2026-06-07 TASK run records - upstream evidence refresh context.
>

### CLM-010 — D-41 R5 T2B PDU-047 Evidence State

> ##### D-41 R5 T2B PDU-047 Evidence State
>
> | Surface | Available evidence | Held residual |
> |---|---|---|
> | 3D centerline/frame target | Deterministic transform and section-property preservation tests; bounded TP-PHYS-015 section-property oracle binding. | Independent validation of broader mechanics suitability. |
> | Component field scalar | Paired paths on valid geometry quantity copy-through. | Runtime result-envelope continuation has no accepted producer/home. |
> | PDU-036 trace-gap witness | Invented unsupported component is omitted with linked `ASSUMPTION_WARNING`; invalid/omitted scalar path is absent. | Project-owned verification only; broader field/runtime trace validation remains open. |

## Completion and Reliance Basis — Epistemology

### CLM-011 — Specification: DEL-13-04 Physical-to-analytical transformation contract

> #### Specification: DEL-13-04 Physical-to-analytical transformation contract
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-012 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-13-04-DECL-001`.
>
> **Generated:** 2026-05-03
> **Status:** Evidence refresh applied 2026-06-07
> **Source posture:** Requirements below are limited to accessible local sources; unresolved particulars remain `TBD`.
>

### CLM-013 — Scope

> ##### Scope
>
> DEL-13-04 specifies the backend contract for transforming the schema-backed physical model into a solver-ready analytical model and recording traceable transformation warnings when physical design data cannot be represented analytically.
>

### CLM-014 — In Scope

> ###### In Scope
>
> - A deterministic physical-to-analytical transformation contract for SOW-066.
> - Warning / diagnostic behavior for omitted, unsupported, incomplete, or non-representable physical design data.
> - Traceability from source physical model records to derived analytical model records, warnings, omissions, or assumptions.
> - Contract-level tests for transformation warnings.
> - Integration posture with architecture-basis constraints: Rust core/application services, schema-first command/query/job result envelopes, JSON Schema 2020-12 contracts, diagnostics/result envelopes, and deterministic tests where applicable, per `_CONTEXT.md`.
>

### CLM-015 — Out of Scope

> ###### Out of Scope
>
> - Protected standards data, code-specific defaults, protected tables, copied formulas, owner standards, proprietary catalog values, or private project data.
> - Final engineering acceptance. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
> - Owner standards or final acceptance logic for physical design constraints, per PKG-13 package exclusions.
> - Final transform-loss taxonomy, release thresholds, dependency versions, external prover behavior, GUI/runtime/API integration, persisted/handoff readiness, broader physical-record coverage, and human/professional acceptance.
>

### CLM-016 — Implementation Evidence

> ##### Implementation Evidence
>
> Current implementation evidence exists for the DEL-13-04 transform and internal solver-boundary adapter:
>
> - `core/model_transform/physical_to_analytical/contract.py` implements `transform_physical_to_analytical(...)`, `TransformSettings`, `TransformResult`, deterministic ordering, `analytical_solver_model` output, `PTA-*` diagnostics, and physical-to-analytical traceability links.
> - `core/model_transform/physical_to_analytical/_solver_boundary_adapter.py` implements the internal `adapt_analytical_solver_model(...)` DTO boundary for strict `analytical_solver_model` mappings. It emits deterministic node DTOs, straight-pipe connectivity, property bindings, support targets, load-case records/applications, adapter DTO identity/hash/source-chain records, `load_case_diagnostics`, and adapter diagnostics.
> - `fixtures/domain/invented_physical_source_of_truth_model.json` is the canonical invented physical source-of-truth fixture for current DEL-13-04 tests. It contains public-permissive invented nodes, straight-pipe element, governed `y_reference`, material/section quantities, support, strict load records, visible assumption/diagnostic evidence, and traceability.
> - `tests/test_physical_to_analytical_transform.py` covers deterministic transform behavior, source model preservation, canonical fixture/schema validation, traceability, unit metadata failures, unsupported physical-record/component behavior, strict load metadata, noncanonical quantity dimensions, and forbidden authority-claim text scanning.
> - `tests/test_analytical_solver_boundary_adapter.py` covers deterministic solver-boundary DTO emission, missing-property no-default diagnostics, noncanonical load semantic rejection, invalid orientation diagnostics, load span/runtime diagnostics, adapter DTO hash/source-chain evidence, and analytical-model role enforcement.
>
> This evidence does not implement a GUI command path, runtime service envelope, public API, persistence/handoff package readiness, external prover behavior, full release threshold policy, or human/professional acceptance.
>

### CLM-017 — Requirements

> ##### Requirements
>
> | ID | Requirement | Verification | Source |
> |---|---|---|---|
> | DEL-13-04-REQ-001 | The contract shall transform the physical model into a solver-ready analytical model deterministically. | `tests/test_physical_to_analytical_transform.py` repeats transform over the canonical fixture and deep-copied inputs and expects identical output. | `_CONTEXT.md` SOW-066; `docs/_Registers/ScopeLedger.csv` SOW-066; `core/model_transform/physical_to_analytical/contract.py` |
> | DEL-13-04-REQ-002 | The contract shall record transformation warnings or diagnostics when physical design data cannot be represented analytically. | Transform tests cover missing units, unsupported element/component data, unresolved references, noncanonical dimensions, and unresolved load dimensions. Adapter tests cover unsupported/noncanonical load semantics, invalid orientation, missing properties, unresolved targets, and nonfinite quantities. | `_CONTEXT.md` SOW-066; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEL-13-04; `tests/test_physical_to_analytical_transform.py`; `tests/test_analytical_solver_boundary_adapter.py` |
> | DEL-13-04-REQ-003 | The contract shall preserve the physical model as the editable source-of-truth and produce the analytical model as a derived solver-oriented view. | Schema/contract review confirms physical model role is not overwritten or reclassified by transform output. | `docs/SPEC.md` section 3; `docs/TYPES.md` `Model` and `ModelRole` rows |
> | DEL-13-04-REQ-004 | The contract shall preserve traceability from physical source records to analytical records, omissions, assumptions, warnings, or other transform outcomes. | Transform tests assert physical-to-analytical links for nodes, elements, materials, sections, supports, load cases, components, and diagnostics; adapter tests assert DTO `source_chain` and hash anchors for emitted load applications. Field-level scalar traceability remains `TBD`. | `execution/_Decomposition/SOFTWARE_DECOMP.md` OBJ-014; `docs/TYPES.md` `TraceabilityLink` row; `core/model_transform/physical_to_analytical/contract.py`; `_solver_boundary_adapter.py` |
> | DEL-13-04-REQ-005 | Unit-bearing physical values crossing the transform boundary shall carry explicit unit metadata unless explicitly dimensionless. | Transform tests reject missing, empty, `TBD`, and noncanonical dimensions; adapter tests require finite `Quantity` records with expected dimensions and do not convert or infer units. | `docs/SPEC.md` section 4; `docs/CONTRACT.md` OPS-K-UNIT-1 |
> | DEL-13-04-REQ-006 | Missing solve-required values shall be represented as explicit findings and shall not be silently defaulted by the transformation contract. | Negative transform and adapter tests expect deterministic `PTA-*` or `ASBA-*` diagnostics instead of defaults. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md` missing-data warning classes |
> | DEL-13-04-REQ-007 | The analytical output shall target the project primary global mechanics boundary: a 3D centerline/frame model. | Contract review confirms no routine shell/solid FEA target is introduced for this deliverable. | `docs/CONTRACT.md` OPS-K-MECH-1; `INIT.md` principles |
> | DEL-13-04-REQ-008 | The contract shall not bundle protected standards content, code-specific default values, protected tables, copied formulas, proprietary catalog values, or owner standards. | Protected-content review / contribution gate; public fixtures use invented or permissive data only. | `docs/CONTRACT.md` OPS-K-IP-1 and OPS-K-DATA-1; `docs/IP_AND_DATA_BOUNDARY.md` |
> | DEL-13-04-REQ-009 | The contract and its warnings shall not claim certification, sealing, approval, authentication, engineering acceptance, or code compliance (PRD §21.2). | Text/schema review for forbidden authority claims; report/result wording remains diagnostic only. | `docs/CONTRACT.md` OPS-K-AUTH-1; `INIT.md` |
> | DEL-13-04-REQ-010 | Solver-facing changes resulting from this contract shall have deterministic verification tests before release. | Current focused tests are executable with `python3 tests/test_physical_to_analytical_transform.py`, `python3 tests/test_analytical_solver_boundary_adapter.py`, and joint `pytest`; release thresholds remain `TBD`. | `docs/CONTRACT.md` OPS-K-SOLVER-1; `Dependencies.csv` row DAG-002-E0666 |
> | DEL-13-04-REQ-011 | The contract shall treat upstream dependency surfaces as prerequisite context and shall not reinterpret the local dependency mirror as independent dispatch authority. | Dependency artifact review confirms current local dependency rows remain evidence under DAG-006 coordination; this refresh does not edit dependency files. | `_DEPENDENCIES.md` Authority Boundary; `Dependencies.csv`; `_COORDINATION.md` |
> | DEL-13-04-REQ-012 | The internal solver-boundary adapter shall accept only strict `analytical_solver_model` mappings and shall not become a public API, GUI/runtime path, external prover path, or final solver acceptance claim. | Adapter tests enforce analytical-model role, strict load union handling, adapter DTO evidence, and diagnostics for unsupported runtime conditions. | `core/model_transform/physical_to_analytical/_solver_boundary_adapter.py`; `tests/test_analytical_solver_boundary_adapter.py`; `MEMORY.md` |
>

### CLM-018 — Standards

> ##### Standards
>
> No external engineering standard text is locally available or authorized as source material for this deliverable. The governing project standards for this draft are:
>
> | Standard / policy surface | Applicability |
> |---|---|
> | `docs/CONTRACT.md` | Invariants for data boundary, missing values, professional authority, mechanics boundary, units, and solver tests. |
> | `docs/SPEC.md` | Schema-first model role, unit metadata, diagnostics/warnings, and warning class context. |
> | `docs/TYPES.md` | Vocabulary for model roles, traceability, diagnostics, frame kernel, supports, and load semantics. |
> | `docs/IP_AND_DATA_BOUNDARY.md` | Protected-data and private-data handling. |
> | `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 | Accepted decomposition and SOW/OBJ mapping. |
> | `core/model_transform/physical_to_analytical/contract.py` | Current transform implementation. |
> | `core/model_transform/physical_to_analytical/_solver_boundary_adapter.py` | Current internal adapter implementation. |
> | `tests/test_physical_to_analytical_transform.py` and `tests/test_analytical_solver_boundary_adapter.py` | Current focused verification evidence. |
>
> Referenced PRD source material (`PRD v0.2 Section8.3`, `FR-MOD-007`) is not locally available in this deliverable's references. Any requirement depending on the PRD beyond the SOW-066 text remains `TBD`.
>

### CLM-019 — Verification

> ##### Verification
>
> | Verification item | Required check | Current status |
> |---|---|---|
> | Deterministic transform | Same input and configuration produce equivalent analytical output and equivalent warnings/diagnostics. | Implemented in `tests/test_physical_to_analytical_transform.py` using `fixtures/domain/invented_physical_source_of_truth_model.json`. |
> | Warning/diagnostic behavior | Non-representable physical data produces deterministic transformation diagnostics. | Implemented for current `PTA-*` and `ASBA-*` coverage; final release taxonomy remains `TBD`. |
> | Traceability coverage | Physical-to-analytical mappings, omissions, warnings, assumptions, adapter DTO hashes, and source chains carry traceability links or explicit gaps. | Implemented at object/DTO level; field-level scalar traceability and full runtime result trace-chain production remain `TBD`. |
> | Unit metadata | Unit-bearing values are not accepted silently without explicit units. | Implemented in transform and adapter tests; no unit conversion is performed. |
> | No silent defaults | Missing solve-required physical data yields findings rather than inferred defaults. | Implemented in transform and adapter tests. |
> | Protected-content boundary | Public contract/tests do not include protected standards text, values, tables, or proprietary data. | Required by OPS-K-IP-1. |
> | Professional boundary | Outputs remain diagnostics / transform artifacts, not compliance or professional-approval claims. | Required by OPS-K-AUTH-1. |
> | DAG mirror preservation | All current local dependency rows remain unchanged by this evidence refresh. | Required by user instruction and `_DEPENDENCIES.md`; dependency files are outside this write scope. |
> | Internal adapter DTO evidence | Strict analytical solver-boundary mappings produce deterministic DTOs or diagnostics. | Implemented in `tests/test_analytical_solver_boundary_adapter.py`. |
>

### CLM-020 — Documentation

> ##### Documentation
>
> Required local artifacts for this setup pass:
>
> - `Datasheet.md`
> - `Specification.md`
> - `Guidance.md`
> - `Procedure.md`
> - `_SEMANTIC.md`
> - `_SEMANTIC_LENSING.md`
>
> Anticipated implementation artifacts from `_CONTEXT.md` and `docs/_Registers/Deliverables.csv`:
>
> - physical-to-analytical transform contract
> - transform warning tests
>
> Current implementation artifact paths and focused test fixture names are no longer `TBD`; they are listed above. Remaining `TBD` items are transform-loss taxonomy, release thresholds, external prover behavior, GUI/runtime/API integration, persisted/handoff readiness, broader physical-record coverage, human acceptance, and professional/code-compliance boundaries.
>

### CLM-021 — D-41 R5 T2B PDU-047 Evidence Disposition (2026-07-12)

> ##### D-41 R5 T2B PDU-047 Evidence Disposition (2026-07-12)
>
> The TP-PHYS-015 production section-property binding is relevant downstream evidence, but it does not independently validate REQ-007's full 3D centerline/frame target suitability. That broader mechanics-validation basis remains held. This tranche changes neither the transform contract nor its output.
>

### CLM-022 — D-41 R5 T2C PDU-023 E6 Evidence (2026-07-12)

> ##### D-41 R5 T2C PDU-023 E6 Evidence (2026-07-12)
>
> REQ-004 now has a minimal field-scalar link for valid copied component geometry quantities. Each link pairs source and target scalar paths on the existing object references. Incomplete quantity metadata is diagnosed and emits no scalar link. Runtime result-envelope continuation remains held.
>

### CLM-023 — D-41 R5 T4 PDU-036 fixture evidence

> ##### D-41 R5 T4 PDU-036 fixture evidence
>
> `fixtures/domain/invented_physical_to_analytical_trace_gap.json` provides one verification-only unsupported-component witness. The transform omits the component, emits `PTA-COMPONENT-TYPE-UNSUPPORTED` as `ASSUMPTION_WARNING`, attaches that warning to the component-to-diagnostic trace link, and emits no scalar link for the omitted quantity. This bounded witness does not promote independent validation or close broader trace coverage.

- **AC-001** — All authoritative source content is preserved and mapped, and the source-defined deterministic transform, warning, traceability, unit, missing-data, protected-content, and professional-boundary requirements remain intact while broader validation and runtime-result continuation remain explicitly deferred.

## Production and Verification Method — Praxeology

### CLM-024 — Procedure: DEL-13-04 Physical-to-analytical transformation contract

> #### Procedure: DEL-13-04 Physical-to-analytical transformation contract
>
> **Generated:** 2026-05-03
> **Status:** Evidence refresh applied 2026-06-07
> **Source posture:** Operational steps describe how to produce/use the contract without inventing unsupported implementation details.
>

### CLM-025 — Purpose

> ##### Purpose
>
> This procedure defines the conservative workflow for checking and extending the DEL-13-04 physical-to-analytical transformation contract. The implemented workflow keeps the physical model as source of truth, derives a solver-ready analytical representation, records transformation diagnostics, and preserves traceability without introducing protected data, hidden defaults, public API expansion, GUI/runtime behavior, external prover claims, or professional compliance claims.
>

### CLM-026 — Prerequisites

> ##### Prerequisites
>
> 1. Confirm current deliverable context:
>    - `_CONTEXT.md` identifies DEL-13-04, PKG-13, BACKEND_FEATURE_SLICE, SOW-066, OBJ-014.
>    - `_STATUS.md` is in a state that permits setup drafting before overwriting local production documents.
> 2. Confirm governing references:
>    - `_REFERENCES.md`
>    - `execution/_Decomposition/SOFTWARE_DECOMP.md`
>    - `docs/_Registers/Deliverables.csv`
>    - `docs/_Registers/ScopeLedger.csv`
>    - `docs/_Registers/ContextBudgetQA.csv`
>    - `docs/CONTRACT.md`
>    - `docs/SPEC.md`
>    - `docs/TYPES.md`
>    - `docs/IP_AND_DATA_BOUNDARY.md`
>    - `INIT.md`
> 3. Treat `Dependencies.csv` as the approved DAG-006 mirror/evidence surface. Preserve approved rows as ACTIVE; do not retire, delete, or reclassify them during this setup workflow.
> 4. Use only source-cleared or invented/public-permissive data in examples and tests. Suspected protected content must be quarantined and escalated under project policy.
> 5. Inspect current implementation and test evidence:
>    - `core/model_transform/physical_to_analytical/contract.py`
>    - `core/model_transform/physical_to_analytical/_solver_boundary_adapter.py`
>    - `fixtures/domain/invented_physical_source_of_truth_model.json`
>    - `tests/test_physical_to_analytical_transform.py`
>    - `tests/test_analytical_solver_boundary_adapter.py`
> 6. Treat current DEL-13-02 and DEL-13-03 four-document kits, `MEMORY.md` files, and TASK run records `TASK_RUN_2026-06-07_1127.md` / `TASK_RUN_2026-06-07_1133.md` as read-only upstream evidence for this refresh.
>

### CLM-027 — Steps

> ##### Steps
>

### CLM-028 — 1. Establish Transform Boundary

> ###### 1. Establish Transform Boundary
>
> 1. Identify the physical model as the source-of-truth input.
> 2. Identify the analytical model as a derived solver-ready output with `model_role = analytical_solver_model` and `source_model_ref` pointing back to the physical model.
> 3. Confirm the default target mechanics boundary is the 3D centerline/frame model.
> 4. Verify supported current physical-record coverage: nodes, straight-pipe elements, supported component metadata when referenced by valid elements, materials, sections, supports, load cases, combinations, diagnostics, unresolved assumptions, and traceability links.
> 5. Record broader physical-record coverage as `TBD` unless current implementation/tests explicitly define its handling.
>

### CLM-029 — 2. Define Contract Inputs

> ###### 2. Define Contract Inputs
>
> 1. Declare required input surfaces from source-grounded context:
>    - canonical model/domain schema;
>    - design knowledge and provenance;
>    - constraint entities and constraint validation;
>    - support/restraint semantics;
>    - primitive load semantics;
>    - diagnostics/result-envelope conventions.
> 2. For each unit-bearing input, require explicit unit metadata unless the field is explicitly dimensionless.
> 3. For each input whose source, provenance, or units are missing, define a diagnostic/warning path instead of a silent default.
> 4. For current implementation evidence, verify schema-shaped input/output fields used or produced by `contract.py`, including `id`, `model_role`, `coordinate_system`, `nodes`, `elements`, `components`, `materials`, `sections`, `supports`, `load_cases`, `combinations`, `design_knowledge_refs`, `constraint_refs`, `diagnostics`, `unresolved_assumptions`, and `traceability_links`.
> 5. Leave unimplemented schema references, runtime service envelopes, external prover behavior, and field-level scalar traceability `TBD` unless fixed by accessible implementation evidence.
>

### CLM-030 — 3. Define Contract Outputs

> ###### 3. Define Contract Outputs
>
> 1. Specify the derived analytical model output at the contract level.
> 2. Specify warning/diagnostic output for non-representable physical data, missing solve-required data, weak provenance, unresolved assumptions, or protected/private-data risk where applicable.
> 3. Specify traceability links from physical inputs to:
>    - analytical output records;
>    - warnings/diagnostics;
>    - omissions;
>    - assumptions;
>    - unresolved `TBD` items.
> 4. For the internal adapter, specify DTO outputs only as internal solver-boundary evidence: nodes, straight-pipe connectivity, property bindings, support targets, load-case records/applications, adapter DTO identity/hash/source-chain records, load-case diagnostics, and diagnostics.
> 5. Do not encode compliance, approval, certification, external-prover success, release readiness, or professional reliance status in transform or adapter outputs (PRD §21.2).
>

### CLM-031 — 4. Define Determinism Rules

> ###### 4. Define Determinism Rules
>
> 1. Define deterministic behavior in terms of equivalent input model, units, transform contract version, and configuration.
> 2. Require repeatable analytical output and repeatable warning/diagnostic output for the same basis.
> 3. Verify current stable record ordering, stable diagnostic ordering, stable traceability ordering, and adapter DTO hash evidence (`sha256` over sorted JSON payloads with `JCS` vocabulary) where implemented.
> 4. Record release-grade canonicalization and broader audit-manifest policy as `TBD` unless fixed by a later governed decision.
>

### CLM-032 — 5. Define Warning Tests

> ###### 5. Define Warning Tests
>
> 1. Create tests for transformation warnings required by SOW-066.
> 2. Include negative cases for at least:
>    - missing solve-required physical input;
>    - physical data that lacks a solver-ready analytical representation;
>    - missing or ambiguous unit metadata on a unit-bearing value.
> 3. Use invented or permissive fixtures only.
> 4. Verify warnings remain diagnostic and do not become compliance or professional acceptance claims.
> 5. For the current implementation, run:
>    - `python3 tests/test_physical_to_analytical_transform.py`
>    - `python3 tests/test_analytical_solver_boundary_adapter.py`
>    - `python3 -m pytest -q tests/test_physical_to_analytical_transform.py tests/test_analytical_solver_boundary_adapter.py`
>

### CLM-033 — 6. Review Data Boundary

> ###### 6. Review Data Boundary
>
> 1. Scan the contract and tests for protected standards text, protected tables, code-specific defaults, copied formulas, proprietary catalog values, owner standards, or private project data.
> 2. Mark uncertain content `TBD` or `protected_suspected` according to policy.
> 3. Escalate suspected protected content; do not normalize it into public examples.
>

### CLM-034 — Verification

> ##### Verification
>
> | Check | Expected evidence |
> |---|---|
> | Four-doc consistency | `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` use the same DEL ID, package, scope, objective, and terminology. |
> | Requirement coverage | Specification requirements trace to SOW-066, OBJ-014, project invariants, or local DAG mirror rows. |
> | Determinism | Tests or planned tests cover repeated transform behavior. |
> | Warning behavior | Tests or planned tests cover warnings for non-representable physical data. |
> | Traceability | Contract records physical-to-analytical links or explicit unresolved gaps. |
> | Unit safety | Missing/ambiguous unit metadata produces findings, not defaults. |
> | Data boundary | No protected/private/proprietary engineering content is introduced. |
> | Professional boundary | Outputs avoid certification, approval, compliance, or professional reliance claims (PRD §21.2). |
> | Internal adapter boundary | Adapter accepts only strict `analytical_solver_model` mappings and reports DTO/load/orientation/property failures as diagnostics. |
> | Dependency preservation | Existing local dependency rows remain unchanged during evidence refresh; historical row IDs remain audit evidence under the current DAG-006 coordination basis. |
>

### CLM-035 — Records

> ##### Records
>
> Maintain the following deliverable-local records:
>
> - `Datasheet.md`
> - `Specification.md`
> - `Guidance.md`
> - `Procedure.md`
> - `_SEMANTIC.md`
> - `_SEMANTIC_LENSING.md`
> - `_STATUS.md`
> - `Dependencies.csv`
> - `_DEPENDENCIES.md`
> - `_run_records/**`
>
> Check the following repo-level implementation and test evidence when verifying this deliverable:
>
> - `core/model_transform/physical_to_analytical/contract.py`
> - `core/model_transform/physical_to_analytical/_solver_boundary_adapter.py`
> - `fixtures/domain/invented_physical_source_of_truth_model.json`
> - `tests/test_physical_to_analytical_transform.py`
> - `tests/test_analytical_solver_boundary_adapter.py`
>
> Implementation records are no longer `TBD` for the current transform, internal adapter, canonical fixture, and focused tests listed above. Remaining `TBD` records include final transform-loss taxonomy, release thresholds, external prover behavior, GUI/runtime/API integration, persisted/handoff readiness, broader physical-record coverage, human acceptance, and professional/code-compliance boundaries.
>

### CLM-036 — D-41 R5 T2B PDU-047 Check

> ##### D-41 R5 T2B PDU-047 Check
>
> Treat section-property numeric evidence as evidence for the carried section-property values only. Do not use it to infer broader 3D frame suitability, shell/solid equivalence, release readiness, or professional acceptance. Record that missing independent basis as a held residual.
>
> For PDU-023, require paired source/target scalar paths only after quantity metadata passes current validation. Do not synthesize a result-envelope continuation.
>
> For PDU-036, load the invented trace-gap fixture, verify the unsupported component is absent from analytical output, verify its `ASSUMPTION_WARNING` is attached to the diagnostic trace link, and verify no field-scalar link is emitted for the omitted record. Record the result as verification, not independent validation.

- **VER-001** — Validate the candidate schema and claim map; confirm parity and source/status hashes; run the focused transform, solver-boundary adapter, and dependency checks; and independently review source grounding and authority boundaries.

## Governing Values and Decisions — Axiology

### CLM-037 — Guidance: DEL-13-04 Physical-to-analytical transformation contract

> #### Guidance: DEL-13-04 Physical-to-analytical transformation contract
>
> **Generated:** 2026-05-03
> **Status:** Evidence refresh applied 2026-06-07
> **Source posture:** Directional guidance is source-grounded where possible; unsupported implementation advice is marked `TBD` or omitted.
>

### CLM-038 — Purpose

> ##### Purpose
>
> DEL-13-04 is the bridge from design authoring to solver execution. Its implemented purpose is to derive a deterministic `analytical_solver_model` from a schema-shaped physical source-of-truth model while preserving diagnostics, omissions, assumptions, and traceability when information cannot cross that boundary exactly.
>
> Sources: `_CONTEXT.md` Scope Detail and Context Envelope; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEL-13-04 and OBJ-014; `core/model_transform/physical_to_analytical/contract.py`; `tests/test_physical_to_analytical_transform.py`.
>

### CLM-039 — Principles

> ##### Principles
>
> | Principle | Guidance | Source |
> |---|---|---|
> | Preserve source-of-truth role | Treat the physical model as the editable source-of-truth and the analytical model as a derived view. Do not overwrite physical design context during transformation. | `docs/SPEC.md` section 3; `docs/TYPES.md` `ModelRole` |
> | Be deterministic | The same physical input and contract configuration should yield the same analytical output and warning set. | SOW-066; `docs/CONTRACT.md` OPS-K-SOLVER-1 |
> | Warn rather than hide loss | When physical data cannot be represented analytically, expose the loss as a warning/diagnostic with traceability. | SOW-066; `docs/TYPES.md` `Diagnostic` |
> | Keep units explicit | Unit-bearing values crossing the boundary need explicit unit metadata; missing or ambiguous units are findings. | `docs/SPEC.md` section 4; `docs/CONTRACT.md` OPS-K-UNIT-1 |
> | Avoid silent engineering defaults | Missing solve-required values must not be silently supplied. | `docs/CONTRACT.md` OPS-K-DATA-2 |
> | Stay code-neutral | Do not introduce protected code data, proprietary values, code-specific public defaults, SIF/flexibility tables, or compliance claims. | `docs/CONTRACT.md`; `docs/IP_AND_DATA_BOUNDARY.md` |
> | Target global frame mechanics | The default analytical target is the project's 3D centerline/frame mechanics boundary, not routine shell/solid FEA. | `docs/CONTRACT.md` OPS-K-MECH-1; `INIT.md` |
>

### CLM-040 — Considerations

> ##### Considerations
>

### CLM-041 — Transform-Loss Classes

> ###### Transform-Loss Classes
>
> The sources require warnings for physical design data that cannot be represented analytically, and current implementation evidence supplies concrete diagnostic behavior. The transform emits deterministic `PTA-*` diagnostics for current coverage, including missing source/model role/coordinate data, missing required fields, missing or `TBD` unit metadata, noncanonical quantity dimensions, unsupported components/supports/elements, and unresolved references. The internal adapter emits deterministic `ASBA-*` diagnostics for strict solver-boundary DTO coverage, including missing properties, invalid `y_reference`, unsupported/noncanonical load semantics, unresolved targets, wrong dimensions, nonfinite quantities, unordered uniform-load spans, and wrong model role.
>
> Guidance: treat those `PTA-*` and `ASBA-*` codes as current implementation/test evidence, not as the final release transform-loss taxonomy. The final taxonomy, release thresholds, and user-facing severity policy remain `TBD` until a governed architecture or release-readiness decision accepts them.
>

### CLM-042 — Upstream Contract Surfaces

> ###### Upstream Contract Surfaces
>
> `Dependencies.csv` identifies prerequisite surfaces for canonical domain model schema, design knowledge, constraints, constraint validation, frame stiffness, supports/restraints, primitive loads, diagnostics/result envelopes, API boundaries, persistence/schema versioning, and layered tests. Those rows are evidence of coordination dependencies, not permission to copy or reinterpret sibling deliverable content.
>
> Current upstream refresh evidence confirms DEL-13-02 has an implemented constraint entity/provenance schema at `schemas/constraint.schema.json`, and DEL-13-03 has an implemented validation engine at `core/constraints/validation/engine.py`. DEL-13-04 currently carries `design_knowledge_refs` and `constraint_refs` through the derived analytical model, but it does not invoke the validation engine, resolve owner criteria, or create a GUI/runtime/API integration path.
>
> Guidance: implementation work should consume approved upstream contracts through explicit sealed scope and should not use this evidence refresh to reclassify dependencies, expand schema vocabulary, or infer owner/code criteria.
>

### CLM-043 — Diagnostic Placement

> ###### Diagnostic Placement
>
> The project vocabulary distinguishes diagnostics and warning classes from professional compliance outcomes. Current transform diagnostics use classes such as `SOLVE_BLOCKING`, `ASSUMPTION_WARNING`, and `NONLINEAR_WARNING`; adapter diagnostics classify blocking findings as `SOLVE_BLOCKING` and other adapter findings as diagnostic evidence only.
>
> Guidance: transformation and adapter findings should be deterministic, source-linked, and reviewable. They should not state that the model is compliant, professionally accepted, release-ready, or externally proven.
>

### CLM-044 — Data Boundary

> ###### Data Boundary
>
> Transformation may touch component, material, section, support, load, constraint, and design-knowledge references. Public artifacts must not supply protected or proprietary values for those surfaces. Unknown or suspected protected sources require review/quarantine rather than normalization into public defaults.
>
> Guidance: tests should use invented or permitted fixtures and should include provenance where data matters.
>

### CLM-045 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Conservative direction |
> |---|---|
> | Rich physical data vs. solver-ready idealization | Preserve physical richness through traceability and warnings rather than forcing all data into solver elements. |
> | Convenience defaults vs. explicit findings | Prefer explicit `TBD`, warning, or diagnostic records over hidden defaults. |
> | Contract specificity vs. unsupported invention | Specify behavior proven by SOW, decomposition, project invariants, current implementation, and focused tests; leave release taxonomy, external prover behavior, GUI/runtime/API integration, persistence/handoff readiness, and human/professional acceptance `TBD` where evidence does not fix them. |
> | Analytical target breadth vs. global mechanics scope | Keep routine transformation aimed at 3D centerline/frame analysis. Treat shell/solid FEA as a separate handoff path. |
>

### CLM-046 — Examples

> ##### Examples
>
> Executable invented examples are available in `fixtures/domain/invented_physical_source_of_truth_model.json`, `tests/test_physical_to_analytical_transform.py`, and `tests/test_analytical_solver_boundary_adapter.py`. They are public-permissive test evidence, not owner/project examples or professional design examples. Publication-grade examples, owner/project examples, and any examples containing code/owner criteria remain `TBD` pending source clearance and human acceptance.
>

### CLM-047 — Source Access Gaps

> ##### Source Access Gaps
>
> | Gap | Impact | Handling |
> |---|---|---|
> | PRD v0.2 Section8.3 / FR-MOD-007 not locally available through `_REFERENCES.md` | Cannot derive PRD-specific transform clauses beyond SOW-066 wording. | Keep PRD-specific particulars `TBD`. |
> | Broader upstream integration beyond current DEL-13-02/DEL-13-03 refresh evidence | Cannot claim full runtime constraint-validation consumption, GUI presentation, public API behavior, or persisted handoff readiness. | Keep those integration surfaces `TBD` until bounded implementation evidence exists. |
> | OI-012 unresolved architecture detail for loss classes | Transform-loss taxonomy cannot be finalized here. | Record `TBD`; require later architecture/detail task. |
>

### CLM-048 — D-41 R5 T2B PDU-047 Boundary

> ##### D-41 R5 T2B PDU-047 Boundary
>
> A correct circular-section calculation is not a validation basis for the overall physical-to-analytical 3D mechanics boundary. Keep the existing deterministic verification and the new bounded oracle binding distinct from the unresolved suitability judgment.
>
> PDU-023 field paths prove scalar identity through the current copy boundary only; they do not claim downstream solver or runtime result-envelope production.
>
> PDU-036 uses an invented negative fixture to make one omission/warning/assumption relationship executable. Treat its missing scalar trace as an explicit gap caused by omission, not as evidence that every scalar or runtime result path is traced.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-066 OBJ-014 | CLM-011 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
