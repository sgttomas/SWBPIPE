#!/usr/bin/env python3
"""D1 R5-4: the D-5 formation check with realized DEC-070 curved bends (standard library only; no
product code built or run).

Usage (from T3/):
  nice -n 19 python3 DESIGN_NUMERICS/_run_records/curved_ef.py <v1_dir> <out.json>

<v1_dir> holds V1's `REVIEW/_run_records/d5_check/probe_d5_check.py.txt` copied to
`probe_d5_check.py` (cited, not copied here). From it: product_frame, local_K, T12, TtKT,
solve_product (product-faithful prepare, dense Cholesky or RCM profile LDL^T, estimate_rcond,
residual gate and refinement) and apply_inverse.

The product's binary64 curved element is ported from `P/core/solver/curved_bend/src/lib.rs`
(geometry, trig_gram, unit_load_actions, end_flexibility, invert_symmetric6 through FK
`solve_dense`, equilibrium_transfer, assemble_macro_stiffness) and FK `transform_global_stiffness`;
the arc centre is computed as `PP` `build_curved_bend_macro_elements` computes it (chord, y
reference, user radius, sagitta). Operation order follows the source as read.

The intended system is formed in Decimal at 60 digits (far below 2^-53 x cond) from the same
binary64 inputs (node coordinates, the product's binary64 centre, E, G, A, I, J, factors):
  frames  exact frame, local coefficients and T^T K T;
  curved  OBJECTIVE re-formation (R5-4 design): mean radius R, cos/sin of the included angle from
          the radial vectors (square roots only), the angle by a Decimal arctangent series, the
          exact end flexibility, its exact inverse, the equilibrium transfer H built from the ACTUAL
          chord x_j - x_i expressed in the exact local axes, and the exact transform.
Checks per model and mode:
  actual  |u - u_int| / (1e-9 max(|u_int|, S*_int)) on free nodal rows;
  EF      the exact-residual estimate with the curved element re-formed (rule 2|w| > 1e-9 max(|q|, S*));
  EF_shared  the same, but the curved element's K_int taken as the product's binary64 matrix
          (what a check that cannot re-form curved would see);
  demote_all  D5C-2 as written in revision 5 (every Passed case with a curved bend).
"""
import json
import math
import os
import sys
from decimal import Decimal as D, getcontext
from fractions import Fraction as Fr

getcontext().prec = 60
sys.path.insert(0, sys.argv[1])
import probe_d5_check as p  # noqa: E402  (V1's product-faithful emulation, cited)

EPS = 2.0 ** -52


# ---------------------------------------------------------------- binary64 port of the curved crate
def c_sub(a, b):
    return [a[0] - b[0], a[1] - b[1], a[2] - b[2]]


def c_dot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def c_cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def c_norm(v):
    return math.sqrt(c_dot(v, v))


def c_normalize(v):
    m = c_norm(v)
    return [v[0] / m, v[1] / m, v[2] / m]


def geometry64(xi, xj, c):
    ri, rj = c_sub(xi, c), c_sub(xj, c)
    R = 0.5 * (c_norm(ri) + c_norm(rj))
    n = c_cross(ri, rj)
    phi = math.atan2(c_norm(n), c_dot(ri, rj))
    x = c_normalize(ri)
    z = c_normalize(n)
    y = c_cross(z, x)
    return R, phi, [x, y, z]


def trig_gram64(phi):
    s, c, s2 = math.sin(phi), math.cos(phi), math.sin(2.0 * phi)
    return [[phi, s, 1.0 - c], [s, 0.5 * phi + 0.25 * s2, 0.5 * s * s], [1.0 - c, 0.5 * s * s, 0.5 * phi - 0.25 * s2]]


def unit_actions(R, phi, s=None, c=None, zero=0.0, one=1.0):
    """unit_load_actions; for Decimal pass s, c and Decimal zero/one."""
    if s is None:
        s, c = math.sin(phi), math.cos(phi)
    z = [zero, zero, zero]
    return [
        dict(ip=[-R * s, zero, R], op=z, t=z, a=[zero, zero, -one]),
        dict(ip=[R * c, -R, zero], op=z, t=z, a=[zero, one, zero]),
        dict(ip=z, op=[zero, R * s, -R * c], t=[R, -R * c, -R * s], a=z),
        dict(ip=z, op=[zero, one, zero], t=[zero, zero, -one], a=z),
        dict(ip=z, op=[zero, zero, one], t=[zero, one, zero], a=z),
        dict(ip=[one, zero, zero], op=z, t=z, a=z),
    ]


