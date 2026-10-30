"""Non-authoritative external-prover metadata contract for DEL-15-04.

The contract records workflow references, notes, attachments, assumptions,
warnings, and unsupported target flags. It does not invoke external tools,
parse commercial result formats, or create professional reliance records.
"""

from __future__ import annotations

from copy import deepcopy
import json
from typing import Any, Mapping

from .authority_boundary import contains_prohibited_authority_term


EXTERNAL_PROVER_METADATA_VERSION = "0.1.0"

SUPPORTED_REFERENCE_KINDS = {
    "external_reference",
    "handoff_package",
    "target_mapping",
    "export_workflow",
    "model_state",
    "attachment",
    "assumption",
    "warning",
    "unsupported_target_flag",
    "TBD",
}

SUPPORTED_ATTACHMENT_KINDS = {
    "metadata_note",
    "external_file_reference",
    "hash_reference",
    "review_note",
    "TBD",
}

ENGINE_PROVENANCE = {
    "source_name": "OpenPipeStress DEL-15-04 external prover boundary metadata",
    "source_location": "core/handoff/external_prover/metadata.py",
    "source_license": "project-governed",
    "contributor": "OpenPipeStress Type 2 worker",
    "contributor_attestation": "implementation-only-no-professional-claim",
    "redistribution_status": "public_permissive",
    "review_classification": "machine_checked",
    "privacy_classification": "public_metadata",
}

PROFESSIONAL_BOUNDARY = {
    "human_review_required": True,
    "metadata_only": True,
    "external_tool_invoked": False,
    "commercial_result_payload_ingested": False,
    "software_makes_compliance_claim": False,
    "software_makes_certification_claim": False,
    "software_makes_sealing_claim": False,
    "software_makes_approval_claim": False,
    "software_makes_authentication_claim": False,
    "software_creates_professional_reliance_record": False,
    "software_creates_external_validation_record": False,
}


def build_external_prover_metadata(
    *,
    metadata_record_id: str,
    names: list[Mapping[str, Any]] | None = None,
    tags: list[str] | None = None,
    notes: list[Mapping[str, Any]] | None = None,
    external_references: list[Mapping[str, Any]] | None = None,
    attachments: list[Mapping[str, Any]] | None = None,
    handoff_package_refs: list[Mapping[str, Any]] | None = None,
    target_mapping_refs: list[Mapping[str, Any]] | None = None,
    export_workflow_refs: list[Mapping[str, Any]] | None = None,
    immutable_model_state_refs: list[Mapping[str, Any]] | None = None,
    assumptions: list[Mapping[str, Any]] | None = None,
    warnings: list[Mapping[str, Any]] | None = None,
    unsupported_target_flags: list[Mapping[str, Any]] | None = None,
    proposed_authority_claims: list[Mapping[str, Any]] | None = None,
    provenance: Mapping[str, Any] | None = None,
) -> dict[str, Any]:
    """Build a deterministic external-prover metadata envelope."""

    record = {
        "schema_version": EXTERNAL_PROVER_METADATA_VERSION,
        "deliverable_id": "DEL-15-04",
        "package_id": "PKG-15",
        "scope_item": "SOW-075",
        "objectives": ["OBJ-017", "OBJ-018"],
        "metadata_record_id": str(metadata_record_id),
        "metadata_contract_status": "non_authoritative_workflow_metadata",
        "names": [_name(item, index) for index, item in enumerate(_list(names))],
        "tags": sorted(str(item) for item in _list(tags)),
        "notes": [_note(item, index) for index, item in enumerate(_list(notes))],
        "external_references": [
            _external_reference(item, index) for index, item in enumerate(_list(external_references))
        ],
        "attachments": [
            _attachment(item, index) for index, item in enumerate(_list(attachments))
        ],
        "handoff_package_refs": [
            _metadata_ref(item, index, "handoff_package") for index, item in enumerate(_list(handoff_package_refs))
        ],
        "target_mapping_refs": [
            _metadata_ref(item, index, "target_mapping") for index, item in enumerate(_list(target_mapping_refs))
        ],
        "export_workflow_refs": [
            _metadata_ref(item, index, "export_workflow") for index, item in enumerate(_list(export_workflow_refs))
        ],
        "immutable_model_state_refs": [
            _metadata_ref(item, index, "model_state") for index, item in enumerate(_list(immutable_model_state_refs))
        ],
        "assumptions": [
            _assumption(item, index) for index, item in enumerate(_list(assumptions))
        ],
        "warnings": [_warning(item, index) for index, item in enumerate(_list(warnings))],
        "unsupported_target_flags": [
            _unsupported_target_flag(item, index)
            for index, item in enumerate(_list(unsupported_target_flags))
        ],
        "proposed_authority_claims": [
            _proposed_claim(item, index)
            for index, item in enumerate(_list(proposed_authority_claims))
        ],
        "diagnostics": [],
        "provenance": deepcopy(dict(provenance or ENGINE_PROVENANCE)),
        "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
    }
    record["diagnostics"] = diagnostics_for_external_prover_metadata(record)
    return _sort_record(record)


