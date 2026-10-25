#!/usr/bin/env python3
"""W-15 independent references: DEC-045 rigid-component stiffness scaling.

Standard-library Python 3 only. This generator imports, reads and runs no
product code and reads no file other than the ones it writes. Every expected
value is derived here in exact rational arithmetic from textbook closed forms
(DEL-03-05 Design `rigid-component-stiffness-scaling.md` section 6):

  W15-R  rigid-link cantilever: a scaled body between two pipe members, a tip
         force (axial and transverse) and a tip torque; unit-load virtual work
         with compliance sums, Euler-Bernoulli and Timoshenko, and the s -> oo
         rigid-link limit (body compliance removed, moment arms kept);
  W15-T  thermal and fit series: a pipe and a scaled body in series between two
         anchors under free strains, N = -sum(eps_i L_i) / sum(L_i / (s_i E A)),
         tension positive; and a free body growing eps L;
  W15-P  closed-end pressure cantilever: the wall force N_w = p A_i on both
         members, and the elastic axial strain (1 - 2 nu) p A_i / (s E A_n),
         s times smaller on the body.

A member with scale s has stiffness s K: s E A, s E I, s G J and s kappa G A.

Every result is q * pi**k with q rational. In W15-R the loads are given forces,
so forces and moments carry k = 0 and motions k = -1 (A, I, J are rational
multiples of pi). In W15-T and W15-P the loads are formed from E A or p A_i, so
forces and moments carry k = 1 and motions k = 0. Values are held as the
rational q with the family's power of pi. Decimals are printed to 40
significant digits from a 90-digit Machin value of pi; binary64 values are the
correctly rounded nearest doubles, checked by rounding with lower and upper
rational bounds of pi.

An independent second route checks every published value: an exact
direct-stiffness solve of the same collinear chain (axial, torsion and in-plane
bending decoupled; Euler-Bernoulli or Timoshenko element stiffness scaled by
s; held-member eigen end actions), its nodal motions, support reactions and
element end actions. The out-of-plane problem of each case is unloaded and
stable, so its components are exactly zero.

Modes:
  python3 -I w15_reference.py           check: rebuild every output in memory and
                                         compare byte for byte with the committed
                                         files, SHA256SUMS included
  python3 -I w15_reference.py --write   write the outputs (intentional changes only)
"""
import argparse
import hashlib
import json
import struct
import sys
from decimal import Decimal, getcontext
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE.parent
FIXED_FILES = ("README.md", "generators/w15_reference.py")

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


def _nearest(q):
    """Correctly rounded binary64 of a rational (Python's float(Fraction) rounds correctly)."""
    return float(q)


def binary64(q, k):
    """Nearest double to q * pi**k, the same for both rational bounds of pi."""
    if k == 0 or q == 0:
        return _nearest(q)
    if k == 1:
        lo, hi = _nearest(q * PI_LO), _nearest(q * PI_HI)
    elif k == -1:
        lo, hi = _nearest(q / PI_HI), _nearest(q / PI_LO)
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
    return f"{s}*pi" if k == 1 else f"{s}/pi"


def record(q, k):
    q = F(q)
    return {"exact": exact_text(q, k), "decimal": decimal40(q, k), "value": binary64(q, k)}


def double(text):
    """The exact rational of the binary64 a JSON number text parses to."""
    return F(struct.unpack("<d", struct.pack("<d", float(text)))[0])


# ----------------------------------------------------------------------------- inputs (invented)
OD, ID = F(2, 10), F(18, 100)  # m
RO, RI = OD / 2, ID / 2
WALL = (OD - ID) / 2  # 0.01 m
E = F(200 * 10 ** 9)  # Pa
NU = F(1, 4)  # exact route; G = E / (2 (1 + nu)) exactly
G = E / (2 * (1 + NU))  # 80 GPa, also the preview route's authored G
assert G == 80 * 10 ** 9
AREA_P = (OD ** 2 - ID ** 2) / 4  # A / pi
INERTIA_P = (OD ** 4 - ID ** 4) / 64  # I / pi
POLAR_P = 2 * INERTIA_P  # J / pi
BORE_P = RI ** 2  # A_i / pi (no mill tolerance, no corrosion allowance)
assert AREA_P == F(19, 10000) and INERTIA_P == F(85975, 10 ** 10) and BORE_P == F(81, 10000)
# Energy-matched shear factor of the annulus (Saint-Venant shear strain energy,
# V^2/(2 kappa G A)); m = ri/ro:
#   kappa = 6 (1+nu)^2 (1+m^2)^2 / [(7+14nu+8nu^2)(1+m^2)^2 + 4 m^2 (5+10nu+4nu^2)]
M_RATIO = RI / RO
KAPPA = (6 * (1 + NU) ** 2 * (1 + M_RATIO ** 2) ** 2
         / ((7 + 14 * NU + 8 * NU ** 2) * (1 + M_RATIO ** 2) ** 2 + 4 * M_RATIO ** 2 * (5 + 10 * NU + 4 * NU ** 2)))
