#!/usr/bin/env python3
"""Focused tests for DEL-15-04 external-prover boundary metadata."""

from __future__ import annotations

from copy import deepcopy
from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.handoff.external_prover import (  # noqa: E402
    build_external_prover_metadata,
    canonical_json,
    diagnostics_for_external_prover_metadata,
)


FORBIDDEN_OUTPUT_PHRASES = {
    "code " + "compliant",
    "cert" + "ified",
    "se" + "aled",
    "authentic" + "ated",
    "professional " + "approval",
    "external " + "validation",
    "engineering " + "acceptance",
}


def ref(object_type: str, value: str) -> dict[str, str]:
    return {"object_type": object_type, "ref": value}


def provenance(source_name: str = "Invented DEL-15-04 fixture") -> dict[str, str]:
    return {
        "source_name": source_name,
        "source_location": "tests/test_external_prover_boundary_metadata.py",
        "source_license": "project-invented-test-data",
        "contributor": "OpenPipeStress",
        "contributor_attestation": "invented non-engineering fixture",
        "redistribution_status": "invented_non_engineering_example",
        "review_classification": "machine_checked",
        "privacy_classification": "invented_public_example",
    }


def checksum(payload_ref: dict[str, str], scope: str) -> dict[str, object]:
    return {
        "algorithm": "sha256",
        "canonicalization": "JCS_compatible_json_payload_hash",
        "payload_ref": payload_ref,
        "payload_scope": scope,
        "value": f"sha256:invented-{scope}",
    }


def metadata_kwargs() -> dict[str, object]:
    return {
        "metadata_record_id": "external-prover-metadata:invented-del-15-04",
        "names": [
            {
                "name_id": "name:workflow",
                "label": "Invented external reference workflow",
                "name_kind": "external_workflow_label",
                "provenance": provenance("Invented workflow name"),
            }
        ],
        "tags": ["invented", "metadata-only", "provider-neutral"],
        "notes": [
            {
                "note_id": "note:boundary",
                "note_kind": "metadata_note",
                "text": "Invented metadata record for downstream review context.",
                "related_refs": [ref("HandoffPackage", "handoff:invented-del-15-04")],
                "provenance": provenance("Invented metadata note"),
            }
        ],
        "external_references": [
            {
                "reference_id": "external-ref:generic-workspace",
                "reference_kind": "external_reference",
                "display_name": "Invented external workspace reference",
                "external_ref": ref("ExternalReference", "external:invented-workspace"),
                "hash_refs": [checksum(ref("ExternalReference", "external:invented-workspace"), "external_ref")],
                "related_refs": [ref("TargetMapping", "tm:invented-del-15-04")],
                "provenance": provenance("Invented external reference"),
            }
        ],
        "attachments": [
            {
                "attachment_id": "attachment:metadata-note",
                "attachment_kind": "external_file_reference",
                "display_name": "Invented metadata attachment reference",
                "uri_or_path_ref": ref("ExternalReference", "attachment:invented-note"),
                "content_hash": checksum(ref("ExternalReference", "attachment:invented-note"), "attachment"),
                "privacy_classification": "invented_public_example",
                "payload_embedded": False,
                "related_refs": [ref("ExportWorkflow", "export:invented-del-15-04")],
                "provenance": provenance("Invented attachment reference"),
            }
        ],
        "handoff_package_refs": [
            {
                "link_id": "handoff:invented-del-15-04",
                "ref": ref("HandoffPackage", "handoff:invented-del-15-04"),
                "hash_refs": [checksum(ref("HandoffPackage", "handoff:invented-del-15-04"), "handoff")],
            }
        ],
        "target_mapping_refs": [
            {
                "link_id": "tm:invented-del-15-04",
                "ref": ref("TargetMapping", "tm:invented-del-15-04"),
                "hash_refs": [checksum(ref("TargetMapping", "tm:invented-del-15-04"), "target_mapping")],
            }
        ],
        "export_workflow_refs": [
            {
                "link_id": "export:invented-del-15-04",
                "ref": ref("ExportWorkflow", "export:invented-del-15-04"),
                "hash_refs": [checksum(ref("ExportWorkflow", "export:invented-del-15-04"), "export")],
            }
        ],
        "immutable_model_state_refs": [
            {
                "link_id": "state:invented-del-15-04",
                "ref": ref("ModelState", "state:invented-del-15-04"),
                "hash_refs": [checksum(ref("ModelState", "state:invented-del-15-04"), "model_state")],
            }
        ],
        "assumptions": [
            {
                "assumption_id": "assumption:external-review",
                "statement": "Invented downstream external context requires human review before reliance.",
                "affected_refs": [ref("ExternalReference", "external:invented-workspace")],
                "provenance": provenance("Invented assumption"),
            }
        ],
        "warnings": [
            {
                "code": "EPM-INVENTED-WARNING",
                "severity": "warning",
                "message": "Invented metadata warning for traceability.",
                "affected_refs": [ref("ExternalReference", "external:invented-workspace")],
                "provenance": provenance("Invented warning"),
            }
        ],
        "unsupported_target_flags": [
            {
                "flag_id": "unsupported-target:solver-not-invoked",
                "behavior_label": "external_solver_not_invoked",
                "status": "not_implemented",
                "target_ref": ref("ExternalReference", "external:invented-workspace"),
                "affected_refs": [ref("ExportWorkflow", "export:invented-del-15-04")],
                "assumption_refs": [ref("Assumption", "assumption:external-review")],
                "warning_refs": [ref("Diagnostic", "EPM-INVENTED-WARNING")],
                "provenance": provenance("Invented unsupported flag"),
            }
        ],
        "provenance": provenance(),
    }


