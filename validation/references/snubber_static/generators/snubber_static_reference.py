#!/usr/bin/env python3
"""Snubbers in static load cases: absent from the solve, disclosed, results unchanged.

Independent analytical references. A snubber is a restraint in dynamic analysis
only (DEL-04-10/REQ-005, AC-003; DEL-04-07/CLM-001): every static load-case
solve leaves it out and discloses it, so every static result of a model with
snubbers equals that of the same model without them. This package gives the
closed-form static results of two invented base models (one linear, one with a
one-way support), the snubber variants whose static results must equal them,
the expected disclosure, and, as labelled negative controls, the results the
models would have if a snubber acted statically as a rigid restraint or as a
spring of its stiffness.

Independence. Python 3 standard library only (fractions, json, hashlib,
argparse, pathlib, sys). Reads, imports and runs no product code, product
output, fixture or schema. All inputs are invented. The models carry no
pressure.

Two formulations, sharing only the inputs, the section formula and the one-way
state law:
  A. Closed form: cantilever influence functions (deflection, slope and axial
     stretch of a cantilever under a point force) with every support force as
     a redundant lambda*d along its direction d, solved from compatibility.
  B. Direct stiffness: planar Euler-Bernoulli frame elements, springs added as
     k d d^T/(d^T d), rigid restraints (axis or skew) as exact constraint rows
     d^T u = 0 with their multipliers, solved by Gauss-Jordan elimination.
Numbers are exact elements of Q(pi) (ratios of polynomials in pi with rational
coefficients), because a snubber stiffness k is combined with EI and EA, which
carry pi. A and B must agree exactly on every published value, every one-way
state must be the unique admissible one, and every equilibrium and
distinguishability check must hold, or nothing is written and the exit status
is 1.

Usage:  python3 -I -B snubber_static_reference.py [--check]  # default: compare frozen bytes
        python3 -I -B snubber_static_reference.py --write    # rewrite references.json and SHA256SUMS
"""

from fractions import Fraction as Fr
import argparse
import hashlib
import json
import sys
from pathlib import Path


class CheckFailure(Exception):
    pass


# ---------------------------------------------------------------------------
# Polynomials in pi over Q, and the field Q(pi) of their ratios
# ---------------------------------------------------------------------------
def p_trim(a):
    a = list(a)
    while a and a[-1] == 0:
        a.pop()
    return a


def p_add(a, b):
    n = max(len(a), len(b))
    return p_trim([(a[i] if i < len(a) else 0) + (b[i] if i < len(b) else 0) for i in range(n)])


def p_neg(a):
    return [-x for x in a]


def p_mul(a, b):
    if not a or not b:
        return []
    out = [Fr(0)] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            out[i + j] += x * y
    return p_trim(out)


def p_divmod(a, b):
    a, q = list(a), [Fr(0)] * max(len(a) - len(b) + 1, 1)
    while len(a) >= len(b) and a:
        c = a[-1] / b[-1]
        k = len(a) - len(b)
        q[k] = c
        for i, y in enumerate(b):
            a[i + k] -= c * y
        a = p_trim(a)
    return p_trim(q), a


def p_gcd(a, b):
    while b:
        a, b = b, p_divmod(a, b)[1]
    return [x / a[-1] for x in a]


