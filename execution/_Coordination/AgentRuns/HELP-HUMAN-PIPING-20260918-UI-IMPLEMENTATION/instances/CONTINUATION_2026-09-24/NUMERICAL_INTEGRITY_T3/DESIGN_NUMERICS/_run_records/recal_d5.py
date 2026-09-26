#!/usr/bin/env python3
"""D1 revision 5, D5C-4: recalibration of the D-5 trigger with the exact-residual EF on the
product-faithful path (standard library only; no product code built or run).

Usage (from T3/):
  nice -n 19 python3 DESIGN_NUMERICS/_run_records/recal_d5.py REFERENCES/references.json <v1_dir> <out.json> [max_members]

<v1_dir> is a scratch folder holding V1's probe `REVIEW/_run_records/d5_check/probe_d5_check.py.txt`
copied to `probe_d5_check.py` (V1's file, cited, not copied into this record; its sha256 is recorded
in the output). From it this script imports, unchanged, the product-faithful pieces:
  product_frame, local_K (binary64 and exact), T12, TtKT, exact_frame, solve_product (prepare, dense
  cholesky or RCM profile LDL^T, estimate_rcond, residual64 and refinement), apply_inverse.
The R1 parsing helpers `num` and `section_f64`-style inputs are D1's own (sweep_d5_r1.py).

Per R1 case (explicit members <= max_members, nodal loads only, global-axis springs, not RF-MECH):
  K_rep  binary64: product frame, local_K, two-stage T^T(K T), element entries added with += in member
         order, springs added with += afterwards (PP:1386-1390 as V1 reads it);
  K_int  intended: exact frame (320-bit square roots), rounded to 256-bit dyadic, exact local
         coefficients from the same binary64 E, G, A, I, J, exact T^T K T, exact assembly, exact springs;
  f      the product's binary64 fold in authored order (pre-S11-F), and the exact ledger sum;
  u      solve_product(m, mode) for mode in (dense, sparse): quality Passed iff rcond >= sqrt(eps);
  EF     w = K~^-1 rho with rho_i = f_i (ledger: exact sum of the authored terms) - sum_j K_int[i][j] u_j, one exact sum per free row, rounded
         once to binary64 (D5C-1), applied with the same mode's binary64 factor;
  actual |u_i - expected_i| / (1e-9 max(|expected_i|, R1 class scale)), nodal rows only.
Trigger scales evaluated (nodal kinds translation and rotation):
  coupled    S*_kind per body from the binary64 u (DESIGN s4.1.6.1 items 4-6);
  uncoupled  S(kind) = largest |q| of that kind over the case's nodal rows (R1's class-scale form).
EF_ratio_coupled_with_folded_f repeats EF with the binary64-folded f (what the kernel receives before S11-F).
Also the condition trigger (a1): c * cond * 2^-53 > 1e-9 for c in (1, 8), cond = 1/rcond.
Outputs per case and mode, and a summary of misses and Passed-band false positives per factor.
"""
import hashlib
import json
import math
import os
import sys
from fractions import Fraction as Fr

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from sweep_d5_r1 import num, yref_for, DOF  # noqa: E402  (D1's own helpers)

V1_DIR = sys.argv[2]
sys.path.insert(0, V1_DIR)
import probe_d5_check as p  # noqa: E402  (V1's product-faithful emulation, cited)

U = 2.0 ** -53
SQRT_EPS = math.sqrt(2.0 ** -52)
FACTORS = (1.0, 1.0 + 2.0 ** -20, 2.0, 8.0)


def dyadic(x, bits=256):
    x = Fr(x)
    if x == 0:
        return x
    e = x.numerator.bit_length() - x.denominator.bit_length()
    s = bits - e
    return Fr(round(x * Fr(2) ** s)) / Fr(2) ** s


def section(s):
    e = float(num(s["E"]))
    g = float(num(s["G"])) if "G" in s else e / (2.0 * (1.0 + float(num(s["nu"]))))
    od, idd = float(num(s["OD"])), float(num(s["ID"]))
    return p.section(od, idd, e, g)


