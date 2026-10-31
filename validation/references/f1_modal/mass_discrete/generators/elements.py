"""Element mass and stiffness for the F1 references, generic over a number field.

Conventions (all global, right-handed):
  - six DOFs per node in the order ux, uy, uz, rx, ry, rz; an element's
    12 DOFs are node i's six, then node j's six;
  - rotations are small right-handed rotation vectors; the velocity of a point
    at offset rho from a node is u_dot + theta_dot x rho.

Straight members (Euler-Bernoulli, DEL-04-09 Design section 2.2). The section
is a circular annulus, so the two bending planes are identical and the global
matrices are independent of the member's local y/z orientation. They are
formed directly in global form from the unit axis e with the projectors
P_a = e e^T, P_t = I - e e^T and S = [e]x. For a 4x4 plane matrix B in the
order (d_i, r_i, d_j, r_j), with transverse displacement d and slope r = -S
theta (local (v, theta_z) and (w, -theta_y)):
  block(u_a, u_b)         = A2[a][b] P_a + B[d_a][d_b] P_t
  block(u_a, theta_b)     = -B[d_a][r_b] S
  block(theta_a, u_b)     =  B[r_a][d_b] S
  block(theta_a, theta_b) = T2[a][b] P_a + B[r_a][r_b] P_t
with A2 the 2x2 axial and T2 the 2x2 torsional matrix. The textbook element
matrices are those of Przemieniecki, Theory of Matrix Structural Analysis
(1968), ch. 5 (stiffness) and ch. 11 (consistent mass, rotary inertia),
restated, and re-derived independently in crosscheck.py.

Realized arcs (DEC-070 curved macro-element): the 12x12 stiffness is formed
from the exact Castigliano tip flexibility of the arc clamped at node i, with
complementary energy density N^2/(EA) + T^2/(GJ) + k (|M|^2 - T^2)/(EI) (one
user flexibility factor k on both bending components, no shear deformation),
inverted and spread to both ends by rigid-body equilibrium (Design section 4.3).
The mass is two rigid half-arc lumps (Design section 2.4), each the exact
centreline line mass about its end node plus the section's own rotary inertia
(j_bar along the tangent, i_bar transverse).

Bending-plane sign map (Design section 2.2): in member axes the transverse
plane matrices act on [u_y, theta_z] (x-y plane) and [u_z, -theta_y] (x-z
plane); here d = (u_y, u_z) and r = -S theta = (theta_z, -theta_y) for a
member along x, which is exactly that map (checked in crosscheck.sign_map_matrix).
"""
from fractions import Fraction

HALF = Fraction(1, 2)


# --------------------------------------------------------------------------
# Small dense linear algebra over a field
# --------------------------------------------------------------------------
def zeros(F, r, c):
    z = F.q(0)
    return [[z for _ in range(c)] for _ in range(r)]


def identity(F, n):
    m = zeros(F, n, n)
    for i in range(n):
        m[i][i] = F.q(1)
    return m


def matmul(F, A, B):
    n, k, m = len(A), len(B), len(B[0])
    out = zeros(F, n, m)
    for i in range(n):
        Ai = A[i]
        row = out[i]
        for p in range(k):
            a = Ai[p]
            if F.is_zero(a):
                continue
            Bp = B[p]
            for j in range(m):
                b = Bp[j]
                if F.is_zero(b):
                    continue
                row[j] = row[j] + a * b
    return out


def transpose(A):
    return [list(r) for r in zip(*A)]


def matvec(F, A, x):
    out = []
    for row in A:
        s = F.q(0)
        for a, b in zip(row, x):
            if F.is_zero(a) or F.is_zero(b):
                continue
            s = s + a * b
        out.append(s)
    return out


def dot(F, x, y):
    s = F.q(0)
    for a, b in zip(x, y):
        s = s + a * b
    return s


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def skew(F, v):
    z = F.q(0)
    return [[z, -v[2], v[1]], [v[2], z, -v[0]], [-v[1], v[0], z]]


