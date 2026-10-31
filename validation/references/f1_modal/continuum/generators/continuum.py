"""F1 class (b) continuum references: beams, rods, controls, tip mass, rings.

Every formula is re-derived (README, "Derivations"); every transcendental
root is found by bisection in decimal at WORK_PREC digits on a sign change
first located by a binary64 scan and then re-confirmed in decimal. Beam roots
are found twice: from the textbook frequency equation and from an independent
4x4 boundary-condition determinant on the general solution.
"""

import math
from decimal import Decimal, localcontext
from fractions import Fraction as Fr

from numerics import (SIG, bisect, count_exact, dcos, dcosh, dec, dsin, dsinh, fmt, inp,
                      pi, rat, scan_brackets, val)

N_BEAM = 6  # beam modes published per case
N_ROD = 4

# --------------------------------------------------------------------------
# Invented sections (binary64 inputs). Stiffness and mass use the same wall:
# no mill tolerance or corrosion allowance (c_m = 0). Insulation has mass and
# polar inertia but no stiffness. Contents are in m-bar only (D-e).

SECTIONS = {
    "SEC-A": {"od": 0.2, "wall": 0.008, "rho_pipe": 7850.0, "rho_contents": 1000.0,
              "insulation_thickness": 0.0, "rho_insulation": 0.0,
              "E": 2.0e11, "G": 8.0e10},
    "SEC-B": {"od": 0.1143, "wall": 0.00602, "rho_pipe": 7900.0, "rho_contents": 850.0,
              "insulation_thickness": 0.05, "rho_insulation": 120.0,
              "E": 1.95e11, "G": 7.5e10},
}

# (section, span) realizations of the nondimensional beam/rod results
SPANS = [("SEC-A", 6.0), ("SEC-B", 4.5), ("SEC-A", 1.0)]


def section_props(name):
    s = SECTIONS[name]
    od, t = Fr(s["od"]), Fr(s["wall"])
    rp, rc = Fr(s["rho_pipe"]), Fr(s["rho_contents"])
    ti, ri = Fr(s["insulation_thickness"]), Fr(s["rho_insulation"])
    E, G = Fr(s["E"]), Fr(s["G"])
    di = od - 2 * t
    oi = od + 2 * ti
    # quantities divided by pi: exact rationals
    a_m = (od**2 - di**2) / 4
    a_c = di**2 / 4
    a_i = (oi**2 - od**2) / 4
    i_m = (od**4 - di**4) / 64
    j_m = 2 * i_m
    j_i = (oi**4 - od**4) / 32
    mbar = rp * a_m + rc * a_c + ri * a_i
    jbar = rp * j_m + ri * j_i
    ibar = jbar / 2
    p = {
        "inner_diameter": di, "metal_area/pi": a_m, "bore_area/pi": a_c,
        "insulation_area/pi": a_i, "I/pi": i_m, "J/pi": j_m, "J_insulation/pi": j_i,
        "mbar/pi": mbar, "jbar/pi": jbar, "ibar/pi": ibar,
        "EI/pi": E * i_m, "EA/pi": E * a_m, "GJ/pi": G * j_m,
        "EI/mbar": E * i_m / mbar, "EA/mbar": E * a_m / mbar, "GJ/jbar": G * j_m / jbar,
        "ibar/mbar": ibar / mbar, "A/I": a_m / i_m, "GJ/EI": G * j_m / (E * i_m),
        "jbar/mbar": jbar / mbar,
    }
    return p


def section_doc(name):
    s = SECTIONS[name]
    p = section_props(name)
    P = pi()
    exact_keys = ["inner_diameter", "EI/mbar", "EA/mbar", "GJ/jbar", "ibar/mbar", "GJ/EI"]
    pi_keys = {"metal_area": "metal_area/pi", "bore_area": "bore_area/pi",
               "insulation_area": "insulation_area/pi", "I": "I/pi", "J": "J/pi",
               "J_insulation": "J_insulation/pi", "mbar": "mbar/pi", "jbar": "jbar/pi",
               "ibar": "ibar/pi", "EI": "EI/pi", "EA": "EA/pi", "GJ": "GJ/pi"}
    return {
        "inputs": {k: inp(v) for k, v in s.items()},
        "derived_pi_free_exact": {k: rat(p[k]) for k in exact_keys},
        "derived": {k: val(P * dec(p[v])) for k, v in pi_keys.items()},
        "units": "SI: m, kg/m^3, Pa; mbar kg/m; jbar and ibar kg m; EI N m^2; EA N; GJ N m^2",
    }


