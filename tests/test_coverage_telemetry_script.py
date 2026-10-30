"""Focused tests for the DEC-060 opt-in coverage telemetry entrypoint."""

from __future__ import annotations

import importlib.util
import json
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
MODULE_PATH = ROOT / "tools" / "release" / "run_coverage_telemetry.py"

EXPECTED_LANE_ORDER = [
    "cargo_llvm_cov",
    "python_coverage",
    "desktop_vitest_coverage",
]


def load_module():
    spec = importlib.util.spec_from_file_location(
        "run_coverage_telemetry", MODULE_PATH
    )
    assert spec is not None
    assert spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def stub_runners(telemetry, overrides=None):
    """Lane runners that return schema-shaped entries without running tools."""

    def measured_rust(lane, root):
        entry = telemetry._lane_entry(lane)
        entry["tool_version"] = "cargo-llvm-cov 0.0.0"
        entry["status"] = telemetry.LANE_STATUS_MEASURED
        entry["metrics"] = {
            "crates": [
                {
                    "manifest": "core/example/Cargo.toml",
                    "measured": True,
                    "lines": {"count": 10, "covered": 5, "percent": 50.0},
                    "functions": {"count": 4, "covered": 2, "percent": 50.0},
                    "regions": {"count": 12, "covered": 6, "percent": 50.0},
                }
            ],
            "aggregation": "count_weighted_sum_across_crates",
            "aggregate": {
                "lines": {"count": 10, "covered": 5, "percent": 50.0},
                "functions": {"count": 4, "covered": 2, "percent": 50.0},
                "regions": {"count": 12, "covered": 6, "percent": 50.0},
            },
        }
        return entry

    def measured_python(lane, root):
        entry = telemetry._lane_entry(lane)
        entry["tool_version"] = "Coverage.py, version 0.0.0"
        entry["status"] = telemetry.LANE_STATUS_MEASURED
        entry["metrics"] = {
            "statements": {"count": 100, "covered": 90, "percent": 90.0}
        }
        return entry

    def not_measured_desktop(lane, root):
        entry = telemetry._lane_entry(lane)
        return telemetry._not_measured(
            entry, "tool_missing: @vitest/coverage-v8 is not installed"
        )

    runners = {
        "cargo_llvm_cov": measured_rust,
        "python_coverage": measured_python,
        "desktop_vitest_coverage": not_measured_desktop,
    }
    runners.update(overrides or {})
    return runners


def test_plan_is_the_three_dec060_lanes_in_sweep_order():
    telemetry = load_module()
    lanes = telemetry.build_lane_plan()

    assert [lane.lane_id for lane in lanes] == EXPECTED_LANE_ORDER
    assert [lane.tool for lane in lanes] == [
        "cargo-llvm-cov",
        "coverage.py",
        "@vitest/coverage-v8",
    ]


def test_telemetry_never_blocks_and_records_not_measured_lanes():
    telemetry = load_module()
    lanes = telemetry.build_lane_plan()

    summary = telemetry.run_telemetry(
        lanes, ROOT, lane_runners=stub_runners(telemetry)
    )

    assert summary["artifact"] == "openpipestress.coverage_telemetry"
    assert summary["schema_version"] == 1
    assert summary["decision_basis"] == "DEC-060"
    assert summary["git"]["commit_hash"]
    assert summary["lanes_measured"] == 2
    assert summary["lanes_not_measured"] == 1
    desktop = summary["lanes"][2]
    assert desktop["status"] == "not_measured"
    assert desktop["reason"].startswith("tool_missing")
    # No overall pass/fail exists: telemetry is recorded, never blocking.
    assert "overall_status" not in summary


def test_lanes_are_independent_not_fail_fast():
    telemetry = load_module()
    lanes = telemetry.build_lane_plan()

    def failing_rust(lane, root):
        entry = telemetry._lane_entry(lane)
        return telemetry._not_measured(
            entry, "run_failed: cargo llvm-cov failed for core/example/Cargo.toml"
        )

    summary = telemetry.run_telemetry(
        lanes,
        ROOT,
        lane_runners=stub_runners(telemetry, {"cargo_llvm_cov": failing_rust}),
    )

    statuses = [entry["status"] for entry in summary["lanes"]]
    # The failed first lane does not stop the later lanes.
    assert statuses == ["not_measured", "measured", "not_measured"]


