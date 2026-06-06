#!/usr/bin/env python3
"""Focused tests for DEL-16-04 agent rationale boundary controls."""

from __future__ import annotations

from copy import deepcopy
import json
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.model_operations.agent_rationale import canonical_json, record_agent_rationale  # noqa: E402
from core.model_operations.audit_trail import record_operation_audit_trail  # noqa: E402
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
        "statement": "Invented operation assumption for rationale testing.",
        "status": status,
        "affected_refs": [ref("ModelOperation", "op:resize")],
        "provenance": deepcopy(provenance),
    }


def acceptance_signal():
    return {
        "decision": "accept",
        "accepted": True,
        "actor_type": "user",
        "actor_ref": "user:reviewer",
        "source_role": "project_user",
        "decided_at": "2026-05-06T12:00:00Z",
        "rationale": "Invented example review accepted for audit trail test.",
    }


def preview_ref():
    return {
        "object_type": "DiffPreview",
        "ref": "preview:resize",
        "hash": "sha256:invented-preview",
    }


def audit_and_preview(envelope):
    preview = validate_and_preview_operations(envelope, model_state())
    audit = record_operation_audit_trail(
        envelope,
        validation_outcome=preview,
        diff_preview_ref=preview_ref(),
        acceptance_signal=acceptance_signal(),
        actor={"actor_type": "user", "actor_ref": "user:reviewer"},
        source={"source_ref": "operation-workbench:invented", "source_channel": "test"},
        accepted_model_state=model_state(),
    )
    return audit, preview


def rationale_record(envelope, *, rationale_text="Agent notes the diameter change needs user review."):
    audit, preview = audit_and_preview(envelope)
    return record_agent_rationale(
        envelope,
        audit_trail=audit,
        validation_context=preview,
        source={"source_ref": "agent:invented", "source_channel": "test", "source_kind": "agent"},
        actor={"actor_type": "agent", "actor_ref": "agent:worker", "actor_role": "proposal_support"},
        rationale_text=rationale_text,
        assumptions=[{"assumption_id": "asm:rationale", "status": "pending"}],
        audit_references=[{"object_type": "RunNote", "ref": "run:invented"}],
        timestamp="2026-05-06T12:10:00Z",
        accepted_model_state=model_state(),
    )


def test_rationale_record_is_deterministic_and_preserves_context():
    envelope = operation_envelope(assumptions=[assumption("asm:operation", "unresolved")])
    first = rationale_record(envelope)
    second = rationale_record(deepcopy(envelope))

    assert canonical_json(first) == canonical_json(second)
    assert first["operation_set_ref"] == ref("ModelOperation", "ops:invented")
    assert first["source"]["source_ref"] == "agent:invented"
    assert first["actor"]["actor_type"] == "agent"
    assert first["rationale"]["decision_support_only"] is True
    assert first["rationale"]["creates_accepted_operation_record"] is False
    assert first["rationale"]["mutates_accepted_model_state"] is False
    assert first["rationale"]["bypasses_user_acceptance"] is False
    assert first["affected_entities"] == [ref("Component", "component:pipe-1")]
    assert first["audit_context"]["audit_trail_hash"] != "TBD"
    assert first["validation_context"]["validation"]["diff_preview_status"] == "generated"
    assert first["assumptions"]["unresolved_count"] == 2
    assert {item["assumption_id"] for item in first["assumptions"]["unresolved"]} == {
        "asm:operation",
        "asm:rationale",
    }


def test_rationale_cannot_mutate_accepted_state_or_bypass_user_acceptance():
    accepted = model_state()
    original = deepcopy(accepted)
    result = record_agent_rationale(
        operation_envelope(),
        rationale_text="Invented proposal note for later user review.",
        accepted_model_state=accepted,
    )

    assert accepted == original
    assert result["accepted_model_state_unchanged"] is True
    assert result["rationale"]["creates_accepted_operation_record"] is False
    assert result["rationale"]["mutates_accepted_model_state"] is False
    assert result["professional_boundary"]["software_can_accept_engineering_work"] is False
    assert result["professional_boundary"]["software_can_mutate_accepted_model_state"] is False
    assert "RATIONALE-AUDIT-CONTEXT-TBD" in {item["code"] for item in result["diagnostics"]}


def test_missing_context_and_unresolved_assumptions_are_visible():
    result = record_agent_rationale(
        operation_envelope(assumptions=[assumption("asm:missing-context", "TBD")])
    )
    codes = {item["code"] for item in result["diagnostics"]}

    assert {
        "RATIONALE-AUDIT-CONTEXT-TBD",
        "RATIONALE-VALIDATION-CONTEXT-TBD",
        "RATIONALE-SOURCE-TBD",
        "RATIONALE-ACTOR-TBD",
        "RATIONALE-TEXT-TBD",
    } <= codes
    assert result["validation_context"]["status"] == "TBD"
    assert result["audit_context"]["audit_trail_hash"] == "TBD"
    assert result["assumptions"]["unresolved_count"] == 1
    assert result["assumptions"]["unresolved"][0]["status"] == "TBD"


