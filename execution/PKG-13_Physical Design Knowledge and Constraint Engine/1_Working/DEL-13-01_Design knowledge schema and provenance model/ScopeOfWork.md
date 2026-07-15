---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-13-01
package_id: PKG-13
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@2770fda4c63c98ee9f18cffbafd14c9aa59f497f
project_scope_refs: [SOW-067]
package_objective_refs: [OBJ-014]
---

# Scope of Work — DEL-13-01

## Purpose and Objective Traceability

This candidate defines `DEL-13-01` in service of project scope [SOW-067] and package objectives [OBJ-014].

- **OUT-001** — A design knowledge schema and provenance model for user-supplied endpoints, line data, routing corridors, no-go volumes, supportable zones, equipment interfaces, access constraints, slope/drain/vent requirements, owner/project metadata, source notes, and unresolved assumptions.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-13-01 Design knowledge schema and provenance model

> #### Datasheet: DEL-13-01 Design knowledge schema and provenance model
>

### CLM-002 — Identification

> ##### Identification
>
> | Field | Value | Source |
> |---|---|---|
> | Deliverable ID | DEL-13-01 | `_CONTEXT.md` |
> | Deliverable name | Design knowledge schema and provenance model | `_CONTEXT.md` |
> | Package ID | PKG-13 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` |
> | Package name | Physical Design Knowledge and Constraint Engine | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` |
> | Deliverable type | DATA_MODEL_CHANGE | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` |
> | Scope item | SOW-067 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md`; `docs/_Registers/ScopeLedger.csv` |
> | Objective support | OBJ-014 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md`; `docs/_Registers/Deliverables.csv` |
> | Anticipated artifacts | `schemas/design_knowledge.schema.json`; design knowledge provenance model | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` |
> | Context envelope | M | `_CONTEXT.md`; `docs/_Registers/ContextBudgetQA.csv` |
>

### CLM-003 — Attributes

