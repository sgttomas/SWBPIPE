"""Independent test-only verifier of pressure-2 publications (3.0.0/exact_pressure_v3).

The Rust reader (``core/reporting/result_export``: ``physics_evidence``,
``arc_evidence``, ``connector_evidence``, ``load_reference``) is the only
product reader of ``openpipestress.result_semantics/0.3.0/pressure-2`` (owner,
2026-10-10: "move to one product reader in Rust"; "All readers."). The
desktop, Python product paths and the VP-STATIC ``exact_pressure_1`` gate take
that reader's verdict (``reader_verdict``'s ``pressure`` route, through
``openpipestress-runner read pressure``). This module checks the same
publications independently, for the shared corpora tests and the test-only
reader-verdict verifier only.

Independence: standard-library imports only, and its sibling test-only
``section_basis`` (itself standard-library only). It imports no product module and
no producer crate, neither directly nor through the subprocess bridges
(``core/serialization/canonical_json/adapter.py``, ``core/units/adapter.py``):
it needs neither canonical JSON nor unit conversion. It reads the pinned
pressure-2 table and, for transported statements only, the results 0.3 and
load/reference-state schemas, by path relative to the project root.

T4-U6/U8 (D-5, D-6): every ``pipe_sections`` entry states the nominal section,
the reduced section and the pressure bore, rederived from its captured authored
values in the fixed binary64 order, and its beam theory (kappa_E and A_s bit for
bit for a Timoshenko straight; arcs Euler-Bernoulli). A realized arc's entry,
and only an arc's, carries ``member_kind = "realized_arc"`` (§2.5), so every arc
is known without the model, in a pressure region or not. Every stress row is
published on both bases and names its basis; each twin pair is checked against
the two sections; each basis has its own maximum coverage and headline, and
``max_open_formula_stress`` is null.

What it verifies (internal source consistency only; never producer origin,
numerical accuracy or model freshness):

- the header: raw 0.2.0, the exact producer, the profile and limitations,
  numerical quality, and the foreign namespaces refused by name;
- straight pressure regions, sections, materials, the stress maxima and the
  v3 pressure RHS ledger (``poisson_eigen``, ``terminal_cap``,
  ``bend_cap_removed``, ``kink_remainder``), with signed ``p_pa``;
- realized arcs: region and member keys, the bend term, junctions, withheld
  rows and arc signs;
- objective connector records and their 24 rows per case, plus pe_x/s_x and (T4-U5b) one force row per tie rod;
- model-scoped review rows, the evidence keys and the load/reference-state
  pre-pass of an envelope that carries the group;
- B7b (A4/A5): the retired ``2.0.0/exact_straight_pressure_v2`` identities
  (physics-1, physics-source-1, load-reference-1, load-reference-source-1)
  are refused by name (``SOURCE_EXACT_PRESSURE_V2_RETIRED``), as the Rust
  header refuses them; there are no cross-read refusals any more.
"""
from __future__ import annotations

from collections.abc import Mapping
from copy import deepcopy
from fractions import Fraction
import hashlib
import json
import math
from pathlib import Path
import struct
from typing import Any

from . import component_body, section_basis as sb, u4b

PROJECT = Path(__file__).resolve().parents[2]

CONTRACT_ID = "openpipestress.result_semantics/0.3.0/pressure-2"
PROFILE = "exact_pressure_v3"
PRESSURE_V3 = ("3.0.0", "exact_pressure_v3")
PRODUCER = {"component_name": "open_pipe_stress_product_physics", "component_version": "0.2.0", "semantic_contract_id": CONTRACT_ID}
TABLE_PATH = PROJECT / "fixtures/results/semantic_contract_v0_3_pressure_2.json"
# T4-U6 (D-5, §2.4): one stress signature per (recovery token x section basis); T4-U5b adds the tie-rod force and
# T4-U4b its 22 per-basis signatures: 129 signatures over 66 kinds.
TABLE_SHA256 = "9183ef9b83f879620df45c03994f71808cc9c04c4a093c86b0bedfca2f96e59f"
TABLE_SIGNATURE_COUNT = 129
TABLE_KIND_COUNT = 66

# Refusals by name.
CONTRACT_UNKNOWN = "SOURCE_PHYSICS_PRESSURE_CONTRACT_UNKNOWN"
# B7b: the retired v2 identities, refused by name with the Rust header's text.
RETIRED_EXACT_V2_IDS = (
    "openpipestress.result_semantics/0.3.0/physics-1",
    "openpipestress.result_semantics/0.3.0/physics-source-1",
    "openpipestress.result_semantics/0.3.0/load-reference-1",
    "openpipestress.result_semantics/0.3.0/load-reference-source-1",
)
SOURCE_EXACT_PRESSURE_V2_RETIRED = "SOURCE_EXACT_PRESSURE_V2_RETIRED"
V2_RETIRED_TEXT = (f"{SOURCE_EXACT_PRESSURE_V2_RETIRED}: 2.0.0/exact_straight_pressure_v2 results are no longer read; "
                   "re-author the model as 0.7.0 with the current exact pressure contract and re-run the analysis")
# Model 0.6.0 (DEL-04-07 Design u4b-u5b-implementation.md §1.2): the retired pressure-1 identity, refused by name.
RETIRED_PRESSURE_1_ID = "openpipestress.result_semantics/0.3.0/pressure-1"
SOURCE_PRESSURE_1_RETIRED = "SOURCE_PRESSURE_1_RETIRED"
PRESSURE_1_RETIRED_TEXT = f"{SOURCE_PRESSURE_1_RETIRED}: pressure-1 results are no longer read; re-run the analysis"
RETAINED_METHOD = "contribution_preserving_multiprecision_v1"
RETAINED_PRECISION_DOWNGRADE_FORBIDDEN = "RETAINED_PRECISION_DOWNGRADE_FORBIDDEN"

# --------------------------------------------------------------------------
# Row and evidence vocabulary
# --------------------------------------------------------------------------

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

# Objective connector rows and records: kind -> (unit, components, locations).
CONNECTOR_KINDS = {
    "connector_generalized_translation_v1": ("m", ("qt_x", "qt_y", "qt_z"), ("connector_local",)),
    "connector_generalized_rotation_v1": ("rad", ("qr_x", "qr_y", "qr_z"), ("connector_local",)),
    "connector_generalized_force_v1": ("N", ("gt_x", "gt_y", "gt_z"), ("connector_local",)),
    "connector_generalized_moment_v1": ("N*m", ("gr_x", "gr_y", "gr_z"), ("connector_local",)),
    "connector_endpoint_force_v1": ("N", ("Fx", "Fy", "Fz"), ("end_i", "end_j")),
    "connector_endpoint_moment_v1": ("N*m", ("Mx", "My", "Mz"), ("end_i", "end_j")),
    "connector_pressure_force_v1": ("N", ("pe_x",), ("connector_local",)),
    "connector_effective_axial_force_v1": ("N", ("s_x",), ("connector_local",)),
    # T4-U5b (DEL-04-07 Design u4b-u5b-implementation.md §3.3): one force row per tie rod, bound by row ID.
    "connector_tie_rod_force_v1": ("N", ("t_r",), ("tie_rod",)),
}
CONNECTOR_ROWS_PER_CASE = 24
CONNECTOR_ROW_BASIS = "objective_connector_v1;symmetric_midpoint_small_rotation_v1"
CONNECTOR_LOCAL_SIGN = "generalized coordinates of the connector frame Q: q - q_ref and g = K(q - q_ref); positive along the connector axes"
CONNECTOR_END_SIGN = "global end action on the connector at its node (node on element), f = B^T g; the connector acts on its node with -f"
CONNECTOR_RECORD = {"component_id", "topology", "replaced_pipe_id", "node_i", "node_j", "motion_basis", "connector_axes_global", "end_i_node_axes_global", "end_j_node_axes_global", "end_i_offset_local_m", "end_j_offset_local_m", "q_ref", "reference_state", "work_matrix", "calibration", "hardware", "pressure_model", "temperature_applicability", "installed_reference_temperature_k", "provenance"}
CONNECTOR_MATRIX = {"representation", "coordinate_order", "translation_scale_m", "rotation_scale_rad", "coefficient_unit", "upper_triangle", "source_reference"}
TIE_ROD_KIND = "connector_tie_rod_force_v1"
TIE_ROD_BASIS = "recovered_from_tie_rod_extension_v1"
TIE_ROD_SIGN = "tension positive along the rod axis: T_r = k_r(e - e_ref), e = n.(v_bj - v_bi) with n from the end-i attachment to the end-j attachment; e_ref < 0 is pretension"
TIE_ROD_RECORD = {"id", "law", "offset_global_m", "stiffness_n_per_m", "reference_extension_m", "length_m", "direction_global", "axial_share", "source_reference"}

# Realized-arc members of a pressure region. A region with an arc adds
# ARC_REGION_KEYS; an arc member adds ARC_GEOMETRY_KEYS and its own applied-load
# record (the bend term).
ARC_MEMBER_KIND = "realized_arc"
ARC_APPROXIMATION = "straight_lame_annulus_and_realized_arc_member_term_h2_small_strain_v3"
JOINT_APPROXIMATION = "straight_lame_annulus_and_joint_area_difference_h2_small_strain_v3"
ARC_JOINT_APPROXIMATION = "straight_lame_annulus_realized_arc_and_joint_area_difference_h2_small_strain_v3"
ARC_TANGENCY_TOLERANCE_RAD = 1e-3
ARC_TANGENCY_RULE = "a region changes direction only at a realized bend; each straight run keeps the 64*epsilon collinearity guard (a straight-straight kink is PRESSURE_REGION_NONCOLLINEAR); at each bend-adjacent junction theta = atan2(|t_in x t_out|, t_in . t_out) <= alpha_tan is admitted and carried exactly by the remainder pAi(t_in - t_out), and a larger theta is refused as a mitre (PRESSURE_REGION_MITRE_UNSUPPORTED)"
# T4-U4b (§2.5): the straight Lame kinds stay off arcs; the region names the two arc definitions.
ARC_WITHHELD_KINDS = ["pipe_lame_radial_stress_v2", "pipe_lame_hoop_stress_v2"]
ARC_STRESS_DEFINITIONS = [
    {"definition": "bore_load_transferred_toroidal_membrane_v1", "statement": "the toroidal membrane hoop is the cut average of a complete torus in an axisymmetric membrane state under p on each basis bore, and the radial values are -p at the bore and 0 outside; ovalization, hoop bending from moments, bending boundary layers, tangent-end effects, the through-thickness distribution and transverse-shear resultants on the cuts are not represented"},
    {"definition": "local_straight_annulus_on_arc", "statement": "the transverse shear is the straight-annulus Saint-Venant field of each basis driven by the tangent-frame shears; the curvature correction of the shear field, ovalization and end effects are not represented"},
]
ARC_BEND_TERM = "K_b*u_free(eps_p)-c_b"
ARC_STRAIN_DEFINITION = "(1-2nu)pAi/(E As)"
ARC_REGION_KEYS = {"tangency_tolerance_rad", "tangency_rule", "bend_adjacent_junctions", "arc_stress_definitions"}
ARC_GEOMETRY_KEYS = {"member_kind", "arc_end_tangents_global", "bend_radius_m"}
ARC_APPLIED_KEYS = {"pipe_id", "bend_term", "arc_pressure_strain", "arc_pressure_strain_definition", "mathematical_cap_pair_local_n", "arc_end_tangents_global", "bend_cap_pair_removed_global_n", "thermal_included"}
ARC_JUNCTION_KEYS = {"node_ref", "pipe_in", "pipe_out", "t_in_global", "t_out_global", "theta_rad"}
ARC_SIGNS = {
    "pipe_wall_endpoint_action_v2": "node-on-element wall action along the arc end tangent (local x toward end j at that end); N_w = N_el + pAi; cap transfer is not subtracted from wall recovery",
    "pipe_wall_axial_force_v2": "tension-positive material wall section resultant Nw = N_el + pAi along the arc tangent; shear and moments are the elastic section actions",
    "pipe_effective_axial_force_v2": "effective wall-fluid resultant S=Nw-pAi (the arc's elastic axial force); not material stress or a support reaction",
    "pipe_axial_membrane_stress_v2": "tension-positive axial wall membrane stress Nw/As on the arc section; no added longitudinal pressure scalar",
}
STATIONS = ("end_i", "end_j", "quarter_1", "midspan", "quarter_3")
ARC_STATIONS = {("end_i", "wall_axial_end_action"), ("end_j", "wall_axial_end_action")} | {(station, component) for station in STATIONS for component in ("wall_axial_force", "effective_axial_force", "axial_membrane_stress")}
STRAIGHT_STATIONS = {(station, component) for station in STATIONS for component in ("wall_axial_force", "effective_axial_force", "axial_membrane_stress", "lame_inner_radial_stress", "lame_outer_radial_stress", "lame_inner_hoop_stress", "lame_outer_hoop_stress")} | {("end_i", "wall_axial_end_action"), ("end_j", "wall_axial_end_action")}
STRESS_COMPONENTS = {"axial_membrane_stress", "lame_inner_radial_stress", "lame_outer_radial_stress", "lame_inner_hoop_stress", "lame_outer_hoop_stress"}
REGION_KEYS = {"load_case_id", "region_id", "profile_mode", "profile_version", "pressure_basis", "p_pa", "external_pressure_increment_pa", "member_pipe_ids", "geometry", "materials", "terminals", "applied_loads", "approximation", "provenance", "geometry_representation_guard", "result_ids"}
# T4-U6 (D-5, §2.5): the pipe_sections entry's keys.
GEOMETRY = sb.SECTION_KEYS
MATERIAL = {"pipe_id", "material_id", "E_pa", "nu", "G_pa", "constitutive_basis", "thermal_consumed", "alpha_per_kelvin", "provenance"}
EVIDENCE_KEYS_0_3_0 = {"pressure", "connector", "exact_cases"}
EVIDENCE_KEYS_0_4_0 = EVIDENCE_KEYS_0_3_0 | {"load_reference_states"}


