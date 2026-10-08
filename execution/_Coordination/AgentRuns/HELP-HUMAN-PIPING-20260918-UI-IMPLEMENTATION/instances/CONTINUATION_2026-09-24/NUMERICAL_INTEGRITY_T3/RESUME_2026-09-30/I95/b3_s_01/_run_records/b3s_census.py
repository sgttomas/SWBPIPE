"""I95 B3-S: the exact route's census from the committed physics-source fixtures (stdlib only, read-only).

1. Row census: every committed exact-route output with explicitly empty pressure regions publishes
   7n + 51m + 8g rows per case (checked per fixture and mode), against the chain's P_final bound
   7n + 51m + 8g + 3.
2. The per-case contract evidence at D1's caps: contract_evidence {connector, exact_cases, pressure}
   replaces the preview tree on this route (PP lib.rs:2841, :4673). Its Value facts (array slots,
   objects, entries, string bytes, key bytes, numbers, depth) are measured on one case of the fixture,
   split into per-member arrays (pipe_materials, pipe_sections, pipe_stress_extrema), per-DOF arrays
   (the three 6n RHS vectors), the node list (n) and the rest (per case), then evaluated at n = m = 32.
   Every string is bounded by max(its length, STR_CAP): identifiers are <= 128 B (D1 text cap) and the
   longest composed string (the elastic-maximum result id) is <= 23 + 2*(3 + 1 + 128) < 300.
3. The componentwise maximum of these facts and the preview tree's facts at the same caps (the
   chain's ordinary_caps PREVIEW / g4_caps PREVIEW_T / ENV / t25_g4 env_vf), which b3s_exact_chain.py
   substitutes for the preview tree.
Usage: python3 b3s_census.py <snapshot projects/chirality-piping dir> <out json>
"""
import json, os, sys

P, out = sys.argv[1:3]
FX = os.path.join(P, "fixtures/product_preview/physics_source")
STR_CAP = 300
n, m, g = 32, 32, 32                      # D1's model caps


def facts(v, cap=True):
    f = dict(arr=0, obj=0, ent=0, strb=0, keyb=0, nums=0, depth=0)
    def walk(x, d):
        f["depth"] = max(f["depth"], d)
        if isinstance(x, dict):
            f["obj"] += 1; f["ent"] += len(x)
            for k, y in x.items():
                f["keyb"] += len(k.encode()); walk(y, d + 1)
        elif isinstance(x, list):
            f["arr"] += len(x)
            for y in x:
                walk(y, d + 1)
        elif isinstance(x, str):
            b = len(x.encode()); f["strb"] += max(b, STR_CAP) if cap else b
        elif isinstance(x, (int, float)) and not isinstance(x, bool):
            f["nums"] += 1
    walk(v, 1)
    return f


def add(a, b, k=1):
    return {x: (max(a[x], b[x]) if x == "depth" else a[x] + k * b[x]) for x in a}


# ---------------------------------------------------------------- 1. rows
rows = []
for name in ("n05", "n05_units", "n05_unicode", "n06", "fields", "mixed", "mixed_units"):
    req = json.load(open(os.path.join(FX, f"{name}.request.json")))["model"]
    nn, mm, gg = len(req["nodes"]), len(req["pipe_segments"]), len(req["supports"])
    for mode in ("sparse_interactive", "dense_scrutiny"):
        e = json.load(open(os.path.join(FX, f"{name}-{mode}.raw.json")))
        cases = req["load_cases"]
        empty = [c["id"] for c in cases if c.get("pressure_regions") == []]
        per_case = 7 * nn + 51 * mm + 8 * gg
        by_case = {}
        for r in e["results"]:
            b = (r.get("basis_ref") or {}).get("ref_id")
            by_case[b] = by_case.get(b, 0) + 1
        rows.append({"fixture": name, "mode": mode, "n": nn, "m": mm, "g": gg, "cases": len(cases),
                     "empty_region_cases": empty, "rows": len(e["results"]), "rows_by_basis_ref": by_case,
                     "7n+51m+8g": per_case, "P_bound_7n+51m+8g+3": per_case + 3,
                     "producer": e["producer"]["semantic_contract_id"],
                     "contract_evidence_keys": sorted(e["contract_evidence"].keys())})

