#!/usr/bin/env python3
"""Independent pressure-free 3D Euler-Bernoulli frame reference.

This script intentionally reads only the input fixture.  It does not import
OpenPipeStress production code or expected-result constants.
"""

from __future__ import annotations

import hashlib
import json
import math
from pathlib import Path

import numpy as np


HERE = Path(__file__).resolve().parent
REPO_ROOT = next(p for p in HERE.parents if (p / ".git").exists() or (p / ".git").is_file())
FIXTURE = REPO_ROOT / "projects/chirality-piping/fixtures/product_preview/invented_preview_model.json"
OUT = HERE / "EXPECTED_RESULTS.json"

NODE_ORDER = ["node:N-100", "node:N-110", "node:N-120", "node:N-130", "node:N-140"]
DOF_ORDER = ["UX", "UY", "UZ", "RX", "RY", "RZ"]
ELEMENT_ORDER = ["pipe:P-100", "pipe:P-110", "pipe:P-120", "pipe:P-130"]


def clean(x: float) -> float:
    return 0.0 if abs(float(x)) < 5.0e-13 else float(x)


def vector6(x: np.ndarray) -> dict[str, float]:
    return {d: clean(v) for d, v in zip(DOF_ORDER, x)}


def node_map(x: np.ndarray) -> dict[str, dict[str, float]]:
    return {n: vector6(x[6 * i : 6 * i + 6]) for i, n in enumerate(NODE_ORDER)}


def local_stiffness(E: float, G: float, A: float, Iy: float, Iz: float, J: float, L: float) -> np.ndarray:
    k = np.zeros((12, 12))

    def add(indices, block):
        k[np.ix_(indices, indices)] += np.asarray(block, dtype=float)

    add([0, 6], E * A / L * np.array([[1, -1], [-1, 1]]))
    add([3, 9], G * J / L * np.array([[1, -1], [-1, 1]]))
    add(
        [1, 5, 7, 11],
        E * Iz / L**3
        * np.array(
            [
                [12, 6 * L, -12, 6 * L],
                [6 * L, 4 * L**2, -6 * L, 2 * L**2],
                [-12, -6 * L, 12, -6 * L],
                [6 * L, 2 * L**2, -6 * L, 4 * L**2],
            ]
        ),
    )
    add(
        [2, 4, 8, 10],
        E * Iy / L**3
        * np.array(
            [
                [12, -6 * L, -12, -6 * L],
                [-6 * L, 4 * L**2, 6 * L, 2 * L**2],
                [-12, 6 * L, 12, 6 * L],
                [-6 * L, 2 * L**2, 6 * L, 4 * L**2],
            ]
        ),
    )
    return k


def triad(xi: np.ndarray, xj: np.ndarray, yref: np.ndarray) -> tuple[np.ndarray, float]:
    dx = xj - xi
    L = np.linalg.norm(dx)
    ex = dx / L
    ey0 = yref - np.dot(yref, ex) * ex
    if np.linalg.norm(ey0) == 0.0:
        raise ValueError("y_reference is parallel to the element axis")
    ey = ey0 / np.linalg.norm(ey0)
    ez = np.cross(ex, ey)
    ez /= np.linalg.norm(ez)
    R = np.vstack([ex, ey, ez])  # local components = R @ global components
    return R, float(L)


def transform(R: np.ndarray) -> np.ndarray:
    T = np.zeros((12, 12))
    for start in (0, 3, 6, 9):
        T[start : start + 3, start : start + 3] = R
    return T


def consistent_uniform(q_local: np.ndarray, L: float) -> np.ndarray:
    qx, qy, qz = q_local
    f = np.zeros(12)
    f[[0, 6]] += qx * L / 2.0
    f[[1, 7]] += qy * L / 2.0
    f[5] += qy * L**2 / 12.0
    f[11] -= qy * L**2 / 12.0
    f[[2, 8]] += qz * L / 2.0
    f[4] -= qz * L**2 / 12.0
    f[10] += qz * L**2 / 12.0
    return f


def thermal_load(E: float, A: float, alpha: float, delta_t: float) -> np.ndarray:
    f = np.zeros(12)
    n = E * A * alpha * delta_t
    f[0], f[6] = -n, n
    return f


