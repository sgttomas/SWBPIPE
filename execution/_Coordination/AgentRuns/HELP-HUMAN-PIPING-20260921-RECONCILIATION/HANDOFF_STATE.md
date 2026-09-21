# Handoff state — whole-corpus reconciliation

A cold session starts here. Read in order: this file, `OWNER_DIRECTIONS.md`,
`PLAN.md`, `WORK_GRAPH.json`, then the D-73 packet and ruling in
`../../_DECISIONS/`. After R0 opens, the evidence folder's `RESUME.md` and
`RUN_STATE.jsonl` take over as the phase cursor.

## Where things stand (2026-09-21)

- Piping development is paused by the owner until this run is done.
- Frozen source: `00115c71931bcae79909602d653740d3bb72dfa1`.
- Drafted on branch `claude/chirality-piping-reconciliation-7f7e70`: the D-73
  packet, the register row (`AWAITING_RULING`), the project profile
  `docs/RECONCILIATION_PROFILE.md`, and this run record.
- Nothing has been dispatched. No deliverable, code, test, DAG or lifecycle
  surface has been written.

## Next actions

1. Owner rules D-73 (Item 4: R0–R4 recommended, or R0–R6).
2. Agent 0 records the ruling, the register flip and `DEC-110`, then launches
   the one fresh read-only reviewer over the complete activation diff.
3. Checks: `harness-self-check`, `harness-pytest`, the claims lint and the
   receipt validator; then PR, CI and merge to `main`.
4. R0 from the merged `main`, per `PLAN.md` Step 2.

## Cautions

- Do not touch `codex/swbpipe-continuation-20260919` or any other session's
  worktree. Never prune worktrees.
- Receipt-161 exists on that unmerged branch; number this run's receipts to
  avoid it.
- The concurrency cap is 16 live agents including Agent 0; check it before
  each sub-batch.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
