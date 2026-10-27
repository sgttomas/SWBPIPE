#!/usr/bin/env python3
"""W-15c independent references: the eccentric reducer offset.

Standard-library Python 3 only. This generator imports, reads and runs no
product code and reads no file other than the ones it writes. Every expected
value is derived here in exact rational arithmetic (DEL-03-05 Design
`eccentric-reducer-offset.md` section 7) by two independent formulations that
must agree before anything is written:

  route 1  unit-load virtual work on the space frame (Timoshenko & Gere,
           Mechanics of Materials; Hibbeler, Structural Analysis): section
           actions by statics from the downstream loads, motions as the exact
           integrals of real times virtual section actions over each straight
           member's compliances 1/(s EA), 1/(s kappa G A) (Timoshenko only),
           1/(s GJ) and 1/(s EI); the force method for the anchored thermal
           case (redundants at the far anchor);
  route 2  an exact direct-stiffness solve of the same space frame: 12 x 12
           Euler-Bernoulli or Timoshenko member stiffness in member axes,
           rotated with exact direction cosines, held free-strain end actions,
           partitioned solve of the free DOFs.

Every motion, support reaction and member end action of route 2 is compared
with route 1; the design table's hand values are compared with the published
decimals; the rigid-link limit is checked as m(s) = m_rigid + c/s at s and 2s
and against rigid-offset kinematics.

Every member is a 5-12-13 or axis-aligned straight member, so every direction
cosine is rational. Sections are annuli, so A, I and J are rational multiples
of pi. Every result is q * pi**k with q rational: under given forces, forces
carry k = 0 and motions k = -1; under a thermal strain, forces carry k = 1 and
motions k = 0. All model data are invented.

Modes:
  python3 -I w15c_reference.py           check: rebuild every output in memory and
                                          compare byte for byte with the committed
                                          files, SHA256SUMS included
  python3 -I w15c_reference.py --write   write the outputs (intentional changes only)
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
FIXED_FILES = ("README.md", "generators/w15c_reference.py")

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
_PI_90 = F(str(PI_DEC)[:92])
PI_LO, PI_HI = _PI_90, _PI_90 + F(1, 10 ** 90)


def binary64(q, k):
    """Nearest double to q * pi**k, the same for both rational bounds of pi."""
    if k == 0 or q == 0:
        return float(q)
    lo, hi = ((float(q * PI_LO), float(q * PI_HI)) if k == 1 else (float(q / PI_HI), float(q / PI_LO)))
    if lo != hi:
        raise SystemExit(f"binary64 rounding of {q}*pi**{k} is not decided by the pi bounds")
    return lo


def decimal40(q, k):
    if q == 0:
        return "0"
    getcontext().prec = 100
    v = Decimal(q.numerator) / Decimal(q.denominator) * (PI_DEC ** k if k else 1)
    return format(v.normalize().__format__(".39e"))


def significant(q, k, digits):
    """q * pi**k to `digits` significant digits, as the design table prints it."""
    getcontext().prec = 100
    v = Decimal(q.numerator) / Decimal(q.denominator) * (PI_DEC ** k if k else 1)
    return format(v, f".{digits - 1}e")


def exact_text(q, k):
    s = str(q)
    if k == 0 or q == 0:
        return s
    return f"{s}*pi" if k == 1 else f"{s}/pi"


def record(q, k):
    q = F(q)
    return {"exact": exact_text(q, k), "decimal": decimal40(q, k), "value": binary64(q, k)}


# ----------------------------------------------------------------------------- inputs (invented)
E = F(200 * 10 ** 9)  # Pa
NU = F(1, 4)
G = E / (2 * (1 + NU))  # 80 GPa, the preview route's authored G and the exact route's derived G
assert G == 80 * 10 ** 9
ALPHA = F(12, 10 ** 6)  # 1/K (preview: per degC)
DT = F(100)  # K
EPS_T = ALPHA * DT  # the free thermal strain, 3/2500


def annulus(od, idia):
    m = idia / od
    kappa = (6 * (1 + NU) ** 2 * (1 + m ** 2) ** 2
             / ((7 + 14 * NU + 8 * NU ** 2) * (1 + m ** 2) ** 2 + 4 * m ** 2 * (5 + 10 * NU + 4 * NU ** 2)))
    area = (od ** 2 - idia ** 2) / 4  # / pi
    inertia = (od ** 4 - idia ** 4) / 64  # / pi
    return {"od": od, "id": idia, "area": area, "inertia": inertia, "polar": 2 * inertia, "kappa": kappa}


SECTIONS = {"S1": annulus(F(1, 5), F(9, 50)), "S2": annulus(F(1, 10), F(9, 100))}
assert SECTIONS["S1"]["area"] == F(19, 10000) and SECTIONS["S1"]["inertia"] == F(85975, 10 ** 10)
assert SECTIONS["S2"]["area"] == F(475, 10 ** 6) and SECTIONS["S2"]["inertia"] == F(53734375, 10 ** 14)

P_FX = F(10000)  # N, W15C-P
Z_FY = F(1000)  # N, W15C-Z

COMPONENTS_F = ("Fx", "Fy", "Fz", "Mx", "My", "Mz")
COMPONENTS_U = ("ux", "uy", "uz", "rx", "ry", "rz")
SECTION_COMPONENTS = ("axial_force", "shear_force_y", "shear_force_z",
                      "torsional_moment", "bending_moment_y", "bending_moment_z")
STATIONS = (F(0), F(1, 2), F(1))
STATION_NAMES = ("0", "1/2", "1")

CROSS = []


def cross(name, a, b):
    CROSS.append({"check": name, "agrees": a == b})
    if a != b:
        print(f"DISAGREES {name}: {a} != {b}")


# ----------------------------------------------------------------------------- exact vectors
def add(a, b):
    return [x + y for x, y in zip(a, b)]


def sub(a, b):
    return [x - y for x, y in zip(a, b)]


def mul(k, a):
    return [k * x for x in a]


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def crossp(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def exact_sqrt(q):
    q = F(q)
    def isqrt_exact(n):
        r = int(n ** 0.5)
        for c in (r - 1, r, r + 1):
            if c >= 0 and c * c == n:
                return c
        raise SystemExit(f"{n} is not a perfect square")
    return F(isqrt_exact(q.numerator), isqrt_exact(q.denominator))


def norm(a):
    return exact_sqrt(dot(a, a))


def unit(a):
    n = norm(a)
    return [x / n for x in a]


# ----------------------------------------------------------------------------- the frame
class Frame:
    """A chain of straight members from the support node `chain[0]`. A member is
    {id, from, to, section, scale, y_reference}; its axes are x from `from` to `to`,
    y the y_reference less its x part, z = x cross y (the product's convention)."""

    def __init__(self, nodes, chain, members):
        self.x = {nid: [F(v) for v in p] for nid, p in nodes.items()}
        self.chain = chain  # node ids from the support outwards
        self.members = members

    def axes(self, m):
        xa = unit(sub(self.x[m["to"]], self.x[m["from"]]))
        yr = [F(v) for v in m["y_reference"]]
        ya = unit(sub(yr, mul(dot(yr, xa), xa)))
        return xa, ya, crossp(xa, ya)

    def length(self, m):
        return norm(sub(self.x[m["to"]], self.x[m["from"]]))

    def position(self, nid):
        return self.chain.index(nid)

    def upstream_end(self, m):
        """+1 when the member's `from` is the end nearer the support."""
        return 1 if self.position(m["from"]) < self.position(m["to"]) else -1

    def downstream_index(self, m):
        return max(self.position(m["from"]), self.position(m["to"]))


def stiffness(m, timoshenko, rigid=False):
    """The member's (EA, kGA, GJ, EI) / pi, scaled by s; None for a rigid body."""
    if rigid:
        return None
    sec, s = SECTIONS[m["section"]], F(m["scale"])
    return {"ea": s * E * sec["area"], "kga": s * sec["kappa"] * G * sec["area"] if timoshenko else None,
            "gj": s * G * sec["polar"], "ei": s * E * sec["inertia"]}


# ----------------------------------------------------------------------------- route 1: statics and virtual work
def section_linear(frame, m, loads):
    """The member's local section action (six components) as (value at xi = 0,
    slope in xi), from the external loads {node: (force, moment)} beyond it,
    on the +x face of the i-side portion."""
    sigma = frame.upstream_end(m)
    xa, ya, za = frame.axes(m)
    xi0, d = frame.x[m["from"]], sub(frame.x[m["to"]], frame.x[m["from"]])
    down = frame.downstream_index(m)
    force, m0, m1 = [F(0)] * 3, [F(0)] * 3, [F(0)] * 3
    for nid, (f, mo) in loads.items():
        if frame.position(nid) < down:
            continue
        force = add(force, f)
        m0 = add(m0, add(crossp(sub(frame.x[nid], xi0), f), mo))
        m1 = add(m1, crossp(mul(-1, d), f))
    a0 = [dot(force, xa), dot(force, ya), dot(force, za), dot(m0, xa), dot(m0, ya), dot(m0, za)]
    a1 = [F(0), F(0), F(0), dot(m1, xa), dot(m1, ya), dot(m1, za)]
    return [sigma * v for v in a0], [sigma * v for v in a1]


def work(frame, m, real, virtual, timoshenko, rigid=False):
    k = stiffness(m, timoshenko, rigid)
    if k is None:
        return F(0)
    L = frame.length(m)
    flex = [1 / k["ea"], (1 / k["kga"]) if timoshenko else F(0), (1 / k["kga"]) if timoshenko else F(0),
            1 / k["gj"], 1 / k["ei"], 1 / k["ei"]]
    total = F(0)
    for c in range(6):
        a, b = real[0][c], real[1][c]
        p, q = virtual[0][c], virtual[1][c]
        total += flex[c] * L * (a * p + (a * q + b * p) / 2 + b * q / 3)
    return total


def unit_load(nid, c):
    f = [F(0)] * 3
    mo = [F(0)] * 3
    (f if c < 3 else mo)[c % 3] = F(1)
    return {nid: (f, mo)}


def closed_motions(frame, loads, timoshenko, rigid_member=None, free_strain=F(0)):
    """Nodal motions (exact, / pi for mechanical loads) by unit-load virtual work."""
    out = {}
    origin = frame.x[frame.chain[0]]
    for nid in frame.chain:
        motion = []
        for c in range(6):
            total = F(0)
            for m in frame.members:
                if frame.downstream_index(m) > frame.position(nid):
                    continue
                total += work(frame, m, section_linear(frame, m, loads), section_linear(frame, m, unit_load(nid, c)),
                              timoshenko, rigid=(m["id"] == rigid_member))
            if c < 3:
                total += free_strain * (frame.x[nid][c] - origin[c])
            motion.append(total)
        out[nid] = motion
    return out


def closed_sections(frame, loads):
    out = {}
    for m in frame.members:
        a0, a1 = section_linear(frame, m, loads)
        out[m["id"]] = [{SECTION_COMPONENTS[c]: a0[c] + st * a1[c] for c in range(6)} for st in STATIONS]
    return out


def support_reaction(frame, nid, loads):
    """Global action on the structure at a fixed end of the chain that balances the
    loads (moments about that node)."""
    f, mo = [F(0)] * 3, [F(0)] * 3
    for lid, (lf, lm) in loads.items():
        f = add(f, lf)
        mo = add(mo, add(crossp(sub(frame.x[lid], frame.x[nid]), lf), lm))
    return dict(zip(COMPONENTS_F, [-v for v in f] + [-v for v in mo]))


def force_method(frame, far, timoshenko, strain):
    """Redundants at the far anchor of a chain anchored at both ends under a free
    strain: flexibility f R = -Delta_free. Returns R as a load at `far`."""
    origin = frame.x[frame.chain[0]]
    flex = [[F(0)] * 6 for _ in range(6)]
    for i in range(6):
        for j in range(6):
            for m in frame.members:
                flex[i][j] += work(frame, m, section_linear(frame, m, unit_load(far, i)),
                                   section_linear(frame, m, unit_load(far, j)), timoshenko)
    delta = [strain * (frame.x[far][c] - origin[c]) for c in range(3)] + [F(0)] * 3
    r = solve(flex, [-v for v in delta])
    return {far: (r[:3], r[3:])}


# ----------------------------------------------------------------------------- route 2: direct stiffness
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


def local_stiffness(frame, m, timoshenko):
    k = stiffness(m, timoshenko)
    L = frame.length(m)
    phi = 12 * k["ei"] / (k["kga"] * L ** 2) if timoshenko else F(0)
    c = k["ei"] / ((1 + phi) * L ** 3)
    K = [[F(0)] * 12 for _ in range(12)]

    def put(i, j, v):
        K[i][j] = v
        K[j][i] = v
    put(0, 0, k["ea"] / L); put(6, 6, k["ea"] / L); put(0, 6, -k["ea"] / L)
    put(3, 3, k["gj"] / L); put(9, 9, k["gj"] / L); put(3, 9, -k["gj"] / L)
    # bending in the local x-y plane (v, theta_z)
    put(1, 1, 12 * c); put(1, 5, 6 * L * c); put(1, 7, -12 * c); put(1, 11, 6 * L * c)
    put(5, 5, (4 + phi) * L ** 2 * c); put(5, 7, -6 * L * c); put(5, 11, (2 - phi) * L ** 2 * c)
    put(7, 7, 12 * c); put(7, 11, -6 * L * c); put(11, 11, (4 + phi) * L ** 2 * c)
    # bending in the local x-z plane (w, theta_y)
    put(2, 2, 12 * c); put(2, 4, -6 * L * c); put(2, 8, -12 * c); put(2, 10, -6 * L * c)
    put(4, 4, (4 + phi) * L ** 2 * c); put(4, 8, 6 * L * c); put(4, 10, (2 - phi) * L ** 2 * c)
    put(8, 8, 12 * c); put(8, 10, 6 * L * c); put(10, 10, (4 + phi) * L ** 2 * c)
    return K


def rotation(frame, m):
    xa, ya, za = frame.axes(m)
    lam = [xa, ya, za]
    T = [[F(0)] * 12 for _ in range(12)]
    for b in range(4):
        for i in range(3):
            for j in range(3):
                T[3 * b + i][3 * b + j] = lam[i][j]
    return T


def matmul(a, b):
    return [[sum(a[i][k] * b[k][j] for k in range(len(b))) for j in range(len(b[0]))] for i in range(len(a))]


def transpose(a):
    return [list(r) for r in zip(*a)]


def matvec(a, x):
    return [sum(r * v for r, v in zip(row, x)) for row in a]


def direct(frame, loads, fixed_nodes, timoshenko, strain=F(0)):
    ids = frame.chain
    index = {nid: 6 * k for k, nid in enumerate(ids)}
    n = 6 * len(ids)
    K = [[F(0)] * n for _ in range(n)]
    P = [F(0)] * n
    data = []
    for m in frame.members:
        kl = local_stiffness(frame, m, timoshenko)
        T = rotation(frame, m)
        kg = matmul(transpose(T), matmul(kl, T))
        held = stiffness(m, timoshenko)["ea"] * strain
        fl = [held] + [F(0)] * 5 + [-held] + [F(0)] * 5  # node-on-element held end actions
        fg = matvec(transpose(T), fl)
        dofs = [index[m["from"]] + d for d in range(6)] + [index[m["to"]] + d for d in range(6)]
        for a in range(12):
            P[dofs[a]] -= fg[a]
            for b in range(12):
                K[dofs[a]][dofs[b]] += kg[a][b]
        data.append((m["id"], dofs, kl, T, fl))
    for nid, (f, mo) in loads.items():
        for c in range(3):
            P[index[nid] + c] += f[c]
            P[index[nid] + 3 + c] += mo[c]
    restrained = {index[nid] + d for nid in fixed_nodes for d in range(6)}
    free = [d for d in range(n) if d not in restrained]
    U = [F(0)] * n
    for f, v in zip(free, solve([[K[a][b] for b in free] for a in free], [P[a] for a in free])):
        U[f] = v
    ends, total = {}, [F(0)] * n
    for mid, dofs, kl, T, fl in data:
        ul = matvec(T, [U[d] for d in dofs])
        el = [x + y for x, y in zip(matvec(kl, ul), fl)]
        ends[mid] = el
        eg = matvec(transpose(T), el)
        for a in range(12):
            total[dofs[a]] += eg[a]
    applied = [F(0)] * n
    for nid, (f, mo) in loads.items():
        for c in range(3):
            applied[index[nid] + c] += f[c]
            applied[index[nid] + 3 + c] += mo[c]
    for d in free:
        cross(f"free-DOF equilibrium {ids[d // 6]} {d % 6}", total[d], applied[d])
    motions = {nid: [U[index[nid] + c] for c in range(6)] for nid in ids}
    reactions = {nid: dict(zip(COMPONENTS_F, [total[index[nid] + c] - applied[index[nid] + c] for c in range(6)]))
                 for nid in fixed_nodes}
    return motions, reactions, ends


# ----------------------------------------------------------------------------- geometry of the cases
N0, A, B, C = "node:n0", "node:a", "node:b", "node:c"
CHAIN = [N0, A, B, C]


def nodes_for(offset):
    """offset: B's displacement from (1.12, 0, 0); C follows by 1 m along X."""
    b = add([F(28, 25), F(0), F(0)], offset)
    return {N0: [F(0)] * 3, A: [F(1), F(0), F(0)], B: b, C: add(b, [F(1), F(0), F(0)])}


def members_for(scale, body_reversed=False):
    body = {"id": "body:a-b", "from": B if body_reversed else A, "to": A if body_reversed else B,
            "section": "S1", "scale": F(scale), "y_reference": [0, 1, 0]}
    return [{"id": "pipe:n0-a", "from": N0, "to": A, "section": "S1", "scale": F(1), "y_reference": [0, 1, 0]},
            body,
            {"id": "pipe:b-c", "from": B, "to": C, "section": "S2", "scale": F(1), "y_reference": [0, 1, 0]}]


OFFSET_Y = [F(0), F(-1, 20), F(0)]
OFFSET_Z = [F(0), F(0), F(-1, 20)]
OFFSET_0 = [F(0), F(0), F(0)]


def s_text(s):
    return str(F(s))


def tag(s):
    return str(F(s)).replace("/", "_")


# ----------------------------------------------------------------------------- publication
def family(u):
    return {"N": "force", "N*m": "moment", "m": "translation", "rad": "rotation"}[u]


def publish(reactions, motions, sections, k_force, k_motion, fallback=None):
    rows = []
    for sup in sorted(reactions):
        for comp in COMPONENTS_F:
            u = "N" if comp.startswith("F") else "N*m"
            rows.append(dict(quantity="support_reaction", entity=sup, component=comp, unit=u, **record(reactions[sup][comp], k_force)))
    for nd in sorted(motions):
        for c, comp in enumerate(COMPONENTS_U):
            u = "m" if comp.startswith("u") else "rad"
            rows.append(dict(quantity="nodal_motion", entity=nd, component=comp, unit=u, **record(motions[nd][c], k_motion)))
    for pipe in sorted(sections):
        for name, sec in zip(STATION_NAMES, sections[pipe]):
            for comp in SECTION_COMPONENTS:
                u = "N" if comp in ("axial_force", "shear_force_y", "shear_force_z") else "N*m"
                rows.append(dict(quantity="section_action", entity=pipe, station=name, component=comp, unit=u,
                                 **record(sec[comp], k_force)))
    tops = {}
    for fam in ("force", "moment", "translation", "rotation"):
        mags = [abs(F(r["exact"].replace("*pi", "").replace("/pi", ""))) for r in rows if family(r["unit"]) == fam]
        tops[fam] = max(mags) if mags else F(0)
    fallback = dict(fallback or {})
    if tops["force"] == 0:
        tops["force"] = fallback["force"]
    if tops["moment"] == 0:
        tops["moment"] = fallback.get("moment", tops["force"])
    if tops["rotation"] == 0:
        tops["rotation"] = tops["translation"]
    scales = {}
    for fam, u, k in (("force", "N", k_force), ("moment", "N*m", k_force), ("translation", "m", k_motion), ("rotation", "rad", k_motion)):
        assert tops[fam] > 0, fam
        scales[fam] = dict(unit=u, **record(tops[fam], k))
    return rows, scales


def support_ids(fixed):
    return {f"support:{nid.split(':')[1]}": nid for nid in fixed}


def compare_routes(label, frame, loads, fixed, timoshenko, motions1, reactions1, sections1, strain=F(0)):
    motions2, reactions2, ends2 = direct(frame, loads, fixed, timoshenko, strain)
    for nid in frame.chain:
        for c in range(6):
            cross(f"{label}: {nid} {COMPONENTS_U[c]} direct stiffness", motions2[nid][c], motions1[nid][c])
    for nid in fixed:
        for comp in COMPONENTS_F:
            cross(f"{label}: reaction {nid} {comp} direct stiffness", reactions2[nid][comp], reactions1[nid][comp])
    for mid, el in ends2.items():
        for c, comp in enumerate(SECTION_COMPONENTS):
            cross(f"{label}: {mid} {comp} end_i", el[c], -sections1[mid][0][comp])
            cross(f"{label}: {mid} {comp} end_j", el[6 + c], sections1[mid][-1][comp])


def inputs(frame, fixed, body_node, scale, offset_mm, loads_text, beam):
    return {"geometry": {
        "nodes": {nid: [str(v) for v in frame.x[nid]] for nid in frame.chain},
        "members": {m["id"]: {"from": m["from"], "to": m["to"], "section": m["section"], "y_reference": m["y_reference"],
                              "stiffness_scale": s_text(m["scale"])} for m in frame.members},
        "supports": {sid: {"node": nid, "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"]} for sid, nid in support_ids(fixed).items()}},
        "reducer": {"member": "body:a-b", "component_kind": "reducer", "component_node": body_node, "stiffness_scale": s_text(scale),
                    "eccentric_offset_mm": [str(v) for v in offset_mm], "rigid_body_length_m": "3/25",
                    "convention": "option A (R-c1): from the centerline at the component's node to the centerline at the body's other end"},
        "loads": loads_text, "beam_theory": beam}