assert 0 < KAPPA < 1

COMPONENTS_F = ("Fx", "Fy", "Fz", "Mx", "My", "Mz")
COMPONENTS_U = ("ux", "uy", "uz", "rx", "ry", "rz")
SECTION_COMPONENTS = ("axial_force", "shear_force_y", "shear_force_z",
                      "torsional_moment", "bending_moment_y", "bending_moment_z")
STATIONS = (F(0), F(1, 2), F(1))
STATION_NAMES = ("0", "1/2", "1")
S_TEXT = {F(1): "1", F(1, 4): "1/4", F(4): "4", **{F(10 ** k): str(10 ** k) for k in range(3, 9)}}

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


# ----------------------------------------------------------------------------- the collinear chain
class Chain:
    """Nodes on +X, members between consecutive nodes. A member is
    (id, i, j, length, scale); local x = +X, local y = +Y, local z = +Z."""

    def __init__(self, nodes, members):
        self.nodes = nodes  # [(id, x)]
        self.members = members  # [(id, i, j, scale)]
        self.x = dict(nodes)

    def length(self, member):
        _, i, j, _ = member
        return self.x[j] - self.x[i]

    def segments(self):
        return [(self.x[i], self.x[j], s) for _, i, j, s in self.members]


def integral(segments, upper, poly, stiffness):
    """sum over segments of int_{x0}^{min(x1, upper)} poly(x) / (s * stiffness) dx,
    poly given by its antiderivative evaluated exactly."""
    total = F(0)
    for x0, x1, s in segments:
        b = min(x1, upper)
        if b <= x0:
            continue
        total += (poly(b) - poly(x0)) / (s * stiffness)
    return total


# ----------------------------------------------------------------------------- route 1: closed forms
def cantilever_closed(chain, length, fx, fy, mx, timoshenko, rigid_body=None):
    """Tip loads at x = length on a cantilever fixed at x = 0. Motions / pi**-1
    (i.e. times pi). rigid_body: a member id whose compliance is removed."""
    segs = [(x0, x1, s) for (x0, x1, s), m in zip(chain.segments(), chain.members) if m[0] != rigid_body]
    ea, gj, ei, kga = E * AREA_P, G * POLAR_P, E * INERTIA_P, KAPPA * G * AREA_P
    nodes = {}
    for nid, xp in chain.nodes:
        ux = fx * integral(segs, xp, lambda x: x, ea)
        rx = mx * integral(segs, xp, lambda x: x, gj)
        # int (L - x)(xp - x) dx = L xp x - (L + xp) x^2/2 + x^3/3
        uy = fy * integral(segs, xp, lambda x, xp=xp: length * xp * x - (length + xp) * x ** 2 / 2 + x ** 3 / 3, ei)
        if timoshenko:
            uy += fy * integral(segs, xp, lambda x: x, kga)
        rz = fy * integral(segs, xp, lambda x: length * x - x ** 2 / 2, ei)
        nodes[nid] = {"ux": ux, "uy": uy, "rx": rx, "rz": rz}
    sections = {}
    for member in chain.members:
        mid, i, _, _ = member
        rows = []
        for st in STATIONS:
            x = chain.x[i] + st * chain.length(member)
            rows.append({"axial_force": fx, "shear_force_y": fy, "shear_force_z": F(0), "torsional_moment": mx,
                         "bending_moment_y": F(0), "bending_moment_z": (length - x) * fy})
        sections[mid] = rows
    reactions = {"Fx": -fx, "Fy": -fy, "Mx": -mx, "Mz": -length * fy}
    return nodes, sections, reactions


