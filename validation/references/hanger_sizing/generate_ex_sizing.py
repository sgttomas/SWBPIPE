#!/usr/bin/env python3
"""Independent reference generator: hanger sizing on the exact route (DEL-04-11).

Writes ex_sizing.json: closed-form sizing references for documents that carry
internal pressure, a per-case ledger of stored loads and per-support, per-case
states, plus one free riser on a document with no pressurised region.

Independence: written from execution/PKG-04/DEL-04-11/ScopeOfWork.md (CLM-001,
REQ-002 to REQ-005, REQ-010), DEL-05-01's definitions of the test condition and
held support, docs/theory/hanger_sizing.md and the task brief only. It reads,
searches and imports no product code and no other generator. Standard library
only. Every value is invented; no supplier catalogue or standards data is used.

Method: every expected value comes from a closed form written below. Each
closed form is checked exactly against a small exact-rational plane-frame
direct-stiffness solve (Euler-Bernoulli, axial EA included, shear neglected)
written here. Every solve's vertical and horizontal equilibrium is asserted.
Pressure independence is asserted by re-running every pressurised solve with
an axial pressure representation and finding every vertical quantity unchanged.
pi enters only through A and I; it is computed here by Machin's formula with
rigorous integer bounds, and every emitted binary64 and every discrete outcome
must be the same at the lower and upper bound. Every discrete outcome must also
be unchanged when each input is replaced by the exact value of its binary64.

Usage:
    python3 generate_ex_sizing.py          # write ex_sizing.json
    python3 generate_ex_sizing.py --check  # exit 1 if the frozen JSON differs
"""

import argparse
import json
import math
import sys
from fractions import Fraction
from pathlib import Path

SCHEMA = "swbpipe.hanger_sizing_reference.ex_sizing/1"
OUT = Path(__file__).resolve().parent / "ex_sizing.json"

# ---------------------------------------------------------------- pi


def pi_bounds(digits=120):
    """Lower bound, midpoint and upper bound of pi as exact rationals.

    Machin: pi = 16 atan(1/5) - 4 atan(1/239). Each series is summed in
    integers scaled by 10**digits; each term is floored at most twice, so each
    term errs by less than 2 units, and the alternating tail after the last
    nonzero term is below 1 unit.
    """
    scale = 10 ** digits

    def atan_inv(n):
        total, power, k = 0, scale // n, 0
        while power:
            term = power // (2 * k + 1)
            total += -term if k % 2 else term
            power //= n * n
            k += 1
        return total, k

    a5, k5 = atan_inv(5)
    a239, k239 = atan_inv(239)
    approx = 16 * a5 - 4 * a239
    err = 16 * (2 * k5 + 1) + 4 * (2 * k239 + 1)
    lo, mid, hi = (Fraction(approx - err, scale), Fraction(approx, scale),
                   Fraction(approx + err, scale))
    assert lo < mid < hi and float(mid) == math.pi
    return lo, mid, hi


PI_LO, PI_MID, PI_HI = pi_bounds()


class Ctx:
    """One evaluation of the package: a value of pi and an input mode.

    binary=False: each input is the exact rational of its decimal string.
    binary=True: each input is the exact value of the nearest binary64.
    """

    def __init__(self, pi, binary):
        self.pi, self.binary = pi, binary

    def q(self, s):
        x = Fraction(s)
        return Fraction(float(x)) if self.binary else x


# ---------------------------------------------------------------- frame solve

DOFS = ("ux", "uz", "ry")


def gauss(a, b):
    n = len(b)
    m = [row[:] + [b[i]] for i, row in enumerate(a)]
    for c in range(n):
        p = next(r for r in range(c, n) if m[r][c] != 0)
        m[c], m[p] = m[p], m[c]
        for r in range(c + 1, n):
            if m[r][c] != 0:
                f = m[r][c] / m[c][c]
                m[r] = [x - f * y for x, y in zip(m[r], m[c])]
    x = [Fraction(0)] * n
    for r in range(n - 1, -1, -1):
        x[r] = (m[r][n] - sum(m[r][j] * x[j] for j in range(r + 1, n))) / m[r][r]
    return x


def frame_solve(nodes, members, rigid, springs, forces, strains):
    """Exact plane-frame solve in the X-Z plane (Z up).

    nodes: {id: (x, z)}; members: [{id, n1, n2, EA, EI}] lying along X or Z;
    rigid: [(sid, node, dof, prescribed)]; springs: [(sid, node, dof, k,
    base, preload)], force on the pipe preload - k (u - base);
    forces: [(node, dof, value)]; strains: {member: free axial strain}.
    Returns displacements {(node, dof)}, rigid reactions {(sid, dof)} and
    spring forces {sid}, each a force on the pipe along the +axis.
    """
    order = list(nodes)
    ix = {(n, d): 3 * i + j for i, n in enumerate(order) for j, d in enumerate(DOFS)}
    nd = 3 * len(order)
    K = [[Fraction(0)] * nd for _ in range(nd)]
    F = [Fraction(0)] * nd
    for m in members:
        (x1, z1), (x2, z2) = nodes[m["n1"]], nodes[m["n2"]]
        dx, dz = x2 - x1, z2 - z1
        assert (dx == 0) != (dz == 0)
        L = abs(dx) + abs(dz)
        c, s = dx / L, dz / L
        EA, EI = m["EA"], m["EI"]
        a, b, g, h4, h2 = EA / L, 12 * EI / L ** 3, 6 * EI / L ** 2, 4 * EI / L, 2 * EI / L
        kl = [[a, 0, 0, -a, 0, 0], [0, b, g, 0, -b, g], [0, g, h4, 0, -g, h2],
              [-a, 0, 0, a, 0, 0], [0, -b, -g, 0, b, -g], [0, g, h2, 0, -g, h4]]
        r3 = [[c, s, 0], [-s, c, 0], [0, 0, 1]]
        T = [[0] * 6 for _ in range(6)]
        for i in range(3):
            for j in range(3):
                T[i][j] = T[i + 3][j + 3] = r3[i][j]
        kg = [[sum(T[k][i] * kl[k][l] * T[l][j] for k in range(6) for l in range(6))
               for j in range(6)] for i in range(6)]
        dofs = [ix[(m["n1"], d)] for d in DOFS] + [ix[(m["n2"], d)] for d in DOFS]
        for i in range(6):
            for j in range(6):
                K[dofs[i]][dofs[j]] += kg[i][j]
        eps = strains.get(m["id"], 0)
        if eps:
            fl = [-EA * eps, 0, 0, EA * eps, 0, 0]
            for i in range(6):
                F[dofs[i]] += sum(T[k][i] * fl[k] for k in range(6))
    for node, dof, v in forces:
        F[ix[(node, dof)]] += v
    for _, node, dof, k, base, pre in springs:
        i = ix[(node, dof)]
        K[i][i] += k
        F[i] += k * base + pre
    pres = {}
    for _, node, dof, v in rigid:
        i = ix[(node, dof)]
        assert i not in pres
        pres[i] = Fraction(v)
    free = [i for i in range(nd) if i not in pres]
    uf = gauss([[K[i][j] for j in free] for i in free],
               [F[i] - sum(K[i][j] * v for j, v in pres.items()) for i in free])
    u = [Fraction(0)] * nd
    for i, v in zip(free, uf):
        u[i] = v
    for i, v in pres.items():
        u[i] = v
    react = {}
    for sid, node, dof, _ in rigid:
        i = ix[(node, dof)]
        react[(sid, dof)] = sum(K[i][j] * u[j] for j in range(nd)) - F[i]
    sprf = {}
    for sid, node, dof, k, base, pre in springs:
        sprf[sid] = pre - k * (u[ix[(node, dof)]] - base)
    # Equilibrium along X and Z: applied forces, reactions and spring forces.
    for d in ("ux", "uz"):
        tot = sum(v for _, dd, v in forces if dd == d)
        tot += sum(v for (_, dd), v in react.items() if dd == d)
        tot += sum(sprf[s[0]] for s in springs if s[2] == d)
        assert tot == 0, (d, tot)
    disp = {(n, d): u[ix[(n, d)]] for n in order for d in DOFS}
    return disp, react, sprf


