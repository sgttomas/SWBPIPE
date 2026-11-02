#!/usr/bin/env python3
"""RT-A3 increment 2a candidate references: a concentric reducer whose mapped
body member is its own pressure region, joined to its neighbours at implicit
pressure junctions (two or more regions ending at one node, every terminal
transfers_to_wall).

Frozen analytical candidate, based on governing source
2656dcb33328b12c5572a30c10124a40f35adffc. Product agreement and independent
scrutiny are separate checks; no acceptance or release is asserted.

Independence. Standard library only (fractions, decimal, json). It imports,
reads and runs no product code or product output. Governing text was read only
to fix the declared model:
  - R-4 (execution/PKG-04/DEL-04-07/Work/route-unification-r4.md) §00 C-3, C-4;
    §2.1 (constitutive law E*A*eps = N_w - 2*nu*p*A_i, caps); §3.1 (junction
    resultant -sum p*A_i,k*s_k; corrected T-3 free-capped-end meaning; ALLHELD is a separate contrast);
  - pressure-contract.md "Tees and pressure junctions" (D-1 implicit junction;
    unequal pressures admitted as a closure with a warning);
  - DEL-03-05 rigid-component-stiffness-scaling.md §2.1 (scale s multiplies the
    member stiffness; the Poisson pair 2*nu*p*A_i has no E and is unchanged;
    N_w = N_m + 2*nu*p*A_i unchanged), §2.4 and §2.7 (equivalent pipe of the
    mapped section; rows are reference-section rows, not body stresses);
  - DEL-05-03 section-bases-and-timoshenko.md §1 and §2.1 (D-5: nominal wall
    for stiffness and A_s; pressure bore r_i,r for caps and the Poisson pair;
    sigma_m = N_w/A_b per basis; Lame rows of each basis's own annulus).
No value is taken from any product output, frozen product reference, standard,
catalog or vendor source. All model data are invented.

Model (small displacement, linear elastic, collinear straight members on the
global x axis). Pressure loads are purely axial on a collinear chain, so every
transverse translation and every rotation is unloaded and exactly zero, and the
axial results are independent of the shear model (euler_bernoulli or
timoshenko_annulus_energy_v1). Only the axial problem is solved.

Two formulations, which share only the input data and the section formulas:
  A. closed form: effective-force equilibrium at each node (sum_k S_k s_k = 0,
     S = N_w - p*A_i), the free-terminal condition S = 0, and series
     compatibility between anchors; written in left/right chain terms with no
     reference to member from/to orientation.
  B. an assembled exact 1-D axial stiffness solve with each member's own
     direction cosine (+1 or -1 from its from/to orientation), the Poisson pair
     as held end actions, the caps as nodal loads, exact Gauss-Jordan elimination
     over Fractions, reactions R = K u - f at anchored DOFs.
Every published value must agree exactly, and the separate equilibrium,
compatibility and constitutive checks must all hold, or nothing is printed and
the exit status is 1.

Usage:  python3 -I -B generate.py            # prints JSON to stdout
        python3 -I -B generate.py --summary  # prints a short check summary
"""

from decimal import Decimal, getcontext
from fractions import Fraction as Fr
import json
import re
import sys
import argparse
import hashlib
from pathlib import Path

getcontext().prec = 120

SOURCE_REVISION = "2656dcb33328b12c5572a30c10124a40f35adffc"

# ---------------------------------------------------------------------------
# Pi: exact Machin/alternating-series bounds; Decimal is display only


def _atan_inv_bounds(n):
    """Exact alternating-series enclosure of atan(1/n)."""
    x = Fr(1, n)
    power = x
    total = Fr(0)
    k = 0
    while True:
        total += power / (2 * k + 1)
        next_power = -power * x * x
        next_term = next_power / (2 * k + 3)
        if abs(next_term) < Fr(1, 10 ** 115):
            return min(total, total + next_term), max(total, total + next_term)
        power = next_power
        k += 1


_a5, _a239 = _atan_inv_bounds(5), _atan_inv_bounds(239)
PI_LO = 16 * _a5[0] - 4 * _a239[1]
PI_HI = 16 * _a5[1] - 4 * _a239[0]
assert Fr(3) < PI_LO < PI_HI < Fr(22, 7)
assert PI_HI - PI_LO < Fr(1, 10 ** 110)
_mid = (PI_LO + PI_HI) / 2
PI_DEC = Decimal(_mid.numerator) / Decimal(_mid.denominator)