# --------------------------------------------------------------------------
# Euler-Bernoulli frequency equations (design 4.2), divided by cosh(x) > 0


def eq_cant(x, lib):
    return lib.cos(x) + 1 / lib.cosh(x)


def eq_ff(x, lib):
    return lib.cos(x) - 1 / lib.cosh(x)


def eq_tip(x, mu, lib):
    ch = lib.cosh(x)
    return 1 / ch + lib.cos(x) + mu * x * (lib.cos(x) * lib.sinh(x) / ch - lib.sin(x))


class FloatLib:
    cos = staticmethod(math.cos)
    sin = staticmethod(math.sin)
    cosh = staticmethod(math.cosh)
    sinh = staticmethod(math.sinh)
    sqrt = staticmethod(math.sqrt)
    conv = staticmethod(float)


class DecLib:
    cos = staticmethod(dcos)
    sin = staticmethod(dsin)
    cosh = staticmethod(dcosh)
    sinh = staticmethod(dsinh)
    sqrt = staticmethod(lambda x: x.sqrt())
    conv = staticmethod(dec)


def roots_of(eq_float, eq_dec, start, count, step=0.01, stop=60.0):
    out = []
    for lo, hi in scan_brackets(eq_float, start, stop, step, count):
        out.append(bisect(eq_dec, dec(lo), dec(hi)))
    return out


_CACHE = {}


def eb_roots(bc, mu=None):
    """beta_n L for bc in cantilever, simply_supported, fixed_fixed, tip(mu)."""
    key = (bc, mu)
    if key in _CACHE:
        return _CACHE[key]
    if bc == "cantilever":
        r = roots_of(lambda x: eq_cant(x, FloatLib), lambda x: eq_cant(x, DecLib), 0.2, N_BEAM)
    elif bc == "fixed_fixed":
        r = roots_of(lambda x: eq_ff(x, FloatLib), lambda x: eq_ff(x, DecLib), 0.5, N_BEAM)
    elif bc == "simply_supported":
        r = [n * pi() for n in range(1, N_BEAM + 1)]
    elif bc == "tip":
        mf, md = float(mu), dec(mu)
        r = roots_of(lambda x: eq_tip(x, mf, FloatLib), lambda x: eq_tip(x, md, DecLib),
                     0.2, N_BEAM)
    _CACHE[key] = r
    return r


# --------------------------------------------------------------------------
# Independent route: 4x4 determinant of the boundary conditions on the
# general solution of EI w'''' - ibar w''_tt + mbar w_tt = 0 (Rayleigh beam;
# r2 = 0 is Euler-Bernoulli). Nondimensional: L = 1, Om = omega sqrt(mbar L^4/EI),
# r2 = ibar/(mbar L^2), mu = M_tip/(mbar L).


def _det(A):
    A = [row[:] for row in A]
    n = len(A)
    det = A[0][0] * 0 + 1
    for k in range(n):
        p = max(range(k, n), key=lambda i: abs(A[i][k]))
        if A[p][k] == 0:
            return A[0][0] * 0
        if p != k:
            A[k], A[p] = A[p], A[k]
            det = -det
        det = det * A[k][k]
        for i in range(k + 1, n):
            l = A[i][k] / A[k][k]
            for j in range(k, n):
                A[i][j] -= l * A[k][j]
    return det


