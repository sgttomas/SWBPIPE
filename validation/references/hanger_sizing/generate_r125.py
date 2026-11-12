#!/usr/bin/env python3
"""Independent reference generator for hanger-sizing derived solves (DEL-04-11).

Covers R-1 (rigid reactions of continuous beams), R-2 (free riser growth) and
R-5 (lift-off) of DEL-04-11: the sizing quantities W (solve S_W), u_C (S_C)
and u_i (S_Oi) per candidate, and each lift-off support's state in every solve.

Independence: written from the design text, the task brief and classical
Euler-Bernoulli beam and frame mechanics only. It reads, searches and imports no
product code (core/, apps/, tests/). Standard library only. Every value is
invented; no standards or supplier data is used.

Method: every expected value comes from a closed form written below for the
case. As a self-check, each solve is also run through a small exact-rational
plane-frame direct-stiffness solve (Euler-Bernoulli, axial EA included,
consistent loads, active-set iteration for lift-off) written here, and the
closed form must agree with it exactly. Every solve's vertical equilibrium is
asserted, and for R-5 the active-set consistency (compressive reaction while
active; zero reaction and an upward-opening gap while inactive).

Usage:
    python3 generate_r125.py          # write r125_derived_solves.json
    python3 generate_r125.py --check  # exit 1 if the frozen JSON differs
"""

import argparse
import json
import math
import sys
from decimal import Decimal, localcontext
from fractions import Fraction
from pathlib import Path

SCHEMA = "swbpipe.hanger_sizing_reference.r125/1"
OUT = Path(__file__).resolve().parent / "r125_derived_solves.json"

# pi to 101 significant digits (a mathematical constant, not data).
PI = Decimal(
    "3.1415926535897932384626433832795028841971693993751058209749445923078164"
    "062862089986280348253421170679")
assert float(PI) == math.pi

ALL6 = ["UX", "UY", "UZ", "RX", "RY", "RZ"]

BASIS = (
    "Independent reference for DEL-04-11 hanger-sizing derived solves: R-1 "
    "(rigid reactions of continuous beams), R-2 (free riser growth) and R-5 "
    "(lift-off), from DEL-04-11 "
    "and classical Euler-Bernoulli beam and frame mechanics. Global "
    "Z is vertical, positive up; gravity acts along -Z. SI units (m, N, Pa, N/m, "
    "1/K, K). Every member is a straight circular tube: d = D - 2t, "
    "A = pi*(D^2 - d^2)/4, I = pi*(D^4 - d^4)/64; shear deformation is neglected "
    "and axial deformation (EA) is included. line_loads give the magnitude w of "
    "a uniform line load acting along -Z on the named member (no self-weight is "
    "added); nodal_forces give fz positive along +Z; delta_t_k is a uniform "
    "temperature change of the named member, free expansion alpha*dT per unit "
    "length. A candidate acts on UZ only, positive along +Z. Each operating case "
    "carries the weight case's line loads. Solves (section 4.2): w_n is the "
    "candidate's UZ reaction in the weight case with every candidate a rigid UZ "
    "restraint (S_W); u_cold_m is its UZ displacement in the weight case with "
    "every candidate replaced by an upward nodal force w_n and no restraint "
    "(S_C); u_operating_m is its UZ displacement in each operating case with the "
    "same replacement (S_Oi). Displacements are measured from the drawn "
    "geometry. A lift_off_uz support restrains UZ only while its reaction is "
    "compressive (pushing the pipe up, reaction >= 0); when inactive its "
    "reaction is zero and its UZ gap opens upward (u >= 0). nonlinear_states "
    "gives each lift-off support's state in S_W (weight_rigid), S_C (cold) and "
    "each S_Oi (by case id). Every case is planar in the X-Z plane and loaded "
    "only in that plane; the out-of-plane DOFs (UY, RX, RZ) carry no load, and "
    "no expected value depends on the shear modulus or Poisson ratio. Inputs are "
    "the exact rational values of the decimals in the generator. Exact "
    "quantities: every w_n and every non-candidate reaction (rational in w and "
    "the lengths, independent of E, A and I), every u_cold_m (exactly 0), R-2's "
    "u_operating_m (alpha*dT*H, rational), R-1's and R-5b's u_operating_m "
    "(exactly 0); these are computed with fractions.Fraction and emitted "
    "rounded to the nearest binary64. Closed-form floating-point quantities: "
    "R-5a's u_operating_m, which is a rational divided by pi; it is evaluated "
    "with pi to 101 significant digits in 80-digit decimal arithmetic and then "
    "rounded to binary64. Each closed form is checked exactly against an "
    "independent exact-rational plane-frame stiffness solve, every solve's "
    "vertical equilibrium is asserted, and the lift-off states are confirmed "
    "unchanged when every input is replaced by its emitted binary64 value. "
    "Every value is invented; no standards or supplier data is used."
)


# ---------------------------------------------------------------- numbers


def q(x):
    """Exact rational from a decimal string, int or Fraction (floats refused)."""
    if isinstance(x, Fraction):
        return x
    if isinstance(x, (str, int)) and not isinstance(x, bool):
        return Fraction(x)
    raise TypeError(f"inexact input {x!r}; use a decimal string or Fraction")


def dec(f):
    return Decimal(f.numerator) / Decimal(f.denominator)


def disp_value(u):
    """u = (a, b) meaning a + b/pi -> Decimal (80 digits)."""
    a, b = u
    with localcontext() as ctx:
        ctx.prec = 80
        return dec(a) + dec(b) / PI


