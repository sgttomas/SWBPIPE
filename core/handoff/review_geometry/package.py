"""glTF review-geometry export package builder for DEL-17-08.

This module emits a deliberately narrow glTF JSON review artifact for invented
centerline segments. It records stable identity, sidecar mappings, loss
reporting, diagnostics, provenance, privacy, and professional-boundary notices.
It does not emit binary GLB, surface/tube geometry, solver-fidelity geometry,
target compatibility evidence, formal validation evidence, or professional
reliance records.
"""

from __future__ import annotations

import base64
from copy import deepcopy
import hashlib
import json
from pathlib import Path
import struct
from typing import Any, Mapping

from core.handoff.external_prover.authority_boundary import (
    contains_prohibited_authority_term,
)


REVIEW_GEOMETRY_EXPORT_VERSION = "0.1.0"
GLTF_VERSION = "2.0"
GLTF_GENERATOR = "OpenPipeStress DEL-17-08 review geometry exporter 0.1.0"
REQUIRED_SOURCE_BASIS_REFS = {
    ("Deliverable", "DEL-17-01"),
    ("Deliverable", "DEL-17-02"),
    ("Document", "GLTF-2.0"),
}
LOSS_CATEGORIES = {
    "exported",
    "omitted",
    "approximated",
    "delegated",
    "unsupported",
    "tbd",
}

ENGINE_PROVENANCE = {
    "source_name": "OpenPipeStress DEL-17-08 review geometry export package builder",
    "source_location": "core/handoff/review_geometry/package.py",
    "source_license": "project-governed",
    "contributor": "OpenPipeStress Type 2 worker",
    "contributor_certification": "implementation-only-no-professional-claim",
    "redistribution_status": "public_permissive",
    "review_status": "machine_checked",
    "privacy_classification": "public_metadata",
}

PROFESSIONAL_BOUNDARY = {
    "human_review_required": True,
    "supports_visual_review": True,
    "supports_identity_correlation": True,
    "software_makes_release_claim": False,
    "software_makes_target_compatibility_claim": False,
    "software_makes_solver_fidelity_claim": False,
    "software_makes_analysis_fidelity_claim": False,
    "software_makes_solver_validation_claim": False,
    "software_makes_compliance_claim": False,
    "software_makes_certification_claim": False,
    "software_makes_sealing_claim": False,
    "software_makes_approval_claim": False,
    "software_creates_professional_reliance_record": False,
}

DEFAULT_PRIVACY = {
    "classification": "invented_public_example",
    "local_only": True,
    "telemetry_allowed": False,
    "private_payload_embedded": False,
    "protected_payload_embedded": False,
    "commercial_tool_payload_embedded": False,
    "redaction_refs": [],
}

DEFAULT_BOUNDARY_NOTES = [
    "glTF review geometry is visual review support only.",
    "The artifact is not solver-fidelity geometry, analysis-fidelity geometry, target compatibility evidence, code compliance evidence, or professional reliance evidence.",
    "v1 emits JSON .gltf line-mode centerline geometry with embedded buffer data; GLB and surface/tube geometry remain future work.",
    "The sidecar ID map remains authoritative for audit correlation even when glTF extras carry direct identity metadata.",
]


