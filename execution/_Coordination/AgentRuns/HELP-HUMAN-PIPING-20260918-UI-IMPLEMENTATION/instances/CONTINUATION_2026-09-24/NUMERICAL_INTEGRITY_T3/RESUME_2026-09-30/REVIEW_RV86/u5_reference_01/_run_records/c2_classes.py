"""RV86 check 2: count and recompute each successor's classes from its own bytes (D1 4.1.6 / 4.1.6.1),
independently of the reader and of U5's report; then compare with the receipt lists, the reader's
classifications (taken from U5's report) and with the oracle's own classification formula (oracle lines 130-155)."""
import json, math, struct, sys, hashlib
from fractions import Fraction as F
from pathlib import Path
bits = lambda x: struct.pack('>d', float(x)).hex()
dec = lambda h: struct.unpack('>d', bytes.fromhex(h))[0]
def ru(q):  # nearest, then next-up if below the exact value
    x = float(q); return math.nextafter(x, math.inf) if F(x) < q else x
succ_files, u5_report = sys.argv[1:-1], sys.argv[-1]
report = json.loads(Path(u5_report).read_text())
reader_rows = {m["summary"]["mode"]: {e["id"]: e for e in m["rows"]} for m in report["modes"]}
NONQ = {"linear_solver_mode_basis", "sparse_live_path_dense_parity_relative_delta", "modulus_basis_record", "combination_modulus_basis_record"}
TR = {"global_nodal_displacement_x", "global_nodal_displacement_y", "global_nodal_displacement_z", "displacement_magnitude"}
RO = {"global_nodal_rotation_x", "global_nodal_rotation_y", "global_nodal_rotation_z"}
FO = {"element_local_axial_force", "element_local_shear_force_y", "element_local_shear_force_z", "support_reaction_force_magnitude_v2"}
MO = {"element_local_torsional_moment", "element_local_bending_moment_y", "element_local_bending_moment_z", "support_reaction_moment_magnitude_v2"}
ST1 = {"element_local_axial_normal_stress", "element_local_bending_normal_stress_y", "element_local_bending_normal_stress_z", "element_local_torsional_shear_stress"}
K = {"element_local_axial_normal_stress": 1.0, "pipe_elastic_normal_stress_maximum_v2": dec("4006a09e667f3bcd")}
def unit_si(r):
    y, u = r["value"], r["unit"]
    return y / 1000.0 if u == "mm" else y * 1000.0 if u in ("kN", "kN*m") else y * 1e6 if u == "MPa" else y
def kind_of(r, dofs):
    k, u = r["kind"], r["unit"]
    if k in NONQ: return "non_quantity"
    comp = {"x": "U", "y": "U", "z": "U"}
    if k.startswith("global_nodal_displacement_") and (r["entity_ref"], "U" + k[-1].upper()) in dofs: return "input_derived"
    if k.startswith("global_nodal_rotation_") and (r["entity_ref"], "R" + k[-1].upper()) in dofs: return "input_derived"
    if k in TR and u in ("m", "mm"): return "translation"
    if k in RO and u == "rad": return "rotation"
    if k in FO and u in ("N", "kN"): return "force"
    if k in MO and u in ("N*m", "kN*m"): return "moment"
    if k == "support_reaction_component_v2": return "force" if u in ("N", "kN") else "moment" if u in ("N*m", "kN*m") else "not_covered"
    if k in ST1 and u in ("Pa", "MPa"): return "stress1"
    if k == "pipe_elastic_normal_stress_maximum_v2" and u == "Pa": return "stress2r2"
    return "not_covered"
