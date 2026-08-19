"""Focused tests for the DEC-025 five-surface evidence sweep entrypoint."""

from __future__ import annotations

import importlib.util
import json
import os
import re
import shutil
import subprocess
import sys
from pathlib import Path

import pytest


ROOT = Path(__file__).resolve().parents[1]
MODULE_PATH = ROOT / "tools" / "release" / "run_evidence_sweep.py"

EXPECTED_SURFACE_ORDER = [
    "cargo_crate_sweep",
    "python_pytest",
    "desktop_vitest",
    "desktop_playwright_e2e",
    "desktop_production_build",
]

EXPECTED_EXACT_DEV_REQUIREMENTS = {
    "jsonschema": "4.26.0",
    "coverage": "7.15.3",
    "pyyaml": "6.0.3",
    "pytest-xdist": "3.8.0",
}


def load_module():
    spec = importlib.util.spec_from_file_location("run_evidence_sweep", MODULE_PATH)
    assert spec is not None
    assert spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def stub_sweep_summary(*, dirty: bool) -> dict:
    surfaces = []
    for order, surface_id in enumerate(EXPECTED_SURFACE_ORDER, start=1):
        surfaces.append(
            {
                "order": order,
                "surface_id": surface_id,
                "description": f"fixture {surface_id}",
                "execution_capability": (
                    "host" if surface_id == "desktop_playwright_e2e" else "sandboxed"
                ),
                "commands": [
                    {"argv": ["fixture"], "exit_code": 0, "duration_seconds": 0.0}
                ],
                "status": "pass",
            }
        )
    return {
        "artifact": "openpipestress.evidence_sweep_summary",
        "schema_version": 2,
        "decision_basis": "DEC-025",
        "git": {
            "commit_hash": "a" * 40,
            "branch": "test",
            "status_capture_failed": False,
            "working_tree_dirty": dirty,
            "dirty_paths": ["tracked-change.txt"] if dirty else [],
        },
        "runtime": {},
        "started_utc": "2026-08-16T12:00:00+00:00",
        "finished_utc": "2026-08-16T12:00:01+00:00",
        "duration_seconds": 1.0,
        "surfaces": surfaces,
        "overall_status": "pass",
    }


def valid_ci_binding(*, head_sha: str, run_attempt: int = 1) -> dict:
    return {
        "workflow_path": ".github/workflows/piping-desktop-e2e.yml",
        "run_id": 987654321,
        "run_attempt": run_attempt,
        "head_sha": head_sha,
        "conclusion": "success",
        "registered_e2e_specs_executed": True,
        "viewport_projects": ["chromium-desktop", "chromium-compact"],
    }


def test_plan_is_the_five_surfaces_in_dec025_order():
    sweep = load_module()
    surfaces = sweep.build_sweep_plan()

    assert [surface.surface_id for surface in surfaces] == EXPECTED_SURFACE_ORDER


def test_python_runtime_preflight_accepts_minimum_and_current_runtime():
    sweep = load_module()

    assert sweep.python_runtime_preflight_error((3, 11, 0)) is None
    assert sweep.python_runtime_preflight_error() is None
    assert sweep.current_python_version()[:2] >= sweep.MINIMUM_PYTHON_VERSION


def test_python_runtime_preflight_rejects_pre_311_before_any_probe_or_surface(
    monkeypatch, capsys, tmp_path
):
    sweep = load_module()

    monkeypatch.setattr(sweep, "current_python_version", lambda: (3, 10, 14))

    def fail_downstream(*args, **kwargs):
        raise AssertionError(
            "Python runtime failure must precede prerequisite and surface work"
        )

    monkeypatch.setattr(sweep, "missing_tools", fail_downstream)
    monkeypatch.setattr(sweep, "preflight_prerequisites", fail_downstream)
    monkeypatch.setattr(sweep, "run_sweep", fail_downstream)

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
    assert "failed before prerequisite probing or surface 1" in captured.err
    assert "current Python is 3.10.14" in captured.err
    assert "required Python is >=3.11" in captured.err
    assert "no prerequisite probe or evidence surface ran" in captured.err
    assert list(tmp_path.glob("SWEEP_*.json")) == []


