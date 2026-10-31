"""Generalized symmetric-definite eigenproblem K phi = lambda M phi for the references.

Route A (published values):
  - Sturm counts: the number of eigenvalues below sigma equals the number of
    negative pivots of an LDL^T factorization of K - sigma M (Sylvester's law
    of inertia; Golub & Van Loan, Matrix Computations, 4th ed., sections 8.4
    and 8.7). The factorization is unpivoted; a symmetric reordering, which
    preserves inertia, is tried when a pivot is zero or (for intervals) not
    sign-definite.
  - Eigenvalues by bisection on the count, in Decimal at the working
    precision, to a relative bracket width of 1e-42.
  - Vectors by inverse iteration with a shift 1e-26 relative below the
    eigenvalue, one block per cluster, M-orthonormalized.
Route B (cross-check only): Cholesky K = R R^T, C = R^-1 M R^-T, cyclic Jacobi
on C (eigenvalues mu = 1/lambda), phi = R^-T z (Golub & Van Loan sections 4.2,
8.5).
Certification: interval Sturm counts (numeric.IntervalField) at bracket ends
lambda (1 -/+ 1e-33) and at every published shift.
"""
from decimal import Decimal
from fractions import Fraction

import elements as el

CLUSTER_REL = Decimal("1e-30")
BISECT_REL = Decimal("1e-42")
SHIFT_REL = Decimal("1e-26")


# --------------------------------------------------------------------------
# Inertia
# --------------------------------------------------------------------------
def _orderings(n):
    yield list(range(n))
    yield list(range(n - 1, -1, -1))
    state = 2463534242
    for _ in range(6):
        perm = list(range(n))
        for i in range(n - 1, 0, -1):
            state = (state * 6364136223846793005 + 1442695040888963407) % (1 << 64)
            j = state % (i + 1)
            perm[i], perm[j] = perm[j], perm[i]
        yield perm


def ldlt_negatives(F, A, order):
    """Negative pivots of the unpivoted LDL^T of A[order][order]; None if a pivot's sign is undecided or zero."""
    n = len(order)
    a = [[A[order[i]][order[j]] for j in range(i + 1)] for i in range(n)]
    neg = 0
    for k in range(n):
        d = a[k][k]
        s = F.sign(d)
        if s is None or s == 0:
            return None
        if s < 0:
            neg += 1
        col = [(i, a[i][k]) for i in range(k + 1, n) if not F.is_zero(a[i][k])]
        for idx, (i, aik) in enumerate(col):
            l = aik / d
            row = a[i]
            for j, ajk in col[: idx + 1]:
                row[j] = row[j] - l * ajk
    return neg


def shifted(F, K, M, sigma):
    n = len(K)
    out = []
    for i in range(n):
        Ki, Mi = K[i], M[i]
        row = []
        for j in range(n):
            k, m = Ki[j], Mi[j]
            if F.is_zero(m):
                row.append(k)
            else:
                row.append(k - sigma * m)
        out.append(row)
    return out


def sturm_count(F, K, M, sigma):
    A = shifted(F, K, M, sigma)
    for idx, order in enumerate(_orderings(len(A))):
        c = ldlt_negatives(F, A, order)
        if c is not None:
            return c, idx
    return None, None


# --------------------------------------------------------------------------
# Dense helpers (point field)
# --------------------------------------------------------------------------
def lu_factor(A):
    n = len(A)
    a = [list(r) for r in A]
    piv = list(range(n))
    for k in range(n):
        p = max(range(k, n), key=lambda i: abs(a[i][k]))
        if a[p][k] == 0:
            raise ZeroDivisionError("singular")
        a[k], a[p] = a[p], a[k]
        piv[k], piv[p] = piv[p], piv[k]
        for i in range(k + 1, n):
            if a[i][k] != 0:
                f = a[i][k] / a[k][k]
                a[i][k] = f
                ak, ai = a[k], a[i]
                for j in range(k + 1, n):
                    ai[j] -= f * ak[j]
    return a, piv


