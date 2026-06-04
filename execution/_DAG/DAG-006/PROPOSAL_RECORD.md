---
doc_id: DAG-006-PROPOSAL-RECORD
doc_kind: coordination.proposal_record
status: approved_active_graph_authority
created: 2026-06-03
---

# DAG-006 Proposal Record

`DAG-006` is based on approved `DAG-005` and preserves its node membership,
dependency edge set, candidate-row treatment, and topological ordering.

The only intended graph-register schema change is removal of lifecycle status
metadata from DAG node artifacts:

- `DeliverableNodes.csv` no longer carries a `LifecycleState` column.
- `dag.json` nodes no longer carry `lifecycle_state`.

Deliverable lifecycle state is authoritative only in deliverable-local
`_STATUS.md` files and should be read through
`tools/coordination/list_deliverable_status.py`.

`DAG-006` was approved by the human project authority on 2026-06-03 as active
graph authority. Candidate rows inherited from `DAG-005` remain non-gating.