def build_review_geometry_export_package(
    *,
    export_id: str,
    source_model_ref: Mapping[str, Any],
    source_model_hash: Mapping[str, Any] | str,
    geometry_payload: Mapping[str, Any],
    stable_id_map: list[Mapping[str, Any]],
    loss_report: list[Mapping[str, Any]],
    export_profile: Mapping[str, Any] | None = None,
    validation_checks: list[Mapping[str, Any]] | None = None,
    diagnostics: list[Mapping[str, Any]] | None = None,
    privacy: Mapping[str, Any] | None = None,
    provenance: Mapping[str, Any] | None = None,
    boundary_notes: list[str] | None = None,
) -> dict[str, Any]:
    """Build a deterministic glTF review-geometry export package."""

    package_ref = _ref("ReviewGeometryExportPackage", export_id)
    source_ref = deepcopy(dict(source_model_ref))
    provenance_record = deepcopy(dict(provenance or ENGINE_PROVENANCE))
    privacy_record = _privacy(privacy)
    source_hash = _source_model_hash(source_model_hash, source_ref)
    notes = list(boundary_notes or DEFAULT_BOUNDARY_NOTES)
    profile = _export_profile(export_profile, notes)
    payload = _geometry_payload(geometry_payload, provenance_record)
    gltf = render_review_geometry_gltf(payload, profile)
    artifact = {
        "artifact_id": f"{export_id}:model.gltf",
        "artifact_kind": "gltf_json_review_geometry",
        "path": "model.gltf",
        "gltf_version": gltf.get("asset", {}).get("version", "unknown"),
        "primitive_mode": "LINES",
        "scene_count": len(gltf.get("scenes", [])),
        "node_count": len(gltf.get("nodes", [])),
        "mesh_count": len(gltf.get("meshes", [])),
        "buffer_count": len(gltf.get("buffers", [])),
        "embedded_buffer": True,
        "hash": _checksum(gltf, _ref("ReviewGeometryMember", f"{export_id}:model_gltf"), "review_geometry_gltf_json"),
    }
    normalized_stable_id_map = _stable_id_map(stable_id_map, provenance_record)
    normalized_loss_report = _loss_report(loss_report, package_ref, provenance_record)
    normalized_diagnostics = _stable(
        [deepcopy(dict(item)) for item in diagnostics or []]
        + (
            [
                _diagnostic(
                    "RG-LOSS-REPORT-MISSING",
                    "blocking",
                    "UNSUPPORTED_BEHAVIOR_WARNING",
                    "Review geometry package has no supplied loss report entries.",
                    "Supply explicit exported, omitted, approximated, delegated, unsupported, or tbd loss records.",
                    [package_ref],
                )
            ]
            if not loss_report
            else []
        )
        + diagnostics_for_review_geometry_export_package(
            geometry_payload=payload,
            gltf=gltf,
            stable_id_map=normalized_stable_id_map,
            loss_report=normalized_loss_report,
            export_profile=profile,
            privacy=privacy_record,
            package_ref=package_ref,
        )
    )
    validation_report = _validation_report(validation_checks, normalized_diagnostics, provenance_record)
    checksums = {
        "model_gltf": artifact["hash"],
        "stable_id_map": _checksum(
            normalized_stable_id_map,
            _ref("ReviewGeometryMember", f"{export_id}:id_map"),
            "review_geometry_id_map",
        ),
        "loss_report": _checksum(
            normalized_loss_report,
            _ref("ReviewGeometryMember", f"{export_id}:loss_report"),
            "review_geometry_loss_report",
        ),
        "validation_report": _checksum(
            validation_report,
            _ref("ReviewGeometryMember", f"{export_id}:validation_report"),
            "review_geometry_validation_report",
        ),
        "diagnostics": _checksum(
            normalized_diagnostics,
            _ref("ReviewGeometryMember", f"{export_id}:diagnostics"),
            "review_geometry_diagnostics",
        ),
    }
    manifest_seed = {
        "manifest_id": f"{export_id}:manifest",
        "source_model_ref": source_ref,
        "source_model_hash": source_hash,
        "export_profile_ref": _ref("ExportProfile", profile["profile_id"]),
        "gltf_artifact": artifact,
        "boundary_notes": notes,
        "member_hashes": checksums,
        "diagnostics": normalized_diagnostics,
    }
    checksums["manifest"] = _checksum(
        manifest_seed,
        _ref("ReviewGeometryMember", f"{export_id}:manifest"),
        "review_geometry_manifest",
    )
    manifest = {
        "manifest_id": f"{export_id}:manifest",
        "source_model_ref": source_ref,
        "source_model_hash": source_hash,
        "export_profile_ref": _ref("ExportProfile", profile["profile_id"]),
        "gltf_artifact": artifact,
        "package_members": _package_members(export_id, checksums),
        "checksums": _stable(list(checksums.values()) + [source_hash]),
        "boundary_notes": notes,
        "diagnostics": normalized_diagnostics,
    }
    package = {
        "schema_version": REVIEW_GEOMETRY_EXPORT_VERSION,
        "deliverable_id": "DEL-17-08",
        "package_id": "PKG-17",
        "scope_items": ["SOW-030", "SOW-074"],
        "objectives": ["OBJ-009", "OBJ-017"],
        "export_id": export_id,
        "package_status": "review_geometry_gltf_export_foundation",
        "source_model_ref": source_ref,
        "source_model_hash": source_hash,
        "export_profile": profile,
        "manifest": manifest,
        "geometry_payload": payload,
        "gltf": gltf,
        "stable_id_map": normalized_stable_id_map,
        "loss_report": normalized_loss_report,
        "validation_report": validation_report,
        "diagnostics": normalized_diagnostics,
        "privacy": privacy_record,
        "provenance": provenance_record,
        "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
    }
    return _sort(package)


