"""Closed admission of the load-reference-1 raw evidence namespace.

Mirror of ``core/reporting/result_export/src/load_reference.rs``: the same
checks run in the same order and raise the same error strings, so the Rust and
Python readers accept and reject the same inputs. Change both together.
Arrays are walked in document order so the first failure is deterministic.

After the load-reference-specific pre-pass, the unchanged physics-1 validator
checks rows, extrema, regions and the pressure RHS on a projected copy. The
projection only removes or neutralizes fields that the pre-pass has already
bound. This establishes internal source consistency, never producer origin,
solver accuracy or model freshness.
"""
from __future__ import annotations

from copy import deepcopy
from functools import lru_cache
import hashlib
import json
import math
import struct
from pathlib import Path
from collections.abc import Mapping
from typing import Any

CONTRACT_ID = "openpipestress.result_semantics/0.3.0/load-reference-1"
PROFILE = "resolved_straight_load_state_v1"
TABLE_SHA256 = "44bc41c06f589fab6ce931ac0eaa5344765ff64fd5f880cc2dd69ecb839c4f4d"
TABLE_PATH = Path(__file__).resolve().parents[2] / "fixtures" / "results" / "semantic_contract_v0_3_load_reference_1.json"
TRANSPORT_SCHEMA_SHA256 = "640fd4477ac2c84f3c02268cfccc3958f51ee6508b3a67e5538b52d84899af65"
TRANSPORT_SCHEMA_PATH = Path(__file__).resolve().parents[2] / "schemas" / "load_reference_state.schema.json"

RECORD_CONTRACT = "openpipestress.load_reference_state/1.0.0"
MATERIAL_BASIS = "resolved_per_member_load_reference_state_v1"
NOT_JOINED = "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED"
# Selected retained-source method and its info diagnostic (joined method only).
EXACT_METHOD = "retained_source_blocks_exact_v1"
SELECTED = "SOURCE_BLOCK_RECOVERY_SELECTED"
# A failed retained-source attempt; never emitted for a selected case.
UNAVAILABLE = "SOURCE_BLOCK_RECOVERY_UNAVAILABLE"
# The method whose namespace is admitted: load-reference-1 (unchanged) or the
# joined load-reference-source-1 pre-pass (``load_reference_source``).
LOAD_REFERENCE = "load_reference"
JOINED = "joined"
REGION_TEMPERATURE_BASIS = "resolved_member_state"
G_BASIS = "E/[2(1+nu)] from the selected pair"
COMPOSITION = "lambda_fit*lambda_thermal-1"
BOUNDARY = "every restrained DOF prescribed; reduced K_ff u_f = f_f - K_fc g_c; complete u includes g; reactions from unreduced K u - f"
EIGENLOAD = "axial E_member*A_s*total_eigenstrain assembled once and removed once in recovery"

EVIDENCE_KEYS = ["pressure", "connector", "exact_cases", "load_reference_states"]
CASE_KEYS = ["load_case_id", "profile_mode", "material_basis", "pipe_materials", "pipe_sections", "pipe_stress_extrema", "stress_maximum_coverage", "pressure_rhs_assembly"]
MATERIAL_KEYS = ["pipe_id", "material_id", "E_pa", "nu", "G_pa", "constitutive_basis", "material_selection_kind", "thermal_consumed", "alpha_per_kelvin", "resolved_eigenstrain", "provenance"]
RECORD_KEYS = ["load_case_id", "contract", "profile", "reference_configuration_id", "provenance", "reference_geometry", "history", "solve", "source_recovery", "members", "support_components", "contributions", "excluded_sources"]
MEMBER_KEYS = [
    "pipe_id", "material_id", "material_selection_kind", "consumed_material_points", "interpolation_fraction",
    "applicability_reference", "analysis_basis_override", "selected_E_pa", "selected_nu", "derived_G_pa", "G_basis",
    "retained_G_ignored", "operating_temperature_k", "material_selection_temperature_k", "reference_basis",
    "installation_temperature_k", "thermal_definition", "expansion_law_id", "coefficient_datum_k",
    "consumed_law_point_indices", "consumed_law_segments", "consulted_law_point_indices", "consulted_law_segments",
    "installation_datum_stretch", "operating_datum_stretch", "thermal_strain", "thermal_stretch", "fit_strain",
    "fit_stretch", "total_eigenstrain", "eigenstrain_composition", "fit_kind", "fit_input", "reference_length_m",
]
POINT_KEYS = ["point_id", "temperature_k", "E_pa", "nu", "retained_G_ignored"]
SEGMENT_KEYS = ["use", "lower_index", "upper_index", "start_k", "end_k"]
SUPPORT_KEYS = ["support_id", "node_id", "dof", "global_dof", "law_kind", "prescribed_value", "unit", "meaning", "physical_state_source"]
STORED_KEYS = ["source_id", "owner_kind", "classification", "factor", "category", "dimension", "authored_normalized_magnitude", "applied_magnitude"]
MEMBER_STATE_KEYS = ["source_id", "owner_kind", "classification", "consumed_input_refs", "value"]
SUPPORT_STATE_KEYS = ["source_id", "owner_kind", "classification", "value"]
PRESSURE_REGION_KEYS = ["source_id", "owner_kind", "classification", "factor"]
EXCLUDED_KEYS = ["source_id", "owner_kind", "classification", "category", "reason"]
REGION_KEYS = ["profile_version", "profile_mode", "load_case_id", "region_id", "member_pipe_ids", "pressure_basis", "p_pa", "external_pressure_increment_pa", "approximation", "geometry_representation_guard", "geometry", "materials", "applied_loads", "terminals", "provenance", "result_ids"]
REGION_MATERIAL_KEYS = ["pipe_id", "material_id", "E_pa", "nu", "G_pa", "constitutive_basis", "thermal_consumed", "alpha_per_kelvin", "provenance", "temperature_basis"]
DOFS = ["UX", "UY", "UZ", "RX", "RY", "RZ"]
LAW_DEFINITIONS = ["engineering_secant", "engineering_dilation", "differential_per_datum_length", "logarithmic_per_current_length"]
DIRECT_DEFINITIONS = ["unchanged_reference", "explicit_interval_strain", "constant_alpha_interval"]
U64_MAX = (1 << 64) - 1


