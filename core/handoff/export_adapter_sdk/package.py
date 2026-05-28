"""Export adapter SDK admission package builder for DEL-17-09.

This module records target-admission and adapter-boundary evidence for
additional export targets. It does not load plugins, grant runtime permissions,
invoke external processes, publish public endpoints, or assert target readiness
or professional reliance.
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


EXPORT_ADAPTER_SDK_VERSION = "0.1.0"
LOSS_CATEGORIES = {
    "exported",
    "omitted",
    "approximated",
    "delegated",
    "unsupported",
    "tbd",
}
ADMISSION_STATES = {
    "candidate_non_gating",
    "source_basis_pending",
    "source_basis_admitted",
    "rejected",
    "quarantined",
    "tbd",
}
DENIED_GRANT_TYPES = {
    "filesystem",
    "network",
    "process",
    "private_data",
    "storage",
    "rule_pack",
    "solver",
    "report_control",
}
CHECKLIST_CATEGORIES = {
    "source_basis",
    "target_version",
    "units_coordinates",
    "stable_identity",
    "loss_report",
    "diagnostics",
    "privacy_screening",
    "export_review",
    "human_review",
}

ENGINE_PROVENANCE = {
    "source_name": "OpenPipeStress DEL-17-09 export adapter SDK package builder",
    "source_location": "core/handoff/export_adapter_sdk/package.py",
    "source_license": "project-governed",
    "contributor": "OpenPipeStress Type 2 worker",
    "contributor_certification": "implementation-only-no-professional-claim",
    "redistribution_status": "public_permissive",
    "review_status": "machine_checked",
    "privacy_classification": "public_metadata",
}

PROFESSIONAL_BOUNDARY = {
    "human_review_required": True,
    "records_admission_only": True,
    "records_identity_correlation": True,
    "creates_runtime_loader": False,
    "creates_public_endpoint": False,
    "creates_external_target_claim": False,
    "creates_release_claim": False,
    "creates_solver_validation_claim": False,
    "creates_code_conformance_claim": False,
    "creates_professional_reliance_record": False,
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
    "This package records adapter admission evidence only.",
    "Candidate targets remain non-gating until a later governed intake admits their source basis.",
    "Runtime grants are recorded as denied by default; this package does not implement a loader, sandbox, endpoint, process launch, network access, or target writer.",
    "Target behavior with missing source evidence remains tbd or unsupported.",
]


def build_export_adapter_sdk_package(
    *,
    sdk_id: str,
    target_registry: list[Mapping[str, Any]],
    adapter_contract: Mapping[str, Any] | None = None,
    validation_checklist: list[Mapping[str, Any]] | None = None,
    requested_runtime_grants: list[Mapping[str, Any]] | None = None,
    diagnostics: list[Mapping[str, Any]] | None = None,
    privacy: Mapping[str, Any] | None = None,
    provenance: Mapping[str, Any] | None = None,
    boundary_notes: list[str] | None = None,
) -> dict[str, Any]:
    """Build a deterministic adapter SDK admission package."""

    package_ref = _ref("ExportAdapterSdkPackage", sdk_id)
    provenance_record = deepcopy(dict(provenance or ENGINE_PROVENANCE))
    privacy_record = _privacy(privacy)
    notes = list(boundary_notes or DEFAULT_BOUNDARY_NOTES)
    normalized_grants = _runtime_grants(requested_runtime_grants or [], provenance_record)
    contract = _adapter_contract(adapter_contract, normalized_grants, notes, provenance_record)
    registry = _target_registry(target_registry, provenance_record)
    checklist = _validation_checklist(validation_checklist, registry, provenance_record)
    normalized_diagnostics = _stable(
        [deepcopy(dict(item)) for item in diagnostics or []]
        + diagnostics_for_export_adapter_sdk_package(
            target_registry=registry,
            adapter_contract=contract,
            validation_checklist=checklist,
            requested_runtime_grants=normalized_grants,
            privacy=privacy_record,
            package_ref=package_ref,
        )
    )
    validation_report = _validation_report(
        validation_checklist=checklist,
        diagnostics=normalized_diagnostics,
        provenance=provenance_record,
    )
    checksums = {
        "target_registry": _checksum(
            registry,
            _ref("ExportAdapterSdkMember", f"{sdk_id}:target_registry"),
            "export_adapter_sdk_target_registry",
        ),
        "adapter_contract": _checksum(
            contract,
            _ref("ExportAdapterSdkMember", f"{sdk_id}:adapter_contract"),
            "export_adapter_sdk_adapter_contract",
        ),
        "validation_checklist": _checksum(
            checklist,
            _ref("ExportAdapterSdkMember", f"{sdk_id}:validation_checklist"),
            "export_adapter_sdk_validation_checklist",
        ),
        "validation_report": _checksum(
            validation_report,
            _ref("ExportAdapterSdkMember", f"{sdk_id}:validation_report"),
            "export_adapter_sdk_validation_report",
        ),
        "diagnostics": _checksum(
            normalized_diagnostics,
            _ref("ExportAdapterSdkMember", f"{sdk_id}:diagnostics"),
            "export_adapter_sdk_diagnostics",
        ),
    }
    manifest_seed = {
        "manifest_id": f"{sdk_id}:manifest",
        "source_basis_refs": deepcopy(contract["source_basis_refs"]),
        "boundary_notes": notes,
        "member_hashes": checksums,
        "diagnostics": normalized_diagnostics,
    }
    checksums["manifest"] = _checksum(
        manifest_seed,
        _ref("ExportAdapterSdkMember", f"{sdk_id}:manifest"),
        "export_adapter_sdk_manifest",
    )
    manifest = {
        "manifest_id": f"{sdk_id}:manifest",
        "source_basis_refs": deepcopy(contract["source_basis_refs"]),
        "package_members": _package_members(sdk_id, checksums),
        "checksums": _stable(list(checksums.values())),
        "boundary_notes": notes,
        "diagnostics": normalized_diagnostics,
    }
    package = {
        "schema_version": EXPORT_ADAPTER_SDK_VERSION,
        "deliverable_id": "DEL-17-09",
        "package_id": "PKG-17",
        "scope_items": ["SOW-030", "SOW-074", "SOW-075"],
        "objectives": ["OBJ-009", "OBJ-017", "OBJ-018"],
        "sdk_id": sdk_id,
        "package_status": "export_adapter_sdk_foundation",
        "adapter_contract": contract,
        "target_registry": registry,
        "runtime_grants": normalized_grants,
        "validation_checklist": checklist,
        "manifest": manifest,
        "validation_report": validation_report,
        "diagnostics": normalized_diagnostics,
        "privacy": privacy_record,
        "provenance": provenance_record,
        "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
    }
    return _sort(package)


def diagnostics_for_export_adapter_sdk_package(
    *,
    target_registry: list[Mapping[str, Any]],
    adapter_contract: Mapping[str, Any],
    validation_checklist: list[Mapping[str, Any]],
    requested_runtime_grants: list[Mapping[str, Any]],
    privacy: Mapping[str, Any],
    package_ref: Mapping[str, Any],
) -> list[dict[str, Any]]:
    """Return deterministic diagnostics for adapter SDK admission packages."""

    diagnostics: list[dict[str, Any]] = []
    if not target_registry:
        diagnostics.append(
            _diagnostic(
                "EASDK-TARGET-REGISTRY-MISSING",
                "blocking",
                "TARGET_MAPPING_WARNING",
                "Export adapter SDK package has no target registry records.",
                "Record at least one candidate or rejected target entry.",
                [package_ref],
            )
        )
    if not adapter_contract.get("source_basis_refs"):
        diagnostics.append(
            _diagnostic(
                "EASDK-CONTRACT-SOURCE-BASIS-MISSING",
                "blocking",
                "PROVENANCE_WARNING",
                "Adapter contract has no source-basis references.",
                "Carry forward DEL-17-01 and DEL-17-02 or admitted target source-basis refs.",
                [package_ref],
            )
        )
    if adapter_contract.get("stable_id_policy") in {"", "TBD", "tbd", None}:
        diagnostics.append(
            _diagnostic(
                "EASDK-STABLE-ID-POLICY-MISSING",
                "blocking",
                "TARGET_MAPPING_WARNING",
                "Adapter contract does not declare stable-ID policy.",
                "Declare canonical identity preservation or sidecar mapping behavior.",
                [package_ref],
            )
        )
    if adapter_contract.get("loss_report_policy") in {"", "TBD", "tbd", None}:
        diagnostics.append(
            _diagnostic(
                "EASDK-LOSS-REPORT-POLICY-MISSING",
                "blocking",
                "UNSUPPORTED_BEHAVIOR_WARNING",
                "Adapter contract does not declare loss-report policy.",
                "Require explicit exported, omitted, approximated, delegated, unsupported, or tbd records.",
                [package_ref],
            )
        )
    if contains_prohibited_authority_term(adapter_contract.get("free_metadata", {})):
        diagnostics.append(
            _diagnostic(
                "EASDK-CONTRACT-AUTHORITY-TERM",
                "blocking",
                "IP_BOUNDARY_WARNING",
                "Adapter contract free metadata contains authority wording.",
                "Remove authority, release, target-readiness, or professional-reliance wording.",
                [package_ref],
            )
        )
    for target in target_registry:
        target_ref = target.get("target_ref", _ref("ExportTarget", "unknown"))
        if target.get("admission_state") not in ADMISSION_STATES:
            diagnostics.append(
                _diagnostic(
                    "EASDK-ADMISSION-STATE-UNKNOWN",
                    "blocking",
                    "ASSUMPTION_WARNING",
                    "Target registry record has an unknown admission state.",
                    "Use candidate_non_gating, source_basis_pending, source_basis_admitted, rejected, quarantined, or tbd.",
                    [target_ref],
                )
            )
        if not target.get("source_basis_refs"):
            diagnostics.append(
                _diagnostic(
                    "EASDK-SOURCE-BASIS-MISSING",
                    "blocking",
                    "PROVENANCE_WARNING",
                    "Target registry record has no source-basis references.",
                    "Keep the target as candidate-only until source evidence is admitted.",
                    [target_ref],
                )
            )
        if target.get("admission_state") == "source_basis_admitted" and target.get("target_version_basis") in {
            "",
            "TBD",
            "tbd",
            None,
        }:
            diagnostics.append(
                _diagnostic(
                    "EASDK-TARGET-VERSION-BASIS-MISSING",
                    "blocking",
                    "PROVENANCE_WARNING",
                    "Admitted target record lacks target version basis.",
                    "Name the source-grounded target version basis or demote the record to pending.",
                    [target_ref],
                )
            )
        if target.get("stable_id_policy") in {"", "TBD", "tbd", None}:
            diagnostics.append(
                _diagnostic(
                    "EASDK-TARGET-STABLE-ID-POLICY-MISSING",
                    "blocking",
                    "TARGET_MAPPING_WARNING",
                    "Target record does not declare stable-ID behavior.",
                    "Record direct identity carriage, sidecar mapping, or explicit tbd blocker.",
                    [target_ref],
                )
            )
        if target.get("loss_report_policy") in {"", "TBD", "tbd", None}:
            diagnostics.append(
                _diagnostic(
                    "EASDK-TARGET-LOSS-REPORT-POLICY-MISSING",
                    "blocking",
                    "UNSUPPORTED_BEHAVIOR_WARNING",
                    "Target record does not declare loss-report behavior.",
                    "Require target limitations and omissions to be classified.",
                    [target_ref],
                )
            )
        if contains_prohibited_authority_term(target.get("free_metadata", {})):
            diagnostics.append(
                _diagnostic(
                    "EASDK-TARGET-AUTHORITY-TERM",
                    "blocking",
                    "IP_BOUNDARY_WARNING",
                    "Target registry free metadata contains authority wording.",
                    "Remove authority, target-readiness, or professional-reliance wording.",
                    [target_ref],
                )
            )
    for grant in requested_runtime_grants:
        if grant.get("requested") is True or grant.get("decision") != "denied_by_default":
            diagnostics.append(
                _diagnostic(
                    "EASDK-RUNTIME-GRANT-REQUESTED",
                    "blocking",
                    "SECURITY_WARNING",
                    "Runtime grant request is not allowed in the DEL-17-09 foundation.",
                    "Record runtime access as denied_by_default until a later governed runtime design exists.",
                    [grant.get("grant_ref", package_ref)],
                )
            )
        if grant.get("grant_type") not in DENIED_GRANT_TYPES:
            diagnostics.append(
                _diagnostic(
                    "EASDK-RUNTIME-GRANT-TYPE-UNKNOWN",
                    "blocking",
                    "SECURITY_WARNING",
                    "Runtime grant type is outside the known denied-by-default taxonomy.",
                    "Use filesystem, network, process, private_data, storage, rule_pack, solver, or report_control.",
                    [grant.get("grant_ref", package_ref)],
                )
            )
    checklist_categories = {item.get("category") for item in validation_checklist}
    missing_categories = sorted(CHECKLIST_CATEGORIES - checklist_categories)
    if missing_categories:
        diagnostics.append(
            _diagnostic(
                "EASDK-CHECKLIST-CATEGORY-MISSING",
                "blocking",
                "ASSUMPTION_WARNING",
                "Validation checklist is missing required admission categories.",
                "Add checklist records for: " + ", ".join(missing_categories),
                [package_ref],
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
                    "EASDK-PRIVACY-BOUNDARY-VIOLATION",
                    "blocking",
                    "IP_BOUNDARY_WARNING",
                    f"Privacy field {field} is not allowed for public adapter SDK fixtures.",
                    "Reference private/protected/commercial content by governed metadata only.",
                    [package_ref],
                )
            )
    return _stable(diagnostics)


def canonical_json(value: Any) -> str:
    """Serialize adapter SDK values with deterministic JSON key ordering."""

    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def write_export_adapter_sdk_package(path: str | Path, package: Mapping[str, Any]) -> None:
    """Write an adapter SDK package using the canonical JSON byte basis."""

    Path(path).write_text(canonical_json(package) + "\n", encoding="utf-8")


def _adapter_contract(
    adapter_contract: Mapping[str, Any] | None,
    runtime_grants: list[Mapping[str, Any]],
    boundary_notes: list[str],
    provenance: Mapping[str, Any],
) -> dict[str, Any]:
    contract = dict(adapter_contract or {})
    return {
        "contract_id": str(contract.get("contract_id", "ops.export_adapter_sdk.foundation")),
        "contract_version": str(contract.get("contract_version", EXPORT_ADAPTER_SDK_VERSION)),
        "contract_kind": "export_adapter_admission_contract",
        "source_basis_refs": deepcopy(
            list(
                contract.get(
                    "source_basis_refs",
                    [_ref("Deliverable", "DEL-17-01"), _ref("Deliverable", "DEL-17-02")],
                )
            )
        ),
        "unit_policy": str(contract.get("unit_policy", "explicit_unit_policy_required")),
        "coordinate_policy": str(contract.get("coordinate_policy", "explicit_coordinate_policy_required")),
        "stable_id_policy": str(contract.get("stable_id_policy", "canonical_identity_required_or_sidecar_mapped")),
        "loss_report_policy": str(contract.get("loss_report_policy", "mandatory_for_every_target_package")),
        "diagnostics_policy": str(contract.get("diagnostics_policy", "blocking_warnings_preserved")),
        "target_admission_policy": str(contract.get("target_admission_policy", "candidate_non_gating_until_source_basis_admitted")),
        "runtime_grant_policy": "denied_by_default",
        "runtime_grants": deepcopy(runtime_grants),
        "boundary_notes": list(contract.get("boundary_notes", boundary_notes)),
        "provenance": deepcopy(dict(contract.get("provenance", provenance))),
        **({"free_metadata": deepcopy(contract["free_metadata"])} if "free_metadata" in contract else {}),
    }


def _target_registry(entries: list[Mapping[str, Any]], provenance: Mapping[str, Any]) -> list[dict[str, Any]]:
    normalized = []
    for entry in entries:
        target_id = str(entry.get("target_id", "target:tbd"))
        normalized.append(
            {
                "target_id": target_id,
                "target_ref": deepcopy(dict(entry.get("target_ref", _ref("ExportTarget", target_id)))),
                "target_family": str(entry.get("target_family", "additional_export_target")),
                "admission_state": str(entry.get("admission_state", "candidate_non_gating")),
                "target_version_basis": str(entry.get("target_version_basis", "tbd")),
                "source_basis_refs": deepcopy(list(entry.get("source_basis_refs", []))),
                "redistribution_status": str(entry.get("redistribution_status", "tbd")),
                "privacy_classification": str(entry.get("privacy_classification", "public_metadata")),
                "unit_policy": str(entry.get("unit_policy", "explicit_unit_policy_required")),
                "coordinate_policy": str(entry.get("coordinate_policy", "explicit_coordinate_policy_required")),
                "stable_id_policy": str(entry.get("stable_id_policy", "canonical_identity_required_or_sidecar_mapped")),
                "loss_report_policy": str(entry.get("loss_report_policy", "mandatory_for_every_target_package")),
                "diagnostics_policy": str(entry.get("diagnostics_policy", "blocking_warnings_preserved")),
                "runtime_grant_policy": "denied_by_default",
                "behavior_families": _behavior_families(entry.get("behavior_families")),
                "boundary_notes": deepcopy(list(entry.get("boundary_notes", DEFAULT_BOUNDARY_NOTES))),
                "provenance": deepcopy(dict(entry.get("provenance", provenance))),
                **({"free_metadata": deepcopy(entry["free_metadata"])} if "free_metadata" in entry else {}),
            }
        )
    return sorted(normalized, key=lambda item: item["target_id"])


def _behavior_families(value: Any) -> list[dict[str, Any]]:
    records = _list(value)
    if not records:
        records = [
            {
                "family": "target_specific_writer_behavior",
                "disposition": "tbd",
                "loss_category": "tbd",
                "reason": "Target-specific behavior requires later source-basis intake.",
            }
        ]
    normalized = []
    for record in records:
        normalized.append(
            {
                "family": str(record.get("family", "target_specific_behavior")),
                "disposition": str(record.get("disposition", "tbd")),
                "loss_category": str(record.get("loss_category", "tbd")),
                "reason": str(record.get("reason", "TBD")),
            }
        )
    return sorted(normalized, key=lambda item: item["family"])


def _runtime_grants(entries: list[Mapping[str, Any]], provenance: Mapping[str, Any]) -> list[dict[str, Any]]:
    records = entries or [{"grant_type": grant_type} for grant_type in sorted(DENIED_GRANT_TYPES)]
    normalized = []
    for entry in records:
        grant_type = str(entry.get("grant_type", "unknown"))
        grant_id = str(entry.get("grant_id", f"grant:{grant_type}"))
        normalized.append(
            {
                "grant_id": grant_id,
                "grant_ref": deepcopy(dict(entry.get("grant_ref", _ref("RuntimeGrant", grant_id)))),
                "grant_type": grant_type,
                "requested": bool(entry.get("requested", False)),
                "decision": str(entry.get("decision", "denied_by_default")),
                "reason": str(entry.get("reason", "Runtime access is outside this foundation tranche.")),
                "provenance": deepcopy(dict(entry.get("provenance", provenance))),
            }
        )
    return sorted(normalized, key=lambda item: item["grant_id"])


def _validation_checklist(
    entries: list[Mapping[str, Any]] | None,
    target_registry: list[Mapping[str, Any]],
    provenance: Mapping[str, Any],
) -> list[dict[str, Any]]:
    if not entries:
        target_refs = [deepcopy(dict(item["target_ref"])) for item in target_registry]
        entries = [
            _check("source_basis", "Source-basis refs recorded for each candidate target.", target_refs),
            _check("target_version", "Target version basis recorded or kept tbd.", target_refs),
            _check("units_coordinates", "Unit and coordinate policies are explicit.", target_refs),
            _check("stable_identity", "Stable identity policy or sidecar mapping is recorded.", target_refs),
            _check("loss_report", "Loss-report policy uses DEL-17-02 categories.", target_refs),
            _check("diagnostics", "Blocking diagnostics and warnings remain visible.", target_refs),
            _check("privacy_screening", "Public/private/protected-content posture is recorded.", target_refs),
            _check("export_review", "Export review is evidence-only and does not create target-readiness claims.", target_refs),
            _check("human_review", "Human review is required before any later target admission.", target_refs),
        ]
    normalized = []
    for index, entry in enumerate(entries):
        normalized.append(
            {
                "check_id": str(entry.get("check_id", f"check:{index:04d}")),
                "category": str(entry.get("category", "tbd")),
                "description": str(entry.get("description", "TBD")),
                "status": str(entry.get("status", "recorded")),
                "required_before_target_admission": bool(entry.get("required_before_target_admission", True)),
                "evidence_refs": deepcopy(list(entry.get("evidence_refs", []))),
                "affected_refs": deepcopy(list(entry.get("affected_refs", []))),
                "human_review_required": True,
                "provenance": deepcopy(dict(entry.get("provenance", provenance))),
            }
        )
    return sorted(normalized, key=lambda item: item["check_id"])


def _check(category: str, description: str, affected_refs: list[Mapping[str, Any]]) -> dict[str, Any]:
    return {
        "check_id": f"check:{category}",
        "category": category,
        "description": description,
        "status": "recorded",
        "evidence_refs": [_ref("Deliverable", "DEL-17-09")],
        "affected_refs": affected_refs,
    }


def _validation_report(
    *,
    validation_checklist: list[Mapping[str, Any]],
    diagnostics: list[Mapping[str, Any]],
    provenance: Mapping[str, Any],
) -> dict[str, Any]:
    status = "blocked" if any(item.get("severity") == "blocking" for item in diagnostics) else "boundary_checked"
    return {
        "validation_status": status,
        "check_count": len(validation_checklist),
        "blocking_diagnostic_count": sum(1 for item in diagnostics if item.get("severity") == "blocking"),
        "checks": deepcopy(validation_checklist),
        "provenance": deepcopy(dict(provenance)),
    }


def _package_members(sdk_id: str, checksums: Mapping[str, Mapping[str, Any]]) -> list[dict[str, Any]]:
    member_specs = [
        ("manifest", "manifest", "manifest.json"),
        ("target_registry", "target_registry", "target_registry.json"),
        ("adapter_contract", "adapter_contract", "adapter_contract.json"),
        ("validation_checklist", "validation_checklist", "validation_checklist.json"),
        ("validation_report", "validation_report", "validation_report.json"),
        ("diagnostics", "diagnostics", "diagnostics.json"),
    ]
    members = []
    for key, role, path in member_specs:
        members.append(
            {
                "member_id": f"{sdk_id}:{role}",
                "member_role": role,
                "path": path,
                "content_kind": "json",
                "payload_ref": _ref("ExportAdapterSdkMember", f"{sdk_id}:{role}"),
                "checksum": deepcopy(dict(checksums[key])),
            }
        )
    return members


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
    affected = affected_refs[0] if affected_refs else _ref("ExportAdapterSdkPackage", "unknown")
    return {
        "code": code,
        "class": diagnostic_class,
        "severity": severity,
        "source": _ref("Deliverable", "DEL-17-09"),
        "affected_object": deepcopy(dict(affected)),
        "message": message,
        "remediation": remediation,
        "provenance": deepcopy(ENGINE_PROVENANCE),
    }


def _ref(object_type: str, value: str) -> dict[str, str]:
    return {"object_type": object_type, "ref": value}


def _list(value: Any) -> list[Any]:
    return value if isinstance(value, list) else []


def _stable(items: list[Mapping[str, Any]]) -> list[dict[str, Any]]:
    return [deepcopy(dict(item)) for item in sorted(items, key=canonical_json)]


def _sort(value: Any) -> Any:
    return json.loads(canonical_json(value))
