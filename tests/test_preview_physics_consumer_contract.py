"""preview-physics-1 Python reader, standing and packaging (T0R S1 §9-§10).

Hand-built envelopes follow the frozen S1 interface exactly; they are reader
controls, not producer output. Tests over the actual producer fixtures are at
the end and skip until those fixtures exist.
"""
from copy import deepcopy
import hashlib
import json
import math
from pathlib import Path

import pytest

from core.analysis_runs.compatibility import (
    FRESH_CONTRACT_IDS, PRECISION_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_SHA256,
    PHYSICS_CONTRACT_ID, PHYSICS_SOURCE_CONTRACT_ID, SOURCE_BLOCKS_CONTRACT_ID, _source_contract,
    LOAD_REFERENCE_CONTRACT_ID, LOAD_REFERENCE_SOURCE_CONTRACT_ID,
    build_analysis_run, is_fresh_contract_id, numerical_use_standing, rule_binding_refusal,
    standing_reason, validate_analysis_run_v0_3, verify_analysis_run_record,
)
from core.analysis_runs.preview_physics_evidence import (
    LIMITATIONS, validate_preview_physics_evidence, validate_transport_metadata,
)
from schema_validation import schema_for_definition, validate_instance

PROJECT = Path(__file__).resolve().parents[1]
SUPPORT_SIGN = "support-on-pipe; positive global force and right-hand couple about attached node; force and moment norms remain separate"
MAX_META = {"component": "maximum_absolute_normal_stress", "coordinate_system": "pipe_section", "location": "governing_station", "basis": "recovered_from_open_mechanics_stress_components", "sign_convention": "nonnegative circumferential maximum |Nw/As|+hypot(My,Mz)/Z; bounded over all straight statics intervals; torsional shear remains separate; no code stress or equivalent stress claim"}
CASES = ["load:A", "load:B"]
PIPES = ["pipe:P1", "pipe:P2"]
NODES = ["node:N1", "node:N2"]


def L(*parts):
    """ID(): diagnostic identities, segments concatenated."""
    return "".join(f"{len(p.encode('utf-8'))}:{p}" for p in parts)


def R(*parts):
    """Row identities (S1 §3): segments joined by ':'."""
    return ":".join(f"{len(p.encode('utf-8'))}:{p}" for p in parts)


def lc(case):
    return {"ref_type": "load_case", "ref_id": case}


def meta(component, coordinate, location, basis, sign="positive value follows the declared convention"):
    return {"component": component, "coordinate_system": coordinate, "location": location, "basis": basis, "sign_convention": sign}


