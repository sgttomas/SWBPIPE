#!/usr/bin/env python3
"""The diagnostic codes a solve of a model document can raise, read from the Rust sources.

Exit criterion 3 of `docs/RELEASE_TARGET_v1.0.md` needs every remaining refusal of a required
capability listed in `docs/RELEASE_LIMITATIONS_v1.0.md`. The codes are string literals at their
call sites, with no registry, so this scanner makes one: it reads the production sources of the
crates a solve runs through (`SCOPE`) and records each code a production call site can raise, with
the files and lines that raise it and the severity the site states. `fixtures/diagnostics/
exact_route_codes.json` holds the result, with a hand-made classification of each code that
`--write` keeps; `tests/test_diagnostic_code_table.py` keeps the two equal.

What counts as production code: the modules reachable from each crate's `src/lib.rs` (or
`src/main.rs`) through `mod` declarations, following `#[path]`, with every item under
`#[cfg(test)]` removed and every file or directory whose name marks it a test (`tests`, `*_tests`,
`tests_*`) left out. `src/bin/` harness binaries are not crate roots here and are not read.

How a code is found. A code is the value that reaches a *code position*:
- an argument, at a parameter named `code` of string type (`&str`, `String`, `impl Into<String>`),
  of any function in scope (`diag`, `problem`, `block`, `refusal`, `Finding::new`, ...), and,
  to a fixed point, an argument at a parameter any such argument is passed on from unchanged;
- the value of a struct field `code: ...`, the shorthand `code` field, or an assignment `.code = ...`;
- the prefix `CODE: ` of a message literal (`"JOINT_PRESSURE_INTERFACE_UNRESOLVED: ..."`, or
  `"{CONST}: ..."`), the form a refusal carried as an error string takes.
The value is followed through `&`, `.into()`, `.to_string()` and the like, `const NAME: &str =
"CODE"` declarations (a code reached only through a constant is the constant's value), local `let`
bindings (tuple bindings by position), `match` and `if` arms, and calls of functions that return a
code (their return expressions, resolved the same way). A value relayed from another record's
`.code` field is not a new code: the record's own construction is the site.

What it cannot see is listed, never guessed (`unresolved_sites`): a code built at run time
(`format!`), a value bound by a closure, `for`, `if let` or `match` pattern, a field other than
`code`, a call whose function does not return a resolvable code, or any other expression. Every
code-shaped literal (`UPPER_SNAKE` with an underscore) in production code that the scanner does
not find raised, and that is not only compared against, is listed too (`unclaimed_literals`), so
that a code reaching a diagnostic by a route the scanner does not know shows up there.

Usage, from the repository root:
  python3 tools/validation/diagnostic_codes.py            # summary to stdout; writes nothing
  python3 tools/validation/diagnostic_codes.py --json     # the mechanical table to stdout
  python3 tools/validation/diagnostic_codes.py --write    # refresh the committed table, keeping the
                                                          # hand-made fields of each code
Standard library only.
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

PROJECT = Path(__file__).resolve().parents[2]
TABLE = 'fixtures/diagnostics/exact_route_codes.json'

# Crate source roots a solve of a model document runs through on either route, and the analyses
# beside it. `operation_applier` is read only for codes it raises by re-using a solve refusal:
# a code position whose value is a constant declared in another scanned crate, or a code relayed
# from a solve validator's finding (which needs no record of its own).
SCOPE = ('core/product_physics', 'core/solver/*', 'core/loads/*', 'core/analysis/hanger_sizing', 'core/numerics/*')
REUSE_SCOPE = ('core/model_operations/operation_applier',)
# The paths the scan reads, for the Python CI selector (`tools/ci/python_test_reads.py`).
CI_READS = ('core/product_physics/src/**', 'core/solver/*/src/**', 'core/loads/*/src/**',
            'core/analysis/hanger_sizing/src/**', 'core/numerics/*/src/**',
            'core/model_operations/operation_applier/src/**')

# The fields a person fills in for each code; `--write` keeps them.
HAND_FIELDS = ('classification', 'reason', 'unsure', 'candidates', 'named_by', 'named_by_statement')
CLASSES = ('absent_capability', 'input_error', 'numerical_standing', 'disclosure', 'internal')
SEVERITIES = {'blocking', 'warning', 'info', 'failure'}

CODE_SHAPE = re.compile(r'^[A-Z][A-Z0-9]*(?:[_-][A-Z0-9]+)+$')
CONST_NAME = re.compile(r'^[A-Z][A-Z0-9_]*$')
PREFIX_LITERAL = re.compile(r'^([A-Z][A-Z0-9]*(?:[_-][A-Z0-9]+)+): ')
PREFIX_FORMAT = re.compile(r'^\{([A-Z][A-Z0-9_]*)\}: ')
TEST_NAME = re.compile(r'(^tests?$|_tests?$|^tests_)')
STRINGISH = re.compile(r"^(&\s*('[a-z_]+\s*)?(mut\s*)?str|String|impl\s*Into\s*<\s*String\s*>|impl\s*AsRef\s*<\s*str\s*>|Cow\s*<.*str\s*>)$")
PRIMITIVES = {'u8', 'u16', 'u32', 'u64', 'u128', 'usize', 'i8', 'i16', 'i32', 'i64', 'i128', 'isize',
              'f32', 'f64', 'bool', 'char', 'str', 'String'}
WRAPPERS = {'String::from', 'Some', 'Cow::Borrowed', 'Cow::Owned', 'Ok', 'Err', 'Box::from'}
SAME_VALUE_METHODS = {'to_string', 'into', 'to_owned', 'clone', 'as_ref', 'to_ascii_uppercase'}
MESSAGE_PARAMS = ('message', 'msg', 'detail', 'text', 'reason', 'remedy')
MAX_DEPTH = 8


# ---------------------------------------------------------------- tokens

class Tok:
    __slots__ = ('kind', 'val', 'line')

    def __init__(self, kind, val, line):
        self.kind, self.val, self.line = kind, val, line

    def __repr__(self):
        return f'{self.kind}:{self.val}@{self.line}'


PUNCT2 = ('->', '=>', '==', '!=', '::', '<=', '>=', '&&', '||', '..', '+=', '-=')


def _unescape(raw):
    """The text of a non-raw string literal's body, close enough to read a code or a message."""
    out, i = [], 0
    while i < len(raw):
        c = raw[i]
        if c == '\\' and i + 1 < len(raw):
            n = raw[i + 1]
            if n == '\n':                      # line continuation: skip the newline and leading space
                i += 2
                while i < len(raw) and raw[i] in ' \t\n':
                    i += 1
                continue
            out.append({'n': '\n', 't': '\t', '0': '\0', 'r': '\r'}.get(n, n))
            i += 2
            if n == 'u':                       # \u{...}
                j = raw.find('}', i)
                i = j + 1 if j >= 0 else i
            elif n == 'x':
                i += 2
            continue
        out.append(c)
        i += 1
    return ''.join(out)


def tokenize(src):
    toks, i, line, n = [], 0, 1, len(src)
    while i < n:
        c = src[i]
        if c == '\n':
            line += 1
            i += 1
        elif c in ' \t\r':
            i += 1
        elif src.startswith('//', i):
            j = src.find('\n', i)
            i = n if j < 0 else j
        elif src.startswith('/*', i):
            depth, j = 1, i + 2
            while j < n and depth:
                if src.startswith('/*', j):
                    depth, j = depth + 1, j + 2
                elif src.startswith('*/', j):
                    depth, j = depth - 1, j + 2
                else:
                    j += 1
            line += src.count('\n', i, j)
            i = j
        elif c == '"' or (c in 'bc' and src.startswith('"', i + 1)):
            start = i + (0 if c == '"' else 1)
            j = start + 1
            while j < n and src[j] != '"':
                j += 2 if src[j] == '\\' else 1
            toks.append(Tok('str', _unescape(src[start + 1:j]), line))
            line += src.count('\n', i, j)
            i = j + 1
        elif (c == 'r' or (c == 'b' and src.startswith('r', i + 1))) and re.match(r'b?r#*"', src[i:i + 40]):
            m = re.match(r'b?r(#*)"', src[i:])
            close = '"' + m.group(1)
            j = src.find(close, i + m.end())
            toks.append(Tok('str', src[i + m.end():j], line))
            line += src.count('\n', i, j)
            i = j + len(close)
        elif c == "'":
            m = re.match(r"'(\\(x..|u\{[^}]*\}|.)|[^\\'])'", src[i:])
            if m:
                toks.append(Tok('char', m.group(0), line))
                i += m.end()
            else:
                m = re.match(r"'[A-Za-z_][A-Za-z0-9_]*", src[i:])
                toks.append(Tok('life', m.group(0) if m else "'", line))
                i += m.end() if m else 1
        elif c.isalpha() or c == '_':
            m = re.match(r'[A-Za-z_][A-Za-z0-9_]*', src[i:])
            toks.append(Tok('id', m.group(0), line))
            i += m.end()
        elif c.isdigit():
            m = re.match(r'[0-9][0-9A-Za-z_]*(\.[0-9][0-9A-Za-z_]*)?', src[i:])
            toks.append(Tok('num', m.group(0), line))
            i += m.end()
        else:
            two = src[i:i + 2]
            if two in PUNCT2:
                toks.append(Tok('p', two, line))
                i += 2
            else:
                toks.append(Tok('p', c, line))
                i += 1
    return toks


OPEN, CLOSE = {'(': ')', '[': ']', '{': '}'}, {')', ']', '}'}