def value_of(q, k):
    """(exact string, 40-digit decimal, nearest binary64) of q*pi^k."""
    q = Fr(q)
    if k == 0:
        exact = str(q)
        dec = Decimal(q.numerator) / Decimal(q.denominator)
        f = float(q)
    else:
        pk_lo, pk_hi = (PI_LO ** k, PI_HI ** k) if k > 0 else (PI_HI ** k, PI_LO ** k)
        lo, hi = sorted([q * pk_lo, q * pk_hi])
        f_lo, f_hi = float(lo), float(hi)
        if f_lo != f_hi:
            raise SystemExit(f"binary64 rounding undecided for {q}*pi^{k}")
        f = f_lo
        dec = (Decimal(q.numerator) / Decimal(q.denominator)) * (PI_DEC ** k)
        exact = f"{q}*pi" if k == 1 else (f"{q}/pi" if k == -1 else f"{q}*pi^{k}")
    if q == 0:
        exact = "0"
    return {"exact": exact, "decimal": f"{dec:.40g}" if q != 0 else "0", "value": f}


# ---------------------------------------------------------------------------
# Material and D-5 sections (exact rationals; areas stored as coefficients of pi)

E = Fr(200_000_000_000)  # Pa
NU = Fr(3, 10)


def section(name, D, t, cm, ca):
    D, t, cm, ca = Fr(D), Fr(t), Fr(cm), Fr(ca)
    ro = D / 2
    ri_n = ro - t
    t_r = (t - cm) - ca
    ri_r = ro - t_r
    return {
        "name": name, "D": D, "t": t, "cm": cm, "ca": ca, "ro": ro,
        "ri_n": ri_n, "t_r": t_r, "ri_r": ri_r,
        "An": ro * ro - ri_n * ri_n,  # nominal metal area / pi (stiffness, A_s)
        "Ar": ro * ro - ri_r * ri_r,  # reduced metal area / pi (reduced basis)
        "Ai": ri_r * ri_r,            # pressure bore area / pi (caps, Poisson pair)
    }


# Invented sections, nonzero mill tolerance and corrosion allowance so that the
# nominal and reduced bases differ and the pressure bore is not the nominal bore.
SEC = {
    "L": section("L", Fr(3, 10), Fr(12, 1000), Fr(1, 2000), Fr(1, 1000)),
    "S": section("S", Fr(1, 5), Fr(8, 1000), Fr(1, 2000), Fr(1, 1000)),
    "M": section("M", Fr(1, 4), Fr(10, 1000), Fr(1, 2000), Fr(1, 1000)),
}

P10 = Fr(10_000_000)  # 10 MPa


# ---------------------------------------------------------------------------
# Case construction


def case(label, purpose, nodes, members, anchors, notes=None, omit_caps=(), control=False):
    return {"label": label, "purpose": purpose, "nodes": nodes, "members": members,
            "anchors": anchors, "notes": notes or [], "omit_caps": set(omit_caps), "control": control}


def mem(mid, frm, to, sec, region, p, scale=Fr(1), role="pipe"):
    return {"id": mid, "from": frm, "to": to, "section": sec, "region": region,
            "p": Fr(p), "scale": Fr(scale), "role": role}


def reducing_chain(p_up=P10, p_body=P10, p_dn=P10, scale=Fr(1), body="L"):
    """Large upstream pipe P->N, reducer body N->F (own region), small pipe F->Q."""
    nodes = {"P": Fr(0), "N": Fr(4), "F": Fr(9, 2), "Q": Fr(15, 2)}
    members = [
        mem("pipe:up", "P", "N", "L", "region:up", p_up),
        mem("pipe:body", "N", "F", body, "region:body", p_body, scale, role="reducer_body"),
        mem("pipe:dn", "F", "Q", "S", "region:dn", p_dn),
    ]
    return nodes, members


def enlarging_reversed_chain(p=P10, scale=Fr(1)):
    """Enlarging along +x: small pipe, body, large pipe; every member's from/to
    reversed relative to +x (from is the right node)."""
    nodes = {"P": Fr(0), "N": Fr(3), "F": Fr(7, 2), "Q": Fr(15, 2)}
    members = [
        mem("pipe:up", "N", "P", "S", "region:up", p),
        mem("pipe:body", "F", "N", "L", "region:body", p, scale, role="reducer_body"),
        mem("pipe:dn", "Q", "F", "L", "region:dn", p),
    ]
    return nodes, members