def render_review_geometry_gltf(
    geometry_payload: Mapping[str, Any],
    export_profile: Mapping[str, Any] | None = None,
) -> dict[str, Any]:
    """Render deterministic JSON glTF line-mode centerline review geometry."""

    profile = export_profile or {}
    nodes_by_id = {str(node.get("node_id")): node for node in _list(geometry_payload.get("nodes"))}
    elements = sorted(_list(geometry_payload.get("elements")), key=lambda item: str(item.get("element_id", "")))
    buffer_bytes = bytearray()
    buffer_views: list[dict[str, Any]] = []
    accessors: list[dict[str, Any]] = []
    gltf_nodes: list[dict[str, Any]] = []
    meshes: list[dict[str, Any]] = []
    scene_nodes: list[int] = []

    for index, element in enumerate(elements):
        from_node = nodes_by_id.get(str(element.get("from_node")))
        to_node = nodes_by_id.get(str(element.get("to_node")))
        if not from_node or not to_node:
            continue
        positions = [
            float(from_node.get("x", 0.0)),
            float(from_node.get("y", 0.0)),
            float(from_node.get("z", 0.0)),
            float(to_node.get("x", 0.0)),
            float(to_node.get("y", 0.0)),
            float(to_node.get("z", 0.0)),
        ]
        byte_offset = len(buffer_bytes)
        buffer_bytes.extend(struct.pack("<6f", *positions))
        buffer_view_index = len(buffer_views)
        accessor_index = len(accessors)
        mesh_index = len(meshes)
        node_index = len(gltf_nodes)
        buffer_views.append({"buffer": 0, "byteOffset": byte_offset, "byteLength": 24, "target": 34962})
        accessors.append(
            {
                "bufferView": buffer_view_index,
                "byteOffset": 0,
                "componentType": 5126,
                "count": 2,
                "type": "VEC3",
                "min": [
                    min(positions[0], positions[3]),
                    min(positions[1], positions[4]),
                    min(positions[2], positions[5]),
                ],
                "max": [
                    max(positions[0], positions[3]),
                    max(positions[1], positions[4]),
                    max(positions[2], positions[5]),
                ],
            }
        )
        primitive = {
            "attributes": {"POSITION": accessor_index},
            "mode": 1,
            "extras": {
                "openpipestress": _identity_metadata(
                    canonical_ref=_ref("CenterlineElement", str(element.get("element_id", f"element:{index}"))),
                    target_ref=_ref("GltfPrimitive", f"mesh:{mesh_index}:primitive:0"),
                    role="centerline_segment_primitive",
                )
            },
        }
        meshes.append(
            {
                "name": str(element.get("target_name", f"ops_centerline_segment_{index + 1:03d}")),
                "primitives": [primitive],
            }
        )
        gltf_nodes.append(
            {
                "name": str(element.get("target_name", f"ops_centerline_segment_{index + 1:03d}")),
                "mesh": mesh_index,
                "extras": {
                    "openpipestress": _identity_metadata(
                        canonical_ref=_ref("CenterlineElement", str(element.get("element_id", f"element:{index}"))),
                        target_ref=_ref("GltfNode", str(node_index)),
                        role="centerline_segment_node",
                    )
                },
            }
        )
        scene_nodes.append(node_index)

    encoded = base64.b64encode(bytes(buffer_bytes)).decode("ascii")
    return _sort(
        {
            "asset": {
                "version": GLTF_VERSION,
                "generator": GLTF_GENERATOR,
                "extras": {
                    "openpipestress": {
                        "profile_id": profile.get("profile_id", "ops.review_geometry.gltf_lines"),
                        "artifact_role": "visual_review_geometry_only",
                        "linear_unit": "m",
                        "coordinate_basis": "gltf_y_up_right_handed_meters",
                        "professional_boundary": "not_solver_geometry_not_analysis_fidelity_not_professional_reliance",
                    }
                },
            },
            "scene": 0,
            "scenes": [{"name": "OpenPipeStress review geometry", "nodes": scene_nodes}],
            "nodes": gltf_nodes,
            "meshes": meshes,
            "buffers": [
                {
                    "uri": "data:application/octet-stream;base64," + encoded,
                    "byteLength": len(buffer_bytes),
                }
            ],
            "bufferViews": buffer_views,
            "accessors": accessors,
        }
    )


