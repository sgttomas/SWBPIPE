"""Finite exact-profile evidence and source joins; never producer authentication.

Test verifier (b7-plan R2): the product reader is Rust's (`core/reporting/result_export/src/physics_evidence.rs`), which
product Python reaches only through `openpipestress-runner read`. Tests only: no module outside `tests/` imports
this one, and it imports nothing from product code (`tests/test_verifier_independence.py`).

B7b (A4/A5): the current exact contract is `3.0.0/exact_pressure_v3` only. This module keeps the two readings
that are live under it: the composite reading of pressure-source-1's straight families (signed `p_pa`, with a
`SourceContract` context from `physics_source.py`), and the exact-retained G7 transport check on the receipt's
pressure-2 projection (`validate_transport_metadata`, results 0.3's `ExactRetainedContractEvidence`). pressure-2
itself is verified by `pressure_2.py`. The retired physics-1 reader (`2.0.0/exact_straight_pressure_v2`) and its
cross-read refusals are deleted with v2.

The published membrane and effective force values are source observations. This
reader validates their provenance relationships without reconstructing them from
rounded neighbouring rows or treating pressure arithmetic as a numerical pass.

T4-U6/U8 (DEL-05-03 ``section-bases-and-timoshenko.md``): each ``pipe_sections`` entry is the D-5
shape, rederived from its captured authored values by ``section_basis.py``, with its beam theory checked as
pressure-2's reader checks it (one straight-member shear model per document). The composite never carries a
realized arc (D7 declines every document with one): an arc region record or a ``pipe_sections`` entry marked
``member_kind = "realized_arc"`` is refused by name ("realized arc unsupported"; Rust
``SOURCE_PHYSICS_ARC_UNSUPPORTED``). Every stress row names its
section basis and has its other-basis twin; the twins are compared with each other against the two
sections' leaves (bit-equal where the bases coincide, else the same resultant within the 64-epsilon
representation guard), and Lame rows against their own annulus. That is a relation between twin rows and
the declared sections, not a reconstruction of a row from force rows. Coverage, maxima and headlines are
per basis; ``max_open_formula_stress`` is null on exact documents.
"""
from __future__ import annotations

import math
import struct
import json
from pathlib import Path
from collections.abc import Mapping
from typing import Any

from . import section_basis as sb
from .pressure_2 import ARC_GEOMETRY_KEYS, equivalent_static_record

PRESSURE_V3 = ("3.0.0", "exact_pressure_v3")
PRESSURE_ID = "openpipestress.result_semantics/0.3.0/pressure-2"
PRESSURE_PROFILE = "exact_pressure_v3"
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

# T4-U6 (D-5, §2.5): the pipe_sections entry's keys.
GEOMETRY = sb.SECTION_KEYS
MATERIAL = {"pipe_id", "material_id", "E_pa", "nu", "G_pa", "constitutive_basis", "thermal_consumed", "alpha_per_kelvin", "provenance"}
STRESS_COMPONENTS = {"axial_membrane_stress", "lame_inner_radial_stress", "lame_outer_radial_stress", "lame_inner_hoop_stress", "lame_outer_hoop_stress"}


def _require(condition: bool, detail: str) -> None:
    if not condition:
        raise ValueError(f"SOURCE_PHYSICS_EVIDENCE_INVALID: {detail}")


def _basis(check, *args):
    """A section-basis check's refusal as this verifier's own (``SOURCE_PHYSICS_EVIDENCE_INVALID: <detail>``)."""
    try:
        return check(*args)
    except sb.BasisError as error:
        raise ValueError(f"SOURCE_PHYSICS_EVIDENCE_INVALID: {error}") from error


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


