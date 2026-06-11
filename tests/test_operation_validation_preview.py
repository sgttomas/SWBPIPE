#!/usr/bin/env python3
"""Focused tests for DEL-16-02 operation validation and diff preview."""

from __future__ import annotations

from copy import deepcopy
import json
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.constraints.validation.engine import (  # noqa: E402
    CANONICAL_DIMENSIONS as CONSTRAINT_CANONICAL_DIMENSIONS,
)
from core.gui.pkg02_boundary import (  # noqa: E402
    CANONICAL_DIMENSIONS as GUI_CANONICAL_DIMENSIONS,
)
from core.model_operations.validation_preview import (  # noqa: E402
    canonical_json,
    validate_and_preview_operations,
)
from core.model_operations.validation_preview.engine import (  # noqa: E402
    CANONICAL_DIMENSIONS,
)


OPERATION_FIXTURE = ROOT / "fixtures" / "model_operations" / "invented_operation_set_valid.json"
MODEL_STATE_FIXTURE = ROOT / "fixtures" / "model_operations" / "invented_accepted_model_state.json"
UNITS_SCHEMA = ROOT / "schemas" / "units.schema.yaml"


def ref(object_type, value):
    return {"object_type": object_type, "ref": value}


def model_state():
    with MODEL_STATE_FIXTURE.open(encoding="utf-8") as fixture_file:
        return json.load(fixture_file)


def operation_envelope(change):
    with OPERATION_FIXTURE.open(encoding="utf-8") as fixture_file:
        envelope = json.load(fixture_file)
    operation = next(
        item for item in envelope["operation_set"]["operations"] if item["operation_id"] == "op:resize"
    )
    operation["changes"] = [change]
    envelope["operation_set"]["operations"] = [operation]
    return envelope


def quantity_change():
    with OPERATION_FIXTURE.open(encoding="utf-8") as fixture_file:
        envelope = json.load(fixture_file)
    operation = next(
        item for item in envelope["operation_set"]["operations"] if item["operation_id"] == "op:resize"
    )
    return deepcopy(operation["changes"][0])


def test_valid_operation_generates_stable_preview_without_mutating_state():
    accepted = model_state()
    original = deepcopy(accepted)
    first = validate_and_preview_operations(operation_envelope(quantity_change()), accepted)
    second = validate_and_preview_operations(operation_envelope(quantity_change()), deepcopy(accepted))

    assert canonical_json(first) == canonical_json(second)
    assert accepted == original
    assert first["validation"]["schema_validation"] == "passed"
    assert first["validation"]["unit_validation"] == "passed"
    assert first["validation"]["diff_preview_status"] == "generated"
    assert first["validation"]["application_status"] == "not_applied"
    assert first["accepted_model_state_ref"]["hash"] == "sha256:invented-state-001"
    assert first["diff_preview"][0]["before"]["diameter"]["value"] == 100.0
    assert first["diff_preview"][0]["after"]["diameter"]["value"] == 125.0


def test_missing_unit_metadata_blocks_preview():
    change = quantity_change()
    change["value_payload"]["quantity_values"] = [{"value": 125.0}]
    result = validate_and_preview_operations(operation_envelope(change), model_state())

    codes = {item["code"] for item in result["diagnostics"]}
    assert "OP-UNIT-METADATA-MISSING" in codes
    assert result["validation"]["diff_preview_status"] == "blocked_by_validation"
    assert result["diff_preview"][0]["preview_status"] == "blocked_by_validation"


def test_unknown_dimension_blocks_unit_validation():
    change = quantity_change()
    change["value_payload"]["quantity_values"][0]["dimension"] = "temperature_difference"
    result = validate_and_preview_operations(operation_envelope(change), model_state())

    codes = {item["code"] for item in result["diagnostics"]}
    assert "OP-UNIT-DIMENSION-UNKNOWN" in codes
    assert result["validation"]["unit_validation"] == "blocked"
    assert result["validation"]["diff_preview_status"] == "blocked_by_validation"


