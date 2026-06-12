#!/usr/bin/env python3
"""Focused tests for DEL-17-07 conservative PCF export packages."""

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

from core.handoff.pcf_export import (  # noqa: E402
    build_pcf_export_package,
    canonical_json,
    render_pcf_text,
)
from schema_validation import (  # noqa: E402
    JsonSchemaDependencyMissing,
    validate_instance,
    validate_schema_document,
)


SCHEMA_PATH = ROOT / "schemas" / "pcf_export.schema.json"
PACKAGE_FIXTURE_PATH = ROOT / "fixtures" / "pcf_export" / "invented" / "pcf_export_package.json"
PCF_FIXTURE_PATH = ROOT / "fixtures" / "pcf_export" / "invented" / "model.pcf"
SOURCE_PAYLOAD_PATH = ROOT / "fixtures" / "pcf_export" / "invented" / "source_pcf_payload.json"
SHA256_PATTERN = re.compile(r"^sha256:[a-f0-9]{64}$")

FORBIDDEN_PAYLOAD_TEXT = {
    "real client",
    "asme table",
    "b31j",
    "cert" + "ified by openpipestress",
    "code " + "compliant",
    "professional " + "acceptance",
    "compatible with caepipe",
}


def load_json(path: Path) -> dict[str, object]:
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def source_payload() -> dict[str, object]:
    return load_json(SOURCE_PAYLOAD_PATH)


def build_from_source() -> dict[str, object]:
    return build_pcf_export_package(**source_payload())


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
            instance_label="build_pcf_export_package output",
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
    assert first["deliverable_id"] == "DEL-17-07"
    assert first["package_status"] == "conservative_pcf_export_foundation"
    assert first["export_profile"]["target_family"] == "pcf"
    assert first["export_profile"]["target_profile_version_basis"] == "TBD"
    assert first["export_profile"]["identity_policy"] == "authoritative_sidecar_id_map"
    member_roles = {item["member_role"] for item in first["manifest"]["package_members"]}
    assert member_roles == {
        "manifest",
        "model_pcf",
        "unit_system_disclosure",
        "stable_id_map",
        "loss_report",
        "validation_report",
        "diagnostics",
    }
    assert first["unit_system_disclosure"]["unit_system_ref"]["ref"] == "unit-system:dec-018-si-dual-display"
    assert first["unit_system_disclosure"]["target_export_units"]["coordinates"] == "MM"
    assert first["unit_system_disclosure"]["conversion_performed"] is True
    assert "node.coordinates" in first["unit_system_disclosure"]["conversion_scope"]
    assert first["unit_system_disclosure"]["protected_content_included"] is False
    assert all(SHA256_PATTERN.match(item["value"]) for item in first["manifest"]["checksums"])
    assert not [item for item in first["diagnostics"] if item["severity"] == "blocking"]


def test_pcf_text_is_byte_stable_ascii_and_sidecar_identity_is_authoritative():
    package = build_from_source()
    pcf_text = package["pcf_text"]

    assert pcf_text == render_pcf_text(package["pcf_payload"], package["export_profile"])
    assert pcf_text == PCF_FIXTURE_PATH.read_text(encoding="ascii")
    assert pcf_text.encode("ascii")
    assert pcf_text.endswith("END-ISOGEN\n")
    assert "PIPELINE-REFERENCE OPS-INVENTED-PCF-001" in pcf_text
    assert "COMPONENT-IDENTIFIER OPS-PIPE-AB" in pcf_text
    assert "OUTSIDE-DIAMETER 60.3" in pcf_text
    assert "WALL-THICKNESS 3.91" in pcf_text
    assert "element:invented:AB" not in pcf_text
    assert {
        entry["metadata_carrier"] for entry in package["stable_id_map"]
    } == {"sidecar_id_map"}


def test_loss_report_covers_required_categories_and_tbd_boundaries():
    package = build_from_source()
    categories = {entry["category"] for entry in package["loss_report"]}

    assert categories == {
        "exported",
        "omitted",
        "approximated",
        "delegated",
        "unsupported",
        "tbd",
    }
    assert any("target profile/version remains unresolved" in entry["reason"] for entry in package["loss_report"])
    assert any("Hidden downstream translator defaults" in entry["reason"] for entry in package["loss_report"])
    assert package["export_profile"]["support_restraint_policy"] == "unsupported_or_tbd_until_source_confirmed"
    assert package["export_profile"]["translator_default_policy"] == "hidden_defaults_blocked_or_loss_reported"


def test_negative_cases_block_hidden_defaults_and_missing_identity():
    payload = source_payload()
    payload["stable_id_map"] = []
    payload["pcf_payload"]["pipe_segments"][0] = deepcopy(payload["pcf_payload"]["pipe_segments"][0])
    del payload["pcf_payload"]["pipe_segments"][0]["wall_thickness"]
    payload["export_profile"] = {"target_profile_version_basis": "CAEPIPE-PCF"}

    package = build_pcf_export_package(**payload)
    codes = {item["code"] for item in package["diagnostics"]}

    assert "PCF-STABLE-ID-MAP-MISSING" in codes
    assert "PCF-EXPLICIT-FIELD-MISSING" in codes
    assert "PCF-TARGET-PROFILE-OVERCLAIM" in codes
    assert package["validation_report"]["validation_status"] == "blocked"


def test_profile_source_basis_refs_are_required():
    payload = source_payload()
    payload["export_profile"] = {"source_basis_refs": [{"object_type": "Deliverable", "ref": "DEL-17-02"}]}

    package = build_pcf_export_package(**payload)

    assert "PCF-SOURCE-BASIS-REFS-MISSING" in {item["code"] for item in package["diagnostics"]}
    assert package["validation_report"]["validation_status"] == "blocked"


def test_no_prohibited_professional_or_external_compatibility_language():
    package = build_from_source()
    text = "\n".join(walk_strings(package)).lower()

    for forbidden in FORBIDDEN_PAYLOAD_TEXT:
        assert forbidden not in text
    boundary = package["professional_boundary"]
    assert boundary["software_makes_release_claim"] is False
    assert boundary["software_makes_target_compatibility_claim"] is False
    assert boundary["software_makes_solver_validation_claim"] is False
    assert boundary["software_makes_code_compliance_claim"] is False
    assert boundary["software_creates_professional_reliance_record"] is False


if __name__ == "__main__":
    test_fixture_and_builder_validate_against_schema()
    test_builder_is_deterministic_and_preserves_package_members()
    test_pcf_text_is_byte_stable_ascii_and_sidecar_identity_is_authoritative()
    test_loss_report_covers_required_categories_and_tbd_boundaries()
    test_negative_cases_block_hidden_defaults_and_missing_identity()
    test_profile_source_basis_refs_are_required()
    test_no_prohibited_professional_or_external_compatibility_language()
    print("PASS: DEL-17-07 PCF export package checks")
