"""Case C: one pipe along X, fully fixed at A, only UZ restrained at the tip B.

Excited along Y the pipe is a cantilever in the XY plane (UY, RZ); along Z a
propped cantilever in the XZ plane (UZ, RY). The planes are uncoupled, so
each response quantity receives contributions from its own plane's modes
only; the other plane's modes and the axial (X) and torsional (RX) modes
contribute exactly zero. Global signs: UY, UZ, FY, FZ along the axes; MZ is
conjugate to RZ = dUY/dX; MY is conjugate to RY = -dUZ/dX, so MY = -M0 of the
planar XZ problem. Reactions are the forces the supports apply to the pipe.
"""

import json
from decimal import Decimal, localcontext
from fractions import Fraction as Fr

from pinned import c8_bytes, load

N_, R_, C_, FE_, B_ = load()
dec, dsqrt, inp, val, sci, pi, solve, rel_diff = (N_.dec, N_.dsqrt, N_.inp, N_.val, N_.sci, N_.pi, N_.solve,
                                                  N_.rel_diff)

PIPE = {"od": 0.0603, "wall": 0.00391, "length": 15.0, "rho_pipe": 7850.0, "E": 200e9, "G": 78.125e9}
L = PIPE["length"]
NS = [1, 2, 4, 8, 16]
FORMS = ["consistent", "lumped"]
ZETA = 0.05
FLAT = {"id": "FLAT", "abscissa": "frequency", "ordinate": "acceleration", "abscissae": [0.125, 1000.0],
        "ordinates": [2.5, 2.5], "damping": ZETA, "rule": ("linear", "linear")}
TWO = {"id": "TWO", "abscissa": "frequency", "ordinate": "acceleration", "abscissae": [0.125, 2.0, 16.0],
       "ordinates": [1.5, 4.5, 2.25], "damping": ZETA, "rule": ("linear", "linear")}
F_RIGID = 1.25
N_PERIODIC = 6
Y_Q = ["tip_UY_m", "base_FY_N", "base_MZ_Nm"]
Z_Q = ["mid_UZ_m", "base_FZ_N", "base_MY_Nm", "tip_reaction_FZ_N"]
C8_MAP = {"tip_UY_m": "B.uy", "base_FY_N": "SA.Fy", "base_MZ_Nm": "SA.Mz", "mid_UZ_m": "M4.uz",
          "base_FZ_N": "SA.Fz", "base_MY_Nm": "SA.My", "tip_reaction_FZ_N": "SB.Fz"}
RULES3 = ["SRSS", "ABSSUM", "PAIRWISE10"]


def section():
    od, t = Fr(PIPE["od"]), Fr(PIPE["wall"])
    rho, E, G = Fr(PIPE["rho_pipe"]), Fr(PIPE["E"]), Fr(PIPE["G"])
    di = od - 2 * t
    am, im = (od ** 2 - di ** 2) / 4, (od ** 4 - di ** 4) / 64      # A/pi, I/pi
    P = pi()
    return {"EI_m": E * im / (rho * am), "ibar_m": im / am, "EA_m": E / rho, "GJ_jbar": G / rho,
            "mbar": P * dec(rho * am), "EI": P * dec(E * im), "ibar": P * dec(rho * im),
            "doc": {"inputs": {k: inp(v) for k, v in PIPE.items()},
                    "premises": "no contents, no insulation, no allowances; I = pi(OD^4 - ID^4)/64; mbar = rho A; "
                                "ibar = rho I; J = 2I, jbar = rho J",
                    "exact_ratios": {"EI/mbar": N_.num(E * im / (rho * am)), "ibar/mbar": N_.num(im / am),
                                     "EA/mbar": N_.num(E / rho), "GJ/jbar": N_.num(G / rho)},
                    "mbar_kg_per_m": val(P * dec(rho * am)), "EI_N_m2": val(P * dec(E * im)),
                    "total_mass_kg": val(P * dec(rho * am) * dec(L))}}


def S_of(table, f):
    s = R_.evaluate(table, f)
    if s is None:
        raise ValueError(f"periodic mode outside {table['id']}: {f}")
    return s if isinstance(s, Decimal) else dec(s)


