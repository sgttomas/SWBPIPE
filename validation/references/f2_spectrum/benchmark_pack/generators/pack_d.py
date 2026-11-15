"""Case D: the Case C pipe along the space diagonal from the origin toward (1,1,1), fully
fixed at A, one linear spring of 8 N/m along global X at the tip, FLAT along Y only.

Continuum closed form in the general plane form of closed_form/beam_skew_and_pairs.json:
axis a = (1,1,1)/sqrt 3, planes p = (2,-1,-1)/sqrt 6 and q = (0,1,-1)/sqrt 2, spring
direction d = X with d.p = 2/sqrt 6, d.q = 0, d.a = 1/sqrt 3, so k_eff = k (d.p)^2 = 2k/3
acts in plane p only; c_p = Y.p = -1/sqrt 6, c_q = Y.q = 1/sqrt 2, c_a = Y.a = 1/sqrt 3.
The pipe is taken as axially rigid under the spring's axial component (neglected coupling,
size stated). Plane p is a cantilever with a tip spring, plane q a plain cantilever.
"""

from decimal import Decimal
from fractions import Fraction as Fr

from pinned import load
import pack_c as PC

N_, R_, C_, FE_, B_ = load()
dec, dsqrt, inp, val, sci, pi, rel_diff = N_.dec, N_.dsqrt, N_.inp, N_.val, N_.sci, N_.pi, N_.rel_diff

K_SPRING = 8.0
N_PER_PLANE = 3
COMP = ["X", "Y", "Z"]


def unit(v):
    v = [dec(Fr(x)) for x in v]
    n = dsqrt(sum(x * x for x in v))
    return [x / n for x in v]


def cross3(u, v):
    return [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]]


def dot(u, v):
    return sum((x * y for x, y in zip(u, v)), Decimal(0))


A_AX, P_, Q_, D_, E_ = unit([1, 1, 1]), unit([2, -1, -1]), unit([0, 1, -1]), unit([1, 0, 0]), unit([0, 1, 0])


def model(sec, mname, acc):
    L, EI, mb = dec(PC.L), sec["EI"], sec["mbar"]
    k = dec(K_SPRING)
    dp, da = dot(D_, P_), dot(D_, A_AX)
    keff = k * dp * dp
    kappa = keff * L ** 3 / EI
    cp, cq, ca = dot(E_, P_), dot(E_, Q_), dot(E_, A_AX)
    r2 = sec["ibar_m"] / Fr(PC.L) ** 2 if mname == "rayleigh" else Fr(0)
    mp = C_.modes("clamped", "spring", r2, kappa, N_PER_PLANE)
    mq = C_.modes("clamped", "free", r2, 0, N_PER_PLANE)
    if mname == "eb":
        bt = C_.eb_equation_roots("tip_spring", N_PER_PLANE, kappa)
        acc["tip_eq"] = max(acc["tip_eq"], *[rel_diff(b * b, m.Om) for b, m in zip(bt, mp)])
    sq = dsqrt(dec(sec["EI_m"]))
    modes = []
    for plane, ms, cj, ej in (("p", mp, cp, P_), ("q", mq, cq, Q_)):
        for m in ms:
            w = m.Om * sq / L ** 2
            modes.append({"plane": plane, "m": m, "w": w, "f": w / (2 * pi()), "c": cj, "e": ej})
    modes.sort(key=lambda d: d["f"])
    rows, units = [], []
    for i, d in enumerate(modes):
        m, cj, ej, w = d["m"], d["c"], d["e"], d["w"]
        den = m.I2 + m.r2 * m.I3
        Aa = cj * (m.I1 / den) / (w * w)
        tip = [Aa * C_.W(m, 1) * x for x in ej]
        Fb = [EI / L ** 3 * Aa * C_.W(m, 0, 3) * x for x in ej]
        Mb = [-EI / L ** 2 * Aa * C_.W(m, 0, 2) * x for x in cross3(A_AX, ej)]
        Fs = [-k * dot(D_, tip) * x for x in D_] if d["plane"] == "p" else [Decimal(0)] * 3
        fa = dot(Fs, A_AX)
        Fc = [Fb[t] - fa * A_AX[t] for t in range(3)]
        inert = [cj * mb * L * m.I1 ** 2 / den * x for x in ej]
        acc["force_eq"] = max(acc["force_eq"], max(abs(Fc[t] + Fs[t] + inert[t]) for t in range(3)) /
                              max(abs(x) for x in inert))
        comps = [abs(x) for x in ej]
        big = max(comps)
        near = [t for t in range(3) if big - comps[t] <= big * Decimal("1e-60")]
        sigma = 1 / (C_.W(m, m.xstar) * ej[near[0]])
        gamma = cj * m.I1 / (sigma * den)
        meff = cj * cj * mb * L * m.I1 ** 2 / den
        q = {}
        for t in range(3):
            q[f"tip_u{COMP[t]}_m"] = tip[t]
        for t in range(3):
            q[f"base_F{COMP[t]}_N"] = Fc[t]
        for t in range(3):
            q[f"base_M{COMP[t]}_Nm"] = Mb[t]
        q["spring_FX_N"] = Fs[0]
        units.append(q)
        rows.append({"mode": i + 1, "plane": d["plane"], "family_mode": m.n, "frequency_hz": val(d["f"]),
                     "participation_factor_along_Y": val(gamma), "normalizing_component": COMP[near[0]],
                     "equal_largest_components_of_opposite_sign": any((ej[t] < 0) != (ej[near[0]] < 0) for t in near),
                     "effective_mass_Y_kg": val(meff), "effective_mass_fraction_Y": val(meff / (mb * L)),
                     "per_unit_spectral_acceleration": {kk: val(v) for kk, v in q.items()}})
    freqs = [d["f"] for d in modes]
    omegas = [d["w"] for d in modes]
    S = [PC.S_of(PC.FLAT, f) for f in freqs]
    names = list(units[0].keys())
    contrib = {n: [units[i][n] * S[i] for i in range(len(modes))] for n in names}

    def comb(count):
        out = {}
        for n in names:
            v = contrib[n][:count]
            out[n] = {"SRSS": R_.srss(v), "ABSSUM": R_.abssum(v), "TEN_PERCENT": R_.pairwise10(v, freqs[:count]),
                      "CQC": R_.cqc(v, omegas[:count], PC.ZETA)}
        return out

    c2, c6 = comb(2), comb(6)
    EA = EI * dec(sec["EA_m"]) / dec(sec["EI_m"])
    dk = k * da * da * (L / EA) / (1 + k * da * da * L / EA)
    est = [{"plane_p_mode": d["m"].n, "first_order_relative_frequency_change":
            sci(-dk * kappa * C_.W(d["m"], 1) ** 2 / (2 * d["m"].Om ** 2 * (d["m"].I2 + d["m"].r2 * d["m"].I3)), 6)}
           for d in modes if d["plane"] == "p"]
    doc = {"model": mname, "k_eff_N_per_m": val(keff), "kappa = k_eff L^3/EI": val(kappa),
           "c_p": val(cp), "c_q": val(cq), "c_axial": val(ca),
           "first_pair_relative_split": val(freqs[1] / freqs[0] - 1),
           "modes_first_two": rows[:2], "modes_3_to_6_for_information": rows[2:],
           "spectral_ordinates_sS_first_two": [val(s) for s in S[:2]],
           "signed_contributions_first_two": {n: [val(x) for x in contrib[n][:2]] for n in names},
           "combined_first_two": {n: {r: val(x) for r, x in c2[n].items()} for n in names},
           "combined_first_six_for_information": {n: {r: val(x) for r, x in c6[n].items()} for n in names},
           "neglected_axial_coupling": {
               "delta = k d_a^2 (L/EA)/(1 + k d_a^2 L/EA)": sci(dk, 6),
               "ratio_axial_to_bending_tip_flexibility 3I/(A L^2)": sci(3 * EI / (EA * L ** 2), 6),
               "frequency_effect": est}}
    return doc, c2, c6


