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

## Schema And Counts

- Edge schema valid: True
- Edge rows: 988
- Node rows: 101
- Packages represented: 18
- Active edges: 945
- Candidate edges: 11
- Endpoint issues: 0
- Ragged edge rows: 0

## Active Graph

- Nodes touched: 101
- Directed edges: 945
- SCCs with more than one node: 0
- Duplicate directed edges: 0
- Bidirectional pairs: 0
- Hubs at degree >= 20: 18

## Active Plus Candidate Graph

- Nodes touched: 101
- Directed edges: 956
- SCCs with more than one node: 2
- Duplicate directed edges: 0
- Bidirectional pairs: 4

## DEV-001 Projection

- Definition: Excludes PKG-00 endpoints and ARCHITECTURE_BASIS rows; candidate rows remain non-gating.
- Projection nodes: 93
- Projection rows: 406
- Projection active edges: 368
- Projection candidate edges: 11
- Projection active SCCs: 0
- Projection active duplicate directed edges: 0
- Projection active bidirectional pairs: 0
- Projection active+candidate SCCs: 2

## Active Plus Candidate SCCs

- SCC-C-001: DEL-10-02, DEL-12-01, DEL-12-05
- SCC-C-002: DEL-08-01, DEL-08-02, DEL-08-04, DEL-08-05, DEL-09-05, DEL-10-04, DEL-10-05

## Active Hubs

- DEL-00-06: in=93 out=0 total=93
- DEL-00-08: in=92 out=0 total=92
- DEL-00-01: in=91 out=0 total=91
- DEL-00-02: in=91 out=0 total=91
- DEL-00-07: in=72 out=0 total=72
- DEL-00-03: in=71 out=0 total=71
- DEL-00-04: in=59 out=0 total=59
- DEL-02-02: in=31 out=8 total=39
- DEL-02-01: in=21 out=7 total=28
- DEL-01-02: in=21 out=5 total=26
- DEL-02-05: in=14 out=10 total=24
- DEL-08-04: in=12 out=11 total=23
- DEL-08-06: in=0 out=22 total=22
- DEL-07-08: in=0 out=21 total=21
- DEL-01-04: in=16 out=4 total=20
- DEL-03-02: in=10 out=10 total=20
- DEL-04-01: in=12 out=8 total=20
- DEL-06-01: in=8 out=12 total=20

