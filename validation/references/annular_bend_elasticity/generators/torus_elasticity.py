"""Linear three-dimensional elasticity of a curved bar of constant section whose centreline is a
circle (a toroidal segment), for its six interior (Saint-Venant) states. It imports the Python
standard library, numpy and scipy.sparse, and no product code.

Method (semi-inverse, semi-analytical). Cylindrical coordinates (r, theta, z) about the axis of
the torus; the section is the (r, z) region at a fixed theta. Every interior state of a curved
bar has stresses independent of theta (n = 0) or varying as cos/sin theta (n = 1). Its
displacement is a periodic field of the same harmonic plus theta times a rigid motion w (a
Volterra dislocation of the closed torus), whose strain sym(e_theta (x) w)/r is again harmonic.
For each such rigid motion, prescribed with unit amplitude, the periodic field minimizes the
strain energy: a two-dimensional finite-element problem on the section (isoparametric 9-node
quadrilaterals, 4x4 Gauss). The four problems give six local states at a section:

  n = 0, w = rotation about z      -> pure in-plane moment
  n = 0, w = translation along z   -> out-of-plane force through the torus axis (with torque R*Fz)
  n = 1, w = translation along x   -> in-plane force through the centre (two states at a section:
                                      its cos part carries V_r, its sin part N and M_n)
  n = 1, w = rotation about y      -> out-of-plane moment about an in-plane axis (cos part: V_z, T;
                                      sin part: M_r)

At a section, with resultants S_k (N, V_r, V_n, T, M_r, M_n about the centroid, positive face
normal e_theta) and the cross-energy matrix E_kl = int sigma_k : eps_l r dA / R (per unit length of
the centroidal line, radius R), the section compliance per unit arc length is
C = S^-T E S^-1. This compliance includes everything three-dimensional elasticity holds in the
interior: shear and its distribution, curvature, Poisson effects and free ovalization of the
section. It excludes end effects (restraint of ovalization by tangents or flanges).

Conditioning. The six rigid motions of the section in its own plane and out of it (SECTION_MODES)
strain the torus only through its curvature, by O(1/R). In each harmonic two of them are exact null
motions of the problem (NULL_MODES) and the others are nearly so as R grows, so a stiffness matrix of
the whole periodic field is ill-conditioned (relative eigenvalues near 1e-11 at R = 1000 D), and the
amplitudes of those motions, on which the resultants depend, were left to roundoff. The periodic field
is therefore split: U = Q a + V, with Q the section motions other than the exact null motions, whose
strains are formed in closed form (`mode_strain`), and V orthogonal to all six. V is found from a
saddle-point system that enforces the orthogonality (no node is pinned); the amplitudes a from the
small Schur complement. The exact null motions carry no strain and are left out.
"""
from __future__ import annotations

import math

import numpy as np
import scipy.sparse as sp
import scipy.sparse.linalg as spla

# ----------------------------------------------------------------- Q9 element
_g, _w = np.polynomial.legendre.leggauss(4)
GP = np.array([(a, b) for a in _g for b in _g])
GW = np.array([wa * wb for wa in _w for wb in _w])


def _l1(x):
    return np.array([x * (x - 1) / 2, 1 - x * x, x * (x + 1) / 2])


def _d1(x):
    return np.array([x - 0.5, -2 * x, x + 0.5])


# local node order: (i, j) with i along xi, j along eta, index = j*3 + i
def q9(xi, eta):
    Nx, Ny, dx, dy = _l1(xi), _l1(eta), _d1(xi), _d1(eta)
    N = np.outer(Ny, Nx).ravel()
    dNx = np.outer(Ny, dx).ravel()
    dNy = np.outer(dy, Nx).ravel()
    return N, dNx, dNy


_Q9 = [q9(a, b) for a, b in GP]


