"""Successor exact-route difference-basis references (`validation/references/exact_difference_basis_completed/`).

Regeneration checks committed bytes. The generator checks carried equilibrium, selected pressure and flexibility
identities, added verdicts and selected mechanical values.

This test imports neither the generator nor the policy verifier. It also reads the frozen package
(`../exact_difference_basis/references.json`, which stays byte for byte) and holds the successor to it: each
carried row keeps its frozen verdict under the stated mapping, and each carried mechanical value equals the frozen
value. Any reversal fails here.
"""
import ast
import hashlib
import json
import re
import subprocess
import sys
from fractions import Fraction
from pathlib import Path

import pytest

PROJECT = Path(__file__).resolve().parents[1]
REFERENCES = PROJECT / "validation" / "references"
PACKAGE = REFERENCES / "exact_difference_basis_completed"
FROZEN = REFERENCES / "exact_difference_basis"
GENERATOR = PACKAGE / "generators" / "exact_difference_basis_completed_reference.py"
PRODUCT_ROOTS = {"core", "apps", "tests", "tools", "fixtures", "schemas"}
FORBIDDEN_CALLS = {"__import__", "import_module", "exec", "eval", "spec_from_file_location", "open"}
FROZEN_TO_COMPLETED = {"published": "admitted", "refused_g1": "refused_g1", "refused_g2": "refused_g2",
                       "refused_g1_and_g2": "refused_g1"}
VERDICTS = {"admitted", "refused_restraint", "refused_g1", "refused_g2"}
KINDS = {"linear_increment", "state_difference", "none"}


def load(path):
    return json.loads(path.read_text(encoding="utf-8"))


def test_generator_check_rebuilds_the_committed_references():
    result = subprocess.run([sys.executable, "-I", "-B", str(GENERATOR)], cwd=GENERATOR.parent,
                            capture_output=True, text=True, timeout=600)
    assert result.returncode == 0, result.stdout + result.stderr


def test_sha256sums_verify():
    lines = (PACKAGE / "SHA256SUMS").read_text().splitlines()
    assert {line.split("  ", 1)[1] for line in lines} == {
        "README.md", "generators/exact_difference_basis_completed_reference.py", "references.json"}
    for line in lines:
        digest, name = line.split("  ", 1)
        assert hashlib.sha256((PACKAGE / name).read_bytes()).hexdigest() == digest, name


# ---- the generator's stiffness text grammar -----------------------------------------------------------------
# The generator's own reader of an exact value (exact_from_text) is exercised in a child process that imports the
# generator and nothing of the verifier. A text outside the grammar must raise ValueError there, at the parser and
# at the generator's own comparison; it must not be read as a value. The first three below were once read. The
# probe reports any other error by its type, so that it fails the test and not the probe.

