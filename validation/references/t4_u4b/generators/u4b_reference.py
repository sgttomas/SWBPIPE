#!/usr/bin/env python3
"""T4-U4b frozen independent references: arc stress quantities.

Standard library only (decimal, fractions, json, hashlib, math). It imports,
reads and runs no product code, product output or result fixture. The T4-U4a
reference files (requests.json, oracle.json) are read as data and bound by
SHA-256.

Check mode (default): rebuild cases.json, oracle.json and SHA256SUMS in memory,
compare them byte for byte with the files on disk, run every cross-check, and
fail on any disagreement or unexpected file. `--write` rewrites the outputs; it
is used only for an intended change of the references, never to absorb a
product result.

Run from anywhere:  python3 -I -B generators/u4b_reference.py [--write]
"""

import argparse
import decimal
import hashlib
import json
import math
import os
import sys
import time
from decimal import Decimal, localcontext
from fractions import Fraction

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
U4A_DIR = os.path.join(os.path.dirname(ROOT), "t4_u4a")
# The U4a files this reference was cross-checked against; a change is refused
# until it is examined (README, re-freeze rules).
U4A_REQUESTS_SHA256 = "6582d2504640628a5fb6ca5dd9c937594c7a4ca532869d99fa80f3c72447a9c8"
U4A_ORACLE_SHA256 = "7e4e384501076d9796b1e789388cc3aa1da3d5c135d668f88a838295341f7cd5"

BASE_PREC = 90          # working precision (significant digits)
OUT_DIGITS = 40         # significant digits written for every value
SAMPLES = 4096          # dense-sampling cells per arc (divisible by 4)

D = Decimal


# ---------------------------------------------------------------------------
# Decimal arithmetic
# ---------------------------------------------------------------------------

class K:
    """Precision-dependent constants (recomputed by set_precision)."""
    PI = None
    HALF_SQRT2 = None
    EPS = None


def set_precision(prec):
    decimal.setcontext(decimal.Context(prec=prec, rounding=decimal.ROUND_HALF_EVEN,
                                       Emax=10 ** 6, Emin=-10 ** 6))
    K.PI = _compute_pi()
    K.HALF_SQRT2 = D(2).sqrt() / 2
    K.EPS = D(10) ** (-(prec - 8))


def _compute_pi():
    with localcontext() as c:
        c.prec += 12
        eps = D(10) ** (-(c.prec + 2))

        def atan_inv(n):
            x = D(1) / n
            x2 = x * x
            s = x
            t = x
            k = 1
            while True:
                t = -t * x2
                k += 2
                term = t / k
                if abs(term) < eps:
                    break
                s += term
            return s

        p = 16 * atan_inv(5) - 4 * atan_inv(239)
    return +p


def dq(fr):
    fr = Fraction(fr)
    return D(fr.numerator) / D(fr.denominator)


def b64(x):
    """The exact value of a binary64 number (used for coordinates, y_reference
    and load components)."""
    return dq(Fraction(float(x)))


def dtarget(x):
    """The exact decimal target of a JSON number (section, material, radius,
    pressure, SIF and k values)."""
    if isinstance(x, int):
        return D(x)
    return D(repr(float(x)))


def sincos(x):
    with localcontext() as c:
        c.prec += 16
        twopi = 2 * K.PI
        k = (x / twopi).to_integral_value()
        y = x - k * twopi
        m = 10
        y2 = y / (2 ** m)
        yy = y2 * y2
        s = y2
        t = y2
        n = 1
        eps = D(10) ** (-(c.prec + 4))
        while True:
            t = -t * yy / ((n + 1) * (n + 2))
            n += 2
            if abs(t) < eps:
                break
            s += t
        cs = (1 - s * s).sqrt()
        for _ in range(m):
            s, cs = 2 * s * cs, cs * cs - s * s
    return +s, +cs


def atan(x):
    with localcontext() as c:
        c.prec += 16
        sign = 1
        if x < 0:
            x = -x
            sign = -1
        inv = False
        if x > 1:
            x = 1 / x
            inv = True
        k = 0
        while x > D("0.001"):
            x = x / (1 + (1 + x * x).sqrt())
            k += 1
        xx = x * x
        s = x
        t = x
        n = 1
        eps = D(10) ** (-(c.prec + 4))
        while True:
            t = -t * xx
            n += 2
            term = t / n
            if abs(term) < eps:
                break
            s += term
        s = s * (2 ** k)
        if inv:
            s = K.PI / 2 - s
        s = sign * s
    return +s


def atan2(y, x):
    if x > 0:
        return atan(y / x)
    if x < 0:
        return atan(y / x) + (K.PI if y >= 0 else -K.PI)
    if y > 0:
        return K.PI / 2
    if y < 0:
        return -K.PI / 2
    return D(0)


def sgn(x):
    return 1 if x > 0 else (-1 if x < 0 else 0)


def gauss_legendre(n):
    nodes = []
    for i in range(1, n + 1):
        x = D(math.cos(math.pi * (i - 0.25) / (n + 0.5)))
        dp = None
        for _ in range(200):
            p0, p1 = D(1), x
            for kk in range(2, n + 1):
                p0, p1 = p1, ((2 * kk - 1) * x * p1 - (kk - 1) * p0) / kk
            dp = n * (x * p1 - p0) / (x * x - 1)
            dx = p1 / dp
            x -= dx
            if abs(dx) < K.EPS * D(10) ** -10:
                break
        p0, p1 = D(1), x
        for kk in range(2, n + 1):
            p0, p1 = p1, ((2 * kk - 1) * x * p1 - (kk - 1) * p0) / kk
        dp = n * (x * p1 - p0) / (x * x - 1)
        w = 2 / ((1 - x * x) * dp * dp)
        nodes.append((x, w))
    return nodes


def gl_integrate(f, a, b, rule):
    h = (b - a) / 2
    m = (a + b) / 2
    total = None
    for x, w in rule:
        v = f(m + h * x)
        if isinstance(v, tuple):
            v = tuple(w * vi for vi in v)
            total = v if total is None else tuple(t + vi for t, vi in zip(total, v))
        else:
            total = w * v if total is None else total + w * v
    if isinstance(total, tuple):
        return tuple(h * t for t in total)
    return h * total


# vectors ------------------------------------------------------------------

ZV = (D(0), D(0), D(0))


def vadd(a, b):
    return (a[0] + b[0], a[1] + b[1], a[2] + b[2])


def vsub(a, b):
    return (a[0] - b[0], a[1] - b[1], a[2] - b[2])


def vsc(k, a):
    return (k * a[0], k * a[1], k * a[2])


def vdot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def vcross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def vnorm(a):
    return vdot(a, a).sqrt()


def vunit(a):
    return vsc(1 / vnorm(a), a)


# output -------------------------------------------------------------------

def fmt(x):
    x = +x
    if x == 0:
        return "0"
    return format(x, ".%de" % (OUT_DIGITS - 1))


# Exact zeros. A value that is zero exactly is computed as an exact zero where
# its zero is known structurally (fibre coordinates are formed from the defining
# vector, so a component that is exactly zero stays exactly zero). Otherwise a
# written value below SNAP_BELOW in SI units is an arithmetic residue of the
# working precision (about 1e-90 here) and is written as "0". Two assertions keep
# the rule honest: no written nonzero value lies below LEGIT_FLOOR (a gap of 30
# orders above the snap), and every value snapped when the cases are recomputed
# at BASE_PREC + 20 digits lies below RESIDUE_AT_HIGH_PREC, so it shrinks with
# the working precision (precision_stability).
SNAP_BELOW = D("1e-60")
LEGIT_FLOOR = D("1e-30")
RESIDUE_AT_HIGH_PREC = D("1e-80")


class SnapLog:
    count = 0
    largest = D(0)
    floor_violations = []

    @classmethod
    def reset(cls):
        cls.count, cls.largest, cls.floor_violations = 0, D(0), []


def snap(x):
    x = +x
    if x == 0:
        return D(0)                 # also turns a signed zero into +0
    if abs(x) < SNAP_BELOW:
        SnapLog.count += 1
        SnapLog.largest = max(SnapLog.largest, abs(x))
        return D(0)
    if abs(x) < LEGIT_FLOOR:
        SnapLog.floor_violations.append(x)
    return x


def V(x):
    x = snap(x)
    return [fmt(x), float(x)]


def sci(x, digits=4):
    x = D(x)
    if x == 0:
        return "0"
    return format(x, ".%de" % (digits - 1))


def deg(x):
    return x * 180 / K.PI


# ---------------------------------------------------------------------------
# Line model
# ---------------------------------------------------------------------------

STATIONS = [("end_i", Fraction(0)), ("quarter_1", Fraction(1, 4)), ("midspan", Fraction(1, 2)),
            ("quarter_3", Fraction(3, 4)), ("end_j", Fraction(1))]
FIBRES = [("y_pos", 0), ("diag_45", 45), ("z_pos", 90), ("diag_135", 135), ("y_neg", 180), ("z_neg", 270)]
NAMED_FIBRES = ["y_pos", "z_pos", "y_neg", "z_neg"]
RESULTANT_KEYS = ["N", "V_y", "V_z", "T", "M_y", "M_z"]


def fibre_cs(deg_angle):
    h = K.HALF_SQRT2
    table = {0: (D(1), D(0)), 45: (h, h), 90: (D(0), D(1)), 135: (-h, h), 180: (D(-1), D(0)),
             225: (-h, -h), 270: (D(0), D(-1)), 315: (h, -h)}
    return table[deg_angle]


class Member:
    def __init__(self, mid, frm, to, kind, yref, section, material, R=None, k=None, sif=None):
        self.id = mid
        self.frm = frm
        self.to = to
        self.kind = kind            # "straight" or "arc"
        self.yref = yref
        self.section = section
        self.material = material
        self.R = R
        self.k = k
        self.sif = sif              # (i_ip, i_op) or None

    def setup(self, pos):
        xi, xj = pos[self.frm], pos[self.to]
        self.xi, self.xj = xi, xj
        d = vsub(xj, xi)
        L = vnorm(d)
        xc = vsc(1 / L, d)
        yp = vsub(self.yref, vsc(vdot(self.yref, xc), xc))
        yc = vunit(yp)
        zc = vcross(xc, yc)
        self.chord = L
        self.xc, self.yc, self.zc = xc, yc, zc
        if self.kind == "straight":
            self.ex, self.ey, self.ez = xc, yc, zc
            self.length = L
            return
        R = self.R
        s = L / (2 * R)
        c = (1 - s * s).sqrt()
        self.s, self.c = s, c
        self.ti = vadd(vsc(c, xc), vsc(s, yc))
        self.yi = vsub(vsc(s, xc), vsc(c, yc))
        self.ez = vsc(-1, zc)
        self.centre = vadd(xi, vsc(R, self.yi))
        self.phi = 2 * atan2(s, c)
        self.cosphi = 1 - 2 * s * s
        self.sinphi = 2 * s * c
        self.length = R * self.phi
        self.ex_phi = self.ex_at(self.cosphi, self.sinphi)
        # closure: the arc ends at x_j
        rj = self.point_cs(self.cosphi, self.sinphi)
        assert vnorm(vsub(rj, xj)) < K.EPS * (1 + vnorm(xj)) * 10 ** 6, "arc closure"

    # arc frame helpers (cs, sn) = (cos psi, sin psi)
    def ex_at(self, cs, sn):
        return vadd(vsc(cs, self.ti), vsc(sn, self.yi))

    def ey_at(self, cs, sn):
        return vadd(vsc(-sn, self.ti), vsc(cs, self.yi))

    def point_cs(self, cs, sn):
        return vsub(self.centre, vsc(self.R, self.ey_at(cs, sn)))

    def cs_at_fraction(self, u):
        u = Fraction(u)
        if u == 0:
            return D(1), D(0)
        if u == 1:
            return self.cosphi, self.sinphi
        if u == Fraction(1, 2):
            return self.c, self.s
        sn, cs = sincos(dq(u) * self.phi)
        return cs, sn

    def frame(self, u):
        if self.kind == "straight":
            return self.ex, self.ey, self.ez
        cs, sn = self.cs_at_fraction(u)
        return self.ex_at(cs, sn), self.ey_at(cs, sn), self.ez

    def point(self, u):
        if self.kind == "straight":
            return vadd(self.xi, vsc(dq(u), vsub(self.xj, self.xi)))
        cs, sn = self.cs_at_fraction(u)
        return self.point_cs(cs, sn)

    def full_load(self, w, P):
        """Resultant force and moment about P of the uniform load w (per unit
        length; per unit arc length on an arc) over the whole member."""
        if self.kind == "straight":
            F = vsc(self.length, w)
            mid = vsc(D(1) / 2, vadd(self.xi, self.xj))
            return F, vcross(vsub(mid, P), F)
        return self.partial_arc_load(w, D(0), self.phi, self.ti, self.ex_phi, P)

    def partial_arc_load(self, w, psi1, psi2, ex1, ex2, P):
        d = psi2 - psi1
        F = vsc(self.R * d, w)
        integral_r = vsub(vsc(d, self.centre), vsc(self.R, vsub(ex2, ex1)))
        arm = vsub(integral_r, vsc(d, P))
        return F, vsc(self.R, vcross(arm, w))

    def partial_straight_load(self, w, u1, u2, P):
        du = u2 - u1
        F = vsc(self.length * du, w)
        mid = vadd(self.xi, vsc((u1 + u2) / 2, vsub(self.xj, self.xi)))
        return F, vcross(vsub(mid, P), F)


class Section:
    def __init__(self, sid, od, wall, cm, ca):
        self.id, self.od, self.wall, self.cm, self.ca = sid, od, wall, cm, ca


def annulus(ro, wall):
    ri = ro - wall
    A = K.PI * (ro * ro - ri * ri)
    I = K.PI * (ro ** 4 - ri ** 4) / 4
    return {"wall": wall, "ri": ri, "ro": ro, "A": A, "I": I, "J": 2 * I, "Z": I / ro}


class Line:
    """A statically determinate line: one anchor, a chain of members, a free tip."""

    def __init__(self, cid, route, nodes, members, sections, materials, anchor, nodal_forces,
                 nodal_moments, member_loads, pressure, description=""):
        self.id = cid
        self.route = route                  # "exact" or "preview"
        self.description = description
        self.pos = nodes
        self.members = members
        self.mby = {m.id: m for m in members}
        self.sections = sections
        self.materials = materials
        self.anchor = anchor
        self.nf = nodal_forces
        self.nm = nodal_moments
        self.ml = member_loads
        self.pressure = pressure            # None or dict(p, members, terminals)
        for m in members:
            m.setup(nodes)
        self._traverse()
        self._bases()

    def _traverse(self):
        adj = {}
        for m in self.members:
            adj.setdefault(m.frm, []).append(m)
            adj.setdefault(m.to, []).append(m)
        assert len(adj[self.anchor]) == 1, "anchor must be a line end"
        order, cur, prev = [], self.anchor, None
        node_seq = [self.anchor]
        while True:
            cand = [m for m in adj[cur] if m is not prev]
            if not cand:
                break
            assert len(cand) == 1, "the line must be a chain"
            m = cand[0]
            fwd = m.frm == cur
            nxt = m.to if fwd else m.frm
            order.append((m, fwd))
            node_seq.append(nxt)
            prev, cur = m, nxt
        assert len(order) == len(self.members)
        self.order = order
        self.node_seq = node_seq
        self.tip = cur
        self.index = {m.id: k for k, (m, f) in enumerate(order)}
        # loads beyond member k (toward the tip), about its tip-side node
        self.beyond = []
        for k, (m, fwd) in enumerate(order):
            Q = self.pos[node_seq[k + 1]]
            F, M = ZV, ZV
            for n in node_seq[k + 1:]:
                f = self.nf.get(n, ZV)
                F = vadd(F, f)
                M = vadd(M, vadd(vcross(vsub(self.pos[n], Q), f), self.nm.get(n, ZV)))
            for m2, f2 in order[k + 1:]:
                w = self.ml.get(m2.id)
                if w is not None:
                    Fl, Ml = m2.full_load(w, Q)
                    F = vadd(F, Fl)
                    M = vadd(M, Ml)
            self.beyond.append((F, M, Q))

    def _bases(self):
        self.basis_props = {}
        self.Ai = None
        for m in self.members:
            sec = self.sections[m.section]
            ro = sec.od / 2
            if self.route == "exact":
                b = {"nominal_section": annulus(ro, sec.wall),
                     "reduced_section": annulus(ro, (sec.wall - sec.cm) - sec.ca)}
                ri_bore = ro - ((sec.wall - sec.cm) - sec.ca)
                b["_bore"] = {"ri": ri_bore, "Ai": K.PI * ri_bore * ri_bore}
            else:
                b = {"preview_single_section": annulus(ro, sec.wall - sec.cm)}
            self.basis_props[m.id] = b
        self.bases = ["nominal_section", "reduced_section"] if self.route == "exact" else ["preview_single_section"]

    def pressurized(self, mid):
        return self.pressure is not None and mid in self.pressure["members"]

    def pAi(self, mid):
        if not self.pressurized(mid):
            return D(0)
        return self.pressure["p"] * self.basis_props[mid]["_bore"]["Ai"]

    def nu(self, mid):
        return self.materials[self.mby[mid].material]["nu"]

    # section action ---------------------------------------------------------

    def cut_global(self, k, cs, sn, u=None):
        """j-side section action (global force, global moment about the section
        point) of member order[k] at (cos psi, sin psi) on an arc or at authored
        fraction u on a straight. Returns (F, M, P, ex, ey, ez, psi_or_u)."""
        m, fwd = self.order[k]
        Fb, MbQ, Q = self.beyond[k]
        w = self.ml.get(m.id)
        if m.kind == "straight":
            uD = dq(u)
            P = vadd(m.xi, vsc(uD, vsub(m.xj, m.xi)))
            ex, ey, ez = m.ex, m.ey, m.ez
        else:
            ex, ey, ez = m.ex_at(cs, sn), m.ey_at(cs, sn), m.ez
            P = m.point_cs(cs, sn)
        F = Fb
        M = vadd(MbQ, vcross(vsub(Q, P), Fb))
        if w is not None:
            if m.kind == "straight":
                u1, u2 = (uD, D(1)) if fwd else (D(0), uD)
                Fl, Ml = m.partial_straight_load(w, u1, u2, P)
            else:
                psi = u  # here u carries psi for arcs
                if fwd:
                    Fl, Ml = m.partial_arc_load(w, psi, m.phi, ex, m.ex_phi, P)
                else:
                    Fl, Ml = m.partial_arc_load(w, D(0), psi, m.ti, ex, P)
            F = vadd(F, Fl)
            M = vadd(M, Ml)
        if not fwd:
            F, M = vsc(-1, F), vsc(-1, M)
        return F, M, P, ex, ey, ez

    def resultants(self, k, frac):
        m, fwd = self.order[k]
        if m.kind == "straight":
            F, M, P, ex, ey, ez = self.cut_global(k, None, None, u=frac)
        else:
            cs, sn = m.cs_at_fraction(frac)
            F, M, P, ex, ey, ez = self.cut_global(k, cs, sn, u=dq(frac) * m.phi)
        r = {"N": vdot(F, ex), "V_y": vdot(F, ey), "V_z": vdot(F, ez),
             "T": vdot(M, ex), "M_y": vdot(M, ey), "M_z": vdot(M, ez)}
        r["_F"], r["_M"], r["_P"] = F, M, P
        r["_frame"] = (ex, ey, ez)
        return r

    def arc_resultants_psi(self, k, psi, cs=None, sn=None):
        m, fwd = self.order[k]
        if cs is None:
            sn, cs = sincos(psi)
        F, M, P, ex, ey, ez = self.cut_global(k, cs, sn, u=psi)
        w = self.ml.get(m.id)
        return {"N": vdot(F, ex), "V_y": vdot(F, ey), "V_z": vdot(F, ez),
                "T": vdot(M, ex), "M_y": vdot(M, ey), "M_z": vdot(M, ez),
                "wx": vdot(w, ex) if w is not None else D(0),
                "wy": vdot(w, ey) if w is not None else D(0),
                "wz": vdot(w, ez) if w is not None else D(0), "_F": F, "_M": M}


# ---------------------------------------------------------------------------
# Stress quantities
# ---------------------------------------------------------------------------

def fibre_normal(N, My, Mz, A, I, r, cb, sb):
    """sigma_x at the fibre (y, z) = r (cos beta, sin beta) (STRESS_REFERENCE 1)."""
    return N / A + (My * sb - Mz * cb) * r / I


def normal_extremes(N, My, Mz, A, Z, ro):
    B = (My * My + Mz * Mz).sqrt()
    out = {"sigma_plus": N / A + B / Z, "sigma_minus": N / A - B / Z, "B": B}
    if B == 0:
        out["tied"] = True
    else:
        out["tied"] = False
        out["plus_yz"] = (-Mz * ro / B, My * ro / B)
        out["plus_dir"] = (-Mz, My)         # sigma_plus fibre direction; sigma_minus is its opposite
    return out


def sv_tau_max_coefficient(ro, ri, nu, I):
    """tau_max = |V| * coefficient (inner surface, fibres normal to V)."""
    return ((3 + 2 * nu) * ro * ro + (1 + 2 * nu) * ri * ri) / (4 * (1 + nu) * I)


def sv_polar(Vy, Vz, r, cb, sb, ro, ri, nu, I):
    """Saint-Venant flexure shear (tau_xy, tau_xz) of the homogeneous annulus at
    (y, z) = r (cos beta, sin beta), driven by (V_y, V_z) by superposition.
    Polar rewrite of SHEAR_REFERENCE INDEPENDENT_REFUTATION section 2."""
    C = 1 / (2 * (1 + nu) * I)
    c = (3 + 2 * nu) / 4
    s = ro * ro + ri * ri
    p = ro * ro * ri * ri
    r2 = r * r
    g = c * (s + p / r2) - (1 - 2 * nu) / 4 * r2
    radial = c * (r2 - ro * ro) * (r2 - ri * ri) / r2
    vrho = Vy * cb + Vz * sb
    vbeta = -Vy * sb + Vz * cb
    tr = -C * radial * vrho
    tb = C * g * vbeta
    return tr * cb - tb * sb, tr * sb + tb * cb


def sv_refutation_field(Vx, X, Y, a, b, nu, I):
    """The refutation's Cartesian field for a force Vx along X (a = r_o, b = r_i);
    returns (tau_X, tau_Y). Used only as an independent check of sv_polar."""
    r2 = X * X + Y * Y
    s = a * a + b * b
    p = a * a * b * b
    c = (3 + 2 * nu) / 4
    e = (1 + 2 * nu) / 4
    B0 = -c * s + r2 / 2
    B2 = c * p / r2 + e * r2
    cos2 = (X * X - Y * Y) / r2
    sin2 = 2 * X * Y / r2
    k = -Vx / (2 * (1 + nu) * I)
    return k * (B0 + B2 * cos2), k * B2 * sin2


