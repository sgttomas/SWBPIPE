#!/usr/bin/env python3
"""Checks for DEL-12-02 redaction/export controls using invented fixtures."""

import copy
import json
from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT))

from core.security.redaction import (  # noqa: E402
    REDACTED_VALUE,
    classify_export_item,
    redact_export_payload,
)


SCHEMA_PATH = ROOT / "schemas" / "redaction_export_controls.schema.yaml"
DOC_PATH = ROOT / "docs" / "security" / "redaction_export_controls.md"
MEMORY_PATH = (
    ROOT
    / "execution"
    / "PKG-12_Security, Privacy, and Private Data Handling"
    / "1_Working"
    / "DEL-12-02_Private data redaction and export controls"
    / "MEMORY.md"
)

REQUIRED_ROOT = {
    "schema_version",
    "deliverable_id",
    "package_id",
    "scope_item",
    "objectives",
    "control_profile",
    "field_policies",
    "export_run",
}

REQUIRED_DEFS = {
    "ControlProfile",
    "ExportContext",
    "ExportRun",
    "FieldClass",
    "FieldPolicy",
    "FindingClass",
    "FindingSeverity",
    "InputSummary",
    "PrivacyClassification",
    "ReasonCode",
    "RedactionAction",
    "RedactionDecision",
    "RedactionFinding",
    "RedactionSummary",
    "RedistributionStatus",
    "ReviewStatus",
}

PRIVATE_CLASSES = {
    "private_project_data",
    "private_material_data",
    "private_component_data",
    "private_rule_pack_data",
    "owner_standard_data",
    "company_design_basis_data",
    "path_data",
    "secret_like_data",
}

POLICY_ACTIONS = {
    "warning_only",
    "redact_value",
    "redact_field",
    "omit_field",
    "block_export",
}

FORBIDDEN_CHANGED_FILE_TERMS = {
    "AS" + "ME",
    "B" + "31",
    "B" + "31J",
    "allowable stress " + "table",
    "stress intensification factor " + "table",
    "vendor catalog " + "value",
    "real " + "secret",
    "private key " + "fixture",
    "cert" + "ified by " + "OpenPipeStress",
    "se" + "aled by " + "OpenPipeStress",
    "code " + "compliant",
}


def load_json(path):
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def enum_at(schema, definition_name):
    return set(schema["$defs"][definition_name]["enum"])


def decision_codes(result):
    return {decision.reason_code for decision in result.decisions}


def finding_codes(result):
    return {finding.code for finding in result.findings}


def invented_payload():
    return {
        "report_id": "invented-redaction-report",
        "project": {
            "name": {
                "field_id": "project.name",
                "field_class": "project",
                "privacy_classification": "private_project_data",
                "redistribution_status": "private_only",
                "review_status": "accepted",
                "value": "Invented Local Project A",
            },
            "status": {
                "field_id": "project.status",
                "field_class": "public_metadata",
                "privacy_classification": "public_metadata",
                "redistribution_status": "public_permissive",
                "review_status": "accepted",
                "value": "MECHANICS_SOLVED",
            },
            "path": {
                "field_id": "project.path",
                "field_class": "path",
                "privacy_classification": "path_data",
                "redistribution_status": "private_only",
                "review_status": "accepted",
                "value": "SYMBOLIC_USER_PROJECT_PACKAGE/invented-project.ops",
            },
        },
        "rule_pack": {
            "identity": {
                "field_id": "rule.identity",
                "field_class": "rule_pack",
                "privacy_classification": "public_metadata",
                "redistribution_status": "public_permissive",
                "review_status": "accepted",
                "value": {
                    "id": "invented-rule-pack",
                    "version": "0.1.0",
                    "checksum": "sha256:invented",
                },
            },
            "private_value": {
                "field_id": "rule.private_value",
                "field_class": "rule_pack",
                "privacy_classification": "private_rule_pack_data",
                "redistribution_status": "private_only",
                "review_status": "accepted",
                "value": 123.45,
            },
        },
        "material": {
            "unknown_source": {
                "field_id": "material.unknown_source",
                "field_class": "material",
                "privacy_classification": "unknown",
                "redistribution_status": "unknown",
                "review_status": "pending",
                "value": "Invented material placeholder",
            }
        },
        "component": {
            "private_component_value": {
                "field_id": "component.private_value",
                "field_class": "component",
                "privacy_classification": "private_component_data",
                "redistribution_status": "private_only",
                "review_status": "accepted",
                "value": 9.87,
            }
        },
        "secret_reference": {
            "field_id": "secret.reference",
            "field_class": "secret_like",
            "privacy_classification": "secret_like_data",
            "redistribution_status": "private_only",
            "review_status": "accepted",
            "value": "SYNTHETIC_SECRET_REFERENCE_ONLY",
        },
    }


