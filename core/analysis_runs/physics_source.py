"""Explicit physics-source-1 consumer binding; no imported record proves custody.

The untouched received source is checked in both method contexts. Actual numerical
eligibility additionally requires the independently captured full invocation.
"""
from __future__ import annotations

import json
import math
import sys
from pathlib import Path
from typing import Any
from collections.abc import Mapping
from . import physics_evidence as physical
from . import source_blocks as blocks

ROOT = Path(__file__).resolve().parents[2]
CONTRACT_ID = "openpipestress.result_semantics/0.3.0/physics-source-1"
PROFILE = "exact_straight_pressure_v2"
EXACT = blocks.EXACT
MAX_BASIS = "retained_source_endpoint_normal_max_v1"
MAX_SIGN = "nonnegative maximum absolute axial-plus-bending normal stress over an unloaded circular straight span; retained endpoint actions with projected-action and arithmetic bounds; endpoint witness does not imply uniqueness; torsional shear separate"
SUPPORT_SIGN = physical.SUPPORT_SIGN
TABLE = json.loads((ROOT / "fixtures/results/semantic_contract_v0_3_physics_source_1.json").read_text())
SCHEMA_PATH = ROOT / "schemas/physics_source_recovery.schema.json"


def _require(condition: Any, detail: str) -> None:
    if not condition:
        raise ValueError("PHYSICS_SOURCE_" + detail)


def _schema() -> dict[str, Any]:
    return json.loads(SCHEMA_PATH.read_text())


def validate_receipt_shape(receipt: Any) -> None:
    schema = _schema()
    _require(blocks._shape(receipt, schema, schema), "RECEIPT_SHAPE")


def _case(source: Mapping[str, Any], cid: str) -> Mapping[str, Any]:
    found = [case for case in source["source_block_recovery"]["body"]["cases"] if case["basis_ref"]["ref_id"] == cid]
    _require(len(found) == 1, "CASE_ID")
    return found[0]


def _validate_transport_metadata(source: Mapping[str, Any]) -> None:
    """Closed transported statements and crosshashes only; no publication/Current claim."""
    receipt = source["source_block_recovery"]
    validate_receipt_shape(receipt)
    body = receipt["body"]
    _require(receipt["receipt_sha256"] == blocks.domain_hash("source_blocks_receipt_v1", body), "RECEIPT_HASH")
    evidence = source["contract_evidence"]
    schema = _schema()
    _require(blocks._shape(evidence, schema["$defs"]["physical_evidence"], schema), "PHYSICAL_SHAPE")
    physical._finite_tree(evidence)
    cases = evidence["exact_cases"]
    _require([c["load_case_id"] for c in cases] == [c["basis_ref"]["ref_id"] for c in body["cases"]], "CASE_ORDER")
    _require(len({c["load_case_id"] for c in cases}) == len(cases), "CASE_UNIQUENESS")
    _require(any(c["selected_method"] == EXACT for c in body["cases"]), "SOURCE_SELECTION_REQUIRED")
    for record, case in zip(body["cases"], cases):
        cid = case["load_case_id"]
        pressure = [p for p in evidence["pressure"] if p["load_case_id"] == cid]
        _require(case["recovery_method"] == record["selected_method"], "RECOVERY_METHOD")
        _require(record["physical_evidence_sha256"] == blocks.domain_hash("physics_source_case_evidence_v1", {"exact_case": case, "pressure": pressure}), "PHYSICAL_CASE_HASH")
        if record["selected_method"] == EXACT:
            _require(not pressure, "SOURCE_PRESSURE_INVENTORY")
            plan = record["source"]
            _require([p["pipe_id"] for p in case["pipe_sections"]] == plan["member_ids"] == [p["pipe_id"] for p in case["pipe_materials"]] == [p["pipe_id"] for p in plan["endpoint_sections"]], "PHYSICAL_MEMBER_ORDER")
            _require(case["stress_maximum_coverage"] == {"complete": True, "unavailable_pipe_ids": []}, "MAXIMUM_COMPLETE")
            _require({p["pipe_id"] for p in case["pipe_stress_extrema"]} == set(plan["member_ids"]), "MAXIMUM_COVERAGE")
            _section_functionals(record)
            for extremum in case["pipe_stress_extrema"]:
                _maximum_link(case, extremum, record)
        else:
            _require(record["derived_checks"] == [] and record["section_stress_checks"] == [] and record["selected_method"] == blocks.MODES[record["requested_mode"]], "ORDINARY_METHOD")
    _require(all(p["load_case_id"] in {c["load_case_id"] for c in cases} for p in evidence["pressure"]), "PRESSURE_CASE")


