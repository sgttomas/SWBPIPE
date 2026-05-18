# Specification: DEL-13-02 Constraint entity and provenance model

## Scope

This deliverable specifies the data-model contract for representing constraint entities and their provenance for the PKG-13 physical design knowledge and constraint engine. It covers constraint records for connectivity, route conflicts, clearance conflicts, no-go/support-zone conflicts, slope/drain/vent conflicts, and missing required data, using the design knowledge categories named by SOW-067 and SOW-068.

This deliverable does not implement the constraint validation engine, infer hidden owner standards, define protected code requirements, bundle proprietary project data, create final engineering acceptance logic, or claim professional approval. Those exclusions are grounded in the PKG-13 decomposition notes, `INIT.md`, `docs/CONTRACT.md`, and `docs/IP_AND_DATA_BOUNDARY.md`.

## Requirements

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
| R-13-02-010 | Exact property names, enum spellings, schema `$id`, versioning fields, and example payload contents are `TBD` until implementation-level schema design. | ASSUMPTION based on DATA_MODEL_CHANGE type and current absence of `schemas/constraint.schema.json` in this deliverable folder |

## Standards

| Standard or basis | Status for this deliverable |
|---|---|
| JSON Schema 2020-12 | Applicable schema basis from architecture injection and `docs/TYPES.md`; exact `$schema` declaration remains implementation `TBD`. |
| Project IP/data-boundary policy | Governing source for public/private data handling and protected-content exclusions. |
| Project invariant catalog | Governing source for provenance, unit safety, missing-data findings, and professional-boundary requirements. |
| Owner standards and engineering codes | Not bundled or paraphrased here. Any private owner/code requirements remain user-supplied or privately imported with provenance and review status. |

## Verification

| Requirement IDs | Verification approach |
|---|---|
| R-13-02-001, R-13-02-002 | Schema review confirms the model has slots for the in-scope constraint and design-knowledge categories or records `TBD` for unresolved category modeling. |
| R-13-02-003 | Schema review confirms provenance fields or references can represent known user/project/import/agent/source origin. |
| R-13-02-004 | Tests or schema examples confirm missing required data is represented as a finding/diagnostic, not defaulted. |
| R-13-02-005 | Schema validation confirms unit-bearing values reference a unit-aware quantity model or explicitly dimensionless classification. |
| R-13-02-006 | Protected-content and data-boundary review confirms no protected standards data, proprietary owner data, or code-specific default values are present. |
| R-13-02-007 | Schema and fixture review confirms no automatic professional approval, certification, sealing, authentication, or code-compliance status is emitted. |
| R-13-02-008 | JSON Schema validator confirms schema dialect compatibility once the schema exists. |
| R-13-02-009 | Model-integration review confirms typed references, diagnostics/warnings, unresolved assumptions, and traceability links are compatible with the physical source-of-truth model contract. |

## Documentation

Required deliverable documentation and evidence:

- `schemas/constraint.schema.json` or a human-approved successor path.
- Constraint provenance model notes or schema section.
- Traceability from schema elements to SOW-067, SOW-068, OBJ-014, and OBJ-018.
- Data-boundary notes for any examples or fixtures.
- Verification notes for JSON Schema validation, unit/provenance compatibility, missing-data diagnostics, and professional-boundary review.
