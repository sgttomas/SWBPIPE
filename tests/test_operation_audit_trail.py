#!/usr/bin/env python3
"""Focused tests for DEL-16-03 user acceptance operation audit trail."""

from __future__ import annotations

from copy import deepcopy
import json
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.model_operations.audit_trail import canonical_json, record_operation_audit_trail  # noqa: E402
from core.model_operations.validation_preview import validate_and_preview_operations  # noqa: E402


OPERATION_FIXTURE = ROOT / "fixtures" / "model_operations" / "invented_operation_set_valid.json"
MODEL_STATE_FIXTURE = ROOT / "fixtures" / "model_operations" / "invented_accepted_model_state.json"


def ref(object_type, value):
    return {"object_type": object_type, "ref": value}


def professional_boundary():
    return {
        "human_review_required": True,
        "software_makes_compliance_claim": False,
        "software_makes_certification_claim": False,
        "software_makes_sealing_claim": False,
        "software_makes_approval_claim": False,
        "software_makes_authentication_claim": False,
    }


def validation():
    return {
        "schema_validation": "pending",
        "constraint_validation": "pending",
        "unit_validation": "pending",
        "diff_preview_status": "not_generated",
        "application_status": "not_applied",
    }


def model_state():
    with MODEL_STATE_FIXTURE.open(encoding="utf-8") as fixture_file:
        return json.load(fixture_file)


def quantity_change():
    return {
        "change_id": "change:diameter",
        "change_kind": "set_field",
        "target_object_type": "Component",
        "target_ref": ref("Component", "component:pipe-1"),
        "value_payload": {
            "value_kind": "quantity",
            "scalar_values": [],
            "quantity_values": [{"value": 125.0, "unit": "mm", "dimension": "length"}],
            "reference_values": [],
            "structured_values": [{"diameter": {"value": 125.0, "unit": "mm", "dimension": "length"}}],
            "notes": [],
        },
        "unit_requirements": {
            "unit_metadata_required": True,
            "dimension_check_required": True,
            "missing_unit_behavior": "emit_diagnostic",
        },
    }


def operation_envelope(*, assumptions=None):
    with OPERATION_FIXTURE.open(encoding="utf-8") as fixture_file:
        envelope = json.load(fixture_file)
    operation = next(
        item for item in envelope["operation_set"]["operations"] if item["operation_id"] == "op:resize"
    )
    operation["operation_status"] = "ready_for_user_review"
    operation["assumptions"] = assumptions or []
    envelope["operation_set"]["operations"] = [operation]
    return envelope


def assumption(assumption_id, status):
    with OPERATION_FIXTURE.open(encoding="utf-8") as fixture_file:
        envelope = json.load(fixture_file)
    provenance = next(
        item for item in envelope["operation_set"]["operations"] if item["operation_id"] == "op:resize"
    )["provenance"]
    return {
        "assumption_id": assumption_id,
        "statement": "Invented operation assumption for audit trail testing.",
        "status": status,
        "affected_refs": [ref("ModelOperation", "op:resize")],
        "provenance": deepcopy(provenance),
    }


def user_acceptance():
    return {
        "decision": "accept",
        "accepted": True,
        "actor_type": "user",
        "actor_ref": "user:reviewer",
        "source_role": "project_user",
        "decided_at": "2026-05-06T12:00:00Z",
        "rationale": "Invented example review accepted for audit trail test.",
    }


def user_rejection():
    return {
        "decision": "reject",
        "accepted": False,
        "actor_type": "user",
        "actor_ref": "user:reviewer",
        "source_role": "project_user",
        "decided_at": "2026-05-06T12:05:00Z",
        "rationale": "Invented example review rejected for audit trail test.",
    }


def preview_ref():
    return {
        "object_type": "DiffPreview",
        "ref": "preview:resize",
        "hash": "sha256:invented-preview",
    }


def test_user_accepted_operation_record_is_deterministic_and_complete():
    envelope = operation_envelope(assumptions=[assumption("asm:1", "unresolved")])
    preview = validate_and_preview_operations(envelope, model_state())
    first = record_operation_audit_trail(
        envelope,
        validation_outcome=preview,
        diff_preview_ref=preview_ref(),
        acceptance_signal=user_acceptance(),
        actor={"actor_type": "user", "actor_ref": "user:reviewer"},
        source={"source_ref": "operation-workbench:invented", "source_channel": "test"},
        accepted_model_state=model_state(),
    )
    second = record_operation_audit_trail(
        deepcopy(envelope),
        validation_outcome=deepcopy(preview),
        diff_preview_ref=preview_ref(),
        acceptance_signal=user_acceptance(),
        actor={"actor_type": "user", "actor_ref": "user:reviewer"},
        source={"source_ref": "operation-workbench:invented", "source_channel": "test"},
        accepted_model_state=model_state(),
    )

    assert canonical_json(first) == canonical_json(second)
    assert first["decision_counts"] == {"accepted": 1, "rejected": 0, "held_for_user_acceptance": 0}
    record = first["records"][0]
    assert record["decision"]["status"] == "accepted"
    assert record["decision"]["explicit_user_acceptance"] is True
    assert record["decision"]["accepted_model_state_mutated"] is False
    assert record["affected_entities"] == [ref("Component", "component:pipe-1")]
    assert record["operation_history"]["changes"][0]["change_id"] == "change:diameter"
    assert record["validation_outcome"]["validation"]["diff_preview_status"] == "generated"
    assert record["diff_preview_ref"]["ref"] == "preview:resize"
    assert record["unresolved_assumptions"][0]["assumption_id"] == "asm:1"