class Q:
    """An element of Q(pi): num/den, coprime, den monic."""
    __slots__ = ("n", "d")

    def __init__(self, n, d=None):
        if isinstance(n, (int, Fr)):
            n = [Fr(n)]
        n = p_trim([Fr(x) for x in n])
        d = p_trim([Fr(x) for x in (d if d is not None else [1])])
        if not d:
            raise CheckFailure("division by zero in Q(pi)")
        if not n:
            self.n, self.d = [], [Fr(1)]
            return
        g = p_gcd(n, d)
        if len(g) > 1:
            n, d = p_divmod(n, g)[0], p_divmod(d, g)[0]
        lead = d[-1]
        self.n, self.d = [x / lead for x in n], [x / lead for x in d]

    @staticmethod
    def of(x):
        return x if isinstance(x, Q) else Q(x)

    def __add__(self, o):
        o = Q.of(o)
        return Q(p_add(p_mul(self.n, o.d), p_mul(o.n, self.d)), p_mul(self.d, o.d))
    __radd__ = __add__

    def __neg__(self):
        return Q(p_neg(self.n), self.d)

    def __sub__(self, o):
        return self + (-Q.of(o))

    def __rsub__(self, o):
        return Q.of(o) - self

    def __mul__(self, o):
        o = Q.of(o)
        return Q(p_mul(self.n, o.n), p_mul(self.d, o.d))
    __rmul__ = __mul__

    def __truediv__(self, o):
        o = Q.of(o)
        if not o.n:
            raise CheckFailure("division by zero in Q(pi)")
        return Q(p_mul(self.n, o.d), p_mul(self.d, o.n))

    def __rtruediv__(self, o):
        return Q.of(o) / self

    def __eq__(self, o):
        o = Q.of(o)
        return self.n == o.n and self.d == o.d

    def __hash__(self):
        return hash((tuple(self.n), tuple(self.d)))

    def is_zero(self):
        return not self.n

    def interval(self):
        a, b = p_interval(self.n), p_interval(self.d)
        if b[0] <= 0 <= b[1]:
            raise CheckFailure("denominator interval contains 0")
        c = [a[0] / b[0], a[0] / b[1], a[1] / b[0], a[1] / b[1]]
        return min(c), max(c)

    def sign(self):
        if self.is_zero():
            return 0
        lo, hi = self.interval()
        if lo > 0:
            return 1
        if hi < 0:
            return -1
        raise CheckFailure("sign not decided by the pi enclosure")

    def __gt__(self, o):
        return (self - o).sign() > 0

    def __lt__(self, o):
        return (self - o).sign() < 0

    def __abs__(self):
        return -self if self.sign() < 0 else self


# pi enclosure by Machin's formula with alternating-series bounds
def _atan_inv(n, terms=90):
    s, sums = Fr(0), []
    for k in range(terms + 1):
        s += Fr((-1) ** k, (2 * k + 1) * n ** (2 * k + 1))
        sums.append(s)
    return sorted(sums[-2:])


_a, _b = _atan_inv(5), _atan_inv(239)
PI_LO, PI_HI = 16 * _a[0] - 4 * _b[1], 16 * _a[1] - 4 * _b[0]
PI = Q([0, 1])


def p_interval(a):
    lo = hi = Fr(0)
    for k, c in enumerate(a):
        x, y = c * PI_LO ** k, c * PI_HI ** k
        lo, hi = lo + min(x, y), hi + max(x, y)
    return lo, hi


def fr_text(q):
    return str(q.numerator) if q.denominator == 1 else f"{q.numerator}/{q.denominator}"


def p_text(a):
    terms = []
    for k, c in enumerate(a):
        if c == 0:
            continue
        power = "pi" if k == 1 else f"pi^{k}"
        if k == 0:
            terms.append(fr_text(c))
        elif c in (1, -1):
            terms.append(("-" if c < 0 else "") + power)
        else:
            terms.append(f"{fr_text(c)}*{power}")
    return " + ".join(terms).replace("+ -", "- ") or "0"


def num(x, unit):
    x = Q.of(x)
    if x.is_zero():
        return {"exact": "0", "value": 0.0, "unit": unit}
    lo, hi = x.interval()
    if float(lo) != float(hi):
        raise CheckFailure("pi enclosure too wide to round")
    text = p_text(x.n) if x.d == [1] else f"({p_text(x.n)})/({p_text(x.d)})"
    return {"exact": text, "value": float(lo), "unit": unit}


# ---------------------------------------------------------------------------
# Inputs (invented; every one exactly representable in binary64)
# ---------------------------------------------------------------------------
E = Fr(200_000_000_000)
D, T_WALL = Fr(1, 2), Fr(1, 32)
LEN = Fr(4)                       # A(0) -> M(2) -> N(3) -> S(4) along X
P_N = Fr(20000)                   # magnitude of the vertical load at N
Q_S = Fr(50000)                   # axial load at S, +X


def section():
    d = D - 2 * T_WALL
    Ah, Ih = (D * D - d * d) / 4, (D ** 4 - d ** 4) / 64
    return Ah, Ih, Q([0, E * Ah]), Q([0, E * Ih])    # EA = E*Ah*pi, EI = E*Ih*pi


AH, IH, EA, EI = section()
X_OF = {"A": Fr(0), "M": Fr(2), "N": Fr(3), "S": LEN}

