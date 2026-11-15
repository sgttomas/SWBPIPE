"""Assembly of item 5 (beams_continuum.json, beams_fe.json) and the data items 6 and the
screening summary draw on. Results are computed once and cached for the run."""

from decimal import Decimal
from fractions import Fraction as Fr

import beams as B
import continuum as C
import rules
from numerics import dec, dsqrt, inp, num, pi, rel_diff, sci, val

_CACHE = {}


def dev(a, c):
    """Signed relative deviation (a - c)/|c|; None where the continuum value is zero."""
    if c == 0:
        return None
    return (a - c) / abs(c)


def dstr(d):
    return None if d is None else sci(d, 6)


def compute():
    if _CACHE:
        return _CACHE
    sec = B.section()
    acc = {k: Decimal(0) for k in ("quad", "force_balance", "moment_balance", "reaction_meff", "eb_equation",
                                   "pp_determinant", "static_textbook", "sturm", "residual", "fe_balance",
                                   "fe_rigid_modal", "fe_rigid_balance")}
    acc["gamma_noise"] = [Decimal(0)]
    acc["rod_counts_ok"] = True
    acc["rigid_frequency_classification_ok"] = True
    real = {}
    for L in B.LENGTHS:
        tables = [B.FLAT, B.two_segment(L)]
        for span in B.SPAN_ORDER:
            rid = f"{span}-L{int(L)}"
            qn = [q[0] for q in B.quantity_list(span)]
            r = {"span": span, "L": L, "tables": tables, "qnames": qn}
            r["cont"] = {m: B.continuum_case(span, L, sec, m, acc) for m in ("eb", "rayleigh")}
            static, exact, poly = B.static_quantities(sec, L, span)
            r["static"], r["static_exact"], r["static_poly"] = static, exact, poly
            for k, coef in B.TEXTBOOK[span].items():
                acc["static_textbook"] = max(acc["static_textbook"], abs(dec(exact[k] - coef)))
            r["fr"] = B.truncation_frequencies(r["cont"]["rayleigh"])
            for n, f_r in r["fr"].items():
                for m in ("eb", "rayleigh"):
                    cc = r["cont"][m]
                    per = [i for i in range(B.N_MODES) if cc.freq[i] < dec(f_r)]
                    acc["rigid_frequency_classification_ok"] &= (len(per) == n)
            # continuum responses
            for m, cc in r["cont"].items():
                cc.resp = {}
                for t in tables:
                    S, contrib, comb = B.respond(cc.freq, cc.omega, cc.unit, t, qn, B.N_MODES)
                    mm = []
                    for n, f_r in r["fr"].items():
                        a_r, per, rig = B.rigid_continuum(cc, static, f_r, t, qn)
                        tot = {}
                        for q in qn:
                            pc = rules.combine_all([contrib[q][i] for i in per], [cc.freq[i] for i in per],
                                                   [cc.omega[i] for i in per], B.ZETA)
                            tot[q] = {rr: {"periodic": pc[rr], **rules.with_rigid(pc[rr], rig[q])}
                                      for rr in ("SRSS", "CQC", "GROUPING")}
                        mm.append({"n": n, "f_r": f_r, "a_r": a_r, "per": per, "rig": rig, "tot": tot})
                    cc.resp[t["id"]] = {"S": S, "contrib": contrib, "comb": comb, "mm": mm}
            # finite elements
            r["fe"] = {}
            for form in B.FORMS:
                for N in B.NS:
                    md, ms, info, units, freqs, omegas, mats = B.fe_case(span, N, L, sec, form, acc)
                    nm = min(B.N_MODES, len(ms))
                    qf = [q[0] for q in B.quantity_list(span, N)]
                    res = {"md": md, "ms": ms, "info": info, "units": units, "freqs": freqs, "omegas": omegas,
                           "nm": nm, "qnames": qf, "resp": {}}
                    for t in tables:
                        S, contrib, comb = B.respond(freqs, omegas, units, t, qf, nm)
                        mm = []
                        for n, f_r in r["fr"].items():
                            a_r, per, rig = B.fe_rigid(md, ms, info, freqs, mats, sec, span, N, f_r, t, acc)
                            Sall = {}
                            for i in per:
                                s = rules.evaluate(t, freqs[i])
                                assert s is not None
                                Sall[i] = dec(B.SCALE) * s
                            tot = {}
                            for q in qf:
                                vals = [units[i][q] * Sall[i] for i in per]
                                pc = rules.combine_all(vals, [freqs[i] for i in per], [omegas[i] for i in per],
                                                       B.ZETA)
                                tot[q] = {rr: {"periodic": pc[rr], **rules.with_rigid(pc[rr], rig[q])}
                                          for rr in ("SRSS", "CQC")}
                            mm.append({"n": n, "f_r": f_r, "a_r": a_r, "per": per, "rig": rig, "tot": tot})
                        res["resp"][t["id"]] = {"S": S, "contrib": contrib, "comb": comb, "mm": mm}
                    r["fe"][(form, N)] = res
            real[rid] = r
    _CACHE.update({"sec": sec, "acc": acc, "real": real})
    return _CACHE