def lu_solve(fac, b):
    a, piv = fac
    n = len(a)
    y = [b[piv[i]] for i in range(n)]
    for i in range(n):
        s = y[i]
        ai = a[i]
        for j in range(i):
            s -= ai[j] * y[j]
        y[i] = s
    for i in range(n - 1, -1, -1):
        s = y[i]
        ai = a[i]
        for j in range(i + 1, n):
            s -= ai[j] * y[j]
        y[i] = s / ai[i]
    return y


def mv(A, x):
    return [sum((a * b for a, b in zip(row, x) if a != 0), Decimal(0)) for row in A]


def vdot(x, y):
    return sum((a * b for a, b in zip(x, y)), Decimal(0))


def m_orthonormalize(M, vecs, against=()):
    out = []
    for v in vecs:
        v = list(v)
        for _ in range(2):
            for q in list(against) + out:
                c = vdot(q, mv(M, v))
                v = [a - c * b for a, b in zip(v, q)]
        nrm = vdot(v, mv(M, v)).sqrt()
        out.append([a / nrm for a in v])
    return out


def rank_psd(M, rel=Decimal("1e-40")):
    """Rank by Gaussian elimination with complete pivoting."""
    a = [list(r) for r in M]
    n = len(a)
    scale = max((abs(x) for r in a for x in r), default=Decimal(0))
    rank = 0
    rows = list(range(n))
    cols = list(range(n))
    for _ in range(n):
        best, bi, bj = Decimal(0), None, None
        for i in rows:
            for j in cols:
                if abs(a[i][j]) > best:
                    best, bi, bj = abs(a[i][j]), i, j
        if bi is None or best <= rel * scale:
            break
        rank += 1
        rows.remove(bi)
        cols.remove(bj)
        for i in rows:
            if a[i][bj] != 0:
                f = a[i][bj] / a[bi][bj]
                for j in cols:
                    a[i][j] -= f * a[bi][j]
    return rank


def start_vectors(n, d, seed):
    state = seed
    out = []
    for _ in range(d):
        v = []
        for _ in range(n):
            state = (state * 6364136223846793005 + 1442695040888963407) % (1 << 64)
            v.append(Decimal(Fraction(state, 1 << 63).numerator) / Decimal(1 << 63) - 1)
        out.append(v)
    return out


# --------------------------------------------------------------------------
# Route A
# --------------------------------------------------------------------------
class Bisection:
    def __init__(self, F, K, M):
        self.F, self.K, self.M = F, K, M
        self.cache = {Decimal(0): 0}
        self.counts = 0

    def count(self, s):
        if s not in self.cache:
            c, _ = sturm_count(self.F, self.K, self.M, s)
            if c is None:
                raise ArithmeticError(f"point Sturm count failed at {s}")
            self.cache[s] = c
            self.counts += 1
        return self.cache[s]

    def eigenvalue(self, k):
        hi = max(self.cache)
        while self.count(hi) < k:
            hi = hi * 16 if hi > 0 else Decimal(1)
        lo = max(s for s, c in self.cache.items() if c < k)
        hi = min(s for s, c in self.cache.items() if c >= k)
        while hi - lo > BISECT_REL * hi:
            if lo > 0 and hi > 4 * lo:
                mid = (lo * hi).sqrt()
            else:
                mid = (lo + hi) / 2
            if self.count(mid) >= k:
                hi = mid
            else:
                lo = mid
        return (lo + hi) / 2


def clusters_of(lams):
    """Group consecutive eigenvalues equal within CLUSTER_REL; returns [(start, d)]."""
    out = []
    k = 0
    while k < len(lams):
        d = 1
        while k + d < len(lams) and lams[k + d] - lams[k + d - 1] <= CLUSTER_REL * lams[k + d]:
            d += 1
        out.append((k, d))
        k += d
    return out


