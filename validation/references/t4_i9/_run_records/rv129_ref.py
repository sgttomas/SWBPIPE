"""RV129: an independent arbitrary-precision reference for T4-I9's arc
load-vector certificate (review of T4-U1b's design, not product code).

Standard library only. Run with `python -I rv129_ref.py <probe path> [--quick]`.

Method (deliberately different from the design's probe):
- Decimal arithmetic at PREC digits, with this script's own pi (Machin),
  own sin/cos (quadrant reduction + Taylor) and own half angle (Newton on
  sin or cos), not the probe's atan series or half-angle identities.
- The arc frame comes from CB's centre-based construction (centre from the
  objective definition, e_x = r_i/|r_i|, e_z = r_i x r_j normalised,
  e_y = e_z x e_x), not the probe's (s, c_h, d, n) formulas.
- Unit loads are applied at node j along the six GLOBAL axes (not local),
  and every internal action is formed from 3D vectors at the section:
  M = (p(phi) - p(theta)) x f + m. The distributed load's section moment
  is itself an inner Gauss-Legendre integral of (p(psi) - p(theta)) x w R.
- Flexibility and tip deflection are Gauss-Legendre quadratures (no
  closed-form Gram entries), F^-1 by Decimal Gaussian elimination with
  partial pivoting, and the node-i share from global rigid equilibrium.

The certificate under test is the design's own ball evaluation
(`formula(BallCtx(), ...)` in T4-I9's arc_cert_probe.py, which implements
L0-L7 with exact-rational radii rounded up once per operation). For each
case the script checks |ref_k - m_k| <= r_k exactly (Fractions), and
reports the binary64 objective element's defect and the probe's guard
statistic computed against this reference.
"""
import importlib.util
import math
import random
import sys
import time
from decimal import Decimal as D, getcontext, localcontext
from fractions import Fraction as Q

PREC = 120
NODES = 56
NODES_INNER = 56

# ----------------------------------------------------------- elementary

_PI = {}


def pi_dec():
    prec = getcontext().prec
    if prec in _PI:
        return _PI[prec]
    with localcontext() as ctx:
        ctx.prec = prec + 10

        def atan_inv(n):
            x = D(1) / n
            x2 = x * x
            term, total, k = x, x, 1
            eps = D(10) ** -(prec + 8)
            while abs(term) > eps:
                term = -term * x2
                total += term / (2 * k + 1)
                k += 1
            return total
        val = 16 * atan_inv(5) - 4 * atan_inv(239)
    _PI[prec] = +val
    return _PI[prec]


def _taylor_sin_cos(r):
    eps = D(10) ** -(getcontext().prec + 5)
    s, term, n = D(0), r, 1
    while abs(term) > eps:
        s += term
        term = -term * r * r / ((n + 1) * (n + 2))
        n += 2
    c, term, n = D(0), D(1), 0
    while abs(term) > eps:
        c += term
        term = -term * r * r / ((n + 1) * (n + 2))
        n += 2
    return s, c


def sin_cos(x):
    """sin and cos of a Decimal x, by quadrant reduction and Taylor series."""
    with localcontext() as ctx:
        ctx.prec += 10
        half_pi = pi_dec() / 2
        k = int((x / half_pi).to_integral_value())
        r = x - k * half_pi
        s, c = _taylor_sin_cos(r)
        k %= 4
        if k == 0:
            out = (s, c)
        elif k == 1:
            out = (c, -s)
        elif k == 2:
            out = (-s, -c)
        else:
            out = (-c, s)
    return +out[0], +out[1]


def half_angle(s, c_h):
    """x in (0, pi/2) with sin x = s, cos x = c_h, by Newton iteration."""
    eps = D(10) ** -(getcontext().prec - 5)
    if s < D("0.7"):
        x = D(repr(math.asin(float(s))))
        for _ in range(60):
            sx, cx = sin_cos(x)
            step = (sx - s) / cx
            x -= step
            if abs(step) < eps:
                break
    else:
        x = D(repr(math.acos(float(c_h))))
        for _ in range(60):
            sx, cx = sin_cos(x)
            step = (cx - c_h) / (-sx)
            x -= step
            if abs(step) < eps:
                break
    return x


