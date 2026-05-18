#!/usr/bin/env python3
"""Stdlib checks for the report sections schema."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "schemas" / "report_sections.schema.yaml"

REQUIRED_ROOT = {
    "schema_version",
    "deliverable_id",
    "package_id",
    "scope_item",
    "objectives",
    "report_renderer_status",
    "report_sections",
}

REQUIRED_DEFS = {
    "AnalysisStatus",
    "AnalysisStatusDisclosure",
    "Assumption",
    "Diagnostic",
    "DimensionId",
    "Limitation",
    "PrivacyClassification",
    "ProfessionalBoundary",
    "Provenance",
    "Reference",
    "ReportEffect",
    "ReportRendererStatus",
    "ReportSectionEnvelope",
    "UnresolvedTbd",
    "UserSuppliedValue",
    "ValueQuantity",
}

REQUIRED_DIAGNOSTIC_CLASSES = {
    "SOLVE_BLOCKING",
    "RULE_CHECK_BLOCKING",
    "PROVENANCE_WARNING",
    "ASSUMPTION_WARNING",
    "NONLINEAR_WARNING",
    "IP_BOUNDARY_WARNING",
    "UNIT_WARNING",
    "REPORT_BLOCKING",
}

REQUIRED_DISCLOSURE_CATEGORIES = {
    "diagnostics",
    "analysis_status_disclosures",
    "provenance_notes",
    "user_supplied_values",
    "assumptions",
    "limitations",
    "unresolved_tbds",
    "professional_boundary",
}

FORBIDDEN_STATUS = {
    "HUMAN_APPROVED_FOR_PROJECT",
    "CODE_COMPLIANT",
    "CERTIFIED",
    "SEALED",
    "APPROVED",
}


def load_schema():
    with SCHEMA_PATH.open(encoding="utf-8") as schema_file:
        return json.load(schema_file)


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

    assert schema["properties"]["deliverable_id"]["const"] == "DEL-08-03"
    assert schema["properties"]["package_id"]["const"] == "PKG-08"
    assert schema["properties"]["scope_item"]["const"] == "SOW-024"
    assert {"OBJ-007", "OBJ-011"} <= set(
        schema["properties"]["objectives"]["items"]["enum"]
    )

    renderer = defs["ReportRendererStatus"]["properties"]
    assert renderer["report_section_contract"]["const"] == (
        "schema_first_report_section_records"
    )
    assert renderer["full_report_renderer"]["const"] == "TBD"
    assert renderer["final_template_layout"]["const"] == "TBD"
    assert renderer["gui_presentation"]["const"] == "TBD"
    assert renderer["cli_runtime"]["const"] == "TBD"
    assert renderer["api_transport"]["const"] == "TBD"
    assert renderer["adapter_behavior"]["const"] == "TBD"
    assert renderer["private_redaction_export_controls"]["const"] == "TBD"

    section_required = required_at(schema, "ReportSectionEnvelope")
    assert REQUIRED_DISCLOSURE_CATEGORIES <= section_required
    assert {
        "report_section_id",
        "model_ref",
        "run_ref",
    } <= section_required
    assert (
        defs["ReportSectionEnvelope"]["properties"]["analysis_status_disclosures"][
            "contains"
        ]["properties"]["status"]["const"]
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

    provenance_required = required_at(schema, "Provenance")
    assert {
        "source_name",
        "source_location",
        "source_license",
        "contributor",
        "contributor_certification",
        "redistribution_status",
        "review_status",
        "privacy_classification",
    } <= provenance_required
    assert {
        "public_metadata",
        "invented_public_example",
        "private_project_data",
        "private_rule_pack_data",
        "protected_suspected",
        "redacted",
    } <= enum_at(schema, "PrivacyClassification")

    user_value_required = required_at(schema, "UserSuppliedValue")
    assert {
        "value_id",
        "value_category",
        "source",
        "provenance",
        "privacy_classification",
        "required_for",
        "review_status",
        "missing_data_finding",
    } <= user_value_required
    assert {
        "mechanics_solve",
        "user_rule_check",
        "reporting",
        "human_review",
    } <= set(
        defs["UserSuppliedValue"]["properties"]["required_for"]["items"]["enum"]
    )

    quantity_required = required_at(schema, "ValueQuantity")
    assert {"magnitude", "unit", "dimension"} <= quantity_required
    assert {
        "dimensionless",
        "length",
        "force",
        "moment",
        "stress",
        "ratio",
        "TBD",
    } <= enum_at(schema, "DimensionId")

    assumption_required = required_at(schema, "Assumption")
    assert {
        "assumption_id",
        "owner",
        "source",
        "affected_scope",
        "statement",
        "review_status",
        "effect",
        "provenance",
    } <= assumption_required
    limitation_required = required_at(schema, "Limitation")
    assert {
        "limitation_id",
        "source",
        "affected_scope",
        "statement",
        "effect",
        "provenance",
    } <= limitation_required

    effect_required = required_at(schema, "ReportEffect")
    assert {
        "mechanics_solve",
        "user_rule_check",
        "report_completeness",
        "human_review",
    } <= effect_required

    tbd_required = required_at(schema, "UnresolvedTbd")
    assert {"tbd_id", "affected_scope", "description", "review_needed"} <= tbd_required
    assert defs["UnresolvedTbd"]["properties"]["review_needed"]["const"] is True

    boundary = defs["ProfessionalBoundary"]["properties"]
    assert boundary["human_review_required"]["const"] is True
    assert boundary["software_makes_compliance_claim"]["const"] is False
    assert boundary["software_makes_certification_claim"]["const"] is False
    assert boundary["software_makes_sealing_claim"]["const"] is False
    assert boundary["software_makes_approval_claim"]["const"] is False
    assert boundary["software_makes_authentication_claim"]["const"] is False


if __name__ == "__main__":
    main()