def beam_det(Om, left, right, r2, mu, lib):
    r2 = lib.conv(r2)
    mu = lib.conv(mu)
    disc = lib.sqrt(r2 * r2 * Om**4 + 4 * Om * Om)
    # W = e^{sx}: s^4 + r2 Om^2 s^2 - Om^2 = 0, so s^2 = a^2 > 0 or -b^2
    a = lib.sqrt((-r2 * Om * Om + disc) / 2)
    b = lib.sqrt((r2 * Om * Om + disc) / 2)

    def basis(x_is_one):
        # rows: derivative order 0..3; columns: cosh, sinh, cos, sin (x=1 scaled by 1/cosh a)
        if x_is_one:
            ch, sh = lib.cosh(a), lib.sinh(a)
            c, s = lib.cos(b) / ch, lib.sin(b) / ch
            sh, ch = sh / ch, 1
        else:
            ch, sh, c, s = 1, 0, 1, 0
        return [
            [ch, sh, c, s],
            [a * sh, a * ch, -b * s, b * c],
            [a * a * ch, a * a * sh, -b * b * c, -b * b * s],
            [a**3 * sh, a**3 * ch, b**3 * s, -b**3 * c],
        ]

    def rows(kind, x_is_one):
        d = basis(x_is_one)
        if kind == "fixed":
            return [d[0], d[1]]
        if kind == "pinned":
            return [d[0], d[2]]
        if kind == "free":  # w'' = 0;  w''' + r2 Om^2 w' + mu Om^2 w = 0  (x = 1 only)
            assert x_is_one
            return [d[2], [d[3][j] + r2 * Om * Om * d[1][j] + mu * Om * Om * d[0][j]
                           for j in range(4)]]
        raise ValueError(kind)

    return _det(rows(left, False) + rows(right, True))


BC_ENDS = {"cantilever": ("fixed", "free"), "simply_supported": ("pinned", "pinned"),
           "fixed_fixed": ("fixed", "fixed"), "tip": ("fixed", "free")}


def det_roots(bc, r2=Fr(0), mu=Fr(0), count=N_BEAM):
    """Om_n from the determinant route (scan in q = sqrt(Om))."""
    key = ("det", bc, r2, mu, count)
    if key in _CACHE:
        return _CACHE[key]
    left, right = BC_ENDS[bc]
    f_scan = lambda q: beam_det(q * q, left, right, r2, mu, FloatLib)
    out = []
    for lo, hi in scan_brackets(f_scan, 0.2, 60.0, 0.01, count):
        f_dec = lambda om: beam_det(om, left, right, r2, mu, DecLib)
        out.append(bisect(f_dec, dec(lo) ** 2, dec(hi) ** 2))
    _CACHE[key] = out
    return out


def rayleigh_ss_om(n, r2):
    """Om_n for the simply supported Rayleigh beam: Om^2 = a^4/(1 + r2 a^2), a = n pi."""
    a = n * pi()
    return (a**4 / (1 + dec(r2) * a * a)).sqrt()


# --------------------------------------------------------------------------
# Assembly of the document


def freq_rows(oms, scale_sq):
    """omega_n = Om_n sqrt(scale_sq), f = omega/2pi."""
    sc = dec(scale_sq).sqrt()
    tp = 2 * pi()
    return [{"mode": n, "omega_rad_s": val(om * sc), "f_hz": val(om * sc / tp)}
            for n, om in enumerate(oms, start=1)]


