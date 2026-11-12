"""Reproduce the independent hanger-sizing references (DEL-04-11 REQ-010) with their maintained generators,
and hold each generator to the independence REQ-010 states: it imports no product code."""
import ast
from pathlib import Path
import subprocess
import sys

import pytest

PROJECT = Path(__file__).resolve().parents[1]
REFERENCES = PROJECT / "validation/references/hanger_sizing"
GENERATORS = ["generate_r3.py", "generate_r125.py", "generate_ex_sizing.py", "generate_preload.py"]
# Roots that hold product code or product output. A generator names none of them.
PRODUCT_ROOTS = ("core/", "apps/", "tests/", "tools/", "fixtures/", "schemas/")


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
        elif isinstance(node, ast.Constant) and isinstance(node.value, str) and "\n" not in node.value:
            # A path literal into a product root would be a read of product code or output. Prose (a docstring
            # or a multi-line description) may name the roots it does not read.
            assert not node.value.startswith(PRODUCT_ROOTS), f"{generator} names a product path: {node.value}"
    assert imported <= set(sys.stdlib_module_names), f"{generator} imports outside the standard library: {imported}"