def rules3(vals, freqs):
    return {"SRSS": R_.srss(vals), "ABSSUM": R_.abssum(vals), "PAIRWISE10": R_.pairwise10(vals, freqs)}


# --------------------------------------------------------------------------
# continuum


def plane_quantities(span, q):
    """Map the closed_form planar quantities onto the global names."""
    if span == "C":
        return {"tip_UY_m": q["w_tip_m"], "base_FY_N": q["F0_N"], "base_MZ_Nm": q["M0_Nm"]}
    return {"mid_UZ_m": q["w_mid_m"], "base_FZ_N": q["F0_N"], "base_MY_Nm": -q["M0_Nm"],
            "tip_reaction_FZ_N": q["FL_N"]}


def continuum(sec, model, acc):
    out = {}
    r2 = sec["ibar_m"] / Fr(L) ** 2 if model == "rayleigh" else Fr(0)
    sq = dsqrt(dec(sec["EI_m"]))
    for span, (left, right) in (("C", ("clamped", "free")), ("P", ("clamped", "pinned"))):
        ms = C_.modes(left, right, r2, 0, 5)
        modes = []
        for m in ms:
            den = m.I2 + m.r2 * m.I3
            g = m.I1 / den
            w = m.Om * sq / dec(L) ** 2
            q = plane_quantities(span, B_.cont_mode_quantities(m, g, sec, L, span))
            meff = sec["mbar"] * dec(L) * m.I1 ** 2 / den
            tot = q["base_FY_N"] if span == "C" else q["base_FZ_N"] + q["tip_reaction_FZ_N"]
            acc["cont_balance"] = max(acc["cont_balance"], rel_diff(tot, -meff))
            modes.append({"Om": m.Om, "w": w, "f": w / (2 * pi()), "gamma": g, "meff": meff, "unit": q})
        if model == "eb":
            bl = C_.eb_equation_roots("cantilever" if span == "C" else "propped", 5)
            acc["eb_eq"] = max(acc["eb_eq"], *[rel_diff(b * b, m.Om) for b, m in zip(bl, ms)])
        st, ex, _ = B_.static_quantities(sec, L, span)
        for k, coef in B_.TEXTBOOK[span].items():
            acc["static_textbook"] = max(acc["static_textbook"], abs(dec(ex[k] - coef)))
        out[span] = {"modes": modes, "static": plane_quantities(span, st)}
    return out


def merged_cont(cont, sec):
    ent = [(m["f"], "Y", i + 1) for i, m in enumerate(cont["C"]["modes"])] + \
          [(m["f"], "Z", i + 1) for i, m in enumerate(cont["P"]["modes"])] + \
          [(f, "X axial", i + 1) for i, f in enumerate(B_.rod_freqs_continuum(L, sec["EA_m"], 3))] + \
          [(f, "RX torsion", i + 1) for i, f in enumerate(B_.rod_freqs_continuum(L, sec["GJ_jbar"], 3))]
    return sorted(ent, key=lambda e: e[0])


# --------------------------------------------------------------------------
# finite elements


def fe_plane(span, N, form, sec, acc):
    md = FE_.build(span, N, L, sec["EI_m"], sec["ibar_m"], form)
    has_t = any(i % 2 == 0 for i in md.free)
    ms = FE_.solve_modes(md) if (form == "consistent" or has_t) else []
    with localcontext() as ctx:
        ctx.prec = FE_.FE_PREC
        nd = len(md.K)
        Kd = [[dec(x) for x in r] for r in md.K]
        Md = [[dec(x) for x in r] for r in md.M]
        Rd = [dec(x) for x in md.R]
        p = [sum((Md[i][j] * Rd[j] for j in range(nd)), Decimal(0)) for i in md.free]
        mb, Ld = sec["mbar"], dec(L)
        modes = []
        for k, m in enumerate(ms, start=1):
            if k <= 6:
                b = FE_.bisect_eig(md.Kf, md.Mf, k, m["lam"] * Decimal("0.999"), m["lam"] * Decimal("1.001"), 40)
                acc["sturm"] = max(acc["sturm"], rel_diff(b, m["lam"]))
            phi = m["phi"]
            n = len(phi)
            gm = sum((phi[i] * sum((md.Mf[i][j] * phi[j] for j in range(n)), Decimal(0)) for i in range(n)), Decimal(0))
            Lk = sum((a * b for a, b in zip(phi, p)), Decimal(0))
            g = Lk / gm
            u = [Decimal(0)] * nd
            for kk, i in enumerate(md.free):
                u[i] = g * phi[kk] / m["lam"]
            v = [x * m["lam"] for x in u]
            q = quantities(span, N, Kd, Md, u, v, mb)
            meff = Lk * Lk / gm * mb
            tot = q["base_FY_N"] if span == "C" else q["base_FZ_N"] + q["tip_reaction_FZ_N"]
            if meff != 0:
                acc["fe_balance"] = max(acc["fe_balance"], abs(tot + meff) / meff)
            modes.append({"lam": m["lam"], "phi": phi, "f": dsqrt(m["lam"]) / (2 * pi()), "gamma": g, "meff": meff,
                          "case": m["case"], "unit": q})
    return {"md": md, "modes": modes, "mats": (Kd, Md, Rd)}


