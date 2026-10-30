"""T4-I6 R01 item 4: K1 as an acceptance test of the stable form at phi = 1e-8 (RV131 S-1; RV2 N-5),
and its force-loaded companion K1F (this round: K1's tip moment alone exercises the force-force
flexibility only weakly).

For K1-IP / K1-SK (round-00 JSON m31b_kill_and_mutant) and K1F-IP / K1F-SK (same geometry and
supports; tip force (1,1,1) N and tip moment (1,1,1) N*m):
  * EXACT: u_int (K1: recomputed and compared with the frozen JSON; K1F: frozen here, 20 digits),
    cond1 (FK radix), crK;
  * labelled emulations: the product's actual and K-D5's trigger for 7 binary64 formations x 2 modes
    (a naive closed form is refused by the factor: a negative pivot), and M31b's trigger (K-D5 with
    H from the product's binary64 formula chord) on the correct product's u;
  * the PP form of K1-IP at N0 = (5e6, 3.5e6, 0): binary64 x1 - x0 == d.

usage: python -I -B i4_k1.py rerun00/u1_reference_cases.json > i4_k1.stdout.txt  (writes parts/i4_k1.json)
"""
import json
import math
import os
import sys
from decimal import Decimal as D, getcontext

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import r01_lib as Lb  # noqa: E402

C = Lb.C
S = Lb.F122_SECTION
FREE = [3, 4, 5, 6, 7, 8, 9, 10, 11]
SPRINGS = [(3, 1.0e6), (4, 1.0e6), (5, 1.0e6)]
LOAD_K1 = {9: 1.0, 10: 1.0, 11: 1.0}
LOAD_K1F = {6: 1.0, 7: 1.0, 8: 1.0, 9: 1.0, 10: 1.0, 11: 1.0}
FORMS = ("cr", "kt_cr", "gl12", "gl16", "gl20", "closed_stable", "closed_naive")


def run(xi, xj, R, y, loads, el, u_int, ar):
    nodes = [xi, xj]
    d = [xj[k] - xi[k] for k in range(3)]
    L = math.sqrt(d[0] * d[0] + d[1] * d[1] + d[2] * d[2])
    phib = 2.0 * math.atan2(L / (2.0 * R), math.sqrt((2.0 * R - L) * (2.0 * R + L)) / (2.0 * R))
    cf = [R * (math.cos(phib) - 1.0), R * math.sin(phib), 0.0]
    K_b, _ = Lb.kd5_b1_at_p(ar, xi, xj, R, y, S, 1.0, 1.0, "stable", "override", chord_override=cf)
    K_c, _ = Lb.kd5_b1_at_p(ar, xi, xj, R, y, S, 1.0, 1.0, "stable", "actual")
    out = {}
    for fm in FORMS:
        Kb, _ = Lb.curved_b64(d, R, y, S, 1.0, 1.0, F_mode=fm, F_exact_local=el["F"])
        Kfull = [row[:] for row in Kb]
        for dd, v in SPRINGS:
            Kfull[dd][dd] = Kfull[dd][dd] + v
        for mode in ("dense", "sparse"):
            try:
                ub = Lb.product_solve_fk(Kfull, loads, FREE, mode)
            except (ValueError, ZeroDivisionError):
                out[f"{fm}/{mode}"] = None
                continue
            act = float(Lb.actual_ratio(nodes, u_int, ub)[0])
            t, _ = Lb.kd5_trigger(K_c, SPRINGS, loads, FREE, ub, nodes, K_solve=Kfull)
            tb, _ = Lb.kd5_trigger(K_b, SPRINGS, loads, FREE, ub, nodes, K_solve=Kfull)
            out[f"{fm}/{mode}"] = (act, t, tb)
    return out


