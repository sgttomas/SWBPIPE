"""I65 U4 G6: resolve each identifier-copy candidate's base variable to its binding (stdlib only).
For every (site, arg) candidate: the base identifier of the copied expression, and the
nearest preceding binding of that identifier in the enclosing function (fn parameter, for
pattern, let, closure parameter, if-let/while-let, match arm). The binding text is what the
audit classifies by (provenance), never the identifier's name."""
import json, re, sys, os
cands, proot, out = sys.argv[1:4]
C = json.load(open(cands))
files = {}
def lines(f):
    if f not in files:
        files[f] = open(os.path.join(proot, f), encoding="utf-8").read().split("\n")
    return files[f]
FN = re.compile(r"^\s*(pub(\([^)]*\))?\s+)?(const\s+)?(async\s+)?(unsafe\s+)?fn\s+\w+")
def fn_start(L, i):
    for j in range(i, -1, -1):
        if FN.match(L[j]):
            return j
    return 0
def base_of(arg):
    a = re.sub(r"\s", "", arg)
    a = re.sub(r"^(stable_suffix|identity|text|id_location|Some|&)+\(?&?", "", a)
    a = a.lstrip("&*(")
    m = re.match(r"([A-Za-z_][A-Za-z0-9_]*)", a)
    return m.group(1) if m else None
res = []
for c in C:
    f, line = c["site"].rsplit(":", 1)
    line = int(line)
    L = lines(f)
    s = fn_start(L, line - 1)
    b = base_of(c["arg"])
    bind = None
    if b and b not in ("self", "Self"):
        pats = [re.compile(r"\bfor\s*\(?[^;{]*\b%s\b[^;{]*\bin\b" % re.escape(b)),
                re.compile(r"\blet\s+(mut\s+)?\(?[^=;]*\b%s\b[^=;]*=" % re.escape(b)),
                re.compile(r"\|[^|]*\b%s\b[^|]*\|" % re.escape(b)),
                re.compile(r"\b%s\s*:\s*[&A-Za-z(\[]" % re.escape(b)),
                re.compile(r"\b(if|while)\s+let\b[^=]*\b%s\b[^=]*=" % re.escape(b)),
                re.compile(r"=>\s*|\b%s\b\s*\)?\s*=>" % re.escape(b))]
        for j in range(line - 1, s - 1, -1):
            t = L[j]
            if any(p.search(t) for p in pats[:5]):
                bind = f"{j+1}: " + " ".join(t.split())[:220]
                break
        if bind is None:
            # the fn signature may span lines
            sig = " ".join(" ".join(L[s:min(s + 12, line)]).split())
            m = re.search(r"\b%s\s*:\s*([^,)]+)" % re.escape(b), sig)
            if m:
                bind = f"{s+1}(sig): {b}: {m.group(1)[:120]}"
    res.append(dict(c, base=b, fn_line=s + 1, fn=" ".join(L[s].split())[:120], binding=bind))
json.dump(res, open(out, "w"), indent=0)
import collections
print(len(res), sum(1 for r in res if r["binding"]), collections.Counter(r["base"] for r in res).most_common(40))
