"""RV84: text allocation volume (TAV) missed where loop_bounds.json's broad zero rules
(`components`, `wind`, `intensity`) zero real D1 loops. Uses the packet's own site byte
classes and function multiplicities; loop sizes read from source at NUM 5ae5fe4f0f
(PP/lib.rs:12320-12349 local_components 4; :10931 node components 6; retained_product.rs
observables_view components 3; lib.rs:9755 three intensity accumulators).
Usage: python3 rv84_zero_quantify.py <I65 _run_records>"""
import json, sys, collections
I65 = sys.argv[1]
tb = json.load(open(I65 + "/text_budget.caps.out.json"))
fm = tb["function_multiplicity"]
def cap(b): return max(8, 2 * b) if b > 0 else 0
per_call = collections.Counter()
for r in tb["rows"]:
    if r["fn"] and r["reached"] and fm.get(r["fn"]) and r["mult"] > 0:
        per_call[r["fn"]] += r["req"] / fm[r["fn"]]
def site(file_end, line):
    return [r for r in tb["rows"] if r["file"].endswith(file_end) and r["line"] == line]
P = "core/product_physics/src/"
cases = [
 ("endpoint stress local_components (lib.rs:12350)", P + "lib.rs", [12355], 224, 4, P + "lib.rs:12505:append_endpoint_stress_result", 448),
 ("station stress local_components (lib.rs:12443)", P + "lib.rs", [12448], 256, 4, P + "lib.rs:12535:append_station_stress_result", 512),
 ("node displacement components (lib.rs:10987)", P + "lib.rs", [10989, 10990], 64, 6, None, 0),
 ("observables_view components (retained_product.rs:2003)", P + "retained_product.rs", [2004], 2, 3, None, 0),
 ("span intensity accumulators (lib.rs:9755, error path)", P + "lib.rs", [9756], 64, 3, None, 0),
]
out, tot = [], 0
for name, f, lines, M, bound, callee, counted in cases:
    miss_sites = sum(M * bound * cap(r["bytes"]) for l in lines for r in site(f, l))
    miss_callee = max(0, M * bound - counted) * per_call.get(callee, 0) if callee else 0
    tot += miss_sites + miss_callee
    out.append({"loop": name, "fn_multiplicity": M, "true_bound": bound, "missed_site_bytes": round(miss_sites),
                "callee": callee, "callee_counted_calls": counted, "missed_callee_bytes": round(miss_callee)})
print(json.dumps({"cases": out, "total_missed_TAV_bytes": round(tot)}, indent=1))
