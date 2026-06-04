#!/usr/bin/env python3
"""Focused tests for DEL-17-04 CAEPIPE MBF export packages."""

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
TESTS_DIR = ROOT / "tests"
if str(TESTS_DIR) not in sys.path:
    sys.path.insert(0, str(TESTS_DIR))

from core.handoff.caepipe_mbf import (  # noqa: E402
    build_caepipe_mbf_export_package,
    canonical_json,
    canonical_text,
    render_caepipe_mbf_text,
    write_caepipe_mbf_export_package,
)
from schema_validation import (  # noqa: E402
    JsonSchemaDependencyMissing,
    validate_instance,
    validate_schema_document,
)


SCHEMA_PATH = ROOT / "schemas" / "caepipe_mbf_export.schema.json"
FIXTURE_PATH = ROOT / "fixtures" / "caepipe_mbf" / "invented" / "caepipe_mbf_export_package.json"
SHA256_PATTERN = re.compile(r"^sha256:[a-f0-9]{64}$")
EXPECTED_LOSS_CATEGORIES = {
    "exported",
    "omitted",
    "approximated",
    "delegated",
    "unsupported",
    "tbd",
}
EXPECTED_SOURCE_BASIS_REFS = {
    ("Deliverable", "DEL-17-01"),
    ("Deliverable", "DEL-17-02"),
    ("SourceID", "CAEPIPE-IMPORT-MBF"),
    ("SourceID", "CAEPIPE-EXPORT-MBF"),
}

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