class LoadReferenceError(ValueError):
    """A shared-code rejection; ``str(error)`` is the exact Rust error string."""


def _code(name: str) -> str:
    return f"SOURCE_LOAD_REFERENCE_{name}"


def _fail(name: str) -> LoadReferenceError:
    return LoadReferenceError(_code(name))


def _require(ok: bool, name: str) -> None:
    if not ok:
        raise _fail(name)


def _get(value: Any, key: str) -> Any:
    """Rust ``Value[key]`` semantics: missing key or non-object reads as null."""
    return value.get(key) if isinstance(value, Mapping) else None


def _keys(value: Any, required: list[str]) -> bool:
    return isinstance(value, Mapping) and len(value) == len(required) and all(key in value for key in required)


def _is_number(value: Any) -> bool:
    """A JSON number of any magnitude (``bool`` excluded), as ``Value::Number`` in Rust."""
    return type(value) in (int, float)


def _as_f64(value: Any) -> float | None:
    """Rust ``Value::as_f64``: the binary64 value of a JSON number, else ``None``.

    ``serde_json`` holds an integer literal outside i64/u64 as the correctly
    rounded binary64 (``float_roundtrip``), and converts i64/u64 integers with
    ``as f64`` (round to nearest, ties to even). ``float(int)`` gives the same
    value. An integer whose binary64 conversion overflows cannot exist in a
    Rust ``Value`` (the JSON text is refused), so it reads as ``None`` here and
    is refused as ``NUMBER_INVALID``, like a non-finite number.
    """
    if type(value) is float:
        return value
    if type(value) is int:
        try:
            return float(value)
        except OverflowError:
            return None
    return None


def _text(value: Any) -> str:
    if not (isinstance(value, str) and value):
        raise _fail("STRING_INVALID")
    return value


def _number(value: Any) -> float:
    number = _as_f64(value)
    if number is None or not math.isfinite(number):
        raise _fail("NUMBER_INVALID")
    return number


def _opt_number(value: Any) -> float | None:
    return None if value is None else _number(value)


def _opt_text(value: Any) -> str | None:
    return None if value is None else _text(value)


def _boolean(value: Any) -> bool:
    if type(value) is not bool:
        raise _fail("BOOLEAN_INVALID")
    return value


def _index(value: Any) -> int:
    if not (type(value) is int and 0 <= value <= U64_MAX):
        raise _fail("INTEGER_INVALID")
    return value


def _array(value: Any) -> list[Any]:
    if not isinstance(value, list):
        raise _fail("ARRAY_INVALID")
    return value


def _num_eq(a: Any, b: Any) -> bool:
    x, y = _as_f64(a), _as_f64(b)
    return x is not None and y is not None and math.isfinite(x) and math.isfinite(y) and x == y


def _same(a: Any, b: Any) -> bool:
    """Structural equality comparing numbers by binary64 value (Rust ``same``)."""
    if type(a) is bool or type(b) is bool:
        return type(a) is bool and type(b) is bool and a == b
    if _is_number(a) and _is_number(b):
        return _num_eq(a, b)
    if isinstance(a, list) and isinstance(b, list):
        return len(a) == len(b) and all(_same(x, y) for x, y in zip(a, b))
    if isinstance(a, Mapping) and isinstance(b, Mapping):
        return len(a) == len(b) and all(key in b and _same(value, b[key]) for key, value in a.items())
    if _is_number(a) or _is_number(b) or isinstance(a, (list, Mapping)) or isinstance(b, (list, Mapping)):
        return False
    return type(a) is type(b) and a == b


def _eq(value: Any, expected: Any) -> bool:
    """Rust ``Value == literal`` for string/bool/null literals."""
    if type(expected) is bool or type(value) is bool:
        return type(value) is type(expected) and value == expected
    if expected is None:
        return value is None
    return isinstance(value, str) and value == expected


