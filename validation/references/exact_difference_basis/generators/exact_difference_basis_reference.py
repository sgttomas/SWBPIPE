"""Independent exact references for the declared difference basis on the exact
pressure route: a result-state subtraction declared "operating_minus_sustained"
(DEL-05-02 REQ-012, AC-003, AC-012), with its gates G-1 and G-2.

Python standard library only. It reads, imports and runs no product code and
no product output. Every input is invented and is a dyadic rational, so it is
its own binary64. Arithmetic is exact: a value is a finite sum of rational
multiples of integer powers of pi (class Pi below), and each published value
is the binary64 nearest to it, decided from rational bounds on pi about
1e-160 apart.

Methods (both must agree exactly on every row, or nothing is written):
  1. direct stiffness: 12 x 12 Euler-Bernoulli frame elements, exact
     Gauss-Jordan elimination, member free strains as held end actions;
  2. force method on the same chain: release the far anchor, virtual work
     with Simpson's rule (exact for the quadratic integrands of nodal loads),
     solve the 6 x 6 compatibility for the redundant, the rest by statics.
A third, hand-formula route (README "M1 by hand") re-derives the M1 thermal
redundants, and a wall-force form of DEL-04-07's pressure loads (caps at every
terminal and Poisson shortening) is checked against the closed-end free strain.

Run ``python3 -I generators/exact_difference_basis_reference.py`` from the
package directory to rebuild every output in memory and compare it byte for
byte with the committed files; ``--write`` rewrites them.
"""
from __future__ import annotations

import hashlib
import json
import sys
from collections import Counter
from fractions import Fraction as Q
from math import isqrt
from pathlib import Path

HERE = Path(__file__).resolve().parent.parent
SELF = Path(__file__).resolve()
ROW_TEXT = "operating minus sustained difference"


# --------------------------------------------------------------------------
# Exact numbers: sums of q * pi^k
# --------------------------------------------------------------------------

class Pi:
    __slots__ = ("t",)

    def __init__(self, terms=None):
        self.t = {}
        for k, v in (terms or {}).items():
            v = Q(v)
            if v:
                self.t[k] = v

    def __add__(self, o):
        o = lift(o)
        r = dict(self.t)
        for k, v in o.t.items():
            r[k] = r.get(k, 0) + v
        return Pi(r)

    __radd__ = __add__

    def __neg__(self):
        return Pi({k: -v for k, v in self.t.items()})

    def __sub__(self, o):
        return self + (-lift(o))

    def __rsub__(self, o):
        return lift(o) - self

    def __mul__(self, o):
        o = lift(o)
        r = {}
        for a, x in self.t.items():
            for b, y in o.t.items():
                r[a + b] = r.get(a + b, 0) + x * y
        return Pi(r)

    __rmul__ = __mul__

    def __truediv__(self, o):
        o = lift(o)
        assert len(o.t) == 1, "division by a monomial only"
        ((k, q),) = o.t.items()
        return Pi({a - k: v / q for a, v in self.t.items()})

    def __eq__(self, o):
        return self.t == lift(o).t

    def __hash__(self):
        return hash(tuple(sorted(self.t.items())))

    def zero(self):
        return not self.t

    def text(self):
        if not self.t:
            return "0"
        out = []
        for k in sorted(self.t):
            q = self.t[k]
            out.append(str(q) if k == 0 else f"{q}*pi" if k == 1 else f"{q}*pi^{k}")
        return " + ".join(out)


def lift(x):
    return x if isinstance(x, Pi) else Pi({0: Q(x)})


PI = Pi({1: 1})
ZERO = Pi()


def pi_bounds(digits=160):
    """Machin: pi = 16 atan(1/5) - 4 atan(1/239), fixed point with a proven error."""
    scale = 10 ** (digits + 10)

    def atan_inv(x):
        total, k, err = 0, 0, 0
        while True:
            term = scale // ((2 * k + 1) * x ** (2 * k + 1))
            if term == 0:
                return total, err + 1  # each floor < 1 unit; the alternating tail < 1 unit
            total += term if k % 2 == 0 else -term
            err += 1
            k += 1

    a5, e5 = atan_inv(5)
    a239, e239 = atan_inv(239)
    mid, err = 16 * a5 - 4 * a239, 16 * e5 + 4 * e239
    return Q(mid - err, scale), Q(mid + err, scale)


PI_LO, PI_HI = pi_bounds()
assert Q(314159265358979, 10 ** 14) < PI_LO < PI_HI < Q(314159265358980, 10 ** 14)


def bounds(v: Pi):
    lo = hi = Q(0)
    for k, q in v.t.items():
        a, b = (PI_LO ** k, PI_HI ** k) if k >= 0 else (PI_HI ** k, PI_LO ** k)
        if q > 0:
            lo, hi = lo + q * a, hi + q * b
        else:
            lo, hi = lo + q * b, hi + q * a
    return lo, hi


def f64(v: Pi) -> float:
    if v.zero():
        return 0.0
    lo, hi = bounds(v)
    a, b = float(lo), float(hi)
    assert a == b, f"pi bounds too wide to round {v.text()}"
    return a


