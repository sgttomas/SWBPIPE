"""CONSISTENCY of every reference package under validation/references/: pure reading, no
generator runs. From the one table in tests/reference_packages.py, this test checks that:

1. every package directory (or package inside a declared container) has an entry, and every file of a
   container that is in no package is named with its reason;
2. the checksum lists a package holds are exactly the lists its entry declares, and every line of every
   verified list matches the bytes it names, each against the base directory the entry records (a line
   known to fail is a named exception that fails once the line passes again);
3. MEMBERSHIP: every file under a package is named by a line of a verified list or is unlisted for a
   stated reason, on the basis the entry states (the generator's own file set, the README's file table,
   or a fixed set with its reason); a digest the package states for an unlisted file is verified; what
   has no stated basis is named and counted (MEMBERSHIP_ON_TRUST_COUNT), never passed silently;
4. every package has a retained check command, a deferred command or a regeneration gap, and every
   reason the table gives for what is not regenerated is stated; a known defect is still there.

Regeneration is tests/test_reference_packages_regeneration.py, kept apart so that this reading, which takes
well under a second, runs whenever a package file changes. Neither test checks that a package's generators
are independent of product code.
"""
from __future__ import annotations

import sys
from dataclasses import replace

import pytest

import reference_packages as R
from reference_packages import PACKAGES


def test_every_package_is_classified():
    problems = R.classification_problems()
    assert not problems, "\n".join(problems)


@pytest.mark.parametrize("name", sorted(PACKAGES))
def test_checksums(name):
    problems = R.checksum_problems(name)
    assert not problems, f"{name}:\n" + "\n".join(problems)


@pytest.mark.parametrize("name", sorted(PACKAGES))
def test_membership(name):
    problems = R.membership_problems(name)
    assert not problems, f"{name}:\n" + "\n".join(problems)


def test_membership_taken_on_trust_is_counted():
    rows = R.membership_on_trust()
    print("\nMembership with no stated basis (named, counted, not passed):")
    for row in rows:
        print("  " + " | ".join(row))
    unpinned = [(name, rel) for name, pkg in sorted(PACKAGES.items())
                for rel in sorted(pkg.members.unlisted) if rel not in pkg.members.stated]
    print(f"Unlisted files no list or stated digest pins ({len(unpinned)}):")
    for name, rel in unpinned:
        print(f"  {name}/{rel}")
    assert len(rows) == R.MEMBERSHIP_ON_TRUST_COUNT, (
        f"{len(rows)} memberships have no stated basis, MEMBERSHIP_ON_TRUST_COUNT says {R.MEMBERSHIP_ON_TRUST_COUNT}; "
        "a change to that list is a decision, so change the count with it")


def test_every_reason_for_what_is_not_regenerated_is_stated():
    for name, what, why in R.taken_on_trust():
        assert why and why.strip(), f"{name}: {what} has no reason"


@pytest.mark.parametrize("name", sorted(n for n, p in PACKAGES.items() if p.defects))
def test_a_known_defect_is_still_there(name):
    problems = R.defect_problems(name)
    assert not problems, f"{name}:\n" + "\n".join(problems)


def test_the_time_limit_is_set_from_the_slowest_observed_time():
    """One limit for every command: the margin over the slowest command observed, rounded up to 10 s."""
    observed, where = R.SLOWEST_OBSERVED
    assert where.strip() and R.CHECK_TIMEOUT_S % 10 == 0
    assert R.LIMIT_MARGIN * observed <= R.CHECK_TIMEOUT_S < R.LIMIT_MARGIN * observed + 10


def test_a_command_over_its_limit_fails_its_check(monkeypatch, tmp_path):
    """A hung command fails its test at its limit, by name, and does not hang the job."""
    pkg = PACKAGES["rt_a3_tee"]
    monkeypatch.setitem(PACKAGES, "rt_a3_tee", replace(pkg, checks=(replace(
        pkg.checks[0], argv=("-c", "import time; time.sleep(30)")),)))
    monkeypatch.setattr(R, "CHECK_TIMEOUT_S", 1)
    problems = R.check_problems("rt_a3_tee", 0, tmp_path / "out")
    assert len(problems) == 1 and "did not finish within its 1 s limit" in problems[0]


def _printing(pkg, text, platforms):
    """pkg's first check replaced by one printing `text`, compared with the package's README only on `platforms`."""
    return replace(pkg, checks=(replace(pkg.checks[0], argv=("-c", f"print({text!r}, end='')"),
                                        stdout_equals="README.md", stdout_platforms=platforms, stdout_gap="test"),))


def test_a_stdout_comparison_made_here_fails_with_what_differs(monkeypatch, tmp_path):
    monkeypatch.setitem(PACKAGES, "rt_a3_tee", _printing(PACKAGES["rt_a3_tee"], "case x\nargs y\n", (sys.platform,)))
    problems = R.check_problems("rt_a3_tee", 0, tmp_path / "out")
    assert len(problems) == 1 and "stdout differs from README.md" in problems[0] and "frozen lines differ" in problems[0]


def test_where_the_comparison_is_not_enforced_reproduction_is_not_established_and_what_differs_is_reported(
        monkeypatch, tmp_path):
    monkeypatch.setitem(PACKAGES, "rt_a3_tee", _printing(PACKAGES["rt_a3_tee"], "case x\nargs y\n", ("no-platform",)))
    with pytest.warns(R.StdoutNotCompared, match=r"(?s)reproduction of the committed file is not established on "
                                                 r"this platform.*differed from the committed file here:.*lines differ"):
        assert R.check_problems("rt_a3_tee", 0, tmp_path / "out") == []


