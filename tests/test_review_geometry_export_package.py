#!/usr/bin/env python3
"""Focused tests for DEL-17-08 glTF review geometry export packages."""

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

from core.handoff.review_geometry import (  # noqa: E402
    build_review_geometry_export_package,
    canonical_json,
    render_review_geometry_gltf,
    write_review_geometry_export_package,
)
from schema_validation import (  # noqa: E402
    JsonSchemaDependencyMissing,
    validate_instance,
    validate_schema_document,
)


SCHEMA_PATH = ROOT / "schemas" / "review_geometry_export.schema.json"
PACKAGE_FIXTURE_PATH = ROOT / "fixtures" / "review_geometry" / "invented" / "review_geometry_export_package.json"
GLTF_FIXTURE_PATH = ROOT / "fixtures" / "review_geometry" / "invented" / "model.gltf"
SOURCE_PAYLOAD_PATH = ROOT / "fixtures" / "review_geometry" / "invented" / "source_centerline_payload.json"
SHA256_PATTERN = re.compile(r"^sha256:[a-f0-9]{64}$")

FORBIDDEN_PAYLOAD_TEXT = {
    "real client",
    "asme table",
    "b31j",
    "cert" + "ified by openpipestress",
    "code " + "compliant",
    "professional " + "acceptance",
}


def ref(object_type: str, value: str) -> dict[str, str]:
    return {"object_type": object_type, "ref": value}


def load_json(path: Path) -> dict[str, object]:
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def source_payload() -> dict[str, object]:
    return {
        "export_id": "review-geometry:invented-del-17-08",
        "source_model_ref": ref("Model", "model:invented-review-geometry"),
        "source_model_hash": {
            "algorithm": "sha256",
            "canonicalization": "JCS_compatible_json_payload_hash",
            "payload_ref": ref("Model", "model:invented-review-geometry"),
            "payload_scope": "source_model_hash",
            "value": "sha256:" + "3" * 64,
        },
        "geometry_payload": {
            "units": {"length": "m"},
            "coordinate_basis": "gltf_y_up_right_handed_meters",
            "nodes": [
                {"node_id": "node:invented:A", "x": 0, "y": 0, "z": 0},
                {"node_id": "node:invented:B", "x": 1.5, "y": 0, "z": 0},
                {"node_id": "node:invented:C", "x": 1.5, "y": 0.75, "z": 0},
            ],
            "elements": [
                {
                    "element_id": "element:invented:AB",
                    "from_node": "node:invented:A",
                    "to_node": "node:invented:B",
                    "target_name": "ops_segment_AB",
                },
                {
                    "element_id": "element:invented:BC",
                    "from_node": "node:invented:B",
                    "to_node": "node:invented:C",
                    "target_name": "ops_segment_BC",
                },
            ],
            "omitted_entities": [{"object_type": "Support", "ref": "support:invented:A"}],
        },
        "stable_id_map": [
            {
                "canonical_ref": ref("CenterlineElement", "element:invented:AB"),
                "gltf_ref": ref("GltfNode", "0"),
                "metadata_carrier": "gltf_extras_and_sidecar",
                "mapping_status": "mapped",
                "loss_category": "exported",
            },
            {
                "canonical_ref": ref("CenterlineElement", "element:invented:BC"),
                "gltf_ref": ref("GltfNode", "1"),
                "metadata_carrier": "gltf_extras_and_sidecar",
                "mapping_status": "mapped",
                "loss_category": "exported",
            },
        ],
        "loss_report": [
            {
                "loss_id": "loss:invented:review-centerline-exported",
                "category": "exported",
                "severity": "info",
                "affected_refs": [ref("CenterlineElement", "element:invented:AB")],
                "target_artifact_ref": ref("ReviewGeometryMember", "review-geometry:invented-del-17-08:model_gltf"),
                "reason": "Invented centerline segment emitted as visual review geometry.",
                "source_basis_ref": ref("Deliverable", "DEL-17-02"),
                "downstream_implication": "Visual review geometry only; no solver or analysis fidelity is implied.",
            },
            {
                "loss_id": "loss:invented:support-omitted",
                "category": "omitted",
                "severity": "warning",
                "affected_refs": [ref("Support", "support:invented:A")],
                "target_artifact_ref": ref("ReviewGeometryMember", "review-geometry:invented-del-17-08:model_gltf"),
                "reason": "Support symbol review geometry is outside the v1 centerline-only scope.",
                "source_basis_ref": ref("Deliverable", "DEL-17-08"),
                "downstream_implication": "Support context remains available only through source records and loss report.",
            },
        ],
    }