def build(cross):
    P = pi()
    doc = {}
    doc["sections"] = {k: section_doc(k) for k in SECTIONS}

    # ---- 1. Euler-Bernoulli beams
    eb = {"equation": "omega_n = (beta_n L)^2 sqrt(EI/(mbar L^4))", "roots": {}, "cases": []}
    eqs = {"cantilever": "cos x cosh x = -1", "simply_supported": "sin x = 0 (x = n pi)",
           "fixed_fixed": "cos x cosh x = 1 (x > 0)"}
    for bc in ("cantilever", "simply_supported", "fixed_fixed"):
        r = eb_roots(bc)
        eb["roots"][bc] = {"frequency_equation": eqs[bc],
                           "beta_L": [fmt(x) for x in r],
                           "Omega=(beta_L)^2": [fmt(x * x) for x in r]}
        # residual of the frequency equation at higher precision
        with localcontext() as c:
            c.prec += 30
            f = {"cantilever": lambda x: 1 + dcos(x) * dcosh(x),
                 "fixed_fixed": lambda x: 1 - dcos(x) * dcosh(x),
                 "simply_supported": lambda x: dsin(x)}[bc]
            res = max(abs(f(dec(fmt(x, 60))) / max(Decimal(1), dcosh(dec(fmt(x, 60)))))
                      for x in r)
        cross.append({"id": f"EB-{bc}-residual", "what": "|frequency equation|/cosh at the 60-digit root",
                      "max": fmt(res, 3), "agrees": res < Decimal("1e-50")})
        d = det_roots(bc)
        diff = max(abs(x * x - y) / y for x, y in zip(r, d))
        cross.append({"id": f"EB-{bc}-determinant", "what": "frequency-equation roots vs 4x4 boundary determinant",
                      "max_rel_diff": fmt(diff, 3), "agrees": diff < Decimal("1e-45")})
    design = {"cantilever": ["1.875104069", "4.694091133", "7.854757438", "10.995540735"],
              "fixed_fixed": ["4.730040745", "7.853204624", "10.995607838"]}
    for bc, vals in design.items():
        r = eb_roots(bc)
        diffs = [abs(dec(v) - x) for v, x in zip(vals, r)]
        cross.append({"id": f"EB-{bc}-design-4.2", "what": "design 4.2 roots (stated from memory) vs derived",
                      "max_abs_diff": fmt(max(diffs), 3),
                      "agrees": all(dd <= Decimal("6e-10") for dd in diffs)})
    for sec, L in SPANS:
        p = section_props(sec)
        scale = p["EI/mbar"] / Fr(L) ** 4
        for bc in ("cantilever", "simply_supported", "fixed_fixed"):
            r = eb_roots(bc)
            eb["cases"].append({"id": f"EB-{bc.upper()}-{sec}-L{L:g}", "section": sec,
                                "span_m": inp(L), "boundary": bc,
                                "EI/(mbar L^4)": rat(scale),
                                "modes": freq_rows([x * x for x in r], scale)})
    doc["euler_bernoulli"] = eb

    # ---- 2. Axial and torsional rods
    rods = {"equations": {
        "cantilever_axial": "omega_n = ((2n-1) pi/2) sqrt(EA/(mbar L^2))",
        "cantilever_torsion": "omega_n = ((2n-1) pi/2) sqrt(GJ/(jbar L^2))",
        "fixed_fixed_axial": "omega_n = n pi sqrt(EA/(mbar L^2))",
        "fixed_fixed_torsion": "omega_n = n pi sqrt(GJ/(jbar L^2))"},
        "note": "mbar includes contents and insulation; jbar excludes contents (D-e)",
        "coefficients": {"cantilever": [fmt((2 * n - 1) * P / 2) for n in range(1, N_ROD + 1)],
                         "fixed_fixed": [fmt(n * P) for n in range(1, N_ROD + 1)]},
        "cases": []}
    for sec, L in SPANS[:2]:
        p = section_props(sec)
        for bc in ("cantilever", "fixed_fixed"):
            coef = [(2 * n - 1) * P / 2 if bc == "cantilever" else n * P for n in range(1, N_ROD + 1)]
            for kind, ratio in (("axial", p["EA/mbar"]), ("torsion", p["GJ/jbar"])):
                scale = ratio / Fr(L) ** 2
                rods["cases"].append({"id": f"ROD-{kind.upper()}-{bc.upper()}-{sec}-L{L:g}",
                                      "section": sec, "span_m": inp(L), "boundary": bc,
                                      "kind": kind, "stiffness/(inertia L^2)": rat(scale),
                                      "modes": freq_rows(coef, scale)})
    doc["rods"] = rods

    # ---- 3. Rayleigh beam (rotary inertia ibar), simply supported, plus
    # cantilever and fixed-fixed by the determinant route (supplementary)
    ray = {"equation_ss": "omega^2 = EI a^4/(mbar + ibar a^2), a = n pi/L; "
                          "nondimensional Om^2 = (n pi)^4/(1 + r2 (n pi)^2), r2 = ibar/(mbar L^2)",
           "cases": []}
    for sec, L in SPANS:
        p = section_props(sec)
        r2 = p["ibar/mbar"] / Fr(L) ** 2
        scale = p["EI/mbar"] / Fr(L) ** 4
        oms = [rayleigh_ss_om(n, r2) for n in range(1, N_BEAM + 1)]
        d = det_roots("simply_supported", r2=r2)
        diff = max(abs(x - y) / x for x, y in zip(oms, d))
        cross.append({"id": f"RAY-SS-{sec}-L{L:g}-determinant",
                      "what": "Rayleigh SS closed form vs 4x4 boundary determinant",
                      "max_rel_diff": fmt(diff, 3), "agrees": diff < Decimal("1e-45")})
        eb_oms = [x * x for x in eb_roots("simply_supported")]
        ray["cases"].append({"id": f"RAY-SIMPLY_SUPPORTED-{sec}-L{L:g}", "section": sec,
                             "span_m": inp(L), "boundary": "simply_supported",
                             "route": "closed form", "r2=ibar/(mbar L^2)": rat(r2),
                             "EI/(mbar L^4)": rat(scale),
                             "Omega": [fmt(x) for x in oms],
                             "rel_shift_from_euler_bernoulli": [fmt(x / y - 1, 6) for x, y in zip(oms, eb_oms)],
                             "modes": freq_rows(oms, scale)})
        for bc in ("cantilever", "fixed_fixed"):
            oms = det_roots(bc, r2=r2)
            eb_oms = [x * x for x in eb_roots(bc)]
            ray["cases"].append({"id": f"RAY-{bc.upper()}-{sec}-L{L:g}", "section": sec,
                                 "span_m": inp(L), "boundary": bc,
                                 "route": "4x4 boundary determinant (supplementary)",
                                 "r2=ibar/(mbar L^2)": rat(r2), "EI/(mbar L^4)": rat(scale),
                                 "Omega": [fmt(x) for x in oms],
                                 "rel_shift_from_euler_bernoulli": [fmt(x / y - 1, 6) for x, y in zip(oms, eb_oms)],
                                 "modes": freq_rows(oms, scale)})
    doc["rayleigh"] = ray

    # ---- 4. Lumped controls
    doc["lumped_controls"] = lumped_controls(cross)

    # ---- 5. Cantilever with distributed mass and tip mass
    doc["tip_mass"] = tip_mass(cross)

    # ---- 6. Free rings
    doc["rings"] = rings(cross)
    return doc