# ---------------------------------------------------------------- inputs

ALL6 = ["UX", "UY", "UZ", "RX", "RY", "RZ"]

SECTION = {"D": "0.25", "t": "0.01", "E": "200000000000", "nu": "0.3",
           "alpha": "0.000012"}

A_DATA = {
    "x": {"N1": "0", "N2": "2", "N3": "4", "N4": "6", "N5": "8", "N6": "10",
          "N7": "12"},
    "loads": [("F-1", "N2", "-8000"), ("F-2", "N4", "-6000"),
              ("F-3", "N6", "-5000"), ("F-4", "N4", "-10000"),
              ("F-5", "N4", "2000")],
    "pressure": ("PR-1", "3500000"),
    "t_install": "20",
    "cases": [
        # id, role, temperature (degC), ledger [(load, factor)], imposed [(sid, g)]
        ("W", "weight", "20",
         [("F-1", "1"), ("F-2", "1.5"), ("F-3", "1"), ("PR-1", "1")], []),
        ("O1", "operating", "260",
         [("F-1", "1"), ("F-2", "1.5"), ("F-3", "1"), ("F-5", "0.25"),
          ("PR-1", "1")],
         [("S-1", "-0.012"), ("S-7", "0.012")]),
        ("O2", "operating", "180",
         [("F-1", "1"), ("F-2", "1.5"), ("F-3", "1"), ("PR-1", "1")],
         [("S-1", "0.009"), ("S-7", "-0.009")]),
    ],
    "candidates": [("H-3", "N3"), ("H-5", "N5")],
}

B_DATA = {
    "z": {"N1": "0", "N2": "4", "N3": "8"},
    "loads": [("F-T", "N3", "-9000"), ("F-M", "N2", "-4000")],
    "t_install": "15",
    "cases": [
        ("W", "weight", "15", [("F-T", "1"), ("F-M", "1")]),
        ("O1", "operating", "315", [("F-T", "1"), ("F-M", "1")]),
        ("O2", "operating", "-35", [("F-T", "1"), ("F-M", "1")]),
    ],
}

# Invented catalogue. (id, type, k, dof, range, travel); None = absent.
CATALOGUE_A = [
    ("VS-NR", "variable_spring_hanger", "100000", "UZ", None, "0.05"),
    ("VS-UX", "variable_spring_hanger", "100000", "UX", ("9000", "14000"), "0.05"),
    ("VS-A", "variable_spring_hanger", "100000", "UZ", ("4000", "8000"), "0.05"),
    ("VS-B", "variable_spring_hanger", "100000", "UZ", ("9000", "10500"), "0.05"),
    ("VS-C", "variable_spring_hanger", "100000", "UZ", ("9700", "13000"), "0.05"),
    ("VS-T", "variable_spring_hanger", "100000", "UZ", ("9000", "12000"), "0.007"),
    ("VS-V", "variable_spring_hanger", "800000", "UZ", ("4000", "18000"), "0.05"),
    ("CE-1", "constant_effort_support", None, None, ("5000", "9000"), "0.011"),
    ("CE-2", "constant_effort_support", None, None, ("8000", "12000"), "0.05"),
    ("CE-3", "constant_effort_support", None, None, ("6000", "8000"), "0.01"),
    ("CE-4", "constant_effort_support", None, None, ("7000", "9500"), "0.05"),
]

CATALOGUE_B = [
    ("VS-R1", "variable_spring_hanger", "50000", "UZ", ("9000", "13000"), "0.05"),
    ("VS-R2", "variable_spring_hanger", "80000", "UZ", ("8000", "12000"), "0.05"),
    ("VS-R3", "variable_spring_hanger", "150000", "UZ", ("6000", "16000"), "0.05"),
    ("CE-R1", "constant_effort_support", None, None, ("10000", "12000"), "0.05"),
]

RUNS_A = [
    ("A-HOT", "hot", "operating_load", "0.25", "0.001", "smallest_capacity",
     {"H-3": "variable", "H-5": "constant"}),
    ("A-COLD", "cold", "operating_load", "0.25", "0.001", "smallest_capacity",
     {"H-3": "variable", "H-5": "constant"}),
    ("A-NOFIT", "hot", "operating_load", "0.25", "0.001", "smallest_capacity",
     {"H-3": "variable", "H-5": "variable"}),
]

RUNS_B = [
    ("B-COLD", "cold", "design_load", "0.3", "0.002", "least_variation",
     {"H-3": "variable"}),
]


def section(ctx):
    D, t, E = ctx.q(SECTION["D"]), ctx.q(SECTION["t"]), ctx.q(SECTION["E"])
    d = D - 2 * t
    A = ctx.pi * (D ** 2 - d ** 2) / 4
    I = ctx.pi * (D ** 4 - d ** 4) / 64
    Ai = ctx.pi * d ** 2 / 4
    return {"D": D, "t": t, "d": d, "E": E, "EA": E * A, "EI": E * I, "Ai": Ai,
            "nu": ctx.q(SECTION["nu"]), "alpha": ctx.q(SECTION["alpha"])}


# ---------------------------------------------------------------- model A


