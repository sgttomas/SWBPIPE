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
- R2 wave 1 (PKG-07, PKG-16) is dispatched on branch
  `claude/piping-recon-r2-w1-20260921`: two managers, launch messages in
  `_run_records/launches/`. Records go to `_run_records/W1-<PKG>-MANAGER/`.
- Verbatim review returns moved from `returns/` to `_run_records/returns/`
  (RELOCATE event); older records, including the D-73 ruling record, cite the
  old path.

## Next actions

1. On manager returns: check the thin summaries, then launch one fresh
   verifier per package (R2-VERIFIER brief, DOUBLE sampling).
2. Gate per `WAVE_PLAN.md`; owner checkpoint with the canonical table,
   the 13-deliverable disclosure, and any reruns or contested rows.

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
