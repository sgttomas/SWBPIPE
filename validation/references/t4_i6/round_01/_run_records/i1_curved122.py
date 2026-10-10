"""T4-I6 R01 item 1: the conditioning-driven curved true positive (RV131 B-1).

F122's supports, loads and section (kd5_models.rs F122 at HEAD 10b70036ef) with its 3 m skew
member realized as a B1 bend of radius R and plane reference y.

References (EXACT): u_int = the exact free displacements of the B1 model (round-00 closed form,
110 digits; checked at 140 digits and against RV131's independent quadrature element, a labelled
cross-check); cond1 of the reduced matrix equilibrated by sqrt(diag) and by FK's radix exponents;
crK = the actual-error ratio of the exact solve of the correctly rounded element (springs exact).

Labelled emulations (NOT product code): the product's actual error and K-D5's trigger for four
plausible binary64 formations of the element (CR = exact local F rounded once; closed_stable;
closed_naive; gl16) under FK's dense Cholesky and sparse_direct's RCM + skyline LDL^T (the solve
emulation reproduces F122's recorded product values, see calib below); M31a's trigger
(K_int = the product's binary64 element).

usage: python -I -B i1_curved122.py > i1_curved122.stdout.txt   (writes parts/i1_curved122.json)
"""
import json
import math
import os
import sys
from decimal import Decimal as D, getcontext, localcontext

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
sys.path.insert(1, os.path.join(HERE, "inputs"))
import r01_lib as Lb  # noqa: E402

C = Lb.C
S = Lb.F122_SECTION
NODES = [[0.0, 0.0, 0.0], [1.0, 2.0, 2.0]]
RIGID = [0, 1, 2]
SPRINGS = [(3, 144.0), (4, 1000000.0), (5, 1000000.0)]
LOADS = {9: 0.0048, 10: 0.0096, 11: 0.0096}
FREE = [q for q in range(12) if q not in RIGID]
F122_RECORDED = {"dense": (2.4133641591897357, 4.826728293925875), "sparse": (1.2139461517530596, 2.4278923279576334)}
GATE = 1.0 / math.sqrt(2.0 ** -52)  # structural.rs: rcond < sqrt(EPSILON) is ordinarily Sensitive


def exact_model(R, y, prec=110):
    with localcontext() as cx:
        cx.prec = prec
        el = Lb.exact_element(NODES[0], NODES[1], R, y)
        K = Lb.assemble(2, [(0, 1, el["K"])], SPRINGS)
        u = Lb.solve_exact(K, LOADS, FREE)
        return el, K, u


def product_full(Kel_b64):
    Kfull = [row[:] for row in Kel_b64]
    for dd, v in SPRINGS:
        Kfull[dd][dd] = Kfull[dd][dd] + v
    return Kfull


FORMATIONS = ("cr", "kt_cr", "closed_stable", "closed_naive", "gl12", "gl16", "gl20")


def emulate(R, y, el, u_ex, formations=("cr", "closed_stable", "closed_naive", "gl16")):
    """Labelled emulations: actual and K-D5 / M31a triggers per formation and mode."""
    out = {}
    Floc = el["F"]
    for fm in formations:
        Kb, _ = Lb.curved_b64([1.0, 2.0, 2.0], R, y, S, 1.0, 1.0, F_mode=fm, F_exact_local=Floc)
        Kfull = product_full(Kb)
        for mode in ("dense", "sparse"):
            u = Lb.product_solve_fk(Kfull, LOADS, FREE, mode)
            act = float(Lb.actual_ratio(NODES, u_ex, u)[0])
            trig, at = Lb.kd5_trigger(el["K"], SPRINGS, LOADS, FREE, u, NODES)
            trig_a, _ = Lb.kd5_trigger(Kb, SPRINGS, LOADS, FREE, u, NODES)
            out[f"{fm}/{mode}"] = dict(actual=act, kd5_trigger=trig, kd5_dof=at, m31a_trigger=trig_a)
    return out


