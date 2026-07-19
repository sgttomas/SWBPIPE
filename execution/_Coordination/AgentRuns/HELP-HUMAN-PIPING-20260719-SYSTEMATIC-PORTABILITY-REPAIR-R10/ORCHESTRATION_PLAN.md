# ORCHESTRATION PLAN — Systematic Portability Repair R10

- RunID: `HELP-HUMAN-PIPING-20260719-SYSTEMATIC-PORTABILITY-REPAIR-R10`
- Parent: `HELP_HUMAN`
- Manager: `HELPS_HUMANS`
- Posture: `SERIAL_AUTHOR_THEN_ADVERSARIAL_VERIFY`
- Selection authority: explicit owner approval of the systematic portability-repair plan
- Source commit: `dca98da8527fc118d9bbdcc1e88ccdc7c96b863d`
- Working anchors: `{REPO_ROOT}` and `{WORKING_ROOT}`
- Plan version: `1`

## Objective

Replace the brittle GEN8 aggregate/path baseline with one shared structural
surface-role policy. Active control surfaces must be portable; governed
evidence may retain exact provenance; historical exceptions must be explicit,
hash-bound, validated, visible, and non-expanding.

## Authority and Boundaries

- Cleanup merge `525ef0903e68b536ff5b22f985263ca737a67986` is an ancestor of the source commit.
- Receipt 56 is the accepted cursor. Receipt 57 may be appended only after a successful repair and independent `COMMIT-SAFE` verdict.
- R3, R7, R8, P1, R9, and every prior reproduction bundle/run are immutable.
- D-55 / DEC-088 is historical one-off authority and is not reused.
- DEL-09-04 `_STATUS.md`, `MEMORY.md`, candidate brief, decisions, decomposition, sibling projects, and domain-engine paths are outside the write fence.
- No stage, commit, push, merge, release, promotion, publication, acceptance, or external effect is authorized.

## Work Graph

Exactly two Agent 2 children execute serially. The author is the sole
Bash-bearing repo-root integration owner and owns all repair writes. Only after
the author terminalizes does a fresh read-only adversarial verifier execute.
Neither child delegates.

## Fan-In Gates

1. Shared `CONTROL` / `EVIDENCE` / `UNCLASSIFIED` policy is consumed by self-check, coordination checking, and path-anchor validation.
2. Unknown AgentRuns artifacts fail closed; valid hashed historical exceptions are facts, not actionable findings.
3. Exact full-tree path pins and aggregate live severity assertions are removed and replaced by semantic invariants.
4. Prior managed history remains byte-identical.
5. Narrow raw-evidence Git attributes and their temporary-repository regression pass.
6. Focused tests, full harness and piping tests, registered validators, containment checks, and one applicable five-surface sweep pass without installs, downloads, or network.
7. Every R10 record is terminal before the final parent-level self-check; the completed run introduces no portability violation.
8. The verifier returns `COMMIT-SAFE` before Receipt 57 is appended.

## Failure Posture

Any authority, containment, classifier, ledger, validation, immutability, or
verification discrepancy terminalizes R10 as `FAIL` or `BLOCKED`. Receipt 57
is not appended on failure. DEL-09-04 state remains unchanged.
