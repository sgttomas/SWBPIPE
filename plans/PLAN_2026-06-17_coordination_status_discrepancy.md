# Coordination Status Discrepancy Plan

**Date:** 2026-06-17

**Epistemic status:** issue-plan note under the project Issue-Plan Rule. This
does not change lifecycle state, DAG authority, completion-plan authority, or
release/professional status.

## Issue

The baseline intake for `TP-R3UX-PACKAGEKIT-001` found a coordination-state
mismatch:

- `execution/_Coordination/NEXT_INSTANCE_PROMPT.md` and
  `execution/_Coordination/_COORDINATION.md` state that current lifecycle state
  has all deliverables `CHECKING` or `ISSUED`.
- `python3 tools/coordination/list_deliverable_status.py --dag DAG-006 --format table --summary`
  reported `Rows: 101`, `CHECKING=8`, `IN_PROGRESS=92`, `ISSUED=1`.

This did not block the selected C5.6 tranche because the active completion plan
and coordination prompt both identify the packaged successor kit as the next
ordinary R3 item.

## Proposed cleanup

Run a bounded coordination cleanup tranche to decide which surface is stale:

1. Inspect the source of each deliverable-local `_STATUS.md` value and the
   historical lifecycle transition records.
2. Determine whether the coordination prose is stale or the status helper is
   reading stale local lifecycle files.
3. Update the non-authoritative coordination prose or prepare the required
   lifecycle/review packet. Do not mass-edit `_STATUS.md` files without the
   owning lifecycle authority.

## Boundary

This note is not a lifecycle transition, issuance claim, release-readiness
claim, professional approval, certification, sealing, authentication, or
code-compliance claim.