# Snubber records: id, node, direction d (X, Z components; not normalized), stiffness (None = rigid)
SNUBBERS = {
    "SNB-1": {"node": "S", "d": (Fr(1), Fr(0)), "k": None, "label": "axis UX at S, rigid"},
    "SNB-2": {"node": "M", "d": (Fr(0), Fr(1)), "k": Fr(200_000_000), "label": "axis UZ at M, k = 2e8 N/m"},
    "SNB-3": {"node": "M", "d": (Fr(1), Fr(2)), "k": None, "label": "skew d = (1, 0, 2) at M, rigid"},
    "SNB-4": {"node": "S", "d": (Fr(2), Fr(1)), "k": Fr(1_000_000_000), "label": "skew d = (2, 0, 1) at S, k = 1e9 N/m"},
}
VARIANTS = [("V1", ["SNB-1"]), ("V2", ["SNB-2"]), ("V3", ["SNB-3"]), ("V4", ["SNB-4"]),
            ("V-ALL", ["SNB-1", "SNB-2", "SNB-3", "SNB-4"])]
DISCLOSURE = "dynamic-only restraint, not active in static cases"

# Base cases: model, loads {(node, comp): value}
CASES = [
    ("SL-1", "linear", "Linear model: anchor at A, vertical support (two-way rigid UZ) at S. Load -20000 N Z at N and +50000 N X at S.",
     {("N", "FZ"): -P_N, ("S", "FX"): Q_S}),
    ("SN-BEAR", "nonlinear", "Nonlinear model: anchor at A, one-way support at S on UZ, sense + (bears upward). Same loads as SL-1: the support bears.",
     {("N", "FZ"): -P_N, ("S", "FX"): Q_S}),
    ("SN-LIFT", "nonlinear", "Nonlinear model with the load at N reversed (+20000 N Z) and +50000 N X at S: the support lifts off.",
     {("N", "FZ"): P_N, ("S", "FX"): Q_S}),
]


# ----------------------------- formulation A --------------------------------
def a_influence(xp, xf, Fx, Fz):
    """Displacements (UX, UZ, RY) at xp of a cantilever rooted at x = 0 under
    a force (Fx, Fz) at xf. RY = -dw/dx (positive RY turns Z toward X)."""
    if xp <= xf:
        w = Fz * xp * xp * (3 * xf - xp) / 6
        slope = Fz * xp * (2 * xf - xp) / 2
    else:
        w = Fz * xf * xf * (3 * xp - xf) / 6
        slope = Fz * xf * xf / 2
    return (Q.of(Fx * min(xp, xf)) / EA, Q.of(w) / EI, -Q.of(slope) / EI)


def a_disp(xp, forces):
    ux = uz = ry = Q(0)
    for xf, Fx, Fz in forces:
        a, b, c = a_influence(xp, xf, Fx, Fz)
        ux, uz, ry = ux + a, uz + b, ry + c
    return ux, uz, ry


def a_solve(loads, restraints):
    """restraints: list of (name, node, (dx, dz), k or None). Support force on the
    pipe = lam*d. Rigid: d.u = 0. Spring: lam*(d.d) = -k*(d.u). Returns lam per restraint."""
    applied = []
    for (node, comp), v in loads.items():
        applied.append((X_OF[node], v if comp == "FX" else Fr(0), v if comp == "FZ" else Fr(0)))
    n = len(restraints)
    rows, rhs = [], []
    for name, node, (dx, dz), k in restraints:
        x0 = X_OF[node]
        u0 = a_disp(x0, applied)
        base = u0[0] * dx + u0[1] * dz
        row = []
        for _, node_j, (ex, ez), _ in restraints:
            uj = a_disp(x0, [(X_OF[node_j], ex, ez)])
            row.append(uj[0] * dx + uj[1] * dz)
        if k is None:
            rows.append(row); rhs.append(-base)
        else:
            dd = dx * dx + dz * dz
            row = [r * k for r in row]
            row[len(rows)] = row[len(rows)] + dd
            rows.append(row); rhs.append(-base * k)
    lam = gauss_jordan(rows, rhs) if n else []
    forces = applied + [(X_OF[node], l * dx, l * dz) for l, (_, node, (dx, dz), _) in zip(lam, restraints)]
    return dict(zip([r[0] for r in restraints], lam)), forces


