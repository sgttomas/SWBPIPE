#!/usr/bin/env python3
"""W-9 independent references: PRD section 22.2 solver benchmarks.

Standard-library Python 3 only. This generator imports, reads and runs no
product code and reads no file other than the ones it writes. Every expected
value is derived here in exact rational arithmetic from textbook closed forms:

  W9-A  sharp-corner L frame, both ends fixed, uniform temperature rise
        (force method: release D, three redundants, compatibility);
  W9-B  simply supported straight pipe under its own metal weight
        (Euler-Bernoulli beam under uniform load);
  W9-C  the W9-A frame with no load and a prescribed settlement of anchor D
        (force method, same flexibility matrix).

Every result is q * pi**k with q rational: forces and moments carry k = 1
(EA, EI and the weight are rational multiples of pi), translations and
rotations carry k = 0. Values are held as the rational q with the family's
power of pi. Decimals are printed to 40 significant digits from a 90-digit
Machin value of pi; binary64 values are the correctly rounded nearest doubles,
checked by rounding with lower and upper rational bounds of pi.

An independent second route checks every published value: an exact
direct-stiffness solve of the same plane frame (3 DOF per node, consistent
fixed-end loads, the same pi-free scaling), its reactions, nodal motions and
element end actions. The out-of-plane problem of each case is unloaded and
stable, so its components are exactly zero.

Modes:
  python3 -I w9_reference.py           check: rebuild every output in memory and
                                        compare byte for byte with the committed
                                        files, SHA256SUMS included
  python3 -I w9_reference.py --write   write the outputs (intentional changes only)
"""
import argparse
import hashlib
import json
import sys
from decimal import Decimal, getcontext
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE.parent
FIXED_FILES = ("README.md", "generators/w9_reference.py")

# ----------------------------------------------------------------------------- pi
getcontext().prec = 100


def _atan_inv(x):
    x = Decimal(x)
    total, term, n, sign = Decimal(0), 1 / x, 1, 1
    x2 = x * x
    eps = Decimal(10) ** -(getcontext().prec - 5)
    while True:
        delta = term / n
        if delta < eps:
            break
        total += sign * delta
        term /= x2
        n += 2
        sign = -sign
    return total


PI_DEC = 16 * _atan_inv(5) - 4 * _atan_inv(239)  # Machin, ~95 correct digits
_PI_90 = F(str(PI_DEC)[:92])  # "3." + 90 digits, truncated
PI_LO, PI_HI = _PI_90, _PI_90 + F(1, 10 ** 90)
assert PI_LO < F(314159265358979323846, 10 ** 20) + F(1, 10 ** 20) and PI_HI > F(314159265358979323846, 10 ** 20)


def binary64(q, k):
    """Nearest double to q * pi**k, the same for both rational bounds of pi."""
    if k == 0:
        return float(q)
    lo, hi = float(q * PI_LO), float(q * PI_HI)
    if lo != hi:
        raise SystemExit(f"binary64 rounding of {q}*pi is not decided by the pi bounds")
    return lo


def decimal40(q, k):
    if q == 0:
        return "0"
    getcontext().prec = 100
    v = Decimal(q.numerator) / Decimal(q.denominator) * (PI_DEC if k else 1)
    return format(v.normalize().__format__(".39e"))


def exact_text(q, k):
    s = str(q)
    return s if k == 0 or q == 0 else f"{s}*pi"


def record(q, k):
    q = F(q)
    return {"exact": exact_text(q, k), "decimal": decimal40(q, k), "value": binary64(q, k)}


# ----------------------------------------------------------------------------- inputs (invented)
OD, ID = F(2, 10), F(18, 100)  # m
WALL = (OD - ID) / 2  # 0.01 m
E = F(200 * 10 ** 9)  # Pa
NU = F(3, 10)  # exact route only; no published value depends on it
G_PF = F(76923076923)  # Pa, PF route only; no published value depends on it
AREA_P = (OD ** 2 - ID ** 2) / 4  # A / pi
INERTIA_P = (OD ** 4 - ID ** 4) / 64  # I / pi
EA_P, EI_P = E * AREA_P, E * INERTIA_P  # EA / pi, EI / pi
assert AREA_P == F(19, 10000) and INERTIA_P == F(85975, 10 ** 10)
assert EA_P == 380000000 and EI_P == 1719500  # t3_r1 N-series coefficients
ALPHA, DELTA_T = F(12, 10 ** 6), F(100)
EPS_T = ALPHA * DELTA_T  # free thermal strain 3/2500
RHO, GRAV = F(7850), F(980665, 100000)
W_P = RHO * GRAV * AREA_P  # weight per length / pi, N/m
L_A, L_B = F(4), F(3)  # A->C along +X, C->D along +Y
L_SPAN = F(6)
SETTLE = (F(0), F(-1, 100), F(0))  # W9-C anchor D motion (UX, UY, RZ)