def brackets(toks):
    """{index of an opening bracket: index of its closing bracket}."""
    pairs, stack = {}, []
    for k, t in enumerate(toks):
        if t.kind == 'p' and t.val in OPEN:
            stack.append(k)
        elif t.kind == 'p' and t.val in CLOSE and stack:
            pairs[stack.pop()] = k
    return pairs


def _is(t, kind, val=None):
    return t is not None and t.kind == kind and (val is None or t.val == val)


def strip_cfg_test(toks):
    """The tokens with every item under `#[cfg(test)]` (or `cfg(all(test, ...))`) removed, and the
    `mod` names those items declared (so that their files are not read)."""
    out, k, n = [], 0, len(toks)
    if n >= 3 and _is(toks[0], 'p', '#') and _is(toks[1], 'p', '!') and _is(toks[2], 'p', '['):
        body = [x.val for x in toks[3:_match_forward(toks, 2)]]
        if body[:1] == ['cfg'] and 'test' in body and 'not' not in body:
            return []                    # `#![cfg(test)]`: the whole file is a test
    while k < n:
        t = toks[k]
        if _is(t, 'p', '#') and k + 1 < n and _is(toks[k + 1], 'p', '['):
            end = _match_forward(toks, k + 1)
            body = [x.val for x in toks[k + 2:end]]
            if body[:1] == ['cfg'] and 'test' in body and 'not' not in body:
                k = _skip_item(toks, end + 1)
                continue
        out.append(t)
        k += 1
    return out


def _match_forward(toks, k):
    depth = 0
    for j in range(k, len(toks)):
        t = toks[j]
        if t.kind == 'p' and t.val in OPEN:
            depth += 1
        elif t.kind == 'p' and t.val in CLOSE:
            depth -= 1
            if depth == 0:
                return j
    return len(toks) - 1


ITEM_KEYWORDS = {'fn', 'mod', 'impl', 'struct', 'enum', 'trait', 'union', 'macro_rules', 'extern',
                 'unsafe', 'async', 'const', 'static', 'type', 'use', 'pub'}


def _skip_item(toks, k):
    """The index after the item or statement that starts at `k` (after its attributes)."""
    n = len(toks)
    while k < n and _is(toks[k], 'p', '#') and k + 1 < n and _is(toks[k + 1], 'p', '['):
        k = _match_forward(toks, k + 1) + 1
    if k >= n:
        return n
    first = toks[k]
    if first.kind == 'id' and first.val in ITEM_KEYWORDS and first.val not in ('const', 'static', 'type', 'use'):
        depth = 0
        for j in range(k, n):
            t = toks[j]
            if t.kind != 'p':
                continue
            if t.val in '([':
                depth += 1
            elif t.val in ')]':
                depth -= 1
            elif depth == 0 and t.val == ';':
                return j + 1
            elif depth == 0 and t.val == '{':
                return _match_forward(toks, j) + 1
        return n
    if _is(first, 'p', '{'):
        return _match_forward(toks, k) + 1
    depth = 0
    for j in range(k, n):
        t = toks[j]
        if t.kind != 'p':
            continue
        if t.val in OPEN:
            depth += 1
        elif t.val in CLOSE:
            if depth == 0:
                return j
            depth -= 1
        elif depth == 0 and t.val in (';', ','):
            return j + 1
    return n


# ---------------------------------------------------------------- module tree

class Source:
    def __init__(self, crate, rel, toks):
        self.crate, self.rel, self.toks = crate, rel, toks
        self.pairs = brackets(toks)
        self.fns = []
        # the innermost `{` and `(` that contain each token, or -1
        self.brace_at, self.paren_at = [-1] * len(toks), [-1] * len(toks)
        braces, parens = [], []
        for k, t in enumerate(toks):
            self.brace_at[k] = braces[-1] if braces else -1
            self.paren_at[k] = parens[-1] if parens else -1
            if t.kind == 'p':
                if t.val == '{':
                    braces.append(k)
                elif t.val == '(':
                    parens.append(k)
                elif t.val == '}' and braces:
                    braces.pop()
                elif t.val == ')' and parens:
                    parens.pop()
        self.fn_at = None

    def index_functions(self):
        """The innermost function or closure whose body contains each token."""
        self.fn_at = [None] * len(self.toks)
        for f in sorted((f for f in self.fns if f.body), key=lambda f: f.body[0]):
            a, b = f.body
            for k in range(a + 1, min(b, len(self.toks))):
                self.fn_at[k] = f


def _module_files(path, project, seen):
    """The production source files of the module rooted at `path`, following `mod` declarations."""
    if path in seen or not path.is_file():
        return []
    seen.add(path)
    rel = path.relative_to(project).as_posix()
    toks = strip_cfg_test(tokenize(path.read_text(encoding='utf-8')))
    found = [(rel, toks)]
    mod_rs = path.name in ('lib.rs', 'main.rs', 'mod.rs')
    base = path.parent if mod_rs else path.parent / path.stem
    pairs = brackets(toks)
    inline = []            # (end index, directory) of the inline `mod name { ... }` blocks open at k
    k = 0
    while k < len(toks):
        while inline and k > inline[-1][0]:
            inline.pop()
        t = toks[k]
        if _is(t, 'id', 'mod') and k + 1 < len(toks) and toks[k + 1].kind == 'id':
            name = toks[k + 1].val
            nxt = toks[k + 2] if k + 2 < len(toks) else None
            here = inline[-1][1] if inline else base
            path_attr = _path_attribute(toks, k)
            if _is(nxt, 'p', ';'):
                if TEST_NAME.search(name):
                    k += 3
                    continue
                if path_attr is not None:
                    target = ((inline[-1][1] if inline else path.parent) / path_attr)
                    candidates = [target]
                else:
                    candidates = [here / f'{name}.rs', here / name / 'mod.rs']
                for candidate in candidates:
                    candidate = candidate.resolve()
                    if not candidate.is_file() or not candidate.is_relative_to(project):
                        continue
                    if not any(TEST_NAME.search(part) for part in candidate.relative_to(project).with_suffix('').parts):
                        found += _module_files(candidate, project, seen)
                        break
                k += 3
                continue
            if _is(nxt, 'p', '{'):
                inline.append((pairs.get(k + 2, len(toks)), here / name))
        k += 1
    return found


def _path_attribute(toks, k):
    """The `#[path = "..."]` value among the attributes before the item at `k`, or None."""
    j = k - 1
    while j >= 0 and toks[j].kind == 'id' and toks[j].val in ('pub', 'crate', 'super', 'self', 'in'):
        j -= 1
    while j >= 0 and _is(toks[j], 'p', ')') and j >= 1:          # pub(crate)
        j -= 1
        while j >= 0 and not _is(toks[j], 'p', '('):
            j -= 1
        j -= 1
        while j >= 0 and toks[j].kind == 'id' and toks[j].val == 'pub':
            j -= 1
    while j >= 0 and _is(toks[j], 'p', ']'):
        start = j
        while start >= 0 and not (_is(toks[start], 'p', '[') and start >= 1 and _is(toks[start - 1], 'p', '#')):
            start -= 1
        body = toks[start + 1:j]
        if len(body) == 3 and _is(body[0], 'id', 'path') and _is(body[1], 'p', '=') and body[2].kind == 'str':
            return body[2].val
        j = start - 2
    return None


def crate_roots(project, patterns):
    roots = []
    for pattern in patterns:
        for crate in sorted(project.glob(pattern)):
            for root in ('src/lib.rs', 'src/main.rs'):
                if (crate / root).is_file():
                    roots.append((crate.relative_to(project).as_posix(), crate / root))
    return roots


def crate_dependencies(project, crates):
    """{crate: the crates it can name}: itself and its path dependencies, transitively, from Cargo.toml."""
    direct = {}
    for crate in crates:
        manifest = project / crate / 'Cargo.toml'
        found = set()
        if manifest.is_file():
            for m in re.finditer(r'path\s*=\s*"([^"]+)"', manifest.read_text(encoding='utf-8')):
                target = (project / crate / m.group(1)).resolve()
                if target.is_relative_to(project):
                    found.add(target.relative_to(project).as_posix())
        direct[crate] = found
    closure = {}
    for crate in crates:
        seen, stack = {crate}, [crate]
        while stack:
            for dep in direct.get(stack.pop(), ()):
                if dep not in seen:
                    seen.add(dep)
                    stack.append(dep)
        closure[crate] = seen
    return closure


def load_sources(project, patterns):
    sources, seen = [], set()
    for crate, root in crate_roots(project, patterns):
        for rel, toks in _module_files(root, project, seen):
            sources.append(Source(crate, rel, toks))
    return sources


# ---------------------------------------------------------------- functions

class Fn:
    def __init__(self, src, name, start, params, has_self, ret, body):
        self.src, self.name, self.start = src, name, start
        self.params = params            # [(name or None, type text)]
        self.has_self = has_self
        self.ret = ret                  # return type text
        self.body = body                # (index of `{`, index of `}`) or None
        self.code_params = set()
        self.severity_params = set()
        self.message_params = set()
        self.fixed_severity = None      # 'blocking' etc. when every diagnostic it builds states it
        self.local_cache = {}
        self.expr_body = None           # a closure's expression body (start, end)
        self.closure = False
        self.impl_type = None           # the type of the `impl` block it is in

    @property
    def key(self):
        return (self.name, len(self.params), self.has_self)

    def __repr__(self):
        return f'{self.src.rel}:{self.src.toks[self.start].line}:{self.name}'


