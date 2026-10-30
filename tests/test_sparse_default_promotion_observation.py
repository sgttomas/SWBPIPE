#!/usr/bin/env python3
"""Guard DEC-053 sparse default-promotion observation and policy boundaries."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OBSERVATION = (
    ROOT
    / "validation"
    / "benchmarks"
    / "sparse_default_promotion_observation.dec053.json"
)
POLICY = (
    ROOT / "validation" / "benchmarks" / "sparse_default_promotion_policy.dec053.json"
)
HARNESS = ROOT / "core" / "solver" / "performance_harness" / "src" / "lib.rs"
PRODUCT = ROOT / "core" / "product_physics" / "src" / "lib.rs"
NONLINEAR = ROOT / "core" / "solver" / "nonlinear_integration" / "src" / "lib.rs"


def test_sparse_default_promotion_packet_closes_named_r4_residual_axes():
    observation = json.loads(OBSERVATION.read_text(encoding="utf-8"))
    policy = json.loads(POLICY.read_text(encoding="utf-8"))
    harness = HARNESS.read_text(encoding="utf-8")
    product = PRODUCT.read_text(encoding="utf-8")
    nonlinear = NONLINEAR.read_text(encoding="utf-8")

    assert observation["record_id"] == "DEC-053-SPARSE-DEFAULT-PROMOTION-OBSERVATION-v1"
    assert observation["decision_ref"] == "DEC-053"
    assert observation["observation_count"] == 9
    assert len(observation["observations"]) == 9
    assert observation["timing_policy"] == "observed_not_threshold_gated"
    assert observation["allocator_rss_policy"] == "observed_not_threshold_gated"
    assert (
        observation["hardware_normalization_policy"]
        == "hardware_metadata_recorded_no_cross_machine_threshold"
    )
    assert observation["ci_gate_status"].endswith("no_hosted_ci")
    assert observation["hardware_metadata"]["os"]
    assert observation["hardware_metadata"]["arch"]
    assert observation["hardware_metadata"]["rss_kib_at_packet_emit"] > 0

    fixture_ids = [item["fixture_id"] for item in observation["observations"]]
    assert fixture_ids[:3] == [
        "invented-cantilever-chain-8",
        "invented-cantilever-chain-24",
        "invented-cantilever-chain-48",
    ]
    assert {"invented-grid-frame-4x3", "invented-grid-frame-6x8"} <= set(
        fixture_ids
    )
    assert len({item["practical_size_band"] for item in observation["observations"]}) == 9

    for item in observation["observations"]:
        assert item["default_sparse_promotion_status"] == (
            "promoted_sparse_interactive_default_dense_scrutiny_available"
        )
        assert item["dense_first_solve_elapsed_nanos"] > 0
        assert item["sparse_first_solve_elapsed_nanos"] > 0
        assert item["dense_reduced_matrix_value_storage_bytes"] > 0
        assert item["sparse_ordered_profile_value_storage_bytes"] > 0
        assert item["allocator_rss_observation_status"].endswith("not_threshold_gated")
        assert item["hardware_normalization_status"] == (
            "hardware_metadata_recorded_no_cross_machine_threshold"
        )
        assert item["true_condition_number_2norm"] >= 1.0
        assert (
            item["true_condition_number_method"]
            == "deterministic_jacobi_symmetric_eigen_extrema_on_reduced_dense_matrix"
        )
        assert item["sparse_pivot_condition_ratio_estimate"] > 0
        assert item["sparse_dense_relative_delta"] <= 1.0e-9
        assert item["max_abs_sparse_residual"] <= 1.0e-6
        assert item["max_abs_sparse_repeat_solution_delta"] == 0
        assert item["nonpositive_pivot_count"] == 0
        assert item["dense_path_role"] == "explicit_dense_scrutiny_and_parity_review"
        assert item["sparse_path_role"] == "default_interactive_preview_and_render_iteration"

    assert policy["record_id"] == observation["policy_ref"]
    assert policy["solver_mode_policy"]["default_mode"] == "sparse_interactive"
    assert policy["solver_mode_policy"]["explicit_secondary_mode"] == "dense_scrutiny"
    assert policy["evidence_requirements"]["observation_count"] == 9
    assert policy["evidence_requirements"]["required_fixture_ids"] == fixture_ids
    assert policy["accepted_thresholds_for_observation_set"] == {
        "sparse_dense_relative_delta_limit": 1.0e-9,
        "sparse_residual_absolute_limit": 1.0e-6,
        "sparse_repeat_solution_delta_absolute_limit": 0.0,
        "nonpositive_pivot_count_limit": 0,
    }
    assert set(policy["closure_status"].values()) == {
        "closed_for_r4_interactive_default",
        "closed_for_r4_as_observation_not_threshold",
        "closed_for_r4_by_local_DEC025_cargo_pytest_coverage",
        "closed_for_r4_by_hardware_metadata_no_cross_machine_threshold",
        "closed_for_r4_by_nine_record_practical_observation_set",
        "closed_for_r4_by_bounded_true_condition_number_observations",
        "closed_for_r4_by_true_condition_number_fields_alongside_pivot_proxy",
    }
    assert "run_sparse_default_promotion_observation_suite" in harness
    assert "DEC_053_SPARSE_DEFAULT_PROMOTION_STATUS" in harness
    assert "deterministic_jacobi_symmetric_eigen_extrema" in harness
    assert "PreviewSolverMode::SparseInteractive" in product
    assert "PreviewSolverMode::DenseScrutiny" in product
    assert "run_linear_static_preview_with_mode" in product
    assert "LinearSolveMode::SparseInteractive" in nonlinear
    assert "LinearSolveMode::DenseScrutiny" in nonlinear

    closure_text = json.dumps(
        {
            "observation": observation,
            "policy": policy,
        },
        sort_keys=True,
    )
    assert "not_promoted_dense_default" not in closure_text
    assert "default sparse promotion remains" not in closure_text
    assert "dense remains the product/default solve path" not in closure_text
