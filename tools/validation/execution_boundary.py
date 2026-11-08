#!/usr/bin/env python3
"""Maintained boundary guard: `execution/` holds declarative specifications, never code that runs.

Threat model. The guard prevents, in tracked repository content, the accidental or careless return of
runnable material to `execution/` and of execution edges into it from maintained sources and
entrypoints:
- in `execution/`: any file kind outside the declarative set (Markdown, YAML, JSON, CSV, text, SHA256
  lists, patches; a `.gitattributes` restricted to whitespace/text/eol attributes), executable modes,
  symlinks and submodules, tool-discovery names (`package.json`, `tsconfig*.json`, `action.yml`) and
  shebang-led files. The 22 accepted SCA-011 helper sources are FIDELITY exceptions: they must stay at
  their exact paths with their exact accepted SHA256 and mode 100644. That is integrity only; the guard
  forbids executing them exactly as it forbids executing anything else in `execution/`;
- outside `execution/`: Python imports of an `execution` package, and an execution path (a literal,
  an f-string, a constant concatenation, or a component-built `Path(...) / "execution" / ...`, followed
  through assignments, containers, same-file calls/returns and module-level names imported or
  re-exported by other maintained modules) reaching `importlib`/`runpy`/`exec`/`eval`/`compile`/loader calls, `sys.path`,
  `site.addsitedir`, `PYTHONPATH`, `subprocess`/`os` process launchers or `pytest.main`. A git
  subprocess is a read, not an execution. Reading source text and compiling or executing it is caught
  because the text read from a tainted path is itself tainted. These EXEC edges have no exemption;
- outside `execution/`: Python file reads/scans of execution paths, except in the declared document
  readers (`SPEC_READERS`), which validate declarative specifications and run on the document route;
  and an execution path passed to a call the guard cannot classify, which must be classified here
  (reviewed) rather than pass silently;
- Rust `include!`/`include_str!`/`include_bytes!`/`#[path]` and other unclassified Rust references;
  JS/TS `import`/`require`/dynamic import/`new URL`/file reads/process launches; Cargo manifests,
  package scripts, TS/Vite/Playwright configuration, shell and PowerShell scripts, `.pth` files,
  GitHub workflow/action entrypoints (other than sparse-checkout exclusion), and registry `command`/
  `cwd`/`pytest_paths` entries naming an execution path; extensionless executable/shebang entrypoints;
  and tracked symlinks resolving into it. Unrecognized executable interpreters fail classification.

Practical limits (stated, not repaired): this is a bounded static check of tracked content, not a proof
about arbitrary programs. It does not follow paths built from runtime input, environment values,
`chr()`/encoding tricks, `getattr` on computed names, non-constant format strings, values passed
between modules other than imported module-level names, or untracked files. Rust, JS/TS and script
checks are lexical (string literals with their line context, three lines for JS/TS). Product jobs also
check out the tree without `execution/`, so a product read that escapes the guard fails there.
"""
from __future__ import annotations

import argparse
import ast
import hashlib
import json
from pathlib import Path, PurePosixPath
import re
import subprocess
import sys
import tomllib

ROOT = Path(__file__).resolve().parents[2]
DECLARATIVE_ROOT = "execution/"
DECLARATIVE_SUFFIXES = (".md", ".yaml", ".yml", ".json", ".csv", ".txt", ".sha256", ".patch")
DISCOVERY_NAMES = {"package.json", "package-lock.json", "deno.json", "action.yml", "action.yaml",
                   "pyproject.toml", "setup.cfg", "pytest.ini", "tox.ini", "conftest.py"}
GITATTRIBUTES_ALLOWED = re.compile(r"^-?(whitespace(=[\w,-]+)?|text(=auto)?|eol=(lf|crlf)|binary|linguist-[\w-]+(=\w+)?)$")

SCA011 = "execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP/"
# Accepted historical SCA-011 helper sources (ACCEPTED_MANIFEST.csv). Fidelity evidence only: never run.
FIDELITY = {SCA011 + path: digest for path, digest in {
    "application/carriers/apply_carriers.py": "2e69a2a95bd37cf140f2a30eeaff7cafb3e1c25335ac160117ed41b1986fc0f9",
    "application/carriers/validate_controls.py": "38880f8d97eab8f51cae57c4fc3a4f38d8a5a97dacafe3016c17d35fcfd88603",
    "application/dependencies/apply_dependencies.py": "0d735cd3490c7948bf161cf7d85426b690c85d822c14593ed3ce55f871635c17",
    "application/dependencies/backcheck_local_sources.py": "27c489966c9d74b68139454b51f9d3181c00e02a0b9e3158c43664cae6fdfa29",
    "application/dependencies/repair_metadata.py": "0c27d18ba6a23198b3c7035de15297dd8e0d6fb45962b7017db7c681a4c7c556",
    "application/dependencies/validate_repaired.py": "bf2504c486bc88df9512bcbedf255926f4682f3cfbccfbba9fab7b337c36a30c",
    "application/dependencies/validate_staged.py": "d7a25b2b1f1866d91535f3047b86adafd945789c99c34e3f34ce67683e5f8923",
    "application/freeze_applied.py": "2d7d9f508a6e2105ad1a839a0e057ea65851ba8fcdb48d3c1460c79ed7035140",
    "application/postchange_audit/assemble_report.py": "cd8f33dcc032cc6c6154ccf131af05c3ad509624cfecd60729ff8c453d56237c",
    "application/postchange_audit/audit_poststate.py": "27bd7086a5f59adaf9602dd718734d2c7cfcf504ba4b0da4921d6b4d4c554c68",
    "application/postchange_audit/initial_exact_application/assemble_report.py": "63c56114404d1e7ccda5e19ec3352e9e0c991593af58569bb6540df237b260b0",
    "application/postchange_audit/initial_exact_application/audit_poststate.py": "d202a53c018c468b168055f7db13ff16f80a924733e43329daf96ec9278b592b",
    "application/reconciliation/build_crosswalk.py": "14684d8b78a9040d8c563f66f7399d99e762d8f5adb6f1468ac7f0df8e5c9a4a",
    "application/review/independent_checks.py": "81b045961eb5d2a3f411378c445c40959ede71c9959bf6911e7e0eb8b683ec0d",
    "application/verify_application.py": "c73920a20ef2d7db6300bd740b3b4e899b0b578a5035b702e88be80760d61493",
    "dependencies/build_candidate.py": "a614974085acb520cf45aa3cbff814728f8e12bbb3a427c4972a6e428cd67423",
    "dependencies/build_local_previews.py": "68614e5156a09894e5a218cea7da8597cd8ac63b422a378bc2dd7617d237df5e",
    "dependencies/validate_candidate.py": "403ab94263c969bd1d308ba5a677e5b674cf803a690f7bdd3aebfc3ca70702cc",
    "evidence/build_review_package.py": "cc074a4153fd7c395b6627de943c887f88015c5078db99f4989002fd7a19206e",
    "evidence/prechange_audit/audit_baseline.py": "b3191acf0e83f114c9cd13b5b5e44679eeb2cb011db0596a419cd95bfce8a691",
    "evidence/validate_all_sows.py": "c6f9ccfbd8789e0fbc49e095989455f35f4f4117ff46c3305ba772105a92e68d",
    "interfaces/check_drafts.py": "99424ddf09ca6ded671091ac0ffed11d4f641c20b0cffe3517770791b36f7796",
}.items()}