def _text(toks):
    out = ''
    for t in toks:
        v = json.dumps(t.val) if t.kind == 'str' else t.val
        if out and (out[-1].isalnum() or out[-1] in '_"') and (v[:1].isalnum() or v[:1] in '_"'):
            out += ' '
        out += v
    return out


def _split_commas(toks, a, b):
    """The [start, end) ranges of the comma-separated parts of toks[a:b] at depth 0."""
    parts, depth, angle, start = [], 0, 0, a
    for j in range(a, b):
        t = toks[j]
        if t.kind != 'p':
            continue
        if t.val in OPEN:
            depth += 1
        elif t.val in CLOSE:
            depth -= 1
        elif t.val == '<' and j > a and (toks[j - 1].kind == 'id' or _is(toks[j - 1], 'p', '::')):
            angle += 1
        elif t.val == '>' and angle > 0:
            angle -= 1
        elif t.val == ',' and depth == 0 and angle == 0:
            parts.append((start, j))
            start = j + 1
    if start < b:
        parts.append((start, b))
    return [(s, e) for s, e in parts if e > s]


def impl_spans(src):
    """[(index of `{`, index of `}`, type name)] of each `impl ... {` block."""
    toks, out = src.toks, []
    for k, t in enumerate(toks):
        if not _is(t, 'id', 'impl'):
            continue
        j, depth, head = k + 1, 0, []
        while j < len(toks):
            x = toks[j]
            if _is(x, 'p', '<'):
                depth += 1
            elif _is(x, 'p', '>'):
                depth -= 1
            elif depth <= 0 and (_is(x, 'p', '{') or _is(x, 'p', ';')):
                break
            elif depth <= 0 and _is(x, 'id', 'where'):
                while j < len(toks) and not _is(toks[j], 'p', '{'):
                    j += 1
                break
            if depth <= 0 and x.kind == 'id':
                head.append(x.val)
            j += 1
        if j >= len(toks) or not _is(toks[j], 'p', '{'):
            continue
        if 'for' in head:
            head = head[head.index('for') + 1:]
        names = [h for h in head if h not in ('dyn', 'mut', 'unsafe', 'const', 'crate', 'super', 'self')]
        out.append((j, src.pairs.get(j, len(toks) - 1), names[-1] if names else None))
    return out


def find_functions(src):
    toks, pairs = src.toks, src.pairs
    impls = impl_spans(src)
    for k, t in enumerate(toks):
        if not (_is(t, 'id', 'fn') and k + 1 < len(toks) and toks[k + 1].kind == 'id'):
            continue
        name, j = toks[k + 1].val, k + 2
        if j < len(toks) and _is(toks[j], 'p', '<'):
            depth = 0
            while j < len(toks):
                if _is(toks[j], 'p', '<'):
                    depth += 1
                elif _is(toks[j], 'p', '>'):
                    depth -= 1
                    if depth == 0:
                        j += 1
                        break
                j += 1
        if j >= len(toks) or not _is(toks[j], 'p', '('):
            continue
        close = pairs.get(j)
        if close is None:
            continue
        params, has_self = [], False
        for s, e in _split_commas(toks, j + 1, close):
            seg = toks[s:e]
            vals = [x.val for x in seg]
            if 'self' in vals and (':' not in vals or vals.index('self') < vals.index(':')):
                has_self = True
                continue
            if ':' in vals:
                colon = vals.index(':')
                pname = vals[colon - 1] if colon >= 1 and seg[colon - 1].kind == 'id' else None
                params.append((pname, _text(seg[colon + 1:])))
            else:
                params.append((None, _text(seg)))
        m, depth = close + 1, 0
        ret_start = None
        body = None
        while m < len(toks):
            x = toks[m]
            if _is(x, 'p', '->') and depth == 0 and ret_start is None:
                ret_start = m + 1
            elif x.kind == 'p' and x.val in '([':
                depth += 1
            elif x.kind == 'p' and x.val in ')]':
                depth -= 1
            elif depth == 0 and _is(x, 'p', ';'):
                break
            elif depth == 0 and _is(x, 'p', '{'):
                body = (m, pairs.get(m, len(toks) - 1))
                break
            m += 1
        ret_end = m
        if ret_start is not None:
            for w in range(ret_start, m):
                if _is(toks[w], 'id', 'where'):
                    ret_end = w
                    break
        ret = _text(toks[ret_start:ret_end]) if ret_start is not None else ''
        f = Fn(src, name, k, params, has_self, ret, body)
        inside = [s for s in impls if s[0] < k < s[1]]
        f.impl_type = max(inside)[2] if inside else None
        src.fns.append(f)
    find_closures(src)


def find_closures(src):
    """Closures bound by `let [mut] name = [move] |params| body`, as functions of that name. Their body is
    a block, or an expression up to the `;` that ends the `let`."""
    toks, pairs = src.toks, src.pairs
    for k, t in enumerate(toks):
        if not _is(t, 'id', 'let'):
            continue
        j = k + 1
        if j < len(toks) and _is(toks[j], 'id', 'mut'):
            j += 1
        if not (j + 2 < len(toks) and toks[j].kind == 'id' and _is(toks[j + 1], 'p', '=')):
            continue
        name, j = toks[j].val, j + 2
        if j < len(toks) and _is(toks[j], 'id', 'move'):
            j += 1
        if j >= len(toks):
            continue
        if _is(toks[j], 'p', '||'):
            params, after = [], j + 1
        elif _is(toks[j], 'p', '|'):
            close = next((m for m in range(j + 1, min(j + 200, len(toks))) if _is(toks[m], 'p', '|')
                          and _depth_of(toks, j + 1, m) == 0), None)
            if close is None:
                continue
            params = []
            for s, e in _split_commas(toks, j + 1, close):
                seg = toks[s:e]
                vals = [x.val for x in seg]
                if ':' in vals:
                    colon = vals.index(':')
                    params.append((vals[colon - 1] if seg[colon - 1].kind == 'id' else None, _text(seg[colon + 1:])))
                else:
                    ids = [x.val for x in seg if x.kind == 'id' and x.val != 'mut']
                    params.append((ids[-1] if len(ids) == 1 else None, ''))
            after = close + 1
        else:
            continue
        ret = ''
        if after < len(toks) and _is(toks[after], 'p', '->'):
            m = after + 1
            while m < len(toks) and not _is(toks[m], 'p', '{'):
                m += 1
            ret, after = _text(toks[after + 1:m]), m
        if after < len(toks) and _is(toks[after], 'p', '{'):
            body = (after, pairs.get(after, len(toks) - 1))
            expr = None
        else:
            e, d = after, 0
            while e < len(toks):
                x = toks[e]
                if x.kind == 'p' and x.val in OPEN:
                    d += 1
                elif x.kind == 'p' and x.val in CLOSE:
                    if d == 0:
                        break
                    d -= 1
                elif d == 0 and _is(x, 'p', ';'):
                    break
                e += 1
            body = (after - 1, e)
            expr = (after, e)
        f = Fn(src, name, k, params, False, ret or 'closure', body)
        f.expr_body = expr
        f.closure = True
        src.fns.append(f)


def _stringish(type_text):
    return bool(STRINGISH.match(type_text.replace(' ', '').replace("&'static", "&").replace('&mut', '&'))) or \
        bool(STRINGISH.match(type_text))


def _returns_text(ret):
    r = ret.replace(' ', '')
    return any(x in r for x in ('str', 'String', 'Cow<', 'closure'))


# ---------------------------------------------------------------- the scan

