# TASK Launch Brief — fresh corrected complete-node review

RequestedBy: `WI-PKG07-DEL0706`

RunID: `HELP-HUMAN-PIPING-20260820-R7-ENGINEERING`

ParentInstanceID: `WI-PKG07-DEL0706`

ChildInstanceID: `TASK-REVIEW-3`

PackageID / DeliverableID: `PKG-07` / `DEL-07-06`

TaskSkill: `software-code-review`

Objective: independently review 100% of `../FROZEN_NODE_DIFF_V3.json`, verify
every hash, confirm Amendment 2 changed only the five declared terminal blank
lines, and ensure the complete corrected node remains correct, contained, and
consistent with the prior product/check/review evidence.

Reads: root/project/TASK/software-code-review instructions; activation/work
graph; Amendments 1-2; v1-v3 snapshots; attempts 1-2 returns; complete current
tracked and untracked contents/diffs; staged and unstaged state read-only.

Required checks: all v3 hashes; 100% content/diff coverage; no blank line at
EOF in the five normalized files or this brief; staged-versus-worktree
awareness; `git diff --check`; `git diff --cached --check` noting the index may
still contain CHANGE's pre-normalization staged bytes; JSON/JSONL parsing;
scope containment; product/evidence/claim consistency.

Allowed tools: read-only file/Git and deterministic read-only workflow checks.
Allowed write targets: none. No modification, staging, unstaging, commit,
delegation, publication, receipt, or sweep. Return only to WORKING_ITEMS.

Acceptance: PASS with no actionable finding, or exact file/line/severity
findings and remediation.
