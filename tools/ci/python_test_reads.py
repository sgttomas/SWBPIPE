#!/usr/bin/env python3
"""The project paths each maintained Python test reads, derived from the test's own code.

A test is selected when a changed path is, or lies under, one of its roots:
- a project path written as one string literal (cut at a format slot or glob);
- a path built by `/` (or `joinpath`, `with_name`) from string parts, from a project-root or
  `__file__`-anchored base, including module-level names bound to such paths, cut where a
  non-literal part begins;
- each project Python file it imports, resolved from the project root, `tests/`, its own
  directory and any directory it adds to `sys.path`, and the files those import in turn;
- each project Python file it names as a path (loaded by spec, `runpy` or a subprocess), with
  that file's own roots: such a file holds the test's scanning code, as a guard's model does.
An imported module's own path reads are product behaviour, not the test's roots, and are not
followed: through the binary build helpers they would tie every runner-backed test to every
Rust change, which the Cargo jobs cover.

A root must be a tracked file or directory below the top level (`git ls-files`, so a sparse
checkout derives the same roots). Roots under `execution/` other than `execution/_DAG/` are
dropped: the Python job does not check them out, so no test there can depend on their content.
"""
import ast
import functools
from pathlib import Path
import subprocess

# Tests the Repository checks job runs on every PR (`.github/workflows/governance-harness.yml`);
# `tests/test_ci_python.py` keeps this list equal to that step's.
REPOSITORY_CHECK_TESTS = ('tools/software_workflow', 'tools/deliverables', 'tools/validation',
                          'tests/test_ci_numerical.py', 'tests/test_ci_apt_sources.py', 'tests/test_release_members.py',
                          'tests/test_release_candidate_scan.py', 'tests/test_repository_history.py')
# The selector's own tests name paths as selection inputs, not reads; `tools/ci/` changes select them.
SELECTOR_TESTS = 'tests/test_ci_'


@functools.lru_cache(maxsize=None)
def tracked(project):
    """Every tracked file and its ancestor directories, project-relative."""
    files = subprocess.run(['git', '-C', str(project), 'ls-files', '-z'], capture_output=True, check=True).stdout
    paths = set()
    for name in files.decode('utf-8').split('\0'):
        if name:
            parts = name.split('/')
            paths.update('/'.join(parts[:end]) for end in range(1, len(parts) + 1))
    return frozenset(paths)


def is_test(local):
    name = Path(local).name
    return local.endswith('.py') and (name.startswith('test_') or name.endswith('_test.py'))


@functools.lru_cache(maxsize=None)
def maintained_tests(project):
    """Every tracked Python test file except the selector's own and those the Repository checks job runs."""
    paths = tracked(project)
    return tuple(sorted(local for local in paths if is_test(local) and (project / local).is_file()
                        and not local.startswith(SELECTOR_TESTS)
                        and not any(local == suite or local.startswith(suite + '/') for suite in REPOSITORY_CHECK_TESTS)))