def test_operation_cannot_be_accepted_without_explicit_user_signal():
    result = record_operation_audit_trail(
        operation_envelope(),
        validation_outcome=validate_and_preview_operations(operation_envelope(), model_state()),
        diff_preview_ref=preview_ref(),
        acceptance_signal={"decision": "accept", "accepted": True, "actor_type": "agent"},
        accepted_model_state=model_state(),
    )

    record = result["records"][0]
    assert record["decision"]["status"] == "held_for_user_acceptance"
    assert record["decision"]["explicit_user_acceptance"] is False
    assert "AUDIT-EXPLICIT-USER-ACCEPTANCE-REQUIRED" in {item["code"] for item in record["diagnostics"]}


def test_blocked_validation_outcome_prevents_accepted_record():
    envelope = operation_envelope()
    accepted = model_state()
    blocked_envelope = deepcopy(envelope)
    blocked_envelope["operation_set"]["operations"][0]["preconditions"]["required_current_hashes"][0][
        "value"
    ] = "sha256:stale"
    blocked_preview = validate_and_preview_operations(blocked_envelope, accepted)

    result = record_operation_audit_trail(
        envelope,
        validation_outcome=blocked_preview,
        diff_preview_ref=preview_ref(),
        acceptance_signal=user_acceptance(),
        accepted_model_state=accepted,
    )

    record = result["records"][0]
    assert record["decision"]["status"] == "held_for_user_acceptance"
    assert "AUDIT-VALIDATION-GATES-BLOCKING" in {item["code"] for item in record["diagnostics"]}


def test_current_model_state_hash_is_required_for_accepted_record():
    accepted = model_state()
    del accepted["model_state"]["state_hash"]
    preview = validate_and_preview_operations(operation_envelope(), model_state())

    result = record_operation_audit_trail(
        operation_envelope(),
        validation_outcome=preview,
        diff_preview_ref=preview_ref(),
        acceptance_signal=user_acceptance(),
        accepted_model_state=accepted,
    )

    record = result["records"][0]
    assert record["decision"]["status"] == "held_for_user_acceptance"
    assert "AUDIT-MODEL-STATE-HASH-MISSING" in {item["code"] for item in record["diagnostics"]}


def test_rejected_operation_is_recorded_without_mutating_accepted_state():
    accepted = model_state()
    original = deepcopy(accepted)
    result = record_operation_audit_trail(
        operation_envelope(),
        validation_outcome=validate_and_preview_operations(operation_envelope(), accepted),
        diff_preview_ref=preview_ref(),
        acceptance_signal=user_rejection(),
        accepted_model_state=accepted,
    )

    assert accepted == original
    assert result["accepted_model_state_unchanged"] is True
    assert result["decision_counts"] == {"accepted": 0, "rejected": 1, "held_for_user_acceptance": 0}
    assert result["records"][0]["decision"]["status"] == "rejected"
    assert result["records"][0]["decision"]["accepted_model_state_mutated"] is False


def test_missing_inputs_are_visible_tbd_diagnostics():
    result = record_operation_audit_trail(operation_envelope())
    top_codes = {item["code"] for item in result["diagnostics"]}
    record_codes = {item["code"] for item in result["records"][0]["diagnostics"]}

    assert {"AUDIT-VALIDATION-OUTCOME-TBD", "AUDIT-DIFF-PREVIEW-REF-TBD", "AUDIT-USER-DECISION-TBD"} <= top_codes
    assert "AUDIT-TIMESTAMP-TBD" in record_codes
    assert "AUDIT-RATIONALE-TBD" in record_codes
    assert result["records"][0]["validation_outcome"]["status"] == "TBD"
    assert result["records"][0]["diff_preview_ref"]["ref"] == "TBD"


def test_output_boundary_language_does_not_make_prohibited_claims():
    result = record_operation_audit_trail(
        operation_envelope(),
        validation_outcome=validate_and_preview_operations(operation_envelope(), model_state()),
        diff_preview_ref=preview_ref(),
        acceptance_signal=user_acceptance(),
    )
    text = canonical_json(result).lower()

    for forbidden in [
        "code compliant",
        "certified",
        "sealed",
        "authenticated",
        "professional approval",
        "engineering acceptance",
    ]:
        assert forbidden not in text
    assert result["professional_boundary"]["software_makes_approval_claim"] is False


def main():
    test_user_accepted_operation_record_is_deterministic_and_complete()
    test_operation_cannot_be_accepted_without_explicit_user_signal()
    test_blocked_validation_outcome_prevents_accepted_record()
    test_current_model_state_hash_is_required_for_accepted_record()
    test_rejected_operation_is_recorded_without_mutating_accepted_state()
    test_missing_inputs_are_visible_tbd_diagnostics()
    test_output_boundary_language_does_not_make_prohibited_claims()


if __name__ == "__main__":
    main()
