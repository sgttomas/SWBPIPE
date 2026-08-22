# TASK-PKG12-REVIEW-001 Attempt 2 Launch Brief

- RequestedBy: `WI-PKG12-001`
- RunID: `HELP-HUMAN-PIPING-20260821-LOCAL-FIRST-RUNTIME`
- ParentInstanceID: `WI-PKG12-001`
- ChildInstanceID: `TASK-PKG12-REVIEW-001-A2`
- AgentType: fresh read-only Agent 2 applying `software-code-review`; do not delegate.
- PackageID / DeliverableID: `PKG-12` / `DEL-12-01`
- Objective: review 100% of repaired integrated diff, including direct backcheck of attempt-1 P1, correctness/security/compatibility/test/scope review, and exact hash verification.
- Base: `1b375af4f1219ecfc00fc2755854aa7fd4220901`.
- FrozenInventory: `../../FROZEN_HASH_INVENTORY_ATTEMPT_002.md` (13/13 paths).
- AcceptedEvidence: implementation `RETURN_AFTER_AMENDMENT_v3.md`; pinned full Piping PASS; host DEC-025 PASS; prior failed review attempt preserved.
- DeclaredReads: all 13 changed files, complete diff, current callers/writers, governing brief/status/SOW, check evidence.
- AllowedTools: read/search/read-only shell; no writes.
- AllowedWriteTargets: none.
- AcceptanceCriteria: attempt-1 malformed-intent bypass is closed end-to-end; all current adapter/result/report route consumers invoke the metadata-only guard or fail closed; native report persistence validates evidence; no payload inspection/network/cloud/telemetry/root selection; compatibility/tests/scope adequate; no actionable findings.
- RequiredReturn: PASS or prioritized findings; exact verified hashes; explicit 100%-diff coverage; P1 backcheck; residual risk; actual model identity.