def dof(node: str, name: str) -> int:
    return 6 * NODE_ORDER.index(node) + DOF_ORDER.index(name)


def assemble(model: dict) -> tuple[np.ndarray, np.ndarray, dict, dict]:
    nodes = {n["id"]: np.array([n["position"][c] for c in "xyz"], dtype=float) for n in model["nodes"]}
    material = {m["id"]: m for m in model["materials"]}
    K = np.zeros((30, 30))
    elements = {}
    section_values = {}
    for p in model["pipe_segments"]:
        i, j = p["from"], p["to"]
        xi, xj = nodes[i], nodes[j]
        R, L = triad(xi, xj, np.array([p["y_reference"][c] for c in "xyz"], dtype=float))
        T = transform(R)
        Do = p["section"]["outside_diameter"]["value"]
        t = p["section"]["wall_thickness"]["value"]
        Di = Do - 2.0 * t
        A = math.pi * (Do**2 - Di**2) / 4.0
        I = math.pi * (Do**4 - Di**4) / 64.0
        J = 2.0 * I
        mat = material[p["material"]]
        E = mat["elastic_modulus"]["value"]
        G = mat["shear_modulus"]["value"]
        alpha = mat["thermal_expansion_coefficient"]["value"]
        kl = local_stiffness(E, G, A, I, I, J, L)
        kg = T.T @ kl @ T
        ids = list(range(6 * NODE_ORDER.index(i), 6 * NODE_ORDER.index(i) + 6)) + list(
            range(6 * NODE_ORDER.index(j), 6 * NODE_ORDER.index(j) + 6)
        )
        K[np.ix_(ids, ids)] += kg
        elements[p["id"]] = {
            "i": i,
            "j": j,
            "xi": xi,
            "xj": xj,
            "L": L,
            "R": R,
            "T": T,
            "kl": kl,
            "ids": ids,
            "E": E,
            "A": A,
            "alpha": alpha,
        }
        section_values[p["id"]] = {"length_m": L, "A_m2": A, "Iy_m4": I, "Iz_m4": I, "J_m4": J}
    return K, nodes, elements, section_values


def pressure_free_case(model: dict, case_id: str, elements: dict) -> tuple[np.ndarray, dict, list]:
    F = np.zeros(30)
    element_loads = {e: {"q_local": np.zeros(3), "f_local": np.zeros(12), "delta_t_C": 0.0} for e in ELEMENT_ORDER}
    physical_loads = []
    case = next(c for c in model["load_cases"] if c["id"] == case_id)
    for load in case["primitive_loads"]:
        if load["category"] == "pressure":
            continue
        mag = float(load["magnitude"]["value"])
        direction = load["direction"]
        axis = {"global_x": 0, "global_y": 1, "global_z": 2}[direction]
        if load["dimension"] == "force":
            node = load["target"]["node"]
            F[dof(node, DOF_ORDER[axis])] += mag
            physical_loads.append({"kind": "point_force", "node": node, "force_global_N": [mag if a == axis else 0.0 for a in range(3)]})
        elif load["dimension"] == "force_per_length":
            eid = load["target"]["pipe"]
            e = elements[eid]
            qg = np.array([mag if a == axis else 0.0 for a in range(3)])
            ql = e["R"] @ qg
            fl = consistent_uniform(ql, e["L"])
            fg = e["T"].T @ fl
            F[e["ids"]] += fg
            element_loads[eid]["q_local"] += ql
            element_loads[eid]["f_local"] += fl
            physical_loads.append(
                {
                    "kind": "uniform_force",
                    "element": eid,
                    "q_global_N_per_m": qg.tolist(),
                    "length_m": e["L"],
                    "point_global_m": ((e["xi"] + e["xj"]) / 2.0).tolist(),
                }
            )
        elif load["dimension"] == "temperature_interval":
            eid = load["target"]["pipe"]
            e = elements[eid]
            fl = thermal_load(e["E"], e["A"], e["alpha"], mag)
            F[e["ids"]] += e["T"].T @ fl
            element_loads[eid]["f_local"] += fl
            element_loads[eid]["delta_t_C"] += mag
        else:
            raise ValueError(f"unsupported pressure-free primitive {load['id']}")
    return F, element_loads, physical_loads


