#!/usr/bin/env python3
"""RT-A3 S6 independent references (DEL-04-07 `Work/route-unification-r4.md` §3.7).

C-1: a pressurized objective joint between two anchors with a one-way guide on
one side, its no-nonlinear, tie-rod, gap, Timoshenko and load/reference-state
(moved anchor) controls. LS-1: W-16 stage 2's A-1, D-3 and C-5
(`validation/references/w16_nonlinear_motion/`) as exact-route
load/reference-state twins, with Timoshenko twins and dropped-motion mutants.

Standard library only. It imports, reads and runs no product code, product
output or generated envelope. It reads only the frozen W-16 request documents
and their oracle (`references.json`), both SHA-bound by that packet's
SHA256SUMS, as inputs and as a cross-check.

Every case is solved twice, exactly (fractions.Fraction):
  A  the force method with physical loads (unit-load integrals on the
     cantilever; axial free bodies with Lame wall force and fluid face loads);
  B  direct stiffness with the product's ledger representation (EB or
     Timoshenko 12x12 restricted to the loaded plane; caps, Poisson pairs and
     the two joint summands; springs' k*g; partitioned prescribed values),
     enumerating every support-state combination.
A and B must agree on every published value, B must find exactly one strict
state, and its state must be A's. Otherwise nothing is written and the exit
status is 1.

Run `python3 -I generators/rt_a3_s6_reference.py` from the packet directory to
rebuild every output in memory and compare it byte for byte with the committed
files; `--write` rewrites them.

`--pressure-elongation {absent,all,none}` (default: $SWBPIPE_PRESSURE_ELONGATION,
else absent) sets `model_settings.pressure_elongation` in the written requests
through the shared `tools/validation/pressure_elongation.py` (DEL-04-07
`Work/pressure-elongation.md` §7); the references are today's mechanics,
pressure elongation included, whatever the switch.
"""
from __future__ import annotations

import copy
import hashlib
import itertools
import json
import math
import pathlib
import sys
from fractions import Fraction as Fr

HERE = pathlib.Path(__file__).resolve().parent.parent
W16 = HERE.parent / "w16_nonlinear_motion"
sys.path.append(str(HERE.parents[2]))  # the repository root, for the shared pressure-elongation switch
from tools.validation import pressure_elongation as PE  # noqa: E402
DOFS = ("UX", "UY", "UZ", "RX", "RY", "RZ")
PROV = "invented RT-A3 S6 reference input (validation/references/rt_a3_s6); not library, catalog, manufacturer or code-rule data"
MODEL_VERSION = "0.9.0"
CONTRACT_STATE = "openpipestress.load_reference_state/1.0.0"


class Fail(Exception):
    pass


CHECKS: list[dict] = []


def check(label: str, ok: bool, detail: str = "") -> None:
    CHECKS.append({"check": label, "agrees": bool(ok)})
    if not ok:
        raise Fail(f"{label}: {detail}")


def eq(label: str, a: Fr, b: Fr) -> None:
    check(label, a == b, f"{a} != {b}")


def num(x: Fr) -> dict:
    """An exact value with its correctly rounded binary64."""
    return {"exact": str(x), "value": float(x)}


def f64(x: float) -> Fr:
    return Fr(x)  # the exact binary64 leaf


# ---------------------------------------------------------------------------
# Shared inputs: W-16's material and section leaves (declared, binary64)

W16_ORACLE_BYTES = (W16 / "references.json").read_bytes()
W16_HASHES = {name: digest for digest, name in
              (line.split("  ", 1) for line in (W16 / "SHA256SUMS").read_text().splitlines())}
if hashlib.sha256(W16_ORACLE_BYTES).hexdigest() != W16_HASHES.get("references.json"):
    raise Fail("W16 references.json does not match its SHA256SUMS binding")
W16_REF = json.loads(W16_ORACLE_BYTES)
LEAF = W16_REF["normalized_section"]
E = f64(W16_REF["elastic_modulus_pa"])            # 5*2^35 Pa
G = f64(W16_REF["shear_modulus_pa"])              # 2^36 Pa
NU = E / (2 * G) - 1                              # 1/4 exactly
assert NU == Fr(1, 4) and E / (2 * (1 + NU)) == G
A_S = f64(LEAF["As_m2"])
I_S = f64(LEAF["I_m4"])
J_S = f64(LEAF["J_m4"])
OD, WALL = Fr("0.206"), Fr("0.011")
RO = OD / 2
RI = RO - WALL
# DEL-05-03 section-bases-and-timoshenko.md §3.1: the energy-matched annular
# shear factor, exact in the decimal radii and nu.
_m2 = (RI / RO) ** 2
_c = (8 * NU + 14) * NU + 7
KAPPA = (6 * (1 + NU) ** 2) * (1 + _m2) ** 2 / (_c * (1 - _m2) ** 2 + 48 * _m2 * (1 + NU) ** 2)
A_SH = A_S * KAPPA
# C-1's pressure bore area: the binary64 leaf fl(pi/4 * d^2), d = 0.184 m.
A_I = f64(math.pi / 4.0 * (0.184 * 0.184))


# ---------------------------------------------------------------------------
# Linear algebra

def solve(a: list[list[Fr]], b: list[Fr]) -> list[Fr]:
    n = len(b)
    m = [row[:] + [b[i]] for i, row in enumerate(a)]
    for c in range(n):
        p = next((r for r in range(c, n) if m[r][c] != 0), None)
        if p is None:
            raise Fail("singular system")
        m[c], m[p] = m[p], m[c]
        piv = m[c][c]
        m[c] = [v / piv for v in m[c]]
        for r in range(n):
            if r != c and m[r][c] != 0:
                f = m[r][c]
                m[r] = [vr - f * vc for vr, vc in zip(m[r], m[c])]
    return [m[i][n] for i in range(n)]


# ---------------------------------------------------------------------------
# Part LS-1: cantilevers along +X bent in the x-z plane (w = UZ, t = RY)
#
# Model: anchor A at x = 0 (possibly moved: w_A, t_A); nodes at given x;
# point forces Fz; tip supports on UZ or RY: rigid (prescribed s), one-way /
# lift-off (ground g, sense sigma), spring (stiffness k, base g_k).