def build(cross, store):
    sec = PC.section()
    acc = {"tip_eq": Decimal(0), "force_eq": Decimal(0)}
    out = {"case": "D", "model": __doc__.strip().split("\n\n")[0].replace("\n", " "),
           "method": __doc__.strip().split("\n\n")[1].replace("\n", " "),
           "spring": {"stiffness_N_per_m": inp(K_SPRING), "direction": "global X"},
           "spectrum": "FLAT of case_c.json along Y only, scale factor 1; CQC at damping 0.05",
           "periodic": "the case defines six periodic modes with no correction; the continuum here gives the first "
                       "two modes (the closely spaced pair) and their combination as asked, and modes 3 to 6 with the "
                       "six-mode combination for information only",
           "axial_component": "c_axial = 1/sqrt 3 excites the axial modes (first near 84 Hz); they are not among the "
                              "first six modes, so without the correction they do not enter",
           "use": "COMPARISON ONLY, NOT FOR VERIFICATION AT THE ANALYTIC TOLERANCE. The continuum assumes an "
                  "axially rigid pipe. The neglected coupling of the spring's axial component to axial deformation "
                  "lowers the first plane-p frequency by about 1.4e-8 relative (neglected_axial_coupling), which "
                  "moves the plane-p mode 1 displacement contributions by about 3e-8 under FLAT through 1/w^2 alone, "
                  "well above the 1e-9 verification tolerance, and the coupled modes acquire axial components that "
                  "participate through c_axial. These values are cross-engine comparison data and cannot verify a "
                  "product model with finite axial stiffness at 1e-9.",
           "n08_values": "The N = 8 values for this case are the sibling discrete package's "
                         "(validation/references/f2_spectrum/discrete/), not this package's.",
           "rules": "SRSS; CQC (damping 0.05); ABSSUM (the absolute sum); TEN_PERCENT (the ten percent method): modal "
                    "rules of DEL-04-10 CLM-001. The first pair is 5 % apart, far outside DEL-04-09's cluster "
                    "tolerance, so no cluster is summed first",
           "models": {}}
    sums = {}
    for m in ("eb", "rayleigh"):
        doc, c2, c6 = model(sec, m, acc)
        out["models"]["euler_bernoulli" if m == "eb" else "rayleigh"] = doc
        sums[m] = (c2, c6)
    cross.append({"id": "D-tip-spring-equation", "what": "plane p roots: determinant against "
                  "beta^3(1 + cos cosh) - kappa(cos sinh - sin cosh) = 0", "max": sci(acc["tip_eq"]),
                  "agrees": acc["tip_eq"] < Decimal("1e-60")})
    cross.append({"id": "D-force-equilibrium", "what": "per mode: clamp + spring + inertia = 0 in global components",
                  "max": sci(acc["force_eq"]), "agrees": acc["force_eq"] < Decimal("1e-40")})
    store["d"] = sums
    return out
