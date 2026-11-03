r"""Rust literal lexer for the retained-pricing text measurements (stdlib only, read-only).

§8.45: replaces the quote-pairing regex `"((?:\\.|[^"\\])*)"`, which did not lex comments, char
literals or raw strings and so paired quotes across code (over- and undercounting).

`literals(text)` walks a Rust source file once and returns every literal token in it:
  - line comments (`//`, `///`, `//!`) and block comments (`/* */`, nested, doc forms) are skipped;
  - char literals (`'x'`, `'"'`, `'\''`, `'\u{1F600}'`) are told apart from lifetimes and labels
    (`'a`, `'static`, `'outer:`) by the rustc rule: `'` + one char + `'` (or `'\` + escape + `'`)
    is a char literal, `'` + identifier not closed by `'` is a lifetime/label;
  - strings `"…"` with escapes `\n \r \t \\ \0 \' \" \xHH \u{…}` and the line continuation
    (backslash-newline, which skips the following ASCII whitespace), across lines;
  - raw strings `r"…"`, `r#"…"#` (any number of hashes), byte strings `b"…"`, `br#"…"#`, byte
    chars `b'x'`, C strings `c"…"` and raw C strings `cr#"…"#`;
  - raw identifiers `r#name` are identifiers, not literals.
Each string literal's `value` is its runtime value after unescaping (str for str/C strings,
bytes for byte strings); `nbytes` is the value's runtime UTF-8 byte length (C strings exclude
the implicit NUL terminator).

Fail closed: any construct it cannot lex with certainty raises `LexError` with file:line — an
unterminated string, raw string, char or block comment; an unknown escape; an out-of-range
`\x`/`\u`; an unknown literal prefix (Rust 2021 reserves `ident"`, `ident'`, `ident#`); a
lifetime immediately followed by `'`; an empty or unescaped-quote char literal; a bare CR
anywhere in the file (rustc's CRLF normalization is not modelled). It never guesses.
"""

import bisect

ESCAPES = {"n": "\n", "r": "\r", "t": "\t", "\\": "\\", "0": "\0", "'": "'", '"': '"'}
WS = " \t\n\r"
STRING_KINDS = ("str", "raw_str", "byte_str", "raw_byte_str", "c_str", "raw_c_str")


class LexError(ValueError):
    pass


def _ident_start(ch):
    return ch == "_" or ch.isalpha()


def _ident_continue(ch):
    return ch == "_" or ch.isalnum()


