"""The one independence rule for the reference generators under validation/references/.

A reference package claims that its generator imports, reads and runs no product code. This module
is the single statement of the rule that checks that claim from the generator's source, with `ast`,
and it is consumed by every test that scans reference generators (tests/test_reference_packages.py
over every package, and the lanes' own reference tests). Its witnesses, one per bypass a reviewer
has found, are in tests/test_reference_independence.py.

This file lives under tools/, which is itself a product root: no reference generator may import it,
and the rule below refuses such an import like any other.

The rule is a list of what is allowed, for every .py file of a package at any depth:
- Imports. Each imported module is on ALLOWED_STDLIB (standard-library modules that compute or do
  ordinary file I/O and cannot load code or start a program; each addition carries its reason
  there), or one the package's allowances name: a sibling module of the package, a module of another
  reference package (admitted only when that file is listed with its current hash in a checksum list
  of its own package, and no file or directory of the same name sits beside the importer to shadow
  it), or an external module the package's README says is supplied from outside the repository. A
  relative import is admitted only for a named sibling. Any other standard-library module is refused
  (kind "stdlib") and the import machinery (FORBIDDEN_MODULES) is refused as a facility; either
  needs an allowance in the file that uses it, with its reason.
- Dynamic facilities. A name in FORBIDDEN_NAMES (code loading and evaluation, the built-ins
  namespace, and the calls in os that start a program) is refused wherever it is named: called,
  referenced bare, as an attribute, as an import name or alias, or as a text or bytes constant (which
  is how `getattr(builtins, "eval")` or `globals()["__builtins__"]` would reach it). `compile` is
  refused as the builtin only (a bare name, an import name, a constant, or an attribute of
  `builtins`/`__builtins__`), because `re.compile` is an attribute named compile and is not a loader.
  A constant that is exactly a forbidden module's name is refused too (`sys.modules["importlib"]`).
- Product paths. Text and bytes constants are judged alike (bytes read as latin-1). A constant names
  a product path when a product root stands as a path segment anywhere in it: opening a segment
  (nothing of a name before it, a separator after it) or closing one (a separator before it, nothing
  of a name after it), so a path after a space in a directory name, inside quotation marks or inside
  a command is found; or when it is only a root; or when it is a dotted name whose first part is a
  root. A str standing alone as a statement (a docstring) is inert and is not scanned: it is never
  evaluated, so it cannot read anything.
- Layout. No directory may sit under a directory named `generators` unless the allowances name it.

An allowance is an explanation, not a waiver: each one names exactly what it admits (a module, a
facility in one file, a literal in one file, a directory, or a directory that holds a product check
rather than reference code and so is outside the rule) with its reason, and an allowance that no
longer matches anything is itself a finding, so the allowances cannot outlive their cause.

Known limits, found by reading and kept as expected-miss witnesses in the test file (each named as a
known gap, so that a closure flips it deliberately); the response to them is not yet decided:
- A module that reads its own docstring. Docstrings are inert as text nothing reads, but a module that
  names `__doc__` can read it: a docstring "core/x.py" then `open(__doc__)`, or a facility name as the
  docstring then `getattr(os, __doc__)`.
- Facilities an allowed module exposes that FORBIDDEN_NAMES does not name, for example
  `concurrent.futures.process.mp.Process(...)`. The specific names found so far (`os._execvpe`,
  `spawnv_passfds`, `SourcelessFileLoader`) are named and caught; the class of form stays open.
- A module named inside a longer text: "import core.solver" passed to a started interpreter is not a
  product path by the pattern (the dotted rule applies to a whole text).
- `independence_findings` judges one file and does not follow its imports. `scan_package` closes this
  inside a package (a sibling must be a module of the package, and every module is scanned); a caller
  that judges a lone file and admits a module beside it must scan that module too.
Closed by one line each: case variants of a root (`CORE/x.py`, the same directory on a case-insensitive
host) are matched without regard to case. Kept as a deliberate false finding: a text such as
"tests/s" (a unit) is reported as a product path; it is allowed with its reason where it occurs.

Outside the threat model, recorded once here: a path or module name assembled at run time (joined
from pieces none of which names a product root, read from a file or the environment, or decoded
from data), code reached through a subprocess, and what a generator does with files it reads. The
rule reads source; it does not execute it.

Standard library only.
"""
from __future__ import annotations

import ast
from dataclasses import dataclass, field
import hashlib
from pathlib import Path
import re
import sys

# The CI selector (tools/ci/python_test_reads.py) cannot see which directory `scan_package` walks; its
# callers pass reference packages.
CI_READS = ("validation/references/**",)

