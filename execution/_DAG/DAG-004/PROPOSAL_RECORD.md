---
doc_id: DAG-004-PROPOSAL-RECORD
doc_kind: coordination.dag_proposal_record
status: proposed_pending_review
created: 2026-05-17
approval_status: not_approved_pending_review
---


# DAG-004 Proposal Record

## Boundary

`DAG-004` is a bounded metadata/evidence refresh proposal against accepted `SCA-003` decomposition revision `0.6`. It preserves the approved `DAG-003` node/edge topology and refreshes stale storage evidence language where SCA-003 resolved the MVP local storage profile.

This record does not approve the graph, run Type 2 work, change lifecycle states, run `PREPARATION`, or make professional/code acceptance claims.

## Sources

- `plans/DAG-004_METADATA_EVIDENCE_REFRESH_HANDOFF_PLAN.md`
- `plans/TP-DAG-003_DAG_STAGE_REFRESH_PLAN.md`
- `plans/TP-DAG-004_DEPENDENCY_SURFACE_REFRESH_PLAN.md`
- `plans/TP-DAG-004_DEPENDENCY_REFRESH_DISPATCH_MATRIX.csv`
- `docs/_ScopeChange/SCA-003_2026-05-17_1658/Authority.md`
- `execution/_ScopeChange/SCA-003_2026-05-17_1658/Handoff_State.md`
- `execution/_ScopeChange/SCA-003_2026-05-17_1658/RUN_SUMMARY.md`
- `plans/SCA-003_DOWNSTREAM_REFRESH_PLAN.md`
- `execution/_DAG/DAG-003/DependencyEdges.csv` and `APPROVAL_RECORD.md` as approved baseline authority

## Result

| Artifact | State |
|---|---|
| DeliverableNodes.csv | 92 current register nodes |
| DependencyEdges.csv | 904 graph rows preserved from DAG-003 topology; 861 active, 11 candidate, 32 retired |
| DAG_Audit.md / DAG_Audit.json | Generated metadata/evidence-refresh graph audit |
| Cycle_Report.md | Active layer acyclic; candidate layer warning only |
| TopologicalWaves.md | 15 active-edge waves; not dispatch order |
| DAG-004_DEPENDENCY_RECONCILIATION.md / .csv | Duplicate and exclusion disposition evidence |
| APPROVAL_RECORD.md | Not created; DAG-004 remains proposed pending review |

## Current Counts

| Fact | State |
|---|---:|
| Packages represented | 17 |
| Deliverable nodes represented | 92 |
| Active edges | 861 |
| Candidate edges | 11 |
| Retired rows | 32 |
| Invalid endpoints | 0 |
| Active SCCs | 0 |
| Active plus candidate SCC warnings | 2 |
| Topological waves | 15 |

## Candidate Treatment

Candidate rows remain non-gating and must not drive blocker queues, dispatch order, staffing, scheduling, lifecycle state, or implementation readiness unless later promoted by explicit human gate and graph revalidation.

## SCA-003 Evidence Refresh

SCA-003 revision `0.6` resolves the MVP physical storage profile as a local SQLite project store/index. Canonical JSON/JCS-compatible payloads remain the domain and interchange truth; SQLite FTS5/BM25 sidecars are rebuildable and non-authoritative; optional NumPy sidecars remain non-authoritative caches only if later justified. No hosted DB, daemon, required network, cloud sync, telemetry path, or direct plugin/adapter SQL access is authorized.

The refresh updates stale storage evidence on current graph rows without changing `FromDeliverableID`, `TargetDeliverableID`, `Status`, or dependency classification.

## Approval Status

`DAG-004` is `proposed_pending_review`. `DAG-003` remains approved graph authority until a later explicit human approval record promotes a successor graph.
