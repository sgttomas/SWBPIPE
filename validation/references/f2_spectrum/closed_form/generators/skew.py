"""Closely spaced modes from a tip spring, and the repeated-pair basis ambiguity.

General case. A straight cantilever along the unit axis a, bending planes
spanned by the orthonormal transverse unit vectors p and q. A tip spring of
effective stiffness k_eff acts in plane p only; uniform support motion along
the unit direction e with components c_p = e.p, c_q = e.q. Bending in p is a
cantilever with a tip spring (kappa = k_eff L^3/EI); bending in q is a plain
cantilever. For mode n of plane j (shape W_n(x), any scaling), with
D_n = int W^2 + r2 int W'^2 and w_n its circular frequency, per unit spectral
acceleration:
  displacement field   u(x) = c_j (I1/D_n) W_n(x) / w_n^2  j_hat      (I1 = int_0^1 W)
  effective mass       c_j^2 mbar L I1^2 / D_n
  clamp force          (EI/L^3) A_n W_n'''(0) j_hat,   A_n = c_j (I1/D_n)/w_n^2
  clamp moment         -(EI/L^2) A_n W_n''(0) (a x j_hat)
  spring reaction      -K_s u_tip (plane p modes only)
so contributions to a component along e of plane j's modes carry the sign of
c_j^2 (all positive) and to the perpendicular component the sign of c_p c_q
(opposite for the two planes when e is not along p or q).
"""

from decimal import Decimal
from fractions import Fraction as Fr

import beams as B
import continuum as C
import rules
from numerics import dec, dsqrt, inp, num, pi, rel_diff, sci, val
from tables import T1

L = 2.0
N_PER_PLANE = 3
TABLE = dict(T1, rule=("log", "log"))
SCALE = 1.0
ZETA = 0.05
K_A = 57.0     # case (a): spring along p, k_eff = k
K_B = 85.5     # case (b): spring along X, k_eff = 2k/3 = 57.0 exactly


def cross3(u, v):
    return [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]]


def unit(v):
    n = dsqrt(sum(x * x for x in v))
    return [x / n for x in v]


def D(v):
    return [dec(Fr(x)) for x in v]


CASES = {
    "a": {"axis": D([1, 0, 0]), "p": unit(D([0, 1, 1])), "q": unit(D([0, 1, -1])), "spring_dir": unit(D([0, 1, 1])),
          "k": K_A, "excitation": D([0, 1, 0]),
          "description": "pipe along X, spring along (0,1,1) in plane p, excitation along Y: c_p = c_q = 1/sqrt 2"},
    "b": {"axis": unit(D([1, 1, 1])), "p": unit(D([2, -1, -1])), "q": unit(D([0, 1, -1])), "spring_dir": D([1, 0, 0]),
          "k": K_B, "excitation": D([0, 1, 0]),
          "description": "pipe along the space diagonal (1,1,1), spring along global X with stiffness k, excitation "
                         "along global Y: p = (2,-1,-1)/sqrt 6, q = (0,1,-1)/sqrt 2, k_eff = 2k/3, "
                         "c_p = -1/sqrt 6, c_q = 1/sqrt 2; the coupling of the spring's axial component to axial "
                         "deformation is neglected (pipe axially rigid)"},
}
COMP = ["X", "Y", "Z"]


def plane_modes(sec, model, kappa):
    r2 = sec["ibar_m"] / Fr(L) ** 2 if model == "rayleigh" else Fr(0)
    mp = C.modes("clamped", "spring", r2, kappa, N_PER_PLANE)
    mq = C.modes("clamped", "free", r2, 0, N_PER_PLANE)
    return mp, mq, r2


