# PS1 seven-row factual source/interface backcheck

Run: `HELP-HUMAN-PIPING-20260908-PHYSICS-UI-EXECUTION`
Instance: `/root/dependency_currency/ps1_interface_backcheck`
Form: ephemeral bounded Agent 2; no delegation
Source: `779dedb8670625b36af07b89fc5557470e47c50e`
Verdict: `PASS`

## Result

The checkout HEAD exactly matches the sealed source binding. A scoped `git diff --quiet` over every source/interface file inspected returned `0`. The four UI source hashes frozen by U7 that are central to these rows (`types.ts`, `App.tsx`, `PipeViewport.tsx`, and the operation applier) match U7's recorded SHA-256 values exactly. No source drift or missing producer behavior was found that factually blocks the frozen F4 or U-A implementation.

| Dependency ID | Producer contract and selected consumer/import/call path | Focused current-head test evidence | Drift / missing behavior finding | Row result |
|---|---|---|---|---|
| `TP-DAG-004-DEL-04-04-E006` | **Verified.** `core/solver/linear_supports/src/lib.rs` re-exports frame DOF/unit types and supplies `LinearSupport` plus `prepare_boundary`; `core/solver/nonlinear_supports/src/lib.rs` consumes the shared DOF/unit types, while `core/product_physics/src/lib.rs` calls `prepare_boundary`, maps spring entries to global DOF/stiffness pairs, and passes them to `solve_active_set_frame_with_mode_and_springs`. | **Verified.** `FOCUSED_BOUNDARY_RUN_V1.md` records the exact current-head `frame_dof_reexport_matches_frame_kernel_boundary` invocation as `1 passed; 14 filtered out`. The retained-spring product test is present at current HEAD; no separate execution output for that additional test is claimed by the six-command record. | None. The frozen friction repair uses an invented `UserStiffnessElement` fixture and does not require a linear-support API change. | `PASS` |
| `TP-DAG-004-DEL-04-04-E007` | **Verified.** `core/solver/frame_kernel/src/lib.rs` provides `FrameDof`, frame/node/element and user-stiffness records, assembly, reduction, and `solve_dense`; `core/solver/nonlinear_integration/Cargo.toml` binds the crate and `nonlinear_integration/src/lib.rs` imports and calls those APIs in the selected solve path. | **Verified.** The exact `sliding_friction_support_applies_bounded_force_independent_of_seed` current-head invocation is recorded as `1 passed; 21 filtered out`; the test exercises the current explicit-normal friction consumer through the frame assembly/solve path. | None. The correction can remain within nonlinear integration; no frame-kernel producer change is needed. | `PASS` |
| `TP-DAG-004-DEL-04-04-E008` | **Verified.** `core/solver/diagnostics/src/lib.rs` provides the `SolverDiagnostic` envelope, `NonConvergence`, `convergence_diagnostic`, and `tolerance_policy_tbd_diagnostic`; nonlinear supports calls the convergence helper, nonlinear integration imports/uses the envelope and emits capped failure/TBD-policy diagnostics, and product physics maps solver diagnostics without changing their semantics. | **Verified.** Three exact current-head invocations in `FOCUSED_BOUNDARY_RUN_V1.md` passed: diagnostics `nonconvergence_after_iteration_limit_is_failure` (`1 passed; 23 filtered out`), nonlinear integration `iteration_cap_returns_nonconvergence_failure_diagnostic` (`1 passed; 21 filtered out`), and `tbd_policy_emits_visible_tolerance_policy_diagnostic_without_defaults` (`1 passed; 21 filtered out`). | None. Existing vocabulary represents capped current-normal consistency failure; no new diagnostic code or tolerance is needed. | `PASS` |
| `DAG-002-E0478` | **Verified.** `apps/desktop/src/types.ts` defines the current model records and structured `EditorOperationIntent`; `core/model_operations/operation_applier/src/lib.rs` validates and constructs canonical node and pipe records. `PipeViewport.tsx` builds `create_node`/`connect_pipe_run` intents, and `App.tsx` already owns single/batch apply and atomic session commit. | **Verified as current test source; execution log not supplied by U7.** `App.test.tsx` contains focused node creation, straight-pipe connectivity, endpoint-picking, stale completion, and atomic-batch rollback/one-checkpoint tests. `operationBatchService.test.ts` covers the exact native and WASM batch routes. New-route Add/Apply cases remain explicit post-implementation tests. | None. The selected UI work adds consumer orchestration inside its frozen source fence; it requires no canonical-model or operation-applier producer change. | `PASS` |
| `DAG-002-E0479` | **Verified.** `core/units/src/lib.rs` is bound through `core/model_operations/operation_applier/Cargo.toml`; the operation applier imports `unit_by_symbol`, `convert_for_dimension`, and `Dimension`, and its node/pipe resolvers require explicit accepted length units. `PipeViewport.tsx` loads the catalog through `unitCatalogService.ts` and emits unit/dimension metadata unchanged through App's service route. | **Verified as current test source; execution log not supplied by U7.** Existing node/pipe App tests assert explicit unit metadata; `operationBatchService.test.ts` verifies numeric conversion pass-through without local conversion. New route-specific validation tests remain required after implementation. | None. No conversion constant, catalog fallback, or units producer change is needed. | `PASS` |
| `DAG-002-E0480` | **Verified.** `projectService.ts` builds blank documents and exposes create/open/save; `apps/desktop/src-tauri/src/lib.rs` contains the corresponding native create/open/save and persist preparation path. `App.tsx` imports and calls those APIs with revision/epoch invalidation guards. | **Verified as current test source; execution log not supplied by U7.** `projectService.test.ts` contains create/save/open round trip and blank explicitly incomplete document coverage; `App.test.tsx` contains delayed-open/apply and open/save stale-completion cases. Node-only save/reopen remains an explicit post-implementation regression. | None. Existing persistence accepts incomplete model documents; no persistence producer change is needed. | `PASS` |
| `DAG-002-E0481` | **Verified.** `types.ts` carries named section records and inline pipe section quantities; `section_bindings.rs` validates/materializes named-section assignment. `PipeViewport.tsx` emits inline OD/wall plus an existing material reference, while `features/toolkit/SectionAssignment.tsx` remains the separate `assign_section` consumer. | **Verified as current test source; execution log not supplied by U7.** `ExistingToolkitEngine.test.tsx` exercises named section assignment, materialization, propagation, and detach; current App straight-pipe tests exercise inline section dimensions/material selection. New route builder cases remain post-implementation evidence. | None. A straight pipe may use inline dimensions before the separate named-section assignment; no section-schema producer change is needed. | `PASS` |