def a_results(loads, restraints):
    lam, forces = a_solve(loads, restraints)
    disp = {nd: dict(zip(("UX", "UZ", "RY"), a_disp(X_OF[nd], forces))) for nd in ORDER[1:]}
    FX = sum((Q.of(f[1]) for f in forces), Q(0))
    FZ = sum((Q.of(f[2]) for f in forces), Q(0))
    MY = sum((Q.of(-f[0] * f[2]) for f in forces), Q(0))   # (r x F)_y = z Fx - x Fz, z = 0
    reac = {"anchor A": {"FX": -FX, "FZ": -FZ, "MY": -MY}}
    for name, node, (dx, dz), k in restraints:
        reac[name] = {"FX": lam[name] * dx, "FZ": lam[name] * dz}
    # end actions by statics: member A-M carries everything beyond A; M-S everything beyond M
    ends = {}
    for m, (n1, n2) in MEMBERS.items():
        x1, x2 = X_OF[n1], X_OF[n2]
        fs = [f for f in forces if f[0] >= x2]     # everything at or beyond the member's far end
        fx = sum((Q.of(f[1]) for f in fs), Q(0)); fz = sum((Q.of(f[2]) for f in fs), Q(0))
        m2 = sum((Q.of(-(f[0] - x2) * f[2]) for f in fs), Q(0))
        m1 = sum((Q.of(-(f[0] - x1) * f[2]) for f in fs), Q(0))
        ends[m] = {n1: {"FX": -fx, "FZ": -fz, "MY": -m1}, n2: {"FX": fx, "FZ": fz, "MY": m2}}
    return disp, reac, ends, lam


# ----------------------------- formulation B --------------------------------
def gauss_jordan(A, b):
    n = len(A)
    M = [[Q.of(x) for x in row] + [Q.of(bi)] for row, bi in zip(A, b)]
    for c in range(n):
        p = next((r for r in range(c, n) if not M[r][c].is_zero()), None)
        if p is None:
            raise CheckFailure("singular system")
        M[c], M[p] = M[p], M[c]
        pv = M[c][c]
        M[c] = [x / pv for x in M[c]]
        for r in range(n):
            if r != c and not M[r][c].is_zero():
                f = M[r][c]
                M[r] = [x - f * y for x, y in zip(M[r], M[c])]
    return [M[r][n] for r in range(n)]


def b_element(Lm):
    """Global 6x6 (UX1, UZ1, RY1, UX2, UZ2, RY2) of a member along +X, built in
    the standard (x, y) = (X, Z) plane with theta = -RY."""
    ea, b12, b6, b4, b2 = EA / Lm, EI * 12 / Lm ** 3, EI * 6 / Lm ** 2, EI * 4 / Lm, EI * 2 / Lm
    z = Q(0)
    k = [[z] * 6 for _ in range(6)]
    k[0][0] = k[3][3] = ea; k[0][3] = k[3][0] = -ea
    k[1][1] = k[4][4] = b12; k[1][4] = k[4][1] = -b12
    k[1][2] = k[2][1] = k[1][5] = k[5][1] = b6
    k[4][2] = k[2][4] = k[4][5] = k[5][4] = -b6
    k[2][2] = k[5][5] = b4; k[2][5] = k[5][2] = b2
    sgn = [1, 1, -1, 1, 1, -1]
    return [[k[i][j] * (sgn[i] * sgn[j]) for j in range(6)] for i in range(6)]


ORDER = ["A", "M", "N", "S"]
DOF = {(n, c): 3 * i + j for i, n in enumerate(ORDER) for j, c in enumerate(("UX", "UZ", "RY"))}
MEMBERS = {"A-M": ("A", "M"), "M-N": ("M", "N"), "N-S": ("N", "S")}


