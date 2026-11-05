"""Independent exact references for skewed restraints (R-6, R-7).

Basis: DEL-04-03 Work `skew-restraints.md` (the build design) and DEL-05-02 Work
`operating-case.md` sections 2.2 to 2.6 (the adopted kernel design). Standard
library only; this file reads, imports and runs no product code. Every value is
an exact rational (fractions.Fraction). A published expectation is written with
its exact value and the binary64 nearest to it (float.hex()); an irrational
scalar is written as its exact square, its sign and its correctly rounded
binary64 value.

Two derivations are kept apart and must agree on every admitted case:
- route K: the constrained equilibrium (KKT) system [K, -C^T; C, 0][u; lam] = [f; g],
  solved exactly; the support-on-pipe reaction of constraint row c is lam*c;
- route N: an exact nullspace basis of C from reduced row echelon form (not the
  product's canonical basis), u = u0 + N y with N^T K N y = N^T (f - K u0), and
  lam recovered from C^T lam = K u - f.
The canonical division-free basis T of the design (section 4) is computed from its
stated rule and checked to span the same nullspace; its entries are published
because the product must form them bit for bit.

Run ``python3 -I generators/skew_reference.py`` from the parent directory to
compare the outputs with the committed files; ``--write`` rewrites them.
"""
from __future__ import annotations

import hashlib
import json
import math
import sys
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent.parent
MIN_NORMAL = F(2) ** -1022

# ---------------------------------------------------------------- formatting


def b64(x: F) -> str:
    """The binary64 nearest to x (Fraction.__float__ rounds correctly)."""
    return float(x).hex()


def num(x: F) -> dict:
    return {"exact": f"{x.numerator}/{x.denominator}", "binary64": b64(x)}


def is_square(q: F) -> F | None:
    if q < 0:
        return None
    n, d = math.isqrt(q.numerator), math.isqrt(q.denominator)
    return F(n, d) if n * n == q.numerator and d * d == q.denominator else None


def sqrt_nearest(q: F) -> float:
    """The binary64 nearest to sqrt(q), q > 0, decided by exact comparisons. q is first
    scaled by an even power of two, 4^e, so the estimate never leaves the float range;
    the result is then scaled by 2^e exactly (it stays normal for every value here)."""
    e = (q.numerator.bit_length() - q.denominator.bit_length()) // 2
    root = sqrt_nearest_scaled(q / F(4) ** e)
    out = math.ldexp(root, e)
    assert F(out) == F(root) * F(2) ** e and out != 0.0 and math.isfinite(out)
    return out


def sqrt_nearest_scaled(q: F) -> float:
    x = math.sqrt(float(q))
    while True:
        lo = (F(math.nextafter(x, 0.0)) + F(x)) / 2
        hi = (F(x) + F(math.nextafter(x, math.inf))) / 2
        if lo * lo > q:
            x = math.nextafter(x, 0.0)
        elif hi * hi < q:
            x = math.nextafter(x, math.inf)
        else:
            assert lo * lo != q and hi * hi != q, "a tie needs the even rule"
            return x


def scalar(s_sign: int, s_square: F) -> dict:
    """A signed scalar given by its sign and exact square."""
    root = is_square(s_square)
    if root is not None:
        return num(s_sign * root)
    return {"square": f"{s_square.numerator}/{s_square.denominator}", "sign": s_sign,
            "binary64": (s_sign * sqrt_nearest(s_square)).hex()}


def vec(v) -> list:
    return [num(F(x)) for x in v]


def squarefree(n: int) -> tuple[int, int]:
    """n = r^2 * q with q squarefree (n > 0)."""
    r, q, f = 1, 1, 2
    while f * f <= n:
        while n % (f * f) == 0:
            n //= f * f
            r *= f
        f += 1
    return r, n


def sign_qs(c: F, b: F, q: int) -> int:
    """The sign of c + b*sqrt(q), exactly (sqrt(q) irrational or q = 1)."""
    sc, sb = (c > 0) - (c < 0), (b > 0) - (b < 0)
    if sb == 0 or sc == sb:
        return sc if sc else sb
    if sc == 0:
        return sb
    return sc if c * c > b * b * q else sb


def nearest_qs(a: F, b: F, q: int) -> float:
    """The binary64 nearest to a + b*sqrt(q), decided by exact comparisons."""
    x = float(a) + float(b) * math.sqrt(q)
    while True:
        lo = (F(math.nextafter(x, -math.inf)) + F(x)) / 2
        hi = (F(x) + F(math.nextafter(x, math.inf))) / 2
        if sign_qs(a - lo, b, q) < 0:
            x = math.nextafter(x, -math.inf)
        elif sign_qs(a - hi, b, q) > 0:
            x = math.nextafter(x, math.inf)
        else:
            assert sign_qs(a - lo, b, q) != 0 and sign_qs(a - hi, b, q) != 0, "a tie needs the even rule"
            return x


def qnum(a: F, b: F, q: int) -> dict:
    """a + b*sqrt(q), q squarefree > 1, exactly, with its correctly rounded binary64."""
    if b == 0:
        return num(a)
    return {"rational": f"{a.numerator}/{a.denominator}", "sqrt_coefficient": f"{b.numerator}/{b.denominator}",
            "radicand": q, "binary64": nearest_qs(a, b, q).hex()}


# ---------------------------------------------------------------- exact algebra


def rref(rows: list[list[F]]) -> tuple[list[list[F]], list[int]]:
    m = [list(r) for r in rows]
    pivots, r = [], 0
    cols = len(m[0]) if m else 0
    for c in range(cols):
        p = next((i for i in range(r, len(m)) if m[i][c] != 0), None)
        if p is None:
            continue
        m[r], m[p] = m[p], m[r]
        piv = m[r][c]
        m[r] = [x / piv for x in m[r]]
        for i in range(len(m)):
            if i != r and m[i][c] != 0:
                f = m[i][c]
                m[i] = [a - f * b for a, b in zip(m[i], m[r])]
        pivots.append(c)
        r += 1
    return m[:r], pivots


def rank(rows: list[list[F]]) -> int:
    return len(rref(rows)[1]) if rows else 0


def solve(a: list[list[F]], b: list[F]) -> list[F]:
    n = len(a)
    reduced, pivots = rref([row + [rhs] for row, rhs in zip(a, b)])
    assert pivots == list(range(n)), "singular system"
    return [reduced[i][n] for i in range(n)]


def nullspace(rows: list[list[F]], n: int) -> list[list[F]]:
    if not rows:
        return [[F(int(i == j)) for i in range(n)] for j in range(n)]
    reduced, pivots = rref(rows)
    free = [c for c in range(n) if c not in pivots]
    basis = []
    for f in free:
        v = [F(0)] * n
        v[f] = F(1)
        for r, p in enumerate(pivots):
            v[p] = -reduced[r][f]
        basis.append(v)
    return basis


def matvec(a, x):
    return [sum((p * q for p, q in zip(row, x)), F(0)) for row in a]


def dot(x, y):
    return sum((p * q for p, q in zip(x, y)), F(0))


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


# ---------------------------------------------------------------- direction policy (design section 4)


