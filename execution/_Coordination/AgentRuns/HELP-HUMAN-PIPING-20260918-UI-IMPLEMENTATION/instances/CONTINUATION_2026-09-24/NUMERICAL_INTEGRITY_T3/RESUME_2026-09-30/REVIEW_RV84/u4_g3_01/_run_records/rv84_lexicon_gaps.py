import json, re, os, sys, collections
I65, SRC = sys.argv[1], sys.argv[2]
tb = json.load(open(os.path.join(I65, "text_budget.caps.out.json")))
cg = json.load(open(os.path.join(I65, "callgraph_edges.json")))
edges = cg["edges"]
roots = [k for k in edges if k.rsplit(":", 1)[1] in ("run_linear_static_preview_value_with_retained_direct", "prepare_observed")]
reach, todo = set(roots), list(roots)
while todo:
    v = todo.pop()
    for w in edges.get(v, []):
        if w not in reach: reach.add(w); todo.append(w)
fm = tb["function_multiplicity"]
spans = collections.defaultdict(list)
for k in edges:
    f, line, name = k.rsplit(":", 2); spans[f].append((int(line), k))
def strip(t):
    m = re.search(r"#\[cfg\(test\)\]\s*\n\s*(pub(\(crate\))?\s+)?mod\s+\w+\s*\{", t)
    if m: t = t[:m.start()] + " " * (len(t) - m.start())
    t = re.sub(r"//[^\n]*", lambda m: " " * len(m.group(0)), t)
    return t
PAT = re.compile(r"String::from\(|\.repeat\(|to_uppercase\(|to_lowercase\(|collect::<String>|collect::<Vec<String>>|serde_json::to_string(_pretty)?\(|String::with_capacity|\.into_owned\(|to_string_lossy|\.concat\(\)|\.to_ascii_lowercase\(|\.to_ascii_uppercase\(|String::new\(\)")
listed = {(r["file"], r["line"]) for r in tb["rows"]}
hits = []
for f in sorted({k.rsplit(":", 2)[0] for k in reach}):
    t = strip(open(os.path.join(SRC, f), encoding="utf-8").read())
    lines = t.split("\n")
    fl = sorted(spans[f])
    for i, L in enumerate(lines, 1):
        for m in PAT.finditer(L):
            # enclosing fn: last fn starting at or before line i (approx)
            enc = None
            for l0, k in fl:
                if l0 <= i: enc = k
            if enc in reach:
                hits.append({"file": f, "line": i, "tok": m.group(0), "fn": enc, "M": fm.get(enc), "listed": (f, i) in listed, "src": L.strip()[:140]})
json.dump(hits, open(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'rv84_lexicon_gaps.out.json'), "w"), indent=1)
c = collections.Counter(h["tok"] for h in hits)
print(len(hits), c)
for h in hits:
    if h["tok"] not in ("String::new()",) and not h["listed"]:
        print(h["M"], h["file"].split("/")[-1] + ":" + str(h["line"]), h["tok"], "|", h["src"][:110])