COMPONENTS_F = ("Fx", "Fy", "Fz", "Mx", "My", "Mz")
COMPONENTS_U = ("ux", "uy", "uz", "rx", "ry", "rz")
SECTION_COMPONENTS = ("axial_force", "shear_force_y", "shear_force_z",
                      "torsional_moment", "bending_moment_y", "bending_moment_z")
STATIONS = (F(0), F(1, 4), F(1, 2), F(3, 4), F(1))
STATION_NAMES = ("0", "1/4", "1/2", "3/4", "1")

CROSS = []


def cross(name, a, b):
    CROSS.append({"check": name, "agrees": a == b})
    if a != b:
        print(f"DISAGREES {name}: {a} != {b}")


# ----------------------------------------------------------------------------- linear algebra
def solve(a, y):
    n = len(a)
    m = [list(row) + [y[i]] for i, row in enumerate(a)]
    for c in range(n):
        p = next(r for r in range(c, n) if m[r][c] != 0)
        m[c], m[p] = m[p], m[c]
        for r in range(n):
            if r != c and m[r][c] != 0:
                k = m[r][c] / m[c][c]
                m[r] = [x - k * z for x, z in zip(m[r], m[c])]
    return [m[i][n] / m[i][i] for i in range(n)]


def matvec(a, x):
    return [sum(r * v for r, v in zip(row, x)) for row in a]


# ----------------------------------------------------------------------------- route 1: force method
def flexibility(a, b):
    """Flexibility at D of the cantilever A(fixed)->C->D, pi-free (times pi).

    Redundants at D: X (+X force), Y (+Y force), M (+Z moment), all actions of
    the support on the structure. Bending moment about +Z of the D-side
    loads, taken at point P: m(P) = M + (xD - xP) Y - (yD - yP) X.
      member C->D, s = yD - yP in [0, b]:  m = M - X s,          N = Y
      member A->C, t = xD - xP in [0, a]:  m = M + Y t - X b,     N = X
    Unit-load virtual work with EB bending and axial terms.
    """
    fxx = (b ** 3 / 3 + a * b ** 2) / EI_P + a / EA_P
    fyy = a ** 3 / (3 * EI_P) + b / EA_P
    fmm = (a + b) / EI_P
    fxy = -(a ** 2) * b / (2 * EI_P)
    fxm = -(b ** 2 / 2 + a * b) / EI_P
    fym = a ** 2 / (2 * EI_P)
    return [[fxx, fxy, fxm], [fxy, fyy, fym], [fxm, fym, fmm]]


def lframe_force_method(d_free, d_imposed):
    """Redundants R_D / pi from f R = d_imposed - d_free (released D motion)."""
    f = flexibility(L_A, L_B)
    rhs = [d_imposed[i] - d_free[i] for i in range(3)]
    return solve(f, rhs)