def classify(authored) -> dict:
    """The design's admission rule for an authored direction, from its statement.

    Not finite -> SUPPORT_DIRECTION_INVALID; every component +-0 ->
    SUPPORT_DIRECTION_ZERO; exactly one nonzero component -> the axis path; else the
    canonical scaling 2^s with max|d_i 2^s| in [1, 2); a nonzero scaled component
    below 2^-1022 (subnormal) -> SUPPORT_DIRECTION_RANGE_UNSUPPORTED.
    """
    if any(not isinstance(c, float) or not math.isfinite(c) for c in authored):
        return {"class": "refused", "code": "SUPPORT_DIRECTION_INVALID"}
    if all(c == 0.0 for c in authored):
        return {"class": "refused", "code": "SUPPORT_DIRECTION_ZERO"}
    nonzero = [i for i, c in enumerate(authored) if c != 0.0]
    if len(nonzero) == 1:
        axis = nonzero[0]
        return {"class": "axis", "axis": "XYZ"[axis], "sign": 1 if authored[axis] > 0 else -1,
                "token": "U" + "XYZ"[axis]}
    largest = max(abs(c) for c in authored)
    _, e = math.frexp(largest)  # largest in [2^(e-1), 2^e)
    s = 1 - e
    scaled = [F(c) * F(2) ** s for c in authored]
    assert F(1) <= max(abs(x) for x in scaled) < 2
    small = [i for i, x in enumerate(scaled) if x != 0 and abs(x) < MIN_NORMAL]
    if small:
        return {"class": "refused", "code": "SUPPORT_DIRECTION_RANGE_UNSUPPORTED",
                "scale_exponent": s, "subnormal_components": ["xyz"[i] for i in small]}
    for x in scaled:
        assert F(float(x)) == x, "a normal power-of-two scaling of a binary64 is exact"
    pivot = max(range(3), key=lambda i: (abs(scaled[i]), -i))
    return {"class": "skew", "scale_exponent": s, "scaled": [float(x) for x in scaled], "pivot": pivot}


def canonical_columns(scaled: list[float], pivot: int, free_axes: list[int]) -> list[list[float]]:
    """T on the free axes F: one column d_m e_j - d_j e_m per j in F other than m."""
    cols = []
    for j in free_axes:
        if j == pivot:
            continue
        col = [0.0, 0.0, 0.0]
        col[j] = scaled[pivot]
        col[pivot] = -scaled[j] if scaled[j] != 0.0 else 0.0  # a structural zero is +0.0
        cols.append(col)
    return cols


def node_pivot(scaled: list[float], free_axes: list[int]) -> int | None:
    candidates = [i for i in free_axes if scaled[i] != 0.0]
    if not candidates:
        return None
    return max(candidates, key=lambda i: (abs(scaled[i]), -i))


def direction_cases() -> list[dict]:
    two = 2.0
    rows = [
        ("d345", [3.0, 4.0, 0.0]),
        ("d345_x2p600", [3.0 * two ** 600, 4.0 * two ** 600, 0.0]),
        ("d345_x2m600", [3.0 * two ** -600, 4.0 * two ** -600, 0.0]),
        ("d345_x2m1020", [3.0 * two ** -1020, 4.0 * two ** -1020, 0.0]),
        ("d345_x3", [9.0, 12.0, 0.0]),
        ("d345_x0p75", [2.25, 3.0, 0.0]),
        ("d345_neg", [-3.0, -4.0, 0.0]),
        ("d345_negx2p600", [-3.0 * two ** 600, -4.0 * two ** 600, 0.0]),
        ("d11", [1.0, 1.0, 0.0]),
        ("d13", [1.0, 3.0, 0.0]),
        ("d12", [1.0, 2.0, 0.0]),
        ("d12_x0p1", [0.1, 0.2, 0.0]),
        ("d345_3d", [3.0, 4.0, 5.0]),
        ("d034", [0.0, 3.0, 4.0]),
        ("d430", [-4.0, 3.0, 0.0]),
        ("d_subnormal_pair", [two ** -1074, two ** -1074, 0.0]),
        ("d_min_normal_ratio", [1.0, two ** -1022, 0.0]),
        ("axis_y", [0.0, 2.0, 0.0]),
        ("axis_neg_x", [-1.0, 0.0, 0.0]),
        ("axis_neg_zero_mix", [-0.0, 0.0, 5.0]),
        ("zero", [0.0, 0.0, 0.0]),
        ("negative_zero", [-0.0, 0.0, -0.0]),
        ("range_subnormal_component", [1.0, two ** -1023, 0.0]),
        ("range_scaled_underflow", [two ** 1000, two ** -30, 0.0]),
        ("range_max_finite", [sys.float_info.max, 1.0, 0.0]),
        ("range_tiny_third", [3.0, 4.0, two ** -1074]),
    ]
    out = []
    for name, d in rows:
        verdict = classify(d)
        entry = {"name": name, "authored": [x.hex() for x in d], "expect": dict(verdict)}
        if verdict["class"] == "skew":
            scaled, pivot = verdict["scaled"], verdict["pivot"]
            entry["expect"]["scaled"] = [x.hex() for x in scaled]
            entry["expect"]["pivot"] = "xyz"[pivot]
            entry["expect"]["T_columns_all_axes_free"] = [[x.hex() for x in col] for col in canonical_columns(scaled, pivot, [0, 1, 2])]
            # Q: column j is T's column for j != m, and e_m for the pivot coordinate.
            q = [[0.0] * 3 for _ in range(3)]
            for j in range(3):
                if j == pivot:
                    q[pivot][j] = 1.0
                else:
                    q[j][j] = scaled[pivot]
                    q[pivot][j] = -scaled[j] if scaled[j] != 0.0 else 0.0
            entry["expect"]["Q_rows"] = [[x.hex() for x in row] for row in q]
            entry["expect"]["norm_squared_of_scaled"] = num(sum((F(x) ** 2 for x in scaled), F(0)))
            for col in canonical_columns(scaled, pivot, [0, 1, 2]):
                assert dot([F(x) for x in d], [F(x) for x in col]) == 0, "d^T T = 0 exactly"
        out.append(entry)
    # Invalid JSON texts: not representable as binary64 vectors.
    for name, text in [("non_finite_overflow_text", '{"x": 1e999, "y": 0, "z": 0}'),
                       ("non_number", '{"x": "NaN", "y": 0, "z": 0}'),
                       ("missing_component", '{"x": 1, "y": 2}'),
                       ("null", "null")]:
        out.append({"name": name, "authored_json": text, "expect": {"class": "refused", "code": "SUPPORT_DIRECTION_INVALID"}})
    return out


def invariance_groups(cases: list[dict]) -> dict:
    by = {c["name"]: c for c in cases}
    bitwise = [["d345", "d345_x2p600", "d345_x2m600", "d345_x2m1020"], ["d345_neg", "d345_negx2p600"],
               ["d11", "d_subnormal_pair"]]
    for group in bitwise:
        first = by[group[0]]["expect"]
        for name in group[1:]:
            other = by[name]["expect"]
            assert other["scaled"] == first["scaled"] and other["pivot"] == first["pivot"]
    positive = [["d345", "d345_x3", "d345_x0p75"], ["d12", "d12_x0p1"]]
    for group in positive:
        base = [F(x) for x in [float.fromhex(h) for h in by[group[0]]["authored"]]]
        for name in group[1:]:
            v = [F(float.fromhex(h)) for h in by[name]["authored"]]
            ratio = next(a / b for a, b in zip(v, base) if b != 0)
            assert ratio > 0 and all(a == ratio * b for a, b in zip(v, base)), name
    return {
        "bitwise_identical_outputs": bitwise,
        "exact_system_identical_positive_factor": positive,
        "negation": [["d345", "d345_neg"]],
        "statement": ("bitwise: identical canonical scaled vector and pivot, so every output is bit for bit "
                      "identical; positive factor: same constraint, spring block and exact solution, while "
                      "binary64 outputs may differ by rounding; negation: the same exact u and global reactions "
                      "and the exactly negated along-direction value are guaranteed (equal within the stated "
                      "bounds); bit for bit up to the sign of a zero only under the solve's sign-symmetry premise "
                      "P-SYM (design section 4.3), which a maintained test checks"),
    }


# ---------------------------------------------------------------- the constrained solve (routes K and N)


def unit(n, i):
    return [F(int(j == i)) for j in range(n)]


def place(n, offset, v):
    out = [F(0)] * n
    for i, x in enumerate(v):
        out[offset + i] = F(x)
    return out