class _Reader:
    """Resolves one Python file's path expressions and imports."""

    def __init__(self, project, local):
        self.project, self.local = project, local
        self.tree = ast.parse((project / local).read_text(encoding='utf-8'), filename=local)
        self.names, self.memo = {}, {}
        for node in self.tree.body:
            if isinstance(node, ast.Assign) and len(node.targets) == 1 and isinstance(node.targets[0], ast.Name):
                self.names[node.targets[0].id] = node.value

    def base(self, node, depth=0):
        """The project-relative path an expression names, or None ('' is the project root)."""
        if depth > 12:
            return None
        key = id(node)
        if key not in self.memo:
            self.memo[key] = self._base(node, depth)
        return self.memo[key]

    def _base(self, node, depth):
        if isinstance(node, ast.Name):
            if node.id == '__file__':
                return self.local
            value = self.names.get(node.id)
            return self.base(value, depth + 1) if value is not None else None
        if isinstance(node, ast.Call):
            func = node.func
            if isinstance(func, ast.Name) and func.id in {'Path', 'PurePath', 'str'} and len(node.args) == 1:
                return self.base(node.args[0], depth + 1)
            if isinstance(func, ast.Attribute) and func.attr in {'resolve', 'absolute'}:
                return self.base(func.value, depth + 1)
            if isinstance(func, ast.Attribute) and func.attr == 'with_name' and len(node.args) == 1:
                inner = self.base(func.value, depth + 1)
                name = node.args[0]
                if inner is not None and isinstance(name, ast.Constant) and isinstance(name.value, str):
                    return _join(_parent(inner, 1), [name.value])
                return None
            if isinstance(func, ast.Attribute) and func.attr == 'joinpath':
                start = self.base(func.value, depth + 1)
                parts = [arg.value for arg in node.args if isinstance(arg, ast.Constant) and isinstance(arg.value, str)]
                return _join(start, parts) if start is not None and len(parts) == len(node.args) else None
            if isinstance(func, ast.Attribute) and func.attr == 'dirname' and len(node.args) == 1:
                inner = self.base(node.args[0], depth + 1)
                return _parent(inner, 1) if inner is not None else None
            if isinstance(func, ast.Attribute) and func.attr == 'abspath' and len(node.args) == 1:
                return self.base(node.args[0], depth + 1)
            return None
        if isinstance(node, ast.Attribute) and node.attr == 'parent':
            inner = self.base(node.value, depth + 1)
            return _parent(inner, 1) if inner is not None else None
        if (isinstance(node, ast.Subscript) and isinstance(node.value, ast.Attribute) and node.value.attr == 'parents'
                and isinstance(node.slice, ast.Constant) and isinstance(node.slice.value, int)):
            inner = self.base(node.value.value, depth + 1)
            return _parent(inner, node.slice.value + 1) if inner is not None else None
        if isinstance(node, ast.BinOp) and isinstance(node.op, ast.Div):
            left = self.base(node.left, depth + 1)
            right = self.names.get(node.right.id, node.right) if isinstance(node.right, ast.Name) else node.right
            if left is not None and isinstance(right, ast.Constant) and isinstance(right.value, str):
                return _join(left, [right.value])
            return None
        return None

    def chain_paths(self):
        """Paths built from a resolvable base: each longest resolvable expression, so a chain that
        continues with a non-literal part is cut there."""
        parents = {child: node for node in ast.walk(self.tree) for child in ast.iter_child_nodes(node)}
        found = set()
        for node in ast.walk(self.tree):
            if not isinstance(node, (ast.BinOp, ast.Call, ast.Attribute, ast.Subscript)):
                continue
            path = self.base(node)
            if path is None or path == self.local:
                continue
            if node in parents and self.base(parents[node]) is not None:
                continue
            found.add(path)
        return found

    def literal_paths(self):
        """Project paths written in one string literal, cut at a format slot."""
        found = set()
        for node in ast.walk(self.tree):
            if isinstance(node, ast.Constant) and isinstance(node.value, str):
                texts = [(node.value, False)]
            elif isinstance(node, ast.JoinedStr) and node.values and isinstance(node.values[0], ast.Constant):
                texts = [(node.values[0].value, len(node.values) > 1)]
            else:
                continue
            for text, cut in texts:
                for word in text.replace(',', ' ').split():
                    word = word.strip('\'"`()[]<>:;')
                    for stop in '*?{$#':
                        if stop in word:
                            word, cut = word[:word.index(stop)], True
                    if '/' in word and not word.startswith(('/', '.')):
                        found.add(_parent(word, 1) if cut and not word.endswith('/') else word.rstrip('/'))
        return found

    def search_dirs(self):
        """The project directories this file's imports may resolve from."""
        dirs = ['', _parent(self.local, 1)]
        for node in ast.walk(self.tree):
            if isinstance(node, ast.Call) and isinstance(node.func, ast.Attribute) and node.args:
                if node.func.attr in {'insert', 'append', 'syspath_prepend'}:
                    path = self.base(node.args[-1])
                    if path is not None:
                        dirs.append(path)
        return dirs

    def imports(self):
        names = set()
        package = _parent(self.local, 1).replace('/', '.')
        for node in ast.walk(self.tree):
            if isinstance(node, ast.Import):
                names.update(alias.name for alias in node.names)
            elif isinstance(node, ast.ImportFrom):
                if node.level:
                    parts = package.split('.') if package else []
                    anchor = parts[:len(parts) - node.level + 1]
                    module = '.'.join(anchor + ([node.module] if node.module else []))
                else:
                    module = node.module or ''
                if module:
                    names.add(module)
                names.update(f'{module}.{alias.name}' if module else alias.name for alias in node.names)
        return names


def _parent(local, levels):
    parts = local.split('/') if local else []
    return '/'.join(parts[:max(len(parts) - levels, 0)])


def _join(base, parts):
    joined = '/'.join(filter(None, [base] + [part.strip('/') for part in parts]))
    return str(Path(joined).as_posix()) if joined else ''


def _module_file(project, paths, directory, module):
    relative = module.replace('.', '/')
    for candidate in (f'{relative}.py', f'{relative}/__init__.py'):
        local = f'{directory}/{candidate}' if directory else candidate
        if local in paths and (project / local).is_file():
            return local
    return None


@functools.lru_cache(maxsize=None)
def _reader(project, local):
    return _Reader(project, local)


@functools.lru_cache(maxsize=None)
def file_reads(project, local):
    """One Python file's usable named paths, the project Python files among them (loaded by path)
    and the project Python files it imports."""
    paths = tracked(project)
    reader = _reader(project, local)
    named = {path.rstrip('/') for path in reader.literal_paths() | reader.chain_paths()}
    roots = frozenset(path for path in named if _usable(paths, path))
    loaded = frozenset(path for path in roots if path.endswith('.py') and path != local and (project / path).is_file())
    imported = set()
    dirs = reader.search_dirs()
    for module in reader.imports():
        for directory in dirs:
            target = _module_file(project, paths, directory, module)
            if target:
                imported.add(target)
                break
    imported.discard(local)
    return roots, loaded, frozenset(imported)


def _usable(paths, local):
    if '/' not in local or local not in paths:
        return False
    return not (local.startswith('execution/') and not local.startswith('execution/_DAG'))


@functools.lru_cache(maxsize=None)
def test_reads(project):
    """{test: sorted project roots it reads} for every maintained test."""
    reads = {}
    for test in maintained_tests(project):
        roots, scanned, imported = set(), set(), set()
        pending = [test]
        while pending:
            local = pending.pop()
            if local in scanned:
                continue
            scanned.add(local)
            named, loaded, imports = file_reads(project, local)
            roots |= named
            pending.extend(loaded)
            imported |= imports
        # Imported modules are roots, and so are the modules they import; their own path reads are not.
        pending, followed = sorted(imported), set()
        while pending:
            local = pending.pop()
            if local not in followed:
                followed.add(local)
                pending.extend(file_reads(project, local)[2])
        roots |= followed
        roots.discard(test)
        reads[test] = tuple(sorted(roots))
    return reads


def reading_tests(project, changed):
    """{test: changed paths it reads} for the maintained tests a change reaches."""
    selected = {}
    for test, roots in test_reads(project).items():
        hits = [path for path in changed if any(path == root or path.startswith(root + '/') for root in roots)]
        if hits:
            selected[test] = hits
    return selected