def _same(a: Any, b: Any) -> bool:
    return blocks._bits(a) == blocks._bits(b)


def _finite(value: float) -> float:
    _require(math.isfinite(value), "ARITHMETIC_RANGE")
    return value


def _out(value: float, upper: bool) -> float:
    _finite(value)
    _require(value >= 0, "NEGATIVE_BOUND")
    return _finite(math.nextafter(value, math.inf) if upper else max(0.0, math.nextafter(value, -math.inf)))


def _div(a: float, b: float, upper: bool) -> float:
    return a if a == 0 or b == 1 else 1.0 if a == b else _out(a / b, upper)


def _mul(a: float, b: float, upper: bool) -> float:
    return 0.0 if a == 0 or b == 0 else b if a == 1 else a if b == 1 else _out(a * b, upper)


def _add(a: float, b: float, upper: bool) -> float:
    return b if a == 0 else a if b == 0 else _out(a + b, upper)


def _abs_interval(interval: list[float]) -> list[float]:
    lo, hi = interval
    return [lo, hi] if lo >= 0 else [-hi, -lo] if hi <= 0 else [0.0, max(-lo, hi)]


def _norm_bound(values: list[float], upper: bool) -> float:
    # The three-component support recipe preserves unit-largest-component terms.
    scale = max(values)
    if scale == 0: return 0.0
    squares = [0.0 if v == 0 else 1.0 if v == scale else _out(_out(v / scale, upper) ** 2, upper) for v in values]
    total = _add(_add(squares[0], squares[1], upper), squares[2], upper)
    root = 1.0 if total == 1 else _out(math.sqrt(total), upper)
    return scale if root == 1 else _out(scale * root, upper)


def _scaled_norm(values: list[float]) -> float:
    scale = max(abs(v) for v in values)
    if scale == 0: return 0.0
    squares = [(v / scale) * (v / scale) for v in values]
    total = squares[0] + squares[1]
    if len(squares) == 3: total += squares[2]
    return _finite(scale * math.sqrt(total))


def _bounds(value: float, lo: float, hi: float) -> tuple[float, float]:
    _require(0 <= lo <= value <= hi and all(math.isfinite(v) for v in (lo, value, hi)), "ENCLOSURE")
    absolute = 0.0 if lo == value == hi else _out(max(value - lo, hi - value), True)
    if hi == 0: return absolute, 0.0
    _require(min(lo, value, hi) >= sys.float_info.min, "PUBLICATION_RANGE")
    relative = 0.0 if absolute == 0 else _out(absolute / lo, True)
    _require(relative <= 1e-9, "PROTECTED_CRITERION")
    return absolute, relative


def _endpoint(area: float, z: float, actions: list[dict[str, Any]]) -> tuple[float, list[float]]:
    values = [abs(float(a["value"])) for a in actions]
    value = _finite(_finite(values[0] / area) + _scaled_norm([_finite(values[1] / z), _finite(values[2] / z)]))
    intervals = [_abs_interval(a["interval"]) for a in actions]
    bounds = []
    for side, upper in enumerate((False, True)):
        axial = _div(intervals[0][side], area, upper)
        y, zed = [_div(intervals[c][side], z, upper) for c in (1, 2)]
        if y == 0 or zed == 0:
            norm = max(y, zed)
        else:
            scale = max(y, zed)
            a, b = _div(y, scale, upper), _div(zed, scale, upper)
            norm = _mul(scale, _out(math.sqrt(_add(_mul(a, a, upper), _mul(b, b, upper), upper)), upper), upper)
        bounds.append(_add(axial, norm, upper))
    _require(bounds[0] <= value <= bounds[1], "ENDPOINT_ENCLOSURE")
    return value, bounds


