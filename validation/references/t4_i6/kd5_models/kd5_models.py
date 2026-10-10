#!/usr/bin/env python3
"""K-D5 kernel-level test models and their exact references, T4-U1 revision.

Writes `core/solver/nonlinear_integration/src/structural_adapter/kd5_models.rs`. Standard
library only.

Usage: python3 -I -B kd5_models.py <v1_dir> <t4_i6_dir> <out.rs> <out.json>

<v1_dir> holds T3's cited modules, imported unchanged: V1's `probe_d5_check.py` (T3
REVIEW/_run_records/d5_check/probe_d5_check.py.txt, sha256 d13cf7c8...) and D1's `curved_ef.py`
(T3 DESIGN_NUMERICS/_run_records/curved_ef.py, sha256 1c862cea...). <t4_i6_dir> is
`validation/references/t4_i6` (T4-I6's frozen references, round 00 and round_01/), read only
to compare the re-derived u_int with the frozen values.

What changed from T3's generator (T3 IMPLEMENTATION/KD5/_run_records/repair/models/
kd5_models.py.txt, sha256 f445b579...):
  * A realized bend is described by (R, y_reference, k), the inputs of T4-U1's objective
    element, not by the product's binary64 arc centre. Each bend's (R, y) is an explicit
    recorded input below: T4-I6 round 00's `t3_models[*].regenerated_bend_inputs` (O6), the
    bow vector of the old arc (the member y_reference where that lies on the old arc's bow
    side, otherwise the binary64 (midpoint - centre)/|midpoint - centre|).
  * The intended curved element is still D1's objective Decimal re-formation (`curved_int`),
    now given the exact centre of the (R, y) arc in Decimal instead of a binary64 centre, at 120
    digits with an arctangent series run to 1e-125 (D1's own runs to 1e-70, enough at 60
    digits but not for the 1e-8 rad K1 bends). Frames are unchanged (V1's exact frame).
  * CSKEW_30_RADIUS_MISMATCH is regenerated as the ordinary k_X = 30 elbow CSKEW_30 (a centre
    mismatch cannot be expressed in (R, y)). CPLANAR_60 and CSKEW_30_N122 keep their names and
    become ordinary elbows on their old bow vectors.
  * New models: CSKEW_9, CSKEW_10 (D1's C-skew series); T4-I6 round 01's seven curved
    true-positive candidates (F122 realized as a bend); K1 and K1F (1e-8 rad); the acceptance
    range ACC samples at X = 0 and X = 7.3e6; round 00's K1 cantilevers at 1e-4 and 1e-8 rad,
    X = 0 and X = 5e6.
  * The binary64 emulation of the pre-T4-U1 curved element is not run (it models an element
    that no longer exists); frame models keep V1's emulation.
Every re-derived u_int is compared with the T4-I6 frozen value (an independent element:
T4-I6's curved_ref.py at 110 digits); the comparison is printed and stored in <out.json>.
"""
import json
import sys
from decimal import Decimal as D, getcontext
from fractions import Fraction as Fr

v1, t4_i6, out_rs, out_json = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4]
sys.path.insert(0, v1)
sys.argv = [sys.argv[0], v1]
import probe_d5_check as p  # noqa: E402
import curved_ef as ce  # noqa: E402

getcontext().prec = 120
DOFS = "UX UY UZ RX RY RZ".split()


def datan(t):
    """D1's arctangent (argument halving, then the series), run to 1e-125."""
    k = 0
    while t > D("0.05"):
        t = t / (1 + (1 + t * t).sqrt())
        k += 1
    s, term, n, t2 = D(0), t, 1, t * t
    while True:
        add = term / n
        if abs(add) < D(10) ** -125:
            break
        s += add
        term = -term * t2
        n += 2
    return s * (2 ** k)


ce.datan = datan
dec = ce.dec


def exact_centre(xi, xj, radius, yref):
    """The centre of the arc of radius R through x_i, x_j bowing towards y (Decimal)."""
    xi, xj, y = [dec(v) for v in xi], [dec(v) for v in xj], [dec(v) for v in yref]
    R = dec(radius)
    d = [xj[k] - xi[k] for k in range(3)]
    l2 = sum(v * v for v in d)
    L = l2.sqrt()
    t = [v / L for v in d]
    a = sum(y[k] * t[k] for k in range(3))
    pv = [y[k] - a * t[k] for k in range(3)]
    pn = sum(v * v for v in pv).sqrt()
    sag = (R * R - l2 / 4).sqrt()
    return [(xi[k] + xj[k]) / 2 - sag * pv[k] / pn for k in range(3)]


