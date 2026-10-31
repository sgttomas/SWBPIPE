"""F1 inertia-risk witnesses (design 3.3, 3.4 check 4).

Small symmetric pencils (K, M) with binary64-exact entries and binary64
shifts. For each shift the exact count of eigenvalues below it (and equal to
it) comes from exact rational LDL^T inertia (`numerics.count_exact`, with a
pivoted fallback). Alongside, the generator records what an unpivoted
binary64 LDL^T in this file's stated operation order observes: the
negative-pivot count, pivot extremes, max|L| and the design's proposed growth
g = max(1, max|L|)^2 max|d| / max|a_jj|. Those observations illustrate the
hazard; they are not reference values, since another operation order rounds
differently. The exact counts are the references.
"""

import math
from decimal import Decimal, localcontext
from fractions import Fraction as Fr

from numerics import (count_exact, dec, fmt, inertia_exact, inp, pencil_eigenvalues, rat,
                      verify_simple_eigenvalues)

PIVOT_GUARD = 1e-12  # the product's absolute pivot guard, recorded for comparison


# --------------------------------------------------------------------------
# binary64 observation: right-looking unpivoted LDL^T on the lower triangle


def form_b64(K, M, s, scale):
    """A = fl(K - fl(s M)); scaled: fl(fl(D_i A_ij) D_j), D_i = fl(1/fl(sqrt|K_ii|))."""
    n = len(K)
    A = [[K[i][j] - s * M[i][j] for j in range(n)] for i in range(n)]
    if scale:
        D = [1.0 / math.sqrt(abs(K[i][i])) for i in range(n)]
        A = [[(D[i] * A[i][j]) * D[j] for j in range(n)] for i in range(n)]
    return A


def ldl_b64(A):
    n = len(A)
    W = [row[:] for row in A]
    neg = 0
    piv = []
    maxl = 0.0
    for k in range(n):
        d = W[k][k]
        if d == 0.0:
            return {"breakdown": f"exact zero pivot at ordered row {k}"}
        neg += d < 0
        piv.append(d)
        for i in range(k + 1, n):
            if W[i][k] == 0.0:
                continue
            l = W[i][k] / d
            maxl = max(maxl, abs(l))
            for j in range(k + 1, i + 1):
                W[i][j] -= l * W[j][k]
    maxd = max(abs(p) for p in piv)
    maxa = max(abs(A[j][j]) for j in range(n))
    g = max(1.0, maxl) ** 2 * maxd / maxa
    return {"count": neg, "min_abs_pivot": repr(min(abs(p) for p in piv)), "max_abs_pivot": repr(maxd),
            "max_abs_L": repr(maxl), "max_abs_diag": repr(maxa), "growth_g": repr(g),
            "pivot_guard_trips": any(abs(p) <= PIVOT_GUARD for p in piv)}


def observe(K, M, s, exact_neg):
    out = {}
    for tag, sc in (("unscaled", False), ("scaled", True)):
        o = ldl_b64(form_b64(K, M, s, sc))
        if "count" in o:
            o["count_correct"] = o["count"] == exact_neg
        out[tag] = o
    return out


# --------------------------------------------------------------------------
# helpers


def F(A):
    return [[Fr(v) for v in row] for row in A]


def mat_doc(A):
    return [[repr(float(v)) for v in row] for row in A]


def congruent_pencil(B, lams):
    """K = B^T diag(lams) B, M = B^T B: eigenvalues exactly lams. Entries must be binary64."""
    n = len(B)
    K = [[sum(B[k][i] * lams[k] * B[k][j] for k in range(n)) for j in range(n)] for i in range(n)]
    M = [[sum(B[k][i] * B[k][j] for k in range(n)) for j in range(n)] for i in range(n)]
    for A in (K, M):
        for row in A:
            for v in row:
                assert Fr(float(v)) == v, "entry not binary64-exact"
    return [[float(v) for v in row] for row in K], [[float(v) for v in row] for row in M]


