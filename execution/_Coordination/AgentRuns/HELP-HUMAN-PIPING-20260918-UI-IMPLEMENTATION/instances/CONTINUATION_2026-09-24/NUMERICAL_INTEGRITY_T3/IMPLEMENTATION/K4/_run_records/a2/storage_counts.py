"""Scratch (records): pattern and profile entries of named K4 cases, from the
generator's emulation of the structure and RCM order (read-only import)."""
import sys, importlib.util
sys.dont_write_bytecode = True
spec = importlib.util.spec_from_file_location("gen", sys.argv[1])
g = importlib.util.module_from_spec(spec); spec.loader.exec_module(g)
by = {m["name"]: m for m in g.models() + g.routed_models()}
for name in sys.argv[2:]:
    m = by[name]
    K, _ = g.assemble_em(m, 128)
    n = 6 * len(m["nodes"])
    diag = sum(1 for (r, c) in K if r == c)
    pattern = 2 * len(K) - diag
    pre = {6 * c["node"] + c["c"] for c in m["constraints"]}
    free = [x for x in range(n) if x not in pre]
    pos = {x: a for a, x in enumerate(free)}
    rows = {x: set() for x in range(n)}
    for (r, c) in K:
        rows[r].add(c); rows[c].add(r)
    adj = [[pos[c] for c in sorted(rows[x]) if c in pos and c != x] for x in free]
    order = g.rcm_em(adj)
    rank = [0] * len(free)
    for k, a in enumerate(order):
        rank[a] = k
    first = [min([rank[b] for b in adj[a]] + [i]) for i, a in enumerate(order)]
    profile = sum(i - f + 1 for i, f in enumerate(first))
    print("%s dofs %d free %d pattern %d profile %d" % (name, n, len(free), pattern, profile))
