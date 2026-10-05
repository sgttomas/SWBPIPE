"""RV86 check 3: an independent closed-form reference (written without the oracle's code), row by row.
Exact rationals; pi by Machin's formula with alternating-series bounds; square roots by integer isqrt.
Compares (i) with the oracle's source-annulus and represented intervals recorded in U5's report and
(ii) the published values against their published class, and quantifies the seven SharperExact misses
against three section readouts: exact annulus, I50's ordinary represented J, and the successor's own
receipt torsional stiffness."""
import json, math, struct, sys
from fractions import Fraction as F
from pathlib import Path
dec = lambda h: struct.unpack('>d', bytes.fromhex(h))[0]
def atan_inv(q, n=60):  # atan(1/q) bracket, alternating series
    s = F(0); p = F(1, q)
    for k in range(n): s += (-1 if k % 2 else 1) * p / (2 * k + 1); p /= q * q
    nxt = p / (2 * n + 1)  # n even: s is below, next term positive
    return (s, s + nxt) if n % 2 == 0 else (s - nxt, s)
a5, a239 = atan_inv(5), atan_inv(239)
PI = (16 * a5[0] - 4 * a239[1], 16 * a5[1] - 4 * a239[0])
assert F(314159265358979323846, 10**20) < PI[0] <= PI[1] < F(314159265358979323847, 10**20) and PI[1] - PI[0] < F(1, 10**70)
def isqrt_bracket(x, bitsn=600):
    k = math.isqrt((x.numerator << (2 * bitsn)) // x.denominator); lo = F(k, 1 << bitsn)
    return (lo, lo) if lo * lo == x else (lo, F(k + 1, 1 << bitsn))
def imul(a, b): v = [x * y for x in a for y in b]; return (min(v), max(v))
def idiv(a, b): assert b[0] > 0; v = [x / y for x in a for y in b]; return (min(v), max(v))
pt = lambda x: (F(x), F(x))
succ_files, u5_report = sys.argv[1:-1], sys.argv[-1]
rep = json.loads(Path(u5_report).read_text())
orc = {m["summary"]["mode"]: {e["id"]: e for e in m["rows"]} for m in rep["modes"]}
results = {}
for path in succ_files:
    doc = json.loads(Path(path).read_text()); mode = doc["invocation"]["solver_mode"]
    m = doc["invocation"]["request"]["model"]; rows = doc["source"]["results"]
    sel = doc["source"]["retained_precision"]["body"]["cases"][0]["selection"]
    st = sel["section_terms"][0]
    # Inputs, as binary64 values.
    Mv = [F(l["magnitude"]["value"]) for l in m["load_cases"][0]["primitive_loads"]]
    assert [l["direction"] for l in m["load_cases"][0]["primitive_loads"]] == ["RX", "RY", "RZ"] and all(l["target"]["node"] == "N1" for l in m["load_cases"][0]["primitive_loads"])
    k = [F(s["stiffness"]["value"]["value"]) for s in m["supports"][1:]]
    p0 = [F(m["nodes"][0]["position"][a]) for a in "xyz"]; p1 = [F(m["nodes"][1]["position"][a]) for a in "xyz"]
    r = [b - a for a, b in zip(p0, p1)]; L2 = sum(x * x for x in r); L = F(math.isqrt(L2.numerator)) if L2 == 9 else None
    assert L == 3
    e = [x / L for x in r]
    G = F(m["materials"][0]["shear_modulus"]["value"]); D = F(m["pipe_segments"][0]["section"]["outside_diameter"]["value"]); t = F(m["pipe_segments"][0]["section"]["wall_thickness"]["value"])
    T = sum(a * b for a, b in zip(Mv, e))  # torque about the member axis
    assert [T * x for x in e] == Mv, "the applied couple is parallel to the member: pure torsion"
    th = [Mv[i] / k[i] for i in range(3)]  # spring rotations at N0
    u1 = [th[1] * r[2] - th[2] * r[1], th[2] * r[0] - th[0] * r[2], th[0] * r[1] - th[1] * r[0]]  # rigid rotation of N1 about N0
    c = D / 2; ri = c - t
    J_exact = imul(PI, pt((c**4 - ri**4) / 2))
    ktb = F(dec(st["torsional_stiffness"])); ulp = F(math.ulp(dec(st["torsional_stiffness"])))
    J_receipt = ((ktb - ulp / 2) * L / G, (ktb + ulp / 2) * L / G)   # the J the receipt's k_t = fl(G*J/L) admits
    J_i50 = pt(F(5.4019685678476534e-05))  # I50's represented (ordinary-route) J, from its captured facts
    readouts = {"exact_annulus": J_exact, "receipt_kt": J_receipt, "i50_represented": J_i50}
    def truth(row, J):
        kd, ent = row["kind"], row["entity_ref"]; meta = row.get("metadata") or {}
        twist = idiv(pt(T * L), imul(pt(G), J))
        if kd.startswith("global_nodal_displacement_"):
            return pt(0) if ent == "N0" else pt(u1["xyz".index(kd[-1])])
        if kd.startswith("global_nodal_rotation_"):
            i = "xyz".index(kd[-1])
            if ent == "N0": return pt(th[i])
            tw = imul(twist, pt(e[i])); return (th[i] + tw[0], th[i] + tw[1])
        if kd == "displacement_magnitude":
            return pt(0) if ent == "N0" else isqrt_bracket(sum(x * x for x in u1))
        if kd.startswith("support_reaction_"):
            v = [F(0)] * 6
            if ent.startswith("spring:N0:"): i = int(ent[-1]); v[3 + i] = -k[i] * th[i]
            if kd == "support_reaction_component_v2": return pt(v[["Fx", "Fy", "Fz", "Mx", "My", "Mz"].index(meta["component"])])
            w = v[:3] if kd == "support_reaction_force_magnitude_v2" else v[3:]
            return isqrt_bracket(sum(x * x for x in w))
        if kd == "element_local_torsional_moment": return pt(-T if meta["location"] == "end_i" else T)
        if kd == "element_local_torsional_shear_stress": return idiv(pt(T * c), J)
        if kd in ("element_local_axial_force", "element_local_shear_force_y", "element_local_shear_force_z", "element_local_bending_moment_y", "element_local_bending_moment_z",
                  "element_local_axial_normal_stress", "element_local_bending_normal_stress_y", "element_local_bending_normal_stress_z", "pipe_elastic_normal_stress_maximum_v2"):
            return pt(0)
        raise KeyError(kd)
    def si(row):
        y = row["value"]; return y / 1000.0 if row["unit"] == "mm" else y * 1e6 if row["unit"] == "MPa" else y
    def dist(n, a): n = F(n); return (a[0] - n if n < a[0] else n - a[1] if n > a[1] else F(0), max(n - a[0], a[1] - n))
    out = {"rows": {}, "tally": {}}
    for row in rows:
        oe = orc[mode][row["id"]]; cls = oe["class"]
        if cls == "non_quantity": continue
        n = si(row); entry = {"class": cls}
        tv = {name: truth(row, J) for name, J in readouts.items()}
        # (i) agreement with the oracle's recorded intervals: they must intersect ours, and ours must be within the oracle's width.
        for oname, mine in [("source_annulus", "exact_annulus"), ("represented", "i50_represented")]:
            o = [F(x) for x in oe["readouts"][oname]["truth"]]; a = tv[mine]
            entry[f"oracle_{oname}_overlaps"] = not (a[1] < o[0] or o[1] < a[0])
        # (ii) the published class claim against our exact-annulus truth
        lo, hi = dist(n, tv["exact_annulus"])
        if cls == "input_derived": allow = F(0)
        elif cls == "absolute_verified": allow = F(dec(oe["bound_bits"]))
        else: allow = abs(F(n)) / 10**9
        entry["class_claim"] = "pass" if hi <= allow else "fail" if lo > allow else "unproved"
        if cls == "relative_verified":
            sc = F(dec(oe["scale_bits"]))
            sharp = F(2)**-64 * max(abs(F(n)), sc) * (1 + F(2)**-21) + F(2)**-53 * abs(F(n)) + F(2)**-1074
            entry["rel_error_upper"] = float(hi / abs(F(n)))
            for name, a in tv.items():
                l2, h2 = dist(n, a)
                entry[f"sharper_{name}"] = ("pass" if h2 <= sharp else "fail" if l2 > sharp else "unproved", float(h2 / sharp))
            # the represented-versus-annulus separation, in sharper allowances
            sep = max(tv["exact_annulus"][0] - tv["i50_represented"][1], tv["i50_represented"][0] - tv["exact_annulus"][1], F(0))
            entry["annulus_vs_i50_separation_over_sharper"] = float(sep / sharp)
        out["rows"][row["id"]] = entry
        key = f'{cls}:{entry["class_claim"]}'; out["tally"][key] = out["tally"].get(key, 0) + 1
        for nm in ["exact_annulus", "receipt_kt", "i50_represented"]:
            if f"sharper_{nm}" in entry:
                kk = f"info_sharper_{nm}:{entry[f'sharper_{nm}'][0]}"; out["tally"][kk] = out["tally"].get(kk, 0) + 1
    out["oracle_overlap_all"] = all(v["oracle_source_annulus_overlaps"] and v["oracle_represented_overlaps"] for v in out["rows"].values())
    out["sharper_misses_vs_i50_represented"] = {i: v["sharper_i50_represented"][1] for i, v in out["rows"].items() if "sharper_i50_represented" in v and v["sharper_i50_represented"][0] != "pass"}
    out["J"] = {kname: [float(a[0]), float(a[1])] for kname, a in readouts.items()}
    out["J_ulps_i50_minus_exact"] = float((F(5.4019685678476534e-05) - (J_exact[0] + J_exact[1]) / 2) / F(math.ulp(5.4019685678476534e-05)))
    out["J_dependent_rows_relative_weight"] = {}
    for row in rows:
        if row["kind"].startswith("global_nodal_rotation_") and row["entity_ref"] == "N1" or row["kind"] == "element_local_torsional_shear_stress":
            i = "xyz".index(row["kind"][-1]) if row["kind"].startswith("global") else None
            if i is None: w = 1.0
            else:
                tw = imul(idiv(pt(T * L), imul(pt(G), J_exact)), pt(e[i])); w = float(tw[0] / (th[i] + tw[0]))
            out["J_dependent_rows_relative_weight"][row["id"]] = w
    results[mode] = out
print(json.dumps({md: {k: v for k, v in o.items() if k != "rows"} for md, o in results.items()}, indent=1))
Path(sys.argv[0]).with_suffix(".rows.json").write_text(json.dumps(results, indent=1) + "\n")
