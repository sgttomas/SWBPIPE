# Read-only Agent 2 return — DAG-008 structural cross-check

## Verdict

**PASS.** A DAG-008 proposal is structurally derivable from DAG-007 with only the 13 keyed satisfaction refreshes. Root `execution/_DAG/_LATEST.md` must remain on approved DAG-007.

## Minimum candidate package

Mirror the 13-file DAG-007 proposal surface:

1. `APPROVAL_RECORD.md` — pending approval template only
2. `Cycle_Report.md`
3. `DAG-008_APPROVAL_REVIEW_PACKET.md`
4. `DAG-008_CandidateEdgeWorklist.csv`
5. `DAG-008_DuplicateEdgeWorklist.csv`
6. `DAG_Audit.json`
7. `DAG_Audit.md`
8. `DeliverableNodes.csv`
9. `DependencyEdges.csv`
10. `PROPOSAL_RECORD.md`
11. `TopologicalWaves.md`
12. `_LATEST.md` — package-local proposal pointer
13. `dag.json`

The initial DAG-007 proposal commit `28219696d6f104cb816c0f8b203450705f24eb9e` contained these 13 roles. Candidate provenance, manifest, and handoff artifacts may be added without changing the canonical graph surface.

## Expected state and delta

- All proposal records: `proposed_pending_human_approval`; `created: 2026-07-22`.
- `approved: TBD` and `approved_by: TBD` only in the approval placeholder.
- Decomposition revision `0.7`; node/topology baseline DAG-007.
- Candidate worklist zero rows; duplicate worklist 30 inherited historical dispositions.
- `DeliverableNodes.csv` byte-identical to DAG-007: 17 columns, 101 nodes, 18 packages.
- Preserve DAG-007's 31-column edge header, row order, IDs, extensions, and every non-target row.
- Seven DEL-08-01 rows change `SatisfactionStatus`, `LastSeen`, and append-only `Notes`.
- Six DEL-10-05 rows additionally change `ProposedMaturity`.
- Historical note tokens remain provenance and must not be rewritten.

## Expected counts

| Measure | Value |
|---|---:|
| Edge rows | 1,480 |
| ACTIVE / RETIRED | 1,395 / 85 |
| Candidate rows | 0 |
| ANCHOR / EXECUTION | 378 / 1,102 |
| SATISFIED / TBD / PENDING / NOT_APPLICABLE | 788 / 329 / 123 / 240 |
| ACTIVE SATISFIED / TBD | 771 / 273 |
| Proposed SEMANTIC_READY / TBD | 1,323 / 57 |
| Unique directed active edges | 972 |
| Active SCCs | 0 |
| Topological waves | 15 |

Active dependency-type counts remain `CONSTRAINT=325`, `ENABLES=7`, `HANDOVER=11`, `INTERFACE=105`, `OTHER=397`, `PREREQUISITE=550`. Wave membership and the `dag.json.edges` array remain identical to DAG-007.

## Tool hazards

- `dependency_type_rectification.py` is DAG-007/DAG-006-specific and assigns `LastSeen=today` to all 1,480 rows. It must not be used to build DAG-008; an in-memory reproduction would change 1,467 unintended rows in addition to the 13 targets.
- `audit_dag.py` computes correct JSON/counts but its Markdown renderer hard-codes DEV-001, DAG-001, and 2026-04-30. DAG-008 Markdown labels must be corrected while preserving the computed facts.

## Validation requirements

- Canonical dependency schema and strict DAG audit.
- JSON parsing for `dag.json` and `DAG_Audit.json`.
- Byte comparison of DAG-007 and DAG-008 nodes.
- Exact keyed 13-row delta and all-other-row equivalence.
- Independent row/status/satisfaction/type/uniqueness counts.
- Topology, wave, cycle, SCC, duplicate, and bidirectional consistency.
- No approval or root-pointer claim.

Read-only scope upheld; no files were created or modified by this child.
