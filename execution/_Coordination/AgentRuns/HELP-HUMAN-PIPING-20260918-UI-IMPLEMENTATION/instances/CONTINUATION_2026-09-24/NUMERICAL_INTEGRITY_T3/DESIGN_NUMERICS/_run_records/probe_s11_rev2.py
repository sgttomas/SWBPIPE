#!/usr/bin/env python3
"""T3 D1 S11 revision 2 probe (standard library only; no product code imported).

Rebuilds V1's S11 check A (REVIEW/_run_records/s11/probe_s11_check.py.txt, read, not
imported) on D1's binary64 element (probe_skew_precision.f64_local_K, imported unchanged)
and adds the revision-2 rule: every recovery-side load sum is one exact sum of its
individual binary64 terms, rounded once (exact_rounded_sum, correctly rounded, +0 for an
exact zero). Checks:

  A2  V1's probe A (one 2 m cantilever, three uniform local-y loads (G, 0.3, -G)):
      published root shear and moment, end forces = fl(K_e u) combined exactly with the
      per-load fixed-end terms; plus a midspan station (V, M) and the extrema w.
  INV the invariant "never newly silent": for each case, C3-detect's verdict (solve on
      the binary64 fold, audit against the exact ledger) against C3-full's published
      errors (exact ledger solve, exact recovery). The invariant fails if C3-detect flags
      a case and C3-full publishes a quantity off by more than 1e-9 of its own magnitude.
  MUT the same with the recovery fold restored (the mutation): must breach 1e-9 at
      G = 1e7 and 1e8, so the mutation is killed by the probe A test.
  SUM exact_rounded_sum on V1's check D terms {2^-110, 2^-53, 1}, on signed zeros, and on
      V1's realistic thermal pair (1.3 N, +4.1e7 N, -4.1e7 N).
  ORD the invariant and the mutation in R1's three authored orders (G,n,-G), (G,-G,n), (n,G,-G).
  CMB a combination A + B - A2 of scalar case results (A = A2 = 1e80, B = 1e-8; and a realistic
      1.35*D - 1.35*D2 + L with D = D2 = 4.1e7, L = 1.3): binary64 fold against exact.
  FLR the guard floor: for each probe-A tip row, the row amplification d_i/|f_i| and the
      largest amplification at which the guard still flags every net-load error that a
      1e-9 net-governed comparison (R1 RF-CANCEL recommended scale) would fail.
  ZW  the zero-witness rule: exact zero publishes the binary64 expression's zero when that
      expression is also zero, else +0.
Every value is invented.
"""
from fractions import Fraction as Fr
import json
import sys

sys.path.insert(0, ".")
from probe_skew_precision import f64_local_K  # noqa: E402

U = Fr(1, 2 ** 53)
CRIT = Fr(1, 10 ** 9)


def gamma(m):
    return m * U / (1 - m * U)


def exact_rounded_sum(values):
    """The single ledger function: exact sum of binary64 terms, correctly rounded
    to nearest-even; an exact zero is +0.0 (float(Fraction(0)) is +0.0)."""
    return float(sum((Fr(v) for v in values), Fr(0)))


def fold(values):
    s = 0.0
    for v in values:
        s += v
    return s


def solve(K, f):
    n = len(f)
    A = [[Fr(K[i][j]) for j in range(n)] + [Fr(f[i])] for i in range(n)]
    for c in range(n):
        p = next(r for r in range(c, n) if A[r][c] != 0)
        A[c], A[p] = A[p], A[c]
        for r in range(n):
            if r != c and A[r][c] != 0:
                q = A[r][c] / A[c][c]
                A[r] = [a - q * b for a, b in zip(A[r], A[c])]
    return [A[i][n] / A[i][i] for i in range(n)]


L = 2.0
KE = f64_local_K(L)
FREE = list(range(6, 12))
KF = [[KE[i][j] for j in FREE] for i in FREE]


def fef_uniform_y(w):
    v = w * L / 2.0
    m = w * L * L / 12.0
    e = [0.0] * 12
    e[1], e[5], e[7], e[11] = v, m, v, -m
    return e


