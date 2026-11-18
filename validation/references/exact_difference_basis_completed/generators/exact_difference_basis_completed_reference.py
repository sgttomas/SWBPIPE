"""Successor exact references for the declared operating-minus-sustained difference on the exact route
(DEL-05-02 REQ-012, AC-003): the completed admission policy, the frozen package's rows re-derived under it, and
the added witnesses W1 to W9, F1, F2 and F2b.

Python standard library only. It imports, reads and runs no product code and no product output, and it does not
read the frozen package either: the frozen inputs are restated here, and the package test compares the carried
rows with the frozen file. Every input is invented. Arithmetic is exact: a value is a finite sum of rational
multiples of integer powers of pi (class Exact), published with the nearest binary64, which is decided from
rational bounds on pi (Gauss's arctangent formula with a proven error).

Two entries per row, never merged:
  admission   the verdict of the policy (restraint, then G-1, then G-2), with its reason and each part's outcome;
  mechanical  the independently derived result: a linear increment on one common operator, a difference of two
              separately solved states on different operators, or none.

Methods:
  frame       direct stiffness, 12 x 12 Euler-Bernoulli elements, exact Gauss-Jordan elimination, free strains as
              held end actions (the carried pairs M0, M1, M2);
  bar         exact axial bar with rigid supports, prescribed motions, springs with a base and consistent loads
              (the added rows), cross-checked against closed forms.

Run ``python3 -I generators/exact_difference_basis_completed_reference.py`` from the package directory to rebuild
every output in memory and compare it byte for byte with the committed files; ``--write`` rewrites them.
"""
from __future__ import annotations

import hashlib
import json
import struct
import sys
from collections import Counter
from fractions import Fraction as F
from math import isqrt
from pathlib import Path

PACKAGE = Path(__file__).resolve().parent.parent
THIS = Path(__file__).resolve()
COUNTS = Counter()


# ==========================================================================================================
# Exact values: sum over k of c_k * pi^k, c_k rational
# ==========================================================================================================

class Exact:
    __slots__ = ("c",)

    def __init__(self, c=None):
        self.c = {}
        for k, v in (c or {}).items():
            v = F(v)
            if v:
                self.c[k] = v

    @staticmethod
    def of(v):
        return v if isinstance(v, Exact) else Exact({0: F(v)})

    def __add__(self, other):
        other = Exact.of(other)
        out = dict(self.c)
        for k, v in other.c.items():
            out[k] = out.get(k, F(0)) + v
        return Exact(out)

    __radd__ = __add__

    def __neg__(self):
        return Exact({k: -v for k, v in self.c.items()})

    def __sub__(self, other):
        return self + (-Exact.of(other))

    def __rsub__(self, other):
        return Exact.of(other) - self

    def __mul__(self, other):
        other = Exact.of(other)
        out = {}
        for a, x in self.c.items():
            for b, y in other.c.items():
                out[a + b] = out.get(a + b, F(0)) + x * y
        return Exact(out)

    __rmul__ = __mul__

    def __truediv__(self, other):
        other = Exact.of(other)
        if len(other.c) != 1:
            raise ValueError("division by a single power of pi only")
        ((k, q),) = other.c.items()
        return Exact({a - k: v / q for a, v in self.c.items()})

    def __eq__(self, other):
        return self.c == Exact.of(other).c

    def __hash__(self):
        return hash(tuple(sorted(self.c.items())))

    def is_zero(self):
        return not self.c

    def text(self):
        if not self.c:
            return "0"
        parts = []
        for k in sorted(self.c):
            q = self.c[k]
            parts.append(str(q) if k == 0 else f"{q}*pi" if k == 1 else f"{q}*pi^{k}")
        return " + ".join(parts)


PI = Exact({1: 1})
ZERO = Exact()


def _pi_interval(bits=640):
    """Gauss: pi = 48 atan(1/18) + 32 atan(1/57) - 20 atan(1/239), each series in fixed point.

    With one = 2^bits, x_k = floor(one / n^(2k+1)) is exact (nested floors of integer division), each term
    floor(x_k / (2k+1)) is low by less than one unit, and the alternating tail after the last non-zero x_k is
    below one unit; so each series is within (terms + 1) units.
    """
    one = 1 << bits

    def atan_inv(n):
        total, k, x, n2 = 0, 0, one // n, n * n
        while x:
            term = x // (2 * k + 1)
            total += -term if k % 2 else term
            x //= n2
            k += 1
        return total, k + 1

    mid, err = 0, 0
    for coefficient, n in ((48, 18), (32, 57), (-20, 239)):
        value, e = atan_inv(n)
        mid += coefficient * value
        err += abs(coefficient) * e
    return F(mid - err, one), F(mid + err, one)


PI_LOW, PI_HIGH = _pi_interval()
assert F(3141592653589793, 10 ** 15) < PI_HIGH and PI_LOW < F(3141592653589794, 10 ** 15)


def _interval(v: Exact):
    lo = hi = F(0)
    for k, q in v.c.items():
        a, b = (PI_LOW ** k, PI_HIGH ** k) if k >= 0 else (PI_HIGH ** k, PI_LOW ** k)
        lo, hi = (lo + q * a, hi + q * b) if q > 0 else (lo + q * b, hi + q * a)
    return lo, hi


def nearest(v: Exact) -> float:
    """The binary64 nearest the exact value (ties to even, by Python's correctly rounded conversion)."""
    if v.is_zero():
        return 0.0
    lo, hi = _interval(v)
    a, b = float(lo), float(hi)
    if a != b:
        raise ArithmeticError(f"pi bounds too wide to round {v.text()}")
    return a


def bits(x: float) -> str:
    return struct.pack(">d", x).hex()


def rational_bits(q) -> str:
    q = F(q)
    assert F(float(q)) == q, f"{q} is not a binary64"
    return bits(float(q))


def entry(v, unit):
    v = Exact.of(v)
    return {"exact": v.text(), "value": nearest(v), "unit": unit}


# ==========================================================================================================
# Exact linear algebra
# ==========================================================================================================

def gauss_jordan(matrix, columns):
    """Solve matrix * x = column for each column; matrix and columns rational."""
    n = len(matrix)
    a = [list(row) + [col[i] for col in columns] for i, row in enumerate(matrix)]
    for c in range(n):
        p = next(r for r in range(c, n) if a[r][c] != 0)
        a[c], a[p] = a[p], a[c]
        pivot = a[c][c]
        a[c] = [x / pivot for x in a[c]]
        for r in range(n):
            if r != c and a[r][c] != 0:
                f = a[r][c]
                a[r] = [x - f * y for x, y in zip(a[r], a[c])]
    return [[a[i][n + j] for i in range(n)] for j in range(len(columns))]


def solve_scaled(matrix, power, rhs):
    """Solve (pi^power * matrix) x = rhs, matrix rational, rhs a list of Exact values."""
    powers = sorted({k for v in rhs for k in v.c})
    if not powers:
        return [ZERO] * len(rhs)
    sols = gauss_jordan(matrix, [[v.c.get(k, F(0)) for v in rhs] for k in powers])
    out = [ZERO] * len(rhs)
    for k, sol in zip(powers, sols):
        out = [x + Exact({k - power: s}) for x, s in zip(out, sol)]
    return out


def v3(*xs):
    return tuple(Exact.of(x) for x in xs)


def vplus(a, b):
    return tuple(x + y for x, y in zip(a, b))


def vminus(a, b):
    return tuple(x - y for x, y in zip(a, b))


def vtimes(s, a):
    return tuple(Exact.of(s) * x for x in a)


def vdot(a, b):
    out = ZERO
    for x, y in zip(a, b):
        out = out + Exact.of(x) * Exact.of(y)
    return out


def vcross(a, b):
    a, b = [Exact.of(x) for x in a], [Exact.of(x) for x in b]
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


NULL3 = v3(0, 0, 0)


def rational_sqrt(q: F) -> F:
    r = F(isqrt(q.numerator), isqrt(q.denominator))
    assert r * r == q, f"{q} has no rational square root"
    return r


# ==========================================================================================================
# The frozen package's inputs, restated (validation/references/exact_difference_basis/README.md, "Inputs")
# ==========================================================================================================

FROZEN_NU = F(9, 32)
FROZEN_POINTS = {                       # T (K): (E Pa, G Pa, alpha /K)
    F(300): (F(194750000000), F(76000000000), F(3, 2 ** 18)),
    F(700): (F(184500000000), F(72000000000), F(3, 2 ** 18)),
}
FROZEN_D, FROZEN_T, FROZEN_CM, FROZEN_CA = F(35, 128), F(9, 1024), F(1, 2048), F(1, 1024)
FROZEN_DT = F(475, 2)
FROZEN_P, FROZEN_P_CHANGED = F(5750000), F(6500000)
BASES = {
    "REF": {"selection_kind": "exact_basis_reference", "temperature": F(300)},
    "HOT": {"selection_kind": "solve_temperature", "temperature": F(600)},
    # K1 only: a solve temperature at the 300 K endpoint, interpolated between 300 K and 700 K at fraction 0
    "T300": {"selection_kind": "solve_temperature", "temperature": F(300), "interpolate_between": (F(300), F(700))},
}
# W11 only: an invented, non-standard material whose two points have equal E and G (so nu = 1/4 at both) and
# different expansion coefficients. It is not the frozen material and not the frozen bases.
W11_MATERIAL = "invented_w11_equal_moduli_two_alphas"
W11_NU = F(1, 4)
W11_POINTS = {                          # T (K): (E Pa, G Pa, alpha /K)
    F(250): (F(187500000000), F(75000000000), F(5, 2 ** 19)),
    F(650): (F(187500000000), F(75000000000), F(7, 2 ** 19)),
}
for _t, (_e, _g, _a) in W11_POINTS.items():
    assert _e / (2 * _g) - 1 == W11_NU
BASES["W11_LO"] = {"selection_kind": "exact_basis_reference", "temperature": F(250), "points": W11_POINTS,
                   "material": W11_MATERIAL, "nu": W11_NU}
BASES["W11_HI"] = {"selection_kind": "exact_basis_reference", "temperature": F(650), "points": W11_POINTS,
                   "material": W11_MATERIAL, "nu": W11_NU}

for _t, (_e, _g, _a) in FROZEN_POINTS.items():
    assert _e / (2 * _g) - 1 == FROZEN_NU


def basis_moduli(label):
    """(E, G, alpha, interpolation fraction or None) for a basis label."""
    t = BASES[label]["temperature"]
    points = BASES[label].get("points", FROZEN_POINTS)
    if "interpolate_between" in BASES[label]:
        lo, hi = BASES[label]["interpolate_between"]
        f = (t - lo) / (hi - lo)
        return tuple(x + f * (y - x) for x, y in zip(points[lo], points[hi])) + (f,)
    if t in points:
        e, g, a = points[t]
        return e, g, a, None
    lo = max(p for p in points if p < t)
    hi = min(p for p in points if p > t)
    f = (t - lo) / (hi - lo)
    return tuple(x + f * (y - x) for x, y in zip(points[lo], points[hi])) + (f,)


FROZEN_RO = FROZEN_D / 2
FROZEN_RI_NOMINAL = FROZEN_RO - FROZEN_T
FROZEN_RI_PRESSURE = FROZEN_RO - ((FROZEN_T - FROZEN_CM) - FROZEN_CA)
AREA = FROZEN_RO ** 2 - FROZEN_RI_NOMINAL ** 2          # times pi
INERTIA = (FROZEN_RO ** 4 - FROZEN_RI_NOMINAL ** 4) / 4  # times pi
POLAR = 2 * INERTIA                                     # times pi
BORE = FROZEN_RI_PRESSURE ** 2                          # times pi
assert AREA == F(2439, 1048576) and BORE == F(70225, 4194304) and INERTIA == F(89660079, 4398046511104)

