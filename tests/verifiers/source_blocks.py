"""Shared source-block receipt statement/binding validation for the composite identities, never arithmetic proof.

B4/D1 (b) retired the non-composite source-blocks-1 identity, and B7b the v2 composites (physics-source-1,
load-reference-source-1); the current composite, pressure-source-1, calls `_validate_source_blocks` with its
contract context.

Test verifier (b7-plan R2): the product reader is Rust's (`core/reporting/result_export/src/source_blocks.rs`), which
product Python reaches only through `openpipestress-runner read`. Tests only: no module outside `tests/` imports
this one, and it imports nothing from product code (`tests/test_verifier_independence.py`).

Positive numerical standing additionally needs independently captured invocation
JSON and the caller's existing authentic source/model/input/build binding.
"""
from __future__ import annotations
from collections.abc import Mapping
import math
import re
import struct
import sys
from typing import Any
from .canonical_json import canonical_sha256 as canonical_sha256_checked_v1
from .preview_physics_evidence import correctly_rounded_norm
from . import section_basis as sb

# Fixed recipe arithmetic bound, additional to the affine input bound.
NORM_ARITHMETIC_BOUND = 64.0 * sys.float_info.epsilon
NORM_INPUT_RELATIVE_LIMIT = (1.0e-9 - NORM_ARITHMETIC_BOUND) / (1.0 + NORM_ARITHMETIC_BOUND)
# Conservative current-carrier allowance for the composites' straight stress.
STRESS_ARITHMETIC_BOUND = 128.0 * sys.float_info.epsilon
STRESS_INPUT_RELATIVE_LIMIT = (1.0e-9 - STRESS_ARITHMETIC_BOUND) / (1.0 + STRESS_ARITHMETIC_BOUND)
EXACT = "retained_source_blocks_exact_v1"
MODES = {"dense_scrutiny": "ordinary_dense_structural_v1", "sparse_interactive": "ordinary_sparse_structural_v1"}
COMPONENTS = ("Fx", "Fy", "Fz", "Mx", "My", "Mz")
# T4-U4b (DEL-04-07 Design u4b-u5b-implementation.md §4.3): the treatment of the section normal-stress
# extremes and transverse-shear maxima a selected source case publishes beside its qualified rows.
NOT_QUALIFIED = "derived_not_qualified"
NOT_QUALIFIED_KINDS = ("element_local_normal_stress_extreme", "element_local_transverse_shear_stress_maximum")
NOT_QUALIFIED_RECIPES = {NOT_QUALIFIED_KINDS[0]: "section_extremes_from_published_stress_rows_v1",
                         NOT_QUALIFIED_KINDS[1]: "transverse_shear_from_published_shear_rows_v1"}


def _require(condition: Any, code: str) -> None:
    if not condition:
        raise ValueError("SOURCE_BLOCKS_" + code)


def _shape(value: Any, schema: Mapping[str, Any], root: Mapping[str, Any], *, resources: Mapping[str, Mapping[str, Any]] | None = None) -> bool:
    """Only the closed, checked-in receipt schema vocabulary is interpreted."""
    if "$ref" in schema:
        reference = schema["$ref"]
        if not isinstance(reference, str): return False
        resource, mark, fragment = reference.partition("#")
        target_root = root if not resource else (resources or {}).get(resource)
        if not isinstance(target_root, Mapping) or (mark and fragment and not fragment.startswith("/")): return False
        target: Any = target_root
        for token in fragment.removeprefix("/").split("/") if fragment else []:
            if re.search(r"~(?![01])", token): return False
            key = token.replace("~1", "/").replace("~0", "~")
            if isinstance(target, Mapping) and key in target: target = target[key]
            elif isinstance(target, list) and key.isascii() and key.isdigit() and (key == "0" or not key.startswith("0")) and int(key) < len(target): target = target[int(key)]
            else: return False
        return isinstance(target, Mapping) and _shape(value, target, target_root, resources=resources)
    if "anyOf" in schema and not any(_shape(value, branch, root, resources=resources) for branch in schema["anyOf"]):
        return False
    if "oneOf" in schema and sum(_shape(value, branch, root, resources=resources) for branch in schema["oneOf"]) != 1:
        return False
    if "allOf" in schema and not all(_shape(value, branch, root, resources=resources) for branch in schema["allOf"]):
        return False
    if "const" in schema and (value != schema["const"] or (type(value) is bool) != (type(schema["const"]) is bool)):
        return False
    if "enum" in schema and not any(value == allowed and (type(value) is bool) == (type(allowed) is bool) for allowed in schema["enum"]):
        return False
    kind = schema.get("type", "object" if "properties" in schema else "array" if "items" in schema else None)
    if isinstance(kind, list): return any(_shape(value, {**schema, "type": item}, root, resources=resources) for item in kind)
    if kind == "null": return value is None
    if kind == "boolean": return type(value) is bool
    if kind == "object":
        if type(value) is not dict: return False
        properties = schema.get("properties", {})
        if not set(schema.get("required", [])) <= set(value): return False
        if schema.get("additionalProperties") is False and not set(value) <= set(properties): return False
        return all(_shape(item, properties[key], root, resources=resources) for key, item in value.items() if key in properties)
    if kind == "array":
        return type(value) is list and schema.get("minItems", 0) <= len(value) <= schema.get("maxItems", 16384) and all(_shape(item, schema["items"], root, resources=resources) for item in value)
    if kind == "string":
        return type(value) is str and len(value) >= schema.get("minLength", 0) and ("pattern" not in schema or re.fullmatch(schema["pattern"], value) is not None)
    if kind in {"number", "integer"}:
        return type(value) in {int, float} and math.isfinite(value) and (kind != "integer" or float(value).is_integer()) and schema.get("minimum", -math.inf) <= value <= schema.get("maximum", math.inf) and value > schema.get("exclusiveMinimum", -math.inf)
    return True