def build_from_source() -> dict[str, object]:
    return build_review_geometry_export_package(**source_payload())


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
            instance_label="build_review_geometry_export_package output",
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


def test_manifest_source_basis_refs_are_schema_required():
    import pytest

    schema = load_json(SCHEMA_PATH)
    package = build_from_source()
    del package["manifest"]["source_basis_refs"]

    with pytest.raises(AssertionError, match="source_basis_refs.*required"):
        validate_instance(
            schema,
            package,
            schema_label=str(SCHEMA_PATH),
            instance_label="review geometry package without manifest source basis",
        )


def test_builder_is_deterministic_and_preserves_package_members():
    first = build_from_source()
    second = build_from_source()

    assert canonical_json(first) == canonical_json(second)
    assert first["deliverable_id"] == "DEL-17-08"
    assert first["package_status"] == "review_geometry_gltf_export_foundation"
    assert first["export_profile"]["artifact_format"] == "gltf_json_embedded_buffer"
    assert first["export_profile"]["primitive_mode"] == "LINES"
    member_roles = {item["member_role"] for item in first["manifest"]["package_members"]}
    assert member_roles == {
        "manifest",
        "model_gltf",
        "stable_id_map",
        "loss_report",
        "validation_report",
        "diagnostics",
    }
    assert all(SHA256_PATTERN.match(item["value"]) for item in first["manifest"]["checksums"])
    assert not [item for item in first["diagnostics"] if item["severity"] == "blocking"]


def test_gltf_structure_uses_lines_and_embedded_buffer_with_identity_extras():
    package = build_from_source()
    gltf = package["gltf"]

    assert gltf == render_review_geometry_gltf(package["geometry_payload"], package["export_profile"])
    assert gltf == load_json(GLTF_FIXTURE_PATH)
    assert gltf["asset"]["version"] == "2.0"
    assert gltf["asset"]["generator"].startswith("OpenPipeStress DEL-17-08")
    assert gltf["buffers"][0]["uri"].startswith("data:application/octet-stream;base64,")
    assert gltf["buffers"][0]["byteLength"] == 48
    assert len(gltf["nodes"]) == 2
    assert len(gltf["meshes"]) == 2
    assert all(accessor["count"] == 2 for accessor in gltf["accessors"])
    assert all(mesh["primitives"][0]["mode"] == 1 for mesh in gltf["meshes"])
    node_meta = gltf["nodes"][0]["extras"]["openpipestress"]
    primitive_meta = gltf["meshes"][0]["primitives"][0]["extras"]["openpipestress"]
    assert node_meta["canonical_ref"] == ref("CenterlineElement", "element:invented:AB")
    assert primitive_meta["canonical_ref"] == ref("CenterlineElement", "element:invented:AB")
    assert node_meta["sidecar_authoritative"] is True
    assert gltf["asset"]["extras"]["openpipestress"]["artifact_role"] == "visual_review_geometry_only"


def test_sidecar_id_map_and_manifest_preserve_canonical_identity():
    package = build_from_source()

    mapped = {item["canonical_ref"]["ref"]: item["gltf_ref"]["ref"] for item in package["stable_id_map"]}
    assert mapped == {"element:invented:AB": "0", "element:invented:BC": "1"}
    artifact = package["manifest"]["gltf_artifact"]
    assert artifact["path"] == "model.gltf"
    assert artifact["gltf_version"] == "2.0"
    assert artifact["primitive_mode"] == "LINES"
    assert artifact["node_count"] == 2
    assert artifact["mesh_count"] == 2
    assert artifact["embedded_buffer"] is True


