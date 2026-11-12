"""Exact-route declared difference basis references (`validation/references/exact_difference_basis/`).

The generator rebuilds `references.json` and `SHA256SUMS` in memory and compares
them byte for byte with the committed files. While rebuilding it solves every case
by two methods that must agree exactly, checks each published difference against
the closed-form response to the loads the operating case has beyond the sustained
case, and checks equilibrium; any disagreement fails the check.
"""
import ast
import hashlib
import json
import subprocess
import sys
from pathlib import Path

PROJECT = Path(__file__).resolve().parents[1]
REFERENCE = PROJECT / "validation" / "references" / "exact_difference_basis"
GENERATOR = REFERENCE / "generators" / "exact_difference_basis_reference.py"


def test_generator_check_rebuilds_the_committed_references():
    result = subprocess.run([sys.executable, "-I", str(GENERATOR)], cwd=GENERATOR.parent,
                            capture_output=True, text=True, timeout=600)
    assert result.returncode == 0, result.stdout + result.stderr


def test_sha256sums_verify():
    lines = (REFERENCE / "SHA256SUMS").read_text().splitlines()
    assert {line.split("  ", 1)[1] for line in lines} == {
        "README.md", "generators/exact_difference_basis_reference.py", "references.json"}
    for line in lines:
        digest, name = line.split("  ", 1)
        assert hashlib.sha256((REFERENCE / name).read_bytes()).hexdigest() == digest, name


def test_generator_imports_only_the_standard_library():
    allowed = {"__future__", "hashlib", "json", "sys", "collections", "fractions", "math", "pathlib"}
    tree = ast.parse(GENERATOR.read_text())
    imported = set()
    for node in ast.walk(tree):
        if isinstance(node, ast.Import):
            imported |= {alias.name.split(".")[0] for alias in node.names}
        elif isinstance(node, ast.ImportFrom):
            assert node.level == 0, "no relative imports"
            imported.add(node.module.split(".")[0])
    assert imported <= allowed, imported - allowed
    assert all(name in sys.stdlib_module_names for name in imported)


def test_outcomes_and_row_text_keep_their_meaning():
    data = json.loads((REFERENCE / "references.json").read_text())
    assert data["row_text"] == "operating minus sustained difference"
    outcomes = {p["id"]: p["outcome"] for p in data["pairs"]}
    assert outcomes["M1/g2/mixed_basis"] == "refused_g2"
    assert outcomes["M1/g2/same_basis_twin"] == "published"
    g1 = {r["id"]: r["outcome_reading_A_primitives"] for r in data["g1"]}
    assert g1["contained"] == "published"
    assert g1["sustained_only_load"] == "refused_g1"
    assert g1["twice_in_sustained_once_in_operating"] == "refused_g1"
    assert g1["same_target_different_magnitude"] == "refused_g1"
