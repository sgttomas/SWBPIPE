"""T4-I6: independent reference for T4-U1's objective curved element.

Standard library only (decimal). Derived from first principles (unit-load /
Castigliano complementary energy of a circular arc, Euler-Bernoulli: axial,
torsion, in-plane and out-of-plane bending scaled by k_in / k_out, no shear),
with the geometry formed only from d = x_j - x_i, the user radius R and the
plane reference vector y_ref (arc bows toward +y_ref projected normal to d).
No product formula is transcribed here.
"""
from decimal import Decimal as D, localcontext, getcontext
import math

PREC = 110


def dec(v):
    """Exact Decimal of a binary64 float (or of an int / Decimal / str)."""
    if isinstance(v, D):
        return v
    if isinstance(v, float):
        return D(v)
    return D(v)


# ---------------------------------------------------------------- scalars
_PI_CACHE = {}


def _atan_series(x):
    # x small (|x| < 0.01): sum (-1)^n x^(2n+1)/(2n+1)
    ctx = getcontext()
    eps = D(10) ** (-(ctx.prec + 5))
    term = x
    x2 = x * x
    total = D(0)
    n = 0
    while True:
        t = term / (2 * n + 1)
        if abs(t) < eps:
            break
        total += t if n % 2 == 0 else -t
        term *= x2
        n += 1
    return total


def pi():
    p = getcontext().prec
    if p not in _PI_CACHE:
        with localcontext() as c:
            c.prec = p + 10
            v = 4 * (4 * _atan_inv(5) - _atan_inv(239))
        _PI_CACHE[p] = +v
    return _PI_CACHE[p]


def _atan_inv(n):
    # atan(1/n) by series
    ctx = getcontext()
    eps = D(10) ** (-(ctx.prec + 5))
    x = D(1) / n
    x2 = x * x
    term = x
    total = D(0)
    k = 0
    while abs(term) > eps:
        t = term / (2 * k + 1)
        total += t if k % 2 == 0 else -t
        term *= x2
        k += 1
    return total


def atan(x):
    """atan for x >= 0."""
    assert x >= 0
    with localcontext() as c:
        c.prec += 15
        if x > 1:
            r = pi() / 2 - atan(D(1) / x)
        else:
            k = 0
            y = x
            while y > D("0.001"):
                y = y / (1 + (1 + y * y).sqrt())
                k += 1
            r = _atan_series(y) * (2 ** k)
    return +r


def atan2_q1(y, x):
    """atan2 for y >= 0, x >= 0 (first quadrant)."""
    assert y >= 0 and x >= 0
    if x == 0:
        return pi() / 2
    if y <= x:
        return atan(y / x)
    return pi() / 2 - atan(x / y)


def sin(x):
    with localcontext() as c:
        c.prec += 20
        eps = D(10) ** (-(c.prec + 5))
        term = x
        total = D(0)
        n = 1
        x2 = x * x
        while abs(term) > eps:
            total += term
            term = -term * x2 / ((n + 1) * (n + 2))
            n += 2
    return +total


def cos(x):
    with localcontext() as c:
        c.prec += 20
        eps = D(10) ** (-(c.prec + 5))
        term = D(1)
        total = D(0)
        n = 0
        x2 = x * x
        while abs(term) > eps:
            total += term
            term = -term * x2 / ((n + 1) * (n + 2))
            n += 2
    return +total


# ---------------------------------------------------------------- vectors
def dot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def sub(a, b):
    return [a[k] - b[k] for k in range(3)]


def add(a, b):
    return [a[k] + b[k] for k in range(3)]


def scale(s, a):
    return [s * a[k] for k in range(3)]


def norm(a):
    return dot(a, a).sqrt()


def matmul(A, B):
    n, m, p = len(A), len(B), len(B[0])
    return [[sum((A[i][k] * B[k][j] for k in range(m)), D(0)) for j in range(p)] for i in range(n)]


def transpose(A):
    return [list(r) for r in zip(*A)]


def matvec(A, v):
    return [sum((A[i][k] * v[k] for k in range(len(v))), D(0)) for i in range(len(A))]


def inverse(M):
    n = len(M)
    A = [list(M[i]) + [D(1) if i == j else D(0) for j in range(n)] for i in range(n)]
    for col in range(n):
        piv = max(range(col, n), key=lambda r: abs(A[r][col]))
        if A[piv][col] == 0:
            raise ZeroDivisionError("singular")
        A[col], A[piv] = A[piv], A[col]
        p = A[col][col]
        A[col] = [v / p for v in A[col]]
        for r in range(n):
            if r != col and A[r][col] != 0:
                f = A[r][col]
                A[r] = [A[r][k] - f * A[col][k] for k in range(2 * n)]
    return [row[n:] for row in A]