def build_cases():
    out = []
    n, m = reducing_chain()
    out.append(case("R2A-FREE-N", "reducing; junction N (upstream pipe to body) anchored; far ends P, Q free and capped; F free (R-4 C-3)", n, m, ["N"]))
    n, m = reducing_chain(scale=Fr(2))
    out.append(case("R2A-FREE-N-S2", "as R2A-FREE-N with body stiffness scale 2 (DEC-045 equivalent reference pipe)", n, m, ["N"]))
    n, m = enlarging_reversed_chain()
    out.append(case("R2A-FREE-F-ENL-REV", "enlarging along +x, every member's from/to reversed; the other junction F (body to large pipe) anchored; far ends free and capped", n, m, ["F"]))
    n, m = reducing_chain()
    out.append(case("R2A-HELD", "reducing; far ends P and Q anchored; both junctions N and F free", n, m, ["P", "Q"]))
    n, m = reducing_chain(scale=Fr(2))
    out.append(case("R2A-HELD-S2", "as R2A-HELD with body stiffness scale 2 (indeterminate: redistributes)", n, m, ["P", "Q"]))
    n, m = enlarging_reversed_chain()
    out.append(case("R2A-HELD-ENL-REV", "enlarging along +x, every member's from/to reversed; far ends anchored, junctions free", n, m, ["P", "Q"]))
    n, m = reducing_chain(p_up=Fr(-100_000), p_body=Fr(-100_000), p_dn=Fr(-100_000))
    out.append(case("R2A-HELD-VAC", "as R2A-HELD with a signed (vacuum) differential p = -100 kPa in every region; collapse not assessed", n, m, ["P", "Q"]))
    n, m = reducing_chain(body="M")
    out.append(case("R2A-HELD-BODYM", "as R2A-HELD with the body on an intermediate section M (bore between L and S): the body bore only moves the fitting load between N and F", n, m, ["P", "Q"]))
    n, m = reducing_chain()
    out.append(case("R2A-ALLHELD", "every node anchored (P, N, F, Q): contrast to the free-capped-end T-3 case; step anchor carries -(1-2nu)*p*dA_i", n, m, ["P", "N", "F", "Q"]))
    n, m = reducing_chain()
    out.append(case("R2A-FREE-N-DROPCAP", "MUTATION CONTROL, not an admitted case: R2A-FREE-N with the body's cap at junction N omitted (a dropped junction cap must be detected)", n, m, ["N"],
                    ["the anchor then carries the omitted cap, -p*A_i,body along +x"], omit_caps=[("pipe:body", "N")], control=True))
    n, m = reducing_chain(p_dn=Fr(5_000_000))
    out.append(case("R2A-HELD-PDIFF", "CONTROL (admitted with EXACT_PRESSURE_JUNCTION_PRESSURES_DIFFER under D-1): downstream region at 5 MPa, a closure at junction F; far ends anchored", n, m, ["P", "Q"],
                    ["numbers are the closure's, by the same cap sums; not an equal-pressure reducer reference"]))
    return out


# ---------------------------------------------------------------------------
# Common geometry helpers (both formulations use these input-only facts)


def direction(c, mbr):
    return 1 if c["nodes"][mbr["to"]] > c["nodes"][mbr["from"]] else -1


def length(c, mbr):
    return abs(c["nodes"][mbr["to"]] - c["nodes"][mbr["from"]])


def s_into(c, mbr, node):
    """x-component of the unit vector from `node` into the member."""
    other = mbr["to"] if node == mbr["from"] else mbr["from"]
    return 1 if c["nodes"][other] > c["nodes"][node] else -1


def caps_at(c, node):
    """Sum of transfers_to_wall caps at a node (coefficient of pi): -sum p*A_i*s."""
    total = Fr(0)
    for mbr in c["members"]:
        if node in (mbr["from"], mbr["to"]) and (mbr["id"], node) not in c["omit_caps"]:
            total += -mbr["p"] * SEC[mbr["section"]]["Ai"] * s_into(c, mbr, node)
    return total


def axial_stiffness(c, mbr):  # coefficient of pi
    return mbr["scale"] * E * SEC[mbr["section"]]["An"] / length(c, mbr)


def poisson_n0(mbr):  # coefficient of pi
    return 2 * NU * mbr["p"] * SEC[mbr["section"]]["Ai"]


# ---------------------------------------------------------------------------
# Formulation A: closed form in left/right chain terms


