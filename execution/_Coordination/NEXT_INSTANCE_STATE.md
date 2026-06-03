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
- Latest review-surface closeout:
  `execution/_Aggregation/TP-CHECKING-REVIEW-GAP-CLOSEOUT-001_2026-06-03/`.

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
- The DAG is a relationship map. Fresh deliverable state is expected in each
  deliverable-local folder: `_STATUS.md`, `MEMORY.md`, dependencies, run
  records, review files when present, semantic/lensing files when present, and
  the four-document kit.
- `TP-CODE-EVIDENCE-MIGRATION-RECONCILIATION-001_2026-06-03` records human
  acceptance that the non-resolving DEV-001 evidence commits for the 11
  reconciled CHECKING candidates are migration artifacts. Do not treat that
  same missing-commit condition as a new blocker for those deliverables.
- `TP-CHECKING-REVIEW-GAP-CLOSEOUT-001_2026-06-03` closed the local review
  surface gap for the 11 CHECKING deliverables. Existing review surfaces were
  inventoried for `DEL-02-01` through `DEL-02-05`, `DEL-06-03`, `DEL-08-04`,
  and `DEL-08-05`; new SELF_CHECK / AGENT_CHECK review surfaces were created
  for `DEL-17-01`, `DEL-17-02`, and `DEL-17-03`. No lifecycle, DAG authority,
  DEV-001 implementation evidence, release, compatibility, code-compliance, or
  professional-reliance claim was made.
- Review closeout residuals: `DEL-06-03`, `DEL-08-04`, `DEL-08-05`, and
  `DEL-17-01` have clean review surfaces for the next human gate. `DEL-02-01`
  through `DEL-02-05`, `DEL-17-02`, and `DEL-17-03` remain held for human
  disposition of AGENT_CHECK findings before any later `ISSUED` consideration.
  `DEL-17-03` has one MAJOR AGENT_CHECK finding about production-document
  implementation posture versus later implementation evidence.

## Immediate Next Actions

1. Read `NEXT_INSTANCE_PROMPT.md`.
2. Read `_COORDINATION.md` and this `NEXT_INSTANCE_STATE.md`.
3. Read `SOFTWARE_DECOMP`, `DAG-005`, and `DEV-001` surfaces as stipulated.
4. Review
   `execution/_Aggregation/TP-CHECKING-REVIEW-GAP-CLOSEOUT-001_2026-06-03/`.
5. Propose exactly one next bounded tranche. Recommended next tranche:
   human-disposition review for the seven held CHECKING deliverables
   (`DEL-02-01` through `DEL-02-05`, `DEL-17-02`, `DEL-17-03`), with no
   lifecycle transition unless the human separately approves the gate action.


## Do Not Change Without Explicit Human Approval

- lifecycle `_STATUS.md` files;
- DAG artifacts or dependency registers;
- candidate-edge promotion or graph authority;
- `DEV-001_IMPLEMENTATION_EVIDENCE.csv`;
- blocker queues except through approved deterministic coordination workflow;
- release records or acceptance records;
- professional, certification, sealing, authentication, code-compliance, or
  release-readiness-for-reliance claims.
