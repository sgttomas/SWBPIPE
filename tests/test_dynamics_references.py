"""The frozen modal and response-spectrum references regenerate from generators that reach no product code.

DEL-04-09 AC-009 and DEL-04-10 AC-006: each frozen reference under
`validation/references/f1_modal` and `validation/references/f2_spectrum`
regenerates byte for byte from its generator, and the generator imports no
product code.

Two checks of different kinds answer this.

**The observation carries the criterion.** Each package's generator is run
twice with a hook in every interpreter the run starts, worker processes included
(`tools/validation/reference_observation.py`): in check mode where it stands,
which regenerates every file and compares it byte for byte; and in write mode in
a scratch copy of the reference packages, whose files are then compared byte for
byte with the committed ones, so that the run that produces the values is itself
observed and nothing committed is rewritten. A file opened, a directory listed or
a module loaded inside a product root, or a program started other than a worker
of the generator itself, fails the test, whatever text asked for it; so does a
worker whose hook did not report. Its limits:

- It observes what those two runs execute. One branch runs in neither:
  `--search-round2` of `f1_modal/continuum`, which repeats a search whose result
  the package holds as a constant.
- The two fixture scripts outside the packages are run by no check here. They
  are under the scan alone.
- It shows that a run reached no product code. It does not show that a reference
  was derived independently; each package's refutation record does that.

**The scan is the first, readable line.** It reads ordinary statements and
catches the dependence an author writes plainly: an import, a path or a module
named in a text or bytes constant, a facility that loads or evaluates code or
starts a program. It is not what carries the criterion. Deliberate indirection
is outside the scan's model and inside the observation's. For every generator
file, at any depth under `generators/`, the scan checks its imports (a named
list of standard-library modules, its own directory, or a named module of
another frozen reference package whose bytes that package's SHA256SUMS pins and
that nothing beside the importer shadows), that it has no relative import, that
it names no such facility under any alias or as a constant, and that no constant
names a product path or a product module. A generator may run workers of itself
through `concurrent.futures`.
"""
import ast
import hashlib
import os
from pathlib import Path
import re
import shutil
import sys

import pytest

from tools.validation.reference_observation import _judge, run_observed

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
# The standard-library modules a generator may import: the ones the generators use. Anything else,
# such as a module that starts a program or reaches the interpreter's built-ins, is refused by
# absence from this list.
ALLOWED_STDLIB = {
    "argparse", "concurrent", "decimal", "fractions", "hashlib", "json", "math", "os", "pathlib", "struct", "sys", "time",
}
# Facilities that load or evaluate code named at run time, or start another program. A generator may
# not name one: as a call, a reference, an import alias, or a text or bytes constant (which is how
# `getattr` or a namespace lookup would reach it).
FORBIDDEN_NAMES = {
    "__import__", "__builtins__", "import_module", "exec", "eval", "compile", "exec_module", "load_module", "run_path",
    "run_module", "spec_from_file_location", "spec_from_loader", "module_from_spec", "SourceFileLoader",
    "system", "popen", "startfile", "posix_spawn", "posix_spawnp", "fork", "forkpty",
    "execl", "execle", "execlp", "execlpe", "execv", "execve", "execvp", "execvpe",
    "spawnl", "spawnle", "spawnlp", "spawnlpe", "spawnv", "spawnve", "spawnvp", "spawnvpe",
}
FORBIDDEN_MODULES = {"importlib", "runpy", "imp", "pkgutil", "zipimport"}
# A spawn function, by any spelling of its name: a generator has no call for one.
_SPAWN = re.compile(r"spawn", re.IGNORECASE)

_ROOT = "(?:" + "|".join(sorted(PRODUCT_ROOTS)) + ")"
# A product root standing as a path segment anywhere in a text: opening a segment (nothing of a name
# before it, a separator after it) or closing one (a separator before it, nothing of a name after it).
# Without regard to case: on a file system that ignores case, `CORE` is `core`.
_PRODUCT_SEGMENT = re.compile(rf"(?<![\w.\-]){_ROOT}(?=[\\/])|(?<=[\\/]){_ROOT}(?![\w.\-])", re.IGNORECASE)
# A product module named in a longer text, as a command would name it.
_PRODUCT_MODULE_IN_TEXT = re.compile(
    rf"\bimport\s+{_ROOT}\b|\bfrom\s+{_ROOT}(?:\.\w+)*\s+import\b|(?<!\S)-m\s+{_ROOT}\b", re.IGNORECASE
)


