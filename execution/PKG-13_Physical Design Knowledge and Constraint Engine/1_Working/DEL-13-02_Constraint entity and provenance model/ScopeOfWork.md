---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-13-02
package_id: PKG-13
decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@2770fda4c63c98ee9f18cffbafd14c9aa59f497f
project_scope_refs: [SOW-068, SOW-067]
package_objective_refs: [OBJ-014, OBJ-018]
---

# Scope of Work — DEL-13-02

## Purpose and Objective Traceability

This candidate defines `DEL-13-02` in service of project scope [SOW-068, SOW-067] and package objectives [OBJ-014, OBJ-018].

- **OUT-001** — A constraint entity and provenance model for connectivity, route, clearance, no-go and support-zone, slope/drain/vent, and missing-data constraints associated with user-supplied design knowledge.

## Deliverable Definition — Ontology

### CLM-001 — Datasheet: DEL-13-02 Constraint entity and provenance model

<!-- sow-source-begin {"disposition":"PRESERVED","file":"Datasheet.md","line_end":2,"line_start":1,"source_sha256":"ea24357be99ffdc9aabf38e281a1814e637dbffed91e4ce5618695008f2ec2a1","target_id":"CLM-001"} -->
#### Datasheet: DEL-13-02 Constraint entity and provenance model

<!-- sow-source-end -->

### CLM-002 — Identification

<!-- sow-source-begin {"disposition":"PRESERVED","file":"Datasheet.md","line_end":17,"line_start":3,"source_sha256":"ea24357be99ffdc9aabf38e281a1814e637dbffed91e4ce5618695008f2ec2a1","target_id":"CLM-002"} -->
##### Identification

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

<!-- sow-source-end -->

### CLM-003 — Attributes

<!-- sow-source-begin {"disposition":"PRESERVED","file":"Datasheet.md","line_end":37,"line_start":18,"source_sha256":"ea24357be99ffdc9aabf38e281a1814e637dbffed91e4ce5618695008f2ec2a1","target_id":"CLM-003"} -->
##### Attributes

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

<!-- sow-source-end -->

### CLM-004 — Conditions

<!-- sow-source-begin {"disposition":"PRESERVED","file":"Datasheet.md","line_end":49,"line_start":38,"source_sha256":"ea24357be99ffdc9aabf38e281a1814e637dbffed91e4ce5618695008f2ec2a1","target_id":"CLM-004"} -->
##### Conditions

| Condition | Handling |
|---|---|
| Source of constraints | Constraints are represented from available user/project/import/agent/source design knowledge. Protected owner or code standards are not bundled as public defaults. |
| Missing required data | Missing data is represented as an explicit constraint/diagnostic finding with provenance rather than silently defaulted. |
| Validation result role | The schema provides diagnostic and `validation_status` slots. Runtime validation behavior remains downstream in `DEL-13-03` and is still `TBD` for this deliverable. |
| Engineering acceptance | Constraint records may support review, but they do not provide professional acceptance or code-compliance certification. |
| Dependency inputs | The approved local dependency mirror records upstream architecture basis, design knowledge schema, canonical domain model schema, unit contract, persistence contract, and professional-boundary policy as active predecessor context. |
| Public examples | The implemented schema encodes `data_boundary.public_examples_policy` as `invented_or_cleared_data_only`; actual public example payload selection, review, and publication remain `TBD`. |
| Downstream consumption | Physical-to-analytical transform consumption, GUI presentation/blocking behavior, and runtime constraint validation remain downstream `TBD` items. |

<!-- sow-source-end -->

### CLM-005 — Construction

<!-- sow-source-begin {"disposition":"PRESERVED","file":"Datasheet.md","line_end":55,"line_start":50,"source_sha256":"ea24357be99ffdc9aabf38e281a1814e637dbffed91e4ce5618695008f2ec2a1","target_id":"CLM-005"} -->
##### Construction

