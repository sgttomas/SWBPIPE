#!/usr/bin/env python3
"""Independent Option-C pressure-reference arithmetic; standard library only.

The candidate Poisson ratio and thermal case are explicit inputs. This script
does not import or execute production code.
"""

from __future__ import annotations

import json
import math


def calculate() -> dict[str, object]:
    outer_diameter_m = 0.168
    wall_thickness_m = 0.007
    length_m = 2.0
    elastic_modulus_pa = 200.0e9
    internal_pressure_pa = 1.0e6
    candidate_poisson_ratio = 0.3
    candidate_alpha_per_k = 1.2e-5
    candidate_delta_t_k = 75.0

    outer_radius_m = outer_diameter_m / 2.0
    inner_radius_m = outer_radius_m - wall_thickness_m
    mean_radius_m = (inner_radius_m + outer_radius_m) / 2.0
    internal_area_m2 = math.pi * inner_radius_m**2
    wall_area_m2 = math.pi * (outer_radius_m**2 - inner_radius_m**2)
    pressure_thrust_n = internal_pressure_pa * internal_area_m2
    mean_radius_cap_proxy_n = internal_pressure_pa * math.pi * mean_radius_m**2
    lame_a_pa = pressure_thrust_n / wall_area_m2
    lame_b_pa_m2 = (
        internal_pressure_pa
        * inner_radius_m**2
        * outer_radius_m**2
        / (outer_radius_m**2 - inner_radius_m**2)
    )
    hoop_inner_pa = lame_a_pa + lame_b_pa_m2 / inner_radius_m**2
    hoop_outer_pa = lame_a_pa + lame_b_pa_m2 / outer_radius_m**2
    hoop_ligament_average_pa = internal_pressure_pa * inner_radius_m / wall_thickness_m
    thin_mean_hoop_pa = internal_pressure_pa * mean_radius_m / wall_thickness_m
    thin_mean_longitudinal_pa = thin_mean_hoop_pa / 2.0
    thin_inner_hoop_pa = internal_pressure_pa * inner_radius_m / wall_thickness_m
    thin_inner_longitudinal_pa = thin_inner_hoop_pa / 2.0

    nu = candidate_poisson_ratio
    free_closed_strain = (1.0 - 2.0 * nu) * lame_a_pa / elastic_modulus_pa
    restrained_closed_wall_force_n = 2.0 * nu * pressure_thrust_n
    restrained_closed_effective_force_n = restrained_closed_wall_force_n - pressure_thrust_n
    open_free_strain = -2.0 * nu * lame_a_pa / elastic_modulus_pa

    thermal_strain = candidate_alpha_per_k * candidate_delta_t_k
    thermal_restraint_force_n = elastic_modulus_pa * wall_area_m2 * thermal_strain
    mixed_restrained_wall_force_n = restrained_closed_wall_force_n - thermal_restraint_force_n
    mixed_restrained_effective_force_n = mixed_restrained_wall_force_n - pressure_thrust_n
    mixed_free_strain = thermal_strain + free_closed_strain

    return {
        "schema_version": "1.0",
        "calculation_boundary": "independent_standard_library_arithmetic_no_production_import",
        "signs": {
            "section_cut": "Nw tension positive",
            "effective_internal_only_closed_end": "S = Nw - P",
            "member_endpoint_action_pair_for_positive_tension": ["-Nw", "+Nw"],
            "section_cut_from_member_endpoint_actions": ["Ncut_i=-Fi_x", "Ncut_j=+Fj_x"],
        },
        "inputs": {
            "outer_diameter_m": outer_diameter_m,
            "wall_thickness_m": wall_thickness_m,
            "length_m": length_m,
            "elastic_modulus_pa": elastic_modulus_pa,
            "internal_pressure_pa": internal_pressure_pa,
            "candidate_poisson_ratio": candidate_poisson_ratio,
            "candidate_alpha_per_k": candidate_alpha_per_k,
            "candidate_delta_t_k": candidate_delta_t_k,
        },
        "geometry_and_pressure": {
            "outer_radius_m": outer_radius_m,
            "inner_radius_m": inner_radius_m,
            "mean_radius_m": mean_radius_m,
            "thickness_over_inner_radius": wall_thickness_m / inner_radius_m,
            "internal_area_m2": internal_area_m2,
            "wall_area_m2": wall_area_m2,
            "pressure_thrust_n": pressure_thrust_n,
            "mean_radius_cap_proxy_n": mean_radius_cap_proxy_n,
            "mean_radius_cap_proxy_relative_to_bore_thrust": (
                mean_radius_cap_proxy_n / pressure_thrust_n - 1.0
            ),
        },
        "stress_models_pa": {
            "exact_closed_end_lame_longitudinal": lame_a_pa,
            "exact_lame_hoop_inner": hoop_inner_pa,
            "exact_lame_hoop_outer": hoop_outer_pa,
            "exact_diametral_ligament_average_hoop": hoop_ligament_average_pa,
            "thin_inner_radius_hoop": thin_inner_hoop_pa,
            "thin_inner_radius_longitudinal": thin_inner_longitudinal_pa,
            "thin_mean_radius_hoop": thin_mean_hoop_pa,
            "thin_mean_radius_longitudinal": thin_mean_longitudinal_pa,
            "mean_radius_longitudinal_relative_to_exact": (
                thin_mean_longitudinal_pa / lame_a_pa - 1.0
            ),
            "mean_radius_hoop_relative_to_exact_inner_surface": (
                thin_mean_hoop_pa / hoop_inner_pa - 1.0
            ),
        },
        "cases": {
            "free_closed_pressure_only": {
                "wall_force_n": pressure_thrust_n,
                "effective_force_n": 0.0,
                "wall_stress_pa": lame_a_pa,
                "axial_strain": free_closed_strain,
                "tip_displacement_m": free_closed_strain * length_m,
                "support_reaction_n_each": 0.0,
            },
            "restrained_closed_pressure_only": {
                "wall_force_n": restrained_closed_wall_force_n,
                "effective_force_n": restrained_closed_effective_force_n,
                "wall_stress_pa": restrained_closed_wall_force_n / wall_area_m2,
                "axial_strain": 0.0,
                "support_on_complete_vessel_inward_magnitude_n_each": -restrained_closed_effective_force_n,
            },
            "open_or_separately_supported_closures_free_barrel": {
                "wall_force_n": 0.0,
                "effective_force_if_fluid_cut_convention_is_declared_n": -pressure_thrust_n,
                "wall_stress_pa": 0.0,
                "axial_strain": open_free_strain,
                "tip_displacement_m": open_free_strain * length_m,
                "closure_support_reaction_outward_pressure_thrust_n_each": pressure_thrust_n,
            },
            "free_closed_thermal_plus_pressure": {
                "wall_force_n": pressure_thrust_n,
                "effective_force_n": 0.0,
                "wall_stress_pa": lame_a_pa,
                "axial_strain": mixed_free_strain,
                "tip_displacement_m": mixed_free_strain * length_m,
            },
            "restrained_closed_thermal_plus_pressure": {
                "thermal_strain": thermal_strain,
                "thermal_restraint_force_magnitude_n": thermal_restraint_force_n,
                "wall_force_n": mixed_restrained_wall_force_n,
                "effective_force_n": mixed_restrained_effective_force_n,
                "wall_stress_pa": mixed_restrained_wall_force_n / wall_area_m2,
                "axial_strain": 0.0,
                "support_on_complete_vessel_inward_magnitude_n_each": -mixed_restrained_effective_force_n,
            },
        },
        "identity_checks": {
            "lame_radial_plus_hoop_pa": 2.0 * lame_a_pa,
            "restrained_closed_force_equals_2nuP": restrained_closed_wall_force_n,
            "free_closed_wall_force_equals_cap_thrust": pressure_thrust_n,
            "open_free_wall_force_zero": 0.0,
        },
    }


if __name__ == "__main__":
    print(json.dumps(calculate(), indent=2, sort_keys=True))