def _without(value: Any, removed: list[str]) -> Any:
    if not isinstance(value, Mapping):
        return deepcopy(value)
    return {key: deepcopy(item) for key, item in value.items() if key not in removed}


def _sha64(value: Any) -> bool:
    return isinstance(value, str) and len(value) == 64 and all(c in "0123456789abcdef" for c in value)


def _finite_tree(value: Any) -> None:
    if type(value) is bool or value is None or isinstance(value, str):
        return
    if type(value) in (int, float):
        _number(value)
    elif isinstance(value, Mapping):
        for item in value.values():
            _finite_tree(item)
    elif isinstance(value, list):
        for item in value:
            _finite_tree(item)
    else:
        raise _fail("NUMBER_INVALID")


def _bits(value: float) -> int:
    return struct.unpack(">Q", struct.pack(">d", value))[0]


def _positive(value: float | None) -> bool:
    return value is None or value > 0


def verify_load_reference_table(data: bytes) -> dict[str, Any]:
    """Pinned table bytes: identity, profile and sha256 are checked, never inferred."""
    if hashlib.sha256(data).hexdigest() != TABLE_SHA256:
        raise LoadReferenceError("SOURCE_LOAD_REFERENCE_TABLE_HASH")
    table = json.loads(data)
    if not isinstance(table, Mapping) or table.get("semantic_contract_id") != CONTRACT_ID or table.get("formulation_profile_id") != PROFILE:
        raise LoadReferenceError("SOURCE_LOAD_REFERENCE_TABLE_IDENTITY")
    return table


@lru_cache(maxsize=1)
def load_reference_table() -> dict[str, Any]:
    return verify_load_reference_table(TABLE_PATH.read_bytes())


def transport_schema() -> dict[str, Any]:
    data = TRANSPORT_SCHEMA_PATH.read_bytes()
    if hashlib.sha256(data).hexdigest() != TRANSPORT_SCHEMA_SHA256:
        raise LoadReferenceError("SOURCE_LOAD_REFERENCE_TRANSPORT_SCHEMA_HASH")
    return json.loads(data)


def validate_load_reference_evidence(source: Mapping[str, Any]) -> None:
    """Raw load-reference-1 publication: closed evidence, joins, rows and diagnostics."""
    _guarded(source, raw=True)


def validate_load_reference_transport_metadata(source: Mapping[str, Any]) -> None:
    """Retained statements without raw rows: frozen schema shape and internal joins only."""
    _guarded(source, raw=False)


def _guarded(source: Mapping[str, Any], *, raw: bool) -> None:
    try:
        _validate(source, raw)
    except (TypeError, KeyError, AttributeError, OverflowError, IndexError) as error:
        raise LoadReferenceError("SOURCE_LOAD_REFERENCE_MALFORMED") from error


def _validate(source: Mapping[str, Any], raw: bool) -> None:
    _prepass(source, raw, LOAD_REFERENCE)
    # S14 inherited physics-1 checks on the projected copy.
    projected = _project(source)
    from .physics_evidence import validate_physics_evidence, validate_transport_metadata
    try:
        if raw:
            validate_physics_evidence(projected)
        else:
            validate_transport_metadata(projected)
    except ValueError as error:
        raise LoadReferenceError(f"{_code('PHYSICS_EVIDENCE')}: {error}") from error
    except (TypeError, KeyError, AttributeError, OverflowError, IndexError) as error:
        raise LoadReferenceError(f"{_code('PHYSICS_EVIDENCE')}: SOURCE_PHYSICS_EVIDENCE_INVALID: malformed evidence") from error


def _is_selected(record: Any) -> bool:
    """A record whose case publishes the selected retained-source response."""
    return _eq(_get(_get(record, "solve"), "recovery_method"), EXACT_METHOD)


