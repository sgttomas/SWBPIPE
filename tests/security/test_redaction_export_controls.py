#!/usr/bin/env python3
"""Checks for DEL-12-02 redaction/export controls using invented fixtures."""

import copy
import hashlib
import json
from pathlib import Path
import sys

import pytest


ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT))

from core.security.redaction import (  # noqa: E402
    REDACTED_VALUE,
    classify_export_item,
    control_route_export,
    redact_export_payload,
)
from core.security.redaction import controls  # noqa: E402

WALK_REFUSAL = (
    r"^USER_RELEASE_ROUTE_REQUIRED: the redaction walk does not admit the "
    r"user_released_report context"
)


SCHEMA_PATH = ROOT / "schemas" / "redaction_export_controls.schema.yaml"
DOC_PATH = ROOT / "docs" / "security" / "redaction_export_controls.md"
PARITY_FIXTURE_PATH = ROOT / "fixtures" / "redaction_export_controls" / "cases.json"

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


def test_route_public_envelope_does_not_promote_opaque_sibling():
    payload = {
        "privacy": {
            "privacy_classification": "public_metadata",
            "redistribution_status": "public_permissive",
        },
        "provenance": {
            "privacy_classification": "invented_public_example",
            "redistribution_status": "invented_non_engineering_example",
        },
        "opaque_sibling": "Invented value without leaf metadata",
    }

    for export_context in ("public_report", "shared_model", "downstream_tool"):
        controlled = control_route_export(
            payload,
            route_id="DOTH-HANDOFF-002",
            export_context=export_context,
        )
        decision = next(
            item
            for item in controlled.decisions
            if item.path.endswith("opaque_sibling")
        )

        assert decision.privacy_classification == "unknown"
        assert decision.redistribution_status == "unknown"
        assert decision.review_status == "pending"
        assert decision.action == "redact_value"
        assert decision.reason_code == "REDISTRIBUTION_STATUS_UNKNOWN"
        assert controlled.payload["opaque_sibling"] == REDACTED_VALUE

    blocked_local = control_route_export(
        payload,
        route_id="DOTH-HANDOFF-002",
        export_context="local_private",
    )
    assert blocked_local.blocked is True
    assert blocked_local.payload is None
    assert blocked_local.summary["local_first"]["reason_code"] == (
        "LOCAL_PRIVATE_INTENT_REQUIRED"
    )

    local = control_route_export(
        payload,
        route_id="DOTH-HANDOFF-002",
        export_context="local_private",
        explicit_local_private_intent=True,
    )
    local_decision = next(
        item for item in local.decisions if item.path.endswith("opaque_sibling")
    )
    assert local_decision.privacy_classification == "unknown"
    assert local_decision.redistribution_status == "unknown"
    assert local_decision.review_status == "pending"
    assert local_decision.action == "warning_only"
    assert local_decision.reason_code == "REDISTRIBUTION_STATUS_UNKNOWN"
    assert local.payload["opaque_sibling"] == payload["opaque_sibling"]


def test_route_public_basis_is_exact_record_local_and_never_inherited():
    payload = {
        "public_record": {
            "privacy_classification": "public_metadata",
            "redistribution_status": "public_permissive",
            "direct_leaf": "Direct record metadata",
            "nested_record": {
                "opaque_leaf": "Nested record without its own metadata",
            },
            "nested_list": [
                {"opaque_leaf": "Nested collection record without metadata"},
            ],
        }
    }

    for export_context in ("public_report", "shared_model", "downstream_tool"):
        controlled = control_route_export(
            payload,
            route_id="DOTH-HANDOFF-002",
            export_context=export_context,
        )
        direct = next(
            item for item in controlled.decisions if item.path.endswith("direct_leaf")
        )
        nested = [
            item
            for item in controlled.decisions
            if item.path.endswith("opaque_leaf")
        ]

        assert direct.action == "include"
        assert direct.privacy_classification == "public_metadata"
        assert len(nested) == 2
        assert all(item.privacy_classification == "unknown" for item in nested)
        assert all(item.redistribution_status == "unknown" for item in nested)
        assert all(item.review_status == "pending" for item in nested)
        assert all(item.action == "redact_value" for item in nested)
        assert (
            controlled.payload["public_record"]["nested_record"]["opaque_leaf"]
            == REDACTED_VALUE
        )
        assert (
            controlled.payload["public_record"]["nested_list"][0]["opaque_leaf"]
            == REDACTED_VALUE
        )

    local = control_route_export(
        payload,
        route_id="DOTH-HANDOFF-002",
        export_context="local_private",
        explicit_local_private_intent=True,
    )
    nested_local = [
        item for item in local.decisions if item.path.endswith("opaque_leaf")
    ]
    assert len(nested_local) == 2
    assert all(item.privacy_classification == "unknown" for item in nested_local)
    assert all(item.redistribution_status == "unknown" for item in nested_local)
    assert all(item.review_status == "pending" for item in nested_local)
    assert all(item.action == "warning_only" for item in nested_local)
    assert all(
        item.reason_code == "REDISTRIBUTION_STATUS_UNKNOWN"
        for item in nested_local
    )
    assert local.payload["public_record"]["nested_record"]["opaque_leaf"] == (
        payload["public_record"]["nested_record"]["opaque_leaf"]
    )
    assert local.payload["public_record"]["nested_list"][0]["opaque_leaf"] == (
        payload["public_record"]["nested_list"][0]["opaque_leaf"]
    )


