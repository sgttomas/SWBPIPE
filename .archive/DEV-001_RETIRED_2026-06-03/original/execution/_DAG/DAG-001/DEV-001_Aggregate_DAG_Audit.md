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
- Edge rows: 624
- Node rows: 73
- Packages represented: 13
- Active edges: 615
- Candidate edges: 9
- Endpoint issues: 0
- Ragged edge rows: 0

## Active Graph

- Nodes touched: 73
- Directed edges: 615
- SCCs with more than one node: 0
- Duplicate directed edges: 0
- Bidirectional pairs: 0
- Hubs at degree >= 20: 11

## Active Plus Candidate Graph

- Nodes touched: 73
- Directed edges: 624
- SCCs with more than one node: 4
- Duplicate directed edges: 0
- Bidirectional pairs: 4

## DEV-001 Projection

- Definition: Excludes PKG-00 endpoints and ARCHITECTURE_BASIS rows; candidate rows remain non-gating.
- Projection nodes: 65
- Projection rows: 236
- Projection active edges: 227
- Projection candidate edges: 9
- Projection active SCCs: 0
- Projection active duplicate directed edges: 0
- Projection active bidirectional pairs: 0
- Projection active+candidate SCCs: 4

## Active Plus Candidate SCCs

- SCC-C-001: DEL-04-04, DEL-04-06
- SCC-C-002: DEL-08-05, DEL-11-04
- SCC-C-003: DEL-09-05, DEL-10-04
- SCC-C-004: DEL-10-02, DEL-12-01, DEL-12-05

## Active Hubs

- DEL-00-01: in=65 out=0 total=65
- DEL-00-02: in=65 out=0 total=65
- DEL-00-06: in=65 out=0 total=65
- DEL-00-08: in=65 out=0 total=65
- DEL-00-07: in=45 out=0 total=45
- DEL-00-03: in=43 out=0 total=43
- DEL-00-04: in=33 out=0 total=33
- DEL-02-02: in=23 out=8 total=31
- DEL-01-02: in=20 out=5 total=25
- DEL-02-01: in=15 out=7 total=22
- DEL-04-01: in=13 out=8 total=21
