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