def envelope():
    """Two cases, two straight pipes, one attributed and one withheld support,
    one intensified end, one admitted and one gated mechanics combination."""
    results, diagnostics, preview_cases, quality = [], [], [], []
    for index, case in enumerate(CASES):
        scale = 1.0 + index  # case B governs
        tag = "" if index == 0 else f"loadcase:{case}:"
        for n, node in enumerate(NODES):
            comps = [0.3 * scale * (n + 1), -0.4 * scale * (n + 1), 1.2 * scale * (n + 1)]
            for axis, value in zip("xyz", comps):
                results.append({"id": f"result:{tag}disp:{node}:u{axis}", "kind": f"global_nodal_displacement_{axis}", "value": value, "unit": "mm", "entity_ref": node, "basis_ref": lc(case), "metadata": meta(f"nodal_displacement_{axis}", "global", "node", "solved_from_global_linear_system")})
            results.append({"id": f"result:{tag}disp:{node}", "kind": "displacement_magnitude", "value": math.hypot(math.hypot(comps[0], comps[1]), comps[2]), "unit": "mm", "entity_ref": node, "basis_ref": lc(case)})
        extrema = []
        for p, pipe in enumerate(PIPES):
            results.append({"id": f"result:{tag}force:{pipe}:end-i:axial", "kind": "element_local_axial_force", "value": -100.0 * scale, "unit": "N", "entity_ref": pipe, "basis_ref": lc(case), "metadata": meta("axial_force", "element_local", "end_i", "recovered_from_local_element_stiffness")})
            lower = 1.0e6 * scale * (p + 1)
            upper = lower + 2.0
            max_id = f"result:elastic-maximum:{R(case, pipe)}"
            results.append({"id": max_id, "kind": "pipe_elastic_normal_stress_maximum_v2", "value": lower + 0.5 * (upper - lower), "unit": "Pa", "entity_ref": pipe, "basis_ref": lc(case), "metadata": dict(MAX_META)})
            extrema.append({"pipe_id": pipe, "result_id": max_id, "station_fraction": 1.0, "span_index": 0, "local_fraction": 1.0, "value_lower_pa": lower, "value_upper_pa": upper, "global_upper_bound_pa": upper, "certified_gap_pa": 2.0, "subdivisions": 0, "approximation": "piecewise_quadratic_straight_section_statics", "coefficient_basis": "j_side_section_equilibrium_binary64", "enclosure_scope": "supplied_binary64_polynomial_coefficients; solution and coefficient formation error are separate"})
        my, mz, z = 30.0 * scale, -40.0 * scale, 2.0e-5
        by_id, bz_id = f"result:{tag}stress:pipe:P1:end-j:bending-normal-y", f"result:{tag}stress:pipe:P1:end-j:bending-normal-z"
        results.append({"id": by_id, "kind": "element_local_bending_normal_stress_y", "value": my / z / 1e6, "unit": "MPa", "entity_ref": "pipe:P1", "basis_ref": lc(case), "metadata": meta("bending_normal_stress_y", "pipe_section", "end_j", "recovered_from_open_mechanics_stress_components")})
        results.append({"id": bz_id, "kind": "element_local_bending_normal_stress_z", "value": mz / z / 1e6, "unit": "MPa", "entity_ref": "pipe:P1", "basis_ref": lc(case), "metadata": meta("bending_normal_stress_z", "pipe_section", "end_j", "recovered_from_open_mechanics_stress_components")})
        sif = 1.15
        intensified_id = f"result:{tag}intensified-bending:component-C1:pipe-P1:end-j"
        sign = f"nonnegative i*hypot(My,Mz)/Z at the member end; i={sif} user-entered bend SIF (source: ref:user-sif-table-3); member Z; not a code effective section modulus; not a code stress; no flexibility factor, axial or torsion term; never combined"
        results.append({"id": intensified_id, "kind": "component_equal_factor_intensified_bending_stress_v1", "value": sif * (math.hypot(my, mz) / z), "unit": "Pa", "entity_ref": "component:C1", "basis_ref": lc(case), "source_result_refs": [by_id, bz_id], "metadata": meta("equal_factor_intensified_bending_stress", "pipe_section", "end_j", "user_sif_times_member_section_bending_stress_v1", sign)})
        vector = {"Fx": 10.0 * scale, "Fy": -20.0 * scale, "Fz": 5.0 * scale, "Mx": 1.0 * scale, "My": 0.0, "Mz": -3.0 * scale}
        vector["force_magnitude"] = math.hypot(math.hypot(vector["Fx"], vector["Fy"]), vector["Fz"])
        vector["moment_magnitude"] = math.hypot(math.hypot(vector["Mx"], vector["My"]), vector["Mz"])
        for component, value in vector.items():
            kind = "support_reaction_force_magnitude_v2" if component == "force_magnitude" else "support_reaction_moment_magnitude_v2" if component == "moment_magnitude" else "support_reaction_component_v2"
            unit = "N" if component in {"Fx", "Fy", "Fz", "force_magnitude"} else "N*m"
            results.append({"id": f"result:support-action:{R(case, 'support:S1')}:{component}", "kind": kind, "value": value, "unit": unit, "entity_ref": "support:S1", "basis_ref": lc(case), "metadata": meta(component, "global", "node", "recovered_from_assembled_support_law", SUPPORT_SIGN)})
        integrity = f"diagnostic:numerical-integrity:{case}"
        diagnostics.append({"id": integrity, "code": "NUMERICAL_INTEGRITY_CHECKS_PASSED", "severity": "info", "message": "synthetic", "affected_refs": [case]})
        diagnostics.append({"id": f"diagnostic:preview-physics:intensification:{L(case, 'component:C1', 'pipe:P1', 'end_j')}", "code": "COMPONENT_EQUAL_FACTOR_INTENSIFICATION_APPLIED", "severity": "info", "message": "synthetic", "affected_refs": ["component:C1", "pipe:P1", intensified_id, "ref:user-sif-table-3", case]})
        diagnostics.append({"id": f"diagnostic:nonlinear:{case}:loop", "code": "NONLINEAR_SUPPORT_LOOP_CONVERGED", "severity": "info", "message": "synthetic", "affected_refs": ["DEC-046", case]})
        quality.append({"basis_ref": lc(case), "structural_status": "passive_model_basis", "solve_quality": "checks_passed", "model_matrix_fidelity": "represented_equations_retained", "accuracy_evidence": "not_claimed", "evidence_refs": [integrity]})
        preview_cases.append({"load_case_id": case, "pipe_stress_extrema": extrema,
            "stress_maximum_coverage": {"complete": True, "unavailable_pipe_ids": [], "outside_domain_pipe_ids": []},
            "support_attribution": {"attributed_support_ids": ["support:S1"], "withheld": [{"support_id": "support:NL", "reason": "SUPPORT_ACTION_ATTRIBUTION_WITHHELD"}]},
            "intensified_measures": [{"result_id": intensified_id, "component_id": "component:C1", "pipe_id": "pipe:P1", "location": "end_j", "factor_role": "bend", "sif": sif, "sif_source_reference": "ref:user-sif-table-3", "section_modulus_m3": z, "bending_moment_y_n_m": my, "bending_moment_z_n_m": mz}]})
    diagnostics.append({"id": f"diagnostic:preview-physics:attribution:{L('support:NL')}", "code": "SUPPORT_ACTION_ATTRIBUTION_WITHHELD", "severity": "warning", "message": "synthetic", "affected_refs": ["support:NL", "node:N2"]})
    diagnostics.append({"id": "diagnostic:spring-hanger:support-SH:hanger", "code": "SPRING_HANGER_USER_DATA_REVIEWED", "severity": "info", "message": "synthetic", "affected_refs": ["support:SH", "hanger"]})
    rows = {row["id"]: row for row in results}
    comb = "combination:C-OK"
    combined = []
    linear = "explicit_user_linear_combination"
    for node in NODES:
        parts = []
        for axis in "xyz":
            a, b = rows[f"result:disp:{node}:u{axis}"], rows[f"result:loadcase:load:B:disp:{node}:u{axis}"]
            value = a["value"] + b["value"]
            parts.append(value)
            combined.append({"id": f"result:combination:C-OK:disp:{node}:u{axis}", "kind": f"global_nodal_displacement_{axis}", "value": value, "unit": "mm", "entity_ref": node, "basis_ref": {"ref_type": "combination", "ref_id": comb}, "source_result_refs": [a["id"], b["id"]], "metadata": meta(f"nodal_displacement_{axis}", "global", "node", linear)})
        combined.append({"id": f"result:combination:C-OK:disp:{node}", "kind": "displacement_magnitude", "value": math.hypot(math.hypot(*parts[:2]), parts[2]), "unit": "mm", "entity_ref": node, "basis_ref": {"ref_type": "combination", "ref_id": comb}, "source_result_refs": [f"result:combination:C-OK:disp:{node}:u{axis}" for axis in "xyz"]})
    sums = {}
    for component in ("Fx", "Fy", "Fz", "Mx", "My", "Mz"):
        a, b = rows[f"result:support-action:{R('load:A', 'support:S1')}:{component}"], rows[f"result:support-action:{R('load:B', 'support:S1')}:{component}"]
        sums[component] = a["value"] + b["value"]
        combined.append({"id": f"result:combination:C-OK:support-action:{R('support:S1')}:{component}", "kind": "support_reaction_component_v2", "value": sums[component], "unit": a["unit"], "entity_ref": "support:S1", "basis_ref": {"ref_type": "combination", "ref_id": comb}, "source_result_refs": [a["id"], b["id"]], "metadata": meta(component, "global", "node", linear)})
    for component, names, kind, unit in (("force_magnitude", ("Fx", "Fy", "Fz"), "support_reaction_force_magnitude_v2", "N"), ("moment_magnitude", ("Mx", "My", "Mz"), "support_reaction_moment_magnitude_v2", "N*m")):
        value = math.hypot(math.hypot(sums[names[0]], sums[names[1]]), sums[names[2]])
        combined.append({"id": f"result:combination:C-OK:support-action:{R('support:S1')}:{component}", "kind": kind, "value": value, "unit": unit, "entity_ref": "support:S1", "basis_ref": {"ref_type": "combination", "ref_id": comb}, "source_result_refs": [f"result:combination:C-OK:support-action:{R('support:S1')}:{name}" for name in names], "metadata": meta(component, "global", "node", linear)})
    results.extend(combined)
    diagnostics.append({"id": f"diagnostic:preview-physics:combination-gate:{L('combination:C-NL')}", "code": "NONLINEAR_COMBINATION_REQUIRES_SOLVE", "severity": "warning", "message": "synthetic", "affected_refs": ["combination:C-NL"]})
    maxima = [row for row in results if row["kind"] == "pipe_elastic_normal_stress_maximum_v2"]
    top = max(maxima, key=lambda row: row["value"])
    disp = [row for row in results if row["kind"] == "displacement_magnitude" and row["basis_ref"]["ref_type"] == "load_case"]
    top_disp = max(disp, key=lambda row: row["value"])
    return {
        "schema_version": "0.2.0",
        "producer": {"component_name": "open_pipe_stress_product_physics", "component_version": "0.2.0", "semantic_contract_id": PREVIEW_PHYSICS_CONTRACT_ID},
        "numerical_quality": {"value_representation": "finite_binary64", "publication_quantization": "none", "integrity_policy": "M03-INTEGRITY-v1", "status": "checks_passed", "cases": quality},
        "formulation_basis": {"profile_id": "product_preview_mechanics_v1", "limitations": list(LIMITATIONS)},
        "document_kind": "openpipestress.product_preview.mechanics_result", "run_id": "preview-reader-control", "model_ref": "preview-reader-model",
        "status": {"mechanics": "MECHANICS_SOLVED", "rule_check": "RULE_INPUTS_INCOMPLETE"},
        "summary": {"node_count": 2, "segment_count": 2, "support_count": 2, "load_case_count": 2, "component_stress_modifier_count": 2,
                    "max_displacement": {"value": top_disp["value"], "unit": "mm", "location_ref": top_disp["entity_ref"], "result_ref": top_disp["id"]},
                    "max_open_formula_stress": {"value": top["value"], "unit": "Pa", "location_ref": top["entity_ref"], "result_ref": top["id"]}},
        "results": results, "diagnostics": diagnostics,
        "professional_boundary": {"human_review_required": True},
        "accepted_model_state_mutated": False,
        "contract_evidence": {"preview_cases": preview_cases, "combination_gates": [
            {"combination_id": "combination:C-OK", "withheld": False, "reason": None},
            {"combination_id": "combination:C-NL", "withheld": True, "reason": "NONLINEAR_COMBINATION_REQUIRES_SOLVE"}]},
    }


def build(source):
    return build_analysis_run(source, input_manifest_ref={"object_type": "InputManifest", "ref": "manifest:preview-reader"}, input_manifest_hash="1" * 64)


def bases(source):
    return [case["basis_ref"] for case in source["numerical_quality"]["cases"]]


def results_schema():
    return json.loads((PROJECT / "schemas/results.v0.3.schema.yaml").read_text())


def test_table_identity_and_registry():
    table = PROJECT / "fixtures/results/semantic_contract_v0_3_preview_physics_1.json"
    assert hashlib.sha256(table.read_bytes()).hexdigest() == PREVIEW_PHYSICS_CONTRACT_SHA256
    assert json.loads(table.read_text())["semantic_contract_id"] == PREVIEW_PHYSICS_CONTRACT_ID
    # T1 activation (DESIGN 10.3): T0R's four plus T1's two 0.4.0 exact-route identities.
    assert FRESH_CONTRACT_IDS == {PREVIEW_PHYSICS_CONTRACT_ID, SOURCE_BLOCKS_CONTRACT_ID, PHYSICS_CONTRACT_ID, PHYSICS_SOURCE_CONTRACT_ID,
                                  LOAD_REFERENCE_CONTRACT_ID, LOAD_REFERENCE_SOURCE_CONTRACT_ID}
    assert not is_fresh_contract_id(PRECISION_CONTRACT_ID) and is_fresh_contract_id(PREVIEW_PHYSICS_CONTRACT_ID)


