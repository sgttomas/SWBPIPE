"""Item 3: two-mass lumped chain, ground - k1 - m1 - k2 - m2 (DEL-04-10 REQ-002 to REQ-004; AC-002).

Free DOFs u1, u2 along the chain, relative to the ground; excitation along
the chain, R = (1, 1), p = M R = (m1, m2) (lumped mass, no coupling to the
ground). K = [[k1 + k2, -k2], [-k2, k2]], M = diag(m1, m2).
det(K - lam M) = 0:  m1 m2 lam^2 - (m1 k2 + m2 (k1 + k2)) lam + k1 k2 = 0.
Shape from the second row: phi1/phi2 = (k2 - lam m2)/k2; normalized so the
component of largest magnitude is +1 (DEL-04-09 REQ-005 case (a)).
Response quantities: u1, u2; spring 1 force k1 u1 and spring 2 force
k2 (u2 - u1) (tension positive); ground reaction K_cf u = -k1 u1 (the same
value whether k1 is an element to a rigidly restrained ground node or a
ground spring, -K_s u).
"""

import math
from decimal import Decimal
from fractions import Fraction

from numerics import dec, dsqrt, inp, num, pi, rel_diff, sci, solve, val
import rules
from tables import T1

TABLE = dict(T1, rule=("log", "log"))
SCALE = 1.25
ZETA = T1["damping"]

SETS = {
    "A": {"m1": 200.0, "m2": 100.0, "k1": 500000.0, "k2": 150000.0,
          "purpose": "well separated modes (5.03 Hz, 9.75 Hz); exact rational eigenvalues 1000 and 3750; "
                     "missing mass with f_r between the modes (mode 2 rigid) and above both",
          "rigid_frequencies": [7.0, 12.0]},
    "B": {"m1": 1000.0, "m2": 4.0, "k1": 400000.0, "k2": 1600.0,
          "purpose": "closely spaced pair (frequency ratio 1.065, within 10 %): contributions of opposite sign "
                     "to the spring 2 force (CQC below SRSS) and of the same sign to u1 and the ground "
                     "reaction (CQC above SRSS)",
          "rigid_frequencies": []},
    "C": {"m1": 200.0, "m2": 100.0, "k1": 500000000.0, "k2": 150000000.0,
          "purpose": "stiff chain (159 Hz, 308 Hz, above T1's range): with f_r = 50 Hz every mode is rigid and "
                     "the response equals the static response to the rigid acceleration; without the "
                     "correction the request is refused (periodic modes outside the tabulated range)",
          "rigid_frequencies": [50.0]},
}

QUANTS = ["u1_m", "u2_m", "spring1_force_N", "spring2_force_N", "ground_reaction_N"]


def exact_sqrt(q):
    q = Fraction(q)
    if q < 0:
        return None
    n, d = math.isqrt(q.numerator), math.isqrt(q.denominator)
    if n * n == q.numerator and d * d == q.denominator:
        return Fraction(n, d)
    return None


def modal(p):
    m1, m2, k1, k2 = (Fraction(p[x]) for x in ("m1", "m2", "k1", "k2"))
    a, b, c = m1 * m2, m1 * k2 + m2 * (k1 + k2), k1 * k2
    disc = b * b - 4 * a * c
    rt = exact_sqrt(disc)
    if rt is not None:
        lams = [(b - rt) / (2 * a), (b + rt) / (2 * a)]
    else:
        r = dsqrt(dec(disc))
        lams = [(dec(b) - r) / (2 * dec(a)), (dec(b) + r) / (2 * dec(a))]
    out = []
    for lam in lams:
        L = Fraction if isinstance(lam, Fraction) else dec
        phi = [(L(k2) - lam * L(m2)) / L(k2), L(1)]
        alt = [L(1), ((L(k1) + L(k2)) - lam * L(m1)) / L(k2)]  # first row
        big = max(phi, key=abs)
        assert abs(abs(phi[0]) - abs(phi[1])) > 0, "normalization tie"
        phi = [x / big for x in phi]
        bigalt = max(alt, key=abs)
        alt = [x / bigalt for x in alt]
        gm = L(m1) * phi[0] ** 2 + L(m2) * phi[1] ** 2
        Lk = L(m1) * phi[0] + L(m2) * phi[1]
        gamma = Lk / gm
        meff = Lk * Lk / gm
        out.append({"lam": lam, "phi": phi, "phi_alt": alt, "gm": gm, "L": Lk, "gamma": gamma, "meff": meff})
    return out, (m1, m2, k1, k2), (a, b, c)


