"""RV87 (u4_g6_01): list every positive-multiplicity (site, expression) of the G6 TEXT run that the
identifier audit table does not cover, with the text_args rule class that priced it.
Usage: python3 rv87_probe_unaudited.py <G6 _run_records> <part2 text_p2> > out.json   (stdlib only)"""
import json, os, re, sys, collections
G, P = sys.argv[1], sys.argv[2]
ta = json.load(open(os.path.join(G, "text_args.g4.json")))
aud = ta["id_audit"]
run = json.load(open(os.path.join(G, "text_g6", "text_budget.caps.out.json")))
inv = json.load(open(os.path.join(G, "template_inventory_head.out.json")))["rows"]
lex = json.load(open(os.path.join(P, "lexicon_p2.json")))["rows"]
rules = [(re.compile(r["re"]), r["max"]) for r in ta["args"]]
def rule_of(e):
    a1 = " ".join(e.split())
    for rx, cls in rules:
        if rx.search(a1):
            return cls
    return None
pos = {(r["file"], r["line"]) for r in run["rows"] if r["mult"] > 0}
mult = collections.defaultdict(int)
for r in run["rows"]:
    mult[(r["file"], r["line"])] = max(mult[(r["file"], r["line"])], r["mult"])
def covered(site, expr):
    t = aud.get(site)
    if not t:
        return False
    k = re.sub(r"\s", "", expr)
    return any(k == kk.split("|")[0] or k.endswith(kk.split("|")[0]) for kk in t)
out = []
for r in inv:
    key = (r["file"], r["line"])
    if key not in pos:
        continue
    site = f"{r['file']}:{r['line']}"
    for ph in r.get("placeholders", []):
        if not covered(site, ph["arg"]):
            out.append({"site": site, "kind": r["kind"], "expr": ph["arg"], "spec": ph["spec"], "class": rule_of(ph["arg"]),
                        "site_size": site.split("core/")[-1] in ta.get("site_size", {}) or site in ta.get("site_size", {}),
                        "mult": mult[key]})
for r in lex:
    key = (r["file"], r["line"])
    if key not in pos or r.get("literal_bytes") is not None:
        continue
    site = f"{r['file']}:{r['line']}"
    e = r["arg"] if r["kind"] == "push_str" else r["receiver"]
    if not covered(site, e):
        out.append({"site": site, "kind": r["kind"], "expr": e, "spec": "", "class": rule_of(e), "mult": mult[key]})
print(json.dumps({"n": len(out), "by_class": collections.Counter(str(x["class"]) for x in out).most_common(), "rows": out}, indent=1))