def test_hand_built_envelope_reads_records_and_is_current_eligible():
    source = envelope()
    original = deepcopy(source)
    validate_preview_physics_evidence(source)
    assert _source_contract(source)[:2] == (PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_SHA256)
    record = build(source)
    assert source == original
    assert verify_analysis_run_record(record) == "match"
    validate_analysis_run_v0_3(record, source)
    validate_instance(json.loads((PROJECT / "schemas/analysis_run.schema.json").read_text()), record, instance_label="preview analysis")
    assert record["analysis_run"]["reproducibility"]["semantic_contract"] == {"id": PREVIEW_PHYSICS_CONTRACT_ID, "sha256": PREVIEW_PHYSICS_CONTRACT_SHA256}
    assert "contract_evidence" not in record["analysis_run"]
    assert all(ref["interpretation"]["status"] != "unavailable" for ref in record["analysis_run"]["result_refs"])
    assert numerical_use_standing(source, bases(source)) == "numerically_eligible"
    assert standing_reason(source) is None
    assert rule_binding_refusal(source, source["results"][0]) is None


def test_f1_positive_control_non_result_references_are_not_resolved():
    source = envelope()
    refs = {ref for item in source["diagnostics"] for ref in item.get("affected_refs", [])}
    assert {"hanger", "DEC-046", "ref:user-sif-table-3"} <= refs
    validate_preview_physics_evidence(source)


def test_arc_variant_resolves_first_and_generic_follows():
    from core.analysis_runs.preview_physics_evidence import signature
    row = {"kind": "element_local_axial_normal_stress", "unit": "MPa", "metadata": meta("axial_normal_stress", "pipe_section", "end_i", "nominal_straight_beam_formula_on_arc_resultants")}
    assert signature(row)["signature_id"] == "preview-physics-arc-015"
    row["metadata"]["basis"] = "recovered_from_open_mechanics_stress_components"
    assert signature(row)["signature_id"] == "supported-source-015"


def test_evidence_schema_is_closed_in_both_documents():
    import jsonschema
    source = envelope()
    for name in ("results.v0.3.schema.yaml", "stress_neutral_export.v0.3.schema.json"):
        schema = json.loads((PROJECT / "schemas" / name).read_text())
        validator = jsonschema.Draft202012Validator({"$ref": "#/$defs/PreviewPhysicsContractEvidence", "$defs": schema["$defs"]})
        validator.validate(source["contract_evidence"])
        for change in (lambda e: e["preview_cases"][0].update(extra=1),
                       lambda e: e["combination_gates"][0].update(reason="NONLINEAR_COMBINATION_REQUIRES_SOLVE"),
                       lambda e: e["combination_gates"][1].update(reason=None),
                       lambda e: e["preview_cases"][0]["support_attribution"]["withheld"][0].update(reason="UNKNOWN")):
            bad = deepcopy(source["contract_evidence"]); change(bad)
            with pytest.raises(jsonschema.ValidationError):
                validator.validate(bad)


def test_preview_metadata_vocabulary_is_scoped_to_its_branch():
    schema = results_schema()
    arc = meta("axial_force", "arc_chord_frame", "end_i", "recovered_from_local_element_stiffness")
    validate_instance(schema_for_definition(schema, "PreviewPhysicsResultMetadata"), arc)
    with pytest.raises(AssertionError):
        validate_instance(schema_for_definition(schema, "ResultMetadata"), arc)
    intensified = meta("equal_factor_intensified_bending_stress", "pipe_section", "end_j", "user_sif_times_member_section_bending_stress_v1")
    validate_instance(schema_for_definition(schema, "PreviewPhysicsResultMetadata"), intensified)
    branch = schema["$defs"]["ResultEnvelope"]["oneOf"][4]
    assert branch["properties"]["formulation_basis"]["properties"]["limitations"] == {"const": LIMITATIONS}
    assert branch["properties"]["result_sets"]["items"] == {"$ref": "#/$defs/PreviewPhysicsResultSet"}


def _retired_kind(s):
    s["results"].append({"id": "result:reaction:support-S1", "kind": "reaction_resultant", "value": 1.0, "unit": "N", "entity_ref": "support:S1", "basis_ref": lc("load:A")})


def _retired_code(s):
    s["diagnostics"].append({"id": "diagnostic:retired", "code": "COMPONENT_STRESS_MULTIPLIER_APPLIED", "severity": "info", "message": "x", "affected_refs": ["component:C1"]})


def _retired_skip_code(s):
    s["diagnostics"].append({"id": "diagnostic:retired-skip", "code": "COMBINATION_STRESS_SUMMARY_SKIPPED", "severity": "warning", "message": "x", "affected_refs": ["combination:C-OK"]})


def _dangling(s):
    s["diagnostics"][0]["affected_refs"].append("result:stress:pipe-P1")


def _dangling_summary(s):
    s["summary"]["max_displacement"]["result_ref"] = "result:disp:missing"


def _headline_not_governing(s):
    low = next(row for row in s["results"] if row["kind"] == "pipe_elastic_normal_stress_maximum_v2")
    s["summary"]["max_open_formula_stress"] = {"value": low["value"], "unit": "Pa", "location_ref": low["entity_ref"], "result_ref": low["id"]}


def _headline_first_case_only(s):
    first = [row for row in s["results"] if row["kind"] == "pipe_elastic_normal_stress_maximum_v2" and row["basis_ref"]["ref_id"] == "load:A"]
    top = max(first, key=lambda row: row["value"])
    s["summary"]["max_open_formula_stress"] = {"value": top["value"], "unit": "Pa", "location_ref": top["entity_ref"], "result_ref": top["id"]}


def _incomplete_with_headline(s):
    case = s["contract_evidence"]["preview_cases"][0]
    extremum = case["pipe_stress_extrema"].pop()
    s["results"] = [row for row in s["results"] if row["id"] != extremum["result_id"]]
    case["stress_maximum_coverage"] = {"complete": False, "unavailable_pipe_ids": [extremum["pipe_id"]], "outside_domain_pipe_ids": []}


def _outside_bounds(s):
    s["contract_evidence"]["preview_cases"][1]["pipe_stress_extrema"][1]["value_upper_pa"] -= 3.0


def _off_midpoint(s):
    row = next(row for row in s["results"] if row["kind"] == "pipe_elastic_normal_stress_maximum_v2" and row["basis_ref"]["ref_id"] == "load:A")
    row["value"] += 0.25


def _support_magnitude(s):
    next(row for row in s["results"] if row["kind"] == "support_reaction_force_magnitude_v2" and row["basis_ref"]["ref_type"] == "load_case")["value"] *= 1.001


def _dropped_moment(s):
    dropped = f"result:support-action:{R('load:A', 'support:S1')}:Mz"
    s["results"] = [row for row in s["results"] if row["id"] != dropped]
    for row in s["results"]:
        if dropped in row.get("source_result_refs", []):
            row["source_result_refs"].remove(dropped)


def _combination_magnitude_linear(s):
    row = next(row for row in s["results"] if row["kind"] == "support_reaction_force_magnitude_v2" and row["basis_ref"]["ref_type"] == "combination")
    a = next(r for r in s["results"] if r["id"] == f"result:support-action:{R('load:A', 'support:S1')}:force_magnitude")
    b = next(r for r in s["results"] if r["id"] == f"result:support-action:{R('load:B', 'support:S1')}:force_magnitude")
    row["value"] = a["value"] + b["value"] + 1.0


def _combination_displacement_magnitude(s):
    next(row for row in s["results"] if row["kind"] == "displacement_magnitude" and row["basis_ref"]["ref_type"] == "combination")["value"] += 1.0


def _gated_combination_row(s):
    row = deepcopy(next(row for row in s["results"] if row["basis_ref"] and row["basis_ref"]["ref_type"] == "combination"))
    row["id"] = "result:combination:C-NL:published"
    row["basis_ref"] = {"ref_type": "combination", "ref_id": "combination:C-NL"}
    s["results"].append(row)


def _missing_withheld_record(s):
    for case in s["contract_evidence"]["preview_cases"]:
        case["support_attribution"]["withheld"] = []


def _withheld_record_in_one_case_only(s):
    s["contract_evidence"]["preview_cases"][1]["support_attribution"]["withheld"] = []


def _withheld_reason_changed(s):
    for case in s["contract_evidence"]["preview_cases"]:
        case["support_attribution"]["withheld"][0]["reason"] = "CONSTANT_EFFORT_NOT_CONSUMED"


