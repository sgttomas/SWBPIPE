---
run-status: SUCCESS_WITH_WARNINGS
deliverable-id: DEL-01-01
package-id: PKG-01
task-skill: dependency-extract
scope: DEL-01-01
mode: UPDATE
strictness: CONSERVATIVE
scope-path: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-01_Governance, IP Boundary, and Professional Responsibility/1_Working/DEL-01-01_Project governance baseline
---

# TASK Run Record — dependency-extract

## Results

- Created `Dependencies.csv` v3.1 with 10 ACTIVE rows.
- Refreshed `_DEPENDENCIES.md`.
- Active rows: 5 ANCHOR, 5 EXECUTION.
- Parent anchor count: 1.

## QA

- `python3 tools/validation/validate_dependencies_schema.py .../Dependencies.csv`: VALID, 29 required columns, 10 data rows.
- `python3 tools/validation/validate_enum.py`: all unique emitted enum values validated.
- `tools/validation/validate_id_format.sh DEL DEL-01-01`: failed because the helper expects `DEL-NNN-NN`.
- `tools/validation/validate_id_format.sh PKG PKG-01`: failed because the helper expects `PKG-NNN`.

## Warning

- Stable software IDs were preserved as `DEL-01-01` and `PKG-01` because those are the IDs in `docs/TYPES.md`, `_CONTEXT.md`, and the registers. The ID-format helper appears aligned to a legacy three-digit PROJECT pattern.

## Missing Inputs

- No human-declared dependency DAG was provided.
