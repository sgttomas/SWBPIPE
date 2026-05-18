# Dependencies: DEL-03-03 Bend and elbow component model fields

## TP-DAG-004 Refresh
- **Status:** REFRESHED_FOR_RECONCILIATION
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer Context:** RECONCILIATION
- **Local Register:** `Dependencies.csv`
- **Rows:** 11 total; 11 ACTIVE; 0 RETIRED.
- **Refreshed:** 2026-05-10
- **Run Record:** `_run_records/TASK_RUN_2026-05-10_2155_dependency-extract.md`

## Authority Boundary
- This local register is an evidence surface for reconciliation, not an aggregate DAG authority.
- No aggregate DAG, lifecycle, status, source, memory, code, schema, test, or coordination file was edited.
- `PKG-00` architecture-basis rows are preserved as injected context evidence; `PKG-00` does not receive local dependency registers.
- Conservative predecessor rows are retained only where supported by assigned context, local deliverable evidence, contract invariants, or `execution/_Decomposition/SOFTWARE_DECOMP.md`.

## Refresh Notes
- Added/restored one enum-valid `IMPLEMENTS_NODE` anchor for `SOW-007`.
- Preserved the objective trace to `OBJ-004`.
- Normalized six SCA-001 architecture-basis edges to v3.1 enum values using `CONSTRAINT`, `NOT_APPLICABLE`, `EXPLICIT`, and `EXTRACTED`.
- Retained three conservative upstream prerequisites: `DEL-03-02`, `DEL-02-02`, and `DEL-01-02`.
- Left prerequisite satisfaction as `TBD` where this bounded worker did not inspect target deliverable folders.

## Validation Closeout
- `python3 tools/validation/validate_dependencies_schema.py <Dependencies.csv>`: PASS.
- Enum validation using `tools/validation/validate_enum.py`: PASS for `DependencyClass`, `AnchorType`, `Direction`, `DependencyType`, `TargetType`, `Explicitness`, `SatisfactionStatus`, `Confidence`, `Origin`, and `Status` across all rows.