def source_payload() -> dict[str, object]:
    return {
        "export_id": "caepipe-mbf:invented-del-17-04",
        "source_model_ref": {"object_type": "Model", "ref": "model:invented-caepipe-mbf"},
        "source_model_hash": {
            "algorithm": "sha256",
            "canonicalization": "JCS_compatible_json_payload_hash",
            "payload_ref": {"object_type": "Model", "ref": "model:invented-caepipe-mbf"},
            "payload_scope": "source_model_hash",
            "value": "sha256:" + "1" * 64,
        },
        "model_payload": {
            "units": {"length": "m", "force": "N", "temperature": "C"},
            "nodes": [
                {"node_id": "node:invented:A", "target_id": "N001", "x": 0, "y": 0, "z": 0},
                {"node_id": "node:invented:B", "target_id": "N002", "x": 1.5, "y": 0, "z": 0},
            ],
            "elements": [
                {
                    "element_id": "pipe:invented:001",
                    "target_id": "P001",
                    "from_node": "N001",
                    "to_node": "N002",
                    "section_ref": "section:invented:small-bore",
                    "material_ref": "material:invented:generic",
                }
            ],
            "supports": [
                {"support_id": "support:invented:A", "target_id": "S001", "node": "N001", "support_kind": "ANCHOR"}
            ],
            "load_cases": [
                {"load_case_id": "load:invented:operating", "target_id": "L001", "load_kind": "invented_operating"}
            ],
            "unsupported_entities": [
                {"object_type": "BranchConnection", "ref": "branch:invented:not-in-first-subset"}
            ],
        },
        "stable_id_map": [
            {
                "canonical_ref": {"object_type": "Node", "ref": "node:invented:A"},
                "target_ref": {"object_type": "CaePipeMbfRecord", "ref": "N001"},
                "mapping_status": "mapped_sidecar",
                "loss_category": "exported",
            },
            {
                "canonical_ref": {"object_type": "PipeElement", "ref": "pipe:invented:001"},
                "target_ref": {"object_type": "CaePipeMbfRecord", "ref": "P001"},
                "mapping_status": "mapped_sidecar",
                "loss_category": "exported",
            },
        ],
        "loss_report": [
            {
                "loss_id": "loss:invented:exported-smoke-subset",
                "category": "exported",
                "severity": "info",
                "affected_refs": [{"object_type": "PipeElement", "ref": "pipe:invented:001"}],
                "target_artifact_ref": {"object_type": "CaePipeMbfMember", "ref": "caepipe-mbf:invented-del-17-04:mbf_text"},
                "reason": "Invented straight pipe smoke subset emitted for deterministic package testing.",
                "source_basis_ref": {"object_type": "Deliverable", "ref": "DEL-17-02"},
                "governing_tbd_id": "TBD-17-01-002",
                "downstream_implication": "Does not close broad MBF record-family coverage.",
            },
            {
                "loss_id": "loss:invented:direct-id-carrier",
                "category": "tbd",
                "severity": "warning",
                "affected_refs": [{"object_type": "CaePipeMbfRecord", "ref": "P001"}],
                "target_artifact_ref": {"object_type": "CaePipeMbfMember", "ref": "caepipe-mbf:invented-del-17-04:stable_id_map"},
                "reason": "Direct MBF stable ID carrier remains unconfirmed; sidecar mapping is used.",
                "source_basis_ref": {"object_type": "Deliverable", "ref": "DEL-17-01"},
                "governing_tbd_id": "TBD-17-01-003",
                "downstream_implication": "Downstream MBF work must keep sidecar mapping until direct carrier evidence is admitted.",
            },
            {
                "loss_id": "loss:invented:omitted-expansion-joint",
                "category": "omitted",
                "severity": "warning",
                "affected_refs": [{"object_type": "ExpansionJoint", "ref": "joint:invented:not-in-first-subset"}],
                "target_artifact_ref": {"object_type": "CaePipeMbfMember", "ref": "caepipe-mbf:invented-del-17-04:loss_report"},
                "reason": "Invented expansion joint behavior is outside the bounded smoke subset.",
                "source_basis_ref": {"object_type": "Deliverable", "ref": "DEL-17-04"},
                "governing_tbd_id": "TBD-17-01-002",
                "downstream_implication": "A later source-confirmed profile tranche must classify expansion joint records before export.",
            },
            {
                "loss_id": "loss:invented:approximated-support",
                "category": "approximated",
                "severity": "warning",
                "affected_refs": [{"object_type": "Support", "ref": "support:invented:A"}],
                "target_artifact_ref": {"object_type": "CaePipeMbfMember", "ref": "caepipe-mbf:invented-del-17-04:mbf_text"},
                "reason": "Invented support is represented only as a minimal smoke-subset support record.",
                "source_basis_ref": {"object_type": "Deliverable", "ref": "DEL-17-04"},
                "governing_tbd_id": "TBD-17-01-002",
                "downstream_implication": "Detailed support target behavior remains source-gated and must not be inferred from this fixture.",
            },
            {
                "loss_id": "loss:invented:delegated-target-options",
                "category": "delegated",
                "severity": "warning",
                "affected_refs": [{"object_type": "LoadCase", "ref": "load:invented:operating"}],
                "target_artifact_ref": {"object_type": "CaePipeMbfMember", "ref": "caepipe-mbf:invented-del-17-04:manifest"},
                "reason": "Target-side execution options are recorded as package metadata and are not executed by this foundation.",
                "source_basis_ref": {"object_type": "Deliverable", "ref": "DEL-17-04"},
                "governing_tbd_id": "TBD-17-04-004",
                "downstream_implication": "External execution and target result interpretation remain DEL-17-05 or later work.",
            },
            {
                "loss_id": "loss:invented:unsupported-branch",
                "category": "unsupported",
                "severity": "warning",
                "affected_refs": [{"object_type": "BranchConnection", "ref": "branch:invented:not-in-first-subset"}],
                "target_artifact_ref": {"object_type": "CaePipeMbfMember", "ref": "caepipe-mbf:invented-del-17-04:loss_report"},
                "reason": "Invented branch connection is not supported by the first CAEPIPE MBF smoke subset.",
                "source_basis_ref": {"object_type": "Deliverable", "ref": "DEL-17-04"},
                "governing_tbd_id": "TBD-17-04-004",
                "downstream_implication": "The unsupported branch must remain visible until a later profile tranche classifies branch record support.",
            },
        ],
    }


def build_from_source() -> dict[str, object]:
    payload = source_payload()
    return build_caepipe_mbf_export_package(**payload)


def walk_strings(value):
    if isinstance(value, str):
        yield value
    elif isinstance(value, dict):
        for item in value.values():
            yield from walk_strings(item)
    elif isinstance(value, list):
        for item in value:
            yield from walk_strings(item)


