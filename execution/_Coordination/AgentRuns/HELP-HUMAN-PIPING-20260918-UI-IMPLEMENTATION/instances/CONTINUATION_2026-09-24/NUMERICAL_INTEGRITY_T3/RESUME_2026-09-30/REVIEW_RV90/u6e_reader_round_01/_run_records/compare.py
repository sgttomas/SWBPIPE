import json, sys
S = sys.argv[1]
L = lambda n: json.load(open(f"{S}/work/{n}.json"))
exp = {}
for c in ("07f", "07g"):
    for line in open(f"{S}/work/applied_{c}.jsonl"):
        d = json.loads(line); k = d["kind"] + ":" + d["id"]
        exp.setdefault(c, {})[k] = (d["expected"], d.get("expected_by_reader"))
def norm(x):
    if isinstance(x, list): return tuple(x)
    return ("pass", tuple(x["classes"]), x["eligible"])
lang = {"py": "python", "rs": "rust", "ts": "typescript"}
report = {}
for r in ("py", "rs", "ts"):
    o = {f"{v}_{c}": L(f"out_{r}_{v}_{c}") for v in ("base", "cand") for c in ("07f", "07g")}
    rep = {}
    # 1) base on 07f vs cand on 07g (the author's delta)
    a, b = o["base_07f"], o["cand_07g"]
    rep["delta_07f_base__07g_cand_changed"] = [k for k in a if norm(a[k]) != norm(b[k])]
    rep["new_in_07g"] = {k: (a.get(k), b[k]) for k in b if k not in a}
    # 2) base reader vs cand reader on 07g
    a, b = o["base_07g"], o["cand_07g"]
    rep["07g_base_vs_cand_changed"] = {k: (a[k], b[k]) for k in a if norm(a[k]) != norm(b[k])}
    # 3) cand on unrepaired 07f
    a, b = o["base_07f"], o["cand_07f"]
    ch = {k: (a[k], b[k]) for k in a if norm(a[k]) != norm(b[k])}
    rep["07f_unrepaired_under_cand"] = {"changed": len(ch), "cases_refused": sum(1 for k in ch if k.startswith("cases:")), "must_pass_refused": sum(1 for k in ch if k.startswith("must_pass:")), "mutations_changed": sum(1 for k in ch if k.startswith("mutations:"))}
    # 4) cand on 07g vs corpus expectation
    miss = []
    for k, v in o["cand_07g"].items():
        e, ebr = exp["07g"][k]
        if ebr and lang[r] in ebr: e = ebr[lang[r]]
        if k.startswith("cases:") or k.startswith("must_pass:"):
            if not (isinstance(v, dict) and v.get("pass")): miss.append((k, v, "expected pass"))
        else:
            if not (isinstance(v, list) and e and v == [e["gate"], e["code"]]): miss.append((k, v, e))
    rep["07g_cand_vs_expected_misses"] = miss
    # also base reader on 07f vs 07f expectations
    miss = []
    for k, v in o["base_07f"].items():
        e, ebr = exp["07f"][k]
        if ebr and lang[r] in ebr: e = ebr[lang[r]]
        if k.startswith("cases:") or k.startswith("must_pass:"):
            if not (isinstance(v, dict) and v.get("pass")): miss.append((k, v))
        elif not (isinstance(v, list) and v == [e["gate"], e["code"]]): miss.append((k, v, e))
    rep["07f_base_vs_expected_misses"] = miss
    rep["eligible_any"] = any(isinstance(v, dict) and v.get("eligible") for oo in o.values() for v in oo.values())
    report[r] = rep
# cross-reader parity on 07g cand (ignoring the per-reader G7 entry)
P = {r: L(f"out_{r}_cand_07g") for r in ("py", "rs", "ts")}
par = [k for k in P["py"] if not (norm(P["py"][k]) == norm(P["rs"][k]) == norm(P["ts"][k]))]
report["parity_07g_cand_diffs"] = {k: [P[r][k] for r in ("py", "rs", "ts")] for k in par}
json.dump(report, open(f"{S}/work/compare_report.json", "w"), indent=1, default=str)
for r in ("py", "rs", "ts"):
    rep = report[r]
    print(r, "delta changed:", rep["delta_07f_base__07g_cand_changed"], "| new:", len(rep["new_in_07g"]), set(map(lambda t: (str(t[0]), str(t[1])), rep["new_in_07g"].values())))
    print("   07g base vs cand changed:", {k: v for k, v in rep["07g_base_vs_cand_changed"].items()})
    print("   unrepaired 07f under cand:", rep["07f_unrepaired_under_cand"])
    print("   cand 07g misses:", rep["07g_cand_vs_expected_misses"], "| base 07f misses:", rep["07f_base_vs_expected_misses"], "| eligible any:", rep["eligible_any"])
print("parity diffs:", report["parity_07g_cand_diffs"])