# ----------------------------------------------------------------- meshes
def annulus_mesh(R, ri, ro, n_around, n_through):
    """Section of an annulus centred at (R, 0) in (r, z). Node grid periodic in psi."""
    nps = 2 * n_around
    nrh = 2 * n_through + 1
    rho = np.linspace(ri, ro, nrh)
    psi = np.arange(nps) * 2 * math.pi / nps
    P, Rh = np.meshgrid(psi, rho)  # [rho index, psi index]
    r = R + Rh * np.cos(P)
    z = Rh * np.sin(P)
    nid = lambda a, b: a * nps + (b % nps)
    conn = []
    for et in range(n_through):
        for ea in range(n_around):
            conn.append([nid(2 * et + j, 2 * ea + i) for j in range(3) for i in range(3)])
    return np.column_stack([r.ravel(), z.ravel()]), np.array(conn)


def rect_mesh(R, h, b, nr, nz):
    rr = np.linspace(R - h / 2, R + h / 2, 2 * nr + 1)
    zz = np.linspace(-b / 2, b / 2, 2 * nz + 1)
    Z, Rr = np.meshgrid(zz, rr)
    nid = lambda a, c: a * len(zz) + c
    conn = []
    for er in range(nr):
        for ez in range(nz):
            # xi along r, eta along z
            conn.append([nid(2 * er + i, 2 * ez + j) for j in range(3) for i in range(3)])
    return np.column_stack([Rr.ravel(), Z.ravel()]), np.array(conn)


# ----------------------------------------------------------------- material
def dmat(E, nu):
    lam = E * nu / ((1 + nu) * (1 - 2 * nu))
    mu = E / (2 * (1 + nu))
    D = np.zeros((6, 6))
    D[:3, :3] = lam
    D[0, 0] = D[1, 1] = D[2, 2] = lam + 2 * mu
    D[3, 3] = D[4, 4] = D[5, 5] = mu
    return D  # order: e_rr, e_tt, e_zz, g_rt, g_tz, g_rz


# ----------------------------------------------------------------- strains
def bmat(cls, N, Nr, Nz, r):
    """6 x 27 strain-displacement amplitudes, dof order per node (U_r, U_t, U_z)."""
    n = len(N)
    B = np.zeros((6, 3 * n))
    ur, ut, uz = slice(0, 3 * n, 3), slice(1, 3 * n, 3), slice(2, 3 * n, 3)
    B[0, ur] = Nr
    B[2, uz] = Nz
    B[5, ur] = Nz
    B[5, uz] = Nr
    if cls == 0:
        B[1, ur] = N / r
        B[3, ut] = Nr - N / r
        B[4, ut] = Nz
    else:  # n = 1, class S: U_r sin, U_t cos, U_z sin
        B[1, ur] = N / r
        B[1, ut] = -N / r
        B[3, ur] = N / r
        B[3, ut] = Nr - N / r
        B[4, ut] = Nz
        B[4, uz] = N / r
    return B


def secular(cls, r, z):
    """Strain amplitudes of the two prescribed dislocations of the class (columns)."""
    s = np.zeros((6, 2))
    if cls == 0:
        s[1, 0] = 1.0          # rotation about z: e_tt = 1
        s[4, 1] = 1.0 / r      # translation along z: g_tz = 1/r
    else:
        s[1, 0] = -1.0 / r     # translation along x
        s[3, 0] = 1.0 / r
        s[1, 1] = -z / r       # rotation about y
        s[3, 1] = z / r
        s[4, 1] = -1.0
    return s


# normal-type strain components (sin part of class S) and shear-type (cos part)
NORMAL = [0, 1, 2, 5]
SHEAR = [3, 4]