def sha256_value(value: str) -> str:
    return "sha256:" + hashlib.sha256(value.encode("utf-8")).hexdigest()


def check_jsonschema_validation():
    schema = load_json(SCHEMA_PATH)
    fixture = load_json(FIXTURE_PATH)
    built = build_from_source()
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
            instance_label="build_caepipe_mbf_export_package output",
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
    assert first["deliverable_id"] == "DEL-17-04"
    assert first["package_id"] == "PKG-17"
    assert first["package_status"] == "caepipe_mbf_export_foundation"
    assert first["export_profile"]["target_family"] == "caepipe_mbf"
    assert first["export_profile"]["target_version_basis"] == "TBD-17-01-001"
    assert first["export_profile"]["record_subset_basis"] == "TBD-17-01-002"
    assert "TBD-17-01-001" in first["export_profile"]["carried_tbd_refs"]
    assert "TBD-17-01-002" in first["export_profile"]["carried_tbd_refs"]
    assert "TBD-17-01-003" in first["export_profile"]["carried_tbd_refs"]
    source_basis_refs = {
        (item["object_type"], item["ref"]) for item in first["export_profile"]["source_basis_refs"]
    }
    assert source_basis_refs == EXPECTED_SOURCE_BASIS_REFS
    assert ("Deliverable", "DEL-17-03") not in source_basis_refs
    assert first["manifest"]["source_basis_refs"] == first["export_profile"]["source_basis_refs"]

    member_roles = {item["member_role"] for item in first["manifest"]["package_members"]}
    assert member_roles == {
        "manifest",
        "mbf_text",
        "stable_id_map",
        "loss_report",
        "validation_report",
        "diagnostics",
    }
    assert all(SHA256_PATTERN.match(item["value"]) for item in first["manifest"]["checksums"])
    assert {item["carrier_mode"] for item in first["stable_id_map"]} == {"sidecar_mapping"}
    assert {item["category"] for item in first["loss_report"]} == EXPECTED_LOSS_CATEGORIES
    assert not [item for item in first["diagnostics"] if item["severity"] == "blocking"]


def test_fixture_loss_report_covers_required_categories():
    fixture = load_json(FIXTURE_PATH)

    assert {item["category"] for item in fixture["loss_report"]} == EXPECTED_LOSS_CATEGORIES


def test_rendered_mbf_text_is_stable_and_ascii_safe():
    package = build_from_source()
    text = package["mbf_text"]

    assert text == render_caepipe_mbf_text(package["model_payload"], package["export_profile"])
    assert text == canonical_text(text)
    text.encode("ascii")
    assert "NODE,N001,0,0,0" in text
    assert "PIPE,P001,N001,N002,section:invented:small-bore,material:invented:generic" in text
    assert "Canonical IDs are preserved in sidecar mapping" in text


def test_missing_subset_stable_ids_and_loss_report_are_blocking():
    payload = source_payload()
    model_payload = deepcopy(payload["model_payload"])
    model_payload["elements"] = []

    package = build_caepipe_mbf_export_package(
        export_id="caepipe-mbf:blocked",
        source_model_ref=payload["source_model_ref"],
        source_model_hash=payload["source_model_hash"],
        model_payload=model_payload,
        stable_id_map=[],
        loss_report=[],
    )

    codes = {item["code"] for item in package["diagnostics"]}
    assert {"MBF-SMOKE-SUBSET-MISSING", "MBF-STABLE-ID-SIDECAR-MISSING"} <= codes
    assert any(item["severity"] == "blocking" for item in package["diagnostics"])
    assert package["validation_report"]["validation_status"] == "blocked"
    assert package["loss_report"][0]["category"] == "tbd"


def test_unsupported_entities_require_reference_shape():
    payload = source_payload()
    model_payload = deepcopy(payload["model_payload"])
    model_payload["unsupported_entities"] = [{"object_type": "BranchConnection"}]

    package = build_caepipe_mbf_export_package(
        export_id="caepipe-mbf:malformed-unsupported",
        source_model_ref=payload["source_model_ref"],
        source_model_hash=payload["source_model_hash"],
        model_payload=model_payload,
        stable_id_map=payload["stable_id_map"],
        loss_report=payload["loss_report"],
    )

    codes = {item["code"] for item in package["diagnostics"]}
    assert "MBF-UNSUPPORTED-ENTITY-REF-MISSING" in codes
    assert package["validation_report"]["validation_status"] == "blocked"