def _prepass(source: Mapping[str, Any], raw: bool, method: str) -> None:
    """Steps S1-S13, shared with the joined reader (Rust ``load_reference::prepass``).

    The joined method differs only in S1 (the receipt is required), the
    transport schema (not applied: the frozen schema admits only not-joined
    records), S7 (``recovery_method``), S10 (a selected record), S10b and S13.
    """
    joined = method == JOINED
    # S1 foreign method namespaces.
    if joined:
        _require(not isinstance(source, Mapping) or "carrier_evidence" not in source, "FOREIGN_METHOD_EVIDENCE")
        _require(isinstance(source, Mapping) and "source_block_recovery" in source, "JOIN_RECEIPT_REQUIRED")
    else:
        _require(not isinstance(source, Mapping) or ("source_block_recovery" not in source and "carrier_evidence" not in source), "FOREIGN_METHOD_EVIDENCE")
    # S2 finite numbers everywhere.
    _finite_tree(source)
    evidence = _get(source, "contract_evidence")
    if not raw and not joined:
        from .source_blocks import _shape as schema_shape
        schema = transport_schema()
        _require(schema_shape(evidence, schema["$defs"]["LoadReferenceContractEvidence"], schema), "TRANSPORT_SHAPE")
    # S3-S5 namespace.
    _require(_keys(evidence, EVIDENCE_KEYS), "EVIDENCE_SHAPE")
    _require(not _array(evidence["connector"]), "CONNECTOR_UNSUPPORTED")
    pressure = _array(evidence["pressure"])
    exact = _array(evidence["exact_cases"])
    records = _array(evidence["load_reference_states"])
    # S6 numerical case identities.
    quality_ids: list[str] = []
    for case in _array(_get(_get(source, "numerical_quality"), "cases")):
        basis = _get(case, "basis_ref")
        _require(_keys(basis, ["ref_type", "ref_id"]) and _eq(basis["ref_type"], "load_case"), "NUMERICAL_CASE_BASIS")
        case_id = _text(basis["ref_id"])
        _require(case_id not in quality_ids, "NUMERICAL_CASE_DUPLICATE")
        quality_ids.append(case_id)
    # S7 exact cases, their resolved materials and section identities.
    cases: list[tuple[str, Mapping[str, Any]]] = []
    case_keys = CASE_KEYS + (["recovery_method"] if joined else [])
    for case in exact:
        _require(_keys(case, case_keys), "CASE_SHAPE")
        case_id = _text(case["load_case_id"])
        _require(all(known != case_id for known, _ in cases), "CASE_DUPLICATE")
        _require(_eq(case["material_basis"], MATERIAL_BASIS), "MATERIAL_BASIS")
        pipes: list[str] = []
        for material in _array(case["pipe_materials"]):
            _require(_keys(material, MATERIAL_KEYS), "MATERIAL_SHAPE")
            pipe = _text(material["pipe_id"])
            _require(pipe not in pipes, "MATERIAL_DUPLICATE")
            pipes.append(pipe)
        sections: list[str] = []
        for section in _array(case["pipe_sections"]):
            pipe = _text(_get(section, "pipe_id"))
            _require(pipe not in sections, "SECTION_DUPLICATE")
            sections.append(pipe)
        cases.append((case_id, case))
    # S8 record identities.
    record_ids: list[str] = []
    for record in records:
        _require(_keys(record, RECORD_KEYS), "RECORD_SHAPE")
        case_id = _text(record["load_case_id"])
        _require(case_id not in record_ids, "RECORD_DUPLICATE")
        record_ids.append(case_id)
    # S9 case trijection. An unsolved envelope retains the empty namespace.
    solved = (not raw) or _eq(_get(_get(source, "status"), "mechanics"), "MECHANICS_SOLVED")
    if solved:
        quality = set(quality_ids)
        _require(bool(record_ids) and set(record_ids) == quality and {case_id for case_id, _ in cases} == quality, "CASE_COVERAGE")
    else:
        _require(not records and not exact and not pressure, "UNSOLVED_EVIDENCE")
    # S10 each record in document order.
    for record in records:
        _validate_record(record, cases, pressure, method)
    # S10b (joined) the published method of each case agrees with its exact
    # case, and at least one case publishes the selected response.
    if joined:
        for record in records:
            case = next((value for known, value in cases if _eq(record["load_case_id"], known)), None)
            if case is None:
                raise _fail("RECORD_CASE_UNRESOLVED")
            _require(_same(case["recovery_method"], _get(record["solve"], "recovery_method")), "JOIN_RECOVERY_METHOD")
        _require(any(_is_selected(record) for record in records), "JOIN_SELECTION_REQUIRED")
    # S11 one model geometry and one requested mode per envelope.
    if records:
        first = records[0]
        for record in records:
            _require(_same(_get(record["reference_geometry"], "projection_sha256"), _get(first["reference_geometry"], "projection_sha256")), "REFERENCE_GEOMETRY_CONSISTENCY")
            _require(_same(_get(record["solve"], "requested_mode"), _get(first["solve"], "requested_mode")), "SOLVE_CONSISTENCY")
    # S12 pressure-region materials are the resolved member pair of their case.
    for region in pressure:
        _require(_keys(region, REGION_KEYS), "REGION_SHAPE")
        case_id = _text(region["load_case_id"])
        _text(region["region_id"])
        case = next((value for known, value in cases if known == case_id), None)
        if case is None:
            raise _fail("REGION_CASE_UNRESOLVED")
        for material in _array(region["materials"]):
            _require(_keys(material, REGION_MATERIAL_KEYS), "REGION_MATERIAL_SHAPE")
            _require(_eq(material["temperature_basis"], REGION_TEMPERATURE_BASIS), "REGION_TEMPERATURE_BASIS")
            pipe = _text(material["pipe_id"])
            member = next((m for m in _array(case["pipe_materials"]) if _eq(_get(m, "pipe_id"), pipe)), None)
            if member is None:
                raise _fail("REGION_MATERIAL_BINDING")
            _require(_same(_without(material, ["temperature_basis"]), _without(member, ["material_selection_kind", "resolved_eigenstrain"])), "REGION_MATERIAL_BINDING")
    # S13 exactly one not-joined info diagnostic per resolved case. In the
    # joined method a selected case instead carries exactly one info
    # SOURCE_BLOCK_RECOVERY_SELECTED diagnostic and no not-joined diagnostic.
    if raw and solved:
        diagnostics = _array(_get(source, "diagnostics"))
        not_joined = selected = 0
        for case_id, record in zip(record_ids, records):
            if joined and _is_selected(record):
                selected += 1
                kind, expected, name = SELECTED, f"diagnostic:source-recovery:{case_id}:selected", "JOIN_SELECTED_DIAGNOSTIC"
            else:
                not_joined += 1
                kind, expected, name = NOT_JOINED, f"diagnostic:load-state:{case_id.replace(':', '-')}:source-recovery-not-joined", "NOT_JOINED_DIAGNOSTIC"
            hits = [d for d in diagnostics if _eq(_get(d, "code"), kind) and _eq(_get(d, "id"), expected)]
            _require(len(hits) == 1 and _eq(_get(hits[0], "severity"), "info") and _same(_get(hits[0], "affected_refs"), [case_id]), name)
        _require(sum(1 for d in diagnostics if _eq(_get(d, "code"), NOT_JOINED)) == not_joined, "NOT_JOINED_DIAGNOSTIC")
        if joined:
            _require(sum(1 for d in diagnostics if _eq(_get(d, "code"), SELECTED)) == selected, "JOIN_SELECTED_DIAGNOSTIC")
            # A selected case's attempt succeeded, so no UNAVAILABLE diagnostic
            # may name it (the producer never emits one; review note N-2).
            selected_ids = [case_id for case_id, record in zip(record_ids, records) if _is_selected(record)]
            _require(not any(_eq(_get(d, "code"), UNAVAILABLE) and isinstance(_get(d, "affected_refs"), list) and any(isinstance(r, str) and r in selected_ids for r in d["affected_refs"]) for d in diagnostics), "JOIN_SELECTED_UNAVAILABLE_DIAGNOSTIC")


