#!/usr/bin/env python3
"""RT-A3 increment 3 candidate references: ideal constant-effort supports
admitted on the exact pressure route (3.0.0/exact_pressure_v3).

Frozen analytical candidate, based on governing source
575fcc776 (main). Product agreement and independent scrutiny are separate
checks; no acceptance or release is asserted.

Independence. Standard library only (fractions, decimal, json, math.isqrt).
It imports, reads and runs no product code or product output. Governing text
was read only to fix the declared model:
  - R-4 (DEL-04-07) S2:
    CE-1, CE-2; caps, Poisson pair, wall force; the constant force
    cancels in O-S when both cases carry it (held/locked H-3 excluded here);
    D-2 (as decided by Agent 0: the constant force is a device action of
    the active support in every state of a load-state document);
  - DEL-04-07 "Constant-effort combinations: the unit factor-sum
    rule" (exact dyadic sum of binary64 factors, |sum f - 1| <= 2^-46, else
    CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE) and "Tees and pressure
    junctions" (D-1 caps -p*A_i*s at transfers_to_wall terminals);
  - validation/hand_calcs/mechanics/constant_effort_support_applied_load.md
    and the DEC-049 consumption conditions (exactly one declared translational
    restraint DOF and a positive constant_load; a constant nodal force of that
    magnitude along the POSITIVE global axis of that DOF in every solved load
    case; zero stiffness; no restraint row; a non-conforming record is not
    consumed: warning, no force, no restraint);
  - DEL-05-03 (D-5 bases;
    energy-matched annular shear factor kappa_E on the nominal section;
    Timoshenko coefficients with phi = 12EI/(G*A_s*L^2)).
No value is taken from any product output, frozen product reference,
standard, catalog or vendor source. All model data are invented.

Model: small-displacement, linear elastic, homogeneous isotropic straight
circular pipe members; nodal loads only (no self-weight, no span loads);
anchors restrain all six DOFs.

Two formulations, sharing only the input data, the section formulas and the
consumption rule that turns a constant-effort record into a nodal force:
  A. force method on the released tree (one anchor kept as root, every other
     anchor released into six redundants), section resultants by statics, and
     displacements by the unit-load (virtual-work) integrals
     int n N/EA + m.M/EI + t T/GJ (+ v.V/(kappa G A) for Timoshenko)
     + int n*eps0 over the members, integrated exactly (Simpson's rule is
     exact for these quadratics); reactions and end actions by statics.
  B. an independently assembled 3-D frame stiffness system: 12x12 member
     matrices (Euler-Bernoulli or Timoshenko) rotated by exact direction
     cosines, caps and the constant force as nodal loads, eigenstrains
     (Poisson pair, thermal) as held end actions, exact Gauss-Jordan
     elimination over Fractions per power of pi; reactions R = K u - f;
     end actions k d + held actions.
Every published value must agree exactly between A and B, and the separate
equilibrium, constitutive, superposition, cancellation, gate and closed-form
checks must all hold, or nothing is written and the exit status is 1.

Usage:  python3 -I -B rt_a3_constant_effort_reference.py [--check]  # default: check frozen bytes
        python3 -I -B rt_a3_constant_effort_reference.py --write    # rewrite references.json and SHA256SUMS
        python3 -I -B rt_a3_constant_effort_reference.py --summary  # derive and show the check count only
"""

from decimal import Decimal, getcontext
from fractions import Fraction as Fr
from math import isqrt
import argparse
import hashlib
import json
import re
import sys
from pathlib import Path

getcontext().prec = 120

SOURCE_REVISION = "575fcc776"

# ---------------------------------------------------------------------------
# Pi: exact Machin/alternating-series bounds; Decimal is display only


def _atan_inv_bounds(n):
    x = Fr(1, n)
    power = x
    total = Fr(0)
    k = 0
    while True:
        total += power / (2 * k + 1)
        next_power = -power * x * x
        next_term = next_power / (2 * k + 3)
        if abs(next_term) < Fr(1, 10 ** 115):
            return min(total, total + next_term), max(total, total + next_term)
        power = next_power
        k += 1


_a5, _a239 = _atan_inv_bounds(5), _atan_inv_bounds(239)
PI_LO = 16 * _a5[0] - 4 * _a239[1]
PI_HI = 16 * _a5[1] - 4 * _a239[0]
assert Fr(3) < PI_LO < PI_HI < Fr(22, 7)
assert PI_HI - PI_LO < Fr(1, 10 ** 110)


def _pi_pow_bounds(k):
    if k >= 0:
        return PI_LO ** k, PI_HI ** k
    return PI_HI ** k, PI_LO ** k


# ---------------------------------------------------------------------------
# Exact numbers: Laurent polynomials in pi with rational coefficients


class Q:
    """sum_k c_k * pi^k, c_k rational."""
    __slots__ = ("t",)

    def __init__(self, t=None):
        self.t = {k: Fr(v) for k, v in (t or {}).items() if v != 0}

    @staticmethod
    def of(x, k=0):
        return Q({k: Fr(x)})

    def __add__(self, o):
        o = _q(o)
        t = dict(self.t)
        for k, v in o.t.items():
            t[k] = t.get(k, Fr(0)) + v
        return Q(t)

    __radd__ = __add__

    def __neg__(self):
        return Q({k: -v for k, v in self.t.items()})

    def __sub__(self, o):
        return self + (-_q(o))

    def __rsub__(self, o):
        return _q(o) - self

    def __mul__(self, o):
        o = _q(o)
        t = {}
        for k1, v1 in self.t.items():
            for k2, v2 in o.t.items():
                t[k1 + k2] = t.get(k1 + k2, Fr(0)) + v1 * v2
        return Q(t)

    __rmul__ = __mul__

    def __truediv__(self, o):
        o = _q(o)
        if len(o.t) != 1:
            raise ValueError("division by a non-monomial")
        (k, v), = o.t.items()
        return Q({kk - k: vv / v for kk, vv in self.t.items()})

    def __eq__(self, o):
        return self.t == _q(o).t

    def __hash__(self):
        return hash(tuple(sorted(self.t.items())))

    def is_zero(self):
        return not self.t

    def shift(self, k):
        return Q({kk + k: v for kk, v in self.t.items()})

    def __repr__(self):
        return f"Q({self.t})"


def _q(x):
    return x if isinstance(x, Q) else Q.of(x)


ZERO = Q()

# An output expression: terms (coefficient, power of pi, is_sqrt): c*pi^k or sqrt(c)*pi^k.


def expr_of(q):
    return [(v, k, False) for k, v in sorted(q.t.items(), key=lambda kv: -kv[0])]


def _sqrt_bounds(c):
    n, d = c.numerator, c.denominator
    scale = 10 ** 120
    r = isqrt(n * d * scale * scale)
    return Fr(r, d * scale), Fr(r + 1, d * scale)


def expr_bounds(e):
    lo = hi = Fr(0)
    for c, k, root in e:
        plo, phi = _pi_pow_bounds(k)
        if root:
            slo, shi = _sqrt_bounds(c)
            lo += slo * plo
            hi += shi * phi
        else:
            a, b = c * plo, c * phi
            lo += min(a, b)
            hi += max(a, b)
    return lo, hi


def expr_text(e):
    if not e:
        return "0"
    parts = []
    for c, k, root in e:
        base = f"sqrt({c})" if root else f"{c}"
        if k == 0:
            parts.append(base)
        elif k == 1:
            parts.append(f"{base}*pi")
        elif k == -1:
            parts.append(f"{base}/pi")
        elif k > 1:
            parts.append(f"{base}*pi^{k}")
        else:
            parts.append(f"{base}/pi^{-k}")
    return " + ".join(parts)


def expr_sign(e):
    if not e:
        return 0
    lo, hi = expr_bounds(e)
    if lo > 0:
        return 1
    if hi < 0:
        return -1
    raise SystemExit(f"sign undecided for {expr_text(e)}")