def sqrt_bounds(lo: Q, hi: Q, bits=420):
    s = 4 ** bits
    a = isqrt(lo.numerator * s // lo.denominator)
    b = isqrt(-(-hi.numerator * s // hi.denominator)) + 1
    return Q(a, 2 ** bits), Q(b, 2 ** bits)


def norm(vec):
    """(exact square, nearest binary64 of its square root)."""
    sq = ZERO
    for c in vec:
        sq = sq + c * c
    if sq.zero():
        return sq, 0.0
    lo, hi = sqrt_bounds(*bounds(sq))
    a, b = float(lo), float(hi)
    assert a == b
    return sq, a


def sqrt_difference(sq_a: Pi, sq_b: Pi) -> float:
    """Nearest binary64 of sqrt(a) - sqrt(b): the subtracted magnitude the rule forbids."""
    if sq_a == sq_b:
        return 0.0
    la, ha = sqrt_bounds(*bounds(sq_a)) if not sq_a.zero() else (Q(0), Q(0))
    lb, hb = sqrt_bounds(*bounds(sq_b)) if not sq_b.zero() else (Q(0), Q(0))
    a, b = float(la - hb), float(ha - lb)
    assert a == b
    return a


# --------------------------------------------------------------------------
# Vectors and exact linear algebra
# --------------------------------------------------------------------------

def vec(*xs):
    return tuple(lift(x) for x in xs)


V0 = vec(0, 0, 0)


def vadd(a, b):
    return tuple(x + y for x, y in zip(a, b))


def vsub(a, b):
    return tuple(x - y for x, y in zip(a, b))


def vscale(s, a):
    return tuple(lift(s) * x for x in a)


def dot(a, b):
    r = ZERO
    for x, y in zip(a, b):
        r = r + lift(x) * lift(y)
    return r


def cross(a, b):
    a, b = [lift(x) for x in a], [lift(x) for x in b]
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def solve_rational(m, rhs_columns):
    """Gauss-Jordan on a rational matrix with several right-hand sides."""
    n = len(m)
    a = [list(row) + [col[i] for col in rhs_columns] for i, row in enumerate(m)]
    for c in range(n):
        p = next(r for r in range(c, n) if a[r][c] != 0)
        a[c], a[p] = a[p], a[c]
        piv = a[c][c]
        a[c] = [x / piv for x in a[c]]
        for r in range(n):
            if r != c and a[r][c] != 0:
                f = a[r][c]
                a[r] = [x - f * y for x, y in zip(a[r], a[c])]
    return [[a[i][n + j] for i in range(n)] for j in range(len(rhs_columns))]


def solve_pi(mprime, shift, rhs):
    """Solve (pi^shift * M') x = rhs, M' rational, rhs a list of Pi values."""
    powers = sorted({k for v in rhs for k in v.t})
    cols = [[v.t.get(k, Q(0)) for v in rhs] for k in powers]
    sols = solve_rational(mprime, cols) if cols else []
    x = [ZERO] * len(rhs)
    for k, sol in zip(powers, sols):
        x = [xi + Pi({k - shift: si}) for xi, si in zip(x, sol)]
    return x


# --------------------------------------------------------------------------
# Inputs (all invented; every one a dyadic rational, so exactly a binary64)
# --------------------------------------------------------------------------

NU = Q(9, 32)                         # poisson_ratio, the member's selected value
POINTS = [                            # temperature points: T (K), E (Pa), G (Pa), alpha (1/K)
    (Q(300), Q(194750000000), Q(76000000000), Q(3, 2 ** 18)),
    (Q(700), Q(184500000000), Q(72000000000), Q(3, 2 ** 18)),
]
D, WALL, C_M, C_A = Q(35, 128), Q(9, 1024), Q(1, 2048), Q(1, 1024)   # m
DT = Q(475, 2)                        # K, thermal primitive on every member of an operating case
P_LO, P_HI = Q(5750000), Q(6500000)  # Pa
REF = ("ref", Q(300))                 # exact basis reference to the 300 K point
HOT = ("temperature", Q(600))         # solve temperature, interpolated between 300 K and 700 K

for _T, _E, _G, _a in POINTS:
    assert _E / (2 * _G) - 1 == NU, "E, G and nu agree at every point (README: readings removed)"


def moduli(basis):
    kind, temp = basis
    if kind == "ref":
        (pt,) = [p for p in POINTS if p[0] == temp]
        return pt[1], pt[2], pt[3]
    lo = max(p for p in POINTS if p[0] < temp)
    hi = min(p for p in POINTS if p[0] > temp)
    f = (temp - lo[0]) / (hi[0] - lo[0])
    return tuple(a + f * (b - a) for a, b in zip(lo[1:], hi[1:]))


R_O = D / 2
R_IN = R_O - WALL                     # nominal section: stiffness
T_R = (WALL - C_M) - C_A
R_IR = R_O - T_R                      # pressure bore
A_N = R_O ** 2 - R_IN ** 2            # A_n = pi * A_N
I_N = (R_O ** 4 - R_IN ** 4) / 4      # I = pi * I_N
J_N = 2 * I_N
A_I = R_IR ** 2                       # A_i = pi * A_I


def eps_pressure(p, e_mod):
    """Closed-end free pressure strain (1 - 2 nu) p A_i / (E A_s), A_s the nominal area."""
    return (1 - 2 * NU) * p * A_I / (e_mod * A_N)


# --------------------------------------------------------------------------
# Models: chains of straight members, one region per straight run
# --------------------------------------------------------------------------

Z_UP, X_REF = (0, 0, 1), (1, 0, 0)
MODELS = {
    "M0": {
        "title": "determinate L: A anchored, B corner, C free closed end",
        "nodes": {"A": (0, 0, 0), "B": (Q(15, 4), 0, 0), "C": (Q(15, 4), Q(9, 4), 0)},
        "anchors": ["A"],
        "members": [("AB", "A", "B", Z_UP), ("BC", "B", "C", Z_UP)],
        "regions": [["AB"], ["BC"]],
        "weights": {"C": (0, 0, Q(-1225, 2))},
    },
    "M1": {
        "title": "indeterminate L: A and C anchored, corner B a junction of two regions",
        "nodes": {"A": (0, 0, 0), "B": (Q(21, 4), 0, 0), "C": (Q(21, 4), Q(27, 8), 0)},
        "anchors": ["A", "C"],
        "members": [("AB", "A", "B", Z_UP), ("BC", "B", "C", Z_UP)],
        "regions": [["AB"], ["BC"]],
        "weights": {"B": (0, 0, Q(-7375, 4))},
    },
    "M2": {
        "title": "indeterminate 3D Z: A and D anchored, junctions B and C, region 1 split at E",
        "nodes": {"A": (0, 0, 0), "E": (Q(5, 2), 0, 0), "B": (Q(19, 4), 0, 0),
                  "C": (Q(19, 4), Q(21, 8), 0), "D": (Q(19, 4), Q(21, 8), Q(-25, 8))},
        "anchors": ["A", "D"],
        "members": [("AE", "A", "E", Z_UP), ("EB", "E", "B", Z_UP), ("BC", "B", "C", Z_UP),
                    ("CD", "C", "D", X_REF)],
        "regions": [["AE", "EB"], ["BC"], ["CD"]],
        "weights": {"E": (0, 0, Q(-2501, 2)), "C": (0, 0, Q(-3625, 4))},
    },
}


def axes(model, member):
    _, a, b, yref = member
    pa, pb = vec(*model["nodes"][a]), vec(*model["nodes"][b])
    d = vsub(pb, pa)
    length2 = dot(d, d).t[0]
    length = Q(isqrt(length2.numerator), isqrt(length2.denominator))
    assert length * length == length2, "axis-aligned members have rational length"
    t = vscale(1 / length, d)
    yr = vec(*yref)
    y = vsub(yr, vscale(dot(yr, t), t))
    ny2 = dot(y, y).t[0]
    ny = Q(isqrt(ny2.numerator), isqrt(ny2.denominator))
    assert ny * ny == ny2
    y = vscale(1 / ny, y)
    return (t, y, cross(t, y)), length


def terminals(model):
    """(node, member, s) for every region terminal; s is the unit vector from the node into the member."""
    mem = {m[0]: m for m in model["members"]}
    out = []
    for region in model["regions"]:
        first, last = mem[region[0]], mem[region[-1]]
        out.append((first[1], first[0], axes(model, first)[0][0]))
        out.append((last[2], last[0], vscale(-1, axes(model, last)[0][0])))
    return out


def junctions(model):
    count = Counter(n for n, _, _ in terminals(model))
    return sorted(n for n, c in count.items() if c >= 2)


def check_region_constraint(model):
    """Within a region, members join only where diameter, wall and direction are identical."""
    mem = {m[0]: m for m in model["members"]}
    for region in model["regions"]:
        for a, b in zip(region, region[1:]):
            assert mem[a][2] == mem[b][1]
            assert axes(model, mem[a])[0][0] == axes(model, mem[b])[0][0]  # one section everywhere


# --------------------------------------------------------------------------
# Load sets. A case is (model, basis, pressure, elongation on/off, thermal on/off).
#   reading "W": pressure enters as DEL-04-07's loads, which on these models equal
#                the closed-end free strain; "off" omits that strain (README).
#   reading "J": alternative: "off" applies the junction resultant -sum p A_i s as an
#                external nodal load, and "on" adds the free strain to it.
#   form "R4":   DEL-04-07's wall form (caps at every terminal, Poisson shortening);
#                used only to check reading W's "on" side.
# --------------------------------------------------------------------------

def load_set(model_id, case, reading="W"):
    model = MODELS[model_id]
    e_mod, _g, alpha = moduli(case["basis"])
    nodal = {n: [V0, V0] for n in model["nodes"]}
    for n, w in model["weights"].items():
        nodal[n][0] = vadd(nodal[n][0], vec(*w))
    eig = {m[0]: ZERO for m in model["members"]}
    p = case["p"]
    for m in model["members"]:
        if case["thermal"]:
            eig[m[0]] = eig[m[0]] + alpha * DT
        if p and reading in ("W", "J") and case["on"]:
            eig[m[0]] = eig[m[0]] + eps_pressure(p, e_mod)
        if p and reading == "R4":
            assert case["on"]
            eig[m[0]] = eig[m[0]] - 2 * NU * p * A_I / (e_mod * A_N)
    if p and reading == "R4":
        for n, _m, s in terminals(model):
            nodal[n][0] = vadd(nodal[n][0], vscale(-p * A_I * PI, s))
    if p and reading == "J":
        js = set(junctions(model))
        for n, _m, s in terminals(model):
            if n in js:
                nodal[n][0] = vadd(nodal[n][0], vscale(-p * A_I * PI, s))
    return {"nodal": {n: tuple(v) for n, v in nodal.items()}, "eig": eig}


def load_difference(a, b):
    return {"nodal": {n: (vsub(a["nodal"][n][0], b["nodal"][n][0]), vsub(a["nodal"][n][1], b["nodal"][n][1]))
                      for n in a["nodal"]},
            "eig": {m: a["eig"][m] - b["eig"][m] for m in a["eig"]}}


COMP_U = ("UX", "UY", "UZ", "RX", "RY", "RZ")
COMP_F = ("FX", "FY", "FZ", "MX", "MY", "MZ")
COMP_L = ("N", "VY", "VZ", "T", "MY", "MZ")


# --------------------------------------------------------------------------
# Method 1: direct stiffness
# --------------------------------------------------------------------------

def local_k(ea, ei, gj, L):
    k = [[Q(0)] * 12 for _ in range(12)]

    def put(i, j, v):
        k[i][j] = v
        k[j][i] = v

    put(0, 0, ea / L); put(6, 6, ea / L); put(0, 6, -ea / L)
    put(3, 3, gj / L); put(9, 9, gj / L); put(3, 9, -gj / L)
    b = ei / L ** 3
    for (v1, r1, v2, r2, sg) in ((1, 5, 7, 11, 1), (2, 4, 8, 10, -1)):
        put(v1, v1, 12 * b); put(v1, r1, sg * 6 * L * b); put(v1, v2, -12 * b); put(v1, r2, sg * 6 * L * b)
        put(r1, r1, 4 * L * L * b); put(r1, v2, -sg * 6 * L * b); put(r1, r2, 2 * L * L * b)
        put(v2, v2, 12 * b); put(v2, r2, -sg * 6 * L * b); put(r2, r2, 4 * L * L * b)
    return k


def method_stiffness(model_id, mods, loads):
    model = MODELS[model_id]
    e_mod, g_mod, _ = mods
    order = list(model["nodes"])
    idx = {n: 6 * i for i, n in enumerate(order)}
    size = 6 * len(order)
    kg_all = [[Q(0)] * size for _ in range(size)]
    f = [ZERO] * size
    elements = []
    for m in model["members"]:
        (t, y, z), L = axes(model, m)
        rot = [[x.t.get(0, Q(0)) for x in r] for r in (t, y, z)]
        tr = [[Q(0)] * 12 for _ in range(12)]
        for blk in range(4):
            for i in range(3):
                for j in range(3):
                    tr[3 * blk + i][3 * blk + j] = rot[i][j]
        kl = local_k(e_mod * A_N, e_mod * I_N, g_mod * J_N, L)
        kg = [[sum(tr[a][i] * kl[a][b] * tr[b][j] for a in range(12) for b in range(12) if tr[a][i] and tr[b][j])
               for j in range(12)] for i in range(12)]
        dofs = [idx[m[1]] + c for c in range(6)] + [idx[m[2]] + c for c in range(6)]
        for i in range(12):
            for j in range(12):
                kg_all[dofs[i]][dofs[j]] += kg[i][j]
        h = loads["eig"][m[0]] * Pi({1: e_mod * A_N})   # E A eps*, held member pushes nodes apart
        fixed = [h * t[c] for c in range(3)] + [ZERO] * 3 + [-h * t[c] for c in range(3)] + [ZERO] * 3
        for i in range(12):
            f[dofs[i]] = f[dofs[i]] - fixed[i]
        elements.append((m, (t, y, z), kg, dofs, fixed))
    for n, (force, moment) in loads["nodal"].items():
        for c in range(3):
            f[idx[n] + c] = f[idx[n] + c] + force[c]
            f[idx[n] + 3 + c] = f[idx[n] + 3 + c] + moment[c]
    free = [d for n in order if n not in model["anchors"] for d in range(idx[n], idx[n] + 6)]
    sub = [[kg_all[i][j] for j in free] for i in free]
    x = solve_pi(sub, 1, [f[i] for i in free])
    u = [ZERO] * size
    for d, v in zip(free, x):
        u[d] = v
    rows = {}
    for n in order:
        for c in range(6):
            rows[f"u/{n}/{COMP_U[c]}"] = u[idx[n] + c]
    react = {n: [ZERO] * 6 for n in model["anchors"]}
    for n in model["anchors"]:
        force, moment = loads["nodal"][n]
        for c in range(3):
            react[n][c] = -force[c]
            react[n][3 + c] = -moment[c]
    for (m, frame, kg, dofs, fixed) in elements:
        fe = [PI * sum((u[dofs[j]] * kg[i][j] for j in range(12) if kg[i][j]), ZERO) + fixed[i] for i in range(12)]
        for end, off, node in (("i", 0, m[1]), ("j", 6, m[2])):
            for c in range(6):
                rows[f"end_force/{m[0]}/{end}/{COMP_F[c]}"] = fe[off + c]
            if node in react:
                react[node] = [r + fe[off + c] for c, r in enumerate(react[node])]
        for station, off, sign in (("0", 0, -1), ("1", 6, 1)):
            F, M = fe[off:off + 3], fe[off + 3:off + 6]
            loc = [dot(F, a) * sign for a in frame] + [dot(M, a) * sign for a in frame]
            for c in range(6):
                rows[f"resultant/{m[0]}/{station}/{COMP_L[c]}"] = loc[c]
        assert rows[f"resultant/{m[0]}/0/N"] == rows[f"resultant/{m[0]}/1/N"]
        rows[f"axial/{m[0]}/S"] = rows[f"resultant/{m[0]}/0/N"]
    for n in model["anchors"]:
        for c in range(6):
            rows[f"reaction/{n}/{COMP_F[c]}"] = react[n][c]
    return rows


# --------------------------------------------------------------------------
# Method 2: force method on the chain, virtual work, Simpson's rule
# --------------------------------------------------------------------------

def method_force(model_id, mods, loads):
    model = MODELS[model_id]
    e_mod, g_mod, _ = mods
    mem = model["members"]
    names = [mem[0][1]] + [m[2] for m in mem]
    assert [m[1] for m in mem] == names[:-1], "chain in order"
    pos = [vec(*model["nodes"][n]) for n in names]
    geo = [axes(model, m) for m in mem]
    ea, ei, gj = Pi({1: e_mod * A_N}), Pi({1: e_mod * I_N}), Pi({1: g_mod * J_N})
    far_anchored = names[-1] in model["anchors"]
    assert names[0] in model["anchors"] and model["anchors"][-1] in (names[0], names[-1])

    def resultant(k, x, P, C, extra):
        F, M = V0, V0
        for j in range(k + 1, len(names)):
            F = vadd(F, P[j])
            M = vadd(M, vadd(cross(vsub(pos[j], x), P[j]), C[j]))
        if extra:
            F = vadd(F, extra[0])
            M = vadd(M, vadd(cross(vsub(pos[-1], x), extra[0]), extra[1]))
        return F, M

    def disp(q, P, C, eig, extra):
        out = [ZERO] * 6
        for k in range(q):
            (t, _y, _z), L = geo[k]
            vals = []
            for s in (Q(0), L / 2, L):
                x = vadd(pos[k], vscale(s, t))
                F, M = resultant(k, x, P, C, extra)
                eps = dot(F, t) / ea + eig[k]
                mt = dot(M, t)
                kappa = vadd(vscale(lift(1) / ei, vsub(M, vscale(mt, t))), vscale(mt / gj, t))
                row = []
                for c in range(3):
                    e = tuple(lift(1 if i == c else 0) for i in range(3))
                    row.append(dot(e, t) * eps + dot(cross(vsub(pos[q], x), e), kappa))
                row += [kappa[c] for c in range(3)]
                vals.append(row)
            for c in range(6):
                out[c] = out[c] + (vals[0][c] + 4 * vals[1][c] + vals[2][c]) * (L / 6)
        return out

    P = [loads["nodal"][n][0] for n in names]
    C = [loads["nodal"][n][1] for n in names]
    eig = [loads["eig"][m[0]] for m in mem]
    zP = [V0] * len(names)
    extra = None
    if far_anchored:
        delta0 = disp(len(names) - 1, P, C, eig, None)
        cols = []
        for i in range(6):
            unit = (tuple(lift(1 if c == i else 0) for c in range(3)), tuple(lift(1 if c == i - 3 else 0) for c in range(3)))
            cols.append(disp(len(names) - 1, zP, zP, [ZERO] * len(mem), unit))
        fprime = []
        for r in range(6):
            row = []
            for c in range(6):
                v = cols[c][r]
                assert set(v.t) <= {-1}, "flexibility is rational times 1/pi"
                row.append(v.t.get(-1, Q(0)))
            fprime.append(row)
        red = solve_pi(fprime, -1, [-d for d in delta0])
        extra = (tuple(red[:3]), tuple(red[3:]))
        assert all(v.zero() for v in disp(len(names) - 1, P, C, eig, extra)), "far anchor compatible"
    rows = {}
    for q, n in enumerate(names):
        d = disp(q, P, C, eig, extra)
        for c in range(6):
            rows[f"u/{n}/{COMP_U[c]}"] = d[c]
    for k, m in enumerate(mem):
        (t, y, z), _L = geo[k]
        Fi, Mi = resultant(k, pos[k], P, C, extra)
        Fj, Mj = resultant(k, pos[k + 1], P, C, extra)
        assert Fi == Fj
        for end, F, M, sg in (("i", Fi, Mi, -1), ("j", Fj, Mj, 1)):
            for c in range(3):
                rows[f"end_force/{m[0]}/{end}/{COMP_F[c]}"] = F[c] * sg
                rows[f"end_force/{m[0]}/{end}/{COMP_F[c + 3]}"] = M[c] * sg
        for station, F, M in (("0", Fi, Mi), ("1", Fj, Mj)):
            loc = [dot(F, a) for a in (t, y, z)] + [dot(M, a) for a in (t, y, z)]
            for c in range(6):
                rows[f"resultant/{m[0]}/{station}/{COMP_L[c]}"] = loc[c]
        rows[f"axial/{m[0]}/S"] = dot(Fi, t)
    # reactions by statics
    totF, totM = V0, V0
    for j in range(len(names)):
        totF = vadd(totF, P[j])
        totM = vadd(totM, vadd(cross(vsub(pos[j], pos[0]), P[j]), C[j]))
    if extra:
        totF = vadd(totF, extra[0])
        totM = vadd(totM, vadd(cross(vsub(pos[-1], pos[0]), extra[0]), extra[1]))
        for c in range(3):
            rows[f"reaction/{names[-1]}/{COMP_F[c]}"] = extra[0][c]
            rows[f"reaction/{names[-1]}/{COMP_F[c + 3]}"] = extra[1][c]
    for c in range(3):
        rows[f"reaction/{names[0]}/{COMP_F[c]}"] = -totF[c]
        rows[f"reaction/{names[0]}/{COMP_F[c + 3]}"] = -totM[c]
    return rows, extra


def add_axial_identities(model_id, rows, p):
    """N_wall = S + p A_i (statics of a closed region); N_mech = S + (1 - 2 nu) p A_i (DEL-05-03)."""
    for m in MODELS[model_id]["members"]:
        s = rows[f"axial/{m[0]}/S"]
        rows[f"axial/{m[0]}/N_wall"] = s + p * A_I * PI
        rows[f"axial/{m[0]}/N_mech"] = s + (1 - 2 * NU) * p * A_I * PI
    return rows


def equilibrium(model_id, rows, loads):
    model = MODELS[model_id]
    F, M = V0, V0
    for n, (force, moment) in loads["nodal"].items():
        r = vec(*model["nodes"][n])
        F, M = vadd(F, force), vadd(M, vadd(cross(r, force), moment))
    for n in model["anchors"]:
        rf = tuple(rows[f"reaction/{n}/{c}"] for c in COMP_F[:3])
        rm = tuple(rows[f"reaction/{n}/{c}"] for c in COMP_F[3:])
        r = vec(*model["nodes"][n])
        F, M = vadd(F, rf), vadd(M, vadd(cross(r, rf), rm))
    return all(v.zero() for v in F + M)


CHECKS = Counter()


def solve_case(model_id, case, reading="W"):
    mods = moduli(case["basis"])
    loads = load_set(model_id, case, reading)
    a = method_stiffness(model_id, mods, loads)
    b, _ = method_force(model_id, mods, loads)
    assert a.keys() == b.keys() and all(a[k] == b[k] for k in a), "methods 1 and 2 agree"
    CHECKS["rows_two_methods"] += len(a)
    assert equilibrium(model_id, a, loads), "global equilibrium of the case"
    CHECKS["case_equilibrium"] += 1
    if reading == "W" and case["p"] and case["on"]:
        w4 = load_set(model_id, case, "R4")
        r4 = method_stiffness(model_id, mods, w4)
        assert equilibrium(model_id, r4, w4)
        for k, v in a.items():
            if k.startswith(("u/", "reaction/")):
                assert r4[k] == v, "wall form = closed-end free strain: motions and reactions"
            if k.endswith("/N") and k.startswith("resultant/"):
                assert r4[k] == v + case["p"] * A_I * PI, "wall form carries N_wall = S + p A_i"
        CHECKS["wall_form_equivalence"] += 1
    return add_axial_identities(model_id, a, case["p"]), loads


# --------------------------------------------------------------------------
# Containment (G-1) and modulus basis (G-2)
# --------------------------------------------------------------------------

def content(prim):
    """Content of a primitive load: everything but its id; the magnitude by its binary64 bits."""
    cat, target, direction, dim, value, unit = prim[1:]
    return (cat, target, direction, dim, float(value).hex(), unit)


def contained(sub, mins):
    s, m = Counter(sub), Counter(mins)
    return all(m[c] >= n for c, n in s.items())


def g1_primitives_only(s_case, o_case):
    """Control (the earlier reading A): primitive loads only."""
    return contained([content(p) for p in s_case["primitives"]], [content(p) for p in o_case["primitives"]])


def applied(case):
    """The decided G-1 rule: everything the case applies, by content and multiplicity.

    Primitive loads by authored content; each pressure region with its pressure (members
    and the pressure's bits); pressure elongation, where the case includes it, as a
    separate action of its own, not part of a region's content; each generated load;
    each prescribed motion; each member's authored state of non-zero free strain,
    compared as authored (kind, expansion law reference, temperature bits and unit).
    A preload present in every case is not among the listed actions.
    """
    out = [("primitive",) + content(p) for p in case["primitives"]]
    for region in case.get("regions", []):
        out.append(("pressure_region", tuple(region[0]), float(region[1]).hex(), "Pa"))
    if case.get("on") and case.get("regions"):
        out.append(("pressure_elongation",))
    for gl in case.get("generated_loads", []):
        out.append(("generated_load",) + tuple(gl))
    for sm in case.get("support_motions", []):
        out.append(("prescribed_motion",) + tuple(sm))
    for ms in case.get("member_states", []):
        member, kind, law, temperature, unit = ms
        out.append(("member_state", member, kind, law, float(temperature).hex(), unit))
    return out


def g2(s_case, o_case):
    return s_case["basis"] == o_case["basis"]


def prim(pid, cat, target, direction, dim, value, unit):
    return (pid, cat, target, direction, dim, Q(value), unit)


def next_up(x: Q) -> Q:
    """The next binary64 above a positive normal value x."""
    e = x.numerator.bit_length() - x.denominator.bit_length()
    while Q(2) ** e > x:
        e -= 1
    while Q(2) ** (e + 1) <= x:
        e += 1
    return x + Q(2) ** (e - 52)


def next_down(x: Q) -> Q:
    """The next binary64 below a negative normal value x (one ulp further from zero)."""
    a = -x
    e = a.numerator.bit_length() - a.denominator.bit_length()
    while Q(2) ** e > a:
        e -= 1
    while Q(2) ** (e + 1) <= a:
        e += 1
    return x - Q(2) ** (e - 52)


def weight_prims(model_id):
    model = MODELS[model_id]
    out = []
    for n, w in model["weights"].items():
        for c, v in zip("xyz", w):
            if v:
                out.append(prim(f"w:{n}:{c}", "weight", f"node:{n}", f"global_{c}", "force", v, "N"))
    return out


def thermal_prims(model_id):
    return [prim(f"t:{m[0]}", "thermal", f"pipe:{m[0]}", "none", "temperature_interval", DT, "K")
            for m in MODELS[model_id]["members"]]


def regions(model_id, p):
    return [(tuple(r), p) for r in MODELS[model_id]["regions"]] if p else []


def g1_rows():
    W = weight_prims("M1")[0]
    Wd = (W[0] + ":dup",) + W[1:]
    T = thermal_prims("M1")
    other = prim("f:B:y", "occasional", "node:B", "global_y", "force", Q(625, 2), "N")
    W2 = W[:5] + (Q(-3687, 2), W[6])
    Wulp = W[:5] + (next_down(W[5]), W[6])
    base = {"basis": REF, "on": True}

    def case(prims, p=P_LO, on=True, **kw):
        c = dict(base, primitives=prims, regions=regions("M1", p), on=on)
        c.update(kw)
        return c

    state = ("pipe:AB", "temperature", "material:temperature_points", Q(600), "K")
    state_ulp = state[:3] + (next_up(Q(600)), "K")
    motion = ("support:C", "UZ", Q(-3, 512), "m")
    preload = ("support:B", "FZ", Q(1500), "N")
    # (id, sustained, operating, expected outcome under the decided rule, reason)
    rows = [
        ("contained", case([W]), case([W] + T), "published",
         "every sustained load and the equal pressure regions and elongation are in the operating case"),
        ("sustained_only_load", case([W, other]), case([W] + T), "refused_g1",
         "the sustained occasional force is absent from the operating case"),
        ("twice_in_sustained_once_in_operating", case([W, Wd]), case([W] + T), "refused_g1",
         "content counted twice in the sustained case, once in the operating case"),
        ("twice_in_both", case([W, Wd]), case([W, Wd] + T), "published",
         "the same content twice in both cases"),
        ("same_target_different_magnitude", case([W]), case([W2] + T), "refused_g1",
         "same target, other magnitude bits: other content"),
        ("same_target_magnitude_one_ulp_apart", case([W]), case([Wulp] + T), "refused_g1",
         "magnitudes one binary64 apart: other content"),
        ("operands_swapped", case([W] + T), case([W]), "refused_g1",
         "the thermal primitives of the minuend-turned-subtrahend are absent"),
        ("no_primitives_in_sustained", case([]), case(T), "published",
         "the sustained case applies only its pressure regions and elongation, both in the operating case"),
        ("fails_g1_and_g2", case([W, other]), case([W] + T, basis=HOT), "refused_g1_and_g2",
         "a sustained-only load, and two modulus bases"),
        ("pressure_differs", case([W], P_LO), case([W] + T, P_HI), "refused_g1",
         "a region with another pressure is other content"),
        ("pressure_absent_in_operating", case([W], P_LO), case([W] + T, 0), "refused_g1",
         "the sustained pressure regions are absent from the operating case"),
        ("elongation_off_sustained_on_operating", case([W], on=False), case([W] + T, on=True), "published",
         "pressure elongation is a separate action, not part of a region's content; the sustained case does "
         "not apply it, so nothing of the sustained case is missing"),
        ("elongation_on_sustained_off_operating", case([W], on=True), case([W] + T, on=False), "refused_g1",
         "the sustained case applies pressure elongation and the operating case does not"),
        ("support_motion_as_primitive_in_sustained_only",
         case([W, prim("d:C:z", "imposed_displacement", "support:C", "global_z", "length", Q(-3, 512), "m")]),
         case([W] + T), "refused_g1", "the sustained imposed-displacement primitive is absent"),
        ("support_motion_as_support_record_in_sustained_only", case([W], support_motions=[motion]),
         case([W] + T), "refused_g1", "a prescribed motion is an action; absent from the operating case"),
        ("member_state_in_sustained_only", case([W], member_states=[state]), case([W] + T), "refused_g1",
         "the authored member state is absent from the operating case"),
        ("member_state_same_authored_temperature", case([W], member_states=[state]),
         case([W] + T, member_states=[state]), "published",
         "the same authored kind, law reference and temperature bits: equal content"),
        ("member_state_temperature_one_ulp_apart", case([W], member_states=[state]),
         case([W] + T, member_states=[state_ulp]), "refused_g1",
         "authored temperatures one binary64 apart: other content, as for two loads one bit apart"),
        ("constant_effort_preload_in_both", case([W], preloads=[preload]), case([W] + T, preloads=[preload]),
         "published", "a preload present in every case is not a listed action; present in both in any event"),
    ]
    out = []
    for name, s, o, expect, reason in rows:
        decided = contained(applied(s), applied(o))
        control = g1_primitives_only(s, o)
        gate2 = g2(s, o)
        outcome = verdict(decided, gate2)
        assert outcome == expect, name
        out.append({"id": name, "outcome": outcome, "reason": reason, "g1_contained": decided,
                    "g2_same_basis": gate2,
                    "control_primitives_only": {"g1_contained": control, "outcome": verdict(control, gate2)},
                    "sustained": describe(s), "operating": describe(o)})
        CHECKS["g1_rows"] += 1
    return out


def verdict(g1_ok, g2_ok):
    return ("published" if g1_ok and g2_ok else "refused_g1_and_g2" if not g1_ok and not g2_ok
            else "refused_g1" if not g1_ok else "refused_g2")


def describe(c):
    d = {"primitives": [{"id": p[0], "category": p[1], "target": p[2], "direction": p[3], "dimension": p[4],
                         "value": float(p[5]), "bits": float(p[5]).hex(), "unit": p[6]} for p in c["primitives"]],
         "pressure_regions": [{"members": list(r[0]), "pressure_Pa": float(r[1])} for r in c.get("regions", [])],
         "pressure_elongation": "on" if c.get("on") else "off",
         "modulus_basis": basis_text(c["basis"])}
    if c.get("member_states"):
        d["member_states"] = [{"member": m, "kind": k, "expansion_law_ref": law, "temperature": float(t),
                               "bits": float(t).hex(), "unit": u} for m, k, law, t, u in c["member_states"]]
    if c.get("support_motions"):
        d["prescribed_motions"] = [{"target": t, "dof": dof, "value": float(v), "bits": float(v).hex(), "unit": u}
                                   for t, dof, v, u in c["support_motions"]]
    if c.get("preloads"):
        d["preloads"] = [{"target": t, "dof": dof, "value": float(v), "bits": float(v).hex(), "unit": u}
                         for t, dof, v, u in c["preloads"]]
    return d


def from_described(d):
    """Rebuild a case from its emitted record, every value from its emitted binary64 bits."""
    def b(x):
        return Q(float.fromhex(x["bits"]))
    return {"primitives": [(p["id"], p["category"], p["target"], p["direction"], p["dimension"], b(p), p["unit"])
                           for p in d["primitives"]],
            "regions": [(tuple(r["members"]), Q(r["pressure_Pa"])) for r in d["pressure_regions"]],
            "on": d["pressure_elongation"] == "on",
            "member_states": [(m["member"], m["kind"], m["expansion_law_ref"], b(m), m["unit"])
                              for m in d.get("member_states", [])],
            "support_motions": [(m["target"], m["dof"], b(m), m["unit"]) for m in d.get("prescribed_motions", [])]}


def basis_text(b):
    return f"exact basis reference: point {b[1]} K" if b[0] == "ref" else f"solve temperature {b[1]} K"


# --------------------------------------------------------------------------
# Numerical pairs
# --------------------------------------------------------------------------

def pairs():
    out = []
    for m in ("M0", "M1", "M2"):
        for o_on, s_on in ((False, False), (True, True), (True, False)):
            out.append((f"{m}/equal_pressure/operating_{'on' if o_on else 'off'}_sustained_{'on' if s_on else 'off'}",
                        m, dict(basis=REF, p=P_LO, on=s_on, thermal=False), dict(basis=REF, p=P_LO, on=o_on, thermal=True)))
    for m in ("M1", "M2"):
        for o_on, s_on in ((False, False), (True, True), (True, False)):
            out.append((f"{m}/pressure_change/operating_{'on' if o_on else 'off'}_sustained_{'on' if s_on else 'off'}",
                        m, dict(basis=REF, p=P_LO, on=s_on, thermal=False), dict(basis=REF, p=P_HI, on=o_on, thermal=True)))
    out.append(("M1/g2/mixed_basis", "M1", dict(basis=REF, p=P_LO, on=True, thermal=False),
                dict(basis=HOT, p=P_LO, on=True, thermal=True)))
    out.append(("M1/g2/same_basis_twin", "M1", dict(basis=HOT, p=P_LO, on=True, thermal=False),
                dict(basis=HOT, p=P_LO, on=True, thermal=True)))
    return out


def unit_of(key):
    kind = key.split("/")[0]
    comp = key.split("/")[-1]
    if kind == "u":
        return "m" if comp.startswith("U") else "rad"
    if kind == "axial":
        return "N"
    return "N" if comp in ("FX", "FY", "FZ", "N", "VY", "VZ") else "N*m"


def exact_row(v):
    return {"exact": v.text(), "value": f64(v)}


def magnitudes(model_id, rows):
    model = MODELS[model_id]
    groups = {}
    for n in model["nodes"]:
        groups[f"u/{n}/translation"] = [rows[f"u/{n}/{c}"] for c in COMP_U[:3]]
        groups[f"u/{n}/rotation"] = [rows[f"u/{n}/{c}"] for c in COMP_U[3:]]
    for n in model["anchors"]:
        groups[f"reaction/{n}/force"] = [rows[f"reaction/{n}/{c}"] for c in COMP_F[:3]]
        groups[f"reaction/{n}/moment"] = [rows[f"reaction/{n}/{c}"] for c in COMP_F[3:]]
    return {k: norm(v) for k, v in groups.items()}


def thermal_only(model_id, mods):
    """The closed-form response to the thermal free strain alone (method 2)."""
    e_mod, _g, alpha = mods
    loads = {"nodal": {n: (V0, V0) for n in MODELS[model_id]["nodes"]},
             "eig": {m[0]: lift(alpha * DT) for m in MODELS[model_id]["members"]}}
    rows, _ = method_force(model_id, mods, loads)
    return rows


def as_g1(model_id, c, operating):
    prims = weight_prims(model_id) + (thermal_prims(model_id) if operating else [])
    return {"primitives": prims, "regions": regions(model_id, c["p"]), "on": c["on"], "basis": c["basis"]}


def operand_difference(o_val: Pi, s_val: Pi) -> float:
    """What a published subtraction must equal bit for bit: the exact difference of the two
    operands' published binary64 values, rounded once to binary64."""
    return float(Q(f64(o_val)) - Q(f64(s_val)))


def pair_record(pid, model_id, s, o):
    sw, s_loads = solve_case(model_id, s)
    ow, o_loads = solve_case(model_id, o)
    diff = {k: ow[k] - sw[k] for k in ow}
    same_basis = s["basis"] == o["basis"]
    rec = {"id": pid, "model": model_id,
           "sustained": {"modulus_basis": basis_text(s["basis"]), "pressure_Pa": float(s["p"]),
                         "pressure_elongation": "on" if s["on"] else "off", "primitives": "weights"},
           "operating": {"modulus_basis": basis_text(o["basis"]), "pressure_Pa": float(o["p"]),
                         "pressure_elongation": "on" if o["on"] else "off", "primitives": "weights + thermal on every member"},
           "g2_same_basis": same_basis}
    g1_ok = contained(applied(as_g1(model_id, s, False)), applied(as_g1(model_id, o, True)))
    rec["g1_contained"] = g1_ok
    rec["control_primitives_only_g1_contained"] = g1_primitives_only(as_g1(model_id, s, False), as_g1(model_id, o, True))
    rec["outcome"] = verdict(g1_ok, same_basis)
    published = rec["outcome"] == "published"
    rec["row_text"] = ROW_TEXT if published else None
    assert g1_ok == (s["p"] == o["p"] and (o["on"] or not s["on"]))
    if same_basis:
        dl = load_difference(o_loads, s_loads)
        mods = moduli(o["basis"])
        closed, _ = method_force(model_id, mods, dl)
        closed = add_axial_identities(model_id, closed, o["p"] - s["p"])
        assert closed.keys() == diff.keys() and all(closed[k] == diff[k] for k in diff), \
            "difference = closed-form response to the loads the operating case has beyond the sustained"
        assert equilibrium(model_id, diff, dl), "global equilibrium of the difference"
        CHECKS["difference_closed_form_rows"] += len(diff)
        CHECKS["difference_equilibrium"] += 1
        # reading J: alternative meaning of the elongation setting
        sj, _ = solve_case(model_id, s, "J")
        oj, _ = solve_case(model_id, o, "J")
        dj = {k: oj[k] - sj[k] for k in oj}
        differs = {k: v for k, v in dj.items() if v != diff[k]}
        if s["p"] == o["p"]:
            assert not differs, "equal pressure: the analytical difference does not depend on the reading of 'off'"
            rec["reading_J_alternative"] = (
                "analytically identical to reading W on every row (exact differences); publication values are "
                "formed from rounded operands and can differ because of that rounding: see reading_J_publication")
        else:
            assert differs
            rec["reading_J_alternative"] = {
                k: dict(exact_row(v), from_published_operands=operand_difference(oj[k], sj[k]))
                for k, v in sorted(differs.items())}
        # what the difference consists of
        th = thermal_only(model_id, mods)
        pressure_part = {k: diff[k] - th.get(k, ZERO) for k in diff if not k.startswith("axial/")}
        rec["consists_of"] = ("thermal free strain only" if all(v.zero() for v in pressure_part.values())
                              else "thermal free strain + pressure free strain")
        if s["p"] == o["p"] and s["on"] == o["on"]:
            assert rec["consists_of"] == "thermal free strain only"
        if s["p"] != o["p"]:
            rec["consists_of"] += "; N_wall and N_mech also carry their (p_o - p_s) A_i terms"
        if pid.startswith(("M1/equal", "M2/equal")):
            assert any(not diff[k].zero() for k in diff if k.startswith("reaction/")), "not trivially zero"
            assert any(not diff[k].zero() for k in diff if k.startswith("u/")), "not trivially zero"
    else:
        twin_basis = o["basis"]
        th = thermal_only(model_id, moduli(twin_basis))
        rec["closed_form_thermal_response_on_operating_basis_differs_from_withheld_values"] = True
        assert any(f64(diff[k]) != f64(th[k]) for k in th if k in diff)
    field = ("difference" if published else "withheld_would_be_difference" if not same_basis
             else "undeclared_subtraction")
    rec["values_field"] = field
    rows_out = {}
    pub = {}
    for k in sorted(diff):
        pub[k] = operand_difference(ow[k], sw[k])
        rows_out[k] = dict(exact_row(diff[k]), unit=unit_of(k), from_published_operands=pub[k])
        if pub[k] != rows_out[k]["value"]:
            CHECKS[f"rows_published_operand_value_differs_{field}"] += 1
        CHECKS[f"rows_{field}"] += 1
    rec[field] = rows_out
    if same_basis:
        rec["reading_J_cases"] = {"sustained": {k: f64(sj[k]) for k in sorted(sj)},
                                  "operating": {k: f64(oj[k]) for k in sorted(oj)}}
        jpub = {}
        tally = Counter()
        for k in sorted(dj):
            value = float(Q(rec["reading_J_cases"]["operating"][k]) - Q(rec["reading_J_cases"]["sustained"][k]))
            assert value == operand_difference(oj[k], sj[k])
            same_exact = dj[k] == diff[k]
            same_bits = value.hex() == pub[k].hex()
            jpub[k] = {"from_published_operands": value, "analytically_identical_to_reading_W": same_exact,
                       "bit_identical_to_reading_W_publication": same_bits}
            if isinstance(rec["reading_J_alternative"], dict) and k in rec["reading_J_alternative"]:
                assert rec["reading_J_alternative"][k]["from_published_operands"] == value
            tally["rows"] += 1
            tally["analytically_identical"] += same_exact
            tally["publication_bit_identical"] += same_bits
            tally["analytically_identical_but_publication_differs"] += same_exact and not same_bits
        rec["reading_J_publication"] = jpub
        rec["reading_J_publication_summary"] = dict(sorted(tally.items()))
        CHECKS["reading_J_publication_rows"] += tally["rows"]
        CHECKS["reading_J_publication_bit_identical_to_W"] += tally["publication_bit_identical"]
        CHECKS["reading_J_analytically_identical_publication_differs"] += \
            tally["analytically_identical_but_publication_differs"]
    mo, ms, md = magnitudes(model_id, ow), magnitudes(model_id, sw), magnitudes(model_id, diff)
    mp = magnitudes(model_id, {k: lift(Q(v)) for k, v in pub.items()})
    mags = {}
    for k in sorted(md):
        mags[k] = {"from_combined_components": md[k][1], "exact_square": md[k][0].text(),
                   "from_published_operand_components": mp[k][1],
                   "forbidden_subtracted_magnitudes": sqrt_difference(mo[k][0], ms[k][0])}
        wrong = mags[k]["forbidden_subtracted_magnitudes"]
        mags[k]["forbidden_value_distinguishes"] = (wrong.hex() != md[k][1].hex()
                                                    and wrong.hex() != mp[k][1].hex())
        CHECKS[f"magnitudes_forbidden_value_distinguishes_{field}"] += mags[k]["forbidden_value_distinguishes"]
        if mp[k][1] != md[k][1]:
            CHECKS[f"magnitudes_published_operand_value_differs_{field}"] += 1
        CHECKS[f"magnitudes_{field}"] += 1
    rec[field.replace("difference", "magnitudes") if field != "undeclared_subtraction"
        else "undeclared_subtraction_magnitudes"] = mags
    rec["cases"] = {"sustained": {k: f64(sw[k]) for k in sorted(sw)}, "operating": {k: f64(ow[k]) for k in sorted(ow)}}
    CHECKS["pairs"] += 1
    return rec


# --------------------------------------------------------------------------
# M1 by hand (README): the in-plane flexibility of the released L, thermal only
# --------------------------------------------------------------------------

def m1_by_hand(basis):
    e_mod, _g, alpha = moduli(basis)
    a, b = Q(21, 4), Q(27, 8)
    EA, EI = e_mod * A_N, e_mod * I_N   # each times pi
    f = [[b ** 3 / (3 * EI) + a * b * b / EI + a / EA, -a * a * b / (2 * EI), -b * b / (2 * EI) - a * b / EI],
         [-a * a * b / (2 * EI), b / EA + a ** 3 / (3 * EI), a * a / (2 * EI)],
         [-b * b / (2 * EI) - a * b / EI, a * a / (2 * EI), (a + b) / EI]]   # times 1/pi
    d0 = [alpha * DT * a, alpha * DT * b, Q(0)]
    (x,) = solve_rational(f, [[-v for v in d0]])
    X, Y, Z = (Pi({1: v}) for v in x)
    rows, _ = method_force("M1", moduli(basis), {"nodal": {n: (V0, V0) for n in MODELS["M1"]["nodes"]},
                                                  "eig": {"AB": lift(alpha * DT), "BC": lift(alpha * DT)}})
    assert (rows["reaction/C/FX"], rows["reaction/C/FY"], rows["reaction/C/MZ"]) == (X, Y, Z)
    assert all(rows[f"reaction/C/{c}"].zero() for c in ("FZ", "MX", "MY"))
    CHECKS["hand_formula_m1"] += 1
    return {"basis": basis_text(basis), "X_FX_at_C": exact_row(X), "Y_FY_at_C": exact_row(Y), "Z_MZ_at_C": exact_row(Z)}


# --------------------------------------------------------------------------
# Inputs as emitted, and the binary64 substitution check
# --------------------------------------------------------------------------

def emitted_inputs():
    vals = {"poisson_ratio": NU, "outside_diameter_m": D, "wall_m": WALL, "mill_tolerance_m": C_M,
            "corrosion_allowance_m": C_A, "thermal_dT_K": DT, "pressure_low_Pa": P_LO, "pressure_high_Pa": P_HI,
            "solve_temperature_K": HOT[1], "basis_reference_point_K": REF[1]}
    for i, (T, E, G, al) in enumerate(POINTS):
        vals.update({f"point{i}_T_K": T, f"point{i}_E_Pa": E, f"point{i}_G_Pa": G, f"point{i}_alpha_per_K": al})
    for mid, m in MODELS.items():
        for n, xyz in m["nodes"].items():
            for c, v in zip("xyz", xyz):
                vals[f"{mid}_{n}_{c}_m"] = Q(v)
        for n, w in m["weights"].items():
            for c, v in zip("xyz", w):
                vals[f"{mid}_weight_{n}_{c}_N"] = Q(v)
    for k, v in vals.items():
        assert Q(float(v)) == v, f"input {k} is not a binary64"
    CHECKS["inputs_binary64_exact"] = len(vals)
    return {k: float(v) for k, v in sorted(vals.items())}


def substitution_check(rows):
    """Recompute every discrete outcome with each input replaced by its emitted binary64."""
    for r in rows:
        s, o = from_described(r["sustained"]), from_described(r["operating"])
        assert contained(applied(s), applied(o)) == r["g1_contained"], r["id"]
        assert g1_primitives_only(s, o) == r["control_primitives_only"]["g1_contained"], r["id"]
    for basis in (REF, HOT):
        assert tuple(Q(float(v)) for v in moduli(basis)) == moduli(basis)
    assert moduli(REF) != moduli(HOT)
    CHECKS["binary64_substitution_outcomes"] += len(rows) + 2


# --------------------------------------------------------------------------

def build() -> dict[str, bytes]:
    for m in MODELS.values():
        check_region_constraint(m)
    inputs = emitted_inputs()
    e_ref = moduli(REF)[0]
    derived = {
        "nominal_inner_radius_m": float(R_IN), "reduced_wall_m": float(T_R), "pressure_bore_radius_m": float(R_IR),
        "A_n_m2": exact_row(Pi({1: A_N})), "I_n_m4": exact_row(Pi({1: I_N})), "J_n_m4": exact_row(Pi({1: J_N})),
        "A_i_m2": exact_row(Pi({1: A_I})),
        "moduli_ref_300K": [float(v) for v in moduli(REF)], "moduli_solve_600K": [float(v) for v in moduli(HOT)],
        "eps_pressure_low_ref": exact_row(lift(eps_pressure(P_LO, e_ref))),
        "eps_pressure_high_ref": exact_row(lift(eps_pressure(P_HI, e_ref))),
        "eps_thermal": exact_row(lift(moduli(REF)[2] * DT)),
        "junctions": {m: junctions(MODELS[m]) for m in MODELS},
    }
    g1 = g1_rows()
    substitution_check(g1)
    records = [pair_record(*p) for p in pairs()]
    setting = {}
    by_id = {r["id"]: r for r in records}
    for m in ("M0", "M1", "M2"):
        off = by_id[f"{m}/equal_pressure/operating_off_sustained_off"]["difference"]
        on = by_id[f"{m}/equal_pressure/operating_on_sustained_on"]["difference"]
        assert all(off[k]["exact"] == on[k]["exact"] for k in off), "analytically identical"
        differ = sorted(k for k in off if off[k]["from_published_operands"].hex() != on[k]["from_published_operands"].hex())
        setting[m] = {"compared": "off-off against on-on, equal pressure", "rows": len(off),
                      "analytically_identical": len(off), "publication_bit_identical": len(off) - len(differ),
                      "publication_differs": differ}
    hand = [m1_by_hand(REF), m1_by_hand(HOT)]
    references = {
        "schema": "exact_difference_basis/v1",
        "row_text": ROW_TEXT,
        "magnitude_rule": "a magnitude row of the difference is the norm of the difference's own components, "
                          "never the difference of the operands' magnitudes",
        "inputs": inputs,
        "derived": derived,
        "models": {k: {"title": m["title"], "nodes": {n: [float(c) for c in p] for n, p in m["nodes"].items()},
                       "anchors": m["anchors"],
                       "members": [{"id": x[0], "from": x[1], "to": x[2], "y_reference": list(x[3])} for x in m["members"]],
                       "pressure_regions": [{"members": r, "terminals": "transfers_to_wall"} for r in m["regions"]],
                       "weights_N": {n: [float(c) for c in w] for n, w in m["weights"].items()}}
                   for k, m in MODELS.items()},
        "g1": g1,
        "pairs": records,
        "m1_by_hand": hand,
        "setting_independence_publication": setting,
        "cross_checks": dict(sorted(CHECKS.items())),
    }
    text = (json.dumps(references, indent=1, sort_keys=True) + "\n").encode()
    sums = "".join(f"{hashlib.sha256(data).hexdigest()}  {name}\n" for name, data in [
        ("README.md", (HERE / "README.md").read_bytes()),
        ("generators/exact_difference_basis_reference.py", SELF.read_bytes()),
        ("references.json", text)])
    return {"references.json": text, "SHA256SUMS": sums.encode()}


def main() -> int:
    outputs = build()
    if "--write" in sys.argv[1:]:
        for name, data in outputs.items():
            (HERE / name).write_bytes(data)
        print("written")
        return 0
    stale = [n for n, d in outputs.items() if not (HERE / n).exists() or (HERE / n).read_bytes() != d]
    print("stale: " + ", ".join(stale) if stale else "references are current")
    return 1 if stale else 0


if __name__ == "__main__":
    raise SystemExit(main())