def test_dev_requirements_are_exactly_pinned():
    requirement_lines = [
        line.strip()
        for line in (ROOT / "requirements-dev.txt").read_text(
            encoding="utf-8"
        ).splitlines()
        if line.strip() and not line.lstrip().startswith("#")
    ]
    parsed = {}

    for requirement in requirement_lines:
        assert re.fullmatch(
            r"[A-Za-z0-9_.-]+==[A-Za-z0-9_.+!-]+", requirement
        ), f"direct dev dependency is not exactly pinned: {requirement}"
        name, version = requirement.split("==", 1)
        parsed[name.lower()] = version

    assert parsed == EXPECTED_EXACT_DEV_REQUIREMENTS


def test_every_surface_declares_a_valid_execution_capability():
    sweep = load_module()
    surfaces = sweep.build_sweep_plan()

    assert all(
        surface.execution_capability in sweep.EXECUTION_CAPABILITIES
        for surface in surfaces
    )
    assert {
        surface.surface_id: surface.execution_capability for surface in surfaces
    } == {
        "cargo_crate_sweep": "sandboxed",
        "python_pytest": "sandboxed",
        "desktop_vitest": "sandboxed",
        "desktop_playwright_e2e": "host",
        "desktop_production_build": "sandboxed",
    }


def test_ci_binding_replaces_only_surface_4_and_records_dual_viewport_proof():
    sweep = load_module()
    binding = valid_ci_binding(head_sha="a" * 40)

    surfaces = sweep.build_sweep_plan(binding)
    surface4 = surfaces[3]

    assert [surface.surface_id for surface in surfaces] == EXPECTED_SURFACE_ORDER
    assert surface4.execution_capability == "ci"
    assert surface4.commands == ()
    assert surface4.ci_binding == binding
    assert [surface.execution_capability for surface in surfaces[:3] + surfaces[4:]] == [
        "sandboxed",
        "sandboxed",
        "sandboxed",
        "sandboxed",
    ]


def test_host_surface_4_commands_and_capability_remain_unchanged():
    sweep = load_module()
    surface4 = sweep.build_sweep_plan()[3]

    assert surface4.execution_capability == "host"
    assert surface4.commands == (
        ("npm", "run", "test:e2e:desktop"),
        ("npm", "run", "test:e2e:dist:desktop"),
    )
    assert surface4.ci_binding is None


def test_ci_binding_accepts_same_sha_rerun_attempt():
    sweep = load_module()
    binding = valid_ci_binding(head_sha="a" * 40, run_attempt=3)

    assert sweep.validate_ci_binding(binding, commit_hash="a" * 40) == []


def test_ci_binding_rejects_head_mismatch_failed_specs_and_incomplete_viewports():
    sweep = load_module()
    binding = valid_ci_binding(head_sha="a" * 40)
    binding["conclusion"] = "failure"
    binding["viewport_projects"] = ["chromium-desktop"]

    errors = sweep.validate_ci_binding(binding, commit_hash="b" * 40)

    assert any("does not equal sweep commit_hash" in error for error in errors)
    assert any("failed specs are not accepted" in error for error in errors)
    assert any("registered dual-viewport" in error for error in errors)


def test_surface_rejects_invalid_execution_capability():
    sweep = load_module()

    with pytest.raises(ValueError, match="invalid execution capability"):
        sweep.Surface(
            surface_id="invalid",
            description="invalid test surface",
            execution_capability="networked",
            commands=(),
        )


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


