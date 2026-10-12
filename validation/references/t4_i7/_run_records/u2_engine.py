"""T4-I7 direct reference engine for T4-U2 (pressure through realized bends).

Standard library only. Python decimal arithmetic; series for pi, sin, cos and atan; Gauss-Legendre
quadrature. A chain of straight and circular-arc members is analysed as a cantilever from node A by the
unit-load (flexibility) method: section actions are the closed-form resultants of every load beyond the
cut (arc wetted-wall load pAi/R along the outward normal, terminal caps, kink forces, self-weight), the
axial eigenstrain alpha*dT - 2 nu pAi/(E As) enters as n*eps0, and an anchored end D is solved by its six
redundants. No curved-element stiffness and no K*u_free is formed here.
"""
from decimal import Decimal as D, getcontext

getcontext().prec = 60
ZERO = D(0)
ONE = D(1)
TWO = D(2)
HALF = D(1) / D(2)


def eps_digits():
    return D(10) ** (-(getcontext().prec + 5))


# ---------------------------------------------------------------- math
def _atan_series(x):
    # |x| small
    tol = eps_digits()
    x2 = x * x
    term = x
    s = x
    n = 1
    while True:
        term = -term * x2
        n += 2
        t = term / n
        s += t
        if abs(t) < tol:
            return s


_PI = None


def pi():
    global _PI
    if _PI is None:
        with_prec = getcontext().prec
        getcontext().prec = with_prec + 10
        v = 16 * _atan_series(ONE / 5) - 4 * _atan_series(ONE / 239)
        getcontext().prec = with_prec
        _PI = +v
    return _PI


def atan(x):
    x = D(x)
    if x < 0:
        return -atan(-x)
    if x > 1:
        return pi() / 2 - atan(ONE / x)
    # halve until small
    k = 0
    while x > D("0.1"):
        x = x / (ONE + (ONE + x * x).sqrt())
        k += 1
    return _atan_series(x) * (2 ** k)


def atan2(y, x):
    y = D(y)
    x = D(x)
    if x > 0:
        return atan(y / x)
    if x < 0:
        return atan(y / x) + (pi() if y >= 0 else -pi())
    if y > 0:
        return pi() / 2
    if y < 0:
        return -pi() / 2
    raise ValueError("atan2(0,0)")


def _sin_series(x):
    tol = eps_digits()
    x2 = x * x
    term = x
    s = x
    n = 1
    while True:
        term = -term * x2 / ((n + 1) * (n + 2))
        n += 2
        s += term
        if abs(term) < tol:
            return s


def _cos_series(x):
    tol = eps_digits()
    x2 = x * x
    term = ONE
    s = ONE
    n = 0
    while True:
        term = -term * x2 / ((n + 1) * (n + 2))
        n += 2
        s += term
        if abs(term) < tol:
            return s


def _reduce(x):
    p2 = 2 * pi()
    x = D(x)
    if abs(x) > p2:
        k = (x / p2).to_integral_value()
        x = x - k * p2
    if x > pi():
        x -= p2
    if x < -pi():
        x += p2
    return x


def sin(x):
    x = _reduce(x)
    if x > pi() / 2:
        x = pi() - x
    elif x < -pi() / 2:
        x = -pi() - x
    return _sin_series(x)


def cos(x):
    return sin(pi() / 2 - D(x))


def asin(x):
    x = D(x)
    if abs(x) >= 1:
        raise ValueError("asin domain")
    return atan(x / (ONE - x * x).sqrt())


def sqrt(x):
    return D(x).sqrt()


# ---------------------------------------------------------------- vectors
def v(x, y, z):
    return (D(x), D(y), D(z))


def add(a, b):
    return (a[0] + b[0], a[1] + b[1], a[2] + b[2])


def sub(a, b):
    return (a[0] - b[0], a[1] - b[1], a[2] - b[2])


def scl(s, a):
    return (s * a[0], s * a[1], s * a[2])


def dot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def cross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def norm(a):
    return dot(a, a).sqrt()


def unit(a):
    n = norm(a)
    return scl(ONE / n, a)


VZERO = (ZERO, ZERO, ZERO)
EAX = [v(1, 0, 0), v(0, 1, 0), v(0, 0, 1)]