def D(x):
    return x if isinstance(x, Decimal) else dec(x)


def div(a, b):
    if isinstance(a, Fraction) and isinstance(b, Fraction):
        return a / b
    return D(a) / D(b)


def quantities(u, k1, k2):
    u1, u2 = D(u[0]), D(u[1])
    return [u1, u2, D(k1) * u1, D(k2) * (u2 - u1), -D(k1) * u1]


def build(cross):
    sets_doc = {}
    worst = {"shape": Decimal(0), "balance": Decimal(0), "sum_meff": Decimal(0), "viete": Decimal(0),
             "rigid_modal": Decimal(0), "rigid_balance": Decimal(0), "static": Decimal(0)}
    prop_checks = []
    for sid, p in SETS.items():
        modes, (m1, m2, k1, k2), (a, b, c) = modal(p)
        T = m1 + m2
        lam1, lam2 = D(modes[0]["lam"]), D(modes[1]["lam"])
        worst["viete"] = max(worst["viete"], rel_diff(lam1 * lam2, D(Fraction(c) / a)),
                             rel_diff(lam1 + lam2, D(Fraction(b) / a)))
        worst["sum_meff"] = max(worst["sum_meff"], rel_diff(D(modes[0]["meff"]) + D(modes[1]["meff"]), D(T)))
        mode_docs, omegas, freqs, per_unit = [], [], [], []
        for i, md in enumerate(modes):
            worst["shape"] = max(worst["shape"], *(rel_diff(D(x), D(y)) for x, y in zip(md["phi"], md["phi_alt"])))
            w = dsqrt(D(md["lam"]))
            f = w / (2 * pi())
            omegas.append(w)
            freqs.append(f)
            g = [D(md["gamma"]) * D(x) / D(md["lam"]) for x in md["phi"]]  # displacement per unit spectral accel
            q = quantities(g, k1, k2)
            per_unit.append(q)
            # balance: ground reaction per unit spectral acceleration = -effective mass
            worst["balance"] = max(worst["balance"], rel_diff(q[4], -D(md["meff"])))
            mode_docs.append({
                "mode": i + 1, "omega_squared": num(md["lam"]), "omega_rad_per_s": val(w),
                "frequency_hz": val(f), "shape_u1_u2": [num(x) for x in md["phi"]],
                "generalized_mass_kg": num(md["gm"]), "participation_factor": num(md["gamma"]),
                "effective_mass_kg": num(md["meff"]), "effective_mass_fraction": num(div(md["meff"], T)),
                "per_unit_spectral_acceleration": {k: val(v) for k, v in zip(QUANTS, q)},
                "in_range_of_T1": rules.evaluate(TABLE, f) is not None,
            })
        doc = {"purpose": p["purpose"],
               "inputs": {k: inp(p[k]) for k in ("m1", "m2", "k1", "k2")},
               "total_mass_kg": num(T),
               "characteristic_coefficients": {"a=m1m2": num(a), "b=m1k2+m2(k1+k2)": num(b), "c=k1k2": num(c)},
               "modes": mode_docs,
               "groups_10pct_of_lowest": [[i + 1 for i in g] for g in rules.groups(freqs)],
               "ten_percent_pairs": [[i + 1, j + 1] for i, j in rules.close_pairs(freqs)]}
        S = [rules.evaluate(TABLE, f) for f in freqs]
        if all(s is not None for s in S):
            sS = [dec(SCALE) * D(s) for s in S]
            contrib = [[per_unit[i][k] * sS[i] for i in range(2)] for k in range(len(QUANTS))]
            doc["spectral_ordinates"] = [{"mode": i + 1, "S_m_per_s2": val(D(S[i])), "sS_m_per_s2": val(sS[i])}
                                         for i in range(2)]
            doc["without_missing_mass"] = {
                "periodic_modes": [1, 2],
                "signed_contributions": {QUANTS[k]: [val(v) for v in contrib[k]] for k in range(len(QUANTS))},
                "combined": {QUANTS[k]: {r: val(v) for r, v in
                                         rules.combine_all(contrib[k], freqs, omegas, ZETA).items()}
                             for k in range(len(QUANTS))},
                "disclosure": {"periodic_effective_mass_fraction": num(div(modes[0]["meff"] + modes[1]["meff"], T)),
                    "missing_mass_fraction": "0 (the two modes are the whole finite spectrum)"},
            }
            if sid == "B":
                cb = rules.combine_all(contrib[3], freqs, omegas, ZETA)
                opp = (contrib[3][0] < 0) != (contrib[3][1] < 0)
                prop_checks.append({"id": "CHAIN-B-opposite-sign-CQC-below-SRSS",
                                    "what": "spring 2 force: contributions of opposite sign and CQC < SRSS",
                                    "agrees": opp and cb["CQC"] < cb["SRSS"]})
                for k in (0, 4):
                    ck = rules.combine_all(contrib[k], freqs, omegas, ZETA)
                    same = (contrib[k][0] < 0) == (contrib[k][1] < 0)
                    prop_checks.append({"id": f"CHAIN-B-same-sign-CQC-above-SRSS-{QUANTS[k]}",
                                        "what": f"{QUANTS[k]}: contributions of the same sign and CQC > SRSS",
                                        "agrees": same and ck["CQC"] > ck["SRSS"]})
                doc["rho_12"] = val(rules.rho(omegas[0], omegas[1], ZETA))
        else:
            doc["without_missing_mass"] = {
                "expected": "refused: periodic modes outside the tabulated range of T1 (no extrapolation)",
                "modes_outside_range": [i + 1 for i in range(2) if S[i] is None]}
        # missing mass
        mm = []
        Kmat = [[D(k1 + k2), D(-k2)], [D(-k2), D(k2)]]
        for fr in p["rigid_frequencies"]:
            frq = Fraction(fr)
            ar_S = rules.evaluate(TABLE, frq)
            assert ar_S is not None
            a_r = dec(SCALE) * D(ar_S)
            per = [i for i in range(2) if freqs[i] < D(frq)]
            rig = [i for i in range(2) if i not in per]
            # static load per unit rigid acceleration, exact when the modal data are exact
            Fu = [m1, m2]
            for i in per:
                Fu = [Fu[j] - modes[i]["gamma"] * [m1, m2][j] * modes[i]["phi"][j] for j in range(2)]
            if all(isinstance(x, Fraction) for x in Fu):
                uu = solve([[k1 + k2, -k2], [-k2, k2]], Fu)
            else:
                uu = solve(Kmat, [D(x) for x in Fu])
            F = [a_r * D(x) for x in Fu]
            u = [a_r * D(x) for x in uu]
            qr = quantities(u, k1, k2)
            # cross-checks: rigid response from the rigid modes' signed static contributions, and balance
            alt = [a_r * sum((per_unit[i][k] for i in rig), Decimal(0)) for k in range(len(QUANTS))]
            worst["rigid_modal"] = max(worst["rigid_modal"],
                                       *[rel_diff(x, y) for x, y in zip(qr, alt)])
            worst["rigid_balance"] = max(worst["rigid_balance"], rel_diff(qr[4], -(F[0] + F[1])) if qr[4] != 0 else 0)
            entry = {"rigid_frequency_hz": inp(fr), "rigid_acceleration_a_r_m_per_s2": val(a_r),
                     "periodic_modes": [i + 1 for i in per], "rigid_modes_extracted": [i + 1 for i in rig],
                     "static_load_F_N": [val(x) for x in F],
                     "rigid_response_signed": {QUANTS[k]: val(qr[k]) for k in range(len(QUANTS))}}
            pf = sum((D(modes[i]["meff"]) for i in per), Decimal(0)) / D(T)
            entry["disclosure"] = {"periodic_effective_mass_fraction": val(pf),
                                   "fraction_not_represented_by_periodic_modes": val(1 - pf)}
            if not per:
                # stiff case: equals the static response to the rigid acceleration
                u1s = a_r * D(T) / D(k1)
                u2s = u1s + a_r * D(m2) / D(k2)
                worst["static"] = max(worst["static"], rel_diff(u[0], u1s), rel_diff(u[1], u2s))
                entry["static_response_to_rigid_acceleration"] = {
                    "u1 = a_r (m1 + m2)/k1": val(u1s), "u2 = u1 + a_r m2/k2": val(u2s)}
            if all(S[i] is not None for i in per):
                sS = {i: dec(SCALE) * D(S[i]) for i in per}
                tot = {}
                for k in range(len(QUANTS)):
                    vals = [per_unit[i][k] * sS[i] for i in per]
                    pc = rules.combine_all(vals, [freqs[i] for i in per], [omegas[i] for i in per], ZETA) if per \
                        else {r: Decimal(0) for r in rules.MODAL_RULES}
                    tot[QUANTS[k]] = {r: {"periodic": val(pc[r]),
                                          "with_rigid_SRSS": val(rules.with_rigid(pc[r], qr[k])["SRSS"]),
                                          "with_rigid_ABSSUM": val(rules.with_rigid(pc[r], qr[k])["ABSSUM"])}
                                      for r in rules.MODAL_RULES}
                entry["combined"] = tot
            mm.append(entry)
        if mm:
            doc["missing_mass"] = mm
        sets_doc[sid] = doc
    lim = {"shape": "1e-80", "balance": "1e-80", "sum_meff": "1e-80", "viete": "1e-80", "rigid_modal": "1e-80",
           "rigid_balance": "1e-80", "static": "1e-80"}
    what = {"shape": "shape from the second row against the first row",
            "balance": "per-mode ground reaction per unit spectral acceleration against minus the effective mass",
            "sum_meff": "sum of effective masses against the total mass",
            "viete": "lam1 lam2 = c/a and lam1 + lam2 = b/a",
            "rigid_modal": "static rigid solve against the rigid modes' signed contributions a_r Gamma phi / w^2",
            "rigid_balance": "rigid ground reaction against minus the total static load",
            "static": "stiff case against the hand static solution"}
    for k in worst:
        cross.append({"id": f"CHAIN-{k}", "what": what[k], "max_rel_diff": sci(worst[k]),
                      "agrees": worst[k] < Decimal(lim[k])})
    cross.extend(prop_checks)
    return {
        "item": "3. Two-mass lumped chain (AC-002: MDOF modal responses and each modal combination, CQC on a closely "
                "spaced pair with opposite-sign contributions, the rigid response with an extracted rigid mode, "
                "and a stiff model equal to the static response)",
        "spectrum": {"table": "T1 of spectrum_tables.json under log-abscissa/log-ordinate",
                     "scale_factor_s": inp(SCALE), "damping_ratio_for_CQC": inp(ZETA)},
        "sign_conventions": "u positive along the excitation axis away from the ground; spring forces tension "
                            "positive; ground reaction is K_cf u, the force the support applies to the chain",
        "rules": "SRSS, CQC, GROUPING, TEN_PERCENT (the ten percent method) and ABSSUM (the absolute sum): the five "
                 "modal rules of DEL-04-10 CLM-001; no mode set here contains a repeated-mode cluster",
        "sets": sets_doc,
    }
