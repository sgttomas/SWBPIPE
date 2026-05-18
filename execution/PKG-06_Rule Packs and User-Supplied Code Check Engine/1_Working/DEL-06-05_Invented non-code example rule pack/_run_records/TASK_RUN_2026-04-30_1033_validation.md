# TASK RUN: local validation

| Field | Value |
|---|---|
| Deliverable | DEL-06-05 Invented non-code example rule pack |
| Package | PKG-06 Rule Packs and User-Supplied Code Check Engine |
| Generated | 2026-04-30 |
| Status | PASS with warning |

## Commands and Results

| Command | Result |
|---|---|
| `tools/validation/check_four_documents.sh <DELIVERABLE_PATH>` | PASS: all four document kit files present |
| `python3 tools/validation/validate_dependencies_schema.py <DELIVERABLE_PATH>/Dependencies.csv` | PASS: 29 required columns and 7 data rows |
| `python3 tools/validation/validate_enum.py <ENUM_NAME> <VALUE>` for emitted enum values | PASS |
| `rg -c '^\\| [ABCFDXE]:' <DELIVERABLE_PATH>/_SEMANTIC_LENSING.md` | PASS: 96 lens coverage rows |
| `rg -n 'Σ|∩| \\+ ' <DELIVERABLE_PATH>/_SEMANTIC.md` | PASS: no semantic algebra leak matches |
| `git status --short -- examples/rule_packs examples/rule_packs/invented_demo.yaml` | PASS: no repo-level rule-pack example changes |
| `awk` evidence/source checks over `Dependencies.csv` | PASS: no missing evidence fields reported |
| `awk` dependency ID uniqueness check over `Dependencies.csv` | PASS: no duplicate IDs reported |
| `tools/validation/validate_id_format.sh PKG PKG-06`, `DEL DEL-06-05`, `SOW SOW-016` | WARNING: helper expects legacy longer ID formats that conflict with current project registers |

## Protected-Data and Professional-Boundary Scan

Targeted search found only prohibition/boundary language and the existing `_CONTEXT.md` package exclusion mentioning ASME. No protected standards data, code tables, copied formulas, engineering allowables, SIF/flexibility tables, proprietary values, or certification claim was introduced.

## Status Check

- `_STATUS.md` current state: `SEMANTIC_READY`.
- `SEMANTIC_READY` is retained because setup artifacts exist and local gates passed, with the ID-format helper mismatch recorded as a project-tool warning rather than a deliverable artifact defect.