def num(x, unit):
    """Published number: exact text, 40-digit decimal, nearest binary64, unit."""
    e = expr_of(x) if isinstance(x, Q) else (x if isinstance(x, list) else expr_of(_q(x)))
    if not e:
        return {"exact": "0", "decimal": "0", "value": 0.0, "unit": unit}
    lo, hi = expr_bounds(e)
    f_lo, f_hi = float(lo), float(hi)
    if f_lo != f_hi:
        raise SystemExit(f"binary64 rounding undecided for {expr_text(e)}")
    mid = (lo + hi) / 2
    dec = Decimal(mid.numerator) / Decimal(mid.denominator)
    return {"exact": expr_text(e), "decimal": f"{dec:.40g}", "value": f_lo, "unit": unit}


def q_abs(q):
    return q if expr_sign(expr_of(q)) >= 0 else -q


# 3-vectors of Q


def vadd(a, b):
    return [x + y for x, y in zip(a, b)]


def vsub(a, b):
    return [x - y for x, y in zip(a, b)]


def vscale(a, s):
    return [x * s for x in a]


def dot(a, b):
    return sum((x * y for x, y in zip(a, b)), ZERO)


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def qv(v):
    return [_q(x) for x in v]


# ---------------------------------------------------------------------------
# Material and D-5 section (exact rationals; areas etc. stored as coefficients of pi)

E = Fr(200_000_000_000)          # Pa
NU = Fr(3, 10)
G = E / (2 * (1 + NU))           # 10^12/13 Pa
ALPHA = Fr(3, 250_000)           # 1/K (12e-6 per K)


def section(D, t, cm, ca):
    D, t, cm, ca = Fr(D), Fr(t), Fr(cm), Fr(ca)
    ro = D / 2
    ri_n = ro - t
    t_r = (t - cm) - ca
    ri_r = ro - t_r
    s = {"D": D, "t": t, "cm": cm, "ca": ca, "ro": ro, "ri_n": ri_n, "t_r": t_r, "ri_r": ri_r}
    for b, ri in (("n", ri_n), ("r", ri_r)):
        s["A" + b] = ro * ro - ri * ri
        s["I" + b] = (ro ** 4 - ri ** 4) / 4
        s["J" + b] = 2 * s["I" + b]
        s["Z" + b] = s["I" + b] / ro
    s["Ai"] = ri_r * ri_r
    # kappa_E, DEL-05-03, evaluated exactly on the nominal section
    m = ri_n / ro
    m2 = m * m
    a, bb, n1 = 1 + m2, 1 - m2, 1 + NU
    c = (8 * NU + 14) * NU + 7
    Dk = c * (bb * bb) + 48 * (m2 * (n1 * n1))
    s["kappa"] = (6 * (n1 * n1)) * (a * a) / Dk
    s["As"] = s["kappa"] * s["An"]
    return s


# S1 header section (R-4 §3.1: r_o = 0.1 m, t = 0.01 m), with invented nonzero
# mill tolerance and corrosion allowance so that the two bases and the pressure
# bore all differ from the nominal annulus.
SEC = section(Fr(1, 5), Fr(1, 100), Fr(1, 2000), Fr(1, 1000))
P10 = Fr(10_000_000)  # Pa

# ---------------------------------------------------------------------------
# Models

DOFS = ("UX", "UY", "UZ", "RX", "RY", "RZ")
FORCE_KEYS = ("FX", "FY", "FZ", "MX", "MY", "MZ")
AXIS = {"UX": 0, "UY": 1, "UZ": 2}


def load(fx=0, fy=0, fz=0, mx=0, my=0, mz=0):
    return [Fr(fx), Fr(fy), Fr(fz), Fr(mx), Fr(my), Fr(mz)]


def ce_consumed(ce):
    trans = [d for d in ce["dofs"] if d in AXIS]
    return len(trans) == 1 and ce["constant_load"] > 0


def resolve(model, state):
    """Shared input resolution: nodal loads (mechanical, caps, constant force)
    as Q 6-vectors, and member eigenstrains."""
    nodes = model["nodes"]
    loads = {k: [ZERO] * 6 for k in nodes}
    for k, v in state.get("loads", {}).items():
        loads[k] = [a + b for a, b in zip(loads[k], v)]
    ce_rows = []
    for ce in model.get("ce", []):
        if not ce_consumed(ce):
            continue
        mult = state.get("ce_multiplier", Fr(1))
        if mult != 0:
            ax = AXIS[next(d for d in ce["dofs"] if d in AXIS)]
            loads[ce["node"]][ax] = loads[ce["node"]][ax] + Q.of(ce["constant_load"] * mult)
        ce_rows.append({"support": ce["id"], "node": ce["node"], "multiplier": mult})
    eps0 = {}
    p = state.get("p", Fr(0))
    dT = state.get("dT", Fr(0))
    for mbr in model["members"]:
        e0 = ALPHA * dT
        if p != 0 and mbr["id"] in state.get("pressurized", ()):
            e0 += -2 * NU * p * SEC["Ai"] / (E * SEC["An"])
            for end, other in ((mbr["from"], mbr["to"]), (mbr["to"], mbr["from"])):
                s = unit_vec(nodes[end], nodes[other])
                for i in range(3):
                    loads[end][i] = loads[end][i] + Q.of(-p * SEC["Ai"] * s[i], 1)
        eps0[mbr["id"]] = e0
    return loads, eps0, ce_rows


def unit_vec(a, b):
    d = [Fr(y) - Fr(x) for x, y in zip(a, b)]
    nz = [x for x in d if x != 0]
    if len(nz) != 1:
        raise SystemExit("members must be axis-aligned (exact direction cosines)")
    L = abs(nz[0])
    return [x / L for x in d]


def length(model, mbr):
    a, b = model["nodes"][mbr["from"]], model["nodes"][mbr["to"]]
    return sum(abs(Fr(y) - Fr(x)) for x, y in zip(a, b))


def rigidities():
    EA = Q.of(E * SEC["An"], 1)
    EI = Q.of(E * SEC["In"], 1)
    GJ = Q.of(G * SEC["Jn"], 1)
    GAs = Q.of(G * SEC["As"], 1)
    return EA, EI, GJ, GAs


# ---------------------------------------------------------------------------
# Formulation A: force method, statics and unit-load integrals on the released tree