# Maintained document readers: they read (never run) declarative specifications and run on the
# documentation route or in the repository checks, which carry execution/.
SPEC_READERS = (
    "tools/deliverables/",
    "tools/validation/execution_boundary.py",
    "tools/validation/test_execution_boundary.py",
    "tools/validation/test_execution_dag.py",
    "tools/validation/test_hanger_sizing_design.py",
    "tools/validation/test_reference_system_ledger.py",
    "tools/release/check_release_readiness.py",
    "tools/software_workflow/documentation_ci.py",
    "tools/software_workflow/test_documentation_ci.py",
)
# Reviewed classifications of execution-path text the guard cannot otherwise classify (file: reason).
# Each must stay a tracked file; an entry no longer needed is reported as stale.
# Never an EXEC edge: classification only admits a read or an unclassified reference.
CLASSIFIED: dict[str, str] = {
    "tests/public_export.py": "presence probe of the retired export layout (loop/ and execution/ both absent); "
                              "reads no content, and loop/ keeps it false in the product checkout without execution/",
}

EXCLUSION = re.compile(r"^(:!|:\^|:\(exclude[^)]*\)|!/?)")
SEED = re.compile(r"(?:^|[/\\:])execution(?:[/\\]|$)")
TOKEN = re.compile(r"[\s'\"`;|&()=,]+")


def is_seed(text: str) -> bool:
    return bool(SEED.search(text)) and not EXCLUSION.match(text)


def mentions(text: str) -> bool:
    """Whether any whitespace/shell-delimited token of text is an execution path."""
    return any(is_seed(token) for token in TOKEN.split(text))


def dotted_execution(text: str) -> bool:
    return "execution" in re.split(r"[./\\]", text)


def declarative(path: str) -> bool:
    return path.startswith(DECLARATIVE_ROOT)


# ----------------------------------------------------------------------------------------------- Python
EXEC_CALLS = {
    "importlib.import_module", "importlib.util.spec_from_file_location", "importlib.util.spec_from_loader",
    "importlib.machinery.SourceFileLoader", "importlib.machinery.SourcelessFileLoader",
    "importlib.machinery.ExtensionFileLoader", "importlib.util.module_from_spec", "imp.load_source",
    "imp.load_module", "imp.load_compiled", "imp.load_dynamic", "runpy.run_path", "runpy.run_module",
    "exec", "eval", "compile", "execfile", "__import__", "site.addsitedir", "sys.path.insert",
    "sys.path.append", "sys.path.extend", "pytest.main", "ctypes.CDLL", "ctypes.PyDLL", "ctypes.cdll.LoadLibrary",
    "os.system", "os.popen", "os.startfile", "os.posix_spawn", "os.posix_spawnp",
    "subprocess.run", "subprocess.call", "subprocess.check_call", "subprocess.check_output", "subprocess.Popen",
    "subprocess.getoutput", "subprocess.getstatusoutput", "asyncio.create_subprocess_exec",
    "asyncio.create_subprocess_shell", "code.InteractiveInterpreter", "zipimport.zipimporter",
}
EXEC_PREFIXES = ("os.exec", "os.spawn")
EXEC_METHODS = {"exec_module", "load_module", "run_path", "run_module", "spec_from_file_location", "import_module"}
SUBPROCESS = {"subprocess.run", "subprocess.call", "subprocess.check_call", "subprocess.check_output",
              "subprocess.Popen", "asyncio.create_subprocess_exec"}
READ_METHODS = {"open", "read_text", "read_bytes", "write_text", "write_bytes", "exists", "is_file", "is_dir",
                "stat", "lstat", "glob", "rglob", "iterdir", "unlink", "rename", "replace", "mkdir", "touch",
                "readlink", "symlink_to", "samefile", "rmdir", "chmod"}
READ_CALLS = {"open", "io.open", "codecs.open", "os.walk", "os.listdir", "os.scandir", "os.stat", "os.lstat",
              "os.path.exists", "os.path.isfile", "os.path.isdir", "os.path.getsize", "glob.glob", "glob.iglob",
              "shutil.copy", "shutil.copy2", "shutil.copyfile", "shutil.copytree", "shutil.rmtree", "shutil.move",
              "os.remove", "os.unlink", "os.rename", "os.replace", "os.makedirs", "os.mkdir", "os.open",
              "tarfile.open", "zipfile.ZipFile", "filecmp.cmp", "filecmp.dircmp", "fileinput.input",
              "linecache.getline", "linecache.getlines", "tokenize.open", "mmap.mmap"}
# Calls whose result says nothing about a path: comparisons, membership and measurements.
BARRIERS = {"startswith", "endswith", "isinstance", "len", "bool", "fnmatch", "fnmatchcase", "match", "fullmatch",
            "search", "count", "find", "rfind", "index", "is_relative_to", "issubclass", "callable", "hasattr", "id"}
# Calls that only carry a value along (their result is tainted by the arguments) or consume it as data.
PURE = {"str", "repr", "print", "format", "sorted", "reversed", "list", "tuple", "set", "frozenset", "dict",
        "enumerate", "zip", "map", "filter", "any", "all", "min", "max", "sum", "iter", "next", "hash",
        "Path", "PurePath", "PurePosixPath", "PosixPath", "PureWindowsPath", "os.fspath", "os.path.join",
        "os.path.normpath", "os.path.abspath", "os.path.realpath", "os.path.relpath", "os.path.basename",
        "os.path.dirname", "os.path.split", "os.path.splitext", "posixpath.join", "json.dumps", "json.dump",
        "hashlib.sha256", "re.escape", "re.compile", "textwrap.dedent", "textwrap.indent", "shlex.quote",
        "shlex.join", "warnings.warn", "pytest.raises", "pytest.mark.parametrize", "pytest.param",
        "pytest.fail", "pytest.skip", "pytest.importorskip", "dataclasses.replace", "copy.deepcopy", "copy.copy",
        "collections.OrderedDict", "collections.Counter", "Fraction", "Decimal", "bytes", "bytearray",
        # Parsers carry content along; executing what they return is still an EXEC sink.
        "json.loads", "json.load", "yaml.safe_load", "yaml.load", "tomllib.loads", "tomllib.load", "csv.reader",
        "csv.DictReader", "ast.parse", "io.StringIO", "io.BytesIO"}
