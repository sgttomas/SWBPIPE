"""RV83 probe: method calls that I65's call graph never records (stdlib only, read-only).

I65's callgraph.py records a method call only when the text immediately before
`.name(` is `self` or an identifier path (receiver regex at callgraph.py:353); the
generic METHOD regex (:41) is defined but unused. So `f(x).name(`, `x.unwrap().name(`,
`x?.name(` and `(expr).name(` add no edge at all. This probe scans every function
reachable on I65's graph for such chained calls (comments and string literals blanked)
and reports the same-named definitions that are not reachable on I65's graph, the
functions reachable only through them, and the text rows those functions hold.
Usage (from P): python3 rv83_chained_calls.py <G3 _run_records dir> <out json>
"""
import json, os, re, sys, collections

g3, out_p = sys.argv[1], sys.argv[2]
cg = json.load(open(os.path.join(g3, "callgraph_edges.json")))
edges, spans = cg["edges"], cg["spans"]
tb = json.load(open(os.path.join(g3, "text_budget.caps.out.json")))
roots = tb["roots"]

def bfs(start, base=frozenset()):
    seen, todo = set(), list(start)
    while todo:
        v = todo.pop()
        if v in seen or v in base:
            continue
        seen.add(v)
        todo.extend(edges.get(v, []))
    return seen

reach = bfs(roots)
by_name = collections.defaultdict(list)
for k in edges:
    by_name[k.rsplit(":", 1)[1]].append(k)

def blank(text):
    """Blank comments and string/char literals, keeping offsets."""
    out, i, n = list(text), 0, len(text)
    def wipe(a, b):
        for j in range(a, b):
            if out[j] != "\n":
                out[j] = " "
    while i < n:
        if text.startswith("//", i):
            j = text.find("\n", i); j = n if j < 0 else j; wipe(i, j); i = j
        elif text.startswith("/*", i):
            j = text.find("*/", i + 2); j = n if j < 0 else j + 2; wipe(i, j); i = j
        elif text[i] == '"' or (text[i] == "r" and re.match(r'r#*"', text[i:i + 8])):
            m = re.match(r'r(#*)"', text[i:i + 8])
            if m:
                close = '"' + m.group(1)
                j = text.find(close, i + m.end()); j = n if j < 0 else j + len(close)
            else:
                j = i + 1
                while j < n and text[j] != '"':
                    j += 2 if text[j] == "\\" else 1
                j += 1
            wipe(i, j); i = j
        else:
            i += 1
    return "".join(out)

cache = {}
CALLM = re.compile(r"\.\s*([a-z_][a-z0-9_]*)\s*(?:::<[^>]*>)?\s*\(")
RECV = re.compile(r"(self|[A-Za-z_][A-Za-z0-9_\]\[\.]*)\s*$")
found = collections.defaultdict(list)
for k in sorted(reach):
    rel, brace, end = spans[k]
    if rel not in cache:
        cache[rel] = blank(open(rel, encoding="utf-8").read())
    body = cache[rel][brace:end]
    for m in CALLM.finditer(body):
        before = body[:m.start()].rstrip()
        if not before or before[-1] not in ")?":
            continue                       # receiver form that I65's resolver does parse
        name = m.group(1)
        defs = [d for d in by_name.get(name, []) if d not in reach]
        if defs:
            line = cache[rel].count("\n", 0, brace + m.start()) + 1
            found[name].append(f"{rel}:{line} in {k.rsplit(':', 1)[1]}")
hidden_roots = [d for name in found for d in by_name[name] if d not in reach]
hidden = bfs(hidden_roots, base=frozenset(reach))
rows = [r for r in tb["rows"] if r["fn"] in hidden]
per_file = collections.Counter(r["file"] for r in rows)
json.dump({"reachable_on_I65_graph": len(reach),
           "chained_call_names_with_unreached_defs": {n: {"call_sites": v[:12], "n_sites": len(v),
                                                          "unreached_defs": [d for d in by_name[n] if d not in reach]}
                                                      for n, v in sorted(found.items())},
           "hidden_function_count": len(hidden), "hidden_text_rows": len(rows),
           "hidden_text_rows_per_file": per_file,
           "hidden_text_rows_detail": [{x: r[x] for x in ("file", "line", "kind", "fn", "bytes")} for r in rows]},
          open(out_p, "w"), indent=1)
print(len(reach), len(found), len(hidden), len(rows))
for f, c in per_file.most_common(40):
    print(c, f)