def quantities(span, N, Kd, Md, u, v, mb):
    nd = len(Kd)

    def nodal(i):
        return mb * sum((Kd[i][j] * u[j] - Md[i][j] * v[j] for j in range(nd)), Decimal(0))

    if span == "C":
        return {"tip_UY_m": u[2 * N], "base_FY_N": nodal(0), "base_MZ_Nm": nodal(1)}
    return {"mid_UZ_m": u[N] if N % 2 == 0 else None, "base_FZ_N": nodal(0), "base_MY_Nm": -nodal(1),
            "tip_reaction_FZ_N": nodal(2 * N)}


def fe_rigid(plane, span, N, f_r, table, sec, acc):
    md, modes = plane["md"], plane["modes"]
    Kd, Md, Rd = plane["mats"]
    nd = len(Kd)
    a_r = S_of(table, Fr(f_r))
    per = [i for i, m in enumerate(modes) if m["f"] < dec(f_r)]
    with localcontext() as ctx:
        ctx.prec = FE_.FE_PREC
        v = Rd[:]
        for i in per:
            for kk, j in enumerate(md.free):
                v[j] -= modes[i]["gamma"] * modes[i]["phi"][kk]
        F = [sum((Md[i][j] * v[j] for j in range(nd)), Decimal(0)) for i in range(nd)]
        uf = solve([[Kd[i][j] for j in md.free] for i in md.free], [F[i] for i in md.free])
        u = [Decimal(0)] * nd
        for kk, i in enumerate(md.free):
            u[i] = uf[kk]
        q = quantities(span, N, Kd, Md, u, v, sec["mbar"])
        rig = [i for i in range(len(modes)) if i not in per]
        alt = [sum((modes[i]["gamma"] * modes[i]["phi"][kk] / modes[i]["lam"] for i in rig), Decimal(0))
               for kk in range(len(md.free))]
        acc["fe_rigid_modal"] = max(acc["fe_rigid_modal"], max(abs(a - b) for a, b in zip(uf, alt)) /
                                    max(abs(x) for x in uf))
        # total reaction check: rigid reactions balance the rigid load on all DOFs, restrained ones included
        T = sec["mbar"] * dec(L)
        pm = sum((modes[i]["meff"] for i in per), Decimal(0))
        tot = q["base_FY_N"] if span == "C" else q["base_FZ_N"] + q["tip_reaction_FZ_N"]
        restrained_mass = sec["mbar"] * sum((Md[c][c] for c in md.fixed if c % 2 == 0), Decimal(0)) \
            if md.form == "lumped" else None
    q = {k: (None if x is None else a_r * x) for k, x in q.items()}
    tot = a_r * tot
    expect = -a_r * (T - pm)
    acc["rigid_total"] = max(acc["rigid_total"], abs(tot - expect) / abs(expect))
    check = {"total_mass_kg": val(T), "periodic_effective_mass_kg": val(pm),
             "sum_of_rigid_reactions_N": val(tot), "minus_a_r_times_(T - periodic_meff)_N": val(expect),
             "relative_residual": sci(abs(tot - expect) / abs(expect))}
    if restrained_mass is not None:
        check["lumped_mass_at_restrained_translations_kg"] = val(restrained_mass)
    return a_r, per, q, check


