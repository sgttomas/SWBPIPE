---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-03-06
package_id: PKG-03
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@69ac259a7113d5a838fb22aa2e84df0e0f109713
project_scope_refs: [SOW-010]
package_objective_refs: [OBJ-004]
---

# Scope of Work — DEL-03-06

## Purpose and Objective Traceability

This Scope of Work defines `DEL-03-06` in service of project scope [SOW-010] and package objectives [OBJ-004].

- **OUT-001** — An expansion-joint component-model contract covering supplied stiffnesses, effective area, movement limits, hardware fields, units, provenance, completeness, and diagnostics is produced for the declared scope and objective.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-03-06 Expansion joint component model

> #### Datasheet: DEL-03-06 Expansion joint component model
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-002 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-03-06-DECL-002`.
>

### CLM-003 — Identification

> ##### Identification
>
> | Field | Value |
> |---|---|
> | Deliverable ID | DEL-03-06 |
> | Package ID | PKG-03 |
> | Package | Piping Components, Materials, and Library Data Model |
> | Type | BACKEND_FEATURE_SLICE |
> | Scope Item | SOW-010 |
> | Objective | OBJ-004 |
> | Evidence Status | Implemented schema, invented fixture, and test evidence for expansion-joint component slots; no public engineering values or lifecycle promotion |
>

### CLM-004 — Attributes

> ##### Attributes
>
> The implemented component-library evidence represents expansion joints as supplied-data component records. The current public repository evidence defines schema slots, an invented fixture record, completeness findings, and diagnostics; it does not supply engineering values.
>
> | Attribute Category | Required Treatment | Source |
> |---|---|---|
> | Stiffness values | Implemented as `linear_stiffness` and `rotational_stiffness` component field kinds and accepted quantity dimensions. Public fixture values remain missing and private/user/manufacturer supplied. Exact per-axis DOF/tensor mapping for downstream solver consumption remains `TBD`. | `schemas/component.schema.yaml`; `fixtures/component/invented_component_library_valid.json`; `tests/test_component_section_schema.py`; execution/_Decomposition/SOFTWARE_DECOMP.md#SOW-010; docs/CONTRACT.md#OPS-K-DATA-1 |
> | Effective area | Implemented as an `effective_area` field kind with provenance and missing-value handling in the invented fixture. Pressure/thrust usage semantics remain downstream solver scope. | `schemas/component.schema.yaml`; `fixtures/component/invented_component_library_valid.json`; `tests/test_component_section_schema.py`; execution/_Decomposition/SOFTWARE_DECOMP.md#SOW-010 |
> | Movement limits | Implemented as a `movement_limit` field kind with explicit missing-value diagnostics. Movement-limit class taxonomy and dimensional validation categories remain `TBD`. | `schemas/component.schema.yaml`; `fixtures/component/invented_component_library_valid.json`; `tests/test_component_section_schema.py`; execution/_Decomposition/SOFTWARE_DECOMP.md#SOW-010; docs/CONTRACT.md#OPS-K-DATA-2 |
> | Hardware data | Implemented as `hardware_flag`/`hardware_reference` schema slots and fixture contract evidence, with `hardware_reference` present on the invented expansion-joint record. Hardware flag/enumeration taxonomy remains `TBD`. | `schemas/component.schema.yaml`; `fixtures/component/invented_component_library_valid.json`; `tests/test_component_section_schema.py`; execution/_Decomposition/SOFTWARE_DECOMP.md#SOW-010 |
> | Provenance | Implemented through source, license/redistribution status, contributor certification, and review status fields on schema-slot fixture records. Public source-catalog policy, fixture-value policy, and human review disposition remain open. | `schemas/component.schema.yaml`; `fixtures/component/invented_component_library_valid.json`; docs/CONTRACT.md#OPS-K-IP-2; docs/CONTRACT.md#OPS-K-DATA-3 |
>

### CLM-005 — Preserved TBDs and Gates

> ###### Preserved TBDs and Gates
>
> - `TBD`: exact per-axis stiffness field shape and solver degree-of-freedom mapping beyond the implemented `linear_stiffness` and `rotational_stiffness` dimensions.
> - `TBD`: release-level required vs optional classification beyond the current schema completeness rule.
> - `TBD`: movement-limit classes and dimensional validation categories.
> - `TBD`: hardware flag/enumeration taxonomy.
> - `TBD`: accepted public expansion-joint source catalog and public fixture-value policy.
> - `TBD`: dependency satisfaction, human disposition of review findings, and lifecycle closure.
>

### CLM-006 — Conditions

> ##### Conditions
>
> - Public repository artifacts must not include manufacturer proprietary values, protected standards text, protected examples, or copied data tables.
> - Missing solve-required or rule-check-required expansion joint values must remain explicit findings, never silent defaults.
> - Current schema dimensions classify stiffness as `linear_stiffness` and `rotational_stiffness`; any supplied numeric quantity must remain unit-aware and dimensionally checked before mechanics or rule-check use.
> - Outputs may support review but must not claim certification, authentication, or code compliance.
>

### CLM-007 — Construction

> ##### Construction
>
> Current construction is bounded to the component-library schema, invented fixture, and schema validation tests. The implementation evidence includes `expansion_joint` as a component type, expansion-joint family contract fields, an invented expansion-joint record with missing supplied values, a blocking completeness finding, and expansion-joint diagnostic codes.
>
> Current model partitions:
>
> | Partition | Content |
> |---|---|
> | Identity | Component ID, component type, library/source reference, provenance metadata |
> | Mechanical inputs | `linear_stiffness`, `rotational_stiffness`, `effective_area`, `movement_limit`, `hardware_flag`/`hardware_reference`, and `manufacturer_reference` slots |
> | Unit metadata | Component quantity dimensions for linear and rotational stiffness plus area-compatible supplied values |
> | Validation state | Missing data diagnostics, provenance warnings, assumption warnings |
> | Persistence hooks | Schema version, strict schema shape, fixture provenance, review status, redistribution status, and open decision records |
>

### CLM-008 — References

> ##### References
>
> - `_CONTEXT.md` for deliverable identity, architecture basis, and package scope.
> - `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 for SOW-010, OBJ-004, PKG-03, and AB-00-01/02/04/06/07/08.
> - `docs/CONTRACT.md` for OPS-K-IP-1..3, OPS-K-DATA-1..3, OPS-K-UNIT-1, OPS-K-MECH-1, OPS-K-AGENT-1..4.
> - `schemas/component.schema.yaml`, `fixtures/component/invented_component_library_valid.json`, and `tests/test_component_section_schema.py` for current implementation evidence.