def test_summary_validates_against_its_own_schema():
    telemetry = load_module()
    lanes = telemetry.build_lane_plan()

    summary = telemetry.run_telemetry(
        lanes, ROOT, lane_runners=stub_runners(telemetry)
    )

    assert telemetry.validate_summary(summary) == []


def test_schema_rejects_a_measured_lane_without_metrics():
    telemetry = load_module()
    lanes = telemetry.build_lane_plan()
    summary = telemetry.run_telemetry(
        lanes, ROOT, lane_runners=stub_runners(telemetry)
    )
    summary["lanes"][1]["metrics"] = None

    errors = telemetry.validate_summary(summary)

    assert errors, "a measured lane with null metrics must fail validation"


def test_summary_filename_binds_commit_and_flags_dirty_tree():
    telemetry = load_module()
    summary = {
        "git": {
            "commit_hash": "a" * 40,
            "status_capture_failed": False,
            "working_tree_dirty": True,
        },
        "started_utc": "2026-07-10T00:00:00+00:00",
    }

    assert (
        telemetry.summary_filename(summary)
        == f"COVERAGE_20260710T000000Z_{'a' * 12}-dirty.json"
    )


def test_lane_selection_accepts_known_ids_and_rejects_unknown():
    telemetry = load_module()

    selected = telemetry.select_lanes("python_coverage")
    assert selected is not None
    assert [lane.lane_id for lane in selected] == ["python_coverage"]

    assert telemetry.select_lanes("python_coverage,nope") is None
    assert telemetry.select_lanes("") is None


def test_rust_lane_records_not_measured_when_tool_is_absent(monkeypatch):
    telemetry = load_module()
    lane = telemetry.build_lane_plan()[0]

    monkeypatch.setattr(telemetry.shutil, "which", lambda name: "/usr/bin/cargo")
    monkeypatch.setattr(telemetry, "_tool_version", lambda argv, root: None)

    entry = telemetry.run_rust_lane(lane, ROOT)

    assert entry["status"] == "not_measured"
    assert entry["reason"].startswith("tool_missing")
    assert entry["commands"] == []
    assert entry["metrics"] is None


def test_rust_lane_reverts_to_not_measured_when_a_crate_fails(monkeypatch):
    telemetry = load_module()
    lane = telemetry.build_lane_plan()[0]

    monkeypatch.setattr(telemetry.shutil, "which", lambda name: "/usr/bin/cargo")
    monkeypatch.setattr(
        telemetry, "_tool_version", lambda argv, root: "cargo-llvm-cov 0.0.0"
    )

    crate_totals = json.dumps(
        {
            "data": [
                {
                    "totals": {
                        "lines": {"count": 10, "covered": 5},
                        "functions": {"count": 4, "covered": 2},
                        "regions": {"count": 12, "covered": 6},
                    }
                }
            ]
        }
    )
    calls = {"n": 0}

    def fake_captured(argv, cwd, env=None):
        calls["n"] += 1
        if calls["n"] == 2:
            return 101, ""
        return 0, crate_totals

    monkeypatch.setattr(telemetry, "_run_captured", fake_captured)

    entry = telemetry.run_rust_lane(lane, ROOT)

    assert entry["status"] == "not_measured"
    assert entry["reason"].startswith("run_failed")
    # Diagnostic per-crate records are retained for the follow-up.
    crates = entry["metrics"]["crates"]
    assert crates[0]["measured"] is True
    assert crates[1]["measured"] is False


def test_dec025_sweep_plan_is_unmodified_by_telemetry():
    """DEC-060 seam 2: the five-surface gate is untouched."""
    telemetry = load_module()
    sweep = telemetry._sweep_module()

    surfaces = [surface.surface_id for surface in sweep.build_sweep_plan()]
    assert surfaces == [
        "cargo_crate_sweep",
        "python_pytest",
        "desktop_vitest",
        "desktop_playwright_e2e",
        "desktop_production_build",
    ]
    assert "coverage" not in " ".join(surfaces)