def case_doc(cid, sec, model, acc):
    cs = CASES[cid]
    EI, mb, Ld = sec["EI"], sec["mbar"], dec(L)
    dvec = cs["spring_dir"]
    k = dec(cs["k"])
    dp = sum(x * y for x, y in zip(dvec, cs["p"]))
    dq = sum(x * y for x, y in zip(dvec, cs["q"]))
    da = sum(x * y for x, y in zip(dvec, cs["axis"]))
    assert abs(dq) < Decimal("1e-80")
    keff = k * dp * dp
    kappa = keff * Ld ** 3 / EI
    cp = sum(x * y for x, y in zip(cs["excitation"], cs["p"]))
    cq = sum(x * y for x, y in zip(cs["excitation"], cs["q"]))
    ca = sum(x * y for x, y in zip(cs["excitation"], cs["axis"]))
    mp, mq, r2 = plane_modes(sec, model, kappa)
    sq = dsqrt(dec(sec["EI_m"]))
    modes = []
    for plane, ms, cj, ej in (("p", mp, cp, cs["p"]), ("q", mq, cq, cs["q"])):
        for m in ms:
            w = m.Om * sq / Ld ** 2
            modes.append({"plane": plane, "m": m, "w": w, "f": w / (2 * pi()), "c": cj, "e": ej})
    modes.sort(key=lambda d: d["f"])
    if model == "eb":
        bt = C.eb_equation_roots("tip_spring", N_PER_PLANE, kappa)
        acc["tip_eq"] = max(acc["tip_eq"], *[rel_diff(b * b, m.Om) for b, m in zip(bt, mp)])
    rows = []
    unit_q = []
    for i, d in enumerate(modes):
        m, cj, ej, w = d["m"], d["c"], d["e"], d["w"]
        den = m.I2 + m.r2 * m.I3
        A = cj * (m.I1 / den) / (w * w)
        tip = [A * C.W(m, 1) * x for x in ej]
        Fb = [EI / Ld ** 3 * A * C.W(m, 0, 3) * x for x in ej]
        Mb = [-EI / Ld ** 2 * A * C.W(m, 0, 2) * x for x in cross3(cs["axis"], ej)]
        Fs = [Decimal(0)] * 3
        if d["plane"] == "p":
            du = sum(x * y for x, y in zip(dvec, tip))
            Fs = [-k * du * x for x in dvec]
        fa = sum(x * y for x, y in zip(Fs, cs["axis"]))
        Fa = [-fa * x for x in cs["axis"]]           # axially rigid pipe carries the spring's axial part to the clamp
        Fc = [x + y for x, y in zip(Fb, Fa)]
        meff = cj * cj * mb * Ld * m.I1 ** 2 / den
        # equilibrium: clamp + spring + transverse inertia = 0
        inert = [cj * mb * Ld * m.I1 ** 2 / den * x for x in ej]
        res = max(abs(Fc[t] + Fs[t] + inert[t]) for t in range(3)) / max(abs(x) for x in inert)
        acc["force_eq"] = max(acc["force_eq"], res)
        q1, q2, q3, j1 = C.quad_check(m)
        mom_in = [cj * (m.I1 / den) * mb * Ld ** 2 * (j1 + m.r2 * C.W(m, 1)) * x for x in cross3(cs["axis"], ej)]
        mom_s = cross3([Ld * x for x in cs["axis"]], Fs)
        res_m = max(abs(Mb[t] + mom_s[t] + mom_in[t]) for t in range(3)) / max(abs(x) for x in mom_in)
        acc["moment_eq"] = max(acc["moment_eq"], res_m)
        # scope normalization in global components
        comps = [abs(x) for x in ej]
        big = max(comps)
        near = [t for t in range(3) if big - comps[t] <= big * Decimal("1e-60")]
        istar = near[0]
        tie_opp = any((ej[t] < 0) != (ej[istar] < 0) for t in near)
        sigma = 1 / (C.W(m, m.xstar) * ej[istar])
        gamma = cj * m.I1 / (sigma * den)
        q = {}
        for t in range(3):
            q[f"tip_u{COMP[t]}_m"] = tip[t]
        for t in range(3):
            q[f"clamp_F{COMP[t]}_N"] = Fc[t]
        for t in range(3):
            q[f"clamp_M{COMP[t]}_Nm"] = Mb[t]
        if cs["k"]:
            for t in range(3):
                q[f"spring_F{COMP[t]}_N"] = Fs[t]
        unit_q.append(q)
        rows.append({"mode": i + 1, "plane": d["plane"], "family_mode": m.n, "Omega": val(m.Om),
                     "frequency_hz": val(d["f"]), "omega_rad_per_s": val(w),
                     "participation_factor_along_excitation": val(gamma),
                     "normalizing_component": COMP[istar],
                     "equal_largest_components_of_opposite_sign": tie_opp,
                     "effective_mass_kg": val(meff), "effective_mass_fraction": val(meff / (mb * Ld)),
                     "plane_quantities_per_unit": {"I1": val(m.I1), "D": val(den), "W(1)": val(C.W(m, 1)),
                                                   "W'''(0)": val(C.W(m, 0, 3)), "W''(0)": val(C.W(m, 0, 2))},
                     "per_unit_spectral_acceleration": {k_: val(v) for k_, v in q.items()}})
    freqs = [d["f"] for d in modes]
    omegas = [d["w"] for d in modes]
    S = [dec(SCALE) * rules.evaluate(TABLE, f) for f in freqs]
    names = list(unit_q[0].keys())
    contrib = {n: [unit_q[i][n] * S[i] for i in range(len(modes))] for n in names}
    comb = {n: {r: val(v) for r, v in rules.combine_all(contrib[n], freqs, omegas, ZETA).items()} for n in names}
    split = modes[1]["f"] / modes[0]["f"] - 1
    doc = {"description": cs["description"], "model": model,
           "axis_a": [val(x) for x in cs["axis"]], "p": [val(x) for x in cs["p"]], "q": [val(x) for x in cs["q"]],
           "spring_direction": [val(x) for x in dvec], "spring_stiffness_k_N_per_m": inp(cs["k"]),
           "k_eff_N_per_m": val(keff), "kappa = k_eff L^3/EI": val(kappa),
           "c_p": val(cp), "c_q": val(cq), "c_axial": val(ca),
           "first_pair_relative_split": val(split),
           "modes": rows, "groups_10pct_of_lowest": [[i + 1 for i in g] for g in rules.groups(freqs)],
           "ten_percent_pairs": [[i + 1, j + 1] for i, j in rules.close_pairs(freqs)],
           "spectral_ordinates_sS": [val(s) for s in S],
           "signed_contributions": {n: [val(v) for v in contrib[n]] for n in names},
           "combined": comb}
    # sign pattern of the first pair
    sgn = {n: ("same" if (contrib[n][0] > 0) == (contrib[n][1] > 0) else "opposite")
           for n in names if contrib[n][0] != 0 and contrib[n][1] != 0}
    doc["first_pair_sign_pattern"] = sgn
    if cid == "a":
        doc["use"] = ("verification and comparison: the spring is perpendicular to the pipe (d.a = 0), so no axial "
                      "coupling is neglected")
    if cid == "b":
        doc["use"] = ("COMPARISON ONLY, NOT FOR VERIFICATION AT THE ANALYTIC TOLERANCE. Case (b) assumes an axially "
                      "rigid pipe. The neglected coupling of the spring's axial component to axial deformation lowers "
                      "the first plane-p frequency by about 8.1e-8 relative and changes the plane-p mode 1 "
                      "contributions by about 1e-7, a hundred times the 1e-9 verification tolerance, and the coupled "
                      "modes acquire axial components that participate through c_axial. Case (b) is cross-engine "
                      "comparison data and cannot verify a product model with finite axial stiffness at 1e-9. Case (a) "
                      "is unaffected.")
        EA = sec["EI"] * dec(sec["EA_m"]) / dec(sec["EI_m"])
        flex_ratio = 3 * dec(sec["EI"]) / (EA * Ld ** 2)
        dk = k * da * da * (Ld / EA) / (1 + k * da * da * Ld / EA)
        est = []
        for d in modes:
            if d["plane"] == "p":
                m = d["m"]
                rel = -dk * kappa * C.W(m, 1) ** 2 / (2 * m.Om ** 2 * (m.I2 + m.r2 * m.I3))
                est.append({"plane_p_mode": m.n, "first_order_relative_frequency_change": sci(rel, 6)})
        fax = dsqrt(dec(sec["EA_m"])) / (4 * Ld)
        doc["neglected_axial_coupling"] = {
            "formula": "with the pipe's axial tip compliance L/EA, the spring seen by plane p is "
                       "k d_p^2 (EA/L)/(EA/L + k d_a^2), a relative reduction delta = k d_a^2 (L/EA)/(1 + k d_a^2 L/EA); "
                       "its first-order effect on a plane-p frequency is -delta kappa W(1)^2/(2 Om^2 D)",
            "ratio_axial_to_bending_tip_flexibility 3I/(A L^2)": sci(flex_ratio, 6),
            "delta": sci(dk, 6), "frequency_effect": est}
        doc["axial_component_of_excitation"] = {
            "note": "c_axial = 1/sqrt 3 excites the axial modes, uncoupled here; their first frequency is far above "
                    "T1's range, so a benchmark must treat them as rigid (missing-mass correction with f_r below "
                    "them) or exclude the axial component; they are not in the combinations above",
            "first_axial_frequency_hz (fixed-free rod, spring neglected)": val(fax),
            "static_axial_response_per_unit_rigid_acceleration": {
                "tip_axial_displacement_m = c_a mbar L^2/(2 EA)": val(ca * mb * Ld ** 2 / (2 * EA)),
                "clamp_axial_reaction_N = -c_a mbar L": val(-ca * mb * Ld)}}
    return doc


