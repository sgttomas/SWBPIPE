#!/usr/bin/env python3
"""Checks for DEL-12-04 secret/private-library reference handling."""

import json
from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT))

from core.security.secret_private_library import (  # noqa: E402
    classify_reference,
    credential_placeholder,
    guard_reference_release,
    private_library_reference,
    private_path_reference,
)


DOC_PATH = ROOT / "docs" / "security" / "secret_private_library_handling.md"
MEMORY_PATH = (
    ROOT
    / "execution"
    / "PKG-12_Security, Privacy, and Private Data Handling"
    / "1_Working"
    / "DEL-12-04_Secret and private-library handling"
    / "MEMORY.md"
)

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


def diagnostic_codes(result):
    return {diagnostic.code for diagnostic in result.diagnostics}


def decision_codes(result):
    return {decision.reason_code for decision in result.decisions}


def invented_private_library():
    return private_library_reference(
        reference_id="invented.private.materials",
        library_kind="material",
        label="Invented Private Material Library Reference",
        version="0.0-placeholder",
        checksum="sha256:0000000000000000000000000000000000000000000000000000000000000000",
        checksum_status="placeholder_checksum_recorded",
        source_note="invented local placeholder source note",
        redistribution_status="private_only",
        review_status="pending",
    )


def test_private_library_classification_is_deterministic_and_metadata_only():
    first = classify_reference(invented_private_library())
    second = classify_reference(invented_private_library())

    assert first == second
    assert first.classification_id.startswith("SPL-CLS-")
    assert first.default_posture == "local_private_metadata_only"
    assert first.metadata["checksum"].startswith("sha256:0000")
    assert first.metadata["source_note"] == "invented local placeholder source note"
    assert first.metadata["contains_payload"] is False
    assert first.metadata["secret_material_present"] is False


def test_public_report_keeps_private_reference_metadata_with_warning_only():
    result = guard_reference_release(
        [invented_private_library()],
        release_context="public_report",
    )

    assert result.blocked is False
    assert result.summary()["metadata_only"] is True
    assert "PRIVATE_REFERENCE_METADATA_ONLY" in decision_codes(result)
    assert result.safe_manifest[0]["reference_id"] == "invented.private.materials"
    assert result.safe_manifest[0]["checksum"].startswith("sha256:0000")


def test_public_fixture_rejects_private_payloads_secret_values_and_unknown_rights():
    unsafe_records = [
        {
            "record_kind": "secret_field_reference",
            "reference_id": "invented.secret.value",
            "label": "api token field",
            "storage_locality": "USER_SECRET_REFERENCE",
            "privacy_classification": "secret_like_data",
            "redistribution_status": "private_only",
            "review_status": "pending",
            "source_state": "credential_reference",
            "value": "SYNTHETIC_SECRET_VALUE_SHOULD_NOT_SURVIVE",
        },
        {
            "record_kind": "private_material_library",
            "reference_id": "invented.private.library.payload",
            "label": "Private Library Payload Fixture",
            "storage_locality": "USER_PRIVATE_LIBRARY_ROOT",
            "privacy_classification": "private_material_data",
            "redistribution_status": "private_only",
            "review_status": "pending",
            "source_state": "private_user_supplied",
            "library_payload": {"invented": "payload blocked"},
        },
        {
            "record_kind": "private_path_reference",
            "reference_id": "invented.private.path.payload",
            "label": "Private Path Payload Fixture",
            "storage_locality": "local_private",
            "privacy_classification": "path_data",
            "redistribution_status": "private_only",
            "review_status": "pending",
            "source_state": "private_user_supplied",
            "path_payload": "FAKE_USER_PRIVATE_LIBRARY_ROOT/invented.ops",
        },
        {
            "record_kind": "private_component_library",
            "reference_id": "invented.unknown.redistribution",
            "label": "Unknown Redistribution Fixture",
            "storage_locality": "USER_PRIVATE_LIBRARY_ROOT",
            "privacy_classification": "private_component_data",
            "redistribution_status": "TBD",
            "review_status": "pending",
            "source_state": "private_user_supplied",
            "source_note": "TBD",
        },
    ]

    result = guard_reference_release(
        unsafe_records,
        release_context="public_fixture",
    )
    serialized = json.dumps(result.as_schema_dict(), sort_keys=True)

    assert result.blocked is True
    assert "SECRET_MATERIAL_REFERENCE_ONLY_REQUIRED" in decision_codes(result)
    assert "PRIVATE_LIBRARY_PAYLOAD_REFERENCE_ONLY_REQUIRED" in decision_codes(result)
    assert "PRIVATE_PATH_PAYLOAD_REFERENCE_ONLY_REQUIRED" in decision_codes(result)
    assert "UNKNOWN_REDIS_PRIVATE_DATA_BLOCKED" in decision_codes(result)
    assert "SYNTHETIC_SECRET_VALUE_SHOULD_NOT_SURVIVE" not in serialized
    assert "payload blocked" not in serialized
    assert "FAKE_USER_PRIVATE_LIBRARY_ROOT/invented.ops" not in serialized


