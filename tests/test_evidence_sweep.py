"""Focused tests for the DEC-025 five-surface evidence sweep entrypoint."""

from __future__ import annotations

import importlib.util
import json
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
MODULE_PATH = ROOT / "tools" / "release" / "run_evidence_sweep.py"

EXPECTED_SURFACE_ORDER = [
    "cargo_crate_sweep",
    "python_pytest",
    "desktop_vitest",
    "desktop_playwright_e2e",
    "desktop_production_build",
]


def load_module():
    spec = importlib.util.spec_from_file_location("run_evidence_sweep", MODULE_PATH)
    assert spec is not None
    assert spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def test_plan_is_the_five_surfaces_in_dec025_order():
    sweep = load_module()
    surfaces = sweep.build_sweep_plan()

    assert [surface.surface_id for surface in surfaces] == EXPECTED_SURFACE_ORDER


def test_wasm_engine_build_precedes_vitest():
    sweep = load_module()
    vitest = next(
        surface
        for surface in sweep.build_sweep_plan()
        if surface.surface_id == "desktop_vitest"
    )
    commands = [" ".join(command) for command in vitest.commands]

    assert commands == ["npm run build:wasm:desktop", "npm run test:desktop"]


def test_plan_uses_local_commands_only():
    sweep = load_module()
    for surface in sweep.build_sweep_plan():
        for command in surface.commands:
            assert isinstance(command, tuple)
            assert command[0] in (sys.executable, "npm")


def test_cargo_surface_reuses_release_readiness_cargo_profile():
    sweep = load_module()
    cargo = sweep.build_sweep_plan()[0]

    assert cargo.commands == (
        (
            sys.executable,
            "tools/release/check_release_readiness.py",
            "--profile",
            "cargo",
            "--execute",
        ),
    )


def test_run_command_forces_cargo_offline_environment(monkeypatch):
    sweep = load_module()
    observed = {}

    class Completed:
        returncode = 0

    def fake_run(command, cwd=None, env=None, check=False):
        observed.update(env)
        return Completed()

    monkeypatch.setattr(sweep.subprocess, "run", fake_run)

    assert sweep.run_command(("npm", "run", "test:desktop"), ROOT) == 0
    assert observed["CARGO_NET_OFFLINE"] == "true"


def test_preflight_rejects_missing_node_dependencies_before_execution(
    monkeypatch, tmp_path
):
    sweep = load_module()
    monkeypatch.setattr(sweep.shutil, "which", lambda executable: f"/bin/{executable}")

    errors = sweep.preflight_prerequisites(tmp_path)

    assert any("node_modules/.bin/vitest" in error for error in errors)
    assert any("node_modules/.bin/playwright" in error for error in errors)


def test_build_wasm_script_forces_offline_cargo():
    script = (ROOT / "apps" / "desktop" / "scripts" / "build-wasm-engine.mjs").read_text(
        encoding="utf-8"
    )

    assert 'CARGO_NET_OFFLINE: "true"' in script
    assert '"build",\n  "--offline",' in script


def test_summary_binds_commit_hash_and_passes_when_all_surfaces_pass():
    sweep = load_module()

    summary = sweep.run_sweep(sweep.build_sweep_plan(), ROOT, runner=lambda c, r: 0)

    assert summary["artifact"] == "openpipestress.evidence_sweep_summary"
    assert summary["schema_version"] == 2
    assert summary["decision_basis"] == "DEC-025"
    git_state = summary["git"]
    assert git_state["commit_hash"] and len(git_state["commit_hash"]) == 40
    assert git_state["status_capture_failed"] is False
    assert isinstance(git_state["working_tree_dirty"], bool)
    assert summary["overall_status"] == "pass"
    assert [entry["surface_id"] for entry in summary["surfaces"]] == (
        EXPECTED_SURFACE_ORDER
    )
    assert all(entry["status"] == "pass" for entry in summary["surfaces"])
    assert all(
        command["exit_code"] == 0
        for entry in summary["surfaces"]
        for command in entry["commands"]
    )