def b_results(loads, restraints):
    ND = 3 * len(ORDER)
    K = [[Q(0)] * ND for _ in range(ND)]
    ke = {}
    for m, (n1, n2) in MEMBERS.items():
        k = b_element(X_OF[n2] - X_OF[n1])
        ke[m] = k
        idx = [DOF[(n1, c)] for c in ("UX", "UZ", "RY")] + [DOF[(n2, c)] for c in ("UX", "UZ", "RY")]
        for i in range(6):
            for j in range(6):
                K[idx[i]][idx[j]] = K[idx[i]][idx[j]] + k[i][j]
    Ks = [row[:] for row in K]      # with springs
    rigid = []
    for name, node, (dx, dz), k in restraints:
        ix, iz = DOF[(node, "UX")], DOF[(node, "UZ")]
        if k is None:
            rigid.append((name, ix, iz, dx, dz))
        else:
            dd = dx * dx + dz * dz
            for (i, a) in ((ix, dx), (iz, dz)):
                for (j, b) in ((ix, dx), (iz, dz)):
                    Ks[i][j] = Ks[i][j] + Q.of(k * a * b / dd)
    f = [Fr(0)] * ND
    for (node, comp), v in loads.items():
        f[DOF[(node, comp.replace("F", "U"))]] += v
    free = [DOF[(n, c)] for n in ORDER[1:] for c in ("UX", "UZ", "RY")]
    nf = len(free)
    nr = len(rigid)
    rows, rhs = [], []
    for gi in free:
        row = [Ks[gi][gj] for gj in free] + [Q(0)] * nr
        for r, (_, ix, iz, dx, dz) in enumerate(rigid):
            if gi == ix:
                row[nf + r] = Q.of(-dx)
            if gi == iz:
                row[nf + r] = Q.of(-dz)
        rows.append(row); rhs.append(f[gi])
    for _, ix, iz, dx, dz in rigid:
        row = [Q(0)] * (nf + nr)
        row[free.index(ix)] = Q.of(dx); row[free.index(iz)] = Q.of(dz)
        rows.append(row); rhs.append(Fr(0))
    x = gauss_jordan(rows, rhs)
    u = [Q(0)] * ND
    for i, gi in enumerate(free):
        u[gi] = x[i]
    lam = {}
    for r, (name, *_rest) in enumerate(rigid):
        lam[name] = x[nf + r]
    for name, node, (dx, dz), k in restraints:
        if k is not None:
            dd = dx * dx + dz * dz
            lam[name] = -(u[DOF[(node, "UX")]] * dx + u[DOF[(node, "UZ")]] * dz) * Q.of(k / dd)
    disp = {nd: {c: u[DOF[(nd, c)]] for c in ("UX", "UZ", "RY")} for nd in ORDER[1:]}
    rA = {c: sum((K[DOF[("A", c)]][j] * u[j] for j in range(ND)), Q(0)) for c in ("UX", "UZ", "RY")}
    reac = {"anchor A": {"FX": rA["UX"], "FZ": rA["UZ"], "MY": rA["RY"]}}
    for name, node, (dx, dz), k in restraints:
        reac[name] = {"FX": lam[name] * dx, "FZ": lam[name] * dz}
    ends = {}
    for m, (n1, n2) in MEMBERS.items():
        ue = [u[DOF[(n, c)]] for n in (n1, n2) for c in ("UX", "UZ", "RY")]
        fe = [sum((ke[m][i][j] * ue[j] for j in range(6)), Q(0)) for i in range(6)]
        ends[m] = {n1: {"FX": fe[0], "FZ": fe[1], "MY": fe[2]}, n2: {"FX": fe[3], "FZ": fe[4], "MY": fe[5]}}
    return disp, reac, ends, lam


# ---------------------------------------------------------------------------
def support_set(model, oneway_active, snubbers_acting):
    """snubbers_acting: list of (snubber id, 'rigid' | 'spring')."""
    rs = []
    if model == "linear" or oneway_active:
        rs.append(("support S", "S", (Fr(0), Fr(1)), None))
    for sid, how in snubbers_acting:
        s = SNUBBERS[sid]
        rs.append((sid, s["node"], s["d"], None if how == "rigid" else s["k"]))
    return rs


def solve_case(chk, tag, model, loads, snubbers_acting):
    """Solve A and B, determine the one-way state by enumeration; return the published block."""
    states = [True] if model == "linear" else [True, False]
    found = []
    for active in states:
        rs = support_set(model, active, snubbers_acting)
        dA, rA, eA, lA = a_results(loads, rs)
        dB, rB, eB, lB = b_results(loads, rs)
        chk.eq("A = B: displacements", f"{tag} active={active}", dA, dB)
        chk.eq("A = B: reactions", f"{tag} active={active}", rA, rB)
        chk.eq("A = B: member end actions", f"{tag} active={active}", eA, eB)
        if model == "linear":
            found.append((None, dA, rA, eA))
            continue
        if active:
            ok = rA["support S"]["FZ"] > 0                      # bears
        else:
            ok = dA["S"]["UZ"] > 0                              # separated
        if ok:
            found.append(("active" if active else "inactive", dA, rA, eA))
    chk.eq("exactly one admissible one-way state", tag, len(found), 1)
    state, d, r, e = found[0]
    if state == "inactive":
        r = dict(r); r["support S"] = {"FX": Q(0), "FZ": Q(0)}       # an inactive one-way support publishes 0
    # equilibrium
    fx = sum((Q.of(v) for (n, c), v in loads.items() if c == "FX"), Q(0))
    fz = sum((Q.of(v) for (n, c), v in loads.items() if c == "FZ"), Q(0))
    my = sum((Q.of(-X_OF[n] * v) for (n, c), v in loads.items() if c == "FZ"), Q(0))
    for name, rv in r.items():
        fx, fz = fx + rv["FX"], fz + rv["FZ"]
        if name == "anchor A":
            my = my + rv["MY"]
        else:
            node = "S" if name == "support S" else SNUBBERS[name]["node"]
            my = my - rv["FZ"] * X_OF[node]
    chk.eq("global equilibrium", tag, (fx, fz, my), (Q(0), Q(0), Q(0)))
    for m, (n1, n2) in MEMBERS.items():
        a, b = e[m][n1], e[m][n2]
        mom = a["MY"] + b["MY"] - (X_OF[n2] - X_OF[n1]) * b["FZ"]
        chk.eq("member equilibrium", f"{tag} {m}", (a["FX"] + b["FX"], a["FZ"] + b["FZ"], mom), (Q(0), Q(0), Q(0)))
    return state, d, r, e