def series_closed(chain, eps, area_e):
    """Both chain ends fixed, member free strains eps[id]; area_e = E_basis A / pi.
    N = -sum(eps L) / sum(L / (s E A)); motions u(x) by accumulation."""
    num = sum(eps[m[0]] * chain.length(m) for m in chain.members)
    den = sum(chain.length(m) / (m[3] * area_e) for m in chain.members)
    n = -num / den
    u, nodes = F(0), {chain.nodes[0][0]: {"ux": F(0)}}
    for m in chain.members:
        u += eps[m[0]] * chain.length(m) + n * chain.length(m) / (m[3] * area_e)
        nodes[m[2]] = {"ux": u}
    cross("series: the far anchor returns to rest", u, F(0))
    return n, nodes


def free_closed(chain, eps):
    u, nodes = F(0), {chain.nodes[0][0]: {"ux": F(0)}}
    for m in chain.members:
        u += eps[m[0]] * chain.length(m)
        nodes[m[2]] = {"ux": u}
    return nodes


def pressure_closed(chain, pai, area_e):
    """Closed-end cantilever region over the whole chain: N_w = p A_i; the elastic
    axial strain (1 - 2 nu) p A_i / (s E A_n) on each member."""
    u, nodes = F(0), {chain.nodes[0][0]: {"ux": F(0)}}
    for m in chain.members:
        u += (1 - 2 * NU) * pai * chain.length(m) / (m[3] * area_e)
        nodes[m[2]] = {"ux": u}
    return nodes


# ----------------------------------------------------------------------------- route 2: direct stiffness
def element_matrices(length, s, timoshenko):
    """(axial, torsion, bending) local stiffness of a scaled member, / pi."""
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