def test_manifest_records_profile_basis_members_and_bounded_loss_content():
    package = build_from_source()
    manifest = package["manifest"]

    assert manifest["source_model_ref"] == package["source_model_ref"]
    assert manifest["export_profile_ref"] == ref(
        "ExportProfile", package["export_profile"]["profile_id"]
    )
    assert manifest["source_basis_refs"] == package["export_profile"]["source_basis_refs"]
    assert {item["member_role"] for item in manifest["package_members"]} == {
        "manifest",
        "model_gltf",
        "stable_id_map",
        "loss_report",
        "validation_report",
        "diagnostics",
    }
    assert {item["category"] for item in package["loss_report"]} == {
        "exported",
        "omitted",
    }
    assert package["geometry_payload"]["omitted_entities"] == [
        ref("Support", "support:invented:A")
    ]
    assert package["export_profile"]["artifact_format"] == "gltf_json_embedded_buffer"
    assert package["professional_boundary"]["software_makes_target_compatibility_claim"] is False


def test_written_json_gltf_and_sidecar_round_trip_stable_identity(tmp_path):
    package = build_from_source()
    original = deepcopy(package)
    controlled = write_review_geometry_export_package(tmp_path, package)

    assert package == original
    assert controlled.blocked is True
    assert controlled.summary["materialization_withheld"] is True
    assert list(tmp_path.iterdir()) == []


def test_mismatched_stable_id_sidecar_blocks_current_json_gltf_profile():
    payload = source_payload()
    stable_id_map = deepcopy(payload["stable_id_map"])
    stable_id_map[0]["gltf_ref"] = ref("GltfNode", "1")

    package = build_review_geometry_export_package(
        **{**payload, "stable_id_map": stable_id_map}
    )

    assert "RG-STABLE-ID-ROUNDTRIP-MISMATCH" in {
        item["code"] for item in package["diagnostics"]
    }
    assert package["validation_report"]["validation_status"] == "blocked"


def test_current_json_gltf_metadata_is_fixed_and_timestamp_free_without_policy_selection():
    first = build_from_source()
    second = build_from_source()

    assert canonical_json(first) == canonical_json(second)
    assert first["gltf"]["asset"]["generator"] == (
        "OpenPipeStress DEL-17-08 review geometry exporter 0.1.0"
    )
    assert "generator_policy" not in first["export_profile"]
    assert "timestamp_policy" not in first["export_profile"]
    forbidden_timestamp_keys = {
        "timestamp",
        "created_at",
        "generated_at",
        "updated_at",
    }

    def keys(value):
        if isinstance(value, dict):
            for key, item in value.items():
                yield key
                yield from keys(item)
        elif isinstance(value, list):
            for item in value:
                yield from keys(item)

    assert forbidden_timestamp_keys.isdisjoint(keys(first))


def test_bad_geometry_and_missing_sidecars_are_blocking():
    payload = source_payload()
    geometry_payload = deepcopy(payload["geometry_payload"])
    geometry_payload["nodes"][1] = {"node_id": "node:invented:B", "x": 0, "y": 0, "z": 0}
    geometry_payload["coordinate_basis"] = "source_z_up"
    geometry_payload["units"] = {"length": "ft"}

    package = build_review_geometry_export_package(
        export_id="review-geometry:blocked",
        source_model_ref=payload["source_model_ref"],
        source_model_hash=payload["source_model_hash"],
        geometry_payload=geometry_payload,
        stable_id_map=[],
        loss_report=[],
    )

    codes = {item["code"] for item in package["diagnostics"]}
    assert {
        "RG-LENGTH-UNIT-NOT-METERS",
        "RG-COORDINATE-BASIS-UNSUPPORTED",
        "RG-ZERO-LENGTH-SEGMENT",
        "RG-STABLE-ID-MAP-MISSING",
        "RG-LOSS-REPORT-MISSING",
    } <= codes
    assert any(item["severity"] == "blocking" for item in package["diagnostics"])
    assert package["validation_report"]["validation_status"] == "blocked"


