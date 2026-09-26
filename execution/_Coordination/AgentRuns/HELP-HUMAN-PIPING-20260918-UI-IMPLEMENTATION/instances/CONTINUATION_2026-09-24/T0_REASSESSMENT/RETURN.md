# T0 — closure reassessment of M01, M03, M05, M08, M14, M33 and M34 against merged PR905

TASK (Type 2), launched by HELP_HUMAN (ROOT). Read-only on product source. Basis: worktree at main `eb56e1083`, which contains the PR905 merge `23aad15d6`. No Piping product path differs between `23aad15d6` and `eb56e1083`. Return goes to ROOT, who integrates it. The independent check of the first version was CLEAR, with four should-fix items (SF-1–SF-4) and three notes (N-5–N-7). This revision applies all of them in place. The dispositions are unchanged.

## Summary

None of the seven groups closes. All seven are **OPEN (partial)** under the graph's closure rule.

PR905 put real, independently checked fixes on main, but they sit behind one gate: the opt-in exact profile (`schema_version` 0.3.0 with `pressure_contract` 2.0.0/`exact_straight_pressure_v2`, producer `physics-1`), plus the retained-source method. On that route main now has:

- the exact straight-pipe pressure composition;
- six signed support components;
- the circular-section normal-stress maximum;
- case-governing headline maxima;
- full-precision publication;
- the M03-INTEGRITY-v1 structural checks.

The ordinary route is what a fresh desktop model uses by default (schema 0.2.0). It solves with `numerical_quality: checks_passed` (producer `precision-1`), and its output is Current-eligible and feeds reports and exports without any disclosure (see the cross-cutting section). On that route the original M05, M14 and M33 defects are unchanged. A read-only probe on main shows them:

- **M33:** the headline maximum is taken from the first case only. It reports 22.77 MPa while the second case reaches 34.16 MPa.
- **M14:** the summary is the sum of absolute bending components. That is up to √2 high (exactly √2 for equal My = Mz). It reports 22.77 MPa where the section maximum is 16.10 MPa.
- **M05:** a pure 500 N·m tip torque publishes only a 0 N reaction force norm and no support reaction moment component. The anchor moment is recoverable only from the member torsion row (−500 N·m).

The M08 defects are also unchanged. The SIF×flexibility review multiplier still enters linear combination algebra: two equal and opposite cases that combine to zero stress publish 28.28 MPa.

The exact profile refuses bends, components, nonlinear and constant-effort supports, combinations and equivalent-static cases, so none of those can reach the corrected semantics yet. M03 and M34 have their core mechanisms merged. Three things remain open for them:

- range support, including the subnormal PHYS-R4 refusal;
- accuracy for general coupled systems, since the bounded retained-source method only covers ≤2-DOF blocks;
- the display, SI and persistence consumers.

## Method

1. For each group, I read the original requirement from its row in `SOLVER_FINDINGS_ASSESSMENT/ASSESSMENT.md` and its appendix rows (`_run_records/{elements,stress,loads,supports,verification,app}/RETURN.md`).
2. I read the product source on main directly. All line numbers below are at `eb56e1083`. `PP` means `projects/chirality-piping/core/product_physics/src/lib.rs`, and `PR` means `.../product_physics/src/pressure_runtime.rs`.
3. I reran the existing targeted tests read-only, with no product, test, fixture, reference or tolerance change.
4. I wrote one standalone read-only probe crate outside the repository. It calls the public `run_linear_static_preview_with_mode` entrypoint and compares the results with hand statics.

The logs, probe source and output are under `_run_records/`, listed at the end.

Verification classes used below:

- **Independent**: the expectation was derived by someone other than the implementer, or frozen before the implementation was read, with no production helpers imported. Examples are the VALIDATION_FOUNDATION first-static comparisons, the ROOT native analytical scripts, the `pressure_runtime.rs` frozen oracles, the STRESS_REFERENCE X1 companion and the NUM `fixtures.json` references.
- **Implementer-analytical**: the physics manager wrote a hand-statics expectation without calling production helpers, but is not independent of the implementation.
- **Regression**: the test asserts existing behaviour, not correctness.

## Cross-cutting fact: two live result routes

