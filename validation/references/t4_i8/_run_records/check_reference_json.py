"""T4-I8: consumer-side check of rebuilt_reference_cases.json (standard library only). Repair round 01,
extended for the D-5 re-freeze of case 1 (T4-U6).

Usage: python -I check_reference_json.py <rebuilt_reference_cases.json> [<earlier frozen json>]

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
   every v2 document has p >= 0 (S-4 c) and points to /sp1_pair_scope; case 1's current-format (0.5.0)
   forms agree likewise, carry the current contract and the explicit wall, mill tolerance and allowance;
6. D-5 (case 1): stress rows carry section_basis and the row key includes it; both families present for
   every stress kind; each discriminator's target resolves and is distinct;
7. optional: against an earlier frozen file, every quantity of cases 2-5 present in both under the same
   pointer has the same exact form and binary64 value (no frozen value changed); for case 1, the
   quantities D-5 keeps (Nw, S, reduced-section sigma_z, Lame and maximum, reactions, P, the force and
   reduced stress scales, and the carried-over discriminators) are unchanged and the declared movers
   (node:B ux free, scale_length, scale_rotation) moved.
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
    keys = [(r["kind"], r["entity_ref"], r["component"], r["location"], r.get("section_basis")) for r in lc["rows"]]
    if len(keys) != len(set(keys)):
        fail(f"{base}: duplicate row selectors")
    based = [r for r in lc["rows"] if "section_basis" in r]
    if based:  # D-5: every stress kind on both bases, no basis on any other row
        counts["section_basis_rows"] = counts.get("section_basis_rows", 0) + len(based)
        stress = {"pipe_axial_membrane_stress_v2", "pipe_lame_radial_stress_v2", "pipe_lame_hoop_stress_v2",
                  "element_local_bending_normal_stress_y", "element_local_bending_normal_stress_z",
                  "element_local_torsional_shear_stress", "pipe_elastic_normal_stress_maximum_v2"}
        for r in lc["rows"]:
            if (r["kind"] in stress) != ("section_basis" in r):
                fail(f"{base}: section_basis on {r['kind']} {r['location']} is {r.get('section_basis')}")
        nom = {k[:4] for k in keys if k[4] == "nominal_section"}
        red = {k[:4] for k in keys if k[4] == "reduced_section"}
        if nom != red or not nom:
            fail(f"{base}: the two stress families differ in their selectors")


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


CURRENT_CONTRACT = {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"}


def check_documents_d5(name, docs):
    """Case 1 after its D-5 re-authoring: model 0.5.0 under the current contract, without and with the
    load/reference-state group; the true wall with explicit mill tolerance and allowance; no v2 twin."""
    d, g = docs["v3_model_0.5.0"]["model"], docs["v3_model_0.5.0_load_state_group"]["model"]
    counts["documents"] += 2
    if set(docs) != {"v3_model_0.5.0", "v3_model_0.5.0_load_state_group", "shear_model_variant", "sp1_pair", "note"}:
        fail(f"{name}: document keys {sorted(docs)}")
    if d["schema_version"] != "0.5.0" or g["schema_version"] != "0.5.0":
        fail(f"{name}: schema versions")
    if d["pressure_contract"] != CURRENT_CONTRACT or g["pressure_contract"] != CURRENT_CONTRACT:
        fail(f"{name}: contract")
    for key in ("nodes", "pipe_segments", "supports"):
        if d[key] != g[key]:
            fail(f"{name}: {key} differ between the two forms")
    for p in d["pipe_segments"]:
        sec = p["section"]
        got = {k: (sec[k]["value"], sec[k]["unit"]) for k in sec}
        if got != {"outside_diameter": (0.2, "m"), "wall_thickness": (0.01, "m"), "mill_tolerance": (0.00125, "m"),
                   "corrosion_allowance": (0.002, "m")}:
            fail(f"{name}: section {got}")
    if "reference_configurations" in d or any("analysis_state" in c for c in d["load_cases"]):
        fail(f"{name}: the plain form carries the load/reference-state group")
    if "reference_configurations" not in g or not all("analysis_state" in c for c in g["load_cases"]):
        fail(f"{name}: the group form lacks the load/reference-state group")
    for c3, c4 in zip(d["load_cases"], g["load_cases"]):
        if c3["pressure_regions"] != c4["pressure_regions"]:
            fail(f"{name}: regions differ")
        src = {s["source_ref"] for s in c4["analysis_state"]["load_sources"]}
        if src != {l["id"] for l in c4["primitive_loads"]}:
            fail(f"{name}: load sources incomplete")
        if {s["support_ref"] for s in c4["analysis_state"]["support_states"]} != {s["id"] for s in g["supports"]}:
            fail(f"{name}: support states incomplete")
    v = docs["shear_model_variant"]
    if (v["op"], v["path"], v["value"]) != ("replace", "/model/pressure_contract/shear_model", "timoshenko_annulus_energy_v1"):
        fail(f"{name}: shear-model variant patch")
    if not docs["sp1_pair"].startswith("not applicable"):
        fail(f"{name}: case 1 has no SP-1 twin after D-5")


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
    d5_case = "/cases/milltol_lame_membrane"
    d5 = "v3_model_0.5.0" in new["cases"]["milltol_lame_membrane"]["variants"]["free_transferring"]["documents"]
    old_single = "v2_model_0.3.0" in old["cases"]["milltol_lame_membrane"]["variants"]["free_transferring"]["documents"]
    for path, q in leaves(old["cases"], "/cases"):
        if d5 and old_single and path.startswith(d5_case + "/"):
            continue  # case 1 across its D-5 re-freeze is compared by the mapping below
        try:
            n = resolve(new, path)
        except (KeyError, IndexError, ValueError, TypeError):
            continue
        counts["frozen_compared"] += 1
        if n.get("exact") != q["exact"] or n.get("value") != q["value"] or n.get("unit") != q["unit"]:
            fail(f"frozen value changed at {path}")
    if d5 and old_single:
        compare_milltol_d5(new["cases"]["milltol_lame_membrane"], old["cases"]["milltol_lame_membrane"])
    # round 00's case-2 side block of support magnitudes moved into rows (N-3): values must be unchanged
    for cid, lc in old["cases"]["tp_phys_pressure_halves"]["load_cases"].items():
        if "support_magnitudes" not in lc:
            continue  # only round 00 had the side block
        e = new["cases"]["tp_phys_pressure_halves"]["load_cases"][cid]["expected"]
        for sup, tag in (("support:A", "A"), ("support:D", "D")):
            for k in ("force_magnitude", "moment_magnitude"):
                q = lc["support_magnitudes"][sup][k]
                n = e.get(f"{tag}_{k}") or e["zero_force" if k[0] == "f" else "zero_moment"]
                counts["frozen_compared"] += 1
                if n["value"] != q["value"]:
                    fail(f"moved support magnitude changed: {cid} {sup} {k}")


def compare_milltol_d5(new, old):
    """Case 1, single-section freeze -> D-5 re-freeze: what D-5 keeps is unchanged, what it moves moved."""
    def same(a, b):
        return a["exact"] == b["exact"] and a["value"] == b["value"] and a["unit"] == b["unit"]
    keep = {"Nw": "Nw", "S": "S", "wall_end_action_i": "wall_end_action_i", "wall_end_action_j": "wall_end_action_j",
            "sigma_z": "sigma_z_reduced_section", "lame_inner_radial": "lame_inner_radial_reduced_section",
            "lame_outer_radial": "lame_outer_radial_reduced_section", "lame_inner_hoop": "lame_inner_hoop_reduced_section",
            "lame_outer_hoop": "lame_outer_hoop_reduced_section", "AB_maximum": "AB_maximum_reduced_section",
            "zero_force": "zero_force", "zero_moment": "zero_moment", "zero_stress": "zero_stress",
            "zero_length": "zero_length", "zero_rotation": "zero_rotation"}
    for variant, ov in old["variants"].items():
        olc = list(ov["load_cases"].values())[0]
        nlc = list(new["variants"][variant]["load_cases"].values())[0]
        for name, q in olc["expected"].items():
            if name in keep:
                counts["frozen_compared"] += 1
                if not same(q, nlc["expected"][keep[name]]):
                    fail(f"milltol {variant}: {name} changed under D-5")
            elif name.startswith(("A_", "B_F", "B_M", "B_force", "B_moment")):
                counts["frozen_compared"] += 1
                if not same(q, nlc["expected"][name]):
                    fail(f"milltol {variant}: reaction {name} changed under D-5")
            elif name == "B_ux":
                counts["frozen_compared"] += 1
                n = nlc["expected"]["B_ux"]
                if same(q, n) or not (0 < n["value"] < q["value"]):
                    fail(f"milltol {variant}: B_ux did not move to the stiffer nominal value")
            else:
                fail(f"milltol {variant}: unmapped frozen quantity {name}")
        for name, q in olc["zero_scales"].items():
            counts["frozen_compared"] += 1
            if name in ("scale_force", "scale_moment"):
                ok = same(q, nlc["zero_scales"][name])
            elif name == "scale_stress":
                ok = same(q, nlc["zero_scales"]["scale_stress_reduced_section"])
            else:  # scale_length, scale_rotation: the free extension on the stiffer section is smaller (a tighter floor)
                ok = 0 < nlc["zero_scales"][name]["value"] < q["value"]
            if not ok:
                fail(f"milltol {variant}: zero scale {name} not as D-5 requires")
    old_d = {d["id"]: d for d in old["wrong_result_discriminators"]}
    carried = 0
    for d in new["wrong_result_discriminators"]:  # carried over (cap_area_on_nominal_bore activated): same wrong value
        src = old_d.get(d.get("former_id", d["id"]))
        if src is not None:
            carried += 1
            counts["frozen_compared"] += 1
            if src["wrong"] != d["wrong"]:
                fail(f"milltol discriminator {d['id']} changed its wrong value")
    if carried != len(old_d):
        fail(f"milltol: {len(old_d) - carried} frozen discriminators were dropped")
    for k in ("ro", "P"):
        counts["frozen_compared"] += 1
        if not same(old["derived"][k], new["derived"][k]):
            fail(f"milltol derived {k} changed")


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
            if "v3_model_0.5.0" in h["documents"]:
                check_documents_d5(base, h["documents"])
            else:
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
    ids = {d["id"]: d for d in m["wrong_result_discriminators"]}
    s3 = ids.get("cap_area_on_bore_without_mill_tolerance")
    if not s3 or s3.get("former_id") != "cap_area_on_authored_bore" or \
            s3["wrong"]["exact"] != {"kind": "rational", "rational": "270848000/20871"} or s3["wrong"]["value"] != 12977.241148004408:
        fail("S-3 control missing or wrong")
    if "producible_today" in ids.get("cap_area_on_nominal_bore", {"producible_today": None}):
        fail("cap_area_on_nominal_bore is not active after the D-5 re-authoring")
    for dsc in m["wrong_result_discriminators"]:
        counts["discriminators"] += 1
        for variant in dsc["target"]["variants"]:
            e = list(m["variants"][variant]["load_cases"].values())[0]["expected"]
            right = e.get(dsc["target"]["expected"])
            if right is None:
                fail(f"milltol discriminator {dsc['id']} targets a missing quantity {dsc['target']}")
            elif not distinct(right["value"], dsc["wrong"]["value"]) or right["unit"] != dsc["wrong"]["unit"]:
                fail(f"milltol discriminator {dsc['id']} not distinct or not in the target unit ({variant})")
    pc = m["refreeze"]["prediction_check"]["entries"]
    bad = [e for e in pc if not e["agrees"] and "radial" not in e["quantity"]]
    if bad or not all(e["outcome"] == "unchanged" for e in pc if "radial" in e["quantity"]):
        fail(f"milltol D-5 prediction check: unexplained disagreements {bad}")
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
