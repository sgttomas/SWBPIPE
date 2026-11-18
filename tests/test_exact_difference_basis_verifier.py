"""The policy verifier (`tests/verifiers/exact_difference_basis.py`) against the successor package
`validation/references/exact_difference_basis_completed/`.

The verifier was written from the policy text, separately from the package's generator. The test compares, on
every row, the verifier's verdict, gate outcomes and gate causes with the recorded ones and both stiffness
readings with the recorded readings; it checks focused policy cases on synthetic operands, each with the Part A
specification section it rests on; and it requires each of eight specified mutants to change a verdict.

Recorded prose is bound to the operands for one row only: X5's admission reason and its G-1 `why` must be the texts
stated here, and the operand facts they rest on (a shared target, a seismic recipe in S, mass bases that differ
only in contents mass per length) must hold. No other row's `reason` or `why` is checked here; for those rows the
verdict, gate outcomes and causes are compared, not their prose.
"""
import ast
import copy
import json
import math
import struct
import sys
from fractions import Fraction
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
    assert len(ROWS) == 19 + 17 + 16 + 6
    assert len({row["id"] for row in ROWS}) == len(ROWS)


PARTS = ("restraint", "g1", "g2")


def disagreements(row):
    """What the verifier finds that differs from the recorded admission: verdict, gate outcomes, gate causes."""
    found = policy.admission(row["sustained"], row["operating"])
    recorded = row["admission"]
    out = []
    if found["verdict"] != recorded["verdict"]:
        out.append("verdict")
    if found["gates"] != {part: recorded["gates"][part]["holds"] for part in PARTS}:
        out.append("gates")
    if found["causes"] != {part: recorded["gates"][part]["cause"] for part in PARTS}:
        out.append("causes")
    return out


@pytest.mark.parametrize("row", ROWS, ids=[row["id"] for row in ROWS])
def test_verifier_verdict_gates_and_causes_equal_the_package(row):
    assert disagreements(row) == []


def test_a_changed_cause_is_found_with_operands_and_gate_outcomes_intact():
    row = copy.deepcopy(next(r for r in ROWS if r["id"] == "X5"))
    assert disagreements(row) == []
    assert row["admission"]["gates"]["g1"]["cause"] == ["generated_recipe"]
    row["admission"]["gates"]["g1"]["cause"] = ["primitive"]       # a false cause; operands and booleans unchanged
    assert disagreements(row) == ["causes"]


@pytest.mark.parametrize("row", ROWS, ids=[row["id"] for row in ROWS])
def test_both_stiffness_readings_equal_the_recorded_members(row):
    recorded = row["stiffness_readings"]
    found = {reading: policy.admission(row["sustained"], row["operating"], reading) for reading in ("S", "R")}
    for reading in ("S", "R"):
        assert found[reading]["verdict"] == recorded[reading]["verdict"], reading
        assert found[reading]["gates"]["g2"] == recorded[reading]["g2_holds"], reading
    assert recorded["S"]["verdict"] == row["admission"]["verdict"]
    assert recorded["differ"] == (found["S"]["verdict"] != found["R"]["verdict"])


# ---- focused policy cases on synthetic operands -------------------------------------------------------------
# Each starts from a recorded row's operands, changes what its name says, and states the expected verdict under
# both stiffness readings and the expected causes under S. The section is the Part A specification's.

def _row(rid):
    found = next(r for r in ROWS if r["id"] == rid)
    return copy.deepcopy(found["sustained"]), copy.deepcopy(found["operating"])


def _support(side, support):
    return next(r for r in side["supports"] if r["support"] == support)


def _bits(x: float) -> str:
    return struct.pack(">d", x).hex()


def _rigid_participation_held():
    s, o = _row("W3")
    _support(o, "line_stop:B")["participation"] = "held"
    return s, o


def _rigid_parameters_changed():
    s, o = _row("W3")
    _support(o, "line_stop:B")["parameters"] = {"gap": "1/1024"}
    return s, o


def _quantity_written_otherwise():
    s, o = _row("W1")
    o["basis"]["AB"]["E"] = "2097152/2"
    o["basis"]["AB"]["selection_record"]["selected_E"] = "2097152/2"
    return s, o


def _label_equal_as_numbers():
    s, o = _row("W1")
    s["basis"]["AB"]["case_basis_label"] = "01"
    o["basis"]["AB"]["case_basis_label"] = "1"
    return s, o


def _fixed_inventories_unequal():
    s, o = _row("constant_effort_preload_in_both")
    o["fixed_loads"] = []
    return s, o


def _declared_free_against_absent():
    s, o = _row("W1")
    s["supports"].append({"support": "line_stop:B", "dof": "UX", "law": "free", "identity": "line_stop:B",
                          "participation": None, "parameters": {}})
    return s, o