def _zero_filled_withheld(s):
    for component, kind, unit in (("Fx", "support_reaction_component_v2", "N"), ("Fy", "support_reaction_component_v2", "N"), ("Fz", "support_reaction_component_v2", "N"), ("Mx", "support_reaction_component_v2", "N*m"), ("My", "support_reaction_component_v2", "N*m"), ("Mz", "support_reaction_component_v2", "N*m"), ("force_magnitude", "support_reaction_force_magnitude_v2", "N"), ("moment_magnitude", "support_reaction_moment_magnitude_v2", "N*m")):
        s["results"].append({"id": f"result:support-action:{R('load:A', 'support:NL')}:{component}", "kind": kind, "value": 0.0, "unit": unit, "entity_ref": "support:NL", "basis_ref": lc("load:A"), "metadata": meta(component, "global", "node", "recovered_from_assembled_support_law", SUPPORT_SIGN)})


def _withheld_final_reaction(s):
    s["results"].append({"id": "result:nonlinear-support:support-NL:uz-reaction", "kind": "nonlinear_support_final_reaction", "value": 1.0, "unit": "N", "entity_ref": "support:NL", "basis_ref": lc("load:A"), "metadata": meta("nonlinear_support_final_reaction", "solver_iteration", "uz", "sparse_interactive_active_set_loop")})


def _intensified_in_combination(s):
    row = deepcopy(next(row for row in s["results"] if row["kind"] == "component_equal_factor_intensified_bending_stress_v1"))
    row.update(id="result:combination:C-OK:intensified", basis_ref={"ref_type": "combination", "ref_id": "combination:C-OK"})
    s["results"].append(row)
    s["summary"]["component_stress_modifier_count"] = 3


def _intensified_with_k(s):
    row = next(row for row in s["results"] if row["kind"] == "component_equal_factor_intensified_bending_stress_v1")
    row["value"] *= 1.08


def _intensified_without_measure(s):
    s["contract_evidence"]["preview_cases"][0]["intensified_measures"] = []


def _intensified_count(s):
    s["summary"]["component_stress_modifier_count"] = 8


def _maximum_in_combination(s):
    row = deepcopy(next(row for row in s["results"] if row["kind"] == "pipe_elastic_normal_stress_maximum_v2"))
    row.update(id="result:combination:C-OK:maximum", basis_ref={"ref_type": "combination", "ref_id": "combination:C-OK"})
    s["results"].append(row)


def _arc_maximum(s):
    case = s["contract_evidence"]["preview_cases"][0]
    case["stress_maximum_coverage"] = {"complete": False, "unavailable_pipe_ids": [], "outside_domain_pipe_ids": ["pipe:P2"]}


def _case_scope(s):
    s["contract_evidence"]["preview_cases"].pop()


def _limitations(s):
    s["formulation_basis"]["limitations"] = s["formulation_basis"]["limitations"][:-1]


def _evidence_extra_key(s):
    s["contract_evidence"]["connector"] = []


def _extrema_shape(s):
    s["contract_evidence"]["preview_cases"][0]["pipe_stress_extrema"][0]["enclosure_scope"] = "unknown"


def _abs_sum_maximum(s):
    row = next(row for row in s["results"] if row["kind"] == "pipe_elastic_normal_stress_maximum_v2")
    row["metadata"]["sign_convention"] = "sum of absolute axial and bending components"


TAMPERS = [
    _retired_kind, _retired_code, _retired_skip_code, _dangling, _dangling_summary, _headline_not_governing,
    _headline_first_case_only, _incomplete_with_headline, _outside_bounds, _off_midpoint, _support_magnitude,
    _dropped_moment, _combination_magnitude_linear, _combination_displacement_magnitude, _gated_combination_row,
    _missing_withheld_record, _withheld_record_in_one_case_only, _withheld_reason_changed, _zero_filled_withheld, _withheld_final_reaction, _intensified_in_combination,
    _intensified_with_k, _intensified_without_measure, _intensified_count, _maximum_in_combination, _arc_maximum,
    _case_scope, _limitations, _evidence_extra_key, _extrema_shape, _abs_sum_maximum,
]


@pytest.mark.parametrize("change", TAMPERS, ids=lambda f: f.__name__.strip("_"))
def test_tampered_envelope_is_refused_before_record_or_standing(change):
    source = envelope()
    change(source)
    with pytest.raises(ValueError, match="SOURCE_PREVIEW_PHYSICS_"):
        _source_contract(source)
    with pytest.raises(ValueError, match="SOURCE_"):
        build(source)
    assert numerical_use_standing(source, bases(source)) == "unsupported"


def test_identity_tie_selects_smaller_case_then_member():
    source = envelope()
    rows = {row["id"]: row for row in source["results"]}
    # Give both cases the same governing maximum on pipe P2.
    for case in source["contract_evidence"]["preview_cases"]:
        extremum = next(e for e in case["pipe_stress_extrema"] if e["pipe_id"] == "pipe:P2")
        extremum.update(value_lower_pa=9.0e6, value_upper_pa=9.0e6 + 2.0, global_upper_bound_pa=9.0e6 + 2.0)
        rows[extremum["result_id"]]["value"] = 9.0e6 + 1.0
    first = rows[f"result:elastic-maximum:{R('load:A', 'pipe:P2')}"]
    source["summary"]["max_open_formula_stress"] = {"value": first["value"], "unit": "Pa", "location_ref": "pipe:P2", "result_ref": first["id"]}
    validate_preview_physics_evidence(source)
    later = rows[f"result:elastic-maximum:{R('load:B', 'pipe:P2')}"]
    source["summary"]["max_open_formula_stress"]["result_ref"] = later["id"]
    with pytest.raises(ValueError, match="does not govern"):
        validate_preview_physics_evidence(source)


def test_incomplete_coverage_withholds_headline_but_keeps_rows():
    source = envelope()
    case = source["contract_evidence"]["preview_cases"][0]
    extremum = case["pipe_stress_extrema"].pop()
    source["results"] = [row for row in source["results"] if row["id"] != extremum["result_id"]]
    case["stress_maximum_coverage"] = {"complete": False, "unavailable_pipe_ids": [], "outside_domain_pipe_ids": [extremum["pipe_id"]]}
    source["summary"]["max_open_formula_stress"] = None
    validate_preview_physics_evidence(source)
    assert verify_analysis_run_record(build(source)) == "match"


def test_blocked_envelope_with_empty_evidence_is_inspectable_not_eligible():
    source = envelope()
    source.update(results=[], diagnostics=[], contract_evidence={"preview_cases": [], "combination_gates": []})
    source["status"]["mechanics"] = "MODEL_INCOMPLETE"
    source["numerical_quality"].update(status="not_assessed", cases=[])
    source["summary"].update(max_displacement=None, max_open_formula_stress=None, component_stress_modifier_count=0)
    validate_preview_physics_evidence(source)
    assert verify_analysis_run_record(build(source)) == "match"
    assert numerical_use_standing(source, []) == "needs_recompute"


def test_transport_metadata_checks_statement_without_rows():
    source = envelope()
    metadata = {"schema_version": "0.2.0", **{key: source[key] for key in ("producer", "numerical_quality", "formulation_basis", "contract_evidence")}}
    validate_transport_metadata(metadata)
    assert _source_contract(metadata, check_receipt=False)[0] == PREVIEW_PHYSICS_CONTRACT_ID
    for change in (lambda m: m["contract_evidence"].update(extra=[]),
                   lambda m: m["contract_evidence"]["preview_cases"][0]["stress_maximum_coverage"].update(complete=False),
                   lambda m: m["contract_evidence"]["combination_gates"][1].update(reason=None),
                   lambda m: m["formulation_basis"]["limitations"].append("extra")):
        bad = deepcopy(metadata); change(bad)
        with pytest.raises(ValueError, match="SOURCE_PREVIEW_PHYSICS_"):
            _source_contract(bad, check_receipt=False)


@pytest.mark.parametrize("identity", [PRECISION_CONTRACT_ID, PHYSICS_CONTRACT_ID, SOURCE_BLOCKS_CONTRACT_ID, "openpipestress.result_semantics/0.3.0/preview-physics-2"])
def test_preview_never_falls_back_to_another_contract(identity):
    source = envelope()
    source["producer"]["semantic_contract_id"] = identity
    with pytest.raises(ValueError, match="SOURCE_"):
        build(source)


def test_preview_evidence_is_required_and_exclusive():
    source = envelope()
    source.pop("contract_evidence")
    with pytest.raises(ValueError, match="SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED"):
        _source_contract(source)
    source = envelope()
    source["source_block_recovery"] = {}
    with pytest.raises(ValueError, match="SOURCE_"):
        _source_contract(source)


