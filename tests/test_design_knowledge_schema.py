#!/usr/bin/env python3
"""Stdlib checks for the design knowledge schema."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "schemas" / "design_knowledge.schema.json"

REQUIRED_ROOT = {
    "schema_version",
    "deliverable_id",
    "package_id",
    "scope_item",
    "objectives",
    "data_boundary",
    "design_knowledge",
}

REQUIRED_DEFS = {
    "AssumptionRecord",
    "Coordinate3",
    "DataBoundary",
    "DesignKnowledgeEnvelope",
    "Diagnostic",
    "EndpointRecord",
    "EquipmentInterfaceRecord",
    "GeometryPayload",
    "Id",
    "LineDataRecord",
    "MetadataRecord",
    "Parameter",
    "PrivacyClassification",
    "ProfessionalBoundary",
    "Provenance",
    "Quantity",
    "RedistributionStatus",
    "Reference",
    "RequirementRecord",
    "ReviewStatus",
    "RoutingCorridorRecord",
    "SourceNote",
    "ZoneRecord",
}

RECORD_DEFS = {
    "EndpointRecord": "endpoint",
    "LineDataRecord": "line_data",
    "RoutingCorridorRecord": "routing_corridor",
    "ZoneRecord": "zone",
    "EquipmentInterfaceRecord": "equipment_interface",
    "RequirementRecord": "requirement",
    "MetadataRecord": "owner_project_metadata",
}

REQUIRED_REQUIREMENT_TYPES = {
    "access",
    "slope",
    "drain",
    "vent",
    "clearance",
    "support",
    "owner_project_metadata",
}

FORBIDDEN_SCHEMA_TEXT = {
    "allowable stress table",
    "stress intensification factor table",
    "B31J",
    "real secret",
    "code compliant",
    "certified",
    "sealed",
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

    assert schema["properties"]["deliverable_id"]["const"] == "DEL-13-01"
    assert schema["properties"]["package_id"]["const"] == "PKG-13"
    assert schema["properties"]["scope_item"]["const"] == "SOW-067"
    assert schema["properties"]["objectives"]["contains"]["const"] == "OBJ-014"

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

    envelope_required = required_at(schema, "DesignKnowledgeEnvelope")
    assert {
        "knowledge_set_id",
        "project_ref",
        "model_ref",
        "records",
        "diagnostics",
        "provenance",
    } <= envelope_required

    for definition_name, record_kind in RECORD_DEFS.items():
        required = required_at(schema, definition_name)
        assert {"id", "record_kind", "name", "source_notes", "assumptions", "provenance"} <= required
        assert defs[definition_name]["properties"]["record_kind"]["const"] == record_kind

    assert {
        "start",
        "end",
        "tie_in",
        "equipment_interface",
        "boundary_condition",
    } <= set(defs["EndpointRecord"]["properties"]["endpoint_role"]["enum"])

    assert {
        "no_go_volume",
        "supportable_zone",
        "access_zone",
        "maintenance_zone",
        "operating_zone",
    } <= set(defs["ZoneRecord"]["properties"]["zone_type"]["enum"])
    assert REQUIRED_REQUIREMENT_TYPES <= set(
        defs["RequirementRecord"]["properties"]["requirement_type"]["enum"]
    )

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
    assert {
        "public_metadata",
        "invented_public_example",
        "private_project_data",
        "owner_project_metadata_private",
        "protected_suspected",
        "redacted",
    } <= enum_at(schema, "PrivacyClassification")

    assert {"value", "unit", "dimension", "provenance"} <= required_at(
        schema, "Quantity"
    )
    dimensions = set(defs["Quantity"]["properties"]["dimension"]["enum"])
    assert dimensions == ACCEPTED_PKG02_DIMENSIONS
    assert "temperature_difference" not in dimensions
    assert "area_moment" not in dimensions
    assert "stiffness" not in dimensions

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
