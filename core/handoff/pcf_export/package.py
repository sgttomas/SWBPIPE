"""Conservative PCF export package builder for DEL-17-07.

This module emits a deliberately narrow, invented-fixture PCF text package. It
records stable identity through an authoritative sidecar map and preserves
losses, diagnostics, provenance, privacy, and professional-boundary notices.
It does not assert PCF completeness, downstream import compatibility, solver
validation, code compliance, release readiness, or professional reliance.
"""

from __future__ import annotations

from copy import deepcopy
import hashlib
import json
from pathlib import Path
from typing import Any, Mapping

from core.handoff.external_prover.authority_boundary import (
    contains_prohibited_authority_term,
)


PCF_EXPORT_VERSION = "0.1.0"
REQUIRED_SOURCE_BASIS_REFS = {
    ("Deliverable", "DEL-17-01"),
    ("Deliverable", "DEL-17-02"),
    ("Document", "CAEPIPE-PCF"),
    ("Document", "PLAN-EXPORT-INTEROP"),
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
    "source_name": "OpenPipeStress DEL-17-07 conservative PCF export package builder",
    "source_location": "core/handoff/pcf_export/package.py",
    "source_license": "project-governed",
    "contributor": "OpenPipeStress Type 2 worker",
    "contributor_certification": "implementation-only-no-professional-claim",
    "redistribution_status": "public_permissive",
    "review_status": "machine_checked",
    "privacy_classification": "public_metadata",
}

PROFESSIONAL_BOUNDARY = {
    "human_review_required": True,
    "supports_export_review": True,
    "supports_identity_correlation": True,
    "software_makes_release_claim": False,
    "software_makes_target_compatibility_claim": False,
    "software_makes_solver_validation_claim": False,
    "software_makes_code_compliance_claim": False,
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
    "PCF export is a conservative invented-fixture foundation only.",
    "The package is not PCF completeness evidence, downstream import compatibility evidence, solver validation evidence, code compliance evidence, release readiness evidence, or professional reliance evidence.",
    "PCF target profile/version, hidden translator defaults, support/restraint preservation, downstream import behavior, and direct PCF stable-ID carriage remain TBD.",
    "The sidecar ID map is authoritative for audit correlation.",
]