def mechanical_case(cid, family_name, nodes, scale, timoshenko, load_node, force, body_node, offset_mm, routes,
                    body_reversed=False, design=None):
    frame = Frame(nodes, CHAIN, members_for(scale, body_reversed))
    loads = {load_node: (force, [F(0)] * 3)}
    motions = closed_motions(frame, loads, timoshenko)
    sections = closed_sections(frame, loads)
    reactions = {N0: support_reaction(frame, N0, loads)}
    compare_routes(cid, frame, loads, [N0], timoshenko, motions, reactions, sections)
    # rigid-link limit: m(s) = m_rigid + c/s, and rigid-offset kinematics
    rigid = closed_motions(frame, loads, timoshenko, rigid_member="body:a-b")
    other = closed_motions(Frame(nodes, CHAIN, members_for(F(scale) * 2, body_reversed)), loads, timoshenko)
    for c in range(6):
        cross(f"{cid}: tip {COMPONENTS_U[c]} tends to the rigid link as c/s",
              (motions[C][c] - rigid[C][c]) * F(scale), (other[C][c] - rigid[C][c]) * F(scale) * 2)
    # rigid offset kinematics: B moves with A as a rigid body
    theta_a, u_a = rigid[A][3:], rigid[A][:3]
    u_b = add(u_a, crossp(theta_a, sub(frame.x[B], frame.x[A])))
    for c in range(3):
        cross(f"{cid}: rigid limit B = A + theta_A x D ({COMPONENTS_U[c]})", rigid[B][c], u_b[c])
        cross(f"{cid}: rigid limit B rotation = A rotation ({COMPONENTS_U[3 + c]})", rigid[B][3 + c], theta_a[c])
    if design:
        for comp, text in design.items():
            source, which = comp
            q = (rigid if source == "rigid" else motions)[C][COMPONENTS_U.index(which)]
            cross(f"{cid}: design table {source} tip {which} = {text}", significant(q, -1, 14), text)
    published = {nid: motions[nid] for nid in (A, B, C)}
    rows, scales = publish({"support:n0": reactions[N0]}, published, sections, 0, -1)
    key = {f"tip_{COMPONENTS_U[c]}": record(motions[C][c], -1) for c in range(6)}
    key.update({f"rigid_link_tip_{COMPONENTS_U[c]}": record(rigid[C][c], -1) for c in range(6)})
    load_text = {"node": load_node, "force_N": [str(v) for v in force]}
    return {"id": cid, "family": family_name, "routes": routes,
            "method": "unit-load virtual work on the space frame (statics for section actions and reactions), cross-checked "
                      "by an exact direct-stiffness solve; rigid-link limit with the body's compliance removed",
            "inputs": inputs(frame, [N0], body_node, scale, offset_mm, load_text,
                             "timoshenko_annulus_energy_v1" if timoshenko else "euler_bernoulli"),
            "key_values": key, "zero_scales": scales, "rows": rows}


