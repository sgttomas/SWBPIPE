"""T4-I6 R01 item 8: the stable form's acceptance range as a test spec with concrete sample points
(Design "Stable small-angle evaluation": guard and K-D5 margin from phi = 1e-4 rad to pi - eps, at
ordinary and UTM coordinates).

New frozen sample points (B2 already holds 1e-4, 5 deg, 45 deg, 90 deg, pi - 1e-6 and 1e-8):
phi_nom in {1e-3, 1e-2, 0.1, 1.0, 2.0, 3.0, pi - 1e-3, pi - 1e-4, pi - 1e-5, pi - 1e-7}, planes IP and SK
(B2's directions and y), k = 1, d on the 2^-30 grid (|d| ~ 0.3 m below 3.0 rad, ~ 0.6 m near pi), R the
binary64 of |d| / (2 sin(phi_nom/2)); nodes at X = 0 and X = 7.3e6 (t_X = (X, 0.7X, 0)), d bit-identical.
EXACT per point: phi, pi - phi, K_global (20 digits) and the ACC model's u_int (K1F's supports and loads:
N0 translations rigid, rotational springs 1e6, tip force (1,1,1) N and moment (1,1,1) N*m).
Labelled emulation: a stable binary64 element (gl16 + O7's exact 4R^2 - |d|^2) - its diagonal-scaled
error and P1 residual, and on ACC its actual error and K-D5 trigger (FK dense and sparse).

usage: python -I -B i8_acceptance.py > i8_acceptance.stdout.txt  (writes parts/i8_acceptance.json)
"""
import json
import math
import os
import sys
from decimal import Decimal as D, getcontext, localcontext

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import r01_lib as Lb  # noqa: E402

C = Lb.C
S = Lb.F122_SECTION
G30 = 2.0 ** -30
PLANES = {"IP": ([math.cos(math.pi / 6), math.sin(math.pi / 6), 0.0], [0.0, 1.0, 0.0]),
          "SK": ([1.0 / 3.0, 2.0 / 3.0, 2.0 / 3.0], [1.0, -1.0, 0.5])}
SPRINGS = [(3, 1.0e6), (4, 1.0e6), (5, 1.0e6)]
LOADS = {6: 1.0, 7: 1.0, 8: 1.0, 9: 1.0, 10: 1.0, 11: 1.0}
FREE = [3, 4, 5, 6, 7, 8, 9, 10, 11]


def g(v):
    return round(v / G30) * G30