def model_a(ctx):
    sec = section(ctx)
    x = {n: ctx.q(v) for n, v in A_DATA["x"].items()}
    nodes = {n: (v, Fraction(0)) for n, v in x.items()}
    names = list(nodes)
    members = [{"id": "M%d" % (i + 1), "n1": names[i], "n2": names[i + 1],
                "EA": sec["EA"], "EI": sec["EI"]} for i in range(len(names) - 1)]
    Ltot = x["N7"] - x["N1"]
    loads = {lid: (node, ctx.q(v)) for lid, node, v in A_DATA["loads"]}
    alpha = sec["alpha"]
    tin = ctx.q(A_DATA["t_install"])
    cases = {}
    for cid, role, temp, ledger, imposed in A_DATA["cases"]:
        f = []
        for lid, fac in ledger:
            if lid in loads:
                node, v = loads[lid]
                f.append((node, "uz", ctx.q(fac) * v))
        cases[cid] = {"role": role, "dT": ctx.q(temp) - tin, "forces": f,
                      "imposed": {s: ctx.q(g) for s, g in imposed}}
    cands = A_DATA["candidates"]
    cnode = dict(cands)
    # Non-candidate supports: S-1 (UZ) and G-1 (UX, and out of plane) at N1,
    # S-7 (UZ) and G-7 (out of plane) at N7. In the plane only UX at N1 and
    # UZ at N1 and N7 act.
    def base_rigid(case):
        imp = cases[case]["imposed"]
        return [("S-1", "N1", "uz", imp.get("S-1", 0)), ("G-1", "N1", "ux", 0),
                ("S-7", "N7", "uz", imp.get("S-7", 0))]

    p = ctx.q(A_DATA["pressure"][1])

    def run(case, rigid_cands=(), force_cands=None, springs=(), pressure=False):
        c = cases[case]
        rigid = base_rigid(case) + [(h, cnode[h], "uz", 0) for h in rigid_cands]
        forces = list(c["forces"])
        for h, v in (force_cands or {}).items():
            forces.append((cnode[h], "uz", v))
        strains = {m["id"]: alpha * c["dT"] for m in members}
        if pressure:
            # An axial representation of internal pressure on the closed run:
            # end-cap thrust p*A_i outward at both ends and the Poisson part of
            # the hoop strain, -nu * p * d / (2 t E).
            thrust = p * sec["Ai"]
            forces += [("N1", "ux", -thrust), ("N7", "ux", thrust)]
            eps_p = -sec["nu"] * p * sec["d"] / (2 * sec["t"] * sec["E"])
            strains = {k: v + eps_p for k, v in strains.items()}
        sp = [(h, cnode[h], "uz", k, 0, pre) for h, k, pre in springs]
        res = frame_solve(nodes, members, rigid, sp, forces, strains)
        if not pressure:
            # Pressure independence: every vertical quantity is unchanged.
            res_p = run(case, rigid_cands, force_cands, springs, pressure=True)
            for key, v in res[0].items():
                if key[1] in ("uz", "ry"):
                    assert res_p[0][key] == v
            for key, v in res[1].items():
                if key[1] == "uz":
                    assert res_p[1][key] == v
            assert res_p[1][("G-1", "ux")] == res[1][("G-1", "ux")] == 0
            assert res_p[2] == res[2]
        return res

    # --- closed forms
    xs = {h: x[n] for h, n in cands}

    def loads_of(case):
        return [(x[n], v) for n, _, v in cases[case]["forces"]]

    def flex(xa, xb):
        """Simply supported span Ltot: deflection at xa per unit force at xb."""
        if xa > xb:
            xa, xb = xb, xa
        b = Ltot - xb
        return b * xa * (Ltot ** 2 - b ** 2 - xa ** 2) / (6 * sec["EI"] * Ltot)

    def rigid_body(case, xp):
        imp = cases[case]["imposed"]
        return (imp.get("S-1", 0) * (Ltot - xp) + imp.get("S-7", 0) * xp) / Ltot

    # S_W: three equal spans, loads at midspans (three-moment equation).
    Ls = x["N3"] - x["N1"]
    assert x["N5"] - x["N3"] == Ls and x["N7"] - x["N5"] == Ls
    span_of = {"N2": 0, "N4": 1, "N6": 2}
    P = [Fraction(0)] * 3
    for n, _, v in cases["W"]["forces"]:
        assert x[n] - x["N1"] - span_of[n] * Ls == Ls / 2
        P[span_of[n]] += -v
    r1 = -Fraction(3) * Ls / 8 * (P[0] + P[1])
    r2 = -Fraction(3) * Ls / 8 * (P[1] + P[2])
    M2 = (4 * r1 - r2) / 15
    M3 = (4 * r2 - r1) / 15
    Ms = [0, M2, M3, 0]
    Vl = [P[i] / 2 + (Ms[i + 1] - Ms[i]) / Ls for i in range(3)]
    Vr = [P[i] / 2 - (Ms[i + 1] - Ms[i]) / Ls for i in range(3)]
    closed_rigid = {"S-1": Vl[0], "H-3": Vr[0] + Vl[1], "H-5": Vr[1] + Vl[2],
                    "S-7": Vr[2]}
    W = {"H-3": closed_rigid["H-3"], "H-5": closed_rigid["H-5"]}

    disp, react, _ = run("W", rigid_cands=("H-3", "H-5"))
    for sid, v in closed_rigid.items():
        assert react[(sid, "uz")] == v, (sid, v)

    def ss_reactions(case, extra):
        """Statics of the simply supported span: R(S-1), R(S-7) on the pipe."""
        fs = loads_of(case) + extra
        return (-sum(v * (Ltot - xp) for xp, v in fs) / Ltot,
                -sum(v * xp for xp, v in fs) / Ltot)

    def ss_u(case, xp, extra):
        """Simply supported span: settlement plus every force of the case."""
        return rigid_body(case, xp) + sum(
            v * flex(xp, xf) for xf, v in loads_of(case) + extra)

    wf = [(xs[h], W[h]) for h in W]
    derived = {}
    for case in ("W", "O1", "O2"):
        disp, react, _ = run(case, force_cands=W)
        u = {h: ss_u(case, xs[h], wf) for h in W}
        for h in W:
            assert disp[(cnode[h], "uz")] == u[h]
        r = ss_reactions(case, [(xs[h], W[h]) for h in W])
        assert react[("S-1", "uz")] == r[0] and react[("S-7", "uz")] == r[1]
        if loads_of(case) == loads_of("W"):
            assert r == (closed_rigid["S-1"], closed_rigid["S-7"])
        # Failure signature: no force in place of the candidates.
        disp0, _, _ = run(case)
        fsig = {}
        for h in W:
            v = ss_u(case, xs[h], [])
            assert disp0[(cnode[h], "uz")] == v
            fsig[h] = v
        derived[case] = {"u": u, "reactions": {"S-1": r[0], "S-7": r[1]},
                         "failure_u": fsig}

    def verify(case, config):
        """Verification solve. config: {h: ("spring", k, F_cold) | ("force", W)}."""
        springs = [(h, c[1], c[2]) for h, c in config.items() if c[0] == "spring"]
        fc = {h: c[1] for h, c in config.items() if c[0] == "force"}
        disp, react, sprf = run(case, force_cands=fc, springs=springs)
        # Closed form: simply supported span. With every candidate a force W
        # the positions are the derived positions; a spring adds its excess
        # F_cold - W - k u at its node.
        sh = [h for h, c in config.items() if c[0] == "spring"]
        assert len(sh) <= 1
        u = {h: ss_u(case, xs[h], wf) for h in config}
        if sh:
            h = sh[0]
            k, Fc = config[h][1], config[h][2]
            ex = Fc - W[h]
            uh = (u[h] + flex(xs[h], xs[h]) * ex) / (1 + k * flex(xs[h], xs[h]))
            net = ex - k * uh
            for g in config:
                u[g] = uh if g == h else u[g] + flex(xs[g], xs[h]) * net
            assert sprf[h] == Fc - k * uh
        for g in config:
            assert disp[(cnode[g], "uz")] == u[g], (case, g)
        return u

    return {"sec": sec, "x": x, "cases": cases, "W": W, "rigid": closed_rigid,
            "derived": derived, "verify": verify, "Ltot": Ltot, "loads": loads,
            "p": p}