def _project(source: Mapping[str, Any]) -> dict[str, Any]:
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


def _validate_record(record: Mapping[str, Any], cases: list[tuple[str, Mapping[str, Any]]], pressure: list[Any], method: str = LOAD_REFERENCE) -> None:
    case_id = _text(record["load_case_id"])
    # R1-R7 record-level closed statements.
    _require(_eq(record["contract"], RECORD_CONTRACT), "RECORD_CONTRACT")
    _require(_eq(record["profile"], PROFILE), "RECORD_PROFILE")
    configuration = _text(record["reference_configuration_id"])
    _text(record["provenance"])
    geometry = record["reference_geometry"]
    _require(_keys(geometry, ["kind", "projection_sha256"]), "REFERENCE_GEOMETRY_SHAPE")
    _require(_eq(geometry["kind"], "authored_model_geometry") and _sha64(geometry["projection_sha256"]), "REFERENCE_GEOMETRY")
    history = record["history"]
    _require(_keys(history, ["kind"]), "HISTORY_SHAPE")
    _require(_eq(history["kind"], "independent_equilibrium"), "HISTORY")
    solve = record["solve"]
    _require(_keys(solve, ["requested_mode", "recovery_method", "boundary", "eigenload"]), "SOLVE_SHAPE")
    pair = (solve["requested_mode"], solve["recovery_method"])
    # The joined method also admits the selected retained-source response in
    # either requested mode (ADDENDUM_2 section 5.3).
    selected = method == JOINED and _eq(solve["recovery_method"], EXACT_METHOD)
    _require(
        all(isinstance(item, str) for item in pair)
        and (pair in {("sparse_interactive", "ordinary_sparse_structural_v1"), ("dense_scrutiny", "ordinary_dense_structural_v1")}
             or (selected and pair[0] in {"sparse_interactive", "dense_scrutiny"}))
        and _eq(solve["boundary"], BOUNDARY) and _eq(solve["eigenload"], EIGENLOAD),
        "SOLVE",
    )
    recovery = record["source_recovery"]
    if selected:
        _require(_keys(recovery, ["status", "method"]), "SOURCE_RECOVERY_SHAPE")
        _require(_eq(recovery["status"], "selected") and _eq(recovery["method"], EXACT_METHOD), "SOURCE_RECOVERY")
    else:
        _require(_keys(recovery, ["status", "code"]), "SOURCE_RECOVERY_SHAPE")
        _require(_eq(recovery["status"], "not_joined") and _eq(recovery["code"], NOT_JOINED), "SOURCE_RECOVERY")
    # R8 members.
    members = _array(record["members"])
    member_ids: list[str] = []
    for member in members:
        _validate_member(member)
        pipe = _text(member["pipe_id"])
        _require(pipe not in member_ids, "MEMBER_DUPLICATE")
        member_ids.append(pipe)
    # R9 prescribed support components.
    components = _array(record["support_components"])
    component_ids: list[tuple[str, str]] = []
    support_nodes: dict[str, str] = {}
    node_bases: dict[str, int] = {}
    for component in components:
        _validate_support_component(component)
        support = _text(component["support_id"])
        node = _text(component["node_id"])
        dof = _text(component["dof"])
        _require((support, dof) not in component_ids, "SUPPORT_COMPONENT_DUPLICATE")
        component_ids.append((support, dof))
        base = _index(component["global_dof"]) // 6
        _require(support_nodes.setdefault(support, node) == node and node_bases.setdefault(node, base) == base, "SUPPORT_COMPONENT_NODE")
    # R10 contributions.
    contributions = _array(record["contributions"])
    contribution_ids: list[str] = []
    for contribution in contributions:
        _validate_contribution(contribution)
        source_id = _text(contribution["source_id"])
        _require(source_id not in contribution_ids, "CONTRIBUTION_DUPLICATE")
        contribution_ids.append(source_id)
    # R11 excluded sources.
    excluded_ids: list[str] = []
    for excluded in _array(record["excluded_sources"]):
        _require(_keys(excluded, EXCLUDED_KEYS), "EXCLUDED_SHAPE")
        _require(_eq(excluded["owner_kind"], "stored_primitive") and _eq(excluded["classification"], "excluded"), "EXCLUDED_CLASSIFICATION")
        source_id = _text(excluded["source_id"])
        _text(excluded["category"])
        _text(excluded["reason"])
        _require(source_id not in contribution_ids and source_id not in excluded_ids, "EXCLUDED_OVERLAP")
        excluded_ids.append(source_id)
    # R12-R14 binding to the exact case of the same load case.
    case = next((value for known, value in cases if known == case_id), None)
    if case is None:
        raise _fail("RECORD_CASE_UNRESOLVED")
    sections = {s["pipe_id"] if isinstance(_get(s, "pipe_id"), str) else "" for s in _array(case["pipe_sections"])}
    _require(bool(member_ids) and set(member_ids) == sections, "MEMBER_COVERAGE")
    materials = _array(case["pipe_materials"])
    _require({m["pipe_id"] if isinstance(_get(m, "pipe_id"), str) else "" for m in materials} == set(member_ids), "MEMBER_MATERIAL_COVERAGE")
    for member in members:
        material = next((m for m in materials if _same(_get(m, "pipe_id"), member["pipe_id"])), None)
        if material is None:
            raise _fail("MEMBER_MATERIAL_COVERAGE")
        _require(
            _num_eq(material["E_pa"], member["selected_E_pa"])
            and _num_eq(material["nu"], member["selected_nu"])
            and _num_eq(material["G_pa"], member["derived_G_pa"])
            and _same(material["material_id"], member["material_id"])
            and _same(material["material_selection_kind"], member["material_selection_kind"])
            and _num_eq(material["resolved_eigenstrain"], member["total_eigenstrain"])
            and _eq(material["thermal_consumed"], False)
            and material["alpha_per_kelvin"] is None,
            "MEMBER_MATERIAL_BINDING",
        )
        _text(material["provenance"])
    # R15 contribution ledgers are bijections with their owners.
    def of_kind(kind: str) -> set[str]:
        return {c["source_id"] for c in contributions if _eq(_get(c, "owner_kind"), kind) and isinstance(_get(c, "source_id"), str)}

    def find(source_id: str) -> Any:
        return next((c for c in contributions if _eq(_get(c, "source_id"), source_id)), None)

    _require(of_kind("resolved_member_state") == {f"member_state:{pipe}" for pipe in member_ids}, "MEMBER_CONTRIBUTION_COVERAGE")
    for member in members:
        pipe = _text(member["pipe_id"])
        contribution = find(f"member_state:{pipe}")
        if contribution is None:
            raise _fail("MEMBER_CONTRIBUTION_COVERAGE")
        _require(
            _num_eq(_get(contribution, "value"), member["total_eigenstrain"])
            and _same(_get(contribution, "consumed_input_refs"), [f"{configuration}:{pipe}", f"{case_id}:element_state:{pipe}"]),
            "MEMBER_CONTRIBUTION_BINDING",
        )
    _require(of_kind("support_state") == {f"support_state:{support}:{dof}" for support, dof in component_ids}, "SUPPORT_CONTRIBUTION_COVERAGE")
    for component in components:
        source_id = f"support_state:{_text(component['support_id'])}:{_text(component['dof'])}"
        contribution = find(source_id)
        if contribution is None:
            raise _fail("SUPPORT_CONTRIBUTION_COVERAGE")
        _require(_num_eq(_get(contribution, "value"), component["prescribed_value"]), "SUPPORT_CONTRIBUTION_BINDING")
    expected_regions: set[str] = set()
    for region in pressure:
        if _eq(_get(region, "load_case_id"), case_id):
            expected_regions.add(f"pressure_region:{_text(_get(region, 'region_id'))}")
    _require(of_kind("pressure_region") == expected_regions, "PRESSURE_CONTRIBUTION_COVERAGE")