def lumped_controls(cross):
    P = pi()
    out = {}
    k, m = 3.0e6, 47.5
    w2 = Fr(k) / Fr(m)
    out["sdof"] = {"id": "LC-SDOF", "k_N_per_m": inp(k), "m_kg": inp(m),
                   "omega^2": rat(w2), "omega_rad_s": val(dec(w2).sqrt()),
                   "f_hz": val(dec(w2).sqrt() / (2 * P))}

    EI, L, Mt = 4.0e6, 2.5, 150.0
    EIf, Lf, Mf = Fr(EI), Fr(L), Fr(Mt)
    w2 = 3 * EIf / (Mf * Lf**3)
    Kt = [[12 * EIf / Lf**3, -6 * EIf / Lf**2], [-6 * EIf / Lf**2, 4 * EIf / Lf]]
    Mtm = [[Mf, Fr(0)], [Fr(0), Fr(0)]]
    counts = []
    for tag, s in (("below", w2 * (1 - Fr(1, 2**40))), ("at", w2), ("above", w2 * (1 + Fr(1, 2**40)))):
        c, z = count_exact(Kt, Mtm, s)
        counts.append({"shift": tag, "shift_exact": str(s), "count_below": c, "count_equal": z})
    ok = [(c["count_below"], c["count_equal"]) for c in counts] == [(0, 0), (0, 1), (1, 0)]
    cross.append({"id": "LC-TIP-kernel-pencil", "what": "exact inertia of the 1-element tip pencil at omega^2 and +-2^-40",
                  "agrees": ok})
    out["point_mass_on_massless_cantilever"] = {
        "id": "LC-TIPMASS", "EI_N_m2": inp(EI), "L_m": inp(L), "M_tip_kg": inp(Mt),
        "equation": "omega^2 = 3 EI/(M_t L^3)", "omega^2": rat(w2),
        "omega_rad_s": val(dec(w2).sqrt()), "f_hz": val(dec(w2).sqrt() / (2 * P)),
        "kernel_pencil": {"dofs": ["w_tip", "theta_tip"], "note": "one Hermitian element, root fixed; M singular (one infinite eigenvalue)",
                          "K": [[str(v) for v in row] for row in Kt],
                          "M": [[str(v) for v in row] for row in Mtm],
                          "exact_counts": counts}}

    chains = []
    k, m = 1.0e7, 25.0
    for N in (3, 5, 10):
        kf, mf = Fr(k), Fr(m)
        K = [[Fr(0)] * N for _ in range(N)]
        for i in range(N):
            K[i][i] = 2 * kf if i < N - 1 else kf
            if i + 1 < N:
                K[i][i + 1] = K[i + 1][i] = -kf
        M = [[mf if i == j else Fr(0) for j in range(N)] for i in range(N)]
        w = [2 * dec(kf / mf).sqrt() * dsin((2 * j - 1) * P / (2 * (2 * N + 1))) for j in range(1, N + 1)]
        ok = True
        for j, wj in enumerate(w, start=1):
            lo = Fr(Decimal(fmt(wj * wj * (1 - Decimal("1e-30")), 45)))
            hi = Fr(Decimal(fmt(wj * wj * (1 + Decimal("1e-30")), 45)))
            ok &= count_exact(K, M, lo) == (j - 1, 0) and count_exact(K, M, hi) == (j, 0)
        cross.append({"id": f"LC-CHAIN-{N}-sturm", "what": "closed form vs exact Sturm counts at omega_j^2 (1 +- 1e-30)",
                      "agrees": ok})
        chains.append({"id": f"LC-CHAIN-{N}", "N": N, "k_N_per_m": inp(k), "m_kg": inp(m),
                       "layout": "ground -k- m1 -k- m2 ... -k- mN (free): N springs, N masses",
                       "equation": "omega_j = 2 sqrt(k/m) sin((2j-1) pi/(2(2N+1)))",
                       "omega_rad_s": [val(x) for x in w]})
    out["axial_chains"] = chains
    return out