# --------------------------------------------------------------------------


def _resp_doc(cc, resp, qn):
    out = {"spectral_ordinates_sS": [val(s) for s in resp["S"]],
           "signed_contributions": {q: [val(v) for v in resp["contrib"][q]] for q in qn},
           "combined_first_5": {q: {rr: val(v) for rr, v in resp["comb"][q].items()} for q in qn},
           "missing_mass": []}
    for e in resp["mm"]:
        pf = sum((cc.meff_frac[i] for i in e["per"]), Decimal(0))
        out["missing_mass"].append({
            "truncation_after_mode": e["n"], "rigid_frequency_hz": inp(e["f_r"]),
            "rigid_acceleration_m_per_s2": val(e["a_r"]), "periodic_modes": [i + 1 for i in e["per"]],
            "disclosure": {"periodic_effective_mass_fraction": val(pf),
                           "fraction_not_represented": val(1 - pf)},
            "rigid_response_signed": {q: val(v) for q, v in e["rig"].items()},
            "combined": {q: {rr: {k: val(x) for k, x in d.items()} for rr, d in e["tot"][q].items()} for q in qn},
        })
    return out


def build_continuum(cross):
    c = compute()
    sec, acc, real = c["sec"], c["acc"], c["real"]
    reals = []
    for rid, r in real.items():
        span, L = r["span"], r["L"]
        left, right, desc = B.SPANS[span]
        eb, ry = r["cont"]["eb"], r["cont"]["rayleigh"]
        doc = {"id": rid, "span": desc, "ends": {"x=0": left, "x=L": right}, "length_m": inp(L),
               "slenderness_note": B.LEN_NOTE[L], "r2 = ibar/(mbar L^2)": num(ry.r2),
               "rotary_inertia_relative_frequency_shift": [sci((ry.freq[i] - eb.freq[i]) / eb.freq[i], 6)
                                                           for i in range(B.N_MODES)],
               "static_per_unit_acceleration": {
                   "w_hat_polynomial_c0_to_c4 (w = (mbar L^4/EI) w_hat, w_hat'''' = 1)": [num(x) for x in r["static_poly"]],
                   "values": {q: {"coefficient": num(r["static_exact"][q]), "value": val(r["static"][q])}
                              for q in r["qnames"]}},
               "rigid_frequencies_hz": {str(n): inp(f) for n, f in r["fr"].items()},
               "models": {}}
        for mname, cc in (("euler_bernoulli", eb), ("rayleigh", ry)):
            doc["models"][mname] = {
                "modes": [B.mode_doc(m, cc, i, span) for i, m in enumerate(cc.modes)],
                "cumulative_effective_mass_fraction_5": val(sum(cc.meff_frac, Decimal(0))),
                "responses": {tid: _resp_doc(cc, resp, r["qnames"]) for tid, resp in cc.resp.items()},
            }
        reals.append(doc)
    lim = Decimal("1e-40")
    checks = [("quad", "closed-form integrals of W, W^2, W'^2 against Gauss-Legendre quadrature", lim),
              ("force_balance", "per mode: F(0) + F(L) + Gamma mbar L int W = 0", lim),
              ("moment_balance", "per mode: moments about x = 0 of reactions, transverse and rotary inertia", lim),
              ("reaction_meff", "per mode: total transverse reaction per unit S = -effective mass", lim),
              ("eb_equation", "Euler-Bernoulli roots: determinant route against the textbook frequency "
                              "equations (cos cosh = -1, tan = tanh, cos cosh = 1)", Decimal("1e-60")),
              ("pp_determinant", "pinned-pinned closed form against the determinant route (both models)",
               Decimal("1e-60")),
              ("static_textbook", "static coefficients against the textbook values qL^4/8EI, qL^4/192EI, "
                                  "5qL^4/384EI, qL^4/384EI and the end reactions", Decimal(0))]
    for k, what, tol in checks:
        cross.append({"id": f"BEAM-{k}", "what": what, "max": sci(acc[k]), "agrees": acc[k] <= tol})
    cross.append({"id": "BEAM-rigid-frequency-classification",
                  "what": "each truncation's f_r leaves exactly n periodic continuum modes in both models",
                  "agrees": acc["rigid_frequency_classification_ok"]})
    slender_ok = True
    stocky_ok = True
    for rid, r in real.items():
        eb, ry = r["cont"]["eb"], r["cont"]["rayleigh"]
        sh = max(abs((ry.freq[i] - eb.freq[i]) / eb.freq[i]) for i in range(3))
        if r["L"] == 64.0:
            slender_ok &= sh < Decimal("1e-6")
        else:
            stocky_ok &= sh > Decimal("1e-6")
    cross.append({"id": "BEAM-slenderness-choice", "what": "L = 64 m: rotary shift of modes 1-3 below 1e-6 in every "
                  "span; L = 2 m: above 1e-6 in every span", "agrees": slender_ok and stocky_ok})
    return {
        "item": "5. Benchmark family, continuum (cantilever, propped cantilever, pinned-pinned, fixed-fixed)",
        "planar": "Each case is one bending plane (equivalently, the pipe is restrained out of that plane). A "
                  "circular pipe free in three dimensions has exactly repeated bending pairs whose basis is "
                  "arbitrary: applied mode by mode, per-mode contributions and SRSS are then basis dependent; under "
                  "the scope's repeated-mode rule (DEL-04-10 CLM-001, Repeated modes: signed sum within the cluster "
                  "first) every rule is basis invariant (beam_skew_and_pairs.json, repeated_pair).",
        "section": sec["doc"],
        "spectra": [rules.table_doc(t) for t in (B.FLAT, B.two_segment(2.0), B.two_segment(64.0))],
        "spectrum_use": "FLAT for every realization; TWO_SEGMENT_L2 for L = 2 m and TWO_SEGMENT_L64 for L = 64 m; "
                        "scale factor s = 1, damping ratio 0.05 (CQC)",
        "scale_factor_s": inp(B.SCALE),
        "quantities": {
            "w_tip_m / w_mid_m": "displacement relative to the supports at the tip (C) or midspan, +y",
            "F0_N, FL_N": "reaction force the support applies to the pipe at x = 0 / x = L, +y",
            "M0_Nm": "reaction moment the support applies at x = 0, counterclockwise (conjugate to theta = dw/dx)",
            "M_mid_Nm, M_quarter_Nm": "internal bending moment EI w'' at x = L/2, L/4 (moment on the left part at "
                                      "the cut)",
            "V_quarter_N, V_mid_N": "internal shear at x = L/4, L/2: force on the left part at the cut, "
                                    "-(EI w''' + w^2 ibar w')",
        },
        "rules": "SRSS, CQC, GROUPING, ABSSUM (the absolute sum) and TEN_PERCENT (the ten percent method): the five "
                 "modal rules of DEL-04-10 CLM-001; no mode set here contains a repeated-mode cluster",
        "realizations": reals,
    }


