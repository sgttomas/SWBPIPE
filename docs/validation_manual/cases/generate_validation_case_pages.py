#!/usr/bin/env python3
"""Generate per-case validation-manual pages from existing repository evidence.

Each generated page assembles the PRD section 16.5 record fields (purpose,
input model, independent reference, expected result, software result,
tolerance, pass/fail, solver version) for one existing validated case, using
only artifacts already committed in the live tree:

- hand-calculation witness notes under `validation/hand_calcs/`;
- fixture constructors and suite tests in `validation/benchmarks/*/src/lib.rs`;
- governed DEC-046 policy records under `validation/benchmarks/nonlinear/`;
- the recorded suite runs referenced in the TP-E2-VALMANUAL-001 run record.

The generator is deterministic: rerunning it against the same tree reproduces
the committed pages byte-for-byte. Problem statements are extracted from the
`## Purpose` section of each hand-calc note where present, so the note remains
the single source; short registry fallbacks cover the older notes that predate
the `## Purpose` convention.

Hard fences honored by construction: no threshold is invented, tightened
beyond evidence, or loosened — tolerance text only records what the named
suite constants, fixture slots, and governed DEC-046/DEC-024/DEC-026 records
already say, and `TBD` entries stay `TBD`. No protected standards content,
catalog values, or defaults are emitted. No release-readiness, professional,
certification, or code-compliance claim is emitted.
"""

from __future__ import annotations

import argparse
import re
from dataclasses import dataclass, field
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
OUT_DIR = Path(__file__).resolve().parent

RUN_RECORD = (
    "execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/"
    "DEL-09-04_Validation manual skeleton/_run_records/"
    "WORKING_ITEMS_RUN_2026-07-10_TP-E2-VALMANUAL-001.md"
)

RECORDED_RUN_DATE = "2026-07-10"
RECORDED_TOOLCHAIN = "rustc 1.92.0 / cargo 1.92.0"


@dataclass
class Suite:
    key: str
    title: str
    deliverable: str
    crate: str
    evidence_class: str
    hand_calc_dir: str
    recorded_result: str


SUITES = {
    "mechanics": Suite(
        key="mechanics",
        title="Mechanics benchmark suite",
        deliverable="DEL-09-01",
        crate="validation/benchmarks/mechanics",
        evidence_class="Mechanics verification",
        hand_calc_dir="validation/hand_calcs/mechanics",
        recorded_result="ok. 30 passed; 0 failed; 0 ignored",
    ),
    "stress": Suite(
        key="stress",
        title="Stress recovery benchmark suite",
        deliverable="DEL-09-02",
        crate="validation/benchmarks/stress",
        evidence_class="Mechanics verification (stress recovery)",
        hand_calc_dir="validation/hand_calcs/stress",
        recorded_result="ok. 22 passed; 0 failed; 0 ignored",
    ),
    "nonlinear": Suite(
        key="nonlinear",
        title="Nonlinear support regression suite",
        deliverable="DEL-09-03",
        crate="validation/benchmarks/nonlinear",
        evidence_class="Mechanics verification (nonlinear supports)",
        hand_calc_dir="validation/hand_calcs/nonlinear",
        recorded_result="ok. 19 passed; 0 failed; 0 ignored",
    ),
}


