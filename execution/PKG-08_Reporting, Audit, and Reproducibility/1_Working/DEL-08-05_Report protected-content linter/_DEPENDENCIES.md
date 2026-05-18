# Dependencies: DEL-08-05 Report protected-content linter

## TP-DAG-004 Dependency-Extract Refresh

- **Status:** REFRESHED_FOR_RECONCILIATION
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer Context:** RECONCILIATION
- **Local Register:** `Dependencies.csv`
- **Rows:** 13 total; 13 ACTIVE; 0 RETIRED.
- **Generated:** 2026-05-10

## Row Summary

| Class | Count | Notes |
|---|---:|---|
| ANCHOR | 3 | Local coverage anchors for `SOW-043`, `OBJ-002`, and `OBJ-007`; includes one `IMPLEMENTS_NODE` row. |
| EXECUTION | 10 | Seven architecture-basis constraints, two governance predecessors, and one report-generator interface predecessor. |

## Dependency Surface Notes

- This refresh replaces the prior synchronized `DAG-002` mirror summary with a DEL-local dependency-extract evidence surface.
- Architecture-basis rows remain context/evidence constraints only; they do not mark `PKG-00` deliverables as `ISSUED` and are not independent Type 2 dispatch authority.
- Upstream target satisfaction for `DEL-01-02`, `DEL-01-04`, and `DEL-08-01` remains `PENDING` because this bounded worker did not inspect or modify target deliverables.
- No candidate rows were promoted, no lifecycle state was changed, and no aggregate DAG, source, status, memory, code, schema, test, or coordination file was edited.

## Reconciliation Handoff

- `Dependencies.csv` uses v3.1 required columns only.
- Enum values were normalized to the local validator sets:
  - `DependencyClass`: `ANCHOR`, `EXECUTION`
  - `AnchorType`: `IMPLEMENTS_NODE`, `TRACES_TO_REQUIREMENT`, `NOT_APPLICABLE`
  - `Direction`: `UPSTREAM`
  - `DependencyType`: `OTHER`, `CONSTRAINT`, `PREREQUISITE`, `INTERFACE`
  - `TargetType`: `REQUIREMENT`, `DELIVERABLE`
  - `SatisfactionStatus`: `SATISFIED`, `PENDING`
  - `Status`: `ACTIVE`
- This local register is evidence for later RECONCILIATION. It does not approve `DAG-003` or change graph authority from approved `DAG-002`.
