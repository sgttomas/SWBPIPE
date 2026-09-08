#!/usr/bin/env python3
"""Independent small-displacement reference for the invented product fixture.

The calculation reads only authored model inputs.  It does not import Chirality
solver crates, production output, or bundled expected-result constants.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import math
from pathlib import Path
from typing import Any

import numpy as np

DOFS = ("UX", "UY", "UZ", "RX", "RY", "RZ")
TRANSLATION = slice(0, 3)
ROTATION = slice(3, 6)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def unit(vector: np.ndarray) -> np.ndarray:
    length = float(np.linalg.norm(vector))
    if not math.isfinite(length) or length <= 0.0:
        raise ValueError("zero/nonfinite direction")
    return vector / length


def axes(point_i: np.ndarray, point_j: np.ndarray, y_reference: np.ndarray) -> tuple[float, np.ndarray]:
    """Return L and R whose rows are the local x,y,z axes in global components."""
    local_x = unit(point_j - point_i)
    projected_y = y_reference - float(y_reference @ local_x) * local_x
    local_y = unit(projected_y)
    local_z = unit(np.cross(local_x, local_y))
    # Re-orthogonalize so the triad is explicitly right handed.
    local_y = unit(np.cross(local_z, local_x))
    rotation = np.vstack((local_x, local_y, local_z))
    return float(np.linalg.norm(point_j - point_i)), rotation


def transformation(rotation: np.ndarray) -> np.ndarray:
    transform = np.zeros((12, 12), dtype=float)
    for start in (0, 3, 6, 9):
        transform[start : start + 3, start : start + 3] = rotation
    return transform


def beam_local_stiffness(E: float, G: float, A: float, Iy: float, Iz: float, J: float, L: float) -> np.ndarray:
    """Conventional 3D Euler-Bernoulli beam, local order [u,v,w,rx,ry,rz] at i,j."""
    k = np.zeros((12, 12), dtype=float)

    def put(indices: tuple[int, ...], block: np.ndarray) -> None:
        for row, gi in enumerate(indices):
            for col, gj in enumerate(indices):
                k[gi, gj] = block[row, col]

    put((0, 6), E * A / L * np.array([[1.0, -1.0], [-1.0, 1.0]]))
    put((3, 9), G * J / L * np.array([[1.0, -1.0], [-1.0, 1.0]]))
    put(
        (1, 5, 7, 11),
        E * Iz
        * np.array(
            [
                [12 / L**3, 6 / L**2, -12 / L**3, 6 / L**2],
                [6 / L**2, 4 / L, -6 / L**2, 2 / L],
                [-12 / L**3, -6 / L**2, 12 / L**3, -6 / L**2],
                [6 / L**2, 2 / L, -6 / L**2, 4 / L],
            ]
        ),
    )
    put(
        (2, 4, 8, 10),
        E * Iy
        * np.array(
            [
                [12 / L**3, -6 / L**2, -12 / L**3, -6 / L**2],
                [-6 / L**2, 4 / L, 6 / L**2, 2 / L],
                [-12 / L**3, 6 / L**2, 12 / L**3, 6 / L**2],
                [-6 / L**2, 2 / L, 6 / L**2, 4 / L],
            ]
        ),
    )
    return k


def consistent_uniform_local(q_local: np.ndarray, L: float) -> np.ndarray:
    """Consistent nodal work vector for constant local force/length qx,qy,qz."""
    qx, qy, qz = [float(value) for value in q_local]
    f = np.zeros(12, dtype=float)
    f[[0, 6]] = qx * L / 2.0
    f[[1, 7]] = qy * L / 2.0
    f[5] = qy * L**2 / 12.0
    f[11] = -qy * L**2 / 12.0
    f[[2, 8]] = qz * L / 2.0
    f[4] = -qz * L**2 / 12.0
    f[10] = qz * L**2 / 12.0
    return f


def thermal_local(E: float, A: float, alpha: float, delta_temperature: float) -> np.ndarray:
    f = np.zeros(12, dtype=float)
    axial = E * A * alpha * delta_temperature
    f[0], f[6] = -axial, axial
    return f


def dof(node_index: int, name: str) -> int:
    return node_index * 6 + DOFS.index(name.upper())


def element_dofs(i: int, j: int) -> list[int]:
    return list(range(i * 6, i * 6 + 6)) + list(range(j * 6, j * 6 + 6))


def wrench_moment_about_origin(position: np.ndarray, wrench: np.ndarray) -> np.ndarray:
    return np.cross(position, wrench[TRANSLATION]) + wrench[ROTATION]


def solve_constrained(K: np.ndarray, force: np.ndarray, constrained: list[int]) -> tuple[np.ndarray, np.ndarray]:
    fixed = sorted(set(constrained))
    free = [index for index in range(K.shape[0]) if index not in fixed]
    u = np.zeros(K.shape[0], dtype=float)
    u[free] = np.linalg.solve(K[np.ix_(free, free)], force[free])
    residual = K @ u - force
    return u, residual


def clean(values: np.ndarray | list[float], floor: float = 1.0e-14) -> list[float]:
    array = np.asarray(values, dtype=float).copy()
    scale = max(1.0, float(np.max(np.abs(array))) if array.size else 1.0)
    array[np.abs(array) <= floor * scale] = 0.0
    return [float(value) for value in array]


def scalar_control_results() -> dict[str, Any]:
    E, I, L, q = 200.0e9, 1.0e-5, 2.0, -100.0
    udl_tip = q * L**4 / (8.0 * E * I)
    udl_root_force = -q * L
    udl_root_moment = -q * L**2 / 2.0
    spring_k, spring_force = 42_000.0, 350.0
    contact_k, contact_force = 100.0, 10.0
    return {
        "order": ["distributed_load", "retained_spring", "frictionless_unilateral_contact"],
        "distributed_load": {
            "input": {"E_Pa": E, "I_m4": I, "L_m": L, "q_y_N_per_m": q},
            "expected": {
                "tip_y_m": udl_tip,
                "root_reaction_y_N": udl_root_force,
                "root_reaction_mz_N_m": udl_root_moment,
            },
            "identities": ["v(L)=qL^4/(8EI)", "Ry=-qL", "Mz=-qL^2/2"],
        },
        "retained_spring": {
            "input": {"k_N_per_m": spring_k, "force_N": spring_force},
            "expected": {"displacement_m": spring_force / spring_k, "spring_action_N": -spring_force},
        },
        "frictionless_unilateral_contact": {
            "input": {"k_N_per_m": contact_k, "force_N": contact_force, "active_when": "negative_reaction"},
            "expected_active_branch": {"displacement_m": 0.0, "reaction_N": -contact_force, "lambda_N": contact_force},
            "complementarity": {"gap_definition": "g=-u >= 0", "lambda_definition": "lambda=-R >= 0", "lambda_times_g": 0.0},
        },
    }


def build_fixture(model: dict[str, Any], load_case_id: str) -> dict[str, Any]:
    nodes = model["nodes"]
    node_index = {node["id"].split(":", 1)[-1]: index for index, node in enumerate(nodes)}
    positions = np.array([[node["position"][axis] for axis in ("x", "y", "z")] for node in nodes], dtype=float)
    material = model["materials"][0]
    E = float(material["elastic_modulus"]["value"])
    G = float(material["shear_modulus"]["value"])
    alpha = float(material["thermal_expansion_coefficient"]["value"])
    total_dofs = len(nodes) * 6
    K_elements = np.zeros((total_dofs, total_dofs), dtype=float)
    F = np.zeros(total_dofs, dtype=float)
    elements: list[dict[str, Any]] = []

    load_case = next(case for case in model["load_cases"] if case["id"] == load_case_id)
    pressure_omissions = [load["id"] for load in load_case["primitive_loads"] if load["dimension"] == "pressure"]

    for pipe in model["pipe_segments"]:
        i = node_index[pipe["from"].split(":", 1)[-1]]
        j = node_index[pipe["to"].split(":", 1)[-1]]
        diameter = float(pipe["section"]["outside_diameter"]["value"])
        wall = float(pipe["section"]["wall_thickness"]["value"])
        inner = diameter - 2.0 * wall
        A = math.pi * (diameter**2 - inner**2) / 4.0
        I = math.pi * (diameter**4 - inner**4) / 64.0
        J = 2.0 * I
        L, R = axes(
            positions[i],
            positions[j],
            np.array([pipe["y_reference"][axis] for axis in ("x", "y", "z")], dtype=float),
        )
        T = transformation(R)
        k_local = beam_local_stiffness(E, G, A, I, I, J, L)
        k_global = T.T @ k_local @ T
        indices = element_dofs(i, j)
        K_elements[np.ix_(indices, indices)] += k_global

        f_dist_local = np.zeros(12, dtype=float)
        f_thermal_local = np.zeros(12, dtype=float)
        q_global = np.zeros(3, dtype=float)
        for load in load_case["primitive_loads"]:
            target = load["target"]
            if load["dimension"] == "pressure":
                continue
            if target["type"] != "element" or target["pipe"] != pipe["id"]:
                continue
            if load["dimension"] == "force_per_length":
                axis = {"global_x": 0, "global_y": 1, "global_z": 2}[load["direction"]]
                q_global[axis] += float(load["magnitude"]["value"])
            elif load["dimension"] in ("temperature_change", "temperature_interval"):
                f_thermal_local += thermal_local(E, A, alpha, float(load["magnitude"]["value"]))
        f_dist_local += consistent_uniform_local(R @ q_global, L)
        F[indices] += T.T @ (f_dist_local + f_thermal_local)
        elements.append(
            {
                "id": pipe["id"].split(":", 1)[-1],
                "i": i,
                "j": j,
                "L": L,
                "R": R,
                "T": T,
                "k_local": k_local,
                "f_dist_local": f_dist_local,
                "f_thermal_local": f_thermal_local,
                "q_local": R @ q_global,
                "section": {"A_m2": A, "Iy_m4": I, "Iz_m4": I, "J_m4": J},
            }
        )

    for load in load_case["primitive_loads"]:
        if load["dimension"] == "pressure" or load["target"]["type"] != "node":
            continue
        if load["dimension"] == "force":
            node = node_index[load["target"]["node"].split(":", 1)[-1]]
            axis = {"global_x": "UX", "global_y": "UY", "global_z": "UZ"}[load["direction"]]
            F[dof(node, axis)] += float(load["magnitude"]["value"])
        elif load["dimension"] == "moment":
            node = node_index[load["target"]["node"].split(":", 1)[-1]]
            axis = {"global_x": "RX", "global_y": "RY", "global_z": "RZ"}[load["direction"]]
            F[dof(node, axis)] += float(load["magnitude"]["value"])

    K_beams = K_elements.copy()

    # Exact authored expansion-joint user stiffness: relative local DOFs in
    # parallel with P-130. It is input stiffness, not a product-derived oracle.
    expansion = next(component for component in model["components"] if component["id"] == "component:C-150")
    pipe_ref = expansion["geometry"]["expansion_joint_pipe_ref"]
    pipe_element = next(element for element in elements if f"pipe:{element['id']}" == pipe_ref)
    modifiers = expansion["modifiers"]
    local_values = np.array(
        [
            modifiers["axial_stiffness_user_value"]["value"],
            modifiers["lateral_stiffness_user_value"]["value"],
            modifiers["lateral_stiffness_user_value"]["value"],
            modifiers["torsional_stiffness_user_value"]["value"],
            modifiers["angular_stiffness_user_value"]["value"],
            modifiers["angular_stiffness_user_value"]["value"],
        ],
        dtype=float,
    )
    k_user_local = np.block([[np.diag(local_values), -np.diag(local_values)], [-np.diag(local_values), np.diag(local_values)]])
    k_user_global = pipe_element["T"].T @ k_user_local @ pipe_element["T"]
    user_indices = element_dofs(pipe_element["i"], pipe_element["j"])
    K_elements[np.ix_(user_indices, user_indices)] += k_user_global

    K = K_elements.copy()
    spring = next(item for item in model["supports"] if item["id"] == "support:SH-140")
    spring_node = node_index[spring["node"].split(":", 1)[-1]]
    spring_dof = dof(spring_node, spring["hanger"]["stiffness"]["dof"])
    spring_k = float(spring["hanger"]["stiffness"]["value"]["value"])
    K[spring_dof, spring_dof] += spring_k

    base_constraints: list[int] = []
    support_dofs: dict[str, list[int]] = {}
    for support in model["supports"]:
        if support.get("nonlinear") is not None or support["id"] in ("support:SH-140", "support:CE-120"):
            continue
        node = node_index[support["node"].split(":", 1)[-1]]
        indices = [dof(node, item) for item in support["restraints"]]
        support_dofs[support["id"].split(":", 1)[-1]] = indices
        base_constraints.extend(indices)

    contact = next(item for item in model["supports"] if item["id"] == "support:NL-140")
    contact_node = node_index[contact["node"].split(":", 1)[-1]]
    contact_dof = dof(contact_node, contact["nonlinear"]["dof"])
    friction = next(item for item in model["supports"] if item["id"] == "support:NL-130-FRIC")
    friction_node = node_index[friction["node"].split(":", 1)[-1]]
    friction_dof = dof(friction_node, friction["nonlinear"]["dof"])
    normal_support = friction["nonlinear"]["normal_reaction_source"]["support_ref"].split(":", 1)[-1]
    normal_dof_name = friction["nonlinear"]["normal_reaction_source"]["dof"]
    normal_dof = dof(friction_node, normal_dof_name)

    return {
        "model": model,
        "positions": positions,
        "node_index": node_index,
        "K": K,
        "K_elements": K_elements,
        "K_beams": K_beams,
        "F": F,
        "elements": elements,
        "pressure_omissions": pressure_omissions,
        "base_constraints": base_constraints,
        "support_dofs": support_dofs,
        "contact_dof": contact_dof,
        "friction_dof": friction_dof,
        "normal_dof": normal_dof,
        "normal_support": normal_support,
        "mu": float(friction["nonlinear"]["friction_coefficient"]["value"]),
        "spring_dof": spring_dof,
        "spring_k": spring_k,
        "material": {"E_Pa": E, "G_Pa": G, "alpha_per_K": alpha},
        "expansion_joint_local_stiffness": clean(local_values),
        "expansion_joint_global_stiffness": k_user_global,
        "expansion_joint_indices": user_indices,
        "connector_enabled": True,
    }


def contact_admissibility(active: bool, displacement: float, reaction: float) -> dict[str, Any]:
    # Fixture active_when=negative_reaction.  Therefore g=-u and lambda=-R.
    gap = -displacement
    multiplier = -reaction if active else 0.0
    return {
        "state": "active" if active else "inactive",
        "displacement_u_m": displacement,
        "reaction_R_N": reaction if active else 0.0,
        "gap_g_minus_u_m": gap,
        "lambda_minus_R_N": multiplier,
        "gap_feasible": gap >= -1.0e-11,
        "multiplier_feasible": multiplier >= -1.0e-7,
        "complementarity_lambda_g_N_m": multiplier * gap,
        "admissible": gap >= -1.0e-11 and multiplier >= -1.0e-7 and abs(multiplier * gap) <= 1.0e-7,
    }


def solve_branch(data: dict[str, Any], contact_active: bool, friction_state: str, slip_sign: int | None = None) -> dict[str, Any]:
    K, base_force = data["K"], data["F"]
    constraints = list(data["base_constraints"])
    if contact_active:
        constraints.append(data["contact_dof"])
    force = base_force.copy()
    friction_force = 0.0

    if friction_state == "stick":
        constraints.append(data["friction_dof"])
        u, residual = solve_constrained(K, force, constraints)
        friction_force = float(residual[data["friction_dof"]])
    elif friction_state == "none":
        u, residual = solve_constrained(K, force, constraints)
    elif friction_state == "slip":
        assert slip_sign in (-1, 1)
        # Linearity gives Rn(t)=a+b*t. Solve t=-mu*|Rn(t)|*sign(u_t)
        u0, r0 = solve_constrained(K, force, constraints)
        unit_force = force.copy()
        unit_force[data["friction_dof"]] += 1.0
        u1_total, r1_total = solve_constrained(K, unit_force, constraints)
        response_u = u1_total - u0
        response_r = r1_total - r0
        candidates: list[tuple[float, np.ndarray, np.ndarray]] = []
        for normal_sign in (-1, 1):
            a = float(r0[data["normal_dof"]])
            b = float(response_r[data["normal_dof"]])
            denominator = 1.0 + data["mu"] * slip_sign * normal_sign * b
            if abs(denominator) <= 1.0e-14:
                continue
            trial_t = -data["mu"] * slip_sign * normal_sign * a / denominator
            trial_force = force.copy()
            trial_force[data["friction_dof"]] += trial_t
            trial_u, trial_r = solve_constrained(K, trial_force, constraints)
            rn = float(trial_r[data["normal_dof"]])
            if (rn == 0.0 or math.copysign(1.0, rn) == normal_sign) and trial_t * slip_sign <= 1.0e-9:
                candidates.append((trial_t, trial_u, trial_r))
        if len(candidates) != 1:
            raise RuntimeError(f"expected one normal-sign solution, got {len(candidates)}")
        friction_force, u, residual = candidates[0]
        force = force.copy()
        force[data["friction_dof"]] += friction_force
    else:
        raise ValueError(friction_state)

    contact_reaction = float(residual[data["contact_dof"]]) if contact_active else 0.0
    contact = contact_admissibility(contact_active, float(u[data["contact_dof"]]), contact_reaction)
    normal_reaction = float(residual[data["normal_dof"]])
    normal = abs(normal_reaction)
    friction_bound = data["mu"] * normal
    tangential_u = float(u[data["friction_dof"]])
    if friction_state == "stick":
        friction_ok = abs(friction_force) <= friction_bound + 1.0e-7
        relation_error = max(0.0, abs(friction_force) - friction_bound)
    elif friction_state == "slip":
        friction_ok = (
            slip_sign == (1 if tangential_u > 0 else -1 if tangential_u < 0 else 0)
            and abs(abs(friction_force) - friction_bound) <= 1.0e-7
            and friction_force * tangential_u <= 0.0
        )
        relation_error = abs(abs(friction_force) - friction_bound)
    else:
        friction_ok, relation_error = True, 0.0

    return {
        "contact_active": contact_active,
        "friction_state": friction_state,
        "slip_sign": slip_sign,
        "u": u,
        "residual": residual,
        "force_rhs": force,
        "friction_force": friction_force,
        "normal_reaction": normal_reaction,
        "normal_magnitude": normal,
        "friction_bound": friction_bound,
        "contact": contact,
        "friction_check": {
            "tangential_displacement_m": tangential_u,
            "tangential_action_N": friction_force,
            "current_normal_reaction_N": normal_reaction,
            "current_normal_magnitude_N": normal,
            "mu": data["mu"],
            "mu_times_current_normal_N": friction_bound,
            "relation_error_N": relation_error,
            "opposes_displacement_or_sticks": friction_ok,
        },
        "admissible": contact["admissible"] and friction_ok,
    }


def nodal_vectors(values: np.ndarray, model: dict[str, Any]) -> dict[str, list[float]]:
    return {node["id"].split(":", 1)[-1]: clean(values[index * 6 : index * 6 + 6]) for index, node in enumerate(model["nodes"])}


def branch_results(data: dict[str, Any], branch: dict[str, Any]) -> dict[str, Any]:
    model, positions = data["model"], data["positions"]
    u, residual = branch["u"], branch["residual"]
    constraint_actions = np.zeros_like(u)
    for index in data["base_constraints"]:
        constraint_actions[index] = residual[index]
    if branch["contact_active"]:
        constraint_actions[data["contact_dof"]] = residual[data["contact_dof"]]
    if branch["friction_state"] == "stick":
        constraint_actions[data["friction_dof"]] = residual[data["friction_dof"]]

    spring_physical = np.zeros_like(u)
    spring_physical[data["spring_dof"]] = -data["spring_k"] * u[data["spring_dof"]]
    slip_physical = np.zeros_like(u)
    if branch["friction_state"] == "slip":
        slip_physical[data["friction_dof"]] = branch["friction_force"]

    applied_nodal_equivalent = data["F"]
    external_sum = applied_nodal_equivalent + constraint_actions + spring_physical + slip_physical
    force_balance = np.zeros(3)
    moment_balance = np.zeros(3)
    for node_index in range(len(model["nodes"])):
        wrench = external_sum[node_index * 6 : node_index * 6 + 6]
        force_balance += wrench[TRANSLATION]
        moment_balance += wrench_moment_about_origin(positions[node_index], wrench)

    element_output: dict[str, Any] = {}
    for element in data["elements"]:
        indices = element_dofs(element["i"], element["j"])
        local_u = element["T"] @ u[indices]
        local_end = element["k_local"] @ local_u - element["f_dist_local"] - element["f_thermal_local"]
        stations = []
        p_i_force = local_end[0:3]
        p_i_moment = local_end[3:6]
        ex = np.array([1.0, 0.0, 0.0])
        for fraction in (0.0, 0.25, 0.5, 0.75, 1.0):
            x = fraction * element["L"]
            force_cut = -(p_i_force + element["q_local"] * x)
            moment_cut = -p_i_moment + x * np.cross(ex, p_i_force) + 0.5 * x**2 * np.cross(ex, element["q_local"])
            stations.append({"x_m": x, "fraction": fraction, "local_cut_vector_N_Nm": clean(np.r_[force_cut, moment_cut])})
        end_force_balance = local_end[0:3] + local_end[6:9] + element["q_local"] * element["L"]
        end_moment_balance_about_i = (
            local_end[3:6]
            + local_end[9:12]
            + np.cross(np.array([element["L"], 0.0, 0.0]), local_end[6:9])
            + np.cross(np.array([element["L"] / 2.0, 0.0, 0.0]), element["q_local"] * element["L"])
        )
        element_output[element["id"]] = {
            "length_m": element["L"],
            "section": element["section"],
            "local_axes_rows_global": [clean(row) for row in element["R"]],
            "local_displacement_vector_m_rad": clean(local_u),
            "local_end_vector_N_Nm": clean(local_end),
            "local_distributed_force_N_per_m": clean(element["q_local"]),
            "station_cut_vector_definition": "action on positive-x cut face of left segment; [Nx,Vy,Vz,Mx,My,Mz]",
            "stations": stations,
            "element_force_balance_N": clean(end_force_balance),
            "element_moment_balance_about_i_N_m": clean(end_moment_balance_about_i),
        }

    named_supports = {name: clean(constraint_actions[indices]) for name, indices in data["support_dofs"].items()}
    named_supports["NL-140"] = [float(constraint_actions[data["contact_dof"]])]
    named_supports["NL-130-FRIC"] = [float(branch["friction_force"])]
    named_supports["SH-140"] = [float(spring_physical[data["spring_dof"]])]

    equation_residual = data["K"] @ u - branch["force_rhs"] - constraint_actions
    connector_diagnostic: dict[str, Any]
    if data.get("connector_enabled"):
        indices = data["expansion_joint_indices"]
        connector_wrench = data["expansion_joint_global_stiffness"] @ u[indices]
        connector_force_sum = connector_wrench[0:3] + connector_wrench[6:9]
        connector_moment_sum = (
            wrench_moment_about_origin(positions[data["elements"][-1]["i"]], connector_wrench[0:6])
            + wrench_moment_about_origin(positions[data["elements"][-1]["j"]], connector_wrench[6:12])
        )
        connector_diagnostic = {
            "enabled": True,
            "global_end_vector_N_Nm": clean(connector_wrench),
            "net_force_N": clean(connector_force_sum),
            "net_moment_about_origin_N_m": clean(connector_moment_sum),
        }
    else:
        connector_diagnostic = {"enabled": False}

    return {
        "state": {
            "contact": branch["contact"],
            "friction": branch["friction_check"],
            "admissible": branch["admissible"],
        },
        "nodal_displacements_global": nodal_vectors(u, model),
        "support_actions": named_supports,
        "constraint_action_global_by_node": nodal_vectors(constraint_actions, model),
        "element_results": element_output,
        "c150_relative_stiffness_diagnostic": connector_diagnostic,
        "checks": {
            "assembled_equation_max_abs_residual_free_dof": float(np.max(np.abs(equation_residual))),
            "global_force_balance_N": clean(force_balance),
            "global_force_balance_max_abs_N": float(np.max(np.abs(force_balance))),
            "global_moment_balance_about_origin_N_m": clean(moment_balance),
            "global_moment_balance_max_abs_N_m": float(np.max(np.abs(moment_balance))),
            "stiffness_symmetry_max_abs": float(np.max(np.abs(data["K"] - data["K"].T))),
        },
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--repo-root", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    fixture = args.repo_root / "projects/chirality-piping/fixtures/product_preview/invented_preview_model.json"
    model = json.loads(fixture.read_text(encoding="utf-8"))
    literal_data = build_fixture(model, "load:L-100")
    physical_data = {**literal_data}
    physical_data["K"] = literal_data["K"].copy()
    physical_data["K_elements"] = literal_data["K_elements"].copy()
    indices = literal_data["expansion_joint_indices"]
    user_global = literal_data["expansion_joint_global_stiffness"]
    physical_data["K"][np.ix_(indices, indices)] -= user_global
    physical_data["K_elements"][np.ix_(indices, indices)] -= user_global
    physical_data["connector_enabled"] = False

    def enumerate_candidates(data: dict[str, Any]) -> list[dict[str, Any]]:
        branches = []
        for contact_active in (False, True):
            branches.append(solve_branch(data, contact_active, "none"))
            branches.append(solve_branch(data, contact_active, "stick"))
            for sign in (-1, 1):
                branches.append(solve_branch(data, contact_active, "slip", sign))
        return branches

    physical_candidates = enumerate_candidates(physical_data)
    physical_frictionless = [item for item in physical_candidates if item["friction_state"] == "none" and item["admissible"]]
    physical_static = [item for item in physical_candidates if item["friction_state"] != "none" and item["admissible"]]
    literal_candidates = enumerate_candidates(literal_data)
    literal_frictionless = [item for item in literal_candidates if item["friction_state"] == "none" and item["admissible"]]
    literal_static = [item for item in literal_candidates if item["friction_state"] != "none" and item["admissible"]]

    result = {
        "schema_version": "1.0",
        "status": "INDEPENDENT_EXPECTATION_FROZEN_BEFORE_PRODUCTION_COMPARISON",
        "calculation_basis": {
            "units": {"length": "m", "force": "N", "rotation": "rad", "moment": "N*m", "stress_or_modulus": "Pa"},
            "unit_scope": "explicit fixture-local SI only; no project-wide canonical conversion acceptance",
            "fixture": "projects/chirality-piping/fixtures/product_preview/invented_preview_model.json",
            "fixture_sha256": sha256(fixture),
            "load_case": "load:L-100",
            "pressure_primitive_loads_omitted": literal_data["pressure_omissions"],
            "common_included": ["four straight Euler-Bernoulli pipe members", "SH-140 UZ ground spring", "P-120 full uniform global-Z load", "N-140 global-Y nodal force", "P-120 uniform thermal strain", "S-100/S-120/S-130 rigid restraints", "NL-140 unilateral UY", "NL-130-FRIC UZ current-normal Coulomb branch"],
            "physical_frame_case": "excludes C-150 relative stiffness because its unresolved finite-end formulation produces an unbalanced global moment; this case establishes the objective frame/support/load reference without deciding audit D02",
            "literal_adapter_case": "adds the exact C-150 authored local relative stiffness in parallel with P-130; retained only as an algebraic adapter mapping diagnostic, not a physically balanced whole-frame oracle",
            "excluded_from_both": ["pressure loads and pressure thrust", "constant-effort metadata with no solve consumption", "rule checks", "large displacement", "load-step/cyclic friction history"],
            "material": literal_data["material"],
            "expansion_joint_local_stiffness_order_UX_UY_UZ_RX_RY_RZ": literal_data["expansion_joint_local_stiffness"],
        },
        "controls": scalar_control_results(),
        "physical_frame_branch_inventory": [
            {
                "contact_active": item["contact_active"],
                "friction_state": item["friction_state"],
                "slip_sign": item["slip_sign"],
                "contact_admissible": item["contact"]["admissible"],
                "friction_admissible": item["friction_check"]["opposes_displacement_or_sticks"],
                "overall_admissible": item["admissible"],
                "contact_u_m": float(item["u"][physical_data["contact_dof"]]),
                "contact_R_N": float(item["residual"][physical_data["contact_dof"]]) if item["contact_active"] else 0.0,
                "friction_u_m": float(item["u"][physical_data["friction_dof"]]),
                "friction_R_N": item["friction_force"],
                "normal_R_N": item["normal_reaction"],
                "mu_abs_normal_N": item["friction_bound"],
            }
            for item in physical_candidates
        ],
        "physical_frame_frictionless_actual_control": branch_results(physical_data, physical_frictionless[0]) if len(physical_frictionless) == 1 else {"candidate_count": len(physical_frictionless)},
        "physical_frame_zero_reference_current_normal_static_result": branch_results(physical_data, physical_static[0]) if len(physical_static) == 1 else {"candidate_count": len(physical_static)},
        "literal_adapter_mapping_diagnostic": {
            "branch_inventory": [
                {
                    "contact_active": item["contact_active"],
                    "friction_state": item["friction_state"],
                    "slip_sign": item["slip_sign"],
                    "overall_admissible": item["admissible"],
                    "contact_u_m": float(item["u"][literal_data["contact_dof"]]),
                    "contact_R_N": float(item["residual"][literal_data["contact_dof"]]) if item["contact_active"] else 0.0,
                    "friction_u_m": float(item["u"][literal_data["friction_dof"]]),
                    "friction_R_N": item["friction_force"],
                    "normal_R_N": item["normal_reaction"],
                    "mu_abs_normal_N": item["friction_bound"],
                }
                for item in literal_candidates
            ],
            "frictionless": branch_results(literal_data, literal_frictionless[0]) if len(literal_frictionless) == 1 else {"candidate_count": len(literal_frictionless)},
            "zero_reference_current_normal_static": branch_results(literal_data, literal_static[0]) if len(literal_static) == 1 else {"candidate_count": len(literal_static)},
            "applicability": "algebraic fixture-to-adapter mapping only; nonzero external moment-balance residual is the independently exposed D02 connector-formulation limitation",
        },
        "interpretation_forks": {
            "zero_reference_static": "calculated branch; current-return normal and friction action solved simultaneously",
            "incremental_history": "OPEN: fixture has an initial state but no ordered load increments, committed slip/reference state, rollback rule, or prior converged load-step history",
            "direction_at_zero": "OPEN only if a nonzero prior slip/reference is asserted; zero-reference static branch determines direction from returned displacement",
            "pressure": "omitted by the selected pressure-free case; P5 owns pressure matrix",
        },
    }
    if len(physical_frictionless) != 1 or len(literal_frictionless) != 1:
        raise RuntimeError("expected one admissible frictionless contact branch per formulation")
    if len(physical_static) != 1 or len(literal_static) != 1:
        raise RuntimeError("expected one admissible static friction/contact branch per formulation")
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    main()