def force_value(r):
    """r = (r0, r1) meaning r0 + r1*pi -> Decimal (80 digits)."""
    r0, r1 = r
    with localcontext() as ctx:
        ctx.prec = 80
        return dec(r0) + dec(r1) * PI


def disp_float(u):
    a, b = u
    if b == 0:
        return float(a)  # int / int true division: correctly rounded
    with localcontext() as ctx:
        ctx.prec = 80
        return float(str(disp_value(u)))  # string -> float: correctly rounded


ZERO = (Fraction(0), Fraction(0))


def section_hat(D, t):
    """A = pi*A_hat, I = pi*I_hat for a circular tube."""
    d = D - 2 * t
    assert d > 0
    return (D * D - d * d) / 4, (D ** 4 - d ** 4) / 64


# ---------------------------------------------------------------- model helpers


def member(mid, n1, n2, D, t, E, alpha):
    return {"id": mid, "nodes": (n1, n2), "D": q(D), "t": q(t), "E": q(E),
            "alpha": q(alpha)}


def support(sid, node, kind, dofs):
    return {"id": sid, "node": node, "kind": kind, "dofs": list(dofs)}


def load_case(lcid, role, line_loads, dt=(), nf=()):
    return {"id": lcid, "role": role,
            "line_loads": [(m, q(w)) for m, w in line_loads],
            "dt": [(m, q(v)) for m, v in dt],
            "nf": [(n, q(f)) for n, f in nf]}


def length(nodes, m):
    (x1, y1, z1), (x2, y2, z2) = nodes[m["nodes"][0]], nodes[m["nodes"][1]]
    diffs = [abs(x2 - x1), abs(y2 - y1), abs(z2 - z1)]
    nz = [v for v in diffs if v != 0]
    assert len(nz) == 1, "members are axis-aligned in these cases"
    return nz[0]


def solve_keys(case):
    return ["weight_rigid", "cold"] + [lc["id"] for lc in case["load_cases"]
                                       if lc["role"] == "operating"]


def solve_load_case(case, key):
    if key in ("weight_rigid", "cold"):
        (lc,) = [lc for lc in case["load_cases"] if lc["role"] == "weight"]
        return lc
    (lc,) = [lc for lc in case["load_cases"] if lc["id"] == key]
    return lc


# ---------------------------------------------------------------- exact frame solve
#
# Plane frame in X-Z, DOFs per node (ux, uz, th), th the in-plane rotation (RY up
# to sign; only its restraint is used). Stiffness K = pi*K_hat with K_hat
# rational, because A and I both carry pi. Mechanical loads F_m are rational;
# thermal loads are pi*F_t_hat. Displacements are a + b/pi with
# K_hat a = F_t_hat and K_hat b = F_m; reactions are r0 + r1*pi.


def gauss(K, rhss):
    n = len(K)
    M = [row[:] + [r[i] for r in rhss] for i, row in enumerate(K)]
    for c in range(n):
        p = next(i for i in range(c, n) if M[i][c] != 0)
        M[c], M[p] = M[p], M[c]
        piv = M[c][c]
        M[c] = [v / piv for v in M[c]]
        for i in range(n):
            if i != c and M[i][c] != 0:
                f = M[i][c]
                M[i] = [vi - f * vc for vi, vc in zip(M[i], M[c])]
    return [[M[i][n + k] for i in range(n)] for k in range(len(rhss))]


DOF2D = {"UX": 0, "UZ": 1, "RY": 2}


