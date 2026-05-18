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

    assert any("validate_dependencies_schema.py" in command for command in commands)
    assert any("test_release_readiness_script.py" in command for command in commands)
    assert all(isinstance(step.command, tuple) for step in steps)
