import json, re, os, sys, random, collections
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'rv84_augment_scc.py')).read().split("METHOD = re.compile")[0])
I65 = sys.argv[1]
tb = json.load(open(os.path.join(I65, "text_budget.caps.out.json")))
oe = cg["edges"]
roots = [k for k in nodes if k.rsplit(":", 1)[1] in ("run_linear_static_preview_value_with_retained_direct", "prepare_observed")]
reach, todo = set(roots), list(roots)
while todo:
    v = todo.pop()
    for w in oe.get(v, []):
        if w not in reach: reach.add(w); todo.append(w)
rows_by_fn = collections.defaultdict(list)
for r in tb["rows"]:
    if r["fn"]: rows_by_fn[r["fn"]].append(r)
def raw_body(key):
    rel, line, name = key.rsplit(":", 2)
    t = open(os.path.join(SRC, rel), encoding="utf-8").read()
    t2 = re.sub(r"//[^\n]*", lambda m: " " * len(m.group(0)), t)
    pos = 0
    for _ in range(int(line) - 1): pos = t2.index("\n", pos) + 1
    m = re.compile(r"\bfn\s+" + re.escape(name) + r"\b").search(t2, pos)
    b = t2.find("{", m.end()); d = 0; i = b
    # brace-match on a copy with strings blanked
    t3 = re.sub(r'"(?:\\.|[^"\\])*"', lambda m: '"' + " " * (len(m.group(0)) - 2) + '"', t2)
    while i < len(t3):
        if t3[i] == "{": d += 1
        elif t3[i] == "}":
            d -= 1
            if d == 0: break
        i += 1
    return t2[b:i], t2.count("\n", 0, b) + 1
PAT = {"format": re.compile(r"\bformat!\s*\("), "write": re.compile(r"\bwrite(ln)?!\s*\("), "to_string": re.compile(r"\.to_string\(\)"),
       "diag": re.compile(r"(?<![A-Za-z0-9_])diag\s*\(")}
random.seed(84)
cands = sorted(k for k in reach if "/tests" not in k and k in rows_by_fn)
sample = random.sample(cands, 30)
out = []
for k in sample:
    bd, l0 = raw_body(k)
    mine = {kk: len(p.findall(bd)) for kk, p in PAT.items()}
    theirs = collections.Counter(r["kind"] for r in rows_by_fn[k])
    out.append({"fn": k, "mine": mine, "theirs": {kk: theirs.get(kk, 0) for kk in PAT},
                "M": tb["function_multiplicity"].get(k), "match": all(mine[kk] <= theirs.get(kk, 0) for kk in PAT)})
json.dump(out, open(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'rv84_sample_sites.out.json'), "w"), indent=1)
for o in out: print(o["match"], o["M"], o["fn"].rsplit("/", 1)[1], o["mine"], o["theirs"])