def test_sandboxed_preflight_does_not_require_playwright(monkeypatch, tmp_path):
    sweep = load_module()
    monkeypatch.setattr(sweep.shutil, "which", lambda executable: None)
    selected = sweep.select_surfaces(sweep.build_sweep_plan(), "sandboxed")

    errors = sweep.preflight_prerequisites(tmp_path, selected)

    assert all("playwright" not in error.lower() for error in errors)


def test_cargo_cache_probe_commands_distinguish_tracked_and_lockless_crates(
    tmp_path,
):
    sweep = load_module()
    root = tmp_path / "source"
    projected_root = tmp_path / "projected"
    manifests = [
        Path("core/lockless/Cargo.toml"),
        Path("core/tracked/Cargo.toml"),
    ]

    probes = sweep.build_cargo_cache_probes(
        manifests,
        root,
        projected_root,
        {Path("core/tracked/Cargo.lock")},
    )
    by_manifest = {probe.source_manifest: probe for probe in probes}

    tracked = by_manifest[Path("core/tracked/Cargo.toml")]
    assert tracked.cwd == root
    assert tracked.command == (
        "cargo",
        "fetch",
        "--locked",
        "--offline",
        "--manifest-path",
        "core/tracked/Cargo.toml",
    )

    lockless = by_manifest[Path("core/lockless/Cargo.toml")]
    assert lockless.cwd == projected_root
    assert lockless.command == (
        "cargo",
        "fetch",
        "--offline",
        "--manifest-path",
        "core/lockless/Cargo.toml",
    )


def test_ignored_existing_cargo_lock_remains_lockless_and_untouched(tmp_path):
    sweep = load_module()
    root = tmp_path / "source"
    tracked = root / "core" / "tracked"
    lockless = root / "core" / "lockless"
    tracked.mkdir(parents=True)
    lockless.mkdir(parents=True)
    (tracked / "Cargo.toml").write_text("[package]\nname='tracked'\n", encoding="utf-8")
    (tracked / "Cargo.lock").write_text("tracked lock\n", encoding="utf-8")
    (lockless / "Cargo.toml").write_text(
        "[package]\nname='lockless'\n", encoding="utf-8"
    )
    ignored_lock = lockless / "Cargo.lock"
    ignored_lock.write_text("ignored sentinel\n", encoding="utf-8")
    (root / ".gitignore").write_text(
        "core/lockless/Cargo.lock\n", encoding="utf-8"
    )
    subprocess.run(("git", "init", "-q"), cwd=root, check=True)
    subprocess.run(("git", "add", "."), cwd=root, check=True)

    tracked_locks = sweep.tracked_cargo_locks(root)
    projected_root = tmp_path / "projected"
    probes = sweep.build_cargo_cache_probes(
        sweep.discover_cargo_manifests(root),
        root,
        projected_root,
        tracked_locks,
    )
    by_manifest = {probe.source_manifest: probe for probe in probes}
    sweep.project_cargo_sources(root, projected_root)

    assert tracked_locks == {Path("core/tracked/Cargo.lock")}
    assert "--locked" in by_manifest[Path("core/tracked/Cargo.toml")].command
    assert "--locked" not in by_manifest[Path("core/lockless/Cargo.toml")].command
    assert ignored_lock.read_text(encoding="utf-8") == "ignored sentinel\n"
    assert not (projected_root / "core" / "lockless" / "Cargo.lock").exists()


