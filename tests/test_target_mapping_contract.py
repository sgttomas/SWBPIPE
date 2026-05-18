#!/usr/bin/env python3
"""Focused tests for DEL-15-02 target mapping contract."""

from __future__ import annotations

import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.handoff.target_mapping import (  # noqa: E402
    build_target_mapping_contract,
    canonical_json,
    diagnostics_for_target_mapping_contract,
)


def ref(object_type, value):
    return {"object_type": object_type, "ref": value}


def source_context():
    return {
        "model_hash": {
            "algorithm": "sha256",
            "value": "invented-hash",
            "payload_ref": ref("Model", "model:invented"),
        },
        "units_manifest_ref": ref("ExternalReference", "units:invented"),
        "entity_id_refs": [ref("Component", "component:pipe-1")],
        "library_refs": [ref("ExternalReference", "library:public-metadata")],
        "rule_pack_refs": [ref("ExternalReference", "rule-pack:metadata-only")],
        "unresolved_assumption_refs": [ref("Diagnostic", "assumption:review")],
        "warning_refs": [ref("Diagnostic", "warning:handoff")],
        "privacy_context": {
            "private_payload_redacted": True,
            "privacy_classification": "public_metadata",
        },
    }


def mapping_record():
    return {
        "mapping_id": "mapping:pipe-diameter",
        "mapping_kind": "field",
        "source_ref": ref("Component", "component:pipe-1"),
        "target_ref": ref("ExternalReference", "target:pipe-diameter"),
        "value_kind": "quantity",
        "unit_metadata": {"unit": "mm", "dimension": "length"},
        "mapping_status": "mapped",
        "assumption_refs": [],
        "warning_refs": [],
    }


def unsupported_flag():
    return {
        "flag_id": "unsupported:mesh",
        "behavior_label": "mesh_generation_not_performed",
        "status": "not_implemented",
        "target_ref": ref("ExternalReference", "target:mesh"),
        "affected_refs": [ref("Model", "model:invented")],
    }


def test_contract_is_deterministic_and_preserves_handoff_context():
    first = build_target_mapping_contract(
        mapping_contract_id="tm:invented",
        target_system_kind="generic_downstream_modeling",
        target_ref=ref("ExternalReference", "target:generic"),
        source_context=source_context(),
        mapping_records=[mapping_record()],
        unsupported_behaviors=[unsupported_flag()],
    )
    second = build_target_mapping_contract(
        mapping_contract_id="tm:invented",
        target_system_kind="generic_downstream_modeling",
        target_ref=ref("ExternalReference", "target:generic"),
        source_context=source_context(),
        mapping_records=[mapping_record()],
        unsupported_behaviors=[unsupported_flag()],
    )

    assert canonical_json(first) == canonical_json(second)
    assert first["deliverable_id"] == "DEL-15-02"
    assert first["source_context"]["model_hash"]["value"] == "invented-hash"
    assert first["source_context"]["units_manifest_ref"]["ref"] == "units:invented"
    assert first["unsupported_behavior_flags"][0]["human_review_required"] is True
    assert not [item for item in first["diagnostics"] if item["severity"] == "blocking"]


def test_unit_bearing_mapping_without_unit_metadata_is_blocked():
    broken = mapping_record()
    broken["unit_metadata"] = None
    contract = build_target_mapping_contract(
        mapping_contract_id="tm:unit-missing",
        target_system_kind="generic_downstream_modeling",
        target_ref=ref("ExternalReference", "target:generic"),
        source_context=source_context(),
        mapping_records=[broken],
    )

    codes = {item["code"] for item in contract["diagnostics"]}
    assert "TM-UNIT-METADATA-MISSING" in codes
    assert any(item["severity"] == "blocking" for item in contract["diagnostics"])


def test_missing_context_and_untraceable_behavior_emit_diagnostics():
    contract = build_target_mapping_contract(
        mapping_contract_id="tm:missing-context",
        target_system_kind="generic_downstream_modeling",
        target_ref=ref("ExternalReference", "target:generic"),
        source_context={},
        mapping_records=[],
        unsupported_behaviors=[{"flag_id": "unsupported:untraced"}],
    )

    codes = {item["code"] for item in diagnostics_for_target_mapping_contract(contract)}
    assert "TM-SOURCE-CONTEXT-FIELD-MISSING" in codes
    assert "TM-AFFECTED-REFS-MISSING" in codes


def test_behavior_label_authority_wording_is_blocking_boundary_diagnostic():
    flagged = unsupported_flag()
    flagged["behavior_label"] = "cert" + "ified_by_target"

    contract = build_target_mapping_contract(
        mapping_contract_id="tm:authority-label",
        target_system_kind="generic_downstream_modeling",
        target_ref=ref("ExternalReference", "target:generic"),
        source_context=source_context(),
        mapping_records=[mapping_record()],
        unsupported_behaviors=[flagged],
    )

    codes = {item["code"] for item in contract["diagnostics"]}
    assert "TM-PROHIBITED-AUTHORITY-TERM" in codes
    assert any(item["severity"] == "blocking" for item in contract["diagnostics"])


def test_output_boundary_language_does_not_make_prohibited_claims():
    contract = build_target_mapping_contract(
        mapping_contract_id="tm:boundary",
        target_system_kind="TBD",
        target_ref=ref("ExternalReference", "target:tbd"),
        source_context=source_context(),
        mapping_records=[mapping_record()],
    )
    text = canonical_json(contract).lower()

    for forbidden in [
        "code compliant",
        "certified",
        "sealed",
        "authenticated",
        "professional approval",
        "external validation",
    ]:
        assert forbidden not in text
    assert contract["professional_boundary"]["software_makes_compliance_claim"] is False


def main():
    test_contract_is_deterministic_and_preserves_handoff_context()
    test_unit_bearing_mapping_without_unit_metadata_is_blocked()
    test_missing_context_and_untraceable_behavior_emit_diagnostics()
    test_behavior_label_authority_wording_is_blocking_boundary_diagnostic()
    test_output_boundary_language_does_not_make_prohibited_claims()


if __name__ == "__main__":
    main()