def solve_linear(K: np.ndarray, F: np.ndarray, constrained: list[int]) -> np.ndarray:
    free = np.array([i for i in range(len(F)) if i not in set(constrained)], dtype=int)
    u = np.zeros(len(F))
    u[free] = np.linalg.solve(K[np.ix_(free, free)], F[free])
    return u


def force_moment_balance(nodes: dict, physical_loads: list, rigid_actions: dict, spring: dict, friction: dict | None) -> dict:
    force = np.zeros(3)
    moment = np.zeros(3)
    for load in physical_loads:
        if load["kind"] == "point_force":
            f = np.array(load["force_global_N"])
            r = nodes[load["node"]]
        else:
            f = np.array(load["q_global_N_per_m"]) * load["length_m"]
            r = np.array(load["point_global_m"])
        force += f
        moment += np.cross(r, f)
    for node, action in rigid_actions.items():
        a = np.array([action[d] for d in DOF_ORDER])
        force += a[:3]
        moment += np.cross(nodes[node], a[:3]) + a[3:]
    sf = np.array(spring["force_global_N"])
    force += sf
    moment += np.cross(nodes[spring["node"]], sf)
    if friction is not None and friction["mode"] == "sliding":
        ff = np.array(friction["force_global_N"])
        force += ff
        moment += np.cross(nodes[friction["node"]], ff)
    return {"force_residual_global_N": [clean(x) for x in force], "moment_residual_global_Nm_about_origin": [clean(x) for x in moment]}


def recover(elements: dict, element_loads: dict, u: np.ndarray) -> tuple[dict, dict]:
    end = {}
    stations = {}
    ex = np.array([1.0, 0.0, 0.0])
    for eid in ELEMENT_ORDER:
        e = elements[eid]
        dl = e["T"] @ u[e["ids"]]
        a = e["kl"] @ dl - element_loads[eid]["f_local"]
        end[eid] = {"i_equilibrium_end_vector_local": vector6(a[:6]), "j_equilibrium_end_vector_local": vector6(a[6:])}
        q = element_loads[eid]["q_local"]
        rows = []
        for frac in (0.0, 0.25, 0.5, 0.75, 1.0):
            x = frac * e["L"]
            cf = -(a[:3] + q * x)
            cm = -a[3:6] + x * np.cross(ex, a[:3]) + 0.5 * x**2 * np.cross(ex, q)
            rows.append({"x_over_L": frac, "cut_action_on_left_segment_local": vector6(np.r_[cf, cm])})
        stations[eid] = rows
    return end, stations


def constrained_dofs(one_way_active: bool, stick: bool) -> tuple[list[int], dict[int, tuple[str, str]]]:
    constraints = []
    owner = {}

    def add(node, names, support):
        for name in names:
            idx = dof(node, name)
            constraints.append(idx)
            owner[idx] = (support, node)

    add("node:N-100", DOF_ORDER, "support:S-100")
    add("node:N-120", ["UX", "UZ"], "support:S-120")
    add("node:N-130", ["UY"], "support:S-130")
    if one_way_active:
        add("node:N-140", ["UY"], "support:NL-140")
    if stick:
        add("node:N-130", ["UZ"], "support:NL-130-FRIC")
    return sorted(constraints), owner