_GL = {}


def gauss_legendre(n):
    key = (n, getcontext().prec)
    if key in _GL:
        return _GL[key]
    nodes = []
    eps = D(10) ** -(getcontext().prec - 4)
    for i in range(1, n + 1):
        x = D(repr(math.cos(math.pi * (i - 0.25) / (n + 0.5))))
        for _ in range(100):
            p0, p1 = D(1), x
            for k in range(1, n):
                p0, p1 = p1, ((2 * k + 1) * x * p1 - k * p0) / (k + 1)
            dp = n * (x * p1 - p0) / (x * x - 1)
            step = p1 / dp
            x -= step
            if abs(step) < eps:
                break
        p0, p1 = D(1), x
        for k in range(1, n):
            p0, p1 = p1, ((2 * k + 1) * x * p1 - k * p0) / (k + 1)
        dp = n * (x * p1 - p0) / (x * x - 1)
        w = 2 / ((1 - x * x) * dp * dp)
        nodes.append((x, w))
    _GL[key] = nodes
    return nodes


def vadd(a, b):
    return [a[0] + b[0], a[1] + b[1], a[2] + b[2]]


def vsub(a, b):
    return [a[0] - b[0], a[1] - b[1], a[2] - b[2]]


def vscale(k, a):
    return [k * a[0], k * a[1], k * a[2]]


def vdot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def vcross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def vnorm(a):
    return vdot(a, a).sqrt()


def solve(F, rhs):
    n = len(F)
    a = [list(F[r]) + [rhs[r]] for r in range(n)]
    for col in range(n):
        piv = max(range(col, n), key=lambda r: abs(a[r][col]))
        a[col], a[piv] = a[piv], a[col]
        for r in range(col + 1, n):
            f = a[r][col] / a[col][col]
            for k in range(col, n + 1):
                a[r][k] -= f * a[col][k]
    x = [D(0)] * n
    for r in reversed(range(n)):
        s = a[r][n]
        for k in range(r + 1, n):
            s -= a[r][k] * x[k]
        x[r] = s / a[r][r]
    return x

# ------------------------------------------------------------ reference


