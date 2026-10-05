"""For each candidate whose base is a fn parameter, list the arguments passed at every call
site of that fn in the snapshot (positional), so the parameter's source can be read."""
import json, re, sys, os, glob, collections
R = json.load(open(sys.argv[1])); proot = sys.argv[2]
src = {}
for f in glob.glob(proot + "/core/**/*.rs", recursive=True):
    if "/target/" in f or "/tests/" in f: continue
    src[f[len(proot)+1:]] = open(f, encoding="utf-8").read()
def params(sig):
    m = re.search(r"fn\s+(\w+)\s*(<[^>]*>)?\s*\((.*)", sig, re.S)
    if not m: return None, []
    name, body, d, cur, out = m.group(1), m.group(3), 0, "", []
    for ch in body:
        if ch in "(<[{": d += 1
        if ch in ")>]}":
            if d == 0: out.append(cur); break
            d -= 1
        if ch == "," and d == 0: out.append(cur); cur = ""
        else: cur += ch
    names = [re.match(r"\s*(mut\s+)?(\w+)", p).group(2) if re.match(r"\s*(mut\s+)?(\w+)", p) else "" for p in out if p.strip()]
    return name, [n for n in names if n not in ("self",) and not n.startswith("&")]
def call_args(text, fname):
    res = []
    for m in re.finditer(r"(?<![\w:])(?:\w+::)*%s\s*\(" % re.escape(fname), text):
        pre = text[max(0, m.start()-4):m.start()]
        if re.search(r"fn\s*$", text[max(0,m.start()-3):m.start()]): continue
        j, d, cur, out = m.end(), 0, "", []
        while j < len(text):
            ch = text[j]
            if ch in "([{": d += 1
            elif ch in ")]}":
                if d == 0: out.append(cur); break
                d -= 1
            if ch == "," and d == 0: out.append(cur); cur = ""
            else: cur += ch
            j += 1
        line = text.count("\n", 0, m.start()) + 1
        res.append((line, [" ".join(a.split()) for a in out]))
    return res
seen = {}
for r in R:
    b = r["binding"] or ""
    if "(sig)" not in b and not re.search(r"\bfn\s+\w+", b): continue
    f = r["site"].rsplit(":", 1)[0]
    L = src[f].split("\n")
    s = r["fn_line"] - 1
    sig = " ".join(L[s:s+10])
    fname, ps = params(sig)
    if not fname or r["base"] not in ps: continue
    idx = ps.index(r["base"])
    key = (f, fname, r["base"])
    if key in seen: continue
    calls = []
    for g, text in src.items():
        for line, args in call_args(text, fname):
            if g == f and line == r["fn_line"]: continue
            a = args[idx] if idx < len(args) else "?"
            # method calls: self is not in ps; positional matches
            calls.append(f"{g.split('core/')[-1]}:{line}: {a[:110]}")
    seen[key] = calls
for (f, fname, base), calls in seen.items():
    print(f"## {f.split('core/')[-1]} fn {fname} param {base}  ({len(calls)} calls)")
    for c in calls[:14]: print("   ", c)