def domain_hash(domain: str, payload: Any) -> str:
    """Fixed checked authority; no injectable hash callback on the admission path."""
    return canonical_sha256_checked_v1({"domain": domain, "payload": payload})


def _unique(items: list[Any], code: str) -> None:
    _require(len(set(items)) == len(items), code)


def _signature(row: Mapping[str, Any], context: Any) -> Mapping[str, Any] | None:
    md = row.get("metadata") or {}
    found = [s for s in context.TABLE["rows"] if s["kind"] == row.get("kind") and s["unit"] == row.get("unit") and (s["component"] is None or s["component"] == md.get("component")) and ("source_basis" not in s or s["source_basis"] == md.get("basis"))]
    return found[0] if len(found) == 1 else None


def _bits(number: Any) -> str:
    _require(type(number) in {int, float} and math.isfinite(number), "VALUE_NONFINITE")
    return struct.pack(">d", float(number)).hex()


def _ordinary(case: Mapping[str, Any], quality: Mapping[str, Any], diagnostics: dict[str, Any], evidence: set[str]) -> bool:
    ordinary = case["ordinary_attempt"]
    _require(ordinary["requested_mode"] == case["requested_mode"], "ORDINARY_MODE")
    report, failure, outcome = ordinary["structural_report_diagnostic_ref"], ordinary["failure"], ordinary["outcome"]
    _require(report is None or report in diagnostics, "ORDINARY_REPORT_REF")
    _require(failure is None or failure["diagnostic_ref"] in diagnostics, "ORDINARY_FAILURE_REF")
    if outcome == "not_attempted":
        _require(report is None and failure is None and quality["solve_quality"] == "not_assessed", "ORDINARY_NOT_ATTEMPTED")
    elif outcome in {"checks_passed", "sensitive"}:
        _require(report is not None and failure is None and quality["solve_quality"] == outcome and report in quality["evidence_refs"], "ORDINARY_REPORT")
        _require(diagnostics[report].get("code") == ("NUMERICAL_INTEGRITY_CHECKS_PASSED" if outcome == "checks_passed" else "NUMERICAL_INTEGRITY_SENSITIVE"), "ORDINARY_REPORT_KIND")
    else:
        _require(failure is not None and quality["solve_quality"] in {"unresolved", "failed"} and failure["diagnostic_ref"] in quality["evidence_refs"], "ORDINARY_REJECTION")
    return quality["solve_quality"] == "checks_passed" and quality["structural_status"] == "passive_model_basis" and quality["model_matrix_fidelity"] == "represented_equations_retained" and quality["accuracy_evidence"] in {"not_claimed", "reference_verified"} and bool(quality["evidence_refs"]) and all(ref in evidence for ref in quality["evidence_refs"])


def _source_plan(plan: Mapping[str, Any], projections: list[Any], model: Mapping[str, Any] | None) -> None:
    n = plan["dof_count"]
    _require(0 < n <= 256 and plan["stiffness_term_count"] > 0 and plan["stiffness_term_count"] + plan["force_term_count"] <= 16384, "SOURCE_COUNTS")
    _require(len(projections) <= plan["functional_count"] <= 16384, "FUNCTIONAL_COUNT")
    partition = plan["free_dofs"] + plan["prescribed_dofs"]
    _require(sorted(partition) == list(range(n)), "SOURCE_PARTITION")
    flattened = [dof for block in plan["free_blocks"] for dof in block]
    _require(sorted(flattened) == sorted(plan["free_dofs"]), "BLOCK_PARTITION")
    _unique(plan["member_ids"], "MEMBER_IDENTITY")
    _unique(plan["support_ids"], "SUPPORT_IDENTITY")
    _require(bool(plan["member_ids"]), "MEMBER_COVERAGE")
    if model is not None:
        _require(n == len(model["nodes"]) * 6 and plan["member_ids"] == [p["id"] for p in model["pipe_segments"]] and plan["support_ids"] == [s["id"] for s in model["supports"]], "CURRENT_MODEL_SOURCE_COVERAGE")


