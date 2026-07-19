from __future__ import annotations

import importlib.util
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
MODULE_PATH = ROOT / "tools" / "release" / "check_release_readiness.py"


def load_module():
    spec = importlib.util.spec_from_file_location("check_release_readiness", MODULE_PATH)
    assert spec is not None
    assert spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def test_required_release_paths_exist():
    release = load_module()
    assert release.check_required_paths(ROOT) == []


def test_latest_dag_dependency_edges_uses_approved_graph_pointer():
    release = load_module()
    assert (
        release.latest_dag_dependency_edges(ROOT)
        == Path("execution/_DAG/DAG-007/DependencyEdges.csv")
    )


def test_cargo_manifest_discovery_is_crate_local():
    release = load_module()
    manifests = release.discover_cargo_manifests(ROOT)

    assert Path("core/runner/headless/Cargo.toml") in manifests
    assert Path("core/reporting/protected_content_linter/Cargo.toml") in manifests
    assert all("target" not in manifest.parts for manifest in manifests)


def test_skeleton_plan_uses_local_commands_only():
    release = load_module()
    steps = release.build_plan("skeleton", ROOT)
    commands = [" ".join(step.command) for step in steps]

    assert any(
        "validate_dependencies_schema.py execution/_DAG/DAG-007/DependencyEdges.csv"
        in command
        for command in commands
    )
    assert any("test_release_readiness_script.py" in command for command in commands)
    assert all(isinstance(step.command, tuple) for step in steps)


def test_python_profiles_use_coordination_maintenance_test():
    release = load_module()
    expected = (
        "-m",
        "pytest",
        "-q",
        "tests/test_coordination_maintenance.py",
    )
    old_target = "tools/coordination"

    for profile in ("python", "all"):
        commands = [step.command for step in release.build_plan(profile, ROOT)]

        assert any(command[1:] == expected for command in commands)
        assert all(old_target not in part for command in commands for part in command)


def test_all_profile_preserves_current_command_surface():
    release = load_module()
    commands = [" ".join(step.command) for step in release.build_plan("all", ROOT)]

    assert any(
        "validate_dependencies_schema.py execution/_DAG/DAG-007/DependencyEdges.csv"
        in command
        for command in commands
    )
    assert any("test_release_readiness_script.py" in command for command in commands)
    assert any(" -m pytest -q tests" in command for command in commands)
    assert any("test_coordination_maintenance.py" in command for command in commands)
    assert any("tests/security" in command for command in commands)
    assert any(
        command
        == "cargo test --offline --manifest-path core/runner/headless/Cargo.toml"
        for command in commands
    )


def test_cargo_profile_forces_offline_argv():
    release = load_module()
    cargo_steps = release.build_plan("cargo", ROOT)

    assert cargo_steps
    assert all("--offline" in step.command for step in cargo_steps)


def test_run_steps_forces_cargo_offline_environment(monkeypatch):
    release = load_module()
    seen = []

    class Completed:
        returncode = 0

    def fake_run(command, cwd=None, env=None, check=False):
        seen.append((command, env))
        return Completed()

    monkeypatch.setattr(release.subprocess, "run", fake_run)
    result = release.run_steps(release.build_plan("cargo", ROOT)[:1], ROOT)

    assert result == 0
    assert seen[0][1]["CARGO_NET_OFFLINE"] == "true"


def test_main_dry_run_prints_plan_without_executing(monkeypatch, capsys):
    release = load_module()

    def fail_run_steps(*args, **kwargs):
        raise AssertionError("dry-run must not execute planned checks")

    monkeypatch.setattr(release, "run_steps", fail_run_steps)

    result = release.main(["--profile", "skeleton", "--repo-root", str(ROOT)])
    captured = capsys.readouterr()

    assert result == 0
    assert "OpenPipeStress release readiness profile (dry-run)" in captured.out
    assert "execution/_DAG/DAG-007/DependencyEdges.csv" in captured.out
    assert "planned checks: 2" in captured.out
    assert "running:" not in captured.out


def test_main_execute_runs_planned_steps(monkeypatch, capsys):
    release = load_module()
    executed: list[str] = []

    def fake_run_steps(steps, root):
        executed.extend(step.name for step in steps)
        assert root == ROOT
        return 0

    monkeypatch.setattr(release, "run_steps", fake_run_steps)

    result = release.main(
        ["--profile", "skeleton", "--execute", "--repo-root", str(ROOT)]
    )
    captured = capsys.readouterr()

    assert result == 0
    assert "OpenPipeStress release readiness profile (execute)" in captured.out
    assert executed == ["dag dependency schema", "release readiness script tests"]