def precision_fixture():
    return json.loads((PROJECT / "fixtures/product_preview/invented_mechanics_result_precision_1_sparse.json").read_text())


def test_precision_1_is_readable_but_never_current():
    source = precision_fixture()
    record = build(source)
    assert verify_analysis_run_record(record) == "match"
    assert source["numerical_quality"]["status"] == "checks_passed"
    assert numerical_use_standing(source, bases(source)) == "needs_recompute"
    assert standing_reason(source) == "PRECISION_1_HISTORICAL_SEMANTICS"


def test_precision_1_offered_under_the_fresh_identity_is_refused():
    source = precision_fixture()
    source["producer"]["semantic_contract_id"] = PREVIEW_PHYSICS_CONTRACT_ID
    with pytest.raises(ValueError, match="SOURCE_PREVIEW_PHYSICS_"):
        _source_contract(source)
    assert numerical_use_standing(source, bases(source)) == "unsupported"


# ---- amendment A1 reader parity (a-g) ----

def test_a1_a_diagnostic_without_affected_refs_is_allowed():
    source = envelope()
    source["diagnostics"].append({"id": "diagnostic:physics:rule-inputs-missing", "code": "RULE_CHECK_INPUTS_MISSING", "severity": "warning", "message": "synthetic"})
    validate_preview_physics_evidence(source)


def _nonlinear_row(support, kind="nonlinear_support_final_displacement", unit="mm", case="load:A"):
    component = {"nonlinear_support_active_set_state_code": "active_set_state_code"}.get(kind, kind)
    return {"id": f"result:nonlinear-support:{support}:{kind}:{case}", "kind": kind, "value": 0.1, "unit": unit, "entity_ref": support, "basis_ref": lc(case), "metadata": meta(component, "solver_iteration", "uz", "sparse_interactive_active_set_loop")}


def test_a1_b_per_support_nonlinear_rows_name_listed_supports():
    source = envelope()
    source["results"].append(_nonlinear_row("support:NL"))  # withheld but listed: allowed
    source["results"].append(_nonlinear_row("support:S1", "nonlinear_support_active_set_state_code", "state_code"))
    source["results"].append(_nonlinear_row("support:S1", "nonlinear_support_final_reaction", "N"))
    validate_preview_physics_evidence(source)


def test_a1_c_range_combination_is_exempt_from_magnitude_consistency():
    source = envelope()
    for row in source["results"]:
        if (row.get("basis_ref") or {}).get("ref_type") == "combination" and row.get("metadata"):
            row["metadata"]["basis"] = "explicit_user_range_envelope"
    next(row for row in source["results"] if row["kind"] == "support_reaction_force_magnitude_v2" and row["basis_ref"]["ref_type"] == "combination")["value"] += 5.0
    validate_preview_physics_evidence(source)


def test_a1_d_no_certified_gap_or_global_bound_is_checked():
    source = envelope()
    extremum = source["contract_evidence"]["preview_cases"][0]["pipe_stress_extrema"][0]
    extremum.update(station_fraction=0.0, local_fraction=1.0, certified_gap_pa=0.0, global_upper_bound_pa=0.0, subdivisions=131072)
    validate_preview_physics_evidence(source)


def blocked():
    source = envelope()
    source.update(results=[], diagnostics=[{"id": "diagnostic:pressure:reauthor", "code": "PRESSURE_MODEL_REAUTHOR_REQUIRED", "severity": "blocking", "message": "synthetic", "affected_refs": ["load:A"]}], contract_evidence={"preview_cases": [], "combination_gates": []})
    source["status"]["mechanics"] = "MODEL_INCOMPLETE"
    source["numerical_quality"].update(status="not_assessed", cases=[])
    source["summary"].update(max_displacement=None, max_open_formula_stress=None, component_stress_modifier_count=0)
    return source


def _blocked_rows(s):
    s["results"].append({"id": "result:disp:node:N1", "kind": "displacement_magnitude", "value": 0.0, "unit": "mm", "entity_ref": "node:N1", "basis_ref": lc("load:A")})


def _blocked_headline(s):
    s["summary"]["max_displacement"] = {"value": 0.0, "unit": "mm", "location_ref": "node:N1", "result_ref": None}


def _blocked_evidence(s):
    s["contract_evidence"]["combination_gates"].append({"combination_id": "combination:C-OK", "withheld": False, "reason": None})


def _blocked_result_ref(s):
    s["diagnostics"][0]["affected_refs"].append("result:stress:pipe-P1")


def _blocked_retired_code(s):
    s["diagnostics"].append({"id": "diagnostic:retired", "code": "COMPONENT_STRESS_MULTIPLIER_APPLIED", "severity": "info", "message": "x", "affected_refs": ["component:C1"]})


@pytest.mark.parametrize("change", [_blocked_rows, _blocked_headline, _blocked_evidence, _blocked_result_ref, _blocked_retired_code], ids=lambda f: f.__name__.strip("_"))
def test_a1_e_blocked_envelope_is_empty_and_clean(change):
    source = blocked()
    validate_preview_physics_evidence(source)
    change(source)
    with pytest.raises(ValueError, match="SOURCE_PREVIEW_PHYSICS_"):
        _source_contract(source)


def _unlisted_nonlinear_support(s):
    s["results"].append(_nonlinear_row("support:UNLISTED", "nonlinear_support_active_set_state_code", "state_code"))


def _withholding_diagnostic_wrong_support(s):
    item = next(d for d in s["diagnostics"] if d["code"] == "SUPPORT_ACTION_ATTRIBUTION_WITHHELD")
    item["affected_refs"][0] = "support:S1"


def _withholding_diagnostic_wrong_id(s):
    next(d for d in s["diagnostics"] if d["code"] == "SUPPORT_ACTION_ATTRIBUTION_WITHHELD")["id"] += ":load:A"


def _withholding_diagnostic_without_refs(s):
    next(d for d in s["diagnostics"] if d["code"] == "SUPPORT_ACTION_ATTRIBUTION_WITHHELD").pop("affected_refs")


def _negative_lower_bound(s):
    case = s["contract_evidence"]["preview_cases"][0]
    extremum = case["pipe_stress_extrema"][0]
    row = next(r for r in s["results"] if r["id"] == extremum["result_id"])
    extremum.update(value_lower_pa=-2.0, value_upper_pa=4.0)
    row["value"] = 1.0


def _intensified_sign_prefix(s):
    row = next(r for r in s["results"] if r["kind"] == "component_equal_factor_intensified_bending_stress_v1")
    row["metadata"]["sign_convention"] = "nonnegative i*k*hypot(My,Mz)/Z; " + row["metadata"]["sign_convention"]


def _intensified_basis(s):
    next(r for r in s["results"] if r["kind"] == "component_equal_factor_intensified_bending_stress_v1")["metadata"]["basis"] = "stress_recovery_summary"


def _support_sign_convention(s):
    next(r for r in s["results"] if r["kind"] == "support_reaction_component_v2")["metadata"]["sign_convention"] = "support_on_pipe_positive_global_force_right_hand_couple_at_attachment_node"


def _mechanics_combination_without_basis(s):
    for row in s["results"]:
        if (row.get("basis_ref") or {}).get("ref_type") == "combination" and row.get("metadata"):
            row["metadata"]["basis"] = "explicit_user_result_state_subtraction"
    next(row for row in s["results"] if row["kind"] == "support_reaction_moment_magnitude_v2" and row["basis_ref"]["ref_type"] == "combination")["value"] += 5.0


@pytest.mark.parametrize("change", [_unlisted_nonlinear_support, _withholding_diagnostic_wrong_support, _withholding_diagnostic_wrong_id, _withholding_diagnostic_without_refs, _negative_lower_bound, _intensified_sign_prefix, _intensified_basis, _support_sign_convention, _mechanics_combination_without_basis], ids=lambda f: f.__name__.strip("_"))
def test_a1_tamper_controls(change):
    source = envelope()
    change(source)
    with pytest.raises(ValueError, match="SOURCE_PREVIEW_PHYSICS_"):
        _source_contract(source)
    assert numerical_use_standing(source, bases(source)) == "unsupported"


# ---- amendment A2 strict list (items 1-10) and the shared tamper vector ----

def _a2_extremum(**values):
    def change(s):
        s["contract_evidence"]["preview_cases"][0]["pipe_stress_extrema"][0].update(values)
    change.__name__ = "_extremum_" + "_".join(f"{k}_{v}" for k, v in values.items())
    return change