def section_rows(sec):
    return dict(E=sec["E"], G=sec["G"], A=sec["A"], Iy=sec["Iy"], J=sec["J"])


def exact_model(m):
    """Intended free displacements of a model in index form (Decimal, 120 digits)."""
    n = 6 * len(m["nodes"])
    K = [[D(0)] * n for _ in range(n)]
    sec = m["section"]
    for mm in m["members"]:
        xi, xj = m["nodes"][mm["i"]], m["nodes"][mm["j"]]
        if mm["bend"] is None:
            Ke = ce.frame_int(xi, xj, mm["yref"], sec)
        else:
            b = mm["bend"]
            c = exact_centre(xi, xj, b["R"], b["y"])
            Ke = ce.curved_int(xi, xj, c, sec, b["k"], b["k"])
        base = [6 * mm["i"] + k for k in range(6)] + [6 * mm["j"] + k for k in range(6)]
        for r in range(12):
            for cc in range(12):
                K[base[r]][base[cc]] += Ke[r][cc]
    for dof, k in m["springs"]:
        K[dof][dof] += dec(k)
    f = [D(0)] * n
    for dof, v in m["loads"]:
        f[dof] += dec(v)
    free = [i for i in range(n) if i not in set(m["rigid"])]
    u = ce.dsolve([[K[i][j] for j in free] for i in free], [f[i] for i in free])
    return free, u


def emulate_frame(m, u):
    out = {}
    for mode in ("dense", "sparse"):
        sol = p.solve_product(m, mode)
        if sol is None or sol.get("outcome") == "unresolved":
            out[mode] = {"outcome": "failed"}
            continue
        uu = sol["u"]
        S = p.s_star(uu, m["L_b"])
        Sint = p.s_star([float(x) for x in u], m["L_b"])
        act = max(abs(Fr(uu[d]) - u[d]) / max(abs(u[d]), Fr(Sint[p.kinds(d)])) for d in m["free"]) * 10**9
        uF = [Fr(x) for x in uu]
        v = [Fr(m["f"][i]) - sum(m["K_int"][i][j] * uF[j] for j in range(12)) for i in m["free"]]
        w = p.apply_inverse(sol, v)
        trig = max(2 * abs(w[r]) / (1e-9 * max(abs(uu[i]), S[p.kinds(i)])) for r, i in enumerate(m["free"]))
        out[mode] = {"outcome": sol["outcome"], "cond": 1 / sol["rcond"], "actual_ratio": float(act),
                     "trigger_ratio": trig}
    return out


def frame_case(name, xj, yref, springs, moments):
    """Unchanged from T3's generator (frames are not touched by T4-U1)."""
    sec = p.section()
    m = p.build(xj, yref, springs, moments, sec)
    u = p.exact_solve(m["K_int"], m["f"], m["free"])
    data = dict(name=name, nodes=[[0.0, 0.0, 0.0], list(xj)],
                members=[dict(i=0, j=1, yref=list(yref), bend=None)],
                rigid=[0, 1, 2], springs=[[d, k] for d, k in springs], loads=[[d, v] for d, v in moments if v != 0.0],
                u_int=[[i, float(u[i])] for i in m["free"]], section=sec)
    return data, emulate_frame(m, u), None


def index_model(key, mdl, bends):
    """D1's named model in index form; `bends` lists each realized bend's recorded (R, y, k)."""
    names = list(mdl["nodes"])
    idx = {nm: i for i, nm in enumerate(names)}
    bends = iter(bends)
    members = []
    for el in mdl["members"]:
        bend = None
        if el.get("bend"):
            R, y, k = next(bends)
            assert R == el["bend"]["R"] and k == el["bend"].get("k", 1.0), key
            bend = dict(R=R, y=list(y), k=k)
        members.append(dict(i=idx[el["i"]], j=idx[el["j"]], yref=list(el["yref"]), bend=bend))
    assert next(bends, None) is None, key
    rigid = sorted(6 * idx[nm] + DOFS.index(d) for nm, ds in mdl.get("rigid", {}).items() for d in ds)
    springs = [[6 * idx[nm] + DOFS.index(d), k] for nm, d, k in mdl.get("springs", [])]
    loads = [[6 * idx[nm] + DOFS.index(d), v] for nm, d, v in mdl.get("loads", [])]
    return dict(name=key, nodes=[list(mdl["nodes"][nm]) for nm in names], members=members, rigid=rigid,
                springs=springs, loads=loads, section=p.section(*mdl.get("section", (0.2, 0.18, 200e9, 80e9))))


