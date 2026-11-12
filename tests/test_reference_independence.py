"""Witnesses for the one independence rule (tools/validation/reference_independence.py).

Each test is named for the bypass it closes. Every witness builds its own synthetic package under
pytest's tmp_path; none reads a reference package or runs a generator, and no product binary is
needed.
"""
from __future__ import annotations

import hashlib
from pathlib import Path
import sys

import pytest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "tools" / "validation"))
import reference_independence as RI  # noqa: E402


def package(tmp_path: Path, files: dict[str, str | bytes]) -> Path:
    root = tmp_path / "pkg"
    for rel, text in files.items():
        path = root / rel
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(text if isinstance(text, bytes) else text.encode("utf-8"))
    return root


def kinds(source: str) -> set[tuple[str, str]]:
    return {(f.kind, f.key) for f in RI.independence_findings(source, "g.py")}


# ---------------------------------------------------------------------------------- product paths

@pytest.mark.parametrize("literal", [
    "core/x.py", "../core/x.py", "/Users/someone/repo/tools/a.py", "C:\\repo\\fixtures\\a.json", "core",
    "schemas//x.json", "api/v1/x.json",
    "core/my file.py",                  # whitespace inside a path segment
    "/Users/a b/x/core/y.py",           # an absolute path through a directory with a space
    "/Volumes/My Drive/repo/core/x.py",
    "run core/x.py now",                # a path token inside prose
    "core/ is not read",                # prose that begins with a root and a separator
    "core.solver.modal",                # a dotted module name under a product root
    "tests.py",
    b"core/x.py", b"tools\\a.py",       # bytes constants
    # the dynamics lane's (87f7c87c4)
    "fixtures", "..\\apps\\desktop", "validation/../tests/a.py", "a/tools/b",
    "core.model_operations.audit_trail.engine", "tools.ci",
    # the hanger lane's (50f279ef3)
    "core/analysis/x.py", "/abs/repo/fixtures/results/a.json", "apps\\desktop\\x.ts",
    "/Users/someone/My Project/core/x.py", b"../schemas/model.schema.yaml",
])
def test_a_product_path_literal_is_flagged(literal):
    assert RI.names_product_path(literal)


@pytest.mark.parametrize("literal", [
    "", "the core of it", "a core of steel", "prose that mentions core and tools", "score/card",
    "validation/references/t4_u6/oracle.json", "coreutils", "scores.core", b"score/card",
    "core-x/y.py", "my.core.module",
    # the dynamics lane's
    "validation/references/f1_modal", "generators/generate.py", "core_values", "hardcore/x", "testsuite",
    "numerics.core", "the tests of this package", "a.b",
    # the hanger lane's
    "ex_sizing.json", "validation/references/hanger_sizing", b"ex_sizing.json",
])
def test_prose_and_near_misses_are_not_flagged(literal):
    assert not RI.names_product_path(literal)


@pytest.mark.parametrize("value, key", [
    ("short", "short"),
    ("x" * 121, "sha256:" + hashlib.sha256(b"x" * 121).hexdigest()),
    (b"core/x.py", "core/x.py"),
])
def test_a_literal_key_is_exact(value, key):
    assert RI.literal_key(value) == key


def test_a_bare_root_name_is_flagged():
    """The `core` of `Path("core") / "x.py"`: a root standing alone is a path segment."""
    assert ("literal", "core") in kinds('from pathlib import Path\np = Path("core") / "x.py"\n')


def test_a_bytes_literal_naming_a_product_path_is_flagged():
    assert RI.names_product_path(b"core/x.py") and RI.names_product_path(b"../schemas/model.schema.yaml")


def test_prose_that_mentions_a_root_inside_a_sentence_is_accepted():
    for text in ("a core of steel", "the tests of this package", "prose that mentions core and tools"):
        assert not RI.names_product_path(text), text


def test_prose_carrying_a_root_path_token_is_flagged():
    """Where the two lanes differed, the stricter rule holds: a whitespace-separated token that is itself a
    path is tested, so these two, which the hanger lane's own rule accepted, are flagged."""
    assert RI.names_product_path("nothing under core/ is read")
    assert RI.names_product_path("reads no product code (core/, apps/, tests/)")