def diagnostics_for_review_geometry_export_package(
    *,
    geometry_payload: Mapping[str, Any],
    gltf: Mapping[str, Any],
    stable_id_map: list[Mapping[str, Any]],
    loss_report: list[Mapping[str, Any]],
    export_profile: Mapping[str, Any],
    privacy: Mapping[str, Any],
    package_ref: Mapping[str, Any],
) -> list[dict[str, Any]]:
    """Return deterministic diagnostics for glTF review-geometry packages."""

    diagnostics: list[dict[str, Any]] = []
    units = geometry_payload.get("units", {})
    if units.get("length") != "m":
        diagnostics.append(
            _diagnostic(
                "RG-LENGTH-UNIT-NOT-METERS",
                "blocking",
                "UNIT_WARNING",
                "glTF review geometry requires source length units resolved to meters.",
                "Convert source geometry to meters or block export before writing glTF.",
                [package_ref],
            )
        )
    if geometry_payload.get("coordinate_basis") != "gltf_y_up_right_handed_meters":
        diagnostics.append(
            _diagnostic(
                "RG-COORDINATE-BASIS-UNSUPPORTED",
                "blocking",
                "ASSUMPTION_WARNING",
                "Review geometry payload does not declare the supported glTF coordinate basis.",
                "Use gltf_y_up_right_handed_meters or defer coordinate transforms to a later tranche.",
                [package_ref],
            )
        )
    nodes = _list(geometry_payload.get("nodes"))
    elements = _list(geometry_payload.get("elements"))
    if not nodes or not elements:
        diagnostics.append(
            _diagnostic(
                "RG-GEOMETRY-MISSING",
                "blocking",
                "EXPORT_BLOCKING",
                "Review geometry requires at least one node and one centerline element.",
                "Provide invented or governed node and centerline element records.",
                [package_ref],
            )
        )
    ids = [str(item.get("node_id")) for item in nodes] + [str(item.get("element_id")) for item in elements]
    duplicate_ids = sorted({item for item in ids if ids.count(item) > 1})
    if duplicate_ids:
        diagnostics.append(
            _diagnostic(
                "RG-DUPLICATE-CANONICAL-ID",
                "blocking",
                "TARGET_MAPPING_WARNING",
                "Review geometry contains duplicate canonical IDs.",
                "Use stable, unique canonical IDs before exporting review geometry.",
                [_ref("CanonicalId", duplicate_ids[0])],
            )
        )
    node_ids = {str(node.get("node_id")) for node in nodes}
    nodes_by_id = {str(node.get("node_id")): node for node in nodes}
    for element in elements:
        from_id = str(element.get("from_node"))
        to_id = str(element.get("to_node"))
        if from_id not in node_ids or to_id not in node_ids:
            diagnostics.append(
                _diagnostic(
                    "RG-UNRESOLVED-NODE-REF",
                    "blocking",
                    "TARGET_MAPPING_WARNING",
                    "Centerline element references a missing node.",
                    "Provide both endpoint nodes or omit the element with an explicit loss report entry.",
                    [_ref("CenterlineElement", str(element.get("element_id", "unknown")))],
                )
            )
            continue
        from_node = nodes_by_id[from_id]
        to_node = nodes_by_id[to_id]
        if _same_point(from_node, to_node):
            diagnostics.append(
                _diagnostic(
                    "RG-ZERO-LENGTH-SEGMENT",
                    "blocking",
                    "EXPORT_BLOCKING",
                    "Centerline element has identical endpoint coordinates.",
                    "Remove or correct zero-length review geometry before package acceptance.",
                    [_ref("CenterlineElement", str(element.get("element_id", "unknown")))],
                )
            )
    if gltf.get("asset", {}).get("version") != GLTF_VERSION:
        diagnostics.append(
            _diagnostic(
                "RG-GLTF-ASSET-VERSION-MISSING",
                "blocking",
                "EXPORT_BLOCKING",
                "Generated glTF does not declare asset.version 2.0.",
                "Emit a glTF 2.0 asset record.",
                [package_ref],
            )
        )
    if not stable_id_map:
        diagnostics.append(
            _diagnostic(
                "RG-STABLE-ID-MAP-MISSING",
                "blocking",
                "TARGET_MAPPING_WARNING",
                "Review geometry package has no sidecar stable ID map entries.",
                "Emit sidecar identity mapping for audit correlation.",
                [package_ref],
            )
        )
    for entry in loss_report:
        if entry.get("category") not in LOSS_CATEGORIES:
            diagnostics.append(
                _diagnostic(
                    "RG-LOSS-CATEGORY-UNSUPPORTED",
                    "blocking",
                    "UNSUPPORTED_BEHAVIOR_WARNING",
                    "Loss report entry uses a category outside the DEL-17-02 taxonomy.",
                    "Use exported, omitted, approximated, delegated, unsupported, or tbd.",
                    [_ref("LossReportEntry", str(entry.get("loss_id", "unknown")))],
                )
            )
    if export_profile.get("artifact_format") != "gltf_json_embedded_buffer":
        diagnostics.append(
            _diagnostic(
                "RG-ARTIFACT-FORMAT-UNSUPPORTED",
                "blocking",
                "ASSUMPTION_WARNING",
                "Review geometry profile does not preserve the v1 JSON glTF embedded-buffer scope.",
                "Keep GLB and external-buffer variants deferred to a later tranche.",
                [_ref("ExportProfile", str(export_profile.get("profile_id", "unknown")))],
            )
        )
    source_basis = _ref_pairs(export_profile.get("source_basis_refs", []))
    missing_source_basis = sorted(REQUIRED_SOURCE_BASIS_REFS - source_basis)
    if missing_source_basis:
        diagnostics.append(
            _diagnostic(
                "RG-SOURCE-BASIS-REFS-MISSING",
                "blocking",
                "PROVENANCE_WARNING",
                "Review geometry profile omits required source-basis references.",
                "Carry DEL-17-01, DEL-17-02, and GLTF-2.0 as source-basis refs.",
                [_ref("ExportProfile", str(export_profile.get("profile_id", "unknown")))],
            )
        )
    if contains_prohibited_authority_term(export_profile.get("free_metadata", {})):
        diagnostics.append(
            _diagnostic(
                "RG-PROFILE-AUTHORITY-TERM",
                "blocking",
                "IP_BOUNDARY_WARNING",
                "Export profile free metadata contains authority wording.",
                "Remove solver-fidelity, analysis-fidelity, validation, compatibility, compliance, approval, or professional-reliance wording.",
                [_ref("ExportProfile", str(export_profile.get("profile_id", "unknown")))],
            )
        )
    for field in (
        "private_payload_embedded",
        "protected_payload_embedded",
        "commercial_tool_payload_embedded",
        "telemetry_allowed",
    ):
        if privacy.get(field) is True:
            diagnostics.append(
                _diagnostic(
                    "RG-PRIVACY-BOUNDARY-VIOLATION",
                    "blocking",
                    "IP_BOUNDARY_WARNING",
                    f"Privacy field {field} is not allowed for public review geometry fixtures.",
                    "Reference private/protected/commercial content by governed metadata only.",
                    [package_ref],
                )
            )
    return _stable(diagnostics)


