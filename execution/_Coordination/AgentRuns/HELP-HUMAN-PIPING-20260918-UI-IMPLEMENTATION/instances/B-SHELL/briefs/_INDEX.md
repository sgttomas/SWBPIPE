# Lane B-SHELL: sealed briefs of the manager's children

Run `HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION`, lane B-SHELL (the shell lane of Tranche B). The lane manager (WORKING_ITEMS, Type 1) writes each child's brief here, hashes it and lists it before the child is launched. Every child is a Type 2 TASK launched with the Claude Code `Agent` tool (general-purpose type, background) in the lane's worktree on branch `codex/swbpipe-b-shell-20260918`; a child does not delegate and runs no state-changing git command; the manager integrates and commits. Returns are retained verbatim under `../returns/` with their SHA-256 and the model that ran. ROOT's records for this lane (addenda, review briefs, reviewers' returns) are in `../../../lanes/B-SHELL/`; the manager does not write there.

| Brief | SHA-256 | Sealed | Model requested | Role | Return |
|---|---|---|---|---|---|
| `B2-STATE.md` | `a938b135e34a7e9e28531274a20bc3f24ad1d0a52de657ab1a57405871442d75` | 2026-09-19T00:24Z | `fable` (Claude Fable 5.1) | TASK implementer; slice B2, state extraction from `App.tsx`, in three stages with a return at each | `../returns/B2-STATE_STAGE1_RETURN.md`, `../returns/B2-STATE_STAGE2_RETURN.md`, `../returns/B2-STATE_STAGE3_RETURN.md` |

## Tools the manager wrote

| File | SHA-256 | Purpose |
|---|---|---|
| `../tools/b2_move_audit.mjs` | `73dea9e52b6d2d3e589c0f78ec7238adf23e3def8bb52f7c74d8918c19957a21` (revision 2, 2026-09-19T00:42Z; revision 1 was `193b5c3c30ae80a8d258a0bc4d2952318e3d0a58667b9ca00a21b8cd392fe7ed`, in commit `28f956d95`) | Slice B2's move audit: parses the base `App.tsx` and a candidate with the TypeScript compiler API and reports, statement by statement, what arrived identical, changed, missing or new, and whether the effect sequence holds. Named in the B2-STATE brief; the reviewer can run it on the slice's diff range. Revision 2 changes the report only, after the child's stage-1 return named a blind spot: the tool compares no import statement and nothing a module writes above its imports. It now prints that text for every new module (its section 5) and prints a new statement with its leading `export`. The comparisons are unchanged: sections 1 to 3 of the stage-1 report are identical under both revisions. |

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