# ---------------------------------------------------------------- model B


def model_b(ctx):
    sec = section(ctx)
    z = {n: ctx.q(v) for n, v in B_DATA["z"].items()}
    nodes = {n: (Fraction(0), v) for n, v in z.items()}
    members = [{"id": "M1", "n1": "N1", "n2": "N2", "EA": sec["EA"], "EI": sec["EI"]},
               {"id": "M2", "n1": "N2", "n2": "N3", "EA": sec["EA"], "EI": sec["EI"]}]
    H = z["N3"] - z["N1"]
    assert z["N2"] - z["N1"] == H / 2
    loads = {lid: (node, ctx.q(v)) for lid, node, v in B_DATA["loads"]}
    tin = ctx.q(B_DATA["t_install"])
    cases = {}
    for cid, role, temp, ledger in B_DATA["cases"]:
        f = [(loads[l][0], "uz", ctx.q(fac) * loads[l][1]) for l, fac in ledger]
        cases[cid] = {"role": role, "dT": ctx.q(temp) - tin, "forces": f}
    alpha, EA = sec["alpha"], sec["EA"]
    anchor = [("A-1", "N1", "ux", 0), ("A-1", "N1", "uz", 0), ("A-1", "N1", "ry", 0)]

    def run(case, rigid=False, force=None, spring=None):
        c = cases[case]
        r = anchor + ([("H-3", "N3", "uz", 0)] if rigid else [])
        f = list(c["forces"]) + ([("N3", "uz", force)] if force is not None else [])
        sp = [("H-3", "N3", "uz", spring[0], 0, spring[1])] if spring else []
        return frame_solve(nodes, members, r, sp, f, {m["id"]: alpha * c["dT"]
                                                      for m in members})

    Pt = -dict((n, v) for n, _, v in cases["W"]["forces"])["N3"]
    Pm = -dict((n, v) for n, _, v in cases["W"]["forces"])["N2"]
    # S_W: the riser is a fixed-fixed axial bar; the top load passes into the
    # rigid candidate and the mid-height load splits equally (uniform EA, load
    # at mid-height): W = Pt + Pm/2, anchor UZ = Pm/2.
    W = Pt + Pm / 2
    _, react, _ = run("W", rigid=True)
    assert react[("H-3", "uz")] == W and react[("A-1", "uz")] == Pm / 2
    derived = {}
    for case in ("W", "O1", "O2"):
        disp, react, _ = run(case, force=W)
        # Upper half carries W - Pt = Pm/2 in tension, lower half Pm/2 in
        # compression: no net shortening, so u = alpha dT H.
        u = alpha * cases[case]["dT"] * H
        assert disp[("N3", "uz")] == u and react[("A-1", "uz")] == Pm / 2
        disp0, _, _ = run(case)
        fsig = u - (2 * Pt + Pm) * H / (2 * EA)
        assert disp0[("N3", "uz")] == fsig
        derived[case] = {"u": {"H-3": u}, "reactions": {"A-1": Pm / 2},
                         "failure_u": {"H-3": fsig}}

    def verify(case, config):
        c = config["H-3"]
        if c[0] == "spring":
            k, Fc = c[1], c[2]
            u = (alpha * cases[case]["dT"] * H + (Fc - W) * H / EA) / (1 + k * H / EA)
            disp, _, sprf = run(case, spring=(k, Fc))
            assert sprf["H-3"] == Fc - k * u
        else:
            u = alpha * cases[case]["dT"] * H
            disp, _, _ = run(case, force=c[1])
        assert disp[("N3", "uz")] == u
        return {"H-3": u}

    return {"sec": sec, "z": z, "cases": cases, "W": {"H-3": W},
            "rigid": {"A-1": Pm / 2, "H-3": W}, "derived": derived,
            "verify": verify, "H": H, "loads": loads}


# ---------------------------------------------------------------- sizing (REQ-004)

VAR_TESTS = ["E1", "E2", "V0", "L", "T", "V"]
CON_TESTS = ["E1", "L", "T"]


def records(ctx, cat):
    out = []
    for rid, typ, k, dof, rng, trav in cat:
        out.append({"id": rid, "type": typ, "k": ctx.q(k) if k else None, "dof": dof,
                    "range": (ctx.q(rng[0]), ctx.q(rng[1])) if rng else None,
                    "travel": ctx.q(trav) if trav else None})
    return out


def span(uC, uops):
    pos = [Fraction(0), uC] + list(uops.values())
    return min(pos), max(pos)


def eval_variable(rec, W, uC, uops, design, m):
    """Returns (first failed test or None, quantities)."""
    if rec["k"] is None or rec["range"] is None or rec["travel"] is None:
        return "E1", None
    if rec["dof"] != "UZ":
        return "E2", None
    k = rec["k"]
    Fc = W + k * (uops[design] if m["design"] == "hot" else uC)
    FC = Fc - k * uC
    Fi = {c: Fc - k * u for c, u in uops.items()}
    lo, hi = span(uC, uops)
    a = m["a"]
    env = (Fc - k * (hi + a), Fc - k * (lo - a))
    req = hi - lo + 2 * a
    qn = {"F_cold": Fc, "F_C": FC, "F_i": Fi, "env": env, "req": req}
    if not (FC > 0 and all(v > 0 for v in Fi.values())):
        return "V0", qn
    var = {c: abs(FC - Fi[c]) / (Fi[c] if m["ref"] == "operating_load" else W)
           for c in uops}
    qn["var"] = var
    if not (rec["range"][0] <= env[0] and env[1] <= rec["range"][1]):
        return "L", qn
    if not req <= rec["travel"]:
        return "T", qn
    if not all(v <= m["limit"] for v in var.values()):
        return "V", qn
    return None, qn


def eval_constant(rec, W, uC, uops, m):
    if rec["range"] is None or rec["travel"] is None:
        return "E1", None
    lo, hi = span(uC, uops)
    req = hi - lo + 2 * m["a"]
    if not (rec["range"][0] <= W <= rec["range"][1]):
        return "L", None
    if not req <= rec["travel"]:
        return "T", None
    return None, {"req": req}