def quad(g, left, right, zero=0.0):
    s = zero
    for r in range(3):
        for cc in range(3):
            s = s + left[r] * g[r][cc] * right[cc]
    return s


def flexibility(R, g, cases, EI, GJ, EA, fin, fout, zero=0.0):
    F = [[zero] * 6 for _ in range(6)]
    for r in range(6):
        for cc in range(r, 6):
            v = R * (fin * quad(g, cases[r]["ip"], cases[cc]["ip"], zero) / EI
                     + fout * quad(g, cases[r]["op"], cases[cc]["op"], zero) / EI
                     + quad(g, cases[r]["t"], cases[cc]["t"], zero) / GJ
                     + quad(g, cases[r]["a"], cases[cc]["a"], zero) / EA)
            F[r][cc] = v
            F[cc][r] = v
    return F


def solve_dense64(A, b):
    n = len(A)
    M = [row[:] for row in A]
    r = b[:]
    for k in range(n):
        # FK solve_dense: the first strictly larger candidate wins
        piv, pv = k, abs(M[k][k])
        for i in range(k + 1, n):
            if abs(M[i][k]) > pv:
                piv, pv = i, abs(M[i][k])
        if piv != k:
            M[k], M[piv] = M[piv], M[k]
            r[k], r[piv] = r[piv], r[k]
        d = M[k][k]
        tail = M[k][k + 1:]
        for i in range(k + 1, n):
            f = M[i][k] / d
            M[i][k] = 0.0
            for j, t in zip(range(k + 1, n), tail):
                M[i][j] -= f * t
            r[i] -= f * r[k]
    x = [0.0] * n
    for i in reversed(range(n)):
        s = r[i]
        for j in range(i + 1, n):
            s -= M[i][j] * x[j]
        x[i] = s / M[i][i]
    return x


def invert6_64(F):
    X = [[0.0] * 6 for _ in range(6)]
    for cc in range(6):
        col = solve_dense64(F, [1.0 if i == cc else 0.0 for i in range(6)])
        for r in range(6):
            X[r][cc] = col[r]
    for r in range(6):
        for cc in range(r + 1, 6):
            a = 0.5 * (X[r][cc] + X[cc][r])
            X[r][cc] = a
            X[cc][r] = a
    return X


def transfer(chord, zero=0.0, one=1.0):
    H = [[zero] * 6 for _ in range(6)]
    for i in range(6):
        H[i][i] = one
    H[3][1] = -chord[2]
    H[3][2] = chord[1]
    H[4][0] = chord[2]
    H[4][2] = -chord[0]
    H[5][0] = -chord[1]
    H[5][1] = chord[0]
    return H


def mul6(A, B, zero=0.0, transpose_right=False):
    C = [[zero] * 6 for _ in range(6)]
    for r in range(6):
        for cc in range(6):
            s = zero
            for k in range(6):
                s = s + A[r][k] * (B[cc][k] if transpose_right else B[k][cc])
            C[r][cc] = s
    return C


def macro(X, chord, zero=0.0, one=1.0):
    H = transfer(chord, zero, one)
    cpl = mul6(H, X, zero)
    anc = mul6(cpl, H, zero, transpose_right=True)
    K = [[zero] * 12 for _ in range(12)]
    for r in range(6):
        for cc in range(6):
            K[r][cc] = anc[r][cc]
            K[r][cc + 6] = -cpl[r][cc]
            K[r + 6][cc] = -cpl[cc][r]
            K[r + 6][cc + 6] = X[r][cc]
    return K


def curved_rep(xi, xj, c, sec, fin, fout):
    R, phi, axes = geometry64(xi, xj, c)
    g = trig_gram64(phi)
    cases = unit_actions(R, phi)
    EI, GJ, EA = sec["E"] * sec["Iy"], sec["G"] * sec["J"], sec["E"] * sec["A"]
    F = flexibility(R, g, cases, EI, GJ, EA, fin, fout)
    X = invert6_64(F)
    chord = [R * (math.cos(phi) - 1.0), R * math.sin(phi), 0.0]
    K = macro(X, chord)
    return p.TtKT(K, p.T12(axes, 0.0), False), dict(R=R, phi=phi, cond_F=cond6(F))