PRODUCT_ROOTS = frozenset({"core", "apps", "api", "tests", "tools", "fixtures", "schemas"})
# The standard-library modules a generator may import, each with why it can neither load code nor
# start a program. A module is added here in the change that needs it, with its reason.
ALLOWED_STDLIB_REASONS = {
    # The rule's list (the dynamics lane's, 64da5e92e).
    "argparse": "parses the command line",
    "concurrent": "runs this file's own functions in worker processes (concurrent.futures)",
    "decimal": "decimal arithmetic",
    "fractions": "exact rational arithmetic",
    "hashlib": "digests of bytes",
    "json": "reads and writes JSON text",
    "math": "floating-point functions",
    "os": "paths and file operations; its program-starting calls are FORBIDDEN_NAMES",
    "pathlib": "paths and file operations",
    "struct": "packs and unpacks binary64 bits",
    "sys": "argv, exit, the interpreter's path list and limits",
    "time": "clock readings",
}
# PROVISIONAL, not yet the rule: the further modules the 45 packages' generators import, proposed after
# reading each use, for the project's Agent 0 to accept or refuse together. Until then they let the
# inventory run; none may be treated as accepted.
PROVISIONAL_STDLIB_REASONS = {
    "__future__": "compiler directives (annotations) only",
    "ast": "parses source text into a tree; running it would need compile or exec, which are forbidden names",
    "collections": "container types",
    "copy": "copies objects already in memory",
    "dataclasses": "declares record classes; the methods it generates come from the class definition, not from outside",
    "itertools": "iteration helpers",
    "random": "seeded pseudo-random numbers",
    "re": "regular expressions",
    "shutil": "copies and removes files; `which` finds a program without running it",
    "tempfile": "temporary files and directories",
    "traceback": "formats exceptions",
}
ALLOWED_STDLIB = frozenset(ALLOWED_STDLIB_REASONS) | frozenset(PROVISIONAL_STDLIB_REASONS)
# Import machinery: refused as imports although standard library.
FORBIDDEN_MODULES = frozenset({"importlib", "runpy", "imp", "pkgutil", "zipimport"})
# Facilities that load or evaluate code named at run time, reach the built-ins namespace, or start
# another program: refused wherever named. `compile` is narrowed to the builtin (BUILTIN_ONLY).
FORBIDDEN_NAMES = frozenset({
    "__import__", "__builtins__", "import_module", "exec", "eval", "compile", "spec_from_file_location",
    "spec_from_loader", "module_from_spec", "exec_module", "load_module", "find_spec", "find_module",
    "SourceFileLoader", "SourcelessFileLoader", "ExtensionFileLoader", "run_path", "run_module",
    "execfile", "load_source", "load_compiled",
    "system", "popen", "startfile", "posix_spawn", "posix_spawnp", "fork", "forkpty",
    "execl", "execle", "execlp", "execlpe", "execv", "execve", "execvp", "execvpe",
    "spawnl", "spawnle", "spawnlp", "spawnlpe", "spawnv", "spawnve", "spawnvp", "spawnvpe",
    "_execvpe", "_spawnvef", "spawnv_passfds", "fork_exec",  # found by the third reading (known limits)
})
BUILTIN_ONLY = frozenset({"compile"})
BUILTIN_MODULES = frozenset({"builtins", "__builtins__"})
STDLIB = frozenset(sys.stdlib_module_names)
GENERATORS_DIR = "generators"
# Literal keys longer than this are a SHA-256 of the text, so an allowance stays exact.
LITERAL_KEY_MAX = 120

_SEPARATORS = re.compile(r"[\\/]+")
_HAS_SEPARATOR = re.compile(r"[\\/]")
_ROOT = "(?:" + "|".join(sorted(PRODUCT_ROOTS)) + ")"
# A product root standing as a path segment anywhere in a text: opening a segment (nothing of a name
# before it, a separator after it) or closing one (a separator before it, nothing of a name after it).
_PRODUCT_SEGMENT = re.compile(rf"(?<![\w.\-]){_ROOT}(?=[\\/])|(?<=[\\/]){_ROOT}(?![\w.\-])")
_DOTTED = re.compile(r"[A-Za-z_]\w*(?:\.[A-Za-z_]\w*)+")


def literal_text(value: str | bytes) -> str:
    """A constant's text: bytes are decoded as latin-1, which maps every byte."""
    return value.decode("latin-1") if isinstance(value, bytes) else value


def _has_product_segment(text: str) -> bool:
    return bool(PRODUCT_ROOTS & set(_SEPARATORS.split(text)))


