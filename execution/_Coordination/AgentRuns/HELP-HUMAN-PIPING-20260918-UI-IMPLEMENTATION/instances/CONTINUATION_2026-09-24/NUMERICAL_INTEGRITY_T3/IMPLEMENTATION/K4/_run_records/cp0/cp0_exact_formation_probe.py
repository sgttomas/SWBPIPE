#!/usr/bin/env python3
"""I12 checkpoint-0 scratch: does K4's exact-sum formation change the probe's stop-rule
outcomes? Imports D1's probe_skew_precision.py read-only (its arithmetic, cases, dense LDL,
recovery, scales and agreement), and replaces only the formation with the K4 plan's:
exact dot products rounded once, D's 4c and 2c as exact doublings, DB and K_e as one exact
expansion per entry rounded once, assembly and loads exact, rounded once.
Not a K4 emulation (no RCM, scaling, refinement or exact recovery sums); indicative only.
"""
import json
import sys
from fractions import Fraction as Fr

sys.path.insert(0, sys.argv[1])
import probe_skew_precision as d1  # noqa: E402

rnd = d1.rnd


def xdot(xs, ys, p):
    return rnd(sum((Fr(x) * Fr(y) for x, y in zip(xs, ys)), Fr(0)), p)


def frame(xi, xj, yref, ar):
    p = ar.p
    d = [rnd(d1.F(xj[k]) - d1.F(xi[k]), p) for k in range(3)]
    L = ar.sqrt(xdot(d, d, p))
    ex = [ar.div(c, L) for c in d]
    yr = [d1.F(c) for c in yref]
    proj = xdot(yr, ex, p)
    yc = [rnd(yr[k] - proj * ex[k], p) for k in range(3)]
    ny = ar.sqrt(xdot(yc, yc, p))
    ey = [ar.div(c, ny) for c in yc]
    zc = [rnd(ex[1] * ey[2] - ex[2] * ey[1], p), rnd(ex[2] * ey[0] - ex[0] * ey[2], p),
          rnd(ex[0] * ey[1] - ex[1] * ey[0], p)]
    nz = ar.sqrt(xdot(zc, zc, p))
    ez = [ar.div(c, nz) for c in zc]
    return [ex, ey, ez], L


def element(xi, xj, yref, sec, ar):
    p = ar.p
    R, L = frame(xi, xj, yref, ar)
    ex, ey, ez = R
    inv = ar.div(Fr(1), L)
    ea = ar.div(rnd(sec["E"] * sec["A"], p), L)
    gj = ar.div(rnd(sec["G"] * sec["J"], p), L)
    cz = ar.div(rnd(sec["E"] * sec["Iz"], p), L)
    cy = ar.div(rnd(sec["E"] * sec["Iy"], p), L)
    B = [[Fr(0)] * 12 for _ in range(6)]
    for k in range(3):
        B[0][k], B[0][6 + k] = -ex[k], ex[k]
        B[1][3 + k], B[1][9 + k] = -ex[k], ex[k]
        iy = rnd(inv * ey[k], p)
        iz = rnd(inv * ez[k], p)
        for row, rot in ((2, 3), (3, 9)):
            B[row][k], B[row][6 + k], B[row][rot + k] = iy, -iy, ez[k]
        for row, rot in ((4, 3), (5, 9)):
            B[row][k], B[row][6 + k], B[row][rot + k] = -iz, iz, ey[k]
    D = [[Fr(0)] * 6 for _ in range(6)]
    D[0][0], D[1][1] = ea, gj
    for a, b, c in ((2, 3, cz), (4, 5, cy)):
        D[a][a] = D[b][b] = 4 * c
        D[a][b] = D[b][a] = 2 * c
    DB = [[rnd(sum((D[r][s] * B[s][c] for s in range(6)), Fr(0)), p) for c in range(12)] for r in range(6)]
    Ke = [[Fr(0)] * 12 for _ in range(12)]
    for a in range(12):
        for b in range(a, 12):
            v = rnd(sum((B[r][a] * DB[r][b] for r in range(6)), Fr(0)), p)
            Ke[a][b] = Ke[b][a] = v
    return Ke, R, L, B, D