TOLERANCE_TEXTS = {
    "mech_numeric": (
        "Numeric agreement assertions in this suite use the fixture-local "
        "absolute assertion epsilon `INTERNAL_ASSERTION_EPSILON = 1.0e-9` "
        "recorded in `validation/benchmarks/mechanics/src/lib.rs`; structural "
        "and deterministic expectations (prepared boundaries, diagnostics, "
        "routing, provenance) are asserted exactly. This matches the "
        "`DEC-026` analytic-class relative seed magnitude but the fixture's "
        "`tolerance_policy` slots remain unresolved: the governed "
        "per-quantity-kind tolerance record required by `DEC-024`/`DEC-026` "
        "remains `TBD`, and this page neither tightens nor loosens any "
        "governed value."
    ),
    "dec092_analytic": (
        "The independent mechanics comparison reuses the existing "
        "`DEC-024`/`DEC-026` analytic-class relative tier `1.0e-9`. The "
        "fixture's `tolerance_policy` remains unresolved (`None`): this case "
        "selects no new governed tolerance, release threshold, CI gate, "
        "publication scope, external-validation claim, or reliance basis."
    ),
    "expansion_loop": (
        "The witness-table comparison uses the measured, reason-annotated "
        "fixture-local constants in "
        "`validation/benchmarks/mechanics/src/lib.rs`: reaction and "
        "displacement relative tolerance `5.0e-7` (limiting factor: residual "
        "axial flexibility of the boosted bending-only comparison model, "
        "measured max deviation `5.8e-8` relative at axial-rigidity boost "
        "`1.0e5`; `u_y(T2)` measured max `4.5e-8`; `u_x(T2)` measured max "
        "`6.0e-8`), whole-body equilibrium absolute floors `1.0e-3` N / "
        "`1.0e-3` N-m (measured max residuals `2.9e-5` N and `7.1e-5` N-m), "
        "and free-expansion self-check floors `1.0e-8` m / `1.0e-2` N. The "
        "comparison replicates the witness bending-only-flexibility "
        "assumption per the witness verification appendix. The fixture's "
        "`tolerance_policy` slots remain unresolved: adoption into the "
        "governed `DEC-024`/`DEC-026` tolerance record remains `TBD` and is "
        "a separate governance step."
    ),
    "stress_numeric": (
        "Numeric agreement assertions in this suite use the fixture-local "
        "absolute assertion epsilon `INTERNAL_ASSERTION_EPSILON = 1.0e-9` "
        "recorded in `validation/benchmarks/stress/src/lib.rs`; structural "
        "and blocking-diagnostic expectations are asserted exactly. The "
        "fixture's `tolerance_policy` slots remain unresolved: the governed "
        "per-quantity-kind tolerance record required by `DEC-024`/`DEC-026` "
        "remains `TBD`, and this page neither tightens nor loosens any "
        "governed value."
    ),
    "nl_classifier": (
        "This classifier-level case asserts exact deterministic expected "
        "outcomes (support state, reaction, and diagnostic expectations) — "
        "no numeric tolerance band applies. The governed `DEC-046` "
        "convergence-tolerance record applies to the assembled global loop, "
        "not to this classifier-level case, and the fixture's "
        "`tolerance_policy` slots remain unresolved (`TBD` per "
        "`DEC-024`/`DEC-026`)."
    ),
    "nl_nonconvergence": (
        "This case verifies diagnostic behavior, not numeric agreement: the "
        "fixture must fail to converge within the iteration cap and must "
        "report a single `NonConvergence` failure diagnostic (message "
        "records `did not converge after 4 iterations`). No success state is "
        "substituted. The fixture's `tolerance_policy` slots remain "
        "unresolved (`TBD` per `DEC-024`/`DEC-026`)."
    ),
    "nl_seed": (
        "Convergence is judged under the governed `DEC-046` record "
        "`DEC-046-CV-B-active-set-count-validation-v1` "
        "(`validation/benchmarks/nonlinear/convergence_policy.dec046.json`): "
        "active-set changed-support-count residual with relative tolerance "
        "`0.0`, absolute floor `0.0`, and max-iteration cap `4` per "
        "nonlinear class, scoped to the current public-original assembled "
        "validation seed only. Final-iteration residuals ride the companion "
        "governed records for free-DOF force/moment, free-DOF work, general "
        "energy, and displacement/reaction deltas "
        "(`*_policy.dec046.json` in the same directory). Fixture-local "
        "overrides may only tighten; loosening is a new governance event. "
        "Broader release/production convergence thresholds remain `TBD`."
    ),
    "nl_depth_obs": (
        "This is an observation-only depth case: it converges in the "
        "recorded run but is deliberately excluded from the governed "
        "`DEC-046` seed policies, its convergence-policy status is `TBD`, "
        "and the solve carries the active `TolerancePolicyTbd` diagnostic. "
        "No threshold is recorded for it; the `TBD` entries stay `TBD` "
        "until filled by measurement under a governance event."
    ),
    "nl_multisupport": (
        "Convergence is judged under the narrow governed `DEC-046` "
        "multi-support records "
        "(`DEC-046-CV-B-multisupport-active-set-count-validation-v1` and "
        "companions in "
        "`validation/benchmarks/nonlinear/multisupport_*.dec046.json`): "
        "active-set changed-support-count residual with relative tolerance "
        "`0.0`, absolute floor `0.0`, max-iteration cap `4`, plus the "
        "multi-support free-DOF force/moment, free-DOF work, general-energy, "
        "and displacement/reaction-delta records — scoped to the accepted "
        "public-original multi-DOF / multi-support fixture set only. "
        "Fixture-local overrides may only tighten; loosening is a new "
        "governance event. Broader release/production thresholds remain "
        "`TBD`."
    ),
}


@dataclass
class Case:
    fixture_id: str
    suite: str
    note: str
    tests: list[str]
    tolerance_key: str
    purpose_fallback: str | None = None
    extras: list[str] = field(default_factory=list)
    created: str = "2026-07-10"
    recorded_run_date: str = RECORDED_RUN_DATE
    recorded_toolchain: str = RECORDED_TOOLCHAIN
    recorded_result: str | None = None
    run_record: str = RUN_RECORD
    reproduction_commands: list[str] | None = None
    expected_result_detail: str | None = None
    software_result_detail: str | None = None
    evidence_detail: str | None = None
    runner_reproduction_detail: str | None = None
    pass_fail_detail: str | None = None
    solver_version_detail: str | None = None