> ##### Attributes
>
> | Attribute | Current value | Source |
> |---|---|---|
> | Primary subject | User-supplied design knowledge for the physical design model | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-067 |
> | Supported knowledge categories named by scope | Endpoints; line data; routing corridors; no-go volumes; supportable zones; equipment interfaces; access constraints; slope, drain, and vent requirements; owner/project metadata | `_CONTEXT.md` Scope Detail; `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-067 |
> | Additional named record concerns | Source notes and unresolved assumptions | `_CONTEXT.md` Description; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEL-13-01 row |
> | Physical-model relationship | Physical model is the source of truth for editable design data; detailed design-knowledge records are owned by PKG-13 specialized schemas/services | `docs/SPEC.md` section 3; `docs/TYPES.md` Canonical domain object registry |
> | Schema baseline | JSON Schema 2020-12 contracts are the accepted architecture basis for schema surfaces | `_CONTEXT.md` Architecture Basis Injection |
> | Public-data boundary | No protected owner standards, protected code data, or private project data may be bundled in public examples | `_CONTEXT.md`; `docs/CONTRACT.md` OPS-K-IP-1/OPS-K-DATA-1; `docs/IP_AND_DATA_BOUNDARY.md` |
> | Professional boundary | Records and software output must not claim certification, sealing, professional approval, or code compliance for reliance | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/TYPES.md` Analysis-status vocabulary |
> | Unit-bearing data boundary | Unit-bearing physical values crossing schema or service boundaries require explicit unit metadata unless explicitly dimensionless | `docs/SPEC.md` section 4 |
> | Missing-data posture | Missing solve-required or rule-check-required values must be explicit findings, never silent defaults | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md` section 4.4 |
>

### CLM-004 — Conditions

> ##### Conditions
>
> | Condition | Datasheet treatment | Source |
> |---|---|---|
> | Protected data exclusion | The deliverable must define schema/provenance slots without embedding owner standards, protected code criteria, proprietary project data, protected dimensional tables, protected standards text, or proprietary vendor data. | `docs/IP_AND_DATA_BOUNDARY.md` sections 3 and 6; `execution/_Decomposition/SOFTWARE_DECOMP.md` PKG-13 exclusions |
> | User-supplied knowledge | Design knowledge remains user/project supplied. Public examples, if later created, must use invented or otherwise cleared data. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-067; `docs/IP_AND_DATA_BOUNDARY.md` section 2 |
> | Provenance need | Public data records require source, location, license or redistribution basis, contributor/certification, redistribution status, and review status. | `docs/IP_AND_DATA_BOUNDARY.md` section 4 |
> | Dependency evidence | Local `Dependencies.csv` preserves DAG mirror/evidence, anchor, and package-local reconciliation rows with 14 ACTIVE rows; current graph authority is DAG-006, and the local register is not independent graph authority. | `_DEPENDENCIES.md`; `Dependencies.csv` |
> | Architecture-basis context | DEL-13-01 has upstream active architecture-basis rows from DEL-00-01, DEL-00-02, DEL-00-03, DEL-00-04, DEL-00-06, DEL-00-07, and DEL-00-08. | `Dependencies.csv` |
> | Domain/governance context | DEL-13-01 has upstream active rows for canonical domain model schema, unit system contract, copyright/protected-data boundary policy, and professional responsibility/product-claims policy. | `Dependencies.csv` |
>

### CLM-005 — Construction

> ##### Construction
>
> | Construction item | Status |
> |---|---|
> | `schemas/design_knowledge.schema.json` | Implemented at repo root as a strict JSON-syntax JSON Schema 2020-12 contract for user-supplied design knowledge records. Evidence: `RUN_2026-05-04_IMPLEMENTATION.md`; `MEMORY.md`; `schemas/design_knowledge.schema.json`. |
> | Schema identifier, `$id`, `$schema`, and versioning fields | Implemented. `$schema` is `https://json-schema.org/draft/2020-12/schema`, `$id` is `https://openpipestress.org/schemas/design_knowledge.schema.json`, and `schema_version` is required with a semantic-version pattern. |
> | Design knowledge record taxonomy | Implemented structurally through `EndpointRecord`, `LineDataRecord`, `RoutingCorridorRecord`, `ZoneRecord`, `EquipmentInterfaceRecord`, `RequirementRecord`, and `MetadataRecord`; downstream GUI behavior and constraint execution remain deferred. |
> | Provenance model fields | Implemented through required `Provenance` fields: `source_name`, `source_location`, `source_license`, `contributor`, `contributor_certification`, `redistribution_status`, `review_status`, and `privacy_classification`, plus source-note and assumption records. |
> | Unit-bearing quantity representation | Implemented through required `Quantity` fields `value`, `unit`, `dimension`, and `provenance`; the dimension enum matches the accepted PKG-02 vocabulary, including `slope` and excluding retired aliases. Runtime unit conversion/check integration remains downstream. |
> | Validation and tests | Implemented by `tests/test_design_knowledge_schema.py`, a focused stdlib structural test for schema draft, required definitions, category enums, provenance/privacy fields, quantity dimensions, no defaults, and professional-boundary flags. |
> | Product code evidence | Schema/test/documentation evidence exists. No runtime GUI authoring, constraint validation, physical-to-analytical transform consumption, persistence/API integration, or public fixture/example implementation is provided by DEL-13-01. |
>

### CLM-006 — References

