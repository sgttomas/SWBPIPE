# Sealed Brief — A2-PR550-PATH-ANCHOR-EXCEPTION-AUTHOR

RequestedBy: `HELP_HUMAN`

RunID: `HELP-HUMAN-PIPING-20260810-DEL0904-OWNER-GATES`

ParentInstanceID: `WORKING_ITEMS-A1-PR550-PATH-ANCHOR-REPAIR`

ChildInstanceID: `A2-PR550-PATH-ANCHOR-EXCEPTION-AUTHOR`

Agent form: fresh non-delegating ephemeral Agent 2

PackageID: `PKG-09`

Bounded integration scope: PR #550 historical-control portability repair

Repository resolution: resolve `REPO_ROOT` with
`git rev-parse --show-toplevel`; set `WORKING_ROOT` to
`${REPO_ROOT}/projects/chirality-piping`.

Accepted basis: branch `codex/piping-del0904-owner-gates-20260810` at
`1613ebfae29634242cf140d55b7309e945ba43f1`; frozen V1 brief SHA-256
`d937f558ee2b38d6d1458790de5efd2d987d9a2ce584ba9cfec2869bebca4a41` and
Git blob `f52c0bf13f4ece6ed2631a0a3c7c941c7a6451d5`.

## Objective

Preserve the completed V1 sealed brief byte-for-byte and add exactly one
hash-bound `control_path_exception` to the project portability policy so the
two historically accurate machine-local anchors are acknowledged as immutable
control evidence. Preserve the tokenized V2 terminal-verifier chain and all
owner-ruling semantics.

## Declared reads

- root and project `AGENTS.md`
- `tools/validation/README.md`
- `tools/validation/validate_path_anchors.py`
- `tools/practitioner_harness/surface_roles.py`
- `APPLICATION_PLAN_AMENDMENT_V5_VERIFIER_V2.md`
- `MANAGER_VALIDATION_APPLICATION.md`
- the V1 and V2 launch/status/return records
- `projects/chirality-piping/validation/portability_policy.json`

## Allowed tools

Read-only shell/Git, SHA-256 and Git-blob hashing, JSON parsing, repository
validators, and `apply_patch`. Do not delegate.

## Allowed write targets

- `projects/chirality-piping/validation/portability_policy.json`
- this instance's `RETURN.md`
- this instance's `STATUS.json`

## Required policy entry

- path: the repo-relative V1 `LAUNCH_BRIEF.md` path
- SHA-256: `d937f558ee2b38d6d1458790de5efd2d987d9a2ce584ba9cfec2869bebca4a41`
- entry type: `control_path_exception`
- role: `CONTROL`
- reason: preserve the completed V1 verifier brief's actual machine-local
  checkout and scratch anchors; tokenized V2 supersedes it as terminal basis
- authority: direct owner PR #550 repair direction plus Amendment V5's frozen
  V1/tokenized V2 treatment

## Acceptance criteria

1. V1 launch brief remains byte- and Git-blob-identical.
2. Policy remains valid schema-1 JSON and gains exactly the one required
   exception, with no other semantic change.
3. `validate_path_anchors.py --text .` passes with zero findings and semantic
   invariants zero except acknowledged-control telemetry.
4. Relevant policy and path-anchor unit tests pass without repository-side
   caches or bytecode.
5. No staged, ignored, or unrelated path appears; Receipt 99 remains the
   cursor and is not edited.

## Escalation and exclusions

Return `HOLD` without repair if any frozen identity differs, if the exception
mechanism does not clear both findings, or if another portability defect
appears. Do not edit any historical V1/V2 record, receipt, deliverable,
register, owner ruling, evidence, lifecycle, release, or other path. Do not
stage, commit, push, merge, fetch, rebase, reset, clean, or delete anything.