MECHANICS_CASES = [
    Case(
        "MECH-CANTILEVER-TIP-FORCE",
        "mechanics",
        "cantilever_tip_force.md",
        ["cantilever_tip_force_matches_open_mechanics_formula"],
        "mech_numeric",
    ),
    Case(
        "MECH-PORTAL-SWAY-ORIGINAL",
        "mechanics",
        "portal_frame_sway.md",
        ["portal_frame_fixture_solves_repeatably"],
        "mech_numeric",
    ),
    Case(
        "MECH-BRANCH-ASSEMBLY-THREE-MEMBER",
        "mechanics",
        "branch_assembly.md",
        ["branch_assembly_fixture_matches_open_stiffness_network"],
        "mech_numeric",
        purpose_fallback=(
            "Verify the PRD section 16.2 branch-assembly solver benchmark: a "
            "three-member invented frame network (run, riser, and branch) is "
            "assembled, solved, and compared against elementary open "
            "frame-member stiffness relationships recorded in the reference "
            "note."
        ),
    ),
    Case(
        "MECH-STRAIGHT-PIPE-WEIGHT-RECOVERY",
        "mechanics",
        "straight_pipe_weight_recovery.md",
        ["straight_pipe_fixture_exercises_weight_and_recovery_path"],
        "mech_numeric",
    ),
    Case(
        "MECH-SUPPORT-BOUNDARY-MIXED",
        "mechanics",
        "support_boundary_mixed.md",
        ["support_boundary_fixture_prepares_mixed_supports"],
        "mech_numeric",
    ),
    Case(
        "MECH-PRIMITIVE-LOAD-PREP",
        "mechanics",
        "primitive_load_preparation.md",
        ["primitive_load_fixture_prepares_all_contribution_types"],
        "mech_numeric",
    ),
    Case(
        "MECH-TP-PHYS-002-LINEAR-STATIC-INTEGRATION",
        "mechanics",
        "tp_phys_002_linear_static_integration.md",
        ["tp_phys_002_integrated_fixture_solves_recovers_and_maps_diagnostics"],
        "mech_numeric",
    ),
    Case(
        "MECH-TP-PHYS-004-LOAD-TO-RESULTANT",
        "mechanics",
        "tp_phys_004_load_to_resultant.md",
        ["tp_phys_004_load_to_resultant_fixture_assembles_solves_and_recovers"],
        "mech_numeric",
    ),
    Case(
        "MECH-TP-PHYS-005-ORIENTED-LOAD-TO-RESULTANT",
        "mechanics",
        "tp_phys_005_oriented_load_to_resultant.md",
        ["tp_phys_005_oriented_load_to_resultant_fixture_transforms_global_loads"],
        "mech_numeric",
    ),
    Case(
        "MECH-TP-PHYS-006-PARTIAL-SPAN-LOAD-TO-RESULTANT",
        "mechanics",
        "tp_phys_006_partial_span_load_to_resultant.md",
        ["tp_phys_006_partial_span_load_to_resultant_fixture_routes_spanned_loads"],
        "mech_numeric",
    ),
    Case(
        "MECH-TP-PHYS-007-STATION-SWEEP-RESULTANTS",
        "mechanics",
        "tp_phys_007_station_sweep_resultants.md",
        ["tp_phys_007_station_sweep_fixture_preserves_requested_order_and_values"],
        "mech_numeric",
    ),
    Case(
        "MECH-FIXED-FIXED-THERMAL-AXIAL",
        "mechanics",
        "fixed_fixed_thermal_axial.md",
        ["thermal_growth_fixture_records_open_axial_restraint_formula"],
        "mech_numeric",
    ),
    Case(
        "MECH-TP-PHYS-008-THERMAL-PRESSURE-AXIAL-EFFECTS",
        "mechanics",
        "tp_phys_008_thermal_pressure_axial_effects.md",
        ["tp_phys_008_thermal_pressure_fixture_prepares_and_recovers_axial_effects"],
        "mech_numeric",
    ),
    Case(
        "MECH-TP-PHYS-009-COMBINED-LOAD-AXIAL-EFFECTS",
        "mechanics",
        "tp_phys_009_combined_load_axial_effects.md",
        ["tp_phys_009_combined_fixture_assembles_solves_and_recovers_axial_effects"],
        "mech_numeric",
    ),
    Case(
        "MECH-TP-PHYS-014-CANONICAL-ANALYTICAL-PAYLOAD",
        "mechanics",
        "tp_phys_014_canonical_analytical_payload.md",
        ["tp_phys_014_canonical_analytical_payload_drives_solver_consumption"],
        "mech_numeric",
        extras=[
            "The committed canonical payload fixture is "
            "`validation/benchmarks/mechanics/fixtures/"
            "tp_phys_014_canonical_analytical_payload.json`."
        ],
    ),
    Case(
        "MECH-TP-PHYS-015-CANONICAL-SOLVE-RESULT-ENVELOPE",
        "mechanics",
        "tp_phys_015a_canonical_solve_result_envelope.md",
        ["tp_phys_015a_canonical_payload_builds_result_boundary_evidence"],
        "mech_numeric",
    ),
    Case(
        "MECH-IMPOSED-DISPLACEMENT-SPRING",
        "mechanics",
        "imposed_displacement_spring.md",
        ["imposed_displacement_fixture_prepares_support_boundary"],
        "mech_numeric",
    ),
    Case(
        "MECH-INCLINED-MEMBER-TRANSFORM",
        "mechanics",
        "inclined_member_transform.md",
        ["inclined_member_transform_fixture_checks_direction_cosines_and_symmetry"],
        "mech_numeric",
    ),
    Case(
        "MECH-EXPANSION-LOOP-CURVED-BEND-THERMAL",
        "mechanics",
        "expansion_loop_curved_bend_thermal.md",
        [
            "expansion_loop_fixture_matches_witness_reference_table",
            "expansion_loop_free_expansion_identity_is_stress_free",
            "expansion_loop_t2_x_displacement_is_k_independent_free_shortening",
            "expansion_loop_k_sweep_reactions_decrease_strictly_monotonically",
            "expansion_loop_reactions_satisfy_whole_body_equilibrium",
            "expansion_loop_stiffer_elbow_forces_larger_departure_from_free_growth",
        ],
        "expansion_loop",
        extras=[
            "This case is the D-34 / `DEC-070` exit-evidence comparison for "
            "the curved-bend macro element, including the k-sweep "
            "monotonicity evidence for the user-entered in-plane flexibility "
            "factors `k in {1, 5, 10, 20}`. The factor `k` is an opaque "
            "user-entered value; no flexibility-factor equation, catalog "
            "value, SIF, or default is used."
        ],
    ),
    Case(
        "MECH-TP-PMM-P3-OCCLOADGEN-EQUIVALENT-STATIC",
        "mechanics",
        "tp_pmm_p3_occloadgen_equivalent_static.md",
        [
            "occloadgen_generation_matches_witness_intensities",
            "occloadgen_generated_loads_pass_boundary_and_lump_to_end_nodes",
            "occloadgen_generation_blocks_without_user_inputs_or_marked_spans",
        ],
        "mech_numeric",
    ),
    Case(
        "MECH-TP-DEC092-TEMPERATURE-INDEXED-SHEAR-MODULUS-TORSION",
        "mechanics",
        "tp_dec092_temperature_indexed_shear_modulus_torsion.md",
        ["temperature_indexed_shear_modulus_torsion_matches_independent_oracle"],
        "dec092_analytic",
        purpose_fallback=(
            "Verify D-45 Option B / `DEC-092` temperature-indexed shear "
            "modulus with an invented hollow straight-pipe cantilever in "
            "pure torsion. Exact-ID and adjacent-interpolation selected `G` "
            "values are compared with an independent elementary-mechanics "
            "oracle while a distinct base `G` witnesses forbidden fallback."
        ),
        created="2026-08-09",
        recorded_run_date="2026-08-03",
        recorded_toolchain="rustc 1.97.1 / cargo 1.97.1",
        recorded_result=(
            "PASS; producing mechanics suite 39 passed with zero failures, "
            "and the commit-bound sweep records Cargo 36/36 and overall pass"
        ),
        run_record=(
            "execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/"
            "DEL-05-02_Load-case algebra engine/_run_records/"
            "WORKING_ITEMS_RUN_2026-08-02_DEC092_TEMPERATURE_G_IMPLEMENTATION.md"
        ),
        reproduction_commands=[
            "cargo test --offline --manifest-path validation/benchmarks/mechanics/Cargo.toml temperature_indexed_shear_modulus_torsion_matches_independent_oracle",
            "cargo test --offline --manifest-path core/product_physics/Cargo.toml dec092_fixture_consumes_exact_and_interpolated_g_with_provenance_and_combination_carry_through",
            "cargo test --offline --manifest-path core/product_physics/Cargo.toml selected_point_g_is_base_independent_and_selected_g_sensitive",
            "cargo test --offline --manifest-path core/product_physics/Cargo.toml selected_basis_blocks_missing_invalid_or_dimensionally_wrong_point_g",
            "cargo test --offline --manifest-path core/product_physics/Cargo.toml temperature_g_interpolation_uses_adjacent_points_and_duplicate_temperatures_still_block",
            "cargo test --offline --manifest-path core/product_physics/Cargo.toml interpolation_blocks_at_and_beyond_stored_range_edges",
            "cargo test --offline --manifest-path core/product_physics/Cargo.toml exact_and_interpolated_basis_fields_are_mutually_exclusive",
            "cargo test --offline --manifest-path core/product_physics/Cargo.toml base_material_values_are_used_when_no_modulus_basis_is_named",
        ],
        expected_result_detail=(
            "The independent note and mechanics fixture record `J = "
            "1.0540043352793751e-5 m^4`, exact-point `G = 50.0e9 Pa`, and "
            "strictly adjacent interpolation between `TP-LOW` (`300 K`, "
            "`60.0e9 Pa`) and `TP-HIGH` (`500 K`, `40.0e9 Pa`) at `425 K`, "
            "giving `f = 0.625` and `G = 47.5e9 Pa`. The resulting exact-ID "
            "and interpolated tip rotations are `9.108121929551094e-2 rad` "
            "and `9.587496767948521e-2 rad`. The deliberately distinct base "
            "value `G_base = 80.0e9 Pa` gives `5.6925762059694338e-2 rad`; "
            "substituting it misses the selected targets by `37.5%` and "
            "`40.625%`."
        ),
        software_result_detail=(
            "Product-physics tests separately prove that exact-ID selection "
            "consumes the selected point's explicit `G`; temperature "
            "selection uses the strictly adjacent bracket and records both "
            "source IDs plus `method=linear_temperature_interpolation`; and "
            "combination records carry the same basis. They also prove that "
            "selection blocks at or outside stored range endpoints without "
            "extrapolation, blocks missing, non-positive, non-finite, or "
            "dimensionally wrong selected-point `G`, rejects duplicate "
            "temperatures and conflicting exact/temperature selectors, and "
            "never falls back to base `G`. A load case with no modulus basis "
            "continues to consume the explicit base `G`. The mechanics test "
            "is an independent numeric oracle; it does not by itself prove "
            "product selection behavior."
        ),
        evidence_detail=(
            "Implementation commit `c394365ca72b8383c7d7203ce5be2cb9ea67d508`; "
            "passing commit-bound sweep "
            "`validation/evidence/sweeps/SWEEP_20260803T194132Z_c394365ca72b.json`; "
            "mechanics fixture/oracle and named test in "
            "`validation/benchmarks/mechanics/src/lib.rs`; product behavior "
            "and the seven named tests in `core/product_physics/src/lib.rs`. "
            "Receipt-87 records closeout and routes this derivative, but is "
            "not used as the sole basis for any mechanics or product claim."
        ),
        runner_reproduction_detail=(
            "Reproduction is through the listed locked/offline crate tests. "
            "No GUI or external-runner reproduction was executed for this "
            "case."
        ),
        pass_fail_detail=(
            "`PASS` as development evidence at the producing run and "
            "commit-bound sweep: the independent mechanics oracle and the "
            "product behavior tests passed. Evidence state remains "
            "`DRAFT_EVIDENCE`; public tolerances, maintainer review, GUI "
            "validation, release, and professional reliance remain open."
        ),
        solver_version_detail=(
            "The in-repo OpenPipeStress solver and benchmark crates at "
            "implementation commit `c394365ca72b8383c7d7203ce5be2cb9ea67d508`, "
            "bound by the passing sweep above; toolchain rustc 1.97.1 / "
            "cargo 1.97.1. No packaged release version exists; release labels "
            "remain `TBD`."
        ),
    ),
]

