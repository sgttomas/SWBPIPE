#!/usr/bin/env python3
"""Stdlib checks for the analysis status schema."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "schemas" / "analysis_status.schema.yaml"
ANALYSIS_BOUNDARY_SCHEMA_PATH = ROOT / "schemas" / "analysis_boundary.schema.yaml"
RESULTS_SCHEMA_PATH = ROOT / "schemas" / "results.schema.yaml"
API_BOUNDARY_CONTRACT_PATH = ROOT / "api" / "api_boundary_contract.yaml"

REQUIRED_VOCABULARY = {
    "MODEL_INCOMPLETE",
    "MECHANICS_SOLVED",
    "RULE_INPUTS_INCOMPLETE",
    "USER_RULE_CHECKED",
    "USER_RULE_FAILED",
    "HUMAN_REVIEW_REQUIRED",
    "HUMAN_APPROVED_FOR_PROJECT",
}

REQUIRED_AUTOMATIC = REQUIRED_VOCABULARY - {"HUMAN_APPROVED_FOR_PROJECT"}

FORBIDDEN_AUTOMATIC = {
    "USER_RULE_PASSED",
    "CODE_COMPLIANT",
    "CODE_COMPLIANCE",
    "COMPLIANT",
    "CERTIFIED",
    "CERTIFICATION",
    "SEALED",
    "SEALING",
    "APPROVED",
    "APPROVAL",
    "HUMAN_APPROVED_FOR_PROJECT",
    "PROFESSIONAL_ACCEPTANCE",
    "PROFESSIONAL_RELIANCE",
}


def load_schema():
    with SCHEMA_PATH.open(encoding="utf-8") as schema_file:
        return json.load(schema_file)


def load_json(path):
    with path.open(encoding="utf-8") as json_file:
        return json.load(json_file)


def enum_at(schema, definition_name):
    return set(schema["$defs"][definition_name]["enum"])


def assert_no_forbidden_automatic_status_claims(statuses):
    assert "USER_RULE_PASSED" not in statuses
    assert "HUMAN_APPROVED_FOR_PROJECT" not in statuses
    for status in statuses:
        assert not any(forbidden in status for forbidden in FORBIDDEN_AUTOMATIC)


def check_schema_contract():
    schema = load_schema()
    defs = schema["$defs"]

    vocabulary = enum_at(schema, "AnalysisStatusVocabulary")
    assert REQUIRED_VOCABULARY <= vocabulary
    assert "USER_RULE_PASSED" not in vocabulary

    automatic = enum_at(schema, "AutomaticAnalysisStatus")
    assert REQUIRED_AUTOMATIC <= automatic
    assert_no_forbidden_automatic_status_claims(automatic)

    software_status = defs["SoftwareStatusRecord"]
    assert software_status["properties"]["primary_status"]["$ref"].endswith(
        "/AutomaticAnalysisStatus"
    )
    assert software_status["properties"]["status_set"]["items"]["$ref"].endswith(
        "/AutomaticAnalysisStatus"
    )
    assert software_status["properties"]["rule_check_details"]["items"]["$ref"].endswith(
        "/UserRuleCheckDetails"
    )
    user_rule_checked_condition = next(
        condition
        for condition in software_status["allOf"]
        if condition["if"]["properties"].get("primary_status", {}).get("const")
        == "USER_RULE_CHECKED"
    )
    assert "rule_check_details" in user_rule_checked_condition["then"]["required"]
    assert user_rule_checked_condition["then"]["properties"]["rule_check_details"][
        "minItems"
    ] == 1
    user_rule_checked_set_condition = next(
        condition
        for condition in software_status["allOf"]
        if condition["if"]["properties"].get("status_set", {}).get("contains", {}).get(
            "const"
        )
        == "USER_RULE_CHECKED"
    )
    assert (
        user_rule_checked_set_condition["if"]["properties"]["status_set"]["contains"][
            "const"
        ]
        == "USER_RULE_CHECKED"
    )
    assert "rule_check_details" in user_rule_checked_set_condition["then"]["required"]
    assert user_rule_checked_set_condition["then"]["properties"]["rule_check_details"][
        "minItems"
    ] == 1

    rule_check_details = defs["UserRuleCheckDetails"]
    assert {
        "rule_pack_ref",
        "evaluation_outcome",
        "checked_at",
        "evidence_refs",
    } <= set(rule_check_details["required"])
    detail_outcomes = set(rule_check_details["properties"]["evaluation_outcome"]["enum"])
    assert "checked_no_failures_reported" in detail_outcomes
    assert "USER_RULE_PASSED" not in detail_outcomes
    assert "accepted_for_project" not in detail_outcomes

    root_properties = schema["properties"]
    assert "human_acceptance_records" in root_properties
    human_record_ref = root_properties["human_acceptance_records"]["items"]["$ref"]
    assert human_record_ref.endswith("/HumanAcceptanceRecord")

    human_record = defs["HumanAcceptanceRecord"]
    assert "acceptance_outcome" in human_record["required"]
    assert "acceptance_status" not in human_record["required"]
    assert human_record["properties"]["acceptance_status"]["const"] == (
        "HUMAN_APPROVED_FOR_PROJECT"
    )
    assert "bound_hashes" in human_record["required"]
    assert "software_status" not in human_record["properties"]
    human_actor_constraints = human_record["properties"]["human_actor"]["allOf"]
    assert human_actor_constraints[0]["$ref"].endswith("/Actor")
    assert human_actor_constraints[1]["properties"]["actor_type"]["const"] == "human"
    accepted_branch = human_record["allOf"][0]
    assert "acceptance_status" in accepted_branch["then"]["required"]
    assert accepted_branch["else"]["not"]["required"] == ["acceptance_status"]

    professional_boundary = defs["ProfessionalBoundary"]["properties"]
    assert professional_boundary["software_makes_compliance_claim"]["const"] is False
    assert professional_boundary["software_makes_certification_claim"]["const"] is False
    assert professional_boundary["software_makes_sealing_claim"]["const"] is False
    assert professional_boundary["software_makes_approval_claim"]["const"] is False
    assert professional_boundary["software_makes_authentication_claim"]["const"] is False
    assert (
        professional_boundary["software_makes_professional_acceptance_claim"]["const"]
        is False
    )


def check_downstream_automatic_status_surfaces():
    analysis_boundary = load_json(ANALYSIS_BOUNDARY_SCHEMA_PATH)
    results_schema = load_json(RESULTS_SCHEMA_PATH)
    api_contract = load_json(API_BOUNDARY_CONTRACT_PATH)

    downstream_status_enums = {
        "analysis_boundary.AutomaticAnalysisStatus": enum_at(
            analysis_boundary, "AutomaticAnalysisStatus"
        ),
        "analysis_boundary.UserRuleCheckBoundary.status": set(
            analysis_boundary["$defs"]["UserRuleCheckBoundary"]["properties"]["status"][
                "enum"
            ]
        ),
        "results.AnalysisStatus": enum_at(results_schema, "AnalysisStatus"),
        "api.result_envelope.analysis_status": set(
            api_contract["$defs"]["result_envelope"]["properties"]["analysis_status"][
                "items"
            ]["enum"]
        ),
    }

    for statuses in downstream_status_enums.values():
        assert_no_forbidden_automatic_status_claims(statuses)


def test_analysis_status_schema_contract():
    check_schema_contract()


def test_downstream_automatic_status_surfaces_exclude_human_approval_and_passed_claims():
    check_downstream_automatic_status_surfaces()


def main():
    check_schema_contract()
    check_downstream_automatic_status_surfaces()


if __name__ == "__main__":
    main()