UP, ALONG_X = (0, 0, 1), (1, 0, 0)
FROZEN_MODELS = {
    "M0": {"nodes": {"A": (0, 0, 0), "B": (F(15, 4), 0, 0), "C": (F(15, 4), F(9, 4), 0)},
           "anchors": ["A"],
           "members": [("AB", "A", "B", UP), ("BC", "B", "C", UP)],
           "regions": [["AB"], ["BC"]],
           "weights": {"C": F(-1225, 2)}},
    "M1": {"nodes": {"A": (0, 0, 0), "B": (F(21, 4), 0, 0), "C": (F(21, 4), F(27, 8), 0)},
           "anchors": ["A", "C"],
           "members": [("AB", "A", "B", UP), ("BC", "B", "C", UP)],
           "regions": [["AB"], ["BC"]],
           "weights": {"B": F(-7375, 4)}},
    "M2": {"nodes": {"A": (0, 0, 0), "E": (F(5, 2), 0, 0), "B": (F(19, 4), 0, 0),
                     "C": (F(19, 4), F(21, 8), 0), "D": (F(19, 4), F(21, 8), F(-25, 8))},
           "anchors": ["A", "D"],
           "members": [("AE", "A", "E", UP), ("EB", "E", "B", UP), ("BC", "B", "C", UP), ("CD", "C", "D", ALONG_X)],
           "regions": [["AE", "EB"], ["BC"], ["CD"]],
           "weights": {"E": F(-2501, 2), "C": F(-3625, 4)}},
}
DOF_U = ("UX", "UY", "UZ", "RX", "RY", "RZ")
DOF_F = ("FX", "FY", "FZ", "MX", "MY", "MZ")
DOF_L = ("N", "VY", "VZ", "T", "MY", "MZ")


def frame_axes(model, member):
    _, a, b, yref = member
    pa, pb = v3(*model["nodes"][a]), v3(*model["nodes"][b])
    d = vminus(pb, pa)
    length = rational_sqrt(vdot(d, d).c[0])
    t = vtimes(1 / length, d)
    y = vminus(v3(*yref), vtimes(vdot(v3(*yref), t), t))
    y = vtimes(1 / rational_sqrt(vdot(y, y).c[0]), y)
    return (t, y, vcross(t, y)), length


def region_terminals(model):
    """(node, s): s the unit vector from the terminal node into its member; every terminal transfers_to_wall."""
    by_id = {m[0]: m for m in model["members"]}
    out = []
    for region in model["regions"]:
        first, last = by_id[region[0]], by_id[region[-1]]
        out.append((first[1], frame_axes(model, first)[0][0]))
        out.append((last[2], vtimes(-1, frame_axes(model, last)[0][0])))
    return out


def junction_nodes(model):
    seen = Counter(n for n, _ in region_terminals(model))
    return {n for n, c in seen.items() if c > 1}


# ==========================================================================================================
# Frame method: direct stiffness
# ==========================================================================================================

def element_matrix(ea, ei, gj, length):
    k = [[F(0)] * 12 for _ in range(12)]

    def sym(i, j, v):
        k[i][j] = k[j][i] = v

    sym(0, 0, ea / length); sym(6, 6, ea / length); sym(0, 6, -ea / length)
    sym(3, 3, gj / length); sym(9, 9, gj / length); sym(3, 9, -gj / length)
    c = ei / length ** 3
    # (v1, rotation coupled to v at 1, v2, rotation at 2, sign): xy plane (v, rz) and xz plane (w, ry)
    for v1, r1, v2, r2, s in ((1, 5, 7, 11, 1), (2, 4, 8, 10, -1)):
        sym(v1, v1, 12 * c); sym(v2, v2, 12 * c); sym(v1, v2, -12 * c)
        sym(v1, r1, s * 6 * length * c); sym(v1, r2, s * 6 * length * c)
        sym(v2, r1, -s * 6 * length * c); sym(v2, r2, -s * 6 * length * c)
        sym(r1, r1, 4 * length ** 2 * c); sym(r2, r2, 4 * length ** 2 * c); sym(r1, r2, 2 * length ** 2 * c)
    return k


def frame_solve(model_id, label, nodal, strains):
    """Solve one load set. nodal: {node: (force3, moment3)}, strains: {member: Exact free strain}."""
    model = FROZEN_MODELS[model_id]
    e_mod, g_mod, _alpha, _f = basis_moduli(label)
    order = list(model["nodes"])
    base = {n: 6 * i for i, n in enumerate(order)}
    size = 6 * len(order)
    stiffness = [[F(0)] * size for _ in range(size)]   # times pi
    load = [ZERO] * size
    elements = []
    for member in model["members"]:
        frame, length = frame_axes(model, member)
        rot = [[x.c.get(0, F(0)) for x in axis] for axis in frame]
        local = element_matrix(e_mod * AREA, e_mod * INERTIA, g_mod * POLAR, length)
        tmat = [[F(0)] * 12 for _ in range(12)]
        for blk in range(4):
            for i in range(3):
                for j in range(3):
                    tmat[3 * blk + i][3 * blk + j] = rot[i][j]
        glob = [[sum(tmat[a][i] * local[a][b] * tmat[b][j] for a in range(12) for b in range(12)
                     if tmat[a][i] and tmat[b][j]) for j in range(12)] for i in range(12)]
        dofs = [base[member[1]] + c for c in range(6)] + [base[member[2]] + c for c in range(6)]
        for i in range(12):
            for j in range(12):
                stiffness[dofs[i]][dofs[j]] += glob[i][j]
        held = strains.get(member[0], ZERO) * Exact({1: e_mod * AREA})
        t = frame[0]
        fixed = [held * t[c] for c in range(3)] + [ZERO] * 3 + [-held * t[c] for c in range(3)] + [ZERO] * 3
        for i in range(12):
            load[dofs[i]] = load[dofs[i]] - fixed[i]
        elements.append((member, frame, glob, dofs, fixed))
    for n, (force, moment) in nodal.items():
        for c in range(3):
            load[base[n] + c] = load[base[n] + c] + force[c]
            load[base[n] + 3 + c] = load[base[n] + 3 + c] + moment[c]
    free = [d for n in order if n not in model["anchors"] for d in range(base[n], base[n] + 6)]
    x = solve_scaled([[stiffness[i][j] for j in free] for i in free], 1, [load[i] for i in free])
    u = [ZERO] * size
    for d, v in zip(free, x):
        u[d] = v
    rows = {}
    for n in order:
        for c in range(6):
            rows[f"u/{n}/{DOF_U[c]}"] = u[base[n] + c]
    reactions = {}
    for n in model["anchors"]:
        force, moment = nodal.get(n, (NULL3, NULL3))
        reactions[n] = [-force[c] for c in range(3)] + [-moment[c] for c in range(3)]
    for member, frame, glob, dofs, fixed in elements:
        ends = [PI * sum((u[dofs[j]] * glob[i][j] for j in range(12) if glob[i][j]), ZERO) + fixed[i]
                for i in range(12)]
        for end, off, node in (("i", 0, member[1]), ("j", 6, member[2])):
            for c in range(6):
                rows[f"end_force/{member[0]}/{end}/{DOF_F[c]}"] = ends[off + c]
            if node in reactions:
                reactions[node] = [r + ends[off + c] for c, r in enumerate(reactions[node])]
        for station, off, sign in (("0", 0, -1), ("1", 6, 1)):
            force, moment = ends[off:off + 3], ends[off + 3:off + 6]
            local = [vdot(force, a) * sign for a in frame] + [vdot(moment, a) * sign for a in frame]
            for c in range(6):
                rows[f"resultant/{member[0]}/{station}/{DOF_L[c]}"] = local[c]
        assert rows[f"resultant/{member[0]}/0/N"] == rows[f"resultant/{member[0]}/1/N"]
        rows[f"axial/{member[0]}/S"] = rows[f"resultant/{member[0]}/0/N"]
    for n in model["anchors"]:
        for c in range(6):
            rows[f"reaction/{n}/{DOF_F[c]}"] = reactions[n][c]
    COUNTS["frame_solves"] += 1
    return rows


def frame_balanced(model_id, rows, nodal):
    model = FROZEN_MODELS[model_id]
    total_f, total_m = NULL3, NULL3
    for n, (force, moment) in nodal.items():
        r = v3(*model["nodes"][n])
        total_f, total_m = vplus(total_f, force), vplus(total_m, vplus(vcross(r, force), moment))
    for n in model["anchors"]:
        rf = tuple(rows[f"reaction/{n}/{c}"] for c in DOF_F[:3])
        rm = tuple(rows[f"reaction/{n}/{c}"] for c in DOF_F[3:])
        r = v3(*model["nodes"][n])
        total_f, total_m = vplus(total_f, rf), vplus(total_m, vplus(vcross(r, rf), rm))
    return all(v.is_zero() for v in total_f + total_m)


def frozen_case_loads(model_id, case, reading):
    """Loads of a carried case. reading W: 'on' is the closed-end free strain, 'off' omits it.
    reading J: 'off' applies the junction resultant as nodal loads, 'on' adds the free strain.
    reading wall: DEL-04-07's wall form (caps at every terminal, Poisson shortening), only to check W's 'on'."""
    model = FROZEN_MODELS[model_id]
    e_mod, _g, alpha, _f = basis_moduli(case["basis"])
    nodal = {n: (NULL3, NULL3) for n in model["nodes"]}
    for n, w in model["weights"].items():
        nodal[n] = (vplus(nodal[n][0], v3(0, 0, w)), nodal[n][1])
    strains = {m[0]: ZERO for m in model["members"]}
    p = case["p"]
    pressure_strain = (1 - 2 * FROZEN_NU) * p * BORE / (e_mod * AREA)
    for m in model["members"]:
        if case["thermal"]:
            strains[m[0]] = strains[m[0]] + alpha * FROZEN_DT
        if p and case["on"] and reading in ("W", "J"):
            strains[m[0]] = strains[m[0]] + pressure_strain
        if p and reading == "wall":
            strains[m[0]] = strains[m[0]] - 2 * FROZEN_NU * p * BORE / (e_mod * AREA)
    if p and reading in ("J", "wall"):
        joints = junction_nodes(model)
        for n, s in region_terminals(model):
            if reading == "wall" or n in joints:
                nodal[n] = (vplus(nodal[n][0], vtimes(-p * BORE * PI, s)), nodal[n][1])
    return nodal, strains


def frozen_case_rows(model_id, case, reading="W"):
    nodal, strains = frozen_case_loads(model_id, case, reading)
    rows = frame_solve(model_id, case["basis"], nodal, strains)
    assert frame_balanced(model_id, rows, nodal), "case equilibrium"
    COUNTS["case_equilibrium"] += 1
    if reading == "W" and case["p"] and case["on"]:
        wn, ws = frozen_case_loads(model_id, case, "wall")
        wall = frame_solve(model_id, case["basis"], wn, ws)
        for k, v in rows.items():
            if k.startswith(("u/", "reaction/")):
                assert wall[k] == v, ("wall form equals the closed-end free strain", k)
            if k.startswith("axial/") and k.endswith("/S"):
                assert wall[k] == v + case["p"] * BORE * PI, ("the wall form carries N_wall", k)
        COUNTS["wall_form_equals_free_strain"] += 1
    p = case["p"]
    for m in FROZEN_MODELS[model_id]["members"]:
        s = rows[f"axial/{m[0]}/S"]
        rows[f"axial/{m[0]}/N_wall"] = s + p * BORE * PI
        rows[f"axial/{m[0]}/N_mech"] = s + (1 - 2 * FROZEN_NU) * p * BORE * PI
    return rows, nodal


