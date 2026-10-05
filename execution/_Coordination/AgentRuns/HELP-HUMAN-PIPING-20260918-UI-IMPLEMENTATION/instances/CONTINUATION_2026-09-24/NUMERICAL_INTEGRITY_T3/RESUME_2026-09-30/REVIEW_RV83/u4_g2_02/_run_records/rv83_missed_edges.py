"""RV83 probe: method calls on generic-typed receivers that I65's call graph drops.

Read-only, stdlib only. Inputs are I65's committed G3 records:
  callgraph_edges.json (edges, spans) and text_budget.caps.out.json (rows).
For every function reachable from the two D1 roots on I65's graph, scan its body for
`recv.name(` where `recv` is a parameter whose declared type is a bare generic
parameter of that function (e.g. `equations: &R` with `<R: Trait>`). I65's resolver
maps such a receiver to a type named `R` with no methods, so the edge is dropped
instead of fanning out. Report each such call whose same-named definitions are
NOT reachable on I65's graph, and the text rows that live in those definitions.
Usage (from P): python3 rv83_missed_edges.py <G3 _run_records dir> <out json>
"""
import json, os, re, sys, collections

g3, out_p = sys.argv[1], sys.argv[2]
cg = json.load(open(os.path.join(g3, "callgraph_edges.json")))
edges, spans = cg["edges"], cg["spans"]
tb = json.load(open(os.path.join(g3, "text_budget.caps.out.json")))
roots = tb["roots"]

reach, todo = set(roots), list(roots)
while todo:
    v = todo.pop()
    for w in edges.get(v, []):
        if w not in reach:
            reach.add(w); todo.append(w)

by_name = collections.defaultdict(list)
for k in edges:
    by_name[k.rsplit(":", 1)[1]].append(k)

texts = {}
def text_of(rel):
    if rel not in texts:
        texts[rel] = open(rel, encoding="utf-8").read()
    return texts[rel]

SIG = re.compile(r"\bfn\s+([A-Za-z_][A-Za-z0-9_]*)\s*(<[^{;]*?>)?\s*\(([^{;]*?)\)\s*(?:->[^{;]*?)?(where[^{;]*)?\{", re.S)
missed = []
for k in sorted(reach):
    rel, brace, end = spans[k]
    text = text_of(rel)
    # signature: from the `fn` keyword preceding the body's opening brace
    head_start = text.rfind("fn ", 0, brace)
    head = text[head_start:brace + 1]
    m = SIG.search(head)
    if not m:
        continue
    generics = set(re.findall(r"\b([A-Z][A-Za-z0-9_]*)\s*:", (m.group(2) or "") + " " + (m.group(4) or "")))
    generics |= set(re.findall(r"[<,]\s*([A-Z])\s*[,>]", m.group(2) or ""))
    params = {}
    for part in m.group(3).split(","):
        if ":" in part:
            name, ty = part.split(":", 1)
            ty = re.sub(r"^&\s*('\w+\s+)?(mut\s+)?", "", ty.strip())
            params[name.replace("mut ", "").strip()] = ty.strip()
    gparams = {p for p, ty in params.items() if ty in generics or re.fullmatch(r"(dyn|impl)\s+.*", ty or "")}
    if not gparams:
        continue
    body = text[brace:end]
    for cm in re.finditer(r"\b([a-z_][a-z0-9_]*)\s*\.\s*([a-z_][a-z0-9_]*)\s*(?:::<[^>]*>)?\s*\(", body):
        recv, name = cm.group(1), cm.group(2)
        if recv not in gparams:
            continue
        defs = [d for d in by_name.get(name, []) if d != k]
        if not defs:
            continue
        unreached = [d for d in defs if d not in reach]
        if unreached and not any(d in edges.get(k, []) for d in defs):
            line = text.count("\n", 0, brace + cm.start()) + 1
            missed.append({"caller": k, "call_line": f"{rel}:{line}", "recv": recv,
                           "recv_type": params[recv], "method": name, "unreached_defs": unreached})

# functions newly reachable if every missed edge fanned out, and their text rows
extra, todo = set(), [d for x in missed for d in x["unreached_defs"]]
while todo:
    v = todo.pop()
    if v in reach or v in extra:
        continue
    extra.add(v)
    todo.extend(edges.get(v, []))
rows = [r for r in tb["rows"] if r["fn"] in extra]
per_file = collections.Counter(r["file"] for r in rows)
json.dump({"reachable_on_I65_graph": len(reach), "missed_generic_receiver_calls": missed,
           "functions_hidden_behind_them": sorted(extra), "hidden_function_count": len(extra),
           "hidden_text_rows": len(rows), "hidden_text_rows_per_file": per_file,
           "hidden_text_rows_detail": [{k: r[k] for k in ("file", "line", "kind", "fn")} for r in rows]},
          open(out_p, "w"), indent=1)
print(len(reach), len(missed), len(extra), len(rows))
for f, c in per_file.most_common(30):
    print(c, f)

# Part 2: dropped generic-receiver calls whose same-named definitions ARE reachable by
# another path. Their multiplicity misses the dropped caller; report the text bytes
# (I65's own per-row `req`) below each such definition, to size the undercount.
under = []
for k in sorted(reach):
    rel, brace, end = spans[k]
    text = text_of(rel)
    head = text[text.rfind("fn ", 0, brace):brace + 1]
    m = SIG.search(head)
    if not m:
        continue
    generics = set(re.findall(r"\b([A-Z][A-Za-z0-9_]*)\s*:", (m.group(2) or "") + " " + (m.group(4) or "")))
    params = {}
    for part in m.group(3).split(","):
        if ":" in part:
            name, ty = part.split(":", 1)
            ty = re.sub(r"^&\s*('\w+\s+)?(mut\s+)?", "", ty.strip())
            params[name.replace("mut ", "").strip()] = ty.strip()
    gparams = {p for p, ty in params.items() if ty in generics}
    body = text[brace:end]
    for cm in re.finditer(r"\b([a-z_][a-z0-9_]*)\s*\.\s*([a-z_][a-z0-9_]*)\s*(?:::<[^>]*>)?\s*\(", body):
        recv, name = cm.group(1), cm.group(2)
        if recv not in gparams:
            continue
        defs = [d for d in by_name.get(name, []) if d in reach and d != k]
        if defs and not any(d in edges.get(k, []) for d in defs):
            sub = set()
            todo2 = list(defs)
            while todo2:
                v = todo2.pop()
                if v in sub:
                    continue
                sub.add(v)
                todo2.extend(edges.get(v, []))
            req = sum(r.get("req", 0) for r in tb["rows"] if r["fn"] in sub)
            under.append({"caller": k, "line": text.count("\n", 0, brace + cm.start()) + 1,
                          "method": name, "reachable_defs": defs, "text_req_below_defs": req})
out = json.load(open(out_p))
out["dropped_calls_to_reachable_defs"] = under
json.dump(out, open(out_p, "w"), indent=1)
print("dropped calls to reachable defs:", len(under))
for u in under:
    print(u["caller"].rsplit(":", 1)[1], u["line"], u["method"], u["text_req_below_defs"])
