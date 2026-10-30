"""T4-I6 R01 item 6: P4's diagonal-scaled criterion |dK_ij| <= 1e-9 sqrt(K_ii K_jj) (RV2 S-4) on the
49 frozen B2 cases (round-00 JSON 'cases', K_global at 20 digits), and the O7 near-pi form.

Per case (EXACT, from the frozen K): max|K|, min K_ii, max|K| / min K_ii, the tightest diagonal-scaled
tolerance relative to P4's matrix-scale tolerance, min_ij sqrt(K_ii K_jj) / max|K|, and the PSD check
|K_ij| <= sqrt(K_ii K_jj). Labelled binary64 emulations: the diagonal-scaled error of a stable element
(gl16, cr) with the binary64-L near-pi geometry and with O7 (4R^2 - |d|^2 formed exactly, rounded once),
and of a partly stable element (closed_stable) - showing what the normative criterion discriminates.

usage: python -I -B i6_p4.py rerun00/u1_reference_cases.json > i6_p4.stdout.txt  (writes parts/i6_p4.json)
"""
import json
import os
import sys
from collections import defaultdict
from decimal import Decimal as D, getcontext

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import r01_lib as Lb  # noqa: E402

C = Lb.C


def main(u1_path):
    getcontext().prec = 60
    doc = json.load(open(u1_path))
    rows = []
    for case in doc["cases"]:
        K = [[D(v) for v in row] for row in case["K_global"]]
        inp = case["inputs"]
        sec = dict(E=inp["E"], G=inp["G"], A=inp["A"], I=inp["I"], J=inp["J"])
        mx = max(abs(v) for row in K for v in row)
        dg = [K[i][i] for i in range(12)]
        mind = min(dg)
        tight = min((dg[i] * dg[j]).sqrt() for i in range(12) for j in range(12)) / mx
        psd = max(abs(K[i][j]) / (dg[i] * dg[j]).sqrt() for i in range(12) for j in range(12))
        d, R, y = inp["d"], inp["R"], inp["y_reference"]
        kin, kout = case["k_in"], case["k_out"]
        Floc = Lb.exact_local_F([0.0, 0.0, 0.0], d, R, y, sec, kin, kout)
        em = {}
        for tag, fm, o7 in (("gl16", "gl16", False), ("gl16+O7", "gl16", True), ("cr+O7", "cr", True), ("closed_stable+O7", "closed_stable", True)):
            Kb, _ = Lb.curved_b64(d, R, y, sec, kin, kout, F_mode=fm, near_pi_exact=o7, F_exact_local=Floc)
            ds = max(abs(Lb.dec(Kb[i][j]) - K[i][j]) / (dg[i] * dg[j]).sqrt() for i in range(12) for j in range(12))
            ms = max(abs(Lb.dec(Kb[i][j]) - K[i][j]) for i in range(12) for j in range(12)) / mx
            em[tag] = (float(ds), float(ms))
        rows.append(dict(id=case["id"], label=case["angle_label"], plane=case["plane"], variant=case["variant"],
                         max_abs_K=float(mx), min_diag=float(mind), ratio=float(mx / mind), tightest=float(tight),
                         psd_max=float(psd), emu=em))
    by = defaultdict(list)
    for r in rows:
        by[r["label"]].append(r)
    print("P4 diagonal-scaled criterion on the 49 frozen cases (EXACT columns from the frozen K; emulation columns labelled)")
    print("%-14s %3s %12s %14s %10s | %-28s %-28s %-28s %-28s" % ("angle", "n", "max|K|/minKii", "min sqrt(KiiKjj)/max|K|", "PSD max",
                                                             "gl16 diag/matrix", "gl16+O7 diag/matrix", "cr+O7 diag/matrix", "closed_stable+O7 diag/matrix"))
    summary = []
    for lab, rs in by.items():
        def mxe(tag, k):
            return max(r["emu"][tag][k] for r in rs)
        line = "%-14s %3d %12.3e %14.3e %10.6f | %.2e / %.2e          %.2e / %.2e          %.2e / %.2e          %.2e / %.2e" % (
            lab, len(rs), max(r["ratio"] for r in rs), min(r["tightest"] for r in rs), max(r["psd_max"] for r in rs),
            mxe("gl16", 0), mxe("gl16", 1), mxe("gl16+O7", 0), mxe("gl16+O7", 1), mxe("cr+O7", 0), mxe("cr+O7", 1),
            mxe("closed_stable+O7", 0), mxe("closed_stable+O7", 1))
        print(line)
        summary.append(dict(angle_label=lab, n=len(rs), max_maxK_over_minKii="%.3e" % max(r["ratio"] for r in rs),
                            min_sqrtKiiKjj_over_maxK="%.3e" % min(r["tightest"] for r in rs),
                            implied_absolute_tolerance_min_over_P4="%.3e" % min(r["tightest"] for r in rs),
                            psd_max_abs_Kij_over_sqrtKiiKjj="%.6f" % max(r["psd_max"] for r in rs),
                            emulated_diag_scaled_error_labelled={t: "%.2e" % mxe(t, 0) for t in ("gl16", "gl16+O7", "cr+O7", "closed_stable+O7")},
                            emulated_matrix_scaled_error_labelled={t: "%.2e" % mxe(t, 1) for t in ("gl16", "gl16+O7", "cr+O7", "closed_stable+O7")}))
    with open(os.path.join(HERE, "parts", "i6_p4.json"), "w") as fh:
        json.dump({"p4_diag_scaled": dict(per_angle=summary,
                                          per_case=[dict(id=r["id"], max_abs_K="%.6e" % r["max_abs_K"], min_diag="%.6e" % r["min_diag"],
                                                         min_sqrtKiiKjj_over_maxK="%.3e" % r["tightest"]) for r in rows])}, fh, indent=1)


if __name__ == "__main__":
    main(sys.argv[1])