| Route | Admission | Producer | Status on main |
|---|---|---|---|
| Ordinary | model 0.1.0/0.2.0, or 0.3.0 with `legacy_pressure_v1`, with zero or absent pressure. A fresh desktop model is authored at `SUPPORTED_MODEL_SCHEMA_VERSION = "0.2.0"` (`apps/desktop/src/services/projectService.ts:36,466`; `apps/desktop/src-tauri/src/model_document_migration.rs:23`) | `precision-1` | Fresh solves are accepted. Nonzero legacy pressure is refused with `PRESSURE_MODEL_REAUTHOR_REQUIRED` (`PR:193-210`). Legacy reaction, summary, maxima and combination semantics are unchanged. |
| Exact | 0.3.0 + `exact_straight_pressure_v2`. Opt-in via the Model-level "Queue pressure profile" operation (`apps/desktop/src/features/pressure-authoring/PressureAuthoringPanel.tsx:5,77-78`) | `physics-1` / `physics-source-1` | Corrected semantics. Refuses components/fittings (`PR:152-160`), nonlinear and constant-effort supports (`PR:161-166`), combinations (`PR:186-189`), equivalent-static (`PR:214-220`) and objective connectors (`PR:140-150`). |

0.3.0 is reached only through the opt-in "Queue pressure profile" operation.

The independent check traced what happens to an ordinary result in the application, and I spot-checked the cited sites:

1. **Solve path.** The native solve runs `solve_preview_mechanics_with_mode` → `run_linear_static_preview_value_with_mode` (`apps/desktop/src-tauri/src/lib.rs:1557-1564`). There `pressure_runtime::is_exact` is false, so it publishes `precision-1` with `checks_passed`.
2. **Current and rule-check eligibility.**
   - `numericalResultStanding` returns `integrity_checked` with `eligible: true`.
   - `buildAnalysisRunV03` accepts the result.
   - `currentSolvedResult` has no route restriction (`apps/desktop/src/features/workspace/resultsSessionState.ts:56-74`).
   - The result is rule-check `numerically_eligible`.
3. **Report and export.**
   - It is the only route the legacy report package accepts (`reportPackageRequest.ts:231-236`); the physics methods are refused there.
   - Stress-neutral export (`StressNeutralExportPanel.tsx:484-490`) and result export accept it.
4. **Headlines.** It feeds the ReportPanel, NativePackagePanel, HandoffPanel and LocalFeaHandoffPanel headlines.
5. **Disclosure.** Nothing discloses the four defects: first-case headline, abs-sum summary, norm-only reactions, and SIF rows in linear combinations. The only guards are:
   - the refusal of nonzero legacy pressure;
   - the professional human-review boundary;
   - the absence of user rule packs.

A corrected capability that exists only on an opt-in route, while the default route keeps the defect and its output is Current and exportable, does not satisfy "fixed as a whole". This is the main reason M05, M14 and M33 stay open even though their exact-route parts are well verified.

**Route decision needed.** What happens to the ordinary route is one cross-cutting ROOT/graph decision, and it affects M05, M14 and M33 together. The options are:

- repair the ordinary route;
- retire it for fresh solves;
- contain it for fresh solves;
- switch the authoring default to the exact profile.

No current tranche owns this decision. My per-group mapping of ordinary-route repairs to T6 below is a proposal only. Containment alone (refusal, or labelling the route non-Current) would not close any group, because the group's required capability would still be missing on that route.

## M01 — pressure

**Requirement.** From ASSESSMENT M-01, ELEM-3, STR-01, STR-04, LOAD-01/09/10 and VER-02:

- Reconcile wall force, effective force, end thrust and membrane stress under one declared convention.
- Integrate the dormant exact-annulus kernel and cover the free, restrained, separately-supported-closure and mixed thermal cases.
- Replace the circular TP-008 premise prospectively with independent oracles.
- Verify connected straight–bend continuity, pressure-region transitions and expansion-joint thrust.
- Block unsupported pressure explicitly.

**On main.**

- The exact profile validates regions, bases, members, two terminals and closure-transfer tokens without inference (`PR:106-340`).
- `build_pressure_case` keeps the pressure-Poisson eigenload, cap-transfer ledger and mechanical loads separate (`PR:1-5`, `PR:400`).
- Recovery publishes `pipe_wall_axial_force_v2`, `pipe_effective_axial_force_v2` (S = Nw − pAi), `pipe_axial_membrane_stress_v2` and Lamé radial/hoop stresses at five stations. Cap transfer is not subtracted in wall recovery (`PP:8749-8912`).
- Legacy nonzero pressure is refused on every non-exact model (`PR:193-210`).
- The formulation limits are stated in the envelope (`PP:827-842`).