class Beam:
    def __init__(self, xs, timo: bool):
        self.xs = xs                  # node x positions, xs[0] = anchor
        self.timo = timo

    # --- B: direct stiffness ------------------------------------------------
    def element(self, length: Fr) -> list[list[Fr]]:
        ei = E * I_S
        phi = 12 * ei / (G * A_SH * length * length) if self.timo else Fr(0)
        L = length
        f = ei / (L ** 3 * (1 + phi))
        # (w_i, t_i, w_j, t_j), t = RY: K[RY_j, UZ_j] = +6EI/L^2 (W-16 D-1).
        return [
            [12 * f, -6 * L * f, -12 * f, -6 * L * f],
            [-6 * L * f, (4 + phi) * L * L * f, 6 * L * f, (2 - phi) * L * L * f],
            [-12 * f, 6 * L * f, 12 * f, 6 * L * f],
            [-6 * L * f, (2 - phi) * L * L * f, 6 * L * f, (4 + phi) * L * L * f],
        ]

    def stiffness(self):
        n = 2 * len(self.xs)
        k = [[Fr(0)] * n for _ in range(n)]
        for e in range(len(self.xs) - 1):
            ke = self.element(self.xs[e + 1] - self.xs[e])
            idx = [2 * e, 2 * e + 1, 2 * e + 2, 2 * e + 3]
            for a in range(4):
                for b in range(4):
                    k[idx[a]][idx[b]] += ke[a][b]
        return k

    def solve_b(self, case, states):
        """Direct stiffness for one state assignment; returns (u, R) or None."""
        n = 2 * len(self.xs)
        k = self.stiffness()
        f = [Fr(0)] * n
        for node, fz in case["forces"]:
            f[2 * node] += fz
        prescribed = {0: case["anchor"][0], 1: case["anchor"][1]}
        for s in case["supports"]:
            d = 2 * s["node"] + (0 if s["dof"] == "UZ" else 1)
            if s["kind"] == "spring":
                k[d][d] += s["k"]
                f[d] += s["k"] * s["g"]
            elif s["kind"] == "rigid":
                prescribed[d] = s["g"]
            elif states[s["id"]] == "active":
                if d in prescribed:
                    raise Fail("repeated prescribed DOF")
                prescribed[d] = s["g"]
        free = [d for d in range(n) if d not in prescribed]
        kff = [[k[i][j] for j in free] for i in free]
        rhs = [f[i] - sum(k[i][j] * v for j, v in prescribed.items()) for i in free]
        uf = solve(kff, rhs)
        u = [Fr(0)] * n
        for i, d in enumerate(free):
            u[d] = uf[i]
        for d, v in prescribed.items():
            u[d] = v
        # Support-on-pipe actions (K_frame u - f_ext at each support DOF, the
        # springs' -k(u - g) separately).
        kf = self.stiffness()
        resid = [sum(kf[i][j] * u[j] for j in range(n)) - sum(fz for nd, fz in case["forces"] if 2 * nd == i) for i in range(n)]
        actions = {"anchor": (resid[0], resid[1])}
        spring_at: dict[int, Fr] = {}
        for s in case["supports"]:
            d = 2 * s["node"] + (0 if s["dof"] == "UZ" else 1)
            if s["kind"] == "spring":
                actions[s["id"]] = -s["k"] * (u[d] - s["g"])
                spring_at[d] = spring_at.get(d, Fr(0)) + actions[s["id"]]
        for s in case["supports"]:
            d = 2 * s["node"] + (0 if s["dof"] == "UZ" else 1)
            if s["kind"] == "rigid" or (s["kind"] != "spring" and states[s["id"]] == "active"):
                actions[s["id"]] = resid[d] - spring_at.get(d, Fr(0))
            elif s["kind"] != "spring":
                actions[s["id"]] = Fr(0)
        return u, actions

    # --- A: force method ----------------------------------------------------
    def flex(self, x0: Fr, d0: str, x1: Fr, d1: str) -> Fr:
        """Generalized displacement d0 at x0 for a unit generalized force d1
        at x1 on the cantilever clamped at x = 0 (unit-load integrals).
        Internal action of a unit Fz at a: m = (a - x), v = 1 on [0, a];
        of a unit RY moment at a: m = -1, v = 0 on [0, a]; w'' = m/EI and
        RY = -w'."""
        lim = min(x0, x1)

        def mp(d, a):  # polynomial m(x) = c0 + c1 x
            return (a, Fr(-1)) if d == "UZ" else (Fr(-1), Fr(0))

        a0, b0 = mp(d0, x0)
        a1, b1 = mp(d1, x1)
        # integral_0^lim (a0 + b0 x)(a1 + b1 x) dx
        c0, c1, c2 = a0 * a1, a0 * b1 + a1 * b0, b0 * b1
        bend = (c0 * lim + c1 * lim ** 2 / 2 + c2 * lim ** 3 / 3) / (E * I_S)
        shear = Fr(0)
        if self.timo and d0 == "UZ" and d1 == "UZ":
            shear = lim / (G * A_SH)
        return bend + shear

    def solve_a(self, case, states):
        """Force method: the tip support actions are the redundants."""
        anchor_w, anchor_t = case["anchor"]

        def rigid_body(x, d):
            return anchor_w - anchor_t * x if d == "UZ" else anchor_t

        reds = [s for s in case["supports"]]
        n = len(reds)
        rows, rhs = [], []
        for i, s in enumerate(reds):
            xi = self.xs[s["node"]]
            base = rigid_body(xi, s["dof"]) + sum(self.flex(xi, s["dof"], self.xs[nd], "UZ") * fz for nd, fz in case["forces"])
            coeff = [self.flex(xi, s["dof"], self.xs[r["node"]], r["dof"]) for r in reds]
            if s["kind"] == "spring":
                # R = -k (u - g): R + k u = k g
                row = [Fr(1) if j == i else Fr(0) for j in range(n)]
                row = [row[j] + s["k"] * coeff[j] for j in range(n)]
                rows.append(row)
                rhs.append(s["k"] * s["g"] - s["k"] * base)
            elif s["kind"] == "rigid" or states[s["id"]] == "active":
                rows.append(coeff)
                rhs.append(s["g"] - base)
            else:
                rows.append([Fr(1) if j == i else Fr(0) for j in range(n)])
                rhs.append(Fr(0))
        red = solve(rows, rhs) if n else []
        loads = list(case["forces"]) + [(r["node"], None) for r in reds]

        def disp(x, d):
            v = rigid_body(x, d) + sum(self.flex(x, d, self.xs[nd], "UZ") * fz for nd, fz in case["forces"])
            v += sum(self.flex(x, d, self.xs[r["node"]], r["dof"]) * red[j] for j, r in enumerate(reds))
            return v

        u = []
        for x in self.xs:
            u += [disp(x, "UZ"), disp(x, "RY")]
        actions = {r["id"]: red[j] for j, r in enumerate(reds)}
        # Anchor reactions from statics: total Fz and RY moment about x = 0
        # (a force Fz at x has RY moment -x*Fz about the origin).
        fz_total = sum(fz for _, fz in case["forces"]) + sum(red[j] for j, r in enumerate(reds) if r["dof"] == "UZ")
        my_total = sum(-self.xs[nd] * fz for nd, fz in case["forces"])
        my_total += sum((-self.xs[r["node"]] * red[j]) if r["dof"] == "UZ" else red[j] for j, r in enumerate(reds))
        actions["anchor"] = (-fz_total, -my_total)
        del loads
        return u, actions


