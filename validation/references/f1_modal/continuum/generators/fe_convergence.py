"""F1 mesh-convergence expectations: the generator's own finite elements.

Hermitian cubic beam elements with consistent translational mass and the
optional Rayleigh rotary-inertia matrix; linear rod elements with consistent
mass. Element matrices are integrated exactly from the shape functions
(Przemieniecki 1968, ch. 11, is the published form they are compared with).
Pencils are exact rationals in nondimensional form (L = 1, EI = 1 or EA = 1,
mbar = 1); eigenvalues come from decimal bisection on unpivoted LDL^T counts
and every published eigenvalue is bracketed by exact rational Sturm counts.
"""

from decimal import Decimal, localcontext
from fractions import Fraction as Fr

import continuum as C
from numerics import (bandwidth, count_exact, dcos, dec, dlog2, fmt, pi, rat, val,
                      verify_simple_eigenvalues)

FE_PREC = 60
FE_DIGITS = 42      # bisection width, relative
FE_SIG = 36         # published digits of FE eigenvalues
VERIFY_DIGITS = 30  # exact-count shifts at lam (1 +- 1e-30)
REFINE = [1, 2, 4, 8, 16]

# --------------------------------------------------------------------------
# Shape functions and exact element integrals (unit length; xi in [0, 1])

HERMITE = [[Fr(1), Fr(0), Fr(-3), Fr(2)], [Fr(0), Fr(1), Fr(-2), Fr(1)],
           [Fr(0), Fr(0), Fr(3), Fr(-2)], [Fr(0), Fr(0), Fr(-1), Fr(1)]]
POW = [0, 1, 0, 1]  # rotation dofs carry one power of h
LINEAR = [[Fr(1), Fr(-1)], [Fr(0), Fr(1)]]


def pmul(a, b):
    out = [Fr(0)] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            out[i + j] += x * y
    return out


def pder(a):
    return [i * a[i] for i in range(1, len(a))] or [Fr(0)]


def pint01(a):
    return sum(c / (i + 1) for i, c in enumerate(a))


def gram(funcs, order):
    def d(p, k):
        for _ in range(k):
            p = pder(p)
        return p
    return [[pint01(pmul(d(f, order), d(g, order))) for g in funcs] for f in funcs]


KHAT = gram(HERMITE, 2)   # EI/h^3 * KHAT_ij * h^(p_i+p_j)
MHAT = gram(HERMITE, 0)   # mbar h * MHAT_ij * h^(p_i+p_j)
RHAT = gram(HERMITE, 1)   # ibar/h * RHAT_ij * h^(p_i+p_j)
KROD = gram(LINEAR, 1)    # EA/h * KROD
MROD = gram(LINEAR, 0)    # mbar h * MROD

STANDARD_K = [[12, 6, -12, 6], [6, 4, -6, 2], [-12, -6, 12, -6], [6, 2, -6, 4]]
DESIGN_M = [[Fr(v, 420) for v in row] for row in
            [[156, 22, 54, -13], [22, 4, 13, -3], [54, 13, 156, -22], [-13, -3, -22, 4]]]
DESIGN_R = [[Fr(v, 30) for v in row] for row in
            [[36, 3, -36, 3], [3, 4, -3, -1], [-36, -3, 36, -3], [3, -1, -3, 4]]]
DESIGN_AXIAL = [[Fr(1, 3), Fr(1, 6)], [Fr(1, 6), Fr(1, 3)]]


def beam_pencil(N, bc, r2=Fr(0), mu=Fr(0), lumped=False):
    h = Fr(1, N)
    nd = 2 * (N + 1)
    K = [[Fr(0)] * nd for _ in range(nd)]
    M = [[Fr(0)] * nd for _ in range(nd)]
    for e in range(N):
        dofs = [2 * e, 2 * e + 1, 2 * e + 2, 2 * e + 3]
        for i in range(4):
            for j in range(4):
                p = POW[i] + POW[j]
                K[dofs[i]][dofs[j]] += KHAT[i][j] * h ** (p - 3)
                if not lumped:
                    M[dofs[i]][dofs[j]] += MHAT[i][j] * h ** (p + 1) + r2 * RHAT[i][j] * h ** (p - 1)
        if lumped:
            M[2 * e][2 * e] += h / 2
            M[2 * e + 2][2 * e + 2] += h / 2
    M[2 * N][2 * N] += mu
    fixed = {"cantilever": {0, 1}, "tip": {0, 1}, "simply_supported": {0, 2 * N},
             "fixed_fixed": {0, 1, 2 * N, 2 * N + 1}}[bc]
    free = [d for d in range(nd) if d not in fixed]
    return [[K[i][j] for j in free] for i in free], [[M[i][j] for j in free] for i in free]