def _projection(projection: Mapping[str, Any], row: Mapping[str, Any], context: Any) -> None:
    value, (lo, hi) = projection["value"], projection["interval"]
    _require(projection["value_bits"] == _bits(value) == _bits(row["value"]) and projection["unit"] == row["unit"], "PROJECTION_VALUE_BINDING")
    _require(lo <= value <= hi and (value == 0 or lo * (1 if value > 0 else -1) > 0 and hi * (1 if value > 0 else -1) > 0), "PROJECTION_INTERVAL")
    basis = projection["basis"]
    if basis in {"exact_zero", "exact_identity"}:
        _require(lo == hi == value and projection["absolute_error_bound"] == projection["relative_error_bound"] == 0 and (value == 0 if basis == "exact_zero" else value != 0), "PROJECTION_EXACT_BASIS")
    else:
        _require(value != 0 and projection["absolute_error_bound"] >= max(abs(value-lo), abs(hi-value)), "PROJECTION_ENCLOSURE")
        _require(projection["relative_error_bound"] >= projection["absolute_error_bound"] / min(abs(lo), abs(hi)), "PROJECTION_RELATIVE_BOUND")
    _require(projection["relative_limit"] == 1e-9 and projection["relative_error_bound"] <= 1e-9, "PROJECTION_CRITERION")
    md = row.get("metadata") or {}
    quantity, kind = projection["quantity"], row["kind"]
    valid = {
        "nodal_translation": kind in {"global_nodal_displacement_x", "global_nodal_displacement_y", "global_nodal_displacement_z"} and row["unit"] == "mm" and md.get("coordinate_system") == "global" and md.get("location") == "node",
        "nodal_rotation": kind in {"global_nodal_rotation_x", "global_nodal_rotation_y", "global_nodal_rotation_z"} and row["unit"] == "rad" and md.get("coordinate_system") == "global" and md.get("location") == "node",
        "member_end_action": kind.startswith("element_local_") and row["unit"] in {"N", "N*m"} and md.get("location") in {"end_i", "end_j"} and md.get("coordinate_system") == "element_local",
        "member_station_action": kind.startswith("element_local_") and row["unit"] in {"N", "N*m"} and md.get("location") in {"quarter_1", "midspan", "quarter_3"} and md.get("coordinate_system") == "element_local",
        "support_action_component": kind == "support_reaction_component_v2" and md.get("component") in COMPONENTS,
        # No public constraint-reaction signature is allocated in this tranche.
        "constraint_reaction": False,
    }
    _require(valid[quantity] and _signature(row, context) is not None, "PROJECTION_TYPED_QUANTITY")


def _derived(recipe: str, row: Mapping[str, Any], inputs: list[Mapping[str, Any]], context: Any) -> None:
    # B4/D1 (b): the composites' only shared non-composite recipe is the translation
    # norm; source-blocks-1's support-norm, straight-stress, stress-summary and
    # section-property recipes went with its preview case rows and are unknown here.
    kind = row["kind"]
    _require(_signature(row, context) is not None, "DERIVED_SIGNATURE")
    _require(recipe == "translation_norm_scaled_v1", "DERIVED_RECIPE")
    _require(kind == "displacement_magnitude" and len(inputs) == 3 and [r["kind"] for r in inputs] == ["global_nodal_displacement_x", "global_nodal_displacement_y", "global_nodal_displacement_z"], "TRANSLATION_NORM_INPUTS")
    _require(all(r.get("entity_ref") == row.get("entity_ref") for r in inputs), "DERIVED_ENTITY")
    # E15: the declared recipe is one correctly rounded norm of the published
    # components (independent exact-integer oracle); a nonzero result is finite and normal.
    values = [float(item["value"]) for item in inputs]
    _require(all(math.isfinite(value) for value in values), "DERIVED_NORM_RANGE")
    expected = correctly_rounded_norm(values)
    _require(expected == 0.0 or (math.isfinite(expected) and expected >= sys.float_info.min), "DERIVED_NORM_RANGE")
    # This binds the declared recipe, not an affine projection error bound.
    _require(_bits(row["value"]) == _bits(expected), "DERIVED_NORM_VALUE")



def _normal_or_exact_zero(value: float, logical_zero: bool) -> bool:
    return math.isfinite(value) and (value == 0.0 if logical_zero else abs(value) >= sys.float_info.min)


def _failure_block_order(failure: Mapping[str, Any], case: Mapping[str, Any]) -> bool:
    """Rust `failure_block_order` (RF source-method extension §7): the current producer solves exact blocks of order
    up to 4, so an unsupported_block refusal records a larger order and no source; other codes record none. The
    receipt schema admits only qualified cases, so the shape refuses a stored failure record first at every entry."""
    return (failure["code"] == "unsupported_block" and failure["block_order"] is not None and failure["block_order"] > 4 and case["source"] is None) or (failure["code"] != "unsupported_block" and failure["block_order"] is None)


