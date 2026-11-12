"""Friction-on-one-way-normal references (`validation/references/friction_one_way_normal/`).

The generator rebuilds `references.json` and `SHA256SUMS` in memory, re-running
every exact check (closed form against an independent stiffness solve, the
classification law and strict confirmation on every state pair, the boundary
witnesses, active-set iteration, equilibrium and robustness), and compares the
bytes with the committed files.
"""
import ast
import hashlib
import json
import re
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
    stick = cases["STICK-BOUNDARY"]
    assert stick["classification"]["returns"] == {"one-way S": "active", "friction S": "stick"}
    assert stick["strict confirmation"]["establishes"] is False
    assert stick["deciding_quantities"]["stick margin mu*N - |T| (> 0)"]["exact"] == "0"
    lift = cases["LIFT-BOUNDARY"]
    assert lift["strict confirmation"]["establishes"] is False
    assert lift["displacements"]["S"]["UZ"]["exact"] == "0"
    assert lift["displacements"]["S"]["UX"]["exact"] == "(-21469056/5517578125)/pi"
    assert lift["classification"]["pairs consistent if a zero reaction does not bear"] == [{"one-way S": "inactive", "friction S": "inactive"}]


def test_the_lifted_cases_rest_on_the_decided_text_with_the_literal_alternative():
    data = json.loads((REFERENCE / "references.json").read_text())
    assert data["confirmation_text"]["decided text"].startswith("(contact reaction, separation")
    cases = {c["id"]: c for c in data["cases"]}
    alternative = "alternative (named; not primary): literal reading of today's text"
    for cid in ("LIFT-0", "LIFT-POS", "LIFT-NEG", "NEAR-LIFT"):
        confirmation = cases[cid]["strict confirmation"]
        assert confirmation["establishes"] is True
        assert confirmation["established pair"] == {"one-way S": "inactive", "friction S": "inactive"}
        assert "not yet in" in confirmation["rests on"]["status"]
        assert confirmation[alternative]["establishes"] is False
        first, second = confirmation["establishment order"]
        assert first["one-way S"] == "released" and first["sign"] == "+" and first["separation u_z(S)"]["value"] > 0
        assert second["friction S"] == "inactive" and second["N"]["exact"] == "0" and second["T"]["exact"] == "0"
        released = next(e for e in cases[cid]["enumeration"] if e["state"] == {"one-way": "inactive", "friction": "inactive"})
        assert released["strictly confirmed"] is True
        assert released["alternative: strictly confirmed under the literal reading of today's text"] is False
    for cid, case in cases.items():
        if cid not in ("LIFT-0", "LIFT-POS", "LIFT-NEG", "NEAR-LIFT"):
            assert alternative not in case["strict confirmation"], cid
            assert not any(k.startswith("alternative") for e in case["enumeration"] for k in e), cid