def thermal_free_case(scale):
    cid = f"W15C-T-S{tag(scale)}"
    frame = Frame(nodes_for(OFFSET_Y), CHAIN, members_for(scale))
    motions = closed_motions(frame, {}, False, free_strain=EPS_T)
    sections = closed_sections(frame, {})
    reactions = {N0: support_reaction(frame, N0, {})}
    compare_routes(cid, frame, {}, [N0], False, motions, reactions, sections, strain=EPS_T)
    cross(f"{cid}: B grows by eps (1.12, -0.05, 0)", motions[B][:3], [EPS_T * F(28, 25), EPS_T * F(-1, 20), F(0)])
    sec = SECTIONS["S1"]
    held = F(scale) * E * sec["area"] * EPS_T  # / pi
    rows, scales = publish({"support:n0": reactions[N0]}, {nid: motions[nid] for nid in (A, B, C)}, sections, 1, 0,
                           fallback={"force": held, "moment": held})
    return {"id": cid, "family": "free_thermal_growth", "routes": ["preview", "exact"],
            "method": "free thermal growth eps (x - x_N0) at every node (a statically determinate chain); "
                      "cross-checked by an exact direct-stiffness solve with held free-strain end actions",
            "inputs": inputs(frame, [N0], A, scale, [F(0), F(-50), F(0)],
                             {"temperature_change_K": str(DT), "thermal_expansion_per_K": str(ALPHA), "free_strain": str(EPS_T)},
                             "euler_bernoulli"),
            "key_values": {f"tip_{COMPONENTS_U[c]}": record(motions[C][c], 0) for c in range(3)},
            "zero_scales": scales, "rows": rows}