def cond6(F):
    X = invert6_64(F)
    n1 = max(sum(abs(F[r][cc]) for r in range(6)) for cc in range(6))
    n2 = max(sum(abs(X[r][cc]) for r in range(6)) for cc in range(6))
    return n1 * n2


def pp_centre(xi, xj, yref, radius):
    ch = c_sub(xj, xi)
    L = math.sqrt(ch[0] * ch[0] + ch[1] * ch[1] + ch[2] * ch[2])
    h = 0.5 * L
    u = [ch[0] / L, ch[1] / L, ch[2] / L]
    ax = yref[0] * u[0] + yref[1] * u[1] + yref[2] * u[2]
    pn = [yref[0] - ax * u[0], yref[1] - ax * u[1], yref[2] - ax * u[2]]
    pm = math.sqrt(pn[0] * pn[0] + pn[1] * pn[1] + pn[2] * pn[2])
    sag = math.sqrt(radius * radius - h * h)
    return [0.5 * (xi[k] + xj[k]) - sag * pn[k] / pm for k in range(3)]


# ---------------------------------------------------------------- Decimal intended system
def dsqrt(x):
    return D(x).sqrt()


def datan(t):
    k = 0
    while t > D("0.05"):
        t = t / (1 + dsqrt(1 + t * t))
        k += 1
    s, term, n, t2 = D(0), t, 1, t * t
    while True:
        add = term / n
        if abs(add) < D(10) ** -70:
            break
        s += add
        term = -term * t2
        n += 2
    return s * (2 ** k)


def datan2(s, c):
    return 2 * datan(s / (1 + c))  # c > -1 for included angles below pi


def dcross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def ddot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def dinv(A):
    n = len(A)
    M = [row[:] + [D(1) if i == j else D(0) for j in range(n)] for i, row in enumerate(A)]
    for k in range(n):
        piv = max(range(k, n), key=lambda i: abs(M[i][k]))
        M[k], M[piv] = M[piv], M[k]
        d = M[k][k]
        M[k] = [v / d for v in M[k]]
        for i in range(n):
            if i != k and M[i][k] != 0:
                f = M[i][k]
                M[i] = [a - f * b for a, b in zip(M[i], M[k])]
    return [row[n:] for row in M]


def dsolve(A, b):
    n = len(A)
    M = [A[i][:] + [b[i]] for i in range(n)]
    for k in range(n):
        piv = max(range(k, n), key=lambda i: abs(M[i][k]))
        M[k], M[piv] = M[piv], M[k]
        d = M[k][k]
        for i in range(k + 1, n):
            if M[i][k] != 0:
                f = M[i][k] / d
                M[i] = [a - f * bb for a, bb in zip(M[i], M[k])]
    x = [D(0)] * n
    for i in reversed(range(n)):
        s = M[i][n] - sum(M[i][j] * x[j] for j in range(i + 1, n))
        x[i] = s / M[i][i]
    return x


def dTtKT(K, axes):
    Z = D(0)
    T = [[Z] * 12 for _ in range(12)]
    for b in range(4):
        for r in range(3):
            for cc in range(3):
                T[3 * b + r][3 * b + cc] = axes[r][cc]
    KT = [[sum(K[r][k] * T[k][cc] for k in range(12)) for cc in range(12)] for r in range(12)]
    return [[sum(T[k][r] * KT[k][cc] for k in range(12)) for cc in range(12)] for r in range(12)]