class Scan:
    def __init__(self, project=PROJECT):
        self.project = project
        self.sources = load_sources(project, SCOPE)
        self.reuse = load_sources(project, REUSE_SCOPE)
        for src in self.sources + self.reuse:
            find_functions(src)
            src.index_functions()
        self.all_sources = self.sources + self.reuse
        self.call_cache = None
        self.deps = crate_dependencies(project, sorted({s.crate for s in self.all_sources}))
        self.fns = [f for src in self.all_sources for f in src.fns]
        self.by_name = {}
        for f in self.fns:
            self.by_name.setdefault(f.name, []).append(f)
        self.consts = {}         # name -> [(value tokens, src, line, index)]
        self.aliases = {}        # `use path::NAME as ALIAS`: ALIAS -> NAME
        for src in self.all_sources:
            self._collect_consts(src)
            toks = src.toks
            for k in range(1, len(toks) - 1):
                if _is(toks[k], 'id', 'as') and toks[k - 1].kind == 'id' and toks[k + 1].kind == 'id' and \
                        CONST_NAME.match(toks[k - 1].val) and CONST_NAME.match(toks[k + 1].val) and \
                        len(toks[k - 1].val) > 1:
                    self.aliases[toks[k + 1].val] = toks[k - 1].val
        self.codes = {}          # code -> {'sites': {(file, line): site}}
        self.unresolved = {}     # (file, line, expression) -> why
        self.claimed = set()     # (file, token index) of literals counted as raised or compared
        self.diag_callers = set()

    # -- constants

    def _collect_consts(self, src):
        toks = src.toks
        for k, t in enumerate(toks):
            if (_is(t, 'id', 'const') or _is(t, 'id', 'static')) and k + 2 < len(toks) and toks[k + 1].kind == 'id' \
                    and _is(toks[k + 2], 'p', ':'):
                name = toks[k + 1].val
                j = k + 3
                while j < len(toks) and not _is(toks[j], 'p', '=') and not _is(toks[j], 'p', ';'):
                    j += 1
                if j >= len(toks) or not _is(toks[j], 'p', '='):
                    continue
                type_text = _text(toks[k + 3:j])
                if 'str' not in type_text:
                    continue
                e = j + 1
                depth = 0
                while e < len(toks):
                    x = toks[e]
                    if x.kind == 'p' and x.val in OPEN:
                        depth += 1
                    elif x.kind == 'p' and x.val in CLOSE:
                        depth -= 1
                    elif depth == 0 and _is(x, 'p', ';'):
                        break
                    e += 1
                self.consts.setdefault(name, []).append((j + 1, e, src, t.line))

    def resolve_const(self, path, src, depth):
        name = path[-1]
        if name not in self.consts and name in self.aliases:
            name = self.aliases[name]
            path = [name]
        visible = self.deps.get(src.crate, {src.crate})
        entries = [x for x in self.consts.get(name, []) if x[2].crate in visible]
        if len(path) > 1:
            qualified = [x for x in entries if x[2].crate.split('/')[-1] in path[-2] or path[-2] in x[2].rel]
            entries = qualified or entries
        if len(entries) > 1:
            same = [x for x in entries if x[2].crate == src.crate]
            entries = same or entries
        out = []
        for a, b, csrc, line in entries:
            for r in self.resolve(csrc.toks[a:b], None, csrc, depth + 1, a):
                if r[0] == 'code':
                    out.append(('code', r[1], r[2], r[3], r[4] + [f'const {name}'], r[5]))
                else:
                    out.append(r)
        if not entries:
            out.append(('unresolved', f'constant {"::".join(path)} not found in scope'))
        return out

    # -- context

    def enclosing_fn(self, src, k):
        return src.fn_at[k] if 0 <= k < len(src.fn_at) else None

    # -- expression resolution

    def resolve(self, toks, fn, src, depth, base=0):
        """[('code', value, src, line, via, token index) | ('param', index) | ('relay',) |
        ('enum',) | ('unresolved', why)] for the value of the expression `toks`."""
        if depth > MAX_DEPTH:
            return [('unresolved', 'resolution too deep')]
        a, b = 0, len(toks)
        # unwrap & * ( ) and same-value methods
        changed = True
        while changed and a < b:
            changed = False
            if _is(toks[a], 'p', '&') or _is(toks[a], 'p', '*'):
                a += 1
                changed = True
                if a < b and _is(toks[a], 'id', 'mut'):
                    a += 1
            if a < b and _is(toks[a], 'p', '(') and _match_forward(toks, a) == b - 1:
                parts = _split_commas(toks, a + 1, b - 1)
                if len(parts) == 1:
                    a, b = a + 1, b - 1
                    changed = True
            if b - a >= 4 and _is(toks[b - 1], 'p', ')') and _is(toks[b - 2], 'p', '(') and \
                    toks[b - 3].kind == 'id' and toks[b - 3].val in SAME_VALUE_METHODS and _is(toks[b - 4], 'p', '.'):
                b -= 4
                changed = True
            for w in WRAPPERS:
                parts = w.split('::')
                L = len(parts) * 2 - 1
                if b - a > L + 1 and [x.val for x in toks[a:a + L]] == [y for p in parts for y in (p, '::')][:L] and \
                        _is(toks[a + L], 'p', '(') and _match_forward(toks, a + L) == b - 1:
                    a, b = a + L + 1, b - 1
                    changed = True
                    break
        seg = toks[a:b]
        if not seg:
            return [('unresolved', 'empty expression')]
        if len(seg) == 1 and seg[0].kind == 'str':
            return [('code', seg[0].val, src, seg[0].line, [], base + a)]
        if len(seg) == 1 and seg[0].kind == 'num':
            return [('unresolved', 'a number')]
        if len(seg) == 1 and _is(seg[0], 'id', 'None'):
            return []                           # no code: the arm raises nothing
        # a path
        if all(x.kind == 'id' or _is(x, 'p', '::') for x in seg):
            path = [x.val for x in seg if x.kind == 'id']
            last = path[-1]
            if CONST_NAME.match(last) and any(c.isalpha() for c in last):
                return self.resolve_const(path, src, depth)
            if len(path) > 1 and last[:1].isupper():
                return [('enum',)]
            if len(path) == 1:
                return self.resolve_name(last, fn, src, depth, base + a)
            return [('unresolved', f'path {"::".join(path)}')]
        # field access: x.code (relay of a record's code) or another field
        if len(seg) >= 3 and _is(seg[-2], 'p', '.') and seg[-1].kind == 'id' and \
                all(x.kind == 'id' or (x.kind == 'p' and x.val in ('.', '::')) for x in seg):
            if seg[-1].val in ('code', 'reason_code'):
                return [('relay',)]
            return [('unresolved', f'field .{seg[-1].val}')]
        # x.code.as_str() / x.code.as_str() relay
        if len(seg) >= 6 and [x.val for x in seg[-5:]] == ['code', '.', 'as_str', '(', ')'] or \
                (len(seg) >= 4 and [x.val for x in seg[-4:]] == ['.', 'code', '(', ')'] and False):
            return [('relay',)]
        first = seg[0]
        # match / if expressions
        if _is(first, 'id', 'match'):
            return self.resolve_match(seg, fn, src, depth, base + a)
        if _is(first, 'id', 'if'):
            return self.resolve_if(seg, fn, src, depth, base + a)
        if _is(first, 'p', '{') and _match_forward(seg, 0) == len(seg) - 1:
            return self.resolve_block(seg, 0, fn, src, depth, base + a)
        # macros
        if first.kind == 'id' and len(seg) >= 3 and _is(seg[1], 'p', '!'):
            if first.val == 'format' and len(seg) >= 4 and seg[3].kind == 'str':
                m = re.fullmatch(r'\{([A-Z][A-Z0-9_]*)\}', seg[3].val)
                if m:
                    return self.resolve_const([m.group(1)], src, depth)
            if first.val == 'concat':
                return [('unresolved', 'concat! macro')]
            return [('unresolved', f'built at run time ({first.val}!)')]
        # a call: F(args), path::F(args), x.F(args)
        if _is(seg[-1], 'p', ')'):
            open_k = None
            depth_b = 0
            for j in range(len(seg) - 1, -1, -1):
                if _is(seg[j], 'p', ')'):
                    depth_b += 1
                elif _is(seg[j], 'p', '('):
                    depth_b -= 1
                    if depth_b == 0:
                        open_k = j
                        break
            if open_k is not None and open_k >= 1 and seg[open_k - 1].kind == 'id':
                return self.resolve_call(seg, open_k, fn, src, depth, base + a)
        # a closure or something else
        return [('unresolved', 'expression')]

    def resolve_name(self, name, fn, src, depth, at):
        if fn is not None:
            for idx, (pname, _type) in enumerate(fn.params):
                if pname == name and self._binding_before(src, fn, name, at) is None:
                    return [('param', idx)]
            bound = self._binding_before(src, fn, name, at)
            if bound is not None:
                return self.resolve_binding(bound, name, fn, src, depth)
            if name == 'code' and self._pattern_bound(src, fn, name, at):
                # `for (code, message) in findings`, `Err((code, message))`, `Refused { code, message }`:
                # a relay of a (code, message) pair or record whose construction is its own site
                return [('relay',)]
            if fn.closure:                      # a name the closure captures from the function around it
                outer = self.enclosing_fn(src, fn.start)
                if outer is not None and outer is not fn:
                    return self.resolve_name(name, outer, src, depth + 1, fn.start)
        return [('unresolved', f'name {name} bound by a pattern, closure or loop')]

    def _pattern_bound(self, src, fn, name, at):
        """Whether `name` is bound before `at` in `fn` by a tuple or struct pattern (of a `for`, `let`,
        `if let`, `match` arm or closure parameter)."""
        toks = src.toks
        lo = fn.body[0] if fn.body else 0
        if fn.closure:
            lo = fn.start
        for j in range(at - 1, lo, -1):
            if not _is(toks[j], 'id', name) or toks[j - 1].kind != 'p' or toks[j - 1].val not in ('(', ',', '{', '|'):
                continue
            if toks[j - 1].val == '|' and fn.closure:
                return True
            o = max(src.paren_at[j], src.brace_at[j])
            if o < 0:
                continue
            c = src.pairs.get(o)
            if c is None:
                continue
            m = c + 1
            while m < len(toks) and toks[m].kind == 'p' and toks[m].val in (')', '}'):
                m += 1
            if m < len(toks) and (_is(toks[m], 'id', 'in') or (toks[m].kind == 'p' and toks[m].val in ('=', '=>', '|', ':'))):
                return True
        return False

    def _binding_before(self, src, fn, name, at):
        """The last `let` that binds `name` before index `at` in the body of `fn`: (index of `let`,
        pattern range, value range) or None."""
        if fn is None or fn.body is None:
            return None
        toks = src.toks
        found = None
        k = fn.body[0]
        while k < at:
            if _is(toks[k], 'id', 'let'):
                # pattern up to `=` (or `;`) at depth 0
                j, depth = k + 1, 0
                while j < len(toks):
                    x = toks[j]
                    if x.kind == 'p' and x.val in OPEN:
                        depth += 1
                    elif x.kind == 'p' and x.val in CLOSE:
                        depth -= 1
                    elif depth == 0 and x.kind == 'p' and x.val in ('=', ';'):
                        break
                    j += 1
                pattern = (k + 1, j)
                names = [toks[w].val for w in range(*pattern) if toks[w].kind == 'id']
                colon = next((w for w in range(*pattern) if _is(toks[w], 'p', ':') and
                              _depth_of(toks, pattern[0], w) == 0), None)
                if colon is not None:
                    names = [toks[w].val for w in range(pattern[0], colon) if toks[w].kind == 'id']
                    pattern = (pattern[0], colon)
                if name in names and j < len(toks) and _is(toks[j], 'p', '='):
                    e, depth = j + 1, 0
                    conditional = k >= 1 and toks[k - 1].kind == 'id' and toks[k - 1].val in ('if', 'while')
                    while e < len(toks):
                        x = toks[e]
                        if conditional and depth == 0 and _is(x, 'p', '{') and e > j + 1 and \
                                not toks[e - 1].val[:1].isupper():
                            break                   # `if let PAT = value {`
                        if x.kind == 'p' and x.val in OPEN:
                            depth += 1
                        elif x.kind == 'p' and x.val in CLOSE:
                            depth -= 1
                        elif depth == 0 and _is(x, 'p', ';'):
                            break
                        e += 1
                    if not (j < at <= e):        # not the binding whose own value is being read
                        found = (k, pattern, (j + 1, e))
            k += 1
        return found

    def resolve_binding(self, bound, name, fn, src, depth):
        toks = src.toks
        _let, (pa, pb), (va, vb) = bound
        pat = [x for x in toks[pa:pb] if not _is(x, 'id', 'mut')]
        value = toks[va:vb]
        # `let ... else { ... }`
        if len(pat) == 1:
            return self.resolve(value, fn, src, depth + 1, va)
        # `if let Some(code) = value`: the wrapped value
        if len(pat) == 4 and pat[0].kind == 'id' and pat[0].val in ('Some', 'Ok', 'Err') and _is(pat[1], 'p', '(') \
                and _is(pat[2], 'id', name) and _is(pat[3], 'p', ')'):
            return self.resolve(value, fn, src, depth + 1, va)
        if pat and _is(pat[0], 'p', '(') and _is(pat[-1], 'p', ')'):
            names = []
            for s, e in _split_commas(pat, 1, len(pat) - 1):
                ids = [x.val for x in pat[s:e] if x.kind == 'id']
                names.append(ids[-1] if ids else None)
            if name in names:
                out = self.resolve_tuple(value, names.index(name), fn, src, depth, va)
                if name == 'code' and all(r[0] == 'unresolved' for r in out):
                    return [('relay',)]         # a (code, message) pair returned by a call
                return out
        if name == 'code':
            return [('relay',)]                 # `if let Refused { code, message } = ...`
        return [('unresolved', f'name {name} bound by a destructuring let')]

    def tuple_arms(self, value, base):
        """[(token start, token end) of the value of each arm] of a tuple-valued expression: the whole
        value when it is a tuple, or each `=> (...)` arm of a `match`."""
        if value and _is(value[0], 'p', '(') and _match_forward(value, 0) == len(value) - 1:
            return [(0, len(value))]
        arms = []
        for j in range(len(value) - 1):
            if _is(value[j], 'p', '=>') and _is(value[j + 1], 'p', '('):
                arms.append((j + 1, _match_forward(value, j + 1) + 1))
        return arms

    def resolve_tuple(self, value, index, fn, src, depth, base):
        arms = self.tuple_arms(value, base)
        if not arms:
            return [('unresolved', 'a tuple binding whose value is not a tuple or match of tuples')]
        out = []
        for s, e in arms:
            parts = _split_commas(value, s + 1, e - 1)
            if index < len(parts):
                ps, pe = parts[index]
                out += self.resolve(value[ps:pe], fn, src, depth + 1, base + ps)
            else:
                out.append(('unresolved', 'a tuple arm with too few elements'))
        return out

    def arm_values(self, seg):
        """[(start, end)] of the arm values of the `match` expression `seg`."""
        brace = next(j for j in range(len(seg)) if _is(seg[j], 'p', '{') and _depth_of(seg, 0, j) == 0)
        end = _match_forward(seg, brace)
        out, j = [], brace + 1
        while j < end:
            # pattern up to => at depth 0
            depth = 0
            while j < end and not (_is(seg[j], 'p', '=>') and depth == 0):
                if seg[j].kind == 'p' and seg[j].val in OPEN:
                    depth += 1
                elif seg[j].kind == 'p' and seg[j].val in CLOSE:
                    depth -= 1
                j += 1
            if j >= end:
                break
            s = j + 1
            if s < end and _is(seg[s], 'p', '{'):
                e = _match_forward(seg, s) + 1
                out.append((s, e))
                j = e
                if j < end and _is(seg[j], 'p', ','):
                    j += 1
                continue
            depth, e = 0, s
            while e < end:
                x = seg[e]
                if x.kind == 'p' and x.val in OPEN:
                    depth += 1
                elif x.kind == 'p' and x.val in CLOSE:
                    depth -= 1
                elif depth == 0 and _is(x, 'p', ','):
                    break
                e += 1
            out.append((s, e))
            j = e + 1
        return out

    def resolve_match(self, seg, fn, src, depth, base):
        out = []
        for s, e in self.arm_values(seg):
            out += self.resolve(seg[s:e], fn, src, depth + 1, base + s)
        return [r for r in out if not (r[0] == 'unresolved' and r[1] in ('a diverging arm',))]

    def resolve_block(self, seg, k, fn, src, depth, base):
        """The value of the block opening at seg[k]: its tail expression after the last `;`."""
        end = _match_forward(seg, k)
        last, d = k + 1, 0
        for j in range(k + 1, end):
            x = seg[j]
            if x.kind == 'p' and x.val in OPEN:
                d += 1
            elif x.kind == 'p' and x.val in CLOSE:
                d -= 1
            elif d == 0 and _is(x, 'p', ';'):
                last = j + 1
        tail = seg[last:end]
        if not tail:
            return [('unresolved', 'a diverging arm')]
        if tail[0].kind == 'id' and tail[0].val in ('return', 'panic', 'unreachable', 'continue', 'break'):
            return [('unresolved', 'a diverging arm')]
        return self.resolve(tail, fn, src, depth + 1, base + last)

    def resolve_if(self, seg, fn, src, depth, base):
        out, j = [], 0
        while j < len(seg):
            if _is(seg[j], 'p', '{') and _depth_of(seg, 0, j) == 0:
                end = _match_forward(seg, j)
                out += self.resolve_block(seg, j, fn, src, depth, base)
                j = end + 1
                continue
            j += 1
        return [r for r in out if not (r[0] == 'unresolved' and r[1] == 'a diverging arm')]

    def resolve_call(self, seg, open_k, fn, src, depth, base):
        name = seg[open_k - 1].val
        method = open_k >= 2 and _is(seg[open_k - 2], 'p', '.')
        close = _match_forward(seg, open_k)
        args = _split_commas(seg, open_k + 1, close)
        candidates = [f for f in self._callable(self.by_name.get(name, []), seg, open_k - 1, fn)
                      if len(f.params) == len(args) and _returns_text(f.ret)]
        if not candidates:
            if method and name in ('as_str', 'code') and len(args) == 0:
                candidates = [f for f in self.by_name.get(name, []) if f.has_self and _returns_text(f.ret)]
            visible = self.deps.get(src.crate, {src.crate})
            candidates = [f for f in candidates if f.src.crate in visible]
            if not candidates:
                return [('unresolved', f'call {name}() returns no resolvable code')]
        visible = self.deps.get(src.crate, {src.crate})
        candidates = [f for f in candidates if f.src.crate in visible]
        if not candidates:
            return [('unresolved', f'call {name}() returns no resolvable code')]
        same = [f for f in candidates if f.src.crate == src.crate]
        candidates = same or candidates
        out = []
        for g in candidates:
            for r in self.returned(g, depth + 1):
                if r[0] == 'param':
                    s, e = args[r[1]]
                    out += self.resolve(seg[s:e], fn, src, depth + 1, base + s)
                elif r[0] == 'code':
                    out.append(('code', r[1], r[2], r[3], r[4] + [f'returned by {g.name}()'], r[5]))
                else:
                    out.append(r)
        codes = [r for r in out if r[0] in ('code', 'param', 'relay')]
        if not codes:
            return [('unresolved', f'call {name}() returns no resolvable code')]
        # a code-returning function's other return points (a computed text) are reported once, at it
        return [r for r in out if r[0] != 'unresolved']

    def returned(self, g, depth):
        if g.body is None:
            return []
        key = ('returned', depth > 4)
        if key in g.local_cache:
            return g.local_cache[key]
        g.local_cache[key] = []          # recursion guard
        toks = g.src.toks
        a, b = g.body
        if g.expr_body is not None:
            s, e = g.expr_body
            out = self.resolve(toks[s:e], g, g.src, depth, s)
            g.local_cache[key] = out
            return out
        out = self.resolve_block(toks[a:b + 1], 0, g, g.src, depth, a)
        for k in range(a, b):
            if _is(toks[k], 'id', 'return'):
                e, d = k + 1, 0
                while e < b:
                    x = toks[e]
                    if x.kind == 'p' and x.val in OPEN:
                        d += 1
                    elif x.kind == 'p' and x.val in CLOSE:
                        if d == 0:
                            break
                        d -= 1
                    elif d == 0 and x.kind == 'p' and x.val in (';', ','):
                        break
                    e += 1
                out += self.resolve(toks[k + 1:e], g, g.src, depth, k + 1)
        out = [r for r in out if not (r[0] == 'unresolved' and r[1] == 'a diverging arm')]
        g.local_cache[key] = out
        return out

    # -- code positions

    def callables(self):
        """Mark each function's code, severity and message parameters, to a fixed point."""
        for f in self.fns:
            for idx, (pname, ptype) in enumerate(f.params):
                if pname == 'code' and (_stringish(ptype) or (f.closure and not ptype)):
                    f.code_params.add(idx)
                if pname == 'severity' and (_stringish(ptype) or (f.closure and not ptype)):
                    f.severity_params.add(idx)
                if pname in MESSAGE_PARAMS:
                    f.message_params.add(idx)
        changed = True
        while changed:
            changed = False
            for src in self.all_sources:
                for k, args, targets, caller in self.calls(src):
                    if caller is None:
                        continue
                    for g in targets:
                        for kind in ('code_params', 'severity_params', 'message_params'):
                            for idx in getattr(g, kind):
                                if idx >= len(args):
                                    continue
                                s, e = args[idx]
                                seg = [x for x in src.toks[s:e] if not (_is(x, 'p', '&'))]
                                while len(seg) >= 4 and seg[-3].kind == 'id' and seg[-3].val in SAME_VALUE_METHODS:
                                    seg = seg[:-4]
                                if len(seg) == 1 and seg[0].kind == 'id':
                                    for pidx, (pname, _t) in enumerate(caller.params):
                                        if pname == seg[0].val and pidx not in getattr(caller, kind) and \
                                                self._binding_before(src, caller, pname, s) is None:
                                            getattr(caller, kind).add(pidx)
                                            changed = True

    def calls(self, src):
        """(index of the name, [argument ranges], [functions it may call], enclosing fn) of each call
        in `src` to a function with a code, severity or message parameter."""
        toks, out = src.toks, []
        for k in range(len(toks) - 1):
            t = toks[k]
            if t.kind != 'id' or not _is(toks[k + 1], 'p', '(') or (k >= 1 and _is(toks[k - 1], 'id', 'fn')):
                continue
            fns = self.by_name.get(t.val)
            if not fns:
                continue
            close = src.pairs.get(k + 1)
            if close is None:
                continue
            args = _split_commas(toks, k + 2, close)
            caller = self.enclosing_fn(src, k)
            visible = self.deps.get(src.crate, {src.crate})
            targets = [f for f in self._callable(fns, toks, k, caller) if len(f.params) == len(args)
                       and f.src.crate in visible
                       and (f.code_params or f.severity_params or f.message_params)]
            if not targets:
                continue
            same = [f for f in targets if f.src.crate == src.crate] or targets
            out.append((k, args, same, caller))
        return out

    @staticmethod
    def _callable(fns, toks, k, caller):
        """The functions named at toks[k] a call there can reach: methods for `.name(`, the associated
        functions of `Type` for `Type::name(` (`Self` is the caller's impl type), free functions and
        closures otherwise."""
        if k >= 1 and _is(toks[k - 1], 'p', '.'):
            return [f for f in fns if f.has_self]
        qual = toks[k - 2].val if k >= 2 and _is(toks[k - 1], 'p', '::') and toks[k - 2].kind == 'id' else None
        if qual == 'Self':
            qual = caller.impl_type if caller is not None and caller.impl_type else None
            if qual is None:
                return [f for f in fns if not f.has_self and f.impl_type is not None]
        if qual and qual[:1].isupper():
            return [f for f in fns if not f.has_self and f.impl_type == qual]
        return [f for f in fns if not f.has_self and f.impl_type is None]

    def cached_calls(self, src):
        if self.call_cache is None:
            self.call_cache = {}
        if src.rel not in self.call_cache:
            self.call_cache[src.rel] = self.calls(src)
        return self.call_cache[src.rel]

    def run(self):
        self.callables()
        for f in self.fns:
            self.fix_severity(f)
        for src in self.sources:
            self.scan_source(src, reuse=False)
        for src in self.reuse:
            self.scan_source(src, reuse=True)
        self.scan_unclaimed()
        return self

    def fix_severity(self, f, seen=None):
        """The one severity every diagnostic `f` builds states, when `f` has no severity parameter."""
        if f.fixed_severity is not None or f.severity_params or f.body is None:
            return f.fixed_severity
        seen = seen or set()
        if id(f) in seen:
            return None
        seen.add(id(f))
        toks = f.src.toks
        found = set()
        for k in range(f.body[0], f.body[1]):
            if _is(toks[k], 'id', 'severity') and k + 1 < len(toks) and _is(toks[k + 1], 'p', ':') and \
                    k + 2 < len(toks) and toks[k + 2].kind == 'str':
                found.add(toks[k + 2].val)
        for k, args, targets, _caller in self.cached_calls(f.src):
            if not (f.body[0] < k < f.body[1]):
                continue
            for g in targets:
                for idx in g.severity_params:
                    if idx < len(args):
                        s, e = args[idx]
                        lits = [x.val for x in toks[s:e] if x.kind == 'str']
                        found.update(lits)
                if not g.severity_params and g.code_params:
                    sev = self.fix_severity(g, seen)
                    if sev:
                        found.add(sev)
        found &= SEVERITIES
        if len(found) == 1:
            f.fixed_severity = found.pop()
        return f.fixed_severity

    def record(self, results, src, k, via, severity, message, reuse):
        toks = src.toks
        expr_line = toks[k].line
        for r in results:
            if r[0] == 'code' and not CODE_SHAPE.match(r[1]):
                # text at a code position that is not code-shaped: the lowercase reason tokens of the
                # retained-precision records, or a message constant; no diagnostic code
                continue
            if r[0] == 'code':
                value, csrc, line, chain, tok_index = r[1], r[2], r[3], r[4], r[5]
                if reuse and not chain:
                    continue            # the applier's own literal codes are not solve refusals
                if reuse and csrc.crate in REUSE_SCOPE:
                    continue
                self.claimed.add((csrc.rel, tok_index))
                entry = self.codes.setdefault(value, {'sites': {}})
                site_key = (src.rel, expr_line)
                site = entry['sites'].setdefault(site_key, {'file': src.rel, 'line': expr_line, 'via': via,
                                                            'severity': severity, 'message': message})
                if chain:
                    site.setdefault('through', [])
                    note = f'{" <- ".join(chain)} ({csrc.rel}:{line})'
                    if note not in site['through']:
                        site['through'].append(note)
                        site['through'].sort()
            elif r[0] == 'unresolved' and not reuse:
                self.unresolved[(src.rel, expr_line, via)] = r[1]

    def scan_source(self, src, reuse):
        toks = src.toks
        # calls to code-taking functions
        for k, args, targets, caller in self.cached_calls(src):
            coded = [g for g in targets if g.code_params]
            if not coded:
                continue
            g = coded[0]
            for idx in sorted(set().union(*(h.code_params for h in coded))):
                if idx >= len(args):
                    continue
                s, e = args[idx]
                results = self.resolve(toks[s:e], caller, src, 0, s)
                severity = None
                for sidx in g.severity_params:
                    if sidx < len(args):
                        sv = self.resolve(toks[args[sidx][0]:args[sidx][1]], caller, src, 0, args[sidx][0])
                        vals = {r[1] for r in sv if r[0] == 'code' and r[1] in SEVERITIES}
                        severity = vals.pop() if len(vals) == 1 else ('varies' if vals else None)
                if not g.severity_params:
                    severity = g.fixed_severity
                message = None
                for midx in sorted(g.message_params):
                    if midx < len(args):
                        message = self.message_text(toks[args[midx][0]:args[midx][1]])
                        break
                results = [r for r in results if r[0] != 'param' or self._param_unmarked(caller, r[1])]
                results = [('unresolved', f'parameter {caller.params[r[1]][0]} of {caller.name}()')
                           if r[0] == 'param' else r for r in results]
                results = [r for r in results if r[0] not in ('relay', 'enum')]
                self.record(results, src, k, f'{g.name}()', severity, message, reuse)
        # struct fields `code: value`, shorthand `code`, and assignments `.code = value`
        for k, t in enumerate(toks):
            if not _is(t, 'id', 'code') or k == 0:
                continue
            prev, nxt = toks[k - 1], toks[k + 1] if k + 1 < len(toks) else None
            if _is(nxt, 'p', ':') and prev.kind == 'p' and prev.val in ('{', ','):
                if self._in_definition(src, k):
                    continue
                e = self._value_end(toks, k + 2)
                value = toks[k + 2:e]
                if self._is_type(value):
                    continue
                fn = self.enclosing_fn(src, k)
                results = self.resolve(value, fn, src, 0, k + 2)
                self.record_field(results, src, k, fn, 'field code', reuse)
            elif _is(nxt, 'p', '=') and _is(prev, 'p', '.'):
                e = self._value_end(toks, k + 2, stop=';')
                fn = self.enclosing_fn(src, k)
                results = self.resolve(toks[k + 2:e], fn, src, 0, k + 2)
                self.record_field(results, src, k, fn, 'assignment .code', reuse)
            elif nxt is not None and prev.kind == 'p' and prev.val in ('{', ',') and nxt.kind == 'p' and \
                    nxt.val in (',', '}') and self._struct_literal(src, k):
                fn = self.enclosing_fn(src, k)
                results = self.resolve([t], fn, src, 0, k)
                self.record_field(results, src, k, fn, 'field code', reuse)
        # message prefixes `CODE: ...` and `{CONST}: ...`
        if reuse:
            return
        self.scan_pairs(src)
        for k, t in enumerate(toks):
            if t.kind != 'str' or self._compared(toks, k) or self._not_a_diagnostic(toks, k):
                continue
            m = PREFIX_LITERAL.match(t.val)
            if m:
                self.claimed.add((src.rel, k))
                self.record([('code', m.group(1), src, t.line, [], k)], src, k, 'message prefix', None,
                            self.message_text([t]), reuse)
                continue
            m = PREFIX_FORMAT.match(t.val)
            if m and self._in_format_macro(src, k):
                results = self.resolve_const([m.group(1)], src, 0)
                self.record(results, src, k, 'message prefix', None, self.message_text([t]), reuse)

    def _in_format_macro(self, src, k):
        """Whether the literal at k is the format string of `format!`, `write!` or `writeln!`."""
        o = src.paren_at[k]
        toks = src.toks
        if o < 2 or not _is(toks[o - 1], 'p', '!') or toks[o - 2].kind != 'id':
            return False
        if toks[o - 2].val not in ('format', 'write', 'writeln'):
            return False
        first = [j for j in range(o + 1, k + 1) if toks[j].kind == 'str']
        return first == [k]

    def scan_pairs(self, src):
        """`(CODE, message)` pairs: a tuple whose first element is a code (a literal or a constant) and
        whose second is a message (a text literal, a `format!`, a `.to_string()`-like value, or a
        name or call that says message). The codebase carries refusals this way into `findings.push`,
        `Err(...)` and `Some(...)`, and relays them through `for (code, message) in ...` patterns."""
        toks = src.toks
        for k, t in enumerate(toks):
            if not _is(t, 'p', '(') or k == 0:
                continue
            prev = toks[k - 1]
            if prev.kind == 'id' and prev.val not in ('return', 'in', 'match', 'if', 'else'):
                continue
            if prev.kind == 'p' and prev.val in ('!', ')', ']', '>'):
                continue
            close = src.pairs.get(k)
            if close is None:
                continue
            nxt = toks[close + 1] if close + 1 < len(toks) else None
            if nxt is not None and (_is(nxt, 'id', 'in') or (nxt.kind == 'p' and nxt.val in ('=', '=>', '|'))):
                continue
            if prev.kind == 'id' and prev.val in ('let', 'for'):
                continue
            parts = _split_commas(toks, k + 1, close)
            if len(parts) not in (2, 3):
                continue
            fn = self.enclosing_fn(src, k)
            for at in range(len(parts) - 1):      # the code first, or second after an id
                first = toks[parts[at][0]:parts[at][1]]
                if not (len(first) == 1 and first[0].kind == 'str' and CODE_SHAPE.match(first[0].val)) and \
                        not (all(x.kind == 'id' or _is(x, 'p', '::') for x in first) and first and
                             CONST_NAME.match(first[-1].val) and len(first[-1].val) > 2):
                    continue
                message = toks[parts[at + 1][0]:parts[at + 1][1]]
                if not self._message_like(message):
                    continue
                results = self.resolve(first, fn, src, 0, parts[at][0])
                results = [r for r in results if r[0] == 'code' and CODE_SHAPE.match(r[1])]
                if not results:
                    continue
                severity = None
                for p in parts:
                    sev = [x.val for x in toks[p[0]:p[1]] if x.kind == 'str' and x.val in SEVERITIES]
                    if len(sev) == 1 and p[1] - p[0] == 1:
                        severity = sev[0]
                self.record(results, src, k, 'code-message pair', severity, self.message_text(message), False)
                break
        # `json!({"code": CODE, ...})` and `Err("CODE")`
        for k, t in enumerate(toks):
            if t.kind == 'str' and t.val == 'code' and k + 2 < len(toks) and _is(toks[k + 1], 'p', ':') and \
                    _is(toks[k - 1], 'p', '{') | _is(toks[k - 1], 'p', ','):
                brace = self._enclosing_brace(src, k)
                if brace is None or not any(x.kind == 'str' and x.val == 'severity'
                                            for x in toks[brace:src.pairs.get(brace, brace)]):
                    continue                # a reason record of the retained wire, not a diagnostic
                e = self._value_end(toks, k + 2)
                fn = self.enclosing_fn(src, k)
                results = [r for r in self.resolve(toks[k + 2:e], fn, src, 0, k + 2) if r[0] in ('code', 'unresolved')]
                severity = message = None
                if brace is not None:
                    end = src.pairs.get(brace, len(toks) - 1)
                    for j in range(brace + 1, end - 1):
                        if toks[j].kind == 'str' and toks[j].val == 'severity' and _is(toks[j + 1], 'p', ':') \
                                and toks[j + 2].kind == 'str':
                            severity = toks[j + 2].val
                        if toks[j].kind == 'str' and toks[j].val == 'message' and _is(toks[j + 1], 'p', ':'):
                            message = self.message_text(toks[j + 2:self._value_end(toks, j + 2)])
                self.record(results, src, k, 'json code', severity, message, False)
            elif _is(t, 'id', 'Err') and k + 2 < len(toks) and _is(toks[k + 1], 'p', '(') and \
                    toks[k + 2].kind == 'str' and CODE_SHAPE.match(toks[k + 2].val):
                close = src.pairs.get(k + 1)
                inner = toks[k + 2:close]
                if len(inner) == 1 or (len(inner) == 5 and inner[2].val in SAME_VALUE_METHODS):
                    self.record([('code', inner[0].val, src, inner[0].line, [], k + 2)], src, k, 'error value',
                                None, None, False)

    @staticmethod
    def _message_like(seg):
        seg = [x for x in seg if not _is(x, 'p', '&')]
        if not seg:
            return False
        if len(seg) == 1 and seg[0].kind == 'str':
            return ' ' in seg[0].val
        if len(seg) == 1 and seg[0].kind == 'id':
            return seg[0].val in MESSAGE_PARAMS or 'message' in seg[0].val or seg[0].val.endswith('_TEXT') \
                or seg[0].val.endswith('_MESSAGE')
        if seg[0].kind == 'id' and seg[0].val == 'format' and len(seg) > 1 and _is(seg[1], 'p', '!'):
            return True
        if len(seg) >= 4 and seg[-3].kind == 'id' and seg[-3].val in ('to_string', 'into', 'to_owned') and \
                any(x.kind == 'str' or 'message' in x.val or x.val.endswith('_TEXT') for x in seg):
            return True
        if any(x.kind == 'id' and ('message' in x.val or x.val.endswith('_TEXT') or x.val.endswith('_MESSAGE'))
               for x in seg[:3]):
            return True
        return False

    def _not_a_diagnostic(self, toks, k):
        """Whether the literal at k is the first argument of a panic, an assertion or a print, or of
        `.expect(...)`: text a person reads in a crash or a log, never a diagnostic."""
        if k >= 3 and _is(toks[k - 1], 'p', '(') and _is(toks[k - 2], 'p', '!') and toks[k - 3].kind == 'id':
            return toks[k - 3].val in ('panic', 'unreachable', 'assert', 'assert_eq', 'assert_ne', 'debug_assert',
                                       'todo', 'unimplemented', 'println', 'eprintln', 'print', 'eprint')
        if k >= 3 and _is(toks[k - 1], 'p', '(') and _is(toks[k - 2], 'id', 'expect') and _is(toks[k - 3], 'p', '.'):
            return True
        return False

    def _param_unmarked(self, caller, idx):
        """A parameter result to report: not a code parameter of the caller (whose own call sites are the
        sites), and of string type (an enum-typed `code` is mapped to its text elsewhere)."""
        if caller is None or idx in caller.code_params:
            return False
        ptype = caller.params[idx][1]
        return _stringish(ptype) or (caller.closure and not ptype)

    def record_field(self, results, src, k, fn, via, reuse):
        results = [r for r in results if r[0] not in ('relay', 'enum')]
        if fn is not None:
            results = [r for r in results if r[0] != 'param' or self._param_unmarked(fn, r[1])]
            results = [('unresolved', f'parameter {fn.params[r[1]][0]} of {fn.name}()') if r[0] == 'param' else r
                       for r in results]
        results = [r for r in results if not (r[0] == 'unresolved' and r[1] in ('a number',))]
        toks = src.toks
        brace = self._enclosing_brace(src, k)
        severity, message = None, None
        if brace is not None:
            end = src.pairs.get(brace, len(toks) - 1)
            for j in range(brace + 1, end):
                if _depth_of(toks, brace + 1, j) != 0:
                    continue
                if _is(toks[j], 'id', 'severity') and _is(toks[j + 1], 'p', ':'):
                    e = self._value_end(toks, j + 2)
                    lits = [x.val for x in toks[j + 2:e] if x.kind == 'str' and x.val in SEVERITIES]
                    if len(lits) == 1:
                        severity = lits[0]
                    elif fn is not None:
                        sv = self.resolve(toks[j + 2:e], fn, src, 0, j + 2)
                        vals = {r[1] for r in sv if r[0] == 'code' and r[1] in SEVERITIES}
                        severity = vals.pop() if len(vals) == 1 else ('varies' if vals else None)
                if _is(toks[j], 'id', 'message') and _is(toks[j + 1], 'p', ':'):
                    message = self.message_text(toks[j + 2:self._value_end(toks, j + 2)])
        self.record(results, src, k, via, severity, message, reuse)

    def _enclosing_brace(self, src, k):
        return src.brace_at[k] if src.brace_at[k] >= 0 else None

    def _struct_literal(self, src, k):
        brace = self._enclosing_brace(src, k)
        if brace is None or brace == 0:
            return False
        before = src.toks[brace - 1]
        if before.kind != 'id' or not before.val[:1].isupper():
            return False
        # not a pattern: `let Finding { code, .. } =`, `Finding { code, .. } =>`
        end = src.pairs.get(brace, len(src.toks) - 1)
        m = end + 1
        while m < len(src.toks) and _is(src.toks[m], 'p', ')'):
            m += 1
        after = src.toks[m] if m < len(src.toks) else None
        if after is not None and after.kind == 'p' and after.val in ('=', '=>', '|'):
            return False
        inner = [x.val for x in src.toks[brace + 1:end]]
        return '..' not in inner

    def _in_definition(self, src, k):
        """Whether `code:` at k is a field of a struct definition or a parameter of a fn."""
        brace = self._enclosing_brace(src, k)
        toks = src.toks
        paren = src.paren_at[k] if src.paren_at[k] >= 0 else None
        if paren is not None and (brace is None or paren > brace):
            j = paren - 1
            while j >= 0 and (toks[j].kind == 'id' or _is(toks[j], 'p', '>') or _is(toks[j], 'p', '<')):
                if _is(toks[j], 'id', 'fn'):
                    return True
                j -= 1
            return False
        if brace is None:
            return False
        j = brace - 1
        while j >= 0 and (toks[j].kind in ('id', 'life') or (toks[j].kind == 'p' and toks[j].val in '<>,:')):
            if toks[j].kind == 'id' and toks[j].val in ('struct', 'union', 'enum'):
                return True
            j -= 1
        return False

    def _is_type(self, value):
        if not value:
            return True
        v = [x.val for x in value]
        if v[0] == '&' and len(v) > 1 and (value[1].kind == 'life' or v[1] in ('str', 'mut', "'static")):
            return True
        if v[0] in ('impl', 'dyn', 'Option', 'Vec', 'Box', 'Cow', 'HashMap', 'BTreeMap') and len(v) > 1 and v[1] in ('<',) or v[0] in ('impl', 'dyn'):
            return True
        if len(v) == 1 and value[0].kind == 'id' and (v[0] in PRIMITIVES or v[0][:1].isupper() and not CONST_NAME.match(v[0])):
            return True
        return False

    def _value_end(self, toks, s, stop=None):
        e, d = s, 0
        while e < len(toks):
            x = toks[e]
            if x.kind == 'p' and x.val in OPEN:
                d += 1
            elif x.kind == 'p' and x.val in CLOSE:
                if d == 0:
                    break
                d -= 1
            elif d == 0 and x.kind == 'p' and x.val in ((',', ';') if stop is None else (stop,)):
                break
            e += 1
        return e

    def _compared(self, toks, k):
        prev = toks[k - 1] if k else None
        nxt = toks[k + 1] if k + 1 < len(toks) else None
        if _is(prev, 'p', '==') or _is(prev, 'p', '!=') or _is(nxt, 'p', '==') or _is(nxt, 'p', '!='):
            return True
        if _is(nxt, 'p', '=>') or (_is(nxt, 'p', '|') and not _is(prev, 'p', '|')):
            return True
        if _is(prev, 'p', '|') and _is(nxt, 'p', '=>'):
            return True
        if k >= 2 and _is(prev, 'p', '(') and toks[k - 2].kind == 'id' and toks[k - 2].val in (
                'contains', 'starts_with', 'ends_with', 'contains_key', 'find', 'strip_prefix', 'eq', 'matches'):
            return True
        if k >= 2 and _is(prev, 'p', '(') and _is(toks[k - 2], 'id', 'Some') and k >= 3 and \
                (_is(toks[k - 3], 'p', '==') or _is(toks[k - 3], 'p', '!=')):
            return True
        return False

    def message_text(self, toks):
        toks = [x for x in toks if not _is(x, 'p', '&')]
        if not toks:
            return None
        if toks[0].kind == 'str':
            return toks[0].val
        for j in range(len(toks) - 2):
            if toks[j].kind == 'id' and toks[j].val == 'format' and _is(toks[j + 1], 'p', '!') and toks[j + 3].kind == 'str':
                return toks[j + 3].val
        strs = [x.val for x in toks if x.kind == 'str']
        if strs:
            return ' '.join(strs)
        return _text(toks)[:200]

    def scan_unclaimed(self):
        self.unclaimed = {}
        for src in self.sources:
            toks = src.toks
            for k, t in enumerate(toks):
                if t.kind != 'str' or not CODE_SHAPE.match(t.val) or t.val in self.codes:
                    continue
                if (src.rel, k) in self.claimed or self._compared(toks, k):
                    continue
                self.unclaimed.setdefault(t.val, set()).add(f'{src.rel}:{t.line}')

    # -- output

    def table(self):
        codes = []
        for code in sorted(self.codes):
            sites = sorted(self.codes[code]['sites'].values(), key=lambda s: (s['file'], s['line'], s['via']))
            stated = {s['severity'] for s in sites if s['severity']}
            severity = stated.pop() if len(stated) == 1 else ('varies' if stated else 'not_stated')
            crates = sorted({self._crate_of(s['file']) for s in sites})
            messages = []
            for s in sites:
                if s['message'] and s['message'] not in messages:
                    messages.append(s['message'])
            codes.append({
                'code': code,
                'crates': crates,
                'severity': severity,
                'sites': [{k: v for k, v in s.items() if k != 'message'} for s in sites],
                'messages': [m[:400] for m in messages[:4]],
            })
        unresolved = [{'file': f, 'line': line, 'at': via, 'why': why}
                      for (f, line, via), why in sorted(self.unresolved.items())]
        unclaimed = [{'literal': lit, 'sites': sorted(sites)} for lit, sites in sorted(self.unclaimed.items())]
        return {'codes': codes, 'unresolved_sites': unresolved, 'unclaimed_literals': unclaimed}

    def _crate_of(self, rel):
        for src in self.all_sources:
            if src.rel == rel:
                return src.crate
        return rel.split('/src/')[0]