# --------------------------------------------------------------------------
# Entry points
# --------------------------------------------------------------------------

def verify_source(source: Mapping[str, Any], *, raw: bool = True) -> None:
    """The whole pressure-2 publication: header, then the raw evidence (or, for
    transported statements without raw rows, the closed transport shape)."""
    _header(source, raw=raw)
    if raw:
        validate_pressure_evidence(source)
    else:
        validate_pressure_transport_metadata(source)


def validate_pressure_evidence(source: Mapping[str, Any]) -> None:
    """Raw pressure-2 evidence. An envelope that carries ``load_reference_states``
    takes the load/reference-state pre-pass first; any other takes the physics
    checks directly."""
    evidence = source.get("contract_evidence") if isinstance(source, Mapping) else None
    if isinstance(evidence, Mapping) and "load_reference_states" in evidence:
        _load_state_evidence(source)
        return
    try:
        _physics(source)
    except (TypeError, KeyError, AttributeError, OverflowError) as error:
        raise ValueError("SOURCE_PHYSICS_EVIDENCE_INVALID: malformed evidence") from error


def validate_exact_pressure_evidence(source: Mapping[str, Any]) -> None:
    """Identity dispatch. pressure-2 is verified; a retired v2 identity is
    refused by name (``SOURCE_EXACT_PRESSURE_V2_RETIRED``), and so is the retired
    pressure-1 (``SOURCE_PRESSURE_1_RETIRED``); any other identity is refused as
    unknown."""
    producer = source.get("producer") if isinstance(source, Mapping) else None
    identity = producer.get("semantic_contract_id") if isinstance(producer, Mapping) else None
    if identity == CONTRACT_ID:
        basis = source.get("formulation_basis")
        _require(isinstance(basis, Mapping) and basis.get("profile_id") == PROFILE, "pressure-2 formulation profile")
        validate_pressure_evidence(source)
    elif identity in RETIRED_EXACT_V2_IDS:
        raise ValueError(V2_RETIRED_TEXT)
    elif identity == RETIRED_PRESSURE_1_ID:
        raise ValueError(PRESSURE_1_RETIRED_TEXT)
    else:
        raise ValueError(f"{CONTRACT_UNKNOWN}: {identity!r}")


def validate_pressure_transport_metadata(source: Mapping[str, Any]) -> None:
    """pressure-2 statements without raw rows: closed shape against the results
    0.3 ``PressureContractEvidence`` (held in ``results_pressure_2.v0.3``; load/reference-state
    records against the frozen load/reference-state schema) and internal joins
    only; never raw row evidence."""
    try:
        _transport(source)
    except (TypeError, KeyError, AttributeError, OverflowError, IndexError) as error:
        raise ValueError("SOURCE_PHYSICS_EVIDENCE_INVALID: malformed transport evidence") from error


def verify_pressure_table(data: bytes) -> dict[str, Any]:
    """The reviewed ordinary pressure table (Rust `semantic_contract::verify_pressure_table`): its bytes, then its
    identity, profile and counts (107 signatures, 60 kinds)."""
    if hashlib.sha256(data).hexdigest() != TABLE_SHA256:
        raise ValueError("SOURCE_PRESSURE_TABLE_HASH")
    table = json.loads(data)
    if (not isinstance(table, Mapping) or table.get("semantic_contract_id") != CONTRACT_ID or table.get("formulation_profile_id") != PROFILE
            or table.get("source_signature_count") != TABLE_SIGNATURE_COUNT or table.get("source_kind_count") != TABLE_KIND_COUNT):
        raise ValueError("SOURCE_PRESSURE_TABLE_IDENTITY")
    return table


def pressure_table() -> dict[str, Any]:
    return verify_pressure_table(TABLE_PATH.read_bytes())


# --------------------------------------------------------------------------
# Header
# --------------------------------------------------------------------------

QUALITY_STATUSES = {"not_assessed", "checks_passed", "sensitive", "unresolved", "failed"}


def _header(source: Any, *, raw: bool) -> None:
    if not isinstance(source, Mapping):
        raise ValueError("SOURCE_PRODUCER_CONTRACT_UNSUPPORTED")
    if "retained_precision" in source:
        raise ValueError(RETAINED_PRECISION_DOWNGRADE_FORBIDDEN)
    if "carrier_evidence" in source:
        raise ValueError("SOURCE_PRODUCER_CONTRACT_UNSUPPORTED")
    if source.get("schema_version") != "0.2.0":
        raise ValueError("SOURCE_SCHEMA_VERSION_UNSUPPORTED")
    if source.get("producer") != PRODUCER:
        raise ValueError("SOURCE_PRODUCER_CONTRACT_UNSUPPORTED")
    if "source_block_recovery" in source:
        raise ValueError("SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN")
    quality = source.get("numerical_quality")
    if (not isinstance(quality, Mapping) or set(quality) != {"value_representation", "publication_quantization", "integrity_policy", "status", "cases"}
            or quality.get("value_representation") != "finite_binary64" or quality.get("publication_quantization") != "none"
            or quality.get("integrity_policy") != "M03-INTEGRITY-v1" or not _member(quality.get("status"), QUALITY_STATUSES)
            or not isinstance(quality.get("cases"), list)):
        raise ValueError("SOURCE_NUMERICAL_QUALITY_INVALID")
    for case in quality["cases"]:
        basis = case.get("basis_ref") if isinstance(case, Mapping) else None
        if (not isinstance(case, Mapping) or set(case) != {"basis_ref", "structural_status", "solve_quality", "model_matrix_fidelity", "accuracy_evidence", "evidence_refs"}
                or not isinstance(basis, Mapping) or set(basis) != {"ref_type", "ref_id"} or not all(_text(value) for value in basis.values())
                or not _member(case.get("solve_quality"), QUALITY_STATUSES)
                or not _member(case.get("structural_status"), {"passive_model_basis", "physical_mechanism_witnessed", "negative_energy_witnessed", "numerically_unresolved"})
                or not _member(case.get("model_matrix_fidelity"), {"represented_equations_retained", "assembly_loss_detected", "assembly_uncertainty", "not_assessed"})
                or not _member(case.get("accuracy_evidence"), {"not_claimed", "reference_verified", "unresolved"})
                or not isinstance(case.get("evidence_refs"), list) or not all(_text(item) for item in case["evidence_refs"])):
            raise ValueError("SOURCE_NUMERICAL_CASE_INVALID")
    formulation = source.get("formulation_basis")
    if (not isinstance(formulation, Mapping) or set(formulation) != {"profile_id", "limitations"} or formulation.get("profile_id") != PROFILE
            or not isinstance(formulation.get("limitations"), list) or not formulation["limitations"] or not all(_text(item) for item in formulation["limitations"])):
        raise ValueError("SOURCE_FORMULATION_BASIS_UNSUPPORTED")
    rows = source.get("results")
    if raw and isinstance(rows, list) and any(isinstance(row, Mapping) and row.get("recovery_method") == RETAINED_METHOD for row in rows):
        raise ValueError(RETAINED_PRECISION_DOWNGRADE_FORBIDDEN)


def _member(value: Any, allowed: set[str]) -> bool:
    return isinstance(value, str) and value in allowed


# --------------------------------------------------------------------------
# Physics evidence (straight, arcs, connectors), contract 3.0.0/exact_pressure_v3
# --------------------------------------------------------------------------

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


def _float_bits(value: float) -> int:
    return int.from_bytes(struct.pack(">d", value), "big")


def arc_members(region: Any) -> set[str]:
    """The realized-arc members of one region; refused when malformed."""
    arcs: set[str] = set()
    for geometry in region.get("geometry", []) if isinstance(region, Mapping) and isinstance(region.get("geometry"), list) else []:
        if not isinstance(geometry, Mapping) or not ARC_GEOMETRY_KEYS & set(geometry):
            continue
        tangents = geometry.get("arc_end_tangents_global")
        radius = geometry.get("bend_radius_m")
        _require(geometry.get("member_kind") == ARC_MEMBER_KIND and isinstance(tangents, list) and len(tangents) == 2 and all(_vector(t, 3) for t in tangents)
                 and _number(radius) and radius > 0, "arc geometry")
        # T4-U8 (§2.5, §3.6): a realized arc is Euler-Bernoulli under either shear model.
        _require(geometry.get("beam_theory") == {"model": sb.EULER_BERNOULLI}, "arc beam theory")
        arcs.add(geometry.get("pipe_id"))
    return arcs


def _bits(vector: list[Any]) -> list[float]:
    """Exact binary64 values (a signed zero equals zero, as in Rust)."""
    return [float(x) for x in vector]


def _arc_region(region: Mapping[str, Any], arcs: set[str], p: float) -> None:
    """Region-level arc statements, arc applied loads and junctions."""
    _require(region["approximation"] in {ARC_APPROXIMATION, ARC_JOINT_APPROXIMATION} and region["tangency_tolerance_rad"] == ARC_TANGENCY_TOLERANCE_RAD and type(region["tangency_tolerance_rad"]) is float
             and region["tangency_rule"] == ARC_TANGENCY_RULE and region["arc_stress_definitions"] == ARC_STRESS_DEFINITIONS, "arc region profile")
    geometry = {g["pipe_id"]: g for g in region["geometry"]}
    loads = {item["pipe_id"]: item for item in region["applied_loads"]}
    directions = []
    for pid in region["member_pipe_ids"]:
        load, forward = loads[pid], geometry[pid]["traversal_forward"]
        if pid in arcs:
            tangents = geometry[pid]["arc_end_tangents_global"]
            _arc_applied(load, tangents, p)
            t_i, t_j = tangents
        else:
            _require(_vector(load.get("local_x_global"), 3), "arc applied load")
            t_i = t_j = load["local_x_global"]
        directions.append((pid, t_i, t_j) if forward else (pid, [-x for x in t_j], [-x for x in t_i]))
    expected = [k for k in range(1, len(directions)) if directions[k - 1][0] in arcs or directions[k][0] in arcs]
    junctions = region["bend_adjacent_junctions"]
    _require(isinstance(junctions, list) and len(junctions) == len(expected), "arc junctions")
    for junction, k in zip(junctions, expected):
        _require(isinstance(junction, Mapping) and set(junction) == ARC_JUNCTION_KEYS and _text(junction["node_ref"])
                 and junction["pipe_in"] == directions[k - 1][0] and junction["pipe_out"] == directions[k][0]
                 and _number(junction["theta_rad"]) and 0 <= junction["theta_rad"] <= ARC_TANGENCY_TOLERANCE_RAD, "arc junctions")
        _require(_vector(junction["t_in_global"], 3) and _vector(junction["t_out_global"], 3)
                 and _bits(junction["t_in_global"]) == _bits(directions[k - 1][2]) and _bits(junction["t_out_global"]) == _bits(directions[k][1]), "arc junction tangent")


def _arc_applied(load: Any, tangents: list[list[float]], p: float) -> None:
    _require(isinstance(load, Mapping) and set(load) == ARC_APPLIED_KEYS and load["bend_term"] == ARC_BEND_TERM and load["arc_pressure_strain_definition"] == ARC_STRAIN_DEFINITION
             and load["thermal_included"] is False and _vector(load["mathematical_cap_pair_local_n"], 2) and _number(load["arc_pressure_strain"])
             # (1-2nu) > 0 for an admitted nu, so eps_p has the sign of p.
             and (load["arc_pressure_strain"] > 0) == (p > 0) and (load["arc_pressure_strain"] < 0) == (p < 0), "arc applied load")
    given = load["arc_end_tangents_global"]
    _require(isinstance(given, list) and len(given) == 2 and all(_vector(t, 3) for t in given) and [_bits(t) for t in given] == [_bits(t) for t in tangents], "arc applied load")
    caps = load["mathematical_cap_pair_local_n"]
    removed = [[-v * caps[0] for v in tangents[0]], [-v * caps[1] for v in tangents[1]]]
    given = load["bend_cap_pair_removed_global_n"]
    _require(isinstance(given, list) and len(given) == 2 and all(_vector(t, 3) for t in given) and [_bits(t) for t in given] == [_bits(t) for t in removed], "arc applied load")


def _model_scoped_review(row: Mapping[str, Any], table: Mapping[str, Any]) -> bool:
    """An input review row without ``basis_ref`` whose pressure-2 signature is
    ``review_evidence`` is model scoped; every other row is case scoped."""
    if "basis_ref" in row:
        return False
    metadata = row.get("metadata") if isinstance(row.get("metadata"), Mapping) else {}
    # T4-U6: the basis joins the match where a signature declares one (never widening it).
    matches = [entry for entry in table["rows"] if entry["kind"] == row.get("kind") and entry["unit"] == row.get("unit") and entry.get("component") in (None, metadata.get("component"))
               and ("source_basis" not in entry or entry["source_basis"] == metadata.get("basis"))]
    return bool(matches) and all(entry["category"] == "review_evidence" for entry in matches)


def _basis(check, *args):
    """A section-basis check's refusal as this verifier's own (``SOURCE_PHYSICS_EVIDENCE_INVALID: <detail>``)."""
    try:
        return check(*args)
    except sb.BasisError as error:
        raise ValueError(f"SOURCE_PHYSICS_EVIDENCE_INVALID: {error}") from error