def reference(xi, xj, radius, yref, em, gm, area, inertia, torsion, kin, kout, w,
              prec=PREC, nodes=NODES, inner=NODES_INNER):
    with localcontext() as ctx:
        ctx.prec = prec
        X_i = [D(v) for v in xi]
        X_j = [D(v) for v in xj]
        R = D(radius)
        y = [D(v) for v in yref]
        wv = [D(v) for v in w]
        E, G, A, I, J = D(em), D(gm), D(area), D(inertia), D(torsion)
        k_in, k_out = D(kin), D(kout)
        d = vsub(X_j, X_i)
        L = vnorm(d)
        s = L / (2 * R)
        c_h = (1 - s * s).sqrt()
        dh = vscale(1 / L, d)
        nr = vsub(y, vscale(vdot(y, dh), dh))
        nh = vscale(1 / vnorm(nr), nr)
        # CB's centre-based frame from the objective centre (PP's bow convention).
        centre_rel_i = vsub(vscale(D(1) / 2, d), vscale(R * c_h, nh))  # x_c - x_i
        r_i = vscale(-1, centre_rel_i)                                  # x_i - x_c
        r_j = vsub(d, centre_rel_i)                                     # x_j - x_c
        e_x = vscale(1 / vnorm(r_i), r_i)
        pl = vcross(r_i, r_j)
        e_z = vscale(1 / vnorm(pl), pl)
        e_y = vcross(e_z, e_x)
        phi = 2 * half_angle(s, c_h)

        def u(theta):
            sn, cs = sin_cos(theta)
            return vadd(vscale(cs, e_x), vscale(sn, e_y)), sn, cs

        u_phi, _, _ = u(phi)
        gl = gauss_legendre(nodes)
        gli = gauss_legendre(inner)
        half = phi / 2
        EI, GJ, EA = E * I, G * J, E * A
        Fm = [[D(0)] * 6 for _ in range(6)]
        dl = [D(0)] * 6
        m_load_i = None
        units = []
        for a in range(3):
            e = [D(0)] * 3
            e[a] = D(1)
            units.append(("f", e))
        for a in range(3):
            e = [D(0)] * 3
            e[a] = D(1)
            units.append(("m", e))

        def load_moment(theta, u_theta):
            # inner integral of (p(psi) - p(theta)) x w R over [theta, phi]
            lo, hi = theta, phi
            mid, hl = (lo + hi) / 2, (hi - lo) / 2
            acc = [D(0)] * 3
            for x, wt in gli:
                psi = mid + hl * x
                u_psi, _, _ = u(psi)
                arm = vscale(R, vsub(u_psi, u_theta))
                acc = vadd(acc, vscale(wt * hl * R, vcross(arm, wv)))
            return acc

        for x, wt in gl:
            theta = half + half * x
            weight = wt * half * R
            u_t, sn, cs = u(theta)
            r_dir = u_t
            t_dir = vadd(vscale(-sn, e_x), vscale(cs, e_y))
            z_dir = e_z
            arm_j = vscale(R, vsub(u_phi, u_t))
            acts = []
            for kind, e in units:
                if kind == "f":
                    Fv, Mv = e, vcross(arm_j, e)
                else:
                    Fv, Mv = [D(0)] * 3, e
                acts.append((vdot(Mv, z_dir), vdot(Mv, r_dir), vdot(Mv, t_dir), vdot(Fv, t_dir)))
            Fw = vscale(R * (phi - theta), wv)
            Mw = load_moment(theta, u_t)
            lw = (vdot(Mw, z_dir), vdot(Mw, r_dir), vdot(Mw, t_dir), vdot(Fw, t_dir))

            def energy(p, q):
                return (k_in * p[0] * q[0] / EI + k_out * p[1] * q[1] / EI
                        + p[2] * q[2] / GJ + p[3] * q[3] / EA)
            for r in range(6):
                for c in range(r, 6):
                    Fm[r][c] += weight * energy(acts[r], acts[c])
                dl[r] += weight * energy(acts[r], lw)
        for r in range(6):
            for c in range(r):
                Fm[r][c] = Fm[c][r]
        Xs = solve(Fm, [-v for v in dl])         # X = -F^-1 delta (global, at j)
        Xf, Xm = Xs[:3], Xs[3:]
        u0, _, _ = u(D(0))
        m_load_i = load_moment(D(0), u0)        # moment of the load about node i
        chord = vscale(R, vsub(u_phi, u0))       # p(phi) - p(0)
        p_i_f = vadd(Xf, vscale(R * phi, wv))
        p_i_m = vadd(vadd(vcross(chord, Xf), Xm), m_load_i)
        p_j_f = [-v for v in Xf]
        p_j_m = [-v for v in Xm]
        out = p_i_f + p_i_m + p_j_f + p_j_m
        return [+v for v in out], phi, Fm

# ---------------------------------------------------------------- cases


def section(od, wall):
    ro, ri = od / 2, od / 2 - wall
    area = math.pi * (ro * ro - ri * ri)
    inertia = math.pi * (ro ** 4 - ri ** 4) / 4
    return area, inertia, 2 * inertia


def unit(v):
    n = math.sqrt(sum(c * c for c in v))
    return [c / n for c in v]


def arc_nodes(origin, chord_dir, plane_dir, radius, phi):
    """End nodes of an arc of included angle phi whose chord runs along
    chord_dir from origin, bowing toward plane_dir, and a y_reference."""
    dh = unit(chord_dir)
    p = plane_dir
    pr = sum(p[k] * dh[k] for k in range(3))
    nh = unit([p[k] - pr * dh[k] for k in range(3)])
    length = 2.0 * radius * math.sin(0.5 * phi)
    xi = list(origin)
    xj = [origin[k] + length * dh[k] for k in range(3)]
    yref = [nh[k] + 0.37 * dh[k] for k in range(3)]   # not orthogonal to the chord
    return xi, xj, yref