def _a2_case_order(s):
    s["contract_evidence"]["preview_cases"].reverse()


def _a2_attribution_differs(s):
    s["contract_evidence"]["preview_cases"][1]["support_attribution"]["attributed_support_ids"].append("support:EXTRA")


def _a2_combination_support_frame(s):
    next(r for r in s["results"] if r["kind"] == "support_reaction_component_v2" and r["basis_ref"]["ref_type"] == "combination")["metadata"]["coordinate_system"] = "element_local"


def _a2_combination_support_location(s):
    next(r for r in s["results"] if r["kind"] == "support_reaction_component_v2" and r["basis_ref"]["ref_type"] == "combination")["metadata"]["location"] = "end_i"


def _a2_combination_support_component(s):
    next(r for r in s["results"] if r["kind"] == "support_reaction_force_magnitude_v2" and r["basis_ref"]["ref_type"] == "combination")["metadata"]["component"] = "Fx"


def _a2_null_affected_refs(s):
    s["diagnostics"][0]["affected_refs"] = None


def _a2_empty_affected_ref(s):
    s["diagnostics"][0]["affected_refs"].append("")


def _a2_empty_sif_source(s):
    s["contract_evidence"]["preview_cases"][0]["intensified_measures"][0]["sif_source_reference"] = ""


def _a2_missing_entity_ref(s):
    s["results"][0].pop("entity_ref")


def _a2_empty_unit(s):
    s["results"][0]["unit"] = ""


@pytest.mark.parametrize("change", [
    _a2_extremum(station_fraction=1.5), _a2_extremum(local_fraction=-0.25), _a2_extremum(span_index=-1),
    _a2_extremum(span_index=1.5), _a2_extremum(subdivisions=4.5), _a2_extremum(subdivisions=131073), _a2_extremum(subdivisions=-1), _a2_extremum(subdivisions=True),
    _a2_case_order, _a2_attribution_differs, _a2_combination_support_frame, _a2_combination_support_location,
    _a2_combination_support_component, _a2_null_affected_refs, _a2_empty_affected_ref, _a2_empty_sif_source,
    _a2_missing_entity_ref, _a2_empty_unit,
], ids=lambda f: f.__name__.strip("_"))
def test_a2_tamper_controls(change):
    source = envelope()
    change(source)
    with pytest.raises(ValueError, match="SOURCE_PREVIEW_PHYSICS_"):
        _source_contract(source)
    assert numerical_use_standing(source, bases(source)) == "unsupported"


def _a3_negative_intensified(s):
    row = next(r for r in s["results"] if r["kind"] == "component_equal_factor_intensified_bending_stress_v1")
    measure = next(m for c in s["contract_evidence"]["preview_cases"] for m in c["intensified_measures"] if m["result_id"] == row["id"])
    # Consistent with its inputs except for the sign: only A3 1 can refuse it.
    measure.update(bending_moment_y_n_m=0.0, bending_moment_z_n_m=0.0)
    row["value"] = -1e-300


def _a3_attributed_and_withheld(s):
    for case in s["contract_evidence"]["preview_cases"]:
        case["support_attribution"]["attributed_support_ids"].append("support:NL")


@pytest.mark.parametrize("change", [_a3_negative_intensified, _a3_attributed_and_withheld], ids=lambda f: f.__name__.strip("_"))
def test_a3_tamper_controls(change):
    source = envelope()
    change(source)
    with pytest.raises(ValueError, match="negative intensified value|support both attributed and withheld"):
        _source_contract(source)
    assert numerical_use_standing(source, bases(source)) == "unsupported"


def test_a4_integers_by_numeric_value_are_accepted():
    source = envelope()
    source["contract_evidence"]["preview_cases"][0]["pipe_stress_extrema"][0].update(span_index=1.0, subdivisions=4.0)
    source["summary"]["component_stress_modifier_count"] = 2.0
    validate_preview_physics_evidence(source)
    for bad in (True, 2.5):
        tampered = deepcopy(source); tampered["summary"]["component_stress_modifier_count"] = bad
        with pytest.raises(ValueError, match="intensified row count"):
            validate_preview_physics_evidence(tampered)
    stopped = blocked()
    stopped["summary"]["component_stress_modifier_count"] = 0.0
    validate_preview_physics_evidence(stopped)
    stopped["summary"]["component_stress_modifier_count"] = False
    with pytest.raises(ValueError, match="blocked envelope counts"):
        validate_preview_physics_evidence(stopped)


def test_a4_n6_support_row_frame_is_global_without_basis():
    source = envelope()
    row = next(r for r in source["results"] if r["kind"] == "support_reaction_component_v2" and r["basis_ref"]["ref_type"] == "load_case")
    row.pop("basis_ref")
    row["metadata"]["coordinate_system"] = "element_local"
    with pytest.raises(ValueError, match="support action frame"):
        validate_preview_physics_evidence(source)


def test_a2_6_blocked_modifier_count_is_zero():
    source = blocked()
    source["summary"]["component_stress_modifier_count"] = 3
    with pytest.raises(ValueError, match="SOURCE_PREVIEW_PHYSICS_"):
        _source_contract(source)


def test_a2_10_standing_validates_before_a_standing_reason():
    precision = precision_fixture()
    assert numerical_use_standing(precision, bases(precision)) == "needs_recompute"
    precision["numerical_quality"]["status"] = "unknown"
    assert numerical_use_standing(precision, bases(precision)) == "unsupported"
    mixed, context = _mixed_source_blocks()
    bases_ = [case["basis_ref"] for case in mixed["source_block_recovery"]["body"]["cases"]]
    assert numerical_use_standing(mixed, bases_, context) == "needs_recompute"
    mixed["source_block_recovery"]["receipt_sha256"] = "0" * 64
    assert numerical_use_standing(mixed, bases_, context) == "unsupported"
    assert standing_reason(mixed) is None


def _pointer(path):
    return [part.replace("~1", "/").replace("~0", "~") for part in path.split("/")[1:]]


def _apply(document, op):
    *parents, leaf = _pointer(op["path"])
    target = document
    for part in parents:
        target = target[int(part)] if isinstance(target, list) else target[part]
    key = int(leaf) if isinstance(target, list) and leaf != "-" else leaf
    if op["op"] == "replace":
        assert key in range(len(target)) if isinstance(target, list) else key in target, op
        target[key] = deepcopy(op["value"])
    elif op["op"] == "add":
        if isinstance(target, list):
            index = len(target) if leaf == "-" else int(leaf)
            assert 0 <= index <= len(target), op
            target.insert(index, deepcopy(op["value"]))
        else:
            target[key] = deepcopy(op["value"])
    elif op["op"] == "remove":
        del target[key]
    elif op["op"] == "reverse":
        target[key].reverse()
    else:
        raise AssertionError(f"unknown op {op['op']}")


TAMPER_VECTOR = json.loads((PROJECT / "fixtures/results/preview_physics_tamper_vector.json").read_text(encoding="utf-8"))


@pytest.mark.parametrize("variant", TAMPER_VECTOR["variants"], ids=lambda v: v["id"])
def test_shared_tamper_vector(variant):
    source = json.loads((PROJECT / TAMPER_VECTOR["bases"][variant["base"]]).read_text(encoding="utf-8"))
    for op in variant["ops"]:
        _apply(source, op)
    if variant["expect"] == "accepted":
        _source_contract(source)
        assert numerical_use_standing(source, bases(source)) != "unsupported"
    else:
        assert variant["expect"] == "refused"
        with pytest.raises(ValueError, match="SOURCE_PREVIEW_PHYSICS_"):
            _source_contract(source)
        assert numerical_use_standing(source, bases(source)) == "unsupported"


# ---- source-blocks-1 standing (non-composite only) and rule binding ----