def build_pcf_export_package(
    *,
    export_id: str,
    source_model_ref: Mapping[str, Any],
    source_model_hash: Mapping[str, Any] | str,
    pcf_payload: Mapping[str, Any],
    stable_id_map: list[Mapping[str, Any]],
    loss_report: list[Mapping[str, Any]],
    unit_system_disclosure: Mapping[str, Any] | None = None,
    export_profile: Mapping[str, Any] | None = None,
    validation_checks: list[Mapping[str, Any]] | None = None,
    diagnostics: list[Mapping[str, Any]] | None = None,
    privacy: Mapping[str, Any] | None = None,
    provenance: Mapping[str, Any] | None = None,
    boundary_notes: list[str] | None = None,
) -> dict[str, Any]:
    """Build a deterministic conservative PCF export package."""

    package_ref = _ref("PcfExportPackage", export_id)
    source_ref = deepcopy(dict(source_model_ref))
    provenance_record = deepcopy(dict(provenance or ENGINE_PROVENANCE))
    privacy_record = _privacy(privacy)
    source_hash = _source_model_hash(source_model_hash, source_ref)
    notes = list(boundary_notes or DEFAULT_BOUNDARY_NOTES)
    profile = _export_profile(export_profile, notes)
    payload = _pcf_payload(pcf_payload, provenance_record)
    unit_record = _unit_system_disclosure(
        unit_system_disclosure,
        source_model_ref=source_ref,
        target_export_units=payload.get("units", {}),
        conversion_policy="pcf_text_uses_millimeter_coordinate_and_pipe_geometry_fields_with_source_unit_disclosure",
        conversion_performed=True,
        conversion_scope=["node.coordinates", "pipe_segments.outside_diameter", "pipe_segments.wall_thickness"],
        provenance=provenance_record,
    )
    pcf_text = render_pcf_text(payload, profile)
    artifact = {
        "artifact_id": f"{export_id}:model.pcf",
        "artifact_kind": "pcf_text",
        "path": "model.pcf",
        "encoding": "ascii",
        "line_count": len(pcf_text.splitlines()),
        "hash": _checksum(
            pcf_text,
            _ref("PcfExportMember", f"{export_id}:model_pcf"),
            "pcf_text",
        ),
    }
    normalized_stable_id_map = _stable_id_map(stable_id_map, provenance_record)
    normalized_loss_report = _loss_report(loss_report, package_ref, provenance_record)
    normalized_diagnostics = _stable(
        [deepcopy(dict(item)) for item in diagnostics or []]
        + diagnostics_for_pcf_export_package(
            pcf_payload=payload,
            pcf_text=pcf_text,
            stable_id_map=normalized_stable_id_map,
            loss_report=normalized_loss_report,
            export_profile=profile,
            privacy=privacy_record,
            package_ref=package_ref,
        )
    )
    validation_report = _validation_report(
        validation_checks,
        normalized_diagnostics,
        provenance_record,
    )
    checksums = {
        "model_pcf": artifact["hash"],
        "unit_system_disclosure": _checksum(
            unit_record,
            _ref("PcfExportMember", f"{export_id}:unit_system_disclosure"),
            "pcf_unit_system_disclosure",
        ),
        "stable_id_map": _checksum(
            normalized_stable_id_map,
            _ref("PcfExportMember", f"{export_id}:id_map"),
            "pcf_id_map",
        ),
        "loss_report": _checksum(
            normalized_loss_report,
            _ref("PcfExportMember", f"{export_id}:loss_report"),
            "pcf_loss_report",
        ),
        "validation_report": _checksum(
            validation_report,
            _ref("PcfExportMember", f"{export_id}:validation_report"),
            "pcf_validation_report",
        ),
        "diagnostics": _checksum(
            normalized_diagnostics,
            _ref("PcfExportMember", f"{export_id}:diagnostics"),
            "pcf_diagnostics",
        ),
    }
    manifest_seed = {
        "manifest_id": f"{export_id}:manifest",
        "source_model_ref": source_ref,
        "source_model_hash": source_hash,
        "export_profile_ref": _ref("ExportProfile", profile["profile_id"]),
        "pcf_artifact": artifact,
        "boundary_notes": notes,
        "member_hashes": checksums,
        "diagnostics": normalized_diagnostics,
    }
    checksums["manifest"] = _checksum(
        manifest_seed,
        _ref("PcfExportMember", f"{export_id}:manifest"),
        "pcf_manifest",
    )
    manifest = {
        "manifest_id": f"{export_id}:manifest",
        "source_model_ref": source_ref,
        "source_model_hash": source_hash,
        "export_profile_ref": _ref("ExportProfile", profile["profile_id"]),
        "pcf_artifact": artifact,
        "package_members": _package_members(export_id, checksums),
        "checksums": _stable(list(checksums.values()) + [source_hash]),
        "boundary_notes": notes,
        "diagnostics": normalized_diagnostics,
    }
    package = {
        "schema_version": PCF_EXPORT_VERSION,
        "deliverable_id": "DEL-17-07",
        "package_id": "PKG-17",
        "scope_items": ["SOW-030", "SOW-074"],
        "objectives": ["OBJ-009", "OBJ-017", "OBJ-018"],
        "export_id": export_id,
        "package_status": "conservative_pcf_export_foundation",
        "source_model_ref": source_ref,
        "source_model_hash": source_hash,
        "export_profile": profile,
        "manifest": manifest,
        "unit_system_disclosure": unit_record,
        "pcf_payload": payload,
        "pcf_text": pcf_text,
        "stable_id_map": normalized_stable_id_map,
        "loss_report": normalized_loss_report,
        "validation_report": validation_report,
        "diagnostics": normalized_diagnostics,
        "privacy": privacy_record,
        "provenance": provenance_record,
        "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
    }
    return _sort(package)