def thermal_anchored_case(scale, timoshenko):
    cid = f"W15C-TF-S{tag(scale)}-{'TIM' if timoshenko else 'EB'}"
    frame = Frame(nodes_for(OFFSET_Y), CHAIN, members_for(scale))
    loads = force_method(frame, C, timoshenko, EPS_T)  # / pi: forces q (times pi)
    motions = closed_motions(frame, loads, timoshenko, free_strain=EPS_T)
    for c in range(6):
        cross(f"{cid}: the far anchor returns to rest ({COMPONENTS_U[c]})", motions[C][c], F(0))
    sections = closed_sections(frame, loads)
    reactions = {C: dict(zip(COMPONENTS_F, loads[C][0] + loads[C][1])), N0: support_reaction(frame, N0, loads)}
    compare_routes(cid, frame, {}, [N0, C], timoshenko, motions, reactions, sections, strain=EPS_T)
    rows, scales = publish({"support:n0": reactions[N0], "support:c": reactions[C]},
                           {nid: motions[nid] for nid in (A, B)}, sections, 1, 0)
    return {"id": cid, "family": "anchored_thermal", "routes": ["exact"] if timoshenko else ["preview", "exact"],
            "method": "force method with the six redundants at the far anchor (unit-load flexibility over the frame, "
                      "compatibility with the free growth), cross-checked by an exact direct-stiffness solve",
            "inputs": inputs(frame, [N0, C], A, scale, [F(0), F(-50), F(0)],
                             {"temperature_change_K": str(DT), "thermal_expansion_per_K": str(ALPHA), "free_strain": str(EPS_T)},
                             "timoshenko_annulus_energy_v1" if timoshenko else "euler_bernoulli"),
            "key_values": {f"reaction_c_{k}": record(reactions[C][k], 1) for k in COMPONENTS_F},
            "zero_scales": scales, "rows": rows}