def test_prohibited_professional_boundary_claims_are_blocked():
    result = rationale_record(
        operation_envelope(),
        rationale_text=(
            "This invented operation is code compliant, certified, sealed, authenticated, "
            "externally validated, has professional approval, and has autonomous engineering acceptance."
        ),
    )

    codes = {item["code"] for item in result["diagnostics"]}
    assert "RATIONALE-AUTHORITY-COMPLIANCE-BLOCKED" in codes
    assert "RATIONALE-AUTHORITY-CERTIFICATION-BLOCKED" in codes
    assert "RATIONALE-AUTHORITY-SEALING-BLOCKED" in codes
    assert "RATIONALE-AUTHORITY-AUTHENTICATION-BLOCKED" in codes
    assert "RATIONALE-AUTHORITY-PROFESSIONAL-APPROVAL-BLOCKED" in codes
    assert "RATIONALE-AUTHORITY-EXTERNAL-VALIDATION-BLOCKED" in codes
    assert "RATIONALE-AUTHORITY-AUTONOMOUS-ACCEPTANCE-BLOCKED" in codes
    assert result["rationale"]["status"] == "blocked_by_professional_boundary"


def test_prohibited_claims_in_copied_context_are_blocked():
    envelope = operation_envelope()
    audit, preview = audit_and_preview(envelope)
    envelope["operation_set"]["operations"][0]["diagnostics"].append(
        {"message": "Copied operation diagnostic says code compliant."}
    )
    preview["diagnostics"].append({"message": "Copied validation diagnostic says certified."})
    audit["records"][0]["decision"]["rationale"] = "Copied audit context says professional approval."

    result = record_agent_rationale(
        envelope,
        audit_trail=audit,
        validation_context=preview,
        source={"source_ref": "agent:invented", "source_channel": "test", "source_kind": "agent"},
        actor={"actor_type": "agent", "actor_ref": "agent:worker", "actor_role": "proposal_support"},
        rationale_text="Agent notes the proposal needs user review.",
        timestamp="2026-05-06T12:10:00Z",
        accepted_model_state=model_state(),
    )

    codes = {item["code"] for item in result["diagnostics"]}
    assert "RATIONALE-AUTHORITY-COMPLIANCE-BLOCKED" in codes
    assert "RATIONALE-AUTHORITY-CERTIFICATION-BLOCKED" in codes
    assert "RATIONALE-AUTHORITY-PROFESSIONAL-APPROVAL-BLOCKED" in codes
    assert result["rationale"]["status"] == "blocked_by_professional_boundary"


def test_enum_style_prohibited_claims_in_copied_context_are_blocked():
    envelope = operation_envelope()
    audit, preview = audit_and_preview(envelope)
    envelope["operation_set"]["operations"][0]["diagnostics"].append(
        {"message": "Copied operation diagnostic says CODE_COMPLIANT."}
    )
    preview["diagnostics"].append({"message": "Copied validation diagnostic says APPROVED."})
    audit["records"][0]["decision"]["rationale"] = (
        "Copied audit context says HUMAN_APPROVED_FOR_PROJECT and engineering acceptance."
    )

    result = record_agent_rationale(
        envelope,
        audit_trail=audit,
        validation_context=preview,
        source={"source_ref": "agent:invented", "source_channel": "test", "source_kind": "agent"},
        actor={"actor_type": "agent", "actor_ref": "agent:worker", "actor_role": "proposal_support"},
        rationale_text="Agent notes the proposal needs user review.",
        timestamp="2026-05-06T12:10:00Z",
        accepted_model_state=model_state(),
    )

    codes = {item["code"] for item in result["diagnostics"]}
    assert "RATIONALE-AUTHORITY-COMPLIANCE-BLOCKED" in codes
    assert "RATIONALE-AUTHORITY-PROFESSIONAL-APPROVAL-BLOCKED" in codes
    assert "RATIONALE-AUTHORITY-AUTONOMOUS-ACCEPTANCE-BLOCKED" in codes
    assert result["rationale"]["status"] == "blocked_by_professional_boundary"


def test_lowercase_approved_coordination_context_is_not_professional_approval():
    envelope = operation_envelope()
    audit, preview = audit_and_preview(envelope)
    preview["diagnostics"].append(
        {"message": "Copied validation context cites approved DAG coordination authority."}
    )

    result = record_agent_rationale(
        envelope,
        audit_trail=audit,
        validation_context=preview,
        source={"source_ref": "agent:invented", "source_channel": "test", "source_kind": "agent"},
        actor={"actor_type": "agent", "actor_ref": "agent:worker", "actor_role": "proposal_support"},
        rationale_text="Agent notes the proposal still needs user review.",
        timestamp="2026-05-06T12:10:00Z",
        accepted_model_state=model_state(),
    )

    assert "RATIONALE-AUTHORITY-PROFESSIONAL-APPROVAL-BLOCKED" not in {
        item["code"] for item in result["diagnostics"]
    }
    assert result["rationale"]["status"] == "captured_for_user_review"


def test_output_boundary_posture_does_not_make_prohibited_claims():
    result = rationale_record(operation_envelope())
    text = canonical_json(result).lower()

    for forbidden in [
        "code_compliant",
        "code compliant",
        "certified",
        "sealed",
        "authenticated",
        "human_approved_for_project",
        "professional approval",
        "engineering acceptance",
    ]:
        assert forbidden not in text
    assert result["professional_boundary"]["software_makes_approval_claim"] is False


def main():
    test_rationale_record_is_deterministic_and_preserves_context()
    test_rationale_cannot_mutate_accepted_state_or_bypass_user_acceptance()
    test_missing_context_and_unresolved_assumptions_are_visible()
    test_prohibited_professional_boundary_claims_are_blocked()
    test_prohibited_claims_in_copied_context_are_blocked()
    test_enum_style_prohibited_claims_in_copied_context_are_blocked()
    test_lowercase_approved_coordination_context_is_not_professional_approval()
    test_output_boundary_posture_does_not_make_prohibited_claims()


if __name__ == "__main__":
    main()