def shift_rows(K, M, shifts, known=None):
    """known: exact sorted eigenvalues (Fractions) to cross-check counts against."""
    rows = []
    ok = True
    Kf, Mf = F(K), F(M)
    for s in shifts:
        s = float(s)
        neg, zero = count_exact(Kf, Mf, Fr(s))
        if known is not None:
            ok &= neg == sum(1 for l in known if l < Fr(s)) and zero == sum(1 for l in known if l == Fr(s))
        rows.append({"shift": inp(s), "exact_count_below": neg, "exact_count_equal": zero,
                     "binary64_observation": observe(K, M, s, neg)})
    return rows, ok


def chain(springs, masses):
    n = len(masses)
    K = [[0.0] * n for _ in range(n)]
    for i in range(n):
        K[i][i] = springs[i] + (springs[i + 1] if i + 1 < n else 0.0)
        if i + 1 < n:
            K[i][i + 1] = K[i + 1][i] = -springs[i + 1]
    M = [[masses[i] if i == j else 0.0 for j in range(n)] for i in range(n)]
    return K, M


def near_shifts(lams, rels=("1e-8", "1e-13")):
    out = []
    for l in lams:
        for r in rels:
            out += [float(l * (1 - Decimal(r))), float(l * (1 + Decimal(r)))]
        out.append(float(l))  # the nearest binary64 to the eigenvalue
    return out


def numeric_eigs(K, M, count):
    with localcontext() as c:
        c.prec = 70
        lams = pencil_eigenvalues(F(K), F(M), count, 55)
        checks = verify_simple_eigenvalues(F(K), F(M), lams, 40)
    return lams, checks


# --------------------------------------------------------------------------
# Codex's round-2 witness: 32x32 tridiagonal, a binary64 shift within 1e-17
# relative of an eigenvalue, wrong binary64 count, unchanged growth.

R2_N = 32
R2_OFF = -0.45
R2_STEP = 2.0 ** -40
R2_KK = 38    # frozen result of `r2_search` (first hit from kk = 0 upward)
R2_MODE = 2


def r2_masses():
    # m_i = round(2^16 * 2^(-i/8)) / 2^16, computed in decimal (no libm)
    out = []
    with localcontext() as c:
        c.prec = 50
        for i in range(R2_N):
            v = (Decimal(2) ** (Decimal(-i) / 8) * 65536).to_integral_value()
            out.append(float(Fr(int(v), 65536)))
    return out


def r2_matrix(kk):
    b = [R2_OFF] * (R2_N - 1)
    b[0] = R2_OFF - kk * R2_STEP
    return [1.0] * R2_N, b, r2_masses()


def tri_count_exact(a, b, m, s):
    s = Fr(s)
    neg = 0
    d = None
    for i in range(len(a)):
        x = Fr(a[i]) - s * Fr(m[i])
        if i:
            x -= Fr(b[i - 1]) ** 2 / d
        if x == 0:
            return None
        neg += x < 0
        d = x
    return neg


def tri_b64(a, b, m, s, variant):
    """Unpivoted LDL^T of tridiagonal K - sM in binary64.
    A: d_i = (a_i - s m_i) - b^2/d;  B: l = b/d, d_i = (a_i - s m_i) - l b;
    C: l = b/d, d_i = (a_i - s m_i) - (l l) d.  B is also the dense routine's order."""
    neg = 0
    d = None
    piv, ls = [], []
    for i in range(len(a)):
        x = a[i] - s * m[i]
        if i:
            if variant == "A":
                x = x - (b[i - 1] * b[i - 1]) / d
            elif variant == "B":
                l = b[i - 1] / d
                ls.append(abs(l))
                x = x - l * b[i - 1]
            else:
                l = b[i - 1] / d
                ls.append(abs(l))
                x = x - (l * l) * d
        neg += x < 0
        piv.append(x)
        d = x
    maxd = max(abs(p) for p in piv)
    maxa = max(abs(a[i] - s * m[i]) for i in range(len(a)))
    maxl = max(ls) if ls else max(abs(b[i - 1] / piv[i - 1]) for i in range(1, len(a)))
    return {"count": neg, "last_pivot": repr(piv[-1]), "min_abs_pivot_rows_1_to_n-1": repr(min(abs(p) for p in piv[:-1])),
            "max_abs_pivot": repr(maxd), "max_abs_L": repr(maxl),
            "growth_g": repr(max(1.0, maxl) ** 2 * maxd / maxa),
            "pivot_guard_trips": any(abs(p) <= PIVOT_GUARD for p in piv)}