The deliverable is a data-model production unit. The implemented construction target is `schemas/constraint.schema.json`, a strict JSON Schema 2020-12 contract for provenance-marked constraint records. It defines root identity fields, `ConstraintSet`, `ConstraintRecord`, `ConstraintKind`, `Diagnostic`, `Parameter`, `Quantity`, `Provenance`, `Reference`, `ValidationStatus`, and `ProfessionalBoundary` definitions.

The schema structure, `$id`, `$schema`, required properties, reusable `$defs`, and enum spellings are no longer `TBD` for the implemented artifact. Future schema revisions remain human-gated bounded work. Runtime constraint validation, GUI presentation, physical-to-analytical transform consumption, and public example payload policy remain `TBD`; any future public examples must be invented or otherwise cleared for redistribution and must not contain protected standards data, owner standards, proprietary project data, code-specific limits, or professional-approval claims.

<!-- sow-source-end -->

### CLM-006 — References

<!-- sow-source-begin {"disposition":"PRESERVED","file":"Datasheet.md","line_end":65,"line_start":56,"source_sha256":"ea24357be99ffdc9aabf38e281a1814e637dbffed91e4ce5618695008f2ec2a1","target_id":"CLM-006"} -->
##### References

- `_CONTEXT.md` - deliverable identity, scope, artifacts, context envelope, architecture-basis injection.
- `_REFERENCES.md` - deliverable-local reference index.
- `_DEPENDENCIES.md` and `Dependencies.csv` - approved DAG-006 mirror/evidence surface for active predecessor context.
- `execution/_Decomposition/SOFTWARE_DECOMP.md` - revision 0.7 package and deliverable basis.
- `docs/_Registers/Deliverables.csv`, `docs/_Registers/ScopeLedger.csv`, `docs/_Registers/ContextBudgetQA.csv` - machine-readable deliverable, scope, and context rows.
- `INIT.md`, `docs/CONTRACT.md`, `docs/SPEC.md`, `docs/TYPES.md`, `docs/IP_AND_DATA_BOUNDARY.md` - governing boundaries for schema, provenance, unit safety, privacy/IP, and professional responsibility.
- `schemas/constraint.schema.json` - implemented JSON Schema 2020-12 constraint entity and provenance contract.
- `tests/test_constraint_schema.py` - stdlib structural evidence for schema identity, required definitions, enum coverage, unit dimensions, data-boundary constants, and professional-boundary constants.
<!-- sow-source-end -->

## Completion and Reliance Basis — Epistemology

### CLM-007 — Specification: DEL-13-02 Constraint entity and provenance model

<!-- sow-source-begin {"disposition":"PRESERVED","file":"Specification.md","line_end":2,"line_start":1,"source_sha256":"6a633b130a1132679e4f610ce5ed5b15e2fa9646518a56180d4d05bad9cf5344","target_id":"CLM-007"} -->
#### Specification: DEL-13-02 Constraint entity and provenance model

<!-- sow-source-end -->

### CLM-008 — Scope

<!-- sow-source-begin {"disposition":"PRESERVED","file":"Specification.md","line_end":8,"line_start":3,"source_sha256":"6a633b130a1132679e4f610ce5ed5b15e2fa9646518a56180d4d05bad9cf5344","target_id":"CLM-008"} -->
##### Scope

This deliverable specifies the data-model contract for representing constraint entities and their provenance for the PKG-13 physical design knowledge and constraint engine. It covers constraint records for connectivity, route conflicts, clearance conflicts, no-go/support-zone conflicts, slope/drain/vent conflicts, and missing required data, using the design knowledge categories named by SOW-067 and SOW-068.

This deliverable does not implement the constraint validation engine, infer hidden owner standards, define protected code requirements, bundle proprietary project data, create final engineering acceptance logic, or claim professional approval. Those exclusions are grounded in the PKG-13 decomposition notes, `INIT.md`, `docs/CONTRACT.md`, and `docs/IP_AND_DATA_BOUNDARY.md`.

<!-- sow-source-end -->

### CLM-009 — Implementation Evidence