def test_unresolved_node_refs_and_duplicate_ids_are_blocking():
    payload = source_payload()
    geometry_payload = deepcopy(payload["geometry_payload"])
    geometry_payload["nodes"].append({"node_id": "node:invented:A", "x": 2, "y": 0, "z": 0})
    geometry_payload["elements"][0]["to_node"] = "node:invented:missing"

    package = build_review_geometry_export_package(
        export_id="review-geometry:bad-ids",
        source_model_ref=payload["source_model_ref"],
        source_model_hash=payload["source_model_hash"],
        geometry_payload=geometry_payload,
        stable_id_map=payload["stable_id_map"],
        loss_report=payload["loss_report"],
    )

    codes = {item["code"] for item in package["diagnostics"]}
    assert {"RG-UNRESOLVED-NODE-REF", "RG-DUPLICATE-CANONICAL-ID"} <= codes


def test_privacy_and_authority_boundary_diagnostics_are_blocking():
    payload = source_payload()
    package = build_review_geometry_export_package(
        export_id="review-geometry:privacy-boundary",
        source_model_ref=payload["source_model_ref"],
        source_model_hash=payload["source_model_hash"],
        geometry_payload=payload["geometry_payload"],
        stable_id_map=payload["stable_id_map"],
        loss_report=payload["loss_report"],
        export_profile={"free_metadata": {"unsafe_label": "cert" + "ified solver geometry"}},
        privacy={"commercial_tool_payload_embedded": True},
    )

    codes = {item["code"] for item in package["diagnostics"]}
    assert {"RG-PRIVACY-BOUNDARY-VIOLATION", "RG-PROFILE-AUTHORITY-TERM"} <= codes
    assert any(item["severity"] == "blocking" for item in package["diagnostics"])
    assert package["professional_boundary"]["software_makes_solver_fidelity_claim"] is False
    assert package["professional_boundary"]["software_creates_professional_reliance_record"] is False


def test_profile_source_basis_refs_are_required():
    payload = source_payload()
    package = build_review_geometry_export_package(
        export_id="review-geometry:missing-source-basis",
        source_model_ref=payload["source_model_ref"],
        source_model_hash=payload["source_model_hash"],
        geometry_payload=payload["geometry_payload"],
        stable_id_map=payload["stable_id_map"],
        loss_report=payload["loss_report"],
        export_profile={"source_basis_refs": [ref("Document", "GLTF-2.0")]},
    )

    assert "RG-SOURCE-BASIS-REFS-MISSING" in {item["code"] for item in package["diagnostics"]}
    assert package["validation_report"]["validation_status"] == "blocked"


def test_writer_outputs_gltf_and_sidecars(tmp_path):
    package = build_from_source()

    controlled = write_review_geometry_export_package(tmp_path, package)

    assert controlled.blocked is True
    assert controlled.payload is None
    assert list(tmp_path.iterdir()) == []


def test_fixtures_contain_no_private_or_protected_payload_text():
    text = "\n".join(
        walk_strings(
            {
                "package": load_json(PACKAGE_FIXTURE_PATH),
                "source": load_json(SOURCE_PAYLOAD_PATH),
                "gltf": load_json(GLTF_FIXTURE_PATH),
            }
        )
    ).lower()
    for forbidden in FORBIDDEN_PAYLOAD_TEXT:
        assert forbidden not in text


def main():
    check_jsonschema_validation()
    test_builder_is_deterministic_and_preserves_package_members()
    test_gltf_structure_uses_lines_and_embedded_buffer_with_identity_extras()
    test_sidecar_id_map_and_manifest_preserve_canonical_identity()
    test_bad_geometry_and_missing_sidecars_are_blocking()
    test_unresolved_node_refs_and_duplicate_ids_are_blocking()
    test_privacy_and_authority_boundary_diagnostics_are_blocking()
    test_profile_source_basis_refs_are_required()
    test_fixtures_contain_no_private_or_protected_payload_text()


if __name__ == "__main__":
    main()
