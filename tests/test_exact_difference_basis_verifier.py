"""The policy verifier (`tests/verifiers/exact_difference_basis.py`) against the successor package
`validation/references/exact_difference_basis_completed/`.

The verifier was written from the policy text, separately from the package's generator. Its verdict must equal
every row's `admission.verdict`, and each of six deliberately wrong policies must reverse at least one row, so that
the rows are shown to bind each part of the policy they are meant to bind.
"""
import ast
import json
import sys
from pathlib import Path

import pytest

from tests.verifiers import exact_difference_basis as policy

PROJECT = Path(__file__).resolve().parents[1]
PACKAGE = PROJECT / "validation" / "references" / "exact_difference_basis_completed"
VERIFIER = PROJECT / "tests" / "verifiers" / "exact_difference_basis.py"


def all_rows():
    data = json.loads((PACKAGE / "references.json").read_text(encoding="utf-8"))
    return (data["carried"]["gate_rows"] + data["carried"]["pairs"] + data["added"]
            + data["builder_witnesses"])


ROWS = all_rows()


def verdicts():
    return {row["id"]: policy.admission(row["sustained"], row["operating"])["verdict"] for row in ROWS}


def test_rows_are_all_read():
    assert len(ROWS) == 19 + 17 + 15 + 2
    assert len({row["id"] for row in ROWS}) == len(ROWS)


@pytest.mark.parametrize("row", ROWS, ids=[row["id"] for row in ROWS])
def test_verifier_verdict_equals_the_package(row):
    found = policy.admission(row["sustained"], row["operating"])
    expected = row["admission"]
    assert found["verdict"] == expected["verdict"]
    assert found["gates"] == {part: expected["gates"][part]["holds"] for part in ("restraint", "g1", "g2")}


@pytest.mark.parametrize("row", ROWS, ids=[row["id"] for row in ROWS])
def test_both_stiffness_readings_equal_the_recorded_members(row):
    recorded = row["stiffness_readings"]
    found = {reading: policy.admission(row["sustained"], row["operating"], reading) for reading in ("S", "R")}
    for reading in ("S", "R"):
        assert found[reading]["verdict"] == recorded[reading]["verdict"], reading
        assert found[reading]["gates"]["g2"] == recorded[reading]["g2_holds"], reading
    assert recorded["S"]["verdict"] == row["admission"]["verdict"]
    assert recorded["differ"] == (found["S"]["verdict"] != found["R"]["verdict"])


def _drop_factor(original):
    return lambda record, factor_bits: original(record, "dropped")


def _drop_extent(original):
    def key(record):
        return original(dict(record, extent=None))
    return key


def _drop_terminals(original):
    def key(region):
        return original(dict(region, terminals=[]))
    return key


def _zero_motion_is_action(original):
    return lambda record: True


def _free_is_fixed(original):
    def law(record):
        return original(dict(record, law="rigid")) if record["law"] == "free" else original(record)
    return law


def _reference_configuration_compared(original):
    return lambda record: original(record) + (record.get("reference_configuration"),)


def _selection_dropped_from_reading_s(original):
    return {"S": original["R"], "R": original["R"]}


MUTANTS = {
    "the factor dropped from the primitive key": ("primitive_key", _drop_factor),
    "extent dropped from the generated key": ("generated_key", _drop_extent),
    "terminals dropped from the region key": ("region_key", _drop_terminals),
    "zero motion treated as an action": ("motion_is_action", _zero_motion_is_action),
    "a free DOF treated as fixed": ("restraint_law", _free_is_fixed),
    "the reference-configuration id added to the member key": ("state_key", _reference_configuration_compared),
    "the selection kind, label and record dropped from reading S": ("READINGS", _selection_dropped_from_reading_s),
}


@pytest.mark.parametrize("name", list(MUTANTS))
def test_each_mutant_reverses_at_least_one_row(name, monkeypatch):
    attribute, make = MUTANTS[name]
    expected = {row["id"]: row["admission"]["verdict"] for row in ROWS}
    assert verdicts() == expected
    monkeypatch.setattr(policy, attribute, make(getattr(policy, attribute)))
    flipped = sorted(rid for rid, verdict in verdicts().items() if verdict != expected[rid])
    print(f"{name}: reverses {flipped}")
    assert flipped, f"no row binds against the mutant: {name}"


def test_verifier_imports_only_the_standard_library_and_not_the_generator():
    tree = ast.parse(VERIFIER.read_text(encoding="utf-8"))
    modules = set()
    for node in ast.walk(tree):
        if isinstance(node, ast.Import):
            modules |= {alias.name.split(".")[0] for alias in node.names}
        elif isinstance(node, ast.ImportFrom):
            assert node.level == 0, "relative import"
            modules.add(node.module.split(".")[0])
        elif isinstance(node, ast.Constant) and isinstance(node.value, str):
            assert "exact_difference_basis_completed_reference" not in node.value
    assert modules <= set(sys.stdlib_module_names), sorted(modules - set(sys.stdlib_module_names))