def route_a(K, M, n_finite, cap, F):
    """Lowest eigenvalues in complete clusters (clusters starting below index cap), and the next one.

    Eigenvalues are computed in ascending order until one starts a cluster at
    or beyond cap; the clusters before it are then complete.
    """
    bis = Bisection(F, K, M)
    lams = []
    for k in range(1, n_finite + 1):
        lams.append(bis.eigenvalue(k))
        if clusters_of(lams)[-1][0] >= cap:
            break
    cl = clusters_of(lams)
    reported = [(s, d) for s, d in cl if s < cap]
    n_rep = sum(d for _, d in reported)
    nxt = lams[n_rep] if n_rep < len(lams) else None
    for (s1, d1), (s2, _) in zip(cl, cl[1:]):
        gap = (lams[s2] - lams[s1 + d1 - 1]) / lams[s2]
        if gap < Decimal("1e-20"):
            raise ArithmeticError("distinct eigenvalues closer than 1e-20 relative: not handled")
    return lams[:n_rep], reported, nxt, bis.counts


def inverse_iteration(K, M, lam, d, seed, against):
    n = len(K)
    sigma = lam * (1 - SHIFT_REL)
    A = [[K[i][j] - sigma * M[i][j] for j in range(n)] for i in range(n)]
    fac = lu_factor(A)
    X = m_orthonormalize(M, start_vectors(n, d, seed), against)
    for _ in range(3):
        X = [lu_solve(fac, mv(M, x)) for x in X]
        X = m_orthonormalize(M, X, against)
    return X


def canonical_basis(M, Q, tie_rel=Decimal("1e-25")):
    """Deterministic basis of span(Q) (M-orthonormal columns): greedy M-projections of unit DOF vectors.

    At each step the DOF j maximizing |Q^T M e_j| (ties to the lowest j) is
    projected onto the remaining subspace; the result is M-normalized and
    removed. Returns M-orthonormal vectors.
    """
    n = len(M)
    Q = [list(q) for q in Q]
    out = []
    while Q:
        MQ = [mv(M, q) for q in Q]
        best, bj = Decimal(-1), None
        for j in range(n):
            v = sum((mq[j] * mq[j] for mq in MQ), Decimal(0))
            if v > best * (1 + tie_rel):
                best, bj = v, j
        # projection of e_j onto span(Q): sum_q q (q^T M e_j)
        coef = [mq[bj] for mq in MQ]
        v = [sum((c * q[i] for c, q in zip(coef, Q)), Decimal(0)) for i in range(n)]
        nrm = vdot(v, mv(M, v)).sqrt()
        v = [a / nrm for a in v]
        out.append(v)
        Q = _drop_dependent(M, Q, v)
    return out


def _drop_dependent(M, Q, v):
    """Basis of span(Q) M-orthogonal to v (len(Q) - 1 vectors)."""
    res = []
    for q in Q:
        c = vdot(v, mv(M, q))
        res.append([a - c * b for a, b in zip(q, v)])
    # Gram-Schmidt with pivoting on norm, keep len(Q) - 1
    keep = []
    for _ in range(len(Q) - 1):
        norms = []
        for r in res:
            w = list(r)
            for k in keep:
                c = vdot(k, mv(M, w))
                w = [a - c * b for a, b in zip(w, k)]
            norms.append((vdot(w, mv(M, w)), w))
        nb, w = max(norms, key=lambda t: t[0])
        keep.append([a / nb.sqrt() for a in w])
    return keep


# --------------------------------------------------------------------------
# Route B
# --------------------------------------------------------------------------
def cholesky(A):
    n = len(A)
    R = [[Decimal(0)] * n for _ in range(n)]
    for j in range(n):
        s = A[j][j] - sum((R[j][k] * R[j][k] for k in range(j)), Decimal(0))
        if s <= 0:
            raise ArithmeticError("K not positive definite")
        R[j][j] = s.sqrt()
        for i in range(j + 1, n):
            if A[i][j] == 0 and all(R[i][k] == 0 or R[j][k] == 0 for k in range(j)):
                continue
            R[i][j] = (A[i][j] - sum((R[i][k] * R[j][k] for k in range(j)), Decimal(0))) / R[j][j]
    return R