def render_pcf_text(
    pcf_payload: Mapping[str, Any],
    export_profile: Mapping[str, Any] | None = None,
) -> str:
    """Render deterministic ASCII PCF text for the conservative invented subset."""

    profile = export_profile or {}
    units = pcf_payload.get("units", {})
    nodes = {str(node.get("node_id")): node for node in _list(pcf_payload.get("nodes"))}
    elements = sorted(
        _list(pcf_payload.get("pipe_segments")),
        key=lambda item: str(item.get("element_id", "")),
    )
    lines = [
        "ISOGEN-FILES ISOGEN.FLS",
        f"PIPELINE-REFERENCE {pcf_payload.get('pipeline_reference', 'OPS-INVENTED')}",
        f"UNITS-BORE {units.get('bore', 'INCH')}",
        f"UNITS-CO-ORDS {units.get('coordinates', 'MM')}",
        f"UNITS-BOLT-DIA {units.get('bolt_diameter', 'INCH')}",
        f"UNITS-BOLT-LENGTH {units.get('bolt_length', 'MM')}",
        f"UNITS-WEIGHT {units.get('weight', 'KG')}",
        f"STATUS {profile.get('support_status', 'FOR-REVIEW')}",
    ]
    for element in elements:
        start = nodes.get(str(element.get("from_node")), {})
        end = nodes.get(str(element.get("to_node")), {})
        lines.extend(
            [
                "PIPE",
                f"    COMPONENT-IDENTIFIER {element.get('target_component_identifier', element.get('element_id', 'UNKNOWN'))}",
                f"    END-POINT {_coord(start)} {element.get('nominal_size', 'TBD')}",
                f"    END-POINT {_coord(end)} {element.get('nominal_size', 'TBD')}",
                f"    ITEM-CODE {element.get('item_code', 'TBD')}",
                f"    MATERIAL {element.get('material_label', 'TBD')}",
                f"    PIPELINE-REFERENCE {pcf_payload.get('pipeline_reference', 'OPS-INVENTED')}",
            ]
        )
        if "outside_diameter" in element:
            lines.append(f"    OUTSIDE-DIAMETER {element['outside_diameter']}")
        if "wall_thickness" in element:
            lines.append(f"    WALL-THICKNESS {element['wall_thickness']}")
    lines.append("END-ISOGEN")
    text = "\n".join(str(line) for line in lines) + "\n"
    text.encode("ascii")
    return text


