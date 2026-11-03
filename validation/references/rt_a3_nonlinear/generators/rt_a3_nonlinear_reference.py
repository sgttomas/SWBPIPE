#!/usr/bin/env python3
"""RT-A3 increment 4 candidate references: nonlinear supports admitted on the
exact pressure route (3.0.0/exact_pressure_v3), R-4 stage S5.

Frozen analytical candidate, based on governing source 6757b2341 (main).
Product agreement and independent scrutiny are separate checks; no acceptance,
qualification or release is asserted.

Independence. Standard library only (fractions, decimal, json, hashlib).
It imports, reads and runs no product code, product output, fixture, result
schema or generated envelope. Governing text was read only to fix the declared
model:
  - R-4 (execution/PKG-04/DEL-04-07/Work/route-unification-r4.md) §00, §0a,
    §2.1-§2.4, §3.5 (S5: G-1, G-2, G-3, F-1, O-1, K-1), §3.8, §6;
  - DEL-04-01 kd5-nonlinear-active-set.md §0 (DEC-067 support laws), §1.2-§1.4
    (states, k = 1 Stage A), §2.2 (strict pass conditions), §2.4 (fail closed);
  - DEL-04-07 pressure-contract.md "Accepted T4 direction" H-2 and "T4-U2 as
    implemented" (each realized arc owns K_b*u_free(eps_p) - c_b), D-1 caps;
  - DEL-05-03 section-bases-and-timoshenko.md §1, §3.1, §3.3, §3.6 (arcs stay
    Euler-Bernoulli);
  - the arc flexibility definition of validation/references/t4_i7 (unit-load
    integrand N n/EA + T t/GJ + k (M.m - T t)/EI; no shear on arcs);
  - DEL-05-02 exact-combinations.md §4, pf-nonlinear-operand-standing.md and
    absolute-sum-srss-combinations.md (NONLINEAR_COMBINATION_REQUIRES_SOLVE).
No value is taken from any product output. All model data are invented.

Two formulations. They share the input data, the section formulas, the DEC-067
state laws and strict pass conditions (the problem definition), the exact
number class (rational functions of pi) and its Gauss-Jordan helper.
  A. Force method on the cantilever rooted at the anchor, with PHYSICAL
     pressure loads: caps p*A_i on the wall at transfers_to_wall terminals, the
     fluid wall load p*A_i/R along the outward normal of each realized arc, and
     the Poisson eigenstrain -2*nu*p*A_i/(E*A) on every member. Section
     resultants by statics; displacements by unit-load integrals, integrated
     exactly (polynomials on straights, Wallis integrals of cos^a sin^b over a
     quarter circle on arcs). Linear restraints and nonlinear supports are
     redundants; the support state is found by hand-calculation trials in a
     fixed order and the first state satisfying every condition is taken.
  B. Direct stiffness: an independently assembled 3-D frame system. Straight
     12x12 Euler-Bernoulli or Timoshenko matrices; each realized arc's 12x12
     matrix is the inverse of its hand-derived closed-form quarter-circle
     cantilever flexibility, mapped by equilibrium. Pressure enters as the H-2
     LEDGER: Poisson pairs on straights, caps -p*A_i*s at transfers_to_wall
     terminals, and K_b*u_free(eps_p) - c_b on each arc (u_free the stress-free
     homothetic expansion). Every combination of support states is solved and
     tested; exactly one must satisfy every strict condition.
A and B must agree exactly on every published displacement, reaction, wall end
action and wall force of every case; closed-form, equilibrium, complementarity,
identity and mutation checks must all hold, or nothing is written.

Usage:  python3 -I -B rt_a3_nonlinear_reference.py [--check]  # default: check frozen bytes
        python3 -I -B rt_a3_nonlinear_reference.py --write    # rewrite references.json and SHA256SUMS
        python3 -I -B rt_a3_nonlinear_reference.py --summary  # derive and show the check count only
"""

from decimal import Decimal, getcontext
from fractions import Fraction as Fr
from itertools import product
from math import gcd
import argparse
import hashlib
import json
import re
from pathlib import Path

getcontext().prec = 120

SOURCE_REVISION = "6757b2341"

# ---------------------------------------------------------------------------
# Pi: Machin's identity with exact alternating-series bounds


def _atan_inv_bounds(n):
    x = Fr(1, n)
    power, total, k = x, Fr(0), 0
    while True:
        total += power / (2 * k + 1)
        nxt = -power * x * x
        term = nxt / (2 * k + 3)
        if abs(term) < Fr(1, 10 ** 115):
            return min(total, total + term), max(total, total + term)
        power, k = nxt, k + 1


_a5, _a239 = _atan_inv_bounds(5), _atan_inv_bounds(239)
PI_LO = 16 * _a5[0] - 4 * _a239[1]
PI_HI = 16 * _a5[1] - 4 * _a239[0]
assert Fr(3) < PI_LO < PI_HI < Fr(22, 7) and PI_HI - PI_LO < Fr(1, 10 ** 110)

# ---------------------------------------------------------------------------
# Exact numbers: rational functions of pi with rational coefficients


def ptrim(p):
    p = list(p)
    while p and p[-1] == 0:
        p.pop()
    return tuple(p)


def padd(a, b):
    n = max(len(a), len(b))
    return ptrim([(a[i] if i < len(a) else 0) + (b[i] if i < len(b) else 0) for i in range(n)])


def pmul(a, b):
    if not a or not b:
        return ()
    out = [Fr(0)] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        if x:
            for j, y in enumerate(b):
                if y:
                    out[i + j] += x * y
    return ptrim(out)


def pdivmod(a, b):
    a = list(a)
    q = [Fr(0)] * max(len(a) - len(b) + 1, 1)
    while a and len(a) >= len(b):
        c, k = a[-1] / b[-1], len(a) - len(b)
        q[k] = c
        for i, y in enumerate(b):
            a[i + k] -= c * y
        a = list(ptrim(a))
    return ptrim(q), ptrim(a)


def pgcd(a, b):
    while b:
        a, b = b, pdivmod(a, b)[1]
    return tuple(x / a[-1] for x in a)


def _low(p):
    return next(i for i, c in enumerate(p) if c != 0)


def _norm(n, d):
    n, d = ptrim(n), ptrim(d)
    if not d:
        raise ZeroDivisionError("rational function with zero denominator")
    if not n:
        return (), (Fr(1),)
    m = min(_low(n), _low(d))
    if m:
        n, d = n[m:], d[m:]
    if sum(1 for c in d if c != 0) > 1 and len(n) > 1:
        g = pgcd(n, d)
        if len(g) > 1:
            n, d = pdivmod(n, g)[0], pdivmod(d, g)[0]
    lc = d[-1]
    if lc != 1:
        n, d = tuple(c / lc for c in n), tuple(c / lc for c in d)
    return n, d


class RF:
    """num(pi)/den(pi), coefficients rational, den monic and coprime to num."""
    __slots__ = ("n", "d")

    def __init__(self, n, d=(Fr(1),)):
        self.n, self.d = _norm(tuple(Fr(x) for x in n), tuple(Fr(x) for x in d))

    def is_zero(self):
        return not self.n

    def __add__(self, o):
        o = _rf(o)
        if o is NotImplemented:
            return o
        if self.d == o.d:
            return RF(padd(self.n, o.n), self.d)
        return RF(padd(pmul(self.n, o.d), pmul(o.n, self.d)), pmul(self.d, o.d))

    __radd__ = __add__

    def __neg__(self):
        r = RF.__new__(RF)
        r.n, r.d = tuple(-c for c in self.n), self.d
        return r

    def __sub__(self, o):
        o = _rf(o)
        if o is NotImplemented:
            return o
        return self + (-o)

    def __rsub__(self, o):
        return _rf(o) - self

    def __mul__(self, o):
        o = _rf(o)
        if o is NotImplemented:
            return o
        if self.is_zero() or o.is_zero():
            return ZERO
        return RF(pmul(self.n, o.n), pmul(self.d, o.d))

    __rmul__ = __mul__

    def __truediv__(self, o):
        o = _rf(o)
        if o.is_zero():
            raise ZeroDivisionError
        return RF(pmul(self.n, o.d), pmul(self.d, o.n))

    def __rtruediv__(self, o):
        return _rf(o) / self

    def __eq__(self, o):
        o = _rf(o)
        if o is NotImplemented:
            return False
        return self.n == o.n and self.d == o.d

    def __hash__(self):
        return hash((self.n, self.d))

    def __repr__(self):
        return f"RF({rf_text(self)})"


def _rf(x):
    if isinstance(x, RF):
        return x
    if isinstance(x, (int, Fr)):
        return RF((Fr(x),))
    return NotImplemented


def rf(x):
    return x if isinstance(x, RF) else RF((Fr(x),))


ZERO = RF(())
ONE = RF((1,))
PI = RF((0, 1))


def _poly_bounds(p):
    lo = hi = Fr(0)
    for k, c in enumerate(p):
        if c > 0:
            lo += c * PI_LO ** k
            hi += c * PI_HI ** k
        elif c < 0:
            lo += c * PI_HI ** k
            hi += c * PI_LO ** k
    return lo, hi


def bounds(x):
    x = rf(x)
    nl, nh = _poly_bounds(x.n)
    dl, dh = _poly_bounds(x.d)
    if not (dl > 0 or dh < 0):
        raise SystemExit(f"denominator sign undecided: {rf_text(x)}")
    qs = [a / b for a in (nl, nh) for b in (dl, dh)]
    return min(qs), max(qs)


def sign(x):
    x = rf(x)
    if x.is_zero():
        return 0
    lo, hi = bounds(x)
    if lo > 0:
        return 1
    if hi < 0:
        return -1
    raise SystemExit(f"sign undecided: {rf_text(x)}")


def rabs(x):
    return x if sign(x) >= 0 else -x


def _poly_text(p, shift=0):
    parts = []
    for k in range(len(p) - 1, -1, -1):
        c = p[k]
        if c == 0:
            continue
        e = k - shift
        a = abs(c)
        if e == 0:
            t = f"{a}"
        elif e == 1:
            t = f"{a}*pi"
        elif e == -1:
            t = f"{a}/pi"
        elif e > 1:
            t = f"{a}*pi^{e}"
        else:
            t = f"{a}/pi^{-e}"
        if not parts:
            parts.append(t if c > 0 else f"-{t}")
        else:
            parts.append(f"+ {t}" if c > 0 else f"- {t}")
    return " ".join(parts) if parts else "0"


def _lcm(a, b):
    return a * b // gcd(a, b)


def rf_text(x):
    x = rf(x)
    if x.is_zero():
        return "0"
    nz = [(k, c) for k, c in enumerate(x.d) if c != 0]
    if len(nz) == 1:  # den = pi^m (monic)
        return _poly_text(x.n, nz[0][0])
    L = 1
    for c in x.n + x.d:
        L = _lcm(L, c.denominator)
    n = [c * L for c in x.n]
    d = [c * L for c in x.d]
    g = 0
    for c in d:
        g = gcd(g, int(c))
    if d[-1] < 0:
        g = -g
    n = tuple(Fr(c, g) for c in n)
    d = tuple(Fr(c, g) for c in d)
    return f"({_poly_text(n)})/({_poly_text(d)})"


def num(x, unit):
    """Published number: exact text, 40-digit decimal, nearest binary64 (decided), unit."""
    x = rf(x)
    if x.is_zero():
        return {"exact": "0", "decimal": "0", "value": 0.0, "unit": unit}
    lo, hi = bounds(x)
    if float(lo) != float(hi):
        raise SystemExit(f"binary64 rounding undecided for {rf_text(x)}")
    mid = (lo + hi) / 2
    dec = Decimal(mid.numerator) / Decimal(mid.denominator)
    return {"exact": rf_text(x), "decimal": f"{dec:.40g}", "value": float(lo), "unit": unit}


def approx(x):
    lo, hi = bounds(x)
    return float((lo + hi) / 2)


def gauss_jordan(A, B):
    """Solve A X = B over RF; B is a list of right-hand-side rows (n x m)."""
    n = len(A)
    M = [list(A[i]) + list(B[i]) for i in range(n)]
    w = len(M[0])
    for col in range(n):
        piv = next((r for r in range(col, n) if not M[r][col].is_zero()), None)
        if piv is None:
            raise SystemExit("singular system")
        M[col], M[piv] = M[piv], M[col]
        inv = ONE / M[col][col]
        M[col] = [x * inv if not x.is_zero() else x for x in M[col]]
        for r in range(n):
            f = M[r][col]
            if r != col and not f.is_zero():
                M[r] = [x - f * y if not y.is_zero() else x for x, y in zip(M[r], M[col])]
    return [M[i][n:w] for i in range(n)]


# 3-vectors


def vadd(a, b):
    return [x + y for x, y in zip(a, b)]


def vsub(a, b):
    return [x - y for x, y in zip(a, b)]


def vsc(a, s):
    return [x * s for x in a]


def vdot(a, b):
    out = ZERO
    for x, y in zip(a, b):
        out = out + x * y
    return out


def vcross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def rv(v):
    return [rf(x) for x in v]


# ---------------------------------------------------------------------------
# Material and section (S1 header; c_m = c_a = 0, so nominal = reduced basis)

