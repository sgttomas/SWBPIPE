#!/usr/bin/env python3
"""W-16 stage 2 independent references: imposed motion with nonlinear supports.

DEL-04-03 Design pf-nonlinear-imposed-motion.md section 5. Standard-library
Python 3 only. This generator imports, reads and runs no product code and
reads no file other than the ones it writes.

It writes each invented request document (requests/*.json), forms every
Euler-Bernoulli 12x12 frame element itself in exact rationals from the
request's binary64 leaves (Przemieniecki, Theory of Matrix Structural
Analysis; McGuire, Gallagher and Ziemian, Matrix Structural Analysis),
assembles, enumerates every nonlinear support state (each sliding support in
both slip senses, each derived normal in both signs) and solves each state
exactly with the case's prescribed values:

  R-a  a rigid anchor or restraint prescribed at its case value g_b;
  R-b  a nonlinear support's ground moved by g_s: its state is measured on
       delta_s = u_d - g_s, and an active or sticking state prescribes g_s
       (g_s + sigma*c for an active gap, sigma = +1 closing in +, -1 in -);
  R-d  a rigid restraint on a mixed nonlinear record (a nonlinear record that
       also restrains other DOFs rigidly) prescribed at its case value g_b,
       as under R-a.

It records every state that satisfies all DEC-067 conditions strictly, with
its exact published rows, and every state that satisfies them only weakly
(a zero margin). Each benchmark's closed form (section 5) is evaluated
separately, in exact rationals, and compared with the enumerated state.

The section leaves (A, I, J) are the normalized binary64 values the product
forms from the authored diameter and wall; the product-side consumer checks
them bit for bit. E = 5*2^35 Pa and G = 2^36 Pa are invented so that every
boundary case is an exact boundary of the system formed from those leaves:
no rounded analytical threshold is used.

Modes:
  python3 -I w16_nl_reference.py          rebuild every output in memory and
                                           compare byte for byte with the
                                           committed files (SHA256SUMS included)
  python3 -I w16_nl_reference.py --write  write the outputs (intentional changes only)
"""
import argparse
import hashlib
import json
import math
import sys
from fractions import Fraction as F
from itertools import product
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE.parent
FIXED_FILES = ("README.md", "generators/w16_nl_reference.py")
PROV = "invented_w16_nonlinear_motion_reference"
DOFS = ["UX", "UY", "UZ", "RX", "RY", "RZ"]

# ----------------------------------------------------------------------------- invented inputs
OD, WALL = 0.206, 0.011  # m, authored
# The normalized binary64 leaves the product forms from (OD, WALL) (checked bit
# for bit by the product-side consumer): A, I (= Iy = Iz) and J = 2I.
LEAVES = {"As_m2": 0.0067387162419501045, "I_m4": 3.213188372067858e-05, "J_m4": 6.426376744135716e-05}
E_PA = float(5 * 2 ** 35)  # 171.8 GPa, invented
G_PA = float(2 ** 36)  # 68.7 GPa, invented
A, I, J = (F(LEAVES[k]) for k in ("As_m2", "I_m4", "J_m4"))
E, G = F(E_PA), F(G_PA)
assert J == 2 * I


def length_cube(length):
    return F(length) ** 3


# ----------------------------------------------------------------------------- request documents
def qv(value, unit):
    return {"value": value, "unit": unit}


def node(node_id, x, y, z):
    return {"id": node_id, "position": {"x": x, "y": y, "z": z}, "provenance": PROV}


def pipe(pipe_id, a, b, yref):
    return {"id": pipe_id, "from": a, "to": b,
            "section": {"outside_diameter": qv(OD, "m"), "wall_thickness": qv(WALL, "m")},
            "material": "material:w16nl", "y_reference": {"x": yref[0], "y": yref[1], "z": yref[2]}, "provenance": PROV}


def anchor(support_id, node_id, restraints=("UX", "UY", "UZ", "RX", "RY", "RZ"), family="anchor"):
    return {"id": support_id, "node": node_id, "family": family, "restraints": list(restraints), "provenance": PROV}


def spring(support_id, node_id, dof, k):
    return {"id": support_id, "node": node_id, "family": "spring", "restraints": [dof],
            "stiffness": {"dof": dof, "value": qv(k, "N/m")}, "provenance": PROV}


def nonlinear(support_id, node_id, spec, restraints=()):
    """A nonlinear record; `restraints` are its rigid restraints on other DOFs (a mixed record, R-d)."""
    return {"id": support_id, "node": node_id, "family": "nonlinear", "restraints": list(restraints), "nonlinear": spec,
            "provenance": PROV}


def force(load_id, node_id, axis, value):
    return {"id": load_id, "category": "concentrated_force", "target": {"type": "node", "node": node_id},
            "direction": {"x": "global_x", "y": "global_y", "z": "global_z"}[axis],
            "magnitude": qv(value, "N"), "dimension": "force", "provenance": PROV}


def motion(load_id, support_id, dof, value):
    rotation = dof.startswith("R")
    return {"id": load_id, "category": "imposed_displacement", "target": {"type": "support", "support": support_id, "dof": dof},
            "direction": dof, "magnitude": qv(value, "rad" if rotation else "m"),
            "dimension": "rotation" if rotation else "displacement", "provenance": PROV}


def case(case_id, loads):
    return {"id": case_id, "primitive_loads": loads, "provenance": PROV}


def document(name, description, nodes, pipes, supports, cases):
    return {"model": {
        "schema_version": "0.8.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": f"project:w16nl-{name}", "name": f"W-16 stage 2 {name}", "description": description,
                    "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing",
                            "professional_acceptance": "not_provided"},
        "nodes": nodes, "pipe_segments": pipes, "supports": supports, "components": [],
        "materials": [{"id": "material:w16nl", "elastic_modulus": qv(E_PA, "Pa"), "shear_modulus": qv(G_PA, "Pa"),
                       "thermal_expansion_coefficient": qv(1.2e-05, "1/degC"), "provenance": PROV}],
        "load_cases": cases, "combinations": []}, "materials": []}


def as_float(q):
    """A binary64 input that must be exact."""
    f = float(q)
    assert F(f) == q, f"{q} is not a binary64 value"
    return f


# ----------------------------------------------------------------------------- exact frame
def exact_axes(xi, xj, yref):
    d = [b - a for a, b in zip(xi, xj)]
    nonzero = [k for k in range(3) if d[k] != 0]
    assert len(nonzero) == 1, "reference members are axis-aligned"
    length = abs(d[nonzero[0]])
    x = [v / length for v in d]
    proj = sum(a * b for a, b in zip(yref, x))
    yc = [a - proj * b for a, b in zip(yref, x)]
    ynz = [k for k in range(3) if yc[k] != 0]
    assert len(ynz) == 1, "reference plane references are axis-aligned"
    y = [v / abs(yc[ynz[0]]) for v in yc]
    z = [x[1] * y[2] - x[2] * y[1], x[2] * y[0] - x[0] * y[2], x[0] * y[1] - x[1] * y[0]]
    return [x, y, z], length


def local_stiffness(length, area, inertia, torsion, e, g):
    L = length
    k = [[F(0)] * 12 for _ in range(12)]
    axial, twist, ei = e * area / L, g * torsion / L, e * inertia
    v12, v6, v4, v2 = 12 * ei / L ** 3, 6 * ei / L ** 2, 4 * ei / L, 2 * ei / L
    for i, j, v in [(0, 6, axial), (3, 9, twist)]:
        k[i][i] = k[j][j] = v
        k[i][j] = k[j][i] = -v
    bz = [[v12, v6, -v12, v6], [v6, v4, -v6, v2], [-v12, -v6, v12, -v6], [v6, v2, -v6, v4]]
    by = [[v12, -v6, -v12, -v6], [-v6, v4, v6, v2], [-v12, v6, v12, v6], [-v6, v2, v6, v4]]
    for ids, block in [([1, 5, 7, 11], bz), ([2, 4, 8, 10], by)]:
        for i in range(4):
            for j in range(4):
                k[ids[i]][ids[j]] = block[i][j]
    return k


