# F4 consumed-interface evidence for pending DEL-04-04 rows E006–E009

Status: `CANDIDATE_EVIDENCE_FOR_PROJECT_SETUP`; no dependency row or accepted DAG mutation
Source binding: `779dedb8670625b36af07b89fc5557470e47c50e`
Proposed F4 change boundary: `core/solver/nonlinear_integration/src/lib.rs` only for solver mechanics/tests; `core/product_physics/src/lib.rs` only for product-boundary regression if released by root. All producer interfaces below remain unchanged. `core/solver/nonlinear_supports/src/lib.rs` is not presently justified as a write.

## Actual focused current-head boundary run

Portability correction: the first working copy of this unfrozen candidate named its host `/tmp` target here. The control-facing record now uses the portable task-temp form; exact executed command/path provenance moved to the deliverable structural record `.../_run_records/PHYSICS_UI_EXECUTION_20260908/FOCUSED_BOUNDARY_RUN_V1.md`.

Run after the unchanged witness replay, in manager-unique `CARGO_TARGET_DIR={TASK_TEMP}/f4-boundary-manager-8728`; source remained unchanged. Each command exited `0`:

| Producer/consumer boundary | Exact filtered test | Result |
|---|---|---|
| E006 linear-support/frame DOF identity | `linear_supports`: `frame_dof_reexport_matches_frame_kernel_boundary` | `1 passed; 14 filtered out` |
| E008 diagnostic envelope | `diagnostics`: `nonconvergence_after_iteration_limit_is_failure` | `1 passed; 23 filtered out` |
| E009 semantic unit conversion | `units`: `conversion_can_be_bound_to_semantic_dimensions_with_shared_units` | `1 passed; 12 filtered out` |
| E007 frame solve plus current explicit-normal friction consumer | `nonlinear_integration`: `sliding_friction_support_applies_bounded_force_independent_of_seed` | `1 passed; 21 filtered out` |
| E008 capped failure in integration | `nonlinear_integration`: `iteration_cap_returns_nonconvergence_failure_diagnostic` | `1 passed; 21 filtered out` |
| E008 accepted/TBD policy visibility | `nonlinear_integration`: `tbd_policy_emits_visible_tolerance_policy_diagnostic_without_defaults` | `1 passed; 21 filtered out` |

These are boundary-current checks, not target-deliverable closure or formal review. The original M1-N-008 replay separately proves the missing derived-current-normal behavior.

## E006 — DEL-04-03 linear support and restraint models

- Accepted contract: DEL-04-03 `ScopeOfWork.md` `AC-001` / `VER-001` preserves implemented linear-support boundaries, `FrameDof`, dimensional metadata, prescribed-displacement integration, and linear/nonlinear separation. Live `_STATUS.md` is `IN_PROGRESS` with an empty `Remaining` section; this is technical currency evidence, not lifecycle/dependency acceptance.
- Exact producer/interface: `core/solver/linear_supports/src/lib.rs` re-exports `FrameDof`, `CanonicalDimension`, `QuantityUnitMetadata`, and `UnitSystemRef` at lines 11–13; `SupportQuantity`, `LinearSupport`, `prepare_boundary`, and spring entries provide the validated linear-support boundary. `core/solver/nonlinear_supports/src/lib.rs` consumes the re-exported DOF/unit types at lines 7–8. `core/product_physics/src/lib.rs` consumes `prepare_boundary` and turns validated `SpringEntry` values into `(global_dof, stiffness)` for `solve_active_set_frame_with_mode_and_springs` at lines 1913–1920.
- Focused existing boundary tests at this source: `frame_dof_reexport_matches_frame_kernel_boundary`, `support_quantity_carries_explicit_canonical_unit_metadata`, and product `audit_retained_spring_fixture_uses_selected_node_state` (both modes). F4 will bind an actual run after the isolated witness frees the reserved compile slot.
- Repair relationship: M1-N-008 itself uses a `frame_kernel::UserStiffnessElement`, not a DEL-04-03 `LinearSupport`; the current-normal solver correction neither changes nor bypasses the linear-support API. The retained-spring product test remains the relevant unchanged consumer boundary.
- Missing/surviving hold: no missing E006 interface behavior was found for this repair. DEL-04-03 lifecycle remains `IN_PROGRESS`; selected-state identity is not independent mixed-fixture adequacy. PROJECT_SETUP must decide whether this is enough to disposition E006 without treating empty `Remaining` as closure.

## E007 — DEL-04-01 3D frame stiffness kernel

- Accepted contract: DEL-04-01 `ScopeOfWork.md` requirements `REQ-001`–`REQ-005` and `REQ-010` provide the 3D six-DOF frame model, deterministic global stiffness assembly, coordinate/boundary interface, and verification; `AC-001` / `VER-001` preserve the implemented kernel and open mechanics-review residuals. Live `_STATUS.md` remains `IN_PROGRESS` with the mechanics-program G1/G2/G4 and M2/M3 owner re-disposition row.
- Exact producer/interface: `core/solver/frame_kernel/src/lib.rs` exports `FrameDof`, `FrameNode`, `FrameElement`, `UserStiffnessElement`, `DenseMatrix`, `DenseVector`, `assemble_global_stiffness_with_user_elements`, `reduce_system_with_prescribed_displacements`, and `solve_dense`. `core/solver/nonlinear_integration/Cargo.toml:13` binds the crate. The consumer imports these at nonlinear integration lines 10–14, assembles at lines 388–393, reduces in `solve_linearized_system`, and uses `solve_dense` for dense scrutiny/fallback. M1-N-008 directly constructs the rotated `UserStiffnessElement` from this producer.
- Focused existing boundary tests: nonlinear-integration `dense_scrutiny_mode_keeps_sparse_parity_evidence`, `sliding_friction_support_applies_bounded_force_independent_of_seed`, and the unchanged M1-N-008 harness cover assembly/solve consumption in both modes. Actual current-head results will be attached after replay.
- Repair relationship: the correction can stay entirely in the nonlinear integration iteration controller; no frame-kernel type/function signature or solver behavior needs to change.
- Missing/surviving hold: the live frame solve provides all behavior consumed by this repair. DEL-04-01's broader mechanics-program re-disposition remains open and must not be inferred closed by this narrow evidence.

