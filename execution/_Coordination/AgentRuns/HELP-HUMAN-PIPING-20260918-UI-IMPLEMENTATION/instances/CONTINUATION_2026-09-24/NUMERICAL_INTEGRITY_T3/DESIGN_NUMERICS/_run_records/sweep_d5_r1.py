#!/usr/bin/env python3
"""D1 D-5 sweep over R1's frozen references (standard library only; no product code).

Usage (from T3/): python3 sweep_d5_r1.py REFERENCES/references.json <out.json> [max_members]

For every R1 case with explicit members (up to max_members, default 100), nodal loads only,
global-axis springs and a unique solution, it emulates main's ordinary binary64 path with D1's
formation emulation (binary64 local matrix, binary64 frame, two-stage T^T(K T), binary64
assembly, main's binary64 load fold, binary64 dense LDL; the element and frame code is D1's
probe_skew_precision.py, parameterised here by section) and compares every published nodal
translation and rotation with R1's expected value under the unchanged predicate
|obs - exp| <= 1e-9 * max(|exp|, scale), R1's published class scales.

It also evaluates two trigger estimators per case (both on the same emulated factor):
  EB  bound-based: w = K^-1 (B |u|), B = the product's per-element formation roundoff bound
      (FK transform_roundoff form) times a stage factor 4 (local coefficients, frame rounding,
      assembly); estimated ratio per component |w_i| / (1e-9 * max(|exp|, scale));
  EF  first-order: w = K^-1 (dK u), dK = K_rep - K_int formed exactly (Fraction) from the same
      binary64 primitives and exact geometry, the matrix W1 would form; |w_i| as above.
Not emulated: M03's Passed/Sensitive verdict, W2 scaling, member and reaction quantities, and
product-specific orderings, so this is evidence about the class, not a product result.
"""
from fractions import Fraction as Fr
import json
import math
import sys

sys.path.insert(0, ".")
import probe_skew_precision as d1  # noqa: E402

DOF = {"UX": 0, "UY": 1, "UZ": 2, "RX": 3, "RY": 4, "RZ": 5}


def num(x):
    x = str(x)
    if "*2^" in x:
        m, k = x.split("*2^")
        return Fr(m) * Fr(2) ** int(k)
    if "/" in x:
        a, b = x.split("/")
        return Fr(a) / Fr(b)
    return Fr(x)


def section_f64(s):
    e = float(num(s["E"]))
    g = float(num(s["G"])) if "G" in s else e / (2.0 * (1.0 + float(num(s["nu"]))))
    od, idd = float(num(s["OD"])), float(num(s["ID"]))
    a = math.pi * (od * od - idd * idd) / 4.0
    iy = math.pi * (od ** 4 - idd ** 4) / 64.0
    return dict(E=e, G=g, A=a, Iy=iy, Iz=iy, J=2.0 * iy)


def local_K(L, s):
    e, g, a, iy, iz, j = s["E"], s["G"], s["A"], s["Iy"], s["Iz"], s["J"]
    L2, L3 = L * L, L * L * L
    K = [[0.0] * 12 for _ in range(12)]

    def sym(r, c, v):
        K[r][c] = v
        K[c][r] = v
    ax, tor = e * a / L, g * j / L
    K[0][0] = K[6][6] = ax
    sym(0, 6, -ax)
    K[3][3] = K[9][9] = tor
    sym(3, 9, -tor)
    for (v, th, ei, sg) in ((1, 5, iz, 1.0), (2, 4, iy, -1.0)):
        k12, k6, k4, k2 = 12 * e * ei / L3, 6 * e * ei / L2, 4 * e * ei / L, 2 * e * ei / L
        K[v][v] = K[v + 6][v + 6] = k12
        sym(v, v + 6, -k12)
        sym(v, th, sg * k6)
        sym(v, th + 6, sg * k6)
        sym(v + 6, th, -sg * k6)
        sym(v + 6, th + 6, -sg * k6)
        K[th][th] = K[th + 6][th + 6] = k4
        sym(th, th + 6, k2)
    return K


def exact_local_K(L, s):
    """Exact rational local matrix from the same binary64 section values and exact length L."""
    e, g, a, iy, j = (Fr(s[k]) for k in ("E", "G", "A", "Iy", "J"))
    K = [[Fr(0)] * 12 for _ in range(12)]

    def sym(r, c, v):
        K[r][c] = v
        K[c][r] = v
    ax, tor = e * a / L, g * j / L
    K[0][0] = K[6][6] = ax
    sym(0, 6, -ax)
    K[3][3] = K[9][9] = tor
    sym(3, 9, -tor)
    for (v, th, sg) in ((1, 5, 1), (2, 4, -1)):
        k12, k6, k4, k2 = 12 * e * iy / L ** 3, 6 * e * iy / L ** 2, 4 * e * iy / L, 2 * e * iy / L
        K[v][v] = K[v + 6][v + 6] = k12
        sym(v, v + 6, -k12)
        sym(v, th, sg * k6)
        sym(v, th + 6, sg * k6)
        sym(v + 6, th, -sg * k6)
        sym(v + 6, th + 6, -sg * k6)
        K[th][th] = K[th + 6][th + 6] = k4
        sym(th, th + 6, k2)
    return K


