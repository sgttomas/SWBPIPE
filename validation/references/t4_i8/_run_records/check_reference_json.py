"""T4-I8: consumer-side check of rebuilt_reference_cases.json (standard library only). Repair round 01.

Usage: python -I check_reference_json.py <rebuilt_reference_cases.json> [<frozen round-00 json>]

Checks, on the written bytes only:
1. every quantity's `exact` re-evaluates (Decimal, 120 digits, pi from the file) to its `decimal`
   within 1e-80 relative and to its `value` bitwise (the load_reference_1 generator rule); symbolic
   forms 'sqrt((a*pi)^2 + (b)^2)' and 'a + (b)/pi' carry `evaluation` at quantity level;
2. T1 transport (T4-RV4 S-1): every row's reference_origin is {kind analytical, pointer,
   reference_unit, transform identity[, zero_scale {tag, base {pointer, reference_unit}, scale_unit}]};
   each pointer resolves (RFC 6901) to a maintained quantity with exactly T1's key set and the declared
   unit; the row unit is reached by an exact factor (identity, m->mm, Pa->MPa); a zero expectation
   has a nonzero scale and absolute_tolerance == 1e-9 * |scale| * factor in the row unit; nonzero
   rows carry relative 1e-9; every zero scale has a definition in zero_scale_definitions;
3. re-freeze tags (S-2): exactly the nonzero case-2 combined uy rows carry the T4-U8 tag;
4. discriminators: every wrong value is distinct from its correct value under the negative rule,
   and S-3's producible control equals 270848000/20871 Pa;
5. documents: v2 0.3.0/0.4.0 agree on geometry, E/nu, supports, regions, thermal states and sources;
   every v2 document has p >= 0 (S-4 c) and points to /sp1_pair_scope;
6. optional: against the frozen round-00 file, every quantity present in both under the same pointer
   has the same exact form and binary64 value (no frozen value changed).
"""
import json
import sys
from decimal import Decimal, getcontext

getcontext().prec = 120
fails = []
counts = {"quantities": 0, "rows": 0, "zero_rows": 0, "discriminators": 0, "documents": 0,
          "refreeze_rows": 0, "frozen_compared": 0}
T1_KEYS = {"unit", "exact", "decimal", "value"}
FACTOR = {("m", "mm"): Decimal(1000), ("Pa", "MPa"): Decimal(1) / Decimal(1000000)}


def fail(msg):
    fails.append(msg)
    print("FAIL", msg)


def frac(text):
    if "/" in text:
        n, d = text.split("/")
        return Decimal(int(n)) / Decimal(int(d))
    return Decimal(int(text))


def evaluate(q, pi):
    e = q["exact"]
    k = e["kind"]
    if k == "rational":
        return frac(e["rational"])
    if k == "rational_times_pi":
        return frac(e["rational"]) * pi
    if k == "rational_over_pi":
        return frac(e["rational"]) / pi
    if k == "symbolic":
        expr = e["expression"]
        if expr.startswith("sqrt(("):
            a_txt, b_txt = expr[len("sqrt(("):].split("*pi)^2 + (")
            b_txt = b_txt[: -len(")^2)")]
            return ((frac(a_txt) * pi) ** 2 + frac(b_txt) ** 2).sqrt()
        a_txt, b_txt = expr.split(" + ")
        assert b_txt.startswith("(") and b_txt.endswith(")/pi")
        return frac(a_txt) + frac(b_txt[1:-4]) / pi
    raise ValueError(k)


def walk(node, path, pi):
    if isinstance(node, dict):
        if "exact" in node and "decimal" in node and "value" in node:
            counts["quantities"] += 1
            keys = T1_KEYS | ({"evaluation"} if node["exact"]["kind"] == "symbolic" else set())
            if set(node) != keys:
                fail(f"{path}: key set {sorted(node)} is not T1's")
            v = evaluate(node, pi)
            d = Decimal(node["decimal"])
            if v != 0 and abs(d - v) > abs(v) * Decimal(10) ** -80:
                fail(f"{path}: decimal disagrees with exact")
            if v == 0 and d != 0:
                fail(f"{path}: nonzero decimal for zero")
            if float(v) != node["value"]:
                fail(f"{path}: value not the binary64 rounding of exact")
            return
        for key, child in node.items():
            walk(child, f"{path}/{key}", pi)
    elif isinstance(node, list):
        for i, child in enumerate(node):
            walk(child, f"{path}/{i}", pi)