def lframe_results(case_id, thermal, d_imposed):
    a, b = L_A, L_B
    d_free = (EPS_T * a, EPS_T * b, F(0)) if thermal else (F(0), F(0), F(0))
    X, Y, M = lframe_force_method(d_free, d_imposed)
    # Reactions (support on structure), /pi; A by statics about A.
    r_d = {"Fx": X, "Fy": Y, "Mz": M}
    r_a = {"Fx": -X, "Fy": -Y, "Mz": -(M + a * Y - b * X)}
    # Corner C by tree integration of A->C under the D-side resultant.
    m_c = M - b * X  # moment of D-side loads about C
    eps = EPS_T if thermal else F(0)
    uc = {"ux": eps * a + X * a / EA_P,
          "uy": Y * a ** 3 / (3 * EI_P) + m_c * a ** 2 / (2 * EI_P),
          "rz": Y * a ** 2 / (2 * EI_P) + m_c * a / EI_P}
    # D by continuing along C->D: compatibility self-check of the flexibility.
    ud = d_motion_from_corner(uc, X, Y, M, eps)
    cross(f"{case_id}: compatibility at D (force method + tree integration)", ud, tuple(d_imposed))
    # Section actions on the +x face of the i-side portion, element-local.
    sections = {}
    for pipe, (frm, length) in (("pipe:a-c", ("A", a)), ("pipe:c-d", ("C", b))):
        rows = []
        for s in STATIONS:
            if pipe == "pipe:a-c":
                t = a * (1 - s)  # xD - xP
                n, vy, mz = X, Y, M + Y * t - X * b
            else:
                sd = b * (1 - s)  # yD - yP
                n, vy, mz = Y, -X, M - X * sd  # local y = -X
            rows.append({"axial_force": n, "shear_force_y": vy, "shear_force_z": F(0),
                         "torsional_moment": F(0), "bending_moment_y": F(0), "bending_moment_z": mz})
        sections[pipe] = rows
    nodes = {"node:a": {"ux": F(0), "uy": F(0), "rz": F(0)},
             "node:c": uc,
             "node:d": {"ux": d_imposed[0], "uy": d_imposed[1], "rz": d_imposed[2]}}
    reactions = {"support:a": r_a, "support:d": r_d}
    return reactions, nodes, sections, {"X": X, "Y": Y, "M": M, "corner_moment": m_c}


def d_motion_from_corner(uc, X, Y, M, eps):
    """Motion of D from C's motion plus member C->D as a cantilever from C.

    Member C->D (length b, along +Y) carries the tip force (X, Y) and tip
    moment M at D. Bending in the X-Y plane: the transverse direction is X.
    With w(s) the +X deflection relative to the rigid motion, s from C:
      EI w'' = -m(s),  m(s) = M - X (b - s)   (moment about +Z)
    since a +Z curvature of a member along +Y bends it toward -X.
    """
    b = L_B
    # rigid motion of C carried to D: u_D = u_C + theta_C x (r_D - r_C)
    rx = uc["ux"] - uc["rz"] * b
    ry = uc["uy"]
    # w(b) from EI w'' = -(M - X b + X s), w(0) = w'(0) = 0:
    #   w(b) = -(M - X b) b^2/2 / EI - X b^3/6 / EI
    w_b = (-(M - X * b) * b ** 2 / 2 - X * b ** 3 / 6) / EI_P
    # rotation about +Z: theta = -w' ; w'(b) = -((M - X b) b + X b^2/2) / EI
    th_b = ((M - X * b) * b + X * b ** 2 / 2) / EI_P
    ext = eps * b + Y * b / EA_P
    return (rx + w_b, ry + ext, uc["rz"] + th_b)


# ----------------------------------------------------------------------------- route 1: simple beam
def beam_results():
    w, L = W_P, L_SPAN  # load acts along -Y
    R = w * L / 2
    reactions = {"support:a": {"Fy": R}, "support:b": {"Fy": R}}
    theta = w * L ** 3 / (24 * EI_P)
    delta = 5 * w * L ** 4 / (384 * EI_P)
    nodes = {"node:a": {"rz": -theta}, "node:mid": {"uy": -delta, "rz": F(0)}, "node:b": {"rz": theta}}
    sections = {}
    for pipe, x0 in (("pipe:a-mid", F(0)), ("pipe:mid-b", L / 2)):
        rows = []
        for s in STATIONS:
            x = x0 + (L / 2) * s
            # i-side portion [0, x]: R (+Y) at 0, w (-Y) over [0, x];
            # +x face: V = w x - R, M = R x - w x^2 / 2
            rows.append({"axial_force": F(0), "shear_force_y": w * x - R, "shear_force_z": F(0),
                         "torsional_moment": F(0), "bending_moment_y": F(0),
                         "bending_moment_z": R * x - w * x ** 2 / 2})
        sections[pipe] = rows
    summary = {"w": w, "R": R, "M_mid": w * L ** 2 / 8, "delta_mid": delta, "theta_end": theta}
    cross("W9-B: midspan section moment equals wL^2/8", sections["pipe:a-mid"][-1]["bending_moment_z"], w * L ** 2 / 8)
    return reactions, nodes, sections, summary


