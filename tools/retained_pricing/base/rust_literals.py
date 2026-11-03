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

    # -- the walk ------------------------------------------------------------------------------
    def run(self):
        t, n = self.t, self.n
        if "\r" in t:
            self.fail(t.index("\r"), "carriage return in source (CRLF normalization is not modelled)")
        out, i = [], 0
        if t.startswith("#!") and not t.startswith("#!["):
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