def repeated_pair(sec, acc):
    """Plain cantilever, mode 1 in two planes with the pair's basis rotated by theta."""
    m = C.modes("clamped", "free", 0, 0, 1)[0]
    sq = dsqrt(dec(sec["EI_m"]))
    Ld = dec(L)
    w = m.Om * sq / Ld ** 2
    f = w / (2 * pi())
    S = dec(SCALE) * rules.evaluate(TABLE, f)
    A = (m.I1 / m.I2) * C.W(m, 1) / (w * w) * S
    rows = []
    P = pi()
    for deg in (0, 15, Fr(45, 2), 30, 45, 60, Fr(135, 2), 90):
        th = dec(Fr(deg)) * P / 180
        c, s = C.dcos(th), C.dsin(th)
        uy = [c * c * A, s * s * A]
        uz = [c * s * A, -s * c * A]
        r = {}
        for name, v in (("tip_uY_m", uy), ("tip_uZ_m", uz)):
            comb = rules.combine_all(v, [f, f], [w, w], ZETA)
            # adopted reading: the cluster's modes are summed with their signs first, then each rule is
            # applied to the single cluster sum
            adopted = rules.combine_all([v[0] + v[1]], [f], [w], ZETA)
            r[name] = {"contributions": [val(x) for x in v], **{k: val(x) for k, x in comb.items()},
                       "basis_invariant_sum_first": val(abs(v[0] + v[1])),
                       "adopted_cluster_sum_first": {k: val(x) for k, x in adopted.items()}}
            acc["pair_cqc"] = max(acc["pair_cqc"], abs(comb["CQC"] - abs(v[0] + v[1])) / abs(A))
            acc["pair_adopted_ok"] &= all(x == abs(v[0] + v[1]) for x in adopted.values())
        rows.append({"theta_deg": num(Fr(deg)), **r})
    r2 = dsqrt(Decimal(2))
    return {
        "description": "a circular cantilever free in three dimensions: mode 1 is an exactly repeated pair. Any "
                       "orthonormal basis (e1, e2) of the pair is a valid output; with e1 = (cos t, sin t), "
                       "e2 = (-sin t, cos t) in (Y, Z) and excitation along Y, mode e1 contributes cos^2 t A to uY and "
                       "cos t sin t A to uZ, mode e2 contributes sin^2 t A and -sin t cos t A. The scope's 'largest "
                       "component is +1' normalization does not fix t.",
        "A = in-plane tip response of mode 1 (m)": val(A), "frequency_hz": val(f),
        "closed_forms": {
            "uY SRSS": "A sqrt(cos^4 t + sin^4 t): max A at t = 0, min A/sqrt 2 at t = 45 deg",
            "uZ SRSS": "A |sin 2t|/sqrt 2: min 0 at t = 0, max A/sqrt 2 at t = 45 deg",
            "basis invariant (signed contributions summed within the pair first)": "uY = A, uZ = 0",
            "adopted reading (cluster summed first, then any rule)": "uY = A, uZ = 0 for every t and every rule",
            "CQC": "rho = 1 for equal frequencies, so CQC = |c1 + c2| = the invariant value for every t",
            "GROUPING, TEN_PERCENT, ABSSUM": "|c1| + |c2|: uY = A (invariant), uZ = A |sin 2t| (0 to A)"},
        "extremes": {"uY_SRSS_max": val(A), "uY_SRSS_min": val(A / r2), "uZ_SRSS_min": "0",
                     "uZ_SRSS_max": val(A / r2), "uZ_GROUPING_max": val(A)},
        "reading_of_the_rule_fields": (
            "In rows, the fields SRSS, CQC, GROUPING, ABSSUM and TEN_PERCENT (and the closed_forms and extremes "
            "entries named after them) apply each rule mode by mode: NOT the adopted reading, published to show the "
            "basis dependence. The adopted rule is DEL-04-10 CLM-001, Repeated modes: the modes of one cluster of the "
            "modal basis (DEL-04-09's cluster tolerance delta_c = 2^-20 on eigenvalues) are summed with their signs "
            "first and every modal rule is applied to the cluster sums. Here the pair is one cluster with one "
            "frequency, so each rule is applied to the single value c1 + c2 and returns |c1 + c2|: every rule gives "
            "the basis_invariant_sum_first values, uY = A and uZ = 0, at every angle; adopted_cluster_sum_first gives "
            "that result per rule for comparison with a product. The invariance of the cluster sum is exact for an "
            "exactly repeated eigenvalue and an M-orthogonal basis, as here."),
        "derivation_of_A": (
            "Euler-Bernoulli cantilever of the item 5 section (beams_continuum.json) at L = 2 m, mode 1: "
            "Omega_1 = beta_1^2 with cos beta cosh beta = -1, w_1 = Omega_1 sqrt(EI/mbar)/L^2, f_1 = w_1/(2 pi); "
            "W normalized to W(1) = +1 (the tip is the maximum); A = (int_0^1 W / int_0^1 W^2) W(1) s S(f_1)/w_1^2 "
            "with S from T1 under log-abscissa/log-ordinate and s = 1. A is the in-plane tip displacement of the mode "
            "under excitation in its own plane; the pair's contributions are A times the products of the basis "
            "vectors' components given in description."),
        "rows": rows,
    }