class _Lexer:
    def __init__(self, text, name):
        self.t, self.name, self.n = text, name, len(text)
        self.nl = [k for k, c in enumerate(text) if c == "\n"]
        self.comments = []                       # (start, end) of every comment, for cross-checks

    def line(self, i):
        return bisect.bisect_left(self.nl, i) + 1

    def fail(self, i, why):
        raise LexError(f"{self.name}:{self.line(i)}: {why}")

    # -- escapes -------------------------------------------------------------------------------
    def escape(self, i, kind, start):
        """At t[i] == '\\'. Returns (decoded char or int byte or None for continuation, next i)."""
        t, n = self.t, self.n
        if i + 1 >= n:
            self.fail(start, f"unterminated {kind} literal")
        e = t[i + 1]
        if e in ESCAPES:
            if kind in ("c_str",) and e == "0":
                self.fail(i, "NUL escape in a C string literal")
            return ESCAPES[e], i + 2
        if e == "x":
            h = t[i + 2:i + 4]
            if len(h) != 2 or any(c not in "0123456789abcdefABCDEF" for c in h):
                self.fail(i, f"malformed \\x escape in {kind} literal")
            v = int(h, 16)
            if kind in ("str", "char") and v > 0x7F:
                self.fail(i, f"\\x escape above 0x7F in {kind} literal")
            if kind == "c_str" and v == 0:
                self.fail(i, "NUL escape in a C string literal")
            if kind in ("byte_str", "byte", "c_str"):
                return v, i + 4
            return chr(v), i + 4
        if e == "u":
            if kind in ("byte_str", "byte"):
                self.fail(i, f"\\u escape in {kind} literal")
            if i + 2 >= n or t[i + 2] != "{":
                self.fail(i, "malformed \\u escape")
            j = t.find("}", i + 3)
            if j < 0:
                self.fail(i, "unterminated \\u escape")
            digits = t[i + 3:j]
            if (not digits or digits[0] == "_" or len(digits.replace("_", "")) > 6
                    or any(c not in "0123456789abcdefABCDEF_" for c in digits)):
                self.fail(i, "malformed \\u escape")
            v = int(digits.replace("_", ""), 16)
            if v > 0x10FFFF or 0xD800 <= v <= 0xDFFF or (kind == "c_str" and v == 0):
                self.fail(i, "\\u escape out of range")
            return chr(v), j + 1
        if e == "\n" and kind != "char" and kind != "byte":
            j = i + 2
            while j < n and t[j] in WS:
                j += 1
            return None, j
        self.fail(i, f"unknown escape \\{e!r} in {kind} literal")

    # -- quoted literals -----------------------------------------------------------------------
    def quoted(self, i, kind, start):
        """At t[i] == '"'. Returns (value, end index after the closing quote)."""
        t, n = self.t, self.n
        parts, j = [], i + 1
        while True:
            if j >= n:
                self.fail(start, f"unterminated {kind} literal")
            ch = t[j]
            if ch == '"':
                break
            if ch == "\\":
                v, j = self.escape(j, kind, start)
                if v is not None:
                    parts.append(v)
                continue
            if kind == "byte_str" and ord(ch) > 0x7F:
                self.fail(j, "non-ASCII character in a byte string literal")
            if kind == "c_str" and ch == "\0":
                self.fail(j, "NUL in a C string literal")
            parts.append(ch)
            j += 1
        return self._value(parts, kind), j + 1

    def _value(self, parts, kind):
        if kind == "byte_str":
            return bytes(p if isinstance(p, int) else ord(p) for p in parts)
        if kind == "c_str":
            out = bytearray()
            for p in parts:
                out += bytes([p]) if isinstance(p, int) else p.encode("utf-8")
            return bytes(out)
        return "".join(parts)

    def raw(self, i, kind, start):
        """At t[i] in '#"'. Returns (value, end index)."""
        t, n = self.t, self.n
        h = 0
        while i < n and t[i] == "#":
            h, i = h + 1, i + 1
        if i >= n or t[i] != '"':
            self.fail(start, f"malformed raw {kind} literal opener")
        close = '"' + "#" * h
        j = t.find(close, i + 1)
        if j < 0:
            self.fail(start, f"unterminated raw {kind} literal")
        body = t[i + 1:j]
        if kind == "raw_byte_str":
            if any(ord(c) > 0x7F for c in body):
                self.fail(start, "non-ASCII character in a raw byte string literal")
            return body.encode("ascii"), j + len(close)
        if kind == "raw_c_str":
            if "\0" in body:
                self.fail(start, "NUL in a raw C string literal")
            return body.encode("utf-8"), j + len(close)
        return body, j + len(close)

    def char(self, i, kind, start):
        """At t[i] == "'" opening a char (or byte) literal known to be one. Returns (value, end)."""
        t, n = self.t, self.n
        if i + 1 >= n:
            self.fail(start, f"unterminated {kind} literal")
        ch = t[i + 1]
        if ch == "\\":
            v, j = self.escape(i + 1, kind, start)
            if v is None:
                self.fail(start, f"line continuation in a {kind} literal")
        else:
            if ch in "'\n\r\t":
                self.fail(start, f"{kind} literal holds an unescaped {ch!r}")
            if kind == "byte" and ord(ch) > 0x7F:
                self.fail(start, "non-ASCII byte literal")
            v, j = (ord(ch) if kind == "byte" else ch), i + 2
        if j >= n or t[j] != "'":
            self.fail(start, f"unterminated {kind} literal")
        return v, j + 1

    def _is_shebang(self):
        """rustc's rule: a leading `#!` is a shebang line unless the next token, after whitespace and
        comments (block comments nest), is `[`, which makes it an inner attribute (`#! [allow(x)]`,
        `#!/* c */[x]`). An unterminated block comment there stops the run."""
        t, n, i = self.t, self.n, 2
        while i < n:
            if t[i].isspace():
                i += 1
            elif t.startswith("//", i):
                j = t.find("\n", i)
                i = n if j < 0 else j + 1
            elif t.startswith("/*", i):
                depth, j = 1, i + 2
                while j < n and depth:
                    if t.startswith("/*", j):
                        depth, j = depth + 1, j + 2
                    elif t.startswith("*/", j):
                        depth, j = depth - 1, j + 2
                    else:
                        j += 1
                if depth:
                    self.fail(i, "unterminated block comment after a leading `#!`")
                i = j
            else:
                return t[i] != "["
        return True

    # -- the walk ------------------------------------------------------------------------------
    def run(self):
        t, n = self.t, self.n
        if "\r" in t:
            self.fail(t.index("\r"), "carriage return in source (CRLF normalization is not modelled)")
        out, i = [], 0
        if t.startswith("#!") and self._is_shebang():
            i = t.find("\n") if "\n" in t else n

        def emit(kind, s, e, value):
            out.append({"kind": kind, "start": s, "end": e, "line": self.line(s),
                        "end_line": self.line(e - 1), "value": value,
                        "nbytes": len(value) if isinstance(value, bytes) else
                        (len(value.encode("utf-8")) if isinstance(value, str) else None)})

        while i < n:
            ch = t[i]
            nx = t[i + 1] if i + 1 < n else ""
            if ch == "/" and nx == "/":
                j = t.find("\n", i)
                j = n if j < 0 else j + 1
                self.comments.append((i, j))
                i = j
            elif ch == "/" and nx == "*":
                depth, j = 1, i + 2
                while depth:
                    if j >= n:
                        self.fail(i, "unterminated block comment")
                    if t.startswith("/*", j):
                        depth, j = depth + 1, j + 2
                    elif t.startswith("*/", j):
                        depth, j = depth - 1, j + 2
                    else:
                        j += 1
                self.comments.append((i, j))
                i = j
            elif ch == '"':
                v, j = self.quoted(i, "str", i)
                emit("str", i, j, v)
                i = j
            elif ch == "'":
                # rustc: '\ or 'X' is a char literal; 'ident (not closed) is a lifetime/label
                if nx == "\\" or (i + 2 < n and t[i + 2] == "'" and nx not in ("", "'")):
                    v, j = self.char(i, "char", i)
                    emit("char", i, j, v)
                    i = j
                elif nx and _ident_start(nx):
                    j = i + 1
                    while j < n and _ident_continue(t[j]):
                        j += 1
                    if j < n and t[j] in "'#\"":
                        self.fail(i, f"lifetime/label immediately followed by {t[j]!r}")
                    i = j
                else:
                    self.fail(i, "unterminated or ambiguous char literal")
            elif ch.isdigit():
                j = i + 1
                while j < n and _ident_continue(t[j]):
                    j += 1
                if j < n and t[j] in "'\"#":
                    self.fail(i, f"numeric literal immediately followed by {t[j]!r}")
                i = j
            elif _ident_start(ch):
                j = i + 1
                while j < n and _ident_continue(t[j]):
                    j += 1
                word, after = t[i:j], (t[j] if j < n else "")
                if after not in ("\"", "'", "#"):
                    i = j
                elif word == "r" and after == "#" and j + 1 < n and _ident_start(t[j + 1]):
                    k = j + 1                                    # raw identifier r#name
                    while k < n and _ident_continue(t[k]):
                        k += 1
                    if k < n and t[k] in "'\"#":
                        self.fail(i, f"raw identifier immediately followed by {t[k]!r}")
                    i = k
                elif word in ("r", "br", "cr") and after in "#\"":
                    kind = {"r": "raw_str", "br": "raw_byte_str", "cr": "raw_c_str"}[word]
                    v, k = self.raw(j, kind, i)
                    emit(kind, i, k, v)
                    i = k
                elif word in ("b", "c") and after == '"':
                    kind = {"b": "byte_str", "c": "c_str"}[word]
                    v, k = self.quoted(j, kind, i)
                    emit(kind, i, k, v)
                    i = k
                elif word == "b" and after == "'":
                    v, k = self.char(j, "byte", i)
                    emit("byte", i, k, v)
                    i = k
                else:
                    self.fail(i, f"unknown literal prefix {word!r} before {after!r}")
            else:
                i += 1
        return out