def test_route_control_emits_local_first_evidence_and_unknown_route_fails_closed():
    governed = control_route_export(
        {"invented": "opaque"},
        route_id="DOTH-HANDOFF-002",
        export_context="downstream_tool",
    )
    unknown = control_route_export(
        {"invented": "opaque"},
        route_id="INVENTED-UNKNOWN-ROUTE",
        export_context="downstream_tool",
    )

    assert governed.summary["local_first"] == {
        "route_id": "DOTH-HANDOFF-002",
        "export_context": "downstream_tool",
        "storage_context": "downstream_tool",
        "action": "include_metadata_only",
        "reason_code": "SAFE_PUBLIC_METADATA",
        "blocked": False,
        "metadata_only": True,
        "explicit_local_private_intent": False,
    }
    assert governed.summary["local_first_blocking_count"] == 0
    assert unknown.blocked is True
    assert unknown.payload is None
    assert unknown.summary["local_first"]["reason_code"] == "LOCAL_FIRST_ROUTE_UNKNOWN"
    assert unknown.summary["local_first"]["metadata_only"] is True
    assert unknown.summary["local_first_blocking_count"] == 1


def test_model_document_save_route_is_governed_local_private_with_explicit_intent():
    """DEL-15-01 W-6a: File > Save Model Document… is DOTH-MODEL-DOC-007."""
    from core.security.local_first_storage.route_control import (
        GOVERNED_LOCAL_FIRST_ROUTE_IDS,
        enforce_local_first_route,
    )

    assert "DOTH-MODEL-DOC-007" in GOVERNED_LOCAL_FIRST_ROUTE_IDS
    allowed = enforce_local_first_route(
        route_id="DOTH-MODEL-DOC-007",
        export_context="local_private",
        explicit_local_private_intent=True,
    ).as_schema_dict()
    assert allowed == {
        "route_id": "DOTH-MODEL-DOC-007",
        "export_context": "local_private",
        "storage_context": "local_private",
        "action": "include_metadata_only",
        "reason_code": "PRIVATE_LOCAL_METADATA_ALLOWED",
        "blocked": False,
        "metadata_only": True,
        "explicit_local_private_intent": True,
    }
    without_intent = enforce_local_first_route(
        route_id="DOTH-MODEL-DOC-007", export_context="local_private"
    )
    assert without_intent.blocked is True
    assert without_intent.reason_code == "LOCAL_PRIVATE_INTENT_REQUIRED"
    lookalike = enforce_local_first_route(
        route_id="DOTH-MODEL-DOC-008",
        export_context="local_private",
        explicit_local_private_intent=True,
    )
    assert lookalike.blocked is True
    assert lookalike.reason_code == "LOCAL_FIRST_ROUTE_UNKNOWN"

    controlled = control_route_export(
        {
            "field_id": "invented.model.private",
            "privacy_classification": "private_project_data",
            "redistribution_status": "private_only",
            "review_status": "accepted",
            "value": "Invented private model value",
        },
        route_id="DOTH-MODEL-DOC-007",
        export_context="local_private",
        explicit_local_private_intent=True,
        require_lossless_materialization=True,
    )
    assert controlled.blocked is False
    assert controlled.payload["value"] == "Invented private model value"
    assert controlled.summary["local_first"]["route_id"] == "DOTH-MODEL-DOC-007"
    assert controlled.summary["local_first"]["blocked"] is False


def test_handoff_package_save_route_is_governed_local_private_with_explicit_intent():
    """DEL-15-01 W-6b: File > Save Handoff… is DOTH-HANDOFF-PACKAGE-008."""
    from core.security.local_first_storage.route_control import (
        GOVERNED_LOCAL_FIRST_ROUTE_IDS,
        enforce_local_first_route,
    )

    assert "DOTH-HANDOFF-PACKAGE-008" in GOVERNED_LOCAL_FIRST_ROUTE_IDS
    allowed = enforce_local_first_route(
        route_id="DOTH-HANDOFF-PACKAGE-008",
        export_context="local_private",
        explicit_local_private_intent=True,
    ).as_schema_dict()
    assert allowed == {
        "route_id": "DOTH-HANDOFF-PACKAGE-008",
        "export_context": "local_private",
        "storage_context": "local_private",
        "action": "include_metadata_only",
        "reason_code": "PRIVATE_LOCAL_METADATA_ALLOWED",
        "blocked": False,
        "metadata_only": True,
        "explicit_local_private_intent": True,
    }
    without_intent = enforce_local_first_route(
        route_id="DOTH-HANDOFF-PACKAGE-008", export_context="local_private"
    )
    assert without_intent.blocked is True
    assert without_intent.reason_code == "LOCAL_PRIVATE_INTENT_REQUIRED"
    # The downstream preview route is a different route; neither stands in for the other.
    lookalike = enforce_local_first_route(
        route_id="DOTH-HANDOFF-PACKAGE-009",
        export_context="local_private",
        explicit_local_private_intent=True,
    )
    assert lookalike.blocked is True
    assert lookalike.reason_code == "LOCAL_FIRST_ROUTE_UNKNOWN"

    controlled = control_route_export(
        {
            "field_id": "invented.handoff.private",
            "privacy_classification": "private_project_data",
            "redistribution_status": "private_only",
            "review_status": "accepted",
            "value": "Invented private handoff value",
        },
        route_id="DOTH-HANDOFF-PACKAGE-008",
        export_context="local_private",
        explicit_local_private_intent=True,
        require_lossless_materialization=True,
    )
    assert controlled.blocked is False
    assert controlled.payload["value"] == "Invented private handoff value"
    assert controlled.summary["local_first"]["route_id"] == "DOTH-HANDOFF-PACKAGE-008"
    assert controlled.summary["local_first"]["blocked"] is False


def test_route_control_preserves_public_example_context_compatibility():
    controlled = control_route_export(
        {
            "field_id": "invented.public.example",
            "privacy_classification": "invented_public_example",
            "redistribution_status": "invented_non_engineering_example",
            "review_status": "accepted",
            "value": "Invented public example",
        },
        route_id="DOTH-JSON-001",
        export_context="public_example",
    )

    assert controlled.blocked is False
    assert controlled.payload["value"] == "Invented public example"
    assert controlled.summary["local_first"]["storage_context"] == "public_example"
    assert controlled.summary["local_first"]["reason_code"] == "SAFE_PUBLIC_METADATA"