def _mixed_source_blocks():
    from test_source_blocks_validation import control, seal
    source, context = control()
    body = source["source_block_recovery"]["body"]
    first = body["cases"][0]
    basis = {"ref_type": "load_case", "ref_id": "ordinary-case"}
    second = deepcopy(first)
    second.update(basis_ref=basis, selected_method="ordinary_dense_structural_v1", source=None, projections=[], supports=[])
    second["ordinary_attempt"]["quality_case_index"] = 1
    second["ordinary_attempt"]["structural_report_diagnostic_ref"] = "ordinary-report"
    extra = []
    second["rows"] = []
    for original in source["results"]:
        if original["kind"] == "support_reaction_component_v2":
            continue
        row = deepcopy(original); row["id"] = "ordinary:" + row["id"]; row["basis_ref"] = basis; extra.append(row)
        second["rows"].append({"result_id": row["id"], "treatment": "ordinary_checked", "projection_id": None, "recipe_id": None, "input_result_ids": []})
    source["results"].extend(extra)
    source["diagnostics"].append({"id": "ordinary-report", "code": "NUMERICAL_INTEGRITY_CHECKS_PASSED", "severity": "info", "message": "Synthetic ordinary statement only."})
    quality = deepcopy(source["numerical_quality"]["cases"][0]); quality.update(basis_ref=basis, evidence_refs=["ordinary-report"])
    source["numerical_quality"]["cases"].append(quality)
    body["cases"].append(second)
    body["invocation_work"]["charged"] += second["work"]["charged"] + second["work"]["reserved_unobserved_failure"]
    context["request"]["model"]["load_cases"].append({"id": "ordinary-case", "primitive_loads": []})
    source["summary"]["load_case_count"] = 2
    seal(source, context)
    return source, context


def test_selected_plus_ordinary_source_blocks_is_not_current():
    from core.analysis_runs.source_blocks import validate_source_blocks
    source, context = _mixed_source_blocks()
    bases_ = [case["basis_ref"] for case in source["source_block_recovery"]["body"]["cases"]]
    assert validate_source_blocks(source, context) is True  # statement itself is valid
    assert numerical_use_standing(source, bases_, context) == "needs_recompute"
    assert standing_reason(source) == "SOURCE_BLOCKS_ORDINARY_CASE_LEGACY_SEMANTICS"


def test_all_selected_source_blocks_keeps_standing():
    from test_source_blocks_validation import received_artifacts
    for name, source, context in received_artifacts():
        bases_ = [{"ref_type": "load_case", "ref_id": case["id"]} for case in context["request"]["model"]["load_cases"]]
        assert standing_reason(source) is None, name
        assert numerical_use_standing(source, bases_, context) == "numerically_eligible", name


def test_partial_source_blocks_receipt_stays_not_eligible():
    from test_source_blocks_validation import control, seal
    source, context = control()
    body = source["source_block_recovery"]["body"]
    case = body["cases"][0]
    case.update(outcome="failed", selected_method=None, source=None, projections=[], supports=[], failure={"stage": "exact_solve", "code": "budget", "diagnostic_ref": "synthetic-report", "block_order": None})
    for row in case["rows"]:
        row.update(treatment="inspection_only", projection_id=None, recipe_id=None, input_result_ids=[])
    body["status"] = "unavailable"
    seal(source, context)
    bases_ = [c["basis_ref"] for c in body["cases"]]
    assert standing_reason(source) is None
    assert numerical_use_standing(source, bases_, context) != "numerically_eligible"


@pytest.mark.parametrize("stem", ["mixed", "mixed_units"])
def test_physics_source_with_ordinary_cases_stays_admitted(stem):
    from core.analysis_runs.physics_source import validate_physics_source
    folder = PROJECT / "fixtures/product_preview/physics_source"
    source = json.loads((folder / f"{stem}-sparse_interactive.raw.json").read_text(), parse_int=lambda t: -0.0 if t == "-0" else int(t))
    request = json.loads((folder / f"{stem}.request.json").read_text(), parse_int=lambda t: -0.0 if t == "-0" else int(t))
    context = {"request": request, "solver_mode": "sparse_interactive"}
    assert standing_reason(source) is None
    expected = "numerically_eligible" if validate_physics_source(source, context) else "needs_recompute"
    bases_ = [c["basis_ref"] for c in source["source_block_recovery"]["body"]["cases"]]
    assert numerical_use_standing(source, bases_, context) == expected


def test_rule_binding_refuses_all_selected_source_blocks_summary_only():
    source = json.loads((PROJECT / "fixtures/product_preview/source_blocks/n05-sparse_interactive.raw.json").read_text())
    summary_row = next(row for row in source["results"] if row["kind"] == "open_formula_stress_summary")
    headline = next(row for row in source["results"] if row["id"] == source["summary"]["max_open_formula_stress"]["result_ref"])
    force = next(row for row in source["results"] if row["kind"] == "element_local_axial_force")
    assert rule_binding_refusal(source, summary_row) == "RULE_SOURCE_BLOCKS_SUMMARY_NOT_RELIABLE"
    assert rule_binding_refusal(source, headline) == "RULE_SOURCE_BLOCKS_SUMMARY_NOT_RELIABLE"
    assert rule_binding_refusal(source, force) is None
    composite = json.loads((PROJECT / "fixtures/product_preview/physics_source/n05-sparse_interactive.raw.json").read_text())
    assert all(rule_binding_refusal(composite, row) is None for row in composite["results"])


# ---- stress-neutral packaging (N-A: no new document or manifest field) ----

def test_preview_stress_neutral_package_roundtrips_without_new_fields(tmp_path):
    from core.handoff.stress_neutral import package_v0_3 as sn
    from test_stress_neutral_physics_source import arguments
    source = envelope()
    # One arc member row exercises the arc vocabulary in the packaged annotations.
    source["results"].append({"id": "result:force:pipe:P1:end-i:axial-arc", "kind": "element_local_shear_force_y", "value": 2.5, "unit": "N", "entity_ref": "pipe:P1", "basis_ref": lc("load:A"), "metadata": meta("shear_force_y", "arc_chord_frame", "end_i", "recovered_from_local_element_stiffness")})
    source["results"].append({"id": "result:stress:pipe:P1:end-i:axial-arc", "kind": "element_local_axial_normal_stress", "value": 0.5, "unit": "MPa", "entity_ref": "pipe:P1", "basis_ref": lc("load:A"), "metadata": meta("axial_normal_stress", "pipe_section", "end_i", "nominal_straight_beam_formula_on_arc_resultants")})
    validate_preview_physics_evidence(source)
    analysis = build(source)
    packet = sn.build_stress_neutral_export_package_v0_3(source_envelope=source, analysis_record=analysis, **arguments(source, analysis))
    assert packet["semantic_contract"] == {"id": PREVIEW_PHYSICS_CONTRACT_ID, "sha256": PREVIEW_PHYSICS_CONTRACT_SHA256}
    assert packet["contract_evidence"] == source["contract_evidence"]
    assert "source_block_recovery" not in packet
    validate_instance(json.loads((PROJECT / "schemas/stress_neutral_export.schema.json").read_text()), packet, instance_label="preview stress-neutral package")
    physics_raw = json.loads((PROJECT / "fixtures/results/physics_connected_mechanics_sparse.json").read_text())
    physics_analysis = build(physics_raw)
    physics_packet = sn.build_stress_neutral_export_package_v0_3(source_envelope=physics_raw, analysis_record=physics_analysis, **arguments(physics_raw, physics_analysis))
    assert set(packet) == set(physics_packet)
    assert set(packet["manifest"]) == set(physics_packet["manifest"])
    assert packet["manifest"]["boundary_notes"] == physics_packet["manifest"]["boundary_notes"]
    written = sn.write_materialized_members_v0_3(packet, tmp_path)
    assert len(written) == 9
    assert sn.read_materialized_members_v0_3(tmp_path, source_envelope=source, analysis_record=analysis) == packet
    arc = next(item for item in packet["source_annotations"] if item["source_result_id"] == "result:stress:pipe:P1:end-i:axial-arc")
    assert arc["source_row"]["metadata"]["basis"] == "nominal_straight_beam_formula_on_arc_resultants"
    witness = {row["result_id"] for row in packet["unit_preservation_witnesses"]}
    assert "result:stress:pipe:P1:end-i:axial-arc" in witness
    tampered = deepcopy(packet); tampered["contract_evidence"]["combination_gates"][0]["withheld"] = True
    with pytest.raises(ValueError):
        sn.validate_stress_neutral_export_package_v0_3(tampered)


# ---- actual producer fixtures ----

ACTUAL = [PROJECT / f"fixtures/results/preview_physics_{name}_{mode}.json" for name in ("connected", "invented") for mode in ("sparse", "dense")]


def actual(path):
    # R2 N8: the producer fixtures are committed; absence is a failure, not a skip.
    assert path.exists(), f"missing actual preview-physics-1 fixture: {path}"
    return json.loads(path.read_text())


