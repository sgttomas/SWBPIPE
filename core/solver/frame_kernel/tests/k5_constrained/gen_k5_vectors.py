#!/usr/bin/env python3
"""K5 (W4) test vectors for `rigid_body::assess_constrained_bodies`.

Standard library only (fractions, hashlib, random, struct). Every input is a
binary64 value, written as its 16-hex-digit bits. Every expectation comes from
exact rational arithmetic on those values:

- the rank and null space of the UNREDUCED stacked map (six unknowns per
  sub-body, six rows per tie, one row per ground), by fraction-free integer
  elimination. This oracle does not use K5's tie reduction;
- for a one-dimensional null space, the canonical representative the function
  must publish: p = [u(node 0), θ] of the null motion, j its first nonzero
  component, r = k·p/p_j for the smallest k in 1..=64 such that r and every
  node motion are exactly representable in binary64.

The case filter (B1 only) keeps cases outside the τ_B band: a full-rank case
needs det(G)/m^6 >= 1e-12, a rank-5 case e5(G)/m^5 >= 1e-12, where G is the
exact Gram matrix of the function's normalized reduced rows (m rows). This is
a selection filter, not an oracle; it mirrors the reduction only to pick cases.
Separately, every B1 case asserts that the reduced rows' nullity equals the
unreduced map's (a cross-check of the reduction's derivation).

RV14-4 (the addendum): `subnormal.txt` carries RV14's tiny-coordinate corpus
(the reviewer's `tiny_case`, seed 1403, ported below; the first
N_SUBNORMAL_SAMPLE of its 1,500 cases) after RV14's minimal case
`rv14_h_tiny_free_x`, with expectations from the same exact oracle, plus the
function's characteristic length L (a power of two from the exact virtual
positions rounded to binary64): a one-dimensional null space whose canonical
witness has a translation t_i with t_i/L not exactly representable is `P`.

Usage:
  gen_k5_vectors.py [--out DIR]          write b1_sample.txt, b1_summary.txt,
                                         cases.txt, subnormal.txt,
                                         subnormal_summary.txt and SHA256SUMS
                                         into DIR
  gen_k5_vectors.py --check [--out DIR]  regenerate and compare byte for byte
  gen_k5_vectors.py --full PATH          also write all B1 records to PATH
  gen_k5_vectors.py --full-subnormal PATH  also write all 1,501 subnormal records

Record line (space-separated fields):
  <name> <expect> n=<N> subs=<a,b;c> ties=<a-b,..|-> grounds=<g,..|->
  coords=<hex,..> motion=<hex,..|-> [rigid=<R|W>]
  ground: d<dof> | t<node>:<hx>:<hy>:<hz> | r<node>:<hx>:<hy>:<hz>
  expect: R  Restrained
          W  MechanismWitnessed with node_motion bits equal to `motion`
          M  one-dimensional null space: W with those bits, or unresolved
          U  NumericallyUnresolved
          D  nullity >= 2: W (checked exactly by the test) or unresolved
          N  not witnessed: Restrained or unresolved
          P  one-dimensional null space whose canonical [t/L, θ] is not
             exactly representable: NumericallyUnresolved, "constrained-body
             witness parameters not representable" (RV14-4)
  motion: 6 values per node [u, θ], the canonical representative.
  rigid:  the welded union's (every tie a rigid link) exact outcome.
"""
import argparse
import hashlib
import math
import os
import random
import struct
import sys
from fractions import Fraction as F

SEED = 20260928
N_B1 = 4000
N_SAMPLE = 1000
BAND = F(1, 10**12)


# ------------------------------------------------------------------ binary64

def hexf(x):
    return struct.pack(">d", float(x)).hex()


def as_binary64(q):
    """The binary64 value equal to the rational q, or None."""
    assert isinstance(q, F), q
    try:
        f = float(q)
    except OverflowError:
        return None
    if math.isinf(f) or F(f) != q:
        return None
    return f


def up(ulps, x):
    """x moved by `ulps` units in the last place (positive x)."""
    bits = struct.unpack(">q", struct.pack(">d", x))[0]
    return struct.unpack(">d", struct.pack(">q", bits + ulps))[0]


# ------------------------------------------------------------------ cases

class Case:
    def __init__(self, name, coords, subs, ties, grounds):
        self.name = name
        self.coords = [tuple(float(v) for v in c) for c in coords]
        self.subs = [list(s) for s in subs]
        self.ties = [tuple(t) for t in ties]
        self.grounds = list(grounds)
        self.expect = None
        self.klass = ""
        self.motion = None
        self.rigid = None
        self.nullity = None


def cross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def sub3(a, b):
    return (a[0] - b[0], a[1] - b[1], a[2] - b[2])


def exact_coords(case):
    return [tuple(F(v) for v in c) for c in case.coords]


# ------------------------------------------------------------------ the unreduced stacked map