def frame_int(xi, xj, yref, sec):
    Rx, Lx = p.exact_frame(xi, xj, yref)
    axes = [[D(v.numerator) / D(v.denominator) for v in row] for row in Rx]
    L = D(Lx.numerator) / D(Lx.denominator)
    s = {k: D(Fr(sec[k]).numerator) / D(Fr(sec[k]).denominator) for k in sec}
    e, g, a, iy, j = s["E"], s["G"], s["A"], s["Iy"], s["J"]
    K = [[D(0)] * 12 for _ in range(12)]

    def put(r, cc, v):
        K[r][cc] = v
        K[cc][r] = v
    ax, tor = e * a / L, g * j / L
    put(0, 6, -ax); K[0][0] = ax; K[6][6] = ax
    put(3, 9, -tor); K[3][3] = tor; K[9][9] = tor
    z12, z6, z4, z2 = 12 * e * iy / L ** 3, 6 * e * iy / L ** 2, 4 * e * iy / L, 2 * e * iy / L
    for idx, t in (((1, 5, 7, 11), [[z12, z6, -z12, z6], [z6, z4, -z6, z2], [-z12, -z6, z12, -z6], [z6, z2, -z6, z4]]),
                   ((2, 4, 8, 10), [[z12, -z6, -z12, -z6], [-z6, z4, z6, z2], [-z12, z6, z12, z6], [-z6, z2, z6, z4]])):
        for r in range(4):
            for cc in range(4):
                K[idx[r]][idx[cc]] += t[r][cc]
    return dTtKT(K, axes)


def dec(x):
    f = Fr(x)
    return D(f.numerator) / D(f.denominator)


def curved_int(xi, xj, c, sec, fin, fout, objective=True):
    xi, xj, c = [dec(v) for v in xi], [dec(v) for v in xj], [dec(v) for v in c]
    ri = [xi[k] - c[k] for k in range(3)]
    rj = [xj[k] - c[k] for k in range(3)]
    ni, nj = dsqrt(ddot(ri, ri)), dsqrt(ddot(rj, rj))
    R = (ni + nj) / 2
    n = dcross(ri, rj)
    nn = dsqrt(ddot(n, n))
    cph = ddot(ri, rj) / (ni * nj)
    sph = nn / (ni * nj)
    phi = datan2(sph, cph)
    s2, c2 = 2 * sph * cph, cph * cph - sph * sph
    g = [[phi, sph, 1 - cph], [sph, phi / 2 + s2 / 4, sph * sph / 2], [1 - cph, sph * sph / 2, phi / 2 - s2 / 4]]
    cases = unit_actions(R, phi, sph, cph, D(0), D(1))
    E, G, A, I, J = (dec(sec[k]) for k in ("E", "G", "A", "Iy", "J"))
    F = flexibility(R, g, cases, E * I, G * J, E * A, dec(fin), dec(fout), D(0))
    X = dinv(F)
    x = [v / ni for v in ri]
    z = [v / nn for v in n]
    y = dcross(z, x)
    axes = [x, y, z]
    if objective:
        d = [xj[k] - xi[k] for k in range(3)]
        chord = [ddot(axes[0], d), ddot(axes[1], d), ddot(axes[2], d)]
    else:
        chord = [R * (cph - 1), R * sph, D(0)]
    K = macro(X, chord, D(0), D(1))
    return dTtKT(K, axes)


# ---------------------------------------------------------------- models
def build(model):
    names = list(model["nodes"])
    idx = {nm: i for i, nm in enumerate(names)}
    n = 6 * len(names)
    K = [[0.0] * n for _ in range(n)]
    Ki = [[D(0)] * n for _ in range(n)]
    Kcs = [[D(0)] * n for _ in range(n)]  # K_int with the curved element shared from binary64
    info = []
    sec = p.section(*model.get("section", (0.2, 0.18, 200e9, 80e9)))
    for el in model["members"]:
        a, b = el["i"], el["j"]
        xi, xj = model["nodes"][a], model["nodes"][b]
        base = [6 * idx[a] + k for k in range(6)] + [6 * idx[b] + k for k in range(6)]
        if el.get("bend"):
            radius = el["bend"]["R"]
            ctr = pp_centre(xi, xj, el["yref"], radius)
            Ke, meta = curved_rep(xi, xj, ctr, sec, el["bend"].get("k", 1.0), el["bend"].get("k", 1.0))
            Kx = curved_int(xi, xj, ctr, sec, el["bend"].get("k", 1.0), el["bend"].get("k", 1.0))
            Kxs = [[dec(v) for v in row] for row in Ke]
            meta.update(centre=ctr)
            info.append(meta)
        else:
            Rp, Lp = p.product_frame(xi, xj, el["yref"])
            Ke = p.TtKT(p.local_K(sec, Lp), p.T12(Rp, 0.0), False)
            Kx = frame_int(xi, xj, el["yref"], sec)
            Kxs = Kx
        for r in range(12):
            for cc in range(12):
                K[base[r]][base[cc]] += Ke[r][cc]
                Ki[base[r]][base[cc]] += Kx[r][cc]
                Kcs[base[r]][base[cc]] += Kxs[r][cc]
    fixed = set()
    for nm, dofs in model.get("rigid", {}).items():
        for d in dofs:
            fixed.add(6 * idx[nm] + "UX UY UZ RX RY RZ".split().index(d))
    for nm, d, k in model.get("springs", []):
        dof = 6 * idx[nm] + "UX UY UZ RX RY RZ".split().index(d)
        K[dof][dof] += k
        Ki[dof][dof] += dec(k)
        Kcs[dof][dof] += dec(k)
    f = [0.0] * n
    for nm, d, v in model.get("loads", []):
        f[6 * idx[nm] + "UX UY UZ RX RY RZ".split().index(d)] += v
    free = [i for i in range(n) if i not in fixed]
    pts = [model["nodes"][nm] for nm in names]
    ext = [max(q[k] for q in pts) - min(q[k] for q in pts) for k in range(3)]
    Lb = math.sqrt(sum(e * e for e in ext))
    return dict(n=n, free=free, K_rep=K, f=f, Ki=Ki, Kcs=Kcs, Lb=Lb, info=info)