@pytest.mark.parametrize("path", ACTUAL, ids=lambda p: p.stem)
def test_actual_producer_output_reads_records_and_packages(path):
    from core.handoff.stress_neutral import package_v0_3 as sn
    from test_stress_neutral_physics_source import arguments
    source = actual(path)
    original = deepcopy(source)
    assert source["producer"]["semantic_contract_id"] == PREVIEW_PHYSICS_CONTRACT_ID
    validate_preview_physics_evidence(source)
    record = build(source)
    validate_analysis_run_v0_3(record, source)
    assert verify_analysis_run_record(record) == "match"
    validate_instance(json.loads((PROJECT / "schemas/analysis_run.schema.json").read_text()), record, instance_label=path.stem)
    assert all(ref["interpretation"]["status"] != "unavailable" for ref in record["analysis_run"]["result_refs"] if ref["category"] == "physical_quantity")
    assert standing_reason(source) is None
    assert numerical_use_standing(source, bases(source)) == ("numerically_eligible" if source["numerical_quality"]["status"] == "checks_passed" else "needs_recompute")
    packet = sn.build_stress_neutral_export_package_v0_3(source_envelope=source, analysis_record=record, **arguments(source, record))
    validate_instance(json.loads((PROJECT / "schemas/stress_neutral_export.schema.json").read_text()), packet, instance_label=path.stem)
    assert source == original


@pytest.mark.parametrize("mode", ["sparse", "dense"])
def test_actual_invented_f1_positive_control(mode):
    """Hanger, nonlinear-loop and intensification diagnostics carry non-result
    references (``hanger``, ``DEC-046``, user SIF source strings); accepted."""
    source = actual(PROJECT / f"fixtures/results/preview_physics_invented_{mode}.json")
    codes = {item["code"]: item for item in source["diagnostics"]}
    assert "hanger" in codes["SPRING_HANGER_USER_DATA_REVIEWED"]["affected_refs"]
    assert "hanger" in codes["CONSTANT_EFFORT_USER_DATA_REVIEWED"]["affected_refs"]
    assert "DEC-046" in codes["NONLINEAR_SUPPORT_LOOP_CONVERGED"]["affected_refs"]
    intensification = [item for item in source["diagnostics"] if item["code"] == "COMPONENT_EQUAL_FACTOR_INTENSIFICATION_APPLIED"]
    sif_sources = {m["sif_source_reference"] for case in source["contract_evidence"]["preview_cases"] for m in case["intensified_measures"]}
    assert intensification and sif_sources and all(sif_sources & set(item["affected_refs"]) for item in intensification)
    validate_preview_physics_evidence(source)
    dangling = deepcopy(source)
    codes = {item["code"]: item for item in dangling["diagnostics"]}
    codes["SPRING_HANGER_USER_DATA_REVIEWED"]["affected_refs"].append("result:reaction:support-SH-140")
    with pytest.raises(ValueError, match="dangling result reference"):
        validate_preview_physics_evidence(dangling)


def _actual_retired_row(s):
    s["results"].append({"id": "result:reaction:support-S-100", "kind": "reaction_resultant", "value": 1.0, "unit": "N", "entity_ref": "support:S-100", "basis_ref": s["results"][-1].get("basis_ref")})


def _actual_first_case_headline(s):
    first = s["contract_evidence"]["preview_cases"][0]["load_case_id"]
    rows = [r for r in s["results"] if r["kind"] == "pipe_elastic_normal_stress_maximum_v2" and r["basis_ref"]["ref_id"] == first]
    low = min(rows, key=lambda r: r["value"])
    s["summary"]["max_open_formula_stress"] = {"value": low["value"], "unit": "Pa", "location_ref": low["entity_ref"], "result_ref": low["id"]}


def _actual_support_norm(s):
    next(r for r in s["results"] if r["kind"] == "support_reaction_force_magnitude_v2")["value"] *= 1.01


def _actual_zero_filled(s):
    case = s["contract_evidence"]["preview_cases"][0]
    withheld = case["support_attribution"]["withheld"][0]["support_id"]
    template = [r for r in s["results"] if r["kind"].startswith("support_reaction_") and r["entity_ref"] == case["support_attribution"]["attributed_support_ids"][0] and r["basis_ref"]["ref_id"] == case["load_case_id"]]
    for row in template:
        s["results"].append({**deepcopy(row), "id": row["id"].replace(row["entity_ref"], withheld), "entity_ref": withheld, "value": 0.0})


def _actual_gated_combination_row(s):
    gate = s["contract_evidence"]["combination_gates"][0]
    row = deepcopy(next(r for r in s["results"] if r["kind"] == "global_nodal_displacement_x"))
    row.update(id="result:combination:published", basis_ref={"ref_type": "combination", "ref_id": gate["combination_id"]})
    s["results"].append(row)


def _actual_intensified_k(s):
    next(r for r in s["results"] if r["kind"] == "component_equal_factor_intensified_bending_stress_v1")["value"] *= 1.08


@pytest.mark.parametrize("mode", ["sparse", "dense"])
@pytest.mark.parametrize("change", [_actual_retired_row, _actual_first_case_headline, _actual_support_norm, _actual_zero_filled, _actual_gated_combination_row, _actual_intensified_k], ids=lambda f: f.__name__.strip("_"))
def test_actual_invented_output_tamper_controls(mode, change):
    source = actual(PROJECT / f"fixtures/results/preview_physics_invented_{mode}.json")
    change(source)
    with pytest.raises(ValueError, match="SOURCE_PREVIEW_PHYSICS_"):
        _source_contract(source)
    assert numerical_use_standing(source, bases(source)) == "unsupported"


@pytest.mark.parametrize("mode", ["sparse", "dense"])
def test_generated_unchanged_demo_output_is_recorded_truthfully(mode):
    """The recipe's preview mode captures the unchanged demo model; whatever
    it records (a blocked envelope for legacy nonzero pressure) is read as is."""
    source = actual(PROJECT / f"fixtures/product_preview/invented_mechanics_result_preview_physics_1_{mode}.json")
    record = json.loads((PROJECT / "fixtures/product_preview/preview_physics_fixture_generation.json").read_text())
    output = next(item for item in record["outputs"] if item["path"].endswith(f"_{mode}.json"))
    path = PROJECT / output["path"]
    assert hashlib.sha256(path.read_bytes()).hexdigest() == output["sha256"]
    assert output["mechanics_status"] == source["status"]["mechanics"]
    validate_preview_physics_evidence(source)
    assert verify_analysis_run_record(build(source)) == "match"
    if source["status"]["mechanics"] != "MECHANICS_SOLVED":
        assert source["contract_evidence"] == {"preview_cases": [], "combination_gates": []}
        assert numerical_use_standing(source, bases(source)) == "needs_recompute"
    precision = json.loads((PROJECT / "fixtures/product_preview/precision_fixture_generation.json").read_text())
    for item in precision["outputs"]:
        assert hashlib.sha256((PROJECT / item["path"]).read_bytes()).hexdigest() == item["sha256"]
    preserved = {item["path"]: item["sha256"] for item in record["historical_precision_fixtures_preserved"]}
    for name, digest in preserved.items():
        assert hashlib.sha256((PROJECT / name).read_bytes()).hexdigest() == digest


def test_shared_unicode_id_vector_is_admitted_and_byte_lengths_are_required():
    """Shared id vector (T0R S6): the same actual producer bytes the Rust and TS
    readers admit. Row ids use UTF-8 byte lengths with ':' between segments;
    diagnostic ids use the concatenated ID() form; character counts are refused."""
    text = (PROJECT / "fixtures/results/preview_physics_unicode_ids_sparse.json").read_text(encoding="utf-8")
    source = json.loads(text)
    validate_preview_physics_evidence(source)
    ids = {row["id"] for row in source["results"]}
    for expected in (
        "result:support-action:7:load:é:11:support:锚:Fx",
        "result:elastic-maximum:7:load:é:10:pipe:α-β",
        "result:elastic-maximum:9:load:𝔫:10:pipe:β-γ",
        "result:intensified-bending:component-ç:pipe-α-β:end-j",
        "result:loadcase:load-𝔫:intensified-bending:component-ç:pipe-β-γ:end-i",
    ):
        assert expected in ids, expected
    assert "diagnostic:preview-physics:constant-effort-not-consumed:13:support:ü-ce" in {d["id"] for d in source["diagnostics"]}
    for good, bad in (
        ("result:elastic-maximum:7:load:é:", "result:elastic-maximum:6:load:é:"),
        ("constant-effort-not-consumed:13:support:ü-ce", "constant-effort-not-consumed:12:support:ü-ce"),
    ):
        with pytest.raises(ValueError):
            validate_preview_physics_evidence(json.loads(text.replace(good, bad)))