def literals(text, name="<source>"):
    """Every literal token in `text`, in source order (see the module docstring)."""
    return _Lexer(text, name).run()


def scan(text, name="<source>"):
    """(literal tokens, comment spans) of `text`."""
    lx = _Lexer(text, name)
    return lx.run(), lx.comments


def string_literals(text, name="<source>"):
    """The double-quoted literal tokens (str, raw, byte, C) with their runtime values."""
    return [x for x in literals(text, name) if x["kind"] in STRING_KINDS]


def debug_escapes(value):
    """True when the value is not printable ASCII free of `"` and `\\`, i.e. when `{:?}` is not
    exactly `"` + value + `"` under the conservative premise (any non-ASCII char counts as escaped;
    str Debug leaves `'` unescaped)."""
    if isinstance(value, bytes):
        value = value.decode("latin-1")
    return any(ord(c) < 0x20 or ord(c) > 0x7E or c in "\"\\" for c in value)


def doubling_safe(value):
    """True when every char's Debug spelling is at most two chars and its value at least one byte
    (printable ASCII, or one of the two-char escapes \\t \\r \\n \\0 \\" \\\\), so
    `len({:?}) <= 2 + 2 * len(value)`."""
    if isinstance(value, bytes):
        value = value.decode("latin-1")
    return all(0x20 <= ord(c) <= 0x7E or c in "\t\r\n\0" for c in value)