class Model:
    """n DOFs; stiffness K; force f; records of constraint rows and springs."""

    def __init__(self, n, k, f):
        self.n, self.k, self.f = n, [list(map(F, r)) for r in k], list(map(F, f))
        self.records = []  # (name, kind, rows [(c, g)], extra)
        self.springs = []  # (name, offset, direction c (3), stiffness, ground g along nothing)

    def rigid(self, name, rows, kind="axis", direction=None):
        self.records.append({"name": name, "kind": kind, "rows": rows, "direction": direction})

    def spring(self, name, offset, c, k):
        c = [F(x) for x in c]
        self.springs.append({"name": name, "offset": offset, "c": c, "k": F(k)})
        cc = dot(c, c)
        for i in range(3):
            for j in range(3):
                self.k[offset + i][offset + j] += F(k) * c[i] * c[j] / cc

    def constraints(self):
        return [(c, g) for r in self.records for (c, g) in r["rows"]]

    def solve(self):
        cons = self.constraints()
        c_rows = [c for c, _ in cons]
        r = len(cons)
        if r and rank(c_rows) < r:
            return None
        n = self.n
        # Route K.
        a = [self.k[i] + [-c_rows[j][i] for j in range(r)] for i in range(n)]
        a += [c_rows[j] + [F(0)] * r for j in range(r)]
        sol = solve(a, self.f + [g for _, g in cons])
        u, lam = sol[:n], sol[n:]
        # Route N.
        if r:
            reduced, pivots = rref([c + [g] for c, g in cons])
            u0 = [F(0)] * n
            for row, p in zip(reduced, pivots):
                u0[p] = row[n]
        else:
            u0 = [F(0)] * n
        basis = nullspace(c_rows, n)
        rhs = [f_i - ku for f_i, ku in zip(self.f, matvec(self.k, u0))]
        if basis:
            kt = [[dot(bi, matvec(self.k, bj)) for bj in basis] for bi in basis]
            y = solve(kt, [dot(bi, rhs) for bi in basis])
            u_n = [u0[i] + sum((y[j] * basis[j][i] for j in range(len(basis))), F(0)) for i in range(n)]
        else:
            u_n = u0
        assert u_n == u, "routes K and N agree on u"
        resid = [ku - f_i for ku, f_i in zip(matvec(self.k, u), self.f)]
        if r:
            ct = [[c_rows[j][i] for j in range(r)] for i in range(n)]
            red, piv = rref([row + [x] for row, x in zip(ct, resid)])
            assert piv == list(range(r)), "C has full row rank"
            lam_n = [red[i][r] for i in range(r)]
            assert lam_n == lam, "routes K and N agree on the multipliers"
        else:
            assert all(x == 0 for x in resid)
        return u, lam


def record_results(model: Model, u, lam) -> dict:
    out, j = {}, 0
    for rec in model.records:
        k = len(rec["rows"])
        mult = lam[j:j + k]
        j += k
        if rec["kind"] == "axis":
            out[rec["name"]] = {"kind": "rigid_axis_rows", "components": [num(x) for x in mult]}
        else:
            # The row is the authored d itself, so the multiplier is per unit authored d.
            (c, g), = rec["rows"]
            d = [F(x) for x in rec["direction"]]
            lam_d = mult[0]
            global_r = [lam_d * x for x in d]
            sign = (lam_d > 0) - (lam_d < 0)
            assert dot(c, u) == g, "the authored constraint holds exactly"
            out[rec["name"]] = {"kind": "skew_stop", "global_force": [num(x) for x in global_r],
                                "along_direction": scalar(sign, lam_d * lam_d * dot(d, d)),
                                "d_dot_u_exact": f"{g.numerator}/{g.denominator}"}
    for sp in model.springs:
        o, c, kk = sp["offset"], sp["c"], sp["k"]
        cu = dot(c, u[o:o + 3])
        cc = dot(c, c)
        force = [-kk * x * cu / cc for x in c]
        s = -kk * cu
        sign = (s > 0) - (s < 0)
        out[sp["name"]] = {"kind": "skew_spring", "global_force": [num(x) for x in force],
                           "along_direction": scalar(sign, s * s / cc)}
    return out


def single_stop_info(model: Model, u, lam, offset: int) -> dict | None:
    """S-K1's range-safe attribution data (design section 7.2) and the lifted-remainder
    exponent rule (section 10.4) for a node with axis rows and exactly one stop."""
    stops = [r for r in model.records if r["kind"] == "skew"]
    if len(stops) != 1:
        return None
    stop = stops[0]
    authored = [float(F(x)) for x in stop["direction"]]
    assert all(F(a) == F(x) for a, x in zip(authored, stop["direction"]))
    verdict = classify(authored)
    if verdict["class"] != "skew":
        return None
    scaled = [F(x) for x in verdict["scaled"]]
    held = {}
    j = 0
    for rec in model.records:
        k = len(rec["rows"])
        if rec["kind"] == "axis":
            for (c, g), mult in zip(rec["rows"], lam[j:j + k]):
                i = next(i for i, x in enumerate(c) if x != 0)
                if offset <= i < offset + 3:
                    held[i - offset] = g
        j += k
    free = [i for i in range(3) if i not in held]
    if all(scaled[i] == 0 for i in free):
        return None
    top = max(abs(scaled[i]) for i in free if scaled[i] != 0)
    s_hat = 0
    while top * F(2) ** s_hat < 1:
        s_hat += 1
    w = [x * F(2) ** s_hat for x in scaled]
    n_w = sum((w[i] ** 2 for i in free), F(0))
    n_f = sum((scaled[i] ** 2 for i in free), F(0))
    lam_d = lam[sum(len(r["rows"]) for r in model.records[:model.records.index(stop)])]
    d = [F(x) for x in stop["direction"]]
    i0 = next(i for i in range(3) if d[i] != 0)
    mu = lam_d * d[i0] / w[i0]
    pivot = max((i for i in free if scaled[i] != 0), key=lambda i: (abs(scaled[i]), -i))
    coords = {i: ([] if i in held else [i] if i != pivot else [j for j in free if j != pivot and scaled[j] != 0])
              for i in range(3)}
    h = stop["rows"][0][1]
    tau = (h - sum((d[a] * g for a, g in held.items()), F(0))) / d[pivot]
    u0 = [held.get(i, F(0)) for i in range(3)]
    u0[pivot] += tau
    assert dot(d, u0) == h and all(u0[a] == g for a, g in held.items()), "C u0 = (g_A, h) exactly"
    return {
        "free_axes": ["xyz"[i] for i in free], "pivot_on_F": "xyz"[pivot],
        "restricted_scale_exponent": s_hat, "w": [float(x).hex() for x in w],
        "n_w": num(n_w), "n_F_exact": num(n_f), "n_F_rounds_to_zero": float(n_f) == 0.0,
        "mu_per_unit_w": num(mu),
        "lifted_remainder_exponent": {"xyz"[i]: ("i32::MIN" if not coords[i] else ["xyz"[j] for j in coords[i]]) for i in range(3)},
        "particular_solution_u0": vec(u0),
    }


# ---------------------------------------------------------------- kernel set (units: forces in P, u in P/k)


def tip(kdiag):
    return [[F(kdiag[i]) if i == j else F(0) for j in range(3)] for i in range(3)]


