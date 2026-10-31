#!/usr/bin/env python3
"""The project paths each maintained Python test reads, derived from the test's own code.

The derivation fails toward over-selection: wherever the code does not show a value, it keeps a
superset root rather than drop the read. A test is selected when a changed path is, lies under,
or matches one of its roots:
- a project path written as a literal or an f-string, or built by `/`, `joinpath`, `with_name`,
  `os.path.join`, `str.format` or `%` from a project-root, `__file__`-anchored or unknown base. A
  name resolves through its bindings in its own scope, so a parameter or comprehension target
  shadows every outer binding: assignments, loop and `with` targets, `pytest.mark.parametrize`
  values, fixtures, defaults and the arguments of the file's own calls. Text the code does not
  show may contain `/`, so a path with an unknown part is `<known prefix>/**/<known suffix>`;
- a location outside the project (a temporary directory, the interpreter, an environment-supplied
  path) still mirrors the project when a test copies it there, so `tmp / "core/x.rs"` is
  `**/core/x.rs`; only such a location with no known text is no read;
- directory scans (`glob`, `rglob`, `iterdir`, `os.walk`, `os.listdir`, `os.scandir`, the `glob`
  module) and the pathspecs of `git ls-files`, `git grep` and `git ls-tree`, as glob patterns that
  also match files added later. A scan or pathspec the code does not show is `**`, and the file
  that makes it must declare `CI_READS` (`declaration_problems`);
- each project Python file it imports, resolved from the project root, `tests/`, its own
  directory and any directory it adds to `sys.path`, and the files those import in turn;
- each non-test Python file it names, which it may load or run, with that file's own roots: such
  a file holds the test's scanning code, as a guard's model does.
An imported module's own path reads are product behaviour, not the test's roots, and are not
followed: through the binary build helpers they would tie every runner-backed test to every
Rust change, which the Cargo jobs cover.

Roots under `execution/` other than `execution/_DAG/` are dropped: the Python job does not check
them out, so no test there can depend on their content.
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
# A test or a file it loads may declare reads its code cannot show; each entry must stay a tracked path or
# a pattern whose literal prefix is tracked (`declaration_problems`).
DECLARED_READS = 'CI_READS'

GLOB = set('*?[')
SLOT = '\0'          # text the code does not show; it may contain `/`
OUTSIDE = '\1'       # a location outside the project
CALLER = '\2'        # text a caller outside the file supplies (a parameter no call in the file shows)
TEMP_FIXTURES = {'tmp_path', 'tmpdir', 'tmp_path_factory', 'tmpdir_factory'}
OUTSIDE_CALLS = {'mkdtemp', 'gettempdir', 'TemporaryDirectory', 'NamedTemporaryFile', 'mktemp', 'home', 'which',
                 'getenv', 'expanduser'}
PATH_TYPES = {'Path', 'PurePath', 'PosixPath', 'PurePosixPath'}
SAME_PATH_CALLS = {'resolve', 'absolute', 'realpath', 'as_posix', 'relative_to', 'fspath', 'normpath', 'abspath'}
SCAN_METHODS = {'glob', 'rglob', 'iterdir'}
OS_SCANS = {'walk', 'listdir', 'scandir'}
SUBPROCESS_CALLS = {'run', 'check_output', 'check_call', 'call', 'Popen'}
GIT_PATHSPEC_COMMANDS = {'ls-files', 'grep', 'ls-tree'}
GIT_VALUED_OPTIONS = {'-C', '-c', '-e', '-f', '--format', '--exclude', '-x', '-X'}
FUNCTIONS = (ast.FunctionDef, ast.AsyncFunctionDef, ast.Lambda)
COMPREHENSIONS = (ast.ListComp, ast.SetComp, ast.DictComp, ast.GeneratorExp)
COMBINATIONS = 64


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


def is_pattern(root):
    return bool(GLOB & set(root))


def root_matches(path, root):
    """Whether a changed path is a root or lies under it; for a glob (`**` spans directories), whether the
    path or one of its directories matches it."""
    if not is_pattern(root):
        return path == root or path.startswith(root + '/')
    parts, pattern = path.split('/'), root.split('/')
    return any(_segments_match(parts[:end], pattern) for end in range(1, len(parts) + 1))


def _segments_match(parts, pattern):
    if not pattern:
        return not parts
    if pattern[0] == '**':
        return any(_segments_match(parts[index:], pattern[1:]) for index in range(len(parts) + 1))
    return bool(parts) and fnmatchcase(parts[0], pattern[0]) and _segments_match(parts[1:], pattern[1:])


def to_roots(text):
    """A project-relative path text, possibly with unknown text, as roots. A wholly unknown segment is `**`; a
    segment partly unknown is both itself with `*` (the unknown text within it) and `prefix*/**/*suffix` (the
    unknown text spanning directories). An outside location keeps only the known text after it, anywhere
    (`**/...`), and is no root without any; a caller-supplied part with no known text after it is no root."""
    if not text.replace(OUTSIDE, '').replace(CALLER, '').replace(SLOT, '').strip('/'):
        return set() if OUTSIDE in text or CALLER in text or not text.strip('/') else {'**'}
    text = text.replace(OUTSIDE, SLOT).replace(CALLER, SLOT).replace('\\', '/')
    segments = [part for part in text.split('/') if part and part != '.']
    alternatives = [[]]
    for part in segments:
        if SLOT not in part:
            options = [[part]]
        elif not part.replace(SLOT, ''):
            options = [['**']]
        else:
            head, tail = part[:part.index(SLOT)], part[part.rindex(SLOT) + 1:]
            options = [[head + '*' + tail], [head + '*', '**', '*' + tail]]
        alternatives = [done + option for done in alternatives for option in options]
        if len(alternatives) > 8:
            alternatives = [done + option for done in alternatives[:1] for option in [['**']]]
    roots = set()
    for parts in alternatives:
        collapsed = []
        for part in parts:
            part = part.replace('**', '*') if part != '**' else part
            if part == '**' and collapsed and collapsed[-1] == '**':
                continue
            collapsed.append(part)
        roots.add('/'.join(collapsed))
    return roots


def to_root(text):
    """One root covering every root `to_roots` gives (the shortest; tests use it for single roots)."""
    roots = to_roots(text)
    return min(roots, key=len) if roots else ''


def _literal_segment(root):
    """Whether a root keeps a wholly known path segment."""
    return any(part and not is_pattern(part) for part in root.split('/'))


def _join(base, text):
    if text.startswith(OUTSIDE) or text.startswith('/'):
        return OUTSIDE  # an outside or absolute path replaces the base
    if base.startswith(OUTSIDE) and not text.replace(SLOT, '').replace(CALLER, '').strip('/'):
        return base  # an outside location with no known text: no read
    return text if not base else base + '/' + text


def _parent(text, levels=1):
    for _ in range(levels):
        if text.startswith(OUTSIDE) and '/' not in text:
            return OUTSIDE
        if SLOT in text.rsplit('/', 1)[-1]:
            return text[:text.rindex(SLOT) + 1]
        text = text.rsplit('/', 1)[0] if '/' in text else ''
    return text


def _attr(func):
    return func.attr if isinstance(func, ast.Attribute) else func.id if isinstance(func, ast.Name) else None


class _Scope:
    def __init__(self, node, parent):
        self.node, self.parent, self.bindings = node, parent, {}

    def bind(self, name, value):
        self.bindings.setdefault(name, []).append(value)


class _Param:
    """A function parameter: resolved from parametrize values, fixtures, in-file call sites and its default."""

    def __init__(self, function, index, name, default, annotated_path):
        self.function, self.index, self.name, self.default, self.annotated_path = function, index, name, default, annotated_path


class _Unknown:
    """A binding whose value the code does not show (an import, an exception, a starred target)."""


UNKNOWN = _Unknown()


class _Supplied:
    """A parameter value a caller outside the file supplies: that caller's own expression is its read."""


