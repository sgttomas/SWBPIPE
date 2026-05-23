#!/usr/bin/env python3
"""Focused tests for DEL-17-04 CAEPIPE MBF export packages."""

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
    assert "TBD-17-01-003" in first["export_profile"]["carried_tbd_refs"]

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
    assert {"exported", "tbd"} <= {item["category"] for item in first["loss_report"]}
    assert not [item for item in first["diagnostics"] if item["severity"] == "blocking"]


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


def test_writer_outputs_mbf_and_sidecars(tmp_path):
    package = build_from_source()

    write_caepipe_mbf_export_package(tmp_path, package)

    assert (tmp_path / "model.mbf").read_text(encoding="ascii") == package["mbf_text"]
    assert load_json(tmp_path / "manifest.json") == package["manifest"]
    assert load_json(tmp_path / "stable_id_map.json") == package["stable_id_map"]
    assert load_json(tmp_path / "loss_report.json") == package["loss_report"]


def test_fixture_contains_no_private_or_protected_payload_text():
    text = "\n".join(walk_strings(load_json(FIXTURE_PATH))).lower()
    for forbidden in FORBIDDEN_PAYLOAD_TEXT:
        assert forbidden not in text


def main():
    check_jsonschema_validation()
    test_builder_is_deterministic_and_preserves_package_members()
    test_rendered_mbf_text_is_stable_and_ascii_safe()
    test_missing_subset_stable_ids_and_loss_report_are_blocking()
    test_privacy_and_authority_boundary_diagnostics_block_public_package()
    test_fixture_contains_no_private_or_protected_payload_text()


if __name__ == "__main__":
    main()