## E008 — DEL-04-06 solver diagnostics and singularity detection

- Accepted contract: DEL-04-06 `ScopeOfWork.md` `AC-001` / `VER-001` preserves diagnostic codes/mappings, caller-supplied tolerance behavior, visible `TolerancePolicyTbd`, nonconvergence, provenance, and mechanics-only authority. Live `_STATUS.md` is `IN_PROGRESS` with an empty `Remaining` section; this does not create lifecycle closure.
- Exact producer/interface: `core/solver/diagnostics/src/lib.rs` exports `SolverDiagnostic{Code,Class}`, `DiagnosticSeverity`, `DiagnosticSource`, `convergence_diagnostic`, and `tolerance_policy_tbd_diagnostic`. Nonlinear supports invokes `convergence_diagnostic` at lines 481–506 and annotates `NonConvergence`; nonlinear integration imports the envelope at lines 20–22, collects classifier diagnostics at lines 490–514, emits an explicit failure on every otherwise silent nonconverged capped exit, and emits `TolerancePolicyTbd` only for `ConvergencePolicyStatus::Tbd`.
- Focused existing boundary tests: nonlinear integration `iteration_cap_returns_nonconvergence_failure_diagnostic`, `sliding_seed_at_single_iteration_cap_emits_nonconvergence_diagnostic`, `sticking_friction_converged_at_single_iteration_cap_has_no_false_positive`, and `tbd_policy_emits_visible_tolerance_policy_diagnostic_without_defaults`; diagnostics `nonconvergence_after_iteration_limit_is_failure`.
- Repair relationship: preserve existing public result semantics (`converged`, final state/vectors, iteration records, diagnostics) and existing diagnostic code/class/source. If current-normal consistency requires another iteration, a cap exit must reuse the existing `NonConvergence` failure envelope; no new DEL-04-06 code/category is required. Product continues mapping solver diagnostics through `product_diag_from_solver_diag` at product lines 1922–1928.
- Missing/surviving hold: the current diagnostics envelope can represent the repair honestly. The exact causal message for a current-normal consistency deferral belongs in nonlinear integration. New diagnostic vocabulary or a new convergence threshold would be a separate held choice.

## E009 — DEL-02-02 unit system and dimensional-analysis core contract

- Accepted contract: DEL-02-02 `ScopeOfWork.md` `U-001`, `U-003`, `U-006`, `U-012`, `U-013`, and `U-015`, plus `AC-001` / `VER-001`, require explicit dimensions, deterministic compatible conversion, explicit dimensionless classification, and fail-closed invalid/missing units. Live `_STATUS.md` retains PDU-037, wider B2/B3 wiring, alias/parser and diagnostic namespace choices, and independent numeric normalization/validation holds.
- Exact producer/interface: `core/units/src/lib.rs` exports `Dimension`, `UnitId`, `canonical_unit`, `unit_by_symbol`, and `convert_for_dimension`. The product boundary imports those at `core/product_physics/src/lib.rs:54`, calls `normalize_model_units` before `build_model` at lines 867–876, normalizes nonlinear `gap` as `Length`, `friction_coefficient` as dimensionless, and explicit `normal_reaction` as `Force` at lines 4463–4501. Only then are raw `f64` values put into `NonlinearSupport`, `FrictionNormalReaction`, and the internal frame solve. `NonlinearSupportUnitMetadata` separately verifies displacement=`Displacement`, reaction=`Force`, coefficient=`Dimensionless` at nonlinear-support lines 48–75.
- Focused existing boundary tests: nonlinear supports `nonlinear_unit_metadata_uses_explicit_canonical_dimensions`; product `invalid_load_unit_blocks_with_diagnostic` plus friction-preview input/derived-normal tests; core units `multiplicative_conversions_use_reviewed_public_definitions`, `incompatible_dimensions_are_rejected`, and `conversion_can_be_bound_to_semantic_dimensions_with_shared_units`.
- Repair relationship: M1-N-008 is already an internal canonical-SI harness (`N/m`, `N`, `m`, dimensionless mu) and bypasses the public quantity conversion boundary by design. The proposed current-normal correction changes no unit API or conversion constant. Product tests must separately prove equivalent authored-unit inputs normalize before reaching unchanged internal solver values if product source is touched.
- Missing/surviving hold: internal SI dimensional consistency is present; this evidence does **not** satisfy PDU-035's formal REVIEW/human disposition or DEL-02-02's independent canonical conversion/normalization validation and wider B2/B3 holds. PROJECT_SETUP should keep those distinct from E009's narrow consumed-interface currency.

## Requested PROJECT_SETUP disposition through root

For each row, evaluate the exact consumed interface and focused current-head test evidence rather than target lifecycle alone. F4 recommends `CURRENT_FOR_NARROW_REPAIR` for E006–E009 if the actual focused runs pass, with explicit non-closure annotations for the surviving formal/Owner holds above. If the v3.1/DAG rules do not permit that disposition while target lifecycle is `IN_PROGRESS`, keep the affected implementation held and return the exact rule; do not rewrite the dependency rows from F4.
