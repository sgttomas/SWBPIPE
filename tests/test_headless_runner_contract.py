#!/usr/bin/env python3
"""Stdlib checks for the headless runner contract."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "schemas" / "headless_runner.schema.yaml"

REQUIRED_ROOT = {
    "schema_version",
    "deliverable_id",
    "package_id",
    "scope_items",
    "objectives",
    "runner_status",
    "tbd_decisions",
    "request",
    "result",
}

REQUIRED_DEFS = {
    "AnalysisStatus",
    "ChecksumRef",
    "Diagnostic",
    "HeadlessRunnerRequest",
    "HeadlessRunnerResult",
    "JobState",
    "PhysicalProjectContainer",
    "PrivacyContext",
    "ProfessionalBoundary",
    "Provenance",
    "Reference",
    "RequestedOutput",
    "ResultEnvelopeRef",
    "RunnerOperation",
    "RunnerStatus",
    "TbdDecisions",
}

REQUIRED_OPERATIONS = {
    "solve",
    "validate_input",
    "export_results",
    "run_benchmark",
    "run_regression",
    "TBD",
}

REQUIRED_DIAGNOSTIC_CLASSES = {
    "SOLVE_BLOCKING",
    "RULE_CHECK_BLOCKING",
    "PROVENANCE_WARNING",
    "ASSUMPTION_WARNING",
    "NONLINEAR_WARNING",
    "IP_BOUNDARY_WARNING",
    "UNIT_WARNING",
    "RUNNER_BLOCKING",
    "EXPORT_BLOCKING",
    "PRIVACY_WARNING",
}

FORBIDDEN_STATUS = {
    "HUMAN_APPROVED_FOR_PROJECT",
    "CODE_COMPLIANT",
    "CERTIFIED",
    "SEALED",
    "APPROVED",
}

REQUIRED_TBD = {
    "final_cli_command_syntax",
    "package_scripts",
    "process_invocation",
    "network_access",
    "filesystem_mutation_policy",
    "ci_provider",
    "release_matrix",
    "public_transport_protocol",
    "external_adapter_formats",
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

    assert schema["properties"]["deliverable_id"]["const"] == "DEL-10-05"
    assert schema["properties"]["package_id"]["const"] == "PKG-10"
    assert {"SOW-054", "SOW-032"} <= set(
        schema["properties"]["scope_items"]["items"]["enum"]
    )
    assert {"OBJ-008", "OBJ-009", "OBJ-012"} <= set(
        schema["properties"]["objectives"]["items"]["enum"]
    )

    runner_status = defs["RunnerStatus"]["properties"]
    assert runner_status["interface_kind"]["const"] == (
        "schema_first_headless_runner_contract"
    )
    assert (
        runner_status["physical_project_container"]["$ref"]
        == "#/$defs/PhysicalProjectContainer"
    )
    for key in [
        "final_cli_command_syntax",
        "package_scripts",
        "public_transport_protocol",
        "ci_provider",
        "release_matrix",
        "external_adapter_formats",
        "local_fea_package_format",
    ]:
        assert runner_status[key]["const"] == "TBD"

    tbd = defs["TbdDecisions"]
    assert REQUIRED_TBD <= set(tbd["required"])
    for key in REQUIRED_TBD:
        assert tbd["properties"][key]["const"] == "TBD"
    assert "physical_project_container" not in set(tbd["required"])
    assert "physical_project_container" not in tbd["properties"]

    physical = defs["PhysicalProjectContainer"]["properties"]
    assert physical["profile"]["const"] == "sqlite_local_project_store"
    assert physical["decision_ref"]["const"] == "SCA-003"
    assert physical["storage_role"]["const"] == "local_store_index_projection"
    assert physical["canonical_truth"]["const"] == "canonical_json_jcs_payload"
    assert physical["sql_public_contract"]["const"] is False
    assert physical["direct_sql_access_allowed"]["const"] is False
    assert physical["hosted_db_allowed"]["const"] is False
    assert physical["network_required"]["const"] is False
    assert physical["sidecars_rebuildable"]["const"] is True

    request_required = required_at(schema, "HeadlessRunnerRequest")
    assert {
        "request_id",
        "operation",
        "operation_ref",
        "project_ref",
        "model_ref",
        "unit_system_ref",
        "load_basis_refs",
        "input_manifest_ref",
        "requested_outputs",
        "privacy",
        "provenance",
        "professional_boundary",
    } <= request_required
    assert REQUIRED_OPERATIONS <= enum_at(schema, "RunnerOperation")
    assert {"result_envelope", "audit_manifest", "diagnostics"} <= enum_at(
        schema, "RequestedOutput"
    )

    result_required = required_at(schema, "HeadlessRunnerResult")
    assert {
        "run_id",
        "job",
        "analysis_status",
        "result_envelope_ref",
        "result_refs",
        "audit_manifest_ref",
        "checksums",
        "diagnostics",
        "privacy",
        "provenance",
        "professional_boundary",
    } <= result_required

    result_ref = defs["ResultEnvelopeRef"]["properties"]
    assert result_ref["schema_ref"]["const"] == "schemas/results.schema.yaml"
    assert result_ref["compatibility"]["const"] == "schema_first_json_result_envelope"
    assert (
        defs["HeadlessRunnerResult"]["properties"]["result_refs"]["items"]["$ref"]
        == "#/$defs/Reference"
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
    assert (
        defs["HeadlessRunnerResult"]["properties"]["analysis_status"]["contains"][
            "const"
        ]
        == "HUMAN_REVIEW_REQUIRED"
    )

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

    privacy = defs["PrivacyContext"]["properties"]
    assert privacy["local_only"]["const"] is True
    assert privacy["telemetry_allowed"]["const"] is False
    assert "protected_suspected" in privacy["classification"]["enum"]

    boundary = defs["ProfessionalBoundary"]["properties"]
    assert boundary["human_review_required"]["const"] is True
    assert boundary["software_makes_compliance_claim"]["const"] is False
    assert boundary["software_makes_certification_claim"]["const"] is False
    assert boundary["software_makes_sealing_claim"]["const"] is False
    assert boundary["software_makes_approval_claim"]["const"] is False
    assert boundary["software_makes_authentication_claim"]["const"] is False

    checksum = defs["ChecksumRef"]["properties"]
    assert set(checksum["algorithm"]["enum"]) == {"sha256", "sha512", "TBD"}
    assert set(checksum["canonicalization"]["enum"]) == {"JCS", "NONE", "TBD"}
    assert "JCS" in checksum["canonicalization"]["enum"]
    assert "NONE" in checksum["canonicalization"]["enum"]
    assert defs["HeadlessRunnerResult"]["properties"]["checksums"]["minItems"] == 1

    schema_upper = SCHEMA_PATH.read_text(encoding="utf-8").upper()
    for term in FORBIDDEN_STATUS:
        assert term not in schema_upper


def test_headless_runner_contract():
    main()


if __name__ == "__main__":
    main()