def test_unsupported_entities_require_matching_loss_report():
    payload = source_payload()
    loss_report = [item for item in payload["loss_report"] if item["category"] != "unsupported"]

    package = build_caepipe_mbf_export_package(
        export_id="caepipe-mbf:unsupported-loss-missing",
        source_model_ref=payload["source_model_ref"],
        source_model_hash=payload["source_model_hash"],
        model_payload=payload["model_payload"],
        stable_id_map=payload["stable_id_map"],
        loss_report=loss_report,
    )

    codes = {item["code"] for item in package["diagnostics"]}
    assert "MBF-UNSUPPORTED-ENTITY-LOSS-MISSING" in codes
    assert package["validation_report"]["validation_status"] == "blocked"


def test_unsupported_warning_or_blocking_loss_classifies_entity():
    for severity in ("warning", "blocking"):
        payload = source_payload()
        for item in payload["loss_report"]:
            if item["category"] == "unsupported":
                item["severity"] = severity

        package = build_caepipe_mbf_export_package(**payload)
        codes = {item["code"] for item in package["diagnostics"]}

        assert "MBF-UNSUPPORTED-ENTITY-LOSS-MISSING" not in codes
        assert "MBF-UNSUPPORTED-LOSS-SEVERITY-UNSAFE" not in codes


def test_unsupported_info_loss_severity_is_blocking():
    payload = source_payload()
    for item in payload["loss_report"]:
        if item["category"] == "unsupported":
            item["severity"] = "info"

    package = build_caepipe_mbf_export_package(**payload)

    codes = {item["code"] for item in package["diagnostics"]}
    assert "MBF-UNSUPPORTED-LOSS-SEVERITY-UNSAFE" in codes
    assert package["validation_report"]["validation_status"] == "blocked"


def test_target_version_basis_must_remain_carried_tbd():
    payload = source_payload()
    payload["export_profile"] = {"target_version_basis": ""}

    package = build_caepipe_mbf_export_package(**payload)

    codes = {item["code"] for item in package["diagnostics"]}
    assert "MBF-TARGET-VERSION-BASIS-UNSAFE" in codes
    assert package["validation_report"]["validation_status"] == "blocked"


def test_record_subset_basis_must_remain_carried_tbd():
    payload = source_payload()
    payload["export_profile"] = {"record_subset_basis": "unreviewed-first-subset"}

    package = build_caepipe_mbf_export_package(**payload)

    codes = {item["code"] for item in package["diagnostics"]}
    assert "MBF-RECORD-SUBSET-BASIS-UNSAFE" in codes
    assert package["validation_report"]["validation_status"] == "blocked"


def test_required_profile_tbd_refs_must_be_carried():
    payload = source_payload()
    payload["export_profile"] = {
        "carried_tbd_refs": ["TBD-17-01-001", "TBD-17-01-003"],
    }

    package = build_caepipe_mbf_export_package(**payload)

    codes = {item["code"] for item in package["diagnostics"]}
    assert "MBF-CARRIED-TBD-REFS-MISSING" in codes
    assert package["validation_report"]["validation_status"] == "blocked"


def test_required_source_basis_refs_must_be_carried():
    payload = source_payload()
    payload["export_profile"] = {
        "source_basis_refs": [
            {"object_type": "Deliverable", "ref": "DEL-17-01"},
            {"object_type": "SourceID", "ref": "CAEPIPE-IMPORT-MBF"},
        ],
    }

    package = build_caepipe_mbf_export_package(**payload)

    codes = {item["code"] for item in package["diagnostics"]}
    assert "MBF-SOURCE-BASIS-REFS-MISSING" in codes
    assert package["validation_report"]["validation_status"] == "blocked"


