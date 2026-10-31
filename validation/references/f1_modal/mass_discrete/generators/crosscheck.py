"""Independent second routes for the element formulas.

  straight mass      kinetic energy of the interpolated fields, integrated
                     exactly in rationals: linear axial and torsional fields,
                     cubic Hermite transverse field (as a 3-vector), its slope
                     as the Rayleigh section rotation (Przemieniecki ch. 11's
                     derivation, re-done here from the shape functions)
  straight stiffness Castigliano tip flexibility of the clamped member,
                     integrated exactly, inverted and spread by equilibrium
  arc flexibility    40-point Gauss-Legendre quadrature of the same energy
  and half-arc lumps density, with sin and cos evaluated directly by series at
                     each node (no trigonometric-polynomial algebra)
  arc resultant      total mass m R phi and the arc centroid at
                     R sin(phi/2) / (phi/2) from the centre on the bisector
  half-arc series    each half-arc's mass, first moment, centreline second
                     moment and section rotary inertia in its own local frame
                     as power series in the half-angle alpha with exact
                     rational coefficients (stable at small alpha), and the
                     closed forms in alpha evaluated directly
  sign map           straight matrices placed explicitly on [u_y, theta_z]
                     and [u_z, -theta_y] for a member along x
"""
from decimal import Decimal
from fractions import Fraction

import elements as el
from numeric import QPiField, sin_cos_decimal, pi_decimal

QF = QPiField()


# ----------------------------------------------------------------- polynomials
def padd(p, q):
    n = max(len(p), len(q))
    return [(p[i] if i < len(p) else 0) + (q[i] if i < len(q) else 0) for i in range(n)]


def pmul(p, q):
    out = [Fraction(0)] * (len(p) + len(q) - 1)
    for i, a in enumerate(p):
        if a:
            for j, b in enumerate(q):
                out[i + j] += a * b
    return out


def pscale(p, s):
    return [a * s for a in p]


def pder(p):
    return [i * p[i] for i in range(1, len(p))] or [Fraction(0)]


def pint01(p):
    return sum(Fraction(a) / (i + 1) for i, a in enumerate(p))


def vzero():
    return [[Fraction(0)], [Fraction(0)], [Fraction(0)]]


def vadd(u, v):
    return [padd(u[k], v[k]) for k in range(3)]


def vscale_const(poly, vec):
    return [pscale(poly, vec[k]) for k in range(3)]


def vdot_int01(u, v):
    return sum(pint01(pmul(u[k], v[k])) for k in range(3))


def straight_mass_shape(e, L, mbar, ibar, jbar):
    """12x12 consistent mass from shape functions (exact rationals, e rational unit vector)."""
    e = [Fraction(v) for v in e]
    N = [[Fraction(1), Fraction(-1)], [Fraction(0), Fraction(1)]]
    Hd = [[Fraction(1), 0, Fraction(-3), Fraction(2)], [0, 0, Fraction(3), Fraction(-2)]]
    Hr = [[0, Fraction(1), Fraction(-2), Fraction(1)], [0, 0, Fraction(-1), Fraction(1)]]
    U, Th, Tx = [], [], []
    for p in range(12):
        a, c = divmod(p, 6)
        u, th, tx = vzero(), vzero(), [Fraction(0)]
        if c < 3:
            g = [Fraction(1) if k == c else Fraction(0) for k in range(3)]
            u = vadd(u, vscale_const(N[a], [e[k] * e[c] for k in range(3)]))
            pt = [g[k] - e[k] * e[c] for k in range(3)]
            u = vadd(u, vscale_const(Hd[a], pt))
            th = vadd(th, vscale_const(pscale(pder(Hd[a]), 1 / L), pt))
        else:
            g = [Fraction(1) if k == c - 3 else Fraction(0) for k in range(3)]
            tx = pscale(N[a], e[c - 3])
            r = [-v for v in el.cross(e, g)]
            u = vadd(u, vscale_const(pscale(Hr[a], L), r))
            th = vadd(th, vscale_const(pder(Hr[a]), r))
        U.append(u)
        Th.append(th)
        Tx.append(tx)
    M = [[Fraction(0)] * 12 for _ in range(12)]
    for p in range(12):
        for q in range(12):
            M[p][q] = (mbar * L * vdot_int01(U[p], U[q]) + ibar * L * vdot_int01(Th[p], Th[q])
                       + jbar * L * pint01(pmul(Tx[p], Tx[q])))
    return M