def _physics(source: Mapping[str, Any]) -> None:
    # Rust `physics_evidence::validate_physics_evidence_in` (ordinary, non-composite): a receipt or carrier namespace
    # is foreign method evidence, refused by its own code.
    if any(key in source for key in ("source_block_recovery", "carrier_evidence")):
        raise ValueError("SOURCE_PHYSICS_FOREIGN_METHOD_EVIDENCE")
    evidence = source.get("contract_evidence")
    _shape(evidence, EVIDENCE_KEYS_0_3_0, "namespace")
    _finite_tree(evidence)
    # W-15 slice d (DEL-03-05 Design §5): the component bodies and their reference-section records.
    bodies = component_body.source_bodies(source, pressure_table())
    _require(isinstance(evidence["connector"], list) and isinstance(evidence["pressure"], list), "unsupported connector")
    cases = _indexed(evidence["exact_cases"], "load_case_id", "exact cases")
    arcs_by_case: dict[str, set[str]] = {}
    # §2.5: each member a region's geometry records, by case; its record and its case section mark the same arcs.
    region_members: set[tuple[Any, Any]] = set()
    for region in evidence["pressure"]:
        cid = region.get("load_case_id") if isinstance(region, Mapping) else None
        arcs_by_case.setdefault(cid, set()).update(arc_members(region))
        geometry = region.get("geometry") if isinstance(region, Mapping) else None
        region_members |= {(cid, g["pipe_id"]) for g in (geometry if isinstance(geometry, list) else []) if isinstance(g, Mapping) and isinstance(g.get("pipe_id"), str)}
    # A case's structural members are its pipes and its objective connectors;
    # a replaced-span-only model is still covered.
    has_connector = bool(evidence["connector"])
    rows = _indexed(source.get("results"), "id", "result IDs")
    quality = source.get("numerical_quality", {}).get("cases", [])
    numerical_ids = [case.get("basis_ref", {}).get("ref_id") for case in quality]
    _require(all(case.get("basis_ref", {}).get("ref_type") == "load_case" for case in quality) and len(set(numerical_ids)) == len(numerical_ids), "numerical case scope")
    if source.get("status", {}).get("mechanics") == "MECHANICS_SOLVED":
        _require(bool(cases) and set(cases) == set(numerical_ids), "case coverage")
    materials: dict[str, dict[str, Any]] = {}
    geometries: dict[str, dict[str, Any]] = {}
    # T4-U6/U8: each case's checked sections; one coverage and one headline per basis; one straight shear model.
    sections: dict[tuple[str, str], dict[str, Any]] = {}
    complete = {basis: bool(cases) for basis in sb.BASES}
    straight_models: set[str] = set()
    extrema_ids: set[str] = set()
    arc_extrema: list[tuple[str, str, Any, Any]] = []
    # §2.5: the document's realized arcs, as every case's pipe_sections marks them (member_kind = "realized_arc"),
    # including arcs outside every pressure region; a model fact, so every case marks the same members.
    marked: set[str] | None = None
    for cid, case in cases.items():
        # T4-U4b (§2.6): intensified_measures only when non-empty.
        directional = "intensified_measures" in case
        _require(not directional or (isinstance(case["intensified_measures"], list) and case["intensified_measures"]), "case shape")
        _shape(case, {"load_case_id", "profile_mode", "material_basis", "pipe_materials", "pipe_sections", "pipe_stress_extrema", "stress_maximum_coverage", "pressure_rhs_assembly"} | ({"intensified_measures"} if directional else set()), "case shape")
        _require(case["profile_mode"] == PRESSURE_V3[1] and _text(case["material_basis"]), "case profile/material basis")
        materials[cid] = _indexed(case["pipe_materials"], "pipe_id", "case materials")
        geometries[cid] = _indexed(case["pipe_sections"], "pipe_id", "case sections")
        _require((bool(geometries[cid]) or has_connector) and set(materials[cid]) == set(geometries[cid]), "material/geometry coverage")
        for material in materials[cid].values():
            _shape(material, MATERIAL, "material shape")
            _require(_text(material["material_id"]) and _text(material["provenance"]) and material["constitutive_basis"] == "homogeneous_isotropic_E_nu_v1", "material basis")
            _require(all(_number(material[k]) for k in ("E_pa", "G_pa", "nu")) and material["E_pa"] > 0 and material["G_pa"] > 0 and -1 < material["nu"] < 0.5, "material range")
            expected_g = material["E_pa"] / (2 * (1 + material["nu"]))
            # The producer uses scaled arithmetic; permit only its final binary64
            # representation difference. Duplicated source records agree exactly.
            _require(math.isfinite(expected_g) and expected_g > 0 and abs(_float_bits(material["G_pa"]) - _float_bits(expected_g)) <= 2, "derived shear modulus")
            _require(type(material["thermal_consumed"]) is bool and (_number(material["alpha_per_kelvin"]) if material["thermal_consumed"] else material["alpha_per_kelvin"] is None), "thermal alpha")
        case_arcs: set[str] = set()
        for pid, geometry in geometries[cid].items():
            # T4-U6 (D-5): the three geometries rederived from the captured authored values; T4-U8 (§3.1): kappa_E
            # from this case's nu; §2.5: an arc is known by its marker, a region records a member as an arc exactly
            # when its case section is marked, and an arc is Euler-Bernoulli.
            parsed, arc = _basis(sb.marked_section, geometry)
            _require((cid, pid) not in region_members or arc == (pid in arcs_by_case.get(cid, set())), "arc marker")
            _basis(sb.beam_theory, parsed, materials[cid][pid]["nu"], arc)
            if arc:
                case_arcs.add(pid)
            else:
                straight_models.add(parsed["beam"]["model"])
            sections[cid, pid] = parsed
        _require(len(straight_models) <= 1, "one straight shear model per document")
        _require(marked is None or marked == case_arcs, "arc marker")
        marked = case_arcs if marked is None else marked
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
            _shape(component_body.without_record(extremum), sb.EXTREMUM_KEYS, "extrema shape")
            _require(_text(extremum["result_id"]) and extremum["result_id"] not in extrema_ids, "duplicate extrema result binding")
            extrema_ids.add(extremum["result_id"])
            row = rows.get(extremum["result_id"], {})
            row_basis = sb.split_token((row.get("metadata") or {}).get("basis"))
            _require(row.get("kind") == "pipe_elastic_normal_stress_maximum_v2" and row.get("entity_ref") == pid and row.get("basis_ref") == {"ref_type": "load_case", "ref_id": cid}
                     and row_basis is not None and row_basis[1] == extremum["section_basis"], "extrema result binding")
            _require(row.get("value") == extremum["value_lower_pa"] + 0.5 * (extremum["value_upper_pa"] - extremum["value_lower_pa"]), "extrema published witness")
            _require(all(_number(extremum[k]) for k in ("station_fraction", "local_fraction", "value_lower_pa", "value_upper_pa", "global_upper_bound_pa", "certified_gap_pa")), "extrema numeric values")
            _require(0 <= extremum["station_fraction"] <= 1 and 0 <= extremum["local_fraction"] <= 1 and 0 <= extremum["value_lower_pa"] <= extremum["value_upper_pa"] <= extremum["global_upper_bound_pa"] and 0 <= extremum["global_upper_bound_pa"] - extremum["value_lower_pa"] <= extremum["certified_gap_pa"] <= 1e-12 + 1e-12 * extremum["value_lower_pa"], "extrema bounds")
            # K-D5 (DEL-04-01 kd5-member-relative-bound.md section 3.2): the public
            # upper bound is the global maximum enclosure's.
            _require(extremum["value_upper_pa"] == extremum["global_upper_bound_pa"], "extrema bounds")
            if pid in case_arcs:
                # T4-U4b (§2.3): the arc record, checked with its station cover once the rows are read.
                arc_extrema.append((cid, pid, extremum, row))
                continue
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
    marked = marked or set()
    for region in evidence["pressure"]:
        arcs = arc_members(region)
        joints = region["approximation"] in {JOINT_APPROXIMATION, ARC_JOINT_APPROXIMATION}
        _shape(region, REGION_KEYS | (ARC_REGION_KEYS if arcs else set()) | ({"member_connector_ids", "joint_edges"} if joints else set()), "region shape")
        cid, rid = region["load_case_id"], region["region_id"]
        _require(cid in cases and _text(rid) and (cid, rid) not in regions, "region case/identity")
        regions.add((cid, rid))
        region_pressures[cid, rid] = region["p_pa"]
        # Signed p_pa under v3.
        _require(region["profile_mode"] == PRESSURE_V3[1] and region["profile_version"] == PRESSURE_V3[0] and region["pressure_basis"] == "internal_differential_zero_external_v1" and region["external_pressure_increment_pa"] == 0 and _number(region["p_pa"]), "region pressure basis")
        _require(_text(region["provenance"]) and (bool(arcs) or region["approximation"] in {"long_straight_annulus_small_strain_v2", JOINT_APPROXIMATION}) and region["geometry_representation_guard"] == {"epsilon_multiplier": 64, "meaning": "arithmetic_representation_only"}, "region provenance/guard")
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
            _shape(gm[pid], GEOMETRY | {"traversal_forward"} | (ARC_GEOMETRY_KEYS if pid in arcs else set()), "region geometry shape")
            # §2.5: an arc's record adds only its end tangents and bend radius to its case section, which carries the marker.
            _require(type(gm[pid]["traversal_forward"]) is bool and {k: v for k, v in gm[pid].items() if k not in {"traversal_forward", "arc_end_tangents_global", "bend_radius_m"}} == geometries[cid][pid], "geometry disagreement")
            _shape(mm[pid], MATERIAL | {"temperature_basis"}, "region material shape")
            _temperature_basis(mm[pid]["temperature_basis"], cases[cid]["material_basis"], mm[pid]["material_id"])
            _require(isinstance(mm[pid]["temperature_basis"], Mapping) and _text(mm[pid]["temperature_basis"].get("selection")) and {k: v for k, v in mm[pid].items() if k != "temperature_basis"} == materials[cid][pid], "material disagreement")
            if pid in arcs:
                continue  # the bend term's record (``_arc_region``)
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
        if arcs:
            _arc_region(region, arcs, region["p_pa"])
            _require(not any(row.get("kind") in ARC_WITHHELD_KINDS and row.get("entity_ref") in arcs and row.get("basis_ref") == {"ref_type": "load_case", "ref_id": cid} for row in rows.values()), "arc withheld")
        for pid in members:
            physical = [row for row in rows.values() if row.get("entity_ref") == pid and row.get("basis_ref") == {"ref_type": "load_case", "ref_id": cid} and row.get("kind") in PIPE_KINDS - {"pipe_elastic_normal_stress_maximum_v2"}]
            # T4-U6 (§2.1): a stress slot is published on both bases, a force slot once.
            observed = [(row.get("metadata", {}).get("location"), row.get("metadata", {}).get("component"),
                         (sb.split_token(row.get("metadata", {}).get("basis")) or (None, "unnamed"))[1] if row.get("kind") in sb.STRESS_KINDS else None) for row in physical]
            expected_locations = {(station, component, basis) for station, component in (ARC_STATIONS if pid in arcs else STRAIGHT_STATIONS)
                                  for basis in (sb.BASES if component in STRESS_COMPONENTS else (None,))}
            _require(len(observed) == len(expected_locations) and set(observed) == expected_locations, "pressure station coverage")
        ids = region["result_ids"]
        expected = {row["id"] for row in rows.values() if row.get("kind") in PIPE_KINDS and row.get("entity_ref") in members and row.get("basis_ref") == {"ref_type": "load_case", "ref_id": cid}}
        _require(_strings(ids, True) and set(ids) == expected and not bindings.intersection(ids), "region result binding")
        bindings.update(ids)
    table = pressure_table()
    support_components: dict[tuple[str, str], list[str]] = {}
    physical_slots: set[tuple[str, str, str, str, str]] = set()
    for row in rows.values():
        _require(_number(row.get("value")), "nonfinite source row")
        if _model_scoped_review(row, table):
            _require(all(_text(row.get(key)) for key in ("kind", "entity_ref", "unit")) and row.get("source_result_refs", []) == [], "source row identity")
            continue
        basis = row.get("basis_ref")
        _shape(basis, {"ref_type", "ref_id"}, "row case reference")
        _require(basis["ref_type"] == "load_case" and basis["ref_id"] in cases, "row case scope")
        _require(all(_text(row.get(key)) for key in ("kind", "entity_ref", "unit")), "source row identity")
        _require(row.get("source_result_refs", []) == [], "unsupported direct-row derivation")
        # T4-U6 (§2.4): an inherited thin-wall row names no section basis; every stress row names its basis (token and
        # id) and belongs to a member whose sections its case states.
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
            arc = row["entity_ref"] in marked
            _require(not (arc and row["kind"] in ARC_WITHHELD_KINDS), "arc withheld")
            arc_sign = u4b.ARC_MAXIMUM_SIGN if row["kind"] == "pipe_elastic_normal_stress_maximum_v2" else ARC_SIGNS.get(row["kind"])
            _require(metadata["sign_convention"] == (arc_sign if arc else SUPPORT_SIGN if support else SIGNS[row["kind"]]), "physical sign convention")
            force = row["kind"] in {"pipe_wall_endpoint_action_v2", "pipe_wall_axial_force_v2", "pipe_effective_axial_force_v2"}
            declared = sb.split_token(metadata["basis"])[0] if stress else metadata["basis"]
            _require(metadata["coordinate_system"] == ("global" if support else "element_local" if force else "pipe_section") and declared == ("recovered_from_assembled_support_law" if support else "recovered_from_local_element_stiffness" if force else "recovered_from_open_mechanics_stress_components"), "physical recovery basis")
            locations = {"node"} if support else {"governing_station"} if row["kind"] == "pipe_elastic_normal_stress_maximum_v2" else {"end_i", "end_j"} if row["kind"] == "pipe_wall_endpoint_action_v2" else set(STATIONS)
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
        # T4-U4b (§5.1): the new kinds' signatures and declared semantics.
        if row.get("kind") in u4b.KINDS:
            metadata = row.get("metadata") or {}
            _require(len([entry for entry in table["rows"] if entry["kind"] == row["kind"] and entry["unit"] == row.get("unit") and entry["component"] == metadata.get("component") and entry.get("source_basis") == metadata.get("basis")]) == 1, "physical signature")
            u4b.row_semantics(row, True, row["entity_ref"] in marked)
    _u4b(source, evidence, rows, sections, marked, arc_extrema)
    _twins(rows, sections, owners, region_pressures)
    _validate_connectors(evidence["connector"], evidence["pressure"], rows, {pid for cid in cases for pid in geometries[cid]}, set(cases), source.get("status", {}).get("mechanics") == "MECHANICS_SOLVED")
    for components in support_components.values():
        _require(len(components) == 8 and set(components) == {"Fx", "Fy", "Fz", "Mx", "My", "Mz", "force_magnitude", "moment_magnitude"}, "support component coverage")
    # T4-U6 (D-5, §2.2): no governing basis; one headline per basis.
    _basis(sb.headlines, source.get("summary"), rows, complete, bodies)
    for cid, case in cases.items():
        arcs = arcs_by_case.get(cid, set())
        joint_face_sources: set[tuple[str, str, str, str]] = set()
        for group in case["pressure_rhs_assembly"]["groups"]:
            for term in group["terms"]:
                if term["kind"] == "joint_effective_area":
                    region = next(r for r in evidence["pressure"] if r["load_case_id"] == cid and r["region_id"] == term["region_id"])
                    joint = next((record for record in evidence["connector"] if record["component_id"] == term["connector_id"]), None)
                    _require(joint is not None and term["connector_id"] in region["member_connector_ids"] and joint["pressure_model"] == "constant_effective_area_reference_v1", "RHS joint binding")
                    axis = ["Fx", "Fy", "Fz"].index(group["component"])
                    direction = joint["connector_axes_global"][axis][0]
                    _require(group["node_ref"] in (joint["node_i"], joint["node_j"]), "RHS joint node")
                    sign = -1 if group["node_ref"] == joint["node_i"] else 1
                    _require(group["direction_component_magnitude"] == abs(direction) and term["coefficient"] == sign * (-1 if direction < 0 else 1), "RHS joint direction")
                    _require(group["pressure_bits"] == struct.pack(">d", region["p_pa"]).hex() and group["effective_area_bits"] == struct.pack(">d", joint["effective_area_m2"]).hex(), "RHS joint source bits")
                    continue
                _require(owners.get((cid, term["pipe_id"])) == term["region_id"], "RHS term region/member binding")
                if term["kind"] == "bend_cap_removed":
                    _require(term["pipe_id"] in arcs and abs(term["coefficient"]) == 1, "arc RHS term")
                elif term["kind"] == "kink_remainder":
                    region = next(r for r in evidence["pressure"] if r["load_case_id"] == cid and r["region_id"] == term["region_id"])
                    _require(any(term["pipe_id"] in (j["pipe_in"], j["pipe_out"]) for j in region.get("bend_adjacent_junctions", [])) and abs(term["coefficient"]) == 1, "arc RHS term")
                else:
                    _require(not (term["kind"] == "poisson_eigen" and term["pipe_id"] in arcs), "arc RHS term")
                    expected_magnitude = 1 if term["kind"] in {"terminal_cap", "joint_face_cap"} else abs(2 * materials[cid][term["pipe_id"]]["nu"])
                    _require(abs(term["coefficient"]) == expected_magnitude, "RHS term coefficient binding")
                    if term["kind"] == "joint_face_cap":
                        region = next(r for r in evidence["pressure"] if r["load_case_id"] == cid and r["region_id"] == term["region_id"])
                        node = group["node_ref"]
                        identity = (term["region_id"], term["pipe_id"], node, group["component"])
                        _require(identity not in joint_face_sources, "duplicate RHS joint face")
                        joint_face_sources.add(identity)
                        edge = next((edge for edge in region.get("joint_edges", []) if (edge["face_i_pipe_id"] == term["pipe_id"] and edge["node_i_ref"] == node) or (edge["face_j_pipe_id"] == term["pipe_id"] and edge["node_j_ref"] == node)), None)
                        _require(edge is not None, "RHS joint face binding")
                        joint = next(record for record in evidence["connector"] if record["component_id"] == edge["connector_id"])
                        expected = joint["connector_axes_global"][["Fx", "Fy", "Fz"].index(group["component"])][0] * (1 if edge["node_i_ref"] == node else -1)
                        _require(abs(term["coefficient"] * group["direction_component_magnitude"] - expected) <= 64 * math.ulp(1.0), "RHS joint face direction")
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