def test_del_17_03_is_not_caepipe_source_basis_authority():
    payload = source_payload()
    payload["export_profile"] = {
        "source_basis_refs": [
            {"object_type": object_type, "ref": ref}
            for object_type, ref in sorted(EXPECTED_SOURCE_BASIS_REFS)
        ]
        + [{"object_type": "Deliverable", "ref": "DEL-17-03"}],
    }

    package = build_caepipe_mbf_export_package(**payload)

    codes = {item["code"] for item in package["diagnostics"]}
    assert "MBF-SOURCE-BASIS-REFS-UNSAFE" in codes
    assert package["validation_report"]["validation_status"] == "blocked"


def test_privacy_and_authority_boundary_diagnostics_block_public_package():
    payload = source_payload()
    model_payload = deepcopy(payload["model_payload"])
    model_payload["free_metadata"] = {"unsafe_label": "cert" + "ified target export"}
    privacy = {"protected_payload_embedded": True}

    package = build_caepipe_mbf_export_package(
        export_id="caepipe-mbf:privacy-boundary",
        source_model_ref=payload["source_model_ref"],
        source_model_hash=payload["source_model_hash"],
        model_payload=model_payload,
        stable_id_map=payload["stable_id_map"],
        loss_report=payload["loss_report"],
        privacy=privacy,
    )

    codes = {item["code"] for item in package["diagnostics"]}
    assert {"MBF-PROHIBITED-AUTHORITY-TERM", "MBF-PRIVACY-BOUNDARY-VIOLATION"} <= codes
    assert package["privacy"]["protected_payload_embedded"] is True
    assert package["professional_boundary"]["software_makes_caepipe_compatibility_claim"] is False
    assert package["professional_boundary"]["software_creates_professional_reliance_record"] is False


def test_writer_outputs_all_manifest_members_with_matching_hashes(tmp_path):
    package = build_from_source()

    write_caepipe_mbf_export_package(tmp_path, package)

    members = package["manifest"]["package_members"]
    expected_paths = {member["path"] for member in members}
    written_paths = {str(path.relative_to(tmp_path)) for path in tmp_path.rglob("*") if path.is_file()}
    assert written_paths == expected_paths

    declared_checksums = package["manifest"]["checksums"]
    for member in members:
        role = member["member_role"]
        path = tmp_path / member["path"]
        assert path.exists()
        assert member["hash"] in declared_checksums

        if role == "manifest":
            assert load_json(path) == package["manifest"]
            continue
        if member["content_kind"] == "text/plain":
            text = path.read_text(encoding="ascii")
            assert text == package[role]
            assert member["hash"]["canonicalization"] == "normalized_ascii_lf_text"
            assert sha256_value(canonical_text(text)) == member["hash"]["value"]
        else:
            text = path.read_text(encoding="utf-8")
            parsed = json.loads(text)
            assert parsed == package[role]
            assert text == canonical_json(package[role]) + "\n"
            assert member["hash"]["canonicalization"] == "JCS_compatible_json_payload_hash"
            assert sha256_value(canonical_json(parsed)) == member["hash"]["value"]


def test_fixture_contains_no_private_or_protected_payload_text():
    text = "\n".join(walk_strings(load_json(FIXTURE_PATH))).lower()
    for forbidden in FORBIDDEN_PAYLOAD_TEXT:
        assert forbidden not in text


def main():
    check_jsonschema_validation()
    test_builder_is_deterministic_and_preserves_package_members()
    test_fixture_loss_report_covers_required_categories()
    test_rendered_mbf_text_is_stable_and_ascii_safe()
    test_missing_subset_stable_ids_and_loss_report_are_blocking()
    test_unsupported_entities_require_reference_shape()
    test_unsupported_entities_require_matching_loss_report()
    test_unsupported_warning_or_blocking_loss_classifies_entity()
    test_unsupported_info_loss_severity_is_blocking()
    test_target_version_basis_must_remain_carried_tbd()
    test_record_subset_basis_must_remain_carried_tbd()
    test_required_profile_tbd_refs_must_be_carried()
    test_required_source_basis_refs_must_be_carried()
    test_del_17_03_is_not_caepipe_source_basis_authority()
    test_privacy_and_authority_boundary_diagnostics_block_public_package()
    test_fixture_contains_no_private_or_protected_payload_text()


if __name__ == "__main__":
    main()
