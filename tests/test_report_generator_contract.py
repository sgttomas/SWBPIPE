#!/usr/bin/env python3
"""Stdlib checks for the calculation report generator schema and fixture."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "schemas" / "report_generator.schema.yaml"
FIXTURE_PATH = ROOT / "fixtures" / "reports" / "invented" / "calculation_report_fixture.json"

REQUIRED_ROOT = {
    "schema_version",
    "deliverable_id",
    "package_id",
    "scope_item",
    "objectives",
    "report_generator_status",
    "calculation_report",
}

REQUIRED_DEFS = {
    "AnalysisStatus",
    "CalculationReport",
    "Checksum",
    "Diagnostic",
    "DimensionId",
    "LoadCaseSummary",
    "ModelInputSummary",
    "PrivacyClassification",
    "ProfessionalBoundary",
    "Provenance",
    "Reference",
    "ReferencedEnvelope",
    "RenderedSection",
    "ReportGeneratorStatus",
    "RulePackRef",
    "TemplateSlot",
    "UnresolvedTbd",
}

REQUIRED_SECTION_KINDS = {
    "model_input_summary",
    "load_cases",
    "results",
    "warnings_assumptions_provenance",
    "audit_manifest",
    "rule_pack_references",
    "limitations",
    "professional_boundary_notice",
}

FORBIDDEN_STATUS = {
    "HUMAN_APPROVED_FOR_PROJECT",
    "CODE_COMPLIANT",
    "CERTIFIED",
    "SEALED",
    "APPROVED",
}

FORBIDDEN_RUNTIME_COMMITMENTS = {
    "gui_presentation",
    "cli_runtime",
    "api_transport",
    "adapter_behavior",
    "protected_content_linter",
    "private_redaction_export_controls",
}


def load_json(path):
    with path.open(encoding="utf-8") as json_file:
        return json.load(json_file)


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
    schema = load_json(SCHEMA_PATH)
    fixture = load_json(FIXTURE_PATH)
    defs = schema["$defs"]

    assert schema["$schema"] == "https://json-schema.org/draft/2020-12/schema"
    assert schema["additionalProperties"] is False
    assert "default" not in set(walk_keys(schema))
    assert REQUIRED_ROOT <= set(schema["required"])
    assert REQUIRED_DEFS <= set(defs)

    assert schema["properties"]["deliverable_id"]["const"] == "DEL-08-01"
    assert schema["properties"]["package_id"]["const"] == "PKG-08"
    assert schema["properties"]["scope_item"]["const"] == "SOW-024"
    assert schema["properties"]["objectives"]["contains"]["const"] == "OBJ-007"

    generator_status = defs["ReportGeneratorStatus"]["properties"]
    assert generator_status["baseline_renderer"]["const"] == (
        "deterministic_in_memory_report_assembly"
    )
    assert generator_status["template_slot_contract"]["const"] == (
        "schema_first_template_slots"
    )
    assert generator_status["neutral_test_output"]["const"] == (
        "structured_text_sections"
    )
    for field in FORBIDDEN_RUNTIME_COMMITMENTS:
        assert generator_status[field]["const"] == "TBD"

    report_required = required_at(schema, "CalculationReport")
    assert {
        "model_input_summary",
        "load_case_summary",
        "result_export_refs",
        "audit_manifest_refs",
        "report_section_refs",
        "rule_pack_refs",
        "diagnostics",
        "template_slots",
        "rendered_sections",
        "analysis_status",
        "professional_boundary",
        "provenance",
        "privacy_classification",
        "unresolved_runtime_tbds",
    } <= report_required
    assert (
        defs["CalculationReport"]["properties"]["analysis_status"]["contains"][
            "const"
        ]
        == "HUMAN_REVIEW_REQUIRED"
    )

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

    model_required = required_at(schema, "ModelInputSummary")
    assert {
        "project_ref",
        "model_ref",
        "persistence_ref",
        "unit_system_ref",
        "model_hash",
        "input_manifest_ref",
        "provenance",
    } <= model_required

    referenced_required = required_at(schema, "ReferencedEnvelope")
    assert {
        "ref",
        "schema_ref",
        "checksum",
        "privacy_classification",
        "provenance",
    } <= referenced_required

    slot_required = required_at(schema, "TemplateSlot")
    assert {
        "slot_id",
        "required",
        "section_kind",
        "source_contract",
        "ordering_index",
    } <= slot_required
    slot_kinds = set(defs["TemplateSlot"]["properties"]["section_kind"]["enum"])
    rendered_kinds = set(defs["RenderedSection"]["properties"]["section_kind"]["enum"])
    assert REQUIRED_SECTION_KINDS <= slot_kinds
    assert REQUIRED_SECTION_KINDS <= rendered_kinds

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
    assert {"REPORT_BLOCKING", "TEMPLATE_BLOCKING"} <= set(
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

    tbd_topics = set(defs["UnresolvedTbd"]["properties"]["topic"]["enum"])
    assert {
        "gui_presentation",
        "cli_runtime",
        "api_transport",
        "adapter_behavior",
        "redaction_export_controls",
        "protected_content_linter",
        "release_template_integration",
        "final_report_styling_layout_policy",
    } <= tbd_topics
    assert defs["UnresolvedTbd"]["properties"]["review_needed"]["const"] is True

    assert fixture["deliverable_id"] == "DEL-08-01"
    report = fixture["calculation_report"]
    slot_order = [slot["ordering_index"] for slot in report["template_slots"]]
    assert slot_order == sorted(slot_order)
    assert REQUIRED_SECTION_KINDS <= {
        section["section_kind"] for section in report["rendered_sections"]
    }
    assert "HUMAN_REVIEW_REQUIRED" in report["analysis_status"]
    assert report["professional_boundary"]["software_makes_compliance_claim"] is False
    assert report["rule_pack_refs"][0]["private_payload_redacted"] is True
    assert report["privacy_classification"] == "invented_public_example"
    assert any(
        tbd["topic"] == "redaction_export_controls"
        for tbd in report["unresolved_runtime_tbds"]
    )


if __name__ == "__main__":
    main()