# ---------------------------------------------------------------- 2. evidence facts at the caps
src = json.load(open(os.path.join(FX, "fields-sparse_interactive.raw.json")))
req = json.load(open(os.path.join(FX, "fields.request.json")))["model"]
assert len(req["nodes"]) == 2 and len(req["pipe_segments"]) == 1
case = src["contract_evidence"]["exact_cases"][0]
PER_PIPE = ("pipe_materials", "pipe_sections", "pipe_stress_extrema")
for k in PER_PIPE:
    assert len(case[k]) == 1, k
rhs = case["pressure_rhs_assembly"]
DOF_KEYS = ("assembled_pressure_rhs_global", "rounded_cap_rhs_global", "rounded_poisson_rhs_global")
for k in DOF_KEYS:
    assert len(rhs[k]) == 12, k
assert rhs["groups"] == [] and len(rhs["node_order"]) == 2
# the per-case remainder: the case with every per-pipe array emptied, every DOF array emptied, no node ids
rest = json.loads(json.dumps(case))
for k in PER_PIPE:
    rest[k] = []
for k in DOF_KEYS:
    rest["pressure_rhs_assembly"][k] = []
rest["pressure_rhs_assembly"]["node_order"] = []
rest["stress_maximum_coverage"] = {k: [] if isinstance(v, list) else v for k, v in case["stress_maximum_coverage"].items()}
f_rest = facts(rest)
f_pipe = {k: facts(case[k][0]) for k in PER_PIPE}
per_pipe = facts({})
for k in PER_PIPE:
    per_pipe = add(per_pipe, f_pipe[k])
    per_pipe["arr"] += 1                       # the element's own array slot
per_dof = dict(arr=3, obj=0, ent=0, strb=0, keyb=0, nums=3, depth=0)        # one slot and number in each of the three vectors
per_node = dict(arr=1, obj=0, ent=0, strb=STR_CAP, keyb=0, nums=0, depth=0)  # node_order entry
coverage = dict(arr=1, obj=0, ent=0, strb=STR_CAP, keyb=0, nums=0, depth=0)  # an unavailable pipe id (<= m)
f_case = add(add(add(add(f_rest, per_pipe, m), per_dof, 6 * n), per_node, n), coverage, m)
f_case["obj"] += 0
# the evidence's top level per case adds pressure [] and connector [] (empty: no regions, no components)
exact = dict(f_case)
preview = dict(arr=3 * m + 2 * g, obj=3 + m + g, ent=9 + 15 * m + 2 * g,
               strb=m * (128 + 1024 + 3 * 120) + (2 * m + g) * 128 + g * (128 + 64),
               keyb=(9 + 15 * m + 2 * g) * 40, nums=10 * m, depth=6)
mx = {k: max(exact[k], preview[k]) for k in preview}

json.dump({"rows": rows, "string_cap": STR_CAP, "caps": dict(n=n, m=m, g=g),
           "exact_evidence_case_parts": {"rest": f_rest, "per_pipe": per_pipe, "per_pipe_by_array": f_pipe,
                                         "per_dof": per_dof, "per_node": per_node, "per_unavailable_pipe": coverage},
           "exact_evidence_per_case_at_caps": exact, "preview_tree_per_case_at_caps": preview,
           "componentwise_max": mx,
           "fixture_case_bytes": len(json.dumps(case))}, open(out, "w"), indent=1)
print(json.dumps({"exact": exact, "preview": preview, "max": mx}))
for r in rows:
    print(r["fixture"], r["mode"], "rows", r["rows"], "per-case 7n+51m+8g", r["7n+51m+8g"], r["rows_by_basis_ref"], r["producer"])