# ----------------------------------------------------------------------------- route 2: direct stiffness
def element_k(length, c, s):
    """Global 6x6 plane-frame stiffness / pi and the transformation."""
    L = length
    ea, ei = EA_P / L, EI_P
    k = [[ea, 0, 0, -ea, 0, 0],
         [0, 12 * ei / L ** 3, 6 * ei / L ** 2, 0, -12 * ei / L ** 3, 6 * ei / L ** 2],
         [0, 6 * ei / L ** 2, 4 * ei / L, 0, -6 * ei / L ** 2, 2 * ei / L],
         [-ea, 0, 0, ea, 0, 0],
         [0, -12 * ei / L ** 3, -6 * ei / L ** 2, 0, 12 * ei / L ** 3, -6 * ei / L ** 2],
         [0, 6 * ei / L ** 2, 2 * ei / L, 0, -6 * ei / L ** 2, 4 * ei / L]]
    k = [[F(v) for v in row] for row in k]
    t3 = [[c, s, 0], [-s, c, 0], [0, 0, 1]]
    t = [[F(0)] * 6 for _ in range(6)]
    for blk in (0, 3):
        for i in range(3):
            for j in range(3):
                t[blk + i][blk + j] = F(t3[i][j])
    return k, t


def transpose(a):
    return [list(r) for r in zip(*a)]


def matmul(a, b):
    bt = transpose(b)
    return [[sum(x * y for x, y in zip(r, col)) for col in bt] for r in a]


def direct_stiffness(nodes, elements, fixed, prescribed):
    """nodes: {id: (x, y)}; elements: [(id, i, j, fixed_end_local)].

    fixed_end_local: node-on-element end actions with both ends held, local.
    Returns nodal motions, support actions on the structure (Σ node-on-element
    at restrained DOFs, no nodal loads) and element end actions (local).
    """
    ids = list(nodes)
    index = {n: 3 * k for k, n in enumerate(ids)}
    n = 3 * len(ids)
    K = [[F(0)] * n for _ in range(n)]
    P = [F(0)] * n
    data = []
    for eid, i, j, fef in elements:
        (xi, yi), (xj, yj) = nodes[i], nodes[j]
        dx, dy = xj - xi, yj - yi
        assert dx == 0 or dy == 0  # members lie along a global axis
        L = abs(dx) + abs(dy)
        c, s = dx / L, dy / L
        k, t = element_k(L, c, s)
        kg = matmul(matmul(transpose(t), k), t)
        dofs = [index[i] + d for d in range(3)] + [index[j] + d for d in range(3)]
        fg = matvec(transpose(t), fef)
        for a_ in range(6):
            P[dofs[a_]] -= fg[a_]
            for b_ in range(6):
                K[dofs[a_]][dofs[b_]] += kg[a_][b_]
        data.append((eid, dofs, k, t, fef))
    restrained = sorted({index[nd] + d for nd, ds in fixed.items() for d in ds})
    U = [F(0)] * n
    for (nd, d), v in prescribed.items():
        U[index[nd] + d] = F(v)
    free = [d for d in range(n) if d not in restrained]
    rhs = [P[f] - sum(K[f][r] * U[r] for r in restrained) for f in free]
    sol = solve([[K[f][g] for g in free] for f in free], rhs)
    for f, v in zip(free, sol):
        U[f] = v
    ends = {}
    total = [F(0)] * n
    for eid, dofs, k, t, fef in data:
        ul = matvec(t, [U[d] for d in dofs])
        fl = [x + y for x, y in zip(matvec(k, ul), fef)]
        ends[eid] = fl
        fg = matvec(transpose(t), fl)
        for a_ in range(6):
            total[dofs[a_]] += fg[a_]
    motions = {nd: {"ux": U[index[nd]], "uy": U[index[nd] + 1], "rz": U[index[nd] + 2]} for nd in ids}
    reactions = {}
    for nd, ds in fixed.items():
        reactions[nd] = {("Fx", "Fy", "Mz")[d]: total[index[nd] + d] for d in ds}
    # free-DOF equilibrium (no nodal loads)
    for f in free:
        assert total[f] == 0
    return motions, reactions, ends