def diagnostics_for_pcf_export_package(
    *,
    pcf_payload: Mapping[str, Any],
    pcf_text: str,
    stable_id_map: list[Mapping[str, Any]],
    loss_report: list[Mapping[str, Any]],
    export_profile: Mapping[str, Any],
    privacy: Mapping[str, Any],
    package_ref: Mapping[str, Any],
) -> list[dict[str, Any]]:
    """Return deterministic diagnostics for conservative PCF export packages."""

    diagnostics: list[dict[str, Any]] = []
    units = pcf_payload.get("units", {})
    for key in ("bore", "coordinates", "weight"):
        if not units.get(key):
            diagnostics.append(
                _diagnostic(
                    "PCF-UNIT-MISSING",
                    "blocking",
                    "UNIT_WARNING",
                    f"PCF payload is missing explicit {key} unit policy.",
                    "Provide explicit invented unit metadata or block export.",
                    [package_ref],
                )
            )
    nodes = _list(pcf_payload.get("nodes"))
    elements = _list(pcf_payload.get("pipe_segments"))
    if not nodes or not elements:
        diagnostics.append(
            _diagnostic(
                "PCF-MODEL-SUBSET-MISSING",
                "blocking",
                "EXPORT_BLOCKING",
                "Conservative PCF export requires at least one node and one pipe segment.",
                "Provide invented node and pipe segment records or omit the export attempt.",
                [package_ref],
            )
        )
    ids = [str(item.get("node_id")) for item in nodes] + [str(item.get("element_id")) for item in elements]
    duplicate_ids = sorted({item for item in ids if ids.count(item) > 1})
    if duplicate_ids:
        diagnostics.append(
            _diagnostic(
                "PCF-DUPLICATE-CANONICAL-ID",
                "blocking",
                "TARGET_MAPPING_WARNING",
                "PCF payload contains duplicate canonical IDs.",
                "Use stable, unique canonical IDs before exporting PCF.",
                [_ref("CanonicalId", duplicate_ids[0])],
            )
        )
    node_ids = {str(node.get("node_id")) for node in nodes}
    for element in elements:
        for field in ("from_node", "to_node"):
            if str(element.get(field)) not in node_ids:
                diagnostics.append(
                    _diagnostic(
                        "PCF-UNRESOLVED-NODE-REF",
                        "blocking",
                        "TARGET_MAPPING_WARNING",
                        "Pipe segment references a missing node.",
                        "Provide both endpoint nodes or omit the segment with an explicit loss report entry.",
                        [_ref("PipeSegment", str(element.get("element_id", "unknown")))],
                    )
                )
        for required_field in ("nominal_size", "outside_diameter", "wall_thickness", "material_label"):
            if not element.get(required_field):
                diagnostics.append(
                    _diagnostic(
                        "PCF-EXPLICIT-FIELD-MISSING",
                        "blocking",
                        "EXPORT_BLOCKING",
                        f"Pipe segment is missing explicit {required_field}; hidden target defaults are not allowed.",
                        "Provide source-owned values or classify the segment as omitted, unsupported, or tbd.",
                        [_ref("PipeSegment", str(element.get("element_id", "unknown")))],
                    )
                )
    if not pcf_text.endswith("\n") or "END-ISOGEN\n" not in pcf_text:
        diagnostics.append(
            _diagnostic(
                "PCF-TEXT-STRUCTURE-INVALID",
                "blocking",
                "EXPORT_BLOCKING",
                "Generated PCF text does not end with an END-ISOGEN record.",
                "Emit deterministic text with a terminal END-ISOGEN record.",
                [package_ref],
            )
        )
    if not stable_id_map:
        diagnostics.append(
            _diagnostic(
                "PCF-STABLE-ID-MAP-MISSING",
                "blocking",
                "TARGET_MAPPING_WARNING",
                "PCF package has no sidecar stable ID map entries.",
                "Emit sidecar identity mapping for audit correlation.",
                [package_ref],
            )
        )
    categories = {entry.get("category") for entry in loss_report}
    missing_categories = LOSS_CATEGORIES - categories
    if missing_categories:
        diagnostics.append(
            _diagnostic(
                "PCF-LOSS-CATEGORY-COVERAGE-INCOMPLETE",
                "warning",
                "UNSUPPORTED_BEHAVIOR_WARNING",
                "Loss report does not include every DEL-17-02 category in the invented fixture.",
                "Include exported, omitted, approximated, delegated, unsupported, and tbd coverage in the focused fixture.",
                [package_ref],
            )
        )
    for entry in loss_report:
        if entry.get("category") not in LOSS_CATEGORIES:
            diagnostics.append(
                _diagnostic(
                    "PCF-LOSS-CATEGORY-UNSUPPORTED",
                    "blocking",
                    "UNSUPPORTED_BEHAVIOR_WARNING",
                    "Loss report entry uses a category outside the DEL-17-02 taxonomy.",
                    "Use exported, omitted, approximated, delegated, unsupported, or tbd.",
                    [_ref("LossReportEntry", str(entry.get("loss_id", "unknown")))],
                )
            )
    if export_profile.get("target_profile_version_basis") != "TBD":
        diagnostics.append(
            _diagnostic(
                "PCF-TARGET-PROFILE-OVERCLAIM",
                "blocking",
                "ASSUMPTION_WARNING",
                "PCF target profile/version is not source-confirmed in this tranche.",
                "Keep target_profile_version_basis as TBD until admitted source evidence or human authority resolves it.",
                [_ref("ExportProfile", str(export_profile.get("profile_id", "unknown")))],
            )
        )
    source_basis = _ref_pairs(export_profile.get("source_basis_refs", []))
    missing_source_basis = sorted(REQUIRED_SOURCE_BASIS_REFS - source_basis)
    if missing_source_basis:
        diagnostics.append(
            _diagnostic(
                "PCF-SOURCE-BASIS-REFS-MISSING",
                "blocking",
                "PROVENANCE_WARNING",
                "PCF profile omits required source-basis references.",
                "Carry DEL-17-01, DEL-17-02, CAEPIPE-PCF, and PLAN-EXPORT-INTEROP as source-basis refs.",
                [_ref("ExportProfile", str(export_profile.get("profile_id", "unknown")))],
            )
        )
    if contains_prohibited_authority_term(export_profile.get("free_metadata", {})):
        diagnostics.append(
            _diagnostic(
                "PCF-PROFILE-AUTHORITY-TERM",
                "blocking",
                "IP_BOUNDARY_WARNING",
                "Export profile free metadata contains authority wording.",
                "Remove compatibility, validation, compliance, approval, or professional-reliance wording.",
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
                    "PCF-PRIVACY-BOUNDARY-VIOLATION",
                    "blocking",
                    "IP_BOUNDARY_WARNING",
                    f"Privacy field {field} is not allowed for public PCF fixtures.",
                    "Reference private/protected/commercial content by governed metadata only.",
                    [package_ref],
                )
            )
    return _stable(diagnostics)


