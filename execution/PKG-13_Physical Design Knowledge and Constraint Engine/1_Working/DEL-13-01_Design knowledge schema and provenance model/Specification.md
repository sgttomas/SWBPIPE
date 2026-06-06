# Specification: DEL-13-01 Design knowledge schema and provenance model

## Scope

This deliverable covers the schema and provenance model for user-supplied design knowledge used by the schema-backed physical design model. The named design-knowledge categories are endpoints, line data, routing corridors, no-go volumes, supportable zones, equipment interfaces, access constraints, slope/drain/vent requirements, owner/project metadata, source notes, and unresolved assumptions. Source: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-067 and DEL-13-01 row.

This deliverable excludes owner standards, protected code data, final engineering acceptance logic, automatic professional approval, and hidden/default engineering values. Source: `execution/_Decomposition/SOFTWARE_DECOMP.md` PKG-13 exclusions; `docs/CONTRACT.md`; `docs/IP_AND_DATA_BOUNDARY.md`.

Implementation evidence now exists for `schemas/design_knowledge.schema.json` and `tests/test_design_knowledge_schema.py`; see `RUN_2026-05-04_IMPLEMENTATION.md` and `MEMORY.md`. This specification records the implemented schema/test contract without implying lifecycle promotion, release readiness, runtime persistence/API integration, GUI behavior, constraint validation, transform consumption, public fixture policy closure, or professional/code-compliance authority. Unsupported downstream behavior remains `TBD`.

## Requirements

