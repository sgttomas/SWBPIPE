#!/usr/bin/env python3
"""Independent deterministic arithmetic for pressure-design refutation.

This script imports no Chirality production module. It checks the exact-annulus
four-case equations, equivalent nodal signs, multi-element cancellation, and
temperature interpolation compatibility of an isotropic E/G/nu triple.
"""

from __future__ import annotations

import json
import math


def close(actual: float, expected: float, *, rel: float = 1.0e-12) -> None:
    if not math.isclose(actual, expected, rel_tol=rel, abs_tol=1.0e-8):
        raise AssertionError(f"{actual!r} != {expected!r}")


do = 0.168
t = 0.007
length = 2.0
young = 200.0e9
pressure_i = 1.0e6
pressure_e = 0.0
nu = 0.3
alpha = 1.2e-5
delta_t = 75.0

ro = do / 2.0
ri = ro - t
area_i = math.pi * ri**2
area_e = math.pi * ro**2
area_s = area_e - area_i
pc = pressure_i * area_i - pressure_e * area_e
ea = young * area_s


def wall_force(strain: float, temperature_change: float) -> float:
    return ea * (strain - alpha * temperature_change) + 2.0 * nu * pc


def effective_force(n_wall: float) -> float:
    return n_wall - pc


def eigen_equivalent(temperature_change: float) -> list[float]:
    # f0 = integral(B^T E A epsilon0 dx), epsilon0=alpha*dT-2*nu*Pc/(EA)
    initial_force = ea * alpha * temperature_change - 2.0 * nu * pc
    return [-initial_force, initial_force]


def cap_transfer() -> list[float]:
    return [-pc, pc]


eps_free_closed = alpha * 0.0 + (1.0 - 2.0 * nu) * pc / ea
n_free_closed = wall_force(eps_free_closed, 0.0)
eps_fixed_closed = 0.0
n_fixed_closed = wall_force(eps_fixed_closed, 0.0)
eps_free_barrel = alpha * 0.0 - 2.0 * nu * pc / ea
n_free_barrel = wall_force(eps_free_barrel, 0.0)
eps_free_mixed = alpha * delta_t + (1.0 - 2.0 * nu) * pc / ea
n_free_mixed = wall_force(eps_free_mixed, delta_t)
n_fixed_mixed = wall_force(0.0, delta_t)

close(n_free_closed, pc)
close(effective_force(n_free_closed), 0.0)
close(n_fixed_closed, 2.0 * nu * pc)
close(effective_force(n_fixed_closed), -(1.0 - 2.0 * nu) * pc)
close(n_free_barrel, 0.0)
close(effective_force(n_free_barrel), -pc)
close(n_free_mixed, pc)
close(n_fixed_mixed, 2.0 * nu * pc - ea * alpha * delta_t)

# q_wall = Kd-f0. At d=0 this must equal [-Nw,+Nw].
fixed_mixed_q_wall = [-value for value in eigen_equivalent(delta_t)]
close(fixed_mixed_q_wall[0], -n_fixed_mixed)
close(fixed_mixed_q_wall[1], n_fixed_mixed)

# A free closed pipe solves Kd=f0+f_cap and recovers q_wall=Kd-f0=f_cap.
free_closed_q_wall = cap_transfer()
close(free_closed_q_wall[0], -n_free_closed)
close(free_closed_q_wall[1], n_free_closed)

# A free barrel with separately supported closures solves Kd=f0 and q_wall=0.
free_barrel_q_wall = [0.0, 0.0]
close(sum(abs(value) for value in free_barrel_q_wall), 0.0)

# Exact Lamé boundary and axial values.
a_lame = pc / area_s
b_lame = (pressure_i - pressure_e) * ri**2 * ro**2 / (ro**2 - ri**2)
sigma_r_i = a_lame - b_lame / ri**2
sigma_r_o = a_lame - b_lame / ro**2
sigma_h_i = a_lame + b_lame / ri**2
sigma_h_o = a_lame + b_lame / ro**2
close(sigma_r_i, -pressure_i)
close(sigma_r_o, -pressure_e)
close(n_free_closed / area_s, a_lame)