The UI rows have current focused test code supporting the reused seams, but U7 did not cite a frozen current-head execution transcript comparable to F4's six-command record. This is reported as a run-evidence gap, not as missing producer behavior. U7's brief already requires targeted and registered post-change execution plus native walkthrough evidence before integration acceptance.

## Evidence bindings

All paths are relative to `projects/chirality-piping/` unless the path begins at repository root.

| Evidence path | SHA-256 |
|---|---|
| `execution/_DAG/DAG-010/DependencyEdges.csv` | `1892d2e7295d936b1dbf57104ab00df149fe17f8afbab4dc8b7e1c8b95082d28` |
| `execution/_Evaluation/PHYSICS_UI_PREPARATION_20260907/Q1/READINESS_MATRIX.csv` | `83c3ce67c8a59da06e55f035a9840f662c9c25ae1d34b574c22c9188ef39ec5f` |
| `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-EXECUTION/instances/F4/PREREQUISITE_INTERFACE_EVIDENCE_V1.md` | `a42912f4acb005ad5b6d7731271454b0bd1fd47e55ae3ce2cda3b1679791d9ee` |
| `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-EXECUTION/instances/F4/CURRENT_NORMAL_EXPECTATIONS_V1.md` | `4de46610000a90b9be3166dace956535613f519416a59f394a6095c6324225ae` |
| `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-EXECUTION/instances/F4/IMPLEMENTATION_BRIEF_V1.md` | `a6fb3b29a39188fd13e83d9a199ae23f9a876f922dd843551060ff1bd51a103b` |
| `execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-04_Nonlinear support active-set solver/_run_records/PHYSICS_UI_EXECUTION_20260908/FOCUSED_BOUNDARY_RUN_V1.md` | `b3cfe3c470231024614b79202d20fdc27970642cffcd96ab9ccf07d0e8c53801` |
| `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/PHYSICS_UI_EXECUTION_20260908/CONSUMED_INTERFACE_EVIDENCE_V1.md` | `27161c9090b34dadb8004a05fcd8ca5857cd120350d846f63b84670366e10419` |
| `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/PHYSICS_UI_EXECUTION_20260908/IMPLEMENTATION_BRIEF_V1.md` | `013aec454e7179d96194827b52f8fa0aa49c07e29365f018212aaa1512809461` |

### Live friction source/test bindings

