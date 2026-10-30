"""Route E's exact census (stdlib only, read-only): I95's B3-S method (b3s_census.py at ddac979e61, archived by
11718e61a2), on the current pressure-source-1 producer fixtures, with D-5's per-basis arrays (U6-RP).

1. Rows: every committed exact-route output case with explicitly empty pressure regions must publish exactly
   7n + Rm + 8g rows (R rows per member), checked per fixture, mode and case; R is the one value all such cases
   share, written as `rows_per_member` (51 before D-5, 72 under it).
2. The per-case contract evidence facts (array slots, objects, entries, string bytes, key bytes, numbers, depth)
   of one case of the m = 1 fixture `fields`, split into its per-member arrays (every entry of pipe_materials,
   pipe_sections and pipe_stress_extrema: one entry per member each, or one per member and section basis), the
   three 6n RHS vectors, the node list and the rest; each stress_maximum_coverage record's unavailable_pipe_ids
   list holds <= m ids (one list before D-5, one per basis under it). Every string is bounded by max(its length,
   STR_CAP): identifiers are <= 128 B (D1 text cap) and the longest composed string, the elastic-maximum result
   id, is <= 23 + 2 * (3 + 1 + 128) = 287 B, plus D-5's ':' and basis token (<= 16 B): 303 B < STR_CAP = 304.
   U6-RL (T4-U6/U8 D3 review R2): a source-selected extremum's `locations` (composite.rs maximum_inner) takes one of
   three shapes, and a fixture shows one of them, so each extremum is counted at the componentwise maximum of its
   facts over the three (strict_endpoint's two strings; endpoint_candidates' three entries and 43 key bytes).
3. The componentwise maximum with the preview tree's facts at D1's caps (census_patch.py substitutes it).
Usage: python3 census_exact.py <projects/chirality-piping dir> <out json>
"""
import json, os, sys

P, out = sys.argv[1:3]
FX = os.path.join(P, "fixtures/product_preview/pressure_source/physics_source")
STR_CAP = 304
n, m, g = 32, 32, 32                      # D1's model caps
# U6-RL: the producer's three `locations` shapes, read from the source that builds them; the census stops unless
# maximum_inner's match builds exactly these (the Locations enum has these three variants).
COMPOSITE = os.path.join(P, "core/product_physics/src/source_receipt/composite.rs")
LOCATION_SOURCES = {
    "strict_endpoint": 'json!({"kind":"strict_endpoint","endpoint":endpoint_name(e)})',
    "whole_span_constant": 'maximum::Locations::WholeSpanConstant => json!({"kind":"whole_span_constant"}),',
    "endpoint_candidates": 'json!({"kind":"endpoint_candidates","exact_tie_proven":exact_tie_proven,'
                           '"interior_equal_possible":interior_equal_possible})',
}
LOCATION_SHAPES = {
    "strict_endpoint": {"kind": "strict_endpoint", "endpoint": "i"},
    "whole_span_constant": {"kind": "whole_span_constant"},
    "endpoint_candidates": {"kind": "endpoint_candidates", "exact_tie_proven": False, "interior_equal_possible": False},
}
_composite = open(COMPOSITE, encoding="utf-8").read()
if _composite.count("maximum::Locations::") != 3 or any(_composite.count(t) != 1 for t in LOCATION_SOURCES.values()):
    raise SystemExit("census: composite.rs no longer builds exactly the three reviewed `locations` shapes; re-review U6-RL")


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


def entry_facts(entry):
    """One per-pipe entry's facts. A source-selected extremum (it carries `locations`) is counted at the
    componentwise maximum of its facts over the three `locations` shapes (U6-RL)."""
    if not (isinstance(entry, dict) and "locations" in entry):
        return facts(entry)
    if entry["locations"].get("kind") not in LOCATION_SHAPES:
        raise SystemExit(f"census: unreviewed locations shape {entry['locations']}")
    shaped = [facts(dict(entry, locations=shape)) for shape in LOCATION_SHAPES.values()]
    return {k: max(f[k] for f in shaped) for k in shaped[0]}


