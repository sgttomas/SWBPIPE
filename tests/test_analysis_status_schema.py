#!/usr/bin/env python3
"""Stdlib checks for the analysis status schema."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "schemas" / "analysis_status.schema.yaml"

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
    "CODE_COMPLIANT",
    "CERTIFIED",
    "SEALED",
    "APPROVED",
    "HUMAN_APPROVED_FOR_PROJECT",
}


def load_schema():
    with SCHEMA_PATH.open(encoding="utf-8") as schema_file:
        return json.load(schema_file)


def enum_at(schema, definition_name):
    return set(schema["$defs"][definition_name]["enum"])


def main():
    schema = load_schema()
    defs = schema["$defs"]

    vocabulary = enum_at(schema, "AnalysisStatusVocabulary")
    assert REQUIRED_VOCABULARY <= vocabulary

    automatic = enum_at(schema, "AutomaticAnalysisStatus")
    assert REQUIRED_AUTOMATIC <= automatic
    assert automatic.isdisjoint(FORBIDDEN_AUTOMATIC)
    for status in automatic:
        assert not any(forbidden in status for forbidden in FORBIDDEN_AUTOMATIC)

    software_status = defs["SoftwareStatusRecord"]
    assert software_status["properties"]["primary_status"]["$ref"].endswith(
        "/AutomaticAnalysisStatus"
    )
    assert software_status["properties"]["status_set"]["items"]["$ref"].endswith(
        "/AutomaticAnalysisStatus"
    )

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


if __name__ == "__main__":
    main()