@pytest.mark.skipif(shutil.which("cargo") is None, reason="cargo is required")
def test_real_cargo_preflight_keeps_lockless_git_checkout_clean(tmp_path):
    sweep = load_module()
    root = tmp_path / "mini-project"
    tracked = root / "core" / "tracked"
    lockless = root / "core" / "lockless"
    for crate, package in ((tracked, "tracked_crate"), (lockless, "lockless_crate")):
        (crate / "src").mkdir(parents=True)
        (crate / "Cargo.toml").write_text(
            "\n".join(
                (
                    "[package]",
                    f'name = "{package}"',
                    'version = "0.1.0"',
                    'edition = "2021"',
                    "",
                )
            ),
            encoding="utf-8",
        )
        (crate / "src" / "lib.rs").write_text(
            "pub fn ready() -> bool { true }\n", encoding="utf-8"
        )
    (root / ".gitignore").write_text(
        "core/lockless/Cargo.lock\n", encoding="utf-8"
    )

    subprocess.run(("git", "init", "-q"), cwd=root, check=True)
    subprocess.run(
        (
            "cargo",
            "generate-lockfile",
            "--offline",
            "--manifest-path",
            "core/tracked/Cargo.toml",
        ),
        cwd=root,
        check=True,
    )
    subprocess.run(("git", "add", "."), cwd=root, check=True)
    subprocess.run(
        (
            "git",
            "-c",
            "user.name=Evidence Sweep Test",
            "-c",
            "user.email=evidence-sweep@example.invalid",
            "commit",
            "-q",
            "-m",
            "fixture",
        ),
        cwd=root,
        check=True,
    )
    before = subprocess.run(
        ("git", "status", "--porcelain"),
        cwd=root,
        capture_output=True,
        text=True,
        check=True,
    ).stdout

    env = os.environ.copy()
    env["CARGO_NET_OFFLINE"] = "true"
    errors = sweep.cargo_cache_preflight_errors(root, env)

    after = subprocess.run(
        ("git", "status", "--porcelain"),
        cwd=root,
        capture_output=True,
        text=True,
        check=True,
    ).stdout
    assert errors == []
    assert before == after == ""
    assert (tracked / "Cargo.lock").is_file()
    assert not (lockless / "Cargo.lock").exists()


def test_build_wasm_script_forces_offline_cargo():
    script = (ROOT / "apps" / "desktop" / "scripts" / "build-wasm-engine.mjs").read_text(
        encoding="utf-8"
    )

    assert 'CARGO_NET_OFFLINE: "true"' in script
    assert '"build",\n  "--offline",' in script


def test_wasm_artifact_resolver_honors_redirected_cargo_target_dir(tmp_path):
    script = ROOT / "apps" / "desktop" / "scripts" / "build-wasm-engine.mjs"
    crate_dir = ROOT / "core" / "model_operations" / "operation_applier"
    redirected_target = tmp_path / "cargo-target"
    node_program = "\n".join(
        (
            f'import {{ resolveWasmArtifact }} from {json.dumps(script.as_uri())};',
            f"console.log(resolveWasmArtifact({json.dumps(str(crate_dir))}));",
        )
    )
    env = os.environ.copy()
    env["CARGO_TARGET_DIR"] = str(redirected_target)

    completed = subprocess.run(
        ("node", "--input-type=module", "--eval", node_program),
        cwd=ROOT,
        env=env,
        capture_output=True,
        text=True,
        check=False,
    )

    assert completed.returncode == 0, completed.stderr
    assert Path(completed.stdout.strip()) == (
        redirected_target
        / "wasm32-unknown-unknown"
        / "release"
        / "open_pipe_stress_operation_applier.wasm"
    )


def test_summary_binds_commit_hash_and_passes_when_all_surfaces_pass():
    sweep = load_module()

    summary = sweep.run_sweep(sweep.build_sweep_plan(), ROOT, runner=lambda c, r: 0)

    assert summary["artifact"] == "openpipestress.evidence_sweep_summary"
    assert summary["schema_version"] == 3
    assert summary["decision_basis"] == ["DEC-025", "DEC-093"]
    git_state = summary["git"]
    assert git_state["commit_hash"] and len(git_state["commit_hash"]) == 40
    assert git_state["status_capture_failed"] is False
    assert isinstance(git_state["working_tree_dirty"], bool)
    assert summary["overall_status"] == "pass"
    assert [entry["surface_id"] for entry in summary["surfaces"]] == (
        EXPECTED_SURFACE_ORDER
    )
    assert all(entry["status"] == "pass" for entry in summary["surfaces"])
    assert [entry["execution_capability"] for entry in summary["surfaces"]] == [
        surface.execution_capability for surface in sweep.build_sweep_plan()
    ]
    assert all(
        command["exit_code"] == 0
        for entry in summary["surfaces"]
        for command in entry["commands"]
    )
    assert sweep.validate_summary(summary) == []