def forward(R, b):
    n = len(R)
    y = [Decimal(0)] * n
    for i in range(n):
        y[i] = (b[i] - sum((R[i][k] * y[k] for k in range(i) if R[i][k] != 0), Decimal(0))) / R[i][i]
    return y


def backward_t(R, b):
    """Solve R^T x = b, R lower triangular."""
    n = len(R)
    x = [Decimal(0)] * n
    for i in range(n - 1, -1, -1):
        x[i] = (b[i] - sum((R[k][i] * x[k] for k in range(i + 1, n) if R[k][i] != 0), Decimal(0))) / R[i][i]
    return x


def jacobi(A, tol=Decimal("1e-70")):
    n = len(A)
    a = [list(r) for r in A]
    V = [[Decimal(1) if i == j else Decimal(0) for j in range(n)] for i in range(n)]
    norm = sum((x * x for r in a for x in r), Decimal(0)).sqrt()
    sweeps = 0
    while True:
        off = sum((a[i][j] * a[i][j] for i in range(n) for j in range(n) if i != j), Decimal(0)).sqrt()
        if off <= tol * norm or sweeps > 60:
            break
        sweeps += 1
        for p in range(n - 1):
            for q in range(p + 1, n):
                apq = a[p][q]
                if apq == 0 or abs(apq) <= tol * norm * Decimal("1e-5"):
                    continue
                theta = (a[q][q] - a[p][p]) / (2 * apq)
                t = 1 / (abs(theta) + (theta * theta + 1).sqrt())
                if theta < 0:
                    t = -t
                c = 1 / (t * t + 1).sqrt()
                s = t * c
                for k in range(n):
                    akp, akq = a[k][p], a[k][q]
                    a[k][p] = c * akp - s * akq
                    a[k][q] = s * akp + c * akq
                for k in range(n):
                    apk, aqk = a[p][k], a[q][k]
                    a[p][k] = c * apk - s * aqk
                    a[q][k] = s * apk + c * aqk
                for k in range(n):
                    vkp, vkq = V[k][p], V[k][q]
                    V[k][p] = c * vkp - s * vkq
                    V[k][q] = s * vkp + c * vkq
    return [a[i][i] for i in range(n)], [[V[k][i] for k in range(n)] for i in range(n)], sweeps


def route_b(K, M):
    n = len(K)
    R = cholesky(K)
    # C = R^-1 M R^-T: X = R^-1 M (column-wise), C = R^-1 X^T
    X = [forward(R, [M[i][j] for i in range(n)]) for j in range(n)]  # columns of R^-1 M
    C = [forward(R, [X[j][i] for j in range(n)]) for i in range(n)]  # R^-1 (row i of R^-1 M)^T ...
    C = [[(C[i][j] + C[j][i]) / 2 for j in range(n)] for i in range(n)]
    mu, Z, sweeps = jacobi(C)
    pairs = sorted(zip(mu, Z), key=lambda t: -t[0])
    out = []
    mmax = pairs[0][0]
    for m, z in pairs:
        if m <= mmax * Decimal("1e-45"):
            continue
        phi = backward_t(R, z)
        out.append((1 / m, phi))
    return out, sweeps


def principal_sines(M, QA, QB):
    """sin of the largest principal angle between M-orthonormal bases QA and QB."""
    d = len(QA)
    G = [[vdot(qa, mv(M, qb)) for qb in QB] for qa in QA]
    GtG = [[sum((G[k][i] * G[k][j] for k in range(d)), Decimal(0)) for j in range(d)] for i in range(d)]
    ev, _, _ = jacobi(GtG)
    cmin2 = min(ev)
    return (max(Decimal(0), 1 - cmin2)).sqrt()