def build_cases(quick=False):
    rng = random.Random(129)
    steel = (2.03e11, 2.03e11 / 2.6)
    cases = []

    def add(label, phi_deg=None, phi_rad=None, origin=(3.0, 0.0, 0.0), chord=(1.0, 0.0, 0.0),
            plane=(0.0, -1.0, 0.0), R=0.2286, od=0.1683, wall=0.00711, mat=steel,
            kin=1.0, kout=1.0, w=(0.0, 0.0, -450.0)):
        phi = phi_rad if phi_rad is not None else math.radians(phi_deg)
        xi, xj, yref = arc_nodes(origin, chord, plane, R, phi)
        area, inertia, torsion = section(od, wall)
        cases.append(dict(label=label, args=(xi, xj, R, yref, mat[0], mat[1], area, inertia,
                                              torsion, kin, kout, list(w))))

    # angles the probe did not choose (L-line geometry, ordinary coordinates)
    for deg in (30.0, 60.0, 120.0, 150.0, 170.0, 179.0, 179.9):
        add("angle %g deg" % deg, phi_deg=deg)
    add("angle pi-1e-6 rad", phi_rad=math.pi - 1e-6)
    add("angle pi-1e-8 rad", phi_rad=math.pi - 1e-8)
    for deg in (0.5, 0.05):
        add("angle %g deg" % deg, phi_deg=deg)
    for rad in (1e-4, 1e-6):
        add("angle %g rad" % rad, phi_rad=rad)
    # UTM and other offsets, skew planes
    utm = (7.3e6 + 0.123456789, 4.6e6 - 7.654321, 312.5 + 1.0 / 3.0)
    add("UTM skew 45 deg", phi_deg=45.0, origin=utm, chord=(0.6, -0.3, 0.74),
        plane=(0.2, 0.9, -0.1), w=(0.0, 0.0, -450.0))
    add("UTM skew 7 deg in-plane", phi_deg=7.0, origin=utm, chord=(0.1, 0.99, 0.0),
        plane=(1.0, 0.0, 0.3), w=(-450.0, 0.0, 0.0))
    add("UTM skew 0.2 deg", phi_deg=0.2, origin=utm, chord=(-0.7, 0.7, 0.14),
        plane=(0.0, 0.0, 1.0), w=(0.0, 300.0, 0.0))
    add("UTM skew 178 deg", phi_deg=178.0, origin=utm, chord=(0.3, 0.3, -0.9),
        plane=(1.0, -1.0, 0.0), w=(0.0, 0.0, -450.0))
    add("far offset 5e5/6e6/-12", phi_deg=90.0, origin=(5.0e5, 6.0e6, -12.0),
        chord=(0.0, 1.0, 0.0), plane=(-1.0, 0.0, 0.0), w=(0.0, 0.0, -450.0))
    # large and small k, unequal factors
    for k in (1e-3, 30.0, 1e4, 1e8, 1e15, 1e20):
        add("k=%g 90 deg" % k, phi_deg=90.0, kin=k, kout=k)
    add("k_in=3 k_out=0.5 60 deg skew", phi_deg=60.0, chord=(0.3, 0.4, 0.5), plane=(0.0, 0.0, 1.0),
        kin=3.0, kout=0.5, w=(0.0, -450.0, 0.0))
    # mixed scales: radius, section, material, load magnitude
    add("R=0.05 small bar 90 deg", phi_deg=90.0, R=0.05, od=0.0213, wall=0.00277)
    add("R=25 big thin 20 deg", phi_deg=20.0, R=25.0, od=1.2192, wall=0.00635)
    add("R=25 big thin 0.3 deg UTM", phi_deg=0.3, R=25.0, od=1.2192, wall=0.00635, origin=utm,
        chord=(0.2, 0.9, 0.1), plane=(0.0, 0.0, 1.0))
    add("aluminium 75 deg", phi_deg=75.0, mat=(7.0e10, 7.0e10 / 2.66))
    add("G<<E 90 deg", phi_deg=90.0, mat=(2.0e11, 1.0e9))
    add("huge w 1e9 skew", phi_deg=33.0, chord=(0.2, -0.5, 0.8), plane=(1.0, 0.0, 0.0), w=(0.0, 1.0e9, 0.0))
    add("tiny w 1e-6", phi_deg=33.0, w=(1.0e-6, 0.0, 0.0))
    add("generic w (3 components)", phi_deg=100.0, chord=(0.5, 0.5, 0.5), plane=(0.0, 1.0, -1.0),
        w=(123.4, -56.7, -890.1))
    # random arcs with full-mantissa inputs
    for idx in range(6):
        phi = rng.uniform(0.01, 3.1)
        origin = (rng.uniform(-1e6, 1e6), rng.uniform(-1e6, 1e6), rng.uniform(-100.0, 100.0))
        chord = (rng.uniform(-1, 1), rng.uniform(-1, 1), rng.uniform(-1, 1))
        plane = (rng.uniform(-1, 1), rng.uniform(-1, 1), rng.uniform(-1, 1))
        R = 10 ** rng.uniform(-1.5, 1.5)
        od = R * rng.uniform(0.1, 0.9)
        wall = od * rng.uniform(0.02, 0.2)
        e = 10 ** rng.uniform(10, 11.5)
        g = e / rng.uniform(2.0, 3.0)
        k = 10 ** rng.uniform(0, 1.3)
        comp = rng.randrange(3)
        wv = [0.0, 0.0, 0.0]
        wv[comp] = rng.choice([-1.0, 1.0]) * 10 ** rng.uniform(0, 4)
        add("random %d (phi=%.4f rad, R=%.3g)" % (idx, phi, R), phi_rad=phi, origin=origin,
            chord=chord, plane=plane, R=R, od=od, wall=wall, mat=(e, g), kin=k, kout=k, w=wv)
    if quick:
        cases = cases[:3]
    return cases