def canonical_json(value: Any) -> str:
    """Serialize values with deterministic JSON key ordering."""

    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def write_review_geometry_export_package(directory: str | Path, package: Mapping[str, Any]) -> None:
    """Write glTF and JSON sidecars using deterministic encodings."""

    root = Path(directory)
    root.mkdir(parents=True, exist_ok=True)
    (root / "model.gltf").write_text(canonical_json(package["gltf"]) + "\n", encoding="utf-8")
    for key, filename in (
        ("manifest", "manifest.json"),
        ("stable_id_map", "id_map.json"),
        ("loss_report", "loss_report.json"),
        ("validation_report", "validation_report.json"),
        ("diagnostics", "diagnostics.json"),
    ):
        (root / filename).write_text(canonical_json(package[key]) + "\n", encoding="utf-8")


def _geometry_payload(geometry_payload: Mapping[str, Any], provenance: Mapping[str, Any]) -> dict[str, Any]:
    payload = dict(geometry_payload)
    return {
        "payload_kind": "review_geometry_centerline_lines",
        "units": deepcopy(dict(payload.get("units", {}))),
        "coordinate_basis": str(payload.get("coordinate_basis", "TBD")),
        "nodes": deepcopy(_list(payload.get("nodes"))),
        "elements": deepcopy(_list(payload.get("elements"))),
        "omitted_entities": deepcopy(_list(payload.get("omitted_entities"))),
        "provenance": deepcopy(dict(payload.get("provenance", provenance))),
        **({"free_metadata": deepcopy(payload["free_metadata"])} if "free_metadata" in payload else {}),
    }


