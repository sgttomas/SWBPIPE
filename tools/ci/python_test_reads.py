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
from fnmatch import fnmatchcase
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


GLOB = set('*?[')
# A test may declare reads its code cannot show (a path assembled at run time); each entry must stay a
# tracked path or a pattern whose literal prefix is tracked (`tests/test_ci_python.py` checks them).
DECLARED_READS = 'CI_READS'


def is_pattern(root):
    return bool(GLOB & set(root))


def root_matches(path, root):
    """Whether a changed path is a root, lies under it, or matches it as a glob (`**` spans directories)."""
    if not is_pattern(root):
        return path == root or path.startswith(root + '/')
    return _segments_match(path.split('/'), root.split('/'))


def _segments_match(parts, pattern):
    if not pattern:
        return not parts
    if pattern[0] == '**':
        return any(_segments_match(parts[index:], pattern[1:]) for index in range(len(parts) + 1))
    return bool(parts) and fnmatchcase(parts[0], pattern[0]) and _segments_match(parts[1:], pattern[1:])


class _Reader:
    """Resolves one Python file's path expressions and imports. A path expression resolves to the set of
    project-relative paths it can name ('' is the project root), or None when any part is unknown; a name
    resolves through every binding the file gives it: assignments, `for` targets, comprehensions and
    `pytest.mark.parametrize` arguments."""

    def __init__(self, project, local):
        self.project, self.local = project, local
        self.tree = ast.parse((project / local).read_text(encoding='utf-8'), filename=local)
        self.memo, self.bindings, self.scopes, loops = {}, {}, {}, []

        def visit(node, scope):
            for child in ast.iter_child_nodes(node):
                inner = scope
                if isinstance(child, (ast.FunctionDef, ast.AsyncFunctionDef, ast.Lambda)):
                    inner = child
                    for decorator in getattr(child, 'decorator_list', []):
                        if isinstance(decorator, ast.Call) and _attr(decorator.func) == 'parametrize' and len(decorator.args) >= 2:
                            self._parametrize(child, decorator.args[0], decorator.args[1])
                self.scopes[child] = inner
                if isinstance(child, (ast.Assign, ast.AnnAssign)) and child.value is not None:
                    for target in (child.targets if isinstance(child, ast.Assign) else [child.target]):
                        self._bind(scope, target, child.value)
                elif isinstance(child, (ast.For, ast.comprehension)):
                    loops.append((scope, child))
                visit(child, inner)
        visit(self.tree, self.tree)
        # Loop targets bind each element of a literal sequence, or of a name bound to one.
        for scope, node in loops:
            for value in self._elements(node.iter, set()):
                self._bind(scope, node.target, value)

    def _lookup(self, node):
        """A name's bindings in its own function, else in the module."""
        scope = self.scopes.get(node, self.tree)
        return self.bindings.get((scope, node.id)) or self.bindings.get((self.tree, node.id)) or []

    def _elements(self, node, seen):
        if isinstance(node, (ast.Tuple, ast.List, ast.Set)):
            return list(node.elts)
        if isinstance(node, ast.Name) and node.id not in seen:
            seen = seen | {node.id}
            return [element for value in self._lookup(node) for element in self._elements(value, seen)]
        return []

    def _bind(self, scope, target, value):
        if isinstance(target, ast.Name):
            self.bindings.setdefault((scope, target.id), []).append(value)
        elif isinstance(target, (ast.Tuple, ast.List)) and isinstance(value, (ast.Tuple, ast.List)) \
                and len(target.elts) == len(value.elts):
            for part, element in zip(target.elts, value.elts):
                self._bind(scope, part, element)

    def _parametrize(self, scope, names, values):
        if isinstance(names, ast.Constant) and isinstance(names.value, str):
            names = [name.strip() for name in names.value.split(',') if name.strip()]
        elif isinstance(names, (ast.Tuple, ast.List)) and all(isinstance(n, ast.Constant) for n in names.elts):
            names = [n.value for n in names.elts]
        else:
            return
        for value in (values.elts if isinstance(values, (ast.Tuple, ast.List)) else []):
            if isinstance(value, ast.Call) and _attr(value.func) == 'param':
                value = ast.Tuple(elts=list(value.args), ctx=ast.Load()) if len(names) > 1 else (value.args or [None])[0]
            if value is None:
                continue
            if len(names) == 1:
                self.bindings.setdefault((scope, names[0]), []).append(value)
            elif isinstance(value, (ast.Tuple, ast.List)) and len(value.elts) == len(names):
                for name, element in zip(names, value.elts):
                    self.bindings.setdefault((scope, name), []).append(element)

    def strings(self, node, depth=0):
        """Every string an expression can be, or None."""
        if depth > 12:
            return None
        if isinstance(node, ast.Constant) and isinstance(node.value, str):
            return frozenset([node.value])
        if isinstance(node, ast.JoinedStr):
            # Each slot is its finite values, else a `*` that matches any one path segment's text.
            texts = {''}
            for part in node.values:
                if isinstance(part, ast.Constant):
                    values = {str(part.value)}
                else:
                    values = self.strings(part.value, depth + 1) if part.conversion == -1 and part.format_spec is None else None
                    values = values if values is not None and '/' not in ''.join(values) else {'*'}
                texts = {text + value for text in texts for value in values}
                if len(texts) > 64:
                    return frozenset({_star(node)})
            return frozenset(texts)
        if isinstance(node, ast.Name) and self._lookup(node):
            found = set()
            for value in self._lookup(node):
                values = self.strings(value, depth + 1)
                if values is None:
                    return None
                found |= values
            return frozenset(found)
        return None

    def paths(self, node, depth=0):
        if depth > 12:
            return None
        key = id(node)
        if key not in self.memo:
            self.memo[key] = None  # a self-referential binding resolves to nothing
            self.memo[key] = self._paths(node, depth)
        return self.memo[key]

    def _paths(self, node, depth):
        if isinstance(node, ast.Name):
            if node.id == '__file__':
                return frozenset([self.local])
            values = self._lookup(node)
            if not values:
                return None
            found = set()
            for value in values:
                resolved = self.paths(value, depth + 1)
                if resolved is None:
                    return None
                found |= resolved
            return frozenset(found)
        if isinstance(node, ast.Call):
            func, attr = node.func, _attr(node.func)
            if isinstance(func, ast.Name) and func.id in {'Path', 'PurePath', 'str'} and len(node.args) == 1:
                return self.paths(node.args[0], depth + 1)
            if attr in {'resolve', 'absolute'}:
                return self.paths(func.value, depth + 1)
            if attr == 'with_name' and len(node.args) == 1:
                inner, names = self.paths(func.value, depth + 1), self.strings(node.args[0])
                return _product(inner and {_parent(path, 1) for path in inner}, names)
            if attr == 'joinpath':
                result = self.paths(func.value, depth + 1)
                for arg in node.args:
                    result = _product(result, self.strings(arg))
                return result
            if attr in {'dirname', 'abspath'} and len(node.args) == 1:
                inner = self.paths(node.args[0], depth + 1)
                return inner if attr == 'abspath' or inner is None else frozenset(_parent(path, 1) for path in inner)
            return None
        if isinstance(node, ast.Attribute) and node.attr == 'parent':
            inner = self.paths(node.value, depth + 1)
            return None if inner is None else frozenset(_parent(path, 1) for path in inner)
        if (isinstance(node, ast.Subscript) and isinstance(node.value, ast.Attribute) and node.value.attr == 'parents'
                and isinstance(node.slice, ast.Constant) and isinstance(node.slice.value, int)):
            inner = self.paths(node.value.value, depth + 1)
            return None if inner is None else frozenset(_parent(path, node.slice.value + 1) for path in inner)
        if isinstance(node, ast.BinOp) and isinstance(node.op, ast.Div):
            return _product(self.paths(node.left, depth + 1), self.strings(node.right))
        return None

    def chain_paths(self):
        """Paths built from a resolvable base: each longest resolvable expression, and the resolvable base of
        a chain that continues with an unknown part (cut there)."""
        parents = {child: node for node in ast.walk(self.tree) for child in ast.iter_child_nodes(node)}
        found = set()
        for node in ast.walk(self.tree):
            if not isinstance(node, (ast.BinOp, ast.Call, ast.Attribute, ast.Subscript, ast.Name)):
                continue
            paths = self.paths(node)
            if paths is None:
                continue
            parent = parents.get(node)
            if parent is not None and self.paths(parent) is not None:
                continue
            found |= paths - {self.local}
        return found

    def scan_patterns(self):
        """Directory scans over a resolvable base: `glob`, `rglob`, `iterdir` and `os.walk`, as patterns."""
        found = set()
        for node in ast.walk(self.tree):
            if not isinstance(node, ast.Call):
                continue
            attr = _attr(node.func)
            if attr in {'glob', 'rglob', 'iterdir'} and isinstance(node.func, ast.Attribute):
                bases = self.paths(node.func.value)
                patterns = frozenset(['*']) if attr == 'iterdir' else (self.strings(node.args[0]) if node.args else None)
                if bases is None or patterns is None:
                    continue
                for base in bases:
                    for pattern in patterns:
                        pattern = ('**/' + pattern) if attr == 'rglob' else pattern
                        found.add(_join(base, [pattern]))
            elif attr == 'walk' and node.args:
                for base in self.paths(node.args[0]) or ():
                    found.add(_join(base, ['**']))
        return found

    def literal_paths(self):
        """Project paths written in one string literal or f-string (a slot becomes its finite values, else a
        `*` pattern), cut where other non-path text begins; a `/` operand is left to `chain_paths`."""
        found = set()
        parents = {child: node for node in ast.walk(self.tree) for child in ast.iter_child_nodes(node)}
        for node in ast.walk(self.tree):
            parent = parents.get(node)
            if isinstance(parent, ast.BinOp) and isinstance(parent.op, ast.Div):
                continue  # a `/` operand: chain_paths reads it from its base, and a temporary base is no read
            if isinstance(node, ast.Constant) and isinstance(node.value, str):
                if isinstance(parent, ast.JoinedStr):
                    continue
                texts = [node.value]
            elif isinstance(node, ast.JoinedStr):
                texts = sorted(self.strings(node) or ())
            else:
                continue
            for text in texts:
                for word in text.replace(',', ' ').split():
                    word = word.strip('\'"`()[]<>:;')
                    cut = False
                    for stop in '[{$#':
                        if stop in word:
                            word, cut = word[:word.index(stop)], True
                    if '/' in word and not word.startswith(('/', '.')):
                        found.add(_parent(word, 1) if cut and not word.endswith('/') else word.rstrip('/'))
        return found

    def declared(self):
        """The module-level `CI_READS` entries, or None when absent; a non-literal entry is kept as None."""
        for value in self.bindings.get((self.tree, DECLARED_READS), []):
            return tuple(element.value if isinstance(element, ast.Constant) and isinstance(element.value, str) else None
                         for element in self._elements(value, set()))
        return None

    def search_dirs(self):
        """The project directories this file's imports may resolve from."""
        dirs = ['', _parent(self.local, 1)]
        for node in ast.walk(self.tree):
            if isinstance(node, ast.Call) and _attr(node.func) in {'insert', 'append', 'syspath_prepend'} and node.args:
                dirs.extend(self.paths(node.args[-1]) or ())
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