def solve(M, b):
    n = len(M)
    A = [list(M[i]) + [b[i]] for i in range(n)]
    for col in range(n):
        piv = max(range(col, n), key=lambda r: abs(A[r][col]))
        if A[piv][col] == 0:
            raise ZeroDivisionError("singular")
        A[col], A[piv] = A[piv], A[col]
        for r in range(col + 1, n):
            if A[r][col] != 0:
                f = A[r][col] / A[col][col]
                A[r] = [A[r][k] - f * A[col][k] for k in range(n + 1)]
    x = [D(0)] * n
    for i in range(n - 1, -1, -1):
        s = A[i][n] - sum((A[i][k] * x[k] for k in range(i + 1, n)), D(0))
        x[i] = s / A[i][i]
    return x


def skew(c):
    return [[D(0), -c[2], c[1]], [c[2], D(0), -c[0]], [-c[1], c[0], D(0)]]


# ---------------------------------------------------------------- the element
# Unit-load actions at section theta of the arc (local frame: centre at the
# origin, P(theta) = R(cos t, sin t, 0), node i at t = 0, node j at t = phi).
# For a unit generalized load f_a at node j (i clamped), the resultant on the
# section is F (constant) and M(theta) = M + (P(phi) - P(theta)) x F.
# Components: N = F.t, T = M(theta).t, M_ip = M(theta).z, M_op = M(theta).e_r,
# t = (-sin, cos, 0), e_r = (cos, sin, 0), z = (0, 0, 1).
# Each is a combination of the basis (1, cos theta, sin theta).
def unit_load_actions(R, sinp, cosp):
    z3 = (D(0), D(0), D(0))
    acts = {}
    # order: Fx, Fy, Fz, Mx, My, Mz ; each: dict action -> (c1, ccos, csin)
    acts[0] = {"N": (D(0), D(0), D(-1)), "Mip": (-R * sinp, D(0), R), "Mop": z3, "T": z3}
    acts[1] = {"N": (D(0), D(1), D(0)), "Mip": (R * cosp, -R, D(0)), "Mop": z3, "T": z3}
    acts[2] = {"N": z3, "Mip": z3, "Mop": (D(0), R * sinp, -R * cosp), "T": (R, -R * cosp, -R * sinp)}
    acts[3] = {"N": z3, "Mip": z3, "Mop": (D(0), D(1), D(0)), "T": (D(0), D(0), D(-1))}
    acts[4] = {"N": z3, "Mip": z3, "Mop": (D(0), D(0), D(1)), "T": (D(0), D(1), D(0))}
    acts[5] = {"N": z3, "Mip": (D(1), D(0), D(0)), "Mop": z3, "T": z3}
    return acts


def gram(phi, sinp, cosp):
    sin2p = 2 * sinp * cosp
    one_minus_cos = 1 - cosp
    return [
        [phi, sinp, one_minus_cos],
        [sinp, phi / 2 + sin2p / 4, sinp * sinp / 2],
        [one_minus_cos, sinp * sinp / 2, phi / 2 - sin2p / 4],
    ]


def quad(G, u, v):
    return sum((u[i] * G[i][j] * v[j] for i in range(3) for j in range(3)), D(0))


def geometry(xi, xj, R, yref):
    """Objective arc geometry from d = xj - xi, R and y_ref (all exact)."""
    d = sub(xj, xi)
    L = norm(d)
    R = dec(R)
    s = L / (2 * R)  # sin(phi/2)
    if not s < 1:
        raise ValueError("R must exceed L/2")
    ch = (((2 * R - L) * (2 * R + L)).sqrt()) / (2 * R)  # cos(phi/2)
    half = atan2_q1(s, ch)
    phi = 2 * half
    dh = scale(1 / L, d)
    yd = dot(yref, dh)
    nr = sub(yref, scale(yd, dh))
    nn = norm(nr)
    n = scale(1 / nn, nr)
    z = cross(n, dh)
    x = add(scale(-s, dh), scale(ch, n))
    y = add(scale(ch, dh), scale(s, n))
    sinp = 2 * s * ch
    cosp = 1 - 2 * s * s
    return dict(d=d, L=L, R=R, s=s, ch=ch, phi=phi, dh=dh, n=n, axes=[x, y, z], sinp=sinp, cosp=cosp,
                yproj_norm=nn)