MALFORMED_STIFFNESS = [
    "7168*pi1", "7168*pi2", "7168*piJUNK", "7168*pi^", "7168*pi^2x", "7168*pi*pi",
    "pi", "*pi", "7168pi", "7168*PI", "7168 *pi", " 7168*pi", "7168*pi ", "7168*pi\n", "",
    "7168*pi^+2", "7168*pi^-", "7168*pi^2.0", "7168*pi+1", "7168*pi + ", "7168*pi  +  1",
    "+7168*pi", "--7168*pi", "1.5*pi", "1e3", "1_000", "1/0", "1/0*pi", "1/-2", "1/2/3", "\uff17*pi",
]
WELL_FORMED_STIFFNESS = {
    "7168*pi": {"1": "7168"}, "14336/2*pi": {"1": "7168"}, "3584*pi": {"1": "3584"},
    "1024": {"0": "1024"}, "2048/2": {"0": "1024"}, "0": {}, "0*pi": {},
    "7*pi^1": {"1": "7"}, "7*pi^0": {"0": "7"}, "-3/4*pi^2": {"2": "-3/4"},
    "3/4*pi + 1/2 + 5/7*pi^-1": {"1": "3/4", "0": "1/2", "-1": "5/7"},
    "1*pi + 2*pi": {"1": "3"},
}
RECORDED_STIFFNESS = {"7168*pi": {"1": "7168"}}
# A run of digits (numerator, denominator or exponent) holds at most 1,000 digits, leading zeros counted. The bound
# is the grammar's own: at 1,000 a text is read with its value, at 1,001 it is refused, in each position.
NINES = "9" * 1000
AT_THE_DIGIT_BOUND = {
    "numerator": (NINES, {"0": NINES}),
    "negative numerator": ("-" + NINES + "*pi", {"1": "-" + NINES}),
    "numerator with leading zeros": ("0" * 999 + "7", {"0": "7"}),
    "denominator": ("1/1" + "0" * 999, {"0": "1/1" + "0" * 999}),
    "denominator with leading zeros": ("1/" + "0" * 999 + "2*pi", {"1": "1/2"}),
    "exponent": ("3*pi^" + NINES, {NINES: "3"}),
    "negative exponent": ("3*pi^-" + NINES, {"-" + NINES: "3"}),
    "exponent with leading zeros": ("3*pi^" + "0" * 999 + "2", {"2": "3"}),
}
PAST_THE_DIGIT_BOUND = {
    "numerator": NINES + "9",
    "negative numerator": "-" + NINES + "9*pi",
    "numerator with leading zeros": "0" * 1000 + "7",
    "denominator": "1/1" + "0" * 1000,
    "denominator with leading zeros": "1/" + "0" * 1000 + "2*pi",
    "exponent": "3*pi^" + NINES + "9",
    "negative exponent": "3*pi^-" + NINES + "9",
    "exponent with leading zeros": "3*pi^" + "0" * 1000 + "2",
}
GENERATOR_PROBE = """
import copy, json, sys
sys.path.insert(0, sys.argv[1])
import exact_difference_basis_completed_reference as generator
request = json.loads(sys.stdin.read())

def parsed(text):
    try:
        return {str(k): str(v) for k, v in generator.exact_from_text(text).c.items()}
    except Exception as error:
        return type(error).__name__

def compared(text, side):
    s, o = copy.deepcopy(request["w4"]["sustained"]), copy.deepcopy(request["w4"]["operating"])
    for support in (s if side == "sustained" else o)["supports"]:
        if support["support"] == "spring:B":
            support["parameters"]["stiffness"] = text
    try:
        return {reading: generator.verdict_under(s, o, reading)[0] for reading in ("S", "R")}
    except Exception as error:
        return type(error).__name__

print(json.dumps({
    "parsed": {text: parsed(text) for text in request["texts"]},
    "compared": {text: {side: compared(text, side) for side in ("sustained", "operating")}
                 for text in request["texts"]},
}))
"""


def _stiffness_texts(value, field=None) -> set:
    if isinstance(value, dict):
        return set().union(*(_stiffness_texts(v, k) for k, v in value.items())) if value else set()
    if isinstance(value, list):
        return set().union(*(_stiffness_texts(v, field) for v in value)) if value else set()
    return {value} if field == "stiffness" else set()


@pytest.fixture(scope="module")
def generator_probe():
    data = load(PACKAGE / "references.json")
    w4 = next(row for row in data["carried"]["gate_rows"] + data["carried"]["pairs"] + data["added"]
              + data["builder_witnesses"] if row["id"] == "W4")
    request = {"texts": (MALFORMED_STIFFNESS + list(WELL_FORMED_STIFFNESS)
                         + [text for text, _ in AT_THE_DIGIT_BOUND.values()] + list(PAST_THE_DIGIT_BOUND.values())),
               "w4": {"sustained": w4["sustained"], "operating": w4["operating"]}}
    result = subprocess.run([sys.executable, "-I", "-B", "-c", GENERATOR_PROBE, str(GENERATOR.parent)],
                            input=json.dumps(request), capture_output=True, text=True, timeout=600)
    assert result.returncode == 0, result.stdout + result.stderr
    return json.loads(result.stdout)


@pytest.mark.parametrize("text", MALFORMED_STIFFNESS)
def test_the_generator_refuses_a_malformed_stiffness_text(generator_probe, text):
    assert generator_probe["parsed"][text] == "ValueError"
    assert generator_probe["compared"][text] == {"sustained": "ValueError", "operating": "ValueError"}


@pytest.mark.parametrize("text", list(WELL_FORMED_STIFFNESS))
def test_the_generator_reads_a_well_formed_stiffness_text_as_its_terms(generator_probe, text):
    assert generator_probe["parsed"][text] == WELL_FORMED_STIFFNESS[text]


@pytest.mark.parametrize("position", list(AT_THE_DIGIT_BOUND))
def test_the_generator_reads_a_run_of_1000_digits_with_its_value(generator_probe, position):
    text, terms = AT_THE_DIGIT_BOUND[position]
    assert generator_probe["parsed"][text] == terms


@pytest.mark.parametrize("position", list(PAST_THE_DIGIT_BOUND))
def test_the_generator_refuses_a_run_of_1001_digits(generator_probe, position):
    text = PAST_THE_DIGIT_BOUND[position]
    assert generator_probe["parsed"][text] == "ValueError"
    assert generator_probe["compared"][text] == {"sustained": "ValueError", "operating": "ValueError"}