def _u4b(source: Mapping[str, Any], evidence: Mapping[str, Any], rows: Mapping[str, Any], sections: Mapping[tuple[str, str], Any],
         marked: set[str], arc_extrema: list[tuple[str, str, Any, Any]]) -> None:
    """T4-U4b (§5.1) on pressure-2: σ±, τ (bit for bit with the case's ν and basis sections), hoop and radial
    (bit-equal from the region's p and R); the arc maxima with their station cover; the directional records."""
    results = source.get("results", [])
    index = u4b.Rows(results)
    u4b.twin_order(results, marked)
    diagnostics = source.get("diagnostics", [])
    covers: dict[tuple[str, str, str], list[float]] = {}
    # Review item 3: the exact route admits only -1 < nu < 1/2, so no Poisson exemption; every basis forms its c_b.
    _require(not any(d.get("code") == u4b.POISSON_OUT_OF_DOMAIN for d in diagnostics), "transverse-shear Poisson exemption on pressure-2")
    for case in evidence["exact_cases"]:
        cid = case["load_case_id"]
        nus = {m["pipe_id"]: m["nu"] for m in case["pipe_materials"]}
        pressurized = {}
        for region in evidence["pressure"]:
            if region["load_case_id"] == cid:
                for g in region["geometry"]:
                    if "bend_radius_m" in g:
                        pressurized[g["pipe_id"]] = (region["p_pa"], g["bend_radius_m"])
        for section in case["pipe_sections"]:
            pid = section["pipe_id"]
            ro = section["ro_m"]
            factors = {basis: u4b.shear_factor(nus[pid], ro, section[basis]["ri_m"], section[basis]["I_m4"]) for basis in sb.BASES}
            toroidal = None
            if pid in marked and pid in pressurized:
                p, r = pressurized[pid]
                toroidal = {basis: (p, section[basis]["ri_m"], section[basis]["wall_m"], ro, r) for basis in sb.BASES}
                if not r > ro:
                    _require(any(d.get("code") == u4b.TOROIDAL_DOMAIN and d.get("affected_refs", [None, None])[:2] == [cid, pid] for d in diagnostics), "toroidal domain warning")
            _require(all(factor is not None for factor in factors.values()), "transverse-shear factor outside the domain")
            for basis, cover in u4b.member_stations(index, cid, pid, list(sb.BASES), factors, False, toroidal).items():
                covers[cid, pid, basis] = cover
        records = case.get("intensified_measures", [])
        bound = u4b.directional_records(cid, records, rows, index, marked, True)
        _require(len(bound) == len(records), "intensified record shape")
        for row in results:
            if row.get("kind") in (u4b.DIRECTIONAL, u4b.DIRECTIONAL_MAX) and row.get("basis_ref", {}).get("ref_id") == cid:
                _require(row["id"] in bound, "unbound directional row")
    for cid, pid, record, row in arc_extrema:
        _require((row.get("metadata") or {}).get("sign_convention") == u4b.ARC_MAXIMUM_SIGN, "arc maximum sign")
        u4b.maximum_record(record, row, u4b.ARC_TOKENS[0], covers.get((cid, pid, record["section_basis"]), []))


def _twins(rows: Mapping[str, Any], sections: Mapping[tuple[str, str], Any], owners: Mapping[tuple[str, str], str],
           region_pressures: Mapping[tuple[str, str], float]) -> None:
    """T4-U6 (D-5, §2.1, §2.5): both families of every stress signature (``section_basis.twins``)."""
    _basis(sb.twins, rows, sections, {key: region_pressures[key[0], rid] for key, rid in owners.items()})


def _connector_matrix3(value: Any) -> bool:
    return isinstance(value, list) and len(value) == 3 and all(_vector(row, 3) for row in value)


def _stable_suffix(value: str) -> str:
    return value.replace(":", "-")


def _tie_rod_tail(component: str, rod: str) -> str:
    """A rod force row's base ID without ``result:`` (the producer's form)."""
    return f"connector:{_stable_suffix(component)}:tie-rod-force:tie_rod:{_stable_suffix(rod)}:t_r"


def _is_tie_rod_row_id(row_id: Any, case: str, tail: str) -> bool:
    """The first case's row carries ``result:{tail}``; any other case's ``result:loadcase:{case}:{tail}``."""
    return isinstance(row_id, str) and row_id in (f"result:{tail}", f"result:loadcase:{_stable_suffix(case)}:{tail}")


def _tie_rods(record: Mapping[str, Any]) -> list[tuple[str, float]]:
    """T4-U5b (§3.3, §5.1): a record's closed ``tie_rods`` entries, as (id, axial share). The axial share is
    RN(n.Q_x), recomputed here from the published direction and the record's Q as an exact rational."""
    rods = record.get("tie_rods")
    _require(isinstance(rods, list) and rods, "connector tie-rod record")
    ids: set[str] = set()
    encoded: set[str] = set()
    out = []
    for rod in rods:
        _shape(rod, TIE_ROD_RECORD, "connector tie-rod record")
        _require(_text(rod["id"]) and rod["id"] not in ids and _stable_suffix(rod["id"]) not in encoded, "connector tie-rod duplicate")
        ids.add(rod["id"])
        encoded.add(_stable_suffix(rod["id"]))
        offsets = rod["offset_global_m"]
        _require(rod["law"] == "bilateral_linear" and isinstance(offsets, list) and len(offsets) == 2 and all(_vector(o, 3) for o in offsets)
                 and _number(rod["stiffness_n_per_m"]) and rod["stiffness_n_per_m"] > 0 and _number(rod["reference_extension_m"])
                 and _number(rod["length_m"]) and rod["length_m"] > 0 and _vector(rod["direction_global"], 3) and _text(rod["source_reference"]),
                 "connector tie-rod record")
        n = rod["direction_global"]
        _require(abs(sum(v * v for v in n) - 1.0) <= 16 * math.ulp(1.0), "connector tie-rod direction")
        axis = [record["connector_axes_global"][k][0] for k in range(3)]
        share = rod["axial_share"]
        _require(_number(share) and float(sum(Fraction(n[k]) * Fraction(axis[k]) for k in range(3))) == share, "connector tie-rod axial share")
        out.append((rod["id"], share))
    return out


def _effective_axial_binding(s: float, g: float, pe: float, rods: list[tuple[float, float]]) -> bool:
    """§5.1: |s_x - Sigma| <= (64 + n_r)eps(|g_x| + |Pe| + sum|t_r*share|) + (n_r + 2)*2^-1074, with
    Sigma = g_x - Pe + sum_r RN(t_r*share_r) summed as exact rationals and rounded once."""
    products = [t * share for t, share in rods]
    if not all(math.isfinite(v) for v in [s, g, pe, *products]):
        return False
    try:
        expected = float(Fraction(g) - Fraction(pe) + sum((Fraction(p) for p in products), Fraction(0)))
    except OverflowError:
        return False
    coefficient = (64 + len(rods)) * math.ulp(1.0)
    tolerance = coefficient * abs(g) + coefficient * abs(pe) + sum(coefficient * abs(p) for p in products) + (len(rods) + 2) * math.ulp(0.0)
    return math.isfinite(tolerance) and abs(s - expected) <= tolerance


