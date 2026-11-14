"""REGENERATION of every reference package under validation/references/: each retained check command the package's
README gives, from its entry in tests/reference_packages.py (`check_problems`), must exit 0 (and, where the
entry says so, print exactly a frozen file); any other exit status is a failure. This test does not check
that a package's generators are independent of product code.

No check writes into a package. Every command is a check mode (a generator's default comparison, its
--check, or a self-test); the one that writes (m3_reproduce.py) is given a fresh directory under pytest's
tmp_path. The test does not rely on that alone: it records every file under validation/references/
(SHA-256, size and modification time; bytecode caches aside, which -B keeps every check from writing)
before and after each run, and fails on any change, addition or removal.

The generators run as `python3 -I -B` from their package directory, one per check, each within
reference_packages.CHECK_TIMEOUT_S, set from the slowest time observed. A stdout comparison limited to named
platforms is reported as a StdoutNotCompared warning elsewhere, with what differs. No product binary and no environment switch is needed, so the
test runs under OPENPIPESTRESS_NO_CARGO_BUILD=1 (tests/conftest.py). What it does not regenerate is listed
in `reference_packages.taken_on_trust()` and counted by TAKEN_ON_TRUST_COUNT.

Each package's checks are one test class, so that the hosted Python job's `--dist loadscope`, which keeps
a class on one worker, spreads the packages over its workers instead of running them all on one.
"""
from __future__ import annotations

from dataclasses import replace
from pathlib import Path

import pytest

import reference_packages as R
from reference_packages import PACKAGES

# The one maintained test that will regenerate the F1 modal packages (PR 268); not on main yet.
AWAITED = Path(__file__).resolve().parent / "test_dynamics_references.py"


def _check_class(name: str) -> type:
    indices = list(range(len(PACKAGES[name].checks)))

    @pytest.mark.parametrize("index", indices)
    def test_check(self, index, tmp_path):
        problems = R.check_problems(name, index, tmp_path / "check_output")
        assert not problems, f"{name}:\n" + "\n".join(problems)

    return type(f"TestRegenerate_{name.replace('/', '__')}", (), {"test_check": test_check, "package": name})


for _name in sorted(n for n, p in PACKAGES.items() if p.checks):
    _cls = _check_class(_name)
    globals()[_cls.__name__] = _cls
del _name, _cls


def test_taken_on_trust_is_counted():
    rows = R.taken_on_trust()
    print("\nTaken on trust (not reproduced by this test):")
    for row in rows:
        print("  " + " | ".join(row))
    assert len(rows) == R.TAKEN_ON_TRUST_COUNT, (
        f"{len(rows)} entries are taken on trust, TAKEN_ON_TRUST_COUNT says {R.TAKEN_ON_TRUST_COUNT}; "
        "a change to that list is a decision, so change the count with it")


@pytest.mark.parametrize("name", sorted(n for n, p in PACKAGES.items() if p.other_test))
def test_a_package_awaiting_its_own_test_is_counted(name):
    """A package whose regeneration waits for a maintained test not yet on main is not skipped: while that
    test is absent it is a printed, counted state (one of `taken_on_trust()`); once the test lands it must
    name the package, and this entry, its trust row and the count change with it."""
    pkg = PACKAGES[name]
    assert pkg.other_test == f"tests/{AWAITED.name}", f"{name}: awaits {pkg.other_test}, this test knows {AWAITED.name}"
    assert (name, R.REGENERATION_GAP, R.F1_TRUST) in R.taken_on_trust(), f"{name}: its trust row is missing"
    if not AWAITED.is_file():
        print(f"\n{name}: regeneration {R.F1_TRUST} ({pkg.other_test} is not on this branch); counted in "
              f"TAKEN_ON_TRUST_COUNT")
        return
    assert f"validation/references/{name}" in AWAITED.read_text(encoding="utf-8"), f"{AWAITED.name} does not name {name}"
    pytest.fail(f"{pkg.other_test} is on this branch and names {name}: replace the entry's other_test and "
                f"regeneration gap with its check commands, and lower TAKEN_ON_TRUST_COUNT")


def test_a_nonzero_exit_of_a_check_is_a_failure(monkeypatch, tmp_path):
    """No exit status but 0 passes: there is no known-failure record to admit one."""
    pkg = R.PACKAGES["rt_a3_tee"]
    monkeypatch.setitem(R.PACKAGES, "rt_a3_tee", replace(pkg, checks=(R.Check(("-c", "raise SystemExit(3)")),)))
    problems = R.check_problems("rt_a3_tee", 0, tmp_path / "check_output")
    assert problems and "exited 3" in problems[0]
