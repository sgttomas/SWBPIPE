"""Friction-on-one-way-normal references (`validation/references/friction_one_way_normal/`).

The generator rebuilds `references.json` and `SHA256SUMS` in memory, re-running
every exact check (closed form against an independent stiffness solve, a unique
admissible state per case, active-set iteration, equilibrium and robustness),
and compares the bytes with the committed files.
"""
import ast
import hashlib
import json
import subprocess
import sys
from pathlib import Path

PROJECT = Path(__file__).resolve().parents[1]
REFERENCE = PROJECT / "validation" / "references" / "friction_one_way_normal"
GENERATOR = REFERENCE / "generators" / "friction_one_way_normal_reference.py"


def test_generator_check_rebuilds_the_committed_references():
    result = subprocess.run([sys.executable, "-I", "-B", str(GENERATOR)], cwd=GENERATOR.parent,
                            capture_output=True, text=True, timeout=600)
    assert result.returncode == 0, result.stdout + result.stderr


def test_sha256sums_verify():
    lines = (REFERENCE / "SHA256SUMS").read_text(encoding="ascii").splitlines()
    assert {line.split("  ", 1)[1] for line in lines} == {
        "README.md", "generators/friction_one_way_normal_reference.py", "references.json"}
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


def test_the_states_and_the_normal_source_controls():
    cases = {c["id"]: c for c in json.loads((REFERENCE / "references.json").read_text())["cases"]}
    assert cases["STICK"]["states"] == {"one-way S": "active", "friction S": "stick"}
    assert cases["SLIDE-NEG-B"]["states"]["friction S"] == "slide-"
    assert cases["LIFT-NEG"]["reactions"]["friction S"]["FX"]["exact"] == "0"
    assert cases["NEAR-STICK"]["states"]["friction S"] == "stick"
    assert cases["NEAR-SLIDE"]["states"]["friction S"] == "slide+"
    assert cases["NEAR-BEAR"]["states"]["one-way S"] == "active"
    assert cases["NEAR-LIFT"]["states"]["one-way S"] == "inactive"
    unload = cases["SLIDE-UNLOAD"]
    assert unload["reactions"]["one-way S"]["FZ"]["exact"] == "62421920/22241"
    weight = next(c for c in unload["negative_controls"] if c["id"] == "CTRL-WEIGHT-NORMAL")
    assert weight["friction state"] == "stick"