def select(policy, recs, W, uC, uops, design, m):
    families = []
    outcome = None

    def run_family(fam):
        tests = VAR_TESTS if fam == "variable" else CON_TESTS
        typ = "variable_spring_hanger" if fam == "variable" else "constant_effort_support"
        counts = {t: 0 for t in tests}
        fits, per = [], {}
        for r in recs:
            if r["type"] != typ:
                continue
            if fam == "variable":
                fail, qn = eval_variable(r, W, uC, uops, design, m)
            else:
                fail, qn = eval_constant(r, W, uC, uops, m)
            per[r["id"]] = fail or "fits"
            if fail:
                counts[fail] += 1
            else:
                gv = max(qn["var"].values()) if fam == "variable" else Fraction(0)
                fits.append((r, qn, gv))
        return counts, fits, per

    counts, per_all = {}, {}
    chosen = None
    if policy in ("variable", "variable_else_constant"):
        c, fits, per = run_family("variable")
        counts["variable"], per_all = c, dict(per_all, **per)
        families.append(fits)
    if policy == "constant" or (policy == "variable_else_constant" and not families[0]):
        c, fits, per = run_family("constant")
        counts["constant"], per_all = c, dict(per_all, **per)
        families.append(fits)
    fits = families[-1]
    if fits:
        if m["rule"] == "smallest_capacity":
            key = lambda t: (t[0]["range"][1], t[2], t[0]["id"].encode())
        else:
            key = lambda t: (t[2], t[0]["range"][1], t[0]["id"].encode())
        chosen = min(fits, key=key)
        outcome = ("selected_by_fallback" if policy == "variable_else_constant"
                   and len(families) == 2 else "selected")
    else:
        outcome = "no_record_fits"
    return outcome, chosen, counts, per_all


def verify_tests(rec, Fc, uv, design, W, m):
    """REQ-005 verified values and the fit tests they fail."""
    uC = uv["cold"]
    uops = uv["ops"]
    lo, hi = span(uC, uops)
    req = hi - lo + 2 * m["a"]
    out = {"uC": uC, "uops": uops, "span": (lo, hi), "req": req,
           "t_pass": req <= rec["travel"]}
    failed = []
    if rec["type"] == "variable_spring_hanger":
        k = rec["k"]
        FC = Fc - k * uC
        Fi = {c: Fc - k * u for c, u in uops.items()}
        var = {}
        for c in uops:
            R = Fi[c] if m["ref"] == "operating_load" else W
            var[c] = abs(FC - Fi[c]) / R if R > 0 else None
        env = (Fc - k * (hi + m["a"]), Fc - k * (lo - m["a"]))
        if not (FC > 0 and all(v > 0 for v in Fi.values())):
            failed.append("V0")
        if not (rec["range"][0] <= env[0] and env[1] <= rec["range"][1]):
            failed.append("L")
        if not out["t_pass"]:
            failed.append("T")
        if not all(v <= m["limit"] for v in var.values() if v is not None):
            failed.append("V")
        dep = (Fi[design] if m["design"] == "hot" else FC) - W
        out.update({"FC": FC, "Fi": Fi, "var": var, "dep": dep, "env": env})
    else:
        if not out["t_pass"]:
            failed.append("T")
    out["failed"] = failed
    return out


# ---------------------------------------------------------------- build


def size_runs(ctx, model, runs, catalogue, cand_ids, design, further):
    recs = records(ctx, catalogue)
    byid = {r["id"]: r for r in recs}
    der = model["derived"]
    ops = [design] + further
    results = []
    for rid, ld, ref, lim, a, rule, policies in runs:
        m = {"design": ld, "ref": ref, "limit": ctx.q(lim), "a": ctx.q(a), "rule": rule}
        cands = []
        config = {}
        for h in cand_ids:
            W = model["W"][h]
            uC = der["W"]["u"][h]
            uops = {c: der[c]["u"][h] for c in ops}
            outcome, chosen, counts, per = select(policies[h], recs, W, uC, uops,
                                                  design, m)
            lo, hi = span(uC, uops)
            row = {"id": h, "policy": policies[h], "W": W, "uC": uC, "uops": uops,
                   "outcome": outcome, "chosen": chosen, "counts": counts,
                   "per": per, "span": (lo, hi), "req": hi - lo + 2 * m["a"]}
            if chosen and chosen[0]["type"] == "variable_spring_hanger":
                config[h] = ("spring", chosen[0]["k"], chosen[1]["F_cold"])
            else:
                config[h] = ("force", W)
            cands.append(row)
        # Verification pass (REQ-005).
        vsol = {c: model["verify"](c, config) for c in ["W"] + ops}
        for row in cands:
            h = row["id"]
            if row["chosen"] is None:
                row["ver"] = None
                continue
            rec, qn = row["chosen"][0], row["chosen"][1]
            Fc = qn["F_cold"] if rec["type"] == "variable_spring_hanger" else row["W"]
            uv = {"cold": vsol["W"][h], "ops": {c: vsol[c][h] for c in ops}}
            row["ver"] = verify_tests(rec, Fc, uv, design, row["W"], m)
        standing = ("conditional" if any(r["chosen"] is None for r in cands)
                    else "unconditional")
        results.append({"id": rid, "method": m, "cands": cands, "standing": standing,
                        "config": config})
    return recs, results


def compute(ctx):
    A = model_a(ctx)
    B = model_b(ctx)
    recA, runsA = size_runs(ctx, A, RUNS_A, CATALOGUE_A, ["H-3", "H-5"], "O1", ["O2"])
    recB, runsB = size_runs(ctx, B, RUNS_B, CATALOGUE_B, ["H-3"], "O1", ["O2"])
    return {"A": A, "B": B, "recA": recA, "runsA": runsA, "recB": recB, "runsB": runsB}


def discrete(res):
    out = []
    for key in ("runsA", "runsB"):
        for run in res[key]:
            for c in run["cands"]:
                out.append((run["id"], c["id"], c["outcome"],
                            c["chosen"][0]["id"] if c["chosen"] else None,
                            json.dumps(c["counts"], sort_keys=True),
                            json.dumps(c["per"], sort_keys=True),
                            tuple(c["ver"]["failed"]) if c["ver"] else None,
                            c["ver"]["t_pass"] if c["ver"] else None))
            out.append((run["id"], run["standing"]))
    return out


def f(x):
    return None if x is None else float(x)


def fd(d):
    return {k: f(v) for k, v in d.items()}


# ---------------------------------------------------------------- emit

NULL_RULE = ('In an expected block, a null or an absent member means that this reference supplies no expectation for that quantity; a product comparison must not assert that the product publishes nothing there. The exceptions, where a null or an empty list is itself the expectation, are listed in comparison_semantics.')

