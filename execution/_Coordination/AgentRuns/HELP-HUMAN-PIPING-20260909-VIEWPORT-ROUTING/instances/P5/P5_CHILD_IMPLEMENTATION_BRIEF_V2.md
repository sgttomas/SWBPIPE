# P5 bounded endpoint section-cut implementation child brief V2

RequestedBy: WORKING_ITEMS Agent 1 `/root/endpoint_repair_manager`
RunID: `HELP-HUMAN-PIPING-20260909-VIEWPORT-ROUTING`
ParentInstanceID: `P5`
ChildInstanceID: `P5-A2-ENDPOINT-IMPLEMENTATION`
Role: `TASK` Agent 2 with `software-bounded-implementation`
Model: `gpt-5.6-sol`
ReasoningEffort: `high`
Delegation: prohibited; role non-delegation is instruction-asserted under the native descendant harness

PackageID: `PKG-05`
DeliverableIDs: `DEL-05-03`
ScopePath: `{WORKING_ROOT}/execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-03_Fundamental stress recovery module`
WorkingRoot: `{REPO_ROOT}/projects/chirality-piping`
PROFILE_PATH: `{WORKING_ROOT}/software-workflow.json`
ApplyEdits: `true`

## Objective

Repair endpoint stress recovery so stress consumes common section-cut resultants rather than raw node-on-element endpoint actions. Preserve raw endpoint action rows and IDs. Use existing straight and curved station-resultant paths at fractions `0.0` and `1.0`, including the actual curved endpoint tangent/radial frames. Add focused colocated Rust tests for the required mechanical and preservation oracles. After focused Rust PASS and only through the existing deterministic generator, regenerate the one authorized preview fixture if it changes.

Also apply the root-authorized same-file metadata-only alignment for existing curved station rows: canonical `coordinate_system=element_local`, canonical `basis=recovered_from_local_element_stiffness`, and exact arc frame/equilibrium semantics in free-text `sign_convention`. Preserve every station numeric leaf, result kind, result ID, case key, and raw endpoint action row.

## Accepted basis

- Repository source: `533332349a4607eee561d4ef90fb05a62d86519e`.
- Root work graph: `WORK_GRAPH_ENDPOINT_REPAIR_AMENDMENT_V5.json`, SHA-256 `b033618adc64f690b52216c6fb9ccd12837447a3ddb7afced77e2bb40e653039`.
- Parent launch brief: `ENDPOINT_SECTION_CUT_IMPLEMENTATION_LAUNCH_BRIEF_V1.md`, SHA-256 `63de9814380cf2403097cf79a175652a09526db92f0c512f783bd739f64ceb58`.
- Selected algorithm: `instances/PREVIEW/design/ENDPOINT_SECTION_CUT_REPAIR_BRIEF_V1.md`, SHA-256 `c85c97a2946bbdf946da5cb26266d6ff42d2bf7f48fd9ad28a5daa6c7018dfe0`.
- Metadata refinement: `instances/PREVIEW/design/ENDPOINT_SECTION_CUT_REPAIR_METADATA_REFINEMENT_V1.md`, SHA-256 `96c222c21bda132bebce7aca315564349d846109222380fa84e68bd89518963c`.
- Validation refinement and root amendment: `instances/PREVIEW/design/ENDPOINT_SECTION_CUT_REPAIR_VALIDATION_REFINEMENT_V2.md`, SHA-256 `e72dc729f68658bfe517554cb33b5a18b3b3bf872a4f514e66389be8c725d3aa`.
- `projects/chirality-piping/AGENTS.md`, `agents/AGENT_TASK.md`, `skills/software-bounded-implementation/**`, `projects/chirality-piping/software-workflow.json`, `docs/SOFTWARE_WORKFLOW_PROFILE.md`, deliverable `_CONTEXT.md`, `ScopeOfWork.md`, and `projects/chirality-piping/docs/CONTRACT.md`.
- Intake source hashes: `core/product_physics/src/lib.rs` SHA-256 `f1ae3322a752d92e2125274e7c5ccc0f7ca2c26a00f19d8d5cd3d6c4234a3ed5`; fixture SHA-256 `e246338e5accd330c4fb7a7602510f15d8fac42cc4cee5c09fecbab227b0df13`.