## Completion and Reliance Basis — Epistemology

### CLM-009 — Specification: DEL-03-06 Expansion joint component model

> #### Specification: DEL-03-06 Expansion joint component model
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-010 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-03-06-DECL-001`.
>

### CLM-011 — Scope

> ##### Scope
>
> This deliverable-local specification covers the implemented backend data-model slice for an expansion joint component model. Current evidence is limited to component-library schema slots, an invented public fixture record, completeness rules, diagnostics, and schema tests for manufacturer/user/private-library supplied stiffness, effective area, movement limits, and hardware data.
>
> Exclusions:
>
> - No public engineering value implementation.
> - No manufacturer proprietary values.
> - No invented expansion joint defaults.
> - No certification, authentication, or compliance claims.
> - No rule evaluator or global solver implementation; PKG-03 explicitly excludes those responsibilities.
>

### CLM-012 — Requirements

> ##### Requirements
>
> | Req ID | Requirement | Basis |
> |---|---|---|
> | DEL-03-06-R-001 | The component model shall represent expansion joint stiffness data as supplied by a user, manufacturer, or lawful private library; current evidence implements `linear_stiffness` and `rotational_stiffness` slots without public values. | SOW-010 |
> | DEL-03-06-R-002 | The component model shall represent effective area as supplied data with units and provenance; current evidence implements the `effective_area` slot without public values. | SOW-010; OPS-K-UNIT-1; OPS-K-DATA-3 |
> | DEL-03-06-R-003 | The component model shall represent movement limits as supplied data with explicit missing-value handling; current evidence implements the `movement_limit` slot and missing-data diagnostic path while taxonomy remains `TBD`. | SOW-010; OPS-K-DATA-2 |
> | DEL-03-06-R-004 | The component model shall represent hardware data without deriving defaults from protected or proprietary sources; current evidence implements `hardware_flag`/`hardware_reference` slots while hardware taxonomy remains `TBD`. | SOW-010; OPS-K-IP-1; OPS-K-IP-3 |
> | DEL-03-06-R-005 | All expansion joint numeric values shall be unit-aware and dimensionally checked when persisted, imported, or used by downstream services. | OPS-K-UNIT-1; AB-00-04 |
> | DEL-03-06-R-006 | Source, provenance, license/redistribution status, and review disposition shall be carried for component data where applicable. | OPS-K-IP-2; OPS-K-DATA-3 |
> | DEL-03-06-R-007 | Missing solve-required or rule-check-required values shall produce explicit diagnostics or findings rather than silent defaults. | OPS-K-DATA-2; AB-00-06 |
> | DEL-03-06-R-008 | The model shall preserve layer/API boundaries so adapters and plugins cannot bypass validation, provenance, units, diagnostics, or public/private data controls. | AB-00-02; AB-00-07 |
> | DEL-03-06-R-009 | Validation tests shall cover schema/field presence, unit handling, missing-data diagnostics, provenance behavior, and protected-content guardrails where relevant. | AB-00-08 |
>

### CLM-013 — Residual TBDs and gates

> ###### Residual TBDs and gates
>
> The implementation evidence technically addresses the stale generic-stiffness concern by using accepted `linear_stiffness` and `rotational_stiffness` dimensions. The following items remain unresolved and must not be treated as closed until human-approved or supported by later authoritative source material:
>
> - `TBD`: exact per-axis stiffness field shape and solver degree-of-freedom mapping beyond the implemented dimensions.
> - `TBD`: release-level required vs optional field classification beyond the current completeness rule.
> - `TBD`: movement-limit validation classes.
> - `TBD`: hardware flag/enumeration taxonomy.
> - `TBD`: public expansion-joint source catalog policy and public fixture-value policy.
> - `TBD`: dependency satisfaction, human disposition of `Review_Findings.csv`, and lifecycle closure.
>

### CLM-014 — Standards

> ##### Standards
>
> No accessible expansion joint manufacturer standard, code clause, or proprietary product data is introduced by the current evidence. Applicable standards and exact clause references remain `TBD`. Any later standards-derived or manufacturer-derived value must be supplied by the user or lawfully imported private data and must not be bundled as a public default.
>

### CLM-015 — Verification

> ##### Verification
>
> | Requirement | Current Verification Evidence |
> |---|---|
> | DEL-03-06-R-001 through R-004 | `schemas/component.schema.yaml` and `fixtures/component/invented_component_library_valid.json` define expansion-joint component type, family contract fields, invented record fields, and protected-value policy; `tests/test_component_section_schema.py` asserts this coverage. |
> | DEL-03-06-R-005 | Tests assert component quantity dimensions include accepted `linear_stiffness` and `rotational_stiffness`, remain within accepted PKG-02 dimensions, and do not use retired dimensions. |
> | DEL-03-06-R-006 | The invented fixture carries source, license/redistribution, contributor certification, and review status on schema-slot records. |
> | DEL-03-06-R-007 | The invented fixture records an incomplete expansion-joint completeness finding and `EXPANSION_JOINT_STIFFNESS_DATA_MISSING` diagnostic. |
> | DEL-03-06-R-008 | Current evidence is schema/fixture/test only; solver, adapter, GUI, persistence-service, and report bypass checks remain downstream package scope or separate authorized scope. |
> | DEL-03-06-R-009 | `python3 -m pytest tests/test_component_section_schema.py` is the targeted validation for this reconciliation pass. |
> | Residual TBDs | Human ruling or later sealed work must define remaining taxonomy, source/value policy, dependency closure, lifecycle state, and review dispositions before completeness or release claims. |
>

### CLM-016 — Documentation

> ##### Documentation
>
> Current artifacts:
>
> - `schemas/component.schema.yaml`
> - `fixtures/component/invented_component_library_valid.json`
> - `tests/test_component_section_schema.py`
> - This DEL-03-06 evidence kit and run records.
>
> Future authorized work may add schema/API notes if the model is exposed through persistence services, import/export, adapters, GUI services, or reports.

- **AC-001** — The contract preserves accepted expansion-joint requirements and boundaries, including supplied-data-only values, explicit units and provenance, protected-content controls, missing-value diagnostics, unresolved taxonomy and solver mappings, and no invented defaults or professional approval.

## Production and Verification Method — Praxeology

### CLM-017 — Procedure: DEL-03-06 Expansion joint component model

> #### Procedure: DEL-03-06 Expansion joint component model
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-018 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-03-06-DECL-004`.
>