def tri_eig(a, b, m, k):
    with localcontext() as c:
        c.prec = 80
        n = len(a)

        def cnt(s):
            neg = 0
            d = None
            for i in range(n):
                x = dec(a[i]) - s * dec(m[i])
                if i:
                    x -= dec(b[i - 1]) ** 2 / d
                if x == 0:
                    x = Decimal("1e-90")
                neg += x < 0
                d = x
            return neg

        lo, hi = Decimal(0), Decimal(4)
        while hi - lo > Decimal("1e-70"):
            mid = (lo + hi) / 2
            if cnt(mid) >= k:
                hi = mid
            else:
                lo = mid
        return +((lo + hi) / 2)


def r2_hit(kk, mode):
    a, b, m = r2_matrix(kk)
    lam = tri_eig(a, b, m, mode)
    s = float(lam)
    rel = abs(dec(s) - lam) / lam
    if not (0 < rel < Decimal("1e-17")):
        return None
    ex = tri_count_exact(a, b, m, s)
    obs = {v: tri_b64(a, b, m, s, v) for v in "ABC"}
    if all(o["count"] != ex and abs(float(o["last_pivot"])) > 1e-11 for o in obs.values()):
        return a, b, m, lam, s, rel, ex, obs
    return None


def r2_search(limit=400):
    for kk in range(limit):
        for mode in (2, 3):
            if r2_hit(kk, mode):
                return kk, mode
    return None