def _maximum_link(case: Mapping[str, Any], ex: Mapping[str, Any], receipt_case: Mapping[str, Any]) -> None:
    plan = receipt_case["source"]
    _require(ex["load_case_id"] == case["load_case_id"] and ex["basis"] == MAX_BASIS and ex["coefficient_basis"] == "retained_section_functionals_binary64", "MAXIMUM_METHOD")
    _require(ex["source_identity_sha256"] == plan["retained_identity_sha256"], "MAXIMUM_SOURCE_IDENTITY")
    section = next((s for s in case["pipe_sections"] if s["pipe_id"] == ex["pipe_id"]), None)
    endpoints = next((s for s in plan["endpoint_sections"] if s["pipe_id"] == ex["pipe_id"]), None)
    _require(section is not None and endpoints is not None and _same(ex["area_m2"], section["As_m2"]) and _same(ex["section_modulus_m3"], section["Z_m3"]), "MAXIMUM_SECTION")
    for index, endpoint in enumerate(ex["endpoints"]):
        _require({k: endpoint[k] for k in ("station_fraction", "functional_indices", "functional_ids", "actions")} == endpoints["endpoints"][index], "MAXIMUM_ENDPOINT_SOURCE")
        _require(endpoint["station_fraction"] == index, "MAXIMUM_ENDPOINT_ORDER")
        for i, fid, action in zip(endpoint["functional_indices"], endpoint["functional_ids"], endpoint["actions"]):
            cid = case["load_case_id"]
            _require(0 <= i < plan["functional_count"] and fid == f"source-functional:{len(cid.encode())}:{cid}:{i}", "MAXIMUM_FUNCTIONAL_ID")
            _require(action["interval"][0] <= action["value"] <= action["interval"][1], "MAXIMUM_ACTION_INTERVAL")
    indices = [i for endpoint in ex["endpoints"] for i in endpoint["functional_indices"]]
    _require(len(indices) == len(set(indices)) == 6, "MAXIMUM_FUNCTIONAL_BIJECTION")


def validate_maximum(source: Mapping[str, Any], case: Mapping[str, Any], ex: Mapping[str, Any], rows: Mapping[str, Any]) -> None:
    receipt_case = _case(source, case["load_case_id"])
    _maximum_link(case, ex, receipt_case)
    row = rows.get(ex["result_id"], {})
    _require(row.get("kind") == "pipe_elastic_normal_stress_maximum_v2" and row.get("entity_ref") == ex["pipe_id"] and row.get("basis_ref") == receipt_case["basis_ref"] and row.get("unit") == "Pa" and _same(row.get("value"), ex["value_pa"]), "MAXIMUM_ROW")
    ends = ex["endpoints"]
    _require(ends[0]["actions"][0]["interval"][0] <= ends[1]["actions"][0]["interval"][1] and ends[1]["actions"][0]["interval"][0] <= ends[0]["actions"][0]["interval"][1], "CONSTANT_AXIAL_BOUND")
    for endpoint in ends:
        value, interval = _endpoint(ex["area_m2"], ex["section_modulus_m3"], endpoint["actions"])
        _require(_same(value, endpoint["value_pa"]) and all(_same(a, b) for a, b in zip(interval, endpoint["interval_pa"])), "MAXIMUM_ENDPOINT_RECIPE")
    value = max(e["value_pa"] for e in ends)
    lo, hi = [max(e["interval_pa"][i] for e in ends) for i in (0, 1)]
    absolute, relative = _bounds(value, lo, hi)
    _require(all(_same(ex[k], v) for k, v in [("value_pa", value), ("value_lower_pa", lo), ("value_upper_pa", hi), ("absolute_error_bound_pa", absolute), ("relative_error_bound", relative)]) and ex["relative_limit"] == 1e-9, "MAXIMUM_BOUNDS")
    _require(ex["station_fraction"] == (1 if ends[1]["value_pa"] > ends[0]["value_pa"] else 0), "MAXIMUM_WITNESS")
    singleton = lambda a: a["interval"][0] == a["interval"][1]
    equal = lambda c, absolute: singleton(ends[0]["actions"][c]) and singleton(ends[1]["actions"][c]) and (abs(ends[0]["actions"][c]["value"]) == abs(ends[1]["actions"][c]["value"]) if absolute else ends[0]["actions"][c]["value"] == ends[1]["actions"][c]["value"])
    if all(equal(c, False) for c in (1, 2)):
        locations = {"kind": "whole_span_constant"}
    elif ends[0]["interval_pa"][0] > ends[1]["interval_pa"][1]:
        locations = {"kind": "strict_endpoint", "endpoint": "i"}
    elif ends[1]["interval_pa"][0] > ends[0]["interval_pa"][1]:
        locations = {"kind": "strict_endpoint", "endpoint": "j"}
    else:
        locations = {"kind": "endpoint_candidates", "exact_tie_proven": all(equal(c, True) for c in (1, 2)), "interior_equal_possible": all(ends[0]["actions"][c]["interval"][0] <= ends[1]["actions"][c]["interval"][1] and ends[1]["actions"][c]["interval"][0] <= ends[0]["actions"][c]["interval"][1] for c in (1, 2))}
    _require(ex["locations"] == locations, "MAXIMUM_LOCATION_CLAIM")