def straight_stiffness_castigliano(e, L, EA, EI, GJ):
    """12x12 stiffness from the exact tip flexibility of the clamped member (rationals)."""
    e = [Fraction(v) for v in e]
    # B rows as polynomials in xi (s = xi L); a(s) = (L - s) e = L (1 - xi) e
    one, zero = [Fraction(1)], [Fraction(0)]
    lin = [Fraction(L), Fraction(-L)]
    S = [[zero, [-e[2]], [e[1]]], [[e[2]], zero, [-e[0]]], [[-e[1]], [e[0]], zero]]
    BM = [[pmul(lin, S[r][c]) for c in range(3)] + [one if r == c else zero for c in range(3)] for r in range(3)]
    BN = [[e[0]], [e[1]], [e[2]], zero, zero, zero]
    BT = []
    for c in range(6):
        p = zero
        for r in range(3):
            p = padd(p, pscale(BM[r][c], e[r]))
        BT.append(p)
    f = [[Fraction(0)] * 6 for _ in range(6)]
    for p in range(6):
        for q in range(6):
            bend = sum(pint01(pmul(BM[r][p], BM[r][q])) for r in range(3))
            tors = pint01(pmul(BT[p], BT[q]))
            f[p][q] = L * (pint01(pmul(BN[p], BN[q])) / EA + tors / GJ + (bend - tors) / EI)
    fq = [[QF.q(x) for x in r] for r in f]
    Kjj = el.invert(QF, fq)
    K = el.spread_tip_stiffness(QF, Kjj, [QF.q(L * v) for v in e])
    return [[x.a for x in r] for r in K]


# ----------------------------------------------------------------- quadrature
def gauss_legendre(n, prec):
    """Nodes and weights on [-1, 1] by Newton on the Legendre recurrence."""
    pi = pi_decimal(prec)
    nodes, weights = [], []
    for i in range(1, n + 1):
        _, x = sin_cos_decimal(pi * (Decimal(i) - Decimal("0.25")) / (Decimal(n) + Decimal("0.5")), prec)
        for _ in range(100):
            p0, p1 = Decimal(1), x
            for k in range(2, n + 1):
                p0, p1 = p1, ((2 * k - 1) * x * p1 - (k - 1) * p0) / k
            dp = n * (x * p1 - p0) / (x * x - 1)
            dx = p1 / dp
            x -= dx
            if abs(dx) < Decimal(10) ** (-(prec - 5)):
                break
        p0, p1 = Decimal(1), x
        for k in range(2, n + 1):
            p0, p1 = p1, ((2 * k - 1) * x * p1 - (k - 1) * p0) / k
        dp = n * (x * p1 - p0) / (x * x - 1)
        nodes.append(x)
        weights.append(2 / ((1 - x * x) * dp * dp))
    return nodes, weights