def test_the_generator_admits_w4_with_its_stiffness_written_differently(generator_probe):
    """The control for the refusals above: the probe's comparison does return verdicts for texts it can read."""
    assert generator_probe["compared"]["14336/2*pi"] == {
        "sustained": {"S": "admitted", "R": "admitted"}, "operating": {"S": "admitted", "R": "admitted"}}
    assert generator_probe["compared"]["3584*pi"]["operating"] == {
        "S": "refused_restraint", "R": "refused_restraint"}


def test_every_recorded_stiffness_text_is_read_by_the_generator_as_its_recorded_value(generator_probe):
    assert _stiffness_texts(load(PACKAGE / "references.json")) == set(RECORDED_STIFFNESS)
    for text, terms in RECORDED_STIFFNESS.items():
        assert generator_probe["parsed"][text] == terms


def names_a_product_path(text: str) -> bool:
    if not text or re.search(r"\s", text):
        return False
    return any(segment in PRODUCT_ROOTS for segment in re.split(r"[\\/]+", text))


def test_generator_is_independent_of_the_product_and_of_the_frozen_package():
    tree = ast.parse(GENERATOR.read_text(encoding="utf-8"))
    imported = set()
    for node in ast.walk(tree):
        if isinstance(node, ast.Import):
            imported |= {alias.name.split(".")[0] for alias in node.names}
        elif isinstance(node, ast.ImportFrom):
            assert node.level == 0, "no relative imports"
            imported.add(node.module.split(".")[0])
        elif isinstance(node, ast.Constant) and isinstance(node.value, str):
            assert not names_a_product_path(node.value), node.value
        elif isinstance(node, ast.Call):
            func = node.func
            name = func.id if isinstance(func, ast.Name) else func.attr if isinstance(func, ast.Attribute) else None
            assert name not in FORBIDDEN_CALLS, name
    assert all(name in sys.stdlib_module_names for name in imported), imported
    assert "exact_difference_basis/references.json" not in GENERATOR.read_text(encoding="utf-8")


def test_status_says_candidate_and_not_frozen():
    data = load(PACKAGE / "references.json")
    status = "Candidate references for the recorded operands. Independent refutation precedes freezing."
    assert data["status"] == status
    readme = " ".join((PACKAGE / "README.md").read_text(encoding="utf-8").split())
    assert status in readme
    assert "**Status.** Frozen" not in readme and "accepted" not in readme.lower()


def every_row(data):
    return data["carried"]["gate_rows"] + data["carried"]["pairs"] + data["added"] + data["builder_witnesses"]


def test_every_row_has_both_entries():
    data = load(PACKAGE / "references.json")
    rows = every_row(data)
    assert [len(data["carried"]["gate_rows"]), len(data["carried"]["pairs"]), len(data["added"]),
            len(data["builder_witnesses"])] == [19, 17, 16, 6]
    for row in rows:
        adm, mech = row["admission"], row["mechanical"]
        assert adm["verdict"] in VERDICTS, row["id"]
        assert adm["reason"] and adm["gate_only_reason"], row["id"]
        assert isinstance(adm["gate_only"], bool), row["id"]
        assert set(adm["gates"]) == {"restraint", "g1", "g2"}, row["id"]
        for part in ("restraint", "g1", "g2"):
            assert adm["gates"][part]["holds"] == (not adm["gates"][part]["cause"]), (row["id"], part)
        first_failing = next((p for p in ("restraint", "g1", "g2") if not adm["gates"][p]["holds"]), None)
        assert adm["verdict"] == ("admitted" if first_failing is None else f"refused_{first_failing}"), row["id"]
        assert mech["kind"] in KINDS and mech["meaning"], row["id"]
        if mech["kind"] != "none":
            values = mech.get("rows") or mech.get("values")
            assert values, row["id"]
            for entry in values.values():
                if "pi" not in entry["exact"]:
                    assert float(Fraction(entry["exact"])) == entry["value"], row["id"]
                assert entry["unit"]