# The design table (DEL-03-05 Design section 7.2), to 14 significant digits.
DESIGN_P = {
    F(1): {("elastic", "ux"): "4.7639233326413e-5", ("elastic", "uy"): "1.5605635758029e-4", ("elastic", "rz"): "9.8575175570147e-5",
           ("rigid", "ux"): "4.6510822276055e-5", ("rigid", "uy"): "1.4994533748698e-4", ("rigid", "rz"): "9.2558850300608e-5"},
    F(1000): {("elastic", "ux"): "4.6511950687106e-5", ("elastic", "uy"): "1.4995144850708e-4", ("elastic", "rz"): "9.2564866625877e-5"},
}
DESIGN_P0 = {F(1): {("elastic", "ux"): "4.2888068875290e-5", ("rigid", "ux"): "4.1882879761025e-5"},
             F(1000): {("elastic", "ux"): "4.1883884950139e-5"}}


def build_cases():
    cases = []
    fx = [P_FX, F(0), F(0)]
    fy = [F(0), Z_FY, F(0)]
    for timoshenko in (False, True):
        beam = "TIM" if timoshenko else "EB"
        routes = ["exact"] if timoshenko else ["preview", "exact"]
        for s in (F(1), F(1000)):
            cases.append(mechanical_case(f"W15C-P-S{tag(s)}-{beam}", "eccentric_step_axial", nodes_for(OFFSET_Y), s, timoshenko,
                                         C, fx, A, [F(0), F(-50), F(0)], routes, design=None if timoshenko else DESIGN_P[s]))
            cases.append(mechanical_case(f"W15C-Z-S{tag(s)}-{beam}", "eccentric_step_torsion", nodes_for(OFFSET_Z), s, timoshenko,
                                         C, fy, A, [F(0), F(0), F(-50)], routes))
    for s in (F(1), F(1000)):
        cases.append(mechanical_case(f"W15C-P0-S{tag(s)}-EB", "concentric_control", nodes_for(OFFSET_0), s, False,
                                     C, fx, A, [F(0)] * 3, ["preview", "exact"], design=DESIGN_P0[s]))
    # convention rows (R-c1 option A): the same structure, the member reversed or the component at B
    cases.append(mechanical_case("W15C-R-S1000-EB", "convention_reversed_member", nodes_for(OFFSET_Y), F(1000), False,
                                 C, fx, A, [F(0), F(-50), F(0)], ["preview", "exact"], body_reversed=True))
    cases.append(mechanical_case("W15C-O-S1000-EB", "convention_opposite_end", nodes_for(OFFSET_Y), F(1000), False,
                                 C, fx, B, [F(0), F(50), F(0)], ["preview", "exact"]))
    cases.append(thermal_free_case(F(1000)))
    for timoshenko in (False, True):
        for s in (F(1), F(1000)):
            cases.append(thermal_anchored_case(s, timoshenko))
    # W15C-Z statics confirmed by Codex r2 (design section 7.4)
    z = next(c for c in cases if c["id"] == "W15C-Z-S1-EB")
    zrows = {(r["quantity"], r["entity"], r.get("station"), r["component"]): F(r["exact"]) for r in z["rows"]
             if r["quantity"] != "nodal_motion"}
    cross("W15C-Z: P1 torsion 50 N m", zrows[("section_action", "pipe:n0-a", "0", "torsional_moment")], F(50))
    cross("W15C-Z: reaction Mx -50 N m", zrows[("support_reaction", "support:n0", None, "Mx")], F(-50))
    cross("W15C-Z: reaction Mz -2120 N m", zrows[("support_reaction", "support:n0", None, "Mz")], F(-2120))
    cross("W15C-Z: body torsion -5000/13 N m", zrows[("section_action", "body:a-b", "0", "torsional_moment")], F(-5000, 13))
    cross("W15C-Z: body bending at A 13690/13 N m", zrows[("section_action", "body:a-b", "0", "bending_moment_z")], F(13690, 13))
    cross("W15C-Z: body bending at B 12000/13 N m", zrows[("section_action", "body:a-b", "1", "bending_moment_z")], F(12000, 13))
    # the convention rows solve as W15C-P at s = 1000 (motions and reactions)
    p = next(c for c in cases if c["id"] == "W15C-P-S1000-EB")
    pick = lambda c: {(r["quantity"], r["entity"], r["component"]): r["exact"] for r in c["rows"] if r["quantity"] != "section_action"}
    for cid in ("W15C-R-S1000-EB", "W15C-O-S1000-EB"):
        cross(f"{cid}: motions and reactions equal W15C-P-S1000-EB", pick(next(c for c in cases if c["id"] == cid)), pick(p))
    return cases


