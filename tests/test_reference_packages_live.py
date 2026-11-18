"""LIVE CHECKS of the reference packages that staged inputs from live repository files.

Such a package's default check, run by tests/test_reference_packages_regeneration.py, reads only what the package
holds, so it establishes that the committed output follows from the staged inputs. Its separate live check
(--check-live) compares those staged inputs with the repository files they were staged from. The table module
names those files as constants (reference_packages.PREVIEW_MODEL, DEMO_MODEL, FRICTION_REVERSAL_TEST), so the CI
selector runs this test when any of them changes. A failure here means a source has moved on and a decision is
needed on re-staging; the committed output still stands for its staged inputs.

Each check runs as `python3 -I -B` from its package directory, in place, within reference_packages.CHECK_TIMEOUT_S,
and must leave every file under validation/references/ as it was. No product binary is built or run.
"""
from __future__ import annotations

import subprocess
import sys
from dataclasses import dataclass, replace
from pathlib import Path

import pytest

import reference_packages as R
from reference_packages import PACKAGES


@dataclass(frozen=True)
class LiveCheck:
    """A package's live check: the command, run from the package directory, and the repository files (repository-
    relative) its staged inputs came from. It is not one of the package's `checks`, which read only the package."""
    argv: tuple
    sources: tuple


LIVE_CHECKS = {
    "t4_i12_preload": LiveCheck(("generators/u3_preload_reference.py", "--check-live"), (R.PREVIEW_MODEL,)),
    "demo_friction_reversal_preload": LiveCheck(("generators/demo_friction_preload_reference.py", "--check-live"),
                                                (R.DEMO_MODEL, R.FRICTION_REVERSAL_TEST)),
}

MOVED_ON = ("a source this package's inputs were staged from has moved on, and a decision is needed: re-stage the "
            "inputs and re-derive, or keep the staged inputs; the committed output still stands for its staged inputs")


def declaration_problems(name: str) -> list[str]:
    """The package has an entry and holds the generator; each source is a committed file, and the generator names
    every part of its path as a quoted string, so the declared sources are the ones it reads."""
    live = LIVE_CHECKS[name]
    if name not in PACKAGES:
        return [f"{name}: a live check with no package entry"]
    generator = R.REFS / name / live.argv[0]
    if not generator.is_file():
        return [f"{name}: the live check runs {live.argv[0]}, which the package does not hold"]
    text = generator.read_text(encoding="utf-8")
    problems = [f"{name}: live-check source {source} is not a committed file"
                for source in live.sources if not (R.ROOT / source).is_file()]
    problems += [f"{name}: {live.argv[0]} does not name {source} (part {part!r} absent)"
                 for source in live.sources for part in Path(source).parts if f'"{part}"' not in text]
    return problems


def live_problems(name: str) -> list[str]:
    """Runs the live check in place. It must leave validation/references/ as it was and exit 0; a nonzero exit
    means a source has moved on (MOVED_ON)."""
    live = LIVE_CHECKS[name]
    before = R.tree_state()
    label = f"python3 -I -B {' '.join(live.argv)} (in validation/references/{name})"
    try:
        run = subprocess.run([sys.executable, "-I", "-B", *live.argv], cwd=R.REFS / name,
                             capture_output=True, timeout=R.CHECK_TIMEOUT_S)
    except subprocess.TimeoutExpired:
        return [f"{label} did not finish within its {R.CHECK_TIMEOUT_S} s limit (reference_packages.CHECK_TIMEOUT_S)"]
    written = R._changes(before, R.tree_state())
    if written:
        return [f"{label} wrote under validation/references/:"] + written
    if run.returncode != 0:
        output = (run.stdout + run.stderr).decode("utf-8", "replace")
        return [f"{label} exited {run.returncode}: {MOVED_ON}. Sources compared: {', '.join(live.sources)}. "
                f"What it reported:\n{output[-3000:]}"]
    return []


@pytest.mark.parametrize("name", sorted(LIVE_CHECKS))
def test_the_live_check_is_declared_with_the_sources_its_generator_reads(name):
    problems = declaration_problems(name)
    assert not problems, "\n".join(problems)


@pytest.mark.parametrize("name", sorted(LIVE_CHECKS))
def test_the_live_check_is_kept_apart_from_the_default_check(name):
    """The default check reads only the package; the live check is not one of the package's checks."""
    assert PACKAGES[name].checks, f"{name}: no default check"
    assert all("--check-live" not in check.argv for check in PACKAGES[name].checks), name


@pytest.mark.parametrize("name", sorted(LIVE_CHECKS))
def test_the_staged_inputs_still_match_their_sources(name):
    problems = live_problems(name)
    assert not problems, f"{name}:\n" + "\n".join(problems)


def test_a_nonzero_exit_says_a_source_has_moved_on(monkeypatch):
    name = sorted(LIVE_CHECKS)[0]
    monkeypatch.setitem(LIVE_CHECKS, name, replace(LIVE_CHECKS[name], argv=("-c", "raise SystemExit(1)")))
    problems = live_problems(name)
    assert len(problems) == 1 and "exited 1" in problems[0] and MOVED_ON in problems[0]


def test_a_source_the_generator_does_not_name_fails_its_declaration(monkeypatch):
    name = sorted(LIVE_CHECKS)[0]
    monkeypatch.setitem(LIVE_CHECKS, name, replace(LIVE_CHECKS[name], sources=("unnamed_source.json",)))
    problems = declaration_problems(name)
    assert any("does not name unnamed_source.json" in p for p in problems), problems