def formulation_a(model, state):
    loads, eps0, _ = resolve(model, state)
    nodes = model["nodes"]
    chain = model["chain"]  # root anchor first
    timo = model["shear"] == "timoshenko"
    EA, EI, GJ, GAs = rigidities()
    seg = []  # (k, member, e from chain[k] to chain[k+1], L)
    for k in range(len(chain) - 1):
        a, b = chain[k], chain[k + 1]
        (mbr,) = [m for m in model["members"] if {m["from"], m["to"]} == {a, b}]
        seg.append((k, mbr, unit_vec(nodes[a], nodes[b]), length(model, mbr)))
    pos = {n: qv(nodes[n]) for n in nodes}

    def resultant(loadset, k, P):
        """Sum of loads on nodes chain[k+1:], reduced to point P."""
        F, M = [ZERO] * 3, [ZERO] * 3
        for n in chain[k + 1:]:
            if n not in loadset:
                continue
            f, m = loadset[n][:3], loadset[n][3:]
            F = vadd(F, f)
            M = vadd(M, vadd(m, cross(vsub(pos[n], P), f)))
        return F, M

    def internal(loadset, k, P, e):
        F, M = resultant(loadset, k, P)
        eq = qv(e)
        N = dot(F, eq)
        T = dot(M, eq)
        V = vsub(F, vscale(eq, N))
        Mb = vsub(M, vscale(eq, T))
        return N, V, T, Mb

    def vw(unit, real, with_eigen):
        tot = ZERO
        for k, mbr, e, L in seg:
            a = pos[chain[k]]
            pts = [a, vadd(a, vscale(qv(e), Fr(L, 2))), vadd(a, vscale(qv(e), L))]
            g = []
            for P in pts:
                n1, v1, t1, m1 = internal(unit, k, P, e)
                n2, v2, t2, m2 = internal(real, k, P, e)
                val = n1 * n2 / EA + t1 * t2 / GJ + dot(m1, m2) / EI
                if timo:
                    val = val + dot(v1, v2) / GAs
                if with_eigen:
                    val = val + n1 * eps0[mbr["id"]]
                g.append(val)
            tot = tot + (g[0] + 4 * g[1] + g[2]) * Fr(L, 6)
        return tot

    def unit_load(node, c):
        v = [ZERO] * 6
        v[c] = Q.of(1)
        return {node: v}

    released = [n for n in model["anchors"] if n != chain[0]]
    red = [(n, c) for n in released for c in range(6)]
    X = {}
    if red:
        nr = len(red)
        flex = [[vw(unit_load(*red[i]), unit_load(*red[j]), False) for j in range(nr)] for i in range(nr)]
        d0 = [vw(unit_load(*red[i]), loads, True) for i in range(nr)]
        # flex = pi^-1 * F'; F' X = -pi * d0, solved per power of pi
        Fp = [[Fr(0)] * nr for _ in range(nr)]
        for i in range(nr):
            for j in range(nr):
                q = flex[i][j].shift(1)
                if set(q.t) - {0}:
                    raise SystemExit("flexibility not a pure pi^-1 monomial")
                Fp[i][j] = q.t.get(0, Fr(0))
        rhs = [(-d0[i]).shift(1) for i in range(nr)]
        sol = solve_q(Fp, rhs)
        for (n, c), x in zip(red, sol):
            X.setdefault(n, [ZERO] * 6)
            X[n][c] = x
    real = {n: list(v) for n, v in loads.items()}
    for n, v in X.items():
        real[n] = [a + b for a, b in zip(real[n], v)]
    res = {"u": {}, "R": {}, "end": {}, "Nw": {}}
    for n in nodes:
        res["u"][n] = [ZERO] * 6 if n == chain[0] else [vw(unit_load(n, c), real, True) for c in range(6)]
    # root reaction by global statics about the root
    root = chain[0]
    Fsum, Msum = [ZERO] * 3, [ZERO] * 3
    for n in nodes:
        f, m = real[n][:3], real[n][3:]
        Fsum = vadd(Fsum, f)
        Msum = vadd(Msum, vadd(m, cross(vsub(pos[n], pos[root]), f)))
    res["R"][root] = [-x for x in Fsum] + [-x for x in Msum]
    for n in released:
        res["R"][n] = X[n]
    # end actions: force/moment the node exerts on the member, global axes
    for k, mbr, e, L in seg:
        near, far = chain[k], chain[k + 1]
        Ff, Mf = resultant(real, k, pos[far])
        Fn, Mn = resultant(real, k, pos[near])
        res["end"][mbr["id"]] = {near: [-x for x in Fn] + [-x for x in Mn], far: Ff + Mf}
        res["Nw"][mbr["id"]] = dot(Ff, qv(e))
    return res


# ---------------------------------------------------------------------------
# Formulation B: assembled 3-D frame stiffness


def solve_q(Kp, rhs):
    """Solve Kp x = rhs, Kp rational, rhs a list of Q: per power of pi."""
    n = len(rhs)
    powers = sorted({k for r in rhs for k in r.t})
    out = [ZERO] * n
    for k in powers:
        b = [r.t.get(k, Fr(0)) for r in rhs]
        x = gauss_jordan(Kp, b)
        out = [o + Q.of(v, k) for o, v in zip(out, x)]
    return out


def gauss_jordan(A, b):
    n = len(b)
    M = [list(A[i]) + [b[i]] for i in range(n)]
    for col in range(n):
        piv = next(r for r in range(col, n) if M[r][col] != 0)
        M[col], M[piv] = M[piv], M[col]
        pv = M[col][col]
        M[col] = [x / pv for x in M[col]]
        for r in range(n):
            if r != col and M[r][col] != 0:
                f = M[r][col]
                M[r] = [x - f * y for x, y in zip(M[r], M[col])]
    return [M[i][n] for i in range(n)]


def local_frame(e):
    ref = [Fr(0), Fr(0), Fr(1)] if e[2] == 0 else [Fr(1), Fr(0), Fr(0)]
    d = sum(r * x for r, x in zip(ref, e))
    y = [r - d * x for r, x in zip(ref, e)]
    z = [e[1] * y[2] - e[2] * y[1], e[2] * y[0] - e[0] * y[2], e[0] * y[1] - e[1] * y[0]]
    return [list(e), y, z]


def local_k(L, timo):
    """12x12 local stiffness, coefficients of pi; DOF order u v w rx ry rz (i), then (j)."""
    EA, EI, GJ = E * SEC["An"], E * SEC["In"], G * SEC["Jn"]
    phi = 12 * EI / (G * SEC["As"] * L * L) if timo else Fr(0)
    a = 12 * EI / (L ** 3 * (1 + phi))
    b = 6 * EI / (L ** 2 * (1 + phi))
    c = (4 + phi) * EI / (L * (1 + phi))
    d = (2 - phi) * EI / (L * (1 + phi))
    K = [[Fr(0)] * 12 for _ in range(12)]

    def put(idx, mat):
        for r, ir in enumerate(idx):
            for s, js in enumerate(idx):
                K[ir][js] += mat[r][s]
    put([0, 6], [[EA / L, -EA / L], [-EA / L, EA / L]])
    put([3, 9], [[GJ / L, -GJ / L], [-GJ / L, GJ / L]])
    put([1, 5, 7, 11], [[a, b, -a, b], [b, c, -b, d], [-a, -b, a, -b], [b, d, -b, c]])
    put([2, 4, 8, 10], [[a, -b, -a, -b], [-b, c, b, d], [-a, b, a, b], [-b, d, b, c]])
    return K, phi


def formulation_b(model, state):
    loads, eps0, _ = resolve(model, state)
    nodes = model["nodes"]
    names = list(nodes)
    idx = {n: i for i, n in enumerate(names)}
    ndof = 6 * len(names)
    timo = model["shear"] == "timoshenko"
    Kp = [[Fr(0)] * ndof for _ in range(ndof)]
    f = [ZERO] * ndof
    for n in names:
        for c in range(6):
            f[6 * idx[n] + c] = loads[n][c]
    elems = []
    for mbr in model["members"]:
        L = length(model, mbr)
        e = unit_vec(nodes[mbr["from"]], nodes[mbr["to"]])
        lam = local_frame(e)
        T = [[Fr(0)] * 12 for _ in range(12)]
        for blk in range(4):
            for r in range(3):
                for s in range(3):
                    T[3 * blk + r][3 * blk + s] = lam[r][s]
        kl, _ = local_k(L, timo)
        kg = [[sum(T[a][r] * kl[a][b] * T[b][s] for a in range(12) for b in range(12) if T[a][r] != 0 and T[b][s] != 0)
               for s in range(12)] for r in range(12)]
        gi = [6 * idx[mbr["from"]] + c for c in range(6)] + [6 * idx[mbr["to"]] + c for c in range(6)]
        for r in range(12):
            for s in range(12):
                Kp[gi[r]][gi[s]] += kg[r][s]
        # held end actions of the eigenstrain (tension N0 = -E A eps0): -N0 e at i, +N0 e at j;
        # equivalent nodal loads are their negatives
        N0 = Q.of(-E * SEC["An"] * eps0[mbr["id"]], 1)
        held = [ZERO] * 12
        for c in range(3):
            held[c] = -N0 * e[c]
            held[6 + c] = N0 * e[c]
        for r in range(12):
            f[gi[r]] = f[gi[r]] - held[r]
        elems.append((mbr, T, kl, gi, held))
    fixed = {6 * idx[a] + c for a in model["anchors"] for c in range(6)}
    free = [i for i in range(ndof) if i not in fixed]
    U = [ZERO] * ndof
    sol = solve_q([[Kp[r][c] for c in free] for r in free], [f[r].shift(-1) for r in free])  # K = pi*K', u = K'^-1 f / pi
    for i, v in zip(free, sol):
        U[i] = v
    Ku = [sum((U[c] * Kp[r][c] for c in range(ndof) if Kp[r][c] != 0), ZERO).shift(1) for r in range(ndof)]
    resid = [Ku[r] - f[r] for r in range(ndof)]
    res = {"u": {n: U[6 * idx[n]:6 * idx[n] + 6] for n in names},
           "R": {a: resid[6 * idx[a]:6 * idx[a] + 6] for a in model["anchors"]},
           "free_residual": [resid[i] for i in free], "end": {}, "Nw": {}, "elong": {}}
    for mbr, T, kl, gi, held in elems:
        ug = [U[g] for g in gi]
        ul = [sum((ug[s] * T[r][s] for s in range(12) if T[r][s] != 0), ZERO) for r in range(12)]
        fl = [sum((ul[s] * kl[r][s] for s in range(12) if kl[r][s] != 0), ZERO).shift(1) for r in range(12)]
        fg = [sum((fl[r] * T[r][s] for r in range(12) if T[r][s] != 0), ZERO) for s in range(12)]
        fg = [x + h for x, h in zip(fg, held)]
        res["end"][mbr["id"]] = {mbr["from"]: fg[:6], mbr["to"]: fg[6:]}
        res["Nw"][mbr["id"]] = fl[6] + Q.of(-E * SEC["An"] * eps0[mbr["id"]], 1)
        res["elong"][mbr["id"]] = ul[6] - ul[0]
    return res