SEMANTICS = {
    "general": NULL_RULE,
    "no_expectation": [
        "A constant selection's f_cold_condition_n, f_operating_n, variation "
        "and envelope_n, and in its verification f_cold_condition_n, "
        "f_operating_n, variation and departure_n: the scope defines these "
        "for a variable selection only (REQ-004 Loads and Variation, REQ-005 "
        "Variable selections). Its governing variation, which REQ-004 does "
        "define, is given in governing_variation.",
        "An unfitted candidate's loads, variation, envelope and verification "
        "(see unfitted_verification_note).",
        "failure_counts and first_failed_test of a family the request does "
        "not test (absent members).",
    ],
    "expected_null_or_empty": [
        "hanger_id null with outcome no_record_fits: no record is selected and "
        "nothing is assumed in its place (REQ-004).",
        "verification.failed_tests []: the verified values fail no fit test, "
        "so nothing is warned (REQ-005).",
        "unfitted_verified_as_force_w []: no candidate is unfitted; the "
        "standing is unconditional (REQ-005).",
    ],
    "inputs_not_expectations": [
        "Nulls and empty lists outside expected blocks describe the document "
        "and catalogue: a record member that is absent (stiffness and its DOF "
        "of a constant record, VS-NR's working load range), no imposed "
        "displacement, no excluded stored load, and no internal pressure on "
        "the riser.",
    ],
}

BASIS = (
    "Independent reference for DEL-04-11 hanger sizing on documents that carry "
    "internal pressure, a per-case ledger of stored loads and per-support, "
    "per-case states (CLM-001, REQ-002 to REQ-005, REQ-010), plus one free "
    "riser on a document with no pressurised region. Classical Euler-Bernoulli "
    "beam theory with axial deformation, shear deformation neglected, small "
    "displacements, straight circular tubes of one uniform section: d = D - 2t, "
    "A = pi (D^2 - d^2)/4, I = pi (D^4 - d^4)/64. Global Z is vertical, positive "
    "up. SI units (m, N, Pa, N/m, 1/K); temperatures in degC, temperature "
    "changes in K. Every load is a stored nodal force along Z (no self-weight, "
    "no contents weight and no line load is generated); a case includes stored "
    "loads by id at a stated factor and excludes every other. A candidate acts "
    "on UZ only and its force acts along +Z. Every position is the UZ "
    "displacement from the drawn geometry, the installed position. A variable "
    "spring set to F_cold carries F_cold - k u. Solves: weight_rigid is the "
    "weight case with every candidate a rigid UZ restraint (W is its reaction); "
    "each force solve is a case with every candidate removed and replaced by a "
    "force W along +Z, entered in that case's ledger at factor 1; each "
    "verification solve replaces each selected variable record by a spring of "
    "its rate plus a force F_cold, and each constant selection and each "
    "unfitted candidate by a force W. Exact quantities (fractions.Fraction) "
    "are emitted rounded to the nearest binary64. Quantities that involve E I "
    "or E A contain pi; they are evaluated exactly at rational lower and upper "
    "bounds of pi (Machin's formula, 120 digits) and every emitted binary64 and "
    "every discrete outcome agrees at both bounds. Every discrete outcome is "
    "also unchanged when each input is replaced by the exact value of its "
    "binary64. Every value is invented; no supplier catalogue or standards "
    "data is used. See README.md for the derivations, the pressure-"
    "independence argument and the readings taken. " + NULL_RULE)

SUPPORTS_A = [
    {"id": "S-1", "node": "N1", "kind": "rigid", "dofs": ["UZ"], "candidate": False},
    {"id": "G-1", "node": "N1", "kind": "rigid", "dofs": ["UX", "UY", "RX"],
     "candidate": False},
    {"id": "H-3", "node": "N3", "kind": "candidate", "dofs": ["UZ"], "candidate": True},
    {"id": "H-5", "node": "N5", "kind": "candidate", "dofs": ["UZ"], "candidate": True},
    {"id": "S-7", "node": "N7", "kind": "rigid", "dofs": ["UZ"], "candidate": False},
    {"id": "G-7", "node": "N7", "kind": "rigid", "dofs": ["UY"], "candidate": False},
]

SUPPORTS_B = [
    {"id": "A-1", "node": "N1", "kind": "anchor", "dofs": ALL6, "candidate": False},
    {"id": "H-3", "node": "N3", "kind": "candidate", "dofs": ["UZ"], "candidate": True},
]

DESC_A = (
    "Pressurised straight run along +X, Z up. Nodes N1..N7 at x = 0, 2, ..., 12 m "
    "(y = z = 0), members M1..M6 between consecutive nodes, one section, closed "
    "at N1 and N7. S-1 (N1) and S-7 (N7) are rigid UZ restraints that are never "
    "candidates; G-1 (N1: UX, UY, RX) is the only axial restraint, so the run is "
    "free to grow; G-7 (N7: UY) completes the out-of-plane restraint. Candidates "
    "H-3 (N3, x = 4) and H-5 (N5, x = 8). Stored loads: F-1 -8000 N at N2, F-2 "
    "-6000 N at N4, F-3 -5000 N at N6, F-4 -10000 N at N4 (stored, included by "
    "no named case), F-5 +2000 N at N4, and PR-1 internal pressure 3.5 MPa on "
    "M1..M6. W includes F-1 x1, F-2 x1.5, F-3 x1, PR-1 x1 at 20 degC (the "
    "installation temperature). O1 (design) adds F-5 x0.25 at 260 degC with S-1 "
    "moved -0.012 m and S-7 moved +0.012 m. O2 has W's loads at 180 degC with "
    "S-1 moved +0.009 m and S-7 moved -0.009 m. weight_rigid: three equal 4 m "
    "spans with midspan loads 8000, 9000, 5000 N; three-moment equation "
    "4 M2 + M3 = -(3L/8)(P1 + P2), M2 + 4 M3 = -(3L/8)(P2 + P3) gives M2 = -5400, "
    "M3 = -3900 N m and reactions S-1 2650, H-3 10225, H-5 7600, S-7 1525 N, "
    "independent of E, A and I. Force solves: the run with both candidates "
    "replaced by forces is a simply supported 12 m span (determinate), so each "
    "position is the settlement line plus the deflection of the case's net "
    "forces (zero for W's loads with the forces W); u_O2 is exact rational, "
    "u_O1 contains F-5's deflection 500 f(x, 6) with "
    "f(x, a) = b x (L^2 - b^2 - x^2)/(6 E I L), b = L - a, x <= a. Thermal "
    "growth and pressure act only along X and change no vertical quantity.")

