"""T4-I6 R01 item 3: K2's frozen binary64 system matrix (RV131 S-2, S-3; RV2 N-4).

For K1-IP and K1-SK (round-00 JSON m31b_kill_and_mutant; nodes x_i = 0, x_j = d):
  * the binary64 formula chord the product mutant forms, c_f = (R(fl(cos phi_b) - 1), fl(R fl(sin phi_b)), 0),
    with phi_b = 2 atan2(s_b, c_b) in binary64 (and, for comparison, 2 asin(L_b / 2R));
    precondition fl(cos phi_b) = 1.0, so c_f,x = 0 exactly;
  * K2's element: the EXACT B1 tip stiffness and exact axes, H from c_f, assembled exactly and
    rounded ONCE to binary64 (independent of any binary64 small-angle evaluation and of CB);
  * the expected K-D5 outcomes on the system (u from FK's dense and sparse solves, emulated):
    correct check (B1 at p = 128, stable and closed spellings), M31b (H from the same binary64
    c_f at p), M31b0 (formula chord at p), M31a (K_int = the system's own element matrix), and
    the historical FK expression of M31a (K_int's curved element = zeros).

usage: python -I -B i3_k2.py rerun00/u1_reference_cases.json > i3_k2.stdout.txt  (writes parts/i3_k2.json)
"""
import json
import math
import os
import sys
from decimal import Decimal as D, getcontext
from fractions import Fraction as Fr

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import r01_lib as Lb  # noqa: E402

C = Lb.C
S = Lb.F122_SECTION
FREE = [3, 4, 5, 6, 7, 8, 9, 10, 11]
SPRINGS = [(3, 1.0e6), (4, 1.0e6), (5, 1.0e6)]
LOADS = {9: 1.0, 10: 1.0, 11: 1.0}


def phi_b64(d, R):
    L = math.sqrt(d[0] * d[0] + d[1] * d[1] + d[2] * d[2])
    s = L / (2.0 * R)
    c = math.sqrt((2.0 * R - L) * (2.0 * R + L)) / (2.0 * R)
    return 2.0 * math.atan2(s, c), 2.0 * math.asin(L / (2.0 * R)), L


