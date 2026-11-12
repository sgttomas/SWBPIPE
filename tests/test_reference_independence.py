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
    # the dynamics lane's second review (64da5e92e): a path inside quotation marks, a command, an
    # assignment, parentheses, a closing segment, and a path inside a sentence
    'cat "core/x.py"', "open('core/x.py').read()", "x=core/y", "(tools/a)", "a/tests", "see a/fixtures, then stop",
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
    # the dynamics lane's second review: a root inside a longer name is not a segment
    "my-core/x", "x.core/y", "a/core.py", "a/tools-old/b", "N/m",
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


@pytest.mark.parametrize("source, expected", [
    # a path given as bytes
    ("data = open(b'core/x.py', 'rb').read()\n", ["literal"]),
    # a path inside quotation marks in a command, and the module that starts programs
    ("import subprocess\nsubprocess.run('cat \"core/x.py\"', shell=True)\n", ["literal", "stdlib"]),
    # a facility reached by its name as text, through the built-ins module
    ("import builtins\ngetattr(builtins, 'eval')(\"open('core/x.py').read()\")\n", ["facility", "literal", "stdlib"]),
    # the same through a namespace
    ("f = globals()['__builtins__']\n", ["facility"]),
    # a program started through an allowed module, by attribute and by name as text
    ("import os\nos.system('true')\n", ["facility"]),
    ("import os\nrun = getattr(os, 'popen')\n", ["facility"]),
    # a standard-library module the generators do not use
    ("import ctypes\n", ["stdlib"]),
])
def test_the_independence_rule_catches_ordinary_access_in_other_forms(source, expected):
    assert sorted(f.kind for f in RI.independence_findings(source, "probe.py")) == expected


@pytest.mark.parametrize("module", ["subprocess", "multiprocessing", "ctypes", "platform", "shutil", "builtins", "code",
                                    "pickle", "socket"])
def test_a_standard_library_module_off_the_allowed_list_is_refused(module):
    assert ("stdlib", module) in kinds(f"import {module}")


def test_the_import_list_is_the_decided_twenty_two():
    assert RI.ALLOWED_STDLIB == frozenset(RI.ALLOWED_STDLIB_REASONS) == {
        "argparse", "concurrent", "decimal", "fractions", "hashlib", "json", "math", "os", "pathlib", "struct", "sys", "time",
        "__future__", "ast", "collections", "copy", "dataclasses", "itertools", "random", "re", "tempfile", "traceback"}
    assert RI.BY_SITE_STDLIB == {"subprocess", "multiprocessing", "platform", "importlib", "shutil"}
    assert not RI.ALLOWED_STDLIB & RI.BY_SITE_STDLIB


def test_every_allowed_standard_library_module_carries_its_reason_and_none_loads_code():
    reasons = RI.ALLOWED_STDLIB_REASONS
    assert all(reason.strip() for reason in reasons.values())
    assert not RI.ALLOWED_STDLIB & (RI.FORBIDDEN_MODULES | RI.BY_SITE_STDLIB | {"ctypes", "builtins", "code", "codeop",
                                                                             "pickle", "marshal", "socket"})
    assert RI.ALLOWED_STDLIB <= frozenset(sys.stdlib_module_names)


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


# ---------------------------------------------------------------------------------- the third reading

def test_a_root_in_another_case_is_flagged():
    """Closed by one line: CORE/x.py is the same directory as core/x.py on a case-insensitive host."""
    assert ("literal", "CORE/x.py") in kinds('open("CORE/x.py")')
    assert ("literal", "CORE/x.py") in kinds('open(b"CORE/x.py")')
    assert RI.names_product_path("Tests") and RI.names_product_path("a/Fixtures/b")


@pytest.mark.parametrize("source, name", [
    ("import os\nos._execvpe('true', ['true'])\n", "_execvpe"),
    ("from concurrent.futures import process\nprocess.mp.util.spawnv_passfds('x', ['x'], ())\n", "spawnv_passfds"),
    ("import sys\nsys.modules['importlib.machinery'].SourcelessFileLoader('x', 'x.pyc')\n", "SourcelessFileLoader"),
])
def test_the_named_internal_facilities_found_by_the_third_reading_are_caught(source, name):
    assert ("facility", name) in kinds(source)