def test_force_per_length_dimension_is_accepted_in_quantity_payloads():
    change = quantity_change()
    change["value_payload"]["quantity_values"][0].update(
        {"value": 250.0, "unit": "N/m", "dimension": "force_per_length"}
    )
    result = validate_and_preview_operations(operation_envelope(change), model_state())

    codes = {item["code"] for item in result["diagnostics"]}
    assert "OP-UNIT-DIMENSION-UNKNOWN" not in codes
    assert "OP-UNIT-METADATA-MISSING" not in codes
    assert result["validation"]["unit_validation"] == "passed"
    assert result["validation"]["diff_preview_status"] == "generated"


def test_canonical_dimensions_match_accepted_pkg02_vocabulary():
    with UNITS_SCHEMA.open(encoding="utf-8") as schema_file:
        accepted = json.load(schema_file)["$defs"]["DimensionId"]["enum"]

    assert CANONICAL_DIMENSIONS == set(accepted)
    assert CONSTRAINT_CANONICAL_DIMENSIONS == set(accepted)
    assert list(GUI_CANONICAL_DIMENSIONS) == accepted


def test_unresolved_target_and_constraint_findings_are_blocking():
    change = quantity_change()
    change["target_ref"] = ref("Component", "component:missing")
    result = validate_and_preview_operations(
        operation_envelope(change),
        model_state(),
        constraint_diagnostics=[{"code": "CV-BLOCK", "severity": "blocking"}],
    )

    codes = {item["code"] for item in result["diagnostics"]}
    assert "OP-TARGET-REF-UNRESOLVED" in codes
    assert "OP-CONSTRAINT-BLOCKING" in codes
    assert result["validation"]["constraint_validation"] == "blocked"


def test_direct_mutation_request_is_rejected():
    envelope = operation_envelope(quantity_change())
    operation = envelope["operation_set"]["operations"][0]
    operation["validation"]["application_status"] = "applied"
    result = validate_and_preview_operations(envelope, model_state())

    assert "OP-DIRECT-MUTATION-BLOCKED" in {item["code"] for item in result["diagnostics"]}
    assert result["validation"]["application_status"] == "not_applied"


def test_schema_invalid_envelope_cannot_report_schema_passed():
    envelope = operation_envelope(quantity_change())
    del envelope["operation_contract_status"]
    result = validate_and_preview_operations(envelope, model_state())

    assert "OP-SCHEMA-VALIDATION-FAILED" in {item["code"] for item in result["diagnostics"]}
    assert result["validation"]["schema_validation"] == "blocked"
    assert result["validation"]["diff_preview_status"] == "blocked_by_validation"


def test_model_role_and_current_hash_are_required_before_preview():
    accepted = model_state()
    accepted["model"]["model_role"] = "analytical_solver_model"
    envelope = operation_envelope(quantity_change())
    envelope["operation_set"]["operations"][0]["preconditions"]["required_current_hashes"][0]["value"] = "sha256:stale"
    result = validate_and_preview_operations(envelope, accepted)

    codes = {item["code"] for item in result["diagnostics"]}
    assert "OP-ACCEPTED-STATE-MODEL-ROLE-BLOCKED" in codes
    assert "OP-CURRENT-HASH-MISMATCH" in codes
    assert result["validation"]["schema_validation"] == "passed"
    assert result["validation"]["diff_preview_status"] == "blocked_by_validation"


def test_output_boundary_language_does_not_make_prohibited_claims():
    result = validate_and_preview_operations(operation_envelope(quantity_change()), model_state())
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
    test_valid_operation_generates_stable_preview_without_mutating_state()
    test_missing_unit_metadata_blocks_preview()
    test_unknown_dimension_blocks_unit_validation()
    test_force_per_length_dimension_is_accepted_in_quantity_payloads()
    test_canonical_dimensions_match_accepted_pkg02_vocabulary()
    test_unresolved_target_and_constraint_findings_are_blocking()
    test_direct_mutation_request_is_rejected()
    test_schema_invalid_envelope_cannot_report_schema_passed()
    test_model_role_and_current_hash_are_required_before_preview()
    test_output_boundary_language_does_not_make_prohibited_claims()


if __name__ == "__main__":
    main()
