"""Item 5: the uniform straight pipe as cantilever (C), propped cantilever (P),
pinned-pinned span (S) and fixed-fixed span (F), under transverse uniform
support motion, in one bending plane (DEL-04-10 REQ-002 to REQ-004; AC-002;
release target benchmark method, spectrum screening class).

Continuum closed forms (continuum.py), Euler-Bernoulli and Rayleigh, and the
generator's own finite elements (fe.py) at N = 2, 4, 8, 16 under the
consistent and the lumped mass formulation. Quantities per unit spectral
acceleration are formed once per mode and multiplied by s S(f_n).
"""

from decimal import Decimal, localcontext
from fractions import Fraction as Fr

import continuum as C
import fe
import rules
from numerics import dec, dsqrt, inp, num, pi, rel_diff, sci, solve, val

# --------------------------------------------------------------------------
# invented section and spans

SECTION = {"od": 0.0213, "wall": 0.00277, "rho_pipe": 7850.0, "rho_contents": 1000.0,
           "E": 2.03e11, "G": 7.8e10}
LENGTHS = [2.0, 64.0]
LEN_NOTE = {2.0: "rotary inertia changes the first three frequencies by MORE than 1e-6 relative (it matters)",
            64.0: "rotary inertia changes the first three frequencies by LESS than 1e-6 relative (negligible)"}
SPANS = {"C": ("clamped", "free", "cantilever"),
         "P": ("clamped", "pinned", "propped cantilever (fixed at x = 0, translation pinned at x = L)"),
         "S": ("pinned", "pinned", "pinned-pinned span"),
         "F": ("clamped", "clamped", "fixed-fixed span")}
SPAN_ORDER = ["C", "P", "S", "F"]
NS = [2, 4, 8, 16]
FORMS = ["consistent", "lumped"]
N_MODES = 5
TRUNCATIONS = [1, 2, 3, 5]
ZETA = 0.05
SCALE = 1.0

FLAT = {"id": "FLAT", "abscissa": "frequency", "ordinate": "acceleration", "abscissae": [0.0001, 1000000.0],
        "ordinates": [2.5, 2.5], "damping": ZETA, "rule": ("log", "log")}


def two_segment(L):
    fa = Fr(2) if L == 2.0 else Fr(1, 512)
    return {"id": f"TWO_SEGMENT_L{int(L)}", "abscissa": "frequency", "ordinate": "acceleration",
            "abscissae": [float(fa), float(16 * fa), float(4096 * fa)], "ordinates": [0.5, 4.0, 4.0],
            "damping": ZETA, "rule": ("log", "log")}


def section():
    s = SECTION
    od, t = Fr(s["od"]), Fr(s["wall"])
    rp, rc, E, G = Fr(s["rho_pipe"]), Fr(s["rho_contents"]), Fr(s["E"]), Fr(s["G"])
    di = od - 2 * t
    am, ac = (od ** 2 - di ** 2) / 4, di ** 2 / 4          # areas / pi
    im = (od ** 4 - di ** 4) / 64                           # I / pi
    mb = rp * am + rc * ac                                  # mbar / pi
    P = pi()
    return {
        "EI_m": E * im / mb, "ibar_m": rp * im / mb, "EA_m": E * am / mb, "GJ_jbar": G / rp,
        "mbar": P * dec(mb), "EI": P * dec(E * im), "ibar": P * dec(rp * im),
        "doc": {
            "inputs": {k: inp(v) for k, v in s.items()},
            "premises": "no insulation, no corrosion or mill allowance; I = pi(OD^4 - ID^4)/64 on the metal; "
                        "mbar = metal + contents; ibar = rho_pipe I (contents excluded from rotary inertia, "
                        "DEL-04-09 REQ-001); J = 2I, jbar = rho_pipe J",
            "exact_ratios": {"EI/mbar": num(E * im / mb), "ibar/mbar": num(rp * im / mb),
                             "EA/mbar": num(E * am / mb), "GJ/jbar": num(G / rp)},
            "mbar_kg_per_m": val(P * dec(mb)), "EI_N_m2": val(P * dec(E * im)),
            "ibar_kg_m": val(P * dec(rp * im)),
        },
    }


