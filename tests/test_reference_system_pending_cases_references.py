"""Reference-system pending-case references (`validation/references/reference_system_pending_cases/`).

The generator rebuilds `references.json` and `SHA256SUMS` in memory from the specification's inputs and compares
them byte for byte with the committed files; it also checks its transcription against the specification and its
RS-SUS weight against the existing `reference_system` package. Exit criterion 2 asks that the generator import no
product code: it may import only the standard library, and may name no product path.
"""
import ast
import hashlib
import re
import subprocess
import sys
from pathlib import Path

PROJECT = Path(__file__).resolve().parents[1]
REFERENCE = PROJECT / "validation" / "references" / "reference_system_pending_cases"
GENERATOR = REFERENCE / "generators" / "rs_pending_reference.py"

PRODUCT_ROOTS = {"core", "apps", "tests", "tools", "fixtures", "schemas"}
# The generator's one input is the reference-system specification, read by design as the source of every value
# (DEL-09-01 owns it; it is the invented system's statement, not product code). That literal, and the bare
# "fixtures" segment the generator joins to form the same path, are the only product-root names allowed.
ALLOWED_LITERALS = {"fixtures/reference_system/SPECIFICATION.md", "fixtures"}
FORBIDDEN_CALLS = {"__import__", "import_module", "exec", "eval", "spec_from_file_location"}


def _tree():
    return ast.parse(GENERATOR.read_text(encoding="utf-8"))


def test_generator_check_rebuilds_the_committed_references():
    result = subprocess.run([sys.executable, "-I", "-B", str(GENERATOR)], cwd=GENERATOR.parent,
                            capture_output=True, text=True, timeout=600)
    assert result.returncode == 0, result.stdout + result.stderr
    assert "0 disagreeing" in result.stdout


def test_sha256sums_verify():
    listed = {}
    for line in (REFERENCE / "SHA256SUMS").read_text().splitlines():
        digest, path = line.split("  ", 1)
        listed[path] = digest
    assert set(listed) == {"README.md", "generators/rs_pending_reference.py", "references.json"}
    for path, digest in listed.items():
        assert hashlib.sha256((REFERENCE / path).read_bytes()).hexdigest() == digest, path


def test_generator_imports_only_the_standard_library():
    modules = set()
    for node in ast.walk(_tree()):
        if isinstance(node, ast.Import):
            modules |= {alias.name.split(".")[0] for alias in node.names}
        elif isinstance(node, ast.ImportFrom):
            assert node.level == 0, "relative import"
            modules.add(node.module.split(".")[0])
    assert modules, "no imports found"
    assert modules <= set(sys.stdlib_module_names), sorted(modules - set(sys.stdlib_module_names))


def test_generator_makes_no_dynamic_import_or_evaluation():
    called = set()
    for node in ast.walk(_tree()):
        if isinstance(node, ast.Call):
            func = node.func
            name = func.id if isinstance(func, ast.Name) else func.attr if isinstance(func, ast.Attribute) else None
            if name in FORBIDDEN_CALLS:
                called.add(name)
    assert not called, sorted(called)


def names_product_path(literal):
    """A literal names a product path when it is not empty, contains no whitespace, and one of its segments,
    split on runs of either separator, is a product root."""
    if not literal or re.search(r"\s", literal):
        return False
    return any(segment in PRODUCT_ROOTS for segment in re.split(r"[\\/]+", literal))


def test_generator_names_no_product_path():
    found = sorted({node.value for node in ast.walk(_tree())
                    if isinstance(node, ast.Constant) and isinstance(node.value, str)
                    and names_product_path(node.value) and node.value not in ALLOWED_LITERALS})
    assert not found, found