TIP_MUS = [Fr(0), Fr(1, 10), Fr(1, 2), Fr(1), Fr(2), Fr(10)]


def tip_mass(cross):
    out = {"equation": "1 + cos x cosh x + mu x (cos x sinh x - sin x cosh x) = 0, x = beta L, mu = M_t/(mbar L)",
           "premise": "tip mass is a point mass at the free end: no rotary inertia, no offset",
           "nondimensional": [], "dimensional": []}
    cant = eb_roots("cantilever")
    for mu in TIP_MUS:
        r = eb_roots("tip", mu)
        d = det_roots("tip", mu=mu)
        diff = max(abs(x * x - y) / y for x, y in zip(r, d))
        cross.append({"id": f"TIP-mu{mu}-determinant", "what": "tip-mass frequency equation vs 4x4 boundary determinant",
                      "max_rel_diff": fmt(diff, 3), "agrees": diff < Decimal("1e-45")})
        # Rayleigh-quotient estimate of mode 1 with the static tip shape: x^4 ~ 3/(mu + 33/140)
        approx = (3 / (dec(mu) + Decimal(33) / 140)).sqrt()
        rel = approx / (r[0] ** 2) - 1
        cross.append({"id": f"TIP-mu{mu}-rayleigh-estimate", "what": "mode-1 Om vs sqrt(3/(mu + 33/140)) (approximate, from above)",
                      "rel_diff": fmt(rel, 3), "agrees": Decimal(0) <= rel < Decimal("0.02")})
        if mu == 0:
            same = max(abs(x - y) for x, y in zip(r, cant))
            cross.append({"id": "TIP-mu0-is-cantilever", "what": "mu = 0 roots vs cantilever roots",
                          "max_abs_diff": fmt(same, 3), "agrees": same < Decimal("1e-50")})
        out["nondimensional"].append({"id": f"TIP-mu{rat_tag(mu)}", "mu": rat(mu),
                                      "beta_L": [fmt(x) for x in r],
                                      "Omega=(beta_L)^2": [fmt(x * x) for x in r]})
    sec, L, W, a = "SEC-A", 6.0, 2000.0, 9.80665
    p = section_props(sec)
    Mt = Fr(W) / Fr(a)
    mu = dec(Mt) / (pi() * dec(p["mbar/pi"]) * dec(Fr(L)))
    mf = float(mu)
    r = roots_of(lambda x: eq_tip(x, mf, FloatLib), lambda x: eq_tip(x, mu, DecLib), 0.2, N_BEAM)
    scale = p["EI/mbar"] / Fr(L) ** 4
    out["dimensional"].append({"id": f"TIP-{sec}-L{L:g}-W{W:g}", "section": sec, "span_m": inp(L),
                               "tip_weight_N": inp(W), "weight_to_mass_acceleration_m_s2": inp(a),
                               "M_tip=W/a_kg": rat(Mt), "mu": val(mu),
                               "beta_L": [fmt(x) for x in r], "EI/(mbar L^4)": rat(scale),
                               "modes": freq_rows([x * x for x in r], scale)})
    return out


def rat_tag(f):
    return str(f.numerator) if f.denominator == 1 else f"{f.numerator}_{f.denominator}"


