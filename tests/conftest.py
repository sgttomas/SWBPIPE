"""Test-session setup for the private checked-JSON authority.

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
    if os.environ.get("OPENPIPESTRESS_CHECKED_JSON_BIN"):
        return
    sys.path.insert(0, str(PROJECT / "tools" / "serialization"))
    from build_checked_json import build
    os.environ["OPENPIPESTRESS_CHECKED_JSON_BIN"] = str(build())
