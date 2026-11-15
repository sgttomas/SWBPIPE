"""Exact-route declared difference basis references (`validation/references/exact_difference_basis/`).

The generator rebuilds `references.json` and `SHA256SUMS` in memory and compares
them byte for byte with the committed files. While rebuilding it solves every case
by two methods that must agree exactly, checks each difference against the
closed-form response to the loads the operating case has beyond the sustained
case, and checks equilibrium; any disagreement fails the check.
"""
import ast
import hashlib
import json
import re
import subprocess
from fractions import Fraction
import sys
from pathlib import Path

PROJECT = Path(__file__).resolve().parents[1]
REFERENCE = PROJECT / "validation" / "references" / "exact_difference_basis"
GENERATOR = REFERENCE / "generators" / "exact_difference_basis_reference.py"
PRODUCT_ROOTS = {"core", "apps", "tests", "tools", "fixtures", "schemas"}
FORBIDDEN_CALLS = {"__import__", "import_module", "exec", "eval", "spec_from_file_location"}


def test_generator_check_rebuilds_the_committed_references():
    result = subprocess.run([sys.executable, "-I", "-B", str(GENERATOR)], cwd=GENERATOR.parent,
                            capture_output=True, text=True, timeout=600)
    assert result.returncode == 0, result.stdout + result.stderr


def test_sha256sums_verify():
    lines = (REFERENCE / "SHA256SUMS").read_text().splitlines()
    assert {line.split("  ", 1)[1] for line in lines} == {
        "README.md", "generators/exact_difference_basis_reference.py", "references.json"}
    for line in lines:
        digest, name = line.split("  ", 1)
        assert hashlib.sha256((REFERENCE / name).read_bytes()).hexdigest() == digest, name


def names_a_product_path(text: str) -> bool:
    if not text or re.search(r"\s", text):
        return False
    return any(segment in PRODUCT_ROOTS for segment in re.split(r"[\\/]+", text))


def test_generator_is_independent_of_the_product():
    tree = ast.parse(GENERATOR.read_text())
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


def test_outcomes_and_row_text_keep_their_meaning():
    data = json.loads((REFERENCE / "references.json").read_text())
    assert data["row_text"] == "operating minus sustained difference"
    outcomes = {p["id"]: p["outcome"] for p in data["pairs"]}
    assert outcomes["M1/g2/mixed_basis"] == "refused_g2"
    assert outcomes["M1/g2/same_basis_twin"] == "published"
    assert outcomes["M1/equal_pressure/operating_on_sustained_off"] == "published"
    assert outcomes["M1/pressure_change/operating_on_sustained_on"] == "refused_g1"
    g1 = {r["id"]: r["outcome"] for r in data["g1"]}
    assert g1["contained"] == "published"
    assert g1["sustained_only_load"] == "refused_g1"
    assert g1["twice_in_sustained_once_in_operating"] == "refused_g1"
    assert g1["same_target_different_magnitude"] == "refused_g1"
    assert g1["pressure_differs"] == "refused_g1"
    assert g1["elongation_off_sustained_on_operating"] == "published"
    assert g1["elongation_on_sustained_off_operating"] == "refused_g1"
    assert g1["member_state_same_authored_temperature"] == "published"
    assert g1["member_state_temperature_one_ulp_apart"] == "refused_g1"
    row = data["pairs"][4]["difference"]["u/B/UX"]
    assert data["pairs"][4]["id"] == "M1/equal_pressure/operating_on_sustained_on"
    assert row["from_published_operands"] == 0.0141342719743767
    assert row["value"] == 0.014134271974376698


def published_difference(operating: float, sustained: float) -> float:
    """The package's rule: the exact difference of two binary64 operands, rounded once."""
    return float(Fraction(operating) - Fraction(sustained))


def test_published_differences_are_reproducible_from_operands_in_the_file():
    data = json.loads((REFERENCE / "references.json").read_text())
    j_rows = 0
    for pair in data["pairs"]:
        cases, rows = pair["cases"], pair[pair["values_field"]]
        for key, row in rows.items():
            assert row["from_published_operands"] == published_difference(
                cases["operating"][key], cases["sustained"][key]), (pair["id"], key)
        if not pair["g2_same_basis"]:
            assert "reading_J_cases" not in pair
            continue
        jcases, jpub = pair["reading_J_cases"], pair["reading_J_publication"]
        assert set(jpub) == set(rows)
        tally = {"rows": 0, "analytically_identical": 0, "publication_bit_identical": 0,
                 "analytically_identical_but_publication_differs": 0}
        alternative = pair["reading_J_alternative"]
        for key, row in jpub.items():
            value = published_difference(jcases["operating"][key], jcases["sustained"][key])
            assert row["from_published_operands"] == value, (pair["id"], key)
            same_bits = value.hex() == rows[key]["from_published_operands"].hex()
            assert row["bit_identical_to_reading_W_publication"] == same_bits, (pair["id"], key)
            assert row["analytically_identical_to_reading_W"] == (
                not isinstance(alternative, dict) or key not in alternative), (pair["id"], key)
            if isinstance(alternative, dict) and key in alternative:
                assert alternative[key]["from_published_operands"] == value, (pair["id"], key)
            tally["rows"] += 1
            tally["analytically_identical"] += row["analytically_identical_to_reading_W"]
            tally["publication_bit_identical"] += same_bits
            tally["analytically_identical_but_publication_differs"] += (
                row["analytically_identical_to_reading_W"] and not same_bits)
        assert pair["reading_J_publication_summary"] == tally, pair["id"]
        j_rows += tally["rows"]
    assert j_rows == data["cross_checks"]["reading_J_publication_rows"]


def test_setting_independence_of_publication_values_is_computed():
    data = json.loads((REFERENCE / "references.json").read_text())
    pairs = {p["id"]: p for p in data["pairs"]}
    for model, record in data["setting_independence_publication"].items():
        off = pairs[f"{model}/equal_pressure/operating_off_sustained_off"]["difference"]
        on = pairs[f"{model}/equal_pressure/operating_on_sustained_on"]["difference"]
        differ = sorted(k for k in off if off[k]["from_published_operands"].hex() != on[k]["from_published_operands"].hex())
        assert record["publication_differs"] == differ
        assert record["publication_bit_identical"] == len(off) - len(differ)
        assert all(off[k]["exact"] == on[k]["exact"] for k in off)


def test_forbidden_value_flags_mark_only_records_that_distinguish():
    data = json.loads((REFERENCE / "references.json").read_text())
    counts = {}
    for pair in data["pairs"]:
        field = pair["values_field"]
        mags = pair[{"difference": "magnitudes", "undeclared_subtraction": "undeclared_subtraction_magnitudes",
                     "withheld_would_be_difference": "withheld_would_be_magnitudes"}[field]]
        for key, rec in mags.items():
            wrong = rec["forbidden_subtracted_magnitudes"]
            expected = wrong.hex() not in (rec["from_combined_components"].hex(),
                                           rec["from_published_operand_components"].hex())
            assert rec["forbidden_value_distinguishes"] == expected, (pair["id"], key)
            total, flagged = counts.get(field, (0, 0))
            counts[field] = (total + 1, flagged + expected)
    assert counts["difference"] == (106, 60)