## Declared reads

- Accepted basis above.
- `{WORKING_ROOT}/core/product_physics/src/lib.rs` and its crate manifest/examples.
- Existing straight/curved solver helpers required to understand contracts, read-only.
- `{WORKING_ROOT}/schemas/results.schema.yaml` and headless result-envelope binding/tests, read-only.
- `{WORKING_ROOT}/fixtures/product_preview/invented_mechanics_result.json` plus all consumers and the existing deterministic generator, read-only until the focused Rust gate passes.

## Allowed tools

Repository reads, `rg`, `git diff/status`, `apply_patch`, Rust formatting for the authorized Rust file, targeted Cargo tests only after the parent confirms the root-coordinated heavy-build slot, the registered generator only after focused Rust PASS, schema/headless validation explicitly within the same coordinated slot, and the software-workflow scope/check selectors. No network, install, release, Git mutation, destructive command, or delegated child.

## Allowed write targets

- `{WORKING_ROOT}/core/product_physics/src/lib.rs`, including colocated tests.
- `{WORKING_ROOT}/fixtures/product_preview/invented_mechanics_result.json`, only after focused Rust PASS and only via `npm run generate:product-preview-mechanics`.
- `{WORKING_ROOT}/execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-03_Fundamental stress recovery module/_run_records/VIEWPORT_ROUTING_20260909/**`.
- `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260909-VIEWPORT-ROUTING/instances/P5/**` for child return/evidence only; do not modify this sealed brief.

## Acceptance criteria

1. Endpoint stress recovery uses existing `straight_section_resultants` at fractions 0/1 and generalized/reused `curved_bend_station_resultants` at fractions 0/1, then the existing stress recovery seam.
2. Raw endpoint action output rows, metadata, stable IDs, kinds, and values remain unchanged.
3. Endpoint stress IDs remain stable. Endpoint metadata uses `element_local` and `recovered_from_local_element_stiffness`. Curved endpoint `sign_convention` states exactly: local x is endpoint arc tangent toward j; local z is bend-plane normal; local y is `z cross x` toward arc center; resultants come from section equilibrium over assembled end actions.
4. Existing curved station rows use the same canonical enum values and retain exact arc-frame/equilibrium detail in `sign_convention`, with no numeric change.
5. Focused oracles cover axial, torsion, bending about both local axes, element reversal, fixed/fixed and fixed/free pressure/thermal behavior, curved endpoints 0/1, all six resultants, end/station parity, dense/sparse parity, raw row preservation, and station numeric preservation. Repair the preexisting one-sided pressure expectation and keep honest failure evidence.
6. Complete affected straight and curved headless envelopes validate against `schemas/results.schema.yaml`; no subset is represented as a complete pass. Report any remaining actual schema failure rather than widening source/schema scope.
7. Fixture consumers are inventoried before broader checks. Fixture regeneration is conditional on focused Rust PASS and occurs solely through the registered generator.
8. A final write-scope check shows no changes outside the four declared evidence/source targets.

## Exclusions

No change to any other source file, schema field/enum, stress-recovery library math, pressure/Poisson/topology behavior, public contract, dependency/DAG/decomposition/lifecycle/decision/receipt state, fixture generator, UI app files, the F4 two solver files, Git state, or build integration. Do not fix or broaden the separately held pressure design. Do not edit `MEMORY.md`, `_STATUS.md`, or any deliverable production contract.

## Expected return

Write `P5-A2-ENDPOINT-IMPLEMENTATION-RETURN.md` and `P5-A2-ENDPOINT-IMPLEMENTATION-STATUS.json` under the P5 instance plus a TASK run record under the declared DEL-05-03 subtree. Return exact changed paths and before/after SHA-256 values, diff summary, test commands/results, fixture-consumer inventory, generator command/result if run, schema-envelope evidence, scope validation, remaining limitations, and whether the diff is ready for a fresh independent 100% review. Report the preexisting broader pressure semantics as explicitly unchanged.

## Escalation

Stop source expansion and report to the parent if a fix needs any undeclared product source or schema file; if raw endpoint rows or station numerics cannot be preserved; if the curved helper contract is inadequate; if focused Rust tests fail for an unrelated defect; or if full envelope validation reveals another schema failure. Do not self-authorize another path.