**Verified.**

- Independent, `core/product_physics/tests/pressure_runtime.rs` (14 tests; expectations frozen before the implementation was read):
  - `six_si_pressure_states_through_both_public_solver_modes` (free/fixed × transfer/separate × thermal);
  - `mixed_closures_use_the_actual_cap_load_path_for_support_reactions`;
  - `signed_pressure_zero_poisson_and_thermal_reversal_preserve_the_selected_equations`;
  - `exact_pressure_rotates_as_vectors_and_normalizes_mm_mpa_inputs`;
  - `namespaces_and_nonzero_legacy_pressure_cannot_silently_fallback`.
- Independent, `pressure_section_geometry.rs` (9 tests, maintained reference fixtures): near-incompressible, two-span, rotation and thin-wall cases.
- `pressure_grouping_limits.rs` (1) and `pressure_membrane_range.rs` (2): explicit range refusals.
- Independent native witness (`_run_records/JOINED_ENGINE_NATIVE/RETURN.md`): pressure plus six actions, 51 checks per mode, including wall/effective/membrane/Lamé fields. It also covers authoring refusals of missing nu or closure and the dependency-deletion refusals.
- All of these pass on main (see the logs).
- None of the four VALIDATION_FOUNDATION comparisons has pressure: both cases use `pressure_regions: []`.

**Remaining.** The graph lists the M01 remainder in T4. The T1/T2/T3/T5/T6 assignments below are my proposed split, for ROOT to record when each tranche is activated.

- Pressure in bends and fittings, and straight–bend continuity: all components are refused, so the legacy bend's inconsistent recovery is unreachable rather than fixed. **T4**.
- Expansion-joint/connector pressure thrust: legacy pressure is refused and `OBJECTIVE_CONNECTOR_NOT_IMPLEMENTED`, so EJ thrust is currently unavailable. **T4** (M07).
- Unequal-bore and region transitions outside the supported collinear equal-bore chain. **T4**.
- Pressure combined with load and reference states. **T1**.
- Hydrostatic head, contents and hydrotest. **T2** (M21).
- Pressure with nonlinear or constant-effort supports. **T5**.
- Exact-profile combinations. **T6**.
- Subnormal-range positive solve (PHYS-R4 is publicly refused). **T3**.
- Bourdon and pressure-stiffening remain separately qualified options (T4, if selected).

**Disposition: OPEN (partial).** The exact straight composition is fixed, independently verified and merged. The group's bend, joint, transition and composition obligations are not.

## M03 — mechanisms and solve integrity

**Requirement.** From M-03, ELEM-4, VER-01 and SUP-17:

- Scaled stability and pivot diagnostics, per-component rigid-mode detection and positive-definiteness distinct from LU pivot sign.
- A blocking original-system residual gate with node/DOF mapping, in both modes.
- Validation on skew mechanisms, disconnected parts, compliant legitimate systems and unit changes.
- No raw global threshold.

**On main.**

- `M03-INTEGRITY-v1` covers the represented contribution audit, symmetry evidence, Cholesky/LDL factorization with a negative-energy witness, and an original-equation componentwise residual with bounded refinement (`core/solver/frame_kernel/src/structural.rs:1-80, 697-975, 1108-1116`). The sparse mode has its equivalent (`core/solver/sparse_direct/src/structural.rs:18-60`).
- Both modes dispatch through `AssemblyEvidence::solve` (`core/solver/nonlinear_integration/src/structural_adapter.rs:252-285`), called from `PP:3297-3378`.
- Mechanism, negative-energy, assembly-unresolved and sensitive outcomes publish blocking or warning diagnostics with the global DOF map (`PP:867-906`) and per-case `numerical_quality` (`PP:909-1001`).
- The legacy unscaled `solve_dense`/sparse observations no longer select a solution (`PP:1940`, `PP:3339-3378`).
- Sensitive or rejected ordinary attempts can be replaced by the bounded retained-source method. The gate is a captured invocation with no nonlinear supports and no combinations (`PP:1864`).