# ---------------------------------------------------------------------------
# Checks


class Checks:
    def __init__(self):
        self.items = []

    def eq(self, name, a, b):
        ok = _q(a) == _q(b)
        self.items.append({"check": name, "pass": ok})
        if not ok:
            self.items[-1]["detail"] = f"{a} != {b}"
        return ok

    def true(self, name, cond, detail=""):
        self.items.append({"check": name, "pass": bool(cond)})
        if not cond:
            self.items[-1]["detail"] = detail
        return bool(cond)


def compare_ab(chk, lab, a, b):
    for n in a["u"]:
        for c in range(6):
            chk.eq(f"{lab}: u[{n}].{DOFS[c]} A==B", a["u"][n][c], b["u"][n][c])
    for n in b["R"]:
        for c in range(6):
            chk.eq(f"{lab}: R[{n}].{FORCE_KEYS[c]} A==B", a["R"][n][c], b["R"][n][c])
    for m in b["end"]:
        for n in b["end"][m]:
            for c in range(6):
                chk.eq(f"{lab}: end[{m}@{n}].{FORCE_KEYS[c]} A==B", a["end"][m][n][c], b["end"][m][n][c])
        chk.eq(f"{lab}: N_w[{m}] A==B", a["Nw"][m], b["Nw"][m])


def equilibrium_checks(chk, lab, model, state, b):
    loads, eps0, _ = resolve(model, state)
    nodes = model["nodes"]
    pos = {n: qv(nodes[n]) for n in nodes}
    chk.true(f"{lab}: B free-DOF residual zero", all(r.is_zero() for r in b["free_residual"]))
    F, M = [ZERO] * 3, [ZERO] * 3
    for n in nodes:
        tot = list(loads[n])
        if n in b["R"]:
            tot = [x + y for x, y in zip(tot, b["R"][n])]
        F = vadd(F, tot[:3])
        M = vadd(M, vadd(tot[3:], cross(pos[n], tot[:3])))
        # node equilibrium: loads + reaction = sum of end actions the node exerts on its members
        ends = [ZERO] * 6
        for m in b["end"]:
            if n in b["end"][m]:
                ends = [x + y for x, y in zip(ends, b["end"][m][n])]
        chk.true(f"{lab}: node {n} equilibrium", all((x - y).is_zero() for x, y in zip(tot, ends)))
    chk.true(f"{lab}: global force and moment balance", all(x.is_zero() for x in F + M))
    EA = Q.of(E * SEC["An"], 1)
    for mbr in model["members"]:
        ea = b["end"][mbr["id"]]
        fi, fj = ea[mbr["from"]], ea[mbr["to"]]
        rij = vsub(pos[mbr["to"]], pos[mbr["from"]])
        chk.true(f"{lab}: {mbr['id']} member force balance", all((x + y).is_zero() for x, y in zip(fi[:3], fj[:3])))
        mom = vadd(vadd(fi[3:], fj[3:]), cross(rij, fj[:3]))
        chk.true(f"{lab}: {mbr['id']} member moment balance", all(x.is_zero() for x in mom))
        L = length(model, mbr)
        chk.eq(f"{lab}: {mbr['id']} constitutive E A_n (elong/L - eps0) = N_w",
               EA * (b["elong"][mbr["id"]] / L - eps0[mbr["id"]]), b["Nw"][mbr["id"]])
    for a in model["anchors"]:
        chk.true(f"{lab}: anchor {a} motion zero", all(x.is_zero() for x in b["u"][a]))


def lin(*pairs):
    """Linear combination of solved results: pairs (factor, result)."""
    out = {"u": {}, "R": {}, "end": {}, "Nw": {}}
    f0, r0 = pairs[0]
    for n in r0["u"]:
        out["u"][n] = [sum((Fr(f) * r["u"][n][c] for f, r in pairs), ZERO) for c in range(6)]
    for n in r0["R"]:
        out["R"][n] = [sum((Fr(f) * r["R"][n][c] for f, r in pairs), ZERO) for c in range(6)]
    for m in r0["end"]:
        out["end"][m] = {n: [sum((Fr(f) * r["end"][m][n][c] for f, r in pairs), ZERO) for c in range(6)]
                         for n in r0["end"][m]}
        out["Nw"][m] = sum((Fr(f) * r["Nw"][m] for f, r in pairs), ZERO)
    return out


def same(chk, name, x, y):
    ok = True
    for key in ("u", "R"):
        for n in x[key]:
            ok &= all((p - q).is_zero() for p, q in zip(x[key][n], y[key][n]))
    for m in x["end"]:
        for n in x["end"][m]:
            ok &= all((p - q).is_zero() for p, q in zip(x["end"][m][n], y["end"][m][n]))
        ok &= (x["Nw"][m] - y["Nw"][m]).is_zero()
    return chk.true(name, ok)


# ---------------------------------------------------------------------------
# Publication


def stress_rows(model, state, r, with_magnitude=True):
    """Per member end and basis: axial normal/membrane N_w/A_b, bending |M_perp|/Z_b,
    torsion |T| r_o/J_b; member elastic maximum per basis."""
    nodes = model["nodes"]
    out = {}
    for mbr in model["members"]:
        e = unit_vec(nodes[mbr["from"]], nodes[mbr["to"]])
        nw = r["Nw"][mbr["id"]]
        rows = {}
        for basis, b in (("nominal_section", "n"), ("reduced_section", "r")):
            A, Z, J = Q.of(SEC["A" + b], 1), Q.of(SEC["Z" + b], 1), Q.of(SEC["J" + b], 1)
            sig = nw / A
            ends = {}
            cands = []
            for n in (mbr["from"], mbr["to"]):
                fv = r["end"][mbr["id"]][n]
                mv = fv[3:]
                tq = dot(mv, qv(e))
                mperp = vsub(mv, vscale(qv(e), tq))
                row = {"axial_normal_stress": num(sig, "Pa")}
                if with_magnitude:
                    mag = magnitude(mperp)
                    bend = [(c / (SEC["Z" + b] ** 2) if root else c / SEC["Z" + b], k - 1, root) for c, k, root in mag]
                    row["bending_normal_stress_resultant"] = num(bend, "Pa")
                    row["torsional_shear_stress"] = num(q_abs(tq * SEC["ro"] / J), "Pa")
                    if any(root for _, _, root in bend):
                        tot = expr_of(q_abs(sig)) + bend
                    else:
                        tot = expr_of(q_abs(sig) + Q({k: c for c, k, _ in bend}))
                    lo, hi = expr_bounds(tot)
                    cands.append((n, tot, lo, hi))
                ends[n] = row
            rows[basis] = {"ends": ends}
            if with_magnitude:
                (n0, t0, l0, h0), (n1, t1, l1, h1) = cands
                if l0 > h1 or t0 == t1:
                    best = cands[0]
                elif l1 > h0:
                    best = cands[1]
                else:
                    raise SystemExit("maximum station undecided")
                rows[basis]["elastic_normal_maximum"] = {"at_node": best[0], **num(best[1], "Pa")}
        out[mbr["id"]] = rows
    return out


