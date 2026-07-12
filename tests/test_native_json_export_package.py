#!/usr/bin/env python3
"""Focused tests for DEL-17-03 native open JSON export packages."""

from __future__ import annotations

from copy import deepcopy
import hashlib
import json
from pathlib import Path
import re
import sys


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.handoff.native_json import (  # noqa: E402
    CANONICALIZATION_LABEL,
    build_native_json_export_package,
    canonical_json,
    write_native_json_export_package,
)
from schema_validation import (  # noqa: E402
    JsonSchemaDependencyMissing,
    validate_instance,
    validate_schema_document,
)


SCHEMA_PATH = ROOT / "schemas" / "native_json_export.schema.json"
FIXTURE_PATH = ROOT / "fixtures" / "native_json" / "invented" / "native_json_export_package.json"
SHA256_PATTERN = re.compile(r"^sha256:[a-f0-9]{64}$")


FORBIDDEN_PAYLOAD_TEXT = {
    "real client",
    "asme table",
    "b31j",
    "cert" + "ified by openpipestress",
    "code " + "compliant",
    "professional " + "acceptance",
}


def load_json(path: Path) -> dict[str, object]:
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def build_from_fixture(fixture: dict[str, object] | None = None) -> dict[str, object]:
    fixture = fixture or load_json(FIXTURE_PATH)
    return build_native_json_export_package(
        native_export_id=fixture["native_export_id"],
        source_model_ref=fixture["manifest"]["source_model_ref"],
        source_model_hash=fixture["manifest"]["source_model_hash"],
        model_payload=fixture["model_payload"],
        stable_id_map=fixture["stable_id_map"],
        loss_report=fixture["loss_report"],
        export_profile=fixture["export_profile"],
        validation_checks=fixture["validation_report"]["checks"],
        privacy=fixture["privacy"],
        provenance=fixture["provenance"],
    )


def walk_strings(value):
    if isinstance(value, str):
        yield value
    elif isinstance(value, dict):
        for item in value.values():
            yield from walk_strings(item)
    elif isinstance(value, list):
        for item in value:
            yield from walk_strings(item)


def walk_mappings(value):
    if isinstance(value, dict):
        yield value
        for item in value.values():
            yield from walk_mappings(item)
    elif isinstance(value, list):
        for item in value:
            yield from walk_mappings(item)


def checksum_for_scope(package, payload_scope):
    return next(
        item
        for item in package["manifest"]["checksums"]
        if item["payload_scope"] == payload_scope
    )


