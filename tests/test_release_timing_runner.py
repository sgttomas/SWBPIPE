#!/usr/bin/env python3
"""Run the DEL-04-05 release timing runner's tests on the DEC-025 pytest surface.

The tests live beside the runner (core/solver/performance_harness/runner/,
test_release_runner.py). This thin wrapper imports their unittest classes so
that pytest collects them. No Rust binary is needed: the live tests spawn only
tiny self-limiting Python children (a stand-in driver; growing children killed
by the 128 MiB RSS cap or, on macOS, a 128 MiB footprint budget, each with its
failsafe at 512 MiB; a busy child under `sample` for the attribution
snapshots). Observation only: no time or memory bound is asserted.
"""

import sys
from pathlib import Path

RUNNER = Path(__file__).resolve().parents[1] / "core" / "solver" / "performance_harness" / "runner"
sys.path.insert(0, str(RUNNER))

from test_release_runner import (  # noqa: E402,F401  (collected by pytest)
    Attribution,
    Classification,
    ClassificationReadings,
    Generate,
    LiveFootprint,
    LiveLaunch,
    MetadataHasNoHostIdentifiers,
    PacketAggregation,
    Plan,
    ProgressLines,
    RunIds,
    Window,
    WritePlan,
)


def _window(tmp_path, packet_status, run_status=0):
    """release_window.sh run beside a stand-in runner whose --run and --packet steps exit as told.

    The script finds the runner beside itself, so a copy of the script in a temporary folder runs
    the stand-in: no driver, no model and no solve.
    """
    import os
    import shutil
    import subprocess

    here = tmp_path / "runner"
    here.mkdir()
    shutil.copy(RUNNER / "release_window.sh", here / "release_window.sh")
    (here / "release_runner.py").write_text(
        "import sys\n"
        "step = next(a for a in sys.argv[1:] if a in ('--plan', '--run', '--packet'))\n"
        "print('stand-in', step)\n"
        f"sys.exit({{'--plan': 0, '--run': {run_status}, '--packet': {packet_status}}}[step])\n"
    )
    models = tmp_path / "models"
    models.mkdir()
    (models / "manifest.json").write_text("{}")
    binary = tmp_path / "driver"
    binary.write_text("#!/bin/sh\nexit 0\n")
    binary.chmod(0o755)
    out = tmp_path / "out"
    out.mkdir()
    env = {key: value for key, value in os.environ.items() if not key.startswith("RELEASE_WINDOW_")}
    done = subprocess.run(
        ["/bin/bash", str(here / "release_window.sh"), str(models), str(out), str(binary)],
        capture_output=True, text=True, env=env, timeout=60,
    )
    logs = [path.read_text() for path in out.glob("window-*/window.log")]
    return done.returncode, done.stdout, (logs[0] if logs else "")


def test_the_window_script_does_not_report_done_after_a_failed_packet_step(tmp_path):
    """A failed packet step is a failed window: its status leaves and DONE is never printed."""
    code, stdout, log = _window(tmp_path, packet_status=3)
    assert code == 3, stdout
    assert "WINDOW DONE" not in stdout and "WINDOW DONE" not in log
    assert "WINDOW NOT DONE: the packet step failed rc=3" in stdout and "WINDOW NOT DONE" in log


def test_the_window_script_keeps_a_failed_runs_status_and_reports_done_only_when_packaged(tmp_path):
    """The runs' own failure keeps its status whatever the packet step does."""
    packaged = tmp_path / "packaged"
    packaged.mkdir()
    code, stdout, _ = _window(packaged, packet_status=0, run_status=1)
    assert code == 1 and "WINDOW DONE" in stdout
    both = tmp_path / "both"
    both.mkdir()
    code, stdout, _ = _window(both, packet_status=3, run_status=1)
    assert code == 1 and "WINDOW DONE" not in stdout and "WINDOW NOT DONE" in stdout
    clean = tmp_path / "clean"
    clean.mkdir()
    code, stdout, _ = _window(clean, packet_status=0)
    assert code == 0 and "WINDOW DONE" in stdout and "WINDOW NOT DONE" not in stdout