def test_metadata_is_deterministic_and_preserves_boundary_links():
    first = build_external_prover_metadata(**metadata_kwargs())
    second = build_external_prover_metadata(**metadata_kwargs())

    assert canonical_json(first) == canonical_json(second)
    assert first["deliverable_id"] == "DEL-15-04"
    assert first["metadata_contract_status"] == "non_authoritative_workflow_metadata"
    assert first["external_references"][0]["external_ref"]["ref"] == "external:invented-workspace"
    assert first["handoff_package_refs"][0]["ref"]["ref"] == "handoff:invented-del-15-04"
    assert first["target_mapping_refs"][0]["ref"]["ref"] == "tm:invented-del-15-04"
    assert first["export_workflow_refs"][0]["ref"]["ref"] == "export:invented-del-15-04"
    assert first["immutable_model_state_refs"][0]["ref"]["ref"] == "state:invented-del-15-04"
    assert first["unsupported_target_flags"][0]["human_review_required"] is True
    assert not [item for item in first["diagnostics"] if item["severity"] == "blocking"]


def test_proposed_authority_and_lifecycle_claims_are_blocking_diagnostics():
    kwargs = metadata_kwargs()
    kwargs["proposed_authority_claims"] = [
        {
            "claim_id": "claim:authority",
            "claim_kind": "prover_" + "status",
            "claim_text": "Software " + "cert" + "ified external " + "validation lifecycle.",
            "source_ref": ref("ExternalReference", "external:invented-workspace"),
            "provenance": provenance("Invented rejected claim"),
        }
    ]

    record = build_external_prover_metadata(**kwargs)
    codes = {item["code"] for item in record["diagnostics"]}
    assert "EPM-AUTHORITY-CLAIM-REJECTED" in codes
    assert "EPM-PROHIBITED-AUTHORITY-TERM" in codes
    assert any(item["severity"] == "blocking" for item in record["diagnostics"])
    assert record["proposed_authority_claims"][0]["disposition"] == "rejected_boundary_claim"


def test_embedded_attachment_payload_is_blocked():
    kwargs = metadata_kwargs()
    attachments = deepcopy(kwargs["attachments"])
    attachments[0]["payload_embedded"] = True
    kwargs["attachments"] = attachments

    record = build_external_prover_metadata(**kwargs)
    codes = {item["code"] for item in record["diagnostics"]}
    assert "EPM-ATTACHMENT-PAYLOAD-EMBEDDED" in codes
    assert any(item["severity"] == "blocking" for item in record["diagnostics"])


def test_boundary_flags_cannot_be_flipped_to_software_authority():
    record = build_external_prover_metadata(**metadata_kwargs())
    mutated = deepcopy(record)
    mutated["professional_boundary"]["software_makes_approval_claim"] = True
    mutated["professional_boundary"]["external_tool_invoked"] = True

    codes = {item["code"] for item in diagnostics_for_external_prover_metadata(mutated)}
    assert "EPM-SOFTWARE-AUTHORITY-FLAG-BLOCKED" in codes
    assert "EPM-EXTERNAL-EXECUTION-BLOCKED" in codes


def test_notes_and_tags_authority_wording_are_blocking_diagnostics():
    kwargs = metadata_kwargs()
    kwargs["tags"] = ["metadata-only", "approved"]
    kwargs["notes"] = [
        {
            "note_id": "note:authority-wording",
            "note_kind": "metadata_note",
            "text": "Downstream reviewer marked this as " + "cert" + "ified.",
            "related_refs": [ref("HandoffPackage", "handoff:invented-del-15-04")],
            "provenance": provenance("Invented boundary probe note"),
        }
    ]

    record = build_external_prover_metadata(**kwargs)
    codes = {item["code"] for item in record["diagnostics"]}
    affected = {
        item["affected_references"][0]["ref"]
        for item in record["diagnostics"]
        if item["code"] == "EPM-PROHIBITED-AUTHORITY-TERM"
    }
    assert "EPM-PROHIBITED-AUTHORITY-TERM" in codes
    assert {"tags", "notes"} <= affected
    assert any(item["severity"] == "blocking" for item in record["diagnostics"])


def test_output_boundary_language_does_not_make_prohibited_claims():
    record = build_external_prover_metadata(**metadata_kwargs())
    text = canonical_json(record).lower()

    for forbidden in FORBIDDEN_OUTPUT_PHRASES:
        assert forbidden not in text
    assert record["professional_boundary"]["external_tool_invoked"] is False
    assert record["professional_boundary"]["commercial_result_payload_ingested"] is False
    assert record["professional_boundary"]["software_creates_professional_reliance_record"] is False


def main():
    test_metadata_is_deterministic_and_preserves_boundary_links()
    test_proposed_authority_and_lifecycle_claims_are_blocking_diagnostics()
    test_embedded_attachment_payload_is_blocked()
    test_boundary_flags_cannot_be_flipped_to_software_authority()
    test_notes_and_tags_authority_wording_are_blocking_diagnostics()
    test_output_boundary_language_does_not_make_prohibited_claims()


if __name__ == "__main__":
    main()
