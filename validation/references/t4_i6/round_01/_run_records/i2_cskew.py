"""T4-I6 R01 item 2: CSKEW_8_5, _9, _10 (RV2 S-1; Design's CSKEW rule).

Confirms the frozen u_int (round-00 JSON t3_models[*].u_int_new) by an independent recomputation
(RV131's quadrature element, labelled cross-check), and gives each model's exact cond1 under FK's
radix scaling against the ordinary gate, crK, and labelled emulations (as i1_curved122.py) of
where a T4-U1 product may land relative to the D5C-1 band [0.45, 0.55].

usage: python -I -B i2_cskew.py ../path/to/u1_reference_cases.json > i2_cskew.stdout.txt
       (writes parts/i2_cskew.json)
"""
import json
import os
import sys
from decimal import Decimal as D, getcontext, localcontext

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
sys.path.insert(1, os.path.join(HERE, "inputs"))
import r01_lib as Lb  # noqa: E402
import i1_curved122 as M  # noqa: E402
import rv131_rv_element as rv  # noqa: E402

C = Lb.C
S = Lb.F122_SECTION
NODES = [[0.0, 0.0, 0.0], [0.3, 0.0, 0.3]]
Y = [0.3333333333333333, -1.3333333333333333, -0.3333333333333333]
R = 0.3
FREE = [3, 4, 5, 6, 7, 8, 9, 10, 11]


def main(u1_path):
    getcontext().prec = 110
    doc = json.load(open(u1_path))
    frozen = {m["model"]: (i, m) for i, m in enumerate(doc["t3_models"])}
    out = []
    for name, kx, load in (("CSKEW_8_5", 8.5, 8.5e-06), ("CSKEW_9", 9.0, 9.0 * 1e-06), ("CSKEW_10", 10.0, 10.0 * 1e-06)):
        springs = [(3, kx), (4, 1000000.0), (5, 1000000.0)]
        loads = {9: load}
        el = Lb.exact_element(NODES[0], NODES[1], R, Y)
        K = Lb.assemble(2, [(0, 1, el["K"])], springs)
        u = Lb.solve_exact(K, loads, FREE)
        idx, fm = frozen[name]
        fz = {int(k): D(v) for k, v in fm["u_int_new"].items()}
        ratio_fz = float(Lb.actual_ratio(NODES, u, fz)[0])
        with localcontext() as cx:
            cx.prec = 90
            Kq, _, _ = rv.element_global([0.3, 0.0, 0.3], R, Y, S["E"], S["G"], S["A"], S["I"], S["J"], 1.0, 1.0)
            uq = Lb.solve_exact(Lb.assemble(2, [(0, 1, Kq)], springs), loads, FREE)
        ratio_q = float(Lb.actual_ratio(NODES, u, uq)[0])
        Kf = Lb.reduce(K, FREE)
        c_rx = float(Lb.cond1_radix(Kf))
        c_sq = float(Lb.cond1_sqrt_equilibrated(Kf))
        Kcr = Lb.round_matrix(el["K"])
        crK = float(Lb.actual_ratio(NODES, u, Lb.solve_exact(Lb.assemble(2, [(0, 1, Kcr)], springs, add_springs="b64"), loads, FREE))[0])
        # emulations (labelled)
        M.NODES, M.SPRINGS, M.LOADS, M.FREE = NODES, springs, loads, FREE
        emu = {}
        for fmode in M.FORMATIONS:
            Kb, _ = Lb.curved_b64([0.3, 0.0, 0.3], R, Y, S, 1.0, 1.0, F_mode=fmode, F_exact_local=el["F"])
            Kfull = [row[:] for row in Kb]
            for dd, v in springs:
                Kfull[dd][dd] = Kfull[dd][dd] + v
            for mode in ("dense", "sparse"):
                ub = Lb.product_solve_fk(Kfull, loads, FREE, mode)
                act = float(Lb.actual_ratio(NODES, u, ub)[0])
                trig, _ = Lb.kd5_trigger(el["K"], springs, loads, FREE, ub, NODES)
                emu[f"{fmode}/{mode}"] = (act, trig)
        print("%s (round-00 JSON key t3_models[%d].u_int_new, model '%s'): k_X %r, load (9, %r)" % (name, idx, name, kx, load))
        print("  frozen u_int_new vs this recomputation: %.1e of the criterion (20-digit storage); RV131 quadrature element: %.1e" % (ratio_fz, ratio_q))
        print("  cond1: FK radix %.4e, sqrt-diag %.4e; gate %.4e (margin x%.3f)  crK %.4f" % (c_rx, c_sq, M.GATE, M.GATE / c_rx, crK))
        for mode in ("dense", "sparse"):
            vals = sorted(v[0] for k, v in emu.items() if k.endswith(mode))
            band = sum(0.45 <= a <= 0.55 for a in vals)
            print("  emulated %-6s actual: %s | >0.5: %d/7, in [0.45,0.55]: %d/7, >1: %d/7" % (
                mode, " ".join("%.3f" % a for a in vals), sum(a > 0.5 for a in vals), band, sum(a > 1 for a in vals)))
        print("  max |EF/actual - 1| over the emulations: %.1e" % max(abs(t / 2 / a - 1) for a, t in emu.values() if a > 0))
        out.append(dict(model=name, frozen_json_key="u1_reference_cases.json t3_models[%d] (model '%s').u_int_new" % (idx, name),
                        nodes=NODES, bend=dict(R=R, y_reference=Y, k_in=1.0, k_out=1.0), rigid=[0, 1, 2],
                        springs=[list(t) for t in springs], loads=[[9, load]], load_literal=repr(load),
                        frozen_u_int_vs_recomputed_ratio="%.1e" % ratio_fz, rv131_quadrature_ratio="%.1e" % ratio_q,
                        cond1_radix_exact="%.4e" % c_rx, cond1_sqrt_equilibrated_exact="%.4e" % c_sq,
                        gate_margin="%.3f" % (M.GATE / c_rx), crK="%.4e" % crK,
                        emulated_actual_labelled={k: round(v[0], 4) for k, v in emu.items()}))
        print()
    with open(os.path.join(HERE, "parts", "i2_cskew.json"), "w") as fh:
        json.dump({"cskew_rule_models": out}, fh, indent=1)


if __name__ == "__main__":
    main(sys.argv[1])
