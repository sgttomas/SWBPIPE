"""Seismic component-weight references (`validation/references/seismic_component_weight/`).

The generator rebuilds `references.json` and `SHA256SUMS` in memory and compares
them byte for byte with the committed files; while rebuilding it solves every case
by two exact methods and repeats the build on binary64 inputs. Here: that check
passes, the committed SHA256SUMS verifies, and the generator imports only the
standard library.
"""
import ast
import hashlib
import json
import subprocess
import sys
from pathlib import Path

PROJECT = Path(__file__).resolve().parents[1]
REFERENCE = PROJECT / "validation" / "references" / "seismic_component_weight"
GENERATOR = REFERENCE / "generators" / "scw_reference.py"


def test_generator_check_rebuilds_the_committed_references():
    result = subprocess.run([sys.executable, "-I", str(GENERATOR)], cwd=GENERATOR.parent,
                            capture_output=True, text=True, timeout=600)
    assert result.returncode == 0, result.stdout + result.stderr


def test_sha256sums_verify():
    lines = (REFERENCE / "SHA256SUMS").read_text().splitlines()
    assert {line.split("  ", 1)[1] for line in lines} == {
        "README.md", "generators/scw_reference.py", "references.json"}
    for line in lines:
        digest, path = line.split("  ", 1)
        assert hashlib.sha256((REFERENCE / path).read_bytes()).hexdigest() == digest, path


def test_generator_imports_only_the_standard_library():
    tree = ast.parse(GENERATOR.read_text())
    modules = set()
    for node in ast.walk(tree):
        if isinstance(node, ast.Import):
            modules.update(alias.name.split(".")[0] for alias in node.names)
        elif isinstance(node, ast.ImportFrom):
            assert node.level == 0, "relative import"
            modules.add(node.module.split(".")[0])
    assert modules <= set(sys.stdlib_module_names), modules - set(sys.stdlib_module_names)


def test_parity_and_cross_checks_are_frozen():
    doc = json.loads((REFERENCE / "references.json").read_text())
    assert all(c["count"] == c["agreeing"] for c in doc["crosscheck"])
    assert doc["binary64_input_check"]["sign_changes"] == 0
    assert doc["binary64_input_check"]["discrete_outcome_changes"] == 0
    for case in doc["cases"]:
        if "parity" not in case:
            continue
        same_gravity = case["gravity"]["exact"] == case["parity"]["weight_gravity"]["exact"]
        expected = case["role"] in ("reference", "alternative") and same_gravity
        assert case["parity"]["holds"] is expected, case["id"]
        if case["role"] == "reference" and not same_gravity:
            assert case["parity"]["at_the_seismic_case_gravity"]["holds"] is True, case["id"]