def strict_ok(case, states, u, actions) -> tuple[bool, bool]:
    """(admissible, strict) for DEC-067 unilateral supports on delta = u - g."""
    adm, strict = True, True
    for s in case["supports"]:
        if s["kind"] not in ("one_way", "lift_off"):
            continue
        d = 2 * s["node"] + (0 if s["dof"] == "UZ" else 1)
        sigma = s["sigma"]
        if states[s["id"]] == "active":
            c = sigma * actions[s["id"]]
        else:
            c = sigma * (u[d] - s["g"])
        adm &= c >= 0
        strict &= c > 0
    return adm, strict


def ls_states(case):
    nl = [s["id"] for s in case["supports"] if s["kind"] in ("one_way", "lift_off")]
    for combo in itertools.product(("active", "inactive"), repeat=len(nl)):
        yield dict(zip(nl, combo))


# ---------------------------------------------------------------------------
# W-16 request -> LS-1 load/reference-state twin

def w16_request(name: str) -> tuple[dict, str]:
    data = (W16 / "requests" / f"{name}.json").read_bytes()
    digest = hashlib.sha256(data).hexdigest()
    sums = (W16 / "SHA256SUMS").read_text()
    check(f"W16 {name}: request bound by its SHA256SUMS", f"{digest}  requests/{name}.json" in sums)
    check(f"W16 {name}: request bound by its oracle", W16_REF["requests"][name]["request_sha256"] == digest)
    return json.loads(data), digest


def acting(support: dict, dof: str) -> str:
    nl = support.get("nonlinear")
    if nl and nl["dof"] == dof:
        return "nonlinear"
    if support.get("family") == "spring" and support.get("stiffness", {}).get("dof") == dof:
        return "spring"
    if dof in support.get("restraints", []):
        return "rigid"
    return "none"


