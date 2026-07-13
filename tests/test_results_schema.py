#!/usr/bin/env python3
"""Stdlib checks for the result export schema."""

import json
from copy import deepcopy
from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[1]
TESTS_DIR = Path(__file__).resolve().parent
if str(TESTS_DIR) not in sys.path:
    sys.path.insert(0, str(TESTS_DIR))

from schema_validation import (  # noqa: E402
    JsonSchemaDependencyMissing,
    schema_for_definition,
    validate_instance,
    validate_schema_document,
)

SCHEMA_PATH = ROOT / "schemas" / "results.schema.yaml"
PRODUCT_PREVIEW_RESULT_PATH = (
    ROOT / "fixtures" / "product_preview" / "invented_mechanics_result.json"
)
TP_PHYS_015_RESULT_ENVELOPE_PATH = (
    ROOT
    / "fixtures"
    / "results"
    / "invented"
    / "tp_phys_015_canonical_solve_result_envelope.json"
)
TP_PHYS_015_SECTION_EVIDENCE_ENVELOPE_PATH = (
    ROOT
    / "fixtures"
    / "results"
    / "invented"
    / "tp_phys_015_section_property_stress_evidence_envelope.json"
)

REQUIRED_ROOT = {
    "schema_version",
    "deliverable_id",
    "package_id",
    "scope_item",
    "objectives",
    "export_format_status",
    "result_envelope",
}

REQUIRED_DEFS = {
    "AnalysisStatus",
    "Checksum",
    "Diagnostic",
    "DimensionId",
    "DownstreamUse",
    "ExportFormatStatus",
    "ProfessionalBoundary",
    "Provenance",
    "QuantityResult",
    "Reference",
    "Reproducibility",
    "ResultEnvelope",
    "ResultFamily",
    "ResultMetadata",
    "ResultSet",
    "ResultSetType",
    "ResultTraceLink",
    "RulePackRef",
    "SolverVersion",
    "UnitPreservationQuantity",
    "UnitPreservationWitness",
}

REQUIRED_FAMILIES = {
    "displacement",
    "rotation",
    "force",
    "moment",
    "reaction",
    "stress",
    "section_property",
    "ratio",
    "rule_check",
}

REQUIRED_DIAGNOSTIC_CLASSES = {
    "SOLVE_BLOCKING",
    "RULE_CHECK_BLOCKING",
    "PROVENANCE_WARNING",
    "ASSUMPTION_WARNING",
    "NONLINEAR_WARNING",
    "IP_BOUNDARY_WARNING",
    "UNIT_WARNING",
    "EXPORT_BLOCKING",
}

FORBIDDEN_STATUS = {
    "HUMAN_APPROVED_FOR_PROJECT",
    "CODE_COMPLIANT",
    "CERTIFIED",
    "SEALED",
    "APPROVED",
}

FORBIDDEN_FORMAT_COMMITMENTS = {
    "csv",
    "spreadsheet",
    "hdf5",
    "local_fea",
    "openapi_transport",
}


def load_schema():
    with SCHEMA_PATH.open(encoding="utf-8") as schema_file:
        return json.load(schema_file)


def load_product_preview_result():
    with PRODUCT_PREVIEW_RESULT_PATH.open(encoding="utf-8") as fixture_file:
        return json.load(fixture_file)


def load_tp_phys_015_result_envelope():
    with TP_PHYS_015_RESULT_ENVELOPE_PATH.open(encoding="utf-8") as fixture_file:
        return json.load(fixture_file)


def load_tp_phys_015_section_evidence_envelope():
    with TP_PHYS_015_SECTION_EVIDENCE_ENVELOPE_PATH.open(
        encoding="utf-8"
    ) as fixture_file:
        return json.load(fixture_file)


def required_at(schema, definition_name):
    return set(schema["$defs"][definition_name]["required"])


def enum_at(schema, definition_name):
    return set(schema["$defs"][definition_name]["enum"])


def walk_keys(value):
    if isinstance(value, dict):
        for key, item in value.items():
            yield key
            yield from walk_keys(item)
    elif isinstance(value, list):
        for item in value:
            yield from walk_keys(item)