> ##### References
>
> | Reference | Use in this datasheet |
> |---|---|
> | `_CONTEXT.md` | Deliverable identity, scope, artifacts, context envelope, architecture-basis injection. |
> | `_REFERENCES.md` | Reference index and source boundary for this deliverable. |
> | `_DEPENDENCIES.md` and `Dependencies.csv` | Approved DAG-002 mirror/evidence surface and active upstream dependency context. |
> | `execution/_Decomposition/SOFTWARE_DECOMP.md` | SOW-067, OBJ-014, PKG-13 boundary, DEL-13-01 row, OI-013. |
> | `docs/_Registers/Deliverables.csv` | Deliverable row and artifacts. |
> | `docs/_Registers/ScopeLedger.csv` | Scope ledger row for SOW-067. |
> | `docs/_Registers/ContextBudgetQA.csv` | Context envelope and no-protected-data note. |
> | `docs/CONTRACT.md` | Invariants for IP, data, authority, units, and agent behavior. |
> | `docs/TYPES.md` | Identifier rules, epistemic labels, analysis-status vocabulary, and domain object registry. |
> | `docs/SPEC.md` | Architecture, physical-model source-of-truth framing, schema/unit/provenance constraints, acceptance semantics. |
> | `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data boundary and required provenance fields. |

## Completion and Reliance Basis — Epistemology

### CLM-007 — Specification: DEL-13-01 Design knowledge schema and provenance model

> #### Specification: DEL-13-01 Design knowledge schema and provenance model
>

### CLM-008 — Scope

> ##### Scope
>
> This deliverable covers the schema and provenance model for user-supplied design knowledge used by the schema-backed physical design model. The named design-knowledge categories are endpoints, line data, routing corridors, no-go volumes, supportable zones, equipment interfaces, access constraints, slope/drain/vent requirements, owner/project metadata, source notes, and unresolved assumptions. Source: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-067 and DEL-13-01 row.
>
> This deliverable excludes owner standards, protected code data, final engineering acceptance logic, automatic professional approval, and hidden/default engineering values. Source: `execution/_Decomposition/SOFTWARE_DECOMP.md` PKG-13 exclusions; `docs/CONTRACT.md`; `docs/IP_AND_DATA_BOUNDARY.md`.
>
> Implementation evidence now exists for `schemas/design_knowledge.schema.json` and `tests/test_design_knowledge_schema.py`; see `RUN_2026-05-04_IMPLEMENTATION.md` and `MEMORY.md`. This specification records the implemented schema/test contract without implying lifecycle promotion, release readiness, runtime persistence/API integration, GUI behavior, constraint validation, transform consumption, public fixture policy closure, or professional/code-compliance authority. Unsupported downstream behavior remains `TBD`.
>

### CLM-009 — Requirements

> ##### Requirements
>
> | ID | Requirement | Source | Verification |
> |---|---|---|---|
> | REQ-13-01-001 | The design knowledge schema shall support records for endpoints, line data, routing corridors, no-go volumes, supportable zones, equipment interfaces, access constraints, slope/drain/vent requirements, and owner/project metadata. | `_CONTEXT.md` Scope Detail; `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-067 | Schema review confirms named categories are represented directly or by traceable equivalent structures. |
> | REQ-13-01-002 | The design knowledge schema shall include source-note and unresolved-assumption capability for design knowledge records. | `_CONTEXT.md` Description; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEL-13-01 row | Schema review confirms source notes and unresolved assumptions are representable and traceable to affected records. |
> | REQ-13-01-003 | Public repository artifacts for this deliverable shall not bundle protected owner standards, protected standards text, protected code data, proprietary project data, code-specific values, protected tables, or proprietary vendor data. | `docs/CONTRACT.md` OPS-K-IP-1 and OPS-K-DATA-1; `docs/IP_AND_DATA_BOUNDARY.md` sections 3 and 6 | Protected-content/private-data review finds no bundled prohibited values or text. |
> | REQ-13-01-004 | Design knowledge values that are user/project supplied shall remain identifiable as user/project supplied rather than public defaults. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-067; `docs/CONTRACT.md` OPS-K-DATA-1 | Schema review confirms value records can carry source/provenance status and do not imply public defaults. |
> | REQ-13-01-005 | Provenance records for public data contributions shall be able to capture source name, source location, source license or redistribution basis, contributor, contributor certification, redistribution status, and review status. | `docs/IP_AND_DATA_BOUNDARY.md` section 4 | Implemented schema review/test confirms required provenance fields and adds privacy classification. |
> | REQ-13-01-006 | Unit-bearing physical values crossing the schema boundary shall carry explicit unit metadata unless the field is explicitly dimensionless, ratio, percentage, or coefficient. | `docs/SPEC.md` section 4 | Schema validation review confirms unit-bearing fields use an explicit quantity/unit representation or are explicitly classified non-unit-bearing. |
> | REQ-13-01-007 | Missing required design knowledge, missing units, and unresolved assumptions shall be represented as explicit findings or records, not silently defaulted. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md` sections 4 and 4.4 | Schema/test review confirms diagnostics, assumptions, source notes, `TBD` enum values, and absence of schema `default` keys. |
> | REQ-13-01-008 | The schema/provenance model shall preserve the product boundary that the physical model is the editable source of truth and that detailed design-knowledge records are owned by PKG-13 specialized schemas/services. | `docs/SPEC.md` section 3; `docs/TYPES.md` Model registry entry | Schema/interface review confirms design knowledge links to the physical model without bypassing the physical-model source-of-truth role. |
> | REQ-13-01-009 | The deliverable shall not introduce software-generated professional approval, certification, sealing, authentication, or code-compliance status. | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/TYPES.md` Analysis-status vocabulary | Status and enum review confirms no automatic professional/code-compliance authority terms are emitted. |
> | REQ-13-01-010 | The schema artifact shall follow the accepted JSON Schema 2020-12 contract baseline unless a later human-approved architecture decision changes it. | `_CONTEXT.md` Architecture Basis Injection; `execution/_Decomposition/SOFTWARE_DECOMP.md` SCA-001/SCA-002 basis | Schema file review/test confirms the JSON Schema 2020-12 declaration and stable `$id`. |
> | REQ-13-01-011 | Schema, adapter, and service paths that consume design knowledge shall preserve schema validation, unit checks, provenance checks, private-data controls, protected-content screening, diagnostics/result envelopes, persistence hashes, and professional-boundary controls. | `docs/SPEC.md` sections 1, 4.4, and 4.5 | Integration review confirms no bypass route is introduced; unsupported routes remain `TBD`. |
>

