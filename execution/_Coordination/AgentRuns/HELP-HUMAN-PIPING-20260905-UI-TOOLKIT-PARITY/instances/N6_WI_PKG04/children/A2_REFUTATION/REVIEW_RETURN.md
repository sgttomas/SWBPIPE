# Independent row 21 contract refutation

RUN_STATUS: SUCCESS
ReviewVerdict: FINDINGS — one blocking contract finding; not implementation or lifecycle acceptance.
ControlSurface: FILE
TaskProfile: NONE
TaskSkill: software-code-review (explicit brief adaptation: proposed contract review, no implementation diff)
ScopePath: {WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/instances/N6_WI_PKG04/children/A2_REFUTATION
Parent: N6_WI_PKG04 / WORKING_ITEMS
Model: unavailable in runtime metadata; inherited model, no override.
Delegation: none; TASK Agent 2 non-delegation is instruction-asserted.

## Accepted review basis and boundaries

Reviewed the entire PHASE_A_CANDIDATE_V1.md together with the overriding PHASE_A_AMENDMENT_V2.md and BASIS_HASHES.json. These are derivative coordination proposals against the parent-declared decomposition 0.12 / DAG-010 / SCA-009 basis, not decomposition truth. Read root/project AGENTS.md, AGENT_TASK.md, the review skill and all three companions, project software-workflow.json, DEL-04-03 ScopeOfWork.md and _CONTEXT.md, and the source/schema seams below. No tests or builds ran. No implementation diff is being accepted. Existing N2 rich-support and future atomic-batch work are declared prerequisites, not findings merely because absent today.

## Blocking finding F-01 — arbitrary rigid rotational subsets do not survive the current adapter

Priority: P1 for release of this contract to implementation.

The v1 interface promises independently selected Free/Rigid/Linear spring modes for all six DOFs and one ordinary record containing exactly the rigid subset. It then describes the existing product adapter as sufficient to map rigid records to Anchor/Guide without an explicit adapter implementation request. That fails for any subset containing a rigid rotation with fewer than six rigid DOFs. For example, Rx=Rigid and the other five DOFs Free creates a one-DOF record; Ux=Rigid, Rz=Rigid, Uy=Spring also creates a partial rotational rigid record. Both map to Guide, which rejects rotational DOFs.

Evidence (paths relative to WORKING_ROOT):

- `core/product_physics/src/lib.rs:3202` — `rigid_linear_support_from_preview` selects Anchor only when `restrained_dofs.len() == 6`; every ordinary partial subset becomes Guide at lines 3210–3212.
- `core/solver/linear_supports/src/lib.rs:514` — `prepare_rigid_support` checks each DOF against the selected family, reports InvalidSupportDof and skips rejected DOFs.
- `core/solver/linear_supports/src/lib.rs:634` — `dof_allowed_for_family` permits every DOF for Anchor but only translations for Guide. The mechanics representation already supports an explicit Anchor-family restrained subset; no new stiffness equation or solver family is required.

Remediation direction: amend the contract to explicitly route a product-physics adapter change to an authorized source owner before row 21 implementation. Select a family capable of preserving the exact ordinary rigid subset when it contains rotations; do not call a constructor that expands a subset into all six DOFs. Keep named guide/line-stop/vertical-support semantics intact. Add original adapter-level cases for rotation-only rigid, mixed translational/rotational rigid with a separate spring, and all-six rigid. Assert the exact prepared constrained indices, spring diagonals and untouched free DOFs. Existing acceptance checks 1–2 should expressly include partial rigid rotations, not only rotational springs. Then re-review the amended contract and eventually the frozen source diff.

This is an omitted concrete adapter work item, not an engineering-equation fork or a reason to redesign the bounded global interface.

## Other reviewed seams

- Unit semantics are coherent at design level: `product_physics/src/lib.rs:3962` normalizes each submitted support stiffness using the DOF-dependent LinearStiffness/RotationalStiffness dimension; lines 3164–3179 construct a positive matching SupportQuantity. `linear_supports/src/lib.rs:536` checks dimensional matching; lines 452–461 add springs only to their global stiffness diagonal. User quantities, no assumed stiffness, and global-only claims are appropriate. Operation parity remains the declared N2 prerequisite.
- v2 correctly makes durable association an explicit schema/resolver request. Current `operation_applier/src/lib.rs:4026` rebuilds a support from selected fields, so copying unknown JSON alone is insufficient. `schemas/model.schema.yaml:1872` defines a closed Support object, requiring a schema addition at the chosen canonical nesting. Both are already recognized by v2's schema/adapter-preservation requirement; these are implementation hold points rather than new refutations.
- `core/project_persistence/service.py:106` copies the model payload; `schemas/project_persistence.schema.yaml:335` delegates its validity to model.schema.yaml. `core/handoff/native_json/package.py:89` wraps native model payloads through `_model_payload`. Contractually requiring native roundtrip plus ordinary-edit preservation is warranted; those paths have not been execution-tested here.
- v2's association is descriptive metadata, with new boundary IDs, member inventory, and no promised aggregate reconstitution. Deleting/editing an individual member is therefore compatible with its stated reduced grouping semantics. Implementation must render actual member node/DOF state rather than imply that the original complete template remains intact.
- No local-coordinate transform, coupled stiffness, nozzle load/allowable, equipment geometry or PCF interpretation is silently introduced. Separately authored imposed movement remains explicitly outside this composite form.

## Fan-in and handoff

Closure: CONTRACT_REVIEW_FINDINGS; row 21 and PKG-04 remain open. Parent should amend F-01 and route adapter ownership alongside existing PKG-16/07 requests. N2 rich-support canonical shape, atomic batch, association storage and schema/persistence checks remain prerequisite source holds. No accepted upstream snapshot or pointer changed. This review is derivative evidence only. Rerun review against the amendment, and fresh source review is still required after implementation. No new human engineering ruling is identified by this finding.

## Tool and write containment

ToolsUsed: shell git rev-parse, cat, sed, rg for read-only inspection through functions.exec/exec_command; functions.exec/apply_patch for the sealed evidence-output exception only.
ToolPolicyCompliance: PASS for the explicitly adapted contract review. Direct code/test reading follows TOOL_POLICY.md; diff scope validation and affected-check execution are not applicable to a proposal with no source diff. No tests, installs, source edits, network, Git mutation, lifecycle edits or delegation occurred.
WriteAuthorization: ALLOWED_WRITE_TARGETS — only children/A2_REFUTATION evidence directory.
ResolvedSkillPath: {REPO_ROOT}/skills/software-code-review
ResolvedSkillVersion: 1
ResolvedTaskProfileRequirement: NONE
CompanionFiles: BRIEF_SCHEMA.md (found), TOOL_POLICY.md (found), QA_CHECKS.md (found)
AllowedTools: read-only file/search/shell inspection; explicit evidence-write exception in sealed brief.
RuntimeOverrides: contract-only adaptation; source unchanged; no tests required; this managed child return/status serve durable evidence.
Outputs: REVIEW_RETURN.md, STATUS.json
AppliedChanges: these evidence files only.
MISSING: implementation evidence intentionally absent for design phase.
NEEDS_HUMAN_RULING: none.
DEPENDENCY_NOTES: existing-capability predecessor; N2 rich-support; atomic batch; association schema/adapter acceptance; newly explicit partial-rigid-rotation adapter work.