def sstar(u, n, Lb):
    St = max(abs(u[d]) for d in range(n) if d % 6 < 3)
    Sr = max(abs(u[d]) for d in range(n) if d % 6 >= 3)
    return {"t": max(St, Lb * Sr), "r": max(Sr, St / Lb if Lb else 0.0)}


def analyse(model):
    m = build(model)
    n, free = m["n"], m["free"]
    uI = dsolve([[m["Ki"][i][j] for j in free] for i in free], [dec(m["f"][i]) for i in free])
    u_int = [0.0] * n
    for r, i in enumerate(free):
        u_int[i] = float(uI[r])
    Sint = sstar(u_int, n, m["Lb"])
    out = {"name": model["name"], "bends": [{"R": b["R"], "phi": b["phi"], "cond_F": b["cond_F"]} for b in m["info"]]}
    for mode in ("dense", "sparse"):
        sol = p.solve_product(m, mode)
        if sol is None or sol.get("outcome") == "unresolved":
            out[mode] = {"outcome": "factor failed" if sol is None else "unresolved"}
            continue
        u = sol["u"]
        S = sstar(u, n, m["Lb"])
        res = {}
        for label, KK in (("EF", m["Ki"]), ("EF_shared", m["Kcs"])):
            rho = []
            for i in free:
                s = dec(m["f"][i])
                for j in range(n):
                    if u[j] != 0.0:
                        s -= KK[i][j] * dec(u[j])
                rho.append(s)
            w = p.apply_inverse(sol, [float(x) for x in rho])
            res[label] = max(2 * abs(w[r]) / (1e-9 * max(abs(u[i]), S["t" if i % 6 < 3 else "r"])) for r, i in enumerate(free))
        act = max(abs(u[i] - float(uI[r])) / (1e-9 * max(abs(float(uI[r])), Sint["t" if i % 6 < 3 else "r"]))
                  for r, i in enumerate(free))
        out[mode] = {"outcome": sol["outcome"], "cond": 1 / sol["rcond"], "actual_ratio": act,
                     "EF_trigger_ratio": res["EF"], "EF_shared_trigger_ratio": res["EF_shared"],
                     "fires": res["EF"] > 1.0, "shared_fires": res["EF_shared"] > 1.0,
                     "demote_all": sol["outcome"] == "Passed" and bool(m["info"])}
    return out


def line(points, yref=(0.0, 0.0, 1.0)):
    return [dict(i=a, j=b, yref=yref) for a, b in zip(points, points[1:])]


