#!/usr/bin/env python3
"""The diagnostic codes the solve crates construct, read from the Rust sources.

Exit criterion 3 of `docs/RELEASE_TARGET_v1.0.md` needs every remaining refusal of a required
capability listed in `docs/RELEASE_LIMITATIONS_v1.0.md`. The codes are string literals at their
call sites, with no registry, so this scanner makes one: it reads the production sources of the
crates a solve runs through (`SCOPE`) and records each code a production call site can publish, with
the files and lines that raise it and the severity the site states. The unit is the code: the value
of the code field of a diagnostic, finding, refusal or reason record that production code of the
scanned crates constructs, whether or not the record carrying it is published; reason tokens printed
inside messages are not codes, and the table is not an inventory of every refusal a person can meet
(`HEADER['scope']`). `fixtures/diagnostics/
exact_route_codes.json` holds the result, with a hand-made classification of each code that
`--write` keeps; `tests/test_diagnostic_code_table.py` checks that the two agree over the producer
shapes listed in `PRODUCER_SHAPES` and the forms in `READ_FORMS`, each with a witness. That is
agreement, not completeness.

What counts as production code: the modules reachable from each crate's `src/lib.rs` (or
`src/main.rs`) through `mod` declarations, following `#[path]`, with every item or statement under a
test-only attribute removed and every file or directory whose name marks it a test (`tests`,
`*_tests`, `tests_*`) left out. A test-only attribute is `cfg(test)` or `cfg(all(test, ..))`; code
under `cfg(any(test, feature = ".."))` may be built in production and is read as production (the
conservative choice). The stripper removes only a statement form it recognises (`_skip_item`); a
test-only attribute on any other form is a strip failure, named by file and line, and the test fails
on it. `src/bin/` harness binaries are not crate roots here and are not read. `STRIPPER_FINDING`
records what repairing the stripper found about the tool as committed until then.

How a code is found. A code is the value that reaches a *code position*:
- the `code` field of a diagnostic record (a struct or enum variant with `code` and `message` or
  `severity`), in a literal or an assignment `.code = ...`; a JSON object's `"code"` key;
- an argument at a parameter that reaches such a position, to a fixed point through helpers;
- by shape or name only (where a code-shaped value counts and any other is listed as unclaimed): a
  text parameter named `code` whose sink is not traced, the `code` field of another struct, a
  `(CODE, message)` pair, `Err("CODE")`, and the prefix `CODE: ` of a message literal;
- typed producers: a method `code(&self)` returning text, and the `as_str` of an enum a diagnostic
  record declares as its code type.
The value is followed through `&`, `.into()`, `.to_string()` and the like, constants, `let`
bindings, `match` and `if` arms and block tails, calls (their return points, with a returned
parameter mapped back to the argument), closures, and tuples: tuple `let`s, `.0` projections, and
the tuple patterns of `for`, `let`, `if let`, `match` arms and closure parameters, followed to the
tuples their source yields (`vec![..]`, `push`, `extend`, `Some`/`Ok`/`Err`, iterator adapters,
`map` closures, a record field's constructions). A value relayed from another record's `.code` field
or a struct pattern's `code` is not a new code: every construction of that field is itself a site.

Every code position the scanner reads is resolved, pinned with its values, or listed as unresolved
with its reason; none is dropped. A part of a value yields no code only in a case of the closed list
`PROVEN_EMPTY`; any other expression that yields no values, or that the scanner cannot follow (built
at run time, behind a signature or a record type it cannot prove, too deep), is an unresolved site.
The test fails on an unresolved site unless the table pins it by hand (`pinned_sites`, with its values
and whole-function anchors the test checks) or lists it (`listed_unresolved_sites`, with the text of
its line and why its value is believed to relay a code built at a position the scanner reads, or that
this is not known). Every code-shaped literal (`UPPER_SNAKE` with an underscore) in
production code that the scanner does not find raised, and that is not only compared against, is
listed (`unclaimed_literals`), with any value at a position known only by shape or name.

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
            'core/model_operations/operation_applier/src/**',
            # each scanned crate's manifest: its path dependencies bound what a call or constant can reach
            'core/product_physics/Cargo.toml', 'core/solver/*/Cargo.toml', 'core/loads/*/Cargo.toml',
            'core/analysis/hanger_sizing/Cargo.toml', 'core/numerics/*/Cargo.toml',
            'core/model_operations/operation_applier/Cargo.toml')

# The fields a person fills in for each code; `--write` keeps them.
HAND_FIELDS = ('classification', 'reason', 'unsure', 'candidates', 'named_by', 'named_by_coverage',
               'named_by_statement')
CLASSES = ('absent_capability', 'input_error', 'numerical_standing', 'disclosure', 'internal')
SEVERITIES = {'blocking', 'warning', 'info', 'failure'}

CODE_SHAPE = re.compile(r'^[A-Z][A-Z0-9]*(?:[_-][A-Z0-9]+)+$')
CONST_NAME = re.compile(r'^[A-Z][A-Z0-9_]*$')
# A token that could be a code in any spelling: no whitespace, a letter first. Used where a code is known
# only by its place (a pair's first element, a whole error value): a code-shaped token there counts, any
# other is listed as unclaimed.
TOKEN_SHAPE = re.compile(r'^[A-Za-z][A-Za-z0-9_.:-]{1,79}$')
PREFIX_LITERAL = re.compile(r'^([A-Z][A-Z0-9]*(?:[_-][A-Z0-9]+)+): ')
PREFIX_FORMAT = re.compile(r'^\{([A-Z][A-Z0-9_]*)\}: ')
TEST_NAME = re.compile(r'(^tests?$|_tests?$|^tests_)')
STRINGISH = re.compile(r"^(&\s*('[a-z_]+\s*)?(mut\s*)?str|String|impl\s*Into\s*<\s*String\s*>|impl\s*AsRef\s*<\s*str\s*>|Cow\s*<.*str\s*>)$")
PRIMITIVES = {'u8', 'u16', 'u32', 'u64', 'u128', 'usize', 'i8', 'i16', 'i32', 'i64', 'i128', 'isize',
              'f32', 'f64', 'bool', 'char', 'str', 'String'}
WRAPPERS = {'String::from', 'Some', 'Cow::Borrowed', 'Cow::Owned', 'Ok', 'Err', 'Box::from'}
SAME_VALUE_METHODS = {'to_string', 'into', 'to_owned', 'clone', 'as_ref'}
MESSAGE_PARAMS = ('message', 'msg', 'detail', 'text', 'reason', 'remedy')
MAX_DEPTH = 12

# The closed list of cases in which a value at a code position, or a part of one, yields no code. They are
# the only places the resolver returns no values (as a ('none', case) result, dropped when recorded); each
# names why it cannot carry a code. Any other expression that yields no values is an unresolved site, which
# fails the test (`Scan.resolve` and `Scan.tuple_values` turn an empty result into one, and `record` does
# the same for a position left with nothing).
PROVEN_EMPTY = {
    'option_none': '`None`: an absent value carries no code',
    'diverges': ('`return`, `break`, `continue`, or a `panic!`, `unreachable!`, `todo!` or `unimplemented!` '
                 'arm or block tail: control leaves, so no value is produced there (a `return` is read as a '
                 'return point of its own function)'),
    'unit_block': 'a block with no tail expression: its value is `()`, which is not text',
    'other_side_of_wrapper': ('an `Ok` or `Some` value where the pattern binds `Err`, or the reverse: that arm does '
                              'not bind it'),
    'empty_collection': ('a collection built empty (`vec![]`, `[]`, or `new()`, `default()` or `with_capacity(..)` '
                         'of Vec, VecDeque, HashMap, BTreeMap, HashSet or BTreeSet): it holds nothing until a '
                         'mutation, which is read separately or makes the local unresolved'),
    'recursion': ('a recursive call met while its function\'s return points are being read: its values are those '
                  'return points, read once'),
    'relay': ('a read of the code field of a record whose type is proven to be built only by code positions the '
              'scanner reads: the type is known and defined in a scanned crate without Deserialize, or the '
              'crate can name no type with a code field defined elsewhere; the value is recorded where the '
              'record is built'),
    'flows_to_callers': ('a text parameter that is itself a code parameter: the value is recorded at each of its '
                         'callers\' arguments'),
    'enum_code_field': ('an enum value at a code field whose declared type is that enum: its text comes from the '
                        'enum\'s as_str, read as an enum code producer'),
}
EMPTY_COLLECTION_TYPES = {'Vec', 'VecDeque', 'HashMap', 'BTreeMap', 'HashSet', 'BTreeSet'}
CASE_CONVERSIONS = {'to_ascii_uppercase': str.upper, 'to_uppercase': str.upper,
                    'to_ascii_lowercase': str.lower, 'to_lowercase': str.lower}


def _none(case):
    """No values, for a case on the closed list."""
    assert case in PROVEN_EMPTY, case
    return [('none', case)]


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


def _attr_test_only(body):
    """Whether the attribute tokens `body` (inside `#[...]`) make what follows test-only: `cfg(test)`, or
    `cfg(all(...))` with `test` among the arguments of `all` and no `any` or `not` anywhere. Any other
    cfg that mentions test (`cfg(any(test, feature = ".."))`) may hold in a production build and is
    read as production."""
    vals = [(x.kind, x.val) for x in body]
    if vals == [('id', 'cfg'), ('p', '('), ('id', 'test'), ('p', ')')]:
        return True
    if vals[:4] != [('id', 'cfg'), ('p', '('), ('id', 'all'), ('p', '(')] or \
            any(v in (('id', 'any'), ('id', 'not')) for v in vals):
        return False
    return any(vals[i] == ('id', 'test') and vals[i - 1] in (('p', '('), ('p', ',')) and
               vals[i + 1] in (('p', ','), ('p', ')')) for i in range(4, len(vals) - 1))


def strip_cfg_test(toks, failures=None):
    """The tokens with every item or statement under a test-only attribute (`_attr_test_only`) removed.
    A test-only attribute on a form `_skip_item` does not recognise is not guessed at: (line, why) goes to
    `failures`, and only the attribute is removed, so the form is read as production."""
    out, k, n = [], 0, len(toks)
    if n >= 3 and _is(toks[0], 'p', '#') and _is(toks[1], 'p', '!') and _is(toks[2], 'p', '['):
        if _attr_test_only(toks[3:_match_forward(toks, 2)]):
            return []                    # `#![cfg(test)]`: the whole file is a test
    while k < n:
        t = toks[k]
        if _is(t, 'p', '#') and k + 1 < n and _is(toks[k + 1], 'p', '['):
            end = _match_forward(toks, k + 1)
            if _attr_test_only(toks[k + 2:end]):
                after, why = _skip_item(toks, end + 1)
                if why is None:
                    k = after
                    continue
                if failures is not None:
                    failures.append((t.line, why))
                k = end + 1
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
    """(the index after the item or statement that starts at `k`, after its attributes, None), or (None,
    why) when its form is not one recognised here: an item (`fn`, `const fn`, `mod`, `impl`, `struct`, `enum`,
    `trait`, `union`, `macro_rules!`, `extern`, `use`, `const`, `static`, `type`, with `pub`, `unsafe`, `async`); a
    field, variant, argument, element or match arm; a `let` or expression statement ending in `;`; a block
    (`{..}`, `unsafe {..}`, `async {..}`); `if`/`else` chains, `while`, `for`, `loop` and `match`, with or
    without a label; and a macro invocation, brace-delimited or ending in `;`."""
    n = len(toks)
    while k < n and _is(toks[k], 'p', '#') and k + 1 < n and _is(toks[k + 1], 'p', '['):
        k = _match_forward(toks, k + 1) + 1
    if k >= n:
        return None, 'a test-only attribute with nothing after it'
    first = toks[k]
    if first.kind == 'life' and k + 2 < n and _is(toks[k + 1], 'p', ':'):
        if toks[k + 2].kind == 'id' and toks[k + 2].val in ('loop', 'while', 'for'):
            return _skip_block_like(toks, k + 2)
        return None, 'a label before something that is not a loop'
    if first.kind == 'id' and first.val in ('if', 'while', 'for', 'loop', 'match'):
        return _skip_block_like(toks, k)
    if _is(first, 'p', '{'):
        return _match_forward(toks, k) + 1, None
    if first.kind == 'id' and first.val in ('unsafe', 'async'):
        q = k + 1 + (1 if k + 1 < n and _is(toks[k + 1], 'id', 'move') else 0)
        if q < n and _is(toks[q], 'p', '{'):
            return _match_forward(toks, q) + 1, None
    # a macro invocation: path!{..} ends at its brace; path!(..) and path![..] end at their `;`
    q = k
    while q + 2 < n and toks[q].kind == 'id' and _is(toks[q + 1], 'p', '::'):
        q += 2
    if toks[q].kind == 'id' and toks[q].val != 'macro_rules' and q + 2 < n and _is(toks[q + 1], 'p', '!') \
            and _is(toks[q + 2], 'p', '{'):
        e = _match_forward(toks, q + 2) + 1
        return (e + 1 if e < n and _is(toks[e], 'p', ';') else e), None
    const_item = _is(first, 'id', 'const') and k + 1 < n and toks[k + 1].kind == 'id' and \
        toks[k + 1].val in ('fn', 'unsafe', 'async', 'extern')          # `const fn`: an item, not a constant
    if first.kind == 'id' and first.val in ITEM_KEYWORDS and (first.val not in ('const', 'static', 'type', 'use')
                                                              or const_item):
        q = k
        if _is(toks[q], 'id', 'pub'):
            q += 1
            if q < n and _is(toks[q], 'p', '('):
                q = _match_forward(toks, q) + 1
        if q + 1 < n and toks[q].kind == 'id' and toks[q].val not in ITEM_KEYWORDS and _is(toks[q + 1], 'p', ':'):
            return _skip_generic(toks, k)       # a `pub` field
        depth = 0
        for j in range(k, n):
            t = toks[j]
            if t.kind != 'p':
                continue
            if t.val in '([':
                depth += 1
            elif t.val in ')]}':
                if depth == 0:
                    return None, 'an item the stripper cannot end'
                depth -= 1
            elif depth == 0 and t.val == ';':
                return j + 1, None
            elif depth == 0 and t.val == '{':
                return _match_forward(toks, j) + 1, None
        return None, 'an item with no end'
    return _skip_generic(toks, k)


def _skip_generic(toks, k):
    """A statement or list member that ends at `;` or `,` at depth 0, or before the bracket that closes
    its list; a block inside it must be followed by more of the expression or by that end, or the form is
    not recognised."""
    n, depth = len(toks), 0
    for j in range(k, n):
        t = toks[j]
        if t.kind != 'p':
            continue
        if t.val in OPEN:
            depth += 1
        elif t.val in CLOSE:
            if depth == 0:
                return j, None
            depth -= 1
            if depth == 0 and t.val == '}':
                nxt = toks[j + 1] if j + 1 < n else None
                if nxt is None:
                    return None, 'a statement with no end'
                if nxt.kind == 'p' and nxt.val in (';', ','):
                    return j + 2, None
                if nxt.kind == 'p' and nxt.val in (')', ']', '}'):
                    return j + 1, None
                if _continues(nxt):
                    continue
                return None, 'a statement ending in a block that the stripper does not recognise'
        elif depth == 0 and t.val in (';', ','):
            return j + 1, None
        elif depth == 0 and t.val == '=>' and j + 1 < n and _is(toks[j + 1], 'p', '{'):
            e = _match_forward(toks, j + 1) + 1    # a match arm with a block body
            return (e + 1 if e < n and _is(toks[e], 'p', ',') else e), None
    return None, 'a statement with no end'


def _skip_block_like(toks, k):
    """`if`/`else if`/`else`, `while`, `for`, `loop` or `match` at `k`: it ends at the brace closing its last
    block (its body found after its condition, pattern or scrutinee), or at the `;` after a method chain on
    it."""
    n, kw, j = len(toks), toks[k].val, k + 1
    while True:
        if kw == 'loop':
            body = j if j < n and _is(toks[j], 'p', '{') else None
        elif kw == 'for':
            q, depth = j, 0
            while q < n and not (depth == 0 and _is(toks[q], 'id', 'in')):
                if toks[q].kind == 'p' and toks[q].val in OPEN:
                    depth += 1
                elif toks[q].kind == 'p' and toks[q].val in CLOSE:
                    depth -= 1
                q += 1
            body = _body_brace(toks, q + 1) if q < n else None
        else:
            body = _body_brace(toks, j)
        if body is None:
            return None, f'a test-only {kw} whose body the stripper cannot find'
        e = _match_forward(toks, body)
        if kw == 'if' and e + 1 < n and _is(toks[e + 1], 'id', 'else'):
            if e + 2 < n and _is(toks[e + 2], 'id', 'if'):
                j = e + 3
                continue
            if e + 2 < n and _is(toks[e + 2], 'p', '{'):
                e = _match_forward(toks, e + 2)
            else:
                return None, 'an else the stripper cannot read'
        break
    nxt = toks[e + 1] if e + 1 < n else None
    if nxt is None:
        return n, None
    if _is(nxt, 'p', ';'):
        return e + 2, None
    if _continues(nxt) and not (nxt.kind == 'p' and nxt.val in (')', ']', '}', ',')):
        return _skip_generic(toks, e + 1)       # `match x {..}.method();`
    return e + 1, None


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
    failures = []
    toks = strip_cfg_test(tokenize(path.read_text(encoding='utf-8')), failures)
    found = [(rel, toks, failures)]
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
        for rel, toks, failures in _module_files(root, project, seen):
            sources.append(Source(crate, rel, toks))
            sources[-1].strip_failures = failures
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
        self.verified_params = set()
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
        self.diag_types = {}     # diagnostic record type -> its declared code type
        self.unresolved = {}     # (file, line, via) -> why
        self.unresolved_expr = {}    # (file, line, via) -> the text of the value at that code position
        self.strip_failures = sorted((src.rel, line, why) for src in self.all_sources
                                     for line, why in getattr(src, 'strip_failures', []))
        self.pin_problems = []
        self.unclaimed_values = {}   # a non-code-shaped value at a position known only by shape or name
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
        """[('code', value, src, line, via, token index) | ('param', index) | ('enum',) | ('none', case) |
        ('unresolved', why)] for the value of the expression `toks`. No values at all is never an answer: an
        expression form the resolver does not read is unresolved."""
        out = self._resolve(toks, fn, src, depth, base)
        return out or [('unresolved', f'no values from {_text(toks[:8]) or "an empty expression"}')]

    def _resolve(self, toks, fn, src, depth, base=0):
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
        # `x.to_ascii_uppercase()` and the like: the value of x, converted
        if len(seg) >= 5 and _is(seg[-1], 'p', ')') and _is(seg[-2], 'p', '(') and seg[-3].kind == 'id' and \
                seg[-3].val in CASE_CONVERSIONS and _is(seg[-4], 'p', '.'):
            convert = CASE_CONVERSIONS[seg[-3].val]
            return [('code', convert(r[1]), r[2], r[3], r[4] + [f'{seg[-3].val}()'], r[5]) if r[0] == 'code' else
                    (('unresolved', f'a parameter changed by {seg[-3].val}()') if r[0] == 'param' else r)
                    for r in self.resolve(seg[:-4], fn, src, depth + 1, base + a)]
        if len(seg) == 1 and seg[0].kind == 'str':
            return [('code', seg[0].val, src, seg[0].line, [], base + a)]
        if len(seg) == 1 and seg[0].kind == 'num':
            return [('unresolved', 'a number')]
        if len(seg) == 1 and _is(seg[0], 'id', 'None'):
            return _none('option_none')
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
        # a tuple element: `code.0` of a local bound to tuples
        if len(seg) == 3 and seg[0].kind == 'id' and _is(seg[1], 'p', '.') and seg[2].kind == 'num' and \
                seg[2].val.isdigit():
            return self.tuple_name(seg[0].val, int(seg[2].val), fn, src, depth + 1, base + a)
        # field access: x.code (relay of a record's code) or another field
        if len(seg) >= 3 and _is(seg[-2], 'p', '.') and seg[-1].kind == 'id' and \
                all(x.kind == 'id' or (x.kind == 'p' and x.val in ('.', '::')) for x in seg):
            if seg[-1].val in ('code', 'reason_code'):
                return self._relay(seg[:-2], fn, src, base + a)
            return [('unresolved', f'field .{seg[-1].val}')]
        # x.code.as_str(), code.as_str(): an enum-to-string producer when `code` is enum-typed
        if len(seg) >= 5 and [x.val for x in seg[-4:]] == ['.', 'as_str', '(', ')'] and \
                all(x.kind == 'id' or _is(x, 'p', '.') for x in seg[:-4]) and seg[-5].val == 'code':
            return self.resolve_enum_code(seg[:-4], fn, src, depth, base + a)
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

    def _param_type(self, fn, name):
        """The type name a parameter of `fn` (or of the function around a closure) is declared with:
        its last capitalized path segment that is not a wrapper, or None."""
        while fn is not None:
            for pname, ptype in fn.params:
                if pname == name and ptype:
                    names = [n for n in re.findall(r'[A-Za-z_][A-Za-z0-9_]*', ptype)
                             if n[:1].isupper() and n not in ('Option', 'Vec', 'Box', 'Result', 'Self')]
                    return names[-1] if names else None
            fn = self.enclosing_fn(fn.src, fn.start) if fn.closure else None
        return None

    def _impl_of(self, fn):
        while fn is not None and fn.closure:
            fn = self.enclosing_fn(fn.src, fn.start)
        return fn.impl_type if fn is not None else None

    def _collect_code_field_types(self):
        """The names of the structs and enum variants with a `code` field: those defined in the scanned
        crates without Deserialize (`code_types`), and, per crate, those it can name that are defined
        elsewhere or derive Deserialize (`outside_code_types`), whose values the scanner does not see built."""
        def types_in(sources):
            found = []
            for src in sources:
                toks = src.toks
                for k in range(len(toks) - 2):
                    if not ((_is(toks[k], 'id', 'struct') or _is(toks[k], 'id', 'enum')) and toks[k + 1].kind == 'id'):
                        continue
                    j = k + 2
                    while j < len(toks) and not (toks[j].kind == 'p' and toks[j].val in ('{', ';', '(')):
                        j += 1
                    if j >= len(toks) or not _is(toks[j], 'p', '{'):
                        continue
                    end = _match_forward(toks, j)
                    attrs, q = [], k - 1
                    while q >= 0 and (toks[q].kind == 'id' and toks[q].val in ('pub', 'crate', 'super', 'in') or
                                      _is(toks[q], 'p', '(') or _is(toks[q], 'p', ')')):
                        q -= 1
                    while q >= 1 and _is(toks[q], 'p', ']'):
                        o = q
                        while o > 0 and not (_is(toks[o], 'p', '[') and _is(toks[o - 1], 'p', '#')):
                            o -= 1
                        attrs += [x.val for x in toks[o + 1:q]]
                        q = o - 2
                    deser = 'Deserialize' in attrs
                    if _is(toks[k], 'id', 'struct'):
                        holders = [(toks[k + 1].val, j, end)]
                    else:                       # each variant with named fields
                        holders = []
                        for m in range(j + 1, end):
                            if toks[m].kind == 'id' and _is(toks[m + 1], 'p', '{') and _depth_of(toks, j + 1, m) == 0:
                                holders.append((toks[m].val, m + 1, _match_forward(toks, m + 1)))
                    for name, a, b in holders:
                        if any(_is(toks[m], 'id', 'code') and _is(toks[m + 1], 'p', ':') and toks[m - 1].kind == 'p'
                               and toks[m - 1].val in ('{', ',') or _is(toks[m - 1], 'id', 'pub') and _is(toks[m], 'id', 'code')
                               and _is(toks[m + 1], 'p', ':') for m in range(a + 1, b)):
                            found.append((src.crate, name, deser))
            return found
        scanned = types_in(self.all_sources)
        self.code_types = {name for _c, name, deser in scanned if not deser}
        scanned_crates = {s.crate for s in self.all_sources}
        others = sorted({d for deps in self.deps.values() for d in deps if d not in scanned_crates and
                         (self.project / d / 'Cargo.toml').is_file()})
        outside = types_in(load_sources(self.project, others)) if others else []
        self.outside_code_types = {}
        for crate in scanned_crates:
            visible = self.deps.get(crate, {crate})
            self.outside_code_types[crate] = {name for c, name, deser in scanned + outside
                                              if c in visible and (deser or c not in scanned_crates)}

    def _receiver_type(self, receiver, fn, src, at):
        """The type (last path segment) of the receiver tokens of a field read, when the scanner can tell it:
        `self` in an impl, a typed parameter, or a local bound to a struct literal or annotated with a type."""
        ids = [x for x in receiver if not (_is(x, 'p', '&') or _is(x, 'p', '*'))]
        if len(ids) != 1 or ids[0].kind != 'id':
            return None
        name = ids[0].val
        if name == 'self':
            return self._impl_of(fn)
        if fn is None:
            return None
        toks = src.toks
        iterated = self._iterated_type(src, fn, name, at)
        if iterated is not None:
            return iterated
        bound = self._binding_before(src, fn, name, at)
        near = self._pattern_binding(src, fn, name, at)
        if near is not None and near[1] is not None and (bound is None or near[0] > bound[1][1]):
            j, o, m = near                      # `Err(error) =>` in a match over a call
            if _is(toks[o], 'p', '(') and o >= 1 and toks[o - 1].kind == 'id' and \
                    toks[o - 1].val in ('Ok', 'Err', 'Some') and len(_split_commas(toks, o + 1, src.pairs[o])) == 1:
                source = self._pattern_source(src, fn, o, m)
                if source is not None:
                    (a, b), ctx = source
                    return self._wrapped_call_type(toks[a:b], toks[o - 1].val, ctx, src)
            return None
        if bound is not None:
            _let, (pa, pb), (va, vb) = bound
            pat = [x for x in toks[pa:pb] if not _is(x, 'id', 'mut')]
            if len(pat) == 4 and pat[0].kind == 'id' and pat[0].val in ('Ok', 'Err', 'Some') and \
                    _is(pat[1], 'p', '(') and _is(pat[2], 'id', name) and _is(pat[3], 'p', ')'):
                return self._wrapped_call_type(toks[va:vb], pat[0].val, fn, src)    # `if let Err(error) = f(..)`
            if len(pat) == 1:
                colon = next((w for w in range(pb, va) if _is(toks[w], 'p', ':')), None)
                value = toks[va:vb]
                brace = next((w for w, x in enumerate(value) if _is(x, 'p', '{')), None)
                if brace is not None and brace >= 1 and value[brace - 1].kind == 'id' and \
                        all(x.kind == 'id' or _is(x, 'p', '::') for x in value[:brace]):
                    return value[brace - 1].val
            return None
        return self._param_type(fn, name)

    def _iterated_type(self, src, fn, name, at):
        """The element type of the collection `name` is bound from when it is a closure's only parameter
        (`.map(|r| ..)` on an element method) or a `for` variable, over a named collection whose declared
        type is a slice or Vec of a type (a typed parameter, or a `let` with a type): that type's last path
        segment; otherwise None."""
        toks = src.toks
        lo = fn.body[0] if fn.body else 0
        for j in range(at - 1, lo, -1):
            if not _is(toks[j], 'id', name):
                continue
            source = None
            if _is(toks[j - 1], 'p', '|') and j + 1 < len(toks) and _is(toks[j + 1], 'p', '|'):
                call = src.paren_at[j - 1]
                if call < 2 or call != j - 2 or toks[call - 1].kind != 'id' or not _is(toks[call - 2], 'p', '.') or \
                        toks[call - 1].val not in self.ELEMENT_CLOSURE_METHODS:
                    return None
                if not (j < at < src.pairs.get(call, j)):
                    continue
                start = call - 2
                while start > 0 and (toks[start - 1].kind == 'id' or (toks[start - 1].kind == 'p' and
                                                                     toks[start - 1].val in ('.', '&', ')'))):
                    if _is(toks[start - 1], 'p', ')'):
                        opener = _match_back(toks, 0, start - 1)
                        if opener is None:
                            return None
                        start = opener
                    else:
                        start -= 1
                source = toks[start:call - 2]
            elif _is(toks[j - 1], 'id', 'for') and j + 1 < len(toks) and _is(toks[j + 1], 'id', 'in'):
                body = _body_brace(toks, j + 2)
                if body is None or not (body < at < src.pairs.get(body, body)):
                    continue
                source = toks[j + 2:body]
            else:
                continue
            seg = [x for x in source if not _is(x, 'p', '&')]
            while len(seg) >= 4 and _is(seg[-1], 'p', ')') and _is(seg[-2], 'p', '(') and seg[-3].kind == 'id' and \
                    seg[-3].val in ('iter', 'into_iter', 'drain') and _is(seg[-4], 'p', '.'):
                seg = seg[:-4]
            if len(seg) != 1 or seg[0].kind != 'id':
                return None
            collection = seg[0].val
            ptype = next((t for p, t in fn.params if p == collection), None)
            if ptype is None:
                bound = self._binding_before(src, fn, collection, j)
                if bound is not None:
                    _let, (pa, pb), (va, vb) = bound
                    colon = next((w for w in range(pa, va) if _is(toks[w], 'p', ':')), None)
                    ptype = _text(toks[colon + 1:va - 1]) if colon is not None else None
            if not ptype or not re.search(r'(\[|\bVec\s*<)', ptype):
                return None
            names = [n for n in re.findall(r'[A-Za-z_][A-Za-z0-9_]*', re.sub(r"'[a-z_]+", '', ptype))
                     if n[:1].isupper() and n not in ('Vec', 'Box', 'Option')]
            return names[-1] if names else None
        return None

    def _wrapped_call_type(self, value, wrapper, fn, src):
        """The type `Err(x)`, `Ok(x)` or `Some(x)` binds over `value` when value is a direct call `path(..)`
        or `x.method(..)` whose candidates all declare one return type `Result<T, E>` or `Option<T>`: the
        last path segment of E (Err), T (Ok, Some); otherwise None."""
        seg = [x for x in value if not _is(x, 'p', '&')]
        if not seg or not _is(seg[-1], 'p', ')'):
            return None
        opener = _match_back(seg, 0, len(seg) - 1)
        if opener is None or opener < 1 or seg[opener - 1].kind != 'id':
            return None
        name = seg[opener - 1].val
        args = _split_commas(seg, opener + 1, len(seg) - 1)
        visible = self.deps.get(src.crate, {src.crate})
        candidates = [f for f in self._callable(self.by_name.get(name, []), seg, opener - 1, fn)
                      if len(f.params) == len(args) and f.src.crate in visible]
        types = set()
        for f in candidates:
            ret = re.sub(r"\s+", '', f.ret or '')
            m = re.fullmatch(r'(?:[A-Za-z_:]*::)?(Result|Option)<(.*)>', ret)
            if not m:
                return None
            inner, depth, parts, cur = m.group(2), 0, [], ''
            for ch in inner:
                if ch in '<([':
                    depth += 1
                elif ch in '>)]':
                    depth -= 1
                if ch == ',' and depth == 0:
                    parts.append(cur)
                    cur = ''
                else:
                    cur += ch
            parts.append(cur)
            if m.group(1) == 'Option' and wrapper == 'Some' and len(parts) == 1:
                chosen = parts[0]
            elif m.group(1) == 'Result' and len(parts) == 2 and wrapper in ('Ok', 'Err'):
                chosen = parts[0] if wrapper == 'Ok' else parts[1]
            else:
                return None
            names = [n for n in re.findall(r'[A-Za-z_][A-Za-z0-9_]*', re.sub(r"'[a-z_]+", '', chosen))
                     if n[:1].isupper() and n not in ('Box', 'Vec', 'Option', 'Result')]
            if not names:
                return None
            types.add(names[-1])
        return types.pop() if len(types) == 1 else None

    def _relay_proven(self, type_name, src):
        if type_name is not None:
            return type_name in self.code_types and type_name not in self.outside_code_types.get(src.crate, set())
        return not self.outside_code_types.get(src.crate, set())

    def _relay(self, receiver, fn, src, at):
        """A read `<receiver>.code`: recorded where the record is built, when that is proven (see
        PROVEN_EMPTY['relay']); otherwise unresolved."""
        rtype = self._receiver_type(receiver, fn, src, at)
        if self._relay_proven(rtype, src):
            return _none('relay')
        return [('unresolved', f'the code field of {_text(receiver) or "a value"}, whose record type the scanner '
                               f'cannot prove is built only at code positions it reads')]

    def resolve_enum_code(self, path, fn, src, depth, at):
        """`<receiver>.code` or `code` read through `.as_str()`: the strings the `as_str` of its declared
        type returns. The type comes from the receiver (`self`, a typed parameter, or a parameter `code`
        of an enum type); when it is unknown, every diagnostic record's enum code type is taken and the
        result is marked as resolved by name."""
        ids = [x.val for x in path if x.kind == 'id']
        code_type, known = None, False
        if ids == ['code'] and fn is not None and self._param_type(fn, 'code'):
            code_type, known = self._param_type(fn, 'code'), True
        elif len(ids) == 2 and ids[0] == 'self':
            record = self._impl_of(fn)
            code_type, known = self.diag_types.get(record), record in self.diag_types
        elif len(ids) == 2 and fn is not None:
            record = self._param_type(fn, ids[0])
            if record in self.diag_types:
                code_type, known = self.diag_types[record], True
        visible = self.deps.get(src.crate, {src.crate})
        stringy = lambda t: t is None or bool(re.search(r'\b(str|String)\b', t))
        if known:
            types = [] if stringy(code_type) else [code_type.split('::')[-1].strip()]
        else:
            types = sorted({t.split('::')[-1].strip() for t in self.diag_types.values() if not stringy(t)})
        candidates = [f for f in self.by_name.get('as_str', []) if f.has_self and f.impl_type in types
                      and f.src.crate in visible]
        if not candidates:
            if known and stringy(code_type):
                return self._relay(path[:-2] if len(ids) == 2 else path, fn, src, at)
            return [('unresolved', 'a code read through as_str() of a type the scanner cannot find')]
        out = []
        for g in candidates:
            note = f'returned by {g.impl_type}::as_str()' + ('' if known else ' [by name]')
            for r in self.returned(g, depth + 1):
                out.append(('code', r[1], r[2], r[3], r[4] + [note], r[5]) if r[0] == 'code' else r)
        return out

    def resolve_name(self, name, fn, src, depth, at):
        if fn is not None:
            bound = self._binding_before(src, fn, name, at)
            bound_by = self._pattern_binding(src, fn, name, at)
            if bound_by is not None and bound_by[1] is None and bound is None:
                bound_by = None                 # a closure's own plain parameter: read as its parameter
            if bound_by is not None and (bound is None or bound_by[0] > bound[1][1]):
                return self.resolve_pattern(bound_by, fn, src, depth)       # the nearer binding shadows
            if bound is not None:
                return self.resolve_binding(bound, name, fn, src, depth)
            for idx, (pname, _type) in enumerate(fn.params):
                if pname == name:
                    return [('param', idx)]
            if fn.closure:                      # a name the closure captures from the function around it
                outer = self.enclosing_fn(src, fn.start)
                if outer is not None and outer is not fn:
                    return self.resolve_name(name, outer, src, depth + 1, fn.start)
        return [('unresolved', f'name {name} bound by a pattern, closure or loop')]

    def _pattern_binding(self, src, fn, name, at):
        """Where `name` is bound before `at` in `fn` by a tuple or struct pattern: (index of the name, index
        of the innermost group around it, index of the token after the pattern), or None."""
        toks = src.toks
        lo = fn.start if fn.closure else (fn.body[0] if fn.body else 0)
        for j in range(at - 1, lo, -1):
            if not _is(toks[j], 'id', name) or toks[j - 1].kind != 'p' or toks[j - 1].val not in ('(', ',', '{', '|'):
                continue
            if toks[j - 1].val == '|' and fn.closure and j - 1 <= fn.start + 4:
                return (j, None, None)              # a closure's own plain parameter
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
                if self._pattern_scope_holds(src, fn, o, m, at):
                    return (j, o, m)
        return None

    def _pattern_scope_holds(self, src, fn, o, m, at):
        """Whether `at` lies where the pattern ending before `m` binds: the body of its `for`, `if let` or
        `while let`, its match arm, or its closure's body; a plain `let` binds to the end of its block."""
        toks = src.toks
        t = toks[m]
        start = o
        while start > 0 and ((toks[start - 1].kind == 'id' and toks[start - 1].val not in PATTERN_STOPS) or
                             (toks[start - 1].kind == 'p' and toks[start - 1].val in ('::', '&', '('))):
            start -= 1
        kw = toks[start - 1] if start >= 1 else None
        if _is(t, 'id', 'in') or (_is(t, 'p', '=') and _is(kw, 'id', 'let') and start >= 2 and
                                  toks[start - 2].kind == 'id' and toks[start - 2].val in ('if', 'while')):
            e, d = m + 1, 0
            while e < len(toks):                # the body: the first `{` at depth 0 after the source
                x = toks[e]
                if x.kind == 'p' and x.val in ('(', '['):
                    d += 1
                elif x.kind == 'p' and x.val in (')', ']'):
                    d -= 1
                elif d == 0 and _is(x, 'p', '{') and e > m + 1 and _ends_operand(toks[e - 1]):
                    break
                elif d == 0 and _is(x, 'p', '{'):
                    e = src.pairs.get(e, e)
                e += 1
            end = src.pairs.get(e)
            return end is not None and e < at < end
        if _is(t, 'p', '=>'):
            e, d = m + 1, 0
            if e < len(toks) and _is(toks[e], 'p', '{'):
                return e < at < src.pairs.get(e, e)
            while e < len(toks):
                x = toks[e]
                if x.kind == 'p' and x.val in OPEN:
                    d += 1
                elif x.kind == 'p' and x.val in CLOSE:
                    if d == 0:
                        break
                    d -= 1
                elif d == 0 and _is(x, 'p', ','):
                    break
                e += 1
            return m < at < e
        if _is(t, 'p', '|'):
            e, d = m + 1, 0
            if e < len(toks) and _is(toks[e], 'p', '{'):
                return e < at < src.pairs.get(e, e)
            while e < len(toks):
                x = toks[e]
                if x.kind == 'p' and x.val in OPEN:
                    d += 1
                elif x.kind == 'p' and x.val in CLOSE:
                    if d == 0:
                        break
                    d -= 1
                elif d == 0 and x.kind == 'p' and x.val in (',', ';'):
                    break
                e += 1
            return m < at <= e
        return True

    def resolve_pattern(self, bound_by, fn, src, depth):
        """The values a name bound by a pattern can take. A struct or enum-variant pattern (`Refused {
        code, message }`) reads a record's `code` field, every construction of which is itself a code
        position: closed by construction. A tuple pattern is followed to the tuples its source yields
        (`tuple_values`); a source the scanner cannot follow is unresolved, never assumed."""
        toks = src.toks
        j, o, m = bound_by
        if o is None:
            return [('unresolved', 'a closure parameter whose argument the scanner cannot follow')]
        if toks[o].val == '{':
            field = toks[j - 2].val if j >= 2 and _is(toks[j - 1], 'p', ':') and toks[j - 2].kind == 'id' else toks[j].val
            type_name = toks[o - 1].val if o >= 1 and toks[o - 1].kind == 'id' else None
            if field not in ('code', 'reason_code'):
                return [('unresolved', f'{toks[j].val} bound from field {field} of a record pattern')]
            if type_name is not None and self._relay_proven(type_name, src):
                return _none('relay')
            return [('unresolved', f'the code field of a {type_name} pattern, which the scanner cannot prove is '
                                   f'built only at code positions it reads')]
        parts = _split_commas(toks, o + 1, src.pairs[o])
        index = next((p for p, (s, e) in enumerate(parts) if s <= j < e), None)
        if index is None or len(parts) < 2:
            return [('unresolved', 'a pattern the scanner cannot read')]
        source = self._pattern_source(src, fn, o, m)
        if source is None:
            return [('unresolved', 'a tuple pattern whose source the scanner cannot find')]
        (a, b), ctx = source
        wrap = toks[o - 2].val if o >= 2 and _is(toks[o - 1], 'p', '(') and toks[o - 2].kind == 'id' and \
            toks[o - 2].val in ('Some', 'Ok', 'Err') else None
        return self.tuple_values(toks[a:b], index, ctx, src, depth + 1, a, wrap)

    def _pattern_source(self, src, fn, o, m):
        """The expression a pattern ending before token `m` destructures, and the function to read it in:
        `for PAT in E {`, `let PAT = E;`, `if let PAT = E {`, a `match E {` arm, or a closure `|PAT|`
        passed to a method of the iterator chain E."""
        toks = src.toks
        t = toks[m]
        outer = self.enclosing_fn(src, o)

        def to_brace_or_semicolon(s):
            # the source ends at the body `{` (after an operand) or at `;`; a block inside it is skipped
            e, d = s, 0
            while e < len(toks):
                x = toks[e]
                if x.kind == 'p' and x.val in ('(', '['):
                    d += 1
                elif x.kind == 'p' and x.val in (')', ']'):
                    d -= 1
                elif d == 0 and _is(x, 'p', ';'):
                    return e
                elif d == 0 and _is(x, 'p', '{'):
                    if e > s and _ends_operand(toks[e - 1]):
                        return e
                    e = src.pairs.get(e, e)
                e += 1
            return e
        if _is(t, 'p', '='):
            start = o
            while start > 0 and not (toks[start - 1].kind == 'id' and toks[start - 1].val == 'let'):
                start -= 1
            conditional = start >= 2 and toks[start - 2].kind == 'id' and toks[start - 2].val in ('if', 'while')
            if not conditional:                 # a plain `let PAT = value;`: the value runs to its `;`
                e = self._value_end(toks, m + 1, stop=';')
                return (m + 1, e), outer
        if _is(t, 'id', 'in') or _is(t, 'p', '='):
            return (m + 1, to_brace_or_semicolon(m + 1)), outer
        if _is(t, 'p', '=>'):
            arms = src.brace_at[o]
            k = arms - 1
            while k > 0 and not _is(toks[k], 'id', 'match'):
                if toks[k].kind == 'p' and toks[k].val in (';', '{', '}') and src.brace_at[k] != src.brace_at[arms]:
                    return None
                k -= 1
            return ((k + 1, arms), outer) if k > 0 else None
        if _is(t, 'p', '|'):
            call = src.paren_at[o]                      # the `(` of the method the closure is passed to
            if call < 2 or toks[call - 1].kind != 'id' or not _is(toks[call - 2], 'p', '.') or \
                    toks[call - 1].val not in self.ELEMENT_CLOSURE_METHODS:
                return None
            start = call - 2                            # walk back over the receiver chain
            while start > 0:
                prev = toks[start - 1]
                if prev.kind == 'p' and prev.val in (')', ']'):
                    opener = next((p for p, c in src.pairs.items() if c == start - 1), None)
                    if opener is None:
                        return None
                    start = opener
                    continue
                if prev.kind == 'id' or (prev.kind == 'p' and prev.val in ('.', '::', '&')):
                    start -= 1
                    continue
                break
            receiver = self.enclosing_fn(src, start)
            return (start, call - 2), receiver
        return None

    # Methods that yield the receiver's elements unchanged (some as Some(element)); `chain` is not among them
    # (its argument adds elements) and a method not listed is read as a call, so unresolved unless it is one.
    TUPLE_ADAPTERS = {'iter', 'into_iter', 'iter_mut', 'as_ref', 'as_deref', 'cloned', 'copied', 'clone', 'drain',
                      'as_slice', 'to_vec', 'rev', 'filter', 'take', 'skip', 'flatten', 'find', 'first',
                      'last', 'next', 'unwrap_or_default', 'collect'}
    OPTION_ADAPTERS = {'find', 'first', 'last', 'next'}
    # Methods whose closure argument receives the receiver's element (or an Option's or Result's Ok value).
    ELEMENT_CLOSURE_METHODS = {'map', 'for_each', 'filter', 'filter_map', 'flat_map', 'find', 'find_map', 'any',
                               'all', 'inspect', 'position', 'take_while', 'skip_while', 'map_while', 'and_then',
                               'is_some_and', 'is_ok_and'}
    TUPLE_MAPPERS = {'map', 'filter_map', 'flat_map', 'and_then', 'find_map'}
    EMPTY_CONSTRUCTORS = {'new', 'default', 'with_capacity'}

    def tuple_values(self, toks, index, fn, src, depth, base, wrap=None):
        out = self._tuple_values(toks, index, fn, src, depth, base, wrap)
        return out or [('unresolved', f'no tuples from {_text(toks[:8]) or "an empty expression"}')]

    def _tuple_values(self, toks, index, fn, src, depth, base, wrap=None):
        """The values element `index` takes in the tuples the expression `toks` can yield, as `resolve`
        results: a tuple; `Some`/`Ok`/`Err` around one; the elements of `vec![..]` or `[..]`; `match`
        and `if` arms and block tails; a call's return points (its parameters mapped back to the call's
        arguments); a local's binding and its `push`, `extend`, `entry` and `insert`; through
        shape-preserving iterator adapters. Anything else is unresolved."""
        # structural steps (a wrapper, a tuple, a collection's elements, arms, tails) are bounded by the
        # expression; only a step to another binding, field or function spends depth
        if depth > MAX_DEPTH:
            return [('unresolved', 'resolution too deep')]
        a, b = 0, len(toks)
        changed = True
        while changed and a < b:
            changed = False
            while a < b and toks[a].kind == 'p' and toks[a].val in ('&', '*'):
                a += 1
                changed = True
            if a < b and _is(toks[a], 'id', 'mut'):
                a += 1
                changed = True
            if b - a >= 4 and _is(toks[b - 1], 'p', ')'):
                opener = _match_back(toks, a, b - 1)
                name_at = opener - 1 if opener is not None else None
                if name_at is not None and _is(toks[name_at], 'p', '>'):
                    # a turbofish: `collect::<Vec<_>>()`
                    d, q = 0, name_at
                    while q > a:
                        if _is(toks[q], 'p', '>'):
                            d += 1
                        elif _is(toks[q], 'p', '<'):
                            d -= 1
                            if d == 0:
                                break
                        q -= 1
                    name_at = q - 2 if q - 2 > a and _is(toks[q - 1], 'p', '::') else None
                if name_at is not None and name_at >= a + 1 and toks[name_at].kind == 'id' and \
                        toks[name_at].val in self.TUPLE_ADAPTERS and _is(toks[name_at - 1], 'p', '.'):
                    if toks[name_at].val in self.OPTION_ADAPTERS and wrap == 'Some':
                        wrap = None             # `find(..)`, `next()`: Some(element), read as the element
                    b = name_at - 1
                    changed = True
        seg = toks[a:b]
        if not seg:
            return [('unresolved', 'an empty tuple source')]
        first = seg[0]
        # no tuple: an absent value or a diverging arm
        if len(seg) == 1 and _is(first, 'id', 'None'):
            return _none('option_none')
        if first.kind == 'id' and first.val in ('return', 'break', 'continue'):
            return _none('diverges')
        if first.kind == 'id' and first.val in ('panic', 'unreachable', 'todo', 'unimplemented') and \
                len(seg) > 1 and _is(seg[1], 'p', '!'):
            return _none('diverges')
        whole = lambda k: _is(seg[k], 'p', '(') or _is(seg[k], 'p', '[')
        if first.kind == 'id' and first.val in ('Some', 'Ok', 'Err') and len(seg) > 2 and _is(seg[1], 'p', '(') \
                and _match_forward(seg, 1) == len(seg) - 1:
            if wrap is not None and first.val != wrap:
                return _none('other_side_of_wrapper')
            return self.tuple_values(seg[2:-1], index, fn, src, depth, base + a + 2)
        if wrap is not None and (_is(first, 'p', '(') or _is(first, 'p', '[') or _is(first, 'id', 'vec')):
            return [('unresolved', f'a bare value where the pattern binds {wrap}(..)')]
        if _is(first, 'p', '(') and _match_forward(seg, 0) == len(seg) - 1:
            parts = _split_commas(seg, 1, len(seg) - 1)
            if len(parts) == 1:
                return self.tuple_values(seg[1:-1], index, fn, src, depth, base + a + 1)
            if index >= len(parts):
                return [('unresolved', 'a tuple with too few elements')]
            s, e = parts[index]
            return self.resolve(seg[s:e], fn, src, depth + 1, base + a + s)
        if (_is(first, 'id', 'vec') and len(seg) > 3 and _is(seg[1], 'p', '!') and whole(2) and
                _match_forward(seg, 2) == len(seg) - 1) or \
                (_is(first, 'p', '[') and _match_forward(seg, 0) == len(seg) - 1):
            open_k = 2 if _is(first, 'id', 'vec') else 0
            elements = _split_commas(seg, open_k + 1, _match_forward(seg, open_k))
            if not elements:
                return _none('empty_collection')
            out = []
            for s, e in elements:
                if any(_is(x, 'p', ';') for x in seg[s:e]):
                    return [('unresolved', 'a repeated array element')]
                out += self.tuple_values(seg[s:e], index, fn, src, depth, base + a + s)
            return out
        if _is(first, 'id', 'match'):
            arms = self.arm_values(seg)
            if arms is None:
                return [('unresolved', 'a match whose arms the scanner cannot find')]
            out = []
            for s, e in arms:
                out += self.tuple_values(seg[s:e], index, fn, src, depth, base + a + s, wrap)
            return out
        if _is(first, 'id', 'if') and not _is(seg[-1], 'p', '}'):
            return [('unresolved', 'an if followed by more of the expression')]
        if _is(first, 'id', 'if') or (_is(first, 'p', '{') and _match_forward(seg, 0) == len(seg) - 1):
            out, k = [], 0
            while k < len(seg):
                if _is(seg[k], 'p', '{') and _depth_of(seg, 0, k) == 0:
                    end = _match_forward(seg, k)
                    tail, case = self._block_tail(seg, k)
                    out += (self.tuple_values(seg[tail:end], index, fn, src, depth, base + a + tail, wrap)
                            if tail is not None else _none(case))
                    k = end + 1
                    continue
                k += 1
            return out
        # a collection built empty: `Vec::new()`, `BTreeMap::<K, V>::new()`, `Vec::with_capacity(n)`
        if _is(seg[-1], 'p', ')') and seg and seg[0].kind == 'id' and seg[0].val in EMPTY_COLLECTION_TYPES:
            opener = _match_back(seg, 0, len(seg) - 1)
            if opener is not None and opener >= 2 and seg[opener - 1].kind == 'id' and \
                    seg[opener - 1].val in self.EMPTY_CONSTRUCTORS and _is(seg[opener - 2], 'p', '::') and \
                    (seg[opener - 1].val == 'with_capacity' or opener + 1 == len(seg) - 1) and \
                    all(x.kind == 'id' or (x.kind == 'p' and x.val in ('::', '<', '>', ',', '_', '&')) or
                        x.kind == 'life' for x in seg[1:opener - 1]):
                return _none('empty_collection')
        if all(x.kind == 'id' for x in seg[:1]) and len(seg) == 1:
            return self.tuple_name(seg[0].val, index, fn, src, depth, base + a, wrap)
        if len(seg) >= 3 and _is(seg[-2], 'p', '.') and seg[-1].kind == 'id' and \
                all(x.kind == 'id' or _is(x, 'p', '.') for x in seg):
            return self.tuple_field(seg[-1].val, index, src, depth, wrap)
        if _is(seg[-1], 'p', ')'):
            opener = _match_back(seg, 0, len(seg) - 1)
            if opener is not None and opener >= 2 and seg[opener - 1].kind == 'id' and \
                    seg[opener - 1].val in self.TUPLE_MAPPERS and _is(seg[opener - 2], 'p', '.'):
                # `.map(|d| (code, message))`: the tuples the closure's body yields
                c = opener + 1
                if c < len(seg) and _is(seg[c], 'id', 'move'):
                    c += 1
                if c < len(seg) and _is(seg[c], 'p', '|'):
                    bar = next((q for q in range(c + 1, len(seg) - 1) if _is(seg[q], 'p', '|')), None)
                    if bar is not None:
                        ctx = self.enclosing_fn(src, base + a + bar + 1) or fn
                        body = seg[bar + 1:-1]
                        out = self.tuple_values(body, index, ctx, src, depth, base + a + bar + 1, wrap)
                        for q in range(len(body)):      # a `return` in the closure returns from the closure
                            if _is(body[q], 'id', 'return'):
                                e = q + 1
                                while e < len(body) and not (_is(body[e], 'p', ';') and _depth_of(body, q + 1, e) == 0) \
                                        and _depth_of(body, q + 1, e) >= 0:
                                    e += 1
                                out += self.tuple_values(body[q + 1:e], index, ctx, src, depth,
                                                         base + a + bar + 1 + q + 1, wrap)
                        return out
                return [('unresolved', f'a tuple yielded through {seg[opener - 1].val}() by something not a closure')]
            if opener is not None and opener >= 1 and seg[opener - 1].kind == 'id':
                return self.tuple_call(seg, opener, index, fn, src, depth, base + a, wrap)
        return [('unresolved', 'a tuple source the scanner cannot follow')]

    def tuple_field(self, field, index, src, depth, wrap=None):
        """A tuple read from a record field `x.field`: every construction of that field in a struct
        literal (`field: value`, or the shorthand `field`) and every assignment `.field = value`, in the
        crates the reading crate can name."""
        visible = self.deps.get(src.crate, {src.crate})
        out, found = [], False
        for other in self.all_sources:
            if other.crate not in visible:
                continue
            toks = other.toks
            for k in range(1, len(toks) - 1):
                if not _is(toks[k], 'id', field):
                    continue
                prev, nxt = toks[k - 1], toks[k + 1]
                ctx = self.enclosing_fn(other, k)
                if _is(nxt, 'p', ':') and prev.kind == 'p' and prev.val in ('{', ',') and \
                        not self._in_definition(other, k) and self._struct_literal(other, k):
                    e = self._value_end(toks, k + 2)
                    if self._is_type(toks[k + 2:e]):
                        continue
                    found = True
                    out += self.tuple_values(toks[k + 2:e], index, ctx, other, depth + 1, k + 2, wrap)
                elif _is(nxt, 'p', '=') and _is(prev, 'p', '.'):
                    found = True
                    e = self._value_end(toks, k + 2, stop=';')
                    out += self.tuple_values(toks[k + 2:e], index, ctx, other, depth + 1, k + 2, wrap)
                elif nxt.kind == 'p' and nxt.val in (',', '}') and prev.kind == 'p' and prev.val in ('{', ',') and \
                        self._struct_literal(other, k):
                    found = True
                    out += self.tuple_values([toks[k]], index, ctx, other, depth + 1, k, wrap)
        return out if found else [('unresolved', f'a tuple in field {field} with no construction the scanner can find')]

    def _block_tail(self, seg, k):
        """The index where the tail expression of the block opening at seg[k] starts, or None when the
        block diverges or has no tail."""
        end = _match_forward(seg, k)
        last, d = k + 1, 0
        for j in range(k + 1, end):
            x = seg[j]
            if x.kind == 'p' and x.val in OPEN:
                d += 1
            elif x.kind == 'p' and x.val in CLOSE:
                d -= 1
                # a block-like statement (`if`, `match`, `for`, `while`, `loop`, a block) ends at its brace
                if d == 0 and x.val == '}' and j + 1 < end and seg[last].kind in ('id', 'p') and seg[last].val in BLOCK_STATEMENTS and \
                        not _continues(seg[j + 1]):
                    last = j + 1
            elif d == 0 and _is(x, 'p', ';'):
                last = j + 1
        if last >= end:
            return None, 'unit_block'
        if seg[last].kind == 'id' and seg[last].val in ('return', 'continue', 'break') or \
                (seg[last].kind == 'id' and seg[last].val in ('panic', 'unreachable', 'todo', 'unimplemented') and
                 last + 1 < end and _is(seg[last + 1], 'p', '!')):
            return None, 'diverges'
        return last, None

    def tuple_name(self, name, index, fn, src, depth, at, wrap=None):
        """A local's tuples: its `let` binding and every `push`, `extend`, `entry` and `insert` on it in
        the function (closures in it included); a parameter or capture the scanner cannot follow is
        unresolved."""
        if fn is None:
            return [('unresolved', f'a tuple source {name} outside a function')]
        toks = src.toks
        outer = fn
        while outer.closure and not self._binding_before(src, outer, name, at) and \
                not any(p == name for p, _t in outer.params):
            parent = self.enclosing_fn(src, outer.start)
            if parent is None:
                break
            outer = parent
        if any(p == name for p, _t in outer.params) and self._binding_before(src, outer, name, at) is None:
            return [('unresolved', f'a tuple source that is parameter {name} of {outer.name}()')]
        out = []
        bound = self._binding_before(src, outer, name, at)
        near = self._pattern_binding(src, outer, name, at)
        if near is not None and bound is not None and near[0] > bound[1][1]:
            bound = None                        # a pattern binding nearer than the let shadows it
        if bound is not None:
            _let, (pa, pb), (va, vb) = bound
            pat = [x for x in toks[pa:pb] if not _is(x, 'id', 'mut')]
            if len(pat) == 4 and pat[0].kind == 'id' and pat[0].val in ('Some', 'Ok', 'Err') and \
                    _is(pat[1], 'p', '(') and _is(pat[2], 'id', name) and _is(pat[3], 'p', ')'):
                # `if let Err(finding) = value`: what that wrapper holds
                out += self.tuple_values(toks[va:vb], index, outer, src, depth + 1, va, pat[0].val)
            elif len(pat) != 1:
                return [('unresolved', f'a tuple source {name} bound by a destructuring let')]
            else:
                out += self.tuple_values(toks[va:vb], index, outer, src, depth + 1, va, wrap)
        else:
            pb = self._pattern_binding(src, outer, name, at)
            if pb is not None and pb[1] is not None and toks[pb[1]].val == '(' and pb[1] >= 1 and \
                    toks[pb[1] - 1].kind == 'id' and toks[pb[1] - 1].val in ('Some', 'Ok', 'Err') and \
                    len(_split_commas(toks, pb[1] + 1, src.pairs[pb[1]])) == 1:
                # `Err(hardware) => ...` in a match: what that wrapper holds in the scrutinee
                source = self._pattern_source(src, outer, pb[1], pb[2])
                if source is None:
                    return [('unresolved', f'a tuple source {name} whose pattern source the scanner cannot find')]
                (sa, sb), ctx = source
                return self.tuple_values(toks[sa:sb], index, ctx, src, depth + 1, sa, toks[pb[1] - 1].val)
            if pb is not None:
                return [('unresolved', f'a tuple source {name} bound by a pattern the scanner does not follow')]
        if outer.body is None:
            return out
        for k in range(outer.body[0], outer.body[1] - 3):
            if not _is(toks[k], 'id', name) or (k > 0 and (_is(toks[k - 1], 'p', '.') or _is(toks[k - 1], 'p', '::'))):
                continue
            if bound is not None and k <= bound[1][1]:
                continue                        # the binding itself
            change = self._local_change(toks, k)
            if change is not None:
                out.append(('unresolved', f'a tuple local {name} {change}'))
                continue
            if not (_is(toks[k + 1], 'p', '.') and toks[k + 2].kind == 'id' and _is(toks[k + 3], 'p', '(')):
                continue
            method, close = toks[k + 2].val, src.pairs.get(k + 3)
            if close is None:
                continue
            args = _split_commas(toks, k + 4, close)
            ctx = self.enclosing_fn(src, k)
            if method == 'push' and len(args) == 1:
                out += self.tuple_values(toks[args[0][0]:args[0][1]], index, ctx, src, depth + 1, args[0][0])
            elif method == 'extend' and len(args) == 1:
                out += self.tuple_values(toks[args[0][0]:args[0][1]], index, ctx, src, depth + 1, args[0][0])
            elif method in ('entry', 'insert') and args and index == 0:
                out += self.resolve(toks[args[0][0]:args[0][1]], ctx, src, depth + 1, args[0][0])
            elif method == 'insert' and len(args) == 2 and index == 1:
                out += self.resolve(toks[args[1][0]:args[1][1]], ctx, src, depth + 1, args[1][0])
            elif method in ('entry',) and index == 1:
                out.append(('unresolved', f'a map value of {name} filled through entry()'))
        if bound is None and not out:
            return [('unresolved', f'a tuple source {name} the scanner cannot follow')]
        return out

    # Methods on a local that only read it or take elements away; with push, extend, entry and insert
    # (read as adding tuples), any other method on a tuple local makes it unresolved.
    LOCAL_READS = {'iter', 'into_iter', 'len', 'is_empty', 'contains', 'contains_key', 'get', 'first', 'last',
                   'clone', 'keys', 'values', 'to_vec', 'as_slice', 'join', 'concat', 'starts_with', 'ends_with',
                   'binary_search', 'to_owned', 'as_ref', 'is_some', 'is_none', 'is_ok', 'is_err', 'windows',
                   'chunks', 'retain', 'clear', 'truncate', 'sort', 'sort_by', 'sort_by_key', 'sort_unstable',
                   'sort_unstable_by', 'sort_unstable_by_key', 'dedup', 'dedup_by', 'dedup_by_key', 'reverse',
                   'pop', 'remove', 'swap_remove', 'drain', 'take', 'iter().any', 'unwrap_or_default'}
    LOCAL_ADDS = {'push', 'extend', 'entry', 'insert'}

    def _local_change(self, toks, k):
        """How the use of a local at toks[k] could change its tuples in a way the scanner does not read, or
        None: a method that is neither a read nor push, extend, entry or insert; a mutable borrow; an
        assignment, to it or to one of its elements."""
        nxt = toks[k + 1] if k + 1 < len(toks) else None
        if k >= 2 and _is(toks[k - 1], 'id', 'mut') and _is(toks[k - 2], 'p', '&'):
            return 'lent mutably'
        if _is(nxt, 'p', '=') or (nxt is not None and nxt.kind == 'p' and nxt.val in ('+=', '-=', '|=', '&=')):
            return 'reassigned'
        if _is(nxt, 'p', '['):
            close = _match_forward(toks, k + 1)
            if close + 1 < len(toks) and _is(toks[close + 1], 'p', '='):
                return 'changed through an index'
        if _is(nxt, 'p', '.') and k + 3 < len(toks) and toks[k + 2].kind == 'id' and \
                (_is(toks[k + 3], 'p', '(') or _is(toks[k + 3], 'p', '::')):
            method = toks[k + 2].val
            if method not in self.LOCAL_READS and method not in self.LOCAL_ADDS:
                return f'changed or read by {method}(), which the scanner does not read'
        return None

    def tuple_call(self, seg, open_k, index, fn, src, depth, base, wrap=None):
        """The tuples a call returns: each candidate's return points (its block tail and `return`
        expressions), with a returned parameter mapped back to the call's argument."""
        name = seg[open_k - 1].val
        method = open_k >= 2 and _is(seg[open_k - 2], 'p', '.')
        args = _split_commas(seg, open_k + 1, _match_forward(seg, open_k))
        named = self.by_name.get(name, [])
        local = self._local_closure(src, base + open_k - 1, name, named) if not method else None
        visible = self.deps.get(src.crate, {src.crate})
        candidates = [f for f in ([local] if local is not None else self._callable(named, seg, open_k - 1, fn))
                      if len(f.params) == len(args) and f.src.crate in visible and f.body]
        if not candidates:
            return [('unresolved', f'a tuple returned by {name}(), which the scanner cannot follow')]
        out = []
        for g in candidates:
            for r in self.tuple_returns(g, index, depth + 1, wrap):
                if r[0] == 'param':
                    s, e = args[r[1]]
                    out += self.resolve(seg[s:e], fn, src, depth + 1, base + s)
                else:
                    out.append(r)
        return out

    def tuple_returns(self, g, index, depth, wrap=None):
        toks = g.src.toks
        if g.expr_body is not None:
            s, e = g.expr_body
            return self.tuple_values(toks[s:e], index, g, g.src, depth, s, wrap)
        a, b = g.body
        out = []
        tail, case = self._block_tail(toks[a:b + 1], 0)
        out += (self.tuple_values(toks[a + tail:b], index, g, g.src, depth, a + tail, wrap)
                if tail is not None else _none(case))
        for k in range(a, b):
            if _is(toks[k], 'id', 'return') and self.enclosing_fn(g.src, k) is g:
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
                out += self.tuple_values(toks[k + 1:e], index, g, g.src, depth, k + 1, wrap)
        return out

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
                return self.tuple_values(value, names.index(name), fn, src, depth + 1, va)
        if any(_is(x, 'p', '{') for x in pat):     # `if let Refused { code, message } = ...`
            q = next((w for w, x in enumerate(pat) if _is(x, 'id', name)), None)
            field = pat[q - 2].val if q is not None and q >= 2 and _is(pat[q - 1], 'p', ':') else name
            brace = next(w for w, x in enumerate(pat) if _is(x, 'p', '{'))
            type_name = pat[brace - 1].val if brace >= 1 and pat[brace - 1].kind == 'id' else None
            if field in ('code', 'reason_code') and type_name is not None and self._relay_proven(type_name, src):
                return _none('relay')
            return [('unresolved', f'{name} bound from field {field} of a {type_name} pattern, which the scanner '
                                   f'cannot prove is built only at code positions it reads')]
        if len(pat) >= 4 and pat[0].kind == 'id' and pat[0].val in ('Some', 'Ok', 'Err') and _is(pat[1], 'p', '(') \
                and _is(pat[2], 'p', '(') and _is(pat[-1], 'p', ')'):
            inner = pat[2:-1]                   # `if let Some((code, message)) = value`
            names = []
            for s, e in _split_commas(inner, 1, len(inner) - 1):
                ids = [x.val for x in inner[s:e] if x.kind == 'id']
                names.append(ids[-1] if ids else None)
            if name in names:
                # only what that arm of the Result or Option holds (`Err((..))` reads no `Ok` tuple)
                return self.tuple_values(value, names.index(name), fn, src, depth + 1, va, pat[0].val)
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
        """[(start, end)] of the arm values of the `match` expression `seg`; None when its arms cannot be found
        or more of the expression follows them (`match x {..}.into_iter()`)."""
        brace = _body_brace(seg, 1)
        if brace is None:
            return None
        end = _match_forward(seg, brace)
        if end != len(seg) - 1:
            return None
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
        arms = self.arm_values(seg)
        if arms is None:
            return [('unresolved', 'a match whose arms the scanner cannot find')]
        out = []
        for s, e in arms:
            out += self.resolve(seg[s:e], fn, src, depth + 1, base + s)
        return out

    def resolve_block(self, seg, k, fn, src, depth, base):
        """The value of the block opening at seg[k]: its tail expression, after the last statement."""
        end = _match_forward(seg, k)
        tail, case = self._block_tail(seg, k)
        if tail is None:
            return _none(case)
        return self.resolve(seg[tail:end], fn, src, depth + 1, base + tail)

    def resolve_if(self, seg, fn, src, depth, base):
        """The values of the branches of `if c {..} else if c {..} else {..}`: each body block, found
        after its condition (a block expression inside a condition is not a branch)."""
        out, j = [], 1
        while j < len(seg):
            body = _body_brace(seg, j)
            if body is None:
                return [('unresolved', 'an if whose branches the scanner cannot find')]
            end = _match_forward(seg, body)
            out += self.resolve_block(seg, body, fn, src, depth, base)
            j = end + 1
            if j < len(seg) and _is(seg[j], 'id', 'else'):
                if j + 1 < len(seg) and _is(seg[j + 1], 'id', 'if'):
                    j += 2
                    continue
                if j + 1 < len(seg) and _is(seg[j + 1], 'p', '{'):
                    out += self.resolve_block(seg, j + 1, fn, src, depth, base)
                    j = _match_forward(seg, j + 1) + 1
            if j < len(seg):
                return [('unresolved', 'an if followed by more of the expression')]
            break
        return out

    def resolve_call(self, seg, open_k, fn, src, depth, base):
        name = seg[open_k - 1].val
        method = open_k >= 2 and _is(seg[open_k - 2], 'p', '.')
        close = _match_forward(seg, open_k)
        args = _split_commas(seg, open_k + 1, close)
        named = self.by_name.get(name, [])
        local = self._local_closure(src, base + open_k - 1, name, named) if not method else None
        candidates = [f for f in ([local] if local is not None else self._callable(named, seg, open_k - 1, fn))
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
        by_name = False
        if method:
            receiver = [x for x in seg[:open_k - 2] if not _is(x, 'p', '&') and not _is(x, 'p', '*')]
            rtype = None
            if len(receiver) == 1 and _is(receiver[0], 'id', 'self'):
                rtype = self._impl_of(fn)
            elif len(receiver) == 1 and receiver[0].kind == 'id':
                rtype = self._param_type(fn, receiver[0].val)
            narrowed = [g for g in candidates if rtype is not None and g.impl_type == rtype]
            if narrowed:
                candidates = narrowed
            else:
                by_name = True               # the receiver's type is not known: resolved by the method's name
        out = []
        for g in candidates:
            arms = self._arms_for_argument(g, seg, args)
            note = f'returned by {g.impl_type + "::" if g.impl_type else ""}{g.name}()' + \
                (' [by name]' if by_name else '') + (' [any arm]' if arms == 'any' else '')
            for r in (arms if isinstance(arms, list) else self.returned(g, depth + 1)):
                if r[0] == 'param':
                    s, e = args[r[1]]
                    out += self.resolve(seg[s:e], fn, src, depth + 1, base + s)
                elif r[0] == 'code':
                    out.append(('code', r[1], r[2], r[3], r[4] + [note], r[5]))
                else:
                    out.append(r)
        codes = [r for r in out if r[0] in ('code', 'param', 'none')]
        if not codes:
            return [('unresolved', f'call {name}() returns no resolvable code')]
        # every return point counts: a computed text beside the resolved codes stays unresolved
        return out

    def _arms_for_argument(self, g, seg, args):
        """For a mapping function whose body is `match <parameter> { ... }` (as `skew_code`): the
        resolved values of the arms an enum-variant argument selects; 'any' when the argument is not a
        variant, so every arm is taken; None when `g` is no such mapping."""
        if g.body is None or g.expr_body is not None or len(args) != len(g.params) or not g.params:
            return None
        toks = g.src.toks
        a, b = g.body
        body = toks[a + 1:b]
        if len(body) < 3 or not _is(body[0], 'id', 'match') or body[1].kind != 'id':
            return None
        names = [p for p, _t in g.params]
        if body[1].val not in names:
            return None
        idx = names.index(body[1].val)
        s, e = args[idx]
        arg = [x for x in seg[s:e] if not _is(x, 'p', '&')]
        if not (len(arg) >= 3 and all(x.kind == 'id' or _is(x, 'p', '::') for x in arg) and arg[-1].val[:1].isupper()):
            return 'any'
        variant = arg[-1].val
        out = []
        brace = next((j for j in range(len(body)) if _is(body[j], 'p', '{')), None)
        if brace is None:
            return None
        for vs, ve in self.arm_values(body) or []:
            # the arm's pattern: the tokens between the previous arm (or the brace) and `=>`
            j = vs - 2
            while j > brace and not (_is(body[j], 'p', ',') and _depth_of(body, brace + 1, j) == 0) and \
                    not _is(body[j], 'p', '}'):
                j -= 1
            pattern = [x.val for x in body[j + 1:vs - 1]]
            if variant in pattern or pattern == ['_']:
                out += self.resolve(body[vs:ve], g, g.src, 1, a + 1 + vs)
                if variant in pattern and 'if' not in pattern:
                    break
        return out or 'any'

    def returned(self, g, depth):
        if g.body is None:
            return [('unresolved', f'{g.name}() has no body the scanner can read')]
        key = ('returned', depth > 4)
        if key in g.local_cache:
            return g.local_cache[key]
        g.local_cache[key] = _none('recursion')         # recursion guard
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
        g.local_cache[key] = out
        return out

    # -- code positions

    def collect_diagnostic_types(self):
        """The diagnostic record types: each struct, or enum variant with named fields, that has a
        `code` field and a `message` or `severity` field; with each one's declared `code` type."""
        self.diag_types = {}
        for src in self.all_sources:
            toks = src.toks
            enum_bodies = []
            for k in range(len(toks) - 2):
                if _is(toks[k], 'id', 'enum') and toks[k + 1].kind == 'id':
                    j = k + 2
                    while j < len(toks) and not _is(toks[j], 'p', '{') and not _is(toks[j], 'p', ';'):
                        j += 1
                    if j < len(toks) and _is(toks[j], 'p', '{'):
                        enum_bodies.append((j, src.pairs.get(j, j)))
            for k in range(len(toks) - 2):
                t = toks[k]
                named = None
                if _is(t, 'id', 'struct') and toks[k + 1].kind == 'id':
                    named, j = toks[k + 1].val, k + 2
                    while j < len(toks) and not (toks[j].kind == 'p' and toks[j].val in ('{', ';', '(')):
                        j += 1
                elif t.kind == 'id' and t.val[:1].isupper() and _is(toks[k + 1], 'p', '{') and \
                        any(a < k < b and src.brace_at[k] == a for a, b in enum_bodies):
                    named, j = t.val, k + 1
                if named is None or j >= len(toks) or not _is(toks[j], 'p', '{'):
                    continue
                end = src.pairs.get(j)
                if end is None:
                    continue
                fields = {}
                for m in range(j + 1, end - 1):
                    if toks[m].kind == 'id' and _is(toks[m + 1], 'p', ':') and src.brace_at[m] == j:
                        fields[toks[m].val] = _text(toks[m + 2:self._value_end(toks, m + 2)])
                if 'code' in fields and ('message' in fields or 'severity' in fields):
                    self.diag_types[named] = fields['code']

    def _literal_type(self, src, brace, fn):
        """The type a struct literal opening at `brace` builds (its last path segment; `Self` is the
        enclosing impl's type), or None."""
        if brace is None or brace == 0 or src.toks[brace - 1].kind != 'id':
            return None
        name = src.toks[brace - 1].val
        if name == 'Self':
            outer = fn
            while outer is not None and outer.closure:
                outer = self.enclosing_fn(src, outer.start)
            return outer.impl_type if outer is not None else None
        return name

    def sinks(self, src):
        """Each verified diagnostic code position in `src`: (index of the position, value token range,
        enclosing fn, via). A `code` field (or shorthand) of a diagnostic record literal, the `"code"`
        key of a `json!` object that also has `"message"` or `"severity"`, or an assignment `.code =`."""
        toks, out = src.toks, []
        for k, t in enumerate(toks):
            if k == 0:
                continue
            prev, nxt = toks[k - 1], toks[k + 1] if k + 1 < len(toks) else None
            if _is(t, 'id', 'code') and prev.kind == 'p' and prev.val in ('{', ',') and nxt is not None:
                fn = self.enclosing_fn(src, k)
                if self._literal_type(src, self._enclosing_brace(src, k), fn) not in self.diag_types:
                    continue
                if _is(nxt, 'p', ':') and not self._in_definition(src, k) and self._struct_literal(src, k):
                    value = (k + 2, self._value_end(toks, k + 2))
                    if not self._is_type(toks[value[0]:value[1]]):
                        out.append((k, value, fn, 'field code'))
                elif nxt.kind == 'p' and nxt.val in (',', '}') and self._struct_literal(src, k):
                    out.append((k, (k, k + 1), fn, 'field code'))
            elif _is(t, 'id', 'code') and _is(prev, 'p', '.') and _is(nxt, 'p', '='):
                out.append((k, (k + 2, self._value_end(toks, k + 2, stop=';')), self.enclosing_fn(src, k),
                            'assignment .code'))
            elif t.kind == 'str' and t.val == 'code' and _is(nxt, 'p', ':') and prev.kind == 'p' and prev.val in ('{', ','):
                out.append((k, (k + 2, self._value_end(toks, k + 2)), self.enclosing_fn(src, k), 'json code'))
        return out

    def callables(self):
        """Mark each function's code, severity and message parameters, to a fixed point. A code
        parameter is *verified* when its value reaches a verified diagnostic position (`sinks`), directly
        or through another verified code parameter, whatever the parameter is called; a stringish
        parameter named `code` whose sink is not traced is a code parameter by name only."""
        for f in self.fns:
            f.verified_params = set()
        for src in self.all_sources:
            for k, (a, b), fn, _via in self.sinks(src):
                if fn is None:
                    continue
                seg = [x for x in src.toks[a:b] if not _is(x, 'p', '&') and not _is(x, 'p', '*')]
                while len(seg) >= 4 and seg[-3].kind == 'id' and seg[-3].val in SAME_VALUE_METHODS:
                    seg = seg[:-4]
                if len(seg) == 1 and seg[0].kind == 'id':
                    for idx, (pname, _t) in enumerate(fn.params):
                        if pname == seg[0].val and self._binding_before(src, fn, pname, a) is None:
                            fn.verified_params.add(idx)
        changed = True
        while changed:
            changed = False
            for src in self.all_sources:
                for k, args, targets, caller in self.calls(src, verified=True):
                    if caller is None:
                        continue
                    for g in targets:
                        for idx in g.verified_params:
                            if idx >= len(args):
                                continue
                            s, e = args[idx]
                            seg = [x for x in src.toks[s:e] if not _is(x, 'p', '&')]
                            while len(seg) >= 4 and seg[-3].kind == 'id' and seg[-3].val in SAME_VALUE_METHODS:
                                seg = seg[:-4]
                            if len(seg) == 1 and seg[0].kind == 'id':
                                for pidx, (pname, _t) in enumerate(caller.params):
                                    if pname == seg[0].val and pidx not in caller.verified_params and \
                                            self._binding_before(src, caller, pname, s) is None:
                                        caller.verified_params.add(pidx)
                                        changed = True
        for f in self.fns:
            f.code_params |= f.verified_params
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

    def calls(self, src, verified=False):
        """(index of the name, [argument ranges], [functions it may call], enclosing fn) of each call
        in `src` to a function with a code, severity or message parameter (with `verified`, to a
        function with a verified code parameter)."""
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
            local = self._local_closure(src, k, t.val, fns)
            if local is not None:            # a closure in scope shadows any function of its name
                fns = [local]
            visible = self.deps.get(src.crate, {src.crate})
            targets = [f for f in self._callable(fns, toks, k, caller) if len(f.params) == len(args)
                       and f.src.crate in visible
                       and ((f.verified_params) if verified else
                            (f.code_params or f.severity_params or f.message_params))]
            if not targets:
                continue
            same = [f for f in targets if f.src.crate == src.crate] or targets
            out.append((k, args, same, caller))
        return out

    def _local_closure(self, src, k, name, fns):
        """The closure named `name` bound by a `let` before `k` in a function that encloses `k`, or None."""
        if k >= 1 and (_is(src.toks[k - 1], 'p', '.') or _is(src.toks[k - 1], 'p', '::')):
            return None
        best = None
        for f in fns:
            if not f.closure or f.src is not src or f.start >= k:
                continue
            outer = self.enclosing_fn(src, f.start)
            if outer is None or outer.body is None or not (outer.body[0] < k < outer.body[1]):
                continue
            if best is None or f.start > best.start:
                best = f
        return best

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
        self._collect_code_field_types()
        self.collect_diagnostic_types()
        self.callables()
        for f in self.fns:
            self.fix_severity(f)
        for src in self.sources:
            self.scan_source(src, reuse=False)
        for src in self.reuse:
            self.scan_source(src, reuse=True)
        self.scan_typed_producers()
        self.scan_unclaimed()
        return self

    def scan_typed_producers(self):
        """Typed code producers, each a verified diagnostic position by its type: a method `code(&self)`
        returning text (a typed error's code, read by callers in and outside the scan, the desktop
        included), and the `as_str` of an enum that a diagnostic record declares as its `code` type."""
        enum_code_types = {t.split('::')[-1].strip() for t in self.diag_types.values()
                           if not re.search(r'\b(str|String)\b', t)}
        for g in self.fns:
            if g.src not in self.sources or not g.has_self or g.params or not g.body or not _returns_text(g.ret):
                continue
            if g.name == 'code' or g.name.endswith('_code'):
                via = f'{g.impl_type}::{g.name}() (typed producer)'
            elif g.name == 'as_str' and g.impl_type in enum_code_types:
                via = f'{g.impl_type}::as_str() (enum code producer)'
            else:
                continue
            results = self._settle(self.returned(g, 1), g)
            self.record(results, g.src, g.start, via, None, None, False, verified=True, expr=f'{g.name}()')

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

    def record(self, results, src, k, via, severity, message, reuse, verified=False, prefix=False, expr=None):
        """Record each resolved code at its site. At a verified diagnostic position a value counts
        whatever its spelling. At a position known only by a shape or a name (a parameter called
        `code` whose sink is not traced, a struct not known as a diagnostic record, a pair) a
        code-shaped value counts and any other value is listed as unclaimed, never dropped. A message
        prefix is a shape by nature: there only a code-shaped value is a code."""
        toks = src.toks
        expr_line = toks[k].line
        if not results:
            results = [('unresolved', 'no values at a code position')]
        for r in results:
            if r[0] == 'none':
                assert r[1] in PROVEN_EMPTY, r
                continue
            if r[0] in ('param', 'enum', 'relay'):
                r = ('unresolved', f'an unsettled {r[0]} result')     # never expected: _settle runs first
            if r[0] == 'code' and not verified and not CODE_SHAPE.match(r[1]):
                if not prefix and not reuse and r[1].strip() and r[1] not in SEVERITIES:
                    self.unclaimed_values.setdefault(r[1], set()).add(f'{src.rel}:{expr_line}')
                continue
            if r[0] == 'code' and not r[1].strip():
                r = ('unresolved', 'an empty text')
            if r[0] == 'code':
                value, csrc, line, chain, tok_index = r[1], r[2], r[3], r[4], r[5]
                if reuse and not chain:
                    continue            # the applier's own literal codes are not solve refusals
                if reuse and csrc.crate in REUSE_SCOPE:
                    continue
                self.claimed.add((csrc.rel, tok_index))
                entry = self.codes.setdefault(value, {'sites': {}})
                site_key = (src.rel, expr_line)
                by_name = any('[by name]' in c for c in chain)
                any_arm = any('[any arm]' in c for c in chain)
                site = entry['sites'].setdefault(site_key, {'file': src.rel, 'line': expr_line, 'via': via,
                                                            'severity': severity, 'message': message,
                                                            'by_name': by_name, 'any_arm': any_arm})
                site['by_name'] = site['by_name'] and by_name
                site['any_arm'] = site['any_arm'] and any_arm
                if chain:
                    site.setdefault('through', [])
                    note = f'{" <- ".join(chain)} ({csrc.rel}:{line})'
                    if note not in site['through']:
                        site['through'].append(note)
                        site['through'].sort()
            elif r[0] == 'unresolved':
                self.unresolved[(src.rel, expr_line, via)] = r[1]
                self.unresolved_expr[(src.rel, expr_line, via)] = expr

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
                types = [h.params[idx][1] for h in coded if idx < len(h.params)]
                if types and all(t and not _stringish(t) and t[:1].isupper() for t in types):
                    continue                # an enum-typed parameter: settled where the callee uses it (_param_result)
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
                verified = any(idx in h.verified_params for h in coded)
                results = self._settle(results, caller)
                self.record(results, src, k, f'{g.name}()', severity, message, reuse, verified=verified,
                            expr=_text(toks[s:e]))
        # verified diagnostic positions: a diagnostic record's `code` field, a json diagnostic's
        # "code", an assignment `.code =`
        verified_at = set()
        for k, (a, b), fn, via in self.sinks(src):
            verified_at.add(k)
            results = self.resolve(src.toks[a:b], fn, src, 0, a)
            record_type = self._literal_type(src, self._enclosing_brace(src, k), fn) if via == 'field code' else None
            text_code = record_type is None or bool(re.search(r'\b(str|String)\b', self.diag_types.get(record_type, 'str')))
            # an enum value at a text code (Display, Into) is unresolved; at an enum-typed code field it is
            # read through that enum's as_str
            self.record_field(results, src, k, fn, via, reuse, verified=True, enum_field=not text_code,
                              expr=_text(src.toks[a:b]))
        # other `code` fields, of a struct not known as a diagnostic record: code-shaped values count,
        # any other value is listed as unclaimed
        for k, t in enumerate(toks):
            if not _is(t, 'id', 'code') or k == 0 or k in verified_at:
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
                self.record_field(results, src, k, fn, 'field code', reuse, verified=False, expr=_text(value))
            elif nxt is not None and prev.kind == 'p' and prev.val in ('{', ',') and nxt.kind == 'p' and \
                    nxt.val in (',', '}') and self._struct_literal(src, k):
                fn = self.enclosing_fn(src, k)
                results = self.resolve([t], fn, src, 0, k)
                self.record_field(results, src, k, fn, 'field code', reuse, verified=False, expr='code')
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
                self.record(results, src, k, 'message prefix', None, self.message_text([t]), reuse, prefix=True,
                            expr=t.val[:80])

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
                if not (len(first) == 1 and first[0].kind == 'str' and TOKEN_SHAPE.match(first[0].val)) and \
                        not (all(x.kind == 'id' or _is(x, 'p', '::') for x in first) and first and
                             CONST_NAME.match(first[-1].val) and len(first[-1].val) > 2):
                    continue
                message = toks[parts[at + 1][0]:parts[at + 1][1]]
                if not self._message_like(message):
                    continue
                results = self.resolve(first, fn, src, 0, parts[at][0])
                if any(r[0] == 'code' and not TOKEN_SHAPE.match(r[1]) for r in results):
                    continue                    # a constant that is not a token: not a code-message pair
                results = self._settle(results, fn)
                severity = None
                for p in parts:
                    sev = [x.val for x in toks[p[0]:p[1]] if x.kind == 'str' and x.val in SEVERITIES]
                    if len(sev) == 1 and p[1] - p[0] == 1:
                        severity = sev[0]
                self.record(results, src, k, 'code-message pair', severity, self.message_text(message), False,
                            expr=_text(first))
                break
        # `Err("CODE")`: a code returned whole as an error value
        for k, t in enumerate(toks):
            if False:
                pass
            elif _is(t, 'id', 'Err') and k + 2 < len(toks) and _is(toks[k + 1], 'p', '(') and \
                    toks[k + 2].kind == 'str' and TOKEN_SHAPE.match(toks[k + 2].val):
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

    def _param_result(self, caller, idx, enum_field=False):
        """What a parameter of `caller` at a code position comes to. A text code parameter's values are
        recorded at its callers' arguments (proven empty here). An enum-typed one at a code field whose type is
        that enum is read through the enum's as_str. Anything else is unresolved."""
        pname, ptype = caller.params[idx]
        text = _stringish(ptype) or (caller.closure and not ptype)
        if idx in caller.code_params and text:
            return _none('flows_to_callers')
        if not text and enum_field:
            return _none('enum_code_field')
        if not text:
            return [('unresolved', f'parameter {pname} of {caller.name}(), of type {ptype}, turned into text')]
        return [('unresolved', f'parameter {pname} of {caller.name}()')]

    def _settle(self, results, fn, enum_field=False):
        """Parameters and enum values among `results`, settled for recording at a code position."""
        out = []
        for r in results:
            if r[0] == 'param':
                out += self._param_result(fn, r[1], enum_field) if fn is not None else \
                    [('unresolved', 'a parameter outside a function')]
            elif r[0] == 'enum':
                out += _none('enum_code_field') if enum_field else \
                    [('unresolved', 'an enum value turned into text at a diagnostic code')]
            else:
                out.append(r)
        return out

    def record_field(self, results, src, k, fn, via, reuse, verified=False, enum_field=False, expr=None):
        results = self._settle(results, fn, enum_field)
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
                if toks[j].kind == 'str' and toks[j].val in ('severity', 'message') and _is(toks[j + 1], 'p', ':'):
                    e = self._value_end(toks, j + 2)
                    if toks[j].val == 'severity':
                        lits = [x.val for x in toks[j + 2:e] if x.kind == 'str' and x.val in SEVERITIES]
                        severity = lits[0] if len(lits) == 1 else severity
                    else:
                        message = self.message_text(toks[j + 2:e])
        self.record(results, src, k, via, severity, message, reuse, verified=verified, expr=expr)

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
        if after is not None and ((after.kind == 'p' and after.val in ('=', '=>', '|', ':')) or _is(after, 'id', 'in')):
            return False                        # a pattern (let, match arm, closure or fn parameter, for)
        return True                             # a literal, a struct update `..base` included

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
        self.unclaimed = {value: set(sites) for value, sites in self.unclaimed_values.items() if value not in self.codes}
        for src in self.sources:
            toks = src.toks
            for k, t in enumerate(toks):
                if t.kind != 'str' or not CODE_SHAPE.match(t.val) or t.val in self.codes:
                    continue
                if (src.rel, k) in self.claimed or self._compared(toks, k):
                    continue
                self.unclaimed.setdefault(t.val, set()).add(f'{src.rel}:{t.line}')

    # -- output

    def apply_pins(self, pins):
        """Pins by hand: an unresolved site whose values the source settles though the scanner cannot
        follow them. A pin holds only while (a) a site the scan left unresolved has its file, its `at`,
        its `why` and the line where the pin's `site_text` stands (once in that file), and (b) each of
        its anchors is a whole function (`function` in `file`, defined once there) whose text is,
        character for character, the anchor's `text`, and one of them is the function the site stands
        in. Any change inside a function the reason rests on, the one that binds, builds and uses the
        value included, fails the pin. A holding pin turns its site into sites of its values; a pin that
        does not hold is a problem, and its site stays unresolved."""
        self.pin_problems = []
        for pin in pins or []:
            where = f'pin at {pin.get("file")} ({pin.get("site_text")!r})'
            problems = []
            line = None
            text = self._read(pin.get('file'))
            if text is None or not pin.get('site_text') or text.count(pin['site_text']) != 1:
                problems.append('its site_text does not stand exactly once in its file')
            else:
                line = text[:text.index(pin['site_text'])].count('\n') + 1
            encloses = False
            for anchor in pin.get('anchors') or []:
                span = self._function_span(anchor.get('file'), anchor.get('function'))
                if span is None:
                    problems.append(f'anchor function {anchor.get("function")!r} is not defined exactly once in '
                                    f'{anchor.get("file")}')
                    continue
                first, last, ftext = span
                if ftext != anchor.get('text'):
                    problems.append(f'anchor function {anchor["function"]} in {anchor["file"]} has changed')
                if anchor.get('file') == pin.get('file') and line is not None and first <= line <= last:
                    encloses = True
            if not pin.get('anchors'):
                problems.append('it has no anchor')
            elif line is not None and not encloses:
                problems.append('no anchor is the whole function its site stands in')
            if not pin.get('values') or not pin.get('reason'):
                problems.append('it names no values or no reason')
            key = (pin.get('file'), line, pin.get('at'))
            if line is not None and self.unresolved.get(key) != pin.get('why'):
                problems.append(f'no site the scan left unresolved is at line {line} ({pin.get("at")}: {pin.get("why")})')
            if problems:
                self.pin_problems.append(f'{where}: ' + '; '.join(problems))
                continue
            del self.unresolved[key]
            for value in pin['values']:
                entry = self.codes.setdefault(value, {'sites': {}})
                entry['sites'].setdefault((pin['file'], line), {
                    'file': pin['file'], 'line': line, 'via': pin['at'], 'severity': None, 'message': None,
                    'by_name': False, 'any_arm': False, 'pinned': True,
                    'through': [f'pinned by hand: {pin["why"]} (pinned_sites)']})
        return self

    def _function_span(self, rel, name):
        """(first line, last line, text) of the one function `name` defined in file `rel`, from the line of
        its `fn` to the line of its closing brace; None when it is not defined exactly once."""
        text = self._read(rel)
        if text is None or not name:
            return None
        toks = tokenize(text)
        at = [k for k in range(len(toks) - 1) if _is(toks[k], 'id', 'fn') and _is(toks[k + 1], 'id', name)]
        if len(at) != 1:
            return None
        k, depth = at[0] + 2, 0
        while k < len(toks):
            t = toks[k]
            if t.kind == 'p' and t.val in ('(', '['):
                depth += 1
            elif t.kind == 'p' and t.val in (')', ']'):
                depth -= 1
            elif depth == 0 and t.kind == 'p' and t.val in ('{', ';'):
                break
            k += 1
        if k >= len(toks) or not _is(toks[k], 'p', '{'):
            return None
        end = _match_forward(toks, k)
        first, last = toks[at[0]].line, toks[end].line
        return first, last, '\n'.join(text.split('\n')[first - 1:last])

    def line_text(self, rel, line):
        text = self._read(rel)
        lines = text.split('\n') if text is not None else []
        return lines[line - 1].strip() if 0 < line <= len(lines) else ''

    def _read(self, rel):
        if not rel:
            return None
        path = self.project / rel
        return path.read_text(encoding='utf-8') if path.is_file() else None

    def table(self):
        codes = []
        for code in sorted(self.codes):
            sites = sorted(self.codes[code]['sites'].values(), key=lambda s: (s['file'], s['line'], s['via']))
            stated = {s['severity'] for s in sites if s['severity']}
            severity = stated.pop() if len(stated) == 1 else ('varies' if stated else 'not_stated')
            crates = sorted({self._crate_of(s['file']) for s in sites})
            # The first site: one resolved by type and arm; else one resolved by type; else, said so,
            # one resolved only by a method's name.
            first = next((s for s in sites if not s['by_name'] and not s['any_arm']), None) or \
                next((s for s in sites if not s['by_name']), None) or sites[0]
            messages = []
            for s in [first] + sites:
                if s['message'] and s['message'] not in messages:
                    messages.append(s['message'])
            record = {
                'code': code,
                'crates': crates,
                'severity': severity,
                'first_site': {'file': first['file'], 'line': first['line'], 'via': first['via']},
                'sites': [{k: v for k, v in s.items() if k != 'message' and not (k in ('by_name', 'any_arm', 'pinned') and not v)}
                          for s in sites],
                'messages': [m[:400] for m in messages[:4]],
            }
            if all(s['by_name'] for s in sites):
                record['sites_by_name_only'] = True
            codes.append(record)
        unresolved = [{'file': f, 'line': line, 'at': via, 'why': why,
                       'expression': self.unresolved_expr.get((f, line, via)) or '',
                       'line_text': self.line_text(f, line)}
                      for (f, line, via), why in sorted(self.unresolved.items())]
        unclaimed = [{'literal': lit, 'sites': sorted(sites)} for lit, sites in sorted(self.unclaimed.items())]
        strip = [{'file': f, 'line': line, 'why': why} for f, line, why in self.strip_failures]
        return {'codes': codes, 'unresolved_sites': unresolved, 'unclaimed_literals': unclaimed,
                'strip_failures': strip}

    def _crate_of(self, rel):
        for src in self.all_sources:
            if src.rel == rel:
                return src.crate
        return rel.split('/src/')[0]


BLOCK_STATEMENTS = {'if', 'match', 'for', 'while', 'loop', 'unsafe', '{'}


PATTERN_STOPS = {'let', 'if', 'while', 'for', 'match', 'in', 'else', 'return', 'loop', 'move'}


def _body_brace(seg, j):
    """The index of the body `{` of the `if`, `while`, `for` or `match` whose condition, pattern or
    scrutinee starts at seg[j]: the first `{` at depth 0 after an operand (a `{` after an operator or keyword
    is a block expression inside the condition); a `let` pattern is skipped to its `=` and a `for` pattern to
    its `in` first. None when there is none."""
    k, depth = j, 0
    if k < len(seg) and seg[k].kind == 'id' and seg[k].val in ('let',):
        while k < len(seg) and not (_is(seg[k], 'p', '=') and _depth_of(seg, j, k) == 0):
            k += 1
        k += 1
    while k < len(seg):
        x = seg[k]
        if x.kind == 'p' and x.val in ('(', '['):
            depth += 1
        elif x.kind == 'p' and x.val in (')', ']'):
            depth -= 1
        elif depth == 0 and _is(x, 'p', '{'):
            if k > j and _ends_operand(seg[k - 1]):
                return k
            k = _match_forward(seg, k)
        k += 1
    return None


def _ends_operand(t):
    """Whether token t can end an operand, so that a `{` after it opens a body rather than a block
    expression (`if x {` against `if {`, `if a && {`)."""
    if t.kind == 'id':
        return t.val not in ('if', 'in', 'match', 'while', 'for', 'let', 'return', 'else', 'mut', 'move', 'as',
                             'loop', 'break', 'unsafe')
    return t.kind in ('str', 'num', 'char', 'life') or (t.kind == 'p' and t.val in (')', ']', '}', '?'))


def _continues(t):
    """Whether token t, after a block's closing brace, continues the same expression."""
    if t.kind == 'id':
        return t.val in ('else', 'as')
    return t.kind == 'p' and t.val not in ('{', '(', '[', '#', '!', '&', '*', '-', '|', '::')


def _match_back(toks, a, k):
    """The index of the bracket that opens the one closing at toks[k], searching no further left than a."""
    depth = 0
    for j in range(k, a - 1, -1):
        t = toks[j]
        if t.kind == 'p' and t.val in CLOSE:
            depth += 1
        elif t.kind == 'p' and t.val in OPEN:
            depth -= 1
            if depth == 0:
                return j
    return None


def _depth_of(toks, a, j):
    d = 0
    for x in toks[a:j]:
        if x.kind == 'p' and x.val in OPEN:
            d += 1
        elif x.kind == 'p' and x.val in CLOSE:
            d -= 1
    return d


# ---------------------------------------------------------------- the committed table

# Reason tokens of the arc maxima (printed inside messages, outside the unit; see HEADER scope).
_ARC = 'core/solver/frame_kernel/src/structural/arc_maximum.rs'
_U4B = 'core/product_physics/src/u4b_rows.rs'
_ARC_PRINTED_IN = ['EXACT_STRESS_GOVERNING_MAXIMUM_UNAVAILABLE', 'DIRECTIONAL_INTENSIFIED_MAXIMUM_UNAVAILABLE',
                   'PREVIEW_STRESS_MAXIMUM_UNAVAILABLE']
ARC_REASON_TOKENS = [
    {'token': token, 'produced_at': f'{where}:{line} ({how})', 'printed_in': list(printed_in)}
    for token, where, line, how, printed_in in [
        ('arc_maximum_invalid_operand', _ARC, 256, 'ArcMaximumFailure::reason', _ARC_PRINTED_IN),
        ('arc_maximum_enclosure_failed', _ARC, 257, 'ArcMaximumFailure::reason', _ARC_PRINTED_IN),
        ('arc_maximum_trig_argument_outside_domain', _ARC, 258, 'ArcMaximumFailure::reason', _ARC_PRINTED_IN),
        ('arc_maximum_trig_term_limit', _ARC, 259, 'ArcMaximumFailure::reason', _ARC_PRINTED_IN),
        ('arc_maximum_curvature_bound_not_finite', _ARC, 260, 'ArcMaximumFailure::reason', _ARC_PRINTED_IN),
        ('arc_maximum_subdivision_limit', _ARC, 261, 'ArcMaximumFailure::reason', _ARC_PRINTED_IN),
        ('arc_maximum_depth_limit', _ARC, 262, 'ArcMaximumFailure::reason', _ARC_PRINTED_IN),
        ('arc_maximum_station_cover_not_closed', _ARC, 263, 'ArcMaximumFailure::reason', _ARC_PRINTED_IN),
        ('arc_maximum_end_axes_unavailable', _U4B, 561, 'map_err in u4b_rows::arc_maximum', _ARC_PRINTED_IN),
        ('arc_maximum_geometry_unavailable', _U4B, 562, 'map_err in u4b_rows::arc_maximum', _ARC_PRINTED_IN),
        ('arc_maximum_end_forces_missing', _U4B, 559, 'Err in u4b_rows::arc_maximum; also in unclaimed_literals',
         _ARC_PRINTED_IN),
        ('arc_maximum_station_rows_unavailable', 'core/product_physics/src/lib.rs', 7274,
         'Err in the exact member rows, and at line 7318; also in unclaimed_literals', _ARC_PRINTED_IN),
    ]
]

# Publication, traced by hand (HEADER scope; the test does not check it). Three states: traced reachable,
# traced not published, unresolved. Only what was traced is recorded; every other code is unresolved.
_NOT_PUBLISHED_TRACE = {
    'state': 'traced_not_published',
    'revision': ('traced at 81bab5c14 and confirmed by the assigner on main at 29226060d; the admission step read '
                 'in the sources of 72854e1b0 (claude/diagnostic-code-table)'),
    'entry_points_examined': ('the headless runner\'s retained entry: run_preview_model_value_with_retained_headless '
                              '(core/runner/headless/src/lib.rs 745), which calls '
                              'run_linear_static_preview_value_with_retained_headless (lines 776 to 779) and takes '
                              'into_parts() of its output; and every non-test Rust caller of '
                              'run_linear_static_preview_value_with_retained_direct, into_publication() and '
                              'into_parts() found by name in core and in the desktop\'s native crate'),
    'decisive_step': ('admission refuses the headless caller: the dispatch (core/product_physics/src/lib.rs 2875 to '
                      '2896) calls retained_memory::admit, whose law_order (core/product_physics/src/'
                      'retained_memory.rs 4006 to 4017) applies caller_clause first (lines 1200 to 1206, "D1.0: '
                      'Direct only (D-2; U3 F-5: admit refuses Headless)"); the refused invocation takes the '
                      'unchanged ordinary route (ordinary_dispatch, lib.rs 2898 to 2915) with no retained result, '
                      'so no retained-precision successor is built on this route and the code is never constructed '
                      'there; the ordinary envelope and the admission report are what the runner consumes'),
    'conditional_discard': ('into_parts() (core/product_physics/src/lib.rs 2851) would drop a successor if one were '
                            'ever produced on the headless route ("dropped here, never published"); on this route '
                            'none is produced'),
    'no_caller_found': ('no production caller of the Direct entry (run_linear_static_preview_value_with_retained_'
                        'direct) and no caller of into_publication() was found outside tests. This is recorded as '
                        '"no caller found" and not as "cannot reach a person"'),
    'registered_build': ('the one registered build identity is the development and test one (core/product_physics/'
                         'src/retained_memory.rs 1 to 8: "One production profile is registered ... the dev/test '
                         'identity"); every other build is Stale and keeps the ordinary route'),
    'not_read': ('the trace searched non-test Rust callers by name in core and in the desktop\'s native crate; the '
                 'wasm packaging and the TypeScript side were not read'),
    'recheck_when': ('any change to the callers named above, to admission\'s caller clause or law order, to the '
                     'registered builds (REGISTERED_PROFILES), to into_parts() or into_publication(), or to how a '
                     'retained-precision successor is published must prompt rechecking this entry'),
}
CONSTRUCTED_NOT_PUBLISHED = [
    dict(code=code, constructed_at=where, **_NOT_PUBLISHED_TRACE) for code, where in [
        ('combination_unresolved', 'core/product_physics/src/retained_wire.rs 2037, 2040, 2042 (combination_entry) and '
         '2171 (serialize_combination_attempt): retained_precision.body.combinations[].reason.code'),
        ('facade_certificate', 'core/product_physics/src/retained_wire.rs 2170 (serialize_combination_attempt) and 2291 '
         '(serialize_attempt): retained_precision.body.combinations[].reason.code and .cases[].reason.code'),
        ('kernel_refused', 'core/product_physics/src/retained_wire.rs 2292, published as code.0 at 2298 '
         '(serialize_attempt): retained_precision.body.cases[].reason.code'),
        ('kernel_unresolved', 'core/product_physics/src/retained_wire.rs 2292, published as code.0 at 2298 '
         '(serialize_attempt): retained_precision.body.cases[].reason.code'),
        ('source_unavailable', 'core/product_physics/src/retained_wire.rs 2275 (serialize_attempt): '
         'retained_precision.body.cases[].reason.code'),
    ]
]

# What repairing the test-code stripper found about the tool as committed on this branch until then.
STRIPPER_FINDING = (
    'A finding about this tool as committed on its branch before the stripper was repaired: its test-code '
    'stripper silently removed production code in 25 files. In 12 of them the code removed is compiled in every '
    'build: a test-only thread_local! { .. } swallowed what followed it (526 lines of '
    'core/model_operations/operation_applier/src/numerical_health.rs, 270 of '
    'core/solver/nonlinear_integration/src/lib.rs), a test-only struct field (`#[cfg(test)] pub trace_fault: ..,` '
    'at core/product_physics/src/retained_product.rs 187) swallowed the fields and code after it, a test-only '
    'if let swallowed the rest of its function and the next, and a test-only const fn swallowed what followed '
    'it; core/product_physics/src/lib.rs 4672 to 4703, which holds the SOLVER_SYSTEM_BLOCKED diagnostic site at '
    '4681, was not read. In the other 14 (one file is in both groups) the code removed is compiled under '
    'cfg(any(test, feature = "..")), now read as production. Reading all of it again added no code to the '
    'table: it added the one site at core/product_physics/src/lib.rs 4681 for a code the table already held, '
    'and sixteen unclaimed literals from core/solver/frame_kernel/src/structural/retained/seeded.rs.')

# Forms the scanner reads besides the producer shapes below, each with its own witness in the test.
READ_FORMS = [
    'a record\'s code field read through a receiver whose type the scanner can prove (self in an impl, a typed '
    'parameter, a local bound to a struct literal or annotated with a type, Err(x), Ok(x) or Some(x) over a direct '
    'call whose declared Result or Option return type names it, or a closure parameter or for variable over a '
    'named slice or Vec): recorded where the record is built, when that type is defined in a scanned crate '
    'without Deserialize and no type of that name with a code field is defined elsewhere for the crate',
    'the nearer of a let and a pattern binding of a name, read only within the pattern\'s scope',
    'the statement forms the test-code stripper recognises under a test-only attribute: items (const fn among '
    'them), fields, labelled loops, brace-delimited macro invocations, if (its condition may be a block) with else '
    'if and else, while, for, loop, match, a method chain after a block, let and expression statements',
    'a module or item under cfg(any(test, feature = "..")), read as production (the conservative choice): '
    'core/solver/frame_kernel/src/structural/retained/seeded.rs under mutation-controls and '
    'exact_boundary/exact_operation_count.rs under exact-operation-count among them; seeded.rs adds sixteen '
    'unclaimed literals (VK-F01 to VK-F17, VK-R02, VK-R28, VK-S1, VK-S2 and FK_SEEDED_FAULT)',
]

# The producer shapes the scanner reads, each with the code its witness adds or removes, the `via` the
# scanner gives that site, and the witness source (tests/test_diagnostic_code_table.py adds each to a
# synthetic crate and removes it again; the comparison must see exactly that code come and go).
PRODUCER_SHAPES = [
    ('the code field of a diagnostic record (a struct or enum variant with code and message or severity) in a '
     'literal', 'ShapeRecordField', 'field code',
     'pub fn s_record() -> Diagnostic { Diagnostic { code: "ShapeRecordField".into(), severity: "info".into(), '
     'message: "m".into() } }'),
    ('an assignment .code = on a diagnostic record', 'ShapeAssigned', 'assignment .code',
     'pub fn s_assign() -> Diagnostic { let mut d = field(); d.code = "ShapeAssigned".into(); d }'),
    ('an argument at a parameter that reaches a code position, through helpers to a fixed point',
     'ShapeHelperFlow', 'relay()', 'pub fn s_flow() -> Diagnostic { relay("ShapeHelperFlow") }'),
    ('an argument at a text parameter named code whose sink is not traced (code-shaped values count, others '
     'are unclaimed)', 'SHAPE_NAMED_PARAM', 'untraced()', 'pub fn s_named() -> usize { untraced("SHAPE_NAMED_PARAM") }'),
    ('the code field of a struct not known as a diagnostic record (code-shaped values count, others are '
     'unclaimed)', 'SHAPE_PLAIN_FIELD', 'field code',
     'pub fn s_plain() -> Tagged { Tagged { code: "SHAPE_PLAIN_FIELD", n: 1 } }'),
    ('a JSON object\'s "code" key, whether or not a message or severity stands beside it', 'shapeJsonReason',
     'json code', 'pub fn s_json() -> String { serde_json::json!({"code": "shapeJsonReason", "phase": "p"}).to_string() }'),
    ('the as_str of an enum a diagnostic record declares as its code type (enum-to-text producer)',
     'ShapeEnumText', 'ShapeKind::as_str() (enum code producer)',
     'pub enum ShapeKind { Only }\n'
     'impl ShapeKind { pub fn as_str(self) -> &\'static str { match self { ShapeKind::Only => "ShapeEnumText" } } }\n'
     'pub struct ShapeFinding { pub code: ShapeKind, pub message: String }'),
    ('a method code(&self) or *_code(&self) returning text (typed producer)', 'ShapeTypedProducer',
     'ShapeError::code() (typed producer)',
     'pub struct ShapeError;\nimpl ShapeError { pub fn code(&self) -> &\'static str { "ShapeTypedProducer" } }'),
    ('the upper-case prefix CODE: of a message literal', 'SHAPE_MESSAGE_PREFIX', 'message prefix',
     'pub fn s_prefix() -> String { "SHAPE_MESSAGE_PREFIX: the detail".to_string() }'),
    ('a (CODE, message) pair (code-shaped values)', 'SHAPE_PAIR', 'code-message pair',
     'pub fn s_pair() -> Vec<(&\'static str, String)> { vec![("SHAPE_PAIR", format!("detail"))] }'),
    ('a code-shaped Err("CODE") error value', 'SHAPE_ERR_VALUE', 'error value',
     'pub fn s_err() -> Result<(), &\'static str> { Err("SHAPE_ERR_VALUE") }'),
    ('a value followed through a constant', 'ShapeConstant', 'emit()',
     'const SHAPE_CONST: &str = "ShapeConstant";\npub fn s_const() -> Diagnostic { emit(SHAPE_CONST, "m") }'),
    ('a value followed through a tuple let bound from match arms', 'ShapeTupleLet', 'emit()',
     'pub fn s_tuple(x: u8) -> Diagnostic { let (c, m) = match x { 1 => ("ShapeTupleLet", "m"), _ => return field() }; '
     'emit(c, m) }'),
    ('a value followed through a for loop\'s tuple pattern over a local filled by push', 'ShapeForTuple', 'emit()',
     'pub fn s_for() -> Vec<Diagnostic> { let mut v = Vec::new(); v.push(("ShapeForTuple", String::new())); '
     'let mut out = Vec::new(); for (code, message) in v { out.push(emit(code, &message)); } out }'),
    ('a value followed through if let Err(x) over a function\'s return and a closure\'s tuple pattern',
     'ShapeIfLetErr', 'emit()',
     'fn s_check() -> Result<(), (&\'static str, u8)> { Err(("ShapeIfLetErr", 0)) }\n'
     'pub fn s_iflet() -> Vec<Diagnostic> { let mut fs = Vec::new(); if let Err(f) = s_check() { fs.push(f); } '
     'fs.into_iter().map(|(code, _n)| emit(code, "m")).collect() }'),
    ('a value followed through a record field holding a tuple, read by a closure pattern', 'ShapeFieldTuple',
     'json code',
     'pub struct Run { pub exit: Option<(&\'static str, u8)> }\n'
     'pub fn s_run() -> Run { Run { exit: Some(("ShapeFieldTuple", 0)) } }\n'
     'pub fn s_exit(r: &Run) -> Option<String> { r.exit.as_ref().map(|(code, n)| serde_json::json!({"code": code, '
     '"n": n}).to_string()) }'),
    ('a value followed through a tuple element code.0', 'ShapeProjection', 'json code',
     'pub fn s_proj(x: u8) -> String { let c = match x { 1 => ("ShapeProjection", "p"), _ => ("ShapeProjection", "q") }; '
     'serde_json::json!({"code": c.0, "phase": c.1}).to_string() }'),
    ('a value converted by to_ascii_uppercase(), to_uppercase(), to_ascii_lowercase() or to_lowercase()',
     'SHAPE_CONVERTED', 'emit()', 'pub fn s_convert() -> Diagnostic { emit(&"shape_converted".to_ascii_uppercase(), "m") }'),
    ('production code after a test-only if', 'ShapeAfterTestIf', 'emit()',
     'pub fn s_after_if() -> Diagnostic {\n    #[cfg(test)]\n    if true { let _ = emit("ShapeInsideTestIf", "m"); }\n'
     '    emit("ShapeAfterTestIf", "m")\n}'),
    ('production code after a test-only match', 'ShapeAfterTestMatch', 'emit()',
     'pub fn s_after_match() -> Diagnostic {\n    #[cfg(test)]\n    match 1 { _ => { let _ = emit("ShapeInsideTestMatch", "m"); } }\n'
     '    emit("ShapeAfterTestMatch", "m")\n}'),
    ('production code after a test-only for', 'ShapeAfterTestFor', 'emit()',
     'pub fn s_after_for() -> Diagnostic {\n    #[cfg(test)]\n    for _ in 0..1 { let _ = emit("ShapeInsideTestFor", "m"); }\n'
     '    emit("ShapeAfterTestFor", "m")\n}'),
]


HEADER = {
    'about': ('The diagnostic codes production code of the solve crates constructs, read from those sources by '
              'tools/validation/diagnostic_codes.py, with a hand-made classification of each. What a code is, what '
              'is read and what is not, and what the test establishes are in scope. The scanner writes code, crates, '
              'severity, first_site, sites and messages; classification, reason, unsure, candidates, named_by, '
              'named_by_coverage and named_by_statement are hand-made and kept by --write, as are pinned_sites and '
              'listed_unresolved_sites. Every code position the scanner reads is resolved, pinned with its values, '
              'or listed as unresolved with its reason; none is dropped. The test cannot check that a text covers a '
              'refusal, only that it mentions a code: each naming carries a hand-made coverage, whole or partial, '
              'and only a whole naming counts as naming the refusal. ' + STRIPPER_FINDING),
    'first_site': ('the first raising site resolved by type (and, where a mapping is involved, by the arm its '
                   'argument selects); a site resolved only by a method\'s name carries by_name, one that may '
                   'raise any arm of a mapping carries any_arm, and neither is chosen as first while another '
                   'exists; sites_by_name_only says that every site was resolved by name'),
    'named_by_coverage': ('for a code named by a LIM entry: {"coverage": "whole" or "partial", "reason"}; a partial '
                          'naming counts as unnamed'),
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
                           '{"status": "in_product", "text", "where", "coverage", "coverage_reason"} for text the '
                           'product publishes today (a formulation_basis limitation), whole or partial as for '
                           'named_by_coverage; {"status": "planned", "statement", "subject", "lands_with"} '
                           'for a per-document limitation statement not yet in the product, which names nothing '
                           'today. Planned statements may name codes of any class.'),
    'pinned_site': ('an entry of pinned_sites: a code position whose value the scanner cannot resolve but the '
                    'source settles, pinned by hand: {"file", "at", "why", "site_text", "values", "reason", '
                    '"anchors": [{"file", "function", "text"}]}. A '
                    'pin holds only while the scan leaves a site unresolved with that file, at and why on the line '
                    'where site_text stands (once in the file), and each anchor is a whole function, defined once '
                    'in its file, whose text equals the anchor text character for character; one anchor must be '
                    'the function the site stands in, which binds, builds and uses the value. A holding pin turns '
                    'the site into sites of its values. Any other unresolved site must be listed in '
                    'listed_unresolved_sites, or the test fails.'),
    'listed_unresolved_site': ('an entry of listed_unresolved_sites, kept by hand: {"file", "line_text", "at", '
                               '"expression", "form", "outside_unit", "not_known", and "relay_of" or "reason"}. An '
                               'unresolved code position the scanner reports must match an entry by file, at, '
                               'expression and the text of its line, or the test fails; an entry that matches no '
                               'reported site (its text changed, or it was resolved) fails too. relay_of says why '
                               'the value is believed to relay a code built at a position the scanner reads, naming '
                               'it; not_known true says that is not known (reason). outside_unit true marks an '
                               'operation applier code position whose own codes are outside the unit.'),
    'scope': {
        'unit': ('The unit of this table is the code: the value of the code field of a diagnostic, finding, '
                 'refusal or reason record that production code of the scanned crates constructs. There is one '
                 'record per distinct value, whatever its spelling.'),
        'what_in_the_table_means': ('The table is conservative about publication among the constructions the '
                                    'scanner discovers: it lists a code whether or not the record carrying it is '
                                    'published. It is not a superset of all refusals a person can meet: the scanner '
                                    'does not claim completeness (shapes_not_read), excludes reason tokens printed '
                                    'inside messages and some producer forms (outside_the_unit), and does not cover '
                                    'every product surface (crates). That a code is in the table does not mean that '
                                    'a person can meet the refusal it names.'),
        'publication': ('Whether a code reaches a person is recorded in three states: traced reachable, traced not '
                        'published, and unresolved. Only a code with an entry in publication_traced has been '
                        'traced; every other code in this table is unresolved as to publication. No code is '
                        'recorded as traced reachable at this revision. The ten source-recovery failure codes '
                        '(first site core/product_physics/src/source_receipt.rs 689) are unresolved; the place to '
                        'look is what becomes of a case\'s failed receipt entry when the invocation\'s receipt is '
                        'finalized or cannot be (core/product_physics/src/lib.rs, the composite finalization and '
                        'its republication).'),
        'publication_traced_note': ('This list is kept by hand and is not checked by the test. Each entry records '
                                    'its revision, the entry points examined, the decisive step, what would discard a '
                                    'result conditionally, the callers not found, the registered build and what was '
                                    'not read, and says when it must be rechecked.'),
        'publication_traced': CONSTRUCTED_NOT_PUBLISHED,
        'what_exit_criterion_3_still_needs': ('This table supports the work exit criterion 3 needs and does not do '
                                              'it. That work is the connection from a required capability to its '
                                              'reachable refusal, its practical consequence and a declared '
                                              'limitation. It is traced by shared producer and publication '
                                              'boundaries, not code by code through this table: one code can cover '
                                              'several materially different refusals, and a missing control can '
                                              'withhold a capability with no code at all. Product text that names a '
                                              'limitation is not a declaration of one.'),
        'every_code_position': ('Every code position the scanner reads is resolved, pinned with its values '
                                '(pinned_sites), or listed as unresolved with its reason (listed_unresolved_sites); '
                                'none is dropped. A part of a value yields no code only in one of the cases of '
                                'proven_empty_cases; anything else that yields none is an unresolved site.'),
        'proven_empty_cases': PROVEN_EMPTY,
        'listed_unresolved': None,      # filled by merge from listed_unresolved_sites
        'crates': list(SCOPE) + [f'{p} (a solve\'s code it relays unchanged only)' for p in REUSE_SCOPE],
        'operation_applier': ('core/model_operations/operation_applier is read only for a solve\'s code that it '
                              'relays unchanged: a code position there whose value is a constant declared in a '
                              'scanned solve crate. The applier\'s own codes are excluded, including an applier '
                              'code that wraps a solve refusal. An applier code position whose value the scanner '
                              'cannot resolve is listed in listed_unresolved_sites like any other, and its entry '
                              'says whether it is outside the unit.'),
        'outside_the_unit': ('Reason tokens printed inside messages are not inventoried by this table, and no '
                             'inventory of them exists. The arc-maximum reason tokens below are such tokens: each '
                             'is produced by a named refusal method or a map_err or Err literal, and is printed '
                             'inside the message of an enclosing diagnostic whose code is listed here. Other token '
                             'fields of a published record (a reason\'s tag, kind, stage or cause, a disposition\'s '
                             'reason) are not codes either.'),
        'reason_tokens_outside_the_unit': ARC_REASON_TOKENS,
        'what_the_test_establishes': ('tests/test_diagnostic_code_table.py establishes that this table agrees '
                                      'with the scanner\'s observation of the sources over the shapes and forms '
                                      'listed in shapes_read, each with a witness in that test (for a producer '
                                      'shape, that adding or removing a code at it fails the test), and that every '
                                      'code position the scanner reads is resolved, pinned or listed. It does not '
                                      'establish that the table is complete. A third independent review found no '
                                      'code missing from the table in the present sources, all twenty producer-'
                                      'shape witnesses working, and its blind classification of the sixteen codes '
                                      'then new agreeing 16 of 16.'),
        'shapes_read': [shape for shape, _code, _via, _source in PRODUCER_SHAPES] + READ_FORMS,
        'shapes_not_read': [
            'named refusal methods (a reason() returning a token) and map_err or Err literals carried into a '
            'message: outside the unit (outside_the_unit); a code-shaped Err literal is still read',
            'raw enum codes never published as text: an enum value at a code field declared with that enum is read '
            'through its as_str only; enum variant names that serde or Display turn into text elsewhere are not read',
            'codes that reach a person by a route that is not a code position: a field not named code, a reason or '
            'tag token, a map value, text built in a macro_rules body or generated code',
            'tuple-returning helpers whose values are not published at a code position',
            'a receiver type behind a crate-local Result alias, a field\'s declared type, a method chain or a run-time '
            'choice of function: such a code field read is an unresolved site, listed with its reason',
            'test-only code marked other than by cfg(test) or cfg(all(test, ..)): it is read as production',
            'mixed-case message prefixes: only an upper-case CODE: or {CONST}: prefix is read',
        ],
    },
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
    listed = (existing or {}).get('listed_unresolved_sites', [])
    header = json.loads(json.dumps(HEADER))
    header['scope']['listed_unresolved'] = listed_summary(listed)
    return dict(header, codes=codes, pinned_sites=(existing or {}).get('pinned_sites', []),
                listed_unresolved_sites=listed, unresolved_sites=mechanical['unresolved_sites'],
                strip_failures=mechanical['strip_failures'], unclaimed_literals=mechanical['unclaimed_literals'])


def listed_summary(listed):
    return (f'{len(listed)} code positions are listed as unresolved (listed_unresolved_sites), '
            f'{sum(1 for e in listed if e.get("not_known"))} of them marked not known; '
            f'{sum(1 for e in listed if e.get("outside_unit"))} are operation applier positions outside the unit.')


def listing_problems(unresolved_sites, listed):
    """The unresolved sites a scan reports that no entry lists, and the entries that match no reported site,
    matched as multisets by (file, at, expression, line text); and entries missing a reason."""
    from collections import Counter
    key = lambda x: (x.get('file'), x.get('at'), x.get('expression'), x.get('line_text'))
    reported, held = Counter(key(u) for u in unresolved_sites), Counter(key(e) for e in listed)
    problems = [f'unlisted unresolved site {k[0]}: {k[3]!r} ({k[1]}: {k[2]})' for k in sorted((reported - held).elements())]
    problems += [f'listed site that no longer stands as listed {k[0]}: {k[3]!r} ({k[1]}: {k[2]})'
                 for k in sorted((held - reported).elements())]
    for e in listed:
        if not e.get('form') or not isinstance(e.get('not_known'), bool) or not isinstance(e.get('outside_unit'), bool):
            problems.append(f'listed site {e.get("file")}: {e.get("line_text")!r} lacks form, not_known or outside_unit')
        elif e['not_known'] and not str(e.get('reason', '')).strip():
            problems.append(f'listed site {e.get("file")}: {e.get("line_text")!r} is marked not known with no reason')
        elif not e['not_known'] and not str(e.get('relay_of', '')).strip():
            problems.append(f'listed site {e.get("file")}: {e.get("line_text")!r} names no relay_of')
    return problems


def committed_pins(project):
    """The pins of the committed table (`pinned_sites`), written by hand."""
    path = project / TABLE
    return json.loads(path.read_text(encoding='utf-8')).get('pinned_sites', []) if path.is_file() else []


def dumps(table):
    return json.dumps(table, indent=1, ensure_ascii=False, sort_keys=False) + '\n'


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.split('\n')[0])
    parser.add_argument('--project', type=Path, default=PROJECT)
    parser.add_argument('--json', action='store_true', help='print the mechanical table')
    parser.add_argument('--write', action='store_true', help='refresh the committed table, keeping hand fields')
    args = parser.parse_args(argv)
    scan = Scan(args.project.resolve()).run().apply_pins(committed_pins(args.project.resolve()))
    table = scan.table()
    for problem in scan.pin_problems:
        print(f'pin problem: {problem}', file=sys.stderr)
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
