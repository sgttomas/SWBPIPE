"""The frozen modal and response-spectrum references regenerate from generators that reach no product code.

DEL-04-09 AC-009 and DEL-04-10 AC-006: each frozen reference under
`validation/references/f1_modal` and `validation/references/f2_spectrum`
regenerates byte for byte from its generator, and the generator imports no
product code.

Threat model: a generator that imports, opens or names a product path in the
ordinary way. For every generator file, at any depth under `generators/`,
this checks its imports (standard library other than the import machinery,
its own directory, or a named module of another frozen reference package
whose bytes that package's SHA256SUMS pins and that nothing beside the
importer shadows), that it has no relative import, that it neither imports
nor names a dynamic import or evaluation facility under any alias, and that
no string constant names a product path or a product module. Outside the
model, recorded and not repaired: a path or module name assembled at run
time to hide its segments.

The scan rule here is moving: one module, `tools/validation/reference_independence.py`,
is to hold it for every reference package's test, and this file then imports it.
"""
import ast
import hashlib
from pathlib import Path
import re
import subprocess
import sys

import pytest

# The directory scans below, for the hosted selection of this test (tests/test_ci_python.py). The
# package directories are scanned through one root, so the declared read is that root.
CI_READS = (
    "validation/references/**",
    "core/solver/modal_eigen/tests/fixtures/*.py",
)

PROJECT = Path(__file__).resolve().parents[1]
REFERENCES = PROJECT / "validation/references"

# Each package and the generator whose `--check` regenerates it in memory and compares byte for byte.
PACKAGES = {
    "f1_modal/continuum": "generators/generate.py",
    "f1_modal/mass_discrete": "generators/f1_mass_discrete.py",
    "f2_spectrum/closed_form": "generators/generate.py",
    "f2_spectrum/discrete": "generators/f2_spectrum_discrete.py",
    "f2_spectrum/benchmark_pack": "generators/generate.py",
}

# A generator may import a module of another frozen reference package: reference code reading
# reference code. Each is admitted by name, and its bytes are the ones its own package's
# SHA256SUMS pins.
ADMITTED_REFERENCE_IMPORTS = {
    "f2_spectrum/discrete": {
        "f1_modal/mass_discrete": {"elements", "eigen", "model", "numeric"},
    },
    "f2_spectrum/benchmark_pack": {
        "f2_spectrum/closed_form": {"numerics", "rules", "continuum", "fe", "beams"},
    },
}

# Scripts outside the packages that derive test fixtures from the modal references, read-only.
FIXTURE_SCRIPTS = {
    "core/solver/modal_eigen/tests/fixtures/reformation_cases.py": {"f1_modal/mass_discrete": {"model", "numeric"}},
    "core/solver/modal_eigen/tests/fixtures/sturm_pencils.py": {"f1_modal/mass_discrete": {"model", "numeric"}},
}

PRODUCT_ROOTS = {"core", "apps", "api", "tests", "tools", "fixtures", "schemas"}
# Facilities that load or evaluate code named at run time. A generator may not name one, as a call,
# a reference or an import alias, and may not import the modules that provide them.
FORBIDDEN_NAMES = {
    "__import__", "import_module", "exec", "eval", "compile", "exec_module", "load_module", "run_path", "run_module",
    "spec_from_file_location", "spec_from_loader", "module_from_spec", "SourceFileLoader",
}
FORBIDDEN_MODULES = {"importlib", "runpy", "imp", "pkgutil", "zipimport"}


def _has_product_segment(text):
    return bool(PRODUCT_ROOTS & set(re.split(r"[\\/]+", text)))


def names_product_path(text):
    """A string constant names a product path when a path segment is a product root: of the whole
    string, so that a directory name with a space in it does not hide the path, or of any
    whitespace-separated token that is itself a path. It names a product module when it is a dotted
    name whose first part is a product root."""
    if not text:
        return False
    if _has_product_segment(text):
        return True
    if any(re.search(r"[\\/]", token) and _has_product_segment(token) for token in text.split()):
        return True
    return bool(re.fullmatch(r"[A-Za-z_]\w*(\.[A-Za-z_]\w*)+", text)) and text.split(".")[0] in PRODUCT_ROOTS


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def checksum_list(package):
    listed = {}
    for line in (REFERENCES / package / "SHA256SUMS").read_text(encoding="utf-8").splitlines():
        digest, name = line.split(maxsplit=1)
        listed[name.strip()] = digest
    return listed


def package_files(package):
    root = REFERENCES / package
    return sorted(
        path.relative_to(root).as_posix()
        for path in root.rglob("*")
        if path.is_file() and "__pycache__" not in path.parts and path.name != "SHA256SUMS"
    )


def generator_files(package):
    return sorted(path for path in (REFERENCES / package / "generators").rglob("*.py") if "__pycache__" not in path.parts)