def _positive_zero_against_absent():
    s, o = _row("W9")
    s["motions"][0]["value_bits"] = _bits(0.0)
    return s, o


def _spring_participation_held():
    s, o = _row("W4")
    _support(o, "spring:B")["participation"] = "held"
    return s, o


def _spring_stiffness_spelled(s_text, o_text):
    def make():
        s, o = _row("W4")
        bits = _support(s, "spring:B")["parameters"]["stiffness_bits"]
        _support(s, "spring:B")["parameters"] = {"stiffness": s_text, "stiffness_bits": bits}
        _support(o, "spring:B")["parameters"] = {"stiffness": o_text, "stiffness_bits": bits}
        return s, o
    return make


def _spring_stiffness_changed():
    s, o = _row("W4")
    _support(o, "spring:B")["parameters"] = {"stiffness": "3584*pi", "stiffness_bits": _bits(3584 * math.pi)}
    return s, o


def _selection_fraction_alone():
    s, o = _row("K1")
    for member in o["basis"]:
        o["basis"][member] = copy.deepcopy(s["basis"][member])
        o["basis"][member]["selection_record"]["interpolation_fraction"] = "0"
    return s, o


def _restraint_and_g1_fail():
    s, o = _row("X3")
    s["primitives"].append(copy.deepcopy(_row("W6")[0]["primitives"][0]))
    return s, o


def _equal_formed_primitive_values():
    s, o = _row("X1")
    s["primitives"] = [dict(s["primitives"][0], value="1024", value_bits=_bits(1024.0))]
    s["ledger"] = [{"primitive": "f:B:x", "factor": "2", "factor_bits": _bits(2.0)}]
    o["primitives"] = [dict(o["primitives"][0], value="2048", value_bits=_bits(2048.0))]
    o["ledger"] = [{"primitive": "f:B:x", "factor": "1", "factor_bits": _bits(1.0)}]
    return s, o


def _natural_length_reference_length():
    s, o = _row("X2")
    s["member_states"][0]["fit"] = {"kind": "natural_length_change", "change": "1/1024", "reference_length": "1"}
    o["member_states"][0]["fit"] = {"kind": "natural_length_change", "change": "1/1024", "reference_length": "2"}
    return s, o


def _seismic_mass_method():
    s, o = _row("X5")
    o["generated"][0]["mass_basis"] = dict(s["generated"][0]["mass_basis"], method="consistent_member_mass")
    return s, o


def _whole_span_extent():
    s, o = _row("X5")
    o["generated"][0] = dict(copy.deepcopy(s["generated"][0]), extent=["0", "1"])
    s["generated"][0].pop("extent", None)
    return s, o


FOCUSED = {
    # name: (Part A section, operands, verdict under S, verdict under R, causes under S)
    "rigid participation alone": ("3.4", _rigid_participation_held, "refused_restraint", "refused_restraint",
                                  {"restraint": ["support_law:line_stop:B/UX"], "g1": [], "g2": []}),
    "rigid parameters alone": ("3.4", _rigid_parameters_changed, "refused_restraint", "refused_restraint",
                               {"restraint": ["support_law:line_stop:B/UX"], "g1": [], "g2": []}),
    "a modulus written as another rational": ("3.6 and 4", _quantity_written_otherwise, "admitted", "admitted",
                                              {"restraint": [], "g1": [], "g2": []}),
    "case basis labels equal as numbers, different as text": (
        "3.6", _label_equal_as_numbers, "refused_g2", "admitted",
        {"restraint": [], "g1": [], "g2": ["AB:case_basis_label"]}),
    "unequal fixed-load inventories": ("3.7", _fixed_inventories_unequal, "refused_restraint", "refused_restraint",
                                       {"restraint": ["fixed_load_inventory"], "g1": [], "g2": []}),
    "a declared free DOF against an absent one": ("3.4", _declared_free_against_absent, "admitted", "admitted",
                                                  {"restraint": [], "g1": [], "g2": []}),
    "a positive zero motion against none": ("3.4", _positive_zero_against_absent, "admitted", "admitted",
                                            {"restraint": [], "g1": [], "g2": []}),
    "spring participation alone": ("3.4", _spring_participation_held, "refused_restraint", "refused_restraint",
                                   {"restraint": ["support_law:spring:B/UX"], "g1": [], "g2": []}),
    "spring stiffness alone": ("3.4", _spring_stiffness_changed, "refused_restraint", "refused_restraint",
                               {"restraint": ["support_law:spring:B/UX"], "g1": [], "g2": []}),
    "equal spring stiffness written 1024 and 2048/2": (
        "3.4", _spring_stiffness_spelled("1024", "2048/2"), "admitted", "admitted",
        {"restraint": [], "g1": [], "g2": []}),
    "equal spring stiffness written 7168*pi and 14336/2*pi": (
        "3.4", _spring_stiffness_spelled("7168*pi", "14336/2*pi"), "admitted", "admitted",
        {"restraint": [], "g1": [], "g2": []}),
    "the selection record's interpolation fraction alone": (
        "3.6", _selection_fraction_alone, "refused_g2", "admitted",
        {"restraint": [], "g1": [], "g2": ["AB:selection_record", "BC:selection_record"]}),
    "restraint and G-1 both fail": ("2 and 3", _restraint_and_g1_fail, "refused_restraint", "refused_restraint",
                                    {"restraint": ["support_law:line_stop:B/UX"], "g1": ["primitive"], "g2": []}),
    "equal formed values from 1024 N at factor 2 and 2048 N at factor 1": (
        "3.1", _equal_formed_primitive_values, "refused_g1", "refused_g1",
        {"restraint": [], "g1": ["primitive"], "g2": []}),
    "natural-length changes with reference lengths 1 m and 2 m": (
        "3.5", _natural_length_reference_length, "refused_g1", "refused_g1",
        {"restraint": [], "g1": ["member_state"], "g2": []}),
    "seismic recipes that differ only in mass method": ("3.3", _seismic_mass_method, "refused_g1", "refused_g1",
                                                        {"restraint": [], "g1": ["generated_recipe"], "g2": []}),
    "an absent extent against [0, 1]": ("3.3", _whole_span_extent, "admitted", "admitted",
                                        {"restraint": [], "g1": [], "g2": []}),
}


