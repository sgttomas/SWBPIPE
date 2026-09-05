# SECTION_CODE_REVIEW_V1 — fresh independent N3_R2 return

RUN_STATUS: SUCCESS
ReviewVerdict: REVISE
ControlSurface: MERGED
TaskProfile: NONE
TaskSkill: software-code-review
ScopePath: /Users/ryan/.codex/worktrees/8728/chirality/projects/chirality-piping/core/product_physics
ResolvedSkillPath: /Users/ryan/.codex/worktrees/8728/chirality/skills/software-code-review
ResolvedSkillVersion: 1
ResolvedTaskProfileRequirement: NONE
CompanionFiles: BRIEF_SCHEMA.md (found), TOOL_POLICY.md (found), QA_CHECKS.md (found)
WriteAuthorization: ALLOWED_WRITE_TARGETS, evidence-only explicit parent instruction; ApplyEdits=false for source.
RuntimeOverrides: sealed brief explicitly permits read-only shell inspection; evidence return authorized separately. No tests, installs, network, Git mutation or delegation.
AllowedTools: read-only inspection plus declared deterministic scope/check selectors; evidence return write only.
ToolPolicyCompliance: PASS.

## Review basis and coverage
Reviewed 100% of all 319 lines of SECTION_FROZEN_DIFF_V1.patch against 740569598f9d00440636b8ea25264127f418e4ec. Sole changed source path is core/product_physics/src/lib.rs. SHA256 verified before and after review: d48dc7337be34eda5303fa7f191d222fc1d3ba0b0faabaedd1abc7f810ad92e7.
Read SECTION_IMPLEMENTATION_BRIEF_V1 and V2 amendment, SECTION_IMPLEMENTATION_RETURN_V1, ROW17_CANDIDATE_V2, PHASE_A_SNAPSHOT_V2 and refutation R3, TASK/skill companions, project instructions and relevant scope/contract material. Traced common solve entry, existing ID validation, quantity normalization, geometry derivation, desktop transport and operation section creation. Scope validation PASS before semantic review.

## Actionable finding

### F1 [P2] Reject blank section identity before resolving a bound pipe
Location: core/product_physics/src/lib.rs:3729–3742 (resolve_shared_sections).

A pipe with section_ref="" and one shared Section with id="" (likewise matching whitespace-only strings), valid matching OD/wall and otherwise valid model passes the resolver. The prior validate_model_inputs call does not catch it: validation.rs:65–141 enumerates other entity collections but omits sections, while its existing detect_empty_ids at :1282 requires explicit nonblank IDs. Therefore the newly bound authoritative section can have no usable identity and still produce mechanics results through the direct common solve API, unlike the existing model-entity identity contract. This is established by source tracing, not an executed reproduction (review brief forbids tests).

Reject reference.trim().is_empty() with a blocking section-reference diagnostic before matching; ensure any referenced section ID satisfies the same nonblank invariant. Add focused common-entry tests for empty and whitespace-only matching IDs/references. A narrow resolver guard can fix this within the existing source fence. Re-review the frozen amended diff and rerun focused/full crate tests.

## Reviewed behavior without additional actionable findings
Exact source/cache value+unit comparison matches V2, including rejection of physically equivalent representations. Missing/duplicate referenced sections, unsupported type/property, absent/invalid OD/wall and incompatible local mill tolerance fail before solving. Common entry resolves before normalization; unbound pipes and supplemental quantities are retained. New transport fields are optional/default-empty and match current desktop Section shape. Canonical schema is a separate representation and no migration is asserted. Previously ignored malformed unbound Section JSON now fails typed deserialization, but no valid supported preview transport regression was established; this is not an additional finding. No new dependency, generated source, concurrency or persistence mutation is introduced in this diff.

## Verification evidence and limits
Implementer reports six focused shared-section tests and all 107 crate tests passing, plus compile-only validation. This review did not execute tests. Independently selected registered affected checks: evidence-sweep, harness-self-check, piping-pytest; these remain manager-owned integration checks. Native/Wasm/headless consumer parity, operation materialization, load/display/export stale-cache guards and solver-result-change integration fixtures are outside this narrow source release and remain required in parent integration. This review is not lifecycle acceptance and does not close row17.

## ToolsUsed
- python3 tools/software_workflow/validate_change_scope.py: PASS for sole source path.
- python3 tools/software_workflow/select_affected_checks.py: selected evidence-sweep, harness-self-check, piping-pytest.
- Shell git rev-parse, cat, rg, sed, head, wc, shasum: read-only context and source inspection.
- Shell cat redirected only to this explicitly authorized evidence return.

Outputs: this file only, serving as durable managed-review return.
AppliedChanges: evidence return only; no source writes.
ProposedChanges: F1 narrow blank-reference guard and regression tests.
MISSING: registered broad/integration evidence remains parent-owned, not claimed here.
NEEDS_HUMAN_RULING: none.
DEPENDENCY_NOTES: derivative review consumes manager-declared accepted upstream SCA009/ROW17 candidate and frozen source; parent resolves findings, records next immutable snapshot and decides source fan-in. No pointers or authoritative lifecycle files changed. Rerun review on source amendment; no delegation performed.