def curved_case(m):
    free, u = exact_model(m)
    m["u_int"] = [[i, float(u[r])] for r, i in enumerate(free)]
    return m, None, {i: u[r] for r, i in enumerate(free)}


def d1_model(name):
    return next(x for x in ce.models() if x["name"] == name)


def cskew(key, kx, bends):
    """D1's C-skew elbow cantilever at k_X, its tip moment taken as the binary64 product
    k_X*1e-6, the literal T4-I6 froze u_int with (equal to D1's 0.01*k_X/1e4 except at k_X = 10:
    9.999999999999999e-06, not 1e-05)."""
    m = index_model(key, d1_model(f"C-skew elbow cantilever, soft root spring k_X = {kx:g}"), bends)
    assert m["loads"] == [[9, 0.01 * kx / 1e4]], key
    m["loads"] = [[9, kx * 1e-6]]
    return m


def cantilever_elbow(name, xj, radius, springs, xi=(0.0, 0.0, 0.0), yref=(0.0, 1.0, 0.0),
                     section=(0.2, 0.18, 200e9, 80e9), loads=None):
    """RV5-B1's cantilever: node 0 translations rigid, rotational springs on node 0, one realized
    bend to node 1, a tip moment (1, 1, 1) N*m (or the given tip loads)."""
    return {"name": name, "nodes": {"N0": list(xi), "N1": list(xj)},
            "members": [dict(i="N0", j="N1", yref=list(yref), bend={"R": radius, "k": 1.0})],
            "rigid": {"N0": ["UX", "UY", "UZ"]},
            "springs": [("N0", d, k) for d, k in zip(("RX", "RY", "RZ"), springs)],
            "loads": loads or [("N1", d, 1.0) for d in ("RX", "RY", "RZ")],
            "section": section}


def f122_bend(name, radius, y, kx):
    """F122 (nodes (0,0,0), (1,2,2); T3's F122 springs and loads) with its member realized as one
    bend of radius R in the plane of y (T4-I6 round 01, item 1). The member y_reference is y."""
    sec = p.section()
    return dict(name=name, nodes=[[0.0, 0.0, 0.0], [1.0, 2.0, 2.0]],
                members=[dict(i=0, j=1, yref=list(y), bend=dict(R=radius, y=list(y), k=1.0))],
                rigid=[0, 1, 2], springs=[[3, kx], [4, 1e6], [5, 1e6]],
                loads=[[9, 0.0048], [10, 0.0096], [11, 0.0096]], section=sec)


def tip_cantilever(name, xi, xj, radius, y, loads):
    """K1 / K1F / ACC: node 0 translations rigid, rotational springs 1e6 on node 0, one bend."""
    return dict(name=name, nodes=[list(xi), list(xj)],
                members=[dict(i=0, j=1, yref=list(y), bend=dict(R=radius, y=list(y), k=1.0))],
                rigid=[0, 1, 2], springs=[[3, 1e6], [4, 1e6], [5, 1e6]], loads=[list(l) for l in loads],
                section=p.section())


def m11():
    sec = p.section()
    pts = [[2.0 * i, 1.0 * i, 0.0] for i in range(31)]
    members = [dict(i=i, j=i + 1, yref=[0.0, 0.0, 1.0], bend=None) for i in range(30)]
    rigid = list(range(6)) + [6 * 30 + 1]
    springs = [[6 * i + 2, 5e4] for i in range(3, 31, 3)]
    loads = [[6 * i + 2, -1000.0] for i in range(1, 31)] + [[6 * 30 + 1, 100.0]]
    return dict(name="M11", nodes=pts, members=members, rigid=rigid, springs=springs, loads=loads, u_int=[],
                section=sec), {"note": "V1 probe_d5_realistic: Passed both modes, first-order error <= 1.5e-4 of the criterion"}, None


def rs(x):
    s = repr(float(x))
    return s if ("." in s or "e" in s) else s + ".0"


def rust_name(key):
    return key.replace("-", "_").replace(".", "_").upper()


