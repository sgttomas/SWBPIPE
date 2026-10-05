"""RV79 confirmation 03: compare every RV79 probe outcome on a894d9d0ba with the outcome the rulings now require.
Rulings that changed an earlier probe expectation: D19 (probes_03), D20 (probes_04), D21 (v-build), D25 (integral float),
D29 (empty inventory, PR2)."""
import json, sys
d = sys.argv[1]
G5A, G5P, G3C = "G5 RETAINED_PRECISION_ATTEMPT_MISMATCH", "G5 RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH", "G3 RETAINED_PRECISION_COVERAGE_MISMATCH"
override = {
    "D5b_vbuild_on_escalating_failed_verification": (G5A, "D21"),
    "D10_integral_float_counter": ("PASS", "D25"),
    "PR2_empty_coverage_and_empty_inventory": (G3C, "D29"),
    "N7_selected_source_ref_foreign": ("not PASS", "N7/D15"),
    "selected_case_without_c3_attempt": (G5P, "D20"),
}
rows = []
for name in ("probes_02_on_a894.jsonl", "probes_03_on_a894.jsonl", "probes_04_on_a894.jsonl", "probes_r03.jsonl"):
    for line in open(f"{d}/{name}"):
        p = json.loads(line)
        ruled, dec = override.get(p["id"], (p.get("ruled") or p.get("rv79_reading"), p.get("decision") or p.get("basis") or "D19"))
        if name.startswith("probes_03"): ruled, dec = G5P, "D19"
        got = p["python"]
        ok = (got != "PASS") if ruled == "not PASS" else (got == ruled)
        rows.append({"file": name, "id": p["id"], "python": got, "ruled": ruled, "decision": dec, "agrees": ok, "raise_lines": p.get("raise_lines")})
for r in rows: print(json.dumps(r))
print(json.dumps({"total": len(rows), "agree": sum(r["agrees"] for r in rows), "disagree": [r["id"] for r in rows if not r["agrees"]]}))