def _validate_connectors(records: Any, regions: Any, rows: Mapping[str, Any], members: set[str], cases: set[str], solved: bool) -> None:
    """``contract_evidence.connector`` records and the rows bound to them."""
    _require(isinstance(records, list), "unsupported connector")
    spans: dict[str, str] = {}
    for record in records:
        constant = record.get("pressure_model") == "constant_effective_area_reference_v1"
        tied = record.get("hardware") == "explicit_axial_rods_v1"
        _require(not (record.get("hardware") == "untied" and "tie_rods" in record), "connector untied record with tie rods")
        _shape(record, CONNECTOR_RECORD | ({"effective_area_m2", "pressure_basis", "pressure_source_reference"} if constant else set()) | ({"tie_rods"} if tied else set()), "connector record shape")
        if constant:
            _require(_number(record["effective_area_m2"]) and record["effective_area_m2"] > 0 and record["pressure_basis"] == "internal_differential_zero_external_v1" and _text(record["pressure_source_reference"]), "connector pressure model")
        _require(all(_text(record[k]) for k in ("component_id", "replaced_pipe_id", "node_i", "node_j", "provenance")) and record["node_i"] != record["node_j"], "connector record identity")
        _require(record["topology"] == "replaces_span" and record["motion_basis"] == "symmetric_midpoint_small_rotation_v1" and record["calibration"] == "constant_structural_elasticity_v1"
                 and record["hardware"] in {"untied", "explicit_axial_rods_v1"} and record["pressure_model"] in {"unpressurized", "constant_effective_area_reference_v1"} and record["temperature_applicability"] == "fixed_installed_parameters_v1"
                 and record["reference_state"] in {"stress_free", "prestressed"}, "connector record law")
        _require(all(_connector_matrix3(record[k]) for k in ("connector_axes_global", "end_i_node_axes_global", "end_j_node_axes_global"))
                 and _vector(record["end_i_offset_local_m"], 3) and _vector(record["end_j_offset_local_m"], 3) and _vector(record["q_ref"], 6)
                 and _number(record["installed_reference_temperature_k"]) and record["installed_reference_temperature_k"] > 0, "connector record frame")
        matrix = record["work_matrix"]
        _require(isinstance(matrix, Mapping) and set(matrix) == CONNECTOR_MATRIX and matrix["representation"] == "scaled_work_coefficients_v1"
                 and matrix["coordinate_order"] == ["tx", "ty", "tz", "rx", "ry", "rz"] and _number(matrix["translation_scale_m"]) and matrix["translation_scale_m"] > 0
                 and _number(matrix["rotation_scale_rad"]) and matrix["rotation_scale_rad"] == 1 and matrix["coefficient_unit"] == "N*m"
                 and _vector(matrix["upper_triangle"], 21) and _text(matrix["source_reference"]), "connector work matrix")
        if tied:
            _tie_rods(record)
        _require(record["component_id"] not in spans, "duplicate connector record")
        _require(record["replaced_pipe_id"] not in spans.values(), "duplicate connector replaced span")
        _require(record["replaced_pipe_id"] not in members, "connector replaced span published")
        spans[record["component_id"]] = record["replaced_pipe_id"]
    replaced = set(spans.values())
    by_id = {record["component_id"]: record for record in records}
    slots: set[tuple[str, str, str, str, str, str]] = set()
    counts: dict[tuple[str, str], int] = {}
    for row in rows.values():
        _require(row.get("entity_ref") not in replaced, "connector replaced span published")
        kind = row.get("kind", "")
        if not kind.startswith("connector_"):
            continue
        _require(kind in CONNECTOR_KINDS, "unknown connector kind")
        _require(row.get("entity_ref") in spans, "unbound connector row")
        unit, components, locations = CONNECTOR_KINDS[kind]
        metadata = row.get("metadata")
        _shape(metadata, {"component", "coordinate_system", "location", "basis", "sign_convention"}, "connector metadata shape")
        local = metadata["location"] == "connector_local"
        rod = kind == TIE_ROD_KIND
        _require(row.get("unit") == unit and metadata["component"] in components and metadata["location"] in locations
                 and metadata["coordinate_system"] == ("element_local" if rod else "connector_axes_q" if local else "global")
                 and metadata["basis"] == (TIE_ROD_BASIS if rod else CONNECTOR_ROW_BASIS)
                 and metadata["sign_convention"] == (TIE_ROD_SIGN if rod else CONNECTOR_LOCAL_SIGN if local else CONNECTOR_END_SIGN), "connector row semantics")
        case = row["basis_ref"]["ref_id"]
        _require(case in cases, "connector row case")
        rod_id = ""
        if rod:
            entries = by_id[row["entity_ref"]].get("tie_rods") or []
            bound = [entry["id"] for entry in entries if isinstance(entry, Mapping) and _is_tie_rod_row_id(row.get("id"), case, _tie_rod_tail(row["entity_ref"], entry["id"]))]
            _require(len(bound) == 1, "connector tie-rod row unbound")
            rod_id = bound[0]
        slot = (case, row["entity_ref"], kind, metadata["component"], metadata["location"], rod_id)
        _require(slot not in slots, "duplicate connector row")
        slots.add(slot)
        counts[(case, row["entity_ref"])] = counts.get((case, row["entity_ref"]), 0) + 1
    if solved:
        for record in records:
            constant = record["pressure_model"] == "constant_effective_area_reference_v1"
            rod_ids = [entry["id"] for entry in record.get("tie_rods") or []]
            for case in cases:
                # A rod record without its rows is refused by name (§5.1).
                _require(all((case, record["component_id"], TIE_ROD_KIND, "t_r", "tie_rod", rid) in slots for rid in rod_ids), "connector tie-rod rows missing")
                expected = CONNECTOR_ROWS_PER_CASE + int(constant) + int(constant or bool(rod_ids)) + len(rod_ids)
                _require(counts.get((case, record["component_id"])) == expected, "connector row coverage")
    _joint_pressure_bindings(records, regions, rows, cases, solved)


def _joint_projection(value: float, expected: float, first: float, second: float, tiny_count: int) -> bool:
    tolerance = (64 * math.ulp(1.0) * abs(first) + 64 * math.ulp(1.0) * abs(second) + tiny_count * math.ulp(0.0))
    return math.isfinite(value) and math.isfinite(expected) and math.isfinite(tolerance) and abs(value - expected) <= tolerance


def _joint_pressure_bindings(records: Any, regions: Any, rows: Mapping[str, Any], cases: set[str], solved: bool) -> None:
    by_id = {record["component_id"]: record for record in records}
    participation: dict[tuple[str, str], Any] = {}
    for region in regions:
        if "member_connector_ids" not in region and "joint_edges" not in region:
            continue
        ids, edges = region.get("member_connector_ids"), region.get("joint_edges")
        _require(_strings(ids, True) and isinstance(edges, list) and len(ids) == len(edges), "joint edge coverage")
        seen: set[str] = set()
        for edge in edges:
            _shape(edge, {"connector_id", "node_i_ref", "node_j_ref", "face_i_pipe_id", "face_j_pipe_id", "p_pa", "effective_area_m2", "face_bore_area_m2", "joint_pressure_force_n", "area_difference_force_n", "attachment_pair_global_n", "representation"}, "joint edge shape")
            cid, jid = region["load_case_id"], edge["connector_id"]
            _require(jid in ids and jid in by_id and jid not in seen and (cid, jid) not in participation, "joint edge binding")
            seen.add(jid)
            record = by_id[jid]
            _require(record["pressure_model"] == "constant_effective_area_reference_v1", "joint edge law")
            pressure, ae = region["p_pa"], record["effective_area_m2"]
            pe = pressure * ae
            _require(edge["p_pa"] == pressure and edge["effective_area_m2"] == ae and _number(edge["joint_pressure_force_n"]) and edge["joint_pressure_force_n"] == pe and record["pressure_basis"] == region["pressure_basis"]
                     and edge["node_i_ref"] == record["node_i"] and edge["node_j_ref"] == record["node_j"] and edge["representation"] == "B: p(Ae - Ai) at the attachments; joint_effective_area + joint_face_cap", "joint edge law")
            _require(_vector(edge["face_bore_area_m2"], 2) and _vector(edge["area_difference_force_n"], 2) and isinstance(edge["attachment_pair_global_n"], list) and len(edge["attachment_pair_global_n"]) == 2 and all(_vector(pair, 3) for pair in edge["attachment_pair_global_n"]), "joint pair shape")
            _require(all(value == 0 for key in ("end_i_offset_local_m", "end_j_offset_local_m") for value in record[key]), "joint pressure offset")
            geometry = _indexed(region["geometry"], "pipe_id", "joint face geometry")
            for end, face in enumerate((edge["face_i_pipe_id"], edge["face_j_pipe_id"])):
                _require(face in region["member_pipe_ids"] and face in geometry and "member_kind" not in geometry[face] and not any(terminal["node_ref"] == edge["node_i_ref" if end == 0 else "node_j_ref"] for terminal in region["terminals"]), "joint face binding")
                # T4-U6 (D-5, §1, §2.3): the joint face p(Ae - Ai) acts on the pressure bore; the region geometry
                # carries Ai only there.
                bore = geometry[face].get("pressure_bore")
                ai = bore.get("Ai_m2") if isinstance(bore, Mapping) else None
                _require(_number(ai), "joint face area")
                _require(edge["face_bore_area_m2"][end] == ai, "joint face area")
                pi = pressure * ai
                _require(_joint_projection(edge["area_difference_force_n"][end], pe - pi, pe, pi, 2), "joint area difference")
                _require(all(_joint_projection(edge["attachment_pair_global_n"][end][axis], (-1 if end == 0 else 1) * (pe - pi) * record["connector_axes_global"][axis][0], pe, pi, 2) for axis in range(3)), "joint attachment pair")
            participation[cid, jid] = edge
    if solved:
        for record in records:
            for case in cases:
                lookup = lambda kind, component: next((row for row in rows.values() if row["entity_ref"] == record["component_id"] and row["kind"] == kind and row["metadata"]["component"] == component and row["basis_ref"]["ref_id"] == case), None)
                pe_row, s_row = lookup("connector_pressure_force_v1", "pe_x"), lookup("connector_effective_axial_force_v1", "s_x")
                rods = _tie_rods(record) if record["hardware"] == "explicit_axial_rods_v1" else []
                if record["pressure_model"] == "unpressurized":
                    # T4-U5b: keyed on hardware; with rods s_x is the face axial force (Pe = 0), never pe_x.
                    _require(pe_row is None and (s_row is not None) == bool(rods), "unpressurized joint pressure rows")
                    if not rods:
                        continue
                    pe = 0.0
                else:
                    edge = participation.get((case, record["component_id"]))
                    pe = edge["joint_pressure_force_n"] if edge else 0.0
                    _require(pe_row is not None, "joint pressure rows missing")
                    _require(pe_row["value"] == pe, "joint pressure row binding")
                gx = lookup("connector_generalized_force_v1", "gt_x")
                _require(s_row is not None and gx is not None, "joint pressure rows missing")
                terms = []
                for rid, share in rods:
                    tail = _tie_rod_tail(record["component_id"], rid)
                    t_row = next((row for row in rows.values() if row["kind"] == TIE_ROD_KIND and row["entity_ref"] == record["component_id"]
                                  and row["basis_ref"]["ref_id"] == case and _is_tie_rod_row_id(row.get("id"), case, tail)), None)
                    _require(t_row is not None and _number(t_row["value"]), "connector tie-rod rows missing")
                    terms.append((t_row["value"], share))
                _require(_effective_axial_binding(s_row["value"], gx["value"], pe, terms), "joint effective row binding")


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
        if "effective_area_bits" in group:
            _shape(group, {"node_ref", "component", "pressure_bits", "effective_area_bits", "direction_component_magnitude", "coefficient_sum", "assembled_force_n", "terms"}, "joint RHS group")
            _require(group["node_ref"] in rhs["node_order"] and group["component"] in ["Fx", "Fy", "Fz"] and 0 < group["direction_component_magnitude"] <= 1 and isinstance(group["terms"], list) and bool(group["terms"]), "joint RHS group values")
            _require(all(isinstance(group[key], str) and len(group[key]) == 16 and all(character in "0123456789abcdef" for character in group[key]) and math.isfinite(struct.unpack(">d", bytes.fromhex(group[key]))[0]) for key in ("pressure_bits", "effective_area_bits")), "joint RHS source bits")
            key = (group["node_ref"], group["component"], group["pressure_bits"], group["effective_area_bits"], "joint", group["direction_component_magnitude"])
            _require(key not in groups, "duplicate joint RHS group")
            groups.add(key)
            identities: set[tuple[str, str]] = set()
            for term in group["terms"]:
                _shape(term, {"coefficient", "region_id", "connector_id", "kind"}, "joint RHS term")
                _require(term["kind"] == "joint_effective_area" and _text(term["region_id"]) and _text(term["connector_id"]) and term["coefficient"] in (-1, 1), "joint RHS term values")
                identity = (term["region_id"], term["connector_id"])
                _require(identity not in identities, "duplicate joint RHS source")
                identities.add(identity)
            coefficient = sum(term["coefficient"] for term in group["terms"])
            _require(len(group["terms"]) == 1, "joint RHS source cardinality")
            pressure, area = (struct.unpack(">d", bytes.fromhex(group[key]))[0] for key in ("pressure_bits", "effective_area_bits"))
            expected = pressure * area * group["direction_component_magnitude"] * coefficient
            _require(group["coefficient_sum"] == coefficient and _joint_projection(group["assembled_force_n"], expected, expected, 0.0, 1), "joint RHS formed value")
            continue
        _shape(group, {"node_ref", "component", "pressure_bits", "source_inner_radius_hi_bits", "source_inner_radius_lo_bits", "direction_component_magnitude", "coefficient_sum", "assembled_force_n", "terms"}, "pressure RHS group")
        _require(group["node_ref"] in rhs["node_order"] and group["component"] in ["Fx", "Fy", "Fz"] and all(isinstance(group[k], str) and len(group[k]) == 16 and all(c in "0123456789abcdef" for c in group[k]) for k in ("pressure_bits", "source_inner_radius_hi_bits", "source_inner_radius_lo_bits")), "pressure RHS group source")
        _require(all(_number(group[k]) for k in ("direction_component_magnitude", "coefficient_sum", "assembled_force_n")) and 0 < group["direction_component_magnitude"] <= 1 and isinstance(group["terms"], list) and bool(group["terms"]), "pressure RHS group values")
        key = (group["node_ref"], group["component"], group["pressure_bits"], group["source_inner_radius_hi_bits"], group["source_inner_radius_lo_bits"], group["direction_component_magnitude"])
        _require(key not in groups, "duplicate pressure RHS group")
        groups.add(key)
        _require(all(math.isfinite(struct.unpack(">d", bytes.fromhex(group[field]))[0]) for field in ("pressure_bits", "source_inner_radius_hi_bits", "source_inner_radius_lo_bits")), "nonfinite pressure RHS source bits")
        for term in group["terms"]:
            _shape(term, {"coefficient", "region_id", "pipe_id", "kind"}, "pressure RHS term")
            _require(_number(term["coefficient"]) and _text(term["region_id"]) and _text(term["pipe_id"]) and term["kind"] in {"poisson_eigen", "terminal_cap", "bend_cap_removed", "kink_remainder", "joint_face_cap"}, "pressure RHS term values")


# --------------------------------------------------------------------------
# Transported statements (no raw rows)
# --------------------------------------------------------------------------