def independence_findings(path, admitted):
    """Every departure of one generator file from the rules, as (kind, line, text)."""
    tree = ast.parse(path.read_text(encoding="utf-8"), filename=str(path))
    local = {sibling.stem for sibling in path.parent.glob("*.py")}
    admitted_names = set().union(*admitted.values()) if admitted else set()
    # A string that stands alone as a statement (a docstring) is read by nothing.
    inert = {id(node.value) for node in ast.walk(tree) if isinstance(node, ast.Expr) and isinstance(node.value, ast.Constant)}
    found = []

    def judge_import(name, line):
        top = name.split(".")[0]
        if top in FORBIDDEN_MODULES:
            found.append(("import machinery", line, name))
        elif not (top in sys.stdlib_module_names or top in local or top in admitted_names):
            found.append(("import", line, name))

    for node in ast.walk(tree):
        if isinstance(node, ast.Constant) and isinstance(node.value, str):
            if id(node) not in inert and names_product_path(node.value):
                found.append(("product path", node.lineno, node.value))
        elif isinstance(node, ast.Import):
            for alias in node.names:
                judge_import(alias.name, node.lineno)
        elif isinstance(node, ast.ImportFrom):
            if node.level:
                found.append(("relative import", node.lineno, node.module or ""))
            else:
                judge_import(node.module or "", node.lineno)
                for alias in node.names:
                    if alias.name in FORBIDDEN_NAMES:
                        found.append(("dynamic facility", node.lineno, alias.name))
        elif isinstance(node, ast.Name) and node.id in FORBIDDEN_NAMES:
            found.append(("dynamic facility", node.lineno, node.id))
        elif isinstance(node, ast.Attribute) and node.attr in FORBIDDEN_NAMES:
            found.append(("dynamic facility", node.lineno, node.attr))
    return found


@pytest.mark.parametrize("package", sorted(PACKAGES))
def test_the_references_regenerate_byte_for_byte(package):
    root = REFERENCES / package
    subprocess.run([sys.executable, "-I", "-B", str(root / PACKAGES[package]), "--check"], cwd=root, check=True, timeout=900)


@pytest.mark.parametrize("package", sorted(PACKAGES))
def test_the_checksum_list_covers_and_matches_every_file(package):
    listed = checksum_list(package)
    assert sorted(listed) == package_files(package)
    wrong = [name for name, digest in listed.items() if sha256(REFERENCES / package / name) != digest]
    assert not wrong, wrong


@pytest.mark.parametrize("package", sorted(PACKAGES))
def test_the_generators_reach_no_product_code(package):
    files = generator_files(package)
    assert REFERENCES / package / PACKAGES[package] in files
    admitted = ADMITTED_REFERENCE_IMPORTS.get(package, {})
    found = {path.name: independence_findings(path, admitted) for path in files}
    assert not any(found.values()), {name: items for name, items in found.items() if items}


@pytest.mark.parametrize("script", sorted(FIXTURE_SCRIPTS))
def test_the_fixture_scripts_reach_only_the_references(script):
    assert not independence_findings(PROJECT / script, FIXTURE_SCRIPTS[script])


def test_an_admitted_reference_import_is_a_file_its_own_package_pins():
    admitted = dict(ADMITTED_REFERENCE_IMPORTS)
    admitted.update({f"script:{name}": sources for name, sources in FIXTURE_SCRIPTS.items()})
    for importer, sources in admitted.items():
        for source, modules in sources.items():
            listed = checksum_list(source)
            for module in sorted(modules):
                name = f"generators/{module}.py"
                assert name in listed, (importer, source, module)
                assert sha256(REFERENCES / source / name) == listed[name], (importer, source, module)
                # The admitted name must not be shadowed by a file or a package beside the importer.
                if importer.startswith("script:"):
                    beside = (PROJECT / importer.removeprefix("script:")).parent
                else:
                    beside = REFERENCES / importer / "generators"
                assert not (beside / f"{module}.py").exists(), (importer, module)
                assert not (beside / module).exists(), (importer, module)


@pytest.mark.parametrize("package", sorted(PACKAGES))
def test_a_generators_directory_holds_only_modules(package):
    """No package directory under `generators/`: one would be importable by name ahead of a pinned module."""
    nested = [path.name for path in (REFERENCES / package / "generators").iterdir() if path.is_dir() and path.name != "__pycache__"]
    assert not nested, nested


@pytest.mark.parametrize(
    "text",
    [
        "core", "core/x.py", "../core/x.py", "..\\apps\\desktop", "validation/../tests/a.py", "fixtures", "a/tools/b", "api/x",
        "/Users/a b/checkout/core/x.py", "run core/x.py now", "core.model_operations.audit_trail.engine", "tools.ci",
    ],
)
def test_the_path_rule_flags_a_product_segment_wherever_it_stands(text):
    assert names_product_path(text)


@pytest.mark.parametrize(
    "text",
    [
        "", "validation/references/f1_modal", "generators/generate.py", "a core of steel", "core_values", "hardcore/x", "testsuite",
        "numerics.core", "the tests of this package", "a.b",
    ],
)
def test_the_path_rule_passes_what_names_no_product_root(text):
    assert not names_product_path(text)


def test_the_independence_rule_catches_each_kind(tmp_path):
    generator = tmp_path / "generators" / "probe.py"
    generator.parent.mkdir()
    generator.write_text(
        "import json\n"
        "import numpy\n"
        "from . import sibling\n"
        "from pathlib import Path\n"
        "p = Path('core') / 'x.py'\n"
        "q = Path('../fixtures/y.json')\n"
        "m = __import__('os')\n"
        "import importlib\n"
        "n = importlib.import_module('sys')\n"
        "from importlib import import_module as load\n"
        "k = load('core.model_operations')\n"
        "'a statement string naming core/x.py is read by nothing'\n",
        encoding="utf-8",
    )
    found = independence_findings(generator, {})
    kinds = sorted(kind for kind, _, _ in found)
    assert kinds == [
        "dynamic facility", "dynamic facility", "dynamic facility", "import", "import machinery", "import machinery",
        "product path", "product path", "product path", "relative import",
    ], found


def test_the_scan_reaches_a_generator_below_the_top_directory(tmp_path, monkeypatch):
    nested = tmp_path / "pkg" / "generators" / "numerics"
    nested.mkdir(parents=True)
    (nested / "__init__.py").write_text("import numpy\n", encoding="utf-8")
    monkeypatch.setattr(sys.modules[__name__], "REFERENCES", tmp_path)
    assert [path.name for path in generator_files("pkg")] == ["__init__.py"]