def _depth_of(toks, a, j):
    d = 0
    for x in toks[a:j]:
        if x.kind == 'p' and x.val in OPEN:
            d += 1
        elif x.kind == 'p' and x.val in CLOSE:
            d -= 1
    return d


# ---------------------------------------------------------------- the committed table

HEADER = {
    'about': ('Every diagnostic code a production call site in the solve crates can raise, read by '
              'tools/validation/diagnostic_codes.py, with a hand-made classification of each. The scanner '
              'writes code, crates, severity, sites and messages; classification, reason, unsure, candidates, '
              'named_by and named_by_statement are kept by --write. tests/test_diagnostic_code_table.py keeps the '
              'codes equal to the sources.'),
    'classes': {
        'absent_capability': 'the product refuses or withholds because it does not do something a user could '
                             'reasonably ask of it (a family, an analysis, a combination or an option not built, '
                             'or not built on this route)',
        'input_error': 'the document is incomplete, inconsistent or invalid and a user can correct it to obtain '
                       'the result',
        'numerical_standing': 'the solve ran and the code states how far the result can be relied on, or that it '
                              'did not converge or is ill-conditioned',
        'disclosure': 'a non-blocking statement of what was done or not applied, which refuses nothing',
        'internal': 'an invariant, a wire or version mismatch, something no document a user authors should reach',
    },
    'named_by': ('for an absent_capability code, the LIM-nnn entry of docs/RELEASE_LIMITATIONS_v1.0.md that names '
                 'the code, or "unnamed"; null for the other classes'),
    'named_by_statement': ('the product texts that name the code, as a list, or null when none does: '
                           '{"status": "in_product", "text", "where"} for text the product publishes today (a '
                           'formulation_basis limitation); {"status": "planned", "statement", "subject", "lands_with"} '
                           'for a per-document limitation statement not yet in the product, which names nothing '
                           'today. Planned statements may name codes of any class.'),
    'scope': list(SCOPE) + [f'{p} (re-used solve refusals only)' for p in REUSE_SCOPE],
}


