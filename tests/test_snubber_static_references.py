"""Snubber static references (`validation/references/snubber_static/`).

The generator rebuilds `references.json` and `SHA256SUMS` in memory, re-running
every exact check (closed form against an independent stiffness solve, the
unique one-way state, equilibrium, the identity and the separation of each
negative control), and compares the bytes with the committed files.
"""
import ast
import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path

PROJECT = Path(__file__).resolve().parents[1]
REFERENCE = PROJECT / "validation" / "references" / "snubber_static"
GENERATOR = REFERENCE / "generators" / "snubber_static_reference.py"


def test_generator_check_rebuilds_the_committed_references():
    result = subprocess.run([sys.executable, "-I", "-B", str(GENERATOR)], cwd=GENERATOR.parent,
                            capture_output=True, text=True, timeout=600)
    assert result.returncode == 0, result.stdout + result.stderr


def test_sha256sums_verify():
    lines = (REFERENCE / "SHA256SUMS").read_text(encoding="ascii").splitlines()
    assert {line.split("  ", 1)[1] for line in lines} == {
        "README.md", "generators/snubber_static_reference.py", "references.json"}
    for line in lines:
        digest, name = line.split("  ", 1)
        assert hashlib.sha256((REFERENCE / name).read_bytes()).hexdigest() == digest, name


PRODUCT_ROOTS = {"core", "apps", "tests", "tools", "fixtures", "schemas"}
DYNAMIC_LOADERS = {"__import__", "import_module", "exec", "eval", "spec_from_file_location"}


def _names_product_path(text):
    return bool(text) and not re.search(r"\s", text) and any(
        segment in PRODUCT_ROOTS for segment in re.split(r"[\\/]+", text))


def test_generator_is_independent_of_the_product():
    tree = ast.parse(GENERATOR.read_text(encoding="utf-8"))
    imported, product_paths, loaders = set(), [], []
    for node in ast.walk(tree):
        if isinstance(node, ast.Import):
            imported.update(alias.name.split(".")[0] for alias in node.names)
        elif isinstance(node, ast.ImportFrom):
            assert node.level == 0, "relative import"
            imported.add(node.module.split(".")[0])
        elif isinstance(node, ast.Constant) and isinstance(node.value, str) and _names_product_path(node.value):
            product_paths.append(node.value)
        elif isinstance(node, ast.Call):
            name = node.func.id if isinstance(node.func, ast.Name) else getattr(node.func, "attr", None)
            if name in DYNAMIC_LOADERS:
                loaders.append(name)
    assert imported <= set(sys.stdlib_module_names), imported - set(sys.stdlib_module_names)
    assert not product_paths, product_paths
    assert not loaders, loaders


def test_every_variant_keeps_the_base_result_and_discloses_each_snubber():
    data = json.loads((REFERENCE / "references.json").read_text())
    cases = {c["id"]: c for c in data["cases"]}
    assert cases["SL-1"]["base result (static; also the expected result of every variant)"]["reactions"]["support S"]["FZ"]["exact"] == "50625/4"
    assert cases["SN-LIFT"]["base result (static; also the expected result of every variant)"]["one-way support S state"] == "inactive"
    for case in data["cases"]:
        for variant in case["variants"]:
            names = [s["id"] for s in variant["snubbers"]]
            disclosure = variant["expected disclosure"]
            assert disclosure["snubbers named"] == names
            assert disclosure["wording"] == "dynamic-only restraint, not active in static cases"
            assert disclosure["severity"] == "non-blocking"
            assert variant["negative_controls"]
            assert all(c["label"].startswith("NEGATIVE CONTROL") for c in variant["negative_controls"])
            for control in variant["negative_controls"]:
                separation = control["separation by kind (largest change of the kind / largest base magnitude of the same kind)"]
                assert max(v["value"] for v in separation.values()) > 0.05