def bodies(names, members):
    parent = {n: n for n in names}

    def find(a):
        while parent[a] != a:
            parent[a] = parent[parent[a]]
            a = parent[a]
        return a
    for (_, a, b, _) in members:
        parent[find(a)] = find(b)
    return {n: find(n) for n in names}


def run_case(cid, c, maxm):
    m = c["model"]
    if "members" not in m:
        return None, "generated model"
    if len(m["members"]) > maxm:
        return None, "too many members for the Python emulation"
    if "member_uniform_loads_N_per_m_global" in m:
        return None, "element loads (W1b)"
    if c.get("flags", {}).get("needs_directional_spring"):
        return None, "directional spring"
    names = list(m["nodes_m"])
    idx = {nm: i for i, nm in enumerate(names)}
    xyz = {nm: [float(num(v)) for v in m["nodes_m"][nm]] for nm in names}
    n = 6 * len(names)
    K = [[0.0] * n for _ in range(n)]
    Kx = [[Fr(0)] * n for _ in range(n)]
    for (mid, a, b, sid) in m["members"]:
        sec = section(m["sections"][sid])
        xi, xj = tuple(xyz[a]), tuple(xyz[b])
        yr = yref_for([xj[k] - xi[k] for k in range(3)])
        Rp, Lp = p.product_frame(xi, xj, yr)
        Ke = p.TtKT(p.local_K(sec, Lp), p.T12(Rp, 0.0), False)
        Rx, Lx = p.exact_frame(xi, xj, yr)
        Rx = [[dyadic(v) for v in row] for row in Rx]
        Lx = dyadic(Lx)
        Kex = p.TtKT(p.local_K(sec, Lx, exact=True), p.T12(Rx, Fr(0)), True)
        base = [6 * idx[a] + k for k in range(6)] + [6 * idx[b] + k for k in range(6)]
        for i in range(12):
            for j in range(12):
                K[base[i]][base[j]] += Ke[i][j]
                Kx[base[i]][base[j]] += Kex[i][j]
    restrained = set()
    for nm, sup in m["supports"].items():
        for dname in sup.get("rigid", []):
            restrained.add(6 * idx[nm] + DOF[dname])
        for sp in sup.get("springs", []):
            dirv = [num(v) for v in sp["direction"]]
            if sorted(abs(v) for v in dirv) != [0, 0, 1]:
                return None, "non-axis spring"
            axis = [abs(v) for v in dirv].index(1)
            dof = 6 * idx[nm] + axis + (3 if sp["kind"] == "rotation" else 0)
            k = float(num(sp["k"]))
            K[dof][dof] += k
            Kx[dof][dof] += Fr(k)
    f = [0.0] * n
    fx = [Fr(0)] * n
    for nm, ld in m.get("loads", {}).items():
        for key, off in (("F", 0), ("M", 3)):
            for a_, v in enumerate(ld.get(key, [])):
                f[6 * idx[nm] + off + a_] += float(num(v))
                fx[6 * idx[nm] + off + a_] += Fr(float(num(v)))
    for nm, dname, v in m.get("load_contributions_in_authored_order", []):
        f[6 * idx[nm] + DOF[dname]] += float(num(v))
        fx[6 * idx[nm] + DOF[dname]] += Fr(float(num(v)))
    free = [i for i in range(n) if i not in restrained]
    model = dict(n=n, free=free, K_rep=K, f=f)
    body = bodies(names, m["members"])
    scales = {k: Fr(v["value"]) for k, v in c["scales"].items()}
    basis = "represented" if c.get("basis") == "represented" else "intended"
    rows = c["expected_represented"] if basis == "represented" and c.get("expected_represented") else c["expected"]
    nodal = []
    for row in rows:
        key, e, cls = row[0], num(row[1]), row[2]
        parts = key.split(".")
        if parts[0] not in ("u", "th") or parts[1] not in idx:
            continue
        nodal.append((key, 6 * idx[parts[1]] + DOF[parts[2]], e, cls, parts[1]))
    out = {"family": c["family"], "members": len(m["members"]), "basis": basis, "n_free": len(free),
           "load_fold_inexact": any(Fr(f[i]) != fx[i] for i in range(n))}
    for mode in ("dense", "sparse"):
        sol = p.solve_product(model, mode)
        if sol is None or sol.get("outcome") == "unresolved":
            out[mode] = {"outcome": "factor failed" if sol is None else "unresolved"}
            continue
        u = sol["u"]
        uF = [Fr(v) for v in u]
        rho = []
        for i in free:
            s = fx[i]
            row = Kx[i]
            for j in range(n):
                if u[j] != 0.0 and row[j] != 0:
                    s -= row[j] * uF[j]
            rho.append(s)
        rho_fold = []
        for r_, i in zip(rho, free):
            rho_fold.append(r_ - fx[i] + Fr(f[i]))
        w = [0.0] * n
        wf = [0.0] * n
        for r_, v in zip(free, p.apply_inverse(sol, rho)):
            w[r_] = v
        for r_, v in zip(free, p.apply_inverse(sol, rho_fold)):
            wf[r_] = v
        # trigger scales
        S_unc = {"translation": 0.0, "rotation": 0.0}
        Sb = {}
        for (key, dof, e, cls, nd) in nodal:
            kind = "translation" if dof % 6 < 3 else "rotation"
            S_unc[kind] = max(S_unc[kind], abs(u[dof]))
            b = body[nd]
            Sb.setdefault(b, {"translation": 0.0, "rotation": 0.0})
            Sb[b][kind] = max(Sb[b][kind], abs(u[dof]))
        Sc = {}
        for b, sk in Sb.items():
            pts = [xyz[nm] for nm in names if body[nm] == b]
            d = [max(q[k] for q in pts) - min(q[k] for q in pts) for k in range(3)]
            Lb = math.sqrt((d[0] * d[0] + d[1] * d[1]) + d[2] * d[2])
            tr, ro = sk["translation"], sk["rotation"]
            if Lb > 0:
                Sc[b] = {"translation": max(tr, Lb * ro), "rotation": max(ro, tr / Lb)}
            else:
                Sc[b] = dict(sk)
        worst = {"actual": 0.0, "EF_R1": 0.0, "EF_coupled": 0.0, "EF_uncoupled": 0.0, "EFfold_coupled": 0.0}
        at = None
        track = []
        for (key, dof, e, cls, nd) in nodal:
            if dof not in free:
                continue
            kind = "translation" if dof % 6 < 3 else "rotation"
            sc = max(abs(e), scales[cls])
            if sc == 0:
                continue
            act = float(abs(uF[dof] - e) / sc) / 1e-9
            efr = abs(w[dof]) / (float(sc) * 1e-9)
            if act > worst["actual"]:
                worst["actual"], at = act, key
            worst["EF_R1"] = max(worst["EF_R1"], efr)
            den_c = max(abs(u[dof]), Sc[body[nd]][kind]) * 1e-9
            den_u = max(abs(u[dof]), S_unc[kind]) * 1e-9
            if den_c > 0:
                worst["EF_coupled"] = max(worst["EF_coupled"], abs(w[dof]) / den_c)
                worst["EFfold_coupled"] = max(worst["EFfold_coupled"], abs(wf[dof]) / den_c)
            if den_u > 0:
                worst["EF_uncoupled"] = max(worst["EF_uncoupled"], abs(w[dof]) / den_u)
            if act > 1e-2:
                track.append(efr / act)
        cond = 1.0 / sol["rcond"]
        out[mode] = {"outcome": sol["outcome"], "rcond": sol["rcond"], "cond": cond,
                     "refinement_attempts": sol["attempts"], "worst_actual_ratio": worst["actual"], "at": at,
                     "EF_ratio_R1_scale": worst["EF_R1"], "EF_ratio_coupled_Sstar": worst["EF_coupled"],
                     "EF_ratio_uncoupled_S": worst["EF_uncoupled"],
                     "EF_ratio_coupled_with_folded_f": worst["EFfold_coupled"],
                     "EF_over_actual_min": min(track) if track else None,
                     "EF_over_actual_max": max(track) if track else None,
                     "a1_c1": cond * U > 1e-9, "a1_c8": 8 * cond * U > 1e-9}
    return out, None


