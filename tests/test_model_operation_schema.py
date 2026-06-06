#!/usr/bin/env python3
"""Stdlib checks for the structured model operation schema."""

import json
from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT / "tests") not in sys.path:
    sys.path.insert(0, str(ROOT / "tests"))

from schema_validation import validate_instance, validate_schema_document  # noqa: E402

SCHEMA_PATH = ROOT / "schemas" / "model_operation.schema.json"
FIXTURE_PATH = ROOT / "fixtures" / "model_operations" / "invented_operation_set_valid.json"

REQUIRED_ROOT = {
    "schema_version",
    "deliverable_id",
    "package_id",
    "scope_item",
    "objectives",
    "operation_contract_status",
    "operation_set",
}

REQUIRED_DEFS = {
    "AssumptionRecord",
    "Checksum",
    "Diagnostic",
    "DiffPreviewRef",
    "Id",
    "ModelOperationRecord",
    "OperationAuthorType",
    "OperationChange",
    "OperationContractStatus",
    "OperationKind",
    "OperationModelBasis",
    "OperationPrecondition",
    "OperationSet",
    "OperationStatus",
    "OperationValidation",
    "OperationValuePayload",
    "PrivacyClassification",
    "ProfessionalBoundary",
    "Provenance",
    "Quantity",
    "RedistributionStatus",
    "Reference",
    "ReviewStatus",
    "UnitRequirements",
    "ValidationState",
}

REQUIRED_OPERATION_KINDS = {
    "add",
    "move",
    "modify",
    "delete",
    "reconnect",
    "constraint",
    "load",
    "support",
    "design_knowledge",
}

FORBIDDEN_STATUS = {
    "agent_accepted_engineering_state",
    "auto_approved",
    "code_compliant",
    "certified",
    "sealed",
}

FORBIDDEN_SCHEMA_TEXT = {
    "code compliant",
    "certified by software",
    "sealed by software",
    "professional approval by the software",
}

CANONICAL_DIMENSIONS = [
    "dimensionless",
    "length",
    "mass",
    "time",
    "temperature",
    "temperature_interval",
    "angle",
    "rotation",
    "force",
    "force_per_length",
    "moment",
    "pressure",
    "stress",
    "area",
    "volume",
    "density",
    "linear_stiffness",
    "rotational_stiffness",
    "displacement",
    "velocity",
    "acceleration",
    "thermal_conductivity",
    "specific_heat",
    "thermal_expansion_coefficient",
    "second_moment_area",
    "section_modulus",
    "mass_per_length",
    "volume_per_length",
    "slope",
    "TBD",
]


def load_schema():
    with SCHEMA_PATH.open(encoding="utf-8") as schema_file:
        return json.load(schema_file)


def load_fixture():
    with FIXTURE_PATH.open(encoding="utf-8") as fixture_file:
        return json.load(fixture_file)


def required_at(schema, definition_name):
    return set(schema["$defs"][definition_name]["required"])


def enum_at(schema, definition_name):
    return set(schema["$defs"][definition_name]["enum"])


def walk_keys(value):
    if isinstance(value, dict):
        for key, item in value.items():
            yield key
            yield from walk_keys(item)
    elif isinstance(value, list):
        for item in value:
            yield from walk_keys(item)


def walk_strings(value):
    if isinstance(value, str):
        yield value
    elif isinstance(value, dict):
        for item in value.values():
            yield from walk_strings(item)
    elif isinstance(value, list):
        for item in value:
            yield from walk_strings(item)


