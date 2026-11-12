#!/usr/bin/env python3
"""Candidate (NOT adopted, NOT frozen) RT-A3 reducer 2b reference package generator.

The cleared 3-D frame reference (26 cases, K and FM formulations, 24 W-15c geometry controls,
eight mutation controls) is carried unchanged in the first part of this file. The PACKAGE LAYER
at the end adds j-side section actions (K member statics and FM statics), reference-section stress
rows on both D-5 bases, endpoint maxima proved by convexity, rigorous hypot enclosures, physically
dimensioned family zero scales for the 1e-9 criterion, tolerance sensitivity of the cleared mutants
and one junction-held C3 analogue. See ../README.md.

Usage: python3 -I -B generators/rt_a3_2b_reference.py [--check | --summary | --write]
(--check is the default: it recomputes, compares bytes and SHA256SUMS, and writes nothing.)

Original header of the cleared frame reference follows.

Independent numerical reference generator for the bounded eccentric reducer
(increment 2b) mechanics. Standard library only. It imports, reads and runs no
product code, product output or frozen product reference. Governing text was
read only to fix the declared model (selected source 74e57d55ffd4, branch
codex/rt-a3-reducer-2a; integrated baseline 2656dcb33):
  - DEL-03-05 (nodes govern; chord body
    member; W-15c run-axis agreement/uncertainty/refusal method)
  - DEL-03-05 (s*K; the Poisson
    pair has no E; N_w = N_m + 2 nu p A_i; reference-section rows)
  - DEL-04-07 D-1 and "Concentric reducer
    pressure-bore step"; DEL-04-07 (caps -p A_i s at
    transfers_to_wall terminals; Poisson pair; N_w and S recovery)
  - DEL-05-03 (D-5 bases;
    sigma_m = N_w/A_b, Lame per basis annulus; kappa_E; phi)

Model (small displacement, linear elastic straight 3-D frame members):
  * stiffness s*E, s*G on the NOMINAL section (A_n, I, J = 2I, A_s = kappa A_n);
  * pressure loads on the pressure bore A_i = pi r_i,r^2: caps -p A_i s_into at
    each transfers_to_wall terminal (local x: [-pA_i, +pA_i]), and the Poisson
    eigen pair +2 nu p A_i at `from`, -2 nu p A_i at `to` along member local x
    (unscaled: it carries no E);
  * constitutive s E A_n eps = N_w - 2 nu p A_i; S = N_w - p A_i;
  * every node at its actual position, every cap along its member's actual
    direction and at its actual node; no averaged axis, no projection to a
    declared offset, no added p*A*e term.

Two formulations, sharing only inputs and the section formulas:
  K  an assembled 3-D frame stiffness (12x12 local EB/Timoshenko, local axes
     from the chord and y_reference, T^T k T), held-end Poisson actions, caps
     as nodal loads, exact elimination; end actions k T u - f_eq, R = K u - f.
  FM a force method on the released cantilever (anchor A held, far node C
     released): physical loads only (caps, mechanical), the Poisson effect as
     the axial eigenstrain eps0 = -2 nu p A_i/(s E A_n), section actions by
     statics, unit-load virtual work with the circular-section compliance
     density N n/EA + V.v/GA_s + T t/GJ + M.m/EI (no local frame used), six
     redundant reactions at C from exact compatibility.
All arithmetic is exact: every number is (element of a multi-quadratic field
Q(sqrt b_1, ..., sqrt b_n)) * pi^k. Binary64 emission is decided only when
both ends of a rational enclosure round identically. The mathematical object is
the exact model with the declared decimal inputs, not the system assembled
from binary64 leaves by the product. Product binding needs the governed
scaled tolerance, not bit equality. Decimal strings are informational
midpoint displays; only the binary64 fields have a rounding decision test.

(Original usage line retired; see the package usage above.)
"""
from fractions import Fraction as Fr
from decimal import Decimal, getcontext
from math import isqrt
import argparse
import hashlib
import json
import sys

getcontext().prec = 90
SOURCE_REVISION = "74e57d55ffd4cb09b68680e36bcaca06f4ad81eb"
BASELINE = "2656dcb33"

# ============================================================================
# Exact pi enclosure (Machin, alternating-series remainders)


def _atan_inv_bounds(n, digits=110):
    x = Fr(1, n)
    power, total, k = x, Fr(0), 0
    while True:
        total += power / (2 * k + 1)
        nxt = -power * x * x
        term = nxt / (2 * k + 3)
        if abs(term) < Fr(1, 10 ** digits):
            return min(total, total + term), max(total, total + term)
        power, k = nxt, k + 1


_a5, _a239 = _atan_inv_bounds(5), _atan_inv_bounds(239)
PI_LO = 16 * _a5[0] - 4 * _a239[1]
PI_HI = 16 * _a5[1] - 4 * _a239[0]
assert Fr(3) < PI_LO < PI_HI < Fr(22, 7) and PI_HI - PI_LO < Fr(1, 10 ** 100)

# ============================================================================
# Multi-quadratic field Q(sqrt b_1..sqrt b_n), b_i squarefree, independent mod squares


def factor_int(n):
    """prime -> exponent (trial division; inputs here are small)."""
    out, d = {}, 2
    while d * d <= n:
        while n % d == 0:
            out[d] = out.get(d, 0) + 1
            n //= d
        d += 1 if d == 2 else 2
    if n > 1:
        out[n] = out.get(n, 0) + 1
    return out