SUPPLIED = _Supplied()


class _Unpack:
    """One target of a tuple unpacked from a call to a function in the file: that position of its returns."""

    def __init__(self, call, index):
        self.call, self.index = call, index


class _Listed:
    """A loop target over a directory scan (the scan call): its paths are resolved when used."""

    def __init__(self, call):
        self.call = call


class _Reader:
    """Resolves one Python file's path expressions, scans and imports."""

    def __init__(self, project, local):
        self.project, self.local = project, local
        self.tree = ast.parse((project / local).read_text(encoding='utf-8'), filename=local)
        self.scopes, self.parents, self.functions, self.loops, self.attributes = {}, {}, {}, [], {}
        self.memo_paths, self.memo_texts = {}, {}
        self.module = _Scope(self.tree, None)
        self._visit(self.tree, self.module)
        self.fixtures = {node.name: node for nodes in self.functions.values() for node in nodes
                         if any(_attr(d.func if isinstance(d, ast.Call) else d) == 'fixture' for d in node.decorator_list)}
        self.calls, self.referenced = {}, set()
        for node in ast.walk(self.tree):
            if isinstance(node, ast.Call) and _attr(node.func) in self.functions:
                self.calls.setdefault((_attr(node.func), isinstance(node.func, ast.Attribute)), []).append(node)
            elif isinstance(node, ast.Name) and node.id in self.functions and isinstance(node.ctx, ast.Load) \
                    and not (isinstance(self.parents.get(node), ast.Call) and self.parents[node].func is node):
                self.referenced.add(node.id)
            elif isinstance(node, ast.Attribute) and node.attr in self.functions \
                    and not (isinstance(self.parents.get(node), ast.Call) and self.parents[node].func is node):
                self.referenced.add(node.attr)
        for scope, target, iterable in self.loops:
            listed = _Listed(iterable) if self._is_scan(iterable) else None
            if listed is not None:
                names = target.elts if isinstance(target, (ast.Tuple, ast.List)) else [target]
                for index, name in enumerate(names):
                    self._bind(scope, name, listed if index == 0 else UNKNOWN)
                continue
            elements = self._elements(iterable, scope, set())
            for element in (elements if elements is not None else [UNKNOWN]):
                self._bind(scope, target, element)

    # Scopes and bindings.

    def _visit(self, node, scope):
        for child in ast.iter_child_nodes(node):
            self.parents[child] = node
            inner = scope
            if isinstance(child, FUNCTIONS + COMPREHENSIONS + (ast.ClassDef,)):
                inner = _Scope(child, scope)
                if isinstance(child, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
                    scope.bind(child.name, UNKNOWN)
                if isinstance(child, (ast.FunctionDef, ast.AsyncFunctionDef)):
                    self.functions.setdefault(child.name, []).append(child)
                if isinstance(child, FUNCTIONS):
                    self._bind_params(child, inner)
            self.scopes[child] = scope
            if isinstance(child, (ast.Assign, ast.AnnAssign)) and child.value is not None:
                for target in (child.targets if isinstance(child, ast.Assign) else [child.target]):
                    self._bind(scope, target, child.value)
                    if isinstance(target, ast.Attribute) and isinstance(target.value, ast.Name) and target.value.id in {'self', 'cls'}:
                        owner = self._class_of(scope)
                        if owner is not None:
                            self.attributes.setdefault((id(owner), target.attr), []).append(child.value)
            elif isinstance(child, ast.AugAssign):
                self._bind(scope, child.target, ast.BinOp(left=_load(child.target), op=child.op, right=child.value))
            elif isinstance(child, ast.NamedExpr):
                self._bind(scope, child.target, child.value)
            elif isinstance(child, (ast.For, ast.AsyncFor, ast.comprehension)):
                self.loops.append((scope, child.target, child.iter))
                self._declare(scope, child.target)
            elif isinstance(child, ast.withitem) and child.optional_vars is not None:
                self._bind(scope, child.optional_vars, child.context_expr)
            elif isinstance(child, (ast.Import, ast.ImportFrom)):
                for alias in child.names:
                    scope.bind((alias.asname or alias.name).split('.')[0], UNKNOWN)
            elif isinstance(child, ast.ExceptHandler) and child.name:
                scope.bind(child.name, UNKNOWN)
            self._visit(child, inner)

    def _container(self, node, seen=frozenset()):
        """The element or value expressions of a literal list, tuple, set or dict (directly, through names, or
        through a subscript of one), or None."""
        if isinstance(node, (ast.List, ast.Tuple, ast.Set)):
            return None if any(isinstance(e, ast.Starred) for e in node.elts) else list(node.elts)
        if isinstance(node, ast.Dict):
            return None if any(key is None for key in node.keys) else list(node.values)
        if isinstance(node, ast.Subscript):
            outer = self._container(node.value, seen)
            if outer is None:
                return None
            found = []
            for element in outer:
                inner = self._container(element, seen) if isinstance(element, ast.AST) else None
                if inner is None:
                    return None
                found.extend(inner)
            return found
        if isinstance(node, ast.Name) and node.id not in seen:
            values = self._lookup(node)
            if not values or any(not isinstance(v, ast.AST) for v in values):
                return None
            found = []
            for value in values:
                elements = self._container(value, seen | {node.id})
                if elements is None:
                    return None
                found.extend(elements)
            return found
        return None

    def _parsed_arguments(self, node):
        """Whether a name holds `parse_args()`'s result: the command line a caller supplies."""
        if not isinstance(node, ast.Name):
            return False
        values = self._lookup(node) or []
        return bool(values) and all(isinstance(v, ast.Call) and _attr(v.func) in {'parse_args', 'parse_known_args'}
                                    or isinstance(v, _Param) for v in values) and any(isinstance(v, ast.Call) for v in values) \
            or bool(values) and all(isinstance(v, _Param) for v in values) and node.id == 'args'

    @staticmethod
    def _scan_call(node):
        """The directory scan a loop iterates over (through sorted/list/reversed), or None."""
        while isinstance(node, ast.Call) and _attr(node.func) in {'sorted', 'list', 'reversed'} and len(node.args) == 1 \
                and not isinstance(node.func, ast.Attribute):
            node = node.args[0]
        if not (isinstance(node, ast.Call) and isinstance(node.func, ast.Attribute)):
            return None
        name, receiver = node.func.attr, node.func.value
        if name == 'walk' and _attr(receiver) == 'os' and node.args or name in SCAN_METHODS \
                or name in {'glob', 'iglob'} and isinstance(receiver, ast.Name) and receiver.id == 'glob' and node.args:
            return node
        return None

    def _is_scan(self, node):
        return self._scan_call(node) is not None

    def _listed(self, listed):
        """The paths a loop over a directory scan yields (a walk yields its directories first)."""
        node = self._scan_call(listed.call)
        name, receiver = node.func.attr, node.func.value
        if name == 'walk':
            return frozenset(_join(base, SLOT) for base in self._pathlike(node.args[0]))
        if name in SCAN_METHODS:
            bases = self.paths(receiver)
            if bases is None:
                bases = self.texts(receiver)
            patterns = [SLOT] if name == 'iterdir' else [('**/' if name == 'rglob' else '') + text
                                                        for text in (self._as_text(node.args[0]) if node.args else [SLOT])]
            return frozenset(_join(base, pattern.replace('*', SLOT)) for base in bases for pattern in patterns)
        return frozenset(text.replace('*', SLOT) for text in self._pathlike(node.args[0]))

    @staticmethod
    def _class_of(scope):
        while scope is not None and not isinstance(scope.node, ast.ClassDef):
            scope = scope.parent
        return None if scope is None else scope.node

    def _self_attribute(self, node):
        """The values assigned to `self.<attr>` in the enclosing class, if the attribute is on `self`."""
        if not (isinstance(node, ast.Attribute) and isinstance(node.value, ast.Name) and node.value.id in {'self', 'cls'}):
            return None
        owner = self._class_of(self.scopes.get(node, self.module))
        return None if owner is None else self.attributes.get((id(owner), node.attr))

    def _declare(self, scope, target):
        """A loop target is local to its scope even before its values are bound."""
        for node in ast.walk(target):
            if isinstance(node, ast.Name):
                scope.bindings.setdefault(node.id, [])

    def _bind_params(self, function, scope):
        args = function.args
        positional = args.posonlyargs + args.args
        defaults = [None] * (len(positional) - len(args.defaults)) + list(args.defaults)
        for index, (arg, default) in enumerate(zip(positional, defaults)):
            scope.bind(arg.arg, _Param(function, index, arg.arg, default, _path_annotation(arg)))
        for arg, default in zip(args.kwonlyargs, args.kw_defaults):
            scope.bind(arg.arg, _Param(function, None, arg.arg, default, _path_annotation(arg)))
        for arg in (args.vararg, args.kwarg):
            if arg is not None:
                scope.bind(arg.arg, SUPPLIED)

    def _bind(self, scope, target, value):
        if isinstance(target, ast.Name):
            scope.bind(target.id, value)
        elif isinstance(target, (ast.Tuple, ast.List)):
            if isinstance(value, (ast.Tuple, ast.List)) and len(target.elts) == len(value.elts) \
                    and not any(isinstance(e, ast.Starred) for e in target.elts + value.elts):
                for part, element in zip(target.elts, value.elts):
                    self._bind(scope, part, element)
            elif isinstance(value, ast.Call) and isinstance(value.func, ast.Name) \
                    and not any(isinstance(e, ast.Starred) for e in target.elts):
                for index, part in enumerate(target.elts):
                    self._bind(scope, part, _Unpack(value, index))
            else:
                for part in target.elts:
                    self._bind(scope, part.value if isinstance(part, ast.Starred) else part, UNKNOWN)
        elif isinstance(target, ast.Starred):
            self._bind(scope, target.value, UNKNOWN)

    def _elements(self, node, scope, seen):
        """The literal elements a loop over `node` yields, or None when unknown."""
        if isinstance(node, (ast.Tuple, ast.List, ast.Set)):
            return None if any(isinstance(e, ast.Starred) for e in node.elts) else list(node.elts)
        if isinstance(node, ast.Call) and _attr(node.func) in {'sorted', 'list', 'tuple', 'set', 'reversed'} \
                and len(node.args) == 1 and not isinstance(node.func, ast.Attribute):
            return self._elements(node.args[0], scope, seen)
        if isinstance(node, ast.Name) and node.id not in seen:
            values = self._lookup_in(self.scopes.get(node, scope), node.id)
            if not values or any(not isinstance(v, ast.AST) for v in values):
                return None
            found = []
            for value in values:
                elements = self._elements(value, scope, seen | {node.id})
                if elements is None:
                    return None
                found.extend(elements)
            return found
        return None

    def _lookup_in(self, scope, name):
        """A name's bindings in the innermost scope that binds it (a class body is not seen from its methods);
        None for a builtin or undefined name."""
        current, first = scope, True
        while current is not None:
            if name in current.bindings and (first or not isinstance(current.node, ast.ClassDef)):
                return current.bindings[name] or [UNKNOWN]
            current, first = current.parent, False
        return None

    def _lookup(self, node):
        values = self._lookup_in(self.scopes.get(node, self.module), node.id)
        if values:
            own = self._own_assignment(node)
            if own is not None:
                values = [value for value in values if value is not own] or values
        return values

    def _own_assignment(self, node):
        """For a name read in the value of an assignment to that same name outside any loop (`work =
        Path(work)`), that value: the read means the name's earlier bindings."""
        child, current = node, self.parents.get(node)
        while current is not None and not isinstance(current, FUNCTIONS + (ast.Module, ast.ClassDef)):
            if isinstance(current, (ast.For, ast.AsyncFor, ast.While) + COMPREHENSIONS):
                return None
            if isinstance(current, ast.Assign) and child is current.value \
                    and any(isinstance(t, ast.Name) and t.id == node.id for t in current.targets):
                return current.value
            child, current = current, self.parents.get(current)
        return None

    def _param_values(self, param):
        """The expressions a parameter may hold, with UNKNOWN for a source the code does not show."""
        function = param.function
        if isinstance(function, ast.Lambda):
            return [SUPPLIED]
        values = []
        for decorator in function.decorator_list:
            if isinstance(decorator, ast.Call) and _attr(decorator.func) == 'parametrize' and len(decorator.args) >= 2:
                values.extend(_parametrize_values(decorator.args[0], decorator.args[1], param.name))
        if values:
            return values
        if function.name.startswith('test') or function.name in self.fixtures:
            if param.name in TEMP_FIXTURES:
                return [OUTSIDE]
            if param.name in self.fixtures:
                return _returns(self.fixtures[param.name]) or [UNKNOWN]
            return [SUPPLIED]  # a fixture from conftest or a plugin
        method = isinstance(self.parents.get(function), ast.ClassDef)
        calls = self.calls.get((function.name, method), [])
        if function.name in self.referenced or not calls:
            values.append(SUPPLIED)
        index = None if param.index is None else param.index - (1 if method else 0)
        for call in calls:
            value = next((k.value for k in call.keywords if k.arg == param.name), None)
            if value is None and index is not None and index >= 0:
                if any(isinstance(a, ast.Starred) for a in call.args[:index + 1]):
                    value = UNKNOWN
                elif index < len(call.args):
                    value = call.args[index]
            if value is None and any(k.arg is None for k in call.keywords):
                value = UNKNOWN
            values.append(value if value is not None else (param.default if param.default is not None else UNKNOWN))
        return values

    # Values. A path is a set of project-relative texts (SLOT within a text for what the code does not show,
    # '' for the project root, OUTSIDE for an outside location), or None when nothing shows the expression is a
    # path. Text is a set of strings, with SLOT for unknown text.

    def texts(self, node):
        if not isinstance(node, ast.AST):
            return frozenset([SLOT])
        key = id(node)
        if key in self.memo_texts:
            return self.memo_texts[key] or frozenset([SLOT])
        self.memo_texts[key] = None
        result = self._texts(node)
        if len(result) > COMBINATIONS:
            result = frozenset([SLOT])
        self.memo_texts[key] = result
        return result

    def _texts(self, node):
        if isinstance(node, ast.Constant):
            return frozenset([node.value]) if isinstance(node.value, str) else frozenset([SLOT])
        if isinstance(node, ast.JoinedStr):
            found = {''}
            for part in node.values:
                if isinstance(part, ast.Constant):
                    values = {str(part.value)}
                elif part.conversion == -1 and part.format_spec is None:
                    values = set(self._as_text(part.value))
                else:
                    values = {SLOT}
                found = {a + b for a in found for b in values}
                if len(found) > COMBINATIONS:
                    return frozenset([''.join(str(p.value) if isinstance(p, ast.Constant) else SLOT for p in node.values)])
            return frozenset(found)
        if isinstance(node, ast.BinOp) and isinstance(node.op, ast.Add):
            return frozenset(a + b for a in self.texts(node.left) for b in self.texts(node.right))
        if isinstance(node, ast.BinOp) and isinstance(node.op, ast.Mod):
            return frozenset(_fill(text, '%') for text in self.texts(node.left))
        if isinstance(node, ast.Call):
            name, func = _attr(node.func), node.func
            if name == 'str' and len(node.args) == 1 and not isinstance(func, ast.Attribute):
                return self._as_text(node.args[0])
            if name == 'format' and isinstance(func, ast.Attribute):
                return frozenset(_fill(text, '{') for text in self.texts(func.value))
            if name == 'join' and isinstance(func, ast.Attribute) and isinstance(func.value, ast.Constant) \
                    and len(node.args) == 1 and isinstance(node.args[0], (ast.List, ast.Tuple)):
                found = {''}
                for index, element in enumerate(node.args[0].elts):
                    found = {a + (func.value.value if index else '') + b for a in found for b in self._as_text(element)}
                return frozenset(found)
            if self.paths(node) is not None:
                return self._as_text(node)
            return frozenset([SLOT])
        if isinstance(node, ast.Name):
            values = self._lookup(node)
            if not values:
                return frozenset([SLOT])
            found = set()
            for value in values:
                found |= self._value_texts(value)
            return frozenset(found)
        if isinstance(node, (ast.Attribute, ast.Subscript, ast.BinOp, ast.IfExp)) and self.paths(node) is not None:
            return self._as_text(node)
        if isinstance(node, ast.Subscript):
            elements = self._container(node.value)
            if elements is not None:
                found = set()
                for element in elements:
                    found |= self._value_texts(element)
                return frozenset(found)
            if isinstance(node.value, ast.Attribute) and node.value.attr == 'argv' and _attr(node.value.value) == 'sys':
                return frozenset([CALLER])
        if isinstance(node, ast.Attribute) and self._parsed_arguments(node.value):
            return frozenset([CALLER])
        if isinstance(node, ast.Attribute):
            if node.attr in {'name', 'stem'}:
                inner = self.paths(node.value)
                if inner is not None:
                    return frozenset(SLOT if text == OUTSIDE or SLOT in text.rsplit('/', 1)[-1] or CALLER in text.rsplit('/', 1)[-1]
                                     else text.rsplit('/', 1)[-1] for text in inner)
            values = self._self_attribute(node)
            if values:
                found = set()
                for value in values:
                    found |= self._value_texts(value)
                return frozenset(found)
        return frozenset([SLOT])

    def _value_texts(self, value):
        if isinstance(value, _Param):
            found = set()
            for source in self._param_values(value):
                found |= self._value_texts(source)
            return found
        if value is SUPPLIED:
            return {CALLER}
        if isinstance(value, _Listed):
            return set(text.replace(OUTSIDE, SLOT) for text in self._listed(value))
        if isinstance(value, _Unpack):
            found = set()
            for source in self._unpacked(value):
                found |= self._value_texts(source)
            return found
        if value is OUTSIDE:
            return {SLOT}
        if not isinstance(value, ast.AST):
            return {SLOT}
        return set(self._as_text(value))

    def _pathlike(self, node):
        """An operand of a path operation: its path (keeping an outside location) when shown, else its text."""
        paths = self.paths(node) if isinstance(node, ast.AST) else None
        return paths if paths is not None else self.texts(node)

    def _as_text(self, node):
        """An expression as text: its path's texts when it is a path, else its own text."""
        paths = self.paths(node) if isinstance(node, ast.AST) else None
        if paths is not None:
            return frozenset(text.replace(OUTSIDE, SLOT) for text in paths)
        return self.texts(node)

    def paths(self, node):
        key = id(node)
        if key in self.memo_paths:
            return self.memo_paths[key]
        self.memo_paths[key] = None  # a binding that refers to itself contributes nothing more
        result = self._paths(node)
        if result is not None and len(result) > COMBINATIONS:
            result = frozenset([SLOT])
        self.memo_paths[key] = result
        return result

    def _paths(self, node):
        if isinstance(node, ast.Name):
            if node.id == '__file__':
                return frozenset([self.local])
            values = self._lookup(node)
            return self._values_paths(values) if values else None
        if isinstance(node, ast.Call):
            return self._call_paths(node)
        if isinstance(node, ast.Attribute):
            if node.attr == 'parent':
                inner = self.paths(node.value)
                return None if inner is None else frozenset(_parent(text) for text in inner)
            if node.attr == 'executable' and _attr(node.value) == 'sys':
                return frozenset([OUTSIDE])
            if node.attr == 'name' and self.paths(node.value) == frozenset([OUTSIDE]):
                return frozenset([OUTSIDE])
            values = self._self_attribute(node)
            return self._values_paths(values) if values else None
        if isinstance(node, ast.Subscript):
            elements = self._container(node.value)
            if elements is not None:
                return self._values_paths(elements)
            if isinstance(node.value, ast.Attribute) and node.value.attr == 'argv' and _attr(node.value.value) == 'sys':
                return None
            if isinstance(node.value, ast.Attribute) and node.value.attr == 'parents':
                inner = self.paths(node.value.value)
                if inner is None:
                    return None
                levels = node.slice.value + 1 if isinstance(node.slice, ast.Constant) and isinstance(node.slice.value, int) else None
                return frozenset((_parent(text, levels) if levels else SLOT) for text in inner)
            if isinstance(node.value, ast.Attribute) and node.value.attr == 'environ':
                return frozenset([OUTSIDE])
            return None
        if isinstance(node, ast.BinOp) and isinstance(node.op, ast.Div):
            left = self.paths(node.left)
            if left is None and not self._texty(node.right):
                return None  # two values of unknown kind: a number divided, as far as the code shows
            left = left if left is not None else self.texts(node.left)
            return frozenset(_join(base, text) for base in left for text in self._pathlike(node.right))
        if isinstance(node, ast.IfExp):
            branches = [self.paths(node.body), self.paths(node.orelse)]
            if all(branch is None for branch in branches):
                return None
            found = set()
            for branch, value in zip(branches, (node.body, node.orelse)):
                if branch is not None:
                    found |= branch
                elif not (isinstance(value, ast.Constant) and value.value is None):
                    found |= self.texts(value)
            return frozenset(found)
        return None

    def _values_paths(self, values):
        values = [value for value in values if not (isinstance(value, ast.Constant) and value.value is None)]  # no path
        results = [self._value_paths(value) for value in values]
        if all(result is None for result in results):
            return None
        found = set()
        for value, result in zip(values, results):
            found |= result if result is not None else self._unshown(value)
        return frozenset(found)

    def _unshown(self, value):
        """The text a binding not shown to be a path stands for, beside paths: its own text."""
        if isinstance(value, _Param):
            found = set()
            for source in self._param_values(value):
                found |= self._unshown(source) if self._value_paths(source) is None else set()
            return found or {SLOT}
        return self._value_texts(value)

    def _value_paths(self, value):
        if isinstance(value, _Param):
            result = self._values_paths(self._param_values(value))
            return frozenset(self._unshown(value)) if result is None and value.annotated_path else result
        if value is OUTSIDE:
            return frozenset([OUTSIDE])
        if isinstance(value, _Listed):
            return self._listed(value)
        if isinstance(value, _Unpack):
            return self._values_paths(self._unpacked(value))
        if not isinstance(value, ast.AST):
            return None
        return self.paths(value)

    def _unpacked(self, unpack):
        """The expressions at one position of the tuples a file-local function returns (UNKNOWN otherwise)."""
        name = unpack.call.func.id
        if name not in self.functions:
            return [UNKNOWN]
        found = []
        for function in self.functions[name]:
            for value in _returns(function):
                if isinstance(value, (ast.Tuple, ast.List)) and unpack.index < len(value.elts) \
                        and not any(isinstance(e, ast.Starred) for e in value.elts):
                    found.append(value.elts[unpack.index])
                else:
                    found.append(UNKNOWN)
        return found or [UNKNOWN]

    def _call_paths(self, node):
        func, name = node.func, _attr(node.func)
        receiver = func.value if isinstance(func, ast.Attribute) else None
        if name in PATH_TYPES:
            found = {''}
            for arg in node.args:
                found = {_join(base, text) if base else text for base in found for text in self._pathlike(arg)}
            return frozenset(found)
        if name in OUTSIDE_CALLS and (receiver is None or _attr(receiver) in {'tempfile', 'Path', 'shutil', 'os', 'path'}
                                      or isinstance(receiver, ast.Name) and receiver.id in TEMP_FIXTURES):
            return frozenset([OUTSIDE])
        if name == 'cwd' and receiver is not None:
            return frozenset([''])
        if name == 'get' and (isinstance(receiver, ast.Attribute) and receiver.attr == 'environ'
                              or isinstance(receiver, ast.Name) and receiver.id in {'environ', 'env', 'environment'}):
            return frozenset([OUTSIDE])
        if name == 'fspath' and node.args:
            return self.paths(node.args[0])
        if receiver is not None and _attr(receiver) == 'path' and name in {'join', 'dirname', 'abspath', 'normpath', 'realpath'}:
            if not node.args:
                return None
            if name == 'join':
                found = None
                for arg in node.args:
                    texts = self._pathlike(arg)
                    found = set(texts) if found is None else {_join(base, text) for base in found for text in texts}
                return frozenset(found)
            inner = self._pathlike(node.args[0])
            return frozenset(_parent(text) for text in inner) if name == 'dirname' else frozenset(inner)
        if receiver is not None and _attr(receiver) == 'shutil' and name in {'copytree', 'copy', 'copy2', 'copyfile', 'move'} \
                and len(node.args) >= 2:
            return self.paths(node.args[1]) or frozenset([SLOT])
        if receiver is not None:
            inner = self.paths(receiver)
            if inner is None:
                return None
            if name in SAME_PATH_CALLS:
                return inner
            if name == 'joinpath':
                found = set(inner)
                for arg in node.args:
                    found = {_join(base, text) for base in found for text in self._pathlike(arg)}
                return frozenset(found)
            if name in {'with_name', 'with_suffix', 'with_stem'}:
                if name == 'with_name' and node.args:
                    return frozenset(_join(_parent(base), text) for base in inner for text in self._as_text(node.args[0]))
                return frozenset(base if base == OUTSIDE else base + SLOT for base in inner)
            return None
        if name in self.functions and not isinstance(func, ast.Attribute):
            # A file-local helper: the paths it returns.
            results = [self.paths(value) for function in self.functions[name] for value in _returns(function)]
            if not results or all(result is None for result in results):
                return None
            found = set()
            for result in results:
                found |= result if result is not None else {SLOT}
            return frozenset(found)
        return None

    def _texty(self, node, seen=frozenset()):
        """Whether an expression is shown to be text (so `x / it` is a path even when `x` is unknown)."""
        if isinstance(node, ast.Constant) and isinstance(node.value, str) or isinstance(node, ast.JoinedStr):
            return True
        if isinstance(node, ast.BinOp) and isinstance(node.op, (ast.Add, ast.Mod)):
            return self._texty(node.left, seen)
        if isinstance(node, ast.Call) and _attr(node.func) in {'str', 'format', 'join'}:
            return True
        if isinstance(node, ast.Name) and node.id not in seen:
            seen = seen | {node.id}
            for value in self._lookup(node) or []:
                sources = self._param_values(value) if isinstance(value, _Param) else [value]
                if any(isinstance(source, ast.AST) and self._texty(source, seen) for source in sources):
                    return True
        return False

    # Roots.

    def chain_paths(self):
        """Every longest expression shown to be a path."""
        found = set()
        kinds = (ast.BinOp, ast.Call, ast.Attribute, ast.Subscript, ast.IfExp)
        for node in ast.walk(self.tree):
            if not isinstance(node, kinds):
                continue
            paths = self.paths(node)
            if paths is None:
                continue
            parent = self.parents.get(node)
            if isinstance(parent, kinds) and self.paths(parent) is not None:
                continue
            found |= {text for text in paths if text != self.local}
        return found

    def scans(self):
        """Directory scans and git pathspecs as path texts, and whether the code does not show one of them."""
        found, unknown = set(), False

        def add(text, shown=False):
            nonlocal unknown
            if text.startswith(OUTSIDE) or text.startswith(CALLER):
                return  # a scan of an outside or caller-supplied directory: the caller's argument is the read
            # Not shown: no base and no known segment, so its root would be the whole project.
            unknown |= not shown and text.startswith(SLOT) and not any(_literal_segment(root) for root in to_roots(text))
            found.add(text)

        for node in ast.walk(self.tree):
            if not isinstance(node, ast.Call) or not isinstance(node.func, ast.Attribute):
                continue
            name, receiver = node.func.attr, node.func.value
            if name in {'glob', 'iglob'} and isinstance(receiver, ast.Name) and receiver.id == 'glob':
                for text in self._as_text(node.args[0]) if node.args else [SLOT]:
                    add(text)
            elif name in OS_SCANS and _attr(receiver) == 'os':
                bases = self.paths(node.args[0]) or self._as_text(node.args[0]) if node.args else frozenset([''])
                for base in bases:
                    if not base.startswith(OUTSIDE):
                        add(_join(base, '**' if name == 'walk' else '*'))
            elif name in SCAN_METHODS:
                bases = self.paths(receiver)
                if bases is None:
                    bases = frozenset([SLOT])
                patterns = ['*'] if name == 'iterdir' else [('**/' if name == 'rglob' else '') + text
                                                             for text in (self._as_text(node.args[0]) if node.args else [SLOT])]
                for base in bases:
                    if base.startswith(OUTSIDE):
                        continue
                    for pattern in patterns:
                        add(_join(base, pattern))
            elif name in SUBPROCESS_CALLS and _attr(receiver) == 'subprocess' and node.args:
                command = self._command(node.args[0])
                if command is None or not command or command[0] != {'git'}:
                    continue
                index = 1
                while index < len(command) and command[index] and all(t.startswith('-') for t in command[index]):
                    index += 2 if command[index] & GIT_VALUED_OPTIONS else 1
                if index >= len(command) or not command[index] & GIT_PATHSPEC_COMMANDS:
                    continue
                specs = _pathspecs(command[index + 1:])
                if specs is None:
                    add(SLOT)
                    continue
                for spec in specs or ['*']:
                    # A git pathspec's `*` also matches `/`; a spec the code shows is shown even when it spans the project.
                    add(spec.replace('*', SLOT), shown=SLOT not in spec)
        return found, unknown

    def _command(self, node):
        """A subprocess argument list as a list of text sets (None when its length is unknown)."""
        if isinstance(node, ast.Name):
            values = self._lookup(node) or []
            if len(values) != 1 or not isinstance(values[0], (ast.List, ast.Tuple)):
                return None
            node = values[0]
        if not isinstance(node, (ast.List, ast.Tuple)) or any(isinstance(e, ast.Starred) for e in node.elts):
            return None
        return [set(self._as_text(element)) for element in node.elts]

    def literal_paths(self):
        """Path-like words of text that no path expression holds (`chain_paths` reads those)."""
        found = set()
        for node in ast.walk(self.tree):
            parent = self.parents.get(node)
            if isinstance(parent, ast.JoinedStr) or isinstance(parent, ast.BinOp) and isinstance(parent.op, (ast.Div, ast.Add, ast.Mod)):
                continue
            if isinstance(node, ast.Constant) and isinstance(node.value, str):
                texts = [_fill(node.value, '{')]
            elif isinstance(node, ast.JoinedStr):
                texts = sorted(self.texts(node))
            else:
                continue
            for text in texts:
                for word in text.replace(',', ' ').split():
                    word = word.strip('\'"`()[]<>:;')
                    if '://' in word:
                        continue
                    for stop in '[$#':
                        if stop in word:
                            word = word[:word.index(stop)] + SLOT
                    if '/' in word.replace(SLOT, '') and not word.startswith(('/', '.')):
                        found.add(word)
        return found

    def declared(self):
        """The module-level `CI_READS` entries, or None when absent; a non-literal entry is kept as None."""
        for value in self.module.bindings.get(DECLARED_READS, []):
            if isinstance(value, (ast.Tuple, ast.List)):
                return tuple(element.value if isinstance(element, ast.Constant) and isinstance(element.value, str) else None
                             for element in value.elts)
            return (None,)
        return None

    def search_dirs(self):
        """The project directories this file's imports may resolve from."""
        dirs = ['', _parent(self.local)]
        for node in ast.walk(self.tree):
            if isinstance(node, ast.Call) and _attr(node.func) in {'insert', 'append', 'syspath_prepend'} and node.args:
                for text in self.paths(node.args[-1]) or ():
                    if text != OUTSIDE and SLOT not in text:
                        dirs.append(text.strip('/'))
        return dirs

    def imports(self):
        names = set()
        package = _parent(self.local).replace('/', '.')
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
            elif isinstance(node, ast.Call) and _attr(node.func) in {'import_module', '__import__'} and node.args:
                names.update(text for text in self.texts(node.args[0]) if SLOT not in text)
        return names


def _load(target):
    if isinstance(target, ast.Name):
        return ast.Name(id=target.id, ctx=ast.Load())
    return target


def _path_annotation(arg):
    return arg.annotation is not None and 'Path' in ast.unparse(arg.annotation)


def _parametrize_values(names, values, name):
    if isinstance(names, ast.Constant) and isinstance(names.value, str):
        names = [n.strip() for n in names.value.split(',') if n.strip()]
    elif isinstance(names, (ast.Tuple, ast.List)) and all(isinstance(n, ast.Constant) for n in names.elts):
        names = [n.value for n in names.elts]
    else:
        return [UNKNOWN]
    if name not in names:
        return []
    position = names.index(name)
    if not isinstance(values, (ast.Tuple, ast.List)):
        return [UNKNOWN]
    found = []
    for value in values.elts:
        if isinstance(value, ast.Call) and _attr(value.func) == 'param':
            if len(names) == 1:
                found.append(value.args[0] if value.args else UNKNOWN)
                continue
            value = ast.Tuple(elts=list(value.args), ctx=ast.Load())
        if len(names) == 1:
            found.append(value)
        elif isinstance(value, (ast.Tuple, ast.List)) and len(value.elts) == len(names):
            found.append(value.elts[position])
        else:
            found.append(UNKNOWN)
    return found


def _returns(function):
    if isinstance(function, ast.Lambda):
        return [function.body]
    return [node.value for node in ast.walk(function) if isinstance(node, (ast.Return, ast.Yield)) and node.value is not None]


def _fill(text, kind):
    """A `%` or `{}` template with each slot as unknown text."""
    out, index = [], 0
    while index < len(text):
        char = text[index]
        if kind == '{' and char in '{}':
            if text[index:index + 2] in ('{{', '}}'):
                out.append(char)
                index += 2
                continue
            if char == '{':
                end = text.find('}', index)
                out.append(SLOT)
                index = len(text) if end < 0 else end + 1
                continue
        if kind == '%' and char == '%' and index + 1 < len(text):
            out.append('%' if text[index + 1] == '%' else SLOT)
            index += 2
            continue
        out.append(char)
        index += 1
    return ''.join(out)


def _pathspecs(arguments):
    """git pathspecs after the subcommand: the arguments after `--`, else every non-option argument (a
    `git grep` pattern among them reads nothing more). None when the arguments are not shown."""
    if any(not texts for texts in arguments):
        return None
    marks = [index for index, texts in enumerate(arguments) if texts == {'--'}]
    after = arguments[marks[0] + 1:] if marks else [texts for texts in arguments if not any(t.startswith('-') for t in texts)]
    return [text for texts in after for text in texts]


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
    """One Python file's roots, the non-test Python files among them (loaded or run by path) and the
    project Python files it imports."""
    paths = tracked(project)
    reader = _reader(project, local)
    named = {root for text in reader.chain_paths() for root in to_roots(text)}
    named |= {root for text in reader.literal_paths() for root in to_roots(text) if _literal_segment(root)}
    declared = reader.declared()
    # A declaration stands in for the file's scans (`declaration_problems` keeps it covering them).
    named |= ({entry for entry in declared if entry} if declared is not None
              else {root for text in reader.scans()[0] for root in to_roots(text)})
    roots = frozenset(root for root in named if root and not _unavailable(root))
    loaded = frozenset(root for root in roots if root.endswith('.py') and root != local and not is_test(root)
                       and not is_pattern(root) and root in paths and (project / root).is_file())
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


def _prefix(pattern):
    """A pattern's literal leading directories."""
    prefix = []
    for part in pattern.split('/'):
        if is_pattern(part):
            break
        prefix.append(part)
    return '/'.join(prefix)


def _usable(paths, entry):
    """A declared entry: a tracked path, or a pattern whose literal prefix is the root or a tracked directory."""
    if not entry or _unavailable(entry):
        return False
    if is_pattern(entry):
        prefix = _prefix(entry)
        return (not prefix or prefix in paths) and entry != prefix
    return entry in paths


def _unavailable(local):
    """Under execution/ outside execution/_DAG/, which the Python job does not check out."""
    return local == 'execution' or local.startswith('execution/') and not local.startswith('execution/_DAG')


def declared_reads(project):
    """{test: its `CI_READS` entries} for every maintained test that declares reads."""
    found = {}
    for test in maintained_tests(project):
        entries = _reader(project, test).declared()
        if entries is not None:
            found[test] = entries
    return found


def _closure(project, test):
    """A test and the non-test files it loads or runs by path, transitively."""
    pending, seen = [test], []
    while pending:
        local = pending.pop()
        if local not in seen:
            seen.append(local)
            pending.extend(sorted(file_reads(project, local)[1]))
    return seen


def declaration_problems(project):
    """Why a test's reads are not shown: a scan or git pathspec the code does not show, in the test or a file it
    loads, with no `CI_READS` in that file; or a stale declaration: a non-literal or unusable entry (no tracked
    path or prefix the Python job has), or a shown scan in that file no declared entry covers."""
    paths, problems, seen = tracked(project), [], set()
    for test in maintained_tests(project):
        for local in _closure(project, test):
            if local in seen:
                continue
            seen.add(local)
            reader = _reader(project, local)
            scans, unknown = reader.scans()
            entries = reader.declared()
            if entries is None:
                if unknown:
                    problems.append((local, 'a scan or git pathspec the code does not show, and no CI_READS'))
                continue
            for entry in entries:
                if not _usable(paths, entry):
                    problems.append((local, f'unusable CI_READS entry {entry!r}'))
            covers = [_prefix(entry) if is_pattern(entry) else entry for entry in entries if entry]
            for scan in sorted(scans):
                if scan.startswith(SLOT):
                    continue  # not shown: the declaration stands in for it
                base = _prefix(to_root(scan))
                if not any(not cover or base == cover or base.startswith(cover + '/') for cover in covers):
                    problems.append((local, f'scan {to_root(scan)!r} not covered by CI_READS'))
    return problems


@functools.lru_cache(maxsize=None)
def test_reads(project):
    """{test: sorted project roots it reads} for every maintained test."""
    reads = {}
    for test in maintained_tests(project):
        roots, imported = set(), set()
        for local in _closure(project, test):
            named, _, imports = file_reads(project, local)
            roots |= named
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
