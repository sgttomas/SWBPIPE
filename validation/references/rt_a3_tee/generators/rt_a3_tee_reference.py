#!/usr/bin/env python3
"""RT-A3 increment 1 independent references: tees on the exact pressure route.

Standard-library Python 3.9+ only. This generator imports, reads and runs no
product code and reads no file other than the ones it writes (and the two fixed
files it hashes). Every expected value is derived here in exact arithmetic by
two independent formulations that must agree on every published value before
anything is written:

  route 1  an exact direct-stiffness solve of the space frame: 12 x 12
           Euler-Bernoulli member stiffness in member axes, rotated with exact
           direction cosines; the Poisson pair as a held axial eigenstrain
           -2 nu p A_i/(E A); terminal caps -p A_i s as nodal loads; partitioned
           solve of the free DOFs; end actions K_e u_e + held end actions;
  route 2  closed forms per case: statics of determinate legs, the textbook
           end-action laws of a member with one fixed end (12EI/L^3, 6EI/L^2,
           4EI/L, EA/L, GJ/L), the cantilever tip laws (L^3/3EI, L^2/2EI), the
           R-4 sec. 3.1 T-4 formula for the free junction and the symmetric
           header's diagonal node stiffness for the bending case.

Every section property of an annulus is a rational multiple of pi, and the
pressure loads p A_i are too; mechanical forces are rational. Every value is
therefore a Laurent polynomial in pi with rational coefficients, carried exactly
(class P). A tee row i |M| / Z is published through its exact square.
All model data are invented.

Modes:
  python3 -I rt_a3_tee_reference.py           check: rebuild every output in memory and
  python3 -I rt_a3_tee_reference.py --check   compare byte for byte with the committed
                                              files, SHA256SUMS included
  python3 -I rt_a3_tee_reference.py --write   write references.json and SHA256SUMS
                                              (intentional changes only)
"""
import argparse
import hashlib
import json
import math
import sys
from decimal import Decimal, getcontext
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE.parent
FIXED_FILES = ("README.md", "generators/rt_a3_tee_reference.py")

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


PI_DEC = 16 * _atan_inv(5) - 4 * _atan_inv(239)  # Machin, about 95 correct digits
_PI_90 = F(str(PI_DEC)[:92])
PI_LO, PI_HI = _PI_90, _PI_90 + F(1, 10 ** 90)
assert PI_LO < F(355, 113) and F(333, 106) < PI_LO


class P:
    """An exact Laurent polynomial in pi: {power: rational coefficient}."""
    __slots__ = ("t",)

    def __init__(self, t=None):
        self.t = {}
        for k, v in (t or {}).items():
            v = F(v)
            if v:
                self.t[k] = v

    @classmethod
    def of(cls, x):
        return x if isinstance(x, P) else cls({0: F(x)})

    @classmethod
    def pi(cls, q, k=1):
        return cls({k: F(q)})

    def __add__(self, o):
        o = P.of(o)
        t = dict(self.t)
        for k, v in o.t.items():
            t[k] = t.get(k, 0) + v
        return P(t)

    __radd__ = __add__

    def __neg__(self):
        return P({k: -v for k, v in self.t.items()})

    def __sub__(self, o):
        return self + (-P.of(o))

    def __rsub__(self, o):
        return P.of(o) - self

    def __mul__(self, o):
        if isinstance(o, P):
            t = {}
            for k1, v1 in self.t.items():
                for k2, v2 in o.t.items():
                    t[k1 + k2] = t.get(k1 + k2, 0) + v1 * v2
            return P(t)
        o = F(o)
        return P({k: v * o for k, v in self.t.items()})

    __rmul__ = __mul__

    def __truediv__(self, o):
        if isinstance(o, P):
            if len(o.t) != 1:
                raise ValueError("division by a polynomial in pi with more than one term")
            (k, c), = o.t.items()
            return P({j - k: v / c for j, v in self.t.items()})
        o = F(o)
        return P({k: v / o for k, v in self.t.items()})

    def __rtruediv__(self, o):
        return P.of(o) / self

    def __eq__(self, o):
        try:
            return self.t == P.of(o).t
        except TypeError:
            return NotImplemented

    __hash__ = None

    def is_zero(self):
        return not self.t

    def coeff(self, k):
        return self.t.get(k, F(0))

    def text(self):
        if not self.t:
            return "0"
        parts = []
        for k in sorted(self.t):
            q = str(self.t[k])
            parts.append(q if k == 0 else f"{q}*pi" if k == 1 else f"{q}*pi^{k}" if k > 1
                         else f"{q}/pi" if k == -1 else f"{q}/pi^{-k}")
        return " + ".join(parts)

    def bounds(self):
        lo = hi = F(0)
        for k, v in self.t.items():
            a, b = v * PI_LO ** k, v * PI_HI ** k
            lo += min(a, b)
            hi += max(a, b)
        return lo, hi

    def dec(self):
        getcontext().prec = 100
        return sum((Decimal(v.numerator) / Decimal(v.denominator) * PI_DEC ** k for k, v in self.t.items()), Decimal(0))


def binary64(p):
    lo, hi = p.bounds()
    a, b = float(lo), float(hi)
    if a != b:
        raise SystemExit(f"binary64 rounding of {p.text()} is not decided by the pi bounds")
    return a


def dec40(d):
    return "0" if d == 0 else format(d, ".39e")


def record(x):
    p = P.of(x)
    return {"exact": p.text(), "decimal": dec40(p.dec()), "value": binary64(p)}


def _perfect_square(q):
    def isq(n):
        r = math.isqrt(n)
        return r if r * r == n else None
    a, b = isq(q.numerator), isq(q.denominator)
    return None if a is None or b is None else F(a, b)