def invert(F, A):
    """Gauss-Jordan with the largest-magnitude pivot (point/exact); SPD input."""
    n = len(A)
    a = [list(r) + [F.q(1) if i == j else F.q(0) for j in range(n)] for i, r in enumerate(A)]
    for k in range(n):
        piv = k
        if F.name == "point":
            piv = max(range(k, n), key=lambda i: abs(a[i][k]))
        elif F.name == "interval":
            piv = max(range(k, n), key=lambda i: min(abs(a[i][k].lo), abs(a[i][k].hi)))
        a[k], a[piv] = a[piv], a[k]
        p = a[k][k]
        a[k] = [x / p for x in a[k]]
        for i in range(n):
            if i != k and not F.is_zero(a[i][k]):
                f = a[i][k]
                a[i] = [x - f * y for x, y in zip(a[i], a[k])]
    return [r[n:] for r in a]


# --------------------------------------------------------------------------
# Straight member
# --------------------------------------------------------------------------
def frame_global(F, e, A2, T2, B):
    """12x12 global matrix from axial A2, torsional T2 and plane matrix B."""
    one, zero = F.q(1), F.q(0)
    Pa = [[e[r] * e[c] for c in range(3)] for r in range(3)]
    Pt = [[(one if r == c else zero) - Pa[r][c] for c in range(3)] for r in range(3)]
    S = skew(F, e)
    K = zeros(F, 12, 12)

    def put(r0, c0, blk):
        for r in range(3):
            for c in range(3):
                K[r0 + r][c0 + c] = K[r0 + r][c0 + c] + blk[r][c]

    def comb(alpha, X, beta=None, Y=None):
        out = [[alpha * X[r][c] for c in range(3)] for r in range(3)]
        if Y is not None:
            out = [[out[r][c] + beta * Y[r][c] for c in range(3)] for r in range(3)]
        return out

    for a in range(2):
        for b in range(2):
            ua, ta, ub, tb = 6 * a, 6 * a + 3, 6 * b, 6 * b + 3
            put(ua, ub, comb(A2[a][b], Pa, B[2 * a][2 * b], Pt))
            put(ua, tb, comb(-B[2 * a][2 * b + 1], S))
            put(ta, ub, comb(B[2 * a + 1][2 * b], S))
            put(ta, tb, comb(T2[a][b], Pa, B[2 * a + 1][2 * b + 1], Pt))
    return K


def _power(F, L, p):
    return F.q(1) if p == 0 else (L if p == 1 else L * L)


def plane(F, L, coeffs, scale):
    """scale * [[c0, c1 L, c2, c3 L], ...] with entries c * L**p as given."""
    out = []
    for row in coeffs:
        out.append([scale * F.q(c) * _power(F, L, p) for c, p in row])
    return out


MASS_PLANE = [
    [(156, 0), (22, 1), (54, 0), (-13, 1)],
    [(22, 1), (4, 2), (13, 1), (-3, 2)],
    [(54, 0), (13, 1), (156, 0), (-22, 1)],
    [(-13, 1), (-3, 2), (-22, 1), (4, 2)],
]
ROTARY_PLANE = [
    [(36, 0), (3, 1), (-36, 0), (3, 1)],
    [(3, 1), (4, 2), (-3, 1), (-1, 2)],
    [(-36, 0), (-3, 1), (36, 0), (-3, 1)],
    [(3, 1), (-1, 2), (-3, 1), (4, 2)],
]
STIFF_PLANE = [
    [(12, 0), (6, 1), (-12, 0), (6, 1)],
    [(6, 1), (4, 2), (-6, 1), (2, 2)],
    [(-12, 0), (-6, 1), (12, 0), (-6, 1)],
    [(6, 1), (2, 2), (-6, 1), (4, 2)],
]


def straight_mass(F, e, L, mbar, ibar, jbar):
    """Consistent Hermitian translation mass, Rayleigh rotary, linear axial and torsional."""
    Bm = plane(F, L, MASS_PLANE, mbar * L / F.q(420))
    Br = plane(F, L, ROTARY_PLANE, ibar / (F.q(30) * L))
    B = [[Bm[r][c] + Br[r][c] for c in range(4)] for r in range(4)]
    s6 = mbar * L / F.q(6)
    A2 = [[F.q(2) * s6, s6], [s6, F.q(2) * s6]]
    t6 = jbar * L / F.q(6)
    T2 = [[F.q(2) * t6, t6], [t6, F.q(2) * t6]]
    return frame_global(F, e, A2, T2, B)


def straight_stiffness(F, e, L, EA, EI, GJ):
    B = plane(F, L, STIFF_PLANE, EI / (L * L * L))
    a = EA / L
    t = GJ / L
    return frame_global(F, e, [[a, -a], [-a, a]], [[t, -t], [-t, t]], B)