def run_case(loads, recovery="exact", solve_force="ledger"):
    fefs = [fef_uniform_y(w) for w in loads]
    per_dof_terms = [[fe[6 + k] for fe in fefs] for k in range(6)]
    if solve_force == "ledger":
        f_tip = [exact_rounded_sum(t) for t in per_dof_terms]
    else:
        f_tip = [fold(t) for t in per_dof_terms]
    u_tip = solve(KF, f_tip)
    u = [Fr(0)] * 6 + u_tip
    u64 = [float(x) for x in u]  # published displacement, correctly rounded
    # elastic end force as the product computes it: binary64 dot K_e * u (published u)
    elastic = [fold([KE[i][j] * u64[j] for j in range(12)]) for i in range(12)]
    if recovery == "exact":
        end = [exact_rounded_sum([elastic[i]] + [-fe[i] for fe in fefs]) for i in range(12)]
    else:  # restored binary64 fold of the equivalents (the mutation, main's behaviour)
        eq = [fold([fe[i] for fe in fefs]) for i in range(12)]
        end = [elastic[i] - eq[i] for i in range(12)]
    # exact references
    fef_exact = [sum((Fr(fe[i]) for fe in fefs), Fr(0)) for i in range(12)]
    Ku = [sum(Fr(KE[i][j]) * u[j] for j in range(12)) for i in range(12)]
    end_exact = [Ku[i] - fef_exact[i] for i in range(12)]
    # midspan station from the i end, section-cut on the j side (V = V_i - sum w x ...):
    x = L / 2.0
    wsum_exact = sum((Fr(w) for w in loads), Fr(0))
    if recovery == "exact":
        V_mid = exact_rounded_sum([end[1]] + [-(w * x) for w in loads])
        M_mid = exact_rounded_sum([end[5], -(end[1] * x)] + [(w * x * x / 2.0) for w in loads])
        w_ext = exact_rounded_sum(loads)
    else:
        V_mid = end[1] - fold([w * x for w in loads])
        M_mid = end[5] - end[1] * x + fold([w * x * x / 2.0 for w in loads])
        w_ext = fold(loads)
    V_mid_exact = end_exact[1] - wsum_exact * Fr(x)
    M_mid_exact = end_exact[5] - end_exact[1] * Fr(x) + wsum_exact * Fr(x) * Fr(x) / 2
    quantities = {
        "root_shear_Fy": (end[1], end_exact[1]),
        "root_moment_Mz": (end[5], end_exact[5]),
        "midspan_shear": (V_mid, V_mid_exact),
        "midspan_moment": (M_mid, M_mid_exact),
        "extrema_w": (w_ext, wsum_exact),
    }
    rel = {}
    for k, (pub, ex) in quantities.items():
        rel[k] = float(abs(Fr(pub) - ex) / abs(ex)) if ex != 0 else (0.0 if pub == 0 else float("inf"))
    # C3-detect verdict: solve on the binary64 fold, audit against the exact ledger
    f_fold = [fold(t) for t in per_dof_terms]
    u_fold = solve(KF, f_fold)
    worst = Fr(0)
    for i in range(6):
        fe = sum((Fr(t) for t in per_dof_terms[i]), Fr(0))
        r = sum(Fr(KF[i][j]) * u_fold[j] for j in range(6)) - fe
        d = abs(fe) + sum(abs(Fr(KF[i][j])) * abs(u_fold[j]) for j in range(6))
        k = sum(1 for j in range(6) if KF[i][j] != 0)
        if d != 0:
            worst = max(worst, (abs(r) / d) / (64 * gamma(2 * k + 2)))
    return rel, float(worst), worst > 1


def zero_witness(exact, witness):
    if exact == 0.0:
        return witness if witness == 0.0 else 0.0
    return exact