def _validate_source_blocks(source: Mapping[str, Any], actual_invocation: Mapping[str, Any] | None, *, context: Any) -> bool:
    """Return numerical eligibility; raise ValueError for malformed statements.

    A structurally consistent receipt with no independent invocation is always
    unqualified. Hashes bind records; they do not reconstruct private ratios.
    """
    _require(type(source) is dict and set(source) == ({"schema_version", "producer", "numerical_quality", "formulation_basis", "document_kind", "run_id", "model_ref", "status", "summary", "results", "diagnostics", "professional_boundary", "accepted_model_state_mutated", "source_block_recovery"} | {"contract_evidence"}), "RAW_FIELDS")
    _require(source["schema_version"] == "0.2.0" and source["producer"] == {"component_name":"open_pipe_stress_product_physics", "component_version":"0.2.0", "semantic_contract_id":context.CONTRACT_ID}, "PRODUCER")
    formulation = source["formulation_basis"]
    _require(type(formulation) is dict and set(formulation) == {"profile_id","limitations"} and formulation["profile_id"] == context.PROFILE and type(formulation["limitations"]) is list and bool(formulation["limitations"]) and all(type(v) is str and v for v in formulation["limitations"]), "FORMULATION")
    receipt = source["source_block_recovery"]
    context.validate_receipt_shape(receipt)
    body = receipt["body"]
    _require(receipt["receipt_sha256"] == domain_hash("source_blocks_receipt_v1", body), "RECEIPT_HASH")
    publication = {key:value for key,value in source.items() if key != "source_block_recovery"}
    _require(body["publication_sha256"] == domain_hash("source_blocks_publication_v1", publication), "PUBLICATION_HASH")
    model = None
    if actual_invocation is not None:
        _require(type(actual_invocation) is dict and set(actual_invocation) == {"request", "solver_mode"} and actual_invocation["solver_mode"] in MODES and type(actual_invocation["request"]) is dict, "ACTUAL_INVOCATION_SHAPE")
        _require(body["invocation"]["value"] == domain_hash("source_blocks_invocation_v1", actual_invocation), "INVOCATION_HASH")
        model = actual_invocation["request"].get("model")
        _require(type(model) is dict and model.get("project", {}).get("id") == source["model_ref"] and all(type(model.get(k)) is list for k in ("nodes", "pipe_segments", "supports", "load_cases")), "CURRENT_MODEL")
    _require(type(source["results"]) is list and type(source["diagnostics"]) is list and len(source["results"]) <= 16384 and len(source["diagnostics"]) <= 16384, "ROWS")
    if model is not None:
        for field in ("nodes","pipe_segments","supports","load_cases"):
            ids = [item.get("id") for item in model[field]]
            _require(all(type(i) is str and i for i in ids), "CURRENT_MODEL_IDS")
            _unique(ids, "CURRENT_MODEL_IDS")
    all_ids = [r.get("id") for r in source["results"] + source["diagnostics"] if type(r) is dict]
    _require(len(all_ids) == len(source["results"]) + len(source["diagnostics"]) and all(type(i) is str and i for i in all_ids), "EVIDENCE_IDS")
    _unique(all_ids, "EVIDENCE_IDS")
    raw = {r["id"]:r for r in source["results"]}; diagnostics = {r["id"]:r for r in source["diagnostics"]}
    for row in raw.values():
        _bits(row.get("value"))
        _require(all(type(row.get(k)) is str and row[k] for k in ("kind","unit","entity_ref")), "ROW_FIELDS")
        if row.get("metadata") is not None:
            _require(type(row["metadata"]) is dict and set(row["metadata"]) == {"component","coordinate_system","location","basis","sign_convention"} and all(type(v) is str and v for v in row["metadata"].values()), "ROW_METADATA")
    cases = body["cases"]
    case_ids = [c["basis_ref"]["ref_id"] for c in cases]; _unique(case_ids, "CASE_IDS")
    q = source["numerical_quality"]
    _require(type(q) is dict and set(q) == {"value_representation","publication_quantization","integrity_policy","status","cases"} and q["value_representation"] == "finite_binary64" and q["publication_quantization"] == "none" and q["integrity_policy"] == "M03-INTEGRITY-v1" and type(q["cases"]) is list and len(q["cases"]) == len(cases), "ORDINARY_QUALITY")
    quality_rank = {"checks_passed":0, "sensitive":1, "not_assessed":2, "unresolved":3, "failed":4}
    for quality in q["cases"]:
        _require(type(quality) is dict and set(quality) == {"basis_ref","structural_status","solve_quality","model_matrix_fidelity","accuracy_evidence","evidence_refs"} and quality["solve_quality"] in quality_rank and quality["structural_status"] in {"passive_model_basis","physical_mechanism_witnessed","negative_energy_witnessed","numerically_unresolved"} and quality["model_matrix_fidelity"] in {"represented_equations_retained","assembly_loss_detected","assembly_uncertainty","not_assessed"} and quality["accuracy_evidence"] in {"not_claimed","reference_verified","unresolved"} and type(quality["evidence_refs"]) is list and all(type(r) is str and r in all_ids for r in quality["evidence_refs"]), "ORDINARY_CASE")
    _require(q["status"] == max((c["solve_quality"] for c in q["cases"]), key=quality_rank.get, default="not_assessed"), "ORDINARY_AGGREGATE")
    if model is not None:
        _require(case_ids == [c["id"] for c in model["load_cases"]], "REQUESTED_CASE_COVERAGE")
    covered: set[str] = set(); qualified = 0; usable = bool(cases); invocation_charged = 0
    for index, case in enumerate(cases):
        basis = case["basis_ref"]
        _require(case["ordinary_attempt"]["quality_case_index"] == index and q["cases"][index]["basis_ref"] == basis, "QUALITY_CASE_BINDING")
        _require(actual_invocation is None or case["requested_mode"] == actual_invocation["solver_mode"], "REQUESTED_MODE")
        ordinary_ok = _ordinary(case, q["cases"][index], diagnostics, set(all_ids))
        work = case["work"]; rejected = work["rejected_reservation"]
        invocation_charged += work["charged"] + work["reserved_unobserved_failure"]
        _require(invocation_charged <= 64_000_000, "INVOCATION_WORK_LIMIT")
        _require(work["charged"] + work["reserved_unobserved_failure"] <= work["limit"] <= 8_000_000 and ((rejected["kind"] == "overflow" and rejected["amount"] is None) or (rejected["kind"] == "finite" and rejected["amount"] is not None)), "WORK_LEDGER")
        success = case["outcome"] == "qualified"; method = case["selected_method"]
        if success:
            qualified += 1
            _require(case["failure"] is None and method is not None and rejected == {"kind":"finite","amount":0} and work["reserved_unobserved_failure"] == 0, "QUALIFIED_OUTCOME")
            if method != EXACT:
                _require(method == MODES[case["requested_mode"]] and ordinary_ok and case["source"] is None and not case["projections"] and not case["supports"], "ORDINARY_SELECTION")
            else:
                _require(case["source"] is not None, "EXACT_SOURCE_REQUIRED")
        else:
            failure = case["failure"]
            _require(method is None and failure is not None and failure["diagnostic_ref"] in diagnostics, "FAILED_OUTCOME")
            _require(_failure_block_order(failure, case), "FAILURE_BLOCK_ORDER")
            _require((case["outcome"] == "unsupported") == (failure["code"] in {"unsupported_family","unsupported_block","unsupported_source_closure","unsupported_derived_quantity","support_attribution_ambiguous"}), "FAILURE_CATEGORY")
        if case["source"] is not None:
            _source_plan(case["source"], case["projections"], model)
            _unique([case["source"]["normalized_source_sha256"],case["source"]["functional_plan_sha256"],body["invocation"]["value"],body["publication_sha256"],receipt["receipt_sha256"]], "COMMITMENT_DOMAIN_SEPARATION")
        projections = {p["projection_id"]:p for p in case["projections"]}
        _unique([p["projection_id"] for p in case["projections"]], "PROJECTION_IDS")
        _unique([p["functional_id"] for p in case["projections"]], "FUNCTIONAL_IDS")
        _unique([p["result_id"] for p in case["projections"]], "PROJECTION_RESULTS")
        treatment = {r["result_id"]:r for r in case["rows"]}; _unique([r["result_id"] for r in case["rows"]], "ROW_IDS")
        if any(row["recipe_id"] == "retained_source_straight_stress_v1" for row in case["rows"]):
            _require(bool(case["projections"]) and max(p["relative_error_bound"] for p in case["projections"]) <= STRESS_INPUT_RELATIVE_LIMIT, "STRESS_TOTAL_RELATIVE_BOUND")
        expected = {i for i,r in raw.items() if r.get("basis_ref") == basis}
        _require(set(treatment) == expected and not covered & expected, "CASE_ROW_COVERAGE"); covered |= expected
        used_projections: set[str] = set()
        for result_id, treatment_row in treatment.items():
            row = raw[result_id]; signature = _signature(row, context); inputs = treatment_row["input_result_ids"]
            _unique(inputs, "ROW_INPUTS")
            _require(all(i in treatment and i != result_id for i in inputs), "SAME_CASE_INPUTS")
            token = treatment_row["treatment"]
            if success and method == EXACT and row.get("kind") in NOT_QUALIFIED_KINDS:
                _require(token == NOT_QUALIFIED, "NOT_QUALIFIED_TREATMENT")
            if token == "qualified_projection":
                projection = projections.get(treatment_row["projection_id"])
                _require(method == EXACT and success and projection is not None and projection["result_id"] == result_id and treatment_row["recipe_id"] is None and not inputs, "PROJECTION_ROW")
                _projection(projection, row, context); used_projections.add(projection["projection_id"])
            elif token == "checked_derived":
                _require(method == EXACT and success and treatment_row["projection_id"] is None and treatment_row["recipe_id"] is not None, "DERIVED_ROW")
                if context.is_composite_recipe(treatment_row["recipe_id"]):
                    context.validate_derived(source, case, treatment_row, row, [raw[i] for i in inputs])
                else:
                    _derived(treatment_row["recipe_id"], row, [raw[i] for i in inputs], context)
                if treatment_row["recipe_id"] == "translation_norm_scaled_v1":
                    _require(all(treatment[i]["treatment"] == "qualified_projection" for i in inputs), "NORM_PROJECTION_INPUT")
                    relative = max(projections[treatment[i]["projection_id"]]["relative_error_bound"] for i in inputs)
                    _require(relative <= NORM_INPUT_RELATIVE_LIMIT, "NORM_TOTAL_RELATIVE_BOUND")
                _require(all(treatment[i]["treatment"] in {"qualified_projection","checked_derived"} for i in inputs), "UNQUALIFIED_DERIVED_INPUT")
            elif token == "ordinary_physics_checked":
                _require(success and method != EXACT and treatment_row["projection_id"] is None and treatment_row["recipe_id"] is None and signature is not None, "ORDINARY_ROW")
            elif token == NOT_QUALIFIED:
                # Neutral: neither qualifies nor disqualifies the case (`usable` unchanged).
                _require(method == EXACT and success and treatment_row["projection_id"] is None, "NOT_QUALIFIED_ROW")
                _not_qualified(treatment_row, row, [raw[i] for i in inputs], signature)
            else:
                _require(treatment_row["projection_id"] is None and treatment_row["recipe_id"] is None, "INSPECTION_ROW")
                if signature is None or signature["category"] == "physical_quantity": usable = False
        _require(used_projections == set(projections), "PROJECTION_BIJECTION")
        # Dependency cycles cannot serve as evidence for checked derived rows.
        pending = set(treatment); done: set[str] = set()
        while pending:
            ready = {i for i in pending if set(treatment[i]["input_result_ids"]) <= done}
            _require(ready, "ROW_DEPENDENCY_CYCLE"); done |= ready; pending -= ready
        if success and method == EXACT:
            _supports(case, raw, projections, model, context)
            _required_affine_rows(case, raw, model, context)
            context.validate_source_case(source, case, actual_invocation)
        elif case["supports"]:
            _require(False, "UNSELECTED_SUPPORT_CERTIFICATE")
    invocation_work = body["invocation_work"]
    _require(invocation_work["charged"] == invocation_charged + invocation_work["publication_charged"] and invocation_work["charged"] <= invocation_work["limit"] <= 64_000_000, "INVOCATION_WORK_LEDGER")
    observations = body["envelope_observation_result_ids"]; _unique(observations, "OBSERVATION_IDS")
    _require(not covered & set(observations) and covered | set(observations) == set(raw), "ENVELOPE_ROW_COVERAGE")
    for identity in observations:
        row = raw[identity]; signature = _signature(row, context)
        _require(row.get("basis_ref") is None and signature is not None and signature["category"] != "physical_quantity", "PHYSICAL_OBSERVATION_ESCAPE")
    _require(not cases or any(c["selected_method"] == EXACT or c["outcome"] != "qualified" for c in cases), "SOURCE_METHOD_RECORD_REQUIRED")
    aggregate = "qualified" if cases and qualified == len(cases) else "partial" if qualified else "unavailable"
    _require(body["status"] == aggregate, "AGGREGATE_STATUS")
    _summary(source, raw, model, context)
    if model is not None and model.get("combinations"): usable = False
    return actual_invocation is not None and aggregate == "qualified" and usable and source["status"].get("mechanics") == "MECHANICS_SOLVED"


