import json, sys
S = sys.argv[1]
exp = {}
for line in open(f"{S}/work/applied_probes_all.jsonl"):
    d = json.loads(line); exp[d["kind"] + ":" + d["id"]] = d["expected"]
o = {f"{l}_{v}": json.load(open(f"{S}/work/out_{l}_{v}_probes_all.json")) for v in ("base", "cand") for l in ("py", "rs", "ts")}
f = lambda x: "pass " + str(x["classes"]) if isinstance(x, dict) else "/".join(x).replace("RETAINED_PRECISION_", "RP_").replace("SOURCE_PREVIEW_PHYSICS_", "SPP_")
print("| probe | expected (pristine) | py base | rs base | ts base | py cand | rs cand | ts cand |")
print("|---|---|---|---|---|---|---|---|")
bad = []
for k in exp:
    row = [f(o[n][k]) for n in ("py_base", "rs_base", "ts_base", "py_cand", "rs_cand", "ts_cand")]
    e = exp[k]; es = "pass" if e == "pass" else ("/".join(e).replace("RETAINED_PRECISION_", "RP_") if e else "—")
    print(f"| {k} | {es} | " + " | ".join(row) + " |")
    if e and e != "pass":
        if any(o[n][k] != e for n in ("py_cand", "rs_cand", "ts_cand")): bad.append(k)
    if e == "pass" and any(not isinstance(o[n][k], dict) for n in ("py_cand", "rs_cand", "ts_cand")): bad.append(k)
cand_div = [k for k in exp if len({json.dumps(o[n][k] if isinstance(o[n][k], list) else o[n][k]["classes"]) for n in ("py_cand", "rs_cand", "ts_cand")}) > 1]
base_div = [k for k in exp if len({json.dumps(o[n][k] if isinstance(o[n][k], list) else o[n][k]["classes"]) for n in ("py_base", "rs_base", "ts_base")}) > 1]
print("\nexpectation misses (candidate):", bad)
print("candidate cross-reader divergences:", cand_div)
print("base cross-reader divergences:", base_div)
