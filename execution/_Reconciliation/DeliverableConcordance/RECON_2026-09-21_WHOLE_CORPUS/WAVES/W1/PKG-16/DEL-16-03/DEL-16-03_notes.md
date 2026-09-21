# DEL-16-03 notes — User acceptance and operation audit trail (worker G1, W1)

## Path aliases

- Deliverable-folder paths contain spaces, so they cannot be evidence tokens
  (W-13). They appear in `ContextRefs` or by alias in Notes:
  `RF-16xx` = `projects/chirality-piping/execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-xx_<name>/Review_Findings.csv`.
- `SOW`, `STATUS`, `CONTEXT`, `MEMORY` = that deliverable folder's
  `ScopeOfWork.md`, `_STATUS.md`, `_CONTEXT.md`, `MEMORY.md`.
- Parity evidence is the repository-root record
  `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/instances/CHANGE-P4/checks/DEL-16-xx/{parity.md,claim-map.csv}`
  (latest per `EVIDENCE_MAP.csv`); migration parity `.../WORKING-P4-PKG16/children/AUTHOR-B1/members/DEL-16-xx/parity-a.md` goes in ContextRefs.
- Project-root tokens (`schemas/`, `core/`, `apps/`, `tests/`, `fixtures/`) resolve under `projects/chirality-piping/`; `tools/validation/...` is the repository-root lint (DEC-081 grant).

## Judgment calls

- **FG-DEL-16-03-01 (W-3).** The SOW presents only the Python audit-record
  constructor (called only by tests) and declares operation application
  outside the slice or TBD. At the freeze:
  - the applier applies intents, including atomic batches, and emits
    AcceptanceRecord receipts (`user_initiated_apply_in_local_session`,
    `acceptance_is_professional_approval false`,
    `session_state_only_not_yet_saved`);
  - the desktop OperationLedgerPanel exports a held-for-user-acceptance
    ledger;
  - session undo/redo exists;
  - editor_intents persist in the SCA-003 SQLite store.

  Rows declaring application or container absent/TBD are marked
  `DOC_BEHIND_CODE`.
- **FG-DEL-16-03-02 (W-6/C6(a)).** Durable accepted/rejected history under
  SOW-070 is incomplete. REQ-005, CLM-023 and STATUS R01 are marked
  `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · PROJECT_BASELINE · BASELINE`.
  R01's text is accurate and its three linked N2 records exist. Because it is
  a product gap, it is not ALIGNED.
- REQ-002 is `PARTIALLY_IMPLEMENTED`: the Python gate holds, but runtime
  acceptance receipts are issued while intents carry `constraint_validation:
  not_run` (cross-reference FG-DEL-16-02-03).
- CLM-011 (audit-log persistence TBD) is ALIGNED with MEDIUM confidence: the
  container is ruled, but no audit-log persistence exists.
- Scope Detail is run together on one line, so it has no CS-06 row. Both
  statements match ScopeLedger.csv, so it is ALIGNED.
- REQ-007 cites the removed `INIT.md` (commit 9c4caf8fd). This is noted as a
  stale source pointer only.
- No `.rNN` split was used.

## Canonical departures

None.

## Convention friction

- Remaining items that accurately describe an open product gap have no named
  pattern (CP-06 covers only items with no open action). I applied C6(a) (W-6).

## Reverse pass

- It did not change my view of anything sealed.
- Answers:
  - CLAIMED_BY: the Python audit trail (RC-16-0050) and the operation review
    ledger (RC-16-0166, the anticipated "operation audit log").
  - UNKEYED: single apply, batch apply (Rust and service) and undo/redo. The
    SOW declares application outside its slice, which is consistent with
    FG-01.
  - PARTIAL: the acceptance-record part of the outcome envelope.
  - COVERS: UI consumers.

## Batch consistency

`validate_ledger_v2.py --batch` over the four PKG-16 forward ledgers: **PASS, 0
consistency findings**. No `CANONICAL_DEPARTURE` was needed or used.

## UNKNOWN rows

None. No row in this ledger is `UNKNOWN`, so no smallest-next-check is owed.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9): since
2026-09-19 Piping selects work through owner-steered work graphs, not
`## Remaining`.

## Fences

Gate evidence is suite-level only (`GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json`;
no suite rerun). No protected standards, vendor or private data is quoted. No
external-corpus equation artifact is used. Nothing here states or implies
release, approval, compliance or certification, and no disposition is an owner
ruling.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
