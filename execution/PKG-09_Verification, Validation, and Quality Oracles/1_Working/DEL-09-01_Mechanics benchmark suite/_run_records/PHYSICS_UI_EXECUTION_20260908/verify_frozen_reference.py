#!/usr/bin/env python3
"""Deterministic checks for the independently frozen reference package."""

from __future__ import annotations

import argparse
import importlib.util
import json
from pathlib import Path

import numpy as np


def load_module(path: Path):
    spec = importlib.util.spec_from_file_location("independent_mixed_reference", path)
    module = importlib.util.module_from_spec(spec)
    assert spec.loader is not None
    spec.loader.exec_module(module)
    return module


def close(actual: float, expected: float, scale: float = 1.0) -> None:
    # Numerical implementation self-check only; this is not an engineering or
    # product comparison tolerance.
    assert abs(actual - expected) <= 2.0e-12 * max(scale, abs(expected)), (actual, expected)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--repo-root", type=Path, required=True)
    parser.add_argument("--expected", type=Path, required=True)
    args = parser.parse_args()
    script = args.expected.parent / "independent_mixed_reference.py"
    module = load_module(script)
    expected = json.loads(args.expected.read_text(encoding="utf-8"))

    # Independent implementation controls against closed-form cantilever and
    # scalar spring/contact results frozen in the package.
    control = expected["controls"]["distributed_load"]
    E, I, L, q = (control["input"][key] for key in ("E_Pa", "I_m4", "L_m", "q_y_N_per_m"))
    k = module.beam_local_stiffness(E, 0.4 * E, 0.01, I, I, 2.0 * I, L)
    force = module.consistent_uniform_local(np.array([0.0, q, 0.0]), L)
    tip = np.linalg.solve(k[np.ix_(range(6, 12), range(6, 12))], force[6:12])
    reaction = k[0:6, 6:12] @ tip - force[0:6]
    close(tip[1], control["expected"]["tip_y_m"])
    close(reaction[1], control["expected"]["root_reaction_y_N"])
    close(reaction[5], control["expected"]["root_reaction_mz_N_m"])

    spring = expected["controls"]["retained_spring"]
    close(spring["input"]["k_N_per_m"] * spring["expected"]["displacement_m"], spring["input"]["force_N"])
    contact = expected["controls"]["frictionless_unilateral_contact"]
    assert contact["expected_active_branch"]["reaction_N"] < 0.0
    close(contact["complementarity"]["lambda_times_g"], 0.0)

    physical = expected["physical_frame_zero_reference_current_normal_static_result"]
    assert physical["state"]["admissible"] is True
    assert physical["state"]["contact"]["state"] == "active"
    assert physical["state"]["friction"]["opposes_displacement_or_sticks"] is True
    assert physical["state"]["friction"]["tangential_action_N"] * physical["state"]["friction"]["tangential_displacement_m"] <= 0.0
    close(
        abs(physical["state"]["friction"]["tangential_action_N"]),
        physical["state"]["friction"]["mu_times_current_normal_N"],
        1.0,
    )
    assert physical["checks"]["global_force_balance_max_abs_N"] < 1.0e-7
    assert physical["checks"]["global_moment_balance_max_abs_N_m"] < 1.0e-7
    for element in physical["element_results"].values():
        assert max(map(abs, element["element_force_balance_N"])) < 1.0e-7
        assert max(map(abs, element["element_moment_balance_about_i_N_m"])) < 1.0e-7

    literal = expected["literal_adapter_mapping_diagnostic"]["zero_reference_current_normal_static"]
    connector = literal["c150_relative_stiffness_diagnostic"]
    assert connector["enabled"] is True
    assert max(map(abs, connector["net_force_N"])) < 1.0e-7
    assert abs(connector["net_moment_about_origin_N_m"][1]) > 1.0
    close(
        literal["checks"]["global_moment_balance_about_origin_N_m"][1],
        connector["net_moment_about_origin_N_m"][1],
        abs(connector["net_moment_about_origin_N_m"][1]),
    )

    print("PASS: frozen independent reference controls, branch conditions, force/moment balances, and C-150 diagnostic")


if __name__ == "__main__":
    main()
