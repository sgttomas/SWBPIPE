"""I65 U4 G2: inventory of text templates on the ordinary route (stdlib only, read-only).

For each non-test source file given, list every format-family macro call
(format!, write!, writeln!, format_args!, panic! excluded) and every diag(...)
call with: line, the literal template, its literal byte length (placeholders
removed, `{{`/`}}` counted as one byte), and each placeholder's spec and argument
text. A placeholder's argument is matched positionally or by inline name. This
is a lexical extraction for the T08 template table; it does not decide
reachability or multiplicity (G3 attaches multiplicity per site).
Usage: python3 template_inventory.py <repo-relative root> <file> [<file> ...]
"""
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

root = sys.argv[1]
MACROS = ("format!", "write!", "writeln!", "format_args!")

def balanced(text, start):
    """Return the index just past the parenthesis group that opens at text[start]."""
    depth, i, in_str, esc = 0, start, False, False
    while i < len(text):
        c = text[i]
        if in_str:
            if esc:
                esc = False
            elif c == "\\":
                esc = True
            elif c == '"':
                in_str = False
        else:
            if c == '"':
                in_str = True
            elif c in "([{":
                depth += 1
            elif c in ")]}":
                depth -= 1
                if depth == 0:
                    return i + 1
        i += 1
    return len(text)

def split_args(inner):
    args, depth, cur, in_str, esc = [], 0, "", False, False
    for c in inner:
        if in_str:
            cur += c
            if esc:
                esc = False
            elif c == "\\":
                esc = True
            elif c == '"':
                in_str = False
            continue
        if c == '"':
            in_str = True
            cur += c
        elif c in "([{":
            depth += 1; cur += c
        elif c in ")]}":
            depth -= 1; cur += c
        elif c == "," and depth == 0:
            args.append(cur.strip()); cur = ""
        else:
            cur += c
    if cur.strip():
        args.append(cur.strip())
    return args

PH = re.compile(r"\{\{|\}\}|\{([^{}]*)\}")

def template_facts(lit, rest):
    body = bytes(lit, "utf-8").decode("unicode_escape") if "\\" in lit else lit
    literal_bytes, placeholders, pos = 0, [], 0
    last = 0
    for m in PH.finditer(body):
        literal_bytes += len(body[last:m.start()].encode())
        last = m.end()
        if m.group(0) in ("{{", "}}"):
            literal_bytes += 1
            continue
        spec = m.group(1)
        name, _, fmt = spec.partition(":")
        if name == "":
            arg = rest[pos] if pos < len(rest) else "?"
            pos += 1
        elif name.isdigit():
            arg = rest[int(name)] if int(name) < len(rest) else "?"
        else:
            arg = next((a.split("=", 1)[1].strip() for a in rest if a.split("=", 1)[0].strip() == name), name)
        placeholders.append({"spec": fmt, "arg": arg})
    literal_bytes += len(body[last:].encode())
    return literal_bytes, placeholders

rows = []
for rel in sys.argv[2:]:
    text = open(os.path.join(root, rel), encoding="utf-8").read()
    # G4 (R1g/R1k): keep the raw text for literals, but skip any macro inside a comment, a
    # string or a test-gated item, wherever it sits (G3 cut at the first test module)
    import loopscan
    mask = loopscan.blank_test_items(loopscan.blank_rust(text))
    scan = text
    for macro in MACROS + ("diag(",):
        for m in re.finditer(re.escape(macro), scan):
            if mask[m.start():m.start() + len(macro)] != scan[m.start():m.start() + len(macro)]:
                continue
            if macro == "diag(" and (scan[m.start() - 1].isalnum() or scan[m.start() - 1] == "_" or scan[max(0, m.start() - 3):m.start()] == "fn "):
                continue
            open_at = m.end() - 1 if macro == "diag(" else scan.find("(", m.end() - 1)
            if open_at < 0:
                continue
            end = balanced(scan, open_at)
            args = split_args(scan[open_at + 1:end - 1])
            line = scan.count("\n", 0, m.start()) + 1
            row = {"file": rel, "line": line, "kind": macro.rstrip("!(")}
            if macro == "diag(":
                row["code"] = args[1].strip('"') if len(args) > 1 else "?"
                row["severity"] = args[2].strip('"') if len(args) > 2 else "?"
                row["message_arg"] = args[3][:200] if len(args) > 3 else "?"
            else:
                lits = [i for i, a in enumerate(args) if a.startswith('"')]
                if not lits:
                    continue
                k = lits[0]
                lit = args[k][1:-1]
                lb, ph = template_facts(lit, args[k + 1:])
                row.update({"template": lit[:240], "literal_bytes": lb, "placeholders": ph})
            rows.append(row)
summary = {}
for r in rows:
    summary[(r["file"], r["kind"])] = summary.get((r["file"], r["kind"]), 0) + 1
print(json.dumps({"files": sys.argv[2:],
                  "counts": [{"file": f, "kind": k, "n": n} for (f, k), n in sorted(summary.items())],
                  "rows": rows}, indent=1))