E = Fr(200_000_000_000)
NU = Fr(3, 10)
G = E / (2 * (1 + NU))
RO, TW = Fr(1, 10), Fr(1, 100)
RI = RO - TW
AN = RF((0, RO ** 2 - RI ** 2))                 # pi (ro^2 - ri^2)
IN = RF((0, (RO ** 4 - RI ** 4) / 4))
JN = 2 * IN
AI = RF((0, RI ** 2))                           # pressure bore area
ZN = IN / rf(RO)
_m = RI / RO
_m2 = _m * _m
_c = (8 * NU + 14) * NU + 7
KAPPA = (6 * (1 + NU) ** 2) * (1 + _m2) ** 2 / (_c * (1 - _m2) ** 2 + 48 * _m2 * (1 + NU) ** 2)
ASH = AN * rf(KAPPA)
EA, EI, GJ, GAS = AN * rf(E), IN * rf(E), JN * rf(G), ASH * rf(G)

DOFS = ("UX", "UY", "UZ", "RX", "RY", "RZ")
FKEYS = ("FX", "FY", "FZ", "MX", "MY", "MZ")


def eps_p(p):
    return rf((1 - 2 * NU) * p) * AI / EA


def e_poisson(p):
    return rf(-2 * NU * p) * AI / EA


# ---------------------------------------------------------------------------
# Models (shared input data)


def unit_axis(a, b):
    d = [Fr(y) - Fr(x) for x, y in zip(a, b)]
    nz = [x for x in d if x != 0]
    assert len(nz) == 1, "straight members are axis-aligned"
    L = abs(nz[0])
    return [x / L for x in d], L


def arc_frame(mdl, mbr):
    X = mdl["nodes"]
    C = mbr["center"]
    ri = [Fr(a) - Fr(c) for a, c in zip(X[mbr["i"]], C)]
    rj = [Fr(a) - Fr(c) for a, c in zip(X[mbr["j"]], C)]
    R2 = sum(x * x for x in ri)
    assert R2 == sum(x * x for x in rj) and sum(x * y for x, y in zip(ri, rj)) == 0, "quarter circle"
    R = mbr["R"]
    assert R * R == R2
    e1 = [x / R for x in ri]
    e2 = [x / R for x in rj]
    n = [e1[1] * e2[2] - e1[2] * e2[1], e1[2] * e2[0] - e1[0] * e2[2], e1[0] * e2[1] - e1[1] * e2[0]]
    return R, e1, e2, n


def tangents(mdl, mbr):
    """(start tangent, end tangent) along i->j, exact."""
    if mbr["kind"] == "straight":
        e, _ = unit_axis(mdl["nodes"][mbr["i"]], mdl["nodes"][mbr["j"]])
        return e, e
    R, e1, e2, n = arc_frame(mdl, mbr)
    return e2, [-x for x in e1]


def model_g(gap_sign=+1, clearance=Fr(1, 2000)):
    return {
        "name": "G", "shear": "euler_bernoulli",
        "nodes": {"N0": (0, 0, 0), "N1": (10, 0, 0)},
        "chain": ["N0", "N1"],
        "members": [{"id": "pipe:1", "kind": "straight", "i": "N0", "j": "N1"}],
        "anchor": {"id": "support:ANCHOR", "node": "N0"},
        "terminals": {"N0": "separately_supported_or_compensated", "N1": "transfers_to_wall"},
        "restraints": [],
        "nonlinear": [{"id": "support:GAP", "kind": "gap", "node": "N1", "dof": 0,
                       "closes": gap_sign, "clearance": Fr(clearance)}],
    }


def model_f(normal=("source", "N1", 0), shear="euler_bernoulli"):
    fr = {"id": "support:FRICTION", "kind": "friction", "node": "N1", "dof": 2, "mu": Fr(3, 10)}
    if normal[0] == "explicit":
        fr["explicit_normal"] = Fr(normal[1])
    else:
        fr["source"] = (normal[1], normal[2])
    return {
        "name": "F", "shear": shear,
        "nodes": {"N0": (0, 0, 0), "N1": (3, 0, 0)},
        "chain": ["N0", "N1"],
        "members": [{"id": "pipe:1", "kind": "straight", "i": "N0", "j": "N1"}],
        "anchor": {"id": "support:ANCHOR", "node": "N0"},
        "terminals": {"N0": "separately_supported_or_compensated", "N1": "transfers_to_wall"},
        "restraints": [{"id": "support:N1-UX", "node": "N1", "dof": 0}],
        "nonlinear": [fr],
    }


O_H, O_R, O_A, O_C, O_K = Fr(1), Fr(3, 10), Fr(1), Fr(1, 2), Fr(2)


def model_o():
    h, R, a, c = O_H, O_R, O_A, O_C
    return {
        "name": "O", "shear": "euler_bernoulli",
        "nodes": {"A": (0, 0, 0), "B1": (0, 0, h), "B2": (R, 0, h + R),
                  "S": (R + a, 0, h + R), "C": (R + a + c, 0, h + R)},
        "chain": ["A", "B1", "B2", "S", "C"],
        "members": [
            {"id": "pipe:riser", "kind": "straight", "i": "A", "j": "B1"},
            {"id": "bend:B", "kind": "arc", "i": "B1", "j": "B2", "center": (R, 0, h), "R": R, "k": O_K},
            {"id": "pipe:run1", "kind": "straight", "i": "B2", "j": "S"},
            {"id": "pipe:run2", "kind": "straight", "i": "S", "j": "C"},
        ],
        "anchor": {"id": "support:ANCHOR", "node": "A"},
        "terminals": {"A": "transfers_to_wall", "C": "transfers_to_wall"},
        "restraints": [],
        "nonlinear": [{"id": "support:REST", "kind": "one_way", "node": "S", "dof": 2, "sense": +1}],
    }


def load_case(p=0, loads=None, ledger_mutation=None):
    return {"p": Fr(p), "loads": {k: [Fr(x) for x in v] for k, v in (loads or {}).items()},
            "ledger_mutation": ledger_mutation}


def nodal(fx=0, fy=0, fz=0, mx=0, my=0, mz=0):
    return [fx, fy, fz, mx, my, mz]


# ---------------------------------------------------------------------------
# DEC-067 state laws and strict pass conditions (shared problem definition)


def state_options(sup):
    if sup["kind"] == "one_way":
        return ["inactive", "active"]
    if sup["kind"] == "gap":
        return ["open", "closed"]
    if "explicit_normal" in sup:
        return ["stick", "slide+", "slide-", "inactive"]
    return ["stick", "slide+/b+", "slide+/b-", "slide-/b+", "slide-/b-", "inactive"]


def slide_dir(st):
    return +1 if st.startswith("slide+") else -1


def slide_beta(st):
    return +1 if st.endswith("b+") else -1


def strict_valid(sup, st, u_d, r_d, normal):
    """normal: explicit N, or the signed source reaction R_n (derived)."""
    k = sup["kind"]
    if k == "one_way":
        s = sup["sense"]
        return sign(r_d) * s > 0 if st == "active" else sign(u_d) * s > 0
    if k == "gap":
        s, c = sup["closes"], sup["clearance"]
        if st == "closed":
            return sign(r_d) * s < 0
        return sign(rf(c) - u_d * rf(s)) > 0
    mu = rf(sup["mu"])
    if "explicit_normal" in sup:
        N = rf(sup["explicit_normal"])
        if st == "inactive":
            return sign(N) <= 0
        if sign(N) <= 0:
            return False
        if st == "stick":
            return sign(mu * N - rabs(r_d)) > 0
        return sign(u_d) * slide_dir(st) > 0
    if st == "inactive":
        return normal.is_zero()
    if normal.is_zero():
        return False
    if st == "stick":
        return sign(mu * rabs(normal) - rabs(r_d)) > 0
    return sign(normal) * slide_beta(st) > 0 and sign(u_d) * slide_dir(st) > 0


# ---------------------------------------------------------------------------
# Formulation A: force method with physical pressure loads


class Fn:
    """Sum of c * cos(psi)^a * sin(psi)^b * s^n over one member's parameter."""
    __slots__ = ("t",)

    def __init__(self, t=None):
        self.t = {}
        for key, v in (t or {}).items():
            v = rf(v)
            if not v.is_zero():
                self.t[key] = v

    def __add__(self, o):
        o = fnc(o)
        t = dict(self.t)
        for key, v in o.t.items():
            t[key] = t[key] + v if key in t else v
        return Fn(t)

    __radd__ = __add__

    def __neg__(self):
        return Fn({key: -v for key, v in self.t.items()})

    def __sub__(self, o):
        return self + (-fnc(o))

    def __rsub__(self, o):
        return fnc(o) - self

    def __mul__(self, o):
        if isinstance(o, Fn):
            t = {}
            for k1, v1 in self.t.items():
                for k2, v2 in o.t.items():
                    key = (k1[0] + k2[0], k1[1] + k2[1], k1[2] + k2[2])
                    t[key] = t[key] + v1 * v2 if key in t else v1 * v2
            return Fn(t)
        o = rf(o)
        return Fn({key: v * o for key, v in self.t.items()})

    __rmul__ = __mul__


def fnc(x):
    return x if isinstance(x, Fn) else Fn({(0, 0, 0): rf(x)})


_WALLIS = {}


def wallis(a, b):
    """int_0^{pi/2} cos^a sin^b, exact."""
    key = (a, b)
    if key not in _WALLIS:
        if a >= 2:
            v = rf(Fr(a - 1, a + b)) * wallis(a - 2, b)
        elif b >= 2:
            v = rf(Fr(b - 1, a + b)) * wallis(a, b - 2)
        else:
            v = {(0, 0): PI * rf(Fr(1, 2)), (1, 0): ONE, (0, 1): ONE, (1, 1): rf(Fr(1, 2))}[key]
        _WALLIS[key] = v
    return _WALLIS[key]


class AMember:
    def __init__(self, mdl, mbr, P, e0):
        X = mdl["nodes"]
        self.id, self.kind = mbr["id"], mbr["kind"]
        self.Xi, self.Xj = rv(X[mbr["i"]]), rv(X[mbr["j"]])
        self.k = rf(mbr.get("k", 1))
        self.e0 = e0
        self.timo = mdl["shear"] == "timoshenko" and self.kind == "straight"
        if self.kind == "straight":
            e, L = unit_axis(X[mbr["i"]], X[mbr["j"]])
            self.L = L
            self.X = [Fn({(0, 0, 0): self.Xi[c], (0, 0, 1): e[c]}) for c in range(3)]
            self.t = [fnc(e[c]) for c in range(3)]
            self.ti = self.tj = rv(e)
            self.own_F = [fnc(0)] * 3
            self.own_M = [fnc(0)] * 3
            self.full_F, self.full_M0 = [ZERO] * 3, [ZERO] * 3
        else:
            R, e1, e2, n = arc_frame(mdl, mbr)
            self.R = rf(R)
            C = rv(mbr["center"])
            self.X = [Fn({(0, 0, 0): C[c], (1, 0, 0): R * e1[c], (0, 1, 0): R * e2[c]}) for c in range(3)]
            self.t = [Fn({(0, 1, 0): -e1[c], (1, 0, 0): e2[c]}) for c in range(3)]
            self.ti, self.tj = rv(e2), rv([-x for x in e1])
            # fluid wall load p*A_i/R along e_r: resultant over [psi, pi/2] and its moment about X(psi)
            self.own_F = [Fn({(0, 0, 0): P * rf(e1[c]), (0, 1, 0): -P * rf(e1[c]), (1, 0, 0): P * rf(e2[c])})
                          for c in range(3)]
            self.own_M = [Fn({(0, 0, 0): -P * self.R * rf(n[c]), (0, 1, 0): P * self.R * rf(n[c])})
                          for c in range(3)]
            self.full_F = [P * rf(e1[c] + e2[c]) for c in range(3)]
            self.full_M0 = vadd([-P * self.R * rf(n[c]) for c in range(3)], vcross(self.Xi, self.full_F))

    def integrate(self, f):
        tot = ZERO
        for (a, b, nn), c in f.t.items():
            if self.kind == "straight":
                assert a == 0 and b == 0
                tot = tot + c * rf(self.L ** (nn + 1) / (nn + 1))
            else:
                assert nn == 0
                tot = tot + c * wallis(a, b) * self.R
        return tot


def fdot(a, b):
    out = fnc(0)
    for x, y in zip(a, b):
        out = out + x * y
    return out


def fcross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