def names_product_path(text):
    """A text names a product path when a product root is one of its path segments, wherever the
    path stands in it and in any letter case: after a space in a directory name, inside quotation
    marks, inside a command. A text that is only a product root names one too. It names a product
    module when it is a dotted name whose first part is a product root, or when it holds an import
    of one. The rule prefers a false finding to an exemption: a unit written `tests/s` is refused,
    and its author rewords it."""
    if not text:
        return False
    if text.lower() in PRODUCT_ROOTS or _PRODUCT_SEGMENT.search(text) or _PRODUCT_MODULE_IN_TEXT.search(text):
        return True
    return bool(re.fullmatch(r"[A-Za-z_]\w*(\.[A-Za-z_]\w*)+", text)) and text.split(".")[0].lower() in PRODUCT_ROOTS


def names_facility(text):
    return text in FORBIDDEN_NAMES or bool(_SPAWN.search(text))


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


def independence_findings(path, admitted, siblings=True):
    """Every departure of one generator file from the rules, as (kind, line, text). With `siblings`
    false the file may import no module beside it: nothing scans the siblings of a script that
    stands outside a package."""
    tree = ast.parse(path.read_text(encoding="utf-8"), filename=str(path))
    local = {sibling.stem for sibling in path.parent.glob("*.py")} if siblings else set()
    admitted_names = set().union(*admitted.values()) if admitted else set()
    # A string that stands alone as a statement (a docstring) is read by nothing, unless the module
    # names `__doc__`, which is how it would read one.
    reads_docstrings = any(
        (isinstance(node, ast.Name) and node.id == "__doc__")
        or (isinstance(node, ast.Attribute) and node.attr == "__doc__")
        or (isinstance(node, ast.Constant) and node.value in ("__doc__", b"__doc__"))
        for node in ast.walk(tree)
    )
    inert = set() if reads_docstrings else {
        id(node.value) for node in ast.walk(tree) if isinstance(node, ast.Expr) and isinstance(node.value, ast.Constant)
    }
    found = []

    def judge_import(name, line):
        top = name.split(".")[0]
        if top in FORBIDDEN_MODULES:
            found.append(("import machinery", line, name))
        elif not (top in ALLOWED_STDLIB or top in local or top in admitted_names):
            found.append(("import", line, name))

    for node in ast.walk(tree):
        if isinstance(node, ast.Constant) and isinstance(node.value, (str, bytes)):
            # Bytes name a path as text does; latin-1 reads every byte.
            text = node.value if isinstance(node.value, str) else node.value.decode("latin-1")
            if id(node) in inert:
                continue
            if names_product_path(text):
                found.append(("product path", node.lineno, text))
            if text in FORBIDDEN_NAMES or (text.isidentifier() and _SPAWN.search(text)):
                found.append(("dynamic facility", node.lineno, text))
        elif isinstance(node, ast.Import):
            for alias in node.names:
                judge_import(alias.name, node.lineno)
        elif isinstance(node, ast.ImportFrom):
            if node.level:
                found.append(("relative import", node.lineno, node.module or ""))
            else:
                judge_import(node.module or "", node.lineno)
                for alias in node.names:
                    if names_facility(alias.name):
                        found.append(("dynamic facility", node.lineno, alias.name))
        elif isinstance(node, ast.Name) and names_facility(node.id):
            found.append(("dynamic facility", node.lineno, node.id))
        elif isinstance(node, ast.Attribute) and names_facility(node.attr):
            found.append(("dynamic facility", node.lineno, node.attr))
    return found


posix_only = pytest.mark.skipif(os.name != "posix", reason="the observation relies on a named pipe")
WATCHED = [PROJECT / root for root in sorted(PRODUCT_ROOTS)]


def observed(script, arguments, cwd, workdir, prefixes=()):
    return run_observed([sys.executable, str(script), *arguments], cwd, workdir, WATCHED, 900, prefixes)


