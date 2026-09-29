#!/usr/bin/env python3
"""Run K6's kernel-observation runner tests on the DEC-025 pytest surface.

The tests live beside the runner (core/solver/performance_harness/runner/,
test_k6_runner.py; ROOT's K6 ruling Q9(b)). This thin wrapper imports their
unittest classes so that pytest collects them: every DEC-025 sweep then runs
the runner's macOS RSS-watchdog path on the Mac and its RLIMIT_AS path on
Linux. The live tests spawn only self-limiting Python children under a 128 MiB
cap (failsafe at 512 MiB). Nothing is skipped. Observation only: no time or
memory bound is asserted.
"""

import sys
from pathlib import Path

RUNNER = Path(__file__).resolve().parents[1] / "core" / "solver" / "performance_harness" / "runner"
sys.path.insert(0, str(RUNNER))

from test_k6_runner import (  # noqa: E402,F401  (collected by pytest)
    Aggregation,
    Classification,
    CountsClosedForms,
    LiveLimit,
    MetadataHasNoHostIdentifiers,
    ModelHashes,
    Parsers,
    PlanAdmission,
    RlimitOnlyOnLinux,
    Schema,
    SectionBits,
    Watchdog,
)