# ---------------------------------------------------------------- 1. rows
rows, per_member = [], set()
for name in sorted({f.split(".request.json")[0] for f in os.listdir(FX) if f.endswith(".request.json")}):
    req = json.load(open(os.path.join(FX, f"{name}.request.json")))["model"]
    nn, mm, gg = len(req["nodes"]), len(req["pipe_segments"]), len(req["supports"])
    for mode in ("sparse_interactive", "dense_scrutiny"):
        e = json.load(open(os.path.join(FX, f"{name}-{mode}.raw.json")))
        empty = [c["id"] for c in req["load_cases"] if c.get("pressure_regions") == []]
        by_case = {}
        for r in e["results"]:
            b = (r.get("basis_ref") or {}).get("ref_id")
            by_case[b] = by_case.get(b, 0) + 1
        for c in empty:
            rest = by_case.get(c, 0) - 7 * nn - 8 * gg
            if rest % mm:
                raise SystemExit(f"census: {name}-{mode} case {c}: rows {by_case.get(c)} are not 7n + Rm + 8g")
            per_member.add(rest // mm)
        rows.append({"fixture": name, "mode": mode, "n": nn, "m": mm, "g": gg, "cases": len(req["load_cases"]),
                     "empty_region_cases": empty, "rows": len(e["results"]), "rows_by_basis_ref": by_case,
                     "producer": e["producer"]["semantic_contract_id"],
                     "contract_evidence_keys": sorted(e["contract_evidence"].keys())})
if len(per_member) != 1:
    raise SystemExit(f"census: the empty-region cases do not share one rows-per-member value: {sorted(per_member)}")
R = per_member.pop()
for r in rows:
    r[f"7n+{R}m+8g"] = 7 * r["n"] + R * r["m"] + 8 * r["g"]

# ---------------------------------------------------------------- 2. evidence facts at the caps
src = json.load(open(os.path.join(FX, "fields-sparse_interactive.raw.json")))
req = json.load(open(os.path.join(FX, "fields.request.json")))["model"]
assert len(req["nodes"]) == 2 and len(req["pipe_segments"]) == 1
case = src["contract_evidence"]["exact_cases"][0]
PER_PIPE = ("pipe_materials", "pipe_sections", "pipe_stress_extrema")
for k in PER_PIPE:
    assert len(case[k]) >= 1, k
# The census case's extrema are the source-selected maximum's (composite.rs maximum_inner), the larger of route E's
# two extremum forms in every fact; the certified/fallback form (lib.rs's case solve) has no `locations`.
assert all("locations" in e for e in case["pipe_stress_extrema"]), "census: the census case is not source-selected"
rhs = case["pressure_rhs_assembly"]
DOF_KEYS = ("assembled_pressure_rhs_global", "rounded_cap_rhs_global", "rounded_poisson_rhs_global")
for k in DOF_KEYS:
    assert len(rhs[k]) == 12, k
assert rhs["groups"] == [] and len(rhs["node_order"]) == 2
coverage_lists = case["stress_maximum_coverage"]
if isinstance(coverage_lists, dict):
    coverage_lists = [coverage_lists]
# the per-case remainder: every per-pipe array emptied, every DOF array emptied, no node ids, no unavailable ids
rest = json.loads(json.dumps(case))
for k in PER_PIPE:
    rest[k] = []
for k in DOF_KEYS:
    rest["pressure_rhs_assembly"][k] = []
rest["pressure_rhs_assembly"]["node_order"] = []
for record in (rest["stress_maximum_coverage"] if isinstance(rest["stress_maximum_coverage"], list) else [rest["stress_maximum_coverage"]]):
    record.update({k: [] for k, v in record.items() if isinstance(v, list)})
f_rest = facts(rest)
f_pipe = {}
for k in PER_PIPE:
    f_pipe[k] = dict(arr=0, obj=0, ent=0, strb=0, keyb=0, nums=0, depth=0)
    for entry in case[k]:
        f_pipe[k] = add(f_pipe[k], entry_facts(entry))
        f_pipe[k]["arr"] += 1                  # the entry's own array slot
per_pipe = facts({})
for k in PER_PIPE:
    per_pipe = add(per_pipe, f_pipe[k])
per_dof = dict(arr=3, obj=0, ent=0, strb=0, keyb=0, nums=3, depth=0)        # one slot and number in each of the three vectors
per_node = dict(arr=1, obj=0, ent=0, strb=STR_CAP, keyb=0, nums=0, depth=0)  # node_order entry
# an unavailable pipe id (<= m) in each coverage list
coverage = dict(arr=len(coverage_lists), obj=0, ent=0, strb=STR_CAP * len(coverage_lists), keyb=0, nums=0, depth=0)
exact = add(add(add(add(f_rest, per_pipe, m), per_dof, 6 * n), per_node, n), coverage, m)
preview = dict(arr=3 * m + 2 * g, obj=3 + m + g, ent=9 + 15 * m + 2 * g,
               strb=m * (128 + 1024 + 3 * 120) + (2 * m + g) * 128 + g * (128 + 64),
               keyb=(9 + 15 * m + 2 * g) * 40, nums=10 * m, depth=6)
mx = {k: max(exact[k], preview[k]) for k in preview}

json.dump({"rows": rows, "rows_per_member": R, "string_cap": STR_CAP, "caps": dict(n=n, m=m, g=g),
           "per_pipe_entries": {k: len(case[k]) for k in PER_PIPE}, "coverage_lists": len(coverage_lists),
           "locations_shapes": {k: facts(v) for k, v in LOCATION_SHAPES.items()},
           "exact_evidence_case_parts": {"rest": f_rest, "per_pipe": per_pipe, "per_pipe_by_array": f_pipe,
                                         "per_dof": per_dof, "per_node": per_node, "per_unavailable_pipe": coverage},
           "exact_evidence_per_case_at_caps": exact, "preview_tree_per_case_at_caps": preview,
           "componentwise_max": mx,
           "fixture_case_bytes": len(json.dumps(case))}, open(out, "w"), indent=1)
print(json.dumps({"rows_per_member": R, "exact": exact, "preview": preview, "max": mx}))
