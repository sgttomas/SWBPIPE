# N1 CHANGE pre-commit closeout failure

- Final status: `FAILED_CLOSEOUT_PRODUCT_OUTPUTS_EXCLUDED`.
- Detection layer: CHANGE pre-commit gate.
- Command: `git diff --cached --check`.
- Finding: one extra terminal blank line in each of six staged N1 immutable/derivative files:
  - `execution/_DAG/DAG-010/APPROVAL_RECORD.md`
  - `execution/_DAG/DAG-010/DAG-010_APPROVAL_REVIEW_PACKET.md`
  - `execution/_DAG/DAG-010/HANDOFF.md`
  - `execution/_DAG/DAG-010/PROPOSAL_RECORD.md`
  - `execution/_DAG/DAG-010/PROVENANCE.json`
  - `execution/_Reconciliation/DeliverableConcordance/SCOPED_SCA009_DEL0709_CURRENT_AUTHORITY_2026-08-21/RUN_BASIS.md`
- Constraint: the affected derivative package contents are hash-pinned. The owner stop rule forbids repair, re-hashing, regeneration, or rerun in this iteration.
- Disposition: preserve `N1_RETURN.md`, amendments, materialization record, and all other N1 AgentRuns evidence as historical execution evidence; exclude every N1-produced project output from the candidate.
- Excluded outputs: the DEL-07-09 scaffold/coverage folder, DAG-010 directory, scoped Reconciliation directory, DAG pointer change, and coupled release-readiness test change. `_DAG/_LATEST.md` and `tests/test_release_readiness_script.py` are restored to `HEAD`.
- Outstanding obligations: PREPARATION scaffold for DEL-07-09; current DAG successor rebuild from live DAG-009; DEL-07-09 dependency extract; targeted current-authority Reconciliation; and formal pre/post `AUDIT_DECOMP` (still parked for lack of a clearly applicable in-session runner).
- No N2 or N3 file was changed by this exclusion.
