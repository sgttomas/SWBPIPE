---
doc_id: DAG-008-AGGREGATE-DAG-AUDIT
doc_kind: coordination.audit
status: generated_candidate_non_authoritative
created: 2026-07-22
---

# DAG-008 Aggregate DAG Audit

## Authority

- Candidate source: `execution/_DAG/DAG-008/DependencyEdges.csv` when materialized by CHANGE.
- Approved authority remains `execution/_DAG/DAG-007/` until a separate owner acceptance and root pointer authorization take effect.
- Local `Dependencies.csv` registers are the refreshed source for the 13 named satisfaction rows; all other rows derive from approved DAG-007.
- `CANDIDATE` rows remain non-gating.
- Canonical dependency enum mode: True.

## Schema And Counts

- Edge schema valid: True
- Edge rows: 1480
- Node rows: 101
- Packages represented: 18
- Active register rows: 1395
- Retired rows: 85
- Candidate rows: 0
- Endpoint issues: 0
- Ragged edge rows: 0
- Canonical findings: 0

## Active Graph

- Nodes touched: 101
- Directed edges: 972
- SCCs with more than one node: 0
- Duplicate directed edges: 0
- Bidirectional pairs: 0
- Hubs at degree >= 20: 20

## Active Plus Candidate Graph

- Nodes touched: 101
- Directed edges: 972
- SCCs with more than one node: 0
- Duplicate directed edges: 0
- Bidirectional pairs: 0

## DEV-001 Projection

- Definition: Excludes PKG-00 endpoints and ARCHITECTURE_BASIS rows; candidate rows remain non-gating.
- Projection nodes: 93
- Projection rows: 893
- Projection active edges: 815
- Projection candidate edges: 0
- Projection active SCCs: 0
- Projection active duplicate directed edges: 0
- Projection active bidirectional pairs: 0
- Projection active+candidate SCCs: 0

## Active Hubs

- DEL-00-06: in=93 out=0 total=93
- DEL-00-08: in=92 out=0 total=92
- DEL-00-01: in=91 out=0 total=91
- DEL-00-02: in=91 out=0 total=91
- DEL-00-07: in=72 out=0 total=72
- DEL-00-03: in=71 out=0 total=71
- DEL-00-04: in=59 out=0 total=59
- DEL-02-02: in=36 out=8 total=44
- DEL-02-05: in=22 out=10 total=32
- DEL-02-01: in=24 out=7 total=31
- DEL-01-02: in=21 out=5 total=26
- DEL-07-08: in=0 out=23 total=23
- DEL-02-03: in=14 out=8 total=22
- DEL-08-04: in=11 out=11 total=22
- DEL-08-06: in=0 out=22 total=22
- DEL-01-04: in=16 out=4 total=20
- DEL-03-02: in=10 out=10 total=20
- DEL-04-01: in=12 out=8 total=20
- DEL-06-01: in=8 out=12 total=20
- DEL-13-04: in=5 out=15 total=20