def models():
    R = 0.3  # long-radius elbow, 1.5 x 0.2 m OD
    ms = []
    # E1: L in plan, anchors at both ends, elbow at the corner, hangers every 1.5 m
    nodes = {"A": [0.0, 0.0, 0.0]}
    xs = [1.5, 3.0, 4.5, 6.0]
    for k, x in enumerate(xs):
        nodes[f"X{k}"] = [x, 0.0, 0.0]
    nodes["T2"] = [6.0 + R, R, 0.0]
    ys = [R + 1.5, R + 3.0, R + 4.5, R + 6.0]
    for k, y in enumerate(ys):
        nodes[f"Y{k}"] = [6.0 + R, y, 0.0]
    mem = line(["A", "X0", "X1", "X2", "X3"]) + [dict(i="X3", j="T2", yref=(1.0, -1.0, 0.0), bend=dict(R=R))] + line(["T2", "Y0", "Y1", "Y2", "Y3"])
    loads = [(nm, "UZ", -1500.0) for nm in ("X0", "X1", "X2", "X3", "T2", "Y0", "Y1", "Y2")]
    ms.append(dict(name="E1 L-shape, two anchors, one elbow", nodes=nodes, members=mem,
                   rigid={"A": "UX UY UZ RX RY RZ".split(), "Y3": "UX UY UZ RX RY RZ".split()},
                   springs=[("X1", "UZ", 2e5), ("Y1", "UZ", 2e5)], loads=loads + [("T2", "UX", 800.0)]))
    # E2: E1 with a free end and a guide (Y3 free, guide on Y1 in UX)
    ms.append(dict(name="E2 L-shape, anchor, guide, free end", nodes=nodes, members=mem,
                   rigid={"A": "UX UY UZ RX RY RZ".split(), "Y1": ["UX"]},
                   springs=[("X1", "UZ", 2e5), ("Y1", "UZ", 2e5), ("Y3", "UZ", 2e5)], loads=loads + [("Y3", "UY", 500.0)]))
    # E3: Z-shape with a riser: elbow in plan, then an elbow turning up (xz plane)
    n3 = {"A": [0.0, 0.0, 0.0], "B": [3.0, 0.0, 0.0], "C": [3.0 + R, R, 0.0], "D": [3.0 + R, 3.0, 0.0],
          "E": [3.0 + R, 3.0 + R, R], "F": [3.0 + R, 3.0 + R, 4.0]}
    mem3 = [dict(i="A", j="B", yref=(0.0, 0.0, 1.0)), dict(i="B", j="C", yref=(1.0, -1.0, 0.0), bend=dict(R=R)),
            dict(i="C", j="D", yref=(0.0, 0.0, 1.0)), dict(i="D", j="E", yref=(0.0, 1.0, -1.0), bend=dict(R=R)),
            dict(i="E", j="F", yref=(1.0, 0.0, 0.0))]
    ms.append(dict(name="E3 plan elbow plus riser elbow, anchors at both ends", nodes=n3, members=mem3,
                   rigid={"A": "UX UY UZ RX RY RZ".split(), "F": "UX UY UZ RX RY RZ".split()},
                   springs=[("D", "UZ", 1e5)], loads=[("B", "UZ", -2000.0), ("D", "UZ", -2000.0), ("C", "UY", 300.0)]))
    # E4: skew line (2,1,0) with an elbow turning up, hangers
    u = [2.0 / math.sqrt(5.0), 1.0 / math.sqrt(5.0), 0.0]
    n4 = {"A": [0.0, 0.0, 0.0]}
    for k in range(1, 5):
        n4[f"S{k}"] = [1.5 * k * u[0], 1.5 * k * u[1], 0.0]
    s4 = n4["S4"]
    n4["T"] = [s4[0] + R * u[0], s4[1] + R * u[1], R]
    n4["U"] = [n4["T"][0], n4["T"][1], R + 3.0]
    mem4 = line(["A", "S1", "S2", "S3", "S4"]) + [dict(i="S4", j="T", yref=(u[0], u[1], -1.0), bend=dict(R=R)),
                                                    dict(i="T", j="U", yref=(1.0, 0.0, 0.0))]
    ms.append(dict(name="E4 skew line (2,1,0) with a riser elbow", nodes=n4, members=mem4,
                   rigid={"A": "UX UY UZ RX RY RZ".split(), "U": "UX UY UZ RX RY RZ".split()},
                   springs=[("S1", "UZ", 2e5), ("S3", "UZ", 2e5)], loads=[(f"S{k}", "UZ", -1500.0) for k in range(1, 5)]))
    # E5: E1 with flexibility factor 5 on the elbow and a soft rotational stabiliser at the free end
    mem5 = line(["A", "X0", "X1", "X2", "X3"]) + [dict(i="X3", j="T2", yref=(1.0, -1.0, 0.0), bend=dict(R=R, k=5.0))] + line(["T2", "Y0", "Y1", "Y2", "Y3"])
    ms.append(dict(name="E5 elbow k = 5, anchor plus hangers, free end with soft rotational stabiliser", nodes=nodes, members=mem5,
                   rigid={"A": "UX UY UZ RX RY RZ".split()},
                   springs=[("X1", "UZ", 2e5), ("Y1", "UZ", 2e5), ("Y3", "UZ", 2e5), ("Y3", "UX", 1e5), ("Y3", "UY", 1e5),
                            ("Y3", "RX", 1e3), ("Y3", "RY", 1e3), ("Y3", "RZ", 1e3)], loads=loads))
    # E6: expansion U-loop in plan: 9 m run, four elbows (arc centres on the inside of each turn),
    # 2.7 m legs, 3 m top, 9 m run, anchors at both ends, hangers, an axial pull at the loop exit
    pts = {"A": [0.0, 0.0, 0.0], "P1": [3.0, 0.0, 0.0], "P2": [6.0, 0.0, 0.0], "B1": [9.0, 0.0, 0.0],
           "B2": [9.0 + R, R, 0.0], "B3": [9.0 + R, 3.0, 0.0], "B4": [9.0 + 2 * R, 3.0 + R, 0.0],
           "M1": [10.5 + 2 * R, 3.0 + R, 0.0], "B5": [12.0 + 2 * R, 3.0 + R, 0.0], "B6": [12.0 + 3 * R, 3.0, 0.0],
           "B7": [12.0 + 3 * R, R, 0.0], "B8": [12.0 + 4 * R, 0.0, 0.0]}
    x0 = 12.0 + 4 * R
    for k in range(1, 4):
        pts[f"Q{k}"] = [x0 + 3.0 * k, 0.0, 0.0]
    mem6 = line(["A", "P1", "P2", "B1"]) + [dict(i="B1", j="B2", yref=(1.0, -1.0, 0.0), bend=dict(R=R)),
                                              dict(i="B2", j="B3", yref=(0.0, 0.0, 1.0)),
                                              dict(i="B3", j="B4", yref=(-1.0, 1.0, 0.0), bend=dict(R=R))]
    mem6 += line(["B4", "M1", "B5"]) + [dict(i="B5", j="B6", yref=(1.0, 1.0, 0.0), bend=dict(R=R)),
                                         dict(i="B6", j="B7", yref=(0.0, 0.0, 1.0)),
                                         dict(i="B7", j="B8", yref=(-1.0, -1.0, 0.0), bend=dict(R=R))]
    mem6 += line(["B8", "Q1", "Q2", "Q3"])
    ms.append(dict(name="E6 expansion U-loop, four elbows, anchors at both ends, hangers", nodes=pts, members=mem6,
                   rigid={"A": "UX UY UZ RX RY RZ".split(), "Q3": "UX UY UZ RX RY RZ".split()},
                   springs=[(nm, "UZ", 2e5) for nm in ("P1", "B3", "M1", "B7", "Q1")],
                   loads=[(nm, "UZ", -1500.0) for nm in ("P1", "P2", "B1", "B3", "M1", "B6", "Q1", "Q2")] + [("B8", "UX", 5000.0)]))
    # C-series: a single realized elbow whose only soft mode is the rigid rotation of the elbow about
    # its root (the 122 mechanism): root translations rigid, root rotational springs, tip moment.
    for kx in (1e4, 1e2, 40.0, 34.0, 1.0, 1e-2, 1e-4):
        nc = {"N0": [0.0, 0.0, 0.0], "N1": [R, R, 0.0]}
        mc = [dict(i="N0", j="N1", yref=(1.0, -1.0, 0.0), bend=dict(R=R))]
        ms.append(dict(name=f"C elbow cantilever, soft root spring k_Z = {kx:g}", nodes=nc, members=mc,
                       rigid={"N0": ["UX", "UY", "UZ"]}, springs=[("N0", "RX", 1e6), ("N0", "RY", 1e6), ("N0", "RZ", kx)],
                       loads=[("N1", "RZ", 0.01 * kx / 1e4)]))
    # C-skew: the same elbow in a skew plane (normal (1,2,2)/3), soft spring about global X.
    for kx in (1e2, 30.0, 15.0, 10.0, 9.0, 8.5, 1.0, 1e-2):
        a = [2.0 / 3.0, -2.0 / 3.0, 1.0 / 3.0]
        b = [1.0 / 3.0, 2.0 / 3.0, 2.0 / 3.0]
        b = [b[k] - (sum(b[i] * a[i] for i in range(3))) * a[k] for k in range(3)]
        nb = math.sqrt(sum(v * v for v in b))
        b = [v / nb for v in b]
        tip = [R * a[k] + R * b[k] for k in range(3)]
        mc = [dict(i="N0", j="N1", yref=tuple(a[k] - b[k] for k in range(3)), bend=dict(R=R))]
        ms.append(dict(name=f"C-skew elbow cantilever, soft root spring k_X = {kx:g}", nodes={"N0": [0.0, 0.0, 0.0], "N1": tip}, members=mc,
                       rigid={"N0": ["UX", "UY", "UZ"]}, springs=[("N0", "RX", kx), ("N0", "RY", 1e6), ("N0", "RZ", 1e6)],
                       loads=[("N1", "RX", 0.01 * kx / 1e4)]))
    # Small-angle bend (included angle about 1e-3 rad) inside a straight run between anchors
    Rb = 30.0
    ph = 1e-3
    n6 = {"A": [0.0, 0.0, 0.0], "B": [3.0, 0.0, 0.0], "C": [3.0 + Rb * math.sin(ph), Rb * (1 - math.cos(ph)), 0.0]}
    n6["D"] = [n6["C"][0] + 3.0 * math.cos(ph), n6["C"][1] + 3.0 * math.sin(ph), 0.0]
    mem6 = [dict(i="A", j="B", yref=(0.0, 0.0, 1.0)), dict(i="B", j="C", yref=(math.sin(ph / 2), -math.cos(ph / 2), 0.0), bend=dict(R=Rb)),
            dict(i="C", j="D", yref=(0.0, 0.0, 1.0))]
    ms.append(dict(name="S small-angle bend (1e-3 rad, R = 30 m) between anchors", nodes=n6, members=mem6,
                   rigid={"A": "UX UY UZ RX RY RZ".split(), "D": "UX UY UZ RX RY RZ".split()},
                   loads=[("B", "UZ", -1000.0), ("C", "UY", 200.0)]))
    return ms