def _validate_physics_evidence(source: Mapping[str, Any], *, context: Any, contract: tuple[str, str] = PRESSURE_V3) -> None:
    # Only pressure-source-1's composite reading, under its caller-selected context; pressure-2 is `pressure_2.py`'s.
    _require(context is not None and contract == PRESSURE_V3, "composite contract")
    _require("carrier_evidence" not in source, "unsupported source namespace")
    evidence = source.get("contract_evidence")
    resolved = isinstance(evidence, Mapping) and "load_reference_states" in evidence
    if resolved:
        from .load_reference_evidence import _prepass, PRESSURE_JOINED
        _prepass(source, True, PRESSURE_JOINED)
    _shape(evidence, {"pressure", "connector", "exact_cases"} | ({"load_reference_states"} if resolved else set()), "namespace")
    _finite_tree(evidence)
    _require(evidence["connector"] == [] and isinstance(evidence["pressure"], list), "unsupported connector")
    cases = _indexed(evidence["exact_cases"], "load_case_id", "exact cases")
    # T4-U2/T4-U8: the composite carries no realized arc (D7). A region geometry record with an arc key is refused by
    # name before anything else is read from it (Rust `arc_evidence::members` where arcs are not admitted).
    for region in evidence["pressure"]:
        geometry = region.get("geometry") if isinstance(region, Mapping) else None
        _require(not any(isinstance(g, Mapping) and ARC_GEOMETRY_KEYS & set(g) for g in (geometry if isinstance(geometry, list) else [])), "realized arc unsupported")
    rows = _indexed(source.get("results"), "id", "result IDs")
    quality = source.get("numerical_quality", {}).get("cases", [])
    numerical_ids = [case.get("basis_ref", {}).get("ref_id") for case in quality]
    _require(all(case.get("basis_ref", {}).get("ref_type") == "load_case" for case in quality) and len(set(numerical_ids)) == len(numerical_ids), "numerical case scope")
    if source.get("status", {}).get("mechanics") == "MECHANICS_SOLVED":
        _require(bool(cases) and set(cases) == set(numerical_ids), "case coverage")
    materials: dict[str, dict[str, Any]] = {}
    geometries: dict[str, dict[str, Any]] = {}
    sections: dict[tuple[str, str], dict[str, Any]] = {}
    complete = {basis: bool(cases) for basis in sb.BASES}
    straight_models: set[str] = set()
    extrema_ids: set[str] = set()
    for cid, case in cases.items():
        # W-16 slice c (DEL-05-01 Design exact-equivalent-static.md §2): a generating case's closed record,
        # never on a selected retained case.
        generation = isinstance(case, Mapping) and "equivalent_static" in case
        if generation:
            equivalent_static_record(cid, case["equivalent_static"])
            _require(context is None or case.get("recovery_method") != context.EXACT, "case shape")
        _shape(case, {"load_case_id", "profile_mode", "material_basis", "pipe_materials", "pipe_sections", "pipe_stress_extrema", "stress_maximum_coverage", "pressure_rhs_assembly"} | ({"recovery_method"} if context is not None else set()) | ({"equivalent_static"} if generation else set()), "case shape")
        _require(case["profile_mode"] == contract[1] and _text(case["material_basis"]), "case profile/material basis")
        materials[cid] = _indexed(case["pipe_materials"], "pipe_id", "case materials")
        geometries[cid] = _indexed(case["pipe_sections"], "pipe_id", "case sections")
        _require(bool(geometries[cid]) and set(materials[cid]) == set(geometries[cid]), "material/geometry coverage")
        for material in materials[cid].values():
            _shape(material, MATERIAL | ({"material_selection_kind", "resolved_eigenstrain"} if resolved else set()), "material shape")
            _require(_text(material["material_id"]) and _text(material["provenance"]) and material["constitutive_basis"] == "homogeneous_isotropic_E_nu_v1", "material basis")
            _require(all(_number(material[k]) for k in ("E_pa", "G_pa", "nu")) and material["E_pa"] > 0 and material["G_pa"] > 0 and -1 < material["nu"] < 0.5, "material range")
            expected_g = material["E_pa"] / (2 * (1 + material["nu"]))
            # Producer uses scaled arithmetic; permit only its final binary64
            # representation difference. Duplicated source records agree exactly.
            _require(math.isfinite(expected_g) and expected_g > 0 and abs(int.from_bytes(struct.pack(">d", material["G_pa"]), "big") - int.from_bytes(struct.pack(">d", expected_g), "big")) <= 2, "derived shear modulus")
            _require(type(material["thermal_consumed"]) is bool and (_number(material["alpha_per_kelvin"]) if material["thermal_consumed"] else material["alpha_per_kelvin"] is None), "thermal alpha")
        for pid, geometry in geometries[cid].items():
            # T4-U6 (D-5): the three geometries rederived from the captured authored values; T4-U8 (§3.1): kappa_E
            # from this case's nu; one straight shear model per document (§3.6). §2.5: a marked arc is refused by name.
            parsed, arc = _basis(sb.marked_section, geometry)
            _require(not arc, "realized arc unsupported")
            _basis(sb.beam_theory, parsed, materials[cid][pid]["nu"], False)
            straight_models.add(parsed["beam"]["model"])
            sections[cid, pid] = parsed
        _require(len(straight_models) <= 1, "one straight shear model per document")
        missing_by_basis: dict[str, set[str]] = {}
        for basis, (missing, done) in zip(sb.BASES, _basis(sb.coverage, case["stress_maximum_coverage"])):
            _require(missing <= set(geometries[cid]), "maximum coverage contradiction")
            complete[basis] = complete[basis] and done
            missing_by_basis[basis] = missing
        _require(isinstance(case["pipe_stress_extrema"], list), "extrema")
        extrema_by_basis: dict[str, dict[str, Any]] = {basis: {} for basis in sb.BASES}
        for extremum in case["pipe_stress_extrema"]:
            basis = _basis(sb.extremum_basis, extremum)
            _require(_text(extremum.get("pipe_id")) and extremum["pipe_id"] not in extrema_by_basis[basis], "duplicate extrema")
            extrema_by_basis[basis][extremum["pipe_id"]] = extremum
        for basis in sb.BASES:
            _require(set(extrema_by_basis[basis]) == set(geometries[cid]) - missing_by_basis[basis], "maximum member coverage")
        for pid, extremum in [(pid, ex) for basis in sb.BASES for pid, ex in extrema_by_basis[basis].items()]:
            if context is not None and case["recovery_method"] == context.EXACT:
                context.validate_maximum(source, case, extremum, rows)
                _require(extremum["result_id"] not in extrema_ids, "duplicate extrema result binding")
                extrema_ids.add(extremum["result_id"])
                continue
            _shape(extremum, sb.EXTREMUM_KEYS, "extrema shape")
            _require(_text(extremum["result_id"]) and extremum["result_id"] not in extrema_ids, "duplicate extrema result binding")
            extrema_ids.add(extremum["result_id"])
            row = rows.get(extremum["result_id"], {})
            row_basis = sb.split_token((row.get("metadata") or {}).get("basis"))
            _require(row.get("kind") == "pipe_elastic_normal_stress_maximum_v2" and row.get("entity_ref") == pid and row.get("basis_ref") == {"ref_type": "load_case", "ref_id": cid}
                     and row_basis is not None and row_basis[1] == extremum["section_basis"], "extrema result binding")
            _require(row.get("value") == extremum["value_lower_pa"] + 0.5 * (extremum["value_upper_pa"] - extremum["value_lower_pa"]), "extrema published witness")
            _require(all(_number(extremum[k]) for k in ("station_fraction", "local_fraction", "value_lower_pa", "value_upper_pa", "global_upper_bound_pa", "certified_gap_pa")), "extrema numeric values")
            _require(0 <= extremum["station_fraction"] <= 1 and 0 <= extremum["local_fraction"] <= 1 and 0 <= extremum["value_lower_pa"] <= extremum["value_upper_pa"] <= extremum["global_upper_bound_pa"] and 0 <= extremum["global_upper_bound_pa"] - extremum["value_lower_pa"] <= extremum["certified_gap_pa"] <= 1e-12 + 1e-12 * extremum["value_lower_pa"], "extrema bounds")
            _require(extremum["value_upper_pa"] == extremum["global_upper_bound_pa"], "extrema bounds")
            _require(type(extremum["span_index"]) is int and extremum["span_index"] >= 0 and type(extremum["subdivisions"]) is int and 0 <= extremum["subdivisions"] <= 131072 and extremum["approximation"] == "piecewise_quadratic_straight_section_statics" and extremum["coefficient_basis"] == "j_side_section_equilibrium_binary64_enclosures" and extremum["enclosure_scope"] == "supplied_binary64_polynomial_coefficient_enclosures; solution error is separate", "extrema witness")
        for pid, nominal in extrema_by_basis[sb.NOMINAL].items():
            if pid in extrema_by_basis[sb.REDUCED]:
                _basis(sb.extrema_twins, nominal, extrema_by_basis[sb.REDUCED][pid], sections[cid, pid])
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
        _require(region["profile_mode"] == contract[1] and region["profile_version"] == contract[0] and region["pressure_basis"] == "internal_differential_zero_external_v1" and region["external_pressure_increment_pa"] == 0 and _number(region["p_pa"]), "region pressure basis")
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
            if resolved:
                from .load_reference_evidence import REGION_TEMPERATURE_BASIS
                _require(mm[pid]["temperature_basis"] == REGION_TEMPERATURE_BASIS and all(mm[pid][key] == materials[cid][pid][key] for key in MATERIAL), "material disagreement")
            else:
                _temperature_basis(mm[pid]["temperature_basis"], cases[cid]["material_basis"], mm[pid]["material_id"])
                _require(isinstance(mm[pid]["temperature_basis"], Mapping) and _text(mm[pid]["temperature_basis"].get("selection")) and {k:v for k,v in mm[pid].items() if k != "temperature_basis"} == materials[cid][pid], "material disagreement")
            _shape(loads[pid], {"pipe_id", "eigenload_pair_local_n", "local_x_global", "mathematical_cap_pair_local_n", "thermal_included"}, "applied load shape")
            _require(_vector(loads[pid]["local_x_global"], 3) and all(_vector(loads[pid][k], 2) for k in ("eigenload_pair_local_n", "mathematical_cap_pair_local_n")) and loads[pid]["thermal_included"] is False, "applied load vectors")
        terminals = region["terminals"]
        _require(isinstance(terminals, list) and len(terminals) == 2 and _strings([terminal.get("node_ref") for terminal in terminals]), "closure terminals")
        for terminal in terminals:
            _shape(terminal, {"node_ref", "closure_transfer", "provenance", "closure_pressure_load_global_n", "pipe_cap_transfer_global_n", "remote_closure_excluded_from_pipe_solve", "remote_closure_support_reaction_global_n"}, "terminal shape")
            _require(_text(terminal["node_ref"]) and _text(terminal["provenance"]) and terminal["closure_transfer"] in {"transfers_to_wall", "separately_supported_or_compensated"} and all(_vector(terminal[k], 3) for k in ("closure_pressure_load_global_n", "pipe_cap_transfer_global_n")), "terminal closure")
            remote = terminal["closure_transfer"] == "separately_supported_or_compensated"
            _require(terminal["remote_closure_excluded_from_pipe_solve"] is remote and (_vector(terminal["remote_closure_support_reaction_global_n"], 3) if remote else terminal["remote_closure_support_reaction_global_n"] is None), "terminal remote closure")
            if remote:
                _require(all(value == 0 for value in terminal["pipe_cap_transfer_global_n"]) and terminal["remote_closure_support_reaction_global_n"] == [-value for value in terminal["closure_pressure_load_global_n"]], "terminal remote transfer disagreement")
            else:
                _require(terminal["pipe_cap_transfer_global_n"] == terminal["closure_pressure_load_global_n"], "terminal wall transfer disagreement")
        for pid in members:
            physical = [row for row in rows.values() if row.get("entity_ref") == pid and row.get("basis_ref") == {"ref_type": "load_case", "ref_id": cid} and row.get("kind") in PIPE_KINDS - {"pipe_elastic_normal_stress_maximum_v2"}]
            # T4-U6 (§2.1): a stress slot is published on both bases, a force slot once.
            observed = [(row.get("metadata", {}).get("location"), row.get("metadata", {}).get("component"),
                         (sb.split_token(row.get("metadata", {}).get("basis")) or (None, "unnamed"))[1] if row.get("kind") in sb.STRESS_KINDS else None) for row in physical]
            slots = {(station, component) for station in ("end_i", "end_j", "quarter_1", "midspan", "quarter_3") for component in ("wall_axial_force", "effective_axial_force", "axial_membrane_stress", "lame_inner_radial_stress", "lame_outer_radial_stress", "lame_inner_hoop_stress", "lame_outer_hoop_stress")} | {("end_i", "wall_axial_end_action"), ("end_j", "wall_axial_end_action")}
            expected_locations = {(station, component, basis) for station, component in slots for basis in (sb.BASES if component in STRESS_COMPONENTS else (None,))}
            _require(len(observed) == len(expected_locations) and set(observed) == expected_locations, "pressure station coverage")
        ids = region["result_ids"]
        expected = {row["id"] for row in rows.values() if row.get("kind") in PIPE_KINDS and row.get("entity_ref") in members and row.get("basis_ref") == {"ref_type": "load_case", "ref_id": cid}}
        _require(_strings(ids, True) and set(ids) == expected and not bindings.intersection(ids), "region result binding")
        bindings.update(ids)
    table = context.TABLE
    support_components: dict[tuple[str, str], list[str]] = {}
    physical_slots: set[tuple[str, str, str, str, str]] = set()
    for row in rows.values():
        _require(_number(row.get("value")), "nonfinite source row")
        basis = row.get("basis_ref")
        _shape(basis, {"ref_type", "ref_id"}, "row case reference")
        _require(basis["ref_type"] == "load_case" and basis["ref_id"] in cases, "row case scope")
        _require(all(_text(row.get(key)) for key in ("kind", "entity_ref", "unit")), "source row identity")
        _require(row.get("source_result_refs", []) == [], "unsupported direct-row derivation")
        # T4-U6 (§2.4): an inherited thin-wall row names no section basis; every stress row names its basis.
        _require(row.get("kind") not in sb.THIN_WALL_KINDS, "thin-wall row names no section basis")
        stress = row.get("kind") in sb.STRESS_KINDS
        section_basis = _basis(sb.row_basis, row)[1] if stress else None
        _require(not stress or (basis["ref_id"], row["entity_ref"]) in sections, "stress row member scope")
        if row.get("kind", "").endswith("_v2"):
            _require(row.get("kind") in PIPE_KINDS or row.get("kind", "").startswith("support_reaction_"), "unknown physical kind")
            metadata = row.get("metadata")
            _shape(metadata, {"component", "coordinate_system", "location", "basis", "sign_convention"}, "physical metadata shape")
            _require(_text(metadata["sign_convention"]) and len([entry for entry in table["rows"] if entry["kind"] == row["kind"] and entry["unit"] == row.get("unit") and entry["component"] == metadata["component"] and ("source_basis" not in entry or entry["source_basis"] == metadata["basis"])]) == 1, "physical signature")
            support = row["kind"].startswith("support_reaction_")
            _require(metadata["sign_convention"] == (context.MAX_SIGN if context is not None and row["kind"] == "pipe_elastic_normal_stress_maximum_v2" and cases[basis["ref_id"]]["recovery_method"] == context.EXACT else SUPPORT_SIGN if support else SIGNS[row["kind"]]), "physical sign convention")
            force = row["kind"] in {"pipe_wall_endpoint_action_v2", "pipe_wall_axial_force_v2", "pipe_effective_axial_force_v2"}
            declared = sb.split_token(metadata["basis"])[0] if stress else metadata["basis"]
            _require(metadata["coordinate_system"] == ("global" if support else "element_local" if force else "pipe_section") and declared == (context.MAX_BASIS if context is not None and row["kind"] == "pipe_elastic_normal_stress_maximum_v2" and cases[basis["ref_id"]]["recovery_method"] == context.EXACT else "recovered_from_assembled_support_law" if support else "recovered_from_local_element_stiffness" if force else "recovered_from_open_mechanics_stress_components"), "physical recovery basis")
            locations = {"node"} if support else {"governing_station"} if row["kind"] == "pipe_elastic_normal_stress_maximum_v2" else {"end_i", "end_j"} if row["kind"] == "pipe_wall_endpoint_action_v2" else {"end_i", "end_j", "quarter_1", "midspan", "quarter_3"}
            _require(metadata["location"] in locations, "physical location/derivation")
            slot = (basis["ref_id"], row["entity_ref"], row["kind"], metadata["component"], metadata["location"], section_basis)
            _require(slot not in physical_slots, "duplicate physical result slot")
            physical_slots.add(slot)
            if support:
                support_components.setdefault((basis["ref_id"], row["entity_ref"]), []).append(metadata["component"])
                if metadata["component"] in {"force_magnitude", "moment_magnitude"}:
                    _require(row["value"] >= 0, "support magnitude sign")
        if row.get("kind") in PIPE_KINDS:
            cid, pid = basis["ref_id"], row.get("entity_ref")
            _require(pid in geometries[cid], "row member scope")
            if row["kind"] == "pipe_elastic_normal_stress_maximum_v2":
                _require(row["id"] in extrema_ids, "unbound extrema row")
            else:
                _require((cid, pid) in owners and row["id"] in bindings, "unbound pressure row")
    for components in support_components.values():
        _require(len(components) == 8 and set(components) == {"Fx", "Fy", "Fz", "Mx", "My", "Mz", "force_magnitude", "moment_magnitude"}, "support component coverage")
    # T4-U6 (D-5, §2.1, §2.2): both families of every stress signature against the two sections; one headline per basis.
    _basis(sb.twins, rows, sections, {key: region_pressures[key[0], rid] for key, rid in owners.items()}, _twin_order(source))
    _u4b_straight(source, cases, geometries, materials, rows, table)
    _basis(sb.headlines, source.get("summary"), rows, complete)
    for cid, case in cases.items():
        for group in case["pressure_rhs_assembly"]["groups"]:
            for term in group["terms"]:
                _require(owners.get((cid, term["pipe_id"])) == term["region_id"], "RHS term region/member binding")
                expected_magnitude = 1 if term["kind"] == "terminal_cap" else abs(2 * materials[cid][term["pipe_id"]]["nu"])
                _require(abs(term["coefficient"]) == expected_magnitude, "RHS term coefficient binding")
                _require(group["pressure_bits"] == struct.pack(">d", region_pressures[cid, term["region_id"]]).hex(), "RHS source pressure bits")
                geometry = geometries[cid][term["pipe_id"]]
                # T4-U6 (D-5): the pressure load path is on the pressure bore (the reduced wall's inner radius).
                a, b = geometry["outside_diameter_m"] * 0.5, geometry["reduced_section"]["wall_m"]
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