def to_global(k, R):
    T = [[F(0)] * 12 for _ in range(12)]
    for b in range(4):
        for i in range(3):
            for j in range(3):
                T[3 * b + i][3 * b + j] = R[i][j]
    KT = [[sum(k[i][m] * T[m][j] for m in range(12)) for j in range(12)] for i in range(12)]
    return [[sum(T[m][i] * KT[m][j] for m in range(12)) for j in range(12)] for i in range(12)], T


def solve(Amat, b):
    n = len(b)
    aug = [list(row) + [rhs] for row, rhs in zip(Amat, b)]
    for col in range(n):
        pivot = next((r for r in range(col, n) if aug[r][col] != 0), None)
        if pivot is None:
            return None
        aug[col], aug[pivot] = aug[pivot], aug[col]
        p = aug[col][col]
        aug[col] = [x / p for x in aug[col]]
        for row in range(n):
            if row != col and aug[row][col]:
                f = aug[row][col]
                aug[row] = [a - f * c for a, c in zip(aug[row], aug[col])]
    return [row[-1] for row in aug]


def bounds(x):
    """The binary64 neighbours (lo <= x <= hi), decided exactly."""
    f = float(x)
    lo = f if F(f) <= x else math.nextafter(f, -math.inf)
    hi = f if F(f) >= x else math.nextafter(f, math.inf)
    assert F(lo) <= x <= F(hi)
    return [lo, hi]


def sqrt_bounds(sq):
    """Binary64 bounds of sqrt(sq), established by exact squared comparisons."""
    if sq == 0:
        return [0.0, 0.0]
    f = math.sqrt(float(sq))
    lo, hi = f, f
    while F(lo) ** 2 > sq:
        lo = math.nextafter(lo, -math.inf)
    while F(hi) ** 2 < sq:
        hi = math.nextafter(hi, math.inf)
    assert F(lo) ** 2 <= sq <= F(hi) ** 2
    return [lo, hi]


class Model:
    """The exact system of one request, formed from its leaves alone."""

    def __init__(self, request, leaves=None, e=None, g=None):
        m = request["model"]
        self.nodes = [n["id"] for n in m["nodes"]]
        self.pos = {n["id"]: [F(n["position"][a]) for a in "xyz"] for n in m["nodes"]}
        self.n = 6 * len(self.nodes)
        self.K = [[F(0)] * self.n for _ in range(self.n)]
        self.members = []
        area, inertia, torsion = leaves or (A, I, J)
        self.e, self.g = (e or E), (g or G)
        for p in m["pipe_segments"]:
            assert (p["section"]["outside_diameter"]["value"], p["section"]["wall_thickness"]["value"]) == (OD, WALL)
            yref = [F(p["y_reference"][a]) for a in "xyz"]
            R, length = exact_axes(self.pos[p["from"]], self.pos[p["to"]], yref)
            kg, T = to_global(local_stiffness(length, area, inertia, torsion, self.e, self.g), R)
            i, j = self.nodes.index(p["from"]), self.nodes.index(p["to"])
            dofs = list(range(6 * i, 6 * i + 6)) + list(range(6 * j, 6 * j + 6))
            for a, ga in enumerate(dofs):
                for b, gb in enumerate(dofs):
                    self.K[ga][gb] += kg[a][b]
            self.members.append((p["id"], kg, T, length, dofs))
        self.rigid, self.springs, self.nonlinear, self.supports = {}, [], [], []
        self.support_by_id = {}
        for s in m["supports"]:
            nd = self.nodes.index(s["node"])
            self.supports.append((s["id"], nd))
            self.support_by_id[s["id"]] = s
            if s.get("nonlinear"):
                nl = s["nonlinear"]
                entry = {"id": s["id"], "dof": 6 * nd + DOFS.index(nl["dof"]), "behavior": nl["behavior"]}
                if nl["behavior"] in ("one_way", "lift_off"):
                    sense = nl["active_when"] if nl["behavior"] == "one_way" else nl["contact_when"]
                    entry["positive"] = sense == "positive_reaction"
                elif nl["behavior"] == "gap":
                    entry["sigma"] = 1 if nl["closes_when"] == "positive_displacement" else -1
                    entry["clearance"] = F(nl["gap"]["value"])
                elif nl["behavior"] == "friction":
                    entry["mu"] = F(nl["friction_coefficient"]["value"])
                    if "normal_reaction" in nl:
                        entry["normal"] = F(nl["normal_reaction"]["value"])
                    else:
                        src = nl["normal_reaction_source"]
                        src_node = self.nodes.index(next(x for x in m["supports"] if x["id"] == src["support_ref"])["node"])
                        entry["source"] = 6 * src_node + DOFS.index(src["dof"])
                self.nonlinear.append(entry)
                # R-d: a mixed record's rigid restraints, which never include its nonlinear DOF.
                assert s.get("family") == "nonlinear" and nl["dof"] not in s["restraints"]
                for r in s["restraints"]:
                    self.rigid.setdefault(s["id"], []).append(6 * nd + DOFS.index(r))
            elif s.get("family") == "spring":
                st = s["stiffness"]
                d = 6 * nd + DOFS.index(st["dof"])
                k = F(st["value"]["value"])
                self.springs.append((s["id"], d, k))
                self.K[d][d] += k
            else:
                for r in s["restraints"]:
                    self.rigid.setdefault(s["id"], []).append(6 * nd + DOFS.index(r))
        self.cases = m["load_cases"]

    def loads(self, case):
        """The case's force vector, base prescribed values g_B and ground offsets g_s."""
        f = [F(0)] * self.n
        base, ground = {}, {}
        for load in case["primitive_loads"]:
            if load["category"] == "imposed_displacement":
                target = load["target"]
                s = self.support_by_id[target["support"]]
                d = 6 * self.nodes.index(s["node"]) + DOFS.index(target["dof"])
                value = F(load["magnitude"]["value"])
                if s.get("nonlinear") and s["nonlinear"]["dof"] == target["dof"]:
                    ground[s["id"]] = value
                else:
                    assert d in self.rigid[s["id"]]
                    base[d] = value
                continue
            assert load["dimension"] == "force" and load["target"]["type"] == "node"
            nd = self.nodes.index(load["target"]["node"])
            axis = {"global_x": 0, "global_y": 1, "global_z": 2}[load["direction"]]
            f[6 * nd + axis] += F(load["magnitude"]["value"])
        return f, base, ground


def states_for(entry):
    if entry["behavior"] in ("one_way", "lift_off", "gap"):
        return [("active", None, None), ("inactive", None, None)]
    out = [("inactive", None, None), ("sticking", None, None)]
    for direction in (1, -1):
        for beta in ((1, -1) if "source" in entry and entry["mu"] != 0 else (None,)):
            out.append(("sliding", direction, beta))
    return out


def prescribed_value(entry, ground):
    g = ground.get(entry["id"], F(0))
    if entry["behavior"] == "gap":
        return g + entry["sigma"] * entry["clearance"]
    return g


def solve_state(model, f, base, ground, combo):
    n = model.n
    constrained = {d: base.get(d, F(0)) for dofs in model.rigid.values() for d in dofs}
    friction_rows = {}
    for entry, (state, direction, beta) in zip(model.nonlinear, combo):
        d = entry["dof"]
        if state in ("active", "sticking"):
            constrained[d] = prescribed_value(entry, ground)
        elif state == "sliding" and entry["mu"] != 0:
            friction_rows[d] = (entry, direction, beta)
    free = [i for i in range(n) if i not in constrained]
    Amat, b = [], []
    for i in free:
        row = [model.K[i][j] for j in free]
        rhs = f[i] - sum(model.K[i][j] * v for j, v in constrained.items())
        if i in friction_rows:
            entry, direction, beta = friction_rows[i]
            if "source" in entry:
                # F = -dir*mu*beta*(K_n u - f_n), moved to the left-hand side.
                c = -direction * entry["mu"] * beta
                src = entry["source"]
                row = [r - c * model.K[src][j] for r, j in zip(row, free)]
                rhs = rhs + c * (-f[src] + sum(model.K[src][j] * v for j, v in constrained.items()))
            else:
                rhs = rhs + (-direction * entry["mu"] * abs(entry["normal"]))
        Amat.append(row)
        b.append(rhs)
    x = solve(Amat, b)
    if x is None:
        return None
    u = [F(0)] * n
    for d, v in constrained.items():
        u[d] = v
    for i, v in zip(free, x):
        u[i] = v
    r = [sum(model.K[i][j] * u[j] for j in range(n)) - f[i] for i in range(n)]
    return u, r