def frame_solve(case, key, W=None):
    """Solve one derived solve. W=None: candidates rigid (S_W); else force W."""
    nodes = case["nodes"]
    ids = list(nodes)
    idx = {n: i for i, n in enumerate(ids)}
    for n in ids:
        assert nodes[n][1] == 0, "plane frame in X-Z"
    nd = 3 * len(ids)
    K = [[Fraction(0)] * nd for _ in range(nd)]
    Fm = [Fraction(0)] * nd
    Ft = [Fraction(0)] * nd
    lc = solve_load_case(case, key)
    wl = dict(lc["line_loads"])
    dt = dict(lc["dt"])
    for m in case["members"]:
        i, j = (idx[n] for n in m["nodes"])
        (x1, _, z1), (x2, _, z2) = nodes[m["nodes"][0]], nodes[m["nodes"][1]]
        L = length(nodes, m)
        c, s = (x2 - x1) / L, (z2 - z1) / L
        Ah, Ih = section_hat(m["D"], m["t"])
        ea, ei = m["E"] * Ah, m["E"] * Ih
        kl = [[Fraction(0)] * 6 for _ in range(6)]
        for (p, r, v) in ((0, 0, 1), (0, 3, -1), (3, 0, -1), (3, 3, 1)):
            kl[p][r] = ea / L * v
        bend = [[12, 6 * L, -12, 6 * L], [6 * L, 4 * L * L, -6 * L, 2 * L * L],
                [-12, -6 * L, 12, -6 * L], [6 * L, 2 * L * L, -6 * L, 4 * L * L]]
        bmap = (1, 2, 4, 5)
        for p in range(4):
            for r in range(4):
                kl[bmap[p]][bmap[r]] = ei / L ** 3 * bend[p][r]
        T = [[Fraction(0)] * 6 for _ in range(6)]
        for o in (0, 3):
            T[o][o], T[o][o + 1] = c, s
            T[o + 1][o], T[o + 1][o + 1] = -s, c
            T[o + 2][o + 2] = Fraction(1)
        kg = [[sum(T[a][p] * kl[a][b] * T[b][r] for a in range(6) for b in range(6))
               for r in range(6)] for p in range(6)]
        g = [3 * i, 3 * i + 1, 3 * i + 2, 3 * j, 3 * j + 1, 3 * j + 2]
        for p in range(6):
            for r in range(6):
                K[g[p]][g[r]] += kg[p][r]
        # uniform global load (0, -w): local axial qa, transverse qt
        w = wl.get(m["id"], Fraction(0))
        qa, qt = s * (-w), c * (-w)
        fl = [qa * L / 2, qt * L / 2, qt * L * L / 12,
              qa * L / 2, qt * L / 2, -qt * L * L / 12]
        # thermal: pi * E * A_hat * alpha * dT pushes the ends apart
        th = m["E"] * Ah * m["alpha"] * dt.get(m["id"], Fraction(0))
        ftl = [-th, 0, 0, th, 0, 0]
        for p in range(6):
            Fm[g[p]] += sum(T[a][p] * fl[a] for a in range(6))
            Ft[g[p]] += sum(T[a][p] * ftl[a] for a in range(6))
    for n, fz in lc["nf"]:
        Fm[3 * idx[n] + 1] += fz
    base = {}  # restrained dof -> support id
    lifts = []
    for sp in case["supports"]:
        k = sp["kind"]
        if k == "lift_off_uz":
            lifts.append(sp)
            continue
        if k == "candidate":
            if W is not None:
                Fm[3 * idx[sp["node"]] + 1] += W[sp["id"]]
                continue
        for dname in sp["dofs"]:
            if dname in DOF2D:
                dof = 3 * idx[sp["node"]] + DOF2D[dname]
                assert dof not in base, "two supports on one in-plane DOF"
                base[dof] = sp["id"]
    active = {sp["id"]: True for sp in lifts}
    for _ in range(10):
        rest = dict(base)
        for sp in lifts:
            if active[sp["id"]]:
                dof = 3 * idx[sp["node"]] + 1
                assert dof not in rest
                rest[dof] = sp["id"]
        free = [d for d in range(nd) if d not in rest]
        Kff = [[K[a][b] for b in free] for a in free]
        a_f, b_f = gauss(Kff, [[Ft[d] for d in free], [Fm[d] for d in free]])
        a = [Fraction(0)] * nd
        b = [Fraction(0)] * nd
        for d, va, vb in zip(free, a_f, b_f):
            a[d], b[d] = va, vb
        reac = {}
        for d, sid in rest.items():
            kd = K[d]
            r1 = sum(kd[e] * a[e] for e in range(nd)) - Ft[d]
            r0 = sum(kd[e] * b[e] for e in range(nd)) - Fm[d]
            if d % 3 == 1:
                reac[sid] = (r0, r1)
        changed = False
        for sp in lifts:
            d = 3 * idx[sp["node"]] + 1
            if active[sp["id"]] and force_value(reac[sp["id"]]) < 0:
                active[sp["id"]], changed = False, True
            elif not active[sp["id"]] and disp_value((a[d], b[d])) < 0:
                active[sp["id"]], changed = True, True
        if not changed:
            break
    else:
        raise AssertionError("active set did not settle")
    for sp in lifts:
        if not active[sp["id"]]:
            reac[sp["id"]] = ZERO
    # applied UZ total (pi part from thermal is purely axial-internal; its UZ
    # resultant must vanish too)
    app0 = sum(Fm[d] for d in range(1, nd, 3))
    app1 = sum(Ft[d] for d in range(1, nd, 3))
    assert sum(r[0] for r in reac.values()) + app0 == 0
    assert sum(r[1] for r in reac.values()) + app1 == 0
    uz = {n: (a[3 * idx[n] + 1], b[3 * idx[n] + 1]) for n in ids}
    return {"reactions": reac, "uz": uz,
            "states": {sp["id"]: "active" if active[sp["id"]] else "inactive"
                       for sp in lifts}}


# ---------------------------------------------------------------- closed-form checks


def applied_uz(case, key, W):
    """Total applied UZ force of a solve (exact; line loads act along -Z)."""
    lc = solve_load_case(case, key)
    mem = {m["id"]: m for m in case["members"]}
    tot = -sum(w * length(case["nodes"], mem[m]) for m, w in lc["line_loads"])
    tot += sum(f for _, f in lc["nf"])
    if key != "weight_rigid":
        tot += sum(W.values())
    return tot