# --------------------------------------------------------------------------
# Rings. lam = omega^2 mbar R^4/EI. alpha = A R^2/I, gamma = GJ/EI,
# rho = ibar/(mbar R^2), rho_t = jbar/(mbar R^2).


def quad_roots(K, M):
    """Both roots of det(K - lam M) = 0 for 2x2 (M may be singular)."""
    a = M[0][0] * M[1][1] - M[0][1] ** 2
    b = K[0][0] * M[1][1] + K[1][1] * M[0][0] - 2 * K[0][1] * M[0][1]
    c = K[0][0] * K[1][1] - K[0][1] ** 2
    if a == 0:
        return [c / b, None]
    disc = (dec(b) ** 2 - 4 * dec(a) * dec(c)).sqrt()
    lo = 2 * dec(c) / (dec(b) + disc)
    hi = (dec(b) + disc) / (2 * dec(a))
    return [lo, hi]


def ring_inplane_ext(n, al, rho):
    K = [[al + n**4, al * n + n**3], [al * n + n**3, al * n * n + n * n]]
    M = [[1 + rho * n * n, rho * n], [rho * n, 1 + rho]]
    return K, M


def ring_outplane(n, ga, rho, rho_t):
    K = [[n**4 + ga * n * n, n * n + ga * n * n], [n * n + ga * n * n, 1 + ga * n * n]]
    M = [[1 + rho * n * n, Fr(0)], [Fr(0), rho_t]]
    return K, M


RINGS = [("SEC-A", 2.0), ("SEC-B", 1.0)]
RING_N = range(2, 7)
MERGE_N = 8


def rings(cross):
    out = {"premises": "thin circular ring of the section, Euler-Bernoulli curved bar: no shear deformation, "
                       "no warping, no ovalization (flexibility factor 1), free (no supports); "
                       "each n >= 1 is a double eigenvalue (cos and sin forms)",
           "lambda": "lambda = omega^2 mbar R^4/EI",
           "variants": {
               "inplane_hoppe": "n^2 (n^2-1)^2/(n^2+1): inextensional, no rotary inertia (Hoppe 1871)",
               "inplane_inextensional_rotary": "n^2 (n^2-1)^2/(n^2+1+rho (n^2-1)^2), rho = ibar/(mbar R^2)",
               "inplane_extensible_rotary": "roots of 2x2: K=[[al+n^4, al n+n^3],[al n+n^3, al n^2+n^2]], "
                                            "M=[[1+rho n^2, rho n],[rho n, 1+rho]], al = A R^2/I; low = flexural, high = extensional",
               "outofplane_love": "n^2 (n^2-1)^2/(n^2 + EI/GJ): torsional and rotary inertia neglected (Love)",
               "outofplane_torsion_rotary": "roots of 2x2: K=[[n^4+ga n^2, n^2+ga n^2],[n^2+ga n^2, 1+ga n^2]], "
                                            "M=[[1+rho n^2, 0],[0, rho_t]], ga = GJ/EI, rho_t = jbar/(mbar R^2); low = flexural, high = torsional"},
           "cases": []}
    for sec, R in RINGS:
        p = section_props(sec)
        Rf = Fr(R)
        al = p["A/I"] * Rf**2
        ga = p["GJ/EI"]
        rho = p["ibar/mbar"] / Rf**2
        rho_t = p["jbar/mbar"] / Rf**2
        scale = p["EI/mbar"] / Rf**4
        sc = dec(scale).sqrt()
        rows = []
        for n in RING_N:
            hoppe = Fr(n * n * (n * n - 1) ** 2, n * n + 1)
            inext = Fr(n * n * (n * n - 1) ** 2) / (n * n + 1 + rho * (n * n - 1) ** 2)
            ext = quad_roots(*ring_inplane_ext(n, al, rho))
            love = Fr(n * n * (n * n - 1) ** 2) / (n * n + 1 / ga)
            tor = quad_roots(*ring_outplane(n, ga, rho, rho_t))
            # exact identity: Love is the out-of-plane pencil with no rotational inertia
            Kc, Mc = ring_outplane(n, ga, Fr(0), Fr(0))
            cond = quad_roots(Kc, Mc)[0]
            cross.append({"id": f"RING-{sec}-n{n}-love-condensation",
                          "what": "Love's formula equals the out-of-plane pencil with rho = rho_t = 0 (exact rationals)",
                          "agrees": cond == love})
            big = Fr(10) ** 40
            e0 = quad_roots(*ring_inplane_ext(n, big, Fr(0)))[0]
            e1 = quad_roots(*ring_inplane_ext(n, big, rho))[0]
            d0 = abs(e0 / dec(hoppe) - 1)
            d1 = abs(e1 / dec(inext) - 1)
            cross.append({"id": f"RING-{sec}-n{n}-inextensional-limit",
                          "what": "extensible pencil at alpha = 1e40 vs Hoppe (rho = 0) and inextensional-rotary formula",
                          "max_rel_diff": fmt(max(d0, d1), 3), "agrees": max(d0, d1) < Decimal("1e-35")})

            def w(lmb):
                return {"lambda": fmt(dec(lmb)), "omega_rad_s": val(dec(lmb).sqrt() * sc),
                        "f_hz": val(dec(lmb).sqrt() * sc / (2 * pi()))}

            rows.append({"n": n, "multiplicity": 2,
                         "inplane_hoppe": w(hoppe), "inplane_inextensional_rotary": w(inext),
                         "inplane_extensible_rotary": {"flexural": w(ext[0]), "extensional": w(ext[1])},
                         "outofplane_love": w(love),
                         "outofplane_torsion_rotary": {"flexural": w(tor[0]), "torsional": w(tor[1])}})
        merged = ring_merged(al, ga, rho, rho_t, sc)
        out["cases"].append({"id": f"RING-{sec}-R{R:g}", "section": sec, "radius_m": inp(R),
                             "alpha=A R^2/I": rat(al), "gamma=GJ/EI": rat(ga),
                             "rho=ibar/(mbar R^2)": rat(rho), "rho_t=jbar/(mbar R^2)": rat(rho_t),
                             "EI/(mbar R^4)": rat(scale), "by_n": rows,
                             "complete_low_spectrum": merged})
    return out


