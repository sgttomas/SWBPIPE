# Sealed implementation brief READY / NOT RELEASED

RequestedBy: HELP_HUMAN; ParentInstanceID N6_WI_PKG04; future ChildInstanceID B1_ANCHOR_ADAPTER; RunID HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY.
Role TASK Agent2 nondelegating; TaskSkill software-bounded-implementation. Scope PKG04 / DEL0403 / SOW011 / OBJ003 / OUT001 / AC001 / VER001. Resolve REPO_ROOT with git rev-parse --show-toplevel and WORKING_ROOT={REPO_ROOT}/projects/chirality-piping.

## Hard start gates

This brief alone does not release source or authorize dispatch. Require parent serial source release after existing-capability predecessor accepted, plus N3 shared source released. Immediately before writes recompute SHA256 of core/product_physics/src/lib.rs and compare to SECTION_ACCEPTED_SNAPSHOT_V1.json file hash:
`89607dac35257329bcc58041cfb62407b537b3cdbb28a61efe82f02544ddb9af`.
If any mismatch, hold and return actual hash for parent predecessor amendment; never overwrite or integrate unaccepted intervening changes. The N3 snapshot records 108 passing product tests in its associated return; verify evidence, do not present it as a new execution.

Read root/project AGENTS, AGENT_TASK, software-bounded-implementation skill and companions, software-workflow profile, DEL0403 SOW/context, N6 v1/v2/v3 contract + independent V3 review, N3 SECTION_ACCEPTED_SNAPSHOT_V1.json, current product_physics function/test region and linear_supports prepare_rigid_support/dof_allowed_for_family.

## Objective and exact code boundary

Expose the existing mechanics capability to honor explicit family=anchor with an exact user-selected rigid DOF subset. In rigid_linear_support_from_preview add `Some("anchor") => SupportFamily::Anchor` to family selection. Keep current struct construction with incoming restrained_dofs verbatim. Do NOT use LinearSupport::anchor constructor, which populates all six DOFs; do not change defaults, implicit family fallback, named LineStop/VerticalSupport logic, global coordinates or equations.

AllowedWriteTargets: only core/product_physics/src/lib.rs and this instance children/B1_ANCHOR_ADAPTER evidence. No solver source, validation.rs, model operations, schema, desktop, Cargo files, dependency/build artifact tracked files, registers/status/pointers, receipts, commits/push. Shared read access project sources/specs and declared prior evidence; tools read/write/bash with no network. Parent exclusively assigns this file for this stage; overlapping source writes prohibited.

## Focused original tests in the same lib.rs only

Use existing rigid_preview_support helper and direct mapped LinearSupport results; pass through linear_supports::prepare_boundary using fully qualified existing dependency if needed. Cases: explicit anchor with RX only; explicit anchor with UX+RZ and a separate LinearSupport::spring on UY; explicit anchor all six; existing guide/line_stop/vertical_support fallback unchanged. Assert family, node, exact restrained_dofs, prepared global indices, no findings, and spring entry/diagonal only on UY. Use a synthetic diagonal dense matrix and existing apply_linear_supports to establish constrained/free indices plus spring diagonal if it fits existing public API. Positive invented stiffness with explicit quantity dimension; no new formula/default engineering value. Original focused tests are mechanics witnesses, not product authoring/atomic/persistence closure.

Run focused test filter then full product_physics crate tests offline/locked with parent provisioned Cargo target environment and existing dependencies; record exact command/results using portable paths. Do not guess full validation pass from focused tests. If dependencies/toolchain prevent tests, report exact error. No package-wide/global sweep from child; parent owns registered check fan-in.

Freeze original source hash, terminal source hash, full source delta from the accepted pre-write bytes (not a diff against initial Git HEAD that includes N3 work), test counts/commands and source-scope containment. Return B1_RETURN.md and checks JSON. Parent then dispatches a fresh read-only TASK+software-code-review over 100% this frozen adapter/test delta; no source acceptance until PASS without actionable findings. Findings repair is serialized and re-reviewed. Snapshot pins accepted output hash and explicit derivative/handoff status. No publish/lifecycle claim.
