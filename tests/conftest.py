"""Test-session setup for the checked-JSON and unit authorities and the Rust reader's runner.

The xdist controller builds once before worker collection. Workers inherit the
explicit executable path and never invoke Cargo themselves.
"""
from __future__ import annotations
import os
from pathlib import Path
import sys


PROJECT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(PROJECT))


def pytest_sessionstart(session) -> None:
    if hasattr(session.config, "workerinput"):
        return
    if not os.environ.get("OPENPIPESTRESS_CHECKED_JSON_BIN"):
        sys.path.insert(0, str(PROJECT / "tools" / "serialization"))
        from build_checked_json import build
        os.environ["OPENPIPESTRESS_CHECKED_JSON_BIN"] = str(build())
    if not os.environ.get("OPENPIPESTRESS_UNITS_BIN"):
        sys.path.insert(0, str(PROJECT / "tools" / "units"))
        from build_units_authority import build as build_units
        os.environ["OPENPIPESTRESS_UNITS_BIN"] = str(build_units())
    if not os.environ.get("OPENPIPESTRESS_RUNNER_BIN"):
        sys.path.insert(0, str(PROJECT / "tools" / "runner"))
        from build_runner import build as build_runner
        os.environ["OPENPIPESTRESS_RUNNER_BIN"] = str(build_runner())
