"""Reproduce the independent hanger-sizing references (DEL-04-11 REQ-010) with their maintained generators,
and hold each generator to the independence REQ-010 states: it imports no product code."""
import ast
from pathlib import Path
import re
import subprocess
import sys

import pytest

PROJECT = Path(__file__).resolve().parents[1]
REFERENCES = PROJECT / "validation/references/hanger_sizing"
GENERATORS = ["generate_r3.py", "generate_r125.py", "generate_ex_sizing.py", "generate_preload.py"]
# Roots that hold product code or product output. A generator names none of them as a path segment.
PRODUCT_ROOTS = {"core", "apps", "tests", "tools", "fixtures", "schemas"}


def _names_product_path(literal):
    """Whether a string or bytes literal has a product root as one of its path segments, on either separator:
    `core/x.py`, `../core/x.py`, `core/my file.py`, an absolute path through a root, `b"core/x.py"`, and the bare
    `core` of `Path("core") / "x.py"`. A segment must be the root's name exactly, so prose that mentions a root
    inside a sentence is not a path; prose that begins with a root and a separator is flagged and is reworded. A
    path built to hide its segments is outside what this scan is for."""
    text = literal.decode("latin-1") if isinstance(literal, bytes) else literal
    return bool(PRODUCT_ROOTS & set(re.split(r"[\\/]+", text)))


@pytest.mark.parametrize("generator", GENERATORS)
def test_hanger_sizing_reference_generator_reproduces_committed_data(generator):
    subprocess.run([sys.executable, str(REFERENCES / generator), "--check"], cwd=PROJECT, check=True)


def test_every_generator_in_the_package_is_checked():
    assert sorted(path.name for path in REFERENCES.glob("generate_*.py")) == sorted(GENERATORS)


@pytest.mark.parametrize("generator", GENERATORS)
def test_hanger_sizing_reference_generator_imports_no_product_code(generator):
    tree = ast.parse((REFERENCES / generator).read_text())
    imported = set()
    for node in ast.walk(tree):
        if isinstance(node, ast.Import):
            imported.update(alias.name.split(".")[0] for alias in node.names)
        elif isinstance(node, ast.ImportFrom):
            assert node.level == 0, f"{generator} uses a relative import"
            imported.add((node.module or "").split(".")[0])
        elif isinstance(node, ast.Call):
            name = getattr(node.func, "id", getattr(node.func, "attr", ""))
            assert name not in {"__import__", "import_module", "exec", "eval", "spec_from_file_location"}, \
                f"{generator} loads code dynamically ({name})"
        elif isinstance(node, ast.Constant) and isinstance(node.value, (str, bytes)):
            assert not _names_product_path(node.value), f"{generator} names a product path: {node.value!r}"
    assert imported <= set(sys.stdlib_module_names), f"{generator} imports outside the standard library: {imported}"


@pytest.mark.parametrize("literal,named", [
    ("core/analysis/x.py", True), ("../core/x.py", True), ("core", True), ("/abs/repo/fixtures/results/a.json", True),
    ("apps\\desktop\\x.ts", True), ("core/my file.py", True), ("/Users/someone/My Project/core/x.py", True),
    (b"core/x.py", True), (b"../schemas/model.schema.yaml", True),
    ("ex_sizing.json", False), ("validation/references/hanger_sizing", False),
    ("reads no product code (core/, apps/, tests/)", False), ("nothing under core/ is read", False),
    ("score/card", False), (b"ex_sizing.json", False), ("", False),
])
def test_the_product_path_scan_recognizes_ordinary_path_literals(literal, named):
    assert _names_product_path(literal) is named
