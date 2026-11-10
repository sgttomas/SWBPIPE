from __future__ import annotations

import importlib.util
import sys
from pathlib import Path

import pytest


ROOT = Path(__file__).resolve().parents[1]
MODULE_PATH = ROOT / "tools" / "release" / "check_release_readiness.py"


@pytest.fixture
def project(tmp_path):
    release = load_module()
    for path in release.REQUIRED_PATHS:
        (tmp_path / path).parent.mkdir(parents=True, exist_ok=True)
        (tmp_path / path).write_text("fixture")
    return tmp_path


def load_module():
    spec = importlib.util.spec_from_file_location("check_release_readiness", MODULE_PATH)
    assert spec is not None
    assert spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def test_required_release_paths(project):
    release = load_module()
    assert release.check_required_paths(project) == []
    assert release.check_required_paths(ROOT) == []
    missing = release.REQUIRED_PATHS[0]
    (project / missing).unlink()
    assert release.check_required_paths(project) == [missing]
    assert release.main(["--profile", "skeleton", "--repo-root", str(project)]) == 1


def test_cargo_manifest_discovery_is_crate_local():
    release = load_module()
    manifests = release.discover_cargo_manifests(ROOT)

    assert Path("core/runner/headless/Cargo.toml") in manifests
    assert Path("core/reporting/protected_content_linter/Cargo.toml") in manifests
    assert all("target" not in manifest.parts for manifest in manifests)


def test_skeleton_plan_uses_local_commands_only(project):
    release = load_module()
    steps = release.build_plan("skeleton", project)
    commands = [" ".join(step.command) for step in steps]

    assert [step.name for step in steps] == ["release readiness script tests"]
    assert any("test_release_readiness_script.py" in command for command in commands)
    assert all(isinstance(step.command, tuple) for step in steps)


def test_python_profiles_use_coordination_maintenance_test(project):
    release = load_module()
    expected = (
        "-m",
        "pytest",
        "-q",
        "tests/test_coordination_maintenance.py",
    )
    old_target = "tools/coordination"

    for profile in ("python", "all"):
        commands = [step.command for step in release.build_plan(profile, project)]

        assert any(command[1:] == expected for command in commands)
        assert all(old_target not in part for command in commands for part in command)


def test_all_profile_preserves_current_command_surface(project, monkeypatch):
    release = load_module()
    # Cargo discovery is the real tree's (test_cargo_manifest_discovery_is_crate_local).
    discover = release.discover_cargo_manifests
    monkeypatch.setattr(release, "discover_cargo_manifests", lambda root: discover(ROOT))
    commands = [" ".join(step.command) for step in release.build_plan("all", project)]

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


def test_main_dry_run_prints_plan_without_executing(project, monkeypatch, capsys):
    release = load_module()

    def fail_run_steps(*args, **kwargs):
        raise AssertionError("dry-run must not execute planned checks")

    monkeypatch.setattr(release, "run_steps", fail_run_steps)

    result = release.main(["--profile", "skeleton", "--repo-root", str(project)])
    captured = capsys.readouterr()

    assert result == 0
    assert "SWBPIPE release readiness profile (dry-run)" in captured.out
    assert "planned checks: 1" in captured.out
    assert "running:" not in captured.out


def test_main_execute_runs_planned_steps(project, monkeypatch, capsys):
    release = load_module()
    executed: list[str] = []

    def fake_run_steps(steps, root):
        executed.extend(step.name for step in steps)
        assert root == project
        return 0

    monkeypatch.setattr(release, "run_steps", fake_run_steps)

    result = release.main(
        ["--profile", "skeleton", "--execute", "--repo-root", str(project)]
    )
    captured = capsys.readouterr()

    assert result == 0
    assert "SWBPIPE release readiness profile (execute)" in captured.out
    assert executed == ["release readiness script tests"]


def _probe_public_export(project: Path, name: str) -> bool:
    (project / "tests").mkdir(exist_ok=True)
    (project / "tests" / "public_export.py").write_text((ROOT / "tests" / "public_export.py").read_text())
    spec = importlib.util.spec_from_file_location(name, project / "tests" / "public_export.py")
    probe = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(probe)
    return probe.PUBLIC_EXPORT


def test_sparse_monorepo_clone_without_private_directories_is_not_the_export(tmp_path):
    # A sparse clone may drop loop/, execution/ and even the root AGENTS.md;
    # it keeps the repository's .git (a directory, or a file in a worktree).
    for index, marker in enumerate(("AGENTS.md", ".git", ".git-file")):
        repository = tmp_path / str(index)
        project = repository / "projects" / "chirality-piping"
        project.mkdir(parents=True)
        if marker == ".git":
            (repository / ".git").mkdir()
        else:
            (repository / (".git" if marker == ".git-file" else marker)).write_text("fixture")
        assert _probe_public_export(project, f"public_export_sparse_probe_{index}") is False
    # A standalone tree (the export, a repository of its own) is the export.
    standalone = tmp_path / "standalone"
    (standalone / ".git").mkdir(parents=True)
    assert _probe_public_export(standalone, "public_export_standalone_probe") is True
    # So is the export checked out inside another repository's projects/ folder.
    superproject = tmp_path / "superproject"
    nested = superproject / "projects" / "swbpipe"
    nested.mkdir(parents=True)
    (superproject / ".git").mkdir()
    (superproject / "AGENTS.md").write_text("fixture")
    (nested / ".git").write_text("gitdir: ../../.git/modules/swbpipe\n")
    assert _probe_public_export(nested, "public_export_nested_probe") is True