def test_parse_porcelain_status_keeps_full_path_for_unstaged_first_record():
    sweep = load_module()
    porcelain = " M projects/chirality-piping/init/init-prompt.md\0"

    assert sweep.parse_porcelain_status(porcelain) == [
        "projects/chirality-piping/init/init-prompt.md"
    ]


def test_parse_porcelain_status_handles_rename_untracked_and_staged_records():
    sweep = load_module()
    porcelain = (
        "R  docs/renamed.md\0docs/original.md\0"
        " M tools/release/run_evidence_sweep.py\0"
        "?? validation/evidence/sweeps/SWEEP_new.json\0"
        "A  tests/test_new.py\0"
    )

    assert sweep.parse_porcelain_status(porcelain) == [
        "docs/original.md",
        "docs/renamed.md",
        "tests/test_new.py",
        "tools/release/run_evidence_sweep.py",
        "validation/evidence/sweeps/SWEEP_new.json",
    ]


def test_collect_git_state_does_not_strip_leading_space_from_dirty_paths(
    monkeypatch,
):
    """Regression: a stripped ` M <path>` capture cut the first path char."""
    sweep = load_module()

    class FakeCompleted:
        def __init__(self, stdout):
            self.returncode = 0
            self.stdout = stdout

    def fake_run(command, cwd=None, capture_output=False, text=False, check=False):
        if "status" in command:
            assert "-z" in command
            return FakeCompleted(
                " M projects/chirality-piping/init/init-prompt.md\0"
            )
        if "--abbrev-ref" in command:
            return FakeCompleted("main\n")
        return FakeCompleted("a" * 40 + "\n")

    monkeypatch.setattr(sweep.subprocess, "run", fake_run)

    state = sweep.collect_git_state(ROOT)

    assert state["commit_hash"] == "a" * 40
    assert state["branch"] == "main"
    assert state["working_tree_dirty"] is True
    assert state["dirty_paths"] == [
        "projects/chirality-piping/init/init-prompt.md"
    ]


def test_sweep_fails_fast_and_marks_later_surfaces_not_run():
    sweep = load_module()

    def failing_pytest(command, root):
        return 1 if "pytest" in command else 0

    summary = sweep.run_sweep(
        sweep.build_sweep_plan(), ROOT, runner=failing_pytest
    )

    statuses = {
        entry["surface_id"]: entry["status"] for entry in summary["surfaces"]
    }
    assert summary["overall_status"] == "fail"
    assert statuses["cargo_crate_sweep"] == "pass"
    assert statuses["python_pytest"] == "fail"
    assert statuses["desktop_vitest"] == "not_run"
    assert statuses["desktop_playwright_e2e"] == "not_run"
    assert statuses["desktop_production_build"] == "not_run"
    not_run = [e for e in summary["surfaces"] if e["status"] == "not_run"]
    assert all(entry["commands"] == [] for entry in not_run)


def test_failing_command_stops_its_surface():
    sweep = load_module()

    def failing_wasm_build(command, root):
        return 1 if "build:wasm:desktop" in command else 0

    summary = sweep.run_sweep(
        sweep.build_sweep_plan(), ROOT, runner=failing_wasm_build
    )

    vitest = next(
        entry
        for entry in summary["surfaces"]
        if entry["surface_id"] == "desktop_vitest"
    )
    assert vitest["status"] == "fail"
    assert [command["argv"][-1] for command in vitest["commands"]] == [
        "build:wasm:desktop"
    ]


def test_summary_filename_binds_commit_and_dirty_state():
    sweep = load_module()
    summary = {
        "git": {"commit_hash": "a" * 40, "working_tree_dirty": True},
        "started_utc": "2026-06-11T22:30:05+00:00",
    }

    assert (
        sweep.summary_filename(summary)
        == "SWEEP_20260611T223005Z_aaaaaaaaaaaa-dirty.json"
    )