<!-- sow-source-begin {"disposition":"PRESERVED","file":"Specification.md","line_end":14,"line_start":9,"source_sha256":"6a633b130a1132679e4f610ce5ed5b15e2fa9646518a56180d4d05bad9cf5344","target_id":"CLM-009"} -->
##### Implementation Evidence

The current implementation evidence is `schemas/constraint.schema.json` and `tests/test_constraint_schema.py`. The schema declares JSON Schema 2020-12, `$id` `https://openpipestress.org/schemas/constraint.schema.json`, root constants for `DEL-13-02` and `PKG-13`, and required root properties for `schema_version`, `deliverable_id`, `package_id`, `scope_items`, `objectives`, `data_boundary`, and `constraint_set`.

The schema defines reusable `$defs` for `ConstraintSet`, `ConstraintRecord`, `ConstraintKind`, `Diagnostic`, `Parameter`, `Quantity`, `Provenance`, `Reference`, `ValidationStatus`, and `ProfessionalBoundary`. The accompanying stdlib test checks schema identity, required definitions, required fields, enum coverage, PKG-02 dimension vocabulary alignment, protected-text exclusions, data-boundary constants, and professional-boundary constants.

<!-- sow-source-end -->

### CLM-010 — Requirements

<!-- sow-source-begin {"disposition":"PRESERVED","file":"Specification.md","line_end":29,"line_start":15,"source_sha256":"6a633b130a1132679e4f610ce5ed5b15e2fa9646518a56180d4d05bad9cf5344","target_id":"CLM-010"} -->
##### Requirements

| ID | Requirement | Source |
|---|---|---|
| R-13-02-001 | The constraint model shall represent constraint records for connectivity, route conflicts, clearance conflicts, support-zone conflicts, slope/drain/vent conflicts, and missing required data. | SOW-068 in `_CONTEXT.md`; `docs/_Registers/ScopeLedger.csv` |
| R-13-02-002 | The constraint model shall support association to user-supplied design knowledge categories including endpoints, line data, routing corridors, no-go volumes, supportable zones, equipment interfaces, access constraints, slope/drain/vent requirements, and owner/project metadata. | SOW-067 in `_CONTEXT.md`; `docs/_Registers/ScopeLedger.csv` |
| R-13-02-003 | Constraint records shall carry provenance sufficient to identify user/project/import/agent/source provenance where known. | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv`; `docs/_Registers/ContextBudgetQA.csv` |
| R-13-02-004 | Missing solve-required or rule-check-required values shall be modeled as explicit findings or diagnostics, not silent defaults. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md#4.3 Analysis status and authority boundary` |
| R-13-02-005 | Unit-bearing physical values referenced by constraints shall use the canonical unit contract or another explicitly approved unit-bearing quantity representation; dimensionless treatment shall not be used as a fallback for missing units. | `docs/SPEC.md#4 Unit system and dimensional analysis`; `docs/CONTRACT.md` OPS-K-UNIT-1 |
| R-13-02-006 | Public schema artifacts and examples shall not embed protected standards text, protected tables, protected criteria, proprietary catalog data, private owner standards, private project data, or code-specific values. | `INIT.md`; `docs/IP_AND_DATA_BOUNDARY.md`; `docs/CONTRACT.md` OPS-K-IP-1, OPS-K-DATA-1 |
| R-13-02-007 | The constraint model shall not define software-generated statuses for certification, sealing, authentication, professional approval, or code-compliance acceptance. | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/SPEC.md#4.3 Analysis status and authority boundary` |
| R-13-02-008 | The schema shall align with the accepted JSON Schema 2020-12 basis where JSON schema artifacts are produced. | `_CONTEXT.md#Architecture Basis Injection`; `docs/TYPES.md#Domain object vocabulary` |
| R-13-02-009 | Constraint records shall be able to participate in the physical source-of-truth model through typed references, diagnostics/warnings, unresolved assumptions, and traceability links. | `docs/SPEC.md#3 Domain model and schema`; `docs/TYPES.md#Domain object vocabulary` |
| R-13-02-010 | The implemented schema shall declare stable schema identity and a version slot: `$schema` = `https://json-schema.org/draft/2020-12/schema`, `$id` = `https://openpipestress.org/schemas/constraint.schema.json`, and required `schema_version` matching `^[0-9]+\\.[0-9]+\\.[0-9]+$`. | `schemas/constraint.schema.json`; `tests/test_constraint_schema.py` |

