#!/usr/bin/env python3
"""I61 U9 G8: compare the PR-head sweeps with u3_grant2_02's and check the pressure and coexistence controls.

Usage: compare_g8.py REG_TSV STALE_TSV BASE_REG_TSV BASE_STALE_TSV PRESSURE_REG_JSON PRESSURE_STALE_JSON OUT_JSON FIXTURE_DIR
Standard library only. Prints a summary; exit 1 on any difference or failed expectation.
"""
import hashlib, json, sys
reg, stale, breg, bstale, preg, pstale, out = sys.argv[1:8]
sha = lambda p: hashlib.sha256(open(p, "rb").read()).hexdigest()
def load(p):
    rows = {}
    for line in open(p).read().splitlines():
        rel, route, mode, rest = line.split("\t", 3)
        rows[(rel, route, mode)] = rest
    return rows
R, ST, BR, BS = load(reg), load(stale), load(breg), load(bstale)
rep = {"sweep": {}, "coexistence": [], "pressure_fixture_rows": [], "pressure": [], "failures": []}
fail = rep["failures"].append
for name, cur, base, cp, bp in (("registered", R, BR, reg, breg), ("stale", ST, BS, stale, bstale)):
    diff = sorted(k for k in set(cur) | set(base) if cur.get(k) != base.get(k))
    rep["sweep"][name] = {"rows": len(cur), "sha256": sha(cp), "base_sha256": sha(bp), "identical": sha(cp) == sha(bp),
                          "differing_rows": [[*k, base.get(k), cur.get(k)] for k in diff]}
    if diff or sha(cp) != sha(bp): fail(f"sweep {name}: {len(diff)} rows differ from u3_grant2_02")
# Direct classes and causes, registered
classes, causes = {}, {}
for k, v in R.items():
    if k[1] == "retained_direct" and not v.startswith("ERR:"):
        cols = v.split("\t"); classes[cols[1]] = classes.get(cols[1], 0) + 1; causes[cols[3]] = causes.get(cols[3], 0) + 1
rep["sweep"]["registered_direct_classes"] = classes
rep["sweep"]["registered_direct_causes"] = causes
# Coexistence: n05/n06 source-block fixtures through Direct
for k in sorted(R):
    if k[1] == "retained_direct" and ("/n05-" in "/" + k[0] or "/n06-" in "/" + k[0]):
        rc, sc = R[k].split("\t"), ST[k].split("\t")
        row = {"fixture": k[0], "mode": k[2], "registered": rc[1:], "stale": sc[1:],
               "direct_equals_value_registered": rc[0] == R[(k[0], "value_mode", k[2])].split("\t")[0],
               "direct_equals_value_stale": sc[0] == ST[(k[0], "value_mode", k[2])].split("\t")[0]}
        rep["coexistence"].append(row)
        ok = (rc[1] == "exact" and rc[2] == "Some((Registered, None))" and rc[3] == "Coexistence" and row["direct_equals_value_registered"]
              and sc[1] == "exact" and sc[2] == 'Some((Stale, Some("D1.1")))' and sc[3] == "none" and row["direct_equals_value_stale"])
        if not ok: fail(f"coexistence {k}: {rc[1:]} / {sc[1:]}")
if not rep["coexistence"]: fail("no n05/n06 rows found")
# Pressure-bearing fixtures in the sweep
for k in sorted(R):
    if k[1] == "retained_direct" and "pressure" in k[0]:
        rep["pressure_fixture_rows"].append({"fixture": k[0], "mode": k[2], "registered": R[k].split("\t")[1:], "stale": ST[k].split("\t")[1:]})
# The pressure harness
P = {(r["label"], r["mode"]): r for r in json.load(open(preg))}
Q = {(r["label"], r["mode"]): r for r in json.load(open(pstale))}
FIX = sys.argv[8]  # dir holding the PR head's committed milestone successor fixtures
PINNED = {m: json.load(open(f"{FIX}/retained_precision_milestone_successor_{m}.json")) for m in ("sparse_interactive", "dense_scrutiny")}
for (label, mode), r in sorted(P.items()):
    s = Q[(label, mode)]
    checks = {}
    checks["stale_exact_at_D1_1_build"] = s.get("class") == "exact" and s.get("clause") == "D1.1" and s.get("cause") == "none" and s.get("notices") == 0 and s.get("profile") == "Stale"
    checks["value_route_same_in_both_builds"] = r.get("value_sha") == s.get("value_sha")
    if label == "milestone":
        checks["registered_successor_equals_pinned_fixture"] = r.get("class") == "successor" and r.get("profile") == "Registered" and r.get("refusal") is None and json.load(open(f"{preg}.successor_{mode}.json")) == PINNED[mode]["source"] and PINNED[mode]["invocation"]["solver_mode"] == mode
    else:
        checks["registered_exact_no_notice"] = r.get("class") == "exact" and r.get("notices") == 0 and r.get("cause") == "none" and r.get("profile") == "Registered"
    if label == "milestone_with_pressure":
        checks["refused_at_D1_5_pressure_regions"] = "PressureRegions" in (r.get("refusal") or "") and "Case" in (r.get("refusal") or "")
    if label == "phys_r4_pressurized":
        checks["refused_at_D1_namespace"] = "Namespace" in (r.get("refusal") or "")
        checks["ordinary_refusal_numerical_integrity"] = r.get("value_status") == "MODEL_INCOMPLETE" and r.get("value_blocking") == ["NUMERICAL_INTEGRITY_UNRESOLVED"]
    if label == "phys_r4_without_pressure":
        checks["refused_at_D1_namespace"] = "Namespace" in (r.get("refusal") or "")
        checks["no_pressure_published"] = r.get("value_status") == "MECHANICS_SOLVED" and r.get("value_blocking") == []
    rep["pressure"].append({"label": label, "mode": mode, "registered": r, "stale": s, "checks": checks})
    for c, v in checks.items():
        if not v: fail(f"pressure {label} {mode}: {c}")
json.dump(rep, open(out, "w"), indent=1); open(out, "a").write("\n")
for name in ("registered", "stale"):
    s = rep["sweep"][name]; print(f"sweep {name}: rows {s['rows']} sha {s['sha256'][:16]} base {s['base_sha256'][:16]} identical {s['identical']} differing {len(s['differing_rows'])}")
print("registered Direct classes", classes, "causes", causes)
for c in rep["coexistence"]: print("coexistence", c["fixture"], c["mode"], c["registered"], c["stale"][:2])
for p in rep["pressure_fixture_rows"]: print("pressure fixture", p["fixture"], p["mode"], p["registered"][:2], p["stale"][:2])
for p in rep["pressure"]: print("pressure", p["label"], p["mode"], p["registered"].get("class"), p["registered"].get("clause"), p["registered"].get("refusal"), "|", p["stale"].get("class"), p["stale"].get("clause"), "|", p["registered"].get("value_status"), p["registered"].get("value_blocking"), "| all", all(p["checks"].values()))
print("FAILURES", rep["failures"])
sys.exit(1 if rep["failures"] else 0)