def build_fe(cross):
    c = compute()
    sec, acc, real = c["sec"], c["acc"], c["real"]
    reals = []
    for rid, r in real.items():
        span, L = r["span"], r["L"]
        ry, eb = r["cont"]["rayleigh"], r["cont"]["eb"]
        doc = {"id": rid, "forms": {}}
        for form in B.FORMS:
            fd = {}
            for N in B.NS:
                res = r["fe"][(form, N)]
                nm, qf = res["nm"], res["qnames"]
                modes = []
                for i in range(nm):
                    m, inf = res["ms"][i], res["info"][i]
                    modes.append({"mode": i + 1, "frequency_hz": val(res["freqs"][i]),
                                  "rel_dev_from_rayleigh": dstr(dev(res["freqs"][i], ry.freq[i])),
                                  "rel_dev_from_euler_bernoulli": dstr(dev(res["freqs"][i], eb.freq[i])),
                                  "normalization_case": m["case"], "equal_maxima_of_opposite_sign": m["tie"],
                                  "participation_factor": val(inf["gamma"]),
                                  "effective_mass_fraction": val(inf["meff_frac"])})
                nd = {"finite_modes": len(res["ms"]), "modes": modes, "responses": {}}
                for t in r["tables"]:
                    rs = res["resp"][t["id"]]
                    cr = ry.resp[t["id"]]
                    # continuum combinations over the same number of modes
                    ccomb = {q: rules.combine_all(cr["contrib"][q][:nm], ry.freq[:nm], ry.omega[:nm], B.ZETA)
                             for q in qf}
                    def trip(v, c):
                        return [val(v)["value"], val(v)["binary64"], dstr(dev(v, c))]

                    def trip0(v, c, scale):
                        d = dev(v, c) if abs(c) > Decimal("1e-30") * scale else None
                        return [val(v)["value"], val(v)["binary64"], dstr(d)]

                    out = {"per_mode": {q: [trip0(rs["contrib"][q][i], cr["contrib"][q][i],
                                                  max(abs(x) for x in cr["contrib"][q])) for i in range(nm)]
                                        for q in qf},
                           "combined_first_modes": {q: {rr: trip(v, ccomb[q][rr]) for rr, v in rs["comb"][q].items()}
                                                    for q in qf},
                           "missing_mass": []}
                    for e, ce in zip(rs["mm"], cr["mm"]):
                        comb = {}
                        for q in qf:
                            row = {"rigid_signed": trip(e["rig"][q], ce["rig"][q])}
                            for rr in ("SRSS", "CQC"):
                                for k in ("SRSS", "ABSSUM"):
                                    row[f"{rr}_periodic_{k}_rigid"] = trip(e["tot"][q][rr][k], ce["tot"][q][rr][k])
                            comb[q] = row
                        out["missing_mass"].append({
                            "truncation_after_mode": e["n"], "rigid_frequency_hz": inp(e["f_r"]),
                            "periodic_modes": [i + 1 for i in e["per"]],
                            "periodic_count_differs_from_continuum": len(e["per"]) != len(ce["per"]),
                            "values": comb})
                    nd["responses"][t["id"]] = out
                fd[str(N)] = nd
            doc["forms"][form] = fd
        reals.append(doc)
    # merged list of the benchmark model (C in Y, P in Z), per L
    merged = []
    list_rule_ok = True

    def cutoff_of(Y, Z):
        return max(Y[min(B.N_MODES, len(Y)) - 1], Z[min(B.N_MODES, len(Z)) - 1])

    for L in B.LENGTHS:
        rc, rp = real[f"C-L{int(L)}"], real[f"P-L{int(L)}"]
        entry = {"length_m": inp(L), "continuum": {}, "fe": {}}
        for m in ("eb", "rayleigh"):
            ax = B.rod_freqs_continuum(L, sec["EA_m"], 3)
            to = B.rod_freqs_continuum(L, sec["GJ_jbar"], 3)
            entry["continuum"][m] = B.merged_doc(rc["cont"][m].freq, rp["cont"][m].freq, ax, to)
            # list rule: the first unbuilt mode of every family lies above the cutoff
            cut = cutoff_of(rc["cont"][m].freq, rp["cont"][m].freq)
            r2 = rc["cont"][m].r2
            hz = dsqrt(dec(sec["EI_m"])) / dec(L) ** 2 / (2 * pi())
            for left, right in (("clamped", "free"), ("clamped", "pinned")):
                om6 = C.roots(left, right, r2, 0, B.N_MODES + 1)[B.N_MODES]
                list_rule_ok &= om6 * hz > cut
            list_rule_ok &= B.rod_freqs_continuum(L, sec["EA_m"], 4)[3] > cut
            list_rule_ok &= B.rod_freqs_continuum(L, sec["GJ_jbar"], 4)[3] > cut
        for form in B.FORMS:
            for N in B.NS:
                ax = B.rod_freqs_fe(L, sec["EA_m"], N, form, 3, acc)
                to = B.rod_freqs_fe(L, sec["GJ_jbar"], N, form, 3, acc) if form == "consistent" else []
                entry["fe"][f"{form}_N{N}"] = B.merged_doc(rc["fe"][(form, N)]["freqs"], rp["fe"][(form, N)]["freqs"],
                                                           ax, to)
                # bending lists hold every finite FE mode; the rods' first unbuilt mode lies above the cutoff
                cut = cutoff_of(rc["fe"][(form, N)]["freqs"], rp["fe"][(form, N)]["freqs"])
                if N > 3:
                    list_rule_ok &= B.rod_freqs_fe(L, sec["EA_m"], N, form, 4, acc)[3] > cut
                    if form == "consistent":
                        list_rule_ok &= B.rod_freqs_fe(L, sec["GJ_jbar"], N, form, 4, acc)[3] > cut
        merged.append(entry)
    counts_ok = True
    expect = {"consistent": {"C": lambda N: 2 * N, "P": lambda N: 2 * N - 1, "S": lambda N: 2 * N,
                             "F": lambda N: 2 * N - 2},
              "lumped": {"C": lambda N: N, "P": lambda N: N - 1, "S": lambda N: N - 1, "F": lambda N: N - 1}}
    for rid, r in real.items():
        for form in B.FORMS:
            for N in B.NS:
                counts_ok &= len(r["fe"][(form, N)]["ms"]) == expect[form][r["span"]](N)
    cross.append({"id": "FE-mode-counts", "what": "finite_modes of every planar FE model equals the formulation's "
                  "count (consistent: free planar DOFs; lumped: free translations)", "agrees": counts_ok})
    cross.append({"id": "MERGED-list-rule", "what": "benchmark-model merged lists: every family's first mode not "
                  "built (continuum bending mode 6, rod mode 4) lies above the list's cutoff, so the list holds "
                  "every mode the rule names; FE bending lists hold every finite mode", "agrees": list_rule_ok})
    lim = Decimal("1e-35")
    for k, what in (("sturm", "FE eigenvalues: Jacobi against bisection on banded LDL^T Sturm counts (40 digits)"),
                    ("residual", "FE eigenpairs: max |K phi - lam M phi| / max |K phi|"),
                    ("fe_balance", "FE per mode: sum of translational reactions = -effective mass"),
                    ("fe_rigid_modal", "FE rigid displacement: static solve against the sum of the non-periodic "
                                       "modes' Gamma phi / lam (complete FE basis)"),
                    ("fe_rigid_balance", "FE rigid response: translational reactions balance the total static load")):
        tol = Decimal("1e-38") if k == "sturm" else lim
        cross.append({"id": f"FE-{k}", "what": what, "max": sci(acc[k]), "agrees": acc[k] <= tol})
    cross.append({"id": "FE-zero-participation-noise",
                  "what": "modes published with zero participation (antisymmetric): computed |phi^T p| / sqrt(m L)",
                  "max": sci(acc["gamma_noise"][0]), "agrees": acc["gamma_noise"][0] < Decimal("1e-40")})
    from fe import element_checks
    cross.append({"id": "FE-element-matrices", "what": "exact Hermite integrals against the printed stiffness, "
                  "consistent mass (mbar h/420 [156 22h 54 -13h ...]) and rotary inertia (ibar/(30h) [36 3h ...])",
                  "agrees": element_checks()})
    cross.append({"id": "FE-rod-closed-forms", "what": "axial and torsional rod frequencies (closed forms) bracketed "
                  "by Sturm counts of the assembled rod pencils", "agrees": acc["rod_counts_ok"]})
    return {
        "item": "5. Benchmark family, discretized into N = 2, 4, 8, 16 equal members, two mass formulations",
        "formulations": {
            "consistent": "Hermitian consistent mass with Rayleigh rotary inertia: the formulation DEL-04-09 REQ-001 "
                          "states (its linear axial and torsional mass are uncoupled from transverse bending and "
                          "enter only the merged frequency list)",
            "lumped": "NOT the product's formulation: half of each member's mass at each end translation, no rotary "
                      "inertia (no torsional inertia either, so no torsion modes); present only to measure the size "
                      "of that modelling difference"},
        "triplets": "each response entry is [value (40 significant digits), nearest binary64, rel_dev]",
        "deviation_basis": "rel_dev = (FE - continuum)/|continuum| against the Rayleigh continuum (the product's "
                           "continuum limit), mode by mode; frequencies also against Euler-Bernoulli. Combined values "
                           "are compared with the continuum combined over the same number of modes. Deviations are "
                           "diagnostics at 6 digits, not references. null: continuum value zero.",
        "missing_mass_in_fe": "f_r is the same binary64 input as the continuum's; FE modes below it are periodic, so "
                              "a coarse mesh may classify differently (flagged). Totals are published for the SRSS "
                              "and CQC modal rules with both rigid rules; the signed contributions allow any other.",
        "realizations": reals,
        "benchmark_model_merged_frequencies": {
            "model": "one pipe along X, fully fixed at A, only UZ restrained at the tip B. Excited along Y it is the "
                     "cantilever (results in UY, RZ); excited along Z it is the propped cantilever (UZ, RY). Axial (X) "
                     "and torsional (RX) modes are uncoupled and unexcited by transverse motion but count as modes.",
            "list_rule": "every bending mode up to the larger of the two planes' fifth frequency (and the first five "
                         "of each plane), plus axial and torsional modes below that frequency",
            "flag_rule": "Y_Z_pairs_within_1_percent lists any Y-plane and Z-plane frequency within 1 percent of each "
                         "other (relative to the lower)",
            "lengths": merged},
    }