STRESS_CASES = [
    Case(
        "STRESS-AXIAL-NORMAL-ORIGINAL",
        "stress",
        "axial_normal.md",
        ["recovers_axial_normal_fixture"],
        "stress_numeric",
        purpose_fallback=(
            "Verify axial normal stress recovery: the recovered axial stress "
            "must equal the supplied axial force divided by the supplied "
            "cross-sectional area of the invented section."
        ),
    ),
    Case(
        "STRESS-BENDING-NORMAL-ORIGINAL",
        "stress",
        "bending_normal.md",
        ["recovers_bending_normal_fixture"],
        "stress_numeric",
        purpose_fallback=(
            "Verify bending normal stress recovery: each recovered bending "
            "stress component must equal the supplied bending moment divided "
            "by the corresponding supplied section modulus."
        ),
    ),
    Case(
        "STRESS-TORSIONAL-SHEAR-ORIGINAL",
        "stress",
        "torsional_shear.md",
        ["recovers_torsional_shear_fixture"],
        "stress_numeric",
        purpose_fallback=(
            "Verify torsional shear stress recovery: the recovered torsional "
            "shear must equal the supplied torque times the supplied radius "
            "divided by the supplied torsion constant."
        ),
    ),
    Case(
        "STRESS-PRESSURE-MEMBRANE-ORIGINAL",
        "stress",
        "pressure_membrane.md",
        ["recovers_pressure_membrane_fixture"],
        "stress_numeric",
        purpose_fallback=(
            "Verify thin-wall pressure membrane stress recovery from "
            "explicit user-supplied pressure basis inputs, within the "
            "mechanics-only stress recovery boundary."
        ),
    ),
    Case(
        "STRESS-RANGE-MECHANICS-ORIGINAL",
        "stress",
        "stress_range.md",
        [
            "computes_mechanics_only_stress_range_fixture",
            "stress_range_blocks_asymmetric_optional_pressure_components",
        ],
        "stress_numeric",
        purpose_fallback=(
            "Verify mechanics-only stress range computation between two "
            "solved states: the range is the component-by-component absolute "
            "difference of recovered mechanics components (not an equivalent "
            "stress and not a code stress range), and asymmetric optional "
            "pressure components must block with a diagnostic rather than "
            "collapse into a silent success."
        ),
    ),
    Case(
        "STRESS-INTEGRATED-STRAIGHT-PIPE-ORIGINAL",
        "stress",
        "integrated_straight_pipe_resultants.md",
        ["integrated_straight_pipe_resultants_feed_stress_recovery_fixture"],
        "stress_numeric",
    ),
    Case(
        "STRESS-TP-PHYS-004-LOAD-TO-RESULTANT",
        "stress",
        "tp_phys_004_load_to_resultant_stress.md",
        ["load_to_resultant_station_stress_fixture_recovers_bending_component"],
        "stress_numeric",
    ),
    Case(
        "STRESS-TP-PHYS-005-ORIENTED-LOAD-TO-STRESS",
        "stress",
        "tp_phys_005_oriented_load_to_stress.md",
        ["oriented_load_to_stress_fixture_recovers_midspan_bending_from_global_model"],
        "stress_numeric",
    ),
    Case(
        "STRESS-TP-PHYS-006-PARTIAL-SPAN-LOAD-TO-STRESS",
        "stress",
        "tp_phys_006_partial_span_load_to_stress.md",
        [
            "partial_span_load_to_stress_fixture_recovers_midspan_bending_from_spanned_load"
        ],
        "stress_numeric",
    ),
    Case(
        "STRESS-TP-PHYS-007-STATION-SWEEP-STRESS",
        "stress",
        "tp_phys_007_station_sweep_stress.md",
        ["station_sweep_stress_fixture_preserves_order_and_recovers_stresses"],
        "stress_numeric",
    ),
    Case(
        "STRESS-TP-PHYS-008-THERMAL-AXIAL-EFFECT-TO-STRESS",
        "stress",
        "tp_phys_008_thermal_axial_effect_to_stress.md",
        ["thermal_axial_effect_resultants_feed_mechanics_only_stress_recovery"],
        "stress_numeric",
    ),
    Case(
        "STRESS-TP-PHYS-009-COMBINED-AXIAL-BENDING-TO-STRESS",
        "stress",
        "tp_phys_009_combined_axial_bending_to_stress.md",
        ["combined_axial_effect_and_bending_resultants_feed_station_stress_recovery"],
        "stress_numeric",
    ),
    Case(
        "STRESS-TP-PHYS-015-CANONICAL-RESULTANT-STRESS-RECOVERY",
        "stress",
        "tp_phys_015_canonical_resultant_stress.md",
        ["canonical_analytical_payload_resultants_feed_station_stress_recovery"],
        "stress_numeric",
        extras=[
            "The authoritative machine-readable hand-calc witness for the "
            "companion section-property/stress derivation is "
            "`validation/witness/fixtures/"
            "tp_phys_015_section_property_stress_witness.json`, with the "
            "deterministic MathML rendering at "
            "`validation/witness/generated/"
            "tp_phys_015_section_property_stress_witness.mathml`."
        ],
    ),
    Case(
        "STRESS-TP-PMM-P3-MILLTOL-EFFECTIVE-WALL-STRESS",
        "stress",
        "tp_pmm_p3_milltol_effective_wall_stress.md",
        [
            "recovers_milltol_effective_wall_fixture",
            "milltol_reduction_strictly_reduces_section_modulus",
        ],
        "stress_numeric",
    ),
    Case(
        "STRESS-TP-PMM-P3-MODULUSBASIS-RANGE-STRESS",
        "stress",
        "tp_pmm_p3_modulusbasis_range_stress.md",
        ["recovers_modulusbasis_range_fixture_with_recorded_bases"],
        "stress_numeric",
    ),
]