@pytest.mark.parametrize("name", list(FOCUSED))
def test_focused_policy_case(name):
    _section, make, under_s, under_r, causes = FOCUSED[name]
    s, o = make()
    found_s = policy.admission(s, o, "S")
    assert found_s["verdict"] == under_s
    assert found_s["causes"] == causes
    assert found_s["gates"] == {part: not causes[part] for part in PARTS}
    assert policy.admission(s, o, "R")["verdict"] == under_r


# ---- recorded prose bound to the operands (X5 only) ---------------------------------------------------------

X5_REASON = "the recipes' mass bases differ, so the subtrahend's seismic recipe is not in the minuend"
X5_G1_WHY = "The subtrahend's generated recipe is absent from the minuend or occurs there too few times."


def x5_prose_is_bound(row) -> list:
    """What is false in X5's recorded prose or in the operand facts it rests on; empty when all holds."""
    (s_recipe,), (o_recipe,) = row["sustained"]["generated"], row["operating"]["generated"]
    facts = {
        "the reason as stated": row["admission"]["reason"] == X5_REASON,
        "the G-1 why as stated": row["admission"]["gates"]["g1"]["why"] == X5_G1_WHY,
        "S's recipe is seismic": s_recipe["kind"] == "seismic",
        "the target member is shared": s_recipe["target"] == o_recipe["target"] == "AB",
        "the inputs are equal": s_recipe["inputs"] == o_recipe["inputs"],
        "the contents mass per length differs": (Fraction(s_recipe["mass_basis"]["contents_mass_per_length"])
                                                 != Fraction(o_recipe["mass_basis"]["contents_mass_per_length"])),
        "nothing else in the mass bases differs": (
            {k: v for k, v in s_recipe["mass_basis"].items() if k != "contents_mass_per_length"}
            == {k: v for k, v in o_recipe["mass_basis"].items() if k != "contents_mass_per_length"}),
    }
    return [name for name, holds in facts.items() if not holds]


def test_x5_prose_is_bound_to_its_operands():
    assert x5_prose_is_bound(next(r for r in ROWS if r["id"] == "X5")) == []


def test_a_false_x5_explanation_is_found_with_the_cause_held_fixed():
    row = copy.deepcopy(next(r for r in ROWS if r["id"] == "X5"))
    row["admission"]["reason"] = "the target member differs, so the subtrahend's seismic recipe is not in the minuend"
    row["admission"]["gates"]["g1"]["why"] = "The subtrahend's recipe targets another member than the minuend's."
    assert disagreements(row) == []                    # verdict, gate outcomes and causes are all unchanged
    assert x5_prose_is_bound(row) == ["the reason as stated", "the G-1 why as stated"]


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
    """An explicitly free DOF read as the canonical active rigid law at the same support identity, so that the
    reversal isolates free against fixed and no other rigid field."""
    def law(record):
        if record["law"] == "free":
            return original(dict(record, law="rigid", participation="active", parameters={}))
        return original(record)
    return law


def _reference_configuration_compared(original):
    return lambda record: original(record) + (record.get("reference_configuration"),)


def _mass_basis_ignored(original):
    def key(record):
        return original(dict(record, mass_basis={}))
    return key


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
    "the mass basis ignored in the generated key": ("generated_key", _mass_basis_ignored),
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