def assemble_solve(chain, dofs_per_node, element_k, fixed, loads, fef):
    """Generic exact solve on the chain. element_k(member) -> local matrix on the
    member's 2*dofs_per_node DOFs; fef(member) -> held end actions; loads
    {(node, d): value}; fixed {(node, d)}. Returns motions, reactions and the
    end actions of every member (node-on-element, local)."""
    ids = [nid for nid, _ in chain.nodes]
    index = {nid: dofs_per_node * k for k, nid in enumerate(ids)}
    n = dofs_per_node * len(ids)
    K = [[F(0)] * n for _ in range(n)]
    P = [F(0)] * n
    data = []
    for m in chain.members:
        k = element_k(m)
        f = fef(m)
        dofs = [index[m[1]] + d for d in range(dofs_per_node)] + [index[m[2]] + d for d in range(dofs_per_node)]
        for a in range(len(dofs)):
            P[dofs[a]] -= f[a]
            for b in range(len(dofs)):
                K[dofs[a]][dofs[b]] += k[a][b]
        data.append((m[0], dofs, k, f))
    for (nid, d), v in loads.items():
        P[index[nid] + d] += v
    restrained = sorted(index[nid] + d for nid, d in fixed)
    free = [d for d in range(n) if d not in restrained]
    U = [F(0)] * n
    sol = solve([[K[f][g] for g in free] for f in free], [P[f] for f in free]) if free else []
    for f, v in zip(free, sol):
        U[f] = v
    ends, total = {}, [F(0)] * n
    for mid, dofs, k, f in data:
        fl = [x + y for x, y in zip(matvec(k, [U[d] for d in dofs]), f)]
        ends[mid] = fl
        for a in range(len(dofs)):
            total[dofs[a]] += fl[a]
    for f in free:
        nid = ids[f // dofs_per_node]
        cross(f"free-DOF equilibrium {nid} {f % dofs_per_node}", total[f], loads.get((nid, f % dofs_per_node), F(0)))
    motions = {nid: [U[index[nid] + d] for d in range(dofs_per_node)] for nid in ids}
    reactions = {(nid, d): total[index[nid] + d] - loads.get((nid, d), F(0)) for nid, d in fixed}
    return motions, reactions, ends


def zero_fef(size):
    return lambda m: [F(0)] * size


# ----------------------------------------------------------------------------- cases
def chain_r(scale):
    return Chain([("node:a", F(0)), ("node:b", F(1)), ("node:c", F(3, 2)), ("node:d", F(3))],
                 [("pipe:a-b", "node:a", "node:b", F(1)), ("body:b-c", "node:b", "node:c", scale),
                  ("pipe:c-d", "node:c", "node:d", F(1))])


def chain_t(scale):
    return Chain([("node:a", F(0)), ("node:b", F(1)), ("node:c", F(3))],
                 [("pipe:a-b", "node:a", "node:b", F(1)), ("body:b-c", "node:b", "node:c", scale)])


FX, FY, MX = F(2000), F(1000), F(500)
R_LENGTH = F(3)


def case_r(scale, timoshenko):
    chain = chain_r(scale)
    label = f"W15-R-S{S_TEXT[scale].replace('/', '_')}-{'TIM' if timoshenko else 'EB'}"
    nodes, sections, reaction = cantilever_closed(chain, R_LENGTH, FX, FY, MX, timoshenko)
    # route 2: axial, torsion and bending chains, fixed at A, loads at D
    for name, d_index, size, loads, comp in (("axial", 0, 1, {("node:d", 0): FX}, "ux"),
                                            ("torsion", 1, 1, {("node:d", 0): MX}, "rx")):
        motions, reactions, ends = assemble_solve(
            chain, 1, lambda m: element_matrices(chain.length(m), m[3], timoshenko)[d_index], {("node:a", 0)}, loads, zero_fef(2))
        for nid, _ in chain.nodes:
            cross(f"{label}: {nid} {comp} direct stiffness", motions[nid][0], nodes[nid][comp])
        cross(f"{label}: support {name} direct stiffness", reactions[("node:a", 0)], reaction["Fx" if name == "axial" else "Mx"])
        key = "axial_force" if name == "axial" else "torsional_moment"
        for mid, fl in ends.items():
            cross(f"{label}: {mid} {key} end_i", fl[0], -sections[mid][0][key])
            cross(f"{label}: {mid} {key} end_j", fl[1], sections[mid][-1][key])
    motions, reactions, ends = assemble_solve(
        chain, 2, lambda m: element_matrices(chain.length(m), m[3], timoshenko)[2], {("node:a", 0), ("node:a", 1)},
        {("node:d", 0): FY}, zero_fef(4))
    for nid, _ in chain.nodes:
        cross(f"{label}: {nid} uy direct stiffness", motions[nid][0], nodes[nid]["uy"])
        cross(f"{label}: {nid} rz direct stiffness", motions[nid][1], nodes[nid]["rz"])
    cross(f"{label}: support Fy direct stiffness", reactions[("node:a", 0)], reaction["Fy"])
    cross(f"{label}: support Mz direct stiffness", reactions[("node:a", 1)], reaction["Mz"])
    for mid, fl in ends.items():
        for slot, comp, row, sign in ((0, "shear_force_y", 0, -1), (1, "bending_moment_z", 0, -1),
                                      (2, "shear_force_y", -1, 1), (3, "bending_moment_z", -1, 1)):
            cross(f"{label}: {mid} {comp} {'end_j' if row else 'end_i'}", fl[slot], sign * sections[mid][row][comp])
    # rigid-link limit: m(s) = m_rigid + c / s with c independent of s
    rigid, _, _ = cantilever_closed(chain, R_LENGTH, FX, FY, MX, timoshenko, rigid_body="body:b-c")
    other, _, _ = cantilever_closed(chain_r(scale * 2), R_LENGTH, FX, FY, MX, timoshenko)
    for comp in ("ux", "uy", "rx", "rz"):
        cross(f"{label}: tip {comp} tends to the rigid link as c/s",
              (nodes["node:d"][comp] - rigid["node:d"][comp]) * scale,
              (other["node:d"][comp] - rigid["node:d"][comp]) * scale * 2)
    reactions_out = {"support:a": reaction}
    rows, scales = publish(reactions_out, {nid: nodes[nid] for nid in ("node:b", "node:c", "node:d")}, sections, 0, -1)
    key = {f"tip_{c}": record(nodes["node:d"][c], -1) for c in ("ux", "uy", "rx", "rz")}
    key.update({f"rigid_link_tip_{c}": record(rigid["node:d"][c], -1) for c in ("ux", "uy", "rx", "rz")})
    inputs = {"geometry": geometry(chain, {"support:a": "node:a"}), "body": {"member": "body:b-c", "component_kind": "valve",
              "component_node": "node:b", "stiffness_scale": S_TEXT[scale]},
              "loads": {"node": "node:d", "Fx_N": str(FX), "Fy_N": str(FY), "Mx_N_m": str(MX)},
              "beam_theory": "timoshenko_annulus_energy_v1" if timoshenko else "euler_bernoulli"}
    routes = ["exact"] if timoshenko else ["preview", "exact"]
    return {"id": label, "family": "rigid_link_cantilever", "routes": routes,
            "method": "unit-load virtual work over the cantilever with segment compliances 1/(s EA), 1/(s GJ), 1/(s EI)"
                      + (" and 1/(s kappa G A)" if timoshenko else "") + "; statics for section actions and reactions; "
                      "rigid-link limit with the body's compliance removed",
            "inputs": inputs, "key_values": key, "zero_scales": scales, "rows": rows}


def geometry(chain, supports):
    return {"nodes": {nid: [str(x), "0", "0"] for nid, x in chain.nodes},
            "members": {m[0]: {"from": m[1], "to": m[2], "y_reference": [0, 1, 0], "stiffness_scale": S_TEXT[m[3]]} for m in chain.members},
            "supports": {sid: {"node": node, "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"]} for sid, node in supports.items()}}