def rod_pencil(N, bc):
    h = Fr(1, N)
    nd = N + 1
    K = [[Fr(0)] * nd for _ in range(nd)]
    M = [[Fr(0)] * nd for _ in range(nd)]
    for e in range(N):
        for i in range(2):
            for j in range(2):
                K[e + i][e + j] += KROD[i][j] / h
                M[e + i][e + j] += MROD[i][j] * h
    fixed = {"cantilever": {0}, "fixed_fixed": {0, N}}[bc]
    free = [d for d in range(nd) if d not in fixed]
    return [[K[i][j] for j in free] for i in free], [[M[i][j] for j in free] for i in free]


def solve(K, M, count, hint):
    from numerics import pencil_eigenvalues
    with localcontext() as c:
        c.prec = FE_PREC
        lams = pencil_eigenvalues(K, M, count, FE_DIGITS, upper_hint=hint)
        checks = verify_simple_eigenvalues(K, M, lams, VERIFY_DIGITS)
    return lams, checks


def finite_count(M):
    """Number of finite eigenvalues = rank of M (diagonal or SPD here)."""
    n = len(M)
    if all(M[i][j] == 0 for i in range(n) for j in range(n) if i != j):
        return sum(1 for i in range(n) if M[i][i] != 0)
    return n


# --------------------------------------------------------------------------


def element_checks(cross):
    k_ok = KHAT == [[Fr(v) for v in row] for row in STANDARD_K]
    m_ok = MHAT == DESIGN_M
    r_ok = RHAT == DESIGN_R
    a_ok = MROD == DESIGN_AXIAL and KROD == [[1, -1], [-1, 1]]
    cross.append({"id": "FE-element-stiffness", "what": "exact integral of Hermite N''N'' vs EI/L^3[12,6L,-12,6L;...]", "agrees": k_ok})
    cross.append({"id": "FE-element-consistent-mass", "what": "exact integral of Hermite NN vs design 2.2 mbar L/420[156,22L,54,-13L;...]", "agrees": m_ok})
    cross.append({"id": "FE-element-rotary-inertia", "what": "exact integral of Hermite N'N' vs design 2.2 ibar/(30L)[36,3L,-36,3L;...]", "agrees": r_ok})
    cross.append({"id": "FE-element-axial-torsion", "what": "linear rod element vs design 2.2 mbar L[1/3,1/6;1/6,1/3]", "agrees": a_ok})
    fr = lambda A: [[str(v) for v in row] for row in A]
    return {"dof_order": ["w_i", "theta_i", "w_j", "theta_j"],
            "scaling": "K_ij = EI/h^3 * Khat_ij * h^(p_i+p_j); M_ij = mbar h * Mhat_ij * h^(p_i+p_j); "
                       "R_ij = ibar/h * Rhat_ij * h^(p_i+p_j); p = (0,1,0,1)",
            "Khat": fr(KHAT), "Mhat": fr(MHAT), "Rhat": fr(RHAT),
            "rod_Khat(EA/h)": fr(KROD), "rod_Mhat(mbar h)": fr(MROD)}


def conv_rows(name, targets, results):
    """targets: continuum Om_k; results: {N: [lam_k]} of Om^2. Frequency errors and orders."""
    rows = []
    for N in sorted(results):
        lams = results[N]["lams"]
        errs = [lam.sqrt() / t - 1 for lam, t in zip(lams, targets)]
        rows.append({"N": N, "Omega^2": [fmt(l, FE_SIG) for l in lams],
                     "rel_freq_error": [fmt(e, 6) for e in errs],
                     "exact_sturm": results[N]["checks"]})
        results[N]["errs"] = errs
    orders = []
    Ns = [N for N in REFINE if N in results]
    for a, b in zip(Ns, Ns[1:]):
        ea, eb = results[a]["errs"], results[b]["errs"]
        orders.append({"N": f"{a}->{b}", "observed_order": [
            fmt(dlog2(abs(x / y)), 4) if x * y > 0 else None for x, y in zip(ea, eb)]})
    return rows, orders


def beam_family(cross, fid, bc, targets, r2=Fr(0), mu=Fr(0), Ns=REFINE, nmodes=6, lumped=False):
    results = {}
    for N in Ns:
        K, M = beam_pencil(N, bc, r2, mu, lumped)
        cnt = min(nmodes, finite_count(M))
        if cnt == 0:
            continue
        lams, checks = solve(K, M, cnt, hint=targets[cnt - 1] ** 2 * 2)
        results[N] = {"lams": lams, "checks": checks}
    rows, orders = conv_rows(fid, targets, results)
    if not lumped:
        above = all(e > 0 for N in results for e in results[N]["errs"])
        cross.append({"id": f"{fid}-from-above", "what": "every consistent-mass FE frequency exceeds its continuum value (Rayleigh-Ritz)",
                      "agrees": above})
    return rows, orders, results