def _export_profile(profile: Mapping[str, Any] | None, boundary_notes: list[str]) -> dict[str, Any]:
    profile = dict(profile or {})
    return {
        "profile_id": str(profile.get("profile_id", "ops.review_geometry.gltf_lines")),
        "profile_version": str(profile.get("profile_version", REVIEW_GEOMETRY_EXPORT_VERSION)),
        "target_family": "gltf_review_geometry",
        "gltf_version_basis": GLTF_VERSION,
        "artifact_format": "gltf_json_embedded_buffer",
        "primitive_mode": "LINES",
        "geometry_scope": "centerline_segments_only",
        "unit_policy": "meters_required",
        "coordinate_policy": "gltf_y_up_right_handed_meters_only",
        "identity_policy": "gltf_extras_plus_authoritative_sidecar_id_map",
        "loss_report_policy": "mandatory_for_every_package",
        "boundary_notes": list(profile.get("boundary_notes", boundary_notes)),
        "source_basis_refs": deepcopy(
            list(
                profile.get(
                    "source_basis_refs",
                    [_ref("Deliverable", "DEL-17-01"), _ref("Deliverable", "DEL-17-02"), _ref("Document", "GLTF-2.0")],
                )
            )
        ),
        **({"free_metadata": deepcopy(profile["free_metadata"])} if "free_metadata" in profile else {}),
    }