def publish(state, d, r, e):
    out = {}
    if state is not None:
        out["one-way support S state"] = state
    out["displacements"] = {n: {c: num(v, "rad" if c == "RY" else "m") for c, v in dv.items()} for n, dv in d.items()}
    out["reactions"] = {s: {c: num(v, "N*m" if c == "MY" else "N") for c, v in rv.items()} for s, rv in r.items()}
    out["member_end_actions"] = {m: {n: {c: num(v, "N*m" if c == "MY" else "N") for c, v in nv.items()} for n, nv in mv.items()} for m, mv in e.items()}
    return out


def max_rel_diff(base, other):
    """The original combined measure, kept for its frozen values and now exact in
    Q(pi): largest |other - base| / max|base| within each group (displacements
    with rotations; anchor reactions; end actions), maximized over the groups.
    It mixes translations with rotations and forces with moments inside a
    group; separation_by_kind below is the dimensionally consistent measure."""
    worst = Q(0)
    for kind in ("displacements", "anchor", "ends"):
        if kind == "displacements":
            pairs = [(base[1][n][c], other[1][n][c]) for n in ORDER[1:] for c in ("UX", "UZ", "RY")]
        elif kind == "anchor":
            pairs = [(base[2]["anchor A"][c], other[2]["anchor A"][c]) for c in ("FX", "FZ", "MY")]
        else:
            pairs = [(base[3][m][n][c], other[3][m][n][c]) for m, (n1, n2) in MEMBERS.items() for n in (n1, n2) for c in ("FX", "FZ", "MY")]
        for pb, po in pairs:
            if pb.is_zero() and po.is_zero():
                continue
            scale = max((abs(x) for x, _ in pairs if not x.is_zero()), default=Q(1))
            r = abs(po - pb) / scale
            if r > worst:
                worst = r
    return worst


KINDS = ("translations", "rotations", "forces", "moments")


def kind_pairs(base, other, kind):
    if kind == "translations":
        return [(base[1][n][c], other[1][n][c]) for n in ORDER[1:] for c in ("UX", "UZ")]
    if kind == "rotations":
        return [(base[1][n]["RY"], other[1][n]["RY"]) for n in ORDER[1:]]
    comps = ("FX", "FZ") if kind == "forces" else ("MY",)
    return ([(base[2]["anchor A"][c], other[2]["anchor A"][c]) for c in comps]
            + [(base[3][m][n][c], other[3][m][n][c]) for m, (n1, n2) in MEMBERS.items() for n in (n1, n2) for c in comps])


def separation_by_kind(base, other):
    """Per kind (translations UX, UZ at M, N, S; rotations RY at M, N, S; forces FX, FZ
    of the anchor and every member end; moments MY of the anchor and every member
    end): the largest |other - base| over the kind, divided by the largest |base|
    of the same kind. Exact in Q(pi)."""
    out = {}
    for kind in KINDS:
        pairs = kind_pairs(base, other, kind)
        scale = Q(0)
        for pb, _ in pairs:
            if abs(pb) > scale:
                scale = abs(pb)
        if scale.is_zero():
            raise CheckFailure(f"base magnitude of kind {kind} is zero")
        worst = Q(0)
        for pb, po in pairs:
            r = abs(po - pb) / scale
            if r > worst:
                worst = r
        out[kind] = worst
    return out