def row_unit(key):
    head, last = key.split("/")[0], key.split("/")[-1]
    if head == "u":
        return "m" if last.startswith("U") else "rad"
    if head == "axial":
        return "N"
    return "N" if last in ("FX", "FY", "FZ", "N", "VY", "VZ") else "N*m"


def hand_redundants(label):
    """README of the frozen package, 'M1 by hand': the in-plane flexibility of the released L at C."""
    e_mod, _g, alpha, _f = basis_moduli(label)
    a, b = F(21, 4), F(27, 8)
    ea, ei = e_mod * AREA, e_mod * INERTIA
    flex = [[b ** 3 / (3 * ei) + a * b * b / ei + a / ea, -a * a * b / (2 * ei), -b * b / (2 * ei) - a * b / ei],
            [-a * a * b / (2 * ei), b / ea + a ** 3 / (3 * ei), a * a / (2 * ei)],
            [-b * b / (2 * ei) - a * b / ei, a * a / (2 * ei), (a + b) / ei]]
    (x,) = gauss_jordan(flex, [[-alpha * FROZEN_DT * a, -alpha * FROZEN_DT * b, F(0)]])
    rows = frame_solve("M1", label, {}, {"AB": Exact.of(alpha * FROZEN_DT), "BC": Exact.of(alpha * FROZEN_DT)})
    assert [rows["reaction/C/FX"], rows["reaction/C/FY"], rows["reaction/C/MZ"]] == [Exact({1: v}) for v in x]
    COUNTS["hand_formula_m1"] += 1


# ==========================================================================================================
# Case sides in the input format of the specification (section 4)
# ==========================================================================================================

ONE_BITS = rational_bits(1)


def q(v) -> str:
    return str(F(v))


def primitive(pid, category, target, direction, dimension, value, unit, value_text=None, value_float=None):
    if value_text is None:
        return {"id": pid, "category": category, "target": target, "direction": direction, "dimension": dimension,
                "value": q(value), "value_bits": rational_bits(value), "unit": unit}
    return {"id": pid, "category": category, "target": target, "direction": direction, "dimension": dimension,
            "value": value_text, "value_bits": bits(value_float), "unit": unit}


def anchor_supports(nodes, identity_prefix="anchor"):
    return [{"support": f"{identity_prefix}:{n}", "dof": d, "law": "rigid", "identity": f"{identity_prefix}:{n}",
             "participation": "active", "parameters": {}} for n in nodes for d in DOF_U]


def section_record():
    return {"outside_diameter": q(FROZEN_D), "wall": q(FROZEN_T), "mill_tolerance": q(FROZEN_CM),
            "corrosion_allowance": q(FROZEN_CA)}


def frozen_basis(model_id, label):
    e_mod, g_mod, _a, fraction = basis_moduli(label)
    kind = BASES[label]["selection_kind"]
    record = {"kind": kind, "selection_temperature": q(BASES[label]["temperature"]),
              "interpolation_fraction": None if fraction is None else q(fraction),
              "selected_E": q(e_mod)}
    nu = BASES[label].get("nu", FROZEN_NU)
    material = BASES[label].get("material", "invented_steel_points_300_700")
    record = dict(record, selected_nu=q(nu))
    return {m[0]: {"E": q(e_mod), "nu": q(nu), "G": q(g_mod), "material": material,
                   "constitutive_basis": "linear_elastic", "selection_kind": kind, "case_basis_label": label,
                   "selection_record": record, "section": section_record()}
            for m in FROZEN_MODELS[model_id]["members"]}


def frozen_regions(model_id, p):
    if not p:
        return []
    model = FROZEN_MODELS[model_id]
    by_id = {m[0]: m for m in model["members"]}
    return [{"pipe_members": list(r), "connector_members": [],
             "terminals": [{"node": by_id[r[0]][1], "closure_transfer": "transfers_to_wall"},
                           {"node": by_id[r[-1]][2], "closure_transfer": "transfers_to_wall"}],
             "pressure": q(p), "pressure_bits": rational_bits(p)} for r in model["regions"]]


def frozen_weights(model_id):
    return [primitive(f"w:{n}:z", "weight", f"node:{n}", "global_z", "force", w, "N")
            for n, w in FROZEN_MODELS[model_id]["weights"].items()]


def frozen_thermal(model_id):
    return [primitive(f"t:{m[0]}", "thermal", f"pipe:{m[0]}", "none", "temperature_interval", FROZEN_DT, "K")
            for m in FROZEN_MODELS[model_id]["members"]]


def side(case_id, *, primitives=(), regions=(), elongation=False, generated=(), supports=(), motions=(),
         member_states=(), fixed_loads=(), basis=None, ledger=None):
    return {"case_id": case_id, "ledger": ledger, "primitives": list(primitives),
            "pressure_regions": list(regions), "pressure_elongation": elongation, "generated": list(generated),
            "supports": list(supports), "motions": list(motions), "member_states": list(member_states),
            "fixed_loads": list(fixed_loads), "basis": basis or {}}


# ==========================================================================================================
# The admission policy (README "The admission policy"; specification section 3)
# ==========================================================================================================

def norm_number(v):
    if v is None:
        return None
    try:
        return str(F(v))
    except (ValueError, ZeroDivisionError):
        return str(v)


def norm_mapping(m):
    if isinstance(m, dict):
        return tuple(sorted((k, norm_mapping(v)) for k, v in m.items()))
    if isinstance(m, list):
        return tuple(norm_mapping(v) for v in m)
    if isinstance(m, bool) or m is None:
        return m
    return norm_number(m)


def is_zero_bits(b):
    return b in ("0000000000000000", "8000000000000000")


def restraint_laws(side_):
    laws = {}
    for s in side_["supports"]:
        if s["law"] == "free":
            law = ("free",)                           # a DOF a side does not list is free too (restraint_holds)
        elif s["law"] == "rigid":
            law = ("rigid", s["identity"])
        else:
            law = (s["law"], s["identity"], s["participation"], norm_mapping(s["parameters"]))
        laws[(s["support"], s["dof"])] = law
    return laws


def restraint_holds(s, o):
    ls, lo = restraint_laws(s), restraint_laws(o)
    for key in set(ls) | set(lo):
        if ls.get(key, ("free",)) != lo.get(key, ("free",)):
            return False, f"the support law at {key[0]} {key[1]} differs between the operands"
    fixed = lambda side_: Counter((f["kind"], f["id"], f["value_bits"]) for f in side_["fixed_loads"])
    if fixed(s) != fixed(o):
        return False, "the fixed-load inventories differ, which is outside this policy"
    return True, "the same restraints and devices, and equal fixed-load inventories"


def applied_actions(side_):
    out = []
    prims = {p["id"]: p for p in side_["primitives"]}
    if side_["ledger"] is None:
        uses = [(p, ONE_BITS) for p in side_["primitives"]]
    else:
        uses = [(prims[e["primitive"]], e["factor_bits"]) for e in side_["ledger"]]
    for p, factor_bits in uses:
        out.append(("primitive", p["category"], p["target"], p["direction"], p["dimension"], p["value_bits"],
                    p["unit"], factor_bits))
    for r in side_["pressure_regions"]:
        terminals = tuple(sorted((t["node"], t["closure_transfer"]) for t in r["terminals"]))
        out.append(("pressure_region", tuple(sorted(r["pipe_members"])), tuple(sorted(r["connector_members"])),
                    terminals, r["pressure_bits"]))
    if side_["pressure_elongation"] and side_["pressure_regions"]:
        out.append(("pressure_elongation",))
    for g in side_["generated"]:
        extent = g.get("extent") or ["0", "1"]
        out.append(("generated", g["kind"], g["target"], (norm_number(extent[0]), norm_number(extent[1])),
                    norm_mapping(g["inputs"]), norm_mapping(g["mass_basis"])))
    for m in side_["motions"]:
        if is_zero_bits(m["value_bits"]):
            continue                                  # zero of either sign, authored or defaulted: no increment
        out.append(("motion", m["support"], m["dof"], m["field"], m["value_bits"]))
    for st in side_["member_states"]:
        if state_is_established_zero(st):
            continue
        out.append(("member_state", st["member"], st["kind"], norm_mapping(st["operands"]),
                    norm_mapping(st.get("fit") or {"kind": "none"})))
    return out


def state_is_established_zero(st):
    ops, kind = st["operands"], st["kind"]
    zero = lambda v: F(v) == 0
    if kind == "unchanged_reference":
        thermal_zero = True
    elif kind == "explicit_interval_strain":
        thermal_zero = zero(ops["strain"])
    elif kind == "constant_alpha_interval":
        thermal_zero = zero(ops["alpha"]) or zero(ops["interval"])
    elif kind == "free_length_state":
        thermal_zero = F(ops["installation_temperature"]) == F(ops["operating_temperature"])
    else:
        raise ValueError(kind)
    fit = st.get("fit") or {"kind": "none"}
    fit_zero = (fit["kind"] == "none" or (fit["kind"] == "fit_strain" and zero(fit["strain"]))
                or (fit["kind"] == "natural_length_change" and zero(fit["change"])))
    return thermal_zero and fit_zero


def g1_holds(s, o):
    have = Counter(applied_actions(o))
    for action, n in Counter(applied_actions(s)).items():
        if have[action] < n:
            return False, f"the subtrahend's {action[0].replace('_', ' ')} is not contained in the minuend"
    return True, "every action of the subtrahend is in the minuend, with multiplicity"


# The stiffness part (G-2): its one definition point, with the two readings side by side.
#   S (specified, and the one every `admission` entry is under): the resolved values AND the selection.
#   R (resolved values only): the selection kind, case basis label and selection record left out.
STIFFNESS_RESOLVED = ("E", "nu", "G", "material", "constitutive_basis", "section")
STIFFNESS_SELECTION = ("selection_kind", "case_basis_label", "selection_record")
STIFFNESS_READINGS = {"S": STIFFNESS_RESOLVED + STIFFNESS_SELECTION, "R": STIFFNESS_RESOLVED}


def g2_holds(s, o, reading="S"):
    fields = STIFFNESS_READINGS[reading]
    if set(s["basis"]) != set(o["basis"]):
        return False, "the operands' members differ in their modulus records"
    for member in s["basis"]:
        for f in fields:
            if norm_mapping(s["basis"][member].get(f)) != norm_mapping(o["basis"][member].get(f)):
                return False, f"member {member} differs in {f}"
    return True, "one modulus basis for every member"


def verdict_under(s, o, reading):
    r, _ = restraint_holds(s, o)
    g1, _ = g1_holds(s, o)
    g2, _ = g2_holds(s, o, reading)
    return ("refused_restraint" if not r else "refused_g1" if not g1 else "refused_g2" if not g2
            else "admitted"), g2


def stiffness_readings(s, o):
    out = {}
    for reading in STIFFNESS_READINGS:
        verdict, g2 = verdict_under(s, o, reading)
        out[reading] = {"verdict": verdict, "g2_holds": g2}
    out["differ"] = out["S"]["verdict"] != out["R"]["verdict"]
    COUNTS["stiffness_readings_differ"] += out["differ"]
    return out


def admission(s, o, reason, gate_only, gate_only_reason):
    r, r_why = restraint_holds(s, o)
    g1, g1_why = g1_holds(s, o)
    g2, g2_why = g2_holds(s, o, "S")
    verdict = ("refused_restraint" if not r else "refused_g1" if not g1 else "refused_g2" if not g2
               else "admitted")
    assert (verdict, g2) == verdict_under(s, o, "S")
    COUNTS[f"verdict_{verdict}"] += 1
    return {"verdict": verdict, "reason": reason,
            "gates": {"restraint": {"holds": r, "why": r_why}, "g1": {"holds": g1, "why": g1_why},
                      "g2": {"holds": g2, "why": g2_why}},
            "gate_only": gate_only, "gate_only_reason": gate_only_reason}


