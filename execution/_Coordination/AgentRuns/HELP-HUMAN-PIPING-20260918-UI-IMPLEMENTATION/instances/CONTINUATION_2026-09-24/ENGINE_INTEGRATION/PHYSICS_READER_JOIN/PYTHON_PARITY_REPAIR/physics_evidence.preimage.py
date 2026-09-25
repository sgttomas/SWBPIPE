"""Finite exact-profile evidence and source joins; never producer authentication.

The published membrane and effective force values are source observations. This
reader validates their provenance relationships without reconstructing them from
rounded neighbouring rows or treating pressure arithmetic as a numerical pass.
"""
from __future__ import annotations

import math
import struct
import json
from pathlib import Path
from collections.abc import Mapping
from typing import Any

PROFILE = "exact_straight_pressure_v2"
PIPE_KINDS = {
    "pipe_wall_endpoint_action_v2", "pipe_wall_axial_force_v2",
    "pipe_effective_axial_force_v2", "pipe_axial_membrane_stress_v2",
    "pipe_lame_radial_stress_v2", "pipe_lame_hoop_stress_v2",
    "pipe_elastic_normal_stress_maximum_v2",
}
SIGNS = {
    "pipe_wall_endpoint_action_v2": "node-on-element wall action, positive along authored local x toward end j; cap transfer is not subtracted from wall recovery",
    "pipe_wall_axial_force_v2": "tension-positive material wall section resultant Nw",
    "pipe_effective_axial_force_v2": "effective wall-fluid resultant S=Nw-pAi; not material stress or a support reaction",
    "pipe_elastic_normal_stress_maximum_v2": "nonnegative circumferential maximum |Nw/As|+hypot(My,Mz)/Z; bounded over all straight statics intervals; torsional shear remains separate; no code stress or equivalent stress claim",
    "pipe_axial_membrane_stress_v2": "tension-positive axial wall membrane stress Nw/As; no added longitudinal pressure scalar",
    "pipe_lame_radial_stress_v2": "tension-positive radial stress at named surface, inner traction -p and zero external pressure increment",
    "pipe_lame_hoop_stress_v2": "tension-positive circumferential stress at named surface for long straight annulus, zero external pressure increment",
}
SUPPORT_SIGN = "support-on-pipe; positive global force and right-hand couple about attached node; force and moment norms remain separate"

GEOMETRY = {"pipe_id", "geometry_basis", "outside_diameter_m", "effective_wall_thickness_m", "ri_m", "ro_m", "Ai_m2", "As_m2", "I_m4", "J_m4", "Z_m3"}
MATERIAL = {"pipe_id", "material_id", "E_pa", "nu", "G_pa", "constitutive_basis", "thermal_consumed", "alpha_per_kelvin", "provenance"}


def _require(condition: bool, detail: str) -> None:
    if not condition:
        raise ValueError(f"SOURCE_PHYSICS_EVIDENCE_INVALID: {detail}")


def _shape(value: Any, keys: set[str], detail: str) -> None:
    _require(isinstance(value, Mapping) and set(value) == keys, detail)


def _text(value: Any) -> bool:
    return isinstance(value, str) and bool(value)


def _number(value: Any) -> bool:
    return type(value) in (int, float) and math.isfinite(value)


def _finite_tree(value: Any) -> None:
    if type(value) in (int, float):
        _require(math.isfinite(value), "nonfinite evidence")
    elif isinstance(value, Mapping):
        for item in value.values():
            _finite_tree(item)
    elif isinstance(value, list):
        for item in value:
            _finite_tree(item)


def _strings(value: Any, nonempty: bool = False) -> bool:
    return isinstance(value, list) and (not nonempty or bool(value)) and all(_text(x) for x in value) and len(set(value)) == len(value)


def _indexed(value: Any, key: str, detail: str) -> dict[str, Any]:
    _require(isinstance(value, list), detail)
    _require(all(isinstance(row, Mapping) and _text(row.get(key)) for row in value), detail)
    indexed = {row[key]: row for row in value}
    _require(len(indexed) == len(value), f"duplicate {detail}")
    return indexed


def _vector(value: Any, size: int) -> bool:
    return isinstance(value, list) and len(value) == size and all(_number(x) for x in value)


