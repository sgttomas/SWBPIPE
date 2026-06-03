# NEXT INSTANCE STATE

Last updated: 2026-06-03
Updated by: WORKING_ITEMS

## Authority Pointers

- Decomposition authority:
  `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.7`.
- Graph authority: `execution/_DAG/DAG-005/`, as pointed to by
  `execution/_DAG/_LATEST.md`.
- Graph approval record: `execution/_DAG/DAG-005/APPROVAL_RECORD.md`.
- Coordination record: `execution/_Coordination/_COORDINATION.md`.
- Implementation evidence:
  `execution/_Coordination/DEV-001_IMPLEMENTATION_EVIDENCE.csv`.
- Blocker queue derivatives:
  `execution/_Coordination/DEV-001_BLOCKER_QUEUE.md` and `.csv`.

## Current Program State

- `SOFTWARE_DECOMP` says what must be built and why.
- `DAG-005` says what depends on what, using approved active edges.
- `DEV-001` says what is currently unblocked or blocked for implementation
  based on committed evidence.
- `_COORDINATION.md` defines the Integrated Verification and Tranche Selection
  Loop: authority intake, state verification, optional read-only integrated
  verification snapshot, gap-to-tranche selection, human approval, bounded
  execution, fan-in, validation, evidence updates, and handoff.
- `DAG-005` remains approved active graph authority. Candidate rows are
  non-gating unless later promoted by explicit human gate and graph
  revalidation.
- `DEV-001_BLOCKER_QUEUE.md` reports 101 implementation-unblocked deliverables
  and 0 blocked deliverables using active `DAG-005` edges only.

## Immediate Next Actions

1. Read `NEXT_INSTANCE_PROMPT.md`.
2. Read `_COORDINATION.md` and this `NEXT_INSTANCE_STATE.md`.
3. Read `SOFTWARE_DECOMP`, `DAG-005`, and `DEV-001` surfaces as stipulated.
4. Apply the Integrated Verification and Tranche Selection Loop from
   `_COORDINATION.md`.


## Do Not Change Without Explicit Human Approval

- lifecycle `_STATUS.md` files;
- DAG artifacts or dependency registers;
- candidate-edge promotion or graph authority;
- `DEV-001_IMPLEMENTATION_EVIDENCE.csv`;
- blocker queues except through approved deterministic coordination workflow;
- release records or acceptance records;
- professional, certification, sealing, authentication, code-compliance, or
  release-readiness-for-reliance claims.