def check_lframe_ds(case_id, thermal, d_imposed, reactions, nodes, sections):
    eps = EPS_T if thermal else F(0)
    fef = [EA_P * eps, F(0), F(0), -EA_P * eps, F(0), F(0)]  # held thermal member, /pi
    m, r, ends = direct_stiffness(
        {"node:a": (F(0), F(0)), "node:c": (L_A, F(0)), "node:d": (L_A, L_B)},
        [("pipe:a-c", "node:a", "node:c", fef), ("pipe:c-d", "node:c", "node:d", fef)],
        {"node:a": (0, 1, 2), "node:d": (0, 1, 2)},
        {("node:d", k): d_imposed[k] for k in range(3)})
    for nd in ("node:a", "node:c", "node:d"):
        for comp in ("ux", "uy", "rz"):
            cross(f"{case_id}: {nd} {comp} direct stiffness", m[nd][comp], nodes[nd][comp])
    for sup, nd in (("support:a", "node:a"), ("support:d", "node:d")):
        for comp in ("Fx", "Fy", "Mz"):
            cross(f"{case_id}: {sup} {comp} direct stiffness", r[nd][comp], reactions[sup][comp])
    check_ends(case_id, ends, sections)


def check_ends(case_id, ends, sections):
    for pipe, rows in sections.items():
        fl = ends[pipe]
        for slot, (comp, row, sign) in enumerate((("axial_force", 0, -1), ("shear_force_y", 0, -1),
                                                  ("bending_moment_z", 0, -1), ("axial_force", -1, 1),
                                                  ("shear_force_y", -1, 1), ("bending_moment_z", -1, 1))):
            cross(f"{case_id}: {pipe} end action {comp} {'end_j' if row else 'end_i'} direct stiffness",
                  fl[slot], sign * rows[row][comp])


def check_beam_ds(reactions, nodes, sections):
    w, L = W_P, L_SPAN
    h = L / 2
    q = -w  # local y load
    fef = [F(0), -q * h / 2, -q * h ** 2 / 12, F(0), -q * h / 2, q * h ** 2 / 12]
    m, r, ends = direct_stiffness(
        {"node:a": (F(0), F(0)), "node:mid": (h, F(0)), "node:b": (L, F(0))},
        [("pipe:a-mid", "node:a", "node:mid", fef), ("pipe:mid-b", "node:mid", "node:b", fef)],
        {"node:a": (0, 1), "node:b": (1,)}, {})
    for nd, comps in (("node:a", ("ux", "uy", "rz")), ("node:mid", ("ux", "uy", "rz")), ("node:b", ("ux", "uy", "rz"))):
        for comp in comps:
            cross(f"W9-B: {nd} {comp} direct stiffness", m[nd][comp], nodes[nd].get(comp, F(0)))
    for sup, nd in (("support:a", "node:a"), ("support:b", "node:b")):
        for comp, val in r[nd].items():
            cross(f"W9-B: {sup} {comp} direct stiffness", val, reactions[sup].get(comp, F(0)))
    check_ends("W9-B", ends, sections)


# ----------------------------------------------------------------------------- publication
def family(unit):
    return {"N": "force", "N*m": "moment", "m": "translation", "rad": "rotation"}[unit]


def publish(case_id, reactions, nodes, sections, node_ids):
    rows = []
    for sup in sorted(reactions):
        for comp in COMPONENTS_F:
            unit = "N" if comp.startswith("F") else "N*m"
            rows.append(dict(quantity="support_reaction", entity=sup, component=comp, unit=unit,
                             **record(reactions[sup].get(comp, F(0)), 1)))
    for nd in node_ids:
        for comp in COMPONENTS_U:
            unit = "m" if comp.startswith("u") else "rad"
            rows.append(dict(quantity="nodal_motion", entity=nd, component=comp, unit=unit,
                             **record(nodes[nd].get(comp, F(0)), 0)))
    for pipe in sorted(sections):
        for name, sec in zip(STATION_NAMES, sections[pipe]):
            for comp in SECTION_COMPONENTS:
                unit = "N" if comp in ("axial_force", "shear_force_y", "shear_force_z") else "N*m"
                rows.append(dict(quantity="section_action", entity=pipe, station=name, component=comp,
                                 unit=unit, **record(sec[comp], 1)))
    scales = {}
    for fam, unit_k in (("force", ("N", 1)), ("moment", ("N*m", 1)), ("translation", ("m", 0)), ("rotation", ("rad", 0))):
        mags = [abs(F(r["exact"].replace("*pi", ""))) for r in rows if family(r["unit"]) == fam]
        top = max(mags)
        assert top > 0, (case_id, fam)
        scales[fam] = dict(unit=unit_k[0], **record(top, unit_k[1]))
    return rows, scales