def validate_physics_evidence(source: Mapping[str, Any]) -> None:
    try:
        _validate_physics_evidence(source)
    except (TypeError, KeyError, AttributeError, OverflowError) as error:
        raise ValueError("SOURCE_PHYSICS_EVIDENCE_INVALID: malformed evidence") from error


def _validate_physics_evidence(source: Mapping[str, Any]) -> None:
    _require(not any(key in source for key in ("source_block_recovery", "carrier_evidence")), "unsupported source namespace")
    evidence = source.get("contract_evidence")
    _shape(evidence, {"pressure", "connector", "exact_cases"}, "namespace")
    _finite_tree(evidence)
    _require(evidence["connector"] == [] and isinstance(evidence["pressure"], list), "unsupported connector")
    cases = _indexed(evidence["exact_cases"], "load_case_id", "exact cases")
    rows = _indexed(source.get("results"), "id", "result IDs")
    quality = source.get("numerical_quality", {}).get("cases", [])
    numerical_ids = [case.get("basis_ref", {}).get("ref_id") for case in quality]
    _require(all(case.get("basis_ref", {}).get("ref_type") == "load_case" for case in quality) and len(set(numerical_ids)) == len(numerical_ids), "numerical case scope")
    if source.get("status", {}).get("mechanics") == "MECHANICS_SOLVED":
        _require(bool(cases) and set(cases) == set(numerical_ids), "case coverage")
    materials: dict[str, dict[str, Any]] = {}
    geometries: dict[str, dict[str, Any]] = {}
    for cid, case in cases.items():
        _shape(case, {"load_case_id", "profile_mode", "material_basis", "pipe_materials", "pipe_sections", "pipe_stress_extrema", "stress_maximum_coverage", "pressure_rhs_assembly"}, "case shape")
        _require(case["profile_mode"] == PROFILE and _text(case["material_basis"]), "case profile/material basis")
        materials[cid] = _indexed(case["pipe_materials"], "pipe_id", "case materials")
        geometries[cid] = _indexed(case["pipe_sections"], "pipe_id", "case sections")
        _require(bool(geometries[cid]) and set(materials[cid]) == set(geometries[cid]), "material/geometry coverage")
        for material in materials[cid].values():
            _shape(material, MATERIAL, "material shape")
            _require(_text(material["material_id"]) and _text(material["provenance"]) and material["constitutive_basis"] == "homogeneous_isotropic_E_nu_v1", "material basis")
            _require(all(_number(material[k]) for k in ("E_pa", "G_pa", "nu")) and material["E_pa"] > 0 and material["G_pa"] > 0 and -1 < material["nu"] < 0.5, "material range")
            expected_g = material["E_pa"] / (2 * (1 + material["nu"]))
            # Producer uses scaled arithmetic; permit only its final binary64
            # representation difference. Duplicated source records agree exactly.
            _require(math.isfinite(expected_g) and expected_g > 0 and abs(int.from_bytes(struct.pack(">d", material["G_pa"]), "big") - int.from_bytes(struct.pack(">d", expected_g), "big")) <= 2, "derived shear modulus")
            _require(type(material["thermal_consumed"]) is bool and (_number(material["alpha_per_kelvin"]) if material["thermal_consumed"] else material["alpha_per_kelvin"] is None), "thermal alpha")
        for geometry in geometries[cid].values():
            _shape(geometry, GEOMETRY, "geometry shape")
            _require(geometry["geometry_basis"] == "authored_normalized_od_wall_v1" and all(_number(geometry[k]) and geometry[k] > 0 for k in GEOMETRY - {"pipe_id", "geometry_basis"}), "source geometry")
            _require(geometry["ri_m"] < geometry["ro_m"] and geometry["effective_wall_thickness_m"] < geometry["outside_diameter_m"] / 2 and geometry["ro_m"] == geometry["outside_diameter_m"] * 0.5 and geometry["ri_m"] == geometry["outside_diameter_m"] * 0.5 - geometry["effective_wall_thickness_m"], "source annulus")
        coverage = case["stress_maximum_coverage"]
        _shape(coverage, {"complete", "unavailable_pipe_ids"}, "maximum coverage")
        _require(type(coverage["complete"]) is bool and _strings(coverage["unavailable_pipe_ids"]), "maximum coverage values")
        missing = set(coverage["unavailable_pipe_ids"])
        _require(missing <= set(geometries[cid]) and coverage["complete"] == (not missing), "maximum coverage contradiction")
        extrema = _indexed(case["pipe_stress_extrema"], "pipe_id", "extrema")
        _require(set(extrema) == set(geometries[cid]) - missing, "maximum member coverage")
        for pid, extremum in extrema.items():
            _shape(extremum, {"pipe_id", "result_id", "approximation", "coefficient_basis", "enclosure_scope", "station_fraction", "span_index", "local_fraction", "value_lower_pa", "value_upper_pa", "global_upper_bound_pa", "certified_gap_pa", "subdivisions"}, "extrema shape")
            row = rows.get(extremum["result_id"], {})
            _require(row.get("kind") == "pipe_elastic_normal_stress_maximum_v2" and row.get("entity_ref") == pid and row.get("basis_ref") == {"ref_type": "load_case", "ref_id": cid}, "extrema result binding")
            _require(row.get("value") == extremum["value_lower_pa"] + 0.5 * (extremum["value_upper_pa"] - extremum["value_lower_pa"]), "extrema published witness")
            _require(all(_number(extremum[k]) for k in ("station_fraction", "local_fraction", "value_lower_pa", "value_upper_pa", "global_upper_bound_pa", "certified_gap_pa")), "extrema numeric values")
            _require(0 <= extremum["station_fraction"] <= 1 and 0 <= extremum["local_fraction"] <= 1 and 0 <= extremum["value_lower_pa"] <= extremum["value_upper_pa"] <= extremum["global_upper_bound_pa"] and 0 <= extremum["certified_gap_pa"] <= 1e-12 + 1e-12 * extremum["value_lower_pa"], "extrema bounds")
            _require(type(extremum["span_index"]) is int and extremum["span_index"] >= 0 and type(extremum["subdivisions"]) is int and 0 <= extremum["subdivisions"] <= 131072 and all(_text(extremum[k]) for k in ("approximation", "coefficient_basis", "enclosure_scope")), "extrema witness")
        _validate_rhs(case["pressure_rhs_assembly"], cid)
    owners: dict[tuple[str, str], str] = {}
    bindings: set[str] = set()
    regions: set[tuple[str, str]] = set()
    region_pressures: dict[tuple[str, str], float] = {}
    _require(not geometries or all(set(members) == set(next(iter(geometries.values()))) for members in geometries.values()), "active member coverage across cases")
    for region in evidence["pressure"]:
        _shape(region, {"load_case_id", "region_id", "profile_mode", "profile_version", "pressure_basis", "p_pa", "external_pressure_increment_pa", "member_pipe_ids", "geometry", "materials", "terminals", "applied_loads", "approximation", "provenance", "geometry_representation_guard", "result_ids"}, "region shape")
        cid, rid = region["load_case_id"], region["region_id"]
        _require(cid in cases and _text(rid) and (cid, rid) not in regions, "region case/identity")
        regions.add((cid, rid))
        region_pressures[cid, rid] = region["p_pa"]
        _require(region["profile_mode"] == PROFILE and region["profile_version"] == "2.0.0" and region["pressure_basis"] == "internal_differential_zero_external_v1" and region["external_pressure_increment_pa"] == 0 and _number(region["p_pa"]) and region["p_pa"] >= 0, "region pressure basis")
        _require(_text(region["provenance"]) and region["approximation"] == "long_straight_annulus_small_strain_v2" and region["geometry_representation_guard"] == {"epsilon_multiplier": 64, "meaning": "arithmetic_representation_only"}, "region provenance/guard")
        members = region["member_pipe_ids"]
        _require(_strings(members, True) and set(members) <= set(geometries[cid]), "region members")
        for pid in members:
            _require((cid, pid) not in owners, "overlapping pressure regions")
            owners[cid, pid] = rid
        gm = _indexed(region["geometry"], "pipe_id", "region geometry")
        mm = _indexed(region["materials"], "pipe_id", "region materials")
        loads = _indexed(region["applied_loads"], "pipe_id", "region loads")
        _require(set(gm) == set(mm) == set(loads) == set(members), "region evidence coverage")
        for pid in members:
            _shape(gm[pid], GEOMETRY | {"traversal_forward"}, "region geometry shape")
            _require(type(gm[pid]["traversal_forward"]) is bool and {k:v for k,v in gm[pid].items() if k != "traversal_forward"} == geometries[cid][pid], "geometry disagreement")
            _shape(mm[pid], MATERIAL | {"temperature_basis"}, "region material shape")
            _temperature_basis(mm[pid]["temperature_basis"], cases[cid]["material_basis"], mm[pid]["material_id"])
            _require(isinstance(mm[pid]["temperature_basis"], Mapping) and _text(mm[pid]["temperature_basis"].get("selection")) and {k:v for k,v in mm[pid].items() if k != "temperature_basis"} == materials[cid][pid], "material disagreement")
            _shape(loads[pid], {"pipe_id", "eigenload_pair_local_n", "local_x_global", "mathematical_cap_pair_local_n", "thermal_included"}, "applied load shape")
            _require(_vector(loads[pid]["local_x_global"], 3) and all(_vector(loads[pid][k], 2) for k in ("eigenload_pair_local_n", "mathematical_cap_pair_local_n")) and type(loads[pid]["thermal_included"]) is bool, "applied load vectors")
        terminals = region["terminals"]
        _require(isinstance(terminals, list) and len(terminals) == 2, "closure terminals")
        for terminal in terminals:
            _shape(terminal, {"node_ref", "closure_transfer", "provenance", "closure_pressure_load_global_n", "pipe_cap_transfer_global_n", "remote_closure_excluded_from_pipe_solve", "remote_closure_support_reaction_global_n"}, "terminal shape")
            _require(_text(terminal["node_ref"]) and _text(terminal["provenance"]) and terminal["closure_transfer"] in {"transfers_to_wall", "separately_supported_or_compensated"} and all(_vector(terminal[k], 3) for k in ("closure_pressure_load_global_n", "pipe_cap_transfer_global_n")), "terminal closure")
            remote = terminal["closure_transfer"] == "separately_supported_or_compensated"
            _require(terminal["remote_closure_excluded_from_pipe_solve"] is remote and (_vector(terminal["remote_closure_support_reaction_global_n"], 3) if remote else terminal["remote_closure_support_reaction_global_n"] is None), "terminal remote closure")
        for pid in members:
            physical = [row for row in rows.values() if row.get("entity_ref") == pid and row.get("basis_ref") == {"ref_type": "load_case", "ref_id": cid} and row.get("kind") in PIPE_KINDS - {"pipe_elastic_normal_stress_maximum_v2"}]
            observed = [(row.get("metadata", {}).get("location"), row.get("metadata", {}).get("component")) for row in physical]
            expected_locations = {(station, component) for station in ("end_i", "end_j", "quarter_1", "midspan", "quarter_3") for component in ("wall_axial_force", "effective_axial_force", "axial_membrane_stress", "lame_inner_radial_stress", "lame_outer_radial_stress", "lame_inner_hoop_stress", "lame_outer_hoop_stress")} | {("end_i", "wall_axial_end_action"), ("end_j", "wall_axial_end_action")}
            _require(len(observed) == len(expected_locations) and set(observed) == expected_locations, "pressure station coverage")
        ids = region["result_ids"]
        expected = {row["id"] for row in rows.values() if row.get("kind") in PIPE_KINDS and row.get("entity_ref") in members and row.get("basis_ref") == {"ref_type": "load_case", "ref_id": cid}}
        _require(_strings(ids, True) and set(ids) == expected and not bindings.intersection(ids), "region result binding")
        bindings.update(ids)
    table = json.loads((Path(__file__).resolve().parents[2] / "fixtures/results/semantic_contract_v0_3_physics_1.json").read_text())
    support_components: dict[tuple[str, str], list[str]] = {}
    for row in rows.values():
        _require(_number(row.get("value")), "nonfinite source row")
        basis = row.get("basis_ref")
        _require(isinstance(basis, Mapping) and basis.get("ref_type") == "load_case" and basis.get("ref_id") in cases, "row case scope")
        if row.get("kind", "").endswith("_v2"):
            _require(row.get("kind") in PIPE_KINDS or row.get("kind", "").startswith("support_reaction_"), "unknown physical kind")
            metadata = row.get("metadata")
            _shape(metadata, {"component", "coordinate_system", "location", "basis", "sign_convention"}, "physical metadata shape")
            _require(_text(metadata["sign_convention"]) and len([entry for entry in table["rows"] if entry["kind"] == row["kind"] and entry["unit"] == row.get("unit") and entry["component"] == metadata["component"]]) == 1, "physical signature")
            support = row["kind"].startswith("support_reaction_")
            _require(metadata["sign_convention"] == (SUPPORT_SIGN if support else SIGNS[row["kind"]]), "physical sign convention")
            force = row["kind"] in {"pipe_wall_endpoint_action_v2", "pipe_wall_axial_force_v2", "pipe_effective_axial_force_v2"}
            _require(metadata["coordinate_system"] == ("global" if support else "element_local" if force else "pipe_section") and metadata["basis"] == ("recovered_from_assembled_support_law" if support else "recovered_from_local_element_stiffness" if force else "recovered_from_open_mechanics_stress_components"), "physical recovery basis")
            locations = {"node"} if support else {"governing_station"} if row["kind"] == "pipe_elastic_normal_stress_maximum_v2" else {"end_i", "end_j"} if row["kind"] == "pipe_wall_endpoint_action_v2" else {"end_i", "end_j", "quarter_1", "midspan", "quarter_3"}
            _require(metadata["location"] in locations and not row.get("source_result_refs"), "physical location/derivation")
            if support:
                support_components.setdefault((basis["ref_id"], row["entity_ref"]), []).append(metadata["component"])
                if metadata["component"] in {"force_magnitude", "moment_magnitude"}:
                    _require(row["value"] >= 0, "support magnitude sign")
        if row.get("kind") in PIPE_KINDS:
            cid, pid = basis["ref_id"], row.get("entity_ref")
            _require(pid in geometries[cid], "row member scope")
            if row["kind"] != "pipe_elastic_normal_stress_maximum_v2":
                _require((cid, pid) in owners and row["id"] in bindings, "unbound pressure row")
    for components in support_components.values():
        _require(len(components) == 8 and set(components) == {"Fx", "Fy", "Fz", "Mx", "My", "Mz", "force_magnitude", "moment_magnitude"}, "support component coverage")
    headline = source.get("summary", {}).get("max_open_formula_stress")
    complete = bool(cases) and all(case["stress_maximum_coverage"]["complete"] for case in cases.values())
    if not complete:
        _require(headline is None, "incomplete stress maximum headline")
    elif headline is not None:
        maximum_rows = [row for row in rows.values() if row.get("kind") == "pipe_elastic_normal_stress_maximum_v2"]
        selected = rows.get(headline.get("result_ref"), {})
        _require(bool(maximum_rows) and selected in maximum_rows and headline.get("value") == selected["value"] == max(row["value"] for row in maximum_rows) and headline.get("unit") == selected["unit"] and headline.get("location_ref") == selected["entity_ref"], "stress maximum headline binding")
    for cid, case in cases.items():
        for group in case["pressure_rhs_assembly"]["groups"]:
            for term in group["terms"]:
                _require(owners.get((cid, term["pipe_id"])) == term["region_id"], "RHS term region/member binding")
                _require(group["pressure_bits"] == struct.pack(">d", region_pressures[cid, term["region_id"]]).hex(), "RHS source pressure bits")
                geometry = geometries[cid][term["pipe_id"]]
                a, b = geometry["outside_diameter_m"] * 0.5, geometry["effective_wall_thickness_m"]
                hi = a - b
                b_virtual = a - hi
                a_virtual = hi + b_virtual
                lo = (a - a_virtual) + (b_virtual - b)
                _require(group["source_inner_radius_hi_bits"] == struct.pack(">d", hi).hex() and group["source_inner_radius_lo_bits"] == struct.pack(">d", 0.0 if lo == 0 else lo).hex(), "RHS source bore bits")
    diagnostics = _indexed(source.get("diagnostics"), "id", "diagnostic IDs")
    _require(not set(diagnostics).intersection(rows), "ambiguous numerical evidence ID")
    for case in quality:
        cid = case["basis_ref"]["ref_id"]
        _require(_strings(case["evidence_refs"]), "duplicate numerical evidence refs")
        for ref in case["evidence_refs"]:
            if ref in rows:
                _require(rows[ref].get("basis_ref") == case["basis_ref"], "numerical row case scope")
            else:
                _require(ref in diagnostics and cid in diagnostics[ref].get("affected_refs", []), "numerical diagnostic case scope")


