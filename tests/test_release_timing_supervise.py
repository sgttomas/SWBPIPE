#!/usr/bin/env python3
"""Cross-check of the two release timing supervisors (DEL-04-05, RT-U4, exit criterion 9).

The Python runner (core/solver/performance_harness/runner/release_runner.py, over k6_runner's
launch) and the driver's own supervise mode (`release_timing --supervise`, core/solver/
release_timing/src/supervise.rs) run the same plan, written by the runner's `plan_document`, on
the same invented models with the same driver binary. Five cases: the invented demo model solved
(ok); the same with a 4 MiB footprint budget, so that each supervisor's footprint watchdog kills
its driver (exceeds_base_machine_budget, with the stage reached); a 0 s timeout (timed_out); the
demo without supports, which the product refuses (refused, SUPPORT_INPUT_MISSING); and the
demo's model checks, a model-checks run of their own (ok). The two apply the same rules; the
driver also has the reading at exit. Where the Python runner's /usr/bin/time is killed with the
driver's group, the self-supervisor still reads the exited driver's lifetime-maximum footprint
and counts it, so the two do not class every case alike: an emergency RSS-cap kill whose sampled
footprint is within the budget and whose reading at exit is above it is exceeds_base_machine_budget
in the driver and killed_by_rss_cap in the Python. That boundary is tested on the driver's side
(core/solver/release_timing/tests/supervise.rs), and each record names the readings its
classification used (classification_readings). The cases here are not on that boundary, and
each requires the same classification, the same driver output (rows and output sha256) where
there is one, and the same record, packet and metadata keys, apart from the named siblings the
self-supervisor adds where the Python's source (/usr/bin/time, ps) is absent.

The binary is OPENPIPESTRESS_RELEASE_TIMING_BIN when set, else a debug build of the crate (Cargo,
unless OPENPIPESTRESS_NO_CARGO_BUILD=1, which takes an existing build or skips). macOS only: the
self-supervisor reads proc_pid_rusage, which only macOS has. Behaviour only: no time or memory
bound is asserted, and the runs here are not timings.
"""
from __future__ import annotations

import copy
import json
import os
import platform
import re
import subprocess
import sys
from pathlib import Path

import pytest

PROJECT = Path(__file__).resolve().parents[1]
CRATE = PROJECT / "core" / "solver" / "release_timing"
RUNNER = PROJECT / "core" / "solver" / "performance_harness" / "runner"
DEMO = PROJECT / "fixtures" / "product_preview" / "invented_demo_model.json"
ENV_BINARY = "OPENPIPESTRESS_RELEASE_TIMING_BIN"
sys.path.insert(0, str(RUNNER))

import release_runner as rr  # noqa: E402

pytestmark = pytest.mark.skipif(
    platform.system() != "Darwin",
    reason="macOS only: the self-supervisor reads proc_pid_rusage, which only macOS has",
)

# Keys the self-supervisor adds beside the Python's, each where a Python source is absent.
LAUNCH_SIBLINGS = {"watchdog_last_resident_bytes", "schema"}
RSS_SIBLINGS = {"resident_max_bytes", "resident_samples", "exit_lifetime_max_phys_footprint_bytes"}
PEAK_MEMORY_SIBLINGS = {"wait4_maxrss", "sampled_resident_size", "exit_lifetime_max_phys_footprint"}
METADATA_SIBLINGS = {"source_identity_from"}
HOST_KEYS = {True: {"quiet", "waited_s", "memorystatus_level", "load1_at_start"}, False: {"quiet", "reason"}}

# (case, task, profile, nodes, timeout_s, expect, footprint budget, classification)
CASES = (
    ("ok", "solve", "linear-ex", 100, 60, "solved", rr.BASE_MACHINE_BUDGET_BYTES, "ok"),
    ("kill", "solve", "linear-ex", 100, 60, "exceeds_base_machine_budget", 4 << 20, "exceeds_base_machine_budget"),
    ("timeout", "solve", "linear-ex", 100, 0, "timed_out", rr.BASE_MACHINE_BUDGET_BYTES, "timed_out"),
    ("refused", "solve", "linear-ex", 200, 60, "refused:SUPPORT_INPUT_MISSING", rr.BASE_MACHINE_BUDGET_BYTES, "refused"),
    ("checks", "model_checks", "linear-ex", 100, 60, "ok", rr.BASE_MACHINE_BUDGET_BYTES, "ok"),
)


def target_debug() -> Path:
    configured = os.environ.get("CARGO_TARGET_DIR")
    if not configured:
        return CRATE / "target" / "debug"
    path = Path(configured)
    return (path if path.is_absolute() else CRATE / path) / "debug"