def is_composite_recipe(recipe: str) -> bool:
    return recipe in {MAX_BASIS, "support_force_norm_scaled_checked_v1", "support_moment_norm_scaled_checked_v1", "retained_source_straight_stress_v1"}


def validate_derived(source: Mapping[str, Any], case: Mapping[str, Any], treatment: Mapping[str, Any], row: Mapping[str, Any], inputs: list[Mapping[str, Any]]) -> None:
    recipe = treatment["recipe_id"]
    if recipe == MAX_BASIS:
        physical_case = next(c for c in source["contract_evidence"]["exact_cases"] if c["load_case_id"] == case["basis_ref"]["ref_id"])
        ex = next((e for e in physical_case["pipe_stress_extrema"] if e["result_id"] == row["id"]), None)
        _require(ex is not None and inputs == [], "MAXIMUM_DERIVED_BINDING")
        validate_maximum(source, physical_case, ex, {r["id"]: r for r in source["results"]})
        return
    if recipe == "retained_source_straight_stress_v1":
        validate_stress(source, case, row, inputs)
        return
    checks = [c for c in case["derived_checks"] if c["result_id"] == row["id"]]
    _require(len(checks) == 1, "NORM_CHECK_ID")
    check = checks[0]
    force = recipe == "support_force_norm_scaled_checked_v1"
    components = ["Fx", "Fy", "Fz"] if force else ["Mx", "My", "Mz"]
    _require(check["recipe_id"] == recipe and row["kind"] == ("support_reaction_force_magnitude_v2" if force else "support_reaction_moment_magnitude_v2") and check["support_id"] == row["entity_ref"] and row["unit"] == ("N" if force else "N*m"), "NORM_SIGNATURE")
    _require(len(inputs) == 3 and [r["metadata"]["component"] for r in inputs] == components and all(r["kind"] == "support_reaction_component_v2" and r["entity_ref"] == row["entity_ref"] and r["unit"] == row["unit"] for r in inputs), "NORM_INPUTS")
    for index, input_row in enumerate(inputs):
        projections = [p for p in case["projections"] if p["result_id"] == input_row["id"]]
        _require(len(projections) == 1, "NORM_PROJECTION")
        projection = projections[0]
        _require(check["functional_ids"][index] == projection["functional_id"] and _same(check["values"][index], projection["value"]) and check["intervals"][index] == projection["interval"], "NORM_PROJECTION_BINDING")
        cid = case["basis_ref"]["ref_id"]
        _require(check["functional_ids"][index] == f'source-functional:{len(cid.encode())}:{cid}:{check["functional_indices"][index]}', "NORM_FUNCTIONAL_INDEX")
    intervals = [_abs_interval(p) for p in check["intervals"]]
    lo, hi = [_norm_bound([p[i] for p in intervals], bool(i)) for i in (0, 1)]
    value = _scaled_norm(check["values"])
    absolute, relative = _bounds(value, lo, hi)
    _require(_same(row["value"], value) and _same(check["value"], value) and all(_same(a,b) for a,b in zip(check["interval"], [lo,hi])) and _same(check["absolute_error_bound"],absolute) and _same(check["relative_error_bound"],relative) and check["relative_limit"] == 1e-9, "NORM_RECIPE")


