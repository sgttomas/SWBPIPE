# Run Record: setup validation

| Field | Value |
|---|---|
| Deliverable | DEL-08-03 |
| Date | 2026-04-30 |
| Actor | TASK |
| Run Status | PASS_WITH_TOOLING_WARNING |

## Validation Commands and Results

| Command | Result |
|---|---|
| `./tools/validation/check_four_documents.sh <deliverable>` | PASS: all 4 document kit files present |
| `rg -c '^\\| [ABCFDXE]:' <deliverable>/_SEMANTIC_LENSING.md` | PASS: 96 lens coverage rows |
| `python3 tools/validation/validate_dependencies_schema.py <deliverable>/Dependencies.csv` | PASS: 29 required columns, 12 data rows |
| `python3 tools/validation/validate_enum.py ...` for all unique dependency enum values | PASS |
| `./tools/validation/validate_id_format.sh PKG PKG-08` | WARNING: helper expects old `PKG-000` pattern |
| `./tools/validation/validate_id_format.sh DEL DEL-08-03` | WARNING: helper expects old `DEL-000-00` pattern |
| `./tools/validation/validate_id_format.sh DEP DEP-08-03-001` | WARNING: helper expects old `DEP-000-00-000` pattern |
| `./tools/validation/validate_id_format.sh SOW SOW-024` | WARNING: helper expects old `SOW-0000` pattern |
| `./tools/validation/validate_id_format.sh OBJ OBJ-007` | PASS |
| `find execution/PKG-08_Reporting, Audit, and Reproducibility/3_Issued -maxdepth 2 -type f` | PASS: no DEL-08-03 artifact moved to ISSUED |

## Boundary Checks

- Write scope remained inside `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-03_Warnings, assumptions, and provenance report section/`.
- No report source code, templates outside the deliverable, tests, schemas, or repository-level artifacts were edited.
- Protected standards content was not introduced; references to protected content are guardrail/prohibition language only.
- No certification, approval, sealing, endorsement, authentication, or automatic code-compliance claim was introduced.

## Open Tooling Issue

The dependency ID-format helper is stale relative to `docs/TYPES.md` and the current registers. This setup preserves canonical project IDs and records the validator mismatch as a tooling warning.