def kernel_cases() -> list[dict]:
    p = F(1)
    load_y = [F(0), -p, F(0)]
    out = []

    def case(name, kdiag, load, build, slice_, note, refusal_s_k1=None):
        m = Model(3, tip(kdiag), load)
        build(m)
        result = m.solve()
        entry = {"name": name, "tip_stiffness_diagonal": [num(F(x)) for x in kdiag], "load": vec(load),
                 "slice": slice_, "note": note}
        if refusal_s_k1:
            entry["s_k1_refusal"] = refusal_s_k1
        if result is None:
            entry["expect"] = {"code": "SUPPORT_CONSTRAINT_REDUNDANT",
                               "rank": rank([c for c, _ in m.constraints()]), "rows": len(m.constraints())}
        else:
            u, lam = result
            entry["expect"] = {"u": vec(u), "records": record_results(m, u, lam),
                               "rank": rank([c for c, _ in m.constraints()]) if m.constraints() else 0}
            info = single_stop_info(m, u, lam, 0)
            if info:
                entry["expect"]["attribution"] = info
        out.append(entry)
        return entry

    def stop(name, d, g=F(0)):
        return lambda m: m.rigid(name, [([F(x) for x in d], g)], kind="skew", direction=d)

    def axis(name, i, g=F(0)):
        return lambda m: m.rigid(name, [(unit(3, i), g)])

    def sqrt_case(name, axes, d, h_rat, h_sqrt, note):
        """Meaning B with an irrational norm: d^T u = h_rat + h_sqrt*sqrt(q), q = squarefree(d^T d)."""
        d = [F(x) for x in d]
        r_d, q = squarefree(int(dot(d, d)))
        m0, m1 = Model(3, tip([1, 1, 1]), load_y), Model(3, tip([1, 1, 1]), [F(0)] * 3)
        for nm, i, g in axes:
            m0.rigid(nm, [(unit(3, i), g)])
            m1.rigid(nm, [(unit(3, i), F(0))])
        m0.rigid("stop", [(d, h_rat)], kind="skew", direction=d)
        m1.rigid("stop", [(d, h_sqrt)], kind="skew", direction=d)
        (u0, l0), (u1, l1) = m0.solve(), m1.solve()
        records, j = {}, 0
        for rec in m0.records:
            k = len(rec["rows"])
            a, b = l0[j:j + k], l1[j:j + k]
            j += k
            if rec["kind"] == "axis":
                records[rec["name"]] = {"kind": "rigid_axis_rows", "components": [qnum(x, y, q) for x, y in zip(a, b)]}
            else:
                records[rec["name"]] = {"kind": "skew_stop", "global_force": [qnum(a[0] * c, b[0] * c, q) for c in d],
                                        "along_direction": qnum(b[0] * r_d * q, a[0] * r_d, q)}
        # The design's particular solution (section 8.3) in Q(sqrt q).
        held = {i: g for _, i, g in axes}
        free = [i for i in range(3) if i not in held]
        scaled = [F(x) for x in classify([float(x) for x in d])["scaled"]]
        pivot = max((i for i in free if scaled[i] != 0), key=lambda i: (abs(scaled[i]), -i))
        tau_r = (h_rat - sum((d[a] * g for a, g in held.items()), F(0))) / d[pivot]
        tau_s = h_sqrt / d[pivot]
        u0r = [held.get(i, F(0)) for i in range(3)]
        u0s = [F(0)] * 3
        u0r[pivot] += tau_r
        u0s[pivot] += tau_s
        assert dot(d, u0r) == h_rat and dot(d, u0s) == h_sqrt
        assert dot(d, u0) == h_rat and dot(d, u1) == h_sqrt, "the constraint holds exactly in Q(sqrt q)"
        out.append({"name": name, "tip_stiffness_diagonal": [num(F(1))] * 3, "load": vec(load_y), "slice": "S-K3",
                    "s_k1_refusal": "IMPOSED_DISPLACEMENT_SKEW_SUPPORT_UNSUPPORTED", "note": note,
                    "meaning": "B", "norm": {"r": r_d, "radicand": q},
                    "expect": {"u": [qnum(a, b, q) for a, b in zip(u0, u1)], "records": records,
                               "particular_solution_u0": [qnum(a, b, q) for a, b in zip(u0r, u0s)],
                               "pivot_on_F": "xyz"[pivot]}})

    def both(*fs):
        def run(m):
            for f in fs:
                f(m)
        return run

    case("S1_spring_isotropic", [1, 1, 1], load_y, lambda m: m.spring("s", 0, [3, 4, 0], 1), "S-K1",
         "design 2.6 S-1: u = (6/25, -17/25, 0) P/k; R_s = (6/25, 8/25) P; +2P/5 along +d")
    case("S2_spring_pair", [1, 1, 1], load_y, both(lambda m: m.spring("s1", 0, [3, 4, 0], 1), lambda m: m.spring("s2", 0, [-4, 3, 0], 1)),
         "S-K1", "design 2.6 S-2: the pair's exact block is k diag(1, 1, 0), so u equals the axis-spring twin's exactly")
    case("S2_axis_twin", [1, 1, 1], load_y, lambda m: (m.spring("ux", 0, [1, 0, 0], 1), m.spring("uy", 0, [0, 1, 0], 1)),
         "today", "the axis twin of S2_spring_pair (axis springs on UX and UY)")
    case("R1_stop_isotropic", [1, 1, 1], load_y, stop("stop", [3, 4, 0]), "S-K1",
         "design 2.6 R-1: +4P/5 along +d, R = (12/25, 16/25) P, u = (12/25, -9/25) P/k")
    case("R1_stop_scaled_2p600", [1, 1, 1], load_y, stop("stop", [F(3) * F(2) ** 600, F(4) * F(2) ** 600, 0]), "S-K1",
         "R-1 with d scaled by 2^600: identical canonical vector, bit-for-bit identical outputs")
    case("R2_stop_anisotropic", [2, 1, 1], load_y, stop("stop", [3, 4, 0]), "S-K1",
         "design 2.6 R-2 (k_a = 2k): +40P/41 along +d, u = (12/41, -9/41) P/k")
    case("R3_stop_negative", [1, 1, 1], load_y, stop("stop", [-3, -4, 0]), "S-K1",
         "design 2.6 R-3: -4P/5 along the authored +d, the same u as R-1")
    case("R3_stop_plane_13", [1, 1, 1], load_y, stop("stop", [1, 3, 0]), "S-K1",
         "design 2.6 R-3: 3P/sqrt(10) along +d; u_x + 3 u_y = 0 exactly in the unrounded lift")
    case("R4_guide_plus_stop", [1, 1, 1], load_y, both(axis("guide_uz", 2), stop("stop", [3, 4, 5])), "S-K1",
         "design 2.6 R-4: the stop reduces to 3u_x + 4u_y = 0 on F = {x, y}; lam = 4P/25 per unit d, guide -4P/5, stop 4*sqrt(2)P/5 along +d")
    case("R4_full_fix_one_free_axis", [1, 1, 1], load_y, both(axis("ux", 0), axis("uz", 2), stop("stop", [3, 4, 0])), "S-K1",
         "|F| = 1 and d_F != 0: the node's translations are fixed; the stop takes P/4 per unit d (+5P/4 along +d) and UX -3P/4")
    case("R4_redundant_axes", [1, 1, 1], load_y, both(axis("ux", 0), axis("uy", 1), stop("stop", [3, 4, 0])), "S-K1",
         "d_F = 0 on F = {z}: the stop row is redundant")
    case("R4_near_dependent_axis", [1, 1, 1], load_y, both(axis("ux", 0), stop("stop", [1, F(2) ** -500, 0])), "S-K1",
         "d_F = (2^-500, 0) != 0: exact rank 2, u_y = 0 exactly; the stop's multiplier is 2^500 times the y residual")
    case("R4_tiny_restricted_direction", [1, 1, 1], load_y, both(axis("ux", 0), axis("uz", 2), stop("stop", [1, F(2) ** -600, 0])), "S-K1",
         "review P1-1: rank exactly 3, r_y = 1 P; n_F = 2^-1200 rounds to 0 in binary64, so attribution rescales "
         "d_F exactly (s_hat = 600, n_w = 1): stop (2^600, 1, 0) P, UX -2^600 P, along +2^600*sqrt(1 + 2^-1200) P")
    case("RS_stop_and_spring", [1, 1, 1], load_y, both(stop("stop", [3, 4, 0]), lambda m: m.spring("spring", 0, [0, 3, 4], 1)), "S-K1",
         "a skew stop and a skew spring at one node")
    case("K2_two_stops_3d", [1, 1, 1], load_y, both(stop("stop1", [3, 4, 0]), stop("stop2", [0, 3, 4])), "S-K2",
         "design 2.6 R-4: t = (16, -12, 9), u = (192, -144, 108) P/(481k), lam = (64/481, 27/481) P per unit d",
         "SUPPORT_SKEW_STOPS_PER_NODE_UNSUPPORTED")
    case("K2_orthogonal_pair", [1, 1, 1], [F(0), -p, -p], both(stop("stop1", [3, 4, 0]), stop("stop2", [-4, 3, 0])), "S-K2",
         "two orthogonal stops fix the xy translations: stop1 +4P/5, stop2 +3P/5 along their +d (the axis pair would give UX 0, UY P)",
         "SUPPORT_SKEW_STOPS_PER_NODE_UNSUPPORTED")
    case("K2_oblique_pair", [1, 1, 1], load_y, both(stop("stop1", [3, 4, 0]), stop("stop2", [4, 3, 0])), "S-K2",
         "an oblique independent pair: lam = (4/7, -3/7) P per unit d, +20P/7 and -15P/7 along +d",
         "SUPPORT_SKEW_STOPS_PER_NODE_UNSUPPORTED")
    case("K2_near_parallel", [1, 1, 1], load_y, both(stop("stop1", [3, 4, 0]), stop("stop2", [3, 4 + F(2) ** -48, 0])), "S-K2",
         "minor 3*2^-48 != 0: exact rank 2, an exact reduction; lam = (-2^48, 2^48) P per unit d (no tolerance)",
         "SUPPORT_SKEW_STOPS_PER_NODE_UNSUPPORTED")
    case("K2_parallel", [1, 1, 1], load_y, both(stop("stop1", [3, 4, 0]), stop("stop2", [6, 8, 0])), "S-K2",
         "exactly parallel: rank 1 < 2, redundant", "SUPPORT_SKEW_STOPS_PER_NODE_UNSUPPORTED")
    case("K2_antiparallel_scaled", [1, 1, 1], load_y, both(stop("stop1", [3, 4, 0]), stop("stop2", [F(-3, 1024), F(-4, 1024), 0])), "S-K2",
         "exactly antiparallel and scaled: rank 1 < 2, redundant", "SUPPORT_SKEW_STOPS_PER_NODE_UNSUPPORTED")
    case("K2_three_stops_fixed", [1, 1, 1], load_y, both(stop("stop1", [3, 4, 0]), stop("stop2", [0, 3, 4]), stop("stop3", [1, 1, 1])), "S-K2",
         "rank 3: the node's translations are fixed; the 3x3 attribution is unique", "SUPPORT_SKEW_STOPS_PER_NODE_UNSUPPORTED")
    case("K2_axis_plus_two_stops_redundant", [1, 1, 1], load_y, both(axis("uz", 2), stop("stop1", [3, 4, 0]), stop("stop2", [3, 4, 7])), "S-K2",
         "UZ plus (3,4,0) and (3,4,7): restricted to F = {x, y} the stops are parallel; rank 2 < 3, redundant",
         "SUPPORT_SKEW_STOPS_PER_NODE_UNSUPPORTED")
    # S-K3: imposed motion at a skew-stop node (refused in S-K1 and S-K2).
    h = F(1, 1024)
    case("K3_moving_guide_plus_stop", [1, 1, 1], load_y, both(axis("guide_uz", 2, h), stop("stop", [3, 4, 5])), "S-K3",
         "design 2.3: u_z = h moves the reduced constraint to 3u_x + 4u_y = -5h (affine reduction)",
         "SUPPORT_SKEW_NODE_PRESCRIBED_MOTION_UNSUPPORTED")
    g = F(1, 1024)
    case("K3_stop_ground_meaning_A", [1, 1, 1], load_y, stop("stop", [3, 4, 0], g * 4), "S-K3",
         "meaning A (reserved choice): the stop's ground moves by Delta = g e_y (a global-axis primitive), so d^T u = d^T Delta = 4g",
         "IMPOSED_DISPLACEMENT_SKEW_SUPPORT_UNSUPPORTED")
    case("K3_stop_ground_meaning_B", [1, 1, 1], load_y, stop("stop", [3, 4, 0], g * 5), "S-K3",
         "meaning B (reserved choice), the rational control: the stop's ground moves by g along +d/|d|, so d^T u = |d| g = 5g",
         "IMPOSED_DISPLACEMENT_SKEW_SUPPORT_UNSUPPORTED")
    case("K3_A_ground_x_with_ux_held", [1, 1, 1], load_y, both(axis("ux", 0), stop("stop", [3, 4, 0], 3 * g)), "S-K3",
         "meaning A with an axis row (review P2-5): the ground moves by Delta = g e_x, so h = d^T Delta = 3g; Delta "
         "violates UX, and the complete system gives u0 = (0, 3g/4, 0), u_y = 3g/4",
         "IMPOSED_DISPLACEMENT_SKEW_SUPPORT_UNSUPPORTED")
    case("K3_A_guide_plus_stop_ground_y", [1, 1, 1], load_y, both(axis("guide_uz", 2), stop("stop", [3, 4, 5], 4 * g)), "S-K3",
         "meaning A with UZ held and stop (3,4,5): Delta = g e_y, h = 4g; u0 = (0, g, 0)",
         "IMPOSED_DISPLACEMENT_SKEW_SUPPORT_UNSUPPORTED")
    sqrt_case("K3_B_plane_13", [], [1, 3, 0], F(0), g,
              "meaning B with an irrational norm: d = (1,3,0), d^T u = sqrt(10) g")
    sqrt_case("K3_B_guide_plus_stop_345", [("guide_uz", 2, F(0))], [3, 4, 5], F(0), 5 * g,
              "meaning B with UZ held: d = (3,4,5), d^T u = 5 sqrt(2) g")
    sqrt_case("K3_B_moving_guide_plus_stop_345", [("guide_uz", 2, g / 2)], [3, 4, 5], F(0), 5 * g,
              "meaning B with UZ moved by g/2 and the stop's ground by g: d^T u = 5 sqrt(2) g, u_z = g/2")
    return out