def _schema_shape(value: Any, schema: Mapping[str, Any], root: Mapping[str, Any]) -> bool:
    """The closed JSON-schema vocabulary the committed schemas use; nothing else is interpreted."""
    if "$ref" in schema:
        return _schema_shape(value, root["$defs"][schema["$ref"].split("#/$defs/", 1)[1]], root)
    if "anyOf" in schema and not any(_schema_shape(value, branch, root) for branch in schema["anyOf"]):
        return False
    if "oneOf" in schema and sum(_schema_shape(value, branch, root) for branch in schema["oneOf"]) != 1:
        return False
    if "allOf" in schema and not all(_schema_shape(value, branch, root) for branch in schema["allOf"]):
        return False
    if "const" in schema and (value != schema["const"] or (type(value) is bool) != (type(schema["const"]) is bool)):
        return False
    if "enum" in schema and not any(value == allowed and (type(value) is bool) == (type(allowed) is bool) for allowed in schema["enum"]):
        return False
    kind = schema.get("type", "object" if "properties" in schema else "array" if "items" in schema else None)
    if isinstance(kind, list):
        return any(_schema_shape(value, {**schema, "type": item}, root) for item in kind)
    if kind == "null":
        return value is None
    if kind == "boolean":
        return type(value) is bool
    if kind == "object":
        if type(value) is not dict:
            return False
        properties = schema.get("properties", {})
        if not set(schema.get("required", [])) <= set(value):
            return False
        if schema.get("additionalProperties") is False and not set(value) <= set(properties):
            return False
        return all(_schema_shape(item, properties[key], root) for key, item in value.items() if key in properties)
    if kind == "array":
        return type(value) is list and schema.get("minItems", 0) <= len(value) <= schema.get("maxItems", 16384) and all(_schema_shape(item, schema["items"], root) for item in value)
    if kind == "string":
        if type(value) is not str or len(value) < schema.get("minLength", 0):
            return False
        if "pattern" in schema:
            import re  # deferred: the raw path never needs it
            return re.fullmatch(schema["pattern"], value) is not None
        return True
    if kind in {"number", "integer"}:
        return (type(value) in {int, float} and math.isfinite(value) and (kind != "integer" or float(value).is_integer())
                and schema.get("minimum", -math.inf) <= value <= schema.get("maximum", math.inf) and value > schema.get("exclusiveMinimum", -math.inf))
    return True


def _transport(source: Mapping[str, Any]) -> None:
    results = json.loads((PROJECT / "schemas/results.v0.3.schema.yaml").read_text())
    pressure = json.loads((PROJECT / "schemas/results_pressure_2.v0.3.schema.json").read_text())
    # The pressure-2 definitions name the shared ones as `results.v0.3.schema.yaml#/$defs/<name>`;
    # every definition name is unique across the two files.
    root = {"$defs": {**results["$defs"], **pressure["$defs"]}}
    evidence = source.get("contract_evidence")
    _require(isinstance(evidence, Mapping), "transport evidence shape")
    records = evidence.get("load_reference_states")
    body = {key: value for key, value in evidence.items() if key != "load_reference_states"}
    _require(_schema_shape(body, root["$defs"]["PressureContractEvidence"], root), "transport evidence shape")
    if records is not None:
        frozen = json.loads((PROJECT / "schemas/load_reference_state.schema.json").read_text())
        _require(isinstance(records, list) and all(_schema_shape(r, frozen["$defs"]["LoadReferenceStateRecord"], frozen) for r in records), "transport evidence shape")
    _finite_tree(evidence)
    cases = _indexed(evidence["exact_cases"], "load_case_id", "transport cases")
    has_connector = bool(evidence["connector"])
    resolved = {"material_selection_kind", "resolved_eigenstrain"}
    for case in cases.values():
        materials = _indexed(case["pipe_materials"], "pipe_id", "transport materials")
        sections = _indexed(case["pipe_sections"], "pipe_id", "transport sections")
        _require((bool(materials) or has_connector) and set(materials) == set(sections), "transport member coverage")
        # T4-U6 (§2.5): one coverage record and one maximum per pipe per basis (T4-U4b §2.1: realized arcs carry
        # theirs too).
        for basis, (missing, _) in zip(sb.BASES, _basis(sb.coverage, case["stress_maximum_coverage"])):
            _require(missing <= set(sections), "transport maximum coverage")
            members = [ex["pipe_id"] for ex in case["pipe_stress_extrema"] if _basis(sb.extremum_basis, ex) == basis]
            _require(len(members) == len(set(members)) and set(members) == set(sections) - missing, "transport maximum members")
    regions: set[tuple[str, str]] = set()
    for region in evidence["pressure"]:
        cid, rid = region["load_case_id"], region["region_id"]
        _require(cid in cases and (cid, rid) not in regions, "transport pressure region")
        regions.add((cid, rid))
        arcs = arc_members(region)
        sections = _indexed(cases[cid]["pipe_sections"], "pipe_id", "transport sections")
        materials = _indexed(cases[cid]["pipe_materials"], "pipe_id", "transport materials")
        gm = _indexed(region["geometry"], "pipe_id", "transport region geometry")
        mm = _indexed(region["materials"], "pipe_id", "transport region materials")
        _require(_strings(region["member_pipe_ids"], True) and set(gm) == set(mm) == set(region["member_pipe_ids"]) and set(gm) <= set(sections), "transport region members")
        for pid in gm:
            # §2.5: an arc's record adds only its end tangents (T4-U4b: and its bend radius) to its case section, which
            # carries the marker.
            _require({k: v for k, v in gm[pid].items() if k not in {"traversal_forward", "arc_end_tangents_global", "bend_radius_m"}} == sections[pid]
                     and {k: v for k, v in mm[pid].items() if k != "temperature_basis"} == {k: v for k, v in materials[pid].items() if k not in resolved}, "transport region member facts")
        if arcs:
            _arc_region(region, arcs, region["p_pa"])


# --------------------------------------------------------------------------
# The load/reference-state group: its pre-pass, then the physics checks on a
# projection. The pre-pass names its own codes (SOURCE_LOAD_REFERENCE_*).
# --------------------------------------------------------------------------

class LoadStateError(ValueError):
    """A pre-pass refusal; ``str(error)`` names its code."""


RECORD_CONTRACT = "openpipestress.load_reference_state/1.0.0"
RECORD_PROFILE = "resolved_straight_load_state_v1"
MATERIAL_BASIS = "resolved_per_member_load_reference_state_v1"
NOT_JOINED = "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED"
REGION_TEMPERATURE_BASIS = "resolved_member_state"
G_BASIS = "E/[2(1+nu)] from the selected pair"
COMPOSITION = "lambda_fit*lambda_thermal-1"
BOUNDARY = "every restrained DOF prescribed; reduced K_ff u_f = f_f - K_fc g_c; complete u includes g; reactions from unreduced K u - f"
EIGENLOAD = "axial E_member*A_s*total_eigenstrain assembled once and removed once in recovery"
LS_EVIDENCE_KEYS = ["pressure", "connector", "exact_cases", "load_reference_states"]
LS_CASE_KEYS = ["load_case_id", "profile_mode", "material_basis", "pipe_materials", "pipe_sections", "pipe_stress_extrema", "stress_maximum_coverage", "pressure_rhs_assembly"]
LS_MATERIAL_KEYS = ["pipe_id", "material_id", "E_pa", "nu", "G_pa", "constitutive_basis", "material_selection_kind", "thermal_consumed", "alpha_per_kelvin", "resolved_eigenstrain", "provenance"]
LS_RECORD_KEYS = ["load_case_id", "contract", "profile", "reference_configuration_id", "provenance", "reference_geometry", "history", "solve", "source_recovery", "members", "support_components", "contributions", "excluded_sources"]
LS_MEMBER_KEYS = [
    "pipe_id", "material_id", "material_selection_kind", "consumed_material_points", "interpolation_fraction",
    "applicability_reference", "analysis_basis_override", "selected_E_pa", "selected_nu", "derived_G_pa", "G_basis",
    "retained_G_ignored", "operating_temperature_k", "material_selection_temperature_k", "reference_basis",
    "installation_temperature_k", "thermal_definition", "expansion_law_id", "coefficient_datum_k",
    "consumed_law_point_indices", "consumed_law_segments", "consulted_law_point_indices", "consulted_law_segments",
    "installation_datum_stretch", "operating_datum_stretch", "thermal_strain", "thermal_stretch", "fit_strain",
    "fit_stretch", "total_eigenstrain", "eigenstrain_composition", "fit_kind", "fit_input", "reference_length_m",
]
LS_POINT_KEYS = ["point_id", "temperature_k", "E_pa", "nu", "retained_G_ignored"]
LS_SEGMENT_KEYS = ["use", "lower_index", "upper_index", "start_k", "end_k"]
LS_SUPPORT_KEYS = ["support_id", "node_id", "dof", "global_dof", "law_kind", "prescribed_value", "unit", "meaning", "physical_state_source"]
LS_STORED_KEYS = ["source_id", "owner_kind", "classification", "factor", "category", "dimension", "authored_normalized_magnitude", "applied_magnitude"]
LS_MEMBER_STATE_KEYS = ["source_id", "owner_kind", "classification", "consumed_input_refs", "value"]
LS_SUPPORT_STATE_KEYS = ["source_id", "owner_kind", "classification", "value"]
LS_PRESSURE_REGION_KEYS = ["source_id", "owner_kind", "classification", "factor"]
LS_EXCLUDED_KEYS = ["source_id", "owner_kind", "classification", "category", "reason"]
LS_REGION_KEYS = ["profile_version", "profile_mode", "load_case_id", "region_id", "member_pipe_ids", "pressure_basis", "p_pa", "external_pressure_increment_pa", "approximation", "geometry_representation_guard", "geometry", "materials", "applied_loads", "terminals", "provenance", "result_ids"]
LS_REGION_MATERIAL_KEYS = ["pipe_id", "material_id", "E_pa", "nu", "G_pa", "constitutive_basis", "thermal_consumed", "alpha_per_kelvin", "provenance", "temperature_basis"]
DOFS = ["UX", "UY", "UZ", "RX", "RY", "RZ"]
LAW_DEFINITIONS = ["engineering_secant", "engineering_dilation", "differential_per_datum_length", "logarithmic_per_current_length"]
DIRECT_DEFINITIONS = ["unchanged_reference", "explicit_interval_strain", "constant_alpha_interval"]
U64_MAX = (1 << 64) - 1


def _load_state_evidence(source: Mapping[str, Any]) -> None:
    try:
        _ls_prepass(source)
        projected = _ls_project(source)
        try:
            _physics(projected)
        except ValueError as error:
            raise LoadStateError(f"{_ls_code('PHYSICS_EVIDENCE')}: {error}") from error
        except (TypeError, KeyError, AttributeError, OverflowError, IndexError) as error:
            raise LoadStateError(f"{_ls_code('PHYSICS_EVIDENCE')}: SOURCE_PHYSICS_EVIDENCE_INVALID: malformed evidence") from error
    except (TypeError, KeyError, AttributeError, OverflowError, IndexError) as error:
        raise LoadStateError("SOURCE_LOAD_REFERENCE_MALFORMED") from error


def _ls_code(name: str) -> str:
    return f"SOURCE_LOAD_REFERENCE_{name}"


def _ls_fail(name: str) -> LoadStateError:
    return LoadStateError(_ls_code(name))


def _ls_require(ok: bool, name: str) -> None:
    if not ok:
        raise _ls_fail(name)


def _ls_get(value: Any, key: str) -> Any:
    """Missing key or non-object reads as null."""
    return value.get(key) if isinstance(value, Mapping) else None


def _ls_keys(value: Any, required: list[str]) -> bool:
    return isinstance(value, Mapping) and len(value) == len(required) and all(key in value for key in required)


def _ls_is_number(value: Any) -> bool:
    return type(value) in (int, float)


def _ls_as_f64(value: Any) -> float | None:
    """The binary64 value of a JSON number (an integer rounds to nearest), else ``None``."""
    if type(value) is float:
        return value
    if type(value) is int:
        try:
            return float(value)
        except OverflowError:
            return None
    return None


def _ls_text(value: Any) -> str:
    if not (isinstance(value, str) and value):
        raise _ls_fail("STRING_INVALID")
    return value


def _ls_number(value: Any) -> float:
    number = _ls_as_f64(value)
    if number is None or not math.isfinite(number):
        raise _ls_fail("NUMBER_INVALID")
    return number


def _ls_opt_number(value: Any) -> float | None:
    return None if value is None else _ls_number(value)


def _ls_opt_text(value: Any) -> str | None:
    return None if value is None else _ls_text(value)


def _ls_boolean(value: Any) -> bool:
    if type(value) is not bool:
        raise _ls_fail("BOOLEAN_INVALID")
    return value


def _ls_index(value: Any) -> int:
    if not (type(value) is int and 0 <= value <= U64_MAX):
        raise _ls_fail("INTEGER_INVALID")
    return value


def _ls_array(value: Any) -> list[Any]:
    if not isinstance(value, list):
        raise _ls_fail("ARRAY_INVALID")
    return value


def _ls_num_eq(a: Any, b: Any) -> bool:
    x, y = _ls_as_f64(a), _ls_as_f64(b)
    return x is not None and y is not None and math.isfinite(x) and math.isfinite(y) and x == y


def _ls_same(a: Any, b: Any) -> bool:
    """Structural equality comparing numbers by binary64 value."""
    if type(a) is bool or type(b) is bool:
        return type(a) is bool and type(b) is bool and a == b
    if _ls_is_number(a) and _ls_is_number(b):
        return _ls_num_eq(a, b)
    if isinstance(a, list) and isinstance(b, list):
        return len(a) == len(b) and all(_ls_same(x, y) for x, y in zip(a, b))
    if isinstance(a, Mapping) and isinstance(b, Mapping):
        return len(a) == len(b) and all(key in b and _ls_same(value, b[key]) for key, value in a.items())
    if _ls_is_number(a) or _ls_is_number(b) or isinstance(a, (list, Mapping)) or isinstance(b, (list, Mapping)):
        return False
    return type(a) is type(b) and a == b