def test_local_private_intent_required_and_payloads_remain_blocked():
    private_ref = invented_private_library()
    blocked = guard_reference_release(
        [private_ref],
        release_context="local_private",
        explicit_local_private_intent=False,
    )
    allowed = guard_reference_release(
        [private_ref],
        release_context="local_private",
        explicit_local_private_intent=True,
    )
    payload_blocked = guard_reference_release(
        [
            private_path_reference(
                reference_id="invented.private.path",
                path_class="USER_PRIVATE_LIBRARY_ROOT",
                label="Invented private path placeholder",
                source_note="invented path descriptor only",
                contains_payload=True,
            )
        ],
        release_context="local_private",
        explicit_local_private_intent=True,
    )

    assert blocked.blocked is True
    assert "LOCAL_PRIVATE_INTENT_REQUIRED" in decision_codes(blocked)
    assert allowed.blocked is False
    assert "PRIVATE_LOCAL_METADATA_ALLOWED" in decision_codes(allowed)
    assert payload_blocked.blocked is True
    assert "PRIVATE_PATH_PAYLOAD_REFERENCE_ONLY_REQUIRED" in decision_codes(payload_blocked)


def test_credential_placeholder_uses_fake_key_id_and_descriptor_only():
    placeholder = credential_placeholder(
        reference_id="invented.credential.placeholder",
        placeholder_key_id="fake-key-id:del-12-04-placeholder",
        credential_descriptor="placeholder:license-token-reference",
        review_status="pending",
    )
    classification = classify_reference(placeholder)

    assert classification.privacy_classification == "secret_like_data"
    assert classification.metadata["storage_locality"] == "USER_SECRET_REFERENCE"
    assert "fake-key-id:del-12-04-placeholder" in classification.metadata["value_descriptor"]
    assert classification.metadata["contains_payload"] is False
    assert classification.metadata["secret_material_present"] is False


def test_documentation_and_memory_record_scope_boundaries():
    doc = DOC_PATH.read_text(encoding="utf-8")
    memory = MEMORY_PATH.read_text(encoding="utf-8")
    for required in {
        "deliverable_id: DEL-12-04",
        "package_id: PKG-12",
        "SOW-040",
        "SOW-029",
        "OBJ-010",
        "metadata-only",
        "local-first",
        "invented fixtures only",
    }:
        assert required in doc
    assert "DEL-12-04" in memory
    assert "invented fixtures only" in memory


def test_changed_files_do_not_embed_disallowed_example_content():
    changed_files = [
        DOC_PATH,
        MEMORY_PATH,
        ROOT / "core" / "security" / "secret_private_library" / "__init__.py",
        ROOT / "core" / "security" / "secret_private_library" / "controls.py",
        Path(__file__),
    ]
    combined = "\n".join(path.read_text(encoding="utf-8") for path in changed_files)
    for forbidden in FORBIDDEN_CHANGED_FILE_TERMS:
        assert forbidden not in combined


if __name__ == "__main__":
    test_private_library_classification_is_deterministic_and_metadata_only()
    test_public_report_keeps_private_reference_metadata_with_warning_only()
    test_public_fixture_rejects_private_payloads_secret_values_and_unknown_rights()
    test_local_private_intent_required_and_payloads_remain_blocked()
    test_credential_placeholder_uses_fake_key_id_and_descriptor_only()
    test_documentation_and_memory_record_scope_boundaries()
    test_changed_files_do_not_embed_disallowed_example_content()