def sv_cartesian(Vy, Vz, y, z, ro, ri, nu, I):
    txy1, txz1 = sv_refutation_field(Vy, y, z, ro, ri, nu, I)
    txz2, txy2 = sv_refutation_field(Vz, z, y, ro, ri, nu, I)
    return txy1 + txy2, txz1 + txz2


def kappa_energy(ro, ri, nu):
    m2 = (ri / ro) ** 2
    n1 = 1 + nu
    return 6 * n1 * n1 * (1 + m2) ** 2 / ((7 + 14 * nu + 8 * nu * nu) * (1 + m2) ** 2
                                          + 4 * m2 * (5 + 10 * nu + 4 * nu * nu))


def toroidal_hoop(p, ri, ro, R, cb):
    """bore_load_transferred_toroidal_membrane_v1: the wall-cut average hoop
    stress at section angle beta (cos beta = cb, beta from +y toward +z)."""
    t = ro - ri
    return p * ri / t * (2 * R - ri * cb) / (2 * R - (ri + ro) * cb)


def toroidal_hoop_variant_b(p, ri_load, ri_wall, ro, R, cb):
    t = ro - ri_wall
    return p * ri_load * (2 * R - ri_load * cb) / (t * (2 * R - (ri_wall + ro) * cb))


def lame_hoop(p, ri, ro, r):
    return p * ri * ri / (ro * ro - ri * ri) * (1 + ro * ro / (r * r))


# ---------------------------------------------------------------------------
# STRESS_REFERENCE section 6: straight-member maximum (polynomial method)
# ---------------------------------------------------------------------------

def pval(c, x):
    v = D(0)
    for a in reversed(c):
        v = v * x + a
    return v


def pder(c):
    return [i * c[i] for i in range(1, len(c))]


def padd(a, b):
    n = max(len(a), len(b))
    return [(a[i] if i < len(a) else D(0)) + (b[i] if i < len(b) else D(0)) for i in range(n)]


def pmul(a, b):
    out = [D(0)] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            out[i + j] += x * y
    return out


def ptrim(c):
    c = list(c)
    big = max([abs(x) for x in c] + [D(0)])
    if big == 0:
        return []
    while c and abs(c[-1]) <= big * D(10) ** -60:
        c.pop()
    return c


def bisect_root(f, a, b, fa, fb, iters=320):
    for _ in range(iters):
        mid = (a + b) / 2
        fm = f(mid)
        if fm == 0:
            return mid
        if (fm > 0) == (fa > 0):
            a, fa = mid, fm
        else:
            b, fb = mid, fm
        if b - a <= K.EPS * D(10) ** -8 * (1 + abs(a)):
            break
    return (a + b) / 2


def poly_points(c, lo, hi):
    """Real roots of the polynomial c in [lo, hi], plus every critical point of
    c and of its derivatives there (harmless extra candidates). Recursive
    isolation: between consecutive critical points c is monotone."""
    c = ptrim(c)
    if len(c) <= 1:
        return []
    if len(c) == 2:
        r = -c[0] / c[1]
        return [r] if lo <= r <= hi else []
    crit = poly_points(pder(c), lo, hi)
    pts = sorted(set([lo] + [x for x in crit if lo <= x <= hi] + [hi]))
    out = list(crit)
    f = lambda x: pval(c, x)
    for a, b in zip(pts[:-1], pts[1:]):
        fa, fb = f(a), f(b)
        if fa == 0:
            out.append(a)
        elif fb != 0 and (fa > 0) != (fb > 0):
            out.append(bisect_root(f, a, b, fa, fb))
    if f(hi) == 0:
        out.append(hi)
    return out


def section6_maximum(a, b, c):
    """max over t in [0, 1] of |a(t)| + sqrt(b(t)^2 + c(t)^2) for polynomials a,
    b, c of degree <= 2 (STRESS_REFERENCE section 6). Returns (value, t)."""
    a, b, c = list(a), list(b), list(c)
    cand = [D(0), D(1)]
    cand += poly_points(a, D(0), D(1))                         # zeros of a
    for t in poly_points(b, D(0), D(1)):                       # common zeros of b, c
        cand.append(t)
    ap = pder(a) or [D(0)]
    Dp = padd(pmul(b, b), pmul(c, c))
    E = padd(pmul(b, pder(b) or [D(0)]), pmul(c, pder(c) or [D(0)]))
    H = padd(pmul(pmul(ap, ap), Dp), [-x for x in pmul(E, E)])
    cand += poly_points(H, D(0), D(1))
    cand += poly_points(pder(a) or [D(0)], D(0), D(1))         # step 5 subdivision at roots of a'
    best = None
    for t in cand:
        if t < 0 or t > 1:
            continue
        v = abs(pval(a, t)) + (pval(b, t) ** 2 + pval(c, t) ** 2).sqrt()
        if best is None or v > best[0] or (v == best[0] and t < best[1]):
            best = (v, t)
    return best


def quad_through(q0, q1, q2):
    """Quadratic in t through (0, q0), (1/2, q1), (1, q2), low to high."""
    return [q0, -3 * q0 + 4 * q1 - q2, 2 * q0 - 4 * q1 + 2 * q2]


# ---------------------------------------------------------------------------
# Along-member maxima
# ---------------------------------------------------------------------------

def obj_max_normal(r, bp, pAi, R):
    A, Z = bp["A"], bp["Z"]
    Ne = r["N"] + pAi
    My, Mz = r["M_y"], r["M_z"]
    B = (My * My + Mz * Mz).sqrt()
    f = abs(Ne) / A + B / Z
    dN = -R * r["wx"] + r["V_y"]
    dMy = R * r["V_z"] - r["T"]
    dMz = -R * r["V_y"]
    fp = sgn(Ne) * dN / A
    if B != 0:
        fp += (My * dMy + Mz * dMz) / B / Z
    return f, fp


def obj_intensified(r, bp, sif, R):
    ip, op = sif
    Z = bp["Z"]
    My, Mz = r["M_y"], r["M_z"]
    G = ((ip * Mz) ** 2 + (op * My) ** 2).sqrt()
    dMy = R * r["V_z"] - r["T"]
    dMz = -R * r["V_y"]
    gp = (ip * ip * Mz * dMz + op * op * My * dMy) / (Z * G) if G != 0 else D(0)
    return G / Z, gp