def test_bytes_constants_are_scanned():
    assert ("literal", "core/x.py") in kinds('open(b"core/x.py")')


def test_a_path_with_whitespace_is_scanned():
    assert ("literal", "core/my file.py") in kinds('open("core/my file.py")')


def test_a_docstring_is_inert():
    """A string standing alone as a statement is never evaluated, so it cannot read anything; the rule
    deliberately does not scan it. The same text bound to a name is scanned."""
    assert kinds('"""Reads nothing under core/ at all."""\n') == set()
    assert kinds('def f():\n    "core/x.py"\n') == set()
    assert ("literal", "Reads core/x.py") in kinds('NOTE = "Reads core/x.py"\n')


# ---------------------------------------------------------------------------------- facilities

@pytest.mark.parametrize("source, key", [
    ("import importlib", "importlib"),
    ("import importlib.util", "importlib"),
    ("from importlib import util", "importlib"),
    ("import runpy", "runpy"),
    ("import imp", "imp"),
    ("import pkgutil", "pkgutil"),
    ("import zipimport", "zipimport"),
])
def test_the_import_machinery_is_refused_as_an_import(source, key):
    assert ("facility", key) in kinds(source)


@pytest.mark.parametrize("source, key", [
    ("exec('x = 1')", "exec"),
    ("eval('1')", "eval"),
    ("__import__('os')", "__import__"),
    ("compile('1', 'f', 'eval')", "compile"),
    ("run = exec", "exec"),                                         # a bare reference
    ("import importlib.util as u\nu.spec_from_file_location('m', 'p')", "spec_from_file_location"),
    ("from importlib import import_module as load", "import_module"),  # an import alias
    ("from builtins import exec as run", "exec"),
    ("import builtins\nbuiltins.compile('1', 'f', 'eval')", "compile"),
    ("import builtins\ngetattr(builtins, 'exec')('x = 1')", "exec"),   # named in a string
    ("import sys\nsys.modules['importlib']", "importlib"),
    ("spec.loader.exec_module(m)", "exec_module"),
])
def test_a_dynamic_facility_is_refused_wherever_it_is_named(source, key):
    assert ("facility", key) in kinds(source)


def test_re_compile_is_not_the_builtin_compile():
    """`compile` is refused as the builtin only: re.compile is an attribute named compile."""
    assert kinds("import re\nPATTERN = re.compile('a+')") == set()
    assert ("facility", "compile") in kinds("code = compile('1', 'f', 'eval')")


def test_a_relative_import_is_refused():
    assert ("relative-import", "helper") in kinds("from .helper import x")


def test_a_product_module_import_is_refused():
    assert ("import", "core") in kinds("import core.solver")
    assert ("import", "tools") in kinds("from tools.validation import reference_independence")


# ---------------------------------------------------------------------------------- packages

def test_the_independence_rule_catches_each_kind(tmp_path):
    root = package(tmp_path, {"generators/g.py": "\n".join([
        "import json",
        "import numpy",
        "import importlib",
        "from . import helper",
        "x = eval('1')",
        "P = 'fixtures/a.json'",
    ])})
    problems = "\n".join(RI.scan_package(root))
    for text in ("imports numpy", "importlib", "relative import", "names eval", "fixtures/a.json"):
        assert text in problems, (text, problems)
    assert "imports json" not in problems


def test_the_independence_rule_catches_each_kind_in_the_dynamics_lanes_probe():
    source = (
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
        "'a statement string naming core/x.py is read by nothing'\n"
    )
    found = sorted(f.kind for f in RI.independence_findings(source, "probe.py"))
    assert found == ["facility"] * 5 + ["import"] + ["literal"] * 3 + ["relative-import"], found


def test_the_scan_reaches_a_generator_below_the_top_directory(tmp_path):
    root = package(tmp_path, {"a/b/c/deep.py": "import core"})
    assert any("deep.py" in p and "imports core" in p for p in RI.scan_package(root))
    nested = package(tmp_path / "n", {"generators/numerics/__init__.py": "import numpy\n"})
    problems = RI.scan_package(nested)
    assert any("generators/numerics/__init__.py" in p and "numpy" in p for p in problems), problems
    assert any(p.startswith("generators/numerics: a directory under generators/") for p in problems), problems


