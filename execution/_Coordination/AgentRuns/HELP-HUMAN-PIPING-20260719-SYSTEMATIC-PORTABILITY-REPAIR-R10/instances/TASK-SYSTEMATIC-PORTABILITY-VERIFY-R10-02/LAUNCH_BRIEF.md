# TASK Launch Brief — Systematic Portability Adversarial Verifier R10-02

- Parent: `HELPS_HUMANS`
- RunID: `HELP-HUMAN-PIPING-20260719-SYSTEMATIC-PORTABILITY-REPAIR-R10`
- InstanceID: `TASK-SYSTEMATIC-PORTABILITY-VERIFY-R10-02`
- Construction: fresh ephemeral bounded Agent 2 generalist under `AGENT_TASK.md`
- WorkingRoot: `{WORKING_ROOT}`
- ScopePath: `{REPO_ROOT}`
- ApplyEdits: `false`
- Delegation: forbidden
- Dependency: terminal PASS from `TASK-SYSTEMATIC-PORTABILITY-AUTHOR-R10-01`

## Objective

Independently and adversarially verify the complete R10 candidate. Recompute
facts; do not rely on the author's conclusions. Return `COMMIT-SAFE` only if
the systematic invariant, write containment, tests, sweep, and protected
history all pass.

## Read Scope

Repository root, limited to applicable instructions, complete R10 candidate
delta, relevant harness/validator callers and tests, policy ledger targets,
the single R10 sweep, Receipt 56, and protected prior-history comparisons.

## Write Authorization

No implementation or repair writes. The only permitted writes are this
instance's `RETURN.md` and `STATUS.json` within the R10 tree.

## Required Adversarial Checks

1. Enumerate the candidate delta from Git and reject every path outside the approved author fence plus verifier records and exactly one new sweep JSON.
2. Confirm prior R3/R7/R8/P1/R9 managed records and all prior reproduction bundles have no Git diff.
3. Inspect shared `SurfaceRole` usage in self-check, coordination checking, and path-anchor validation; reject duplicated or divergent classification logic.
4. Prove control precedence, structural standard roles, unknown AgentRuns fail-closed, active-control boundary, and continued historical observability.
5. Validate the ledger schema and exact seven entries: four P1 role overrides and three control exceptions. Recompute whole-file hashes and reject path/type/reason/authority/hit/duplicate/stale drift.
6. Confirm GEN8 has zero unacknowledged active control paths, zero active absolute-path unclassified artifacts, and zero policy issues; valid exceptions are facts, not REVIEW.
7. Confirm aggregate severity-count and exact full-tree path baselines were removed and no equivalent brittle pin was introduced.
8. Re-run focused tests, full practitioner-harness tests, full piping tests, self-check, claims/path/receipt/instruction-entrypoint/JSON+JSONL/containment/Git-diff checks as feasible.
9. Recompute the temporary-Git raw-evidence proof: reproduction stdout/stderr ending with `\n\n` passes `git diff --check`; malformed authored Markdown fails.
10. Validate exactly one R10 sweep JSON, overall `pass`, exactly five expected surfaces all `pass`, and no install/download/network evidence.
11. Confirm all R10 controls use placeholders or repo-relative paths and the completed R10 record set introduces no portability violation.
12. Determine whether Receipt 57 is eligible. The verifier must not append it.

Return exact evidence, discrepancies, hashes, and either `COMMIT-SAFE` or
`BLOCK`. Do not stage, commit, push, merge, or create external effects.