def build():
    chk = Checks()
    cases = []
    smallest = {k: None for k in KINDS}
    smallest_any = None
    for cid, model, desc, loads in CASES:
        base = solve_case(chk, f"{cid} base", model, loads, [])
        # closed-form spot checks of the base results (textbook propped cantilever, free cantilever)
        Lq = LEN
        if base[0] in (None, "active"):
            a_, b_ = X_OF["N"], LEN - X_OF["N"]
            chk.eq("propped cantilever: R_S = P a^2 (3L - a)/(2L^3)", cid, base[2]["support S"]["FZ"], Q(P_N * a_ ** 2 * (3 * Lq - a_) / (2 * Lq ** 3)))
            chk.eq("propped cantilever: anchor MY = -P a b (L + b)/(2L^2)", cid, base[2]["anchor A"]["MY"], Q(-P_N * a_ * b_ * (Lq + b_) / (2 * Lq ** 2)))
        else:
            a_ = X_OF["N"]
            chk.eq("free cantilever: u_z(S) = P a^2 (3L - a)/(6EI)", cid, base[1]["S"]["UZ"], Q(P_N * a_ ** 2 * (3 * Lq - a_) / 6) / EI)
        chk.eq("axial: u_x(S) = QL/EA", cid, base[1]["S"]["UX"], Q(Q_S * Lq) / EA)
        variants = []
        for vid, sids in VARIANTS:
            # the static solve leaves every snubber out: the identity is the base result itself
            ident = solve_case(chk, f"{cid} {vid} static (snubbers left out)", model, loads, [])
            chk.eq("static result with snubbers = base result", f"{cid} {vid}", ident[1:], base[1:])
            controls = []
            plans = []
            if len(sids) == 1:
                s = SNUBBERS[sids[0]]
                plans.append(("as rigid restraint", [(sids[0], "rigid")]))
                if s["k"] is not None:
                    plans.append(("as spring of its stiffness", [(sids[0], "spring")]))
            else:
                plans.append(("each as authored (rigid ones rigid, finite ones springs)",
                              [(sid, "rigid" if SNUBBERS[sid]["k"] is None else "spring") for sid in sids]))
            for how, acting in plans:
                ctl = solve_case(chk, f"{cid} {vid} control {how}", model, loads, acting)
                diff = max_rel_diff(base, ctl)
                sep = separation_by_kind(base, ctl)
                best = Q(0)
                for k in KINDS:
                    if sep[k] > best:
                        best = sep[k]
                    if smallest[k] is None or smallest[k] > sep[k]:
                        smallest[k] = sep[k]
                if smallest_any is None or smallest_any > best:
                    smallest_any = best
                chk.true("negative control distinguishable (in some kind, relative difference > 5e-2)", f"{cid} {vid} {how}", best > Fr(5, 100))
                rec = {"label": "NEGATIVE CONTROL: a wrong static answer", "snubbers act": how,
                       "largest relative difference from the base result (displacements, anchor reactions, end actions; each kind scaled by its largest base magnitude)": num(diff, "1"),
                       "separation by kind (largest change of the kind / largest base magnitude of the same kind)": {k: num(sep[k], "1") for k in KINDS}}
                rec.update(publish(*ctl))
                if ctl[0] is not None and ctl[0] != base[0]:
                    rec["note"] = f"the one-way support changes state: {base[0]} -> {ctl[0]}"
                controls.append(rec)
            variants.append({
                "id": vid,
                "snubbers": [{"id": sid, **{k: v for k, v in SNUBBERS[sid].items() if k == "label"}} for sid in sids],
                "expected static result": f"numerically identical to the base result of {cid}: every displacement, rotation, reaction and member end action unchanged; the result documents are not byte-identical, because the disclosures differ",
                "expected disclosure": {"snubbers named": sids, "severity": "non-blocking", "wording": DISCLOSURE,
                                        "open presentation choices (settled by the product's design)": [
                                            "whether a snubber has no reaction row or a reaction row of zero in a static result",
                                            "whether each snubber has its own finding or one finding names several snubbers"]},
                "negative_controls": controls,
            })
        rec = {"id": cid, "model": model, "description": desc,
               "loads": [{"node": n, "component": c, "value": num(v, "N")} for (n, c), v in loads.items()]}
        rec["base result (static; also the expected result of every variant)"] = publish(*base)
        rec["variants"] = variants
        cases.append(rec)

    by_kind = {}
    for k in chk.items:
        by_kind[k] = by_kind.get(k, 0) + 1
    return {
        "schema": "swbpipe.validation_reference/snubber_static/1",
        "status": "frozen analytical reference; no product comparison, acceptance or release is asserted",
        "independence": "Python standard library only; no product code, product output, fixture or schema read, imported or run",
        "governing_text": ["execution/PKG-04/DEL-04-10/ScopeOfWork.md (REQ-005, AC-003, REQ-007)",
                           "execution/PKG-04/DEL-04-07/ScopeOfWork.md (CLM-001)",
                           "execution/PKG-04/DEL-04-03/ScopeOfWork.md (REQ-010: skew stiffness k d d^T/(d^T d); rigid skew d^T u = 0)"],
        "model": {
            "plane": "X-Z, Z up; every out-of-plane displacement, reaction and end action is exactly 0",
            "nodes": {"A": [0, 0, 0], "M": [2, 0, 0], "N": [3, 0, 0], "S": [4, 0, 0]},
            "members": {"A-M": ["A", "M"], "M-N": ["M", "N"], "N-S": ["N", "S"]},
            "supports": {"anchor A": "all six DOF",
                         "support S (linear model)": "rigid two-way restraint on UZ",
                         "support S (nonlinear model)": "one-way on UZ, sense + (bears upward); ground at u_z = 0"},
            "section": {"D": num(D, "m"), "t": num(T_WALL, "m"), "corrosion allowance": num(0, "m"), "mill tolerance": num(0, "1"),
                        "A": {"exact": f"({fr_text(AH)})*pi", "unit": "m^2"}, "I": {"exact": f"({fr_text(IH)})*pi", "unit": "m^4"},
                        "basis": "nominal and reduced bases coincide (zero allowance and tolerance)"},
            "material": {"E": num(E, "Pa")},
            "shear_model": "euler_bernoulli",
            "pressure": "none: the models carry no pressure; the snubber's static absence does not depend on it",
            "self_weight": "none; nodal forces only",
        },
        "snubbers": {sid: {"node": s["node"], "direction": [num(s["d"][0], "1"), num(0, "1"), num(s["d"][1], "1")],
                           "stiffness": "rigid" if s["k"] is None else num(s["k"], "N/m"), "label": s["label"]}
                     for sid, s in SNUBBERS.items()},
        "conventions": {
            "reaction": "support-on-pipe force in global axes; a skew restraint's force is lambda*d along its authored d",
            "end action": "force or moment the node exerts on the member at that end, global axes; MY about +Y (Z toward X positive)",
            "exact form": "elements of Q(pi): rational functions of pi with rational coefficients",
            "value": "nearest binary64 of the exact value",
        },
        "tolerance_class": "Identity (AC-003 'unchanged'): numerically identical static results; see the README sections 'What a product must give' and 'Tolerance class'. Base values: linear model at the analytic tolerance (DEL-04-03/AC-001: 1e-9 relative with its governed floor); nonlinear model, one-way state exactly and values within the nonlinear class tolerance (DEL-04-07/AC-002). This package chooses no number.",
        "cases": cases,
        "control separation summary": {
            "smallest separation per kind over all controls": {k: num(smallest[k], "1") for k in KINDS},
            "smallest over all controls of the largest separation of any kind": num(smallest_any, "1"),
        },
        "checks": {"count": len(chk.items), "all_pass": True, "by_kind": dict(sorted(by_kind.items()))},
    }


