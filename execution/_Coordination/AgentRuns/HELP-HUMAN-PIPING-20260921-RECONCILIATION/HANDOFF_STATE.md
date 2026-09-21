# Handoff state — whole-corpus reconciliation

A cold session starts here. Read in order: this file, `OWNER_DIRECTIONS.md`,
`PLAN.md`, `WORK_GRAPH.json`, then the D-73 packet and ruling in
`../../_DECISIONS/`. After R0 opens, the evidence folder's `RESUME.md` and
`RUN_STATE.jsonl` take over as the phase cursor.

## Where things stand (2026-09-21)

- Piping development is paused by the owner until this run is done.
- Frozen source: `00115c71931bcae79909602d653740d3bb72dfa1`.
- D-73 is **RULED** (2026-09-21): whole corpus, R0–R4 only, no seeding, the
  Item 5 surface boundary, Item 6 parameters. Ruling record, register flip and
  `DEC-110` are on branch `claude/chirality-piping-reconciliation-7f7e70`,
  not yet merged. R5/R6 need a separate owner authorization.
- Nothing has been dispatched. No deliverable, code, test, DAG or lifecycle
  surface has been written.

## Next actions

1. Launch the one fresh read-only reviewer over the complete activation diff;
   repair and backcheck findings; list any post-ruling profile change in the
   ruling record's "Changes after ruling".
2. Checks: `harness-self-check`, `harness-pytest`, the claims lint and the
   receipt validator; then PR, CI and merge to `main`.
3. R0 from the merged `main`, per `PLAN.md` Step 2.

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
