# TASK Launch Brief — complete-node software code review

RequestedBy: `WI-PKG07-DEL0706`

RunID: `HELP-HUMAN-PIPING-20260820-R7-ENGINEERING`

ParentInstanceID: `WI-PKG07-DEL0706`

ChildInstanceID: `TASK-REVIEW`

PackageID: `PKG-07`

DeliverableID: `DEL-07-06`

TaskSkill: `software-code-review`

## Objective

Perform a fresh, read-only review over 100% of the frozen complete node diff.
Review the Rust product/test implementation, actual packaged-binary proof,
deliverable status/memory/run record, SMOKE entry, and all manager AgentRuns
artifacts listed in `../FROZEN_NODE_DIFF.json`. Return PASS with no actionable
finding, or exact actionable findings with file/line/severity and remediation.

## Accepted basis and reads

- Base: `8eea5d06d3f98dd91b56b53a2c7caec2f7ed5919`.
- Branch: `codex/piping-r7-engineering-20260820`.
- Frozen snapshot: `../FROZEN_NODE_DIFF.json`; verify every content SHA-256
  before reviewing.
- Read root/project instructions, software profile, activation/work graph,
  DEL-07-06 SOW/status/context, actual diffs, changed files, and check records.

## Review focus

- correctness of structured `350 N -> 425 N` edit;
- file-backed SQLite persistence, connection-close/reopen semantics, cleanup on
  success and error, no repository/default user-data write;
- restored solve identity/status/result assertions and false-pass prevention;
- packaged CLI argument behavior and ordinary GUI startup preservation;
- embedded invented fixture and private/protected-data boundaries;
- agreement among code, SMOKE, status, memory, run record, manager return, and
  handoff; no false GUI closure/lifecycle/release/professional claim;
- validation adequacy and exact write containment.

## Permissions and return

Allowed tools: read-only file/Git inspection and deterministic read-only checks.

Allowed write targets: none. Do not modify files, commit, stage, delegate, or
contact other agents. Return findings to WORKING_ITEMS only.

Acceptance: `PASS` with no actionable finding. Any finding requires manager
remediation, a newly frozen diff, and a fresh reviewer instance.