def resolve(doc, pointer):
    node = doc
    for raw in pointer[1:].split("/"):
        key = raw.replace("~1", "/").replace("~0", "~")
        node = node[int(key)] if isinstance(node, list) else node[key]
    return node


def check_load_case(doc, base, lc, pi):
    exp, scales, defs = lc["expected"], lc["zero_scales"], lc.get("zero_scale_definitions", {})
    for name in scales:
        if not defs.get(name):
            fail(f"{base}: zero scale {name} has no definition")
    for r in lc["rows"]:
        counts["rows"] += 1
        o = r["reference_origin"]
        if o.get("kind") != "analytical" or o.get("transform") != "identity":
            fail(f"{base}: origin kind/transform {o}")
        try:
            q = resolve(doc, o["pointer"])
        except (KeyError, IndexError, ValueError):
            fail(f"{base}: pointer does not resolve {o['pointer']}")
            continue
        if q is not exp.get(r["expected"]) and q != exp.get(r["expected"]):
            fail(f"{base}: pointer and expected name disagree {o['pointer']}")
        if q["unit"] != o["reference_unit"]:
            fail(f"{base}: unit {q['unit']} != declared {o['reference_unit']}")
        if o["reference_unit"] == r["unit"]:
            factor = Decimal(1)
        elif (o["reference_unit"], r["unit"]) in FACTOR:
            factor = FACTOR[(o["reference_unit"], r["unit"])]
        else:
            fail(f"{base}: no exact factor {o['reference_unit']} -> {r['unit']}")
            continue
        c = r["criterion"]
        if q["value"] == 0:
            counts["zero_rows"] += 1
            zs = o.get("zero_scale")
            if c["kind"] != "zero_scale" or not zs:
                fail(f"{base}: zero row without zero scale {r['kind']} {r['entity_ref']} {r['location']}")
                continue
            sq = resolve(doc, zs["base"]["pointer"])
            if sq["unit"] != zs["base"]["reference_unit"] or zs["scale_unit"] != sq["unit"] or sq["value"] == 0:
                fail(f"{base}: bad zero scale {zs}")
                continue
            want = float(Decimal("1e-9") * abs(evaluate(sq, pi)) * factor)
            if c["absolute_tolerance"] != want:
                fail(f"{base}: zero tolerance {c['absolute_tolerance']} != {want}")
        else:
            if c["kind"] != "relative" or c["relative_tolerance"] != 1e-9 or "zero_scale" in o:
                fail(f"{base}: nonzero row criterion {c}")
        if "refreeze" in r:
            counts["refreeze_rows"] += 1
    keys = [(r["kind"], r["entity_ref"], r["component"], r["location"]) for r in lc["rows"]]
    if len(keys) != len(set(keys)):
        fail(f"{base}: duplicate row selectors")


def distinct(correct, wrong):
    return abs(correct - wrong) > 1e-9 * max(abs(correct), abs(wrong))