<!-- sow-source-end -->

### CLM-011 — Standards

<!-- sow-source-begin {"disposition":"PRESERVED","file":"Specification.md","line_end":38,"line_start":30,"source_sha256":"6a633b130a1132679e4f610ce5ed5b15e2fa9646518a56180d4d05bad9cf5344","target_id":"CLM-011"} -->
##### Standards

| Standard or basis | Status for this deliverable |
|---|---|
| JSON Schema 2020-12 | Implemented by `schemas/constraint.schema.json`; verified by `python3 -m json.tool schemas/constraint.schema.json` and `tests/test_constraint_schema.py`. |
| Project IP/data-boundary policy | Governing source for public/private data handling and protected-content exclusions. |
| Project invariant catalog | Governing source for provenance, unit safety, missing-data findings, and professional-boundary requirements. |
| Owner standards and engineering codes | Not bundled or paraphrased here. Any private owner/code requirements remain user-supplied or privately imported with provenance and review status. |

<!-- sow-source-end -->

### CLM-012 — Verification

<!-- sow-source-begin {"disposition":"PRESERVED","file":"Specification.md","line_end":51,"line_start":39,"source_sha256":"6a633b130a1132679e4f610ce5ed5b15e2fa9646518a56180d4d05bad9cf5344","target_id":"CLM-012"} -->
##### Verification

| Requirement IDs | Verification approach |
|---|---|
| R-13-02-001, R-13-02-002 | `tests/test_constraint_schema.py` checks required `ConstraintKind` values and diagnostic classes for the in-scope categories. |
| R-13-02-003 | `tests/test_constraint_schema.py` checks `SourceType` coverage for `user`, `project`, `import`, `agent`, and `source_derived`, plus required `Provenance` fields. |
| R-13-02-004 | Schema review checks `missing_required_data`, `CONSTRAINT_MISSING_DATA`, diagnostics, assumptions, and missing-data validation statuses; runtime validation behavior remains downstream in `DEL-13-03`. |
| R-13-02-005 | `tests/test_constraint_schema.py` checks `Quantity` required fields and exact alignment with the accepted PKG-02 dimension vocabulary. |
| R-13-02-006 | `tests/test_constraint_schema.py` checks data-boundary constants and scans schema strings for forbidden protected/professional-claim text. |
| R-13-02-007 | `tests/test_constraint_schema.py` checks `ProfessionalBoundary` constants require human review and prohibit software compliance, certification, sealing, approval, and authentication claims. |
| R-13-02-008, R-13-02-010 | `python3 -m json.tool schemas/constraint.schema.json` checks JSON syntax; `schemas/constraint.schema.json` declares `$id`; `tests/test_constraint_schema.py` checks the JSON Schema 2020-12 declaration, root identity constants, and required version slot. |
| R-13-02-009 | Schema review confirms typed `Reference` objects, diagnostics, assumptions, provenance, and model/design-knowledge references are available; physical-to-analytical transform consumption remains downstream `TBD`. |

<!-- sow-source-end -->

### CLM-013 — Documentation

<!-- sow-source-begin {"disposition":"PRESERVED","file":"Specification.md","line_end":62,"line_start":52,"source_sha256":"6a633b130a1132679e4f610ce5ed5b15e2fa9646518a56180d4d05bad9cf5344","target_id":"CLM-013"} -->
##### Documentation

Required deliverable documentation and evidence:

- `schemas/constraint.schema.json` or a human-approved successor path.
- Constraint provenance model definitions inside `schemas/constraint.schema.json`.
- `tests/test_constraint_schema.py` as current focused stdlib structural evidence.
- Traceability from schema elements to SOW-067, SOW-068, OBJ-014, and OBJ-018.
- Data-boundary notes for any examples or fixtures.
- Verification notes for JSON syntax validation, unit/provenance compatibility, missing-data diagnostics, and professional-boundary review.
- Public example payloads, GUI presentation behavior, runtime validation behavior, and transform consumption remain `TBD` until later bounded work.
<!-- sow-source-end -->

- **AC-001** — All authoritative source content is preserved and mapped, and the source-defined constraint categories, provenance, unit, missing-data, public/private-data, and professional-boundary requirements remain intact while runtime constraint validation remains deferred to DEL-13-03.

## Production and Verification Method — Praxeology

### CLM-014 — Procedure: DEL-13-02 Constraint entity and provenance model

<!-- sow-source-begin {"disposition":"PRESERVED","file":"Procedure.md","line_end":3,"line_start":1,"source_sha256":"6a1771f082701da20ee6580b8dde7389d7504c1c5a61dc0f131197838f5fc934","target_id":"CLM-014"} -->
#### Procedure: DEL-13-02 Constraint entity and provenance model

<!-- D41-R5-T7-PDU055-CURRENTNESS -->
<!-- sow-source-end -->

### CLM-015 — D-41 R5 T7 PDU-055 current declaration

<!-- sow-source-begin {"disposition":"PRESERVED","file":"Procedure.md","line_end":11,"line_start":4,"source_sha256":"6a1771f082701da20ee6580b8dde7389d7504c1c5a61dc0f131197838f5fc934","target_id":"CLM-015"} -->
##### D-41 R5 T7 PDU-055 current declaration

Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.

Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

PDU-055 cited claim(s): `DEL-13-02-DECL-004`.

<!-- sow-source-end -->

### CLM-016 — Purpose

<!-- sow-source-begin {"disposition":"PRESERVED","file":"Procedure.md","line_end":15,"line_start":12,"source_sha256":"6a1771f082701da20ee6580b8dde7389d7504c1c5a61dc0f131197838f5fc934","target_id":"CLM-016"} -->
##### Purpose

Provide a bounded procedure for producing and checking the DEL-13-02 constraint entity and provenance model artifacts without crossing the data boundary or claiming implementation evidence that does not yet exist.

<!-- sow-source-end -->

### CLM-017 — Prerequisites

<!-- sow-source-begin {"disposition":"PRESERVED","file":"Procedure.md","line_end":26,"line_start":16,"source_sha256":"6a1771f082701da20ee6580b8dde7389d7504c1c5a61dc0f131197838f5fc934","target_id":"CLM-017"} -->
##### Prerequisites

| Prerequisite | Source |
|---|---|
| Current deliverable context and register rows for DEL-13-02, SOW-067, SOW-068, OBJ-014, and OBJ-018 | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv`; `docs/_Registers/ScopeLedger.csv`; `docs/_Registers/ContextBudgetQA.csv` |
| Accepted revision 0.7 current decomposition basis for PKG-13 and DEL-13-02 | `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| Project invariants for protected data, missing-data findings, provenance, units, and professional boundaries | `INIT.md`; `docs/CONTRACT.md`; `docs/SPEC.md`; `docs/IP_AND_DATA_BOUNDARY.md` |
| Active predecessor context from approved DAG-006 mirror | `_DEPENDENCIES.md`; `Dependencies.csv` |
| Architecture basis: Rust core/application services, JSON Schema 2020-12, schema-first envelopes, canonical JSON/JCS-compatible hash basis where JSON payloads are hashed | `_CONTEXT.md#Architecture Basis Injection` |
| Implemented constraint schema and focused structural test | `schemas/constraint.schema.json`; `tests/test_constraint_schema.py` |

<!-- sow-source-end -->

### CLM-018 — Steps

