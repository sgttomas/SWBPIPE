#!/usr/bin/env python3
"""F1 modal references: mass model (DEL-04-09 AC-001) and class (a) discrete eigen references (AC-003).

Standard-library Python 3 only. This generator imports, reads and runs no
product code and reads no file other than the ones it writes. All model data
are invented (cases.py). Method, premises and cases: ../README.md.

Outputs (in the parent directory):
  mass_references.json   element and assembled masses on the PF and EX mass
                         bases, category totals, free-DOF mass per direction,
                         weight-basis identity values, component range premise
  eigen_references.json  eigenvalues, frequencies, exact Sturm counts, mode
                         shapes in both normalization cases, generalized masses,
                         participation factors and effective masses
  SHA256SUMS             hashes of README.md, the generators and both JSON files

Modes:
  python3 -I generators/f1_mass_discrete.py           check: regenerate in memory and
                                                      compare byte for byte with the
                                                      committed files, SHA256SUMS included
  python3 -I generators/f1_mass_discrete.py --write   write the outputs (intentional changes only)
  --jobs N                                            worker processes (default: CPU count);
                                                      the output does not depend on it
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import argparse  # noqa: E402
import hashlib  # noqa: E402
import json  # noqa: E402
import math  # noqa: E402
from concurrent.futures import ProcessPoolExecutor  # noqa: E402
from decimal import Decimal  # noqa: E402
from fractions import Fraction  # noqa: E402
from pathlib import Path  # noqa: E402

import cases  # noqa: E402
import crosscheck as cc  # noqa: E402
import eigen as eg  # noqa: E402
import elements as el  # noqa: E402
from model import DOFS, Model, form_mass, form_stiffness, reduce, section_basis, rigid_translation  # noqa: E402
from numeric import (PointField, QPiField, IntervalField, QPi, fmt, set_point_precision,  # noqa: E402
                     pi_decimal, exact_double)

HERE = Path(__file__).resolve().parent
OUT = HERE.parent
FIXED_FILES = ("README.md", "REFUTATION.md", "generators/cases.py", "generators/crosscheck.py", "generators/eigen.py",
               "generators/elements.py", "generators/f1_mass_discrete.py", "generators/model.py",
               "generators/numeric.py")
POINT_PREC = 80
IV_PREC = 110
DIG = 40          # mass values
EIG_DIG = 36      # eigenvalues, frequencies
SHAPE_DIG = 25    # mode shapes, participation
REPORT_CAP = 24   # modes reported: all finite modes up to this count, completed to whole clusters
NEAR_CLUSTERS = 4
TAU_T = Decimal("1e-6")
FLUSH = Decimal("1e-50")
AXES = ["X", "Y", "Z"]
QF = QPiField()


# ---------------------------------------------------------------------------- formatting
def num(x, d=DIG):
    x = Decimal(x)
    return {"decimal": fmt(x, d), "value": float(x)}


def qnum(q):
    """Exact a + b*pi -> exact string, decimal and nearest binary64."""
    q = QPi._c(q)
    v = q.to_decimal(DIG + 10)
    return {"exact": q.exact_str(), "decimal": fmt(v, DIG), "value": float(v)}


def frac_num(fr):
    return qnum(QPi(fr, 0))


def upper_nonzero(A, exact=None):
    out = []
    for i in range(len(A)):
        for j in range(i, len(A)):
            if A[i][j] != 0:
                row = [i, j, fmt(A[i][j], DIG)]
                if exact is not None:
                    row.insert(2, QPi._c(exact[i][j]).exact_str())
                out.append(row)
    return out


def vec(v, d=DIG):
    return [fmt(x, d) for x in v]


# ---------------------------------------------------------------------------- mass
def range_premise(W, a, r=None):
    """DEL-04-09 section 2.3 item 6, evaluated in binary64 (r: the centre of gravity, for the fourth part)."""
    tiny = sys.float_info.min
    m = W / a
    am = a * m

    def normal(x):
        return math.isfinite(x) and abs(x) >= tiny

    holds = W > 0 and a > 0 and normal(W) and normal(a) and normal(m) and normal(am)
    block_ok = None
    if holds and r is not None:
        ents = [m * x for x in r] + [m * x * y for x in r for y in r]
        block_ok = all(v == 0 or normal(v) for v in ents)
        holds = holds and block_ok
    out = {"W_N": W, "a_m_per_s2": a, "fl_W_over_a_kg": m, "fl_a_times_m_N": am,
           "spatial_block_entries_finite_and_zero_or_normal": block_ok, "premise_holds": holds}
    if holds:
        rel = (Fraction(a) * Fraction(m) - Fraction(W)) / Fraction(W)
        out["relative_a_times_fl_W_over_a_minus_W_exact"] = fmt(Decimal(rel.numerator) / Decimal(rel.denominator), 6) if rel else "0"
        out["relative_fl_a_m_minus_W"] = fmt(Decimal((Fraction(am) - Fraction(W)) / Fraction(W) * 10 ** 30) / Decimal(10) ** 30, 6) if am != W else "0"
        out["within_2u_plus_u2"] = abs(Fraction(am) - Fraction(W)) / Fraction(W) <= Fraction(2, 2 ** 53) + Fraction(1, 2 ** 106)
    return out


def range_witnesses():
    """Inputs outside the range premise, with the operand that fails (binary64)."""
    big = sys.float_info.max
    tiny = sys.float_info.min
    out = []

    def failing(W, a, r=(0.0, 0.0, 0.0)):
        m = W / a
        if not (math.isfinite(W) and W >= tiny):
            return "W"
        if not (math.isfinite(a) and a >= tiny):
            return "a"
        if not (math.isfinite(m) and m >= tiny):
            return "W/a"
        am = a * m
        if not (math.isfinite(am) and am >= tiny):
            return "a*(W/a)"
        for x in r:
            for v in [m * x] + [m * x * y for y in r]:
                if not (v == 0 or (math.isfinite(v) and abs(v) >= tiny)):
                    return "spatial block m*r_i*r_j"
        return None

    out.append({"W_N": 5e-324, "a_m_per_s2": 9.8125, "fails": failing(5e-324, 9.8125), "note": "W subnormal"})
    out.append({"W_N": 1e-300, "a_m_per_s2": 1e10, "fails": failing(1e-300, 1e10), "note": "W/a subnormal"})
    out.append({"W_N": big, "a_m_per_s2": 0.5, "fails": failing(big, 0.5), "note": "W/a overflows"})
    r = (1e10, 0.0, 0.0)
    out.append({"W_N": 1.0, "a_m_per_s2": 1e-300, "center_of_gravity_m": list(r), "fails": failing(1.0, 1e-300, r),
                "note": "scalar parts hold; m*r_x^2 overflows (Design section 2.3 item 6, fourth part)"})
    # W at the top of the range with W/a normal but a*fl(W/a) rounding above the largest double
    found = None
    for k in range(1, 4000):
        a = 1 + k / 4096
        if failing(big, a) == "a*(W/a)":
            found = a
            break
    if found is not None:
        out.append({"W_N": big, "a_m_per_s2": found, "fails": failing(big, found),
                    "fl_W_over_a_kg": big / found, "note": "W/a normal, a*fl(W/a) overflows"})
    found = None
    for k in range(1, 4000):
        a = 1 + k / 4096
        W = tiny * a
        if failing(W, a) == "a*(W/a)":
            found = (W, a)
            break
    if found is not None:
        out.append({"W_N": found[0], "a_m_per_s2": found[1], "fails": "a*(W/a)",
                    "fl_W_over_a_kg": found[0] / found[1], "note": "W and W/a normal, a*fl(W/a) below the normal range"})
    return out


def section_report(model, basis):
    out = {}
    for name, sec in model.spec["sections"].items():
        sb = section_basis(sec, model.spec["materials"]["steel"], basis)
        out[name] = {
            "mass_method": sb["method"],
            "inner_diameter_metal_m": frac_num(sb["inner_diameter_metal"]),
            "inner_diameter_bore_m": frac_num(sb["inner_diameter_bore"]),
            "mass_per_length_kg_per_m": {c: qnum(QPi(0, sb[c])) for c in ("metal", "contents", "insulation", "mbar")},
            "polar_rotary_inertia_per_length_kg_m": qnum(QPi(0, sb["jbar"])),
            "bending_rotary_inertia_per_length_kg_m": qnum(QPi(0, sb["ibar"])),
            "stiffness_section": {"A_m2": qnum(QPi(0, sb["A"])), "I_m4": qnum(QPi(0, sb["I"])),
                                  "J_m4": qnum(QPi(0, sb["J"]))},
        }
    return out


def cross3(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def mass_case(case):
    set_point_precision(POINT_PREC)
    P = PointField()
    model = Model(case["model"])
    eligible = bool(model.body_of)
    acc_used = model.acc if eligible else Fraction(1)
    pc = [P.q(v) for v in model.node_centroid()]
    res = {"id": case["id"], "purpose": case["purpose"], "model": case["model"],
           "component_status": model.component_status,
           "identity_acceleration_m_per_s2": float(acc_used),
           "identity_reference_point_m": [float(v) for v in model.node_centroid()],
           "bases": {}}
    checks = {}
    for basis in ("PF", "EX"):
        Mp, contrib, cats = form_mass(model, P, basis)
        has_arc = any(c["kind"] == "arc" for c in contrib)
        Mq, contrib_q, cats_q = form_mass_exact(model, basis)
        b = {"sections": section_report(model, basis), "elements": []}
        for c, cq in zip(contrib, contrib_q):
            b["elements"].append(element_report(model, c, cq, acc_used, pc, checks, basis))
        totals = {}
        for cat in cats:
            if has_arc and cat != "component_bodies":
                totals[cat] = num(cats[cat])
            else:
                totals[cat] = qnum(cats_q[cat])
        b["category_totals_kg"] = totals
        b["total_mass_kg"] = num(sum(cats.values(), Decimal(0)))
        b["per_direction_kg"] = per_direction(model, P, Mp)
        b["assembled_full_mass_upper_nonzero"] = upper_nonzero(Mp, None if has_arc else Mq)
        b["assembled_dof_order"] = [[nid, d] for nid in model.node_ids for d in DOFS]
        b["weight_identity"] = model_identity(model, P, Mp, acc_used, pc)
        res["bases"][basis] = b
    res["cross_checks"] = checks
    return res


SERIES = None


def half_arc_series_check(geo, c):
    """Each half's lump in its own local frame (e1 radial at the node, e2 the tangent into the half,
    e3 = e1 x e2) against the exact-coefficient series and the direct closed forms in alpha."""
    global SERIES
    if SERIES is None:
        SERIES = cc.half_arc_series()
    alpha = geo.phi / 2
    R, mbar, jbar, ibar = geo.R, c["mbar"], c["jbar"], c["ibar"]
    e1, e2, e3 = geo.Lambda
    cphi, sphi = geo.cos_phi, geo.sin_phi
    e1j = [cphi * e1[k] + sphi * e2[k] for k in range(3)]
    e2j = [sphi * e1[k] - cphi * e2[k] for k in range(3)]
    frames = [[e1, e2, e3], [e1j, e2j, cross3(e1j, e2j)]]
    closed = cc.half_arc_closed_forms(alpha, POINT_PREC)
    worst_closed = Decimal(0)
    for key, p in SERIES.items():
        sv_ = cc.series_eval(p, alpha)
        worst_closed = max(worst_closed, abs(closed[key] - sv_) / abs(sv_))
    m_s, S_s, Jc_s, Js_s = cc.half_arc_local_from_series(SERIES, alpha, R, mbar, jbar, ibar)
    out, worst = [], Decimal(0)
    for lp, Lh in zip(c["lumps"], frames):
        S_l = [sum(Lh[a][k] * lp["first_moment"][k] for k in range(3)) for a in range(3)]

        def loc(J):
            return [[sum(Lh[a][k] * J[k][q] * Lh[b][q] for k in range(3) for q in range(3)) for b in range(3)]
                    for a in range(3)]
        Jc_l, Js_l = loc(lp["centreline_second_moment"]), loc(lp["section_rotary_inertia"])
        pairs = [(lp["mass"], m_s)] + list(zip(S_l, S_s))
        pairs += [(Jc_l[a][b], Jc_s[a][b]) for a in range(3) for b in range(3)]
        pairs += [(Js_l[a][b], Js_s[a][b]) for a in range(3) for b in range(3)]
        floor = m_s * R * R * Decimal("1e-60")
        for got, want in pairs:
            worst = max(worst, abs(got - want) / abs(want) if abs(want) > floor else abs(got) / (m_s * R * R))
        out.append({"node": lp["node"], "local_frame_rows_e1_e2_e3": [vec(r) for r in Lh],
                    "mass_kg": num(m_s), "first_moment_kg_m": vec(S_s),
                    "centreline_second_moment_kg_m2": [vec(r) for r in Jc_s],
                    "section_rotary_inertia_kg_m2": [vec(r) for r in Js_s]})
    return out, worst, worst_closed


def sign_map_check():
    """Design section 2.2: straight matrices for a member along +x equal the plane matrices placed on
    [u_y, theta_z] and [u_z, -theta_y] (exact, invented rational section values)."""
    L, mbar, ibar, jbar = Fraction(5, 2), Fraction(37, 3), Fraction(1, 17), Fraction(2, 17)
    EA, EI, GJ = Fraction(1000), Fraction(13), Fraction(9)
    e = [QF.q(1), QF.q(0), QF.q(0)]

    def plane(coeffs, scale):
        return [[scale * c * (L ** p) for c, p in row] for row in coeffs]

    Bm = plane(el.MASS_PLANE, mbar * L / 420)
    Br = plane(el.ROTARY_PLANE, ibar / (30 * L))
    Bmass = [[Bm[r][c] + Br[r][c] for c in range(4)] for r in range(4)]
    Mx = cc.sign_map_matrix(L, [[mbar * L / 3, mbar * L / 6], [mbar * L / 6, mbar * L / 3]],
                            [[jbar * L / 3, jbar * L / 6], [jbar * L / 6, jbar * L / 3]], Bmass)
    Kx = cc.sign_map_matrix(L, [[EA / L, -EA / L], [-EA / L, EA / L]], [[GJ / L, -GJ / L], [-GJ / L, GJ / L]],
                            plane(el.STIFF_PLANE, EI / L ** 3))
    M = el.straight_mass(QF, e, QF.q(L), QF.q(mbar), QF.q(ibar), QF.q(jbar))
    K = el.straight_stiffness(QF, e, QF.q(L), QF.q(EA), QF.q(EI), QF.q(GJ))
    ok_m = all(M[i][j] == QPi(Mx[i][j]) for i in range(12) for j in range(12))
    ok_k = all(K[i][j] == QPi(Kx[i][j]) for i in range(12) for j in range(12))
    assert ok_m and ok_k
    return {"map": "x-y plane on [u_y, theta_z], x-z plane on [u_z, -theta_y] (member axes, x along the member)",
            "mass_equal_exactly": ok_m, "stiffness_equal_exactly": ok_k,
            "examples_member_along_x": {"M[uy_i][rz_i] / (mbar L^2/420)": str(Mx[1][5] / (mbar * L * L / 420) - Br[0][1] / (mbar * L * L / 420)),
                                        "M[uz_i][ry_i] / (mbar L^2/420)": str(Mx[2][4] / (mbar * L * L / 420) + Br[0][1] / (mbar * L * L / 420))}}


def series_block():
    ser = cc.half_arc_series()
    return {
        "frame": "each half-arc in its own frame at its end node n: e1 radial (centre to n), e2 the tangent at n "
                 "pointing into the half-arc, e3 = e1 x e2; rho/R = (cos psi - 1, sin psi, 0), "
                 "t = (-sin psi, cos psi, 0), psi in [0, a], a the half-angle, ds = R dpsi",
        "scaling": {"mass": "mbar R", "S_i": "mbar R^2", "Jc_ij": "mbar R^3",
                    "Tt_ij": "R; section term Js = R [jbar Tt + ibar (a I - Tt)]"},
        "zero_entries": "S_3, Jc_13, Jc_23, Tt_13, Tt_23, Tt_33 are 0",
        "series_order": cc.N_SERIES,
        "entries": {k: {"closed_form": cc.HALF_ARC_CLOSED_FORMS[k], "leading_terms": cc.leading_terms(p)}
                    for k, p in ser.items()},
    }


def form_mass_exact(model, basis):
    """Exact masses for straights and components (arcs are not in Q + Q pi)."""
    M = el.zeros(QF, model.n, model.n)
    contribs = []
    cats = {"metal": QPi(), "contents": QPi(), "insulation": QPi(), "component_bodies": QPi()}
    pi = QF.pi()
    for mid in model.member_order:
        m = model.members[mid]
        if m["kind"] != "straight" or mid in model.body_of:
            contribs.append(None)
            continue
        sb = model.section(m, basis)
        e, L, _ = model.straight_axis(QF, m)
        blk = el.straight_mass(QF, e, L, QF.q(sb["mbar"]) * pi, QF.q(sb["ibar"]) * pi, QF.q(sb["jbar"]) * pi)
        dofs = model.dofs_of(m["i"]) + model.dofs_of(m["j"])
        for a, ra in enumerate(dofs):
            for c, rc in enumerate(dofs):
                M[ra][rc] = M[ra][rc] + blk[a][c]
        for cat in ("metal", "contents", "insulation"):
            cats[cat] = cats[cat] + QF.q(sb[cat]) * pi * L
        contribs.append({"matrix": blk, "length": L, "axis": e, "mbar": QF.q(sb["mbar"]) * pi})
    for c in model.spec.get("components", []):
        if model.body_of.get(c.get("maps_member")) is not c:
            continue
        mass = QF.q(exact_double(c["weight_N"]) / model.acc)
        r = [QF.q(exact_double(v)) for v in c["center_of_gravity_m"]]
        blk = el.point_mass_lump(QF, mass, r)
        b0 = 6 * model.idx[c["node"]]
        for a in range(6):
            for k in range(6):
                M[b0 + a][b0 + k] = M[b0 + a][b0 + k] + blk[a][k]
        cats["component_bodies"] = cats["component_bodies"] + mass
        contribs.append({"matrix": blk, "mass": mass, "cog": r})
    return M, contribs, cats


def per_direction(model, P, M):
    free = model.free
    out = {}
    for k, ax in enumerate(AXES):
        r = rigid_translation(model, P, k)
        full = el.dot(P, r, el.matvec(P, M, r))
        rf = [r[i] for i in free]
        Mf = reduce(M, free)
        fr = el.dot(P, rf, el.matvec(P, Mf, rf))
        out[ax] = {"full_model": num(full), "free_dofs": num(fr), "not_participating": num(full - fr)}
    return out


def element_report(model, c, cq, acc, pc, checks, basis):
    P = PointField()
    a = P.q(acc)
    kind = c["kind"]
    if c.get("massless"):
        return {"id": c["id"], "kind": kind, "mass_kg": 0, "note": "massless family; no entry in M"}
    if c.get("replaced_by_component"):
        return {"id": c["id"], "kind": kind, "mass_kg": 0,
                "note": f"pipe mass replaced by the body of component {c['replaced_by_component']}"}
    if kind == "straight":
        m = model.members[c["id"]]
        blk_q = cq["matrix"]
        out = {"id": c["id"], "kind": kind, "nodes": [m["i"], m["j"]],
               "length_m": qnum(cq["length"]), "axis": [QPi._c(v).exact_str() for v in cq["axis"]],
               "mass_kg": qnum(cq["mbar"] * cq["length"]),
               "matrix_12x12_upper_nonzero": upper_nonzero(c["matrix"], blk_q)}
        # identity, nodal: a M_e r_k against the consistent uniform-load vector
        L, e, mbar = cq["length"], cq["axis"], cq["mbar"]
        A = QF.q(acc)
        ident = {}
        for k, ax in enumerate(AXES):
            r = [QF.q(1 if i % 6 == k else 0) for i in range(12)]
            got = el.matvec(QF, blk_q, r)
            kh = [QF.q(1 if i == k else 0) for i in range(3)]
            mom = [A * mbar * L * L / QF.q(12) * v for v in cross3(e, kh)]
            fz = [A * mbar * L / QF.q(2) * v for v in kh]
            want = fz + mom + fz + [-v for v in mom]
            gotA = [A * v for v in got]
            ident[ax] = {"a_M_r": [QPi._c(v).exact_str() for v in gotA],
                         "consistent_uniform_load": [QPi._c(v).exact_str() for v in want],
                         "equal_exactly": all(x == y for x, y in zip(gotA, want))}
        out["weight_identity_nodal"] = ident
        # cross-check: shape-function kinetic energy, exact (pi factored)
        sb = model.section(m, basis)
        shape = cc.straight_mass_shape([QPi._c(v).a for v in e], QPi._c(L).a, sb["mbar"], sb["ibar"], sb["jbar"])
        ok = all(blk_q[i][j] == QPi(0, shape[i][j]) for i in range(12) for j in range(12))
        checks[f"{basis}:{c['id']}:shape_function_mass_equal_exactly"] = ok
        assert ok
        return out
    if kind == "arc":
        geo = c["geometry"]
        m = model.members[c["id"]]
        mbar = c["mbar"]
        lumps = []
        tot_m = Decimal(0)
        for lp in c["lumps"]:
            S = lp["first_moment"]
            blk = lp["matrix"]
            lumps.append({"node": lp["node"], "mass_kg": num(lp["mass"]), "first_moment_kg_m": vec(S),
                          "centroid_offset_from_node_m": vec([v / lp["mass"] for v in S]),
                          "second_moment_about_node_kg_m2": [vec(r) for r in [row[3:] for row in blk[3:]]],
                          "matrix_6x6": [vec(r) for r in blk],
                          "variant_centreline_only": {
                              "note": "centreline line mass only, no section rotary inertia (not the design's lump)",
                              "second_moment_about_node_kg_m2": [vec(r) for r in lp["centreline_second_moment"]],
                              "matrix_6x6": [vec(r) for r in lp["matrix_centreline_only"]]},
                          "section_rotary_inertia_kg_m2": [vec(r) for r in lp["section_rotary_inertia"]]})
            tot_m += lp["mass"]
        # identity, resultant: lumps against the arc's weight at its centroid
        centre = geo.centre
        e1 = [(geo.xi[k] - centre[k]) / geo.R for k in range(3)]
        ej = [(geo.xj[k] - centre[k]) / geo.R for k in range(3)]
        bis = [e1[k] + ej[k] for k in range(3)]
        nb = (sum(v * v for v in bis)).sqrt()
        half = geo.phi / 2
        sin_h = (sum(v * v for v in [ej[k] - e1[k] for k in range(3)])).sqrt() / 2
        dist = geo.R * sin_h / half
        centroid = [centre[k] + dist * bis[k] / nb for k in range(3)]
        mass_arc = mbar * geo.R * geo.phi
        ident = {}
        worst = Decimal(0)
        for k, ax in enumerate(AXES):
            F_l, M_l = [Decimal(0)] * 3, [Decimal(0)] * 3
            for lp in c["lumps"]:
                x = model.x(P, lp["node"])
                f = [a * lp["mass"] * (1 if i == k else 0) for i in range(3)]
                kh = [Decimal(1 if i == k else 0) for i in range(3)]
                mo = [a * v for v in cross3(lp["first_moment"], kh)]
                arm = [x[i] - pc[i] for i in range(3)]
                tq = cross3(arm, f)
                F_l = [F_l[i] + f[i] for i in range(3)]
                M_l = [M_l[i] + tq[i] + mo[i] for i in range(3)]
            Fw = [a * mass_arc * (1 if i == k else 0) for i in range(3)]
            Mw = cross3([centroid[i] - pc[i] for i in range(3)], Fw)
            ident[ax] = {"lumps_force_N": vec(F_l), "lumps_moment_about_reference_N_m": vec(M_l),
                         "arc_weight_force_N": vec(Fw), "arc_weight_moment_about_reference_N_m": vec(Mw)}
            scale = max(abs(v) for v in Fw) * (1 + max(abs(v) for v in pc))
            worst = max(worst, max(abs(F_l[i] - Fw[i]) for i in range(3)) / scale,
                        max(abs(M_l[i] - Mw[i]) for i in range(3)) / scale)
        checks[f"{basis}:{c['id']}:lumps_vs_closed_form_arc_resultant_max_rel"] = fmt(worst, 3) if worst else "0"
        assert worst < Decimal("1e-70")
        # cross-check: quadrature of the centreline lumps
        _, qlumps = cc.arc_quadrature(geo, P.q(1), P.q(1), P.q(1), P.q(1), POINT_PREC)
        unit = el.half_arc_unit_lumps(P, geo)
        worst = Decimal(0)
        for (side, mu, S, J), (qm, qS, qJ) in zip(unit, qlumps):
            qSg = geo.to_global_vec(qS)
            qJg = geo.to_global_mat(qJ)
            worst = max(worst, abs(mu - qm) / mu, max(abs(S[i] - qSg[i]) for i in range(3)) / (mu * geo.R),
                        max(abs(J[i][j] - qJg[i][j]) for i in range(3) for j in range(3)) / (mu * geo.R ** 2))
        checks[f"{basis}:{c['id']}:half_arc_lumps_vs_quadrature_max_rel"] = fmt(worst, 3)
        assert worst < Decimal("1e-60")
        # cross-check: each half in its own local frame, against the exact-coefficient series
        local, worst_series, worst_closed = half_arc_series_check(geo, c)
        checks[f"{basis}:{c['id']}:half_arc_local_entries_vs_series_max_rel"] = fmt(worst_series, 3) if worst_series else "0"
        checks[f"{basis}:{c['id']}:series_vs_direct_closed_forms_max_rel"] = fmt(worst_closed, 3) if worst_closed else "0"
        assert worst_series < Decimal("1e-40")
        return {"id": c["id"], "kind": kind, "nodes": [m["i"], m["j"]], "radius_m": float(exact_double(m["radius_m"])),
                "angle_rad": num(geo.phi), "half_angle_alpha_rad": num(geo.phi / 2),
                "angle_deg": num(geo.phi * 180 / pi_decimal(POINT_PREC)),
                "centre_m": vec(centre), "bend_plane_normal": vec(geo.Lambda[2]),
                "length_m": num(geo.R * geo.phi), "mass_kg": num(mass_arc), "centroid_m": vec(centroid),
                "lumps": lumps, "lumps_total_mass_kg": num(tot_m),
                "half_arcs_in_local_frames": local,
                "weight_identity_resultant": ident}
    if kind == "component_body":
        comp = next(x for x in model.spec["components"] if x["id"] == c["id"])
        W = exact_double(comp["weight_N"])
        blk_q = cq["matrix"]
        ident = {}
        for k, ax in enumerate(AXES):
            r = [QF.q(1 if i == k else 0) for i in range(3)] + [QF.q(0)] * 3
            got = [QF.q(acc) * v for v in el.matvec(QF, blk_q, r)]
            kh = [Fraction(1 if i == k else 0) for i in range(3)]
            Fw = [W * v for v in kh]
            want = Fw + cross3([QPi._c(v).a for v in cq["cog"]], Fw)
            ident[ax] = {"a_M_r": [QPi._c(v).exact_str() for v in got],
                         "wrench_W_g_and_r_x_W_g": [str(v) for v in want],
                         "equal_exactly": all(QPi._c(x) == QPi(y, 0) for x, y in zip(got, want))}
        return {"id": c["id"], "kind": kind, "node": c["node"], "maps_member": comp["maps_member"],
                "mass_kg": frac_num(W / model.acc),
                "center_of_gravity_m": [float(exact_double(v)) for v in comp["center_of_gravity_m"]],
                "matrix_6x6": [[QPi._c(v).exact_str() for v in r] for r in blk_q],
                "matrix_6x6_decimal": [vec(r) for r in c["matrix"]],
                "range_premise": range_premise(float(W), float(model.acc),
                                               [float(exact_double(v)) for v in comp["center_of_gravity_m"]]),
                "weight_identity_wrench": ident}
    raise ValueError(kind)


def model_identity(model, P, M, acc, pc):
    a = P.q(acc)
    out = {}
    for k, ax in enumerate(AXES):
        r = rigid_translation(model, P, k)
        f = [a * v for v in el.matvec(P, M, r)]
        Fr, Mr = [Decimal(0)] * 3, [Decimal(0)] * 3
        nodal = {}
        for n, nid in enumerate(model.node_ids):
            fn, mn = f[6 * n:6 * n + 3], f[6 * n + 3:6 * n + 6]
            x = model.x(P, nid)
            tq = cross3([x[i] - pc[i] for i in range(3)], fn)
            Fr = [Fr[i] + fn[i] for i in range(3)]
            Mr = [Mr[i] + mn[i] + tq[i] for i in range(3)]
            nodal[nid] = vec(f[6 * n:6 * n + 6])
        out[ax] = {"a_M_r_nodal": nodal, "resultant_force_N": vec(Fr),
                   "resultant_moment_about_reference_N_m": vec(Mr)}
    return out


# ---------------------------------------------------------------------------- eigen
def eigen_case(case):
    set_point_precision(POINT_PREC)
    P = PointField()
    model = Model(case["model"])
    free = model.free
    K = form_stiffness(model, P)
    M, contrib, cats = form_mass(model, P, "PF")
    Kf, Mf = reduce(K, free), reduce(M, free)
    n = len(free)
    n_finite = eg.rank_psd(Mf)
    cap = min(REPORT_CAP, n_finite)
    lams, reported, nxt, n_counts = eg.route_a(Kf, Mf, n_finite, cap, P)
    n_rep = len(lams)
    checks = {"route_a_point_sturm_factorizations": n_counts}

    # --- vectors (route A): inverse iteration per cluster, canonical basis in clusters
    vecs = []
    for ci, (s, d) in enumerate(reported):
        X = eg.inverse_iteration(Kf, Mf, lams[s], d, seed=1000003 * (ci + 1), against=vecs)
        if d > 1:
            X = eg.canonical_basis(Mf, X)
        vecs.extend(X)
    rq = max(abs(eg.vdot(v, eg.mv(Kf, v)) / eg.vdot(v, eg.mv(Mf, v)) - lams[i]) / lams[i] for i, v in enumerate(vecs))
    res = max(max(abs(x - lams[i] * y) for x, y in zip(eg.mv(Kf, v), eg.mv(Mf, v)))
              / max(abs(x) for x in eg.mv(Kf, v)) for i, v in enumerate(vecs))
    orth = Decimal(0)
    for i in range(n_rep):
        Mv = eg.mv(Mf, vecs[i])
        for j in range(i):
            orth = max(orth, abs(eg.vdot(vecs[j], Mv)))
    checks["route_a_rayleigh_quotient_vs_bisection_max_rel"] = fmt(rq, 3)
    checks["route_a_residual_max_rel"] = fmt(res, 3)
    checks["route_a_m_orthogonality_max"] = fmt(orth, 3) if orth else "0"

    # --- route B: Jacobi on R^-1 M R^-T
    pairs, sweeps = eg.route_b(Kf, Mf)
    lb = [p[0] for p in pairs]
    checks["route_b_jacobi_sweeps"] = sweeps
    checks["route_b_finite_eigenvalue_count"] = len(lb)
    assert len(lb) == n_finite, (len(lb), n_finite)
    checks["route_b_eigenvalue_max_rel_diff"] = fmt(max(abs(lb[i] - lams[i]) / lams[i] for i in range(n_rep)), 3)
    worst_sin = Decimal(0)
    for s, d in reported:
        QB = eg.m_orthonormalize(Mf, [pairs[s + t][1] for t in range(d)])
        worst_sin = max(worst_sin, eg.principal_sines(Mf, vecs[s:s + d], QB))
    checks["route_b_subspace_max_sin_principal_angle"] = fmt(worst_sin, 3) if worst_sin else "0"
    assert Decimal(checks["route_b_eigenvalue_max_rel_diff"]) < Decimal("1e-40")
    assert worst_sin < Decimal("1e-30")

    # --- masses and normalization
    ell = Decimal(model.bbox_diagonal_sq().numerator) / Decimal(model.bbox_diagonal_sq().denominator)
    ell = ell.sqrt()
    # Uniform-support-motion excitation (Design section 1.5, D-v): R_k is the rigid unit
    # translation over all DOFs, p_k = (M R_k)_f = M_ff R_k,f + M_fc R_k,c, and the
    # fractions are relative to T_k = R_k^T M R_k over all DOFs.
    Rk = [rigid_translation(model, P, k) for k in range(3)]
    MR = [eg.mv(M, r) for r in Rk]
    Mr = [[MR[k][g] for g in free] for k in range(3)]
    free_mass = [eg.vdot(Rk[k], MR[k]) for k in range(3)]
    modes, cum = [], [Decimal(0)] * 3
    cluster_of = {}
    for ci, (s, d) in enumerate(reported):
        for t in range(d):
            cluster_of[s + t] = ci + 1
    twopi = 2 * pi_decimal(POINT_PREC)
    for i, v in enumerate(vecs):
        mode, cum = normalize_mode(model, free, v, Mf, Mr, free_mass, ell, cum)
        lam = lams[i]
        omega = lam.sqrt()
        f = omega / twopi
        mode = {"mode": i + 1, "cluster": cluster_of[i],
                "eigenvalue_rad2_per_s2": fmt(lam, EIG_DIG), "omega_rad_per_s": fmt(omega, EIG_DIG),
                "frequency_Hz": fmt(f, EIG_DIG), "frequency_Hz_value": float(f), "period_s": fmt(1 / f, EIG_DIG),
                **mode}
        modes.append(mode)
    clusters = []
    for ci, (s, d) in enumerate(reported):
        effs = {}
        for k, ax in enumerate(AXES):
            effs[ax] = fmt(sum((Decimal(modes[s + t]["effective_mass_kg"][ax]) for t in range(d)), Decimal(0)), SHAPE_DIG)
        clusters.append({"cluster": ci + 1, "modes": list(range(s + 1, s + d + 1)), "multiplicity": d,
                         "eigenvalue_rad2_per_s2": fmt(lams[s], EIG_DIG),
                         "effective_mass_sum_kg": effs})

    # --- certification by interval Sturm counts
    I = IntervalField(IV_PREC)
    Ki = reduce(form_stiffness(model, I), free)
    Mi = reduce(form_mass(model, I, "PF")[0], free)
    fallbacks = 0

    def count_at(sigma_fraction):
        nonlocal fallbacks
        c, idx = eg.sturm_count(I, Ki, Mi, I.q(sigma_fraction))
        if c is None:
            raise ArithmeticError(f"{case['id']}: interval count undecided at {sigma_fraction}")
        fallbacks += 1 if idx else 0
        return c

    brackets = []
    eps = Decimal("1e-33")
    known = lams + ([nxt] if nxt is not None else [])
    for ci, (s, d) in enumerate(reported):
        lo = Decimal(fmt(lams[s] * (1 - eps), DIG))
        hi = Decimal(fmt(lams[s] * (1 + eps), DIG))
        clo, chi = count_at(Fraction(lo)), count_at(Fraction(hi))
        assert (clo, chi) == (s, s + d), (case["id"], ci, clo, chi, s, d)
        brackets.append({"cluster": ci + 1, "sigma_lo": str(lo), "count_lo": clo, "sigma_hi": str(hi), "count_hi": chi})
    if nxt is not None:
        lo = Decimal(fmt(nxt * (1 - eps), DIG))
        c = count_at(Fraction(lo))
        assert c == n_rep
        brackets.append({"cluster": "next", "sigma_lo": str(lo), "count_lo": c})

    shifts = []

    def add_shift(kind, sigma):
        sigma = float(sigma)
        c = count_at(Fraction(sigma))
        sd = Decimal(Fraction(sigma).numerator) / Decimal(Fraction(sigma).denominator)
        near = min(abs(sd - lam) / lam for lam in known)
        below = sum(1 for lam in known if lam < sd)
        if sd < known[-1]:
            assert c == below, (case["id"], kind, sigma, c, below)
        shifts.append({"kind": kind, "sigma_rad2_per_s2": sigma, "sigma_hex": sigma.hex(), "count": c,
                       "nearest_eigenvalue_relative_distance": fmt(near, 3) if near else "0"})

    add_shift("below_lowest", float(lams[0] / 2))
    cl_bounds = [(lams[s], lams[s + d - 1]) for s, d in reported] + ([(nxt, nxt)] if nxt is not None else [])
    for (a_lo, a_hi), (b_lo, _) in zip(cl_bounds, cl_bounds[1:]):
        add_shift("gap_geometric_mean", float((a_hi * b_lo).sqrt()))
    if nxt is None:
        add_shift("above_all_finite", float(2 * lams[-1]))
    for ci, (s, d) in enumerate(reported[:NEAR_CLUSTERS] if not case["id"].startswith("E-RIGID") else reported):
        lam = lams[s]
        fl = float(lam)
        add_shift(f"cluster_{ci + 1}_relative_minus_1e-9", float(lam * (1 - Decimal("1e-9"))))
        add_shift(f"cluster_{ci + 1}_binary64_below", math.nextafter(fl, -math.inf))
        add_shift(f"cluster_{ci + 1}_binary64_nearest", fl)
        add_shift(f"cluster_{ci + 1}_binary64_above", math.nextafter(fl, math.inf))
        add_shift(f"cluster_{ci + 1}_relative_plus_1e-9", float(lam * (1 + Decimal("1e-9"))))
    checks["interval_counts_reordering_fallbacks"] = fallbacks

    mass = {
        "category_totals_kg": {k: num(v) for k, v in cats.items()},
        "total_mass_kg": num(sum(cats.values(), Decimal(0))),
        "per_direction_kg": per_direction(model, P, M),
    }
    # T_k against the sum of effective masses over every finite mode (route B vectors), so
    # that the mass moving with the supports, T_k - sum, is visible.
    sums_all = [Decimal(0)] * 3
    for _, phi in pairs:
        gm = eg.vdot(phi, eg.mv(Mf, phi))
        for k in range(3):
            L = eg.vdot(phi, Mr[k])
            sums_all[k] += L * L / gm
    direct = None
    if n_finite == n:
        fac = eg.lu_factor(Mf)
        direct = [eg.vdot(Mr[k], eg.lu_solve(fac, Mr[k])) for k in range(3)]
        worst = max(abs(direct[k] - sums_all[k]) / free_mass[k] for k in range(3))
        checks["sum_effective_mass_vs_p_Mff_inverse_p_max_rel"] = sv(worst)
        assert worst < Decimal("1e-35")
    mass["participation"] = {
        "excitation": "p_k = (M R_k)_f = M_ff R_k,f + M_fc R_k,c, R_k the rigid unit translation over all DOFs",
        "fraction_denominator": "T_k = R_k^T M R_k over all DOFs (full assembled mass)",
        "per_direction_kg": {AXES[k]: {
            "T_k": num(free_mass[k]),
            "sum_effective_mass_all_finite_modes": num(sums_all[k]),
            "sum_effective_mass_reported_modes": num(sum((Decimal(md["effective_mass_kg"][AXES[k]]) for md in modes),
                                                         Decimal(0))),
            "not_captured_by_any_mode": num(free_mass[k] - sums_all[k]),
        } for k in range(3)},
    }
    if case.get("witness") == "axial_flat_spectrum":
        mass["participation"]["flat_spectrum_witness"] = axial_witness(model, modes, lams)
    return {
        "id": case["id"], "purpose": case["purpose"], "model": case["model"],
        "component_status": model.component_status,
        "free_dofs": [[model.node_ids[k // 6], DOFS[k % 6]] for k in free],
        "n_free": n, "n_finite_eigenvalues": n_finite,
        "bounding_box_diagonal_m": fmt(ell, DIG),
        "mass": mass,
        "reported_mode_count": n_rep,
        "next_eigenvalue_above_reported_rad2_per_s2": fmt(nxt, EIG_DIG) if nxt is not None else None,
        "modes": modes,
        "clusters": clusters,
        "certified_brackets": brackets,
        "sturm_counts": shifts,
        "cross_checks": checks,
        "element_checks": element_checks(model, P),
    }


def normalize_mode(model, free, v, Mf, Mr, free_mass, ell, cum):
    """DEL-04-09 section 1.5: case (a) or (b), generalized mass, participation."""
    n_nodes = len(model.node_ids)
    full = [Decimal(0)] * (6 * n_nodes)
    for k, g in enumerate(free):
        full[g] = v[k]
    trans = [(g, full[g]) for g in free if g % 6 < 3]
    rots = [(g, full[g]) for g in free if g % 6 >= 3]
    t = max((abs(x) for _, x in trans), default=Decimal(0))
    th = max((abs(x) for _, x in rots), default=Decimal(0))
    # Design section 1.5 (revised 2026-10-31): case (b) iff t <= tau_t ell theta; no participation condition
    if t <= TAU_T * ell * th:
        case, pool = "unit_max_rotation", rots
    else:
        case, pool = "unit_max_translation", trans
    big = max(abs(x) for _, x in pool)
    tied = [g for g, x in pool if abs(x) >= big * (1 - Decimal("1e-25"))]
    g0 = min(tied)
    scale = 1 / full[g0]
    full = [x * scale for x in full]
    v = [x * scale for x in v]
    gm = eg.vdot(v, eg.mv(Mf, v))
    Lk = [eg.vdot(v, Mr[k]) for k in range(3)]
    gamma = [Lk[k] / gm for k in range(3)]
    meff = [Lk[k] * Lk[k] / gm for k in range(3)]
    frac = [meff[k] / free_mass[k] if free_mass[k] else None for k in range(3)]
    cum = [cum[k] + frac[k] if frac[k] is not None else None for k in range(3)]
    shape = {}
    for n, nid in enumerate(model.node_ids):
        shape[nid] = [sv(x) for x in full[6 * n:6 * n + 6]]
    ab = case == "unit_max_translation"
    out = {
        "normalization_case": case,
        "normalizing_dof": [model.node_ids[g0 // 6], DOFS[g0 % 6]],
        "normalization_tie_dofs": [[model.node_ids[g // 6], DOFS[g % 6]] for g in tied] if len(tied) > 1 else [],
        "compare_sign_invariant": len(tied) > 1,
        "max_translation_over_max_rotation_times_ell": ratio_str(t, th, ell),
        "shape": shape,
        "generalized_mass": {"value": fmt(gm, SHAPE_DIG), "unit": "kg" if ab else "kg*m^2*rad^-2"},
        "participation_factor": {AXES[k]: sv(gamma[k]) for k in range(3)},
        "participation_factor_unit": "1" if ab else "rad/m",
        "effective_mass_kg": {AXES[k]: sv(meff[k]) for k in range(3)},
        "effective_mass_fraction": {AXES[k]: sv(frac[k]) for k in range(3)},
        "cumulative_effective_mass_fraction": {AXES[k]: sv(cum[k]) for k in range(3)},
    }
    return out, cum


def sv(x):
    """Shape-scale value: SHAPE_DIG digits; |x| below FLUSH printed as 0 (zero in exact arithmetic
    to the generator's resolution); None (no free mass in that direction) printed as null."""
    if x is None:
        return None
    return "0" if abs(x) < FLUSH else fmt(x, SHAPE_DIG)


def axial_witness(model, modes, lams):
    """One axial consistent-mass element fixed at one end: M_ff = m/3, M_fc = m/6, p = m/2.

    Under a flat spectrum of acceleration A the modal displacement is
    u = Gamma phi A / omega^2 = p A / k, so u/A = m / (2 k) with m = mbar L, k = EA / L.
    """
    m = model.members["P1"]
    sb = model.section(m, "PF")
    E = exact_double(model.spec["materials"]["steel"]["elastic_modulus_Pa"])
    e, L, Le = model.straight_axis(QF, m)
    exact = sb["mbar"] * Le * Le / (2 * E * sb["A"])  # pi cancels
    md = modes[0]
    u = Decimal(md["participation_factor"]["X"]) * Decimal(md["shape"][m["j"]][0]) / lams[0]
    mass_m = Decimal(sb["mbar"].numerator) / Decimal(sb["mbar"].denominator) * pi_decimal(POINT_PREC) * Decimal(Le.numerator) / Decimal(Le.denominator)
    return {
        "element_mass_m_kg": num(mass_m),
        "M_ff_kg": num(mass_m / 3), "M_fc_kg": num(mass_m / 6), "p_X_kg": num(mass_m / 2),
        "effective_mass_X_kg": num(3 * mass_m / 4),
        "u_over_A_s2_exact": str(exact), "u_over_A_s2": num(Decimal(exact.numerator) / Decimal(exact.denominator)),
        "u_over_A_from_modes_s2": fmt(u, SHAPE_DIG),
    }


def ratio_str(t, th, ell):
    """t / (theta ell) with 3 digits; below FLUSH printed as 0; no rotation printed as inf."""
    if not th:
        return "inf" if t else "0"
    r = t / (th * ell)
    return "0" if r < FLUSH else fmt(r, 3)


def element_checks(model, P):
    """Straight K against Castigliano (exact); arc K rigid-body nullity and flexibility against quadrature."""
    out = {}
    for mid in model.member_order:
        m = model.members[mid]
        mat = model.spec["materials"][m["material"]]
        E, G = exact_double(mat["elastic_modulus_Pa"]), exact_double(mat["shear_modulus_Pa"])
        s = model.scale_of.get(mid, Fraction(1))
        sb = model.section(m, "PF")
        if m["kind"] == "straight":
            e, L, Le = model.straight_axis(QF, m)
            K = el.straight_stiffness(QF, e, L, QF.q(s * E * sb["A"]) * QF.pi(), QF.q(s * E * sb["I"]) * QF.pi(),
                                      QF.q(s * G * sb["J"]) * QF.pi())
            Kc = cc.straight_stiffness_castigliano([QPi._c(v).a for v in e], Le, s * E * sb["A"], s * E * sb["I"],
                                                   s * G * sb["J"])
            ok = all(K[i][j] == QPi(0, Kc[i][j]) for i in range(12) for j in range(12))
            assert ok
            out[f"{mid}:stiffness_equals_castigliano_exactly"] = ok
        elif m["kind"] == "arc":
            geo = model.arc_geometry(P, m)
            pi = P.pi()
            EA, EI, GJ = P.q(E * sb["A"]) * pi, P.q(E * sb["I"]) * pi, P.q(G * sb["J"]) * pi
            k = P.q(exact_double(m["flexibility_factor"]))
            f = el.arc_tip_flexibility_local(P, geo, EA, EI, GJ, k)
            fq, _ = cc.arc_quadrature(geo, EA, EI, GJ, k, POINT_PREC)
            worst = max(abs(f[i][j] - fq[i][j]) / (f[i][i] * f[j][j]).sqrt() for i in range(6) for j in range(6))
            assert worst < Decimal("1e-60")
            out[f"{mid}:flexibility_vs_quadrature_max_rel"] = fmt(worst, 3)
            Ka = el.arc_stiffness(P, geo, EA, EI, GJ, k)
            nullity = Decimal(0)
            kmax = max(abs(x) for r in Ka for x in r)
            for c in range(6):
                v = [Decimal(0)] * 12
                if c < 3:
                    v[c] = v[6 + c] = Decimal(1)
                else:
                    th = [Decimal(1 if q == c - 3 else 0) for q in range(3)]
                    v[0:3], v[3:6] = cross3(th, geo.xi), th
                    v[6:9], v[9:12] = cross3(th, geo.xj), th
                nullity = max(nullity, max(abs(x) for x in el.matvec(P, Ka, v)) / kmax)
            assert nullity < Decimal("1e-60")
            out[f"{mid}:rigid_body_residual_max_rel"] = fmt(nullity, 3) if nullity else "0"
    return out


# ---------------------------------------------------------------------------- driver
def dumps(obj):
    return json.dumps(obj, indent=1, ensure_ascii=False) + "\n"


def build(jobs):
    mcases = cases.mass_cases()
    ecases = cases.eigen_cases()
    with ProcessPoolExecutor(max_workers=jobs) as ex:
        mres = list(ex.map(mass_case, mcases))
        eres = list(ex.map(eigen_case, sorted(ecases, key=lambda c: -len(c["model"]["nodes"]))))
    order = {c["id"]: k for k, c in enumerate(ecases)}
    eres.sort(key=lambda r: order[r["id"]])
    common = {
        "status": "candidate frozen references; derived and cross-checked by the author TASK, not yet independently refuted",
        "deliverable": "DEL-04-09 (F1 modal analysis)",
        "units": {"length": "m", "mass": "kg", "force": "N", "moment": "N*m", "time": "s",
                  "eigenvalue": "rad^2/s^2", "frequency": "Hz"},
        "dof_order": DOFS,
        "number_format": {"exact": "rational or rational + rational*pi", "decimal": f"{DIG} significant digits",
                          "value": "nearest binary64"},
    }
    mass = {"document": "openpipestress.f1_modal.mass_discrete.mass_references", **common,
            "acceptance_criterion": "AC-001",
            "range_premise_witnesses": range_witnesses(), "half_arc_local_series": series_block(),
            "bending_plane_sign_map": sign_map_check(), "cases": mres}
    eig = {"document": "openpipestress.f1_modal.mass_discrete.eigen_references", **common,
           "acceptance_criterion": "AC-003",
           "class": "(a) formulation-matched discrete references (same discretization as the product, PF basis)",
           "participation": "uniform-support-motion excitation p_k = (M R_k)_f; fractions relative to T_k = R_k^T M R_k "
                            "(DEL-04-09 Design section 1.5, D-v)",
           "parameters": {"tau_t": "1e-6", "report_cap": REPORT_CAP, "cluster_relative_equality": str(eg.CLUSTER_REL),
                          "eigenvalue_digits": EIG_DIG, "shape_digits": SHAPE_DIG,
                          "certified_bracket_half_width_relative": "1e-33"},
           "cases": eres}
    return {"mass_references.json": dumps(mass), "eigen_references.json": dumps(eig)}


def sums(files):
    lines = []
    for name in sorted(list(FIXED_FILES) + list(files)):
        data = files[name].encode() if name in files else (OUT / name).read_bytes()
        lines.append(f"{hashlib.sha256(data).hexdigest()}  {name}")
    return "\n".join(lines) + "\n"


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--write", action="store_true")
    ap.add_argument("--check", action="store_true", help="default; regenerate and compare")
    ap.add_argument("--jobs", type=int, default=os.cpu_count())
    args = ap.parse_args()
    files = build(args.jobs)
    files["SHA256SUMS"] = sums(files)
    if args.write:
        for name, text in files.items():
            (OUT / name).write_text(text)
        print("wrote", ", ".join(sorted(files)))
        return 0
    bad = []
    for name, text in files.items():
        p = OUT / name
        if not p.exists() or p.read_text() != text:
            bad.append(name)
    if bad:
        print("MISMATCH:", ", ".join(bad))
        return 1
    print("check passed:", ", ".join(sorted(files)))
    return 0


if __name__ == "__main__":
    sys.exit(main())