class FormulationA:
    def __init__(self, mdl, lc):
        self.mdl, self.lc = mdl, lc
        p = lc["p"]
        P = rf(p) * AI
        self.P = P
        self.chain = mdl["chain"]
        self.idx = {n: i for i, n in enumerate(self.chain)}
        self.members = [AMember(mdl, m, P, e_poisson(p) if p != 0 else ZERO) for m in mdl["members"]]
        for m, mbr in zip(self.members, mdl["members"]):
            assert self.chain[self.members.index(m)] == mbr["i"]
        for k in range(len(self.members) - 1):
            assert self.members[k].tj == self.members[k + 1].ti, "tangent-continuous chain (no kink terms)"
        self.pos = {n: rv(mdl["nodes"][n]) for n in self.chain}
        # physical point loads: applied loads and caps p*A_i on the wall, outward
        self.pts = {n: [ZERO] * 6 for n in self.chain}
        for n, v in lc["loads"].items():
            self.pts[n] = vadd(self.pts[n], rv(v))
        if p != 0:
            first, last = self.members[0], self.members[-1]
            if mdl["terminals"][self.chain[0]] == "transfers_to_wall":
                self.pts[self.chain[0]] = vadd(self.pts[self.chain[0]], vsc(first.ti, -P) + [ZERO] * 3)
            if mdl["terminals"][self.chain[-1]] == "transfers_to_wall":
                self.pts[self.chain[-1]] = vadd(self.pts[self.chain[-1]], vsc(last.tj, P) + [ZERO] * 3)
        self.red = [(r["node"], r["dof"]) for r in mdl["restraints"]] + \
                   [(s["node"], s["dof"]) for s in mdl["nonlinear"]]
        self._cache = {}

    def after(self, m, extra):
        """(F, M about origin) of point loads at chain nodes > m and full member loads beyond member m."""
        F, M = [ZERO] * 3, [ZERO] * 3
        for n in self.chain[m + 1:]:
            pl = vadd(self.pts[n], extra.get(n, [ZERO] * 6))
            F = vadd(F, pl[:3])
            M = vadd(M, vadd(pl[3:], vcross(self.pos[n], pl[:3])))
        for mm in self.members[m + 1:]:
            F = vadd(F, mm.full_F)
            M = vadd(M, mm.full_M0)
        return F, M

    def actual_cut(self, m):
        mem = self.members[m]
        F, M0 = self.after(m, {})
        Ff = [fnc(x) + y for x, y in zip(F, mem.own_F)]
        Mf = [fnc(x) for x in M0]
        Mf = [a - b for a, b in zip(Mf, fcross(mem.X, [fnc(x) for x in F]))]
        Mf = [a + b for a, b in zip(Mf, mem.own_M)]
        return Ff, Mf

    def unit_cut(self, m, q, a):
        mem = self.members[m]
        Fu = [ONE if (a < 3 and c == a) else ZERO for c in range(3)]
        Mu = [ONE if (a >= 3 and c == a - 3) else ZERO for c in range(3)]
        d = [fnc(x) - y for x, y in zip(self.pos[q], mem.X)]
        Mf = [fnc(x) + y for x, y in zip(Mu, fcross(d, [fnc(x) for x in Fu]))]
        return [fnc(x) for x in Fu], Mf

    def work(self, m, U, V, eig):
        mem = self.members[m]
        Fu, Mu = U
        F, M = V
        t = mem.t
        Nu, N = fdot(Fu, t), fdot(F, t)
        Tu, T = fdot(Mu, t), fdot(M, t)
        f = Nu * N * (ONE / EA) + Tu * T * (ONE / GJ) + (fdot(Mu, M) - Tu * T) * (mem.k / EI)
        if mem.timo:
            f = f + (fdot(Fu, F) - Nu * N) * (ONE / GAS)
        if eig and not mem.e0.is_zero():
            f = f + Nu * mem.e0
        return mem.integrate(f)

    def disp(self, q, a, other):
        """Displacement at (q, a) under the actual loads (other=None) or a unit load at other."""
        key = (q, a, other)
        if key in self._cache:
            return self._cache[key]
        if other is not None:
            k2 = (other[0], other[1], (q, a))
            if k2 in self._cache:
                return self._cache[k2]
        tot = ZERO
        iq = self.idx[q]
        lim = iq if other is None else min(iq, self.idx[other[0]])
        for m in range(lim):
            U = self.unit_cut(m, q, a)
            if other is None:
                tot = tot + self.work(m, U, self.actual_cut(m), True)
            else:
                tot = tot + self.work(m, U, self.unit_cut(m, other[0], other[1]), False)
        self._cache[key] = tot
        return tot

    def solve_state(self, states):
        """states: support id -> state. Returns the solved result or None if singular."""
        mdl = self.mdl
        nl = {s["id"]: s for s in mdl["nonlinear"]}
        red_ids = [r["id"] for r in mdl["restraints"]] + [s["id"] for s in mdl["nonlinear"]]
        kinds = {}
        for r in mdl["restraints"]:
            kinds[r["id"]] = ("fixed", ZERO)
        for sid, sup in nl.items():
            st = states[sid]
            if sup["kind"] == "one_way":
                kinds[sid] = ("fixed", ZERO) if st == "active" else ("zero",)
            elif sup["kind"] == "gap":
                kinds[sid] = ("fixed", rf(sup["closes"] * sup["clearance"])) if st == "closed" else ("zero",)
            else:
                if st == "stick":
                    kinds[sid] = ("fixed", ZERO)
                elif st == "inactive":
                    kinds[sid] = ("zero",)
                elif "explicit_normal" in sup:
                    kinds[sid] = ("known", rf(-slide_dir(st)) * rf(sup["mu"]) * rf(sup["explicit_normal"]))
                else:
                    src = [r["id"] for r in mdl["restraints"] if (r["node"], r["dof"]) == sup["source"]]
                    assert len(src) == 1, "formulation A takes a derived normal from a redundant restraint"
                    kinds[sid] = ("coupled", src[0], rf(-slide_dir(st) * slide_beta(st)) * rf(sup["mu"]))
        unknown = [i for i in red_ids if kinds[i][0] in ("fixed",)]
        pos = {i: k for k, i in enumerate(red_ids)}

        def coeffs(i):
            """X_i as a linear form over unknowns: (dict unknown->coef, constant)."""
            kd = kinds[i]
            if kd[0] == "fixed":
                return {i: ONE}, ZERO
            if kd[0] == "zero":
                return {}, ZERO
            if kd[0] == "known":
                return {}, kd[1]
            return {kd[1]: kd[2]}, ZERO

        rows, rhs = [], []
        for i in unknown:
            qi = self.red[pos[i]]
            row = {u: ZERO for u in unknown}
            const = self.disp(qi[0], qi[1], None)
            for j in red_ids:
                cj, kj = coeffs(j)
                fij = self.disp(qi[0], qi[1], self.red[pos[j]])
                for u, c in cj.items():
                    row[u] = row[u] + c * fij
                const = const + kj * fij
            rows.append([row[u] for u in unknown])
            rhs.append([kinds[i][1] - const])
        sol = gauss_jordan(rows, rhs) if unknown else []
        Xval = {u: sol[k][0] for k, u in enumerate(unknown)}
        X = {}
        for i in red_ids:
            cj, kj = coeffs(i)
            X[i] = kj
            for u, c in cj.items():
                X[i] = X[i] + c * Xval[u]
        return self.result(X, red_ids, pos)

    def result(self, X, red_ids, pos):
        extra = {}
        for i in red_ids:
            n, d = self.red[pos[i]]
            v = [ZERO] * 6
            v[d] = X[i]
            extra[n] = vadd(extra.get(n, [ZERO] * 6), v)
        u = {}
        for n in self.chain:
            u[n] = []
            for a in range(6):
                val = self.disp(n, a, None)
                for i in red_ids:
                    if not X[i].is_zero():
                        val = val + X[i] * self.disp(n, a, self.red[pos[i]])
                u[n].append(val)
        # anchor reaction by global equilibrium (moments about the anchor node)
        a0 = self.chain[0]
        F, M = [ZERO] * 3, [ZERO] * 3
        for n in self.chain:
            pl = vadd(self.pts[n], extra.get(n, [ZERO] * 6))
            F = vadd(F, pl[:3])
            M = vadd(M, vadd(pl[3:], vcross(vsub(self.pos[n], self.pos[a0]), pl[:3])))
        for mm in self.members:
            F = vadd(F, mm.full_F)
            M = vadd(M, vsub(mm.full_M0, vcross(self.pos[a0], mm.full_F)))
        R = {self.mdl["anchor"]["id"]: [-x for x in F + M]}
        for i in red_ids:
            R[i] = X[i]
        ends = {}
        for m, mem in enumerate(self.members):
            ni, nj = self.chain[m], self.chain[m + 1]
            Fj, M0j = self.after(m, extra)
            Mj = vsub(M0j, vcross(self.pos[nj], Fj))
            Fi = vadd(Fj, mem.full_F)
            M0i = vadd(M0j, mem.full_M0)
            Mi = vsub(M0i, vcross(self.pos[ni], Fi))
            ends[mem.id] = {ni: [-x for x in Fi + Mi], nj: Fj + Mj}
        Nw = {mem.id: {self.chain[m]: vdot(ends[mem.id][self.chain[m]][:3], vsc(mem.ti, -1)),
                       self.chain[m + 1]: vdot(ends[mem.id][self.chain[m + 1]][:3], mem.tj)}
              for m, mem in enumerate(self.members)}
        return {"u": u, "R": R, "end": ends, "Nw": Nw}

    def hand_solve(self, order):
        """Hand-calculation trials: the first state in `order` that satisfies every condition."""
        for states in order:
            res = self.solve_state(states)
            if all_valid(self.mdl, states, res):
                return states, res
        raise SystemExit("formulation A found no admissible state")


def support_quantities(mdl, sup, res):
    """(u_d, r_d, normal) of a nonlinear support in a solved result."""
    u_d = res["u"][sup["node"]][sup["dof"]]
    r_d = res["R"][sup["id"]]
    normal = None
    if sup["kind"] == "friction" and "explicit_normal" not in sup:
        normal = source_reaction(mdl, sup, res)
    return u_d, r_d, normal


def source_reaction(mdl, sup, res):
    n, d = sup["source"]
    for r in mdl["restraints"]:
        if (r["node"], r["dof"]) == (n, d):
            return res["R"][r["id"]]
    assert n == mdl["anchor"]["node"]
    return res["R"][mdl["anchor"]["id"]][d]


def all_valid(mdl, states, res):
    for sup in mdl["nonlinear"]:
        u_d, r_d, normal = support_quantities(mdl, sup, res)
        if not strict_valid(sup, states[sup["id"]], u_d, r_d, normal):
            return False
    return True


# ---------------------------------------------------------------------------
# Formulation B: assembled 3-D stiffness with the H-2 pressure ledger


def local_frame(e):
    ref = [Fr(0), Fr(0), Fr(1)] if e[2] == 0 else [Fr(1), Fr(0), Fr(0)]
    d = sum(r * x for r, x in zip(ref, e))
    y = [r - d * x for r, x in zip(ref, e)]
    z = [e[1] * y[2] - e[2] * y[1], e[2] * y[0] - e[0] * y[2], e[0] * y[1] - e[1] * y[0]]
    return [list(e), y, z]


def straight_local_k(L, timo):
    L = rf(L)
    phi = rf(12) * EI / (GAS * L * L) if timo else ZERO
    a = rf(12) * EI / (L * L * L * (ONE + phi))
    b = rf(6) * EI / (L * L * (ONE + phi))
    c = (rf(4) + phi) * EI / (L * (ONE + phi))
    d = (rf(2) - phi) * EI / (L * (ONE + phi))
    K = [[ZERO] * 12 for _ in range(12)]

    def put(ix, mat):
        for r, ir in enumerate(ix):
            for s, js in enumerate(ix):
                K[ir][js] = K[ir][js] + mat[r][s]
    put([0, 6], [[EA / L, -EA / L], [-EA / L, EA / L]])
    put([3, 9], [[GJ / L, -GJ / L], [-GJ / L, GJ / L]])
    put([1, 5, 7, 11], [[a, b, -a, b], [b, c, -b, d], [-a, -b, a, -b], [b, d, -b, c]])
    put([2, 4, 8, 10], [[a, -b, -a, -b], [-b, c, b, d], [-a, b, a, b], [-b, d, b, c]])
    return K


def rot12(lam):
    T = [[ZERO] * 12 for _ in range(12)]
    for blk in range(4):
        for r in range(3):
            for s in range(3):
                T[3 * blk + r][3 * blk + s] = rf(lam[r][s])
    return T


def congruence(T, k):
    """T^T k T."""
    n = len(T[0])
    kT = [[sum((k[r][a] * T[a][s] for a in range(len(k)) if not T[a][s].is_zero() and not k[r][a].is_zero()), ZERO)
           for s in range(n)] for r in range(len(k))]
    return [[sum((T[a][r] * kT[a][s] for a in range(len(k)) if not T[a][r].is_zero() and not kT[a][s].is_zero()), ZERO)
             for s in range(n)] for r in range(n)]