def main():
    getcontext().prec = 110
    parts = {}
    # ---- calibration of the solve emulation on F122 itself (straight frame, FK's operation order)
    Kex = C.frame_element(NODES[0], NODES[1], [1.0, 0.0, 0.0], S["E"], S["G"], S["A"], S["I"], S["I"], S["J"])
    uex = Lb.solve_exact(Lb.assemble(2, [(0, 1, Kex)], SPRINGS), LOADS, FREE)
    Kfull = product_full(Lb.fk_frame_b64(NODES[0], NODES[1], [1.0, 0.0, 0.0]))
    print("calib: F122 straight, FK frame in FK's order, FK prepare + factor (emulation) vs the recorded product run")
    for mode in ("dense", "sparse"):
        u = Lb.product_solve_fk(Kfull, LOADS, FREE, mode)
        act = float(Lb.actual_ratio(NODES, uex, u)[0])
        trig, at = Lb.kd5_trigger(Kex, SPRINGS, LOADS, FREE, u, NODES)
        print("  %-6s actual %.10f (recorded %.10f) | K-D5 trigger %.10f at dof %s (recorded %.10f)" % (
            mode, act, F122_RECORDED[mode][0], trig, at, F122_RECORDED[mode][1]))
    Kf = Lb.reduce(Lb.assemble(2, [(0, 1, Kex)], SPRINGS), FREE)
    print("  F122 cond1 sqrt-equilibrated %.4e, radix-scaled %.4e (gate 1/sqrt(eps) = %.4e)" % (
        Lb.cond1_sqrt_equilibrated(Kf), Lb.cond1_radix(Kf), GATE))
    print()

    # ---- the briefed candidates
    cands = [("C122-R10-Y100", 10.0, [1.0, 0.0, 0.0], 144.0), ("C122-R10-Y01M1", 10.0, [0.0, 1.0, -1.0], 144.0),
             ("C122-R100-Y100", 100.0, [1.0, 0.0, 0.0], 144.0), ("C122-R100-Y01M1", 100.0, [0.0, 1.0, -1.0], 144.0),
             # alternates chosen by i1b/i1c (exploratory): k_X = 120 raises cond toward the gate
             ("C122K120-R100-Y100", 100.0, [1.0, 0.0, 0.0], 120.0), ("C122-R10-YM100", 10.0, [-1.0, 0.0, 0.0], 144.0),
             ("C122-R100-Y011", 100.0, [0.0, 1.0, 1.0], 144.0)]
    rv = None
    try:
        import rv131_rv_element as rv
    except ImportError:
        pass
    models = []
    for name, R, y, kx in cands:
        SPRINGS[0] = (3, kx)
        el, K, u = exact_model(R, y, 110)
        _, _, u140 = exact_model(R, y, 140)
        umax = max(abs(v) for v in u140.values())
        agree = max(abs(u[q] - u140[q]) for q in FREE) / umax
        # an exact zero (a symmetry of the y = (+-1, 0, 0) plane) shows as rounding noise
        zero = {q for q in FREE if abs(u140[q]) < D("1e-95") * umax}
        Kf = Lb.reduce(K, FREE)
        c_sq = Lb.cond1_sqrt_equilibrated(Kf)
        c_rx = Lb.cond1_radix(Kf)
        Kcr = Lb.round_matrix(el["K"])
        ucr = Lb.solve_exact(Lb.assemble(2, [(0, 1, Kcr)], SPRINGS), LOADS, FREE)
        crK, crK_at = Lb.actual_ratio(NODES, u, ucr)
        ucr2 = Lb.solve_exact(Lb.assemble(2, [(0, 1, Kcr)], SPRINGS, add_springs="b64"), LOADS, FREE)
        crK2, _ = Lb.actual_ratio(NODES, u, ucr2)
        xr = None
        if rv is not None:
            with localcontext() as cx:
                cx.prec = 90
                Kq, geo_q, _ = rv.element_global([1.0, 2.0, 2.0], R, y, S["E"], S["G"], S["A"], S["I"], S["J"], 1.0, 1.0)
                uq = Lb.solve_exact(Lb.assemble(2, [(0, 1, Kq)], SPRINGS), LOADS, FREE)
                xr = float(Lb.actual_ratio(NODES, u, uq)[0])
        geo = el["geo"]
        emu = emulate(R, y, el, u, FORMATIONS)
        print("%s  R %g  y %s  k_X %g  phi %s rad  L %s" % (name, R, y, kx, C.sci(geo["phi"], 20), C.sci(geo["L"], 20)))
        print("  u_int (exact, 20 digits; 110 vs 140 digits agree to %.1e of max|u|; RV131 quadrature element: %s of the criterion)" % (
            float(agree), "%.1e" % xr if xr is not None else "n/a"))
        for q in FREE:
            print("    dof %2d  %s" % (q, "0 (exact zero; |u| < 1e-95 max|u| at 110 and 140 digits)" if q in zero else Lb.sig20(u[q])))
        print("  cond1 equilibrated: sqrt-diag %.4e, FK radix %.4e (gate %.4e; margin x%.2f)" % (
            float(c_sq), float(c_rx), GATE, GATE / float(c_rx)))
        print("  crK %.4f at dof %s (springs added exactly) | %.4f (springs added in binary64, as SA does)" % (
            float(crK), crK_at, float(crK2)))
        for mode in ("dense", "sparse"):
            acts = [v["actual"] for k, v in emu.items() if k.endswith(mode)]
            print("  %s: actual > 0.55 in %d/7 formations, > 1 in %d/7, min %.3f" % (mode, sum(a > 0.55 for a in acts), sum(a > 1 for a in acts), min(acts)))
        for key, v in emu.items():
            print("  emulation %-20s actual %.4f  K-D5 trigger %.4f (EF %.4f, EF/actual-1 %.1e)  M31a trigger %.4f" % (
                key, v["actual"], v["kd5_trigger"], v["kd5_trigger"] / 2, v["kd5_trigger"] / 2 / v["actual"] - 1 if v["actual"] else float("nan"),
                v["m31a_trigger"]))
        print()
        models.append(dict(
            id=name,
            description="F122 (kd5_models.rs F122 at 10b70036ef) with its member realized as a B1 bend",
            nodes=NODES, member=dict(i=0, j=1, bend_R=R, y_reference=y, k_in=1.0, k_out=1.0),
            section=S, rigid=RIGID, springs=[list(t) for t in SPRINGS], loads=[[q, v] for q, v in LOADS.items()],
            derived=dict(phi=C.sci(geo["phi"], 20), L=C.sci(geo["L"], 20), s=C.sci(geo["s"], 20)),
            u_int={str(q): ("0" if q in zero else Lb.sig20(u[q])) for q in FREE},
            u_int_binary64={str(q): ("0.0" if q in zero else repr(float(u[q]))) for q in FREE},
            cond1_sqrt_equilibrated_exact=C.sci(c_sq, 4), cond1_radix_scaled_exact=C.sci(c_rx, 4),
            gate_1_over_sqrt_eps=C.sci(D(GATE), 6),
            crK_springs_exact=C.sci(crK, 4), crK_springs_binary64=C.sci(crK2, 4), crK_at_dof=crK_at,
            check_110_vs_140_digits_over_max_u=C.sci(agree, 2),
            check_rv131_quadrature_element_ratio=("%.2e" % xr) if xr is not None else None,
            emulations_labelled={k: {kk: (round(vv, 6) if isinstance(vv, float) else vv) for kk, vv in v.items()} for k, v in emu.items()},
        ))
    SPRINGS[0] = (3, 144.0)
    parts["curved_true_positive_candidates"] = models

    # ---- exploratory sweep (radius and plane) for a more robust candidate
    print("sweep: R x plane angle (y = cos a * n1 + sin a * n2, n1 = (8,-2,-2)/sqrt72 normal to d in F122's own plane, n2 = (0,1,-1)/sqrt2)")
    n1 = [8.0 / math.sqrt(72.0), -2.0 / math.sqrt(72.0), -2.0 / math.sqrt(72.0)]
    n2 = [0.0, 1.0 / math.sqrt(2.0), -1.0 / math.sqrt(2.0)]
    sweep = []
    for R in (3.0, 5.0, 10.0, 20.0, 30.0, 50.0, 100.0, 300.0):
        for adeg in (0, 45, 90, 135, 180, 225, 270, 315):
            a = math.radians(adeg)
            y = [math.cos(a) * n1[k] + math.sin(a) * n2[k] for k in range(3)]
            getcontext().prec = 60
            el, K, u = exact_model(R, y, 60)
            Kf = Lb.reduce(K, FREE)
            c_rx = float(Lb.cond1_radix(Kf))
            Kcr = Lb.round_matrix(el["K"])
            crK = float(Lb.actual_ratio(NODES, u, Lb.solve_exact(Lb.assemble(2, [(0, 1, Kcr)], SPRINGS, add_springs="b64"), LOADS, FREE))[0])
            emu = emulate(R, y, el, u)
            mins = {mode: min(v["actual"] for k, v in emu.items() if k.endswith(mode)) for mode in ("dense", "sparse")}
            maxs = {mode: max(v["actual"] for k, v in emu.items() if k.endswith(mode)) for mode in ("dense", "sparse")}
            ma = max(v["m31a_trigger"] for v in emu.values())
            row = (R, adeg, c_rx, crK, mins["dense"], maxs["dense"], mins["sparse"], maxs["sparse"], ma)
            sweep.append(row)
            print("  R %-5g a %3d  cond_radix %.2e  crK %.3f  emulated actual dense [%.3f, %.3f] sparse [%.3f, %.3f]  max M31a trigger %.3f" % row)
    getcontext().prec = 110
    parts["curved_true_positive_sweep_labelled"] = [dict(R=r[0], plane_angle_deg=r[1], cond1_radix=round(r[2], -4), crK=round(r[3], 4),
                                                         emulated_actual_dense=[round(r[4], 4), round(r[5], 4)],
                                                         emulated_actual_sparse=[round(r[6], 4), round(r[7], 4)],
                                                         max_m31a_trigger=round(r[8], 4)) for r in sweep]
    with open(os.path.join(HERE, "parts", "i1_curved122.json"), "w") as fh:
        json.dump(parts, fh, indent=1)


if __name__ == "__main__":
    main()