def test_schema_is_strict_and_traceable():
    schema = load_json(SCHEMA_PATH)
    assert schema["$schema"] == "https://json-schema.org/draft/2020-12/schema"
    assert schema["additionalProperties"] is False
    assert REQUIRED_ROOT <= set(schema["required"])
    assert REQUIRED_DEFS <= set(schema["$defs"])
    assert schema["properties"]["deliverable_id"]["const"] == "DEL-12-02"
    assert schema["properties"]["package_id"]["const"] == "PKG-12"
    assert schema["properties"]["scope_item"]["const"] == "SOW-040"
    assert {"OBJ-010"} == set(schema["properties"]["objectives"]["items"]["enum"])


def test_schema_vocabularies_cover_required_controls():
    schema = load_json(SCHEMA_PATH)
    assert PRIVATE_CLASSES <= enum_at(schema, "PrivacyClassification")
    assert POLICY_ACTIONS <= enum_at(schema, "RedactionAction")
    assert {
        "public_report",
        "public_example",
        "shared_model",
        "downstream_tool",
        "local_private",
    } <= enum_at(schema, "ExportContext")
    assert {
        "UNKNOWN_PROVENANCE_WARNING",
        "REDISTRIBUTION_STATUS_UNKNOWN",
        "PROTECTED_CONTENT_BLOCKED",
        "MISSING_METADATA_REDACTED",
        "LOCAL_PRIVATE_INTENT_REQUIRED",
        "PROFESSIONAL_BOUNDARY_BLOCKED",
        "PAYLOAD_METADATA_ONLY_REQUIRED",
        "SECRET_MATERIAL_BLOCKED",
        "CLOUD_OR_NETWORK_REFERENCE_BLOCKED",
        "DIRECT_SQL_ACCESS_BLOCKED",
        "STORAGE_BYPASS_BLOCKED",
        "CONCRETE_PATH_REDACTED",
    } <= enum_at(schema, "ReasonCode")

    profile = schema["$defs"]["ControlProfile"]["properties"]
    assert profile["local_first"]["const"] is True
    assert profile["source_mutation_allowed"]["const"] is False
    assert profile["cloud_transmission_allowed"]["const"] is False
    assert profile["local_private_requires_explicit_intent"]["const"] is True
    assert profile["professional_claims_allowed"]["const"] is False
    assert profile["classification_basis"]["const"] == "explicit_metadata_only"


def test_public_report_redacts_private_and_unknown_values_without_mutating_source():
    source = invented_payload()
    original = copy.deepcopy(source)
    result = redact_export_payload(source, export_context="public_report")

    assert source == original
    assert result.blocked is False
    assert result.payload["project"]["name"]["value"] == REDACTED_VALUE
    assert result.payload["project"]["path"]["value"] == REDACTED_VALUE
    assert result.payload["rule_pack"]["private_value"]["value"] == REDACTED_VALUE
    assert result.payload["component"]["private_component_value"]["value"] == REDACTED_VALUE
    assert result.payload["secret_reference"]["value"] == REDACTED_VALUE
    assert result.payload["material"]["unknown_source"]["value"] == REDACTED_VALUE
    assert result.payload["rule_pack"]["identity"]["value"]["checksum"] == "sha256:invented"
    assert "PRIVATE_DATA_REDACTED" in decision_codes(result)
    assert "REDISTRIBUTION_STATUS_UNKNOWN" in decision_codes(result)
    assert "PRIVATE_DATA_WARNING" in {finding.class_ for finding in result.findings}
    assert "PROVENANCE_WARNING" in {finding.class_ for finding in result.findings}


