# TASK Launch Brief — fresh remediated complete-node review

RequestedBy: `WI-PKG07-DEL0706`

RunID: `HELP-HUMAN-PIPING-20260820-R7-ENGINEERING`

ParentInstanceID: `WI-PKG07-DEL0706`

ChildInstanceID: `TASK-REVIEW-2`

PackageID / DeliverableID: `PKG-07` / `DEL-07-06`

TaskSkill: `software-code-review`

Objective: independently review 100% of `../FROZEN_NODE_DIFF_V2.json`, verify
every content hash, confirm attempt-1 P2 identity false-pass is fully closed,
and review all product/evidence/status/manager artifacts for correctness,
containment, and claim discipline.

Required P2 checks: restored SQLite row ID, restored model project ID, and
solved `model_ref` must each be fail-closed against
`project:packaged-edited-load-smoke`; the focused Rust test must assert the
solve model ref; no evidence-only comparison may substitute for the guard.

Reads: root/project/TASK/software-code-review instructions; software profile;
activation/work graph/amendment; DEL SOW/context/status; v1 failed return; v2
snapshot; complete changed contents/diffs; check and actual bundle evidence.

Allowed tools: read-only file/Git and deterministic read-only workflow checks.
Allowed write targets: none. No modification, staging, commit, delegation, or
sibling contact. Return only to WORKING_ITEMS.

Acceptance: PASS with no actionable finding. Otherwise exact findings with
file/line/severity and remediation.