# ==========================================================================================================
# Carried rows: the frozen G-1 rows (19) and numerical pairs (17)
# ==========================================================================================================

FROZEN_TO_COMPLETED = {"published": "admitted", "refused_g1": "refused_g1", "refused_g2": "refused_g2",
                       "refused_g1_and_g2": "refused_g1"}
CARRIED_GATE_ONLY = "Carried reference case. Exact-route document reachability has not been rechecked."
FROZEN_REFERENCE_CONFIGURATION = "frozen:shared_reference"
FROZEN_INSTALLATION_K = F(300)   # invented: the shared reference configuration's resolved installation temperature


def frozen_state(temperature):
    e300, e700 = FROZEN_POINTS[F(300)], FROZEN_POINTS[F(700)]
    return {"member": "AB", "kind": "free_length_state",
            "operands": {"installation_temperature": q(FROZEN_INSTALLATION_K),
                         "operating_temperature": q(temperature),
                         "expansion_law": {"definition": "instantaneous_coefficient_table", "datum": "300",
                                           "points": [["300", q(e300[2])], ["700", q(e700[2])]],
                                           "interpolation": "linear"}},
            "fit": {"kind": "none"}, "reference_configuration": FROZEN_REFERENCE_CONFIGURATION}


def next_binary64_up(x: F) -> F:
    y = float(x)
    assert F(y) == x
    n = struct.unpack(">q", struct.pack(">d", y))[0]
    return F(struct.unpack(">d", struct.pack(">q", n + 1))[0])


def next_binary64_away(x: F) -> F:
    return -next_binary64_up(-x) if x < 0 else next_binary64_up(x)


def carried_gate_rows():
    m = "M1"
    weight = frozen_weights(m)[0]
    weight_dup = dict(weight, id=weight["id"] + ":dup")
    thermal = frozen_thermal(m)
    occasional = primitive("f:B:y", "occasional", "node:B", "global_y", "force", F(625, 2), "N")
    weight_other = primitive(weight["id"], "weight", "node:B", "global_z", "force", F(-3687, 2), "N")
    weight_ulp = primitive(weight["id"], "weight", "node:B", "global_z", "force",
                           next_binary64_away(F(weight["value"])), "N")
    imposed = primitive("d:C:z", "imposed_displacement", "support:C", "global_z", "length", F(-3, 512), "m")
    motion = {"support": "anchor:C", "dof": "UZ", "field": "rigid_boundary", "value": q(F(-3, 512)),
              "value_bits": rational_bits(F(-3, 512)), "authored": True}
    preload = {"kind": "constant_effort_force", "id": "support:B/FZ", "value_bits": rational_bits(1500)}
    state, state_ulp = frozen_state(600), frozen_state(next_binary64_up(F(600)))

    def c(cid, prims, p=FROZEN_P, on=True, label="REF", **kw):
        return side(cid, primitives=prims, regions=frozen_regions(m, p), elongation=on,
                    supports=anchor_supports(FROZEN_MODELS[m]["anchors"]), basis=frozen_basis(m, label), **kw)

    rows = [
        ("contained", c("S", [weight]), c("O", [weight] + thermal), "published",
         "every subtrahend load, region and elongation is in the minuend"),
        ("sustained_only_load", c("S", [weight, occasional]), c("O", [weight] + thermal), "refused_g1",
         "the subtrahend's occasional force is not in the minuend"),
        ("twice_in_sustained_once_in_operating", c("S", [weight, weight_dup]), c("O", [weight] + thermal),
         "refused_g1", "the same load twice in the subtrahend and once in the minuend"),
        ("twice_in_both", c("S", [weight, weight_dup]), c("O", [weight, weight_dup] + thermal), "published",
         "the same load twice in both"),
        ("same_target_different_magnitude", c("S", [weight]), c("O", [weight_other] + thermal), "refused_g1",
         "another magnitude is another action"),
        ("same_target_magnitude_one_ulp_apart", c("S", [weight]), c("O", [weight_ulp] + thermal), "refused_g1",
         "magnitudes one binary64 apart are different actions"),
        ("operands_swapped", c("S", [weight] + thermal), c("O", [weight]), "refused_g1",
         "the thermal primitives of the subtrahend are not in the minuend"),
        ("no_primitives_in_sustained", c("S", []), c("O", thermal), "published",
         "the subtrahend applies only its regions and elongation, both in the minuend"),
        ("fails_g1_and_g2", c("S", [weight, occasional]), c("O", [weight] + thermal, label="HOT"),
         "refused_g1_and_g2", "G-1 fails (a subtrahend-only load) before G-2 is reached"),
        ("pressure_differs", c("S", [weight]), c("O", [weight] + thermal, p=FROZEN_P_CHANGED), "refused_g1",
         "a region with another pressure is another action"),
        ("pressure_absent_in_operating", c("S", [weight]), c("O", [weight] + thermal, p=0), "refused_g1",
         "the subtrahend's regions are not in the minuend"),
        ("elongation_off_sustained_on_operating", c("S", [weight], on=False), c("O", [weight] + thermal, on=True),
         "published", "elongation is an action of its own; switching it on in the minuend adds an action"),
        ("elongation_on_sustained_off_operating", c("S", [weight], on=True), c("O", [weight] + thermal, on=False),
         "refused_g1", "the subtrahend's elongation is not in the minuend"),
        ("support_motion_as_primitive_in_sustained_only", c("S", [weight, imposed]), c("O", [weight] + thermal),
         "refused_g1", "the subtrahend's imposed-displacement primitive is not in the minuend"),
        ("support_motion_as_support_record_in_sustained_only", c("S", [weight], motions=[motion]),
         c("O", [weight] + thermal), "refused_g1",
         "the subtrahend's non-zero motion on the same rigid boundary is not in the minuend"),
        ("member_state_in_sustained_only", c("S", [weight], member_states=[state]), c("O", [weight] + thermal),
         "refused_g1", "the subtrahend's non-zero free-strain state is not in the minuend"),
        ("member_state_same_authored_temperature", c("S", [weight], member_states=[state]),
         c("O", [weight] + thermal, member_states=[state]), "published",
         "equal resolved operands: the same state"),
        ("member_state_temperature_one_ulp_apart", c("S", [weight], member_states=[state]),
         c("O", [weight] + thermal, member_states=[state_ulp]), "refused_g1",
         "operating temperatures one binary64 apart are different states"),
        ("constant_effort_preload_in_both", c("S", [weight], fixed_loads=[preload]),
         c("O", [weight] + thermal, fixed_loads=[preload]), "published",
         "a constant-effort force present in both is a fixed load, not an action; the inventories are equal"),
    ]
    out = []
    for rid, s, o, frozen, reason in rows:
        adm = admission(s, o, reason, False, CARRIED_GATE_ONLY)
        if adm["verdict"] != FROZEN_TO_COMPLETED[frozen]:
            raise AssertionError(f"carried row {rid}: {adm['verdict']} against frozen {frozen}")
        out.append({"id": rid, "frozen_outcome": frozen, "sustained": s, "operating": o, "admission": adm,
                    "stiffness_readings": stiffness_readings(s, o),
                    "mechanical": {"kind": "none", "meaning": "the frozen package gives no numerical result for "
                                                              "its G-1 rows"}})
        COUNTS["carried_gate_rows"] += 1
    return out


def frozen_pairs():
    out = []
    for m in ("M0", "M1", "M2"):
        for o_on, s_on in ((False, False), (True, True), (True, False)):
            out.append((f"{m}/equal_pressure/operating_{'on' if o_on else 'off'}_sustained_{'on' if s_on else 'off'}",
                        m, {"basis": "REF", "p": FROZEN_P, "on": s_on, "thermal": False},
                        {"basis": "REF", "p": FROZEN_P, "on": o_on, "thermal": True}))
    for m in ("M1", "M2"):
        for o_on, s_on in ((False, False), (True, True), (True, False)):
            out.append((f"{m}/pressure_change/operating_{'on' if o_on else 'off'}_sustained_{'on' if s_on else 'off'}",
                        m, {"basis": "REF", "p": FROZEN_P, "on": s_on, "thermal": False},
                        {"basis": "REF", "p": FROZEN_P_CHANGED, "on": o_on, "thermal": True}))
    out.append(("M1/g2/mixed_basis", "M1", {"basis": "REF", "p": FROZEN_P, "on": True, "thermal": False},
                {"basis": "HOT", "p": FROZEN_P, "on": True, "thermal": True}))
    out.append(("M1/g2/same_basis_twin", "M1", {"basis": "HOT", "p": FROZEN_P, "on": True, "thermal": False},
                {"basis": "HOT", "p": FROZEN_P, "on": True, "thermal": True}))
    return out


def frozen_expected(pid):
    """The frozen outcome of each pair, as the frozen README's pair table states it."""
    if pid == "M1/g2/mixed_basis":
        return "refused_g2"
    return "refused_g1" if "/pressure_change/" in pid else "published"


def pair_side(model_id, case, operating):
    return side("O" if operating else "S",
                primitives=frozen_weights(model_id) + (frozen_thermal(model_id) if operating else []),
                regions=frozen_regions(model_id, case["p"]), elongation=case["on"],
                supports=anchor_supports(FROZEN_MODELS[model_id]["anchors"]),
                basis=frozen_basis(model_id, case["basis"]))


def nodal_minus(a, b):
    return {n: (vminus(a[n][0], b[n][0]), vminus(a[n][1], b[n][1])) for n in a}


def carried_pairs():
    out = []
    for pid, m, s, o in frozen_pairs():
        frozen = frozen_expected(pid)
        s_side, o_side = pair_side(m, s, False), pair_side(m, o, True)
        if frozen == "published":
            reason = "O retains S's authored actions and adds thermal primitives"
        elif frozen == "refused_g1":
            reason = "the subtrahend's regions carry another pressure than the minuend's"
        else:
            reason = "the operands select different modulus bases"
        adm = admission(s_side, o_side, reason, False, CARRIED_GATE_ONLY)
        if adm["verdict"] != FROZEN_TO_COMPLETED[frozen]:
            raise AssertionError(f"carried pair {pid}: {adm['verdict']} against frozen {frozen}")
        sw, sn = frozen_case_rows(m, s)
        ow, on_ = frozen_case_rows(m, o)
        diff = {k: ow[k] - sw[k] for k in sorted(ow)}
        same_operator = s["basis"] == o["basis"]
        dn = nodal_minus(on_, sn)
        assert frame_balanced(m, diff, dn), "equilibrium of the difference"
        COUNTS["difference_equilibrium"] += 1
        mech = {"kind": "linear_increment" if same_operator else "state_difference",
                "meaning": ("the response of the common operator to the difference of the two load vectors"
                            if same_operator else
                            "the difference of two independently solved states on different moduli; no "
                            "superposition claim"),
                "frozen_values_field": {"published": "difference", "refused_g1": "undeclared_subtraction",
                                        "refused_g2": "withheld_would_be_difference"}[frozen],
                "rows": {k: entry(v, row_unit(k)) for k, v in diff.items()}}
        if same_operator:
            sj, _ = frozen_case_rows(m, s, "J")
            oj, _ = frozen_case_rows(m, o, "J")
            alt = {k: oj[k] - sj[k] for k in sorted(oj) if oj[k] - sj[k] != diff[k]}
            if s["p"] == o["p"]:
                assert not alt
                mech["reading_J_alternative"] = "analytically identical to reading W on every row"
            else:
                assert alt
                mech["reading_J_alternative"] = {k: entry(v, row_unit(k)) for k, v in alt.items()}
            COUNTS["reading_J_pairs"] += 1
        COUNTS["carried_pair_rows"] += len(diff)
        out.append({"id": pid, "model": m, "frozen_outcome": frozen, "sustained": s_side, "operating": o_side,
                    "admission": adm, "stiffness_readings": stiffness_readings(s_side, o_side), "mechanical": mech})
        COUNTS["carried_pairs"] += 1
    return out