# ---------------------------------------------------------------- linear algebra
def solve(A, b):
    n = len(b)
    M = [list(A[i]) + [b[i]] for i in range(n)]
    for c in range(n):
        piv = max(range(c, n), key=lambda r: abs(M[r][c]))
        if M[piv][c] == 0:
            raise ZeroDivisionError("singular")
        M[c], M[piv] = M[piv], M[c]
        for r in range(c + 1, n):
            f = M[r][c] / M[c][c]
            if f != 0:
                row_r = M[r]
                row_c = M[c]
                for k in range(c, n + 1):
                    row_r[k] -= f * row_c[k]
    x = [ZERO] * n
    for r in range(n - 1, -1, -1):
        s = M[r][n]
        for k in range(r + 1, n):
            s -= M[r][k] * x[k]
        x[r] = s / M[r][r]
    return x


def inverse(A):
    n = len(A)
    cols = [solve(A, [ONE if i == j else ZERO for i in range(n)]) for j in range(n)]
    return [[cols[j][i] for j in range(n)] for i in range(n)]


# ---------------------------------------------------------------- Gauss-Legendre
_GL = {}


def gauss_legendre(n):
    if n in _GL:
        return _GL[n]
    import math
    xs, ws = [], []
    tol = D(10) ** (-(getcontext().prec - 4))
    for i in range(1, n + 1):
        x = D(repr(math.cos(math.pi * (i - 0.25) / (n + 0.5))))
        for _ in range(200):
            p0, p1 = ONE, x
            for k in range(2, n + 1):
                p0, p1 = p1, ((2 * k - 1) * x * p1 - (k - 1) * p0) / k
            dp = n * (x * p1 - p0) / (x * x - 1)
            dx = p1 / dp
            x -= dx
            if abs(dx) < tol:
                break
        # recompute derivative at converged x
        p0, p1 = ONE, x
        for k in range(2, n + 1):
            p0, p1 = p1, ((2 * k - 1) * x * p1 - (k - 1) * p0) / k
        dp = n * (x * p1 - p0) / (x * x - 1)
        xs.append(x)
        ws.append(2 / ((1 - x * x) * dp * dp))
    _GL[n] = (xs, ws)
    return _GL[n]


GL_STRAIGHT = 4
GL_ARC = 30


# ---------------------------------------------------------------- section and material
class Section:
    def __init__(self, od, wall, mill="0"):
        self.od = D(od)
        self.wall = D(wall)
        self.mill = D(mill)
        self.t = self.wall - self.mill
        self.ro = self.od / 2
        self.ri = self.ro - self.t
        self.idia = self.od - 2 * self.t
        p = pi()
        self.As = p * (self.od ** 2 - self.idia ** 2) / 4
        self.Ai = p * self.idia ** 2 / 4
        self.I = p * (self.od ** 4 - self.idia ** 4) / 64
        self.J = 2 * self.I
        self.Z = self.I / self.ro


class Material:
    def __init__(self, E, nu, alpha="0"):
        self.E = D(E)
        self.nu = D(nu)
        self.alpha = D(alpha)
        self.G = self.E / (2 * (1 + self.nu))