def measure(root, files):
    """The double-quoted literal inventory of `files` (paths relative to `root`): the longest
    literal's runtime byte length and site, the count whose Debug escapes (`debug_escapes`), and
    the sites outside the Debug doubling premise (`doubling_safe`). Stops on any LexError."""
    import os
    best, at, n_bad, unsafe, count = 0, None, 0, [], 0
    for f in files:
        with open(os.path.join(root, f), encoding="utf-8") as h:
            text = h.read()
        for x in string_literals(text, f):
            count += 1
            if x["nbytes"] > best:
                best, at = x["nbytes"], f"{f}:{x['line']}"
            n_bad += debug_escapes(x["value"])
            if not doubling_safe(x["value"]):
                unsafe.append(f"{f}:{x['line']}")
    return {"max_bytes": best, "at": at, "with_escapes": n_bad, "not_doubling_safe": unsafe, "count": count}


# ---- §8.45 follow-up: production scope and the long-literal disposition check -------------------
import os as _os
import re as _re

_CFG_TEST = _re.compile(r"#\s*\[\s*cfg\s*\(\s*test\s*\)\s*\]")
_VIS = _re.compile(r"pub(\s*\(\s*(crate|super|self|in\s+[\w:]+)\s*\))?\s+")
_MOD = _re.compile(r"mod\s+([A-Za-z_]\w*)\s*([{;])")


def _masks(text, toks, comments):
    """Per-offset flag: 1 inside a literal or comment (not code)."""
    m = bytearray(len(text))
    for s, e in [(x["start"], x["end"]) for x in toks] + list(comments):
        m[s:e] = b"\x01" * (e - s)
    return m


def _depths(text, mask):
    """Brace depth before each offset, counting only code braces."""
    d, out = 0, [0] * (len(text) + 1)
    for i, ch in enumerate(text):
        out[i] = d
        if not mask[i]:
            d += (ch == "{") - (ch == "}")
    out[len(text)] = d
    return out