def quantity_list(span, N=None):
    q = [("w_tip_m" if span == "C" else "w_mid_m", "displacement", ("w", Fr(1) if span == "C" else Fr(1, 2))),
         ("F0_N", "reaction", ("F0",))]
    if SPANS[span][0] == "clamped":
        q.append(("M0_Nm", "reaction", ("M0",)))
    if SPANS[span][1] != "free":
        q.append(("FL_N", "reaction", ("FL",)))
    q.append(("M_mid_Nm", "moment", ("M", Fr(1, 2))))
    if N is None or N >= 4:
        q.append(("M_quarter_Nm", "moment", ("M", Fr(1, 4))))
        q.append(("V_quarter_N", "force", ("V", Fr(1, 4))))
    if span in ("C", "P"):
        q.append(("V_mid_N", "force", ("V", Fr(1, 2))))
    return q


# --------------------------------------------------------------------------
# continuum


def cont_mode_quantities(md, gamma, sec, L, span):
    """Signed per-unit-spectral-acceleration quantities of one continuum mode."""
    Ld = dec(L)
    mb, Om2, r2 = sec["mbar"], md.Om ** 2, md.r2
    EI_m = dec(sec["EI_m"])
    g = gamma
    out = {}
    for name, kind, spec in quantity_list(span):
        t = spec[0]
        if t == "w":
            v = g * C.W(md, spec[1]) * Ld ** 4 / (EI_m * Om2)
        elif t == "F0":
            v = g * mb * Ld * (C.W(md, 0, 3) / Om2 + r2 * C.W(md, 0, 1))
        elif t == "FL":
            v = -g * mb * Ld * (C.W(md, 1, 3) / Om2 + r2 * C.W(md, 1, 1))
        elif t == "M0":
            v = -g * mb * Ld ** 2 * C.W(md, 0, 2) / Om2
        elif t == "M":
            v = g * mb * Ld ** 2 * C.W(md, spec[1], 2) / Om2
        elif t == "V":
            v = -g * mb * Ld * (C.W(md, spec[1], 3) / Om2 + r2 * C.W(md, spec[1], 1))
        out[name] = v
    return out


def static_quantities(sec, L, span):
    c = C.static_uniform(SPANS[span][0], SPANS[span][1])
    Ld, mb, EI_m = dec(L), sec["mbar"], dec(sec["EI_m"])
    out, exact = {}, {}
    for name, kind, spec in quantity_list(span):
        t = spec[0]
        if t == "w":
            e = C.poly_eval(c, spec[1]); v = dec(e) * Ld ** 4 / EI_m
        elif t == "F0":
            e = C.poly_eval(c, 0, 3); v = mb * Ld * dec(e)
        elif t == "FL":
            e = -C.poly_eval(c, 1, 3); v = mb * Ld * dec(e)
        elif t == "M0":
            e = -C.poly_eval(c, 0, 2); v = mb * Ld ** 2 * dec(e)
        elif t == "M":
            e = C.poly_eval(c, spec[1], 2); v = mb * Ld ** 2 * dec(e)
        elif t == "V":
            e = -C.poly_eval(c, spec[1], 3); v = mb * Ld * dec(e)
        out[name] = v
        exact[name] = e
    return out, exact, c