def check_documents(name, docs):
    d3, d4 = docs["v2_model_0.3.0"]["model"], docs["v2_model_0.4.0"]["model"]
    counts["documents"] += 2
    if docs.get("sp1_pair") != "/sp1_pair_scope":
        fail(f"{name}: documents do not point to /sp1_pair_scope")
    if d3["schema_version"] != "0.3.0" or d4["schema_version"] != "0.4.0":
        fail(f"{name}: schema versions")
    for key in ("nodes", "pipe_segments", "supports"):
        if d3[key] != d4[key]:
            fail(f"{name}: {key} differ between 0.3.0 and 0.4.0")
    for m3, m4 in zip(d3["materials"], d4["materials"]):
        if (m3["elastic_modulus"], m3["poisson_ratio"]) != (m4["elastic_modulus"], m4["poisson_ratio"]):
            fail(f"{name}: material E/nu differ")
    for c3, c4 in zip(d3["load_cases"], d4["load_cases"]):
        if c3["pressure_regions"] != c4["pressure_regions"]:
            fail(f"{name}: regions differ")
        if any(reg["pressure"]["value"] < 0 for reg in c3["pressure_regions"]):
            fail(f"{name}: p < 0 has no v2 twin")
        th3 = {l["target"]["pipe"]: l["magnitude"]["value"] for l in c3["primitive_loads"] if l["category"] == "thermal"}
        th4 = {e["pipe_ref"]: e["thermal_state"]["temperature_change"]["value"] for e in c4["analysis_state"]["element_states"]
               if e["thermal_state"]["kind"] == "constant_alpha_interval"}
        if th3 != th4:
            fail(f"{name}: thermal states differ {th3} {th4}")
        src = {s["source_ref"] for s in c4["analysis_state"]["load_sources"]}
        if src != {l["id"] for l in c4["primitive_loads"]} or any(l["category"] == "thermal" for l in c4["primitive_loads"]):
            fail(f"{name}: 0.4.0 load sources incomplete or a thermal primitive remains")
        if {s["support_ref"] for s in c4["analysis_state"]["support_states"]} != {s["id"] for s in d4["supports"]}:
            fail(f"{name}: support states incomplete")
    if docs["v3_patch_for_both"]["value"] != {"version": "3.0.0", "mode": "exact_pressure_v3"}:
        fail(f"{name}: v3 patch")


def holders(cases):
    for key, case in cases.items():
        yield f"/cases/{key}", case
        for vk, v in case.get("variants", {}).items():
            yield f"/cases/{key}/variants/{vk}", v


def compare_frozen(new, old):
    """Every maintained quantity of the frozen file whose pointer still exists keeps its exact and value."""
    def leaves(node, path):
        if isinstance(node, dict):
            if "exact" in node and "decimal" in node and "value" in node:
                yield path, node
                return
            for k, v in node.items():
                yield from leaves(v, f"{path}/{k}")
        elif isinstance(node, list):
            for i, v in enumerate(node):
                yield from leaves(v, f"{path}/{i}")
    for path, q in leaves(old["cases"], "/cases"):
        try:
            n = resolve(new, path)
        except (KeyError, IndexError, ValueError, TypeError):
            continue
        counts["frozen_compared"] += 1
        if n.get("exact") != q["exact"] or n.get("value") != q["value"] or n.get("unit") != q["unit"]:
            fail(f"frozen value changed at {path}")
    # round 00's case-2 side block of support magnitudes moved into rows (N-3): values must be unchanged
    for cid, lc in old["cases"]["tp_phys_pressure_halves"]["load_cases"].items():
        e = new["cases"]["tp_phys_pressure_halves"]["load_cases"][cid]["expected"]
        for sup, tag in (("support:A", "A"), ("support:D", "D")):
            for k in ("force_magnitude", "moment_magnitude"):
                q = lc["support_magnitudes"][sup][k]
                n = e.get(f"{tag}_{k}") or e["zero_force" if k[0] == "f" else "zero_moment"]
                counts["frozen_compared"] += 1
                if n["value"] != q["value"]:
                    fail(f"moved support magnitude changed: {cid} {sup} {k}")