def _module_decls(path, text):
    """The `mod` items of one file: name, kind ('{' inline, ';' file), whether an exact `#[cfg(test)]` is in
    its attribute run, its `#[path]` (False when unparsed), code brace depth and span."""
    toks, comments = scan(text, path)
    mask = _masks(text, toks, comments)
    depth = _depths(text, mask)
    decls = []
    for m in _re.finditer(r"\bmod\b", text):
        start = m.start()
        mm = _MOD.match(text, start)
        if mask[start] or not mm:
            continue
        # `mod` is a reserved keyword, so every code `mod NAME` followed by `;` or `{` is a module item,
        # wherever it sits on its line (after an attribute, inside an inline module or a macro body).
        # Step back over a visibility qualifier, then collect the attributes that precede the item.
        k = start
        vm = _re.search(r"pub(\s*\(\s*(crate|super|self|in\s+[\w:]+)\s*\))?\s*$", text[max(0, start - 80):start])
        if vm and not any(mask[max(0, start - 80) + vm.start():start]):
            k = max(0, start - 80) + vm.start()
        attrs, p = [], k
        while True:
            q = p
            while q > 0 and text[q - 1].isspace():
                q -= 1
            if q > 0 and text[q - 1] == "]" and not mask[q - 1]:
                depth_b, r = 0, q - 1
                while r >= 0:
                    if not mask[r]:
                        depth_b += (text[r] == "]") - (text[r] == "[")
                        if depth_b == 0:
                            break
                    r -= 1
                h = r - 1
                while h >= 0 and text[h].isspace():
                    h -= 1
                if h >= 0 and text[h] == "#" and not mask[h]:
                    attrs.append(text[h:q])
                    p = h
                    continue
            break
        cfg_test = any(_CFG_TEST.fullmatch(a.strip()) for a in attrs)
        path_attr = None
        for a in attrs:
            pm = _re.fullmatch(r'#\s*\[\s*path\s*=\s*"([^"\\]*)"\s*\]', a.strip())
            if pm:
                path_attr = pm.group(1)
            elif _re.match(r"#\s*\[\s*path\b", a.strip()):
                path_attr = False                           # unparsed path attribute: never resolved
        end = mm.end()
        if mm.group(2) == "{":
            open_at = mm.end() - 1
            d0, e = depth[open_at], open_at + 1
            while e < len(text) and not (not mask[e] and text[e] == "}" and depth[e] == d0 + 1):
                e += 1
            end = e + 1
        decls.append({"name": mm.group(1), "kind": mm.group(2), "cfg_test": cfg_test, "path": path_attr,
                      "depth": depth[start], "span": (start, end)})
    return decls


def _child_file(root, parent, decl):
    """The file of a top-level `mod name;` in `parent` (relative to root), or None when not uniquely resolved."""
    if decl["depth"] != 0 or decl["path"] is False:
        return None
    pdir = _os.path.dirname(parent)
    if decl["path"]:
        cands = [_os.path.normpath(_os.path.join(pdir, decl["path"]))]
    elif _os.path.basename(parent) in ("lib.rs", "main.rs", "mod.rs"):
        cands = [_os.path.join(pdir, decl["name"] + ".rs"), _os.path.join(pdir, decl["name"], "mod.rs")]
    else:
        stem = _os.path.join(pdir, _os.path.basename(parent)[:-3])
        cands = [_os.path.join(stem, decl["name"] + ".rs"), _os.path.join(stem, decl["name"], "mod.rs")]
    hits = [c for c in cands if _os.path.isfile(_os.path.join(root, c))]
    return hits[0] if len(hits) == 1 else None


def scope_files(root, crate_dirs):
    """Every `.rs` file under the crate source directories (relative to root), sorted."""
    out = []
    for d in crate_dirs:
        for base, dirs, files in _os.walk(_os.path.join(root, d)):
            dirs.sort()
            out += [_os.path.relpath(_os.path.join(base, f), root) for f in sorted(files) if f.endswith(".rs")]
    return sorted(out)


