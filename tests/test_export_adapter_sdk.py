#!/usr/bin/env python3
"""Focused tests for DEL-17-09 export adapter SDK admission packages."""

from __future__ import annotations

from copy import deepcopy
import json
from pathlib import Path
import re
import sys


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))
TESTS_DIR = ROOT / "tests"
if str(TESTS_DIR) not in sys.path:
    sys.path.insert(0, str(TESTS_DIR))

from core.handoff.export_adapter_sdk import (  # noqa: E402
    build_export_adapter_sdk_package,
    canonical_json,
)
from schema_validation import (  # noqa: E402
    JsonSchemaDependencyMissing,
    validate_instance,
    validate_schema_document,
)


SCHEMA_PATH = ROOT / "schemas" / "export_adapter_sdk.schema.json"
PACKAGE_FIXTURE_PATH = ROOT / "fixtures" / "export_adapter_sdk" / "invented" / "export_adapter_sdk_package.json"
SOURCE_PAYLOAD_PATH = ROOT / "fixtures" / "export_adapter_sdk" / "invented" / "source_adapter_sdk_payload.json"
SHA256_PATTERN = re.compile(r"^sha256:[a-f0-9]{64}$")

FORBIDDEN_PAYLOAD_TEXT = {
    "real client",
    "asme table",
    "b31j",
    "cert" + "ified by openpipestress",
    "code " + "compliant",
    "professional " + "acceptance",
    "compatible with",
    "vendor-compatible",
}


def load_json(path: Path) -> dict[str, object]:
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def source_payload() -> dict[str, object]:
    return load_json(SOURCE_PAYLOAD_PATH)


def build_from_source() -> dict[str, object]:
    return build_export_adapter_sdk_package(**source_payload())


def walk_strings(value):
    if isinstance(value, str):
        yield value
    elif isinstance(value, dict):
        for item in value.values():
            yield from walk_strings(item)
    elif isinstance(value, list):
        for item in value:
            yield from walk_strings(item)


def check_jsonschema_validation():
    schema = load_json(SCHEMA_PATH)
    fixture = load_json(PACKAGE_FIXTURE_PATH)
    built = build_from_source()
    try:
        assert validate_schema_document(schema, schema_label=str(SCHEMA_PATH))
        assert validate_instance(
            schema,
            fixture,
            schema_label=str(SCHEMA_PATH),
            instance_label=str(PACKAGE_FIXTURE_PATH),
        )
        assert validate_instance(
            schema,
            built,
            schema_label=str(SCHEMA_PATH),
            instance_label="build_export_adapter_sdk_package output",
        )
    except JsonSchemaDependencyMissing as exc:
        _skip_or_note_missing_jsonschema(exc)


def _skip_or_note_missing_jsonschema(exc):
    if "pytest" in sys.modules:
        import pytest

        pytest.skip(str(exc))
    print(f"SKIP: {exc}")


def test_fixture_and_builder_validate_against_schema():
    check_jsonschema_validation()


def test_builder_is_deterministic_and_preserves_package_members():
    first = build_from_source()
    second = build_from_source()

    assert canonical_json(first) == canonical_json(second)
    assert first["deliverable_id"] == "DEL-17-09"
    assert first["package_status"] == "export_adapter_sdk_foundation"
    assert first["adapter_contract"]["runtime_grant_policy"] == "denied_by_default"
    member_roles = {item["member_role"] for item in first["manifest"]["package_members"]}
    assert member_roles == {
        "manifest",
        "target_registry",
        "adapter_contract",
        "validation_checklist",
        "unit_policy_evidence",
        "validation_report",
        "diagnostics",
    }
    assert all(SHA256_PATTERN.match(item["value"]) for item in first["manifest"]["checksums"])
    assert not [item for item in first["diagnostics"] if item["severity"] == "blocking"]


def test_unit_policy_evidence_records_dec018_without_conversion_or_target_claim():
    package = build_from_source()
    evidence = package["unit_policy_evidence"]

    assert evidence["unit_system_ref"]["ref"] == "unit-system:dec-018-si-dual-display"
    assert evidence["storage_convention"] == "entered_units_preserved"
    assert evidence["adapter_scope"] == "adapter_admission_metadata_only"
    assert evidence["conversion_policy"] == "no_adapter_sdk_conversion_performed"
    assert evidence["conversion_performed"] is False
    assert evidence["conversion_scope"] == []
    assert evidence["witness_count"] == len(package["target_registry"])
    assert evidence["target_refs"][0]["ref"] == package["target_registry"][0]["target_id"]
    assert {item["ref"] for item in evidence["decision_basis_refs"]} == {"DEC-018", "DEL-02-02"}
    assert evidence["protected_content_included"] is False
    assert evidence["private_payload_included"] is False


