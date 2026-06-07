# Specification: DEL-13-02 Constraint entity and provenance model

## Scope

This deliverable specifies the data-model contract for representing constraint entities and their provenance for the PKG-13 physical design knowledge and constraint engine. It covers constraint records for connectivity, route conflicts, clearance conflicts, no-go/support-zone conflicts, slope/drain/vent conflicts, and missing required data, using the design knowledge categories named by SOW-067 and SOW-068.

This deliverable does not implement the constraint validation engine, infer hidden owner standards, define protected code requirements, bundle proprietary project data, create final engineering acceptance logic, or claim professional approval. Those exclusions are grounded in the PKG-13 decomposition notes, `INIT.md`, `docs/CONTRACT.md`, and `docs/IP_AND_DATA_BOUNDARY.md`.

## Implementation Evidence

The current implementation evidence is `schemas/constraint.schema.json` and `tests/test_constraint_schema.py`. The schema declares JSON Schema 2020-12, `$id` `https://openpipestress.org/schemas/constraint.schema.json`, root constants for `DEL-13-02` and `PKG-13`, and required root properties for `schema_version`, `deliverable_id`, `package_id`, `scope_items`, `objectives`, `data_boundary`, and `constraint_set`.

The schema defines reusable `$defs` for `ConstraintSet`, `ConstraintRecord`, `ConstraintKind`, `Diagnostic`, `Parameter`, `Quantity`, `Provenance`, `Reference`, `ValidationStatus`, and `ProfessionalBoundary`. The accompanying stdlib test checks schema identity, required definitions, required fields, enum coverage, PKG-02 dimension vocabulary alignment, protected-text exclusions, data-boundary constants, and professional-boundary constants.

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
| R-13-02-010 | The implemented schema shall declare stable schema identity and a version slot: `$schema` = `https://json-schema.org/draft/2020-12/schema`, `$id` = `https://openpipestress.org/schemas/constraint.schema.json`, and required `schema_version` matching `^[0-9]+\\.[0-9]+\\.[0-9]+$`. | `schemas/constraint.schema.json`; `tests/test_constraint_schema.py` |

## Standards

| Standard or basis | Status for this deliverable |
|---|---|
| JSON Schema 2020-12 | Implemented by `schemas/constraint.schema.json`; verified by `python3 -m json.tool schemas/constraint.schema.json` and `tests/test_constraint_schema.py`. |
| Project IP/data-boundary policy | Governing source for public/private data handling and protected-content exclusions. |
| Project invariant catalog | Governing source for provenance, unit safety, missing-data findings, and professional-boundary requirements. |
| Owner standards and engineering codes | Not bundled or paraphrased here. Any private owner/code requirements remain user-supplied or privately imported with provenance and review status. |

## Verification

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

## Documentation

Required deliverable documentation and evidence:

- `schemas/constraint.schema.json` or a human-approved successor path.
- Constraint provenance model definitions inside `schemas/constraint.schema.json`.
- `tests/test_constraint_schema.py` as current focused stdlib structural evidence.
- Traceability from schema elements to SOW-067, SOW-068, OBJ-014, and OBJ-018.
- Data-boundary notes for any examples or fixtures.
- Verification notes for JSON syntax validation, unit/provenance compatibility, missing-data diagnostics, and professional-boundary review.
- Public example payloads, GUI presentation behavior, runtime validation behavior, and transform consumption remain `TBD` until later bounded work.
