---
doc_id: DAG-003-AUDIT
doc_kind: coordination.audit
status: approved_active_edge_set_guarded_followups
created: 2026-05-11
approval_status: approved_2026-05-11
---


# DAG-003 Dependency-Refreshed Graph Audit

## Authority

- Source graph: `execution/_DAG/DAG-003/DependencyEdges.csv`.
- Node source: `execution/_DAG/DAG-003/DeliverableNodes.csv`.
- Edge source: reconciled from 84 refreshed deliverable-local `Dependencies.csv` files produced by `TP-DAG-004`.
- Baseline approved graph remains `execution/_DAG/DAG-002/` revision `0.5` until a human approval record exists for this proposal.
- `CANDIDATE` rows remain non-gating and were not promoted by reconciliation.
- This audit does not approve lifecycle state, Type 2 dispatch, release readiness, code compliance, certification, sealing, or professional acceptance.

## Schema And Counts

- Edge schema valid: True
- Edge rows: 904
- Node rows: 92
- Packages represented: 17
- Status counts: 861 active, 11 candidate, 32 retired
- Endpoint issues: 0
- Ragged edge rows: 0
- Ragged node rows: 0

## Reconciliation Inputs

- Refreshed local dependency files read: 84
- Total local rows read: 1171
- Execution deliverable graph rows selected: 904
- Anchor/context rows excluded from graph topology: 204
- Non-deliverable execution rows excluded from graph topology: 33
- Normalized duplicate edge pairs dispositioned: 29
- Duplicate local rows omitted from aggregate graph: 30

## Active Graph

- Nodes touched: 92
- Directed edges: 861
- SCCs with more than one node: 0
- Duplicate directed edges: 0
- Bidirectional pairs: 0
- Self dependencies: 0
- Hubs at degree >= 20: 17

## Active Plus Candidate Graph

- Nodes touched: 92
- Directed edges: 872
- SCCs with more than one node: 2
- Duplicate directed edges: 0
- Bidirectional pairs: 4

## DEV-001 Projection

- Definition: Excludes PKG-00 endpoints and ARCHITECTURE_BASIS rows; candidate rows remain non-gating.
- Projection nodes: 84
- Projection rows: 385
- Projection active edges: 347
- Projection candidate edges: 11
- Projection active SCCs: 0
- Projection active duplicate directed edges: 0
- Projection active bidirectional pairs: 0
- Projection active+candidate SCCs: 2

## Candidate-Layer SCCs

- SCC-C-001: `DEL-10-02`, `DEL-12-01`, `DEL-12-05`
- SCC-C-002: `DEL-08-01`, `DEL-08-02`, `DEL-08-04`, `DEL-08-05`, `DEL-09-05`, `DEL-10-04`, `DEL-10-05`

## Active Hubs

- `DEL-00-06`: in=84 out=0 total=84
- `DEL-00-08`: in=83 out=0 total=83
- `DEL-00-01`: in=82 out=0 total=82
- `DEL-00-02`: in=82 out=0 total=82
- `DEL-00-07`: in=63 out=0 total=63
- `DEL-00-03`: in=62 out=0 total=62
- `DEL-00-04`: in=50 out=0 total=50
- `DEL-02-02`: in=31 out=8 total=39
- `DEL-02-01`: in=20 out=7 total=27
- `DEL-01-02`: in=21 out=5 total=26
- `DEL-02-05`: in=14 out=10 total=24
- `DEL-08-04`: in=11 out=11 total=22
- `DEL-08-06`: in=0 out=22 total=22
- `DEL-07-08`: in=0 out=21 total=21
- `DEL-01-04`: in=16 out=4 total=20
- `DEL-04-01`: in=12 out=8 total=20
- `DEL-06-01`: in=8 out=12 total=20

## Audit Verdict

- Strict active graph audit: PASS.
- Approval status: not approved; ready for human review as a dependency-refreshed graph proposal.