# textbook coefficients of the static response to a uniform load q (independent cross-check)
TEXTBOOK = {
    "C": {"w_tip_m": Fr(1, 8), "F0_N": Fr(-1), "M0_Nm": Fr(-1, 2)},
    "P": {"w_mid_m": Fr(1, 192), "F0_N": Fr(-5, 8), "M0_Nm": Fr(-1, 8), "FL_N": Fr(-3, 8)},
    "S": {"w_mid_m": Fr(5, 384), "F0_N": Fr(-1, 2), "FL_N": Fr(-1, 2), "M_mid_Nm": Fr(-1, 8)},
    "F": {"w_mid_m": Fr(1, 384), "F0_N": Fr(-1, 2), "M0_Nm": Fr(-1, 12), "FL_N": Fr(-1, 2), "M_mid_Nm": Fr(-1, 24)},
}

EB_EQ = {"C": "cantilever", "P": "propped", "F": "fixed_fixed"}


class Cont:
    pass


def continuum_case(span, L, sec, model, cross_acc):
    left, right, _ = SPANS[span]
    r2 = sec["ibar_m"] / Fr(L) ** 2 if model == "rayleigh" else Fr(0)
    mods = C.modes(left, right, r2, 0, N_MODES, closed_pp=(span == "S"))
    Ld = dec(L)
    sqrt_EI_m = dsqrt(dec(sec["EI_m"]))
    cc = Cont()
    cc.modes, cc.r2 = mods, r2
    cc.omega = [m.Om * sqrt_EI_m / Ld ** 2 for m in mods]
    cc.freq = [w / (2 * pi()) for w in cc.omega]
    cc.gamma, cc.meff_frac, cc.unit = [], [], []
    for m in mods:
        anti = abs(m.I1) < Decimal(10) ** -60
        denom = m.I2 + m.r2 * m.I3
        g = Decimal(0) if anti else m.I1 / denom
        cc.gamma.append(g)
        cc.meff_frac.append(Decimal(0) if anti else m.I1 ** 2 / denom)
        q = cont_mode_quantities(m, g, sec, L, span)
        cc.unit.append(q)
        # cross-checks: quadrature and balance
        q1, q2, q3, j1 = C.quad_check(m)
        cross_acc["quad"] = max(cross_acc["quad"], rel_diff(q1, m.I1) if abs(m.I1) > Decimal("1e-40") else abs(q1),
                                rel_diff(q2, m.I2), rel_diff(q3, m.I3))
        F0 = g * sec["mbar"] * Ld * (C.W(m, 0, 3) / m.Om ** 2 + m.r2 * C.W(m, 0, 1))
        FL = -g * sec["mbar"] * Ld * (C.W(m, 1, 3) / m.Om ** 2 + m.r2 * C.W(m, 1, 1))
        M0 = -g * sec["mbar"] * Ld ** 2 * C.W(m, 0, 2) / m.Om ** 2
        ML = g * sec["mbar"] * Ld ** 2 * C.W(m, 1, 2) / m.Om ** 2
        tot = g * sec["mbar"] * Ld * m.I1
        if g != 0:
            cross_acc["force_balance"] = max(cross_acc["force_balance"], abs(F0 + FL + tot) / abs(tot))
            mom = M0 + ML + Ld * FL + g * sec["mbar"] * Ld ** 2 * (j1 + m.r2 * (C.W(m, 1) - C.W(m, 0)))
            scale = max(abs(M0), abs(Ld * FL), abs(g * sec["mbar"] * Ld ** 2 * j1))
            cross_acc["moment_balance"] = max(cross_acc["moment_balance"], abs(mom) / scale)
            cross_acc["reaction_meff"] = max(cross_acc["reaction_meff"],
                                             rel_diff(F0 + FL, -sec["mbar"] * Ld * cc.meff_frac[-1]))
    # frequency equations, independent route
    if model == "eb" and span in EB_EQ:
        bl = C.eb_equation_roots(EB_EQ[span], N_MODES)
        cross_acc["eb_equation"] = max(cross_acc["eb_equation"], *[rel_diff(b * b, m.Om) for b, m in zip(bl, mods)])
    if span == "S":
        alt = C.roots("pinned", "pinned", r2, 0, N_MODES)
        cross_acc["pp_determinant"] = max(cross_acc["pp_determinant"], *[rel_diff(a, m.Om) for a, m in zip(alt, mods)])
    return cc