def _section_basis(row: Mapping[str, Any]) -> str | None:
    basis = (row.get("metadata") or {}).get("basis")
    return basis.rsplit("; section_basis=", 1)[1] if isinstance(basis, str) and "; section_basis=" in basis else None


def _not_qualified(treatment: Mapping[str, Any], row: Mapping[str, Any], inputs: list[Mapping[str, Any]], signature: Any) -> None:
    """§4.3: the kind's own recipe and exactly the station rows it reads, in formula order; sigma+/sigma-
    recomputed bit for bit with the exact-integer norm oracle; tau nonnegative (its c_b is the base reader's)."""
    kind = row.get("kind")
    _require(kind in NOT_QUALIFIED_KINDS, "NOT_QUALIFIED_KIND")
    _require(signature is not None, "NOT_QUALIFIED_SIGNATURE")
    _require(treatment.get("recipe_id") == NOT_QUALIFIED_RECIPES[kind], "NOT_QUALIFIED_RECIPE")
    metadata = row.get("metadata") or {}
    station = all(r.get("entity_ref") == row.get("entity_ref") and r.get("basis_ref") == row.get("basis_ref")
                  and (r.get("metadata") or {}).get("location") == metadata.get("location") for r in inputs)
    kinds = [r.get("kind") for r in inputs]
    value = row.get("value")
    if kind == NOT_QUALIFIED_KINDS[0]:
        basis = _section_basis(row)
        _require(station and basis is not None and all(_section_basis(r) == basis for r in inputs)
                 and kinds == ["element_local_axial_normal_stress", "element_local_bending_normal_stress_y", "element_local_bending_normal_stress_z"],
                 "NOT_QUALIFIED_INPUTS")
        n, y, z = (float(r["value"]) for r in inputs)
        h = correctly_rounded_norm([y, z])
        component = metadata.get("component")
        _require(component in ("sigma_plus", "sigma_minus"), "NOT_QUALIFIED_KIND")
        expected = n + h if component == "sigma_plus" else n - h
        _require(type(value) in (int, float) and struct.pack(">d", float(value)) == struct.pack(">d", expected), "NOT_QUALIFIED_VALUE")
    else:
        _require(station and kinds == ["element_local_shear_force_y", "element_local_shear_force_z"], "NOT_QUALIFIED_INPUTS")
        _require(metadata.get("component") == "tau_v_max" and type(value) in (int, float) and math.isfinite(value) and value >= 0, "NOT_QUALIFIED_VALUE")