<!-- sow-source-begin {"disposition":"PRESERVED","file":"Procedure.md","line_end":41,"line_start":27,"source_sha256":"6a1771f082701da20ee6580b8dde7389d7504c1c5a61dc0f131197838f5fc934","target_id":"CLM-018"} -->
##### Steps

1. Confirm the deliverable identity remains DEL-13-02 in PKG-13 and the intended artifact remains `schemas/constraint.schema.json` plus a constraint provenance model.
2. Enumerate only the source-grounded constraint categories named by SOW-068: connectivity, route conflicts, clearance conflicts, support-zone conflicts, slope/drain/vent conflicts, and missing required data.
3. Enumerate only the source-grounded related design knowledge categories named by SOW-067: endpoints, line data, routing corridors, no-go volumes, supportable zones, equipment interfaces, access constraints, slope/drain/vent requirements, and owner/project metadata.
4. Inspect `schemas/constraint.schema.json` for the implemented root properties: `schema_version`, `deliverable_id`, `package_id`, `scope_items`, `objectives`, `data_boundary`, and `constraint_set`.
5. Confirm the schema identity: `$schema` = `https://json-schema.org/draft/2020-12/schema`, `$id` = `https://openpipestress.org/schemas/constraint.schema.json`, and `schema_version` matching `^[0-9]+\\.[0-9]+\\.[0-9]+$`.
6. Confirm the implemented `ConstraintSet`, `ConstraintRecord`, `ConstraintKind`, `Diagnostic`, `Parameter`, `Quantity`, `Provenance`, `Reference`, `ValidationStatus`, and `ProfessionalBoundary` definitions are present.
7. Ensure every unit-bearing physical quantity in the model is represented through the implemented `Quantity` contract with explicit `unit`, `dimension`, and `provenance`.
8. Include provenance capacity for user/project/import/agent/source-derived origin where known; represent unknown or unresolved provenance explicitly through the schema's `TBD`, review, privacy, and redistribution statuses.
9. Ensure missing required data is represented as a finding, diagnostic, assumption, or validation status, not a silent default.
10. Exclude protected standards text, protected tables, owner standards, proprietary project data, code-specific values, and professional approval/code-compliance statuses from public schema artifacts and examples.
11. Record traceability from schema content back to SOW-067, SOW-068, OBJ-014, OBJ-018, and applicable governance invariants.
12. Defer validation-engine behavior, user-interface behavior, physical-to-analytical transform consumption, public example payload policy, and acceptance tests that depend on downstream runtime behavior to later bounded work unless a human-approved brief expands this deliverable.

<!-- sow-source-end -->

### CLM-019 — Verification

<!-- sow-source-begin {"disposition":"PRESERVED","file":"Procedure.md","line_end":55,"line_start":42,"source_sha256":"6a1771f082701da20ee6580b8dde7389d7504c1c5a61dc0f131197838f5fc934","target_id":"CLM-019"} -->
##### Verification

| Check | Expected result |
|---|---|
| Scope check | Schema/model content maps to SOW-067 and SOW-068 without adding unapproved protected or owner-specific requirements. |
| Provenance check | Constraint records can identify known user/project/import/agent/source provenance or explicitly mark unknown provenance. |
| Missing-data check | Required missing information appears as diagnostics/findings, not defaults. |
| Unit check | Unit-bearing values are unit-aware or blocked as `TBD`. |
| Professional-boundary check | No schema field or fixture asserts software-generated approval, certification, sealing, authentication, or code compliance. |
| Data-boundary check | Public artifacts contain no protected standards text, protected tables, proprietary catalog data, private owner data, or code-specific values. |
| Dependency mirror check | Current DAG-006 predecessor context and preserved historical DAG-002 local mirror rows remain unchanged if dependency extraction is run in this folder. |
| JSON syntax check | `python3 -m json.tool schemas/constraint.schema.json` completes successfully. |
| Schema structural test | `python3 tests/test_constraint_schema.py` completes successfully. |

<!-- sow-source-end -->

### CLM-020 — Records