def build(cross):
    sec = B.section()
    acc = {"tip_eq": Decimal(0), "force_eq": Decimal(0), "moment_eq": Decimal(0), "pair_cqc": Decimal(0),
           "pair_adopted_ok": True}
    out = {"item": "Closely spaced pair from a tip spring (general p, q form), and the exactly repeated pair under a "
                   "rotated basis (DEL-04-10 AC-002)",
           "common": {"section": "as beams_continuum.json", "length_m": inp(L),
                      "spectrum": "T1 under log-abscissa/log-ordinate", "scale_factor_s": inp(SCALE),
                      "damping_ratio_for_CQC": inp(ZETA), "modes": f"first {N_PER_PLANE} of each plane"},
           "cases": {}}
    for cid in ("a", "b"):
        out["cases"][cid] = {m: case_doc(cid, sec, m, acc) for m in ("eb", "rayleigh")}
    out["repeated_pair"] = repeated_pair(sec, acc)
    cross.append({"id": "SKEW-tip-spring-equation", "what": "plane p roots: determinant route against "
                  "beta^3(1 + cos cosh) - kappa(cos sinh - sin cosh) = 0", "max": sci(acc["tip_eq"]),
                  "agrees": acc["tip_eq"] < Decimal("1e-60")})
    cross.append({"id": "SKEW-force-equilibrium", "what": "per mode: clamp + spring + inertia = 0 in global components",
                  "max": sci(acc["force_eq"]), "agrees": acc["force_eq"] < Decimal("1e-40")})
    cross.append({"id": "SKEW-moment-equilibrium", "what": "per mode: clamp moment + L a x F_spring + inertia moments "
                  "(quadrature of int x W) = 0", "max": sci(acc["moment_eq"]), "agrees": acc["moment_eq"] < Decimal("1e-40")})
    cross.append({"id": "PAIR-adopted-reading", "what": "under the adopted reading every rule equals the "
                  "basis-invariant value at every theta, exactly", "agrees": acc["pair_adopted_ok"]})
    cross.append({"id": "PAIR-CQC-invariant", "what": "CQC of the rotated repeated pair equals the basis-invariant "
                  "value for every theta", "max": sci(acc["pair_cqc"]), "agrees": acc["pair_cqc"] < Decimal("1e-60")})
    return out