def test_ci_summary_records_binding_and_validates_against_commit(monkeypatch):
    sweep = load_module()
    commit = "a" * 40
    binding = valid_ci_binding(head_sha=commit, run_attempt=2)
    monkeypatch.setattr(
        sweep,
        "collect_git_state",
        lambda root: {
            "commit_hash": commit,
            "branch": "ci-proof",
            "status_capture_failed": False,
            "working_tree_dirty": False,
            "dirty_paths": [],
        },
    )

    summary = sweep.run_sweep(
        sweep.build_sweep_plan(binding), ROOT, runner=lambda command, root: 0
    )
    surface4 = summary["surfaces"][3]

    assert surface4["execution_capability"] == "ci"
    assert surface4["commands"] == []
    assert surface4["ci_binding"] == binding
    assert surface4["status"] == "pass"
    assert summary["overall_status"] == "pass"
    assert sweep.validate_summary(summary) == []


def test_summary_validator_rejects_ci_head_sha_mismatch(monkeypatch):
    sweep = load_module()
    binding = valid_ci_binding(head_sha="a" * 40)
    monkeypatch.setattr(
        sweep,
        "collect_git_state",
        lambda root: {
            "commit_hash": "b" * 40,
            "branch": "ci-proof",
            "status_capture_failed": False,
            "working_tree_dirty": False,
            "dirty_paths": [],
        },
    )

    summary = sweep.run_sweep(
        sweep.build_sweep_plan(binding), ROOT, runner=lambda command, root: 0
    )
    errors = sweep.validate_summary(summary)

    assert any("does not equal sweep commit_hash" in error for error in errors)


def test_committed_historical_v2_corpus_remains_valid():
    sweep = load_module()
    paths = sorted((ROOT / "validation/evidence/sweeps").glob("SWEEP_*.json"))
    v2_summaries = []
    for path in paths:
        summary = json.loads(path.read_text(encoding="utf-8"))
        if summary.get("schema_version") == 2:
            v2_summaries.append((path, summary))

    assert v2_summaries
    assert {
        path.name: sweep.validate_summary(summary)
        for path, summary in v2_summaries
        if sweep.validate_summary(summary)
    } == {}
    assert any(
        sweep.is_complete_sweep_summary(summary)
        for _, summary in v2_summaries
    )


def test_historical_v2_validator_rejects_malformed_or_truncated_summary():
    sweep = load_module()
    summary = stub_sweep_summary(dirty=False)
    summary["surfaces"] = summary["surfaces"][:2]
    summary["surfaces"][1]["surface_id"] = "cargo_crate_sweep"

    errors = sweep.validate_summary(summary)

    assert any("selected DEC-025 order" in error for error in errors)
    assert any("surface_id values must be unique" in error for error in errors)
    assert sweep.is_complete_sweep_summary(summary) is False


def test_validator_rejects_non_string_or_non_utc_started_timestamp():
    sweep = load_module()
    summary = stub_sweep_summary(dirty=False)

    summary["started_utc"] = {"malformed": True}
    assert any("started_utc" in error for error in sweep.validate_summary(summary))
    assert sweep.is_complete_sweep_summary(summary) is False

    summary["started_utc"] = "2026-08-19T12:00:00"
    assert any("started_utc" in error for error in sweep.validate_summary(summary))