# ==========================================================================================================
# The added rows: one straight member AB along +x, axial response only
# ==========================================================================================================

W_L = F(1)
W_E = F(2 ** 20)
W_NU = F(1, 4)
W_D, W_WALL = F(1, 4), F(1, 32)
W_RI = W_D / 2 - W_WALL
W_AREA = (W_D / 2) ** 2 - W_RI ** 2        # times pi
W_EA = Exact({1: W_E * W_AREA})            # 7168 pi N
assert W_RI == F(3, 32) and W_AREA == F(7, 1024) and W_EA == Exact({1: 7168})


def bar_solve(xs, members, supports, nodal):
    """Exact axial bar.

    xs: {node: x}; members: [(id, a, b, EA Exact, free strain Exact)];
    supports: {node: ("rigid", value) | ("spring", k Exact, base Exact)}; nodal: {node: Exact force along +x}.
    Returns u, member forces N (tension positive), rigid reactions on the pipe, spring forces on the pipe.
    """
    order = list(xs)
    unknown = [n for n in order if supports.get(n, ("free",))[0] != "rigid"]
    known = {n: Exact.of(supports[n][1]) for n in order if supports.get(n, ("free",))[0] == "rigid"}
    index = {n: i for i, n in enumerate(unknown)}
    size = len(unknown)
    kmat = [[ZERO] * size for _ in range(size)]
    rhs = [Exact.of(nodal.get(n, 0)) for n in unknown]
    for _mid, a, b, ea, strain in members:
        k = ea / Exact.of(xs[b] - xs[a])
        h = ea * strain
        for n, sgn in ((a, -1), (b, 1)):
            if n in index:
                rhs[index[n]] = rhs[index[n]] + sgn * h
        for n1, s1 in ((a, 1), (b, -1)):
            for n2, s2 in ((a, 1), (b, -1)):
                term = s1 * s2 * k
                if n1 in index and n2 in index:
                    kmat[index[n1]][index[n2]] = kmat[index[n1]][index[n2]] + term
                elif n1 in index:
                    rhs[index[n1]] = rhs[index[n1]] - term * known[n2]
    for n, spec in supports.items():
        if spec[0] == "spring":
            kmat[index[n]][index[n]] = kmat[index[n]][index[n]] + spec[1]
            rhs[index[n]] = rhs[index[n]] + spec[1] * Exact.of(spec[2])
    # every stiffness here is a rational multiple of pi
    if size:
        flat = [[x.c.get(1, F(0)) for x in row] for row in kmat]
        assert all(Exact({1: f}) == x for row, frow in zip(kmat, flat) for f, x in zip(frow, row))
        sol = solve_scaled(flat, 1, rhs)
    else:
        sol = []
    u = dict(known)
    u.update({n: v for n, v in zip(unknown, sol)})
    forces = {mid: ea * ((u[b] - u[a]) / Exact.of(xs[b] - xs[a]) - strain) for mid, a, b, ea, strain in members}
    reactions = {}
    for n in known:
        r = -Exact.of(nodal.get(n, 0))
        for mid, a, b, _ea, _s in members:
            r = r + (-forces[mid] if n == a else forces[mid] if n == b else ZERO)
        reactions[n] = r
    springs = {n: -spec[1] * (u[n] - Exact.of(spec[2])) for n, spec in supports.items() if spec[0] == "spring"}
    total = sum((Exact.of(v) for v in nodal.values()), ZERO) + sum(reactions.values(), ZERO) + \
        sum(springs.values(), ZERO)
    assert total.is_zero(), "axial equilibrium"
    COUNTS["bar_solves"] += 1
    return u, forces, reactions, springs


def consistent_axial(q_line, start, end, length):
    """Consistent nodal loads of a uniform axial line load over [start, end] of a member [0, length]."""
    a = q_line * ((end - start) - (end ** 2 - start ** 2) / (2 * length))
    b = q_line * (end ** 2 - start ** 2) / (2 * length)
    return a, b


def wall_form_bar(regions, nu_of, ea_of, bore_pi, elongation):
    """DEL-04-07's wall form on collinear members along +x: each transferring terminal carries the cap
    -p A_i s (s from the node into the member); each pressurised member the Poisson eigenstrain -2 nu p A_i / EA.
    With elongation off the closed-end free strain (1 - 2 nu) p A_i / EA is removed (reading W).
    regions: [(members [(id, from, to)], terminals {node: closure}, p)]. Returns (nodal, strains)."""
    nodal, strains = Counter(), {}
    for members, terminals, p in regions:
        for mid, a, b in members:
            pa_i = p * bore_pi
            strain = -2 * nu_of[mid] * pa_i / ea_of[mid]
            if not elongation:
                strain = strain - (1 - 2 * nu_of[mid]) * pa_i / ea_of[mid]
            strains[mid] = strains.get(mid, ZERO) + strain
        first, last = members[0], members[-1]
        for node, s in ((first[1], 1), (last[2], -1)):
            if terminals[node] == "transfers_to_wall":
                nodal[node] = nodal.get(node, ZERO) + (-p * bore_pi * s)
    return dict(nodal), strains


W_ANCHOR = anchor_supports(["A"])
W_BASIS = {"AB": {"E": q(W_E), "nu": q(W_NU), "G": q(W_E / (2 * (1 + W_NU))), "material": "invented_w",
                  "constitutive_basis": "linear_elastic", "selection_kind": "exact_basis_reference",
                  "case_basis_label": "W", "selection_record": {"kind": "exact_basis_reference",
                                                                "selection_temperature": "293",
                                                                "interpolation_fraction": None,
                                                                "selected_E": q(W_E), "selected_nu": q(W_NU)},
                  "section": {"outside_diameter": q(W_D), "wall": q(W_WALL), "mill_tolerance": "0",
                              "corrosion_allowance": "0"}}}
SEVEN_PI = Exact({1: 7})
SEVEN_PI_LOAD = primitive("f:B:x", "occasional", "node:B", "global_x", "force", None, "N",
                          value_text="7*pi", value_float=nearest(SEVEN_PI))
LINE_STOP = {"support": "line_stop:B", "dof": "UX", "law": "rigid", "identity": "line_stop:B",
             "participation": "active", "parameters": {}}
FREE_B = {"support": "line_stop:B", "dof": "UX", "law": "free", "identity": "line_stop:B", "participation": None,
          "parameters": {}}


def spring_at_b(identity):
    k = W_EA / Exact.of(W_L)
    return {"support": "spring:B", "dof": "UX", "law": "spring", "identity": identity, "participation": "active",
            "parameters": {"stiffness": k.text(), "stiffness_bits": bits(nearest(k))}}


def w_region(members, terminals, p):
    return {"pipe_members": members, "connector_members": [],
            "terminals": [{"node": n, "closure_transfer": c} for n, c in terminals],
            "pressure": q(p), "pressure_bits": rational_bits(p)}


def wind(extent, pressure, shape, diameter):
    g = {"kind": "wind", "target": "AB",
         "inputs": {"direction": "global_x", "pressure": q(pressure), "pressure_bits": rational_bits(pressure),
                    "shape_factor": q(shape), "exposed_diameter": q(diameter)},
         "mass_basis": {}}
    if extent is not None:
        g["extent"] = [q(extent[0]), q(extent[1])]
    return g


def w_side(cid, **kw):
    kw.setdefault("supports", W_ANCHOR)
    kw.setdefault("basis", W_BASIS)
    return side(cid, **kw)


W_XS = {"A": F(0), "B": F(1)}
W_MEMBER = [("AB", "A", "B")]
WIND_GATE_ONLY = ("not established here whether exact-route wind generation admits a load along the member axis; "
                  "establishing it would need product code, so the row tests the policy function only")
SUPPORT_GATE_ONLY = "W6, W7a and W7b compare different model-level support definitions and are gate-only."
NOT_GATE_ONLY = "Proposed document case. Exact-route validation has not been checked here."