### CLM-019 — Purpose

> ##### Purpose
>
> Define the procedure for maintaining and extending the current expansion-joint data-model evidence without crossing protected-data, architecture, lifecycle, or package-boundary constraints.
>

### CLM-020 — Prerequisites

> ##### Prerequisites
>
> - Sealed brief for DEL-03-06 with explicit write scope.
> - Current `_CONTEXT.md`, `_REFERENCES.md`, `Datasheet.md`, `Specification.md`, and `Guidance.md`.
> - Current implementation evidence in `schemas/component.schema.yaml`, `fixtures/component/invented_component_library_valid.json`, and `tests/test_component_section_schema.py`.
> - Human-approved implementation scope before editing product files outside this deliverable folder.
> - Applicable architecture-basis constraints AB-00-01, AB-00-02, AB-00-04, AB-00-06, AB-00-07, and AB-00-08.
>

### CLM-021 — Steps

> ##### Steps
>
> 1. Confirm the requested work is still bounded to DEL-03-06 and SOW-010.
> 2. Identify the exact file targets and tests authorized by the active brief.
> 3. Reconcile documentation against current implementation evidence before adding new claims.
> 4. Confirm expansion joint fields for `linear_stiffness`, `rotational_stiffness`, `effective_area`, `movement_limit`, `hardware_flag`/`hardware_reference`, and `manufacturer_reference` remain unit/provenance bounded.
> 5. Leave all unsupported or source-dependent values as `TBD`; do not add defaults.
> 6. Preserve validation paths for missing required values, invalid dimensions, unsupported provenance, and protected-content risk.
> 7. Preserve architecture/API boundaries so adapters, plugins, persistence, GUI services, and reports consume validated envelopes rather than bypassing domain contracts.
> 8. Run or update validation tests for schema/model behavior, unit handling, diagnostics, provenance, and guardrails when authorized.
> 9. Record any unresolved field taxonomy, solver semantics, source/value policy, lifecycle, dependency-satisfaction, or standard/manufacturer interpretation question for human ruling.
>

