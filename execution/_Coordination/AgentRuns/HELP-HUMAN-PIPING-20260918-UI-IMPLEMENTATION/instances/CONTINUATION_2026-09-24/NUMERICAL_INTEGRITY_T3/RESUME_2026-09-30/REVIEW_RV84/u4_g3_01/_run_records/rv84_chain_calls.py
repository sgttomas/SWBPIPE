"""RV84: find method calls in chain position (after ')' or '?') inside reachable fn bodies
whose name matches a crate-defined fn, and report whether the caller->name edge exists."""
import json, re, sys, os, collections
I65, SRC = sys.argv[1], sys.argv[2]
cg = json.load(open(os.path.join(I65, "callgraph_edges.json")))
tb = json.load(open(os.path.join(I65, "text_budget.caps.out.json")))
edges, spans = cg["edges"], cg["spans"]
reach = set(tb["function_multiplicity"].keys())
names = collections.defaultdict(list)
for k in spans:
    names[k.rsplit(":", 1)[1]].append(k)
def strip(t):
    t = re.sub(r"//[^\n]*", lambda m: " " * len(m.group(0)), t)
    t = re.sub(r"/\*.*?\*/", lambda m: " " * len(m.group(0)), t, flags=re.S)
    t = re.sub(r'"(?:\\.|[^"\\])*"', lambda m: '"' + " " * (len(m.group(0)) - 2) + '"', t)
    return t
cache = {}
def body(key):
    rel, line, name = key.rsplit(":", 2)
    if rel not in cache:
        cache[rel] = strip(open(os.path.join(SRC, rel), encoding="utf-8").read())
    t = cache[rel]
    # locate line
    pos = 0
    for _ in range(int(line) - 1):
        pos = t.index("\n", pos) + 1
    m = re.compile(r"\bfn\s+" + re.escape(name) + r"\b").search(t, pos)
    b = t.find("{", m.end()); d = 0; i = b
    while i < len(t):
        if t[i] == "{": d += 1
        elif t[i] == "}":
            d -= 1
            if d == 0: break
        i += 1
    return t[b:i]
CHAIN = re.compile(r"[)?]\s*\.\s*([A-Za-z_][A-Za-z0-9_]*)\s*(?:::<[^>]*>)?\s*\(")
missing = collections.defaultdict(set)
for key in sorted(reach):
    try:
        bd = body(key)
    except Exception as e:
        continue
    have = {t.rsplit(":", 1)[1] for t in edges.get(key, [])}
    for m in CHAIN.finditer(bd):
        nm = m.group(1)
        if nm in names and nm not in have:
            missing[nm].add(key)
out = {}
for nm, callers in sorted(missing.items()):
    tg = names[nm]
    out[nm] = {"targets": tg, "targets_unreached": [t for t in tg if t not in reach], "callers": sorted(callers)[:6], "n_callers": len(callers)}
json.dump(out, open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "rv84_chain_calls.out.json"), "w"), indent=1)
print(len(out), "names with chain-call edges absent")
for nm, v in out.items():
    print(nm, "targets", len(v["targets"]), "unreached", len(v["targets_unreached"]), "callers", v["n_callers"])
