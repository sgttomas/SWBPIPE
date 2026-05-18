---
doc_id: DAG-002-AUDIT
doc_kind: coordination.dag_audit
status: generated_approved_revision_0_5_graph
created: 2026-05-03
decomposition_revision: "0.5"
---

# DAG-002 Graph Audit

## Authority

`DAG-002` is the approved revision `0.5` active edge set for OpenPipeStress
SOFTWARE development coordination. Approval is recorded at
`execution/_DAG/DAG-002/APPROVAL_RECORD.md`.

This audit does not run blocker readiness, authorize lifecycle transitions,
refresh deliverable-local dependency mirrors, dispatch Type 2 work, run
`PREPARATION`, or promote Chirality corpus material. Candidate rows remain
non-gating.

## Counts

| Fact | State |
|---|---:|
| Deliverable nodes | 92 |
| Packages represented | 17 |
| Historical DAG-001 nodes carried forward | 73 |
| Revision 0.5 nodes added to proposal | 19 |
| Approved active edges | 859 |
| Non-gating candidate rows | 8 |
| Retired proposal rows | 1 |
| Total dependency rows | 868 |
| Invalid endpoints | 0 |
| Self dependencies | 0 |
| Duplicate active directed edges | 0 |
| Bidirectional active pairs | 0 |
| Active SCCs | 0 |
| Active plus candidate SCCs | 3 |
| Topological waves from approved active edges | 15 |

## Revision 0.5 Additions

The proposal node register includes all 19 accepted revision `0.5` nodes absent from `DAG-001`: `DEL-07-08`, `DEL-08-06`, `DEL-13-01`, `DEL-13-02`, `DEL-13-03`, `DEL-13-04`, `DEL-14-01`, `DEL-14-02`, `DEL-14-03`, `DEL-14-04`, `DEL-14-05`, `DEL-15-01`, `DEL-15-02`, `DEL-15-03`, `DEL-15-04`, `DEL-16-01`, `DEL-16-02`, `DEL-16-03`, `DEL-16-04`.

The edge-disposition review added active rows for all SCA-002 added nodes and applicable architecture-basis context. No revision `0.5` added node remains without an active incident edge.

## Review Flags

- `DEL-01-04` is accepted for revision `0.5` graph-authoring reliance as a professional-boundary predecessor.
- `DEL-02-01` is accepted as a foundational predecessor, and the required supplemental revision `0.5` schema/context work has been executed and validated in the working tree.
- `DAG-002-E0621` is retired from the candidate layer because `DEL-08-05` linter evidence uses invented synthetic fixtures and does not depend on `DEL-11-04` educational examples.
- Remaining candidate rows are non-gating and must not affect blocker queues, wave placement, schedule, staffing, priority, or dispatch readiness.
- `docs/_ScopeChange/chirality-app-docs/` remains quarantined reference material and created no DAG edges.

## Machine Summary

Detailed counts are recorded in `DAG_Audit.json`.

## Approval

The refreshed proposal was packaged for human approval review in
`DAG-002_APPROVAL_REVIEW_PACKET.md`. The human project authority approved the
active edge set on 2026-05-03. Later blocker readiness recomputation and
dependency mirror refresh require their own guarded workflow steps.
