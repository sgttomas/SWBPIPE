# R2 independent frozen integration review

RUN_STATUS: SUCCESS
ReviewVerdict: FAIL — two actionable findings; remediation and re-review required.
ControlSurface: MERGED
TaskProfile: NONE
TaskSkill: software-code-review
ScopePath: /Users/ryan/.codex/worktrees/8728/chirality/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/N2_WI_PKG16
ResolvedSkillPath: /Users/ryan/.codex/worktrees/8728/chirality/skills/software-code-review
ResolvedSkillVersion: 1
ResolvedTaskProfileRequirement: NONE
CompanionFiles: BRIEF_SCHEMA.md (found), TOOL_POLICY.md (found), QA_CHECKS.md (found)
AllowedTools: read/bash per sealed brief and parent clarification; evidence-only write R2_RETURN.md.
RuntimeOverrides: Parent confirms instruction root equals resolved git root despite absent CHIRALITY_INSTRUCTION_ROOT environment. Native execution instruction+config asserted. Runtime return is durable managed-read-only record; no separate source/run-record writes.
ToolsUsed: shell read operations; python3 tools/software_workflow/validate_change_scope.py; python3 tools/software_workflow/select_affected_checks.py; Python standard-library hash/read/report operations.
ToolPolicyCompliance: PASS within explicit parent clarification. Scope validator used before final judging; first discovery reads preceded validator (method-order deviation recorded).
WriteAuthorization: EXPLICIT_BRIEF_TEXT — R2_RETURN.md only.

## Reviewed basis and coverage

Reviewed 100% B2_FROZEN.diff, entire rich_authoring.rs and display_units.rs. All six hashes in B2_FROZEN_HASHES.json match current sources at review start and conclusion. Read root/project AGENTS, TASK, skill plus companions, B1/B2 briefs/amendments, B2 UI interface, A2 consumer design, B2 return/checks. Read existing create resolver, operation dispatch/preview/apply changes, product_physics nonlinear parsing/dependency and modulus-basis consumers. Frozen paths comply with declared integration fence. B1 Section/deletion implementation remains included in this whole review.

## Actionable findings

### R2-1 [P2] No-op wind exposure changes stored model shape

Location: core/model_operations/operation_applier/src/rich_authoring.rs:209–228 (exposure replacement writes), with exposure projection at :117.

An existing wind object may omit either exposure array. Its current display projection inserts empty arrays, so an after payload equal to that displayed projection is a no-op. The resolver nevertheless returns two writes, inserting arrays absent in the source model. Example: wind={pressure:{value:100,unit:Pa},direction:global_x}; before and after both canonical {exposed_pipe_refs:[],exposed_spans:[]}. Applying adds both keys and changes the model hash. This contradicts B2 source/no-op preservation and B2_RETURN's exact-original representation claim. The current integration test exercises this shape but only asserts the input is unchanged, not that applied model equals input.

Remediation: after validation, preserve the original optional-key presence for projected no-ops (e.g. return zero writes); retain readiness diagnostics and strict validation. Add apply/hash equality regression for each absent-array combination, including existing integer quantity representations.

### R2-2 [P2] Rich support creation validates a lossy projection of its payload

Location: core/model_operations/operation_applier/src/lib.rs:4097–4105, interacting with raw restraint filtering at :4045 and properties reconstruction at :4092.

The newly integrated rich creation validator receives reconstructed support, after the legacy path has filtered non-string restraints and discarded properties members other than linear_stiffness. The new allowlist expressly accepts properties but never checks its nested keys. Thus a rich create payload with restraints=["UY",7] or properties={unexpected:1} succeeds while silently losing authored content. An extra property alongside valid linear_stiffness is likewise dropped; quantity metadata is reconstructed away. The underlying legacy normalization predates B2, but extending it as the accepted strict rich-create route leaves the new contract (unknown nested fields must reject, no silent source drops) unmet. Replacement configuration already validates raw nested content strictly, so the two advertised equivalent entry points differ.

Remediation: validate raw create payload shape before normalization: all restraints must be strings, properties may contain only explicitly supported legacy members, and its quantity must reject unknown members. Preserve the supported legacy value/unit behavior. Add integrated validate/apply tests asserting blocking/no model/no diff for unsupported raw fields and a successful legacy stiffness case.

## Verification and residual risk

B2_CHECKS records 101 unit + 1 native contract + 2 canonical hash tests (104 total), including 10 rich module tests; parent reports Python schema pass. No tests executed by this read-only reviewer. Display facade conversion is pure; failures remain explicit. Rich dependent reaction checks and used-material exact/interpolated bases agree with inspected consumers. No further actionable findings in frozen Section/deletion, schema, native/Wasm wrappers or display module.

Affected-check selector with project-relative source paths selects desktop-build, desktop-test, evidence-sweep, harness-self-check and piping-pytest. These wider checks remain parent-owned and not evidenced complete by B2_CHECKS. Initial selector invocation used repo-relative paths and matched only harness-self-check; corrected invocation above is authoritative. Native/Wasm/browser verification and subsequent integrated-source changes remain outside this frozen verdict.

Outputs: R2_RETURN.md
AppliedChanges: evidence only; no product writes.
ProposedChanges: R2-1 and R2-2 above.
MISSING: wider registered checks pending parent; no claimed waiver.
NEEDS_HUMAN_RULING: none.
DEPENDENCY_NOTES: repair both findings, freeze changed source hashes, and re-review before manager acceptance. No lifecycle acceptance granted.