### CLM-010 — Standards

> ##### Standards
>
> | Standard or governing source | Status in this deliverable |
> |---|---|
> | JSON Schema 2020-12 | Applicable architecture baseline from `_CONTEXT.md`; implemented by `$schema` and `$id` in `schemas/design_knowledge.schema.json`. |
> | `docs/CONTRACT.md` | Governs IP, data, authority, unit, and agent invariants. |
> | `docs/IP_AND_DATA_BOUNDARY.md` | Governs protected/private data exclusion and provenance requirements. |
> | `docs/TYPES.md` | Governs identifiers, epistemic labels, analysis-status vocabulary, and domain object vocabulary. |
> | `docs/SPEC.md` | Governs architecture, physical-model source-of-truth framing, unit metadata, provenance, persistence, and no-bypass controls. |
> | External engineering standards | No authoritative external standards text is accessible or needed for this deliverable. Do not derive requirements from inaccessible standards. |
>

### CLM-011 — Verification

> ##### Verification
>
> | Verification target | Method | Acceptance signal |
> |---|---|---|
> | Scope coverage | Compare schema/provenance model against SOW-067 named categories. | Implemented record definitions and requirement/zone/endpoint enums cover the named categories; downstream behavior remains deferred where noted. |
> | Public/private data boundary | Run protected-content/private-data review appropriate to the repository. | No prohibited protected/private data is present. |
> | Provenance completeness | Review provenance fields against `docs/IP_AND_DATA_BOUNDARY.md` section 4. | Required provenance fields are implemented and tested; review disposition for findings remains human-gated. |
> | Unit metadata | Review all unit-bearing fields. | `Quantity` requires `value`, `unit`, `dimension`, and `provenance`; no dimensionless fallback hides missing units. |
> | Missing-data behavior | Review schema structure and tests. | Missing values are represented through diagnostics, assumptions, source notes, or `TBD` enums; the test confirms no schema `default` keys are present. |
> | Professional boundary | Review enum/status/diagnostic strings. | Professional-boundary flags require human review and require false software compliance/certification/sealing/approval/authentication claims. |
> | Dependency boundary | Confirm local dependency mirror remains approved DAG-006 evidence and all approved rows remain ACTIVE. | `Dependencies.csv` validates structurally and preserves DAG-002 rows. |
>

### CLM-012 — Documentation

> ##### Documentation
>
> Required deliverable artifacts from `_CONTEXT.md` and `docs/_Registers/Deliverables.csv`:
>
> - `schemas/design_knowledge.schema.json`
> - design knowledge provenance model
>
> Setup artifacts created by this workflow:
>
> - `Datasheet.md`
> - `Specification.md`
> - `Guidance.md`
> - `Procedure.md`
> - `_SEMANTIC.md`
> - `_SEMANTIC_LENSING.md`
>
> Implemented evidence:
>
> - `schemas/design_knowledge.schema.json`
> - `tests/test_design_knowledge_schema.py`
> - `RUN_2026-05-04_IMPLEMENTATION.md`
> - `MEMORY.md`
>
> Deferred items remain `TBD`: GUI authoring behavior, constraint records and validation, physical-to-analytical transform consumption, runtime persistence/API integration, and public example/fixture generation policy closure.