def check_case(case):
    cf = case["closed"]
    W = cf["W"]
    keys = solve_keys(case)
    cands = [sp for sp in case["supports"] if sp["kind"] == "candidate"]
    for key in keys:
        # closed-form vertical equilibrium: sum of UZ reactions + applied = 0
        reac = cf["reactions"][key]
        assert sum(r[0] for r in reac.values()) + applied_uz(case, key, W) == 0, \
            (case["id"], key, "equilibrium")
        assert sum(r[1] for r in reac.values()) == 0, (case["id"], key)
        # independent exact frame solve agrees exactly
        fs = frame_solve(case, key, None if key == "weight_rigid" else W)
        assert fs["reactions"] == reac, (case["id"], key, fs["reactions"], reac)
        assert fs["states"] == cf["states"][key], (case["id"], key, fs["states"])
        if key == "weight_rigid":
            for sp in cands:
                assert fs["reactions"][sp["id"]] == (W[sp["id"]], 0)
        else:
            for sp in cands:
                assert fs["uz"][sp["node"]] == cf["u"][key][sp["id"]], \
                    (case["id"], key, sp["id"], fs["uz"][sp["node"]])
        # active-set consistency of each lift-off support
        for sp in case["supports"]:
            if sp["kind"] != "lift_off_uz":
                continue
            r, g = reac[sp["id"]], fs["uz"][sp["node"]]
            if cf["states"][key][sp["id"]] == "active":
                assert force_value(r) >= 0 and g == ZERO, (case["id"], key)
            else:
                assert r == ZERO and disp_value(g) > 0, (case["id"], key)
                assert g == cf["gaps"][key][sp["id"]], (case["id"], key, g)
    for sp in cands:
        assert W[sp["id"]] > 0
        assert cf["u"]["cold"][sp["id"]] == ZERO  # linear at the cold state


# ---------------------------------------------------------------- cases


SEC = {"D": "0.25", "t": "0.0125", "E": "200000000000", "alpha": "0.0000125"}


def r1a(w="1500", L="6"):
    w, L = q(w), q(L)
    nodes = {"N1": (0, 0, 0), "N2": (L, 0, 0), "N3": (2 * L, 0, 0)}
    s = SEC
    # Closed form (three-moment, M_A = M_C = 0, equal spans): M_B = -w L^2/8;
    # ends R = w L/2 + M_B/L = 3wL/8; interior W = 2(wL/2 - M_B/L) = 5wL/4.
    # S_C: the force W reproduces the rigid equilibrium (linear), u_C = 0.
    # S_O1: the beam is axially restrained at N1 only, so free growth along X
    # changes no UZ; u_O1 = 0 and the reactions are unchanged.
    W = 5 * w * L / 4
    end = 3 * w * L / 8
    reac = {"S-1": (end, 0), "S-3": (end, 0), "H-2": (W, 0)}
    reac_f = {"S-1": (end, 0), "S-3": (end, 0)}
    return {
        "id": "R-1a",
        "description": (
            "Two equal spans L = 6 m on three supports, uniform w = 1500 N/m on both "
            "spans. Ends N1 and N3 are rigid_uz (S-1, S-3). Rigid-body motion is "
            "removed by guide G-1 at N1 (UX, UY, RX) and guide G-3 at N3 (UY): UX at "
            "one end only, so axial growth is free; UY at both ends removes UY and RZ; "
            "RX at N1 removes rotation about the beam axis, which no translational "
            "restraint on collinear nodes can remove. No end restrains RY (an anchor "
            "at an end would fix its rotation and change W). Interior N2 is the "
            "candidate H-2. Three-moment equation with M_A = M_C = 0: "
            "M_B = -wL^2/8, so each end reaction is wL/2 + M_B/L = 3wL/8 = 3375 N and "
            "W = 2(wL/2 - M_B/L) = 5wL/4 = 11250 N, independent of E, A and I. "
            "u_C = 0: in a linear model the force W reproduces the rigid equilibrium. "
            "Operating case O1 adds dT = 150 K on both members: growth along X is "
            "unrestrained (UX at N1 only) and changes no UZ, so u_O1 = 0."),
        "nodes": nodes,
        "members": [member("M1", "N1", "N2", **s), member("M2", "N2", "N3", **s)],
        "supports": [
            support("S-1", "N1", "rigid_uz", ["UZ"]),
            support("G-1", "N1", "guide", ["UX", "UY", "RX"]),
            support("H-2", "N2", "candidate", ["UZ"]),
            support("S-3", "N3", "rigid_uz", ["UZ"]),
            support("G-3", "N3", "guide", ["UY"]),
        ],
        "load_cases": [
            load_case("W", "weight", [("M1", w), ("M2", w)]),
            load_case("O1", "operating", [("M1", w), ("M2", w)],
                      dt=[("M1", "150"), ("M2", "150")]),
        ],
        "closed": {
            "W": {"H-2": W},
            "reactions": {"weight_rigid": reac, "cold": reac_f, "O1": reac_f},
            "u": {"cold": {"H-2": ZERO}, "O1": {"H-2": ZERO}},
            "states": {"weight_rigid": {}, "cold": {}, "O1": {}},
            "gaps": {},
        },
        "stated": [(W, Fraction(11250)), (end, Fraction(3375))],
    }


def r1b(w="2000", L="5"):
    w, L = q(w), q(L)
    nodes = {"N1": (0, 0, 0), "N2": (L, 0, 0), "N3": (2 * L, 0, 0),
             "N4": (3 * L, 0, 0)}
    s = SEC
    # Closed form (three-moment, equal spans, symmetric): M_B = M_C,
    # 2 M_B (2L) + M_C L = -wL^3/2 -> M_B = M_C = -wL^2/10.
    # Ends R = wL/2 + M_B/L = 2wL/5; interior W = wL/2 - M_B/L + wL/2 = 11wL/10.
    W = 11 * w * L / 10
    end = 2 * w * L / 5
    reac = {"S-1": (end, 0), "S-4": (end, 0), "H-2": (W, 0), "H-3": (W, 0)}
    reac_f = {"S-1": (end, 0), "S-4": (end, 0)}
    return {
        "id": "R-1b",
        "description": (
            "Three equal spans L = 5 m, uniform w = 2000 N/m on all spans. Ends N1, N4 "
            "rigid_uz (S-1, S-4) with guides G-1 at N1 (UX, UY, RX) and G-4 at N4 (UY), "
            "as in R-1a. Interior N2, N3 are candidates H-2, H-3. Three-moment "
            "equation, symmetric: M_B = M_C = -wL^2/10, so each end reaction is "
            "wL/2 + M_B/L = 2wL/5 = 4000 N and each W = (wL/2 - M_B/L) + wL/2 = "
            "11wL/10 = 11000 N (the outer span's share plus half the symmetric "
            "centre span). u_C = 0 at both candidates (linear). Operating case O1 adds "
            "dT = 150 K on all members; growth along X is free, so u_O1 = 0."),
        "nodes": nodes,
        "members": [member("M1", "N1", "N2", **s), member("M2", "N2", "N3", **s),
                    member("M3", "N3", "N4", **s)],
        "supports": [
            support("S-1", "N1", "rigid_uz", ["UZ"]),
            support("G-1", "N1", "guide", ["UX", "UY", "RX"]),
            support("H-2", "N2", "candidate", ["UZ"]),
            support("H-3", "N3", "candidate", ["UZ"]),
            support("S-4", "N4", "rigid_uz", ["UZ"]),
            support("G-4", "N4", "guide", ["UY"]),
        ],
        "load_cases": [
            load_case("W", "weight", [("M1", w), ("M2", w), ("M3", w)]),
            load_case("O1", "operating", [("M1", w), ("M2", w), ("M3", w)],
                      dt=[("M1", "150"), ("M2", "150"), ("M3", "150")]),
        ],
        "closed": {
            "W": {"H-2": W, "H-3": W},
            "reactions": {"weight_rigid": reac, "cold": reac_f, "O1": reac_f},
            "u": {"cold": {"H-2": ZERO, "H-3": ZERO},
                  "O1": {"H-2": ZERO, "H-3": ZERO}},
            "states": {"weight_rigid": {}, "cold": {}, "O1": {}},
            "gaps": {},
        },
        "stated": [(W, Fraction(11000)), (end, Fraction(4000))],
    }


def r1c(w="1200", L1="4", L2="6"):
    w, L1, L2 = q(w), q(L1), q(L2)
    assert L1 != L2
    nodes = {"N1": (0, 0, 0), "N2": (L1, 0, 0), "N3": (L1 + L2, 0, 0)}
    s = SEC
    # Closed form (three-moment, M_A = M_C = 0, uniform EI):
    #   2 M_B (L1 + L2) = -w (L1^3 + L2^3)/4
    #   M_B = -w (L1^3 + L2^3) / (8 (L1 + L2))
    #   R_A = w L1/2 + M_B/L1,  R_C = w L2/2 + M_B/L2,
    #   W = R_B = w (L1 + L2)/2 - M_B (1/L1 + 1/L2).
    MB = -w * (L1 ** 3 + L2 ** 3) / (8 * (L1 + L2))
    RA = w * L1 / 2 + MB / L1
    RC = w * L2 / 2 + MB / L2
    W = w * (L1 + L2) / 2 - MB * (1 / L1 + 1 / L2)
    reac = {"S-1": (RA, 0), "S-3": (RC, 0), "H-2": (W, 0)}
    reac_f = {"S-1": (RA, 0), "S-3": (RC, 0)}
    return {
        "id": "R-1c",
        "description": (
            "Two unequal spans L1 = 4 m (N1-N2) and L2 = 6 m (N2-N3), uniform "
            "w = 1200 N/m on both, same section. Ends rigid_uz (S-1, S-3) with guides "
            "G-1 at N1 (UX, UY, RX) and G-3 at N3 (UY), as in R-1a; interior N2 is the "
            "candidate H-2. Three-moment equation with M_A = M_C = 0 and uniform EI: "
            "M_B = -w(L1^3 + L2^3)/(8(L1 + L2)) = -4200 N m; "
            "R_A = wL1/2 + M_B/L1 = 1350 N; R_C = wL2/2 + M_B/L2 = 2900 N; "
            "W = w(L1 + L2)/2 - M_B(1/L1 + 1/L2) = 7750 N (exact; independent of E, A "
            "and I). u_C = 0 (linear). Operating case O1 adds dT = 150 K on both "
            "members; growth along X is free, so u_O1 = 0."),
        "nodes": nodes,
        "members": [member("M1", "N1", "N2", **s), member("M2", "N2", "N3", **s)],
        "supports": [
            support("S-1", "N1", "rigid_uz", ["UZ"]),
            support("G-1", "N1", "guide", ["UX", "UY", "RX"]),
            support("H-2", "N2", "candidate", ["UZ"]),
            support("S-3", "N3", "rigid_uz", ["UZ"]),
            support("G-3", "N3", "guide", ["UY"]),
        ],
        "load_cases": [
            load_case("W", "weight", [("M1", w), ("M2", w)]),
            load_case("O1", "operating", [("M1", w), ("M2", w)],
                      dt=[("M1", "150"), ("M2", "150")]),
        ],
        "closed": {
            "W": {"H-2": W},
            "reactions": {"weight_rigid": reac, "cold": reac_f, "O1": reac_f},
            "u": {"cold": {"H-2": ZERO}, "O1": {"H-2": ZERO}},
            "states": {"weight_rigid": {}, "cold": {}, "O1": {}},
            "gaps": {},
        },
        "stated": [(W, Fraction(7750)), (RA, Fraction(1350)), (RC, Fraction(2900))],
    }