def flexibility(geo, E, G, A, I, J, kin, kout):
    R, phi, sinp, cosp = geo["R"], geo["phi"], geo["sinp"], geo["cosp"]
    Gm = gram(phi, sinp, cosp)
    acts = unit_load_actions(R, sinp, cosp)
    EI, GJ, EA = E * I, G * J, E * A
    F = [[D(0)] * 6 for _ in range(6)]
    for a in range(6):
        for b in range(a, 6):
            v = R * (kin * quad(Gm, acts[a]["Mip"], acts[b]["Mip"]) / EI
                     + kout * quad(Gm, acts[a]["Mop"], acts[b]["Mop"]) / EI
                     + quad(Gm, acts[a]["T"], acts[b]["T"]) / GJ
                     + quad(Gm, acts[a]["N"], acts[b]["N"]) / EA)
            F[a][b] = v
            F[b][a] = v
    return F


def flexibility_parts(geo, E, G, A, I, J):
    """The four energy parts separately (k = 1), for cross-checks."""
    R, phi, sinp, cosp = geo["R"], geo["phi"], geo["sinp"], geo["cosp"]
    Gm = gram(phi, sinp, cosp)
    acts = unit_load_actions(R, sinp, cosp)
    out = {}
    for key, rig in (("Mip", E * I), ("Mop", E * I), ("T", G * J), ("N", E * A)):
        out[key] = [[R * quad(Gm, acts[a][key], acts[b][key]) / rig for b in range(6)] for a in range(6)]
    return out


def local_from_tip(Kt, c):
    """12x12 local matrix [[H Kt H^T, -H Kt], [-Kt H^T, Kt]], H = [[I,0],[skew(c), I]]."""
    S = skew(c)
    H = [[D(0)] * 6 for _ in range(6)]
    for i in range(3):
        H[i][i] = D(1)
        H[i + 3][i + 3] = D(1)
        for j in range(3):
            H[i + 3][j] = S[i][j]
    HK = matmul(H, Kt)
    HKHt = matmul(HK, transpose(H))
    KHt = matmul(Kt, transpose(H))
    K = [[D(0)] * 12 for _ in range(12)]
    for i in range(6):
        for j in range(6):
            K[i][j] = HKHt[i][j]
            K[i][j + 6] = -HK[i][j]
            K[i + 6][j] = -KHt[i][j]
            K[i + 6][j + 6] = Kt[i][j]
    return K


def to_global(Kl, axes):
    T = [[D(0)] * 12 for _ in range(12)]
    for blk in range(4):
        for i in range(3):
            for j in range(3):
                T[3 * blk + i][3 * blk + j] = axes[i][j]
    return matmul(transpose(T), matmul(Kl, T))


def curved_element(xi, xj, R, yref, E, G, A, I, J, kin, kout, chord=None):
    """Global 12x12 of the intended element. chord=None: H from the actual chord
    axes.d (the definition). chord=[cx,cy,cz]: a mutant H from a supplied local chord."""
    xi = [dec(v) for v in xi]
    xj = [dec(v) for v in xj]
    yref = [dec(v) for v in yref]
    E, G, A, I, J, kin, kout = (dec(v) for v in (E, G, A, I, J, kin, kout))
    geo = geometry(xi, xj, R, yref)
    F = flexibility(geo, E, G, A, I, J, kin, kout)
    Kt = inverse(F)
    c_act = [dot(geo["axes"][k], geo["d"]) for k in range(3)]
    c = c_act if chord is None else [dec(v) for v in chord]
    Kl = local_from_tip(Kt, c)
    Kg = to_global(Kl, geo["axes"])
    return dict(geo=geo, F=F, Kt=Kt, c_act=c_act, Kl=Kl, K=Kg)


def frame_element(xi, xj, yref, E, G, A, Iy, Iz, J):
    """Straight Euler-Bernoulli 3D frame (no shear), local x along d, y from y_ref."""
    xi = [dec(v) for v in xi]
    xj = [dec(v) for v in xj]
    yref = [dec(v) for v in yref]
    E, G, A, Iy, Iz, J = (dec(v) for v in (E, G, A, Iy, Iz, J))
    d = sub(xj, xi)
    L = norm(d)
    ex = scale(1 / L, d)
    yr = sub(yref, scale(dot(yref, ex), ex))
    ey = scale(1 / norm(yr), yr)
    ez = cross(ex, ey)
    K = [[D(0)] * 12 for _ in range(12)]

    def put(i, j, v):
        K[i][j] += v
    ea, gj = E * A / L, G * J / L
    for (i, j, s) in ((0, 0, 1), (0, 6, -1), (6, 0, -1), (6, 6, 1)):
        put(i, j, s * ea)
    for (i, j, s) in ((3, 3, 1), (3, 9, -1), (9, 3, -1), (9, 9, 1)):
        put(i, j, s * gj)
    kz = E * Iz / L ** 3
    dofs = (1, 5, 7, 11)
    mz = [[12, 6 * L, -12, 6 * L], [6 * L, 4 * L * L, -6 * L, 2 * L * L],
          [-12, -6 * L, 12, -6 * L], [6 * L, 2 * L * L, -6 * L, 4 * L * L]]
    for a in range(4):
        for b in range(4):
            put(dofs[a], dofs[b], kz * mz[a][b])
    ky = E * Iy / L ** 3
    dofs = (2, 4, 8, 10)
    my = [[12, -6 * L, -12, -6 * L], [-6 * L, 4 * L * L, 6 * L, 2 * L * L],
          [-12, 6 * L, 12, 6 * L], [-6 * L, 2 * L * L, 6 * L, 4 * L * L]]
    for a in range(4):
        for b in range(4):
            put(dofs[a], dofs[b], ky * my[a][b])
    return to_global(K, [ex, ey, ez])