def _validate_member(member: Any) -> None:
    # M1-M3 shape, identity and fixed statements.
    _require(_keys(member, MEMBER_KEYS), "MEMBER_SHAPE")
    _text(member["pipe_id"])
    _text(member["material_id"])
    _require(_eq(member["G_basis"], G_BASIS) and _eq(member["eigenstrain_composition"], COMPOSITION), "MEMBER_BASIS")
    # M4 selected pair and derived G.
    e = _number(member["selected_E_pa"])
    nu = _number(member["selected_nu"])
    g = _number(member["derived_G_pa"])
    _require(e > 0 and -1 < nu < 0.5 and g > 0, "MEMBER_MATERIAL_RANGE")
    direct = e / (2.0 * (1.0 + nu))
    _require(math.isfinite(direct) and abs(_bits(g) - _bits(direct)) <= 2, "MEMBER_G_BINDING")
    retained = _boolean(member["retained_G_ignored"])
    # M6 consumed material points.
    consumed: list[tuple[float | None, float, float, bool]] = []
    for point in _array(member["consumed_material_points"]):
        _require(_keys(point, POINT_KEYS), "MATERIAL_POINT_SHAPE")
        _text(point["point_id"])
        temperature = _opt_number(point["temperature_k"])
        _require(_positive(temperature), "TEMPERATURE_RANGE")
        pe = _number(point["E_pa"])
        pnu = _number(point["nu"])
        _require(pe > 0 and -1 < pnu < 0.5, "MATERIAL_POINT_RANGE")
        consumed.append((temperature, pe, pnu, _boolean(point["retained_G_ignored"])))
    # M7-M9 optional selection data and temperatures.
    fraction = _opt_number(member["interpolation_fraction"])
    applicability = _opt_text(member["applicability_reference"])
    override = member["analysis_basis_override"]
    if override is not None:
        _require(_keys(override, ["reason", "provenance"]), "OVERRIDE_SHAPE")
        _text(override["reason"])
        _text(override["provenance"])
    operating = _opt_number(member["operating_temperature_k"])
    selection = _opt_number(member["material_selection_temperature_k"])
    installation = _opt_number(member["installation_temperature_k"])
    datum = _opt_number(member["coefficient_datum_k"])
    _require(all(_positive(value) for value in (operating, selection, installation, datum)), "TEMPERATURE_RANGE")
    # M10 material selection.
    kind = member["material_selection_kind"]
    if _eq(kind, "explicit_base_properties"):
        selected = not consumed and fraction is None and applicability is not None and selection is None
    elif _eq(kind, "exact_point"):
        selected = len(consumed) == 1 and fraction is None and applicability is None and e == consumed[0][1] and nu == consumed[0][2] and selection == consumed[0][0] and retained == consumed[0][3]
    elif _eq(kind, "temperature_interpolation"):
        if len(consumed) == 1:
            shape = fraction is None and e == consumed[0][1] and nu == consumed[0][2] and selection == consumed[0][0]
        elif len(consumed) == 2:
            shape = fraction is not None and 0 < fraction < 1 and all(point[0] is not None for point in consumed)
        else:
            shape = False
        selected = applicability is None and selection is not None and retained == any(point[3] for point in consumed) and shape
    else:
        raise _fail("MATERIAL_SELECTION_KIND")
    _require(selected, "MATERIAL_SELECTION")
    # M11 reference basis.
    basis = member["reference_basis"]
    _require((_eq(basis, "temperature_reference") and installation is not None) or (_eq(basis, "direct_strain_reference") and installation is None), "REFERENCE_BASIS")
    # M12 strains and stretches.
    thermal_strain = _number(member["thermal_strain"])
    thermal_stretch = _number(member["thermal_stretch"])
    fit_strain = _number(member["fit_strain"])
    fit_stretch = _number(member["fit_stretch"])
    total = _number(member["total_eigenstrain"])
    _require(thermal_stretch > 0 and fit_stretch > 0 and thermal_stretch == 1.0 + thermal_strain and fit_stretch == 1.0 + fit_strain and 1.0 + total > 0, "STRETCH_BINDING")
    # M13 consumed and consulted law data.
    law_data_empty = True
    for key in ("consumed_law_point_indices", "consulted_law_point_indices"):
        previous: int | None = None
        for item in _array(member[key]):
            index = _index(item)
            _require(previous is None or previous < index, "LAW_INDICES")
            previous = index
            law_data_empty = False
    # CP3_WIRE_ADDENDUM section 1: adjacent indices, a sample at one temperature,
    # an interval over start < end, and no repeated entry within one list. The
    # consumed and consulted lists are independent and may share entries.
    for key in ("consumed_law_segments", "consulted_law_segments"):
        entries: list[tuple[bool, int, int, float, float]] = []
        for segment in _array(member[key]):
            _require(_keys(segment, SEGMENT_KEYS), "LAW_SEGMENT_SHAPE")
            if _eq(segment["use"], "interpolation_sample"):
                sample = True
            elif _eq(segment["use"], "integration_interval"):
                sample = False
            else:
                raise _fail("LAW_SEGMENT")
            lower = _index(segment["lower_index"])
            upper = _index(segment["upper_index"])
            start = _number(segment["start_k"])
            end = _number(segment["end_k"])
            _require(upper == lower + 1 and (start == end if sample else start < end), "LAW_SEGMENT")
            entry = (sample, lower, upper, start, end)
            _require(entry not in entries, "LAW_SEGMENT_DUPLICATE")
            entries.append(entry)
            law_data_empty = False
    installation_stretch = _opt_number(member["installation_datum_stretch"])
    operating_stretch = _opt_number(member["operating_datum_stretch"])
    law_id = _opt_text(member["expansion_law_id"])
    # M14 thermal definition.
    definition = member["thermal_definition"] if isinstance(member["thermal_definition"], str) else ""
    if definition in LAW_DEFINITIONS:
        _require(
            law_id is not None and datum is not None
            and installation_stretch is not None and installation_stretch > 0
            and operating_stretch is not None and operating_stretch > 0
            and installation is not None and operating is not None,
            "THERMAL_LAW_BINDING",
        )
    elif definition in DIRECT_DEFINITIONS:
        _require(
            law_id is None and datum is None and installation_stretch is None and operating_stretch is None
            and law_data_empty and (definition != "unchanged_reference" or thermal_strain == 0),
            "THERMAL_LAW_BINDING",
        )
    else:
        raise _fail("THERMAL_DEFINITION")
    # M15-M16 reference length and fit.
    length = _number(member["reference_length_m"])
    _require(length > 0, "REFERENCE_LENGTH")
    fit = member["fit_input"]
    fit_kind = member["fit_kind"]
    if _eq(fit_kind, "none"):
        fitted = fit is None and fit_strain == 0
    elif _eq(fit_kind, "natural_length_change"):
        change = _as_f64(_get(fit, "length_change_m"))
        fitted = _keys(fit, ["length_change_m"]) and change is not None and math.isfinite(change) and fit_strain == change / length and length + change > 0
    elif _eq(fit_kind, "fit_strain"):
        strain = _as_f64(_get(fit, "strain"))
        fitted = _keys(fit, ["strain"]) and strain is not None and math.isfinite(strain) and fit_strain == strain
    else:
        raise _fail("FIT_KIND")
    _require(fitted, "FIT_BINDING")