def r2(H="6", Lh="4", wr="900", wh="1800", dT1="160", dT2="80"):
    H, Lh, wr, wh, dT1, dT2 = (q(v) for v in (H, Lh, wr, wh, dT1, dT2))
    alpha = q(SEC["alpha"])
    nodes = {"N1": (0, 0, 0), "N2": (0, 0, H), "N3": (Lh, 0, H)}
    s = SEC
    # Closed form.
    # S_W: UZ(N2) = 0 is prescribed, and UZ(N1) = 0 by the anchor. In a straight
    # member, small-displacement axial and bending responses are uncoupled, so
    # the riser's axial problem is a fixed-fixed bar under its own uniform axial
    # load w_r: each end carries w_r H/2. The leg is a cantilever from N2; its
    # shear w_h Lh arrives at N2, where UZ is rigid and the riser's axial force is
    # already fixed by the prescribed end displacements, so the restraint takes
    # it all. Its end moment w_h Lh^2/2 bends the riser laterally (UX, RY at N2)
    # and changes no UZ. W = w_h Lh + w_r H/2; anchor UZ reaction = w_r H/2.
    # S_C: the riser is a cantilever bar; axial force N(z) = W - w_h Lh
    # - w_r (H - z) = w_r H/2 - w_r (H - z), and u(H) = int_0^H N/EA dz = 0.
    # S_Oi: candidate as force, the frame is statically determinate (a
    # cantilever from the anchor), so uniform dT gives free growth and no
    # stress: the riser lengthens by alpha dT H, the leg's growth is along X.
    # u_Oi = 0 + alpha dT_i H; reactions unchanged.
    W = wh * Lh + wr * H / 2
    ra = wr * H / 2
    u1 = alpha * dT1 * H
    u2 = alpha * dT2 * H
    reac = {"A-1": (ra, 0), "H-2": (W, 0)}
    reac_f = {"A-1": (ra, 0)}
    return {
        "id": "R-2",
        "description": (
            "Free riser. Riser M1 vertical from N1 (0,0,0) to N2 (0,0,H), H = 6 m, "
            "anchored at N1 (A-1, all six DOF). Leg M2 horizontal along +X from N2 to "
            "N3 (Lh,0,H), Lh = 4 m, with no other support; N3 is free. The candidate "
            "H-2 is at the riser top N2 (UZ). Line loads w_r = 900 N/m on the riser "
            "(along -Z, axial) and w_h = 1800 N/m on the leg. S_W: UZ is prescribed "
            "zero at both riser ends, and in a straight member axial and bending "
            "responses are uncoupled, so the riser's own weight is shared half and "
            "half as in a fixed-fixed bar under uniform axial load (w_r H/2 each); "
            "the leg's shear w_h Lh reaches the corner, where the rigid UZ takes it "
            "all; the leg's end moment w_h Lh^2/2 bends the riser laterally and "
            "changes no UZ. W = w_h Lh + w_r H/2 = 7200 + 2700 = 9900 N; the anchor's "
            "UZ reaction is w_r H/2 = 2700 N; neither depends on E, A or I. S_C: with "
            "the upward force W the riser's axial force is N(z) = w_r H/2 - "
            "w_r (H - z), whose integral over the height is zero, so u_C = 0. S_Oi: "
            "with the candidate as a force the frame is a statically determinate "
            "cantilever, so uniform dT gives free growth with no stress; the riser "
            "lengthens by alpha dT H and the leg grows along X, so "
            "u_i = alpha dT_i H (alpha = 1.25e-5 1/K): O1 dT = 160 K on riser and "
            "leg, u_O1 = 0.012 m; O2 dT = 80 K, u_O2 = 0.006 m. The hanger force is "
            "the same W cold and hot; the anchor reaction is unchanged."),
        "nodes": nodes,
        "members": [member("M1", "N1", "N2", **s), member("M2", "N2", "N3", **s)],
        "supports": [
            support("A-1", "N1", "anchor", ALL6),
            support("H-2", "N2", "candidate", ["UZ"]),
        ],
        "load_cases": [
            load_case("W", "weight", [("M1", wr), ("M2", wh)]),
            load_case("O1", "operating", [("M1", wr), ("M2", wh)],
                      dt=[("M1", dT1), ("M2", dT1)]),
            load_case("O2", "operating", [("M1", wr), ("M2", wh)],
                      dt=[("M1", dT2), ("M2", dT2)]),
        ],
        "closed": {
            "W": {"H-2": W},
            "reactions": {"weight_rigid": reac, "cold": reac_f, "O1": reac_f,
                          "O2": reac_f},
            "u": {"cold": {"H-2": ZERO}, "O1": {"H-2": (u1, Fraction(0))},
                  "O2": {"H-2": (u2, Fraction(0))}},
            "states": {"weight_rigid": {}, "cold": {}, "O1": {}, "O2": {}},
            "gaps": {},
        },
        "stated": [(W, Fraction(9900)), (ra, Fraction(2700)),
                   (u1, Fraction("0.012")), (u2, Fraction("0.006"))],
    }


