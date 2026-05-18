#!/usr/bin/env python3
"""Stdlib checks for the constraint entity schema."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "schemas" / "constraint.schema.json"

REQUIRED_ROOT = {
    "schema_version",
    "deliverable_id",
    "package_id",
    "scope_items",
    "objectives",
    "data_boundary",
    "constraint_set",
}

REQUIRED_DEFS = {
    "AssumptionRecord",
    "ConstraintKind",
    "ConstraintRecord",
    "ConstraintSet",
    "DataBoundary",
    "Diagnostic",
    "Id",
    "Parameter",
    "PrivacyClassification",
    "ProfessionalBoundary",
    "Provenance",
    "Quantity",
    "RedistributionStatus",
    "Reference",
    "ReviewStatus",
    "SourceType",
    "ValidationStatus",
}

REQUIRED_CONSTRAINT_KINDS = {
    "connectivity",
    "clearance",
    "no_go_volume",
    "support_zone",
    "route_conflict",
    "slope",
    "drain",
    "vent",
    "access",
    "equipment_interface",
    "missing_required_data",
}

REQUIRED_DIAGNOSTIC_CLASSES = {
    "CONSTRAINT_MISSING_DATA",
    "CONNECTIVITY_CONFLICT",
    "CLEARANCE_CONFLICT",
    "ROUTE_CONFLICT",
    "SUPPORT_ZONE_CONFLICT",
    "SLOPE_DRAIN_VENT_CONFLICT",
    "PROVENANCE_WARNING",
    "UNIT_WARNING",
    "IP_BOUNDARY_WARNING",
}

FORBIDDEN_SCHEMA_TEXT = {
    "allowable stress table",
    "stress intensification factor table",
    "B31J",
    "real secret",
    "code compliant",
    "certified by software",
    "sealed by software",
    "professional approval by the software",
}

ACCEPTED_PKG02_DIMENSIONS = {
    "dimensionless",
    "length",
    "mass",
    "time",
    "temperature",
    "temperature_interval",
    "angle",
    "rotation",
    "force",
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
}


def load_schema():
    with SCHEMA_PATH.open(encoding="utf-8") as schema_file:
        return json.load(schema_file)


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

    assert schema["$schema"] == "https://json-schema.org/draft/2020-12/schema"
    assert schema["additionalProperties"] is False
    assert "default" not in set(walk_keys(schema))
    assert REQUIRED_ROOT <= set(schema["required"])
    assert REQUIRED_DEFS <= set(defs)

    assert schema["properties"]["deliverable_id"]["const"] == "DEL-13-02"
    assert schema["properties"]["package_id"]["const"] == "PKG-13"
    assert {"SOW-067", "SOW-068"} <= set(
        schema["properties"]["scope_items"]["items"]["enum"]
    )
    assert {"OBJ-014", "OBJ-018"} <= set(
        schema["properties"]["objectives"]["items"]["enum"]
    )

    boundary = defs["DataBoundary"]["properties"]
    assert boundary["public_examples_policy"]["const"] == "invented_or_cleared_data_only"
    assert (
        boundary["protected_source_policy"]["const"]
        == "no_bundled_protected_owner_or_standards_data"
    )
    assert (
        boundary["unit_policy"]["const"]
        == "unit_bearing_values_require_explicit_unit_metadata"
    )
    assert boundary["engineering_authority"]["const"] == (
        "human_review_required_outside_software"
    )

    assert REQUIRED_CONSTRAINT_KINDS <= enum_at(schema, "ConstraintKind")
    assert {"user", "project", "import", "agent", "source_derived"} <= enum_at(
        schema, "SourceType"
    )
    assert {
        "unvalidated",
        "schema_validated",
        "constraint_validated",
        "conflict_detected",
        "missing_data",
        "blocked_by_missing_data",
    } <= enum_at(schema, "ValidationStatus")

    set_required = required_at(schema, "ConstraintSet")
    assert {
        "constraint_set_id",
        "project_ref",
        "model_ref",
        "design_knowledge_refs",
        "constraints",
        "diagnostics",
        "provenance",
        "professional_boundary",
    } <= set_required
    assert (
        defs["ConstraintSet"]["properties"]["constraints"]["items"]["$ref"]
        == "#/$defs/ConstraintRecord"
    )

    record_required = required_at(schema, "ConstraintRecord")
    assert {
        "constraint_id",
        "constraint_kind",
        "state",
        "source_type",
        "target_refs",
        "design_knowledge_refs",
        "parameters",
        "diagnostics",
        "assumptions",
        "validation_status",
        "provenance",
        "professional_boundary",
    } <= record_required

    diagnostic_required = required_at(schema, "Diagnostic")
    assert {
        "code",
        "class",
        "severity",
        "source",
        "affected_object",
        "message",
        "remediation",
        "provenance",
    } <= diagnostic_required
    assert REQUIRED_DIAGNOSTIC_CLASSES <= set(
        defs["Diagnostic"]["properties"]["class"]["enum"]
    )

    assert {"value", "unit", "dimension", "provenance"} <= required_at(
        schema, "Quantity"
    )
    dimensions = set(defs["Quantity"]["properties"]["dimension"]["enum"])
    assert dimensions == ACCEPTED_PKG02_DIMENSIONS
    assert "temperature_difference" not in dimensions
    assert "area_moment" not in dimensions
    assert "stiffness" not in dimensions

    provenance_required = required_at(schema, "Provenance")
    assert {
        "source_name",
        "source_location",
        "source_license",
        "contributor",
        "contributor_certification",
        "redistribution_status",
        "review_status",
        "privacy_classification",
    } <= provenance_required
    assert {
        "public_permissive",
        "private_only",
        "unknown",
        "protected_suspected",
        "invented_non_engineering_example",
    } <= enum_at(schema, "RedistributionStatus")

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


if __name__ == "__main__":
    main()
