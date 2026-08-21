# Sealed launch brief — fresh read-only code review

- RequestedBy: `WORKING_ITEMS-PKG05-DEL0504-STALE-HASH`
- RunID: `HELP-HUMAN-PIPING-20260820-R9-STALE-HASH`
- ParentInstanceID: `WORKING-ITEMS-PKG05-DEL0504-STALE-HASH`
- ChildInstanceID: assigned at dispatch and recorded in `REVIEW_STATUS.json`
- Role: `TASK` (Agent 2)
- TaskSkill: `software-code-review`
- PackageID: `PKG-05`
- DeliverableID: `DEL-05-04`
- Objective: independently review 100% of the N1 frozen diff for correctness, fail-closed behavior, regression risk, native/Wasm parity, test adequacy, and write containment.
- ScopePath: frozen diff from activation base `cd823be3badd034c86390f2707dcf01952c782f0` listed in `FROZEN_DIFF_MANIFEST.md` at dispatch.
- AcceptedBasis: `ACTIVATION.md`, `WORK_GRAPH_V1.json`, `IMPLEMENTATION_BRIEF.md`, DEL-05-04 ScopeOfWork/Remaining, `DEC-020`, and the registered software workflow.
- Dependencies: implementation complete; focused checks complete; frozen diff manifest present.
- DeclaredReads: governing instructions and skill; accepted basis above; 100% frozen changed files and relevant runtime/test context.
- AllowedTools: read-only file/Git inspection and read-only test/check execution if useful.
- AllowedWriteTargets: this AgentRuns root only for the review return and status; no product, deliverable, shared, Git, or other writes.
- ExpectedOutputs: `REVIEW_RETURN.md` with finding severity, file/line evidence, scope coverage, validation assessment, and terminal `PASS` or `FAIL`.
- AcceptanceCriteria: 100% frozen-diff coverage; no actionable finding for PASS; explicit check of malformed/unsupported/stale/matching/no-claim paths and no-applied-model behavior; explicit native/Wasm corpus coverage assessment; no professional-boundary regression.
- EXCLUSIONS: no edits, fixes, staging, commits, branch operations, delegation, lifecycle/release/acceptance decisions, or expansion beyond N1.
- Escalation: any actionable finding returns FAIL to WORKING_ITEMS; incomplete scope coverage returns FAIL.

This launch brief was frozen before product-code edits. Dispatch remains gated on an implementation-complete frozen diff manifest.
