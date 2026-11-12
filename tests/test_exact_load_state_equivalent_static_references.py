"""Exact-route generated loads in a load and reference-state document
(`validation/references/exact_load_state_equivalent_static/`).

The generator rebuilds `references.json` and `SHA256SUMS` in memory, compares
them byte for byte with the committed files and fails if any of its
cross-checks disagrees. The package stays independent: its generator imports
only the Python standard library.
"""
import ast
import hashlib
import json
import subprocess
import sys
from pathlib import Path

PROJECT = Path(__file__).resolve().parents[1]
REFERENCE = PROJECT / "validation" / "references" / "exact_load_state_equivalent_static"
GENERATOR = REFERENCE / "generators" / "exact_load_state_reference.py"


def test_generator_check_rebuilds_the_committed_references():
    result = subprocess.run([sys.executable, "-I", str(GENERATOR)], cwd=GENERATOR.parent,
                            capture_output=True, text=True, timeout=900)
    assert result.returncode == 0, result.stdout + result.stderr


def test_sha256sums_verify():
    lines = (REFERENCE / "SHA256SUMS").read_text().splitlines()
    assert lines
    listed = set()
    for line in lines:
        digest, path = line.split("  ", 1)
        listed.add(path)
        assert hashlib.sha256((REFERENCE / path).read_bytes()).hexdigest() == digest, path
    assert listed == {"README.md", "references.json", "generators/exact_load_state_reference.py"}


def test_generator_imports_only_the_standard_library():
    tree = ast.parse(GENERATOR.read_text())
    modules = set()
    for node in ast.walk(tree):
        if isinstance(node, ast.Import):
            modules |= {alias.name.split(".")[0] for alias in node.names}
        elif isinstance(node, ast.ImportFrom):
            assert node.level == 0, "relative import"
            modules.add(node.module.split(".")[0])
    assert modules, "no imports found"
    assert modules <= set(sys.stdlib_module_names), sorted(modules - set(sys.stdlib_module_names))


def test_every_cross_check_agrees_and_pressure_rows_are_labelled():
    doc = json.loads((REFERENCE / "references.json").read_text())
    assert doc["crosscheck"] and all(c["agrees"] for c in doc["crosscheck"])
    dependent = 0
    for case in doc["cases"]:
        for row in case["rows"]:
            if row["pressure_elongation"] == "independent":
                assert "expected" in row
            else:
                assert row["pressure_elongation"] == "dependent"
                assert {"expected_pressure_elongation_off", "expected_pressure_elongation_on"} <= set(row)
                dependent += 1
    assert dependent > 0