def _section_functionals(case: Mapping[str, Any]) -> None:
    plan = case["source"]
    _require([s["pipe_id"] for s in plan["section_functionals"]] == plan["member_ids"], "SECTION_FUNCTIONAL_MEMBERS")
    ids: set[int] = set()
    cid = case["basis_ref"]["ref_id"]
    for member, endpoint in zip(plan["section_functionals"], plan["endpoint_sections"]):
        _require([s["station_fraction"] for s in member["stations"]] == [0,.25,.5,.75,1], "SECTION_FUNCTIONAL_STATIONS")
        for station in member["stations"]:
            for i, fid, action in zip(station["functional_indices"],station["functional_ids"],station["actions"]):
                _require(i not in ids and 0 <= i < plan["functional_count"] and fid == f"source-functional:{len(cid.encode())}:{cid}:{i}", "SECTION_FUNCTIONAL_ID")
                ids.add(i)
                lo, hi = action["interval"]
                _require(lo <= action["value"] <= hi, "SECTION_FUNCTIONAL_INTERVAL")
                if action["value"] == 0:
                    _require(lo == hi == 0, "SECTION_ZERO_CERTIFICATE")
                else:
                    _require(((lo > 0 and hi > 0) or (lo < 0 and hi < 0)) and max(abs(action["value"]-lo),abs(hi-action["value"]))/min(abs(lo),abs(hi)) <= blocks.STRESS_INPUT_RELATIVE_LIMIT, "SECTION_FUNCTIONAL_CRITERION")
        for i, station in enumerate((member["stations"][0],member["stations"][4])):
            _require(endpoint["endpoints"][i] == {"station_fraction":station["station_fraction"],"functional_indices":[station["functional_indices"][c] for c in (0,4,5)],"functional_ids":[station["functional_ids"][c] for c in (0,4,5)],"actions":[station["actions"][c] for c in (0,4,5)]}, "ENDPOINT_SECTION_LINK")


def validate_stress(source: Mapping[str, Any], case: Mapping[str, Any], row: Mapping[str, Any], inputs: list[Mapping[str, Any]]) -> None:
    checks = [c for c in case["section_stress_checks"] if c["result_id"] == row["id"]]
    _require(len(checks) == 1 and not inputs, "STRESS_CHECK_ID")
    check = checks[0]
    kind_components = {"element_local_axial_normal_stress": (0,"axial_normal_stress"), "element_local_bending_normal_stress_y":(4,"bending_normal_stress_y"), "element_local_bending_normal_stress_z":(5,"bending_normal_stress_z"), "element_local_torsional_shear_stress":(3,"torsional_shear_stress")}
    _require(row["kind"] in kind_components, "STRESS_KIND")
    component, label = kind_components[row["kind"]]
    location = row["metadata"]["location"]
    locations = ["end_i","quarter_1","midspan","quarter_3","end_j"]
    _require(location in locations and row["metadata"]["component"] == label and row["unit"] == "MPa" and check["pipe_id"] == row["entity_ref"] and check["location"] == location and check["component"] == label and check["recipe_id"] == "retained_source_straight_stress_v1", "STRESS_SIGNATURE")
    member = next(m for m in case["source"]["section_functionals"] if m["pipe_id"] == row["entity_ref"])
    station = member["stations"][locations.index(location)]
    _require(check["functional_index"] == station["functional_indices"][component] and check["functional_id"] == station["functional_ids"][component] and check["action"] == station["actions"][component], "STRESS_FUNCTIONAL_LINK")
    exact = next(c for c in source["contract_evidence"]["exact_cases"] if c["load_case_id"] == case["basis_ref"]["ref_id"])
    section = next(s for s in exact["pipe_sections"] if s["pipe_id"] == row["entity_ref"])
    params = check["parameters"]
    _require(params == {"area_m2":section["As_m2"],"section_modulus_m3":section["Z_m3"],"torsion_radius_m":section["ro_m"],"torsion_constant_m4":section["J_m4"],"pa_per_mpa":1e6}, "STRESS_SECTION_PARAMETERS")
    action = check["action"]["value"]
    if component == 3 and action != 0:
        intermediate = action * params["torsion_radius_m"]
        _require(math.isfinite(intermediate) and abs(intermediate) >= sys.float_info.min, "STRESS_TORSION_INTERMEDIATE_RANGE")
    pa = action / params["area_m2"] if component == 0 else action * params["torsion_radius_m"] / params["torsion_constant_m4"] if component == 3 else action / params["section_modulus_m3"]
    value = pa / 1e6
    _require(blocks._normal_or_exact_zero(pa, action == 0) and blocks._normal_or_exact_zero(value, action == 0) and _same(row["value"],value), "STRESS_RECIPE_VALUE")