def diagnostics_for_external_prover_metadata(record: Mapping[str, Any]) -> list[dict[str, Any]]:
    """Return deterministic diagnostics for an external-prover metadata envelope."""

    diagnostics: list[dict[str, Any]] = []
    if record.get("deliverable_id") != "DEL-15-04":
        diagnostics.append(
            _diagnostic(
                "EPM-RECORD-DELIVERABLE-MISMATCH",
                "blocking",
                "EXTERNAL_PROVER_METADATA_BOUNDARY",
                "External-prover metadata must use the DEL-15-04 envelope.",
                "Rebuild the record with build_external_prover_metadata.",
                [_affected("ExternalProverMetadata", str(record.get("metadata_record_id", "unknown")))],
            )
        )

    for field in (
        "names",
        "tags",
        "notes",
        "external_references",
        "attachments",
        "handoff_package_refs",
        "target_mapping_refs",
        "export_workflow_refs",
        "immutable_model_state_refs",
        "assumptions",
        "warnings",
        "unsupported_target_flags",
    ):
        for item in _list(record.get(field)):
            diagnostics.extend(_boundary_term_diagnostics(field, item))

    diagnostics.extend(_reference_diagnostics(record))
    diagnostics.extend(_attachment_diagnostics(record))
    diagnostics.extend(_unsupported_target_diagnostics(record))
    diagnostics.extend(_proposed_claim_diagnostics(record))
    diagnostics.extend(_professional_boundary_diagnostics(record))
    return sorted(diagnostics, key=canonical_json)


def canonical_json(value: Any) -> str:
    """Serialize metadata with stable key ordering."""

    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def _name(item: Mapping[str, Any], index: int) -> dict[str, Any]:
    return {
        "name_id": str(item.get("name_id", f"name:{index:04d}")),
        "label": str(item.get("label", "TBD")),
        "name_kind": str(item.get("name_kind", "external_workflow_label")),
        "language": str(item.get("language", "en")),
        "provenance": deepcopy(dict(item.get("provenance", ENGINE_PROVENANCE))),
    }


def _note(item: Mapping[str, Any], index: int) -> dict[str, Any]:
    return {
        "note_id": str(item.get("note_id", f"note:{index:04d}")),
        "note_kind": str(item.get("note_kind", "metadata_note")),
        "text": str(item.get("text", "")),
        "related_refs": deepcopy(_list(item.get("related_refs"))),
        "provenance": deepcopy(dict(item.get("provenance", ENGINE_PROVENANCE))),
    }


def _external_reference(item: Mapping[str, Any], index: int) -> dict[str, Any]:
    return {
        "reference_id": str(item.get("reference_id", f"external-ref:{index:04d}")),
        "reference_kind": str(item.get("reference_kind", "external_reference")),
        "display_name": str(item.get("display_name", "TBD")),
        "external_ref": deepcopy(dict(item.get("external_ref", _ref("ExternalReference", "external:TBD")))),
        "hash_refs": deepcopy(_list(item.get("hash_refs"))),
        "related_refs": deepcopy(_list(item.get("related_refs"))),
        "provenance": deepcopy(dict(item.get("provenance", ENGINE_PROVENANCE))),
    }