def arc_flexibility_local(R, k):
    """Closed-form quarter-circle cantilever flexibility (clamped at i, loads at j), basis (e1, e2, n);
    order F1, F2, Fn, M1, M2, Mn. Euler-Bernoulli; k on both bending terms (T4-I7 definition)."""
    R, k = rf(R), rf(k)
    q = PI * rf(Fr(1, 4))
    h = PI * rf(Fr(1, 2))
    f = [[ZERO] * 6 for _ in range(6)]

    def s(i, j, v):
        f[i][j] = v
        f[j][i] = v
    # in plane: F1, F2, Mn
    s(0, 0, R * (q / EA + k * R * R * (rf(3) * q - rf(2)) / EI))
    s(1, 1, R * (q / EA + k * R * R * q / EI))
    s(0, 1, R * (rf(Fr(-1, 2)) / EA + k * R * R * rf(Fr(1, 2)) / EI))
    s(0, 5, -k * R * R * (h - ONE) / EI)
    s(1, 5, -k * R * R / EI)
    s(5, 5, k * R * h / EI)
    # out of plane: Fn, M1, M2
    s(2, 2, R * (R * R * (rf(3) * q - rf(2)) / GJ + k * R * R * q / EI))
    s(2, 3, R * (-R * (ONE - q) / GJ + k * R * q / EI))
    s(2, 4, R * (R * rf(Fr(1, 2)) / GJ + k * R * rf(Fr(1, 2)) / EI))
    s(3, 3, R * (q / GJ + k * q / EI))
    s(4, 4, R * (q / GJ + k * q / EI))
    s(3, 4, R * (rf(Fr(-1, 2)) / GJ + k * rf(Fr(1, 2)) / EI))
    return f


def straight_flexibility_local(L):
    """Straight cantilever flexibility in its own frame (x along the member), order Fx Fy Fz Mx My Mz."""
    L = rf(L)
    f = [[ZERO] * 6 for _ in range(6)]

    def s(i, j, v):
        f[i][j] = v
        f[j][i] = v
    s(0, 0, L / EA)
    s(1, 1, L * L * L / (rf(3) * EI))
    s(1, 5, L * L / (rf(2) * EI))
    s(5, 5, L / EI)
    s(2, 2, L * L * L / (rf(3) * EI))
    s(2, 4, -L * L / (rf(2) * EI))
    s(4, 4, L / EI)
    s(3, 3, L / GJ)
    return f


def stiffness_from_flexibility(flex, basis, r):
    """12x12 global stiffness from a 6x6 cantilever flexibility in `basis` (rows = local axes),
    with r = X_j - X_i. P_j = K (u_j - u_i - theta_i x r, theta_j - theta_i)."""
    kl = [list(row) for row in gauss_jordan(flex, [[ONE if i == j else ZERO for j in range(6)] for i in range(6)])]
    Rm = [[ZERO] * 6 for _ in range(6)]
    for blk in range(2):
        for a in range(3):
            for b in range(3):
                Rm[3 * blk + a][3 * blk + b] = rf(basis[a][b])
    kg = congruence(Rm, kl)
    r = rv(r)
    skew = [[ZERO, -r[2], r[1]], [r[2], ZERO, -r[0]], [-r[1], r[0], ZERO]]
    Bm = [[ZERO] * 12 for _ in range(6)]
    for a in range(3):
        Bm[a][a] = -ONE
        Bm[3 + a][3 + a] = -ONE
        Bm[a][6 + a] = ONE
        Bm[3 + a][9 + a] = ONE
        for b in range(3):
            Bm[a][3 + b] = skew[a][b]
    return congruence(Bm, kg)


class FormulationB:
    def __init__(self, mdl, lc):
        self.mdl, self.lc = mdl, lc
        p = lc["p"]
        P = rf(p) * AI
        self.P = P
        names = list(mdl["nodes"])
        self.names = names
        self.idx = {n: i for i, n in enumerate(names)}
        nd = 6 * len(names)
        self.nd = nd
        K = [[ZERO] * nd for _ in range(nd)]
        f = [ZERO] * nd
        for n, v in lc["loads"].items():
            for c in range(6):
                f[6 * self.idx[n] + c] = f[6 * self.idx[n] + c] + rf(v[c])
        mut = lc.get("ledger_mutation")
        self.elems = []
        X = mdl["nodes"]
        for mbr in mdl["members"]:
            gi = [6 * self.idx[mbr["i"]] + c for c in range(6)] + [6 * self.idx[mbr["j"]] + c for c in range(6)]
            if mbr["kind"] == "straight":
                e, L = unit_axis(X[mbr["i"]], X[mbr["j"]])
                T = rot12(local_frame(e))
                kg = congruence(T, straight_local_k(L, mdl["shear"] == "timoshenko"))
                fel = [ZERO] * 12
                if p != 0:
                    # Poisson pair: held tension N0 = 2 nu p A_i; equivalent loads are its negatives
                    N0 = rf(2 * NU) * P
                    for c in range(3):
                        fel[c] = N0 * rf(e[c])
                        fel[6 + c] = -N0 * rf(e[c])
            else:
                R, e1, e2, nrm = arc_frame(mdl, mbr)
                r = [Fr(b) - Fr(a) for a, b in zip(X[mbr["i"]], X[mbr["j"]])]
                kg = stiffness_from_flexibility(arc_flexibility_local(R, mbr["k"]), [e1, e2, nrm], r)
                fel = [ZERO] * 12
                if p != 0:
                    ep = eps_p(p)
                    ufree = [ZERO] * 6 + [ep * rf(x) for x in r] + [ZERO] * 3
                    kuf = [sum((kg[a][b] * ufree[b] for b in range(12) if not ufree[b].is_zero()), ZERO)
                           for a in range(12)]
                    ti, tj = rv(e2), rv([-x for x in e1])
                    cb = vsc(ti, -P) + [ZERO] * 3 + vsc(tj, P) + [ZERO] * 3
                    keep_u = mut not in ("drop_arc_ufree", "drop_arc_all")
                    keep_c = mut != "drop_arc_all"
                    fel = [(kuf[a] if keep_u else ZERO) - (cb[a] if keep_c else ZERO) for a in range(12)]
            for a in range(12):
                f[gi[a]] = f[gi[a]] + fel[a]
                for b in range(12):
                    if not kg[a][b].is_zero():
                        K[gi[a]][gi[b]] = K[gi[a]][gi[b]] + kg[a][b]
            self.elems.append((mbr, gi, kg, fel))
        # caps at transfers_to_wall terminals: -p A_i s, s from the node into the member
        if p != 0:
            ch = mdl["chain"]
            first = mdl["members"][0]
            last = mdl["members"][-1]
            if mdl["terminals"][ch[0]] == "transfers_to_wall":
                s0 = tangents(mdl, first)[0]
                for c in range(3):
                    f[6 * self.idx[ch[0]] + c] = f[6 * self.idx[ch[0]] + c] - P * rf(s0[c])
            if mdl["terminals"][ch[-1]] == "transfers_to_wall":
                s1 = [-x for x in tangents(mdl, last)[1]]
                for c in range(3):
                    f[6 * self.idx[ch[-1]] + c] = f[6 * self.idx[ch[-1]] + c] - P * rf(s1[c])
            # kink remainders p A_i (t_in - t_out) at interior chain nodes (zero for a tangent chain)
            for a_m, b_m in zip(mdl["members"], mdl["members"][1:]):
                tin, tout = tangents(mdl, a_m)[1], tangents(mdl, b_m)[0]
                for c in range(3):
                    f[6 * self.idx[a_m["j"]] + c] = f[6 * self.idx[a_m["j"]] + c] + P * rf(tin[c] - tout[c])
        self.K, self.f = K, f

    def dof(self, n, d):
        return 6 * self.idx[n] + d

    def solve_state(self, states):
        mdl = self.mdl
        cons = {}
        for c in range(6):
            cons[self.dof(mdl["anchor"]["node"], c)] = ZERO
        for r in mdl["restraints"]:
            cons[self.dof(r["node"], r["dof"])] = ZERO
        load = list(self.f)
        coupled = []
        for sup in mdl["nonlinear"]:
            st = states[sup["id"]]
            d = self.dof(sup["node"], sup["dof"])
            if sup["kind"] == "one_way" and st == "active":
                cons[d] = ZERO
            elif sup["kind"] == "gap" and st == "closed":
                cons[d] = rf(sup["closes"] * sup["clearance"])
            elif sup["kind"] == "friction":
                if st == "stick":
                    cons[d] = ZERO
                elif st.startswith("slide"):
                    if "explicit_normal" in sup:
                        load[d] = load[d] + rf(-slide_dir(st)) * rf(sup["mu"]) * rf(sup["explicit_normal"])
                    else:
                        coupled.append((d, self.dof(*sup["source"]),
                                        rf(-slide_dir(st) * slide_beta(st)) * rf(sup["mu"])))
        free = [i for i in range(self.nd) if i not in cons]
        nf = len(free)
        A = [[self.K[r][c] for c in free] + [ZERO] * len(coupled) for r in free]
        b = [load[r] - sum((self.K[r][c] * v for c, v in cons.items() if not v.is_zero()), ZERO) for r in free]
        for k, (d, n, cf) in enumerate(coupled):
            A[free.index(d)][nf + k] = -ONE          # K_FF u - phi e_d = f
            # phi - cf * (K_n u - f_n) = 0
            row = [-cf * self.K[n][c] for c in free] + [ZERO] * len(coupled)
            row[nf + k] = ONE
            A.append(row)
            b.append(-cf * (load[n] - sum((self.K[n][c] * v for c, v in cons.items() if not v.is_zero()), ZERO)))
        sol = gauss_jordan(A, [[x] for x in b])
        u = [ZERO] * self.nd
        for c, v in cons.items():
            u[c] = v
        for i, c in enumerate(free):
            u[c] = sol[i][0]
        phis = [sol[nf + k][0] for k in range(len(coupled))]
        Ku = [sum((self.K[r][c] * u[c] for c in range(self.nd) if not self.K[r][c].is_zero() and not u[c].is_zero()), ZERO)
              for r in range(self.nd)]
        resid = [Ku[r] - load[r] for r in range(self.nd)]
        # explicit sliding force: support-on-pipe force = the applied friction load at that DOF
        for sup in mdl["nonlinear"]:
            st = states[sup["id"]]
            if sup["kind"] == "friction" and st.startswith("slide") and "explicit_normal" in sup:
                d = self.dof(sup["node"], sup["dof"])
                resid[d] = resid[d] + (load[d] - self.f[d])
        res = {"u": {n: u[6 * self.idx[n]:6 * self.idx[n] + 6] for n in self.names},
               "R": {mdl["anchor"]["id"]: resid[6 * self.idx[mdl["anchor"]["node"]]:6 * self.idx[mdl["anchor"]["node"]] + 6]},
               "free_resid": [], "end": {}, "Nw": {}, "phis": phis, "resid": resid}
        for r in mdl["restraints"]:
            res["R"][r["id"]] = resid[self.dof(r["node"], r["dof"])]
        nl_dofs = set()
        for sup in mdl["nonlinear"]:
            d = self.dof(sup["node"], sup["dof"])
            nl_dofs.add(d)
            res["R"][sup["id"]] = resid[d]
        res["free_resid"] = [resid[i] for i in free if i not in nl_dofs]
        for k, (d, n, cf) in enumerate(coupled):
            res["coupling_check"] = (phis[k], cf * resid[n], resid[d])
        for mbr, gi, kg, fel in self.elems:
            ug = [u[g] for g in gi]
            fe = [sum((kg[a][b] * ug[b] for b in range(12) if not kg[a][b].is_zero() and not ug[b].is_zero()), ZERO) - fel[a]
                  for a in range(12)]
            res["end"][mbr["id"]] = {mbr["i"]: fe[:6], mbr["j"]: fe[6:]}
            ti, tj = tangents(mdl, mbr)
            res["Nw"][mbr["id"]] = {mbr["i"]: vdot(fe[:3], vsc(rv(ti), -1)), mbr["j"]: vdot(fe[6:9], rv(tj))}
        return res

    def enumerate_states(self):
        sups = self.mdl["nonlinear"]
        valid = []
        tried = 0
        for combo in product(*[state_options(s) for s in sups]):
            states = {s["id"]: st for s, st in zip(sups, combo)}
            res = self.solve_state(states)
            tried += 1
            if all_valid(self.mdl, states, res):
                valid.append((states, res))
        return valid, tried


# ---------------------------------------------------------------------------
# Checks


class Checks:
    def __init__(self):
        self.items = []

    def eq(self, name, a, b):
        ok = rf(a) == rf(b)
        self.items.append({"check": name, "pass": ok})
        if not ok:
            raise SystemExit(f"check failed: {name}: {rf_text(a)} != {rf_text(b)}")
        return ok

    def true(self, name, cond, detail=""):
        self.items.append({"check": name, "pass": bool(cond)})
        if not cond:
            raise SystemExit(f"check failed: {name} {detail}")
        return True


def compare_ab(chk, lab, a, b):
    for n in b["u"]:
        for c in range(6):
            chk.eq(f"{lab}: u[{n}].{DOFS[c]} A==B", a["u"][n][c], b["u"][n][c])
    for sid in b["R"]:
        va, vb = a["R"][sid], b["R"][sid]
        if isinstance(vb, list):
            for c in range(6):
                chk.eq(f"{lab}: R[{sid}].{FKEYS[c]} A==B", va[c], vb[c])
        else:
            chk.eq(f"{lab}: R[{sid}] A==B", va, vb)
    for m in b["end"]:
        for n in b["end"][m]:
            for c in range(6):
                chk.eq(f"{lab}: end[{m}@{n}].{FKEYS[c]} A==B", a["end"][m][n][c], b["end"][m][n][c])
            chk.eq(f"{lab}: N_w[{m}@{n}] A==B", a["Nw"][m][n], b["Nw"][m][n])