def build_cases():
    cases = []
    common_section = {"outside_diameter_m": str(OD), "inside_diameter_m": str(ID), "wall_thickness_m": str(WALL),
                      "elastic_modulus_Pa": str(E), "area_m2": f"{AREA_P}*pi",
                      "second_moment_m4": f"{INERTIA_P}*pi", "EA_N": f"{EA_P}*pi", "EI_N_m2": f"{EI_P}*pi",
                      "shear_modulus_Pa_preview_route": str(G_PF), "poisson_ratio_exact_route": str(NU)}
    geometry_l = {"nodes": {"node:a": [0, 0, 0], "node:c": [4, 0, 0], "node:d": [4, 3, 0]},
                  "pipes": {"pipe:a-c": {"from": "node:a", "to": "node:c", "y_reference": [0, 1, 0]},
                            "pipe:c-d": {"from": "node:c", "to": "node:d", "y_reference": [-1, 0, 0]}},
                  "supports": {"support:a": {"node": "node:a", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"]},
                               "support:d": {"node": "node:d", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"]}}}
    for case_id, thermal, motion in (("W9-A-LFRAME-THERMAL", True, (F(0), F(0), F(0))),
                                     ("W9-C-LFRAME-ANCHOR-SETTLEMENT", False, SETTLE)):
        reactions, nodes, sections, red = lframe_results(case_id, thermal, motion)
        check_lframe_ds(case_id, thermal, motion, reactions, nodes, sections)
        rows, scales = publish(case_id, reactions, nodes, sections, ("node:a", "node:c", "node:d"))
        inputs = {"section": common_section, "geometry": geometry_l}
        if thermal:
            inputs["thermal"] = {"alpha_per_K": str(ALPHA), "delta_T_K": str(DELTA_T), "free_strain": str(EPS_T),
                                 "applied_to": ["pipe:a-c", "pipe:c-d"]}
        else:
            inputs["support_motion"] = {"support": "support:d", "UX_m": str(motion[0]), "UY_m": str(motion[1]),
                                        "RZ_rad": str(motion[2]), "other_dofs": "0"}
        key = {"redundant_X_N": record(red["X"], 1), "redundant_Y_N": record(red["Y"], 1),
               "redundant_M_N_m": record(red["M"], 1),
               "anchor_A_moment_N_m": record(reactions["support:a"]["Mz"], 1),
               "corner_moment_N_m": record(red["corner_moment"], 1)}
        cases.append({"id": case_id, "method": "force method (flexibility), release D, compatibility; A by statics; "
                      "corner by tree integration of A->C", "inputs": inputs, "key_values": key,
                      "zero_scales": scales, "rows": rows})
    reactions, nodes, sections, summary = beam_results()
    check_beam_ds(reactions, nodes, sections)
    rows, scales = publish("W9-B", reactions, nodes, sections, ("node:a", "node:mid", "node:b"))
    inputs = {"section": common_section,
              "geometry": {"nodes": {"node:a": [0, 0, 0], "node:mid": [3, 0, 0], "node:b": [6, 0, 0]},
                           "pipes": {"pipe:a-mid": {"from": "node:a", "to": "node:mid", "y_reference": [0, 1, 0]},
                                     "pipe:mid-b": {"from": "node:mid", "to": "node:b", "y_reference": [0, 1, 0]}},
                           "supports": {"support:a": {"node": "node:a", "restraints": ["UX", "UY", "UZ", "RX"]},
                                        "support:b": {"node": "node:b", "restraints": ["UY", "UZ"]}}},
              "weight": {"material_density_kg_m3": str(RHO), "gravity_m_s2": str(GRAV), "gravity_axis": "global_y",
                         "gravity_sign": "-1", "intensity_N_per_m": f"{W_P}*pi", "basis": "metal area only"}}
    key = {"w_N_per_m": record(summary["w"], 1), "R_end_N": record(summary["R"], 1),
           "M_mid_N_m": record(summary["M_mid"], 1), "delta_mid_m": record(summary["delta_mid"], 0),
           "theta_end_rad": record(summary["theta_end"], 0)}
    cases.insert(1, {"id": "W9-B-SIMPLE-BEAM-SELF-WEIGHT", "method": "Euler-Bernoulli simply supported beam, uniform load",
                     "inputs": inputs, "key_values": key, "zero_scales": scales, "rows": rows})
    return cases


def document(cases):
    return {
        "document": "openpipestress.w9_benchmark_references",
        "status": "candidate frozen references; derived and cross-checked by the author TASK, not yet independently refuted",
        "units": {"force": "N", "moment": "N*m", "translation": "m", "rotation": "rad"},
        "theory": "small-displacement linear-elastic Euler-Bernoulli space frame, no shear deformation; annular section, Iy = Iz = I",
        "conventions": {
            "support_reaction": "global action of the support on the structure at its node (moments about that node)",
            "nodal_motion": "global translation and right-handed rotation",
            "section_action": "element-local action on the +x face of the i-side portion at station s/L (local x from i to j); "
                              "a node-on-element end action is -section(0) at end_i and +section(1) at end_j",
            "exact": "rational q, or q*pi",
        },
        "criterion": "|observed - expected| <= 1e-9 * |expected|; an exactly zero expectation uses 1e-9 * zero_scale "
                     "of its family (largest nonzero magnitude of that family in the case)",
        "crosscheck": list(CROSS),
        "cases": cases,
    }


def dumps(obj):
    def inline(o):
        return json.dumps(o, ensure_ascii=False, separators=(", ", ": "))

    def emit(o, level):
        pad = " " * level
        if isinstance(o, dict) and ("exact" in o or "check" in o):
            return inline(o)
        if isinstance(o, dict):
            items = [f'{pad} {json.dumps(k)}: {emit(v, level + 1)}' for k, v in o.items()]
            return "{\n" + ",\n".join(items) + "\n" + pad + "}" if items else "{}"
        if isinstance(o, list) and o and all(isinstance(x, dict) for x in o):
            items = [f"{pad} {emit(v, level + 1)}" for v in o]
            return "[\n" + ",\n".join(items) + "\n" + pad + "]"
        return inline(o)
    return emit(obj, 0) + "\n"


def sha256sums(files):
    lines = [f"{hashlib.sha256(data).hexdigest()}  {path}" for path, data in sorted(files.items())]
    return ("\n".join(lines) + "\n").encode()


def build():
    CROSS.clear()
    cases = build_cases()
    doc = document(cases)
    text = dumps(doc)
    assert json.loads(text) == json.loads(json.dumps(doc))
    files = {"references.json": text.encode()}
    for fixed in FIXED_FILES:
        p = OUT / fixed
        if p.exists():
            files[fixed] = p.read_bytes()
    files["SHA256SUMS"] = sha256sums({k: v for k, v in files.items() if k != "SHA256SUMS"})
    return files, cases


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    files, cases = build()
    bad = [c for c in CROSS if not c["agrees"]]
    print(f"{len(cases)} cases, {sum(len(c['rows']) for c in cases)} rows")
    print(f"cross-checks: {len(CROSS)} recorded, {len(bad)} disagreeing")
    for c in cases:
        print(c["id"], ", ".join(f"{k} {v['value']:.6g}" for k, v in c["key_values"].items()))
    if args.write:
        for path, data in files.items():
            if path in FIXED_FILES:
                continue
            target = OUT / path
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(data)
        print(f"wrote {len(files) - len([f for f in FIXED_FILES if f in files])} files")
        return 0 if not bad else 1
    failures = [p for p, d in files.items() if not (OUT / p).exists() or (OUT / p).read_bytes() != d]
    committed = {str(p.relative_to(OUT)) for p in OUT.rglob("*") if p.is_file() and "__pycache__" not in p.parts}
    extra = sorted(committed - set(files))
    if failures or extra or bad:
        print("MISMATCH: " + ", ".join(failures + [f"unexpected {e}" for e in extra]))
        return 1
    print(f"verified {len(files)} files byte for byte (SHA256SUMS included)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
