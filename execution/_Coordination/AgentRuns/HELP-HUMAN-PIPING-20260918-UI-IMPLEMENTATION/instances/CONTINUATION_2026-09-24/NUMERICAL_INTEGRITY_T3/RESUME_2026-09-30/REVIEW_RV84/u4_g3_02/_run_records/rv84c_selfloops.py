"""RV84 confirmation (N-3): the 22 self-recursive components of I65's G4 graph. For each: reachable
from the Direct root?, the self-call sites in its body (source at b1f80234dc), and the recursion
argument read at that site (what shrinks: a Value child, a schema node, a guarded flag).
Usage: python3 rv84c_selfloops.py <G4 _run_records> <P root at b1f80234dc>"""
import json, re, os, sys
G4, SRC = sys.argv[1], sys.argv[2]
cg = json.load(open(os.path.join(G4, "callgraph_edges.json")))
co = json.load(open(os.path.join(G4, "callgraph.out.json")))
edges = cg["edges"]
root = [k for k in edges if k.endswith(":run_linear_static_preview_value_with_retained_direct")]
reach, todo = set(root), list(root)
while todo:
    v = todo.pop()
    for w in edges[v]:
        if w not in reach: reach.add(w); todo.append(w)
out = []
for comp in co["cyclic_components"]:
    k = comp[0]
    rel, line, name = k.rsplit(":", 2)
    t = open(os.path.join(SRC, rel), encoding="utf-8").read()
    lines = t.split("\n")
    # body by brace matching from the fn line
    start = sum(len(x) + 1 for x in lines[:int(line) - 1])
    b = t.find("{", t.find("fn " + name, start)); d = 0; i = b
    while i < len(t):
        if t[i] == "{": d += 1
        elif t[i] == "}":
            d -= 1
            if d == 0: break
        i += 1
    body = t[b:i]
    sites = [(t.count("\n", 0, b + m.start()) + 1, body[max(0, m.start() - 60):m.end() + 40].replace("\n", " "))
             for m in re.finditer(r"(?<![A-Za-z0-9_])" + re.escape(name) + r"\s*(?:::<[^>]*>)?\s*\(", body)]
    out.append({"fn": k.split("/core/")[-1] if "/core/" in k else k, "reachable": k in reach, "self_call_sites": sites})
json.dump(out, open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "rv84c_selfloops.out.json"), "w"), indent=1)
print("components", len(out), "reachable", sum(o["reachable"] for o in out))
for o in out:
    print(("R " if o["reachable"] else "- ") + o["fn"], "| self-calls:", len(o["self_call_sites"]))
    for ln, s in o["self_call_sites"][:2]: print("      :%d %s" % (ln, " ".join(s.split())[:150]))