def test_validator_rejects_contradictory_or_unverified_dirty_paths():
    sweep = load_module()
    summary = stub_sweep_summary(dirty=False)
    summary["git"]["dirty_paths"] = ["modified-file"]

    assert any(
        "working_tree_dirty" in error for error in sweep.validate_summary(summary)
    )
    assert sweep.is_complete_sweep_summary(summary) is False

    summary["git"].update(
        status_capture_failed=True,
        working_tree_dirty=None,
    )
    assert any("dirty_paths must be empty" in error for error in sweep.validate_summary(summary))


def test_schema_v2_cannot_claim_a_ci_capability_selection():
    sweep = load_module()
    summary = stub_sweep_summary(dirty=False)
    summary["surfaces"] = [summary["surfaces"][3]]
    summary["surfaces"][0]["order"] = 1
    summary["surface_selection"] = {
        "only_capability": "ci",
        "complete_dec025_sweep": False,
    }

    errors = sweep.validate_summary(summary)

    assert any("does not match surface_selection 'ci'" in error for error in errors)


def test_ci_binding_remains_valid_when_an_earlier_surface_fails(monkeypatch):
    sweep = load_module()
    commit = "a" * 40
    binding = valid_ci_binding(head_sha=commit)
    monkeypatch.setattr(
        sweep,
        "collect_git_state",
        lambda root: {
            "commit_hash": commit,
            "branch": "ci-proof",
            "status_capture_failed": False,
            "working_tree_dirty": False,
            "dirty_paths": [],
        },
    )

    summary = sweep.run_sweep(
        sweep.build_sweep_plan(binding),
        ROOT,
        runner=lambda command, root: 1,
    )

    assert summary["overall_status"] == "fail"
    assert summary["surfaces"][3]["status"] == "not_run"
    assert sweep.validate_summary(summary) == []


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


def test_dirty_default_routes_outside_canonical_evidence(
    monkeypatch, capsys, tmp_path
):
    sweep = load_module()
    root = tmp_path / "repo"
    root.mkdir()
    temporary_root = tmp_path / "temporary-output"
    monkeypatch.setattr(sweep.tempfile, "gettempdir", lambda: str(temporary_root))
    monkeypatch.setattr(sweep, "missing_tools", lambda surfaces: [])
    monkeypatch.setattr(sweep, "preflight_prerequisites", lambda root, surfaces: [])
    monkeypatch.setattr(
        sweep, "run_sweep", lambda surfaces, root: stub_sweep_summary(dirty=True)
    )

    result = sweep.main(["--execute", "--repo-root", str(root)])
    captured = capsys.readouterr()

    canonical = root / sweep.DEFAULT_OUTPUT_DIR
    routed = temporary_root / sweep.DIRTY_DEFAULT_OUTPUT_DIR_NAME
    assert result == 0
    assert not canonical.exists()
    assert len(list(routed.glob("SWEEP_*-dirty.json"))) == 1
    assert "canonical evidence output is disabled" in captured.out
    assert str(routed) in captured.out


def test_clean_default_writes_canonical_evidence(monkeypatch, tmp_path):
    sweep = load_module()
    root = tmp_path / "repo"
    root.mkdir()
    monkeypatch.setattr(sweep, "missing_tools", lambda surfaces: [])
    monkeypatch.setattr(sweep, "preflight_prerequisites", lambda root, surfaces: [])
    monkeypatch.setattr(
        sweep, "run_sweep", lambda surfaces, root: stub_sweep_summary(dirty=False)
    )

    result = sweep.main(["--execute", "--repo-root", str(root)])

    summaries = list((root / sweep.DEFAULT_OUTPUT_DIR).glob("SWEEP_*.json"))
    assert result == 0
    assert len(summaries) == 1
    assert json.loads(summaries[0].read_text(encoding="utf-8"))["schema_version"] == 2


