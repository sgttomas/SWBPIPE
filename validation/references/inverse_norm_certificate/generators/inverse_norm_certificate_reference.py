#!/usr/bin/env python3
"""Inverse-norm certificate: small exact cases, their exact truth, and the stated procedure's emulation.

DEL-04-01's integrity check takes Q = mul_up(1024, est) as an upper bound on the infinity norm of the inverse of
its scaled binary64 stiffness matrix. Exit criterion 18 asks that this be established per matrix: a bound B with
||A^-1||_inf <= B <= Q, or the check unavailable. The frame kernel's certificate arithmetic
(core/solver/frame_kernel/src/structural/inverse_certificate.rs) states a procedure for B: connected blocks,
Lemma E on a shifted profile LDL^T under schedule T, and Lemma D on the unshifted factor. This package gives
that arithmetic cases whose truth is known exactly, and states what the procedure itself should do on them.

Three kinds of value, kept apart (see the README):
  truth      the exact rational inverse infinity norm of each case and of each of its blocks, and what truth
             alone requires of any certificate: a case whose exact norm exceeds Q must not be verified, and any
             bound reported (verified or candidate) is at least the exact norm it bounds.
  avail      availability expectations: which lemma and shift verifies each block, with every figure, from this
             generator's own binary64 emulation of the stated procedure. They say what the procedure does, not
             what is true of the matrix.
  direction  exact-rational checks of the procedure's directed figures on the emulated factor: each upward
             figure is at least, and each downward figure at most, the exact quantity it stands for. Each line
             gives the binary64 number next to the exact quantity on the side the figure must lie.
  operand    the same check against the exact result of a figure's own last operation on that operation's
             binary64 operands as the emulation formed them (gamma, beta, loss, sigma_prime, t, uc): a figure
             can lie on the right side of the ideal quantity only because its operands were already rounded
             outward, and still be on the wrong side of the operation that formed it.

Independence. Python 3 standard library only (fractions, math, hashlib, argparse, pathlib). It reads, imports and
runs no product code, product output, fixture or schema. The procedure is emulated from its statement (the
README's "Procedure"), in Python's binary64 arithmetic, which rounds each operation to nearest, ties to even,
with gradual underflow and no fused multiply-add. The witness matrix (C5) is built from the construction the
README of validation/references/inverse_norm_witness states (K0, lambda, w and its scaling rule), not by reading
that package's generator or outputs.

Usage:  python3 -I -B inverse_norm_certificate_reference.py [--check]  # default: rebuild in memory, compare bytes
        python3 -I -B inverse_norm_certificate_reference.py --write    # rewrite cases.txt, directed.txt, SHA256SUMS
"""

import argparse
import hashlib
import math
import struct
import sys
from fractions import Fraction as F
from pathlib import Path

INF = math.inf
U = 2.0 ** -53            # unit roundoff of binary64, round to nearest
TINY = 2.0 ** -1074       # least positive subnormal; eta = TINY / 2
U_EXACT = F(1, 2 ** 53)
TINY_EXACT = F(1, 2 ** 1074)


class CheckFailure(Exception):
    pass


def check(condition, what):
    if not condition:
        raise CheckFailure(what)


def bits(x):
    return format(struct.unpack("<Q", struct.pack("<d", x))[0], "016x")


def exact_binary64(value):
    f = float(value)
    return math.isfinite(f) and F(f) == value


def ru(q):
    """The least binary64 number >= q (inf beyond the largest finite)."""
    try:
        r = float(q)
    except OverflowError:
        return INF if q > 0 else -sys.float_info.max
    if math.isinf(r):
        return r if r > 0 else -sys.float_info.max
    return r if F(r) >= q else math.nextafter(r, INF)


def rd(q):
    """The greatest binary64 number <= q."""
    return -ru(-q)


def rat(q):
    return f"{q.numerator}/{q.denominator}"


# ---- the directed operations, as stated (operands non-negative; zero operands exact) --------------------------
def add_up(a, b):
    return b if a == 0.0 else a if b == 0.0 else math.nextafter(a + b, INF)


def mul_up(a, b):
    return 0.0 if a == 0.0 or b == 0.0 else math.nextafter(a * b, INF)


