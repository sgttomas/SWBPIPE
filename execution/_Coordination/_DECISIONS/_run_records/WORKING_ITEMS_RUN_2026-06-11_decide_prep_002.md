# WORKING_ITEMS Run Record — D-07 Direct Ruling + D-09 Packet (TP-DECIDE-PREP-002)

- Date: 2026-06-11
- Agent: WORKING_ITEMS (Type 1 persona); one TASK (Type 2) worker for the
  D-09 packet with a sealed single-file write scope. Continuation of the
  same-day decision session (see
  `WORKING_ITEMS_RUN_2026-06-11_decide_prep_001.md`).

## Part 1 — D-07 ruled directly (human authority, no packet)

The human project authority ruled D-07 directly in session: maintainer
quorum is one (sole developer/maintainer = release authority); external
contributions are not accepted at this time; no contributor legal
mechanism stood up. Recorded as `DEC-027`; register row D-07 → RULED
(ruled directly, no packet); new register row `D-07b` gates any future
contribution intake; R5 "IP contribution process" milestone mapping
deferred to the R5 gate alongside D-12. Committed `0a47f307d`.

## Part 2 — D-09 packet prepared (agent; PROPOSAL only)

`D-09_native_package_container.md` (150 lines, zero NUL bytes, pinned at
HEAD `4b2c0619e`, D-10 exemplar structure). Fan-in confirmed the worker's
only write was the packet. Key FACTs surfaced: no per-project file exists
today (one app-wide SQLite db holds all projects as rows,
`apps/desktop/src-tauri/src/lib.rs`); the
`schemas/project_persistence.schema.yaml` `PhysicalContainer` $def
already pins canonical-JSON-as-truth and store-as-projection; PKG-17
contracts the logical export-package shape but never pins a physical
archive form (TBD-17-03-004); R2's exit criterion does not require a
project file (timing is a human pacing call). Options: A — per-project
SQLite file as container; B — single-file canonical-JSON envelope as
container and transport form; C — zip/directory package per PKG-17
manifests; D — defer to Phase E. Recommendation (PROPOSAL): B now with a
named D-09b archive follow-up at Phase E.

Register row D-09 → AWAITING_RULING; completion-plan §2 row updated.

## External-scope note

An untracked human draft `plans/PLAN_2026-06-12_caepipe_external_oracle_feedback_loop.md`
appeared in the worktree during this tranche; recorded and bypassed per
the external-scope rule (human file; not staged, not modified).

## Boundary review

Packet is PROPOSAL-only; invented examples; no protected content; no
network; no release/professional/certification/code-compliance claims.
Rulings remain human records per K-AUTH-1.
