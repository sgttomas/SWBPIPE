"""The generator's own planar beam finite elements, two mass formulations.

consistent  Hermitian cubic bending element with consistent translational mass
            and the Rayleigh rotary-inertia matrix (the formulation DEL-04-09
            REQ-001 states for straight members, restricted to one bending plane;
            its linear axial and torsional mass are uncoupled from transverse
            bending and unexcited by transverse support motion);
lumped      the same stiffness with half of each member's mass at each end
            translation and no rotary inertia. NOT the product's formulation;
            present only to measure the size of that modelling difference.

Element matrices are integrated exactly from the shape functions (Fractions),
then compared with the standard printed forms as a cross-check. Pencils are
formed per unit mbar (K/mbar and M/mbar are exact rationals of the binary64
inputs because pi cancels) and solved in decimal at FE_PREC digits: Cholesky
reduction (consistent) or static condensation of the massless rotations
(lumped), then cyclic Jacobi. Every eigenvalue is re-found by bisection on
unpivoted banded LDL^T Sturm counts of K - sigma M, an independent route.
"""

from decimal import Decimal, localcontext
from fractions import Fraction as Fr

from numerics import dec, dsqrt

FE_PREC = 60

HERMITE = [[Fr(1), Fr(0), Fr(-3), Fr(2)], [Fr(0), Fr(1), Fr(-2), Fr(1)],
           [Fr(0), Fr(0), Fr(3), Fr(-2)], [Fr(0), Fr(0), Fr(-1), Fr(1)]]
POW = [0, 1, 0, 1]


def _pmul(a, b):
    out = [Fr(0)] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            out[i + j] += x * y
    return out


def _pder(a, k):
    for _ in range(k):
        a = [i * a[i] for i in range(1, len(a))] or [Fr(0)]
    return a


def _int01(a):
    return sum(c / (i + 1) for i, c in enumerate(a))


def _gram(order):
    return [[_int01(_pmul(_pder(f, order), _pder(g, order))) for g in HERMITE] for f in HERMITE]


KHAT, MHAT, RHAT = _gram(2), _gram(0), _gram(1)
PRINTED_K = [[12, 6, -12, 6], [6, 4, -6, 2], [-12, -6, 12, -6], [6, 2, -6, 4]]
PRINTED_M = [[Fr(v, 420) for v in r] for r in [[156, 22, 54, -13], [22, 4, 13, -3], [54, 13, 156, -22], [-13, -3, -22, 4]]]
PRINTED_R = [[Fr(v, 30) for v in r] for r in [[36, 3, -36, 3], [3, 4, -3, -1], [-36, -3, 36, -3], [3, -1, -3, 4]]]


def element_checks():
    return (KHAT == [[Fr(v) for v in r] for r in PRINTED_K]) and MHAT == PRINTED_M and RHAT == PRINTED_R


def element(h, EI_m, ibar_m, form):
    """(K_e, M_e) per unit mbar for a member of length h (exact Fractions)."""
    K = [[EI_m * KHAT[i][j] * h ** (POW[i] + POW[j] - 3) for j in range(4)] for i in range(4)]
    if form == "consistent":
        M = [[MHAT[i][j] * h ** (POW[i] + POW[j] + 1) + ibar_m * RHAT[i][j] * h ** (POW[i] + POW[j] - 1)
              for j in range(4)] for i in range(4)]
    else:
        M = [[Fr(0)] * 4 for _ in range(4)]
        M[0][0] = M[2][2] = h / 2
    return K, M


RESTRAINTS = {"C": lambda N: {0, 1}, "P": lambda N: {0, 1, 2 * N},
              "S": lambda N: {0, 2 * N}, "F": lambda N: {0, 1, 2 * N, 2 * N + 1}}


class Model:
    pass


