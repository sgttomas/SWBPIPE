# Handoff state — whole-corpus reconciliation

A cold session starts here. Read in order: this file, `OWNER_DIRECTIONS.md`,
`PLAN.md`, `WORK_GRAPH.json`, then the D-73 packet and ruling in
`../../_DECISIONS/`. After R0 opens, the evidence folder's `RESUME.md` and
`RUN_STATE.jsonl` take over as the phase cursor.

## Where things stand (2026-09-21)

- Piping development is paused by the owner until this run is done.
- Frozen source: `00115c71931bcae79909602d653740d3bb72dfa1`.
- D-73 is **RULED** and merged to `main` (PR #837). R5/R6 need a separate
  owner authorization.
- R0 is ruled (`R0_CALIBRATION/R0_RULING.md`, with addendum). R1 is complete:
  598 capabilities, mechanical indexes, authority map, routing.
- Branch `claude/piping-recon-r0-20260921` carries R0 and R1 and is in
  independent review (backchecks 1–4 repaired). No R2 worker has been
  dispatched. No deliverable, code, test, DAG or lifecycle surface has been
  written.

## Next actions

1. Close the R0/R1 PR review, then push, open the PR, pass CI and merge.
2. Stale check, then R2 wave 1 per the evidence folder's `WAVE_PLAN.md`
   (PKG-07 and PKG-16; two managers, four workers; then two verifiers at
   double sampling). Gate before scale-out; show the owner the canonical
   table with wave 1 results.

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