def main(u1_path):
    getcontext().prec = 110
    doc = json.load(open(u1_path))
    k1 = {m["model"]: m for m in doc["m31b_kill_and_mutant"]}
    ar = Lb.BinP(128)
    parts = []
    for plane, key in (("IP", "K1-IP-1E-8-X0"), ("SK", "K1-SK-1E-8-X0")):
        m = k1[key]
        xi, xj = m["nodes"]
        d = [xj[k] - xi[k] for k in range(3)]
        R, y = m["R"], m["y_reference"]
        nodes = [xi, xj]
        pa, pb, L64 = phi_b64(d, R)
        cos_a, sin_a = math.cos(pa), math.sin(pa)
        cf = [R * (cos_a - 1.0), R * sin_a, 0.0]
        print("%s: d %r R %r y %r" % (plane, d, R, y))
        print("  phi_b = 2 atan2(s_b, c_b) = %r ; 2 asin(L_b/2R) = %r ; fl(cos phi_b) = %r (precondition 1.0: %s); fl(sin phi_b) = %r" % (
            pa, pb, cos_a, cos_a == 1.0, sin_a))
        print("  formula chord c_f = %r ; c_f,y bits %s" % (cf, cf[1].hex()))
        # K2's element: exact Kt and axes, H from c_f, rounded once
        el = Lb.exact_element(xi, xj, R, y)
        el_mut = Lb.exact_element(xi, xj, R, y, chord_local=cf)
        Ksys = Lb.round_matrix(el_mut["K"])
        # exact actual chord for reference
        c_act = el["c_act"]
        print("  actual chord (exact) = (%s, %s, %s); |c_f - c_act|/L = %.4e" % (
            C.sci(c_act[0], 12), C.sci(c_act[1], 12), C.sci(c_act[2], 4),
            float(C.norm([Lb.dec(cf[k]) - c_act[k] for k in range(3)]) / el["geo"]["L"])))
        # u_int (the exact intended solution) and u_mut (exact solution of K2's system)
        u_int = Lb.solve_exact(Lb.assemble(2, [(0, 1, el["K"])], SPRINGS), LOADS, FREE)
        u_sys = Lb.solve_exact(Lb.assemble(2, [(0, 1, Ksys)], SPRINGS), LOADS, FREE)
        print("  system vs intended (exact solves): actual ratio %.4f" % float(Lb.actual_ratio(nodes, u_int, u_sys)[0]))
        Kfull = [row[:] for row in Ksys]
        for dd, v in SPRINGS:
            Kfull[dd][dd] = Kfull[dd][dd] + v
        # K-D5 variants at p = 128
        K_st, info = Lb.kd5_b1_at_p(ar, xi, xj, R, y, S, 1.0, 1.0, "stable", "actual")
        K_cl, _ = Lb.kd5_b1_at_p(ar, xi, xj, R, y, S, 1.0, 1.0, "closed", "actual")
        K_b, _ = Lb.kd5_b1_at_p(ar, xi, xj, R, y, S, 1.0, 1.0, "stable", "override", chord_override=cf)
        K_b0, _ = Lb.kd5_b1_at_p(ar, xi, xj, R, y, S, 1.0, 1.0, "stable", "formula_p")
        K_zero = [[0.0] * 12 for _ in range(12)]
        res = {}
        for mode in ("dense", "sparse"):
            ub = Lb.product_solve_fk(Kfull, LOADS, FREE, mode)
            row = {}
            for name, Kint in (("correct_stable", K_st), ("correct_closed", K_cl), ("M31b", K_b), ("M31b0", K_b0),
                               ("M31a_system_element", Ksys), ("M31a_historical_zeros", K_zero)):
                t, at = Lb.kd5_trigger(Kint, SPRINGS, LOADS, FREE, ub, nodes, K_solve=Ksys)
                row[name] = (t, at)
            res[mode] = row
            print("  %-6s " % mode + " | ".join("%s %.4g (dof %s)" % (k, v[0], v[1]) for k, v in row.items()))
        # an off-by-one-ulp M31b (cos one ulp low): survives?
        cf_lo = [R * ((1.0 - 2.0 ** -53) - 1.0), R * sin_a, 0.0]
        K_blo, _ = Lb.kd5_b1_at_p(ar, xi, xj, R, y, S, 1.0, 1.0, "stable", "override", chord_override=cf_lo)
        ub = Lb.product_solve_fk(Kfull, LOADS, FREE, "dense")
        t_lo, _ = Lb.kd5_trigger(K_blo, SPRINGS, LOADS, FREE, ub, nodes, K_solve=Ksys)
        print("  M31b with cos one ulp low (chord %r): trigger %.4g (DEMOTES: the kill needs bit-equal chords)" % (cf_lo, t_lo))
        print()
        parts.append(dict(
            id="K2-%s" % plane, base_model=key, nodes=nodes, R=R, y_reference=y, section=S, k_in=1.0, k_out=1.0,
            supports="N0 UX,UY,UZ prescribed 0; N0 RX,RY,RZ ground springs 1e6 N*m/rad (dofs 3,4,5); N1 free",
            loads=[[q, v] for q, v in LOADS.items()],
            phi_b_atan2=repr(pa), phi_b_asin=repr(pb), precondition_fl_cos_phi_b_is_1=(cos_a == 1.0),
            formula_chord_binary64=[repr(v) for v in cf],
            construction="exact B1 tip stiffness and exact B1 axes; H from the binary64 formula chord; T^T K_loc T exact; each entry rounded once to binary64",
            system_element_matrix_binary64=[[repr(v) for v in row] for row in Ksys],
            system_springs="added in binary64 to the diagonal after the element (SA order)",
            u_int={str(q): Lb.sig20(u_int[q]) for q in FREE},
            u_system_exact={str(q): Lb.sig20(u_sys[q]) for q in FREE},
            expected_triggers_labelled_emulation={mode: {k: [round(v[0], 6), v[1]] for k, v in row.items()} for mode, row in res.items()},
            m31b_cos_one_ulp_low_trigger=round(t_lo, 4)))
    with open(os.path.join(HERE, "parts", "i3_k2.json"), "w") as fh:
        json.dump({"k2_kernel_kill": parts}, fh, indent=1)


if __name__ == "__main__":
    main(sys.argv[1])