**Verified.**

- Frame kernel: 85 unit tests plus 1, including `soft_diagonal_and_unit_congruence_are_not_mechanisms`, `lost_stabilizer_retains_unresolved_instead_of_physical_mechanism` and `original_residual_kills_wrong_solution_wrong_matrix_and_missing_load`.
- Sparse direct: 25 tests.
- Product library (268 passed, 1 ignored resource diagnostic), including `integrity_oblique_torsion_requires_actual_rotational_ground_in_both_modes`, `integrity_disconnected_body_is_not_restrained_by_another_bodys_anchor` and the `integrity_exact_case_ids_*` tests.
- Independent: `source_block_recovery.rs::retained_source_rotation_member_torque_and_device_action_reach_public_rows`, covering N05/N06 × sign × rotation × both modes at the unchanged 1e-9 criterion.
- Independent: the native N05/N06 witnesses.
- The M03 designs are independently reviewed. The unit tests are mostly implementer-authored with derived expectations.

**Remaining** (all **T3** unless marked):

- Retained-source recovery is limited to free blocks of order ≤2 on straight unmodified members with no pressure regions, combinations or nonlinear supports. Larger, skewed or weakly coupled systems are not covered.
- PHYS-R4: a subnormal transformation allowance is refused before the solve (`pressure_membrane_range.rs`), so the range capability is open.
- The physical rigid-null witness is unqualified for bodies with user-matrix or curved elements (`structural_adapter.rs:260-264`).
- Nonlinear mixed recovery publishes `NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED` (`PP:2012-2036`). **T5** (M06).
- The ground-DOF precheck still names "missing global rigid-body DOF classes" (`PP:1337-1357`), which SUP-17 said to reword. This item alone would keep M03 open.
- VP-ROBUST coverage (rotations, scales, larger systems) does not exist yet.

Related but separate: N05/N06 ordinary-method accuracy is the graph's own T3 item (N05 ordinary accuracy), not an M03 requirement. The ordinary method still misses 1e-9 on N05/N06; this is contained as Sensitive or rejected, not repaired.

**Disposition: OPEN (partial).** The integrity policy and diagnostics are merged and tested. Accuracy and range for general systems are not.

## M05 — reactions

**Requirement.** From M-05, ELEM-8, STR-08/15, VER-05, SUP-08 and APP-4:

- Publish signed Fx/Fy/Fz/Mx/My/Mz per support in an explicit frame, with global force and moment equilibrium, and keep the resultant only as a derived quantity.
- Repair coincident nonlinear attribution.
- Carry the result through combinations, UI, export and report.
- Verify pure torque, springs, contact, shared DOFs, frame changes and combinations.

**On main.**

- Exact route: `append_signed_support_results` publishes six `support_reaction_component_v2` rows in the global frame with a support-on-pipe sign convention, plus separate force and moment magnitudes (`PP:8712-8747`, called at `PP:2183-2198`).
- Ordinary route with source recovery selected: six components (`PP:2159-2175`).
- Ordinary route otherwise: only `reaction_resultant`, the norm of the three translation slots (`PP:2180-2211`).
- Combination vectors are three-component only (`support_force_vectors`, `PP:2182`; `append_combined_vector_magnitude`, `PP:10277`).
- The nonlinear support slot still reads the nodal reaction without checking the selected state (`PP:2148-2157`).
- Consumers of the six-component rows exist: `core/reporting/result_export/src/physics_source.rs`, `source_blocks.rs`, and the desktop `features/results/physicsResultEvidence.ts`.
- The legacy report package refuses the richer methods instead of dropping them.

**Verified.**

- Implementer-analytical, `support_reactions_runtime.rs` (4):
  - `six_nonzero_support_components_follow_global_force_and_moment_balance`, with a rotated variant and both modes;
  - `two_coincident_tip_springs_and_root_rigid_support_have_distinct_actions`;
  - `duplicate_rigid_devices_cannot_each_publish_the_complete_nodal_reaction`;
  - `same_record_spring_cannot_silently_replace_an_extra_rigid_guide`.