# ---------------------------------------------------------------- members
class Member:
    """A straight or arc member in authored direction i->j, with a traversal flag."""

    def __init__(self, pid, kind, i_id, j_id, xi, xj, sec, mat, yref, R=None, k=None):
        self.pid = pid
        self.kind = kind
        self.i_id = i_id
        self.j_id = j_id
        self.xi = xi
        self.xj = xj
        self.sec = sec
        self.mat = mat
        self.yref = yref
        self.k = D(1) if kind == "straight" else D(k)
        d = sub(xj, xi)
        self.chord = d
        self.chord_len = norm(d)
        cx = scl(ONE / self.chord_len, d)
        yp = sub(yref, scl(dot(yref, cx), cx))
        cy = unit(yp)
        cz = cross(cx, cy)
        self.chord_frame = (cx, cy, cz)
        if kind == "straight":
            self.length = self.chord_len
            self.ex, self.ey, self.ez = cx, cy, cz
        else:
            self.R = D(R)
            half = self.chord_len / 2
            if self.R <= half:
                raise ValueError("radius cannot span chord")
            self.Phi = 2 * asin(half / self.R)
            s = (self.R * self.R - half * half).sqrt()
            mid = scl(HALF, add(xi, xj))
            self.centre = sub(mid, scl(s, cy))
            ri = sub(xi, self.centre)
            rj = sub(xj, self.centre)
            self.xhat = scl(ONE / self.R, ri)  # |ri| = R exactly in exact arithmetic
            self.zhat = unit(cross(ri, rj))
            self.yhat = cross(self.zhat, self.xhat)
            self.length = self.R * self.Phi
        self.forward = True
        self._set_trav()

    def set_forward(self, forward):
        self.forward = forward
        self._set_trav()

    def _set_trav(self):
        if self.kind == "straight":
            if self.forward:
                self.p0, self.t0 = self.xi, self.ex
            else:
                self.p0, self.t0 = self.xj, scl(-ONE, self.ex)
        else:
            if self.forward:
                self.e1, self.e2 = self.xhat, self.yhat
            else:
                c, s = cos(self.Phi), sin(self.Phi)
                self.e1 = add(scl(c, self.xhat), scl(s, self.yhat))
                self.e2 = sub(scl(s, self.xhat), scl(c, self.yhat))
            self.nhat = cross(self.e1, self.e2)
            self.p0 = self.pos(ZERO)

    # traversal parameter s in [0, length]
    def pos(self, s):
        if self.kind == "straight":
            return add(self.p0, scl(s, self.t0))
        psi = s / self.R
        return add(self.centre, scl(self.R, add(scl(cos(psi), self.e1), scl(sin(psi), self.e2))))

    def tan(self, s):
        if self.kind == "straight":
            return self.t0
        psi = s / self.R
        return add(scl(-sin(psi), self.e1), scl(cos(psi), self.e2))

    def er(self, s):
        psi = s / self.R
        return add(scl(cos(psi), self.e1), scl(sin(psi), self.e2))

    def start_point(self):
        return self.pos(ZERO)

    def end_point(self):
        return self.pos(self.length)

    def partial(self, s, P, wvec, wall=True):
        """Resultant (F, M about pos(s)) of the member's own distributed loads on [s, end]."""
        if self.kind == "straight":
            r = self.length - s
            F = scl(r, wvec)
            M = cross(scl(r / 2, self.t0), F)
            return F, M
        psi = s / self.R
        Dl = self.Phi - psi
        sp, cp = sin(psi), cos(psi)
        sP, cP = sin(self.Phi), cos(self.Phi)
        Er = add(scl(sP - sp, self.e1), scl(cp - cP, self.e2))
        F = VZERO
        M = VZERO
        if wall and P != 0:
            F = add(F, scl(P, Er))
            M = add(M, scl(-self.R * P * (1 - cos(Dl)), self.nhat))
        if wvec != VZERO:
            F = add(F, scl(self.R * Dl, wvec))
            erpsi = add(scl(cp, self.e1), scl(sp, self.e2))
            M = add(M, cross(scl(self.R * self.R, sub(Er, scl(Dl, erpsi))), wvec))
        return F, M

    def gl_points(self):
        n = GL_STRAIGHT if self.kind == "straight" else GL_ARC
        xs, ws = gauss_legendre(n)
        h = self.length / 2
        return [(h * (x + 1), w * h) for x, w in zip(xs, ws)]

    # authored helpers
    def authored_param(self, f):
        """Traversal parameter at authored fraction f."""
        f = D(f)
        return f * self.length if self.forward else (1 - f) * self.length

    def frame_at_authored(self, f):
        """Reporting frame (x, y, z) at authored fraction f."""
        if self.kind == "straight":
            return (self.ex, self.ey, self.ez)
        th = D(f) * self.Phi
        t = add(scl(-sin(th), self.xhat), scl(cos(th), self.yhat))
        inward = scl(-ONE, add(scl(cos(th), self.xhat), scl(sin(th), self.yhat)))
        return (t, inward, self.zhat)

    def authored_tangent(self, f):
        return self.frame_at_authored(f)[0]