def _ls_eq(value: Any, expected: Any) -> bool:
    """Equality with a string, bool or null literal."""
    if type(expected) is bool or type(value) is bool:
        return type(value) is type(expected) and value == expected
    if expected is None:
        return value is None
    return isinstance(value, str) and value == expected


def _ls_without(value: Any, removed: list[str]) -> Any:
    if not isinstance(value, Mapping):
        return deepcopy(value)
    return {key: deepcopy(item) for key, item in value.items() if key not in removed}


def _ls_sha64(value: Any) -> bool:
    return isinstance(value, str) and len(value) == 64 and all(c in "0123456789abcdef" for c in value)


def _ls_finite_tree(value: Any) -> None:
    if type(value) is bool or value is None or isinstance(value, str):
        return
    if type(value) in (int, float):
        _ls_number(value)
    elif isinstance(value, Mapping):
        for item in value.values():
            _ls_finite_tree(item)
    elif isinstance(value, list):
        for item in value:
            _ls_finite_tree(item)
    else:
        raise _ls_fail("NUMBER_INVALID")


def _ls_bits(value: float) -> int:
    return struct.unpack(">Q", struct.pack(">d", value))[0]


def _ls_positive(value: float | None) -> bool:
    return value is None or value > 0


def _ls_prepass(source: Mapping[str, Any]) -> None:
    """Steps S1-S13 of the not-joined load/reference-state pre-pass, with
    connector records and realized-arc region keys admitted (pressure-2)."""
    # S1 foreign method namespaces.
    _ls_require(not isinstance(source, Mapping) or ("source_block_recovery" not in source and "carrier_evidence" not in source), "FOREIGN_METHOD_EVIDENCE")
    # S2 finite numbers everywhere.
    _ls_finite_tree(source)
    evidence = _ls_get(source, "contract_evidence")
    # S3-S5 namespace; connector records are left to the physics pass.
    _ls_require(_ls_keys(evidence, LS_EVIDENCE_KEYS), "EVIDENCE_SHAPE")
    _ls_array(evidence["connector"])
    pressure = _ls_array(evidence["pressure"])
    exact = _ls_array(evidence["exact_cases"])
    records = _ls_array(evidence["load_reference_states"])
    # S6 numerical case identities.
    quality_ids: list[str] = []
    for case in _ls_array(_ls_get(_ls_get(source, "numerical_quality"), "cases")):
        basis = _ls_get(case, "basis_ref")
        _ls_require(_ls_keys(basis, ["ref_type", "ref_id"]) and _ls_eq(basis["ref_type"], "load_case"), "NUMERICAL_CASE_BASIS")
        case_id = _ls_text(basis["ref_id"])
        _ls_require(case_id not in quality_ids, "NUMERICAL_CASE_DUPLICATE")
        quality_ids.append(case_id)
    # S7 exact cases, their resolved materials and section identities.
    cases: list[tuple[str, Mapping[str, Any]]] = []
    for case in exact:
        # T4-U4b (§2.6): a case with a directional arc adds intensified_measures (checked by the physics pass).
        _ls_require(_ls_keys(case, LS_CASE_KEYS + (["intensified_measures"] if isinstance(case, Mapping) and "intensified_measures" in case else [])), "CASE_SHAPE")
        case_id = _ls_text(case["load_case_id"])
        _ls_require(all(known != case_id for known, _ in cases), "CASE_DUPLICATE")
        _ls_require(_ls_eq(case["material_basis"], MATERIAL_BASIS), "MATERIAL_BASIS")
        pipes: list[str] = []
        for material in _ls_array(case["pipe_materials"]):
            _ls_require(_ls_keys(material, LS_MATERIAL_KEYS), "MATERIAL_SHAPE")
            pipe = _ls_text(material["pipe_id"])
            _ls_require(pipe not in pipes, "MATERIAL_DUPLICATE")
            pipes.append(pipe)
        sections: list[str] = []
        for section in _ls_array(case["pipe_sections"]):
            pipe = _ls_text(_ls_get(section, "pipe_id"))
            _ls_require(pipe not in sections, "SECTION_DUPLICATE")
            sections.append(pipe)
        cases.append((case_id, case))
    # S8 record identities.
    record_ids: list[str] = []
    for record in records:
        _ls_require(_ls_keys(record, LS_RECORD_KEYS), "RECORD_SHAPE")
        case_id = _ls_text(record["load_case_id"])
        _ls_require(case_id not in record_ids, "RECORD_DUPLICATE")
        record_ids.append(case_id)
    # S9 case trijection. An unsolved envelope retains the empty namespace.
    solved = _ls_eq(_ls_get(_ls_get(source, "status"), "mechanics"), "MECHANICS_SOLVED")
    if solved:
        quality = set(quality_ids)
        _ls_require(bool(record_ids) and set(record_ids) == quality and {case_id for case_id, _ in cases} == quality, "CASE_COVERAGE")
    else:
        _ls_require(not records and not exact and not pressure, "UNSOLVED_EVIDENCE")
    # S10 each record in document order.
    for record in records:
        _ls_record(record, cases, pressure)
    # S11 one model geometry and one requested mode per envelope.
    if records:
        first = records[0]
        for record in records:
            _ls_require(_ls_same(_ls_get(record["reference_geometry"], "projection_sha256"), _ls_get(first["reference_geometry"], "projection_sha256")), "REFERENCE_GEOMETRY_CONSISTENCY")
            _ls_require(_ls_same(_ls_get(record["solve"], "requested_mode"), _ls_get(first["solve"], "requested_mode")), "SOLVE_CONSISTENCY")
    # S12 pressure-region materials are the resolved member pair of their case;
    # a region with a realized arc adds the arc keys (checked by the physics pass).
    for region in pressure:
        arcs = arc_members(region)
        _ls_require(_ls_keys(region, LS_REGION_KEYS + (sorted(ARC_REGION_KEYS) if arcs else [])), "REGION_SHAPE")
        case_id = _ls_text(region["load_case_id"])
        _ls_text(region["region_id"])
        case = next((value for known, value in cases if known == case_id), None)
        if case is None:
            raise _ls_fail("REGION_CASE_UNRESOLVED")
        for material in _ls_array(region["materials"]):
            _ls_require(_ls_keys(material, LS_REGION_MATERIAL_KEYS), "REGION_MATERIAL_SHAPE")
            _ls_require(_ls_eq(material["temperature_basis"], REGION_TEMPERATURE_BASIS), "REGION_TEMPERATURE_BASIS")
            pipe = _ls_text(material["pipe_id"])
            member = next((m for m in _ls_array(case["pipe_materials"]) if _ls_eq(_ls_get(m, "pipe_id"), pipe)), None)
            if member is None:
                raise _ls_fail("REGION_MATERIAL_BINDING")
            _ls_require(_ls_same(_ls_without(material, ["temperature_basis"]), _ls_without(member, ["material_selection_kind", "resolved_eigenstrain"])), "REGION_MATERIAL_BINDING")
    # S13 exactly one not-joined info diagnostic per resolved case.
    if solved:
        diagnostics = _ls_array(_ls_get(source, "diagnostics"))
        for case_id in record_ids:
            expected = f"diagnostic:load-state:{case_id.replace(':', '-')}:source-recovery-not-joined"
            hits = [d for d in diagnostics if _ls_eq(_ls_get(d, "code"), NOT_JOINED) and _ls_eq(_ls_get(d, "id"), expected)]
            _ls_require(len(hits) == 1 and _ls_eq(_ls_get(hits[0], "severity"), "info") and _ls_same(_ls_get(hits[0], "affected_refs"), [case_id]), "NOT_JOINED_DIAGNOSTIC")
        _ls_require(sum(1 for d in diagnostics if _ls_eq(_ls_get(d, "code"), NOT_JOINED)) == len(record_ids), "NOT_JOINED_DIAGNOSTIC")


def _ls_project(source: Mapping[str, Any]) -> dict[str, Any]:
    """Removes or neutralizes only fields the pre-pass has bound."""
    projected = deepcopy(dict(source)) if isinstance(source, Mapping) else deepcopy(source)
    evidence = projected.get("contract_evidence") if isinstance(projected, dict) else None
    if isinstance(evidence, dict):
        evidence.pop("load_reference_states", None)
        for case in evidence.get("exact_cases") if isinstance(evidence.get("exact_cases"), list) else []:
            if isinstance(case, dict):
                case["material_basis"] = "base_material_common_E_nu"
                for material in case.get("pipe_materials") if isinstance(case.get("pipe_materials"), list) else []:
                    if isinstance(material, dict):
                        material.pop("material_selection_kind", None)
                        material.pop("resolved_eigenstrain", None)
        for region in evidence.get("pressure") if isinstance(evidence.get("pressure"), list) else []:
            if isinstance(region, dict):
                for material in region.get("materials") if isinstance(region.get("materials"), list) else []:
                    if isinstance(material, dict):
                        material["temperature_basis"] = {"selection": "base_material"}
    return projected


def _ls_record(record: Mapping[str, Any], cases: list[tuple[str, Mapping[str, Any]]], pressure: list[Any]) -> None:
    case_id = _ls_text(record["load_case_id"])
    # R1-R7 record-level closed statements.
    _ls_require(_ls_eq(record["contract"], RECORD_CONTRACT), "RECORD_CONTRACT")
    _ls_require(_ls_eq(record["profile"], RECORD_PROFILE), "RECORD_PROFILE")
    configuration = _ls_text(record["reference_configuration_id"])
    _ls_text(record["provenance"])
    geometry = record["reference_geometry"]
    _ls_require(_ls_keys(geometry, ["kind", "projection_sha256"]), "REFERENCE_GEOMETRY_SHAPE")
    _ls_require(_ls_eq(geometry["kind"], "authored_model_geometry") and _ls_sha64(geometry["projection_sha256"]), "REFERENCE_GEOMETRY")
    history = record["history"]
    _ls_require(_ls_keys(history, ["kind"]), "HISTORY_SHAPE")
    _ls_require(_ls_eq(history["kind"], "independent_equilibrium"), "HISTORY")
    solve = record["solve"]
    _ls_require(_ls_keys(solve, ["requested_mode", "recovery_method", "boundary", "eigenload"]), "SOLVE_SHAPE")
    pair = (solve["requested_mode"], solve["recovery_method"])
    _ls_require(
        all(isinstance(item, str) for item in pair)
        and pair in {("sparse_interactive", "ordinary_sparse_structural_v1"), ("dense_scrutiny", "ordinary_dense_structural_v1")}
        and _ls_eq(solve["boundary"], BOUNDARY) and _ls_eq(solve["eigenload"], EIGENLOAD),
        "SOLVE",
    )
    recovery = record["source_recovery"]
    _ls_require(_ls_keys(recovery, ["status", "code"]), "SOURCE_RECOVERY_SHAPE")
    _ls_require(_ls_eq(recovery["status"], "not_joined") and _ls_eq(recovery["code"], NOT_JOINED), "SOURCE_RECOVERY")
    # R8 members.
    members = _ls_array(record["members"])
    member_ids: list[str] = []
    for member in members:
        _ls_member(member)
        pipe = _ls_text(member["pipe_id"])
        _ls_require(pipe not in member_ids, "MEMBER_DUPLICATE")
        member_ids.append(pipe)
    # R9 prescribed support components.
    components = _ls_array(record["support_components"])
    component_ids: list[tuple[str, str]] = []
    support_nodes: dict[str, str] = {}
    node_bases: dict[str, int] = {}
    for component in components:
        _ls_support_component(component)
        support = _ls_text(component["support_id"])
        node = _ls_text(component["node_id"])
        dof = _ls_text(component["dof"])
        _ls_require((support, dof) not in component_ids, "SUPPORT_COMPONENT_DUPLICATE")
        component_ids.append((support, dof))
        base = _ls_index(component["global_dof"]) // 6
        _ls_require(support_nodes.setdefault(support, node) == node and node_bases.setdefault(node, base) == base, "SUPPORT_COMPONENT_NODE")
    # R10 contributions.
    contributions = _ls_array(record["contributions"])
    contribution_ids: list[str] = []
    for contribution in contributions:
        _ls_contribution(contribution)
        source_id = _ls_text(contribution["source_id"])
        _ls_require(source_id not in contribution_ids, "CONTRIBUTION_DUPLICATE")
        contribution_ids.append(source_id)
    # R11 excluded sources.
    excluded_ids: list[str] = []
    for excluded in _ls_array(record["excluded_sources"]):
        _ls_require(_ls_keys(excluded, LS_EXCLUDED_KEYS), "EXCLUDED_SHAPE")
        _ls_require(_ls_eq(excluded["owner_kind"], "stored_primitive") and _ls_eq(excluded["classification"], "excluded"), "EXCLUDED_CLASSIFICATION")
        source_id = _ls_text(excluded["source_id"])
        _ls_text(excluded["category"])
        _ls_text(excluded["reason"])
        _ls_require(source_id not in contribution_ids and source_id not in excluded_ids, "EXCLUDED_OVERLAP")
        excluded_ids.append(source_id)
    # R12-R14 binding to the exact case of the same load case.
    case = next((value for known, value in cases if known == case_id), None)
    if case is None:
        raise _ls_fail("RECORD_CASE_UNRESOLVED")
    sections = {s["pipe_id"] if isinstance(_ls_get(s, "pipe_id"), str) else "" for s in _ls_array(case["pipe_sections"])}
    _ls_require(bool(member_ids) and set(member_ids) == sections, "MEMBER_COVERAGE")
    materials = _ls_array(case["pipe_materials"])
    _ls_require({m["pipe_id"] if isinstance(_ls_get(m, "pipe_id"), str) else "" for m in materials} == set(member_ids), "MEMBER_MATERIAL_COVERAGE")
    for member in members:
        material = next((m for m in materials if _ls_same(_ls_get(m, "pipe_id"), member["pipe_id"])), None)
        if material is None:
            raise _ls_fail("MEMBER_MATERIAL_COVERAGE")
        _ls_require(
            _ls_num_eq(material["E_pa"], member["selected_E_pa"])
            and _ls_num_eq(material["nu"], member["selected_nu"])
            and _ls_num_eq(material["G_pa"], member["derived_G_pa"])
            and _ls_same(material["material_id"], member["material_id"])
            and _ls_same(material["material_selection_kind"], member["material_selection_kind"])
            and _ls_num_eq(material["resolved_eigenstrain"], member["total_eigenstrain"])
            and _ls_eq(material["thermal_consumed"], False)
            and material["alpha_per_kelvin"] is None,
            "MEMBER_MATERIAL_BINDING",
        )
        _ls_text(material["provenance"])

    # R15 contribution ledgers are bijections with their owners.
    def of_kind(kind: str) -> set[str]:
        return {c["source_id"] for c in contributions if _ls_eq(_ls_get(c, "owner_kind"), kind) and isinstance(_ls_get(c, "source_id"), str)}

    def find(source_id: str) -> Any:
        return next((c for c in contributions if _ls_eq(_ls_get(c, "source_id"), source_id)), None)

    _ls_require(of_kind("resolved_member_state") == {f"member_state:{pipe}" for pipe in member_ids}, "MEMBER_CONTRIBUTION_COVERAGE")
    for member in members:
        pipe = _ls_text(member["pipe_id"])
        contribution = find(f"member_state:{pipe}")
        if contribution is None:
            raise _ls_fail("MEMBER_CONTRIBUTION_COVERAGE")
        _ls_require(
            _ls_num_eq(_ls_get(contribution, "value"), member["total_eigenstrain"])
            and _ls_same(_ls_get(contribution, "consumed_input_refs"), [f"{configuration}:{pipe}", f"{case_id}:element_state:{pipe}"]),
            "MEMBER_CONTRIBUTION_BINDING",
        )
    _ls_require(of_kind("support_state") == {f"support_state:{support}:{dof}" for support, dof in component_ids}, "SUPPORT_CONTRIBUTION_COVERAGE")
    for component in components:
        source_id = f"support_state:{_ls_text(component['support_id'])}:{_ls_text(component['dof'])}"
        contribution = find(source_id)
        if contribution is None:
            raise _ls_fail("SUPPORT_CONTRIBUTION_COVERAGE")
        _ls_require(_ls_num_eq(_ls_get(contribution, "value"), component["prescribed_value"]), "SUPPORT_CONTRIBUTION_BINDING")
    expected_regions: set[str] = set()
    for region in pressure:
        if _ls_eq(_ls_get(region, "load_case_id"), case_id):
            expected_regions.add(f"pressure_region:{_ls_text(_ls_get(region, 'region_id'))}")
    _ls_require(of_kind("pressure_region") == expected_regions, "PRESSURE_CONTRIBUTION_COVERAGE")