### CLM-022 — Verification

> ##### Verification
>
> - Confirm no manufacturer proprietary values or protected standards content were introduced.
> - Confirm no certification/compliance claim was introduced.
> - Confirm missing values remain explicit diagnostics or `TBD`.
> - Confirm unit/provenance fields are represented in schema, fixture, and test evidence.
> - Confirm tests exercise missing-data and provenance guardrails.
>

### CLM-023 — Records

> ##### Records
>
> - Updated product implementation artifacts only under explicit authorized write scope.
> - Validation test results.
> - Human rulings for any field taxonomy, standards interpretation, or manufacturer-data handling decisions.
> - Dependency/register updates only when separately authorized and when implementation creates or consumes cross-deliverable interfaces.

- **VER-001** — Validate the contract and review source parity, field and fixture coverage, dimensional and provenance boundaries, missing-data diagnostics, protected-content controls, downstream solver limits, and unresolved human-review items.

## Governing Values and Decisions — Axiology

### CLM-024 — Guidance: DEL-03-06 Expansion joint component model

> #### Guidance: DEL-03-06 Expansion joint component model
>

### CLM-025 — Purpose

> ##### Purpose
>
> The purpose of this deliverable is to document the implemented expansion-joint component data-model slice while preserving OpenPipeStress boundaries: open mechanics and public schemas are allowed, but protected standards content, vendor proprietary values, and invented defaults are not.
>

### CLM-026 — Principles

> ##### Principles
>
> - Treat stiffness, effective area, movement limits, and hardware data as supplied inputs, not built-in knowledge.
> - Make missing required values visible with diagnostics and `TBD` placeholders.
> - Preserve provenance on every material/component data path where values may affect analysis, rules, reports, or redistribution.
> - Keep the expansion joint model inside PKG-03's data-model responsibility; solver interpretation and rule evaluation remain downstream package concerns.
> - Treat current schema, fixture, and test evidence as bounded data-model implementation evidence, not as a release, lifecycle, dependency-closure, or professional-reliance claim.
>

### CLM-027 — Considerations

> ##### Considerations
>
> Expansion joints can introduce nonlinear, directional, hardware-dependent, or manufacturer-specific behavior. Current evidence implements schema slots, fixture omissions, completeness findings, and diagnostics, but it does not include authoritative product data or design rules. Specific per-axis stiffness shape, movement-limit taxonomy, hardware enumerations, and downstream solver semantics remain `TBD`.
>
> The Pass 3 lensing register specifically keeps the hardware flag/enumeration taxonomy as `TBD`. Do not convert this into a fixed list without authoritative source material or human ruling.
>

### CLM-028 — Trade-offs

> ##### Trade-offs
>
> | Choice | Benefit | Risk / Constraint |
> |---|---|---|
> | Supplied-data-only fields | Protects IP boundary and avoids invented defaults. | Requires clear missing-data diagnostics and user/library workflows. |
> | Unit-aware field structure using accepted dimensions | Keeps persistence and adapters deterministic for the current schema evidence. | Exact per-axis solver mapping and movement/hardware taxonomy remain TBD. |
> | Provenance-first data model | Supports auditability and public/private library separation. | Requires validation and review fields even for simple examples. |
> | Data-model-only implementation scope | Respects PKG-03 boundaries. | Solver behavior and rule checks must be handled by downstream deliverables. |
>

### CLM-029 — Examples

> ##### Examples
>
> The current public fixture uses invented, non-engineering expansion-joint records with missing supplied values and blocking diagnostics. No manufacturer example values are provided or invented. Additional examples for documentation or tests must remain clearly non-commercial and must avoid protected standards or manufacturer data.
>

### CLM-030 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | None | No source conflict identified in accessible evidence sources. | N/A | N/A | N/A | N/A | N/A |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-010 OBJ-004 | CLM-009 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