def mode_doc(m, cc, i, span):
    d = {"mode": i + 1, "Omega": val(m.Om), "omega_rad_per_s": val(cc.omega[i]), "frequency_hz": val(cc.freq[i]),
         "participation_factor": val(cc.gamma[i]), "effective_mass_fraction": val(cc.meff_frac[i]),
         "shape": {"a": val(m.a), "b": val(m.b),
                   "c_cosh_sinh_cos_sin": [val(c) for c in m.c],
                   "normalizing_point_x_over_L": val(m.xstar),
                   "opposite_sign_equal_maxima_x_over_L": [val(t) for t in m.ties],
                   "W_at_x_over_L": {k: val(C.W(m, Fr(k[0], k[1]))) for k in ((1, 4), (1, 2), (3, 4), (1, 1))}},
         "integrals": {"int_W": val(m.I1), "int_W2": val(m.I2), "int_Wprime2": val(m.I3)},
         "per_unit_spectral_acceleration": {k: val(v) for k, v in cc.unit[i].items()}}
    d["shape"]["W_at_x_over_L"] = {f"{a}/{b}": v for (a, b), v in
                                   zip(((1, 4), (1, 2), (3, 4), (1, 1)), d["shape"]["W_at_x_over_L"].values())}
    if cc.gamma[i] == 0:
        d["participation_note"] = "zero by symmetry (antisymmetric shape, int W = 0 exactly)"
    if m.ties:
        d["normalization_note"] = ("|W| has equal maxima of opposite sign; the scope's 'largest component is +1' "
                                   "does not fix the sign; the smallest x is taken. The participation factor's "
                                   "sign depends on this choice; every response does not.")
    return d


def respond(freqs, omegas, unit, table, qnames, count):
    """Per-mode signed contributions and the combinations over the first `count` modes."""
    S = []
    for f in freqs[:count]:
        s = rules.evaluate(table, f)
        assert s is not None, ("periodic mode outside the table", table["id"], f)
        S.append(dec(SCALE) * (s if isinstance(s, Decimal) else dec(s)))
    contrib = {q: [unit[i][q] * S[i] for i in range(count)] for q in qnames}
    comb = {q: rules.combine_all(contrib[q], freqs[:count], omegas[:count], ZETA) for q in qnames}
    return S, contrib, comb


def rigid_continuum(cc, static, f_r, table, qnames):
    a_r = dec(SCALE) * dec(rules.evaluate(table, Fr(f_r)))
    per = [i for i in range(N_MODES) if cc.freq[i] < dec(f_r)]
    rig = {q: a_r * (static[q] - sum((cc.unit[i][q] for i in per), Decimal(0))) for q in qnames}
    return a_r, per, rig


def truncation_frequencies(cr):
    """f_r just above mode n of the Rayleigh continuum: 1.01 f_n rounded to 6 significant digits (binary64 input)."""
    out = {}
    for n in TRUNCATIONS:
        x = float(f"{float(cr.freq[n - 1] * Decimal('1.01')):.6g}")
        out[n] = x
    return out


# --------------------------------------------------------------------------
# finite elements


