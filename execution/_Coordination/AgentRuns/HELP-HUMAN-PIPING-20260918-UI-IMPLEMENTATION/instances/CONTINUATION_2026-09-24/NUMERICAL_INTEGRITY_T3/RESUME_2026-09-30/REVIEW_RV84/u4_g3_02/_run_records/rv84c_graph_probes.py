"""RV84 confirmation (u4_g3_02): probes of I65's repaired G4 call graph (stdlib only).
(1) colon-adjacent calls (S-1): every `:f(` (not `::f(`) in a reached fn body whose name is
    defined in the graph, and whether the caller->name edge now exists;
(2) chained calls (RV83 R-1(b) / my G3 chain probe): `).g(` / `?.g(` names absent as edges;
(3) a blunt fan-out of every `.name(` by name (my G3 augment probe) re-run on the G4 graph:
    the multi-node SCCs it creates are the candidate collisions the G4 rules must explain.
Usage: python3 rv84c_graph_probes.py <G4 _run_records> <P root at b1f80234dc>"""
import json, re, os, sys, collections
G4, SRC = sys.argv[1], sys.argv[2]
cg = json.load(open(os.path.join(G4, "callgraph_edges.json")))
tb = json.load(open(os.path.join(G4, "text_budget.caps.out.json")))
edges = {k: set(v) for k, v in cg["edges"].items()}
nodes = list(edges)
fm = tb["function_multiplicity"]
root = [k for k in nodes if k.endswith(":run_linear_static_preview_value_with_retained_direct")]
reach, todo = set(root), list(root)
while todo:
    v = todo.pop()
    for w in edges[v]:
        if w not in reach: reach.add(w); todo.append(w)
crate = {k: k.split("/src/")[0] for k in nodes}
byname = collections.defaultdict(list)
for k in nodes: byname[k.rsplit(":", 1)[1]].append(k)
def blank(t):
    t = re.sub(r"//[^\n]*", lambda m: " " * len(m.group(0)), t)
    t = re.sub(r"/\*.*?\*/", lambda m: " " * len(m.group(0)), t, flags=re.S)
    t = re.sub(r'"(?:\\.|[^"\\])*"', lambda m: '"' + " " * (len(m.group(0)) - 2) + '"', t, flags=re.S)
    return t
cache = {}
def body(key):
    rel, line, name = key.rsplit(":", 2)
    if rel not in cache: cache[rel] = blank(open(os.path.join(SRC, rel), encoding="utf-8").read())
    t = cache[rel]; pos = 0
    for _ in range(int(line) - 1): pos = t.index("\n", pos) + 1
    m = re.compile(r"\bfn\s+" + re.escape(name) + r"\b").search(t, pos)
    if not m: return ""
    b = t.find("{", m.end()); d = 0; i = b
    while i < len(t):
        if t[i] == "{": d += 1
        elif t[i] == "}":
            d -= 1
            if d == 0: break
        i += 1
    return t[b:i]
COLON = re.compile(r"(?<!:):\s*([A-Za-z_][A-Za-z0-9_]*)\s*(?:::<[^>]*>)?\s*\(")
CHAIN = re.compile(r"[)?]\s*\.\s*([A-Za-z_][A-Za-z0-9_]*)\s*(?:::<[^>]*>)?\s*\(")
colon_missing, chain_missing = [], []
for k in sorted(reach):
    bd = body(k)
    have = {t.rsplit(":", 1)[1] for t in edges[k]}
    for m in COLON.finditer(bd):
        nm = m.group(1)
        if nm in byname and nm not in have and nm[0].islower():
            colon_missing.append((k, nm))
    for m in CHAIN.finditer(bd):
        nm = m.group(1)
        if nm in byname and nm not in have:
            chain_missing.append((k, nm))
out = {"reach": len(reach),
       "colon_missing": sorted(set(colon_missing)), "chain_missing_names": sorted({n for _, n in chain_missing}),
       "chain_missing": sorted(set(chain_missing))}
# specific S-1 edges
want = [("source.rs:252:commitment", n) for n in ("matrix", "matrix12", "products", "recipes")] + \
       [("source.rs:57:function", n) for n in ("lowered_products", "functional_id", "bits", "unit")]
spec = {}
for caller_suffix, nm in want:
    cs = [k for k in nodes if k.endswith(caller_suffix.split(":", 1)[0] + ":" + caller_suffix.split(":", 1)[1]) or k.endswith("/" + caller_suffix)]
    cs = [k for k in nodes if k.split("/")[-1].startswith(caller_suffix.split(":")[0]) and k.endswith(":" + caller_suffix.split(":")[-1]) and "source_receipt" in k]
    spec[f"{caller_suffix}->{nm}"] = [any(t.rsplit(":", 1)[1] == nm for t in edges[c]) for c in cs]
out["s1_specific_edges"] = spec
# blunt fan-out SCCs
aug = {k: set(v) for k, v in edges.items()}
METHOD = re.compile(r"\.\s*([A-Za-z_][A-Za-z0-9_]*)\s*(?:::<[^>]*>)?\s*\(")
for k in reach:
    bd = body(k)
    for m in METHOD.finditer(bd):
        for t in byname.get(m.group(1), []):
            if t != k: aug[k].add(t)
sys.setrecursionlimit(1000000)
idx, low, on, st, sccs, c = {}, {}, set(), [], [], [0]
def sc(v):
    work = [(v, iter(sorted(aug[v] & reach)))]
    idx[v] = low[v] = c[0]; c[0] += 1; st.append(v); on.add(v)
    while work:
        u, it = work[-1]; adv = False
        for w in it:
            if w not in idx:
                idx[w] = low[w] = c[0]; c[0] += 1; st.append(w); on.add(w)
                work.append((w, iter(sorted(aug[w] & reach)))); adv = True; break
            elif w in on: low[u] = min(low[u], idx[w])
        if adv: continue
        work.pop()
        if work: low[work[-1][0]] = min(low[work[-1][0]], low[u])
        if low[u] == idx[u]:
            comp = []
            while True:
                w = st.pop(); on.discard(w); comp.append(w)
                if w == u: break
            sccs.append(comp)
for v in sorted(reach):
    if v not in idx: sc(v)
multi = sorted((sorted(x) for x in sccs if len(x) > 1), key=len)
out["blunt_multi_node_sccs"] = [{"size": len(x), "members": x[:12]} for x in multi]
json.dump(out, open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "rv84c_graph_probes.out.json"), "w"), indent=1)
print("reach", len(reach), "colon_missing", len(set(colon_missing)), "chain_missing", len(set(chain_missing)), "blunt multi-node SCCs", len(multi), [len(x) for x in multi])
print("S-1 specific edges:", spec)
for k, n in sorted(set(colon_missing)): print("  colon:", k.split("/")[-1], "->", n)
for k, n in sorted(set(chain_missing))[:40]: print("  chain:", k.split("/")[-1], "->", n)
