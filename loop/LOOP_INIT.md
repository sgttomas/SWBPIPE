# Piping development — session entry

Resolve `REPO_ROOT` with `git rev-parse --show-toplevel` and `WORKING_ROOT` as
`{REPO_ROOT}/projects/chirality-piping`. Read root `AGENTS.md`, the active role
instructions and `{WORKING_ROOT}/AGENTS.md`.

## Recurrent instructions

The standing development procedure is in the project `AGENTS.md`. Read the
current committed standing plan for its procedural pointer and retained
boundaries: enumerate direct `loop/WORKPLAN_*.md` entries in committed `HEAD`,
sort paths bytewise and select the last. Require a regular blob with mode
`100644`, and read its committed bytes with `git show HEAD:<selected-path>`.
If selection or reading fails, report the failure rather than falling back to
an older plan. Uncommitted candidates and `.archive/` copies are not selectable.

## State pointers and discovery

Use the owner's assignment to identify the owning run. Read its work graph,
latest handoff, recorded directions and applicable plan/specifications. Inspect
named branch and worktree state, including unmerged work, before proposing new
implementation. When no run is identified, validate `loop/LOOP_RECEIPTS.md` with
`tools/validation/validate_piping_loop_receipts.py --repo-root .`, then use its
latest applicable pointers to discover the current run records and coordination
notices. A receipt is navigation and history, not an instruction to repeat its
completed work. Report the proposed continuation; do not restart historical
work by default. A failed receipt check blocks reliance on that cursor, not
independent work with a verified basis.

## Steering

Apply the owner's current directions and any per-run steer, including an
owner-supplied handoff prompt, the launcher's steer or the owning run's direction
record. Keep specific priorities, lane and worktree identities,
active handoff paths, pause conditions and next actions in those state or
steering records, not in these recurrent instructions. Distinguish a recorded
owner direction from the previous agent's recommendation.

Return a short orientation and the proposed next bounded work. Apply the
project's discovery, strategy, testing, review and integration requirements.
Continue under existing authority and an applicable approved strategy; bring
the owner only consequential choices or unresolved authority conflicts. A
handoff's procedural habit does not create a new owner gate.
