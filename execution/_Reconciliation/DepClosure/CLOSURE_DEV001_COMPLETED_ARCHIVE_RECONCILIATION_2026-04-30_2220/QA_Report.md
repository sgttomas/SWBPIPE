---
doc_id: QA-DEP-CLOSURE-DEV001-COMPLETED-ARCHIVE
doc_kind: audit.qa_report
status: complete
created: 2026-05-01
---

# QA Report - DEV001 Completed Archive Dependency Closure

## Coverage

The scoped run covered 16 approved completed product deliverables. Each scoped
deliverable had a readable `Dependencies.csv` file.

Coverage evidence:

- `Evidence/coverage.csv`
- `Evidence/closure_summary.json`

## Schema

All 16 scoped local registers passed the v3.1 dependency schema check.

The analyzer loaded 130 dependency rows. No schema-invalid registers were
excluded.

## Topology

The scoped active graph had:

- 23 nodes;
- 130 edges;
- 0 orphan deliverables;
- 0 SCCs larger than one node;
- 0 bidirectional active pairs.

## Data Hygiene

The run found:

- 130 / 130 rows with populated evidence fields;
- 0 ID normalizations;
- 0 hub nodes at the threshold of 20.

## Known Limits

- The deterministic analyzer consumes local `Dependencies.csv` files. It does
  not directly consume aggregate `DAG-001` unless wrapped or given a scoped
  execution tree.
- `IMPLEMENTS_NODE` anchors are absent in the scoped local mirrors. This is a
  known DEV-001 mirror-policy artifact, not a blocker for this run.
- This audit checks dependency closure and register hygiene. It does not review
  product implementation quality, test sufficiency, protected-content wording,
  or open `TBD` resolution.