def test_the_platform_exception_is_the_one_named_command_and_file():
    limited = [(n, c.argv[0], c.stdout_equals) for n, p in PACKAGES.items() for c in p.checks if c.stdout_platforms]
    assert limited == list(R.PLATFORM_LIMITED_STDOUT) == [("t4_i9", "t4_i17_arc_refs.py", "arc_load_references.txt")]
    assert PACKAGES["t4_i9"].checks[0].stdout_platforms == ("darwin",)


def test_a_platform_limited_comparison_on_another_check_fails_classification(monkeypatch):
    pkg = PACKAGES["t4_i9"]
    monkeypatch.setitem(PACKAGES, "t4_i9", replace(pkg, checks=(pkg.checks[0], replace(
        pkg.checks[1], stdout_platforms=("darwin",), stdout_gap="test"), pkg.checks[2])))
    assert any("is not the exception PLATFORM_LIMITED_STDOUT names" in p for p in R.classification_problems())


def test_the_stdout_report_counts_cases_with_identical_args_and_differing_refs():
    frozen = (b"case a\nphi 1\nargs 1 2\nref 1\nref 2\nend\n"
              b"case b\nphi 2\nargs 3 4\nref 3\nref 4\nend\n"
              b"case c\nphi 3\nargs 5 6\nref 5\nref 6\nend\n")
    printed = (b"case a\nphi 1\nargs 1 9\nref 1\nref 8\nend\n"   # inputs differ, so do refs
               b"case b\nphi 2\nargs 3 4\nref 3\nref 7\nend\n"   # inputs identical, refs differ: deciding
               b"case c\nphi 3\nargs 5 6\nref 5\nref 6\nend\n")  # identical
    report = R.stdout_report(frozen, printed)
    assert report.splitlines()[0] == "3 of 18 frozen lines differ (18 printed): args 1, ref 2"
    assert "first args at line 3: frozen 'args 1 2'; printed 'args 1 9'" in report
    assert ("cases: 3 committed, 3 printed; args identical in 2; cases with identical args and differing refs: 1"
            in report)
    assert "  case a: args differ; refs differ (1 of 2)" in report
    assert "  case b: args identical; refs differ (1 of 2)" in report
    assert "  case c: args identical; refs identical" in report


def test_a_platform_limited_comparison_must_say_what_was_observed(monkeypatch):
    pkg = PACKAGES["t4_i9"]
    monkeypatch.setitem(PACKAGES, "t4_i9", replace(pkg, checks=(replace(pkg.checks[0], stdout_gap=None),
                                                                  *pkg.checks[1:])))
    assert any(p.startswith("t4_i9:") and "only on named platforms" in p for p in R.classification_problems())


def test_a_package_with_no_check_command_and_no_gap_fails_classification(monkeypatch):
    """The reader's mutation: emptying one package's check commands must not make its regeneration vanish."""
    monkeypatch.setitem(PACKAGES, "rt_a3_tee", replace(PACKAGES["rt_a3_tee"], checks=()))
    assert any(p.startswith("rt_a3_tee: no check command, no deferred command and no regeneration gap")
               for p in R.classification_problems())


def test_a_regeneration_gap_is_counted():
    gaps = [(name, pkg.regeneration_gap) for name, pkg in sorted(PACKAGES.items()) if not pkg.checks and not pkg.deferred]
    assert gaps and all(why for _, why in gaps)
    assert all((name, R.REGENERATION_GAP, why) in R.taken_on_trust() for name, why in gaps)


# ------------------------------------------------------------------------------------- deferred commands

def test_each_deferred_command_is_printed_and_counted():
    rows = [(name, d) for name, pkg in sorted(PACKAGES.items()) for d in pkg.deferred]
    print("\nDeferred commands (not run; counted in TAKEN_ON_TRUST_COUNT):")
    for name, d in rows:
        print(f"  {name}: {R.deferred_text(d)}")
        assert (name, R.deferred_text(d), "deferred, not run") in R.taken_on_trust()
        assert R.NOT_ESTABLISHED in R.deferred_text(d)
    assert rows


def test_a_package_may_carry_retained_checks_beside_a_deferred_command():
    names = ("kd5_timoshenko", "pf_timoshenko_m3t", "m3_judge")
    for name in names:
        assert PACKAGES[name].checks and PACKAGES[name].deferred, name
    assert not any(p.startswith(names) for p in R.classification_problems())


@pytest.mark.parametrize("change, expected", [
    ({"observed_at": "feffd237f"}, "names no blocker or no full SHA"),         # not a full SHA
    ({"blocker": " "}, "names no blocker or no full SHA"),
    ({"argv": ("kd5t_engine.py", "--check")}, "is both a check and deferred"),
])
def test_a_malformed_deferred_entry_fails_classification(monkeypatch, change, expected):
    pkg = PACKAGES["kd5_timoshenko"]
    monkeypatch.setitem(PACKAGES, "kd5_timoshenko", replace(pkg, deferred=(replace(pkg.deferred[0], **change),)))
    assert any(expected in p for p in R.classification_problems())


def test_a_regeneration_gap_beside_a_deferred_command_fails_classification(monkeypatch):
    pkg = PACKAGES["m3_judge"]
    monkeypatch.setitem(PACKAGES, "m3_judge", replace(pkg, checks=(), regeneration_gap="a reason"))
    assert any(p.startswith("m3_judge: a regeneration gap beside") for p in R.classification_problems())
