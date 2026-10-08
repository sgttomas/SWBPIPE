import json, sys
recs = {}
for l in open(sys.argv[1]):
    r = json.loads(l); m = recs.setdefault(r["id"], {"description": r["description"], "lanes": {}}); m["lanes"].update(r["lanes"])
def cell(lane, v):
    if lane == "probe": return f'{v["differences_from_head"]} probes differ' if v["differences_from_head"] else "0 differ"
    if v.get("error"): return "ERROR"
    return f'killed ({v["failed"]})' if v.get("failed") else ("passes" if mid == "N0" else "survives")
print("| Id | Mutant | TS | Python | Rust | TS probes (232) | Outcome |")
print("|---|---|---|---|---|---|---|")
for mid, m in recs.items():
    L = m["lanes"]
    c = {k: cell(k, L[k]) if k in L else "—" for k in ("ts", "py", "rs", "probe")}
    suite_killed = any(L[k].get("failed") for k in ("ts", "py", "rs") if k in L)
    if mid == "N0": out = "all lanes pass"
    elif mid in ("BT", "BP", "BC"): out = "head tests fail at base (expected)"
    elif suite_killed: out = "killed"
    elif L.get("probe", {}).get("differences_from_head"): out = "**survives the suite**; killed by RV108's probes"
    else: out = "survives"
    print(f'| {mid} | {m["description"]} | {c["ts"]} | {c["py"]} | {c["rs"]} | {c["probe"]} | {out} |')
