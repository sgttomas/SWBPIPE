"""RV84: augment I65's call graph with every `.name(` method call (any receiver form, fan-out by
name within the crate-dependency closure) and recompute SCCs reachable from the D1 roots."""
import json, re, os, sys, collections
I65, SRC = sys.argv[1], sys.argv[2]
cg = json.load(open(os.path.join(I65, "callgraph_edges.json")))
edges = {k: set(v) for k, v in cg["edges"].items()}
nodes = list(edges)
crate = {k: k.split("/src/")[0] for k in nodes}
DEPS = {
 "core/product_physics": ["core/serialization/canonical_json", "core/solver/curved_bend", "core/solver/frame_kernel",
    "core/loads/load_case_algebra", "core/solver/linear_supports", "core/solver/nonlinear_integration",
    "core/solver/nonlinear_supports", "core/loads/primitive_loads", "core/solver/diagnostics",
    "core/solver/sparse_direct", "core/solver/straight_pipe", "core/loads/stress_recovery", "core/units"],
 "core/solver/sparse_direct": ["core/solver/frame_kernel"], "core/solver/linear_supports": ["core/solver/frame_kernel"],
 "core/solver/straight_pipe": ["core/solver/frame_kernel"],
 "core/solver/nonlinear_integration": ["core/solver/curved_bend", "core/solver/frame_kernel", "core/solver/nonlinear_supports", "core/solver/diagnostics", "core/solver/sparse_direct"],
 "core/solver/curved_bend": ["core/solver/frame_kernel"], "core/solver/nonlinear_supports": ["core/solver/linear_supports", "core/solver/diagnostics"],
 "core/solver/diagnostics": ["core/solver/frame_kernel", "core/solver/linear_supports", "core/loads/primitive_loads", "core/solver/sparse_direct"],
 "core/loads/primitive_loads": ["core/solver/frame_kernel", "core/solver/linear_supports"],
 "core/loads/stress_recovery": ["core/loads/primitive_loads", "core/solver/straight_pipe"],
 "core/loads/load_case_algebra": ["core/loads/primitive_loads", "core/solver/frame_kernel"],
 "core/reporting/result_export": ["core/serialization/canonical_json", "core/units"]}
def clo(c, s=None):
    s = set() if s is None else s
    if c in s: return s
    s.add(c)
    for d in DEPS.get(c, []): clo(d, s)
    return s
allowed = {c: clo(c) for c in set(crate.values())}
byname = collections.defaultdict(list)
for k in nodes: byname[k.rsplit(":", 1)[1]].append(k)
def strip(t):
    t = re.sub(r"//[^\n]*", lambda m: " " * len(m.group(0)), t)
    t = re.sub(r"/\*.*?\*/", lambda m: " " * len(m.group(0)), t, flags=re.S)
    t = re.sub(r'"(?:\\.|[^"\\])*"', lambda m: '"' + " " * (len(m.group(0)) - 2) + '"', t)
    return t
cache = {}
def body(key):
    rel, line, name = key.rsplit(":", 2)
    if rel not in cache: cache[rel] = strip(open(os.path.join(SRC, rel), encoding="utf-8").read())
    t = cache[rel]; pos = 0
    for _ in range(int(line) - 1): pos = t.index("\n", pos) + 1
    m = re.compile(r"\bfn\s+" + re.escape(name) + r"\b").search(t, pos)
    b = t.find("{", m.end()); d = 0; i = b
    while i < len(t):
        if t[i] == "{": d += 1
        elif t[i] == "}":
            d -= 1
            if d == 0: break
        i += 1
    return t[b:i]
METHOD = re.compile(r"\.\s*([A-Za-z_][A-Za-z0-9_]*)\s*(?:::<[^>]*>)?\s*\(")
STD = set("len is_empty iter iter_mut into_iter map filter collect clone push get insert contains extend unwrap expect ok_or ok_or_else map_err and_then as_ref as_mut as_str to_string to_owned sum max min abs sqrt cmp partial_cmp eq ne copied cloned zip enumerate rev take skip any all find position fold count chain flat_map filter_map last first sort sort_by sort_unstable dedup retain remove pop entry or_insert or_default keys values get_mut join split trim starts_with ends_with replace chars bytes push_str is_some is_none is_ok is_err unwrap_or unwrap_or_default unwrap_or_else checked_add checked_sub checked_mul checked_div saturating_add saturating_mul saturating_sub to_bits from_bits powi powf floor ceil round signum is_finite is_nan max_by min_by max_by_key min_by_key windows chunks contains_key with_capacity reserve truncate drain split_at swap fill copy_from_slice extend_from_slice to_vec into_boxed_slice as_slice as_ptr borrow borrow_mut set take replace_with then then_some write_str write_fmt fmt next peekable step_by flatten parse into try_into try_from from hash".split())
added = collections.Counter()
for k in nodes:
    try: bd = body(k)
    except Exception: continue
    have = {t.rsplit(":", 1)[1] for t in edges[k]}
    for m in METHOD.finditer(bd):
        nm = m.group(1)
        if nm in have or nm not in byname: continue
        for t in byname[nm]:
            if crate[t] in allowed.get(crate[k], {crate[k]}):
                if t not in edges[k]:
                    edges[k].add(t); added[nm] += 1
roots = [k for k in nodes if k.rsplit(":", 1)[1] in ("run_linear_static_preview_value_with_retained_direct", "prepare_observed")]
reach, todo = set(roots), list(roots)
while todo:
    v = todo.pop()
    for w in edges[v]:
        if w not in reach: reach.add(w); todo.append(w)
# Tarjan on reach
sys.setrecursionlimit(100000)
idx, low, on, st, sccs, c = {}, {}, set(), [], [], [0]
def sc(v):
    idx[v] = low[v] = c[0]; c[0] += 1; st.append(v); on.add(v)
    for w in edges[v]:
        if w not in reach: continue
        if w not in idx: sc(w); low[v] = min(low[v], low[w])
        elif w in on: low[v] = min(low[v], idx[w])
    if low[v] == idx[v]:
        comp = []
        while True:
            w = st.pop(); on.discard(w); comp.append(w)
            if w == v: break
        sccs.append(comp)
for v in sorted(reach):
    if v not in idx: sc(v)
cyc = [sorted(x) for x in sccs if len(x) > 1 or x[0] in edges[x[0]]]
orig = json.load(open(os.path.join(I65, "callgraph.out.json")))
orig_set = {tuple(x) for x in orig["cyclic_components"]}
new = [x for x in cyc if tuple(x) not in orig_set]
out = {"added_edges_by_name": dict(added.most_common()), "reach_before": None, "reach_after": len(reach),
       "cyclic_after": len(cyc), "new_cyclic": new, "multi_node_sccs": [x for x in cyc if len(x) > 1]}
json.dump(out, open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "rv84_augment_scc.out.json"), "w"), indent=1)
print("reach", len(reach), "cyclic", len(cyc), "new", len(new), "multi-node", len(out["multi_node_sccs"]))
for x in new: print(" NEW", len(x), x[:4])
