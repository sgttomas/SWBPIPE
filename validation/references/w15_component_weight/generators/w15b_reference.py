#!/usr/bin/env python3
"""W-15 slice b independent references: rigid-component weight at its centre of gravity.

Standard-library Python 3 only. This generator imports, reads and runs no
product code and reads no file other than the ones it writes. Every expected
value is derived here in exact rational arithmetic (DEL-03-05 Design
`rigid-component-stiffness-scaling.md` sections 3 and 6):

  W15-B-WRENCH  the nodal wrench of a component weight W whose centre of
                gravity is r, a vector in global X/Y/Z measured from the
                component's node (the owner's R10 convention, 2026-10-24):
                F = W g_hat and M = r x F, for every gravity axis and sign and
                for an offset along each axis separately and in combination;
  W15-B-CANT    a cantilever whose tip member is a component body (a valve at
                the free end, optionally stiffness-scaled) carrying only that
                weight: support reactions by statics, tip and intermediate
                motions by unit-load virtual work, section actions by statics,
                Euler-Bernoulli and Timoshenko.

A member with DEC-045 stiffness scale s has stiffness s K: s E A, s E I, s G J
and s kappa G A.

The loads are given forces, so forces and moments are rational (pi**0) and
motions are rational multiples of 1/pi (A, I and J are rational multiples of
pi). Decimals are printed to 40 significant digits from a 90-digit Machin value
of pi; binary64 values are the correctly rounded nearest doubles, checked by
rounding with lower and upper rational bounds of pi.

An independent second route checks every published value: the wrench by the
general determinant form of r x F with the force as a point load at r, and its
static equivalence (the same reactions about the anchor as the point load); the
cantilever by an exact direct-stiffness solve of the same collinear chain (the
axial, torsion and two bending problems decoupled; Euler-Bernoulli or
Timoshenko element stiffness scaled by s), its nodal motions, support reactions
and element end actions, with global equilibrium of the reactions.

Modes:
  python3 -I w15b_reference.py           check: rebuild every output in memory and
                                          compare byte for byte with the committed
                                          files, SHA256SUMS included
  python3 -I w15b_reference.py --write   write the outputs (intentional changes only)
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
FIXED_FILES = ("README.md", "generators/w15b_reference.py")

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
    """Nearest double to q * pi**k, the same for both rational bounds of pi
    (Python's float(Fraction) rounds correctly)."""
    if k == 0 or q == 0:
        return float(q)
    if k == -1:
        lo, hi = float(q / PI_HI), float(q / PI_LO)
    else:
        raise SystemExit(f"pi power {k}")
    if lo != hi:
        raise SystemExit(f"binary64 rounding of {q}*pi**{k} is not decided by the pi bounds")
    return lo


def decimal40(q, k):
    if q == 0:
        return "0"
    getcontext().prec = 100
    v = Decimal(q.numerator) / Decimal(q.denominator) * (PI_DEC ** k if k else 1)
    return format(v.normalize().__format__(".39e"))


def exact_text(q, k):
    s = str(q)
    if k == 0 or q == 0:
        return s
    return f"{s}/pi"


def record(q, k):
    q = F(q)
    return {"exact": exact_text(q, k), "decimal": decimal40(q, k), "value": binary64(q, k)}


# ----------------------------------------------------------------------------- inputs (invented)
OD, ID = F(2, 10), F(18, 100)  # m
WALL = (OD - ID) / 2  # 0.01 m
E = F(200 * 10 ** 9)  # Pa
NU = F(1, 4)
G = E / (2 * (1 + NU))  # 80 GPa, also the preview route's authored G
assert G == 80 * 10 ** 9
AREA_P = (OD ** 2 - ID ** 2) / 4  # A / pi
INERTIA_P = (OD ** 4 - ID ** 4) / 64  # I / pi
POLAR_P = 2 * INERTIA_P  # J / pi
assert AREA_P == F(19, 10000) and INERTIA_P == F(85975, 10 ** 10)
# Energy-matched shear factor of the annulus (Saint-Venant shear strain energy,
# V^2/(2 kappa G A)); m = ri/ro:
#   kappa = 6 (1+nu)^2 (1+m^2)^2 / [(7+14nu+8nu^2)(1+m^2)^2 + 4 m^2 (5+10nu+4nu^2)]
M_RATIO = ID / OD
KAPPA = (6 * (1 + NU) ** 2 * (1 + M_RATIO ** 2) ** 2
         / ((7 + 14 * NU + 8 * NU ** 2) * (1 + M_RATIO ** 2) ** 2 + 4 * M_RATIO ** 2 * (5 + 10 * NU + 4 * NU ** 2)))
assert 0 < KAPPA < 1

WEIGHT = F(600)  # N, the component's user-entered weight (invented)
# Centre-of-gravity offsets from the component node, global X/Y/Z (m), each
# exact in binary64. "XYZ" is the combination; the others one axis each.
OFFSETS = {
    "X": (F(-3, 4), F(0), F(0)),
    "Y": (F(0), F(1, 8), F(0)),
    "Z": (F(0), F(0), F(-1, 16)),
    "XYZ": (F(-3, 4), F(1, 8), F(-1, 16)),
}
GRAVITY = [(axis, sign) for axis in "xyz" for sign in (-1, 1)]
AXIS = {"x": 0, "y": 1, "z": 2}

COMPONENTS_F = ("Fx", "Fy", "Fz", "Mx", "My", "Mz")
COMPONENTS_U = ("ux", "uy", "uz", "rx", "ry", "rz")
SECTION_COMPONENTS = ("axial_force", "shear_force_y", "shear_force_z",
                      "torsional_moment", "bending_moment_y", "bending_moment_z")
STATIONS = (F(0), F(1, 2), F(1))
STATION_NAMES = ("0", "1/2", "1")
S_TEXT = {F(1): "1", F(4): "4"}

CROSS = []


def cross(name, a, b):
    CROSS.append({"check": name, "agrees": a == b})
    if a != b:
        print(f"DISAGREES {name}: {a} != {b}")


def gravity_text(axis, sign):
    return f"{'+' if sign > 0 else '-'}{axis.upper()}"


# ----------------------------------------------------------------------------- the wrench
def wrench_closed(weight, r, axis, sign):
    """Route 1: the wrench of F = sign * W along `axis`, tabulated per axis.
    F along x: M = (0, r_z f, -r_y f); along y: M = (-r_z f, 0, r_x f);
    along z: M = (r_y f, -r_x f, 0)."""
    f = sign * weight
    rx, ry, rz = r
    force = [F(0)] * 3
    force[AXIS[axis]] = f
    moment = {"x": [F(0), rz * f, -ry * f], "y": [-rz * f, F(0), rx * f], "z": [ry * f, -rx * f, F(0)]}[axis]
    return force, moment


def cross_product(a, b):
    """Route 2: the general determinant form of a x b."""
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def case_wrench(offset, axis, sign):
    r = OFFSETS[offset]
    label = f"W15-B-WRENCH-{offset}-G{gravity_text(axis, sign)}"
    force, moment = wrench_closed(WEIGHT, r, axis, sign)
    cross(f"{label}: M = r x F (determinant form)", moment, cross_product(list(r), force))
    # Static equivalence: the nodal wrench and the point force at the centre of
    # gravity have the same moment about any point p: (node - p) x F + M equals
    # (node + r - p) x F. Checked about an arbitrary invented point.
    node, p = [F(3), F(0), F(0)], [F(1, 3), F(-2, 7), F(5, 11)]
    arm = [n - q for n, q in zip(node, p)]
    nodal = [a + b for a, b in zip(cross_product(arm, force), moment)]
    point = cross_product([a + b for a, b in zip(arm, r)], force)
    cross(f"{label}: the nodal wrench is statically equivalent to the weight at the centre of gravity", nodal, point)
    rows = []
    for i, comp in enumerate(("Fx", "Fy", "Fz")):
        rows.append(dict(quantity="wrench", component=comp, unit="N", **record(force[i], 0)))
    for i, comp in enumerate(("Mx", "My", "Mz")):
        rows.append(dict(quantity="wrench", component=comp, unit="N*m", **record(moment[i], 0)))
    primitives = [{"part": "force", "axis": axis, "direction": f"global_{axis}", **record(force[AXIS[axis]], 0)}]
    for i, a in enumerate("xyz"):
        if moment[i] != 0:
            primitives.append({"part": "moment", "axis": a, "direction": f"rotation_{a}", **record(moment[i], 0)})
    scales = {"force": dict(unit="N", **record(WEIGHT, 0)),
              "moment": dict(unit="N*m", **record(max(abs(m) for m in moment) or WEIGHT * max(abs(c) for c in r), 0))}
    return {"id": label, "family": "component_weight_wrench", "routes": ["generator"],
            "method": "F = W g_hat along the gravity axis with its sign; M = r x F, r from the component node in global X/Y/Z",
            "inputs": {"weight_N": str(WEIGHT), "center_of_gravity_m": [str(c) for c in r],
                       "gravity_axis": f"global_{axis}", "gravity_sign": sign},
            "primitives": primitives, "zero_scales": scales, "rows": rows}


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


# ----------------------------------------------------------------------------- the cantilever
NODES = [("node:a", F(0)), ("node:b", F(1)), ("node:c", F(3, 2)), ("node:d", F(3))]
BODY = "body:c-d"
L_TIP = F(3)


def members(scale):
    return [("pipe:a-b", "node:a", "node:b", F(1)), ("pipe:b-c", "node:b", "node:c", F(1)),
            (BODY, "node:c", "node:d", scale)]


X = dict(NODES)


def segments(scale):
    return [(X[i], X[j], s) for _, i, j, s in members(scale)]


def integral(segs, upper, antiderivative, stiffness):
    """sum over segments of int_{x0}^{min(x1, upper)} p(x) / (s * stiffness) dx,
    from the exact antiderivative of p."""
    total = F(0)
    for x0, x1, s in segs:
        b = min(x1, upper)
        if b <= x0:
            continue
        total += (antiderivative(b) - antiderivative(x0)) / (s * stiffness)
    return total


def cantilever_closed(scale, force, moment, timoshenko):
    """Route 1. Fixed at x = 0; the tip wrench (force, moment) at x = L. Motions
    are returned times pi. Internal actions on the +x face of the i-side
    portion: N = Fx, V_y = Fy, V_z = Fz, T = Mx,
    M_y(x) = My - (L - x) Fz and M_z(x) = Mz + (L - x) Fy.
    Unit-load virtual work: u_x = Fx int 1/(sEA); theta_x = Mx int 1/(sGJ);
    theta_z = int M_z/(sEI), u_y = int M_z (x_P - x)/(sEI) [+ Fy int 1/(s kappa G A)];
    theta_y = int M_y/(sEI), u_z = -int M_y (x_P - x)/(sEI) [+ Fz int 1/(s kappa G A)]
    (a right-handed rotation about +y turns +x toward -z)."""
    fx, fy, fz = force
    mx, my, mz = moment
    segs = segments(scale)
    ea, gj, ei, kga = E * AREA_P, G * POLAR_P, E * INERTIA_P, KAPPA * G * AREA_P
    L = L_TIP
    nodes = {}
    for nid, xp in NODES:
        # int M_z dx with M_z = Mz + (L - x) Fy: Mz x + Fy (L x - x^2/2)
        rz = integral(segs, xp, lambda x: mz * x + fy * (L * x - x ** 2 / 2), ei)
        # int M_z (xp - x) dx = Mz (xp x - x^2/2) + Fy (L xp x - (L + xp) x^2/2 + x^3/3)
        uy = integral(segs, xp, lambda x, xp=xp: mz * (xp * x - x ** 2 / 2)
                      + fy * (L * xp * x - (L + xp) * x ** 2 / 2 + x ** 3 / 3), ei)
        ry = integral(segs, xp, lambda x: my * x - fz * (L * x - x ** 2 / 2), ei)
        uz = -integral(segs, xp, lambda x, xp=xp: my * (xp * x - x ** 2 / 2)
                       - fz * (L * xp * x - (L + xp) * x ** 2 / 2 + x ** 3 / 3), ei)
        if timoshenko:
            uy += fy * integral(segs, xp, lambda x: x, kga)
            uz += fz * integral(segs, xp, lambda x: x, kga)
        nodes[nid] = {"ux": fx * integral(segs, xp, lambda x: x, ea), "uy": uy, "uz": uz,
                      "rx": mx * integral(segs, xp, lambda x: x, gj), "ry": ry, "rz": rz}
    sections = {}
    for mid, i, j, _ in members(scale):
        rows = []
        for st in STATIONS:
            x = X[i] + st * (X[j] - X[i])
            rows.append({"axial_force": fx, "shear_force_y": fy, "shear_force_z": fz, "torsional_moment": mx,
                         "bending_moment_y": my - (L - x) * fz, "bending_moment_z": mz + (L - x) * fy})
        sections[mid] = rows
    # Statics of the whole: the support's action on the structure balances the
    # tip wrench about the anchor, d = (L, 0, 0): d x F = (0, -L Fz, L Fy).
    reaction = {"Fx": -fx, "Fy": -fy, "Fz": -fz, "Mx": -mx, "My": -(my - L * fz), "Mz": -(mz + L * fy)}
    return nodes, sections, reaction


def element_matrices(length, s, timoshenko):
    """(axial, torsion, bending) local stiffness of a scaled member, / pi. The
    bending matrix acts on (v_i, theta_i, v_j, theta_j) of either plane."""
    L = length
    ea, gj, ei = s * E * AREA_P, s * G * POLAR_P, s * E * INERTIA_P
    axial = [[ea / L, -ea / L], [-ea / L, ea / L]]
    torsion = [[gj / L, -gj / L], [-gj / L, gj / L]]
    phi = 12 * ei / (s * KAPPA * G * AREA_P * L ** 2) if timoshenko else F(0)
    c = ei / ((1 + phi) * L ** 3)
    bending = [[12 * c, 6 * L * c, -12 * c, 6 * L * c],
               [6 * L * c, (4 + phi) * L ** 2 * c, -6 * L * c, (2 - phi) * L ** 2 * c],
               [-12 * c, -6 * L * c, 12 * c, -6 * L * c],
               [6 * L * c, (2 - phi) * L ** 2 * c, -6 * L * c, (4 + phi) * L ** 2 * c]]
    return axial, torsion, bending


def assemble_solve(mems, dofs_per_node, element_k, fixed, loads):
    """Exact solve on the chain: element_k(member) -> local matrix on its
    2*dofs_per_node DOFs; loads {(node, d): value}; fixed {(node, d)}. Returns
    motions, reactions and the end actions of every member."""
    ids = [nid for nid, _ in NODES]
    index = {nid: dofs_per_node * k for k, nid in enumerate(ids)}
    n = dofs_per_node * len(ids)
    K = [[F(0)] * n for _ in range(n)]
    P = [F(0)] * n
    data = []
    for m in mems:
        k = element_k(m)
        dofs = [index[m[1]] + d for d in range(dofs_per_node)] + [index[m[2]] + d for d in range(dofs_per_node)]
        for a in range(len(dofs)):
            for b in range(len(dofs)):
                K[dofs[a]][dofs[b]] += k[a][b]
        data.append((m[0], dofs, k))
    for (nid, d), v in loads.items():
        P[index[nid] + d] += v
    restrained = sorted(index[nid] + d for nid, d in fixed)
    free = [d for d in range(n) if d not in restrained]
    U = [F(0)] * n
    for f, v in zip(free, solve([[K[f][g] for g in free] for f in free], [P[f] for f in free])):
        U[f] = v
    ends, total = {}, [F(0)] * n
    for mid, dofs, k in data:
        fl = matvec(k, [U[d] for d in dofs])
        ends[mid] = fl
        for a in range(len(dofs)):
            total[dofs[a]] += fl[a]
    for f in free:
        nid = ids[f // dofs_per_node]
        cross(f"free-DOF equilibrium {nid} {f % dofs_per_node}", total[f], loads.get((nid, f % dofs_per_node), F(0)))
    motions = {nid: [U[index[nid] + d] for d in range(dofs_per_node)] for nid in ids}
    reactions = {(nid, d): total[index[nid] + d] - loads.get((nid, d), F(0)) for nid, d in fixed}
    return motions, reactions, ends


def cantilever_check(label, scale, force, moment, timoshenko, nodes, sections, reaction):
    """Route 2: the four decoupled direct-stiffness problems. The x-z plane
    maps onto the x-y bending matrix through (u_z, -theta_y) <-> (u_y, theta_z),
    with the loads (Fz, -My) and the actions V_z = V~_y, M_y = -M~_z."""
    fx, fy, fz = force
    mx, my, mz = moment
    mems = members(scale)
    km = lambda m, k: element_matrices(X[m[2]] - X[m[1]], m[3], timoshenko)[k]
    for name, k, load, comp, rcomp, skey in (("axial", 0, fx, "ux", "Fx", "axial_force"),
                                             ("torsion", 1, mx, "rx", "Mx", "torsional_moment")):
        motions, reactions, ends = assemble_solve(mems, 1, lambda m, k=k: km(m, k), {("node:a", 0)}, {("node:d", 0): load})
        for nid, _ in NODES:
            cross(f"{label}: {nid} {comp} direct stiffness", motions[nid][0], nodes[nid][comp])
        cross(f"{label}: support {rcomp} direct stiffness", reactions[("node:a", 0)], reaction[rcomp])
        for mid, fl in ends.items():
            cross(f"{label}: {mid} {skey} end_i", fl[0], -sections[mid][0][skey])
            cross(f"{label}: {mid} {skey} end_j", fl[1], sections[mid][-1][skey])
    for plane, (pf, pm), (ucomp, rcomp, rsign), (rf, rm, msign), (vkey, mkey) in (
            ("x-y", (fy, mz), ("uy", "rz", 1), ("Fy", "Mz", 1), ("shear_force_y", "bending_moment_z")),
            ("x-z", (fz, -my), ("uz", "ry", -1), ("Fz", "My", -1), ("shear_force_z", "bending_moment_y"))):
        motions, reactions, ends = assemble_solve(mems, 2, lambda m: km(m, 2), {("node:a", 0), ("node:a", 1)},
                                                  {("node:d", 0): pf, ("node:d", 1): pm})
        for nid, _ in NODES:
            cross(f"{label}: {nid} {ucomp} direct stiffness ({plane})", motions[nid][0], nodes[nid][ucomp])
            cross(f"{label}: {nid} {rcomp} direct stiffness ({plane})", rsign * motions[nid][1], nodes[nid][rcomp])
        cross(f"{label}: support {rf} direct stiffness", reactions[("node:a", 0)], reaction[rf])
        cross(f"{label}: support {rm} direct stiffness", msign * reactions[("node:a", 1)], reaction[rm])
        for mid, fl in ends.items():
            cross(f"{label}: {mid} {vkey} end_i", fl[0], -sections[mid][0][vkey])
            cross(f"{label}: {mid} {mkey} end_i", msign * fl[1], -sections[mid][0][mkey])
            cross(f"{label}: {mid} {vkey} end_j", fl[2], sections[mid][-1][vkey])
            cross(f"{label}: {mid} {mkey} end_j", msign * fl[3], sections[mid][-1][mkey])
    # Global equilibrium of the support action and the tip wrench about the anchor.
    applied_moment = [a + b for a, b in zip(moment, cross_product([L_TIP, F(0), F(0)], list(force)))]
    cross(f"{label}: reactions balance the tip force", [reaction[c] for c in ("Fx", "Fy", "Fz")], [-c for c in force])
    cross(f"{label}: reactions balance the tip wrench about the anchor", [reaction[c] for c in ("Mx", "My", "Mz")],
          [-c for c in applied_moment])


def case_cantilever(offset, axis, sign, scale, timoshenko):
    r = OFFSETS[offset]
    label = (f"W15-B-CANT-{offset}-G{gravity_text(axis, sign)}-S{S_TEXT[scale]}-"
             f"{'TIM' if timoshenko else 'EB'}")
    force, moment = wrench_closed(WEIGHT, r, axis, sign)
    nodes, sections, reaction = cantilever_closed(scale, force, moment, timoshenko)
    cantilever_check(label, scale, force, moment, timoshenko, nodes, sections, reaction)
    rows, scales = publish({"support:a": reaction}, {nid: nodes[nid] for nid in ("node:b", "node:c", "node:d")}, sections)
    key = {f"tip_{c}": record(nodes["node:d"][c], -1) for c in COMPONENTS_U}
    key.update({f"reaction_{c}": record(reaction[c], 0) for c in COMPONENTS_F})
    inputs = {"geometry": {"nodes": {nid: [str(x), "0", "0"] for nid, x in NODES},
                           "members": {m[0]: {"from": m[1], "to": m[2], "y_reference": [0, 1, 0]} for m in members(scale)},
                           "supports": {"support:a": {"node": "node:a", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"]}}},
              "body": {"member": BODY, "component_kind": "valve", "component_node": "node:d",
                       "stiffness_scale": S_TEXT[scale] if scale != 1 else None,
                       "weight_N": str(WEIGHT), "center_of_gravity_m": [str(c) for c in r]},
              "self_weight": {"selected_members": [BODY], "gravity_axis": f"global_{axis}", "gravity_sign": sign},
              "beam_theory": "timoshenko_annulus_energy_v1" if timoshenko else "euler_bernoulli"}
    return {"id": label, "family": "component_weight_cantilever",
            "routes": ["exact"] if timoshenko else ["preview", "exact"],
            "method": "the component weight's nodal wrench at the free end of a cantilever; statics for reactions and "
                      "section actions; unit-load virtual work with segment compliances 1/(s EA), 1/(s GJ), 1/(s EI)"
                      + (" and 1/(s kappa G A)" if timoshenko else ""),
            "inputs": inputs, "key_values": key, "zero_scales": scales, "rows": rows}


# ----------------------------------------------------------------------------- publication
def family(unit):
    return {"N": "force", "N*m": "moment", "m": "translation", "rad": "rotation"}[unit]


def publish(reactions, nodes, sections):
    """Rows of a solved case; forces carry pi**0 and motions pi**-1."""
    rows = []
    for sup in sorted(reactions):
        for comp in COMPONENTS_F:
            unit = "N" if comp.startswith("F") else "N*m"
            rows.append(dict(quantity="support_reaction", entity=sup, component=comp, unit=unit,
                             **record(reactions[sup][comp], 0)))
    for nd in sorted(nodes):
        for comp in COMPONENTS_U:
            unit = "m" if comp.startswith("u") else "rad"
            rows.append(dict(quantity="nodal_motion", entity=nd, component=comp, unit=unit, **record(nodes[nd][comp], -1)))
    for pipe in sorted(sections):
        for name, sec in zip(STATION_NAMES, sections[pipe]):
            for comp in SECTION_COMPONENTS:
                unit = "N" if comp in ("axial_force", "shear_force_y", "shear_force_z") else "N*m"
                rows.append(dict(quantity="section_action", entity=pipe, station=name, component=comp,
                                 unit=unit, **record(sec[comp], 0)))
    scales = {}
    for fam, unit, k in (("force", "N", 0), ("moment", "N*m", 0), ("translation", "m", -1), ("rotation", "rad", -1)):
        mags = [abs(F(r["exact"].replace("/pi", ""))) for r in rows if family(r["unit"]) == fam]
        top = max(mags)
        assert top > 0, fam
        scales[fam] = dict(unit=unit, **record(top, k))
    return rows, scales


def build_cases():
    cases = [case_wrench(offset, axis, sign) for offset in OFFSETS for axis, sign in GRAVITY]
    # Every gravity axis and sign with the combined offset, a scaled body (s = 4).
    for axis, sign in GRAVITY:
        cases.append(case_cantilever("XYZ", axis, sign, F(4), False))
    # Gravity -Y with each offset axis separately.
    for offset in ("X", "Y", "Z"):
        cases.append(case_cantilever(offset, "y", -1, F(4), False))
    # Timoshenko on the exact route, two gravity axes.
    for axis in ("y", "z"):
        cases.append(case_cantilever("XYZ", axis, -1, F(4), True))
    # No stiffness scale: the weight is generated for an unscaled body (s = 1).
    cases.append(case_cantilever("XYZ", "y", -1, F(1), False))
    return cases


def document(cases):
    return {
        "document": "openpipestress.w15_component_weight_references",
        "status": "candidate frozen references; derived and cross-checked by the author TASK, not yet independently refuted",
        "units": {"force": "N", "moment": "N*m", "translation": "m", "rotation": "rad"},
        "theory": "the weight W of a component at its centre of gravity r (global X/Y/Z from the component node, the "
                  "owner's R10 convention of 2026-10-24) is the nodal wrench F = W g_hat, M = r x F; small-displacement "
                  "linear-elastic space frame, Euler-Bernoulli or Timoshenko with the energy-matched annular shear "
                  "factor; a member with DEC-045 stiffness scale s has stiffness s K; annular section, Iy = Iz = I, J = 2I",
        "section": {"outside_diameter_m": str(OD), "inside_diameter_m": str(ID), "wall_thickness_m": str(WALL),
                    "elastic_modulus_Pa": str(E), "poisson_ratio": str(NU), "shear_modulus_Pa": str(G),
                    "area_m2": f"{AREA_P}*pi", "second_moment_m4": f"{INERTIA_P}*pi", "polar_moment_m4": f"{POLAR_P}*pi",
                    "shear_factor_kappa": str(KAPPA)},
        "conventions": {
            "wrench": "global force and moment components at the component node; a primitive is emitted for the force "
                      "along the gravity axis and for each nonzero moment component",
            "support_reaction": "global action of the support on the structure at its node (moments about that node)",
            "nodal_motion": "global translation and right-handed rotation",
            "section_action": "element-local action on the +x face of the i-side portion at station s/L (local x from i to j); "
                              "a node-on-element end action is -section(0) at end_i and +section(1) at end_j",
            "exact": "rational q or q/pi",
        },
        "criterion": "|observed - expected| <= 1e-9 * |expected|; an exactly zero expectation uses 1e-9 * zero_scale "
                     "of its family (largest nonzero magnitude of that family in the case; for a wrench, the weight or "
                     "the largest moment, or the weight times the largest offset when every moment is zero)",
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
        if "key_values" in c:
            print(c["id"], ", ".join(f"{k} {v['value']:.6g}" for k, v in c["key_values"].items() if k.startswith("tip_u")))
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