def div_up(a, b):
    return 0.0 if a == 0.0 else math.nextafter(a / b, INF)


def sub_dn(a, b):
    return a if b == 0.0 else math.nextafter(a - b, -INF)


# ---- exact linear algebra ------------------------------------------------------------------------------------
def inverse(a):
    n = len(a)
    m = [row[:] + [F(int(i == j)) for j in range(n)] for i, row in enumerate(a)]
    for c in range(n):
        p = next(r for r in range(c, n) if m[r][c] != 0)
        m[c], m[p] = m[p], m[c]
        pv = m[c][c]
        m[c] = [v / pv for v in m[c]]
        for r in range(n):
            if r != c and m[r][c] != 0:
                f = m[r][c]
                m[r] = [x - f * y for x, y in zip(m[r], m[c])]
    return [row[n:] for row in m]


def inf_norm(a):
    return max(sum(abs(v) for v in row) for row in a)


def ldl_positive(a):
    """True when every exact LDL^T pivot is positive (a symmetric positive definite)."""
    n = len(a)
    m = [row[:] for row in a]
    for c in range(n):
        if m[c][c] <= 0:
            return False
        for r in range(c + 1, n):
            f = m[r][c] / m[c][c]
            for k in range(c, n):
                m[r][k] -= f * m[c][k]
    return True


# ---- the cases -----------------------------------------------------------------------------------------------
def chain(n, diag, off):
    return {(i, i): F(diag) for i in range(n)} | {(i + 1, i): F(off) for i in range(n - 1)}


def witness_family(lam):
    """blockdiag(4(I - (1 - lam) w w^T), 1, 1), w = (1, 1, -1, -1)/2; its exact inverse norm is 1/(4 lam)."""
    w = [F(1, 2), F(1, 2), F(-1, 2), F(-1, 2)]
    lower = {}
    for i in range(4):
        for j in range(i + 1):
            lower[(i, j)] = 4 * (F(int(i == j)) - (1 - lam) * w[i] * w[j])
    lower[(4, 4)] = F(1)
    lower[(5, 5)] = F(1)
    return lower