def merged_fe(planes, N, form, sec, acc):
    acc_r = {"rod_counts_ok": True}
    ax = B_.rod_freqs_fe(L, sec["EA_m"], N, form, 3, acc_r)
    to = B_.rod_freqs_fe(L, sec["GJ_jbar"], N, form, 3, acc_r) if form == "consistent" else []
    acc["rod_ok"] &= acc_r["rod_counts_ok"]
    ent = [(m["f"], "Y", i + 1) for i, m in enumerate(planes["C"]["modes"])] + \
          [(m["f"], "Z", i + 1) for i, m in enumerate(planes["P"]["modes"])] + \
          [(f, "X axial", i + 1) for i, f in enumerate(ax)] + [(f, "RX torsion", i + 1) for i, f in enumerate(to)]
    return sorted(ent, key=lambda e: e[0])


# --------------------------------------------------------------------------
# responses


def respond(merged, planes_modes, table, count):
    """Per-mode signed contributions of the first `count` merged modes and the three combinations."""
    sel = merged[:count]
    freqs = [e[0] for e in sel]
    S = [S_of(table, f) for f in freqs]
    contrib = {}
    for q in Y_Q + Z_Q:
        plane = "Y" if q in Y_Q else "Z"
        row = []
        for (f, fam, idx), s in zip(sel, S):
            if fam == plane:
                x = planes_modes["C" if plane == "Y" else "P"][idx - 1]["unit"][q]
                row.append(None if x is None else x * s)
            else:
                row.append(Decimal(0))
        contrib[q] = row
    comb = {}
    for q in contrib:
        plane = "Y" if q in Y_Q else "Z"
        if any(x is None for x in contrib[q]):
            comb[q] = None
            contrib[q] = [None if e[1] == plane else x for e, x in zip(sel, contrib[q])]
        elif not any(e[1] == plane for e in sel):
            comb[q] = None
        else:
            comb[q] = rules3(contrib[q], freqs)
    return sel, S, contrib, comb


def why_none(q, sel):
    if q == "mid_UZ_m" and any(e[1] == "Z" for e in sel):
        return "not published: mid-span is not a node at this N"
    return "not published: no finite mode in this plane at this N and formulation"


def fam_label(e):
    return {"Y": "Y bending (cantilever, XY)", "Z": "Z bending (propped, XZ)"}.get(e[1], e[1])


def dev(a, c):
    if a is None or c is None or c == 0:
        return None
    return sci((a - c) / abs(c), 6)