PURE_METHODS = {"join", "joinpath", "format", "replace", "split", "rsplit", "strip", "lstrip", "rstrip", "lower",
                "upper", "encode", "decode", "partition", "rpartition", "splitlines", "as_posix", "resolve",
                "absolute", "relative_to", "with_name", "with_suffix", "with_stem", "parent", "get", "setdefault",
                "items", "keys", "values", "copy", "removeprefix", "removesuffix", "casefold", "title",
                "expanduser", "add", "append", "extend", "insert", "update", "write", "writelines", "dumps",
                "sub", "subn", "findall", "finditer", "group", "groups", "ljust", "rjust",
                "center", "zfill", "translate", "maketrans", "pop", "remove", "discard", "union", "intersection",
                "difference", "issubset", "issuperset", "sort", "subTest", "debug", "info", "warning", "error",
                "exception", "critical", "fail", "skipTest"}
CONTAINER_MUTATORS = {"append", "extend", "insert", "add", "update", "setdefault", "appendleft", "extendleft"}


def canonical(node: ast.AST, aliases: dict[str, str]) -> str:
    parts = []
    while isinstance(node, ast.Attribute):
        parts.append(node.attr)
        node = node.value
    if isinstance(node, ast.Name):
        parts.append(aliases.get(node.id, node.id))
    else:
        return ".".join(["?", *reversed(parts)])
    return ".".join(reversed(parts))


PATHLIKE = re.compile(r"^\S*$")
SCOPES = (ast.FunctionDef, ast.AsyncFunctionDef, ast.Lambda)
# Locations outside the repository: a path under one is a temporary fixture, not the repository's execution/.
TEMP_PARAMETERS = {"tmp_path", "tmpdir", "tmp_path_factory", "tmpdir_factory"}
TEMP_CALLS = {"tempfile.mkdtemp", "tempfile.gettempdir", "tempfile.TemporaryDirectory", "tempfile.NamedTemporaryFile",
              "tempfile.mkstemp"}
MAX_EXPORT_ROUNDS = 32
MAX_LOCAL_ROUNDS = 50


