# V3 independent contract re-review

RUN_STATUS: SUCCESS
ReviewVerdict: PASS — design only; F-01 resolved in the proposed contract.
Parent: N6_WI_PKG04 / WORKING_ITEMS
Child: A2_REFUTATION / TASK Agent 2
Model: unavailable; inherited runtime, no override.

## Reviewed basis

Read PHASE_A_AMENDMENT_V3.md and rechecked its exact mechanics claims against current `core/solver/linear_supports/src/lib.rs`. Unchanged v1/v2 remain within the initial review scope and conclusions recorded in REVIEW_RETURN.md. This return supplements and preserves that initial finding record; it does not rewrite it or claim the initial finding was absent.

## F-01 disposition

V3 explicitly requires composed rigid members to emit family `anchor`, requests the corresponding `product_physics` branch, and preserves the supplied restrained DOF vector verbatim. It explicitly forbids the all-six-DOF convenience constructor and labels a partial subset truthfully in the UI.

Source evidence, relative to WORKING_ROOT:

- `core/solver/linear_supports/src/lib.rs:118` defines `LinearSupport` with independently supplied family and restrained_dofs.
- Lines 129–145 show why `LinearSupport::anchor` must not be used for a partial subset: that convenience constructor populates all six DOFs.
- Lines 402–408 route the Anchor family through the ordinary rigid-support preparation path.
- Lines 514–532 check and apply only the supplied restrained_dofs, without expanding the set.
- Lines 634–640 accept every DOF type for Anchor while retaining the narrower rules for Guide, LineStop and VerticalSupport.

Therefore the specified adapter change exposes existing explicit-subset mechanics without new equations or solver changes. Its preservation of existing family fallback is appropriate. V3 adds the missing downstream tests: Rx-only rigid, mixed Ux/Rz rigid with Uy spring, all-six rigid, exact constrained/free DOF assertions and named-family regression coverage. These are required implementation gates; they were not executed in this contract review.

No actionable residual contract finding identified. The durable association, units, global coordinate restriction and explicit implementation prerequisites remain as assessed in the initial review.

## Handoff and limitations

Contract review closure: PASS_DESIGN_ONLY. Row 21, source implementation and lifecycle closure remain open. Product-physics source ownership remains held until the parent releases it after N3; N2 rich-support and atomic-batch prerequisites remain in force. PKG-16 must accept exact association/payload/schema shape before PKG-07 authoring. Operation-to-preview tests, schema/native persistence checks, human/agent parity, frozen-source independent review and applicable registered checks remain required downstream.

No implementation or test execution is claimed. No human engineering ruling is newly required. This evidence is derivative coordination material against the parent-declared decomposition 0.12 / DAG-010 / SCA-009 basis; no accepted snapshot or pointer changed. Re-review if the proposed subset mapping, source ownership or accepted contract changes.

## Containment and method

ControlSurface: MERGED — original sealed brief plus parent bounded V3 follow-up.
TaskProfile: NONE
TaskSkill: software-code-review, explicit contract-only adaptation.
ResolvedSkillVersion: 1
CompanionFiles: BRIEF_SCHEMA.md, TOOL_POLICY.md and QA_CHECKS.md previously read.
ToolsUsed: read-only shell cat/sed via functions.exec/exec_command; functions.exec/apply_patch for evidence only.
ToolPolicyCompliance: PASS for the bounded adapted review; no tests/builds, network, source/Git/lifecycle mutation or delegation.
WriteAuthorization: ALLOWED_WRITE_TARGETS — own evidence directory, only RE_REVIEW_V3.md and STATUS_V3.json this follow-up.
Outputs: RE_REVIEW_V3.md; STATUS_V3.json.
MISSING: downstream implementation and executed test evidence intentionally pending.
NEEDS_HUMAN_RULING: none.
DEPENDENCY_NOTES: N3 product-physics ownership release; N2 support/atomic-batch prerequisites; exact PKG-16 association contract; PKG-07 authoring; subsequent independent source review and checks.