def magnitude(v):
    nz = [x for x in v if not x.is_zero()]
    if not nz:
        return []
    if len(nz) == 1:
        return expr_of(q_abs(nz[0]))
    ks = {k for x in nz for k in x.t}
    if len(ks) != 1 or any(len(x.t) != 1 for x in nz):
        raise SystemExit("magnitude of a mixed-power vector is not published")
    (k,) = ks
    s = sum(x.t[k] ** 2 for x in nz)
    rn, rd = isqrt(s.numerator), isqrt(s.denominator)
    if rn * rn == s.numerator and rd * rd == s.denominator:
        return [(Fr(rn, rd), k, False)]
    return [(s, k, True)]


def lame_rows():
    out = {}
    for basis, ri in (("nominal_section", SEC["ri_n"]), ("reduced_section", SEC["ri_r"])):
        area = SEC["ro"] ** 2 - ri ** 2
        hoop_outer = 2 * P10 * ri * ri / area
        out[basis] = {"hoop_outer": num(hoop_outer, "Pa"), "hoop_inner": num(hoop_outer + P10, "Pa"),
                      "radial_inner": num(-P10, "Pa"), "radial_outer": num(0, "Pa")}
    return out


def pub_state(model, state, r, stresses=True, magnitude_rows=True):
    nodes = model["nodes"]
    pressurized = state.get("p", 0) != 0
    _, _, ce_rows = resolve(model, state)
    out = {
        "displacements": {n: {DOFS[c]: num(r["u"][n][c], "m" if c < 3 else "rad") for c in range(6)} for n in nodes},
        "reactions": {n: {FORCE_KEYS[c]: num(r["R"][n][c], "N" if c < 3 else "N*m") for c in range(6)} for n in r["R"]},
        "member_end_actions_global": {m: {n: {FORCE_KEYS[c]: num(v[c], "N" if c < 3 else "N*m") for c in range(6)}
                                          for n, v in ends.items()} for m, ends in r["end"].items()},
        "axial_wall_force_N_w": {m: num(v, "N") for m, v in r["Nw"].items()},
    }
    if pressurized:
        out["effective_axial_force_S"] = {
            m: num(r["Nw"][m] - Q.of(state["p"] * SEC["Ai"], 1) if m in state["pressurized"] else r["Nw"][m], "N")
            for m in r["Nw"]}
    cer = []
    for row in ce_rows:
        ce = next(c for c in model["ce"] if c["id"] == row["support"])
        dof = next(d for d in ce["dofs"] if d in AXIS)
        cer.append({"support": ce["id"], "node": ce["node"], "acting_dof": dof,
                    "direction": "+" + dof[1], "applied_load": num(ce["constant_load"] * row["multiplier"], "N")})
    out["constant_effort_applied_load"] = cer
    if stresses:
        out["section_stress_rows"] = stress_rows(model, state, r, magnitude_rows)
    return out


# ---------------------------------------------------------------------------
# Cases

L1 = Fr(5)                 # CE-1 cantilever length, m
W1, F1 = Fr(3000), Fr(8000)  # tip weight (-Z) and constant load (+Z), N
QY = Fr(2000)              # CE-2 case B lateral tip load (+Y), N
A3, B3 = Fr(4), Fr(3)      # CE-3 L-line legs, m
W3, F3 = Fr(6000), Fr(4500)
DT_O = Fr(100)             # K above the reference (installation) temperature


def cantilever(shear, ce_dofs=("UZ",), with_ce=True, load_value=F1):
    m = {"nodes": {"A": (0, 0, 0), "B": (L1, 0, 0)},
         "members": [{"id": "pipe:1", "from": "A", "to": "B"}],
         "anchors": ["A"], "chain": ["A", "B"], "shear": shear, "ce": []}
    if with_ce:
        m["ce"] = [{"id": "support:CE1", "node": "B", "dofs": list(ce_dofs), "constant_load": load_value}]
    return m


def lline(shear):
    """A=(0,0,0) anchor; C=(4,0,0) corner with the hanger; B=(4,0,-3) anchor
    (a riser down to the floor). pipe:2 runs B->C (orientation reversed against
    the chain)."""
    return {"nodes": {"A": (0, 0, 0), "C": (A3, 0, 0), "B": (A3, 0, -B3)},
            "members": [{"id": "pipe:1", "from": "A", "to": "C"}, {"id": "pipe:2", "from": "B", "to": "C"}],
            "anchors": ["A", "B"], "chain": ["A", "C", "B"], "shear": shear,
            "ce": [{"id": "support:CE3", "node": "C", "dofs": ["UZ"], "constant_load": F3}]}


def tip(w=W1, qy=0):
    return {"B": qv(load(fy=qy, fz=-w))}


def state(loads, p=0, members=(), dT=0, mult=Fr(1)):
    return {"loads": loads, "p": Fr(p), "pressurized": tuple(members), "dT": Fr(dT), "ce_multiplier": Fr(mult)}


def solve_both(chk, lab, model, st, compare=True):
    b = formulation_b(model, st)
    a = formulation_a(model, st)
    if compare:
        compare_ab(chk, lab, a, b)
    equilibrium_checks(chk, lab, model, st, b)
    return b


def model_inputs(model):
    return {
        "nodes_m": {n: [str(Fr(x)) for x in v] for n, v in model["nodes"].items()},
        "members": [{"id": m["id"], "from": m["from"], "to": m["to"], "length_m": str(length(model, m))}
                    for m in model["members"]],
        "anchors_all_six_dof": model["anchors"],
        "shear_model": "timoshenko_annulus_energy_v1" if model["shear"] == "timoshenko" else "euler_bernoulli",
        "constant_effort_records": [{"id": c["id"], "node": c["node"], "declared_restraint_dofs": c["dofs"],
                                     "hanger.constant_load_N": str(c["constant_load"]),
                                     "consumed": ce_consumed(c)} for c in model.get("ce", [])],
    }


def lod_text(st):
    out = {}
    for n, v in st["loads"].items():
        out[n] = {FORCE_KEYS[c]: str(v[c].t.get(0, Fr(0))) for c in range(6) if not v[c].is_zero()}
    return out


def state_inputs(st):
    d = {"nodal_loads_N": lod_text(st)}
    if st["p"] != 0:
        d["pressure_regions"] = [{"id": f"region:{m.split(':')[1]}", "members": [m], "pressure_Pa": str(st["p"]),
                                  "terminals": "both transfers_to_wall"} for m in st["pressurized"]]
    if st["dT"] != 0:
        d["temperature_above_reference_K"] = str(st["dT"])
    if st["ce_multiplier"] != 1:
        d["MUTATION_constant_force_multiplier"] = str(st["ce_multiplier"])
    return d