def solve_branch(Kstruct, Ktotal, F, nodes, elements, element_loads, physical_loads, one_way_active, mode, mu, friction_force=0.0):
    stick = mode == "sticking"
    constrained, owner = constrained_dofs(one_way_active, stick)
    Fsolve = F.copy()
    if mode == "sliding":
        Fsolve[dof("node:N-130", "UZ")] += friction_force
    u = solve_linear(Ktotal, Fsolve, constrained)
    residual = Ktotal @ u - Fsolve
    actions_by_support = {}
    for idx in constrained:
        support, node = owner[idx]
        actions_by_support.setdefault(support, {"node": node, "vector": np.zeros(6)})
        actions_by_support[support]["vector"][idx % 6] += residual[idx]
    rigid_actions_by_node = {}
    for support, item in actions_by_support.items():
        node = item["node"]
        rigid_actions_by_node.setdefault(node, np.zeros(6))
        rigid_actions_by_node[node] += item["vector"]
    spring_u = u[dof("node:N-140", "UZ")]
    spring_force = -42000.0 * spring_u
    spring = {"node": "node:N-140", "dof": "UZ", "zero_reference": True, "force_global_N": [0.0, 0.0, clean(spring_force)]}
    normal_reaction = actions_by_support["support:S-130"]["vector"][1]
    if stick:
        friction_reaction = actions_by_support["support:NL-130-FRIC"]["vector"][2]
        friction = {
            "mode": "sticking",
            "node": "node:N-130",
            "force_global_N": [0.0, 0.0, clean(friction_reaction)],
            "normal_reaction_S130_UY_N": clean(normal_reaction),
            "normal_magnitude_N": clean(abs(normal_reaction)),
            "mu_N_N": clean(mu * abs(normal_reaction)),
            "stick_margin_muN_minus_abs_Rf_N": clean(mu * abs(normal_reaction) - abs(friction_reaction)),
            "admissible": bool(abs(friction_reaction) <= mu * abs(normal_reaction)),
        }
    elif mode == "sliding":
        friction = {
            "mode": "sliding",
            "node": "node:N-130",
            "force_global_N": [0.0, 0.0, clean(friction_force)],
            "normal_reaction_S130_UY_N": clean(normal_reaction),
            "normal_magnitude_N": clean(abs(normal_reaction)),
            "mu_N_N": clean(mu * abs(normal_reaction)),
            "abs_Rf_minus_muN_N": clean(abs(friction_force) - mu * abs(normal_reaction)),
            "slip_displacement_UZ_m": clean(u[dof("node:N-130", "UZ")]),
        }
    else:
        friction = None
    contact_reaction = actions_by_support.get("support:NL-140", {"vector": np.zeros(6)})["vector"][1]
    contact = {
        "assumed_state": "active" if one_way_active else "open",
        "UY_m": clean(u[dof("node:N-140", "UY")]),
        "reaction_UY_N": clean(contact_reaction),
        "fixture_active_when_negative_reaction": True,
        "active_sign_admissible": bool(contact_reaction <= 0.0) if one_way_active else None,
        "open_gap_admissible_under_block_positive_UY_interpretation": bool(u[dof("node:N-140", "UY")] <= 0.0) if not one_way_active else None,
    }
    end, stations = recover(elements, element_loads, u)
    support_actions = {support: {"node": item["node"], **vector6(item["vector"])} for support, item in actions_by_support.items()}
    balance = force_moment_balance(nodes, physical_loads, {n: vector6(v) for n, v in rigid_actions_by_node.items()}, spring, friction)
    free = [i for i in range(30) if i not in set(constrained)]
    balance["max_abs_free_equilibrium_residual_N_or_Nm"] = clean(np.max(np.abs(residual[free])))
    return {
        "nodal_displacements_global": node_map(u),
        "support_actions_on_structure_global": support_actions,
        "spring_action": spring,
        "friction": friction,
        "one_way_contact": contact,
        "element_end_equilibrium_vectors_local": end,
        "station_cut_vectors_local": stations,
        "independent_balance": balance,
    }, u, normal_reaction


