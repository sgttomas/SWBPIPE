"""Reference-system pending-case references (`validation/references/reference_system_pending_cases/`).

The generator rebuilds `references.json` and `SHA256SUMS` in memory from the specification's inputs and compares
them byte for byte with the committed files; it also checks its transcription against the specification and its
RS-SUS weight against the existing `reference_system` package. Exit criterion 2 asks that the generator import no
product code: it may import only the standard library.
"""
import ast
import hashlib
import subprocess
import sys
from pathlib import Path

PROJECT = Path(__file__).resolve().parents[1]
REFERENCE = PROJECT / "validation" / "references" / "reference_system_pending_cases"
GENERATOR = REFERENCE / "generators" / "rs_pending_reference.py"


def test_generator_check_rebuilds_the_committed_references():
    result = subprocess.run([sys.executable, "-I", str(GENERATOR)], cwd=GENERATOR.parent,
                            capture_output=True, text=True, timeout=600)
    assert result.returncode == 0, result.stdout + result.stderr
    assert "0 disagreeing" in result.stdout


def test_sha256sums_verify():
    lines = (REFERENCE / "SHA256SUMS").read_text().splitlines()
    listed = {}
    for line in lines:
        digest, path = line.split("  ", 1)
        listed[path] = digest
    assert set(listed) == {"README.md", "generators/rs_pending_reference.py", "references.json"}
    for path, digest in listed.items():
        assert hashlib.sha256((REFERENCE / path).read_bytes()).hexdigest() == digest, path


def test_generator_imports_only_the_standard_library():
    tree = ast.parse(GENERATOR.read_text(encoding="utf-8"))
    modules = set()
    for node in ast.walk(tree):
        if isinstance(node, ast.Import):
            modules |= {alias.name.split(".")[0] for alias in node.names}
        elif isinstance(node, ast.ImportFrom):
            assert node.level == 0, "relative import"
            modules.add(node.module.split(".")[0])
        elif isinstance(node, ast.Call) and getattr(node.func, "id", None) == "__import__":
            raise AssertionError("dynamic import")
    assert modules, "no imports found"
    assert modules <= set(sys.stdlib_module_names), sorted(modules - set(sys.stdlib_module_names))