# Equal Pc cap-transfer pairs cancel at the shared node. A Pc change does not;
# the residual is a physical area/pressure transition load that needs ownership.
equal_two_element_cap = [-pc, pc - pc, pc]
pc_2 = 0.8 * pc
changed_two_element_cap = [-pc, pc - pc_2, pc_2]
close(equal_two_element_cap[1], 0.0)
close(changed_two_element_cap[1], 0.2 * pc)

# Even if each stored temperature endpoint is isotropically compatible,
# independent linear interpolation of E, G, and nu is generally incompatible
# when nu varies.
e0, nu0 = 200.0e9, 0.30
e1, nu1 = 150.0e9, 0.25
g0 = e0 / (2.0 * (1.0 + nu0))
g1 = e1 / (2.0 * (1.0 + nu1))
e_mid = 0.5 * (e0 + e1)
nu_mid = 0.5 * (nu0 + nu1)
g_mid_linear = 0.5 * (g0 + g1)
g_mid_required = e_mid / (2.0 * (1.0 + nu_mid))
interpolation_relative_residual = (g_mid_linear - g_mid_required) / g_mid_required
if abs(interpolation_relative_residual) <= 1.0e-6:
    raise AssertionError("example must expose nonlinear isotropic compatibility")

output = {
    "inputs": {
        "do_m": do,
        "t_m": t,
        "length_m": length,
        "E_pa": young,
        "pi_pa": pressure_i,
        "pe_pa": pressure_e,
        "nu": nu,
        "alpha_per_k": alpha,
        "delta_t_k": delta_t,
    },
    "geometry": {
        "ri_m": ri,
        "ro_m": ro,
        "Ai_m2": area_i,
        "Ae_m2": area_e,
        "As_m2": area_s,
        "Pc_n": pc,
    },
    "cases": {
        "free_closed_pressure": {
            "Nw_n": n_free_closed,
            "S_n": effective_force(n_free_closed),
            "strain": eps_free_closed,
            "displacement_m": eps_free_closed * length,
        },
        "fixed_closed_pressure": {
            "Nw_n": n_fixed_closed,
            "S_n": effective_force(n_fixed_closed),
            "strain": eps_fixed_closed,
        },
        "free_barrel_separate_closure_pressure": {
            "Nw_n": n_free_barrel,
            "S_n": effective_force(n_free_barrel),
            "strain": eps_free_barrel,
            "displacement_m": eps_free_barrel * length,
        },
        "free_closed_mixed": {
            "Nw_n": n_free_mixed,
            "S_n": effective_force(n_free_mixed),
            "strain": eps_free_mixed,
            "displacement_m": eps_free_mixed * length,
        },
        "fixed_closed_mixed": {
            "Nw_n": n_fixed_mixed,
            "S_n": effective_force(n_fixed_mixed),
            "strain": 0.0,
        },
    },
    "nodal_signs": {
        "pressure_poisson_and_thermal_eigen_equivalent_mixed_n": eigen_equivalent(delta_t),
        "closed_cap_transfer_n": cap_transfer(),
        "fixed_mixed_wall_actions_n": fixed_mixed_q_wall,
        "free_closed_wall_actions_n": free_closed_q_wall,
    },
    "lame_pa": {
        "sigma_r_inner": sigma_r_i,
        "sigma_r_outer": sigma_r_o,
        "sigma_hoop_inner": sigma_h_i,
        "sigma_hoop_outer": sigma_h_o,
        "sigma_axial_free_closed": n_free_closed / area_s,
    },
    "two_element_cap_assembly_n": {
        "equal_pressure_area": equal_two_element_cap,
        "changed_pressure_area": changed_two_element_cap,
    },
    "temperature_interpolation_counterexample": {
        "endpoint_0": {"E_pa": e0, "G_pa": g0, "nu": nu0},
        "endpoint_1": {"E_pa": e1, "G_pa": g1, "nu": nu1},
        "midpoint_linear": {"E_pa": e_mid, "G_pa": g_mid_linear, "nu": nu_mid},
        "midpoint_required_G_pa": g_mid_required,
        "relative_G_residual": interpolation_relative_residual,
    },
}

print(json.dumps(output, indent=2, sort_keys=True))
