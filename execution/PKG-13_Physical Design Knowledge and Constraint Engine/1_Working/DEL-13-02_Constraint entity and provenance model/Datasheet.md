# Datasheet: DEL-13-02 Constraint entity and provenance model

## Identification

| Field | Value | Source |
|---|---|---|
| Deliverable ID | DEL-13-02 | `_CONTEXT.md` |
| Name | Constraint entity and provenance model | `_CONTEXT.md` |
| Package ID | PKG-13 | `_CONTEXT.md` |
| Package name | Physical Design Knowledge and Constraint Engine | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#PKG-13` |
| Type | DATA_MODEL_CHANGE | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` |
| Anticipated artifact | `schemas/constraint.schema.json` | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` |
| Anticipated artifact | constraint provenance model | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` |
| Scope coverage | SOW-068, SOW-067 | `_CONTEXT.md`; `docs/_Registers/ScopeLedger.csv` |
| Objective support | OBJ-014, OBJ-018 | `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#Objective-to-Deliverable Mapping` |
| Context envelope | M | `_CONTEXT.md`; `docs/_Registers/ContextBudgetQA.csv` |

## Attributes

| Attribute | Current source-grounded value |
|---|---|
| Constraint categories in scope | connectivity, route conflicts, clearance conflicts, support-zone conflicts, slope/drain/vent conflicts, and missing required data (`SOW-068`) |
| Related design knowledge inputs | endpoints, line data, routing corridors, no-go volumes, supportable zones, equipment interfaces, access constraints, slope/drain/vent requirements, and owner/project metadata (`SOW-067`) |
| Provenance posture | Constraint records must identify user/project/import/agent/source provenance where known (`_CONTEXT.md`; `docs/_Registers/Deliverables.csv`) |
| Model relationship | The physical model is the editable source of truth and carries typed references to design knowledge, constraints, diagnostics, unresolved assumptions, and traceability (`docs/SPEC.md#3 Domain model and schema`) |
| Schema baseline | `schemas/constraint.schema.json` implements a strict JSON Schema 2020-12 contract with `$schema` = `https://json-schema.org/draft/2020-12/schema` and `$id` = `https://openpipestress.org/schemas/constraint.schema.json` |
| Schema identity and version slot | The implemented root object requires `schema_version` with semantic-version string pattern `^[0-9]+\\.[0-9]+\\.[0-9]+$`; concrete instance version values remain payload-specific and review-controlled |
| Root required properties | `schema_version`, `deliverable_id`, `package_id`, `scope_items`, `objectives`, `data_boundary`, and `constraint_set` |
| Constraint set shape | `ConstraintSet` requires `constraint_set_id`, `project_ref`, `model_ref`, `design_knowledge_refs`, `constraints`, `diagnostics`, `provenance`, and `professional_boundary` |
| Constraint record shape | `ConstraintRecord` requires `constraint_id`, `constraint_kind`, `name`, `state`, `source_type`, `target_refs`, `design_knowledge_refs`, `parameters`, `diagnostics`, `assumptions`, `validation_status`, `provenance`, and `professional_boundary` |
| Constraint kind enum | `connectivity`, `clearance`, `no_go_volume`, `support_zone`, `route_conflict`, `slope`, `drain`, `vent`, `access`, `equipment_interface`, `missing_required_data`, and `TBD` |
| Diagnostic class enum | `CONSTRAINT_MISSING_DATA`, `CONNECTIVITY_CONFLICT`, `CLEARANCE_CONFLICT`, `ROUTE_CONFLICT`, `SUPPORT_ZONE_CONFLICT`, `SLOPE_DRAIN_VENT_CONFLICT`, `PROVENANCE_WARNING`, `UNIT_WARNING`, `IP_BOUNDARY_WARNING`, `SCHEMA_VALIDATION`, and `TBD` |
| Provenance shape | `Provenance` requires `source_name`, `source_location`, `source_license`, `contributor`, `contributor_certification`, `redistribution_status`, `review_status`, and `privacy_classification` |
| Unit posture | Unit-bearing values use `Quantity` with required `value`, `unit`, `dimension`, and `provenance`; the current dimension enum matches the accepted PKG-02 dimension vocabulary tested by `tests/test_constraint_schema.py` |
| Professional-boundary posture | Constraint entities and outputs must not encode software-generated professional approval, certification, sealing, authentication, or code-compliance labels (`docs/CONTRACT.md`; `docs/SPEC.md#4.3 Analysis status and authority boundary`) |
| IP/data posture | Public artifacts must not bundle protected owner standards, protected code criteria, proprietary project data, copied standards text, protected tables, or code-specific values (`INIT.md`; `docs/IP_AND_DATA_BOUNDARY.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#PKG-13`) |