# --------------------------------------------------------------------------
# Rigid-body lump (component body, half-arc)
# --------------------------------------------------------------------------
def rigid_lump(F, m, S, J):
    """6x6 about a node: mass m, first moment S = m c (3), second moment J (3x3).

    Kinetic energy of v = u_dot + theta_dot x rho: M_uu = m I,
    M_u_theta = -[S]x, M_theta_u = [S]x, M_theta_theta = J.
    """
    Sx = skew(F, S)
    out = zeros(F, 6, 6)
    for r in range(3):
        out[r][r] = m
        for c in range(3):
            out[r][3 + c] = -Sx[r][c]
            out[3 + r][c] = Sx[r][c]
            out[3 + r][3 + c] = J[r][c]
    return out


def point_mass_lump(F, m, r):
    """Point mass m at offset r from the node: J = m [r]x^T [r]x = m (|r|^2 I - r r^T)."""
    rr = r[0] * r[0] + r[1] * r[1] + r[2] * r[2]
    J = [[m * ((rr if a == b else F.q(0)) - r[a] * r[b]) for b in range(3)] for a in range(3)]
    return rigid_lump(F, m, [m * x for x in r], J)


# --------------------------------------------------------------------------
# Trigonometric polynomials sum_k a_k cos(k psi) + b_k sin(k psi)
# --------------------------------------------------------------------------
class TrigPoly:
    __slots__ = ("F", "c")

    def __init__(self, F, c=None):
        self.F = F
        self.c = c or {}

    @staticmethod
    def const(F, v):
        return TrigPoly(F, {0: [v, F.q(0)]})

    def _get(self, k):
        return self.c.get(k, [self.F.q(0), self.F.q(0)])

    def __add__(self, o):
        F = self.F
        out = {k: list(v) for k, v in self.c.items()}
        for k, (a, b) in o.c.items():
            cur = out.get(k, [F.q(0), F.q(0)])
            out[k] = [cur[0] + a, cur[1] + b]
        return TrigPoly(F, out)

    def __neg__(self):
        return TrigPoly(self.F, {k: [-a, -b] for k, (a, b) in self.c.items()})

    def __sub__(self, o):
        return self + (-o)

    def scale(self, s):
        return TrigPoly(self.F, {k: [s * a, s * b] for k, (a, b) in self.c.items()})

    def __mul__(self, o):
        F = self.F
        if not isinstance(o, TrigPoly):
            return self.scale(o)
        h = F.q(HALF)
        out = {}

        def add(k, a, b):
            if k < 0:
                k, b = -k, -b
            cur = out.get(k)
            if cur is None:
                out[k] = [a, b]
            else:
                out[k] = [cur[0] + a, cur[1] + b]

        z = F.q(0)
        for m, (am, bm) in self.c.items():
            for n, (an, bn) in o.c.items():
                if not F.is_zero(am) and not F.is_zero(an):
                    p = h * am * an
                    add(m - n, p, z)
                    add(m + n, p, z)
                if not F.is_zero(bm) and not F.is_zero(bn):
                    p = h * bm * bn
                    add(m - n, p, z)
                    add(m + n, -p, z)
                if not F.is_zero(bm) and not F.is_zero(an):
                    p = h * bm * an  # sin m cos n
                    add(m + n, z, p)
                    add(m - n, z, p)
                if not F.is_zero(am) and not F.is_zero(bn):
                    p = h * am * bn  # cos m sin n
                    add(m + n, z, p)
                    add(m - n, z, -p)
        return TrigPoly(F, out)

    def integrate(self, table, m_lo, m_hi):
        """Integral over psi in [m_lo*phi/2, m_hi*phi/2]; table holds angles and trig values."""
        F = self.F
        total = F.q(0)
        for k, (a, b) in self.c.items():
            if k == 0:
                if not F.is_zero(a):
                    total = total + a * (table.angle(m_hi) - table.angle(m_lo))
                continue
            kk = F.q(k)
            if not F.is_zero(a):
                total = total + a * (table.sin(k * m_hi) - table.sin(k * m_lo)) / kk
            if not F.is_zero(b):
                total = total + b * (table.cos(k * m_lo) - table.cos(k * m_hi)) / kk
        return total