def document(cases):
    return {
        "document": "openpipestress.w15c_eccentric_reducer_references",
        "status": "candidate frozen references; derived by two exact formulations that agree, not yet independently refuted",
        "units": {"force": "N", "moment": "N*m", "translation": "m", "rotation": "rad"},
        "theory": "small-displacement linear-elastic space frame; Euler-Bernoulli, or Timoshenko with the energy-matched annular "
                  "shear factor; a member with DEC-045 stiffness scale s has stiffness s K; the reducer's body is the straight "
                  "member between the two end centerlines",
        "material": {"elastic_modulus_Pa": str(E), "poisson_ratio": str(NU), "shear_modulus_Pa": str(G),
                     "thermal_expansion_per_K": str(ALPHA)},
        "sections": {k: {"outside_diameter_m": str(v["od"]), "inside_diameter_m": str(v["id"]), "area_m2": f"{v['area']}*pi",
                         "second_moment_m4": f"{v['inertia']}*pi", "polar_moment_m4": f"{v['polar']}*pi",
                         "shear_factor_kappa": str(v["kappa"])} for k, v in SECTIONS.items()},
        "conventions": {
            "support_reaction": "global action of the support on the structure at its node (moments about that node)",
            "nodal_motion": "global translation and right-handed rotation",
            "section_action": "member-axis action on the +x face of the i-side portion at station s/L (x from `from` to `to`, "
                              "y the y_reference less its x part, z = x cross y); a node-on-element end action is "
                              "-section(0) at end_i and +section(1) at end_j",
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
        print(c["id"], ", ".join(f"{k} {v['value']:.14g}" for k, v in c["key_values"].items() if not k.startswith("rigid")))
    if bad:
        print("REFUSED: the formulations disagree; nothing is written or verified")
        return 1
    if args.write:
        for path, data in files.items():
            if path in FIXED_FILES:
                continue
            target = OUT / path
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(data)
        print(f"wrote {len(files) - len([f for f in FIXED_FILES if f in files])} files")
        return 0
    failures = [p for p, d in files.items() if not (OUT / p).exists() or (OUT / p).read_bytes() != d]
    committed = {str(p.relative_to(OUT)) for p in OUT.rglob("*") if p.is_file() and "__pycache__" not in p.parts}
    extra = sorted(committed - set(files))
    if failures or extra:
        print("MISMATCH: " + ", ".join(failures + [f"unexpected {e}" for e in extra]))
        return 1
    print(f"verified {len(files)} files byte for byte (SHA256SUMS included)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