def _attachment(item: Mapping[str, Any], index: int) -> dict[str, Any]:
    return {
        "attachment_id": str(item.get("attachment_id", f"attachment:{index:04d}")),
        "attachment_kind": str(item.get("attachment_kind", "external_file_reference")),
        "display_name": str(item.get("display_name", "TBD")),
        "uri_or_path_ref": deepcopy(dict(item.get("uri_or_path_ref", _ref("ExternalReference", "attachment:TBD")))),
        "content_hash": deepcopy(item.get("content_hash")),
        "privacy_classification": str(item.get("privacy_classification", "public_metadata")),
        "payload_embedded": bool(item.get("payload_embedded", False)),
        "related_refs": deepcopy(_list(item.get("related_refs"))),
        "provenance": deepcopy(dict(item.get("provenance", ENGINE_PROVENANCE))),
    }


def _metadata_ref(item: Mapping[str, Any], index: int, default_kind: str) -> dict[str, Any]:
    return {
        "link_id": str(item.get("link_id", f"{default_kind}:{index:04d}")),
        "reference_kind": str(item.get("reference_kind", default_kind)),
        "ref": deepcopy(dict(item.get("ref", _ref("ExternalReference", f"{default_kind}:TBD")))),
        "hash_refs": deepcopy(_list(item.get("hash_refs"))),
        "relationship": str(item.get("relationship", "metadata_link")),
        "provenance": deepcopy(dict(item.get("provenance", ENGINE_PROVENANCE))),
    }


def _assumption(item: Mapping[str, Any], index: int) -> dict[str, Any]:
    return {
        "assumption_id": str(item.get("assumption_id", f"assumption:{index:04d}")),
        "statement": str(item.get("statement", "")),
        "affected_refs": deepcopy(_list(item.get("affected_refs"))),
        "provenance": deepcopy(dict(item.get("provenance", ENGINE_PROVENANCE))),
    }


def _warning(item: Mapping[str, Any], index: int) -> dict[str, Any]:
    return {
        "code": str(item.get("code", f"EPM-WARNING-{index:04d}")),
        "severity": str(item.get("severity", "warning")),
        "message": str(item.get("message", "")),
        "affected_refs": deepcopy(_list(item.get("affected_refs"))),
        "provenance": deepcopy(dict(item.get("provenance", ENGINE_PROVENANCE))),
    }


def _unsupported_target_flag(item: Mapping[str, Any], index: int) -> dict[str, Any]:
    return {
        "flag_id": str(item.get("flag_id", f"unsupported-target:{index:04d}")),
        "behavior_label": str(item.get("behavior_label", "unsupported_target_behavior")),
        "status": str(item.get("status", "unsupported")),
        "target_ref": deepcopy(dict(item.get("target_ref", _ref("ExternalReference", "target:TBD")))),
        "affected_refs": deepcopy(_list(item.get("affected_refs"))),
        "assumption_refs": deepcopy(_list(item.get("assumption_refs"))),
        "warning_refs": deepcopy(_list(item.get("warning_refs"))),
        "human_review_required": True,
        "provenance": deepcopy(dict(item.get("provenance", ENGINE_PROVENANCE))),
    }


def _proposed_claim(item: Mapping[str, Any], index: int) -> dict[str, Any]:
    return {
        "claim_id": str(item.get("claim_id", f"rejected-authority-claim:{index:04d}")),
        "claim_kind": str(item.get("claim_kind", "TBD")),
        "claim_text": str(item.get("claim_text", "")),
        "source_ref": deepcopy(dict(item.get("source_ref", _ref("ExternalReference", "authority-claim:TBD")))),
        "disposition": "rejected_boundary_claim",
        "provenance": deepcopy(dict(item.get("provenance", ENGINE_PROVENANCE))),
    }