class PythonFile:
    """Taint of execution-path values through one module, scope-aware and flow-insensitive."""

    def __init__(self, path: str, tree: ast.Module, exported: dict[str, frozenset[str]],
                 classified: dict[str, str] = CLASSIFIED):
        self.path, self.tree, self.exported, self.classified = path, tree, exported, classified
        self.aliases: dict[str, str] = {}
        self.module_aliases: dict[tuple, str] = {}
        self.tainted: set[tuple] = set()
        self.outside: set[tuple] = set()
        self.returns: set[str] = set()
        self.outside_returns: set[str] = set()
        self.mappings: set[tuple] = set()
        self.mapping_returns: set[str] = set()
        self.key_taint: set[tuple] = set()
        self.key_returns: set[str] = set()
        self.value_taint: set[tuple] = set()
        self.value_returns: set[str] = set()
        self.incomplete = False
        self.functions: dict[str, ast.AST] = {}
        self.scope_of: dict[int, ast.AST] = {}
        self.parent_scope: dict[int, ast.AST] = {}
        self.locals: dict[int, set[str]] = {id(tree): set()}
        self._scopes(tree, tree)
        tainted_imports = []
        for node in ast.walk(tree):
            scope = id(self.scope_of.get(id(node), tree))
            if isinstance(node, ast.Import):
                for alias in node.names:
                    self.aliases[(alias.asname or alias.name).split(".")[0]] = (
                        alias.name if alias.asname else alias.name.split(".")[0])
                    binding = alias.asname or alias.name.split(".")[0]
                    self.module_aliases[(scope, binding)] = alias.name if alias.asname else binding
            elif isinstance(node, ast.ImportFrom):
                module = node.module or ""
                if node.level:
                    package = PurePosixPath(path).parts[:-1]
                    module = ".".join((*package[:len(package) - node.level + 1], *module.split("."))).strip(".")
                for alias in node.names:
                    binding = alias.asname or alias.name
                    imported = f"{module}.{alias.name}".strip(".")
                    self.aliases[binding] = imported
                    if imported in exported:
                        self.module_aliases[(scope, binding)] = imported
                    names = exported.get(module)
                    if names and alias.name in names:
                        tainted_imports.append((node, binding))
            elif isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
                self.functions[node.name] = node
        # Taint starts only at a shown execution path or an imported tainted name; without one there is none.
        if tainted_imports or any(self.seed(node) or self.imported_name(node) for node in ast.walk(tree)
                               if isinstance(node, (ast.Constant, ast.JoinedStr, ast.BinOp, ast.Call, ast.Attribute))):
            for node, name in tainted_imports:
                self.tainted.add((id(self.scope_of.get(id(node), tree)), name))
            self._propagate()

    def imported_module(self, node) -> str | None:
        if isinstance(node, ast.Name):
            return self.module_aliases.get(self.key(node))
        if isinstance(node, ast.Attribute):
            parent = self.imported_module(node.value)
            return f"{parent}.{node.attr}" if parent else None
        return None

    def imported_name(self, node) -> bool:
        """`module.NAME` where NAME is a tainted module-level name of an imported maintained module."""
        return (isinstance(node, ast.Attribute)
                and node.attr in self.exported.get(self.imported_module(node.value), ()))

    def _scopes(self, node, scope):
        for child in ast.iter_child_nodes(node):
            self.scope_of[id(child)] = scope
            if isinstance(child, SCOPES):
                self.parent_scope[id(child)] = scope
                self.locals[id(child)] = {a.arg for a in [*child.args.posonlyargs, *child.args.args,
                                                         *child.args.kwonlyargs, child.args.vararg, child.args.kwarg] if a}
                if not isinstance(child, ast.Lambda):
                    self.locals[id(scope)].add(child.name)
                self._scopes(child, child)
            else:
                if isinstance(child, ast.Name) and isinstance(child.ctx, ast.Store):
                    self.locals[id(scope)].add(child.id)
                elif isinstance(child, (ast.Import, ast.ImportFrom)):
                    self.locals[id(scope)].update((a.asname or a.name).split(".")[0] for a in child.names)
                elif isinstance(child, ast.ExceptHandler) and child.name:
                    self.locals[id(scope)].add(child.name)
                elif isinstance(child, ast.ClassDef):
                    self.locals[id(scope)].add(child.name)
                self._scopes(child, scope)

    def key(self, node: ast.Name) -> tuple:
        scope = self.scope_of.get(id(node), self.tree)
        while scope is not self.tree and node.id not in self.locals.get(id(scope), ()):
            scope = self.parent_scope.get(id(scope), self.tree)
        return (id(scope), node.id)

    # Constant folding of string text; None where the code does not show it.
    def fold(self, node):
        if isinstance(node, ast.Constant) and isinstance(node.value, str):
            return node.value
        if isinstance(node, ast.JoinedStr):
            parts = []
            for value in node.values:
                folded = self.fold(value.value) if isinstance(value, ast.FormattedValue) else self.fold(value)
                parts.append("\0" if folded is None else folded)
            return "".join(parts)
        if isinstance(node, ast.BinOp) and isinstance(node.op, (ast.Add, ast.Div)):
            left, right = self.fold(node.left), self.fold(node.right)
            if left is not None and right is not None:
                return left + ("/" if isinstance(node.op, ast.Div) else "") + right
        if (isinstance(node, ast.Call) and isinstance(node.func, ast.Attribute) and node.func.attr == "join"
                and len(node.args) == 1 and isinstance(node.args[0], (ast.List, ast.Tuple))):
            separator = self.fold(node.func.value)
            items = [self.fold(item) for item in node.args[0].elts]
            if separator is not None and None not in items:
                return separator.join(items)
        return None

    def target_keys(self, target) -> list[tuple]:
        if isinstance(target, ast.Name):
            return [self.key(target)]
        if isinstance(target, (ast.Tuple, ast.List)):
            return [key for element in target.elts for key in self.target_keys(element)]
        if isinstance(target, ast.Starred):
            return self.target_keys(target.value)
        if isinstance(target, ast.Attribute):
            return [("attr", ast.unparse(target))]
        if isinstance(target, ast.Subscript):
            return self.target_keys(target.value)
        return []

    def is_outside(self, node) -> bool:
        """A value the code shows to be a temporary location outside the repository (or a path under it)."""
        if isinstance(node, ast.Name):
            return node.id in TEMP_PARAMETERS and self.key(node) not in self.tainted or self.key(node) in self.outside
        if isinstance(node, ast.Attribute):
            return ("attr", ast.unparse(node)) in self.outside or (
                node.attr in {"parent", "name"} and self.is_outside(node.value))
        if isinstance(node, ast.BinOp) and isinstance(node.op, ast.Div):
            return self.is_outside(node.left)
        if isinstance(node, ast.Call):
            name = canonical(node.func, self.aliases)
            if name in TEMP_CALLS or name.endswith(".mktemp"):
                return True
            if isinstance(node.func, ast.Name) and node.func.id in self.functions:
                return node.func.id in self.outside_returns
            if isinstance(node.func, ast.Attribute) and node.func.attr in {"joinpath", "resolve", "absolute", "with_name",
                                                                           "with_suffix", "mkdtemp", "mktemp"}:
                return self.is_outside(node.func.value)
            if name.rsplit(".", 1)[-1] in {"Path", "PurePath", "PurePosixPath", "PosixPath", "str", "fspath",
                                           "join", "abspath", "realpath"} and node.args:
                return self.is_outside(node.args[0])
        return False

    def seed(self, node) -> bool:
        text = self.fold(node)
        return text is not None and is_seed(text) and bool(PATHLIKE.match(text))

    def is_tainted(self, node) -> bool:
        if node is None or isinstance(node, (ast.Compare, ast.Lambda)):
            return False
        if self.seed(node):
            return True
        if isinstance(node, ast.Constant):
            return False
        if self.is_outside(node):
            return False
        if isinstance(node, ast.Name):
            return self.key(node) in self.tainted
        if isinstance(node, ast.Dict):
            return any(self.is_tainted(value) for value in [*node.keys, *node.values])
        if isinstance(node, ast.DictComp):
            return self.is_tainted(node.key) or self.is_tainted(node.value)
        if isinstance(node, ast.Subscript) and self.is_mapping(node.value):
            return self.values_tainted(node.value)
        if isinstance(node, ast.IfExp):
            return self.is_tainted(node.body) or self.is_tainted(node.orelse)
        if isinstance(node, ast.Attribute):
            if ("attr", ast.unparse(node)) in self.tainted:
                return True
            return self.imported_name(node) or self.is_tainted(node.value)
        if isinstance(node, ast.UnaryOp):
            return not isinstance(node.op, ast.Not) and self.is_tainted(node.operand)
        if isinstance(node, ast.Call):
            name = canonical(node.func, self.aliases)
            tail = name.rsplit(".", 1)[-1]
            if isinstance(node.func, ast.Attribute) and self.is_mapping(node.func.value):
                if tail == "keys":
                    return self.keys_tainted(node.func.value)
                if tail in {"get", "values", "pop"}:
                    return self.values_tainted(node.func.value)
            if tail in BARRIERS or tail in {"exists", "is_file", "is_dir", "samefile"}:
                return False
            if isinstance(node.func, ast.Name) and node.func.id in self.functions:
                return node.func.id in self.returns
            if isinstance(node.func, ast.Attribute) and self.is_tainted(node.func.value):
                return True
            return any(self.is_tainted(arg) for arg in [*node.args, *(k.value for k in node.keywords)])
        if isinstance(node, (ast.ListComp, ast.SetComp, ast.GeneratorExp)):
            return self.is_tainted(node.elt)
        return any(self.is_tainted(child) for child in ast.iter_child_nodes(node)
                   if isinstance(child, ast.expr))

    def is_mapping(self, node) -> bool:
        if isinstance(node, (ast.Dict, ast.DictComp)):
            return True
        if isinstance(node, ast.Name):
            return self.key(node) in self.mappings
        return (isinstance(node, ast.Call) and isinstance(node.func, ast.Name)
                and node.func.id in self.mapping_returns)

    def keys_tainted(self, node) -> bool:
        if isinstance(node, ast.Dict):
            return any(self.is_tainted(key) if key is not None else self.keys_tainted(value)
                       for key, value in zip(node.keys, node.values))
        if isinstance(node, ast.DictComp):
            return self.is_tainted(node.key)
        if isinstance(node, ast.Name) and self.is_mapping(node):
            return self.key(node) in self.key_taint
        if self.is_mapping(node):
            return node.func.id in self.key_returns
        return self.is_tainted(node)  # unknown imported/container shape stays conservative

    def values_tainted(self, node) -> bool:
        if isinstance(node, ast.Dict):
            return any(self.is_tainted(value) for value in node.values)
        if isinstance(node, ast.DictComp):
            return self.is_tainted(node.value)
        if isinstance(node, ast.Name) and self.is_mapping(node):
            return self.key(node) in self.value_taint
        if self.is_mapping(node):
            return node.func.id in self.value_returns
        return self.is_tainted(node)

    def _propagate_mapping(self, node):
        if isinstance(node, ast.Call) and isinstance(node.func, ast.Attribute):
            source, kind = node.func.value, node.func.attr
            tainted_key = (kind == "update" and any(self.keys_tainted(arg) for arg in node.args)
                           or kind == "setdefault" and node.args and self.is_tainted(node.args[0]))
            if tainted_key:
                self.key_taint.update(self.target_keys(source))
                self.tainted.update(self.target_keys(source))
            tainted_value = (kind == "update" and (any(self.values_tainted(arg) for arg in node.args)
                                                   or any(self.is_tainted(k.value) for k in node.keywords))
                             or kind == "setdefault" and len(node.args) > 1 and self.is_tainted(node.args[1]))
            if tainted_value:
                self.value_taint.update(self.target_keys(source))
                self.tainted.update(self.target_keys(source))
            return
        if isinstance(node, ast.Assign):
            targets, value = node.targets, node.value
        elif isinstance(node, (ast.AnnAssign, ast.NamedExpr)):
            targets, value = [node.target], node.value
        elif isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            for child in ast.walk(node):
                if (isinstance(child, ast.Return) and self.scope_of.get(id(child)) is node
                        and self.is_mapping(child.value)):
                    self.mapping_returns.add(node.name)
                    if self.keys_tainted(child.value):
                        self.key_returns.add(node.name)
                    if self.values_tainted(child.value):
                        self.value_returns.add(node.name)
            return
        else:
            return
        for target in targets:
            if isinstance(target, ast.Subscript) and self.is_tainted(target.slice):
                self.key_taint.update(self.target_keys(target.value))
                self.tainted.update(self.target_keys(target.value))
            if isinstance(target, ast.Subscript) and self.is_tainted(value):
                self.value_taint.update(self.target_keys(target.value))
        if self.is_mapping(value):
            for target in targets:
                self.mappings.update(self.target_keys(target))
                if self.keys_tainted(value):
                    self.key_taint.update(self.target_keys(target))
                if self.values_tainted(value):
                    self.value_taint.update(self.target_keys(target))

    def _propagate_iteration(self, target, iterable):
        while (isinstance(iterable, ast.Call) and isinstance(iterable.func, ast.Name)
               and iterable.func.id in {"sorted", "reversed", "list", "tuple", "iter"} and iterable.args):
            iterable = iterable.args[0]
        if isinstance(iterable, ast.Call) and isinstance(iterable.func, ast.Attribute):
            source, kind = iterable.func.value, iterable.func.attr
            if kind == "items" and isinstance(target, (ast.Tuple, ast.List)) and len(target.elts) == 2:
                for part, tainted in zip(target.elts, [self.keys_tainted(source), self.values_tainted(source)]):
                    if tainted:
                        self.tainted.update(self.target_keys(part))
                return
            if kind == "keys":
                if self.keys_tainted(source):
                    self.tainted.update(self.target_keys(target))
                return
        tainted = self.keys_tainted(iterable) if self.is_mapping(iterable) else self.is_tainted(iterable)
        if tainted:
            self.tainted.update(self.target_keys(target))

    def _propagate(self):
        def state():
            return tuple(map(len, (self.tainted, self.returns, self.outside, self.outside_returns,
                                   self.mappings, self.mapping_returns, self.key_taint, self.key_returns,
                                   self.value_taint, self.value_returns)))

        for _ in range(MAX_LOCAL_ROUNDS):
            before = state()
            for node in ast.walk(self.tree):
                self._propagate_outside(node)
                self._propagate_mapping(node)
                if isinstance(node, ast.Assign) and self.is_tainted(node.value):
                    for target in node.targets:
                        self.tainted.update(self.target_keys(target))
                elif isinstance(node, (ast.AnnAssign, ast.AugAssign, ast.NamedExpr)) and self.is_tainted(node.value):
                    self.tainted.update(self.target_keys(node.target))
                elif isinstance(node, (ast.For, ast.AsyncFor, ast.comprehension)):
                    self._propagate_iteration(node.target, node.iter)
                elif isinstance(node, ast.withitem) and node.optional_vars and self.is_tainted(node.context_expr):
                    self.tainted.update(self.target_keys(node.optional_vars))
                elif isinstance(node, ast.Call):
                    if (isinstance(node.func, ast.Attribute) and node.func.attr in CONTAINER_MUTATORS
                            and any(self.is_tainted(a) for a in [*node.args, *(k.value for k in node.keywords)])):
                        self.tainted.update(self.target_keys(node.func.value))
                    if isinstance(node.func, ast.Name) and node.func.id in self.functions:
                        function = self.functions[node.func.id]
                        params = [*function.args.posonlyargs, *function.args.args]
                        for param, arg in zip(params, node.args):
                            if self.is_tainted(arg):
                                self.tainted.add((id(function), param.arg))
                        names = {p.arg for p in [*params, *function.args.kwonlyargs]}
                        for keyword in node.keywords:
                            if keyword.arg in names and self.is_tainted(keyword.value):
                                self.tainted.add((id(function), keyword.arg))
                elif isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
                    params = [*node.args.posonlyargs, *node.args.args]
                    for param, default in zip(params[len(params) - len(node.args.defaults):], node.args.defaults):
                        if self.is_tainted(default):
                            self.tainted.add((id(node), param.arg))
                    for param, default in zip(node.args.kwonlyargs, node.args.kw_defaults):
                        if default is not None and self.is_tainted(default):
                            self.tainted.add((id(node), param.arg))
                    for child in ast.walk(node):
                        if (isinstance(child, (ast.Return, ast.Yield)) and self.scope_of.get(id(child)) is node
                                and self.is_tainted(child.value)):
                            self.returns.add(node.name)
            if state() == before:
                return
        self.incomplete = True

    def _propagate_outside(self, node):
        if isinstance(node, ast.Assign) and self.is_outside(node.value):
            for target in node.targets:
                self.outside.update(self.target_keys(target))
        elif isinstance(node, (ast.AnnAssign, ast.NamedExpr)) and node.value is not None and self.is_outside(node.value):
            self.outside.update(self.target_keys(node.target))
        elif isinstance(node, ast.withitem) and node.optional_vars and self.is_outside(node.context_expr):
            self.outside.update(self.target_keys(node.optional_vars))
        elif isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            for child in ast.walk(node):
                if (isinstance(child, (ast.Return, ast.Yield)) and self.scope_of.get(id(child)) is node
                        and child.value is not None and self.is_outside(child.value)):
                    self.outside_returns.add(node.name)
            # A pytest fixture returning a temporary location: a parameter of that name in this file is one too.
            if node.name in self.outside_returns and any("fixture" in ast.unparse(d) for d in node.decorator_list):
                for function in self.functions.values():
                    for param in [*function.args.posonlyargs, *function.args.args]:
                        if param.arg == node.name:
                            self.outside.add((id(function), param.arg))

    def module_names(self) -> frozenset[str]:
        return frozenset(name for scope, name in self.tainted if scope == id(self.tree))

    def findings(self) -> list[tuple[str, int, str]]:
        found = []
        if self.incomplete:
            found.append(("unclassified", 0, f"Python path analysis did not converge in {MAX_LOCAL_ROUNDS} rounds"))
        for node in ast.walk(self.tree):
            if isinstance(node, ast.Import):
                for alias in node.names:
                    if alias.name.split(".")[0] == "execution":
                        found.append(("exec", node.lineno, f"imports {alias.name}"))
            elif isinstance(node, ast.ImportFrom) and node.module and node.module.split(".")[0] == "execution":
                found.append(("exec", node.lineno, f"imports from {node.module}"))
            elif isinstance(node, (ast.Assign, ast.AugAssign)):
                targets = node.targets if isinstance(node, ast.Assign) else [node.target]
                for target in targets:
                    text = ast.unparse(target)
                    if (text.startswith("sys.path") or "PYTHONPATH" in text) and self.is_tainted(node.value):
                        found.append(("exec", node.lineno, f"sets {text} to an execution path"))
            elif isinstance(node, ast.Dict):
                for key, value in zip(node.keys, node.values):
                    if key is not None and self.fold(key) == "PYTHONPATH" and self.is_tainted(value):
                        found.append(("exec", node.lineno, "PYTHONPATH names an execution path"))
            elif isinstance(node, ast.Call):
                found.extend(self.classify_call(node))
        return found

    def command_text(self, node) -> bool:
        """A shell/argv command whose shown text names an execution path, even inside a longer string."""
        nodes = node.elts if isinstance(node, (ast.List, ast.Tuple)) else [node]
        for item in nodes:
            text = self.fold(item)
            if text is not None and mentions(text):
                return True
        return False

    def classify_call(self, node: ast.Call) -> list[tuple[str, int, str]]:
        name = canonical(node.func, self.aliases)
        tail = name.rsplit(".", 1)[-1]
        args = [*node.args, *(k.value for k in node.keywords)]
        receiver = node.func.value if isinstance(node.func, ast.Attribute) else None
        if name in {"importlib.import_module", "__import__"} and node.args:
            text = self.fold(node.args[0])
            if text is not None and dotted_execution(text):
                return [("exec", node.lineno, f"{name} of {text}")]
        is_exec = (name in EXEC_CALLS or name.startswith(EXEC_PREFIXES)
                   or (receiver is not None and tail in EXEC_METHODS))
        if is_exec:
            hit = (any(self.is_tainted(arg) or self.command_text(arg) for arg in args)
                   or (receiver is not None and tail in EXEC_METHODS and self.is_tainted(receiver)))
            if not hit:
                return []
            if name in SUBPROCESS and node.args and self.program(node.args[0]) == "git":
                return [] if self.reader() else [("read", node.lineno, "git reads an execution path")]
            return [("exec", node.lineno, f"{name} with an execution path")]
        if name in READ_CALLS:
            target = node.args[0] if node.args else next((k.value for k in node.keywords
                                                         if k.arg in {"file", "path", "src", "top", "pathname"}), None)
            if target is not None and self.is_tainted(target) and not self.reader() and self.path not in self.classified:
                return [("read", node.lineno, f"{name} reads an execution path")]
            return []
        if receiver is not None and tail in READ_METHODS:
            if self.is_tainted(receiver) and not self.reader() and self.path not in self.classified:
                return [("read", node.lineno, f"{name} reads an execution path")]
            return []
        direct = [arg for arg in args if self.is_tainted(arg)
                  and not isinstance(arg, (ast.List, ast.Tuple, ast.Set, ast.ListComp, ast.SetComp, ast.GeneratorExp))]
        if not direct:
            return []
        if (tail in BARRIERS or name in PURE or tail in PURE or (receiver is not None and tail in PURE_METHODS)
                or (receiver is not None and tail.startswith("assert"))):
            return []
        if isinstance(node.func, ast.Name) and node.func.id in self.functions:
            return []  # followed through its parameters
        if self.reader() or self.path in self.classified:
            return []
        return [("unclassified", node.lineno, f"execution path passed to {name}")]

    def program(self, node) -> str | None:
        if isinstance(node, (ast.List, ast.Tuple)) and node.elts:
            text = self.fold(node.elts[0])
        else:
            text = self.fold(node)
            text = text.split()[0] if text and text.split() else None
        return PurePosixPath(text).name if text else None

    def reader(self) -> bool:
        return any(self.path == r or (r.endswith("/") and self.path.startswith(r)) for r in SPEC_READERS)