def b_internal_checks(chk, lab, mdl, lc, B, res):
    chk.true(f"{lab}: B free-DOF residual zero", all(r.is_zero() for r in res["free_resid"]))
    pos = {n: rv(mdl["nodes"][n]) for n in mdl["nodes"]}
    # global equilibrium: PHYSICAL loads (applied, caps on the wall, arc fluid wall loads) with B's support
    # actions (K u - f_ledger at every DOF; zero at free DOFs, the friction force at a sliding DOF)
    F, M = [ZERO] * 3, [ZERO] * 3
    P = B.P
    phys = {n: [ZERO] * 6 for n in mdl["nodes"]}
    for n, v in lc["loads"].items():
        phys[n] = vadd(phys[n], rv(v))
    if lc["p"] != 0:
        ch = mdl["chain"]
        if mdl["terminals"][ch[0]] == "transfers_to_wall":
            phys[ch[0]] = vadd(phys[ch[0]], vsc(rv(tangents(mdl, mdl["members"][0])[0]), -P) + [ZERO] * 3)
        if mdl["terminals"][ch[-1]] == "transfers_to_wall":
            phys[ch[-1]] = vadd(phys[ch[-1]], vsc(rv(tangents(mdl, mdl["members"][-1])[1]), P) + [ZERO] * 3)
        for mbr in mdl["members"]:
            if mbr["kind"] == "arc":
                R, e1, e2, n = arc_frame(mdl, mbr)
                Fd = vsc(rv([x + y for x, y in zip(e1, e2)]), P)
                F = vadd(F, Fd)
                M = vadd(M, vadd(vsc(rv(n), -P * rf(R)), vcross(pos[mbr["i"]], Fd)))
    for n in mdl["nodes"]:
        i = 6 * B.idx[n]
        v = vadd(phys[n], [res["resid"][i + c] for c in range(6)])
        F = vadd(F, v[:3])
        M = vadd(M, vadd(v[3:], vcross(pos[n], v[:3])))
    chk.true(f"{lab}: global balance of physical loads and B's support actions", all(x.is_zero() for x in F + M))
    # node equilibrium: physical nodal loads + support actions = sum of the wall end actions at the node
    for n in mdl["nodes"]:
        i = 6 * B.idx[n]
        lhs = vadd(phys[n], [res["resid"][i + c] for c in range(6)])
        rhs = [ZERO] * 6
        for mbr, gi, kg, fel in B.elems:
            if n in res["end"][mbr["id"]]:
                rhs = vadd(rhs, res["end"][mbr["id"]][n])
        chk.true(f"{lab}: node {n} equilibrium (physical loads, support actions, wall end actions)",
                 all((x - y).is_zero() for x, y in zip(lhs, rhs)))
    # member equilibrium of wall end actions with the physical arc wall load
    for mbr, gi, kg, fel in B.elems:
        ea = res["end"][mbr["id"]]
        fi, fj = ea[mbr["i"]], ea[mbr["j"]]
        Fd, Md = [ZERO] * 3, [ZERO] * 3
        if mbr["kind"] == "arc" and lc["p"] != 0:
            R, e1, e2, n = arc_frame(mdl, mbr)
            Fd = vsc(rv([a + b for a, b in zip(e1, e2)]), B.P)
            Md = vadd(vsc(rv(n), -B.P * rf(R)), vcross(pos[mbr["i"]], Fd))
        Ft = vadd(vadd(fi[:3], fj[:3]), Fd)
        Mt = vadd(vadd(vadd(fi[3:], vcross(pos[mbr["i"]], fi[:3])), vadd(fj[3:], vcross(pos[mbr["j"]], fj[:3]))), Md)
        chk.true(f"{lab}: {mbr['id']} wall end actions balance its physical wall load", all(x.is_zero() for x in Ft + Mt))
        if mbr["kind"] == "straight":
            e, L = unit_axis(mdl["nodes"][mbr["i"]], mdl["nodes"][mbr["j"]])
            el = vdot(vsub(res["u"][mbr["j"]][:3], res["u"][mbr["i"]][:3]), rv(e))
            e0 = e_poisson(lc["p"]) if lc["p"] != 0 else ZERO
            chk.eq(f"{lab}: {mbr['id']} constitutive E A (elong/L - eps0) = N_w - p A_i + p A_i",
                   EA * (el / rf(L) - e0), res["Nw"][mbr["id"]][mbr["j"]])
    chk.true(f"{lab}: anchor motion zero", all(x.is_zero() for x in res["u"][mdl["anchor"]["node"]]))
    if "coupling_check" in res:
        phi, crn, rd = res["coupling_check"]
        chk.eq(f"{lab}: B derived friction phi = -dir mu beta R_n", phi, crn)
        chk.eq(f"{lab}: B sliding DOF action equals phi", rd, phi)


def complementarity_checks(chk, lab, mdl, states, res):
    out = {}
    for sup in mdl["nonlinear"]:
        st = states[sup["id"]]
        u_d, r_d, normal = support_quantities(mdl, sup, res)
        sid = sup["id"]
        if sup["kind"] in ("gap", "one_way"):
            if sup["kind"] == "gap":
                s, c = sup["closes"], sup["clearance"]
                sep = rf(c) - rf(s) * u_d
                lam = -rf(s) * r_d
            else:
                s = sup["sense"]
                sep = rf(s) * u_d
                lam = rf(s) * r_d
            chk.true(f"{lab}: {sid} separation >= 0", sign(sep) >= 0)
            chk.true(f"{lab}: {sid} contact force >= 0", sign(lam) >= 0)
            chk.true(f"{lab}: {sid} separation * contact = 0", (sep * lam).is_zero())
            chk.true(f"{lab}: {sid} strictly complementary (not weakly active)", sign(sep) + sign(lam) == 1)
            out[sid] = {"state": st, "u_d": u_d, "reaction": r_d, "separation": sep, "contact": lam}
        else:
            mu = rf(sup["mu"])
            N = rf(sup["explicit_normal"]) if "explicit_normal" in sup else rabs(normal)
            chk.true(f"{lab}: {sid} |R_t| <= mu N", sign(mu * N - rabs(r_d)) >= 0)
            q = {"state": st, "u_d": u_d, "reaction": r_d, "N": N, "muN": mu * N}
            if normal is not None:
                q["source_reaction"] = normal
            if st == "stick":
                chk.true(f"{lab}: {sid} stick: u_t = 0", u_d.is_zero())
                chk.true(f"{lab}: {sid} stick margin mu N - |R_t| > 0", sign(mu * N - rabs(r_d)) > 0)
                q["stick_margin"] = mu * N - rabs(r_d)
            elif st.startswith("slide"):
                dr = slide_dir(st)
                chk.true(f"{lab}: {sid} slip opposes motion: sign(u_t) = dir", sign(u_d) == dr)
                chk.eq(f"{lab}: {sid} sliding force = -dir mu N", r_d, rf(-dr) * mu * N)
            else:
                chk.true(f"{lab}: {sid} inactive: N = 0 and no friction force", N.is_zero() and r_d.is_zero())
            out[sid] = q
    return out


# ---------------------------------------------------------------------------
# Cases


def solve_case(chk, lab, mdl, lc):
    A = FormulationA(mdl, lc)
    B = FormulationB(mdl, lc)
    valid, tried = B.enumerate_states()
    chk.true(f"{lab}: B exactly one admissible state of {tried}", len(valid) == 1, f"({len(valid)} valid)")
    states_b, rb = valid[0]
    order = []
    for combo in product(*[(["open", "closed"] if s["kind"] == "gap" else
                            ["inactive", "active"] if s["kind"] == "one_way" else state_options(s))
                           for s in mdl["nonlinear"]]):
        order.append(dict(zip([s["id"] for s in mdl["nonlinear"]], combo)))
    states_a, ra = A.hand_solve(order)
    chk.true(f"{lab}: A hand-calculation state equals B's unique state", states_a == states_b)
    compare_ab(chk, lab, ra, rb)
    b_internal_checks(chk, lab, mdl, lc, B, rb)
    dec = complementarity_checks(chk, lab, mdl, states_b, rb)
    return states_b, rb, dec, B


def solve_b_only(mdl, lc):
    B = FormulationB(mdl, lc)
    valid, tried = B.enumerate_states()
    if len(valid) != 1:
        raise SystemExit(f"mutant has {len(valid)} admissible states")
    return valid[0]


# ---------------------------------------------------------------------------
# Publication


def pub_vec(v, units):
    return {k: num(x, u) for k, x, u in zip(units[0], v, units[1])}


U_UNITS = (DOFS, ("m", "m", "m", "rad", "rad", "rad"))
F_UNITS = (FKEYS, ("N", "N", "N", "N*m", "N*m", "N*m"))


def pub_result(mdl, lc, states, res, dec):
    P = rf(lc["p"]) * AI
    out = {"support_states": dict(states)}
    out["displacements"] = {n: pub_vec(res["u"][n], U_UNITS) for n in mdl["nodes"]}
    R = {}
    for sid, v in res["R"].items():
        R[sid] = pub_vec(v, F_UNITS) if isinstance(v, list) else None
    for r in mdl["restraints"]:
        R[r["id"]] = {FKEYS[r["dof"]]: num(res["R"][r["id"]], "N")}
    for s in mdl["nonlinear"]:
        R[s["id"]] = {FKEYS[s["dof"]]: num(res["R"][s["id"]], "N")}
    out["reactions"] = R
    mem = {}
    for mbr in mdl["members"]:
        ends = {}
        for n in (mbr["i"], mbr["j"]):
            Nw = res["Nw"][mbr["id"]][n]
            ends[n] = {
                "wall_end_action_global": pub_vec(res["end"][mbr["id"]][n], F_UNITS),
                "N_w": num(Nw, "N"),
                "S_effective": num(Nw - P, "N"),
                "axial_wall_stress": num(Nw / AN, "Pa"),
            }
        mem[mbr["id"]] = ends
    out["members"] = mem
    nl = {}
    for sid, q in dec.items():
        d = {"state": q["state"]}
        for key, unit in (("u_d", "m"), ("reaction", "N"), ("separation", "m"), ("contact", "N"), ("N", "N"),
                          ("muN", "N"), ("source_reaction", "N"), ("stick_margin", "N")):
            if key in q:
                d[key] = num(q[key], unit)
        nl[sid] = d
    out["nonlinear_supports"] = nl
    return out


def lame(p):
    p = rf(p)
    ro2, ri2 = rf(RO * RO), rf(RI * RI)
    return {
        "hoop_inner": num(p * (ro2 + ri2) / (ro2 - ri2), "Pa"),
        "radial_inner": num(-p, "Pa"),
        "hoop_outer": num(rf(2) * p * ri2 / (ro2 - ri2), "Pa"),
        "radial_outer": num(ZERO, "Pa"),
    }


def model_inputs(mdl):
    def s(x):
        return str(Fr(x))
    out = {
        "nodes_m": {n: [s(c) for c in v] for n, v in mdl["nodes"].items()},
        "members": [],
        "anchor": mdl["anchor"],
        "region": {"members": [m["id"] for m in mdl["members"]],
                   "terminals": dict(mdl["terminals"])},
        "linear_restraints": [{"id": r["id"], "node": r["node"], "dof": DOFS[r["dof"]]} for r in mdl["restraints"]],
        "nonlinear_supports": [],
        "shear_model": mdl["shear"],
    }
    for m in mdl["members"]:
        d = {"id": m["id"], "kind": "realized_arc" if m["kind"] == "arc" else "straight", "from": m["i"], "to": m["j"]}
        if m["kind"] == "arc":
            d.update({"center_m": [s(c) for c in m["center"]], "radius_m": s(m["R"]), "angle": "pi/2",
                      "flexibility_factor_k": s(m["k"])})
        out["members"].append(d)
    for sp in mdl["nonlinear"]:
        d = {"id": sp["id"], "kind": sp["kind"], "node": sp["node"], "dof": DOFS[sp["dof"]]}
        if sp["kind"] == "gap":
            d.update({"closes": "+" if sp["closes"] > 0 else "-", "clearance_m": s(sp["clearance"])})
        elif sp["kind"] == "one_way":
            d.update({"sense": "+" if sp["sense"] > 0 else "-",
                      "law": "pushes the pipe only along +Z; active iff its reaction > 0; lifts off when u_Z > 0"})
        else:
            d["mu"] = s(sp["mu"])
            if "explicit_normal" in sp:
                d["normal"] = {"kind": "explicit", "value_N": s(sp["explicit_normal"])}
            else:
                d["normal"] = {"kind": "derived", "source_node": sp["source"][0], "source_dof": DOFS[sp["source"][1]],
                               "N": "|R_n| of that linear restraint"}
        out["nonlinear_supports"].append(d)
    return out


def lc_inputs(lc):
    return {"p_Pa": str(lc["p"]), "regions": "one region over every member" if lc["p"] != 0 else "none (no pressure_regions)",
            "nodal_loads": {n: {k: str(x) for k, x in zip(FKEYS, v) if x != 0} for n, v in lc["loads"].items()}}


