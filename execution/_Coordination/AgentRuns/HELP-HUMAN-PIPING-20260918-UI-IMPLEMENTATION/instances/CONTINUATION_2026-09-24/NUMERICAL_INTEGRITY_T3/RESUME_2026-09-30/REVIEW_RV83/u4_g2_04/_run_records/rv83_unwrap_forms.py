"""RV83 probe: unwrap-idiom rebindings of a parameter that R4c does not recognise.
R4c (callgraph_g5.py) types `p` as the payload only for `let Some(p) = p` with an optional
`.as_mut()`/`.as_ref()`/`.as_deref()`/`.take()`. This probe finds, in every function reachable
on the R-4 graph, `let Some(p)`/`Ok(p)` = `p.<m>()` for any other method chain (for example
`.as_deref_mut()`), or `match p { Some(p) => .. }`, where `p` is a parameter typed
Option<..>/Result<..>, and lists the calls `p.f(` in the body that have no edge.
Run from the snapshot's projects/chirality-piping.
Usage: python3 rv83_unwrap_forms.py <edges_r4.json> <text_budget.caps.r4.out.json> <out json>
"""
import json, re, sys, collections
ep, tp, out_p = sys.argv[1:4]
cg = json.load(open(ep)); E, S = cg["edges"], cg["spans"]
tb = json.load(open(tp))
reach, todo = set(tb["roots"]), list(tb["roots"])
while todo:
    v = todo.pop()
    for w in E.get(v, []):
        if w not in reach:
            reach.add(w); todo.append(w)
by = collections.defaultdict(list)
for k in E:
    by[k.rsplit(":", 1)[1]].append(k)
handled = {"as_mut", "as_ref", "as_deref", "take"}
src, found = {}, []
for k in sorted(reach):
    rel, a, b = S[k]
    if rel not in src:
        src[rel] = open(rel, encoding="utf-8").read()
    t = src[rel]
    head = t[t.rfind("fn ", 0, a):a]
    body = t[a:b]
    params = {}
    for pm in re.finditer(r"[(,]\s*(?:mut\s+)?([a-z_][A-Za-z0-9_]*)\s*:\s*([^,)]*(?:<[^>]*>[^,)]*)?)", head):
        params[pm.group(1)] = pm.group(2)
    for um in re.finditer(r"\blet\s+(?:Some|Ok)\s*\(\s*(?:mut\s+|ref\s+(?:mut\s+)?)?([a-z_][A-Za-z0-9_]*)\s*\)\s*=\s*([a-z_][A-Za-z0-9_]*)((?:\s*\.\s*[a-z_][a-z0-9_]*\s*\(\s*\))*)", body):
        p, src_name, chain = um.group(1), um.group(2), um.group(3)
        if p != src_name or p not in params or not re.search(r"\b(Option|Result)\s*<", params[p]):
            continue
        methods = re.findall(r"\.\s*([a-z_][a-z0-9_]*)", chain)
        if len(methods) <= 1 and (not methods or methods[0] in handled):
            continue                                   # a form R4c handles
        for cm in re.finditer(r"\b" + p + r"\s*\.\s*([a-z_][a-z0-9_]*)\s*\(", body[um.end():]):
            name = cm.group(1)
            defs = by.get(name, [])
            if defs and not any(d in E.get(k, []) for d in defs):
                line = t.count("\n", 0, a + um.end() + cm.start()) + 1
                found.append({"caller": k, "line": f"{rel}:{line}", "param": p, "param_type": params[p],
                              "unwrap_chain": "".join("." + m + "()" for m in methods), "call": name,
                              "defs": defs, "defs_reachable": [d in reach for d in defs]})
json.dump({"found": found}, open(out_p, "w"), indent=1)
print(len(found))
for f in found:
    print(f["line"].split("src/")[-1], f["param"], ":", f["param_type"], f["unwrap_chain"], "->", f["call"], f["defs_reachable"], [d.split("src/")[-1] for d in f["defs"]])