def test_collect_git_state_records_capture_failure_explicitly(monkeypatch):
    """Regression: a failed `git status` capture must never read as clean."""
    sweep = load_module()

    class FakeCompleted:
        def __init__(self, stdout, returncode=0):
            self.returncode = returncode
            self.stdout = stdout

    def fake_run(command, cwd=None, capture_output=False, text=False, check=False):
        if "status" in command:
            return FakeCompleted("", returncode=128)
        if "--abbrev-ref" in command:
            return FakeCompleted("main\n")
        return FakeCompleted("a" * 40 + "\n")

    monkeypatch.setattr(sweep.subprocess, "run", fake_run)

    state = sweep.collect_git_state(ROOT)

    assert state["status_capture_failed"] is True
    assert state["working_tree_dirty"] is None
    assert state["dirty_paths"] == []
    assert sweep.git_state_unverified(state) is True


def test_summary_filename_marks_unverified_git_state():
    sweep = load_module()
    summary = {
        "git": {
            "commit_hash": "a" * 40,
            "status_capture_failed": True,
            "working_tree_dirty": None,
        },
        "started_utc": "2026-06-11T22:30:05+00:00",
    }

    assert (
        sweep.summary_filename(summary)
        == "SWEEP_20260611T223005Z_aaaaaaaaaaaa-gitunverified.json"
    )


def test_write_summary_emits_valid_json(tmp_path):
    sweep = load_module()
    summary = sweep.run_sweep(sweep.build_sweep_plan(), ROOT, runner=lambda c, r: 0)

    output_path = sweep.write_summary(summary, tmp_path)

    parsed = json.loads(output_path.read_text(encoding="utf-8"))
    assert parsed["overall_status"] == "pass"
    assert parsed["git"]["commit_hash"] == summary["git"]["commit_hash"]


def test_main_dry_run_prints_plan_without_executing(monkeypatch, capsys):
    sweep = load_module()

    def fail_run_sweep(*args, **kwargs):
        raise AssertionError("dry-run must not execute the sweep")

    monkeypatch.setattr(sweep, "run_sweep", fail_run_sweep)

    result = sweep.main(["--repo-root", str(ROOT)])
    captured = capsys.readouterr()

    assert result == 0
    assert "evidence sweep (dry-run)" in captured.out
    assert "surfaces (sequential, F-4-safe order): 5" in captured.out


def test_main_execute_writes_summary_and_returns_failure_exit(
    monkeypatch, capsys, tmp_path
):
    sweep = load_module()

    monkeypatch.setattr(
        sweep, "run_command", lambda command, root: 1 if "pytest" in command else 0
    )
    monkeypatch.setattr(sweep, "preflight_prerequisites", lambda root: [])

    result = sweep.main(
        [
            "--execute",
            "--repo-root",
            str(ROOT),
            "--output-dir",
            str(tmp_path),
        ]
    )
    captured = capsys.readouterr()

    assert result == 1
    summaries = list(Path(tmp_path).glob("SWEEP_*.json"))
    assert len(summaries) == 1
    parsed = json.loads(summaries[0].read_text(encoding="utf-8"))
    assert parsed["overall_status"] == "fail"
    assert "overall: fail" in captured.out


def test_main_preflight_failure_runs_no_surface_and_writes_no_summary(
    monkeypatch, tmp_path
):
    sweep = load_module()

    monkeypatch.setattr(
        sweep,
        "preflight_prerequisites",
        lambda root: ["missing local prerequisite"],
    )

    def fail_run_sweep(*args, **kwargs):
        raise AssertionError("preflight failure must stop before surface 1")

    monkeypatch.setattr(sweep, "run_sweep", fail_run_sweep)

    result = sweep.main(
        [
            "--execute",
            "--repo-root",
            str(ROOT),
            "--output-dir",
            str(tmp_path),
        ]
    )

    assert result == 1
    assert list(tmp_path.glob("SWEEP_*.json")) == []