def build_extra_cases():
    steel = (2.03e11, 2.03e11 / 2.6)
    cases = []

    def add(label, phi_rad, kin=1.0, kout=1.0, R=0.2286):
        xi, xj, yref = arc_nodes((3.0, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, -1.0, 0.0), R, phi_rad)
        area, inertia, torsion = section(0.1683, 0.00711)
        cases.append(dict(label=label, args=(xi, xj, R, yref, steel[0], steel[1], area, inertia,
                                              torsion, kin, kout, [0.0, 0.0, -450.0])))
    for k in (1e11, 1e25, 1e30, 1e33, 1e35, 1e36, 1e37, 1e38, 1e40):
        add("k=%g 90 deg" % k, math.pi / 2, kin=k, kout=k)
    for eps in (3e-9, 2e-9, 1.5e-9, 1.1e-9):
        add("angle pi-%g rad" % eps, math.pi - eps)
    for rad in (1e-7, 1e-8, 1.01e-9):
        add("angle %g rad" % rad, rad)
    return cases


def inversion_error(Fref):
    """Relative error of a binary64 Gaussian-elimination inverse of F (F
    rounded to binary64), against the exact inverse of the Decimal F:
    max over entries |K64 - K| / max |K|, and the same on the 3x3 out-of-plane
    block's largest entry (the torsion-type direction)."""
    F64 = [[float(Fref[r][c]) for c in range(6)] for r in range(6)]
    K = []
    for col in range(6):
        e = [D(0)] * 6
        e[col] = D(1)
        K.append(solve(Fref, e))
    K = [[K[c][r] for c in range(6)] for r in range(6)]
    K64 = [[0.0] * 6 for _ in range(6)]
    for col in range(6):
        e = [0.0] * 6
        e[col] = 1.0
        try:
            x = _solve64(F64, e)
        except ZeroDivisionError:
            return math.inf, 0.0
        for r in range(6):
            K64[r][col] = x[r]
    big = max(abs(K[r][c]) for r in range(6) for c in range(6))
    err = max(abs(D(K64[r][c]) - K[r][c]) for r in range(6) for c in range(6))
    return float(err / big), float(big)


def _solve64(Fm, rhs):
    n = 6
    a = [Fm[r][:] + [rhs[r]] for r in range(n)]
    for col in range(n):
        piv = max(range(col, n), key=lambda r: abs(a[r][col]))
        a[col], a[piv] = a[piv], a[col]
        for r in range(col + 1, n):
            f = a[r][col] / a[col][col]
            for k in range(col, n + 1):
                a[r][k] -= f * a[col][k]
    x = [0.0] * n
    for r in reversed(range(n)):
        s = a[r][n]
        for k in range(r + 1, n):
            s -= a[r][k] * x[k]
        x[r] = s / a[r][r]
    return x