def canonical_json(value: Any) -> str:
    """Serialize values with deterministic JSON key ordering."""

    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def write_pcf_export_package(directory: str | Path, package: Mapping[str, Any]) -> None:
    """Write PCF text and JSON sidecars using deterministic encodings."""

    root = Path(directory)
    root.mkdir(parents=True, exist_ok=True)
    (root / "model.pcf").write_text(str(package["pcf_text"]), encoding="ascii")
    for key, filename in (
        ("manifest", "manifest.json"),
        ("stable_id_map", "id_map.json"),
        ("loss_report", "loss_report.json"),
        ("validation_report", "validation_report.json"),
        ("diagnostics", "diagnostics.json"),
    ):
        (root / filename).write_text(canonical_json(package[key]) + "\n", encoding="utf-8")


def _pcf_payload(pcf_payload: Mapping[str, Any], provenance: Mapping[str, Any]) -> dict[str, Any]:
    payload = dict(pcf_payload)
    return {
        "payload_kind": "conservative_pcf_pipe_subset",
        "pipeline_reference": str(payload.get("pipeline_reference", "OPS-INVENTED")),
        "units": deepcopy(dict(payload.get("units", {}))),
        "coordinate_basis": str(payload.get("coordinate_basis", "project_xyz_millimeters")),
        "nodes": deepcopy(_list(payload.get("nodes"))),
        "pipe_segments": deepcopy(_list(payload.get("pipe_segments"))),
        "omitted_entities": deepcopy(_list(payload.get("omitted_entities"))),
        "unsupported_entities": deepcopy(_list(payload.get("unsupported_entities"))),
        "tbd_entities": deepcopy(_list(payload.get("tbd_entities"))),
        "provenance": deepcopy(dict(payload.get("provenance", provenance))),
        **({"free_metadata": deepcopy(payload["free_metadata"])} if "free_metadata" in payload else {}),
    }