def main():
    getcontext().prec = 110
    with localcontext() as cx:
        cx.prec = 60
        PI = C.pi()
    samples = [("A1E-3", D("1e-3")), ("A1E-2", D("1e-2")), ("A0.1", D("0.1")), ("A1.0", D("1.0")), ("A2.0", D("2.0")),
               ("A3.0", D("3.0")), ("API-1E-3", PI - D("1e-3")), ("API-1E-4", PI - D("1e-4")), ("API-1E-5", PI - D("1e-5")),
               ("API-1E-7", PI - D("1e-7"))]
    out = []
    print("%-9s %-2s %-26s %-12s | %-9s %-9s | %-30s" % ("sample", "pl", "phi (exact)", "pi - phi", "emu diag", "emu P1", "ACC emu actual / trigger dense, sparse"))
    for label, phin in samples:
        for plane, (dh, y) in PLANES.items():
            scale = 0.6 if phin > D(3) else 0.3
            d = [g(scale * c) for c in dh]
            with localcontext() as cx:
                cx.prec = 60
                Ld = C.norm([D(v) for v in d])
                R = float(Ld / (2 * C.sin(phin / 2)))
            L64 = math.sqrt(d[0] * d[0] + d[1] * d[1] + d[2] * d[2])
            admissible_b64 = R > L64 / 2.0
            el = Lb.exact_element([0.0, 0.0, 0.0], d, R, y)
            geo = el["geo"]
            K = el["K"]
            # X = 7.3e6 nodes: d bit-identical
            X = 7.3e6
            xi = [X, 0.7 * X, 0.0]
            xj = [xi[k] + d[k] for k in range(3)]
            same = all(xj[k] - xi[k] == d[k] for k in range(3))
            # emulated stable product
            Kb, _ = Lb.curved_b64(d, R, y, S, 1.0, 1.0, F_mode="gl16", near_pi_exact=True)
            dg = [K[i][i] for i in range(12)]
            ds = max(abs(Lb.dec(Kb[i][j]) - K[i][j]) / (dg[i] * dg[j]).sqrt() for i in range(12) for j in range(12))
            # P1: rigid modes of the actual nodes (X = 0), row-relative residual
            p1 = D(0)
            for _, mvec in C.rigid_modes([0.0, 0.0, 0.0], d):
                for r in range(12):
                    num = abs(sum(Lb.dec(Kb[r][c]) * mvec[c] for c in range(12)))
                    den = sum(abs(Lb.dec(Kb[r][c]) * mvec[c]) for c in range(12))
                    if den > 0:
                        p1 = max(p1, num / den)
            u = Lb.solve_exact(Lb.assemble(2, [(0, 1, K)], SPRINGS), LOADS, FREE)
            Kfull = [row[:] for row in Kb]
            for dd, v in SPRINGS:
                Kfull[dd][dd] = Kfull[dd][dd] + v
            nodes = [[0.0, 0.0, 0.0], d]
            emu = {}
            for mode in ("dense", "sparse"):
                ub = Lb.product_solve_fk(Kfull, LOADS, FREE, mode)
                a = float(Lb.actual_ratio(nodes, u, ub)[0])
                t, _ = Lb.kd5_trigger(K, SPRINGS, LOADS, FREE, ub, nodes, K_solve=Kfull)
                emu[mode] = (a, t)
            print("%-9s %-2s %-26s %-12s | %.2e  %.2e  | %.1e/%.1e, %.1e/%.1e %s%s" % (
                label, plane, C.sci(geo["phi"], 20), C.sci(PI - geo["phi"], 4), float(ds), float(p1),
                emu["dense"][0], emu["dense"][1], emu["sparse"][0], emu["sparse"][1],
                "" if admissible_b64 else " [R <= L/2 in binary64: REFUSED by PP]", "" if same else " [d differs at X=7.3e6]"))
            out.append(dict(id="%s-%s-k1.0" % (label, plane), phi_nominal=str(phin)[:24], inputs=dict(d=d, R=R, y_reference=y, **S, k_in=1.0, k_out=1.0),
                            nodes_by_X={"0.0": [[0.0, 0.0, 0.0], d], "7300000.0": [xi, xj]}, d_bit_identical_at_7_3e6=same,
                            admissible_in_binary64_R_gt_L_over_2=admissible_b64,
                            derived=dict(phi=C.sci(geo["phi"], 20), pi_minus_phi=C.sci(PI - geo["phi"], 20)),
                            K_global=[[Lb.sig20(v) for v in row] for row in K],
                            acc_model=dict(supports="N0 UX,UY,UZ rigid; N0 RX,RY,RZ springs 1e6 N*m/rad", loads=[[q, v] for q, v in LOADS.items()],
                                           u_int={str(q): Lb.sig20(u[q]) for q in FREE}),
                            emulation_labelled=dict(gl16_O7_diag_scaled="%.2e" % ds, gl16_O7_P1="%.2e" % p1,
                                                    acc_dense_actual_trigger=["%.2e" % x for x in emu["dense"]],
                                                    acc_sparse_actual_trigger=["%.2e" % x for x in emu["sparse"]])))
    with open(os.path.join(HERE, "parts", "i8_acceptance.json"), "w") as fh:
        json.dump({"acceptance_range_samples": out}, fh, indent=1)


if __name__ == "__main__":
    main()