def module_name(path: str) -> list[str]:
    parts = PurePosixPath(path).with_suffix("").parts
    names = [".".join(parts)]
    if parts and parts[-1] == "__init__":
        names.append(".".join(parts[:-1]))
    for prefix in ("tests/", "tools/ci/", "tools/software_workflow/"):
        if path.startswith(prefix):
            names.append(".".join(PurePosixPath(path[len(prefix):]).with_suffix("").parts))
    return names


def python_findings(sources: dict[str, bytes], classified: dict[str, str]) -> list[tuple[str, str, int, str]]:
    trees, problems = {}, []
    for path, data in sources.items():
        try:
            trees[path] = ast.parse(data, filename=path)
        except (SyntaxError, ValueError) as error:
            if b"execution" in data:
                problems.append(("unclassified", path, getattr(error, "lineno", 0) or 0,
                                 "unparsable Python naming execution"))
    exported: dict[str, frozenset[str]] = {}
    files = {}
    for _ in range(MAX_EXPORT_ROUNDS):
        # Every file: an execution path may be assembled from fragments that never spell it.
        files = {path: PythonFile(path, tree, exported, classified) for path, tree in trees.items()}
        previous, exported = exported, {}
        for path, analysis in files.items():
            names = analysis.module_names()
            if names:
                for name in module_name(path):
                    exported[name] = names
        if exported == previous:
            break
    else:
        problems.append(("unclassified", "tools/validation/execution_boundary.py", 0,
                         f"Python exported-path analysis did not converge in {MAX_EXPORT_ROUNDS} rounds"))
    for path, analysis in files.items():
        problems.extend((kind, path, line, message) for kind, line, message in analysis.findings())
    return problems


