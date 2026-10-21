"""The release sweep's `stack-witnesses` profile (DEL-04-08 Design `stack-headroom-repair.md`,
`w1-coverage-repair.md` §1.4). This profile reads no `execution/` file, so this test also runs in
hosted lanes that check out without it."""
from __future__ import annotations

import importlib.util
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
MODULE_PATH = ROOT / "tools" / "release" / "check_release_readiness.py"


def load_module():
    spec = importlib.util.spec_from_file_location("check_release_readiness_stack", MODULE_PATH)
    assert spec is not None and spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def test_stack_witness_profile_runs_the_hook_driven_exhaustion_control():
    release = load_module()
    commands = [list(step.command) for step in release.build_plan("stack-witnesses", ROOT)]
    hooks = [c for c in commands if "loaded-maximum-hooks" in c]
    assert hooks == [[
        "cargo", "test", "--offline", "--manifest-path", "core/product_physics/Cargo.toml", "--lib",
        "--features", "loaded-maximum-hooks", "--",
        "w1_member_load_exhaustion_retains_typed_interval_work", "w1_final_scales_equal_reader_scales",
    ]]
    assert len(commands) == 7