def series_check(label, chain, eps, area_e, n, nodes, fixed_both):
    fef = lambda m: [m[3] * area_e * eps[m[0]], -m[3] * area_e * eps[m[0]]]
    fixed = {(chain.nodes[0][0], 0)} | ({(chain.nodes[-1][0], 0)} if fixed_both else set())
    motions, reactions, ends = assemble_solve(
        chain, 1, lambda m: [[m[3] * area_e / chain.length(m), -m[3] * area_e / chain.length(m)],
                             [-m[3] * area_e / chain.length(m), m[3] * area_e / chain.length(m)]], fixed, {}, fef)
    for nid, _ in chain.nodes:
        cross(f"{label}: {nid} ux direct stiffness", motions[nid][0], nodes[nid]["ux"])
    for mid, fl in ends.items():
        cross(f"{label}: {mid} axial end_i", fl[0], -n)
        cross(f"{label}: {mid} axial end_j", fl[1], n)
    return reactions


THERMAL_BASES = {
    # basis: (E Pa, alpha per K or None, delta T K or None, description)
    "PF-MAT": (E, F(12, 10 ** 6), F(40), "preview route, base material E and alpha, element thermal load"),
    "PF-NAMED": (F(190 * 10 ** 9), F(13, 10 ** 6), F(40), "preview route, named temperature point E and alpha"),
    "PF-INTERP": (E + (F(180 * 10 ** 9) - E) * F(125, 200), F(12, 10 ** 6) + F(2, 10 ** 6) * F(125, 200), F(40),
                  "preview route, E and alpha interpolated linearly at 425 K between 300 K and 500 K points"),
}
T_STRAIN = F(1, 2 ** 11)  # explicit interval strain on the exact route, binary64 exact
FIT = F(-1, 2 ** 12)
CANCEL_TEXT = repr(float(-T_STRAIN / (1 + T_STRAIN)))  # the authored fit strain of the cancellation case


def eigenstrain(thermal, fit):
    """The load/reference-state composition lambda_fit * lambda_thermal - 1."""
    return fit + thermal + fit * thermal