- Independent: the VALIDATION_FOUNDATION axial and bending/torsion cases compare all six components and both magnitudes in both modes (`_run_records/ORDINARY_PHYSICS_ADAPTER/RETURN.md`; 76 gate tests rerun here, all passed).
- Independent: native six-action witnesses and the source-recovery spring action.
- T0 probe (`_run_records/t0_probe_output.log`): ordinary pure torque 500 N·m publishes only a 0 N reaction force norm (`reaction_resultant`) and no support reaction moment component. The anchor moment is recoverable only from the member torsion row (−500 N·m). The exact route gives support Mx = −500 N·m.

**Remaining.**

- The ordinary route is still norm-only. This depends on the cross-cutting route decision (see the cross-cutting section). No tranche owns it; T6 is my proposal if repair is chosen.
- Nonlinear-support, constant-effort and hanger attribution, including the inactive-contact shared-DOF read. **T5**.
- Signed six-component combinations and envelopes; exact combinations are refused. **T6**.
- Support-local and nozzle-local frames. **T6/T7**.
- Report and structured-export carriage for the physics methods (M26/M36). **T6**.

**Disposition: OPEN (partial).**

## M08 — SIF and stress intensification

**Requirement.** From M-08, ELEM-5 and STR-02/03:

- Separate stiffness flexibility from stress intensification: no i×k.
- Recover directional signed moments at bends and branches, and apply a named SIF to the correct components, ends and stations.
- Exclude derived absolute stress magnitudes from linear and subtraction combination algebra.
- Verify invariance of stress to flexibility when the forces are held fixed.

**On main.** Unchanged from the assessment basis.

- `component_user_stress_multiplier_review` still multiplies the abs-sum open-formula summary by `sif*flexibility` in geometry-only mode, or by SIF alone in the macro-bend mode (`PP:9142-9152`).
- `is_combination_excluded_result_kind` does not exclude that kind (`PP:10584-10600`), so it goes through scalar algebra (`PP:10380-10500`).
- The regression test `tests::bend_component_user_multipliers_emit_stress_review_rows` asserts that the row "should participate in explicit combinations" (`PP:15210-15227`).
- The exact profile refuses components, so no corrected route exists.

**Verified.** Nothing verifies a fix. The T0 probe confirms the defect: two equal and opposite cases combined 1.0 + 1.0 give 0 for every signed bending stress and 0 N reaction, but the modifier row is 28.28 MPa, which is 2 × 14.14 MPa and equals 11.385 × 1.15 × 1.08 per case.

**Remaining.**

- The whole group. Directional recovery and SIF separation: **T4**.
- Exclusion from combination algebra. The graph lists M08 in T4; moving this piece to **T6** with M15 is my proposed split, for ROOT to record at activation. The ordinary-route instance also depends on the route decision.

**Disposition: OPEN.** Effectively unimplemented. The only change is that the exact profile refuses components.

## M14 — summary stress

**Requirement.** From M-14, STR-05 and ELEM-15:

- Define the summary quantity.
- Use the resultant bending magnitude for the circular-section extreme normal stress, including the analytic extremum search, and expose torsion separately or as a named equivalent.
- Verify axis-rotation invariance, biaxial bending, pure torsion, combined loading and interior extrema.

**On main.**

- Exact route: `pipe_elastic_normal_stress_maximum_v2` = max |Nw/As| + hypot(My,Mz)/Z, bounded over all straight statics intervals, with torsion kept separate (`PP:2688-2716`, `exact_straight_summary_extrema` at `PP:7487`).
- Retained-source route: `retained_source_endpoint_normal_max_v1`, limited to unloaded straight spans (`PP:2665-2687`; `source_receipt/endpoint_maximum.rs`).
- Ordinary route and all bends: `open_formula_summary_mpa` still computes |σy| + |σz| (`PP:8930-8955`), and it feeds the headline and the SIF rows. `straight_summary_extrema` (`PP:7570`) does not call it, but it computes the same abs-sum quantity through the eight sign-mask maxima (`PP:7612-7640`).
- The exact headline still travels in `summary.max_open_formula_stress`, now in Pa. That is a naming carry-over.

**Verified.**

