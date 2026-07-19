# HELPS_HUMANS Return — Systematic Portability Repair R10

## Terminal Result

- Manager result: `SUCCESSFUL_FAN_IN`
- Objective result: `PASS`
- Source commit: `dca98da8527fc118d9bbdcc1e88ccdc7c96b863d`
- Cleanup merge `525ef0903e68b536ff5b22f985263ca737a67986`: confirmed ancestor
- Final verifier verdict: `COMMIT-SAFE`
- Git disposition: `COMMIT_READY_NOT_STAGED`

The repair replaces GEN8's aggregate severity and exact full-path pins with a
shared structural portability policy. Active controls must be portable,
standard evidence may retain exact provenance, and unknown AgentRuns artifacts
fail closed. Seven historical cases are explicit, whole-file hash-bound, and
validated against current path hits.

## Managed Execution and Adversarial Closure

Exactly two Agent 2 identities ran serially. The author was the sole
Bash-bearing integration owner. The fresh verifier's v1 `BLOCK` correctly
found broad evidence-token inference; Amendment 002 removed it and added the
complete adversarial corpus. The same verifier identity then performed a fresh
v2 review and returned `COMMIT-SAFE`. All v1/v2/v3 returns and statuses remain
preserved.

## Final Invariants

- Piping: `unacknowledged_control=0; active_unclassified=0; policy_issues=0; acknowledged_control=3`.
- Exact standard evidence shapes are case-sensitive; misleading, near-match, unknown-extension, and non-normalized artifacts fail closed.
- Control precedence applies even beneath `_run_records/`.
- Historical/non-active absolute paths remain observable facts and are not converted into an aggregate gate.
- Ledger contains exactly four P1 evidence overrides and three control exceptions; all target hashes, roles, reasons, authorities, and current hits validate.
- R3/R7/R8/P1/R9, every prior reproduction bundle, DEL-09-04 status/memory, decisions, decomposition, sibling projects, and domain engines have no Git diff.

## Validation

- Focused author v2 suite: `81 passed`.
- Full practitioner-harness plus path-validator suite: `316 passed`.
- Full piping pytest: `505 passed`.
- Independent verifier reran and confirmed the full suites.
- Claims, path-anchor, receipt, instruction-entrypoint, JSON/JSONL, containment, `git diff --check`, and `git diff --cached --check`: PASS.
- Temporary-Git proof: raw reproduction output ending `\n\n` passes the narrow attributes rule; malformed authored Markdown fails.

## Sweep Disposition

- Superseded/non-admitted: `validation/evidence/sweeps/SWEEP_20260719T193438Z_dca98da8527f-dirty.json`, SHA-256 `367bc963039af0c6b74aec19273e3e781fd395a3ef64ab4c57c798c4d9dcd564`, invalidated by the verifier-v1 remediation.
- Final/admitted: `validation/evidence/sweeps/SWEEP_20260719T195631Z_dca98da8527f-dirty.json`, SHA-256 `4ac56dda2b2c5169f083f5f010166492aa0586fda622bd618fd5b9282324fa4a`, overall PASS with all five surfaces PASS.
- No implementation, test, policy, documentation, or Git-attribute file changed after the admitted sweep finished.

## Changed-Path Inventory

The commit-ready scope is exactly:

- 11 harness/validator implementation, documentation, and test paths under `tools/practitioner_harness/**` and the declared path-validator files;
- `projects/chirality-piping/.gitattributes`;
- `projects/chirality-piping/validation/portability_policy.json`;
- the two R10 sweep JSON files with the dispositions above;
- this R10 managed-run tree; and
- `projects/chirality-piping/loop/LOOP_RECEIPTS.md` Receipt 57 only.

There are no other authorized changed paths. The exact pre-parent-close list
was 37 paths; manager `RETURN.md`, `HANDOFF_STATE.md`, and Receipt 57 bring the
terminal candidate to 40 paths.

## Key Hashes

- shared classifier: `bb046f041ed33e0d7b4cee797aa0da52920cc5fd98eb95fbb087f795b580f2be`
- adversarial tests: `e30f16608ffad2e3f3aa90490190215d631e278ff042d27c83d0fd3b025e5b9a`
- portability ledger: `b95d1a12db73557224148719209805ec3c1c9641c4a3857bce440a1aee82a9c9`
- Git attributes: `789c2961a1eb692099c4d90b912de046de219a3e524821f3e38954e4ad65e891`
- verifier v2 return: `ea747170d2d2283453d716511f2c8571667e66a79e1cfa16e9b6a1f49de4bca9`

## Boundaries and Next Owner

Receipt 57 records repair execution only. DEL-09-04 remains `IN_PROGRESS`; its
clean-reproduction item remains open. R11 may begin only from a clean committed
repair source under its separate sealed authority. No stage, commit, push,
merge, reproduction, acceptance, lifecycle or stage change, promotion,
release, publication, or external effect occurred. Return commit-ready scope
to HELP_HUMAN; Git closeout belongs to CHANGE.

## Model Attribution

HELPS_HUMANS manager and both Agent 2 identities used the inherited Codex
runtime capability; exact model strings were not exposed. No model override or
mid-task substitution occurred.