def arc_quadrature(geo, EA, EI, GJ, k, prec, n=40):
    """Tip flexibility (local) and unit half-arc lumps (local) by quadrature."""
    R, phi = geo.R, geo.phi
    nodes, weights = gauss_legendre(n, prec)
    cphi, sphi = sin_cos_decimal(phi, prec)[1], sin_cos_decimal(phi, prec)[0]
    xj = [R * cphi, R * sphi, Decimal(0)]

    def at(psi):
        s, c = sin_cos_decimal(psi, prec)
        return [R * c, R * s, Decimal(0)], [-s, c, Decimal(0)]

    f = [[Decimal(0)] * 6 for _ in range(6)]
    for xq, wq in zip(nodes, weights):
        psi = phi * (xq + 1) / 2
        w = wq * phi / 2 * R
        x, t = at(psi)
        a = [xj[r] - x[r] for r in range(3)]
        ax = [[0, -a[2], a[1]], [a[2], 0, -a[0]], [-a[1], a[0], 0]]
        BM = [[Decimal(ax[r][c]) for c in range(3)] + [Decimal(1 if r == c else 0) for c in range(3)] for r in range(3)]
        BN = t + [Decimal(0)] * 3
        BT = [sum(t[r] * BM[r][c] for r in range(3)) for c in range(6)]
        for p in range(6):
            for q in range(6):
                bend = sum(BM[r][p] * BM[r][q] for r in range(3))
                f[p][q] += w * (BN[p] * BN[q] / EA + BT[p] * BT[q] / GJ + k * (bend - BT[p] * BT[q]) / EI)
    lumps = []
    for lo, hi, P in ((Decimal(0), phi / 2, [R, Decimal(0), Decimal(0)]), (phi / 2, phi, xj)):
        m, S = Decimal(0), [Decimal(0)] * 3
        J = [[Decimal(0)] * 3 for _ in range(3)]
        for xq, wq in zip(nodes, weights):
            psi = lo + (hi - lo) * (xq + 1) / 2
            w = wq * (hi - lo) / 2 * R
            x, _ = at(psi)
            rho = [x[r] - P[r] for r in range(3)]
            rr = sum(v * v for v in rho)
            m += w
            for r in range(3):
                S[r] += w * rho[r]
                for c in range(3):
                    J[r][c] += w * ((rr if r == c else 0) - rho[r] * rho[c])
        lumps.append((m, S, J))
    return f, lumps


# ----------------------------------------------------------------- sign map
def sign_map_matrix(L, A2, T2, B):
    """12x12 for a member along +x, placing the plane matrix B on [u_y, theta_z] and on [u_z, -theta_y].

    B is 4x4 in the order (d_i, r_i, d_j, r_j); A2, T2 2x2 axial and torsional. Exact rationals.
    """
    K = [[Fraction(0)] * 12 for _ in range(12)]
    for a in range(2):
        for b in range(2):
            K[6 * a][6 * b] += A2[a][b]
            K[6 * a + 3][6 * b + 3] += T2[a][b]
    # x-y plane: (u_y, theta_z) with sign +1; x-z plane: (u_z, -theta_y)
    for d_off, r_off, sign in ((1, 5, 1), (2, 4, -1)):
        idx = [(0 + d_off, 1), (0 + r_off, sign), (6 + d_off, 1), (6 + r_off, sign)]
        for p, (gp, sp) in enumerate(idx):
            for q, (gq, sq) in enumerate(idx):
                K[gp][gq] += sp * sq * B[p][q]
    return K


# ----------------------------------------------------------------- half-arc series
N_SERIES = 70


