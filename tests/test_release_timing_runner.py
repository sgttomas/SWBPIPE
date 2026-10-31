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
    Generate,
    LiveFootprint,
    LiveLaunch,
    MetadataHasNoHostIdentifiers,
    PacketAggregation,
    Plan,
    ProgressLines,
    Window,
)