def _twin_order(source: Mapping[str, Any]):
    """T4-U4b: D-5's row order (section-bases-and-timoshenko.md §2.5) on the composite, Rust's
    ``SOURCE_PHYSICS_ROW_BASIS_TWIN_ORDER`` at its place in the twin pass (after the twin is matched, before the pair's
    values): a U4b-family nominal row's reduced twin is the published row immediately after it. The arc maxima's
    clause is vacuous here, since the composite declines every realized arc."""
    from . import u4b
    published = source["results"]
    positions = {id(row): position for position, row in enumerate(published)}

    def check(row: Mapping[str, Any], twin: Mapping[str, Any]) -> None:
        if row["kind"] in u4b.KINDS:
            following = positions[id(row)] + 1
            if not (following < len(published) and published[following] is twin):
                raise ValueError("SOURCE_PHYSICS_ROW_BASIS_TWIN_ORDER")
    return check


def _u4b_straight(source: Mapping[str, Any], cases: Mapping[str, Any], geometries: Mapping[str, Mapping[str, Any]],
                  materials: Mapping[str, Mapping[str, Any]], rows: Mapping[str, Any], table: Mapping[str, Any]) -> None:
    """T4-U4b (§4.3, §5.1; S4 round 2): the straight-member derived rows of every case, exact-recovered and
    ordinary-method alike: their signatures and semantics, σ± and τ bit for bit (c_b from the case's ν and basis
    sections) and their station × basis coverage. Rust `physics_evidence::u4b_cases` on the composite, in its order
    and with its codes (``SOURCE_PHYSICS_EXTREME_*``, ``SOURCE_PHYSICS_SHEAR_*``)."""
    from . import u4b

    def fail(ok: bool, code: str) -> None:
        if not ok:
            raise ValueError(f"SOURCE_PHYSICS_{code}")
    for row in rows.values():
        if row.get("kind") in u4b.KINDS:
            metadata = row.get("metadata") or {}
            fail(row["kind"] in (u4b.EXTREME, u4b.SHEAR_MAX), "ROW_SIGNATURE")
            fail(len([entry for entry in table["rows"] if entry["kind"] == row["kind"] and entry["unit"] == row.get("unit") and entry["component"] == metadata.get("component") and entry.get("source_basis") == metadata.get("basis")]) == 1, "ROW_SIGNATURE")
            u4b.row_semantics(row, True, False)
    index = u4b.Rows(list(rows.values()))
    diagnostics = source.get("diagnostics", [])
    # S4 round 3: the composite's materials require -1 < ν < 1/2 ("material range"), so
    # TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN can never name one of its members: it is refused by name and exempts
    # nothing, so every member's τ coverage is checked.
    fail(not any(isinstance(d, Mapping) and d.get("code") == u4b.POISSON_OUT_OF_DOMAIN for d in diagnostics), "POISSON_DOMAIN_WARNING")
    for cid in cases:
        for pid, section in geometries[cid].items():
            nu = materials[cid][pid]["nu"]
            for basis in sb.BASES:
                factor = u4b.shear_factor(nu, section["ro_m"], section[basis]["ri_m"], section[basis]["I_m4"])
                for location in u4b.STATIONS:
                    y = index.value(cid, pid, "element_local_bending_normal_stress_y", "bending_normal_stress_y", location, basis)
                    z = index.value(cid, pid, "element_local_bending_normal_stress_z", "bending_normal_stress_z", location, basis)
                    n = index.value(cid, pid, "element_local_axial_normal_stress", "axial_normal_stress", location, basis)
                    if n is None:
                        membrane = index.value(cid, pid, "pipe_axial_membrane_stress_v2", "axial_membrane_stress", location, basis)
                        n = None if membrane is None else membrane / 1e6
                    plus = index.value(cid, pid, u4b.EXTREME, "sigma_plus", location, basis)
                    minus = index.value(cid, pid, u4b.EXTREME, "sigma_minus", location, basis)
                    if n is not None and y is not None and z is not None:
                        fail(plus is not None and minus is not None, "EXTREME_COVERAGE")
                        want = u4b.extremes(n, y, z)
                        fail(u4b.bits(plus) == u4b.bits(want[0]) and u4b.bits(minus) == u4b.bits(want[1]), "EXTREME_VALUE")
                    else:
                        fail(plus is None and minus is None, "EXTREME_COVERAGE")
                    vy = index.any(cid, pid, "element_local_shear_force_y", location)
                    vz = index.any(cid, pid, "element_local_shear_force_z", location)
                    tau = index.value(cid, pid, u4b.SHEAR_MAX, "tau_v_max", location, basis)
                    if vy is None or vz is None:
                        fail(tau is None, "SHEAR_COVERAGE")
                    else:
                        fail(tau is not None, "SHEAR_COVERAGE")
                        fail(tau >= 0, "SHEAR_NEGATIVE")
                        if factor is not None:
                            fail(u4b.bits(tau) == u4b.bits(u4b.shear_maximum(vy, vz, factor)), "SHEAR_VALUE")


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
    for key in ("assembled_pressure_rhs_global", "rounded_cap_rhs_global", "rounded_poisson_rhs_global"):
        _require(all(value == 0 for index, value in enumerate(rhs[key]) if index % 6 >= 3), "unsupported pressure moment")
    groups: set[tuple[Any, ...]] = set()
    for group in rhs["groups"]:
        _shape(group, {"node_ref", "component", "pressure_bits", "source_inner_radius_hi_bits", "source_inner_radius_lo_bits", "direction_component_magnitude", "coefficient_sum", "assembled_force_n", "terms"}, "pressure RHS group")
        _require(group["node_ref"] in rhs["node_order"] and group["component"] in ["Fx", "Fy", "Fz"] and all(isinstance(group[k], str) and len(group[k]) == 16 and all(c in "0123456789abcdef" for c in group[k]) for k in ("pressure_bits", "source_inner_radius_hi_bits", "source_inner_radius_lo_bits")), "pressure RHS group source")
        _require(all(_number(group[k]) for k in ("direction_component_magnitude", "coefficient_sum", "assembled_force_n")) and 0 < group["direction_component_magnitude"] <= 1 and isinstance(group["terms"], list) and bool(group["terms"]), "pressure RHS group values")
        key = (group["node_ref"], group["component"], group["pressure_bits"], group["source_inner_radius_hi_bits"], group["source_inner_radius_lo_bits"], group["direction_component_magnitude"])
        _require(key not in groups, "duplicate pressure RHS group")
        groups.add(key)
        _require(all(math.isfinite(struct.unpack(">d", bytes.fromhex(group[field]))[0]) for field in ("pressure_bits", "source_inner_radius_hi_bits", "source_inner_radius_lo_bits")), "nonfinite pressure RHS source bits")
        for term in group["terms"]:
            _shape(term, {"coefficient", "region_id", "pipe_id", "kind"}, "pressure RHS term")
            _require(_number(term["coefficient"]) and _text(term["region_id"]) and _text(term["pipe_id"]) and term["kind"] in {"poisson_eigen", "terminal_cap"}, "pressure RHS term values")