# ---------------------------------------------------------------- chain model
class Chain:
    """Members in traversal order from node 0 (A, always anchored) to node M (D)."""

    def __init__(self, node_ids, coords, members):
        self.node_ids = node_ids
        self.coords = coords  # dict id -> vec
        self.members = members
        for m, mem in enumerate(members):
            a, b = node_ids[m], node_ids[m + 1]
            if (mem.i_id, mem.j_id) == (a, b):
                mem.set_forward(True)
            elif (mem.i_id, mem.j_id) == (b, a):
                mem.set_forward(False)
            else:
                raise ValueError("member does not connect consecutive nodes")
        self.xn = [coords[i] for i in node_ids]

    def trav_tangents(self):
        out = []
        for mem in self.members:
            out.append((mem.tan(ZERO), mem.tan(mem.length)))
        return out


class LoadCase:
    def __init__(self, p="0", dT="0", w="0", gdir=None, transfer_A=True, transfer_D=True,
                 anchor_D=False, arc_wall=True, arc_poisson=True, kink_forces=True,
                 extra_point_loads=None, remove_kink_at=None):
        self.p = D(p)
        self.dT = D(dT)
        self.w = D(w)
        self.gdir = gdir if gdir is not None else v(0, 0, -1)
        self.transfer_A = transfer_A
        self.transfer_D = transfer_D
        self.anchor_D = anchor_D
        self.arc_wall = arc_wall
        self.arc_poisson = arc_poisson
        self.kink_forces = kink_forces
        self.extra_point_loads = extra_point_loads or {}  # node index -> list of (F, M)
        self.remove_kink_at = set(remove_kink_at or [])


def member_P(mem, lc):
    return lc.p * mem.sec.Ai


def member_eps0(mem, lc):
    P = member_P(mem, lc)
    e = mem.mat.alpha * lc.dT
    if mem.kind == "straight" or lc.arc_poisson:
        e += -2 * mem.mat.nu * P / (mem.mat.E * mem.sec.As)
    return e


def point_loads(chain, lc):
    """Point loads (F, M) per node index in traversal order."""
    n_nodes = len(chain.node_ids)
    pts = {i: [] for i in range(n_nodes)}
    tans = chain.trav_tangents()
    mems = chain.members
    P0 = member_P(mems[0], lc)
    PM = member_P(mems[-1], lc)
    if lc.transfer_A and P0 != 0:
        pts[0].append((scl(-P0, tans[0][0]), VZERO))
    if lc.transfer_D and PM != 0:
        pts[n_nodes - 1].append((scl(PM, tans[-1][1]), VZERO))
    if lc.kink_forces:
        for m in range(len(mems) - 1):
            node = m + 1
            if node in lc.remove_kink_at:
                continue
            P = member_P(mems[m], lc)  # equal bore assumed
            kf = scl(P, sub(tans[m][1], tans[m + 1][0]))
            if kf != VZERO:
                pts[node].append((kf, VZERO))
    for node, lst in lc.extra_point_loads.items():
        pts[node].extend(lst)
    return pts


class Solution:
    pass