def build(cross, store):
    sec = section()
    acc = {k: Decimal(0) for k in ("cont_balance", "eb_eq", "static_textbook", "sturm", "fe_balance",
                                   "fe_rigid_modal", "rigid_total")}
    acc["rod_ok"] = True
    cont = {m: continuum(sec, m, acc) for m in ("eb", "rayleigh")}
    cmerged = {m: merged_cont(cont[m], sec) for m in cont}
    fe = {}
    for form in FORMS:
        for N in NS:
            planes = {s: fe_plane(s, N, form, sec, acc) for s in ("C", "P")}
            fe[(form, N)] = {"planes": planes, "merged": merged_fe(planes, N, form, sec, acc)}

    def pm(c):
        return {"C": c["C"]["modes"], "P": c["P"]["modes"]}

    # reference: Rayleigh continuum, six periodic modes
    ref = {t["id"]: respond(cmerged["rayleigh"], pm(cont["rayleigh"]), t, N_PERIODIC) for t in (FLAT, TWO)}

    def resp_doc(merged, pmodes, table, count, refr):
        sel, S, contrib, comb = respond(merged, pmodes, table, count)
        rsel, _, rcontrib, rcomb = refr
        out = {"periodic_modes": [{"merged_index": i + 1, "family": fam_label(e), "family_mode": e[2],
                                   "frequency_hz": val(e[0]), "sS_m_per_s2": val(s)} for i, (e, s) in
                                  enumerate(zip(sel, S))], "quantities": {}}
        for q in Y_Q + Z_Q:
            per = []
            for i, (e, x) in enumerate(zip(sel, contrib[q])):
                # deviation against the continuum mode of the same family and index
                cx = None
                for j, re_ in enumerate(rsel):
                    if re_[1] == e[1] and re_[2] == e[2]:
                        cx = rcontrib[q][j]
                per.append({"merged_index": i + 1, "value": None if x is None else val(x)["value"],
                            "binary64": None if x is None else val(x)["binary64"],
                            "rel_dev": dev(x, cx) if x not in (None, Decimal(0)) else None})
            cq = {"per_mode_signed": per}
            if comb[q] is None:
                cq["combined"] = why_none(q, sel)
            else:
                cq["combined"] = {r: {"value": val(comb[q][r])["value"], "binary64": val(comb[q][r])["binary64"],
                                      "rel_dev": dev(comb[q][r], rcomb[q][r])} for r in RULES3}
            out["quantities"][q] = cq
        return out, comb

    def merged_doc(merged, ref_merged, limit=12):
        rows = []
        for i, e in enumerate(merged[:limit]):
            cf = next((r[0] for r in ref_merged if r[1] == e[1] and r[2] == e[2]), None)
            rows.append({"index": i + 1, "family": fam_label(e), "family_mode": e[2], "frequency_hz": val(e[0]),
                         "rel_dev_from_rayleigh": dev(e[0], cf)})
        close = []
        for a in merged[:limit]:
            for b in merged[:limit]:
                if a[1] == "Y" and b[1] == "Z" and abs(a[0] - b[0]) <= Decimal("0.01") * min(a[0], b[0]):
                    close.append([a[2], b[2]])
        return {"finite_modes_total": len(merged), "first_modes": rows, "Y_Z_pairs_within_1_percent": close}

    doc = {"case": "C", "model": "pipe along X, fully fixed at A, only UZ restrained at the tip B",
           "section": sec["doc"],
           "spectra": [R_.table_doc(FLAT), R_.table_doc(TWO)], "scale_factor": inp(1.0),
           "periodic_rule": "the first six modes of the model's merged ascending list (all finite modes where fewer "
                            "exist), no missing-mass correction; axial and torsional modes count in the list but "
                            "contribute zero",
           "quantities": {"from Y": Y_Q, "from Z": Z_Q},
           "sign_conventions": __doc__.split("Global signs: ")[1].strip(),
           "rules": "SRSS; ABSSUM (absolute sum, not in the scope today); PAIRWISE10 (pairwise ten percent, not in "
                    "the scope today)",
           "deviation_basis": "rel_dev against the Rayleigh continuum with its first six merged modes periodic "
                              "(the case as defined); per-mode values against the continuum mode of the same plane "
                              "and index",
           "continuum": {}, "fe": {}, "truncation": {}}
    sums = {}
    for m in ("eb", "rayleigh"):
        d = {"merged_frequencies": merged_doc(cmerged[m], cmerged["rayleigh"]), "responses": {}}
        for t in (FLAT, TWO):
            d["responses"][t["id"]], comb = resp_doc(cmerged[m], pm(cont[m]), t, N_PERIODIC, ref[t["id"]])
            sums[("cont", m, t["id"])] = comb
        doc["continuum"]["euler_bernoulli" if m == "eb" else "rayleigh"] = d
    for form in FORMS:
        for N in NS:
            e = fe[(form, N)]
            pmo = {"C": e["planes"]["C"]["modes"], "P": e["planes"]["P"]["modes"]}
            d = {"merged_frequencies": merged_doc(e["merged"], cmerged["rayleigh"]),
                 "normalization_cases": {s: [m["case"] for m in e["planes"][s]["modes"][:3]] for s in ("C", "P")},
                 "responses": {}}
            for t in (FLAT, TWO):
                if t is TWO and N != 8:
                    continue
                d["responses"][t["id"]], comb = resp_doc(e["merged"], pmo, t, N_PERIODIC, ref[t["id"]])
                sums[(form, N, t["id"])] = comb
            doc["fe"][f"{form}_N{N}"] = d
    # truncation: first two merged modes periodic (Y1, Z1), correction on, f_r = 1.25 Hz, FLAT
    trunc_ref = None
    for key in [("rayleigh",), ("eb",), ("consistent", 8), ("lumped", 8)]:
        if len(key) == 1:
            c = cont[key[0]]
            merged = cmerged[key[0]]
            planes_modes = pm(c)
        else:
            e = fe[key]
            merged = e["merged"]
            planes_modes = {"C": e["planes"]["C"]["modes"], "P": e["planes"]["P"]["modes"]}
        a_r = S_of(FLAT, Fr(F_RIGID))
        out = {"periodic_modes": [], "rigid_acceleration_m_per_s2": val(a_r), "quantities": {}, "total_reaction_check": {}}
        sel = merged[:2]
        assert sorted((s[1], s[2]) for s in sel) == [("Y", 1), ("Z", 1)]
        for e_ in sel:
            out["periodic_modes"].append({"family": fam_label(e_), "frequency_hz": val(e_[0]),
                                          "below_rigid_frequency": e_[0] < dec(F_RIGID)})
        assert all(e_[0] < dec(F_RIGID) for e_ in sel) and merged[2][0] > dec(F_RIGID)
        vals = {}
        for span, qs, plane in (("C", Y_Q, "Y"), ("P", Z_Q, "Z")):
            mode1 = planes_modes[span][0]
            S1 = S_of(FLAT, mode1["f"])
            if len(key) == 1:
                rig = {q: a_r * (c[span]["static"][q] - mode1["unit"][q]) for q in qs}
                T = sec["mbar"] * dec(L)
                tot = rig["base_FY_N"] if span == "C" else rig["base_FZ_N"] + rig["tip_reaction_FZ_N"]
                expect = -a_r * (T - mode1["meff"])
                acc["rigid_total"] = max(acc["rigid_total"], abs(tot - expect) / abs(expect))
                chk = {"total_mass_kg": val(T), "periodic_effective_mass_kg": val(mode1["meff"]),
                       "sum_of_rigid_reactions_N": val(tot), "minus_a_r_times_(T - periodic_meff)_N": val(expect),
                       "relative_residual": sci(abs(tot - expect) / abs(expect))}
            else:
                _, per, rig, chk = fe_rigid(e["planes"][span], span, 8, F_RIGID, FLAT, sec, acc)
                assert per == [0]
            out["total_reaction_check"][plane] = chk
            for q in qs:
                pv = mode1["unit"][q] * S1
                vals[q] = {"periodic": pv, "rigid": rig[q], "SRSS": dsqrt(pv * pv + rig[q] * rig[q]),
                           "ABSSUM": abs(pv) + abs(rig[q])}
        if trunc_ref is None:
            trunc_ref = vals
        for q, v in vals.items():
            out["quantities"][q] = {k: {"value": val(x)["value"], "binary64": val(x)["binary64"],
                                        "rel_dev": dev(x, trunc_ref[q][k])} for k, x in v.items()}
        name = key[0] if len(key) == 1 else f"{key[0]}_N8"
        doc["truncation"]["euler_bernoulli" if name == "eb" else name] = out
        sums[("trunc",) + key] = vals
    doc["truncation"]["definition"] = (
        "first two merged modes periodic (Y1 and Z1), correction on, f_r = 1.25 Hz, FLAT (a_r = 2.5 m/s^2). The "
        "rigid part of direction k is the static response to a_r (M R_k - Gamma_1k M phi_1) over all DOFs; it is "
        "combined with the single periodic contribution by SRSS and by absolute sum. Lumped: the rigid load is "
        "formed on all DOFs, restrained ones included; a restrained translation's lumped mass (mbar h/2) is absent "
        "from the modes but present in the total mass, so it enters the rigid load at its restrained DOF and "
        "reacts there directly (K_cf u - F_c). The total reaction check confirms that the rigid reactions balance "
        "a_r times (total mass - periodic effective mass)")
    doc["truncation"]["rigid_frequency_hz"] = inp(F_RIGID)
    # cross-check against the sibling's N = 8 consistent values
    c8 = json.loads(c8_bytes().decode("utf-8"))["cases"][0]
    e = fe[("consistent", 8)]
    worst_f = max(rel_diff(e["merged"][i][0], dec(c8["frequency_list"]["modes"][i]["frequency_Hz"][0]))
                  for i in range(6))
    worst_c, sign_ok = Decimal(0), True
    for aid, t in (("C8-I-FLAT", FLAT), ("C8-II-TWO-LL", TWO)):
        an = next(a for a in c8["analyses"] if a["request"]["id"] == aid)
        sel, S, contrib, comb = respond(e["merged"], {"C": e["planes"]["C"]["modes"], "P": e["planes"]["P"]["modes"]},
                                        t, 6)
        for q in Y_Q + Z_Q:
            d = "Y" if q in Y_Q else "Z"
            for i in range(6):
                theirs = dec(an["contributions_signed"][d][i]["values"][C8_MAP[q]][0])
                mine = contrib[q][i]
                if mine == 0 and theirs == 0:
                    continue
                worst_c = max(worst_c, rel_diff(mine, theirs))
                sign_ok &= (mine > 0) == (theirs > 0)
    an = next(a for a in c8["analyses"] if a["request"]["id"] == "C8-III-TRUNCATION")
    a3 = dec(an["rigid_acceleration_m_per_s2"]["Y"][0])
    worst_r = Decimal(0)
    for span, qs, d in (("C", Y_Q, "Y"), ("P", Z_Q, "Z")):
        _, _, rig, _ = fe_rigid(e["planes"][span], span, 8, F_RIGID, FLAT, sec, {"fe_rigid_modal": Decimal(0),
                                                                                "rigid_total": Decimal(0)})
        for q in qs:
            mine = rig[q] / S_of(FLAT, Fr(F_RIGID)) * a3
            theirs = dec(an["rigid_response_signed"][d][C8_MAP[q]][0])
            worst_r = max(worst_r, rel_diff(mine, theirs))
            sign_ok &= (mine > 0) == (theirs > 0)
    cross.append({"id": "C8-sibling-frequencies", "what": "N = 8 consistent: first six merged frequencies against "
                  "discrete/c8_pipe_x.json frequency_list", "max_rel_diff": sci(worst_f), "agrees": worst_f < Decimal("1e-35")})
    cross.append({"id": "C8-sibling-contributions", "what": "N = 8 consistent: signed per-mode contributions of the seven "
                  "quantities under FLAT and TWO (linear-linear) against the sibling's C8-I-FLAT and C8-II-TWO-LL",
                  "max_rel_diff": sci(worst_c), "agrees": worst_c < Decimal("1e-35")})
    cross.append({"id": "C8-sibling-rigid", "what": "N = 8 consistent: rigid response per unit rigid acceleration (periodic "
                  "Y1, Z1, f_r = 1.25 Hz) against the sibling's C8-III-TRUNCATION rigid_response_signed scaled by its "
                  "a_r = 3.5 m/s^2", "max_rel_diff": sci(worst_r), "agrees": worst_r < Decimal("1e-35")})
    cross.append({"id": "C8-sibling-signs", "what": "every compared signed value has the sibling's sign", "agrees": sign_ok})
    lim = Decimal("1e-40")
    for k, what, tol in (("cont_balance", "continuum per mode: total reaction = -effective mass", lim),
                         ("eb_eq", "Euler-Bernoulli roots: determinant against cos cosh = -1 and tan = tanh", Decimal("1e-60")),
                         ("static_textbook", "static coefficients against qL^4/8EI, qL^4/192EI and the end reactions", Decimal(0)),
                         ("sturm", "FE eigenvalues: Jacobi against Sturm bisection (40 digits)", Decimal("1e-38")),
                         ("fe_balance", "FE per mode: total reaction = -effective mass", Decimal("1e-35")),
                         ("fe_rigid_modal", "FE rigid displacement against the non-periodic modes' Gamma phi / lam", Decimal("1e-35")),
                         ("rigid_total", "rigid reactions = -a_r (total mass - periodic effective mass), all formulations",
                          Decimal("1e-35"))):
        cross.append({"id": f"C-{k}", "what": what, "max": sci(acc[k]), "agrees": acc[k] <= tol})
    cross.append({"id": "C-rod-closed-forms", "what": "axial and torsional rod frequencies bracketed by Sturm counts",
                  "agrees": acc["rod_ok"]})
    store["c"] = sums
    store["c_merged"] = {"rayleigh": cmerged["rayleigh"], **{k: v["merged"] for k, v in fe.items()}}
    return doc