def stacked_rows(case):
    """Rows of the unreduced map over (t_B, θ_B) per sub-body, with
    u(x) = t_B + θ_B × (x − x_0)."""
    xs = exact_coords(case)
    o = xs[0]
    owner = {}
    for b, nodes in enumerate(case.subs):
        for x in nodes:
            owner[x] = b
    ncol = 6 * len(case.subs)

    def u_row(x, k):
        row = [F(0)] * ncol
        b = owner[x]
        r = sub3(xs[x], o)
        # (θ × r)_k as coefficients on θ.
        m = [[0, r[2], -r[1]], [-r[2], 0, r[0]], [r[1], -r[0], 0]][k]
        row[6 * b + k] = F(1)
        for j in range(3):
            row[6 * b + 3 + j] += m[j]
        return row

    def th_row(x, k):
        row = [F(0)] * ncol
        row[6 * owner[x] + 3 + k] = F(1)
        return row

    rows = []
    for a, b in case.ties:
        for k in range(3):
            rows.append([p - q for p, q in zip(u_row(a, k), u_row(b, k))])
            rows.append([p - q for p, q in zip(th_row(a, k), th_row(b, k))])
    for g in case.grounds:
        if g[0] == "d":
            x, k = divmod(g[1], 6)
            rows.append(u_row(x, k) if k < 3 else th_row(x, k - 3))
        else:
            _, x, n = g
            n = [F(v) for v in n]
            parts = [u_row(x, k) if g[0] == "t" else th_row(x, k) for k in range(3)]
            rows.append([sum(n[k] * parts[k][i] for k in range(3)) for i in range(ncol)])
    return rows, ncol


