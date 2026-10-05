"""I65 U4 G3: lexical inventory of heap-allocation sites with loop context (stdlib only, read-only).

For each non-test source file, list every token that can request heap memory:
vec!, Vec::new/with_capacity, .collect, .to_vec, .to_owned, .to_string,
String::new/from/with_capacity, format!, .clone(), Box::new, Arc::new, Rc::new,
HashMap/HashSet/BTreeMap/BTreeSet constructors, .push/.extend/.insert/.reserve
(growth), .into_boxed_slice, json!.
Each row carries the enclosing fn and the stack of enclosing loop headers
(`for`/`while`/`loop`, by brace nesting), so a reviewer can attach a count
bound. It is a lexical lead: `.clone()` of a Copy or Arc value allocates nothing,
and a site inside a closure counts toward the closure's caller. Test modules
(`#[cfg(test)] mod`) and `#[cfg(test)]` items are skipped.
Usage: python3 alloc_sites.py <repo-relative root> <file> [<file> ...]
"""
import json, os, re, sys

TOK = re.compile(r"vec!\s*\[|Vec::new\(|Vec::with_capacity\(|\.collect(?:::<[^>]*>)?\(|\.to_vec\(|\.to_owned\(|"
                 r"\.to_string\(|String::new\(|String::from\(|String::with_capacity\(|format!\(|\.clone\(\)|"
                 r"Box::new\(|Arc::new\(|Rc::new\(|HashMap::|HashSet::|BTreeMap::|BTreeSet::|\.push\(|\.extend\(|"
                 r"\.insert\(|\.reserve(?:_exact)?\(|\.into_boxed_slice\(|json!\(|\.into_iter\(\)\.map|\.repeat\(")
FN = re.compile(r"\bfn\s+([A-Za-z_][A-Za-z0-9_]*)")
LOOP = re.compile(r"(?<![A-Za-z0-9_])(for\s+[^{]+?\s+in\s+[^{]+|while\s+[^{]+|loop)\s*\{")

def strip_tests(text):
    m = re.search(r"#\[cfg\(test\)\]\s*\n\s*(pub(\(crate\))?\s+)?mod\s+\w+\s*\{", text)
    return text if m is None else text[:m.start()]

root = sys.argv[1]
rows = []
for rel in sys.argv[2:]:
    raw = open(os.path.join(root, rel), encoding="utf-8").read()
    text = strip_tests(raw)
    # blank out comments and string contents (keep length) so braces/tokens inside them are ignored
    clean = re.sub(r"//[^\n]*", lambda m: " " * len(m.group(0)), text)
    clean = re.sub(r'"(?:\\.|[^"\\])*"', lambda m: '"' + " " * (len(m.group(0)) - 2) + '"', clean)
    events = []
    for m in FN.finditer(clean):
        events.append((m.start(), "fn", m.group(1)))
    for m in LOOP.finditer(clean):
        events.append((m.end() - 1, "loop", " ".join(m.group(1).split())[:120]))
    events.sort()
    # brace scan with a stack of (kind, label, depth)
    stack, depth, ev_i, pending = [], 0, 0, None
    loop_at = {pos: lab for pos, kind, lab in events if kind == "loop"}
    fn_starts = [(pos, lab) for pos, kind, lab in events if kind == "fn"]
    fi = 0
    tokens = {m.start(): m.group(0) for m in TOK.finditer(clean)}
    for i, c in enumerate(clean):
        while fi < len(fn_starts) and fn_starts[fi][0] <= i:
            pending = fn_starts[fi][1]; fi += 1
        if c == "{":
            depth += 1
            if i in loop_at:
                stack.append(("loop", loop_at[i], depth))
            elif pending is not None:
                stack.append(("fn", pending, depth)); pending = None
        elif c == "}":
            while stack and stack[-1][2] == depth:
                stack.pop()
            depth -= 1
        elif c == ";" and pending is not None and not any(k == "fn" for k, _, _ in stack[-1:]):
            pending = None   # trait method declaration without body
        if i in tokens:
            fns = [lab for k, lab, _ in stack if k == "fn"]
            loops = [lab for k, lab, _ in stack if k == "loop"]
            line = text.count("\n", 0, i) + 1
            rows.append({"file": rel, "line": line, "token": tokens[i].rstrip("(["),
                         "fn": fns[-1] if fns else "?", "outer_fns": fns[:-1], "loops": loops})
counts = {}
for r in rows:
    counts[r["file"]] = counts.get(r["file"], 0) + 1
print(json.dumps({"files": sys.argv[2:], "counts": counts, "rows": rows}, indent=1))