def validate_source_case(source: Mapping[str, Any], case: Mapping[str, Any], invocation: Mapping[str, Any] | None) -> None:
    _require(case["ordinary_attempt"]["outcome"] in {"sensitive", "rejected"}, "SOURCE_FALLBACK_TRIGGER")
    physical_case = next(c for c in source["contract_evidence"]["exact_cases"] if c["load_case_id"] == case["basis_ref"]["ref_id"])
    _require(not physical_case["pressure_rhs_assembly"]["groups"] and all(v == 0 for k in ("assembled_pressure_rhs_global", "rounded_cap_rhs_global", "rounded_poisson_rhs_global") for v in physical_case["pressure_rhs_assembly"][k]), "SOURCE_PRESSURE_RHS")
    _section_functionals(case)
    checks = case["derived_checks"]
    expected = {r["result_id"] for r in case["rows"] if r["recipe_id"] in {"support_force_norm_scaled_checked_v1", "support_moment_norm_scaled_checked_v1"}}
    _require(len(checks) == len(expected) and {c["result_id"] for c in checks} == expected and len(checks) == 2 * len(case["source"]["support_ids"]), "DERIVED_CHECK_COVERAGE")
    stresses = case["section_stress_checks"]
    expected_stresses = {r["result_id"] for r in case["rows"] if r["recipe_id"] == "retained_source_straight_stress_v1"}
    _require(len(stresses) == len(expected_stresses) == 20 * len(case["source"]["member_ids"]) and {c["result_id"] for c in stresses} == expected_stresses, "STRESS_CHECK_COVERAGE")
    if invocation is not None:
        model = invocation["request"]["model"]
        _require(model.get("schema_version") == "0.3.0" and model.get("pressure_contract") == {"version":"2.0.0","mode":PROFILE}, "ACTUAL_EXACT_PROFILE")
        actual = next(c for c in model["load_cases"] if c["id"] == case["basis_ref"]["ref_id"])
        _require(actual.get("pressure_regions") == [] and "pressure_regions" in actual, "ACTUAL_EMPTY_PRESSURE_INVENTORY")
        _require(not model.get("components") and not model.get("combinations") and actual.get("equivalent_static") is None, "ACTUAL_SOURCE_FAMILY")
        _require(all(load.get("target", {}).get("type") == "node" and load.get("category") != "thermal" for load in actual.get("primitive_loads", [])), "ACTUAL_NODAL_LOADS")
        _require(all(s.get("nonlinear") is None and s.get("family") != "constant_effort_support" and (s.get("hanger") or {}).get("constant_load") is None for s in model["supports"]), "ACTUAL_SUPPORT_FAMILY")


def _canonical_inputs(invocation: Mapping[str, Any]):
    """Batch original quantities through the same Rust unit authority as producer."""
    from core.units.adapter import convert_quantities_to_canonical
    model = invocation["request"]["model"]
    materials = invocation["request"].get("materials") or model.get("materials", [])
    queries = []
    identities = {}
    def add(quantity, dimension):
        if quantity is None: return
        key = (id(quantity), dimension)
        if key not in identities:
            token = str(len(queries))
            identities[key] = token
            queries.append({"id":token,"value":quantity["value"],"unit":quantity["unit"],"dimension":dimension})
    for material in materials:
        for record in [material, *material.get("temperature_points", [])]:
            for field,dimension in [("elastic_modulus","stress"),("shear_modulus","stress"),("thermal_expansion_coefficient","thermal_expansion_coefficient"),("temperature","temperature")]:
                add(record.get(field),dimension)
    for pipe in model["pipe_segments"]:
        for key in ["outside_diameter","wall_thickness","mill_tolerance"]:
            add(pipe["section"].get(key),"length")
    for case in model["load_cases"]:
        add(case.get("modulus_basis_temperature"),"temperature")
    normalized = {r["id"]:r["value"] for r in convert_quantities_to_canonical(queries)}
    return lambda quantity,dimension: normalized[identities[(id(quantity),dimension)]]


