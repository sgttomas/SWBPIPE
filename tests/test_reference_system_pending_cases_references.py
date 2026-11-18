"""Reference-system pending-case references (`validation/references/reference_system_pending_cases/`).

The generator rebuilds `references.json` and `SHA256SUMS` in memory from the specification's inputs and compares
them byte for byte with the committed files; it also checks its transcription against the specification and its
RS-SUS weight against the existing `reference_system` package. Exit criterion 2 asks that the generator import no
product code: it may import only the standard library, and may name no product path.

The specification has since adopted RS-R1 and RS-CE1 and cleared their ledger rows, one of which the frozen generator
quotes. Its check therefore runs on the specification as it was generated from, staged by
`tests/reference_packages.py` (the package's entry). The current specification must still state every input the
generator transcribes: its tables parse to the same values, and every quoted statement but that one row is present.
"""
import ast
import hashlib
import importlib.util
import re
import sys
from pathlib import Path

import reference_packages as R

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


CURRENT_SPECIFICATION = PROJECT / "fixtures" / "reference_system" / "SPECIFICATION.md"
# The one quoted statement adoption retired: RS-R1's §10 row while it was pending.
ADOPTED_ROW = ("| Eccentric reducer RS-R1 in region RG-R | pending; RS-P150 is in tranche 1 as a plain S270 member in its "
               "own region RG-R |")


def test_generator_check_rebuilds_the_committed_references(tmp_path):
    problems = R.check_problems("reference_system_pending_cases", 0, tmp_path / "check_output")
    assert not problems, "\n".join(problems)


def test_the_current_specification_states_every_transcribed_input_but_the_adopted_row():
    spec = importlib.util.spec_from_file_location("rs_pending_reference_read", GENERATOR)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    module.SPEC = CURRENT_SPECIFICATION
    module.CROSS.clear()
    module.check_transcription()
    tables = [c for c in module.CROSS if c["check"].startswith(("§3.1", "§3.2", "§4.2"))]
    assert len(tables) == 3 and all(c["agrees"] for c in tables), tables
    text = CURRENT_SPECIFICATION.read_text(encoding="utf-8")
    assert [s for s in module.SPEC_SENTENCES if s not in text] == [ADOPTED_ROW]


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