def clean(run):
    assert run.returncode == 0, run.stdout[-2000:] + run.stderr[-2000:]
    assert run.interpreters >= 1, "the run was not observed"
    assert not run.records, run.records


@posix_only
@pytest.mark.parametrize("package", sorted(PACKAGES))
def test_the_references_regenerate_byte_for_byte_and_the_run_reaches_no_product_code(package, tmp_path):
    root = REFERENCES / package
    clean(observed(root / PACKAGES[package], ["--check"], root, tmp_path))


@posix_only
@pytest.mark.parametrize("package", sorted(PACKAGES))
def test_the_run_that_writes_the_references_reaches_no_product_code_and_writes_the_committed_bytes(package, tmp_path):
    """Write mode, in a scratch copy laid out as the project is, so that a path taken from the
    generator's own location to a product root falls under a watched prefix."""
    scratch = tmp_path / "project"
    for family in sorted({name.split("/")[0] for name in PACKAGES}):
        shutil.copytree(REFERENCES / family, scratch / "validation/references" / family, ignore=shutil.ignore_patterns("__pycache__"))
    workdir = tmp_path / "work"
    workdir.mkdir()
    root = scratch / "validation/references" / package
    clean(observed(root / PACKAGES[package], ["--write"], root, workdir, [scratch / name for name in sorted(PRODUCT_ROOTS)]))
    written = sorted(path.relative_to(root).as_posix() for path in root.rglob("*") if path.is_file() and "__pycache__" not in path.parts)
    assert written == sorted(package_files(package) + ["SHA256SUMS"])
    differing = [name for name in written if (root / name).read_bytes() != (REFERENCES / package / name).read_bytes()]
    assert not differing, differing


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
    assert not independence_findings(PROJECT / script, FIXTURE_SCRIPTS[script], siblings=False)


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
        'cat "core/x.py"', "open('core/x.py').read()", "x=core/y", "(tools/a)", "a/tests", "see a/fixtures, then stop",
        "CORE/x.py", "..\\Apps\\desktop", "Tools", "Core.solver", "python3 -c 'import core.solver'", "from tools.ci import x",
        "python3 -m tools.deliverables",
        # A benign text the rule refuses on purpose: an exemption for it would be a way round the rule.
        "tests/s",
    ],
)
def test_the_path_rule_flags_a_product_segment_wherever_it_stands(text):
    assert names_product_path(text)