def _export_profile(profile: Mapping[str, Any] | None, boundary_notes: list[str]) -> dict[str, Any]:
    profile = dict(profile or {})
    return {
        "profile_id": str(profile.get("profile_id", "ops.pcf.conservative_subset")),
        "profile_version": str(profile.get("profile_version", PCF_EXPORT_VERSION)),
        "target_family": "pcf",
        "target_profile_version_basis": str(profile.get("target_profile_version_basis", "TBD")),
        "artifact_format": "ascii_pcf_text",
        "subset_scope": "invented_straight_pipe_segments_only",
        "unit_policy": "explicit_units_required_no_hidden_defaults",
        "coordinate_policy": "project_xyz_millimeters_only",
        "identity_policy": "authoritative_sidecar_id_map",
        "loss_report_policy": "mandatory_for_every_package",
        "translator_default_policy": "hidden_defaults_blocked_or_loss_reported",
        "support_restraint_policy": "unsupported_or_tbd_until_source_confirmed",
        "boundary_notes": list(profile.get("boundary_notes", boundary_notes)),
        "source_basis_refs": deepcopy(
            list(
                profile.get(
                    "source_basis_refs",
                    [
                        _ref("Deliverable", "DEL-17-01"),
                        _ref("Deliverable", "DEL-17-02"),
                        _ref("Document", "CAEPIPE-PCF"),
                        _ref("Document", "PLAN-EXPORT-INTEROP"),
                    ],
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
                "canonical_ref": deepcopy(dict(entry.get("canonical_ref", _ref("PipeSegment", "missing")))),
                "pcf_ref": deepcopy(dict(entry.get("pcf_ref", _ref("PcfRecord", "missing")))),
                "metadata_carrier": str(entry.get("metadata_carrier", "sidecar_id_map")),
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
                "downstream_implication": "PCF package cannot be treated as complete until losses are classified.",
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
            "check_id": "pcf-export-boundary-diagnostics",
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
        "model_pcf": "model.pcf",
        "unit_system_disclosure": "unit_system_disclosure.json",
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


def _unit_system_disclosure(
    value: Mapping[str, Any] | None,
    *,
    source_model_ref: Mapping[str, Any],
    target_export_units: Mapping[str, Any],
    conversion_policy: str,
    conversion_performed: bool,
    conversion_scope: list[str],
    provenance: Mapping[str, Any],
) -> dict[str, Any]:
    record = dict(value or {})
    return {
        "unit_system_ref": deepcopy(dict(record.get("unit_system_ref", _ref("UnitSystem", "unit-system:dec-018-si-dual-display")))),
        "source_model_ref": deepcopy(dict(record.get("source_model_ref", source_model_ref))),
        "storage_convention": str(record.get("storage_convention", "entered_units_preserved")),
        "model_units": _string_record(record.get("model_units", {})),
        "result_units": sorted(str(item) for item in _list(record.get("result_units", [])) if str(item)),
        "target_export_units": _string_record(record.get("target_export_units", target_export_units)),
        "conversion_policy": str(record.get("conversion_policy", conversion_policy)),
        "conversion_performed": bool(record.get("conversion_performed", conversion_performed)),
        "conversion_scope": sorted(str(item) for item in _list(record.get("conversion_scope", conversion_scope))),
        "decision_basis_refs": deepcopy(
            list(record.get("decision_basis_refs", [_ref("Decision", "DEC-018"), _ref("Deliverable", "DEL-02-02")]))
        ),
        "protected_content_included": False,
        "private_payload_included": False,
        "provenance": deepcopy(dict(record.get("provenance", provenance))),
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
        "source": _ref("Deliverable", "DEL-17-07"),
        "affected_object": deepcopy(dict(affected_refs[0] if affected_refs else _ref("Unknown", "unknown"))),
        "message": message,
        "remediation": remediation,
        "provenance": deepcopy(ENGINE_PROVENANCE),
    }


def _coord(node: Mapping[str, Any]) -> str:
    return f"{_num(node.get('x', 0))} {_num(node.get('y', 0))} {_num(node.get('z', 0))}"


def _num(value: Any) -> str:
    number = float(value)
    if number.is_integer():
        return str(int(number))
    return f"{number:.6f}".rstrip("0").rstrip(".")


def _ref(object_type: str, value: str) -> dict[str, str]:
    return {"object_type": object_type, "ref": value}


def _list(value: Any) -> list[Any]:
    if value is None:
        return []
    if isinstance(value, list):
        return value
    return [value]


def _string_record(value: Any) -> dict[str, str]:
    if not isinstance(value, Mapping):
        return {}
    return {str(key): str(value[key]) for key in sorted(value)}


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
