# Procedure: DEL-03-07 Public/private library import provenance checker

## Purpose

Define the procedure for checking and maintaining the implemented library import provenance checker while preserving unresolved policy, parser, review, and lifecycle boundaries.

## Prerequisites

- Sealed context for DEL-03-07.
- `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, and `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7.
- Applicable invariants from `docs/CONTRACT.md`.
- Implementation evidence in `core/library_import/provenance_checker.py`, `core/library_import/README.md`, and `tests/test_library_import_provenance.py`.
- Human-approved decisions for license/redistribution acceptance vocabulary, public source catalogs, and import parser contracts before those fields are treated as project policy.

## Steps

1. Confirm the import record type being checked is within material, section, or component library scope.
2. Confirm the record carries source, provenance, license, redistribution-status, contributor, and review-disposition metadata fields or emits an explicit missing-field diagnostic.
3. Confirm public imports with missing provenance or missing redistribution status are blocked, rejected, or marked not publicly acceptable.
4. Confirm private imports remain marked private/local and do not become public bundled data by default.
5. Confirm suspected protected content produces a quarantine/escalation diagnostic without copying or paraphrasing protected content into public data.
6. Confirm numeric imported values preserve unit metadata where numeric component/material values are present.
7. Confirm result diagnostics fit the schema-first command/query/job result-envelope pattern and expose unresolved items as `TBD` or review-needed statuses.
8. Confirm provenance tests use invented fixtures only and do not establish engineering reliance for fixture values.
9. Confirm review findings remain technically addressed pending human disposition unless an authorized human review updates them.

## Verification

| Check | Expected evidence |
|---|---|
| Metadata completeness | Tests for accepted records and missing-field diagnostics. |
| Public/private boundary | Tests that private records remain private and public records require provenance metadata. |
| Protected-content response | Tests assert quarantine/escalation status using invented markers only. |
| Unit handling | Tests assert unit fields are preserved for invented numeric values. |
| No legal conclusions | Tests assert unresolved rights questions remain review-needed/TBD. |
| No protected examples | Fixture review confirms no standards tables, vendor data, or real protected examples are included. |
| Review disposition boundary | Local review findings remain pending human disposition; no CSV status promotion is made by this procedure. |

## Records

- `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`.
- `core/library_import/provenance_checker.py`, `core/library_import/README.md`, and `tests/test_library_import_provenance.py` as implementation evidence.
- `_SEMANTIC.md` semantic matrix lens.
- `_SEMANTIC_LENSING.md` lensing register.
- `Dependencies.csv` v3.1 and `_DEPENDENCIES.md`.
- `_run_records/TASK_RUN_*.md` records for the deliverable work sequence.
