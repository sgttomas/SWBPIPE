#!/usr/bin/env python3
"""D1 D-5: first-order formation-error estimate for invented realistic models (standard library).

For the models M9 (axis-aligned) and M11 (skew line (2,1,0)) of cond_realistic.py, with nodal
gravity-like loads (Fz = -1000 N at every node, Fy = +100 N at the free end), computes
EF = K_rep^-1 ((K_rep - K_int) u), K_int formed exactly from the same binary64 primitives and
exact geometry, and reports max |EF_i| over translations and rotations relative to the largest
|u| of the same kind (the class scale), in units of 1e-9. sweep_d5_r1.py shows EF tracks the
actual binary64 error within about 1x to 5x on R1's cases. Invented models; not a product run.
"""
from fractions import Fraction as Fr
import json
import sys

sys.path.insert(0, ".")
import sweep_d5_r1 as sw  # noqa: E402
import probe_skew_precision as d1  # noqa: E402

SEC = sw.section_f64({"E": "200000000000", "G": "80000000000", "OD": "0.2", "ID": "0.18"})


def run(points, supports, loads):
    n = 6 * len(points)
    K = [[0.0] * n for _ in range(n)]
    Kx = [[Fr(0)] * n for _ in range(n)]
    for i in range(len(points) - 1):
        xi, xj = tuple(float(c) for c in points[i]), tuple(float(c) for c in points[i + 1])
        d = [xj[k] - xi[k] for k in range(3)]
        yr = sw.yref_for(d)
        R, L = d1.f64_frame(xi, xj, yr)
        Kg, T = d1.f64_global_K(sw.local_K(L, SEC), R)
        Rx, Lx = sw.exact_frame(tuple(Fr(c) for c in points[i]), tuple(Fr(c) for c in points[i + 1]), tuple(Fr(v) for v in yr))
        Tx = d1.transform12(Rx)
        Kgx = d1.matmul(d1.transpose(Tx), d1.matmul(sw.exact_local_K(Lx, SEC), Tx, d1.A(None)), d1.A(None))
        mp = d1.dofmap(i, i + 1)
        for a in range(12):
            for b in range(12):
                K[mp[a]][mp[b]] += Kg[a][b]
                Kx[mp[a]][mp[b]] += Kgx[a][b]
    restrained = set()
    for node, (rigid, springs) in supports.items():
        for dn in rigid:
            restrained.add(6 * node + sw.DOF[dn])
        for dn, k in springs:
            K[6 * node + sw.DOF[dn]][6 * node + sw.DOF[dn]] += k
            Kx[6 * node + sw.DOF[dn]][6 * node + sw.DOF[dn]] += Fr(k)
    f = [0.0] * n
    for dof, v in loads:
        f[dof] += v
    free = [i for i in range(n) if i not in restrained]
    Kf = [[K[a][b] for b in free] for a in free]
    x, _ = d1.f64_ldl_solve(Kf, [f[a] for a in free])
    u = [0.0] * n
    for a, v in zip(free, x):
        u[a] = v
    dKu = [float(sum((Fr(K[i][j]) - Kx[i][j]) * Fr(u[j]) for j in range(n) if u[j] != 0)) for i in range(n)]
    w, _ = d1.f64_ldl_solve(Kf, [dKu[a] for a in free])
    W = [0.0] * n
    for a, v in zip(free, w):
        W[a] = abs(v)
    st = max(abs(u[i]) for i in range(n) if i % 6 < 3)
    sr = max(abs(u[i]) for i in range(n) if i % 6 >= 3)
    return {"EF_translation_over_class_scale_in_1e-9": max(W[i] for i in range(n) if i % 6 < 3) / st / 1e-9,
            "EF_rotation_over_class_scale_in_1e-9": max(W[i] for i in range(n) if i % 6 >= 3) / sr / 1e-9}


def main():
    A6 = ["UX", "UY", "UZ", "RX", "RY", "RZ"]
    out = {}
    for name, pts in (("M9 axis-aligned 60 m", [(2 * i, 0, 0) for i in range(31)]),
                      ("M11 skew (2,1,0) 60 m", [(2 * i, i, 0) for i in range(31)])):
        sup = {0: (A6, [])}
        for i in range(3, 31, 3):
            sup[i] = ([], [("UZ", 5e4)])
        sup[30] = (["UY"], [("UZ", 5e4)])
        loads = [(6 * i + 2, -1000.0) for i in range(1, 31)] + [(6 * 30 + 1, 100.0)]
        out[name] = run(pts, sup, loads)
    print(json.dumps(out, indent=1))


if __name__ == "__main__":
    main()
