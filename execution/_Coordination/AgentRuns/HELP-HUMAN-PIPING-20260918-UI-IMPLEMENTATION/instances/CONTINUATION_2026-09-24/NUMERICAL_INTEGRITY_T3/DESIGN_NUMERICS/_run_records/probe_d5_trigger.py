#!/usr/bin/env python3
"""D1 D-5 trigger probe (standard library only; no product code imported).

P1 found RF-SKEW-T-CANT-OFF-122-r1e-04 Passed on main yet missing 1e-9; its 345 sibling passes.
This probe rebuilds both from R1's model (one member, root translations rigid, global rotational
springs k_X (soft), 1e6, 1e6 at the root, a pure tip torque along the member) on D1's binary64
element emulation (probe_skew_precision.py, imported unchanged: binary64 local K, binary64
frame, two-stage T^T(K T), binary64 LDL), and reports:

  ERR   ordinary binary64 and "represented matrix solved exactly" errors against the exact
        reference, per quantity, as ratio to 1e-9*max(|exp|, coupled class scale);
  ATTR  the artificial stiffness the represented element gives the soft rigid rotation about
        global X (exact r^T K_rep r, which is 0 for the intended element), split into the
        axial, torsional and bending parts of the local matrix, relative to the soft spring k;
  EST   option (a)'s estimator: B = the product's own formation roundoff bound
        (transform_roundoff: gamma(24)*(|T|^T M/(1-g) + |T|^T|K T|), M = |K||T|), then
        w = K_rep^{-1} (B |u|) (one extra solve with the existing factor), and per displacement
        component the estimated error |w_i| against the actual error and against 1e-9*scale.
Cases: the two P1 cases with y_ref (0,0,1) and (0,1,0); a stiff companion (k_X = 1e6) of each;
the D1 axis-aligned N05-class control; and an intermediate k sweep for the 122 geometry.
Every value is invented or taken from R1's published model; nothing here is a reference.
"""
from fractions import Fraction as Fr
import json
import math
import sys

sys.path.insert(0, ".")
import probe_skew_precision as d1  # noqa: E402

CRIT = Fr(1, 10 ** 9)


def case(direction, springs, moment, yref):
    xi, xj = (0.0, 0.0, 0.0), tuple(float(c) for c in direction)
    return d1.build_case([(0, 1, xi, xj, yref)], [(3, springs[0]), (4, springs[1]), (5, springs[2])],
                         [0, 1, 2], [(9, moment[0]), (10, moment[1]), (11, moment[2])], 2)


def gamma(m):
    u = 2.0 ** -53
    return m * u / (1 - m * u)


def f64_local_parts(L, part):
    """binary64 local K with only one family of terms kept: 'axial', 'torsion', 'bending'."""
    K = d1.f64_local_K(L)
    keep = {"axial": {0, 6}, "torsion": {3, 9}, "bending": {1, 2, 4, 5, 7, 8, 10, 11}}[part]
    return [[K[r][c] if (r in keep and c in keep) else 0.0 for c in range(12)] for r in range(12)]


def exact_global(Kl_exact, R_exact):
    T = d1.transform12(R_exact)
    return d1.matmul(d1.transpose(T), d1.matmul(Kl_exact, T, d1.A(None)), d1.A(None))


def formation_bound(Kl, T):
    g = gamma(24)
    temp = [[sum(Kl[i][k] * T[k][j] for k in range(12)) for j in range(12)] for i in range(12)]
    mag = [[sum(abs(Kl[i][k] * T[k][j]) for k in range(12)) for j in range(12)] for i in range(12)]
    B = [[0.0] * 12 for _ in range(12)]
    for i in range(12):
        for j in range(12):
            inh = sum(abs(T[k][i]) * mag[k][j] for k in range(12))
            sec = sum(abs(T[k][i]) * abs(temp[k][j]) for k in range(12))
            B[i][j] = g * (inh / (1 - g) + sec) * (1 + 64 * 2.0 ** -52)
    return B