def formulation_a(c):
    order = sorted(c["nodes"], key=lambda k: c["nodes"][k])
    # members in chain order (left node, right node)
    chain = []
    for left, right in zip(order, order[1:]):
        (mbr,) = [x for x in c["members"] if {x["from"], x["to"]} == {left, right}]
        chain.append((left, right, mbr))
    anchors = set(c["anchors"])

    def compliance(mbr):  # L/(sEA_n), coefficient 1/pi
        return length(c, mbr) / (mbr["scale"] * E * SEC[mbr["section"]]["An"])

    def free_mech(mbr):  # (1-2nu) p A_i: N_m of a member carrying S = 0
        return (1 - 2 * NU) * mbr["p"] * SEC[mbr["section"]]["Ai"]

    S = {}
    if anchors == set(order):
        for _, _, mbr in chain:
            S[mbr["id"]] = (2 * NU - 1) * mbr["p"] * SEC[mbr["section"]]["Ai"]  # N_m = 0
    elif len(anchors) == 1:
        for _, _, mbr in chain:
            S[mbr["id"]] = Fr(0)  # free terminals and S-balance at free nodes
    elif anchors == {order[0], order[-1]}:
        # S uniform between the anchors; sum of elongations is zero.
        num = sum(free_mech(mbr) * compliance(mbr) for _, _, mbr in chain)
        den = sum(compliance(mbr) for _, _, mbr in chain)
        s0 = -num / den
        for _, _, mbr in chain:
            S[mbr["id"]] = s0
    else:
        raise SystemExit(f"{c['label']}: no closed form for anchors {anchors}")

    res = {"u": {}, "members": {}, "reactions": {}}
    elong = {}
    for _, _, mbr in chain:
        sk = S[mbr["id"]]
        nm = sk + (1 - 2 * NU) * mbr["p"] * SEC[mbr["section"]]["Ai"]
        nw = sk + mbr["p"] * SEC[mbr["section"]]["Ai"]
        elong[mbr["id"]] = nm * compliance(mbr)  # rational (pi cancels)
        res["members"][mbr["id"]] = {"N_w": nw, "N_m": nm, "S": sk}
    # displacements: walk from the leftmost anchor
    start = next(k for k in order if k in anchors)
    u = {start: Fr(0)}
    i0 = order.index(start)
    for i in range(i0, len(order) - 1):
        left, right, mbr = chain[i]
        u[right] = u[left] + elong[mbr["id"]]
    for i in range(i0 - 1, -1, -1):
        left, right, mbr = chain[i]
        u[left] = u[right] - elong[mbr["id"]]
    res["u"] = u
    # reactions: R_n = -sum_k S_k s_k (node equilibrium with caps folded into S)
    for a in c["anchors"]:
        r = Fr(0)
        for _, _, mbr in chain:
            if a in (mbr["from"], mbr["to"]):
                r -= S[mbr["id"]] * s_into(c, mbr, a)
        res["reactions"][a] = r
    res["elongation"] = elong
    return res


# ---------------------------------------------------------------------------
# Formulation B: assembled exact axial stiffness with member orientation


def solve_exact(A, b):
    n = len(b)
    M = [row[:] + [b[i]] for i, row in enumerate(A)]
    for col in range(n):
        piv = next(r for r in range(col, n) if M[r][col] != 0)
        M[col], M[piv] = M[piv], M[col]
        pv = M[col][col]
        M[col] = [x / pv for x in M[col]]
        for r in range(n):
            if r != col and M[r][col] != 0:
                fac = M[r][col]
                M[r] = [x - fac * y for x, y in zip(M[r], M[col])]
    return [M[i][n] for i in range(n)]


def formulation_b(c):
    names = sorted(c["nodes"], key=lambda k: c["nodes"][k])
    idx = {k: i for i, k in enumerate(names)}
    n = len(names)
    K = [[Fr(0)] * n for _ in range(n)]
    f = [Fr(0)] * n
    for mbr in c["members"]:
        i, j = idx[mbr["from"]], idx[mbr["to"]]
        cdir = direction(c, mbr)
        k = axial_stiffness(c, mbr)
        # local (u_i, u_j) = cdir * (U_i, U_j); c^2 = 1
        K[i][i] += k; K[j][j] += k; K[i][j] -= k; K[j][i] -= k
        n0 = poisson_n0(mbr)
        # minus held end actions: +N0 at i, -N0 at j along local x
        f[i] += cdir * n0
        f[j] -= cdir * n0
        # caps: at a transfers_to_wall terminal, -p*A_i*s (s into the member)
        ai = SEC[mbr["section"]]["Ai"]
        if (mbr["id"], mbr["from"]) not in c["omit_caps"]:
            f[i] += -mbr["p"] * ai * cdir        # s at from = +local x
        if (mbr["id"], mbr["to"]) not in c["omit_caps"]:
            f[j] += -mbr["p"] * ai * (-cdir)     # s at to = -local x
    fixed = {idx[a] for a in c["anchors"]}
    free = [i for i in range(n) if i not in fixed]
    U = [Fr(0)] * n
    if free:
        sol = solve_exact([[K[r][cc] for cc in free] for r in free], [f[r] for r in free])
        for i, v in zip(free, sol):
            U[i] = v
    Ku = [sum(K[r][cc] * U[cc] for cc in range(n)) for r in range(n)]
    resid = [Ku[r] - f[r] for r in range(n)]
    res = {"u": {names[i]: U[i] for i in range(n)},
           "reactions": {names[i]: resid[i] for i in sorted(fixed)},
           "free_residual": [resid[i] for i in free],
           "members": {}, "elongation": {}}
    for mbr in c["members"]:
        i, j = idx[mbr["from"]], idx[mbr["to"]]
        cdir = direction(c, mbr)
        du_local = cdir * (U[j] - U[i])
        nm = axial_stiffness(c, mbr) * du_local
        nw = nm + poisson_n0(mbr)
        res["members"][mbr["id"]] = {"N_w": nw, "N_m": nm, "S": nw - mbr["p"] * SEC[mbr["section"]]["Ai"],
                                     # end actions on the member, local x: -N_w at from, +N_w at to
                                     "end_from": -nw, "end_to": nw}
        res["elongation"][mbr["id"]] = du_local
    return res


# ---------------------------------------------------------------------------
# Stress rows (D-5) per member and basis; rationals (pi cancels)