def _ls_member(member: Any) -> None:
    # M1-M3 shape, identity and fixed statements.
    _ls_require(_ls_keys(member, LS_MEMBER_KEYS), "MEMBER_SHAPE")
    _ls_text(member["pipe_id"])
    _ls_text(member["material_id"])
    _ls_require(_ls_eq(member["G_basis"], G_BASIS) and _ls_eq(member["eigenstrain_composition"], COMPOSITION), "MEMBER_BASIS")
    # M4 selected pair and derived G.
    e = _ls_number(member["selected_E_pa"])
    nu = _ls_number(member["selected_nu"])
    g = _ls_number(member["derived_G_pa"])
    _ls_require(e > 0 and -1 < nu < 0.5 and g > 0, "MEMBER_MATERIAL_RANGE")
    direct = e / (2.0 * (1.0 + nu))
    _ls_require(math.isfinite(direct) and abs(_ls_bits(g) - _ls_bits(direct)) <= 2, "MEMBER_G_BINDING")
    retained = _ls_boolean(member["retained_G_ignored"])
    # M6 consumed material points.
    consumed: list[tuple[float | None, float, float, bool]] = []
    for point in _ls_array(member["consumed_material_points"]):
        _ls_require(_ls_keys(point, LS_POINT_KEYS), "MATERIAL_POINT_SHAPE")
        _ls_text(point["point_id"])
        temperature = _ls_opt_number(point["temperature_k"])
        _ls_require(_ls_positive(temperature), "TEMPERATURE_RANGE")
        pe = _ls_number(point["E_pa"])
        pnu = _ls_number(point["nu"])
        _ls_require(pe > 0 and -1 < pnu < 0.5, "MATERIAL_POINT_RANGE")
        consumed.append((temperature, pe, pnu, _ls_boolean(point["retained_G_ignored"])))
    # M7-M9 optional selection data and temperatures.
    fraction = _ls_opt_number(member["interpolation_fraction"])
    applicability = _ls_opt_text(member["applicability_reference"])
    override = member["analysis_basis_override"]
    if override is not None:
        _ls_require(_ls_keys(override, ["reason", "provenance"]), "OVERRIDE_SHAPE")
        _ls_text(override["reason"])
        _ls_text(override["provenance"])
    operating = _ls_opt_number(member["operating_temperature_k"])
    selection = _ls_opt_number(member["material_selection_temperature_k"])
    installation = _ls_opt_number(member["installation_temperature_k"])
    datum = _ls_opt_number(member["coefficient_datum_k"])
    _ls_require(all(_ls_positive(value) for value in (operating, selection, installation, datum)), "TEMPERATURE_RANGE")
    # M10 material selection.
    kind = member["material_selection_kind"]
    if _ls_eq(kind, "explicit_base_properties"):
        selected = not consumed and fraction is None and applicability is not None and selection is None
    elif _ls_eq(kind, "exact_point"):
        selected = len(consumed) == 1 and fraction is None and applicability is None and e == consumed[0][1] and nu == consumed[0][2] and selection == consumed[0][0] and retained == consumed[0][3]
    elif _ls_eq(kind, "temperature_interpolation"):
        if len(consumed) == 1:
            shape = fraction is None and e == consumed[0][1] and nu == consumed[0][2] and selection == consumed[0][0]
        elif len(consumed) == 2:
            shape = fraction is not None and 0 < fraction < 1 and all(point[0] is not None for point in consumed)
        else:
            shape = False
        selected = applicability is None and selection is not None and retained == any(point[3] for point in consumed) and shape
    else:
        raise _ls_fail("MATERIAL_SELECTION_KIND")
    _ls_require(selected, "MATERIAL_SELECTION")
    # M11 reference basis.
    basis = member["reference_basis"]
    _ls_require((_ls_eq(basis, "temperature_reference") and installation is not None) or (_ls_eq(basis, "direct_strain_reference") and installation is None), "REFERENCE_BASIS")
    # M12 strains and stretches.
    thermal_strain = _ls_number(member["thermal_strain"])
    thermal_stretch = _ls_number(member["thermal_stretch"])
    fit_strain = _ls_number(member["fit_strain"])
    fit_stretch = _ls_number(member["fit_stretch"])
    total = _ls_number(member["total_eigenstrain"])
    _ls_require(thermal_stretch > 0 and fit_stretch > 0 and thermal_stretch == 1.0 + thermal_strain and fit_stretch == 1.0 + fit_strain and 1.0 + total > 0, "STRETCH_BINDING")
    # M13 consumed and consulted law data: adjacent indices, a sample at one
    # temperature, an interval over start < end, and no repeated entry within
    # one list. The consumed and consulted lists may share entries.
    law_data_empty = True
    for key in ("consumed_law_point_indices", "consulted_law_point_indices"):
        previous: int | None = None
        for item in _ls_array(member[key]):
            index = _ls_index(item)
            _ls_require(previous is None or previous < index, "LAW_INDICES")
            previous = index
            law_data_empty = False
    for key in ("consumed_law_segments", "consulted_law_segments"):
        entries: list[tuple[bool, int, int, float, float]] = []
        for segment in _ls_array(member[key]):
            _ls_require(_ls_keys(segment, LS_SEGMENT_KEYS), "LAW_SEGMENT_SHAPE")
            if _ls_eq(segment["use"], "interpolation_sample"):
                sample = True
            elif _ls_eq(segment["use"], "integration_interval"):
                sample = False
            else:
                raise _ls_fail("LAW_SEGMENT")
            lower = _ls_index(segment["lower_index"])
            upper = _ls_index(segment["upper_index"])
            start = _ls_number(segment["start_k"])
            end = _ls_number(segment["end_k"])
            _ls_require(upper == lower + 1 and (start == end if sample else start < end), "LAW_SEGMENT")
            entry = (sample, lower, upper, start, end)
            _ls_require(entry not in entries, "LAW_SEGMENT_DUPLICATE")
            entries.append(entry)
            law_data_empty = False
    installation_stretch = _ls_opt_number(member["installation_datum_stretch"])
    operating_stretch = _ls_opt_number(member["operating_datum_stretch"])
    law_id = _ls_opt_text(member["expansion_law_id"])
    # M14 thermal definition.
    definition = member["thermal_definition"] if isinstance(member["thermal_definition"], str) else ""
    if definition in LAW_DEFINITIONS:
        _ls_require(
            law_id is not None and datum is not None
            and installation_stretch is not None and installation_stretch > 0
            and operating_stretch is not None and operating_stretch > 0
            and installation is not None and operating is not None,
            "THERMAL_LAW_BINDING",
        )
    elif definition in DIRECT_DEFINITIONS:
        _ls_require(
            law_id is None and datum is None and installation_stretch is None and operating_stretch is None
            and law_data_empty and (definition != "unchanged_reference" or thermal_strain == 0),
            "THERMAL_LAW_BINDING",
        )
    else:
        raise _ls_fail("THERMAL_DEFINITION")
    # M15-M16 reference length and fit.
    length = _ls_number(member["reference_length_m"])
    _ls_require(length > 0, "REFERENCE_LENGTH")
    fit = member["fit_input"]
    fit_kind = member["fit_kind"]
    if _ls_eq(fit_kind, "none"):
        fitted = fit is None and fit_strain == 0
    elif _ls_eq(fit_kind, "natural_length_change"):
        change = _ls_as_f64(_ls_get(fit, "length_change_m"))
        fitted = _ls_keys(fit, ["length_change_m"]) and change is not None and math.isfinite(change) and fit_strain == change / length and length + change > 0
    elif _ls_eq(fit_kind, "fit_strain"):
        strain = _ls_as_f64(_ls_get(fit, "strain"))
        fitted = _ls_keys(fit, ["strain"]) and strain is not None and math.isfinite(strain) and fit_strain == strain
    else:
        raise _ls_fail("FIT_KIND")
    _ls_require(fitted, "FIT_BINDING")


def _ls_support_component(component: Any) -> None:
    _ls_require(_ls_keys(component, LS_SUPPORT_KEYS), "SUPPORT_COMPONENT_SHAPE")
    _ls_text(component["support_id"])
    _ls_text(component["node_id"])
    dof = component["dof"]
    if not (isinstance(dof, str) and dof in DOFS):
        raise _ls_fail("SUPPORT_COMPONENT_DOF")
    position = DOFS.index(dof)
    global_dof = _ls_index(component["global_dof"])
    _ls_require(global_dof % 6 == position, "SUPPORT_COMPONENT_DOF")
    _ls_require(_ls_eq(component["law_kind"], "rigid_prescribed") and _ls_eq(component["meaning"], "absolute_reference_displacement") and _ls_eq(component["physical_state_source"], "support_state.boundary_motion"), "SUPPORT_COMPONENT_LAW")
    _ls_require(_ls_eq(component["unit"], "m" if position < 3 else "rad"), "SUPPORT_COMPONENT_UNIT")
    _ls_number(component["prescribed_value"])


def _ls_contribution(contribution: Any) -> None:
    kind = _ls_get(contribution, "owner_kind")
    shapes = {
        "stored_primitive": (LS_STORED_KEYS, "ordinary_applied"),
        "resolved_member_state": (LS_MEMBER_STATE_KEYS, "eigenstrain"),
        "support_state": (LS_SUPPORT_STATE_KEYS, "prescribed_boundary"),
        "pressure_region": (LS_PRESSURE_REGION_KEYS, "pressure_eigen_and_closure"),
    }
    if not (isinstance(kind, str) and kind in shapes):
        raise _ls_fail("CONTRIBUTION_KIND")
    shape, classification = shapes[kind]
    _ls_require(_ls_keys(contribution, shape), "CONTRIBUTION_SHAPE")
    _ls_require(_ls_eq(contribution["classification"], classification), "CONTRIBUTION_CLASSIFICATION")
    _ls_text(contribution["source_id"])
    if kind == "stored_primitive":
        factor = _ls_as_f64(contribution["factor"])
        if not (factor is not None and math.isfinite(factor) and factor != 0):
            raise _ls_fail("CONTRIBUTION_FACTOR")
        _ls_text(contribution["category"])
        _ls_text(contribution["dimension"])
        authored = _ls_number(contribution["authored_normalized_magnitude"])
        applied = _ls_number(contribution["applied_magnitude"])
        _ls_require(applied == authored * factor, "CONTRIBUTION_APPLIED_MAGNITUDE")
    elif kind == "resolved_member_state":
        for reference in _ls_array(contribution["consumed_input_refs"]):
            _ls_text(reference)
        _ls_number(contribution["value"])
    elif kind == "support_state":
        _ls_number(contribution["value"])
    else:
        _ls_require(contribution["factor"] is None, "CONTRIBUTION_FACTOR")