def mutant_case() -> dict:
    third = F(1 / 3)
    assert third == F(6004799503160661, 2 ** 54)
    residual_per_v = 1 - 3 * third  # d^T (column of the rounded-division basis) per unit coordinate
    return {"name": "R3_rounded_division_mutant", "direction": [1.0, 3.0],
            "mutant_column": [num(F(1)), num(-third)],
            "exact_constraint_residual_per_unit_coordinate": num(residual_per_v),
            "statement": "the canonical basis gives d^T(Q v) = 0 exactly for every v; the rounded-division basis "
                         "c = (fl(1/3), 1) gives d^T(T_mut v) = 2^-54 v, so an exact check of the unrounded lift kills it"}


# ---------------------------------------------------------------- product set (guided cantilever, SI)

LEAVES = {"A_m2": 0.0067387162419501045, "I_m4": 3.213188372067858e-05, "J_m4": 6.426376744135716e-05}
E, G, L = F(5 * 2 ** 35), F(2 ** 36), F(4)


def element_matrix() -> list[list[F]]:
    a, i, j = (F(LEAVES[k]) for k in ("A_m2", "I_m4", "J_m4"))
    k = [[F(0)] * 12 for _ in range(12)]

    def put(r, c, v):
        k[r][c] = v
        k[c][r] = v
    ea, gj = E * a / L, G * j / L
    b3, b2, b1, b1h = 12 * E * i / L ** 3, 6 * E * i / L ** 2, 4 * E * i / L, 2 * E * i / L
    put(0, 0, ea); put(6, 6, ea); put(0, 6, -ea)
    put(3, 3, gj); put(9, 9, gj); put(3, 9, -gj)
    # xy plane: v (1, 7), theta_z (5, 11)
    put(1, 1, b3); put(7, 7, b3); put(1, 7, -b3)
    put(1, 5, b2); put(1, 11, b2); put(5, 7, -b2); put(7, 11, -b2)
    put(5, 5, b1); put(11, 11, b1); put(5, 11, b1h)
    # xz plane: w (2, 8), theta_y (4, 10)
    put(2, 2, b3); put(8, 8, b3); put(2, 8, -b3)
    put(2, 4, -b2); put(2, 10, -b2); put(4, 8, b2); put(8, 10, b2)
    put(4, 4, b1); put(10, 10, b1); put(4, 10, b1h)
    return k


