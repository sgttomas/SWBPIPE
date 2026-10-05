"""RV91: relative-import graph of desktop src (non-test), SCCs in base vs candidate."""
import os, re, sys, json
def graph(src):
    g = {}
    for dp, _, fs in os.walk(src):
        for f in fs:
            if not f.endswith((".ts", ".tsx")) or ".test." in f: continue
            p = os.path.join(dp, f); rel = os.path.relpath(p, src)
            text = open(p).read()
            deps = set()
            for m in re.finditer(r'(?:import|export)\s+(?:type\s+)?[^;]*?from\s+["\'](\.[^"\']+)["\']', text):
                if re.match(r'(?:import|export)\s+type\s', m.group(0)): continue
                t = os.path.normpath(os.path.join(dp, m.group(1)))
                for ext in (".ts", ".tsx", "/index.ts", "/index.tsx"):
                    if os.path.exists(t + ext): deps.add(os.path.relpath(t + ext, src)); break
            g[rel] = deps
    return g
def sccs(g):
    idx, low, st, on, out, n = {}, {}, [], set(), [], [0]
    sys.setrecursionlimit(100000)
    def visit(v):
        idx[v] = low[v] = n[0]; n[0] += 1; st.append(v); on.add(v)
        for w in g.get(v, ()):
            if w not in idx: visit(w); low[v] = min(low[v], low[w])
            elif w in on: low[v] = min(low[v], idx[w])
        if low[v] == idx[v]:
            comp = []
            while True:
                w = st.pop(); on.discard(w); comp.append(w)
                if w == v: break
            if len(comp) > 1: out.append(sorted(comp))
    for v in g:
        if v not in idx: visit(v)
    return out
res = {}
for lane in ("base", "cand"):
    g = graph(sys.argv[1].replace("LANE", lane))
    res[lane] = sccs(g)
print(json.dumps(res, indent=1))