DESC_B = (
    "Free vertical riser with no pressurised region. N1 (0, 0, 0), N2 (0, 0, 4), "
    "N3 (0, 0, 8); M1 N1-N2 and M2 N2-N3, one section; anchor A-1 at N1 (all six "
    "DOFs); candidate H-3 at the top, N3 (UZ). Stored loads F-T -9000 N at N3 and "
    "F-M -4000 N at N2, included at factor 1 in every case. Installation "
    "temperature 15 degC; W at 15 degC, O1 (design) at 315 degC (dT = +300 K), O2 "
    "at -35 degC (dT = -50 K). weight_rigid: a fixed-fixed axial bar; F-T passes "
    "into the rigid candidate and F-M splits equally, so W = 9000 + 2000 = "
    "11000 N and A-1's UZ reaction is 2000 N, independent of E A. Force solves: "
    "with H-3 a force W the riser is a cantilever; the upper half carries "
    "+2000 N and the lower half -2000 N, so the mechanical shortening is zero "
    "and u = alpha dT H from the installation temperature: u_C = 0, "
    "u_O1 = 1.2e-5 * 300 * 8 = 0.0288 m, u_O2 = -0.0048 m.")


def section_out():
    return {"outside_diameter_m": float(Fraction(SECTION["D"])),
            "wall_thickness_m": float(Fraction(SECTION["t"])),
            "elastic_modulus_pa": float(Fraction(SECTION["E"])),
            "poisson_ratio": float(Fraction(SECTION["nu"])),
            "thermal_expansion_per_k": float(Fraction(SECTION["alpha"]))}


def records_out(cat):
    out = []
    for rid, typ, k, dof, rng, trav in cat:
        out.append({"hanger_id": rid, "hanger_type": typ,
                    "stiffness_n_per_m": float(Fraction(k)) if k else None,
                    "stiffness_dof": dof,
                    "working_load_range_n": [float(Fraction(rng[0])),
                                             float(Fraction(rng[1]))] if rng else None,
                    "travel_range_m": float(Fraction(trav)) if trav else None})
    return out


def run_out(run, desc, design, further, ops):
    m = run["method"]
    cands = []
    for c in run["cands"]:
        rec, qn = (c["chosen"][0], c["chosen"][1]) if c["chosen"] else (None, None)
        var = rec is not None and rec["type"] == "variable_spring_hanger"
        row = {
            "support": c["id"], "family_policy": c["policy"], "outcome": c["outcome"],
            "hanger_id": rec["id"] if rec else None,
            "w_n": f(c["W"]), "u_cold_m": f(c["uC"]),
            "u_operating_m": fd(c["uops"]),
            "f_cold_n": f(qn["F_cold"]) if var else (f(c["W"]) if rec else None),
            "f_cold_condition_n": f(qn["F_C"]) if var else None,
            "f_operating_n": fd(qn["F_i"]) if var else None,
            "variation": fd(qn["var"]) if var else None,
            "span_m": [f(c["span"][0]), f(c["span"][1])],
            "required_travel_m": f(c["req"]),
            "envelope_n": [f(qn["env"][0]), f(qn["env"][1])] if var else None,
            "governing_variation": (
                None if rec is None else
                f(max(qn["var"].values())) if var else 0.0),
            "failure_counts": c["counts"],
            "first_failed_test": c["per"],
            "verification": None,
        }
        v = c["ver"]
        if v:
            row["verification"] = {
                "configuration": ({"kind": "spring_plus_force", "stiffness_n_per_m":
                                   f(rec["k"]), "force_n": f(qn["F_cold"])} if var
                                  else {"kind": "force", "force_n": f(c["W"])}),
                "u_cold_m": f(v["uC"]), "u_operating_m": fd(v["uops"]),
                "span_m": [f(v["span"][0]), f(v["span"][1])],
                "required_travel_m": f(v["req"]), "t_pass": v["t_pass"],
                "f_cold_condition_n": f(v["FC"]) if var else None,
                "f_operating_n": fd(v["Fi"]) if var else None,
                "variation": fd(v["var"]) if var else None,
                "departure_n": f(v["dep"]) if var else None,
                "failed_tests": v["failed"],
            }
        cands.append(row)
    unfitted = [c["id"] for c in run["cands"] if c["chosen"] is None]
    return {
        "id": run["id"], "description": desc,
        "request": {
            "weight_case": "W", "weight_support_state": "weight_case_own",
            "design_operating_case": design, "further_operating_cases": further,
            "load_design": m["design"], "variation_reference": m["ref"],
            "variation_limit": f(m["limit"]), "travel_allowance_m": f(m["a"]),
            "selection_rule": m["rule"],
            "candidates": [{"support": c["id"], "family_policy": c["policy"]}
                           for c in run["cands"]]},
        "expected": dict(
            {"candidates": cands,
             "verification_standing": run["standing"],
             "unfitted_verified_as_force_w": unfitted},
            **({"unfitted_verification_note": UNFITTED_NOTE} if unfitted else {})),
    }


UNFITTED_NOTE = (
    "The verification of an unfitted candidate is null because this reference "
    "supplies no expectation for it. The scope requires verified values for "
    "every selected family and says nothing of an unfitted candidate, so a "
    "product comparison must not assert that the product publishes nothing "
    "there.")

RUN_DESC = {
    "A-HOT": (
        "Hot design, operating-load variation, limit 0.25, allowance 1 mm, smallest "
        "capacity. H-3 variable: F_cold = W + k u_O1. VS-B fits; VS-C fails L at the "
        "low end; VS-V (k = 800000) passes L and T and fails V in O2. H-5 constant: "
        "CE-1 (travel 11 mm) fits the sized travel and is selected over CE-4 by "
        "capacity; in the verification the spring at H-3 pulls H-5 further out "
        "and CE-1 fails T, which is warned and changes no selection. In a linear "
        "model the departure is exactly zero (the spring set by hot design "
        "carries W at u_O1, which is the force solve's equilibrium)."),
    "A-COLD": (
        "As A-HOT with cold design only: F_cold = W, so H-3's envelope moves up "
        "by k u_O1 and the selection changes: VS-B fails L at the high end and "
        "VS-C is selected. The verified cold position is exactly zero. CE-1 again "
        "fails T in the verification."),
    "A-NOFIT": (
        "As A-HOT with H-5 under the variable policy: no variable record fits H-5 "
        "(L, and V for VS-V), so H-5 is 'no record fits'. This reference supplies "
        "no verification expectation for H-5 (see unfitted_verification_note); "
        "the verification pass takes it as a force W, and the standing is "
        "conditional. H-3's "
        "verified values equal A-HOT's, because both runs verify H-5 as a force W."),
    "B-COLD": (
        "Riser, cold design, design-load variation (reference W), limit 0.3, "
        "allowance 2 mm, least variation. VS-R1 (k = 50000) and VS-R2 (k = 80000) "
        "fit; least variation takes VS-R1 although VS-R2 has the smaller "
        "capacity; VS-R3 fails V. Verification: one spring on an axial bar, "
        "u^v = alpha dT H / (1 + k H/(E A)), u_C^v = 0, departure 0."),
}