<!-- sow-source-begin {"disposition":"PRESERVED","file":"Procedure.md","line_end":62,"line_start":56,"source_sha256":"6a1771f082701da20ee6580b8dde7389d7504c1c5a61dc0f131197838f5fc934","target_id":"CLM-020"} -->
##### Records

- Updated `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`.
- Updated `_REFERENCES.md`, `MEMORY.md`, and a TASK run record when an evidence-refresh pass is performed.
- Existing lifecycle state in `_STATUS.md` is not changed by an evidence-refresh pass unless a human explicitly approves a lifecycle gate.
- Implemented schema artifact: `schemas/constraint.schema.json` or human-approved successor path.
- Current verification evidence: `python3 -m json.tool schemas/constraint.schema.json`, `python3 tests/test_constraint_schema.py`, and `git diff --check` when feasible.
<!-- sow-source-end -->

- **VER-001** — Validate the candidate schema and claim map; confirm parity and source/status hashes; run the focused constraint-schema and dependency checks; and independently review source grounding and authority boundaries.

## Governing Values and Decisions — Axiology

### CLM-021 — Guidance: DEL-13-02 Constraint entity and provenance model

<!-- sow-source-begin {"disposition":"PRESERVED","file":"Guidance.md","line_end":2,"line_start":1,"source_sha256":"ffd2f5630ad767b05f55a817f7dceffa1ab00209912ae355a99e10106ef0641a","target_id":"CLM-021"} -->
#### Guidance: DEL-13-02 Constraint entity and provenance model

<!-- sow-source-end -->

### CLM-022 — Purpose

<!-- sow-source-begin {"disposition":"PRESERVED","file":"Guidance.md","line_end":6,"line_start":3,"source_sha256":"ffd2f5630ad767b05f55a817f7dceffa1ab00209912ae355a99e10106ef0641a","target_id":"CLM-022"} -->
##### Purpose

DEL-13-02 exists to make physical design constraints explicit, traceable, and reviewable before a later validation engine evaluates available design knowledge. The deliverable supports OBJ-014 by contributing constraint records to the schema-backed piping design model, and it supports OBJ-018 by preserving IP and professional-boundary limits across constraint data.

<!-- sow-source-end -->

### CLM-023 — Principles

<!-- sow-source-begin {"disposition":"PRESERVED","file":"Guidance.md","line_end":16,"line_start":7,"source_sha256":"ffd2f5630ad767b05f55a817f7dceffa1ab00209912ae355a99e10106ef0641a","target_id":"CLM-023"} -->
##### Principles

| Principle | Guidance | Source |
|---|---|---|
| Represent findings, not hidden defaults | Missing data and unmet constraints should surface as explicit findings or diagnostics with provenance. Do not silently supply engineering values. | `INIT.md`; `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md#4.3 Analysis status and authority boundary` |
| Keep design knowledge user/project supplied | Constraint categories may refer to owner/project design knowledge, but public artifacts must not bundle owner standards, protected code criteria, proprietary project data, or code-specific values. | SOW-067; `docs/IP_AND_DATA_BOUNDARY.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#PKG-13` |
| Preserve provenance | Constraint records should identify where known information came from, including user, project, import, agent, or source provenance. Unknown provenance should remain visible as `TBD`, not normalized away. | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv`; `docs/CONTRACT.md` OPS-K-IP-2, OPS-K-DATA-3 |
| Separate schema from validation engine | This deliverable defines representation and provenance. Deterministic validation behavior belongs to DEL-13-03. | `execution/_Decomposition/SOFTWARE_DECOMP.md#PKG-13` |
| Preserve professional responsibility | Constraint records and validation messages support review but must not become automatic professional approval or code-compliance certification. | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/SPEC.md#4.3 Analysis status and authority boundary` |

<!-- sow-source-end -->

### CLM-024 — Considerations

<!-- sow-source-begin {"disposition":"PRESERVED","file":"Guidance.md","line_end":25,"line_start":17,"source_sha256":"ffd2f5630ad767b05f55a817f7dceffa1ab00209912ae355a99e10106ef0641a","target_id":"CLM-024"} -->
##### Considerations