def _stable_id_map(entries: list[Mapping[str, Any]], provenance: Mapping[str, Any]) -> list[dict[str, Any]]:
    normalized = []
    for entry in entries:
        normalized.append(
            {
                "canonical_ref": deepcopy(dict(entry.get("canonical_ref", _ref("CenterlineElement", "missing")))),
                "gltf_ref": deepcopy(dict(entry.get("gltf_ref", _ref("GltfNode", "missing")))),
                "metadata_carrier": str(entry.get("metadata_carrier", "gltf_extras_and_sidecar")),
                "mapping_status": str(entry.get("mapping_status", "mapped")),
                "loss_category": str(entry.get("loss_category", "exported")),
                "provenance": deepcopy(dict(entry.get("provenance", provenance))),
            }
        )
    return sorted(normalized, key=lambda item: canonical_json(item["canonical_ref"]))


def _loss_report(
    entries: list[Mapping[str, Any]],
    package_ref: Mapping[str, Any],
    provenance: Mapping[str, Any],
) -> list[dict[str, Any]]:
    if not entries:
        entries = [
            {
                "loss_id": "loss:TBD",
                "category": "tbd",
                "severity": "blocking",
                "affected_refs": [package_ref],
                "target_artifact_ref": package_ref,
                "reason": "Loss report entries were not supplied.",
                "source_basis_ref": _ref("Deliverable", "DEL-17-02"),
                "downstream_implication": "Review geometry package cannot be treated as complete until losses are classified.",
            }
        ]
    normalized = []
    for entry in entries:
        normalized.append(
            {
                "loss_id": str(entry.get("loss_id", "loss:TBD")),
                "category": str(entry.get("category", "tbd")),
                "severity": str(entry.get("severity", "warning")),
                "affected_refs": deepcopy(list(entry.get("affected_refs", [package_ref]))),
                "target_artifact_ref": deepcopy(dict(entry.get("target_artifact_ref", package_ref))),
                "reason": str(entry.get("reason", "TBD")),
                "source_basis_ref": deepcopy(dict(entry.get("source_basis_ref", _ref("Deliverable", "DEL-17-02")))),
                "downstream_implication": str(entry.get("downstream_implication", "Requires downstream review.")),
                "human_review_required": True,
                "provenance": deepcopy(dict(entry.get("provenance", provenance))),
            }
        )
    return sorted(normalized, key=lambda item: item["loss_id"])


def _validation_report(
    validation_checks: list[Mapping[str, Any]] | None,
    diagnostics: list[Mapping[str, Any]],
    provenance: Mapping[str, Any],
) -> dict[str, Any]:
    checks = deepcopy(list(validation_checks or []))
    blocking = [item for item in diagnostics if item.get("severity") == "blocking"]
    checks.append(
        {
            "check_id": "review-geometry-boundary-diagnostics",
            "check_status": "blocked" if blocking else "passed",
            "diagnostic_count": len(diagnostics),
            "blocking_count": len(blocking),
            "provenance": deepcopy(dict(provenance)),
        }
    )
    return {
        "validation_status": "blocked" if blocking else "passed",
        "checks": checks,
        "human_review_required": True,
        "provenance": deepcopy(dict(provenance)),
    }


def _package_members(export_id: str, checksums: Mapping[str, Mapping[str, Any]]) -> list[dict[str, Any]]:
    filenames = {
        "manifest": "manifest.json",
        "model_gltf": "model.gltf",
        "stable_id_map": "id_map.json",
        "loss_report": "loss_report.json",
        "validation_report": "validation_report.json",
        "diagnostics": "diagnostics.json",
    }
    return [
        {
            "member_id": f"{export_id}:{role}",
            "member_role": role,
            "path": filenames[role],
            "checksum": deepcopy(dict(checksums[role])),
        }
        for role in sorted(filenames)
    ]