def witness_scaled():
    """The witness's scaled free matrix from validation/references/inverse_norm_witness/README.md: K_free =
    K0 (I - (1 - lambda) w w^T) on (UX, UY, UZ, RX, RY, RZ), K0 = 2^20, lambda = 2^-20, w = (1, 1, -1, -1, 0, 0)/2,
    scaled as K_s = D K D, D = diag(2^e_i), e_i = -floor(floor(log2 K_ii) / 2) (its "scaling" assumption), which
    the README states gives exponents -9 on UX-RX and -10 on RY and RZ."""
    k0, lam = F(2 ** 20), F(1, 2 ** 20)
    w = [F(1, 2), F(1, 2), F(-1, 2), F(-1, 2), F(0), F(0)]
    k = [[k0 * (F(int(i == j)) - (1 - lam) * w[i] * w[j]) for j in range(6)] for i in range(6)]
    exps = []
    for i in range(6):
        check(exact_binary64(k[i][i]) and k[i][i] > 0, "witness diagonal exact and positive")
        log2 = math.frexp(float(k[i][i]))[1] - 1
        exps.append(-(log2 // 2))
    check(exps == [-9, -9, -9, -9, -10, -10], "witness exponents as its README states")
    ks = [[k[i][j] * F(2) ** (exps[i] + exps[j]) for j in range(6)] for i in range(6)]
    check(ks == full_from_lower(witness_family(lam), 6), "witness scaled matrix is the family at 2^-20")
    return {(i, j): ks[i][j] for i in range(6) for j in range(i + 1) if ks[i][j] != 0}


def full_from_lower(lower, n):
    a = [[F(0)] * n for _ in range(n)]
    for (r, c), v in lower.items():
        a[r][c] = v
        a[c][r] = v
    return a


def natural_first(lower, n, order):
    pos = {p: i for i, p in enumerate(order)}
    first = list(range(n))
    for (r, c), v in lower.items():
        if v != 0:
            i, j = max(pos[r], pos[c]), min(pos[r], pos[c])
            first[i] = min(first[i], j)
    return first


def q_from(est):
    """The integrity check's quantity, mul_up(1024, est)."""
    return mul_up(1024.0, est)


def case_list():
    cases = []

    def add(name, role, about, n, lower, q, order=None, first=None, zeros=None):
        """zeros: explicit stored zeros {(row, column): signed binary64 zero}, beside the lower triangle."""
        order = order or list(range(n))
        first = first or natural_first(lower, n, order)
        zeros = zeros or {}
        check(all(z not in lower and v == 0.0 for z, v in zeros.items()), "explicit zeros are new positions")
        stored = dict(lower) | {z: F(0) for z in zeros}
        cases.append(dict(name=name, role=role, about=about, n=n, lower=lower, stored=stored, zero_signs=zeros,
                          q=q, order=order, first=first))

    add("C1", "control_success",
        "a well-conditioned 12-DOF chain (diagonal 4, neighbours -1), natural order; Q = mul_up(1024, 1/2)",
        12, chain(12, 4, -1), q_from(0.5))
    add("C2", "-", "the witness family at lambda = 2^-10 (exact norm 256); Q = mul_up(1024, 1)",
        6, witness_family(F(1, 2 ** 10)), q_from(1.0))
    add("C3", "-", "the witness family at lambda = 2^-11 (exact norm 512); Q = mul_up(1024, 1 + 2^-52)",
        6, witness_family(F(1, 2 ** 11)), q_from(1.0 + 2.0 ** -52))
    add("C4", "-", "the witness family at lambda = 2^-12 (exact norm 1024 <= Q): the premise holds, "
        "yet the procedure does not verify it; Q = mul_up(1024, 1)",
        6, witness_family(F(1, 2 ** 12)), q_from(1.0))
    add("C5", "control_underestimate",
        "the inverse-norm witness's scaled free matrix (exact norm 262144, estimate 1); Q = mul_up(1024, 1)",
        6, witness_scaled(), q_from(1.0))
    # C6: block A on the even positions 0..8 (chain 4, -1), block B on the odd positions 1..7 (diagonal 3,
    # neighbours 1), eliminated in a non-identity order with a full envelope (first = 0), so every cross-block
    # position is computed and must be an exact zero.
    c6 = {}
    for idx, p in enumerate([0, 2, 4, 6, 8]):
        c6[(p, p)] = F(4)
        if idx:
            c6[(p, p - 2)] = F(-1)
    for idx, p in enumerate([1, 3, 5, 7]):
        c6[(p, p)] = F(3)
        if idx:
            c6[(p, p - 2)] = F(1)
    add("C6", "-", "two blocks of sizes 5 and 4 interleaved in free positions, a non-identity elimination order "
        "and a full envelope; Q = mul_up(1024, 1)",
        9, c6, q_from(1.0), order=[8, 1, 6, 3, 4, 5, 2, 7, 0], first=[0] * 9)
    # C7: two separate components joined only by explicit stored zeros, +0.0 at (2, 1) and -0.0 at (3, 0).
    c7 = {(0, 0): F(2), (1, 0): F(-1), (1, 1): F(2), (2, 2): F(3), (3, 2): F(1), (3, 3): F(3)}
    add("C7", "-", "two components joined only by explicit stored zeros (+0.0 at (2,1), -0.0 at (3,0)), "
        "inside the envelope; Q = mul_up(1024, 1)",
        4, c7, q_from(1.0), first=[0, 0, 1, 0], zeros={(2, 1): 0.0, (3, 0): -0.0})
    # C8: [3] with Q = 2^60. sigma_1 = 2^-59 is below E's loss (delta alone is 2u*3), so E does not verify and D
    # runs; there the round-to-nearest quotient U/den lies below the exact quotient, so the Uc operand line tells
    # the directed division apart from round to nearest.
    add("C8", "-", "the 1x1 matrix [3] with Q = 2^60: E's shift is below its loss, D verifies, and the "
        "round-to-nearest quotient for Uc lies below the exact quotient",
        1, {(0, 0): F(3)}, 2.0 ** 60)
    return cases


# ---- the procedure, emulated in binary64 from its statement ----------------------------------------------------
def blocks_of(n, stored):
    """Connected components of the graph whose edges are the nonzero off-diagonal stored values; numbered by
    least free position."""
    adj = {p: set() for p in range(n)}
    for (r, c), v in stored.items():
        if r != c and v != 0:
            adj[r].add(c)
            adj[c].add(r)
    of = [None] * n
    members = []
    for start in range(n):
        if of[start] is not None:
            continue
        b = len(members)
        of[start] = b
        comp, stack = [], [start]
        while stack:
            a = stack.pop()
            comp.append(a)
            for x in adj[a]:
                if of[x] is None:
                    of[x] = b
                    stack.append(x)
        members.append(sorted(comp))
    return of, members


def factor(case, of, shifts):
    """The profile LDL^T loop on the stored values, per-block shifted diagonal, pivot test > 0 (a failed pivot
    replaced by 1). Returns (rows by elimination row: dict j -> value, j <= i; shifted diagonals; first failure
    per block: None, 'pivot' or 'nonfinite')."""
    n, order, first = case["n"], case["order"], case["first"]
    pos = {p: i for i, p in enumerate(order)}
    rows = []
    for i in range(n):
        row = {j: 0.0 for j in range(first[i], i + 1)}
        rows.append(row)
    for (r, c), v in case["stored"].items():
        i, j = max(pos[r], pos[c]), min(pos[r], pos[c])
        if j >= first[i]:
            rows[i][j] = case["zero_signs"].get((r, c), float(v))
        else:
            check(v == 0, f"{case['name']}: envelope holds every stored nonzero")
    failure = [None] * (max(of) + 1 if of else 0)
    work = [0.0] * n
    shifted = [0.0] * n

    def note(b, kind):
        if failure[b] is None:
            failure[b] = kind

    def fin(b, x):
        if math.isfinite(x):
            return x
        note(b, "nonfinite")
        return 0.0

    for i in range(n):
        b = of[order[i]]
        for j in range(first[i], i):
            s = rows[i][j]
            for k in range(max(first[i], first[j]), j):
                s = fin(b, s - fin(b, work[k] * rows[j][k]))
            work[j] = s
            d = rows[j][j]
            check(d > 0 and math.isfinite(d), "divisor finite and positive")
            rows[i][j] = fin(b, s / d)
        dt = fin(b, rows[i][i] - shifts[b])
        shifted[i] = dt
        p = dt
        for k in range(first[i], i):
            p = fin(b, p - fin(b, work[k] * rows[i][k]))
        if not p > 0:
            note(b, "pivot")
            p = 1.0
        rows[i][i] = p
    return rows, shifted, failure


def passes(case, of, rows, b):
    """Upward passes on block b's rows: N' (|L| D |L^T| row sums), nu (|L| D row sums), U (comparison-matrix
    bound), each with its exact counterpart from the same computed factor."""
    n, order, first = case["n"], case["order"], case["first"]
    mine = [i for i in range(n) if of[order[i]] == b]
    lo = {i: [(j, abs(rows[i][j])) for j in range(first[i], i) if rows[i][j] != 0.0] for i in mine}
    for i in mine:
        check(all(of[order[j]] == b for j, _ in lo[i]), "cross-block computed entries are exact zeros")
    d = {i: rows[i][i] for i in mine}
    # a'_j = 1 + sum_{i>j} |l_ij|, rows ascending
    ap, apx = {i: 1.0 for i in mine}, {i: F(1) for i in mine}
    for i in mine:
        for j, l in lo[i]:
            ap[j] = add_up(ap[j], l)
            apx[j] += F(l)
    bp = {i: mul_up(d[i], ap[i]) for i in mine}
    bpx = {i: F(d[i]) * apx[i] for i in mine}
    cp, cpx, nu, nux = {}, {}, {}, {}
    for i in mine:
        acc, accx, v, vx = bp[i], bpx[i], d[i], F(d[i])
        for j, l in lo[i]:
            acc = add_up(acc, mul_up(l, bp[j]))
            accx += F(l) * bpx[j]
            v = add_up(v, mul_up(l, d[j]))
            vx += F(l) * F(d[j])
        cp[i], cpx[i], nu[i], nux[i] = acc, accx, v, vx
    # a_i = 1 + sum_{j<i} |l_ij| a_j; b_i = a_i / d_i; c_j = b_j + sum_{i>j} |l_ij| c_i (rows descending)
    a, ax = {}, {}
    for i in mine:
        acc, accx = 1.0, F(1)
        for j, l in lo[i]:
            acc = add_up(acc, mul_up(l, a[j]))
            accx += F(l) * ax[j]
        a[i], ax[i] = acc, accx
    c = {i: div_up(a[i], d[i]) for i in mine}
    cx = {i: ax[i] / F(d[i]) for i in mine}
    for i in reversed(mine):
        for j, l in lo[i]:
            c[j] = add_up(c[j], mul_up(l, c[i]))
            cx[j] += F(l) * cx[i]
    return dict(n_prime=(max(cp.values()), max(cpx.values())), nu=(max(nu.values()), max(nux.values())),
                u=(max(c.values()), max(cx.values())), mine=mine)


def certify(case):
    """The stated procedure. Returns (verdict, bound or None, blocks), each block a dict with its attempts."""
    n, q, order = case["n"], case["q"], case["order"]
    check(math.isfinite(q) and q > 0, "Q finite and positive")
    m = 2 * n + 2
    check(m <= 2 ** 52, "(2n+2)u <= 1/2")
    of, members = blocks_of(n, case["stored"])
    mu = mul_up(float(m), U)
    one_less = sub_dn(1.0, mu)
    gamma = div_up(mu, one_less)
    gamma_last = F(mu) / F(one_less)
    gamma_x = m * U_EXACT / (1 - m * U_EXACT)
    nf = float(n)
    blocks = []
    for b, mem in enumerate(members):
        k = math.isqrt(len(mem))
        k += k * k < len(mem)
        blocks.append(dict(least=mem[0], size=len(mem), k=k, attempts=[], bound=None, members=mem))

    def beta_of(nu):
        """beta, and the exact product of its last operation's stored operands (None when one is not finite)."""
        left, right = mul_up(TINY, nf), add_up(nf, nu)
        return mul_up(left, right), F(left) * F(right) if math.isfinite(right) else None

    def beta_exact(nux):
        return TINY_EXACT * n * (n + nux)

    def e_attempt(step, sigmas, which):
        rows, shifted, failure = factor(case, of, sigmas)
        for b in which:
            blk = blocks[b]
            sigma = sigmas[b]
            att = dict(attempt=step, figures=[("sigma", sigma, None)], operands=[], candidate=None)
            blk["attempts"].append(att)
            if failure[b] is not None:
                att["outcome"] = ("lemma_e_nonpositive_pivot" if failure[b] == "pivot" else "arithmetic_nonfinite",
                                  None)
                continue
            p = passes(case, of, rows, b)
            dmax = max(abs(shifted[i]) for i in p["mine"])
            delta = mul_up(2.0 * U, dmax)
            npr, nprx = p["n_prime"]
            nu, nux = p["nu"]
            beta, beta_last = beta_of(nu)
            inner = add_up(mul_up(gamma, npr), delta)
            loss = add_up(inner, beta)
            lossx = gamma_x * nprx + 2 * U_EXACT * F(dmax) + beta_exact(nux)
            att["figures"] += [("gamma", gamma, ("ge", gamma_x)), ("n_prime", npr, ("ge", nprx)),
                               ("nu", nu, ("ge", nux)), ("delta", delta, ("ge", 2 * U_EXACT * F(dmax))),
                               ("beta", beta, ("ge", beta_exact(nux))), ("loss", loss, ("ge", lossx))]
            if not all(math.isfinite(x) for x in (npr, nu, delta, beta, loss)):
                att["outcome"] = ("arithmetic_nonfinite", None)
                continue
            att["operands"] += [("gamma", "ge", gamma_last), ("beta", "ge", beta_last),
                                ("loss", "ge", F(inner) + F(beta))]
            sp = sub_dn(sigma, loss)
            att["figures"].append(("sigma_prime", sp, ("le", F(sigma) - lossx)))
            att["operands"].append(("sigma_prime", "le", F(sigma) - F(loss)))
            if not sp > 0:
                att["outcome"] = ("lemma_e_sigma_nonpositive", None)
                continue
            s = div_up(float(blk["k"]), sp)
            att["figures"].append(("s", s, ("ge", F(blk["k"]) / F(sp))))
            if not math.isfinite(s):
                att["outcome"] = ("arithmetic_nonfinite", None)
            elif s <= q:
                att["outcome"] = ("verified", s)
                blk["bound"] = (step, s)
            else:
                att["outcome"] = ("bound_exceeds_existing_quantity", None)
                att["candidate"] = s

    nb = len(blocks)
    sig1 = [div_up(mul_up(2.0, float(blk["k"])), q) for blk in blocks]
    e_attempt("e1", sig1, range(nb))
    retry = [b for b in range(nb) if blocks[b]["attempts"][0]["outcome"][0] == "lemma_e_nonpositive_pivot"]
    if retry:
        sig2 = [div_up(mul_up(1.125, float(blk["k"])), q) if b in retry else sig1[b] for b, blk in enumerate(blocks)]
        e_attempt("e2", sig2, retry)
    need_d = [b for b in range(nb) if blocks[b]["bound"] is None]
    if need_d:
        rows, _, failure = factor(case, of, [0.0] * nb)
        for b in need_d:
            blk = blocks[b]
            att = dict(attempt="d", figures=[("sigma", 0.0, None)], operands=[], candidate=None)
            blk["attempts"].append(att)
            if failure[b] is not None:
                att["outcome"] = ("lemma_d_nonpositive_pivot" if failure[b] == "pivot" else "arithmetic_nonfinite",
                                  None)
                continue
            p = passes(case, of, rows, b)
            uu, ux = p["u"]
            nl, nlx = p["n_prime"]
            nu, nux = p["nu"]
            beta, beta_last = beta_of(nu)
            err = add_up(mul_up(gamma, nl), beta)
            t = mul_up(uu, err)
            tx = ux * (gamma_x * nlx + beta_exact(nux))
            att["figures"] += [("gamma", gamma, ("ge", gamma_x)), ("u", uu, ("ge", ux)), ("n_l", nl, ("ge", nlx)),
                               ("nu", nu, ("ge", nux)), ("beta", beta, ("ge", beta_exact(nux))),
                               ("t", t, ("ge", tx))]
            if not all(math.isfinite(x) for x in (uu, nl, nu, beta, t)):
                att["outcome"] = ("arithmetic_nonfinite", None)
                continue
            att["operands"] += [("gamma", "ge", gamma_last), ("beta", "ge", beta_last),
                                ("t", "ge", F(uu) * F(err))]
            if not t < 1:
                att["outcome"] = ("lemma_d_contraction_not_below_one", None)
                continue
            den = sub_dn(1.0, t)
            att["figures"].append(("den", den, ("le", 1 - F(t))))
            if not den > 0:
                att["outcome"] = ("lemma_d_contraction_not_below_one", None)
                continue
            uc = div_up(uu, den)
            att["figures"].append(("uc", uc, ("ge", ux / (1 - tx))))
            att["operands"].append(("uc", "ge", F(uu) / F(den)))
            if not math.isfinite(uc):
                att["outcome"] = ("arithmetic_nonfinite", None)
            elif uc <= q:
                att["outcome"] = ("verified", uc)
                blk["bound"] = ("d", uc)
            else:
                att["outcome"] = ("bound_exceeds_existing_quantity", None)
                att["candidate"] = uc
    if all(blk["bound"] is not None for blk in blocks):
        bound = max(blk["bound"][1] for blk in blocks)
        check(bound <= q, "a verified bound is at most Q")
        return "verified", bound, blocks
    return "not_verified", None, blocks


# ---- each case: truth, availability, direction ----------------------------------------------------------------
def render_case(case):
    n, q = case["n"], case["q"]
    for (r, c), v in case["stored"].items():
        check(c <= r and exact_binary64(v), f"{case['name']}: stored lower entries exact in binary64")
    a = full_from_lower(case["lower"], n)
    check(ldl_positive(a), f"{case['name']}: symmetric positive definite")
    inv = inverse(a)
    norm = inf_norm(inv)
    verdict, bound, blocks = certify(case)
    out = [f"case {case['name']}", f"role {case['role']}", f"about {case['about']}", f"n {n}",
           "order " + " ".join(map(str, case["order"])), "first " + " ".join(map(str, case["first"])),
           f"q {bits(q)}"]
    for (r, c) in sorted(case["stored"]):
        v = case["zero_signs"].get((r, c), float(case["stored"][(r, c)]))
        out.append(f"entry {r} {c} {bits(v)}")
    # truth
    out.append(f"truth norm {rat(norm)}")
    for blk in blocks:
        sub = [[a[r][c] for c in blk["members"]] for r in blk["members"]]
        bnorm = inf_norm(inverse(sub))
        blk["norm"] = bnorm
        out.append(f"truth block {blk['least']} {blk['size']} {rat(bnorm)}")
    check(max(blk["norm"] for blk in blocks) == norm, "the norm is the largest block norm")
    out.append("truth requires " + ("not_verified" if norm > F(q) else "bound_at_least_norm"))
    # availability, with the truth's consequences checked on the emulation itself
    check(not (norm > F(q) and verdict == "verified"), f"{case['name']}: emulation consistent with truth")
    for blk in blocks:
        out.append(f"avail block {blk['least']} {blk['size']} {blk['k']}")
        for att in blk["attempts"]:
            outcome, value = att["outcome"]
            shown = value if value is not None else att["candidate"]
            if shown is not None:
                check(F(shown) >= blk["norm"], f"{case['name']}: a bound covers its block's exact norm")
            out.append(f"avail attempt {blk['least']} {att['attempt']} {outcome} "
                       f"{bits(shown) if shown is not None else '-'}")
            for name, value, exact in att["figures"]:
                out.append(f"avail figure {blk['least']} {att['attempt']} {name} {bits(value)}")
                if exact is not None:
                    side, x = exact
                    edge = ru(x) if side == "ge" else rd(x)
                    check(value >= edge if side == "ge" else value <= edge,
                          f"{case['name']} {att['attempt']} {name}: directed figure on its side")
                    out.append(f"direction {blk['least']} {att['attempt']} {name} {side} {bits(edge)}")
            formed = {name: value for name, value, _ in att["figures"]}
            for name, side, x in att["operands"]:
                edge = ru(x) if side == "ge" else rd(x)
                check(formed[name] >= edge if side == "ge" else formed[name] <= edge,
                      f"{case['name']} {att['attempt']} {name}: directed figure on the side of its own operation")
                out.append(f"operand {blk['least']} {att['attempt']} {name} {side} {bits(edge)}")
    out.append(f"avail verdict {verdict} {bits(bound) if bound is not None else '-'}")
    out.append("end")
    return out, verdict, bound, norm, blocks


# ---- directed-operation vectors: each directed operation against its exact result ----------------------------------------
def directed_vectors():
    h = TINY
    nd1 = math.nextafter(1.0, 0.0)
    big = sys.float_info.max
    vectors = [
        ("add_up", 1.0, U, "tie to even below the exact sum"),
        ("add_up", 1.0, 2.0 ** -54, "below half an ulp"),
        ("add_up", 1.0 / 3.0, 2.0 / 3.0, "two roundings"),
        ("add_up", h, h, "subnormal, exact"),
        ("add_up", big, big, "overflow"),
        ("mul_up", h, 0.5, "RN(h/2) = 0 < h/2"),
        ("mul_up", 3.0, 1.0 / 3.0, "RN(3 RN(1/3)) = 1 above the exact product"),
        ("mul_up", 1.0 + U * 2, 1.0 + U * 2, "adjacent floats"),
        ("mul_up", 2.0 ** -600, 2.0 ** -600, "product underflows to zero"),
        ("mul_up", big, 2.0, "overflow"),
        ("div_up", 1.0, 3.0, "RN(1/3) < 1/3"),
        ("div_up", h, 2.0, "subnormal quotient rounds to zero"),
        ("div_up", 2.0, 3.0, "RN(2/3) > 2/3"),
        ("sub_dn", 1.0, nd1, "cancellation, exact"),
        ("sub_dn", 1.0, 2.0 ** -54, "below half an ulp"),
        ("sub_dn", 1.0, 1.0 / 3.0, "RN(1 - RN(1/3))"),
        ("sub_dn", 2.0 ** -1073, h, "subnormal, exact"),
        ("sub_dn", 0.5, 0.5, "exact zero"),
    ]
    check(F(1.0 / 3.0) < F(1, 3), "RN(1/3) < 1/3")
    check(h / 2 == 0.0 and F(0) < F(h) / 2, "RN(h/2) = 0 < h/2")
    ops = {"add_up": (add_up, lambda a, b: a + b, "ge"), "mul_up": (mul_up, lambda a, b: a * b, "ge"),
           "div_up": (div_up, lambda a, b: a / b, "ge"), "sub_dn": (sub_dn, lambda a, b: a - b, "le")}
    out = []
    for op, a, b, why in vectors:
        fn, exact, side = ops[op]
        got = fn(a, b)
        x = exact(F(a), F(b))
        edge = ru(x) if side == "ge" else rd(x)
        check(got >= edge if side == "ge" else got <= edge, f"{op} {why}: on its side")
        out.append(f"op {op} {bits(a)} {bits(b)} {bits(got)} {side} {bits(edge)} # {why}")
    # [[2, h], [h, 2]], h = 2^-1074: the loop's l21 = RN(h/2) = 0 and d2 = 2, so L D L^T = 2I. The exact residual
    # at (2,1) is h, nonzero; the relative term gamma (|L| D |L^T|)_21 = gamma |l21| d1 is zero; only the absolute
    # allowance 2^-1074 (n + nu) covers it.
    l21 = h / 2.0
    d2 = 2.0 - (h * l21)
    check(l21 == 0.0 and d2 == 2.0, "the erased off-diagonal")
    residual = F(h) - F(l21) * F(2)
    relative = F(abs(l21)) * F(2)
    nu = F(2)
    allowance = TINY_EXACT * (2 + nu)
    check(residual == F(h) and residual != 0 and relative == 0 and allowance >= residual,
          "beta is necessary and sufficient here")
    check(all(exact_binary64(x) for x in (residual, relative, allowance)), "erased-case values exact in binary64")
    out.append(f"erased {bits(h)} l21 {bits(l21)} d2 {bits(d2)} residual {bits(float(residual))} "
               f"relative {bits(float(relative))} allowance {bits(float(allowance))}")
    return out


def build():
    cases = case_list()
    lines = ["# Inverse-norm certificate cases: see README.md for the format and the three kinds of value.",
             "# Binary64 values are f64::to_bits in hex; rationals are p/q."]
    summary = []
    for case in cases:
        out, verdict, bound, norm, blocks = render_case(case)
        lines += out
        summary.append((case["name"], verdict, bound, norm))
    directed = ["# Directed operations against exact arithmetic: op a b result side edge, where edge is the",
                "# binary64 number next to the exact result on the side the result must lie (ge: result >= edge).",
                *directed_vectors()]
    return "\n".join(lines) + "\n", "\n".join(directed) + "\n", summary


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--write", action="store_true", help="rewrite cases.txt, directed.txt and SHA256SUMS")
    mode.add_argument("--check", action="store_true", help="rebuild in memory and compare bytes (the default)")
    args = parser.parse_args()
    try:
        cases, directed, summary = build()
    except CheckFailure as exc:
        print(f"check failed: {exc}", file=sys.stderr)
        return 1
    root = Path(__file__).resolve().parents[1]
    outputs = {"cases.txt": cases.encode("utf-8"), "directed.txt": directed.encode("utf-8")}
    files = {"README.md": (root / "README.md").read_bytes(),
             "generators/inverse_norm_certificate_reference.py": Path(__file__).read_bytes(), **outputs}
    outputs["SHA256SUMS"] = "".join(f"{hashlib.sha256(d).hexdigest()}  {n}\n"
                                    for n, d in sorted(files.items())).encode("ascii")
    if args.write:
        for name, data in outputs.items():
            (root / name).write_bytes(data)
    else:
        diff = [n for n, d in outputs.items() if not (root / n).is_file() or (root / n).read_bytes() != d]
        if diff:
            print("reference bytes differ: " + ", ".join(diff), file=sys.stderr)
            return 1
    for name, verdict, bound, norm in summary:
        print(f"{name}: exact norm {float(norm)!r}, {verdict}" + (f", B = {bound!r}" if bound is not None else ""))
    print(f"{len(summary)} cases; frozen bytes {'written' if args.write else 'match'}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
