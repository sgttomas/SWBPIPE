"""Native open JSON export package builder for DEL-17-03.

The native package is a project-owned interchange artifact. It preserves model
identity, stable IDs, units, provenance, diagnostics, loss reporting, and
professional-boundary notices without selecting a commercial target, invoking an
external solver, or making reliance claims.
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


NATIVE_JSON_EXPORT_VERSION = "0.1.0"
CANONICALIZATION_LABEL = "deterministic_sorted_compact_json_payload_hash"

LOSS_CATEGORIES = {
    "exported",
    "omitted",
    "approximated",
    "delegated",
    "unsupported",
    "TBD",
}

ENGINE_PROVENANCE = {
    "source_name": "OpenPipeStress DEL-17-03 native JSON export package builder",
    "source_location": "core/handoff/native_json/package.py",
    "source_license": "project-governed",
    "contributor": "OpenPipeStress Type 2 worker",
    "contributor_certification": "implementation-only-no-professional-claim",
    "redistribution_status": "public_permissive",
    "review_status": "machine_checked",
    "privacy_classification": "public_metadata",
}

PROFESSIONAL_BOUNDARY = {
    "human_review_required": True,
    "supports_downstream_interchange": True,
    "supports_adapter_input": True,
    "software_makes_release_claim": False,
    "software_makes_external_compatibility_claim": False,
    "software_makes_solver_validation_claim": False,
    "software_makes_compliance_claim": False,
    "software_makes_certification_claim": False,
    "software_makes_sealing_claim": False,
    "software_makes_approval_claim": False,
    "software_makes_authentication_claim": False,
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
    "Native JSON is project-owned interchange and adapter input.",
    "Package evidence does not assert external target support, solver validation, release readiness, code compliance, or professional reliance.",
    "Protected standards values, private payloads, proprietary target examples, and owner criteria are excluded.",
]


def build_native_json_export_package(
    *,
    native_export_id: str,
    source_model_ref: Mapping[str, Any],
    source_model_hash: Mapping[str, Any] | str,
    model_payload: Mapping[str, Any],
    stable_id_map: list[Mapping[str, Any]],
    loss_report: list[Mapping[str, Any]],
    units_manifest: Mapping[str, Any] | None = None,
    export_profile: Mapping[str, Any] | None = None,
    validation_checks: list[Mapping[str, Any]] | None = None,
    diagnostics: list[Mapping[str, Any]] | None = None,
    privacy: Mapping[str, Any] | None = None,
    provenance: Mapping[str, Any] | None = None,
    boundary_notes: list[str] | None = None,
) -> dict[str, Any]:
    """Build a deterministic native JSON export package envelope."""

    package_ref = _ref("NativeJsonExportPackage", native_export_id)
    source_ref = deepcopy(dict(source_model_ref))
    provenance_record = deepcopy(dict(provenance or ENGINE_PROVENANCE))
    privacy_record = _privacy(privacy)
    source_hash = _source_model_hash(source_model_hash, source_ref)
    notes = list(boundary_notes or DEFAULT_BOUNDARY_NOTES)
    profile = _export_profile(export_profile, notes)
    normalized_model_payload = _model_payload(
        model_payload=model_payload,
        source_model_ref=source_ref,
        units_manifest=units_manifest,
        provenance=provenance_record,
    )
    normalized_stable_id_map = _stable_id_map(stable_id_map, provenance_record)
    normalized_loss_report = _loss_report(loss_report, package_ref, provenance_record)
    normalized_diagnostics = _stable(
        [deepcopy(dict(item)) for item in diagnostics or []]
        + diagnostics_for_native_json_export_package(
            model_payload=normalized_model_payload,
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
        "model_payload": _checksum(
            normalized_model_payload,
            _ref("NativeJsonMember", f"{native_export_id}:model_payload"),
            "native_json_model_payload",
        ),
        "stable_id_map": _checksum(
            normalized_stable_id_map,
            _ref("NativeJsonMember", f"{native_export_id}:stable_id_map"),
            "native_json_stable_id_map",
        ),
        "loss_report": _checksum(
            normalized_loss_report,
            _ref("NativeJsonMember", f"{native_export_id}:loss_report"),
            "native_json_loss_report",
        ),
        "validation_report": _checksum(
            validation_report,
            _ref("NativeJsonMember", f"{native_export_id}:validation_report"),
            "native_json_validation_report",
        ),
        "diagnostics": _checksum(
            normalized_diagnostics,
            _ref("NativeJsonMember", f"{native_export_id}:diagnostics"),
            "native_json_diagnostics",
        ),
    }
    manifest_seed = {
        "manifest_id": f"{native_export_id}:manifest",
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
        _ref("NativeJsonMember", f"{native_export_id}:manifest"),
        "native_json_manifest",
    )

    manifest = {
        "manifest_id": f"{native_export_id}:manifest",
        "source_model_ref": source_ref,
        "source_model_hash": source_hash,
        "export_profile_ref": _ref("ExportProfile", profile["profile_id"]),
        "package_members": _package_members(native_export_id, checksums),
        "checksums": _stable(list(checksums.values()) + [source_hash]),
        "source_basis_refs": deepcopy(profile["source_basis_refs"]),
        "boundary_notes": notes,
        "diagnostics": normalized_diagnostics,
    }

    package = {
        "schema_version": NATIVE_JSON_EXPORT_VERSION,
        "deliverable_id": "DEL-17-03",
        "package_id": "PKG-17",
        "scope_items": ["SOW-030", "SOW-074"],
        "objectives": ["OBJ-009", "OBJ-017"],
        "native_export_id": native_export_id,
        "package_status": "native_open_json_export_package",
        "export_profile": profile,
        "manifest": manifest,
        "model_payload": normalized_model_payload,
        "stable_id_map": normalized_stable_id_map,
        "loss_report": normalized_loss_report,
        "validation_report": validation_report,
        "diagnostics": normalized_diagnostics,
        "privacy": privacy_record,
        "provenance": provenance_record,
        "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
    }
    return _sort(package)


def diagnostics_for_native_json_export_package(
    *,
    model_payload: Mapping[str, Any],
    stable_id_map: list[Mapping[str, Any]],
    loss_report: list[Mapping[str, Any]],
    export_profile: Mapping[str, Any],
    privacy: Mapping[str, Any],
    package_ref: Mapping[str, Any],
) -> list[dict[str, Any]]:
    """Return deterministic diagnostics for native JSON package boundaries."""

    diagnostics: list[dict[str, Any]] = []
    if not model_payload.get("entities"):
        diagnostics.append(
            _diagnostic(
                "NJ-MODEL-PAYLOAD-EMPTY",
                "blocking",
                "EXPORT_BLOCKING",
                "Native JSON package has no model entities.",
                "Provide an invented or governed model payload; do not emit a silent empty export.",
                [package_ref],
            )
        )
    if not model_payload.get("units_manifest"):
        diagnostics.append(
            _diagnostic(
                "NJ-UNITS-MANIFEST-MISSING",
                "blocking",
                "UNIT_WARNING",
                "Native JSON package has no units manifest.",
                "Carry forward the source units manifest instead of relying on hidden units.",
                [package_ref],
            )
        )
    if not stable_id_map:
        diagnostics.append(
            _diagnostic(
                "NJ-STABLE-ID-MAP-MISSING",
                "blocking",
                "TARGET_MAPPING_WARNING",
                "Native JSON package has no stable ID map entries.",
                "Map every exported or omitted canonical identity explicitly.",
                [package_ref],
            )
        )
    if not loss_report:
        diagnostics.append(
            _diagnostic(
                "NJ-LOSS-REPORT-MISSING",
                "blocking",
                "UNSUPPORTED_BEHAVIOR_WARNING",
                "Native JSON package has no loss report entries.",
                "Record exported, omitted, approximated, delegated, unsupported, or TBD behavior explicitly.",
                [package_ref],
            )
        )
    for entry in loss_report:
        if entry.get("category") not in LOSS_CATEGORIES:
            diagnostics.append(
                _diagnostic(
                    "NJ-LOSS-CATEGORY-UNSUPPORTED",
                    "blocking",
                    "UNSUPPORTED_BEHAVIOR_WARNING",
                    "Loss report entry uses a category outside the DEL-17-02 taxonomy.",
                    "Use exported, omitted, approximated, delegated, unsupported, or TBD.",
                    [_ref("LossReportEntry", str(entry.get("loss_id", "unknown")))],
                )
            )
    if contains_prohibited_authority_term(model_payload.get("free_metadata", {})):
        diagnostics.append(
            _diagnostic(
                "NJ-PROHIBITED-AUTHORITY-TERM",
                "blocking",
                "IP_BOUNDARY_WARNING",
                "Model payload free metadata contains authority wording.",
                "Remove release, target-support, validation, compliance, or professional-reliance wording from user metadata.",
                [package_ref],
            )
        )
    if contains_prohibited_authority_term(export_profile.get("free_metadata", {})):
        diagnostics.append(
            _diagnostic(
                "NJ-PROFILE-AUTHORITY-TERM",
                "blocking",
                "IP_BOUNDARY_WARNING",
                "Export profile free metadata contains authority wording.",
                "Keep target-support and reliance decisions outside the native package.",
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
                    "NJ-PRIVACY-BOUNDARY-VIOLATION",
                    "blocking",
                    "IP_BOUNDARY_WARNING",
                    f"Privacy field {field} is not allowed for public native JSON package fixtures.",
                    "Reference private/protected/commercial content by governed metadata only.",
                    [package_ref],
                )
            )
    return _stable(diagnostics)


def canonical_json(value: Any) -> str:
    """Serialize native package values with deterministic JSON key ordering."""

    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def write_native_json_export_package(path: str | Path, package: Mapping[str, Any]) -> None:
    """Write a native JSON export package using the canonical JSON byte basis."""

    Path(path).write_text(canonical_json(package) + "\n", encoding="utf-8")


def _export_profile(profile: Mapping[str, Any] | None, boundary_notes: list[str]) -> dict[str, Any]:
    profile = dict(profile or {})
    return {
        "profile_id": str(profile.get("profile_id", "ops.native_json.default")),
        "profile_version": str(profile.get("profile_version", NATIVE_JSON_EXPORT_VERSION)),
        "target_family": "native_open_json",
        "unit_policy": "preserve_source_units_manifest",
        "coordinate_policy": str(profile.get("coordinate_policy", "preserve_source_coordinate_policy")),
        "stable_id_policy": "canonical_ids_preserved_or_mapped",
        "loss_report_policy": "mandatory_for_every_package",
        "source_basis_refs": deepcopy(
            list(
                profile.get(
                    "source_basis_refs",
                    [
                        _ref("Deliverable", "DEL-17-01"),
                        _ref("Deliverable", "DEL-17-02"),
                        _ref("Deliverable", "DEL-02-01"),
                        _ref("Deliverable", "DEL-02-02"),
                    ],
                )
            )
        ),
        "boundary_notes": list(profile.get("boundary_notes", boundary_notes)),
        **({"free_metadata": deepcopy(profile["free_metadata"])} if "free_metadata" in profile else {}),
    }


def _model_payload(
    *,
    model_payload: Mapping[str, Any],
    source_model_ref: Mapping[str, Any],
    units_manifest: Mapping[str, Any] | None,
    provenance: Mapping[str, Any],
) -> dict[str, Any]:
    payload = dict(model_payload)
    normalized = {
        "payload_kind": "native_open_json_model_payload",
        "source_model_ref": deepcopy(dict(payload.get("source_model_ref", source_model_ref))),
        "model_schema": "schemas/model.schema.yaml",
        "model_role": str(payload.get("model_role", "derived_export_view")),
        "entities": deepcopy(dict(payload.get("entities", {}))),
        "units_manifest": deepcopy(dict(payload.get("units_manifest", units_manifest or {}))),
        "provenance": deepcopy(dict(payload.get("provenance", provenance))),
    }
    if "free_metadata" in payload:
        normalized["free_metadata"] = deepcopy(dict(payload["free_metadata"]))
    return normalized


def _stable_id_map(entries: list[Mapping[str, Any]], provenance: Mapping[str, Any]) -> list[dict[str, Any]]:
    normalized = []
    for entry in entries:
        normalized.append(
            {
                "canonical_ref": deepcopy(dict(entry.get("canonical_ref", _ref("TBD", "missing")))),
                "package_ref": deepcopy(dict(entry.get("package_ref", _ref("TBD", "missing")))),
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
                "category": "TBD",
                "severity": "blocking",
                "affected_refs": [package_ref],
                "target_artifact_ref": package_ref,
                "reason": "Loss report entries were not supplied.",
                "source_basis_ref": _ref("Deliverable", "DEL-17-02"),
                "downstream_implication": "Native package cannot be treated as complete until losses are explicitly classified.",
            }
        ]
    normalized = []
    for entry in entries:
        normalized.append(
            {
                "loss_id": str(entry.get("loss_id", "loss:TBD")),
                "category": str(entry.get("category", "TBD")),
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
    *,
    validation_checks: list[Mapping[str, Any]] | None,
    diagnostics: list[Mapping[str, Any]],
    provenance: Mapping[str, Any],
) -> dict[str, Any]:
    checks = [
        {
            "check_id": str(item.get("check_id", f"check:{index:04d}")),
            "check_name": str(item.get("check_name", "Native package validation check")),
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
                "check_name": "Native JSON package shape check",
                "status": "pass",
                "evidence_refs": [_ref("Schema", "schemas/native_json_export.schema.json")],
                "diagnostics": [],
            }
        ]
    status = "blocked" if any(item.get("severity") == "blocking" for item in diagnostics) else "boundary_checked"
    return {
        "validation_status": status,
        "checks": sorted(checks, key=lambda item: item["check_id"]),
        "provenance": deepcopy(dict(provenance)),
    }


def _package_members(native_export_id: str, checksums: Mapping[str, Mapping[str, Any]]) -> list[dict[str, Any]]:
    member_specs = [
        ("manifest", "manifest", "manifest.json", "native_json_manifest"),
        ("model_payload", "model_payload", "model_payload.json", "native_json_model_payload"),
        ("stable_id_map", "stable_id_map", "stable_id_map.json", "native_json_stable_id_map"),
        ("loss_report", "loss_report", "loss_report.json", "native_json_loss_report"),
        ("validation_report", "validation_report", "validation_report.json", "native_json_validation_report"),
        ("diagnostics", "diagnostics", "diagnostics.json", "native_json_diagnostics"),
    ]
    members = []
    for key, role, path, _scope in member_specs:
        members.append(
            {
                "member_name": f"{native_export_id}:{role}",
                "member_role": role,
                "path": path,
                "content_kind": "json",
                "payload_ref": _ref("NativeJsonMember", f"{native_export_id}:{role}"),
                "hash": deepcopy(dict(checksums[key])),
            }
        )
    return members


def _source_model_hash(source_model_hash: Mapping[str, Any] | str, source_ref: Mapping[str, Any]) -> dict[str, Any]:
    if isinstance(source_model_hash, Mapping):
        record = dict(source_model_hash)
        if record.get("algorithm") != "sha256":
            raise ValueError("source_model_hash mapping must declare algorithm='sha256'")
        if not isinstance(record.get("canonicalization"), str) or not record["canonicalization"].strip():
            raise ValueError("source_model_hash mapping must declare its canonicalization label")
        value = record.get("value")
        if not isinstance(value, str) or not _is_sha256_digest(value):
            raise ValueError("source_model_hash mapping must contain a sha256:<64 lowercase hex> digest")
        record.setdefault("payload_ref", deepcopy(dict(source_ref)))
        record.setdefault("payload_scope", "source_model_hash")
        if record["payload_scope"] != "source_model_hash":
            raise ValueError("source_model_hash mapping payload_scope must be 'source_model_hash'")
        return deepcopy(record)
    return {
        "algorithm": "sha256",
        "canonicalization": CANONICALIZATION_LABEL,
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
        "canonicalization": CANONICALIZATION_LABEL,
        "payload_ref": deepcopy(dict(payload_ref)),
        "payload_scope": payload_scope,
        "value": _sha256(canonical_json(value)),
    }


def _sha256(value: str) -> str:
    return "sha256:" + hashlib.sha256(value.encode("utf-8")).hexdigest()


def _is_sha256_digest(value: str) -> bool:
    digest = value.removeprefix("sha256:")
    return value.startswith("sha256:") and len(digest) == 64 and all(
        character in "0123456789abcdef" for character in digest
    )


def _diagnostic(
    code: str,
    severity: str,
    diagnostic_class: str,
    message: str,
    remediation: str,
    affected_refs: list[Mapping[str, Any]],
) -> dict[str, Any]:
    affected = affected_refs[0] if affected_refs else _ref("NativeJsonExportPackage", "unknown")
    return {
        "code": code,
        "class": diagnostic_class,
        "severity": severity,
        "source": _ref("Deliverable", "DEL-17-03"),
        "affected_object": deepcopy(dict(affected)),
        "message": message,
        "remediation": remediation,
        "provenance": deepcopy(ENGINE_PROVENANCE),
    }


def _ref(object_type: str, value: str) -> dict[str, str]:
    return {"object_type": object_type, "ref": value}


def _stable(items: list[Mapping[str, Any]]) -> list[dict[str, Any]]:
    return [deepcopy(dict(item)) for item in sorted(items, key=canonical_json)]


def _sort(value: Any) -> Any:
    return json.loads(canonical_json(value))