def build(cross):
    doc = {"method": "nondimensional pencils (L = 1, EI = 1, mbar = 1; rods EA = 1). Omega^2 = omega^2 mbar L^4/EI "
                     "(beams), omega^2 mbar L^2/EA (rods). rel_freq_error = sqrt(Omega^2_FE)/Omega_continuum - 1. "
                     "observed_order = log2(|err_N/err_2N|) (null where the two errors differ in sign).",
           "expected": {"consistent_hermitian_beam": "frequency error O(h^4), positive (from above)",
                        "consistent_linear_rod": "frequency error O(h^2), positive (from above)",
                        "lumped_beam_control": "frequency error not bounded, negative here; observed order about 4 for this simply supported span (O(h^2) in general)"},
           "elements": element_checks(cross)}
    P = pi()

    # ---- Euler-Bernoulli beams, N = 1..16 (the coarse data reuse every N)
    eb = []
    coarse_rows = []
    lam_ff1 = C.eb_roots("fixed_fixed")[0]
    for bc in ("cantilever", "simply_supported", "fixed_fixed"):
        targets = [x * x for x in C.eb_roots(bc)]
        rows, orders, res = beam_family(cross, f"FE-EB-{bc}", bc, targets, Ns=range(1, 17))
        eb.append({"id": f"FE-EB-{bc.upper()}", "boundary": bc, "continuum_Omega": [fmt(t) for t in targets],
                   "refinement": [r for r in rows if r["N"] in REFINE],
                   "observed_order": orders})
        roots = C.eb_roots(bc)
        for N in sorted(res):
            for k, e in enumerate(res[N]["errs"], start=1):
                ratio = (lam_ff1 * N / roots[k - 1]) ** 2
                coarse_rows.append((bc, N, k, ratio, e))
    doc["euler_bernoulli"] = eb

    # design 2.2 claims, re-derived
    one_ss = res_first("simply_supported", 1)
    cross.append({"id": "FE-design-2.2-one-consistent-element", "what": "design: one consistent SS element gives omega_1 ~ 10.95 sqrt(EI/mbar L^4), ~11% high",
                  "value": fmt(one_ss, 12), "rel_error": fmt(one_ss / (P * P) - 1, 6),
                  "agrees": abs(one_ss - dec("10.95")) < dec("0.005") and abs(one_ss - dec(120).sqrt()) < dec("1e-35")})

    # ---- Rayleigh beams (rotary inertia) at two slenderness ratios
    ray = []
    for sec, L in (("SEC-A", 6.0), ("SEC-A", 1.0)):
        r2 = C.section_props(sec)["ibar/mbar"] / Fr(L) ** 2
        for bc in ("cantilever", "simply_supported", "fixed_fixed"):
            if bc == "simply_supported":
                targets = [C.rayleigh_ss_om(n, r2) for n in range(1, 7)]
            else:
                targets = C.det_roots(bc, r2=r2)
            fid = f"FE-RAY-{bc.upper()}-{sec}-L{L:g}"
            rows, orders, res = beam_family(cross, fid, bc, targets, r2=r2)
            ray.append({"id": fid, "boundary": bc, "r2=ibar/(mbar L^2)": rat(r2),
                        "continuum_Omega": [fmt(t) for t in targets], "refinement": rows,
                        "observed_order": orders})
    doc["rayleigh"] = ray

    # ---- Tip mass
    tip = []
    for mu in C.TIP_MUS[1:]:
        targets = [x * x for x in C.eb_roots("tip", mu)][:5]
        fid = f"FE-TIP-mu{C.rat_tag(mu)}"
        rows, orders, res = beam_family(cross, fid, "tip", targets, mu=mu, nmodes=5)
        tip.append({"id": fid, "mu": rat(mu), "continuum_Omega": [fmt(t) for t in targets],
                    "refinement": rows, "observed_order": orders})
    doc["tip_mass"] = tip

    # ---- Lumped-mass internal control (simply supported, no rotary mass)
    targets = [x * x for x in C.eb_roots("simply_supported")][:4]
    rows, orders, res = beam_family(cross, "FE-LUMPED-SS", "simply_supported", targets,
                                    Ns=[2, 4, 8, 16], nmodes=4, lumped=True)
    lam2 = res[2]["lams"][0]
    cross.append({"id": "FE-design-2.2-two-lumped-elements", "what": "design: two lumped SS elements give sqrt(96) ~ 9.80, ~0.7% low",
                  "Omega^2": fmt(lam2, 12), "rel_error": fmt(res[2]["errs"][0], 6),
                  "agrees": abs(lam2 - 96) < dec("1e-30")})
    doc["lumped_control"] = {"id": "FE-LUMPED-SS", "boundary": "simply_supported",
                             "mass": "translational nodal lumps mbar h/2 per element end; no rotational mass (M singular)",
                             "internal_control_only": True, "continuum_Omega": [fmt(t) for t in targets],
                             "refinement": rows, "observed_order": orders}

    # ---- Rods (linear consistent elements): closed form verified by exact counts
    doc["rods"] = rods(cross)

    # ---- Coarse-discretization data for the warning factor c (design 2.6)
    doc["coarse_discretization"] = coarse(coarse_rows)
    return doc


