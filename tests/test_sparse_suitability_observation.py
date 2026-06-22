#!/usr/bin/env python3
"""Guard DEC-050 sparse suitability observation and threshold-policy boundaries."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
RECORD = ROOT / "validation" / "benchmarks" / "sparse_suitability_observation.dec050.json"
POLICY = ROOT / "validation" / "benchmarks" / "sparse_suitability_threshold_policy.dec050.json"
SOURCE = ROOT / "core" / "solver" / "performance_harness" / "src" / "lib.rs"


def test_sparse_suitability_observation_record_preserves_dense_default_boundary():
    record = json.loads(RECORD.read_text(encoding="utf-8"))
    policy = json.loads(POLICY.read_text(encoding="utf-8"))
    source = SOURCE.read_text(encoding="utf-8")

    assert record["record_id"] == "DEC-050-SPARSE-SUITABILITY-OBSERVATION-v1"
    assert record["status"] == "bounded_threshold_policy_accepted"
    assert (
        record["threshold_policy_ref"]
        == "DEC-050-SPARSE-SUITABILITY-GENERATED-GRID-THRESHOLD-POLICY-v1"
    )
    assert record["threshold_policy_status"] == "accepted_for_generated_grid_observation_set"
    assert record["default_sparse_promotion_status"] == "not_promoted_dense_default"
    assert (
        record["memory_observation_status"]
        == "deterministic_value_storage_observed_threshold_tbd"
    )
    assert "f64 value-storage bytes only" in record["memory_observation_basis"]
    assert record["timing_memory_threshold_status"] == "tbd"
    assert record["conditioning_ci_threshold_status"] == "tbd"
    assert record["hardware_normalization_status"] == "tbd"
    assert record["dense_path_role"] == "default_solver_and_parity_oracle"
    assert record["evidence_fixture_ids"] == [
        "invented-grid-frame-4x3",
        "invented-grid-frame-6x8",
    ]
    assert "SPARSE_SUITABILITY_GRID_BANDS: [(usize, usize); 2] = [(4, 3), (6, 8)]" in source
    assert "SPARSE_SUITABILITY_RELATIVE_DELTA_LIMIT: f64 = 1.0e-9" in source
    assert "SPARSE_SUITABILITY_RESIDUAL_ABSOLUTE_LIMIT: f64 = 1.0e-6" in source
    assert "SPARSE_SUITABILITY_REPEAT_DELTA_ABSOLUTE_LIMIT: f64 = 0.0" in source
    assert "SPARSE_SUITABILITY_NONPOSITIVE_PIVOT_COUNT_LIMIT: usize = 0" in source
    assert "F64_VALUE_STORAGE_BYTES: usize = std::mem::size_of::<f64>()" in source
    assert "dense_reduced_matrix_value_storage_bytes" in record["observed_metrics"]
    assert "sparse_original_profile_value_storage_bytes" in record["observed_metrics"]
    assert "sparse_ordered_profile_value_storage_bytes" in record["observed_metrics"]
    assert "sparse_ordered_vs_dense_value_storage_ratio" in record["observed_metrics"]
    assert "run_sparse_suitability_observation_suite" in source
    assert "no default sparse promotion is made" in " ".join(record["boundary"])
    assert (
        "deterministic value-storage memory observations are recorded"
        in " ".join(record["boundary"])
    )
    assert "no timing, memory" in " ".join(record["boundary"])

    assert (
        policy["record_id"]
        == "DEC-050-SPARSE-SUITABILITY-GENERATED-GRID-THRESHOLD-POLICY-v1"
    )
    assert policy["status"] == "accepted_for_generated_grid_observation_set"
    assert policy["source_observation_record"] == record["record_id"]
    assert policy["dense_path_role"] == "default_solver_and_parity_oracle"
    assert policy["sparse_path_role"] == "evidence_lane_only"
    assert policy["still_tbd"] == [
        "default_sparse_promotion",
        "timing_memory_thresholds",
        "conditioning_ci_thresholds",
        "hardware_normalization_methodology",
        "practical_size_band_thresholds",
        "release_or_external_validation_thresholds",
    ]
    assert len(policy["entries"]) == 1
    entry = policy["entries"][0]
    assert entry["policy_ref"] == policy["record_id"]
    assert entry["observation_ref"] == record["record_id"]
    assert entry["sparse_dense_relative_delta_limit"] == 1.0e-9
    assert entry["sparse_residual_absolute_limit"] == 1.0e-6
    assert entry["sparse_repeat_solution_delta_absolute_limit"] == 0.0
    assert entry["nonpositive_pivot_count_limit"] == 0
    assert entry["evidence_fixture_ids"] == record["evidence_fixture_ids"]
    assert "Does not make the sparse path the default" in " ".join(policy["limitations"])