# Rigid motions of the section, as (U_r, U_t, U_z) fields of rho = r - R and z, with their gradients:
# in-plane translations along r and z and rotation, out-of-plane translation and rotations about z and r.
SECTION_MODES = (
    ("translation r", lambda rho, z: (1.0, 0.0, 0.0), ((0, 0), (0, 0), (0, 0))),
    ("translation z", lambda rho, z: (0.0, 0.0, 1.0), ((0, 0), (0, 0), (0, 0))),
    ("rotation in plane", lambda rho, z: (-z, 0.0, rho), ((0, -1), (0, 0), (1, 0))),
    ("translation t", lambda rho, z: (0.0, 1.0, 0.0), ((0, 0), (0, 0), (0, 0))),
    ("rotation about z", lambda rho, z: (0.0, rho, 0.0), ((0, 0), (1, 0), (0, 0))),
    ("rotation about r", lambda rho, z: (0.0, z, 0.0), ((0, 0), (0, 1), (0, 0))),
)
# Per harmonic: the combinations of SECTION_MODES (coefficients over the six, with R) that strain nothing
# (n = 0: translation along the torus axis, rotation about it; n = 1: translation along x, rotation about
# y), and the four section motions kept as amplitudes, which with the null motions span all six.
NULL_MODES = {0: (lambda R: (0, 1, 0, 0, 0, 0), lambda R: (0, 0, 0, R, 1, 0)),
              1: (lambda R: (1, 0, 0, 1, 0, 0), lambda R: (0, -R, -1, 0, 0, 1))}
KEPT_MODES = {0: (0, 2, 3, 5), 1: (0, 1, 2, 4)}
REFINE = 3
# A deliberate perturbation of the assembled data, for the check's estimate of each figure's sensitivity to
# roundoff: None, or (seed, relative size). Each element matrix and load is multiplied entrywise by
# 1 + size * u, u uniform in [-1, 1] (symmetric for the matrices), from numpy's platform-independent generator.
PERTURBATION = None


def _noise(rng, shape, size, symmetric=False):
    u = rng.uniform(-1.0, 1.0, shape)
    if symmetric:
        u = 0.5 * (u + np.swapaxes(u, -1, -2))
    return 1.0 + size * u


def mode_strain(cls, k, r, z, R):
    """Strain amplitudes (6) of section mode k in harmonic cls at (r, z), in closed form."""
    fr, ft, fz = SECTION_MODES[k][1](r - R, z)
    (dr_r, dz_r), (dr_t, dz_t), (dr_z, dz_z) = SECTION_MODES[k][2]
    e = np.zeros(6)
    e[0] = dr_r
    e[2] = dz_z
    e[5] = dz_r + dr_z
    if cls == 0:
        e[1] = fr / r
        e[3] = dr_t - ft / r
        e[4] = dz_t
    else:
        e[1] = fr / r - ft / r
        e[3] = fr / r + dr_t - ft / r
        e[4] = dz_t + fz / r
    return e