def summarise(cases):
    s = {}
    for scale in ("EF_ratio_coupled_Sstar", "EF_ratio_uncoupled_S", "EF_ratio_coupled_with_folded_f"):
        for fac in FACTORS:
            miss, fp, fire_passed, breaches = [], [], [], []
            for cid, r in cases.items():
                for mode in ("dense", "sparse"):
                    x = r.get(mode, {})
                    if x.get("outcome") != "Passed":
                        continue
                    fires = fac * x[scale] > 1.0
                    breach = x["worst_actual_ratio"] > 1.0
                    if breach:
                        breaches.append((cid, mode))
                        if not fires:
                            miss.append((cid, mode, x["worst_actual_ratio"], x[scale]))
                    elif fires:
                        fp.append((cid, mode, x["worst_actual_ratio"], x[scale]))
            s[f"{scale}@{fac!r}"] = {"passed_breaches": len(breaches), "missed": miss,
                                    "false_positives_in_passed_band": fp, "fp_count": len(fp)}
    for c in (1, 8):
        fp = [(cid, mode, r[mode]["worst_actual_ratio"]) for cid, r in cases.items() for mode in ("dense", "sparse")
              if r.get(mode, {}).get("outcome") == "Passed" and r[mode][f"a1_c{c}"] and r[mode]["worst_actual_ratio"] <= 1.0]
        miss = [(cid, mode) for cid, r in cases.items() for mode in ("dense", "sparse")
                if r.get(mode, {}).get("outcome") == "Passed" and not r[mode][f"a1_c{c}"] and r[mode]["worst_actual_ratio"] > 1.0]
        s[f"a1_c{c}"] = {"false_positives_in_passed_band": fp, "fp_count": len(fp), "missed": miss}
    tr = [(r[m]["EF_over_actual_min"], r[m]["EF_over_actual_max"]) for r in cases.values() for m in ("dense", "sparse")
          if isinstance(r.get(m), dict) and r[m].get("EF_over_actual_min") is not None]
    s["EF_R1_over_actual_rows_above_1e-2"] = {"min": min(a for a, _ in tr), "max": max(b for _, b in tr), "case_modes": len(tr)}
    return s


def main():
    ref_path, out_path = sys.argv[1], sys.argv[3]
    maxm = int(sys.argv[4]) if len(sys.argv) > 4 else 12
    ref = json.load(open(ref_path))["cases"]
    v1 = os.path.join(V1_DIR, "probe_d5_check.py")
    out = {"python": sys.version.split()[0],
           "references_sha256": hashlib.sha256(open(ref_path, "rb").read()).hexdigest(),
           "v1_probe_sha256": hashlib.sha256(open(v1, "rb").read()).hexdigest(),
           "max_members": maxm, "cases": {}, "skipped": {}}
    for cid, c in ref.items():
        if c["family"] == "RF-MECH" or not c.get("expected"):
            out["skipped"][cid] = "mechanism or no expected values"
            continue
        r, why = run_case(cid, c, maxm)
        if r is None:
            out["skipped"][cid] = why
        else:
            out["cases"][cid] = r
    out["summary"] = summarise(out["cases"])
    json.dump(out, open(out_path, "w"), indent=1)
    print(json.dumps({"cases": len(out["cases"]), "skipped": len(out["skipped"]), "summary": out["summary"]}, indent=1))


if __name__ == "__main__":
    main()