def validate_transport_metadata(source: Mapping[str, Any]) -> None:
    """Physical statement shape/internal bounds only; never creates raw row evidence."""
    from .source_blocks import _shape as schema_shape
    schema = json.loads((Path(__file__).resolve().parents[2] / "schemas/results.v0.3.schema.yaml").read_text())
    evidence = source.get("contract_evidence")
    _require(schema_shape(evidence, schema["$defs"]["ExactRetainedContractEvidence"], schema), "transport evidence shape")
    _finite_tree(evidence)
    cases = _indexed(evidence["exact_cases"], "load_case_id", "transport cases")
    result_ids: set[str] = set()
    regions: set[tuple[str,str]] = set()
    for cid, case in cases.items():
        materials = _indexed(case["pipe_materials"], "pipe_id", "transport materials")
        sections = _indexed(case["pipe_sections"], "pipe_id", "transport sections")
        _require(bool(materials) and set(materials) == set(sections), "transport member coverage")
        for material in materials.values():
            e, nu, g = material["E_pa"], material["nu"], material["G_pa"]
            _require(e > 0 and g > 0 and -1 < nu < .5, "transport material range")
            expected = e / (2 * (1 + nu))
            _require(math.isfinite(expected) and abs(int.from_bytes(struct.pack(">d", g), "big") - int.from_bytes(struct.pack(">d", expected), "big")) <= 2, "transport shear modulus")
        for section in sections.values():
            # T4-U6 (D-5): the three geometries rederived from the captured authored values.
            _basis(sb.section, section)
        maxima: dict[tuple[str, str], Any] = {}
        for basis, (missing, _) in zip(sb.BASES, _basis(sb.coverage, case["stress_maximum_coverage"])):
            _require(missing <= set(sections), "transport maximum coverage")
            own = [ex for ex in case["pipe_stress_extrema"] if _basis(sb.extremum_basis, ex) == basis]
            _require(len({ex["pipe_id"] for ex in own}) == len(own) and {ex["pipe_id"] for ex in own} == set(sections) - missing, "transport maximum members")
            maxima.update({(ex["pipe_id"], basis): ex for ex in own})
        for ex in maxima.values():
            _require(ex["result_id"] not in result_ids, "transport maximum result ID")
            result_ids.add(ex["result_id"])
            _require(0 <= ex["value_lower_pa"] <= ex["value_upper_pa"] <= ex["global_upper_bound_pa"] and 0 <= ex["global_upper_bound_pa"]-ex["value_lower_pa"] <= ex["certified_gap_pa"] <= 1e-12+1e-12*ex["value_lower_pa"], "transport maximum bounds")
        _validate_rhs(case["pressure_rhs_assembly"], cid)
    for region in evidence["pressure"]:
        cid, rid = region["load_case_id"], region["region_id"]
        _require(cid in cases and (cid,rid) not in regions and region["p_pa"] >= 0, "transport pressure region")
        regions.add((cid,rid))
        case = cases[cid]
        sections = _indexed(case["pipe_sections"], "pipe_id", "transport sections")
        materials = _indexed(case["pipe_materials"], "pipe_id", "transport materials")
        gm = _indexed(region["geometry"], "pipe_id", "transport region geometry")
        mm = _indexed(region["materials"], "pipe_id", "transport region materials")
        _require(_strings(region["member_pipe_ids"], True) and set(gm) == set(mm) == set(region["member_pipe_ids"]) and set(gm) <= set(sections), "transport region members")
        for pid in gm:
            _require({k:v for k,v in gm[pid].items() if k != "traversal_forward"} == sections[pid] and {k:v for k,v in mm[pid].items() if k != "temperature_basis"} == materials[pid], "transport region member facts")
            _temperature_basis(mm[pid]["temperature_basis"], case["material_basis"], mm[pid]["material_id"])
