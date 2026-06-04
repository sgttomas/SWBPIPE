"""Bounded CAEPIPE MBF export package builder for DEL-17-04.

This module renders a deliberately narrow, invented smoke subset into a stable
text package. It records sidecar IDs, loss reporting, diagnostics, provenance,
privacy, and professional-boundary notices. It does not invoke CAEPIPE, parse
CAEPIPE output, select a code basis, or claim target compatibility.
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


CAEPIPE_MBF_EXPORT_VERSION = "0.1.0"
TARGET_VERSION_TBD = "TBD-17-01-001"
RECORD_SUBSET_TBD = "TBD-17-01-002"
DIRECT_STABLE_ID_TBD = "TBD-17-01-003"
REQUIRED_PROFILE_TBD_REFS = {
    TARGET_VERSION_TBD,
    RECORD_SUBSET_TBD,
    DIRECT_STABLE_ID_TBD,
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
    "source_name": "OpenPipeStress DEL-17-04 CAEPIPE MBF export package builder",
    "source_location": "core/handoff/caepipe_mbf/package.py",
    "source_license": "project-governed",
    "contributor": "OpenPipeStress Type 2 worker",
    "contributor_certification": "implementation-only-no-professional-claim",
    "redistribution_status": "public_permissive",
    "review_status": "machine_checked",
    "privacy_classification": "public_metadata",
}

PROFESSIONAL_BOUNDARY = {
    "human_review_required": True,
    "supports_interchange_smoke_testing": True,
    "software_makes_release_claim": False,
    "software_makes_caepipe_compatibility_claim": False,
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
    "CAEPIPE MBF output is a bounded text handoff foundation.",
    "Target version, record subset closure, and direct MBF stable-ID carrying remain TBD.",
    "Sidecar stable-ID mapping is the conservative default for this tranche.",
    "Package evidence does not assert CAEPIPE compatibility, solver validation, release readiness, code compliance, or professional reliance.",
]


def build_caepipe_mbf_export_package(
    *,
    export_id: str,
    source_model_ref: Mapping[str, Any],
    source_model_hash: Mapping[str, Any] | str,
    model_payload: Mapping[str, Any],
    stable_id_map: list[Mapping[str, Any]],
    loss_report: list[Mapping[str, Any]],
    export_profile: Mapping[str, Any] | None = None,
    diagnostics: list[Mapping[str, Any]] | None = None,
    validation_checks: list[Mapping[str, Any]] | None = None,
    privacy: Mapping[str, Any] | None = None,
    provenance: Mapping[str, Any] | None = None,
    boundary_notes: list[str] | None = None,
) -> dict[str, Any]:
    """Build a deterministic CAEPIPE MBF export package envelope."""

    package_ref = _ref("CaePipeMbfExportPackage", export_id)
    source_ref = deepcopy(dict(source_model_ref))
    provenance_record = deepcopy(dict(provenance or ENGINE_PROVENANCE))
    privacy_record = _privacy(privacy)
    source_hash = _source_model_hash(source_model_hash, source_ref)
    notes = list(boundary_notes or DEFAULT_BOUNDARY_NOTES)
    profile = _export_profile(export_profile, notes)
    payload = _model_payload(model_payload, provenance_record)
    mbf_text = render_caepipe_mbf_text(payload, profile)
    normalized_stable_id_map = _stable_id_map(stable_id_map, provenance_record)
    normalized_loss_report = _loss_report(loss_report, package_ref, provenance_record)
    normalized_diagnostics = _stable(
        [deepcopy(dict(item)) for item in diagnostics or []]
        + diagnostics_for_caepipe_mbf_export_package(
            model_payload=payload,
            mbf_text=mbf_text,
            stable_id_map=normalized_stable_id_map,
            loss_report=normalized_loss_report,
            export_profile=profile,
            privacy=privacy_record,
            package_ref=package_ref,
        )
    )
    validation_report = _validation_report(
        validation_checks=validation_checks,
        diagnostics=normalized_diagnostics,
        provenance=provenance_record,
    )
    checksums = {
        "mbf_text": _text_checksum(mbf_text, _ref("CaePipeMbfMember", f"{export_id}:mbf_text")),
        "stable_id_map": _checksum(
            normalized_stable_id_map,
            _ref("CaePipeMbfMember", f"{export_id}:stable_id_map"),
            "caepipe_mbf_stable_id_map",
        ),
        "loss_report": _checksum(
            normalized_loss_report,
            _ref("CaePipeMbfMember", f"{export_id}:loss_report"),
            "caepipe_mbf_loss_report",
        ),
        "validation_report": _checksum(
            validation_report,
            _ref("CaePipeMbfMember", f"{export_id}:validation_report"),
            "caepipe_mbf_validation_report",
        ),
        "diagnostics": _checksum(
            normalized_diagnostics,
            _ref("CaePipeMbfMember", f"{export_id}:diagnostics"),
            "caepipe_mbf_diagnostics",
        ),
    }
    manifest_seed = {
        "manifest_id": f"{export_id}:manifest",
        "source_model_ref": source_ref,
        "source_model_hash": source_hash,
        "export_profile_ref": _ref("ExportProfile", profile["profile_id"]),
        "source_basis_refs": deepcopy(profile["source_basis_refs"]),
        "boundary_notes": notes,
        "diagnostics": normalized_diagnostics,
        "member_hashes": checksums,
    }
    checksums["manifest"] = _checksum(
        manifest_seed,
        _ref("CaePipeMbfMember", f"{export_id}:manifest"),
        "caepipe_mbf_manifest",
    )

    manifest = {
        "manifest_id": f"{export_id}:manifest",
        "source_model_ref": source_ref,
        "source_model_hash": source_hash,
        "export_profile_ref": _ref("ExportProfile", profile["profile_id"]),
        "package_members": _package_members(export_id, checksums),
        "checksums": _stable(list(checksums.values()) + [source_hash]),
        "source_basis_refs": deepcopy(profile["source_basis_refs"]),
        "boundary_notes": notes,
        "diagnostics": normalized_diagnostics,
    }
    package = {
        "schema_version": CAEPIPE_MBF_EXPORT_VERSION,
        "deliverable_id": "DEL-17-04",
        "package_id": "PKG-17",
        "scope_items": ["SOW-030", "SOW-074", "SOW-075"],
        "objectives": ["OBJ-009", "OBJ-017", "OBJ-018"],
        "export_id": export_id,
        "package_status": "caepipe_mbf_export_foundation",
        "export_profile": profile,
        "manifest": manifest,
        "mbf_text": mbf_text,
        "model_payload": payload,
        "stable_id_map": normalized_stable_id_map,
        "loss_report": normalized_loss_report,
        "validation_report": validation_report,
        "diagnostics": normalized_diagnostics,
        "privacy": privacy_record,
        "provenance": provenance_record,
        "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
    }
    return _sort(package)


def render_caepipe_mbf_text(model_payload: Mapping[str, Any], export_profile: Mapping[str, Any] | None = None) -> str:
    """Render a stable, minimal MBF-style text payload from invented records."""

    profile = export_profile or {}
    header = [
        "$ OpenPipeStress invented CAEPIPE MBF smoke subset",
        "$ Source basis: DEL-17-01 / DEL-17-02 / DEL-17-04",
        f"$ Target version basis: {profile.get('target_version_basis', 'TBD-17-01-001')}",
        "$ Canonical IDs are preserved in sidecar mapping, not asserted as direct MBF fields",
    ]
    lines = header + ["UNITS"]
    units = model_payload.get("units", {})
    lines.append(_record("UNIT", [units.get("length", "m"), units.get("force", "N"), units.get("temperature", "C")]))
    lines.append("NODES")
    for node in sorted(_list(model_payload.get("nodes")), key=lambda item: str(item.get("node_id", ""))):
        lines.append(
            _record(
                "NODE",
                [
                    node.get("target_id"),
                    _num(node.get("x", 0)),
                    _num(node.get("y", 0)),
                    _num(node.get("z", 0)),
                ],
            )
        )
    lines.append("ELEMENTS")
    for element in sorted(_list(model_payload.get("elements")), key=lambda item: str(item.get("element_id", ""))):
        lines.append(
            _record(
                "PIPE",
                [
                    element.get("target_id"),
                    element.get("from_node"),
                    element.get("to_node"),
                    element.get("section_ref", "SECTION-TBD"),
                    element.get("material_ref", "MATERIAL-TBD"),
                ],
            )
        )
    lines.append("SUPPORTS")
    for support in sorted(_list(model_payload.get("supports")), key=lambda item: str(item.get("support_id", ""))):
        lines.append(
            _record(
                "SUPPORT",
                [
                    support.get("target_id"),
                    support.get("node"),
                    support.get("support_kind", "ANCHOR"),
                ],
            )
        )
    lines.append("LOAD_CASES")
    for load_case in sorted(_list(model_payload.get("load_cases")), key=lambda item: str(item.get("load_case_id", ""))):
        lines.append(_record("LOADCASE", [load_case.get("target_id"), load_case.get("load_kind", "TBD")]))
    lines.append("END")
    return canonical_text("\n".join(lines))


def diagnostics_for_caepipe_mbf_export_package(
    *,
    model_payload: Mapping[str, Any],
    mbf_text: str,
    stable_id_map: list[Mapping[str, Any]],
    loss_report: list[Mapping[str, Any]],
    export_profile: Mapping[str, Any],
    privacy: Mapping[str, Any],
    package_ref: Mapping[str, Any],
) -> list[dict[str, Any]]:
    """Return deterministic diagnostics for CAEPIPE MBF package boundaries."""

    diagnostics: list[dict[str, Any]] = []
    if not _list(model_payload.get("nodes")) or not _list(model_payload.get("elements")):
        diagnostics.append(
            _diagnostic(
                "MBF-SMOKE-SUBSET-MISSING",
                "blocking",
                "EXPORT_BLOCKING",
                "CAEPIPE MBF smoke subset requires at least one node and one pipe element.",
                "Provide invented node and pipe element records or record an explicit omitted/unsupported loss.",
                [package_ref],
            )
        )
    if not _is_ascii(mbf_text):
        diagnostics.append(
            _diagnostic(
                "MBF-TEXT-NON-ASCII",
                "blocking",
                "EXPORT_BLOCKING",
                "Rendered MBF text contains non-ASCII characters.",
                "Use ASCII-safe invented labels for MBF text output.",
                [package_ref],
            )
        )
    if not stable_id_map:
        diagnostics.append(
            _diagnostic(
                "MBF-STABLE-ID-SIDECAR-MISSING",
                "blocking",
                "TARGET_MAPPING_WARNING",
                "CAEPIPE MBF package has no sidecar stable ID map entries.",
                "Use sidecar mapping while direct MBF stable-ID carrying remains TBD-17-01-003.",
                [package_ref],
            )
        )
    if not loss_report:
        diagnostics.append(
            _diagnostic(
                "MBF-LOSS-REPORT-MISSING",
                "blocking",
                "UNSUPPORTED_BEHAVIOR_WARNING",
                "CAEPIPE MBF package has no loss report entries.",
                "Record exported, omitted, approximated, delegated, unsupported, or tbd behavior explicitly.",
                [package_ref],
            )
        )
    if export_profile.get("stable_id_policy") != "sidecar_mapping_until_direct_mbf_carrier_confirmed":
        diagnostics.append(
            _diagnostic(
                "MBF-STABLE-ID-POLICY-UNSAFE",
                "blocking",
                "TARGET_MAPPING_WARNING",
                "CAEPIPE MBF profile does not preserve the sidecar-first stable ID policy.",
                "Keep direct MBF ID carrying as TBD-17-01-003 until source-confirmed.",
                [_ref("ExportProfile", str(export_profile.get("profile_id", "unknown")))],
            )
        )
    if str(export_profile.get("target_version_basis", "")).strip() != TARGET_VERSION_TBD:
        diagnostics.append(
            _diagnostic(
                "MBF-TARGET-VERSION-BASIS-UNSAFE",
                "blocking",
                "TBD",
                "CAEPIPE MBF target version basis is not carried as the open source-confirmation TBD.",
                f"Use {TARGET_VERSION_TBD} until a later guarded tranche admits source-confirmed target-version evidence.",
                [_ref("ExportProfile", str(export_profile.get("profile_id", "unknown")))],
            )
        )
    if str(export_profile.get("record_subset_basis", "")).strip() != RECORD_SUBSET_TBD:
        diagnostics.append(
            _diagnostic(
                "MBF-RECORD-SUBSET-BASIS-UNSAFE",
                "blocking",
                "TBD",
                "CAEPIPE MBF record subset basis is not carried as the open source-confirmation TBD.",
                f"Use {RECORD_SUBSET_TBD} until a later guarded tranche admits source-confirmed record-family evidence.",
                [_ref("ExportProfile", str(export_profile.get("profile_id", "unknown")))],
            )
        )
    missing_tbd_refs = REQUIRED_PROFILE_TBD_REFS - set(_list(export_profile.get("carried_tbd_refs")))
    if missing_tbd_refs:
        diagnostics.append(
            _diagnostic(
                "MBF-CARRIED-TBD-REFS-MISSING",
                "blocking",
                "TBD",
                "CAEPIPE MBF profile does not carry all required open TBD references.",
                "Carry target version, record subset, and direct stable-ID TBD refs until each is closed by source evidence.",
                [_ref("ExportProfile", str(export_profile.get("profile_id", "unknown")))],
            )
        )
    for entry in loss_report:
        if entry.get("category") not in LOSS_CATEGORIES:
            diagnostics.append(
                _diagnostic(
                    "MBF-LOSS-CATEGORY-UNSUPPORTED",
                    "blocking",
                    "UNSUPPORTED_BEHAVIOR_WARNING",
                    "Loss report entry uses a category outside the DEL-17-02 taxonomy.",
                    "Use exported, omitted, approximated, delegated, unsupported, or tbd.",
                    [_ref("LossReportEntry", str(entry.get("loss_id", "unknown")))],
                )
            )
    unsupported_loss_refs: set[tuple[str, str]] = set()
    for entry in loss_report:
        if entry.get("category") != "unsupported":
            continue
        if entry.get("severity") == "info":
            diagnostics.append(
                _diagnostic(
                    "MBF-UNSUPPORTED-LOSS-SEVERITY-UNSAFE",
                    "blocking",
                    "UNSUPPORTED_BEHAVIOR_WARNING",
                    "Unsupported CAEPIPE MBF behavior cannot be classified as informational only.",
                    "Classify unsupported behavior as warning or blocking until target support evidence closes it.",
                    [_ref("LossReportEntry", str(entry.get("loss_id", "unknown")))],
                )
            )
        for affected_ref in _list(entry.get("affected_refs")):
            if _is_reference(affected_ref):
                unsupported_loss_refs.add(_reference_key(affected_ref))
    for unsupported_entity in _list(model_payload.get("unsupported_entities")):
        if not _is_reference(unsupported_entity):
            diagnostics.append(
                _diagnostic(
                    "MBF-UNSUPPORTED-ENTITY-REF-MISSING",
                    "blocking",
                    "UNSUPPORTED_BEHAVIOR_WARNING",
                    "Unsupported CAEPIPE MBF entity is not recorded as a stable reference.",
                    "Record unsupported entities with object_type and ref so loss reports can cover them.",
                    [package_ref],
                )
            )
            continue
        if _reference_key(unsupported_entity) not in unsupported_loss_refs:
            diagnostics.append(
                _diagnostic(
                    "MBF-UNSUPPORTED-ENTITY-LOSS-MISSING",
                    "blocking",
                    "UNSUPPORTED_BEHAVIOR_WARNING",
                    "Unsupported CAEPIPE MBF entity lacks a matching unsupported loss-report entry.",
                    "Add a loss_report entry with category unsupported and the unsupported entity in affected_refs.",
                    [unsupported_entity],
                )
            )
    if contains_prohibited_authority_term(model_payload.get("free_metadata", {})):
        diagnostics.append(
            _diagnostic(
                "MBF-PROHIBITED-AUTHORITY-TERM",
                "blocking",
                "IP_BOUNDARY_WARNING",
                "Model payload free metadata contains authority wording.",
                "Remove release, validation, compatibility, compliance, or professional-reliance wording.",
                [package_ref],
            )
        )
    if contains_prohibited_authority_term(export_profile.get("free_metadata", {})):
        diagnostics.append(
            _diagnostic(
                "MBF-PROFILE-AUTHORITY-TERM",
                "blocking",
                "IP_BOUNDARY_WARNING",
                "Export profile free metadata contains authority wording.",
                "Keep target-support and reliance decisions outside this package.",
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
                    "MBF-PRIVACY-BOUNDARY-VIOLATION",
                    "blocking",
                    "IP_BOUNDARY_WARNING",
                    f"Privacy field {field} is not allowed for public CAEPIPE MBF fixtures.",
                    "Reference private/protected/commercial content by governed metadata only.",
                    [package_ref],
                )
            )
    return _stable(diagnostics)


def canonical_json(value: Any) -> str:
    """Serialize CAEPIPE MBF package values with deterministic JSON key ordering."""

    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def canonical_text(value: str) -> str:
    """Normalize rendered MBF text for deterministic hashing and writing."""

    lines = [line.rstrip() for line in value.replace("\r\n", "\n").replace("\r", "\n").split("\n")]
    while lines and lines[-1] == "":
        lines.pop()
    return "\n".join(lines) + "\n"


def write_caepipe_mbf_export_package(directory: str | Path, package: Mapping[str, Any]) -> None:
    """Write manifest-declared MBF package members using deterministic encodings."""

    root = Path(directory)
    root.mkdir(parents=True, exist_ok=True)
    for member in package["manifest"]["package_members"]:
        role = str(member["member_role"])
        member_path = Path(str(member["path"]))
        if member_path.is_absolute() or ".." in member_path.parts:
            raise ValueError(f"Unsafe CAEPIPE MBF package member path: {member_path}")
        destination = root / member_path
        destination.parent.mkdir(parents=True, exist_ok=True)

        if role == "mbf_text":
            destination.write_text(str(package["mbf_text"]), encoding="ascii")
            continue
        if role not in package:
            raise ValueError(f"Unknown CAEPIPE MBF package member role: {role}")
        destination.write_text(canonical_json(package[role]) + "\n", encoding="utf-8")


def _export_profile(profile: Mapping[str, Any] | None, boundary_notes: list[str]) -> dict[str, Any]:
    profile = dict(profile or {})
    return {
        "profile_id": str(profile.get("profile_id", "ops.caepipe_mbf.smoke_tbd")),
        "profile_version": str(profile.get("profile_version", CAEPIPE_MBF_EXPORT_VERSION)),
        "target_family": "caepipe_mbf",
        "target_version_basis": str(profile.get("target_version_basis", TARGET_VERSION_TBD)),
        "record_subset_basis": str(profile.get("record_subset_basis", RECORD_SUBSET_TBD)),
        "stable_id_policy": "sidecar_mapping_until_direct_mbf_carrier_confirmed",
        "unit_policy": str(profile.get("unit_policy", "explicit_units_record_required")),
        "loss_report_policy": "mandatory_for_every_package",
        "external_execution_policy": "not_invoked_by_this_package",
        "source_basis_refs": deepcopy(
            list(
                profile.get(
                    "source_basis_refs",
                    [
                        _ref("Deliverable", "DEL-17-01"),
                        _ref("Deliverable", "DEL-17-02"),
                        _ref("Deliverable", "DEL-17-03"),
                        _ref("SourceID", "CAEPIPE-IMPORT-MBF"),
                        _ref("SourceID", "CAEPIPE-EXPORT-MBF"),
                    ],
                )
            )
        ),
        "carried_tbd_refs": deepcopy(
            list(
                profile.get(
                    "carried_tbd_refs",
                    [
                        TARGET_VERSION_TBD,
                        RECORD_SUBSET_TBD,
                        DIRECT_STABLE_ID_TBD,
                    ],
                )
            )
        ),
        "boundary_notes": list(profile.get("boundary_notes", boundary_notes)),
        **({"free_metadata": deepcopy(profile["free_metadata"])} if "free_metadata" in profile else {}),
    }


def _model_payload(model_payload: Mapping[str, Any], provenance: Mapping[str, Any]) -> dict[str, Any]:
    payload = dict(model_payload)
    return {
        "payload_kind": "caepipe_mbf_smoke_subset_payload",
        "units": deepcopy(dict(payload.get("units", {}))),
        "nodes": deepcopy(_list(payload.get("nodes"))),
        "elements": deepcopy(_list(payload.get("elements"))),
        "supports": deepcopy(_list(payload.get("supports"))),
        "load_cases": deepcopy(_list(payload.get("load_cases"))),
        "unsupported_entities": deepcopy(_list(payload.get("unsupported_entities"))),
        "provenance": deepcopy(dict(payload.get("provenance", provenance))),
        **({"free_metadata": deepcopy(payload["free_metadata"])} if "free_metadata" in payload else {}),
    }


def _stable_id_map(entries: list[Mapping[str, Any]], provenance: Mapping[str, Any]) -> list[dict[str, Any]]:
    normalized = []
    for entry in entries:
        normalized.append(
            {
                "canonical_ref": deepcopy(dict(entry.get("canonical_ref", _ref("TBD", "missing")))),
                "target_ref": deepcopy(dict(entry.get("target_ref", _ref("CaePipeMbfRecord", "missing")))),
                "carrier_mode": "sidecar_mapping",
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
                "governing_tbd_id": "TBD-17-01-002",
                "downstream_implication": "CAEPIPE MBF package cannot be treated as complete until losses are classified.",
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
                "governing_tbd_id": str(entry.get("governing_tbd_id", "TBD")),
                "downstream_implication": str(entry.get("downstream_implication", "Requires downstream review.")),
                "human_review_required": True,
                "provenance": deepcopy(dict(entry.get("provenance", provenance))),
            }
        )
    return sorted(normalized, key=lambda item: item["loss_id"])


def _validation_report(
    *,
    validation_checks: list[Mapping[str, Any]] | None,
    diagnostics: list[Mapping[str, Any]],
    provenance: Mapping[str, Any],
) -> dict[str, Any]:
    checks = [
        {
            "check_id": str(item.get("check_id", f"check:{index:04d}")),
            "check_name": str(item.get("check_name", "CAEPIPE MBF package validation check")),
            "status": str(item.get("status", "pass")),
            "evidence_refs": deepcopy(list(item.get("evidence_refs", []))),
            "diagnostics": deepcopy(list(item.get("diagnostics", []))),
        }
        for index, item in enumerate(validation_checks or [])
    ]
    if not checks:
        checks = [
            {
                "check_id": "check:shape",
                "check_name": "CAEPIPE MBF package shape check",
                "status": "pass",
                "evidence_refs": [_ref("Schema", "schemas/caepipe_mbf_export.schema.json")],
                "diagnostics": [],
            }
        ]
    status = "blocked" if any(item.get("severity") == "blocking" for item in diagnostics) else "boundary_checked"
    return {
        "validation_status": status,
        "checks": sorted(checks, key=lambda item: item["check_id"]),
        "provenance": deepcopy(dict(provenance)),
    }


def _package_members(export_id: str, checksums: Mapping[str, Mapping[str, Any]]) -> list[dict[str, Any]]:
    member_specs = [
        ("manifest", "manifest", "manifest.json", "json"),
        ("mbf_text", "mbf_text", "model.mbf", "text/plain"),
        ("stable_id_map", "stable_id_map", "stable_id_map.json", "json"),
        ("loss_report", "loss_report", "loss_report.json", "json"),
        ("validation_report", "validation_report", "validation_report.json", "json"),
        ("diagnostics", "diagnostics", "diagnostics.json", "json"),
    ]
    members = []
    for key, role, path, content_kind in member_specs:
        members.append(
            {
                "member_name": f"{export_id}:{role}",
                "member_role": role,
                "path": path,
                "content_kind": content_kind,
                "payload_ref": _ref("CaePipeMbfMember", f"{export_id}:{role}"),
                "hash": deepcopy(dict(checksums[key])),
            }
        )
    return members


def _source_model_hash(source_model_hash: Mapping[str, Any] | str, source_ref: Mapping[str, Any]) -> dict[str, Any]:
    if isinstance(source_model_hash, Mapping):
        record = dict(source_model_hash)
        record.setdefault("algorithm", "sha256")
        record.setdefault("canonicalization", "JCS_compatible_json_payload_hash")
        record.setdefault("payload_ref", deepcopy(dict(source_ref)))
        record.setdefault("payload_scope", "source_model_hash")
        value = str(record.get("value", ""))
        if not value.startswith("sha256:"):
            record["value"] = _sha256(value)
        return deepcopy(record)
    return {
        "algorithm": "sha256",
        "canonicalization": "JCS_compatible_json_payload_hash",
        "payload_ref": deepcopy(dict(source_ref)),
        "payload_scope": "source_model_hash",
        "value": _sha256(str(source_model_hash)),
    }


def _privacy(privacy: Mapping[str, Any] | None) -> dict[str, Any]:
    record = deepcopy(DEFAULT_PRIVACY)
    if privacy:
        record.update(deepcopy(dict(privacy)))
    return record


def _checksum(value: Any, payload_ref: Mapping[str, Any], payload_scope: str) -> dict[str, Any]:
    return {
        "algorithm": "sha256",
        "canonicalization": "JCS_compatible_json_payload_hash",
        "payload_ref": deepcopy(dict(payload_ref)),
        "payload_scope": payload_scope,
        "value": _sha256(canonical_json(value)),
    }


def _text_checksum(value: str, payload_ref: Mapping[str, Any]) -> dict[str, Any]:
    return {
        "algorithm": "sha256",
        "canonicalization": "normalized_ascii_lf_text",
        "payload_ref": deepcopy(dict(payload_ref)),
        "payload_scope": "caepipe_mbf_text",
        "value": _sha256(canonical_text(value)),
    }


def _sha256(value: str) -> str:
    return "sha256:" + hashlib.sha256(value.encode("utf-8")).hexdigest()


def _diagnostic(
    code: str,
    severity: str,
    diagnostic_class: str,
    message: str,
    remediation: str,
    affected_refs: list[Mapping[str, Any]],
) -> dict[str, Any]:
    affected = affected_refs[0] if affected_refs else _ref("CaePipeMbfExportPackage", "unknown")
    return {
        "code": code,
        "class": diagnostic_class,
        "severity": severity,
        "source": _ref("Deliverable", "DEL-17-04"),
        "affected_object": deepcopy(dict(affected)),
        "message": message,
        "remediation": remediation,
        "provenance": deepcopy(ENGINE_PROVENANCE),
    }


def _record(name: str, fields: list[Any]) -> str:
    return ",".join([_safe(name)] + [_safe(value) for value in fields])


def _safe(value: Any) -> str:
    text = str(value if value is not None else "TBD")
    return "".join(char if 32 <= ord(char) < 127 and char not in {",", "\n", "\r"} else "_" for char in text)


def _num(value: Any) -> str:
    return f"{float(value):.6g}"


def _is_ascii(value: str) -> bool:
    try:
        value.encode("ascii")
    except UnicodeEncodeError:
        return False
    return True


def _list(value: Any) -> list[Any]:
    return value if isinstance(value, list) else []


def _is_reference(value: Any) -> bool:
    return isinstance(value, Mapping) and bool(value.get("object_type")) and bool(value.get("ref"))


def _reference_key(value: Mapping[str, Any]) -> tuple[str, str]:
    return (str(value.get("object_type")), str(value.get("ref")))


def _ref(object_type: str, value: str) -> dict[str, str]:
    return {"object_type": object_type, "ref": value}


def _stable(items: list[Mapping[str, Any]]) -> list[dict[str, Any]]:
    return [deepcopy(dict(item)) for item in sorted(items, key=canonical_json)]


def _sort(value: Any) -> Any:
    return json.loads(canonical_json(value))
