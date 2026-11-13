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
    for name in ("kd5_timoshenko", "pf_timoshenko_m3t", "m3_judge"):
        assert PACKAGES[name].checks and PACKAGES[name].deferred, name
    assert not any(p.startswith(("kd5_timoshenko", "pf_timoshenko_m3t", "m3_judge")) for p in R.classification_problems())


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