def round2(cross):
    hit = r2_hit(R2_KK, R2_MODE)
    assert hit, "frozen round-2 witness no longer reproduces"
    a, b, m, lam, s, rel, ex, obs = hit
    lam_prev = tri_eig(a, b, m, R2_MODE - 1)
    lam_next = tri_eig(a, b, m, R2_MODE + 1)
    # observation at neighbouring binary64 shifts and at separated shifts
    probes = [("sigma", s), ("sigma - 1 ulp", math.nextafter(s, -math.inf)),
              ("sigma + 1 ulp", math.nextafter(s, math.inf)),
              ("lambda (1 - 1e-9)", float(lam * (1 - Decimal("1e-9")))),
              ("lambda (1 + 1e-9)", float(lam * (1 + Decimal("1e-9")))),
              ("gap midpoint below", float((lam_prev + lam) / 2)),
              ("gap midpoint above", float((lam + lam_next) / 2))]
    prow = []
    for tag, x in probes:
        e = tri_count_exact(a, b, m, x)
        o = {v: tri_b64(a, b, m, x, v) for v in "ABC"}
        prow.append({"probe": tag, "shift": inp(x), "exact_count_below": e,
                     "binary64": {v: dict(o[v], count_correct=o[v]["count"] == e) for v in "ABC"}})
    # unchanged growth: max|L| and max|d| at sigma equal those at lambda(1 +- 1e-9)
    def g(tag, v, key):
        return float(next(p for p in prow if p["probe"] == tag)["binary64"][v][key])
    same = all(abs(g("sigma", v, k) / g(t, v, k) - 1) < 1e-6
               for v in "ABC" for k in ("max_abs_L", "max_abs_pivot", "growth_g")
               for t in ("lambda (1 - 1e-9)", "lambda (1 + 1e-9)"))
    cross.append({"id": "WIT-ROUND2-unchanged-growth", "what": "max|L|, max|d| and g at sigma agree to 1e-6 with lambda(1 +- 1e-9), where the count is right",
                  "agrees": same})
    sep_ok = all(p["binary64"][v]["count_correct"] for p in prow if p["probe"] != "sigma" and "ulp" not in p["probe"] for v in "ABC")
    cross.append({"id": "WIT-ROUND2-separated-shifts-correct", "what": "binary64 counts are right at the separated probes",
                  "agrees": sep_ok})
    star = 2 * lam - dec(s)  # mirrored intended shift on the other side of lambda
    star_ex = tri_count_exact(a, b, m, Fr(star))
    cross.append({"id": "WIT-ROUND2-rounds-away", "what": "the mirrored shift 2 lambda - sigma (within 1e-17 rel.) rounds to sigma and has the other exact count",
                  "agrees": float(star) == s and star_ex != ex})
    return {
        "id": "WIT-ROUND2",
        "description": "Codex's round-2 witness (design 3.3): a 32x32 symmetric tridiagonal pencil on which a binary64 shift "
                       "within 1e-17 relative of an eigenvalue gives a wrong unpivoted-LDL^T count in binary64, with growth "
                       "observations unchanged from separated shifts where the count is right. The separation rule must "
                       "refuse this shift; it is not to be counted.",
        "construction": "K: unit diagonal, off-diagonal -0.45 except K[0][1] = -0.45 - kk 2^-40; M = diag(m_i), "
                        "m_i = round(2^16 2^(-i/8))/2^16. kk is the first value from 0 upward (modes 2 and 3 tried) at which the "
                        "nearest binary64 sigma to an eigenvalue lies within 1e-17 relative, the exact count at sigma differs "
                        "from all three binary64 variants, and each variant's last pivot exceeds 1e-11 in magnitude. "
                        "D = diag(|K_ii|^-1/2) = I, so the product's scaling leaves this pencil unchanged.",
        "kk": R2_KK, "mode": R2_MODE, "n": R2_N,
        "K_diagonal": [repr(x) for x in a], "K_offdiagonal": [repr(x) for x in b],
        "K_offdiagonal_exact": [inp(x) for x in b[:1]],
        "M_diagonal": [inp(x) for x in m],
        "eigenvalue": fmt(lam, 50), "neighbour_eigenvalues": [fmt(lam_prev, 30), fmt(lam_next, 30)],
        "shift": inp(s), "relative_distance": fmt(rel, 6),
        "exact_count_below_shift": ex, "exact_count_equal": 0,
        "binary64_variants": {v: dict(obs[v], count_correct=False) for v in "ABC"},
        "variant_definitions": "A: d_i = (a_i - s m_i) - b_{i-1}^2/d_{i-1}; B: l = b_{i-1}/d_{i-1}, d_i = (a_i - s m_i) - l b_{i-1} "
                               "(also the dense right-looking order); C: d_i = (a_i - s m_i) - (l l) d_{i-1}. a_i - s m_i is fl(a_i - fl(s m_i)).",
        "probes": prow,
        "mirrored_intended_shift": {"value": fmt(star, 50), "rounds_to_shift": float(star) == s,
                                    "exact_count_below": star_ex},
    }


# --------------------------------------------------------------------------