def stresses(mbr, nw):
    sec = SEC[mbr["section"]]
    p = mbr["p"]
    out = {}
    for basis, area, ri in (("nominal_section", sec["An"], sec["ri_n"]),
                            ("reduced_section", sec["Ar"], sec["ri_r"])):
        out[basis] = {"axial_membrane_sigma_m": nw / area}
    return out


def lame_rows(sec_name, p):
    sec = SEC[sec_name]
    out = {}
    for basis, area, ri in (("nominal_section", sec["An"], sec["ri_n"]),
                            ("reduced_section", sec["Ar"], sec["ri_r"])):
        hoop_outer = 2 * p * ri * ri / area
        out[basis] = {k: value_of(v, 0) for k, v in (
            ("hoop_outer", hoop_outer), ("hoop_inner", hoop_outer + p),
            ("radial_inner", -p), ("radial_outer", Fr(0)))}
    return out


# ---------------------------------------------------------------------------
# Checks


class Checks:
    def __init__(self):
        self.items = []

    def eq(self, name, a, b):
        ok = (a == b)
        self.items.append({"check": name, "pass": ok})
        if not ok:
            self.items[-1]["detail"] = f"{a} != {b}"
        return ok

    def true(self, name, cond, detail=""):
        self.items.append({"check": name, "pass": bool(cond)})
        if not cond:
            self.items[-1]["detail"] = detail
        return bool(cond)


def run_case(c, chk):
    b = formulation_b(c)
    L = c["label"]
    if c["control"]:
        a = None
    else:
        a = formulation_a(c)
    for k in (c["nodes"] if a else []):
        chk.eq(f"{L}: u[{k}] A==B", a["u"][k], b["u"][k])
    for k in (c["anchors"] if a else []):
        chk.eq(f"{L}: R[{k}] A==B", a["reactions"][k], b["reactions"][k])
    for mbr in c["members"]:
        for q in (("N_w", "N_m", "S") if a else ()):
            chk.eq(f"{L}: {mbr['id']} {q} A==B", a["members"][mbr["id"]][q], b["members"][mbr["id"]][q])
        if a:
            chk.eq(f"{L}: {mbr['id']} elongation A==B", a["elongation"][mbr["id"]], b["elongation"][mbr["id"]])
        # constitutive: s E A_n eps = N_w - 2 nu p A_i
        lhs = axial_stiffness(c, mbr) * b["elongation"][mbr["id"]]
        chk.eq(f"{L}: {mbr['id']} constitutive", lhs, b["members"][mbr["id"]]["N_w"] - poisson_n0(mbr))
    chk.true(f"{L}: B free-DOF residual zero", all(r == 0 for r in b["free_residual"]))
    # separate nodal equilibrium with wall forces, caps and reactions
    for k in c["nodes"]:
        tot = caps_at(c, k) + b["reactions"].get(k, Fr(0))
        for mbr in c["members"]:
            if k in (mbr["from"], mbr["to"]):
                tot += b["members"][mbr["id"]]["N_w"] * s_into(c, mbr, k)
        chk.eq(f"{L}: node {k} equilibrium (N_w s + caps + R)", tot, Fr(0))
    total_caps = sum(caps_at(c, k) for k in c["nodes"])
    total_r = sum(b["reactions"].values(), Fr(0))
    chk.eq(f"{L}: global balance sum R + sum caps", total_r + total_caps, Fr(0))
    if not c["control"]:
        chk.eq(f"{L}: each single-member closed region self-equilibrated (sum caps 0)", total_caps, Fr(0))
    for a_ in c["anchors"]:
        if a_ not in c["nodes"]:
            raise SystemExit("bad anchor")
    for k in c["anchors"]:
        chk.eq(f"{L}: anchor {k} displacement zero", b["u"][k], Fr(0))
    return a, b


def junction_nodes(c):
    out = []
    for k in c["nodes"]:
        regions = {m["region"] for m in c["members"] if k in (m["from"], m["to"])}
        if len(regions) >= 2:
            out.append(k)
    return out


