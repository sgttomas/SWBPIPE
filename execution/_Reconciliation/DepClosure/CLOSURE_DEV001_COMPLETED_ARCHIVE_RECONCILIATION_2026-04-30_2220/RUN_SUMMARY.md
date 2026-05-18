---
doc_id: RUN-SUMMARY-DEP-CLOSURE-DEV001-COMPLETED-ARCHIVE
doc_kind: audit.run_summary
status: complete
created: 2026-05-01
run_label: DEV001_COMPLETED_ARCHIVE_RECONCILIATION
scope: completed product deliverables archived in NEXT_INSTANCE_STATE
run_status: OK
---

# Dependency Closure Run Summary - DEV001 Completed Archive

## Purpose

Verify dependency closure and local-register hygiene for the completed product
deliverables listed in the compact archive table of
`execution/_Coordination/NEXT_INSTANCE_STATE.md`.

## Inputs

- Coordination state:
  `execution/_Coordination/NEXT_INSTANCE_STATE.md`
- Aggregate authority:
  `execution/_DAG/DAG-001/DependencyEdges.csv`
- Scoped temporary execution tree:
  `/tmp/dev001_completed_archive_scope_7tqusmjp`
- Audit tool:
  `tools/coordination/analyze_dep_closure.py`

## Scope

Included 16 product deliverables:

`DEL-01-01`, `DEL-01-02`, `DEL-01-03`, `DEL-01-04`,
`DEL-02-01`, `DEL-02-02`, `DEL-02-03`, `DEL-02-04`, `DEL-02-05`,
`DEL-03-01`, `DEL-03-02`, `DEL-03-03`, `DEL-03-04`, `DEL-03-05`,
`DEL-03-06`, `DEL-03-07`.

Control-plane archive rows were excluded from this product-deliverable audit.

## Results

| Check | Result |
|---|---:|
| Local `Dependencies.csv` files found | 16 |
| Dependency rows loaded | 130 |
| Schema-valid local registers | 16 |
| Schema-invalid local registers | 0 |
| Evidence populated | 130 / 130 |
| Graph nodes in scoped closure view | 23 |
| Graph edges in scoped closure view | 130 |
| Orphan deliverables | 0 |
| Active SCCs larger than one node | 0 |
| Hub nodes at threshold >= 20 | 0 |
| Bidirectional active pairs | 0 |
| ID normalizations | 0 |

`RUN_STATUS = OK`.

## Notes

- The analyzer reported missing `IMPLEMENTS_NODE` anchors for all 16 scoped
  registers. This is expected for DEV-001 materialized local mirrors because
  `DAG-001` is the aggregate execution-dependency authority and local
  registers are synchronized mirrors/evidence, not independent sequencing
  authority.
- This run did not promote candidate edges, mutate `DAG-001`, mutate local
  dependency registers, recompute the DEV-001 blocker queue, or change
  deliverable lifecycle states.