def main():
    schema = load_schema()
    defs = schema["$defs"]
    validate_schema_document(schema, schema_label="model_operation.schema.json")

    assert schema["$schema"] == "https://json-schema.org/draft/2020-12/schema"
    assert schema["additionalProperties"] is False
    assert "default" not in set(walk_keys(schema))
    assert REQUIRED_ROOT <= set(schema["required"])
    assert REQUIRED_DEFS <= set(defs)

    assert schema["properties"]["deliverable_id"]["const"] == "DEL-16-01"
    assert schema["properties"]["package_id"]["const"] == "PKG-16"
    assert schema["properties"]["scope_item"]["const"] == "SOW-069"
    assert schema["properties"]["objectives"]["contains"]["const"] == "OBJ-015"

    contract = defs["OperationContractStatus"]["properties"]
    assert contract["record_contract"]["const"] == "schema_first_model_operation_records"
    assert contract["mutation_route"]["const"] == "structured_operations_only"
    assert contract["direct_model_mutation_allowed"]["const"] is False
    assert contract["user_acceptance_boundary"]["const"] == (
        "downstream_user_acceptance_required"
    )
    assert contract["diff_preview_binding"]["const"] == "downstream_DEL-16-02"
    assert contract["audit_trail_binding"]["const"] == "downstream_DEL-16-03"

    assert REQUIRED_OPERATION_KINDS <= enum_at(schema, "OperationKind")
    assert {"user", "agent", "import_adapter", "project_template"} <= enum_at(
        schema, "OperationAuthorType"
    )
    operation_status = enum_at(schema, "OperationStatus")
    assert {
        "proposed",
        "schema_validated",
        "blocked_by_diagnostics",
        "ready_for_user_review",
        "rejected",
    } <= operation_status
    assert operation_status.isdisjoint(FORBIDDEN_STATUS)

    set_required = required_at(schema, "OperationSet")
    assert {
        "operation_set_id",
        "project_ref",
        "model_ref",
        "model_basis",
        "operations",
        "diagnostics",
        "provenance",
        "professional_boundary",
    } <= set_required
    model_basis_required = required_at(schema, "OperationModelBasis")
    assert {
        "model_ref",
        "canonical_model_role",
        "physical_source_of_truth_ref",
        "accepted_model_state_ref",
        "accepted_model_state_hash",
    } <= model_basis_required
    assert defs["OperationModelBasis"]["properties"]["canonical_model_role"]["const"] == "physical_source_of_truth"

    operation_required = required_at(schema, "ModelOperationRecord")
    assert {
        "operation_id",
        "operation_kind",
        "operation_status",
        "author_type",
        "target_refs",
        "preconditions",
        "changes",
        "validation",
        "diagnostics",
        "diff_preview_refs",
        "assumptions",
        "provenance",
        "professional_boundary",
    } <= operation_required

    precondition_required = required_at(schema, "OperationPrecondition")
    assert {
        "base_model_state_ref",
        "required_current_hashes",
        "required_refs",
        "assumptions",
    } <= precondition_required

    change_required = required_at(schema, "OperationChange")
    assert {
        "change_id",
        "change_kind",
        "target_object_type",
        "target_ref",
        "value_payload",
        "unit_requirements",
        "provenance",
    } <= change_required
    assert {
        "add_object",
        "remove_object",
        "set_field",
        "move_geometry",
        "reconnect",
        "update_constraint",
        "update_load",
        "update_support",
        "attach_design_knowledge",
    } <= set(defs["OperationChange"]["properties"]["change_kind"]["enum"])

    validation_required = required_at(schema, "OperationValidation")
    assert {
        "schema_validation",
        "constraint_validation",
        "unit_validation",
        "diff_preview_status",
        "application_status",
    } <= validation_required
    assert {
        "not_applied",
        "held_for_user_review",
        "downstream_application_required",
    } <= set(defs["OperationValidation"]["properties"]["application_status"]["enum"])

    units = defs["UnitRequirements"]["properties"]
    assert units["unit_metadata_required"]["const"] is True
    assert units["dimension_check_required"]["const"] is True
    assert units["missing_unit_behavior"]["const"] == "emit_diagnostic"
    assert defs["Quantity"]["properties"]["dimension"]["enum"] == CANONICAL_DIMENSIONS

    professional = defs["ProfessionalBoundary"]["properties"]
    assert professional["human_review_required"]["const"] is True
    assert professional["software_makes_compliance_claim"]["const"] is False
    assert professional["software_makes_certification_claim"]["const"] is False
    assert professional["software_makes_sealing_claim"]["const"] is False
    assert professional["software_makes_approval_claim"]["const"] is False
    assert professional["software_makes_authentication_claim"]["const"] is False

    joined_strings = "\n".join(walk_strings(schema)).lower()
    for forbidden in FORBIDDEN_SCHEMA_TEXT:
        assert forbidden.lower() not in joined_strings

    fixture = load_fixture()
    validate_instance(
        schema,
        fixture,
        schema_label="model_operation.schema.json",
        instance_label="invented_operation_set_valid.json",
    )
    operations = fixture["operation_set"]["operations"]
    assert {item["operation_kind"] for item in operations} >= REQUIRED_OPERATION_KINDS
    assert {
        change["change_kind"]
        for operation in operations
        for change in operation["changes"]
    } >= {
        "add_object",
        "remove_object",
        "set_field",
        "move_geometry",
        "reconnect",
        "update_constraint",
        "update_load",
        "update_support",
        "attach_design_knowledge",
    }
    assert fixture["operation_set"]["model_basis"]["canonical_model_role"] == "physical_source_of_truth"
    for operation in operations:
        assert any(
            item["payload_scope"] == "model_state_record"
            for item in operation["preconditions"]["required_current_hashes"]
        )


if __name__ == "__main__":
    main()