def names_product_path(value: str | bytes) -> bool:
    """Whether a str or bytes constant names a product path (see the module docstring)."""
    text = literal_text(value)
    folded = text.lower()  # a root in another case is the same directory on a case-insensitive host
    if folded in PRODUCT_ROOTS or _PRODUCT_SEGMENT.search(folded) or _has_product_segment(folded):
        return True
    if any(_HAS_SEPARATOR.search(token) and _has_product_segment(token) for token in text.split()):
        return True
    return bool(_DOTTED.fullmatch(text)) and text.split(".", 1)[0] in PRODUCT_ROOTS


def literal_key(value: str | bytes) -> str:
    """How an allowance names a literal: its text, or `sha256:<hex>` of the text (as UTF-8) when the
    text is longer than LITERAL_KEY_MAX characters."""
    text = literal_text(value)
    if len(text) <= LITERAL_KEY_MAX:
        return text
    return "sha256:" + hashlib.sha256(text.encode("utf-8")).hexdigest()


@dataclass(frozen=True)
class Finding:
    """One site the rule does not admit by itself. `key` is what an allowance names."""
    file: str
    line: int
    kind: str  # "import", "stdlib", "relative-import", "facility", "literal"
    key: str
    detail: str

    def __str__(self) -> str:
        return f"{self.file}:{self.line}: {self.detail}"


def _inert_strings(tree: ast.AST) -> set[int]:
    """The ids of str constants that stand alone as a statement (docstrings and the like)."""
    return {id(node.value) for node in ast.walk(tree)
            if isinstance(node, ast.Expr) and isinstance(node.value, ast.Constant)
            and isinstance(node.value.value, str)}


def _top(module: str) -> str:
    return module.split(".", 1)[0]


def independence_findings(source: str | bytes, file: str) -> list[Finding]:
    """Every site in one generator file that the rule does not admit by itself: a non-stdlib import
    (kind "import"), a standard-library module not on ALLOWED_STDLIB ("stdlib"), a relative import,
    a forbidden module or name ("facility") and a product-path literal ("literal")."""
    tree = ast.parse(source, filename=file)
    inert = _inert_strings(tree)
    out: list[Finding] = []

    def facility(node: ast.AST, name: str, how: str) -> None:
        out.append(Finding(file, getattr(node, "lineno", 0), "facility", name, f"{how} {name}"))

    def imported(node: ast.AST, module: str) -> None:
        top = _top(module)
        if top in FORBIDDEN_MODULES:
            facility(node, top, "imports the import machinery")
        elif top in STDLIB:
            if top not in ALLOWED_STDLIB:
                out.append(Finding(file, node.lineno, "stdlib", top, f"imports {module}, not on ALLOWED_STDLIB"))
        else:
            out.append(Finding(file, node.lineno, "import", top, f"imports {module}"))

    for node in ast.walk(tree):
        if isinstance(node, ast.Import):
            for alias in node.names:
                imported(node, alias.name)
                if alias.asname in FORBIDDEN_NAMES:
                    facility(node, alias.asname, "imports under the name")
        elif isinstance(node, ast.ImportFrom):
            if node.level:
                out.append(Finding(file, node.lineno, "relative-import", _top(node.module or ""),
                                   f"relative import {'.' * node.level}{node.module or ''}"))
            else:
                imported(node, node.module)
            for alias in node.names:
                for name in {alias.name, alias.asname} & FORBIDDEN_NAMES:
                    facility(node, name, "imports the name")
        elif isinstance(node, ast.Name) and node.id in FORBIDDEN_NAMES:
            facility(node, node.id, "names")
        elif isinstance(node, ast.Attribute) and node.attr in FORBIDDEN_NAMES:
            if node.attr not in BUILTIN_ONLY or (isinstance(node.value, ast.Name) and node.value.id in BUILTIN_MODULES):
                facility(node, node.attr, "names the attribute")
        elif isinstance(node, ast.Constant) and isinstance(node.value, (str, bytes)) and id(node) not in inert:
            text = literal_text(node.value)
            if text in FORBIDDEN_NAMES or text in FORBIDDEN_MODULES:
                facility(node, text, "names in a constant")
            if names_product_path(node.value):
                out.append(Finding(file, node.lineno, "literal", literal_key(node.value),
                                   f"names a product path {text!r:.200} (key {literal_key(node.value)!r})"))
    return out


def sums_lines(path: Path) -> list[tuple[str, str]]:
    """The (sha256, name) lines of a SHA256SUMS-style list; any other non-blank line is an error."""
    lines = []
    for number, raw in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        if not raw.strip():
            continue
        match = re.match(r"^([0-9a-f]{64}) [ *](.+)$", raw)
        if not match:
            raise ValueError(f"{path}:{number} is not a checksum line: {raw!r}")
        lines.append((match.group(1), match.group(2)))
    return lines