def case_t(tag, scale):
    chain = chain_t(scale)
    label = f"W15-T-{tag}-S{S_TEXT[scale].replace('/', '_')}"
    inputs = {"geometry": geometry(chain, {"support:a": "node:a", "support:c": "node:c"}),
              "body": {"member": "body:b-c", "component_kind": "valve", "component_node": "node:b", "stiffness_scale": S_TEXT[scale]}}
    if tag in THERMAL_BASES:
        e_basis, alpha, dt, text = THERMAL_BASES[tag]
        eps = {m[0]: alpha * dt for m in chain.members}
        inputs["thermal"] = {"basis": text, "E_Pa": str(e_basis), "alpha_per_K": str(alpha), "delta_T_K": str(dt),
                             "applied_to": [m[0] for m in chain.members]}
        routes = ["preview"]
    else:
        e_basis = {"EX-MAT-THERMAL": E, "EX-NAMED-FIT": F(190 * 10 ** 9), "EX-INTERP-FITONLY": E + (F(180 * 10 ** 9) - E) * F(125, 200),
                   "EX-CANCEL": E}[tag]
        thermal = {"EX-MAT-THERMAL": T_STRAIN, "EX-NAMED-FIT": T_STRAIN, "EX-INTERP-FITONLY": F(0), "EX-CANCEL": T_STRAIN}[tag]
        fits = {"EX-MAT-THERMAL": {}, "EX-NAMED-FIT": {"body:b-c": FIT}, "EX-INTERP-FITONLY": {"body:b-c": -FIT},
                "EX-CANCEL": {m[0]: double(CANCEL_TEXT) for m in chain.members}}[tag]
        eps = {m[0]: eigenstrain(thermal, fits.get(m[0], F(0))) for m in chain.members}
        inputs["load_state"] = {"material_selection": {"EX-MAT-THERMAL": "explicit_base_properties", "EX-NAMED-FIT": "exact_point",
                                                       "EX-INTERP-FITONLY": "temperature_interpolation at 425 K", "EX-CANCEL": "explicit_base_properties"}[tag],
                                "E_Pa": str(e_basis), "nu": str(NU),
                                "explicit_interval_strain": str(thermal), "fit_strain": {k: (CANCEL_TEXT if tag == "EX-CANCEL" else str(v)) for k, v in fits.items()},
                                "eigenstrain": {k: str(v) for k, v in eps.items()},
                                "composition": "fit + thermal + fit * thermal (lambda_fit * lambda_thermal - 1)"}
        routes = ["exact"]
    area_e = e_basis * AREA_P
    n, nodes = series_closed(chain, eps, area_e)
    reactions = series_check(label, chain, eps, area_e, n, nodes, True)
    cross(f"{label}: support a direct stiffness", reactions[("node:a", 0)], -n)
    cross(f"{label}: support c direct stiffness", reactions[("node:c", 0)], n)
    sections = {m[0]: [{"axial_force": n, "shear_force_y": F(0), "shear_force_z": F(0), "torsional_moment": F(0),
                        "bending_moment_y": F(0), "bending_moment_z": F(0)} for _ in STATIONS] for m in chain.members}
    rows, scales = publish({"support:a": {"Fx": -n}, "support:c": {"Fx": n}}, {"node:b": nodes["node:b"]}, sections, 1, 0)
    key = {"N_N": record(n, 1), "joint_ux_m": record(nodes["node:b"]["ux"], 0)}
    return {"id": label, "family": "thermal_fit_series", "routes": routes,
            "method": "series compatibility: sum of free plus elastic elongations is zero, N = -sum(eps_i L_i) / sum(L_i/(s_i E A))",
            "inputs": inputs, "key_values": key, "zero_scales": scales, "rows": rows}


def case_free(scale):
    chain = chain_t(scale)
    label = f"W15-T-FREE-S{S_TEXT[scale]}"
    eps = {m[0]: F(12, 10 ** 6) * F(40) for m in chain.members}
    nodes = free_closed(chain, eps)
    reactions = series_check(label, chain, eps, E * AREA_P, F(0), nodes, False)
    cross(f"{label}: support a direct stiffness", reactions[("node:a", 0)], F(0))
    sections = {m[0]: [{c: F(0) for c in SECTION_COMPONENTS} for _ in STATIONS] for m in chain.members}
    held = scale * E * AREA_P * eps["body:b-c"]  # the body's held force, the force zero scale
    rows, scales = publish({"support:a": {}}, {"node:b": nodes["node:b"], "node:c": nodes["node:c"]}, sections, 1, 0,
                           fallback={"force": held, "moment": held})
    inputs = {"geometry": geometry(chain, {"support:a": "node:a"}),
              "body": {"member": "body:b-c", "component_kind": "valve", "component_node": "node:b", "stiffness_scale": S_TEXT[scale]},
              "thermal": {"basis": THERMAL_BASES["PF-MAT"][3], "E_Pa": str(E), "alpha_per_K": "3/250000", "delta_T_K": "40",
                          "applied_to": [m[0] for m in chain.members]}}
    return {"id": label, "family": "free_growth", "routes": ["preview"],
            "method": "a free scaled body grows eps L whatever its scale; no force arises",
            "inputs": inputs, "key_values": {"tip_ux_m": record(nodes["node:c"]["ux"], 0)}, "zero_scales": scales, "rows": rows}


P_PRESSURE = F(2 * 10 ** 6)  # Pa


