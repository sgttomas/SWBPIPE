"""Hydrostatic test references (`validation/references/hydrostatic_test/`).

The generator rebuilds `references.json` and `SHA256SUMS` in memory and compares
them byte for byte with the committed files. While rebuilding, every case is
solved exactly in Q(pi) by its closed form and by a second method (flexibility,
beam stiffness, three-moment), and every input is replaced by its binary64
value to show that no discrete outcome changes.
"""
import ast
import hashlib
import json
import subprocess
import sys
from pathlib import Path

PROJECT = Path(__file__).resolve().parents[1]
REFERENCE = PROJECT / "validation" / "references" / "hydrostatic_test"
GENERATOR = REFERENCE / "generators" / "hydrostatic_test_reference.py"


def test_generator_check_rebuilds_the_committed_references():
    result = subprocess.run([sys.executable, "-I", str(GENERATOR)], cwd=GENERATOR.parent,
                            capture_output=True, text=True, timeout=600)
    assert result.returncode == 0, result.stdout + result.stderr


def test_sha256sums_verify():
    lines = (REFERENCE / "SHA256SUMS").read_text().splitlines()
    assert {line.split("  ", 1)[1] for line in lines} == {
        "README.md", "generators/hydrostatic_test_reference.py", "references.json"}
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
    assert modules, "no imports found"
    assert modules <= set(sys.stdlib_module_names), modules - set(sys.stdlib_module_names)


def test_the_closed_forms_keep_their_values():
    doc = json.loads((REFERENCE / "references.json").read_text())
    assert all(c["agrees"] for c in doc["crosscheck"])
    assert doc["binary64_input_check"]["discrete_outcomes_unchanged"]
    cases = {c["id"]: c for c in doc["cases"]}
    held = {r["id"]: r for r in cases["HT-2S-SPRING-HELD"]["rows"]}
    wa = doc["derived"]["w_test"]["value"] * doc["inputs"]["a"]["value"]
    assert abs(held["support:b.fz"]["value"] - 10 / 8 * wa) <= 1e-12 * wa
    assert abs(held["support:a.fz"]["value"] - 3 / 8 * wa) <= 1e-12 * wa
    assert held["node:b.uz"]["value"] == 0.0
    assert cases["HT-2S-CE-HELD"]["expected_outcome"]["constant_effort_applied_force"][0]["applied_force"]["value"] == 0.0
    assert cases["HT-2S-SPRING-ORDINARY"]["test_condition"] is None