def classify(model, combo, u, r, ground):
    """'strict', 'weak' or None for one state against every DEC-067 condition, on delta = u - g_s."""
    weak = False
    for entry, (state, direction, beta) in zip(model.nonlinear, combo):
        d = entry["dof"]
        delta = u[d] - ground.get(entry["id"], F(0))
        if entry["behavior"] in ("one_way", "lift_off"):
            s = 1 if entry["positive"] else -1
            value = s * r[d] if state == "active" else s * delta
        elif entry["behavior"] == "gap":
            # Closed: the support bears against further closing (sigma*r <= 0);
            # open: sigma*delta below the clearance.
            value = -entry["sigma"] * r[d] if state == "active" else entry["clearance"] - entry["sigma"] * delta
        else:
            value = None
        if value is not None:
            if value < 0:
                return None
            weak |= value == 0
            continue
        normal = abs(r[entry["source"]]) if "source" in entry else entry["normal"]
        if state == "inactive":
            if normal > 0:
                return None
            weak |= "source" in entry
        elif state == "sticking":
            margin = entry["mu"] * normal - abs(r[d])
            if normal <= 0 or margin < 0:
                return None
            weak |= margin == 0
        else:
            if normal <= 0 or direction * delta < 0:
                return None
            if "source" in entry and entry["mu"] != 0 and beta * r[entry["source"]] < 0:
                return None
            weak |= direction * delta == 0
            if "source" in entry and entry["mu"] != 0:
                weak |= r[entry["source"]] == 0
    return "weak" if weak else "strict"


def published(model, combo, u, r, ground):
    """Exact published rows of one state: member and support groups, signed slots, displacements."""
    groups, signed = {}, {}
    for pipe_id, kg, T, length, dofs in model.members:
        ue = [u[d] for d in dofs]
        fg = [sum(kg[a][b] * ue[b] for b in range(12)) for a in range(12)]
        vl = [sum(T[a][b] * fg[b] for b in range(12)) for a in range(12)]
        locations = {"end_i": vl[:6], "end_j": vl[6:]}
        for name, frac in zip(["cut_i", "quarter_1", "midspan", "quarter_3", "cut_j"], [F(0), F(1, 4), F(1, 2), F(3, 4), F(1)]):
            x = length * frac
            v = [-z for z in vl[:6]]
            v[4] -= x * vl[2]
            v[5] += x * vl[1]
            locations[name] = v
        for loc, v in locations.items():
            for kind, rows in [("axial", v[0:1]), ("shear", v[1:3]), ("torsion", v[3:4]), ("bending", v[4:6])]:
                groups.setdefault(pipe_id, {}).setdefault(loc, {})[kind] = sqrt_bounds(sum(x * x for x in rows))
    slots = {}
    for entry, (state, direction, beta) in zip(model.nonlinear, combo):
        d = entry["dof"]
        if state in ("active", "sticking") or (state == "sliding" and entry["mu"] != 0):
            value = r[d]
        else:
            value = F(0)
        slots[entry["id"]] = (d, value)
        signed[entry["id"]] = {"slot": DOFS[d % 6], "value": bounds(value)}
        if entry["id"] in ground:
            signed[entry["id"]]["relative_displacement"] = bounds(u[d] - ground[entry["id"]])
    for support_id, nd in model.supports:
        rows = [F(0)] * 6
        for d in model.rigid.get(support_id, []):
            rows[d % 6] = r[d]
        for sid, d, k in model.springs:
            if sid == support_id:
                rows[d % 6] = -k * u[d]
        if support_id in slots:
            d, value = slots[support_id]
            rows[d % 6] = value
        force_sq = sum(x * x for x in rows[:3])
        moment_sq = sum(x * x for x in rows[3:])
        groups[support_id] = {"node": {
            "support_force": sqrt_bounds(force_sq), "support_moment": sqrt_bounds(moment_sq),
            "support_force_magnitude": sqrt_bounds(force_sq), "support_moment_magnitude": sqrt_bounds(moment_sq)}}
        signed.setdefault(support_id, {})["components"] = [bounds(x) for x in rows]
    displacements = {nid: [bounds(u[6 * k + c]) for c in range(6)] for k, nid in enumerate(model.nodes)}
    return groups, signed, displacements


def coupling_gain(model, combo, entry, direction, beta):
    """c*g for a sliding derived-normal support: g = dR_n/dF of the symmetric system at zero prescribed values."""
    n = model.n
    constrained = {d: F(0) for dofs in model.rigid.values() for d in dofs}
    for e, (state, _, _) in zip(model.nonlinear, combo):
        if e is not entry and state in ("active", "sticking"):
            constrained[e["dof"]] = F(0)
    free = [i for i in range(n) if i not in constrained]
    b = [F(1) if i == entry["dof"] else F(0) for i in free]
    z = solve([[model.K[i][j] for j in free] for i in free], b)
    g = sum(model.K[entry["source"]][j] * v for j, v in zip(free, z))
    return g