def test_dirty_canonical_output_requires_and_honors_explicit_opt_in(
    monkeypatch, capsys, tmp_path
):
    sweep = load_module()
    root = tmp_path / "repo"
    root.mkdir()
    monkeypatch.setattr(sweep, "missing_tools", lambda surfaces: [])
    monkeypatch.setattr(sweep, "preflight_prerequisites", lambda root, surfaces: [])
    monkeypatch.setattr(
        sweep, "run_sweep", lambda surfaces, root: stub_sweep_summary(dirty=True)
    )

    rejected = sweep.main(
        [
            "--execute",
            "--repo-root",
            str(root),
            "--output-dir",
            sweep.DEFAULT_OUTPUT_DIR,
        ]
    )
    rejected_output = capsys.readouterr()

    canonical = root / sweep.DEFAULT_OUTPUT_DIR
    assert rejected == 1
    assert not canonical.exists()
    assert "--allow-dirty-canonical-output" in rejected_output.err
    assert "no summary was written" in rejected_output.err

    accepted = sweep.main(
        [
            "--execute",
            "--repo-root",
            str(root),
            "--allow-dirty-canonical-output",
        ]
    )
    accepted_output = capsys.readouterr()

    assert accepted == 0
    assert len(list(canonical.glob("SWEEP_*-dirty.json"))) == 1
    assert "dirty canonical output explicitly enabled" in accepted_output.out


def test_dirty_explicit_noncanonical_output_dir_remains_supported(tmp_path):
    sweep = load_module()
    root = tmp_path / "repo"
    explicit = tmp_path / "caller-selected"

    output_dir, disposition = sweep.resolve_summary_output_dir(
        root=root,
        requested_output_dir=str(explicit),
        git_state=stub_sweep_summary(dirty=True)["git"],
        allow_dirty_canonical_output=False,
    )

    assert output_dir == explicit.resolve()
    assert disposition == "explicit-noncanonical"


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
    assert "execution capability: host" in captured.out


def test_sandboxed_capability_preflight_fails_before_any_execution(
    monkeypatch, capsys
):
    sweep = load_module()

    def fail_preflight(*args, **kwargs):
        raise AssertionError("capability failure must precede prerequisite probes")

    def fail_run_sweep(*args, **kwargs):
        raise AssertionError("capability failure must stop before surface 1")

    monkeypatch.setattr(sweep, "preflight_prerequisites", fail_preflight)
    monkeypatch.setattr(sweep, "run_sweep", fail_run_sweep)

    result = sweep.main(
        [
            "--execute",
            "--require-capability",
            "sandboxed",
            "--repo-root",
            str(ROOT),
        ]
    )
    captured = capsys.readouterr()

    assert result == 1
    assert "capability preflight failed before surface 1" in captured.err
    assert "desktop_playwright_e2e requires host" in captured.err
    assert "no evidence surface ran" in captured.err


def test_host_capability_preflight_accepts_the_complete_plan():
    sweep = load_module()

    assert sweep.capability_preflight_errors(
        sweep.build_sweep_plan(), "host"
    ) == []


def test_ci_capability_preflight_accepts_bound_surface_and_sandboxed_surfaces():
    sweep = load_module()
    binding = valid_ci_binding(head_sha="a" * 40)

    assert sweep.capability_preflight_errors(
        sweep.build_sweep_plan(binding), "ci"
    ) == []


def test_main_accepts_ci_binding_and_writes_valid_summary(
    monkeypatch, tmp_path
):
    sweep = load_module()
    commit = sweep.collect_git_state(ROOT)["commit_hash"]
    binding_path = tmp_path / "ci-binding.json"
    binding_path.write_text(
        json.dumps(valid_ci_binding(head_sha=commit, run_attempt=2)),
        encoding="utf-8",
    )
    monkeypatch.setattr(sweep, "missing_tools", lambda surfaces: [])
    monkeypatch.setattr(sweep, "preflight_prerequisites", lambda root, surfaces: [])
    monkeypatch.setattr(sweep, "run_command", lambda command, root: 0)

    result = sweep.main(
        [
            "--execute",
            "--repo-root",
            str(ROOT),
            "--surface4-ci-binding",
            str(binding_path),
            "--require-capability",
            "ci",
            "--output-dir",
            str(tmp_path / "summaries"),
        ]
    )

    assert result == 0
    summaries = list((tmp_path / "summaries").glob("SWEEP_*.json"))
    assert len(summaries) == 1
    summary = json.loads(summaries[0].read_text(encoding="utf-8"))
    assert summary["surfaces"][3]["execution_capability"] == "ci"
    assert summary["surfaces"][3]["ci_binding"]["run_attempt"] == 2
    assert sweep.validate_summary(summary) == []