# ------------------------------------------------------------------------------------- other languages
STRING = re.compile(r'r#*"(?:[^"]|"(?!#))*"#*|"(?:\\.|[^"\\\n])*"|\'(?:\\.|[^\'\\\n])*\'|`(?:\\.|[^`\\])*`')
RUST_EXEC = re.compile(r"\binclude(_str|_bytes)?!|#\s*!?\[\s*path\s*=")
RUST_IGNORE = re.compile(r"starts_with|ends_with|contains\(|==|!=|matches!|\":!")
JS_EXEC = re.compile(r"\bimport\b|\brequire\s*\(|new\s+URL\s*\(|readFile|createReadStream|readdir|opendir|"
                     r"\bfetch\s*\(|\bspawn|\bexecFile|\bexecSync|\bexec\s*\(|\bfork\s*\(|\?raw|\?url|"
                     r"existsSync|statSync|\bglob\b|\bcp\s*\(|copyFile|loadFile|addInitScript|webServer")


def strip_comments(text: str, line: str = "//", block: bool = True) -> str:
    """Blank comments while keeping string literals and line numbers (lexical, best effort)."""
    out, i, n = [], 0, len(text)
    while i < n:
        if text.startswith(line, i):
            j = text.find("\n", i)
            j = n if j < 0 else j
            out.append(" " * (j - i))
            i = j
        elif block and text.startswith("/*", i):
            j = text.find("*/", i + 2)
            j = n if j < 0 else j + 2
            out.append(re.sub(r"[^\n]", " ", text[i:j]))
            i = j
        else:
            match = STRING.match(text, i) if text[i] in "\"'`r" else None
            if match and match.group(0)[0] in "\"'`" or (match and text[i] == "r" and (i == 0 or not text[i - 1].isalnum())):
                out.append(match.group(0))
                i = match.end()
            else:
                out.append(text[i])
                i += 1
    return "".join(out)


