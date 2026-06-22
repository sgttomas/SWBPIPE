#!/usr/bin/env python3
"""Guard DEC-050 sparse suitability observation boundaries."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
RECORD = ROOT / "validation" / "benchmarks" / "sparse_suitability_observation.dec050.json"
SOURCE = ROOT / "core" / "solver" / "performance_harness" / "src" / "lib.rs"


def test_sparse_suitability_observation_record_preserves_dense_default_boundary():
    record = json.loads(RECORD.read_text(encoding="utf-8"))
    source = SOURCE.read_text(encoding="utf-8")

    assert record["record_id"] == "DEC-050-SPARSE-SUITABILITY-OBSERVATION-v1"
    assert record["status"] == "observation_only"
    assert record["threshold_policy_status"] == "tbd"
    assert record["default_sparse_promotion_status"] == "not_promoted_dense_default"
    assert record["timing_memory_threshold_status"] == "tbd"
    assert record["conditioning_ci_threshold_status"] == "tbd"
    assert record["hardware_normalization_status"] == "tbd"
    assert record["dense_path_role"] == "default_solver_and_parity_oracle"
    assert record["evidence_fixture_ids"] == [
        "invented-grid-frame-4x3",
        "invented-grid-frame-6x8",
    ]
    assert "SPARSE_SUITABILITY_GRID_BANDS: [(usize, usize); 2] = [(4, 3), (6, 8)]" in source
    assert "run_sparse_suitability_observation_suite" in source
    assert "no default sparse promotion is made" in " ".join(record["boundary"])