- Constraint categories are source-grounded by SOW-068 and implemented as `ConstraintKind`: `connectivity`, `clearance`, `no_go_volume`, `support_zone`, `route_conflict`, `slope`, `drain`, `vent`, `access`, `equipment_interface`, `missing_required_data`, and `TBD`.
- Design-knowledge associations are source-grounded by SOW-067 and represented through typed `Reference` objects plus `design_knowledge_refs`; runtime interpretation of those references remains downstream.
- Unit-bearing quantities referenced by constraints should not bypass the canonical unit contract. The implemented `Quantity` definition requires `value`, `unit`, `dimension`, and `provenance`, and the test fixes the dimension enum to the accepted PKG-02 vocabulary.
- Provenance should distinguish known facts from unresolved assumptions or imported claims. The implemented `Provenance` definition requires source, contributor, redistribution, review, and privacy-classification fields.
- The approved dependency mirror and current DAG-006 coordination basis indicate upstream architecture, canonical model, unit, persistence, design knowledge, and professional-boundary context. They are evidence for predecessor context, not authority to reclassify dependencies.
- Runtime constraint validation, GUI presentation/blocking behavior, physical-to-analytical transform consumption, and actual public example payload policy remain `TBD`.

<!-- sow-source-end -->

### CLM-025 — Trade-offs

<!-- sow-source-begin {"disposition":"PRESERVED","file":"Guidance.md","line_end":34,"line_start":26,"source_sha256":"ffd2f5630ad767b05f55a817f7dceffa1ab00209912ae355a99e10106ef0641a","target_id":"CLM-025"} -->
##### Trade-offs

| Topic | Conservative position |
|---|---|
| Category breadth | Include the categories explicitly named by SOW-068; defer additional categories to human-approved scope or later decomposition change. |
| Example payloads | Prefer no examples until invented/public-permissive examples are reviewed. Do not use real owner/project standards or protected code examples. |
| Validation detail | Use the implemented diagnostic and validation-status slots, but avoid encoding DEL-13-03 engine behavior as if already implemented. |
| Professional status | Use diagnostics and review-needed findings; avoid approval/certification/compliance statuses. |

<!-- sow-source-end -->

### CLM-026 — Examples

<!-- sow-source-begin {"disposition":"PRESERVED","file":"Guidance.md","line_end":38,"line_start":35,"source_sha256":"ffd2f5630ad767b05f55a817f7dceffa1ab00209912ae355a99e10106ef0641a","target_id":"CLM-026"} -->
##### Examples

No source-grounded example payload is available in the local references. The schema includes `data_boundary.public_examples_policy = invented_or_cleared_data_only`, but example records and any public example publication policy remain `TBD` and must be invented or otherwise cleared for redistribution before inclusion in public artifacts.

<!-- sow-source-end -->

### CLM-027 — Conflict Table (for human ruling)

<!-- sow-source-begin {"disposition":"PRESERVED","file":"Guidance.md","line_end":43,"line_start":39,"source_sha256":"ffd2f5630ad767b05f55a817f7dceffa1ab00209912ae355a99e10106ef0641a","target_id":"CLM-027"} -->
##### Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| NONE | No direct source conflict detected in the accessible DEL-13-02 context. Remaining `TBD` items are downstream runtime validation, GUI behavior, transform consumption, and public examples. | `_CONTEXT.md`; `_REFERENCES.md`; `Dependencies.csv`; decomposition/register slices; `schemas/constraint.schema.json`; `tests/test_constraint_schema.py` | N/A | N/A | N/A | N/A |
<!-- sow-source-end -->

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-068 SOW-067 OBJ-014 OBJ-018 | CLM-007 | AC-001 | VER-001 | Claim map, parity report, and applicable verification evidence |

<!-- pilot-variance: D-GOV-15@58aa81d62f4a32e3c2d687e4356a1e4be8141674 -->