def production_literals(root, files):
    """The double-quoted literals of `files` outside test-only code. A file is test-only exactly when it is
    not reachable from production: production starts at every file no `mod name;` resolves to (crate roots,
    binaries, build scripts, and files reached only through declarations this pass cannot resolve), and
    follows every resolved file declaration that is not test-only (not under an exact `#[cfg(test)]`, and
    not inside a `#[cfg(test)] mod name { … }` body). A file a test module also declares, or reaches
    through `#[path]`, therefore stays in scope whenever any production declaration reaches it. A
    production declaration this pass cannot resolve keeps every file it could name in scope (each file
    whose stem, or `mod.rs` directory, matches the name), and one with an unparsed `#[path]` stops the run.
    The bodies of inline `#[cfg(test)]` modules are excluded. Over-inclusion is the safe side."""
    texts = {f: open(_os.path.join(root, f), encoding="utf-8").read() for f in files}
    decls = {f: _module_decls(f, t) for f, t in texts.items()}
    children, referenced, unresolved = {}, set(), set()
    for f, ds in decls.items():
        tspans = [d["span"] for d in ds if d["cfg_test"] and d["kind"] == "{"]
        for d in ds:
            if d["kind"] != ";":
                continue
            is_test = d["cfg_test"] or any(s < d["span"][0] < e for s, e in tspans)
            c = _child_file(root, f, d)
            if c in texts:
                referenced.add(c)
                children.setdefault(f, []).append((c, is_test))
            elif not is_test:
                if d["path"] is False:
                    raise LexError(f"{f}: production `mod {d['name']};` has an unparsed #[path]; its file is "
                                   "unknown, so test-only exclusion cannot be decided")
                unresolved.add(_os.path.splitext(_os.path.basename(d["path"]))[0] if d["path"] else d["name"])
    seeds = {f for f in texts if f not in referenced}
    seeds |= {f for f in texts for u in unresolved
              if _os.path.basename(f) == u + ".rs" or f.endswith(_os.sep + u + _os.sep + "mod.rs")}
    production, frontier = set(), sorted(seeds)
    while frontier:
        f = frontier.pop()
        if f in production:
            continue
        production.add(f)
        frontier += [c for c, is_test in children.get(f, []) if not is_test]
    test_files = sorted(set(texts) - production)
    out = []
    for f in files:
        if f not in production:
            continue
        spans = [d["span"] for d in decls[f] if d["cfg_test"] and d["kind"] == "{"]
        for x in string_literals(texts[f], f):
            if not any(s <= x["start"] < e for s, e in spans):
                out.append(dict(x, file=f))
    return out, test_files


def long_literal_check(root, crate_dirs, threshold, entries):
    """§8.45: every production literal longer than `threshold` bytes must match exactly one reviewed
    disposition entry ({file, head, disposition, cite}: the literal's value starts with `head`), and
    every entry must match exactly one such literal. Returns (matched, problems)."""
    lits, _ = production_literals(root, scope_files(root, crate_dirs))
    long_ = [x for x in lits if x["nbytes"] > threshold]
    problems, matched, used = [], [], {}
    for x in long_:
        v = x["value"].decode("latin-1") if isinstance(x["value"], bytes) else x["value"]
        hits = [i for i, e in enumerate(entries) if e["file"] == x["file"] and v.startswith(e["head"])]
        if len(hits) != 1:
            problems.append(f"{x['file']}:{x['line']}: {x['nbytes']} B literal > {threshold} B with "
                            f"{len(hits)} disposition entries")
            continue
        used.setdefault(hits[0], []).append(x)
        matched.append({"at": f"{x['file']}:{x['line']}", "bytes": x["nbytes"], **entries[hits[0]]})
    for i, e in enumerate(entries):
        if len(used.get(i, [])) != 1:
            problems.append(f"disposition entry {e['file']} {e['head']!r} matches {len(used.get(i, []))} long literals")
    return matched, problems