def build(span, N, L, EI_m, ibar_m, form):
    md = Model()
    md.span, md.N, md.form, md.L = span, N, form, Fr(L)
    h = Fr(L) / N
    md.h = h
    nd = 2 * (N + 1)
    K = [[Fr(0)] * nd for _ in range(nd)]
    M = [[Fr(0)] * nd for _ in range(nd)]
    Ke, Me = element(h, EI_m, ibar_m, form)
    md.Ke, md.Me = Ke, Me
    for e in range(N):
        d = [2 * e, 2 * e + 1, 2 * e + 2, 2 * e + 3]
        for i in range(4):
            for j in range(4):
                K[d[i]][d[j]] += Ke[i][j]
                M[d[i]][d[j]] += Me[i][j]
    md.K, md.M = K, M
    md.fixed = sorted(RESTRAINTS[span](N))
    md.free = [i for i in range(nd) if i not in md.fixed]
    md.R = [Fr(1) if i % 2 == 0 else Fr(0) for i in range(nd)]
    return md


# --------------------------------------------------------------------------
# decimal linear algebra


def _cholesky(A):
    n = len(A)
    Lm = [[Decimal(0)] * n for _ in range(n)]
    for i in range(n):
        for j in range(i + 1):
            s = A[i][j] - sum((Lm[i][k] * Lm[j][k] for k in range(j)), Decimal(0))
            Lm[i][j] = dsqrt(s) if i == j else s / Lm[j][j]
    return Lm


def _lower_solve(Lm, b):
    n = len(b)
    x = [Decimal(0)] * n
    for i in range(n):
        x[i] = (b[i] - sum((Lm[i][k] * x[k] for k in range(i)), Decimal(0))) / Lm[i][i]
    return x


def _upper_t_solve(Lm, b):
    """Solve L^T x = b."""
    n = len(b)
    x = [Decimal(0)] * n
    for i in range(n - 1, -1, -1):
        x[i] = (b[i] - sum((Lm[k][i] * x[k] for k in range(i + 1, n)), Decimal(0))) / Lm[i][i]
    return x


def jacobi(A, tol_digits):
    n = len(A)
    A = [row[:] for row in A]
    V = [[Decimal(int(i == j)) for j in range(n)] for i in range(n)]
    tol = Decimal(10) ** -tol_digits
    for _sweep in range(100):
        off = max((abs(A[p][q]) for p in range(n) for q in range(p + 1, n)), default=Decimal(0))
        scale = max(abs(A[i][i]) for i in range(n))
        if off <= tol * scale:
            break
        for p in range(n):
            for q in range(p + 1, n):
                apq = A[p][q]
                if apq == 0 or abs(apq) <= tol * Decimal(10) ** -5 * scale:
                    continue
                th = (A[q][q] - A[p][p]) / (2 * apq)
                t = (1 if th >= 0 else -1) / (abs(th) + dsqrt(th * th + 1))
                c = 1 / dsqrt(t * t + 1)
                s = t * c
                for k in range(n):
                    akp, akq = A[k][p], A[k][q]
                    A[k][p] = c * akp - s * akq
                    A[k][q] = s * akp + c * akq
                for k in range(n):
                    apk, aqk = A[p][k], A[q][k]
                    A[p][k] = c * apk - s * aqk
                    A[q][k] = s * apk + c * aqk
                for k in range(n):
                    vkp, vkq = V[k][p], V[k][q]
                    V[k][p] = c * vkp - s * vkq
                    V[k][q] = s * vkp + c * vkq
    else:
        raise RuntimeError("Jacobi did not converge")
    return [A[i][i] for i in range(n)], [[V[k][i] for k in range(n)] for i in range(n)]


def sturm_count(Kf, Mf, sigma):
    """Negative pivots of unpivoted LDL^T of K - sigma M (dense; small n)."""
    n = len(Kf)
    A = [[Kf[i][j] - sigma * Mf[i][j] for j in range(n)] for i in range(n)]
    neg = 0
    for k in range(n):
        d = A[k][k]
        if d == 0:
            d = Decimal(10) ** -(FE_PREC + 10)
        neg += d < 0
        for i in range(k + 1, min(n, k + 4)):
            if A[i][k] != 0:
                l = A[i][k] / d
                for j in range(k + 1, min(n, k + 4)):
                    A[i][j] -= l * A[k][j]
    return neg