| ID | Requirement | Source | Verification |
|---|---|---|---|
| REQ-13-01-001 | The design knowledge schema shall support records for endpoints, line data, routing corridors, no-go volumes, supportable zones, equipment interfaces, access constraints, slope/drain/vent requirements, and owner/project metadata. | `_CONTEXT.md` Scope Detail; `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-067 | Schema review confirms named categories are represented directly or by traceable equivalent structures. |
| REQ-13-01-002 | The design knowledge schema shall include source-note and unresolved-assumption capability for design knowledge records. | `_CONTEXT.md` Description; `execution/_Decomposition/SOFTWARE_DECOMP.md` DEL-13-01 row | Schema review confirms source notes and unresolved assumptions are representable and traceable to affected records. |
| REQ-13-01-003 | Public repository artifacts for this deliverable shall not bundle protected owner standards, protected standards text, protected code data, proprietary project data, code-specific values, protected tables, or proprietary vendor data. | `docs/CONTRACT.md` OPS-K-IP-1 and OPS-K-DATA-1; `docs/IP_AND_DATA_BOUNDARY.md` sections 3 and 6 | Protected-content/private-data review finds no bundled prohibited values or text. |
| REQ-13-01-004 | Design knowledge values that are user/project supplied shall remain identifiable as user/project supplied rather than public defaults. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-067; `docs/CONTRACT.md` OPS-K-DATA-1 | Schema review confirms value records can carry source/provenance status and do not imply public defaults. |
| REQ-13-01-005 | Provenance records for public data contributions shall be able to capture source name, source location, source license or redistribution basis, contributor, contributor certification, redistribution status, and review status. | `docs/IP_AND_DATA_BOUNDARY.md` section 4 | Implemented schema review/test confirms required provenance fields and adds privacy classification. |
| REQ-13-01-006 | Unit-bearing physical values crossing the schema boundary shall carry explicit unit metadata unless the field is explicitly dimensionless, ratio, percentage, or coefficient. | `docs/SPEC.md` section 4 | Schema validation review confirms unit-bearing fields use an explicit quantity/unit representation or are explicitly classified non-unit-bearing. |
| REQ-13-01-007 | Missing required design knowledge, missing units, and unresolved assumptions shall be represented as explicit findings or records, not silently defaulted. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md` sections 4 and 4.4 | Schema/test review confirms diagnostics, assumptions, source notes, `TBD` enum values, and absence of schema `default` keys. |
| REQ-13-01-008 | The schema/provenance model shall preserve the product boundary that the physical model is the editable source of truth and that detailed design-knowledge records are owned by PKG-13 specialized schemas/services. | `docs/SPEC.md` section 3; `docs/TYPES.md` Model registry entry | Schema/interface review confirms design knowledge links to the physical model without bypassing the physical-model source-of-truth role. |
| REQ-13-01-009 | The deliverable shall not introduce software-generated professional approval, certification, sealing, authentication, or code-compliance status. | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/TYPES.md` Analysis-status vocabulary | Status and enum review confirms no automatic professional/code-compliance authority terms are emitted. |
| REQ-13-01-010 | The schema artifact shall follow the accepted JSON Schema 2020-12 contract baseline unless a later human-approved architecture decision changes it. | `_CONTEXT.md` Architecture Basis Injection; `execution/_Decomposition/SOFTWARE_DECOMP.md` SCA-001/SCA-002 basis | Schema file review/test confirms the JSON Schema 2020-12 declaration and stable `$id`. |
| REQ-13-01-011 | Schema, adapter, and service paths that consume design knowledge shall preserve schema validation, unit checks, provenance checks, private-data controls, protected-content screening, diagnostics/result envelopes, persistence hashes, and professional-boundary controls. | `docs/SPEC.md` sections 1, 4.4, and 4.5 | Integration review confirms no bypass route is introduced; unsupported routes remain `TBD`. |

## Standards

| Standard or governing source | Status in this deliverable |
|---|---|
| JSON Schema 2020-12 | Applicable architecture baseline from `_CONTEXT.md`; implemented by `$schema` and `$id` in `schemas/design_knowledge.schema.json`. |
| `docs/CONTRACT.md` | Governs IP, data, authority, unit, and agent invariants. |
| `docs/IP_AND_DATA_BOUNDARY.md` | Governs protected/private data exclusion and provenance requirements. |
| `docs/TYPES.md` | Governs identifiers, epistemic labels, analysis-status vocabulary, and domain object vocabulary. |
| `docs/SPEC.md` | Governs architecture, physical-model source-of-truth framing, unit metadata, provenance, persistence, and no-bypass controls. |
| External engineering standards | No authoritative external standards text is accessible or needed for this deliverable. Do not derive requirements from inaccessible standards. |

## Verification

| Verification target | Method | Acceptance signal |
|---|---|---|
| Scope coverage | Compare schema/provenance model against SOW-067 named categories. | Implemented record definitions and requirement/zone/endpoint enums cover the named categories; downstream behavior remains deferred where noted. |
| Public/private data boundary | Run protected-content/private-data review appropriate to the repository. | No prohibited protected/private data is present. |
| Provenance completeness | Review provenance fields against `docs/IP_AND_DATA_BOUNDARY.md` section 4. | Required provenance fields are implemented and tested; review disposition for findings remains human-gated. |
| Unit metadata | Review all unit-bearing fields. | `Quantity` requires `value`, `unit`, `dimension`, and `provenance`; no dimensionless fallback hides missing units. |
| Missing-data behavior | Review schema structure and tests. | Missing values are represented through diagnostics, assumptions, source notes, or `TBD` enums; the test confirms no schema `default` keys are present. |
| Professional boundary | Review enum/status/diagnostic strings. | Professional-boundary flags require human review and require false software compliance/certification/sealing/approval/authentication claims. |
| Dependency boundary | Confirm local dependency mirror remains approved DAG-006 evidence and all approved rows remain ACTIVE. | `Dependencies.csv` validates structurally and preserves DAG-002 rows. |

## Documentation

Required deliverable artifacts from `_CONTEXT.md` and `docs/_Registers/Deliverables.csv`:

- `schemas/design_knowledge.schema.json`
- design knowledge provenance model

Setup artifacts created by this workflow:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`

Implemented evidence:

- `schemas/design_knowledge.schema.json`
- `tests/test_design_knowledge_schema.py`
- `RUN_2026-05-04_IMPLEMENTATION.md`
- `MEMORY.md`

Deferred items remain `TBD`: GUI authoring behavior, constraint records and validation, physical-to-analytical transform consumption, runtime persistence/API integration, and public example/fixture generation policy closure.
