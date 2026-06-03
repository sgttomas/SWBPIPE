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
- Lifecycle vocabulary: `docs/TYPES.md`.
- Deliverable status discovery helper:
  `tools/coordination/list_deliverable_status.py`.
- Latest review-surface closeout:
  `execution/_Aggregation/TP-CHECKING-REVIEW-GAP-CLOSEOUT-001_2026-06-03/`.

## Current Program State

- `SOFTWARE_DECOMP` says what must be built and why.
- `DAG-005` says what depends on what, using approved active edges.
- Deliverable-local `_STATUS.md` files say the current lifecycle state for work
  selection. The current status-discovery helper reports 101 local status files:
  82 `IN_PROGRESS`, 11 `CHECKING`, and 8 tolerated nonstandard
  `SEMANTIC_READY` architecture-basis statuses.
- `_COORDINATION.md` defines the Local Status And DAG-Guided Development Loop:
  authority intake, status discovery, candidate selection, deliverable-local
  context inspection, DAG-guided related context, human approval, bounded
  execution, fan-in, validation, and handoff.
- `DAG-005` remains approved active graph authority. Candidate rows are
  non-gating unless later promoted by explicit human gate and graph
  revalidation.
- The DAG is a relationship map. Fresh deliverable state is expected in each
  deliverable-local folder: `_STATUS.md`, `MEMORY.md`, dependencies, run
  records, review files when present, semantic/lensing files when present, and
  the four-document kit.
- `TP-CHECKING-REVIEW-GAP-CLOSEOUT-001_2026-06-03` closed the local review
  surface gap for the 11 CHECKING deliverables. Existing review surfaces were
  inventoried for `DEL-02-01` through `DEL-02-05`, `DEL-06-03`, `DEL-08-04`,
  and `DEL-08-05`; new SELF_CHECK / AGENT_CHECK review surfaces were created
  for `DEL-17-01`, `DEL-17-02`, and `DEL-17-03`. No lifecycle, DAG authority,
  release, compatibility, code-compliance, or professional-reliance claim was
  made.
- Review closeout residuals after subsequent DEL-17-03 remediation:
  `DEL-06-03`, `DEL-08-04`, `DEL-08-05`, and `DEL-17-01` have clean review
  surfaces for the next human gate. `DEL-02-01` through `DEL-02-05` and
  `DEL-17-02` remain held for human disposition of AGENT_CHECK findings before
  any later `ISSUED` consideration. `DEL-17-03` RF-001 was resolved by
  human-directed revision of production documents to recognize the bounded
  native JSON package foundation implementation. `DEL-17-03` RF-002 remains
  open for DAG-005 artifact-flag refresh or owning-workflow disposition.

## Immediate Next Actions

1. Read `NEXT_INSTANCE_PROMPT.md`.
2. Read `_COORDINATION.md` and this `NEXT_INSTANCE_STATE.md`.
3. Read `SOFTWARE_DECOMP`, `docs/TYPES.md`, and needed `DAG-005` surfaces.
4. Run
   `python3 tools/coordination/list_deliverable_status.py --dag DAG-005 --format table --summary`.
5. Review
   `execution/_Aggregation/TP-CHECKING-REVIEW-GAP-CLOSEOUT-001_2026-06-03/`.
6. Propose exactly one next bounded tranche. Recommended next tranche:
   human-disposition review for the six held CHECKING deliverables
   (`DEL-02-01` through `DEL-02-05`, `DEL-17-02`) plus a coordination
   disposition or refresh proposal for `DEL-17-03` RF-002. No lifecycle
   transition should occur unless the human separately approves the gate
   action.


## Do Not Change Without Explicit Human Approval

- lifecycle `_STATUS.md` files;
- DAG artifacts or dependency registers;
- candidate-edge promotion or graph authority;
- release records or acceptance records;
- professional, certification, sealing, authentication, code-compliance, or
  release-readiness-for-reliance claims.