def _star(node):
    """An f-string as one pattern: its constant text, with `*` for every slot."""
    return ''.join(str(part.value) if isinstance(part, ast.Constant) else '*' for part in node.values)


def _attr(func):
    return func.attr if isinstance(func, ast.Attribute) else None


def _product(bases, parts):
    if bases is None or parts is None:
        return None
    return frozenset(_join(base, [part]) for base in bases for part in parts)


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
    """One Python file's usable named paths and scan patterns, the project Python files among the paths
    (loaded by path) and the project Python files it imports."""
    paths = tracked(project)
    reader = _reader(project, local)
    # A pattern written as text needs a literal directory prefix; only a scan or a declaration spans the project.
    named = {path.rstrip('/') for path in reader.literal_paths() | reader.chain_paths()
             if not is_pattern(path) or _prefix(path)}
    declared = reader.declared()
    # A declaration stands in for the file's derived scans (`declaration_problems` keeps it covering them).
    named |= {entry for entry in declared if entry} if declared is not None else reader.scan_patterns()
    roots = {path for path in named if _usable(paths, path)}
    # A named Python file is loaded and read with its own reads, unless it is another test (named, not run).
    loaded = frozenset(path for path in roots if path.endswith('.py') and path != local and not is_test(path)
                       and (project / path).is_file())
    imported = set()
    dirs = reader.search_dirs()
    for module in reader.imports():
        for directory in dirs:
            target = _module_file(project, paths, directory, module)
            if target:
                imported.add(target)
                break
    imported.discard(local)
    return frozenset(roots), loaded, frozenset(imported)


