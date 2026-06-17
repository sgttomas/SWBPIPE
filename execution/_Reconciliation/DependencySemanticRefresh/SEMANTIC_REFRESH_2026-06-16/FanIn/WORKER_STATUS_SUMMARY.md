# Dependency Semantic Refresh Worker Status Summary

## Scope

- Workflow: ORCHESTRATOR all-register dependency semantic refresh.
- Task skill: `dependency-extract`.
- Mode: `UPDATE`.
- Strictness: `CONSERVATIVE`.
- Deliverable-local registers covered: 93.
- PKG-00 write scope: excluded.
- PKG-00 consistency trackers: retained and reviewed where applicable.

## Fan-In Counts

| Metric | Before | After local refresh | Proposed DAG-007 |
|---|---:|---:|---:|
| Dependency rows | 1289 | 1480 | 1480 |
| Active rows | 1239 | 1425 | 1395 |
| Retired rows | 50 | 55 | 85 |
| Candidate status rows | 0 | 0 | 0 |
| Active `DEL-00-*` target rows | 514 | 577 | 577 |
| Validation failures | 0 | 0 | 0 |

`DAG-007` retires 30 duplicate directed active-edge representatives in the aggregate proposal only. The duplicate rows remain visible in `DAG-007_DuplicateEdgeWorklist.csv`; deliverable-local registers are not rewritten by that aggregate consolidation.

## Package Worker Results

| Package | Deliverables | Rows added | Rows newly retired | PKG-00 tracker result | Validation |
|---|---:|---:|---:|---|---|
| PKG-01 | 4 | 0 | 0 | reviewed and retained | PASS |
| PKG-02 | 5 | 0 | 0 | reviewed and retained | PASS |
| PKG-03 | 8 | 18 | 0 | reviewed; selected rows refined | PASS |
| PKG-04 | 6 | 9 | 0 | reviewed; selected rows refined | PASS |
| PKG-05 | 5 | 0 | 1 | reviewed and retained | PASS |
| PKG-06 | 5 | 46 | 0 | reviewed and retained | PASS |
| PKG-07 | 8 | 15 | 0 | reviewed and retained | PASS |
| PKG-08 | 6 | 14 | 0 | reviewed and retained | PASS |
| PKG-09 | 5 | 3 | 0 | reviewed and retained | PASS |
| PKG-10 | 5 | 5 | 3 | reviewed and retained | PASS |
| PKG-11 | 5 | 0 | 0 | reviewed and retained | PASS |
| PKG-12 | 5 | 0 | 0 | reviewed; 21 tracker rows refined | PASS |
| PKG-13 | 4 | 0 | 0 | reviewed; 28 tracker rows refined | PASS |
| PKG-14 | 5 | 0 | 0 | reviewed and retained | PASS |
| PKG-15 | 4 | 0 | 1 | reviewed and retained | PASS |
| PKG-16 | 4 | 0 | 0 | reviewed and retained | PASS |
| PKG-17 | 9 | 81 | 0 | missing tracker rows added and reviewed | PASS |

## Reconciliation Findings

- Current local core enum drift: zero rows.
- Current local `Status=CANDIDATE`: zero rows.
- Candidate worklist rows emitted into `DAG-007_CandidateEdgeWorklist.csv`: zero.
- Active SCCs in proposed `DAG-007`: zero.
- Endpoint issues in proposed `DAG-007`: zero.
- Bidirectional active pairs in proposed `DAG-007`: zero.
- Duplicate directed active edges before aggregate consolidation: 30.
- Duplicate directed active edges after aggregate consolidation: zero.
- Human graph-decision flags: none recorded by workers.

## Approval Boundary

`execution/_DAG/DAG-007/` remains proposed pending human approval. This refresh does not update `execution/_DAG/_LATEST.md`, current-authority coordination pointers, lifecycle state, review state, release readiness, or git state.