- **AC-001** — All authoritative source content is preserved and mapped, and the source-defined category, provenance, unit, missing-data, public/private-data, and professional-boundary checks remain intact while unsupported downstream behavior remains explicitly deferred.

## Production and Verification Method — Praxeology

### CLM-013 — Procedure: DEL-13-01 Design knowledge schema and provenance model

> #### Procedure: DEL-13-01 Design knowledge schema and provenance model
>
> <!-- D41-R5-T7-PDU055-CURRENTNESS -->

### CLM-014 — D-41 R5 T7 PDU-055 current declaration

> ##### D-41 R5 T7 PDU-055 current declaration
>
> Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.
>
> Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.
>
> PDU-055 cited claim(s): `DEL-13-01-DECL-004`.
>

### CLM-015 — Purpose

> ##### Purpose
>
> Define a conservative procedure for maintaining and reviewing the implemented `schemas/design_knowledge.schema.json` artifact and its design knowledge provenance model. This procedure is delivery evidence for the schema/test surface, not runtime GUI, constraint-engine, transform, persistence/API, fixture, release, or professional-approval evidence.
>

### CLM-016 — Prerequisites

> ##### Prerequisites
>
> | Prerequisite | Status |
> |---|---|
> | Deliverable context for DEL-13-01 | Present in `_CONTEXT.md`. |
> | Reference index | Present in `_REFERENCES.md`. |
> | Approved dependency mirror | Present as `_DEPENDENCIES.md` and `Dependencies.csv`; preserve all approved DAG-006 rows as ACTIVE. |
> | Decomposition basis | `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7. |
> | Applicable invariants | `docs/CONTRACT.md`, `docs/TYPES.md`, `docs/SPEC.md`, and `docs/IP_AND_DATA_BOUNDARY.md`. |
> | Implementation evidence | Present in `RUN_2026-05-04_IMPLEMENTATION.md`, `MEMORY.md`, `schemas/design_knowledge.schema.json`, and `tests/test_design_knowledge_schema.py`. |
> | Product schema artifact | Present at repo root: `schemas/design_knowledge.schema.json`. |
>

### CLM-017 — Steps

> ##### Steps
>
> 1. Confirm scope identity.
>    - Verify the active deliverable is DEL-13-01 under PKG-13.
>    - Confirm scope item SOW-067 and objective OBJ-014.
>
> 2. Establish the source boundary.
>    - Use `_CONTEXT.md`, `_REFERENCES.md`, the approved local dependency mirror, the decomposition, and cited governing documents.
>    - Do not import owner standards, protected code data, private project values, proprietary vendor data, or inaccessible standards text.
>
> 3. Maintain the design knowledge taxonomy.
>    - Include the sourced categories: endpoints, line data, routing corridors, no-go volumes, supportable zones, equipment interfaces, access constraints, slope/drain/vent requirements, owner/project metadata, source notes, and assumptions.
>    - Preserve implemented record kinds, required fields, and enums in `schemas/design_knowledge.schema.json`.
>    - Keep coordinate-frame policy, tolerance handling, numeric criteria, GUI behavior, constraint validation, and transform consumption as `TBD` unless a later bounded deliverable resolves them.
>
> 4. Draft the provenance model.
>    - Include support for source name, source location, source license or redistribution basis, contributor, contributor certification, redistribution status, and review status where public data contribution rules apply.
>    - Include source notes and unresolved assumptions attached to affected design knowledge records.
>
> 5. Apply unit and missing-data rules.
>    - For unit-bearing physical values, require explicit unit metadata unless a field is explicitly dimensionless, ratio, percentage, or coefficient.
>    - Represent missing data, missing units, unknown source, and unresolved assumptions as explicit records/findings.
>
> 6. Preserve boundaries to adjacent deliverables.
>    - Keep deterministic constraint validation in DEL-13-03.
>    - Keep constraint entity/provenance details that are not design-knowledge records in DEL-13-02.
>    - Keep physical-to-analytical transformation warnings and omissions in DEL-13-04.
>    - Use links/references rather than duplicating adjacent deliverable responsibilities.
>
> 7. Review the public/private and professional boundaries.
>    - Confirm no protected standards text, code-specific tables, proprietary project data, owner standards, private rule-pack data, or copied commercial examples are present.
>    - Confirm no automatic professional approval, certification, sealing, authentication, or code-compliance status is introduced.
>
> 8. Validate current artifacts.
>    - Validate JSON Schema syntax and declared draft/version.
>    - Run `python3 tests/test_design_knowledge_schema.py`.
>    - Validate fixtures against the schema only when governed fixtures exist.
>    - Run protected-content/private-data checks appropriate to public examples.
>    - Run dependency schema validation on local `Dependencies.csv` when present.
>

### CLM-018 — Verification

> ##### Verification
>
> | Check | Expected result |
> |---|---|
> | Scope check | DEL-13-01, PKG-13, SOW-067, and OBJ-014 match `_CONTEXT.md` and decomposition. |
> | Four-document consistency | Datasheet, Specification, Guidance, and Procedure use the same category list and boundary terms. |
> | Unsupported details | Unsupported downstream/runtime specifics are marked `TBD` or `ASSUMPTION`. |
> | Public/private boundary | No prohibited protected/private data is introduced. |
> | Professional boundary | No automatic professional or code-compliance authority is introduced. |
> | Dependency mirror | Existing approved DAG-006 rows remain ACTIVE and structurally valid. |
>

### CLM-019 — Records

> ##### Records
>
> | Record | Purpose |
> |---|---|
> | `Datasheet.md` | Descriptive identity, attributes, conditions, construction status, and references. |
> | `Specification.md` | Normative scope, requirements, standards, verification, and documentation. |
> | `Guidance.md` | Directional principles, considerations, trade-offs, examples, and conflicts. |
> | `Procedure.md` | Operational production/review sequence and evidence expectations. |
> | `_SEMANTIC.md` | Semantic lens generated after initial four-document setup. |
> | `_SEMANTIC_LENSING.md` | Candidate enrichment register generated from semantic lenses and production docs. |
> | `Dependencies.csv` | Approved local DAG-002 mirror/evidence surface; not independently regenerated in conflict with the project rule. |
> | `schemas/design_knowledge.schema.json` | Implemented JSON Schema 2020-12 design knowledge contract at repo root. |
> | `tests/test_design_knowledge_schema.py` | Focused stdlib structural validation for the implemented schema. |
> | `RUN_2026-05-04_IMPLEMENTATION.md` | Deliverable-local implementation evidence note. |

- **VER-001** — Validate the candidate schema and claim map; confirm parity and source/status hashes; run the focused design-knowledge schema test and applicable protected-content and dependency checks; and independently review source grounding and authority boundaries.

## Governing Values and Decisions — Axiology

### CLM-020 — Guidance: DEL-13-01 Design knowledge schema and provenance model

> #### Guidance: DEL-13-01 Design knowledge schema and provenance model
>

### CLM-021 — Purpose

> ##### Purpose
>
> This deliverable prepares the design-knowledge schema/provenance surface for the PKG-13 physical design knowledge and constraint engine. It exists so the physical model can carry user-supplied design context, record where that context came from, expose missing assumptions, and remain inside the public/private data and professional-responsibility boundaries. Source: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-067, OBJ-014, and PKG-13.
>

### CLM-022 — Principles

> ##### Principles
>
> | Principle | Guidance | Source |
> |---|---|---|
> | Schema first | Treat the implemented `schemas/design_knowledge.schema.json` as the contract surface for design knowledge records; runtime behavior and adjacent deliverable consumption remain deferred where not implemented. | `_CONTEXT.md` Anticipated Artifacts; `_CONTEXT.md` Architecture Basis Injection; `RUN_2026-05-04_IMPLEMENTATION.md`; `MEMORY.md` |
> | User-supplied by default | Design knowledge values are user/project supplied unless explicitly invented or otherwise cleared for public redistribution. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-067; `docs/IP_AND_DATA_BOUNDARY.md` |
> | Provenance is part of the record | Source notes, source status, review status, redistribution status, and assumptions should be close to the data they qualify. | `docs/IP_AND_DATA_BOUNDARY.md` section 4; `_CONTEXT.md` Description |
> | No silent defaults | Missing design knowledge, missing units, or unresolved assumptions should surface as findings/TBD records. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md` sections 4 and 4.4 |
> | Physical source of truth | Design knowledge should support the editable physical model without bypassing model schema validation or downstream transformation traceability. | `docs/SPEC.md` section 3; `docs/TYPES.md` Model registry entry |
> | Professional boundary | Avoid terms that imply software certification, approval, sealing, authentication, or code compliance. | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/TYPES.md` section 4 |
>

### CLM-023 — Considerations

> ##### Considerations
>
> | Topic | Consideration |
> |---|---|
> | Category granularity | The implemented schema defines concrete record kinds for endpoints, line data, routing corridors, zones, equipment interfaces, requirements, and owner/project metadata. GUI presentation, editing workflows, and constraint-engine behavior remain downstream. |
> | Owner/project metadata | Owner/project metadata is in scope, but public examples must not include private project data or protected owner standards. |
> | Equipment interfaces | The schema provides `equipment_ref`, `interface_role`, `location`, `interface_parameters`, source notes, assumptions, and provenance. Runtime equipment-picking behavior and downstream transform semantics remain `TBD`. |
> | Access, slope, drain, and vent requirements | These may reflect owner/project requirements. The schema should provide slots and provenance without embedding protected or private criteria. |
> | No-go and supportable zones | The schema provides `ZoneRecord` and `GeometryPayload` structures for no-go, supportable, access, maintenance, and operating zones. Coordinate-frame policy, tolerance handling, and numeric clearance/support criteria remain `TBD` and must not be invented in public artifacts. |
> | Relationship to constraints | DEL-13-02 and DEL-13-03 own constraint entity/provenance and validation behavior. DEL-13-01 should provide design knowledge records that can later be consumed by those deliverables. |
> | Relationship to transformation | DEL-13-04 owns physical-to-analytical transformation. DEL-13-01 should preserve enough provenance/assumption structure for transformation warnings, but exact warning classes remain downstream. |
>

### CLM-024 — Trade-offs

> ##### Trade-offs
>
> | Trade-off | Preferred posture | Rationale |
> |---|---|---|
> | Rich taxonomy vs premature specificity | Use the implemented schema taxonomy for records and provenance; leave downstream numeric criteria, validation algorithms, GUI workflows, and transform behavior `TBD` where sources are silent. | `docs/CONTRACT.md` OPS-K-AGENT-1 prohibits invented engineering values or scope. |
> | Public examples vs realistic owner data | Use invented or cleared public data only. | `docs/IP_AND_DATA_BOUNDARY.md` excludes private owner standards and protected data. |
> | Embedded validation vs separate constraint engine | Keep record schema and provenance in this deliverable; leave deterministic constraint validation to DEL-13-03. | `execution/_Decomposition/SOFTWARE_DECOMP.md` separates DEL-13-01, DEL-13-02, DEL-13-03, and DEL-13-04. |
> | Flexibility vs interoperability | Preserve stable identifiers, source notes, unit metadata, and review status so records can participate in persistence, diagnostics, and handoff workflows. | `docs/SPEC.md` sections 3, 4, and 4.5. |
>

### CLM-025 — Examples

> ##### Examples
>
> No authoritative example payloads or public fixtures are implemented for this deliverable. Example JSON records, numeric values, owner/project metadata samples, and public fixtures remain `TBD`. Any later examples must be invented or otherwise cleared for public redistribution and must not copy protected standards text, protected tables, private project data, or proprietary vendor data.
>

### CLM-026 — Conflict Table (for human ruling)

> ##### Conflict Table (for human ruling)
>
> | Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
> |---|---|---|---|---|---|---|
> | None | No source conflicts identified during the implementation-evidence refresh. Unsupported downstream details remain marked `TBD` rather than resolved by inference. | N/A | N/A | N/A | N/A | N/A |

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-067 OBJ-014 | CLM-007 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |
