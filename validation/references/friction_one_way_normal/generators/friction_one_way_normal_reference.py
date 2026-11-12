#!/usr/bin/env python3
"""Friction support whose normal is the reaction of a one-way support at the same node.

Independent analytical references: an invented L-shaped pipe frame anchored at
its foot, whose free end rests on a one-way (resting) support that can lift off
and carries a friction support taking its normal from that one-way support.

Independence. Python 3 standard library only (fractions, json, hashlib,
argparse, pathlib). Reads, imports and runs no product code, product output,
fixture or schema. Every value is derived from mechanics and the governing scope
text: DEL-04-04 (friction law, one-way and lift-off state laws, state names,
REQ-013) and DEL-04-07 (nonlinear iteration term, CLM-002, AC-002). All inputs
are invented. The models carry no pressure.

Two formulations, sharing only the input data, the section formula and the
state law (which defines the problem):
  A. Closed form. Flexibility of the cantilever frame rooted at the anchor, by
     hand-derived unit-load integrals written out as formulas; the support
     forces at S are the redundants, solved per state combination from the 2x2
     compatibility at S; everything else by statics. Every one of the eight
     state combinations is tried and the state law checked on each.
  B. Direct stiffness. Independently assembled planar Euler-Bernoulli frame
     (6x6 element matrices, rotated to global), solved with the support states
     as extra equations, by exact Gauss-Jordan elimination. The state is found
     by an active-set iteration (solve, classify, update) from two different
     starting states, and separately by enumerating all eight combinations.
A and B must agree exactly on every published value, exactly one combination
must satisfy the state law, and every equilibrium, closed-form and robustness
check must hold, or nothing is written and the exit status is 1.

Usage:  python3 -I -B friction_one_way_normal_reference.py [--check]  # default: compare frozen bytes
        python3 -I -B friction_one_way_normal_reference.py --write    # rewrite references.json and SHA256SUMS
"""

from fractions import Fraction as Fr
import argparse
import hashlib
import json
import sys
from pathlib import Path

# ---------------------------------------------------------------------------
# Inputs (invented; every one is exactly representable in binary64)
# ---------------------------------------------------------------------------
INPUTS = {
    "E": Fr(200_000_000_000),   # Pa
    "nu": Fr(1, 4),             # not exercised: all response is in plane (no torsion)
    "D": Fr(1, 4),              # m, outside diameter 0.25
    "t": Fr(1, 64),             # m, wall 0.015625
    "h": Fr(3),                 # m, riser A -> B
    "a": Fr(2),                 # m, run B -> S
    "mu": Fr(3, 8),             # friction coefficient 0.375
}
PI_DIGITS_TERMS = 80            # Machin terms; gives a pi enclosure far tighter than binary64 needs


class CheckFailure(Exception):
    pass


class Checks:
    def __init__(self):
        self.items = []

    def eq(self, kind, name, x, y):
        if x != y:
            raise CheckFailure(f"{kind}: {name}: {x} != {y}")
        self.items.append(kind)

    def true(self, kind, name, cond):
        if not cond:
            raise CheckFailure(f"{kind}: {name}")
        self.items.append(kind)


# ---------------------------------------------------------------------------
# pi enclosure (Machin) and correctly rounded binary64 emission
# ---------------------------------------------------------------------------
def atan_inv_bounds(n, terms):
    """atan(1/n) enclosed by consecutive partial sums of the alternating series."""
    s = Fr(0)
    sums = []
    for k in range(terms + 1):
        s += Fr((-1) ** k, (2 * k + 1) * n ** (2 * k + 1))
        sums.append(s)
    lo, hi = sorted(sums[-2:])
    return lo, hi


def pi_bounds():
    a_lo, a_hi = atan_inv_bounds(5, PI_DIGITS_TERMS)
    b_lo, b_hi = atan_inv_bounds(239, PI_DIGITS_TERMS)
    return 16 * a_lo - 4 * b_hi, 16 * a_hi - 4 * b_lo


PI_LO, PI_HI = pi_bounds()


def fr_text(q):
    return str(q.numerator) if q.denominator == 1 else f"{q.numerator}/{q.denominator}"


def num(q, unit):
    """A pi-free exact rational: binary64 by correct rounding (int/int true division)."""
    return {"exact": fr_text(q), "value": float(q) if q != 0 else 0.0, "unit": unit}


def num_over_pi(q, unit):
    """q/pi: the nearest binary64 is emitted only when both ends of the enclosure round alike."""
    if q == 0:
        return {"exact": "0", "value": 0.0, "unit": unit}
    ends = sorted((q / PI_LO, q / PI_HI))
    lo, hi = float(ends[0]), float(ends[1])
    if lo != hi:
        raise CheckFailure(f"pi enclosure too wide to round {q}/pi")
    return {"exact": f"({fr_text(q)})/pi", "value": lo, "unit": unit}


# ---------------------------------------------------------------------------
# Section: annulus with zero corrosion allowance and mill tolerance. A and I
# carry a factor pi, which is kept outside: A = pi*Ah, I = pi*Ih.
# ---------------------------------------------------------------------------
def section(inp):
    D, t = inp["D"], inp["t"]
    d = D - 2 * t
    Ah = (D * D - d * d) / 4
    Ih = (D ** 4 - d ** 4) / 64
    return {"Ah": Ah, "Ih": Ih, "EAh": inp["E"] * Ah, "EIh": inp["E"] * Ih}


# ---------------------------------------------------------------------------
# Model. Planar in X-Z (Z up). A(0,0,0) anchor; B(0,0,h); S(a,0,h).
# riser A->B, run B->S. At S: one-way support on UZ, sense + (bears upward);
# friction support on UX at S whose normal is that one-way support's reaction.
# Loads: nodal forces only, Gx at B and Px, Pz at S (possibly several primitive
# loads per node; only their sums enter).
# Displacements are computed with pi set aside: u_hat = pi*u. Reactions do not
# depend on pi (all flexibilities scale by 1/pi together).
# ---------------------------------------------------------------------------
STATES_ONEWAY = ("active", "inactive")
STATES_FRICTION = ("inactive", "stick", "slide+", "slide-")


def dirn(fs):
    return {"slide+": 1, "slide-": -1}[fs]


# ----------------------------- formulation A --------------------------------
def a_displacements(inp, sec, Fx, Fz, Gx, Gz):
    """Hand-derived unit-load closed forms for the cantilever frame rooted at A,
    under total nodal forces (Fx, Fz) at S and (Gx, Gz) at B. Returns pi*u."""
    h, a, EI, EA = inp["h"], inp["a"], sec["EIh"], sec["EAh"]
    X = Fx + Gx
    return {
        "S": {
            "UX": (X * h ** 3 / 3 - a * Fz * h ** 2 / 2) / EI + Fx * a / EA,
            "UZ": (-a * h ** 2 * X / 2 + a ** 2 * h * Fz + a ** 3 * Fz / 3) / EI + (Fz + Gz) * h / EA,
            "RY": (X * h ** 2 / 2 - a * Fz * h - Fz * a ** 2 / 2) / EI,
        },
        "B": {
            "UX": (X * h ** 3 / 3 - a * Fz * h ** 2 / 2) / EI,
            "UZ": (Fz + Gz) * h / EA,
            "RY": (X * h ** 2 / 2 - a * Fz * h) / EI,
        },
    }


def a_flex(inp, sec):
    """f_xx, f_xz, f_zz at S (pi*f), as written in the README."""
    h, a, EI, EA = inp["h"], inp["a"], sec["EIh"], sec["EAh"]
    return (h ** 3 / (3 * EI) + a / EA, -a * h ** 2 / (2 * EI), (a ** 3 / 3 + a ** 2 * h) / EI + h / EA)


def a_state_forces(inp, sec, loads, ow, fs):
    """Support forces (Rx friction, Rz one-way) at S for a given state pair."""
    fxx, fxz, fzz = a_flex(inp, sec)
    mu = inp["mu"]
    u0 = a_displacements(inp, sec, loads["Px"], loads["Pz"], loads["Gx"], Fr(0))["S"]
    ux0, uz0 = u0["UX"], u0["UZ"]
    if ow == "inactive":
        Rz = Fr(0)
        Rx = -ux0 / fxx if fs == "stick" else Fr(0)
    elif fs == "stick":
        det = fxx * fzz - fxz * fxz
        Rx = -(fzz * ux0 - fxz * uz0) / det
        Rz = -(-fxz * ux0 + fxx * uz0) / det
    elif fs == "inactive":
        Rx, Rz = Fr(0), -uz0 / fzz
    else:
        d = dirn(fs)
        Rz = -uz0 / (fzz - d * mu * fxz)
        Rx = -d * mu * Rz
    return Rx, Rz


def law_violations(mu, ow, fs, Rx, Rz, ux, uz):
    """DEL-04-04 state law, each deciding quantity strictly on its state's side."""
    v = []
    if ow == "active" and not Rz > 0:
        v.append("one-way active needs reaction > 0 (bears)")
    if ow == "inactive" and not uz > 0:
        v.append("one-way inactive needs separation u_z > 0 (released support would penetrate)")
    bears = ow == "active" and Rz > 0
    N = Rz if ow == "active" else Fr(0)
    if fs == "inactive" and bears:
        v.append("friction inactive needs a normal that does not bear")
    if fs != "inactive" and not bears:
        v.append("friction stick or slide needs a normal that bears")
    if fs == "stick" and bears and not abs(Rx) < mu * N:
        v.append("stick needs |T| < mu*N")
    if fs in ("slide+", "slide-") and bears and not dirn(fs) * ux > 0:
        v.append("slide needs motion u_x in its own direction")
    return v


def a_solve(inp, sec, loads):
    """Try all eight state pairs in closed form; return the admissible one(s)."""
    out = []
    for ow in STATES_ONEWAY:
        for fs in STATES_FRICTION:
            Rx, Rz = a_state_forces(inp, sec, loads, ow, fs)
            uS = a_displacements(inp, sec, loads["Px"] + Rx, loads["Pz"] + Rz, loads["Gx"], Fr(0))["S"]
            out.append({"state": (ow, fs), "Rx": Rx, "Rz": Rz,
                        "violations": law_violations(inp["mu"], ow, fs, Rx, Rz, uS["UX"], uS["UZ"])})
    return out


def a_results(inp, sec, loads, Rx, Rz):
    """Published values of formulation A, given the support forces at S."""
    h, a = inp["h"], inp["a"]
    Fx, Fz, Gx = loads["Px"] + Rx, loads["Pz"] + Rz, loads["Gx"]
    X = Fx + Gx
    disp = a_displacements(inp, sec, Fx, Fz, Gx, Fr(0))
    reac = {
        "anchor A": {"FX": -X, "FZ": -Fz, "MY": -(h * X - a * Fz)},
        "one-way S": {"FZ": Rz},
        "friction S": {"FX": Rx},
    }
    # end action = force or moment the node exerts on the member, global axes
    ends = {
        "riser": {"A": {"FX": -X, "FZ": -Fz, "MY": a * Fz - h * X},
                  "B": {"FX": X, "FZ": Fz, "MY": -a * Fz}},
        "run": {"B": {"FX": -Fx, "FZ": -Fz, "MY": a * Fz},
                "S": {"FX": Fx, "FZ": Fz, "MY": Fr(0)}},
    }
    axial = {"riser": Fz, "run": Fx}   # tension positive
    return disp, reac, ends, axial


# ----------------------------- formulation B --------------------------------
def gauss_jordan(Amat, b):
    n = len(Amat)
    M = [list(row) + [bi] for row, bi in zip(Amat, b)]
    for c in range(n):
        p = next((r for r in range(c, n) if M[r][c] != 0), None)
        if p is None:
            raise CheckFailure("singular system")
        M[c], M[p] = M[p], M[c]
        pv = M[c][c]
        M[c] = [x / pv for x in M[c]]
        for r in range(n):
            if r != c and M[r][c] != 0:
                f = M[r][c]
                M[r] = [x - f * y for x, y in zip(M[r], M[c])]
    return [M[r][n] for r in range(n)]


def b_element(x1, z1, x2, z2, EA, EI):
    """Global 6x6 for dofs (UX1, UZ1, RY1, UX2, UZ2, RY2). Built in the standard
    plane (x, y) = (X, Z) with counter-clockwise rotation theta = -RY, then mapped."""
    dx, dz = x2 - x1, z2 - z1
    L2 = dx * dx + dz * dz
    L = Fr(int(round(float(L2) ** 0.5)))
    if L * L != L2:
        raise CheckFailure("member length not rational")
    c, s = dx / L, dz / L
    k = [[Fr(0)] * 6 for _ in range(6)]
    ea, b12, b6, b4, b2 = EA / L, 12 * EI / L ** 3, 6 * EI / L ** 2, 4 * EI / L, 2 * EI / L
    k[0][0] = k[3][3] = ea; k[0][3] = k[3][0] = -ea
    k[1][1] = k[4][4] = b12; k[1][4] = k[4][1] = -b12
    k[1][2] = k[2][1] = k[1][5] = k[5][1] = b6
    k[4][2] = k[2][4] = k[4][5] = k[5][4] = -b6
    k[2][2] = k[5][5] = b4; k[2][5] = k[5][2] = b2
    T = [[Fr(0)] * 6 for _ in range(6)]
    for o in (0, 3):
        T[o][o], T[o][o + 1] = c, s
        T[o + 1][o], T[o + 1][o + 1] = -s, c
        T[o + 2][o + 2] = Fr(1)
    kg = [[sum(T[m][i] * k[m][n] * T[n][j] for m in range(6) for n in range(6)) for j in range(6)] for i in range(6)]
    # map standard theta (CCW from X toward Z) to RY: theta = -RY
    sign = [1, 1, -1, 1, 1, -1]
    return [[sign[i] * sign[j] * kg[i][j] for j in range(6)] for i in range(6)]


def b_model(inp, sec):
    h, a = inp["h"], inp["a"]
    nodes = {"A": (Fr(0), Fr(0)), "B": (Fr(0), h), "S": (a, h)}
    members = {"riser": ("A", "B"), "run": ("B", "S")}
    order = ["A", "B", "S"]
    dof = {(n, c): 3 * i + j for i, n in enumerate(order) for j, c in enumerate(("UX", "UZ", "RY"))}
    K = [[Fr(0)] * 9 for _ in range(9)]
    ke = {}
    for m, (n1, n2) in members.items():
        k = b_element(*nodes[n1], *nodes[n2], sec["EAh"], sec["EIh"])
        ke[m] = k
        idx = [dof[(n1, c)] for c in ("UX", "UZ", "RY")] + [dof[(n2, c)] for c in ("UX", "UZ", "RY")]
        for i in range(6):
            for j in range(6):
                K[idx[i]][idx[j]] += k[i][j]
    return {"K": K, "ke": ke, "dof": dof, "members": members}


def b_solve_state(inp, mdl, loads, ow, fs):
    """Unknowns: the six free displacements (B, S) and the two support forces at S.
    Six equilibrium rows K_ff u - f - r = 0, one row per support state."""
    mu, dof, K = inp["mu"], mdl["dof"], mdl["K"]
    free = [dof[(n, c)] for n in ("B", "S") for c in ("UX", "UZ", "RY")]
    f = [Fr(0)] * 9
    f[dof[("B", "UX")]] += loads["Gx"]
    f[dof[("S", "UX")]] += loads["Px"]
    f[dof[("S", "UZ")]] += loads["Pz"]
    iSx, iSz = free.index(dof[("S", "UX")]), free.index(dof[("S", "UZ")])
    rows, rhs = [], []
    for i, gi in enumerate(free):
        row = [K[gi][gj] for gj in free] + [Fr(0), Fr(0)]
        if i == iSx:
            row[6] = Fr(-1)
        if i == iSz:
            row[7] = Fr(-1)
        rows.append(row)
        rhs.append(f[gi])
    z = [Fr(0)] * 8
    if ow == "active":
        r = list(z); r[iSz] = Fr(1)            # u_z(S) = 0
    else:
        r = list(z); r[7] = Fr(1)              # R_z = 0
    rows.append(r); rhs.append(Fr(0))
    r = list(z)
    if fs == "stick":
        r[iSx] = Fr(1)                         # u_x(S) = 0
    elif fs == "inactive":
        r[6] = Fr(1)                           # R_x = 0
    else:
        r[6] = Fr(1); r[7] = dirn(fs) * mu     # R_x + d*mu*N = 0 with N = R_z (coupled, k = current iterate)
        if ow != "active":
            r = list(z); r[6] = Fr(1)          # no normal: no force
    rows.append(r); rhs.append(Fr(0))
    x = gauss_jordan(rows, rhs)
    u = [Fr(0)] * 9
    for i, gi in enumerate(free):
        u[gi] = x[i]
    return u, x[6], x[7], f


def b_results(inp, mdl, u, Rx, Rz, f):
    dof, K = mdl["dof"], mdl["K"]
    disp = {n: {c: u[dof[(n, c)]] for c in ("UX", "UZ", "RY")} for n in ("S", "B")}
    rA = {c: sum(K[dof[("A", c)]][j] * u[j] for j in range(9)) - f[dof[("A", c)]] for c in ("UX", "UZ", "RY")}
    reac = {"anchor A": {"FX": rA["UX"], "FZ": rA["UZ"], "MY": rA["RY"]},
            "one-way S": {"FZ": Rz}, "friction S": {"FX": Rx}}
    ends = {}
    for m, (n1, n2) in mdl["members"].items():
        ue = [u[dof[(n, c)]] for n in (n1, n2) for c in ("UX", "UZ", "RY")]
        fe = [sum(mdl["ke"][m][i][j] * ue[j] for j in range(6)) for i in range(6)]
        ends[m] = {n1: {"FX": fe[0], "FZ": fe[1], "MY": fe[2]}, n2: {"FX": fe[3], "FZ": fe[4], "MY": fe[5]}}
    return disp, reac, ends


def b_classify(mu, ow, fs, u, Rx, Rz, dof):
    """One classification step of the active-set iteration (state-switched test)."""
    ux, uz = u[dof[("S", "UX")]], u[dof[("S", "UZ")]]
    if ow == "active":
        ow2 = "active" if Rz > 0 else "inactive"
    else:
        ow2 = "active" if uz < 0 else "inactive"
    bears = ow == "active" and Rz > 0 and ow2 == "active"
    if not bears:
        fs2 = "inactive"
    elif fs == "inactive":
        fs2 = "stick"
    elif fs == "stick":
        fs2 = "stick" if abs(Rx) < mu * Rz else ("slide+" if Rx < 0 else "slide-")
    else:
        fs2 = fs if dirn(fs) * ux > 0 else "stick"
    return ow2, fs2


def b_iterate(inp, mdl, loads, start, cap=50):
    ow, fs = start
    path = [f"{ow}/{fs}"]
    for _ in range(cap):
        u, Rx, Rz, f = b_solve_state(inp, mdl, loads, ow, fs)
        ow2, fs2 = b_classify(inp["mu"], ow, fs, u, Rx, Rz, mdl["dof"])
        if (ow2, fs2) == (ow, fs):
            return (ow, fs), u, Rx, Rz, f, path
        ow, fs = ow2, fs2
        path.append(f"{ow}/{fs}")
    raise CheckFailure("active-set iteration did not settle")


# ---------------------------------------------------------------------------
# Cases
# ---------------------------------------------------------------------------
def L(**kw):
    return {k: Fr(v) for k, v in kw.items()}


BASE_CASES = [
    ("STICK", "Weight on the shoe and a moderate lateral load at the riser top: the shoe bears and sticks.",
     [("B", "FX", 2000), ("S", "FZ", -10000)]),
    ("SLIDE-POS-B", "Lateral load +X at B beyond the stick limit: slides +X. The +X load also presses S down, so N exceeds the weight.",
     [("B", "FX", 8000), ("S", "FZ", -10000)]),
    ("SLIDE-NEG-B", "Lateral load -X at B beyond the stick limit: slides -X. The -X load lifts S, so N is below the weight.",
     [("B", "FX", -8000), ("S", "FZ", -10000)]),
    ("SLIDE-POS-S", "Lateral load +X applied at the shoe itself: slides +X.",
     [("S", "FX", 6000), ("S", "FZ", -10000)]),
    ("SLIDE-UNLOAD", "Weight, an uplift at the shoe and a -X lateral load at B: the uplift and the lateral load both unload the shoe, which slides -X on a normal far below the weight.",
     [("B", "FX", -3000), ("S", "FZ", -10000), ("S", "FZ", 6000)]),
    ("LIFT-0", "Net uplift at the shoe, no lateral load: lifted.",
     [("S", "FZ", 2000)]),
    ("LIFT-POS", "Net uplift and a +X lateral load at B that pushes S down but not onto its seat: lifted; the friction support carries nothing although the lateral load is large.",
     [("B", "FX", 3000), ("S", "FZ", 2000)]),
    ("LIFT-NEG", "Net uplift and a large -X lateral load at B: lifted further; the friction support carries nothing.",
     [("B", "FX", -20000), ("S", "FZ", 2000)]),
]


def loads_of(prims):
    s = {"Gx": Fr(0), "Px": Fr(0), "Pz": Fr(0)}
    for node, comp, val in prims:
        key = {("B", "FX"): "Gx", ("S", "FX"): "Px", ("S", "FZ"): "Pz"}[(node, comp)]
        s[key] += Fr(val)
    return s


def thresholds(inp, sec):
    """Exact stick-slide threshold of Gx (Pz = -10000, Px = 0, slide +X) and lift-off
    threshold of Pz (Gx = -8000, Px = 0), with dyadic brackets of step 1/256 N."""
    fxx, fxz, fzz = a_flex(inp, sec)
    mu = inp["mu"]

    def stick(G):
        return a_state_forces(inp, sec, {"Gx": G, "Px": Fr(0), "Pz": Fr(-10000)}, "active", "stick")
    Rx0, Rz0 = stick(Fr(0))
    Rx1, Rz1 = stick(Fr(1))
    cx, cz = Rx1 - Rx0, Rz1 - Rz0
    G_star = -(mu * Rz0 + Rx0) / (mu * cz + cx)        # mu*Rz = -Rx (Rx < 0 on this branch)
    u = a_displacements(inp, sec, Fr(0), Fr(1), Fr(-8000), Fr(0))["S"]["UZ"]
    u0 = a_displacements(inp, sec, Fr(0), Fr(0), Fr(-8000), Fr(0))["S"]["UZ"]
    Pz_star = -u0 / (u - u0)                            # free u_z(S) = 0
    step = Fr(1, 256)

    def bracket(x):
        lo = Fr((x / step).__floor__()) * step
        return lo, lo + step
    return G_star, bracket(G_star), Pz_star, bracket(Pz_star)