def sqrt_record(sq):
    """A nonnegative square root published through its exact square sq (a P)."""
    if sq.is_zero():
        return {"exact": "0", "exact_square": "0", "decimal": "0", "value": 0.0}
    lo, hi = sq.bounds()
    if lo <= 0:
        raise SystemExit("negative square")
    getcontext().prec = 130
    def dsqrt(q):
        return (Decimal(q.numerator) / Decimal(q.denominator)).sqrt()
    s_lo = F(dsqrt(lo)) * (1 - F(1, 10 ** 120))
    s_hi = F(dsqrt(hi)) * (1 + F(1, 10 ** 120))
    assert s_lo * s_lo <= lo and s_hi * s_hi >= hi
    if float(s_lo) != float(s_hi):
        raise SystemExit(f"binary64 rounding of sqrt({sq.text()}) is not decided")
    getcontext().prec = 100
    d = sq.dec().sqrt()
    if len(sq.t) == 1:
        (k, c), = sq.t.items()
        root = _perfect_square(c)
        if k % 2 == 0 and root is not None:
            exact = P.pi(root, k // 2).text()
        elif k % 2 == 0:
            exact = f"sqrt({c})" + ("" if k == 0 else f"*pi^{k // 2}" if k > 2 else "*pi" if k == 2
                                    else "/pi" if k == -2 else f"/pi^{-k // 2}")
        else:
            exact = f"sqrt({sq.text()})"
    else:
        exact = f"sqrt({sq.text()})"
    return {"exact": exact, "exact_square": sq.text(), "decimal": dec40(d), "value": float(s_lo)}


# ----------------------------------------------------------------------------- vectors
def vec(*a):
    return [P.of(x) for x in a]


def add(a, b):
    return [x + y for x, y in zip(a, b)]


def sub(a, b):
    return [x - y for x, y in zip(a, b)]


def mul(k, a):
    return [x * k for x in a]


def dot(a, b):
    return sum((x * y for x, y in zip(a, b)), P())


def crossp(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def rational(p):
    p = P.of(p)
    assert set(p.t) <= {0}, p.text()
    return p.coeff(0)


def exact_sqrt(q):
    r = _perfect_square(F(q))
    if r is None:
        raise SystemExit(f"{q} is not a perfect square")
    return r


def frame(xi, xj, yref):
    """Member axes (the product convention): x from `from` to `to`, y the y_reference
    less its x part, normalized, z = x cross y. Exact: every member is axis-aligned."""
    d = [rational(c) for c in sub(xj, xi)]
    length = exact_sqrt(sum(c * c for c in d))
    ex = [c / length for c in d]
    yr = [rational(c) for c in yref]
    yv = [a - sum(p * q for p, q in zip(yr, ex)) * b for a, b in zip(yr, ex)]
    yn = exact_sqrt(sum(c * c for c in yv))
    ey = [c / yn for c in yv]
    ez = [ex[1] * ey[2] - ex[2] * ey[1], ex[2] * ey[0] - ex[0] * ey[2], ex[0] * ey[1] - ex[1] * ey[0]]
    return length, vec(*ex), vec(*ey), vec(*ez)


# ----------------------------------------------------------------------------- inputs (invented)
E = F(200 * 10 ** 9)  # Pa
NU = F(3, 10)
G = E / (2 * (1 + NU))  # derived, 10^12/13 Pa
assert G == F(10 ** 12, 13)
PRESSURE = F(10 ** 7)  # Pa, internal differential
Y_REF = vec(0, 0, 1)
HEADER = {"D": F(1, 5), "t": F(1, 100)}
BRANCH = {"D": F(1, 10), "t": F(1, 200)}
I_HEADER, I_BRANCH = F(21, 10), F(8, 5)
CA = F(1, 1000)  # m, the corrosion-allowance variants
CM = F(1, 2000)  # m, the mill-tolerance variant
F_C = vec(2000, 0, -1000)  # N, T5's nodal force at C
TRANSFER, COMPENSATED = "transfers_to_wall", "separately_supported_or_compensated"


def section(dims, c_m, c_a):
    """D-5 (section-bases-and-timoshenko.md sec. 1): nominal wall t; reduced wall
    t_r = (t - c_m) - c_a; r_o = D/2 for both; pressure bore r_i,r = D/2 - t_r."""
    D, t = dims["D"], dims["t"]
    ro = D / 2
    t_r = (t - c_m) - c_a
    assert 0 < t_r <= t

    def annulus(wall):
        ri = ro - wall
        inertia = (ro ** 4 - ri ** 4) / 4
        return {"wall": wall, "ri": ri, "A": P.pi(ro ** 2 - ri ** 2), "I": P.pi(inertia),
                "J": P.pi(2 * inertia), "Z": P.pi(inertia / ro)}
    rir = ro - t_r
    return {"D": D, "t": t, "c_m": c_m, "c_a": c_a, "ro": ro, "nominal": annulus(t), "reduced": annulus(t_r),
            "bore": {"ri": rir, "Ai": P.pi(rir ** 2)}}


BASES = ("nominal", "reduced")
BASIS_NAME = {"nominal": "nominal_section", "reduced": "reduced_section"}

CROSS = []


def cross(name, a, b):
    ok = a == b
    CROSS.append({"check": name, "agrees": ok})
    if not ok:
        print(f"DISAGREES {name}: {a} != {b}")
    return ok


# ----------------------------------------------------------------------------- cases
TEE_NODES = {"node:T": vec(0, 0, 0), "node:A": vec(-4, 0, 0), "node:B": vec(4, 0, 0), "node:C": vec(0, 3, 0)}
TEE_PIPES = [("pipe:A", "node:T", "node:A", "header"), ("pipe:B", "node:T", "node:B", "header"),
             ("pipe:C", "node:T", "node:C", "branch")]
STEP_NODES = {"node:S": vec(0, 0, 0), "node:P": vec(-4, 0, 0), "node:Q": vec(4, 0, 0)}
STEP_PIPES = [("pipe:1", "node:S", "node:P", "header"), ("pipe:2", "node:S", "node:Q", "branch")]
TEE = {"node": "node:T", "header_pipe_ref": "pipe:A", "branch_pipe_ref": "pipe:C",
       "i_header": I_HEADER, "i_branch": I_BRANCH}


def make_case(cid, status, purpose, nodes, pipes, supports, hand, hub, c_m=F(0), c_a=F(0), terminals=None,
              loads=None, tee=None, notes=()):
    terminals = terminals or {}
    secs = {"header": section(HEADER, c_m, c_a), "branch": section(BRANCH, c_m, c_a)}
    members = [{"id": pid, "from": a, "to": b, "section_name": s, "section": secs[s]} for pid, a, b, s in pipes]
    regions = []
    for m in members:
        regions.append({"id": "region:" + m["id"].split(":")[1], "pipe": m["id"], "pressure": PRESSURE,
                        "terminals": {m["from"]: terminals.get((m["id"], m["from"]), TRANSFER),
                                      m["to"]: terminals.get((m["id"], m["to"]), TRANSFER)}})
    return {"id": cid, "status": status, "purpose": purpose, "nodes": nodes, "members": members, "regions": regions,
            "supports": supports, "loads": loads or [], "tee": tee, "hand": hand, "hub": hub, "notes": list(notes)}


def build_case_list():
    anchor_t = {"node:T": "anchor"}
    far = {"node:A": "anchor", "node:B": "anchor", "node:C": "anchor"}
    c = []
    c.append(make_case("T1", "case", "anchored tee, three capped free legs: the closed tee is self-equilibrated",
                       TEE_NODES, TEE_PIPES, anchor_t, "anchored_hub", "node:T", tee=TEE,
                       notes=["R-4 T-1 (with `junction` at T in R-4; here every terminal at T is transfers_to_wall, "
                              "which the R-4 ledger treats identically)"]))
    c.append(make_case("T1-CA", "case", "T1 with corrosion allowance: the two bases and the pressure bore differ",
                       TEE_NODES, TEE_PIPES, anchor_t, "anchored_hub", "node:T", c_a=CA, tee=TEE))
    c.append(make_case("T2", "case", "R-4 T-2: as T1 with pipe:C's far terminal (at C) separately_supported_or_compensated",
                       TEE_NODES, TEE_PIPES, anchor_t, "anchored_hub", "node:T", tee=TEE,
                       terminals={("pipe:C", "node:C"): COMPENSATED},
                       notes=["the compensated terminal is a single region's closure at an unshared node"]))
    c.append(make_case("T2-CTRL", "mutation_control",
                       "dropped-branch-cap control: T1 with pipe:C's cap at T omitted (not an admitted case)",
                       TEE_NODES, TEE_PIPES, anchor_t, "anchored_hub", "node:T", tee=TEE,
                       terminals={("pipe:C", "node:T"): COMPENSATED},
                       notes=["a value a producer that drops one junction cap of three would show; pressure-contract.md "
                              "records that a compensated terminal at a node shared by several regions is refused by name",
                              "R-4 sec. 00 C-1's shared-terminal request: R_T = -20,250 pi y N"]))
    c.append(make_case("T3-FREE", "case", "collinear bore step at an anchored node, far ends free and capped (C-3: reaction 0)",
                       STEP_NODES, STEP_PIPES, {"node:S": "anchor"}, "anchored_hub", "node:S"))
    c.append(make_case("T3-HELD", "case", "collinear bore step, the step node and both far ends anchored",
                       STEP_NODES, STEP_PIPES, {"node:S": "anchor", "node:P": "anchor", "node:Q": "anchor"},
                       "all_anchored", "node:S"))
    c.append(make_case("T3-COMP", "case",
                       "collinear bore step at an anchored node, far ends free with far terminals compensated: "
                       "the anchor carries the whole step load (R-4 T-3's stated reaction)",
                       STEP_NODES, STEP_PIPES, {"node:S": "anchor"}, "anchored_hub", "node:S",
                       terminals={("pipe:1", "node:P"): COMPENSATED, ("pipe:2", "node:Q"): COMPENSATED}))
    c.append(make_case("T4", "case", "free junction, far ends A, B and C anchored (R-4 T-4)",
                       TEE_NODES, TEE_PIPES, far, "free_hub_t4", "node:T", tee=TEE))
    c.append(make_case("T4-CMCA", "case", "T4 with mill tolerance 0.5 mm and corrosion allowance 1 mm on every pipe",
                       TEE_NODES, TEE_PIPES, far, "free_hub_t4", "node:T", c_m=CM, c_a=CA, tee=TEE))
    loads = [{"node": "node:C", "force": F_C, "moment": vec(0, 0, 0)}]
    c.append(make_case("T5", "case", "bending of both tee ends: A and B anchored, C free and capped, force (2000, 0, -1000) N at C",
                       TEE_NODES, TEE_PIPES, {"node:A": "anchor", "node:B": "anchor"}, "free_hub_t5", "node:T",
                       loads=loads, tee=TEE))
    c.append(make_case("T5-CA", "case", "T5 with corrosion allowance: the tee rows differ by basis",
                       TEE_NODES, TEE_PIPES, {"node:A": "anchor", "node:B": "anchor"}, "free_hub_t5", "node:T",
                       c_a=CA, loads=loads, tee=TEE))
    return c


def member_props(m, nodes):
    s = m["section"]
    length, ex, ey, ez = frame(nodes[m["from"]], nodes[m["to"]], Y_REF)
    nom = s["nominal"]
    return {"L": length, "ex": ex, "ey": ey, "ez": ez, "EA": nom["A"] * E, "EI": nom["I"] * E, "GJ": nom["J"] * G,
            "Ai": s["bore"]["Ai"]}


def region_of(case, pid):
    return next(r for r in case["regions"] if r["pipe"] == pid)


def poisson_held(case, m):
    """Held axial force of the Poisson pair: EA eps = N_w - 2 nu p A_i, so at eps = 0, N_w = 2 nu p A_i."""
    r = region_of(case, m["id"])
    return m["section"]["bore"]["Ai"] * (2 * NU * r["pressure"])


def node_loads(case):
    """Applied nodal loads: terminal caps (p A_i along -s, s from the node into the member) and mechanical loads."""
    loads = {n: (vec(0, 0, 0), vec(0, 0, 0)) for n in case["nodes"]}
    for r in case["regions"]:
        m = next(x for x in case["members"] if x["id"] == r["pipe"])
        props = member_props(m, case["nodes"])
        for node, closure in r["terminals"].items():
            if closure != TRANSFER:
                continue
            s = props["ex"] if node == m["from"] else mul(-1, props["ex"])
            f, mo = loads[node]
            loads[node] = (add(f, mul(-(r["pressure"] * props["Ai"]), s)), mo)
    for ld in case["loads"]:
        f, mo = loads[ld["node"]]
        loads[ld["node"]] = (add(f, ld["force"]), add(mo, ld["moment"]))
    return loads


# ----------------------------------------------------------------------------- route 1: direct stiffness
def local_stiffness(L, EA, EI, GJ):
    """12 x 12 Euler-Bernoulli member stiffness in member axes, DOFs (u v w rx ry rz) at i then j."""
    k = [[P() for _ in range(12)] for _ in range(12)]

    def put(a, b, v):
        k[a][b] = k[a][b] + v
        if a != b:
            k[b][a] = k[b][a] + v
    put(0, 0, EA / L); put(0, 6, -EA / L); put(6, 6, EA / L)
    put(3, 3, GJ / L); put(3, 9, -GJ / L); put(9, 9, GJ / L)
    a, b, c, d = EI * 12 / L ** 3, EI * 6 / L ** 2, EI * 4 / L, EI * 2 / L
    # x-y plane: v, rz
    put(1, 1, a); put(1, 5, b); put(1, 7, -a); put(1, 11, b)
    put(5, 5, c); put(5, 7, -b); put(5, 11, d)
    put(7, 7, a); put(7, 11, -b); put(11, 11, c)
    # x-z plane: w, ry
    put(2, 2, a); put(2, 4, -b); put(2, 8, -a); put(2, 10, -b)
    put(4, 4, c); put(4, 8, b); put(4, 10, d)
    put(8, 8, a); put(8, 10, b); put(10, 10, c)
    return k


def matvec(A, x):
    return [sum((a * b for a, b in zip(row, x)), P()) for row in A]


def solve_rational(K, rhs):
    """Gauss-Jordan on a rational matrix with P right-hand sides (solved per power of pi)."""
    n = len(K)
    powers = sorted({k for v in rhs for k in v.t})
    M = [[F(x) for x in row] + [v.coeff(k) for k in powers] for row, v in zip(K, rhs)]
    for col in range(n):
        piv = next(r for r in range(col, n) if M[r][col] != 0)
        M[col], M[piv] = M[piv], M[col]
        pv = M[col][col]
        M[col] = [x / pv for x in M[col]]
        for r in range(n):
            if r != col and M[r][col] != 0:
                fac = M[r][col]
                M[r] = [x - fac * y for x, y in zip(M[r], M[col])]
    return [P({k: M[i][n + j] for j, k in enumerate(powers)}) for i in range(n)]


def route_stiffness(case):
    nodes = list(case["nodes"])
    idx = {n: i for i, n in enumerate(nodes)}
    ndof = 6 * len(nodes)
    K = [[F(0)] * ndof for _ in range(ndof)]  # coefficients of pi (every stiffness term is q * pi)
    f = [P() for _ in range(ndof)]
    mdata = {}
    for m in case["members"]:
        pr = member_props(m, case["nodes"])
        kl = local_stiffness(pr["L"], pr["EA"], pr["EI"], pr["GJ"])
        R = [pr["ex"], pr["ey"], pr["ez"]]
        T = [[P() for _ in range(12)] for _ in range(12)]
        for blk in range(4):
            for a in range(3):
                for b in range(3):
                    T[3 * blk + a][3 * blk + b] = R[a][b]
        kT = [[sum((kl[a][c] * T[c][b] for c in range(12)), P()) for b in range(12)] for a in range(12)]
        kg = [[sum((T[c][a] * kT[c][b] for c in range(12)), P()) for b in range(12)] for a in range(12)]
        dofs = [6 * idx[m["from"]] + c for c in range(6)] + [6 * idx[m["to"]] + c for c in range(6)]
        for a in range(12):
            for b in range(12):
                v = kg[a][b]
                assert set(v.t) <= {1}
                K[dofs[a]][dofs[b]] += v.coeff(1)
        n0 = poisson_held(case, m)
        fixed_local = [-n0] + [P()] * 5 + [n0] + [P()] * 5  # node-on-member held end actions at u = 0
        fixed_global = [sum((T[c][a] * fixed_local[c] for c in range(12)), P()) for a in range(12)]
        for a in range(12):
            f[dofs[a]] = f[dofs[a]] - fixed_global[a]
        mdata[m["id"]] = (pr, kl, T, dofs, fixed_local)
    for n, (fo, mo) in node_loads(case).items():
        for c in range(3):
            f[6 * idx[n] + c] = f[6 * idx[n] + c] + fo[c]
            f[6 * idx[n] + 3 + c] = f[6 * idx[n] + 3 + c] + mo[c]
    fixed = {6 * idx[n] + c for n, kind in case["supports"].items() for c in range(6) if kind == "anchor"}
    free = [d for d in range(ndof) if d not in fixed]
    Kff = [[K[a][b] for b in free] for a in free]
    uf = solve_rational(Kff, [f[a] / P.pi(1) for a in free])
    u = [P() for _ in range(ndof)]
    for d, v in zip(free, uf):
        u[d] = v
    # free-DOF equilibrium, exactly
    Ku = [sum((P.pi(K[a][b]) * u[b] for b in range(ndof) if K[a][b] != 0), P()) for a in range(ndof)]
    for a in free:
        cross(f"{case['id']} route 1: free-DOF equilibrium {nodes[a // 6]} {a % 6}", Ku[a] - f[a], P())
    motions = {n: u[6 * idx[n]: 6 * idx[n] + 6] for n in nodes}
    reactions = {n: [Ku[6 * idx[n] + c] - f[6 * idx[n] + c] for c in range(6)]
                 for n, kind in case["supports"].items() if kind == "anchor"}
    ends, axial = {}, {}
    for m in case["members"]:
        pr, kl, T, dofs, fixed_local = mdata[m["id"]]
        ul = matvec(T, [u[d] for d in dofs])
        act = add(matvec(kl, ul), fixed_local)
        ends[(m["id"], m["from"])] = act[:6]
        ends[(m["id"], m["to"])] = act[6:]
        nw = act[6]
        cross(f"{case['id']} route 1: {m['id']} axial end actions balance", act[0], -nw)
        nm = pr["EA"] * (ul[6] - ul[0]) / pr["L"]
        axial[m["id"]] = {"wall": nw, "mechanical": nm}
    return {"motions": motions, "reactions": reactions, "ends": ends, "axial": axial}


# ----------------------------------------------------------------------------- route 2: closed forms
def project(v, pr):
    return [dot(v, pr["ex"]), dot(v, pr["ey"]), dot(v, pr["ez"])]


def split(v, e):
    along = mul(dot(v, e), e)
    return along, sub(v, along)


def fixed_far_actions(pr, d, th, n_held):
    """Node-on-member action at the near end of a member whose far end is fixed, for a near-end
    translation d and rotation th (textbook laws), plus the held Poisson force; and the far end by statics."""
    e, L = pr["ex"], pr["L"]
    d_ax, d_p = split(d, e)
    th_ax, th_p = split(th, e)
    force = add(add(mul(pr["EA"] / L, d_ax), mul(pr["EI"] * 12 / L ** 3, d_p)), mul(pr["EI"] * 6 / L ** 2, crossp(th_p, e)))
    force = add(force, mul(-n_held, e))
    moment = add(add(mul(pr["GJ"] / L, th_ax), mul(pr["EI"] * 4 / L, th_p)), mul(pr["EI"] * 6 / L ** 2, crossp(e, d_p)))
    far_force = mul(-1, force)
    far_moment = add(mul(-1, moment), crossp(mul(L, e), force))
    return (force, moment), (far_force, far_moment)


def route_hand(case):
    cid = case["id"]
    nodes = case["nodes"]
    loads = node_loads(case)
    hub = case["hub"]
    zero3 = vec(0, 0, 0)
    anchored = {n for n, k in case["supports"].items() if k == "anchor"}
    mem = [(m, member_props(m, nodes)) for m in case["members"]]
    for m, _ in mem:
        assert m["from"] == hub
    motions = {n: zero3 + zero3 for n in nodes}
    ends_g = {}
    axial = {}

    def free_leg(m, pr, d_hub, th_hub):
        """A leg free at its far end: statics from the far node's loads; tip motion by the cantilever laws."""
        e, L = pr["ex"], pr["L"]
        pf, qf = loads[m["to"]]
        assert all(x.is_zero() for x in qf)
        ends_g[(m["id"], m["to"])] = (pf, qf)
        ends_g[(m["id"], hub)] = (mul(-1, pf), sub(mul(-1, qf), crossp(mul(L, e), pf)))
        nw = dot(pf, e)
        nm = nw - poisson_held(case, m)
        axial[m["id"]] = {"wall": nw, "mechanical": nm}
        _, p_perp = split(pf, e)
        d = add(add(add(d_hub, crossp(th_hub, mul(L, e))), mul(nm * L / pr["EA"], e)), mul(L ** 3 / (pr["EI"] * 3), p_perp))
        th = add(th_hub, mul(L ** 2 / (pr["EI"] * 2), crossp(e, p_perp)))
        motions[m["to"]] = d + th

    cfg = case["hand"]
    if cfg == "anchored_hub":
        for m, pr in mem:
            assert m["to"] not in anchored
            free_leg(m, pr, zero3, zero3)
    elif cfg == "all_anchored":
        for m, pr in mem:
            n_held = poisson_held(case, m)
            ends_g[(m["id"], hub)] = (mul(-n_held, pr["ex"]), zero3)
            ends_g[(m["id"], m["to"])] = (mul(n_held, pr["ex"]), zero3)
            axial[m["id"]] = {"wall": n_held, "mechanical": P()}
    else:
        # the hub's net load: its own loads, what the free legs transmit, and the fixed legs' held Poisson pull
        ph, qh = loads[hub]
        fixed_legs = [(m, pr) for m, pr in mem if m["to"] in anchored]
        free_legs = [(m, pr) for m, pr in mem if m["to"] not in anchored]
        for m, pr in free_legs:
            pf, qf = loads[m["to"]]
            ph = add(ph, pf)
            qh = add(add(qh, qf), crossp(mul(pr["L"], pr["ex"]), pf))
        for m, pr in fixed_legs:
            ph = add(ph, mul(poisson_held(case, m), pr["ex"]))
        head = [(m, pr) for m, pr in fixed_legs if m["section_name"] == "header"]
        assert len(head) == 2 and add(head[0][1]["ex"], head[1][1]["ex"]) == zero3
        assert head[0][1]["ex"] in (vec(1, 0, 0), vec(-1, 0, 0)) and head[0][1]["L"] == head[1][1]["L"]
        a, hpr = head[0][1]["L"], head[0][1]
        if cfg == "free_hub_t4":
            # R-4 sec. 3.1 T-4: only the y component loads the hub; header bending (fixed-guided, theta_z = 0 by
            # symmetry) and the branch's axial stiffness resist it
            (bm, bpr), = [(m, pr) for m, pr in fixed_legs if m["section_name"] == "branch"]
            assert bpr["ex"] == vec(0, 1, 0) and ph[0].is_zero() and ph[2].is_zero() and all(x.is_zero() for x in qh)
            v = ph[1] / (hpr["EI"] * 24 / a ** 3 + bpr["EA"] / bpr["L"])
            d_hub, th_hub = vec(0, v, 0), zero3
            # the R-4 printed closed form, symbolically in the radii
            s_br, s_hd = bm["section"], head[0][0]["section"]
            r_bi = s_br["bore"]["ri"]  # the branch pressure bore (load)
            ro, ri = s_hd["ro"], s_hd["nominal"]["ri"]  # header stiffness radii
            rbo, rbn = s_br["ro"], s_br["nominal"]["ri"]  # branch stiffness radii
            p = region_of(case, bm["id"])["pressure"]
            v_r4 = -(1 - 2 * NU) * p * r_bi ** 2 / (E * (6 * (ro ** 4 - ri ** 4) / a ** 3 + (rbo ** 2 - rbn ** 2) / bpr["L"]))
            cross(f"{cid} route 2: v_T equals R-4's closed form -(1-2nu) p r_bi^2 / (E [6(r_o^4-r_i^4)/a^3 + (r_bo^2-r_bi^2)/L_b])",
                  v, P.of(v_r4))
        else:
            # symmetric header pair along x, branch free: the pair's node stiffness is diagonal
            assert cfg == "free_hub_t5" and not [m for m, _ in fixed_legs if m["section_name"] != "header"]
            k = [hpr["EA"] * 2 / a, hpr["EI"] * 24 / a ** 3, hpr["EI"] * 24 / a ** 3,
                 hpr["GJ"] * 2 / a, hpr["EI"] * 8 / a, hpr["EI"] * 8 / a]
            d_hub = [ph[c] / k[c] for c in range(3)]
            th_hub = [qh[c] / k[3 + c] for c in range(3)]
        motions[hub] = d_hub + th_hub
        for m, pr in fixed_legs:
            near, far_ = fixed_far_actions(pr, d_hub, th_hub, poisson_held(case, m))
            ends_g[(m["id"], hub)] = near
            ends_g[(m["id"], m["to"])] = far_
            nw = -dot(near[0], pr["ex"])
            axial[m["id"]] = {"wall": nw, "mechanical": nw - poisson_held(case, m)}
            cross(f"{cid} route 2: {m['id']} wall force from the near end equals the far end", nw, dot(far_[0], pr["ex"]))
        for m, pr in free_legs:
            free_leg(m, pr, d_hub, th_hub)
        # the hub's equilibrium is a check, not an input
        rf, rm = loads[hub]
        for m, _ in mem:
            fo, mo = ends_g[(m["id"], hub)]
            rf, rm = sub(rf, fo), sub(rm, mo)
        cross(f"{cid} route 2: hub equilibrium", rf + rm, [P()] * 6)
    reactions = {}
    for n in anchored:
        fo, mo = loads[n]
        rf, rm = mul(-1, fo), mul(-1, mo)
        for m, _ in mem:
            for end_node in (m["from"], m["to"]):
                if end_node == n:
                    ef, em = ends_g[(m["id"], n)]
                    rf, rm = add(rf, ef), add(rm, em)
        reactions[n] = rf + rm
    ends = {}
    for m, pr in mem:
        for n in (m["from"], m["to"]):
            fo, mo = ends_g[(m["id"], n)]
            ends[(m["id"], n)] = project(fo, pr) + project(mo, pr)
    return {"motions": motions, "reactions": reactions, "ends": ends, "axial": axial}


# ----------------------------------------------------------------------------- rows
U_COMP = ("ux", "uy", "uz", "rx", "ry", "rz")
R_COMP = ("Fx", "Fy", "Fz", "Mx", "My", "Mz")


def tee_rows_sq(case, sol):
    """Exact squares of i hypot(My, Mz) / Z_basis at the tee's referenced member ends."""
    out = []
    tee = case["tee"]
    if not tee:
        return out
    for role, pid, i in (("header", tee["header_pipe_ref"], tee["i_header"]), ("branch", tee["branch_pipe_ref"], tee["i_branch"])):
        act = sol["ends"][(pid, tee["node"])]
        m2 = act[4] * act[4] + act[5] * act[5]
        m = next(x for x in case["members"] if x["id"] == pid)
        for b in BASES:
            z = m["section"][b]["Z"]
            out.append(((role, pid, b), m2 * (i * i) / (z * z), m2))
    return out


def solution_values(case, sol):
    """Every published value of one route as {key: P}, keys in publication order."""
    vals = {}
    for n in case["nodes"]:
        for c, comp in enumerate(U_COMP):
            vals[("nodal_motion", n, None, comp, None)] = sol["motions"][n][c]
    for n in case["nodes"]:
        if n in sol["reactions"]:
            for c, comp in enumerate(R_COMP):
                vals[("support_reaction", n, None, comp, None)] = sol["reactions"][n][c]
    for m in case["members"]:
        for n in (m["from"], m["to"]):
            for c, comp in enumerate(R_COMP):
                vals[("member_end_action", m["id"], n, comp, None)] = sol["ends"][(m["id"], n)][c]
    for m in case["members"]:
        ax = sol["axial"][m["id"]]
        ai = m["section"]["bore"]["Ai"] * region_of(case, m["id"])["pressure"]
        vals[("wall_axial_force", m["id"], None, "N_w", None)] = ax["wall"]
        vals[("effective_axial_force", m["id"], None, "N_el", None)] = ax["wall"] - ai
        vals[("mechanical_axial_force", m["id"], None, "N_m", None)] = ax["mechanical"]
        for b in BASES:
            vals[("axial_membrane_stress", m["id"], None, "sigma_m", b)] = ax["wall"] / m["section"][b]["A"]
    for (role, pid, b), sq, _ in tee_rows_sq(case, sol):
        vals[("tee_intensified_bending_stress_squared", pid, case["tee"]["node"], role, b)] = sq
    return vals


UNIT = {"nodal_motion": {"u": "m", "r": "rad"}, "support_reaction": {"F": "N", "M": "N*m"},
        "member_end_action": {"F": "N", "M": "N*m"}}
FAMILY = {"u": "translation", "r": "rotation", "F": "force", "M": "moment"}


def rows_for(case, sol):
    rows = []
    for (q, ent, loc, comp, basis), v in solution_values(case, sol).items():
        if q == "tee_intensified_bending_stress_squared":
            continue
        row = {"quantity": q, "entity": ent}
        if loc:
            row["location"] = loc
        if q == "member_end_action":
            row["frame"] = "member"
        row["component"] = comp
        if basis:
            row["basis"] = BASIS_NAME[basis]
        if q in UNIT:
            row["unit"] = UNIT[q][comp[0]]
            row["family"] = FAMILY[comp[0]]
        elif q == "axial_membrane_stress":
            row["unit"], row["family"] = "Pa", "stress"
        else:
            row["unit"], row["family"] = "N", "force"
        row.update(record(v))
        rows.append(row)
    for (role, pid, b), sq, m2 in tee_rows_sq(case, sol):
        tee = case["tee"]
        row = {"quantity": "tee_intensified_bending_stress", "entity": pid, "location": tee["node"], "component": role,
               "basis": BASIS_NAME[b], "sif": str(tee["i_header"] if role == "header" else tee["i_branch"]),
               "unit": "Pa", "family": "stress"}
        row.update(sqrt_record(sq))
        row["bending_moment_magnitude_N_m"] = sqrt_record(m2)
        rows.append(row)
    return rows


def zero_scales(case, rows):
    fam = {}
    for r in rows:
        fam[r["family"]] = max(fam.get(r["family"], 0.0), abs(r["value"]))
    cap = max(binary64(m["section"]["bore"]["Ai"] * PRESSURE) for m in case["members"])
    elong = max(binary64(m["section"]["bore"]["Ai"] * PRESSURE * (1 - 2 * NU) * member_props(m, case["nodes"])["L"]
                         / (m["section"]["nominal"]["A"] * E)) for m in case["members"])
    fallback = {"force": cap, "moment": cap * 1.0, "stress": float(PRESSURE), "translation": elong, "rotation": elong / 1.0}
    out = {}
    for k in ("translation", "rotation", "force", "moment", "stress"):
        v = fam.get(k, 0.0)
        out[k] = {"value": v if v > 0 else fallback[k], "source": "largest_in_case" if v > 0 else "fallback"}
    return out


# ----------------------------------------------------------------------------- inputs as published
def num(q):
    return record(q)


def inputs_doc(case):
    secs = {}
    for m in case["members"]:
        s = m["section"]
        def basis_doc(b):
            x = s[b]
            return {"wall_m": num(x["wall"]), "ri_m": num(x["ri"]), "A_m2": record(x["A"]), "I_m4": record(x["I"]),
                    "J_m4": record(x["J"]), "Z_m3": record(x["Z"])}
        secs[m["id"]] = {"section_name": m["section_name"], "outside_diameter_m": num(s["D"]), "ro_m": num(s["ro"]),
                         "wall_thickness_m": num(s["t"]), "mill_tolerance_m": num(s["c_m"]),
                         "corrosion_allowance_m": num(s["c_a"]), "nominal_section": basis_doc("nominal"),
                         "reduced_section": basis_doc("reduced"),
                         "pressure_bore": {"ri_m": num(s["bore"]["ri"]), "Ai_m2": record(s["bore"]["Ai"])}}
    doc = {
        "nodes": {n: {"exact": [x.text() for x in v], "value": [binary64(x) for x in v]} for n, v in case["nodes"].items()},
        "pipes": [{"id": m["id"], "from": m["from"], "to": m["to"], "y_reference": [x.text() for x in Y_REF],
                   "beam_theory": "euler_bernoulli"} for m in case["members"]],
        "pipe_sections": secs,
        "supports": [{"node": n, "kind": k, "dofs": list(U_COMP)} for n, k in case["supports"].items()],
        "nodal_loads": [{"node": ld["node"], "force_N": {"exact": [x.text() for x in ld["force"]], "value": [binary64(x) for x in ld["force"]]},
                         "moment_N_m": {"exact": [x.text() for x in ld["moment"]], "value": [binary64(x) for x in ld["moment"]]}}
                        for ld in case["loads"]],
        "pressure_regions": [{"id": r["id"], "pipes": [r["pipe"]], "pressure_Pa": num(r["pressure"]),
                              "terminals": [{"node": n, "closure_transfer": c} for n, c in r["terminals"].items()]}
                             for r in case["regions"]],
        "tee": None if not case["tee"] else {
            "node": case["tee"]["node"], "header_pipe_ref": case["tee"]["header_pipe_ref"],
            "branch_pipe_ref": case["tee"]["branch_pipe_ref"], "i_header": num(case["tee"]["i_header"]),
            "i_branch": num(case["tee"]["i_branch"]),
            "rows_published_on": [{"role": "header", "pipe": case["tee"]["header_pipe_ref"], "end": case["tee"]["node"]},
                                  {"role": "branch", "pipe": case["tee"]["branch_pipe_ref"], "end": case["tee"]["node"]}]},
    }
    return doc


def junction_resultant(case, node):
    """-sum p A_i,k s_k over the transferring terminals at a node (the fluid force on the fitting)."""
    total = vec(0, 0, 0)
    for r in case["regions"]:
        m = next(x for x in case["members"] if x["id"] == r["pipe"])
        if node in r["terminals"] and r["terminals"][node] == TRANSFER and node in (m["from"], m["to"]):
            pr = member_props(m, case["nodes"])
            s = pr["ex"] if node == m["from"] else mul(-1, pr["ex"])
            total = add(total, mul(-(r["pressure"] * pr["Ai"]), s))
    return total


# ----------------------------------------------------------------------------- build
def solve_case(case):
    cid = case["id"]
    s1, s2 = route_stiffness(case), route_hand(case)
    v1, v2 = solution_values(case, s1), solution_values(case, s2)
    cross(f"{cid}: both routes publish the same value set", list(v1), list(v2))
    disagree = [k for k in v1 if k in v2 and not v1[k] == v2[k]]
    for k in disagree:
        print(f"DISAGREES {cid} {k}: {v1[k].text()} != {v2[k].text()}")
    cross(f"{cid}: every value of route 1 equals route 2 ({len(v1)} values)", disagree, [])
    # route 1: N_w = N_m + 2 nu p A_i on every member
    for m in case["members"]:
        ax = s1["axial"][m["id"]]
        cross(f"{cid} route 1: {m['id']} N_w = N_m + 2 nu p A_i", ax["wall"], ax["mechanical"] + poisson_held(case, m))
    # global equilibrium of route 1: reactions + applied loads (caps and mechanical) = 0, moments about the origin
    loads = node_loads(case)
    tf, tm = vec(0, 0, 0), vec(0, 0, 0)
    for n, (fo, mo) in loads.items():
        fo2, mo2 = fo, mo
        if n in s1["reactions"]:
            fo2, mo2 = add(fo, s1["reactions"][n][:3]), add(mo, s1["reactions"][n][3:])
        tf = add(tf, fo2)
        tm = add(add(tm, mo2), crossp(case["nodes"][n], fo2))
    cross(f"{cid} route 1: global equilibrium of reactions and applied loads", tf + tm, [P()] * 6)
    return s1, v1


def build_cases():
    out = []
    solved = {}
    for case in build_case_list():
        s1, v1 = solve_case(case)
        solved[case["id"]] = (case, s1, v1)
        rows = rows_for(case, s1)
        out.append({"id": case["id"], "status": case["status"], "purpose": case["purpose"], "notes": case["notes"],
                    "inputs": inputs_doc(case), "rows": rows, "zero_scale": zero_scales(case, rows), "key_values": {}})
    kv = {c["id"]: c["key_values"] for c in out}
    byid = {c["id"]: c for c in out}

    def val(cid, q, ent, comp, loc=None, basis=None):
        return solved[cid][2][(q, ent, loc, comp, basis)]

    def put(cid, name, v):
        kv[cid][name] = record(v)

    # --- T1 / R-4 T-1
    t1_ux_a, t1_ux_b, t1_uy_c = (val("T1", "nodal_motion", "node:A", "ux"), val("T1", "nodal_motion", "node:B", "ux"),
                                 val("T1", "nodal_motion", "node:C", "uy"))
    cross("T1 = R-4 T-1: A moves -81/237500 m in x", t1_ux_a, P.of(F(-81, 237500)))
    cross("T1 = R-4 T-1: B moves +81/237500 m in x", t1_ux_b, P.of(F(81, 237500)))
    cross("T1 = R-4 T-1: C moves +243/950000 m in y", t1_uy_c, P.of(F(243, 950000)))
    cross("T1 = R-4 T-1: printed 0.341053 mm", f"{float(F(81, 237500)) * 1000:.6f}", "0.341053")
    cross("T1 = R-4 T-1: printed 0.255789 mm", f"{float(F(243, 950000)) * 1000:.6f}", "0.255789")
    others = [v for (q, ent, loc, comp, b), v in solved["T1"][2].items() if q == "nodal_motion"
              and (ent, comp) not in (("node:A", "ux"), ("node:B", "ux"), ("node:C", "uy"))]
    cross("T1 = R-4 T-1: every other motion component is 0", all(v.is_zero() for v in others), True)
    cross("T1 = R-4 T-1: anchor reaction exactly 0 in all six components", solved["T1"][1]["reactions"]["node:T"], [P()] * 6)
    for pid, w in (("pipe:A", 81000), ("pipe:B", 81000), ("pipe:C", 20250)):
        cross(f"T1 = R-4 T-1: {pid} wall force {w} pi N", val("T1", "wall_axial_force", pid, "N_w"), P.pi(w))
        for b in BASES:
            cross(f"T1 = R-4 T-1: {pid} axial wall stress 810000000/19 Pa ({b})",
                  val("T1", "axial_membrane_stress", pid, "sigma_m", basis=b), P.of(F(810000000, 19)))
    cross("T1: the closed-tee junction resultant at T is -p A_i,b y (header caps cancel)",
          junction_resultant(solved["T1"][0], "node:T"), vec(0, P.pi(-20250), 0))
    put("T1", "ux_A_m", t1_ux_a); put("T1", "ux_B_m", t1_ux_b); put("T1", "uy_C_m", t1_uy_c)
    put("T1", "Fy_T_N", val("T1", "support_reaction", "node:T", "Fy"))
    put("T1", "N_w_pipe_C_N", val("T1", "wall_axial_force", "pipe:C", "N_w"))
    # --- T1-CA: bases and bore differ; free elongation uses the nominal A_s and the reduced-bore A_i
    c = solved["T1-CA"][0]
    for m in c["members"]:
        s = m["section"]
        cross(f"T1-CA: {m['id']} bases differ", s["nominal"]["Z"] == s["reduced"]["Z"], False)
        L = member_props(m, c["nodes"])["L"]
        free = s["bore"]["Ai"] * PRESSURE * (1 - 2 * NU) * L / (s["nominal"]["A"] * E)
        comp = "ux" if m["id"] != "pipe:C" else "uy"
        sign = -1 if m["id"] == "pipe:A" else 1
        cross(f"T1-CA: {m['id']} far end moves L (1-2nu) p A_i,r/(E A_n)", val("T1-CA", "nodal_motion", m["to"], comp), free * sign)
    cross("T1-CA: anchor reaction exactly 0", solved["T1-CA"][1]["reactions"]["node:T"], [P()] * 6)
    put("T1-CA", "ux_A_m", val("T1-CA", "nodal_motion", "node:A", "ux"))
    put("T1-CA", "uy_C_m", val("T1-CA", "nodal_motion", "node:C", "uy"))
    put("T1-CA", "sigma_m_pipe_A_nominal_Pa", val("T1-CA", "axial_membrane_stress", "pipe:A", "sigma_m", basis="nominal"))
    put("T1-CA", "sigma_m_pipe_A_reduced_Pa", val("T1-CA", "axial_membrane_stress", "pipe:A", "sigma_m", basis="reduced"))
    # --- T2 / R-4 T-2
    cross("T2 = R-4 T-2: anchor reaction +20250 pi N along +y", solved["T2"][1]["reactions"]["node:T"],
          [P(), P.pi(20250), P(), P(), P(), P()])
    cross("T2 = R-4 T-2: C moves -729/1900000 m", val("T2", "nodal_motion", "node:C", "uy"), P.of(F(-729, 1900000)))
    cross("T2 = R-4 T-2: A and B as in T1", (val("T2", "nodal_motion", "node:A", "ux"), val("T2", "nodal_motion", "node:B", "ux")),
          (t1_ux_a, t1_ux_b))
    cross("T2 = R-4 T-2: branch wall force 0", val("T2", "wall_axial_force", "pipe:C", "N_w"), P())
    put("T2", "Fy_T_N", val("T2", "support_reaction", "node:T", "Fy"))
    put("T2", "uy_C_m", val("T2", "nodal_motion", "node:C", "uy"))
    # --- T2-CTRL / C-1
    cross("T2-CTRL = R-4 C-1: R_T = -20250 pi y N", solved["T2-CTRL"][1]["reactions"]["node:T"],
          [P(), P.pi(-20250), P(), P(), P(), P()])
    cross("T2-CTRL = R-4 T-1: |R_T| printed 63,617 N", f"{binary64(P.pi(20250)):,.0f}", "63,617")
    cross("T2-CTRL: motions equal T1's", [v for k, v in solved["T2-CTRL"][2].items() if k[0] == "nodal_motion"],
          [v for k, v in solved["T1"][2].items() if k[0] == "nodal_motion"])
    put("T2-CTRL", "Fy_T_N", val("T2-CTRL", "support_reaction", "node:T", "Fy"))
    # --- T3: C-3 and T-3
    c = solved["T3-FREE"][0]
    a1 = c["members"][0]["section"]["bore"]["Ai"]
    a2 = c["members"][1]["section"]["bore"]["Ai"]
    dA = (a1 - a2) * PRESSURE
    cross("T3: the bore-step fitting load is p (A_i,1 - A_i,2) x (C-3)", junction_resultant(c, "node:S"), vec(dA, 0, 0))
    cross("T3-FREE = C-3: anchor reaction exactly 0 with free capped far ends", solved["T3-FREE"][1]["reactions"]["node:S"], [P()] * 6)
    cross("T3-HELD: step anchor reaction -(1-2nu) p (A_i,1 - A_i,2) x", solved["T3-HELD"][1]["reactions"]["node:S"],
          [-(dA * (1 - 2 * NU))] + [P()] * 5)
    cross("T3-COMP = R-4 T-3: step anchor reaction -p (A_i,1 - A_i,2) x", solved["T3-COMP"][1]["reactions"]["node:S"],
          [-dA] + [P()] * 5)
    for cid in ("T3-FREE", "T3-HELD", "T3-COMP"):
        put(cid, "Fx_S_N", val(cid, "support_reaction", "node:S", "Fx"))
    put("T3-HELD", "Fx_P_N", val("T3-HELD", "support_reaction", "node:P", "Fx"))
    put("T3-HELD", "Fx_Q_N", val("T3-HELD", "support_reaction", "node:Q", "Fx"))
    put("T3-FREE", "ux_P_m", val("T3-FREE", "nodal_motion", "node:P", "ux"))
    put("T3-COMP", "ux_P_m", val("T3-COMP", "nodal_motion", "node:P", "ux"))
    # --- T4 / R-4 T-4
    vt = val("T4", "nodal_motion", "node:T", "uy")
    cross("T4 = R-4 T-4: v_T = -1944/7754755 m", vt, P.of(F(-1944, 7754755)))
    cross("T4 = R-4 T-4: printed -0.250685 mm", f"{float(F(-1944, 7754755)) * 1000:.6f}", "-0.250685")
    cross("T4 = R-4 T-4: theta_z(T) = 0", val("T4", "nodal_motion", "node:T", "rz"), P())
    cross("T4 = R-4 T-4: header axial displacement at T = 0", val("T4", "nodal_motion", "node:T", "ux"), P())
    cross("T4 = R-4 T-4: branch row 0", [r["value"] for r in byid["T4"]["rows"] if r["quantity"] == "tee_intensified_bending_stress"
                                         and r["component"] == "branch"], [0.0, 0.0])
    cross("T4: header row nonzero", all(r["value"] > 0 for r in byid["T4"]["rows"] if r["quantity"] == "tee_intensified_bending_stress"
                                        and r["component"] == "header"), True)
    for cid in ("T4", "T4-CMCA"):
        put(cid, "uy_T_m", val(cid, "nodal_motion", "node:T", "uy"))
        put(cid, "N_w_pipe_C_N", val(cid, "wall_axial_force", "pipe:C", "N_w"))
        put(cid, "My_pipe_A_at_T_member_axes_N_m", val(cid, "member_end_action", "pipe:A", "My", "node:T"))
    vt2 = val("T4-CMCA", "nodal_motion", "node:T", "uy")
    sb = solved["T4-CMCA"][0]["members"][2]["section"]
    cross("T4-CMCA: v_T scales with the branch pressure bore only (stiffness stays nominal): v_T * (r_i,r / r_i,n)^2",
          vt2, vt * (sb["bore"]["ri"] / sb["nominal"]["ri"]) ** 2)
    # --- T5: both tee rows bend in both member planes
    for cid in ("T5", "T5-CA"):
        for pid in ("pipe:A", "pipe:C"):
            for comp in ("My", "Mz"):
                cross(f"{cid}: {pid} end at T has nonzero {comp}", val(cid, "member_end_action", pid, comp, "node:T").is_zero(), False)
        for comp in U_COMP:
            put(cid, f"{comp}_T", val(cid, "nodal_motion", "node:T", comp))
    hdr = [r for r in byid["T5"]["rows"] if r["quantity"] == "tee_intensified_bending_stress"]
    cross("T5: header |M| at T = 1000 sqrt(10) N m (half the hub load by symmetry)",
          [r["bending_moment_magnitude_N_m"]["exact_square"] for r in hdr if r["component"] == "header"], ["10000000", "10000000"])
    cross("T5: branch |M| at T = |3 y x F_C| = 3000 sqrt(5) N m",
          [r["bending_moment_magnitude_N_m"]["exact_square"] for r in hdr if r["component"] == "branch"], ["45000000", "45000000"])
    for o in out:
        case = solved[o["id"]][0]
        jr = junction_resultant(case, case["hub"])
        for comp, x in zip("xyz", jr):
            kv[o["id"]][f"junction_resultant_{comp}_at_{case['hub'].split(':')[1]}_N"] = record(x)
        for r in o["rows"]:
            if r["quantity"] == "tee_intensified_bending_stress":
                kv[o["id"]][f"tee_{r['component']}_{r['basis']}_Pa"] = {k: r[k] for k in ("exact", "decimal", "value")}
    return out


def document(cases):
    return {
        "document": "swbpipe.rt_a3_tee_references",
        "status": "candidate frozen references; derived by two exact formulations that agree, not yet independently refuted",
        "units": {"length": "m", "force": "N", "moment": "N*m", "translation": "m", "rotation": "rad", "stress": "Pa",
                  "pressure": "Pa", "area": "m^2", "second_moment": "m^4", "section_modulus": "m^3"},
        "theory": "small-displacement linear-elastic space frame, Euler-Bernoulli straight members, homogeneous isotropic; "
                  "exact pressure route: uniform internal differential p per region, EA eps = N_w - 2 nu p A_i on each "
                  "member of a pressurized region, a cap -p A_i s at each transfers_to_wall terminal (s from the node into "
                  "the member), no follower pressure, no Bourdon rotation, no pressure stiffening",
        "material": {"elastic_modulus_Pa": str(E), "poisson_ratio": str(NU), "shear_modulus_Pa": str(G)},
        "section_bases": "D-5: stiffness (A, I, J) and A_s in the free elongation on the nominal wall t; reduced wall "
                         "t_r = (t - c_m) - c_a; r_o = D/2 for both; pressure bore r_i,r = D/2 - t_r, A_i = pi r_i,r^2",
        "conventions": {
            "nodal_motion": "global translation and right-handed rotation",
            "support_reaction": "global action of the support on the structure at its node (moments about that node)",
            "member_end_action": "action of the node on the member at that end, in member axes (x from `from` to `to`, "
                                 "y the y_reference less its x part, z = x cross y); the axial component is -N_w at the "
                                 "`from` end and +N_w at the `to` end (tension positive), i.e. the physical wall force",
            "wall_axial_force": "N_w, the wall's axial force, tension positive",
            "effective_axial_force": "N_el = N_w - p A_i (the effective force; N_el = N_w outside a pressure region)",
            "mechanical_axial_force": "N_m = E A_n eps, so N_w = N_m + 2 nu p A_i",
            "axial_membrane_stress": "N_w / A_basis",
            "tee_intensified_bending_stress": "i hypot(My, Mz) / Z_basis at the referenced member end at the tee node "
                                              "(member-axis bending moments; torsion and axial force excluded); "
                                              "published through its exact square",
            "exact_text": "a sum of terms q, q*pi, q*pi^k, q/pi or q/pi^k with q rational, joined by ' + '; "
                     "a square root is 'sqrt(<exact>)' and carries exact_square",
            "decimal": "40 significant digits; value is the nearest binary64",
        },
        "criterion_proposed": "|observed - expected| <= 1e-9 * |expected|; an exactly zero expectation uses 1e-9 * "
                              "zero_scale of its family (largest nonzero magnitude of that family in the case, or the "
                              "stated fallback); the product binding chooses its own criterion",
        "crosscheck": list(CROSS),
        "cases": cases,
    }


def dumps(obj):
    def inline(o):
        return json.dumps(o, ensure_ascii=False, separators=(", ", ": "))

    def emit(o, level):
        pad = " " * level
        if isinstance(o, dict) and ("exact" in o or "check" in o or "quantity" in o):
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
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--write", action="store_true")
    mode.add_argument("--check", action="store_true")
    args = parser.parse_args()
    files, cases = build()
    bad = [c for c in CROSS if not c["agrees"]]
    print(f"{len(cases)} cases, {sum(len(c['rows']) for c in cases)} rows")
    print(f"cross-checks: {len(CROSS)} recorded, {len(bad)} disagreeing")
    for c in cases:
        print(c["id"], ", ".join(f"{k} {v['value']:.12g}" for k, v in c["key_values"].items()))
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