def _poisson(quantity):
    _require(quantity["unit"] == "1", "ACTUAL_POISSON_UNIT")
    nu = quantity["value"]
    _require(physical._number(nu) and -1 < nu < .5, "ACTUAL_POISSON_RANGE")
    return nu


def _checked_material_pair(e_pa, nu, stage):
    """Mirror pressure_exact::IsotropicENu::new's Scaled representability domain.

    The denominator uses the same bounded binary64 operations as Scaled add/mul:
    -1 < nu < .5 makes 1+nu and 2*(1+nu) positive normal. The E division must
    stay in normalized mantissa/exponent form until the producer's to_f64 rules.
    frexp also normalizes subnormal E exactly; no full-range direct quotient.
    """
    _require(physical._number(e_pa) and e_pa > 0 and physical._number(nu) and -1 < nu < .5, f"ACTUAL_PAIR_INPUT_RANGE:{stage}")
    denominator = 2.0 * (1.0 + nu)
    em, ee = math.frexp(e_pa)
    dm, de = math.frexp(denominator)
    gm, ge = math.frexp(em / dm)  # Scaled::div then from_parts normalization
    exponent = ee - de + ge
    if exponent > 1024:
        g_pa = math.inf
    elif exponent == 1024:
        g_pa = (gm * math.ldexp(1.0, 1023)) * 2.0
    elif exponent < -1074:
        g_pa = 0.0
    else:
        g_pa = gm * math.ldexp(1.0, exponent)
    _require(math.isfinite(g_pa) and g_pa > 0, f"ACTUAL_DERIVED_SHEAR_RANGE:{stage}")
    return g_pa


def _selected_material(authored, actual, canonical):
    def normalized_record(record):
        result = dict(record)
        for key,dimension in [("elastic_modulus","stress"),("shear_modulus","stress"),("thermal_expansion_coefficient","thermal_expansion_coefficient"),("temperature","temperature")]:
            if record.get(key) is not None:
                result[key] = {"value":canonical(record[key],dimension)}
        return result
    base = normalized_record(authored)
    # Producer resolve_base precedes every exact case, including selected points.
    _checked_material_pair(base["elastic_modulus"]["value"], _poisson(base["poisson_ratio"]), "base")
    points = [normalized_record(point) for point in authored.get("temperature_points", [])]
    _require(actual.get("modulus_basis_ref") is None or actual.get("modulus_basis_temperature") is None, "ACTUAL_MATERIAL_SELECTION_CONFLICT")
    if actual.get("modulus_basis_ref") is not None:
        selected = next(p for p in points if p["id"] == actual["modulus_basis_ref"])
    elif actual.get("modulus_basis_temperature") is None:
        selected = base
    else:
        t = canonical(actual["modulus_basis_temperature"],"temperature")
        points = sorted((p for p in points if p.get("temperature") is not None),key=lambda p:p["temperature"]["value"])
        temperatures = [p["temperature"]["value"] for p in points]
        _require(len(temperatures) == len(set(temperatures)), "ACTUAL_MATERIAL_DUPLICATE_TEMPERATURE")
        # Exact pressure profile admits strict interior brackets, never endpoints.
        a,b = next((a,b) for a,b in zip(points,points[1:]) if a["temperature"]["value"] < t < b["temperature"]["value"])
        f = (t-a["temperature"]["value"])/(b["temperature"]["value"]-a["temperature"]["value"])
        interpolate = lambda a,b: (1-f)*a+f*b
        e0,e1 = a["elastic_modulus"]["value"],b["elastic_modulus"]["value"]
        nu0,nu1 = _poisson(a["poisson_ratio"]),_poisson(b["poisson_ratio"])
        _checked_material_pair(e0,nu0,"lower_bracket")
        _checked_material_pair(e1,nu1,"upper_bracket")
        alpha = None if a.get("thermal_expansion_coefficient") is None or b.get("thermal_expansion_coefficient") is None else {"value":interpolate(a["thermal_expansion_coefficient"]["value"],b["thermal_expansion_coefficient"]["value"])}
        selected = {"elastic_modulus":{"value":interpolate(e0,e1)},"poisson_ratio":{"unit":"1","value":interpolate(nu0,nu1)},"thermal_expansion_coefficient":alpha}
    e,nu = selected["elastic_modulus"]["value"],_poisson(selected["poisson_ratio"])
    _checked_material_pair(e,nu,"selected")
    alpha = (selected.get("thermal_expansion_coefficient") or {}).get("value")
    _require(alpha is None or physical._number(alpha),"ACTUAL_ALPHA_RANGE")
    return e,nu,alpha


