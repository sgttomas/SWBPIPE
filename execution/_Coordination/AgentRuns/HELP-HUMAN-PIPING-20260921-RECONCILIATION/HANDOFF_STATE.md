# Handoff state — whole-corpus reconciliation

A cold session starts here. Read in order: this file, `OWNER_DIRECTIONS.md`,
`PLAN.md`, `WORK_GRAPH.json`, then the D-73 packet and ruling in
`../../_DECISIONS/`. After R0 opens, the evidence folder's `RESUME.md` and
`RUN_STATE.jsonl` take over as the phase cursor.

## Where things stand (2026-09-21)

- Piping development is paused by the owner until this run is done.
- Frozen source: `00115c71931bcae79909602d653740d3bb72dfa1`.
- D-73 is **RULED** and merged (PR #837). R5/R6 need a separate owner
  authorization.
- R0 (ruled conventions) and R1 (598-capability inventory, indexes, routing)
  are merged to `main` (PR #839, merge `d50d8883e`), after independent review
  with seven backchecks.
- R2 is **closed** (2026-09-22) on branch
  `claude/piping-recon-r2-w1-20260921`: all 102 deliverables are encoded,
  validated and verified, the W1/W2/W3/CROSS_WAVE resolutions are adopted,
  and the corpus-wide batch passes with 0 findings. The close-out is
  `RUN/WAVES/W3/W3_ASSESSMENT.md`; the independent review is
  `_run_records/returns/R2-W3-CLOSE-REVIEW_return.md` (PASS at backcheck 1).
  Launch messages are in `_run_records/launches/`; manager records in
  `_run_records/<WAVE>-<PKG>-MANAGER/`.
- Verbatim review returns moved from `returns/` to `_run_records/returns/`
  (RELOCATE event); older records, including the D-73 ruling record, cite the
  old path.

## Next actions

Status (2026-09-22): R2 closed; R2 PR to `main` next, then R3.

1. Open the R2 PR from this branch; merge under the standing Git
   authorization once required CI passes (the independent reviews cover the
   candidate: R2-AMEND, R2-W2-GATE, R2-W3-CLOSE).
2. R3 synthesis (writes only in the run folder), from the sealed ledgers read
   with the four adopted resolutions files
   (`WAVES/CROSS_WAVE/ALL_WAVES_RESOLUTIONS_COMBINED.csv`):
   final unmapped capability set, duplicate ownership, stale evidence,
   Remaining and lifecycle defects, clustering of non-aligned claims by cause
   and tier; resolve the contested corpus clusters listed in
   `W3_ASSESSMENT.md` (SR-1, DEC-009, unit vocabulary, F1 on CONTEXT, export
   plan, in-scope REQ tier, acceptance-workflow cause) or route them to R4;
   method notes (the `.sNN` batch blind spot); separate scope-change,
   code-fix-candidate and engineering-authority outputs.
3. R3/R4 PR with independent review, then the R4 owner gate (owner items in
   `W2_GATE_ASSESSMENT.md` and `W3_ASSESSMENT.md`, plus SEMANTIC_READY status
   advance, rerun-rule reading, PKG-12 post-gate question). R5/R6 need a
   separate owner authorization.

## Departures from the approved plan

- `PLAN.md` Step 1 listed a coordination notice
  `NOTICE_2026-09-21_RECONCILIATION_ACTIVATION.md`. It was not written: the
  owner has already notified and paused the other sessions, and D-73 Item 5b
  allows notices only when another loop is affected. Reverse by writing the
  notice if a loop turns out to be affected.

## Cautions

- Do not touch `codex/swbpipe-continuation-20260919` or any other session's
  worktree. Never prune worktrees.
- Receipt-161 exists on that unmerged branch; number this run's receipts to
  avoid it.
- The concurrency cap is 16 live agents including Agent 0; check it before
  each sub-batch.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
