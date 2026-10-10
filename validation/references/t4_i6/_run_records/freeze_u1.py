"""T4-I6 Part B: freeze the T4-U1 element references (u1_reference_cases.json).

usage: python -I freeze_u1.py OUT_JSON
Standard library only. Imports curved_ref.py from this script's directory.
"""
import sys, os, json, math, struct, platform, hashlib, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from decimal import Decimal as D, getcontext, localcontext
import curved_ref as C

PREC = 110
CHECK_PREC = 140
getcontext().prec = PREC

GRID_Q = 30  # node coordinates and translations on the 2^-30 m grid
GRID = 2.0 ** -GRID_Q
E, NU = 2.0e11, 0.25
G = E / (2.0 * (1.0 + NU))  # = 8e10 exactly in binary64 and in reals
assert G == 8.0e10
A, I, J = 0.005969026041820614, 2.700984283923829e-05, 5.401968567847658e-05
XS = [0.0, 5.0e5, 2.0e6, 5.0e6, 7.3e6]


def grid(v):
    return round(v / GRID) * GRID


def translation(X):
    # t_X = (X, 0.7 X, 0) with 0.7 X formed exactly (all integers here)
    return [X, float(int(X) * 7 // 10), 0.0]


def rot(axis, ang):
    n = math.sqrt(sum(a * a for a in axis))
    x, y, z = (a / n for a in axis)
    c, s = math.cos(ang), math.sin(ang)
    C1 = 1 - c
    return [[c + x * x * C1, x * y * C1 - z * s, x * z * C1 + y * s],
            [y * x * C1 + z * s, c + y * y * C1, y * z * C1 - x * s],
            [z * x * C1 - y * s, z * y * C1 + x * s, c + z * z * C1]]


QROT = rot([1.0, -2.0, 3.0], 0.7)  # the generic rotation for rotated copies (binary64 entries)
PERM = [[0, 0, 1], [1, 0, 0], [0, 1, 0]]  # (x,y,z) -> (z,x,y): exact proper rotation


def mv(M, v):
    return [M[i][0] * v[0] + M[i][1] * v[1] + M[i][2] * v[2] for i in range(3)]


ANGLES = [
    # label, nominal phi, mode: 'R' fixes R = 0.3; 'L' fixes |d| ~ L0 and derives R
    ("B90", math.pi / 2, "R", 0.3),
    ("B45", math.pi / 4, "R", 0.3),
    ("B5", math.pi / 36, "R", 0.3),
    ("B1E-4", 1.0e-4, "L", 0.3),
    ("B1E-8", 1.0e-8, "L", 0.3),
    ("BPI-1E-6", math.pi - 1.0e-6, "L", 0.6),
]
PLANES = {
    "IP": ([math.cos(math.pi / 6), math.sin(math.pi / 6), 0.0], [0.0, 1.0, 0.0]),
    "SK": ([1.0 / 3.0, 2.0 / 3.0, 2.0 / 3.0], [1.0, -1.0, 0.5]),
}
KS = [1.0, 2.5]


def radius_for(d, phi_nom):
    # binary64 R from the actual |d| (exact) and the nominal angle
    with localcontext() as c:
        c.prec = 60
        L = C.norm([C.dec(v) for v in d])
        half = C.dec(phi_nom) / 2
        R = L / (2 * C.sin(half))
    return float(R)


def make_geometry(label, phi, mode, size, plane):
    dhat, yref = PLANES[plane]
    if mode == "R":
        R = size
        L = 2 * R * math.sin(phi / 2)
        d = [grid(L * c) for c in dhat]
    else:
        d = [grid(size * c) for c in dhat]
        R = radius_for(d, phi)
    return d, R, yref


def rotated(d, yref, phi, mode, R):
    dr = [grid(v) for v in mv(QROT, d)]
    yr = mv(QROT, yref)
    Rr = R if mode == "R" else radius_for(dr, phi)
    return dr, yr, Rr


def mat_str(M, n=20):
    return [[C.sci(v, n) for v in row] for row in M]


def element(d, R, yref, k, prec=PREC, xi=(0.0, 0.0, 0.0)):
    with localcontext() as c:
        c.prec = prec
        xj = [xi[i] + d[i] for i in range(3)]
        for i in range(3):
            assert xj[i] - xi[i] == d[i], "translation not exact"
        return C.curved_element(list(xi), xj, R, yref, E, G, A, I, J, k, k)


def null_residual(K, xi, xj):
    """max over modes and rows of |K r| / sum_c |K_rc r_c| (row-relative)."""
    worst = D(0)
    for name, m in C.rigid_modes(xi, xj):
        for r in range(12):
            s = sum((K[r][c] * m[c] for c in range(12)), D(0))
            den = sum((abs(K[r][c] * m[c]) for c in range(12)), D(0))
            if den > 0:
                worst = max(worst, abs(s) / den)
    return worst


def straight_K(d, yref):
    return C.frame_element([0.0, 0.0, 0.0], d, yref, E, G, A, I, I, J)


# --------------------------------------------------------------- naive binary64 emulation
# A straightforward binary64 evaluation of the same definition (this record's own
# formulas, closed-form Gram, phi = 2 asin(L/2R)). It is NOT product code; it
# shows the accuracy a naive implementation reaches, for the requirements list.
def naive_b64(d, R, yref, k):
    L = math.sqrt(d[0] * d[0] + d[1] * d[1] + d[2] * d[2])
    s = L / (2 * R)
    phi = 2 * math.asin(s)
    sinp, cosp = math.sin(phi), math.cos(phi)
    dh = [v / L for v in d]
    yd = sum(yref[i] * dh[i] for i in range(3))
    nr = [yref[i] - yd * dh[i] for i in range(3)]
    nn = math.sqrt(sum(v * v for v in nr))
    n = [v / nn for v in nr]
    z = [n[1] * dh[2] - n[2] * dh[1], n[2] * dh[0] - n[0] * dh[2], n[0] * dh[1] - n[1] * dh[0]]
    x = [-math.sin(phi / 2) * dh[i] + math.cos(phi / 2) * n[i] for i in range(3)]
    y = [math.cos(phi / 2) * dh[i] + math.sin(phi / 2) * n[i] for i in range(3)]
    Gm = [[phi, sinp, 1 - cosp], [sinp, phi / 2 + math.sin(2 * phi) / 4, sinp * sinp / 2],
          [1 - cosp, sinp * sinp / 2, phi / 2 - math.sin(2 * phi) / 4]]
    z3 = (0.0, 0.0, 0.0)
    acts = [
        {"N": (0, 0, -1), "Mip": (-R * sinp, 0, R), "Mop": z3, "T": z3},
        {"N": (0, 1, 0), "Mip": (R * cosp, -R, 0), "Mop": z3, "T": z3},
        {"N": z3, "Mip": z3, "Mop": (0, R * sinp, -R * cosp), "T": (R, -R * cosp, -R * sinp)},
        {"N": z3, "Mip": z3, "Mop": (0, 1, 0), "T": (0, 0, -1)},
        {"N": z3, "Mip": z3, "Mop": (0, 0, 1), "T": (0, 1, 0)},
        {"N": z3, "Mip": (1, 0, 0), "Mop": z3, "T": z3},
    ]

    def q(u, v):
        return sum(u[i] * Gm[i][j] * v[j] for i in range(3) for j in range(3))
    F = [[R * (k * q(acts[a]["Mip"], acts[b]["Mip"]) / (E * I) + k * q(acts[a]["Mop"], acts[b]["Mop"]) / (E * I)
               + q(acts[a]["T"], acts[b]["T"]) / (G * J) + q(acts[a]["N"], acts[b]["N"]) / (E * A))
          for b in range(6)] for a in range(6)]
    # invert (Gauss-Jordan, binary64)
    n6 = 6
    M = [F[i][:] + [1.0 if i == j else 0.0 for j in range(n6)] for i in range(n6)]
    for col in range(n6):
        p = max(range(col, n6), key=lambda r: abs(M[r][col]))
        M[col], M[p] = M[p], M[col]
        pv = M[col][col]
        if pv == 0.0:
            return None
        M[col] = [v / pv for v in M[col]]
        for r in range(n6):
            if r != col:
                f = M[r][col]
                M[r] = [M[r][c] - f * M[col][c] for c in range(2 * n6)]
    Kt = [row[n6:] for row in M]
    axes = [x, y, z]
    c = [sum(axes[a][i] * d[i] for i in range(3)) for a in range(3)]
    S = [[0, -c[2], c[1]], [c[2], 0, -c[0]], [-c[1], c[0], 0]]
    H = [[0.0] * 6 for _ in range(6)]
    for i in range(3):
        H[i][i] = 1.0
        H[i + 3][i + 3] = 1.0
        for j in range(3):
            H[i + 3][j] = S[i][j]

    def mm(P, Q):
        return [[sum(P[i][t] * Q[t][j] for t in range(len(Q))) for j in range(len(Q[0]))] for i in range(len(P))]
    Ht = [list(r) for r in zip(*H)]
    HK = mm(H, Kt)
    HKHt = mm(HK, Ht)
    KHt = mm(Kt, Ht)
    Kl = [[0.0] * 12 for _ in range(12)]
    for i in range(6):
        for j in range(6):
            Kl[i][j] = HKHt[i][j]
            Kl[i][j + 6] = -HK[i][j]
            Kl[i + 6][j] = -KHt[i][j]
            Kl[i + 6][j + 6] = Kt[i][j]
    T = [[0.0] * 12 for _ in range(12)]
    for b in range(4):
        for i in range(3):
            for j in range(3):
                T[3 * b + i][3 * b + j] = axes[i][j]
    Tt = [list(r) for r in zip(*T)]
    return mm(Tt, mm(Kl, T)), phi


def compare(Kb, Kref):
    scale = C.max_abs(Kref)
    worst_ms = D(0)
    worst_diag = D(0)
    for i in range(12):
        for j in range(12):
            diff = abs(C.dec(Kb[i][j]) - Kref[i][j])
            worst_ms = max(worst_ms, diff / scale)
            worst_diag = max(worst_diag, diff / (Kref[i][i] * Kref[j][j]).sqrt())
    return worst_ms, worst_diag


def main(out):
    t0 = time.time()
    cases = []
    summary = []
    for label, phi_nom, mode, size in ANGLES:
        for plane in PLANES:
            d0, R0, y0 = make_geometry(label, phi_nom, mode, size, plane)
            dr, yr, Rr = rotated(d0, y0, phi_nom, mode, R0)
            for k in KS:
                for variant, (d, R, yref) in (("base", (d0, R0, y0)), ("rotated", (dr, Rr, yr))):
                    cid = f"{label}-{plane}-k{k}-{variant}"
                    el = element(d, R, yref, k)
                    K = el["K"]
                    geo = el["geo"]
                    # digits check at higher precision
                    el2 = element(d, R, yref, k, prec=CHECK_PREC)
                    dig = max(abs(el2["K"][i][j] - K[i][j]) for i in range(12) for j in range(12)) / C.max_abs(K)
                    # translation: recompute at X = 7.3e6 from the translated coordinates
                    tx = translation(7.3e6)
                    elX = element(d, R, yref, k, xi=tuple(tx))
                    trans_equal = all(elX["K"][i][j] == K[i][j] for i in range(12) for j in range(12))
                    sym = max(abs(K[i][j] - K[j][i]) for i in range(12) for j in range(12)) / C.max_abs(K)
                    nres = null_residual(K, [0.0, 0.0, 0.0], d)
                    # permutation covariance (exact)
                    dp = [d[2], d[0], d[1]]
                    yp = [yref[2], yref[0], yref[1]]
                    elP = element(dp, R, yp, k)
                    P12 = [[D(0)] * 12 for _ in range(12)]
                    for b in range(4):
                        for i in range(3):
                            for j in range(3):
                                P12[3 * b + i][3 * b + j] = D(PERM[i][j])
                    PKPt = C.matmul(P12, C.matmul(K, C.transpose(P12)))
                    perm_dev = max(abs(PKPt[i][j] - elP["K"][i][j]) for i in range(12) for j in range(12)) / C.max_abs(K)
                    # straight-limit comparison (exact EB straight frame on the chord)
                    Ks = straight_K(d, yref)
                    str_dev = max(abs(Ks[i][j] - K[i][j]) for i in range(12) for j in range(12)) / C.max_abs(K)
                    # naive binary64 emulation
                    nb = naive_b64(d, R, yref, k)
                    if nb is None:
                        naive = None
                    else:
                        ms, dg = compare(nb[0], K)
                        naive = {"max_abs_diff_over_matrix_scale": C.sci(ms, 3),
                                 "max_abs_diff_over_sqrt_KiiKjj": C.sci(dg, 3),
                                 "phi_b64_minus_phi": C.sci(C.dec(nb[1]) - geo["phi"], 3)}
                    fc = [geo["R"] * (geo["cosp"] - 1), geo["R"] * geo["sinp"], D(0)]
                    fc_dev = C.norm([fc[q] - el["c_act"][q] for q in range(3)]) / geo["L"]
                    nodes = {}
                    for X in XS:
                        t = translation(X)
                        xj = [t[i] + d[i] for i in range(3)]
                        assert all(abs(v) < 2.0 ** 23 for v in xj)
                        nodes[repr(X)] = {"x_i": t, "x_j": xj}
                    case = {
                        "id": cid, "angle_label": label, "plane": plane, "variant": variant,
                        "k_in": k, "k_out": k,
                        "inputs": {"d": d, "R": R, "y_reference": yref,
                                   "E": E, "nu": NU, "G": G, "A": A, "I": I, "J": J},
                        "derived": {"L": C.sci(geo["L"], 25), "phi": C.sci(geo["phi"], 25),
                                    "pi_minus_phi": C.sci(C.pi() - geo["phi"], 20),
                                    "sin_half_phi": C.sci(geo["s"], 25), "cos_half_phi": C.sci(geo["ch"], 25),
                                    "local_axes_rows_global": mat_str(geo["axes"], 22),
                                    "chord_local": [C.sci(v, 22) for v in el["c_act"]],
                                    "tip_flexibility_local": mat_str(el["F"], 20)},
                        "nodes_by_X": nodes,
                        "K_global": mat_str(K, 20),
                        "checks": {
                            "digits_vs_prec140_rel_matrix_scale": C.sci(dig, 3),
                            "K_bit_identical_at_X_7.3e6_from_translated_nodes": trans_equal,
                            "symmetry_rel": C.sci(sym, 3),
                            "rigid_null_residual_row_relative": C.sci(nres, 3),
                            "perm_rotation_covariance_rel": C.sci(perm_dev, 3),
                            "straight_frame_on_chord_rel_diff": C.sci(str_dev, 3),
                            "formula_chord_at_p_minus_actual_chord_over_L": C.sci(fc_dev, 3),
                            "naive_binary64_emulation": naive,
                        },
                    }
                    cases.append(case)
                    summary.append((cid, geo["phi"], dig, trans_equal, nres, perm_dev, str_dev, naive))
    # CB's toy quarter circle (k_in != k_out), the only unequal-factor case, at the origin
    # and translated on the grid to X = 7.3e6 (CB large-coordinate control)
    toy = dict(E=100.0, G=40.0, A=3.0, I=5.0, J=7.0, R=2.0, kin=2.5, kout=1.75)
    toy_cases = []
    for tag, xi in (("origin", [2.0, 0.0, 0.0]), ("X7.3e6", [7300002.0, 5110000.0, 0.0])):
        xj = [xi[0] - 2.0, xi[1] + 2.0, xi[2]]
        el = C.curved_element(xi, xj, toy["R"], [1.0, 1.0, 0.0], toy["E"], toy["G"], toy["A"], toy["I"], toy["J"], toy["kin"], toy["kout"])
        toy_cases.append((tag, xi, xj, el))
    toy_equal = all(toy_cases[0][3]["K"][i][j] == toy_cases[1][3]["K"][i][j] for i in range(12) for j in range(12))
    cases.append({
        "id": "CB_Q90_TOY-k2.5-1.75", "angle_label": "B90", "plane": "global xy", "variant": "CB toy section",
        "k_in": toy["kin"], "k_out": toy["kout"],
        "inputs": {"d": [-2.0, 2.0, 0.0], "R": toy["R"], "y_reference": [1.0, 1.0, 0.0], "E": toy["E"], "G": toy["G"],
                   "A": toy["A"], "I": toy["I"], "J": toy["J"], "nu": None},
        "derived": {"phi": C.sci(toy_cases[0][3]["geo"]["phi"], 25),
                    "local_axes_rows_global": mat_str(toy_cases[0][3]["geo"]["axes"], 22),
                    "chord_local": [C.sci(v, 22) for v in toy_cases[0][3]["c_act"]],
                    "tip_flexibility_local": mat_str(toy_cases[0][3]["F"], 20)},
        "nodes_by_X": {tag: {"x_i": xi, "x_j": xj} for tag, xi, xj, _ in toy_cases},
        "K_global": mat_str(toy_cases[0][3]["K"], 20),
        "checks": {"K_bit_identical_origin_vs_X7.3e6": toy_equal,
                   "rigid_null_residual_row_relative": C.sci(null_residual(toy_cases[0][3]["K"], toy_cases[0][1], toy_cases[0][2]), 3)},
    })
    print("CB toy quarter circle: K identical at origin and X=7.3e6:", toy_equal)
    # naive binary64 small-angle sweep (B1-type geometry, IP plane, |d| ~ 0.3, k = 1)
    sweep = []
    for phi_nom in (1e-1, 3e-2, 1e-2, 5e-3, 3e-3, 2e-3, 1e-3, 1e-4, 1e-6, 1e-8):
        d, R, yref = make_geometry("sweep", phi_nom, "L", 0.3, "IP")
        el = element(d, R, yref, 1.0)
        nb = naive_b64(d, R, yref, 1.0)
        ms, dg = compare(nb[0], el["K"])
        sweep.append({"phi_nominal": phi_nom, "max_abs_diff_over_matrix_scale": C.sci(ms, 3), "max_abs_diff_over_sqrt_KiiKjj": C.sci(dg, 3)})
        print("naive sweep phi %-8g ms %s diag %s" % (phi_nom, C.sci(ms, 3), C.sci(dg, 3)))
    for eps in (1e-3, 1e-4, 1e-5, 1e-6, 1e-7):
        d, R, yref = make_geometry("sweep", math.pi - eps, "L", 0.6, "IP")
        el = element(d, R, yref, 1.0)
        nb = naive_b64(d, R, yref, 1.0)
        ms, dg = compare(nb[0], el["K"])
        sweep.append({"pi_minus_phi_nominal": eps, "max_abs_diff_over_matrix_scale": C.sci(ms, 3), "max_abs_diff_over_sqrt_KiiKjj": C.sci(dg, 3)})
        print("naive sweep pi-phi %-8g ms %s diag %s" % (eps, C.sci(ms, 3), C.sci(dg, 3)))

    # generic rotation vs Q K Q^T for the base/rotated pairs (shows the grid-rounding effect only)
    pairs = []
    byid = {c["id"]: c for c in cases}
    Q12 = [[D(0)] * 12 for _ in range(12)]
    for b in range(4):
        for i in range(3):
            for j in range(3):
                Q12[3 * b + i][3 * b + j] = C.dec(QROT[i][j])
    for c in cases:
        if c["variant"] != "base":
            continue
        if c["id"].startswith("CB_"):
            continue
        rid = c["id"].replace("-base", "-rotated")
        Kb = [[D(v) for v in row] for row in c["K_global"]]
        Kr = [[D(v) for v in row] for row in byid[rid]["K_global"]]
        QKQ = C.matmul(Q12, C.matmul(Kb, C.transpose(Q12)))
        dev = max(abs(QKQ[i][j] - Kr[i][j]) for i in range(12) for j in range(12)) / C.max_abs(Kb)
        pairs.append({"base": c["id"], "rotated": rid, "max_abs(K_rot - Q K Q^T)/max|K|": C.sci(dev, 3)})

    doc = {
        "record": "T4-I6 frozen references for T4-U1 (objective curved element)",
        "status": "frozen by T4-I6; to be refuted by an independent TASK before T4-U1's code is read",
        "definition": "U1_REFERENCE.md section B1 (the element) and B2 (this set)",
        "units": "SI: m, N, Pa, rad",
        "dof_order": "[ux, uy, uz, rx, ry, rz] at node i then node j, global axes",
        "precision": {"decimal_digits": PREC, "check_digits": CHECK_PREC,
                      "values_significant_digits": 20},
        "grid": {"quantum_m": "2^-30", "rule": "every coordinate and translation is a multiple of 2^-30 m with |x| < 2^23 m, so x_j - x_i is identical in binary64 at every X"},
        "translations": {"t_X": "(X, 0.7 X, 0)", "X": XS},
        "generic_rotation_Q_rows": QROT,
        "generic_rotation_note": "rotated copies: d_rot = grid(Q d), y_rot = binary64(Q y); for 'L'-mode angles R is re-derived from |d_rot| and the nominal angle; each rotated copy is its own frozen reference",
        "exact_rotation_P": {"rows": PERM, "rule": "(x,y,z) -> (z,x,y) applied to d and y_reference gives K exactly equal to P K P^T (checked per case)"},
        "section_inputs_note": "A, I, J are the binary64 values used by T3's K-D5 models for OD 0.2 m, t 0.01 m; G = E/(2(1+nu)) = 8e10 exactly",
        "cases": cases,
        "generic_rotation_pairs": pairs,
        "naive_binary64_emulation_sweep": {"note": "this record's own formulas evaluated naively in binary64 (closed-form Gram, phi = 2 asin(L/2R)); NOT product code; shows where a naive evaluation misses 1e-9", "rows": sweep},
        "run": {"python": platform.python_version()},
    }
    with open(out, "w") as f:
        json.dump(doc, f, indent=1, sort_keys=False)
        f.write("\n")
    # stdout summary
    print("T4-I6 freeze_u1: %d cases, python %s, decimal prec %d (check %d)" % (len(cases), platform.python_version(), PREC, CHECK_PREC))
    print("%-28s %-12s %-9s %-6s %-9s %-9s %-9s %-26s" % ("case", "phi", "digits", "trans", "null", "perm", "straight", "naive b64 (ms / diag)"))
    for cid, phi, dig, te, nres, pdv, sdv, naive in summary:
        nv = "-" if naive is None else naive["max_abs_diff_over_matrix_scale"] + " / " + naive["max_abs_diff_over_sqrt_KiiKjj"]
        print("%-28s %-12s %-9s %-6s %-9s %-9s %-9s %-26s" % (cid, C.sci(phi, 6), C.sci(dig, 2), te, C.sci(nres, 2), C.sci(pdv, 2), C.sci(sdv, 2), nv))
    for p in pairs:
        print("rotation pair", p["base"], "->", p["rotated"], "dev", p["max_abs(K_rot - Q K Q^T)/max|K|"])


if __name__ == "__main__":
    main(sys.argv[1])
