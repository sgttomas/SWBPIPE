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
    "decomposition_revision": "0.7",
    "scope_change_authority": "SCA-004",
    "public_transport_protocol": "TBD",
    "endpoint_syntax": "TBD",
    "openapi_transport_binding": "TBD",
    "external_format_list": "TBD",
    "export_profile_contract": "TBD",
    "stable_id_map_contract": "TBD",
    "loss_report_contract": "TBD",
    "external_run_evidence_contract": "TBD",
    "stress_neutral_export_contract": "TBD",
    "conservative_pcf_export_contract": "TBD",
    "review_geometry_export_contract": "TBD",
    "adapter_sdk_contract": "TBD",
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
    "export_profile",
    "stable_id_map",
    "loss_report",
    "external_run_evidence",
    "adapter_sdk",
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
    "export_profile",
    "stable_id_map",
    "loss_report",
    "external_run_evidence",
    "stress_neutral_export",
    "conservative_pcf_export",
    "review_geometry_export",
    "adapter_sdk",
    "validation_test",
    "no_bypass",
}

REQUIRED_COMMANDS = {
    "ops.model.import",
    "ops.model.export",
    "ops.solve.start",
    "ops.rule_pack.attach",
    "ops.export.profile",
    "ops.export.id_map",
    "ops.export.loss_report",
    "ops.external_run.record",
    "ops.adapter_sdk.register",
}

REQUIRED_QUERIES = {
    "ops.model.describe",
    "ops.results.get",
    "ops.diagnostics.list",
    "ops.plugin.permissions.describe",
    "ops.export.profile.describe",
    "ops.export.id_map.describe",
    "ops.export.loss_report.describe",
    "ops.adapter_sdk.capabilities.describe",
}

REQUIRED_JOBS = {
    "ops.solve.job",
    "ops.export.job",
    "ops.report.job",
    "ops.validation.job",
    "ops.export.stress_neutral.job",
    "ops.export.pcf.job",
    "ops.export.review_geometry.job",
}

REQUIRED_EXPORT_COMMON_CONCEPTS = {
    "export_profile",
    "stable_id_map",
    "loss_report",
    "external_run_evidence",
    "adapter_sdk",
}

REQUIRED_EXPORT_TARGETS = {
    "stress_neutral_export",
    "conservative_pcf_export",
    "review_geometry_export",
    "adapter_sdk",
}

REQUIRED_DEFERRED_EXPORT_DECISIONS = {
    "public_transport_protocol",
    "endpoint_syntax",
    "plugin_runtime",
    "plugin_loading_signing_isolation",
    "permission_grant_persistence",
    "code_generation_tooling",
    "concrete_writer_behavior",
    "external_tool_invocation",
}

REQUIRED_PLUGIN_ENTRYPOINTS = {
    "export_profile_provider",
    "stable_id_map_provider",
    "loss_report_provider",
    "external_run_evidence_provider",
    "stress_neutral_export",
    "conservative_pcf_export",
    "review_geometry_export",
    "adapter_sdk_provider",
}

REQUIRED_PERMISSION_TOKENS = {
    "read_export_profile",
    "write_export_profile",
    "read_stable_id_map",
    "write_stable_id_map",
    "read_loss_report",
    "write_loss_report",
    "record_external_run_evidence",
    "use_adapter_sdk",
    "export_stress_neutral",
    "export_conservative_pcf",
    "export_review_geometry",
}

REQUIRED_CHECKSUM_KINDS = {
    "export_profile",
    "stable_id_map",
    "loss_report",
    "external_run_evidence",
    "review_geometry_asset",
    "adapter_sdk_manifest",
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
    "must_preserve_export_profile_registry",
    "must_preserve_stable_id_maps",
    "must_emit_loss_report_for_target_limitations",
    "must_classify_unsupported_approximated_omitted_or_delegated_behavior",
    "must_record_external_run_evidence_as_non_authoritative",
    "must_not_claim_external_solver_validation",
    "must_not_treat_review_geometry_as_solver_geometry",
    "must_not_rely_on_hidden_translator_defaults",
    "must_preserve_stress_neutral_status_boundary",
    "must_preserve_adapter_sdk_no_bypass_controls",
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
    assert "SCA-004 export concepts" in metadata["export_interoperability_authority"]
    for deferred_key in [
        "public_transport_protocol",
        "endpoint_syntax",
        "plugin_runtime",
        "permission_grant_persistence",
        "code_generation_tooling",
    ]:
        assert metadata[deferred_key] == "TBD"

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

    export_registry = contract["x_export_interoperability_registry"]
    assert export_registry["authority"] == "SCA-004"
    assert export_registry["status"] == "boundary_concepts_only"
    assert REQUIRED_EXPORT_COMMON_CONCEPTS <= set(export_registry["common_concepts"])
    assert REQUIRED_EXPORT_TARGETS <= ids(export_registry["target_families"])
    assert REQUIRED_DEFERRED_EXPORT_DECISIONS <= set(
        export_registry["deferred_decisions"]
    )
    assert "endpoint syntax" in export_registry["scope_boundary"]
    assert "plugin runtime" in export_registry["scope_boundary"]
    assert "permission persistence" in export_registry["scope_boundary"]

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

    plugin_manifest = defs["plugin_manifest"]
    entrypoints = set(
        plugin_manifest["properties"]["entrypoints"]["items"]["enum"]
    )
    assert REQUIRED_PLUGIN_ENTRYPOINTS <= entrypoints

    privacy_context = defs["privacy_context"]
    assert privacy_context["properties"]["telemetry_allowed"]["const"] is False
    assert "protected_suspected" in privacy_context["properties"]["classification"]["enum"]

    permissions = defs["permission_request"]
    assert permissions["properties"]["denied_by_default"]["const"] is True
    assert permissions["properties"]["grant_state"]["enum"][0] == "TBD"
    permission_tokens = set(permissions["properties"]["requested"]["items"]["enum"])
    assert REQUIRED_PERMISSION_TOKENS <= permission_tokens

    checksum_set = defs["checksum_set"]
    assert "JCS-compatible-json" in checksum_set["properties"]["canonicalization"]["enum"]
    checksum_kinds = set(defs["checksum"]["properties"]["kind"]["enum"])
    assert REQUIRED_CHECKSUM_KINDS <= checksum_kinds

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
        "SCA-004 export interoperability surfaces",
        "Export profiles",
        "Stable ID maps",
        "Loss reports",
        "External-run evidence",
        "Stress-neutral CSV/JSON exports",
        "Conservative PCF export",
        "GLB/glTF review geometry",
        "Export adapter SDK surfaces",
        "concept-level boundary tokens",
    ]:
        assert phrase in boundary_text


if __name__ == "__main__":
    main()
