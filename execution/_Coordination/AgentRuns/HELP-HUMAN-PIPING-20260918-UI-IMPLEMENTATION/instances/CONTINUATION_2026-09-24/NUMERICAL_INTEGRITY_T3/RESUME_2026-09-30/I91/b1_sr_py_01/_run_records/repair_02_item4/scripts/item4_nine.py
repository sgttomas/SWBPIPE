"""I91 repair 02, item 4: the census at the new head against R01, entry by entry, and the nine against RS.

Usage: item4_nine.py <run census_rv113.jsonl> <R01 census (RV113's py_head.jsonl)> <RS census at 6e3e4fe219> <out.json>
Stops (exit 1) unless the only changes are the transport verdicts of 07m mutations 277 and 286-293, each
G7 -> G2 with the same code and detail, and each new transport verdict's gate and code equal RS's.
"""
import json, sys
NINE = {277, 286, 287, 288, 289, 290, 291, 292, 293}
L = lambda p: {(r["set"], r["i"]): r for r in map(json.loads, open(p)) if r}
new, old, rs = L(sys.argv[1]), L(sys.argv[2]), L(sys.argv[3])
assert new.keys() == old.keys() == rs.keys()
changes = [(k, v) for k in sorted(new) for v in ("input_sha256", "bound", "unbound", "transport") if new[k][v] != old[k][v]]
other = [c for c in changes if not (c[0][0] == "mutation" and c[0][1] in NINE and c[1] == "transport")]
gc = lambda v: ["admitted", v["ok"]["numerical_eligible"]] if "ok" in v else [v["err"]["gate"], v["err"]["code"]]
rows, ok = [], not other and len(changes) == 9
for k in sorted(k for k, v in changes if v == "transport"):
    o, n, r = old[k]["transport"]["err"], new[k]["transport"]["err"], rs[k]["transport"]
    row = {"entry": k[1], "id": new[k]["id"], "old": o, "new": n, "rs_6e3e4fe219": gc(r),
           "g7_to_g2_same_code_and_detail": o["gate"] == "G7" and n["gate"] == "G2" and o["code"] == n["code"] and o["detail"] == n["detail"],
           "gate_and_code_equal_rs": [n["gate"], n["code"]] == gc(r)}
    ok = ok and row["g7_to_g2_same_code_and_detail"] and row["gate_and_code_equal_rs"]
    rows.append(row)
summary = {"entries": len(new), "changes": len(changes), "other_changes": other, "nine": rows,
           "transport_equal_rs_gate_code": {"r01": sum(gc(old[k]["transport"]) == gc(rs[k]["transport"]) for k in new),
                                            "head": sum(gc(new[k]["transport"]) == gc(rs[k]["transport"]) for k in new)},
           "ok": ok}
json.dump(summary, open(sys.argv[4], "w"), indent=1)
print(f"{len(new)} entries; {len(changes)} changes against R01; other changes (a stop): {len(other)}")
for r in rows:
    print(f"  {r['entry']} {r['id']}: transport {r['old']['gate']} {r['old']['code']} -> {r['new']['gate']} {r['new']['code']} (detail unchanged: {r['old']['detail'] == r['new']['detail']}); RS at 6e3e4fe219: {r['rs_6e3e4fe219'][0]} {r['rs_6e3e4fe219'][1]}; equal: {r['gate_and_code_equal_rs']}")
t = summary["transport_equal_rs_gate_code"]
print(f"transport gate and code equal to RS at 6e3e4fe219: R01 {t['r01']} of {len(new)}, head {t['head']} of {len(new)}")
print("OK" if ok else "STOP")
sys.exit(0 if ok else 1)
