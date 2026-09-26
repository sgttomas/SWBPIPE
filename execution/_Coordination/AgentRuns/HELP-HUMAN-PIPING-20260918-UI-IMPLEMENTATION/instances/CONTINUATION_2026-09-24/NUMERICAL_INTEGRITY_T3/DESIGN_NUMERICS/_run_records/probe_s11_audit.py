#!/usr/bin/env python3
"""T3 D1 S11 containment probe (standard library only; no product code imported).

A 2 m cantilever of the N-series section, fixed at the root, with nodal load
contributions on tip DOFs. The ordinary path folds contributions per DOF in
binary64, in authored order, and solves with that folded load. This probe:

1. builds the represented binary64 element matrix (product formation order),
2. solves it exactly (Fraction) with the folded load, standing in for an accurate
   ordinary solve, so the only defect measured is the load fold,
3. evaluates the proposed load-fidelity screen: the existing M03 componentwise
   intended-action predicate with the EXACT load sum,
       ratio_i = |sum_j K_ij u_j - f_exact_i| / (|f_exact_i| + sum_j |K_ij||u_j|)
   against 64*gamma(m_i), m_i = 2*k_i + 2 (k_i nonzero coefficients in row i),
   which is the operation basis evaluate_original_residual uses. The product's
   audit_intended_action counts expansion operations and would give a larger m
   (a looser target); the verdicts below do not depend on that difference except
   where noted by the margin column.
4. reports the tip response error of the folded answer against the exact-load answer,
   on the response's own scale (the largest translation or rotation at the tip).

Every value is invented. The scale choice is stated in the design, not here.
"""
from fractions import Fraction as Fr
import json
import math
import sys

sys.path.insert(0, ".")
from probe_skew_precision import f64_local_K  # product-order binary64 local stiffness

U = Fr(1, 2 ** 53)


def gamma(m):
    return m * U / (1 - m * U)


def exact_solve(K, f):
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
Kl = f64_local_K(L)          # member along global x: local == global
FREE = list(range(6, 12))     # node 0 fixed, node 1 free
KF = [[Kl[i][j] for j in FREE] for i in FREE]


def fold(values):
    s = 0.0
    for v in values:
        s += v
    return s


def run(name, contributions):
    """contributions: {tip_dof(0..5): [binary64 values in authored order]}"""
    f_fold = [0.0] * 6
    f_exact = [Fr(0)] * 6
    for d, vals in contributions.items():
        f_fold[d] = fold(vals)
        f_exact[d] = sum((Fr(v) for v in vals), Fr(0))
    u_fold = exact_solve(KF, f_fold)
    u_true = exact_solve(KF, f_exact)
    rows = []
    worst_margin = None
    for i in range(6):
        r = sum(Fr(KF[i][j]) * u_fold[j] for j in range(6)) - f_exact[i]
        d = abs(f_exact[i]) + sum(abs(Fr(KF[i][j])) * abs(u_fold[j]) for j in range(6))
        k = sum(1 for j in range(6) if KF[i][j] != 0)
        target = 64 * gamma(2 * k + 2)
        ratio = Fr(0) if d == 0 and r == 0 else (abs(r) / d if d != 0 else Fr(10 ** 30))
        margin = ratio / target
        worst_margin = margin if worst_margin is None else max(worst_margin, margin)
        rows.append({"tip_dof": i, "ratio": float(ratio), "target": float(target), "flagged": ratio > target})
    s_tr = max(abs(u_true[j]) for j in range(3))
    s_rot = max(abs(u_true[j]) for j in range(3, 6))
    err = 0.0
    for j in range(6):
        s = s_tr if j < 3 else s_rot
        if s != 0:
            err = max(err, float(abs(u_fold[j] - u_true[j]) / s))
    net_err = max((float(abs(Fr(f_fold[d]) - f_exact[d]) / abs(f_exact[d])) if f_exact[d] != 0 else 0.0)
                  for d in contributions)
    return {"case": name, "contributions": {str(k): v for k, v in contributions.items()},
            "net_load_relative_error": net_err,
            "tip_response_error_on_own_scale": err,
            "screen_flags_case": any(r["flagged"] for r in rows),
            "worst_ratio_over_target": float(worst_margin),
            "rows": rows}


UY, UZ, RX, RZ = 1, 2, 3, 5
cases = []
for G in (1e5, 1e6, 1e7, 1e8):
    cases.append(run(f"force UY (G={G:g}, 0.3, -G)", {UY: [G, 0.3, -G]}))
for G in (1e5, 1e6, 1e7, 1e8):
    cases.append(run(f"moment RZ (G={G:g}, 0.3, -G) N*m", {RZ: [G, 0.3, -G]}))
cases.append(run("V1 check L analogue UY (1e80, 1e-8, -1e80) + UZ 2e-8", {UY: [1e80, 1e-8, -1e80], UZ: [2e-8]}))
cases.append(run("V1 check L loads only UY (1e80, 1e-8, -1e80)", {UY: [1e80, 1e-8, -1e80]}))
cases.append(run("with a response elsewhere: UY (1e7, 0.3, -1e7) + RX 5 N*m", {UY: [1e7, 0.3, -1e7], RX: [5.0]}))
# Controls that must NOT be flagged
cases.append(run("control: order (G, -G, n) G=1e8", {UY: [1e8, -1e8, 0.3]}))
cases.append(run("control: two terms (1e8, 0.3)", {UY: [1e8, 0.3]}))
cases.append(run("control: three ordinary terms (0.1, 0.2, 0.3)", {UY: [0.1, 0.2, 0.3]}))
cases.append(run("control: same-sign weight-like (1234.5, 987.25, 55.125)", {UY: [1234.5, 987.25, 55.125]}))
cases.append(run("control: mild cancellation (1000.1, -1000.0, 0.05)", {UY: [1000.1, -1000.0, 0.05]}))
print(json.dumps({"python": sys.version.split()[0], "member_length_m": L, "cases": cases}, indent=1))