@pytest.mark.parametrize(
    "malformed_intent",
    ("false", 1, None, {"explicit": True}),
    ids=("string-false", "integer-one", "none", "mapping"),
)
def test_route_control_rejects_malformed_truthy_local_private_intent(
    malformed_intent,
):
    controlled = control_route_export(
        {
            "field_id": "invented.private.route",
            "privacy_classification": "private_project_data",
            "redistribution_status": "private_only",
            "review_status": "accepted",
            "value": "Invented private value",
        },
        route_id="REXC-CORE-007",
        export_context="local_private",
        explicit_local_private_intent=malformed_intent,
    )

    assert controlled.blocked is True
    assert controlled.payload is None
    assert controlled.summary["local_first"]["explicit_local_private_intent"] is False
    assert controlled.summary["local_first"]["reason_code"] == (
        "LOCAL_PRIVATE_INTENT_REQUIRED"
    )
    assert any(
        decision.reason_code == "LOCAL_PRIVATE_INTENT_REQUIRED"
        and decision.action == "block_export"
        for decision in controlled.decisions
    )


def test_route_key_names_and_lookalikes_never_authorize_public_classification():
    payloads = (
        {
            "document_kind": "openpipestress.technical_preview.conservative_pcf_export_package",
            "deliverable_id": "DEL-17-07",
            "schema": "opaque schema",
            "nested_deliverable_id": "DEL-99-99",
            "target_family": "opaque target",
            "nested": {
                "schema_version": "opaque version",
                "target_mapping_refs": ["opaque mapping"],
                "deliverable_id_status": "opaque status",
            },
        },
        {
            "document_kind": "openpipestress.technical_preview.conservative_pcf_export_package",
            "deliverable_id": "DEL-99-99",
            "schema": "wrong deliverable schema",
            "nested_deliverable_id": "wrong deliverable",
            "target_family": "wrong deliverable target",
        },
    )

    for route_id in ("DOTH-FORMAT-003", "DOTH-HANDOFF-002"):
        for payload in payloads:
            controlled = control_route_export(
                payload,
                route_id=route_id,
                export_context="downstream_tool",
            )
            adversarial = [
                decision
                for decision in controlled.decisions
                if any(
                    token in decision.path
                    for token in (
                        "schema",
                        "nested_deliverable_id",
                        "target_family",
                        "target_mapping_refs",
                        "deliverable_id_status",
                    )
                )
            ]
            assert adversarial
            assert all(
                decision.privacy_classification == "unknown"
                for decision in adversarial
            )
            assert all(
                decision.redistribution_status == "unknown"
                for decision in adversarial
            )
            assert all(decision.review_status == "pending" for decision in adversarial)
            assert all(decision.action == "redact_value" for decision in adversarial)


def test_shared_parity_fixture_matches_core_decisions():
    """The shared fixture pins the core contract for the app-side TS mirror.

    `fixtures/redaction_export_controls/cases.json` is consumed by BOTH this
    suite and the desktop mirror test
    (`apps/desktop/src/features/redaction-controls/redactionExportControls.test.ts`),
    so a semantic change on either side fails one of the two suites instead of
    drifting silently.
    """
    corpus = load_json(PARITY_FIXTURE_PATH)
    assert corpus["deliverable_id"] == "DEL-12-02"
    assert corpus["cases"], "parity corpus must not be empty"

    covered_reason_codes = set()
    seen_case_ids = set()
    for case in corpus["cases"]:
        case_id = case["case_id"]
        assert case_id not in seen_case_ids, f"duplicate case_id {case_id}"
        seen_case_ids.add(case_id)

        original_item = copy.deepcopy(case["item"])
        expected = case["expected"]
        decision = classify_export_item(
            case["item"],
            export_context=case["export_context"],
            explicit_local_private_intent=case["explicit_local_private_intent"],
            explicit_user_release_intent=case.get("explicit_user_release_intent", False),
        )
        assert case["item"] == original_item, f"{case_id}: classification mutated the item"
        assert decision.action == expected["action"], case_id
        assert decision.reason_code == expected["reason_code"], case_id
        assert decision.source_metadata_present == expected["source_metadata_present"], case_id
        covered_reason_codes.add(decision.reason_code)

        if case["export_context"] == "user_released_report":
            # The redaction walk refuses the context (§3.2), and no Python route uses
            # the user-release projector, so the case's finding is checked on the item
            # rule's own finding (``_classify``, the function the walk used).
            with pytest.raises(ValueError, match=WALK_REFUSAL):
                redact_export_payload(
                    {"item": case["item"]},
                    export_context=case["export_context"],
                    explicit_local_private_intent=case["explicit_local_private_intent"],
                    explicit_user_release_intent=case["explicit_user_release_intent"],
                )
            item_decision, item_finding = controls._classify(
                case["item"],
                path="$.item",
                export_context=case["export_context"],
                explicit_local_private_intent=case["explicit_local_private_intent"],
                explicit_user_release_intent=case["explicit_user_release_intent"] is True,
                decision_index=1,
                finding_index=1,
            )
            assert item_decision.reason_code == expected["reason_code"], case_id
            findings = () if item_finding is None else (item_finding,)
        else:
            findings = redact_export_payload(
                {"item": case["item"]},
                export_context=case["export_context"],
                explicit_local_private_intent=case["explicit_local_private_intent"],
                explicit_user_release_intent=case.get("explicit_user_release_intent", False),
            ).findings
        if expected["finding"] is None:
            assert not findings, case_id
        else:
            assert len(findings) == 1, case_id
            finding = findings[0]
            assert finding.code == expected["reason_code"], case_id
            assert finding.severity == expected["finding"]["severity"], case_id
            assert finding.as_schema_dict()["class"] == expected["finding"]["class"], case_id

    schema = load_json(SCHEMA_PATH)
    # Two governed codes no item-classifier corpus case can reach.  The TypeScript
    # plain-data gate's refusal (HELP_HUMAN ruling, 2026-10-28: "(a), the plain-data gate at
    # the TS entry points.") needs non-plain input, and every corpus item is plain data; the
    # vitest suite `redactionPlainDataGate.test.ts` holds it.  The route exporter's
    # reserved-name block (HELP_HUMAN ruling, 2026-10-28: "fail closed on reserved internal
    # names, in all three implementations") decides a whole route input before projection,
    # never an item; the value-record suites of all three implementations hold it.  Every
    # other code is exercised here.
    typescript_gate_only = {"EXPORT_INPUT_NOT_PLAIN_DATA", "EXPORT_INPUT_RESERVED_NAME"}
    assert not covered_reason_codes & typescript_gate_only
    assert covered_reason_codes | typescript_gate_only == enum_at(schema, "ReasonCode"), (
        "parity corpus must exercise every governed reason code"
    )


