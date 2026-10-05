"""I65 U4 G4 (R-1, STACK): type-containment graph of every struct/enum in the scanned crates
(stdlib only, read-only). An edge T -> U when U's name appears in T's definition body (fields
or variant payloads, through any wrapper: Box, Vec, Option, Arc, references, generics). A cycle
would be a recursive type, the only way a chain of implicit trait calls (Display/Debug, From,
Deserialize, Ord/Eq, Clone, Drop) on contained values could recurse to a bounded depth.
Name matching over-approximates containment (a same-named type in another crate also counts);
enum variant names are dropped (a variant named like a type is not a reference to it).
Usage: python3 recursive_types.py <P root> <crate src dir> ...
"""
import os, re, sys, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import loopscan
root = sys.argv[1]
defs = {}
for d in sys.argv[2:]:
    for r, _, fs in os.walk(os.path.join(root, d)):
        if "/tests" in r or "/benches" in r:
            continue
        for f in fs:
            if not f.endswith(".rs") or "test" in f:
                continue
            t = open(os.path.join(r, f), encoding="utf-8").read()
            m = re.search(r"#\[cfg\(test\)\]\s*\n\s*(pub(\(crate\))?\s+)?mod\s+\w+\s*\{", t)
            b = loopscan.blank_rust(t[:m.start()] if m else t)
            for dm in re.finditer(r"\b(struct|enum)\s+([A-Za-z_]\w*)", b):
                j = dm.end(); dd = 0
                while j < len(b) and b[j] not in "{;(":
                    j += 1
                if j >= len(b):
                    continue
                if b[j] == ";":
                    body = ""
                else:
                    op = b[j]; cl = {"{": "}", "(": ")"}[op]; k = j; dd = 0
                    while k < len(b):
                        if b[k] == op: dd += 1
                        elif b[k] == cl:
                            dd -= 1
                            if dd == 0: break
                        k += 1
                    body = b[j:k + 1]
                if dm.group(1) == "enum" and body.startswith("{"):
                    # drop variant names: an identifier at depth 1 that starts a variant
                    keep, dd2, prev = [], 0, "{"
                    for tk in re.finditer(r"[{}()\[\]<>,]|#\[[^\]]*\]|[A-Za-z_]\w*|\S", body):
                        x = tk.group(0)
                        if x in "{([<": dd2 += 1
                        elif x in "})]>": dd2 -= 1
                        if dd2 == 1 and prev in ("{", ",") and re.match(r"[A-Z]", x):
                            prev = "variant"; continue
                        if not x.startswith("#["):
                            prev = x
                        keep.append(x)
                    body = " ".join(keep)
                defs.setdefault(dm.group(2), []).append((os.path.relpath(os.path.join(r, f), root), body))
names = set(defs)
edges = {n: set() for n in names}
for n, lst in defs.items():
    for _, body in lst:
        for tok in set(re.findall(r"\b([A-Z][A-Za-z0-9_]*)\b", body)):
            if tok in names:
                edges[n].add(tok)
# cycles (Tarjan)
idx, low, st, on, out, c = {}, {}, [], set(), [], [0]
sys.setrecursionlimit(100000)
def sc(v):
    idx[v] = low[v] = c[0]; c[0] += 1; st.append(v); on.add(v)
    for w in edges[v]:
        if w not in idx:
            sc(w); low[v] = min(low[v], low[w])
        elif w in on:
            low[v] = min(low[v], idx[w])
    if low[v] == idx[v]:
        comp = []
        while True:
            w = st.pop(); on.discard(w); comp.append(w)
            if w == v: break
        if len(comp) > 1 or v in edges[v]:
            out.append(sorted(comp))
for v in sorted(names):
    if v not in idx:
        sc(v)
print(json.dumps({"types": len(names), "recursive_components": out,
                  "where": {n: [f for f, _ in defs[n]] for comp in out for n in comp}}, indent=1))