def added_rows():
    rows = []
    ea = W_EA
    bore = Exact({1: W_RI ** 2})           # A_i = pi r_i^2

    # W1: elongation added
    p = F(1024)
    s_nodal, s_strain = wall_form_bar([(W_MEMBER, {"A": "transfers_to_wall", "B": "transfers_to_wall"}, p)],
                                      {"AB": W_NU}, {"AB": ea}, bore, False)
    o_nodal, o_strain = wall_form_bar([(W_MEMBER, {"A": "transfers_to_wall", "B": "transfers_to_wall"}, p)],
                                      {"AB": W_NU}, {"AB": ea}, bore, True)
    us, *_ = bar_solve(W_XS, [("AB", "A", "B", ea, s_strain["AB"])], {"A": ("rigid", 0)}, s_nodal)
    uo, *_ = bar_solve(W_XS, [("AB", "A", "B", ea, o_strain["AB"])], {"A": ("rigid", 0)}, o_nodal)
    d = uo["B"] - us["B"]
    assert d == Exact.of(F(9, 14336))
    assert d == (1 - 2 * W_NU) * p * bore * W_L / ea, "closed-end free strain times L"
    region = w_region(["AB"], [("A", "transfers_to_wall"), ("B", "transfers_to_wall")], p)
    rows.append(("W1", "elongation added",
                 w_side("S", regions=[region], elongation=False), w_side("O", regions=[region], elongation=True),
                 "elongation is an action of its own, added in the minuend", False, NOT_GATE_ONLY,
                 {"kind": "linear_increment", "values": {"u/B/UX": entry(d, "m")}}))

    # W2: shared-terminal grouping
    xs2 = {"A": F(0), "B": F(1), "C": F(2)}
    nus = {"AB": F(-1, 8), "BC": F(1, 4)}
    eas = {"AB": ea, "BC": ea}
    ab = ([("AB", "A", "B")], {"A": "transfers_to_wall", "B": "transfers_to_wall"}, p)
    bc = ([("BC", "B", "C")], {"B": "transfers_to_wall", "C": "transfers_to_wall"}, p)
    res = {}
    for name, regs in (("S", [ab]), ("O", [ab, bc])):
        nodal, strains = wall_form_bar(regs, nus, eas, bore, True)
        u, n_force, *_ = bar_solve(xs2, [("AB", "A", "B", ea, strains.get("AB", ZERO)),
                                         ("BC", "B", "C", ea, strains.get("BC", ZERO))], {"A": ("rigid", 0)}, nodal)
        res[name] = (u, n_force)
    du_b = res["O"][0]["B"] - res["S"][0]["B"]
    du_c = res["O"][0]["C"] - res["S"][0]["C"]
    assert du_b.is_zero() and du_c == Exact.of(F(9, 14336))
    # BC's mechanical force S + (1 - 2 nu) p A_i, with S its effective force (wall force less p A_i)
    n_mech_bc = res["O"][1]["BC"] - p * bore + (1 - 2 * nus["BC"]) * p * bore
    assert n_mech_bc == p * bore / 2
    r_ab = w_region(["AB"], [("A", "transfers_to_wall"), ("B", "transfers_to_wall")], p)
    r_bc = w_region(["BC"], [("B", "transfers_to_wall"), ("C", "transfers_to_wall")], p)
    w2_material = {"AB": "invented_w2_ab_auxetic", "BC": "invented_w2_bc"}
    basis2 = {mid: dict(W_BASIS["AB"], nu=q(nus[mid]), G=q(W_E / (2 * (1 + nus[mid]))), material=w2_material[mid],
                        selection_record=dict(W_BASIS["AB"]["selection_record"], selected_nu=q(nus[mid])))
              for mid in ("AB", "BC")}
    rows.append(("W2", "shared-terminal grouping",
                 w_side("S", regions=[r_ab], elongation=True, basis=basis2),
                 w_side("O", regions=[r_ab, r_bc], elongation=True, basis=basis2),
                 "BC's region is an added action; AB's region, with its terminals, is in both", False,
                 NOT_GATE_ONLY + " Reachability with the auxetic AB material (nu = -1/8) has not been established.",
                 {"kind": "linear_increment",
                  "values": {"u/B/UX": entry(du_b, "m"), "u/C/UX": entry(du_c, "m"),
                             "axial/BC/N_mech (minuend)": entry(n_mech_bc, "N")},
                  "note": "the specification's formation note (a grouped binary64 force at B differing from the "
                          "separately formed extra region by -2^-49 N) concerns a product's binary64 formation "
                          "order and is not derived here"}))

    # W3: operating-only rigid motion
    delta = F(1, 1024)
    u0, n0, r0, _ = bar_solve(W_XS, [("AB", "A", "B", ea, ZERO)], {"A": ("rigid", 0), "B": ("rigid", 0)}, {})
    u1, n1, r1, _ = bar_solve(W_XS, [("AB", "A", "B", ea, ZERO)], {"A": ("rigid", 0), "B": ("rigid", delta)}, {})
    assert n1["AB"] - n0["AB"] == SEVEN_PI and r1["A"] - r0["A"] == -SEVEN_PI
    motion = {"support": "line_stop:B", "dof": "UX", "field": "rigid_boundary", "value": q(delta),
              "value_bits": rational_bits(delta), "authored": True}
    rows.append(("W3", "operating-only rigid motion",
                 w_side("S", supports=W_ANCHOR + [LINE_STOP]),
                 w_side("O", supports=W_ANCHOR + [LINE_STOP], motions=[motion]),
                 "the same rigid line stop; the subtrahend's zero motion is no increment, the minuend's motion is "
                 "an added action", False, NOT_GATE_ONLY,
                 {"kind": "linear_increment",
                  "values": {"u/B/UX": entry(u1["B"] - u0["B"], "m"), "axial/AB/N": entry(n1["AB"] - n0["AB"], "N"),
                             "reaction/A/FX": entry(r1["A"] - r0["A"], "N")}}))

    # W4: operating-only spring base
    k = ea / Exact.of(W_L)
    u0, _, _, f0 = bar_solve(W_XS, [("AB", "A", "B", ea, ZERO)], {"A": ("rigid", 0), "B": ("spring", k, 0)}, {})
    u1, _, _, f1 = bar_solve(W_XS, [("AB", "A", "B", ea, ZERO)], {"A": ("rigid", 0), "B": ("spring", k, delta)}, {})
    assert u1["B"] - u0["B"] == Exact.of(F(1, 2048)) and f1["B"] - f0["B"] == Exact({1: F(7, 2)})
    base_motion = {"support": "spring:B", "dof": "UX", "field": "spring_base", "value": q(delta),
                   "value_bits": rational_bits(delta), "authored": True}
    rows.append(("W4", "operating-only spring base",
                 w_side("S", supports=W_ANCHOR + [spring_at_b("spring:K")]),
                 w_side("O", supports=W_ANCHOR + [spring_at_b("spring:K")], motions=[base_motion]),
                 "the same spring, participation and stiffness; the base movement is an added excitation", False,
                 NOT_GATE_ONLY,
                 {"kind": "linear_increment",
                  "values": {"u/B/UX": entry(u1["B"] - u0["B"], "m"),
                             "spring/B/FX": entry(f1["B"] - f0["B"], "N")}}))

    # W5: wind extent substitution
    wq = F(32) * 1 * F(1, 4)
    assert wq == 8
    s_a, s_b = consistent_axial(wq, F(0), F(1, 2), W_L)
    o_a, o_b = consistent_axial(wq, F(1, 2), F(1), W_L)
    assert (s_a, s_b, o_a, o_b) == (3, 1, 1, 3)
    us, *_ = bar_solve(W_XS, [("AB", "A", "B", ea, ZERO)], {"A": ("rigid", 0)}, {"A": s_a, "B": s_b})
    uo, *_ = bar_solve(W_XS, [("AB", "A", "B", ea, ZERO)], {"A": ("rigid", 0)}, {"A": o_a + 3, "B": o_b + 1})
    ea_du = ea * (uo["B"] - us["B"])
    # closed form for a cantilever: EA u_B = integral of x q(x) dx
    assert ea * us["B"] == Exact.of(wq * (F(1, 2) ** 2) / 2)
    assert ea_du == Exact.of(3)
    nodal_only = ea * bar_solve(W_XS, [("AB", "A", "B", ea, ZERO)], {"A": ("rigid", 0)}, {"A": 3, "B": 1})[0]["B"]
    assert nodal_only == Exact.of(1)
    nodal_a = primitive("f:A:x", "occasional", "node:A", "global_x", "force", 3, "N")
    nodal_b = primitive("f:B:x", "occasional", "node:B", "global_x", "force", 1, "N")
    rows.append(("W5", "wind extent substitution",
                 w_side("S", generated=[wind((0, F(1, 2)), 32, 1, F(1, 4))]),
                 w_side("O", generated=[wind((F(1, 2), 1), 32, 1, F(1, 4))], primitives=[nodal_a, nodal_b]),
                 "the subtrahend's wind recipe (extent [0, 1/2]) is not in the minuend; nodal forces with the same "
                 "bits are not the same action", True, WIND_GATE_ONLY,
                 {"kind": "linear_increment",
                  "values": {"EA*u/B/UX": entry(ea_du, "N*m"), "u/B/UX": entry(uo["B"] - us["B"], "m"),
                             "EA*u/B/UX (the minuend's nodal forces alone)": entry(nodal_only, "N*m")},
                  "consistent_nodal_forces_N": {"subtrahend_wind": [q(s_a), q(s_b)], "minuend_wind": [q(o_a), q(o_b)]},
                  "divergence": "a common linear operator gives a response to any difference of load vectors, "
                                "which cannot tell a changed wind extent disguised by nodal loads"}))

    # W6: free DOF against one fixed at zero
    us, *_ = bar_solve(W_XS, [("AB", "A", "B", ea, ZERO)], {"A": ("rigid", 0)}, {"B": SEVEN_PI})
    uo, *_ = bar_solve(W_XS, [("AB", "A", "B", ea, ZERO)], {"A": ("rigid", 0), "B": ("rigid", 0)}, {"B": SEVEN_PI})
    assert us["B"] == Exact.of(F(1, 1024)) and uo["B"].is_zero()
    rows.append(("W6", "free DOF against one fixed at zero",
                 w_side("S", supports=W_ANCHOR + [FREE_B], primitives=[SEVEN_PI_LOAD]),
                 w_side("O", supports=W_ANCHOR + [LINE_STOP], primitives=[SEVEN_PI_LOAD]),
                 "the restraints differ: B.UX is free in the subtrahend and rigid in the minuend", True,
                 SUPPORT_GATE_ONLY,
                 {"kind": "state_difference",
                  "values": {"u/B/UX (subtrahend)": entry(us["B"], "m"), "u/B/UX (minuend)": entry(uo["B"], "m"),
                             "u/B/UX (minuend - subtrahend)": entry(uo["B"] - us["B"], "m")}}))

    # W7a: removed spring
    us, *_ = bar_solve(W_XS, [("AB", "A", "B", ea, ZERO)], {"A": ("rigid", 0), "B": ("spring", k, 0)}, {"B": SEVEN_PI})
    uo, *_ = bar_solve(W_XS, [("AB", "A", "B", ea, ZERO)], {"A": ("rigid", 0)}, {"B": SEVEN_PI})
    assert us["B"] == Exact.of(F(1, 2048)) and uo["B"] == Exact.of(F(1, 1024))
    rows.append(("W7a", "removed spring",
                 w_side("S", supports=W_ANCHOR + [spring_at_b("spring:K")], primitives=[SEVEN_PI_LOAD]),
                 w_side("O", primitives=[SEVEN_PI_LOAD]),
                 "the restraints differ: the spring at B.UX is removed in the minuend", True, SUPPORT_GATE_ONLY,
                 {"kind": "state_difference",
                  "values": {"u/B/UX (subtrahend)": entry(us["B"], "m"), "u/B/UX (minuend)": entry(uo["B"], "m"),
                             "u/B/UX (minuend - subtrahend)": entry(uo["B"] - us["B"], "m")}}))

    # W7b: different spring at the same node (common +7 pi N at B, as in W7a)
    u1, *_ = bar_solve(W_XS, [("AB", "A", "B", ea, ZERO)], {"A": ("rigid", 0), "B": ("spring", k, 0)}, {"B": SEVEN_PI})
    rows.append(("W7b", "different spring at the same node",
                 w_side("S", supports=W_ANCHOR + [spring_at_b("spring:K1")], primitives=[SEVEN_PI_LOAD]),
                 w_side("O", supports=W_ANCHOR + [spring_at_b("spring:K2")], primitives=[SEVEN_PI_LOAD]),
                 "a different spring is a different case: the identities differ", True, SUPPORT_GATE_ONLY,
                 {"kind": "state_difference",
                  "values": {"u/B/UX (each operand)": entry(u1["B"], "m"),
                             "u/B/UX (minuend - subtrahend)": entry(u1["B"] - u1["B"], "m")},
                  "note": "the specification gives no load for W7b; the common +7 pi N of W7a is used, and the "
                          "difference is zero with or without it"}))

    # W8: two recipes, equal values
    p_low = F(0.1)                       # the binary64 nearest 0.1
    formed = {}
    for name, (pp, shape) in (("S", (0.1, 10.0)), ("O", (1.0, 1.0))):
        orders = {(pp * shape) * 0.25, pp * (shape * 0.25)}
        assert orders == {0.25}, orders
        formed[name] = F(0.25)
    exact_s = p_low * 10 * F(1, 4)
    exact_o = F(1) * 1 * F(1, 4)
    _, fb_s = consistent_axial(exact_s, F(0), F(1), W_L)
    _, fb_o = consistent_axial(exact_o, F(0), F(1), W_L)
    assert fb_o - fb_s == F(-1, 2 ** 57)
    rows.append(("W8", "two recipes, equal values",
                 w_side("S", generated=[wind(None, p_low, 10, F(1, 4))]),
                 w_side("O", generated=[wind(None, 1, 1, F(1, 4))]),
                 "conservative: the recipes differ (pressure and shape factor), although their formed intensities "
                 "are equal", True, WIND_GATE_ONLY,
                 {"kind": "linear_increment",
                  "values": {"EA*u/B/UX (formed loads)": entry((formed["O"] - formed["S"]) * W_L ** 2 / 2, "N*m"),
                             "consistent force at B, exact recipe difference": entry(fb_o - fb_s, "N")},
                  "formed_intensity_N_per_m": {"subtrahend": q(formed["S"]), "minuend": q(formed["O"])},
                  "divergence": "the conservative rule refuses a pair whose formed loads are equal; the exact "
                                "recipes differ by -2^-56 N/m"}))

    # W9: explicit zero against default zero
    neg_zero = {"support": "line_stop:B", "dof": "UX", "field": "rigid_boundary", "value": "0",
                "value_bits": bits(-0.0), "authored": True}
    rows.append(("W9", "explicit zero against default zero",
                 w_side("S", supports=W_ANCHOR + [LINE_STOP], motions=[neg_zero]),
                 w_side("O", supports=W_ANCHOR + [LINE_STOP]),
                 "on the same rigid line stop a zero of either sign normalizes with the default: no action", False,
                 NOT_GATE_ONLY,
                 {"kind": "linear_increment", "values": {"u/B/UX": entry(ZERO, "m")}}))

    # F1: terminal witness
    p1, e1, l1 = F(5750000), F(194750000000), F(15, 4)
    ea1, bore1 = Exact({1: e1 * AREA}), Exact({1: BORE})
    xs1 = {"A": F(0), "B": l1}
    res = {}
    for name, closure in (("S", "transfers_to_wall"), ("O", "separately_supported_or_compensated")):
        nodal, strains = wall_form_bar([([("AB", "A", "B")], {"A": "transfers_to_wall", "B": closure}, p1)],
                                       {"AB": FROZEN_NU}, {"AB": ea1}, bore1, True)
        res[name] = bar_solve(xs1, [("AB", "A", "B", ea1, strains["AB"])], {"A": ("rigid", 0)}, nodal)[0]["B"]
    d1 = res["O"] - res["S"]
    assert d1 == Exact.of(F(-64607, 81065856)) and d1 == -p1 * bore1 * l1 / ea1
    f1_basis = {"AB": {"E": q(e1), "nu": q(FROZEN_NU), "G": "76000000000", "material": "invented_steel_points_300_700",
                       "constitutive_basis": "linear_elastic", "selection_kind": "exact_basis_reference",
                       "case_basis_label": "REF", "selection_record": {"kind": "exact_basis_reference",
                                                                       "selection_temperature": "300",
                                                                       "interpolation_fraction": None,
                                                                       "selected_E": q(e1),
                                                                       "selected_nu": q(FROZEN_NU)},
                       "section": section_record()}}
    rows.append(("F1", "first reader's terminal witness",
                 w_side("S", regions=[w_region(["AB"], [("A", "transfers_to_wall"), ("B", "transfers_to_wall")], p1)],
                        elongation=True, basis=f1_basis),
                 w_side("O", regions=[w_region(["AB"], [("A", "transfers_to_wall"),
                                                        ("B", "separately_supported_or_compensated")], p1)],
                        elongation=True, basis=f1_basis),
                 "the free-end terminal's closure transfer differs, so the region is another action", False,
                 NOT_GATE_ONLY,
                 {"kind": "linear_increment", "values": {"u/B/UX": entry(d1, "m")},
                  "note": "solved with elongation on in both; the increment -p A_i L / (E A) does not depend on the "
                          "setting or on nu"}))

    # F2 and F2b: fit and installation witnesses
    alpha, datum = F(1, 2 ** 16), F(300)

    def secant_strain(installation, operating, fit):
        return (1 + alpha * (operating - installation) / (1 + alpha * (installation - datum))) * (1 + fit) - 1

    def f_state(installation, fit, rc):
        return {"member": "AB", "kind": "free_length_state",
                "operands": {"installation_temperature": q(installation), "operating_temperature": "500",
                             "expansion_law": {"definition": "secant_constant", "datum": q(datum),
                                               "constant": q(alpha)}},
                "fit": {"kind": "fit_strain", "strain": q(fit)} if fit else {"kind": "none"},
                "reference_configuration": rc}

    fixed_fixed = W_ANCHOR + [LINE_STOP]
    for rid, title, s_args, o_args, expected, reason in (
            ("F2", "first reader's fit witness", (F(300), F(1, 1024), "RC-AB"), (F(300), F(0), "RC-AB"),
             F(-8217, 8388608), "the fit differs, so the resolved free strain differs"),
            ("F2b", "installation-only variant", (F(300), F(0), "RC-AB-INSTALLED-300"),
             (F(400), F(0), "RC-AB-INSTALLED-400"), F(-205425, 134422528),
             "the installation temperature differs, so the resolved free strain differs")):
        de = secant_strain(o_args[0], F(500), o_args[1]) - secant_strain(s_args[0], F(500), s_args[1])
        assert de == expected, (rid, de)
        _, nf, *_ = bar_solve(W_XS, [("AB", "A", "B", ea, Exact.of(de))], {"A": ("rigid", 0), "B": ("rigid", 0)}, {})
        assert nf["AB"] == -ea * de
        rows.append((rid, title, w_side("S", supports=fixed_fixed, member_states=[f_state(*s_args)]),
                     w_side("O", supports=fixed_fixed, member_states=[f_state(*o_args)]), reason, False,
                     NOT_GATE_ONLY,
                     {"kind": "linear_increment",
                      "values": {"free strain increment": entry(de, "1"),
                                 "axial/AB/N on a fixed-fixed bar of the shared W section": entry(nf["AB"], "N")}}))

    # K1: the selection record alone differs (the two stiffness readings separate here)
    rows.append(selection_record_row())
    # W11: equal moduli, different expansion coefficients, the same authored temperature change in both
    rows.append(expansion_coefficient_row())
    return rows