def _cos_series():
    out = [Fraction(0)] * (N_SERIES + 1)
    f = 1
    for n in range(0, N_SERIES + 1, 2):
        out[n] = Fraction((-1) ** (n // 2), f)
        f *= (n + 1) * (n + 2)
    return out


def _sin_series():
    out = [Fraction(0)] * (N_SERIES + 1)
    f = 1
    for n in range(1, N_SERIES + 1, 2):
        out[n] = Fraction((-1) ** (n // 2), f)
        f *= (n + 1) * (n + 2)
    return out


def _trunc(p):
    return (p + [Fraction(0)] * (N_SERIES + 1))[: N_SERIES + 1]


def _integrate0(p):
    """Integral from 0 to alpha of a power series in psi: a power series in alpha."""
    return _trunc([Fraction(0)] + [c / (n + 1) for n, c in enumerate(p)])


def half_arc_series():
    """Local-frame half-arc quantities as power series in alpha (exact coefficients).

    Local frame at the half's end node n: e1 radial (centre to n), e2 the tangent
    at n pointing into the half-arc, e3 = e1 x e2. rho/R = (cos psi - 1, sin psi, 0),
    t = (-sin psi, cos psi, 0), psi in [0, alpha], ds = R dpsi. Each series s gives
      mass / (mbar R)          = alpha
      S_i / (mbar R^2)         = int rho_i/R
      Jc_ij / (mbar R^3)       = int (|rho|^2 delta_ij - rho_i rho_j) / R^2
      Tt_ij / R                = int t_i t_j        (section term R [jbar Tt + ibar (alpha I - Tt)])
    """
    c, s = _cos_series(), _sin_series()
    one = _trunc([Fraction(1)])
    r1 = _trunc(padd(c, pscale(one, -1)))
    r2 = s
    t1 = pscale(s, -1)
    t2 = c
    rr = _trunc(padd(pmul(r1, r1), pmul(r2, r2)))
    out = {
        "mass": _integrate0(one),
        "S_1": _integrate0(r1),
        "S_2": _integrate0(r2),
        "Jc_11": _integrate0(_trunc(pmul(r2, r2))),
        "Jc_22": _integrate0(_trunc(pmul(r1, r1))),
        "Jc_33": _integrate0(rr),
        "Jc_12": _integrate0(_trunc(pscale(pmul(r1, r2), -1))),
        "Tt_11": _integrate0(_trunc(pmul(t1, t1))),
        "Tt_22": _integrate0(_trunc(pmul(t2, t2))),
        "Tt_12": _integrate0(_trunc(pmul(t1, t2))),
    }
    return out


HALF_ARC_CLOSED_FORMS = {
    "mass": "a",
    "S_1": "sin a - a",
    "S_2": "1 - cos a",
    "Jc_11": "a/2 - sin(2a)/4",
    "Jc_22": "3a/2 + sin(2a)/4 - 2 sin a",
    "Jc_33": "2 (a - sin a)",
    "Jc_12": "1 - cos a - sin(a)^2/2",
    "Tt_11": "a/2 - sin(2a)/4",
    "Tt_22": "a/2 + sin(2a)/4",
    "Tt_12": "-sin(a)^2/2",
}


def half_arc_closed_forms(alpha, prec):
    """The closed forms above, evaluated directly in Decimal (cancelling at small alpha)."""
    sa, ca = sin_cos_decimal(alpha, prec)
    s2, _ = sin_cos_decimal(2 * alpha, prec)
    return {
        "mass": alpha, "S_1": sa - alpha, "S_2": 1 - ca,
        "Jc_11": alpha / 2 - s2 / 4, "Jc_22": 3 * alpha / 2 + s2 / 4 - 2 * sa, "Jc_33": 2 * (alpha - sa),
        "Jc_12": 1 - ca - sa * sa / 2, "Tt_11": alpha / 2 - s2 / 4, "Tt_22": alpha / 2 + s2 / 4,
        "Tt_12": -sa * sa / 2,
    }


def series_eval(p, x):
    total = Decimal(0)
    for c in reversed(p):
        total = total * x + (Decimal(c.numerator) / Decimal(c.denominator) if c else 0)
    return total


def leading_terms(p, n=3):
    out = []
    for k, c in enumerate(p):
        if c:
            out.append(f"{c}*a^{k}")
            if len(out) == n:
                break
    return out


def half_arc_local_from_series(series, alpha, R, mbar, jbar, ibar):
    """Mass, S (3), Jc (3x3), Js (3x3) of one half in its local frame, from the series."""
    v = {k: series_eval(p, alpha) for k, p in series.items()}
    z = Decimal(0)
    m = mbar * R * v["mass"]
    S = [mbar * R * R * v["S_1"], mbar * R * R * v["S_2"], z]
    R3 = mbar * R * R * R
    Jc = [[R3 * v["Jc_11"], R3 * v["Jc_12"], z], [R3 * v["Jc_12"], R3 * v["Jc_22"], z], [z, z, R3 * v["Jc_33"]]]
    Tt = [[v["Tt_11"], v["Tt_12"], z], [v["Tt_12"], v["Tt_22"], z], [z, z, z]]
    Js = [[R * (jbar * Tt[a][b] + ibar * ((v["mass"] if a == b else z) - Tt[a][b])) for b in range(3)]
          for a in range(3)]
    return m, S, Jc, Js