class Torus:
    def __init__(self, nodes, conn, R, E, nu):
        self.X, self.conn, self.R, self.E, self.nu = nodes, conn, R, E, nu
        self.D = dmat(E, nu)
        self._geom()

    def _geom(self):
        """Gauss-point data: N, dN/dr, dN/dz, r, z, weight (dA)."""
        self.gp = []
        for el in self.conn:
            xe = self.X[el]
            for (N, dx, dy), w in zip(_Q9, GW):
                J = np.array([[dx @ xe[:, 0], dx @ xe[:, 1]], [dy @ xe[:, 0], dy @ xe[:, 1]]])
                det = np.linalg.det(J)
                inv = np.linalg.inv(J)
                Nr = inv[0, 0] * dx + inv[0, 1] * dy
                Nz = inv[1, 0] * dx + inv[1, 1] * dy
                r, z = N @ xe[:, 0], N @ xe[:, 1]
                self.gp.append((el, N, Nr, Nz, r, z, w * abs(det)))
        self.area = sum(g[6] for g in self.gp)
        self.rc = sum(g[4] * g[6] for g in self.gp) / self.area

    def nodal_modes(self, cls):
        """Nodal values (3 n x 6) of the six section modes, and the kept modes' indices."""
        rho, z = self.X[:, 0] - self.R, self.X[:, 1]
        Q = np.zeros((3 * len(self.X), 6))
        for k, (_, f, _) in enumerate(SECTION_MODES):
            fr, ft, fz = f(rho, z)
            Q[0::3, k], Q[1::3, k], Q[2::3, k] = fr, ft, fz
        return Q

    def solve_class(self, cls):
        """The periodic field of harmonic cls for its two prescribed dislocations: (V, a), with the field
        U = Q[:, kept] a + V (see Conditioning in the module docstring)."""
        n = 3 * len(self.X)
        kept = KEPT_MODES[cls]
        m = len(kept)
        rows, cols, vals = [], [], []
        F = np.zeros((n, 2))
        KQ = np.zeros((n, m))
        Kqq = np.zeros((m, m))
        Fq = np.zeros((m, 2))
        D = self.D
        if PERTURBATION is not None:
            rng = np.random.default_rng([PERTURBATION[0], cls])
            ng, size = len(self.gp), PERTURBATION[1]
            noise = (_noise(rng, (ng, 27, 27), size, True), _noise(rng, (ng, m, m), size, True),
                     _noise(rng, (ng, 27, 2), size), _noise(rng, (ng, 27, m), size), _noise(rng, (ng, m, 2), size))
        for g, (el, N, Nr, Nz, r, z, dA) in enumerate(self.gp):
            B = bmat(cls, N, Nr, Nz, r)
            dofs = (3 * el[:, None] + np.arange(3)).ravel()
            w = r * dA
            ke = w * (B.T @ D @ B)
            sec = secular(cls, r, z)
            fe = -w * (B.T @ D @ sec)
            Eq = np.column_stack([mode_strain(cls, k, r, z, self.R) for k in kept])
            DEq = D @ Eq
            kq, qq, fq = w * (B.T @ DEq), w * (Eq.T @ DEq), -w * (DEq.T @ sec)
            if PERTURBATION is not None:
                ke, qq, fe, kq, fq = (x * u[g] for x, u in zip((ke, qq, fe, kq, fq), noise))
            rows.append(np.repeat(dofs, len(dofs)))
            cols.append(np.tile(dofs, len(dofs)))
            vals.append(ke.ravel())
            F[dofs] += fe
            KQ[dofs] += kq
            Kqq += qq
            Fq += fq
        K = sp.csr_matrix((np.concatenate(vals), (np.concatenate(rows), np.concatenate(cols))), shape=(n, n))
        # V orthogonal to all six section modes, by a saddle-point system (no node pinned)
        Q = self.nodal_modes(cls)
        Cq = Q / np.linalg.norm(Q, axis=0) * float(np.mean(np.abs(K.diagonal())))
        A = sp.bmat([[K, sp.csr_matrix(Cq)], [sp.csr_matrix(Cq.T), None]], format="csc")
        rhs = np.zeros((n + 6, 2 + m))
        rhs[:n, :2] = F
        rhs[:n, 2:] = KQ
        lu = spla.splu(A)
        Y = lu.solve(rhs)
        for _ in range(REFINE):  # iterative refinement: the backward error to roundoff, whatever the ordering
            Y += lu.solve(rhs - A @ Y)
        Y = Y[:n]
        a = np.linalg.solve(Kqq - KQ.T @ Y[:, 2:], Fq - KQ.T @ Y[:, :2])
        V = Y[:, :2] - Y[:, 2:] @ a
        self.residual = getattr(self, "residual", {})
        Kv = K @ V + KQ @ a - F  # stationarity: orthogonal to V's space and to the kept modes
        self.residual[cls] = float(np.linalg.norm(Kv - Q @ np.linalg.lstsq(Q, Kv, rcond=None)[0]) / np.linalg.norm(F))
        return V, a

    def strain(self, cls, sol, g):
        """Strain amplitudes (6 x 2) of harmonic cls's two states at Gauss point g."""
        V, a = sol
        el, N, Nr, Nz, r, z, dA = g
        dofs = (3 * el[:, None] + np.arange(3)).ravel()
        Eq = np.column_stack([mode_strain(cls, k, r, z, self.R) for k in KEPT_MODES[cls]])
        return bmat(cls, N, Nr, Nz, r) @ V[dofs] + Eq @ a + secular(cls, r, z)

    def states(self):
        """Stress and strain at a section for the six local states; returns (S, E) with
        S[:, k] the resultants of state k and E the cross-energy per unit centroidal length."""
        sol0 = self.solve_class(0)
        sol1 = self.solve_class(1)
        R = self.R
        D = self.D
        # strain vectors per state at each Gauss point
        sig = [[] for _ in range(6)]
        eps = [[] for _ in range(6)]
        for g in self.gp:
            e0 = self.strain(0, sol0, g)
            e1 = self.strain(1, sol1, g)
            shear_part = e1.copy()
            shear_part[NORMAL] = 0.0
            normal_part = e1.copy()
            normal_part[SHEAR] = 0.0
            local = [e0[:, 0], e0[:, 1], shear_part[:, 0], shear_part[:, 1], normal_part[:, 0], normal_part[:, 1]]
            for k, e in enumerate(local):
                eps[k].append(e)
                sig[k].append(D @ e)
        r = np.array([g[4] for g in self.gp])
        z = np.array([g[5] for g in self.gp])
        dA = np.array([g[6] for g in self.gp])
        S = np.zeros((6, 6))
        Em = np.zeros((6, 6))
        for k in range(6):
            s = np.array(sig[k])
            # traction on the theta face: (tau_rt, sigma_tt, tau_tz)
            st, trt, ttz = s[:, 1], s[:, 3], s[:, 4]
            S[0, k] = np.sum(st * dA)
            S[1, k] = np.sum(trt * dA)
            S[2, k] = np.sum(ttz * dA)
            S[3, k] = np.sum((z * trt - (r - R) * ttz) * dA)
            S[4, k] = -np.sum(z * st * dA)
            S[5, k] = np.sum((r - R) * st * dA)
            for l in range(6):
                Em[k, l] = np.sum(np.einsum("ij,ij->i", s, np.array(eps[l])) * r * dA) / R
        return S, 0.5 * (Em + Em.T)

    def compliance(self):
        """C = S^-T E S^-1; keeps S and E (self.S, self.Em) for the cut-work and reciprocity checks."""
        S, Em = self.states()
        self.S, self.Em = S, Em
        Si = np.linalg.inv(S)
        C = Si.T @ Em @ Si
        return 0.5 * (C + C.T)