def build():
    chk = Checks()
    inp = dict(INPUTS)
    sec = section(inp)
    mdl = b_model(inp, sec)
    mu = inp["mu"]
    fxx, fxz, fzz = a_flex(inp, sec)

    # Flexibility closed forms against unit loads through formulation B (stiffness inverse at S)
    for (Fx, Fz), (ex, ez) in (((1, 0), (fxx, fxz)), ((0, 1), (fxz, fzz))):
        u, _, _, _ = b_solve_state(inp, mdl, {"Gx": Fr(0), "Px": Fr(Fx), "Pz": Fr(Fz)}, "inactive", "inactive")
        chk.eq("flexibility closed form = stiffness inverse", f"unit ({Fx},{Fz})", (u[mdl["dof"][("S", "UX")]], u[mdl["dof"][("S", "UZ")]]), (ex, ez))
    chk.true("well-posed slide (f_zz - d*mu*f_xz > 0 both ways)", "slide", fzz - mu * fxz > 0 and fzz + mu * fxz > 0)

    G_star, (G_lo, G_hi), Pz_star, (P_lo, P_hi) = thresholds(inp, sec)
    cases = list(BASE_CASES) + [
        ("NEAR-STICK", f"Stick-slide pair, below: Gx = {G_lo} N, under 1/256 N below the exact threshold Gx* = {G_star} N (Pz = -10000 N).",
         [("B", "FX", G_lo), ("S", "FZ", -10000)]),
        ("NEAR-SLIDE", f"Stick-slide pair, above: Gx = {G_hi} N, under 1/256 N above the threshold.",
         [("B", "FX", G_hi), ("S", "FZ", -10000)]),
        ("NEAR-BEAR", f"Lift-off pair, bearing side: Pz = {P_lo} N, Gx = -8000 N; the exact threshold is Pz* = {Pz_star} N and this is under 1/256 N more downward.",
         [("B", "FX", -8000), ("S", "FZ", P_lo)]),
        ("NEAR-LIFT", f"Lift-off pair, lifted side: Pz = {P_hi} N, Gx = -8000 N, under 1/256 N less downward than the threshold.",
         [("B", "FX", -8000), ("S", "FZ", P_hi)]),
    ]

    out_cases = []
    for cid, desc, prims in cases:
        loads = loads_of(prims)
        # ---- A: closed form, all eight combinations
        tries = a_solve(inp, sec, loads)
        admissible = [t for t in tries if not t["violations"]]
        chk.eq("A: exactly one admissible state pair", cid, len(admissible), 1)
        stA = admissible[0]["state"]
        RxA, RzA = admissible[0]["Rx"], admissible[0]["Rz"]
        dispA, reacA, endsA, axialA = a_results(inp, sec, loads, RxA, RzA)
        # ---- B: enumeration and active-set iteration from two starts
        okB = []
        for ow in STATES_ONEWAY:
            for fs in STATES_FRICTION:
                u, Rx, Rz, f = b_solve_state(inp, mdl, loads, ow, fs)
                d = mdl["dof"]
                if not law_violations(mu, ow, fs, Rx, Rz, u[d[("S", "UX")]], u[d[("S", "UZ")]]):
                    okB.append((ow, fs))
        chk.eq("B: enumeration finds exactly A's state", cid, okB, [stA])
        paths = {}
        for start in (("active", "stick"), ("inactive", "inactive")):
            stB, u, Rx, Rz, f, path = b_iterate(inp, mdl, loads, start)
            chk.eq("B: active-set iteration settles on A's state", f"{cid} from {start}", stB, stA)
            paths["/".join(start)] = path
        dispB, reacB, endsB = b_results(inp, mdl, u, Rx, Rz, f)
        chk.eq("A = B: displacements (pi*u)", cid, dispA, dispB)
        chk.eq("A = B: reactions", cid, reacA, reacB)
        chk.eq("A = B: member end actions", cid, endsA, endsB)
        # ---- equilibrium
        h, a = inp["h"], inp["a"]
        sx = loads["Gx"] + loads["Px"] + reacA["anchor A"]["FX"] + RxA
        sz = loads["Pz"] + reacA["anchor A"]["FZ"] + RzA
        my = reacA["anchor A"]["MY"] + h * (loads["Gx"] + loads["Px"] + RxA) - a * (loads["Pz"] + RzA)
        chk.eq("global equilibrium", cid, (sx, sz, my), (0, 0, 0))
        for m, (n1, n2) in mdl["members"].items():
            e1, e2 = endsA[m][n1], endsA[m][n2]
            (x1, z1), (x2, z2) = {"A": (0, 0), "B": (0, h), "S": (a, h)}[n1], {"A": (0, 0), "B": (0, h), "S": (a, h)}[n2]
            mom = e1["MY"] + e2["MY"] + (z2 - z1) * e2["FX"] - (x2 - x1) * e2["FZ"]
            chk.eq("member equilibrium", f"{cid} {m}", (e1["FX"] + e2["FX"], e1["FZ"] + e2["FZ"], mom), (0, 0, 0))
        nb = {c: endsA["riser"]["B"][c] + endsA["run"]["B"][c] for c in ("FX", "FZ", "MY")}
        chk.eq("node B balance (member end actions = applied load)", cid, nb, {"FX": loads["Gx"], "FZ": Fr(0), "MY": Fr(0)})
        # ---- state-law deciding quantities
        ow, fs = stA
        N = RzA if ow == "active" else Fr(0)
        dq = {"normal N (one-way reaction)": num(N, "N"),
              "tangential T (friction reaction)": num(RxA, "N"),
              "mu*N": num(mu * N, "N")}
        if ow == "active":
            dq["one-way contact reaction (> 0)"] = num(RzA, "N")
        else:
            dq["one-way separation u_z(S) (> 0)"] = num_over_pi(dispA["S"]["UZ"], "m")
        if fs == "stick":
            dq["stick margin mu*N - |T| (> 0)"] = num(mu * N - abs(RxA), "N")
        if fs.startswith("slide"):
            dq["slip u_x(S), sign = slide direction"] = num_over_pi(dispA["S"]["UX"], "m")
            chk.eq("sliding force = -d*mu*N", cid, RxA, -dirn(fs) * mu * N)
        # ---- robustness: binary64 inputs, and EI, EA perturbed by 1e-9 independently
        for key in INPUTS:
            inp2 = dict(inp); inp2[key] = Fr(float(inp[key]))
            sec2 = section(inp2)
            loads2 = {k: Fr(float(v)) for k, v in loads.items()}
            st2 = [t["state"] for t in a_solve(inp2, sec2, loads2) if not t["violations"]]
            chk.eq("state unchanged with each input replaced by its binary64 value", f"{cid} {key}", st2, [stA])
        for sa in (Fr(1) - Fr(1, 10 ** 9), Fr(1) + Fr(1, 10 ** 9)):
            for sb in (Fr(1) - Fr(1, 10 ** 9), Fr(1) + Fr(1, 10 ** 9)):
                sec2 = dict(sec); sec2["EAh"] = sec["EAh"] * sa; sec2["EIh"] = sec["EIh"] * sb
                st2 = [t["state"] for t in a_solve(inp, sec2, loads) if not t["violations"]]
                chk.eq("state unchanged with EA, EI each perturbed by 1e-9 relative", cid, st2, [stA])
        # ---- reactions independent of pi: B with pi-scaled stiffness 3 and 4 times
        for scale in (Fr(3), Fr(4)):
            sec3 = dict(sec); sec3["EAh"] = sec["EAh"] * scale; sec3["EIh"] = sec["EIh"] * scale
            mdl3 = b_model(inp, sec3)
            u3, Rx3, Rz3, _ = b_solve_state(inp, mdl3, loads, *stA)
            chk.eq("reactions do not depend on pi (stiffness scaled)", f"{cid} x{scale}", (Rx3, Rz3), (RxA, RzA))
            chk.eq("displacements scale as 1/pi", f"{cid} x{scale}", [x * scale for x in u3], u)

        # ---- normal source: lagged iteration and negative controls (sliding cases)
        lag = None
        controls = []
        if fs.startswith("slide"):
            d = dirn(fs)
            rho = d * mu * fxz / fzz
            Nfree = a_state_forces(inp, sec, loads, "active", "inactive")[1]
            chk.true("lagged-normal iteration contracts (|rho| < 1)", cid, abs(rho) < 1)
            chk.eq("fixed point N* = N_free/(1 - rho)", cid, Nfree / (1 - rho), N)
            Nk = Nfree
            seq = []
            for k in range(1, 6):
                Nk = Nfree + rho * Nk
                seq.append(Nk)
                chk.eq("lagged iterate N_k - N* = rho^k (N_0 - N*)", f"{cid} k={k}", Nk - N, rho ** k * (Nfree - N))
            lag = {"rho = d*mu*f_xz/f_zz": num(rho, "1"),
                   "N_0: one-way reaction with the friction force zero (start of the lagged sequence)": num(Nfree, "N"),
                   "N_1 .. N_5 (normal of iterate k, friction applied from N_(k-1))": [num(x, "N") for x in seq],
                   "N* (converged, the reference)": num(N, "N")}
            # Controls: friction force formed from a wrong normal held fixed
            weight = -sum(Fr(v) for n, c, v in prims if (n, c) == ("S", "FZ") and v < 0)
            srcs = [("CTRL-FREE-NORMAL", "normal taken from a solve with the friction force zero (N_0 above), not updated", Nfree)]
            if not cid.startswith("NEAR"):
                Nstick = a_state_forces(inp, sec, loads, "active", "stick")[1]
                srcs.append(("CTRL-STICK-NORMAL", "normal taken from the stick iterate that precedes the switch to slide, not updated", Nstick))
            if cid in ("SLIDE-UNLOAD",):
                srcs.append(("CTRL-WEIGHT-NORMAL", "normal carried over from the weight alone (10000 N, the shoe's reaction under the weight-only load), ignoring the uplift and the lateral load", weight))
            for cname, cdesc, Nw in srcs:
                # with N fixed at Nw: stick if the stick demand is within mu*Nw, else slide with force -d*mu*Nw
                Rxs, Rzs = a_state_forces(inp, sec, loads, "active", "stick")
                if abs(Rxs) < mu * Nw:
                    cst, Rxc, Rzc = "stick", Rxs, Rzs
                else:
                    u0 = a_displacements(inp, sec, loads["Px"], loads["Pz"], loads["Gx"], Fr(0))["S"]
                    dd = 1 if Rxs < 0 else -1
                    Rxc = -dd * mu * Nw
                    Rzc = -(u0["UZ"] + fxz * Rxc) / fzz
                    cst = "slide+" if dd == 1 else "slide-"
                dc, rc, _, _ = a_results(inp, sec, loads, Rxc, Rzc)
                differs = (cst != fs) or (Rxc != RxA) or (dc["S"]["UX"] != dispA["S"]["UX"])
                chk.true("negative control differs from the reference", f"{cid} {cname}", differs)
                controls.append({"id": cname, "label": "NEGATIVE CONTROL: a wrong answer", "normal used": cdesc,
                                 "N used": num(Nw, "N"), "friction state": cst,
                                 "friction force FX": num(Rxc, "N"),
                                 "one-way reaction FZ": num(Rzc, "N"),
                                 "u_x(S)": num_over_pi(dc["S"]["UX"], "m"),
                                 "u_x(B)": num_over_pi(dc["B"]["UX"], "m"),
                                 "relative difference of friction force from the reference": num((Rxc - RxA) / RxA, "1"),
                                 "relative difference of u_x(S) from the reference": num((dc["S"]["UX"] - dispA["S"]["UX"]) / dispA["S"]["UX"], "1")})

        enum = [{"state": {"one-way": t["state"][0], "friction": t["state"][1]},
                 "admissible": not t["violations"], "violations": t["violations"]} for t in tries]
        rec = {
            "id": cid,
            "description": desc,
            "primitive_loads": [{"node": n, "component": c, "value": num(Fr(v), "N")} for n, c, v in prims],
            "load sums": {"Gx at B": num(loads["Gx"], "N"), "Px at S": num(loads["Px"], "N"), "Pz at S": num(loads["Pz"], "N")},
            "states": {"one-way S": ow, "friction S": fs},
            "deciding_quantities": dq,
            "displacements": {n: {c: num_over_pi(dispA[n][c], "rad" if c == "RY" else "m") for c in ("UX", "UZ", "RY")} for n in ("B", "S")},
            "reactions": {s: {c: num(v, "N*m" if c == "MY" else "N") for c, v in r.items()} for s, r in reacA.items()},
            "member_end_actions": {m: {e: {c: num(v, "N*m" if c == "MY" else "N") for c, v in ev.items()} for e, ev in mv.items()} for m, mv in endsA.items()},
            "member_axial_force_tension_positive": {m: num(v, "N") for m, v in axialA.items()},
            "enumeration": enum,
            "active_set_paths": paths,
        }
        if lag:
            rec["lagged_normal"] = lag
        if controls:
            rec["negative_controls"] = controls
        out_cases.append(rec)

    # near-threshold pairs: the pair members straddle their thresholds
    st = {c["id"]: (c["states"]["one-way S"], c["states"]["friction S"]) for c in out_cases}
    chk.eq("stick-slide pair straddles", "NEAR-STICK/NEAR-SLIDE", (st["NEAR-STICK"], st["NEAR-SLIDE"]), (("active", "stick"), ("active", "slide+")))
    chk.eq("lift-off pair straddles", "NEAR-BEAR/NEAR-LIFT", (st["NEAR-BEAR"][0], st["NEAR-LIFT"]), ("active", ("inactive", "inactive")))
    chk.true("brackets contain thresholds", "pairs", G_lo < G_star < G_hi and P_lo < Pz_star < P_hi)

    by_kind = {}
    for k in chk.items:
        by_kind[k] = by_kind.get(k, 0) + 1
    out = {
        "schema": "swbpipe.validation_reference/friction_one_way_normal/1",
        "status": "frozen analytical reference; no product comparison, acceptance or release is asserted",
        "independence": "Python standard library only; no product code, product output, fixture or schema read, imported or run",
        "governing_text": ["execution/PKG-04/DEL-04-04/ScopeOfWork.md (friction law, state-switched test, state names, REQ-013)",
                           "execution/PKG-04/DEL-04-07/ScopeOfWork.md (nonlinear iteration: sliding force from the previous iterate's normal; CLM-002; AC-002)"],
        "model": {
            "plane": "X-Z, Z up; every out-of-plane displacement (UY, RX, RZ), reaction (FY, MX, MZ) and end action is exactly 0",
            "nodes": {"A": [0, 0, 0], "B": [0, 0, 3], "S": [2, 0, 3]},
            "members": {"riser": ["A", "B"], "run": ["B", "S"]},
            "supports": {
                "anchor A": "all six degrees of freedom",
                "one-way S": "UZ, sense + (bears upward only); ground at u_z = 0; no imposed movement",
                "friction S": "UX, coefficient mu; normal = the reaction of 'one-way S' (its normal reaction source)",
            },
            "section": {"D": num(INPUTS["D"], "m"), "t": num(INPUTS["t"], "m"),
                        "corrosion allowance": num(Fr(0), "m"), "mill tolerance": num(Fr(0), "1"),
                        "A": {"exact": f"({fr_text(sec['Ah'])})*pi", "unit": "m^2"},
                        "I": {"exact": f"({fr_text(sec['Ih'])})*pi", "unit": "m^4"},
                        "basis": "nominal and reduced bases coincide (zero allowance and tolerance), so the stiffness is the same on either"},
            "material": {"E": num(INPUTS["E"], "Pa"), "nu": num(INPUTS["nu"], "1")},
            "shear_model": "euler_bernoulli (no shear deformation)",
            "friction_coefficient": num(INPUTS["mu"], "1"),
            "pressure": "none: the models carry no pressure; the friction law does not depend on it",
            "self_weight": "none; every load is a nodal force listed per case",
        },
        "flexibility_at_S": {"f_xx": {"exact": f"({fr_text(fxx)})/pi", "unit": "m/N"},
                             "f_xz": {"exact": f"({fr_text(fxz)})/pi", "unit": "m/N"},
                             "f_zz": {"exact": f"({fr_text(fzz)})/pi", "unit": "m/N"}},
        "thresholds": {"stick-slide Gx* (Pz = -10000 N)": num(G_star, "N"),
                       "lift-off Pz* (Gx = -8000 N)": num(Pz_star, "N")},
        "conventions": {
            "reaction": "support-on-pipe force (K u - f) at the support's degree of freedom, global axes",
            "N": "the one-way support's reaction FZ while it is active; 0 while it is inactive",
            "states": "one-way: active | inactive; friction: inactive | stick | slide+ | slide- (slide sign = sign of u_x(S))",
            "end action": "force or moment the node exerts on the member at that end, global axes; MY about +Y (Z toward X positive)",
            "displacement exact form": "rational / pi; reactions and end actions are rational (pi cancels)",
            "value": "nearest binary64 of the exact value",
        },
        "tolerance_class": "DEL-04-07/AC-002 and DEL-04-04/AC-001: support states must equal these exactly; displacements and reactions agree within the nonlinear tolerance of their class (the governed per-class record, DEL-09-03/REQ-007). This package chooses no number.",
        "cases": out_cases,
        "checks": {"count": len(chk.items), "all_pass": True, "by_kind": dict(sorted(by_kind.items()))},
    }
    return out


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--write", action="store_true", help="rewrite references.json and SHA256SUMS")
    mode.add_argument("--check", action="store_true", help="compare frozen bytes (the default)")
    args = parser.parse_args()
    try:
        out = build()
    except CheckFailure as exc:
        print(f"check failed: {exc}", file=sys.stderr)
        return 1
    root = Path(__file__).resolve().parents[1]
    rendered = (json.dumps(out, indent=1, ensure_ascii=False) + "\n").encode("utf-8")
    files = {
        "README.md": (root / "README.md").read_bytes(),
        "generators/friction_one_way_normal_reference.py": Path(__file__).read_bytes(),
        "references.json": rendered,
    }
    sums = "".join(f"{hashlib.sha256(d).hexdigest()}  {n}\n" for n, d in files.items()).encode("ascii")
    outputs = {"references.json": rendered, "SHA256SUMS": sums}
    if args.write:
        for name, data in outputs.items():
            (root / name).write_bytes(data)
    else:
        diff = [n for n, d in outputs.items() if not (root / n).is_file() or (root / n).read_bytes() != d]
        if diff:
            print("reference bytes differ: " + ", ".join(diff), file=sys.stderr)
            return 1
    print(f"{len(out['cases'])} cases, {out['checks']['count']} exact checks, frozen bytes {'written' if args.write else 'match'}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