def ring_merged(al, ga, rho, rho_t, sc):
    """Every mode of the refined model (extensible-rotary in plane, torsion-rotary
    out of plane) below the lowest n > MERGE_N branch, with multiplicities."""
    modes = [("inplane n=0 rigid rotation", Decimal(0), 1), ("outofplane n=0 rigid translation", Decimal(0), 1),
             ("inplane n=0 extensional (breathing)", dec(al), 1),
             ("outofplane n=0 torsional (uniform twist)", dec(1 / rho_t), 1)]
    for n in range(1, MERGE_N + 1):
        e = quad_roots(*ring_inplane_ext(n, al, rho))
        t = quad_roots(*ring_outplane(n, ga, rho, rho_t))
        labels = ("rigid translation" if n == 1 else "flexural", "extensional",
                  "rigid rotation" if n == 1 else "flexural", "torsional")
        modes += [(f"inplane n={n} {labels[0]}", dec(e[0]), 2), (f"inplane n={n} {labels[1]}", e[1], 2),
                  (f"outofplane n={n} {labels[2]}", dec(t[0]), 2), (f"outofplane n={n} {labels[3]}", t[1], 2)]
    for lab, lmb, mult in modes:
        if "rigid" in lab:
            assert abs(lmb) < Decimal("1e-60"), (lab, lmb)
    # cutoff: low branches of n > MERGE_N increase with n (checked to n = 400)
    lows = []
    for n in range(MERGE_N + 1, 401):
        lows.append(min(quad_roots(*ring_inplane_ext(n, al, rho))[0],
                        quad_roots(*ring_outplane(n, ga, rho, rho_t))[0]))
    assert all(x < y for x, y in zip(lows, lows[1:]))
    cutoff = lows[0]
    kept = sorted(((lab, max(lmb, Decimal(0)), mult) for lab, lmb, mult in modes if lmb < cutoff),
                  key=lambda t: t[1])
    rows = []
    below = 0
    for lab, lmb, mult in kept:
        rows.append({"mode": lab, "multiplicity": mult, "lambda": fmt(lmb) if lmb else "0",
                     "omega_rad_s": val(lmb.sqrt() * sc) if lmb else {"value": "0", "binary64": "0.0"},
                     "count_below": below})
        below += mult
    return {"cutoff_lambda": fmt(cutoff), "cutoff_note": "lowest branch of any n > 8; all branches of n > 8 lie at or above it",
            "total_count_below_cutoff": below, "modes": rows}