def sliding_candidates(Kstruct, Ktotal, F, nodes, elements, element_loads, physical_loads, one_way_active, mu):
    # Rn(f)=a+b*f.  Enumerate assumed slip sign s and reaction sign rho,
    # solve f=-mu*s*rho*(a+b*f) exactly, then check every assumption.
    zero, _, a = solve_branch(Kstruct, Ktotal, F, nodes, elements, element_loads, physical_loads, one_way_active, "sliding", mu, 0.0)
    unit, _, r1 = solve_branch(Kstruct, Ktotal, F, nodes, elements, element_loads, physical_loads, one_way_active, "sliding", mu, 1.0)
    b = r1 - a
    candidates = []
    for s in (-1.0, 1.0):
        for rho in (-1.0, 1.0):
            denom = 1.0 + mu * s * rho * b
            f = -mu * s * rho * a / denom
            result, u, rn = solve_branch(Kstruct, Ktotal, F, nodes, elements, element_loads, physical_loads, one_way_active, "sliding", mu, f)
            uz = u[dof("node:N-130", "UZ")]
            checks = {
                "assumed_slip_sign": int(s),
                "assumed_normal_reaction_sign": int(rho),
                "slip_sign_matches": bool(s * uz > 0.0),
                "normal_sign_matches": bool(rho * rn >= 0.0),
                "opposes_slip": bool(f * uz < 0.0),
                "coulomb_equality_residual_N": clean(abs(f) - mu * abs(rn)),
                "contact_active_sign_admissible": result["one_way_contact"]["active_sign_admissible"],
                "contact_open_gap_admissible": result["one_way_contact"]["open_gap_admissible_under_block_positive_UY_interpretation"],
            }
            checks["all_branch_conditions"] = bool(
                checks["slip_sign_matches"]
                and checks["normal_sign_matches"]
                and checks["opposes_slip"]
                and (checks["contact_active_sign_admissible"] is not False)
                and (checks["contact_open_gap_admissible"] is not False)
            )
            result["branch_checks"] = checks
            candidates.append(result)
    return candidates