def build(cross):
    wits = []

    # missed eigenvalue
    K = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
    M = [[1.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 3.0]]
    known = [Fr(1, 3), Fr(1, 2), Fr(1)]
    rows, ok = shift_rows(K, M, [0.25, 0.4, 0.5, 0.75, 1.5], known)
    cross.append({"id": "WIT-MISSED-known", "what": "exact counts vs eigenvalues 1/3, 1/2, 1", "agrees": ok})
    wits.append({"id": "WIT-MISSED", "description": "K = I, M = diag(1, 2, 3): eigenvalues 1/3, 1/2, 1. A solver reporting "
                 "{1/2, 1} has missed 1/3: the count at 1.5 is 3, not 2. A solver reporting {1/3, 1/2} places its final shift in "
                 "(1/2, 1) and counts 2. The shift 0.5 lies exactly on an eigenvalue (unpivoted LDL^T meets a zero pivot).",
                 "K": mat_doc(K), "M": mat_doc(M), "eigenvalues_exact": [rat(x) for x in known], "shifts": rows})

    # near-coincident pair and cluster: K = B^T L B, M = B^T B
    B4 = [[2, 1, 0, 0], [1, 2, 1, 0], [0, 1, 2, 1], [1, 0, 1, 3]]
    lam4 = [Fr(1), 1 + Fr(1, 2**40), 1 + Fr(1, 2**20), Fr(3)]
    K, M = congruent_pencil(B4, lam4)
    assert inertia_exact(F(M)) == (0, 0, 4)
    sh = [0.5, 1 + 2.0**-41, 1 + 2.0**-40 - 2.0**-52, 1 + 2.0**-40 + 2.0**-52, 1 + 2.0**-30,
          1 + 2.0**-20 - 2.0**-52, 1 + 2.0**-20 + 2.0**-52, 2.0, 4.0]
    rows, ok = shift_rows(K, M, sh, lam4)
    cross.append({"id": "WIT-NEAR-PAIR-known", "what": "exact counts vs constructed eigenvalues", "agrees": ok})
    wits.append({"id": "WIT-NEAR-PAIR", "description": "K = B^T diag(1, 1+2^-40, 1+2^-20, 3) B, M = B^T B (integer B): "
                 "eigenvalues exactly 1, 1+2^-40, 1+2^-20, 3; shifts one ulp-scale step either side of each.",
                 "B": B4, "K": mat_doc(K), "M": mat_doc(M), "eigenvalues_exact": [rat(x) for x in lam4], "shifts": rows})

    B5 = [[1, 1, 0, 0, 0], [0, 2, 1, 0, 1], [1, 0, 2, 1, 0], [0, 1, 0, 1, 1], [1, 0, 0, 1, 2]]
    e = Fr(1, 2**36)
    lam5 = [Fr(1), 1 + e, 1 + 2 * e, 1 + 3 * e, Fr(2)]
    K, M = congruent_pencil(B5, lam5)
    assert inertia_exact(F(M)) == (0, 0, 5)
    sh = [1 - 2.0**-40, 1 + 2.0**-37, 1 + 3 * 2.0**-37, 1 + 5 * 2.0**-37, 1 + 7 * 2.0**-37,
          1 + 2.0**-36 - 2.0**-50, 1 + 2.0**-36 + 2.0**-50, 1.5]
    rows, ok = shift_rows(K, M, sh, lam5)
    cross.append({"id": "WIT-CLUSTER-known", "what": "exact counts vs constructed eigenvalues", "agrees": ok})
    wits.append({"id": "WIT-CLUSTER", "description": "A cluster of four eigenvalues 1 + k 2^-36 (k = 0..3) and one at 2, "
                 "K = B^T diag B, M = B^T B; shifts in each intra-cluster gap and 2^-50 either side of 1+2^-36.",
                 "B": B5, "K": mat_doc(K), "M": mat_doc(M), "eigenvalues_exact": [rat(x) for x in lam5], "shifts": rows})

    # stiffness scales: spring chains with 1e3 and 1e6 contrast
    for s_, tag in ((1.0e3, "1E3"), (1.0e6, "1E6")):
        springs = [1.0, 1.0, s_, s_, 1.0, 1.0]
        masses = [1.0, 2.0, 1.0, 0.5, 1.0, 1.0]
        K, M = chain(springs, masses)
        lams, checks = numeric_eigs(K, M, 6)
        rows, _ = shift_rows(K, M, near_shifts(lams))
        wits.append({"id": f"WIT-SCALE-{tag}", "description": f"fixed-free spring chain, springs {springs} (stiff pair x{s_:g}, "
                     "a rigid-family scale), masses as listed; shifts at lambda(1 +- 1e-8), lambda(1 +- 1e-13) and the "
                     "nearest binary64 to each eigenvalue.",
                     "springs": [repr(x) for x in springs], "masses": [repr(x) for x in masses],
                     "K": mat_doc(K), "M": mat_doc(M), "eigenvalues": [fmt(x, 45) for x in lams],
                     "eigenvalue_exact_sturm": checks, "shifts": rows})

    # zero pivot at a non-eigenvalue
    K, M = chain([1.0, 1.0, 1.0], [1.0, 1.0, 1.0])
    lams, checks = numeric_eigs(K, M, 3)
    rows, _ = shift_rows(K, M, [1.0, 2.0, 2.0 - 2.0**-30, 2.0 + 2.0**-30])
    wits.append({"id": "WIT-ZERO-PIVOT", "description": "K = chain(1,1,1), M = I. At shift 2 = K_00/M_00 the first pivot is "
                 "exactly zero though 2 is not an eigenvalue (eigenvalues 2 - 2 cos((2j-1) pi/7)); at 2 +- 2^-30 the first "
                 "pivot is tiny and |L| ~ 2^30. Exact counts by pivoted inertia.",
                 "K": mat_doc(K), "M": mat_doc(M), "eigenvalues": [fmt(x, 45) for x in lams],
                 "eigenvalue_exact_sturm": checks, "shifts": rows})

    # consistent-mass FE pencil with binary64-exact entries
    import fe_convergence as FE
    Kc, Mc = FE.beam_pencil(4, "cantilever")
    h = Fr(1, 4)
    Mp = [[v * 420 / h for v in row] for row in Mc]
    for A in (Kc, Mp):
        for row in A:
            for v in row:
                assert Fr(float(v)) == v
    K = [[float(v) for v in row] for row in Kc]
    M = [[float(v) for v in row] for row in Mp]
    lams, checks = numeric_eigs(K, M, 8)
    rows, _ = shift_rows(K, M, near_shifts(lams[:4], ("1e-10", "1e-14")))
    wits.append({"id": "WIT-FE-CANT4", "description": "Euler-Bernoulli cantilever, 4 consistent-mass elements, L = EI = mbar = 1, "
                 "with M scaled by 420/h (h = 1/4) so every entry is binary64-exact; eigenvalues are Omega^2 h/420. Shifts at "
                 "lambda(1 +- 1e-10), lambda(1 +- 1e-14) and the nearest binary64 to each of the lowest four.",
                 "dof_order": "w1, theta1, ..., w4, theta4", "K": mat_doc(K), "M": mat_doc(M),
                 "eigenvalues": [fmt(x, 45) for x in lams], "eigenvalue_exact_sturm": checks, "shifts": rows})

    wits.append(round2(cross))

    # summary of binary64 miscounts observed (illustrative)
    summary = []
    for w in wits:
        for r in w.get("shifts", []):
            for t in ("unscaled", "scaled"):
                o = r["binary64_observation"][t]
                if "count" in o and not o["count_correct"]:
                    summary.append({"witness": w["id"], "shift": r["shift"]["binary64"], "form": t,
                                    "binary64_count": o["count"], "exact_count": r["exact_count_below"]})
    return {"observation_order": "binary64, round to nearest, no fused multiply-add: A = fl(K - fl(s M)); scaled "
                                 "A~ = fl(fl(D_i A_ij) D_j), D_i = fl(1/fl(sqrt|K_ii|)); right-looking unpivoted LDL^T on "
                                 "the lower triangle, l_ik = a_ik/d_k, a_ij <- a_ij - l_ik a_jk for k < j <= i. g = "
                                 "max(1, max|L|)^2 max|d|/max|a_jj| (design 3.4 proposal).",
            "pivot_guard_recorded": repr(PIVOT_GUARD),
            "witnesses": wits, "binary64_miscounts_observed": summary}