def bisect_eig(Kf, Mf, k, lo, hi, digits):
    tol = Decimal(10) ** -digits
    while hi - lo > tol * hi:
        mid = (lo + hi) / 2
        if sturm_count(Kf, Mf, mid) >= k:
            hi = mid
        else:
            lo = mid
    return (lo + hi) / 2


def solve_modes(md):
    """All finite modes, ascending: list of (lam, phi_free) with phi normalized to max |translation| = +1."""
    with localcontext() as ctx:
        ctx.prec = FE_PREC
        f = md.free
        Kf = [[dec(md.K[i][j]) for j in f] for i in f]
        Mf = [[dec(md.M[i][j]) for j in f] for i in f]
        n = len(f)
        trans = [k for k in range(n) if f[k] % 2 == 0]
        if md.form == "consistent":
            Lm = _cholesky(Mf)
            # C = L^-1 K L^-T
            cols = [_lower_solve(Lm, [Kf[i][j] for i in range(n)]) for j in range(n)]  # L^-1 K, by column
            LK = [[cols[j][i] for j in range(n)] for i in range(n)]
            C = [_lower_solve(Lm, LK[i]) for i in range(n)]  # rows of (L^-1 K) L^-T = (L^-1 (L^-1 K)^T)^T
            C = [[(C[i][j] + C[j][i]) / 2 for j in range(n)] for i in range(n)]
            lams, Y = jacobi(C, FE_PREC - 8)
            vecs = [_upper_t_solve(Lm, y) for y in Y]
        else:
            rot = [k for k in range(n) if k not in trans]
            Ktt = [[Kf[i][j] for j in trans] for i in trans]
            Ktr = [[Kf[i][j] for j in rot] for i in trans]
            Krr = [[Kf[i][j] for j in rot] for i in rot]
            from numerics import solve
            # X = Krr^-1 Krt, column by column
            X = [solve(Krr, [Ktr[i][c] for c in range(len(rot))]) for i in range(len(trans))]  # rows: Krr^-1 Krt col i
            Ks = [[Ktt[i][j] - sum((Ktr[i][c] * X[j][c] for c in range(len(rot))), Decimal(0))
                   for j in range(len(trans))] for i in range(len(trans))]
            mt = [Mf[i][i] for i in trans]
            sq = [dsqrt(m) for m in mt]
            C = [[Ks[i][j] / (sq[i] * sq[j]) for j in range(len(trans))] for i in range(len(trans))]
            C = [[(C[i][j] + C[j][i]) / 2 for j in range(len(trans))] for i in range(len(trans))]
            lams, Y = jacobi(C, FE_PREC - 8)
            vecs = []
            for y in Y:
                pt = [y[i] / sq[i] for i in range(len(trans))]
                pr = [-sum((X[i][c] * pt[i] for i in range(len(trans))), Decimal(0)) for c in range(len(rot))]
                v = [Decimal(0)] * n
                for i, k in enumerate(trans):
                    v[k] = pt[i]
                for c, k in enumerate(rot):
                    v[k] = pr[c]
                vecs.append(v)
        order = sorted(range(len(lams)), key=lambda i: lams[i])
        out = []
        for i in order:
            v = vecs[i]
            rots = [k for k in range(n) if k not in trans]
            bigt = max((abs(v[k]) for k in trans), default=Decimal(0))
            bigr = max((abs(v[k]) for k in rots), default=Decimal(0))
            # DEL-04-09 REQ-005: case (b) when the largest translation is at most
            # tau_t * l times the largest rotation (tau_t = 1e-6, l = L for a straight span)
            case = "b" if bigt <= Decimal("1e-6") * dec(md.L) * bigr else "a"
            pool = trans if case == "a" else rots
            big = bigt if case == "a" else bigr
            near = [k for k in pool if big - abs(v[k]) <= big * Decimal(10) ** -40]
            piv = near[0]
            tie = any((v[k] < 0) != (v[piv] < 0) for k in near)
            v = [x / v[piv] for x in v]
            out.append({"lam": lams[i], "phi": v, "tie": tie, "norm_dof": f[piv], "case": case})
        md.Kf, md.Mf = Kf, Mf
        return out