def _identity_metadata(
    *,
    canonical_ref: Mapping[str, Any],
    target_ref: Mapping[str, Any],
    role: str,
) -> dict[str, Any]:
    return {
        "canonical_ref": deepcopy(dict(canonical_ref)),
        "target_ref": deepcopy(dict(target_ref)),
        "metadata_role": role,
        "review_boundary": "visual_review_only",
        "sidecar_authoritative": True,
    }


def _source_model_hash(source_model_hash: Mapping[str, Any] | str, source_ref: Mapping[str, Any]) -> dict[str, Any]:
    if isinstance(source_model_hash, Mapping):
        record = deepcopy(dict(source_model_hash))
        record.setdefault("algorithm", "sha256")
        record.setdefault("canonicalization", "JCS_compatible_json_payload_hash")
        record.setdefault("payload_ref", deepcopy(dict(source_ref)))
        record.setdefault("payload_scope", "source_model_hash")
        return record
    return {
        "algorithm": "sha256",
        "canonicalization": "JCS_compatible_json_payload_hash",
        "payload_ref": deepcopy(dict(source_ref)),
        "payload_scope": "source_model_hash",
        "value": str(source_model_hash),
    }


def _checksum(value: Any, payload_ref: Mapping[str, Any], payload_scope: str) -> dict[str, Any]:
    return {
        "algorithm": "sha256",
        "canonicalization": "JCS_compatible_json_payload_hash",
        "payload_ref": deepcopy(dict(payload_ref)),
        "payload_scope": payload_scope,
        "value": "sha256:" + hashlib.sha256(canonical_json(value).encode("utf-8")).hexdigest(),
    }


def _privacy(value: Mapping[str, Any] | None) -> dict[str, Any]:
    privacy = deepcopy(DEFAULT_PRIVACY)
    privacy.update(dict(value or {}))
    privacy["redaction_refs"] = deepcopy(_list(privacy.get("redaction_refs")))
    return privacy


def _diagnostic(
    code: str,
    severity: str,
    diagnostic_class: str,
    message: str,
    remediation: str,
    affected_refs: list[Mapping[str, Any]],
) -> dict[str, Any]:
    return {
        "code": code,
        "class": diagnostic_class,
        "severity": severity,
        "source": _ref("Deliverable", "DEL-17-08"),
        "affected_object": deepcopy(dict(affected_refs[0] if affected_refs else _ref("Unknown", "unknown"))),
        "message": message,
        "remediation": remediation,
        "provenance": deepcopy(ENGINE_PROVENANCE),
    }


def _same_point(first: Mapping[str, Any], second: Mapping[str, Any]) -> bool:
    return (
        float(first.get("x", 0.0)) == float(second.get("x", 0.0))
        and float(first.get("y", 0.0)) == float(second.get("y", 0.0))
        and float(first.get("z", 0.0)) == float(second.get("z", 0.0))
    )


def _ref(object_type: str, value: str) -> dict[str, str]:
    return {"object_type": object_type, "ref": value}


def _list(value: Any) -> list[Any]:
    if value is None:
        return []
    if isinstance(value, list):
        return value
    return [value]


def _ref_pairs(refs: Any) -> set[tuple[str, str]]:
    pairs = set()
    for item in _list(refs):
        if isinstance(item, Mapping):
            pairs.add((str(item.get("object_type", "")), str(item.get("ref", ""))))
    return pairs


def _stable(values: list[Any]) -> list[Any]:
    return sorted(values, key=canonical_json)


def _sort(value: Any) -> Any:
    if isinstance(value, dict):
        return {key: _sort(value[key]) for key in sorted(value)}
    if isinstance(value, list):
        return [_sort(item) for item in value]
    return value