def publish(c, b):
    sec_used = sorted({m["section"] for m in c["members"]})
    members = []
    for mbr in c["members"]:
        r = b["members"][mbr["id"]]
        st = stresses(mbr, r["N_w"])
        entry = {
            "id": mbr["id"], "from": mbr["from"], "to": mbr["to"], "section": mbr["section"],
            "region": mbr["region"], "role": mbr["role"],
            "pressure_Pa": str(mbr["p"]), "stiffness_scale": str(mbr["scale"]),
            "length_m": str(length(c, mbr)),
            "wall_force_N_w_N": value_of(r["N_w"], 1),
            "mechanical_force_N_m_N": value_of(r["N_m"], 1),
            "effective_force_S_N": value_of(r["S"], 1),
            "end_action_axial_local_from_N": value_of(r["end_from"], 1),
            "end_action_axial_local_to_N": value_of(r["end_to"], 1),
            "elongation_m": value_of(b["elongation"][mbr["id"]], 0),
            "axial_membrane_stress_Pa": {basis: {k: value_of(v, 0) for k, v in rows.items()} for basis, rows in st.items()},
        }
        if mbr["role"] == "reducer_body":
            entry["note"] = ("equivalent mapped-section (reference pipe) rows under the body's forces, "
                             "not a reducer body or taper stress (DEL-03-05 §2.4, component_body)")
        members.append(entry)
    juncs = junction_nodes(c)
    return {
        "label": c["label"],
        "purpose": c["purpose"],
        "notes": c["notes"],
        "admitted_case": not c["control"],
        "inputs": {
            "nodes_x_m": {k: str(v) for k, v in sorted(c["nodes"].items(), key=lambda kv: kv[1])},
            "anchors_all_six_dof": c["anchors"],
            "sections_used": sec_used,
            "terminals": "every region is one member; both its end nodes are terminals, transfers_to_wall",
            "junction_nodes": juncs,
        },
        "results": {
            "axial_displacement_x_m": {k: value_of(b["u"][k], 0) for k in sorted(c["nodes"], key=lambda k: c["nodes"][k])},
            "transverse_and_rotations": "exactly 0 at every node (unloaded collinear chain)",
            "reaction_x_N": {k: value_of(v, 1) for k, v in b["reactions"].items()},
            "reaction_other_components": "exactly 0",
            "total_reaction_x_N": value_of(sum(b["reactions"].values(), Fr(0)), 1),
            "junction_cap_sum_x_N": {k: value_of(caps_at(c, k), 1) for k in juncs},
            "fitting_resultant_x_N": value_of(sum(caps_at(c, k) for k in juncs), 1),
            "members": members,
        },
    }


def _check_kinds(items):
    kinds = {}
    for i in items:
        name = i["check"].split(": ", 1)[-1]
        key = re.sub(r"\[[A-Z]\]|pipe:\w+|node [A-Z]\b|anchor [A-Z]\b", "*", name)
        kinds[key] = kinds.get(key, 0) + 1
    return dict(sorted(kinds.items()))


