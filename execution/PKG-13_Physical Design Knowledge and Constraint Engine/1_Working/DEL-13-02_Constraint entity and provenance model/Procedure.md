# Procedure: DEL-13-02 Constraint entity and provenance model

## Purpose

Provide a bounded procedure for producing and checking the DEL-13-02 constraint entity and provenance model artifacts without crossing the data boundary or claiming implementation evidence that does not yet exist.

## Prerequisites

| Prerequisite | Source |
|---|---|
| Current deliverable context and register rows for DEL-13-02, SOW-067, SOW-068, OBJ-014, and OBJ-018 | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv`; `docs/_Registers/ScopeLedger.csv`; `docs/_Registers/ContextBudgetQA.csv` |
| Accepted revision 0.5 decomposition basis for PKG-13 and DEL-13-02 | `execution/_Decomposition/SOFTWARE_DECOMP.md` |
| Project invariants for protected data, missing-data findings, provenance, units, and professional boundaries | `INIT.md`; `docs/CONTRACT.md`; `docs/SPEC.md`; `docs/IP_AND_DATA_BOUNDARY.md` |
| Active predecessor context from approved DAG-002 mirror | `_DEPENDENCIES.md`; `Dependencies.csv` |
| Architecture basis: Rust core/application services, JSON Schema 2020-12, schema-first envelopes, canonical JSON/JCS-compatible hash basis where JSON payloads are hashed | `_CONTEXT.md#Architecture Basis Injection` |

## Steps

1. Confirm the deliverable identity remains DEL-13-02 in PKG-13 and the intended artifact remains `schemas/constraint.schema.json` plus a constraint provenance model.
2. Enumerate only the source-grounded constraint categories named by SOW-068: connectivity, route conflicts, clearance conflicts, support-zone conflicts, slope/drain/vent conflicts, and missing required data.
3. Enumerate only the source-grounded related design knowledge categories named by SOW-067: endpoints, line data, routing corridors, no-go volumes, supportable zones, equipment interfaces, access constraints, slope/drain/vent requirements, and owner/project metadata.
4. Define the constraint record shape as `TBD` fields until implementation work resolves exact property names, enum spellings, identifiers, schema `$id`, and versioning fields.
5. Ensure every unit-bearing physical quantity in the model is represented through an explicit unit-aware contract or marked `TBD` pending unit-contract integration.
6. Include provenance capacity for user/project/import/agent/source origin where known; represent unknown or unresolved provenance explicitly.
7. Ensure missing required data is represented as a finding or diagnostic, not a silent default.
8. Exclude protected standards text, protected tables, owner standards, proprietary project data, code-specific values, and professional approval/code-compliance statuses from public schema artifacts and examples.
9. Record traceability from schema content back to SOW-067, SOW-068, OBJ-014, OBJ-018, and applicable governance invariants.
10. Defer validation-engine behavior, user-interface behavior, final example fixtures, and acceptance tests that depend on implementation details to later bounded work unless a human-approved brief expands this deliverable.

## Verification

| Check | Expected result |
|---|---|
| Scope check | Schema/model content maps to SOW-067 and SOW-068 without adding unapproved protected or owner-specific requirements. |
| Provenance check | Constraint records can identify known user/project/import/agent/source provenance or explicitly mark unknown provenance. |
| Missing-data check | Required missing information appears as diagnostics/findings, not defaults. |
| Unit check | Unit-bearing values are unit-aware or blocked as `TBD`. |
| Professional-boundary check | No schema field or fixture asserts software-generated approval, certification, sealing, authentication, or code compliance. |
| Data-boundary check | Public artifacts contain no protected standards text, protected tables, proprietary catalog data, private owner data, or code-specific values. |
| Dependency mirror check | Existing DAG-002 rows remain preserved as ACTIVE evidence if dependency extraction is run in this folder. |

## Records

- Updated `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`.
- Updated lifecycle state in `_STATUS.md` when the four-document pass safely transitions `OPEN` to `INITIALIZED`.
- Future schema artifact: `schemas/constraint.schema.json` or human-approved successor path.
- Future verification evidence for schema validation, provenance review, data-boundary review, unit compatibility, and professional-boundary review.
