# Procedure: DEL-03-07 Public/private library import provenance checker

## Purpose

Define the setup-stage procedure for producing and checking the future library import provenance checker without implementing product code in this pass.

## Prerequisites

- Sealed context for DEL-03-07.
- `_CONTEXT.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, and `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4.
- Applicable invariants from `docs/CONTRACT.md`.
- Human-approved decisions for any license/redistribution acceptance vocabulary before product implementation finalizes those fields.

## Steps

1. Confirm the import record type being checked is within component/material library scope.
2. Confirm the record carries source, provenance, license, redistribution-status, contributor, and review-disposition metadata fields or emits an explicit missing-field diagnostic.
3. Confirm public imports with missing provenance or missing redistribution status are blocked, rejected, or marked not publicly acceptable.
4. Confirm private imports remain marked private/local and do not become public bundled data by default.
5. Confirm suspected protected content produces a quarantine/escalation diagnostic without copying or paraphrasing protected content into public data.
6. Confirm numeric imported values preserve unit metadata where numeric component/material values are present.
7. Confirm result diagnostics fit the schema-first command/query/job result-envelope pattern and expose unresolved items as `TBD` or review-needed statuses.
8. Write provenance tests using invented fixtures only.

## Verification

| Check | Expected evidence |
|---|---|
| Metadata completeness | Tests for accepted records and missing-field diagnostics. |
| Public/private boundary | Tests that private records remain private and public records require provenance metadata. |
| Protected-content response | Tests assert quarantine/escalation status using invented markers only. |
| Unit handling | Tests assert unit fields are preserved for invented numeric values. |
| No legal conclusions | Tests assert unresolved rights questions remain review-needed/TBD. |
| No protected examples | Fixture review confirms no standards tables, vendor data, or real protected examples are included. |

## Records

- `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` setup kit.
- `_SEMANTIC.md` semantic matrix lens.
- `_SEMANTIC_LENSING.md` lensing register.
- `Dependencies.csv` v3.1 and `_DEPENDENCIES.md`.
- `_run_records/TASK_RUN_*.md` records for the required setup sequence.