def pinned_problem(file: Path, sums: Path, base: Path) -> str | None:
    """None when `sums` (names relative to `base`) lists `file` with its current hash."""
    for digest, name in sums_lines(sums):
        if (base / name).resolve() == file.resolve():
            if hashlib.sha256(file.read_bytes()).hexdigest() == digest:
                return None
            return f"{file.name} differs from its line in {sums.name}"
    return f"{file.name} is not listed in {sums.name}"


@dataclass(frozen=True)
class Foreign:
    """A module of another reference package: its file, the checksum list of its own package that must
    pin it, that list's base directory, and the reason it is imported."""
    file: Path
    sums: Path
    base: Path
    reason: str


@dataclass(frozen=True)
class Allowances:
    """What a package's entry admits beyond the rule, each with its reason. File paths are
    package-relative POSIX paths."""
    siblings: dict = field(default_factory=dict)  # {module: reason}; a .py file of the package
    foreign: dict = field(default_factory=dict)  # {module: Foreign}
    external: dict = field(default_factory=dict)  # {module: reason}; supplied from outside the repository
    facilities: dict = field(default_factory=dict)  # {(file, name): reason}
    literals: dict = field(default_factory=dict)  # {(file, literal_key): reason}
    directories: dict = field(default_factory=dict)  # {directory under generators/: reason}
    outside: dict = field(default_factory=dict)  # {package directory: reason}; not reference code, not scanned


def _outside(path: Path, package: Path, outside) -> bool:
    rel = path.relative_to(package).as_posix()
    return any(rel == d or rel.startswith(d + "/") for d in outside)


def generator_files(package: Path, outside=()) -> list[Path]:
    """Every .py file of a package, at any depth (bytecode caches and `outside` directories excluded)."""
    return sorted(p for p in package.rglob("*.py")
                  if p.is_file() and "__pycache__" not in p.parts and not _outside(p, package, outside))


def scan_package(package: Path, allow: Allowances = Allowances()) -> list[str]:
    """Every finding of the rule over one package that its allowances do not admit, and every
    allowance that admits nothing. Empty means the package passes."""
    files = generator_files(package, allow.outside)
    modules = {p.stem for p in files}
    used: set[tuple[str, object]] = set()
    problems: list[str] = []
    for path in files:
        rel = path.relative_to(package).as_posix()
        for f in independence_findings(path.read_bytes(), rel):
            if f.kind == "relative-import":
                if f.key in allow.siblings:
                    used.add(("siblings", f.key))
                else:
                    problems.append(str(f))
            elif f.kind == "import":
                if f.key in allow.siblings:
                    used.add(("siblings", f.key))
                    if f.key not in modules:
                        problems.append(f"{f}: sibling {f.key} is not a module of the package")
                elif f.key in allow.foreign:
                    used.add(("foreign", f.key))
                    foreign = allow.foreign[f.key]
                    beside = path.parent / f.key
                    if foreign.file.stem != f.key:
                        problems.append(f"{f}: foreign {f.key} is recorded as {foreign.file.name}")
                    elif beside.with_suffix(".py").exists() or beside.is_dir():
                        problems.append(f"{f}: {f.key} beside the importer shadows {foreign.file}")
                    elif (problem := pinned_problem(foreign.file, foreign.sums, foreign.base)):
                        problems.append(f"{f}: {problem}")
                elif f.key in allow.external:
                    used.add(("external", f.key))
                else:
                    problems.append(f"{f}, neither standard library nor allowed")
            elif f.kind in ("facility", "stdlib"):
                if (rel, f.key) in allow.facilities:
                    used.add(("facilities", (rel, f.key)))
                else:
                    problems.append(str(f))
            elif (rel, f.key) in allow.literals:
                used.add(("literals", (rel, f.key)))
            else:
                problems.append(str(f))
    for name in allow.outside:
        if (package / name).is_dir():
            used.add(("outside", name))
    for gen in sorted(p for p in package.rglob(GENERATORS_DIR) if p.is_dir() and not _outside(p, package, allow.outside)):
        for sub in sorted(p for p in gen.rglob("*") if p.is_dir() and p.name != "__pycache__"):
            rel = sub.relative_to(package).as_posix()
            if rel in allow.directories:
                used.add(("directories", rel))
            elif "__pycache__" not in sub.parts:
                problems.append(f"{rel}: a directory under {GENERATORS_DIR}/ that no allowance names")
    for kind in ("siblings", "foreign", "external", "facilities", "literals", "directories", "outside"):
        for key in getattr(allow, kind):
            if (kind, key) not in used:
                problems.append(f"allowance no longer used: {kind} {key!r}")
    return problems
