"""RV83 independent text-site extraction (stdlib only, read-only).

Method (independent of the I65 script): a small Rust lexer that removes comments,
recognises string/raw-string/byte-string/char literals versus lifetimes, drops every
item or statement carrying a `#[cfg(test)]` attribute (to its matching `}` or `;`),
and then finds, in code tokens only:
  - format-family macro invocations: format!, write!, writeln!, format_args!,
    panic!, unreachable!, todo!, unimplemented!, assert!/assert_eq!/assert_ne!
    (the last group is reported separately; they abort rather than return text);
  - free calls `diag(` that are not `fn diag(` and not method calls `.diag(`;
  - `.to_string()` calls (reported as a count only: allocation sites that are not
    templates, e.g. literal or Display conversions).
For each format-family site the first string-literal argument is taken as the
template and its `{...}` placeholders are classified by format spec.
Usage: python3 rv83_text_sites.py <repo-relative root> <file> [<file> ...]
Prints JSON with per-file counts and, for each site, file:line, macro, spec list.
"""
import json, os, re, sys

FMT = {"format", "write", "writeln", "format_args"}
ABORT = {"panic", "unreachable", "todo", "unimplemented", "assert", "assert_eq", "assert_ne",
         "debug_assert", "debug_assert_eq", "debug_assert_ne"}


def lex(text):
    """Yield (kind, value, offset). kinds: id, str, punct. Comments are dropped."""
    i, n = 0, len(text)
    while i < n:
        c = text[i]
        if c.isspace():
            i += 1
            continue
        if text.startswith("//", i):
            j = text.find("\n", i)
            i = n if j < 0 else j
            continue
        if text.startswith("/*", i):
            depth, i = 1, i + 2
            while i < n and depth:
                if text.startswith("/*", i):
                    depth += 1; i += 2
                elif text.startswith("*/", i):
                    depth -= 1; i += 2
                else:
                    i += 1
            continue
        m = re.match(r'(b?r)(#*)"', text[i:i + 300])
        if m:
            hashes = m.group(2)
            start = i + m.end()
            end = text.find('"' + hashes, start)
            yield ("str", text[start:end], i)
            i = end + 1 + len(hashes)
            continue
        if c == '"' or (c == "b" and i + 1 < n and text[i + 1] == '"'):
            j = i + (2 if c == "b" else 1)
            buf = []
            while text[j] != '"':
                if text[j] == "\\":
                    buf.append(text[j:j + 2]); j += 2
                else:
                    buf.append(text[j]); j += 1
            yield ("str", "".join(buf), i)
            i = j + 1
            continue
        if c == "'":
            # char literal or lifetime
            m = re.match(r"'(\\u\{[0-9a-fA-F]+\}|\\x[0-9a-fA-F]{2}|\\.|[^\\'])'", text[i:i + 16])
            if m:
                yield ("chr", m.group(0), i)
                i += m.end()
                continue
            m = re.match(r"'[A-Za-z_][A-Za-z0-9_]*", text[i:])
            yield ("life", m.group(0), i)
            i += m.end()
            continue
        if c.isalpha() or c == "_":
            m = re.match(r"[A-Za-z_][A-Za-z0-9_]*", text[i:])
            yield ("id", m.group(0), i)
            i += m.end()
            continue
        if c.isdigit():
            m = re.match(r"[0-9][0-9A-Za-z_.]*", text[i:])
            v = m.group(0)
            # do not swallow a range or method dot: stop at '..' or '.ident'
            k = re.match(r"[0-9][0-9_]*(\.[0-9][0-9_]*)?([eE][+-]?[0-9_]+)?[A-Za-z0-9_]*", text[i:])
            yield ("num", k.group(0), i)
            i += len(k.group(0))
            continue
        yield ("punct", c, i)
        i += 1


def strip_cfg_test(tokens):
    """Remove every item/statement that carries #[cfg(test)]."""
    out, i, n = [], 0, len(tokens)
    while i < n:
        t = tokens[i]
        if (t[1] == "#" and i + 6 < n and tokens[i + 1][1] == "[" and tokens[i + 2][1] == "cfg"
                and tokens[i + 3][1] == "(" and tokens[i + 4][1] == "test" and tokens[i + 5][1] == ")"
                and tokens[i + 6][1] == "]"):
            j = i + 7
            depth = 0
            while j < n:
                v = tokens[j][1] if tokens[j][0] == "punct" else None
                if v in ("(", "["):
                    depth += 1
                elif v in (")", "]"):
                    depth -= 1
                elif v == "{" and depth == 0:
                    # skip the braced body
                    d = 0
                    while j < n:
                        w = tokens[j][1] if tokens[j][0] == "punct" else None
                        if w == "{":
                            d += 1
                        elif w == "}":
                            d -= 1
                            if d == 0:
                                break
                        j += 1
                    # a statement like `#[cfg(test)] foo(|| { .. });` continues to ';'
                    if j + 1 < n and tokens[j + 1][1] in (")", ";"):
                        while j < n and tokens[j][1] != ";":
                            j += 1
                    break
                elif v == ";" and depth == 0:
                    break
                j += 1
            i = j + 1
            continue
        out.append(t)
        i += 1
    return out


PH = re.compile(r"\{\{|\}\}|\{([^{}]*)\}")


def specs(template):
    out = []
    for m in PH.finditer(template):
        if m.group(0) in ("{{", "}}"):
            continue
        name, _, spec = m.group(1).partition(":")
        out.append({"name": name, "spec": spec})
    return out


def scan(path, rel):
    text = open(path, encoding="utf-8").read()
    line_of = lambda off: text.count("\n", 0, off) + 1
    toks = strip_cfg_test(list(lex(text)))
    sites, aborts, to_string = [], 0, 0
    for k, (kind, val, off) in enumerate(toks):
        if kind != "id":
            continue
        nxt = toks[k + 1][1] if k + 1 < len(toks) else ""
        prv = toks[k - 1][1] if k > 0 else ""
        if nxt == "!" and k + 2 < len(toks) and toks[k + 2][1] in ("(", "[", "{"):
            if val in FMT:
                tpl = next((t[1] for t in toks[k + 3:k + 12] if t[0] == "str"), None)
                sites.append({"file": rel, "line": line_of(off), "kind": val,
                              "template": tpl, "placeholders": specs(tpl) if tpl is not None else None})
            elif val in ABORT:
                aborts += 1
        elif val == "diag" and nxt == "(" and prv not in ("fn", ".", "::"):
            sites.append({"file": rel, "line": line_of(off), "kind": "diag"})
        elif val == "to_string" and prv == "." and nxt == "(":
            to_string += 1
    return sites, aborts, to_string


def main():
    root = sys.argv[1]
    report = {"files": {}, "sites": []}
    for rel in sys.argv[2:]:
        s, a, ts = scan(os.path.join(root, rel), rel)
        counts = {}
        for x in s:
            counts[x["kind"]] = counts.get(x["kind"], 0) + 1
        hist = {}
        for x in s:
            for p in (x.get("placeholders") or []):
                hist[p["spec"]] = hist.get(p["spec"], 0) + 1
        report["files"][rel] = {"sites": counts, "abort_macros": a, "to_string_calls": ts,
                                "placeholder_specs": hist}
        report["sites"].extend(s)
    print(json.dumps(report, indent=1))


main()