| Path | SHA-256 |
|---|---|
| `core/solver/linear_supports/src/lib.rs` | `01ac02c48411813d57adf39b4dd97659f19941628002379890e792b2600e5934` |
| `core/solver/nonlinear_supports/src/lib.rs` | `f6c6e62994564dddfc84e60df63f2e2f3a293b1d48a5950469c4368509733062` |
| `core/solver/frame_kernel/src/lib.rs` | `d657e6507aa5d4ef80124e3f6d10bd5a8af8eae54fe21af787c92814c8b73fde` |
| `core/solver/nonlinear_integration/Cargo.toml` | `6b6760eafdb97092345cb28aebac456cfccc9381542dcf0ec28bdcd3771587ea` |
| `core/solver/nonlinear_integration/src/lib.rs` | `fb02a52273637f844c63ac2e1545bc7fbe8ab79b58d7f5e259829e46be4b1439` |
| `core/solver/diagnostics/src/lib.rs` | `b9092ce5a8b3f38b9124c14f8dbd5853ca250b41c1b7571cc02b913bb74fdfa1` |
| `core/product_physics/src/lib.rs` | `e757b8a51e2c4ae68ac4d6c37620663bf4d698ff03b8d6349b40b484bb591903` |

### Live UI source/test bindings

| Path | SHA-256 |
|---|---|
| `apps/desktop/src/types.ts` | `b23991700988d1fc80811089dd9daf54ddddc9690610f21493c71cd7b7a71caf` |
| `apps/desktop/src/App.tsx` | `72f1d67875de11108c696a0923463a4d4c6f8216d391c84b3da1c94ab4ca9a66` |
| `apps/desktop/src/App.test.tsx` | `679981e56cea23cb84924a243a0757680e79e5efb81c5711fd66caa8aae6f942` |
| `apps/desktop/src/features/viewport/PipeViewport.tsx` | `286823e90c3594d8fa893261698036a545674975ed12cb3b19c57dd6817d1c9b` |
| `apps/desktop/src/features/model-tree/PropertyInspector.tsx` | `697df27e88d87e7004c0a331c4905857573b90c428f62310aaf15f3a1fb42022` |
| `apps/desktop/src/services/operationBatchService.ts` | `0cf84f7b8ced2786fa03729087fd1197bf8773afb6d15a9653cb86e8a0ecc52f` |
| `apps/desktop/src/services/projectService.ts` | `da38305ab02c69be95cdae4e79c9546568038716d597cc72385883938cc1363d` |
| `apps/desktop/src/services/unitCatalogService.ts` | `0ef7dfe55e078d0cca10a954fd2cef67feb00d16b34e4942c2da07b8a78382b8` |
| `apps/desktop/src-tauri/src/lib.rs` | `d9f8cfc11a5763a22da42eb2c2fbb43cb299f6205d1d481c6b3a8cb55cc9de81` |
| `core/model_operations/operation_applier/Cargo.toml` | `0a60331dac87de644cdc3ff4c4b2a0a125a313b73fb74efcf3e6e29c55deeff6` |
| `core/model_operations/operation_applier/src/lib.rs` | `83f7e3063fc483a15aecd4fc2a513abba1ee001c2cc8f568037281194640cbc7` |
| `core/model_operations/operation_applier/src/atomic_batch.rs` | `f73a763cd9d273e8f802304f1150c316ff5470739692fb41c95dc19ff2450aa6` |
| `core/model_operations/operation_applier/src/section_bindings.rs` | `424cba3f1e6292d75233bd403b60c922e2e56365c7c7b15b463c6efe58d9fd72` |
| `core/units/src/lib.rs` | `4bd102e98cc6be95562002637b48af4517dc88c9d82c23f8d1aede7c5fcf8592` |
| `apps/desktop/src/services/projectService.test.ts` | `a7e28cff52c2d4f02dc54d3ade763ee5d70eb6c759ca2ced15418c9eee395bed` |
| `apps/desktop/src/services/operationBatchService.test.ts` | `2c93b34ddbbaf6f985ac797cfb565d6e303cacc8b56215868b42b94057b012f1` |
| `apps/desktop/src/features/toolkit/ExistingToolkitEngine.test.tsx` | `960b157bad9b78bf50ec0b8161aa653634b73689648c5e344270d0de29034c78` |
| `apps/desktop/src/features/toolkit/SectionAssignment.tsx` | `d15ad7fc5978c907ee4b3198aa08094010405c5dcde98b38b8661fd90390983a` |

## Claim fence

This `PASS` is a factual source/interface gate only. It does not satisfy any dependency row, alter DAG-010 or a pointer, establish lifecycle closure, replace independent product-diff review, or replace human engineering review, acceptance, or approval. The existing formal and Owner gates remain unchanged.