def build_reference():
    chk = Checks()
    cases = build_cases()
    solved = {}
    for c in cases:
        a, b = run_case(c, chk)
        solved[c["label"]] = (c, a, b)

    def m(label, mid):
        return solved[label][2]["members"][mid]

    def u(label, node):
        return solved[label][2]["u"][node]

    def R(label, node):
        return solved[label][2]["reactions"][node]

    AiL, AiS, AiM = SEC["L"]["Ai"], SEC["S"]["Ai"], SEC["M"]["Ai"]
    p = P10
    # C-3: free capped far ends, junction anchored -> anchor reaction exactly 0
    for lab, node in (("R2A-FREE-N", "N"), ("R2A-FREE-N-S2", "N"), ("R2A-FREE-F-ENL-REV", "F")):
        chk.eq(f"{lab}: C-3 junction anchor reaction exactly 0", R(lab, node), Fr(0))
        for mid in ("pipe:up", "pipe:body", "pipe:dn"):
            mbr = next(x for x in solved[lab][0]["members"] if x["id"] == mid)
            chk.eq(f"{lab}: {mid} free closed leg N_w = p A_i", m(lab, mid)["N_w"], mbr["p"] * SEC[mbr["section"]]["Ai"])
    # fitting resultant p(A_i,large-side - A_i,small-side) along the step, s1=-x, s2=+x (reducing)
    for lab in ("R2A-FREE-N", "R2A-HELD", "R2A-HELD-S2", "R2A-HELD-BODYM", "R2A-ALLHELD"):
        c = solved[lab][0]
        chk.eq(f"{lab}: fitting resultant = p(A_i,L - A_i,S) (+x)", sum(caps_at(c, k) for k in junction_nodes(c)), p * (AiL - AiS))
    c = solved["R2A-FREE-F-ENL-REV"][0]
    chk.eq("R2A-FREE-F-ENL-REV: enlarging fitting resultant = p(A_i,S - A_i,L) (+x)", sum(caps_at(c, k) for k in junction_nodes(c)), p * (AiS - AiL))
    # body bore only redistributes the fitting load between N and F
    cH, cM = solved["R2A-HELD"][0], solved["R2A-HELD-BODYM"][0]
    chk.eq("R2A-HELD: body on L section: junction N carries 0", caps_at(cH, "N"), Fr(0))
    chk.eq("R2A-HELD-BODYM: junction N carries p(A_i,L - A_i,M)", caps_at(cM, "N"), p * (AiL - AiM))
    chk.eq("R2A-HELD-BODYM: junction F carries p(A_i,M - A_i,S)", caps_at(cM, "F"), p * (AiM - AiS))
    # scale-2 body: determinate forces unchanged, body free elongation halved
    for mid in ("pipe:up", "pipe:body", "pipe:dn"):
        chk.eq(f"S2 free: {mid} N_w unchanged by scale", m("R2A-FREE-N-S2", mid)["N_w"], m("R2A-FREE-N", mid)["N_w"])
    chk.eq("S2 free: body elongation halved", solved["R2A-FREE-N-S2"][2]["elongation"]["pipe:body"] * 2,
           solved["R2A-FREE-N"][2]["elongation"]["pipe:body"])
    chk.eq("S2 free: body elongation = (1-2nu) p A_i L/(2 E A_n)", solved["R2A-FREE-N-S2"][2]["elongation"]["pipe:body"],
           (1 - 2 * NU) * p * AiL * Fr(1, 2) / (2 * E * SEC["L"]["An"]))
    chk.true("S2 held: scale redistributes (S0 changes)", m("R2A-HELD-S2", "pipe:up")["S"] != m("R2A-HELD", "pipe:up")["S"])
    # held: effective force uniform, far reactions -S0 at P and +S0 at Q; total zero
    for lab in ("R2A-HELD", "R2A-HELD-S2", "R2A-HELD-ENL-REV", "R2A-HELD-VAC", "R2A-HELD-BODYM", "R2A-HELD-PDIFF"):
        s0 = m(lab, "pipe:up")["S"]
        chk.true(f"{lab}: effective force uniform along the chain", all(m(lab, x)["S"] == s0 for x in ("pipe:body", "pipe:dn")))
        chk.eq(f"{lab}: R_P = -S0", R(lab, "P"), -s0)
        chk.eq(f"{lab}: R_Q = +S0", R(lab, "Q"), s0)
        chk.eq(f"{lab}: far anchors compatible (sum of elongations 0)", sum(solved[lab][2]["elongation"].values(), Fr(0)), Fr(0))
    # vacuum: exact linearity, sign reversal
    for node in ("N", "F"):
        chk.eq(f"VAC: u[{node}] = -1/100 x HELD", u("R2A-HELD-VAC", node), u("R2A-HELD", node) * Fr(-1, 100))
    for node in ("P", "Q"):
        chk.eq(f"VAC: R[{node}] = -1/100 x HELD", R("R2A-HELD-VAC", node), R("R2A-HELD", node) * Fr(-1, 100))
    # Separate all-held contrast: every node held -> step anchors carry -(1-2nu) p dA_i
    chk.eq("ALLHELD: R_N = -(1-2nu) p (A_i,up - A_i,body) = 0 (body on L)", R("R2A-ALLHELD", "N"), -(1 - 2 * NU) * p * (AiL - AiL))
    chk.eq("ALLHELD: R_F = -(1-2nu) p (A_i,body - A_i,dn)", R("R2A-ALLHELD", "F"), -(1 - 2 * NU) * p * (AiL - AiS))
    for mid, sec in (("pipe:up", "L"), ("pipe:body", "L"), ("pipe:dn", "S")):
        chk.eq(f"ALLHELD: {mid} N_w = 2 nu p A_i", m("R2A-ALLHELD", mid)["N_w"], 2 * NU * p * SEC[sec]["Ai"])
    # dropped-cap mutation control: the anchor carries the omitted cap
    chk.eq("DROPCAP: R_N = omitted cap = -p A_i,body (+x)", R("R2A-FREE-N-DROPCAP", "N"), -p * AiL)
    chk.true("DROPCAP: differs from the admitted R2A-FREE-N (0)", R("R2A-FREE-N-DROPCAP", "N") != R("R2A-FREE-N", "N"))
    # unequal pressures (D-1 closure, warned): junction F load is p_body*A_i,body - p_dn*A_i,dn
    cP = solved["R2A-HELD-PDIFF"][0]
    chk.eq("PDIFF: junction F closure load = p_b A_i,L - p_dn A_i,S", caps_at(cP, "F"), p * AiL - Fr(5_000_000) * AiS)
    chk.true("PDIFF: differs from equal-pressure R2A-HELD", R("R2A-HELD-PDIFF", "P") != R("R2A-HELD", "P"))
    # reversed orientation: enlarging reversed chain mirrors the reducing chain's physics
    chk.true("ENL-REV: end actions follow member orientation (-N_w at from, +N_w at to)",
             all(m("R2A-HELD-ENL-REV", x)["end_from"] == -m("R2A-HELD-ENL-REV", x)["N_w"] for x in ("pipe:up", "pipe:body", "pipe:dn")))

    if not all(i["pass"] for i in chk.items):
        bad = [i for i in chk.items if not i["pass"]]
        sys.stderr.write(json.dumps(bad[:20], indent=1) + "\n")
        sys.exit(1)

    out = {
        "reference_id": "rt_a3_reducer_2a",
        "status": "frozen analytical candidate; product agreement and independent scrutiny recorded separately",
        "source_revision_read": SOURCE_REVISION,
        "governing_sources": [
            "execution/PKG-04/DEL-04-07/Work/route-unification-r4.md §00 C-3, C-4; §2.1; §3.1",
            "execution/PKG-04/DEL-04-07/Work/pressure-contract.md 'Tees and pressure junctions' (D-1)",
            "execution/PKG-03/DEL-03-05/Work/rigid-component-stiffness-scaling.md §2.1, §2.4, §2.7",
            "execution/PKG-03/DEL-03-05/Work/eccentric-reducer-offset.md §2, §3.2 (absent vs zero offset; end sizes descriptive)",
            "execution/PKG-05/DEL-05-03/Work/section-bases-and-timoshenko.md §1, §2.1 (D-5)",
        ],
        "independence": "standard library only; no product code, product output or frozen product reference is imported, read or run; all data invented",
        "model": {
            "theory": "small-displacement linear elastic collinear straight members; axial problem only (transverse and rotational DOFs unloaded, exactly 0); shear-model independent",
            "constitutive": "s*E*A_n*eps = N_w - 2*nu*p*A_i; N_w = N_m + 2*nu*p*A_i; S = N_w - p*A_i (s = DEC-045 scale; Poisson pair has no E)",
            "caps": "-p*A_i*s at each transfers_to_wall terminal, s the unit vector from the node into the member; implicit junction = node where two or more regions end",
            "d5": "A_n (nominal wall) for stiffness; A_i = pi*r_i,r^2 (pressure bore) for caps and the Poisson pair; sigma_m = N_w/A_b per basis; Lame per basis's own annulus: radial -p inner, 0 outer; hoop outer 2p*r_i,b^2/(r_o^2-r_i,b^2), inner = outer + p",
            "end_sizes": "descriptive only; never enter a number (bores come from the member sections)",
            "material": {"E_Pa": str(E), "nu": str(NU)},
            "sections": {k: {"D_m": str(s["D"]), "t_m": str(s["t"]), "c_m_m": str(s["cm"]), "c_a_m": str(s["ca"]),
                             "r_o_m": str(s["ro"]), "r_i_nominal_m": str(s["ri_n"]), "t_reduced_m": str(s["t_r"]),
                             "r_i_pressure_bore_m": str(s["ri_r"]),
                             "A_nominal_m2": value_of(s["An"], 1), "A_reduced_m2": value_of(s["Ar"], 1),
                             "A_i_bore_m2": value_of(s["Ai"], 1)} for k, s in SEC.items()},
            "formulations": {
                "A": "closed form: sum S_k s_k = 0 at free nodes, S = 0 beyond free capped terminals, series compatibility between far anchors; left/right chain terms, no member orientation",
                "B": "assembled exact 1-D axial stiffness with member direction cosines, held Poisson end actions, caps as nodal loads, Gauss-Jordan over Fractions, R = K u - f",
            },
            "exact_form": "q*pi^k with q rational; forces k=1, displacements and stresses k=0; 'value' is the nearest binary64, decided by rational pi bounds 2e-100 apart",
            "units": "m, N, Pa",
        },
        "lame_rows_Pa": {f"{s}@{p_}Pa": lame_rows(s, Fr(p_)) for s, p_ in sorted({(m_["section"], str(m_["p"])) for (c_, a_, b_) in solved.values() for m_ in c_["members"]})},
        "cases": [publish(c, b) for (c, a, b) in solved.values()],
        "checks": {"count": len(chk.items), "all_pass": True,
                   "by_kind": _check_kinds(chk.items)},
    }
    return out


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--write", action="store_true", help="rewrite frozen outputs intentionally")
    mode.add_argument("--check", action="store_true", help="check frozen bytes (the default)")
    mode.add_argument("--summary", action="store_true", help="derive and show the check count only")
    args = parser.parse_args()
    out = build_reference()
    if args.summary:
        print(f"{len(out['cases'])} cases, {out['checks']['count']} exact checks, all pass")
        return
    root = Path(__file__).resolve().parents[1]
    rendered = (json.dumps(out, indent=1, ensure_ascii=False) + "\n").encode("utf-8")
    files = {
        "README.md": (root / "README.md").read_bytes(),
        "generators/rt_a3_reducer_reference.py": Path(__file__).read_bytes(),
        "references.json": rendered,
    }
    sums = "".join(f"{hashlib.sha256(data).hexdigest()}  {name}\n" for name, data in files.items()).encode("ascii")
    outputs = {"references.json": rendered, "SHA256SUMS": sums}
    if args.write:
        for name, data in outputs.items():
            (root / name).write_bytes(data)
    else:
        different = [name for name, data in outputs.items() if not (root / name).is_file() or (root / name).read_bytes() != data]
        if different:
            raise SystemExit("Reference bytes differ: " + ", ".join(different))
    print(f"{len(out['cases'])} cases, {out['checks']['count']} exact checks, frozen bytes {'written' if args.write else 'match'}")


if __name__ == "__main__":
    main()
