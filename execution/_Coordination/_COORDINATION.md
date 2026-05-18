# Coordination Record

## Active Surface

Keep this surface lean. Use canonical project instructions, approved DAG
artifacts, and deliverable-local records as the active state.

Primary pointers:

- `AGENTS.md`
- `docs/CONTRACT.md`
- `docs/IP_AND_DATA_BOUNDARY.md`
- `docs/_Registers/Deliverables.csv`
- `execution/_DAG/_LATEST.md`
- `execution/_DAG/DAG-003/APPROVAL_RECORD.md`
- `execution/_DAG/DAG-003/DAG_Audit.md`
- `execution/_DAG/DAG-003/DAG-003_DEPENDENCY_RECONCILIATION.md`
- `execution/_Coordination/DEV-001_BLOCKER_QUEUE.md`
- `execution/_Coordination/DEV-001_BLOCKER_QUEUE.csv`
- `execution/PKG-*/1_Working/DEL-*/MEMORY.md`
- `execution/PKG-*/1_Working/DEL-*/_STATUS.md`

Archived coordination files remain inspectable evidence under
`execution/_Coordination/_Archive/`, but they are not the active state surface.

## Current Authority

`DAG-003` is approved graph authority as of 2026-05-11. Approval applies to the
active edge set only. Candidate rows remain non-gating unless a later explicit
human gate promotes them and the graph is revalidated.

`TP-DAG-004` refreshed all 84 non-`PKG-00` deliverable-local dependency
surfaces and reconciled them into `DAG-003`. The 8 `PKG-00` deliverables remain
architecture-basis context, not implementation work.

`DEV-001` remains the current development path. The blocker queue has been
recomputed from approved `DAG-003`: 92 unblocked, 0 blocked.

## Operating Loop

Future work should proceed by bounded tranches:

1. Agent reads the active surface and proposes one tranche with objective,
   scope, write bounds, validation, and expected closeout.
2. Human approves or redirects the tranche.
3. Agent dispatches canonical `TASK` workers where useful, normally one
   deliverable or one clearly owned slice per worker. Any brief with
   `DeliverablePath` uses `TASK` deliverable-local mode and must read
   `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`,
   `MEMORY.md`, and primary deliverable artifacts before acting.
4. Parent agent fans in results, runs review/audit, and records concise
   closeout evidence.
5. Human approval is required for lifecycle changes, candidate promotion,
   commits, release claims, acceptance records, or any professional/code
   compliance claim.

Recommended next tranche type: release-readiness gap audit or integrated
verification sweep, not more dependency-surface refresh.