class HalfAngleTable:
    """cos and sin of j*phi/2 for j = 0..16, by angle addition from cos/sin(phi/2)."""

    def __init__(self, F, half_phi, ch, sh, n=16):
        self.F = F
        self.half_phi = half_phi
        self.cs = [(F.q(1), F.q(0))]
        for _ in range(n):
            c, s = self.cs[-1]
            self.cs.append((c * ch - s * sh, s * ch + c * sh))

    def cos(self, j):
        return self.cs[j][0]

    def sin(self, j):
        return self.cs[j][1]

    def angle(self, j):
        return self.F.q(j) * self.half_phi


# --------------------------------------------------------------------------
# Realized arc
# --------------------------------------------------------------------------
class ArcGeometry:
    """Arc from its end nodes, radius and bow direction (y_reference).

    The arc bows toward y_reference projected normal to the chord; its centre is
    on the other side of the chord. Local frame: e1 from the centre to node i,
    e2 in the bend plane toward node j, e3 = e1 x e2 (the bend-plane normal
    (x_i - c) x (x_j - c), normalised). Lambda has rows e1, e2, e3, so
    v_local = Lambda v_global.
    """

    def __init__(self, F, xi, xj, R, yref):
        self.F = F
        self.xi, self.xj, self.R = xi, xj, R
        d = [xj[k] - xi[k] for k in range(3)]
        L2 = dot(F, d, d)
        L = F.sqrt(L2)
        xc = [v / L for v in d]
        yr = [F.q(v) for v in yref]
        p = dot(F, yr, xc)
        y = [yr[k] - p * xc[k] for k in range(3)]
        ny = F.sqrt(dot(F, y, y))
        y = [v / ny for v in y]
        h = F.sqrt(R * R - L2 / F.q(4))
        self.centre = [(xi[k] + xj[k]) / F.q(2) - h * y[k] for k in range(3)]
        e1 = [(xi[k] - self.centre[k]) / R for k in range(3)]
        ej = [(xj[k] - self.centre[k]) / R for k in range(3)]
        cphi = dot(F, e1, ej)
        w = [ej[k] - cphi * e1[k] for k in range(3)]
        sphi = F.sqrt(dot(F, w, w))
        e2 = [v / sphi for v in w]
        e3 = cross(e1, e2)
        self.Lambda = [e1, e2, e3]
        self.cos_phi, self.sin_phi = cphi, sphi
        self.phi = F.q(2) * F.atan(sphi / (F.q(1) + cphi))
        ch = F.sqrt((F.q(1) + cphi) / F.q(2))
        sh = F.sqrt((F.q(1) - cphi) / F.q(2))
        self.table = HalfAngleTable(F, self.phi / F.q(2), ch, sh)
        self.chord = L

    def to_global_vec(self, v):
        F = self.F
        LT = transpose(self.Lambda)
        return matvec(F, LT, v)

    def to_global_mat(self, A):
        F = self.F
        LT = transpose(self.Lambda)
        return matmul(F, matmul(F, LT, A), self.Lambda)


def _arc_position(F, R):
    """Local position x(psi) = R (cos psi, sin psi, 0) and tangent t(psi)."""
    z = TrigPoly(F, {})
    x = [TrigPoly(F, {1: [R, F.q(0)]}), TrigPoly(F, {1: [F.q(0), R]}), z]
    t = [TrigPoly(F, {1: [F.q(0), F.q(-1)]}), TrigPoly(F, {1: [F.q(1), F.q(0)]}), z]
    return x, t


def arc_tip_flexibility_local(F, geo, EA, EI, GJ, k):
    """6x6 tip flexibility at node j, node i clamped, local arc frame."""
    R = geo.R
    x, t = _arc_position(F, R)
    xj = [R * geo.cos_phi, R * geo.sin_phi, F.q(0)]
    a = [TrigPoly.const(F, xj[r]) - x[r] for r in range(3)]
    zero = TrigPoly(F, {})
    one = TrigPoly.const(F, F.q(1))
    # M_s = [a]x F + M  -> B_M rows (3 x 6)
    ax = [[zero, -a[2], a[1]], [a[2], zero, -a[0]], [-a[1], a[0], zero]]
    BM = [[ax[r][c] for c in range(3)] + [one if r == c else zero for c in range(3)] for r in range(3)]
    BN = [t[0], t[1], t[2], zero, zero, zero]
    BT = []
    for c in range(6):
        s = zero
        for r in range(3):
            s = s + t[r] * BM[r][c]
        BT.append(s)
    cN = F.q(1) / EA
    cB = k / EI
    cT = F.q(1) / GJ - cB
    f = zeros(F, 6, 6)
    for p in range(6):
        for q in range(p, 6):
            integrand = (BN[p] * BN[q]).scale(cN) + (BT[p] * BT[q]).scale(cT)
            bend = zero
            for r in range(3):
                bend = bend + BM[r][p] * BM[r][q]
            integrand = integrand + bend.scale(cB)
            v = R * integrand.integrate(geo.table, 0, 2)
            f[p][q] = v
            f[q][p] = v
    return f