def test_local_private_export_requires_explicit_intent_then_retains_with_warning():
    blocked = redact_export_payload(
        invented_payload(),
        export_context="local_private",
        explicit_local_private_intent=False,
    )
    assert blocked.blocked is True
    assert "LOCAL_PRIVATE_INTENT_REQUIRED" in finding_codes(blocked)

    allowed = redact_export_payload(
        invented_payload(),
        export_context="local_private",
        explicit_local_private_intent=True,
    )
    assert allowed.blocked is False
    assert allowed.payload["project"]["name"]["value"] == "Invented Local Project A"
    assert allowed.payload["rule_pack"]["private_value"]["value"] == 123.45
    assert allowed.payload["secret_reference"]["value"] == "SYNTHETIC_SECRET_REFERENCE_ONLY"
    assert "PRIVATE_LOCAL_ALLOWED_WITH_WARNING" in finding_codes(allowed)
    assert allowed.summary()["warning_count"] >= 1
    assert allowed.summary()["cloud_transmission_attempted"] is False


def test_storage_privacy_metadata_blocks_or_redacts_unsafe_export_details():
    payload = {
        "payload_marker": {
            "field_id": "storage.payload",
            "field_class": "export_manifest",
            "privacy_classification": "private_project_data",
            "redistribution_status": "private_only",
            "review_status": "accepted",
            "contains_payload": True,
            "payload": "PAYLOAD_DETAIL_SHOULD_NOT_SURVIVE",
            "value": "PAYLOAD_VALUE_SHOULD_NOT_SURVIVE",
        },
        "secret_material": {
            "field_id": "storage.secret",
            "field_class": "secret_like",
            "privacy_classification": "secret_like_data",
            "redistribution_status": "private_only",
            "review_status": "accepted",
            "secret_material_present": True,
            "secret_value": "SECRET_DETAIL_SHOULD_NOT_SURVIVE",
            "value": "SECRET_VALUE_SHOULD_NOT_SURVIVE",
        },
        "cloud_reference": {
            "field_id": "storage.cloud",
            "field_class": "export_manifest",
            "privacy_classification": "private_project_data",
            "redistribution_status": "private_only",
            "review_status": "pending",
            "cloud_or_network_reference": True,
            "cloud_sync_target": "CLOUD_TARGET_SHOULD_NOT_SURVIVE",
            "value": "CLOUD_VALUE_SHOULD_NOT_SURVIVE",
        },
        "direct_sql": {
            "field_id": "storage.sql",
            "field_class": "export_manifest",
            "privacy_classification": "private_project_data",
            "redistribution_status": "private_only",
            "review_status": "pending",
            "direct_sql_access": True,
            "table_name": "SQL_TABLE_DETAIL_SHOULD_NOT_SURVIVE",
            "value": "SQL_VALUE_SHOULD_NOT_SURVIVE",
        },
        "storage_bypass": {
            "field_id": "storage.bypass",
            "field_class": "export_manifest",
            "privacy_classification": "private_project_data",
            "redistribution_status": "private_only",
            "review_status": "pending",
            "storage_bypass_requested": True,
            "value": "BYPASS_VALUE_SHOULD_NOT_SURVIVE",
        },
        "concrete_path": {
            "field_id": "storage.path",
            "field_class": "path",
            "privacy_classification": "path_data",
            "redistribution_status": "private_only",
            "review_status": "accepted",
            "concrete_path_present": True,
            "file_path": "FAKE_CONCRETE_PATH_SHOULD_NOT_SURVIVE",
            "value": "PATH_VALUE_SHOULD_NOT_SURVIVE",
        },
    }

    result = redact_export_payload(payload, export_context="public_report")
    serialized_payload = json.dumps(result.payload, sort_keys=True)

    assert result.blocked is True
    assert {
        "PAYLOAD_METADATA_ONLY_REQUIRED",
        "SECRET_MATERIAL_BLOCKED",
        "CLOUD_OR_NETWORK_REFERENCE_BLOCKED",
        "DIRECT_SQL_ACCESS_BLOCKED",
        "STORAGE_BYPASS_BLOCKED",
        "CONCRETE_PATH_REDACTED",
    } <= finding_codes(result)
    assert result.payload["concrete_path"]["value"] == REDACTED_VALUE
    assert "file_path" not in result.payload["concrete_path"]
    for leaked in {
        "PAYLOAD_DETAIL_SHOULD_NOT_SURVIVE",
        "PAYLOAD_VALUE_SHOULD_NOT_SURVIVE",
        "SECRET_DETAIL_SHOULD_NOT_SURVIVE",
        "SECRET_VALUE_SHOULD_NOT_SURVIVE",
        "CLOUD_TARGET_SHOULD_NOT_SURVIVE",
        "CLOUD_VALUE_SHOULD_NOT_SURVIVE",
        "SQL_TABLE_DETAIL_SHOULD_NOT_SURVIVE",
        "SQL_VALUE_SHOULD_NOT_SURVIVE",
        "BYPASS_VALUE_SHOULD_NOT_SURVIVE",
        "FAKE_CONCRETE_PATH_SHOULD_NOT_SURVIVE",
        "PATH_VALUE_SHOULD_NOT_SURVIVE",
    }:
        assert leaked not in serialized_payload