def _reference_diagnostics(record: Mapping[str, Any]) -> list[dict[str, Any]]:
    diagnostics: list[dict[str, Any]] = []
    if not _list(record.get("external_references")):
        diagnostics.append(
            _diagnostic(
                "EPM-EXTERNAL-REFERENCE-MISSING",
                "warning",
                "EXTERNAL_PROVER_METADATA_BOUNDARY",
                "External-prover metadata has no external reference records.",
                "Add invented or cleared external references when available, or leave target selection as TBD.",
                [_affected("ExternalProverMetadata", str(record.get("metadata_record_id", "unknown")))],
            )
        )
    for field in (
        "handoff_package_refs",
        "target_mapping_refs",
        "export_workflow_refs",
        "immutable_model_state_refs",
    ):
        if not _list(record.get(field)):
            diagnostics.append(
                _diagnostic(
                    "EPM-CONTEXT-LINK-MISSING",
                    "warning",
                    "EXTERNAL_PROVER_METADATA_BOUNDARY",
                    f"External-prover metadata is missing {field}.",
                    "Carry related handoff, mapping, export, and immutable model state references when available.",
                    [_affected("ExternalProverMetadata", field)],
                )
            )
    return diagnostics


def _attachment_diagnostics(record: Mapping[str, Any]) -> list[dict[str, Any]]:
    diagnostics: list[dict[str, Any]] = []
    for attachment in _list(record.get("attachments")):
        attachment_id = str(attachment.get("attachment_id", "attachment:unknown"))
        if attachment.get("attachment_kind") not in SUPPORTED_ATTACHMENT_KINDS:
            diagnostics.append(
                _diagnostic(
                    "EPM-ATTACHMENT-KIND-UNSUPPORTED",
                    "warning",
                    "EXTERNAL_PROVER_METADATA_BOUNDARY",
                    "Attachment kind is outside the metadata-only taxonomy.",
                    "Use metadata_note, external_file_reference, hash_reference, review_note, or TBD.",
                    [_affected("Attachment", attachment_id)],
                )
            )
        if attachment.get("payload_embedded"):
            diagnostics.append(
                _diagnostic(
                    "EPM-ATTACHMENT-PAYLOAD-EMBEDDED",
                    "blocking",
                    "PRIVACY_WARNING",
                    "Attachment embeds payload content instead of a metadata reference.",
                    "Store only references, hashes, and cleared metadata in this contract.",
                    [_affected("Attachment", attachment_id)],
                )
            )
    return diagnostics


def _unsupported_target_diagnostics(record: Mapping[str, Any]) -> list[dict[str, Any]]:
    diagnostics: list[dict[str, Any]] = []
    for flag in _list(record.get("unsupported_target_flags")):
        flag_id = str(flag.get("flag_id", "unsupported-target:unknown"))
        if not flag.get("affected_refs"):
            diagnostics.append(
                _diagnostic(
                    "EPM-UNSUPPORTED-AFFECTED-REFS-MISSING",
                    "warning",
                    "UNSUPPORTED_BEHAVIOR_WARNING",
                    "Unsupported target flags should identify affected source or target records.",
                    "Add affected_refs for traceability.",
                    [_affected("UnsupportedTargetFlag", flag_id)],
                )
            )
    return diagnostics


def _proposed_claim_diagnostics(record: Mapping[str, Any]) -> list[dict[str, Any]]:
    diagnostics: list[dict[str, Any]] = []
    for claim in _list(record.get("proposed_authority_claims")):
        claim_id = str(claim.get("claim_id", "rejected-authority-claim:unknown"))
        diagnostics.append(
            _diagnostic(
                "EPM-AUTHORITY-CLAIM-REJECTED",
                "blocking",
                "AUTHORITY_BOUNDARY",
                "A proposed authority or lifecycle claim was rejected from the metadata contract.",
                "Keep this record as non-authoritative metadata and route any human-owned decision outside software-generated status.",
                [_affected("RejectedAuthorityClaim", claim_id)],
            )
        )
        diagnostics.extend(_boundary_term_diagnostics("proposed_authority_claims", claim))
    return diagnostics


