---
doc_id: DEV-001-AGGREGATE-DAG-AUDIT
doc_kind: coordination.audit
status: generated
created: 2026-04-30
---

# DEV-001 Aggregate DAG Audit

## Authority

- Source of truth: `execution/_DAG/DAG-001/DependencyEdges.csv`.
- Local `Dependencies.csv` files are synchronized mirrors, not independent sequencing authority.
- `CANDIDATE` rows remain non-gating.
- Canonical dependency enum mode: True.

## Schema And Counts

- Edge schema valid: True
- Edge rows: 1571
- Node rows: 106
- Packages represented: 18
- Active edges: 1486
- Candidate edges: 0
- Endpoint issues: 0
- Ragged edge rows: 0
- Canonical findings: 0

## Active Graph

- Nodes touched: 106
- Directed edges: 1034
- SCCs with more than one node: 0
- Duplicate directed edges: 0
- Bidirectional pairs: 0
- Hubs at degree >= 20: 23

## Active Plus Candidate Graph

- Nodes touched: 106
- Directed edges: 1034
- SCCs with more than one node: 0
- Duplicate directed edges: 0
- Bidirectional pairs: 0

## DEV-001 Projection

- Definition: Excludes PKG-00 endpoints and ARCHITECTURE_BASIS rows; candidate rows remain non-gating.
- Projection nodes: 98
- Projection rows: 972
- Projection active edges: 894
- Projection candidate edges: 0
- Projection active SCCs: 0
- Projection active duplicate directed edges: 0
- Projection active bidirectional pairs: 0
- Projection active+candidate SCCs: 0

## Active Hubs

- DEL-00-06: in=95 out=0 total=95
- DEL-00-08: in=93 out=0 total=93
- DEL-00-02: in=92 out=0 total=92
- DEL-00-01: in=91 out=0 total=91
- DEL-00-07: in=75 out=0 total=75
- DEL-00-03: in=74 out=0 total=74
- DEL-00-04: in=59 out=0 total=59
- DEL-02-02: in=39 out=8 total=47
- DEL-02-05: in=24 out=10 total=34
- DEL-02-01: in=26 out=7 total=33
- DEL-01-02: in=21 out=5 total=26
- DEL-07-08: in=0 out=24 total=24
- DEL-02-03: in=15 out=8 total=23
- DEL-08-04: in=11 out=11 total=22
- DEL-08-06: in=0 out=22 total=22
- DEL-04-01: in=13 out=8 total=21
- DEL-07-12: in=0 out=21 total=21
- DEL-01-04: in=16 out=4 total=20
- DEL-03-02: in=10 out=10 total=20
- DEL-04-06: in=12 out=8 total=20
- DEL-06-01: in=8 out=12 total=20
- DEL-07-01: in=4 out=16 total=20
- DEL-13-04: in=5 out=15 total=20