def case_p(scale):
    chain = chain_t(scale)
    label = f"W15-P-S{S_TEXT[scale]}"
    pai = P_PRESSURE * BORE_P  # p A_i / pi
    area_e = E * AREA_P
    nodes = pressure_closed(chain, pai, area_e)
    # route 2: caps +pA_i at C (and -pA_i at the anchored A), the Poisson pair
    # -+2 nu p A_i as held end actions; N_w = N_m + 2 nu p A_i
    eps_p = {m[0]: -2 * NU * pai / (m[3] * area_e) for m in chain.members}
    fef = lambda m: [m[3] * area_e * eps_p[m[0]], -m[3] * area_e * eps_p[m[0]]]
    motions, reactions, ends = assemble_solve(
        chain, 1, lambda m: [[m[3] * area_e / chain.length(m), -m[3] * area_e / chain.length(m)],
                             [-m[3] * area_e / chain.length(m), m[3] * area_e / chain.length(m)]],
        {("node:a", 0)}, {("node:c", 0): pai, ("node:a", 0): -pai}, fef)
    for nid, _ in chain.nodes:
        cross(f"{label}: {nid} ux direct stiffness", motions[nid][0], nodes[nid]["ux"])
    cross(f"{label}: closed vessel, no support reaction", reactions[("node:a", 0)], F(0))
    walls = {}
    for m in chain.members:
        fl = ends[m[0]]
        # fl is the total end action K u + held; the mechanical N_m = K u is fl minus the held Poisson action.
        mechanical = fl[1] - fef(m)[1]
        walls[m[0]] = mechanical + 2 * NU * pai
        cross(f"{label}: {m[0]} mechanical N_m = (1 - 2 nu) p A_i", mechanical, (1 - 2 * NU) * pai)
        cross(f"{label}: {m[0]} wall force N_w = N_m + 2 nu p A_i = p A_i", walls[m[0]], pai)
        cross(f"{label}: {m[0]} end actions balance", fl[0], -fl[1])
    sections = {}
    rows, scales = publish({"support:a": {}}, {"node:b": nodes["node:b"], "node:c": nodes["node:c"]}, sections, 1, 0,
                           fallback={"force": pai, "moment": pai})
    for mid in sorted(walls):
        rows.append(dict(quantity="wall_axial_force", entity=mid, component="wall_axial_force", unit="N", **record(walls[mid], 1)))
    inputs = {"geometry": geometry(chain, {"support:a": "node:a"}),
              "body": {"member": "body:b-c", "component_kind": "valve", "component_node": "node:b", "stiffness_scale": S_TEXT[scale]},
              "pressure": {"region_members": [m[0] for m in chain.members], "terminals": ["node:a", "node:c"],
                           "closure_transfer": "transfers_to_wall", "pressure_Pa": str(P_PRESSURE), "bore_radius_m": str(RI),
                           "E_Pa": str(E), "nu": str(NU)}}
    return {"id": label, "family": "closed_end_pressure", "routes": ["exact"],
            "method": "closed-end cantilever: N_w = p A_i on every member; the elastic axial strain (1-2nu) p A_i/(s E A_n)",
            "inputs": inputs, "key_values": {"wall_force_N": record(pai, 1), "tip_ux_m": record(nodes["node:c"]["ux"], 0)},
            "zero_scales": scales, "rows": rows}


# ----------------------------------------------------------------------------- publication
def family(unit):
    return {"N": "force", "N*m": "moment", "m": "translation", "rad": "rotation"}[unit]