def test_local_private_intent_metadata_allows_private_value_with_warning():
    item = {
        "field_id": "project.intent",
        "field_class": "project",
        "privacy_classification": "private_project_data",
        "redistribution_status": "private_only",
        "review_status": "accepted",
        "local_private_intent": True,
        "value": "Invented local-only value",
    }

    result = redact_export_payload({"item": item}, export_context="local_private")
    decision = classify_export_item(item, export_context="local_private")

    assert result.blocked is False
    assert result.payload["item"]["value"] == "Invented local-only value"
    assert "PRIVATE_LOCAL_ALLOWED_WITH_WARNING" in finding_codes(result)
    assert decision.action == "warning_only"
    assert decision.reason_code == "PRIVATE_LOCAL_ALLOWED_WITH_WARNING"


def test_protected_or_professional_boundary_metadata_blocks_export():
    payload = invented_payload()
    payload["protected"] = {
        "field_id": "protected.synthetic",
        "field_class": "report_body",
        "privacy_classification": "protected_suspected",
        "redistribution_status": "protected_suspected",
        "review_status": "quarantined",
        "value": "OPS_SYNTHETIC_PROTECTED_MARKER",
    }
    payload["claim"] = {
        "field_id": "claim.synthetic",
        "field_class": "report_body",
        "privacy_classification": "public_metadata",
        "redistribution_status": "public_permissive",
        "review_status": "accepted",
        "professional_claim": True,
        "value": "OPS_SYNTHETIC_PROFESSIONAL_CLAIM_MARKER",
    }

    result = redact_export_payload(payload, export_context="shared_model")

    assert result.blocked is True
    assert "PROTECTED_CONTENT_BLOCKED" in finding_codes(result)
    assert "PROFESSIONAL_BOUNDARY_BLOCKED" in finding_codes(result)
    assert all(
        finding.severity == "BLOCKING"
        for finding in result.findings
        if finding.code in {"PROTECTED_CONTENT_BLOCKED", "PROFESSIONAL_BOUNDARY_BLOCKED"}
    )


def test_missing_metadata_does_not_silently_export_value_bearing_record():
    payload = {"unclassified": {"field_id": "missing.metadata", "value": "Invented value"}}
    result = redact_export_payload(payload, export_context="downstream_tool")

    assert result.payload["unclassified"]["value"] == REDACTED_VALUE
    assert "MISSING_METADATA_REDACTED" in finding_codes(result)


def test_documentation_and_memory_record_scope_boundaries():
    doc = DOC_PATH.read_text(encoding="utf-8")
    memory = MEMORY_PATH.read_text(encoding="utf-8")
    for required in {
        "deliverable_id: DEL-12-02",
        "package_id: PKG-12",
        "SOW-040",
        "OBJ-010",
        "explicit metadata only",
        "local-first",
        "does not mutate source project data",
    }:
        assert required in doc
    assert "DEL-12-02" in memory
    assert "invented fixtures only" in memory


def test_changed_files_do_not_embed_disallowed_example_content():
    changed_files = [
        SCHEMA_PATH,
        DOC_PATH,
        MEMORY_PATH,
        ROOT / "core" / "security" / "redaction" / "__init__.py",
        ROOT / "core" / "security" / "redaction" / "controls.py",
        Path(__file__),
    ]
    combined = "\n".join(path.read_text(encoding="utf-8") for path in changed_files)
    for forbidden in FORBIDDEN_CHANGED_FILE_TERMS:
        assert forbidden not in combined


if __name__ == "__main__":
    test_schema_is_strict_and_traceable()
    test_schema_vocabularies_cover_required_controls()
    test_public_report_redacts_private_and_unknown_values_without_mutating_source()
    test_local_private_export_requires_explicit_intent_then_retains_with_warning()
    test_protected_or_professional_boundary_metadata_blocks_export()
    test_missing_metadata_does_not_silently_export_value_bearing_record()
    test_documentation_and_memory_record_scope_boundaries()
    test_changed_files_do_not_embed_disallowed_example_content()