def main():
    schema = load_schema()
    defs = schema["$defs"]

    assert schema["$schema"] == "https://json-schema.org/draft/2020-12/schema"
    assert schema["additionalProperties"] is False
    assert "default" not in set(walk_keys(schema))
    assert REQUIRED_ROOT <= set(schema["required"])
    assert REQUIRED_DEFS <= set(defs)

    assert schema["properties"]["deliverable_id"]["const"] == "DEL-08-04"
    assert schema["properties"]["package_id"]["const"] == "PKG-08"
    assert schema["properties"]["scope_item"]["const"] == "SOW-046"
    assert {"OBJ-007", "OBJ-009"} <= set(
        schema["properties"]["objectives"]["items"]["enum"]
    )

    export_status = defs["ExportFormatStatus"]["properties"]
    assert export_status["baseline_format"]["const"] == (
        "schema_first_json_result_envelope"
    )
    assert export_status["additional_formats"]["const"] == "TBD"
    assert export_status["public_transport_protocol"]["const"] == "TBD"
    assert export_status["local_fea_package_format"]["const"] == "TBD"
    assert export_status["external_adapter_formats"]["const"] == "TBD"
    for name in FORBIDDEN_FORMAT_COMMITMENTS:
        assert name not in {
            export_status["additional_formats"]["const"].lower(),
            export_status["public_transport_protocol"]["const"].lower(),
            export_status["local_fea_package_format"]["const"].lower(),
            export_status["external_adapter_formats"]["const"].lower(),
        }

    envelope_required = required_at(schema, "ResultEnvelope")
    assert {
        "schema_version",
        "envelope_id",
        "model_ref",
        "run_ref",
        "solver_version",
        "unit_system_ref",
        "load_basis_refs",
        "result_sets",
        "diagnostics",
        "provenance",
        "reproducibility",
        "analysis_status",
        "professional_boundary",
        "downstream_use",
    } <= envelope_required

    status = enum_at(schema, "AnalysisStatus")
    assert {
        "MODEL_INCOMPLETE",
        "MECHANICS_SOLVED",
        "RULE_INPUTS_INCOMPLETE",
        "USER_RULE_CHECKED",
        "USER_RULE_FAILED",
        "HUMAN_REVIEW_REQUIRED",
    } <= status
    assert status.isdisjoint(FORBIDDEN_STATUS)
    assert (
        defs["ResultEnvelope"]["properties"]["analysis_status"]["contains"]["const"]
        == "HUMAN_REVIEW_REQUIRED"
    )

    quantity_required = required_at(schema, "QuantityResult")
    assert {
        "result_id",
        "family",
        "object_ref",
        "basis_ref",
        "magnitude",
        "unit",
        "dimension",
        "provenance",
    } <= quantity_required
    assert "metadata" not in quantity_required
    assert (
        defs["QuantityResult"]["properties"]["metadata"]["$ref"]
        == "#/$defs/ResultMetadata"
    )
    assert (
        defs["QuantityResult"]["properties"]["trace_chain"]["items"]["$ref"]
        == "#/$defs/ResultTraceLink"
    )
    assert {
        "trace_id",
        "trace_type",
        "source_ref",
        "target_ref",
        "provenance",
    } <= required_at(schema, "ResultTraceLink")
    assert {
        "physical_source_to_analytical_model",
        "analytical_model_to_adapter_dto",
        "adapter_dto_to_solver_input",
        "solver_input_to_result_value",
        "section_property_evidence_to_result_value",
        "source_trace_link_reference",
        "TBD",
    } <= set(defs["ResultTraceLink"]["properties"]["trace_type"]["enum"])
    assert (
        defs["ResultEnvelope"]["properties"]["unit_witness_policy"]["const"]
        == "preserve_source_result_value_unit_and_dimension_per_exported_result_row"
    )
    assert (
        defs["ResultEnvelope"]["properties"]["unit_preservation_witnesses"]["items"][
            "$ref"
        ]
        == "#/$defs/UnitPreservationWitness"
    )
    assert {
        "witness_id",
        "source_result_ref",
        "source_field_path",
        "source_quantity",
        "target_result_ref",
        "target_field_path",
        "target_quantity",
        "target_quantity_policy",
        "export_unit_policy",
        "conversion_performed",
        "unit_system_ref",
        "provenance",
    } <= required_at(schema, "UnitPreservationWitness")
    witness = defs["UnitPreservationWitness"]["properties"]
    assert (
        witness["target_quantity_policy"]["const"]
        == "exported_result_row_preserves_source_value_unit_and_dimension"
    )
    assert (
        witness["export_unit_policy"]["const"]
        == "preserve_source_result_unit_and_dimension"
    )
    assert witness["conversion_performed"]["const"] is False
    assert {
        "value",
        "unit",
        "dimension",
    } <= required_at(schema, "UnitPreservationQuantity")
    assert {
        "component",
        "coordinate_system",
        "location",
        "basis",
        "sign_convention",
    } <= required_at(schema, "ResultMetadata")
    metadata = defs["ResultMetadata"]["properties"]
    assert {
        "axial_force",
        "shear_force_y",
        "shear_force_z",
        "torsional_moment",
        "bending_moment_y",
        "bending_moment_z",
        "nodal_force_x",
        "nodal_force_y",
        "nodal_force_z",
        "nodal_moment_x",
        "nodal_moment_y",
        "nodal_moment_z",
        "axial_normal_stress",
        "bending_normal_stress_y",
        "bending_normal_stress_z",
        "torsional_shear_stress",
        "pressure_hoop_stress",
        "pressure_longitudinal_stress",
        "section_area",
        "section_modulus_y",
        "section_modulus_z",
        "torsion_constant",
        "torsion_radius",
    } <= set(metadata["component"]["enum"])
    assert {"element_local", "pipe_section"} <= set(metadata["coordinate_system"]["enum"])
    assert {"end_i", "end_j", "node", "quarter_1", "midspan", "quarter_3"} <= set(
        metadata["location"]["enum"]
    )
    assert (
        "recovered_from_local_element_stiffness"
        in set(metadata["basis"]["enum"])
    )
    assert (
        "recovered_from_open_mechanics_stress_components"
        in set(metadata["basis"]["enum"])
    )
    assert "assembled_solver_load_vector" in set(metadata["basis"]["enum"])
    assert "interpolated_from_endpoint_resultants" in set(metadata["basis"]["enum"])
    assert "derived_from_user_entered_section_geometry" in set(
        metadata["basis"]["enum"]
    )
    assert "explicit_user_linear_combination" in set(metadata["basis"]["enum"])
    quantity_condition = defs["QuantityResult"]["allOf"][0]
    assert set(quantity_condition["if"]["properties"]["family"]["enum"]) == {
        "force",
        "moment",
        "section_property",
    }
    assert "metadata" in quantity_condition["then"]["required"]
    assert {"area", "section_modulus", "second_moment_area"} <= enum_at(
        schema, "DimensionId"
    )
    assert REQUIRED_FAMILIES <= enum_at(schema, "ResultFamily")
    assert {
        "mechanics",
        "stress_recovery",
        "load_vector_evidence",
        "station_resultants",
        "section_property_evidence",
        "user_rule_check",
        "diagnostic_only",
    } <= enum_at(schema, "ResultSetType")
    assert (
        defs["ResultSet"]["properties"]["set_type"]["$ref"]
        == "#/$defs/ResultSetType"
    )

    preview_result = load_product_preview_result()
    axial_force = next(
        result
        for result in preview_result["results"]
        if result["id"] == "result:force:pipe-P-120:axial"
    )
    axial_force_end_j = next(
        result
        for result in preview_result["results"]
        if result["id"] == "result:force:pipe-P-120:axial:end-j"
    )
    axial_force_midspan = next(
        result
        for result in preview_result["results"]
        if result["id"] == "result:force:pipe-P-120:midspan:axial"
    )
    shear_force_end_i = next(
        result
        for result in preview_result["results"]
        if result["id"] == "result:force:pipe-P-120:shear-y"
    )
    shear_force_quarter = next(
        result
        for result in preview_result["results"]
        if result["id"] == "result:force:pipe-P-120:quarter-1:shear-z"
    )
    axial_metadata = axial_force["metadata"]
    axial_end_j_metadata = axial_force_end_j["metadata"]
    axial_midspan_metadata = axial_force_midspan["metadata"]
    torsional_stress_end_j = next(
        result
        for result in preview_result["results"]
        if result["id"] == "result:stress:pipe-P-120:end-j:torsional-shear"
    )
    torsional_stress_midspan = next(
        result
        for result in preview_result["results"]
        if result["id"] == "result:stress:pipe-P-120:midspan:torsional-shear"
    )
    combination_axial_force = next(
        result
        for result in preview_result["results"]
        if result["id"] == "result:combination:combination-C-OPER-ALT:force:pipe-P-120:axial"
    )
    pressure_hoop = next(
        result
        for result in preview_result["results"]
        if result["id"] == "result:stress:pipe-P-120:end-i:pressure-hoop"
    )
    stress_summary = next(
        result
        for result in preview_result["results"]
        if result["id"] == "result:stress:pipe-P-120"
    )
    assert axial_force["unit"] == "N"
    assert axial_force["basis_ref"] == {"ref_type": "load_case", "ref_id": "load:L-100"}
    assert axial_metadata["component"] in metadata["component"]["enum"]
    assert axial_metadata["coordinate_system"] in metadata["coordinate_system"]["enum"]
    assert axial_metadata["location"] in metadata["location"]["enum"]
    assert axial_metadata["basis"] in metadata["basis"]["enum"]
    assert axial_metadata["sign_convention"]
    assert axial_force_end_j["unit"] == "N"
    assert axial_end_j_metadata["component"] == "axial_force"
    assert axial_end_j_metadata["coordinate_system"] == "element_local"
    assert axial_end_j_metadata["location"] == "end_j"
    assert axial_end_j_metadata["basis"] in metadata["basis"]["enum"]
    assert "j-end" in axial_end_j_metadata["sign_convention"]
    assert axial_force_midspan["unit"] == "N"
    assert axial_midspan_metadata["component"] == "axial_force"
    assert axial_midspan_metadata["coordinate_system"] == "element_local"
    assert axial_midspan_metadata["location"] == "midspan"
    assert axial_midspan_metadata["basis"] == "interpolated_from_endpoint_resultants"
    assert shear_force_end_i["unit"] == "N"
    assert shear_force_end_i["metadata"]["component"] == "shear_force_y"
    assert shear_force_end_i["metadata"]["location"] == "end_i"
    assert shear_force_end_i["metadata"]["basis"] == "recovered_from_local_element_stiffness"
    assert shear_force_quarter["unit"] == "N"
    assert shear_force_quarter["metadata"]["component"] == "shear_force_z"
    assert shear_force_quarter["metadata"]["location"] == "quarter_1"
    assert shear_force_quarter["metadata"]["basis"] == "interpolated_from_endpoint_resultants"
    assert stress_summary["kind"] == "open_formula_stress_summary"
    assert "metadata" not in stress_summary
    assert torsional_stress_end_j["unit"] == "MPa"
    assert torsional_stress_end_j["metadata"]["component"] == "torsional_shear_stress"
    assert torsional_stress_end_j["metadata"]["coordinate_system"] == "element_local"
    assert torsional_stress_end_j["metadata"]["location"] == "end_j"
    assert (
        torsional_stress_end_j["metadata"]["basis"]
        == "recovered_from_open_mechanics_stress_components"
    )
    assert torsional_stress_midspan["unit"] == "MPa"
    assert torsional_stress_midspan["metadata"]["component"] == "torsional_shear_stress"
    assert torsional_stress_midspan["metadata"]["coordinate_system"] == "element_local"
    assert torsional_stress_midspan["metadata"]["location"] == "midspan"
    assert (
        torsional_stress_midspan["metadata"]["basis"]
        == "interpolated_from_endpoint_resultants"
    )
    assert pressure_hoop["unit"] == "MPa"
    assert pressure_hoop["metadata"]["component"] == "pressure_hoop_stress"
    assert pressure_hoop["metadata"]["coordinate_system"] == "pipe_section"
    assert combination_axial_force["unit"] == "N"
    assert combination_axial_force["basis_ref"] == {
        "ref_type": "combination",
        "ref_id": "combination:C-OPER-ALT",
    }
    assert combination_axial_force["source_result_refs"] == [
        "result:force:pipe-P-120:axial",
        "result:loadcase:load-L-200:force:pipe-P-120:axial",
    ]
    assert combination_axial_force["metadata"]["basis"] == "explicit_user_linear_combination"
    assert combination_axial_force["metadata"]["basis"] in metadata["basis"]["enum"]

    tp_phys_015_result = load_tp_phys_015_result_envelope()
    result_envelope = tp_phys_015_result["result_envelope"]
    assert (
        result_envelope["envelope_id"]
        == "MECH-TP-PHYS-015-CANONICAL-SOLVE-RESULT-ENVELOPE"
    )
    assert result_envelope["solver_version"]["solver_name"] == (
        "open_pipe_stress_validation_mechanics_benchmark"
    )
    assert "HUMAN_REVIEW_REQUIRED" in result_envelope["analysis_status"]
    result_sets = {
        result_set["set_type"]: result_set
        for result_set in result_envelope["result_sets"]
    }
    assert {"mechanics", "load_vector_evidence", "station_resultants"} <= set(
        result_sets
    )
    load_vector_value = result_sets["load_vector_evidence"]["values"][0]
    assert load_vector_value["metadata"]["component"] == "nodal_force_y"
    assert load_vector_value["metadata"]["basis"] == "assembled_solver_load_vector"
    assert load_vector_value["diagnostics"][0]["code"] == (
        "TP_RESULT_017_VALUE_TRACE_NOTE"
    )
    assert load_vector_value["trace_chain"][0]["trace_type"] == (
        "solver_input_to_result_value"
    )
    assert load_vector_value["trace_chain"][0]["source_ref"] == {
        "ref_type": "adapter_dto",
        "ref_id": "dto:load_application:LC-TP-PHYS-014:0",
    }
    assert load_vector_value["trace_chain"][0]["target_ref"] == {
        "ref_type": "result_value",
        "ref_id": "result:load-vector:node-N-1:uy",
    }
    station_value = result_sets["station_resultants"]["values"][0]
    assert station_value["station_ref"] == {
        "ref_type": "result_station",
        "ref_id": "E-1:midspan",
    }
    assert station_value["metadata"]["component"] == "bending_moment_z"

    section_result = load_tp_phys_015_section_evidence_envelope()["result_envelope"]
    section_result_sets = {
        result_set["set_type"]: result_set
        for result_set in section_result["result_sets"]
    }
    assert {"section_property_evidence", "stress_recovery"} <= set(
        section_result_sets
    )
    section_values = section_result_sets["section_property_evidence"]["values"]
    section_by_id = {value["result_id"]: value for value in section_values}
    assert (
        section_by_id["result:section-property:tp-stress-016:section-modulus-z"][
            "dimension"
        ]
        == "section_modulus"
    )
    assert (
        section_by_id["result:section-property:tp-stress-016:torsion-constant"][
            "dimension"
        ]
        == "second_moment_area"
    )
    for value in section_values:
        assert value["family"] == "section_property"
        assert value["object_ref"] == {
            "ref_type": "section_property_evidence",
            "ref_id": "SECTION-PROP-TP-STRESS-016-INVENTED-PIPE-OD2-WALL0P25",
        }
        assert value["metadata"]["basis"] == "derived_from_user_entered_section_geometry"
    stress_value = section_result_sets["stress_recovery"]["values"][0]
    assert stress_value["trace_chain"][0]["trace_type"] == (
        "section_property_evidence_to_result_value"
    )
    assert stress_value["trace_chain"][0]["source_ref"] == {
        "ref_type": "section_property_evidence",
        "ref_id": "SECTION-PROP-TP-STRESS-016-INVENTED-PIPE-OD2-WALL0P25",
    }

    diagnostic_required = required_at(schema, "Diagnostic")
    assert {
        "code",
        "class",
        "severity",
        "source",
        "affected_object",
        "message",
        "remediation",
        "provenance",
    } <= diagnostic_required
    assert REQUIRED_DIAGNOSTIC_CLASSES <= set(
        defs["Diagnostic"]["properties"]["class"]["enum"]
    )

    rule_pack_required = required_at(schema, "RulePackRef")
    assert {
        "rule_pack_id",
        "version",
        "checksum",
        "source_notice",
        "redistribution_status",
        "completeness_status",
        "private_payload_redacted",
    } <= rule_pack_required
    assert defs["RulePackRef"]["properties"]["private_payload_redacted"]["const"] is True

    boundary = defs["ProfessionalBoundary"]["properties"]
    assert boundary["human_review_required"]["const"] is True
    assert boundary["software_makes_compliance_claim"]["const"] is False
    assert boundary["software_makes_certification_claim"]["const"] is False
    assert boundary["software_makes_sealing_claim"]["const"] is False
    assert boundary["software_makes_approval_claim"]["const"] is False
    assert boundary["software_makes_authentication_claim"]["const"] is False

    downstream = defs["DownstreamUse"]["properties"]
    assert downstream["review"]["const"] is True
    assert downstream["regression_comparison"]["const"] is True
    assert downstream["report_consumption"]["const"] is True
    assert downstream["headless_automation"]["const"] is True
    assert downstream["governed_downstream_tooling"]["const"] is True
    assert downstream["additional_export_formats"]["const"] == "TBD"