def fe_unit(md, ms, sec, span, N, gam_noise):
    """Per-mode Gamma, effective-mass fraction and per-unit quantities for an FE model."""
    with localcontext() as ctx:
        ctx.prec = fe.FE_PREC
        nd = len(md.K)
        Kd = [[dec(x) for x in r] for r in md.K]
        Md = [[dec(x) for x in r] for r in md.M]
        Rd = [dec(x) for x in md.R]
        p = [sum((Md[i][j] * Rd[j] for j in range(nd)), Decimal(0)) for i in md.free]
        Ld = dec(md.L)
        out = []
        for m in ms:
            phi = m["phi"]
            Mphi = [sum((md.Mf[i][j] * phi[j] for j in range(len(phi))), Decimal(0)) for i in range(len(phi))]
            gm = sum((a * b for a, b in zip(phi, Mphi)), Decimal(0))
            Lk = sum((a * b for a, b in zip(phi, p)), Decimal(0))
            if abs(Lk) <= Decimal(10) ** -40 * dsqrt(abs(gm) * Ld):
                gam_noise[0] = max(gam_noise[0], abs(Lk) / dsqrt(abs(gm) * Ld))
                Lk = Decimal(0)
            g = Lk / gm
            out.append({"gamma": g, "meff_frac": Lk * Lk / gm / Ld, "gm": gm, "Lk": Lk})
        return out, Kd, Md, Rd


def fe_quantities(md, Kd, Md, u_full, v_full, sec, span, N):
    """Quantities for displacement u and inertia multiplier v: forces K u - M v (times mbar)."""
    mb = sec["mbar"]
    nd = len(Kd)

    def nodal(i):
        return sum((Kd[i][j] * u_full[j] - Md[i][j] * v_full[j] for j in range(max(0, i - 3), min(nd, i + 4))),
                   Decimal(0))

    def elem_right(e):
        d = [2 * e, 2 * e + 1, 2 * e + 2, 2 * e + 3]
        f = [sum((dec(md.Ke[a][b]) * u_full[d[b]] - dec(md.Me[a][b]) * v_full[d[b]] for b in range(4)), Decimal(0))
             for a in range(4)]
        return f[2], f[3]

    out = {}
    for name, kind, spec in quantity_list(span, N):
        t = spec[0]
        if t == "w":
            out[name] = u_full[2 * int(spec[1] * N)]
        elif t == "F0":
            out[name] = mb * nodal(0)
        elif t == "M0":
            out[name] = mb * nodal(1)
        elif t == "FL":
            out[name] = mb * nodal(2 * N)
        elif t in ("M", "V"):
            j = int(spec[1] * N)
            V, M = elem_right(j - 1)
            out[name] = mb * (M if t == "M" else V)
    return out


def fe_case(span, N, L, sec, form, acc):
    md = fe.build(span, N, L, sec["EI_m"], sec["ibar_m"], form)
    ms = fe.solve_modes(md)
    info, Kd, Md, Rd = fe_unit(md, ms, sec, span, N, acc["gamma_noise"])
    nd = len(Kd)
    with localcontext() as ctx:
        ctx.prec = fe.FE_PREC
        # Sturm-count bisection (independent of Jacobi) and residuals
        for k, m in enumerate(ms[:N_MODES], start=1):
            lam = m["lam"]
            b = fe.bisect_eig(md.Kf, md.Mf, k, lam * Decimal("0.999"), lam * Decimal("1.001"), 40)
            acc["sturm"] = max(acc["sturm"], rel_diff(b, lam))
            n = len(m["phi"])
            r = [sum((md.Kf[i][j] - lam * md.Mf[i][j]) * m["phi"][j] for j in range(n)) for i in range(n)]
            kp = [sum(md.Kf[i][j] * m["phi"][j] for j in range(n)) for i in range(n)]
            acc["residual"] = max(acc["residual"], max(abs(x) for x in r) / max(abs(x) for x in kp))
        units = []
        for m, inf in zip(ms, info):
            u = [Decimal(0)] * nd
            for k, i in enumerate(md.free):
                u[i] = inf["gamma"] * m["phi"][k] / m["lam"]
            v = [x * m["lam"] for x in u]
            q = fe_quantities(md, Kd, Md, u, v, sec, span, N)
            units.append(q)
            # reaction balance: sum of translational reactions = -effective mass (per unit S)
            tr = [c for c in md.fixed if c % 2 == 0]
            s = sum((sum((Kd[c][j] * u[j] - Md[c][j] * v[j] for j in range(nd)), Decimal(0)) for c in tr),
                    Decimal(0)) * sec["mbar"]
            me = inf["meff_frac"] * dec(md.L) * sec["mbar"]
            if me != 0:
                acc["fe_balance"] = max(acc["fe_balance"], abs(s + me) / me)
    freqs = [dsqrt(m["lam"] * dec(1)) / (2 * pi()) for m in ms]
    omegas = [dsqrt(m["lam"]) for m in ms]
    return md, ms, info, units, freqs, omegas, (Kd, Md, Rd)