NL_CLASSIFIER_TESTS = [
    "fixtures_are_public_original_and_unit_aware",
    "active_set_gap_lift_off_and_friction_outcomes_are_deterministic",
]
NL_SEED_TESTS = [
    "assembled_global_loop_seed_cases_converge_under_governed_policy",
    "assembled_convergence_observations_record_measured_fixture_values_only",
    "assembled_force_displacement_residual_observations_use_free_dof_force_moment_thresholds",
]
NL_DEC067_TESTS = NL_SEED_TESTS + [
    "dec_067_transition_fixtures_witness_state_switched_and_bounded_outcomes",
]
NL_MULTISUPPORT_TESTS = [
    "multisupport_acceptance_inventory_uses_narrow_dec_046_policy",
]

NONLINEAR_CASES = [
    Case(
        "NL-ACTIVE-ONE-WAY-ORIGINAL",
        "nonlinear",
        "active_set_one_way.md",
        NL_CLASSIFIER_TESTS,
        "nl_classifier",
    ),
    Case(
        "NL-GAP-CLOSURE-ORIGINAL",
        "nonlinear",
        "gap_closure.md",
        NL_CLASSIFIER_TESTS,
        "nl_classifier",
    ),
    Case(
        "NL-LIFT-OFF-ORIGINAL",
        "nonlinear",
        "lift_off.md",
        NL_CLASSIFIER_TESTS,
        "nl_classifier",
    ),
    Case(
        "NL-FRICTION-STICK-SLIDE-ORIGINAL",
        "nonlinear",
        "friction_transition.md",
        NL_CLASSIFIER_TESTS,
        "nl_classifier",
    ),
    Case(
        "NL-NONCONVERGENCE-LIMIT-ORIGINAL",
        "nonlinear",
        "unresolved_nonconvergence.md",
        [
            "nonconvergence_fixture_reports_failure_diagnostic",
            "expected_reports_preserve_warning_and_failure_statuses",
        ],
        "nl_nonconvergence",
    ),
    Case(
        "NL-ASSEMBLED-ONE-WAY-DEACTIVATE-ORIGINAL",
        "nonlinear",
        "assembled_one_way_deactivation.md",
        NL_SEED_TESTS,
        "nl_seed",
    ),
    Case(
        "NL-ASSEMBLED-GAP-CLOSURE-ORIGINAL",
        "nonlinear",
        "assembled_gap_closure.md",
        NL_SEED_TESTS,
        "nl_seed",
    ),
    Case(
        "NL-ASSEMBLED-LIFT-OFF-ORIGINAL",
        "nonlinear",
        "assembled_lift_off.md",
        NL_SEED_TESTS,
        "nl_seed",
    ),
    Case(
        "NL-ASSEMBLED-FRICTION-STICK-ORIGINAL",
        "nonlinear",
        "assembled_friction_sticking.md",
        NL_SEED_TESTS,
        "nl_seed",
    ),
    Case(
        "NL-ASSEMBLED-FRICTION-SLIDE-ORIGINAL",
        "nonlinear",
        "assembled_friction_sliding.md",
        NL_SEED_TESTS,
        "nl_seed",
    ),
    Case(
        "NL-ASSEMBLED-FRICTION-DERIVED-NORMAL-ORIGINAL",
        "nonlinear",
        "assembled_friction_derived_normal.md",
        NL_SEED_TESTS,
        "nl_seed",
    ),
    Case(
        "NL-ASSEMBLED-ONE-WAY-REENGAGE-ORIGINAL",
        "nonlinear",
        "assembled_one_way_reengagement.md",
        NL_DEC067_TESTS,
        "nl_seed",
        extras=["This is a `DEC-067` state-transition repair witness case."],
    ),
    Case(
        "NL-ASSEMBLED-GAP-LIFT-OFF-ORIGINAL",
        "nonlinear",
        "assembled_gap_lift_off.md",
        NL_DEC067_TESTS,
        "nl_seed",
        extras=["This is a `DEC-067` state-transition repair witness case."],
    ),
    Case(
        "NL-ASSEMBLED-FRICTION-BOUNDED-SLIDE-ORIGINAL",
        "nonlinear",
        "assembled_friction_bounded_sliding.md",
        NL_DEC067_TESTS,
        "nl_seed",
        extras=["This is a `DEC-067` state-transition repair witness case."],
    ),
    Case(
        "NL-ASSEMBLED-MULTI-DOF-MULTI-SUPPORT-OBS-ORIGINAL",
        "nonlinear",
        "assembled_multi_support_multi_dof.md",
        ["multisupport_depth_inventory_is_observation_only_and_separate_from_seed"],
        "nl_depth_obs",
    ),
    Case(
        "NL-ASSEMBLED-MULTI-DOF-MULTI-SUPPORT-ACCEPTED-ORIGINAL",
        "nonlinear",
        "assembled_multi_support_multi_dof_acceptance.md",
        NL_MULTISUPPORT_TESTS,
        "nl_multisupport",
    ),
    Case(
        "NL-ASSEMBLED-MULTI-DOF-GAP-LIFT-OFF-ACCEPTED-ORIGINAL",
        "nonlinear",
        "assembled_multi_support_gap_lift_off_acceptance.md",
        NL_MULTISUPPORT_TESTS,
        "nl_multisupport",
    ),
    Case(
        "NL-ASSEMBLED-MULTI-DOF-FRICTION-GAP-ACCEPTED-ORIGINAL",
        "nonlinear",
        "assembled_multi_support_friction_gap_acceptance.md",
        NL_MULTISUPPORT_TESTS,
        "nl_multisupport",
    ),
    Case(
        "NL-ASSEMBLED-MULTI-DOF-THREE-SUPPORT-ACCEPTED-ORIGINAL",
        "nonlinear",
        "assembled_multi_support_three_dof_acceptance.md",
        NL_MULTISUPPORT_TESTS,
        "nl_multisupport",
    ),
    Case(
        "NL-ASSEMBLED-MULTI-DOF-ROTATIONAL-ACCEPTED-ORIGINAL",
        "nonlinear",
        "assembled_multi_support_rotational_acceptance.md",
        NL_MULTISUPPORT_TESTS,
        "nl_multisupport",
    ),
    Case(
        "NL-ASSEMBLED-MULTI-DOF-DERIVED-NORMAL-GAP-ACCEPTED-ORIGINAL",
        "nonlinear",
        "assembled_multi_support_derived_normal_gap_acceptance.md",
        NL_MULTISUPPORT_TESTS,
        "nl_multisupport",
    ),
    Case(
        "NL-ASSEMBLED-MULTI-DOF-DERIVED-NORMAL-ROTATIONAL-ACCEPTED-ORIGINAL",
        "nonlinear",
        "assembled_multi_support_derived_normal_rotational_acceptance.md",
        NL_MULTISUPPORT_TESTS,
        "nl_multisupport",
    ),
    Case(
        "NL-ASSEMBLED-MULTI-DOF-CASCADE-GAP-LIFT-OFF-ACCEPTED-ORIGINAL",
        "nonlinear",
        "assembled_multi_support_cascade_gap_lift_off_acceptance.md",
        NL_MULTISUPPORT_TESTS,
        "nl_multisupport",
    ),
    Case(
        "NL-ASSEMBLED-MULTI-DOF-NEGATIVE-GAP-ACCEPTED-ORIGINAL",
        "nonlinear",
        "assembled_multi_support_negative_gap_acceptance.md",
        NL_MULTISUPPORT_TESTS,
        "nl_multisupport",
    ),
    Case(
        "NL-ASSEMBLED-MULTI-DOF-FOUR-CLASS-ACCEPTED-ORIGINAL",
        "nonlinear",
        "assembled_multi_support_four_class_acceptance.md",
        NL_MULTISUPPORT_TESTS,
        "nl_multisupport",
    ),
    Case(
        "NL-ASSEMBLED-MULTI-DOF-OPPOSING-GAPS-ACCEPTED-ORIGINAL",
        "nonlinear",
        "assembled_multi_support_opposing_gaps_acceptance.md",
        NL_MULTISUPPORT_TESTS,
        "nl_multisupport",
    ),
    Case(
        "NL-ASSEMBLED-MULTI-DOF-TWO-SPAN-ACCEPTED-ORIGINAL",
        "nonlinear",
        "assembled_multi_support_two_span_acceptance.md",
        NL_MULTISUPPORT_TESTS,
        "nl_multisupport",
    ),
    Case(
        "NL-ASSEMBLED-MULTI-DOF-TWO-SPAN-OPPOSING-GAPS-ACCEPTED-ORIGINAL",
        "nonlinear",
        "assembled_multi_support_two_span_opposing_gaps_acceptance.md",
        NL_MULTISUPPORT_TESTS,
        "nl_multisupport",
    ),
]