def test_a_directory_outside_the_rule_is_named_and_not_scanned(tmp_path):
    root = package(tmp_path, {"g.py": "X = 1\n", "check/run_product.py": "import core\n"})
    assert any("imports core" in p for p in RI.scan_package(root))
    assert RI.scan_package(root, RI.Allowances(outside={"check": "a product check, not reference code"})) == []
    assert RI.scan_package(root, RI.Allowances(outside={"check": "why", "gone": "why"})) == [
        "allowance no longer used: outside 'gone'"]


def test_an_allowance_admits_exactly_what_it_names(tmp_path):
    root = package(tmp_path, {"g.py": "import importlib.util\nP = 'fixtures/a.json'\n", "h.py": "import importlib\n"})
    allow = RI.Allowances(facilities={("g.py", "importlib"): "why"}, literals={("g.py", "fixtures/a.json"): "why"})
    problems = RI.scan_package(root, allow)
    assert problems == ["h.py:1: imports the import machinery importlib"]


def test_an_unused_allowance_is_a_finding(tmp_path):
    root = package(tmp_path, {"g.py": "import json\n"})
    problems = RI.scan_package(root, RI.Allowances(siblings={"gone": "why"}, literals={("g.py", "core"): "why"}))
    assert sorted(problems) == ["allowance no longer used: literals ('g.py', 'core')",
                                "allowance no longer used: siblings 'gone'"]


def test_a_sibling_must_be_a_module_of_the_package(tmp_path):
    root = package(tmp_path, {"g.py": "import helper\n"})
    assert any("not a module of the package" in p for p in RI.scan_package(root, RI.Allowances(siblings={"helper": "why"})))
    root = package(tmp_path / "ok", {"g.py": "import helper\n", "lib/helper.py": "X = 1\n"})
    assert RI.scan_package(root, RI.Allowances(siblings={"helper": "why"})) == []


def _foreign_source(tmp_path: Path, listed: bool = True, tamper: bool = False) -> RI.Foreign:
    other = package(tmp_path / "refs", {"other/engine.py": "X = 1\n"})
    engine = other / "other" / "engine.py"
    digest = hashlib.sha256(engine.read_bytes()).hexdigest()
    sums = other / "other" / "SHA256SUMS"
    sums.write_text(f"{digest}  engine.py\n" if listed else f"{'0' * 64}  unrelated.py\n", encoding="utf-8")
    if tamper:
        engine.write_text("X = 2\n", encoding="utf-8")
    return RI.Foreign(engine, sums, sums.parent, "the other package's engine")


def test_an_admitted_reference_import_is_a_file_its_own_package_pins(tmp_path):
    importer = {"g.py": "import engine\n"}
    ok = RI.Allowances(foreign={"engine": _foreign_source(tmp_path / "a")})
    assert RI.scan_package(package(tmp_path / "a", importer), ok) == []
    unlisted = RI.Allowances(foreign={"engine": _foreign_source(tmp_path / "b", listed=False)})
    assert any("not listed" in p for p in RI.scan_package(package(tmp_path / "b", importer), unlisted))
    changed = RI.Allowances(foreign={"engine": _foreign_source(tmp_path / "c", tamper=True)})
    assert any("differs from its line" in p for p in RI.scan_package(package(tmp_path / "c", importer), changed))


def test_a_file_beside_the_importer_cannot_shadow_an_admitted_reference_import(tmp_path):
    allow = RI.Allowances(foreign={"engine": _foreign_source(tmp_path)})
    root = package(tmp_path, {"g.py": "import engine\n", "engine.py": "X = 'shadow'\n"})
    assert any("shadows" in p for p in RI.scan_package(root, allow))


def test_a_generators_directory_holds_only_modules(tmp_path):
    root = package(tmp_path, {"generators/g.py": "X = 1\n", "generators/vendor/lib.py": "X = 1\n"})
    assert any("generators/vendor" in p for p in RI.scan_package(root))
    allow = RI.Allowances(directories={"generators/vendor": "why"})
    assert RI.scan_package(root, allow) == []
