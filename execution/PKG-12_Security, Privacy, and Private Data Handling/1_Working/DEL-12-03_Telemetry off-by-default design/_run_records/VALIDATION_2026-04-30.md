# Run Record: Validation

| Field | Value |
|---|---|
| DeliverableID | DEL-12-03 |
| PackageID | PKG-12 |
| Date | 2026-04-30 |
| Run status | SUCCESS |

## Command Results

| Check | Command | Result |
|---|---|---|
| Four documents exist | `tools/validation/check_four_documents.sh <ScopePath>` | PASS: all 4 document kit files present |
| Dependencies schema | `python3 tools/validation/validate_dependencies_schema.py <ScopePath>/Dependencies.csv` | VALID: 29 required columns, 9 data rows |
| Semantic matrix markers | `rg -n "MatrixError\|MATRIX_ERROR" <ScopePath>/_SEMANTIC.md` | PASS: no matches |
| Semantic result operators | Python scan of `_SEMANTIC.md` result tables for `Σ`, `∩`, or ` + ` | PASS: 8 result tables scanned, 0 leaks |
| Lensing coverage | Python count of `_SEMANTIC_LENSING.md` coverage rows | PASS: 96 rows; A=12, B=16, C=12, F=12, D=12, X=16, E=16 |
| Lifecycle state | `rg -n "**Current State:**" <ScopePath>/_STATUS.md` | PASS: `SEMANTIC_READY` |
| Dependency integrity | Python uniqueness/evidence/anchor check over `Dependencies.csv` | PASS: 9 unique IDs, 9 active rows, 1 parent anchor, evidence present |

## Notes

- `_STATUS.md` is `SEMANTIC_READY`, not `ISSUED`.
- No validation command required network access.
- All changed files are inside the assigned deliverable folder.