- Independent: `elastic_extrema_runtime.rs` (2), `actual_x1_loads_find_peak_missed_by_old_signed_sum_candidates` and `pure_torque_has_zero_normal_maximum_and_signed_torsion`.
- Independent: the VALIDATION_FOUNDATION bending/torsion case, which has biaxial My/Mz, torsion and `pipe_elastic_normal_stress_maximum_v2` in both modes, plus 36 nested section checks.
- Independent: the native root normal maxima.
- T0 probe, ordinary case A: equal 1000 N tip forces in y and z. The summary is 22.770 MPa = 2000/Z; the hand-statics section maximum is √2·1000/Z = 16.101 MPa. Exact route: 16.101 MPa.

**Remaining.**

- The ordinary-route and curved-bend summaries still use the abs-sum. The bend part is **T4**. The ordinary-route part depends on the cross-cutting route decision, which no tranche owns yet.
- Direct transverse-shear stress output and signed circumferential fibre outputs (`VALIDATION_FOUNDATION/_run_records/FIRST_STATIC_BINDINGS/OUTPUT_OBLIGATIONS.json`). **T4**, with results carriage in **T6**.
- A retained-source maximum for loaded spans. **T3/T4**.

**Disposition: OPEN (partial).**

## M33 — maxima and mixed moduli

**Requirement.** From M-33, STR-09/10 and LOAD-07:

- Summarize all requested cases with explicit scope, giving a genuine governing maximum with case references.
- Handle combinations validly.
- Diagnose or reject mixed-modulus ranges, or require an explicit compatible basis.
- Verify case-order permutation, a larger later case and missing quantities.

**On main.**

- Exact or source-selected: `maximum_across_cases` is case-governing, withholds the headline on an incomplete domain, and breaks ties deterministically (`PP:1665-1701`, selected at `PP:1498-1507`).
- Ordinary: `load_case_solves.first()` only (`PP:1500-1501`, `1505-1506`).
- `HIGH_DISPLACEMENT_REVIEW` inherits the same scope (`PP:1584-1595`).
- Mixed moduli: `append_combination_modulus_basis_records` still emits presence records only, with no compatibility diagnosis (`PP:7271-7330`).
- Combination maxima are not summarized; exact combinations are refused.

**Verified.**

- Implementer-analytical, `stress_maximum_coverage.rs` (4):
  - `incomplete_case_domain_withholds_overall_maximum_in_both_case_orders`;
  - `incomplete_member_domain_withholds_case_maximum_in_both_member_orders`;
  - `exact_zero_displacement_ties_use_location_identity_not_node_order`;
  - `exact_zero_stress_ties_use_location_identity_not_member_order`.
- Independent: `pressure_runtime.rs::same_region_id_in_distinct_cases_has_unique_binding_and_order_independent_maximum`.
- Independent: the native mixed-methods model, where the global maximum belongs to the pressure case.
- The validation comparisons are single-case and do not test governing maxima.
- T0 probe, ordinary route with case A listed first: the headline is 22.77 MPa from case A, while case B's own row is 34.16 MPa. The exact route governs correctly at 34.155 MPa from case B.

**Remaining.**

- Ordinary first-case headline. This depends on the cross-cutting route decision, which no tranche owns; T6 is my proposal if repair is chosen.
- Mixed-modulus diagnosis. **T6**, with the T1 reference-state basis.
- Combination-scoped maxima. **T6**.
- Bend and pressure maxima. **T4**.

**Disposition: OPEN (partial).**

## M34 — tolerances and rounding

**Requirement.** From M-34, STR-13 and VER-07:

- Store full precision and round only for display.
- Carry precision through export, user-rule inputs and persistence.
- Adopt dimension-aware residual and comparison rules with near-zero floors, and scale- and unit-invariance tests.
- Keep protected criteria unless an evidenced change is approved.

**On main.**

- Publication quantization is gone. `round6` is `#[cfg(test)]` only (`PP:11064-11067`), and the envelope declares `value_representation: finite_binary64` with `publication_quantization: none` (`PP:1003-1011`).
- `serde_json` has `float_roundtrip` (`core/product_physics/Cargo.toml`).
- The residual metric is row-normalized and dimensionless (`frame_kernel/src/structural.rs:697-760`).
- Display rounds only at presentation, to 4 significant digits (`apps/desktop/src/services/unitConversion.ts:81-85`).
- The standalone `openpipestress_jcs_binary64_v1` profile (PR901) is not consumed by any product, runner or desktop result path: grep finds it only in `core/serialization/canonical_json` and `tools/serialization`.
- The protected benchmark predicate is still absolute 1e-9 for most fixtures (`validation/benchmarks/mechanics/src/lib.rs:1437-1441`), unchanged by design.