def emit(models, acc_names, k1_names):
    lines = ["//! Generated by `validation/references/t4_i6/kd5_models/kd5_models.py` (T4-U1",
             "//! revision of T3's K-D5 run record `kd5_models.py`); do not edit by hand. Invented",
             "//! inputs only; a realized bend is (R, y_reference of its plane, k). u_int is the exact",
             "//! intended solution of each model, rounded once to binary64.",
             "#![cfg_attr(rustfmt, rustfmt::skip)]",
             "#![allow(dead_code, clippy::approx_constant, clippy::excessive_precision)]",
             "use super::{MemberData, ModelData, SectionData};", ""]
    for d in models:
        sec = d["section"]
        lines.append(f"pub(super) const {rust_name(d['name'])}: ModelData = ModelData {{")
        lines.append(f"    name: \"{d['name']}\",")
        lines.append(f"    section: SectionData {{ e: {rs(sec['E'])}, g: {rs(sec['G'])}, a: {rs(sec['A'])}, i: {rs(sec['Iy'])}, j: {rs(sec['J'])} }},")
        lines.append("    nodes: &[" + ", ".join(f"[{rs(a)}, {rs(b)}, {rs(c)}]" for a, b, c in d["nodes"]) + "],")
        mem = []
        for mm in d["members"]:
            b = mm["bend"]
            bend = "None" if b is None else f"Some(({rs(b['R'])}, [{', '.join(rs(v) for v in b['y'])}], {rs(b['k'])}))"
            mem.append(f"MemberData {{ i: {mm['i']}, j: {mm['j']}, y_reference: [{', '.join(rs(v) for v in mm['yref'])}], bend: {bend} }}")
        lines.append("    members: &[" + ", ".join(mem) + "],")
        lines.append("    rigid: &[" + ", ".join(str(x) for x in d["rigid"]) + "],")
        lines.append("    springs: &[" + ", ".join(f"({a}, {rs(b)})" for a, b in d["springs"]) + "],")
        lines.append("    loads: &[" + ", ".join(f"({a}, {rs(b)})" for a, b in d["loads"]) + "],")
        lines.append("    u_int: &[" + ", ".join(f"({a}, {rs(b)})" for a, b in d["u_int"]) + "],")
        lines.append("};")
        lines.append("")
    lines.append("/// T4-I6 round 01's acceptance-range samples (ACC model), each at X = 0 and X = 7.3e6.")
    lines.append("pub(super) const ACC_SAMPLES: &[&ModelData] = &[" + ", ".join(f"&{rust_name(n)}" for n in acc_names) + "];")
    lines.append("/// T4-I6 round 00's K1 cantilevers at 1e-4 and 1e-8 rad, X = 0 and X = 5e6.")
    lines.append("pub(super) const K1_RANGE_SAMPLES: &[&ModelData] = &[" + ", ".join(f"&{rust_name(n)}" for n in k1_names) + "];")
    lines.append("")
    return "\n".join(lines)


# ---------------------------------------------------------------- frozen T4-I6 values
def frozen():
    r00 = json.load(open(f"{t4_i6}/u1_reference_cases.json"))
    r01 = json.load(open(f"{t4_i6}/round_01/u1_reference_cases_r01.json"))
    t3 = {x["model"]: x for x in r00["t3_models"]}
    m31b = {x["model"]: x for x in r00["m31b_kill_and_mutant"]}
    return r00, r01, t3, m31b


def compare(name, u_exact, ref, nodes):
    """max |u - u_ref| / (1e-9 max(|u_ref|, S*)) (the tests' actual-error measure) and the
    number of free rows whose binary64 rounding is bit-identical."""
    ref = {int(k): D(v) for k, v in ref.items()}
    assert set(ref) == set(u_exact), name
    st = max((abs(v) for k, v in ref.items() if k % 6 < 3), default=D(0))
    sr = max((abs(v) for k, v in ref.items() if k % 6 >= 3), default=D(0))
    ext = [max(q[k] for q in nodes) - min(q[k] for q in nodes) for k in range(3)]
    lb = D(sum(e * e for e in ext)).sqrt()
    tr, ro = max(st, lb * sr), max(sr, st / lb)
    worst = max(abs(u_exact[k] - v) / (D("1e-9") * max(abs(v), tr if k % 6 < 3 else ro)) for k, v in ref.items())
    same = sum(1 for k, v in ref.items() if float(u_exact[k]) == float(v))
    return {"model": name, "max_ratio_to_criterion": f"{worst:.3E}", "binary64_equal": f"{same}/{len(ref)}"}