def m1_hand_reactions(e_mod, strain):
    """README of the frozen package, 'M1 by hand', for any modulus and uniform member free strain: the in-plane
    flexibility of the released L at C gives the support force X, Y and moment Z at C (each a multiple of pi)."""
    a, b = F(21, 4), F(27, 8)
    ea, ei = e_mod * AREA, e_mod * INERTIA
    flex = [[b ** 3 / (3 * ei) + a * b * b / ei + a / ea, -a * a * b / (2 * ei), -b * b / (2 * ei) - a * b / ei],
            [-a * a * b / (2 * ei), b / ea + a ** 3 / (3 * ei), a * a / (2 * ei)],
            [-b * b / (2 * ei) - a * b / ei, a * a / (2 * ei), (a + b) / ei]]
    (x,) = gauss_jordan(flex, [[-strain * a, -strain * b, F(0)]])
    return [Exact({1: v}) for v in x]


def expansion_coefficient_row():
    """M1's geometry and the frozen section, no pressure, in both operands: the weight at B and the thermal
    primitive dT = 475/2 K on every member. S selects the invented W11 material's 250 K point, O its 650 K point:
    equal E, nu and G, alpha 5 * 2^-19 against 7 * 2^-19 per K. One operator, equal authored actions; the
    difference of the two solved states is the response to the free strain (alpha_O - alpha_S) * dT alone."""
    m = "M1"
    e_lo, g_lo, a_lo, _ = basis_moduli("W11_LO")
    e_hi, g_hi, a_hi, _ = basis_moduli("W11_HI")
    assert (e_lo, g_lo) == (e_hi, g_hi) and a_lo != a_hi
    rs, _ = frozen_case_rows(m, {"basis": "W11_LO", "p": F(0), "on": False, "thermal": True})
    ro, _ = frozen_case_rows(m, {"basis": "W11_HI", "p": F(0), "on": False, "thermal": True})
    diff = {k: ro[k] - rs[k] for k in sorted(ro)}
    strain = (a_hi - a_lo) * FROZEN_DT
    assert strain == F(475, 2 ** 19)
    # the difference is the free-strain response on the common operator, solved separately
    alone = frame_solve(m, "W11_LO", {}, {mem[0]: Exact.of(strain) for mem in FROZEN_MODELS[m]["members"]})
    for k, v in alone.items():
        assert diff[k] == v, ("W11 difference = response to (alpha_O - alpha_S) dT", k)
    # closed form from the inputs: the M1 hand formula
    x, y, z = m1_hand_reactions(e_lo, strain)
    assert [diff["reaction/C/FX"], diff["reaction/C/FY"], diff["reaction/C/MZ"]] == [x, y, z]
    assert not diff["reaction/C/FX"].is_zero()
    COUNTS["hand_formula_w11"] += 1

    def w_side_m1(cid, label):
        return side(cid, primitives=frozen_weights(m) + frozen_thermal(m), regions=[], elongation=False,
                    supports=anchor_supports(FROZEN_MODELS[m]["anchors"]), basis=frozen_basis(m, label))
    return ("W11", "equal moduli, different expansion coefficients, thermal in both",
            w_side_m1("S", "W11_LO"), w_side_m1("O", "W11_HI"),
            "the case basis label and the selection record's selection temperature differ although E, nu and G are "
            "equal on every member (stiffness reading S)",
            True,
            "not established here whether the exact route's resolver accepts a material whose points share E and G "
            "with different expansion coefficients, and records the two selections as given; establishing it would "
            "need product code. Selection of the two coefficients for the thermal primitive is an assumption of the "
            "reference calculation.",
            {"kind": "state_difference",
             "response_to": "the uniform free strain (alpha_O - alpha_S) * dT = 475/524288 on every member, on the "
                            "common operator",
             "rows": {k: entry(v, row_unit(k)) for k, v in diff.items()},
             "closed_form_check": {
                 "method": "the frozen package's M1 hand formula (release the anchor at C; in-plane flexibility "
                           "with EA = E A_n, EI = E I_n) with the free strain 475/524288 on both members",
                 "reaction/C/FX": entry(x, "N"), "reaction/C/FY": entry(y, "N"), "reaction/C/MZ": entry(z, "N*m")},
             "divergence": "In the reference calculation both operands share one elastic operator. Their identical "
                           "weight cancels. The direct-stiffness difference equals the response to the uniform free "
                           "strain 475/524288 on all 84 output rows. Under R the equal authored actions give an "
                           "admission verdict even though the calculation has a nonzero difference from the two "
                           "selected expansion coefficients. The package records it as a `state_difference` under "
                           "`rows`."})


def selection_record_row():
    """M1 as the frozen 'contained' operating case, in both operands: weights, both regions at 5.75 MPa,
    elongation on, the thermal primitive on every member. S selects the exact 300 K point (REF); O selects a
    solve temperature of 300 K, interpolated between 300 K and 700 K at fraction 0 (T300). The resolved E, nu
    and G are equal on every member, so the operator is common and the load vectors are equal."""
    m = "M1"
    assert basis_moduli("REF")[:3] == basis_moduli("T300")[:3] and basis_moduli("T300")[3] == 0
    case_s = {"basis": "REF", "p": FROZEN_P, "on": True, "thermal": True}
    case_o = {"basis": "T300", "p": FROZEN_P, "on": True, "thermal": True}
    rs, _ = frozen_case_rows(m, case_s)
    ro, _ = frozen_case_rows(m, case_o)
    diff = {k: ro[k] - rs[k] for k in sorted(ro)}
    assert all(v.is_zero() for v in diff.values()) and len(diff) == 84

    def k_side(cid, label):
        return side(cid, primitives=frozen_weights(m) + frozen_thermal(m), regions=frozen_regions(m, FROZEN_P),
                    elongation=True, supports=anchor_supports(FROZEN_MODELS[m]["anchors"]),
                    basis=frozen_basis(m, label))
    return ("K1", "equal resolved moduli, exact point against interpolation at that endpoint",
            k_side("S", "REF"), k_side("O", "T300"),
            "the selection kind, case basis label and selection record differ although E, nu and G are equal on "
            "every member (stiffness reading S). K1 is a gate-only analogue of W10. It uses M1 and synthetic "
            "selection records, including fraction 0. W10 specifies a common 1 N bar load and an endpoint record "
            "without a fraction. K1 establishes S and R's verdicts for its supplied records and zero analytical "
            "difference on the common M1 operator.", True,
            "not established here whether the exact route's resolver records a solve temperature at a table point "
            "as an interpolation at fraction 0 rather than as that point; establishing it would need product code",
            {"kind": "linear_increment",
             "values": {"u/B/UX": entry(diff["u/B/UX"], "m"), "reaction/C/FX": entry(diff["reaction/C/FX"], "N"),
                        "axial/AB/N_mech": entry(diff["axial/AB/N_mech"], "N")},
             "note": "every one of M1's 84 rows is equal in the two operands, so the increment is zero on each",
             "divergence": "the stricter stiffness policy refuses a pair whose exact mechanical difference "
                           "exists"})