def publish(reactions, nodes, sections, k_force, k_motion, fallback=None):
    rows = []
    for sup in sorted(reactions):
        for comp in COMPONENTS_F:
            unit = "N" if comp.startswith("F") else "N*m"
            rows.append(dict(quantity="support_reaction", entity=sup, component=comp, unit=unit,
                             **record(reactions[sup].get(comp, F(0)), k_force)))
    for nd in sorted(nodes):
        for comp in COMPONENTS_U:
            unit = "m" if comp.startswith("u") else "rad"
            rows.append(dict(quantity="nodal_motion", entity=nd, component=comp, unit=unit,
                             **record(nodes[nd].get(comp, F(0)), k_motion)))
    for pipe in sorted(sections):
        for name, sec in zip(STATION_NAMES, sections[pipe]):
            for comp in SECTION_COMPONENTS:
                unit = "N" if comp in ("axial_force", "shear_force_y", "shear_force_z") else "N*m"
                rows.append(dict(quantity="section_action", entity=pipe, station=name, component=comp,
                                 unit=unit, **record(sec[comp], k_force)))
    tops = {}
    for fam in ("force", "moment", "translation", "rotation"):
        mags = [abs(F(r["exact"].replace("*pi", "").replace("/pi", ""))) for r in rows if family(r["unit"]) == fam]
        tops[fam] = max(mags) if mags else F(0)
    fallback = dict(fallback or {})
    if tops["force"] == 0:
        tops["force"] = fallback["force"]
    if tops["moment"] == 0:
        tops["moment"] = fallback.get("moment", tops["force"])  # the force scale times 1 m
    if tops["rotation"] == 0:
        tops["rotation"] = tops["translation"]  # the translation scale per 1 m
    scales = {}
    for fam, unit, k in (("force", "N", k_force), ("moment", "N*m", k_force), ("translation", "m", k_motion), ("rotation", "rad", k_motion)):
        assert tops[fam] > 0, fam
        scales[fam] = dict(unit=unit, **record(tops[fam], k))
    return rows, scales


def build_cases():
    cases = []
    for timoshenko in (False, True):
        # s = 1, 1/4 and every decade from 1e3 to 1e8: the decades locate the product's
        # precision threshold (DEL-03-05 Design §2.5); every case is exact for every s.
        for scale in (F(1), F(1, 4), *(F(10 ** k) for k in range(3, 9))):
            cases.append(case_r(scale, timoshenko))
    # s = 1 equals plain pipe: the scaled cantilever at s = 1 is the unscaled one
    plain, _, _ = cantilever_closed(chain_r(F(1)), R_LENGTH, FX, FY, MX, False)
    cross("W15-R: s = 1 equals plain pipe (no body)", plain["node:d"]["uy"],
          FY * R_LENGTH ** 3 / (3 * E * INERTIA_P))
    for tag, scale in (("PF-MAT", F(1, 4)), ("PF-MAT", F(4)), ("PF-NAMED", F(4)), ("PF-INTERP", F(4)),
                       ("EX-MAT-THERMAL", F(4)), ("EX-NAMED-FIT", F(4)), ("EX-INTERP-FITONLY", F(4)), ("EX-CANCEL", F(4))):
        cases.append(case_t(tag, scale))
    cases.append(case_free(F(10 ** 6)))
    cases.append(case_p(F(4)))
    return cases


def document(cases):
    return {
        "document": "openpipestress.w15_rigid_scaling_references",
        "status": "candidate frozen references; derived and cross-checked by the author TASK, not yet independently refuted",
        "units": {"force": "N", "moment": "N*m", "translation": "m", "rotation": "rad"},
        "theory": "small-displacement linear-elastic space frame; Euler-Bernoulli, or Timoshenko with the energy-matched annular "
                  "shear factor; a member with DEC-045 stiffness scale s has stiffness s K; annular section, Iy = Iz = I, J = 2I",
        "section": {"outside_diameter_m": str(OD), "inside_diameter_m": str(ID), "wall_thickness_m": str(WALL),
                    "elastic_modulus_Pa": str(E), "poisson_ratio": str(NU), "shear_modulus_Pa": str(G),
                    "area_m2": f"{AREA_P}*pi", "second_moment_m4": f"{INERTIA_P}*pi", "polar_moment_m4": f"{POLAR_P}*pi",
                    "bore_area_m2": f"{BORE_P}*pi", "shear_factor_kappa": str(KAPPA)},
        "conventions": {
            "support_reaction": "global action of the support on the structure at its node (moments about that node)",
            "nodal_motion": "global translation and right-handed rotation",
            "section_action": "element-local action on the +x face of the i-side portion at station s/L (local x from i to j); "
                              "a node-on-element end action is -section(0) at end_i and +section(1) at end_j",
            "wall_axial_force": "the pressure-2 wall axial force N_w of the member",
            "exact": "rational q, q*pi or q/pi",
        },
        "criterion": "|observed - expected| <= 1e-9 * |expected|; an exactly zero expectation uses 1e-9 * zero_scale "
                     "of its family (largest nonzero magnitude of that family in the case, or the case's stated fallback)",
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