def _temperature_basis(basis: Any, case_basis: str, material_id: str) -> None:
    _require(isinstance(basis, Mapping), "temperature basis")
    selection = basis.get("selection")
    if selection == "base_material":
        _shape(basis, {"selection"}, "temperature basis")
        _require(case_basis == "base_material_common_E_nu", "temperature case basis")
    elif selection == "exact_point":
        _shape(basis, {"selection", "point_id"}, "temperature basis")
        _require(_text(basis["point_id"]) and f"material={material_id};common_E_nu_basis=point:{basis['point_id']};G=E/[2(1+nu)];alpha_same_basis=" in case_basis, "temperature point binding")
    elif selection == "interpolation":
        _shape(basis, {"selection", "temperature_value", "temperature_unit"}, "temperature basis")
        _require(_number(basis["temperature_value"]) and basis["temperature_unit"] in {"K", "degC", "degF"} and f"material={material_id};common_E_nu_basis=interpolated:" in case_basis and ";temperature_kelvin=" in case_basis, "temperature interpolation binding")
    else:
        _require(False, "temperature selection")


def _validate_rhs(rhs: Any, cid: str) -> None:
    _shape(rhs, {"method", "load_case_id", "node_order", "dof_order", "dof_units", "assembled_pressure_rhs_global", "groups", "rounded_cap_rhs_global", "rounded_poisson_rhs_global", "rounded_cap_and_eigen_ledgers_are_observational", "cancellation_screen", "screen_limit", "screen_roundoff_multiplier", "screen_is_not_numerical_qualification"}, "pressure RHS shape")
    _require(rhs["method"] == "source_factor_grouped_pressure_rhs_v1" and rhs["load_case_id"] == cid and _strings(rhs["node_order"], True) and rhs["dof_order"] == ["Fx", "Fy", "Fz", "Mx", "My", "Mz"] and rhs["dof_units"] == ["N", "N", "N", "N*m", "N*m", "N*m"], "pressure RHS basis")
    _require(rhs["rounded_cap_and_eigen_ledgers_are_observational"] is True and rhs["screen_is_not_numerical_qualification"] is True and rhs["screen_limit"] == 1e-9 and rhs["screen_roundoff_multiplier"] == 32 and _number(rhs["cancellation_screen"]) and 0 <= rhs["cancellation_screen"] <= rhs["screen_limit"], "pressure RHS screen")
    _require(all(_vector(rhs[k], 6 * len(rhs["node_order"])) for k in ("assembled_pressure_rhs_global", "rounded_cap_rhs_global", "rounded_poisson_rhs_global")) and isinstance(rhs["groups"], list), "pressure RHS vectors")
    for group in rhs["groups"]:
        _shape(group, {"node_ref", "component", "pressure_bits", "source_inner_radius_hi_bits", "source_inner_radius_lo_bits", "direction_component_magnitude", "coefficient_sum", "assembled_force_n", "terms"}, "pressure RHS group")
        _require(group["node_ref"] in rhs["node_order"] and group["component"] in ["Fx", "Fy", "Fz"] and all(isinstance(group[k], str) and len(group[k]) == 16 and all(c in "0123456789abcdef" for c in group[k]) for k in ("pressure_bits", "source_inner_radius_hi_bits", "source_inner_radius_lo_bits")), "pressure RHS group source")
        _require(all(_number(group[k]) for k in ("direction_component_magnitude", "coefficient_sum", "assembled_force_n")) and 0 < group["direction_component_magnitude"] <= 1 and isinstance(group["terms"], list) and bool(group["terms"]), "pressure RHS group values")
        for term in group["terms"]:
            _shape(term, {"coefficient", "region_id", "pipe_id", "kind"}, "pressure RHS term")
            _require(_number(term["coefficient"]) and _text(term["region_id"]) and _text(term["pipe_id"]) and term["kind"] in {"poisson_eigen", "terminal_cap"}, "pressure RHS term values")
