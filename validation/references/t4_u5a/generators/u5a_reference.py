#!/usr/bin/env python3
"""T4-U5a independent references: untied expansion-joint pressure thrust.

Standard-library Python 3 only. This generator never imports, reads or runs
product code; it reads nothing but its own fixture definitions below and the
files it wrote. Every value is derived in exact arithmetic over Q(pi): numbers
are rational functions of pi with rational coefficients, normalized (gcd
removed, monic denominator) so that equality is exact.

Physics source: the Joint Reference contract (JR, archived at 11718e61a2^,
`CORRECTNESS_DESIGN/JOINT_REFERENCE/CONTRACT.md` sections 2, 4 and 6) and its
independent refutation (sections 2 and 3), re-derived here from the axial
free bodies and the straight-pipe wall law Nw = EA(eps - eps_T) + 2 nu p Ai.
No equation is taken from an extracted equation artifact (DEC-043).

Modes:
  python3 -I u5a_reference.py            verify: rebuild every request and the
                                          oracle in memory and compare them byte
                                          for byte with the committed files
  python3 -I u5a_reference.py --write    write requests/*.json and oracle.json
"""
import argparse
import copy
import hashlib
import json
import sys
from decimal import Decimal, getcontext
from fractions import Fraction as F
from math import gcd
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

# --------------------------------------------------------------------------
# Q(pi): polynomials (tuples of Fractions, lowest degree first) and their
# quotients.


def ptrim(p):
    p = list(p)
    while p and p[-1] == 0:
        p.pop()
    return tuple(p)


def padd(a, b):
    n = max(len(a), len(b))
    return ptrim([(a[i] if i < len(a) else F(0)) + (b[i] if i < len(b) else F(0)) for i in range(n)])


def pmul(a, b):
    if not a or not b:
        return ()
    out = [F(0)] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            out[i + j] += x * y
    return ptrim(out)


def pdivmod(a, b):
    a = list(ptrim(a))
    b = ptrim(b)
    if not b:
        raise ZeroDivisionError("polynomial division by zero")
    q = [F(0)] * max(len(a) - len(b) + 1, 1)
    while a and len(a) >= len(b):
        d = len(a) - len(b)
        c = a[-1] / b[-1]
        q[d] = c
        for i, y in enumerate(b):
            a[i + d] -= c * y
        a = list(ptrim(a))
    return ptrim(q), ptrim(a)


def pgcd(a, b):
    a, b = ptrim(a), ptrim(b)
    while b:
        a, b = b, pdivmod(a, b)[1]
    return tuple(x / a[-1] for x in a)