def r5(cid, P, w="1400", L="4"):
    w, L, P = q(w), q(L), q(P)
    nodes = {"N1": (0, 0, 0), "N2": (L, 0, 0), "N3": (2 * L, 0, 0)}
    s = SEC
    E = q(s["E"])
    _, Ih = section_hat(q(s["D"]), q(s["t"]))
    # Closed form.
    # S_W (lift-off active): fixed at x = 0, simple supports at L and 2L, equal
    # spans, uniform w. Three-moment with a zero-length span at the fixed end:
    #   2 M_A + M_B = -wL^2/4,  M_A + 4 M_B = -wL^2/2
    #   -> M_B = -3wL^2/28, M_A = -wL^2/14.
    #   Span reactions V_l = wL/2 + (M_r - M_l)/L, V_r = wL/2 + (M_l - M_r)/L:
    #   R_A = 13wL/28, W = R_B = 15wL/28 + 17wL/28 = 8wL/7, R_C = 11wL/28 > 0
    #   (compressive, so the lift-off support is active: consistent).
    # S_C (candidate force W, lift-off active): the rigid field is reproduced
    #   (u = 0 at B and C), R_C = 11wL/28 > 0, active (consistent; the stiffness
    #   is positive definite, so the active-set solution is unique).
    # S_O (force W at B, upward P at C):
    #   active trial: a force applied at a restrained node passes straight into
    #   the restraint, so R_C = 11wL/28 - P and u = 0 at B and C.
    #   If P > 11wL/28 that reaction is tensile, so the support is inactive:
    #   start from the S_C field (a cantilever of length 2L carrying w, W at B and
    #   R_C0 = 11wL/28 at C, with zero displacement at B and C) and add the excess
    #   dP = P - 11wL/28 at C on the cantilever. Cantilever flexibilities for a
    #   tip load at b = 2L: at a = L, a^2 (3b - a)/(6EI) = 5L^3/(6EI); at the tip,
    #   b^3/(3EI) = 8L^3/(3EI). So u_B = dP 5L^3/(6EI) and gap_C = dP 8L^3/(3EI)
    #   > 0 (opens upward), R_C = 0: consistent. I = pi I_hat.
    #   Anchor UZ reaction = 2wL - W - P.
    W = 8 * w * L / 7
    RA = 13 * w * L / 28
    RC0 = 11 * w * L / 28
    reac_w = {"A-1": (RA, 0), "H-2": (W, 0), "L-3": (RC0, 0)}
    reac_c = {"A-1": (RA, 0), "L-3": (RC0, 0)}
    dP = P - RC0
    if dP > 0:
        state = "inactive"
        uB = (Fraction(0), dP * 5 * L ** 3 / (6 * E * Ih))
        gap = (Fraction(0), dP * 8 * L ** 3 / (3 * E * Ih))
        reac_o = {"A-1": (2 * w * L - W - P, 0), "L-3": ZERO}
        gaps = {"O1": {"L-3": gap}}
    else:
        state = "active"
        uB = ZERO
        reac_o = {"A-1": (RA, 0), "L-3": (RC0 - P, 0)}
        gaps = {}
    if cid == "R-5a":
        tail = (
            "O1 adds an upward nodal force P = 5000 N at N3 > 11wL/28 = 2200 N. The "
            "active trial would need R_C = 11wL/28 - P = -2800 N (tensile), so L-3 "
            "is inactive: superposing the excess dP = P - 11wL/28 = 2800 N at the tip "
            "of the cantilever (length 2L) on the S_C field gives "
            "u_O1 = dP 5L^3/(6EI) at the candidate and an upward gap "
            "dP 8L^3/(3EI) at N3 with zero reaction (consistent); "
            "I = pi(D^4 - d^4)/64. The anchor UZ reaction is 2wL - W - P = 1800 N. "
            "u_O1 is a rational divided by pi, evaluated in floating point.")
    else:
        tail = (
            "O1 adds an upward nodal force P = 1000 N at N3, below 11wL/28 = 2200 N. "
            "A force applied at a restrained node passes straight into the "
            "restraint, so R_C = 11wL/28 - P = 1200 N is still compressive: L-3 "
            "stays active (consistent), the displacement field is the S_C field, "
            "u_O1 = 0, and the anchor reaction stays 13wL/28.")
    desc = (
        "Lift-off beam. Members M1 N1-N2 and M2 N2-N3 along +X, L = 4 m each, "
        "uniform w = 1400 N/m on both. Anchor A-1 at N1 (x = 0, all six DOF), "
        "candidate H-2 at N2 (x = L), lift_off_uz support L-3 at N3 (x = 2L). "
        "S_W (L-3 active): fixed-pinned-pinned two-span beam; three-moment with a "
        "zero-length span at the fixed end gives M_A = -wL^2/14, M_B = -3wL^2/28, "
        "so R_A = 13wL/28 = 2600 N, W = 8wL/7 = 6400 N, R_C = 11wL/28 = 2200 N "
        "> 0 (active, consistent). S_C (H-2 as upward force W, L-3 active): the "
        "rigid equilibrium is reproduced, R_C = 2200 N > 0, u_C = 0. " + tail)
    return {
        "id": cid,
        "description": desc,
        "nodes": nodes,
        "members": [member("M1", "N1", "N2", **s), member("M2", "N2", "N3", **s)],
        "supports": [
            support("A-1", "N1", "anchor", ALL6),
            support("H-2", "N2", "candidate", ["UZ"]),
            support("L-3", "N3", "lift_off_uz", ["UZ"]),
        ],
        "load_cases": [
            load_case("W", "weight", [("M1", w), ("M2", w)]),
            load_case("O1", "operating", [("M1", w), ("M2", w)], nf=[("N3", P)]),
        ],
        "closed": {
            "W": {"H-2": W},
            "reactions": {"weight_rigid": reac_w, "cold": reac_c, "O1": reac_o},
            "u": {"cold": {"H-2": ZERO}, "O1": {"H-2": uB}},
            "states": {"weight_rigid": {"L-3": "active"},
                       "cold": {"L-3": "active"}, "O1": {"L-3": state}},
            "gaps": gaps,
        },
        "stated": [(W, Fraction(6400)), (RA, Fraction(2600)),
                   (RC0, Fraction(2200))],
        "intent": "inactive" if cid == "R-5a" else "active",
    }