def test_a_units_text_that_looks_like_a_path_is_a_deliberate_finding():
    """A false finding kept on purpose: "tests/s" is reported, and allowed with its reason where it occurs."""
    assert ("literal", "tests/s") in kinds('units = "tests/s"\n')


def test_known_gap_a_module_reading_its_own_docstring():
    """Known gap (a): docstrings are inert, but a module that names __doc__ can read one."""
    assert kinds('"core/x.py"\ndata = open(__doc__).read()\n') == set()
    assert kinds('"eval"\nimport os\nrun = getattr(os, __doc__)\n') == set()


def test_known_gap_an_unnamed_facility_of_an_allowed_module():
    """Known gap (b): a facility an allowed module exposes and FORBIDDEN_NAMES does not name."""
    assert kinds("from concurrent.futures import process\nprocess.mp.Process(target=print).start()\n") == set()


def test_known_gap_a_module_named_inside_a_longer_text():
    """Known gap (c): "import core.solver" handed to a started interpreter is not a product path."""
    assert not RI.names_product_path("import core.solver")


def test_a_sibling_module_is_scanned_by_scan_package_but_not_by_a_single_file_judgment(tmp_path):
    """Form (d): one file's judgment does not follow its imports; scan_package scans every module of the
    package, so a sibling that imports product code is found there."""
    root = package(tmp_path, {"script.py": "import sys, os\nsys.path.insert(0, os.path.dirname(__file__))\nimport helper\n",
                              "helper.py": "import core\n"})
    assert {f.key for f in RI.independence_findings((root / "script.py").read_text(), "script.py")} == {"helper"}
    problems = RI.scan_package(root, RI.Allowances(siblings={"helper": "the script's helper"}))
    assert problems == ["helper.py:1: imports core, neither standard library nor allowed"], problems


# ---------------------------------------------------------------------------------- conditions

@pytest.mark.parametrize("source", [
    "import ast\ntree = ast.parse('x = 1')\nnames = [n for n in ast.walk(tree) if isinstance(n, (ast.Import, ast.ImportFrom))]\n",
    "from ast import parse, walk, Name\n",
    "import ast as a\na.iter_child_nodes(a.parse('x'))\n",
])
def test_ast_is_allowed_for_reading_source(source):
    assert kinds(source) == set()


@pytest.mark.parametrize("source, key", [
    ("import ast\nast.literal_eval('1')\n", ("condition", "ast.literal_eval")),
    ("from ast import literal_eval\n", ("condition", "ast.literal_eval")),
    ("import ast\ncode = compile(ast.parse('x = 1'), 'f', 'exec')\n", ("facility", "compile")),
    ("import ast\nexec(compile(ast.parse('x = 1'), 'f', 'exec'))\n", ("facility", "exec")),
])
def test_ast_beyond_reading_is_a_finding(source, key):
    """The ast allowance is of reading: compile and exec stay refused, and so does anything else of ast."""
    assert key in kinds(source)


@pytest.mark.parametrize("source", [
    "import random\nrng = random.Random(20261101)\nx = rng.random()\n",
    "from random import Random\nrng = Random(seed=3)\n",
    "import random as r\nrng = r.Random(7)\n",
])
def test_a_seeded_random_generator_is_allowed(source):
    assert kinds(source) == set()


@pytest.mark.parametrize("source, key", [
    ("import random\nx = random.random()\n", "random.random"),
    ("import random\nrandom.seed(1)\n", "random.seed"),
    ("import random\nx = random.choice([1, 2])\n", "random.choice"),
    ("import random\nrng = random.SystemRandom()\n", "random.SystemRandom"),
    ("import random\nrng = random.Random()\n", "random.Random()"),
    ("from random import Random\nrng = Random()\n", "random.Random()"),
    ("from random import shuffle\n", "random.shuffle"),
    ("from random import SystemRandom\n", "random.SystemRandom"),
])
def test_an_unseeded_random_use_is_a_finding(source, key):
    """A reference is reproducible or it is not a reference."""
    assert ("condition", key) in kinds(source)