def check_jsonschema_validation():
    schema = load_schema()
    fixture = load_tp_phys_015_result_envelope()
    section_fixture = load_tp_phys_015_section_evidence_envelope()
    try:
        assert validate_schema_document(schema, schema_label=str(SCHEMA_PATH))
        assert validate_instance(
            schema,
            fixture,
            schema_label=str(SCHEMA_PATH),
            instance_label=str(TP_PHYS_015_RESULT_ENVELOPE_PATH),
        )
        assert validate_instance(
            schema,
            section_fixture,
            schema_label=str(SCHEMA_PATH),
            instance_label=str(TP_PHYS_015_SECTION_EVIDENCE_ENVELOPE_PATH),
        )
    except JsonSchemaDependencyMissing as exc:
        _skip_or_note_missing_jsonschema(exc)


def _skip_or_note_missing_jsonschema(exc):
    if "pytest" in sys.modules:
        import pytest

        pytest.skip(str(exc))
    print(f"SKIP: {exc}")


def test_results_schema_contract():
    main()


def test_results_schema_jsonschema_validation_helper():
    check_jsonschema_validation()


def test_result_trace_link_field_scalar_paths_are_paired():
    schema = load_schema()
    definition = schema["$defs"]["ResultTraceLink"]
    assert {"source_field_path", "target_field_path"} <= set(
        definition["properties"]
    )
    assert definition["dependentRequired"] == {
        "source_field_path": ["target_field_path"],
        "target_field_path": ["source_field_path"],
    }

    envelope = load_tp_phys_015_section_evidence_envelope()["result_envelope"]
    link = deepcopy(envelope["result_sets"][1]["values"][0]["trace_chain"][0])
    link["source_field_path"] = (
        "result_sets[set_type=section_property_evidence].values"
        "[result_id=result:section-property:tp-stress-016:section-modulus-z].magnitude"
    )
    link["target_field_path"] = (
        "result_sets[set_type=stress_recovery].values"
        "[result_id=result:stress:element-E-1:midspan:bending-normal-z].magnitude"
    )
    link_schema = schema_for_definition(schema, "ResultTraceLink")
    assert validate_instance(link_schema, link)

    unpaired = deepcopy(link)
    del unpaired["target_field_path"]
    try:
        validate_instance(link_schema, unpaired)
    except AssertionError as exc:
        assert "target_field_path" in str(exc)
    else:
        raise AssertionError("unpaired result field-scalar trace path must be rejected")


if __name__ == "__main__":
    main()
    check_jsonschema_validation()