def solve_case(case, p, sections=None):
    ar = d1.A(p)
    n = case["n"]
    terms = [[[] for _ in range(n)] for _ in range(n)]
    for mem in case["members"]:
        i, j, xi, xj, yref = mem[:5]
        sec = mem[5] if len(mem) > 5 else d1.SECTION
        Ke, *_ = element(xi, xj, yref, sec, ar)
        m = d1.dofmap(i, j)
        for a in range(12):
            for b in range(12):
                if Ke[a][b] != 0:
                    terms[m[a]][m[b]].append(Ke[a][b])
    for dof, k in case["springs"]:
        terms[dof][dof].append(d1.F(k))
    K = [[rnd(sum(t, Fr(0)), p) for t in row] for row in terms]
    f = [Fr(0)] * n
    exact = {}
    for dof, v in case["loads"]:
        exact[dof] = exact.get(dof, Fr(0)) + d1.F(v)
    for dof, s in exact.items():
        f[dof] = rnd(s, p)
    free = [d for d in range(n) if d not in case["restrained"]]
    x, err = d1.ldl_solve([[K[a][b] for b in free] for a in free], [f[a] for a in free], ar)
    return (x, err, free, K, f, ar, d1.PIVOT_EVIDENCE.get("last"))


def published(case, p):
    x, err, free, K, f, ar, margin = solve_case(case, p)
    if err:
        return None, err, margin
    n = case["n"]
    u = [Fr(0)] * n
    for a, v in zip(free, x):
        u[a] = v
    # Recovery as the probe does it (sequential at p), for the stop-rule indication only.
    element_F = []
    for mem in case["members"]:
        i, j, xi, xj, yref = mem[:5]
        sec = mem[5] if len(mem) > 5 else d1.SECTION
        _, R, L, _, _ = element(xi, xj, yref, sec, ar)
        T = d1.transform12(R)
        Bl = d1.basic_B_local(L, ar)
        D = d1.basic_D(sec, L, ar)
        ue = [u[d] for d in d1.dofmap(i, j)]
        dl = [xdot(T[r], ue, p) for r in range(12)]
        e = [xdot(Bl[r], dl, p) for r in range(6)]
        Q = [xdot(D[r], e, p) for r in range(6)]
        mi = ar.sqrt(xdot([Q[2], Q[4]], [Q[2], Q[4]], p))
        mj = ar.sqrt(xdot([Q[3], Q[5]], [Q[3], Q[5]], p))
        element_F.append(dict(N=Q[0], T=Q[1], Mi=mi, Mj=mj))
    reactions = {}
    for dof, k in case["springs"]:
        reactions[dof] = rnd(-d1.F(k) * u[dof], p)
    for dof in case["restrained"]:
        reactions[dof] = rnd(sum((K[dof][c] * u[c] for c in range(n)), Fr(0)) - f[dof], p)
    return d1.published(case, u, element_F, reactions), None, margin


def run(name, case, ps):
    ref, err, _ = d1.method_exact_reference(case)
    assert err is None
    out = {"case": name}
    pubs = {}
    for p in ps:
        pub, err, margin = published(case, p)
        pubs[p] = pub
        out[f"p{p}"] = err or dict(d1.compare(pub, ref, case), min_pivot_margin=margin)
    for p in ps:
        if 2 * p in pubs and pubs[p] is not None and pubs[2 * p] is not None:
            out[f"agree_{p}_{2 * p}"] = d1.agreement(pubs[p], pubs[2 * p], case)
    return out


def main():
    skew = (3, 4, 0)
    mom = lambda k: (1e-8 * k / 1e-4, 2e-8 * k / 1e-4, 0.0)
    res = [
        run("SKEW k=1e-28", d1.pin_case(skew, 1e-28, mom(1e-28)), [128, 256, 512]),
        run("SKEW 6-member k=1e-12", d1.pin_case(skew, 1e-12, mom(1e-12), members=6), [128, 256]),
        run("SKEW k=1e-12 (N06 class)", d1.pin_case(skew, 1e-12, mom(1e-12)), [128, 256]),
    ]
    print(json.dumps(res, indent=1))


if __name__ == "__main__":
    main()
