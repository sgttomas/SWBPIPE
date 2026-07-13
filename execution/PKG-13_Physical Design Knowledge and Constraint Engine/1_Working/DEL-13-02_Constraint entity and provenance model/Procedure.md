# Procedure: DEL-13-02 Constraint entity and provenance model

<!-- D41-R5-T7-PDU055-CURRENTNESS -->
## D-41 R5 T7 PDU-055 current declaration

Current authority is `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.8, approved `execution/_DAG/DAG-007/` graph context, and D-41/`DEC-074` through the completed T1-T6 bounded records. The implemented working-tree slice and its evidence supersede this surface's setup-only, future-only, or overtaken TBD wording as a current declaration; that earlier wording remains historical setup context only.

Surviving deliverable-local residuals and gates are those recorded in `_STATUS.md ## Remaining`; dated MEMORY and formal-review history remain unchanged. This refresh does not imply lifecycle, review, validation, release, professional-reliance, or code-compliance closure.

PDU-055 cited claim(s): `DEL-13-02-DECL-004`.

## Purpose

Provide a bounded procedure for producing and checking the DEL-13-02 constraint entity and provenance model artifacts without crossing the data boundary or claiming implementation evidence that does not yet exist.

## Prerequisites

| Prerequisite | Source |
|---|---|
| Current deliverable context and register rows for DEL-13-02, SOW-067, SOW-068, OBJ-014, and OBJ-018 | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv`; `docs/_Registers/ScopeLedger.csv`; `docs/_Registers/ContextBudgetQA.csv` |
| Accepted revision 0.7 current decomposition basis for PKG-13 and DEL-13-02 | `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| Project invariants for protected data, missing-data findings, provenance, units, and professional boundaries | `INIT.md`; `docs/CONTRACT.md`; `docs/SPEC.md`; `docs/IP_AND_DATA_BOUNDARY.md` |
| Active predecessor context from approved DAG-006 mirror | `_DEPENDENCIES.md`; `Dependencies.csv` |
| Architecture basis: Rust core/application services, JSON Schema 2020-12, schema-first envelopes, canonical JSON/JCS-compatible hash basis where JSON payloads are hashed | `_CONTEXT.md#Architecture Basis Injection` |
| Implemented constraint schema and focused structural test | `schemas/constraint.schema.json`; `tests/test_constraint_schema.py` |

## Steps

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

## Verification

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

## Records

- Updated `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`.
- Updated `_REFERENCES.md`, `MEMORY.md`, and a TASK run record when an evidence-refresh pass is performed.
- Existing lifecycle state in `_STATUS.md` is not changed by an evidence-refresh pass unless a human explicitly approves a lifecycle gate.
- Implemented schema artifact: `schemas/constraint.schema.json` or human-approved successor path.
- Current verification evidence: `python3 -m json.tool schemas/constraint.schema.json`, `python3 tests/test_constraint_schema.py`, and `git diff --check` when feasible.