def builder_witnesses():
    """Rows the builder added (README 'Builder's witnesses'): X1 and X2 for two of the verifier test's mutants
    (a ledger factor, a reference-configuration identifier); X3 the rigid-identity control; X4 the zero-law
    free-length state that stays an action; X5 two seismic recipes that differ only in mass basis."""
    rows = []
    ea = W_EA
    force = F(1024)
    load = primitive("f:B:x", "occasional", "node:B", "global_x", "force", force, "N")
    s_ledger = [{"primitive": "f:B:x", "factor": "1", "factor_bits": rational_bits(1)}]
    o_ledger = [{"primitive": "f:B:x", "factor": "3/2", "factor_bits": rational_bits(F(3, 2))}]
    us, *_ = bar_solve(W_XS, [("AB", "A", "B", ea, ZERO)], {"A": ("rigid", 0)}, {"B": force})
    uo, *_ = bar_solve(W_XS, [("AB", "A", "B", ea, ZERO)], {"A": ("rigid", 0)}, {"B": force * F(3, 2)})
    rows.append(("X1", "same stored primitive, other ledger factor",
                 w_side("S", primitives=[load], ledger=s_ledger), w_side("O", primitives=[load], ledger=o_ledger),
                 "the ledger factors' bits differ, so the referenced loads are different actions", True,
                 "not established here whether the exact route accepts a ledger on this document; the row tests the "
                 "policy function only",
                 {"kind": "linear_increment", "values": {"u/B/UX": entry(uo["B"] - us["B"], "m")},
                  "divergence": "the mechanical increment of 512 N at B exists; the policy refuses because the "
                                "subtrahend's factored load is not in the minuend"}))

    def ca_state(rc):
        return {"member": "AB", "kind": "constant_alpha_interval",
                "operands": {"alpha": q(F(1, 2 ** 16)), "interval": "64"}, "fit": {"kind": "none"},
                "reference_configuration": rc}
    rows.append(("X2", "equal resolved state under two reference-configuration identifiers",
                 w_side("S", member_states=[ca_state("RC-1")]), w_side("O", member_states=[ca_state("RC-2")]),
                 "no reference-configuration identifier is compared; the resolved operands are equal", True,
                 "not established here whether one exact-route document can carry two reference configurations "
                 "for one member; the row tests the policy function only",
                 {"kind": "linear_increment", "values": {"u/B/UX": entry(ZERO, "m")}}))

    # X3: W3 with only the rigid line stop's identity changed in the minuend
    delta = F(1, 1024)
    u0, *_ = bar_solve(W_XS, [("AB", "A", "B", ea, ZERO)], {"A": ("rigid", 0), "B": ("rigid", 0)}, {})
    u1, *_ = bar_solve(W_XS, [("AB", "A", "B", ea, ZERO)], {"A": ("rigid", 0), "B": ("rigid", delta)}, {})
    motion = {"support": "line_stop:B", "dof": "UX", "field": "rigid_boundary", "value": q(delta),
              "value_bits": rational_bits(delta), "authored": True}
    other_stop = dict(LINE_STOP, identity="line_stop:B:other")
    rows.append(("X3", "W3 with only the rigid stop's identity changed",
                 w_side("S", supports=W_ANCHOR + [LINE_STOP]),
                 w_side("O", supports=W_ANCHOR + [other_stop], motions=[motion]),
                 "the rigid restraint at B.UX has another identity in the minuend", True,
                 "Compares different model-level support definitions and is gate-only.",
                 {"kind": "state_difference",
                  "values": {"u/B/UX (subtrahend)": entry(u0["B"], "m"), "u/B/UX (minuend)": entry(u1["B"], "m"),
                             "u/B/UX (minuend - subtrahend)": entry(u1["B"] - u0["B"], "m")}}))

    # X4: a free-length state with a constant zero law and unequal temperatures stays an action
    zero_law_state = {"member": "AB", "kind": "free_length_state",
                      "operands": {"installation_temperature": "300", "operating_temperature": "500",
                                   "expansion_law": {"definition": "secant_constant", "datum": "300",
                                                     "constant": "0"}},
                      "fit": {"kind": "none"}, "reference_configuration": "RC-AB"}
    strain = F(0) * (F(500) - F(300)) / (1 + F(0) * (F(300) - F(300)))
    _, nf, *_ = bar_solve(W_XS, [("AB", "A", "B", ea, Exact.of(-strain))], {"A": ("rigid", 0), "B": ("rigid", 0)}, {})
    assert strain == 0 and nf["AB"].is_zero()
    rows.append(("X4", "zero-law free-length state with unequal temperatures",
                 w_side("S", supports=W_ANCHOR + [LINE_STOP], member_states=[zero_law_state]),
                 w_side("O", supports=W_ANCHOR + [LINE_STOP]),
                 "the subtrahend's free-length state has unequal installation and operating temperatures, so it "
                 "is an action even though its law's constant is zero; it is not in the minuend", False,
                 NOT_GATE_ONLY,
                 {"kind": "linear_increment",
                  "values": {"free strain increment": entry(ZERO - strain, "1"),
                             "axial/AB/N on a fixed-fixed bar of the shared W section": entry(nf["AB"], "N")},
                  "divergence": "the free strain of the subtrahend's state is exactly zero, so the increment is "
                                "zero; the policy refuses because the state is not established zero by its kind"}))

    # X5: two seismic recipes that differ only in their mass basis
    def seismic(contents):
        return {"kind": "seismic", "target": "AB", "inputs": {"axis": "global_x", "factor": "1/4"},
                "mass_basis": {"method": "lumped_member_mass", "section_mass_per_length": "24",
                               "contents_mass_per_length": q(contents), "component_masses": []}}
    rows.append(("X5", "two seismic recipes that differ only in mass basis",
                 w_side("S", generated=[seismic(F(8))]), w_side("O", generated=[seismic(F(0))]),
                 "the recipes' mass bases differ, so the subtrahend's seismic recipe is not in the minuend", True,
                 "Proposed policy case. Which mass inputs exact-route seismic generation reads has not been checked "
                 "here.",
                 {"kind": "none"}))
    return rows


MEANINGS = {
    "linear_increment": "the response of the common operator to the difference of the two load vectors",
    "state_difference": "the difference of two independently solved states; no superposition claim",
    "none": "no numerical result: this package states no seismic generation rule, so no exact value is given",
}


def as_records(rows, group):
    out = []
    for rid, title, s, o, reason, gate_only, gate_why, mech in rows:
        mech = dict(mech, meaning=MEANINGS[mech["kind"]])
        out.append({"id": rid, "title": title, "sustained": s, "operating": o,
                    "admission": admission(s, o, reason, gate_only, gate_why),
                    "stiffness_readings": stiffness_readings(s, o), "mechanical": mech})
        COUNTS[group] += 1
    return out


EXPECTED_ADDED = {"W1": "admitted", "W2": "admitted", "W3": "admitted", "W4": "admitted", "W5": "refused_g1",
                  "W6": "refused_restraint", "W7a": "refused_restraint", "W7b": "refused_restraint",
                  "W8": "refused_g1", "W9": "admitted", "F1": "refused_g1", "F2": "refused_g1", "F2b": "refused_g1",
                  "K1": "refused_g2", "W11": "refused_g2",
                  "X1": "refused_g1", "X2": "admitted", "X3": "refused_restraint", "X4": "refused_g1",
                  "X5": "refused_g1"}


# ==========================================================================================================

def build() -> dict[str, bytes]:
    COUNTS.clear()
    hand_redundants("REF")
    hand_redundants("HOT")
    gate_rows = carried_gate_rows()
    pairs = carried_pairs()
    added = as_records(added_rows(), "added_rows")
    witnesses = as_records(builder_witnesses(), "builder_witnesses")
    for r in added + witnesses:
        if r["admission"]["verdict"] != EXPECTED_ADDED[r["id"]]:
            raise AssertionError(f"{r['id']}: {r['admission']['verdict']} against the specification's "
                                 f"{EXPECTED_ADDED[r['id']]}")
    references = {
        "schema": "exact_difference_basis_completed/v1",
        "status": "candidate references, derived and cross-checked by their author, awaiting independent refutation",
        "supersedes": {
            "package": "validation/references/exact_difference_basis",
            "section": "G-1: what 'contained by content and with multiplicity' means",
            "definitions": [
                "A pressure region's content is its members and its pressure's bits.",
                "A member state is compared as authored: member, kind, expansion law reference, and the "
                "temperature's bits and unit. It is not compared by the resolved strain."]},
        "frozen_outcome_mapping": FROZEN_TO_COMPLETED,
        "pi_convention": "a value is a finite sum of rational multiples of integer powers of pi, written as in "
                         "3/4*pi + 1/2 + 5/7*pi^-1; value is the nearest binary64",
        "inputs": {
            "frozen": {"poisson_ratio": q(FROZEN_NU),
                       "points": {q(t): {"E": q(e), "G": q(g), "alpha": q(a)} for t, (e, g, a) in FROZEN_POINTS.items()},
                       "section": section_record(), "thermal_dT_K": q(FROZEN_DT),
                       "pressures_Pa": [q(FROZEN_P), q(FROZEN_P_CHANGED)],
                       "bases": {k: {"selection_kind": v["selection_kind"], "temperature_K": q(v["temperature"])}
                                 for k, v in BASES.items() if "points" not in v},
                       "models": {mid: {"nodes": {n: [q(c) for c in xyz] for n, xyz in m["nodes"].items()},
                                        "anchors": m["anchors"],
                                        "members": [{"id": x[0], "from": x[1], "to": x[2], "y_reference": list(x[3])}
                                                    for x in m["members"]],
                                        "regions": m["regions"],
                                        "weights_z_N": {n: q(w) for n, w in m["weights"].items()}}
                                  for mid, m in FROZEN_MODELS.items()}},
            "w11_material": {"name": W11_MATERIAL, "poisson_ratio": q(W11_NU),
                             "points": {q(T): {"E": q(e), "G": q(g), "alpha": q(al)}
                                        for T, (e, g, al) in W11_POINTS.items()},
                             "selections": {"S": "exact basis reference to the 250 K point (W11_LO)",
                                            "O": "exact basis reference to the 650 K point (W11_HI)"},
                             "model": "M1 geometry and the frozen section, no pressure; weight at B and dT on "
                                      "every member in both operands"},
            "shared_w": {"length_m": q(W_L), "outside_diameter_m": q(W_D), "wall_m": q(W_WALL),
                         "inner_radius_m": q(W_RI), "wall_area_m2": Exact({1: W_AREA}).text(), "E_Pa": q(W_E),
                         "nu": q(W_NU), "EA_N": W_EA.text(), "anchor": "A, all six DOFs rigid"},
        },
        "carried": {"gate_rows": gate_rows, "pairs": pairs},
        "added": added,
        "builder_witnesses": witnesses,
        "cross_checks": dict(sorted(COUNTS.items())),
    }
    text = (json.dumps(references, indent=1, sort_keys=True) + "\n").encode()
    sums = "".join(f"{hashlib.sha256(data).hexdigest()}  {name}\n" for name, data in (
        ("README.md", (PACKAGE / "README.md").read_bytes()),
        ("generators/exact_difference_basis_completed_reference.py", THIS.read_bytes()),
        ("references.json", text)))
    return {"references.json": text, "SHA256SUMS": sums.encode()}


def main() -> int:
    outputs = build()
    if "--write" in sys.argv[1:]:
        for name, data in outputs.items():
            (PACKAGE / name).write_bytes(data)
        print("written")
        return 0
    stale = [n for n, d in outputs.items() if not (PACKAGE / n).exists() or (PACKAGE / n).read_bytes() != d]
    print("stale: " + ", ".join(stale) if stale else "references are current")
    return 1 if stale else 0


if __name__ == "__main__":
    raise SystemExit(main())