GAP_STANDING = ("NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED: gap-bearing case, R1's unqualified branch; "
                "no confirmation runs until R-3 (R-4 §3.5 'Gaps'; kd5-nonlinear-active-set.md R1)")
CONF_STANDING = ("K-D5 confirmation applies (non-gap converged state); its outcome is the product's, not asserted "
                 "here. Every deciding quantity below is strictly signed, so the state is not weakly active (R3).")
DERIVED_INACTIVE = ("Expected NUMERICAL_INTEGRITY_SENSITIVE with 'support support:FRICTION: friction normal not "
                    "established': a derived-normal friction support in state Inactive fails closed "
                    "(kd5-nonlinear-active-set.md §2.2 table, §2.4 item 7). The loop rows are as below.")


def build_reference():
    chk = Checks()
    cases = []
    P10, PV, P0, P5 = Fr(10_000_000), Fr(-1_000_000), Fr(0), Fr(5_000_000)
    pi = PI
    ep10 = eps_p(P10)
    PA10 = rf(P10) * AI

    # section facts
    chk.eq("section: A_i/A_n = 81/19", AI / AN, rf(Fr(81, 19)))
    chk.eq("section: p A_i at 10 MPa = 81000 pi N", PA10, rf(81000) * pi)
    chk.eq("section: eps_p at 10 MPa = 81/950000", ep10, rf(Fr(81, 950000)))
    chk.eq("section: EI = 1719500 pi N m^2", EI, rf(1719500) * pi)
    chk.eq("section: EA = 380000000 pi N", EA, rf(380000000) * pi)

    # B's flexibility-to-stiffness map, proved on a straight member against the standard 12x12
    for e, L in (([Fr(1), Fr(0), Fr(0)], Fr(3)), ([Fr(0), Fr(0), Fr(1)], Fr(1))):
        lam = local_frame(e)
        k_std = congruence(rot12(lam), straight_local_k(L, False))
        k_flx = stiffness_from_flexibility(straight_flexibility_local(L), lam, [x * L for x in e])
        chk.true(f"B: flexibility route reproduces the standard straight 12x12 (axis {e})",
                 all(k_std[i][j] == k_flx[i][j] for i in range(12) for j in range(12)))
    # the arc element: symmetric, with six rigid-body modes
    om_ = model_o()
    arc = om_["members"][1]
    Ra, e1a, e2a, na = arc_frame(om_, arc)
    ra = [Fr(b) - Fr(a) for a, b in zip(om_["nodes"]["B1"], om_["nodes"]["B2"])]
    ka = stiffness_from_flexibility(arc_flexibility_local(Ra, arc["k"]), [e1a, e2a, na], ra)
    chk.true("B: arc 12x12 symmetric", all(ka[i][j] == ka[j][i] for i in range(12) for j in range(12)))
    Xi, Xj = rv(om_["nodes"]["B1"]), rv(om_["nodes"]["B2"])
    for ax in range(3):
        tr = [ONE if c % 6 == ax and c % 6 < 3 else ZERO for c in range(12)]
        th = [ZERO] * 3
        th[ax] = ONE
        rot = vcross(th, Xi) + th + vcross(th, Xj) + th
        for mode, v in (("translation", tr), ("rotation", rot)):
            kv = [sum((ka[i][j] * v[j] for j in range(12) if not v[j].is_zero()), ZERO) for i in range(12)]
            chk.true(f"B: arc 12x12 annihilates rigid {mode} about axis {ax}", all(x.is_zero() for x in kv))

    def add(case_id, title, mdl, lc, role, standing, closed=None, extra=None):
        lab = case_id
        states, res, dec, B = solve_case(chk, lab, mdl, lc)
        entry = {"id": case_id, "title": title, "role": role,
                 "model": model_inputs(mdl), "load_case": lc_inputs(lc),
                 "expected_standing": standing}
        entry.update(pub_result(mdl, lc, states, res, dec))
        if lc["p"] != 0:
            entry["lame_rows_unchanged_by_supports"] = lame(lc["p"])
        if closed:
            cf = {}
            for name, got, want, unit in closed(res, dec):
                if got is not None:  # got = None marks a published definition, not a check
                    chk.eq(f"{lab}: closed form: {name}", got, want)
                cf[name] = num(want, unit)
            entry["closed_form"] = cf
        if extra:
            entry.update(extra)
        cases.append(entry)
        return states, res, dec

    # ---------------- G: gap closing under pressure elongation
    def g_closed(gap_c, p, closes):
        ep = eps_p(p) if p != 0 else ZERO
        L = rf(10)
        free = ep * L

        def f(res, dec):
            q = dec["support:GAP"]
            rows = [("free elongation eps_p L", None, free, "m")]
            ov = free * rf(closes) - rf(gap_c)
            if sign(ov) > 0:
                Fc = EA / L * ov
                Pw = rf(p) * AI
                rows += [
                    ("overlap", None, ov, "m"),
                    ("contact force = (EA/L) overlap", q["contact"], Fc, "N"),
                    ("u_X(N1) = closes * clearance", res["u"]["N1"][0], rf(closes * gap_c), "m"),
                    ("N_w = p A_i - closes * F_c", res["Nw"]["pipe:1"]["N1"], Pw - rf(closes) * Fc, "N"),
                    ("axial wall stress", res["Nw"]["pipe:1"]["N1"] / AN, (Pw - rf(closes) * Fc) / AN, "Pa"),
                    ("anchor R_X = -N_w", res["R"]["support:ANCHOR"][0], -(Pw - rf(closes) * Fc), "N"),
                ]
            else:
                rows += [("u_X(N1) = free elongation", res["u"]["N1"][0], free, "m"),
                         ("contact force +0", q["contact"], ZERO, "N"),
                         ("separation = clearance - closes*u", q["separation"], rf(gap_c) - rf(closes) * free, "m"),
                         ("N_w = p A_i", res["Nw"]["pipe:1"]["N1"], rf(p) * AI, "N")]
            return rows
        return f

    s_g1, r_g1, d_g1 = add("G1", "Gap closing under pressure elongation (R-4 G-1)", model_g(+1, Fr(1, 2000)),
                           load_case(P10), "R-4 printed reference", GAP_STANDING, g_closed(Fr(1, 2000), P10, +1))
    chk.eq("G1: R-4 printed contact force 13400 pi N", d_g1["support:GAP"]["contact"], rf(13400) * pi)
    chk.eq("G1: R-4 printed overlap 67/190000 m", ep10 * rf(10) - rf(Fr(1, 2000)), rf(Fr(67, 190000)))
    chk.eq("G1: R-4 printed free elongation 81/95000 m", ep10 * rf(10), rf(Fr(81, 95000)))
    chk.eq("G1: R-4 printed N_w 67600 pi N", r_g1["Nw"]["pipe:1"]["N1"], rf(67600) * pi)
    chk.eq("G1: R-4 printed axial wall stress 676000000/19 Pa", r_g1["Nw"]["pipe:1"]["N1"] / AN, rf(Fr(676000000, 19)))
    chk.eq("G1: signed gap reaction -13400 pi N (support-on-pipe, closes in +X)", r_g1["R"]["support:GAP"], rf(-13400) * pi)

    s_g2, r_g2, d_g2 = add("G2", "The same gap staying open, g = 1 mm (R-4 G-2)", model_g(+1, Fr(1, 1000)),
                           load_case(P10), "R-4 printed reference", GAP_STANDING, g_closed(Fr(1, 1000), P10, +1))
    chk.eq("G2: R-4 printed contact +0", d_g2["support:GAP"]["contact"], ZERO)
    chk.eq("G2: R-4 printed end motion 81/95000 m", r_g2["u"]["N1"][0], rf(Fr(81, 95000)))
    chk.true("G2: state open", s_g2["support:GAP"] == "open")

    s_g3, r_g3, d_g3 = add("G3", "Vacuum: p = -1 MPa closes a -X gap of 0.05 mm (R-4 G-3)", model_g(-1, Fr(1, 20000)),
                           load_case(PV), "R-4 reference (values completed here)", GAP_STANDING,
                           g_closed(Fr(1, 20000), PV, -1))
    chk.eq("G3: R-4 printed free contraction -81/950000 m", eps_p(PV) * rf(10), rf(Fr(-81, 950000)))
    chk.eq("G3: R-4 printed overlap 67/1900000 m (0.0353 mm)", -eps_p(PV) * rf(10) - rf(Fr(1, 20000)), rf(Fr(67, 1900000)))
    chk.eq("G3: contact 1340 pi N", d_g3["support:GAP"]["contact"], rf(1340) * pi)
    chk.eq("G3: N_w = -6760 pi N", r_g3["Nw"]["pipe:1"]["N1"], rf(-6760) * pi)
    chk.eq("G3: axial wall stress -67600000/19 Pa", r_g3["Nw"]["pipe:1"]["N1"] / AN, rf(Fr(-67600000, 19)))
    for n in ("N0", "N1"):
        for c in range(6):
            chk.eq(f"G3 = -G1/10: u[{n}].{DOFS[c]}", r_g3["u"][n][c], -r_g1["u"][n][c] / rf(10))
    for c in range(6):
        chk.eq(f"G3 = -G1/10: R[ANCHOR].{FKEYS[c]}", r_g3["R"]["support:ANCHOR"][c], -r_g1["R"]["support:ANCHOR"][c] / rf(10))
    chk.eq("G3 = -G1/10: gap reaction", r_g3["R"]["support:GAP"], -r_g1["R"]["support:GAP"] / rf(10))

    s_g0, r_g0, d_g0 = add("G1-P0", "Unpressurized control of G-1 (p = 0, no regions)", model_g(+1, Fr(1, 2000)),
                           load_case(P0), "pressure isolation control", GAP_STANDING, g_closed(Fr(1, 2000), P0, +1))
    chk.true("G1-P0: every displacement and reaction is exactly zero",
             all(x.is_zero() for n in r_g0["u"] for x in r_g0["u"][n]) and
             all(x.is_zero() for x in r_g0["R"]["support:ANCHOR"]) and r_g0["R"]["support:GAP"].is_zero())
    chk.true("G1-P0: gap open with separation = clearance", s_g0["support:GAP"] == "open" and
             d_g0["support:GAP"]["separation"] == rf(Fr(1, 2000)))

    s_gh, r_gh, d_gh = add("G1-HALF", "G-1 at p = 5 MPa (combination illustration)", model_g(+1, Fr(1, 2000)),
                           load_case(P5), "illustrates NONLINEAR_COMBINATION_REQUIRES_SOLVE", GAP_STANDING,
                           g_closed(Fr(1, 2000), P5, +1))
    chk.true("G1-HALF: the gap stays open at half pressure", s_gh["support:GAP"] == "open")

    # ---------------- F: friction under a pressure-thrust normal
    L3 = rf(3)
    tipflex_eb = L3 * L3 * L3 / (rf(3) * EI)
    tipflex_t = tipflex_eb + L3 / GAS
    mu = rf(Fr(3, 10))

    def f_closed(Fz, p, N_expected, flex, explicit=False):
        def f(res, dec):
            q = dec["support:FRICTION"]
            rows = []
            if p != 0 and not explicit:
                rows.append(("derived normal N = (1-2nu) p A_i", q["N"], N_expected, "N"))
                rows.append(("source reaction R_n = -(1-2nu) p A_i", res["R"]["support:N1-UX"],
                             rf(-(1 - 2 * NU) * p) * AI, "N"))
            if p != 0:
                rows.append(("anchor R_X = -2 nu p A_i (axial wall tension at the anchor)",
                             res["R"]["support:ANCHOR"][0], rf(-2 * NU * p) * AI, "N"))
                rows.append(("N_w = 2 nu p A_i", res["Nw"]["pipe:1"]["N1"], rf(2 * NU * p) * AI, "N"))
                rows.append(("axial wall stress", res["Nw"]["pipe:1"]["N1"] / AN, rf(2 * NU * p) * AI / AN, "Pa"))
            muN = mu * N_expected
            if N_expected.is_zero():
                rows.append(("u_Z(N1) = F_z L^3/(3EI) (friction inactive)", res["u"]["N1"][2], rf(Fz) * flex, "m"))
                rows.append(("friction force +0", q["reaction"], ZERO, "N"))
            elif sign(rf(Fz) - muN) > 0:
                rows.append(("u_Z(N1) = (F_z - mu N) * tip flexibility", res["u"]["N1"][2], (rf(Fz) - muN) * flex, "m"))
                rows.append(("friction force = -mu N", q["reaction"], -muN, "N"))
                rows.append(("anchor R_Z = -(F_z - mu N)", res["R"]["support:ANCHOR"][2], -(rf(Fz) - muN), "N"))
                rows.append(("anchor M_Y = L (F_z - mu N)", res["R"]["support:ANCHOR"][4], L3 * (rf(Fz) - muN), "N*m"))
            else:
                rows.append(("u_Z(N1) = 0 (stick)", res["u"]["N1"][2], ZERO, "m"))
                rows.append(("R_t = -F_z", q["reaction"], rf(-Fz), "N"))
                rows.append(("stick margin mu N - F_z", q["stick_margin"], muN - rf(Fz), "N"))
            return rows
        return f

    N10 = rf(32400) * pi
    s, r_f1, d_f1 = add("F1-SLIDE", "Friction under a pressure-thrust normal, F_z = 50 kN: slides (R-4 F-1)", model_f(),
                        load_case(P10, {"N1": nodal(fz=50000)}), "R-4 printed reference", CONF_STANDING,
                        f_closed(50000, P10, N10, tipflex_eb))
    chk.eq("F1-SLIDE: R-4 printed N = 32400 pi N", d_f1["support:FRICTION"]["N"], N10)
    chk.eq("F1-SLIDE: R-4 printed mu N = 9720 pi N", d_f1["support:FRICTION"]["muN"], rf(9720) * pi)
    chk.eq("F1-SLIDE: R-4 printed u_z = 27(50000 - 9720 pi)/(3*1719500 pi) m", r_f1["u"]["N1"][2],
           rf(27) * (rf(50000) - rf(9720) * pi) / (rf(3 * 1719500) * pi))
    chk.eq("F1-SLIDE: R-4 printed friction -9720 pi N", r_f1["R"]["support:FRICTION"], rf(-9720) * pi)
    chk.eq("F1-SLIDE: R-4 printed anchor axial wall tension 48600 pi N", -r_f1["R"]["support:ANCHOR"][0], rf(48600) * pi)
    chk.eq("F1-SLIDE: R-4 printed sigma_a = 486000000/19 Pa", r_f1["Nw"]["pipe:1"]["N0"] / AN, rf(Fr(486000000, 19)))
    chk.true("F1-SLIDE: state slide+ with beta = -1 (R_n < 0)", s["support:FRICTION"] == "slide+/b-")

    s, r_f1s, d_f1s = add("F1-STICK", "The same with F_z = 25 kN: sticks (R-4 F-1)", model_f(),
                          load_case(P10, {"N1": nodal(fz=25000)}), "R-4 printed reference", CONF_STANDING,
                          f_closed(25000, P10, N10, tipflex_eb))
    chk.eq("F1-STICK: R-4 printed u_z = 0", r_f1s["u"]["N1"][2], ZERO)
    chk.eq("F1-STICK: R-4 printed R_t = -25000 N", r_f1s["R"]["support:FRICTION"], rf(-25000))
    chk.true("F1-STICK: state stick", s["support:FRICTION"] == "stick")

    NE = rf(100000)
    s, r_fe, d_fe = add("F1E-SLIDE", "Explicit-normal twin, N = 100 kN entered, F_z = 50 kN: slides",
                        model_f(("explicit", 100000)), load_case(P10, {"N1": nodal(fz=50000)}),
                        "explicit-normal twin", CONF_STANDING + " Explicit normal: k = 0, Stage B only.",
                        f_closed(50000, P10, NE, tipflex_eb, explicit=True))
    s, r_fes, d_fes = add("F1E-STICK", "Explicit-normal twin, N = 100 kN, F_z = 25 kN: sticks",
                          model_f(("explicit", 100000)), load_case(P10, {"N1": nodal(fz=25000)}),
                          "explicit-normal twin", CONF_STANDING + " Explicit normal: k = 0, Stage B only.",
                          f_closed(25000, P10, NE, tipflex_eb, explicit=True))

    s, r_ft, d_ft = add("F1-T-SLIDE", "F-1 slide with Timoshenko straight members", model_f(shear="timoshenko"),
                        load_case(P10, {"N1": nodal(fz=50000)}), "Timoshenko confirmation twin (R-4 §2.2)",
                        CONF_STANDING, f_closed(50000, P10, N10, tipflex_t))
    chk.eq("F1-T-SLIDE: shear part of u_z = (F_z - mu N) L/(kappa_E G A)", r_ft["u"]["N1"][2] - r_f1["u"]["N1"][2],
           (rf(50000) - rf(9720) * pi) * L3 / GAS)
    chk.eq("F1-T-SLIDE: tip rotation (section rotation) equals Euler-Bernoulli's", r_ft["u"]["N1"][4], r_f1["u"]["N1"][4])
    s, r_fts, d_fts = add("F1-T-STICK", "F-1 stick with Timoshenko straight members", model_f(shear="timoshenko"),
                          load_case(P10, {"N1": nodal(fz=25000)}), "Timoshenko confirmation twin (R-4 §2.2)",
                          CONF_STANDING, f_closed(25000, P10, N10, tipflex_t))

    s, r_f0, d_f0 = add("F1-P0-SLIDELOAD", "Unpressurized control of F-1, F_z = 50 kN: derived normal is 0, friction inactive",
                        model_f(), load_case(P0, {"N1": nodal(fz=50000)}), "pressure isolation control",
                        DERIVED_INACTIVE, f_closed(50000, P0, ZERO, tipflex_eb))
    chk.true("F1-P0-SLIDELOAD: friction inactive", s["support:FRICTION"] == "inactive")
    chk.eq("F1-P0-SLIDELOAD: source reaction exactly 0", r_f0["R"]["support:N1-UX"], ZERO)
    s, r_f0s, d_f0s = add("F1-P0-STICKLOAD", "Unpressurized control of F-1, F_z = 25 kN: friction inactive",
                          model_f(), load_case(P0, {"N1": nodal(fz=25000)}), "pressure isolation control",
                          DERIVED_INACTIVE, f_closed(25000, P0, ZERO, tipflex_eb))
    chk.true("F1-P0-STICKLOAD: friction inactive", s["support:FRICTION"] == "inactive")
    s, r_fe0, d_fe0 = add("F1E-P0-SLIDE", "Unpressurized explicit-normal twin, N = 100 kN, F_z = 50 kN",
                          model_f(("explicit", 100000)), load_case(P0, {"N1": nodal(fz=50000)}),
                          "pressure isolation control", CONF_STANDING + " Explicit normal: k = 0, Stage B only.",
                          f_closed(50000, P0, NE, tipflex_eb, explicit=True))
    s, r_fe0s, d_fe0s = add("F1E-P0-STICK", "Unpressurized explicit-normal twin, N = 100 kN, F_z = 25 kN",
                            model_f(("explicit", 100000)), load_case(P0, {"N1": nodal(fz=25000)}),
                            "pressure isolation control", CONF_STANDING + " Explicit normal: k = 0, Stage B only.",
                            f_closed(25000, P0, NE, tipflex_eb, explicit=True))
    for c in (1, 2, 3, 4, 5):
        chk.eq(f"F1E: pressure does not move the explicit-normal transverse response: u[N1].{DOFS[c]} (slide)",
               r_fe["u"]["N1"][c], r_fe0["u"]["N1"][c])
        chk.eq(f"F1E: pressure does not move the explicit-normal transverse response: u[N1].{DOFS[c]} (stick)",
               r_fes["u"]["N1"][c], r_fe0s["u"]["N1"][c])
    chk.eq("F1: pressure contribution to slide u_z = -mu (1-2nu) p A_i L^3/(3EI)",
           r_f1["u"]["N1"][2] - r_f0["u"]["N1"][2], -rf(9720) * pi * tipflex_eb)

    s, r_fm, d_fm = add("F1-MID", "F-1 with F_z = 37.5 kN (combination illustration)", model_f(),
                        load_case(P10, {"N1": nodal(fz=37500)}), "illustrates NONLINEAR_COMBINATION_REQUIRES_SOLVE",
                        CONF_STANDING, f_closed(37500, P10, N10, tipflex_eb))

    # ---------------- O: one-way lift-off under pressure through a realized bend
    h, R, a, k = rf(O_H), rf(O_R), rf(O_A), rf(O_K)
    f_ss = (a * a * a / rf(3) + k * R * (a * a * PI / rf(2) + rf(2) * a * R + R * R * PI / rf(4)) + h * (a + R) * (a + R)) / EI \
        + (R * PI / rf(4) + h) / EA

    def o_closed(p, W):
        dp = (eps_p(p) * (h + R)) if p != 0 else ZERO

        def f(res, dec):
            q = dec["support:REST"]
            rows = [("free pressure lift at S = eps_p (h + R) (riser + bend chord; K_b-independent)", None, dp, "m"),
                    ("vertical flexibility at S, f_SS (riser, arc with k, run)", None, f_ss, "m/N")]
            free = dp - rf(W) * f_ss
            if sign(free) > 0:
                rows += [("lift-off: u_Z(S) = eps_p (h+R) - W f_SS", res["u"]["S"][2], free, "m"),
                         ("rest support reaction +0", q["reaction"], ZERO, "N")]
            else:
                rows += [("contact: R_S = W - eps_p (h+R)/f_SS", q["reaction"], rf(W) - dp / f_ss, "N"),
                         ("u_Z(S) = 0", res["u"]["S"][2], ZERO, "m")]
            if p != 0:
                rows.append(("N_w at the cap C = p A_i", res["Nw"]["pipe:run2"]["C"], rf(p) * AI, "N"))
            return rows
        return f

    om = model_o()
    s, r_ol, d_ol = add("O1-LIFT", "One-way rest support lifted off by pressure growth through a realized bend, W = 100 N",
                        om, load_case(P10, {"S": nodal(fz=-100)}), "O-1 (designed here)", CONF_STANDING,
                        o_closed(P10, 100))
    chk.true("O1-LIFT: rest support inactive (lifted off)", s["support:REST"] == "inactive")
    s, r_oc, d_oc = add("O1-CONTACT", "The same rest support staying in contact, W = 400 N", om,
                        load_case(P10, {"S": nodal(fz=-400)}), "O-1 contrast (designed here)", CONF_STANDING,
                        o_closed(P10, 400))
    chk.true("O1-CONTACT: rest support active", s["support:REST"] == "active")
    s, r_ov, d_ov = add("O1-VAC", "Vacuum, p = -1 MPa, W = 0: contraction pulls the run onto the rest", om,
                        load_case(PV, {}), "O-1 vacuum contrast", CONF_STANDING, o_closed(PV, 0))
    chk.true("O1-VAC: rest support active", s["support:REST"] == "active")
    s, r_o0l, d_o0l = add("O1-P0-W100", "Unpressurized control of O1-LIFT (p = 0, no regions), W = 100 N", om,
                          load_case(P0, {"S": nodal(fz=-100)}), "pressure isolation control", CONF_STANDING,
                          o_closed(P0, 100))
    chk.eq("O1-P0-W100: R_S = W exactly", r_o0l["R"]["support:REST"], rf(100))
    s, r_o0c, d_o0c = add("O1-P0-W400", "Unpressurized control of O1-CONTACT (p = 0, no regions), W = 400 N", om,
                          load_case(P0, {"S": nodal(fz=-400)}), "pressure isolation control", CONF_STANDING,
                          o_closed(P0, 400))
    chk.eq("O1-P0-W400: R_S = W exactly", r_o0c["R"]["support:REST"], rf(400))
    chk.eq("O1: pressure contribution to the contact reaction = -eps_p(h+R)/f_SS",
           r_oc["R"]["support:REST"] - r_o0c["R"]["support:REST"], -(ep10 * (h + R)) / f_ss)
    chk.eq("O1-VAC: R_S = -(1/10) of the 10 MPa pressure contribution",
           r_ov["R"]["support:REST"], (ep10 * (h + R)) / f_ss / rf(10))
    # the bend's own term: its homothetic chord expansion is the eps_p*R part of the lift
    chk.eq("O1-LIFT: across the bend u_Z(B2) - u_Z(B1) = eps_p R - R theta_Y(B1) + arc elastic part",
           r_ol["u"]["B2"][2] - r_ol["u"]["B1"][2],
           ep10 * R + (r_ol["u"]["B1"][4] * (-R)) + (bend_flex_z(r_ol, om, P10, 100)))

    # ---------------- mutation controls (formulation B)
    mutants = []

    def deviation(ref, mut):
        """Largest |mut - ref| over published scalars, in units of 1e-9 * max(|ref|, group scale)."""
        groups = {"u_t": [], "u_r": [], "F": [], "M": []}
        pairs = []
        for n in ref["u"]:
            for c in range(6):
                pairs.append(("u_t" if c < 3 else "u_r", ref["u"][n][c], mut["u"][n][c]))
        for sid, v in ref["R"].items():
            if isinstance(v, list):
                for c in range(6):
                    pairs.append(("F" if c < 3 else "M", v[c], mut["R"][sid][c]))
            else:
                pairs.append(("F", v, mut["R"][sid]))
        for g, rv_, mv in pairs:
            groups[g].append(abs(approx(rv_)))
        scale = {g: max(v) if v else 0.0 for g, v in groups.items()}
        worst = 0.0
        for g, rv_, mv in pairs:
            d = abs(approx(mv) - approx(rv_))
            tol = 1e-9 * max(abs(approx(rv_)), scale[g])
            if d > 0 and tol == 0:
                return float("inf")
            if d > 0 and d / tol > worst:
                worst = d / tol
        return worst

    def mutant(mid, desc, base_id, ref_states, ref_res, mdl, lc, published_key):
        states, res = solve_b_only(mdl, lc)
        dev = deviation(ref_res, res)
        changed = states != ref_states
        detected = changed or dev > 1e3
        chk.true(f"mutant {mid}: detected (state change or > 1e3 tolerances)", detected)
        unit = "m" if published_key.__name__.startswith("u") else "N"
        mutants.append({"id": mid, "of": base_id, "mutation": desc, "mutant_states": states,
                        "state_changed": changed,
                        "largest_deviation_in_tolerances": ("unbounded: a reference group that is exactly zero moves"
                                                            if dev == float("inf") else f"{dev:.3e}"),
                        "tolerance_rule": "1e-9 * max(|reference|, largest |reference| in its group: translations, rotations, forces, moments)",
                        "key_quantity": published_key.__doc__,
                        "reference_value": num(published_key(ref_res), unit),
                        "mutant_value": num(published_key(res), unit),
                        "detected": detected})
        return states, res

    def u_x_n1(res):
        """u_X at N1"""
        return res["u"]["N1"][0]

    def u_z_n1(res):
        """u_Z at N1"""
        return res["u"]["N1"][2]

    def u_z_s(res):
        """u_Z at S"""
        return res["u"]["S"][2]

    def r_rest(res):
        """rest support reaction at S"""
        return res["R"]["support:REST"]

    mutant("MUT-GAP-SIGN", "G-1 with the gap closing in -X instead of +X", "G1", s_g1, r_g1,
           model_g(-1, Fr(1, 2000)), load_case(P10), u_x_n1)
    m_fs = model_f(("source", "N0", 0))
    mutant("MUT-NORMAL-SOURCE-SLIDE", "F-1 slide with the normal taken from the anchor's UX reaction "
           "(|R| = 2 nu p A_i = 48600 pi N) instead of the N1 UX restraint", "F1-SLIDE",
           {"support:FRICTION": "slide+/b-"}, r_f1, m_fs, load_case(P10, {"N1": nodal(fz=50000)}), u_z_n1)
    st_ns, res_ns = solve_b_only(m_fs, load_case(P10, {"N1": nodal(fz=25000)}))
    chk.true("mutant MUT-NORMAL-SOURCE-STICK: published friction-normal row differs (48600 pi vs 32400 pi)",
             st_ns["support:FRICTION"] == "stick" and
             rabs(res_ns["R"]["support:ANCHOR"][0]) != rabs(r_f1s["R"]["support:N1-UX"]))
    mutants.append({"id": "MUT-NORMAL-SOURCE-STICK", "of": "F1-STICK",
                    "mutation": "F-1 stick with the normal from the anchor's UX reaction",
                    "mutant_states": st_ns, "state_changed": False,
                    "largest_deviation_in_tolerances": "0 in motions and reactions (still sticks with R_t = -25000 N)",
                    "key_quantity": "derived friction-normal row N",
                    "reference_value": num(rabs(r_f1s["R"]["support:N1-UX"]), "N"),
                    "mutant_value": num(rabs(res_ns["R"]["support:ANCHOR"][0]), "N"),
                    "detected": True,
                    "detected_by": "the derived friction-normal row: 48600 pi N against 32400 pi N, and the stick "
                                   "margin 14580 pi - 25000 N against 9720 pi - 25000 N"})
    mutant("MUT-NORMAL-NO-PRESSURE-SLIDE", "F-1 slide with the normal formed without the pressure terms "
           "(N from the unpressurized state = 0, entered as an explicit 0)", "F1-SLIDE",
           {"support:FRICTION": "slide+/b-"}, r_f1, model_f(("explicit", 0)), load_case(P10, {"N1": nodal(fz=50000)}), u_z_n1)
    mutant("MUT-NORMAL-NO-PRESSURE-STICK", "F-1 stick with the normal formed without the pressure terms (N = 0)",
           "F1-STICK", {"support:FRICTION": "stick"}, r_f1s, model_f(("explicit", 0)),
           load_case(P10, {"N1": nodal(fz=25000)}), u_z_n1)
    _, mu_l = mutant("MUT-ARC-UFREE-LIFT", "O1-LIFT with the bend's K_b u_free(eps_p) term omitted (c_b kept)",
                     "O1-LIFT", {"support:REST": "inactive"}, r_ol, om,
                     load_case(P10, {"S": nodal(fz=-100)}, "drop_arc_ufree"), u_z_s)
    chk.eq("MUT-ARC-UFREE-LIFT: the bend's K_b u_free term is exactly eps_p R of the lift at S",
           r_ol["u"]["S"][2] - mu_l["u"]["S"][2], ep10 * R)
    _, mu_c = mutant("MUT-ARC-UFREE-CONTACT", "O1-CONTACT with the bend's K_b u_free(eps_p) term omitted", "O1-CONTACT",
                     {"support:REST": "active"}, r_oc, om, load_case(P10, {"S": nodal(fz=-400)}, "drop_arc_ufree"), r_rest)
    chk.eq("MUT-ARC-UFREE-CONTACT: reaction moves by eps_p R / f_SS",
           mu_c["R"]["support:REST"] - r_oc["R"]["support:REST"], ep10 * R / f_ss)
    mutant("MUT-ARC-ALL-LIFT", "O1-LIFT with the bend's whole pressure term K_b u_free - c_b omitted", "O1-LIFT",
           {"support:REST": "inactive"}, r_ol, om, load_case(P10, {"S": nodal(fz=-100)}, "drop_arc_all"), r_rest)

    # ---------------- combinations
    half = {"u_X(N1)": (r_g1["u"]["N1"][0] / rf(2), r_gh["u"]["N1"][0]),
            "gap reaction": (r_g1["R"]["support:GAP"] / rf(2), r_gh["R"]["support:GAP"])}
    for key, (comb, solved) in half.items():
        chk.true(f"combination illustration: 0.5*G1 != solved G1-HALF ({key})", comb != solved)
    mid = {"u_Z(N1)": ((r_f1["u"]["N1"][2] + r_f1s["u"]["N1"][2]) / rf(2), r_fm["u"]["N1"][2]),
           "friction R_t": ((r_f1["R"]["support:FRICTION"] + r_f1s["R"]["support:FRICTION"]) / rf(2),
                            r_fm["R"]["support:FRICTION"])}
    for key, (comb, solved) in mid.items():
        chk.true(f"combination illustration: 0.5*F1-SLIDE + 0.5*F1-STICK != solved F1-MID ({key})", comb != solved)
    combinations = {
        "rule": ("A mechanics combination of a model with any nonlinear support is withheld by name, "
                 "NONLINEAR_COMBINATION_REQUIRES_SOLVE (pf-nonlinear-operand-standing.md; exact-combinations.md §4, "
                 "to be lifted onto EX in S5 by R-4 §3.5). It is a model condition: the factors, the operands' states and "
                 "the pressure do not matter. result_state_subtraction and range_envelope are formed from the operand "
                 "cases' published rows, as derived results with operand standing, never an equilibrium state."),
        "withheld": [
            "every flat mechanics combination over the G, F or O models, e.g. mechanics(1.0*G1) (singleton), "
            "mechanics(0.5*G1 + 0.5*G1-P0), mechanics(1.0*F1-SLIDE + 1.0*F1-P0-SLIDELOAD), "
            "mechanics(0.3*O1-LIFT + 0.7*O1-CONTACT) (unit factor sum: still withheld)",
            "the same combinations with gap-bearing operands (G models); their operands are also unqualified",
            "absolute-sum/SRSS forms outside OQ-1's admitted shape (absolute-sum-srss-combinations.md V-9)",
        ],
        "formed_not_withheld": ["result_state_subtraction (e.g. G1 - G1-P0), range_envelope over G1, G2 and G1-P0"],
        "why": {
            "0.5*G1 vs solved G1-HALF": {
                "combined_u_X_N1": num(half["u_X(N1)"][0], "m"), "solved_u_X_N1": num(half["u_X(N1)"][1], "m"),
                "combined_gap_reaction": num(half["gap reaction"][0], "N"),
                "solved_gap_reaction": num(half["gap reaction"][1], "N"),
                "solved_state": s_gh["support:GAP"]},
            "0.5*F1-SLIDE + 0.5*F1-STICK vs solved F1-MID (F_z = 37.5 kN)": {
                "combined_u_Z_N1": num(mid["u_Z(N1)"][0], "m"), "solved_u_Z_N1": num(mid["u_Z(N1)"][1], "m"),
                "combined_R_t": num(mid["friction R_t"][0], "N"), "solved_R_t": num(mid["friction R_t"][1], "N")},
        },
    }

    out = {
        "reference_id": "rt_a3_nonlinear_4",
        "status": "frozen analytical candidate; product agreement and independent scrutiny recorded separately",
        "source_revision_read": SOURCE_REVISION,
        "governing_sources": [
            "execution/PKG-04/DEL-04-07/Work/route-unification-r4.md §00, §0a, §2.1-§2.4, §3.5 (S5: G-1, G-2, G-3, F-1, O-1, K-1), §3.8, §6",
            "execution/PKG-04/DEL-04-01/Work/kd5-nonlinear-active-set.md §0, §1.2-§1.4, §2.2, §2.4 (DEC-067 laws, k = 1 Stage A, strict conditions, fail closed)",
            "execution/PKG-04/DEL-04-07/Work/pressure-contract.md 'Accepted T4 direction' H-2; 'T4-U2 as implemented' (arc ledger K_b*u_free(eps_p) - c_b); 'Tees and pressure junctions' (caps)",
            "execution/PKG-05/DEL-05-03/Work/section-bases-and-timoshenko.md §1, §3.1, §3.3, §3.6",
            "validation/references/t4_i7 (arc unit-load flexibility definition: N n/EA + T t/GJ + k (M.m - T t)/EI)",
            "execution/PKG-05/DEL-05-02/Work/exact-combinations.md §4; pf-nonlinear-operand-standing.md; absolute-sum-srss-combinations.md",
        ],
        "independence": "standard library only; no product code, product output, fixture, result schema or generated envelope is imported, read or run; all data invented",
        "model": {
            "theory": "small-displacement linear elastic circular pipe members; nodal loads only; anchors restrain all six DOFs; pressure is a fixed right-hand side inside the active-set problem (R-4 §2.1)",
            "material": {"E_Pa": str(E), "nu": str(NU), "G_Pa": str(G)},
            "section": {"D_m": "1/5", "t_m": "1/100", "c_m_m": "0", "c_a_m": "0", "r_o_m": str(RO), "r_i_m": str(RI),
                        "A_n_m2": num(AN, "m^2"), "I_m4": num(IN, "m^4"), "J_m4": num(JN, "m^4"), "Z_m3": num(ZN, "m^3"),
                        "A_i_m2": num(AI, "m^2"), "kappa_E": str(KAPPA), "A_s_m2": num(ASH, "m^2"),
                        "note": "c_m = c_a = 0: the nominal and reduced bases coincide; every stress value is the value of both basis families"},
            "pressure": "caps p*A_i along the outward axis at transfers_to_wall terminals; none at separately_supported_or_compensated terminals; eps_p = (1-2nu) p A_i/(E A_n); N_w = S + p A_i",
            "realized_arc": "Euler-Bernoulli under either shear model; quarter circle; the user's k multiplies both bending flexibilities; axial and torsional flexibility unmodified; no shear term (T4-I7 definition; section-bases §3.6)",
            "support_laws": "DEC-067 as in kd5-nonlinear-active-set.md §0; r = (K u - f)_d is the support-on-pipe force with f including any cap at that node; states decided with the strict §2.2 conditions",
            "exact_form": "rational functions of pi with rational coefficients; Laurent forms printed as sums of q*pi^k; 'value' is the nearest binary64, decided by rational pi bounds 1e-110 apart",
            "units": "m, rad, N, N*m, Pa",
        },
        "cases": cases,
        "mutation_controls": mutants,
        "combinations": combinations,
        "checks": {"count": len(chk.items), "all_pass": True, "by_kind": _check_kinds(chk.items)},
    }
    return out