def main():
    out = {"python": sys.version.split()[0], "decimal_digits": getcontext().prec, "models": []}
    for mdl in models():
        r = analyse(mdl)
        out["models"].append(r)
        print(json.dumps(r), flush=True)
    passed = [(r["name"], md) for r in out["models"] for md in ("dense", "sparse") if r.get(md, {}).get("outcome") == "Passed"]
    out["summary"] = {
        "passed_case_modes": len(passed),
        "demote_all": sum(1 for r in out["models"] for md in ("dense", "sparse") if r.get(md, {}).get("demote_all")),
        "EF_fires_on_passed": [(r["name"], md) for r in out["models"] for md in ("dense", "sparse")
                               if r.get(md, {}).get("outcome") == "Passed" and r[md]["fires"]],
        "passed_breaches": [(r["name"], md, r[md]["actual_ratio"]) for r in out["models"] for md in ("dense", "sparse")
                            if r.get(md, {}).get("outcome") == "Passed" and r[md]["actual_ratio"] > 1.0],
        "missed_by_EF": [(r["name"], md) for r in out["models"] for md in ("dense", "sparse")
                         if r.get(md, {}).get("outcome") == "Passed" and r[md]["actual_ratio"] > 1.0 and not r[md]["fires"]],
        "missed_by_EF_shared": [(r["name"], md) for r in out["models"] for md in ("dense", "sparse")
                                if r.get(md, {}).get("outcome") == "Passed" and r[md]["actual_ratio"] > 1.0 and not r[md]["shared_fires"]],
    }
    json.dump(out, open(sys.argv[2], "w"), indent=1)
    print(json.dumps(out["summary"], indent=1))


if __name__ == "__main__":
    main()