def rigid_modes(xi, xj):
    """Six rigid motions of the actual nodes: 3 translations, 3 rotations about node i."""
    xi = [dec(v) for v in xi]
    xj = [dec(v) for v in xj]
    d = sub(xj, xi)
    modes = []
    for k in range(3):
        m = [D(0)] * 12
        m[k] = D(1)
        m[6 + k] = D(1)
        modes.append(("T" + "xyz"[k], m))
    for k in range(3):
        w = [D(0)] * 3
        w[k] = D(1)
        sw = cross(w, d)
        m = [D(0)] * 12
        for c in range(3):
            m[3 + c] = w[c]
            m[6 + c] = sw[c]
            m[9 + c] = w[c]
        modes.append(("R" + "xyz"[k], m))
    return modes


def max_abs(M):
    return max(abs(v) for row in M for v in row)


def sig(v, n=20):
    """Decimal -> string with n significant digits (round half even)."""
    if v == 0:
        return "0"
    with localcontext() as c:
        c.prec = n
        r = +v
    return format(r, "E") if (abs(r) >= D("1e21") or abs(r) < D("1e-6")) else format(r, "f") if r == r.to_integral() else str(r)


def sci(v, n=20):
    if v == 0:
        return "0"
    with localcontext() as c:
        c.prec = n
        r = +v
    s = format(r, "E")
    return s


# ---------------------------------------------------------------- quadrature (independent check)
def gauss_legendre(n):
    """Nodes/weights on [-1, 1] by Newton on P_n (decimal)."""
    xs, ws = [], []
    for i in range(1, n + 1):
        x = dec(math.cos(math.pi * (i - 0.25) / (n + 0.5)))
        for _ in range(100):
            p0, p1 = D(1), x
            for k in range(2, n + 1):
                p0, p1 = p1, ((2 * k - 1) * x * p1 - (k - 1) * p0) / k
            dp = n * (x * p1 - p0) / (x * x - 1)
            dx = p1 / dp
            x -= dx
            if abs(dx) < D(10) ** (-(getcontext().prec - 5)):
                break
        p0, p1 = D(1), x
        for k in range(2, n + 1):
            p0, p1 = p1, ((2 * k - 1) * x * p1 - (k - 1) * p0) / k
        dp = n * (x * p1 - p0) / (x * x - 1)
        xs.append(x)
        ws.append(2 / ((1 - x * x) * dp * dp))
    return xs, ws


def flexibility_by_quadrature(R, phi, E, G, A, I, J, kin, kout, n=40):
    """F by Gauss-Legendre quadrature of the action products, the actions formed
    directly from vectors (no basis decomposition, no closed-form Gram)."""
    R, phi, E, G, A, I, J, kin, kout = (dec(v) for v in (R, phi, E, G, A, I, J, kin, kout))
    xs, ws = gauss_legendre(n)
    Pj = [R * cos(phi), R * sin(phi), D(0)]
    zh = [D(0), D(0), D(1)]
    F = [[D(0)] * 6 for _ in range(6)]
    for x, w in zip(xs, ws):
        th = phi * (x + 1) / 2
        ct, st = cos(th), sin(th)
        P = [R * ct, R * st, D(0)]
        t = [-st, ct, D(0)]
        er = [ct, st, D(0)]
        arm = sub(Pj, P)
        acts = []
        for a in range(6):
            Fv = [D(0)] * 3
            Mv = [D(0)] * 3
            if a < 3:
                Fv[a] = D(1)
            else:
                Mv[a - 3] = D(1)
            Mt = add(Mv, cross(arm, Fv))
            acts.append((dot(Fv, t), dot(Mt, t), dot(Mt, zh), dot(Mt, er)))
        wt = w * phi / 2
        for a in range(6):
            for b in range(6):
                Na, Ta, Ia, Oa = acts[a]
                Nb, Tb, Ib, Ob = acts[b]
                F[a][b] += wt * R * (kin * Ia * Ib / (E * I) + kout * Oa * Ob / (E * I)
                                     + Ta * Tb / (G * J) + Na * Nb / (E * A))
    return F