def _validate_support_component(component: Any) -> None:
    _require(_keys(component, SUPPORT_KEYS), "SUPPORT_COMPONENT_SHAPE")
    _text(component["support_id"])
    _text(component["node_id"])
    dof = component["dof"]
    if not (isinstance(dof, str) and dof in DOFS):
        raise _fail("SUPPORT_COMPONENT_DOF")
    position = DOFS.index(dof)
    global_dof = _index(component["global_dof"])
    _require(global_dof % 6 == position, "SUPPORT_COMPONENT_DOF")
    _require(_eq(component["law_kind"], "rigid_prescribed") and _eq(component["meaning"], "absolute_reference_displacement") and _eq(component["physical_state_source"], "support_state.boundary_motion"), "SUPPORT_COMPONENT_LAW")
    _require(_eq(component["unit"], "m" if position < 3 else "rad"), "SUPPORT_COMPONENT_UNIT")
    _number(component["prescribed_value"])


def _validate_contribution(contribution: Any) -> None:
    kind = _get(contribution, "owner_kind")
    shapes = {
        "stored_primitive": (STORED_KEYS, "ordinary_applied"),
        "resolved_member_state": (MEMBER_STATE_KEYS, "eigenstrain"),
        "support_state": (SUPPORT_STATE_KEYS, "prescribed_boundary"),
        "pressure_region": (PRESSURE_REGION_KEYS, "pressure_eigen_and_closure"),
    }
    if not (isinstance(kind, str) and kind in shapes):
        raise _fail("CONTRIBUTION_KIND")
    shape, classification = shapes[kind]
    _require(_keys(contribution, shape), "CONTRIBUTION_SHAPE")
    _require(_eq(contribution["classification"], classification), "CONTRIBUTION_CLASSIFICATION")
    _text(contribution["source_id"])
    if kind == "stored_primitive":
        factor = _as_f64(contribution["factor"])
        if not (factor is not None and math.isfinite(factor) and factor != 0):
            raise _fail("CONTRIBUTION_FACTOR")
        _text(contribution["category"])
        _text(contribution["dimension"])
        authored = _number(contribution["authored_normalized_magnitude"])
        applied = _number(contribution["applied_magnitude"])
        _require(applied == authored * factor, "CONTRIBUTION_APPLIED_MAGNITUDE")
    elif kind == "resolved_member_state":
        for reference in _array(contribution["consumed_input_refs"]):
            _text(reference)
        _number(contribution["value"])
    elif kind == "support_state":
        _number(contribution["value"])
    else:
        _require(contribution["factor"] is None, "CONTRIBUTION_FACTOR")