## Conditions

| Condition | Handling |
|---|---|
| Source of constraints | Constraints are represented from available user/project/import/agent/source design knowledge. Protected owner or code standards are not bundled as public defaults. |
| Missing required data | Missing data is represented as an explicit constraint/diagnostic finding with provenance rather than silently defaulted. |
| Validation result role | The schema provides diagnostic and `validation_status` slots. Runtime validation behavior remains downstream in `DEL-13-03` and is still `TBD` for this deliverable. |
| Engineering acceptance | Constraint records may support review, but they do not provide professional acceptance or code-compliance certification. |
| Dependency inputs | The approved local dependency mirror records upstream architecture basis, design knowledge schema, canonical domain model schema, unit contract, persistence contract, and professional-boundary policy as active predecessor context. |
| Public examples | The implemented schema encodes `data_boundary.public_examples_policy` as `invented_or_cleared_data_only`; actual public example payload selection, review, and publication remain `TBD`. |
| Downstream consumption | Physical-to-analytical transform consumption, GUI presentation/blocking behavior, and runtime constraint validation remain downstream `TBD` items. |

## Construction

The deliverable is a data-model production unit. The implemented construction target is `schemas/constraint.schema.json`, a strict JSON Schema 2020-12 contract for provenance-marked constraint records. It defines root identity fields, `ConstraintSet`, `ConstraintRecord`, `ConstraintKind`, `Diagnostic`, `Parameter`, `Quantity`, `Provenance`, `Reference`, `ValidationStatus`, and `ProfessionalBoundary` definitions.

The schema structure, `$id`, `$schema`, required properties, reusable `$defs`, and enum spellings are no longer `TBD` for the implemented artifact. Future schema revisions remain human-gated bounded work. Runtime constraint validation, GUI presentation, physical-to-analytical transform consumption, and public example payload policy remain `TBD`; any future public examples must be invented or otherwise cleared for redistribution and must not contain protected standards data, owner standards, proprietary project data, code-specific limits, or professional-approval claims.

## References

- `_CONTEXT.md` - deliverable identity, scope, artifacts, context envelope, architecture-basis injection.
- `_REFERENCES.md` - deliverable-local reference index.
- `_DEPENDENCIES.md` and `Dependencies.csv` - approved DAG-006 mirror/evidence surface for active predecessor context.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` - revision 0.7 package and deliverable basis.
- `docs/_Registers/Deliverables.csv`, `docs/_Registers/ScopeLedger.csv`, `docs/_Registers/ContextBudgetQA.csv` - machine-readable deliverable, scope, and context rows.
- `INIT.md`, `docs/CONTRACT.md`, `docs/SPEC.md`, `docs/TYPES.md`, `docs/IP_AND_DATA_BOUNDARY.md` - governing boundaries for schema, provenance, unit safety, privacy/IP, and professional responsibility.
- `schemas/constraint.schema.json` - implemented JSON Schema 2020-12 constraint entity and provenance contract.
- `tests/test_constraint_schema.py` - stdlib structural evidence for schema identity, required definitions, enum coverage, unit dimensions, data-boundary constants, and professional-boundary constants.