def analyse(name, c):
    ref, err, _ = d1.method_exact_reference(c)
    assert err is None
    sc = d1.scales(ref, c)
    out = {"case": name}
    pub, err = d1.method_binary64(c)
    rep, err2 = d1.method_represented_exact(c)
    for label, p in (("ordinary_binary64", pub), ("represented_solved_exactly", rep)):
        worst, at, fails = Fr(0), None, []
        for key, (g, kind, e) in ref.items():
            ep, op = Fr(float(Fr(e))), Fr(float(Fr(p[key][2])))
            s = max(abs(ep), sc[key])
            r = Fr(0) if s == 0 else abs(op - ep) / s / CRIT
            if r > 1:
                fails.append((key, float(r)))
            if r > worst:
                worst, at = r, key
        out[label] = {"max_ratio_to_1e-9": float(worst), "at": at, "failing": fails}
    # ATTR: artificial stiffness of the rigid rotation about global X
    (i, j, xi, xj, yref) = c["members"][0]
    Rf, L = d1.f64_frame(xi, xj, yref)
    Rx, Lx = d1.element_frame(xi, xj, yref, d1.A(None))
    r = [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, -xj[2], xj[1], 1.0, 0.0, 0.0]  # theta_X x (x_j - x_i)
    r = [Fr(v) for v in r]
    k_soft = Fr(c["springs"][0][1])
    attr = {}
    for part in ("axial", "torsion", "bending", "all"):
        Kl = d1.f64_local_K(L) if part == "all" else f64_local_parts(L, part)
        Kg, T = d1.f64_global_K(Kl, Rf)
        e = sum(r[a] * Fr(Kg[a][b]) * r[b] for a in range(12) for b in range(12))
        attr[part] = float(e / k_soft)
    out["artificial_rigid_X_stiffness_over_k"] = attr
    # EST: option (a) estimator on the represented system
    K, f, elems = d1.represented_f64(c)
    n = c["n"]
    free = [d for d in range(n) if d not in c["restrained"]]
    Kl = d1.f64_local_K(L)
    Kg, T = d1.f64_global_K(Kl, Rf)
    B = formation_bound(Kl, T)
    x, _ = d1.f64_ldl_solve([[K[a][b] for b in free] for a in free], [f[a] for a in free])
    u = [0.0] * n
    for a, v in zip(free, x):
        u[a] = v
    m = elems[0][0]
    Bu = [0.0] * n
    for a in range(12):
        for b in range(12):
            Bu[m[a]] += B[a][b] * abs(u[m[b]])
    w, _ = d1.f64_ldl_solve([[K[a][b] for b in free] for a in free], [Bu[a] for a in free])
    W = [0.0] * n
    for a, v in zip(free, w):
        W[a] = abs(v)
    est = {}
    worst_est = 0.0
    for key, (g, kind, e) in ref.items():
        if not key.startswith("u:1:"):
            continue
        dof = 6 + int(key.split(":")[2])
        s = float(max(abs(Fr(float(Fr(e)))), sc[key]))
        actual = abs(float(Fr(pub[key][2])) - float(Fr(e))) / s / 1e-9
        estimate = W[dof] / s / 1e-9
        worst_est = max(worst_est, estimate)
        est[key] = {"actual_ratio": actual, "estimated_ratio": estimate}
    out["estimator_per_tip_component"] = est
    out["estimator_worst_ratio"] = worst_est
    out["trigger_(a)_fires_at_ratio_1"] = worst_est > 1.0
    return out


def main():
    res = {"python": sys.version.split()[0], "cases": []}
    m122 = (0.0048, 0.0096, 0.0096)
    m345 = (0.005184, 0.006912, 0.0)
    for yref in ((0.0, 0.0, 1.0), (0.0, 1.0, 0.0)):
        res["cases"].append(analyse(f"OFF-122 k_X=144 yref={yref}", case((1, 2, 2), (144.0, 1e6, 1e6), m122, yref)))
    res["cases"].append(analyse("OFF-345 k_X=86.4 yref=(0,0,1)", case((3, 4, 0), (86.4, 1e6, 1e6), m345, (0.0, 0.0, 1.0))))
    res["cases"].append(analyse("OFF-345 k_X=86.4 yref=(0,1,0)... (x-y plane member, yref (0,1,0) not parallel)", case((3, 4, 0), (86.4, 1e6, 1e6), m345, (0.0, 1.0, 0.0))))
    for k in (1e6, 1e4, 1e2, 1.44):
        res["cases"].append(analyse(f"122 geometry k_X={k:g}", case((1, 2, 2), (k, 1e6, 1e6), m122, (0.0, 0.0, 1.0))))
    res["cases"].append(analyse("AXIS (5,0,0) k=1e-4 all three (D1 N05-class control)", case((5, 0, 0), (1e-4, 1e-4, 1e-4), (1e-8, 2e-8, 0.0), (0.0, 0.0, 1.0))))
    res["cases"].append(analyse("AXIS (5,0,0) k_X=144 torque (axis-aligned sibling)", case((5, 0, 0), (144.0, 1e6, 1e6), (0.0144, 0.0, 0.0), (0.0, 0.0, 1.0))))
    print(json.dumps(res, indent=1))


if __name__ == "__main__":
    main()