def test_documentation_records_scope_boundaries():
    doc = DOC_PATH.read_text(encoding="utf-8")
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


def test_changed_files_do_not_embed_disallowed_example_content():
    changed_files = [
        SCHEMA_PATH,
        DOC_PATH,
        PARITY_FIXTURE_PATH,
        ROOT / "core" / "security" / "redaction" / "__init__.py",
        ROOT / "core" / "security" / "redaction" / "controls.py",
        ROOT / "core" / "security" / "redaction" / "route_control.py",
        ROOT / "core" / "security" / "local_first_storage" / "route_control.py",
        Path(__file__),
    ]
    combined = "\n".join(path.read_text(encoding="utf-8") for path in changed_files)
    for forbidden in FORBIDDEN_CHANGED_FILE_TERMS:
        assert forbidden not in combined


# W-18b (r1), DEL-08-01 Design `report-user-release.md` §3 and §6: the user-released
# context.  Twins of the vitest checks in `redactionExportControls.test.ts`.

EXISTING_CONTEXTS = (
    "public_report",
    "public_example",
    "shared_model",
    "downstream_tool",
    "local_private",
)
USER_RELEASED_REPORT = "user_released_report"
USER_RELEASE_ROUTE = "DREP-PACKAGE-USER-RELEASE-010"
EXISTING_CASE_COUNT = 23
EXISTING_CASES_SHA256 = "1ad77bb8ba548ee5b9cc018a26493f73797df4751316cc1c335327b91afd142f"
NON_PROJECT_PRIVATE_CLASSES = PRIVATE_CLASSES - {"private_project_data"}
REQUEST_FIXTURE_PATH = (
    ROOT
    / "fixtures"
    / "reports"
    / "invented"
    / "exact_report_package_requests"
    / "pressure_source_1.json"
)


