"""The registered native-smoke node-test check and the hosted step that runs it agree.

`desktop-native-smoke-node-tests` (software-workflow.json) runs the native smoke tooling's own
`node --test` files, which build and launch nothing. The hosted desktop job runs the same command
on its first shard. These tests keep the registration, its path rule and the hosted step aligned.
"""
from __future__ import annotations

import json
import shlex
from pathlib import Path

import yaml

from select_affected_checks import select_checks

PROJECT = Path(__file__).resolve().parents[2]
CHECK = "desktop-native-smoke-node-tests"


def profile() -> dict:
    return json.loads((PROJECT / "software-workflow.json").read_text())


def test_the_check_runs_node_test_over_the_native_smoke_tests_without_a_shell():
    check = profile()["checks"][CHECK]
    assert check == {"cwd": ".", "command": ["node", "--test", "apps/desktop/native-smoke/*.test.mjs"]}
    assert sorted(PROJECT.glob("apps/desktop/native-smoke/*.test.mjs")), "the glob names at least one test"


def test_the_path_rule_selects_the_check_for_the_tests_and_what_they_read():
    tests = sorted(path.relative_to(PROJECT).as_posix() for path in PROJECT.glob("apps/desktop/native-smoke/*.test.mjs"))
    inputs = tests + [
        "apps/desktop/native-smoke/driver.js",
        "apps/desktop/native-smoke/perf.mjs",
        "apps/desktop/native-smoke/evidence-checks.mjs",
        "apps/desktop/native-smoke/evidence-journeys.mjs",
        "apps/desktop/src-tauri/src/native_smoke.rs",
        "apps/desktop/scripts/appBinaryBinding.mjs",
        "apps/desktop/scripts/engineSourceStamp.mjs",
        "apps/desktop/scripts/worktreeBuildLock.mjs",
        "apps/desktop/package.json",
        "core/solver/performance_harness/observations/rack/models_sha256.txt",
        "core/solver/performance_harness/rack/rack_model.py",
        "docs/PRD.md",
    ]
    for path in inputs:
        assert CHECK in select_checks(profile(), [path])["checks"], path
    assert CHECK not in select_checks(profile(), ["apps/desktop/src/main.tsx"])["checks"]


def test_the_hosted_desktop_job_runs_the_registered_command_once():
    workflow = yaml.safe_load((PROJECT / ".github/workflows/piping-desktop-e2e.yml").read_text())
    command = profile()["checks"][CHECK]["command"]
    steps = [step for step in workflow["jobs"]["desktop"]["steps"] if step.get("run", "").startswith("node --test")]
    assert len(steps) == 1
    [step] = steps
    assert shlex.split(step["run"]) == command
    assert step.get("working-directory") == "."
    assert "matrix.shard == 1" in step["if"] and "cache-hit != 'true'" in step["if"]