def main():
    doc = json.loads(open(sys.argv[1], "rb").read())
    pi = Decimal(doc["numeric_representation"]["pi_decimal"])
    walk(doc, "", pi)
    cases = doc["cases"]
    if "sp1_pair_scope" not in doc or "closed_exclusion_list" not in doc["sp1_pair_scope"]:
        fail("no closed SP-1 pair scope")
    for base, h in holders(cases):
        for cid, lc in h.get("load_cases", {}).items():
            check_load_case(doc, f"{base}/load_cases/{cid}", lc, pi)
        if "documents" in h:
            check_documents(base, h["documents"])
    # re-freeze tags: exactly the nonzero uy rows of case 2 combined
    tagged = [(r["entity_ref"], r["kind"]) for base, h in holders(cases) for lc in h.get("load_cases", {}).values()
              for r in lc["rows"] if "refreeze" in r]
    want = [(n, "global_nodal_displacement_y") for n in ("node:B", "node:C", "node:D")]
    if sorted(tagged) != sorted(want):
        fail(f"refreeze tags {tagged}")
    if cases["tp_phys_pressure_halves"].get("refreeze", {}).get("unit") != "T4-U8" or \
            cases["v3_straight_twin"]["expectation"].get("refreeze", {}).get("unit") != "T4-U8":
        fail("missing T4-U8 re-freeze blocks")
    # discriminators
    m = cases["milltol_lame_membrane"]
    mf = list(m["variants"]["free_transferring"]["load_cases"].values())[0]["expected"]
    mr = list(m["variants"]["axially_restrained_transferring"]["load_cases"].values())[0]["expected"]
    target = {"lame_inner_hoop": mf["lame_inner_hoop"]["value"], "sigma_z (free)": mf["sigma_z"]["value"],
              "sigma_z (restrained)": mr["sigma_z"]["value"], "Nw (restrained)": mr["Nw"]["value"]}
    ids = {d["id"]: d for d in m["wrong_result_discriminators"]}
    s3 = ids.get("cap_area_on_authored_bore")
    if not s3 or s3["wrong"]["exact"] != {"kind": "rational", "rational": "270848000/20871"} or s3["wrong"]["value"] != 12977.241148004408:
        fail("S-3 control missing or wrong")
    for dsc in m["wrong_result_discriminators"]:
        counts["discriminators"] += 1
        if not distinct(target[dsc["quantity"]], dsc["wrong"]["value"]):
            fail(f"milltol discriminator {dsc['id']} not distinct")
    t = cases["tp_phys_pressure_halves"]["load_cases"]
    for dsc in cases["tp_phys_pressure_halves"]["wrong_result_discriminators"]:
        counts["discriminators"] += 1
        e = t[dsc["case"]]["expected"]
        name = {"Nw": "Nw", "S": "S", "support:A Fx": "A_Fx", "support:A Mz": "A_Mz",
                "pipe:A-B midspan bending_moment_z": "A-B_midspan_bending_moment_z", "node:D uy": "D_uy"}[dsc["quantity"]]
        if not distinct(e[name]["value"], dsc["wrong"]["value"]):
            fail(f"tp discriminator {dsc['id']} not distinct")
    c3 = cases["pressure_membrane_thin_wall_limit"]["load_cases"]["case:membrane-free"]["expected"]
    for dsc in cases["pressure_membrane_thin_wall_limit"]["wrong_result_discriminators"]:
        counts["discriminators"] += 1
        names = ["lame_inner_hoop", "lame_outer_hoop"] if "outer" in dsc["quantity"] else (["sigma_z"] if dsc["quantity"] == "sigma_z" else ["lame_inner_hoop"])
        for n in names:
            if not distinct(c3[n]["value"], dsc["wrong"]["value"]):
                fail(f"membrane discriminator {dsc['id']} vs {n} not distinct")
    e = t["case:combined"]["expected"]
    if not (e["A_Fx"]["value"] == -e["S"]["value"] and e["D_Fx"]["value"] == e["S"]["value"]):
        fail("combined: root Fx != -S or stop Fx != S")
    if not e["Nw"]["value"] < 0 < t["case:pressure-half"]["expected"]["Nw"]["value"]:
        fail("sign: pressure-half Nw must be tension and combined Nw compression")
    if len(sys.argv) > 2:
        compare_frozen(doc, json.loads(open(sys.argv[2], "rb").read()))
    print("counts:", json.dumps(counts, sort_keys=True))
    print("RESULT:", "PASS" if not fails else f"FAIL ({len(fails)})")
    return 0 if not fails else 1


if __name__ == "__main__":
    sys.exit(main())