def load_probe(path):
    spec = importlib.util.spec_from_file_location("t4i9_probe", path)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def fmt(x):
    return "%.2e" % x


def main():
    probe = load_probe(sys.argv[1])
    quick = "--quick" in sys.argv
    getcontext().prec = PREC
    print("RV129 independent enclosure test of T4-I9's ball certificate (python %s)"
          % sys.version.split()[0])
    print("reference: Decimal %d digits, Gauss-Legendre %d x %d nodes, global unit loads, "
          "own trig; cross-check at %d digits / %d nodes" % (PREC, NODES, NODES_INNER, PREC + 20, NODES + 8))
    print("columns: certificate | enclosed | max |ref-m|/r | rad=max r/max|f| | "
          "refcheck=max|ref-ref2|/max r | probe-ref agreement | binary64 objective defect")
    total = enclosed_count = refused = 0
    worst_ratio = 0.0
    selected = build_extra_cases() if "--extra" in sys.argv else build_cases(quick)
    for case in selected:
        t0 = time.time()
        args = case["args"]
        label = case["label"]
        ref, phi, Fref = reference(*args)
        ref2, _, _ = reference(*args, prec=PREC + 20, nodes=NODES + 8, inner=NODES_INNER + 8)
        scale = max(abs(v) for v in ref)
        refdiff = max(abs(ref[k] - ref2[k]) for k in range(12))
        try:
            ball, _ = probe.formula(probe.BallCtx(), *args)
            rho = probe.invert.last_rho
        except ArithmeticError as error:
            refused += 1
            # Even when refused, report how wrong the binary64 element is.
            v = probe.objective_binary64(args[0], args[1], args[2], args[3], tuple(args[4:11]),
                                         args[11], libm_trig=False)
            defect = max(abs(Q(v[k]) - Q(ref[k])) for k in range(12)) / Q(scale)
            inv_err, kmax = inversion_error(Fref)
            print("%s | REFUSED (%s) | binary64 objective defect vs reference = %s | "
                  "binary64 inverse of F: max entry error / max|K| = %s (max|K| = %s) | phi=%.10g"
                  % (label, error, fmt(float(defect)), fmt(inv_err), fmt(kmax), float(phi)))
            continue
        total += 1
        ratios = []
        ok = True
        for k in range(12):
            diff = abs(Q(ref[k]) - ball[k].m)
            r = Q(ball[k].r)
            if diff > r:
                ok = False
            ratios.append(float(diff / r) if r else (0.0 if diff == 0 else math.inf))
        enclosed_count += ok
        worst = max(ratios)
        worst_ratio = max(worst_ratio, worst)
        rad_rel = max(b.r for b in ball) / float(scale)
        refcheck = float(refdiff / D(max(b.r for b in ball)))
        dec, _ = probe.formula(probe.DecCtx(), *args)
        getcontext().prec = PREC
        agree = max(abs(D(dec[k]) - ref[k]) for k in range(12)) / scale
        v = probe.objective_binary64(args[0], args[1], args[2], args[3], tuple(args[4:11]),
                                     args[11], libm_trig=False)
        defect = max(abs(Q(v[k]) - Q(ref[k])) for k in range(12)) / Q(scale)
        if "--extra" in sys.argv:
            inv_err, kmax = inversion_error(Fref)
            label = label + " [b64 inverse err %s]" % fmt(inv_err)
        print("%s | certified rho=%s | enclosed=%s | max|ref-m|/r=%s | rad=%s | refcheck=%s | "
              "probe-ref=%s | b64 defect=%s | phi=%.10g | %.1fs"
              % (label, fmt(rho), ok, fmt(worst), fmt(rad_rel), fmt(refcheck), fmt(float(agree)),
                 fmt(float(defect)), float(phi), time.time() - t0))
        sys.stdout.flush()
    print()
    print("certified cases: %d; enclosed: %d; refused: %d; worst |ref-m|/r over certified: %s"
          % (total, enclosed_count, refused, fmt(worst_ratio)))


if __name__ == "__main__":
    main()