def _usable(paths, local):
    if is_pattern(local):
        return _usable_pattern(paths, local)
    if '/' not in local or local not in paths:
        return False
    return not _unavailable(local)


def _prefix(pattern):
    """A pattern's literal leading directories."""
    prefix = []
    for part in pattern.split('/'):
        if is_pattern(part):
            break
        prefix.append(part)
    return '/'.join(prefix)


def _usable_pattern(paths, pattern):
    """A scan pattern whose literal prefix is the project root or a tracked directory the Python job has."""
    prefix = _prefix(pattern)
    return (not prefix or prefix in paths) and not _unavailable(prefix) and pattern != prefix


def _unavailable(local):
    return local == 'execution' or local.startswith('execution/') and not local.startswith('execution/_DAG')


def declared_reads(project):
    """{test: its `CI_READS` entries} for every maintained test that declares reads."""
    found = {}
    for test in maintained_tests(project):
        entries = _reader(project, test).declared()
        if entries is not None:
            found[test] = entries
    return found


def declaration_problems(project):
    """Why any test's `CI_READS` is stale: a non-literal or unusable entry (no tracked path or prefix the
    Python job has), or a directory scan in the test that no declared pattern's prefix covers."""
    paths, problems = tracked(project), []
    for test, entries in declared_reads(project).items():
        for entry in entries:
            if entry is None or not _usable(paths, entry):
                problems.append((test, f'unusable entry {entry!r}'))
        prefixes = [_prefix(entry) for entry in entries if entry and is_pattern(entry)]
        for scan in _reader(project, test).scan_patterns():
            base = _prefix(scan)
            if not any(not prefix or base == prefix or base.startswith(prefix + '/') for prefix in prefixes):
                problems.append((test, f'undeclared scan {scan!r}'))
    return problems


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
        hits = [path for path in changed if any(root_matches(path, root) for root in roots)]
        if hits:
            selected[test] = hits
    return selected