**Verified.**

- `precision_signed_subquantum_torsion_both_modes_matches_independent_annulus`, `precision_same_unit_raw_quantity_bits_and_numeric_strings_roundtrip`, `precision_nonfinite_values_are_rejected_not_serialized_as_null` and `historical_round6_demonstrates_subquantum_information_loss`, all passing.
- `mixed_units_are_normalized_at_preview_mechanics_boundary_without_pressure` and the exact mm/MPa normalization test.
- The source-recovery test `actual_source_recovery_scaled_norms_admit_tiny_actions_and_keep_large_json_boundary`.
- `derived_stress_unit_range_cannot_mint_source_qualification`, a guard.
- PR905 interchange checks (reported in `ENGINE_INTEGRATION/RETURN.md`, not rerun here).

**Remaining.**

- Range: subnormal allowance refusal, and no relative representability bound on UI cross-unit display of subnormal or zero projections (`ENGINE_INTEGRATION/RETURN.md`, "Conversion scope"). **T3**.
- SI carrier and scientific-profile adoption in product transport. **T3**.
- A dimension-aware comparison policy for the benchmark suite, through its owning decision. **T3/T9**.
- Persistence of manifest bodies (`HISTORICAL_INPUT_MANIFEST_MISSING`) and regeneration of the precision-1 fixture pair. **T6**.
- The DEL-10-05 witness inputs and their generator still carry `product_physics` 0.1.0. They are not T6's: the graph holds them open until they are regenerated in their own scope.

**Disposition: OPEN (partial).**

## VP-SCOPE delta

The capability map is `projects/chirality-piping/validation/qualification/capability_inventory.json`. Its `basis.main` is `eec2855d`, with the engine at the unmerged checkpoint `22452ecd`. It was not edited. Corrections against merged main `23aad15d6`/`eb56e1083`:

1. **`basis`:** add merged main `23aad15d6`. Engine `22452ecd` is now a merged ancestor. The numerical `ed688`/`45d412` and scientific `05a9baa` checkpoints are not ancestors, and their content is superseded by merged PR905 (`23aad15d6`) and PR901 (`72da655ac`). Keep `ancestor_of_pinned_main: false` for them. The graph snapshot `d90e4ee0` is also not an ancestor.
2. **`contracts.production_cli.actual_mode_dispatch` and `harness_gaps.HG-09`:** "main: sparse_interactive only" is outdated. The main CLI has a closed `solve --solver-mode sparse_interactive|dense_scrutiny` (`core/runner/headless/src/bin/openpipestress-runner.rs:212`). Mark HG-09 resolved at the product level; the physics-1 adapter on main binds it.
3. **`straight_static`:** stays merged_partial. Update the formulation to record that M03-INTEGRITY-v1 is merged in both modes and that CLI dense mode exists. The Euler–Bernoulli-only and dense-global-matrix (M32) limits stand.
4. **`pressure_thermal`:** checkpoint_partial → merged_partial.
   - Implemented: the exact straight circular profile, thermal plus pressure in one case, explicit refusal of legacy nonzero pressure.
   - Unsupported and refused: pressure in bends and components, joints and connectors, nonlinear or constant-effort supports, combinations, equivalent-static, hydrostatic head.
   - Delete the limit "22452 … composite semantics/schema/headless/native work incomplete". Composite `physics-source-1`, the schemas, headless and native witnesses are merged.
5. **`stress_results`:** checkpoint_partial → merged_partial, split by route.
   - Exact/physics-1 is implemented: six signed components, circular maximum, case-governing maxima.
   - Ordinary/precision-1 still has norm-only reactions, the abs-sum summary and the first-case headline. Any profile case on that route must mark these outputs unsupported rather than count them.
   - The FE-01 loss is now a refusal. Report carriage is unsupported for the physics methods. The report package carries only precision-1, with its defective headline and norm-only reactions.
   - Direct transverse-shear and signed circumferential fibre outputs are unsupported.
