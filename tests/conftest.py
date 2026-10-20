"""Test-session setup for the checked-JSON and unit authorities and the Rust reader's runner.

The xdist controller builds once before worker collection. Workers inherit the
explicit executable path and never invoke Cargo themselves.

With OPENPIPESTRESS_NO_CARGO_BUILD=1 no test runs Cargo. Each binary comes from
its OPENPIPESTRESS_*_BIN preset or an existing build at the location the
explicit build would use; a test that needs an absent binary, or runs Cargo
itself (marker `runs_cargo`), is skipped with the reason. Without it, behaviour
is unchanged: missing binaries are built, as hosted CI relies on.
"""
from __future__ import annotations
import atexit
import os
from pathlib import Path
import shutil
import stat
import sys
import tempfile

import pytest


PROJECT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(PROJECT))

# "1": tests never run Cargo; they use the *_BIN presets or existing builds, else skip.
NO_CARGO_BUILD = "OPENPIPESTRESS_NO_CARGO_BUILD"
# Preset for the compatibility `headless_preview_runner` (test_headless_runner_contract.py).
PREVIEW_RUNNER_BIN = "OPENPIPESTRESS_PREVIEW_RUNNER_BIN"

# (preset, tools/ subdirectory, build module, executable) for the session authorities.
SESSION_BINARIES = (
    ("OPENPIPESTRESS_CHECKED_JSON_BIN", "serialization", "build_checked_json", "openpipestress_jcs_ijson"),
    ("OPENPIPESTRESS_UNITS_BIN", "units", "build_units_authority", "openpipestress_units"),
    ("OPENPIPESTRESS_RUNNER_BIN", "runner", "build_runner", "openpipestress-runner"),
)
# The adapters' refusals when a session authority is absent.
MISSING_AUTHORITY = {
    "OPENPIPESTRESS_CHECKED_JSON_BIN": "CHECKED-JSON-AUTHORITY-MISSING",
    "OPENPIPESTRESS_UNITS_BIN": "UNITS-AUTHORITY-MISSING",
    "OPENPIPESTRESS_RUNNER_BIN": "RUST-READER-MISSING",
}
# Session authorities found absent at session start under the opt-out.
_ABSENT: set[str] = set()


def cargo_build_disabled() -> bool:
    return os.environ.get(NO_CARGO_BUILD) == "1"


def available_binary(env_name: str, built: Path) -> Path | None:
    """The preset executable, else an existing build at `built`; never builds."""
    configured = os.environ.get(env_name)
    path = Path(configured) if configured else built
    return path.resolve() if path.is_file() else None


def _refuse_cargo_on_path() -> None:
    """Put a `cargo` first on PATH that refuses to run, so no test reaches the real one."""
    guard = Path(tempfile.mkdtemp(prefix="openpipestress-no-cargo-"))
    atexit.register(shutil.rmtree, guard, True)
    cargo = guard / "cargo"
    cargo.write_text(f'#!/bin/sh\necho "{NO_CARGO_BUILD}=1: cargo is not run in this test session" >&2\nexit 1\n',
                     encoding="utf-8")
    cargo.chmod(cargo.stat().st_mode | stat.S_IXUSR | stat.S_IXGRP | stat.S_IXOTH)
    os.environ["PATH"] = os.pathsep.join([str(guard), os.environ.get("PATH", "")])


def _use_existing_session_binaries() -> None:
    for env_name, tool, module, executable in SESSION_BINARIES:
        sys.path.insert(0, str(PROJECT / "tools" / tool))
        built = __import__(module).DEFAULT_TARGET / "release" / executable
        found = available_binary(env_name, built)
        if found is None:
            os.environ.pop(env_name, None)
        else:
            os.environ[env_name] = str(found)


def pytest_configure(config) -> None:
    config.addinivalue_line(
        "markers", f"runs_cargo(what): the test runs Cargo itself; skipped when {NO_CARGO_BUILD}=1"
    )


def pytest_sessionstart(session) -> None:
    if hasattr(session.config, "workerinput"):
        if cargo_build_disabled():
            _ABSENT.update(name for name in MISSING_AUTHORITY if not os.environ.get(name))
        return
    if cargo_build_disabled():
        _use_existing_session_binaries()
        _ABSENT.update(name for name in MISSING_AUTHORITY if not os.environ.get(name))
        if os.name == "posix":
            _refuse_cargo_on_path()
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


def pytest_collection_modifyitems(config, items) -> None:
    if not cargo_build_disabled():
        return
    for item in items:
        marker = item.get_closest_marker("runs_cargo")
        if marker is not None:
            item.add_marker(pytest.mark.skip(reason=f"{NO_CARGO_BUILD}=1: this test runs Cargo itself ({marker.args[0]})"))


@pytest.hookimpl(hookwrapper=True)
def pytest_runtest_makereport(item, call):
    """Under the opt-out, a test stopped by an absent session authority is a skip, not a failure."""
    outcome = yield
    if not cargo_build_disabled() or call.excinfo is None or not _ABSENT:
        return
    message = str(call.excinfo.value)  # includes refusals a test caught and reported in its assertion
    for env_name, code in MISSING_AUTHORITY.items():
        if f"{code}:" in message and env_name in _ABSENT:
            report = outcome.get_result()
            report.outcome = "skipped"
            report.longrepr = (str(item.path), item.location[1] or 0,
                               f"Skipped: {NO_CARGO_BUILD}=1 and {env_name} is neither supplied nor built ({code})")
            return


@pytest.fixture(scope="session")
def no_cargo_build() -> bool:
    return cargo_build_disabled()


@pytest.fixture(scope="session")
def prebuilt_binary():
    """Under the opt-out: `require(preset, built, what)` returns the binary or skips."""
    def require(env_name: str, built: Path, what: str) -> Path:
        found = available_binary(env_name, built)
        if found is None:
            pytest.skip(f"{NO_CARGO_BUILD}=1 and {what} is neither supplied by {env_name} nor built at {built}")
        return found
    return require