def _professional_boundary_diagnostics(record: Mapping[str, Any]) -> list[dict[str, Any]]:
    boundary = record.get("professional_boundary")
    if not isinstance(boundary, Mapping):
        return [
            _diagnostic(
                "EPM-PROFESSIONAL-BOUNDARY-MISSING",
                "blocking",
                "AUTHORITY_BOUNDARY",
                "External-prover metadata must carry the professional boundary object.",
                "Rebuild the record with the standard boundary metadata.",
                [_affected("ExternalProverMetadata", str(record.get("metadata_record_id", "unknown")))],
            )
        ]
    diagnostics: list[dict[str, Any]] = []
    for key, value in boundary.items():
        if key.startswith("software_makes_") and value:
            diagnostics.append(
                _diagnostic(
                    "EPM-SOFTWARE-AUTHORITY-FLAG-BLOCKED",
                    "blocking",
                    "AUTHORITY_BOUNDARY",
                    "Software authority flags must remain false.",
                    "Remove automatic authority semantics from the metadata record.",
                    [_affected("ProfessionalBoundary", str(key))],
                )
            )
    if boundary.get("external_tool_invoked") or boundary.get("commercial_result_payload_ingested"):
        diagnostics.append(
            _diagnostic(
                "EPM-EXTERNAL-EXECUTION-BLOCKED",
                "blocking",
                "EXTERNAL_PROVER_METADATA_BOUNDARY",
                "External tool execution and commercial result ingestion are outside this metadata contract.",
                "Record only references and metadata.",
                [_affected("ProfessionalBoundary", "external_execution")],
            )
        )
    return diagnostics


def _boundary_term_diagnostics(field: str, value: Any) -> list[dict[str, Any]]:
    diagnostics: list[dict[str, Any]] = []
    if contains_prohibited_authority_term(value):
        diagnostics.append(
            _diagnostic(
                "EPM-PROHIBITED-AUTHORITY-TERM",
                "blocking",
                "AUTHORITY_BOUNDARY",
                "Metadata text contains a prohibited authority or lifecycle term.",
                "Replace authority wording with neutral external-reference metadata.",
                [_affected("ExternalProverMetadataField", field)],
            )
        )
    return diagnostics


def _diagnostic(
    code: str,
    severity: str,
    diagnostic_class: str,
    message: str,
    remediation: str,
    affected_refs: list[dict[str, str]],
) -> dict[str, Any]:
    return {
        "code": code,
        "severity": severity,
        "class": diagnostic_class,
        "source": _ref("ExternalReference", "core/handoff/external_prover/metadata.py"),
        "affected_references": affected_refs,
        "message": message,
        "remediation": remediation,
        "provenance": deepcopy(ENGINE_PROVENANCE),
    }


def _affected(object_type: str, ref: str) -> dict[str, str]:
    return {"object_type": object_type, "ref": ref}


def _ref(object_type: str, ref: str) -> dict[str, str]:
    return {"object_type": object_type, "ref": ref}


def _list(value: Any) -> list[Any]:
    return value if isinstance(value, list) else []


def _sort_record(record: dict[str, Any]) -> dict[str, Any]:
    for field, key in (
        ("names", "name_id"),
        ("notes", "note_id"),
        ("external_references", "reference_id"),
        ("attachments", "attachment_id"),
        ("handoff_package_refs", "link_id"),
        ("target_mapping_refs", "link_id"),
        ("export_workflow_refs", "link_id"),
        ("immutable_model_state_refs", "link_id"),
        ("assumptions", "assumption_id"),
        ("warnings", "code"),
        ("unsupported_target_flags", "flag_id"),
        ("proposed_authority_claims", "claim_id"),
    ):
        record[field].sort(key=lambda item, sort_key=key: str(item.get(sort_key, "")))
    record["diagnostics"].sort(key=canonical_json)
    return record