# The characters Rust's `str::trim` removes from each end: those with the Unicode White_Space property (Rust std
# `char::is_whitespace`). Python's `str.strip()` also removes U+001C-U+001F, which Rust keeps, so the set is explicit.
RUST_WHITESPACE = frozenset(map(chr, [*range(0x09, 0x0E), 0x20, 0x85, 0xA0, 0x1680, *range(0x2000, 0x200B),
                                      0x2028, 0x2029, 0x202F, 0x205F, 0x3000]))


def _rust_trim(value: str) -> str:
    """Rust's `str::trim`."""
    start, end = 0, len(value)
    while start < end and value[start] in RUST_WHITESPACE: start += 1
    while end > start and value[end - 1] in RUST_WHITESPACE: end -= 1
    return value[start:end]


def _solver_support_hanger_type(support: Mapping[str, Any]) -> str | None:
    """The solver's `support_hanger_type` (core/product_physics/src/lib.rs), as Rust's reader mirrors it
    (`solver_support_hanger_type`, core/reporting/result_export/src/source_blocks.rs): the trimmed explicit
    `hanger.hanger_type` when non-empty, otherwise the trimmed family."""
    hanger = support.get("hanger")
    explicit = hanger.get("hanger_type") if isinstance(hanger, Mapping) else None
    reading = _rust_trim(explicit) if isinstance(explicit, str) else ""
    if not reading:
        family = support.get("family")
        reading = _rust_trim(family) if isinstance(family, str) else None
    return reading