def test_target_registry_records_non_gating_candidate_and_checklist_categories():
    package = build_from_source()
    target = package["target_registry"][0]

    assert target["admission_state"] == "candidate_non_gating"
    assert target["target_version_basis"] == "tbd"
    assert target["runtime_grant_policy"] == "denied_by_default"
    assert {item["loss_category"] for item in target["behavior_families"]} == {"tbd", "unsupported"}
    assert {item["category"] for item in package["validation_checklist"]} == {
        "source_basis",
        "target_version",
        "units_coordinates",
        "stable_identity",
        "loss_report",
        "diagnostics",
        "privacy_screening",
        "export_review",
        "human_review",
    }


def test_runtime_grants_are_denied_by_default():
    package = build_from_source()
    grants = {item["grant_type"]: item for item in package["runtime_grants"]}

    assert set(grants) == {
        "filesystem",
        "network",
        "process",
        "private_data",
        "storage",
        "rule_pack",
        "solver",
        "report_control",
    }
    assert all(item["requested"] is False for item in grants.values())
    assert all(item["decision"] == "denied_by_default" for item in grants.values())
    boundary = package["professional_boundary"]
    assert boundary["creates_runtime_loader"] is False
    assert boundary["creates_public_endpoint"] is False
    assert boundary["creates_external_target_claim"] is False
    assert boundary["creates_release_claim"] is False
    assert boundary["creates_professional_reliance_record"] is False


def test_negative_cases_block_grants_missing_source_basis_and_missing_policies():
    payload = source_payload()
    payload["requested_runtime_grants"] = [
        {"grant_type": "network", "requested": True, "decision": "requested"}
    ]
    payload["target_registry"] = [deepcopy(payload["target_registry"][0])]
    payload["target_registry"][0]["source_basis_refs"] = []
    payload["target_registry"][0]["stable_id_policy"] = "tbd"
    payload["target_registry"][0]["loss_report_policy"] = "tbd"

    package = build_export_adapter_sdk_package(**payload)
    codes = {item["code"] for item in package["diagnostics"]}

    assert "EASDK-RUNTIME-GRANT-REQUESTED" in codes
    assert "EASDK-SOURCE-BASIS-MISSING" in codes
    assert "EASDK-TARGET-STABLE-ID-POLICY-MISSING" in codes
    assert "EASDK-TARGET-LOSS-REPORT-POLICY-MISSING" in codes
    assert package["validation_report"]["validation_status"] == "blocked"


def test_contract_and_admitted_target_source_basis_are_guarded():
    payload = source_payload()
    payload["adapter_contract"] = {"source_basis_refs": [{"object_type": "Deliverable", "ref": "DEL-17-02"}]}
    payload["target_registry"] = [deepcopy(payload["target_registry"][0])]
    payload["target_registry"][0]["admission_state"] = "source_basis_admitted"
    payload["target_registry"][0]["target_version_basis"] = "invented-target-v1"
    payload["target_registry"][0]["source_basis_refs"] = [
        {"object_type": "Deliverable", "ref": "DEL-17-01"},
        {"object_type": "Deliverable", "ref": "DEL-17-02"},
    ]

    package = build_export_adapter_sdk_package(**payload)
    codes = {item["code"] for item in package["diagnostics"]}

    assert "EASDK-CONTRACT-SOURCE-BASIS-INCOMPLETE" in codes
    assert "EASDK-ADMITTED-TARGET-SOURCE-BASIS-INSUFFICIENT" in codes
    assert package["validation_report"]["validation_status"] == "blocked"


def test_no_prohibited_professional_or_target_claim_language():
    package = build_from_source()
    text = "\n".join(walk_strings(package)).lower()

    for forbidden in FORBIDDEN_PAYLOAD_TEXT:
        assert forbidden not in text
    assert "target support" not in text
    assert "release readiness" not in text
    assert "code compliance" not in text
    assert "professional acceptance" not in text


if __name__ == "__main__":
    test_fixture_and_builder_validate_against_schema()
    test_builder_is_deterministic_and_preserves_package_members()
    test_unit_policy_evidence_records_dec018_without_conversion_or_target_claim()
    test_target_registry_records_non_gating_candidate_and_checklist_categories()
    test_runtime_grants_are_denied_by_default()
    test_negative_cases_block_grants_missing_source_basis_and_missing_policies()
    test_contract_and_admitted_target_source_basis_are_guarded()
    test_no_prohibited_professional_or_target_claim_language()
    print("PASS: DEL-17-09 export adapter SDK checks")