def _actual_materials(invocation, actual, exact, canonical):
    model = invocation["request"]["model"]
    materials = {m["id"]:m for m in (invocation["request"].get("materials") or model.get("materials", []))}
    _require(len(model["pipe_segments"]) == len(exact["pipe_sections"]) == len(exact["pipe_materials"]),"ACTUAL_MATERIAL_COVERAGE")
    for pipe,section,material in zip(model["pipe_segments"],exact["pipe_sections"],exact["pipe_materials"]):
        _require(pipe["id"] == section["pipe_id"] == material["pipe_id"] and pipe["material"] == material["material_id"],"ACTUAL_MATERIAL_MEMBER")
        authored_section = pipe["section"]
        tolerance = canonical(authored_section["mill_tolerance"],"length") if authored_section.get("mill_tolerance") is not None else 0.0
        _require(_same(canonical(authored_section["outside_diameter"],"length"),section["outside_diameter_m"]) and _same(canonical(authored_section["wall_thickness"],"length")-tolerance,section["effective_wall_thickness_m"]),"ACTUAL_SOURCE_GEOMETRY")
        authored = materials[pipe["material"]]
        _require(authored.get("constitutive_basis") == material["constitutive_basis"],"ACTUAL_CONSTITUTIVE_BASIS")
        e,nu,alpha = _selected_material(authored,actual,canonical)
        thermal = any(load.get("category") == "thermal" and load.get("dimension") in {"temperature_change","temperature_interval"} and load.get("target",{}).get("type") == "element" and load["target"].get("pipe") == pipe["id"] for load in actual.get("primitive_loads",[]))
        _require(_same(e,material["E_pa"]) and _same(nu,material["nu"]) and material["thermal_consumed"] is thermal,"ACTUAL_SELECTED_MATERIAL")
        _require((alpha is not None and _same(alpha,material["alpha_per_kelvin"])) if thermal else material["alpha_per_kelvin"] is None,"ACTUAL_SELECTED_ALPHA")


def validate_physics_source(source: Mapping[str, Any], actual_invocation: Mapping[str, Any] | None = None) -> bool:
    try:
        validate_transport_metadata(source)
        context = sys.modules[__name__]
        physical._validate_physics_evidence(source, context=context)
        eligible = blocks._validate_source_blocks(source, actual_invocation, context=context)
        if actual_invocation is not None:
            canonical = _canonical_inputs(actual_invocation)
            for exact in source["contract_evidence"]["exact_cases"]:
                actual = next(c for c in actual_invocation["request"]["model"]["load_cases"] if c["id"] == exact["load_case_id"])
                _actual_materials(actual_invocation,actual,exact,canonical)
        return eligible
    except (KeyError, TypeError, IndexError, AttributeError, OverflowError, StopIteration) as error:
        raise ValueError("PHYSICS_SOURCE_MALFORMED_VALUE") from error


def validate_transport_metadata(source: Mapping[str, Any]) -> None:
    """Validate transported statements only, with uniform malformed-input errors."""
    try:
        _validate_transport_metadata(source)
    except (KeyError, TypeError, IndexError, AttributeError, OverflowError, StopIteration) as error:
        raise ValueError("PHYSICS_SOURCE_MALFORMED_TRANSPORT") from error