ALL_CASES = MECHANICS_CASES + STRESS_CASES + NONLINEAR_CASES


def extract_purpose(note_path: Path) -> str | None:
    text = note_path.read_text(encoding="utf-8")
    match = re.search(r"^## Purpose\n(.*?)(?=^## )", text, re.M | re.S)
    if not match:
        return None
    return match.group(1).strip()


def find_constructor(lib_source: str, fixture_id: str) -> str | None:
    for match in re.finditer(re.escape(f'fixture_id: "{fixture_id}"'), lib_source):
        prefix = lib_source[: match.start()]
        fn_match = None
        for fn_match in re.finditer(r"(?:pub )?fn (\w+)\(", prefix):
            pass
        if fn_match and fn_match.group(1).endswith("_fixture"):
            return fn_match.group(1)
    return None


def slug(fixture_id: str) -> str:
    return fixture_id.lower()


def render_case(case: Case, constructor: str | None) -> str:
    suite = SUITES[case.suite]
    note_rel = f"{suite.hand_calc_dir}/{case.note}"
    purpose = extract_purpose(ROOT / note_rel)
    if purpose is None:
        purpose = case.purpose_fallback
    assert purpose, f"no purpose available for {case.fixture_id}"

    if constructor:
        input_model = (
            f"The machine-readable input model is the fixture constructor "
            f"`{constructor}()` (fixture id `{case.fixture_id}`) in "
            f"`{suite.crate}/src/lib.rs`, carrying explicit fixture-local "
            f"units and public-original provenance metadata. The reference "
            f"note records the same invented inputs with the longhand "
            f"derivation."
        )
    else:
        input_model = (
            f"The machine-readable input model is the fixture registered "
            f"with fixture id `{case.fixture_id}` in "
            f"`{suite.crate}/src/lib.rs`, carrying explicit fixture-local "
            f"units and public-original provenance metadata. The reference "
            f"note records the same invented inputs with the longhand "
            f"derivation."
        )

    reproduction_commands = case.reproduction_commands or [
        f"cargo test --manifest-path {suite.crate}/Cargo.toml {name}"
        for name in case.tests
    ]
    test_lines = "\n".join(reproduction_commands)
    test_list = ", ".join(f"`{name}`" for name in case.tests)

    if case.pass_fail_detail is not None:
        pass_fail = case.pass_fail_detail
    elif case.fixture_id == "NL-NONCONVERGENCE-LIMIT-ORIGINAL":
        pass_fail = (
            "`PASS` (expected-diagnostic case) at the recorded run: the "
            "fixture failed to converge exactly as the reference predicts "
            "and reported the expected `NonConvergence` failure diagnostic. "
            "This is software-quality evidence only."
        )
    else:
        pass_fail = (
            "`PASS` at the recorded run above: every named test asserted "
            "the measured-vs-reference expectations and completed with "
            "`ok`. This is software-quality evidence only."
        )

    extras = "".join(f"\n{extra}\n" for extra in case.extras)
    expected_result = case.expected_result_detail or (
        "The expected values and their derivation are recorded in the independent\n"
        f"reference note `{note_rel}` and mirrored in the fixture's expected-value\n"
        "slots. The reference derivation uses elementary open mechanics only."
    )
    software_result_detail = (
        f"\n\n{case.software_result_detail}" if case.software_result_detail else ""
    )
    recorded_result = case.recorded_result or suite.recorded_result
    evidence_detail = f"\n\nEvidence basis: {case.evidence_detail}" if case.evidence_detail else ""
    runner_reproduction = case.runner_reproduction_detail or (
        "Headless-runner binding: the `DEC-065` `openpipestress-runner run-benchmark`\n"
        "verb accepts a schema-first benchmark request but currently exits `1` with\n"
        "the structured blocking diagnostic\n"
        "`HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD` because\n"
        "benchmark payload binding remains future bounded `DEL-10-05` work. Until that\n"
        "binding lands, the exact reproduction command for this case is the crate test\n"
        "above; the runner-reproducible external path is recorded in\n"
        "[headless_runner_reproduction.md](../../headless_runner_reproduction.md)."
    )
    solver_version = case.solver_version_detail or (
        "The in-repo OpenPipeStress solver and benchmark crates at the commit recorded\n"
        f"in `{case.run_record}`; toolchain {case.recorded_toolchain}. No packaged release version\n"
        "exists; release labels remain `TBD`."
    )

    return f"""---
doc_id: OPS-VALIDATION-MANUAL-CASE-{case.fixture_id}
doc_kind: governance.validation_manual_case
status: draft_evidence
created: {case.created}
generated_by: docs/validation_manual/cases/generate_validation_case_pages.py
refs:
  - rel: implements
    to: DEL-09-04
  - rel: evidences
    to: {suite.deliverable}
---

# {case.fixture_id}

| Item | Record |
|---|---|
| Suite | {suite.title} (`{suite.deliverable}`), crate `{suite.crate}/` |
| Evidence class | {suite.evidence_class} |
| Evidence state | `DRAFT_EVIDENCE` |
| Independent reference | `{note_rel}` |
| Provenance | Invented public-original fixture data only (recorded in the reference note and fixture metadata) |

## Test Purpose And Problem Statement

{purpose}

All inputs are invented or user-entered fixture data; nothing is copied from
protected standards, commercial software examples, or proprietary data.
{extras}
## Input Model

{input_model}

## Expected Result And Independent Reference

{expected_result}

## Software Result And Reproduction

The measured-vs-reference comparison executes inside the named suite
test(s) {test_list}, which run the current in-repo solver path on the fixture
and assert agreement with the reference expectations.{software_result_detail}

Reproduction (from `projects/chirality-piping`):

```bash
{test_lines}
```

Recorded run: {case.recorded_run_date}, toolchain {case.recorded_toolchain}; suite
result `{recorded_result}` with the named test(s) passing. The full
suite output and the exact commit are recorded in the run record
`{case.run_record}`.{evidence_detail}

{runner_reproduction}

## Tolerance

{TOLERANCE_TEXTS[case.tolerance_key]}

## Pass/Fail

{pass_fail}

## Solver Version

{solver_version}

## Boundary

This case page documents software verification evidence for an invented
public-original fixture. It is development verification and screening
evidence (claims registry BS-VALID, DEC-081), and it does not settle any
`TBD` threshold or governed tolerance decision.
"""


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--fixture-id", help="render exactly one registered fixture id")
    parser.add_argument("--output-root", type=Path, default=OUT_DIR)
    parser.add_argument("--check", action="store_true", help="compare without writing")
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    lib_sources = {
        key: (ROOT / suite.crate / "src" / "lib.rs").read_text(encoding="utf-8")
        for key, suite in SUITES.items()
    }
    cases = ALL_CASES
    if args.fixture_id:
        cases = [case for case in ALL_CASES if case.fixture_id == args.fixture_id]
        if not cases:
            raise SystemExit(f"unknown fixture id: {args.fixture_id}")
    mismatches: list[Path] = []
    for case in cases:
        constructor = find_constructor(lib_sources[case.suite], case.fixture_id)
        page = render_case(case, constructor)
        out_path = args.output_root / case.suite / f"{slug(case.fixture_id)}.md"
        if args.check:
            if not out_path.exists() or out_path.read_text(encoding="utf-8") != page:
                mismatches.append(out_path)
        else:
            out_path.parent.mkdir(parents=True, exist_ok=True)
            out_path.write_text(page, encoding="utf-8")
    if mismatches:
        for path in mismatches:
            print(f"mismatch: {path}")
        raise SystemExit(1)
    verb = "checked" if args.check else "generated"
    print(f"{verb} {len(cases)} case page(s) under {args.output_root}")


if __name__ == "__main__":
    main()