def merge(mechanical, existing):
    old = {c['code']: c for c in (existing or {}).get('codes', [])}
    codes = []
    for record in mechanical['codes']:
        merged = dict(record)
        prior = old.get(record['code'], {})
        for field in HAND_FIELDS:
            merged[field] = prior.get(field, None if field != 'unsure' else False)
        if merged['named_by_statement'] is None:
            merged['named_by_statement'] = None
        codes.append(merged)
    return dict(HEADER, codes=codes, unresolved_sites=mechanical['unresolved_sites'],
                unclaimed_literals=mechanical['unclaimed_literals'])


def dumps(table):
    return json.dumps(table, indent=1, ensure_ascii=False, sort_keys=False) + '\n'


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.split('\n')[0])
    parser.add_argument('--project', type=Path, default=PROJECT)
    parser.add_argument('--json', action='store_true', help='print the mechanical table')
    parser.add_argument('--write', action='store_true', help='refresh the committed table, keeping hand fields')
    args = parser.parse_args(argv)
    scan = Scan(args.project.resolve()).run()
    table = scan.table()
    if args.json:
        sys.stdout.write(dumps(table))
        return 0
    if args.write:
        path = args.project / TABLE
        existing = json.loads(path.read_text(encoding='utf-8')) if path.is_file() else None
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(dumps(merge(table, existing)), encoding='utf-8')
    print(f'{len(table["codes"])} codes, {len(table["unresolved_sites"])} unresolved sites, '
          f'{len(table["unclaimed_literals"])} unclaimed literals, from {len(scan.sources)} files')
    return 0


if __name__ == '__main__':
    sys.exit(main())
