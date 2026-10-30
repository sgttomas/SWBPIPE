"""T4-I6 R01 item 5: the L = 30 m O4 kernel test (RV131 O4 / S-5; Design "Stable small-angle evaluation").

A K-D5 kernel-level (FK) test whose system matrix is K-D5's own B1 re-formation at p = 128 in the
stable spelling (1 - cos phi = 2 s^2 rounded once), each entry rounded once to binary64 - independent
of CB. Geometry: d = grid_2^-30(L (cos 30deg, sin 30deg, 0)), y = (0, 1, 0), R = binary64 of
|d| / (2 sin(phi/2)) at 60 digits; N0 all six DOFs prescribed 0; a unit force at N1 along the chord,
f = binary64(d / |d|). u from FK's dense and sparse solves (emulated). Triggers of K-D5 at p = 128 in
both spellings ('stable' must not demote; 'closed' = the cancelling 1 - fl(cos phi) is the reversion
the test must kill), and at p = 192 closed (a precision-only remedy) for comparison.

usage: python -I -B i5_o4.py > i5_o4.stdout.txt  (writes parts/i5_o4.json)
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
FREE = [6, 7, 8, 9, 10, 11]
G30 = 2.0 ** -30


def grid(v):
    return round(v / G30) * G30


def build(Lnom, phi_nom):
    dh = [math.cos(math.pi / 6), math.sin(math.pi / 6), 0.0]
    d = [grid(Lnom * c) for c in dh]
    with localcontext() as cx:
        cx.prec = 60
        Ld = C.norm([D(v) for v in d])
        R = float(Ld / (2 * C.sin(D(phi_nom) / 2)))
    return d, R


def main():
    getcontext().prec = 90
    y = [0.0, 1.0, 0.0]
    xi = [0.0, 0.0, 0.0]
    out = []
    for Lnom, phi_nom in ((30.0, 1e-9), (30.0, 2e-9), (30.0, 1e-8), (3.0, 1e-9)):
        d, R = build(Lnom, phi_nom)
        xj = d[:]
        nodes = [xi, xj]
        L64 = math.sqrt(d[0] * d[0] + d[1] * d[1] + d[2] * d[2])
        f = {6: d[0] / L64, 7: d[1] / L64, 8: d[2] / L64}
        el = Lb.exact_element(xi, xj, R, y)
        ar = Lb.BinP(128)
        K_st, info = Lb.kd5_b1_at_p(ar, xi, xj, R, y, S, 1.0, 1.0, "stable", "actual")
        K_cl, _ = Lb.kd5_b1_at_p(ar, xi, xj, R, y, S, 1.0, 1.0, "closed", "actual")
        K_192, _ = Lb.kd5_b1_at_p(Lb.BinP(192), xi, xj, R, y, S, 1.0, 1.0, "closed", "actual")
        Ksys = [[float(v) for v in row] for row in K_st]
        # system matrix vs the exact element (diag-scaled), and the stable/closed K-D5 forms vs exact
        Kex = el["K"]

        def ds(A):
            worst = D(0)
            for i in range(12):
                for j in range(12):
                    a = A[i][j]
                    av = Lb.fr_dec(a) if not isinstance(a, float) else Lb.dec(a)
                    worst = max(worst, abs(av - Kex[i][j]) / (Kex[i][i] * Kex[j][j]).sqrt())
            return worst
        u_int = Lb.solve_exact(Lb.assemble(2, [(0, 1, Kex)], []), f, FREE)
        print("L %g phi_nom %g: d %r R %r phi %s ; f %r" % (Lnom, phi_nom, d, R, C.sci(el["geo"]["phi"], 20), [f[6], f[7], f[8]]))
        print("  diag-scaled |K - K_exact|: system (stable p128 rounded once) %.2e ; stable p128 %.2e ; closed p128 %.2e ; closed p192 %.2e" % (
            float(ds(Ksys)), float(ds(K_st)), float(ds(K_cl)), float(ds(K_192))))
        res = {}
        for mode in ("dense", "sparse"):
            ub = Lb.product_solve_fk(Ksys, f, FREE, mode)
            act = float(Lb.actual_ratio(nodes, u_int, ub)[0])
            row = {"actual_vs_exact_intended": act}
            for name, Kint in (("kd5_stable_p128", K_st), ("kd5_closed_p128", K_cl), ("kd5_closed_p192", K_192)):
                t, at = Lb.kd5_trigger(Kint, [], f, FREE, ub, nodes, K_solve=Ksys)
                row[name] = (t, at)
            res[mode] = row
            print("  %-6s actual vs exact intended %.2e | " % (mode, act) + " | ".join(
                "%s trigger %.3e (dof %s)%s" % (k, v[0], v[1], " DEMOTES" if v[0] > 1 else "") for k, v in row.items() if k != "actual_vs_exact_intended"))
        print()
        out.append(dict(id="O4-L%g-PHI%g" % (Lnom, phi_nom), nodes=nodes, R=R, y_reference=y, phi=C.sci(el["geo"]["phi"], 20),
                        section=S, k_in=1.0, k_out=1.0, prescribed="N0 all six DOFs = 0", free=FREE,
                        load={str(k): repr(v) for k, v in f.items()},
                        system_element_matrix_binary64=[[repr(v) for v in row] for row in Ksys],
                        construction="K-D5's B1 re-formation at p = 128, stable spelling (2 s^2), formation_check.rs operation order, each entry rounded once",
                        u_int_exact={str(q): Lb.sig20(u_int[q]) for q in FREE},
                        expected_triggers_labelled={m: {k: ([float("%.4g" % v[0]), v[1]] if isinstance(v, tuple) else float("%.4g" % v)) for k, v in r.items()} for m, r in res.items()}))
    with open(os.path.join(HERE, "parts", "i5_o4.json"), "w") as fh:
        json.dump({"o4_kernel_test": out}, fh, indent=1)


if __name__ == "__main__":
    main()
