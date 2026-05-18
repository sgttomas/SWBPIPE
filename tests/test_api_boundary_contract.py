#!/usr/bin/env python3
"""Stdlib checks for the public API boundary contract."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CONTRACT_PATH = ROOT / "api" / "api_boundary_contract.yaml"
PLUGIN_BOUNDARY_PATH = ROOT / "docs" / "architecture" / "plugin_boundary.md"

REQUIRED_METADATA = {
    "deliverable_id": "DEL-10-01",
    "package_id": "PKG-10",
    "scope_item": "SOW-030",
    "objective": "OBJ-009",
    "public_transport_protocol": "TBD",
    "endpoint_syntax": "TBD",
    "openapi_transport_binding": "TBD",
    "external_format_list": "TBD",
    "plugin_runtime": "TBD",
    "plugin_loading_signing_isolation": "TBD",
    "permission_grant_persistence": "TBD",
    "api_stability_level": "TBD",
    "code_generation_tooling": "TBD",
    "schema_basis": "JSON Schema 2020-12",
}

REQUIRED_PERSISTENCE_BOUNDARY_PHRASES = {
    "application-service commands, queries, or jobs",
    "must not expose SQL",
    "raw SQLite handles",
    "table names",
    "direct store mutation",
}

REQUIRED_OPERATION_CATEGORIES = {
    "command",
    "query",
    "job",
    "result_envelope",
    "plugin_manifest",
    "diagnostic",
    "provenance",
    "permission",
    "privacy",
    "checksum",
    "validation_test",
    "no_bypass",
}

REQUIRED_BOUNDARY_AREAS = {
    "model_import",
    "model_export",
    "solver_invocation",
    "results",
    "rule_pack_hooks",
    "plugin_manifest",
    "permissions",
    "diagnostics",
    "provenance",
    "privacy",
    "checksums",
    "validation_test",
    "no_bypass",
}

REQUIRED_COMMANDS = {
    "ops.model.import",
    "ops.model.export",
    "ops.solve.start",
    "ops.rule_pack.attach",
}

REQUIRED_QUERIES = {
    "ops.model.describe",
    "ops.results.get",
    "ops.diagnostics.list",
    "ops.plugin.permissions.describe",
}

REQUIRED_JOBS = {
    "ops.solve.job",
    "ops.export.job",
    "ops.report.job",
    "ops.validation.job",
}

REQUIRED_NO_BYPASS = {
    "must_use_domain_validation",
    "must_use_unit_validation",
    "must_preserve_provenance",
    "must_preserve_privacy_classification",
    "must_return_result_envelope",
    "must_preserve_diagnostics",
    "must_preserve_persistence_hash_controls",
    "must_route_persistence_through_application_services",
    "must_not_expose_sql_or_raw_sqlite",
    "must_not_expose_table_names",
    "must_not_mutate_project_store_directly",
    "must_preserve_report_controls",
    "must_preserve_human_acceptance_boundary",
    "must_not_execute_arbitrary_rule_code",
    "must_not_skip_rule_pack_sandbox",
    "must_not_claim_code_compliance",
    "must_not_bundle_protected_content",
    "must_not_transmit_private_data_by_default",
    "must_not_write_private_library_without_permission",
}

FORBIDDEN_STATUS_TERMS = {
    "CODE_COMPLIANT",
    "CERTIFIED",
    "SEALED",
    "APPROVED_FOR_PROFESSIONAL_RELIANCE",
}


def load_contract():
    with CONTRACT_PATH.open(encoding="utf-8") as contract_file:
        return json.load(contract_file)


def ids(items):
    return {item["id"] for item in items}


def main():
    contract = load_contract()
    defs = contract["$defs"]

    assert contract["type"] == "object"
    assert contract["additionalProperties"] is False

    metadata = contract["x_contract_metadata"]
    for key, expected in REQUIRED_METADATA.items():
        assert metadata[key] == expected
    for phrase in REQUIRED_PERSISTENCE_BOUNDARY_PHRASES:
        assert phrase in metadata["persistence_boundary"]

    categories = set(contract["properties"]["operation_category"]["enum"])
    assert REQUIRED_OPERATION_CATEGORIES <= categories

    boundary_areas = set(
        contract["properties"]["operation"]["properties"]["boundary_area"]["enum"]
    )
    assert REQUIRED_BOUNDARY_AREAS <= boundary_areas
    assert contract["properties"]["operation"]["properties"]["transport"]["const"] == "TBD"

    registry = contract["x_operation_registry"]
    assert REQUIRED_COMMANDS <= ids(registry["commands"])
    assert REQUIRED_QUERIES <= ids(registry["queries"])
    assert REQUIRED_JOBS <= ids(registry["jobs"])

    request = defs["request_envelope"]
    assert {"privacy", "provenance", "permissions", "checksums"} <= set(
        request["required"]
    )

    result = defs["result_envelope"]
    assert {"diagnostics", "provenance", "privacy", "checksums", "limitations"} <= set(
        result["required"]
    )
    analysis_status = set(
        result["properties"]["analysis_status"]["items"]["enum"]
    )
    assert "HUMAN_REVIEW_REQUIRED" in analysis_status
    assert "HUMAN_APPROVED_FOR_PROJECT" not in analysis_status
    assert "CODE_COMPLIANT" not in analysis_status

    human_acceptance = defs["human_acceptance_record_ref"]
    assert human_acceptance["properties"]["software_generated"]["const"] is False

    privacy_context = defs["privacy_context"]
    assert privacy_context["properties"]["telemetry_allowed"]["const"] is False
    assert "protected_suspected" in privacy_context["properties"]["classification"]["enum"]

    permissions = defs["permission_request"]
    assert permissions["properties"]["denied_by_default"]["const"] is True
    assert permissions["properties"]["grant_state"]["enum"][0] == "TBD"

    checksum_set = defs["checksum_set"]
    assert "JCS-compatible-json" in checksum_set["properties"]["canonicalization"]["enum"]

    no_bypass = set(defs["no_bypass_constraints"]["items"]["enum"])
    assert REQUIRED_NO_BYPASS <= no_bypass

    contract_text = CONTRACT_PATH.read_text(encoding="utf-8")
    boundary_text = PLUGIN_BOUNDARY_PATH.read_text(encoding="utf-8")
    combined_upper = f"{contract_text}\n{boundary_text}".upper()
    for term in FORBIDDEN_STATUS_TERMS:
        assert term not in combined_upper

    for phrase in [
        "Public transport protocol",
        "OpenAPI transport binding",
        "plugin runtime",
        "Validation-test execution",
        "human-acceptance boundary controls",
        "application-service commands, queries, or jobs",
        "raw SQLite",
        "table names",
        "direct project-store mutation hooks",
    ]:
        assert phrase in boundary_text


if __name__ == "__main__":
    main()