def res_first(bc, N):
    K, M = beam_pencil(N, bc)
    lams, _ = solve(K, M, 1, hint=200)
    return lams[0].sqrt()


def orders_for(res, Ns):
    out = []
    for a, b in zip(Ns, Ns[1:]):
        ea, eb = res[a]["errs"], res[b]["errs"]
        out.append({"N": f"{a}->{b}", "observed_order": [
            fmt(dlog2(abs(x / y)), 4) if x * y > 0 else None for x, y in zip(ea, eb)]})
    return out


def rods(cross):
    P = pi()
    out = []
    for bc in ("cantilever", "fixed_fixed"):
        if bc == "cantilever":
            cont = [(2 * j - 1) * P / 2 for j in range(1, 5)]
        else:
            cont = [j * P for j in range(1, 5)]
        rows = []
        errs_by_N = {}
        for N in REFINE:
            K, M = rod_pencil(N, bc)
            m = len(K)
            cnt = min(4, m)
            with localcontext() as c:
                c.prec = FE_PREC
                lams = []
                for j in range(1, cnt + 1):
                    th = (2 * j - 1) * pi() / (2 * N) if bc == "cantilever" else j * pi() / N
                    lams.append(6 * N * N * (1 - dcos(th)) / (2 + dcos(th)))
                checks = verify_simple_eigenvalues(K, M, lams, VERIFY_DIGITS)
            errs = [l.sqrt() / t - 1 for l, t in zip(lams, cont)]
            errs_by_N[N] = {"errs": errs}
            rows.append({"N": N, "Omega^2": [fmt(l, FE_SIG) for l in lams],
                         "rel_freq_error": [fmt(e, 6) for e in errs], "exact_sturm": checks})
        cross.append({"id": f"FE-ROD-{bc}-from-above", "what": "rod FE frequencies exceed continuum",
                      "agrees": all(e > 0 for N in errs_by_N for e in errs_by_N[N]["errs"])})
        out.append({"id": f"FE-ROD-{bc.upper()}", "boundary": bc,
                    "closed_form": "Omega_j^2 = 6 N^2 (1 - cos t_j)/(2 + cos t_j), t_j = (2j-1) pi/(2N) (cantilever) or j pi/N (fixed-fixed)",
                    "continuum_Omega": [fmt(t) for t in cont], "refinement": rows,
                    "observed_order": orders_for(errs_by_N, REFINE)})
    return out


C_LEVELS = ["1", "1.5", "2", "2.5", "3", "4", "5", "6", "8", "10"]


def coarse(rows):
    out_rows = [{"boundary": bc, "N": N, "mode": k, "fe_over_fn": fmt(r, 6), "rel_freq_error": fmt(e, 6)}
                for bc, N, k, r, e in rows]
    summary = []
    for c in C_LEVELS:
        cd = Decimal(c)
        entry = {"c": c}
        for bc in ("cantilever", "simply_supported", "fixed_fixed", "all"):
            sel = [e for b, N, k, r, e in rows if (bc == "all" or b == bc) and r >= cd]
            entry[bc] = {"rows": len(sel), "max_rel_freq_error": fmt(max(sel), 4) if sel else None}
        summary.append(entry)
    return {"definition": "Uniform mesh of N Euler-Bernoulli consistent-mass elements on span L. "
                          "f_e = first fixed-fixed frequency of one element alone (length L/N); f_n = continuum "
                          "frequency of mode n. fe_over_fn = (4.7300407... N/(beta_n L))^2. A later slice derives c "
                          "so that fe_over_fn >= c bounds the error; summary gives the largest error among rows "
                          "with fe_over_fn >= c. Rows: N = 1..16, modes 1..min(6, dofs).",
            "summary": summary, "rows": out_rows}