@pytest.fixture(scope="module")
def binary(no_cargo_build, prebuilt_binary) -> Path:
    if os.environ.get(ENV_BINARY) or no_cargo_build:
        return prebuilt_binary(ENV_BINARY, target_debug() / "release_timing", "the release timing driver")
    build = subprocess.run(["cargo", "build", "--quiet", "--locked", "--bin", "release_timing"], cwd=CRATE,
                           capture_output=True, text=True, check=False)
    assert build.returncode == 0, build.stderr
    built = target_debug() / "release_timing"
    assert built.is_file()
    return built


def models(folder: Path) -> Path:
    """The demo as the 100-node model, the demo without supports as the 200-node one, and a
    manifest listing both, so that both supervisors check each model's sha256."""
    path = folder / "models"
    path.mkdir()
    demo = json.loads(DEMO.read_text())
    (path / rr.model_name("linear-ex", 100)).write_text(DEMO.read_text())
    unsupported = copy.deepcopy(demo)
    unsupported["supports"] = []
    (path / rr.model_name("linear-ex", 200)).write_text(json.dumps(unsupported))
    manifest = {"schema": rr.MANIFEST_SCHEMA, "models": {
        name: {"profile": "linear-ex", "nodes": nodes, "sha256": rr.sha256_file(str(path / name)),
               "bytes": (path / name).stat().st_size, "stats": {"invented": True}}
        for name, nodes in ((rr.model_name("linear-ex", 100), 100), (rr.model_name("linear-ex", 200), 200))}}
    (path / "manifest.json").write_text(json.dumps(manifest))
    return path


@pytest.fixture(scope="module")
def outcomes(binary, tmp_path_factory):
    """Each case run by both supervisors: {case: (python out dir, rust out dir)}."""
    folder = tmp_path_factory.mktemp("release-timing-supervise")
    model_dir = models(folder)
    found = {}
    for case, task, profile, nodes, timeout, expect, budget, _ in CASES:
        plan = rr.build_plan(((profile, nodes, timeout, 1, expect, False, task),), footprint_budget_bytes=budget)
        python_out, rust_out = folder / ("python-" + case), folder / ("rust-" + case)
        rr.run_plan(plan, str(model_dir), str(python_out), str(binary), quiet_wait_s=0, log=lambda *a: None)
        rr.write_packet(str(python_out))
        plan_path = folder / ("plan-" + case + ".json")
        rr.write_plan(str(plan_path), plan, quiet_wait_s=0, models_dir=str(model_dir))
        done = subprocess.run([str(binary), "--supervise", str(plan_path), "--out", str(rust_out), "--models",
                               str(model_dir)], capture_output=True, text=True, check=False)
        assert done.returncode == 0, done.stdout + done.stderr
        found[case] = (python_out, rust_out)
    return found


def record(out: Path) -> dict:
    records = [json.loads(line) for line in (out / "records.jsonl").read_text().splitlines() if line.strip()]
    assert len(records) == 1
    return records[0]


def packet(out: Path) -> dict:
    return json.loads((out / "packet.json").read_text())


@pytest.mark.parametrize("case", [c[0] for c in CASES])
def test_same_classification_and_driver_output(outcomes, case):
    expected = next(c[-1] for c in CASES if c[0] == case)
    py, rs = (record(out) for out in outcomes[case])
    assert (py["classification"], rs["classification"]) == (expected, expected), (py, rs)
    assert py["as_expected"] is True and rs["as_expected"] is True
    if case == "kill":
        # The footprint watchdog killed each driver above the 4 MiB budget, after a stage.
        for r in (py, rs):
            assert re.fullmatch(r"footprint_\d+_above_budget_4194304_killed", r["classification_detail"]), r
            assert r["launch"]["killed_by_sampler"] is True and r["launch"]["survivors"] == []
            assert r["last_stage"] and r["last_stage"]["stage"], r
            assert r["stages_finished"] >= 1
    else:
        assert py["classification_detail"] == rs["classification_detail"]
    if case == "timeout":
        assert py["launch"]["timed_out"] is True and rs["launch"]["timed_out"] is True
        assert py["launch"]["survivors"] == [] and rs["launch"]["survivors"] == []
    if case in ("ok", "refused"):
        for key in ("status", "rows", "output_bytes", "output_sha256", "blocking_codes", "row_kinds", "diagnostic_counts"):
            assert py["driver"][key] == rs["driver"][key], key
    elif case == "checks":
        for key in ("invocation", "status", "output_bytes", "output_sha256", "model_checks", "stages"):
            assert py["driver"][key] == rs["driver"][key], key
        assert py["driver"]["invocation"] == "model_checks" and py["task"] == rs["task"] == "model_checks"
        assert py["launch"]["argv"][1:] == rs["launch"]["argv"][1:] == ["linear-ex-n0100.json", "--model-checks"]
    else:
        assert py["driver"] is None and rs["driver"] is None
    assert (py["model_sha256"], py["model_stats"]) == (rs["model_sha256"], rs["model_stats"])