class Q:
    """An exact element of Q(pi)."""

    __slots__ = ("n", "d")

    def __init__(self, n, d=(F(1),)):
        n, d = ptrim(n), ptrim(d)
        if not d:
            raise ZeroDivisionError("Q(pi) division by zero")
        if not n:
            self.n, self.d = (), (F(1),)
            return
        g = pgcd(n, d)
        if len(g) > 1:
            n, d = pdivmod(n, g)[0], pdivmod(d, g)[0]
        lead = d[-1]
        self.n = tuple(x / lead for x in n)
        self.d = tuple(x / lead for x in d)

    @staticmethod
    def of(x):
        if isinstance(x, Q):
            return x
        return Q((F(x),))

    def __add__(self, o):
        o = Q.of(o)
        return Q(padd(pmul(self.n, o.d), pmul(o.n, self.d)), pmul(self.d, o.d))

    __radd__ = __add__

    def __neg__(self):
        return Q(tuple(-x for x in self.n), self.d)

    def __sub__(self, o):
        return self + (-Q.of(o))

    def __rsub__(self, o):
        return Q.of(o) - self

    def __mul__(self, o):
        o = Q.of(o)
        return Q(pmul(self.n, o.n), pmul(self.d, o.d))

    __rmul__ = __mul__

    def __truediv__(self, o):
        o = Q.of(o)
        if not o.n:
            raise ZeroDivisionError("Q(pi) division by zero")
        return Q(pmul(self.n, o.d), pmul(self.d, o.n))

    def __rtruediv__(self, o):
        return Q.of(o) / self

    def __eq__(self, o):
        o = Q.of(o)
        return self.n == o.n and self.d == o.d

    def __hash__(self):
        return hash((self.n, self.d))

    def is_zero(self):
        return not self.n

    def decimal(self):
        def ev(p):
            total = Decimal(0)
            power = Decimal(1)
            for c in p:
                total += Decimal(c.numerator) / Decimal(c.denominator) * power
                power *= PI_DECIMAL
            return total

        return ev(self.n) / ev(self.d)

    def exact(self):
        coeffs = list(self.n) + list(self.d)
        lcm = 1
        for c in coeffs:
            lcm = lcm * c.denominator // gcd(lcm, c.denominator)
        num = [int(c * lcm) for c in self.n]
        den = [int(c * lcm) for c in self.d]
        g = 0
        for c in num + den:
            g = gcd(g, abs(c))
        num = [c // g for c in num]
        den = [c // g for c in den]

        def poly(p):
            terms = []
            for i, c in enumerate(p):
                if c == 0:
                    continue
                mag = abs(c)
                if i == 0:
                    body = str(mag)
                elif i == 1:
                    body = "pi" if mag == 1 else f"{mag}*pi"
                else:
                    body = f"pi^{i}" if mag == 1 else f"{mag}*pi^{i}"
                terms.append(("-" if c < 0 else "+", body))
            if not terms:
                return "0", 1
            text = ("-" if terms[0][0] == "-" else "") + terms[0][1]
            for sign, body in terms[1:]:
                text += f" {sign} {body}"
            return text, len(terms)

        if not num:
            return "0"
        ntext, nterms = poly(num)
        if den == [1]:
            return ntext
        dtext, dterms = poly(den)
        if nterms > 1:
            ntext = f"({ntext})"
        if dterms > 1:
            dtext = f"({dtext})"
        return f"{ntext}/{dtext}"


getcontext().prec = 80


def _pi_decimal():
    # Machin: pi = 16 atan(1/5) - 4 atan(1/239), each by its alternating series.
    def atan_inv(x):
        x = Decimal(x)
        total, term, n, sign = Decimal(0), 1 / x, 1, 1
        x2 = x * x
        while True:
            delta = term / n
            if delta < Decimal(10) ** -78:
                break
            total += sign * delta
            term /= x2
            n += 2
            sign = -sign
        return total

    return 16 * atan_inv(5) - 4 * atan_inv(239)


PI_DECIMAL = _pi_decimal()
PI = Q((F(0), F(1)))
ZERO = Q.of(0)


def num(x):
    """An oracle number: exact form, 30-digit decimal and binary64 value."""
    x = Q.of(x)
    d = x.decimal()
    return {
        "exact": x.exact(),
        "decimal": format(d, ".30g") if not x.is_zero() else "0",
        "value": float(format(d, ".40e")) if not x.is_zero() else 0.0,
    }


def vec(v):
    return [num(c) for c in v]


# --------------------------------------------------------------------------
# Small exact linear algebra over Q(pi).


def solve(a, b):
    n = len(b)
    m = [list(row) + [b[i]] for i, row in enumerate(a)]
    for col in range(n):
        pivot = next(r for r in range(col, n) if not m[r][col].is_zero())
        m[col], m[pivot] = m[pivot], m[col]
        for r in range(n):
            if r != col and not m[r][col].is_zero():
                factor = m[r][col] / m[col][col]
                m[r] = [x - factor * y for x, y in zip(m[r], m[col])]
    return [m[i][n] / m[i][i] for i in range(n)]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def dot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def matvec(m, v):
    return [dot(row, v) for row in m]


def column(m, c):
    return [m[0][c], m[1][c], m[2][c]]


def matmul(a, b):
    return [[sum((a[i][k] * b[k][j] for k in range(3)), F(0)) for j in range(3)] for i in range(3)]


# --------------------------------------------------------------------------
# The invented fixture (JR J3's values; synthetic SI controls, not catalog,
# manufacturer, library or code-rule data).

PROVENANCE = "invented T4-U5a reference fixture (JR J3 values); not library, catalog, manufacturer or code-rule data"
E = F(200000000000)
NU = F(3, 10)
ALPHA = F(12, 1000000)
OD = F(6, 100)
WALL = F(5, 1000)
R_IN = OD / 2 - WALL  # 1/40 m
R_OUT = OD / 2  # 3/100 m
P_PA = F(200000)
AE = F(4, 1000)
K_AXIAL = F(200000)
K_OTHER = [F(80000), F(80000), F(600), F(900), F(900)]
STUB = F(2)
JOINT = F(3, 10)
IDENTITY = [[F(1), F(0), F(0)], [F(0), F(1), F(0)], [F(0), F(0), F(1)]]
# A proper rational rotation (quaternion (1, 2, 2, 4) / 5): every axis skew.
ROTATION = [[F(-15, 25), F(0), F(20, 25)], [F(16, 25), F(-15, 25), F(12, 25)], [F(12, 25), F(20, 25), F(9, 25)]]
FLIP = [[F(-1), F(0), F(0)], [F(0), F(1), F(0)], [F(0), F(0), F(-1)]]  # JR's J


def config(cid, **kw):
    base = {
        "id": cid,
        "nodes": [("node:A", F(0)), ("node:B", STUB), ("node:C", STUB + JOINT), ("node:D", 2 * STUB + JOINT)],
        "pipes": [("pipe:L", "node:A", "node:B"), ("pipe:J", "node:B", "node:C"), ("pipe:R", "node:C", "node:D")],
        "replaced": "pipe:J",
        "connector_ends": ("node:B", "node:C"),
        "connector_flip": False,
        "k_axial": K_AXIAL,
        "q_ref_x": F(0),
        "pressure_model": "constant_effective_area_reference_v1",
        "effective_area": AE,
        "anchors": ["node:A", "node:D"],
        "region": {"pipes": ["pipe:L", "pipe:R"], "connector": True, "p": P_PA,
                   "terminals": [("node:A", "transfers_to_wall"), ("node:D", "transfers_to_wall")]},
        "thermal": {},
        "nu": NU,
        "rotation": IDENTITY,
        "origin": [F(0), F(0), F(0)],
    }
    base.update(kw)
    return base


REMOTE_NODES = [("node:A", F(0)), ("node:B", STUB), ("node:C", STUB + JOINT), ("node:D", 2 * STUB + JOINT),
                ("node:E", 3 * STUB + JOINT)]
REMOTE_PIPES = [("pipe:L", "node:A", "node:B"), ("pipe:J", "node:B", "node:C"), ("pipe:R", "node:C", "node:D"),
                ("pipe:X", "node:D", "node:E")]
REMOTE_REGION = {"pipes": ["pipe:X"], "connector": False, "p": P_PA,
                 "terminals": [("node:D", "transfers_to_wall"), ("node:E", "transfers_to_wall")]}

FREE = ["node:A"]
CONFIGS = [
    (config("J3-1"), "JR J3 row 1: anchored outer ends, untied"),
    (config("J3-1T", thermal={"pipe:L": F(10), "pipe:R": F(10)}),
     "J3 row 1 with a uniform stub temperature change of 10 degC (JR's thermal anchored relation, q_ref = 0)"),
    (config("J3-1Q", q_ref_x=F(1, 1000)),
     "J3 row 1 with a prestressed joint, q_ref = (0.001 m, 0, 0, 0, 0, 0) (JR's anchored relation, eps_T = 0)"),
    (config("J3-1TQ", thermal={"pipe:L": F(10), "pipe:R": F(10)}, q_ref_x=F(1, 1000)),
     "JR's thermal anchored relation (k + kp/2) q = Pe - (1 - 2 nu) Pi - EA eps_T + k q_ref, both terms present"),
    (config("J3-1N", nu=F(0)), "J3 row 1 with nu = 0 (no Poisson eigenload: h = 0)"),
    (config("J3-1K", k_axial=F(2000000000)), "J3 row 1, stiff joint limit (k = 2e9 N/m)"),
    (config("J3-1F", k_axial=F(2000)), "J3 row 1, soft joint limit (k = 2e3 N/m)"),
    (config("J3-1R", rotation=ROTATION, origin=[F(10), F(20), F(5)]),
     "J3 row 1 rotated by a proper rational rotation (every axis skew) and translated (whole-model covariance)"),
    (config("J3-1V", connector_ends=("node:C", "node:B"), connector_flip=True),
     "J3 row 1 with the connector's ends reversed (end_i at C, Q' = Q J, J = diag(-1, 1, -1)): face-sign control"),
    (config("J3-1W", pipes=[("pipe:L", "node:A", "node:B"), ("pipe:J", "node:B", "node:C"), ("pipe:R", "node:D", "node:C")]),
     "J3 row 1 with pipe R authored from D to C (traversal against authoring): face-cap sign control"),
    (config("J3-AE1", effective_area=PI / 1600),
     "area-equality control, anchored: Ae = Ai exactly, so the joint correction p(Ae - Ai) is zero"),
    (config("J3-0", region={"pipes": ["pipe:L", "pipe:R"], "connector": True, "p": F(0),
                            "terminals": [("node:A", "transfers_to_wall"), ("node:D", "transfers_to_wall")]}),
     "no-pressure control: p = 0 in the same region, everything zero"),
    (config("J3-3", anchors=FREE), "JR J3 row 3: free closed outer ends with one axial datum (A), untied"),
    (config("J3-3S", anchors=FREE, region={"pipes": ["pipe:L", "pipe:R"], "connector": True, "p": P_PA,
                                          "terminals": [("node:A", "transfers_to_wall"),
                                                        ("node:D", "separately_supported_or_compensated")]}),
     "J3 row 3 with the closure at D separately supported (its cap is reacted outside the pipe solve)"),
    (config("J3-AE3", anchors=FREE, effective_area=PI / 1600),
     "area-equality control, free: Ae = Ai, the joint carries exactly Pi"),
    (config("J3-5U", anchors=FREE, k_axial=F(20200000)),
     "JR J3 row 5 in its untied form: the free closed chain with an untied joint of stiffness k + k_rod = 2.02e7 N/m "
     "(same q as row 5; U5b splits it into Tb and Tt)"),
    (config("J3-5UA", k_axial=F(20200000)), "the row-5 stiffness (2.02e7 N/m) untied, anchored outer ends"),
    (config("J3-R", nodes=REMOTE_NODES, pipes=REMOTE_PIPES, anchors=["node:A", "node:E"], region=REMOTE_REGION,
            pressure_model="unpressurized", effective_area=None),
     "remote region: an unpressurized joint beside a pressurized region that does not touch it (the blanket "
     "refusal lifted); the joint carries the remote cap's share"),
    (config("J3-RP", nodes=REMOTE_NODES, pipes=REMOTE_PIPES, anchors=["node:A", "node:E"], region=REMOTE_REGION),
     "J3-R with a constant-effective-area joint outside every region: p = 0 at the joint, Pe = 0, S = Tb"),
]


# --------------------------------------------------------------------------
# Derived geometry and the axial model.


def section():
    ai = PI * R_IN * R_IN
    a_s = PI * (R_OUT * R_OUT - R_IN * R_IN)
    return ai, a_s


def geometry(cfg):
    rot = cfg["rotation"]
    n = column(rot, 0)
    s = dict(cfg["nodes"])
    pos = {nid: [cfg["origin"][k] + s[nid] * n[k] for k in range(3)] for nid in s}
    axes = matmul(rot, FLIP) if cfg["connector_flip"] else rot
    return n, s, pos, axes


def sigma(s, a, b):
    return 1 if s[b] > s[a] else -1


def build(cfg, representation="B", mutant=None):
    """The axial stiffness, the full and pressure-only RHS (along n) and data."""
    ai, a_s = section()
    nu = cfg["nu"]
    n, s, pos, axes = geometry(cfg)
    ids = [nid for nid, _ in cfg["nodes"]]
    index = {nid: i for i, nid in enumerate(ids)}
    size = len(ids)
    kmat = [[ZERO] * size for _ in range(size)]
    f_pressure = [ZERO] * size
    f_other = [ZERO] * size
    region = cfg["region"]
    p = Q.of(region["p"]) if region else ZERO
    pi_force = p * ai
    h = 2 * nu * pi_force
    ea = E * a_s
    pipes = {pid: (a, b) for pid, a, b in cfg["pipes"]}
    built = [pid for pid in pipes if pid != cfg["replaced"]]
    in_region = set(region["pipes"]) if region else set()

    def stiff(a, b, k):
        i, j = index[a], index[b]
        kmat[i][i] += k
        kmat[j][j] += k
        kmat[i][j] -= k
        kmat[j][i] -= k

    def outward(pid, node):
        a, b = pipes[pid]
        sp = sigma(s, a, b)
        return -sp if node == a else sp

    for pid in built:
        a, b = pipes[pid]
        length = abs(s[b] - s[a])
        stiff(a, b, ea / length)
        sp = sigma(s, a, b)
        delta_t = cfg["thermal"].get(pid)
        if delta_t:
            thermal = ea * ALPHA * delta_t
            f_other[index[a]] -= thermal * sp
            f_other[index[b]] += thermal * sp
        if pid in in_region:
            f_pressure[index[a]] += h * sp
            f_pressure[index[b]] -= h * sp
            if representation == "A":
                f_pressure[index[a]] += pi_force * outward(pid, a)
                f_pressure[index[b]] += pi_force * outward(pid, b)
    end_i, end_j = cfg["connector_ends"]
    k = Q.of(cfg["k_axial"])
    stiff(end_i, end_j, k)
    qx = column(axes, 0)
    sc = 1 if dot(qx, n) > 0 else -1
    reference = k * cfg["q_ref_x"]
    f_other[index[end_i]] -= reference * sc
    f_other[index[end_j]] += reference * sc
    joint_in_region = bool(region and region["connector"])
    pe = p * Q.of(cfg["effective_area"]) if (joint_in_region and cfg["effective_area"] is not None) else ZERO
    moments = None
    if joint_in_region:
        if representation == "A":
            # JR section 4: the joint's generalized pressure load B^T gp with
            # gp = [p Ae, 0, 0, 0, 0, 0], from JR's B (zero offsets here).
            fi, mi, fj, mj = bt_gp(pos[end_i], pos[end_j], [ZERO] * 3, [ZERO] * 3, axes, pe)
            f_pressure[index[end_i]] += dot(fi, n)
            f_pressure[index[end_j]] += dot(fj, n)
            moments = (mi, mj)
        else:
            adjacent = {}
            for pid in sorted(in_region):
                a, b = pipes[pid]
                for node in (a, b):
                    if node in (end_i, end_j):
                        adjacent[node] = pid
            ae_part = pe
            if mutant == "ae_dropped":
                ae_part = ZERO
            face_area = PI * OD * OD / 4 if mutant == "bore_from_od" else ai
            face = p * face_area
            flip = -1 if mutant == "sign_flip" else 1
            f_pressure[index[end_i]] += flip * (-ae_part * sc)
            f_pressure[index[end_j]] += flip * (ae_part * sc)
            if mutant != "double_count":
                for node in (end_i, end_j):
                    f_pressure[index[node]] += flip * face * outward(adjacent[node], node)
    if region:
        for node, disposition in region["terminals"]:
            pid = next(m for m in sorted(in_region) if node in pipes[m])
            if representation == "B" and disposition == "transfers_to_wall":
                f_pressure[index[node]] += pi_force * outward(pid, node)
            if representation == "A" and disposition != "transfers_to_wall":
                # the remote closure takes the member's mathematical cap
                f_pressure[index[node]] -= pi_force * outward(pid, node)
    return {
        "k": kmat, "f_pressure": f_pressure, "f_other": f_other, "ids": ids, "index": index, "s": s, "n": n,
        "pos": pos, "axes": axes, "sc": sc, "pe": pe, "pi_force": pi_force, "h": h, "ea": ea, "p": p,
        "pipes": pipes, "built": built, "in_region": in_region, "moments": moments, "ai": ai,
    }


def bt_gp(xi, xj, ai, aj, axes, pe):
    """JR's B^T gp: Fi = -F, Fj = F, Mi = -(ai + r/2) x F, Mj = (aj - r/2) x F."""
    r = [xj[k] + aj[k] - xi[k] - ai[k] for k in range(3)]
    force = [axes[k][0] * pe for k in range(3)]
    half = [c / 2 for c in r]
    mi = [-c for c in cross([ai[k] + half[k] for k in range(3)], force)]
    mj = cross([aj[k] - half[k] for k in range(3)], force)
    return [-c for c in force], mi, force, mj


def evaluate(cfg, mutant=None):
    model = build(cfg, "B", mutant)
    ids, index = model["ids"], model["index"]
    free = [nid for nid in ids if nid not in cfg["anchors"]]
    f = [model["f_pressure"][i] + model["f_other"][i] for i in range(len(ids))]
    kf = [[model["k"][index[a]][index[b]] for b in free] for a in free]
    sol = solve(kf, [f[index[a]] for a in free])
    u = {nid: ZERO for nid in ids}
    for nid, value in zip(free, sol):
        u[nid] = value
    s, n = model["s"], model["n"]
    end_i, end_j = cfg["connector_ends"]
    q = (u[end_j] - u[end_i]) * model["sc"]
    k = Q.of(cfg["k_axial"])
    deformation = q - cfg["q_ref_x"]
    tb = k * deformation
    pe = model["pe"]
    out = {"u": u, "q": q, "deformation": deformation, "tb": tb, "pe": pe, "s_joint": tb - pe, "model": model}
    pipes = {}
    for pid in model["built"]:
        a, b = model["pipes"][pid]
        sp = sigma(s, a, b)
        length = abs(s[b] - s[a])
        delta = (u[b] - u[a]) * sp
        nw = model["ea"] / length * delta
        if cfg["thermal"].get(pid):
            nw -= model["ea"] * ALPHA * cfg["thermal"][pid]
        pressurized = pid in model["in_region"]
        if pressurized:
            nw += model["h"]
        pipes[pid] = {"elongation": delta, "nw": nw, "s": nw - (model["pi_force"] if pressurized else ZERO)}
    out["pipes"] = pipes
    reactions = {}
    for nid in cfg["anchors"]:
        i = index[nid]
        reactions[nid] = sum((model["k"][i][index[b]] * u[b] for b in ids), ZERO) - f[i]
    out["reactions"] = reactions
    return out


# --------------------------------------------------------------------------
# Requests (product preview model 0.5.0 with U5a fields, explicit CA 0 m and EB shear).


def fl(x):
    return float(Q.of(x).decimal()) if isinstance(x, Q) else float(x)


def quantity(value, unit):
    return {"value": fl(value), "unit": unit}


def request(cfg):
    n, s, pos, axes = geometry(cfg)
    rot = cfg["rotation"]
    yref = column(rot, 1)
    nodes = [{"id": nid, "position": {"x": fl(pos[nid][0]), "y": fl(pos[nid][1]), "z": fl(pos[nid][2])},
              "provenance": PROVENANCE} for nid, _ in cfg["nodes"]]
    pipes = [{"id": pid, "from": a, "to": b,
              "section": {"outside_diameter": quantity(OD, "m"), "wall_thickness": quantity(WALL, "m"),
                          "corrosion_allowance": quantity(0, "m")},
              "material": "material:invented-steel",
              "y_reference": {"x": fl(yref[0]), "y": fl(yref[1]), "z": fl(yref[2])},
              "provenance": PROVENANCE} for pid, a, b in cfg["pipes"]]
    supports = [{"id": "support:" + nid.split(":")[1], "node": nid, "family": "anchor",
                 "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROVENANCE}
                for nid in cfg["anchors"]]
    end_i, end_j = cfg["connector_ends"]
    identity_rows = [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
    zero_offset = {"x": 0, "y": 0, "z": 0, "unit": "m"}
    upper = [0.0] * 21
    diagonal = [cfg["k_axial"]] + K_OTHER
    position = 0
    for row in range(6):
        for col in range(row, 6):
            if row == col:
                upper[position] = fl(diagonal[row])
            position += 1
    if cfg["pressure_model"] == "unpressurized":
        pressure_model = {"kind": "unpressurized"}
    else:
        pressure_model = {"kind": "constant_effective_area_reference_v1",
                          "effective_area": quantity(cfg["effective_area"], "m^2"),
                          "pressure_basis": "internal_differential_zero_external_v1",
                          "source_reference": "invented T4-U5a effective area (JR J3); not manufacturer data"}
    stress_free = cfg["q_ref_x"] == 0
    connector = {
        "version": "1.0.0",
        "motion_basis": "symmetric_midpoint_small_rotation_v1",
        "end_i": {"node_ref": end_i, "initial_node_axes_global": identity_rows, "offset_local": zero_offset},
        "end_j": {"node_ref": end_j, "initial_node_axes_global": identity_rows, "offset_local": zero_offset},
        "connector_axes_global": [[fl(axes[r][c]) for c in range(3)] for r in range(3)],
        "installed_reference_temperature": {"value": 20, "unit": "degC"},
        "temperature_applicability": "fixed_installed_parameters_v1",
        "reference_state": "stress_free" if stress_free else "prestressed",
        "q_ref": {"translation": {"x": fl(cfg["q_ref_x"]), "y": 0, "z": 0, "unit": "m"},
                  "rotation": {"x": 0, "y": 0, "z": 0, "unit": "rad"}},
        "stiffness": {
            "version": "1.0.0", "representation": "scaled_work_coefficients_v1",
            "translation_scale": {"value": 1, "unit": "m"}, "rotation_scale": {"value": 1, "unit": "rad"},
            "coefficient_unit": "N*m", "upper_triangle": upper,
            "coordinate_order": ["tx", "ty", "tz", "rx", "ry", "rz"],
            "provenance": {"source_reference": "invented T4-U5a uncoupled 6x6 (JR J3 axial rate)",
                           "measurement_restraints": "invented; symmetric midpoint basis declared by the author",
                           "basis_transform_reference": "none (authored directly in the midpoint basis)"}},
        "topology": {"type": "replaces_span", "span_ref": cfg["replaced"]},
        "calibration": {"kind": "constant_structural_elasticity_v1", "includes_pressure_dependent_tangent": False,
                        "installed_geometry": "authored model geometry"},
        "hardware": {"kind": "untied"},
        "pressure_model": pressure_model,
        "provenance": {"source_reference": PROVENANCE, "measurement_restraints": "invented",
                       "basis_transform_reference": "none",
                       "validity_statement": "invented analytical control; no applicability claim"},
    }
    component = {"id": "component:C-J", "label": "Invented expansion joint (objective connector)",
                 "kind": "expansion_joint", "node": end_j, "provenance": PROVENANCE,
                 "objective_connector": connector}
    material = {"id": "material:invented-steel", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
                "elastic_modulus": quantity(E, "Pa"), "poisson_ratio": quantity(cfg["nu"], "1"),
                "thermal_expansion_coefficient": quantity(ALPHA, "1/degC"), "provenance": PROVENANCE}
    loads = [{"id": "load:T-" + pid.split(":")[1], "category": "thermal",
              "target": {"type": "element", "pipe": pid}, "direction": "global_z",
              "magnitude": quantity(dt, "degC"), "dimension": "temperature_interval", "provenance": PROVENANCE}
             for pid, dt in sorted(cfg["thermal"].items())]
    regions = []
    region = cfg["region"]
    if region:
        entry = {"id": "region:U5A", "member_pipe_ids": list(region["pipes"])}
        if region["connector"]:
            entry["member_connector_ids"] = ["component:C-J"]
        entry.update({"pressure_basis": "internal_differential_zero_external_v1",
                      "pressure": quantity(region["p"], "Pa"),
                      "terminals": [{"node_ref": node, "closure_transfer": disposition, "provenance": PROVENANCE}
                                    for node, disposition in region["terminals"]],
                      "provenance": PROVENANCE})
        regions.append(entry)
    model = {
        "schema_version": "0.5.0",
        "document_kind": "openpipestress.product_preview.model",
        "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"},
        "project": {"id": "project:invented-t4-u5a-" + cfg["id"].lower(),
                    "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa",
                              "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
                            "rule_check": "not_performed_user_rule_inputs_missing",
                            "professional_acceptance": "not_provided"},
        "nodes": nodes, "pipe_segments": pipes, "supports": supports, "components": [component],
        "materials": [material],
        "load_cases": [{"id": "load:U5A", "primitive_loads": loads, "pressure_regions": regions,
                        "provenance": PROVENANCE}],
        "combinations": [],
    }
    return {"model": model, "materials": []}


def dumps(value):
    return json.dumps(value, indent=1, sort_keys=False) + "\n"


# --------------------------------------------------------------------------
# Oracle.


def closed_forms(cfg, out):
    """JR's closed forms, checked against the general solve."""
    ai, a_s = section()
    ea = E * a_s
    kp = ea / STUB
    pi_force = out["model"]["pi_force"]
    h = out["model"]["h"]
    k = Q.of(cfg["k_axial"])
    pe = out["pe"]
    eps = ALPHA * cfg["thermal"].get("pipe:L", 0)
    cid = cfg["id"]
    checks = {}
    if cid.startswith("J3-1") or cid in ("J3-AE1", "J3-5UA", "J3-0"):
        expected = (pe - (1 - 2 * cfg["nu"]) * pi_force - ea * eps + k * cfg["q_ref_x"]) / (k + kp / 2)
        checks["anchored relation (k + kp/2) q = Pe - (1 - 2 nu) Pi - EA eps_T + k q_ref"] = expected
    if cid in ("J3-3", "J3-AE3", "J3-5U"):
        checks["free relation q = Pe / k"] = pe / k
    if cid == "J3-3S":
        checks["remote closure at D: q = (Pe - Pi) / k"] = (pe - pi_force) / k
    if cid == "J3-1":
        checks["JR literal q = (800 - 50 pi)/(200000 + 13750000 pi)"] = (800 - 50 * PI) / (200000 + 13750000 * PI)
    if cid == "J3-3":
        checks["JR literal q = 0.004"] = Q.of(F(4, 1000))
    result = {}
    for name, value in checks.items():
        if not value == out["q"]:
            raise AssertionError(f"{cid}: closed form {name} disagrees with the solve")
        result[name] = num(value)
    return result


def case_entry(cfg, description, request_bytes):
    out = evaluate(cfg)
    model = out["model"]
    n = model["n"]
    region = cfg["region"]
    joint_in_region = bool(region and region["connector"])
    a_model = build(cfg, "A")
    if not all(x == y for x, y in zip(a_model["f_pressure"], model["f_pressure"])):
        raise AssertionError(f"{cfg['id']}: representation A and B pressure RHS differ")
    if a_model["moments"] is not None and not all(c.is_zero() for m in a_model["moments"] for c in m):
        raise AssertionError(f"{cfg['id']}: B^T gp has nonzero moments for a coaxial zero-offset joint")
    # the A-assembled system solves to the same state
    ids, index = model["ids"], model["index"]
    free = [nid for nid in ids if nid not in cfg["anchors"]]
    fa = [a_model["f_pressure"][i] + a_model["f_other"][i] for i in range(len(ids))]
    sol_a = solve([[a_model["k"][index[a]][index[b]] for b in free] for a in free], [fa[index[a]] for a in free])
    if not all(sol_a[i] == out["u"][nid] for i, nid in enumerate(free)):
        raise AssertionError(f"{cfg['id']}: representation A solves to a different state")
    if joint_in_region:
        end_i, end_j = cfg["connector_ends"]
        for node in (end_i, end_j):
            for pid in sorted(model["in_region"]):
                if node in model["pipes"][pid] and not out["pipes"][pid]["s"] == out["s_joint"]:
                    raise AssertionError(f"{cfg['id']}: interface identity S_pipe = S_joint fails at {node}")
    axes = model["axes"]
    qx = column(axes, 0)
    end_force = [qx[k] * out["tb"] for k in range(3)]
    entry = {
        "description": description,
        "request": f"requests/{cfg['id']}.request.json",
        "request_sha256": hashlib.sha256(request_bytes).hexdigest(),
        "connector": "component:C-J",
        "connector_pressure_model": cfg["pressure_model"],
        "connector_in_region": joint_in_region,
        "axis_global": vec(n),
        "scalars": {
            "Ai_m2": num(model["ai"]),
            "Pi_n": num(model["pi_force"]),
            "h_2nuPi_n": num(model["h"]),
            "q_m": num(out["q"]),
            "qt_x_minus_q_ref_m": num(out["deformation"]),
            "Tb_gt_x_n": num(out["tb"]),
        },
        "pipes": {pid: {"elongation_m": num(v["elongation"]), "Nw_n": num(v["nw"]), "S_n": num(v["s"])}
                  for pid, v in out["pipes"].items()},
        "nodes": {nid: {"axial_displacement_m": num(out["u"][nid]),
                        "displacement_global_m": vec([out["u"][nid] * c for c in n]),
                        "rotation_global_rad": vec([ZERO] * 3)} for nid in model["ids"]},
        "reactions_support_on_model": {
            "support:" + nid.split(":")[1]: {"force_global_n": vec([r * c for c in n]),
                                             "moment_global_nm": vec([ZERO] * 3)}
            for nid, r in out["reactions"].items()},
        "connector_rows": {
            "connector_generalized_translation_v1": vec([out["deformation"], ZERO, ZERO]),
            "connector_generalized_rotation_v1": vec([ZERO] * 3),
            "connector_generalized_force_v1": vec([out["tb"], ZERO, ZERO]),
            "connector_generalized_moment_v1": vec([ZERO] * 3),
            "connector_endpoint_force_v1:end_i": vec([-c for c in end_force]),
            "connector_endpoint_moment_v1:end_i": vec([ZERO] * 3),
            "connector_endpoint_force_v1:end_j": vec(end_force),
            "connector_endpoint_moment_v1:end_j": vec([ZERO] * 3),
        },
        "pressure_rhs": {
            "representation_B_axial_n": {nid: num(model["f_pressure"][model["index"][nid]]) for nid in model["ids"]},
            "representation_A_axial_n": {nid: num(a_model["f_pressure"][a_model["index"][nid]]) for nid in model["ids"]},
            "representation_A_equals_B_exactly": True,
            "representation_A_solves_to_the_same_state_exactly": True,
            "global_n": {nid: vec([model["f_pressure"][model["index"][nid]] * c for c in n]) for nid in model["ids"]},
        },
        "closed_forms": closed_forms(cfg, out),
    }
    if cfg["pressure_model"] != "unpressurized":
        entry["scalars"]["Pe_n"] = num(out["pe"])
        entry["scalars"]["S_joint_n"] = num(out["s_joint"])
        entry["connector_rows"]["connector_pressure_force_v1"] = vec([out["pe"], ZERO, ZERO])
        entry["connector_rows"]["connector_effective_axial_force_v1"] = vec([out["s_joint"], ZERO, ZERO])
    if joint_in_region:
        entry["scalars"]["joint_area_difference_force_n"] = num(out["pe"] - model["pi_force"])
        entry["interface_identity"] = "S_pipe = S_joint at both joint faces (checked exactly)"
    if cfg["anchors"] == FREE:
        a_id, last = model["ids"][0], model["ids"][-1]
        entry["scalars"]["outer_end_separation_change_m"] = num(out["u"][last] - out["u"][a_id])
    return entry


def mutants():
    """Ledger mutants of the joint term (producer-level): expected wrong values."""
    result = {}
    names = {
        "double_count": "JR J4: terminal caps plus the full joint pair p Ae (the joint_face_cap summands dropped)",
        "sign_flip": "the joint term with its i/j faces exchanged (the pair -(p(Ae - Ai)) reversed)",
        "bore_from_od": "the face caps formed from the outside diameter, pi OD^2/4, instead of the bore",
        "ae_dropped": "the joint_effective_area summands dropped (only the face caps remain)",
    }
    for base in ("J3-1", "J3-3"):
        cfg = next(c for c, _ in CONFIGS if c["id"] == base)
        correct = evaluate(cfg)
        block = {}
        for mutant, text in names.items():
            out = evaluate(cfg, mutant)
            adjacent = out["pipes"]["pipe:L"]["s"]
            entry = {
                "mutant": text,
                "q_m": num(out["q"]),
                "Tb_n": num(out["tb"]),
                "S_joint_n": num(out["s_joint"]),
                "S_pipe_L_n": num(adjacent),
                "Nw_L_n": num(out["pipes"]["pipe:L"]["nw"]),
                "reaction_A_axial_n": num(out["reactions"]["node:A"]),
                "differs_from_correct": {
                    "q": not out["q"] == correct["q"],
                    "Tb": not out["tb"] == correct["tb"],
                    "Nw_L": not out["pipes"]["pipe:L"]["nw"] == correct["pipes"]["pipe:L"]["nw"],
                    "reaction_A": not out["reactions"]["node:A"] == correct["reactions"]["node:A"],
                    "interface_identity_S_pipe_eq_S_joint": not adjacent == out["s_joint"],
                },
            }
            block[mutant] = entry
        result[base] = block
    # JR's literal J4 values, checked
    ai, a_s = section()
    pi_force = P_PA * ai
    h = 2 * NU * pi_force
    kp = E * a_s / STUB
    jr = {
        "J3-1": ((h + P_PA * AE) / (K_AXIAL + kp / 2), "q_wrong = (h + Pe)/(k + kp/2)"),
        "J3-3": ((P_PA * AE + pi_force) / K_AXIAL, "q_wrong = (Pe + Pi)/k"),
    }
    for base, (value, text) in jr.items():
        if not evaluate(next(c for c, _ in CONFIGS if c["id"] == base), "double_count")["q"] == value:
            raise AssertionError(f"JR J4 literal disagrees on {base}")
        result[base]["double_count"]["jr_literal"] = {"relation": text, "value": num(value)}
    return result


def offset_demonstration():
    """Why a lateral attachment offset is refused: B^T gp gains end moments that
    the nodal pair p(Ae - Ai) cannot represent."""
    cfg = CONFIGS[0][0]
    n, s, pos, axes = geometry(cfg)
    pe = P_PA * AE
    lateral = [F(0), F(1, 100), F(0)]
    fi, mi, fj, mj = bt_gp(pos["node:B"], pos["node:C"], lateral, lateral, axes, Q.of(pe))
    return {
        "offsets_global_m": {"end_i": vec(lateral), "end_j": vec(lateral)},
        "bt_gp_end_i_moment_nm": vec(mi),
        "bt_gp_end_j_moment_nm": vec(mj),
        "meaning": "with equal lateral offsets 0.01 m the full joint pair p Ae carries end moments of magnitude "
                   "0.01 m x p Ae = 8 N*m about z, which the adjacent pipes' centreline face caps do not cancel; a nodal p(Ae - Ai) pair "
                   "would drop them, so U5a refuses nonzero attachment offsets on a pressurized joint",
    }


REFUSALS = [
    ("RV-01", "the region spans both joint faces but does not list the connector (region-chain edge missing)",
     [{"op": "remove", "path": "/model/load_cases/0/pressure_regions/0/member_connector_ids"}],
     ["JOINT_PRESSURE_INTERFACE_UNRESOLVED"], ["component:C-J"]),
    ("RV-02", "a region terminal at a joint face (region over pipe L only, terminals A and B)",
     [{"op": "replace", "path": "/model/load_cases/0/pressure_regions/0/member_pipe_ids", "value": ["pipe:L"]},
      {"op": "remove", "path": "/model/load_cases/0/pressure_regions/0/member_connector_ids"},
      {"op": "replace", "path": "/model/load_cases/0/pressure_regions/0/terminals/1/node_ref", "value": "node:B"}],
     ["JOINT_PRESSURE_INTERFACE_UNRESOLVED"], ["component:C-J"]),
    ("RV-03", "the region also lists the replaced span",
     [{"op": "replace", "path": "/model/load_cases/0/pressure_regions/0/member_pipe_ids",
       "value": ["pipe:L", "pipe:J", "pipe:R"]}],
     ["JOINT_PRESSURE_INTERFACE_UNRESOLVED"], ["component:C-J", "pipe:J"]),
    ("RV-04", "an unpressurized connector listed as a region edge",
     [{"op": "replace", "path": "/model/components/0/objective_connector/pressure_model",
       "value": {"kind": "unpressurized"}}],
     ["JOINT_PRESSURE_INTERFACE_UNRESOLVED"], ["component:C-J"]),
    ("RV-05a", "the effective area is absent",
     [{"op": "remove", "path": "/model/components/0/objective_connector/pressure_model/effective_area"}],
     ["JOINT_PRESSURE_AREA_MISSING"], ["component:C-J"]),
    ("RV-05b", "the effective area is zero",
     [{"op": "replace", "path": "/model/components/0/objective_connector/pressure_model/effective_area/value",
       "value": 0.0}],
     ["JOINT_PRESSURE_AREA_MISSING"], ["component:C-J"]),
    ("RV-05c", "the effective area has no source reference",
     [{"op": "remove", "path": "/model/components/0/objective_connector/pressure_model/source_reference"}],
     ["JOINT_PRESSURE_AREA_MISSING"], ["component:C-J"]),
    ("RV-06", "equal lateral attachment offsets (FK alignment still holds; the pressure faces leave the pipe axis)",
     [{"op": "replace", "path": "/model/components/0/objective_connector/end_i/offset_local/y", "value": 0.01},
      {"op": "replace", "path": "/model/components/0/objective_connector/end_j/offset_local/y", "value": 0.01}],
     ["JOINT_PRESSURE_INTERFACE_UNRESOLVED"], ["component:C-J"]),
    ("RV-06b", "an axial attachment offset at end j",
     [{"op": "replace", "path": "/model/components/0/objective_connector/end_j/offset_local/x", "value": 0.01}],
     ["JOINT_PRESSURE_INTERFACE_UNRESOLVED"], ["component:C-J"]),
    ("RV-07", "pipe R leaves the joint axis (node D moved 0.1 m in y): a straight-joint kink",
     [{"op": "replace", "path": "/model/nodes/3/position/y", "value": 0.1}],
     ["PRESSURE_REGION_NONCOLLINEAR"], ["region:U5A"]),
    ("RV-08", "unequal bore across the joint (pipe R wall 0.006 m)",
     [{"op": "replace", "path": "/model/pipe_segments/2/section/wall_thickness/value", "value": 0.006}],
     ["PRESSURE_REGION_BORE_MISMATCH"], ["region:U5A", "pipe:R"]),
    ("RV-09", "the joint faces in two regions at different pressures (each side its own region)",
     [{"op": "replace", "path": "/model/load_cases/0/pressure_regions",
       "value": [
           {"id": "region:U5A-L", "member_pipe_ids": ["pipe:L"], "member_connector_ids": ["component:C-J"],
            "pressure_basis": "internal_differential_zero_external_v1", "pressure": {"value": 200000.0, "unit": "Pa"},
            "terminals": [{"node_ref": "node:A", "closure_transfer": "transfers_to_wall", "provenance": PROVENANCE},
                          {"node_ref": "node:C", "closure_transfer": "transfers_to_wall", "provenance": PROVENANCE}],
            "provenance": PROVENANCE},
           {"id": "region:U5A-R", "member_pipe_ids": ["pipe:R"],
            "pressure_basis": "internal_differential_zero_external_v1", "pressure": {"value": 100000.0, "unit": "Pa"},
            "terminals": [{"node_ref": "node:C", "closure_transfer": "transfers_to_wall", "provenance": PROVENANCE},
                          {"node_ref": "node:D", "closure_transfer": "transfers_to_wall", "provenance": PROVENANCE}],
            "provenance": PROVENANCE}]}],
     ["JOINT_PRESSURE_INTERFACE_UNRESOLVED"], ["component:C-J"]),
    ("RV-11", "explicit axial rods with a pressurized joint (until U5b)",
     [{"op": "replace", "path": "/model/components/0/objective_connector/hardware",
       "value": {"kind": "explicit_axial_rods_v1", "rods": []}}],
     ["JOINT_HARDWARE_LAW_UNSUPPORTED"], ["component:C-J"]),
    ("RV-12", "an unknown pressure-model kind",
     [{"op": "replace", "path": "/model/components/0/objective_connector/pressure_model/kind",
       "value": "pressure_balanced_v1"}],
     ["JOINT_PRESSURE_INTERFACE_UNRESOLVED"], ["component:C-J"]),
    ("RV-13", "a pressure-model basis different from the region's",
     [{"op": "replace", "path": "/model/components/0/objective_connector/pressure_model/pressure_basis",
       "value": "internal_increment_zero_external_v1"}],
     ["JOINT_PRESSURE_INTERFACE_UNRESOLVED"], ["component:C-J"]),
    ("RV-14", "a thermal load on the replaced span (existing S20 refusal, kept)",
     [{"op": "add", "path": "/model/load_cases/0/primitive_loads/-",
       "value": {"id": "load:T-J", "category": "thermal", "target": {"type": "element", "pipe": "pipe:J"},
                 "direction": "global_z", "magnitude": {"value": 10.0, "unit": "degC"},
                 "dimension": "temperature_interval", "provenance": PROVENANCE}}],
     ["JOINT_REPLACED_SPAN_LOAD_UNOWNED"], ["component:C-J", "pipe:J", "load:T-J"]),
    ("RV-15", "member_connector_ids names a component that is not an objective connector of the model",
     [{"op": "replace", "path": "/model/load_cases/0/pressure_regions/0/member_connector_ids",
       "value": ["component:C-MISSING"]}],
     ["JOINT_PRESSURE_INTERFACE_UNRESOLVED"], ["component:C-MISSING"]),
]


def pointer(doc, path):
    parts = [p.replace("~1", "/").replace("~0", "~") for p in path.split("/")[1:]]
    parent = doc
    for part in parts[:-1]:
        parent = parent[int(part)] if isinstance(parent, list) else parent[part]
    return parent, parts[-1]


def apply(doc, operations):
    doc = copy.deepcopy(doc)
    for op in operations:
        parent, key = pointer(doc, op["path"])
        if isinstance(parent, list):
            if op["op"] == "add" and key == "-":
                parent.append(copy.deepcopy(op["value"]))
                continue
            key = int(key)
            if op["op"] == "remove":
                parent.pop(key)
            elif op["op"] == "replace":
                if key >= len(parent):
                    raise KeyError(op["path"])
                parent[key] = copy.deepcopy(op["value"])
            else:
                parent.insert(key, copy.deepcopy(op["value"]))
        else:
            if op["op"] in ("remove", "replace") and key not in parent:
                raise KeyError(op["path"])
            if op["op"] == "remove":
                del parent[key]
            else:
                parent[key] = copy.deepcopy(op["value"])
    return doc


def oracle(requests):
    cases = {}
    for cfg, description in CONFIGS:
        cases[cfg["id"]] = case_entry(cfg, description, requests[cfg["id"]])
    base = json.loads(requests["J3-1"])
    refusals = {}
    for rid, text, operations, codes, refs in REFUSALS:
        apply(base, operations)  # every path exists on the base request
        refusals[rid] = {"base": "J3-1", "description": text, "operations": operations,
                         "expected": {"status_mechanics": "MODEL_INCOMPLETE", "results": "none",
                                      "blocking_codes_exactly": codes, "affected_refs_include": refs}}
    ai, a_s = section()
    return {
        "schema": "openpipestress.t4_u5a_reference/1",
        "unit": "T4-U5a (untied joint pressure thrust)",
        "status": "frozen independent reference; not product output, not a qualification or acceptance claim",
        "basis": "exact arithmetic over Q(pi) (rational functions of pi); decimals and binary64 values are "
                 "rounded from the exact forms; mill tolerance = corrosion allowance = 0 and every non-axial "
                 "response is zero, so the values are section-basis and shear-model neutral",
        "independence": "generators/u5a_reference.py imports no product code and reads only its own fixture",
        "physics_source": "JR CONTRACT.md sections 2, 4, 6 and its independent refutation sections 2-3 "
                          "(archived at 11718e61a2^); re-derived here from the axial free bodies",
        "fixture": {
            "E_pa": num(E), "nu": num(NU), "alpha_per_degC": num(ALPHA), "outside_diameter_m": num(OD),
            "wall_thickness_m": num(WALL), "inner_radius_m": num(R_IN), "Ai_m2": num(ai), "As_m2": num(a_s),
            "EA_n": num(E * a_s), "stub_axial_stiffness_n_per_m": num(E * a_s / STUB), "p_pa": num(P_PA),
            "Ae_m2": num(AE), "k_n_per_m": num(K_AXIAL), "stub_length_m": num(STUB), "joint_length_m": num(JOINT),
            "connector_other_diagonal": [num(v) for v in K_OTHER],
            "rotation_J3_1R": [[num(c) for c in row] for row in ROTATION],
        },
        "criterion": {
            "rule": "an entry passes when |obs - exp| <= 1e-9 * max(|exp|, F), F the floor of its family; "
                    "both solver modes",
            "relative": 1e-9,
            "floors": {"force_n": 1.0, "moment_nm": 1.0, "length_m": 1e-5, "rotation_rad": 1e-5},
            "exact_families": "blocking-code sets are compared exactly",
        },
        "sign_conventions": {
            "axial": "scalars along the chain axis n (axis_global) from A towards D; q = (u_j - u_i) . Q_x",
            "Tb": "joint elastic tensile force g_x = k (q - q_ref)",
            "Pe": "joint pressure generalized force p Ae, conjugate to qt_x (dead load, reference geometry)",
            "S_joint": "effective axial force Tb + Tt - Pe with Tt = 0 (untied)",
            "Nw": "pipe wall tension EA(eps - eps_T) + 2 nu Pi (pressurized members)",
            "S_pipe": "Nw - Pi on a pressurized member",
            "reactions": "support on model, R = K u - f",
            "pressure_rhs": "the pressure-only RHS (caps, Poisson pairs, joint terms); thermal and q_ref loads "
                            "are not pressure terms",
        },
        "cases": cases,
        "j4_and_ledger_mutants": mutants(),
        "lateral_offset_demonstration": offset_demonstration(),
        "refusals": refusals,
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    requests = {cfg["id"]: dumps(request(cfg)).encode() for cfg, _ in CONFIGS}
    oracle_bytes = dumps(oracle(requests)).encode()
    if args.write:
        (ROOT / "requests").mkdir(exist_ok=True)
        for cid, data in requests.items():
            (ROOT / "requests" / f"{cid}.request.json").write_bytes(data)
        (ROOT / "oracle.json").write_bytes(oracle_bytes)
        print(f"wrote {len(requests)} requests and oracle.json")
        return 0
    failures = []
    for cid, data in requests.items():
        path = ROOT / "requests" / f"{cid}.request.json"
        if not path.exists() or path.read_bytes() != data:
            failures.append(str(path.relative_to(ROOT)))
    if (ROOT / "oracle.json").read_bytes() != oracle_bytes:
        failures.append("oracle.json")
    if failures:
        print("MISMATCH: " + ", ".join(failures))
        return 1
    print(f"verified {len(requests)} requests and oracle.json byte for byte")
    return 0


if __name__ == "__main__":
    sys.exit(main())