def product_cases() -> tuple[dict, list[dict]]:
    p = F(1024)
    ks = F(2 ** 20)
    spec = {
        "units": "N, m, rad",
        "nodes": {"A": [0, 0, 0], "B": [4, 0, 0]},
        "member": "one straight Euler-Bernoulli member A-B (model_settings shear_model euler_bernoulli)",
        "material": {"E_Pa": num(E), "G_Pa": num(G)},
        "section": "OD 0.206 m, wall 0.011 m; the leaves below are the binary64 values the product forms "
                   "from them, which the product-side consumer checks bit for bit",
        "leaves": {k: v.hex() for k, v in LEAVES.items()},
        "records": "A: anchor (all six DOFs). B: one rigid record restraining RX, RY and RZ (`rot_B`), "
                   "plus each case's records at B. Load: a concentrated nodal force at B.",
        "condensed_tip_stiffness": {"k_axial_EA_over_L": num(E * F(LEAVES['A_m2']) / L),
                                    "k_transverse_12EI_over_L3": num(12 * E * F(LEAVES['I_m4']) / L ** 3)},
        "skew_spring_stiffness_N_per_m": num(ks),
        "load_magnitude_P_N": num(p),
    }
    out = []

    def case(name, load, build, slice_, note, refusal_s_k1=None, twin=None):
        k = element_matrix()
        m = Model(12, k, place(12, 6, load))
        m.rigid("anchor_A", [(unit(12, i), F(0)) for i in range(6)])
        m.rigid("rot_B", [(unit(12, 6 + i), F(0)) for i in (3, 4, 5)])
        build(m)
        result = m.solve()
        entry = {"name": name, "load_at_B": vec(load), "slice": slice_, "note": note}
        if refusal_s_k1:
            entry["s_k1_refusal"] = refusal_s_k1
        if twin:
            entry["byte_identical_twin"] = twin
        if result is None:
            entry["expect"] = {"code": "SUPPORT_CONSTRAINT_REDUNDANT"}
        else:
            u, lam = result
            assert all(x == 0 for x in u[:6] + u[9:12]), "A fixed, B rotations fixed"
            entry["expect"] = {"u_B_translations": vec(u[6:9]), "records": record_results(m, u, lam)}
            info = single_stop_info(m, u, lam, 6)
            if info:
                entry["expect"]["attribution"] = info
        out.append(entry)

    def stop(name, d, g=F(0)):
        return lambda m: m.rigid(name, [(place(12, 6, d), g)], kind="skew", direction=d)

    def axis(name, i, g=F(0)):
        return lambda m: m.rigid(name, [(unit(12, 6 + i), g)])

    def spring(name, d, k=ks):
        return lambda m: m.spring(name, 6, d, k)

    def both(*fs):
        def run(m):
            for f in fs:
                f(m)
        return run

    ly = [F(0), -p, F(0)]
    case("P_S1_spring", ly, spring("spring_B", [3, 4, 0]), "S-K1", "skew spring k_s along (3,4,0)")
    case("P_S1_spring_scaled_2m600", ly, spring("spring_B", [F(3) * F(2) ** -600, F(4) * F(2) ** -600, 0]), "S-K1",
         "P_S1_spring with d scaled by 2^-600: bit-for-bit identical outputs")
    case("P_S2_spring_pair", ly, both(spring("spring_1", [3, 4, 0]), spring("spring_2", [-4, 3, 0])), "S-K1",
         "the pair's exact block equals axis springs k_s on UX and UY: u equals the axis twin's exact u")
    case("P_S2_axis_twin", ly, both(spring("spring_ux", [1, 0, 0]), spring("spring_uy", [0, 1, 0])), "today",
         "today's axis springs: the twin of P_S2_spring_pair")
    case("P_R1_stop", ly, stop("stop_B", [3, 4, 0]), "S-K1", "skew rigid stop along (3,4,0)")
    case("P_R1_stop_scaled_2p600", ly, stop("stop_B", [F(3) * F(2) ** 600, F(4) * F(2) ** 600, 0]), "S-K1",
         "bit-for-bit identical to P_R1_stop")
    case("P_R1_stop_scaled_2m600", ly, stop("stop_B", [F(3) * F(2) ** -600, F(4) * F(2) ** -600, 0]), "S-K1",
         "bit-for-bit identical to P_R1_stop")
    case("P_R1_stop_times_3", ly, stop("stop_B", [9, 12, 0]), "S-K1",
         "the same exact system as P_R1_stop (positive factor 3); binary64 outputs may differ by rounding")
    case("P_R1_stop_negative", ly, stop("stop_B", [-3, -4, 0]), "S-K1",
         "the same u and global force as P_R1_stop; the along-direction scalar negated")
    case("P_R3_stop_plane_13", ly, stop("stop_B", [1, 3, 0]), "S-K1", "irrational along-direction scalar")
    case("P_R4_guide_plus_stop", ly, both(axis("guide_uz_B", 2), stop("stop_B", [3, 4, 5])), "S-K1",
         "UZ rigid record plus stop (3,4,5): reduced to 3u_x + 4u_y = 0 on F = {x, y}")
    case("P_R4_full_fix", ly, both(axis("ux_B", 0), axis("uz_B", 2), stop("stop_B", [3, 4, 0])), "S-K1",
         "the node's translations fixed; attribution unique")
    case("P_R4_tiny_restricted_direction", ly, both(axis("ux_B", 0), axis("uz_B", 2), stop("stop_B", [1, F(2) ** -600, 0])), "S-K1",
         "review P1-1's regression at product level: the node fixed, d_F = (2^-600); attribution must rescale d_F exactly")
    case("P_R4_redundant", ly, both(axis("ux_B", 0), axis("uy_B", 1), stop("stop_B", [3, 4, 0])), "S-K1",
         "redundant: SUPPORT_CONSTRAINT_REDUNDANT")
    case("P_RS_stop_and_spring", ly, both(stop("stop_B", [3, 4, 0]), spring("spring_B", [0, 3, 4])), "S-K1",
         "a skew stop and a skew spring at B")
    case("P_AX_axis_y_direction", ly, axis("stop_B", 1), "today",
         "a line_stop with token UX and direction (0,2,0) lowers to UY: byte-identical to a plain UY line_stop",
         twin="the same document with the record's restraint UY and no direction")
    case("P_AX_axis_neg_x_direction", [p, F(0), F(0)], axis("stop_B", 0), "today",
         "a line_stop with token UX and direction (-1,0,0): byte-identical to a plain UX line_stop (no along-direction row)",
         twin="the same document without direction")
    case("P_K2_two_stops_3d", ly, both(stop("stop_1", [3, 4, 0]), stop("stop_2", [0, 3, 4])), "S-K2",
         "two stops at B", "SUPPORT_SKEW_STOPS_PER_NODE_UNSUPPORTED")
    case("P_K2_orthogonal_pair", [F(0), -p, -p], both(stop("stop_1", [3, 4, 0]), stop("stop_2", [-4, 3, 0])), "S-K2",
         "two orthogonal stops: the xy translations fixed", "SUPPORT_SKEW_STOPS_PER_NODE_UNSUPPORTED")
    case("P_K2_near_parallel", ly, both(stop("stop_1", [3, 4, 0]), stop("stop_2", [3, 4 + F(2) ** -48, 0])), "S-K2",
         "exact rank 2 with multipliers of order 2^48 P", "SUPPORT_SKEW_STOPS_PER_NODE_UNSUPPORTED")
    case("P_K2_parallel", ly, both(stop("stop_1", [3, 4, 0]), stop("stop_2", [6, 8, 0])), "S-K2",
         "redundant", "SUPPORT_SKEW_STOPS_PER_NODE_UNSUPPORTED")
    h = F(1, 1024)
    case("P_K3_moving_guide_plus_stop", ly, both(axis("guide_uz_B", 2, h), stop("stop_B", [3, 4, 5])), "S-K3",
         "UZ moved by h = 2^-10 m: 3u_x + 4u_y = -5h", "SUPPORT_SKEW_NODE_PRESCRIBED_MOTION_UNSUPPORTED")
    case("P_K3_stop_ground_meaning_A", ly, stop("stop_B", [3, 4, 0], 4 * h), "S-K3",
         "meaning A: the stop's ground moves by h e_y: d^T u = 4h", "IMPOSED_DISPLACEMENT_SKEW_SUPPORT_UNSUPPORTED")
    case("P_K3_stop_ground_meaning_B", ly, stop("stop_B", [3, 4, 0], 5 * h), "S-K3",
         "meaning B: the stop's ground moves by h along +d/|d|: d^T u = 5h", "IMPOSED_DISPLACEMENT_SKEW_SUPPORT_UNSUPPORTED")
    case("P_K3_A_ground_x_with_ux_held", ly, both(axis("ux_B", 0), stop("stop_B", [3, 4, 0], 3 * h)), "S-K3",
         "meaning A with UX held at B: Delta = h e_x gives d^T u = 3h; Delta is not a particular solution",
         "IMPOSED_DISPLACEMENT_SKEW_SUPPORT_UNSUPPORTED")
    return spec, out