def build_cases():
    return [r1a(), r1b(), r1c(), r2(), r5("R-5a", "5000"), r5("R-5b", "1000")]


def binarized_cases():
    """Same cases with every input replaced by the exact value of its binary64."""
    def b(x):
        return Fraction(float(q(x)))
    saved = dict(SEC)
    try:
        for k in SEC:
            SEC[k] = b(SEC[k])
        return [r1a(b("1500"), b("6")), r1b(b("2000"), b("5")),
                r1c(b("1200"), b("4"), b("6")),
                r2(b("6"), b("4"), b("900"), b("1800"), b("160"), b("80")),
                r5("R-5a", b("5000"), b("1400"), b("4")),
                r5("R-5b", b("1000"), b("1400"), b("4"))]
    finally:
        SEC.update(saved)


# ---------------------------------------------------------------- emit


def emit_case(case):
    cf = case["closed"]
    op_ids = [lc["id"] for lc in case["load_cases"] if lc["role"] == "operating"]
    lifts = [sp["id"] for sp in case["supports"] if sp["kind"] == "lift_off_uz"]
    return {
        "id": case["id"],
        "description": case["description"],
        "nodes": {n: [float(v) for v in xyz] for n, xyz in case["nodes"].items()},
        "members": [{
            "id": m["id"],
            "end_nodes": list(m["nodes"]),
            "outside_diameter_m": float(m["D"]),
            "wall_thickness_m": float(m["t"]),
            "elastic_modulus_pa": float(m["E"]),
            "thermal_expansion_per_k": float(m["alpha"]),
        } for m in case["members"]],
        "supports": [{"id": sp["id"], "node": sp["node"], "kind": sp["kind"],
                      "dofs": sp["dofs"]} for sp in case["supports"]],
        "load_cases": [{
            "id": lc["id"],
            "role": lc["role"],
            "line_loads": [{"member": m, "w_n_per_m": float(w)}
                           for m, w in lc["line_loads"]],
            "delta_t_k": [{"member": m, "value": float(v)} for m, v in lc["dt"]],
            "nodal_forces": [{"node": n, "fz_n": float(f)} for n, f in lc["nf"]],
        } for lc in case["load_cases"]],
        "expected": {
            "candidates": [{
                "support": sp["id"],
                "node": sp["node"],
                "w_n": float(cf["W"][sp["id"]]),
                "u_cold_m": disp_float(cf["u"]["cold"][sp["id"]]),
                "u_operating_m": {k: disp_float(cf["u"][k][sp["id"]])
                                  for k in op_ids},
            } for sp in case["supports"] if sp["kind"] == "candidate"],
            "nonlinear_states": {sid: {k: cf["states"][k][sid]
                                       for k in solve_keys(case)}
                                 for sid in lifts},
        },
    }


def build_document():
    cases = build_cases()
    ids = [c["id"] for c in cases]
    assert len(ids) == len(set(ids)), "duplicate case id"
    for case in cases:
        check_case(case)
        for got, want in case["stated"]:
            assert got == want, (case["id"], got, want)
        if "intent" in case:
            assert case["closed"]["states"]["O1"]["L-3"] == case["intent"]
    # discrete outcomes and values are insensitive to binary64 input rounding
    for case, bcase in zip(cases, binarized_cases()):
        check_case(bcase)
        assert case["closed"]["states"] == bcase["closed"]["states"], case["id"]
        e, be = emit_case(case)["expected"], emit_case(bcase)["expected"]
        for c, bc in zip(e["candidates"], be["candidates"]):
            pairs = [(c["w_n"], bc["w_n"]), (c["u_cold_m"], bc["u_cold_m"])]
            pairs += [(c["u_operating_m"][k], bc["u_operating_m"][k])
                      for k in c["u_operating_m"]]
            for x, y in pairs:
                assert abs(x - y) <= 1e-12 * max(abs(x), 1e-300), (case["id"], x, y)
    return {"schema": SCHEMA, "basis": BASIS,
            "cases": [emit_case(c) for c in cases]}


def render():
    return json.dumps(build_document(), indent=2) + "\n"


def main(argv=None):
    p = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    p.add_argument("--check", action="store_true",
                   help="exit 1 if the frozen JSON differs from the generated output")
    args = p.parse_args(argv)
    text = render()
    if args.check:
        try:
            frozen = OUT.read_text(encoding="utf-8")
        except FileNotFoundError:
            print(f"missing: {OUT}", file=sys.stderr)
            return 1
        if frozen != text:
            print(f"differs: {OUT}", file=sys.stderr)
            return 1
        print(f"ok: {OUT.name} matches ({len(json.loads(text)['cases'])} cases)")
        return 0
    OUT.write_text(text, encoding="utf-8")
    print(f"wrote {OUT}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