6. **`bend`:** add the known-incorrect behaviour. The SIF review rows still use i×k in geometry-only mode and still enter linear combinations (M08, STR-03). The exact profile refuses bends, so there is no corrected bend stress route.
7. **`linear_restraints`:** the signed six-component contract is implemented on the exact and source routes only. On the ordinary route it is unsupported, and there is no local or skew frame. `HG-07` ("main lacks full signed support/recovery/current-method contracts") becomes partial: resolved for physics-1, physics-source-1 and source-blocks-1; open for precision-1.
8. **`numerical_integrity`:** checkpoint_partial → merged_partial.
   - Implemented: M03-INTEGRITY-v1, source-blocks-1 and physics-source-1, no publication quantization.
   - Partial: N05/N06 are correct only through bounded retained-source recovery.
   - Unsupported: the subnormal range (PHYS-R4), and the general >2-DOF coupled, pressure, combination or nonlinear source recovery.
   - The limit "Scientific transport is standalone; product adoption … open" still holds.
   - FE-02: the graph records it repaired in the joined candidate. Not re-verified here.
9. **`output_sets.support_actions`:** "engine/source checkpoint support_reaction_component_v2" should read "merged physics-1 and source routes"; the precision-1 scalar still cannot stand in.
10. **`findings`:** M01, M03, M05, M08, M14, M33 and M34 stay `integration_state: open`, which is correct.

## Limits of this assessment

- This is a source reading at `eb56e1083`, plus read-only reruns of existing Rust and Python tests and one probe. I did not run the native app, browser, hosted CI, a DEC-025 sweep or a full workspace test suite.
- Native and validation evidence is taken from the PR905 records. The native witness was taken at `8b982aa7`, and ROOT reused it for PR905.
- I inspected the desktop, export and report consumers only far enough to confirm that they exist or refuse. Their correctness is not assessed here.
- Some claims are taken from PR905 records rather than re-verified: FE-01/FE-02, interchange and persistence.
- The probe uses my hand statics. It is independent of production but not independently reviewed. Its inputs are invented; no material or component library or code rule was used.
- Line numbers refer to `eb56e1083` and will drift.
- The graph requires an independent check of this return before ROOT records the dispositions.
- The Python gate tests triggered a cached cargo `release` build in an ignored target directory. `git status` shows no tracked or untracked change outside this directory, apart from the pre-existing untracked `OWNER_RESUME_2026-09-26.md`, which this task did not create.

## Run records

All under `_run_records/`. Paths are repo-relative; `{REPO_ROOT}` and `${CARGO_TARGET_DIR}` replace machine paths.

- `product_physics_integration.log`: 10 integration test binaries, 62 tests, all passed.
- `product_physics_lib.log`: 268 passed, 1 ignored (the private resource measurement).
- `frame_kernel.log` (86 passed) and `sparse_direct.log` (25 passed).
- `validation_gate_pytest.log`: five `tests/test_qualification_*`/`test_first_static_selection.py` files, 76 passed plus 83 subtests.
- `t0_probe/` (`Cargo.toml.txt`, `src/main.rs.txt`, SHA256SUMS) and `t0_probe_output.log`. ROOT renamed the two probe files to `.txt` at integration so that tools discovering Cargo manifests cannot treat this record as a crate; the bytes are unchanged and still match SHA256SUMS.
- `toolchain.txt`: the toolchains actually used.
  - The Cargo tests ran with cwd under `projects/chirality-piping`, whose rustup directory override selects rustc/cargo 1.97.1.
  - The probe ran from a scratch directory on the default stable toolchain, 1.94.1.
  - The first version of this file recorded only 1.94.1, because it was captured outside the override. It has been corrected.

Rerun:

- Cargo tests: from `projects/chirality-piping/core/product_physics`, run `CARGO_TARGET_DIR=<scratch> cargo test --offline --locked --lib`, and again with `--test <name>` for each integration file. Do the same in `core/solver/frame_kernel` and `core/solver/sparse_direct`.
- Probe: copy `t0_probe/` outside the repository, drop the `.txt` suffixes, set its dependency path to the absolute path of `projects/chirality-piping/core/product_physics` in your checkout (the recorded run used the default 1.94.1 toolchain, outside the project override), then run `cargo run --offline -q`.
- Python gate: run `python -m pytest -q` on the five test files, using the DEC-025 venv.