def test_the_added_rows_state_the_specified_verdicts_and_gate_only_marks():
    data = load(PACKAGE / "references.json")
    found = {r["id"]: (r["admission"]["verdict"], r["admission"]["gate_only"], r["mechanical"]["kind"])
             for r in data["added"] + data["builder_witnesses"]}
    assert found == {
        "W1": ("admitted", False, "linear_increment"), "W2": ("admitted", False, "linear_increment"),
        "W3": ("admitted", False, "linear_increment"), "W4": ("admitted", False, "linear_increment"),
        "W5": ("refused_g1", True, "linear_increment"), "W6": ("refused_restraint", True, "state_difference"),
        "W7a": ("refused_restraint", True, "state_difference"),
        "W7b": ("refused_restraint", True, "state_difference"),
        "W8": ("refused_g1", True, "linear_increment"), "W9": ("admitted", False, "linear_increment"),
        "F1": ("refused_g1", False, "linear_increment"), "F2": ("refused_g1", False, "linear_increment"),
        "F2b": ("refused_g1", False, "linear_increment"), "K1": ("refused_g2", True, "linear_increment"),
        "W10": ("refused_g2", False, "linear_increment"), "W11": ("refused_g2", True, "state_difference"),
        "X1": ("refused_g1", True, "linear_increment"), "X2": ("admitted", True, "linear_increment"),
        "X3": ("refused_restraint", True, "state_difference"), "X4": ("refused_g1", False, "linear_increment"),
        "X5": ("refused_g1", True, "none"), "X6": ("refused_g2", True, "linear_increment")}


def test_the_stiffness_readings_are_recorded_and_differ_only_where_the_readme_says():
    data = load(PACKAGE / "references.json")
    differ = []
    for row in every_row(data):
        readings = row["stiffness_readings"]
        assert readings["S"]["verdict"] == row["admission"]["verdict"], row["id"]
        assert readings["differ"] == (readings["S"]["verdict"] != readings["R"]["verdict"]), row["id"]
        if readings["differ"]:
            differ.append((row["id"], readings["S"]["verdict"], readings["R"]["verdict"]))
    assert differ == [("K1", "refused_g2", "admitted"), ("W10", "refused_g2", "admitted"),
                      ("W11", "refused_g2", "admitted")]
    readme = (PACKAGE / "README.md").read_text(encoding="utf-8")
    table = readme.split("### Where the two stiffness readings differ", 1)[1].split("\n\n", 2)[1]
    assert [line.split(" | ")[0].strip("| ") for line in table.splitlines()[2:]] == [rid for rid, _, _ in differ]


def exact_terms(text):
    """'3/4*pi + 1/2 + 5/7*pi^-1' as {power: Fraction}, so that the comparison does not rest on formatting."""
    if text == "0":
        return {}
    terms = {}
    for part in text.split(" + "):
        if "*pi" in part:
            coefficient, power = part.split("*pi")
            power = int(power[1:]) if power.startswith("^") else 1
        else:
            coefficient, power = part, 0
        terms[power] = terms.get(power, Fraction(0)) + Fraction(coefficient)
    return {k: v for k, v in terms.items() if v}


def test_carried_gate_rows_keep_their_frozen_verdicts():
    frozen = {r["id"]: r["outcome"] for r in load(FROZEN / "references.json")["g1"]}
    carried = load(PACKAGE / "references.json")["carried"]["gate_rows"]
    assert [r["id"] for r in carried] == list(frozen)
    for row in carried:
        assert row["frozen_outcome"] == frozen[row["id"]], row["id"]
        assert row["admission"]["verdict"] == FROZEN_TO_COMPLETED[frozen[row["id"]]], row["id"]


def test_carried_pairs_keep_their_frozen_verdicts_and_values():
    frozen = {p["id"]: p for p in load(FROZEN / "references.json")["pairs"]}
    carried = load(PACKAGE / "references.json")["carried"]["pairs"]
    assert [p["id"] for p in carried] == list(frozen)
    compared = alternatives = 0
    for pair in carried:
        old = frozen[pair["id"]]
        assert pair["frozen_outcome"] == old["outcome"], pair["id"]
        assert pair["admission"]["verdict"] == FROZEN_TO_COMPLETED[old["outcome"]], pair["id"]
        mech = pair["mechanical"]
        assert mech["kind"] == ("linear_increment" if old["g2_same_basis"] else "state_difference"), pair["id"]
        field = old["values_field"]
        assert mech["frozen_values_field"] == field
        assert set(mech["rows"]) == set(old[field]), pair["id"]
        for key, row in mech["rows"].items():
            was = old[field][key]
            assert exact_terms(row["exact"]) == exact_terms(was["exact"]), (pair["id"], key)
            assert row["value"] == was["value"] and row["unit"] == was["unit"], (pair["id"], key)
            compared += 1
        old_alt, new_alt = old.get("reading_J_alternative"), mech.get("reading_J_alternative")
        if isinstance(old_alt, dict):
            assert set(new_alt) == set(old_alt), pair["id"]
            for key, row in new_alt.items():
                assert exact_terms(row["exact"]) == exact_terms(old_alt[key]["exact"]), (pair["id"], key)
                assert row["value"] == old_alt[key]["value"], (pair["id"], key)
                alternatives += 1
        else:
            assert (old_alt is None) == (new_alt is None), pair["id"]
    assert (compared, alternatives) == (1806, 531)