def bend_flex_z(res, mdl, p, W):
    """Elastic +Z motion of B2 relative to a clamped B1, from B's closed-form quarter-circle flexibility,
    under the effective load carried from beyond B2 in O1-LIFT (support inactive): the weight -W at S.
    Beyond B2 the pressure acts only as eigenstrain (both terminals transfer to the wall), so no effective
    force crosses B2. Arc basis: e1 = -X, e2 = +Z, n = +Y; F2 = -W; Mn = ((S - B2) x F).n = a W."""
    R, a, k = rf(O_R), rf(O_A), rf(O_K)
    fl = arc_flexibility_local(R, k)
    return fl[1][1] * rf(-W) + fl[1][5] * (a * rf(W))


def _check_kinds(items):
    kinds = {}
    for i in items:
        name = i["check"].split(": ", 1)[-1]
        key = re.sub(r"\[[^\]]*\]|\.(UX|UY|UZ|RX|RY|RZ|FX|FY|FZ|MX|MY|MZ)\b|pipe:\w+|bend:\w+|support:[\w-]+", "*", name)
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
        print(f"{len(out['cases'])} cases, {len(out['mutation_controls'])} mutation controls, "
              f"{out['checks']['count']} exact checks, all pass")
        return
    root = Path(__file__).resolve().parents[1]
    rendered = (json.dumps(out, indent=1, ensure_ascii=False) + "\n").encode("utf-8")
    files = {
        "README.md": (root / "README.md").read_bytes(),
        "generators/rt_a3_nonlinear_reference.py": Path(__file__).read_bytes(),
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