class Checks:
    def __init__(self):
        self.items = []

    def eq(self, kind, name, x, y):
        if x != y:
            raise CheckFailure(f"{kind}: {name}")
        self.items.append(kind)

    def true(self, kind, name, cond):
        if not cond:
            raise CheckFailure(f"{kind}: {name}")
        self.items.append(kind)


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--write", action="store_true", help="rewrite references.json and SHA256SUMS")
    mode.add_argument("--check", action="store_true", help="compare frozen bytes (the default)")
    args = parser.parse_args()
    try:
        out = build()
    except CheckFailure as exc:
        print(f"check failed: {exc}", file=sys.stderr)
        return 1
    root = Path(__file__).resolve().parents[1]
    rendered = (json.dumps(out, indent=1, ensure_ascii=False) + "\n").encode("utf-8")
    files = {
        "README.md": (root / "README.md").read_bytes(),
        "generators/snubber_static_reference.py": Path(__file__).read_bytes(),
        "references.json": rendered,
    }
    sums = "".join(f"{hashlib.sha256(d).hexdigest()}  {n}\n" for n, d in files.items()).encode("ascii")
    outputs = {"references.json": rendered, "SHA256SUMS": sums}
    if args.write:
        for name, data in outputs.items():
            (root / name).write_bytes(data)
    else:
        diff = [n for n, d in outputs.items() if not (root / n).is_file() or (root / n).read_bytes() != d]
        if diff:
            print("reference bytes differ: " + ", ".join(diff), file=sys.stderr)
            return 1
    print(f"{len(out['cases'])} cases, {out['checks']['count']} exact checks, frozen bytes {'written' if args.write else 'match'}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