def enumerate_case(model, case):
    f, base, ground = model.loads(case)
    qualifying, exact_states = [], []
    for combo in product(*[states_for(e) for e in model.nonlinear]):
        solved = solve_state(model, f, base, ground, combo)
        if solved is None:
            continue
        u, r = solved
        kind = classify(model, combo, u, r, ground)
        if kind is None:
            continue
        groups, signed, displacements = published(model, combo, u, r, ground)
        states = {e["id"]: s for e, (s, _, _) in zip(model.nonlinear, combo)}
        record = {"standing": kind, "states": states, "groups": groups, "signed": signed, "displacements": displacements}
        for e, (s, direction, beta) in zip(model.nonlinear, combo):
            if s == "sliding" and "source" in e and e["mu"] != 0:
                c = -direction * e["mu"] * beta
                record.setdefault("coupling", {})[e["id"]] = float(c * coupling_gain(model, combo, e, direction, beta))
        qualifying.append(record)
        exact_states.append((kind, states, u, r))
    strict = [q for q in qualifying if q["standing"] == "strict"]
    return {"strict_states": strict, "weak_states": [q["states"] for q in qualifying if q["standing"] == "weak"],
            "base_prescribed": {DOFS[d % 6] + "@" + model.nodes[d // 6]: bounds(v) for d, v in sorted(base.items())},
            "ground_motion": {k: bounds(v) for k, v in sorted(ground.items())}}, exact_states


# ----------------------------------------------------------------------------- the benchmarks
CROSS = []


def check(name, agrees):
    CROSS.append({"check": name, "agrees": bool(agrees)})


L4 = 4.0
KB4 = 3 * E * I / length_cube(4)  # 3EI/L^3 at L = 4 m
# A-1's prop threshold s* = 5PL^3/(48EI), chosen dyadic; P follows exactly.
S_STAR = F(1, 2 ** 10)
P_A1 = 48 * E * I * S_STAR / (5 * length_cube(4))


def a1_request():
    """A-1: clamped cantilever, midspan load P, tip one-way prop; the anchor rises by s."""
    P = as_float(P_A1)
    nodes = [node("node:A", 0.0, 0.0, 0.0), node("node:M", 2.0, 0.0, 0.0), node("node:T", 4.0, 0.0, 0.0)]
    pipes = [pipe("pipe:AM", "node:A", "node:M", (0, 0, 1)), pipe("pipe:MT", "node:M", "node:T", (0, 0, 1))]
    supports = [anchor("support:ANCHOR", "node:A"),
                nonlinear("support:PROP", "node:T", {"behavior": "one_way", "dof": "UZ", "initial_state": "active",
                                                      "active_when": "positive_reaction"})]
    cases = []
    for cid, s in (("load:ACTIVE", S_STAR / 2), ("load:LIFTED", 2 * S_STAR), ("load:BOUNDARY", S_STAR)):
        cases.append(case(cid, [force(cid + "-P", "node:M", "z", -P), motion(cid + "-S", "support:ANCHOR", "UZ", as_float(s))]))
    # The anchor-rotation twin: RY = theta raises the tip by -theta*L.
    for cid, s in (("load:ROT-ACTIVE", S_STAR / 2), ("load:ROT-LIFTED", 2 * S_STAR), ("load:ROT-BOUNDARY", S_STAR)):
        cases.append(case(cid, [force(cid + "-P", "node:M", "z", -P), motion(cid + "-T", "support:ANCHOR", "RY", as_float(-s / 4))]))
    return document("a1", "W-16 stage 2 A-1 (R-a): a moving anchor under a one-way tip prop.", nodes, pipes, supports, cases)


def a1_closed_form(model, case_id, exact_states):
    rise = {"load:ACTIVE": S_STAR / 2, "load:LIFTED": 2 * S_STAR, "load:BOUNDARY": S_STAR,
            "load:ROT-ACTIVE": S_STAR / 2, "load:ROT-LIFTED": 2 * S_STAR, "load:ROT-BOUNDARY": S_STAR}[case_id]
    tip = 6 * model.nodes.index("node:T") + 2
    R = 5 * P_A1 / 16 - KB4 * rise
    states = {(k, s["support:PROP"]): (u, r) for k, s, u, r in exact_states}
    if rise < S_STAR:
        u, r = states[("strict", "active")]
        check(f"A-1 {case_id}: prop reaction 5P/16 - 3EI s/L^3 = 5P/32", r[tip] == R == 5 * P_A1 / 32)
    elif rise > S_STAR:
        u, r = states[("strict", "inactive")]
        check(f"A-1 {case_id}: lifted tip u = s - s* = s*", u[tip] == rise - S_STAR == S_STAR)
    else:
        check(f"A-1 {case_id}: at s = s* both states are weak (R = 0, u_tip = 0)",
              set(states) == {("weak", "active"), ("weak", "inactive")} and R == 0)


# A-2: a tip load P presses the tip onto a rigid UZ restraint whose ground
# settles by d; a tip friction support on UX takes its derived normal from it.
D_A2 = F(1, 2 ** 12)
RN_A2 = KB4 * D_A2  # the normal after settlement
P_A2 = 2 * RN_A2  # R_n = P - k_b*d = k_b*d
MU_A2 = F(1, 2)


def a2_request():
    P = as_float(P_A2)
    capacity = MU_A2 * RN_A2
    nodes = [node("node:A", 0.0, 0.0, 0.0), node("node:T", 4.0, 0.0, 0.0)]
    pipes = [pipe("pipe:AT", "node:A", "node:T", (0, 0, 1))]
    supports = [anchor("support:ANCHOR", "node:A"), anchor("support:N", "node:T", ("UZ",), family="guide"),
                nonlinear("support:NL-F", "node:T", {"behavior": "friction", "dof": "UX", "initial_state": "sticking",
                                                      "friction_coefficient": qv(float(MU_A2), "none"),
                                                      "normal_reaction_source": {"support_ref": "support:N", "dof": "UZ"}})]
    cases = []
    for cid, t in (("load:STICK", capacity / 2), ("load:SLIDE", 2 * capacity), ("load:BOUNDARY", capacity)):
        cases.append(case(cid, [force(cid + "-P", "node:T", "z", -P), force(cid + "-T", "node:T", "x", as_float(t)),
                                motion(cid + "-D", "support:N", "UZ", as_float(-D_A2))]))
    return document("a2", "W-16 stage 2 A-2 (R-a): a settling derived-normal source under a tip friction support.",
                    nodes, pipes, supports, cases)


def a2_closed_form(model, case_id, exact_states):
    t = {"load:STICK": MU_A2 * RN_A2 / 2, "load:SLIDE": 2 * MU_A2 * RN_A2, "load:BOUNDARY": MU_A2 * RN_A2}[case_id]
    tip = 6 * model.nodes.index("node:T")
    states = {(k, s["support:NL-F"]): (u, r) for k, s, u, r in exact_states}
    kx = E * A / 4
    if case_id == "load:STICK":
        u, r = states[("strict", "sticking")]
        check("A-2 STICK: R_n = P - k_b d and the tangential reaction -T", r[tip + 2] == P_A2 - KB4 * D_A2 and r[tip] == -t and u[tip] == 0)
    elif case_id == "load:SLIDE":
        u, r = states[("strict", "sliding")]
        check("A-2 SLIDE: u_t = (T - mu R_n)/(EA/L) and the reaction -mu R_n", u[tip] == (t - MU_A2 * RN_A2) / kx and r[tip] == -MU_A2 * RN_A2)
    else:
        check("A-2 BOUNDARY: |T| = mu R_n is weak (sticking at capacity)", ("weak", "sticking") in states and not any(k == "strict" for k, _ in states))


# A-3: the planar L-frame; the derived normal source settles and the frame
# couples UX to it, so the influence gamma is nonzero.
L3 = F(2)
MU_A3 = F(1, 4)
G_N_A3 = F(-1, 2 ** 12)  # settlement of the tip UY restraint
F_N_A3 = F(-2048)  # tip UY load (N)
F_T_A3 = F(4096)  # tip UX load (N)


def a3_request():
    nodes = [node("node:A", 0.0, 0.0, 0.0), node("node:C", 2.0, 0.0, 0.0), node("node:T", 2.0, 2.0, 0.0)]
    pipes = [pipe("pipe:AC", "node:A", "node:C", (0, 1, 0)), pipe("pipe:CT", "node:C", "node:T", (-1, 0, 0))]
    supports = [anchor("support:ANCHOR", "node:A"), anchor("support:N", "node:T", ("UY",), family="guide"),
                nonlinear("support:NL-F", "node:T", {"behavior": "friction", "dof": "UX", "initial_state": "sliding",
                                                      "friction_coefficient": qv(float(MU_A3), "none"),
                                                      "normal_reaction_source": {"support_ref": "support:N", "dof": "UY"}})]
    cases = [case("load:SLIDE", [force("load:SLIDE-N", "node:T", "y", as_float(F_N_A3)),
                                 force("load:SLIDE-T", "node:T", "x", as_float(F_T_A3)),
                                 motion("load:SLIDE-G", "support:N", "UY", as_float(G_N_A3))])]
    return document("a3", "W-16 stage 2 A-3 (R-a): a settling derived-normal source on an L-frame (coupled friction).",
                    nodes, pipes, supports, cases)


def condensed(model, keep, fixed):
    """The stiffness on `keep` with `fixed` held at zero and every other DOF condensed out."""
    rest = [i for i in range(model.n) if i not in keep and i not in fixed]
    out = []
    for a in keep:
        row = []
        for b in keep:
            # K_ab - K_a,rest K_rest^-1 K_rest,b
            z = solve([[model.K[i][j] for j in rest] for i in rest], [model.K[i][b] for i in rest])
            row.append(model.K[a][b] - sum(model.K[a][i] * zi for i, zi in zip(rest, z)))
        out.append(row)
    return out


A3_CLOSED = {}


def a3_closed_form(model, case_id, exact_states):
    tip = 6 * model.nodes.index("node:T")
    n_dof, t_dof = tip + 1, tip
    fixed = set(model.rigid["support:ANCHOR"])
    (a, b), (_, d) = condensed(model, [n_dof, t_dof], fixed)
    gamma = b / d
    strict = [(s, u, r) for k, s, u, r in exact_states if k == "strict"]
    check("A-3: one strict state, sliding", len(strict) == 1 and strict[0][0]["support:NL-F"] == "sliding")
    states, u, r = strict[0]
    direction = 1 if u[t_dof] > 0 else -1
    beta = 1 if r[n_dof] > 0 else -1
    c_f = -direction * MU_A3 * beta
    alpha = (a - b * b / d) * G_N_A3 + (b / d) * F_T_A3 - F_N_A3
    check("A-3: the denominator 1 - c_f*gamma is nonzero", 1 - c_f * gamma != 0)
    phi = c_f * alpha / (1 - c_f * gamma)
    u_t = (F_T_A3 + phi - b * G_N_A3) / d
    check("A-3: the reduced closed form phi = c_f alpha/(1 - c_f gamma) equals the full solve's friction force", r[t_dof] == phi)
    check("A-3: the reduced closed form u_t = (f_t + phi - b g_n)/d equals the full solve", u[t_dof] == u_t)
    check("A-3: Coulomb equilibrium |phi| = mu |R_n| opposing the slip", abs(phi) == MU_A3 * abs(r[n_dof]) and phi * u[t_dof] < 0)
    check("A-3: gamma is nonzero", gamma != 0)
    # Mutants the benchmark must kill: an influence system that carries g_n
    # (unit and base solved with the motion on one side only), and a Stage A
    # that omits the motion.
    gamma_carried = gamma + (a - b * b / d) * G_N_A3
    phi_carried = c_f * alpha / (1 - c_f * gamma_carried)
    alpha_omitted = (b / d) * F_T_A3 - F_N_A3
    phi_omitted = c_f * alpha_omitted / (1 - c_f * gamma)
    A3_CLOSED.update({"a": bounds(a), "b": bounds(b), "d": bounds(d), "gamma": bounds(gamma), "alpha": bounds(alpha),
                      "dir": direction, "beta": beta, "phi": bounds(phi), "u_t": bounds(u_t),
                      "mutant_influence_carries_g_phi": bounds(phi_carried), "mutant_stage_a_omits_motion_phi": bounds(phi_omitted)})
    check("A-3: the influence-carries-g mutant changes phi", phi_carried != phi)
    check("A-3: the motion-omitting Stage A mutant changes phi", phi_omitted != phi)


def a3_unit_check():
    """The independent L-frame at L = EA = EI = 1, tip rotation free: K_(n,t) = (1/103)[[84, 18], [18, 48]]."""
    request = a3_request()
    for n in request["model"]["nodes"]:
        for axis in "xyz":
            n["position"][axis] = n["position"][axis] / 2
    model = Model(request, leaves=(F(1), F(1), F(2)), e=F(1), g=F(1))
    tip = 6 * model.nodes.index("node:T")
    k = condensed(model, [tip + 1, tip], set(model.rigid["support:ANCHOR"]))
    check("A-3 unit L-frame: K_(n,t) = (1/103)[[84, 18], [18, 48]] and gamma = 3/8",
          k == [[F(84, 103), F(18, 103)], [F(18, 103), F(48, 103)]] and k[0][1] / k[1][1] == F(3, 8))


# A property instance under R-a: A-2 with every anchor and rigid restraint
# translated rigidly by t in Y, a direction no nonlinear support acts on.
T_PROP = F(1, 2 ** 8)


def a2_translated_request():
    request = a2_request()
    m = request["model"]
    m["project"]["id"] = "project:w16nl-a2-translated"
    m["project"]["description"] = "W-16 stage 2 A-2 (R-a) translated rigidly by t in Y (the property)."
    for c in m["load_cases"]:
        c["primitive_loads"].append(motion(c["id"] + "-TY", "support:ANCHOR", "UY", as_float(T_PROP)))
    return request


def translated(request, name, axis):
    """The property (§5): every rigid restraint and nonlinear ground acting on U<axis> moves by
    one more t = 2^-8 m, so the whole model translates rigidly in that axis."""
    dof = "U" + axis
    m = request["model"]
    m["project"]["id"] = f"project:w16nl-{name}"
    m["project"]["description"] += f" Translated rigidly by t in {axis} (the property)."
    for c in m["load_cases"]:
        loads = c["primitive_loads"]
        for s in m["supports"]:
            nl = s.get("nonlinear")
            # A nonlinear ground on its own DOF (R-b); a rigid restraint, a mixed record's included (R-d).
            acts = (nl is not None and nl["dof"] == dof) or (s.get("family") != "spring" and dof in s["restraints"])
            if not acts:
                continue
            existing = [l for l in loads if l["category"] == "imposed_displacement"
                        and l["target"]["support"] == s["id"] and l["target"]["dof"] == dof]
            if existing:
                existing[0]["magnitude"]["value"] = as_float(F(existing[0]["magnitude"]["value"]) + T_PROP)
            else:
                loads.append(motion(f"{c['id']}-T{axis}-{s['id'].split(':')[-1]}", s["id"], dof, as_float(T_PROP)))
    return request


def translated_check(model, name, axis, case_id, exact_states, base_states):
    shifted = [6 * k + axis for k in range(len(model.nodes))]
    base = {(k, tuple(sorted(s.items()))): (u, r) for k, s, u, r in base_states}
    moved = {(k, tuple(sorted(s.items()))): (u, r) for k, s, u, r in exact_states}
    same = set(base) == set(moved) and all(
        moved[key][1] == base[key][1]
        and all(moved[key][0][i] == base[key][0][i] + (T_PROP if i in shifted else 0) for i in range(model.n))
        for key in base)
    check(f"{name} {case_id}: states and reactions unchanged, u shifted by t in {'XYZ'[axis]}", same)


# ----------------------------------------------------------------------------- R-b
# B-1: a tip lift-off support whose ground drops by d; d* = P L^3/(3EI) = P/k_b.
D_STAR = F(1, 2 ** 10)
P_B1 = KB4 * D_STAR


def cantilever_nodes():
    return [node("node:A", 0.0, 0.0, 0.0), node("node:T", 4.0, 0.0, 0.0)], [pipe("pipe:AT", "node:A", "node:T", (0, 0, 1))]


def b1_request():
    P = as_float(P_B1)
    nodes, pipes = cantilever_nodes()
    supports = [anchor("support:ANCHOR", "node:A"),
                nonlinear("support:LIFT", "node:T", {"behavior": "lift_off", "dof": "UZ", "initial_state": "active",
                                                      "contact_when": "positive_reaction"})]
    cases = []
    for cid, d in (("load:ACTIVE", D_STAR / 2), ("load:LIFTED", 2 * D_STAR), ("load:BOUNDARY", D_STAR)):
        cases.append(case(cid, [force(cid + "-P", "node:T", "z", -P), motion(cid + "-G", "support:LIFT", "UZ", as_float(-d))]))
    return document("b1", "W-16 stage 2 B-1 (R-b): a tip lift-off support whose ground drops.", nodes, pipes, supports, cases)


def b1_closed_form(model, case_id, exact_states):
    d = {"load:ACTIVE": D_STAR / 2, "load:LIFTED": 2 * D_STAR, "load:BOUNDARY": D_STAR}[case_id]
    tip = 6 * model.nodes.index("node:T") + 2
    states = {(k, s["support:LIFT"]): (u, r) for k, s, u, r in exact_states}
    if d < D_STAR:
        u, r = states[("strict", "active")]
        check(f"B-1 {case_id}: R = P - 3EI d/L^3 = P/2 and u = -d", r[tip] == P_B1 - KB4 * d == P_B1 / 2 and u[tip] == -d)
    elif d > D_STAR:
        u, r = states[("strict", "inactive")]
        check(f"B-1 {case_id}: u = -d*, separation d - d* = d*", u[tip] == -D_STAR and u[tip] - (-d) == d - D_STAR == D_STAR)
        # A classifier on the absolute u (u < 0: contact) would close it: the mutant is visible.
        check(f"B-1 {case_id}: the absolute-u classifier would wrongly engage (u < 0)", u[tip] < 0)
    else:
        check(f"B-1 {case_id}: at d = d* both states are weak", set(states) == {("weak", "active"), ("weak", "inactive")})


def b1_spring_request():
    """B-1's spring twin: a separate spring record k = k_b at the tip UZ; R = P - (k + k_b) d. Kills K-D5's M04."""
    P = as_float(P_B1)
    nodes, pipes = cantilever_nodes()
    supports = [anchor("support:ANCHOR", "node:A"),
                nonlinear("support:LIFT", "node:T", {"behavior": "lift_off", "dof": "UZ", "initial_state": "active",
                                                      "contact_when": "positive_reaction"}),
                spring("support:K", "node:T", "UZ", as_float(KB4))]
    star = D_STAR / 2  # P/(k + k_b)
    cases = []
    for cid, d in (("load:ACTIVE", star / 2), ("load:LIFTED", 2 * star)):
        cases.append(case(cid, [force(cid + "-P", "node:T", "z", -P), motion(cid + "-G", "support:LIFT", "UZ", as_float(-d))]))
    return document("b1_spring", "W-16 stage 2 B-1 (R-b) spring twin: a fixed-ground parallel spring.", nodes, pipes, supports, cases)


def b1_spring_closed_form(model, case_id, exact_states):
    star = D_STAR / 2
    d = {"load:ACTIVE": star / 2, "load:LIFTED": 2 * star}[case_id]
    tip = 6 * model.nodes.index("node:T") + 2
    states = {(k, s["support:LIFT"]): (u, r) for k, s, u, r in exact_states}
    if d < star:
        u, r = states[("strict", "active")]
        # The contact force is the nonlinear slot, (K u - f) at the DOF with the
        # spring in K: the spring's ground stays fixed, so its force is -k u = k d.
        check(f"B-1 spring {case_id}: R = P - (k + 3EI/L^3) d, and u = -d", r[tip] == P_B1 - 2 * KB4 * d and u[tip] == -d)
        # K-D5's M04 (the spring at d excluded from the contact force) would publish P - k_b d instead.
        check(f"B-1 spring {case_id}: M04 is observable (k d != 0)", r[tip] != P_B1 - KB4 * d)
    else:
        u, r = states[("strict", "inactive")]
        check(f"B-1 spring {case_id}: u = -P/(k + k_b)", u[tip] == -star)


# The rotational twin: a tip lift-off on RY (pushing -RY) with the tip UZ free;
# a tip load -F; the condensed rotational tip stiffness k_theta = EI/L.
THETA_STAR = F(1, 2 ** 10)  # the free tip rotation F L^2/(2EI)
F_B1R = 2 * E * I * THETA_STAR / 16


def b1_rot_request():
    nodes, pipes = cantilever_nodes()
    supports = [anchor("support:ANCHOR", "node:A"),
                nonlinear("support:LIFT", "node:T", {"behavior": "lift_off", "dof": "RY", "initial_state": "active",
                                                      "contact_when": "negative_reaction"})]
    cases = []
    for cid, g in (("load:ACTIVE", THETA_STAR / 2), ("load:LIFTED", 2 * THETA_STAR)):
        cases.append(case(cid, [force(cid + "-F", "node:T", "z", as_float(-F_B1R)), motion(cid + "-G", "support:LIFT", "RY", as_float(g))]))
    return document("b1_rot", "W-16 stage 2 B-1 (R-b) rotational twin: a tip rotational lift-off whose ground rotates.",
                    nodes, pipes, supports, cases)


def b1_rot_closed_form(model, case_id, exact_states):
    g = {"load:ACTIVE": THETA_STAR / 2, "load:LIFTED": 2 * THETA_STAR}[case_id]
    tip = 6 * model.nodes.index("node:T")
    (k_theta,), = condensed(model, [tip + 4], set(model.rigid["support:ANCHOR"]))
    check(f"B-1 rotational {case_id}: the condensed rotational tip stiffness is EI/L", k_theta == E * I / 4)
    states = {(k, s["support:LIFT"]): (u, r) for k, s, u, r in exact_states}
    if g < THETA_STAR:
        u, r = states[("strict", "active")]
        check(f"B-1 rotational {case_id}: R = -k_theta (theta* - g) and theta = g", r[tip + 4] == -k_theta * (THETA_STAR - g) and u[tip + 4] == g)
    else:
        u, r = states[("strict", "inactive")]
        check(f"B-1 rotational {case_id}: theta = theta* (free), separation g - theta*", u[tip + 4] == THETA_STAR)


# B-2: a gap closing in -, whose ground rises by h; delta0 = P L^3/(3EI) < c.
DELTA0 = F(1, 2 ** 10)
P_B2 = KB4 * DELTA0
C_B2 = 2 * DELTA0


def b2_request(sense="negative"):
    """Both closing senses: in -, the load is -P and the ground rises by h; in +, mirrored."""
    s = -1 if sense == "negative" else 1
    nodes, pipes = cantilever_nodes()
    supports = [anchor("support:ANCHOR", "node:A"),
                nonlinear("support:GAP", "node:T", {"behavior": "gap", "dof": "UZ", "initial_state": "inactive",
                                                     "closes_when": f"{sense}_displacement", "gap": qv(as_float(C_B2), "m")})]
    cases = []
    for cid, h in (("load:OPEN", DELTA0 / 2), ("load:CLOSED", 3 * DELTA0), ("load:EQUAL", C_B2 - DELTA0), ("load:CANCEL", C_B2)):
        cases.append(case(cid, [force(cid + "-P", "node:T", "z", as_float(s * P_B2)), motion(cid + "-G", "support:GAP", "UZ", as_float(-s * h))]))
    name = "b2" if sense == "negative" else "b2_positive"
    return document(name, f"W-16 stage 2 B-2 (R-b): a gap closing in {'-' if s < 0 else '+'} whose ground moves toward it.",
                    nodes, pipes, supports, cases)


def b2_closed_form(model, case_id, exact_states, s=-1):
    h = {"load:OPEN": DELTA0 / 2, "load:CLOSED": 3 * DELTA0, "load:EQUAL": C_B2 - DELTA0, "load:CANCEL": C_B2}[case_id]
    tip = 6 * model.nodes.index("node:T") + 2
    states = {(k, st["support:GAP"]): (u, r) for k, st, u, r in exact_states}
    label = "B-2" if s < 0 else "B-2 (+)"
    if h < C_B2 - DELTA0:
        u, r = states[("strict", "inactive")]
        check(f"{label} {case_id}: open, u = the free tip deflection", u[tip] == s * DELTA0)
    elif h > C_B2 - DELTA0:
        u, r = states[("strict", "active")]
        check(f"{label} {case_id}: closed at u = sigma*(c + sigma*g_s), R = P + k_b (h - c)",
              u[tip] == -s * (h - C_B2) and -s * r[tip] == P_B2 + KB4 * (h - C_B2))
    else:
        check(f"{label} {case_id}: at h = c - delta0 both states are weak (R = 0, closing)", set(states) == {("weak", "active"), ("weak", "inactive")})


def b2_negative_clearance_request():
    """The effective clearance c + sigma g_s < 0: the ground has risen past the clearance."""
    nodes, pipes = cantilever_nodes()
    c = DELTA0 / 2
    supports = [anchor("support:ANCHOR", "node:A"),
                nonlinear("support:GAP", "node:T", {"behavior": "gap", "dof": "UZ", "initial_state": "inactive",
                                                     "closes_when": "negative_displacement", "gap": qv(as_float(c), "m")})]
    cases = [case("load:CLOSED", [force("load:CLOSED-P", "node:T", "z", as_float(-P_B2)), motion("load:CLOSED-G", "support:GAP", "UZ", as_float(2 * DELTA0))])]
    return document("b2_negative_clearance", "W-16 stage 2 B-2 (R-b): a negative effective clearance.", nodes, pipes, supports, cases)


def b2_negative_clearance_closed_form(model, case_id, exact_states):
    c, h = DELTA0 / 2, 2 * DELTA0
    tip = 6 * model.nodes.index("node:T") + 2
    states = {(k, st["support:GAP"]): (u, r) for k, st, u, r in exact_states}
    u, r = states[("strict", "active")]
    check("B-2 negative clearance: c - h < 0, closed at u = h - c, R = P + k_b (h - c)", c - h < 0 and u[tip] == h - c and r[tip] == P_B2 + KB4 * (h - c))


# B-3: friction under ground motion g_t; g* = mu |P_y| L/(EA).
G_STAR = F(1, 2 ** 12)
MU_B3 = F(1, 2)
KX = E * A / 4
N_B3 = KX * G_STAR / MU_B3  # the normal |P_y|


def b3_request(normal="derived", px=F(0)):
    nodes, pipes = cantilever_nodes()
    spec = {"behavior": "friction", "dof": "UX", "initial_state": "sticking", "friction_coefficient": qv(float(MU_B3), "none")}
    supports = [anchor("support:ANCHOR", "node:A")]
    if normal == "derived":
        supports.append(anchor("support:N", "node:T", ("UY",), family="guide"))
        spec["normal_reaction_source"] = {"support_ref": "support:N", "dof": "UY"}
    else:
        spec["normal_reaction"] = qv(as_float(N_B3), "N")
    supports.append(nonlinear("support:NL-F", "node:T", spec))
    lo, hi = (px - MU_B3 * N_B3) / KX, (px + MU_B3 * N_B3) / KX
    picks = {"load:STICK": (lo + hi) / 2 + (hi - lo) / 4, "load:STICK-NEG": (lo + hi) / 2 - (hi - lo) / 4,
             "load:SLIDE": hi + (hi - lo), "load:SLIDE-NEG": lo - (hi - lo), "load:BOUNDARY": hi}
    cases = []
    for cid, g in picks.items():
        loads = []
        if normal == "derived":
            loads.append(force(cid + "-PY", "node:T", "y", as_float(-N_B3)))
        if px:
            loads.append(force(cid + "-PX", "node:T", "x", as_float(px)))
        loads.append(motion(cid + "-G", "support:NL-F", "UX", as_float(g)))
        cases.append(case(cid, loads))
    name = {("derived", 0): "b3", ("explicit", 0): "b3_explicit"}.get((normal, px), "b3_px")
    return document(name, f"W-16 stage 2 B-3 (R-b): friction under ground motion ({normal} normal, P_x = {float(px)} N).",
                    nodes, pipes, supports, cases), picks


PX_B3 = MU_B3 * N_B3 / 2


def b3_closed_form_for(px, label="B-3"):
    def closed(model, case_id, exact_states):
        lo, hi = (px - MU_B3 * N_B3) / KX, (px + MU_B3 * N_B3) / KX
        tip = 6 * model.nodes.index("node:T")
        picks = {"load:STICK": (lo + hi) / 2 + (hi - lo) / 4, "load:STICK-NEG": (lo + hi) / 2 - (hi - lo) / 4,
                 "load:SLIDE": hi + (hi - lo), "load:SLIDE-NEG": lo - (hi - lo), "load:BOUNDARY": hi}
        g = picks[case_id]
        states = {(k, st["support:NL-F"]): (u, r) for k, st, u, r in exact_states}
        if lo < g < hi:
            u, r = states[("strict", "sticking")]
            check(f"{label} (P_x={float(px)}) {case_id}: sticking, u_x = g_t, R_t = k_x g_t - P_x", u[tip] == g and r[tip] == KX * g - px)
        elif g > hi or g < lo:
            u, r = states[("strict", "sliding")]
            sign = 1 if g > hi else -1
            check(f"{label} (P_x={float(px)}) {case_id}: sliding, u_x = (P_x + sign(g) mu N)/k_x; the force opposes the relative slip",
                  u[tip] == (px + sign * MU_B3 * N_B3) / KX and r[tip] * (u[tip] - g) < 0)
            # An absolute-slip classifier reads sign(u_x), which differs from sign(u_x - g_t) here.
            check(f"{label} (P_x={float(px)}) {case_id}: the relative slip u_x - g_t opposes u_x itself, so an absolute-slip classifier fails",
                  (u[tip] - g) * u[tip] < 0)
        else:
            check(f"{label} (P_x={float(px)}) {case_id}: at the boundary sticking is weak (|R_t| = mu N, zero slip)",
                  ("weak", "sticking") in states and not any(k == "strict" for k, _ in states))
    return closed


# ----------------------------------------------------------------------------- R-d
# A record of family "nonlinear" assembles its rigid restraints as a guide,
# which holds translations only (a rotational restraint on it is refused), so
# the moving rigid restraint is translational and the nonlinear support
# rotational. Signs: the member runs along +X; the condensed tip coupling
# K[RY, UZ] is +6EI/L^2.
#
# D-1: a 4 m cantilever (midspan node), midspan load -P; the tip record is
# mixed: a rotational lift-off on RY (contact for a positive moment) with
# rigid restraints on UY and UZ. The tip UZ drops by s. Held at RY = 0, the
# tip carries the fixed-end moment PL/8 less the coupling 6EI s/L^2 (the
# condensed 4 m cantilever), so the support lifts off at s* = PL^3/(48EI),
# chosen dyadic; P follows exactly.
S_STAR_D = F(1, 2 ** 10)
V6 = 6 * E * I / F(16)  # 6EI/L^2 at L = 4 m
K_THETA = 4 * E * I / F(4)  # 4EI/L
P_D = 48 * E * I * S_STAR_D / length_cube(4)


def d_nodes():
    nodes = [node("node:A", 0.0, 0.0, 0.0), node("node:M", 2.0, 0.0, 0.0), node("node:T", 4.0, 0.0, 0.0)]
    pipes = [pipe("pipe:AM", "node:A", "node:M", (0, 0, 1)), pipe("pipe:MT", "node:M", "node:T", (0, 0, 1))]
    return nodes, pipes


def tip_record(restraints):
    return nonlinear("support:TIP", "node:T", {"behavior": "lift_off", "dof": "RY", "initial_state": "active",
                                                "contact_when": "positive_reaction"}, restraints=restraints)


def d1_request():
    P = as_float(P_D)
    nodes, pipes = d_nodes()
    supports = [anchor("support:ANCHOR", "node:A"), tip_record(("UY", "UZ"))]
    cases = []
    for cid, sv in (("load:ACTIVE", S_STAR_D / 2), ("load:LIFTED", 2 * S_STAR_D), ("load:BOUNDARY", S_STAR_D)):
        cases.append(case(cid, [force(cid + "-P", "node:M", "z", -P), motion(cid + "-S", "support:TIP", "UZ", as_float(-sv))]))
    return document("d1", "W-16 stage 2 D-1 (R-d): a mixed tip record, rotational lift-off with rigid UY and UZ; UZ drops.",
                    nodes, pipes, supports, cases)


def tip_condensed(model, at):
    """The condensed 4 m cantilever on the tip (UZ, RY), the anchor held: [[12EI/L^3, 6EI/L^2], [6EI/L^2, 4EI/L]]."""
    tip = 6 * model.nodes.index("node:T")
    (kzz, kzr), (krz, krr) = condensed(model, [tip + 2, tip + 4], set(model.rigid["support:ANCHOR"]))
    check(f"{at}: the condensed tip stiffness is 12EI/L^3, 6EI/L^2 and 4EI/L",
          kzz == 12 * E * I / length_cube(4) and kzr == krz == V6 and krr == K_THETA)
    return tip


def d1_closed_form(model, case_id, exact_states):
    sv = {"load:ACTIVE": S_STAR_D / 2, "load:LIFTED": 2 * S_STAR_D, "load:BOUNDARY": S_STAR_D}[case_id]
    tip = tip_condensed(model, f"D-1 {case_id}")
    states = {(k, st["support:TIP"]): (u, r) for k, st, u, r in exact_states}
    if sv < S_STAR_D:
        u, r = states[("strict", "active")]
        check(f"D-1 {case_id}: R = PL/8 - 6EI s/L^2 = PL/16, RY = 0 and UZ = -s (the base value)",
              r[tip + 4] == P_D * 4 / 8 - V6 * sv == P_D * 4 / 16 and u[tip + 4] == 0 and u[tip + 2] == -sv)
        # A record whose rigid UZ stayed fixed would carry the full fixed-end moment.
        check(f"D-1 {case_id}: the motion is observable (R != PL/8)", r[tip + 4] != P_D * 4 / 8)
    elif sv > S_STAR_D:
        u, r = states[("strict", "inactive")]
        check(f"D-1 {case_id}: R = 0, RY = 3(s - s*)/(2L) > 0 (separation) and UZ = -s",
              r[tip + 4] == 0 and u[tip + 4] == 3 * (sv - S_STAR_D) / 8 and u[tip + 4] > 0 and u[tip + 2] == -sv)
    else:
        check(f"D-1 {case_id}: at s = s* both states are weak", set(states) == {("weak", "active"), ("weak", "inactive")})


# D-3: D-1's record without UY, whose own ground rotates by -g (R-b) while its
# rigid UZ drops by s (R-d): the record is classified per DOF. Each motion
# alone, then both. Active: R = PL/8 - 6EI s/L^2 - 4EI g/L.
G_D3 = 3 * S_STAR_D / 32  # 4EI g/L = PL/32
D3_CASES = {"load:GROUND": (F(0), G_D3), "load:RIGID": (S_STAR_D / 4, F(0)),
            "load:ACTIVE": (S_STAR_D / 4, G_D3), "load:LIFTED": (S_STAR_D, G_D3)}


def d3_request():
    P = as_float(P_D)
    nodes, pipes = d_nodes()
    supports = [anchor("support:ANCHOR", "node:A"), tip_record(("UZ",))]
    cases = []
    for cid, (sv, gv) in D3_CASES.items():
        loads = [force(cid + "-P", "node:M", "z", -P)]
        if gv:
            loads.append(motion(cid + "-G", "support:TIP", "RY", as_float(-gv)))
        if sv:
            loads.append(motion(cid + "-S", "support:TIP", "UZ", as_float(-sv)))
        cases.append(case(cid, loads))
    return document("d3", "W-16 stage 2 D-3 (R-b and R-d on one record): a tip rotational lift-off whose ground rotates and whose rigid UZ drops, each alone and together.",
                    nodes, pipes, supports, cases)


def d3_closed_form(model, case_id, exact_states):
    sv, gv = D3_CASES[case_id]
    tip = tip_condensed(model, f"D-3 {case_id}")
    states = {(k, st["support:TIP"]): (u, r) for k, st, u, r in exact_states}
    fixed_end = P_D * 4 / 8
    reaction = fixed_end - V6 * sv - K_THETA * gv
    if case_id != "load:LIFTED":
        u, r = states[("strict", "active")]
        check(f"D-3 {case_id}: R = PL/8 - 6EI s/L^2 - 4EI g/L > 0, RY = -g and UZ = -s",
              r[tip + 4] == reaction > 0 and u[tip + 4] == -gv and u[tip + 2] == -sv)
        # Each motion present is observable: R differs without it.
        check(f"D-3 {case_id}: each motion present is observable",
              (sv == 0 or r[tip + 4] != fixed_end - K_THETA * gv) and (gv == 0 or r[tip + 4] != fixed_end - V6 * sv))
        if sv and gv:
            check(f"D-3 {case_id}: R = PL/16", r[tip + 4] == P_D * 4 / 16)
    else:
        u, r = states[("strict", "inactive")]
        check(f"D-3 {case_id}: R = 0, RY = 3(s - s*)/(2L) = 0, separation RY - (-g) = g > 0, UZ = -s",
              r[tip + 4] == 0 and reaction < 0 and u[tip + 4] == 0 and u[tip + 4] + gv == gv and u[tip + 2] == -sv)


REQUESTS = [
    ("a1", "R-a", a1_request, a1_closed_form),
    ("a2", "R-a", a2_request, a2_closed_form),
    ("a3", "R-a", a3_request, a3_closed_form),
    ("a2_translated", "R-a", a2_translated_request, None),
    ("b1", "R-b", b1_request, b1_closed_form),
    ("b1_spring", "R-b", b1_spring_request, b1_spring_closed_form),
    ("b1_rot", "R-b", b1_rot_request, b1_rot_closed_form),
    ("b1_translated", "R-b", lambda: translated(b1_request(), "b1_translated", "Z"), None),
    ("b2", "R-b", b2_request, b2_closed_form),
    ("b2_positive", "R-b", lambda: b2_request("positive"), lambda m, c, e: b2_closed_form(m, c, e, s=1)),
    ("b2_negative_clearance", "R-b", b2_negative_clearance_request, b2_negative_clearance_closed_form),
    ("b2_translated", "R-b", lambda: translated(b2_request(), "b2_translated", "Z"), None),
    ("b3", "R-b", lambda: b3_request()[0], b3_closed_form_for(F(0))),
    ("b3_explicit", "R-b", lambda: b3_request("explicit")[0], b3_closed_form_for(F(0), "B-3 explicit normal")),
    ("b3_px", "R-b", lambda: b3_request("derived", PX_B3)[0], b3_closed_form_for(PX_B3)),
    ("b3_translated", "R-b", lambda: translated(b3_request()[0], "b3_translated", "X"), None),
    ("d1", "R-d", d1_request, d1_closed_form),
    ("d3", "R-d", d3_request, d3_closed_form),
    ("d1_translated", "R-d", lambda: translated(d1_request(), "d1_translated", "Y"), None),
]

# The base of each translated request (the property).
TRANSLATED = {"a2_translated": ("a2", 1), "b1_translated": ("b1", 2), "b2_translated": ("b2", 2), "b3_translated": ("b3", 0),
              "d1_translated": ("d1", 1)}


def encode(obj):
    return (json.dumps(obj, indent=1, sort_keys=True) + "\n").encode()


def build():
    CROSS.clear()
    A3_CLOSED.clear()
    a3_unit_check()
    files, oracles = {}, {}
    kept = {}
    for name, part, make, closed in REQUESTS:
        request = make()
        data = encode(request)
        files[f"requests/{name}.json"] = data
        model = Model(json.loads(data))
        cases = {}
        for c in model.cases:
            oracle, exact_states = enumerate_case(model, c)
            cases[c["id"]] = oracle
            kept[(name, c["id"])] = exact_states
            if closed:
                closed(model, c["id"], exact_states)
            if name in TRANSLATED:
                base, axis = TRANSLATED[name]
                translated_check(model, name, axis, c["id"], exact_states, kept[(base, c["id"])])
        oracles[name] = {"part": part, "request": f"requests/{name}.json", "request_sha256": hashlib.sha256(data).hexdigest(),
                         "cases": cases}
    references = {
        "document": "openpipestress.w16_nonlinear_motion_references",
        "status": "candidate frozen references; derived and cross-checked by the author TASK, not yet independently refuted",
        "basis": "exact rational solve of independently formed Euler-Bernoulli frames with the case's prescribed values, every support state enumerated; states measured on u - g_s",
        "units": {"force": "N", "moment": "N*m", "translation": "m", "rotation": "rad"},
        "normalized_section": {"outside_diameter_m": OD, "wall_thickness_m": WALL, **LEAVES},
        "elastic_modulus_pa": E_PA,
        "shear_modulus_pa": G_PA,
        "criterion": "certified rows within the K-D5 ball bound B of the exact value; displacements within 1e-9 relative (an exact zero within 1e-9 of the case's largest translation or rotation); prescribed DOFs bit for bit",
        "a3_reduced_closed_form": dict(A3_CLOSED),
        "crosscheck": list(CROSS),
        "requests": oracles,
    }
    files["references.json"] = encode(references)
    sums = []
    for path in sorted(list(files) + list(FIXED_FILES)):
        data = files[path] if path in files else (OUT / path).read_bytes()
        sums.append(f"{hashlib.sha256(data).hexdigest()}  {path}")
    files["SHA256SUMS"] = ("\n".join(sums) + "\n").encode()
    return files


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    files = build()
    bad = [c for c in CROSS if not c["agrees"]]
    oracles = json.loads(files["references.json"])["requests"]
    for name, oracle in oracles.items():
        print(f"{name} ({oracle['part']}): " + ", ".join(
            f"{cid}:{len(c['strict_states'])} strict/{len(c['weak_states'])} weak" for cid, c in oracle["cases"].items()))
    print(f"cross-checks: {len(CROSS)} recorded, {len(bad)} disagreeing")
    for c in bad:
        print("DISAGREES:", c["check"])
    if args.write:
        if bad:
            print("not written: a cross-check disagrees")
            return 1
        for path, data in files.items():
            target = OUT / path
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(data)
        print(f"wrote {len(files)} files")
        return 0
    failures = [p for p, d in files.items() if not (OUT / p).exists() or (OUT / p).read_bytes() != d]
    committed = {str(p.relative_to(OUT)) for p in OUT.rglob("*") if p.is_file() and "__pycache__" not in p.parts}
    extra = sorted(committed - set(files) - set(FIXED_FILES))
    if failures or extra or bad:
        print("MISMATCH: " + ", ".join(failures + [f"unexpected {e}" for e in extra]))
        return 1
    print(f"verified {len(files)} files byte for byte (SHA256SUMS included)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