@pytest.mark.parametrize("case", [c[0] for c in CASES])
def test_each_record_names_the_readings_its_classification_used(outcomes, case):
    py, rs = (record(out) for out in outcomes[case])
    for r in (py, rs):
        assert set(r["classification_readings"]) == {"used", "at_exit", "at_exit_absent"}
    # The driver reads its child at exit in every case; the Python runner's reading at exit is
    # /usr/bin/time's, which a killed run does not have.
    assert rs["classification_readings"]["at_exit"] == "rss:exit_lifetime_max_phys_footprint_bytes"
    if case in ("kill", "timeout"):
        assert py["classification_readings"]["at_exit"] is None
        assert "killed with its process group" in py["classification_readings"]["at_exit_absent"]
    else:
        assert py["classification_readings"]["at_exit"] == "rss:time_peak_footprint_bytes"
    if case == "timeout":
        assert py["classification_readings"]["used"] == rs["classification_readings"]["used"] == []
    if case == "kill":
        assert py["classification_readings"]["used"] == rs["classification_readings"]["used"] == ["sampler:kill_footprint_bytes"]


@pytest.mark.parametrize("case", [c[0] for c in CASES])
def test_same_record_keys_apart_from_named_siblings(outcomes, case):
    py, rs = (record(out) for out in outcomes[case])
    assert set(rs) == set(py)
    assert set(rs["launch"]) == set(py["launch"]) | LAUNCH_SIBLINGS
    # /usr/bin/time is in the killed process group, so a killed Python run has no time_* keys; the
    # self-supervisor keeps them, null, in every run.
    time_keys = {"time_max_rss_bytes", "time_peak_footprint_bytes"}
    assert set(py["launch"]["rss"]) & time_keys == (set() if case in ("kill", "timeout") else time_keys)
    assert set(rs["launch"]["rss"]) == set(py["launch"]["rss"]) | RSS_SIBLINGS | time_keys
    assert set(rs["footprint_watchdog"]) == set(py["footprint_watchdog"])
    for r in (py, rs):
        assert set(r["host"]) == HOST_KEYS[r["host"]["quiet"]]
    # The Python's /usr/bin/time and ps readings have no source in the self-supervisor.
    for key in ("time_max_rss_bytes", "time_peak_footprint_bytes", "ps_max_kib"):
        assert rs["launch"]["rss"][key] is None
    assert rs["launch"]["peak_rss_bytes"] == rs["launch"]["rss"]["wait4_maxrss_bytes"]


@pytest.mark.parametrize("case", [c[0] for c in CASES])
def test_same_packet_and_metadata_keys(outcomes, case):
    py, rs = (packet(out) for out in outcomes[case])
    assert py["schema"] == rs["schema"] == rr.PACKET_SCHEMA
    assert set(rs) == set(py)
    assert py["classification_counts"] == rs["classification_counts"]
    assert len(rs["conditions"]) == len(rr.CONDITIONS) + 1 and len(py["conditions"]) == len(rr.CONDITIONS) + 1
    assert set(rs["metadata"]) == set(py["metadata"]) | METADATA_SIBLINGS
    for key in ("os", "os_release", "machine", "os_product_version", "os_build_version", "cpu_model", "cpu_physical",
                "cpu_logical", "memory_bytes", "binary_sha256", "binary_name", "base_machine_budget_bytes",
                "emergency_rss_cap_bytes", "footprint_watchdog", "driver_schema", "watchdog_poll_s", "budget_s",
                "quiet_wait_s", "write_results", "plan"):
        assert py["metadata"][key] == rs["metadata"][key], key
    (py_row,), (rs_row,) = py["runs"], rs["runs"]
    assert set(rs_row) == set(py_row)
    assert set(rs_row["peak_memory_bytes"]) == set(py_row["peak_memory_bytes"]) | PEAK_MEMORY_SIBLINGS
    for key in ("run_id", "task", "classification", "as_expected", "model_sha256", "rows", "output_sha256", "counts",
                "status", "blocking_codes", "bases"):
        assert py_row[key] == rs_row[key], key


def test_each_supervisor_keeps_the_same_per_run_files(outcomes):
    for case, (py, rs) in outcomes.items():
        run_id = record(py)["run_id"]
        for suffix in (".jsonl", ".stderr.txt", ".record.json"):
            assert (py / (run_id + suffix)).is_file() and (rs / (run_id + suffix)).is_file(), (case, suffix)
        for out in (py, rs):
            assert str(out.parent) not in (out / "records.jsonl").read_text(), "no machine path in a record"