out = {}
for path in succ_files:
    raw = Path(path).read_bytes(); doc = json.loads(raw)
    src = doc["source"]; sel = src["retained_precision"]["body"]["cases"][0]["selection"]
    mode = doc["invocation"]["solver_mode"]; rows = src["results"]
    dofs = {(d["node_id"], d["component"]) for d in sel["input_derived_dofs"]}
    model = doc["invocation"]["request"]["model"]
    # input_derived_dofs equals the rigid restraint set from the invocation (D1 rule 2a, spring supports excluded)
    rigid = {(s["node"], c) for s in model["supports"] for c in s.get("restraints", []) if s.get("family") != "spring" and not s["id"].startswith("spring")}
    kinds = [kind_of(r, dofs) for r in rows]
    S = {k: 0.0 for k in ["translation", "rotation", "force", "moment"]}
    for r, k in zip(rows, kinds):
        if k in S: S[k] = max(S[k], abs(unit_si(r)))
    xs = [n["position"] for n in model["nodes"]]
    d = [max(p[a] for p in xs) - min(p[a] for p in xs) for a in "xyz"]
    Lb = math.sqrt((d[0] * d[0] + d[1] * d[1]) + d[2] * d[2])
    tr = max(S["translation"], Lb * S["rotation"]); ro = max(S["rotation"], S["translation"] / Lb)
    fo = max(S["force"], S["moment"] / Lb); mo = max(S["moment"], Lb * S["force"])
    st = sel["section_terms"][0]; A = dec(st["area"]); Z = dec(st["section_modulus"])
    star = {"translation": tr, "rotation": ro, "force": fo, "moment": mo,
            "stress1": (fo / A) + (1.0 * (mo / Z)), "stress2r2": (fo / A) + (K["pipe_elastic_normal_stress_maximum_v2"] * (mo / Z))}
    bs = sel["body_scales"][0]
    scales_match = {k: bits(star[k]) == bs[k] for k in ["translation", "rotation", "force", "moment"]}
    R = dec(sel["floor_ratio"]); assert R == 2.0 ** -34
    cls, bound, scale = {}, {}, {}
    for r, k in zip(rows, kinds):
        if k in ("non_quantity", "input_derived", "not_covered"): cls[r["id"]] = k; continue
        s = star[k]; n = unit_si(r); scale[r["id"]] = bits(s)
        if s < 2.0 ** -988 or abs(n) < R * s:
            cls[r["id"]] = "absolute_verified"; bound[r["id"]] = bits(ru(F(s) * F(2) ** -64) if s else 0.0)
        else: cls[r["id"]] = "relative_verified"
    counts = {}
    for v in cls.values(): counts[v] = counts.get(v, 0) + 1
    receipt_abs = {a["result_id"]: a["bound"] for a in sel["absolute_verified"]}
    rr = reader_rows[mode]
    # The oracle's own classification formula (lines 130-155), applied to the successor rows,
    # with (a) I50's represented facts A, Z and (b) the successor's receipt section terms.
    facts = {"A": 0.005969026041820609, "Z": 0.00027009842839238263}
    def oracle_kind(r):
        k = r['kind']
        if k == 'displacement_magnitude' or k.startswith('global_nodal_displacement'): return 0
        if k.startswith('global_nodal_rotation'): return 1
        if k.startswith('element_local_') and 'force' in k: return 2
        if k.startswith('element_local_') and 'moment' in k: return 3
        if k == 'support_reaction_component_v2': return 2 if r['metadata']['component'][0] == 'F' else 3
        if k == 'support_reaction_force_magnitude_v2': return 2
        if k == 'support_reaction_moment_magnitude_v2': return 3
    def norm_value(r):
        y = r['value']; return y / 1000.0 if r['unit'] == 'mm' else y * 1e6 if r['unit'] == 'MPa' else y
    primary = [0.0] * 4
    for r in rows:
        k = oracle_kind(r)
        if k is not None and not (r['entity_ref'] == 'N0' and r['kind'].startswith('global_nodal_displacement_')): primary[k] = max(primary[k], abs(norm_value(r)))
    trp, rop, fop, mop = primary; oscales = [max(trp, 3.0 * rop), max(rop, trp / 3.0), max(fop, mop / 3.0), max(mop, 3.0 * fop)]
    def oracle_class(r, AA, ZZ):
        n = norm_value(r); k = oracle_kind(r)
        scale = oscales[k] if k is not None else oscales[2] / AA + (dec('4006a09e667f3bcd') if r['kind'] == 'pipe_elastic_normal_stress_maximum_v2' else 1.0) * (oscales[3] / ZZ)
        inp = r['entity_ref'] == 'N0' and r['kind'].startswith('global_nodal_displacement_')
        c = 'input_derived' if inp else 'absolute_verified' if scale < 2.0 ** -988 or abs(n) < 2.0 ** -34 * scale else 'relative_verified'
        b = None
        if c == 'absolute_verified':
            b = ru(F(scale) * F(2) ** -64) if scale else 0.0
            if 0 < scale < 2.0 ** -988: b = ru(F(b) + F(ru(F(2) ** -53 * abs(F(n)))) + F(2) ** -1074)
        return c, (bits(b) if b is not None else None), bits(scale)
    diffs = {"independent_vs_receipt_abs_ids": sorted(set(receipt_abs) ^ {i for i, c in cls.items() if c == "absolute_verified"}),
             "independent_vs_receipt_bound_bits": [i for i in receipt_abs if bound.get(i) != receipt_abs[i]],
             "independent_vs_reader_class": [i for i in cls if cls[i] != rr[i]["class"]],
             "independent_vs_reader_bound": [i for i in bound if bound[i] != rr[i]["bound_bits"]],
             "independent_vs_reader_scale": [i for i in scale if scale[i] != rr[i]["scale_bits"]],
             "receipt_not_covered": sel["not_covered"]}
    orc = {"facts": {}, "section_terms": {}}
    for label, (AA, ZZ) in [("facts", (facts["A"], facts["Z"])), ("section_terms", (A, Z))]:
        for r in rows:
            if r["kind"] in NONQ: continue
            c, b, s = oracle_class(r, AA, ZZ)
            if c != cls[r["id"]] or (b is not None and b != bound.get(r["id"])) or (c != "input_derived" and s != scale[r["id"]]):
                orc[label][r["id"]] = {"oracle": [c, b, s], "independent": [cls[r["id"]], bound.get(r["id"]), scale.get(r["id"])]}
    out[mode] = {"file_sha256": hashlib.sha256(raw).hexdigest(), "rows": len(rows), "counts": dict(sorted(counts.items())), "L_b": Lb,
                 "S_kind": {k: bits(v) for k, v in S.items()}, "S_star": {k: bits(v) for k, v in star.items()}, "receipt_body_scales_match": scales_match,
                 "input_derived_dofs": sorted(dofs), "rigid_restraints_from_invocation": sorted(rigid), "diffs": diffs,
                 "oracle_formula_disagreements": {k: v for k, v in orc.items()}}
print(json.dumps(out, indent=1))