def fe_rigid(md, ms, info, freqs, mats, sec, span, N, f_r, table, acc):
    Kd, Md, Rd = mats
    nd = len(Kd)
    a_r = dec(SCALE) * dec(rules.evaluate(table, Fr(f_r)))
    per = [i for i in range(len(ms)) if freqs[i] < dec(f_r)]
    with localcontext() as ctx:
        ctx.prec = fe.FE_PREC
        v = Rd[:]
        for i in per:
            ph = [Decimal(0)] * nd
            for k, j in enumerate(md.free):
                ph[j] = ms[i]["phi"][k]
            v = [v[j] - info[i]["gamma"] * ph[j] for j in range(nd)]
        rig = [i for i in range(len(ms)) if i not in per]
        F = [sum((Md[i][j] * v[j] for j in range(nd)), Decimal(0)) for i in range(nd)]
        if not rig:
            # every finite mode is periodic: the free part of F vanishes exactly (complete basis), so u = 0
            # and only the mass at and coupled to the restrained DOFs loads the supports
            for j in md.free:
                F[j] = Decimal(0)
        uf = solve([[Kd[i][j] for j in md.free] for i in md.free], [F[i] for i in md.free]) if rig \
            else [Decimal(0)] * len(md.free)
        u = [Decimal(0)] * nd
        for k, i in enumerate(md.free):
            u[i] = uf[k]
        q = fe_quantities(md, Kd, Md, u, v, sec, span, N)
        # cross-check 1: displacement from the rigid modes' static contributions (complete FE basis)
        alt = [sum((info[i]["gamma"] * ms[i]["phi"][k] / ms[i]["lam"] for i in rig), Decimal(0))
               for k in range(len(md.free))]
        full = [sum((info[i]["gamma"] * ms[i]["phi"][k] / ms[i]["lam"] for i in range(len(ms))), Decimal(0))
                for k in range(len(md.free))]
        sc = max(abs(x) for x in full)
        acc["fe_rigid_modal"] = max(acc["fe_rigid_modal"], max(abs(a - b) for a, b in zip(uf, alt)) / sc)
        # cross-check 2: translational reactions balance the total static load
        tr = [c for c in md.fixed if c % 2 == 0]
        s = sum((sum((Kd[c][j] * u[j] for j in range(nd)), Decimal(0)) - F[c] for c in tr), Decimal(0))
        tot = sum((F[i] for i in range(0, nd, 2)), Decimal(0))
        if tot != 0:
            acc["fe_rigid_balance"] = max(acc["fe_rigid_balance"], abs(s + tot) / max(abs(tot), abs(s)))
    return a_r, per, {k: a_r * x for k, x in q.items()}


# --------------------------------------------------------------------------
# merged frequency list of the benchmark model: one pipe along X, fixed at A,
# UZ restrained at B. Excited along Y it is the cantilever (plane XY), along Z
# the propped cantilever (plane XZ); axial (X) and torsional (RX) modes are
# uncoupled and unexcited by transverse motion but belong to the mode count.


def rod_freqs_continuum(L, c2, count):
    return [(2 * n - 1) * dsqrt(dec(c2)) / (4 * dec(L)) for n in range(1, count + 1)]