def sqrt_parts(q):
    """q > 0 rational -> (rational c, frozenset primes P) with sqrt(q) = c*sqrt(prod P)."""
    q = Fr(q)
    assert q > 0
    c, primes = Fr(1), set()
    for n, sign in ((q.numerator, 1), (q.denominator, -1)):
        for p, e in factor_int(n).items():
            c *= Fr(p) ** (sign * (e // 2))
            if e % 2:
                primes ^= {p}
                if sign < 0:
                    c /= p  # 1/sqrt(p) = sqrt(p)/p
    return c, frozenset(primes)


class Field:
    def __init__(self, radicand_rationals):
        basis = []  # fully reduced GF(2) basis: list of [pivot, set]
        for q in radicand_rationals:
            _, s = sqrt_parts(q)
            s = set(s)
            for piv, bs in basis:
                if piv in s:
                    s ^= bs
            if s:
                piv = max(s)
                for item in basis:
                    if piv in item[1]:
                        item[1] ^= s
                basis.append([piv, set(s)])
        self.basis = [(piv, frozenset(s)) for piv, s in basis]
        self.b = []
        for _, s in self.basis:
            v = 1
            for p in s:
                v *= p
            self.b.append(v)
        self.n = 1 << len(self.b)
        self.fac = []
        for m in range(self.n):
            v = 1
            for i, bi in enumerate(self.b):
                if m >> i & 1:
                    v *= bi
            self.fac.append(v)
        self.parity = [[bin(i & m).count("1") & 1 for i in range(self.n)] for m in range(self.n)]
        self._zero = QF(self, [Fr(0)] * self.n)
        self._one = self.r(1)

    def r(self, x):
        c = [Fr(0)] * self.n
        c[0] = Fr(x)
        return QF(self, c)

    def zero(self):
        return self._zero

    def sqrt(self, q):
        """sqrt of a positive rational as a field element."""
        c, primes = sqrt_parts(q)
        s, mask = set(primes), 0
        for i, (piv, bs) in enumerate(self.basis):
            if piv in s:
                s ^= bs
                mask |= 1 << i
        if s:
            raise ValueError(f"sqrt({q}) outside the declared field")
        # sqrt(prod primes) * prod sqrt(b_i) = integer square
        prod_p = 1
        for p in primes:
            prod_p *= p
        sq = prod_p * self.fac[mask]
        rt = isqrt(sq)
        assert rt * rt == sq
        out = [Fr(0)] * self.n
        out[mask] = c * Fr(rt, self.fac[mask])
        return QF(self, out)

    def names(self):
        out = []
        for m in range(self.n):
            out.append("1" if m == 0 else "*".join(f"sqrt({self.b[i]})" for i in range(len(self.b)) if m >> i & 1))
        return out


class QF:
    __slots__ = ("F", "c")

    def __init__(self, F, c):
        self.F, self.c = F, c

    def _co(self, o):
        return o if isinstance(o, QF) else self.F.r(o)

    def __add__(self, o):
        o = self._co(o)
        return QF(self.F, [a + b for a, b in zip(self.c, o.c)])

    __radd__ = __add__

    def __sub__(self, o):
        o = self._co(o)
        return QF(self.F, [a - b for a, b in zip(self.c, o.c)])

    def __rsub__(self, o):
        return self._co(o) - self

    def __neg__(self):
        return QF(self.F, [-a for a in self.c])

    def __mul__(self, o):
        if not isinstance(o, QF):
            o = Fr(o)
            return QF(self.F, [a * o for a in self.c])
        out = [Fr(0)] * self.F.n
        fac = self.F.fac
        for i, a in enumerate(self.c):
            if a:
                for j, b in enumerate(o.c):
                    if b:
                        out[i ^ j] += a * b * fac[i & j]
        return QF(self.F, out)

    __rmul__ = __mul__

    def conj(self, m):
        par = self.F.parity[m]
        return QF(self.F, [-a if par[i] else a for i, a in enumerate(self.c)])

    def inv(self):
        if self.is_zero():
            raise ZeroDivisionError
        if all(a == 0 for a in self.c[1:]):
            return self.F.r(1 / self.c[0])
        prod = self.F.r(1)
        for m in range(1, self.F.n):
            prod = prod * self.conj(m)
        norm = self * prod
        assert all(a == 0 for a in norm.c[1:])
        return prod * (1 / norm.c[0])

    def __truediv__(self, o):
        if not isinstance(o, QF):
            return self * (1 / Fr(o))
        return self * o.inv()

    def is_zero(self):
        return not any(self.c)

    def __eq__(self, o):
        return (self - self._co(o)).is_zero()

    def __hash__(self):
        return hash(tuple(self.c))

    def is_rational(self):
        return not any(self.c[1:])

    def exact(self):
        terms = [(a, nm) for a, nm in zip(self.c, self.F.names()) if a]
        if not terms:
            return "0"
        return " + ".join(f"{a}" if nm == "1" else f"({a})*{nm}" for a, nm in terms)

    def interval(self, digits=70):
        lo = hi = Fr(0)
        scale = 10 ** digits
        sb = []
        for b in self.F.b:
            n = isqrt(b * scale * scale)
            exact = n * n == b * scale * scale
            sb.append((Fr(n, scale), Fr(n if exact else n + 1, scale)))
        for m, a in enumerate(self.c):
            if not a:
                continue
            mlo = mhi = Fr(1)
            for i in range(len(self.F.b)):
                if m >> i & 1:
                    mlo, mhi = mlo * sb[i][0], mhi * sb[i][1]
            if a > 0:
                lo, hi = lo + a * mlo, hi + a * mhi
            else:
                lo, hi = lo + a * mhi, hi + a * mlo
        return lo, hi


def emit(x, k):
    """x * pi^k -> exact text, informational 40-digit midpoint display, decided binary64 (or null)."""
    lo, hi = x.interval()
    if k:
        plo, phi_ = (PI_LO ** k, PI_HI ** k) if k > 0 else (PI_HI ** k, PI_LO ** k)
        cands = [lo * plo, lo * phi_, hi * plo, hi * phi_]
        lo, hi = min(cands), max(cands)
    mid = (lo + hi) / 2
    dec = Decimal(mid.numerator) / Decimal(mid.denominator)
    f_lo, f_hi = float(lo), float(hi)
    exact = x.exact()
    if exact != "0" and k:
        exact = f"({exact})*" + ("pi" if k == 1 else f"pi^{k}")
    if x.is_zero():
        return {"exact": "0", "decimal": "0", "binary64": 0.0}
    return {"exact": exact, "decimal": f"{dec:.40g}", "binary64": f_lo if f_lo == f_hi else None}


# ============================================================================
# Vector helpers (QF or rational components)


def vadd(a, b):
    return [x + y for x, y in zip(a, b)]


def vsub(a, b):
    return [x - y for x, y in zip(a, b)]


def vsc(a, s):
    return [x * s for x in a]


def dot(a, b):
    t = a[0] * b[0]
    for x, y in zip(a[1:], b[1:]):
        t = t + x * y
    return t


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def qv(F, v):
    return [x if isinstance(x, QF) else F.r(x) for x in v]


def eqv(a, b):
    return all((x - y).is_zero() for x, y in zip(a, b))


# ============================================================================
# Material and D-5 sections (exact). Areas/inertias are coefficients of pi.

E = Fr(200_000_000_000)
NU = Fr(3, 10)
G = E / (2 * (1 + NU))


def section(name, D, t, cm, ca):
    D, t, cm, ca = Fr(D), Fr(t), Fr(cm), Fr(ca)
    ro = D / 2
    ri_n = ro - t
    t_r = (t - cm) - ca
    ri_r = ro - t_r
    m2 = (ri_n / ro) ** 2
    a, b, n1 = 1 + m2, 1 - m2, 1 + NU
    c = (8 * NU + 14) * NU + 7
    Dk = c * (b * b) + 48 * (m2 * (n1 * n1))
    kappa = (6 * (n1 * n1)) * (a * a) / Dk
    I = (ro ** 4 - ri_n ** 4) / 4
    return {"name": name, "D": D, "t": t, "cm": cm, "ca": ca, "ro": ro, "ri_n": ri_n, "t_r": t_r, "ri_r": ri_r,
            "An": ro * ro - ri_n * ri_n, "Ar": ro * ro - ri_r * ri_r, "Ai": ri_r * ri_r, "Ai_nominal": ri_n * ri_n,
            "I": I, "J": 2 * I, "kappa": kappa}


def mm(x):
    return Fr(str(x)) / 1000


SEC = {
    "S270": section("S270", mm(270), mm(9), mm("1.0"), mm("1.5")),
    "S220": section("S220", mm(220), mm(8), mm("0.9"), mm("1.5")),
}
KAPPA_COWPER_MUTANT = Fr(5, 6)

# ============================================================================
# Case construction


def P3(x, y, z):
    return (Fr(str(x)), Fr(str(y)), Fr(str(z)))


RS = {"RS-140": P3(6, 18, 7), "RS-150": P3(6, 20, 7), "RS-160": P3(6, "20.4", "6.975"), "RS-170": P3(6, 23, "6.975")}
P_OP = Fr(1_600_000)
YREF = (Fr(0), Fr(0), Fr(1))
MECH = {  # invented 6-component nodal perturbation (N, N m), pi^0
    "N": ((Fr(1500), Fr(-2500), Fr(4000)), (Fr(-1200), Fr(800), Fr(600))),
    "F": ((Fr(-700), Fr(900), Fr(-3000)), (Fr(500), Fr(-1500), Fr(2500))),
}


def rs_case(label, purpose, *, shear, scale, anchors=("A", "C"), p=(P_OP, P_OP, P_OP), comp_C=False,
            body_reversed=False, mech=False, nodes=None, yrefs=None, mut=None, control=False):
    """Four-node RS-R1 neighbourhood control. Chain A=RS-170, N=RS-160, F=RS-150, C=RS-140.
    nbrN = RS-P160 (RS-160 -> RS-170, S220, 2.6 m); body = RS-P150 (RS-150 -> RS-160, S270, mapped,
    scale s); nbrF = RS-P140 (RS-140 -> RS-150, S270, 2 m). Each member is treated as its own
    pressure region with both terminals transfers_to_wall (a control, not the full RS fixture)."""
    nodes = dict(nodes or {"A": RS["RS-170"], "N": RS["RS-160"], "F": RS["RS-150"], "C": RS["RS-140"]})
    yrefs = yrefs or {"nbrN": YREF, "body": YREF, "nbrF": YREF}
    pN, pB, pF = p
    bf, bt = ("F", "N") if not body_reversed else ("N", "F")
    members = [
        {"id": "nbrN:RS-P160", "key": "nbrN", "from": "N", "to": "A", "sec": "S220", "p": pN, "scale": Fr(1), "yref": yrefs["nbrN"], "role": "pipe"},
        {"id": "body:RS-P150", "key": "body", "from": bf, "to": bt, "sec": "S270", "p": pB, "scale": Fr(scale), "yref": yrefs["body"], "role": "reducer_body"},
        {"id": "nbrF:RS-P140", "key": "nbrF", "from": "C", "to": "F", "sec": "S270", "p": pF, "scale": Fr(1), "yref": yrefs["nbrF"], "role": "pipe"},
    ]
    caps = set()
    for m in members:
        for nd in (m["from"], m["to"]):
            if m["p"] != 0 and not (comp_C and nd == "C"):
                caps.add((m["key"], nd))
    loads = {}
    if mech:
        loads = {k: v for k, v in MECH.items()}
    power = 0 if mech else 1
    assert not (mech and any(p))
    return {"label": label, "purpose": purpose, "nodes": nodes, "chain": ["A", "N", "F", "C"], "members": members,
            "caps": caps, "anchors": list(anchors), "mech": loads, "shear": shear, "power": power,
            "mut": mut or {}, "control": control}


def rat_rotation(w, x, y, z):
    n = w * w + x * x + y * y + z * z
    R = [[w * w + x * x - y * y - z * z, 2 * (x * y - w * z), 2 * (x * z + w * y)],
         [2 * (x * y + w * z), w * w - x * x + y * y - z * z, 2 * (y * z - w * x)],
         [2 * (x * z - w * y), 2 * (y * z + w * x), w * w - x * x - y * y + z * z]]
    return [[Fr(v, n) for v in row] for row in R]


def mat_vec(R, v):
    return tuple(sum((R[i][j] * v[j] for j in range(3)), Fr(0)) for i in range(3))


ROT = rat_rotation(3, 1, 2, 5)
SHIFT = (Fr("1234.5"), Fr("-678.25"), Fr("4321.125"))


def rotated(c, label, purpose):
    c = dict(c)
    c["label"], c["purpose"] = label, purpose
    c["nodes"] = {k: tuple(a + b for a, b in zip(mat_vec(ROT, v), SHIFT)) for k, v in c["nodes"].items()}
    c["members"] = [dict(m, yref=mat_vec(ROT, m["yref"])) for m in c["members"]]
    c["mech"] = {k: (mat_vec(ROT, f), mat_vec(ROT, mo)) for k, (f, mo) in c["mech"].items()}
    c["rotation_of"] = True
    return c


NONPAR_A = P3("6.003", 23, "6.976")  # RS-170 moved +3 mm x, +1 mm z: W-15c-accepted out-of-plane tilt


def build_cases():
    cs = []
    for shear in ("eb", "timo"):
        for s in (1, 2):
            cs.append(rs_case(f"RS2B-HELD-{shear.upper()}-S{s}", "far nodes A=RS-170 and C=RS-140 fully anchored, N and F free; 1.6 MPa in all three members (indeterminate to degree 6)", shear=shear, scale=s))
    cs.append(rs_case("RS2B-HELD-EB-S4", "as RS2B-HELD-EB-S2 with body scale 4 (shows indeterminate scale response is not affine in 1/s)", shear="eb", scale=4))
    for shear in ("eb", "timo"):
        for s in (1, 2, 4):
            cs.append(rs_case(f"RS2B-DET-FREE-{shear.upper()}-S{s}", "determinate: A anchored; N, F, C free; far end C free and capped (transfers_to_wall)", shear=shear, scale=s, anchors=("A",)))
        for s in (1, 2):
            cs.append(rs_case(f"RS2B-DET-COMP-{shear.upper()}-S{s}", "determinate: A anchored; C free with its terminal separately supported/compensated (cap at C omitted): carries the eccentric step wrench to A", shear=shear, scale=s, anchors=("A",), comp_C=True))
        cs.append(rs_case(f"RS2B-HELD-REV-{shear.upper()}-S2", "RS2B-HELD with the body member reversed (from RS-160 to RS-150)", shear=shear, scale=2, body_reversed=True))
        for s in (1, 2):
            cs.append(rs_case(f"RS2B-MECH-{shear.upper()}-S{s}", "far nodes anchored, zero pressure; invented 6-component force/moment at N and F (all axes, torsion, out-of-plane)", shear=shear, scale=s, p=(0, 0, 0), mech=True))
        cs.append(rs_case(f"RS2B-NONPAR-HELD-{shear.upper()}-S2", "far nodes anchored, 1.6 MPa; RS-170 moved to (6.003, 23, 6.976): nbrN tilted 1.216e-3 rad (W-15c accepted); caps along actual member directions", shear=shear, scale=2, nodes={"A": NONPAR_A, "N": RS["RS-160"], "F": RS["RS-150"], "C": RS["RS-140"]}))
    cs.append(rs_case("RS2B-HELD-UNEQ-TIMO-S2", "far nodes anchored; D-1 warned closure: p 1.6 (nbrN), 2.0 (body), 1.2 (nbrF) MPa; caps' actual resultant, not a hydrostatic gradient", shear="timo", scale=2, p=(Fr(1_600_000), Fr(2_000_000), Fr(1_200_000))))
    base_h = next(c for c in cs if c["label"] == "RS2B-HELD-TIMO-S2")
    base_m = next(c for c in cs if c["label"] == "RS2B-MECH-TIMO-S2")
    cs.append(rotated(base_h, "RS2B-ROT-HELD-TIMO-S2", "RS2B-HELD-TIMO-S2 under the rational rotation q=(3,1,2,5)/sqrt(39) and translation (1234.5, -678.25, 4321.125) m"))
    cs.append(rotated(base_m, "RS2B-ROT-MECH-TIMO-S2", "RS2B-MECH-TIMO-S2 under the same rotation and translation (loads rotated)"))
    return cs


# ============================================================================
# Shared geometry (input-convention facts)


def case_field(c):
    rads = []
    for m in c["members"]:
        xi, xj = c["nodes"][m["from"]], c["nodes"][m["to"]]
        d = [b - a for a, b in zip(xi, xj)]
        L2 = sum(x * x for x in d)
        rads.append(L2)
        y = m["yref"]
        yd = sum(a * b for a, b in zip(y, d))
        ny2 = sum(a * a for a in y) - yd * yd / L2
        rads.append(ny2)
    return Field(rads)


def member_geom(F, c, m):
    xi, xj = c["nodes"][m["from"]], c["nodes"][m["to"]]
    d = [b - a for a, b in zip(xi, xj)]
    L2 = sum(x * x for x in d)
    L = F.sqrt(L2)
    Linv = L.inv()
    x = [F.r(di) * Linv for di in d]
    y_ref = m["yref"]
    yd = sum(a * b for a, b in zip(y_ref, d))
    ny = F.sqrt(sum(a * a for a in y_ref) - yd * yd / L2)
    proj = dot(qv(F, y_ref), x)
    yc = [F.r(a) - proj * xk for a, xk in zip(y_ref, x)]
    y = [v / ny for v in yc]
    z = cross(x, y)
    for u, v, w in ((x, x, 1), (y, y, 1), (z, z, 1), (x, y, 0), (x, z, 0), (y, z, 0)):
        assert dot(u, v) == w
    return {"d": d, "L2": L2, "L": L, "x": x, "y": y, "z": z}


def member_props(m, shear, kappa_override=None):
    s = SEC[m["sec"]]
    sc = m["scale"]
    kap = kappa_override if kappa_override is not None else s["kappa"]
    return {"EA": sc * E * s["An"], "EI": sc * E * s["I"], "GJ": sc * G * s["J"],
            "GAs": (sc * G * kap * s["An"]) if shear == "timo" else None}


def pai(m, nominal_bore=False):
    s = SEC[m["sec"]]
    return m["p"] * (s["Ai_nominal"] if nominal_bore else s["Ai"])


# ============================================================================
# Formulation K: assembled 3-D frame stiffness


def local_k(F, pr, g):
    L, L2 = g["L"], g["L2"]
    Li = L.inv()
    k = [[F.zero() for _ in range(12)] for _ in range(12)]

    def put(i, j, v):
        k[i][j] = k[i][j] + v
        if i != j:
            k[j][i] = k[j][i] + v

    ea = Li * pr["EA"]
    put(0, 0, ea); put(6, 6, ea); put(0, 6, -ea)
    gj = Li * pr["GJ"]
    put(3, 3, gj); put(9, 9, gj); put(3, 9, -gj)
    EI = pr["EI"]
    phi = Fr(0) if pr["GAs"] is None else 12 * EI / (pr["GAs"] * L2)
    a = Li * Li * Li * (12 * EI / (1 + phi))
    b = Li * Li * (6 * EI / (1 + phi))
    cc = Li * ((4 + phi) * EI / (1 + phi))
    dd = Li * ((2 - phi) * EI / (1 + phi))
    # x-y plane (v = u_y, theta = r_z): 1, 5, 7, 11
    put(1, 1, a); put(1, 5, b); put(1, 7, -a); put(1, 11, b)
    put(5, 5, cc); put(5, 7, -b); put(5, 11, dd)
    put(7, 7, a); put(7, 11, -b); put(11, 11, cc)
    # x-z plane (w = u_z, theta = -w'): 2, 4, 8, 10
    put(2, 2, a); put(2, 4, -b); put(2, 8, -a); put(2, 10, -b)
    put(4, 4, cc); put(4, 8, b); put(4, 10, dd)
    put(8, 8, a); put(8, 10, b); put(10, 10, cc)
    return k


def lam(g):
    return [g["x"], g["y"], g["z"]]


def to_local(Lm, v):
    return [dot(row, v) for row in Lm]


def to_global(Lm, v):
    return [Lm[0][i] * v[0] + Lm[1][i] * v[1] + Lm[2][i] * v[2] for i in range(3)]


def solve(F, A, b):
    n = len(b)
    M = [row[:] + [b[i]] for i, row in enumerate(A)]
    for col in range(n):
        piv = next(r for r in range(col, n) if not M[r][col].is_zero())
        M[col], M[piv] = M[piv], M[col]
        inv = M[col][col].inv()
        M[col] = [x * inv if not x.is_zero() else x for x in M[col]]
        for r in range(n):
            if r != col and not M[r][col].is_zero():
                fac = M[r][col]
                M[r] = [x - fac * y if not y.is_zero() else x for x, y in zip(M[r], M[col])]
    return [M[i][n] for i in range(n)]


def stiffness_formulation(F, c):
    mut = c["mut"]
    names = c["chain"]
    idx = {nm: i for i, nm in enumerate(names)}
    ndof = 6 * len(names)
    K = [[F.zero() for _ in range(ndof)] for _ in range(ndof)]
    f = [F.zero() for _ in range(ndof)]
    geo, kloc, feq = {}, {}, {}
    for m in c["members"]:
        g = member_geom(F, c, m)
        geo[m["key"]] = g
        pr = member_props(m, c["shear"], mut.get("kappa"))
        k = local_k(F, pr, g)
        kloc[m["key"]] = k
        Lm = lam(g)
        # T^T k T, block-wise
        dofs = [6 * idx[m["from"]] + i for i in range(6)] + [6 * idx[m["to"]] + i for i in range(6)]
        T = [[F.zero() for _ in range(12)] for _ in range(12)]
        for blk in range(4):
            for i in range(3):
                for j in range(3):
                    T[3 * blk + i][3 * blk + j] = Lm[i][j]
        kT = [[sum((k[i][q] * T[q][j] for q in range(12) if not k[i][q].is_zero() and not T[q][j].is_zero()), F.zero()) for j in range(12)] for i in range(12)]
        for i in range(12):
            for j in range(12):
                v = sum((T[q][i] * kT[q][j] for q in range(12) if not T[q][i].is_zero() and not kT[q][j].is_zero()), F.zero())
                if not v.is_zero():
                    K[dofs[i]][dofs[j]] = K[dofs[i]][dofs[j]] + v
        # Poisson eigen pair (held end actions, local x)
        n0 = 2 * NU * pai(m, mut.get("nominal_bore", False))
        if mut.get("poisson_scaled"):
            n0 = n0 * m["scale"]
        fe = [F.zero() for _ in range(12)]
        fe[0], fe[6] = F.r(n0), F.r(-n0)
        feq[m["key"]] = fe
        for blk, base in ((0, 6 * idx[m["from"]]), (6, 6 * idx[m["to"]])):
            gv = to_global(Lm, fe[blk:blk + 3])
            for i in range(3):
                f[base + i] = f[base + i] + gv[i]
        # caps
        for nd in (m["from"], m["to"]):
            if (m["key"], nd) not in c["caps"] or (m["key"], nd) in mut.get("drop_caps", ()):
                continue
            s_into = g["x"] if nd == m["from"] else vsc(g["x"], -1)
            if (m["key"], nd) in mut.get("cap_dir", {}):
                s_into = qv(F, mut["cap_dir"][(m["key"], nd)])
            cap = vsc(s_into, -pai(m, mut.get("nominal_bore", False)))
            base = 6 * idx[nd]
            for i in range(3):
                f[base + i] = f[base + i] + cap[i]
            if (m["key"], nd) in mut.get("cap_point", {}):
                arm = [F.r(a - b) for a, b in zip(mut["cap_point"][(m["key"], nd)], c["nodes"][nd])]
                mo = cross(arm, cap)
                for i in range(3):
                    f[base + 3 + i] = f[base + 3 + i] + mo[i]
    for nd, (fv, mv) in c["mech"].items():
        for i in range(3):
            f[6 * idx[nd] + i] = f[6 * idx[nd] + i] + fv[i]
            f[6 * idx[nd] + 3 + i] = f[6 * idx[nd] + 3 + i] + mv[i]
    for nd, mv in mut.get("extra_moment", {}).items():
        for i in range(3):
            f[6 * idx[nd] + 3 + i] = f[6 * idx[nd] + 3 + i] + mv[i]
    fixed = {6 * idx[a] + i for a in c["anchors"] for i in range(6)}
    free = [i for i in range(ndof) if i not in fixed]
    sol = solve(F, [[K[r][q] for q in free] for r in free], [f[r] for r in free])
    U = [F.zero() for _ in range(ndof)]
    for i, v in zip(free, sol):
        U[i] = v
    Ku = [sum((K[r][q] * U[q] for q in range(ndof) if not K[r][q].is_zero() and not U[q].is_zero()), F.zero()) for r in range(ndof)]
    resid = [Ku[r] - f[r] for r in range(ndof)]
    res = {"u": {nm: U[6 * idx[nm]:6 * idx[nm] + 6] for nm in names},
           "R": {a: resid[6 * idx[a]:6 * idx[a] + 6] for a in c["anchors"]},
           "free_residual_zero": all(resid[i].is_zero() for i in free), "members": {}, "geo": geo}
    for m in c["members"]:
        g = geo[m["key"]]
        Lm = lam(g)
        ue = res["u"][m["from"]] + res["u"][m["to"]]
        ul = []
        for blk in range(4):
            ul += to_local(Lm, ue[3 * blk:3 * blk + 3])
        k = kloc[m["key"]]
        fl = [sum((k[i][q] * ul[q] for q in range(12) if not k[i][q].is_zero()), F.zero()) - feq[m["key"]][i] for i in range(12)]
        fg = []
        for blk in range(4):
            fg += to_global(Lm, fl[3 * blk:3 * blk + 3])
        res["members"][m["key"]] = {"local": fl, "global": fg, "N_m": fl[6] + feq[m["key"]][6], "N_w": fl[6]}
    return res


# ============================================================================
# Formulation FM: force method / unit-load virtual work on the released cantilever


def force_method(F, c):
    names = c["chain"]
    pos = {nm: qv(F, c["nodes"][nm]) for nm in names}
    cidx = {nm: i for i, nm in enumerate(names)}
    assert c["anchors"][0] == names[0] and set(c["anchors"]) <= {names[0], names[-1]}
    segs = []
    for k in range(len(names) - 1):
        a_, b_ = names[k], names[k + 1]
        (m,) = [m for m in c["members"] if {m["from"], m["to"]} == {a_, b_}]
        d = [y - x for x, y in zip(c["nodes"][a_], c["nodes"][b_])]
        L2 = sum(v * v for v in d)
        L = F.sqrt(L2)
        xo = [F.r(v) / L for v in d]  # from the A-side end to the C-side end
        pr = member_props(m, c["shear"])
        eps0 = -2 * NU * pai(m) / pr["EA"] if m["p"] else Fr(0)
        segs.append({"k": k, "m": m, "A": a_, "B": b_, "L": L, "xo": xo, "pr": pr, "eps0": eps0})
    phys = []  # (node, f, mom): physical loads only
    for m in c["members"]:
        for nd in (m["from"], m["to"]):
            if (m["key"], nd) in c["caps"]:
                other = m["to"] if nd == m["from"] else m["from"]
                d = [y - x for x, y in zip(c["nodes"][nd], c["nodes"][other])]
                L = F.sqrt(sum(v * v for v in d))
                s_into = [F.r(v) / L for v in d]
                phys.append((nd, vsc(s_into, -pai(m)), [F.zero()] * 3))
    for nd, (fv, mv) in c["mech"].items():
        phys.append((nd, qv(F, fv), qv(F, mv)))

    def W(loads, seg, P):
        Fs, Ms = [F.zero()] * 3, [F.zero()] * 3
        for nd, fv, mv in loads:
            if cidx[nd] >= seg["k"] + 1:
                Fs = vadd(Fs, fv)
                Ms = vadd(Ms, vadd(cross(vsub(pos[nd], P), fv), mv))
        return Fs, Ms

    def dens(seg, a, b):
        (Fa, Ma), (Fb, Mb) = a, b
        x, pr = seg["xo"], seg["pr"]
        na, nb = dot(Fa, x), dot(Fb, x)
        ta, tb = dot(Ma, x), dot(Mb, x)
        v = na * nb / pr["EA"] + ta * tb / pr["GJ"] + (dot(Ma, Mb) - ta * tb) / pr["EI"]
        if pr["GAs"] is not None:
            v = v + (dot(Fa, Fb) - na * nb) / pr["GAs"]
        return v

    def integral(seg, la, lb, eig):
        P0, P1 = pos[seg["A"]], pos[seg["B"]]
        Pm = vsc(vadd(P0, P1), Fr(1, 2))
        vals = [dens(seg, W(la, seg, P), W(lb, seg, P)) for P in (P0, Pm, P1)]
        out = seg["L"] * (vals[0] + 4 * vals[1] + vals[2]) / 6
        if eig and seg["eps0"]:
            out = out + seg["L"] * seg["eps0"] * dot(W(lb, seg, P0)[0], seg["xo"])
        return out

    def unit(nd, i):
        e = [F.zero()] * 3
        e[i % 3] = F.r(1)
        return [(nd, e, [F.zero()] * 3)] if i < 3 else [(nd, [F.zero()] * 3, e)]

    C = names[-1]
    loads = list(phys)
    X = None
    if C in c["anchors"]:
        units = [unit(C, i) for i in range(6)]
        Fm = [[sum((integral(sg, units[i], units[j], False) for sg in segs), F.zero()) for j in range(6)] for i in range(6)]
        d0 = [sum((integral(sg, phys, units[i], True) for sg in segs), F.zero()) for i in range(6)]
        X = solve(F, Fm, [-v for v in d0])
        loads.append((C, X[:3], X[3:]))
    res = {"u": {}, "R": {}, "members": {}, "segs": segs, "loads": loads, "phys": phys, "W": W}
    for nd in names:
        if nd in c["anchors"]:
            res["u"][nd] = [F.zero()] * 6
            if nd == C:
                comp = [sum((integral(sg, loads, unit(C, i), True) for sg in segs), F.zero()) for i in range(6)]
                res["compat_residual_zero"] = all(v.is_zero() for v in comp)
            continue
        res["u"][nd] = [sum((integral(sg, loads, unit(nd, i), True) for sg in segs if cidx[nd] >= sg["k"] + 1), F.zero()) for i in range(6)]
    A0 = names[0]
    Ft, Mt = [F.zero()] * 3, [F.zero()] * 3
    for nd, fv, mv in loads:
        Ft = vadd(Ft, fv)
        Mt = vadd(Mt, vadd(cross(vsub(pos[nd], pos[A0]), fv), mv))
    res["R"][A0] = [-v for v in Ft] + [-v for v in Mt]
    if X is not None:
        res["R"][C] = list(X)
    for sg in segs:
        m = sg["m"]
        Fb, Mb = W(loads, sg, pos[sg["B"]])
        Fa, Ma = W(loads, sg, pos[sg["A"]])
        end = {sg["B"]: Fb + Mb, sg["A"]: [-v for v in Fa] + [-v for v in Ma]}
        g = member_geom(F, c, m)
        fg = end[m["from"]] + end[m["to"]]
        res["members"][m["key"]] = {"global": fg, "N_w": dot(fg[6:9], g["x"])}
    return res


# ============================================================================
# Checks, recovery and publication


class Checks:
    def __init__(self):
        self.items = []

    def ok(self, name, cond, detail=""):
        self.items.append({"check": name, "pass": bool(cond)})
        if not cond:
            self.items[-1]["detail"] = detail
        return bool(cond)


def run_case(c, chk):
    F = case_field(c)
    K = stiffness_formulation(F, c)
    if c["control"]:
        return F, K, None
    FM = force_method(F, c)
    L = c["label"]
    chk.ok(f"{L}: K free-DOF residual exactly zero", K["free_residual_zero"])
    if "compat_residual_zero" in FM:
        chk.ok(f"{L}: FM compatibility at released anchor exactly zero", FM["compat_residual_zero"])
    for nd in c["chain"]:
        chk.ok(f"{L}: u[{nd}] K == FM (6 comps)", eqv(K["u"][nd], FM["u"][nd]))
    for a in c["anchors"]:
        chk.ok(f"{L}: R[{a}] K == FM (6 comps)", eqv(K["R"][a], FM["R"][a]))
        chk.ok(f"{L}: anchor {a} motion zero", all(v.is_zero() for v in K["u"][a]))
    for m in c["members"]:
        km, fm = K["members"][m["key"]], FM["members"][m["key"]]
        chk.ok(f"{L}: {m['key']} global end wrenches K == FM (12 comps)", eqv(km["global"], fm["global"]))
        chk.ok(f"{L}: {m['key']} N_w K == FM", km["N_w"] == fm["N_w"])
        g = K["geo"][m["key"]]
        du = vsub(K["u"][m["to"]][:3], K["u"][m["from"]][:3])
        pr = member_props(m, c["shear"])
        lhs = dot(du, g["x"]) * pr["EA"] / g["L"]
        chk.ok(f"{L}: {m['key']} constitutive sEA_n eps = N_w - 2 nu p A_i (K u, FM N_w)", lhs == fm["N_w"] - 2 * NU * pai(m))
        # member equilibrium: from-end + to-end wrench balance
        fg = fm["global"]
        xi, xj = qv(F, c["nodes"][m["from"]]), qv(F, c["nodes"][m["to"]])
        chk.ok(f"{L}: {m['key']} member free-body balance", eqv(vadd(fg[0:3], fg[6:9]), [F.zero()] * 3)
               and eqv(vadd(vadd(fg[3:6], fg[9:12]), cross(vsub(xj, xi), fg[6:9])), [F.zero()] * 3))
    # node equilibrium with FM end wrenches, caps/mech loads (as FM physical loads) and K reactions
    for nd in c["chain"]:
        tot = [F.zero()] * 6
        for m in c["members"]:
            fg = FM["members"][m["key"]]["global"]
            if nd == m["from"]:
                tot = vsub(tot, fg[:6])
            if nd == m["to"]:
                tot = vsub(tot, fg[6:])
        for n2, fv, mv in FM["phys"]:
            if n2 == nd:
                tot = vadd(tot, fv + mv)
        if nd in K["R"]:
            tot = vadd(tot, K["R"][nd])
        chk.ok(f"{L}: node {nd} six-component equilibrium", all(v.is_zero() for v in tot))
    # global balance of physical loads + reactions about the origin
    Ft, Mt = [F.zero()] * 3, [F.zero()] * 3
    for n2, fv, mv in FM["phys"]:
        Ft = vadd(Ft, fv)
        Mt = vadd(Mt, vadd(cross(qv(F, c["nodes"][n2]), fv), mv))
    for a, r in K["R"].items():
        Ft = vadd(Ft, r[:3])
        Mt = vadd(Mt, vadd(cross(qv(F, c["nodes"][a]), r[:3]), r[3:]))
    chk.ok(f"{L}: global wrench balance (loads + reactions) about origin", eqv(Ft + Mt, [F.zero()] * 6))
    # virtual work: external work of physical loads on K displacements = internal work of FM actions
    ext = F.zero()
    for n2, fv, mv in FM["phys"]:
        ext = ext + dot(fv, K["u"][n2][:3]) + dot(mv, K["u"][n2][3:])
    internal = F.zero()
    for sg in FM["segs"]:
        P0, P1 = qv(F, c["nodes"][sg["A"]]), qv(F, c["nodes"][sg["B"]])
        Pm = vsc(vadd(P0, P1), Fr(1, 2))
        vals = []
        for P in (P0, Pm, P1):
            Fs, Ms = FM["W"](FM["loads"], sg, P)
            x, pr = sg["xo"], sg["pr"]
            n_, t_ = dot(Fs, x), dot(Ms, x)
            v = n_ * n_ / pr["EA"] + t_ * t_ / pr["GJ"] + (dot(Ms, Ms) - t_ * t_) / pr["EI"]
            if pr["GAs"] is not None:
                v = v + (dot(Fs, Fs) - n_ * n_) / pr["GAs"]
            vals.append(v)
        internal = internal + sg["L"] * (vals[0] + 4 * vals[1] + vals[2]) / 6
        if sg["eps0"]:
            internal = internal + sg["L"] * sg["eps0"] * dot(FM["W"](FM["loads"], sg, P0)[0], sg["xo"])
    chk.ok(f"{L}: virtual work: sum loads.u (K) = internal work (FM actions, total strains)", ext == internal)
    return F, K, FM


def lame_rows(sec_name, p):
    s = SEC[sec_name]
    out = {}
    for basis, area, ri in (("nominal_section", s["An"], s["ri_n"]), ("reduced_section", s["Ar"], s["ri_r"])):
        hoop_outer = 2 * p * ri * ri / area
        out[basis] = {"radial_inner_Pa": str(-p), "radial_outer_Pa": "0", "hoop_outer_Pa": str(hoop_outer),
                      "hoop_inner_Pa": str(hoop_outer + p), "hoop_outer_binary64": float(hoop_outer),
                      "hoop_inner_binary64": float(hoop_outer + p)}
    return out


def publish(c, F, K):
    P = c["power"]
    comps = ["ux_m", "uy_m", "uz_m", "rx_rad", "ry_rad", "rz_rad"]
    fcomps = ["Fx_N", "Fy_N", "Fz_N", "Mx_Nm", "My_Nm", "Mz_Nm"]
    out = {"label": c["label"], "purpose": c["purpose"], "field_basis": F.names(),
           "inputs": {
               "nodes_m": {k: [str(v) for v in c["nodes"][k]] for k in c["chain"]},
               "node_identity": {"A": "RS-170", "N": "RS-160 (component node, option A origin)", "F": "RS-150", "C": "RS-140"},
               "anchors_all_six_dof": c["anchors"], "shear_model": {"eb": "euler_bernoulli", "timo": "timoshenko_annulus_energy_v1"}[c["shear"]],
               "members": [{"id": m["id"], "from": m["from"], "to": m["to"], "section": m["sec"], "pressure_Pa": str(m["p"]),
                            "stiffness_scale": str(m["scale"]), "y_reference": [str(v) for v in m["yref"]], "role": m["role"],
                            "transferring_terminals": sorted(nd for (k, nd) in c["caps"] if k == m["key"])} for m in c["members"]],
               "mechanical_nodal_loads": {k: {"F_N": [str(v) for v in f], "M_Nm": [str(v) for v in mo]} for k, (f, mo) in c["mech"].items()},
           },
           "results": {"displacements": {}, "reactions": {}, "members": []}}
    for nd in c["chain"]:
        if nd in c["anchors"]:
            out["results"]["displacements"][nd] = "exactly 0 (anchored)"
            continue
        out["results"]["displacements"][nd] = {comps[i]: emit(K["u"][nd][i], P - 1) for i in range(6)}
    for a in c["anchors"]:
        out["results"]["reactions"][a] = {fcomps[i]: emit(K["R"][a][i], P) for i in range(6)}
    for m in c["members"]:
        km = K["members"][m["key"]]
        s = SEC[m["sec"]]
        Nw = km["N_w"]
        pa = pai(m)
        g = K["geo"][m["key"]]
        entry = {
            "id": m["id"], "from": m["from"], "to": m["to"], "length_m": emit(g["L"], 0),
            "local_axes": {"x": [emit(v, 0)["exact"] for v in g["x"]], "y": [emit(v, 0)["exact"] for v in g["y"]], "z": [emit(v, 0)["exact"] for v in g["z"]]},
            "end_wrench_on_member_local": {"from": {fcomps[i]: emit(km["local"][i], P) for i in range(6)},
                                           "to": {fcomps[i]: emit(km["local"][6 + i], P) for i in range(6)}},
            "end_wrench_on_member_global": {"from": {fcomps[i]: emit(km["global"][i], P) for i in range(6)},
                                            "to": {fcomps[i]: emit(km["global"][6 + i], P) for i in range(6)}},
            "wall_axial_force_N_w_N": emit(Nw, P),
            "mechanical_axial_force_N_m_N": emit(km["N_m"], P),
            "effective_axial_force_S_N": emit(Nw - pa, P),
            "axial_membrane_stress_Pa": {b: emit(Nw / area, P - 1) for b, area in (("nominal_section", s["An"]), ("reduced_section", s["Ar"]))},
        }
        if m["role"] == "reducer_body":
            entry["note"] = ("mapped reference-section (S270) rows under the body's chord forces; component_body attribution; "
                             "not a cone/taper body stress")
        out["results"]["members"].append(entry)
    return out


def published_vector(c, K):
    vals = []
    for nd in c["chain"]:
        vals += K["u"][nd]
    for a in c["anchors"]:
        vals += K["R"][a]
    for m in c["members"]:
        vals += K["members"][m["key"]]["global"]
    return vals


# ============================================================================
# W-15c run-axis / offset method, re-derived from DEL-03-05


Q_ROUND = Decimal("0.001")
RHO_BASE = Decimal("0.001")
THETA_BASE = Decimal("0.001")
TAU = Decimal("0.005")
SQRT3 = Decimal(3).sqrt()


def d_atan(x):
    x = Decimal(x)
    if x < 0:
        return -d_atan(-x)
    mult = 1
    while x > Decimal("0.05"):
        x = x / (1 + (1 + x * x).sqrt())
        mult *= 2
    term, total, k, x2 = x, Decimal(0), 0, x * x
    eps = Decimal(10) ** -(getcontext().prec - 5)
    while abs(term) > eps:
        total += term / (2 * k + 1)
        term = -term * x2
        k += 1
    return total * mult


def d_asin(x):
    return d_atan(x / (1 - x * x).sqrt())


def dvec(v):
    return [Decimal(Fr(x).numerator) / Decimal(Fr(x).denominator) for x in v]


def dnorm(v):
    return sum(x * x for x in v).sqrt()


def fe(x, p):
    return "0" if x == 0 else f"{x:.{p}e}"


def w15c_judge(xN, xF, ends, e, tau=TAU):
    """ends: {'N': [(class, other_node_xyz)], 'F': [...]}; class 'run' (straight) or 'unsupported'.
    e: None (absent), or a 3-vector in metres. Returns the decision and its quantities."""
    out = {}
    if e is None:
        return {"outcome": "no_check (absent offset: no declaration; nodes govern; not concentric)"}
    status = {}
    for end, X in (("N", xN), ("F", xF)):
        lst = ends[end]
        if not lst:
            status[end] = ("none",)
        elif len(lst) > 1:
            status[end] = ("ambiguous",)
        else:
            cls, other = lst[0]
            if cls != "run":
                status[end] = ("unsupported", cls)
            else:
                d = [b - a for a, b in zip(dvec(X), dvec(other))]
                L = dnorm(d)
                ratio = SQRT3 * Q_ROUND / L
                if not ratio < 1:
                    status[end] = ("unresolvable", f"sqrt(3)q/L = {ratio:.6e} >= 1")
                else:
                    status[end] = ("direction", [v / L for v in d], d_asin(ratio), ratio)
    for end in ("N", "F"):
        if status[end][0] == "ambiguous":
            return {"outcome": "INVALID ambiguous_run", "end": end}
    for end in ("N", "F"):
        if status[end][0] == "unsupported":
            return {"outcome": "INVALID unsupported_run_evidence", "end": end, "class": status[end][1]}
    for end in ("N", "F"):
        if status[end][0] == "unresolvable":
            return {"outcome": "INVALID unresolvable_run_evidence", "end": end, "detail": status[end][1]}
    sN, sF = status["N"], status["F"]
    if sN[0] == "none" and sF[0] == "none":
        return {"outcome": "INVALID standalone_body"}
    axes = []
    if sN[0] == "direction" and sF[0] == "direction":
        dn, df = sN[1], sF[1]
        cr = dnorm([dn[1] * df[2] - dn[2] * df[1], dn[2] * df[0] - dn[0] * df[2], dn[0] * df[1] - dn[1] * df[0]])
        al = abs(sum(a * b for a, b in zip(dn, df)))
        theta = d_atan(cr / al) if al > 0 else 2 * d_atan(Decimal(1))  # pi/2
        bound = THETA_BASE + sN[2] + sF[2]
        out.update({"theta_rad": f"{theta:.15e}", "agreement_bound_rad": f"{bound:.15e}", "agreement_margin_rad": f"{bound - theta:.6e}",
                    "beta_N_rad": f"{sN[2]:.15e}", "beta_F_rad": f"{sF[2]:.15e}"})
        if not theta <= bound:
            out["outcome"] = "INVALID run_directions_disagree"
            return out
        if sN[2] < sF[2]:
            axes, out["governing_end"] = [(sN[1], sN[2], sN[3])], "N"
        elif sF[2] < sN[2]:
            axes, out["governing_end"] = [(sF[1], sF[2], sF[3])], "F"
        else:
            axes, out["governing_end"] = [(sN[1], sN[2], sN[3]), (sF[1], sF[2], sF[3])], "tie (J3: each end judged)"
    else:
        s = sN if sN[0] == "direction" else sF
        axes, out["governing_end"] = [(s[1], s[2], s[3])], "N" if s is sN else "F"
    D = [b - a for a, b in zip(dvec(xN), dvec(xF))]
    ev = dvec(e)
    judgments = []
    for u, beta, sinb in axes:  # sin(beta) = sqrt(3) q / L exactly for straight evidence
        t = u if sum(a * b for a, b in zip(u, D)) >= 0 else [-v for v in u]
        Dt = sum(a * b for a, b in zip(D, t))
        Dp = [a - Dt * b for a, b in zip(D, t)]
        en = dnorm(ev)
        a_e = RHO_BASE * en + SQRT3 / 2 * Q_ROUND + en * sinb
        resid = dnorm([a - b for a, b in zip(Dp, ev)])
        axis_allow = dnorm(D) * sinb
        j = {"t": [f"{v:.15g}" for v in t], "D_dot_t_m": f"{Dt:.15g}", "Dperp_m": [fe(v, 12) for v in Dp],
             "beta_run_rad": f"{beta:.15e}", "residual_m": fe(resid, 12), "placement_allowance_m": f"{tau + axis_allow:.12e}",
             "placement_margin_m": f"{tau + axis_allow - resid:.6e}"}
        zero = all(v == 0 for v in ev)
        if not zero:
            j.update({"a_e_m": f"{a_e:.12e}", "precision_margin_m": f"{en - a_e:.6e}", "e_dot_t_m": f"{abs(sum(a * b for a, b in zip(ev, t))):.12e}",
                      "transversality_margin_m": f"{a_e - abs(sum(a * b for a, b in zip(ev, t))):.6e}"})
        if not Dt > 0:
            j["outcome"] = "INVALID no_axial_length"
        elif not zero and not a_e < en:
            j["outcome"] = "BELOW_PRECISION"
        elif not zero and not abs(sum(a * b for a, b in zip(ev, t))) <= a_e:
            j["outcome"] = "NOT_TRANSVERSE"
        elif not resid <= tau + axis_allow:
            j["outcome"] = "MISMATCH"
        else:
            j["outcome"] = "REALIZED" + (" with AXIS_ALLOWANCE_USED warning" if resid > tau else "")
        judgments.append(j)
    if len(judgments) == 1:
        out.update(judgments[0])
    else:
        refused = [j for j in judgments if not j["outcome"].startswith("REALIZED")]
        rep = max(refused or judgments, key=lambda j: Decimal(j["residual_m"]))
        out.update(rep)
        out["tie_judgments"] = [j["outcome"] for j in judgments]
    return out


def b64(v):
    return tuple(Fr(float(Fr(x))) for x in v)


def geometry_controls():
    N, Fp, A, C = RS["RS-160"], RS["RS-150"], RS["RS-170"], RS["RS-140"]
    run = lambda X: [("run", X)]
    std = {"N": run(A), "F": run(C)}
    e25 = (0, 0, mm(25))
    ctrl = []

    def add(name, xN, xF, ends, e, expect, note=""):
        r = w15c_judge(xN, xF, ends, e)
        rb = w15c_judge(b64(xN), b64(xF), {k: [(cl, b64(o)) for cl, o in v] for k, v in ends.items()}, None if e is None else b64(e))
        ctrl.append({"control": name, "note": note, "inputs": {"x_N": [str(v) for v in xN], "x_F": [str(v) for v in xF],
                     "ends": {k: [[cl, [str(x) for x in o]] for cl, o in v] for k, v in ends.items()},
                     "declared_offset_m": None if e is None else [str(v) for v in e]},
                     "expected_by_design": expect, "exact_decimal_inputs": r, "decimal_on_binary64_rounded_inputs": {"outcome": rb["outcome"]}})
        return r

    add("G-RS-A (RS-R1 actual, option A, +25 mm z)", N, Fp, std, e25, "REALIZED", "residual 0; N end (2.6 m) governs")
    add("G-RS-ABSENT", N, Fp, std, None, "no_check", "mechanics from nodes, identical to G-RS-A")
    add("G-RS-ZERO (explicit zero on eccentric geometry)", N, Fp, std, (0, 0, 0), "MISMATCH", "25 mm residual")
    add("G-RS-WRONGSIGN (-25 mm)", N, Fp, std, (0, 0, mm(-25)), "MISMATCH", "50 mm residual")
    add("G-RS-22MM (3 mm residual)", N, Fp, std, (0, 0, mm(22)), "REALIZED", "no warning; mechanics unchanged (nodes govern)")
    add("G-RS-19.8MM (5.2 mm residual, in rule-B band)", N, Fp, std, (0, 0, mm("19.8")), "REALIZED with AXIS_ALLOWANCE_USED warning")
    add("G-RS-19.7MM (5.3 mm residual, beyond band)", N, Fp, std, (0, 0, mm("19.7")), "MISMATCH")
    add("G-RS-AXIAL-0.90MM (transversality inside)", N, Fp, std, (0, mm("0.90"), mm(25)), "REALIZED")
    add("G-RS-AXIAL-0.91MM (transversality outside)", N, Fp, std, (0, mm("0.91"), mm(25)), "NOT_TRANSVERSE")
    for h, exp in (("0.8", "BELOW_PRECISION"), ("0.9", "REALIZED")):
        zN = Fr(7) - mm(h)
        add(f"G-PREC-{h}MM (geometry and declared offset {h} mm)", (Fr(6), Fr("20.4"), zN), Fp,
            {"N": run((Fr(6), Fr(23), zN)), "F": run(C)}, (0, 0, mm(h)), exp)
    add("G-NONPAR (RS-170 at (6.003, 23, 6.976); mechanics case RS2B-NONPAR)", N, Fp, {"N": run(NONPAR_A), "F": run(C)}, e25,
        "REALIZED", "N governs; agreement theta 1.216e-3 <= bound 2.53e-3")
    # agreement boundary: tilt RS-170 out of plane by dx; find the boundary and step 0.1 um either side
    lo, hi = Decimal(0), Decimal("0.01")
    for _ in range(90):
        mid = (lo + hi) / 2
        r = w15c_judge(N, Fp, {"N": run((Fr(6) + Fr(str(mid)), Fr(23), Fr("6.975"))), "F": run(C)}, (0, 0, 0))
        if r["outcome"] == "INVALID run_directions_disagree":
            hi = mid
        else:
            lo = mid
    dx_in = (lo * 10 ** 7).to_integral_value(rounding="ROUND_FLOOR") / Decimal(10 ** 7)
    dx_out = dx_in + Decimal("1e-7")
    for tag, dx, exp in (("inside", dx_in, "REALIZED"), ("outside", dx_out, "INVALID run_directions_disagree")):
        add(f"G-AGREE-{tag} (RS-170 x + {dx} m)", N, Fp, {"N": run((Fr(6) + Fr(str(dx)), Fr(23), Fr("6.975"))), "F": run(C)}, e25, exp,
            note=f"boundary dx* in ({lo:.12e}, {hi:.12e}) m by bisection")
    add("G-TIE (equal 2 m neighbours)", N, Fp, {"N": run((Fr(6), Fr("22.4"), Fr("6.975"))), "F": run(C)}, e25, "REALIZED", "J3 tie: both ends judged, both realized")
    add("G-SHORT-UNRESOLVABLE (N neighbour 1.7 mm)", N, Fp, {"N": run((Fr(6), Fr("20.4017"), Fr("6.975"))), "F": run(C)}, e25, "INVALID unresolvable_run_evidence")
    add("G-SHORT-RESOLVED (N neighbour 1.8 mm, wide beta; F governs)", N, Fp, {"N": run((Fr(6), Fr("20.4018"), Fr("6.975"))), "F": run(C)}, e25, "REALIZED", "F governs")
    add("G-AMBIGUOUS (tee at N)", N, Fp, {"N": run(A) + run((Fr(7), Fr("20.4"), Fr("6.975"))), "F": run(C)}, e25, "INVALID ambiguous_run")
    add("G-UNSUPPORTED (other reducer body at N)", N, Fp, {"N": [("reducer_body", A)], "F": run(C)}, e25, "INVALID unsupported_run_evidence")
    add("G-UNSUPPORTED-BEND-CHORD (unrealized bend member at F)", N, Fp, {"N": run(A), "F": [("bend_unrealized", C)]}, e25, "INVALID unsupported_run_evidence")
    add("G-STANDALONE", N, Fp, {"N": [], "F": []}, e25, "INVALID standalone_body")
    add("G-TERMINAL-F (no member at F; N governs)", N, Fp, {"N": run(A), "F": []}, e25, "REALIZED")
    add("G-NO-AXIAL-LENGTH (body across the run)", (Fr(6), Fr(20), Fr("6.6")), Fp, {"N": run((Fr(6), Fr(23), Fr("6.6"))), "F": run(C)}, (0, 0, Fr("0.4")),
        "INVALID no_axial_length")
    # far origin, short skewed concentric body (the 2a 64-epsilon review case family)
    O = (Fr("100.123"), Fr("200.456"), Fr("50.789"))
    def at(s):
        return tuple(o + Fr(s) * Fr(k, 13) for o, k in zip(O, (3, 4, 12)))
    xA, xN2, xF2, xC2 = at("0"), at("2.6"), at("2.73"), at("5.33")
    r = add("G-FARORIGIN-SKEW-SHORT (0.13 m concentric body near 100-200 m, skew (3,4,12)/13, explicit zero)", xN2, xF2,
            {"N": run(xA), "F": run(xC2)}, (0, 0, 0), "REALIZED", "exact and binary64-rounded inputs")
    xb = [b64(p) for p in (xA, xN2, xF2, xC2)]
    def unit_b(a, b):
        d = [float(y) - float(x) for x, y in zip(a, b)]
        n = sum(v * v for v in d) ** 0.5
        return [v / n for v in d]
    ub, un = unit_b(xb[1], xb[2]), unit_b(xb[1], xb[0])
    crs = [ub[1] * un[2] - ub[2] * un[1], ub[2] * un[0] - ub[0] * un[2], ub[0] * un[1] - ub[1] * un[0]]
    ctrl[-1]["information_64eps_representation_metric"] = {
        "binary64_norm_of_body_x_neighbour_cross": sum(v * v for v in crs) ** 0.5, "64_eps": 64 * 2.0 ** -52,
        "note": "informational binary64 evaluation of the 2a representation guard metric; not the 2b admission rule"}
    return ctrl


# ============================================================================
# Cross-case relations and mutation controls


def by_label(solved, lab):
    return solved[lab]


def relations(solved, chk):
    def K(l):
        return solved[l][2]

    def c_(l):
        return solved[l][0]

    def F_(l):
        return solved[l][1]

    # determinate free capped: zero anchor reaction, N_w = p A_i, zero shear/moment everywhere
    for sh in ("EB", "TIMO"):
        for s in (1, 2, 4):
            l = f"RS2B-DET-FREE-{sh}-S{s}"
            chk.ok(f"{l}: anchor reaction exactly zero (each region self-equilibrated)", all(v.is_zero() for v in K(l)["R"]["A"]))
            for m in c_(l)["members"]:
                loc = K(l)["members"][m["key"]]["local"]
                chk.ok(f"{l}: {m['key']} N_w = p A_i, no shear/torsion/bending", loc[6] == pai(m) and all(loc[i].is_zero() for i in (1, 2, 3, 4, 5, 7, 8, 9, 10, 11)))
        # affine law in 1/s holds for determinate cases
        for nd in ("N", "F", "C"):
            u1, u2, u4 = (K(f"RS2B-DET-FREE-{sh}-S{s}")["u"][nd] for s in (1, 2, 4))
            chk.ok(f"DET-FREE-{sh}: u[{nd}] affine in 1/s (determinate only)", eqv(vsub(u1, u2), vsc(vsub(u2, u4), 2)))
        for s in (1, 2):
            l = f"RS2B-DET-COMP-{sh}-S{s}"
            c, F = c_(l), F_(l)
            m = next(x for x in c["members"] if x["key"] == "nbrF")
            pa = pai(m)
            sC = [F.r(v) for v in (0, 1, 0)]
            xCA = [F.r(b - a) for a, b in zip(c["nodes"]["A"], c["nodes"]["C"])]
            RA = [v * (-pa) for v in sC] + [v * (-pa) for v in cross(xCA, sC)]
            chk.ok(f"{l}: R_A = -(p A_F) s_C, M_A = -(x_C - x_A) x (p A_F s_C) (eccentric step statics)", eqv(K(l)["R"]["A"], RA))
            chk.ok(f"{l}: R_A moment x-component = +p A_F * 0.025 m (P*|Dperp|)", K(l)["R"]["A"][3] == pa * Fr(1, 40))
        for m in ("nbrN", "body", "nbrF"):
            chk.ok(f"DET-COMP-{sh}: {m} end wrenches independent of body scale (s=1 vs 2; determinate)",
                   eqv(K(f"RS2B-DET-COMP-{sh}-S1")["members"][m]["global"], K(f"RS2B-DET-COMP-{sh}-S2")["members"][m]["global"]))
    for s in (1, 2):
        l1, l2 = f"RS2B-DET-COMP-EB-S{s}", f"RS2B-DET-COMP-TIMO-S{s}"
        chk.ok(f"DET-COMP S{s}: EB and Timoshenko forces identical (determinate)", all(eqv(K(l1)["members"][k]["global"], K(l2)["members"][k]["global"]) for k in ("nbrN", "body", "nbrF")))
        chk.ok(f"DET-COMP S{s}: EB and Timoshenko displacements differ (shear compliance)", not eqv(K(l1)["u"]["C"], K(l2)["u"]["C"]))
    # indeterminate scale response: changes, and is not asserted affine
    for sh in ("EB", "TIMO"):
        chk.ok(f"HELD-{sh}: body scale redistributes reactions (s=1 vs 2)", not eqv(K(f"RS2B-HELD-{sh}-S1")["R"]["A"], K(f"RS2B-HELD-{sh}-S2")["R"]["A"]))
        chk.ok(f"HELD-{sh}: EB vs Timoshenko differ", not eqv(K("RS2B-HELD-EB-S2")["R"]["A"], K("RS2B-HELD-TIMO-S2")["R"]["A"]))
    u1, u2, u4 = (K(f"RS2B-HELD-EB-S{s}")["u"]["N"] for s in (1, 2, 4))
    affine = eqv(vsub(u1, u2), vsc(vsub(u2, u4), 2))
    chk.ok("HELD-EB: indeterminate u[N](s) is NOT affine in 1/s (m_rigid + c/s not imposed)", not affine)
    # orientation reversal of the body
    for sh in ("EB", "TIMO"):
        a, b = f"RS2B-HELD-{sh}-S2", f"RS2B-HELD-REV-{sh}-S2"
        chk.ok(f"REV-{sh}: displacements and reactions invariant to body orientation", all(eqv(K(a)["u"][n], K(b)["u"][n]) for n in "ANFC") and all(eqv(K(a)["R"][n], K(b)["R"][n]) for n in "AC"))
        ka, kb = K(a)["members"]["body"], K(b)["members"]["body"]
        flip = [-1, 1, -1, -1, 1, -1]
        chk.ok(f"REV-{sh}: N_w invariant", ka["N_w"] == kb["N_w"])
        chk.ok(f"REV-{sh}: local end wrench at new from = diag(-1,1,-1,-1,1,-1) x old to (and vice versa)",
               eqv(kb["local"][:6], [v * f for v, f in zip(ka["local"][6:], flip)]) and eqv(kb["local"][6:], [v * f for v, f in zip(ka["local"][:6], flip)]))
    # rotation covariance
    for base, rot in (("RS2B-HELD-TIMO-S2", "RS2B-ROT-HELD-TIMO-S2"), ("RS2B-MECH-TIMO-S2", "RS2B-ROT-MECH-TIMO-S2")):
        Fb = F_(rot)
        assert F_(base).b == Fb.b

        def R6(v):
            return [sum((ROT[i][j] * v[j] for j in range(3)), Fb.zero()) for i in range(3)] + [sum((ROT[i][j] * v[3 + j] for j in range(3)), Fb.zero()) for i in range(3)]

        okc = True
        for n in "ANFC":
            ub = K(base)["u"][n]
            ubb = [QF(Fb, list(x.c)) for x in ub]
            okc &= eqv(K(rot)["u"][n], R6(ubb))
        for n in "AC":
            okc &= eqv(K(rot)["R"][n], R6([QF(Fb, list(x.c)) for x in K(base)["R"][n]]))
        chk.ok(f"{rot}: displacements/reactions = R x base (rational rotation + far translation)", okc)
        chk.ok(f"{rot}: local end wrenches and N_w identical to base", all(eqv(K(rot)["members"][k]["local"], [QF(Fb, list(x.c)) for x in K(base)["members"][k]["local"]]) for k in ("nbrN", "body", "nbrF")))
    # junction cap wrench = closed-surface end-disc formula (actual end directions), no extra pAe
    for l in ("RS2B-HELD-TIMO-S2", "RS2B-NONPAR-HELD-TIMO-S2", "RS2B-HELD-UNEQ-TIMO-S2"):
        c, F = c_(l), F_(l)
        xN = qv(F, c["nodes"]["N"])
        tot_f, tot_m = [F.zero()] * 3, [F.zero()] * 3
        for m in c["members"]:
            for nd in ("N", "F"):
                if (m["key"], nd) in c["caps"] and nd in (m["from"], m["to"]):
                    other = m["to"] if nd == m["from"] else m["from"]
                    d = [b - a for a, b in zip(c["nodes"][nd], c["nodes"][other])]
                    s_in = [F.r(v) / F.sqrt(sum(x * x for x in d)) for v in d]
                    cap = vsc(s_in, -pai(m))
                    tot_f = vadd(tot_f, cap)
                    tot_m = vadd(tot_m, cross(vsub(qv(F, c["nodes"][nd]), xN), cap))
        disc_f, disc_m = [F.zero()] * 3, [F.zero()] * 3
        for nd, key, other in (("N", "nbrN", "A"), ("F", "nbrF", "C")):
            m = next(x for x in c["members"] if x["key"] == key)
            d = [b - a for a, b in zip(c["nodes"][nd], c["nodes"][other])]
            n_out = [F.r(v) / F.sqrt(sum(x * x for x in d)) for v in d]
            fd = vsc(n_out, -pai(m))
            disc_f = vadd(disc_f, fd)
            disc_m = vadd(disc_m, cross(vsub(qv(F, c["nodes"][nd]), xN), fd))
        chk.ok(f"{l}: junction caps (N, F) = fitting wall wrench by closed-surface end discs, actual directions", eqv(tot_f + tot_m, disc_f + disc_m))
        if l != "RS2B-NONPAR-HELD-TIMO-S2":
            mN = next(x for x in c["members"] if x["key"] == "nbrN")
            mF = next(x for x in c["members"] if x["key"] == "nbrF")
            t = [F.r(v) for v in (0, -1, 0)]
            Dp = [F.zero(), F.zero(), F.r(Fr(1, 40))]
            cf = vsc(t, pai(mN) - pai(mF))
            cm = vsc(cross(Dp, t), -pai(mF))
            chk.ok(f"{l}: parallel-end closed form F = (pA_N - pA_F) t, M_xN = -pA_F (Dperp x t)", eqv(tot_f + tot_m, cf + cm))


MUTANT_SOLUTIONS = {}  # package addition: name -> (reference label, mutated case, field, K solution)
_STATE = {}  # package addition: the solved cleared cases, for the package layer


def mutations(solved, chk):
    out = {}
    ref = "RS2B-HELD-TIMO-S2"
    c0 = solved[ref][0]
    t = (Fr(0), Fr(-1), Fr(0))
    muts = {
        "drop_body_cap_at_F": dict(drop_caps={("body", "F")}),
        "extra_pAe_moment_at_N (double count)": dict(extra_moment={"N": None}),
        "poisson_pair_scaled_by_s": dict(poisson_scaled=True),
        "caps_on_nominal_bore": dict(nominal_bore=True),
        "kappa_5_6_instead_of_kappa_E": dict(kappa=KAPPA_COWPER_MUTANT),
        "body_caps_along_run_axis_not_chord (averaged-axis substitution)": dict(cap_dir={("body", "F"): t, ("body", "N"): tuple(-v for v in t)}),
        "nbrF_cap_projected_to_declared_offset_point (e = 22 mm, 3 mm residual accepted)": dict(cap_point={("nbrF", "F"): (Fr(6), Fr(20), Fr("6.997"))}),
    }
    for name, mu in muts.items():
        mu = dict(mu)
        if "extra_moment" in mu:
            F = solved[ref][1]
            mF = next(x for x in c0["members"] if x["key"] == "nbrF")
            pa = pai(mF)
            Dp = (Fr(0), Fr(0), Fr(1, 40))
            mo = tuple(-pa * v for v in (Dp[1] * t[2] - Dp[2] * t[1], Dp[2] * t[0] - Dp[0] * t[2], Dp[0] * t[1] - Dp[1] * t[0]))
            mu["extra_moment"] = {"N": mo}
        c = dict(c0, mut=mu, control=True, label=ref + " MUTANT " + name)
        F, Km, _ = run_case(c, chk)
        diff = [not (a - QF(F, list(b.c))).is_zero() for a, b in zip(published_vector(c, Km), published_vector(c0, solved[ref][2]))]
        out[name] = {"against": ref, "published_values_changed": sum(diff), "of": len(diff)}
        MUTANT_SOLUTIONS[name] = (ref, c, F, Km)  # package addition: retained for tolerance sensitivity
        chk.ok(f"mutation detected: {name}", any(diff))
    # actual-direction mutant on the nonparallel case: neighbour caps along the governing run axis
    refn = "RS2B-NONPAR-HELD-TIMO-S2"
    cn = solved[refn][0]
    mu = {"cap_dir": {("nbrN", "N"): (Fr(0), Fr(1), Fr(0))}}
    c = dict(cn, mut=mu, control=True, label=refn + " MUTANT nbrN cap along axis (0,1,0) instead of actual direction")
    F, Km, _ = run_case(c, chk)
    diff = [not (a - b).is_zero() for a, b in zip(published_vector(c, Km), published_vector(cn, solved[refn][2]))]
    out["nbrN_cap_along_chosen_axis_not_actual_direction"] = {"against": refn, "published_values_changed": sum(diff), "of": len(diff)}
    MUTANT_SOLUTIONS["nbrN_cap_along_chosen_axis_not_actual_direction"] = (refn, c, F, Km)  # package addition
    chk.ok("mutation detected: nonparallel neighbour cap along chosen axis", any(diff))
    return out


def build():
    chk = Checks()
    solved = {}
    for c in build_cases():
        F, K, FM = run_case(c, chk)
        solved[c["label"]] = (c, F, K)
    _STATE["solved"] = solved  # package addition
    relations(solved, chk)
    mut = mutations(solved, chk)
    geo = geometry_controls()
    for g in geo:
        exp, got = g["expected_by_design"], g["exact_decimal_inputs"]["outcome"]
        chk.ok(f"geometry {g['control']}: outcome equals the design expectation", got == exp or (exp.startswith("no_check") and got.startswith("no_check")), f"{got} vs {exp}")
        chk.ok(f"geometry {g['control']}: Decimal re-evaluation of binary64-rounded inputs gives the same outcome", g["decimal_on_binary64_rounded_inputs"]["outcome"] == got)
    undecided = []
    pub = []
    for lab, (c, F, K) in solved.items():
        p = publish(c, F, K)
        pub.append(p)

        def walk(o, path):
            if isinstance(o, dict):
                if "binary64" in o and o["binary64"] is None:
                    undecided.append(path)
                for k, v in o.items():
                    walk(v, path + "/" + str(k))
            elif isinstance(o, list):
                for i, v in enumerate(o):
                    walk(v, f"{path}[{i}]")
        walk(p, lab)
    lame = {f"{s}@{p}Pa": lame_rows(s, p) for s in SEC for p in (Fr(1_600_000), Fr(2_000_000), Fr(1_200_000))}
    failed = [i for i in chk.items if not i["pass"]]
    out = {
        "reference_id": "rt_a3_reducer_2b_frame_candidate",
        "status": "CANDIDATE ONLY: not adopted, not frozen, not product evidence; independent scrutiny and adoption are separate",
        "source_revision_read": SOURCE_REVISION, "integrated_baseline": BASELINE,
        "independence": "standard library only; no product code/output/frozen product reference imported, read or run; invented controls labelled",
        "model": {
            "material_invented_control": {"E_Pa": str(E), "nu": str(NU), "G_Pa": str(G), "note": "RS-M1-like invented values; G = E/(2(1+nu))"},
            "sections": {k: {kk: str(vv) for kk, vv in s.items() if kk != "name"} for k, s in SEC.items()},
            "section_note": "areas/inertias are coefficients of pi; A_n nominal (stiffness), A_r reduced, A_i = r_i,r^2 pressure bore; kappa = kappa_E (DEL-05-03 §3.1) on the nominal annulus",
            "theory": "exact mathematical model with declared decimal inputs, not product binary64 leaves; product binding uses scaled tolerance, not bit equality; decimal fields are informational midpoint displays; small-displacement linear elastic straight 3-D frame; s scales E and G of the mapped body; Poisson pair 2 nu p A_i unscaled; caps -p A_i s_into at transferring terminals; N_w = N_m + 2 nu p A_i; S = N_w - p A_i",
            "sign_conventions": {
                "end_wrench_on_member": "force/moment exerted ON the member BY its end node, moment about that end node; local = (k T u - f_eq), global = T^T local; at the to end local Fx = +N_w (tension positive), at the from end local Fx = -N_w",
                "local_axes": "x = (x_to - x_from)/L; y = normalized(y_ref - (y_ref.x) x); z = x cross y (frame kernel convention)",
                "reaction": "force/moment ON the structure FROM the support, moment about the anchored node: R = K u - f",
                "dof_order": "ux, uy, uz, rx, ry, rz per node; Timoshenko rotations are section rotations",
                "Poisson_eigen_pair": "f_eq local: +2 nu p A_i at from, -2 nu p A_i at to (index 0 and 6)",
            },
            "exact_form": "value = (sum_m c_m * prod sqrt(b_i)) * pi^k, c_m rational; forces k=1 and displacements k=0 in pressure cases; forces k=0 and displacements k=-1 in mechanical cases; 'binary64' is null when the rational enclosure (pi to 1e-100, square roots to 1e-70) does not decide the rounding",
            "formulations": {
                "K": "assembled 3-D frame stiffness, exact Gauss-Jordan over the field",
                "FM": "force method on the cantilever released at C (six redundants by exact unit-load compatibility) or the determinate cantilever; Poisson as eigenstrain; Simpson integration exact for the quadratic densities (no interior loads)",
            },
        },
        "lame_rows_Pa": lame,
        "cases": pub,
        "geometry_admission_controls_w15c": geo,
        "mutation_controls": mut,
        "emission": {"binary64_undecided_paths": undecided, "float_emission_ready": not undecided},
        "checks": {"count": len(chk.items), "failed": failed, "all_pass": not failed},
    }
    return out




# ############################################################################
# PACKAGE LAYER (2b reference-package candidate). Everything above this line is the cleared
# frame mathematics, unchanged apart from the five marked retention lines labelled "package addition".
# Below: j-side section actions by two routes, reference-section stress rows on both D-5 bases,
# rigorous hypot enclosures, endpoint maxima by convexity, physically dimensioned zero scales,
# tolerance sensitivity of the cleared mutants, one junction-held C3 analogue, and --check.
# ############################################################################
from math import floor as _floor, ceil as _ceil
import pathlib

PACKAGE_ID = "rt_a3_reducer_2b_reference_package_candidate"
REL_TOL = Fr(1, 10 ** 9)
STATIONS = (("end_i", Fr(0)), ("quarter_1", Fr(1, 4)), ("midspan", Fr(1, 2)), ("quarter_3", Fr(3, 4)), ("end_j", Fr(1)))
BASES = ("nominal_section", "reduced_section")
DIGITS = (70, 140, 280, 560)


def basis_section(sec_name, basis):
    """D-5 basis section; areas, inertias and moduli are coefficients of pi (true value = coef * pi)."""
    s = SEC[sec_name]
    ro = s["ro"]
    ri = s["ri_n"] if basis == "nominal_section" else s["ri_r"]
    I = (ro ** 4 - ri ** 4) / 4
    A = ro * ro - ri * ri
    assert A == (s["An"] if basis == "nominal_section" else s["Ar"])
    return {"ro": ro, "ri": ri, "A": A, "I": I, "Z": I / ro, "J": 2 * I,
            "cb": ((3 + 2 * NU) * ro * ro + (1 + 2 * NU) * ri * ri) / (4 * (1 + NU) * I)}


def qsign(x):
    if x.is_zero():
        return 0
    for d in DIGITS + (1120,):
        lo, hi = x.interval(d)
        if lo > 0:
            return 1
        if hi < 0:
            return -1
    raise AssertionError("sign undecided")  # unreachable: a nonzero field element is a nonzero real


def qabs(x):
    return -x if qsign(x) < 0 else x


def times_pi(lo, hi, k):
    if k == 0:
        return lo, hi
    plo, phi_ = (PI_LO ** k, PI_HI ** k) if k > 0 else (PI_HI ** k, PI_LO ** k)
    c = [lo * plo, lo * phi_, hi * plo, hi * phi_]
    return min(c), max(c)


def sqrt_bounds(q, digits):
    if q.is_zero():
        return Fr(0), Fr(0)
    lo, hi = q.interval(digits)
    lo = max(lo, Fr(0))
    S = 10 ** digits
    return Fr(isqrt(_floor(lo * S * S)), S), Fr(isqrt(_ceil(hi * S * S)) + 1, S)


class V:
    """Exact value (a + c*sqrt(q)) * pi^k, a and q field elements, c rational (c = 0: a field value)."""
    __slots__ = ("a", "c", "q", "k")

    def __init__(self, a, k, c=Fr(0), q=None):
        self.a, self.k, self.c, self.q = a, k, Fr(c), q

    def is_field(self):
        return self.c == 0 or self.q is None or self.q.is_zero()

    def is_zero(self):
        return self.a.is_zero() and self.is_field()

    def enc(self, digits=70):
        lo, hi = self.a.interval(digits)
        if not self.is_field():
            slo, shi = sqrt_bounds(self.q, digits)
            if self.c > 0:
                lo, hi = lo + self.c * slo, hi + self.c * shi
            else:
                lo, hi = lo + self.c * shi, hi + self.c * slo
        return times_pi(lo, hi, self.k)

    def exact_text(self):
        if self.is_zero():
            return "0"
        t = self.a.exact() if not self.a.is_zero() else ""
        if not self.is_field():
            t = (f"({t}) + " if t else "") + f"({self.c})*sqrt({self.q.exact()})"
        elif t:
            t = f"({t})"
        return t if self.k == 0 else (f"({t})*" if not self.is_field() and not self.a.is_zero() else f"{t}*") + ("pi" if self.k == 1 else f"pi^{self.k}")

    def emit(self, exact=True):
        if self.is_zero():
            return {"exact": "0", "decimal": "0", "binary64": 0.0}
        f = None
        for d in DIGITS:
            lo, hi = self.enc(d)
            if float(lo) == float(hi):
                f = float(lo)
                break
        mid = (lo + hi) / 2
        out = {"exact": self.exact_text() if exact else None, "decimal": f"{Decimal(mid.numerator) / Decimal(mid.denominator):.40g}",
               "binary64": f}
        if not exact:
            del out["exact"]
        return out


def vcmp(u, v):
    """Exact comparison of two V values (-1, 0, 1)."""
    if u.k == v.k and u.is_field() and v.is_field():
        return qsign(u.a - v.a)
    if u.k == v.k and u.c == v.c and (u.q == v.q if (u.q is not None and v.q is not None) else u.q is v.q) and u.a == v.a:
        return 0
    for d in DIGITS:
        ul, uh = u.enc(d)
        vl, vh = v.enc(d)
        if uh < vl:
            return -1
        if vh < ul:
            return 1
    raise AssertionError("comparison undecided at 560 digits")


def vmax(vals):
    best = None
    for v in vals:
        if best is None or vcmp(v, best) > 0:
            best = v
    return best


def vabs_field(x, k):
    return V(qabs(x), k)


# ---------------------------------------------------------------------------- section actions


def section_actions_K(F, c, K, m):
    """j-side section action (exerted ON the from-side part BY the to-side part, element-local frame) by
    member statics from the K from-end action f_i (on the member, about the from node): with no interior
    load, S_F = -F_i and S_M(xi) = -M_i + xi L e_x x F_i, i.e. N = -Fx_i, V = -(Fy_i, Fz_i), T = -Mx_i,
    M_y = -My_i - xi L Fz_i, M_z = -Mz_i + xi L Fy_i."""
    fl = K["members"][m["key"]]["local"]
    L = K["geo"][m["key"]]["L"]
    out = []
    for _, xi in STATIONS:
        xL = L * xi
        out.append([-fl[0], -fl[1], -fl[2], -fl[3], -fl[4] - xL * fl[2], -fl[5] + xL * fl[1]])
    return out


def section_actions_FM(F, c, FM, m):
    """Independent j-side section action from the force-method statics (frame-free resultant of every
    load, including the solved redundant, on the C-side of the cut, about the cut point). The C-side
    resultant is the action of the C-side part on the A-side part; it is the j-side action when the
    member's to node is the C-side end, and its negative otherwise. Returns global (F, M) per station."""
    (sg,) = [s for s in FM["segs"] if s["m"]["key"] == m["key"]]
    sgn = 1 if m["to"] == sg["B"] else -1
    xi0, xj0 = qv(F, c["nodes"][m["from"]]), qv(F, c["nodes"][m["to"]])
    out = []
    for _, xi in STATIONS:
        P = vadd(xi0, vsc(vsub(xj0, xi0), xi))
        Fs, Ms = FM["W"](FM["loads"], sg, P)
        out.append((vsc(Fs, sgn), vsc(Ms, sgn)))
    return out


def stress_rows(c, m, sa, P):
    """Reference-section stress rows on both bases from j-side actions sa = [N, Vy, Vz, T, My, Mz]."""
    k = P - 1
    rows = {}
    for b in BASES:
        bs = basis_section(m["sec"], b)
        N, Vy, Vz, T, My, Mz = sa
        q = My * My + Mz * Mz
        w = Vy * Vy + Vz * Vz
        f = 1 / bs["Z"]
        rows[b] = {
            "axial_membrane_Pa": V(N / bs["A"], k),
            "bending_normal_y_Pa": V(My * f, k),
            "bending_normal_z_Pa": V(Mz * f, k),
            "torsional_shear_Pa": V(T * (bs["ro"] / bs["J"]), k),
            "normal_extreme_plus_Pa": V(N / bs["A"], k, f, q),
            "normal_extreme_minus_Pa": V(N / bs["A"], k, -f, q),
            "transverse_shear_maximum_Pa": V(F0(N), k, bs["cb"], w),
            "max_abs_normal_Pa": V(qabs(N) / bs["A"], k, f, q),
        }
    return rows


def F0(x):
    return x.F.zero()


def member_recovery(F, c, K, m, P, fm=None, chk=None):
    L = c["label"]
    sa = section_actions_K(F, c, K, m)
    km = K["members"][m["key"]]
    g = K["geo"][m["key"]]
    if chk is not None:
        chk.ok(f"{L}: {m['key']} j-side action at end_j == K to-end action (member statics)", eqv(sa[-1], km["local"][6:12]))
        chk.ok(f"{L}: {m['key']} j-side axial == N_w at every station", all(s[0] == km["N_w"] for s in sa))
        if fm is not None:
            sf = section_actions_FM(F, c, fm, m)
            ok_comp = ok_inv = True
            for s_k, (Fg, Mg) in zip(sa, sf):
                loc = [dot(Fg, g["x"]), dot(Fg, g["y"]), dot(Fg, g["z"]), dot(Mg, g["x"]), dot(Mg, g["y"]), dot(Mg, g["z"])]
                ok_comp &= eqv(s_k, loc)
                # frame-free invariants: N, T, |V|^2, |M_perp|^2
                n_, t_ = dot(Fg, g["x"]), dot(Mg, g["x"])
                ok_inv &= (s_k[0] == n_ and s_k[3] == t_ and s_k[1] * s_k[1] + s_k[2] * s_k[2] == dot(Fg, Fg) - n_ * n_
                           and s_k[4] * s_k[4] + s_k[5] * s_k[5] == dot(Mg, Mg) - t_ * t_)
            chk.ok(f"{L}: {m['key']} section actions K-statics == FM-statics at 5 stations (6 comps)", ok_comp)
            chk.ok(f"{L}: {m['key']} frame-free invariants N, T, |V|^2, |M_perp|^2 agree (K vs FM)", ok_inv)
    stations = []
    for (loc, xi), s in zip(STATIONS, sa):
        stations.append({"location": loc, "fraction": xi, "action": s, "stress": stress_rows(c, m, s, P)})
    if chk is not None:
        ok = True
        for b in BASES:
            area = SEC[m["sec"]]["An" if b == "nominal_section" else "Ar"]
            ok &= all(st["stress"][b]["axial_membrane_Pa"].a == km["N_w"] / area for st in stations)
        chk.ok(f"{L}: {m['key']} station membrane stress == cleared axial_membrane_stress_Pa (both bases)", ok)
    # maxima: N, V, T constant; M affine in xi => xi -> |N|/A + |M_perp(xi)|/Z convex => endpoint maximum
    qi = sa[0][4] * sa[0][4] + sa[0][5] * sa[0][5]
    qj = sa[-1][4] * sa[-1][4] + sa[-1][5] * sa[-1][5]
    dM = [sa[-1][4] - sa[0][4], sa[-1][5] - sa[0][5]]
    s_ = qsign(qi - qj)
    if all(v.is_zero() for v in dM):
        gov = ["end_i", "quarter_1", "midspan", "quarter_3", "end_j"]
        why = "M_perp constant along the member: every station ties"
    elif s_ == 0:
        gov = ["end_i", "end_j"]
        why = "|M_perp(end_i)| = |M_perp(end_j)| exactly: both endpoints govern (interior strictly smaller unless M_perp constant)"
    else:
        gov = ["end_i"] if s_ > 0 else ["end_j"]
        why = "strictly larger |M_perp| at the governing endpoint (exact sign of |M_perp,i|^2 - |M_perp,j|^2)"
    maxima = {}
    for b in BASES:
        gi = 0 if gov[0] == "end_i" else 4
        v = stations[gi]["stress"][b]["max_abs_normal_Pa"]
        if chk is not None:
            for st in stations:
                chk.ok(f"{L}: {m['key']} {b} member maximum >= station value at {st['location']}", vcmp(v, st["stress"][b]["max_abs_normal_Pa"]) >= 0)
        maxima[b] = {"value": v, "governing": gov, "reason": why}
    return {"stations": stations, "maxima": maxima, "N_m": km["N_m"], "S": km["N_w"] - pai(m)}


# ---------------------------------------------------------------------------- binding rows and zero scales

FAMILY_UNITS = {"translation": "m", "rotation": "rad", "reaction_force": "N", "reaction_moment": "N*m",
                "member_force": "N", "member_moment": "N*m", "normal_stress": "Pa", "shear_stress": "Pa"}
NORMAL_KEYS = ("axial_membrane_Pa", "bending_normal_y_Pa", "bending_normal_z_Pa", "normal_extreme_plus_Pa",
               "normal_extreme_minus_Pa")
SHEAR_KEYS = ("torsional_shear_Pa", "transverse_shear_maximum_Pa")


def binding_rows(c, F, K, rec):
    """Every bindable quantity: (path, family scope, V). Family scope keys a zero scale."""
    P = c["power"]
    rows = []
    comps = ["ux", "uy", "uz", "rx", "ry", "rz"]
    for nd in c["chain"]:
        if nd in c["anchors"]:
            continue
        for i in range(6):
            rows.append((f"displacements/{nd}/{comps[i]}", ("translation" if i < 3 else "rotation",), V(K["u"][nd][i], P - 1)))
    for a in c["anchors"]:
        for i in range(6):
            rows.append((f"reactions/{a}/{i}", ("reaction_force" if i < 3 else "reaction_moment",), V(K["R"][a][i], P)))
    for m in c["members"]:
        key = m["key"]
        km = K["members"][key]
        for frame in ("local", "global"):
            for i in range(12):
                fam = "member_force" if i % 6 < 3 else "member_moment"
                rows.append((f"members/{key}/end_wrench_{frame}/{i}", (fam, key), V(km[frame][i], P)))
        rows.append((f"members/{key}/N_w", ("member_force", key), V(km["N_w"], P)))
        rows.append((f"members/{key}/N_m", ("member_force", key), V(km["N_m"], P)))
        rows.append((f"members/{key}/S", ("member_force", key), V(km["N_w"] - pai(m), P)))
        r = rec[key]
        names = ["N_w", "V_y", "V_z", "T", "M_y", "M_z"]
        for st in r["stations"]:
            for i in range(6):
                rows.append((f"members/{key}/{st['location']}/{names[i]}", ("member_force" if i < 3 else "member_moment", key), V(st["action"][i], P)))
            for b in BASES:
                for nm in NORMAL_KEYS:
                    rows.append((f"members/{key}/{st['location']}/{b}/{nm}", ("normal_stress", key, b), st["stress"][b][nm]))
                for nm in SHEAR_KEYS:
                    rows.append((f"members/{key}/{st['location']}/{b}/{nm}", ("shear_stress", key, b), st["stress"][b][nm]))
        for b in BASES:
            rows.append((f"members/{key}/maximum/{b}", ("normal_stress", key, b), r["maxima"][b]["value"]))
    return rows


def vabs(v):
    if v.is_field():
        return V(qabs(v.a), v.k)
    lo, hi = v.enc()
    if lo >= 0:
        return v
    if hi <= 0:
        return V(-v.a, v.k, -v.c, v.q)
    for d in DIGITS[1:]:
        lo, hi = v.enc(d)
        if lo >= 0:
            return v
        if hi <= 0:
            return V(-v.a, v.k, -v.c, v.q)
    raise AssertionError("abs undecided")


def zero_scales(c, F, K, rec, rows):
    """Family zero scales: the largest |expected| among the stated family's rows in its scope. A family
    whose rows are all exactly zero takes a dimensioned fallback built from independent inputs and
    other scales (never another unit's number): rotation = translation / L_ref; reaction moment =
    reaction force * L_ref; member moment = member force * L_member; stresses = member force / A_b."""
    P = c["power"]
    groups = {}
    for _, fam, v in rows:
        groups.setdefault(fam, []).append(v)
    L_ref = None
    for m in c["members"]:
        Lm = K["geo"][m["key"]]["L"]
        L_ref = Lm if L_ref is None or qsign(Lm - L_ref) > 0 else L_ref
    sc = {}

    def top(fam):
        vals = [vabs(v) for v in groups.get(fam, []) if not v.is_zero()]
        return vmax(vals) if vals else None

    rule = {}
    t = top(("translation",))
    assert t is not None
    sc[("translation",)], rule[("translation",)] = t, "largest |expected| in family"
    t = top(("reaction_force",))
    if t is None:
        t = vmax([top(("member_force", m["key"])) for m in c["members"]])
        rule[("reaction_force",)] = "fallback: largest member force scale of the case (every reaction force exactly zero)"
    else:
        rule[("reaction_force",)] = "largest |expected| in family"
    sc[("reaction_force",)] = t
    t = top(("rotation",))
    if t is None:
        tr = sc[("translation",)]
        t, rule[("rotation",)] = V(tr.a / L_ref, tr.k), "fallback: translation scale / L_ref (every rotation exactly zero)"
    else:
        rule[("rotation",)] = "largest |expected| in family"
    sc[("rotation",)] = t
    t = top(("reaction_moment",))
    if t is None:
        rf = sc[("reaction_force",)]
        t, rule[("reaction_moment",)] = V(rf.a * L_ref, rf.k), "fallback: reaction force scale * L_ref (every reaction moment exactly zero)"
    else:
        rule[("reaction_moment",)] = "largest |expected| in family"
    sc[("reaction_moment",)] = t
    for m in c["members"]:
        key = m["key"]
        f = top(("member_force", key))
        assert f is not None
        sc[("member_force", key)], rule[("member_force", key)] = f, "largest |expected| in family"
        mo = top(("member_moment", key))
        if mo is None:
            mo, rule[("member_moment", key)] = V(f.a * K["geo"][key]["L"], f.k), "fallback: member force scale * member length (every moment exactly zero)"
        else:
            rule[("member_moment", key)] = "largest |expected| in family"
        sc[("member_moment", key)] = mo
        for b in BASES:
            A = basis_section(m["sec"], b)["A"]
            for fam in ("normal_stress", "shear_stress"):
                t = top((fam, key, b))
                if t is None:
                    t, rule[(fam, key, b)] = V(f.a / A, f.k - 1), "fallback: member force scale / A_b (every row exactly zero)"
                else:
                    rule[(fam, key, b)] = "largest |expected| in family"
                sc[(fam, key, b)] = t
    return sc, rule, L_ref


def publish_scales(sc, rule):
    out = {"case": {}, "members": {}}
    for fam, v in sc.items():
        rec = dict(unit=FAMILY_UNITS[fam[0]], rule=rule[fam], **v.emit())
        if len(fam) == 1:
            out["case"][fam[0]] = rec
        elif len(fam) == 2:
            out["members"].setdefault(fam[1], {})[fam[0]] = rec
        else:
            out["members"].setdefault(fam[1], {}).setdefault(fam[0], {})[fam[2]] = rec
    return out


def tolerance_ratio(exp, obs, zs):
    """Rigorous [lo, hi] of |obs - exp| / (1e-9 max(|exp|, zero_scale))."""
    if exp.is_field() and obs.is_field() and exp.k == obs.k:
        d = V(qabs(obs.a - exp.a), exp.k)
        dlo, dhi = d.enc()
    else:
        el, eh = exp.enc()
        ol, oh = obs.enc()
        dlo, dhi = max(Fr(0), ol - eh, el - oh), max(oh - el, eh - ol)
    el, eh = vabs(exp).enc()
    zl, zh = zs.enc()
    tl, th = REL_TOL * max(el, zl), REL_TOL * max(eh, zh)
    return dlo / th, dhi / tl


# ---------------------------------------------------------------------------- C3 analogue (junction node held)


def junction_held_case():
    return rs_case("RS2B-C3-JHELD-FREE-TIMO-S2", "C3 analogue (package addition): junction node N=RS-160 fully anchored; A, F, C free; 1.6 MPa in all three members, every terminal transfers_to_wall; each region self-equilibrated, so the anchor reaction is exactly zero and no pAe moment arises", shear="timo", scale=2, anchors=("N",))


def junction_held_fm(F, c):
    """Force method reused unchanged on the two determinate branches that leave the held junction N."""
    mem = {m["key"]: m for m in c["members"]}
    out = {}
    for keys, chain in ((("nbrN",), ["N", "A"]), (("body", "nbrF"), ["N", "F", "C"])):
        br = dict(c, chain=chain, members=[mem[k] for k in keys], caps={x for x in c["caps"] if x[0] in keys}, anchors=["N"])
        fm = force_method(F, br)
        for k in keys:
            out[k] = fm
    return out


def run_junction_case(c, chk):
    F = case_field(c)
    K = stiffness_formulation(F, c)
    fms = junction_held_fm(F, c)
    L = c["label"]
    chk.ok(f"{L}: K free-DOF residual exactly zero", K["free_residual_zero"])
    for nd in ("A",):
        chk.ok(f"{L}: u[{nd}] K == FM branch N-A", eqv(K["u"][nd], fms["nbrN"]["u"][nd]))
    for nd in ("F", "C"):
        chk.ok(f"{L}: u[{nd}] K == FM branch N-F-C", eqv(K["u"][nd], fms["body"]["u"][nd]))
    Rfm = vadd(fms["nbrN"]["R"]["N"], fms["body"]["R"]["N"])
    chk.ok(f"{L}: R[N] K == sum of FM branch reactions", eqv(K["R"]["N"], Rfm))
    chk.ok(f"{L}: R[N] exactly zero (every region self-equilibrated; no pAe)", all(v.is_zero() for v in K["R"]["N"]))
    for m in c["members"]:
        km, fm = K["members"][m["key"]], fms[m["key"]]["members"][m["key"]]
        chk.ok(f"{L}: {m['key']} global end wrenches K == FM", eqv(km["global"], fm["global"]))
        chk.ok(f"{L}: {m['key']} N_w = p A_i; no shear, torsion or bending", km["local"][6] == pai(m) and all(km["local"][i].is_zero() for i in (1, 2, 3, 4, 5, 7, 8, 9, 10, 11)))
        g = K["geo"][m["key"]]
        du = vsub(K["u"][m["to"]][:3], K["u"][m["from"]][:3])
        pr = member_props(m, c["shear"])
        chk.ok(f"{L}: {m['key']} constitutive sEA_n eps = N_w - 2 nu p A_i", dot(du, g["x"]) * pr["EA"] / g["L"] == fm["N_w"] - 2 * NU * pai(m))
        # rigid rotation-free stretch: the member's transverse relative motion is zero (no bending)
        chk.ok(f"{L}: {m['key']} no transverse relative motion (pure stretch along the actual chord)",
               dot(du, g["y"]).is_zero() and dot(du, g["z"]).is_zero())
    chk.ok(f"{L}: every nodal rotation exactly zero", all(v.is_zero() for nd in "ANFC" for v in K["u"][nd][3:]))
    return F, K, fms


# ---------------------------------------------------------------------------- publication of the package layer


def publish_recovery(c, K, rec):
    out = {"members": []}
    for m in c["members"]:
        r = rec[m["key"]]
        sts = []
        for st in r["stations"]:
            names = ["N_w_N", "V_y_N", "V_z_N", "T_Nm", "M_y_Nm", "M_z_Nm"]
            P = c["power"]
            sts.append({"location": st["location"], "fraction": str(st["fraction"]),
                        "section_action_j_side_local": {names[i]: V(st["action"][i], P).emit() for i in range(6)},
                        "stress": {b: {nm: v.emit(exact=nm in ("axial_membrane_Pa", "bending_normal_y_Pa", "bending_normal_z_Pa", "torsional_shear_Pa"))
                                       for nm, v in st["stress"][b].items()} for b in BASES}})
        out["members"].append({
            "id": m["id"], "role": m["role"], "section": m["sec"],
            "stations": sts,
            "member_maximum_abs_normal_Pa": {b: {"value": r["maxima"][b]["value"].emit(exact=False), "governing_locations": r["maxima"][b]["governing"],
                                                 "reason": r["maxima"][b]["reason"]} for b in BASES},
        })
        if m["role"] == "reducer_body":
            out["members"][-1]["note"] = ("reference-section (S270) rows of the mapped equivalent chord pipe under DEC-045; "
                                          "component_body attribution; not a cone/taper/body stress")
    return out


ROW_SEMANTICS = {
    "units": "SI: N, N*m, m, rad, Pa. Product element stress rows are MPa (compare after exact conversion x 1e6); membrane, Lame and maximum rows are Pa",
    "section_action_j_side_local": "action exerted ON the from-side part BY the to-side part across the cut, element-local frame (x = (x_to - x_from)/L, y from y_reference, z = x cross y); end_i equals MINUS the from-end action on the member, end_j equals the to-end action on the member. Product interior local force/moment rows and stress recovery use this j-side convention; endpoint local force/moment rows are node-on-member end-DOF actions and bind to end_wrench_on_member_local instead. Axial is the wall force N_w (N_m = N_w - 2 nu p A_i and S = N_w - p A_i are constant along the member)",
    "stations": "end_i (0), quarter_1 (1/4), midspan (1/2), quarter_3 (3/4), end_j (1) of the member length from the from node",
    "axial_membrane_Pa": "N_w / A_b (pipe_axial_membrane_stress_v2 on a pressurized pressure-2 member; element_local_axial_normal_stress outside every pressure region, where N_w = N); equals the cleared axial_membrane_stress_Pa",
    "bending_normal_y_Pa": "element_local_bending_normal_stress_y = M_y / Z_b, signed; tension at the +z outer fibre when positive",
    "bending_normal_z_Pa": "element_local_bending_normal_stress_z = M_z / Z_b, signed; tension at the -y outer fibre when positive",
    "outer_fibre_normal_stress": "sigma_x(y, z) = sigma_N + sigma_y z / r_o - sigma_z y / r_o on the outer surface (= N/A + M_y z/I - M_z y/I)",
    "torsional_shear_Pa": "element_local_torsional_shear_stress = T r_o / J_b, signed with T (right-hand about +x on the j-side face)",
    "normal_extreme_plus_Pa": "element_local_normal_stress_extreme sigma+ = sigma_N + hypot(sigma_y, sigma_z) at the outer fibre r_o(-sigma_z, sigma_y)/hypot (fibre not emitted here); a signed normal-stress envelope, not a combined, Tresca, equivalent or code stress",
    "normal_extreme_minus_Pa": "element_local_normal_stress_extreme sigma- = sigma_N - hypot(sigma_y, sigma_z) at the opposite fibre",
    "transverse_shear_maximum_Pa": "element_local_transverse_shear_stress_maximum = |V| ((3+2nu) r_o^2 + (1+2nu) r_i,b^2) / (4 (1+nu) I_b), nonnegative Saint-Venant maximum on the inner surface at the fibres normal to V; torsional shear separate",
    "max_abs_normal_Pa": "|sigma_N| + hypot(sigma_y, sigma_z) at the station (informational per station)",
    "member_maximum_abs_normal_Pa": "max over the whole member of |N_w|/A_b + hypot(M_y, M_z)/Z_b (DEL-05-03 §2.1 member maximum; pipe_elastic_normal_stress_maximum_v2 on ordinary exact straight members, including per-member body rows; component_body members are excluded from case maxima and headlines); a normal-stress maximum only, torsional and transverse shear separate",
    "maximum_proof": "No interior load acts on any member (caps and the Poisson pair are end actions), so N_w, V and T are constant and M_perp(xi) = M_perp(0) + xi (M_perp(1) - M_perp(0)) is affine. xi -> |M_perp(xi)| is a norm of an affine map, hence convex, and |N_w|/A_b is constant, so the maximum over [0, 1] is attained at an endpoint. The governing endpoint is decided by the exact sign of |M_perp(0)|^2 - |M_perp(1)|^2 in the case field; equality makes both endpoints govern, and a constant M_perp makes every station tie. No sampling is used. The same convexity gives max sigma+ and min sigma- at an endpoint; T and |V| are constant.",
    "exact_text": "exact is given for field values; hypot-type rows (sigma+-, transverse shear maximum, maxima) are (a + c sqrt(q)) pi^k with q = M_y^2 + M_z^2 or V_y^2 + V_z^2 of the same station; their exact key is omitted; zero scales retain exact expressions with the entire sum grouped before pi^k; their binary64 is decided from rigorous rational enclosures (70 to 560 digits) only when both endpoints round alike",
}


def package_layer(out):
    solved = _STATE["solved"]
    chk = Checks()
    fm_cache = {}
    rec_all = {}
    scales_all = {}
    # cleared cases: recompute FM (cheap) for the independent section-action route
    for i, (lab, (c, F, K)) in enumerate(solved.items()):
        assert out["cases"][i]["label"] == lab
        FM = force_method(F, c)
        rec = {m["key"]: member_recovery(F, c, K, m, c["power"], FM, chk) for m in c["members"]}
        rows = binding_rows(c, F, K, rec)
        sc, rule, L_ref = zero_scales(c, F, K, rec, rows)
        rec_all[lab], scales_all[lab] = (rec, rows), sc
        out["cases"][i]["section_recovery"] = publish_recovery(c, K, rec)
        out["cases"][i]["zero_scales"] = publish_scales(sc, rule)
    # C3 analogue
    cj = junction_held_case()
    Fj, Kj, fms = run_junction_case(cj, chk)
    pj = publish(cj, Fj, Kj)
    recj = {m["key"]: member_recovery(Fj, cj, Kj, m, cj["power"], fms[m["key"]], chk) for m in cj["members"]}
    rowsj = binding_rows(cj, Fj, Kj, recj)
    scj, rulej, _ = zero_scales(cj, Fj, Kj, recj, rowsj)
    pj["package_addition"] = True
    pj["section_recovery"] = publish_recovery(cj, Kj, recj)
    pj["zero_scales"] = publish_scales(scj, rulej)
    out["cases"].append(pj)
    # determinate controls of the new rows
    for sh in ("EB", "TIMO"):
        for key in ("nbrN", "body", "nbrF"):
            a = rec_all[f"RS2B-DET-COMP-{sh}-S1"][0][key]
            b = rec_all[f"RS2B-DET-COMP-{sh}-S2"][0][key]
            chk.ok(f"DET-COMP-{sh}: {key} section actions independent of body scale (determinate)",
                   all(eqv(x["action"], y["action"]) for x, y in zip(a["stations"], b["stations"])))
    for s in (1, 2):
        chk.ok(f"DET-COMP S{s}: EB and Timoshenko section actions identical (determinate)",
               all(eqv(x["action"], y["action"]) for key in ("nbrN", "body", "nbrF")
                   for x, y in zip(rec_all[f"RS2B-DET-COMP-EB-S{s}"][0][key]["stations"], rec_all[f"RS2B-DET-COMP-TIMO-S{s}"][0][key]["stations"])))
    # DET-COMP closed form: the eccentric step wrench reaches A through nbrN; A is nbrN's to node, so its
    # j-side action at end_j is the to-end action on the member; node-A equilibrium (R + cap_A - f_to = 0,
    # nbrN's own cap at A being -p A_N s_into = +p A_N x) makes it the anchor reaction plus that cap
    for sh in ("EB", "TIMO"):
        l = f"RS2B-DET-COMP-{sh}-S1"
        c0, F0_, K0 = solved[l]
        st = rec_all[l][0]["nbrN"]["stations"][-1]["action"]
        g = K0["geo"]["nbrN"]
        R = K0["R"]["A"]
        mNb = next(x for x in c0["members"] if x["key"] == "nbrN")
        loc = [dot(R[:3], g["x"]) + pai(mNb), dot(R[:3], g["y"]), dot(R[:3], g["z"]), dot(R[3:], g["x"]), dot(R[3:], g["y"]), dot(R[3:], g["z"])]
        chk.ok(f"{l}: nbrN j-side action at A = anchor reaction + nbrN cap at A (local; node equilibrium)", eqv(st, loc))
        mF = next(x for x in c0["members"] if x["key"] == "nbrF")
        st0 = rec_all[l][0]["nbrN"]["stations"][0]["action"]
        chk.ok(f"{l}: nbrN carries no torsion and a constant |M_perp| = p A_F * 0.025 m (the step moment; nbrN is along the run)",
               st[3].is_zero() and st0[3].is_zero() and st[4] * st[4] + st[5] * st[5] == (pai(mF) * Fr(1, 40)) ** 2
               and st0[4] * st0[4] + st0[5] * st0[5] == (pai(mF) * Fr(1, 40)) ** 2)
    # rotation covariance and body reversal of the new rows
    for base, rot in (("RS2B-HELD-TIMO-S2", "RS2B-ROT-HELD-TIMO-S2"), ("RS2B-MECH-TIMO-S2", "RS2B-ROT-MECH-TIMO-S2")):
        Fb = solved[rot][1]
        ok = True
        for key in ("nbrN", "body", "nbrF"):
            for x, y in zip(rec_all[base][0][key]["stations"], rec_all[rot][0][key]["stations"]):
                ok &= eqv(y["action"], [QF(Fb, list(v.c)) for v in x["action"]])
        chk.ok(f"{rot}: j-side local section actions identical to base (frame objectivity)", ok)
    for sh in ("EB", "TIMO"):
        a = rec_all[f"RS2B-HELD-{sh}-S2"][0]["body"]["stations"]
        b = rec_all[f"RS2B-HELD-REV-{sh}-S2"][0]["body"]["stations"]
        # reversed member: x' = -x, y' = y (y_reference (0,0,1) is normal to both), z' = -z. Its j-side action at
        # xi' is the action on the other part at the same cut, xi = 1 - xi': S' = -S, with primed components
        # (-S.x', -S.y', -S.z') = (S.x, -S.y, S.z): [N, -V_y, V_z, T, -M_y, M_z].
        ok = True
        for x, y in zip(a, reversed(b)):
            Sx = x["action"]
            exp = [Sx[0], -Sx[1], Sx[2], Sx[3], -Sx[4], Sx[5]]
            ok &= eqv(y["action"], exp)
        chk.ok(f"REV-{sh}: body section actions of the reversed member = reflected actions at mirrored stations", ok)
        ok = all(vcmp(rec_all[f"RS2B-HELD-{sh}-S2"][0]["body"]["maxima"][bb]["value"], rec_all[f"RS2B-HELD-REV-{sh}-S2"][0]["body"]["maxima"][bb]["value"]) == 0 for bb in BASES)
        chk.ok(f"REV-{sh}: body member maxima invariant to orientation (both bases)", ok)
    # nonzero bending in the eccentric/out-of-plane cases (the rows are exercised, not vacuous)
    for l in ("RS2B-HELD-TIMO-S2", "RS2B-NONPAR-HELD-TIMO-S2", "RS2B-MECH-TIMO-S2", "RS2B-HELD-UNEQ-TIMO-S2", "RS2B-DET-COMP-TIMO-S1"):
        r = rec_all[l][0]
        chk.ok(f"{l}: body carries nonzero bending (both y and z somewhere) and the eccentric rows are exercised",
               any(not st["action"][4].is_zero() for st in r["body"]["stations"]) or any(not st["action"][5].is_zero() for st in r["body"]["stations"]))
    for l in ("RS2B-MECH-TIMO-S2", "RS2B-NONPAR-HELD-TIMO-S2"):
        r = rec_all[l][0]
        chk.ok(f"{l}: torsion and both bending components nonzero on some member (out-of-plane coverage)",
               any(not st["action"][3].is_zero() for k in r for st in r[k]["stations"])
               and any(not st["action"][4].is_zero() for k in r for st in r[k]["stations"])
               and any(not st["action"][5].is_zero() for k in r for st in r[k]["stations"]))
    # tolerance sensitivity of the cleared mutants
    intended = {
        "drop_body_cap_at_F": ("reaction_force", "member_force", "normal_stress"),
        "extra_pAe_moment_at_N (double count)": ("reaction_moment", "member_moment", "normal_stress"),
        "poisson_pair_scaled_by_s": ("member_force", "translation", "normal_stress"),
        "caps_on_nominal_bore": ("reaction_force", "member_force", "normal_stress"),
        "kappa_5_6_instead_of_kappa_E": ("translation", "rotation", "reaction_moment", "member_moment"),
        "body_caps_along_run_axis_not_chord (averaged-axis substitution)": ("reaction_moment", "member_moment", "normal_stress"),
        "nbrF_cap_projected_to_declared_offset_point (e = 22 mm, 3 mm residual accepted)": ("reaction_moment", "member_moment", "normal_stress"),
        "nbrN_cap_along_chosen_axis_not_actual_direction": ("reaction_force", "reaction_moment", "member_force", "member_moment", "normal_stress"),
    }
    sens = {}
    assert set(intended) == set(MUTANT_SOLUTIONS)
    for name, (ref, cm, Fm, Km) in MUTANT_SOLUTIONS.items():
        c0, F0_, K0 = solved[ref]
        rec0, rows0 = rec_all[ref]
        sc0 = scales_all[ref]
        assert Fm.b == F0_.b
        Kc = {"u": {n: [QF(F0_, list(v.c)) for v in Km["u"][n]] for n in Km["u"]},
              "R": {a: [QF(F0_, list(v.c)) for v in Km["R"][a]] for a in Km["R"]},
              "members": {k: {kk: ([QF(F0_, list(v.c)) for v in vv] if isinstance(vv, list) else QF(F0_, list(vv.c))) for kk, vv in mv.items()} for k, mv in Km["members"].items()},
              "geo": K0["geo"]}
        recm = {m["key"]: member_recovery(F0_, cm, Kc, m, cm["power"]) for m in cm["members"]}
        rowsm = binding_rows(cm, F0_, Kc, recm)
        assert [r[0] for r in rowsm] == [r[0] for r in rows0]
        fam_count, fam_max, listed, total_fail, undecided_rows = {}, {}, [], 0, 0
        for (path, fam, e), (_, _, o) in zip(rows0, rowsm):
            lo, hi = tolerance_ratio(e, o, sc0[fam])
            if lo > 1:
                total_fail += 1
                fam_count[fam[0]] = fam_count.get(fam[0], 0) + 1
                fam_max[fam[0]] = max(fam_max.get(fam[0], Fr(0)), lo)
                listed.append((lo, path, fam[0]))
            elif hi > 1:
                undecided_rows += 1
        listed.sort(key=lambda t: -t[0])
        best_intended = max((fam_max.get(f, Fr(0)) for f in intended[name]), default=Fr(0))
        chk.ok(f"mutant tolerance sensitivity: {name} fails the 1e-9 zero-scale criterion by >= 1e3 tolerances in an intended family",
               best_intended >= 1000)
        chk.ok(f"mutant tolerance sensitivity: {name} every row decided (inside or outside tolerance)", undecided_rows == 0)
        sens[name] = {"against": ref, "rows_compared": len(rows0), "rows_outside_tolerance": total_fail,
                      "rows_outside_tolerance_by_family": dict(sorted(fam_count.items())),
                      "largest_separation_tolerances_by_family": {k: float(v) for k, v in sorted(fam_max.items())},
                      "intended_families": list(intended[name]), "largest_intended_separation_tolerances": float(best_intended),
                      "strongest_rows": [{"path": p, "family": f, "separation_tolerances_lower_bound": float(lo)} for lo, p, f in listed[:6]]}
    # undecided emissions in the package layer
    undecided = []

    def walk(o, path):
        if isinstance(o, dict):
            if "binary64" in o and o["binary64"] is None:
                undecided.append(path)
            for k, v in o.items():
                walk(v, path + "/" + str(k))
        elif isinstance(o, list):
            for i, v in enumerate(o):
                walk(v, f"{path}[{i}]")
    for cs in out["cases"]:
        for key in ("section_recovery", "zero_scales"):
            walk(cs[key], cs["label"] + "/" + key)
        if cs.get("package_addition"):
            walk(cs["results"], cs["label"] + "/results")
    lame_scales = {}
    for nm, blk in out["lame_rows_Pa"].items():
        lame_scales[nm] = {b: {"unit": "Pa", "rule": "max(|p|, |hoop inner|, |hoop outer|, |radial|) = hoop inner of the basis", "value": blk[b]["hoop_inner_Pa"],
                               "binary64": blk[b]["hoop_inner_binary64"]} for b in BASES}
    failed = [i for i in chk.items if not i["pass"]]
    out["package"] = {
        "package_id": PACKAGE_ID,
        "status": "CANDIDATE ONLY: not adopted, not frozen, not product evidence; additions need focused independent delta scrutiny and manager integration",
        "row_semantics": ROW_SEMANTICS,
        "criterion": {
            "rule": "a row passes when |observed - expected| <= 1e-9 * max(|expected|, zero_scale of its family and scope)",
            "families": {
                "translation": "m; per case: largest |u_x|, |u_y|, |u_z| over unanchored nodes",
                "rotation": "rad; per case: largest |r_x|, |r_y|, |r_z| over unanchored nodes; all zero -> translation scale / L_ref, L_ref = longest member chord of the case",
                "reaction_force": "N; per case: largest reaction force component; all zero -> largest member force scale of the case",
                "reaction_moment": "N*m; per case: largest reaction moment component; all zero -> reaction force scale * L_ref",
                "member_force": "N; per member: largest of its end-wrench force components (local and global), N_w, N_m, S and station N_w, V_y, V_z",
                "member_moment": "N*m; per member: largest of its end-wrench moment components (local and global) and station T, M_y, M_z; all zero -> member force scale * member chord length",
                "normal_stress": "Pa; per member and basis: largest |row| of sigma_N, sigma_y, sigma_z, sigma+, sigma- and the member maximum (= the member maximum); all zero -> member force scale / A_b",
                "shear_stress": "Pa; per member and basis: largest of |tau_T| and the transverse shear maximum; all zero -> member force scale / A_b",
                "pressure_stress": "Pa; per section, pressure and basis: the Lame hoop inner value (>= p); see lame_zero_scales_Pa",
            },
            "independence": "every scale is a function of independent expected values and declared inputs (lengths, areas) only; none is chosen from product output or observed error; units never mix",
            "same_as_2a": "the governing 2a product tests use the same 1e-9 relative criterion; their zero-scale fallbacks are not changed by this package",
        },
        "lame_zero_scales_Pa": lame_scales,
        "mutant_tolerance_sensitivity": sens,
        "geometry_scope": {
            "carried": "the 24 cleared W-15c controls above (geometry_admission_controls_w15c), unchanged, as a SUBSET of admission behaviour; not a complete pressure-admission proof",
            "reviewer_confirmed_boundaries": {"agreement_dx_star_m": "0.0065837258", "placement_allowance_mm": "5.26699", "transversality_a_e_mm": "0.9077", "precision_h_star_mm": "0.86747"},
            "left_to_governed_W15c_product_tests": ["beta_arc evidence", "full J3 signed-zero and precedence", "J2 discrimination", "split ties", "not_solver_consumed", "not_reducer", "nonfinite inputs", "mapping invalid", "connectors and joints"],
            "not_claimed": "the Decimal judge here is a simplified re-derivation for these controls, not a product implementation; absent offset no_check is only the existing OFFSET check state, neither a pressure admission nor a pressure refusal; legacy 2a concentric back-to-back bodies are not W-15c run evidence",
        },
        "c3_analogue": {"label": cj["label"], "formulations": "K on the full chain; FM reused unchanged on the two determinate branches leaving the held junction"},
        "emission": {"binary64_undecided_paths": undecided, "float_emission_ready": not undecided},
        "checks": {"count": len(chk.items), "failed": failed, "all_pass": not failed},
    }
    return out


# ---------------------------------------------------------------------------- command line

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parent
REFERENCE = ROOT / "references.json"
SUMS = ROOT / "SHA256SUMS"
SUMMED = ("generators/rt_a3_2b_reference.py", "references.json", "README.md")


def build_package():
    out = build()
    return package_layer(out)


def render(out):
    return json.dumps(out, indent=1, ensure_ascii=False) + "\n"


def sha(b):
    return hashlib.sha256(b).hexdigest()


def main():
    ap = argparse.ArgumentParser(description="RT-A3 reducer 2b reference package (candidate)")
    mode = ap.add_mutually_exclusive_group()
    mode.add_argument("--check", action="store_true", help="recompute and compare with the saved bytes (default); writes nothing")
    mode.add_argument("--write", action="store_true", help="intentionally rewrite references.json and SHA256SUMS")
    mode.add_argument("--summary", action="store_true", help="recompute and print counts only")
    a = ap.parse_args()
    out = build_package()
    txt = render(out).encode("utf-8")
    core, pkg = out["checks"], out["package"]["checks"]
    und = out["emission"]["binary64_undecided_paths"] + out["package"]["emission"]["binary64_undecided_paths"]
    print(f"{len(out['cases'])} cases; cleared checks {core['count']} (failed {len(core['failed'])}); "
          f"package checks {pkg['count']} (failed {len(pkg['failed'])}); binary64 undecided {len(und)}")
    for f in (core["failed"] + pkg["failed"])[:40]:
        print("FAIL", f)
    bad = bool(core["failed"] or pkg["failed"] or und)
    if a.summary:
        for k, v in out["package"]["mutant_tolerance_sensitivity"].items():
            print("MUT", k, v["rows_outside_tolerance"], "/", v["rows_compared"], "largest intended", f"{v['largest_intended_separation_tolerances']:.3e}")
        return 1 if bad else 0
    if a.write:
        if bad:
            print("refusing to write: failed checks or undecided values")
            return 1
        REFERENCE.write_bytes(txt)
        lines = []
        for rel in SUMMED:
            p = ROOT / rel
            if p.exists():
                lines.append(f"{sha(p.read_bytes())}  {rel}\n")
        SUMS.write_text("".join(lines), encoding="utf-8")
        print(f"wrote references.json sha256 {sha(txt)} and SHA256SUMS")
        return 0
    # --check (default)
    stale = []
    if not REFERENCE.exists() or REFERENCE.read_bytes() != txt:
        stale.append("references.json differs from the recomputed bytes")
    if not SUMS.exists():
        stale.append("SHA256SUMS missing")
    else:
        listed = {}
        for line in SUMS.read_text(encoding="utf-8").splitlines():
            h, rel = line.split("  ", 1)
            listed[rel] = h
        for rel in SUMMED:
            p = ROOT / rel
            if rel not in listed:
                stale.append(f"SHA256SUMS lacks {rel}")
            elif not p.exists() or sha(p.read_bytes()) != listed[rel]:
                stale.append(f"SHA256SUMS mismatch for {rel}")
    for s in stale:
        print("STALE", s)
    if bad or stale:
        return 1
    print(f"check passed: references.json sha256 {sha(txt)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