@pytest.mark.parametrize(
    "text",
    [
        "", "validation/references/f1_modal", "generators/generate.py", "a core of steel", "core_values", "hardcore/x", "testsuite",
        "numerics.core", "the tests of this package", "a.b", "my-core/x", "x.core/y", "a/core.py", "a/tools-old/b", "N/m",
        "an important core idea", "import corelib", "taken from tests of the method", "the-m core option",
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


@pytest.mark.parametrize(
    ("source", "kinds"),
    [
        # A path given as bytes.
        ("data = open(b'core/x.py', 'rb').read()\n", ["product path"]),
        # A path inside quotation marks in a command, and the module that starts programs.
        ("import subprocess\nsubprocess.run('cat \"core/x.py\"', shell=True)\n", ["import", "product path"]),
        # A facility reached by its name as text, through the built-ins module.
        (
            "import builtins\ngetattr(builtins, 'eval')(\"open('core/x.py').read()\")\n",
            ["dynamic facility", "import", "product path"],
        ),
        # The same through a namespace, and a program started through an allowed module.
        ("f = globals()['__builtins__']\n", ["dynamic facility"]),
        ("import os\nos.system('true')\n", ["dynamic facility"]),
        ("import os\nrun = getattr(os, 'popen')\n", ["dynamic facility"]),
        # A standard-library module the generators do not use.
        ("import ctypes\n", ["import"]),
        # A docstring the module reads: a path, and a facility's name.
        ('"core/x.py"\nopen(__doc__)\n', ["product path"]),
        ('"system"\nimport os\ngetattr(os, __doc__)("true")\n', ["dynamic facility"]),
        # The spawn function the observation cannot see, reached through an allowed module.
        (
            "from concurrent.futures import process\nprocess.mp.util.spawnv_passfds(b'/usr/bin/true', [b'true'], [])\n",
            ["dynamic facility"],
        ),
        # A case variant of a product root, and a product module named inside a command.
        ("open('CORE/x.py')\n", ["product path"]),
        ("arguments = ['python3', '-c', 'import core.solver']\n", ["product path"]),
    ],
)
def test_the_independence_rule_catches_ordinary_access_in_other_forms(tmp_path, source, kinds):
    generator = tmp_path / "generators" / "probe.py"
    generator.parent.mkdir()
    generator.write_text(source, encoding="utf-8")
    found = independence_findings(generator, {})
    assert sorted(kind for kind, _, _ in found) == kinds, found


def test_the_scan_reaches_a_generator_below_the_top_directory(tmp_path, monkeypatch):
    nested = tmp_path / "pkg" / "generators" / "numerics"
    nested.mkdir(parents=True)
    (nested / "__init__.py").write_text("import numpy\n", encoding="utf-8")
    monkeypatch.setattr(sys.modules[__name__], "REFERENCES", tmp_path)
    assert [path.name for path in generator_files("pkg")] == ["__init__.py"]


def test_a_script_outside_a_package_may_import_no_sibling(tmp_path):
    script = tmp_path / "script.py"
    script.write_text("import helper\n", encoding="utf-8")
    (tmp_path / "helper.py").write_text("import core\n", encoding="utf-8")
    assert independence_findings(script, {}) == []
    assert independence_findings(script, {}, siblings=False) == [("import", 1, "helper")]


# What the observation must record, each by a form the scan does not read: a path put together at
# run time, a case variant, bytes, a listing, a product module loaded, an access the script catches
# an error around, the access of a worker that then dies, and programs started.
OBSERVATION_PROBE = """
import os
import sys
from concurrent.futures import ProcessPoolExecutor

PROJECT = sys.argv[1]
ROOT = os.path.join(PROJECT, "to" + "ols")


def work(index):
    with open(os.path.join(ROOT, "validation", "qualification_gate.py"), "rb") as handle:
        handle.read(1)
    os._exit(7)


if __name__ == "__main__":
    with open(os.path.join(ROOT, "validation", "reference_observation.py")) as handle:
        handle.read(1)
    with open(os.fsencode(os.path.join(ROOT, "validation", "execution_boundary.py")), "rb") as handle:
        handle.read(1)
    variant = os.path.join(PROJECT, "TOOLS", "validation", "qualification_process.py")
    if os.path.exists(variant):
        with open(variant) as handle:
            handle.read(1)
    try:
        open(os.path.join(ROOT, "no_such_file_here"))
    except OSError:
        pass
    os.listdir(os.path.join(PROJECT, "fix" + "tures"))
    sys.path.insert(0, PROJECT)
    __import__("to" + "ols.validation.reference_observation")
    try:
        with ProcessPoolExecutor(1) as pool:
            list(pool.map(work, [0]))
    except Exception:
        pass
    os.system("true")
    run = __import__("sub" + "process").run
    run(["true"])
    run([sys.executable, "-c", "pass"])
    run([sys.executable, "-I", "-c", "from multiprocessing.spawn import spawn_main"])
    run([sys.executable, "-c", "from multiprocessing.spawn import spawn_main"], env={})
"""


@posix_only
def test_the_observation_records_what_the_scan_does_not_read(tmp_path):
    script = tmp_path / "probe" / "probe.py"
    script.parent.mkdir()
    script.write_text(OBSERVATION_PROBE, encoding="utf-8")
    workdir = tmp_path / "work"
    workdir.mkdir()
    run = observed(script, [str(PROJECT)], script.parent, workdir)
    assert run.returncode == 0, run.stderr[-2000:]
    assert run.interpreters >= 2, "the worker ran outside the hook"
    seen = {(kind, event) for kind, event, _ in run.records}
    assert {("path", "open"), ("path", "os.listdir"), ("start", "os.system"), ("start", "subprocess.Popen")} <= seen, run.records
    opened = " ".join(detail for kind, event, detail in run.records if (kind, event) == ("path", "open"))
    # Text and bytes paths, a file that does not exist, and the file a worker opened before it died.
    for name in ("reference_observation.py", "execution_boundary.py", "no_such_file_here", "qualification_gate.py"):
        assert name in opened, (name, run.records)
    if os.path.exists(PROJECT / "TOOLS"):
        assert "qualification_process.py" in opened, run.records
    # Each program started is a record: another program, a bare interpreter, an interpreter in isolated
    # mode posing as a worker, and one posing as a worker with the environment cleared.
    started = [detail for kind, event, detail in run.records if (kind, event) == ("start", "exec")]
    assert len(started) == 4, run.records


@posix_only
def test_the_observation_records_nothing_for_a_run_that_stays_outside(tmp_path):
    script = tmp_path / "probe" / "probe.py"
    script.parent.mkdir()
    script.write_text(
        "from concurrent.futures import ProcessPoolExecutor\n"
        "if __name__ == '__main__':\n"
        "    with ProcessPoolExecutor(2) as pool:\n"
        "        print(list(pool.map(abs, [-1, 2])))\n",
        encoding="utf-8",
    )
    workdir = tmp_path / "work"
    workdir.mkdir()
    run = observed(script, [], script.parent, workdir)
    assert (run.returncode, run.stdout, run.records) == (0, "[1, 2]\n", []), run
    assert run.interpreters >= 2, "the pool's workers did not report"


@posix_only
def test_a_forked_worker_is_the_same_program_and_reports_itself(tmp_path):
    """Where workers are forked and not started as new interpreters, each child reports, its access is
    recorded, and the fork itself is not a started program."""
    script = tmp_path / "probe" / "probe.py"
    script.parent.mkdir()
    script.write_text(
        "import multiprocessing\n"
        "import sys\n"
        "def work(path):\n"
        "    open(path).close()\n"
        "if __name__ == '__main__':\n"
        "    with multiprocessing.get_context('fork').Pool(1) as pool:\n"
        "        pool.map(work, [sys.argv[1]])\n",
        encoding="utf-8",
    )
    workdir = tmp_path / "work"
    workdir.mkdir()
    run = observed(script, [str(PROJECT / "tools/validation/reference_observation.py")], script.parent, workdir)
    assert run.returncode == 0, run.stderr[-2000:]
    assert run.interpreters >= 2, "the forked worker did not report"
    assert [(kind, event) for kind, event, _ in run.records] == [("path", "open")], run.records


@posix_only
def test_a_path_from_the_generators_own_location_falls_under_a_watched_prefix(tmp_path):
    script = tmp_path / "project" / "validation" / "probe.py"
    script.parent.mkdir(parents=True)
    script.write_text(
        "from pathlib import Path\n"
        "try:\n"
        "    open(Path(__file__).parents[1] / 'co' 're' / 'x.py')\n"
        "except OSError:\n"
        "    pass\n",
        encoding="utf-8",
    )
    workdir = tmp_path / "work"
    workdir.mkdir()
    run = observed(script, [], script.parent, workdir, [tmp_path / "project" / name for name in sorted(PRODUCT_ROOTS)])
    assert [(kind, event) for kind, event, _ in run.records] == [("path", "open")], run.records


def test_silence_is_not_counted_as_no_access():
    """A started worker that never reports its hook, and a fork whose child never reports, are records."""
    worker = repr((True, [sys.executable, "-s", "-B", "-c", "from multiprocessing.spawn import spawn_main; spawn_main()"]))
    lines = ["interpreter\t10\tstart\tpython", f"exec\t10\t11\t{worker}", f"exec\t10\t12\t{worker}", "interpreter\t12\tstart\tpython"]
    assert _judge(lines, sys.executable) == ([("unobserved", "worker", "11")], 2)
    assert _judge(["interpreter\t10\tstart\tpython", "fork\t10\tos.fork\t"], sys.executable)[0] == [("unobserved", "fork", "1 forks, 0 children reported")]
    assert _judge(["interpreter\t10\tstart\tpython", "fork\t10\tos.fork\t", "interpreter\t13\tfork\t"], sys.executable) == ([], 2)
    assert _judge(["half a line"], sys.executable)[0] == [("unreadable", "report", "half a line")]