def analyse(chain, lc, disp_nodes=None):
    mems = chain.members
    n_nodes = len(chain.node_ids)
    wvec = scl(lc.w, lc.gdir)
    pts = point_loads(chain, lc)
    Ps = [member_P(m, lc) for m in mems]
    walls = [(m.kind == "arc" and lc.arc_wall) for m in mems]
    # full member loads about origin
    full = []
    for mi, mem in enumerate(mems):
        F, M = mem.partial(ZERO, Ps[mi], wvec, walls[mi])
        x0 = mem.start_point()
        full.append((F, add(M, cross(x0, F))))
    # suffix sums S[n] about origin: loads at nodes >= n and members starting at node >= n
    S = [None] * (n_nodes + 1)
    S[n_nodes] = (VZERO, VZERO)
    for n in range(n_nodes - 1, -1, -1):
        F, M = S[n + 1]
        for (f, mo) in pts[n]:
            F = add(F, f)
            M = add(M, add(cross(chain.xn[n], f), mo))
        if n < len(mems):
            F = add(F, full[n][0])
            M = add(M, full[n][1])
        S[n] = (F, M)

    def cut_loads(mi, s):
        mem = mems[mi]
        X = mem.pos(s)
        F, M = mem.partial(s, Ps[mi], wvec, walls[mi])
        Fs, Ms = S[mi + 1]
        F = add(F, Fs)
        M = add(M, sub(Ms, cross(X, Fs)))
        return X, F, M

    # GL points with precomputed data
    gp = []  # per member list of dicts
    for mi, mem in enumerate(mems):
        EA = mem.mat.E * mem.sec.As
        GJ = mem.mat.G * mem.sec.J
        EI = mem.mat.E * mem.sec.I
        e0 = member_eps0(mem, lc)
        lst = []
        for s, w in mem.gl_points():
            X, F, M = cut_loads(mi, s)
            lst.append(dict(s=s, w=w, X=X, t=mem.tan(s), F=F, M=M))
        gp.append((EA, GJ, EI, mem.k, e0, lst))

    def unit_action(node, a, X):
        if a < 3:
            e = EAX[a]
            return e, cross(sub(chain.xn[node], X), e)
        return VZERO, EAX[a - 3]

    def integ(node, a, F2=None, eig=True):
        """Integral of unit(node,a) against actual loads, or against unit F2=(node2,b)."""
        tot = ZERO
        for mi in range(node):
            EA, GJ, EI, k, e0, lst = gp[mi]
            for q in lst:
                Fu, Mu = unit_action(node, a, q["X"])
                if F2 is None:
                    F, M = q["F"], q["M"]
                else:
                    F, M = unit_action(F2[0], F2[1], q["X"])
                t = q["t"]
                Nu = dot(Fu, t)
                N = dot(F, t)
                Tu = dot(Mu, t)
                T = dot(M, t)
                val = Nu * N / EA + Tu * T / GJ + k * (dot(Mu, M) - Tu * T) / EI
                if F2 is None and eig:
                    val += Nu * e0
                tot += q["w"] * val
        return tot

    last = n_nodes - 1
    # redundants
    r = [ZERO] * 6
    if lc.anchor_D:
        C = [[integ(last, a, (last, b)) for b in range(6)] for a in range(6)]
        d0 = [integ(last, a) for a in range(6)]
        r = solve(C, [-x for x in d0])
    sol = Solution()
    sol.r_D = r
    fD = (r[0], r[1], r[2])
    mD = (r[3], r[4], r[5])
    xD = chain.xn[last]

    def total_cut(mi, s):
        X, F, M = cut_loads(mi, s)
        F = add(F, fD)
        M = add(M, add(cross(sub(xD, X), fD), mD))
        return X, F, M

    sol.total_cut = total_cut
    # displacements
    disp = []
    for n in range(n_nodes):
        if disp_nodes is not None and n not in disp_nodes:
            disp.append(None)
            continue
        dn = []
        for a in range(6):
            val = integ(n, a)
            if lc.anchor_D and n > 0:
                for b in range(6):
                    if r[b] != 0:
                        val += integ(n, a, (last, b)) * r[b]
            dn.append(val)
        disp.append(dn)
    sol.disp = disp
    # reactions
    X0, F0, M0 = total_cut(0, ZERO)
    capA = VZERO
    for (f, mo) in pts[0]:
        capA = add(capA, f)
    RA = scl(-ONE, add(F0, capA))
    MA = scl(-ONE, M0)
    sol.R_A = list(RA) + list(MA)
    sol.R_D = list(fD) + list(mD) if lc.anchor_D else None
    # global equilibrium check
    Ftot = add(RA, fD)
    Mtot = add(add(MA, mD), cross(sub(xD, chain.xn[0]), fD))
    for n in range(n_nodes):
        for (f, mo) in pts[n]:
            Ftot = add(Ftot, f)
            Mtot = add(Mtot, add(cross(sub(chain.xn[n], chain.xn[0]), f), mo))
    for mi, mem in enumerate(mems):
        F, M = mem.partial(ZERO, Ps[mi], wvec, walls[mi])
        Ftot = add(Ftot, F)
        Mtot = add(Mtot, add(M, cross(sub(mem.start_point(), chain.xn[0]), F)))
    sol.equilibrium_residual = (Ftot, Mtot)
    sol.points = pts
    sol.Ps = Ps
    sol.wvec = wvec
    return sol


def j_side_action(sol, chain, mi, f):
    """j-side section action (F, M about the cut point) at authored fraction f, global."""
    mem = chain.members[mi]
    s = mem.authored_param(f)
    X, F, M = sol.total_cut(mi, s)
    if not mem.forward:
        F = scl(-ONE, F)
        M = scl(-ONE, M)
    return X, F, M