def twin(name: str, shear_model: str) -> tuple[dict, str]:
    """The exact-route load/reference-state twin of a W-16 request: the same
    nodes, members, supports, loads and units; v3 contract; E/nu with nu =
    E/(2G) - 1; explicit zero corrosion allowance; and per case an
    analysis_state whose load sources are the non-motion primitives at factor
    1 and whose support states carry each motion primitive as the law of the
    DOF it names (rigid/nonlinear: boundary_motion; spring: base_motion)."""
    doc, digest = w16_request(name)
    m = copy.deepcopy(doc["model"])
    m["schema_version"] = MODEL_VERSION
    m["pressure_contract"] = {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": shear_model}
    m["project"]["id"] = f"project:invented-rt-a3-s6-{name}-{shear_model.split('_')[0]}"
    for mat in m["materials"]:
        e, g = mat["elastic_modulus"]["value"], mat["shear_modulus"]["value"]
        nu = e / (2.0 * g) - 1.0
        assert (e / (2.0 * (1.0 + nu))) == g and nu == 0.25
        m_new = {"id": mat["id"], "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
                 "elastic_modulus": mat["elastic_modulus"], "poisson_ratio": {"value": nu, "unit": "1"},
                 "provenance": mat["provenance"]}
        mat.clear()
        mat.update(m_new)
    for pipe in m["pipe_segments"]:
        pipe["section"]["corrosion_allowance"] = {"value": 0.0, "unit": "m"}
    m["reference_configurations"] = [{
        "id": "reference:installed", "label": "Installed reference",
        "geometry_ref": {"kind": "authored_model_geometry"},
        "member_references": [{"pipe_ref": p["id"], "basis": {"kind": "direct_strain_reference"},
                               "fit": {"kind": "none"}, "provenance": PROV} for p in m["pipe_segments"]],
        "provenance": PROV}]
    supports = {s["id"]: s for s in m["supports"]}
    for case in m["load_cases"]:
        motions: dict[str, dict] = {s: {"boundary_motion": [], "base_motion": []} for s in supports}
        kept = []
        for load in case["primitive_loads"]:
            t = load["target"]
            if t.get("type") == "support":
                act = acting(supports[t["support"]], t["dof"])
                assert act in ("rigid", "nonlinear", "spring")
                field = "base_motion" if act == "spring" else "boundary_motion"
                motions[t["support"]][field].append({"dof": t["dof"], "value": load["magnitude"],
                                                     "meaning": "absolute_reference_displacement"})
            else:
                kept.append(load)
        case["primitive_loads"] = kept
        case["pressure_regions"] = []
        states = []
        for sid in supports:
            entry = {"support_ref": sid, "participation": {"kind": "active_model_device"}}
            for field in ("boundary_motion", "base_motion"):
                if motions[sid][field]:
                    entry[field] = motions[sid][field]
            states.append(entry)
        case["analysis_state"] = {
            "contract": CONTRACT_STATE,
            "reference_configuration_ref": "reference:installed",
            "element_states": [{"pipe_ref": p["id"],
                                "material_selection": {"kind": "explicit_base_properties", "material_ref": p["material"],
                                                       "applicability_reference": PROV},
                                "thermal_state": {"kind": "unchanged_reference", "provenance": PROV}}
                               for p in m["pipe_segments"]],
            "support_states": states,
            "load_sources": [{"source_ref": l["id"], "factor": 1.0} for l in kept],
            "history": {"kind": "independent_equilibrium"},
            "provenance": PROV,
        }
    return {"model": m, "materials": []}, digest


def beam_case(request: dict, case_id: str, drop: tuple[str, str, str] | None = None):
    """The LS-1 mechanics of one twin case, read from the document (A and B
    share this reading only). `drop` = (support, field, dof) removes one
    motion entry: the dropped-motion mutant."""
    m = request["model"]
    nodes = {n["id"]: i for i, n in enumerate(m["nodes"])}
    xs = [f64(n["position"]["x"]) for n in m["nodes"]]
    for n in m["nodes"]:
        assert n["position"]["y"] == 0 and n["position"]["z"] == 0
    case = next(c for c in m["load_cases"] if c["id"] == case_id)
    forces = []
    for load in case["primitive_loads"]:
        assert load["category"] == "concentrated_force" and load["direction"] == "global_z"
        forces.append((nodes[load["target"]["node"]], f64(load["magnitude"]["value"])))
    motion = {}
    for st in case["analysis_state"]["support_states"]:
        for field in ("boundary_motion", "base_motion"):
            for mo in st.get(field, []):
                if drop == (st["support_ref"], field, mo["dof"]):
                    continue
                motion[(st["support_ref"], mo["dof"])] = f64(mo["value"]["value"])
    anchor = (Fr(0), Fr(0))
    sups = []
    for s in m["supports"]:
        node = nodes[s["node"]]
        if s["family"] == "anchor":
            assert node == 0
            assert all((s["id"], d) not in motion for d in ("UX", "UY", "RX", "RZ"))
            anchor = (motion.get((s["id"], "UZ"), Fr(0)), motion.get((s["id"], "RY"), Fr(0)))
            continue
        nl = s.get("nonlinear")
        if nl:
            sigma = 1 if (nl.get("active_when") or nl.get("contact_when")) == "positive_reaction" else -1
            sups.append({"id": s["id"], "node": node, "dof": nl["dof"], "kind": nl["behavior"], "sigma": sigma,
                         "g": motion.get((s["id"], nl["dof"]), Fr(0)), "initial": nl["initial_state"]})
            for r in s.get("restraints", []):
                assert r in ("UY", "UZ")
                if r == "UZ":
                    sups.append({"id": s["id"] + "#UZ", "node": node, "dof": "UZ", "kind": "rigid",
                                 "g": motion.get((s["id"], "UZ"), Fr(0))})
        elif s["family"] == "spring":
            sups.append({"id": s["id"], "node": node, "dof": s["stiffness"]["dof"], "kind": "spring",
                         "k": f64(s["stiffness"]["value"]["value"]), "g": motion.get((s["id"], s["stiffness"]["dof"]), Fr(0))})
        else:
            raise Fail(f"unexpected support {s['id']}")
    return xs, {"forces": forces, "anchor": anchor, "supports": sups}


def solve_ls(request: dict, case_id: str, timo: bool, label: str, drop=None, allow_weak=False):
    xs, case = beam_case(request, case_id, drop)
    beam = Beam(xs, timo)
    strict, weak = [], []
    for states in ls_states(case):
        u, act = beam.solve_b(case, states)
        adm, st = strict_ok(case, states, u, act)
        if st:
            strict.append((states, u, act))
        elif adm:
            weak.append(states)
    if allow_weak and not strict and weak:
        # Every weak state of an exact boundary publishes the same response
        # (a zero contact and a zero separation); A confirms it.
        responses = [beam.solve_b(case, st) for st in weak]
        for u2, act2 in responses[1:]:
            check(f"{label}: the weak states coincide", u2 == responses[0][0] and act2 == responses[0][1])
        ua, acta = beam.solve_a(case, weak[0])
        check(f"{label}: A = B at the weak boundary", ua == responses[0][0])
        return None, weak, (xs, case, weak[0]) + responses[0]
    check(f"{label}: B finds exactly one strict state and no weak state", len(strict) == 1 and not weak, f"{len(strict)} strict, {weak}")
    states, u, act = strict[0]
    ua, acta = beam.solve_a(case, states)
    for i in range(len(u)):
        eq(f"{label}: A = B displacement {i}", ua[i], u[i])
    for key in act:
        if key == "anchor":
            eq(f"{label}: A = B anchor Fz", acta["anchor"][0], act["anchor"][0])
            eq(f"{label}: A = B anchor My", acta["anchor"][1], act["anchor"][1])
        else:
            eq(f"{label}: A = B {key}", acta[key], act[key])
    return xs, case, states, u, act


def ls_record(request, case_id, xs, case, states, u, act):
    m = request["model"]
    nodes = [n["id"] for n in m["nodes"]]
    disp = {}
    for i, nid in enumerate(nodes):
        comp = [Fr(0)] * 6
        comp[2], comp[4] = u[2 * i], u[2 * i + 1]
        disp[nid] = [num(c) for c in comp]
    sup = {}
    anchor_id = next(s["id"] for s in m["supports"] if s["family"] == "anchor")
    comp = [Fr(0)] * 6
    comp[2], comp[4] = act["anchor"]
    sup[anchor_id] = {"components": [num(c) for c in comp]}
    relative = {}
    for s in case["supports"]:
        if s["kind"] == "rigid":
            base = s["id"].split("#")[0]
            entry = sup.setdefault(base, {"components": [num(Fr(0))] * 6})
            entry["components"][2] = num(act[s["id"]])
            continue
        k = DOFS.index(s["dof"])
        entry = sup.setdefault(s["id"], {"components": [num(Fr(0))] * 6})
        entry["components"][k] = num(act[s["id"]])
        if s["kind"] in ("one_way", "lift_off"):
            entry["slot"] = s["dof"]
            entry["state"] = states[s["id"]]
            d = 2 * s["node"] + (0 if s["dof"] == "UZ" else 1)
            relative[s["id"]] = num(u[d] - s["g"])
    return {"states": states, "displacements": disp, "supports": sup, "relative_displacement": relative,
            "scales": scales(disp, sup)}


def scales(disp, sup):
    """The case's family zero scales: the largest expected magnitude of each
    family (translation m, rotation rad, force N, moment N*m)."""
    t = max([abs(Fr(c["exact"])) for v in disp.values() for c in v[:3]] + [Fr(0)])
    r = max([abs(Fr(c["exact"])) for v in disp.values() for c in v[3:]] + [Fr(0)])
    f = max([abs(Fr(c["exact"])) for v in sup.values() for c in v["components"][:3]] + [Fr(0)])
    mo = max([abs(Fr(c["exact"])) for v in sup.values() for c in v["components"][3:]] + [Fr(0)])
    return {"translation_m": num(t), "rotation_rad": num(r), "force_n": num(f), "moment_nm": num(mo)}


def separated(a: dict, b: dict) -> Fr:
    """The largest separation of two records' displacement and support
    values, each in units of the 1e-9 criterion of record a."""
    sc = a["scales"]
    worst = Fr(0)
    tol = Fr(1, 10 ** 9)
    for nid, comps in a["displacements"].items():
        for k, c in enumerate(comps):
            x, y = Fr(c["exact"]), Fr(b["displacements"][nid][k]["exact"])
            z = Fr(sc["translation_m" if k < 3 else "rotation_rad"]["exact"])
            denom = tol * (abs(x) if x != 0 else z)
            if denom:
                worst = max(worst, abs(x - y) / denom)
    for sid, entry in a["supports"].items():
        for k, c in enumerate(entry["components"]):
            x, y = Fr(c["exact"]), Fr(b["supports"].get(sid, {"components": [{"exact": "0"}] * 6})["components"][k]["exact"])
            z = Fr(sc["force_n" if k < 3 else "moment_nm"]["exact"])
            denom = tol * (abs(x) if x != 0 else z)
            if denom:
                worst = max(worst, abs(x - y) / denom)
    return worst


def w16_crosscheck(name, case_id, rec):
    """EB twins: the exact values lie in W-16's frozen binary64 enclosures."""
    ref = W16_REF["requests"][name]["cases"][case_id]["strict_states"]
    check(f"W16 {name} {case_id}: one strict state in the frozen oracle", len(ref) == 1)
    ref = ref[0]
    check(f"W16 {name} {case_id}: states agree", {k: v for k, v in ref["states"].items()} == rec["states"], f"{ref['states']} {rec['states']}")
    for nid, comps in rec["displacements"].items():
        for k, c in enumerate(comps):
            lo, hi = (f64(v) for v in ref["displacements"][nid][k])
            x = Fr(c["exact"])
            check(f"W16 {name} {case_id}: {nid} {DOFS[k]} in the frozen enclosure", lo <= x <= hi or float(x) in (float(lo), float(hi)), f"{x} not in [{lo}, {hi}]")
    for sid, entry in rec["supports"].items():
        if sid not in ref["signed"]:
            continue
        for k, c in enumerate(entry["components"]):
            lo, hi = (f64(v) for v in ref["signed"][sid]["components"][k])
            x = Fr(c["exact"])
            check(f"W16 {name} {case_id}: {sid} {DOFS[k]} in the frozen enclosure", lo <= x <= hi or float(x) in (float(lo), float(hi)), f"{x} not in [{lo}, {hi}]")


LS_TWINS = [
    # (request, shear model, cases)
    ("a1", "euler_bernoulli", ["load:ACTIVE", "load:LIFTED", "load:ROT-ACTIVE", "load:ROT-LIFTED"]),
    ("d3", "euler_bernoulli", ["load:GROUND", "load:RIGID", "load:ACTIVE", "load:LIFTED"]),
    ("c5", "euler_bernoulli", ["load:ACTIVE", "load:LIFTED", "load:UNEQUAL"]),
    ("d3", "timoshenko_annulus_energy_v1", ["load:ACTIVE", "load:LIFTED"]),
    ("c5", "timoshenko_annulus_energy_v1", ["load:ACTIVE", "load:UNEQUAL"]),
]


def motion_entries(request, case_id):
    case = next(c for c in request["model"]["load_cases"] if c["id"] == case_id)
    out = []
    for st in case["analysis_state"]["support_states"]:
        for field in ("boundary_motion", "base_motion"):
            for mo in st.get(field, []):
                out.append((st["support_ref"], field, mo["dof"]))
    return out


def build_ls():
    requests, refs = {}, {}
    for name, shear, cases in LS_TWINS:
        timo = shear != "euler_bernoulli"
        key = f"ls1_{name}" + ("_timoshenko" if timo else "")
        request, digest = twin(name, shear)
        out = {"part": "LS-1", "derives_from": {"request": f"w16_nonlinear_motion/requests/{name}.json", "request_sha256": digest},
               "shear_model": shear, "request": f"requests/{key}.json", "cases": {}}
        for case_id in cases:
            label = f"{key} {case_id}"
            xs, case, states, u, act = solve_ls(request, case_id, timo, label)
            rec = ls_record(request, case_id, xs, case, states, u, act)
            if not timo:
                w16_crosscheck(name, case_id, rec)
            mutants = {}
            for entry in motion_entries(request, case_id):
                mlabel = f"{label} drop {entry}"
                solved = solve_ls(request, case_id, timo, mlabel, drop=entry, allow_weak=True)
                if solved[0] is None:
                    # The mutant sits on an exact weak boundary: no strict
                    # state, so the product cannot certify it, while the
                    # reference is strict. Its boundary response must still
                    # differ from the reference's, or the reference's
                    # moved-ground separation must be nonzero.
                    wrec = ls_record(request, case_id, *solved[2])
                    sep = separated(rec, wrec)
                    check(f"{mlabel}: the weak-boundary mutant is distinguished",
                          sep > 10 ** 6 or any(Fr(v["exact"]) != 0 for v in rec["relative_displacement"].values()), str(float(sep)))
                    wrec["weak_boundary"] = True
                    wrec["weak_states"] = solved[1]
                    wrec["separation_in_criteria"] = float(sep)
                    mutants[f"{entry[0]}|{entry[1]}|{entry[2]}"] = wrec
                    continue
                xs2, case2, states2, u2, act2 = solved
                mrec = ls_record(request, case_id, xs2, case2, states2, u2, act2)
                sep = separated(rec, mrec)
                check(f"{mlabel}: the mutant is separated from the reference by more than 10^6 criteria", sep > 10 ** 6, str(float(sep)))
                mrec["separation_in_criteria"] = float(sep)
                mutants[f"{entry[0]}|{entry[1]}|{entry[2]}"] = mrec
            rec["dropped_motion_mutants"] = mutants
            out["cases"][case_id] = rec
        requests[key] = request
        refs[key] = out
    # Timoshenko twins move the response (shear flexibility is observable).
    for name in ("d3", "c5"):
        for case_id, rec in refs[f"ls1_{name}_timoshenko"]["cases"].items():
            eb = refs[f"ls1_{name}"]["cases"][case_id]
            check(f"ls1_{name} {case_id}: Timoshenko differs from Euler-Bernoulli by more than 10^3 criteria", separated(eb, rec) > 10 ** 3)
    return requests, refs


# ---------------------------------------------------------------------------
# Part C-1: anchored pipe - objective joint - anchored pipe, axial only.
#
# Nodes A(0) - pipe:L - B(2) - connector (replaces pipe:J) - C(2.5) - pipe:R - D(4.5).
# Every load is axial and the run is collinear, so every non-axial DOF is 0
# exactly: the joint's midpoint relative translation along the axis is
# u_Cx - u_Bx, and no other component couples to it.

C1 = {
    "xs": [Fr(0), Fr(2), Fr(5, 2), Fr(9, 2)],
    "nodes": ["node:A", "node:B", "node:C", "node:D"],
    "k_joint": Fr(2 ** 20),             # N/m, the connector's axial work coefficient at a 1 m scale
    "k_rod": Fr(2 ** 24),               # N/m per rod (tied control: two rods at +-0.15 m in y)
    "rod_offset": Fr(3, 20),
    "Ae": Fr(5, 128),                   # m^2, effective area 0.0390625
    "p": Fr(10 ** 6),                   # Pa
    "g_D": Fr(-1, 2 ** 13),             # m, the load/reference-state twin's anchor D motion
}
K_PIPE = [E * A_S / (C1["xs"][1] - C1["xs"][0]), E * A_S / (C1["xs"][3] - C1["xs"][2])]


def c1_closed(p, k_j, sup, g_d=Fr(0)):
    """A: axial free bodies. Pipe walls carry N = EA*eps + 2*nu*p*A_i (Lame);
    the bellows face annulus at B (and C) receives the fluid force p(Ae - Ai)
    away from the joint; the joint carries F_j = k_j (u_C - u_B). Node B:
    -N_L - p(Ae - Ai) + F_j = 0; node C: N_R + p(Ae - Ai) - F_j + R = 0.
    `sup` = None, or (kind, sigma, state) of the one-way/gap at C on UX."""
    kl, kr = K_PIPE
    q = p * (C1["Ae"] - A_I)
    pois = 2 * NU * p * A_I
    # unknowns u_B, u_C (R as needed)
    # N_L = kl*u_B + pois; N_R = kr*(g_d - u_C) + pois
    # B: -(kl uB + pois) - q + kj (uC - uB) = 0
    # C: kr (g_d - uC) + pois + q - kj (uC - uB) + R = 0
    if sup is None or sup[2] == "inactive":
        a = [[kl + k_j, -k_j], [-k_j, k_j + kr]]
        b = [-pois - q, kr * g_d + pois + q]
        ub, uc = solve(a, b)
        r = Fr(0)
    else:
        uc = sup[3]  # prescribed (0 for one-way, -+c for a closed gap)
        ub = (k_j * uc - pois - q) / (kl + k_j)
        r = -(kr * (g_d - uc) + pois + q - k_j * (uc - ub))
    n_l = kl * ub + pois
    n_r = kr * (g_d - uc) + pois
    f_j = k_j * (uc - ub)
    # Anchor reactions: node A: N_L - p*A_i + R_A = 0 (the cap at A pushes -x);
    # node D: -N_R + p*A_i + R_D = 0.
    r_a = p * A_I - n_l
    r_d = n_r - p * A_I
    return {"u": [Fr(0), ub, uc, g_d], "R_C": r, "R_A": r_a, "R_D": r_d, "N_L": n_l, "N_R": n_r, "F_j": f_j}


def c1_ledger(p, k_j, sup, states_c, g_d=Fr(0), face_caps=1, effective=1):
    """B: direct stiffness on (u_A, u_B, u_C, u_D) with the ledger: caps
    -p*A_i at A and +p*A_i at D (transfers_to_wall), Poisson pairs
    +-2*nu*p*A_i on each pipe, and at the joint the effective-area summands
    -p*Ae at B and +p*Ae at C with the face caps +p*A_i at B and -p*A_i at C."""
    kl, kr = K_PIPE
    pi_, pe, po = p * A_I, p * C1["Ae"], 2 * NU * p * A_I
    f = [-pi_ + po, -po - effective * pe + face_caps * pi_, po + effective * pe - face_caps * pi_, -po + pi_]
    k = [[Fr(0)] * 4 for _ in range(4)]
    for (i, j), kk in (((0, 1), kl), ((1, 2), k_j), ((2, 3), kr)):
        k[i][i] += kk
        k[j][j] += kk
        k[i][j] -= kk
        k[j][i] -= kk
    pres = {0: Fr(0), 3: g_d}
    if sup is not None and states_c == "active":
        pres[2] = sup[3]
    free = [d for d in range(4) if d not in pres]
    uf = solve([[k[i][j] for j in free] for i in free], [f[i] - sum(k[i][j] * v for j, v in pres.items()) for i in free])
    u = [Fr(0)] * 4
    for i, d in enumerate(free):
        u[d] = uf[i]
    for d, v in pres.items():
        u[d] = v
    res = [sum(k[i][j] * u[j] for j in range(4)) - f[i] for i in range(4)]
    return u, res


def c1_case(label, p, tied, sup_spec, g_d=Fr(0)):
    """Enumerate the guide's states with B; A at the strict one."""
    k_j = C1["k_joint"] + (2 * C1["k_rod"] if tied else 0)
    found = []
    states = ["active", "inactive"] if sup_spec else [None]
    for st in states:
        sup = None
        if sup_spec:
            kind, sigma, clearance = sup_spec
            # A one-way support holds its ground (0); a gap closing in the
            # sense sigma engages at u = sigma*c.
            closed_at = Fr(0) if kind == "one_way" else sigma * clearance
            sup = (kind, sigma, st, closed_at)
        u, res = c1_ledger(p, k_j, sup, st, g_d)
        r = res[2] if (sup and st == "active") else Fr(0)
        if sup:
            kind, sigma, clearance = sup_spec
            if kind == "one_way":
                margin = sigma * r if st == "active" else sigma * u[2]
            else:  # gap closing in sigma: separation c - sigma*u; contact -sigma*R
                margin = -sigma * r if st == "active" else clearance - sigma * u[2]
            if margin <= 0:
                if margin == 0:
                    raise Fail(f"{label}: weak state {st}")
                continue
        found.append((st, sup, u, res, r))
    check(f"{label}: B finds exactly one strict state", len(found) == 1, str(len(found)))
    st, sup, u, res, r = found[0]
    a = c1_closed(p, k_j, sup, g_d)
    for i in range(4):
        eq(f"{label}: A = B u_x {C1['nodes'][i]}", a["u"][i], u[i])
    eq(f"{label}: A = B anchor A", a["R_A"], res[0])
    eq(f"{label}: A = B anchor D", a["R_D"], res[3])
    eq(f"{label}: A = B guide reaction", a["R_C"], r)
    # Global balance of physical loads: the fluid forces on the closed
    # system cancel (caps -pAi at A, +pAi at D; joint annuli -+p(Ae-Ai)),
    # so the support actions sum to zero.
    eq(f"{label}: support actions balance", a["R_A"] + a["R_D"] + a["R_C"], Fr(0))
    rods = {}
    if tied:
        for rid in ("rod:yp", "rod:yn"):
            rods[rid] = num(C1["k_rod"] * (u[2] - u[1]))
    disp = {C1["nodes"][i]: [num(u[i])] + [num(Fr(0))] * 5 for i in range(4)}
    sup_rows = {"support:A": {"components": [num(a["R_A"])] + [num(Fr(0))] * 5},
                "support:D": {"components": [num(a["R_D"])] + [num(Fr(0))] * 5}}
    if sup_spec:
        sup_rows["support:GUIDE"] = {"components": [num(r)] + [num(Fr(0))] * 5, "slot": "UX", "state": st}
    rec = {"state": st, "displacements": disp, "supports": sup_rows,
           "joint": {"axial_force_n": num(C1["k_joint"] * (u[2] - u[1])), "joint_pressure_force_n": num(p * C1["Ae"]),
                     "area_difference_force_n": num(p * (C1["Ae"] - A_I))},
           "wall_axial_force_n": {"pipe:L": num(a["N_L"]), "pipe:R": num(a["N_R"])},
           "tie_rod_force_n": rods}
    rec["scales"] = scales(disp, sup_rows)
    return rec


C1_REQUESTS = {
    # name: (tied, guide spec (kind, sigma, clearance) or None, shear model, load_state)
    "c1": (False, ("one_way", 1, None), "euler_bernoulli", False),
    "c1_linear": (False, None, "euler_bernoulli", False),
    "c1_tied": (True, ("one_way", 1, None), "euler_bernoulli", False),
    "c1_timoshenko": (False, ("one_way", 1, None), "timoshenko_annulus_energy_v1", False),
    "c1_gap": (False, ("gap", 1, Fr(1, 2 ** 15)), "euler_bernoulli", False),
    "c1_ls": (False, ("one_way", 1, None), "euler_bernoulli", True),
}
C1_CASES = {"load:PRESSURE": Fr(10 ** 6), "load:VACUUM": Fr(-(10 ** 6))}


def q(value, unit):
    return {"value": float(value), "unit": unit}


def c1_document(name, tied, guide, shear, load_state):
    nodes = [{"id": nid, "position": {"x": float(x), "y": 0.0, "z": 0.0}, "provenance": PROV} for nid, x in zip(C1["nodes"], C1["xs"])]
    section = {"outside_diameter": q(OD, "m"), "wall_thickness": q(WALL, "m"), "corrosion_allowance": q(0, "m")}
    pipes = [{"id": pid, "from": a, "to": b, "section": section, "material": "material:s6", "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0}, "provenance": PROV}
             for pid, a, b in (("pipe:L", "node:A", "node:B"), ("pipe:J", "node:B", "node:C"), ("pipe:R", "node:C", "node:D"))]
    supports = [{"id": "support:A", "node": "node:A", "family": "anchor", "restraints": list(DOFS), "provenance": PROV},
                {"id": "support:D", "node": "node:D", "family": "anchor", "restraints": list(DOFS), "provenance": PROV}]
    if guide:
        kind, sigma, clearance = guide
        nl = {"behavior": kind, "dof": "UX", "initial_state": "active"}
        if kind == "one_way":
            nl["active_when"] = "positive_reaction" if sigma > 0 else "negative_reaction"
        else:
            nl["closes_when"] = "positive_displacement" if sigma > 0 else "negative_displacement"
            nl["gap"] = q(clearance, "m")
            nl["initial_state"] = "inactive"
        supports.append({"id": "support:GUIDE", "node": "node:C", "family": "nonlinear", "restraints": [], "nonlinear": nl, "provenance": PROV})
    axes = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
    end = lambda n: {"node_ref": n, "initial_node_axes_global": axes, "offset_local": {"x": 0, "y": 0, "z": 0, "unit": "m"}}
    k_j = float(C1["k_joint"])
    hardware = {"kind": "untied"}
    if tied:
        off = float(C1["rod_offset"])
        hardware = {"kind": "explicit_axial_rods_v1", "rods": [
            {"id": rid, "law": "bilateral_linear",
             "offset_i": {"x": q(0, "m"), "y": q(sgn * off, "m"), "z": q(0, "m")},
             "offset_j": {"x": q(0, "m"), "y": q(sgn * off, "m"), "z": q(0, "m")},
             "stiffness": q(C1["k_rod"], "N/m"), "reference_extension": q(0, "m"), "source_reference": PROV}
            for rid, sgn in (("rod:yp", 1.0), ("rod:yn", -1.0))]}
    connector = {
        "version": "1.0.0", "motion_basis": "symmetric_midpoint_small_rotation_v1",
        "end_i": end("node:B"), "end_j": end("node:C"), "connector_axes_global": axes,
        "installed_reference_temperature": {"value": 20, "unit": "degC"},
        "temperature_applicability": "fixed_installed_parameters_v1", "reference_state": "stress_free",
        "q_ref": {"translation": {"x": 0.0, "y": 0, "z": 0, "unit": "m"}, "rotation": {"x": 0, "y": 0, "z": 0, "unit": "rad"}},
        "stiffness": {"version": "1.0.0", "representation": "scaled_work_coefficients_v1",
                      "translation_scale": {"value": 1, "unit": "m"}, "rotation_scale": {"value": 1, "unit": "rad"},
                      "coefficient_unit": "N*m",
                      # tx, ty, tz, rx, ry, rz uncoupled; only tx carries load here.
                      "upper_triangle": [k_j, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0 ** 22, 0.0, 0.0, 0.0, 0.0, 2.0 ** 22, 0.0, 0.0, 0.0,
                                         2.0 ** 12, 0.0, 0.0, 2.0 ** 12, 0.0, 2.0 ** 12],
                      "coordinate_order": ["tx", "ty", "tz", "rx", "ry", "rz"],
                      "provenance": {"source_reference": PROV, "measurement_restraints": "invented",
                                     "basis_transform_reference": "none (authored directly in the midpoint basis)"}},
        "topology": {"type": "replaces_span", "span_ref": "pipe:J"},
        "calibration": {"kind": "constant_structural_elasticity_v1", "includes_pressure_dependent_tangent": False,
                        "installed_geometry": "authored model geometry"},
        "hardware": hardware,
        "pressure_model": {"kind": "constant_effective_area_reference_v1", "effective_area": q(C1["Ae"], "m^2"),
                           "pressure_basis": "internal_differential_zero_external_v1", "source_reference": PROV},
        "provenance": {"source_reference": PROV, "measurement_restraints": "invented", "basis_transform_reference": "none",
                       "validity_statement": "invented analytical control; no applicability claim"}}
    components = [{"id": "component:JOINT", "label": "Invented expansion joint (objective connector)", "kind": "expansion_joint",
                   "node": "node:C", "objective_connector": connector, "provenance": PROV}]
    cases = []
    for cid, p in C1_CASES.items():
        region = {"id": f"region:{cid.split(':')[1]}", "member_pipe_ids": ["pipe:L", "pipe:R"], "member_connector_ids": ["component:JOINT"],
                  "pressure_basis": "internal_differential_zero_external_v1", "pressure": q(p, "Pa"),
                  "terminals": [{"node_ref": "node:A", "closure_transfer": "transfers_to_wall", "provenance": PROV},
                                {"node_ref": "node:D", "closure_transfer": "transfers_to_wall", "provenance": PROV}],
                  "provenance": PROV}
        case = {"id": cid, "primitive_loads": [], "pressure_regions": [region], "provenance": PROV}
        if load_state:
            sstates = []
            for s in supports:
                entry = {"support_ref": s["id"], "participation": {"kind": "active_model_device"}}
                if s["id"] == "support:D":
                    entry["boundary_motion"] = [{"dof": "UX", "value": q(C1["g_D"], "m"), "meaning": "absolute_reference_displacement"}]
                sstates.append(entry)
            case["analysis_state"] = {
                "contract": CONTRACT_STATE, "reference_configuration_ref": "reference:installed",
                "element_states": [{"pipe_ref": p_["id"], "material_selection": {"kind": "explicit_base_properties", "material_ref": "material:s6",
                                                                               "applicability_reference": PROV},
                                    "thermal_state": {"kind": "unchanged_reference", "provenance": PROV}} for p_ in pipes],
                "support_states": sstates, "load_sources": [], "history": {"kind": "independent_equilibrium"}, "provenance": PROV}
        cases.append(case)
    model = {
        "schema_version": MODEL_VERSION, "document_kind": "openpipestress.product_preview.model",
        "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": shear},
        "project": {"id": f"project:invented-rt-a3-s6-{name}", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
        "nodes": nodes, "pipe_segments": pipes, "supports": supports, "components": components,
        "materials": [{"id": "material:s6", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
                       "elastic_modulus": q(E, "Pa"), "poisson_ratio": q(NU, "1"), "provenance": PROV}],
        "load_cases": cases, "combinations": []}
    if load_state:
        model["reference_configurations"] = [{
            "id": "reference:installed", "label": "Installed reference", "geometry_ref": {"kind": "authored_model_geometry"},
            "member_references": [{"pipe_ref": p_["id"], "basis": {"kind": "direct_strain_reference"}, "fit": {"kind": "none"}, "provenance": PROV} for p_ in pipes],
            "provenance": PROV}]
    return {"model": model, "materials": []}


def build_c1():
    requests, refs = {}, {}
    for name, (tied, guide, shear, ls) in C1_REQUESTS.items():
        out = {"part": "C-1", "request": f"requests/{name}.json", "shear_model": shear, "cases": {}}
        for cid, p in C1_CASES.items():
            g_d = C1["g_D"] if ls else Fr(0)
            label = f"{name} {cid}"
            rec = c1_case(label, p, tied, guide, g_d)
            if ls:
                rec["dropped_motion_mutants"] = {}
                mrec = c1_case(label + " drop support:D UX", p, tied, guide, Fr(0))
                sep = separated(rec, mrec)
                check(f"{label}: dropping the anchor motion is separated by more than 10^6 criteria", sep > 10 ** 6, str(float(sep)))
                mrec["separation_in_criteria"] = float(sep)
                rec["dropped_motion_mutants"]["support:D|boundary_motion|UX"] = mrec
            out["cases"][cid] = rec
        requests[name] = c1_document(name, tied, guide, shear, ls)
        refs[name] = out
    # Controls among the C-1 records.
    c1, lin = refs["c1"]["cases"], refs["c1_linear"]["cases"]
    check("C-1: the pressure opens the guide and the vacuum closes it", c1["load:PRESSURE"]["state"] == "inactive" and c1["load:VACUUM"]["state"] == "active")
    check("C-1: the open guide gives the no-nonlinear response", c1["load:PRESSURE"]["displacements"] == lin["load:PRESSURE"]["displacements"])
    check("C-1: the closed guide differs from the no-nonlinear response", separated(lin["load:VACUUM"], c1["load:VACUUM"]) > 10 ** 6)
    check("C-1: Timoshenko does not move an axial response", refs["c1_timoshenko"]["cases"] == {k: v for k, v in c1.items()})
    check("C-1: the tie rods move the response", separated(c1["load:PRESSURE"], refs["c1_tied"]["cases"]["load:PRESSURE"]) > 10 ** 6)
    check("C-1 LS: the moved anchor closes the guide under pressure", refs["c1_ls"]["cases"]["load:PRESSURE"]["state"] == "active")
    # Perturb the independently assembled ledger, one joint term at a time.
    base = refs["c1_linear"]["cases"]["load:PRESSURE"]
    uc = Fr(base["displacements"]["node:C"][0]["exact"])
    for mutant, caps, effective in (("face caps dropped", 0, 1), ("face caps doubled", 2, 1),
                                  ("effective area dropped", 1, 0)):
        u, _ = c1_ledger(C1_CASES["load:PRESSURE"], C1["k_joint"], None, None,
                         face_caps=caps, effective=effective)
        check(f"C-1 ledger mutant ({mutant}) moves u_C by more than 10^6 criteria",
              abs(u[2] - uc) > Fr(1, 1000) * abs(uc))
    return requests, refs


# ---------------------------------------------------------------------------

def dumps(obj) -> str:
    return json.dumps(obj, indent=1, sort_keys=True) + "\n"


def main() -> int:
    write = "--write" in sys.argv[1:]
    elongation = PE.argv_mode(sys.argv[1:])
    try:
        ls_requests, ls_refs = build_ls()
        c1_requests, c1_refs = build_c1()
    except Fail as error:
        print(f"FAIL: {error}", file=sys.stderr)
        return 1
    files = {}
    for name, request in {**ls_requests, **c1_requests}.items():
        files[f"requests/{name}.json"] = dumps(PE.apply_to_container(request, elongation))
    refs = {**ls_refs, **c1_refs}
    for key, out in refs.items():
        out["request_sha256"] = hashlib.sha256(files[out["request"]].encode()).hexdigest()
    document = {
        "document": "RT-A3 S6 references (DEL-04-07 route-unification-r4.md §3.7): C-1 and LS-1",
        "status": "frozen analytical candidate; product agreement and independent scrutiny are separate",
        "units": "SI: m, rad, N, N*m, Pa",
        "inputs": {
            "elastic_modulus_pa": num(E), "poisson_ratio": num(NU), "shear_modulus_pa": num(G),
            "section_leaves": {"As_m2": num(A_S), "I_m4": num(I_S), "J_m4": num(J_S), "kappa_E": num(KAPPA), "A_shear_m2": num(A_SH),
                               "A_i_m2": num(A_I), "source": "W-16 normalized_section leaves (binary64); A_i = fl(pi/4*0.184^2)"},
            "c1": {k: num(v) for k, v in C1.items() if not isinstance(v, list)},
        },
        "reference_object": "the exact rational response of the system formed from the binary64 leaves above; the exact route forms its section constants by D-5's sequence, which may differ in the last place (k1-twin-bound.md), far inside the criterion",
        "criterion": "each published value within 1e-9*|expected|, or 1e-9 of the case's family zero scale (the largest expected magnitude of its family) when the expected value is 0",
        "requests": refs,
        "crosscheck": CHECKS,
    }
    files["references.json"] = dumps(document)
    readme = (HERE / "README.md").read_text() if (HERE / "README.md").exists() else ""
    gen = pathlib.Path(__file__).read_text()
    sums_files = dict(files)
    sums_files["README.md"] = readme
    sums_files["generators/rt_a3_s6_reference.py"] = gen
    files["SHA256SUMS"] = "".join(f"{hashlib.sha256(text.encode()).hexdigest()}  {path}\n" for path, text in sorted(sums_files.items()))
    if write:
        for path, text in files.items():
            target = HERE / path
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(text)
        print(f"wrote {len(files)} files; {len(CHECKS)} checks agree")
        return 0
    stale = [path for path, text in files.items() if not (HERE / path).exists() or (HERE / path).read_text() != text]
    if stale:
        print("STALE: " + ", ".join(stale), file=sys.stderr)
        return 1
    print(f"OK: {len(files)} files byte-identical; {len(CHECKS)} checks agree")
    return 0


if __name__ == "__main__":
    sys.exit(main())
