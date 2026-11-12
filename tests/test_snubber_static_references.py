"""Snubber static references (`validation/references/snubber_static/`).

The generator rebuilds `references.json` and `SHA256SUMS` in memory, re-running
every exact check (closed form against an independent stiffness solve, the
unique one-way state, equilibrium, the identity and the separation of each
negative control), and compares the bytes with the committed files.
"""
import ast
import hashlib
import json
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


def test_generator_imports_only_the_standard_library():
    tree = ast.parse(GENERATOR.read_text(encoding="utf-8"))
    imported = set()
    for node in ast.walk(tree):
        if isinstance(node, ast.Import):
            imported.update(alias.name.split(".")[0] for alias in node.names)
        elif isinstance(node, ast.ImportFrom):
            assert node.level == 0, "relative import"
            imported.add(node.module.split(".")[0])
    assert imported <= set(sys.stdlib_module_names), imported - set(sys.stdlib_module_names)


def test_every_variant_keeps_the_base_result_and_discloses_each_snubber():
    data = json.loads((REFERENCE / "references.json").read_text())
    cases = {c["id"]: c for c in data["cases"]}
    assert cases["SL-1"]["base result (static; also the expected result of every variant)"]["reactions"]["support S"]["FZ"]["exact"] == "50625/4"
    assert cases["SN-LIFT"]["base result (static; also the expected result of every variant)"]["one-way support S state"] == "inactive"
    for case in data["cases"]:
        for variant in case["variants"]:
            names = [s["id"] for s in variant["snubbers"]]
            assert [d["snubber"] for d in variant["expected disclosure"]] == names
            assert {d["text"] for d in variant["expected disclosure"]} == {"dynamic-only restraint, not active in static cases"}
            assert variant["negative_controls"]
            assert all(c["label"].startswith("NEGATIVE CONTROL") for c in variant["negative_controls"])