def canonical_text(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def private_item(**extra):
    item = {
        "field_id": "invented.project.name",
        "field_class": "project",
        "privacy_classification": "private_project_data",
        "redistribution_status": "private_only",
        "review_status": "accepted",
        "value": "Invented Released Project A",
    }
    item.update(extra)
    return item


def test_existing_corpus_cases_are_pinned_unchanged():
    """Owner property 2: editing an existing expectation fails."""

    corpus = load_json(PARITY_FIXTURE_PATH)
    existing = [case for case in corpus["cases"] if "explicit_user_release_intent" not in case]
    assert len(existing) == EXISTING_CASE_COUNT
    assert all(case["export_context"] in EXISTING_CONTEXTS for case in existing)
    digest = hashlib.sha256(canonical_text(existing).encode("utf-8")).hexdigest()
    assert digest == EXISTING_CASES_SHA256


def test_user_release_cases_carry_the_release_intent_only_in_the_new_context():
    corpus = load_json(PARITY_FIXTURE_PATH)
    added = [case for case in corpus["cases"] if "explicit_user_release_intent" in case]
    assert added
    assert all(isinstance(case["explicit_user_release_intent"], bool) for case in added)
    assert all(
        case["explicit_user_release_intent"] is False
        or case["export_context"] == USER_RELEASED_REPORT
        for case in added
    )
    assert {
        "USER_RELEASED_PROJECT_DATA",
        "USER_RELEASE_INTENT_REQUIRED",
        "USER_RELEASE_VALUE_UNBOUND",
    } <= {case["expected"]["reason_code"] for case in added}


def test_schema_vocabulary_adds_the_user_released_context_and_reasons():
    schema = load_json(SCHEMA_PATH)
    assert USER_RELEASED_REPORT in enum_at(schema, "ExportContext")
    assert {
        "USER_RELEASED_PROJECT_DATA",
        "USER_RELEASE_INTENT_REQUIRED",
        "USER_RELEASE_VALUE_UNBOUND",
    } <= enum_at(schema, "ReasonCode")


def test_release_intent_cannot_enable_export_in_another_context():
    """Owner property 1: the flag raises with every corpus item in each existing context."""

    corpus = load_json(PARITY_FIXTURE_PATH)
    for case in corpus["cases"]:
        for context in EXISTING_CONTEXTS:
            with pytest.raises(ValueError, match="admitted only in the user_released_report"):
                classify_export_item(
                    case["item"], export_context=context, explicit_user_release_intent=True
                )
            with pytest.raises(ValueError, match="admitted only in the user_released_report"):
                redact_export_payload(
                    {"item": case["item"]},
                    export_context=context,
                    explicit_user_release_intent=True,
                )
            for route_id in ("DREP-PACKAGE-SAVE-009", USER_RELEASE_ROUTE):
                with pytest.raises(ValueError, match="admitted only in the user_released_report"):
                    control_route_export(
                        {"item": case["item"]},
                        route_id=route_id,
                        export_context=context,
                        explicit_user_release_intent=True,
                    )


def test_absent_or_false_release_intent_is_inert_in_existing_contexts():
    for context in EXISTING_CONTEXTS:
        without = redact_export_payload(
            {"item": private_item()},
            export_context=context,
            explicit_local_private_intent=True,
        )
        with_false = redact_export_payload(
            {"item": private_item()},
            export_context=context,
            explicit_local_private_intent=True,
            explicit_user_release_intent=False,
        )
        assert with_false == without


def test_payload_carried_intent_grants_nothing_in_the_user_released_context():
    carried = {
        "user_release_intent": True,
        "explicit_user_release_intent": True,
        "local_private_intent": True,
        "explicit_local_private_intent": True,
        "user_intent": True,
    }
    payload = {
        "item": private_item(**carried, export_policy=dict(carried)),
        "nested": {"deeper": [{"item": private_item(**carried)}]},
    }
    with pytest.raises(ValueError, match=WALK_REFUSAL):
        redact_export_payload(
            payload,
            export_context=USER_RELEASED_REPORT,
            explicit_local_private_intent=True,
        )
    # Each item is decided by step 11 without the wrapper's intent.
    for item in (payload["item"], payload["nested"]["deeper"][0]["item"]):
        decision = classify_export_item(
            item, export_context=USER_RELEASED_REPORT, explicit_local_private_intent=True
        )
        assert decision.reason_code == "USER_RELEASE_INTENT_REQUIRED"


@pytest.mark.parametrize("privacy", sorted(NON_PROJECT_PRIVATE_CLASSES))
@pytest.mark.parametrize(
    "redistribution",
    ("public_permissive", "invented_non_engineering_example", "private_only", "unknown", "TBD", None, "absent"),
)
def test_user_released_context_redacts_non_project_private_classes(privacy, redistribution):
    item = {"field_id": "invented.private", "privacy_classification": privacy, "value": 4.2}
    if redistribution != "absent":
        item["redistribution_status"] = redistribution
    decision = classify_export_item(
        item, export_context=USER_RELEASED_REPORT, explicit_user_release_intent=True
    )
    assert (decision.action, decision.reason_code) == ("redact_value", "PRIVATE_DATA_REDACTED")


def test_user_released_context_decides_by_steps_10_to_13_before_unknown_branches():
    def decide(item, intent):
        return classify_export_item(
            item, export_context=USER_RELEASED_REPORT, explicit_user_release_intent=intent
        )

    step10 = {"field_id": "x", "privacy_classification": "private_rule_pack_data", "redistribution_status": "unknown", "value": 1}
    assert classify_export_item(step10, export_context="public_report").reason_code == "REDISTRIBUTION_STATUS_UNKNOWN"
    assert decide(step10, True).reason_code == "PRIVATE_DATA_REDACTED"

    step11 = {"field_id": "x", "privacy_classification": "private_project_data", "redistribution_status": None, "value": 1}
    assert classify_export_item(step11, export_context="public_report").reason_code == "REDISTRIBUTION_STATUS_UNKNOWN"
    assert (decide(step11, True).action, decide(step11, True).reason_code) == ("warning_only", "USER_RELEASED_PROJECT_DATA")
    assert (decide(step11, False).action, decide(step11, False).reason_code) == ("block_export", "USER_RELEASE_INTENT_REQUIRED")

    missing = {"field_id": "x", "value": 1}
    assert classify_export_item(missing, export_context="local_private").action == "warning_only"
    assert (decide(missing, True).action, decide(missing, True).reason_code) == ("block_export", "USER_RELEASE_VALUE_UNBOUND")

    for extra in (
        {"privacy_classification": "protected_suspected"},
        {"redistribution_status": "protected_suspected"},
        {"review_status": "quarantined"},
        {"review_status": "rejected"},
    ):
        assert decide(private_item(**extra), True).reason_code == "PROTECTED_CONTENT_BLOCKED"


def test_user_release_local_first_admission_requires_route_010_and_wrapper_intent():
    from core.security.local_first_storage.route_control import (
        GOVERNED_LOCAL_FIRST_ROUTE_IDS,
        enforce_local_first_route,
    )

    assert USER_RELEASE_ROUTE in GOVERNED_LOCAL_FIRST_ROUTE_IDS
    allowed = enforce_local_first_route(
        route_id=USER_RELEASE_ROUTE,
        export_context=USER_RELEASED_REPORT,
        explicit_user_release_intent=True,
    ).as_schema_dict()
    assert allowed == {
        "route_id": USER_RELEASE_ROUTE,
        "export_context": USER_RELEASED_REPORT,
        "storage_context": "private_report",
        "action": "include_metadata_only",
        "reason_code": "USER_RELEASE_METADATA_ALLOWED",
        "blocked": False,
        "metadata_only": True,
        "explicit_local_private_intent": False,
        "explicit_user_release_intent": True,
    }
    for intent in (False, "true", 1, None):
        refused = enforce_local_first_route(
            route_id=USER_RELEASE_ROUTE,
            export_context=USER_RELEASED_REPORT,
            explicit_local_private_intent=True,
            explicit_user_release_intent=intent,
        )
        assert refused.blocked is True
        assert refused.reason_code == "USER_RELEASE_INTENT_REQUIRED"
        assert refused.explicit_user_release_intent is False
    for route_id in ("DREP-PACKAGE-SAVE-009", "DREP-IPC-003", "DREP-HTML-SAVE-005", "DOTH-JSON-001"):
        refused = enforce_local_first_route(
            route_id=route_id,
            export_context=USER_RELEASED_REPORT,
            explicit_user_release_intent=True,
        )
        assert (refused.blocked, refused.reason_code) == (True, "USER_RELEASE_ROUTE_REQUIRED")
        assert refused.storage_context == "private_report"
    for context in EXISTING_CONTEXTS:
        refused = enforce_local_first_route(
            route_id=USER_RELEASE_ROUTE,
            export_context=context,
            explicit_local_private_intent=True,
            explicit_user_release_intent=True,
        )
        assert (refused.blocked, refused.reason_code) == (True, "USER_RELEASE_ROUTE_REQUIRED")
        assert refused.storage_context == context


def test_existing_route_local_first_evidence_keeps_its_exact_members():
    from core.security.local_first_storage.route_control import enforce_local_first_route

    for context in EXISTING_CONTEXTS:
        for intent in (False, None):
            evidence = enforce_local_first_route(
                route_id="DREP-PACKAGE-SAVE-009",
                export_context=context,
                explicit_local_private_intent=True,
                explicit_user_release_intent=intent,
            ).as_schema_dict()
            assert list(evidence) == [
                "route_id",
                "export_context",
                "storage_context",
                "action",
                "reason_code",
                "blocked",
                "metadata_only",
                "explicit_local_private_intent",
            ]
            assert evidence["storage_context"] == context


def test_existing_contexts_refuse_user_release_intent_on_every_governed_route():
    """Codex r1 finding 2: stated release intent is refused by name outside its context.

    An absent, None or False flag leaves the evidence exactly as without the option.
    """

    from core.security.local_first_storage.route_control import (
        GOVERNED_LOCAL_FIRST_ROUTE_IDS,
        enforce_local_first_route,
    )

    routes = sorted(GOVERNED_LOCAL_FIRST_ROUTE_IDS - {USER_RELEASE_ROUTE})
    assert len(routes) > 10
    for route_id in routes:
        for context in EXISTING_CONTEXTS:
            for local_intent in (False, True):
                without = enforce_local_first_route(
                    route_id=route_id,
                    export_context=context,
                    explicit_local_private_intent=local_intent,
                )
                assert without.explicit_user_release_intent is None
                assert not without.reason_code.startswith("USER_RELEASE_")
                for absent_or_false in (False, None):
                    same = enforce_local_first_route(
                        route_id=route_id,
                        export_context=context,
                        explicit_local_private_intent=local_intent,
                        explicit_user_release_intent=absent_or_false,
                    )
                    assert same == without
                    assert canonical_text(same.as_schema_dict()) == canonical_text(
                        without.as_schema_dict()
                    )
                for stated in (True, "yes", 1):
                    refused = enforce_local_first_route(
                        route_id=route_id,
                        export_context=context,
                        explicit_local_private_intent=local_intent,
                        explicit_user_release_intent=stated,
                    ).as_schema_dict()
                    assert refused == {
                        "route_id": route_id,
                        "export_context": context,
                        "storage_context": context,
                        "action": "block_storage",
                        "reason_code": "USER_RELEASE_ROUTE_REQUIRED",
                        "blocked": True,
                        "metadata_only": True,
                        "explicit_local_private_intent": local_intent,
                        "explicit_user_release_intent": stated is True,
                    }
    # The reproduction from the review.
    refused = enforce_local_first_route(
        route_id="DREP-PACKAGE-SAVE-009",
        export_context="public_report",
        explicit_user_release_intent=True,
    )
    assert (refused.action, refused.reason_code, refused.blocked) == (
        "block_storage",
        "USER_RELEASE_ROUTE_REQUIRED",
        True,
    )
    # Unknown routes and invalid contexts keep their own refusals and record the intent.
    unknown = enforce_local_first_route(
        route_id="INVENTED-UNKNOWN-ROUTE",
        export_context="public_report",
        explicit_user_release_intent=True,
    )
    assert (unknown.reason_code, unknown.explicit_user_release_intent) == (
        "LOCAL_FIRST_ROUTE_UNKNOWN",
        True,
    )
    invalid = enforce_local_first_route(
        route_id="DREP-PACKAGE-SAVE-009",
        export_context="invented_context",
        explicit_user_release_intent=True,
    )
    assert (invalid.reason_code, invalid.explicit_user_release_intent) == (
        "LOCAL_FIRST_EXPORT_CONTEXT_INVALID",
        True,
    )


def test_payload_carried_local_private_intent_is_not_read_in_the_user_released_context(
    monkeypatch,
):
    """Codex r1 finding 4: the item intent helper is never called there."""

    from core.security.redaction import controls

    calls = []
    original = controls._local_private_intent

    def spy(item, **kwargs):
        calls.append(item)
        return original(item, **kwargs)

    monkeypatch.setattr(controls, "_local_private_intent", spy)
    item = private_item(local_private_intent=True, export_policy={"user_intent": True})
    decision = classify_export_item(
        item, export_context=USER_RELEASED_REPORT, explicit_user_release_intent=False
    )
    assert decision.reason_code == "USER_RELEASE_INTENT_REQUIRED"
    with pytest.raises(ValueError, match=WALK_REFUSAL):
        redact_export_payload({"item": item}, export_context=USER_RELEASED_REPORT)
    assert calls == []
    local = classify_export_item(item, export_context="local_private")
    assert local.reason_code == "PRIVATE_LOCAL_ALLOWED_WITH_WARNING"
    assert calls


def _request_with_raw_release_members():
    request = copy.deepcopy(load_json(REQUEST_FIXTURE_PATH)["request"])
    rows = request["result_documents"][0]["result_envelope"]["result_sets"][0]["values"]
    row = next(item for item in rows if isinstance(item.get("metadata"), dict))
    row["metadata"]["user_release_intent"] = True
    row["metadata"]["explicit_user_release_intent"] = True
    section = request["state_comparison_handoff_records"][0]["sections"]["state_run_sections"][0]
    section["user_release_intent"] = "explicit"
    section["explicit_user_release_intent"] = True
    return request


@pytest.mark.parametrize("route_id", ("DREP-PACKAGE-SAVE-009", "DREP-IPC-003"))
def test_existing_contexts_preserve_raw_intent_named_members(route_id):
    """Raw members named like the release intent stay content, byte for byte (§3.3)."""

    request = _request_with_raw_release_members()
    before = canonical_text(request)
    controlled = control_route_export(
        request,
        route_id=route_id,
        export_context="local_private",
        explicit_local_private_intent=True,
        require_lossless_materialization=True,
    )
    assert controlled.blocked is False
    assert controlled.summary["materialization_withheld"] is False
    assert canonical_text(controlled.payload) == before
    assert canonical_text(request) == before
    assert "explicit_user_release_intent" not in controlled.summary["local_first"]


def test_existing_html_route_preserves_raw_intent_named_text():
    html = '<!doctype html><p data-user_release_intent="true">explicit_user_release_intent: true</p>'
    controlled = control_route_export(
        html,
        route_id="DREP-HTML-SAVE-005",
        export_context="local_private",
        explicit_local_private_intent=True,
        require_lossless_materialization=True,
    )
    assert controlled.blocked is False
    assert controlled.payload == html


# Codex r2: the generic route exporter refuses the user-released context and
# route 010 outright, in every pairing; only the user-release projector admits
# them.  Steps 1-9 markers and the non-project private classes ride on a
# project-labelled object, as in the vitest twin.
GENERIC_REFUSAL = r"^USER_RELEASE_ROUTE_REQUIRED: the generic route exporter does not admit"
PROJECT_METADATA = {"privacy_classification": "private_project_data", "redistribution_status": "private_only"}
BOUNDARY_MARKERS = (
    {"professional_claim": True},
    {"professional_claim": "yes"},
    {"cloud_or_network_reference": True},
    {"remote_url": "invented-remote"},
    {"direct_sql_access": True},
    {"connection_string": "invented-connection"},
    {"storage_bypass": True},
    {"storage_bypass_requested": "yes"},
    {"direct_storage_bypass": True},
    {"application_service_bypass": 1},
    {"secret_material_present": True},
    {"password": "INVENTED_SECRET_MARKER"},
    {"contains_payload": True},
    {"payload_present": "present"},
    {"cache_payload": 1},
    {"sqlite_payload": 1},
    {"concrete_path_present": True},
    {"file_path": "invented/relative/file"},
    {"privacy_classification": "protected_suspected"},
    {"redistribution_status": "protected_suspected"},
    {"privacy_classification": None, "provenance": {"privacy_classification": "protected_suspected"}},
    {"review_status": "quarantined"},
    {"review_status": "rejected"},
    {"review_status": "unknown", "provenance": {"review_status": "quarantined"}},
)
GENERIC_REFUSAL_PAYLOADS = (
    {
        "project_ref": {
            "privacy_classification": "private_rule_pack_data",
            "secret_material_present": True,
            "data": "INVENTED_PRIVATE_RULE_VALUE",
        }
    },
    *({"project_ref": {**PROJECT_METADATA, **marker, "data": "INVENTED_MARKED_VALUE"}} for marker in BOUNDARY_MARKERS),
    *(
        {"project_ref": {"privacy_classification": privacy, "redistribution_status": "private_only", "data": "INVENTED_PRIVATE_VALUE"}}
        for privacy in sorted(NON_PROJECT_PRIVATE_CLASSES)
    ),
    {"project_ref": {**PROJECT_METADATA, "data": "INVENTED_CLEAN_PROJECT_VALUE"}},
    {"project_ref": {"data": "INVENTED_RAW_VALUE"}},
)
GENERIC_REFUSAL_OPTIONS = (
    {},
    {"require_lossless_materialization": True},
    {"explicit_local_private_intent": True},
    {"explicit_local_private_intent": True, "require_lossless_materialization": True},
)


def test_review_payload_marker_and_private_class_variants_are_refused_by_the_generic_exporter():
    from core.security.local_first_storage.route_control import GOVERNED_LOCAL_FIRST_ROUTE_IDS

    routes = sorted(GOVERNED_LOCAL_FIRST_ROUTE_IDS) + ["INVENTED-UNKNOWN-ROUTE"]
    for payload in GENERIC_REFUSAL_PAYLOADS:
        snapshot = canonical_text(payload)
        for route_id in routes:
            for flag in ({"explicit_user_release_intent": True}, {"explicit_user_release_intent": False}, {}):
                for options in GENERIC_REFUSAL_OPTIONS:
                    with pytest.raises(ValueError, match=GENERIC_REFUSAL):
                        control_route_export(
                            payload,
                            route_id=route_id,
                            export_context=USER_RELEASED_REPORT,
                            **flag,
                            **options,
                        )
        for context in EXISTING_CONTEXTS:
            for options in GENERIC_REFUSAL_OPTIONS:
                for flag in ({"explicit_user_release_intent": False}, {}):
                    with pytest.raises(ValueError, match=GENERIC_REFUSAL):
                        control_route_export(
                            payload, route_id=USER_RELEASE_ROUTE, export_context=context, **flag, **options
                        )
                with pytest.raises(ValueError, match="admitted only in the user_released_report"):
                    control_route_export(
                        payload,
                        route_id=USER_RELEASE_ROUTE,
                        export_context=context,
                        explicit_user_release_intent=True,
                        **options,
                    )
        assert canonical_text(payload) == snapshot


# Codex r3 of the r1 slice: the walk decided a value-bearing parent as a whole, so a
# labelled parent hid restricted descendants.  The walk now refuses the context
# outright (§3.2); the user-release projector (TypeScript) is the only payload entry.
# Twins of the desktop generator in ``redactionExportControls.test.ts``.
STEP_1_TO_9_MARKERS = (
    {"professional_claim": True},
    {"cloud_or_network_reference": True},
    {"remote_url": "invented://remote"},
    {"direct_sql_access": True},
    {"sql": "INVENTED_QUERY"},
    {"storage_bypass": True},
    {"application_service_bypass": "yes"},
    {"secret_material_present": True},
    {"password": "INVENTED_SECRET"},
    {"contains_payload": True},
    {"cache_payload": 1},
    {"payload": "INVENTED_PAYLOAD"},
    {"concrete_path_present": True},
    {"file_path": "SYMBOLIC_INVENTED_PATH"},
    {"privacy_classification": "protected_suspected"},
    {"redistribution_status": "protected_suspected"},
    {"review_status": "quarantined"},
    {"review_status": "rejected"},
    {"provenance": {"review_status": "quarantined"}},
    {"provenance": {"privacy_classification": "protected_suspected"}},
)
CODEX_R3_NESTED_REPRO = {
    "privacy_classification": "private_project_data",
    "redistribution_status": "private_only",
    "value": {
        "privacy_classification": "private_rule_pack_data",
        "secret_material_present": True,
        "value": "INVENTED_NESTED",
    },
}
CODEX_R3_SHALLOW_REPRO = {
    "privacy_classification": "private_rule_pack_data",
    "redistribution_status": "private_only",
    "rows": ["INVENTED_NONPROJECT"],
}


def _under_labelled_parent(child, depth):
    node = child
    for level in range(depth, 0, -1):
        node = [{"note": "Invented sibling"}, node] if level % 2 == 1 else {f"level_{level}": node}
    return {**private_item(), "value": node}


def _generated_release_inputs():
    inputs = [
        CODEX_R3_NESTED_REPRO,
        CODEX_R3_SHALLOW_REPRO,
        {"project_ref": {"data": "INVENTED_RAW_VALUE"}},
        "INVENTED_TOP",
        None,
    ]
    for depth in range(4):
        for marker in STEP_1_TO_9_MARKERS:
            provenance_privacy = "privacy_classification" in marker.get("provenance", {})
            for own in (None,) if provenance_privacy else (None, "private_project_data", "private_rule_pack_data"):
                child = {**({} if own is None else {"privacy_classification": own}), **marker, "value": "INVENTED_NESTED"}
                inputs.append(_under_labelled_parent(child, depth))
        for privacy in sorted(NON_PROJECT_PRIVATE_CLASSES):
            inputs.append(
                _under_labelled_parent({"privacy_classification": privacy, "rows": ["INVENTED_NONPROJECT", {"cell": 1}]}, depth)
            )
    return inputs


def test_user_released_walk_refuses_the_repros_generated_inputs_and_corpus_items():
    corpus_items = [{"item": case["item"]} for case in load_json(PARITY_FIXTURE_PATH)["cases"]]
    payloads = _generated_release_inputs() + corpus_items
    assert len(payloads) == 5 + 4 * ((len(STEP_1_TO_9_MARKERS) - 1) * 3 + 1 + len(NON_PROJECT_PRIVATE_CLASSES)) + len(corpus_items)
    for payload in payloads:
        before = copy.deepcopy(payload)
        for user_release in ({"explicit_user_release_intent": True}, {"explicit_user_release_intent": False}, {}):
            for local_private in (True, False):
                with pytest.raises(ValueError, match=WALK_REFUSAL):
                    redact_export_payload(
                        payload,
                        export_context=USER_RELEASED_REPORT,
                        explicit_local_private_intent=local_private,
                        **user_release,
                    )
        assert payload == before


def test_user_released_walk_refusal_follows_the_flag_assertion():
    # Release intent with an existing context keeps its own refusal; the new context's
    # walk refusal comes after it, so neither hides the other.
    for context in EXISTING_CONTEXTS:
        with pytest.raises(ValueError, match="admitted only in the user_released_report context"):
            redact_export_payload(CODEX_R3_NESTED_REPRO, export_context=context, explicit_user_release_intent=True)
    with pytest.raises(ValueError, match="unsupported export_context"):
        redact_export_payload(CODEX_R3_NESTED_REPRO, export_context="cloud_sync")


def test_user_released_item_rule_still_decides_the_nested_markers_by_item():
    # The item rule is unchanged: each nested marker object, decided alone, blocks or
    # redacts by steps 1-9, and Codex's nested object by its secret marker.
    assert classify_export_item(
        CODEX_R3_NESTED_REPRO["value"],
        export_context=USER_RELEASED_REPORT,
        explicit_user_release_intent=True,
    ).reason_code == "SECRET_MATERIAL_BLOCKED"
    for marker in STEP_1_TO_9_MARKERS:
        decision = classify_export_item(
            {**marker, "value": "INVENTED_NESTED"},
            export_context=USER_RELEASED_REPORT,
            explicit_user_release_intent=True,
        )
        assert decision.action in {"block_export", "redact_value"}, marker


def test_existing_contexts_keep_an_unlabelled_value_undecided_as_today():
    for context in EXISTING_CONTEXTS:
        result = redact_export_payload({"project_ref": {"data": "INVENTED_RAW_VALUE"}}, export_context=context)
        assert result.decisions == ()
        assert result.payload == {"project_ref": {"data": "INVENTED_RAW_VALUE"}}


if __name__ == "__main__":
    test_schema_is_strict_and_traceable()
    test_schema_vocabularies_cover_required_controls()
    test_public_report_redacts_private_and_unknown_values_without_mutating_source()
    test_local_private_export_requires_explicit_intent_then_retains_with_warning()
    test_protected_or_professional_boundary_metadata_blocks_export()
    test_missing_metadata_does_not_silently_export_value_bearing_record()
    test_shared_parity_fixture_matches_core_decisions()
    test_documentation_records_scope_boundaries()
    test_changed_files_do_not_embed_disallowed_example_content()