def yref_for(d):
    ax = [abs(c) for c in d]
    return (0.0, 1.0, 0.0) if ax[0] == 0 and ax[1] == 0 else (0.0, 0.0, 1.0)


def gamma(m):
    u = 2.0 ** -53
    return m * u / (1 - m * u)


def bound(Kl, T):
    g = gamma(24)
    temp = [[sum(Kl[i][k] * T[k][j] for k in range(12)) for j in range(12)] for i in range(12)]
    mag = [[sum(abs(Kl[i][k] * T[k][j]) for k in range(12)) for j in range(12)] for i in range(12)]
    B = [[0.0] * 12 for _ in range(12)]
    for i in range(12):
        for j in range(12):
            inh = sum(abs(T[k][i]) * mag[k][j] for k in range(12))
            sec = sum(abs(T[k][i]) * abs(temp[k][j]) for k in range(12))
            B[i][j] = 4.0 * g * (inh / (1 - g) + sec)
    return B


def exact_frame(xi, xj, yref):
    """Exact-geometry frame where the axes are rational; otherwise 200-bit rounded (as in D1's probe)."""
    R, L = d1.element_frame(xi, xj, yref, d1.A(200))
    return R, L


def run_case(cid, c, max_members):
    m = c["model"]
    if "members" not in m:
        return None, "generated model"
    if len(m["members"]) > max_members:
        return None, "too many members for the dense emulation"
    if "member_uniform_loads_N_per_m_global" in m:
        return None, "element loads (W1b)"
    if c.get("flags", {}).get("needs_directional_spring"):
        return None, "directional spring"
    names = list(m["nodes_m"])
    idx = {nm: i for i, nm in enumerate(names)}
    xyz = {nm: [num(v) for v in m["nodes_m"][nm]] for nm in names}
    n = 6 * len(names)
    K = [[0.0] * n for _ in range(n)]
    Kx = [[Fr(0)] * n for _ in range(n)]
    Bg = [[0.0] * n for _ in range(n)]
    for (mid, a, b, sid) in m["members"]:
        s = section_f64(m["sections"][sid])
        xi = tuple(float(v) for v in xyz[a])
        xj = tuple(float(v) for v in xyz[b])
        d = [xj[k] - xi[k] for k in range(3)]
        yr = yref_for(d)
        R, L = d1.f64_frame(xi, xj, yr)
        Kl = local_K(L, s)
        Kg, T = d1.f64_global_K(Kl, R)
        Bl = bound(Kl, T)
        Rx, Lx = exact_frame(tuple(xyz[a]), tuple(xyz[b]), tuple(Fr(v) for v in yr))
        Tx = d1.transform12(Rx)
        Klx = exact_local_K(Lx, s)
        Kgx = d1.matmul(d1.transpose(Tx), d1.matmul(Klx, Tx, d1.A(None)), d1.A(None))
        mp = d1.dofmap(idx[a], idx[b])
        for i in range(12):
            for j in range(12):
                K[mp[i]][mp[j]] += Kg[i][j]
                Kx[mp[i]][mp[j]] += Kgx[i][j]
                Bg[mp[i]][mp[j]] += Bl[i][j]
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
    for nm, ld in m.get("loads", {}).items():
        for key, off in (("F", 0), ("M", 3)):
            for a, v in enumerate(ld.get(key, [])):
                f[6 * idx[nm] + off + a] += float(num(v))
    for nm, dname, v in m.get("load_contributions_in_authored_order", []):
        f[6 * idx[nm] + DOF[dname]] += float(num(v))
    free = [i for i in range(n) if i not in restrained]
    Kf = [[K[a][b] for b in free] for a in free]
    x, err = d1.f64_ldl_solve(Kf, [f[a] for a in free])
    if err:
        return None, "binary64 LDL: " + err
    u = [0.0] * n
    for a, v in zip(free, x):
        u[a] = v
    Bu = [sum(Bg[i][j] * abs(u[j]) for j in range(n)) for i in range(n)]
    dKu = [float(sum((Fr(K[i][j]) - Kx[i][j]) * Fr(u[j]) for j in range(n) if u[j] != 0)) for i in range(n)]
    wb, _ = d1.f64_ldl_solve(Kf, [Bu[a] for a in free])
    wf, _ = d1.f64_ldl_solve(Kf, [dKu[a] for a in free])
    WB, WF = [0.0] * n, [0.0] * n
    for a, vb, vf in zip(free, wb, wf):
        WB[a], WF[a] = abs(vb), abs(vf)
    # condition number of the radix-equilibrated reduced matrix, as prepare_structural scales it
    ex = [-(math.frexp(Kf[r][r])[1] - 1) // 2 for r in range(len(free))]
    Af = [[math.ldexp(Kf[r][q], ex[r] + ex[q]) for q in range(len(free))] for r in range(len(free))]
    norm_a = max(sum(abs(Af[r][q]) for r in range(len(free))) for q in range(len(free)))
    norm_inv = 0.0
    for q in range(len(free)):
        col, _ = d1.f64_ldl_solve(Af, [1.0 if r == q else 0.0 for r in range(len(free))])
        norm_inv = max(norm_inv, sum(abs(v) for v in col))
    cond1 = norm_a * norm_inv
    scales = {k: Fr(v["value"]) for k, v in c["scales"].items()}
    basis = "represented" if c.get("basis") == "represented" else "intended"
    rows = c["expected_represented"] if basis == "represented" and c.get("expected_represented") else c["expected"]
    worst = {"actual": 0.0, "EB": 0.0, "EF": 0.0}
    at = None
    for row in rows:
        key, e, cls = row[0], num(row[1]), row[2]
        parts = key.split(".")
        if parts[0] not in ("u", "th") or parts[1] not in idx:
            continue
        dof = 6 * idx[parts[1]] + DOF[parts[2]]
        sc = max(abs(e), scales[cls])
        if sc == 0:
            continue
        den = float(sc) * 1e-9
        act = float(abs(Fr(u[dof]) - e)) / den
        if act > worst["actual"]:
            worst["actual"], at = act, key
        worst["EB"] = max(worst["EB"], WB[dof] / den)
        worst["EF"] = max(worst["EF"], WF[dof] / den)
    return {"family": c["family"], "members": len(m["members"]), "basis": basis, "worst_actual_ratio": worst["actual"],
            "at": at, "EB_ratio": worst["EB"], "EF_ratio": worst["EF"], "cond1_equilibrated": cond1,
            "non_permutation": any(sorted(abs(float(v)) for v in [xyz[b][k] - xyz[a][k] for k in range(3)]).count(0.0) < 2
                                   for (_, a, b, _) in m["members"])}, None


def main():
    ref = json.load(open(sys.argv[1]))["cases"]
    maxm = int(sys.argv[3]) if len(sys.argv) > 3 else 100
    out = {"python": sys.version.split()[0], "cases": {}, "skipped": {}}
    for cid, c in ref.items():
        if c["family"] == "RF-MECH" or not c.get("expected"):
            out["skipped"][cid] = "mechanism or no expected values"
            continue
        r, why = run_case(cid, c, maxm)
        if r is None:
            out["skipped"][cid] = why
        else:
            out["cases"][cid] = r
    breaches = {k: v for k, v in out["cases"].items() if v["worst_actual_ratio"] > 1}
    out["summary"] = {
        "emulated_cases": len(out["cases"]), "skipped": len(out["skipped"]),
        "emulated_breaches": sorted(breaches),
        "EB_misses_a_breach": sorted(k for k, v in breaches.items() if v["EB_ratio"] <= 1),
        "EF_misses_a_breach": sorted(k for k, v in breaches.items() if v["EF_ratio"] <= 1),
        "EF_x8_misses_a_breach": sorted(k for k, v in breaches.items() if 8 * v["EF_ratio"] <= 1),
        "EB_fires": sum(1 for v in out["cases"].values() if v["EB_ratio"] > 1),
        "EF_x8_fires": sum(1 for v in out["cases"].values() if 8 * v["EF_ratio"] > 1),
        "EB_false_positives": sorted(k for k, v in out["cases"].items() if v["EB_ratio"] > 1 and v["worst_actual_ratio"] <= 1),
        "EF_x8_false_positives": sorted(k for k, v in out["cases"].items() if 8 * v["EF_ratio"] > 1 and v["worst_actual_ratio"] <= 1),
        "non_permutation_cases": sum(1 for v in out["cases"].values() if v["non_permutation"]),
        "cond_trigger": {str(t): {"fires": sum(1 for v in out["cases"].values() if v["cond1_equilibrated"] > t),
                                  "misses_a_breach": sorted(k for k, v in breaches.items() if v["cond1_equilibrated"] <= t),
                                  "false_positives": sum(1 for v in out["cases"].values() if v["cond1_equilibrated"] > t and v["worst_actual_ratio"] <= 1)}
                         for t in (1e5, 3e5, 1e6, 4.5e6)},
    }
    json.dump(out, open(sys.argv[2], "w"), indent=1)
    print(json.dumps(out["summary"], indent=1))


if __name__ == "__main__":
    main()