def main():
    out = {"python": sys.version.split()[0], "A2_and_INV": [], "MUT": []}
    invariant_holds = True
    cases = [[G, 0.3, -G] for G in (1e5, 1e6, 1e7, 1e8)] + [[1e80, 1e-8, -1e80]]
    cases += [[G, -G, 0.3] for G in (1e7, 1e8)] + [[0.3, G, -G] for G in (1e7, 1e8)]
    for loads in cases:
        rel, worst, detect_flags = run_case(loads, "exact", "ledger")
        breach = {k: v for k, v in rel.items() if v > float(CRIT)}
        if detect_flags and breach:
            invariant_holds = False
        out["A2_and_INV"].append({"loads_N_per_m": loads, "c3_full_rel_error_own": rel,
                                  "c3_full_quantities_above_1e-9": sorted(breach),
                                  "c3_detect_audit_ratio_over_target": worst, "c3_detect_flags": detect_flags})
        mrel, _, _ = run_case(loads, "fold", "ledger")
        out["MUT"].append({"loads_N_per_m": loads, "restored_fold_rel_error_own": mrel,
                           "killed_at_1e-9": any(v > float(CRIT) for v in mrel.values())})
    out["invariant_never_newly_silent_holds"] = invariant_holds
    def comb(vals):
        return {"fold": fold(vals), "exact_rounded_sum": exact_rounded_sum(vals),
                "exact": float(sum((Fr(v) for v in vals), Fr(0)))}
    out["CMB"] = {"A+B-A2 (1e80,1e-8)": comb([1e80, 1e-8, -1e80]),
                  "1.35*D-1.35*D2+L (4.1e7,1.3), products rounded": comb([1.35 * 4.1e7, -(1.35 * 4.1e7), 1.3]),
                  "same, order L first": comb([1.3, 1.35 * 4.1e7, -(1.35 * 4.1e7)])}
    flr = []
    for G in (1e5, 1e7):
        per = [[fe[6 + k] for fe in [fef_uniform_y(w) for w in (G, 0.3, -G)]] for k in range(6)]
        f = [sum((Fr(t) for t in row), Fr(0)) for row in per]
        u = solve(KF, [float(x) for x in f])
        for i in range(6):
            if f[i] == 0:
                continue
            d = abs(f[i]) + sum(abs(Fr(KF[i][j])) * abs(u[j]) for j in range(6))
            k = sum(1 for j in range(6) if KF[i][j] != 0)
            tgt = 64 * gamma(2 * k + 2)
            flr.append({"G": G, "row": i, "amplification_d_over_f": float(d / abs(f[i])),
                        "target_64gamma": float(tgt), "max_amplification_guard_complete": float(CRIT / tgt)})
    out["FLR"] = flr
    out["ZW"] = {"station pass-through [-0.0], witness -0.0": repr(zero_witness(exact_rounded_sum([-0.0]), -0.0)),
                 "ROOT +0 rule on the same": repr(exact_rounded_sum([-0.0])),
                 "repaired absorption: exact 0, witness 1.0": repr(zero_witness(0.0, 1.0)),
                 "nonzero exact unchanged": repr(zero_witness(1.3, 1.2999999970197678))}
    d_terms = [2.0 ** -110, 2.0 ** -53, 1.0]
    out["SUM"] = {
        "check_D_terms": d_terms,
        "naive_ascending_sum": fold(d_terms),
        "exact_rounded_sum": exact_rounded_sum(d_terms),
        "expected_1_plus_2^-52": 1.0 + 2.0 ** -52,
        "zero_cases": {"[1e8,-1e8]": repr(exact_rounded_sum([1e8, -1e8])),
                       "[-0.0]": repr(exact_rounded_sum([-0.0])),
                       "[-0.0,-0.0]": repr(exact_rounded_sum([-0.0, -0.0])),
                       "[]": repr(exact_rounded_sum([]))},
        "thermal_pair_after_1.3N": {"fold": fold([1.3, 4.1e7, -4.1e7]),
                                   "exact_rounded_sum": exact_rounded_sum([1.3, 4.1e7, -4.1e7]),
                                   "fold_rel_error": float(abs(Fr(fold([1.3, 4.1e7, -4.1e7])) - Fr(1.3)) / Fr(1.3))},
    }
    print(json.dumps(out, indent=1))


if __name__ == "__main__":
    main()