def arc_profile(line, k, n=SAMPLES):
    m, fwd = line.order[k]
    h = m.phi / n
    sh, ch = sincos(h)
    anchors = {0: (D(1), D(0)), n // 4: m.cs_at_fraction(Fraction(1, 4)), n // 2: (m.c, m.s),
               3 * n // 4: m.cs_at_fraction(Fraction(3, 4)), n: (m.cosphi, m.sinphi)}
    psi_exact = {n // 2: m.phi / 2, n: m.phi, 0: D(0)}
    cs, sn = D(1), D(0)
    out = []
    for i in range(n + 1):
        if i in anchors:
            cs, sn = anchors[i]
        psi = psi_exact.get(i, h * i)
        out.append((psi, line.arc_resultants_psi(k, psi, cs, sn)))
        cs, sn = cs * ch - sn * sh, sn * ch + cs * sh
    return out, h


def golden_max(func, a, b, iters=170):
    gr = (D(5).sqrt() - 1) / 2
    c = b - gr * (b - a)
    d = a + gr * (b - a)
    fc, fd = func(c), func(d)
    for _ in range(iters):
        if fc >= fd:
            b, d, fd = d, c, fc
            c = b - gr * (b - a)
            fc = func(c)
        else:
            a, c, fc = c, d, fd
            d = a + gr * (b - a)
            fd = func(d)
    return max(fc, fd)


def arc_lipschitz(line, k, prof):
    m, fwd = line.order[k]
    w = line.ml.get(m.id)
    wn = vnorm(w) if w is not None else D(0)
    F0 = vnorm(prof[0][1]["_F"])
    M0 = vnorm(prof[0][1]["_M"])
    Fmax = F0 + wn * m.R * m.phi
    Mmax = M0 + m.R * m.phi * Fmax
    return wn, Fmax, Mmax


def arc_analytic_max(line, k, prof, h, objective, lip):
    """Maximum over the whole arc of objective(r) -> (f, f'), with f' the exact
    derivative in psi from the closed-form resultants (section 4 of the README).
    Candidates: both ends and every interior root of f' where it changes sign
    from + to - on the dense grid (refined by bisection). Returns a dict."""
    m, fwd = line.order[k]
    vals = [objective(r) for (psi, r) in prof]

    def at(psi):
        return objective(line.arc_resultants_psi(k, psi))

    cands = [(prof[0][0], vals[0][0], "end"), (prof[-1][0], vals[-1][0], "end")]
    for i in range(len(prof) - 1):
        a, b = vals[i][1], vals[i + 1][1]
        if a > 0 and b < 0:
            root = bisect_root(lambda x: at(x)[1], prof[i][0], prof[i + 1][0], a, b, iters=170)
            cands.append((root, at(root)[0], "stationary"))
        elif a == 0 and 0 < i:
            cands.append((prof[i][0], vals[i][0], "stationary"))
    best = None
    for c in cands:
        if best is None or c[1] > best[1]:
            best = c
    maxS = max(v[0] for v in vals)
    imax = [v[0] for v in vals].index(maxS)
    n = len(prof) - 1
    station_idx = [0, n // 4, n // 2, 3 * n // 4, n]
    station_vals = [vals[i][0] for i in station_idx]
    width = lip * h / 2
    out = {"psi": best[0], "value": best[1], "kind": best[2], "max_samples": maxS,
           "argmax_sample_psi": prof[imax][0], "bracket_upper": maxS + width, "lipschitz": lip,
           "h": h, "samples": n + 1, "station_values": station_vals,
           "station_max": max(station_vals), "candidates": cands}
    if best[2] == "stationary":
        hh = D(10) ** -22
        f2 = (at(best[0] + hh)[1] - at(best[0] - hh)[1]) / (2 * hh)
        out["second_derivative"] = f2
    out["grid"] = [(psi, v[0]) for (psi, r), v in zip(prof, vals)]
    out["evaluate"] = lambda psi: at(psi)[0]
    return out


# Certification of a maximum at the criterion ------------------------------------
#
# Both maximized measures are sums of terms sup_e (e . u(x)) over unit vectors e:
# |N_eff| (u = N_eff, e = +/-1), hypot(M_y, M_z) and hypot(i_op M_y, i_ip M_z).
# Each e . u has second derivative e . u'' >= -|u''|, and a supremum of functions
# whose second derivative is at least -K is one: f + K x^2/2 is convex as a
# supremum of convex functions, kinks of |.| and of the hypot included. So with
# f'' >= -K on a cell [a, b] of width h,
#     f(x) <= linear interpolant + K (x - a)(b - x)/2 <= max(f(a), f(b)) + K h^2/8.
# Starting from the dense grid, every cell whose bound exceeds value + CERT_FRACTION
# tolerances is bisected until all close. The result bounds the true maximum from
# above; the frozen value is attained at its location, so it bounds it from below.
#
# K on an arc, from dF/ds = -w, dM/ds = -e_x x F, de_x/ds = e_y/R, de_y/ds = -e_x/R:
#     N''   = -2 R w_y - N
#     M_y'' = -R^2 w_z - M_y
#     M_z'' =  R^2 w_y + R N
# so |N''| <= 2 R |w| + F_max and |(M_y, M_z)''| <= R^2 |w| + M_max + R F_max, with
# F_max = |F(0)| + |w| R phi and M_max = |M(0)| + R phi F_max (the Lipschitz bounds).
# On a straight in the fraction t, N_eff is linear and (M_y, M_z)/Z = (b, c) is
# quadratic, so K = 2 hypot(b_2, c_2) exactly.

CERT_FRACTION = D("0.01")          # each maximum is certified to 1/100 of its tolerance
CERT_MAX_BISECTIONS = 200000


def certify_maximum(f, grid, K, value, t):
    target = value + CERT_FRACTION * t
    stack = [(grid[i][0], grid[i + 1][0], grid[i][1], grid[i + 1][1]) for i in range(len(grid) - 1)]
    ub = None
    sample_max = max(g[1] for g in grid)
    bisections = 0
    finest = grid[1][0] - grid[0][0]
    while stack:
        a, b, fa, fb = stack.pop()
        w = b - a
        bound = max(fa, fb) + K * w * w / 8
        if bound <= target:
            ub = bound if ub is None or bound > ub else ub
            continue
        bisections += 1
        require(bisections <= CERT_MAX_BISECTIONS, "maximum certification did not close")
        mid = (a + b) / 2
        fm = f(mid)
        sample_max = max(sample_max, fm)
        finest = min(finest, w / 2)
        stack.append((mid, b, fm, fb))
        stack.append((a, mid, fa, fm))
    require(ub is not None and ub <= target, "maximum certification")
    require(sample_max <= value + abs(value) * D(10) ** -60, "a sample exceeds the analytic maximum")
    return {"upper_bound": ub, "bisections": bisections, "finest_cell": finest, "initial_cells": len(grid) - 1,
            "K": K, "excess_tolerances": (ub - value) / t}


def cert_record(c, unit):
    return {"method": "rigorous upper bound from f'' >= -K (kinks of |N_eff| and of the hypot included): on a "
                      "cell of width h, max f <= max(endpoint values) + K h^2/8; starting from the dense grid, "
                      "cells whose bound exceeds the value + %s tolerance are bisected until every cell closes"
                      % sci(CERT_FRACTION, 1),
            "K_second_derivative_bound": sci(c["K"], 6) + " " + unit,
            "initial_cells": c["initial_cells"], "bisections": c["bisections"],
            "finest_cell": sci(c["finest_cell"]) + (" rad" if unit.endswith("rad^2") else " fraction"),
            "upper_bound_minus_value_tolerances": sci(c["excess_tolerances"]),
            "certified": "the true maximum lies within %s tolerance above the frozen value, which is attained at "
                         "the frozen location" % sci(CERT_FRACTION, 1)}


def guaranteed_location_band(evaluate, slope, x, end, K2, value, zero_scale, arc=False):
    """A finite-distance band from the same global f'' >= -K used above.

    If g = f + K*x*x/2 is convex, a supporting slope s at c gives
        f(c+d) >= f(c) + s*d - K*d*d/2.
    Thus loss from a stationary maximum is <= K*d*d/2, even across the
    upward kinks of the absolute value or norm. A pointwise f'' is insufficient.
    The callers supply a supporting slope: at a zero norm, the zero vector is
    an active member of its unit-ball supremum. No branch must stay nonzero
    away from c. The global maximum itself is independently certified above.

    Use 99% of the tolerance for the quadratic and residual-slope terms. Check
    the remaining margin against both serialized centre/value representations,
    both arc coordinates, and radius rounding. K is unrounded working precision,
    not the six-digit informational K in the certification record.
    """
    require(0 < x < end and K2 >= 0, "location band needs an interior maximum and nonnegative K")

    def written(v):
        return (D(fmt(v)), D.from_float(float(v)))

    centres = list(written(x))
    if arc:
        centres.extend(c * end for c in written(x / end))
    require(all(0 < c < end for c in centres), "serialized interior maximum left the member")
    expected = written(value)
    t = min(tol(v, z) for v in expected for z in written(zero_scale))
    E = max(abs(slope(c)) for c in centres)
    require(E * end <= D("0.001") * t, "location band centre is not stationary at serialized precision")
    budget = D("0.99") * t
    if K2 == 0:
        # A convex function with an interior global maximum is constant. With
        # zero supporting slope the whole domain is safe; no division by K.
        radius = end if E == 0 else min(end, budget / E)
    else:
        # Stable positive root of E*r + K*r*r/2 = budget (also valid for E=0).
        radius = min(end, 2 * budget / ((E * E + 2 * K2 * budget).sqrt() + E))
    radii = list(written(radius))
    if arc:
        radii.extend(r * end for r in written(radius / end))
    rounded_radius = max(radii)
    require(rounded_radius > 0, "location band underflow")
    for c in centres:
        deficit = max(D(0), max(expected) - evaluate(c))
        require(deficit + abs(slope(c)) * rounded_radius + K2 * rounded_radius ** 2 / 2 <= t,
                "serialized location band exceeds its finite-distance tolerance")
        # These direct checks supplement the full-interval convexity proof.
        # Comparison locations are the intersection with the member domain.
        lo, hi = max(D(0), c - rounded_radius), min(end, c + rounded_radius)
        for j in range(5):
            observed = evaluate(lo + (hi - lo) * D(j) / 4)
            require(all(abs(observed - v) <= t for v in expected), "location band endpoint/interior loss")
    return radius


def check_location_band_edge_cases():
    """Exercise clipping, flat/degenerate curvature and an upward norm kink."""
    one, half = D(1), D("0.5")
    # The left edge is clipped at zero, while the centre remains interior.
    c = D("0.000001")
    radius = guaranteed_location_band(lambda x: one - (x - c) ** 2,
                                     lambda x: -2 * (x - c), c, one, D(2), one, one)
    require(radius > c, "location band clipping control")
    # Zero K and an identically zero norm: the full member is safe.
    require(guaranteed_location_band(lambda x: one, lambda x: D(0), half, one,
                                    D(0), one, one) == one, "zero-curvature location band")
    # f'' at the maximum can be zero although a global lower bound is positive.
    guaranteed_location_band(lambda x: one - (x - half) ** 4,
                             lambda x: -4 * (x - half) ** 3, half, one, D(3), one, one)
    # An upward kink inside the band does not invalidate the lower bound.
    a, w = D("0.00001"), D("0.000001")
    def f(x):
        u = x - half
        return one - u * u + w * (abs(u - a) + u - a)
    def fp(x):
        u = x - half
        return -2 * u + w * (sgn(u - a) + 1)
    radius = guaranteed_location_band(f, fp, half, one, D(2), one, one)
    require(radius > a, "location band kink control")
    # End maxima have exact locations, never a stationary location band.
    try:
        guaranteed_location_band(lambda x: one - x, lambda x: -one,
                                 D(0), one, D(0), one, one)
    except Fail:
        pass
    else:
        raise Fail("endpoint maximum incorrectly admitted to location-band comparison")


def arc_second_derivative_check(line, k):
    """The closed-form second derivatives used for K against central differences of
    the exact first derivatives, at two interior angles."""
    m, fwd = line.order[k]
    worst = D(0)
    hh = D(10) ** -25
    for frac in (D(1) / 3, D(2) / 3):
        psi = frac * m.phi
        r = line.arc_resultants_psi(k, psi)
        rp, rm = line.arc_resultants_psi(k, psi + hh), line.arc_resultants_psi(k, psi - hh)

        def d1(q):
            return (-m.R * q["wx"] + q["V_y"], m.R * q["V_z"] - q["T"], -m.R * q["V_y"])
        fd = [(a - b) / (2 * hh) for a, b in zip(d1(rp), d1(rm))]
        cf = (-2 * m.R * r["wy"] - r["N"], -m.R * m.R * r["wz"] - r["M_y"], m.R * m.R * r["wy"] + m.R * r["N"])
        scale = (vnorm(r["_F"]) * (1 + m.R) + vnorm(r["_M"]) + m.R * m.R * (vnorm(line.ml[m.id])
                                                                             if m.id in line.ml else 0))
        worst = max(worst, max(abs(a - b) for a, b in zip(fd, cf)) / scale)
    require(worst < D(10) ** -30, "arc second-derivative formulas")
    return worst


def fibre_of_max(Ne, My, Mz):
    """Direction (a, b) of the outer fibre governing |N_eff|/A + hypot(M_y, M_z)/Z:
    the sigma_plus fibre (-M_z, M_y) when N_eff >= 0, else the sigma_minus fibre."""
    if My == 0 and Mz == 0:
        return None
    return (-Mz, My) if Ne >= 0 else (Mz, -My)


def straight_polys(line, k, bp, pAi):
    m, fwd = line.order[k]
    r0 = line.resultants(k, Fraction(0))
    w = line.ml.get(m.id) or ZV
    L = m.length
    wx, wy, wz = vdot(w, m.ex), vdot(w, m.ey), vdot(w, m.ez)
    A, Z = bp["A"], bp["Z"]
    a = [(r0["N"] + pAi) / A, -wx * L / A]
    b = [r0["M_y"] / Z, L * r0["V_z"] / Z, -L * L * wz / 2 / Z]
    c = [r0["M_z"] / Z, -L * r0["V_y"] / Z, L * L * wy / 2 / Z]
    return a, b, c, (vnorm(r0["_F"]), vnorm(w), L)


# ---------------------------------------------------------------------------
# Analysis of one line
# ---------------------------------------------------------------------------

NORMAL_Q = {"fibre_normal", "sigma_plus", "sigma_minus", "max_abs_normal", "intensified_bending",
            "axial_plus_intensified_bending"}
SHEAR_Q = {"sv_max", "sv", "torsional_shear", "flexure_plus_torsion"}
NU_03 = D("0.3")


def station_quantities(m, r, bp, pAi, nu):
    A, I, J, Z, ro, ri = bp["A"], bp["I"], bp["J"], bp["Z"], bp["ro"], bp["ri"]
    Ne = r["N"] + pAi
    Vy, Vz, T, My, Mz = r["V_y"], r["V_z"], r["T"], r["M_y"], r["M_z"]
    q, mu, aux = {}, {}, {}

    def mut(name, key, val):
        mu.setdefault(name, {})[key] = val

    arc = m.kind == "arc"
    for surf, rr in (("outer", ro), ("inner", ri)):
        for name, ang in FIBRES:
            if name not in NAMED_FIBRES:
                continue
            cb, sb = fibre_cs(ang)
            key = ("fibre_normal", surf, name)
            q[key] = fibre_normal(Ne, My, Mz, A, I, rr, cb, sb)
            mut("fibre_mz_y_sign_flipped", key, Ne / A + (My * sb + Mz * cb) * rr / I)
            mut("fibre_frame_rotated_y_away_from_centre", key, Ne / A - (My * sb - Mz * cb) * rr / I)
            if arc:
                mut("k_multiplying_stress", key, Ne / A + m.k * (My * sb - Mz * cb) * rr / I)
    ext = normal_extremes(Ne, My, Mz, A, Z, ro)
    B = ext["B"]
    q[("sigma_plus",)] = ext["sigma_plus"]
    q[("sigma_minus",)] = ext["sigma_minus"]
    q[("max_abs_normal",)] = abs(Ne) / A + B / Z
    abs_sum = abs(Ne) / A + abs(My) / Z + abs(Mz) / Z
    mut("fibre_component_abs_sum", ("max_abs_normal",), abs_sum)
    mut("fibre_component_abs_sum", ("sigma_plus",), Ne / A + abs(My) / Z + abs(Mz) / Z)
    if arc:
        mut("k_multiplying_stress", ("max_abs_normal",), abs(Ne) / A + m.k * B / Z)
    aux["ext"] = ext
    # Saint-Venant transverse shear (local_straight_annulus_on_arc on arcs)
    coef = sv_tau_max_coefficient(ro, ri, nu, I)
    Vm = (Vy * Vy + Vz * Vz).sqrt()
    q[("sv_max",)] = Vm * coef
    mut("sv_v_over_a", ("sv_max",), Vm / A)
    mut("sv_v_over_kappa_a", ("sv_max",), Vm / (kappa_energy(ro, ri, nu) * A))
    mut("sv_two_v_over_a", ("sv_max",), 2 * Vm / A)
    mut("sv_solid_circle_centre", ("sv_max",), (3 + 2 * nu) / (2 * (1 + nu)) * Vm / A)
    mut("sv_solid_circle_four_thirds", ("sv_max",), 4 * Vm / (3 * A))
    mut("sv_max_at_outer_surface", ("sv_max",),
        Vm * ((1 + 2 * nu) * ro * ro + (3 + 2 * nu) * ri * ri) / (4 * (1 + nu) * I))
    mut("sv_max_on_axis_parallel_to_v", ("sv_max",),
        Vm * (3 + 2 * nu) / 4 * (ro - ri) ** 2 / (2 * (1 + nu) * I))
    if nu != NU_03:
        mut("sv_nu_fixed_0_3", ("sv_max",), Vm * sv_tau_max_coefficient(ro, ri, NU_03, I))
    if Vm == 0:
        aux["sv_fibres"] = None
    else:
        # the two inner fibres normal to V: directions +/-(-V_z, V_y)/|V| (an unordered pair)
        aux["sv_fibres"] = [(-Vz, Vy), (Vz, -Vy)]
    for surf, rr in (("outer", ro), ("inner", ri)):
        for name, ang in FIBRES:
            cb, sb = fibre_cs(ang)
            txy, txz = sv_polar(Vy, Vz, rr, cb, sb, ro, ri, nu, I)
            q[("sv", surf, name, "tau_xy")] = txy
            q[("sv", surf, name, "tau_xz")] = txz
            wxy, wxz = sv_polar(Vz, Vy, rr, cb, sb, ro, ri, nu, I)
            mut("sv_vy_vz_swapped", ("sv", surf, name, "tau_xy"), wxy)
            mut("sv_vy_vz_swapped", ("sv", surf, name, "tau_xz"), wxz)
            if nu != NU_03:
                nxy, nxz = sv_polar(Vy, Vz, rr, cb, sb, ro, ri, NU_03, I)
                mut("sv_nu_fixed_0_3", ("sv", surf, name, "tau_xy"), nxy)
                mut("sv_nu_fixed_0_3", ("sv", surf, name, "tau_xz"), nxz)
    tt = T * ro / J
    q[("torsional_shear",)] = tt
    for name in NAMED_FIBRES:
        ang = dict(FIBRES)[name]
        cb, sb = fibre_cs(ang)
        y, z = ro * cb, ro * sb
        txy, txz = sv_polar(Vy, Vz, ro, cb, sb, ro, ri, nu, I)
        q[("flexure_plus_torsion", name, "tau_xy")] = txy - T * z / J
        q[("flexure_plus_torsion", name, "tau_xz")] = txz + T * y / J
    if arc and m.sif is not None:
        ip, op = m.sif
        G = ((ip * Mz) ** 2 + (op * My) ** 2).sqrt() / Z
        q[("intensified_bending",)] = G
        mut("sif_factors_swapped", ("intensified_bending",), ((op * Mz) ** 2 + (ip * My) ** 2).sqrt() / Z)
        mut("sif_one_scalar_for_both_directions", ("intensified_bending",), max(ip, op) * B / Z)
        mut("k_multiplying_stress", ("intensified_bending",), m.k * B / Z)
        q[("axial_plus_intensified_bending",)] = abs(Ne) / A + G
        mut("sif_applied_to_whole_max_normal", ("axial_plus_intensified_bending",), ip * (abs(Ne) / A + B / Z))
        mut("sif_applied_to_axial", ("axial_plus_intensified_bending",), ip * abs(Ne) / A + G)
        mut("sif_applied_to_torsion", ("torsional_shear",), ip * tt)
    return q, mu, aux


class Analysis:
    pass


def analyse(line, profile_samples=SAMPLES):
    an = Analysis()
    an.line = line
    an.res, an.q, an.mut, an.aux = {}, {}, {}, {}
    an.mmax, an.imax, an.hoop, an.prof = {}, {}, {}, {}
    an.nsign, an.d2check = {}, {}
    for k, (m, fwd) in enumerate(line.order):
        mid = m.id
        pAi = line.pAi(mid)
        nu = line.nu(mid)
        an.res[mid] = {}
        for st, frac in STATIONS:
            r = line.resultants(k, frac)
            an.res[mid][st] = r
            for basis in line.bases:
                bp = line.basis_props[mid][basis]
                an.q[(mid, st, basis)], an.mut[(mid, st, basis)], an.aux[(mid, st, basis)] = \
                    station_quantities(m, r, bp, pAi, nu)
        if m.kind == "arc":
            prof, h = arc_profile(line, k, profile_samples)
            an.prof[mid] = (prof, h)
            wn, Fmax, Mmax = arc_lipschitz(line, k, prof)
            # sign change of the effective axial force along the arc
            nvals = [r["N"] + pAi for (psi, r) in prof]
            zeros = []
            for i in range(len(nvals) - 1):
                if nvals[i] != 0 and nvals[i + 1] != 0 and (nvals[i] > 0) != (nvals[i + 1] > 0):
                    z = bisect_root(lambda x: line.arc_resultants_psi(k, x)["N"] + pAi,
                                    prof[i][0], prof[i + 1][0], nvals[i], nvals[i + 1], iters=170)
                    zeros.append(z)
            an.nsign[mid] = zeros
            an.d2check[mid] = arc_second_derivative_check(line, k)
            Kn = 2 * m.R * wn + Fmax                       # >= |N''| over the arc
            Km = m.R * m.R * wn + Mmax + m.R * Fmax        # >= |(M_y, M_z)''| over the arc
            for basis in line.bases:
                bp = line.basis_props[mid][basis]
                lip_f = (m.R * wn + Fmax) / bp["A"] + (m.R * Fmax + Mmax) / bp["Z"]
                an.mmax[(mid, basis)] = arc_analytic_max(
                    line, k, prof, h, lambda r, bp=bp, pAi=pAi, R=m.R: obj_max_normal(r, bp, pAi, R), lip_f)
                an.mmax[(mid, basis)]["bp"] = bp
                an.mmax[(mid, basis)]["K2"] = Kn / bp["A"] + Km / bp["Z"]
                if m.sif is not None:
                    lip_g = max(m.sif) * (m.R * Fmax + Mmax) / bp["Z"]
                    an.imax[(mid, basis)] = arc_analytic_max(
                        line, k, prof, h, lambda r, bp=bp, sif=m.sif, R=m.R: obj_intensified(r, bp, sif, R), lip_g)
                    an.imax[(mid, basis)]["K2"] = max(m.sif) * Km / bp["Z"]
        else:
            for basis in line.bases:
                bp = line.basis_props[mid][basis]
                a, b, c, (F0, wn, L) = straight_polys(line, k, bp, pAi)
                val, u = section6_maximum(a, b, c)
                an.mmax[(mid, basis)] = {"value": val, "u": u, "polys": (a, b, c), "bp": bp,
                                         "F0": F0, "wn": wn, "L": L}
        # toroidal hoop and radial values on pressurized arcs
        if m.kind == "arc" and line.pressurized(mid):
            p = line.pressure["p"]
            for basis in line.bases:
                bp = line.basis_props[mid][basis]
                vals = {}
                for name, ang in FIBRES:
                    cb, sb = fibre_cs(ang)
                    vals[name] = toroidal_hoop(p, bp["ri"], bp["ro"], m.R, cb)
                an.hoop[(mid, basis)] = {"p": p, "ri": bp["ri"], "ro": bp["ro"], "t": bp["wall"], "R": m.R,
                                         "values": vals, "lame_mean": p * bp["ri"] / bp["wall"]}
    return an


# zero scales -----------------------------------------------------------------

def zero_scales(an):
    line = an.line
    zs = {}
    case_force = D(0)
    case_moment = D(0)
    case_nw = D(0)
    for m, fwd in line.order:
        f = max(max(abs(r["N"]), abs(r["V_y"]), abs(r["V_z"])) for r in an.res[m.id].values())
        mo = max(max(abs(r["T"]), abs(r["M_y"]), abs(r["M_z"])) for r in an.res[m.id].values())
        nw = max(abs(r["N"] + line.pAi(m.id)) for r in an.res[m.id].values())
        zs[m.id] = {"force": f, "moment": mo}
        case_force, case_moment, case_nw = max(case_force, f), max(case_moment, mo), max(case_nw, nw)
    case_normal = {b: D(0) for b in line.bases}
    for m, fwd in line.order:
        for basis in line.bases:
            nm = max(an.q[(m.id, st, basis)][("max_abs_normal",)] for st, fr in STATIONS)
            sh = max(an.q[(m.id, st, basis)][("sv_max",)] + abs(an.q[(m.id, st, basis)][("torsional_shear",)])
                     for st, fr in STATIONS)
            zs[m.id][basis] = {"normal_stress": nm, "shear_stress": sh}
            if (m.id, basis) in an.hoop:
                h = an.hoop[(m.id, basis)]
                zs[m.id][basis]["pressure_stress"] = max([abs(h["p"])] + [abs(v) for v in h["values"].values()])
            case_normal[basis] = max(case_normal[basis], nm)
    for m, fwd in line.order:
        if zs[m.id]["force"] == 0:
            zs[m.id]["force"] = case_force if case_force != 0 else case_nw
        if zs[m.id]["moment"] == 0:
            zs[m.id]["moment"] = case_moment
        for basis in line.bases:
            z = zs[m.id][basis]
            if z["normal_stress"] == 0:
                z["normal_stress"] = case_normal[basis]
            if z["shear_stress"] == 0:
                z["shear_stress"] = z["normal_stress"]
    an.case_normal = case_normal
    an.zs = zs
    return zs


def tol(exp, zs):
    return D("1e-9") * max(abs(exp), zs)


# ---------------------------------------------------------------------------
# Invented cases (request-agnostic case descriptions)
# ---------------------------------------------------------------------------

PROV = "invented T4-U4b reference input; not library, catalog, component or code-rule data"
ANCHOR = ["UX", "UY", "UZ", "RX", "RY", "RZ"]
BOW_RULE = "the arc bows toward +y_reference projected normal to the chord (the centre lies on the opposite side)"
K_NOTE = ("invented user flexibility factor; no frozen value depends on it because the line is statically "
          "determinate (one anchor, free tip)")
SIF_NOTE = ("invented user-supplied directional factors, not code or library data: i_ip multiplies the in-plane "
            "moment M_z and i_op the out-of-plane moment M_y of the arc's tangent frame")
SHEAR_NOTE = ("every frozen value is identical under euler_bernoulli and timoshenko_annulus_energy_v1: the line "
              "is statically determinate, and realized arcs are Euler-Bernoulli under either model")


def straight(mid, a, b, yref, sec, mat):
    return {"id": mid, "from": a, "to": b, "kind": "straight", "section": sec, "material": mat,
            "y_reference": yref}


BEND_ANGLE_NOTE = ("the binary64 nearest to the included angle 2 atan2(s, c), s = chord/(2R), c = sqrt(1 - s^2), "
                   "formed from the binary64 node positions and the decimal R (filled by the generator); the bend "
                   "component states it")
ATTACH_NOTE = "the bend component is attached at the arc's authored from node, as in U4a"


def bend(mid, a, b, yref, sec, mat, R, k, ip, op):
    return {"id": mid, "from": a, "to": b, "kind": "realized_arc", "section": sec, "material": mat,
            "y_reference": yref,
            "arc": {"bend_radius": R, "bend_angle_binary64_for_authoring": None, "bend_angle_note": BEND_ANGLE_NOTE,
                    "bend_component_attachment_node": a, "attachment_note": ATTACH_NOTE,
                    "bow_rule": BOW_RULE, "solver_consumption": "curved_bend_macro_element",
                    "flexibility_factor_user_value": k, "flexibility_factor_note": K_NOTE},
            "directional_sif_user_values": {"i_ip": ip, "i_op": op, "note": SIF_NOTE}}


def fill_bend_angles(doc):
    """Write each arc's bend angle from its own geometry (after any rigid motion)."""
    nodes = {n["id"]: tuple(b64(x) for x in n["position"]) for n in doc["nodes"]}
    for mm in doc["members"]:
        if mm["kind"] != "realized_arc":
            continue
        m = Member(mm["id"], mm["from"], mm["to"], "arc", tuple(b64(x) for x in mm["y_reference"]), None, None,
                   R=dtarget(mm["arc"]["bend_radius"]))
        m.setup(nodes)
        mm["arc"]["bend_angle_binary64_for_authoring"] = float(m.phi)
        mm["arc"]["bend_component_attachment_node"] = mm["from"]


def section_doc(sid, od, wall, cm, ca):
    d = {"id": sid, "outside_diameter": od, "wall_thickness": wall, "mill_tolerance": cm}
    if ca is not None:
        d["corrosion_allowance"] = ca
    return d


def material_doc(mid, E, nu, alpha):
    return {"id": mid, "constitutive_basis": "homogeneous_isotropic_E_nu_v1", "elastic_modulus": E,
            "poisson_ratio": nu, "thermal_expansion_coefficient": alpha}


def base_doc(case_id, route, description):
    d = {"case_id": case_id, "description": description, "route": route,
         "semantic_identity_when_defined": "pressure-2" if route == "exact" else "preview-physics-2",
         "model_version_when_defined": "0.6.0"}
    return d


def loads_block(forces, moments, member_loads, pressure=None):
    lc = {"id": "case:u4b", "thermal_state": "unchanged_reference (no temperature change)",
          "self_weight": "none (only the listed loads act)",
          "nodal_forces": [{"id": "load:force:%s" % n, "node": n, "global_components": list(v)}
                           for n, v in forces],
          "nodal_moments": [{"id": "load:moment:%s" % n, "node": n, "global_components": list(v)}
                            for n, v in moments],
          "member_uniform_forces": [{"id": "load:line:%s" % mm, "member": mm,
                                     "global_components_per_length": list(v), "length_measure": lm}
                                    for mm, v, lm in member_loads],
          "pressure_regions": []}
    if pressure is not None:
        p, members, a, b = pressure
        lc["pressure_regions"] = [{
            "id": "region:u4b", "member_ids": members,
            "pressure_basis": "internal_differential_zero_external_v1", "pressure": p,
            "terminals": [{"node": a, "closure_transfer": "transfers_to_wall"},
                          {"node": b, "closure_transfer": "transfers_to_wall"}]}]
    return lc


def doc_L90(case_id, cm, ca, shear_model, description):
    d = base_doc(case_id, "exact", description)
    d["pressure_contract"] = {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": shear_model,
                              "shear_model_note": SHEAR_NOTE}
    d["nodes"] = [{"id": "node:A", "position": [0.0, 0.0, 0.0]},
                  {"id": "node:B", "position": [2.5, 0.0, 0.0]},
                  {"id": "node:C", "position": [3.0, 0.5, 0.0]},
                  {"id": "node:D", "position": [3.0, 2.5, 0.0]}]
    d["members"] = [straight("pipe:S1", "node:A", "node:B", [0.0, 0.0, 1.0], "section:main", "material:main"),
                    bend("pipe:BEND", "node:B", "node:C", [1.0, -1.0, 0.0], "section:main", "material:main",
                         0.5, 3.1, 2.4, 1.9),
                    straight("pipe:S2", "node:C", "node:D", [0.0, 0.0, 1.0], "section:main", "material:main")]
    d["sections"] = [section_doc("section:main", 0.2191, 0.00818, cm, ca)]
    d["materials"] = [material_doc("material:main", 2.03e11, 0.27, 1.17e-05)]
    d["supports"] = [{"id": "support:A", "node": "node:A", "family": "anchor", "restraints": ANCHOR}]
    d["load_case"] = loads_block(
        [("node:D", (1800.0, -1100.0, 2400.0))], [("node:D", (-500.0, 750.0, 900.0))],
        [("pipe:S1", (0.0, 0.0, -420.0), "member_length"), ("pipe:BEND", (150.0, -90.0, -420.0), "arc_length"),
         ("pipe:S2", (80.0, 0.0, -420.0), "member_length")],
        (4200000.0, ["pipe:S1", "pipe:BEND", "pipe:S2"], "node:A", "node:D"))
    return d


ROT_Q = [[Fraction(13, 39), Fraction(-34, 39), Fraction(-14, 39)],
         [Fraction(26, 39), Fraction(19, 39), Fraction(-22, 39)],
         [Fraction(26, 39), Fraction(-2, 39), Fraction(29, 39)]]
ROT_T = [Fraction(24690, 20), Fraction(-2269, 4), Fraction(713, 8)]   # (1234.5, -567.25, 89.125)


def rot(v, translate=False):
    fv = [Fraction(float(x)) for x in v]
    out = [sum(ROT_Q[i][j] * fv[j] for j in range(3)) for i in range(3)]
    if translate:
        out = [out[i] + ROT_T[i] for i in range(3)]
    return [float(x) for x in out]


def doc_moved(src, case_id, description):
    d = json.loads(json.dumps(src))
    d["case_id"] = case_id
    d["description"] = description
    d["rigid_motion_of"] = {"case_id": src["case_id"],
                            "rotation_rows": [[str(x) for x in row] for row in ROT_Q],
                            "translation": [str(x) for x in ROT_T],
                            "rounding": "positions, y_reference and load components are the binary64 values "
                                        "nearest to the exactly rotated (and, for positions, translated) "
                                        "source values"}
    for n in d["nodes"]:
        n["position"] = rot(n["position"], True)
    for m in d["members"]:
        m["y_reference"] = rot(m["y_reference"])
    lc = d["load_case"]
    for f in lc["nodal_forces"] + lc["nodal_moments"]:
        f["global_components"] = rot(f["global_components"])
    for f in lc["member_uniform_forces"]:
        f["global_components_per_length"] = rot(f["global_components_per_length"])
    return d


def doc_arc106():
    d = base_doc("U4B-ARC106-RADIAL-NEGP", "exact",
                 "single realized arc of 2*asin(4/5) (about 106.26 deg), long radius R/r_o = 18.57, anchored at "
                 "A, free tip B; radial tip force toward the centre; signed pressure -0.6 MPa with transferring "
                 "closures; the bending and maximum peak lies between the end_i and quarter_1 stations")
    d["pressure_contract"] = {"version": "3.0.0", "mode": "exact_pressure_v3",
                              "shear_model": "timoshenko_annulus_energy_v1", "shear_model_note": SHEAR_NOTE}
    d["nodes"] = [{"id": "node:A", "position": [0.0, 0.0, 0.0]},
                  {"id": "node:B", "position": [1.5, 2.0, 0.0]}]
    d["members"] = [bend("pipe:BEND", "node:A", "node:B", [4.0, -3.0, 0.0], "section:arc106", "material:arc106",
                         1.5625, 1.6, 2.0, 1.5)]
    d["sections"] = [section_doc("section:arc106", 0.1683, 0.00711, 0.00089, 0.0012)]
    d["materials"] = [material_doc("material:arc106", 1.95e11, 0.31, 1.2e-05)]
    d["supports"] = [{"id": "support:A", "node": "node:A", "family": "anchor", "restraints": ANCHOR}]
    d["load_case"] = loads_block([("node:B", (-7200.0, -2100.0, 0.0))], [], [],
                                 (-600000.0, ["pipe:BEND"], "node:A", "node:B"))
    return d


def doc_tight():
    d = base_doc("U4B-ARC90-TIGHT-NSIGN-P", "exact",
                 "single realized 90 deg arc with a tight radius R/r_o = 1.5, anchored at A, free tip B; "
                 "tip force and moment; 0.8 MPa with transferring closures; the wall axial force N_w changes "
                 "sign along the arc and the maximum lies at the anchored end")
    d["pressure_contract"] = {"version": "3.0.0", "mode": "exact_pressure_v3",
                              "shear_model": "timoshenko_annulus_energy_v1", "shear_model_note": SHEAR_NOTE}
    d["nodes"] = [{"id": "node:A", "position": [0.0, 0.0, 0.0]},
                  {"id": "node:B", "position": [0.1875, 0.1875, 0.0]}]
    d["members"] = [bend("pipe:BEND", "node:A", "node:B", [1.0, -1.0, 0.0], "section:tight", "material:tight",
                         0.1875, 6.5, 2.6, 2.2)]
    d["sections"] = [section_doc("section:tight", 0.25, 0.0125, 0.001, 0.002)]
    d["materials"] = [material_doc("material:tight", 2.0e11, 0.29, 1.15e-05)]
    d["supports"] = [{"id": "support:A", "node": "node:A", "family": "anchor", "restraints": ANCHOR}]
    d["load_case"] = loads_block([("node:B", (-20000.0, -60000.0, 5000.0))],
                                 [("node:B", (300.0, -400.0, -2000.0))], [],
                                 (800000.0, ["pipe:BEND"], "node:A", "node:B"))
    return d


def doc_oop_preview():
    d = base_doc("U4B-ARC90-OOP-PREVIEW", "preview",
                 "pressure-free preview-physics route: single realized 90 deg arc (R/r_o = 13.12), anchored at A, "
                 "free tip B; tip force normal to the bend plane, tip moments about all three axes, and a "
                 "uniform load per unit arc length with in-plane and out-of-plane components; one section, "
                 "wall minus mill tolerance")
    d["pressure_contract"] = None
    d["nodes"] = [{"id": "node:A", "position": [0.0, 0.0, 0.0]},
                  {"id": "node:B", "position": [0.75, 0.75, 0.0]}]
    d["members"] = [bend("pipe:BEND", "node:A", "node:B", [1.0, -1.0, 0.0], "section:oop", "material:oop",
                         0.75, 4.2, 1.8, 1.4)]
    d["sections"] = [section_doc("section:oop", 0.1143, 0.00602, 0.00075, None)]
    d["materials"] = [material_doc("material:oop", 2.0e11, 0.3, 1.2e-05)]
    d["supports"] = [{"id": "support:A", "node": "node:A", "family": "anchor", "restraints": ANCHOR}]
    d["load_case"] = loads_block([("node:B", (0.0, 0.0, 1500.0))], [("node:B", (200.0, -350.0, 120.0))],
                                 [("pipe:BEND", (100.0, 50.0, -800.0), "arc_length")])
    return d


def doc_z_preview():
    d = base_doc("U4B-Z-PREVIEW-REV", "preview",
                 "pressure-free preview-physics route: planar Z line anchored at A, free tip F; a left-turn arc "
                 "authored against the traversal (C to B), a right-turn arc authored along it, and the last "
                 "straight authored against it; tip force and moment and uniform loads on every member")
    d["pressure_contract"] = None
    d["nodes"] = [{"id": "node:A", "position": [0.0, 0.0, 0.0]},
                  {"id": "node:B", "position": [2.0, 0.0, 0.0]},
                  {"id": "node:C", "position": [2.5, 0.5, 0.0]},
                  {"id": "node:D", "position": [2.5, 2.0, 0.0]},
                  {"id": "node:E", "position": [3.0, 2.5, 0.0]},
                  {"id": "node:F", "position": [4.5, 2.5, 0.0]}]
    d["members"] = [straight("pipe:S1", "node:A", "node:B", [0.0, 0.0, 1.0], "section:z", "material:z"),
                    bend("pipe:BEND1", "node:C", "node:B", [1.0, -1.0, 0.0], "section:z", "material:z",
                         0.5, 2.8, 2.2, 1.6),
                    straight("pipe:S2", "node:C", "node:D", [0.0, 0.0, 1.0], "section:z", "material:z"),
                    bend("pipe:BEND2", "node:D", "node:E", [-1.0, 1.0, 0.0], "section:z", "material:z",
                         0.5, 2.8, 1.7, 2.5),
                    straight("pipe:S3", "node:F", "node:E", [0.0, 0.0, 1.0], "section:z", "material:z")]
    d["sections"] = [section_doc("section:z", 0.1683, 0.00711, 0.0009, None)]
    d["materials"] = [material_doc("material:z", 1.98e11, 0.26, 1.25e-05)]
    d["supports"] = [{"id": "support:A", "node": "node:A", "family": "anchor", "restraints": ANCHOR}]
    d["load_case"] = loads_block(
        [("node:F", (-900.0, 1300.0, 2100.0))], [("node:F", (400.0, -250.0, 650.0))],
        [("pipe:S1", (0.0, 0.0, -300.0), "member_length"), ("pipe:BEND1", (0.0, 0.0, -300.0), "arc_length"),
         ("pipe:S2", (60.0, 0.0, -300.0), "member_length"), ("pipe:BEND2", (-40.0, 70.0, -300.0), "arc_length"),
         ("pipe:S3", (0.0, 0.0, -300.0), "member_length")])
    return d


def doc_rise_rev():
    """Added in the repair revision (refutation finding 8): a pressurized arc authored against the traversal,
    and a straight whose maximum is interior. sin(phi/2) = 3/5 and R = 25/32 make every node dyadic."""
    d = base_doc("U4B-RISE74-P-REV", "exact",
                 "line in the vertical x-z plane, anchored at A, free tip D: straight S1 along +x; a realized arc of "
                 "2*asin(3/5) (about 73.74 deg) rising toward +z, authored against the traversal (C to B), R/r_o = "
                 "11.06; straight S2 along the arc's end tangent; 3.6 MPa with transferring closures over the whole "
                 "line; tip force and moment, uniform loads on every member (on S1 axial and in both transverse "
                 "planes, on the arc in-plane and out-of-plane); distinct section bases; nu = 0.28; the maximum of "
                 "S1 is interior (fraction about 0.4336), where the uniform load and the opposing beyond-shear give "
                 "an interior bending peak")
    d["pressure_contract"] = {"version": "3.0.0", "mode": "exact_pressure_v3",
                              "shear_model": "timoshenko_annulus_energy_v1", "shear_model_note": SHEAR_NOTE}
    d["nodes"] = [{"id": "node:A", "position": [0.0, 0.0, 0.0]},
                  {"id": "node:B", "position": [2.0, 0.0, 0.0]},
                  {"id": "node:C", "position": [2.75, 0.0, 0.5625]},
                  {"id": "node:D", "position": [3.1875, 0.0, 2.0625]}]
    d["members"] = [straight("pipe:S1", "node:A", "node:B", [0.0, 0.0, 1.0], "section:rise", "material:rise"),
                    bend("pipe:BEND", "node:C", "node:B", [1.0, 0.0, -1.0], "section:rise", "material:rise",
                         0.78125, 3.4, 2.1, 1.7),
                    straight("pipe:S2", "node:C", "node:D", [0.0, 1.0, 0.0], "section:rise", "material:rise")]
    d["sections"] = [section_doc("section:rise", 0.1413, 0.00655, 0.00082, 0.0013)]
    d["materials"] = [material_doc("material:rise", 1.93e11, 0.28, 1.6e-05)]
    d["supports"] = [{"id": "support:A", "node": "node:A", "family": "anchor", "restraints": ANCHOR}]
    d["load_case"] = loads_block(
        [("node:D", (600.0, -800.0, 1850.0))], [("node:D", (-250.0, 300.0, 180.0))],
        [("pipe:S1", (150.0, 800.0, -1200.0), "member_length"), ("pipe:BEND", (120.0, -200.0, -350.0), "arc_length"),
         ("pipe:S2", (0.0, 0.0, -250.0), "member_length")],
        (3600000.0, ["pipe:S1", "pipe:BEND", "pipe:S2"], "node:A", "node:D"))
    return d


def case_documents():
    l90 = doc_L90("U4B-L90-P", 0.00102, 0.0015, "timoshenko_annulus_energy_v1",
                  "planar L line (straight, realized 90 deg left-turn arc R/r_o = 4.56, straight), anchored at A, "
                  "free tip D; 4.2 MPa with transferring closures; tip force and moment, uniform loads on every "
                  "member (out-of-plane and in-plane on the arc); distinct section bases")
    twin = doc_L90("U4B-L90-P-COINCIDENT", 0.0, 0.0, "euler_bernoulli",
                   "U4B-L90-P with zero mill tolerance and zero corrosion allowance (the two bases coincide) "
                   "and the euler_bernoulli shear model")
    moved = doc_moved(l90, "U4B-L90-P-MOVED",
                      "U4B-L90-P rigidly rotated (rational rotation, every axis skew) and translated, with its "
                      "y_reference and loads rotated: every frame-local value equals U4B-L90-P's up to the "
                      "binary64 rounding of the moved inputs")
    docs = [l90, twin, moved, doc_arc106(), doc_tight(), doc_oop_preview(), doc_z_preview(), doc_rise_rev()]
    for d in docs:
        fill_bend_angles(d)
    return json.loads(json.dumps(docs))   # exactly what cases.json holds


def line_from_doc(doc):
    nodes = {n["id"]: tuple(b64(x) for x in n["position"]) for n in doc["nodes"]}
    secs = {}
    for s in doc["sections"]:
        secs[s["id"]] = Section(s["id"], dtarget(s["outside_diameter"]), dtarget(s["wall_thickness"]),
                                dtarget(s.get("mill_tolerance", 0)), dtarget(s.get("corrosion_allowance", 0)))
    mats = {m["id"]: {"E": dtarget(m["elastic_modulus"]), "nu": dtarget(m["poisson_ratio"])}
            for m in doc["materials"]}
    members = []
    for mm in doc["members"]:
        yref = tuple(b64(x) for x in mm["y_reference"])
        if mm["kind"] == "realized_arc":
            sif = mm.get("directional_sif_user_values")
            members.append(Member(mm["id"], mm["from"], mm["to"], "arc", yref, mm["section"], mm["material"],
                                  R=dtarget(mm["arc"]["bend_radius"]),
                                  k=dtarget(mm["arc"]["flexibility_factor_user_value"]),
                                  sif=(dtarget(sif["i_ip"]), dtarget(sif["i_op"])) if sif else None))
        else:
            members.append(Member(mm["id"], mm["from"], mm["to"], "straight", yref, mm["section"],
                                  mm["material"]))
    sup = [s for s in doc["supports"] if s["family"] == "anchor"]
    assert len(sup) == 1 and len(doc["supports"]) == 1 and sup[0]["restraints"] == ANCHOR
    lc = doc["load_case"]
    nf, nm, ml = {}, {}, {}
    for f in lc["nodal_forces"]:
        nf[f["node"]] = vadd(nf.get(f["node"], ZV), tuple(b64(x) for x in f["global_components"]))
    for f in lc["nodal_moments"]:
        nm[f["node"]] = vadd(nm.get(f["node"], ZV), tuple(b64(x) for x in f["global_components"]))
    for f in lc["member_uniform_forces"]:
        ml[f["member"]] = vadd(ml.get(f["member"], ZV), tuple(b64(x) for x in f["global_components_per_length"]))
    pressure = None
    if lc["pressure_regions"]:
        (reg,) = lc["pressure_regions"]
        pressure = {"p": dtarget(reg["pressure"]), "members": set(reg["member_ids"]),
                    "terminals": {t["node"]: t["closure_transfer"] for t in reg["terminals"]}}
    line = Line(doc["case_id"], doc["route"], nodes, members, secs, mats, sup[0]["node"], nf, nm, ml, pressure,
                doc.get("description", ""))
    for mm in doc["members"]:
        if mm["kind"] == "realized_arc":
            require(mm["arc"]["bend_angle_binary64_for_authoring"] == float(line.mby[mm["id"]].phi)
                    and mm["arc"]["bend_component_attachment_node"] == mm["from"], "bend authoring fields " + mm["id"])
    check_pressure_premise(line)
    return line


def check_pressure_premise(line):
    """N_w = S + p*Ai at every section needs the region to cover the whole line
    with a closure transferring to the wall at the free tip (README section 3)."""
    if line.pressure is None:
        return
    assert line.pressure["members"] == set(m.id for m in line.members), "region must cover the line"
    assert line.pressure["terminals"].get(line.tip) == "transfers_to_wall", "tip closure must transfer"
    assert line.pressure["terminals"].get(line.anchor) == "transfers_to_wall", "anchor closure must transfer"


# U4a requests read as data --------------------------------------------------

def line_from_u4a(cid, req):
    m = req["model"]
    nodes = {n["id"]: tuple(b64(n["position"][a]) for a in "xyz") for n in m["nodes"]}
    comps = {c["geometry"]["bend_pipe_ref"]: c for c in m.get("components", []) if c.get("kind") == "bend"}
    mats = {}
    for mt in m["materials"]:
        E = dtarget(mt["elastic_modulus"]["value"])
        if "poisson_ratio" in mt:
            nu = dtarget(mt["poisson_ratio"]["value"])
        else:   # 0.2.0 documents state G; nu enters no U4a cross-checked row
            nu = E / (2 * dtarget(mt["shear_modulus"]["value"])) - 1
        mats[mt["id"]] = {"E": E, "nu": nu}
    secs, members = {}, []
    for p in m["pipe_segments"]:
        s = p["section"]
        sid = "section:" + p["id"]
        secs[sid] = Section(sid, dtarget(s["outside_diameter"]["value"]), dtarget(s["wall_thickness"]["value"]),
                            dtarget(s.get("mill_tolerance", {}).get("value", 0)),
                            dtarget(s.get("corrosion_allowance", {}).get("value", 0)))
        yref = tuple(b64(p["y_reference"][a]) for a in "xyz")
        if p["id"] in comps:
            c = comps[p["id"]]
            assert c["mechanics_interface"]["solver_consumption"] == "curved_bend_macro_element"
            members.append(Member(p["id"], p["from"], p["to"], "arc", yref, sid, p["material"],
                                  R=dtarget(c["geometry"]["bend_radius"]["value"]),
                                  k=dtarget(c["modifiers"]["flexibility_factor_user_value"]["value"])))
        else:
            members.append(Member(p["id"], p["from"], p["to"], "straight", yref, sid, p["material"]))
    sups = m["supports"]
    assert len(sups) == 1 and sups[0]["family"] == "anchor" and sups[0]["restraints"] == ANCHOR
    (lc,) = m["load_cases"]
    nf, nm, ml = {}, {}, {}
    axis = {"global_x": 0, "global_y": 1, "global_z": 2, "rotation_x": 0, "rotation_y": 1, "rotation_z": 2}
    for ld in lc["primitive_loads"]:
        v = [D(0), D(0), D(0)]
        v[axis[ld["direction"]]] = b64(ld["magnitude"]["value"])
        v = tuple(v)
        cat = ld["category"]
        if cat == "concentrated_force":
            nf[ld["target"]["node"]] = vadd(nf.get(ld["target"]["node"], ZV), v)
        elif cat == "concentrated_moment":
            nm[ld["target"]["node"]] = vadd(nm.get(ld["target"]["node"], ZV), v)
        elif cat == "distributed_force":
            ml[ld["target"]["pipe"]] = vadd(ml.get(ld["target"]["pipe"], ZV), v)
        else:
            raise AssertionError("unexpected load category in a free-tip U4a case: " + cat)
    pressure = None
    if lc.get("pressure_regions"):
        (reg,) = lc["pressure_regions"]
        pressure = {"p": dtarget(reg["pressure"]["value"]), "members": set(reg["member_pipe_ids"]),
                    "terminals": {t["node_ref"]: t["closure_transfer"] for t in reg["terminals"]}}
    route = "exact" if "pressure_contract" in m else "preview"
    line = Line(cid, route, nodes, members, secs, mats, sups[0]["node"], nf, nm, ml, pressure)
    check_pressure_premise(line)
    return line


def u4a_free_tip(req):
    m = req["model"]
    sups = m.get("supports", [])
    return len(sups) == 1 and sups[0]["family"] == "anchor" and sups[0]["restraints"] == ANCHOR


# ---------------------------------------------------------------------------
# Emission of one case
# ---------------------------------------------------------------------------

def vec_str(v):
    return [fmt(snap(x)) for x in v]


def fibre_at(a, b, r):
    """The section fibre at radius r in the direction (a, b) of the (y, z) plane:
    (beta_deg, y, z) with (y, z) = r (a, b) / hypot(a, b), formed from the vector
    so that a component that is exactly zero stays exactly zero. beta_deg is
    informational, in (-180, 180]: exactly 0, 90, 180 or -90 on an axis, so a
    fibre on the -y axis (z = 0, y < 0) is +180, never -180."""
    h = (a * a + b * b).sqrt()
    y, z = snap(r * a / h), snap(r * b / h)
    if z == 0:
        bdeg = D(0) if y > 0 else D(180)
    elif y == 0:
        bdeg = D(90) if z > 0 else D(-90)
    else:
        bdeg = deg(atan2(z, y))
    return bdeg, y, z


def fibre_obj(a, b, r):
    bdeg, y, z = fibre_at(a, b, r)
    return {"beta_deg": V(bdeg), "y": V(y), "z": V(z)}


def sample_refine_max(line, k, prof, value_of_r):
    """Maximum of an auxiliary (mutant) objective along an arc: dense samples,
    then a golden-section refinement around the best sample."""
    vals = [value_of_r(r) for (psi, r) in prof]
    best = max(vals)
    i = vals.index(best)
    a = prof[max(i - 1, 0)][0]
    b = prof[min(i + 1, len(prof) - 1)][0]
    ref = golden_max(lambda x: value_of_r(line.arc_resultants_psi(k, x)), a, b)
    return max(best, ref)


def straight_sample_max(poly_fn, n=512):
    best = None
    for i in range(n + 1):
        v = poly_fn(D(i) / n)
        best = v if best is None or v > best else best
    return best


def emit_case(an, doc):
    line = an.line
    zs = zero_scales(an)
    disc = []          # (mutant, path, expected, wrong, zero_scale)
    out = {"case_id": line.id, "route": line.route,
           "semantic_identity_when_defined": "pressure-2" if line.route == "exact" else "preview-physics-2",
           "description": line.description,
           "anchor": line.anchor, "free_tip": line.tip,
           "traversal": [{"member": m.id, "matches_authored": fwd} for m, fwd in line.order]}
    # ---- derived geometry
    der = {}
    for m, fwd in line.order:
        d = {"kind": "realized_arc" if m.kind == "arc" else "straight",
             "length_m": V(m.length), "nu": V(line.nu(m.id))}
        if m.kind == "arc":
            d["arc"] = {"R": V(m.R), "half_angle_sin": V(m.s), "half_angle_cos": V(m.c),
                        "included_angle_rad": V(m.phi), "included_angle_deg": V(deg(m.phi)),
                        "bend_angle_binary64_for_authoring": float(m.phi),
                        "centre": vec_str(m.centre), "bend_plane_normal_z": vec_str(m.ez),
                        "R_over_ro": V(m.R / (line.sections[m.section].od / 2)),
                        "flexibility_factor_user_value": V(m.k)}
            if m.sif is not None:
                d["directional_sif"] = {"i_ip": V(m.sif[0]), "i_op": V(m.sif[1]),
                                        "i_ip_moment": "M_z", "i_op_moment": "M_y"}
        bases = {}
        for basis in line.bases:
            bp = line.basis_props[m.id][basis]
            bases[basis] = {"wall": V(bp["wall"]), "ri": V(bp["ri"]), "ro": V(bp["ro"]), "A": V(bp["A"]),
                            "I": V(bp["I"]), "J": V(bp["J"]), "Z": V(bp["Z"])}
        d["section_bases"] = bases
        if line.route == "exact":
            bore = line.basis_props[m.id]["_bore"]
            d["pressure_bore"] = {"ri": V(bore["ri"]), "Ai": V(bore["Ai"])}
        if line.pressurized(m.id):
            d["pressure"] = V(line.pressure["p"])
            d["p_Ai"] = V(line.pAi(m.id))
        frames = {}
        for st, fr in STATIONS:
            ex, ey, ez = an.res[m.id][st]["_frame"]
            frames[st] = {"x": vec_str(ex), "y": vec_str(ey), "z": vec_str(ez),
                          "point": vec_str(an.res[m.id][st]["_P"])}
        d["tangent_frames_global_components"] = frames
        der[m.id] = d
    out["derived"] = der
    # ---- zero scales
    zso = {}
    for m, fwd in line.order:
        zz = {"force": V(zs[m.id]["force"]), "moment": V(zs[m.id]["moment"])}
        for basis in line.bases:
            zz[basis] = {kk: V(vv) for kk, vv in zs[m.id][basis].items()}
        zso[m.id] = zz
    out["zero_scales"] = zso
    # ---- resultants
    resj = {}
    for m, fwd in line.order:
        resj[m.id] = {}
        pAi = line.pAi(m.id)
        for st, fr in STATIONS:
            r = an.res[m.id][st]
            if line.pressurized(m.id):
                row = {"S": V(r["N"]), "N_w": V(r["N"] + pAi)}
            else:
                row = {"N": V(r["N"])}
            for kk in ["V_y", "V_z", "T", "M_y", "M_z"]:
                row[kk] = V(r[kk])
            resj[m.id][st] = row
    q = {"resultants_tangent_frame_j_side": resj}
    # ---- station quantities
    fibre_j, ext_j, svmax_j, sv_j, tors_j, ft_j, int_j, api_j = {}, {}, {}, {}, {}, {}, {}, {}
    other = {"nominal_section": "reduced_section", "reduced_section": "nominal_section"}
    distinct = line.route == "exact"
    for m, fwd in line.order:
        for st, fr in STATIONS:
            for basis in line.bases:
                key3 = (m.id, st, basis)
                qq, mu, aux = an.q[key3], an.mut[key3], an.aux[key3]
                bp = line.basis_props[m.id][basis]
                fj = {"outer": {}, "inner": {}}
                for (qn, surf, name) in [k for k in qq if k[0] == "fibre_normal"]:
                    fj[surf][name] = V(qq[(qn, surf, name)])
                fibre_j.setdefault(m.id, {}).setdefault(st, {})[basis] = fj
                ext = aux["ext"]
                e = {"sigma_plus": V(qq[("sigma_plus",)]), "sigma_minus": V(qq[("sigma_minus",)]),
                     "max_abs_normal": V(qq[("max_abs_normal",)])}
                if ext["tied"]:
                    e["fibres"] = "tied: bending is zero, every outer fibre carries N/A"
                else:
                    pa, pb = ext["plus_dir"]
                    e["sigma_plus_fibre"] = fibre_obj(pa, pb, bp["ro"])
                    e["sigma_minus_fibre"] = fibre_obj(-pa, -pb, bp["ro"])
                ext_j.setdefault(m.id, {}).setdefault(st, {})[basis] = e
                s = {"value": V(qq[("sv_max",)]), "surface": "inner"}
                if aux["sv_fibres"] is None:
                    s["fibres"] = "tied: V_y = V_z = 0, the flexure field vanishes"
                else:
                    # an unordered pair, listed by ascending beta_deg
                    fl = sorted((fibre_at(a_, b_, bp["ri"]) for a_, b_ in aux["sv_fibres"]), key=lambda t_: t_[0])
                    s["fibres"] = [{"beta_deg": V(bd), "y": V(y_), "z": V(z_)} for bd, y_, z_ in fl]
                svmax_j.setdefault(m.id, {}).setdefault(st, {})[basis] = s
                sj = {"outer": {}, "inner": {}}
                for surf in ("outer", "inner"):
                    for name, ang in FIBRES:
                        sj[surf][name] = {"tau_xy": V(qq[("sv", surf, name, "tau_xy")]),
                                          "tau_xz": V(qq[("sv", surf, name, "tau_xz")])}
                sv_j.setdefault(m.id, {}).setdefault(st, {})[basis] = sj
                tors_j.setdefault(m.id, {}).setdefault(st, {})[basis] = V(qq[("torsional_shear",)])
                ftj = {}
                for name in NAMED_FIBRES:
                    a_, b_ = qq[("flexure_plus_torsion", name, "tau_xy")], qq[("flexure_plus_torsion", name, "tau_xz")]
                    ftj[name] = {"tau_xy": V(a_), "tau_xz": V(b_), "magnitude": V((a_ * a_ + b_ * b_).sqrt())}
                ft_j.setdefault(m.id, {}).setdefault(st, {})[basis] = ftj
                if ("intensified_bending",) in qq:
                    int_j.setdefault(m.id, {}).setdefault(st, {})[basis] = V(qq[("intensified_bending",)])
                    api_j.setdefault(m.id, {}).setdefault(st, {})[basis] = V(qq[("axial_plus_intensified_bending",)])
                # discriminators at this station
                zsn = zs[m.id][basis]["normal_stress"]
                zss = zs[m.id][basis]["shear_stress"]
                for mutant, vals in mu.items():
                    for kq, wrong in vals.items():
                        z = zsn if kq[0] in NORMAL_Q else zss
                        disc.append((mutant, row_path(kq, m.id, st, basis), qq[kq], wrong, z))
                if distinct:
                    oq = an.q[(m.id, st, other[basis])]
                    for kq, val in qq.items():
                        z = zsn if kq[0] in NORMAL_Q else zss
                        disc.append(("rows_other_basis_section", row_path(kq, m.id, st, basis), val, oq[kq], z))
    q["fibre_normal"] = fibre_j
    q["normal_extremes"] = ext_j
    q["sv_transverse_shear_max"] = svmax_j
    q["sv_transverse_shear"] = sv_j
    q["torsional_shear"] = tors_j
    q["flexure_plus_torsion_check"] = ft_j
    if int_j:
        q["intensified_bending"] = int_j
        q["axial_plus_intensified_bending"] = api_j
    # ---- hoop and radial
    if an.hoop:
        hj, rj = {}, {}
        for (mid, basis), h in an.hoop.items():
            m = line.mby[mid]
            z = zs[mid][basis]["pressure_stress"]
            fib = {name: V(v) for name, v in h["values"].items()}
            hj.setdefault(mid, {})[basis] = {
                "fibres": fib, "station_independent": True,
                "parameters": {"p": V(h["p"]), "ri": V(h["ri"]), "ro": V(h["ro"]), "t": V(h["t"]), "R": V(h["R"])},
                "straight_lame_wall_mean_p_ri_over_t": V(h["lame_mean"])}
            rj.setdefault(mid, {})[basis] = {"inner": V(-h["p"]), "outer": V(D(0)), "station_independent": True}
            p, ri, ro, R, t = h["p"], h["ri"], h["ro"], h["R"], h["t"]
            rm = (ri + ro) / 2
            for name, ang in FIBRES:
                cb, sb = fibre_cs(ang)
                path = "toroidal_membrane_hoop/%s/%s/%s" % (mid, basis, name)
                exp = h["values"][name]
                disc.append(("hoop_straight_lame_inner", path, exp, lame_hoop(p, ri, ro, ri), z))
                disc.append(("hoop_straight_lame_outer", path, exp, lame_hoop(p, ri, ro, ro), z))
                disc.append(("hoop_classical_mid_radius_thin", path, exp,
                             p * rm / (2 * t) * (2 * R - rm * cb) / (R - rm * cb), z))
                disc.append(("hoop_intrados_extrados_swapped", path, exp, toroidal_hoop(p, ri, ro, R, -cb), z))
                if basis == "nominal_section":
                    rib = line.basis_props[mid]["_bore"]["ri"]
                    if rib != ri:
                        disc.append(("hoop_variant_b_pressure_bore_on_nominal_wall", path, exp,
                                     toroidal_hoop_variant_b(p, rib, ri, ro, R, cb), z))
                if distinct:
                    oh = an.hoop[(mid, other[basis])]["values"][name]
                    disc.append(("rows_other_basis_section", path, exp, oh, z))
            disc.append(("radial_sign_flipped", "radial_surface/%s/%s/inner" % (mid, basis), -p, p, z))
        q["toroidal_membrane_hoop"] = hj
        q["radial_surface"] = rj
    # ---- maxima
    mm_j, im_j = {}, {}
    case_best = {b: None for b in line.bases}
    case_station_best = {b: D(0) for b in line.bases}
    for k, (m, fwd) in enumerate(line.order):
        pAi = line.pAi(m.id)
        for basis in line.bases:
            bp = line.basis_props[m.id][basis]
            mx = an.mmax[(m.id, basis)]
            z = zs[m.id][basis]["normal_stress"]
            path = "member_maximum/%s/%s" % (m.id, basis)
            if m.kind == "arc":
                rec, extra = emit_arc_max(line, k, m, mx, bp, pAi, z, "max_normal")
                prof, h = an.prof[m.id]
                # mutants
                disc.append(("max_station_only", path, mx["value"], mx["station_max"], z))
                sv_ = [an.res[m.id][st] for st in ("end_i", "midspan", "end_j")]
                qa = quad_through(*[(r["N"] + pAi) / bp["A"] for r in sv_])
                qb = quad_through(*[r["M_y"] / bp["Z"] for r in sv_])
                qc = quad_through(*[r["M_z"] / bp["Z"] for r in sv_])
                qv, qu = section6_maximum(qa, qb, qc)
                rec["mutant_values"] = {"quadratic_straight_method_on_arc": V(qv)}
                disc.append(("max_quadratic_straight_method_on_arc", path, mx["value"], qv, z))
                absum = sample_refine_max(line, k, prof, lambda r: abs(r["N"] + pAi) / bp["A"]
                                          + abs(r["M_y"]) / bp["Z"] + abs(r["M_z"]) / bp["Z"])
                disc.append(("max_abs_sum_objective", path, mx["value"], absum, z))
                kmax = sample_refine_max(line, k, prof, lambda r: abs(r["N"] + pAi) / bp["A"]
                                         + m.k * (r["M_y"] ** 2 + r["M_z"] ** 2).sqrt() / bp["Z"])
                disc.append(("k_multiplying_stress", path, mx["value"], kmax, z))
                if (m.id, basis) in an.imax:
                    ix = an.imax[(m.id, basis)]
                    irec, _ = emit_arc_max(line, k, m, ix, bp, pAi, z, "intensified")
                    ipath = "intensified_bending_maximum/%s/%s" % (m.id, basis)
                    disc.append(("max_station_only", ipath, ix["value"], ix["station_max"], z))
                    ip, op = m.sif
                    sw = sample_refine_max(line, k, prof, lambda r: ((op * r["M_z"]) ** 2 + (ip * r["M_y"]) ** 2).sqrt() / bp["Z"])
                    disc.append(("sif_factors_swapped", ipath, ix["value"], sw, z))
                    bmax = sample_refine_max(line, k, prof, lambda r: (r["M_y"] ** 2 + r["M_z"] ** 2).sqrt() / bp["Z"])
                    disc.append(("sif_one_scalar_for_both_directions", ipath, ix["value"], max(ip, op) * bmax, z))
                    disc.append(("k_multiplying_stress", ipath, ix["value"], m.k * bmax, z))
                    im_j.setdefault(m.id, {})[basis] = irec
            else:
                rec = emit_straight_max(line, k, m, mx, bp, pAi, z)
                st_vals = [an.q[(m.id, st, basis)][("max_abs_normal",)] for st, fr in STATIONS]
                disc.append(("max_station_only", path, mx["value"], max(st_vals), z))
                a, b, c = mx["polys"]
                absum = straight_sample_max(lambda t: abs(pval(a, t)) + abs(pval(b, t)) + abs(pval(c, t)), 4096)
                disc.append(("max_abs_sum_objective", path, mx["value"], absum, z))
            mm_j.setdefault(m.id, {})[basis] = rec
            stmax = max(an.q[(m.id, st, basis)][("max_abs_normal",)] for st, fr in STATIONS)
            case_station_best[basis] = max(case_station_best[basis], stmax)
            cand = (mx["value"], m.id, rec)
            if case_best[basis] is None or cand[0] > case_best[basis][0]:
                case_best[basis] = cand
    if distinct:
        for m, fwd in line.order:
            for basis in line.bases:
                z = zs[m.id][basis]["normal_stress"]
                disc.append(("rows_other_basis_section", "member_maximum/%s/%s" % (m.id, basis),
                             an.mmax[(m.id, basis)]["value"], an.mmax[(m.id, other[basis])]["value"], z))
                if (m.id, basis) in an.imax:
                    disc.append(("rows_other_basis_section", "intensified_bending_maximum/%s/%s" % (m.id, basis),
                                 an.imax[(m.id, basis)]["value"], an.imax[(m.id, other[basis])]["value"], z))
    q["member_maximum"] = mm_j
    if im_j:
        q["intensified_bending_maximum"] = im_j
    head = {}
    for basis in line.bases:
        val, mid, rec = case_best[basis]
        co = [m.id for m, f in line.order
              if abs(an.mmax[(m.id, basis)]["value"] - val) <= val * D(10) ** -50]
        zc = an.case_normal[basis]
        hd = {"value": V(val), "member": mid, "co_governing_members": co, "location": rec["location"]}
        if "location_band" in rec:
            # an interior headline carries its member's band: the value is at least every
            # station value of the case, so 1e-9 max(value, zero scale) is the same tolerance
            # under the member's and the headline's zero scale
            require(val >= zc and val >= zs[mid][basis]["normal_stress"], "headline band tolerance")
            hd["location_band"] = rec["location_band"]
        hd["governing_fibre"] = rec["governing_fibre"]
        head[basis] = hd
        disc.append(("max_station_only", "headline/%s" % basis, val, case_station_best[basis], zc))
        if distinct:
            disc.append(("rows_other_basis_section", "headline/%s" % basis, val, case_best[other[basis]][0], zc))
    q["headline"] = head
    out["quantities"] = q
    # ---- N_w sign changes
    if an.nsign:
        out["effective_axial_force_zeros_on_arcs"] = {
            mid: [{"psi_rad": V(z_), "fraction": V(z_ / line.mby[mid].phi)} for z_ in zz]
            for mid, zz in an.nsign.items()}
    out["discriminators"] = summarize_discriminators(disc)
    return out, disc


def row_path(kq, mid, st, basis):
    name = kq[0]
    if name == "fibre_normal":
        return "fibre_normal/%s/%s/%s/%s/%s" % (mid, st, basis, kq[1], kq[2])
    if name in ("sigma_plus", "sigma_minus", "max_abs_normal"):
        return "normal_extremes/%s/%s/%s/%s" % (mid, st, basis, name)
    if name == "sv_max":
        return "sv_transverse_shear_max/%s/%s/%s/value" % (mid, st, basis)
    if name == "sv":
        return "sv_transverse_shear/%s/%s/%s/%s/%s/%s" % (mid, st, basis, kq[1], kq[2], kq[3])
    if name == "torsional_shear":
        return "torsional_shear/%s/%s/%s" % (mid, st, basis)
    if name == "flexure_plus_torsion":
        return "flexure_plus_torsion_check/%s/%s/%s/%s/%s" % (mid, st, basis, kq[1], kq[2])
    return "%s/%s/%s/%s" % (name, mid, st, basis)


def location_fields(line, k, m, psi, kind):
    phi = m.phi
    if kind == "end" or psi == 0 or psi == phi:
        lab = "end_i" if psi == 0 else "end_j"
        return {"kind": lab, "psi_rad": V(psi), "fraction": V(psi / phi)}
    return {"kind": "interior", "psi_rad": V(psi), "fraction": V(psi / phi),
            "psi_deg": V(deg(psi))}


def emit_arc_max(line, k, m, mx, bp, pAi, z, objective):
    psi = mx["psi"]
    r = line.arc_resultants_psi(k, psi)
    Ne = r["N"] + pAi
    loc = location_fields(line, k, m, psi, mx["kind"])
    rec = {"value": V(mx["value"]),
           "method": "closed-form arc resultants; both ends and every interior root of the exact derivative "
                     "(sign change + to - on the dense grid, refined by bisection to the working precision)",
           "location": loc}
    if objective == "max_normal":
        gd = fibre_of_max(Ne, r["M_y"], r["M_z"])
        rec["governing_fibre"] = ({"surface": "outer", "sign": "sigma_plus" if Ne >= 0 else "sigma_minus",
                                   **fibre_obj(gd[0], gd[1], bp["ro"])} if gd is not None else "tied: zero bending")
    else:
        rec["governing_fibre"] = "not applicable (an intensified bending measure, not a fibre stress)"
    rec["resultants_at_maximum"] = {"N_effective": V(Ne), "M_y": V(r["M_y"]), "M_z": V(r["M_z"])}
    if loc["kind"] == "interior":
        def slope(x):
            rr = line.arc_resultants_psi(k, x)
            return (obj_max_normal(rr, bp, pAi, m.R) if objective == "max_normal"
                    else obj_intensified(rr, bp, m.sif, m.R))[1]
        band = guaranteed_location_band(mx["evaluate"], slope, psi, m.phi, mx["K2"], mx["value"], z, arc=True)
        rec["location_band"] = {"psi_rad": V(band), "fraction": V(band / m.phi),
                                "meaning": "within the member domain [0, phi], a location within this distance of "
                                           "psi* stays within the 1e-9 value criterion; from the global lower-curvature "
                                           "bound f'' >= -K, using 99% of the tolerance and checking serialized "
                                           "centres, supporting-slope residuals and radius rounding"}
        rec["second_derivative_per_rad2"] = V(mx["second_derivative"])
    inside = mx["max_samples"] <= mx["value"] * (1 + D(10) ** -60) and mx["value"] <= mx["bracket_upper"]
    assert inside, ("analytic maximum outside the sampling bracket", line.id, m.id, objective)
    rec["dense_sampling_bracket"] = {"samples": mx["samples"], "h_rad": V(mx["h"]),
                                     "lipschitz_bound_per_rad": V(mx["lipschitz"]),
                                     "lower_max_of_samples": V(mx["max_samples"]),
                                     "upper_max_plus_L_h_over_2": V(mx["bracket_upper"]),
                                     "analytic_inside": inside}
    mx["cert"] = certify_maximum(mx["evaluate"], mx["grid"], mx["K2"], mx["value"], tol(mx["value"], z))
    rec["certification"] = cert_record(mx["cert"], "Pa/rad^2")
    names = [st for st, fr in STATIONS]
    sv = mx["station_values"]
    smax = max(sv)
    rec["station_only_maximum"] = {"value": V(smax), "station": names[sv.index(smax)],
                                   "shortfall_tolerances": sci((mx["value"] - smax) / tol(mx["value"], z))}
    rec["interior_peak_missed_by_stations"] = bool(mx["value"] - smax > tol(mx["value"], z))
    assert smax <= mx["value"] * (1 + D(10) ** -60)
    return rec, None


def emit_straight_max(line, k, m, mx, bp, pAi, z):
    a, b, c = mx["polys"]
    u = mx["u"]
    val = mx["value"]
    # polynomial forms reproduce the station resultants (statics consistency)
    for st, fr in STATIONS:
        r = line.resultants(k, fr)
        t = dq(fr)
        for poly, exact in ((a, (r["N"] + pAi) / bp["A"]), (b, r["M_y"] / bp["Z"]), (c, r["M_z"] / bp["Z"])):
            assert abs(pval(poly, t) - exact) <= K.EPS * 10 ** 6 * (1 + abs(exact)), "straight polynomial"
    Ne = pval(a, u) * bp["A"]
    My, Mz = pval(b, u) * bp["Z"], pval(c, u) * bp["Z"]
    if u == 0 or u == 1:
        loc = {"kind": "end_i" if u == 0 else "end_j", "fraction": V(u)}
    else:
        loc = {"kind": "interior", "fraction": V(u)}
    gd = fibre_of_max(Ne, My, Mz)
    rec = {"value": V(val), "method": "STRESS_REFERENCE section 6 (exact polynomial resultants, all real roots "
                                      "of H, zeros of a, common zeros of b and c, ends)",
           "location": loc,
           "governing_fibre": ({"surface": "outer", "sign": "sigma_plus" if Ne >= 0 else "sigma_minus",
                                **fibre_obj(gd[0], gd[1], bp["ro"])} if gd is not None else "tied: zero bending"),
           "resultants_at_maximum": {"N_effective": V(Ne), "M_y": V(My), "M_z": V(Mz)}}
    # dense sampling with a Lipschitz bound (|f'| <= L|w|/A + L F_max / Z per unit fraction)
    n = 4096

    def fs(t):
        return abs(pval(a, t)) + (pval(b, t) ** 2 + pval(c, t) ** 2).sqrt()
    b2 = b[2] if len(b) > 2 else D(0)
    c2 = c[2] if len(c) > 2 else D(0)
    K2 = 2 * (b2 * b2 + c2 * c2).sqrt()
    if loc["kind"] == "interior":
        # The pointwise second derivative remains diagnostic; the finite-distance
        # guarantee uses the global lower-curvature bound, including norm kinks.
        f1, f2 = straight_derivatives(a, b, c, u)
        require(f2 < 0 and abs(f1) <= abs(f2) * D(10) ** -50,
                "interior straight maximum is not a nondegenerate stationary maximum: %s %s" % (line.id, m.id))
        hh = D(10) ** -20
        fd = (fs(u + hh) - 2 * fs(u) + fs(u - hh)) / (hh * hh)
        mx["d2_central_difference_error"] = abs(fd - f2) / abs(f2)
        require(mx["d2_central_difference_error"] < D(10) ** -30, "straight second derivative")
        band = guaranteed_location_band(fs, lambda x: straight_derivatives(a, b, c, x)[0],
                                        u, D(1), K2, val, z)
        rec["location_band"] = {"fraction": V(band),
                                "meaning": "within the member domain [0, 1], a location within this distance of "
                                           "the fraction u* stays within the 1e-9 value criterion; from the global "
                                           "lower-curvature bound f'' >= -K, using 99% of the tolerance and checking "
                                           "serialized centres, supporting-slope residuals and radius rounding"}
        rec["second_derivative_per_fraction2"] = V(f2)
    grid = [(D(i) / n, fs(D(i) / n)) for i in range(n + 1)]
    smax = max(g[1] for g in grid)
    Fmax = mx["F0"] + mx["wn"] * mx["L"]
    lip = mx["L"] * mx["wn"] / bp["A"] + mx["L"] * Fmax / bp["Z"]
    upper = smax + lip / n / 2
    inside = smax <= val * (1 + D(10) ** -60) and val <= upper
    assert inside, ("straight maximum outside the sampling bracket", line.id, m.id)
    rec["dense_sampling_bracket"] = {"samples": n + 1, "h_fraction": V(D(1) / n),
                                     "lipschitz_bound_per_unit_fraction": V(lip),
                                     "lower_max_of_samples": V(smax), "upper_max_plus_L_h_over_2": V(upper),
                                     "analytic_inside": inside}
    mx["cert"] = certify_maximum(fs, grid, K2, val, tol(val, z))
    rec["certification"] = cert_record(mx["cert"], "Pa per unit fraction^2")
    names = [st for st, fr in STATIONS]
    svals = [abs(pval(a, dq(fr))) + (pval(b, dq(fr)) ** 2 + pval(c, dq(fr)) ** 2).sqrt() for st, fr in STATIONS]
    stmax = max(svals)
    rec["station_only_maximum"] = {"value": V(stmax), "station": names[svals.index(stmax)],
                                   "shortfall_tolerances": sci((val - stmax) / tol(val, z))}
    rec["interior_peak_missed_by_stations"] = bool(val - stmax > tol(val, z))
    assert stmax <= val * (1 + D(10) ** -60)
    return rec


def straight_derivatives(a, b, c, u):
    """Exact f'(u) and f''(u) in the fraction for f = |a| + hypot(b, c) on a straight,
    from its polynomial resultants (a linear, b and c quadratic). Valid where a(u) != 0
    and hypot(b, c) > 0, which holds at an interior maximum: f' jumps upward at a kink
    of |a| or of the hypot, so a kink is never a maximum (README item 4)."""
    def d(p):
        return pder(p) or [D(0)]
    a0, a1, a2 = pval(a, u), pval(d(a), u), pval(d(d(a)), u)
    b0, b1, b2 = pval(b, u), pval(d(b), u), pval(d(d(b)), u)
    c0, c1, c2 = pval(c, u), pval(d(c), u), pval(d(d(c)), u)
    h = (b0 * b0 + c0 * c0).sqrt()
    require(a0 != 0 and h > 0, "straight derivatives at a kink")
    s = sgn(a0)
    e = b0 * b1 + c0 * c1
    f1 = s * a1 + e / h
    f2 = s * a2 + (b1 * b1 + c1 * c1 + b0 * b2 + c0 * c2) / h - e * e / (h * h * h)
    return f1, f2


def summarize_discriminators(disc):
    groups = {}
    for mutant, path, exp, wrong, z in disc:
        t = tol(exp, z)
        sep = abs(wrong - exp) / t if t != 0 else D(0)
        groups.setdefault(mutant, []).append((sep, path, exp, wrong))
    out = {}
    for mutant in sorted(groups):
        rows = groups[mutant]
        moved = [r for r in rows if r[0] >= 1000]
        rows_sorted = sorted(moved, key=lambda r: (-r[0], r[1]))
        listed, seen = [], set()
        for sep, path, exp, wrong in rows_sorted:
            fam = "/".join(path.split("/")[:2])
            if fam in seen:
                continue
            seen.add(fam)
            listed.append({"row": path, "expected": V(exp), "wrong": V(wrong), "separation_tolerances": sci(sep)})
            if len(listed) >= 6:
                break
        out[mutant] = {"rows_evaluated": len(rows), "rows_moved_at_least_1e3_tolerances": len(moved),
                       "largest_separation_tolerances": sci(rows_sorted[0][0]) if moved else "0",
                       "smallest_listed_separation_tolerances": sci(listed[-1]["separation_tolerances"]) if listed else None,
                       "discriminable_in_this_case": bool(moved), "listed": listed}
    return out


# ---------------------------------------------------------------------------
# Cross-checks
# ---------------------------------------------------------------------------

class Fail(Exception):
    pass


def require(cond, msg):
    if not cond:
        raise Fail(msg)


def member_load_quadrature(m, w, O, rule):
    """Force and moment about O of a uniform member load, by Gauss-Legendre
    quadrature over the member (independent of the closed forms)."""
    if m.kind == "straight":
        def f(u):
            P = vadd(m.xi, vsc(u, vsub(m.xj, m.xi)))
            Fd = vsc(m.length, w)
            return Fd + vcross(vsub(P, O), Fd)
        v = gl_integrate(f, D(0), D(1), rule)
    else:
        def f(psi):
            sn, cs = sincos(psi)
            P = m.point_cs(cs, sn)
            Fd = vsc(m.R, w)
            return Fd + vcross(vsub(P, O), Fd)
        v = gl_integrate(f, D(0), m.phi, rule)
    return v[:3], v[3:]


def check_statics(line, rule):
    O = (D("0.123"), D("-4.56"), D("7.89"))
    m0, fwd0 = line.order[0]
    r = line.resultants(0, Fraction(0) if fwd0 else Fraction(1))
    sgn0 = -1 if fwd0 else 1
    RA_F, RA_M = vsc(sgn0, r["_F"]), vsc(sgn0, r["_M"])
    Ft, Mt = ZV, ZV
    scale_F, scale_M = D(0), D(0)
    for n, f in line.nf.items():
        Ft = vadd(Ft, f)
        Mt = vadd(Mt, vcross(vsub(line.pos[n], O), f))
        scale_F += vnorm(f)
    for n, mo in line.nm.items():
        Mt = vadd(Mt, mo)
        scale_M += vnorm(mo)
    for m in line.members:
        w = line.ml.get(m.id)
        if w is not None:
            Fl, Ml = member_load_quadrature(m, w, O, rule)
            Ft, Mt = vadd(Ft, Fl), vadd(Mt, Ml)
            scale_F += vnorm(w) * m.length
    scale_M += scale_F * (vnorm(vsub(line.pos[line.anchor], O)) + sum(m.length for m in line.members))
    resF = vnorm(vadd(RA_F, Ft))
    resM = vnorm(vadd(vadd(vcross(vsub(line.pos[line.anchor], O), RA_F), RA_M), Mt))
    out = {"force_residual_relative": sci(resF / scale_F), "moment_residual_relative": sci(resM / scale_M)}
    require(resF / scale_F < D(10) ** -55 and resM / scale_M < D(10) ** -55, "global equilibrium " + line.id)
    # junction continuity (global, then the shared tangent)
    worst_g, worst_t, worst_nt = D(0), D(0), D(0)
    for k in range(len(line.order) - 1):
        (m1, f1), (m2, f2) = line.order[k], line.order[k + 1]
        n = line.node_seq[k + 1]
        r1 = line.resultants(k, Fraction(1) if f1 else Fraction(0))
        r2 = line.resultants(k + 1, Fraction(0) if f2 else Fraction(1))
        s1, s2 = (1 if f1 else -1), (1 if f2 else -1)
        dF = vsub(vsc(s1, r1["_F"]), vsc(s2, r2["_F"]))
        dM = vsub(vsc(s1, r1["_M"]), vsc(s2, r2["_M"]))
        dF = vsub(dF, line.nf.get(n, ZV))
        dM = vsub(dM, line.nm.get(n, ZV))
        worst_g = max(worst_g, (vnorm(dF) + vnorm(dM)) / (scale_F + scale_M))
        t1 = vsc(s1, r1["_frame"][0])
        t2 = vsc(s2, r2["_frame"][0])
        worst_t = max(worst_t, vnorm(vsub(t1, t2)))
        if n not in line.nf and n not in line.nm:
            worst_nt = max(worst_nt, (abs(r1["N"] - r2["N"]) + abs(r1["T"] - r2["T"])) / (scale_F + scale_M))
    out["junction_global_residual_relative"] = sci(worst_g)
    out["junction_tangent_mismatch"] = sci(worst_t)
    out["junction_N_T_mismatch_relative"] = sci(worst_nt)
    out["junction_note"] = ("global continuity is exact; the shared tangent (and so N and T across the node) holds to "
                            "the binary64 rounding of the authored coordinates (exact for dyadic inputs)")
    require(worst_g < D(10) ** -55 and worst_t < D(10) ** -12 and worst_nt < D(10) ** -12, "junctions " + line.id)
    # wall statics: pressure on the wall beyond each cut gives exactly p*Ai along x and no moment
    if line.pressure is not None:
        worst = D(0)
        last, lf = line.order[-1]
        t_tip = vsc(1 if lf else -1, last.frame(Fraction(1) if lf else Fraction(0))[0])
        for k, (m, fwd) in enumerate(line.order):
            pAi = line.pAi(m.id)
            for st, fr in STATIONS:
                r = line.resultants(k, fr)
                P = r["_P"]
                cap = vsc(pAi, t_tip)
                Fp, Mp = cap, vcross(vsub(line.pos[line.tip], P), cap)
                segs = []
                if m.kind == "arc":
                    psi = dq(fr) * m.phi
                    segs.append((m, psi, m.phi) if fwd else (m, D(0), psi))
                for m2, f2 in line.order[k + 1:]:
                    if m2.kind == "arc":
                        segs.append((m2, D(0), m2.phi))
                for mm, a, b in segs:
                    if b == a:
                        continue

                    def f(psi, mm=mm):
                        sn, cs = sincos(psi)
                        q_ = vsc(-pAi, mm.ey_at(cs, sn))           # -p Ai e_y per unit psi
                        return q_ + vcross(vsub(mm.point_cs(cs, sn), P), q_)
                    v = gl_integrate(f, a, b, rule)
                    Fp, Mp = vadd(Fp, v[:3]), vadd(Mp, v[3:])
                s = 1 if fwd else -1
                Fp, Mp = vsc(s, Fp), vsc(s, Mp)
                ex = r["_frame"][0]
                worst = max(worst, (vnorm(vsub(Fp, vsc(pAi, ex))) + vnorm(Mp) / (1 + m.length)) / abs(pAi))
        out["wall_pressure_statics_residual_relative"] = sci(worst)
        out["wall_pressure_statics_meaning"] = ("on every section, the bore pressure on the wall beyond the cut "
                                                "(arc line loads -p Ai e_y/R by quadrature, plus the free-tip cap) "
                                                "equals p Ai along the section's x with zero moment: N_w = S + p Ai "
                                                "and pressure adds no shear or bending")
        out["wall_pressure_statics_note"] = ("exact (below 1e-50) for exactly tangent junctions; on the moved copy "
                                             "the binary64-rounded coordinates leave kinks of about 1e-16 rad, "
                                             "whose pressure resultant p Ai (t_in - t_out) is of that relative size")
        require(worst < D(10) ** -12, "wall pressure statics " + line.id)
    return out


def check_objectivity(an_a, an_b):
    """Frame-local values of a case and its rigidly moved copy."""
    worst = D(0)
    for m, fwd in an_a.line.order:
        zs = an_a.zs[m.id]
        for st, fr in STATIONS:
            ra, rb = an_a.res[m.id][st], an_b.res[m.id][st]
            for kk in RESULTANT_KEYS:
                fam = "moment" if kk in ("T", "M_y", "M_z") else "force"
                worst = max(worst, abs(ra[kk] - rb[kk]) / tol(ra[kk], zs[fam]))
            for basis in an_a.line.bases:
                qa, qb = an_a.q[(m.id, st, basis)], an_b.q[(m.id, st, basis)]
                for kq in qa:
                    z = zs[basis]["normal_stress"] if kq[0] in NORMAL_Q else zs[basis]["shear_stress"]
                    worst = max(worst, abs(qa[kq] - qb[kq]) / tol(qa[kq], z))
        for basis in an_a.line.bases:
            va, vb = an_a.mmax[(m.id, basis)]["value"], an_b.mmax[(m.id, basis)]["value"]
            worst = max(worst, abs(va - vb) / tol(va, an_a.zs[m.id][basis]["normal_stress"]))
    require(worst < D("1e-3"), "objectivity")
    return worst


# SV field qualification ------------------------------------------------------

def sv_qualify(ro, ri, nu, rule, n_r=24, n_b=360):
    I = K.PI * (ro ** 4 - ri ** 4) / 4
    A = K.PI * (ro * ro - ri * ri)
    out = {"ro": V(ro), "ri": V(ri), "nu": V(nu)}
    # 1. polar rewrite equals the refutation's Cartesian field
    worst = D(0)
    for (Vy, Vz) in ((D(1), D(0)), (D(0), D(1)), (D("0.6"), D("-0.8"))):
        for rr in (ri, (ri + ro) / 2, ro):
            for j in range(16):
                sb, cb = sincos(2 * K.PI * j / 16 + D("0.1"))
                a1 = sv_polar(Vy, Vz, rr, cb, sb, ro, ri, nu, I)
                a2 = sv_cartesian(Vy, Vz, rr * cb, rr * sb, ro, ri, nu, I)
                worst = max(worst, abs(a1[0] - a2[0]) + abs(a1[1] - a2[1]))
    out["polar_equals_refutation_field_max_abs_per_unit_V_times_A"] = sci(worst * A)
    require(worst * A < D(10) ** -70, "polar field")
    # 2. resultants, torque and energy by quadrature (trapezoid in beta, GL in log r)
    nb = 64
    angles = [sincos(2 * K.PI * j / nb) for j in range(nb)]
    lo, hi = ri.ln(), ro.ln()

    def ring(x, Vy, Vz):
        rr = x.exp()
        sxy = sxz = tq = en = D(0)
        for sb, cb in angles:
            txy, txz = sv_cartesian(Vy, Vz, rr * cb, rr * sb, ro, ri, nu, I)
            sxy += txy
            sxz += txz
            tq += rr * cb * txz - rr * sb * txy
            en += txy * txy + txz * txz
        wgt = 2 * K.PI / nb * rr * rr        # dA = r dr dbeta, dr = r dx
        return (sxy * wgt, sxz * wgt, tq * wgt, en * wgt)

    res = {}
    for lab, Vy, Vz in (("V_y", D(1), D(0)), ("V_z", D(0), D(1))):
        v = gl_integrate(lambda x: ring(x, Vy, Vz), lo, hi, rule)
        res[lab] = v
    kap = kappa_energy(ro, ri, nu)
    errs = [abs(res["V_y"][0] - 1), abs(res["V_y"][1]), abs(res["V_y"][2] / ro),
            abs(res["V_z"][0]), abs(res["V_z"][1] - 1), abs(res["V_z"][2] / ro)]
    out["resultant_and_torque_max_abs_error_per_unit_V"] = sci(max(errs))
    eE = abs(res["V_y"][3] * kap * A - 1)
    out["energy_identity_relative_error"] = sci(eE)
    out["energy_identity"] = "integral (tau_xy^2 + tau_xz^2) dA / V^2 = 1 / (kappa_E A), SHEAR_REFERENCE's kappa_E"
    require(max(errs) < D(10) ** -45 and eE < D(10) ** -45, "sv quadrature")
    # 3. traction-free surfaces (Cartesian refutation form), circulation
    worst = D(0)
    for rr in (ri, ro):
        for j in range(24):
            sb, cb = sincos(2 * K.PI * j / 24 + D("0.05"))
            txy, txz = sv_cartesian(D("0.6"), D("-0.8"), rr * cb, rr * sb, ro, ri, nu, I)
            worst = max(worst, abs(txy * cb + txz * sb) * A)
    out["radial_shear_at_both_surfaces_max_per_unit_V_times_A"] = sci(worst)
    require(worst < D(10) ** -70, "traction-free surfaces")
    # 4. equilibrium and compatibility (central differences)
    h = D(10) ** -25
    worst_eq = worst_cp = D(0)
    for (y0, z0) in (((ri + ro) / 2, D(0)), ((ri + ro) / 3, (ri + ro) / 4), (-(2 * ri + ro) / 3 * D("0.6"), (2 * ri + ro) / 3 * D("0.8"))):
        for lab, Vy, Vz in (("V_y", D(1), D(0)), ("V_z", D(0), D(1))):
            f = lambda y, z: sv_cartesian(Vy, Vz, y, z, ro, ri, nu, I)
            div = (f(y0 + h, z0)[0] - f(y0 - h, z0)[0] + f(y0, z0 + h)[1] - f(y0, z0 - h)[1]) / (2 * h)
            worst_eq = max(worst_eq, abs(div + (Vy * y0 + Vz * z0) / I) * I / ro)
            lap = [(f(y0 + h, z0)[c] + f(y0 - h, z0)[c] + f(y0, z0 + h)[c] + f(y0, z0 - h)[c] - 4 * f(y0, z0)[c])
                   / (h * h) for c in (0, 1)]
            want = [-Vy / ((1 + nu) * I), -Vz / ((1 + nu) * I)]
            worst_cp = max(worst_cp, (abs(lap[0] - want[0]) + abs(lap[1] - want[1])) * I)
    out["equilibrium_divergence_error"] = sci(worst_eq)
    out["compatibility_laplacian_error"] = sci(worst_cp)
    out["compatibility_meaning"] = ("Beltrami-Michell for Saint-Venant flexure: the Laplacian of the shear component "
                                    "along V is -V/((1+nu) I) and of the other component 0; this fixes the "
                                    "nu-dependence that equilibrium and traction-free surfaces leave open")
    require(worst_eq < D(10) ** -30 and worst_cp < D(10) ** -20, "sv equilibrium/compatibility")
    # 5. dense sampling of |tau| confirms the maximum's value and location
    Vy, Vz = D("0.6"), D("-0.8")
    bv = atan2(Vz, Vy)
    best, arg = D(-1), None
    angs = []
    for j in range(n_b):
        b_ = bv + 2 * K.PI * j / n_b
        angs.append((j, sincos(b_)))
    for i in range(n_r + 1):
        rr = ri + (ro - ri) * i / n_r
        for j, (sb, cb) in angs:
            txy, txz = sv_polar(Vy, Vz, rr, cb, sb, ro, ri, nu, I)
            mag = (txy * txy + txz * txz).sqrt()
            if mag > best * (1 + D(10) ** -60):
                best, arg = mag, (i, j)
    tmax = sv_tau_max_coefficient(ro, ri, nu, I)
    out["dense_sampling"] = {"grid": "%d radii x %d angles (angles measured from the direction of V)" % (n_r + 1, n_b),
                             "max_sampled": V(best), "closed_form": V(tmax),
                             "argmax_radius_index": arg[0], "argmax_angle_from_V_deg": V(D(360) * arg[1] / n_b)}
    require(abs(best - tmax) <= tmax * D(10) ** -60 and arg[0] == 0 and arg[1] in (n_b // 4, 3 * n_b // 4),
            "sv maximum location")
    return out


def sv_limits():
    out = {}
    nus = [D(0), D("0.27"), D("0.3"), D("0.45")]
    rows = []
    for nu in nus:
        ro = D(1)
        # thin wall
        ri = ro * (1 - D(10) ** -12)
        A = K.PI * (ro * ro - ri * ri)
        I = K.PI * (ro ** 4 - ri ** 4) / 4
        thin = sv_tau_max_coefficient(ro, ri, nu, I) * A
        # small hole
        ri2 = D(10) ** -15
        A2 = K.PI * (ro * ro - ri2 * ri2)
        I2 = K.PI * (ro ** 4 - ri2 ** 4) / 4
        small = sv_tau_max_coefficient(ro, ri2, nu, I2) * A2
        solid = (3 + 2 * nu) / (2 * (1 + nu))
        # solid field at the centre (p = 0): C c ro^2 |V|, times A
        Is = K.PI / 4
        centre = (3 + 2 * nu) / 4 / (2 * (1 + nu) * Is) * K.PI
        rows.append({"nu": V(nu), "thin_wall_ratio_to_V_over_A": V(thin), "thin_wall_limit": "2",
                     "small_hole_ratio_to_V_over_A": V(small), "solid_centre_ratio_to_V_over_A": V(centre),
                     "solid_closed_form": V(solid), "small_hole_factor": V(small / solid)})
        require(abs(thin - 2) < D(10) ** -10, "thin-wall limit")
        require(abs(small / solid - 2) < D(10) ** -25, "small-hole factor")
        require(abs(centre - solid) < D(10) ** -70, "solid centre")
        if nu == 0:
            m = D("0.9")
            I3 = K.PI * (1 - m ** 4) / 4
            out["nu_zero"] = {"closed_form_coefficient_times_I": V(sv_tau_max_coefficient(D(1), m, nu, I3) * I3),
                              "expected_(3ro^2+ri^2)/4": V((3 + m * m) / 4)}
            require(abs(sv_tau_max_coefficient(D(1), m, nu, I3) * I3 - (3 + m * m) / 4) < D(10) ** -80, "nu=0")
            kc = 6 * (1 + nu) * (1 + m * m) ** 2 / ((7 + 6 * nu) * (1 + m * m) ** 2 + (20 + 12 * nu) * m * m)
            out["nu_zero"]["kappa_E_equals_kappa_C"] = sci(abs(kappa_energy(D(1), m, nu) - kc))
    out["limits"] = rows
    # ratios to the wrong shortcuts at hollow ratios
    tab = []
    for m in (D("0.5"), D("0.8"), D("0.9"), D("0.95")):
        for nu in (D(0), D("0.27"), D("0.3")):
            ksv = ((3 + 2 * nu) + (1 + 2 * nu) * m * m) / ((1 + nu) * (1 + m * m))
            tab.append({"ri_over_ro": V(m), "nu": V(nu), "tau_max_over_V_over_A": V(ksv),
                        "one_over_kappa_E": V(1 / kappa_energy(D(1), m, nu)),
                        "solid_centre_value": V((3 + 2 * nu) / (2 * (1 + nu))), "two": "2", "one": "1",
                        "four_thirds": V(D(4) / 3)})
            for wrong in (D(1), 1 / kappa_energy(D(1), m, nu), D(2), (3 + 2 * nu) / (2 * (1 + nu)), D(4) / 3):
                require(abs(ksv - wrong) > D("1e-3"), "SV shortcut coincides")
    out["ratio_table"] = tab
    out["ratio_formula"] = "tau_max / (V/A) = ((3+2nu) + (1+2nu) m^2) / ((1+nu)(1+m^2)), m = ri/ro"
    # domain scan for nu < -1/2 (outside the proof; informational)
    scan = []
    for nu in (D("-0.95"), D("-0.8"), D("-0.6")):
        for m in (D("0.1"), D("0.5"), D("0.9")):
            ro, ri = D(1), m
            I = K.PI * (1 - m ** 4) / 4
            tmax = sv_tau_max_coefficient(ro, ri, nu, I)
            best = D(0)
            for i in range(41):
                rr = ri + (ro - ri) * i / 40
                for j in range(73):
                    sb, cb = sincos(2 * K.PI * j / 72)
                    txy, txz = sv_polar(D(1), D(0), rr, cb, sb, ro, ri, nu, I)
                    best = max(best, (txy * txy + txz * txz).sqrt())
            scan.append({"nu": V(nu), "ri_over_ro": V(m), "sampled_max_over_closed_form": V(best / tmax)})
            require(best <= tmax * (1 + D(10) ** -60), "auxetic scan exceeds closed form")
    for row in scan:
        require(abs(D(row["sampled_max_over_closed_form"][0]) - 1) < D(10) ** -60, "auxetic scan")
    out["auxetic_scan_confirms_closed_form"] = scan
    return out


# hoop qualification -------------------------------------------------------------

def hoop_qualify(p, ri, ro, R, rule_b, rule_t, rule_r):
    t = ro - ri
    Theta = D("0.3")
    out = {"p": V(p), "ri": V(ri), "ro": V(ro), "R": V(R)}

    def bore_force_Z(b1, b2):
        # pressure on the bore surface patch beta' in [b1, b2], theta in [0, Theta]:
        # dF = p (dP/dbeta' x dP/dtheta) dbeta' dtheta, torus axis = global Z
        def fb(bp):
            sb, cb = sincos(bp)
            w = R - ri * cb

            def ft(th):
                st_, ct = sincos(th)
                dPb = (ri * sb * ct, ri * sb * st_, ri * cb)
                dPt = (-w * st_, w * ct, D(0))
                n = vcross(dPb, dPt)
                return n[2]
            return gl_integrate(ft, D(0), Theta, rule_t)
        return p * gl_integrate(fb, b1, b2, rule_b)

    def cut_area_Z(beta, sense):
        sb, cb = sincos(beta)

        def fr(rho):
            w = R - rho * cb

            def ft(th):
                st_, ct = sincos(th)
                dPr = (-cb * ct, -cb * st_, sb)
                dPt = (-w * st_, w * ct, D(0))
                n = vcross(dPr, dPt)          # = varpi * (-e_beta)
                return sense * n[2]
            return gl_integrate(ft, D(0), Theta, rule_t)
        return gl_integrate(fr, ri, ro, rule_r)

    worst = D(0)
    vals = []
    for bdeg in (0, 30, 60, 80, 100, 135, 180):
        beta = K.PI * bdeg / 180
        cb = sincos(beta)[1]
        closed = toroidal_hoop(p, ri, ro, R, cb)
        # free body A: wall between beta and the +z crown; outward cut normal -e_beta
        sA = -bore_force_Z(beta, K.PI / 2) / cut_area_Z(beta, 1)
        # free body B: wall between the -z crown and beta; outward cut normal +e_beta
        sB = -bore_force_Z(-K.PI / 2, beta) / cut_area_Z(beta, -1)
        e = max(abs(sA - closed), abs(sB - closed)) / abs(closed)
        worst = max(worst, e)
        vals.append({"beta_deg": bdeg, "closed_form": V(closed), "free_body_to_+z_crown": V(sA),
                     "free_body_from_-z_crown": V(sB)})
    out["quadrature"] = vals
    out["quadrature_max_relative_difference"] = sci(worst)
    require(worst < D(10) ** -40, "hoop quadrature")
    crown = toroidal_hoop(p, ri, ro, R, D(0))
    lame_mean = gl_integrate(lambda r: lame_hoop(p, ri, ro, r), ri, ro, rule_r) / t
    out["crown_closed_form"] = V(crown)
    out["crown_equals_p_ri_over_t"] = sci(abs(crown - p * ri / t) / abs(crown))
    out["crown_equals_lame_wall_mean_by_quadrature"] = sci(abs(crown - lame_mean) / abs(crown))
    require(abs(crown - p * ri / t) <= abs(crown) * D(10) ** -80, "crown")
    require(abs(crown - lame_mean) <= abs(crown) * D(10) ** -40, "crown vs Lame mean")
    big = D(10) ** 40
    lim = max(abs(toroidal_hoop(p, ri, ro, big, D(c)) - p * ri / t) for c in (-1, 0, 1)) / abs(p * ri / t)
    out["straight_limit_R_1e40_relative"] = sci(lim)
    require(lim < D(10) ** -38, "straight limit")
    # thin-wall limit: ri = ro (1 - eps)
    eps = D(10) ** -30
    rit = ro * (1 - eps)
    tt = ro - rit
    worst_t = D(0)
    for c in (D(1), D("0.5"), D(0), D(-1)):
        cl = p * ro / (2 * tt) * (2 * R - ro * c) / (R - ro * c)
        worst_t = max(worst_t, abs(toroidal_hoop(p, rit, ro, R, c) - cl) / abs(cl))
    out["thin_wall_limit_relative_eps_1e-30"] = sci(worst_t)
    require(worst_t < D(10) ** -25, "thin-wall limit")
    hi_, cr_, lo_ = (toroidal_hoop(p, ri, ro, R, D(c)) for c in (1, 0, -1))
    order = (hi_ > cr_ > lo_) if p > 0 else (hi_ < cr_ < lo_)
    out["ordering"] = "intrados > crown > extrados" if p > 0 else "intrados < crown < extrados (p < 0)"
    require(order, "hoop ordering")
    return out


# STRESS_REFERENCE reproduction ---------------------------------------------------

def stress_reference_reproduction():
    out = {}
    ri, ro = D("0.04"), D("0.05")
    A = K.PI * (ro * ro - ri * ri)
    I = K.PI * (ro ** 4 - ri ** 4) / 4
    Z, J = I / ro, 2 * I
    N, My, Mz, T = 1800 * K.PI, D("55.35") * K.PI, D("73.8") * K.PI, D("221.4") * K.PI
    ext = normal_extremes(N, My, Mz, A, Z, ro)
    tors = T * ro / J
    yz = ext["plus_yz"]
    s1 = {"sigma_plus": ext["sigma_plus"], "sigma_minus": ext["sigma_minus"],
          "max_abs_normal": abs(N) / A + ext["B"] / Z, "torsional_shear": tors,
          "fibre_y": yz[0], "fibre_z": yz[1], "abs_sum_wrong": abs(N) / A + abs(My) / Z + abs(Mz) / Z}
    want = {"sigma_plus": D(7000000), "sigma_minus": D(-3000000), "max_abs_normal": D(7000000),
            "torsional_shear": D(6000000), "fibre_y": D("-0.04"), "fibre_z": D("0.03"), "abs_sum_wrong": D(9000000)}
    err = max(abs(s1[k] - want[k]) / max(abs(want[k]), 1) for k in want)
    # fibre stress at the S1 tensile fibre from the general fibre formula
    fpl = fibre_normal(N, My, Mz, A, I, ro, D("-0.8"), D("0.6"))
    err = max(err, abs(fpl - 7000000) / 7000000)
    out["S1"] = {"values": {k: V(v) for k, v in s1.items()}, "max_relative_error": sci(err)}
    require(err < D(10) ** -80, "S1")
    ext2 = normal_extremes(D(0), D(0), D(0), A, Z, ro)
    out["S2"] = {"normal_extremes": [V(ext2["sigma_plus"]), V(ext2["sigma_minus"])], "tied": ext2["tied"],
                 "torsional_shear": V(tors), "von_mises_S1_outer_named_not_default": V((D(157)).sqrt() * 10 ** 6)}
    require(ext2["tied"] and abs(tors - 6 * 10 ** 6) < D(10) ** -70, "S2")
    # I1 (component frame: first axis M_y amplitude 3 MPa with factor 2; second M_z 4 MPa with factor 3)
    i_op, i_ip = D(2), D(3)
    inten = ((i_ip * Mz) ** 2 + (i_op * My) ** 2).sqrt() / Z
    api = abs(N) / A + inten
    e1 = abs(inten - 6 * D(5).sqrt() * 10 ** 6) / inten
    e2 = abs(api - (2 + 6 * D(5).sqrt()) * 10 ** 6) / api
    pure_first = ((i_ip * 0) ** 2 + (i_op * My) ** 2).sqrt() / Z
    pure_second = ((i_ip * Mz) ** 2 + (i_op * 0) ** 2).sqrt() / Z
    swapped_axes = ((i_op * My) ** 2 + (i_ip * Mz) ** 2).sqrt() / Z  # axes and factors swapped together
    out["I1"] = {"intensified_bending": V(inten), "expected": "6*sqrt(5) MPa",
                 "axial_plus_intensified": V(api), "expected_axial_plus": "2 + 6*sqrt(5) MPa",
                 "axial_unchanged": V(N / A), "torsion_unchanged": V(tors),
                 "pure_first_axis_only_its_factor": V(pure_first), "pure_second_axis_only_its_factor": V(pure_second),
                 "axes_and_factors_swapped_together": V(swapped_axes),
                 "max_relative_error": sci(max(e1, e2))}
    require(max(e1, e2) < D(10) ** -80 and abs(pure_first - 6 * 10 ** 6) < 1 and abs(pure_second - 12 * 10 ** 6) < 1,
            "I1")
    # C1 and C2
    def ext_of(n, my, mz):
        e = normal_extremes(n, my, mz, A, Z, ro)
        return abs(n) / A + e["B"] / Z
    sumAB = ext_of(N - N, My - My, Mz - Mz)
    diff = ext_of(2 * N, 2 * My, 2 * Mz)
    out["C1"] = {"A_max_abs": V(ext_of(N, My, Mz)), "B_max_abs": V(ext_of(-N, -My, -Mz)),
                 "signed_sum": V(sumAB), "signed_difference_max_abs": V(diff),
                 "torsion_range": V(2 * tors), "wrong_magnitude_sum": V(2 * ext_of(N, My, Mz)),
                 "wrong_scalar_difference": "0",
                 "fixed_fibre_A": V(fibre_normal(N, My, Mz, A, I, ro, D("-0.8"), D("0.6"))),
                 "fixed_fibre_B": V(fibre_normal(-N, -My, -Mz, A, I, ro, D("-0.8"), D("0.6")))}
    require(sumAB == 0 and abs(diff - 14 * 10 ** 6) < D(10) ** -70 and abs(2 * tors - 12 * 10 ** 6) < D(10) ** -70, "C1")
    d2 = ((D(3) - 3) ** 2 + (D(4) + 4) ** 2).sqrt()
    out["C2"] = {"bending_magnitudes": ["5 MPa", "5 MPa"], "difference_field_bending_magnitude_MPa": V(d2)}
    require(d2 == 8, "C2")
    # X1 through the section-6 implementation
    a, b, c = [D(0)], [D(0), D(4), D(-4)], [D(0), D(1)]
    val, t = section6_maximum(a, b, c)
    tx = (6 - D(2).sqrt()) / 8
    fx = (71 + 8 * D(2).sqrt()).sqrt() / 8
    old = (D(325)).sqrt() / 16
    out["X1"] = {"t_star": V(t), "expected_t_star": "(6 - sqrt(2))/8", "f_star_MPa": V(val),
                 "expected_f_star": "sqrt(71 + 8 sqrt(2))/8", "old_eight_sign_candidates_value": V(old),
                 "errors": [sci(abs(t - tx)), sci(abs(val - fx))]}
    require(abs(t - tx) < D(10) ** -40 and abs(val - fx) < D(10) ** -70, "X1")
    return out


def marker_control():
    """Markers: no vetted basis says which axis is in-plane at a geometry-only
    marker; only the component-frame I1-level control is frozen."""
    return {"basis": "STRESS_REFERENCE section 4, I1, in a named component frame",
            "component_frame_amplitudes_MPa": {"first_axis": 3, "second_axis": 4},
            "factors": {"first_axis": 2, "second_axis": 3},
            "intensified_bending_MPa": "6*sqrt(5) = " + fmt(6 * D(5).sqrt()),
            "axial_plus_intensified_MPa": "2 + 6*sqrt(5) = " + fmt(2 + 6 * D(5).sqrt()),
            "swap_rule": "swapping the axes requires swapping the factors; pure first-axis bending uses only the "
                         "first factor (6 MPa), pure second-axis only the second (12 MPa)",
            "gap": "no vetted basis fixes which local moment axis is in-plane at a geometry-only bend or branch "
                   "marker at a straight end; no marker row is frozen beyond this control"}


# U4a cross-check -------------------------------------------------------------------

def u4a_cross_check():
    rq_path = os.path.join(U4A_DIR, "requests.json")
    or_path = os.path.join(U4A_DIR, "oracle.json")
    rq_bytes = open(rq_path, "rb").read()
    or_bytes = open(or_path, "rb").read()
    require(hashlib.sha256(rq_bytes).hexdigest() == U4A_REQUESTS_SHA256, "U4a requests.json changed")
    require(hashlib.sha256(or_bytes).hexdigest() == U4A_ORACLE_SHA256, "U4a oracle.json changed")
    req = json.loads(rq_bytes)
    orc = json.loads(or_bytes)
    require(orc["requests_sha256"] == hashlib.sha256(rq_bytes).hexdigest(), "U4a binding")
    cases = []
    worst_cut = worst_cut_tol = worst_str = D(0)
    count = 0
    smap = {"sigma_axial": "N", "sigma_m_wall_membrane": "N_w", "sigma_b_y": "M_y", "sigma_b_z": "M_z",
            "tau_t": "T"}
    for cid, c in req["cases"].items():
        if not u4a_free_tip(c):
            continue
        line = line_from_u4a(cid, c)
        oc = orc["cases"][cid]
        wc = ws = D(0)
        for k, (m, fwd) in enumerate(line.order):
            om = oc["members"][m.id]
            zsc = om["zero_scale"]
            pAi = line.pAi(m.id)
            for st, fr in STATIONS:
                r = line.resultants(k, fr)
                mine = {"V_y": r["V_y"], "V_z": r["V_z"], "T": r["T"], "M_y": r["M_y"], "M_z": r["M_z"],
                        "N": r["N"], "S": r["N"], "N_w": r["N"] + pAi}
                for key, e in om["cuts_tangent_frame_j_side"][st].items():
                    e = D(e)
                    fam = "moment" if key in ("T", "M_y", "M_z") else "force"
                    d = abs(mine[key] - e) / max(abs(e), D(zsc[fam]))
                    wc = max(wc, d)
                    count += 1
                if "stresses_SI_by_section_basis" in om:
                    blocks = om["stresses_SI_by_section_basis"]
                else:
                    blocks = {"preview_single_section": om["nominal_stresses_SI"]}
                for basis, blk in blocks.items():
                    bp = line.basis_props[m.id][basis]
                    for key, e in blk[st].items():
                        e = D(e)
                        src = smap[key]
                        val = mine[src]
                        if key in ("sigma_axial", "sigma_m_wall_membrane"):
                            mv, floor = val / bp["A"], D(zsc["force"]) / bp["A"]
                        elif key == "tau_t":
                            mv, floor = val * bp["ro"] / bp["J"], D(zsc["moment"]) * bp["ro"] / bp["J"]
                        else:
                            mv, floor = val / bp["Z"], D(zsc["moment"]) / bp["Z"]
                        ws = max(ws, abs(mv - e) / max(abs(e), floor))
                        count += 1
        worst_cut, worst_str = max(worst_cut, wc), max(worst_str, ws)
        cases.append({"case": cid, "route": line.route, "max_relative_difference_cuts": sci(wc),
                      "max_relative_difference_stresses": sci(ws)})
    require(worst_cut < D(10) ** -20 and worst_str < D(10) ** -20, "U4a cross-check")
    return {"requests_file": "../t4_u4a/requests.json",
            "requests_sha256": hashlib.sha256(rq_bytes).hexdigest(),
            "oracle_file": "../t4_u4a/oracle.json",
            "oracle_sha256": hashlib.sha256(or_bytes).hexdigest(),
            "selection": "every case whose only support is one full anchor (statically determinate free tip)",
            "compared": "the five j-side tangent-frame cuts (N or N_w and S, V_y, V_z, T, M_y, M_z) and the "
                        "per-basis stress components (axial or wall membrane, bending y and z, torsion) at every "
                        "station of every member, recomputed with this generator's statics and D-5 sections",
            "values_compared": count, "cases": cases,
            "worst_relative_difference_cuts": sci(worst_cut),
            "worst_relative_difference_stresses": sci(worst_str),
            "criterion": "|mine - U4a| / max(|U4a|, U4a's member zero_scale (stresses: that scale over A, Z or "
                         "J/ro)) < 1e-20; U4a prints 22 significant digits",
            "note": "U4a's arc loads are per unit arc length; this generator reproduces U4a only under that reading"}


# ---------------------------------------------------------------------------
# Conventions, definitions, criterion (frozen text)
# ---------------------------------------------------------------------------

CONVENTIONS = {
    "tangent_frame": "at arc angle psi from authored node i: x = arc tangent toward authored j; z = bend-plane "
                     "normal (x_i - c) x (x_j - c), normalised; y = z x x, toward the arc centre (DEL-04-07 "
                     "arc-frames-and-signs.md section 2). On a straight: x = chord, y = y_reference projected, "
                     "z = x x y",
    "cuts": "j-side section action (the authored-j side acting on the authored-i side), N positive in tension. On "
            "pressurized members of the exact route S is the effective (elastic) axial force and N_w = S + p Ai "
            "the wall force; elsewhere N",
    "stations": "end_i, quarter_1, midspan, quarter_3, end_j at authored fractions 0, 1/4, 1/2, 3/4, 1 (psi = "
                "fraction * phi on an arc)",
    "fibres": "angle beta on the section from +y toward +z: y_pos 0 deg (intrados on an arc), diag_45, z_pos 90 "
              "deg (crown), diag_135, y_neg 180 deg (extrados), z_neg 270 deg (crown); surfaces outer (r_o) and "
              "inner (that basis's r_i); fibre coordinates (y, z) = r (cos beta, sin beta)",
    "normal_stress": "sigma_x(y, z) = N_eff/A_b + M_y z/I_b - M_z y/I_b (STRESS_REFERENCE section 1), N_eff = N_w "
                     "on pressurized members of the exact route, otherwise N",
    "normal_extremes": "sigma_plus/minus = N_eff/A_b +/- hypot(M_y, M_z)/Z_b at the outer fibres (y, z) = +/- r_o "
                       "(-M_z, M_y)/hypot; tied when the bending is zero; max_abs_normal = |N_eff|/A_b + "
                       "hypot(M_y, M_z)/Z_b",
    "shear_components": "tau_xy, tau_xz are the traction components on the +x (j-side) face; Saint-Venant flexure "
                        "(sv_transverse_shear) and torsion (torsional_shear = T r_o/J_b, signed with T) are "
                        "separate quantities; flexure_plus_torsion_check adds the torsion field (-T z/J_b, "
                        "T y/J_b) at the outer named fibres",
    "section_bases": "exact route (D-5): nominal_section (wall t) and reduced_section (t_r = (t - c_m) - c_a), each "
                     "its own A, I, J = 2I, Z = I/r_o with r_o = D/2; the pressure bore r_i,r = D/2 - t_r with "
                     "Ai = pi r_i,r^2 governs p Ai; preview route: preview_single_section (wall t - c_m)",
    "directional_sif": "on an arc M_z is in-plane and M_y out-of-plane at every section (U4a section 9); "
                       "intensified_bending = hypot(i_ip M_z, i_op M_y)/Z_b",
    "maximum_location": "psi (rad from authored i) and fraction psi/phi on an arc; fraction u on a straight",
    "angles": "degrees in (-180, 180]",
    "fibre_coordinates": "the compared quantities of a named fibre (sigma_plus_fibre, sigma_minus_fibre, "
                         "governing_fibre, the sv_transverse_shear_max fibres) are its coordinates (y, z), at the "
                         "criterion with zero scale r_o; they are formed from the defining vector, so a component "
                         "that is exactly zero is written as an exact \"0\"",
    "beta_deg": "informational, never compared: a fibre's angle in (-180, 180], exactly 0, 90, 180 or -90 on an "
                "axis, so a fibre on the -y axis (z = 0, y < 0) is +180, never -180",
    "sv_maximum_fibres": "the two inner fibres of sv_transverse_shear_max (normal to V) are an unordered pair; the "
                         "oracle lists them by ascending beta_deg and a consumer matches them as a set",
}

DEFINITIONS = {
    "bore_load_transferred_toroidal_membrane_v1": {
        "quantity": "toroidal_membrane_hoop: the area average of the hoop stress over the wall cut at section "
                    "angle beta, from exact equilibrium of the bore pressure transferred to that cut, the arc "
                    "idealized as a complete torus in an axisymmetric membrane state (no transverse shear "
                    "resultant on the cuts, no end or tangent-junction effects, reference geometry, small "
                    "strain, internal p signed, external zero)",
        "closed_form": "sigma_h(beta) = (p r_i,b / t_b) (2R - r_i,b cos beta) / (2R - (r_i,b + r_o) cos beta)",
        "bore": "ruling (a): on each basis b the bore is that basis's own annulus bore r_i,b with wall t_b; the "
                "nominal basis uses r_i,n",
        "domain": "R > r_o (the torus does not self-intersect); the expression stays finite for R > r_m",
        "properties": "crown (cos beta = 0) = p r_i,b / t_b, the straight Lame wall mean; R -> infinity gives "
                      "the same everywhere; thin wall gives the classical (p r/2t)(2R - r cos beta)/(R - r cos "
                      "beta); for p > 0 intrados > crown > extrados (monotone in cos beta)",
        "radial_surface": "-p at the basis's bore, 0 at the outer surface",
        "named_alternate_variant_b": "pressure bore r_i,r loading the nominal wall: p r_i,r (2R - r_i,r cos "
                                     "beta) / (t_n (2R - (r_i,n + r_o) cos beta)); information and discriminator "
                                     "only (its crown differs from the straight nominal Lame mean)"},
    "local_straight_annulus_on_arc": {
        "quantity": "sv_transverse_shear: the Saint-Venant flexure field of the homogeneous straight annulus of "
                    "each basis (SHEAR_REFERENCE INDEPENDENT_REFUTATION section 2), driven by the tangent-frame "
                    "V_y and V_z of each section by superposition, with no curvature correction; on straights the "
                    "same field",
        "field": "C = 1/(2(1+nu) I_b), c = (3+2nu)/4; at (y, z) = r (cos beta, sin beta): tau_r = -C c (r^2 - "
                 "r_o^2)(r^2 - r_i^2)/r^2 (V . e_r), tau_beta = C g(r) (V . e_beta), g(r) = c (r_o^2 + r_i^2 + "
                 "r_o^2 r_i^2 / r^2) - (1-2nu) r^2 / 4",
        "maximum": "tau_max = |V| ((3+2nu) r_o^2 + (1+2nu) r_i^2) / (4 (1+nu) I_b), on the inner surface at the "
                   "two fibres normal to V (beta = atan2(V_z, V_y) +/- 90 deg; an unordered pair), directed along V",
        "domain": "every isotropic -1 < nu < 1/2 with 0 < r_i < r_o (proof in the README: at r = r_i, B0 < 0 < "
                  "B2; g = B2 - B0 decreases strictly in r and exceeds |B0 + B2| <= c (r_o - r_i)^2 elsewhere); "
                  "the location is unique up to the two fibres",
        "limits": "thin wall: 2V/A; small hole: twice the solid-circle centre value (3+2nu)/(2(1+nu)) V/A; "
                  "nu = 0: |V|(3 r_o^2 + r_i^2)/(4I)"},
    "member_maximum": "pipe_elastic_normal_stress_maximum_v2 and the preview maximum: max over the whole member "
                      "of |N_eff|/A_b + hypot(M_y, M_z)/Z_b, per member and basis; headline = maximum over "
                      "members per basis",
    "intensified_bending": "hypot(i_ip M_z, i_op M_y)/Z_b on arcs, per station and basis, never combined with "
                           "anything; axial_plus_intensified_bending = |N_eff|/A_b + intensified_bending is the "
                           "explicitly declared measure of STRESS_REFERENCE I1",
    "intensified_bending_maximum": "the along-arc maximum of the pure intensified bending measure hypot(i_ip M_z, "
                                   "i_op M_y)/Z_b, per arc and basis; no along-arc maximum of "
                                   "axial_plus_intensified_bending is frozen",
}

CRITERION = {
    "rule": "|observed - expected| <= 1e-9 * max(|expected|, zero_scale), in both solver modes",
    "zero_scales": {
        "force": "per member: max |N or S|, |V_y|, |V_z| over its five cuts (N_w rows use it too); if zero, the "
                 "case's largest, then the case's largest |N_w|",
        "moment": "per member: max |T|, |M_y|, |M_z| over its five cuts; if zero, the case's largest",
        "normal_stress": "per member and basis: max over its five stations of max_abs_normal (fibre_normal, "
                         "normal_extremes, member_maximum, intensified rows); if zero, the case's largest for "
                         "that basis",
        "shear_stress": "per member and basis: max over its five stations of sv max + |torsional_shear| (all "
                        "shear rows); if zero, the normal_stress scale",
        "pressure_stress": "per arc and basis: max(|p|, |hoop| over the fibres) (hoop and radial rows)",
        "headline": "the case's largest station max_abs_normal for that basis"},
    "locations": "an end maximum's location is exact; an interior maximum's psi or fraction is compared within its "
                 "location_band, intersected with the member domain (the distance over which the value stays inside the criterion); fibre "
                 "coordinates (y, z) are compared at the criterion with zero scale r_o; beta_deg is informational "
                 "and not compared; the two sv_transverse_shear_max fibres are matched as an unordered pair",
    "location_bands": "an interior psi (arcs) or fraction (straights) is compared within its location_band; end "
                      "maxima are exact and carry no band; an interior headline carries its member's location_band; "
                      "the global f'' >= -K bound gives loss <= |s|*distance + K*distance^2/2 from a supporting "
                      "slope s at the centre; 99% tolerance budget leaves serialization margin, checked explicitly",
    "maxima": "every member and intensified maximum is certified: a rigorous upper bound on the true maximum (from "
              "a second-derivative lower bound) lies within 1e-2 tolerance above the frozen value, which is attained "
              "at the frozen location; the dense-sampling bracket is a coarser consistency check",
    "discriminators": "each listed row lies at least 1e3 tolerances from its expected value, so an observation "
                      "that passes the criterion is more than 1e-9 max(|obs|, |wrong|) from the wrong value",
    "unit_transforms": "values are SI; compare a product's MPa or mm after exact conversion",
}

ALGEBRA = {
    "status": "information only: a classification, not a physics reference (STRESS_REFERENCE section 5)",
    "signed_linear": ["resultants_tangent_frame_j_side", "fibre_normal", "sv_transverse_shear components",
                      "torsional_shear", "flexure_plus_torsion_check components",
                      "toroidal_membrane_hoop (linear in p)", "radial_surface (linear in p)"],
    "never_combined": ["normal_extremes (sigma_plus, sigma_minus, max_abs_normal)", "sv_transverse_shear_max",
                       "member_maximum", "headline", "intensified_bending", "axial_plus_intensified_bending",
                       "intensified_bending_maximum"],
    "rule": "signed linear quantities combine only as matching signed quantities (same frame, station, fibre and "
            "basis) under the same linear operator; maxima and intensified measures are recomputed from the "
            "combined signed resultants or enveloped, never added, subtracted or scaled",
}


# ---------------------------------------------------------------------------
# Documents
# ---------------------------------------------------------------------------

CLAIM = "Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081)."


def emit_json(obj, ind=0):
    """Deterministic JSON: objects indented one space per level, leaf arrays inline."""
    pad = " " * (ind + 1)
    if isinstance(obj, dict):
        if not obj:
            return "{}"
        items = ["%s%s: %s" % (pad, json.dumps(k), emit_json(v, ind + 1)) for k, v in obj.items()]
        return "{\n" + ",\n".join(items) + "\n" + " " * ind + "}"
    if isinstance(obj, list):
        if not obj:
            return "[]"
        if all(not isinstance(x, (dict, list)) for x in obj):
            return json.dumps(obj)
        items = [pad + emit_json(x, ind + 1) for x in obj]
        return "[\n" + ",\n".join(items) + "\n" + " " * ind + "]"
    return json.dumps(obj)


def cases_file(docs):
    return {
        "schema_version": "u4b.cases/1.0.0",
        "purpose": "Invented, statically determinate T4-U4b cases as request-agnostic case descriptions: every "
                   "physical input a model 0.6.0 product request needs. Model 0.6.0, its directional SIF input "
                   "fields and the U4b row kinds are not yet defined, so no product request is frozen here.",
        "claim_fence": CLAIM,
        "units": {"length": "m", "force": "N", "moment": "N*m", "force_per_length": "N/m", "pressure": "Pa",
                  "stress": "Pa", "angle": "rad", "elastic_modulus": "Pa", "thermal_expansion": "1/K"},
        "number_conventions": {
            "binary64_exact": "node positions, y_reference components and load components are the binary64 "
                              "values written here, taken as exact",
            "decimal_targets": "outside diameter, wall, mill tolerance, corrosion allowance, E, nu, alpha, bend "
                               "radius, pressure, k and SIF values are exact decimal targets (the shortest decimal "
                               "of the written number); their binary64 rounding moves results by about 1e-16 "
                               "relative, far inside the criterion",
            "bend_angle": "each arc's arc.bend_angle_binary64_for_authoring (repeated in the oracle's "
                          "derived.arc) is the binary64 nearest to the included angle formed from the chord and R, "
                          "for the bend component that states it"},
        "fields": {
            "route": "exact: a pressure_contract document (identity pressure-2 when defined); preview: a "
                     "pressure-free document on the preview-physics route (identity preview-physics-2 when "
                     "defined)",
            "members.kind": "straight, or realized_arc (a span bend realized as curved_bend_macro_element)",
            "members.y_reference": "on a straight the element-local y reference; on an arc the bow direction "
                                   "(the arc bows toward +y_reference projected normal to the chord)",
            "members.arc": "bend_radius R; bend_angle_binary64_for_authoring; bend_component_attachment_node, the "
                           "arc's authored from node (as in U4a); the bow rule; k",
            "members.directional_sif_user_values": "invented i_ip (on M_z, in-plane) and i_op (on M_y, "
                                                   "out-of-plane) in the arc's tangent frame",
            "load_case.member_uniform_forces": "constant global force per unit length: per unit member length on a "
                                               "straight, per unit arc length on an arc (length_measure)",
            "load_case.pressure_regions": "internal pressure p (signed; external zero) over the listed members, "
                                          "each terminal with its closure disposition",
            "supports": "one full anchor; the other line end is free"},
        "binding_when_0_6_0_is_defined": [
            "a builder authors one product request per case from these descriptions (bend components, primitive "
            "loads per global component, pressure region and terminals, shear model, SIF fields);",
            "each realized arc becomes one bend component attached at arc.bend_component_attachment_node (the "
            "arc's authored from node, as in U4a), referencing the arc's pipe, with bend_radius R, bend angle "
            "arc.bend_angle_binary64_for_authoring, the user k and the SIF fields;",
            "the U4b design's row-kind table maps the oracle's agnostic quantity keys to row kinds;",
            "oracle values never move to match a product;",
            "the request SHA-256 values are added to the oracle when the requests exist."],
        "cases": docs}


def build(check_time=False):
    t0 = time.time()
    set_precision(BASE_PREC)
    check_location_band_edge_cases()
    docs = case_documents()
    cases_bytes = (emit_json(cases_file(docs)) + "\n").encode()
    rule = gauss_legendre(40)
    rule_t = gauss_legendre(24)
    rule_r = gauss_legendre(16)
    timing = {}
    analyses, emitted, statics = {}, {}, {}
    all_disc = {}
    SnapLog.reset()
    case_snaps = 0
    for doc in docs:
        t1 = time.time()
        line = line_from_doc(doc)
        an = analyse(line)
        before = SnapLog.count
        em, disc = emit_case(an, doc)
        case_snaps += SnapLog.count - before
        analyses[line.id] = an
        emitted[line.id] = em
        all_disc[line.id] = em["discriminators"]
        statics[line.id] = check_statics(line, rule)
        timing[line.id] = time.time() - t1
    # SV qualification on every annulus the cases publish
    sv_q = {}
    seen = set()
    for cid, an in analyses.items():
        for m, fwd in an.line.order:
            for basis in an.line.bases:
                bp = an.line.basis_props[m.id][basis]
                nu = an.line.nu(m.id)
                key = (fmt(bp["ro"]), fmt(bp["ri"]), fmt(nu))
                if key in seen:
                    continue
                seen.add(key)
                sv_q["%s/%s/%s" % (cid, m.id, basis)] = sv_qualify(bp["ro"], bp["ri"], nu, rule)
    sv_lim = sv_limits()
    # hoop qualification on every pressurized arc and basis
    hoop_q = {}
    for cid, an in analyses.items():
        for (mid, basis), h in an.hoop.items():
            hoop_q["%s/%s/%s" % (cid, mid, basis)] = hoop_qualify(h["p"], h["ri"], h["ro"], h["R"], rule, rule_t,
                                                                   rule_r)
    # twin and variant (b)
    a1, a2 = analyses["U4B-L90-P"], analyses["U4B-L90-P-COINCIDENT"]
    eq_twin = all(a2.q[(m.id, st, "nominal_section")] == a2.q[(m.id, st, "reduced_section")]
                  for m, f in a2.line.order for st, fr in STATIONS)
    eq_hoop = all(a1.hoop[("pipe:BEND", "nominal_section")]["values"][n] ==
                  a2.hoop[("pipe:BEND", b)]["values"][n] for n, ang in FIBRES for b in a2.line.bases)
    require(eq_twin and eq_hoop, "twin")
    bm = a1.line.mby["pipe:BEND"]
    hb = a1.hoop[("pipe:BEND", "nominal_section")]
    rib = a1.line.basis_props["pipe:BEND"]["_bore"]["ri"]
    crown_b = toroidal_hoop_variant_b(hb["p"], rib, hb["ri"], hb["ro"], bm.R, D(0))
    zt = a1.zs["pipe:BEND"]["nominal_section"]["pressure_stress"]
    twin = {"coincident_case_nominal_equals_reduced_everywhere": eq_twin,
            "nominal_hoop_of_U4B-L90-P_equals_both_hoops_of_the_coincident_twin": eq_hoop,
            "variant_b_crown_on_U4B-L90-P_nominal": V(crown_b),
            "straight_lame_wall_mean_nominal": V(hb["lame_mean"]),
            "variant_b_crown_minus_straight_mean_tolerances": sci((crown_b - hb["lame_mean"]) /
                                                                  tol(hb["lame_mean"], zt)),
            "meaning": "ruling (a) makes the arc crown equal the straight nominal Lame wall mean p r_i,n / t_n on "
                       "each basis; variant (b) (pressure bore r_i,r loading the nominal wall) breaks that equality"}
    obj = check_objectivity(analyses["U4B-L90-P"], analyses["U4B-L90-P-MOVED"])
    closed = arc106_closed_form(analyses["U4B-ARC106-RADIAL-NEGP"])
    maxima = maxima_summary(emitted, analyses)
    u4a = u4a_cross_check()
    sref = stress_reference_reproduction()
    require(SnapLog.count == case_snaps, "a value outside the cases' emission was written as a residue zero")
    require(not SnapLog.floor_violations, "a written nonzero value lies below the legitimate floor: %s"
            % SnapLog.floor_violations[:3])
    stability = precision_stability(docs, emitted, case_snaps)
    # discriminator summary: a mutant is not formed in a case that has no row it
    # could apply to (for example a hoop mutant on an unpressurized case, or the
    # nu = 0.3 mutant where nu is 0.3); a formed mutant may still not be
    # discriminable (it moves no row by 1e3 tolerances)
    summary = {}
    for cid, dd in all_disc.items():
        for mutant, rec in dd.items():
            s = summary.setdefault(mutant, {"discriminable_in": [], "not_discriminable_in": [], "not_formed_in": [],
                                            "smallest_listed_separation_tolerances": None})
            if rec["discriminable_in_this_case"]:
                s["discriminable_in"].append(cid)
                v = D(rec["smallest_listed_separation_tolerances"])
                cur = s["smallest_listed_separation_tolerances"]
                if cur is None or v < D(cur):
                    s["smallest_listed_separation_tolerances"] = sci(v)
            else:
                s["not_discriminable_in"].append(cid)
    for mutant, s in summary.items():
        s["not_formed_in"] = [cid for cid in all_disc if mutant not in all_disc[cid]]
        require(len(s["discriminable_in"]) + len(s["not_discriminable_in"]) + len(s["not_formed_in"])
                == len(all_disc), "discriminator summary partition: " + mutant)
        require(s["discriminable_in"], "mutant never discriminable: " + mutant)
    oracle = {
        "schema_version": "u4b.oracle/1.0.0",
        "purpose": "Frozen independent expectations for T4-U4b (DEL-04-07 Design arc-frames-and-signs.md section "
                   "9): arc stress quantities on realized arcs, with the straight members of the same lines, on "
                   "both D-5 section bases (exact route) or the one preview section.",
        "claim_fence": CLAIM,
        "cases_file": "cases.json",
        "cases_sha256": hashlib.sha256(cases_bytes).hexdigest(),
        "request_sha256": "none yet: model 0.6.0 requests are authored from cases.json when defined",
        "generator": "generators/u4b_reference.py (standard library; decimal at %d significant digits)" % BASE_PREC,
        "value_format": "every value is [decimal with %d significant digits, binary64 nearest]; \"0\" is exact "
                        "zero: either known exactly (a fibre coordinate formed from a vector component that is "
                        "exactly zero) or an arithmetic residue of the %d-digit working precision below %s in SI "
                        "units; the generator asserts that no written nonzero value is below %s and that every such "
                        "residue shrinks below %s at %d digits; vectors under "
                        "derived.*.tangent_frames_global_components are informational decimals"
                        % (OUT_DIGITS, BASE_PREC, sci(SNAP_BELOW, 1), sci(LEGIT_FLOOR, 1),
                           sci(RESIDUE_AT_HIGH_PREC, 1), BASE_PREC + 20),
        "conventions": CONVENTIONS,
        "definitions": DEFINITIONS,
        "criterion": CRITERION,
        "cases": emitted,
        "discriminator_summary": summary,
        "markers": marker_control(),
        "algebra_classes_informational": ALGEBRA,
        "cross_checks": {
            "u4a": u4a,
            "stress_reference": sref,
            "saint_venant_field": {"per_annulus": sv_q, "limits": sv_lim},
            "toroidal_hoop": hoop_q,
            "statics": statics,
            "objectivity": {"pair": ["U4B-L90-P", "U4B-L90-P-MOVED"],
                            "max_difference_in_tolerances": sci(obj),
                            "meaning": "every frame-local resultant, station stress quantity and member maximum "
                                       "of the moved copy equals the original's within this fraction of one "
                                       "tolerance; the residue is the binary64 rounding of the moved inputs"},
            "twin_and_variant_b": twin,
            "arc_maximum_closed_form_U4B-ARC106": closed,
            "maxima": maxima,
            "precision_stability": stability,
        },
    }
    oracle_bytes = (emit_json(oracle) + "\n").encode()
    return cases_bytes, oracle_bytes, timing, time.time() - t0


def arc106_closed_form(an):
    """Radial tip force F toward the centre, p < 0: N_eff = -(F sin theta + |p| Ai), M_z = F R sin theta,
    theta = phi - psi, so f = |p| Ai/A + F sin theta (1/A + R/Z) peaks at theta = 90 deg exactly."""
    line = an.line
    m = line.mby["pipe:BEND"]
    F = vnorm(line.nf[line.tip])
    out = {"psi_star_closed_form": "phi - pi/2 = 2 atan(4/3) - pi/2", "psi_star": V(m.phi - K.PI / 2)}
    for basis in line.bases:
        bp = line.basis_props[m.id][basis]
        fstar = abs(line.pAi(m.id)) / bp["A"] + F * (1 / bp["A"] + m.R / bp["Z"])
        istar = m.sif[0] * F * m.R / bp["Z"]
        mx, ix = an.mmax[(m.id, basis)], an.imax[(m.id, basis)]
        e = max(abs(mx["value"] - fstar) / fstar, abs(ix["value"] - istar) / istar,
                abs(mx["psi"] - (m.phi - K.PI / 2)), abs(ix["psi"] - (m.phi - K.PI / 2)))
        out[basis] = {"f_star_closed_form": "|p| Ai/A_b + F (1/A_b + R/Z_b)", "f_star": V(fstar),
                      "intensified_star_closed_form": "i_ip F R / Z_b", "intensified_star": V(istar),
                      "max_difference_from_the_general_method": sci(e)}
        require(e < D(10) ** -40, "ARC106 closed form")
    return out


def maxima_summary(emitted, analyses):
    n_arc = n_str = n_int = 0
    n_str_interior = 0
    worst_excess = D(0)
    rel_w = []
    bis = 0
    for cid, em in emitted.items():
        q = em["quantities"]
        an = analyses[cid]
        recs = []
        for mid, bb in q["member_maximum"].items():
            for b, rec in bb.items():
                recs.append((rec, an.mmax[(mid, b)]))
                if "psi_rad" in rec["location"]:
                    n_arc += 1
                else:
                    n_str += 1
                    n_str_interior += rec["location"]["kind"] == "interior"
        for mid, bb in q.get("intensified_bending_maximum", {}).items():
            for b, rec in bb.items():
                recs.append((rec, an.imax[(mid, b)]))
                n_int += 1
        for rec, mx in recs:
            br = rec["dense_sampling_bracket"]
            require(br["analytic_inside"], "bracket")
            v = D(rec["value"][0])
            rel_w.append((D(br["upper_max_plus_L_h_over_2"][0]) - D(br["lower_max_of_samples"][0])) / v)
            c = mx["cert"]
            worst_excess = max(worst_excess, c["excess_tolerances"])
            bis += c["bisections"]
    require(worst_excess <= CERT_FRACTION, "certification")
    d2 = max(v for an in analyses.values() for v in an.d2check.values())
    d2s = [mx["d2_central_difference_error"] for an in analyses.values() for mx in an.mmax.values()
           if "d2_central_difference_error" in mx]
    require(len(d2s) == n_str_interior, "every interior straight maximum has a checked second derivative")
    return {"arc_member_maxima": n_arc, "straight_member_maxima": n_str,
            "straight_member_maxima_interior": n_str_interior, "arc_intensified_maxima": n_int,
            "every_analytic_value_inside_its_dense_sampling_bracket": True,
            "dense_sampling_bracket_relative_width_range": [sci(min(rel_w)), sci(max(rel_w))],
            "dense_sampling_bracket_role": "a coarse consistency check of the analytic value against 4,097 samples "
                                          "and a rigorous Lipschitz bound; its width is far wider than the criterion, "
                                          "so it does not by itself certify the maximum",
            "every_maximum_certified_at_the_criterion": True,
            "certification_worst_upper_bound_minus_value_tolerances": sci(worst_excess),
            "certification_total_bisections": bis,
            "certification_meaning": "for every member and intensified maximum, a rigorous upper bound from f'' >= -K "
                                     "(K from the closed-form second derivatives) lies within %s tolerance of the "
                                     "frozen value, and no sample exceeds it; so the sign-change search missed no "
                                     "higher peak" % sci(CERT_FRACTION, 1),
            "arc_second_derivative_formulas_worst_relative_error": sci(d2),
            "every_station_only_maximum_at_most_the_analytic_maximum": True,
            "straight_interior_second_derivative_worst_relative_error": sci(max(d2s)),
            "straight_interior_second_derivative_check": "the exact f'' in the fraction at each interior straight "
                                                         "maximum (from the polynomial resultants), reported separately "
                                                         "from the global lower-curvature bound that sets its location_band, "
                                                         "against a central second difference of f "
                                                         "with step 1e-20"}


VECTOR_KEYS = {"x", "y", "z", "point", "centre", "bend_plane_normal_z"}
STABILITY_PARTS = ("derived", "zero_scales", "quantities", "effective_axial_force_zeros_on_arcs")


def value_leaves(obj, path=""):
    """Every written value: [decimal, binary64] pairs, and the decimal strings of
    the derived vectors."""
    if isinstance(obj, dict):
        for k, v in obj.items():
            yield from value_leaves(v, path + "/" + k)
    elif isinstance(obj, list):
        if len(obj) == 2 and isinstance(obj[0], str) and isinstance(obj[1], float):
            yield path, obj[0]
        elif obj and all(isinstance(x, str) for x in obj) and path.split("/")[-1] in VECTOR_KEYS:
            for i, v in enumerate(obj):
                yield path + "/%d" % i, v
        else:
            for i, v in enumerate(obj):
                yield from value_leaves(v, path + "/%d" % i)


def location_band_repair_leaves(em):
    """Paths of the values the location-band repair added (README, Status): the band and
    second derivative of each interior straight maximum, and the band an interior headline
    carries. precision_stability compares them like every other value and counts them
    separately, so the count confirmed before the repair is unchanged."""
    q = em["quantities"]
    out = set()
    for mid, bb in q["member_maximum"].items():
        for b, rec in bb.items():
            if "psi_rad" in rec["location"]:
                continue
            for key in ("location_band", "second_derivative_per_fraction2"):
                if key in rec:
                    out |= {p for p, v in value_leaves(rec[key], "quantities/member_maximum/%s/%s/%s"
                                                       % (mid, b, key))}
    for b, h in q["headline"].items():
        if "location_band" in h:
            out |= {p for p, v in value_leaves(h["location_band"], "quantities/headline/%s/location_band" % b)}
    return out


def precision_stability(docs, emitted, case_snaps):
    """Recompute every case at BASE_PREC + 20 digits and compare every written value
    of its derived geometry, zero scales, quantities and N_w zeros. An exact zero
    must be an exact zero at both precisions, and every residue written as zero
    must shrink with the precision (README, exact zeros)."""
    hi = BASE_PREC + 20
    set_precision(hi)
    SnapLog.reset()
    worst = D(0)
    count = zeros = added = 0
    try:
        for doc in docs:
            an = analyse(line_from_doc(doc))
            em, _ = emit_case(an, doc)
            band_paths = location_band_repair_leaves(emitted[doc["case_id"]])
            for part in STABILITY_PARTS:
                require((part in em) == (part in emitted[doc["case_id"]]), "precision stability parts")
                if part not in em:
                    continue
                a = dict(value_leaves(emitted[doc["case_id"]][part], part))
                b = dict(value_leaves(em[part], part))
                require(a.keys() == b.keys(), "precision stability keys")
                for k in a:
                    x, y = D(a[k]), D(b[k])
                    if k in band_paths:
                        added += 1
                    else:
                        count += 1
                    if x == y:
                        zeros += x == 0
                        continue
                    require(x != 0 and y != 0, "a zero at one precision is nonzero at the other: %s %s"
                            % (doc["case_id"], k))
                    worst = max(worst, abs(x - y) / max(abs(x), abs(y)))
        hi_snaps, hi_largest, hi_floor = SnapLog.count, SnapLog.largest, list(SnapLog.floor_violations)
    finally:
        set_precision(BASE_PREC)
    require(worst < D(10) ** -38, "precision stability")
    require(hi_snaps == case_snaps and hi_largest < RESIDUE_AT_HIGH_PREC and not hi_floor,
            "residue rule: %d/%d snaps, largest %s at %d digits" % (hi_snaps, case_snaps, sci(hi_largest), hi))
    return {"recomputed_at_digits": hi, "parts_compared": list(STABILITY_PARTS), "values_compared": count,
            "values_compared_added_by_the_location_band_repair": added,
            "exact_zeros_equal_at_both_precisions": zeros, "worst_relative_difference": sci(worst),
            "residues_written_as_zero": case_snaps,
            "largest_such_residue_at_recomputation": sci(hi_largest),
            "meaning": "every written %d-digit value of the cases is stable against 20 more working digits; an "
                       "exact zero is exactly zero at both precisions; every residue written as zero (below %s "
                       "in SI units at %d digits) is below %s at %d digits, so it shrinks with the working "
                       "precision; no written nonzero value is below %s"
                       % (OUT_DIGITS, sci(SNAP_BELOW, 1), BASE_PREC, sci(RESIDUE_AT_HIGH_PREC, 1), hi,
                          sci(LEGIT_FLOOR, 1))}


EXPECTED_FILES = ["README.md", "SHA256SUMS", "cases.json", "oracle.json", "generators/u4b_reference.py"]


def sums_bytes(files):
    lines = []
    for name in ["cases.json", "oracle.json", "README.md", "generators/u4b_reference.py"]:
        lines.append("%s  %s" % (hashlib.sha256(files[name]).hexdigest(), name))
    return ("\n".join(lines) + "\n").encode()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--write", action="store_true")
    args = ap.parse_args()
    cases_bytes, oracle_bytes, timing, total = build()
    files = {"cases.json": cases_bytes, "oracle.json": oracle_bytes}
    readme = os.path.join(ROOT, "README.md")
    files["README.md"] = open(readme, "rb").read() if os.path.exists(readme) else b""
    files["generators/u4b_reference.py"] = open(os.path.abspath(__file__), "rb").read()
    files["SHA256SUMS"] = sums_bytes(files)
    if args.write:
        for name in ("cases.json", "oracle.json", "SHA256SUMS"):
            with open(os.path.join(ROOT, name), "wb") as f:
                f.write(files[name])
        print("wrote cases.json, oracle.json, SHA256SUMS (%.1f s)" % total)
        return 0
    bad = []
    present = []
    for dp, dn, fn in os.walk(ROOT):
        for f in fn:
            present.append(os.path.relpath(os.path.join(dp, f), ROOT))
    extra = sorted(set(present) - set(EXPECTED_FILES))
    missing = sorted(set(EXPECTED_FILES) - set(present))
    if extra:
        bad.append("unexpected files: %s" % extra)
    if missing:
        bad.append("missing files: %s" % missing)
    for name in ("cases.json", "oracle.json", "SHA256SUMS"):
        p = os.path.join(ROOT, name)
        if os.path.exists(p) and open(p, "rb").read() != files[name]:
            bad.append("%s differs from the rebuilt bytes" % name)
    if bad:
        for b in bad:
            print("FAIL:", b)
        return 1
    print("OK: cases.json, oracle.json and SHA256SUMS reproduce byte for byte; every cross-check passed "
          "(%.1f s)" % total)
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Fail as e:
        print("FAIL:", e)
        sys.exit(1)