def annulus_compliance(R, D, t, E=2.0e11, nu=0.3, n_around=96, n_through=2):
    """The interior section compliance of an annular curved bar (outside diameter D, wall t, centroid radius
    R) on a mesh of n_around Q9 elements around and n_through through the wall; returns (C, the Torus)."""
    ro, ri = D / 2, D / 2 - t
    X, conn = annulus_mesh(R, ri, ro, n_around, n_through)
    tor = Torus(X, conn, R, E, nu)
    return tor.compliance(), tor


def beam_reference(R, D, t, E=2.0e11, nu=0.3):
    """Straight-beam section compliance diag with DEL-05-03's kappa_E (for the R -> infinity check)."""
    ro, ri = D / 2, D / 2 - t
    A = math.pi * (ro ** 2 - ri ** 2)
    I = math.pi * (ro ** 4 - ri ** 4) / 4
    G = E / (2 * (1 + nu))
    m = ri / ro
    m2 = m * m
    a, b, n1 = 1 + m2, 1 - m2, 1 + nu
    c = (8 * nu + 14) * nu + 7
    kap = (6 * n1 * n1) * a * a / (c * b * b + 48 * m2 * n1 * n1)
    return np.diag([1 / (E * A), 1 / (kap * G * A), 1 / (kap * G * A), 1 / (G * 2 * I), 1 / (E * I), 1 / (E * I)]), kap