def _solver_spring_support(support: Mapping[str, Any]) -> bool:
    """Whether the solver makes an authored support a ground spring, as Rust's reader mirrors it
    (`solver_spring_support`): family `spring` exactly, or a variable spring hanger as `support_hanger_type` reads it."""
    return support.get("family") == "spring" or _solver_support_hanger_type(support) in ("variable_spring_hanger", "spring_hanger")


def _solver_constant_effort_support(support: Mapping[str, Any]) -> bool:
    """The solver's `is_constant_effort_support`, as Rust's reader mirrors it (`solver_constant_effort_support`):
    `support_hanger_type` is `constant_effort_support`; a family `spring` does not override it. The solver builds no
    support from such a declaration and source recovery declines any model holding one, so no solver-produced plan
    names one in `support_ids` (CONSTANT_EFFORT_SUPPORT)."""
    return _solver_support_hanger_type(support) == "constant_effort_support"


def _supports(case: Mapping[str, Any], raw: dict[str, Any], projections: dict[str, Any], model: Mapping[str, Any] | None, context: Any) -> None:
    records = case["supports"]; plan = case["source"]
    _require([s["support_id"] for s in records] == plan["support_ids"], "SUPPORT_COVERAGE")
    ideals: set[int] = set(); spring_sources: set[tuple[str,int]] = set()
    projection_by_result = {p["result_id"]:p for p in projections.values()}
    nodes = {n["id"]:i for i,n in enumerate(model["nodes"])} if model is not None else None
    actual = {s["id"]:s for s in model["supports"]} if model is not None else None
    for support in records:
        _require({c["component"] for c in support["components"]} == set(COMPONENTS), "SIX_SUPPORT_COMPONENTS")
        if actual is not None:
            _require(actual[support["support_id"]]["node"] == support["node_id"] and support["node_id"] in nodes, "SUPPORT_NODE")
        inferred_node = None
        for component in support["components"]:
            slot = COMPONENTS.index(component["component"]); row = raw.get(component["result_id"]); projection = projection_by_result.get(component["result_id"])
            _require(row is not None and projection is not None and projection["functional_id"] == component["functional_id"] and projection["quantity"] == "support_action_component", "SUPPORT_PROJECTION")
            md = row.get("metadata") or {}
            _require(row["kind"] == "support_reaction_component_v2" and row["entity_ref"] == support["support_id"] and row["basis_ref"] == case["basis_ref"] and row["unit"] == ("N" if slot < 3 else "N*m") and md == {"component":component["component"],"coordinate_system":"global","location":"node","basis":"recovered_from_assembled_support_law","sign_convention":context.SUPPORT_SIGN}, "SUPPORT_ROW_SEMANTICS")
            terms = component["action_terms"]; _require(bool(terms), "SUPPORT_ACTION_TERMS")
            if actual is not None:
                authored = actual[support["support_id"]]
                stiffness = authored.get("stiffness") or (authored.get("hanger") or {}).get("stiffness")
                _require(not _solver_constant_effort_support(authored), "CONSTANT_EFFORT_SUPPORT")
                spring = _solver_spring_support(authored)
                dof_name = ("UX","UY","UZ","RX","RY","RZ")[slot]
                expected_kind = "ground_spring" if spring and stiffness and stiffness.get("dof") == dof_name else "ideal_constraint" if not spring and dof_name in authored.get("restraints",[]) else "structural_zero"
                _require(len(terms) == 1 and terms[0]["kind"] == expected_kind, "ACTUAL_SUPPORT_LAW")
            _unique([(t["kind"],t["source_id"],t["global_dof"]) for t in terms], "SUPPORT_TERM_DUPLICATE")
            for term in terms:
                dof = term["global_dof"]
                _require(dof < plan["dof_count"] and dof % 6 == slot and term["source_id"] == support["support_id"], "SUPPORT_ACTION_OWNER")
                inferred_node = dof // 6 if inferred_node is None else inferred_node
                _require(dof // 6 == inferred_node and (nodes is None or dof // 6 == nodes[support["node_id"]]), "SUPPORT_ACTION_DOF")
                if term["kind"] == "ideal_constraint":
                    _require(dof in plan["prescribed_dofs"] and dof not in ideals, "IDEAL_ATTRIBUTION_AMBIGUOUS"); ideals.add(dof)
                elif term["kind"] == "ground_spring":
                    source = (term["source_id"],dof); _require(source not in spring_sources, "SPRING_ATTRIBUTION_DUPLICATE"); spring_sources.add(source)
                else:
                    _require(len(terms) == 1 and row["value"] == 0 and projection["basis"] == "exact_zero", "STRUCTURAL_ZERO")
    _require(ideals == set(plan["prescribed_dofs"]), "IDEAL_SOURCE_COVERAGE")
    _require({c["result_id"] for s in records for c in s["components"]} == {p["result_id"] for p in projections.values() if p["quantity"] == "support_action_component"}, "SUPPORT_RESULT_BIJECTION")


def _required_affine_rows(case: Mapping[str, Any], raw: dict[str, Any], model: Mapping[str, Any] | None, context: Any) -> None:
    """The claimed complete source family must publish all primary affine rows."""
    projected = [raw[p["result_id"]] for p in case["projections"]]
    case_rows = [raw[r["result_id"]] for r in case["rows"]]
    components = ("axial_force","shear_force_y","shear_force_z","torsional_moment","bending_moment_y","bending_moment_z")
    for member in case["source"]["member_ids"]:
        for location in ("end_i","end_j","quarter_1","midspan","quarter_3"):
            for component in components:
                _require(sum(r["entity_ref"] == member and (r.get("metadata") or {}).get("component") == component and r["metadata"].get("location") == location for r in projected) == 1, "MEMBER_PRIMARY_COVERAGE")
        # T4-U6 (§2.1): one source stress row and one maximum per basis.
        for location in ("end_i","end_j","quarter_1","midspan","quarter_3"):
            for kind in ("element_local_axial_normal_stress","element_local_bending_normal_stress_y","element_local_bending_normal_stress_z","element_local_torsional_shear_stress"):
                for basis in sb.BASES:
                    _require(sum(r["entity_ref"] == member and r["kind"] == kind and (r.get("metadata") or {}).get("location") == location and _row_section_basis(r) == basis for r in case_rows) == 1, "MEMBER_STRESS_COVERAGE")
        for basis in sb.BASES:
            _require(sum(r["entity_ref"] == member and r["kind"] == "pipe_elastic_normal_stress_maximum_v2" and _row_section_basis(r) == basis for r in case_rows) == 1, "MEMBER_SUMMARY_COVERAGE")
    for support in case["source"]["support_ids"]:
        _require(sum(r["entity_ref"] == support and r["kind"] == "support_reaction_force_magnitude_v2" for r in case_rows) == 1, "SUPPORT_MAGNITUDE_COVERAGE")
    if model is not None:
        for node in model["nodes"]:
            _require(sum(r["entity_ref"] == node["id"] and r["kind"] == "displacement_magnitude" for r in case_rows) == 1, "NODAL_MAGNITUDE_COVERAGE")
            for kind in ("global_nodal_displacement_x","global_nodal_displacement_y","global_nodal_displacement_z","global_nodal_rotation_x","global_nodal_rotation_y","global_nodal_rotation_z"):
                _require(sum(r["entity_ref"] == node["id"] and r["kind"] == kind for r in projected) == 1, "NODAL_PRIMARY_COVERAGE")


def _row_section_basis(row: Mapping[str, Any]) -> str | None:
    """A stress row's section basis, named by its token and its id alike."""
    try:
        return sb.row_basis(row)[1]
    except sb.BasisError:
        return None


def _summary(source: Mapping[str, Any], raw: dict[str, Any], model: Mapping[str, Any] | None, context: Any) -> None:
    summary = source["summary"]
    if model is not None:
        _require(all(summary.get(key) == len(model[field]) for key,field in [("node_count","nodes"),("segment_count","pipe_segments"),("support_count","supports"),("load_case_count","load_cases")]), "SUMMARY_MODEL_COUNTS")
    # T4-U6 (D-5, §2.2): the composite's stress headline is per basis, bound to its own basis's maxima.
    _require("max_open_formula_stress" in summary and summary["max_open_formula_stress"] is None, "SUMMARY_HEADLINE")
    stress = "pipe_elastic_normal_stress_maximum_v2"
    for field, kind, basis in [("max_displacement", "displacement_magnitude", None),
                               ("max_open_formula_stress_nominal_section", stress, sb.NOMINAL), ("max_open_formula_stress_reduced_section", stress, sb.REDUCED)]:
        _require(basis is None or field in summary, "SUMMARY_HEADLINE")
        headline = summary.get(field)
        candidates = [r for r in raw.values() if r["kind"] == kind and (basis is None or _row_section_basis(r) == basis)]
        if candidates:
            _require(type(headline) is dict and set(headline) == {"value","unit","location_ref","result_ref"}, "SUMMARY_HEADLINE")
            selected = raw.get(headline["result_ref"])
            _require(selected is not None and selected["kind"] == kind and (basis is None or _row_section_basis(selected) == basis) and selected["unit"] == headline["unit"] and selected["entity_ref"] == headline["location_ref"] and _bits(selected["value"]) == _bits(headline["value"]) and headline["value"] == max(r["value"] for r in candidates), "SUMMARY_RESULT_BINDING")
        else:
            _require(headline is None, "SUMMARY_UNSOURCED_HEADLINE")