def check_jsonschema_validation():
    schema = load_json(SCHEMA_PATH)
    fixture = load_json(FIXTURE_PATH)
    built = build_from_fixture()
    try:
        assert validate_schema_document(schema, schema_label=str(SCHEMA_PATH))
        assert validate_instance(
            schema,
            fixture,
            schema_label=str(SCHEMA_PATH),
            instance_label=str(FIXTURE_PATH),
        )
        assert validate_instance(
            schema,
            built,
            schema_label=str(SCHEMA_PATH),
            instance_label="build_native_json_export_package output",
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


def test_builder_is_deterministic_and_preserves_native_package_members():
    first = build_from_fixture()
    second = build_from_fixture()

    assert canonical_json(first) == canonical_json(second)
    assert first["deliverable_id"] == "DEL-17-03"
    assert first["package_id"] == "PKG-17"
    assert first["package_status"] == "native_open_json_export_package"
    assert first["export_profile"]["target_family"] == "native_open_json"

    member_roles = {item["member_role"] for item in first["manifest"]["package_members"]}
    assert member_roles == {
        "manifest",
        "model_payload",
        "stable_id_map",
        "loss_report",
        "validation_report",
        "diagnostics",
    }
    assert len(first["manifest"]["checksums"]) == 7
    assert all(SHA256_PATTERN.match(item["value"]) for item in first["manifest"]["checksums"])
    assert {item["loss_category"] for item in first["stable_id_map"]} == {"exported"}
    assert {"exported", "TBD"} <= {item["category"] for item in first["loss_report"]}
    assert not [item for item in first["diagnostics"] if item["severity"] == "blocking"]


def test_hash_label_bytes_and_mutation_match_the_implemented_contract():
    fixture = load_json(FIXTURE_PATH)
    first = build_from_fixture(fixture)
    second = build_from_fixture(deepcopy(fixture))
    first_bytes = canonical_json(first).encode("ascii")
    second_bytes = canonical_json(second).encode("ascii")

    assert first_bytes == second_bytes
    assert hashlib.sha256(first_bytes).digest() == hashlib.sha256(second_bytes).digest()

    checksum_records = [
        item
        for item in walk_mappings(first)
        if item.get("algorithm") == "sha256" and "canonicalization" in item
    ]
    assert checksum_records
    assert {item["canonicalization"] for item in checksum_records} == {CANONICALIZATION_LABEL}
    assert "JCS" not in first_bytes.decode("ascii")
    assert not {
        key
        for item in walk_mappings(first)
        for key in item
        if "timestamp" in key.lower()
    }

    mutated_fixture = deepcopy(fixture)
    mutated_fixture["model_payload"]["entities"]["nodes"][0]["id"] = "node:mutated"
    mutated = build_from_fixture(mutated_fixture)
    repeated_mutation = build_from_fixture(deepcopy(mutated_fixture))
    original_hash = checksum_for_scope(first, "native_json_model_payload")["value"]
    mutated_hash = checksum_for_scope(mutated, "native_json_model_payload")["value"]

    assert mutated_hash != original_hash
    assert mutated_hash == checksum_for_scope(
        repeated_mutation, "native_json_model_payload"
    )["value"]


def test_supplied_source_checksum_preserves_governed_metadata_without_relabelling():
    fixture = load_json(FIXTURE_PATH)
    supplied = deepcopy(fixture["manifest"]["source_model_hash"])
    supplied["canonicalization"] = "upstream_governed_hash_v2"
    supplied["value"] = "sha256:" + "ab" * 32
    fixture["manifest"]["source_model_hash"] = supplied

    package = build_from_fixture(fixture)
    emitted = package["manifest"]["source_model_hash"]

    assert emitted["canonicalization"] == supplied["canonicalization"]
    assert emitted["value"] == supplied["value"]
    assert checksum_for_scope(package, "source_model_hash") == emitted
    assert {
        item["canonicalization"]
        for item in package["manifest"]["checksums"]
        if item["payload_scope"] != "source_model_hash"
    } == {CANONICALIZATION_LABEL}

    schema = load_json(SCHEMA_PATH)
    assert validate_instance(
        schema,
        package,
        schema_label=str(SCHEMA_PATH),
        instance_label="native package with governed upstream source checksum",
    )

    malformed_fixture = deepcopy(fixture)
    del malformed_fixture["manifest"]["source_model_hash"]["canonicalization"]
    try:
        build_from_fixture(malformed_fixture)
    except ValueError as exc:
        assert "canonicalization label" in str(exc)
    else:
        raise AssertionError("malformed supplied source checksum was not rejected")


def test_schema_and_governed_fixture_make_no_jcs_claim():
    schema_text = SCHEMA_PATH.read_text(encoding="utf-8")
    fixture_text = FIXTURE_PATH.read_text(encoding="utf-8")

    assert CANONICALIZATION_LABEL in schema_text
    assert CANONICALIZATION_LABEL in fixture_text
    assert "JCS" not in schema_text
    assert "JCS" not in fixture_text


def test_missing_units_stable_ids_and_loss_report_are_blocking_not_defaulted():
    fixture = load_json(FIXTURE_PATH)
    payload = deepcopy(fixture["model_payload"])
    payload["units_manifest"] = {}

    package = build_native_json_export_package(
        native_export_id="native-json:blocked",
        source_model_ref=fixture["manifest"]["source_model_ref"],
        source_model_hash=fixture["manifest"]["source_model_hash"],
        model_payload=payload,
        stable_id_map=[],
        loss_report=[],
        export_profile=fixture["export_profile"],
        privacy=fixture["privacy"],
        provenance=fixture["provenance"],
    )

    codes = {item["code"] for item in package["diagnostics"]}
    assert "NJ-LOSS-CATEGORY-UNSUPPORTED" not in codes
    assert {"NJ-UNITS-MANIFEST-MISSING", "NJ-STABLE-ID-MAP-MISSING"} <= codes
    assert any(item["severity"] == "blocking" for item in package["diagnostics"])
    assert package["validation_report"]["validation_status"] == "blocked"
    assert package["loss_report"][0]["category"] == "TBD"


def test_privacy_and_authority_boundary_diagnostics_block_public_package():
    fixture = load_json(FIXTURE_PATH)
    payload = deepcopy(fixture["model_payload"])
    payload["free_metadata"] = {"unsafe_label": "cert" + "ified target export"}
    privacy = deepcopy(fixture["privacy"])
    privacy["protected_payload_embedded"] = True

    package = build_native_json_export_package(
        native_export_id="native-json:privacy-boundary",
        source_model_ref=fixture["manifest"]["source_model_ref"],
        source_model_hash=fixture["manifest"]["source_model_hash"],
        model_payload=payload,
        stable_id_map=fixture["stable_id_map"],
        loss_report=fixture["loss_report"],
        export_profile=fixture["export_profile"],
        privacy=privacy,
        provenance=fixture["provenance"],
    )

    codes = {item["code"] for item in package["diagnostics"]}
    assert {"NJ-PROHIBITED-AUTHORITY-TERM", "NJ-PRIVACY-BOUNDARY-VIOLATION"} <= codes
    assert package["privacy"]["protected_payload_embedded"] is True
    assert package["professional_boundary"]["software_makes_compliance_claim"] is False
    assert package["professional_boundary"]["software_creates_professional_reliance_record"] is False


def test_writer_uses_canonical_json(tmp_path):
    package = build_from_fixture()
    output_path = tmp_path / "native-package.json"

    write_native_json_export_package(output_path, package)

    assert output_path.read_text(encoding="utf-8") == canonical_json(package) + "\n"


def test_fixture_contains_no_private_or_protected_payload_text():
    text = "\n".join(walk_strings(load_json(FIXTURE_PATH))).lower()
    for forbidden in FORBIDDEN_PAYLOAD_TEXT:
        assert forbidden not in text


def main():
    check_jsonschema_validation()
    test_builder_is_deterministic_and_preserves_native_package_members()
    test_hash_label_bytes_and_mutation_match_the_implemented_contract()
    test_supplied_source_checksum_preserves_governed_metadata_without_relabelling()
    test_schema_and_governed_fixture_make_no_jcs_claim()
    test_missing_units_stable_ids_and_loss_report_are_blocking_not_defaulted()
    test_privacy_and_authority_boundary_diagnostics_block_public_package()
    test_fixture_contains_no_private_or_protected_payload_text()


if __name__ == "__main__":
    main()