def emit(res):
    A, B = res["A"], res["B"]
    models = []
    for key, model, data, desc, sup, runs, cat, recs in (
            ("A", A, A_DATA, DESC_A, SUPPORTS_A, res["runsA"], CATALOGUE_A, res["recA"]),
            ("B", B, B_DATA, DESC_B, SUPPORTS_B, res["runsB"], CATALOGUE_B, res["recB"])):
        if key == "A":
            nodes = {n: [float(Fraction(v)), 0.0, 0.0] for n, v in data["x"].items()}
        else:
            nodes = {n: [0.0, 0.0, float(Fraction(v))] for n, v in data["z"].items()}
        names = list(nodes)
        members = [{"id": "M%d" % (i + 1), "end_nodes": [names[i], names[i + 1]]}
                   for i in range(len(names) - 1)]
        stored = [{"id": l, "kind": "nodal_force", "node": n, "direction": "+Z",
                   "value_n": float(Fraction(v))} for l, n, v in data["loads"]]
        tin = Fraction(data["t_install"])
        cases = []
        for case in data["cases"]:
            cid, role, temp, ledger = case[:4]
            imposed = case[4] if len(case) > 4 else []
            included = [l for l, _ in ledger]
            entry = {"id": cid, "role": role,
                     "temperature_c": float(Fraction(temp)),
                     "delta_t_k": float(Fraction(temp) - tin),
                     "includes": [{"load": l, "factor": float(Fraction(fac))}
                                  for l, fac in ledger],
                     "excluded_stored_loads": [s["id"] for s in stored
                                               if s["id"] not in included],
                     "imposed_displacements": [{"support": s, "dof": "UZ",
                                                "value_m": float(Fraction(g))}
                                               for s, g in imposed],
                     "candidate_support_state": "active, no imposed motion",
                     "internal_pressure_pa": (float(Fraction(data["pressure"][1]))
                                              if key == "A" else None)}
            cases.append(entry)
        if key == "A":
            stored.append({"id": "PR-1", "kind": "internal_pressure",
                           "members": [m["id"] for m in members], "closed_ends":
                           ["N1", "N7"], "value_pa": float(Fraction(data["pressure"][1]))})
            for c in cases:
                c["excluded_stored_loads"] = [s["id"] for s in stored if s["id"] not in
                                              [i["load"] for i in c["includes"]]]
        der = model["derived"]
        expected = {
            "weight_rigid_reactions_n": fd(model["rigid"]),
            "hot_load_basis_n": fd(model["W"]),
            "force_solve_positions_m": {c: fd(der[c]["u"]) for c in ("W", "O1", "O2")},
            "force_solve_reactions_n": {c: fd(der[c]["reactions"])
                                        for c in ("W", "O1", "O2")},
            "added_force_sources": {c: [{"support": h, "node": dict(
                (s["id"], s["node"]) for s in sup)[h], "dof": "UZ",
                "value_n": f(model["W"][h]), "factor": 1.0} for h in model["W"]]
                for c in ("W", "O1", "O2")},
            "failure_signature_positions_m": {
                c: fd(der[c]["failure_u"]) for c in ("W", "O1", "O2")},
        }
        models.append({
            "id": "EXS-" + key, "description": desc,
            "pressurised": key == "A",
            "section": section_out(),
            "installation_temperature_c": float(tin),
            "nodes": nodes, "members": members, "supports": sup,
            "stored_loads": stored, "load_cases": cases,
            "catalogue": records_out(cat),
            "expected": expected,
            "runs": [run_out(r, RUN_DESC[r["id"]], "O1", ["O2"], ["O1", "O2"])
                     for r in runs],
        })
    return {"schema": SCHEMA, "basis": BASIS, "comparison_semantics": SEMANTICS,
            "models": models}


def stated_checks(res):
    """The values and outcomes the README states, asserted on every run."""
    A, B = res["A"], res["B"]
    assert A["rigid"] == {"S-1": 2650, "H-3": 10225, "H-5": 7600, "S-7": 1525}
    assert A["derived"]["O2"]["u"] == {"H-3": Fraction("0.003"), "H-5": Fraction("-0.003")}
    assert A["derived"]["W"]["u"] == {"H-3": 0, "H-5": 0}
    assert B["rigid"] == {"A-1": 2000, "H-3": 11000}
    assert B["derived"]["O1"]["u"]["H-3"] == Fraction("0.0288")
    assert B["derived"]["O2"]["u"]["H-3"] == Fraction("-0.0048")
    runs = {r["id"]: {c["id"]: c for c in r["cands"]} for r in res["runsA"] + res["runsB"]}
    sel = {(r, h): (c["outcome"], c["chosen"][0]["id"] if c["chosen"] else None)
           for r, cs in runs.items() for h, c in cs.items()}
    assert sel == {("A-HOT", "H-3"): ("selected", "VS-B"),
                   ("A-HOT", "H-5"): ("selected", "CE-1"),
                   ("A-COLD", "H-3"): ("selected", "VS-C"),
                   ("A-COLD", "H-5"): ("selected", "CE-1"),
                   ("A-NOFIT", "H-3"): ("selected", "VS-B"),
                   ("A-NOFIT", "H-5"): ("no_record_fits", None),
                   ("B-COLD", "H-3"): ("selected", "VS-R1")}
    for r in ("A-HOT", "A-COLD"):
        assert runs[r]["H-5"]["ver"]["failed"] == ["T"]
        assert runs[r]["H-3"]["ver"]["failed"] == []
    assert runs["A-COLD"]["H-3"]["ver"]["uC"] == 0
    for r in runs.values():
        for c in r.values():
            if c["ver"] and "dep" in c["ver"]:
                assert c["ver"]["dep"] == 0
    # A-NOFIT verifies H-3 exactly as A-HOT does.
    assert runs["A-NOFIT"]["H-3"]["ver"] == runs["A-HOT"]["H-3"]["ver"]
    # The variable records least_variation and smallest_capacity disagree on.
    pr = runs["B-COLD"]["H-3"]["per"]
    assert pr["VS-R1"] == pr["VS-R2"] == "fits"


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--check", action="store_true",
                    help="exit 1 if the frozen JSON differs from a fresh generation")
    args = ap.parse_args()
    res = compute(Ctx(PI_MID, False))
    stated_checks(res)
    doc = emit(res)
    ref = discrete(res)
    for pi in (PI_LO, PI_HI):
        other = compute(Ctx(pi, False))
        assert emit(other) == doc, "a value differs between the bounds of pi"
        assert discrete(other) == ref
    for pi in (PI_LO, PI_MID, PI_HI):
        assert discrete(compute(Ctx(pi, True))) == ref, "binary64 inputs change an outcome"
    text = json.dumps(doc, indent=2, ensure_ascii=False) + "\n"
    if args.check:
        if not OUT.exists() or OUT.read_text(encoding="utf-8") != text:
            print("ex_sizing.json differs from a fresh generation", file=sys.stderr)
            return 1
        print("ex_sizing.json matches")
        return 0
    OUT.write_text(text, encoding="utf-8")
    print("wrote", OUT.name)
    return 0


if __name__ == "__main__":
    sys.exit(main())
