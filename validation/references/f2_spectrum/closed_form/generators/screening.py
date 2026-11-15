"""screening_basis.json: the largest relative deviations of item 5's FE results from the
Rayleigh continuum, per response kind, mass formulation and N, and item 6's largest
sensitivities. Data only: no screening class value is proposed here."""

from decimal import Decimal

import beams as B
import beams_build as BB
import rules
from numerics import sci

IN_SCOPE = ("SRSS", "CQC", "GROUPING")
KINDS = ["displacement", "force", "moment", "reaction"]


def build(cross):
    c = BB.compute()
    real = c["real"]
    cats = ("frequency", "per_mode", "per_mode_significant", "combined", "missing_mass_total",
            "missing_mass_total_same_classification")
    best = {}

    def put(cat, kind, form, N, L, d, where):
        if d is None:
            return
        for key in ((cat, kind, form, N, "all"), (cat, kind, form, N, L)):
            if abs(d) > best.get(key, (Decimal(-1),))[0]:
                best[key] = (abs(d), where)

    for rid, r in real.items():
        ry = r["cont"]["rayleigh"]
        L = r["L"]
        for form in B.FORMS:
            for N in B.NS:
                res = r["fe"][(form, N)]
                nm = res["nm"]
                kinds = {q[0]: q[1] for q in B.quantity_list(r["span"], N)}
                for i in range(nm):
                    put("frequency", "frequency", form, N, L, BB.dev(res["freqs"][i], ry.freq[i]), f"{rid} mode {i + 1}")
                for t in r["tables"]:
                    rs, cr = res["resp"][t["id"]], ry.resp[t["id"]]
                    for q in res["qnames"]:
                        sc = max(abs(x) for x in cr["contrib"][q])
                        ccomb = rules.combine_all(cr["contrib"][q][:nm], ry.freq[:nm], ry.omega[:nm], B.ZETA)
                        for i in range(nm):
                            cv = cr["contrib"][q][i]
                            if abs(cv) > Decimal("1e-30") * sc:
                                d = BB.dev(rs["contrib"][q][i], cv)
                                w = f"{rid} {t['id']} {q} mode {i + 1}"
                                put("per_mode", kinds[q], form, N, L, d, w)
                                if abs(cv) >= ccomb["SRSS"] / 100:
                                    put("per_mode_significant", kinds[q], form, N, L, d, w)
                        for rr in IN_SCOPE:
                            put("combined", kinds[q], form, N, L, BB.dev(rs["comb"][q][rr], ccomb[rr]),
                                f"{rid} {t['id']} {q} {rr} over {nm} modes")
                        for e, ce in zip(rs["mm"], cr["mm"]):
                            for rr in ("SRSS", "CQC"):
                                for k in ("SRSS", "ABSSUM"):
                                    d = BB.dev(e["tot"][q][rr][k], ce["tot"][q][rr][k])
                                    w = f"{rid} {t['id']} {q} truncation {e['n']} {rr}+{k}"
                                    put("missing_mass_total", kinds[q], form, N, L, d, w)
                                    if len(e["per"]) == len(ce["per"]):
                                        put("missing_mass_total_same_classification", kinds[q], form, N, L, d, w)
    table = {}
    for cat in cats:
        ks = ["frequency"] if cat == "frequency" else KINDS
        table[cat] = {}
        for kind in ks:
            table[cat][kind] = {}
            for form in B.FORMS:
                table[cat][kind][form] = {}
                for N in B.NS:
                    row = {}
                    for Lk, lab in (("all", "all"), (2.0, "L2"), (64.0, "L64")):
                        v = best.get((cat, kind, form, N, Lk))
                        row[lab] = None if v is None else sci(v[0], 3)
                        if Lk == "all" and v is not None:
                            row["where"] = v[1]
                    table[cat][kind][form][str(N)] = row
    sens = c.get("sens_largest", {})
    out_sens = {}
    for (kind, spec), v in sorted(sens.items()):
        out_sens.setdefault(kind, {})[spec] = {"max_abs_dlnR_dlnf": sci(v[0], 6),
                                               "where": f"{v[1]} {v[2]} mode {v[3]}"}
    ok = all(table["per_mode"][k]["consistent"]["16"]["all"] is not None for k in KINDS)
    cross.append({"id": "SCREEN-complete", "what": "every response kind has a consistent N = 16 deviation", "agrees": ok})
    return {
        "item": "Screening basis: data from items 5 and 6 for the spectrum screening class. No class value is proposed.",
        "definitions": {
            "deviation": "|FE - continuum| / |continuum| against the Rayleigh continuum (the product's continuum "
                         "limit), maximum over the cantilever, propped cantilever, pinned-pinned and fixed-fixed "
                         "spans, both lengths unless split, both spectra and every quantity of the kind",
            "kinds": {"displacement": "tip or midspan displacement", "force": "internal shear at L/4 and L/2",
                      "moment": "internal bending moment at L/4 and L/2", "reaction": "end reaction forces and moments"},
            "categories": {
                "frequency": "first five (or all finite) modes",
                "per_mode": "signed modal contributions whose continuum value exceeds 1e-30 times the largest "
                            "absolute continuum contribution of that quantity over the five continuum modes (the rest, "
                            "zero by symmetry or rounding residue, are excluded); dominated by small "
                            "contributions near a zero of the quantity's modal distribution",
                "per_mode_significant": "the same, restricted to contributions at least 1 % of the quantity's "
                                        "continuum SRSS over nm = min(5, the FE model's finite mode count) continuum "
                                        "modes (not always five)",
                "combined": "SRSS, CQC and grouping over the first five (or all finite) modes, against the "
                            "continuum combined over the same number of modes",
                "missing_mass_total": "periodic part (SRSS or CQC) with the rigid part (SRSS or absolute sum), "
                                      "truncation after 1, 2, 3 and 5 modes (includes cases where the FE mesh "
                                      "classifies a different number of modes as periodic, because an FE "
                                      "frequency lies on the other side of f_r = 1.01 f_n)",
                "missing_mass_total_same_classification": "the same, restricted to cases with the continuum's "
                                                          "periodic count"},
            "lengths": {"L2": B.LEN_NOTE[2.0], "L64": B.LEN_NOTE[64.0]},
            "lumped": "NOT the product's formulation; measures the size of that modelling difference",
            "for_the_screening_derivation": {
                "sensitivity": "largest_sensitivity is a conditional SRSS sensitivity: one mode's frequency varies "
                               "alone, with its shape, its participation and its force-recovery coefficient q_n held "
                               "fixed; it excludes changes of the CQC coefficient, of the rigid response, of the shape "
                               "and recovery, and rotations",
                "coarse_mesh_classification": "on a coarse mesh the same rigid frequency f_r can classify a different "
                                              "number of modes as periodic; missing_mass_total includes such cases, "
                                              "missing_mass_total_same_classification excludes them",
                "left_out_of_the_maxima": "the quarter-point quantities (M_quarter_Nm, V_quarter_N) at N = 2, where "
                                          "L/4 is not a node; and quantities whose continuum value is zero by "
                                          "symmetry (per-mode values of zero-participation modes; V_mid_N and the "
                                          "moment reaction at a pin, which are not published for the symmetric "
                                          "spans and the pinned end)",
                "L64_negligible_applies_to_modes_1_to_3": "at L = 64 m rotary inertia shifts modes 4 and 5 by "
                                                          "6.6e-7 to 1.23e-6 (mode 5 above 1e-6 in every span); the "
                                                          "five-mode categories include them"},
        },
        "largest_relative_deviation": table,
        "largest_sensitivity": {"definition": "max |d ln R_SRSS / d ln f_n| over realizations, quantities and modes "
                                              "(sensitivity.json), a conditional SRSS sensitivity with shape, "
                                              "participation and force-recovery coefficient held fixed; under that "
                                              "premise a relative frequency disagreement e moves R by about e times "
                                              "this", "by_kind_and_spectrum": out_sens},
    }