# ---------------------------------------------------------------- modal kernel (point mass m at a node)


def modal_cases() -> list[dict]:
    """K = diag(k_a, k_b, k_c), M = m I; units: omega^2 in k/m with k = m = 1."""
    out = []
    for name, kd, stops, springs in [
        ("M1_stop_anisotropic", [2, 1, 3], [[3, 4, 0]], []),
        ("M2_spring_isotropic", [1, 1, 1], [], [([3, 4, 0], 1)]),
        ("M3_stop_isotropic", [1, 1, 1], [[3, 4, 0]], []),
    ]:
        k = [[F(kd[i]) if i == j else F(0) for j in range(3)] for i in range(3)]
        for d, ks in springs:
            d = [F(x) for x in d]
            for i in range(3):
                for j in range(3):
                    k[i][j] += F(ks) * d[i] * d[j] / dot(d, d)
        basis = nullspace([[F(x) for x in s] for s in stops], 3)
        kt = [[dot(bi, matvec(k, bj)) for bj in basis] for bi in basis]
        mt = [[dot(bi, bj) for bj in basis] for bi in basis]
        # Eigenvalues of the 2x2 or 3x3 pencil via the characteristic polynomial (exact when rational).
        n = len(basis)
        if n == 2:
            a, b, c = kt[0][0], kt[0][1], kt[1][1]
            p_, q_, r_ = mt[0][0], mt[0][1], mt[1][1]
            # det(K - w M) = 0: (p r - q^2) w^2 - (a r + c p - 2 b q) w + (a c - b^2) = 0
            A2, B2, C2 = p_ * r_ - q_ * q_, -(a * r_ + c * p_ - 2 * b * q_), a * c - b * b
            disc = B2 * B2 - 4 * A2 * C2
            root = is_square(disc)
            assert root is not None, name
            eig = sorted([(-B2 - root) / (2 * A2), (-B2 + root) / (2 * A2)])
        else:
            # diagonal plus rank-1: eigenvalues of k; here the isotropic spring case
            assert name == "M2_spring_isotropic"
            eig = sorted([F(1), F(1), F(2)])
        out.append({"name": name, "k_diagonal": [num(F(x)) for x in kd], "stops": stops,
                    "springs": [{"direction": d, "k": ks} for d, ks in springs],
                    "omega_squared": [num(x) for x in eig],
                    "statement": "every mode shape is orthogonal to each stop's d exactly; the spring raises only the d mode"})
    return out


# ---------------------------------------------------------------- negatives


def negatives() -> list[dict]:
    return [
        {"code": "SUPPORT_DIRECTION_INVALID", "stage": "document validator (applier and solve)", "inputs": "non-number, missing, null or non-finite component"},
        {"code": "SUPPORT_DIRECTION_ZERO", "stage": "document validator", "inputs": "(0,0,0), (-0,0,-0)"},
        {"code": "SUPPORT_DIRECTION_RECORD_UNSUPPORTED", "stage": "document validator", "inputs": "an anchor, guide, inferred family, nonlinear member or two DOFs with direction"},
        {"code": "SUPPORT_DIRECTION_ROTATIONAL_UNSUPPORTED", "stage": "document validator", "inputs": "a line_stop on RX with direction (1,1,0)"},
        {"code": "SUPPORT_DIRECTION_SKEW_EXACT_UNSUPPORTED", "stage": "skew admission (solve and applier)", "inputs": "P_R1_stop on a document with a pressure contract"},
        {"code": "SUPPORT_DIRECTION_SKEW_NONLINEAR_MODEL_UNSUPPORTED", "stage": "skew admission (solve)", "inputs": "P_R1_stop plus any nonlinear support in the model"},
        {"code": "SUPPORT_DIRECTION_RANGE_UNSUPPORTED", "stage": "skew admission (solve and applier)", "inputs": "direction_policy range_* cases"},
        {"code": "SUPPORT_DIRECTION_NODE_UNCONNECTED", "stage": "boundary preparation", "inputs": "a skew record on a node with no element"},
        {"code": "SUPPORT_SKEW_STOPS_PER_NODE_UNSUPPORTED", "stage": "boundary preparation (S-K1 only)", "inputs": "every K2_* and P_K2_* case"},
        {"code": "SUPPORT_CONSTRAINT_REDUNDANT", "stage": "boundary preparation", "inputs": "R4_redundant_axes, P_R4_redundant (S-K1); K2_parallel and the other redundant K2 cases (S-K2)"},
        {"code": "IMPOSED_DISPLACEMENT_SKEW_SUPPORT_UNSUPPORTED", "stage": "imposed-motion validation", "inputs": "an imposed_displacement primitive targeting a skewed record"},
        {"code": "SUPPORT_SKEW_NODE_PRESCRIBED_MOTION_UNSUPPORTED", "stage": "case solve, before reduction", "inputs": "K3_moving_guide_plus_stop, P_K3_moving_guide_plus_stop"},
        {"code": "SUPPORT_SKEW_REACTION_RANGE_UNSUPPORTED", "stage": "case publication (range-safe attribution, design 7.2)", "inputs": "a constrained node whose multiplier, attributed component or along-direction value leaves binary64"},
        {"code": "SUPPORT_DIRECTION_SKEW_FORCE_SCALED_UNSUPPORTED", "stage": "case solve (W2 not attempted; a range-deferred basis)", "inputs": "a skew model whose ordinary formation leaves the binary64 range"},
    ]