def literal_sites(text: str):
    for match in STRING.finditer(text):
        body = match.group(0).strip("r#").strip("\"'`")
        if is_seed(body) and re.search(r"[/\\]", body):
            line = text.count("\n", 0, match.start()) + 1
            yield line, body


def rust_findings(path: str, text: str):
    text = strip_comments(text)
    lines = text.splitlines()
    for number, body in literal_sites(text):
        context = lines[number - 1]
        if RUST_EXEC.search(context):
            yield ("exec", path, number, "Rust include/path of an execution path")
        elif not RUST_IGNORE.search(context):
            yield ("unclassified", path, number, f"Rust reference to {body}")
    for number, context in enumerate(lines, 1):
        if re.search(r'(join|Path::new|PathBuf::from)\(\s*"execution"\s*\)', context):
            yield ("unclassified", path, number, "Rust path component execution")


def js_findings(path: str, text: str, config: bool):
    text = strip_comments(text)
    lines = text.splitlines()
    for number, body in literal_sites(text):
        window = "\n".join(lines[max(0, number - 3):number])
        if config:
            yield ("exec", path, number, f"build/test configuration names {body}")
        elif JS_EXEC.search(window):
            yield ("exec", path, number, f"JS/TS load of {body}")
    for number, context in enumerate(lines, 1):
        if re.search(r'(join|resolve)\([^)]*["\']execution["\']', context):
            yield ("exec", path, number, "JS/TS path component execution")


def names_tree(token: str, tops: frozenset[str]) -> bool:
    """A lexical token naming execution/ itself or an entry under one of its tracked top-level names."""
    bare = token.strip("{}[]$").replace("\\", "/")
    if EXCLUSION.match(bare) or not is_seed(bare) and bare != "execution":
        return False
    after = re.split(r"(?:^|/)execution(?:/|$)", bare, maxsplit=1)[-1]
    return not after or after.split("/", 1)[0] in tops


def script_findings(path: str, text: str, tops: frozenset[str], comment: str = "#"):
    for number, line in enumerate(text.splitlines(), 1):
        code = line.split(comment, 1)[0] if comment else line
        for token in re.findall(r"[^\s'\"`=(),;]+", code):
            if names_tree(token, tops):
                yield ("exec", path, number, f"script names {token}")


def workflow_findings(path: str, text: str, tops: frozenset[str]):
    for number, line in enumerate(text.splitlines(), 1):
        code = line.split(" #", 1)[0] if not line.lstrip().startswith("#") else ""
        for token in re.findall(r"[^\s'\"`=(),;]+", code):
            if names_tree(token, tops):
                yield ("exec", path, number, f"workflow names {token}")


def walk_json(value, keys=()):
    if isinstance(value, dict):
        for key, item in value.items():
            yield from walk_json(item, (*keys, str(key)))
    elif isinstance(value, list):
        for item in value:
            yield from walk_json(item, keys)
    elif isinstance(value, str):
        yield keys, value


REGISTRY_KEYS = {"command", "commands", "cwd", "run", "args", "argv", "script", "scripts", "entrypoint", "main",
                 "pytest_paths", "exec", "bin", "module", "include", "files", "testDir", "testMatch", "paths_exec"}


def registry_findings(path: str, data: bytes, config: bool):
    try:
        value = json.loads(data)
    except (ValueError, UnicodeError):
        return
    for keys, text in walk_json(value):
        if mentions(text) and (config or REGISTRY_KEYS & set(keys)):
            yield ("exec", path, 0, f"{'/'.join(keys)} names {text}")


def toml_findings(path: str, data: bytes):
    try:
        value = tomllib.loads(data.decode("utf-8"))
    except (tomllib.TOMLDecodeError, UnicodeError):
        if b"execution" in data:
            yield ("unclassified", path, 0, "unparsable TOML naming execution")
        return
    for keys, text in walk_json(value):
        if (mentions(text) or text == "execution") and not any("exclude" in key for key in keys):
            yield ("exec", path, 0, f"{'.'.join(keys)} names {text}")


# --------------------------------------------------------------------------------------------- the tree
def shebang_language(data: bytes) -> str | None:
    if not data.startswith(b"#!") or data.startswith(b"#!["):
        return None
    header = data.split(b"\n", 1)[0].decode("utf-8", "replace")
    if re.search(r"\bpython(?:\d+(?:\.\d+)*)?\b", header):
        return "python"
    if re.search(r"\b(node|nodejs|deno|bun)\b", header):
        return "javascript"
    if re.search(r"\b(sh|bash|dash|ksh|zsh|ash|fish)\b", header):
        return "shell"
    if re.search(r"\b(pwsh|powershell)\b", header):
        return "powershell"
    return "unknown"