def integer_row(row):
    den = 1
    for v in row:
        den = den * v.denominator // math.gcd(den, v.denominator)
    ints = [int(v * den) for v in row]
    g = 0
    for v in ints:
        g = math.gcd(g, v)
    return [v // g for v in ints] if g > 1 else ints


def echelon(rows, ncol):
    rows = [integer_row(r) for r in rows]
    rows = [r for r in rows if any(r)]
    pivots = []
    top = 0
    for col in range(ncol):
        best = None
        for i in range(top, len(rows)):
            v = rows[i][col]
            if v and (best is None or abs(v) < abs(rows[best][col])):
                best = i
        if best is None:
            continue
        rows[top], rows[best] = rows[best], rows[top]
        p = rows[top]
        pc = p[col]
        for i in range(top + 1, len(rows)):
            c = rows[i][col]
            if c:
                new = [pc * a - c * b for a, b in zip(rows[i], p)]
                g = 0
                for v in new:
                    g = math.gcd(g, v)
                rows[i] = [v // g for v in new] if g > 1 else new
        pivots.append(col)
        top += 1
    return rows[:top], pivots


def null_vector(ech, pivots, ncol):
    free = [c for c in range(ncol) if c not in pivots]
    assert len(free) == 1
    x = [F(0)] * ncol
    x[free[0]] = F(1)
    for i in reversed(range(len(pivots))):
        col = pivots[i]
        s = sum((F(ech[i][j]) * x[j] for j in range(col + 1, ncol) if ech[i][j]), F(0))
        x[col] = -s / F(ech[i][col])
    assert all(isinstance(v, F) for v in x)
    return x


def node_motions(case, x):
    xs = exact_coords(case)
    o = xs[0]
    owner = {}
    for b, nodes in enumerate(case.subs):
        for y in nodes:
            owner[y] = b
    out = []
    for y in range(len(xs)):
        b = owner[y]
        t = x[6 * b:6 * b + 3]
        th = x[6 * b + 3:6 * b + 6]
        w = cross(th, sub3(xs[y], o))
        out.append([t[k] + w[k] for k in range(3)] + list(th))
    return out


def canonical(motions):
    """The canonical representative's node motions (floats), or None."""
    p = motions[0]
    j = next(i for i, v in enumerate(p) if v != 0)
    for k in range(1, 65):
        s = F(k) / p[j]
        out = []
        for m in motions:
            vals = [as_binary64(s * v) for v in m]
            if any(v is None for v in vals):
                break
            out.append(vals)
        else:
            return out
    return None


def assess_oracle(case):
    """nullity, and the canonical motion (nullity 1), from the unreduced map."""
    rows, ncol = stacked_rows(case)
    ech, pivots = echelon(rows, ncol)
    nullity = ncol - len(pivots)
    motion = None
    if nullity == 1:
        motion = canonical(node_motions(case, null_vector(ech, pivots, ncol)))
    return nullity, motion


def rigid_link_nullity(case):
    """The welded union (every tie a rigid link): one sub-body, no ties."""
    welded = Case(case.name, case.coords, [list(range(len(case.coords)))], [], case.grounds)
    rows, ncol = stacked_rows(welded)
    _, pivots = echelon(rows, ncol)
    return ncol - len(pivots)


# ------------------------------------------------------------------ the reduced rows (filter only)

def reduction(case):
    """The function's tie reduction, exactly: the exact coordinates, the
    virtual positions v and their binary64 roundings, L, the canonical ties and
    which are tree ties."""
    xs = exact_coords(case)
    o = xs[0]
    bodies = sorted((sorted(s) for s in case.subs), key=lambda s: s[0])
    owner = {}
    for b, nodes in enumerate(bodies):
        for x in nodes:
            owner[x] = b
    ties = sorted((min(a, b), max(a, b)) for a, b in case.ties)
    adjacency = [[] for _ in bodies]
    for k, (a, b) in enumerate(ties):
        if owner[a] != owner[b]:
            adjacency[owner[a]].append(k)
            adjacency[owner[b]].append(k)
    offsets = [None] * len(bodies)
    offsets[0] = (F(0), F(0), F(0))
    tree = [False] * len(ties)
    queue = [0]
    head = 0
    while head < len(queue):
        cur = queue[head]
        head += 1
        for k in adjacency[cur]:
            a, b = ties[k]
            near, far = (a, b) if owner[a] == cur else (b, a)
            other = owner[far]
            if offsets[other] is not None:
                continue
            offsets[other] = tuple(offsets[cur][i] + xs[near][i] - xs[far][i] for i in range(3))
            tree[k] = True
            queue.append(other)
    assert all(s is not None for s in offsets)
    v = [tuple(xs[x][i] - o[i] + offsets[owner[x]][i] for i in range(3)) for x in range(len(xs))]
    vhat = [tuple(F(float(c)) for c in p) for p in v]
    m = max(abs(c) for p in vhat for c in p)
    length = F(1) if m == 0 else F(2) ** (math.frexp(float(m))[1] - 1)
    return xs, v, vhat, length, ties, tree


def reduced_rows(case):
    """The function's reduced rows in [t/L, θ], exactly (a selection filter)."""
    xs, v, vhat, length, ties, tree = reduction(case)
    rows = []
    for g in case.grounds:
        if g[0] == "d":
            x, k = divmod(g[1], 6)
            if k < 3:
                w = tuple(c / length for c in vhat[x])
                rows.append([[1, 0, 0, 0, w[2], -w[1]], [0, 1, 0, -w[2], 0, w[0]],
                             [0, 0, 1, w[1], -w[0], 0]][k])
            else:
                row = [0] * 6
                row[k] = 1
                rows.append(row)
        else:
            _, x, n = g
            n = tuple(F(c) for c in n)
            if g[0] == "t":
                w = tuple(c / length for c in vhat[x])
                rows.append(list(n) + list(cross(w, n)))
            else:
                rows.append([0, 0, 0] + list(n))
    for k, (a, b) in enumerate(ties):
        if tree[k]:
            continue
        c = tuple(F(float(v[a][i] - v[b][i])) for i in range(3))
        for mrow in ([0, c[2], -c[1]], [-c[2], 0, c[0]], [c[1], -c[0], 0]):
            rows.append([0, 0, 0] + mrow)
    return [[F(x) for x in r] for r in rows if any(r)]


def det(m):
    m = [row[:] for row in m]
    n = len(m)
    d = F(1)
    for c in range(n):
        p = next((i for i in range(c, n) if m[i][c] != 0), None)
        if p is None:
            return F(0)
        if p != c:
            m[c], m[p] = m[p], m[c]
            d = -d
        d *= m[c][c]
        for i in range(c + 1, n):
            f = m[i][c] / m[c][c]
            if f:
                for j in range(c, n):
                    m[i][j] -= f * m[c][j]
    return d


def gram(rows):
    g = [[F(0)] * 6 for _ in range(6)]
    for r in rows:
        nn = sum(x * x for x in r)
        for i in range(6):
            if r[i]:
                for j in range(6):
                    g[i][j] += r[i] * r[j] / nn
    return g


def outside_band(case, nullity):
    if nullity >= 2:
        return True
    rows = reduced_rows(case)
    m = len(rows)
    if m == 0:
        return False
    g = gram(rows)
    if nullity == 0:
        return det(g) / F(m) ** 6 >= BAND
    e5 = sum(det([[g[i][j] for j in range(6) if j != skip] for i in range(6) if i != skip])
             for skip in range(6))
    return e5 / F(m) ** 5 >= BAND


def reduced_nullity(case):
    rows = reduced_rows(case)
    if not rows:
        return 6
    _, pivots = echelon(rows, 6)
    return 6 - len(pivots)


# ------------------------------------------------------------------ B1 generation

def random_geometry(rng):
    s = rng.randint(1, 8)
    sizes = [rng.randint(1, 4) for _ in range(s)]
    n = sum(sizes)
    labels = list(range(n))
    rng.shuffle(labels)
    subs = []
    i = 0
    for size in sizes:
        subs.append(sorted(labels[i:i + size]))
        i += size
    scale = 2.0 ** rng.randint(-2, 2)
    coords = [tuple(rng.randint(-4, 4) * scale for _ in range(3)) for _ in range(n)]
    order = list(range(s))
    rng.shuffle(order)
    ties = []
    for i in range(1, s):
        a = order[rng.randrange(i)]
        b = order[i]
        ties.append((rng.choice(subs[a]), rng.choice(subs[b])))
    for _ in range(rng.randint(0, 10 - (s - 1))):
        a, b = rng.randrange(n), rng.randrange(n)
        if a != b:
            ties.append((a, b))
    ties = [(b, a) if rng.random() < 0.5 else (a, b) for a, b in ties]
    rng.shuffle(ties)
    return coords, subs, ties


def small_vector(rng, avoid=None):
    while True:
        v = tuple(rng.randint(-2, 2) for _ in range(3))
        if any(v) and (avoid is None or any(cross(v, avoid))):
            return v


def primitive(v):
    """The integer vector along the rational v, with gcd 1."""
    den = 1
    for c in v:
        den = den * F(c).denominator // math.gcd(den, F(c).denominator)
    ints = [int(F(c) * den) for c in v]
    g = 0
    for c in ints:
        g = math.gcd(g, c)
    return tuple(c // g for c in ints)


def random_grounds(rng, n):
    grounds = []
    for _ in range(rng.randint(3, 14)):
        r = rng.random()
        x = rng.randrange(n)
        if r < 0.5:
            grounds.append(("d", 6 * x + rng.randrange(6)))
        elif r < 0.75:
            grounds.append(("t", x, tuple(float(c) for c in small_vector(rng))))
        else:
            grounds.append(("r", x, tuple(float(c) for c in small_vector(rng))))
    return grounds


def null_case(rng, name):
    """A case built from a small-integer null motion, with a one-dimensional
    null space, or None."""
    coords, subs, ties = random_geometry(rng)
    xs = [tuple(F(v) for v in c) for c in coords]
    o = xs[0]
    bodies = sorted((sorted(s) for s in subs), key=lambda s: s[0])
    owner = {}
    for b, nodes in enumerate(bodies):
        for x in nodes:
            owner[x] = b
    cti = sorted((min(a, b), max(a, b)) for a, b in ties)
    # Any spanning tree gives the same admissible motions; this one grows
    # from the sub-body of node 0 in passes over the ties.
    offsets = {0: (F(0),) * 3}
    tree = set()
    changed = True
    while changed:
        changed = False
        for k, (a, b) in enumerate(cti):
            for near, far in ((a, b), (b, a)):
                if owner[near] in offsets and owner[far] not in offsets:
                    offsets[owner[far]] = tuple(offsets[owner[near]][i] + xs[near][i] - xs[far][i]
                                                for i in range(3))
                    tree.add(k)
                    changed = True
    v = [tuple(xs[x][i] - o[i] + offsets[owner[x]][i] for i in range(3)) for x in range(len(xs))]
    cycles = [sub3(v[a], v[b]) for k, (a, b) in enumerate(cti) if k not in tree]
    cycles = [c for c in cycles if any(c)]
    if cycles:
        base = primitive(cycles[0])
        if all(not any(cross(base, c)) for c in cycles):
            sign = rng.choice((1, -1))
            theta = tuple(sign * c for c in base)
        else:
            theta = (0, 0, 0)
    else:
        theta = small_vector(rng) if rng.random() < 0.8 else (0, 0, 0)
    t = tuple(rng.randint(-2, 2) for _ in range(3))
    if not any(t) and not any(theta):
        t = small_vector(rng)
    theta = tuple(F(c) for c in theta)
    u = [tuple(F(t[i]) + cross(theta, v[x])[i] for i in range(3)) for x in range(len(xs))]
    grounds = []
    for batch in (8, 4, 4, 4, 4):
        for _ in range(batch):
            x = rng.randrange(len(xs))
            options = [("d", 6 * x + k) for k in range(3) if u[x][k] == 0]
            options += [("d", 6 * x + 3 + k) for k in range(3) if theta[k] == 0]
            if any(u[x]):
                a = small_vector(rng, avoid=u[x])
                options.append(("t", x, tuple(float(c) for c in primitive(cross(u[x], a)))))
            else:
                options.append(("t", x, tuple(float(c) for c in small_vector(rng))))
            if any(theta):
                a = small_vector(rng, avoid=theta)
                options.append(("r", x, tuple(float(c) for c in primitive(cross(theta, a)))))
            else:
                options.append(("r", x, tuple(float(c) for c in small_vector(rng))))
            grounds.append(rng.choice(options))
        case = Case(name, coords, subs, ties, grounds)
        nullity, motion = assess_oracle(case)
        if nullity == 1:
            if motion is None or not outside_band(case, 1):
                return None
            # The one null motion is the constructed one.
            expected = canonical([list(u[x]) + list(theta) for x in range(len(xs))])
            assert expected == motion, name
            case.nullity = 1
            case.motion = motion
            case.expect = "W"
            case.klass = "null"
            return case
        assert nullity >= 1, name
    return None


def random_case(rng, name):
    coords, subs, ties = random_geometry(rng)
    case = Case(name, coords, subs, ties, random_grounds(rng, len(coords)))
    nullity, motion = assess_oracle(case)
    if not outside_band(case, nullity):
        return None
    case.nullity = nullity
    case.klass = "random"
    if nullity == 0:
        case.expect = "R"
    elif nullity == 1:
        case.expect = "M" if motion is not None else "U"
        case.motion = motion
    else:
        case.expect = "D"
    return case


def shuffle_inputs(rng, case):
    rng.shuffle(case.grounds)
    rng.shuffle(case.subs)
    for s in case.subs:
        rng.shuffle(s)
    return case


def b1_cases():
    rng = random.Random(SEED)
    out = []
    rejected = {"null": 0, "random": 0}
    while len(out) < N_B1:
        name = f"b1_{len(out):04d}"
        if len(out) % 3 == 0:
            case = null_case(rng, name)
            if case is None:
                rejected["null"] += 1
                continue
        else:
            case = random_case(rng, name)
            if case is None:
                rejected["random"] += 1
                continue
        # The reduction's nullity equals the unreduced map's (a derivation
        # cross-check; never used as an oracle).
        assert reduced_nullity(case) == case.nullity, name
        out.append(shuffle_inputs(rng, case))
    return out, rejected


# ------------------------------------------------------------------ constructed cases (B2-B5, B9)

def pins(*nodes):
    return [("d", 6 * x + k) for x in nodes for k in range(3)]


def constructed():
    cases = []

    def add(name, coords, subs, ties, grounds, expect, rigid=False):
        case = Case(name, coords, subs, ties, grounds)
        nullity, motion = assess_oracle(case)
        case.nullity = nullity
        if expect == "W":
            assert nullity == 1 and motion is not None, name
            case.motion = motion
        elif expect == "R":
            assert nullity == 0, name
        elif expect == "U":
            # Either exactly full rank inside the τ_B band (by design), or a
            # one-dimensional null space with no representable canonical motion.
            assert nullity == 0 or (nullity == 1 and motion is None), name
        elif expect == "N":
            assert nullity == 0, name
        case.expect = expect
        if rigid:
            case.rigid = "R" if rigid_link_nullity(case) == 0 else "W"
        cases.append(case)

    # B2: an NP-C-like internal mechanism. The welded union is restrained (three
    # non-collinear actual pins); the ties leave the rotation about the virtual
    # line (the x axis) free. Its stabilized companion adds RX at node 3.
    npc = [(0, 0, 0), (2, 0, 0), (2, 1, 0), (4, 1, 0)]
    add("npc_internal", npc, [[0, 1], [2, 3]], [(1, 2)], pins(0, 1, 3), "W", rigid=True)
    add("npc_companion", npc, [[0, 1], [2, 3]], [(1, 2)], pins(0, 1, 3) + [("d", 21)], "R", rigid=True)
    # B2: near-collinear virtual pins (the actual pins are not collinear):
    # exact, one ulp off (τ_B band, no exact null vector), separated.
    def nc(y3):
        return [(0, 0, 0), (1, 0, 0), (1, 3, 0), (5, y3, 0)]
    add("nc_exact", nc(3.0), [[0, 1], [2, 3]], [(1, 2)], pins(0, 2, 3), "W", rigid=True)
    add("nc_ulp", nc(up(1, 3.0)), [[0, 1], [2, 3]], [(1, 2)], pins(0, 2, 3), "U")
    add("nc_separated", nc(4.0), [[0, 1], [2, 3]], [(1, 2)], pins(0, 2, 3), "R")
    # B2: NP-C's near_collinear_rows (exact rank 2) as two rows at the one
    # node, the other four DOFs grounded: exact rank 6, in the τ_B band.
    add("npc_rows", [(0, 0, 0)], [[0]], [],
        [("d", 0), ("t", 0, (1.0, 1e-16, 0.0)), ("d", 2), ("d", 3), ("d", 4), ("d", 5)], "U")
    # B3: the rigid-link model misses npc_internal's mechanism (rigid=R), and
    # here asserts a false one: actual pins collinear on the x axis, virtual
    # pins not.
    add("b3_false_mechanism", [(0, 0, 0), (2, 0, 0), (2, 1, 0), (4, 0, 0)], [[0, 1], [2, 3]],
        [(1, 2)], pins(0, 1, 3), "R", rigid=True)
    # B4: cycles. The tree tie is (0,3) (canonical order); the cycle (1,2) has
    # offset (0,2,0) and removes the only free mode (rotation about z).
    ground_z = pins(0) + [("d", 3), ("d", 4)]
    add("b4_cycle_restrains", [(0, 0, 0), (2, 0, 0), (2, 1, 0), (0, 3, 0)], [[0, 1], [2, 3]],
        [(1, 2), (0, 3)], ground_z, "R")
    add("b4_cycle_zero", [(0, 0, 0), (2, 0, 0), (2, 1, 0), (0, 1, 0)], [[0, 1], [2, 3]],
        [(1, 2), (0, 3)], ground_z, "W")
    add("b4_inside_free", [(0, 0, 0), (3, 0, 0)], [[0, 1]], [(0, 1)], pins(0) + [("d", 4), ("d", 5)], "W")
    add("b4_inside_blocks", [(0, 0, 0), (3, 0, 0)], [[0, 1]], [(0, 1)], pins(0) + [("d", 3), ("d", 4)], "R")
    # B4 (RV14-1): RV14's band cycles T1_cycle_band_0 and _4. Sub-bodies {0,2}
    # and {1,3}, tree tie 0-1, cycle 2-3 with exact offset c = (ε, 0, 1), ε =
    # 9u and 901u (u = 2^-53). The cycle restrains the rotation about z
    # (exact nullity 0), but σ_min lies below τ_B. The violated cycle row's
    # |θ × ĉ| ≈ ε passes the 2^-20 prefilter, so only the exact tie check
    # refuses θ = e_z.
    for tag, ulps in (("0", 9), ("4", 901)):
        add(f"b4_cycle_band_rv14_{tag}", [(0, 0, 0), (1, 0, 0), (0, 1, 0), (up(-ulps, 1.0), 1, -1)],
            [[0, 2], [1, 3]], [(0, 1), (2, 3)], ground_z, "U")
    # B5: KREV-01 analogues through a tie: the rounded candidate looks null,
    # the exact check refutes it.
    add("b5_big16", [(1e16, 1e16, 0), (-1e16, -1e16, 0), (5, 5, 5), (5, 7, 5)], [[0, 1], [2, 3]],
        [(1, 2)], pins(0, 1, 3), "N")
    add("b5_big200", [(0, 0, 0), (1e200, 0, 0), (0, 0, 0), (-1e200, 1e-200, 0)], [[0, 1], [2, 3]],
        [(1, 2)], pins(0, 1, 3), "N")
    # B5: valid witnesses under origin shifts and radix units.
    for units in (0.5, 1.0, 2.0):
        for shift in (0.0, 8.0):
            x1 = (shift + 10.0, shift - 7.0, 1.0)
            x2 = (x1[0] + 3 * units, x1[1] + 4 * units, 1.0)
            add(f"b5_radix_u{units}_s{shift}", [(shift, shift, 0.0), x1, x2], [[0], [1, 2]],
                [(0, 1)], pins(0, 2), "W")
    # B5: an exact mechanism whose canonical direction is not representable.
    a = 1.0 + 2.0 ** -52
    add("b5_unrepresentable", [(0, 0, 0), (a, a, 0), (a, 2.0 ** 53, 0)], [[0, 1, 2]], [],
        pins(0, 1), "U")
    # B9: directional rows. npc_internal's pins as axis triads, and as an
    # oblique spanning triad, at every pinned node.
    def triad(x, dirs):
        return [("t", x, d) for d in dirs]
    axes = [(1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)]
    oblique = [(1.0, 1.0, 0.0), (1.0, -1.0, 0.0), (0.0, 0.0, 1.0)]
    add("b9_triad_axes", npc, [[0, 1], [2, 3]], [(1, 2)],
        sum((triad(x, axes) for x in (0, 1, 3)), []), "W")
    add("b9_triad_oblique", npc, [[0, 1], [2, 3]], [(1, 2)],
        sum((triad(x, oblique) for x in (0, 1, 3)), []), "W")
    # B9: a translation-pinned line along (1,2,2) with an axial rotational row
    # (the RF-SKEW-T-PIN-AX class: restrained) or a perpendicular one (the
    # rotation about the line stays free).
    line = [(0, 0, 0), (1, 2, 2), (2, 4, 4)]
    add("b9_line_axial_rotation", line, [[0, 1, 2]], [], pins(0, 2) + [("r", 1, (1.0, 2.0, 2.0))], "R")
    add("b9_line_perpendicular_rotation", line, [[0, 1, 2]], [],
        pins(0, 2) + [("r", 1, (2.0, -1.0, 0.0))], "W")
    # B9: a non-spanning set with a real mechanism (translation along (1,-1,0)).
    add("b9_nonspanning", [(0, 0, 0), (1, 0, 0)], [[0, 1]], [],
        [("t", 0, (1.0, 1.0, 0.0)), ("t", 0, (0.0, 0.0, 1.0)), ("d", 3), ("d", 4), ("d", 5)], "W")
    # B9: a line pinned by directional triads at a node and at a tied node
    # whose virtual position lies on the line.
    add("b9_tied_line", [(0, 0, 0), (1, 2, 2), (7, 7, 7), (8, 9, 9)], [[0, 1], [2, 3]], [(1, 2)],
        triad(0, oblique) + triad(3, axes), "W")
    # B9 (K5-M17): the exact check of a directional row. The origin is pinned
    # and two rotation rows leave only rotations about w = (1, 0, 1); node 2 at
    # (1, -2^53, 0) then moves u = w x v = (2^53, 1, -2^53) under θ = w, and the
    # translation row n = (1, 1, 1) there has exact action 1 (so the body is
    # restrained exactly), while its binary64 evaluation (2^53 + 1) - 2^53 = 0
    # cancels. A binary64 check would publish θ = w as a false witness.
    add("b9_directional_exactness", [(0, 0, 0), (1, 0, 1), (1, -(2.0 ** 53), 0)], [[0, 1, 2]], [],
        pins(0) + [("r", 0, (1.0, 0.0, -1.0)), ("r", 0, (0.0, 1.0, 0.0)), ("t", 2, (1.0, 1.0, 1.0))], "N")
    return cases


# ------------------------------------------------------------------ RV14-4: tiny coordinates

SUBNORMAL_SEED = 1403
N_SUBNORMAL = 1500
N_SUBNORMAL_SAMPLE = 300


def rv14_partition(rng, n, s):
    """RV14's `partition` (REVIEW/_run_records/k5_review/oracle/rv14_oracle.py.txt)."""
    nodes = list(range(n))
    rng.shuffle(nodes)
    cuts = sorted(rng.sample(range(1, n), s - 1)) if s > 1 else []
    parts, prev = [], 0
    for c in cuts + [n]:
        parts.append(sorted(nodes[prev:c]))
        prev = c
    return parts


def rv14_rand_dir(rng):
    """RV14's `rand_dir`."""
    while True:
        style = rng.random()
        if style < 0.4:
            d = [float(rng.randint(-3, 3)) for _ in range(3)]
        elif style < 0.7:
            d = [round(rng.uniform(-1, 1), 3) for _ in range(3)]
        else:
            d = [rng.uniform(-1, 1) for _ in range(3)]
        if any(d):
            return d


def rv14_tiny_case(rng, name):
    """RV14's `tiny_case`: 1-7 nodes, subnormal, edge and mixed coordinates."""
    n = rng.randint(1, 7)
    s = rng.randint(1, min(n, 4))
    subs = rv14_partition(rng, n, s)
    ties = []
    order = list(range(s))
    rng.shuffle(order)
    for i in range(1, s):
        ties.append((rng.choice(subs[order[rng.randrange(i)]]), rng.choice(subs[order[i]])))
    for _ in range(rng.randint(0, 2)):
        a, b = rng.randrange(n), rng.randrange(n)
        if a != b:
            ties.append((a, b))
    eta = 2.0 ** -1074
    style = rng.choice(["sub", "sub", "edge", "mix"])

    def c():
        if style == "sub":
            return rng.randint(-2 ** rng.randint(0, 22), 2 ** rng.randint(0, 22)) * eta
        if style == "edge":
            return rng.choice([1, -1]) * rng.randint(1, 8) * 2.0 ** rng.randint(-1060, -1018)
        return rng.choice([0.0, rng.randint(-8, 8) * eta, rng.randint(-8, 8) * 2.0 ** -1030])
    coords = [[c() for _ in range(3)] for _ in range(n)]
    grounds = [("d", d) for d in rng.sample(range(6 * n), rng.randint(0, min(6 * n, 12)))]
    for _ in range(rng.choice([0, 1, 2])):
        grounds.append((rng.choice("tr"), rng.randrange(n), tuple(rv14_rand_dir(rng))))
    return Case(name, coords, subs, ties, grounds)


def subnormal_expect(case):
    """The exact expectation, with RV14-4's representability of [t/L, θ]."""
    nullity, motion = assess_oracle(case)
    case.nullity = nullity
    case.motion = motion
    if nullity == 0:
        return "N"
    if nullity >= 2:
        return "D"
    if motion is None:
        return "U"
    length = reduction(case)[3]
    if any(as_binary64(F(motion[0][i]) / length) is None for i in range(3)):
        return "P"
    return "M"


def subnormal_cases():
    # RV14's minimal case: two nodes 2^-1070 apart, grounds d1-d5. The free x
    # translation is exact (u = (1, 0, 0) at both nodes), but L = 2^-1070 and
    # t/L = 2^1070 overflows.
    h = Case("rv14_h_tiny_free_x", [(0, 0, 0), (2.0 ** -1070, 0, 0)], [[0, 1]], [],
             [("d", k) for k in range(1, 6)])
    out = [h]
    rng = random.Random(SUBNORMAL_SEED)
    out += [rv14_tiny_case(rng, f"rv14_S{i:04d}") for i in range(N_SUBNORMAL)]
    for case in out:
        case.expect = subnormal_expect(case)
        case.klass = "rv14_tiny"
    assert out[0].expect == "P"
    return out


# ------------------------------------------------------------------ output

def fmt(case):
    subs = ";".join(",".join(str(x) for x in s) for s in case.subs)
    ties = ",".join(f"{a}-{b}" for a, b in case.ties) or "-"
    grounds = []
    for g in case.grounds:
        if g[0] == "d":
            grounds.append(f"d{g[1]}")
        else:
            grounds.append(f"{g[0]}{g[1]}:" + ":".join(hexf(c) for c in g[2]))
    coords = ",".join(hexf(c) for p in case.coords for c in p)
    motion = ",".join(hexf(c) for m in case.motion for c in m) if case.motion else "-"
    fields = [case.name, case.expect, f"n={len(case.coords)}", f"subs={subs}", f"ties={ties}",
              f"grounds={','.join(grounds) or '-'}", f"coords={coords}", f"motion={motion}"]
    if case.rigid:
        fields.append(f"rigid={case.rigid}")
    return " ".join(fields)


def build():
    b1, rejected = b1_cases()
    lines = [fmt(c) for c in b1]
    full = "\n".join(lines) + "\n"
    sample = "\n".join(lines[:N_SAMPLE]) + "\n"
    tally = {}
    for c in b1:
        key = f"{c.klass}:{c.expect}"
        tally[key] = tally.get(key, 0) + 1
    summary = [
        "K5 B1 summary (gen_k5_vectors.py)",
        f"seed={SEED}",
        f"records={N_B1} committed_sample={N_SAMPLE}",
        f"full_sha256={hashlib.sha256(full.encode()).hexdigest()}",
        f"sample_sha256={hashlib.sha256(sample.encode()).hexdigest()}",
        f"rejected_null_attempts={rejected['null']} rejected_random_band={rejected['random']}",
    ]
    summary += [f"class {k} {tally[k]}" for k in sorted(tally)]
    summary = "\n".join(summary) + "\n"
    cases = "\n".join(fmt(c) for c in constructed()) + "\n"
    tiny = subnormal_cases()
    tiny_lines = [fmt(c) for c in tiny]
    tiny_full = "\n".join(tiny_lines) + "\n"
    tiny_sample = "\n".join(tiny_lines[:N_SUBNORMAL_SAMPLE + 1]) + "\n"
    tiny_tally = {}
    for c in tiny:
        tiny_tally[c.expect] = tiny_tally.get(c.expect, 0) + 1
    sample_tally = {}
    for c in tiny[:N_SUBNORMAL_SAMPLE + 1]:
        sample_tally[c.expect] = sample_tally.get(c.expect, 0) + 1
    tiny_summary = [
        "K5 RV14-4 subnormal summary (gen_k5_vectors.py)",
        f"source=RV14's tiny-coordinate corpus (tiny_case), seed={SUBNORMAL_SEED}, plus rv14_h_tiny_free_x",
        f"records={len(tiny)} committed_sample={N_SUBNORMAL_SAMPLE + 1}",
        f"full_sha256={hashlib.sha256(tiny_full.encode()).hexdigest()}",
        f"sample_sha256={hashlib.sha256(tiny_sample.encode()).hexdigest()}",
    ]
    tiny_summary += [f"full expect {k} {tiny_tally[k]}" for k in sorted(tiny_tally)]
    tiny_summary += [f"sample expect {k} {sample_tally[k]}" for k in sorted(sample_tally)]
    tiny_summary = "\n".join(tiny_summary) + "\n"
    return {"b1_sample.txt": sample, "b1_summary.txt": summary, "cases.txt": cases,
            "subnormal.txt": tiny_sample, "subnormal_summary.txt": tiny_summary}, (full, tiny_full)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--out", default=os.path.dirname(os.path.abspath(__file__)))
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--full")
    parser.add_argument("--full-subnormal")
    args = parser.parse_args()
    files, (full, tiny_full) = build()
    with open(os.path.abspath(__file__), "rb") as f:
        gen = f.read()
    sums = "".join(f"{hashlib.sha256(files[n].encode()).hexdigest()}  {n}\n" for n in sorted(files))
    sums += f"{hashlib.sha256(gen).hexdigest()}  gen_k5_vectors.py\n"
    files["SHA256SUMS"] = sums
    if args.full:
        with open(args.full, "w") as f:
            f.write(full)
    if args.full_subnormal:
        with open(args.full_subnormal, "w") as f:
            f.write(tiny_full)
    if args.check:
        bad = []
        for name, text in files.items():
            path = os.path.join(args.out, name)
            if not os.path.exists(path) or open(path).read() != text:
                bad.append(name)
        print("check: " + ("OK" if not bad else "MISMATCH " + ", ".join(bad)))
        sys.exit(1 if bad else 0)
    for name, text in files.items():
        with open(os.path.join(args.out, name), "w") as f:
            f.write(text)
    print(files["b1_summary.txt"], end="")


if __name__ == "__main__":
    main()