def test_main_rejects_ci_binding_head_mismatch_before_surface_1(
    monkeypatch, capsys, tmp_path
):
    sweep = load_module()
    binding_path = tmp_path / "ci-binding.json"
    binding_path.write_text(
        json.dumps(valid_ci_binding(head_sha="0" * 40)), encoding="utf-8"
    )

    def fail_run(*args, **kwargs):
        raise AssertionError("a mismatched CI binding must stop before surface 1")

    monkeypatch.setattr(sweep, "run_sweep", fail_run)

    result = sweep.main(
        [
            "--execute",
            "--repo-root",
            str(ROOT),
            "--surface4-ci-binding",
            str(binding_path),
            "--output-dir",
            str(tmp_path / "summaries"),
        ]
    )
    captured = capsys.readouterr()

    assert result == 1
    assert "rejected before surface 1" in captured.err
    assert "does not equal sweep commit_hash" in captured.err
    assert not (tmp_path / "summaries").exists()


def test_sandboxed_capability_selection_excludes_host_surface():
    sweep = load_module()

    selected = sweep.select_surfaces(sweep.build_sweep_plan(), "sandboxed")

    assert [surface.surface_id for surface in selected] == [
        "cargo_crate_sweep",
        "python_pytest",
        "desktop_vitest",
        "desktop_production_build",
    ]
    assert sweep.capability_preflight_errors(selected, "sandboxed") == []


def test_main_executes_only_sandboxed_selection_and_records_partial_summary(
    monkeypatch, capsys, tmp_path
):
    sweep = load_module()
    observed_commands = []

    def record_command(command, root):
        observed_commands.append(command)
        return 0

    monkeypatch.setattr(sweep, "missing_tools", lambda surfaces: [])
    monkeypatch.setattr(sweep, "preflight_prerequisites", lambda root, surfaces: [])
    monkeypatch.setattr(sweep, "run_command", record_command)

    result = sweep.main(
        [
            "--execute",
            "--only-capability",
            "sandboxed",
            "--require-capability",
            "sandboxed",
            "--repo-root",
            str(ROOT),
            "--output-dir",
            str(tmp_path),
        ]
    )
    captured = capsys.readouterr()

    assert result == 0
    assert all(
        all("test:e2e" not in argument for argument in command)
        for command in observed_commands
    )
    assert "surface selection: only sandboxed (partial DEC-025 sweep)" in captured.out
    summaries = list(tmp_path.glob("SWEEP_*.json"))
    assert len(summaries) == 1
    parsed = json.loads(summaries[0].read_text(encoding="utf-8"))
    assert parsed["surface_selection"] == {
        "only_capability": "sandboxed",
        "complete_dec025_sweep": False,
    }
    assert [entry["surface_id"] for entry in parsed["surfaces"]] == [
        "cargo_crate_sweep",
        "python_pytest",
        "desktop_vitest",
        "desktop_production_build",
    ]


def test_main_execute_writes_summary_and_returns_failure_exit(
    monkeypatch, capsys, tmp_path
):
    sweep = load_module()

    monkeypatch.setattr(
        sweep, "run_command", lambda command, root: 1 if "pytest" in command else 0
    )
    monkeypatch.setattr(sweep, "preflight_prerequisites", lambda root, surfaces: [])

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
        lambda root, surfaces: ["missing local prerequisite"],
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