def rod_freqs_fe(L, c2, N, form, count, acc):
    """Fixed-free rod of N linear elements: consistent (closed form) or lumped (half mass at each end)."""
    P = pi()
    out = []
    Ld = dec(L)
    for j in range(1, min(count, N) + 1):
        th = (2 * j - 1) * P / (2 * N)
        if form == "consistent":
            lam_hat = 6 * N * N * (1 - C.dcos(th)) / (2 + C.dcos(th))
        else:
            lam_hat = 2 * N * N * (1 - C.dcos(th))
        out.append(dsqrt(lam_hat * dec(c2)) / Ld / (2 * P))
    # cross-check by exact-rational Sturm counts of the assembled rod pencil, bracketing each eigenvalue
    h = Fr(1, N)
    K = [[Fr(0)] * (N + 1) for _ in range(N + 1)]
    M = [[Fr(0)] * (N + 1) for _ in range(N + 1)]
    for e in range(N):
        for a in range(2):
            for b in range(2):
                K[e + a][e + b] += (1 if a == b else -1) / h
                if form == "consistent":
                    M[e + a][e + b] += h * (Fr(1, 3) if a == b else Fr(1, 6))
                elif a == b:
                    M[e + a][e + b] += h / 2
    Kf = [r[1:] for r in K[1:]]
    Mf = [r[1:] for r in M[1:]]
    with localcontext() as ctx:
        ctx.prec = fe.FE_PREC
        Kd = [[dec(x) for x in r] for r in Kf]
        Md = [[dec(x) for x in r] for r in Mf]
        for j in range(1, min(count, N) + 1):
            th = (2 * j - 1) * P / (2 * N)
            lam_hat = (6 * N * N * (1 - C.dcos(th)) / (2 + C.dcos(th))) if form == "consistent" \
                else 2 * N * N * (1 - C.dcos(th))
            lo = fe.sturm_count(Kd, Md, lam_hat * (1 - Decimal("1e-30")))
            hi = fe.sturm_count(Kd, Md, lam_hat * (1 + Decimal("1e-30")))
            acc["rod_counts_ok"] &= (lo == j - 1 and hi == j)
    return out


def merged_list(entries):
    """entries: list of (freq, family, index). Returns the sorted list and Y/Z pairs within 1 percent."""
    entries = sorted(entries, key=lambda t: t[0])
    close = []
    ys = [e for e in entries if e[1].startswith("Y")]
    zs = [e for e in entries if e[1].startswith("Z")]
    for y in ys:
        for z in zs:
            if abs(y[0] - z[0]) <= Decimal("0.01") * min(y[0], z[0]):
                close.append({"Y_mode": y[2], "Z_mode": z[2], "relative_gap": val(abs(y[0] - z[0]) / min(y[0], z[0]))})
    return entries, close


def merged_doc(Ybend, Zbend, axial, tors):
    ent = [(f, "Y bending (cantilever plane XY)", i + 1) for i, f in enumerate(Ybend)] + \
          [(f, "Z bending (propped plane XZ)", i + 1) for i, f in enumerate(Zbend)]
    cutoff = max(Ybend[min(N_MODES, len(Ybend)) - 1], Zbend[min(N_MODES, len(Zbend)) - 1])
    ent = [e for e in ent if e[0] <= cutoff or (e[1].startswith("Y") and e[2] <= N_MODES)
           or (e[1].startswith("Z") and e[2] <= N_MODES)]
    ent += [(f, "X axial", i + 1) for i, f in enumerate(axial) if f <= cutoff]
    ent += [(f, "RX torsion", i + 1) for i, f in enumerate(tors) if f <= cutoff]
    ent, close = merged_list(ent)
    return {"cutoff_hz": val(cutoff),
            "modes_ascending": [{"count": k + 1, "frequency_hz": val(f), "family": fam, "family_mode": i}
                                for k, (f, fam, i) in enumerate(ent)],
            "Y_Z_pairs_within_1_percent": close}