def main() -> None:
    model = json.loads(FIXTURE.read_text())
    Kstruct, nodes, elements, section_values = assemble(model)
    Ktotal = Kstruct.copy()
    Ktotal[dof("node:N-140", "UZ"), dof("node:N-140", "UZ")] += 42000.0
    mu = 0.01
    cases = {}
    for cid in ("load:L-100", "load:L-200"):
        F, element_loads, physical_loads = pressure_free_case(model, cid, elements)
        active_free, _, _ = solve_branch(Kstruct, Ktotal, F, nodes, elements, element_loads, physical_loads, True, "frictionless", mu)
        open_free, _, _ = solve_branch(Kstruct, Ktotal, F, nodes, elements, element_loads, physical_loads, False, "frictionless", mu)
        active_stick, _, _ = solve_branch(Kstruct, Ktotal, F, nodes, elements, element_loads, physical_loads, True, "sticking", mu)
        open_stick, _, _ = solve_branch(Kstruct, Ktotal, F, nodes, elements, element_loads, physical_loads, False, "sticking", mu)
        active_slides = sliding_candidates(Kstruct, Ktotal, F, nodes, elements, element_loads, physical_loads, True, mu)
        open_slides = sliding_candidates(Kstruct, Ktotal, F, nodes, elements, element_loads, physical_loads, False, mu)
        valid = [x for x in active_slides + open_slides if x["branch_checks"]["all_branch_conditions"]]
        cases[cid] = {
            "pressure_primitives_included": False,
            "thermal_included": cid == "load:L-100",
            "component_user_stiffness_included": False,
            "spring_zero_reference_included": True,
            "frictionless_controls": {"one_way_active": active_free, "one_way_open": open_free},
            "stick_candidates": {"one_way_active": active_stick, "one_way_open": open_stick},
            "sliding_candidates": {"one_way_active": active_slides, "one_way_open": open_slides},
            "admissible_current_normal_zero_reference_branches": valid,
        }

    data = {
        "schema": "chirality.m9-c1-independent-reference/v1",
        "status": "DERIVATIVE_CANDIDATE_NOT_ACCEPTED_AUTHORITY",
        "fixture": {
            "path": str(FIXTURE.relative_to(REPO_ROOT)),
            "sha256": hashlib.sha256(FIXTURE.read_bytes()).hexdigest(),
        },
        "basis": {
            "coordinates": "global right-handed X,Y,Z; radians; N,m,Pa",
            "nodal_dof_order": DOF_ORDER,
            "node_order": NODE_ORDER,
            "element_order": ELEMENT_ORDER,
            "local_axes": {eid: {"ex_global": e["R"][0].tolist(), "ey_global": e["R"][1].tolist(), "ez_global": e["R"][2].tolist()} for eid, e in elements.items()},
            "section_values": section_values,
            "model_assumptions": [
                "small-displacement prismatic 3D Euler-Bernoulli frame",
                "circular section uses Iy=Iz and J=2I",
                "linear spring SH-140 acts at N-140 UZ against displacement from zero reference",
                "CE-120 and SH-140 installed/cold/hot loads are review metadata and not applied loads",
                "component C-110/C-120/C-130 modifiers and C-130 weight are not primitive mechanics loads/stiffness in this branch",
                "C-150 user flexibility is excluded because the fixture gives no second node or insertion/replacement rule",
                "pressure primitives and expansion-joint pressure thrust are excluded",
                "normal magnitude for Coulomb bound is abs(returned S-130 UY support action)",
                "static zero-slip reference branch; no history state is inferred",
            ],
        },
        "frictionless_scalar_controls": {
            "P120_cantilever_under_L100_uniform_Z": {
                "source_values": {"q_local_y_N_per_m": -190.0, "L_m": elements["pipe:P-120"]["L"]},
                "consistent_nodal_load_local": vector6(consistent_uniform(np.array([0.0, -190.0, 0.0]), elements["pipe:P-120"]["L"])[:6])
                | {"j_" + k: v for k, v in vector6(consistent_uniform(np.array([0.0, -190.0, 0.0]), elements["pipe:P-120"]["L"])[6:]).items()},
                "fixed_end_support_action": {
                    "UY_local_N": clean(190.0 * elements["pipe:P-120"]["L"]),
                    "RZ_local_Nm": clean(190.0 * elements["pipe:P-120"]["L"] ** 2 / 2.0),
                },
                "free_end_displacement": {
                    "UY_local_m": clean(-190.0 * elements["pipe:P-120"]["L"] ** 4 / (8.0 * elements["pipe:P-120"]["E"] * (section_values["pipe:P-120"]["Iy_m4"]))),
                    "RZ_local_rad": clean(-190.0 * elements["pipe:P-120"]["L"] ** 3 / (6.0 * elements["pipe:P-120"]["E"] * (section_values["pipe:P-120"]["Iy_m4"]))),
                },
            },
            "SH140_scalar_zero_reference_under_P120_L100_resultant": {
                "applied_UZ_N": clean(-190.0 * elements["pipe:P-120"]["L"]),
                "displacement_UZ_m": clean(-190.0 * elements["pipe:P-120"]["L"] / 42000.0),
                "spring_action_UZ_N": clean(190.0 * elements["pipe:P-120"]["L"]),
            },
        },
        "cases": cases,
        "unresolved_unique_actual_composition_blockers": [
            {
                "id": "B-COMPONENT-TOPOLOGY",
                "fields": ["component:C-150.geometry.expansion_joint_pipe_ref", "component:C-150.modifiers.*"],
                "missing_choice": "whether the six user stiffnesses replace pipe:P-130, act in series through a new internal node/rigid offsets, or act in parallel, plus the connector local-axis and length/reference definition",
                "effect": "no unique component-stiffness-inclusive global matrix or signed result vector can be derived",
            },
            {
                "id": "B-SPRING-REFERENCE",
                "fields": ["support:SH-140.hanger.installed_load", "cold_load", "hot_load", "mechanics_consumption"],
                "missing_choice": "spring force-free/reference displacement and whether any listed load is an applied preload",
                "effect": "only the explicitly labelled zero-reference stiffness-only branch is unique",
            },
            {
                "id": "B-FRICTION-HISTORY",
                "fields": ["support:NL-130-FRIC.initial_state"],
                "missing_choice": "committed slip reference/load-step history versus static zero-reference interpretation",
                "effect": "the current-normal zero-reference branch is calculated; path-dependent results remain undefined",
            },
            {
                "id": "B-ONE-WAY-GAP-SIDE",
                "fields": ["support:NL-140.nonlinear.active_when"],
                "missing_choice": "explicit signed gap function/allowed displacement side",
                "effect": "active reaction sign can be checked directly; open-branch gap feasibility is reported under the stated block-positive-UY interpretation",
            },
        ],
    }
    OUT.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