def build_reference():
    chk = Checks()
    cases = []
    EA, EI, GJ, GAs = rigidities()

    def tb(P, L, timo):  # textbook cantilever tip deflection (Q)
        v = Q.of(P) * Fr(L) ** 3 / (3 * EI)
        return v + Q.of(P) * Fr(L) / GAs if timo else v

    # ---- CE-1 ---------------------------------------------------------------
    ce1 = {}
    for shear in ("euler_bernoulli", "timoshenko"):
        timo = shear == "timoshenko"
        tag = "T" if timo else "EB"
        for pz in (False, True):
            lab = f"CE1-{tag}" + ("-P" if pz else "")
            mod = cantilever(shear)
            p, mem = (P10, ("pipe:1",)) if pz else (0, ())
            with_ = solve_both(chk, lab, mod, state(tip(), p, mem))
            without = solve_both(chk, lab + " without record", cantilever(shear, with_ce=False), state(tip(), p, mem))
            fonly = solve_both(chk, lab + " F-only", mod, state({}, 0, ()))
            ce1[lab] = (with_, without, fonly)
            same(chk, f"{lab}: superposition with - without == F-only", lin((1, with_), (-1, without)), fonly)
            net = F1 - W1
            chk.eq(f"{lab}: tip UZ = (F-W) L^3/(3EI) [+ (F-W) L/(kGA)]", with_["u"]["B"][2], tb(net, L1, timo))
            chk.eq(f"{lab}: tip RY = -(F-W) L^2/(2EI)", with_["u"]["B"][4], Q.of(-net) * L1 ** 2 / (2 * EI))
            chk.eq(f"{lab}: without: tip UZ = -W L^3/(3EI) [+ shear]", without["u"]["B"][2], tb(-W1, L1, timo))
            chk.eq(f"{lab}: F-only tip UZ = F L^3/(3EI) [+ shear]", fonly["u"]["B"][2], tb(F1, L1, timo))
            chk.eq(f"{lab}: R_A.FZ = -(F-W)", with_["R"]["A"][2], -net)
            chk.eq(f"{lab}: R_A.MY = (F-W) L", with_["R"]["A"][4], net * L1)
            chk.eq(f"{lab}: R_A.FX = 0 (capped single region, free tip)", with_["R"]["A"][0], 0)
            if not timo:
                chk.eq(f"{lab}: R-4 S2 CE-1: (F_ce - W) L^3/(3EI)", with_["u"]["B"][2], Q.of(F1 - W1) * L1 ** 3 / (3 * EI))
            if pz:
                ai = Q.of(P10 * SEC["Ai"], 1)
                chk.eq(f"{lab}: N_w = p A_i (free capped leg)", with_["Nw"]["pipe:1"], ai)
                chk.eq(f"{lab}: tip UX = (1-2nu) p A_i L/(E A_n)", with_["u"]["B"][0], (1 - 2 * NU) * ai * L1 / EA)
                for q, i in (("UX", 0),):
                    chk.eq(f"{lab}: support leaves pressure {q} unchanged", with_["u"]["B"][i], without["u"]["B"][i])
                chk.eq(f"{lab}: support leaves N_w unchanged", with_["Nw"]["pipe:1"], without["Nw"]["pipe:1"])
                unp = ce1[f"CE1-{tag}"][0]
                for i in (1, 2, 3, 4, 5):
                    chk.eq(f"{lab}: pressure leaves tip {DOFS[i]} unchanged", with_["u"]["B"][i], unp["u"]["B"][i])
                for i in (1, 2, 3, 4, 5):
                    chk.eq(f"{lab}: pressure leaves R_A.{FORCE_KEYS[i]} unchanged", with_["R"]["A"][i], unp["R"]["A"][i])
            mod_pub = cantilever(shear)
            st = state(tip(), p, mem)
            cases.append({
                "label": lab, "family": "CE-1", "admitted_case": True,
                "purpose": ("horizontal cantilever along +X, anchored at A, constant-effort support on UZ at the free tip B "
                            "(+F along +Z), tip weight W along -Z" + ("; one capped pressurized region" if pz else "; unpressurized")
                            + f"; {shear}"),
                "expected_outcome": "constant-effort record consumed; one constant_effort_support_applied_load row per solved case",
                "inputs": {**model_inputs(mod_pub), **state_inputs(st)},
                "results": {
                    "with_support": {**pub_state(mod_pub, st, with_),
                                     **({"lame_rows": lame_rows()} if pz else {})},
                    "without_record": pub_state(cantilever(shear, with_ce=False), st, without),
                    "F_only": {"tip_displacement_B": {DOFS[c]: num(fonly["u"]["B"][c], "m" if c < 3 else "rad") for c in range(6)},
                               "reaction_A": {FORCE_KEYS[c]: num(fonly["R"]["A"][c], "N" if c < 3 else "N*m") for c in range(6)}},
                    "superposition_identity": "with_support - without_record == F_only for every displacement, reaction, end action and N_w (exact)",
                },
            })
    # mutation controls (wrong sign; counted twice)
    for lab, mult, rel in (("CE1-EB-MUT-SIGN", Fr(-1), Fr(-2)), ("CE1-EB-MUT-TWICE", Fr(2), Fr(1))):
        mod = cantilever("euler_bernoulli")
        mut = solve_both(chk, lab, mod, state(tip(), 0, (), mult=mult))
        good, _, fonly = ce1["CE1-EB"]
        same(chk, f"{lab}: mutant - admitted == {rel} x F-only", lin((1, mut), (-1, good)), lin((rel, fonly)))
        chk.true(f"{lab}: tip UZ differs from the admitted value", not (mut["u"]["B"][2] - good["u"]["B"][2]).is_zero())
        chk.true(f"{lab}: R_A.FZ differs from the admitted value", not (mut["R"]["A"][2] - good["R"]["A"][2]).is_zero())
        cases.append({
            "label": lab, "family": "CE-1 control", "admitted_case": False,
            "purpose": ("MUTATION CONTROL, not an admitted case: CE1-EB with the constant force "
                        + ("applied along -Z (wrong sign)" if mult < 0 else "counted twice")),
            "expected_outcome": "must differ from CE1-EB; the difference is "
                                + f"{rel} x the F-only solution",
            "inputs": {**model_inputs(mod), **state_inputs(state(tip(), 0, (), mult=mult))},
            "results": {"tip_displacement_B": {DOFS[c]: num(mut["u"]["B"][c], "m" if c < 3 else "rad") for c in range(6)},
                        "reaction_A": {FORCE_KEYS[c]: num(mut["R"]["A"][c], "N" if c < 3 else "N*m") for c in range(6)}},
        })

    # ---- CE-1n: non-consumption controls ------------------------------------
    for shear in ("euler_bernoulli", "timoshenko"):
        timo = shear == "timoshenko"
        tag = "T" if timo else "EB"
        for dofs, kind in ((("UY", "UZ"), "2T"), (("RX",), "ROT")):
            lab = f"CE1N-{kind}-{tag}"
            mod = cantilever(shear, ce_dofs=dofs)
            chk.true(f"{lab}: record not consumed", not ce_consumed(mod["ce"][0]))
            r = solve_both(chk, lab, mod, state(tip(), 0, ()))
            same(chk, f"{lab}: identical to the model without the record", r, ce1[f"CE1-{tag}"][1])
            chk.eq(f"{lab}: tip UZ = -W L^3/(3EI) [+ shear]", r["u"]["B"][2], tb(-W1, L1, timo))
            chk.true(f"{lab}: no applied-load row", resolve(mod, state(tip()))[2] == [])
            cases.append({
                "label": lab, "family": "CE-1n", "admitted_case": True,
                "purpose": (f"CE-1 cantilever ({shear}) with a constant-effort record declaring "
                            + ("two translational DOFs (UY, UZ)" if kind == "2T" else "a rotational DOF only (RX)")
                            + ": does not meet the consumption conditions"),
                "expected_outcome": ("not consumed: no constant force, no stiffness, no restraint row; no "
                                     "constant_effort_support_applied_load row; one non-blocking warning "
                                     "SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED naming the support (code name as recorded in "
                                     "docs/validation_manual/headless_runner_reproduction.md; not derived here)"),
                "inputs": {**model_inputs(mod), **state_inputs(state(tip()))},
                "results": pub_state(mod, state(tip()), r),
            })

    # ---- CE-2: combinations ---------------------------------------------------
    mod = cantilever("euler_bernoulli")
    stA = state(tip())
    stB = state(tip(qy=QY), P10, ("pipe:1",))
    rA = solve_both(chk, "CE2 case A", mod, stA)
    rB = solve_both(chk, "CE2 case B", mod, stB)
    fonly = ce1["CE1-EB"][2]
    combos = [
        ("0.3*A + 0.7*B", (0.3, 0.7)),
        ("1.2*A - 0.2*B", (1.2, -0.2)),
        ("fl(1/3)*A + fl(2/3)*B", (1 / 3, 2 / 3)),
        ("(1 + 2^-46)*A (closed upper bound)", (1 + 2 ** -46, 0.0)),
        ("(1 - 2^-46)*A (closed lower bound)", (1 - 2 ** -46, 0.0)),
        ("2*A", (2.0, 0.0)),
        ("(1 + 2^-45)*A", (1 + 2 ** -45, 0.0)),
        ("2^52*A - 2^52*B", (2.0 ** 52, -(2.0 ** 52))),
    ]
    tol = Fr(1, 2 ** 46)
    comb_out = []
    for name, (fa, fb) in combos:
        fA, fB = Fr(fa), Fr(fb)
        s = fA + fB
        admitted = abs(s - 1) <= tol
        entry = {"combination": name, "kind": "mechanics",
                 "factors_binary64": {"A": {"decimal_input": repr(fa), "exact_dyadic": str(fA)},
                                      "B": {"decimal_input": repr(fb), "exact_dyadic": str(fB)}},
                 "exact_factor_sum": str(s), "sum_minus_one": str(s - 1),
                 "gate": "admitted" if admitted else "withheld: CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE"}
        if name.startswith(("0.3", "1.2", "fl(1/3)")):
            chk.eq(f"CE2 {name}: exact factor sum = 1 - 2^-54", s, 1 - Fr(1, 2 ** 54))
        chk.true(f"CE2 {name}: gate decision", admitted == (name.startswith(("0.3", "1.2", "fl(", "(1 + 2^-46)", "(1 - 2^-46)"))))
        if admitted:
            comb = lin((fA, rA), (fB, rB))
            # the true single-F system of the same loads: factored loads, pressure f_B*p, constant force once
            loads_true = {"B": qv(load(fy=fB * QY, fz=-(fA + fB) * W1))}
            st_true = state(loads_true, fB * P10, ("pipe:1",) if fB != 0 else ())
            r_true = solve_both(chk, f"CE2 {name} true single-F system", mod, st_true)
            same(chk, f"CE2 {name}: combined - true system == (sum f - 1) x F-only", lin((1, comb), (-1, r_true)), lin((s - 1, fonly)))
            # basis rows of a combination: only the linear axial normal stress per basis
            sig = {}
            for basis, b in (("nominal_section", "n"), ("reduced_section", "r")):
                sig[basis] = num(comb["Nw"]["pipe:1"] / Q.of(SEC["A" + b], 1), "Pa")
            entry["combined"] = {
                "constant_force_counted_times": str(s),
                "constant_force_in_combined_state": num(s * F1, "N"),
                "constant_force_excess_over_one_count": num((s - 1) * F1, "N"),
                "displacements": {n: {DOFS[c]: num(comb["u"][n][c], "m" if c < 3 else "rad") for c in range(6)} for n in comb["u"]},
                "reactions": {n: {FORCE_KEYS[c]: num(comb["R"][n][c], "N" if c < 3 else "N*m") for c in range(6)} for n in comb["R"]},
                "member_end_actions_global": {m: {n: {FORCE_KEYS[c]: num(v[c], "N" if c < 3 else "N*m") for c in range(6)}
                                                  for n, v in ends.items()} for m, ends in comb["end"].items()},
                "axial_wall_force_N_w": {m: num(v, "N") for m, v in comb["Nw"].items()},
                "axial_normal_stress_pipe:1": sig,
                "deviation_from_true_single_F_system": "(sum f - 1) x the F-only solution, exactly",
            }
        comb_out.append(entry)
    comb_out.append({"combination": "2^53*A - 2^53*B", "kind": "mechanics",
                     "gate": ("refused upstream by the value entry (CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT): factors of "
                              "magnitude 2^53 or more are integral; only a typed entry reaches the gate, which withholds "
                              "it (exact sum 0, |0 - 1| = 1 > 2^-46)"),
                     "exact_factor_sum": "0"})
    chk.true("CE2: 2^53 - 2^53 would be withheld by the gate", abs(Fr(2 ** 53) - Fr(2 ** 53) - 1) > tol)
    cases.append({
        "label": "CE2", "family": "CE-2", "admitted_case": True,
        "purpose": "CE-1 cantilever (euler_bernoulli) in two load cases, each carrying the support, and mechanics combinations",
        "inputs": {**model_inputs(mod), "load_cases": {"A": state_inputs(stA), "B": state_inputs(stB)},
                   "gate_rule": "exact dyadic sum of the binary64 factors; admitted iff |sum f - 1| <= 2^-46 (closed)"},
        "results": {"case_A": pub_state(mod, stA, rA), "case_B": {**pub_state(mod, stB, rB), "lame_rows": lame_rows()},
                    "combinations": comb_out},
    })

    # ---- CE-3: load-state document (D-2) ----------------------------------------
    for shear, pz in (("euler_bernoulli", False), ("euler_bernoulli", True), ("timoshenko", True)):
        tag = ("T" if shear == "timoshenko" else "EB") + ("-P" if pz else "")
        lab = f"CE3-{tag}"
        mod = lline(shear)
        p, mem = (P10, ("pipe:1", "pipe:2")) if pz else (0, ())
        wl = {"C": qv(load(fz=-W3))}
        stS = state(wl, p, mem, 0)
        stO = state(wl, p, mem, DT_O)
        rS = solve_both(chk, f"{lab} state S", mod, stS)
        rO = solve_both(chk, f"{lab} state O", mod, stO)
        rT = solve_both(chk, f"{lab} thermal only", lline(shear) | {"ce": []}, state({}, 0, (), DT_O))
        rF = solve_both(chk, f"{lab} F-only", mod, state({}, 0, ()))
        diff = lin((1, rO), (-1, rS))
        same(chk, f"{lab}: O - S == thermal-only solution (constant force cancels)", diff, rT)
        # O - S does not depend on F: the same with the record removed in both states
        mod0 = lline(shear) | {"ce": []}
        rS0 = solve_both(chk, f"{lab} state S without record", mod0, stS, compare=False)
        rO0 = solve_both(chk, f"{lab} state O without record", mod0, stO, compare=False)
        same(chk, f"{lab}: O - S independent of the constant force", lin((1, rO0), (-1, rS0)), diff)
        same(chk, f"{lab}: S - S(without record) == F-only", lin((1, rS), (-1, rS0)), rF)
        same(chk, f"{lab}: O - O(without record) == F-only", lin((1, rO), (-1, rO0)), rF)
        chk.true(f"{lab}: thermal moves the hanger node along its DOF (UZ at C nonzero)", not rT["u"]["C"][2].is_zero())
        chk.true(f"{lab}: constant force present in both states", len(resolve(mod, stS)[2]) == 1 and len(resolve(mod, stO)[2]) == 1)
        if pz:
            ai = Q.of(P10 * SEC["Ai"], 1)
            chk.eq(f"{lab}: junction cap at C = p A_i (+X +Z)", resolve(mod, stS)[0]["C"][0], ai)
            chk.eq(f"{lab}: junction cap at C, Z part", resolve(mod, stS)[0]["C"][2], ai - Q.of(W3) + Q.of(F3))
        res = {"state_S": pub_state(mod, stS, rS), "state_O": pub_state(mod, stO, rO),
               "O_minus_S": pub_state(mod0, state({}, 0, (), DT_O), diff, stresses=True, magnitude_rows=False),
               "thermal_only_check": "O_minus_S equals the thermal-only solution exactly (constant force, weight and pressure cancel)",
               "F_only": {"displacement_C": {DOFS[c]: num(rF["u"]["C"][c], "m" if c < 3 else "rad") for c in range(6)}}}
        del res["O_minus_S"]["constant_effort_applied_load"]
        if pz:
            res["lame_rows_both_states"] = lame_rows()
        cases.append({
            "label": lab, "family": "CE-3", "admitted_case": True,
            "purpose": ("load-state document: L-line A->C along +X, riser C->B down to an anchor; constant-effort support "
                        "on UZ at the corner C carrying a weight W; reference state at the installation temperature, "
                        "state S (sustained: W, F" + (", p" if pz else "") + ") and state O (operating: as S plus "
                        f"+{DT_O} K); {shear}"),
            "expected_outcome": "D-2: the constant force is a device action of the active support in every state; it cancels exactly in O - S",
            "inputs": {**model_inputs(mod), "material_alpha_per_K": str(ALPHA),
                       "states": {"S": state_inputs(stS), "O": state_inputs(stO)}},
            "results": res,
        })
        if lab == "CE3-EB":
            # mutation control: the force omitted from state O only
            stOm = state(wl, p, mem, DT_O, mult=0)
            rOm = solve_both(chk, f"{lab}-MUT-OMIT-O", mod, stOm)
            mdiff = lin((1, rOm), (-1, rS))
            same(chk, "CE3-EB-MUT-OMIT-O: (O without F) - S == thermal-only - F-only", mdiff, lin((1, rT), (-1, rF)))
            chk.true("CE3-EB-MUT-OMIT-O: O - S differs from the admitted thermal-only result",
                     not (mdiff["u"]["C"][2] - rT["u"]["C"][2]).is_zero())
            cases.append({
                "label": "CE3-EB-MUT-OMIT-O", "family": "CE-3 control", "admitted_case": False,
                "purpose": "MUTATION CONTROL, not an admitted case: CE3-EB with the constant force omitted from state O only",
                "expected_outcome": "O - S must differ from CE3-EB's O - S by minus the F-only solution",
                "inputs": {**model_inputs(mod), "states": {"S": state_inputs(stS), "O": state_inputs(stOm)}},
                "results": {"O_minus_S_displacement_C": {DOFS[c]: num(mdiff["u"]["C"][c], "m" if c < 3 else "rad") for c in range(6)}},
            })

    if not all(i["pass"] for i in chk.items):
        bad = [i for i in chk.items if not i["pass"]]
        sys.stderr.write(json.dumps(bad[:20], indent=1) + "\n")
        sys.exit(1)

    out = {
        "reference_id": "rt_a3_constant_effort_3",
        "status": "frozen analytical candidate; product agreement and independent scrutiny recorded separately",
        "source_revision_read": SOURCE_REVISION,
        "governing_sources": [
            "DEL-04-07 (R-4 S2: CE-1, CE-2; H-3 excluded; D-2)",
            "D-2 as decided by Agent 0 (device action of the active support in every state; held/locked state is the load-case lane's)",
            "DEL-04-07 'Constant-effort combinations: the unit factor-sum rule'; 'Tees and pressure junctions' (D-1)",
            "validation/hand_calcs/mechanics/constant_effort_support_applied_load.md (DEC-049 semantics and sign convention)",
            "DEL-05-03",
        ],
        "independence": "standard library only; no product code, product output, fixture or generated envelope is imported, read or run; all data invented",
        "model": {
            "theory": "small-displacement linear elastic straight circular pipe members; nodal loads only; anchors restrain all six DOFs",
            "constant_effort": ("consumed iff exactly one declared translational restraint DOF and constant_load > 0; then a constant nodal "
                                "force of constant_load along the POSITIVE global axis of that DOF in every solved case/state, zero "
                                "stiffness, no restraint row; otherwise not consumed (no force, no restraint)"),
            "pressure": ("caps -p*A_i*s at every transfers_to_wall terminal (s from the node into the member); Poisson pair as the axial "
                         "eigenstrain -2*nu*p*A_i/(E*A_n); N_w = statics wall force; S = N_w - p*A_i"),
            "thermal": "axial eigenstrain alpha*dT on the nominal section (E*A_n*alpha*dT held force)",
            "d5": ("nominal wall for A, I, J and the Timoshenko shear area; A_i = pi*r_i,r^2 for caps and the Poisson pair; per basis b: "
                   "sigma = N_w/A_b, bending |M_perp|/Z_b, torsion |T|*r_o/J_b; Lame of each basis's own annulus"),
            "timoshenko": "phi = 12*E*I_n/(G*kappa_E*A_n*L^2); kappa_E exact rational of DEL-05-03 on the nominal section; rotations are section rotations",
            "material": {"E_Pa": str(E), "nu": str(NU), "G_Pa": str(G), "alpha_per_K": str(ALPHA)},
            "section": {"D_m": str(SEC["D"]), "t_m": str(SEC["t"]), "c_m_m": str(SEC["cm"]), "c_a_m": str(SEC["ca"]),
                        "r_o_m": str(SEC["ro"]), "r_i_nominal_m": str(SEC["ri_n"]), "t_reduced_m": str(SEC["t_r"]),
                        "r_i_reduced_and_pressure_bore_m": str(SEC["ri_r"]),
                        "A_nominal_m2": num(Q.of(SEC["An"], 1), "m^2"), "A_reduced_m2": num(Q.of(SEC["Ar"], 1), "m^2"),
                        "I_nominal_m4": num(Q.of(SEC["In"], 1), "m^4"), "I_reduced_m4": num(Q.of(SEC["Ir"], 1), "m^4"),
                        "Z_nominal_m3": num(Q.of(SEC["Zn"], 1), "m^3"), "Z_reduced_m3": num(Q.of(SEC["Zr"], 1), "m^3"),
                        "J_nominal_m4": num(Q.of(SEC["Jn"], 1), "m^4"), "J_reduced_m4": num(Q.of(SEC["Jr"], 1), "m^4"),
                        "A_i_bore_m2": num(Q.of(SEC["Ai"], 1), "m^2"),
                        "kappa_E": num(SEC["kappa"], "1"), "A_shear_m2": num(Q.of(SEC["As"], 1), "m^2")},
            "phi_by_length": {f"L={L}": num(local_k(Fr(L), True)[1], "1") for L in (L1, A3, B3)},
            "formulations": {
                "A": ("force method on the released tree: root anchor kept, other anchors released into six redundants; statics "
                      "resultants; unit-load integrals int nN/EA + m.M/EI + tT/GJ (+ v.V/(kappa G A)) + n*eps0, exact Simpson"),
                "B": ("assembled 3-D frame stiffness (EB or Timoshenko 12x12, exact direction cosines), caps and constant force as "
                      "nodal loads, eigenstrains as held end actions, Gauss-Jordan over Fractions per power of pi, R = K u - f"),
            },
            "conventions": {
                "global_axes": "Z is vertical (up); a weight W acts along -Z",
                "reaction": "force/moment the anchor exerts on the pipe (R = K u - f), global axes",
                "member_end_action": "force/moment the node exerts on the member at that end, global axes; axial part at the far end is +N_w",
                "stress_stations": "member ends (moment is linear along every member: nodal loads only)",
                "combinations": "linear quantities only (motions, reactions, end actions, N_w, N_w/A_b); magnitudes and maxima of combinations are not published",
            },
            "exact_form": "sum of q*pi^k (q rational) or sqrt(q)*pi^k terms; 'value' is the nearest binary64, decided by rational pi bounds 1e-110 apart",
            "units": "m, rad, N, N*m, Pa",
        },
        "cases": cases,
        "checks": {"count": len(chk.items), "all_pass": True, "by_kind": _check_kinds(chk.items)},
    }
    return out