def main(u1_path):
    getcontext().prec = 110
    doc = json.load(open(u1_path))
    k1 = {m["model"]: m for m in doc["m31b_kill_and_mutant"]}
    ar = Lb.BinP(128)
    parts = {"k1": [], "k1f": []}
    for plane in ("IP", "SK"):
        m = k1["K1-%s-1E-8-X0" % plane]
        xi, xj = m["nodes"]
        R, y = m["R"], m["y_reference"]
        nodes = [xi, xj]
        el = Lb.exact_element(xi, xj, R, y)
        for label, loads in (("K1", LOAD_K1), ("K1F", LOAD_K1F)):
            K = Lb.assemble(2, [(0, 1, el["K"])], SPRINGS)
            u = Lb.solve_exact(K, loads, FREE)
            Kf = Lb.reduce(K, FREE)
            crad = float(Lb.cond1_radix(Kf))
            crK = float(Lb.actual_ratio(nodes, u, Lb.solve_exact(Lb.assemble(2, [(0, 1, Lb.round_matrix(el["K"]))], SPRINGS, "b64"), loads, FREE))[0])
            print("%s-%s: x_i %r x_j %r R %r y %r phi %s" % (label, plane, xi, xj, R, y, C.sci(el["geo"]["phi"], 20)))
            if label == "K1":
                fz = {int(k): D(v) for k, v in m["u_int"].items()}
                print("  frozen K1 u_int (JSON m31b_kill_and_mutant '%s') vs recomputed: %.1e of the criterion" % (
                    m["model"], float(Lb.actual_ratio(nodes, u, fz)[0])))
            else:
                for q in FREE:
                    print("    u_int dof %2d  %s" % (q, Lb.sig20(u[q])))
            print("  cond1 (FK radix) %.3e  crK %.2e" % (crad, crK))
            emu = run(xi, xj, R, y, loads, el, u, ar)
            for k, v in emu.items():
                if v is None:
                    print("  emulation %-20s REFUSED by the factor (negative pivot): the product cannot publish" % k)
                else:
                    print("  emulation %-20s actual %.3e  K-D5 trigger %.3e  | M31b trigger %.3f" % (k, v[0], v[1], v[2]))
            rec = dict(id="%s-%s-1E-8" % (label, plane), nodes=nodes, R=R, y_reference=y, phi=C.sci(el["geo"]["phi"], 20),
                       section=S, supports="N0 UX,UY,UZ rigid; N0 RX,RY,RZ springs 1e6 N*m/rad",
                       loads=[[q, v] for q, v in loads.items()], cond1_radix="%.3e" % crad, crK="%.2e" % crK,
                       emulations_labelled={k: (None if v is None else [float("%.4g" % x) for x in v]) for k, v in emu.items()})
            if label == "K1F":
                rec["u_int"] = {str(q): Lb.sig20(u[q]) for q in FREE}
            else:
                rec["u_int_json_key"] = "m31b_kill_and_mutant '%s'.u_int" % m["model"]
            parts["k1" if label == "K1" else "k1f"].append(rec)
            print()
    # PP form (K1-IP at N0 = (5e6, 3.5e6, 0))
    m = k1["K1-IP-1E-8-X5e6"]
    x0, x1 = m["nodes"]
    d0 = k1["K1-IP-1E-8-X0"]["nodes"][1]
    ok = all(x1[k] - x0[k] == d0[k] for k in range(3))
    print("PP form K1-IP: x0 %r x1 %r (repr round trip %s) ; binary64 x1 - x0 == d: %s ; bend_radius %r" % (
        x0, x1, [repr(v) for v in x1], ok, m["R"]))
    A = math.pi / 4.0 * (0.2 * 0.2 - 0.18 * 0.18)
    print("PP section from OD 0.2, t 0.01 (one binary64 spelling): A %r (KM %r), relative difference %.1e" % (A, S["A"], abs(A - S["A"]) / S["A"]))
    parts["k1_pp_form"] = dict(x0=x0, x1=x1, bend_radius=m["R"], y_reference=[0.0, 1.0, 0.0], x1_minus_x0_equals_d=ok,
                               u_int_json_key="m31b_kill_and_mutant 'K1-IP-1E-8-X5e6'.u_int (equal to X0)")
    with open(os.path.join(HERE, "parts", "i4_k1.json"), "w") as fh:
        json.dump(parts, fh, indent=1)


if __name__ == "__main__":
    main(sys.argv[1])
