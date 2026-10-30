#!/usr/bin/env python3
"""Stdlib checks for the plugin manifest schema."""

import json
import sys
from pathlib import Path

if str(Path(__file__).resolve().parent) not in sys.path:
    sys.path.insert(0, str(Path(__file__).resolve().parent))

from schema_validation import load_schema  # noqa: E402


ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "schemas" / "plugin_manifest.schema.yaml"
FIXTURE_PATH = ROOT / "fixtures" / "plugin_manifest" / "invented_manifest_no_bypass.json"

REQUIRED_TOP_LEVEL = {
    "metadata",
    "api_boundary_compatibility",
    "entrypoints",
    "permissions",
    "provenance",
    "privacy",
    "checksums",
    "sandbox",
    "no_bypass_constraints",
    "professional_boundary",
}

REQUIRED_NO_BYPASS = {
    "unit_controls",
    "provenance_controls",
    "privacy_controls",
    "rule_sandbox_controls",
    "analysis_boundary_controls",
    "persistence_controls",
    "human_acceptance_controls",
}


def load_fixture():
    with FIXTURE_PATH.open(encoding="utf-8") as fixture_file:
        return json.load(fixture_file)


def definition(schema, name):
    return schema["$defs"][name]


def check_schema_contract():
    schema = load_schema(SCHEMA_PATH)
    defs = schema["$defs"]

    assert REQUIRED_TOP_LEVEL <= set(schema["required"])
    assert schema["additionalProperties"] is False

    permissions = definition(schema, "PermissionModel")
    assert "denied_by_default" in permissions["required"]
    assert permissions["properties"]["denied_by_default"]["const"] is True
    assert permissions["properties"]["grant_state"]["enum"][0] == "not_granted"

    sandbox = definition(schema, "SandboxDeclaration")
    assert "sandbox_required" in sandbox["required"]
    assert sandbox["properties"]["sandbox_required"]["const"] is True
    assert sandbox["properties"]["arbitrary_code_execution_allowed"]["const"] is False
    assert sandbox["properties"]["filesystem_access_default"]["enum"][0] == "denied"
    assert sandbox["properties"]["network_access_default"]["enum"][0] == "denied"
    assert sandbox["properties"]["process_spawn_default"]["enum"] == ["denied"]
    assert sandbox["properties"]["capability_declaration_required"]["const"] is True

    provenance = definition(schema, "ProvenanceRecord")
    required_provenance = {
        "source_name",
        "source_location",
        "source_license",
        "contributor",
        "contributor_certification",
        "redistribution_status",
        "review_status",
    }
    assert required_provenance <= set(provenance["required"])

    no_bypass = definition(schema, "NoBypassConstraints")
    assert REQUIRED_NO_BYPASS <= set(no_bypass["required"])
    for control in REQUIRED_NO_BYPASS:
        assert no_bypass["properties"][control]["const"] is True

    privacy = definition(schema, "PrivacyDeclaration")
    assert privacy["properties"]["local_first"]["const"] is True
    assert privacy["properties"]["private_data_transmission_default"]["const"] is False
    assert privacy["properties"]["telemetry_enabled_by_default"]["const"] is False
    assert privacy["properties"]["redaction_supported"]["const"] is True

    compatibility = definition(schema, "ApiBoundaryCompatibility")
    assert compatibility["properties"]["domain_contract_ref"]["const"] == (
        "docs/architecture/extension_domain_contracts.md"
    )
    assert compatibility["properties"]["api_boundary_contract_ref"]["const"] == (
        "api/api_boundary_contract.yaml"
    )
    assert compatibility["properties"]["schema_version_contract"]["enum"][0] == (
        "JSON Schema 2020-12"
    )
    assert compatibility["properties"]["result_envelope_required"]["const"] is True

    entrypoint = definition(schema, "Entrypoint")
    assert "domain_surface" in entrypoint["required"]
    assert "canonical_model" in entrypoint["properties"]["domain_surface"]["enum"]
    assert "project_persistence" in entrypoint["properties"]["domain_surface"]["enum"]
    assert "persistence_access" in entrypoint["properties"]

    persistence_access = definition(schema, "PersistenceAccess")
    assert persistence_access["additionalProperties"] is False
    assert persistence_access["properties"]["access_mode"]["const"] == (
        "application_service_command_query_job_only"
    )
    assert persistence_access["properties"]["direct_sql_allowed"]["const"] is False
    assert (
        persistence_access["properties"]["raw_sqlite_handle_allowed"]["const"] is False
    )
    assert (
        persistence_access["properties"]["table_name_coupling_allowed"]["const"] is False
    )
    assert (
        persistence_access["properties"]["direct_store_mutation_allowed"]["const"] is False
    )

    professional_boundary = definition(schema, "ProfessionalBoundary")
    assert (
        professional_boundary["properties"][
            "human_acceptance_record_software_generated"
        ]["const"]
        is False
    )


def check_plugin_manifest_fixture():
    schema = load_schema(SCHEMA_PATH)
    manifest = load_fixture()

    assert REQUIRED_TOP_LEVEL <= set(manifest)
    assert manifest["manifest_kind"] == "openpipestress.plugin_manifest"
    assert manifest["permissions"]["denied_by_default"] is True
    assert manifest["permissions"]["grant_state"] == "not_granted"
    assert manifest["sandbox"]["sandbox_required"] is True
    assert manifest["sandbox"]["arbitrary_code_execution_allowed"] is False
    assert manifest["sandbox"]["filesystem_access_default"] == "denied"
    assert manifest["sandbox"]["network_access_default"] == "denied"
    assert manifest["sandbox"]["process_spawn_default"] == "denied"
    assert manifest["privacy"]["local_first"] is True
    assert manifest["privacy"]["private_data_transmission_default"] is False
    assert manifest["privacy"]["telemetry_enabled_by_default"] is False
    assert manifest["privacy"]["redaction_supported"] is True

    for control in definition(schema, "NoBypassConstraints")["required"]:
        assert manifest["no_bypass_constraints"][control] is True

    surfaces = {entry["domain_surface"] for entry in manifest["entrypoints"]}
    assert {"canonical_model", "project_persistence", "units"} <= surfaces
    assert manifest["api_boundary_compatibility"]["transport"] == "TBD"
    assert manifest["professional_boundary"]["human_review_required"] is True
    assert (
        manifest["professional_boundary"]["human_acceptance_record_software_generated"]
        is False
    )


def test_plugin_manifest_schema_contract():
    check_schema_contract()


def test_plugin_manifest_fixture_preserves_no_bypass_defaults():
    check_plugin_manifest_fixture()


def test_future_plugin_and_bug_report_routes_remain_absent():
    """Future egress must add a controlled-export boundary, not appear silently."""

    forbidden_names = {
        "plugin_loader",
        "plugin_runtime",
        "bug_report",
        "bug_reporter",
        "crash_report",
        "crash_reporter",
    }
    discovered = []
    for root in (ROOT / "apps", ROOT / "core", ROOT / "tools"):
        for path in root.rglob("*"):
            if "node_modules" in path.parts or "target" in path.parts:
                continue
            normalized = path.name.lower().replace("-", "_")
            stem = Path(normalized).stem
            if stem in forbidden_names or normalized in forbidden_names:
                discovered.append(path.relative_to(ROOT).as_posix())
    assert discovered == []


def main():
    check_schema_contract()
    check_plugin_manifest_fixture()
    test_future_plugin_and_bug_report_routes_remain_absent()


if __name__ == "__main__":
    main()