def _check_kinds(items):
    kinds = {}
    for i in items:
        name = i["check"].split(": ", 1)[-1]
        key = re.sub(r"\[[^\]]*\]|node [A-Z]\b|anchor [A-Z]\b|pipe:\d|\.(UX|UY|UZ|RX|RY|RZ|FX|FY|FZ|MX|MY|MZ)\b", "*", name)
        kinds[key] = kinds.get(key, 0) + 1
    return dict(sorted(kinds.items()))


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--write", action="store_true", help="rewrite frozen outputs intentionally")
    mode.add_argument("--check", action="store_true", help="check frozen bytes (the default)")
    mode.add_argument("--summary", action="store_true", help="derive and show the check count only")
    args = parser.parse_args()
    out = build_reference()
    if args.summary:
        print(f"{len(out['cases'])} cases, {out['checks']['count']} exact checks, all pass")
        return
    root = Path(__file__).resolve().parents[1]
    rendered = (json.dumps(out, indent=1, ensure_ascii=False) + "\n").encode("utf-8")
    files = {
        "README.md": (root / "README.md").read_bytes(),
        "generators/rt_a3_constant_effort_reference.py": Path(__file__).read_bytes(),
        "references.json": rendered,
    }
    sums = "".join(f"{hashlib.sha256(data).hexdigest()}  {name}\n" for name, data in files.items()).encode("ascii")
    outputs = {"references.json": rendered, "SHA256SUMS": sums}
    if args.write:
        for name, data in outputs.items():
            (root / name).write_bytes(data)
    else:
        different = [name for name, data in outputs.items() if not (root / name).is_file() or (root / name).read_bytes() != data]
        if different:
            raise SystemExit("Reference bytes differ: " + ", ".join(different))
    print(f"{len(out['cases'])} cases, {out['checks']['count']} exact checks, frozen bytes {'written' if args.write else 'match'}")


if __name__ == "__main__":
    main()