def tracked(root: Path) -> list[tuple[str, str, str]]:
    raw = subprocess.check_output(["git", "-C", str(root), "ls-files", "--stage", "-z"])
    entries = []
    for record in filter(None, raw.split(b"\0")):
        info, path = record.split(b"\t", 1)
        mode, oid, stage = info.decode().split()
        if stage != "0":
            raise ValueError("unmerged index")
        entries.append((mode, oid, path.decode("utf-8")))
    return entries


def blobs(root: Path, oids: list[str]) -> dict[str, bytes]:
    if not oids:
        return {}
    process = subprocess.run(["git", "-C", str(root), "cat-file", "--batch"], input="\n".join(oids).encode() + b"\n",
                             capture_output=True, check=True)
    out, data, i = {}, process.stdout, 0
    for oid in oids:
        end = data.index(b"\n", i)
        header = data[i:end].split()
        if len(header) != 3:
            raise ValueError(f"missing blob {oid}")
        size = int(header[2])
        out[oid] = data[end + 1:end + 1 + size]
        i = end + 1 + size + 1
    return out


def execution_findings(entries, contents, fidelity) -> list[tuple[str, str, int, str]]:
    found, present = [], set()
    for mode, oid, path in entries:
        if not declarative(path):
            continue
        name = PurePosixPath(path).name
        if mode != "100644":
            found.append(("tree", path, 0, f"mode {mode} (executable, link or submodule) in execution/"))
            continue
        data = contents[oid]
        if path in fidelity:
            present.add(path)
            if hashlib.sha256(data).hexdigest() != fidelity[path]:
                found.append(("tree", path, 0, "accepted SCA-011 helper bytes changed"))
            continue
        if name in DISCOVERY_NAMES or name.startswith("tsconfig") or name.endswith(".pth"):
            found.append(("tree", path, 0, "tool-discovery file in execution/"))
        elif name == ".gitattributes":
            for number, line in enumerate(data.decode("utf-8", "replace").splitlines(), 1):
                fields = line.split("#", 1)[0].split()
                if fields and not all(GITATTRIBUTES_ALLOWED.match(f) for f in fields[1:]):
                    found.append(("tree", path, number, "gitattributes beyond whitespace/text/eol"))
        elif not name.endswith(DECLARATIVE_SUFFIXES):
            found.append(("tree", path, 0, "not a declarative specification file kind"))
        elif b"\0" in data or data.startswith(b"#!"):
            found.append(("tree", path, 0, "binary or shebang-led content"))
        else:
            try:
                data.decode("utf-8")
            except UnicodeDecodeError:
                found.append(("tree", path, 0, "not UTF-8 text"))
    for path in sorted(set(fidelity) - present):
        found.append(("tree", path, 0, "accepted SCA-011 helper missing or not a regular 100644 file"))
    return found


def check(root: Path = ROOT, fidelity: dict[str, str] = FIDELITY,
          classified: dict[str, str] = CLASSIFIED) -> list[tuple[str, str, int, str]]:
    entries = tracked(root)
    # Read headers regardless of suffix: a regular non-executable file may still be
    # invoked by an interpreter. Only executable/shebang sources are analyzed below;
    # other unknown formats remain data rather than acquiring executable exemptions.
    wanted = [oid for mode, oid, path in entries if mode != "160000"]
    contents = blobs(root, sorted(set(wanted)))
    found = execution_findings(entries, contents, fidelity)
    tops = frozenset(path.split("/")[1] for _, _, path in entries if declarative(path) and path.count("/") >= 1)
    python = {}
    for mode, oid, path in entries:
        if declarative(path) or oid not in contents:
            continue
        data = contents[oid]
        name = PurePosixPath(path).name
        if mode == "120000":
            target = (PurePosixPath(path).parent / data.decode("utf-8", "replace")).as_posix()
            parts = []
            for part in target.split("/"):
                if part == "..":
                    parts = parts[:-1]
                elif part not in ("", "."):
                    parts.append(part)
            if "/".join(parts).startswith(DECLARATIVE_ROOT) or "/".join(parts) == "execution":
                found.append(("exec", path, 0, "symlink into execution/"))
            continue
        language = shebang_language(data)
        if language == "unknown" or (mode == "100755" and language is None
                                      and not path.endswith((".py", ".sh", ".bash", ".zsh", ".js", ".mjs",
                                                             ".cjs", ".ps1", ".psm1", ".bat", ".cmd"))):
            found.append(("unclassified", path, 1, "executable entrypoint has an unclassified interpreter"))
            continue
        if b"execution" not in data and not path.endswith(".py") and language != "python":
            continue
        text = data.decode("utf-8", "replace")
        if path.endswith(".py") or language == "python":
            python[path] = data
        elif path.endswith(".rs"):
            found.extend(rust_findings(path, text))
        elif path.endswith((".ts", ".tsx", ".js", ".mjs", ".cjs", ".mts", ".cts", ".jsx")) or language == "javascript":
            found.extend(js_findings(path, text, config=re.search(r"\.config\.|(^|/)vite|playwright", name) is not None))
        elif path.endswith((".sh", ".bash", ".zsh", ".pth")) or name in {"Makefile", "justfile"} or language == "shell":
            found.extend(script_findings(path, text, tops))
        elif path.endswith((".ps1", ".psm1")) or language == "powershell":
            found.extend(script_findings(path, text, tops))
        elif path.endswith((".bat", ".cmd")):
            found.extend(script_findings(path, text, tops, comment="REM "))
        elif path.startswith(".github/") and path.endswith((".yml", ".yaml")):
            found.extend(workflow_findings(path, text, tops))
        elif path.endswith(".toml"):
            found.extend(toml_findings(path, data))
        elif path.endswith((".ini", ".cfg")):
            found.extend(script_findings(path, text, tops, comment="#"))
        elif path.endswith(".json") and (name == "package.json" or name.startswith("tsconfig")
                                         or path == "software-workflow.json" or re.fullmatch(r"tools/[^/]*\.json", path)):
            found.extend(registry_findings(path, data, config=name == "package.json" or name.startswith("tsconfig")))
    found.extend(python_findings(python, classified))
    known = {path for _, _, path in entries}
    for path in classified:
        if path not in known:
            found.append(("stale", path, 0, "classification names an untracked file"))
    return sorted(set(found), key=lambda f: (f[1], f[2], f[0], f[3]))


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    parser.add_argument("--root", type=Path, default=ROOT)
    args = parser.parse_args()
    found = check(args.root.resolve())
    for kind, path, line, message in found:
        print(f"{path}:{line}: [{kind}] {message}")
    print(f"execution boundary: {len(found)} finding(s); {len(FIDELITY)} fidelity-only exceptions hash-checked")
    return 1 if found else 0


if __name__ == "__main__":
    raise SystemExit(main())