def regressions(kernel: list[dict]) -> dict:
    """The independent review's findings on revision 1, as exact regressions."""
    by = {c["name"]: c for c in kernel}
    out = {}
    # P1-3: the skewed spring's along-direction value branches from -k t.
    springs = []
    for name in ("S1_spring_isotropic", "S2_spring_pair", "RS_stop_and_spring"):
        m = Model(3, tip([1, 1, 1]), [F(0), F(-1), F(0)])
        if name == "S1_spring_isotropic":
            m.spring("s", 0, [3, 4, 0], 1)
        elif name == "S2_spring_pair":
            m.spring("s1", 0, [3, 4, 0], 1)
            m.spring("s2", 0, [-4, 3, 0], 1)
        else:
            m.rigid("stop", [([F(3), F(4), F(0)], F(0))], kind="skew", direction=[3, 4, 0])
            m.spring("spring", 0, [0, 3, 4], 1)
        u, lam = m.solve()
        for sp in m.springs:
            dt = [F(x) for x in classify([float(x) for x in sp["c"]])["scaled"]]
            t, n = dot(dt, u), dot(dt, dt)
            kt = sp["k"] * t
            comps = [-kt * x / n for x in dt]
            assert [num(x) for x in comps] == by[name]["expect"]["records"][sp["name"]]["global_force"]
            correct, wrong = kt * kt / n, kt * kt / n ** 3
            assert scalar((-kt > 0) - (-kt < 0), correct) == by[name]["expect"]["records"][sp["name"]]["along_direction"]
            springs.append({"case": name, "spring": sp["name"], "t_scaled": num(t), "n_scaled": num(n),
                            "components_minus_kt_d_over_n": [num(x) for x in comps],
                            "along_minus_kt_over_norm": scalar((-kt > 0) - (-kt < 0), correct),
                            "rejected_chain_minus_kt_over_n_over_norm": scalar((-kt > 0) - (-kt < 0), wrong)})
    assert springs[0]["along_minus_kt_over_norm"]["exact"] == "2/5"
    assert springs[0]["rejected_chain_minus_kt_over_n_over_norm"]["exact"] == "32/125"
    out["spring_along_direction"] = springs
    # P1-2: physical reaction rows must include every skewed spring block.
    m = Model(3, tip([1, 1, 1]), [F(0), F(-1), F(0)])
    m.rigid("stop", [([F(3), F(4), F(0)], F(0))], kind="skew", direction=[3, 4, 0])
    m.spring("spring", 0, [0, 3, 4], 1)
    u, lam = m.solve()
    full = [ku - f for ku, f in zip(matvec(m.k, u), m.f)]
    c = [F(0), F(3), F(4)]
    block = [[x * y / dot(c, c) for y in c] for x in c]
    omitted = [r - b for r, b in zip(full, matvec(block, u))]
    assert full == [lam[0] * x for x in (F(3), F(4), F(0))], "with the block, r = lambda d exactly"
    assert omitted[2] == F(54, 553), "the reviewer's value"
    out["reaction_rows_include_skew_springs"] = {
        "case": "RS_stop_and_spring", "rows_with_block": vec(full), "rows_block_omitted": vec(omitted),
        "statement": "with the spring block r = lambda (3,4,0) exactly; omitting it leaves r_z = 54/553 P, so the "
                     "attribution's consistency check would fail on a valid case"}
    # P1-1: attribution's n_F underflows; the rescaled n_w does not.
    att = by["R4_tiny_restricted_direction"]["expect"]["attribution"]
    assert att["n_F_rounds_to_zero"] and att["restricted_scale_exponent"] == 600 and att["n_w"]["exact"] == "1/1"
    out["attribution_range"] = {"case": "R4_tiny_restricted_direction", "attribution": att}
    # P2-4: a stop-fixed node's physical translations have no free coordinate.
    ff = by["R4_full_fix_one_free_axis"]["expect"]["attribution"]["lifted_remainder_exponent"]
    assert set(ff.values()) == {"i32::MIN"}
    out["lifted_remainder_exponent_minimum"] = {"case": "R4_full_fix_one_free_axis", "exponents": ff}
    # P2-5: a ground displacement is not a particular solution.
    a = by["K3_A_ground_x_with_ux_held"]["expect"]["attribution"]["particular_solution_u0"]
    assert [x["exact"] for x in a] == ["0/1", "3/4096", "0/1"]
    out["particular_solution_not_ground_displacement"] = {
        "case": "K3_A_ground_x_with_ux_held", "ground_displacement_delta": vec([F(1, 1024), F(0), F(0)]),
        "violated_row": "ux (u_x = 0)", "particular_solution_u0": a}
    return out


# ---------------------------------------------------------------- build


def build() -> dict[str, bytes]:
    directions = direction_cases()
    spec, product = product_cases()
    kernel = kernel_cases()
    # Spot checks against the adopted design's stated values (section 2.6).
    by = {c["name"]: c for c in kernel}
    assert by["S1_spring_isotropic"]["expect"]["u"][0]["exact"] == "6/25"
    assert by["R1_stop_isotropic"]["expect"]["records"]["stop"]["along_direction"]["exact"] == "4/5"
    assert by["R2_stop_anisotropic"]["expect"]["records"]["stop"]["along_direction"]["exact"] == "40/41"
    assert by["R3_stop_negative"]["expect"]["records"]["stop"]["along_direction"]["exact"] == "-4/5"
    assert by["R3_stop_plane_13"]["expect"]["records"]["stop"]["along_direction"]["square"] == "9/10"
    assert by["R4_guide_plus_stop"]["expect"]["records"]["guide_uz"]["components"][0]["exact"] == "-4/5"
    assert [x["exact"] for x in by["K2_two_stops_3d"]["expect"]["u"]] == ["192/481", "-144/481", "108/481"]
    assert by["S2_spring_pair"]["expect"]["u"] == by["S2_axis_twin"]["expect"]["u"]
    pby = {c["name"]: c for c in product}
    assert pby["P_S2_spring_pair"]["expect"]["u_B_translations"] == pby["P_S2_axis_twin"]["expect"]["u_B_translations"]
    for a, b in [("P_R1_stop", "P_R1_stop_scaled_2p600"), ("P_R1_stop", "P_R1_stop_times_3"), ("P_R1_stop", "P_R1_stop_negative")]:
        assert pby[a]["expect"]["u_B_translations"] == pby[b]["expect"]["u_B_translations"]
    references = {
        "basis": "DEL-04-03 Work skew-restraints.md; DEL-05-02 Work operating-case.md sections 2.2-2.6; generated by generators/skew_reference.py",
        "direction_policy": directions,
        "invariance": invariance_groups(directions),
        "kernel": {"units": "forces in P, displacements in P/k (k = P = 1)", "cases": kernel},
        "mutant": mutant_case(),
        "product": {"spec": spec, "cases": product},
        "modal_kernel": modal_cases(),
        "regressions": regressions(kernel),
        "negatives": negatives(),
    }
    text = (json.dumps(references, indent=1, sort_keys=True) + "\n").encode()
    sums = "".join(f"{hashlib.sha256(data).hexdigest()}  {name}\n" for name, data in [
        ("README.md", (HERE / "README.md").read_bytes()),
        ("generators/skew_reference.py", (HERE / "generators" / "skew_reference.py").read_bytes()),
        ("references.json", text)])
    return {"references.json": text, "SHA256SUMS": sums.encode()}


def main() -> int:
    outputs = build()
    if "--write" in sys.argv[1:]:
        for name, data in outputs.items():
            (HERE / name).write_bytes(data)
        return 0
    stale = [name for name, data in outputs.items() if not (HERE / name).exists() or (HERE / name).read_bytes() != data]
    print("stale: " + ", ".join(stale) if stale else "references are current")
    return 1 if stale else 0


if __name__ == "__main__":
    raise SystemExit(main())