def main():
    r00, r01, t3, m31b = frozen()
    rbi = {k: [(b["R"], tuple(b["y_reference"]), b["k"]) for b in v.get("regenerated_bend_inputs", [])] for k, v in t3.items()}
    m122 = [(9, 0.0048), (10, 0.0096), (11, 0.0096)]
    m345 = [(9, 0.005184), (10, 0.006912), (11, 0.0)]
    # Recorded bend inputs (R, y_reference, k), T4-I6 round 00 t3_models[*].regenerated_bend_inputs.
    BENDS = {
        "E1": [(0.3, (1.0, -1.0, 0.0), 1.0)],
        "E6": [(0.3, (1.0, -1.0, 0.0), 1.0), (0.3, (-1.0, 1.0, 0.0), 1.0), (0.3, (1.0, 1.0, 0.0), 1.0),
               (0.3, (-1.0, -1.0, 0.0), 1.0)],
        "CSKEW": [(0.3, (0.3333333333333333, -1.3333333333333333, -0.3333333333333333), 1.0)],
        "CPLANAR_60": [(0.3, (0.5000000009999997, -0.8660254032070885, 0.0), 1.0)],
        "CSKEW_30_N122": [(0.3, (0.24401693767765975, -0.7440169359456088, 0.622008467106779), 1.0)],
        "PP_UTM_2": [(0.3, (0.0, 1.0, 0.0), 1.0)],
    }
    for key, src in (("E1", "E1"), ("E6", "E6"), ("CSKEW", "CSKEW_8_5"), ("CSKEW", "CSKEW_30_RADIUS_MISMATCH"),
                     ("CSKEW", "CSKEW_9"), ("CSKEW", "CSKEW_10"), ("CPLANAR_60", "CPLANAR_60"),
                     ("CSKEW_30_N122", "CSKEW_30_N122"), ("PP_UTM_2", "PP_UTM_2")):
        assert BENDS[key] == rbi[src], (key, src, rbi[src])

    def bow_elbow(name, xj, y):
        mdl = cantilever_elbow(name, xj, 0.3, (1e6, 1e6, 1e6))
        return index_model(name, mdl, BENDS[name])

    cases = [
        frame_case("F122", (1.0, 2.0, 2.0), (1.0, 0.0, 0.0), [(3, 144.0), (4, 1e6), (5, 1e6)], m122),
        frame_case("F345", (3.0, 4.0, 0.0), (0.0, 0.0, 1.0), [(3, 86.4), (4, 1e6), (5, 1e6)], m345),
        frame_case("PROBE_C", (5.115, 0.0, 0.0), (0.0, 0.0, 1.0), [(3, 0.051614650352348725), (4, 1e6), (5, 1e6)],
                   [(9, 0.0144)]),
        frame_case("PROBE_D", (5.015, 0.0, 0.0), (0.0, 0.0, 1.0), [(3, 0.05222604738082737), (4, 1e6), (5, 1e6)],
                   [(9, 0.0144)]),
        frame_case("BENDING_SOFT", (3.0, 0.0, 0.0), (0.0, 0.0, 1.0), [(3, 1e6), (4, 2.2), (5, 1e6)], [(10, 0.01)]),
        curved_case(index_model("E1", d1_model("E1 L-shape, two anchors, one elbow"), BENDS["E1"])),
        curved_case(index_model("E6", d1_model("E6 expansion U-loop, four elbows, anchors at both ends, hangers"),
                                BENDS["E6"])),
        curved_case(cskew("CSKEW_8_5", 8.5, BENDS["CSKEW"])),
        curved_case(cskew("CSKEW_9", 9.0, BENDS["CSKEW"])),
        curved_case(cskew("CSKEW_10", 10.0, BENDS["CSKEW"])),
        curved_case(cskew("CSKEW_30", 30.0, BENDS["CSKEW"])),
        m11(),
        curved_case(bow_elbow("CPLANAR_60", (0.25980762113533157, 0.14999999999999997, 0.0), None)),
        curved_case(bow_elbow("CSKEW_30_N122", (0.14142135623730948, -0.00693503541210147, -0.06377564270655325), None)),
        curved_case(index_model("PP_UTM_2", cantilever_elbow(
            "PP_UTM_2", (500000.010469849, 350000.0001827519, 0.0), 0.3, (1e6, 1e6, 1e6),
            xi=(500000.0, 350000.0, 0.0)), BENDS["PP_UTM_2"])),
    ]
    reference_of = {"E1": t3["E1"]["u_int_new"], "E6": t3["E6"]["u_int_new"],
                    "CSKEW_8_5": t3["CSKEW_8_5"]["u_int_new"], "CSKEW_9": t3["CSKEW_9"]["u_int_new"],
                    "CSKEW_10": t3["CSKEW_10"]["u_int_new"],
                    "CSKEW_30": t3["CSKEW_30_RADIUS_MISMATCH"]["u_int_new"],
                    "CPLANAR_60": t3["CPLANAR_60"]["u_int_new"], "CSKEW_30_N122": t3["CSKEW_30_N122"]["u_int_new"],
                    "PP_UTM_2": t3["PP_UTM_2"]["u_int_new"]}
    # T4-I6 round 01, item 1: the curved true-positive candidates, in round 01's rank order.
    for c in sorted(r01["curved_true_positive_candidates"], key=lambda c: [
            "C122K120-R100-Y100", "C122-R10-Y100", "C122-R10-YM100", "C122-R100-Y011", "C122-R100-Y01M1",
            "C122-R100-Y100", "C122-R10-Y01M1"].index(c["id"])):
        mem = c["member"]
        m = f122_bend(c["id"], mem["bend_R"], mem["y_reference"], c["springs"][0][1])
        assert c["nodes"] == m["nodes"] and c["springs"] == m["springs"] and c["loads"] == m["loads"], c["id"]
        assert [c["section"][k] for k in ("E", "G", "A", "I", "J")] == [m["section"][k] for k in ("E", "G", "A", "Iy", "J")]
        cases.append(curved_case(m))
        reference_of[c["id"]] = c["u_int"]
    # Item 4: K1 and K1F (1e-8 rad, in-plane and skew), X = 0.
    for c in r01["k1"]:
        m = tip_cantilever(c["id"].replace("-1E-8", ""), c["nodes"][0], c["nodes"][1], c["R"], c["y_reference"],
                           c["loads"])
        cases.append(curved_case(m))
        reference_of[m["name"]] = m31b[c["id"] + "-X0"]["u_int"]
    for c in r01["k1f"]:
        m = tip_cantilever(c["id"].replace("-1E-8", ""), c["nodes"][0], c["nodes"][1], c["R"], c["y_reference"],
                           c["loads"])
        cases.append(curved_case(m))
        reference_of[m["name"]] = c["u_int"]
    # Item 8: the acceptance range's 1e-4 and 1e-8 rad points: round 00's K1 cantilevers (tip
    # moment) at X = 0 and X = 5e6 (d bit-identical), in-plane and skew.
    k1_names = []
    for c in r00["m31b_kill_and_mutant"]:
        if any(t in c["model"] for t in ("-1E-4-", "-1E-8-")):
            assert c["supports"].startswith("N0 UX,UY,UZ rigid; N0 RX,RY,RZ springs 1"), c["model"]
            m = tip_cantilever(c["model"], c["nodes"][0], c["nodes"][1], c["R"], c["y_reference"],
                               [[9, 1.0], [10, 1.0], [11, 1.0]])
            cases.append(curved_case(m))
            reference_of[m["name"]] = c["u_int"]
            k1_names.append(m["name"])
    # Item 8: acceptance-range samples (ACC model) at X = 0 and X = 7.3e6 (d bit-identical).
    acc_names = []
    for s in r01["acceptance_range_samples"]:
        inp = s["inputs"]
        assert s["d_bit_identical_at_7_3e6"]
        for xkey, tag in (("0.0", "X0"), ("7300000.0", "X7E6")):
            xi, xj = s["nodes_by_X"][xkey]
            name = f"ACC-{s['id'][1:].replace('-k1.0', '')}-{tag}"
            m = tip_cantilever(name, xi, xj, inp["R"], inp["y_reference"], s["acc_model"]["loads"])
            assert [inp[k] for k in ("E", "G", "A", "I", "J")] == [m["section"][k] for k in ("E", "G", "A", "Iy", "J")]
            cases.append(curved_case(m))
            reference_of[name] = s["acc_model"]["u_int"]
            acc_names.append(name)
    open(out_rs, "w").write(emit([c[0] for c in cases], acc_names, k1_names))
    comparisons = []
    for data, _, exact in cases:
        if exact is not None:
            r = compare(data["name"], exact, reference_of[data["name"]], data["nodes"])
            comparisons.append(r)
            print("compare", json.dumps(r))
    json.dump({"python": sys.version.split()[0], "decimal_digits": getcontext().prec,
               "frame_emulation": {c[0]["name"]: c[1] for c in cases if c[1] is not None},
               "u_int_vs_t4_i6_frozen": comparisons}, open(out_json, "w"), indent=1)
    for c in cases:
        if c[1] is not None:
            print(c[0]["name"], json.dumps(c[1]))


main()
