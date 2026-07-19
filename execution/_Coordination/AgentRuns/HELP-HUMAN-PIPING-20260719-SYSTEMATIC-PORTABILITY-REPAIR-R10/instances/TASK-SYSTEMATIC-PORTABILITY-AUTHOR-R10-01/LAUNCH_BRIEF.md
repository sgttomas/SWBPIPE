# TASK Launch Brief — Systematic Portability Author R10-01

- Parent: `HELPS_HUMANS`
- RunID: `HELP-HUMAN-PIPING-20260719-SYSTEMATIC-PORTABILITY-REPAIR-R10`
- InstanceID: `TASK-SYSTEMATIC-PORTABILITY-AUTHOR-R10-01`
- Construction: ephemeral bounded Agent 2 generalist under `AGENT_TASK.md`
- WorkingRoot: `{WORKING_ROOT}`
- ScopePath: `{REPO_ROOT}`
- ApplyEdits: `true`
- Delegation: forbidden
- Integration ownership: sole Bash-bearing repo-root integration owner

## Accepted Basis

Owner-approved systematic portability-repair plan; source commit
`dca98da8527fc118d9bbdcc1e88ccdc7c96b863d`; cleanup merge
`525ef0903e68b536ff5b22f985263ca737a67986` is an ancestor; Receipt 56 is
latest. Prior R3/R7/R8/P1/R9 managed records and all prior reproduction bundles
are immutable. D-55 / DEC-088 cannot be reused.

## Objective

Implement one systematic surface-role and hashed-exception policy exactly as
frozen in the parent orchestration plan. Use deterministic shared code and
tests, not accumulating path/count pins or broad filename exemptions.

## Allowed Write Targets

- `tools/practitioner_harness/**`
- `tools/validation/validate_path_anchors.py`
- `tools/validation/test_validate_path_anchors.py`
- `tools/validation/README.md`
- `projects/chirality-piping/.gitattributes`
- `projects/chirality-piping/validation/portability_policy.json`
- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-SYSTEMATIC-PORTABILITY-REPAIR-R10/**`

The child may write only its own instance records inside the R10 tree; parent
manager records remain manager-owned.

## Required Implementation

1. Add shared `SurfaceRole` values `CONTROL`, `EVIDENCE`, and `UNCLASSIFIED`, consumed by self-check, coordination checking, and path-anchor validation.
2. Structurally classify standard AgentRuns controls and evidence. Control precedence is mandatory. Unknown AgentRuns artifacts fail closed.
3. Add the project portability ledger with hash-bound historical role overrides for the four P1 runtime-capture JSON files and control exceptions for D-54 owner-verbatim plus P1 `ORCHESTRATION_PLAN.md` and child `LAUNCH_BRIEF.md`.
4. Validate normalized repo-relative path, SHA-256, entry type, reason, authority, duplicates, file presence, exact role, current absolute-path hit, and stale entries.
5. GEN8 actionable findings are limited to unacknowledged control paths, absolute paths in unclassified artifacts, and invalid/stale ledger entries. Valid exceptions remain visible facts.
6. Remove exact live-path and aggregate live-severity assertions; replace them with zero-unacknowledged-control, zero-absolute-path-unclassified, and zero-stale-ledger semantic assertions.
7. Add narrow `.gitattributes` rules for reproduction-bundle `stdout/*.txt` and `stderr/*.txt` only, using `-diff -merge -text`, plus temporary-Git proof.
8. Document portability validation separately from raw-evidence preservation.

Historical overrides are a migration mechanism, never a broad basename
allowlist. Do not edit immutable historical files to make their hashes fit.

## Required Validation and Return

Run focused role/ledger/coord/path/near-match/Git-attribute tests; full
practitioner-harness pytest; full piping pytest; applicable claims, path,
receipt, instruction-entrypoint, JSON/JSONL, containment, and Git-diff checks;
and one DEC-025 five-surface sweep if existing local dependencies permit it
without installs, downloads, or network. Return exact changed paths, tests,
hashes, preserved-history proof, blockers, and a terminal verdict. Do not stage
or commit.