def arc_stiffness(F, geo, EA, EI, GJ, k):
    """12x12 global stiffness of the arc (exact for the stated energy)."""
    f = arc_tip_flexibility_local(F, geo, EA, EI, GJ, k)
    Kl = invert(F, f)
    L6 = zeros(F, 6, 6)
    for r in range(3):
        for c in range(3):
            L6[r][c] = geo.Lambda[r][c]
            L6[3 + r][3 + c] = geo.Lambda[r][c]
    Kg = matmul(F, matmul(F, transpose(L6), Kl), L6)
    return spread_tip_stiffness(F, Kg, [geo.xj[r] - geo.xi[r] for r in range(3)])


def spread_tip_stiffness(F, Kjj, delta):
    """K = B^T Kjj B, B = [-Gamma, I], Gamma = [[I, -[delta]x], [0, I]].

    The deformation at j relative to the rigid-body motion carried from node i
    is d_j - Gamma d_i, since u_j_rigid = u_i + theta_i x delta.
    """
    Gamma = identity(F, 6)
    dx = skew(F, delta)
    for r in range(3):
        for c in range(3):
            Gamma[r][3 + c] = -dx[r][c]
    Bm = zeros(F, 6, 12)
    for r in range(6):
        for c in range(6):
            Bm[r][c] = -Gamma[r][c]
        Bm[r][6 + r] = F.q(1)
    return matmul(F, matmul(F, transpose(Bm), Kjj), Bm)


def half_arc_unit_lumps(F, geo):
    """Half-arc lumps for unit mass per length, in global axes.

    Returns [(node_side, m, S_global, J_global)] for the half from node i
    (psi in [0, phi/2]) and the half from node j (psi in [phi/2, phi]).
    """
    R = geo.R
    x, _ = _arc_position(F, R)
    ends = [("i", 0, 1, [R, F.q(0), F.q(0)]),
            ("j", 1, 2, [R * geo.cos_phi, R * geo.sin_phi, F.q(0)])]
    out = []
    for side, m_lo, m_hi, P in ends:
        rho = [x[r] - TrigPoly.const(F, P[r]) for r in range(3)]
        mass = R * (geo.table.angle(m_hi) - geo.table.angle(m_lo))
        S = [R * rho[r].integrate(geo.table, m_lo, m_hi) for r in range(3)]
        rr = rho[0] * rho[0] + rho[1] * rho[1] + rho[2] * rho[2]
        J = zeros(F, 3, 3)
        for a in range(3):
            for b in range(a, 3):
                integrand = (rr if a == b else TrigPoly(F, {})) - rho[a] * rho[b]
                v = R * integrand.integrate(geo.table, m_lo, m_hi)
                J[a][b] = v
                J[b][a] = v
        out.append((side, mass, geo.to_global_vec(S), geo.to_global_mat(J)))
    return out


def half_arc_section_inertia(F, geo, jbar, ibar):
    """Section rotary inertia of each half-arc, added to its lump (Design section 2.4).

    integral of T diag(jbar, ibar, ibar) T^T ds = integral of (jbar t t^T + ibar (I - t t^T)) R dpsi
    over each half, with T the local frame (tangent first); global axes.
    """
    R = geo.R
    _, t = _arc_position(F, R)
    out = []
    for side, m_lo, m_hi in (("i", 0, 1), ("j", 1, 2)):
        J = zeros(F, 3, 3)
        length = R * (geo.table.angle(m_hi) - geo.table.angle(m_lo))
        for a in range(3):
            for b in range(3):
                tt = R * (t[a] * t[b]).integrate(geo.table, m_lo, m_hi)
                J[a][b] = jbar * tt + ibar * ((length if a == b else F.q(0)) - tt)
        out.append((side, geo.to_global_mat(J)))
    return out
