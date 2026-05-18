---
doc_id: DAG-003-DEPENDENCY-RECONCILIATION
doc_kind: coordination.reconciliation
status: approved_active_edge_set_guarded_followups
created: 2026-05-11
approval_status: approved_2026-05-11
---


# DAG-003 Dependency Reconciliation

## Method

- Read all 84 refreshed non-PKG-00 deliverable-local `Dependencies.csv` files.
- Selected only `DependencyClass=EXECUTION` rows with `TargetType=DELIVERABLE` for aggregate graph topology.
- Excluded local `ANCHOR` rows and non-deliverable execution rows from the aggregate graph while preserving them in deliverable-local evidence.
- Collapsed duplicate normalized directed edges into one proposal row, preferring `ACTIVE` over `CANDIDATE` over `RETIRED`, then higher confidence and stronger source origin.
- Assigned aggregate proposal IDs `DAG-003-R####`; each row notes the selected source dependency ID and local source file.

## Counts

| Metric | Count |
|---|---:|
| Local dependency files read | 84 |
| Local rows read | 1171 |
| Proposal graph rows | 904 |
| Active proposal rows | 861 |
| Candidate proposal rows | 11 |
| Retired proposal rows | 32 |
| Anchor/context rows excluded | 204 |
| Non-deliverable execution rows excluded | 33 |
| Duplicate normalized edge pairs | 29 |
| Duplicate local rows omitted | 30 |

## Exclusions

Excluded rows remain valid local dependency evidence, but they are not deliverable-to-deliverable graph edges for DAG topology.

- `non_deliverable_target:DOCUMENT`: 27
- `non_deliverable_target:EXTERNAL`: 3
- `non_deliverable_target:PACKAGE`: 2
- `non_deliverable_target:UNKNOWN`: 1
- `non_execution_anchor_or_context`: 204

## Review Artifacts

- Detailed edge dispositions: `execution/_DAG/DAG-003/DAG-003_DEPENDENCY_RECONCILIATION.csv`
- Machine summary: `execution/_DAG/DAG-003/DAG-003_DEPENDENCY_RECONCILIATION_SUMMARY.json`
