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
- `execution/_DAG/DAG-005/APPROVAL_RECORD.md`
- `execution/_DAG/DAG-005/DAG_Audit.md`
- `execution/_DAG/DAG-005/DependencyEdges.csv`
- `execution/_DAG/DAG-005/DeliverableNodes.csv`
- `execution/_Coordination/DEV-001_IMPLEMENTATION_EVIDENCE.csv`
- `execution/_Coordination/DEV-001_BLOCKER_QUEUE.md`
- `execution/_Coordination/DEV-001_BLOCKER_QUEUE.csv`
- `execution/PKG-*/1_Working/DEL-*/MEMORY.md`
- `execution/PKG-*/1_Working/DEL-*/_STATUS.md`

## Current Authority

`DAG-005` is approved graph authority as of 2026-05-18. Approval applies to the
active edge set only. Candidate rows remain non-gating unless a later explicit
human gate promotes them and the graph is revalidated.

`DAG-005` is oriented toward export-format contract fulfillment from SCA-004.
The 8 `PKG-00` deliverables remain architecture-basis context, not
implementation work.

`DEV-001` remains the current development path. The blocker queue has been
recomputed from approved `DAG-005`: 101 unblocked, 0 blocked. TP-EXPORT-CLOSEOUT-001
refreshed PKG-17 implementation evidence for `DEL-17-07`, `DEL-17-08`, and
`DEL-17-09`; lifecycle states remain unchanged.

## Operating Loop

Future work should proceed by bounded tranches:

1. Agent reads the active surface and proposes one tranche with objective,
   scope, write bounds, validation, and expected closeout. For
   coordination-sensitive tranches, run
   `tools/coordination/maintain_dev001_coordination.py --dag DAG-005 --check`
   first so stale derivative blocker queues are visible before planning.
2. Human approves or redirects the tranche.
3. Agent dispatches canonical `TASK` workers where useful, normally one
   deliverable or one clearly owned slice per worker. Any brief with
   `DeliverablePath` uses `TASK` deliverable-local mode and must read
   `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`,
   `MEMORY.md`, and primary deliverable artifacts before acting.
4. Parent agent fans in results, runs review/audit, and records concise
   closeout evidence. After approved implementation-evidence edits, run
   `tools/coordination/maintain_dev001_coordination.py --dag DAG-005 --write`
   to refresh derivative blocker queue mirrors, then rerun `--check`.
5. The coordination maintenance tool is deterministic TOOLMAKER-style support:
   it validates evidence and regenerates blocker queue derivatives, but it does
   not approve or edit implementation evidence rows, lifecycle states,
   candidate rows, release claims, professional claims, or graph authority.
6. Human approval is required for lifecycle changes, candidate promotion,
   commits, release claims, acceptance records, or any professional/code
   compliance claim.

Recommended next tranche type: release-readiness gap audit or integrated
verification sweep, not more dependency-surface refresh.
