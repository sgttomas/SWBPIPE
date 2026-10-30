#!/usr/bin/env python3
"""Focused tests for DEL-13-03 constraint validation."""

import sys
from copy import deepcopy
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.constraints.validation import diagnostic_dicts, validate_constraint_envelope


FORBIDDEN_CLAIMS = {
    "certification",
    "certified",
    "sealing",
    "sealed",
    "authentication",
    "code " + "compliant",
    "professional approval",
    "engineering acceptance",
}


def provenance(source_name="invented fixture"):
    return {
        "source_name": source_name,
        "source_location": f"fixtures/{source_name.replace(' ', '_')}",
        "source_license": "invented test fixture",
        "contributor": "DEL-13-03 test",
        "contributor_certification": "invented non-engineering validation fixture",
        "redistribution_status": "invented_non_engineering_example",
        "review_status": "pending",
        "privacy_classification": "invented_public_example",
    }


def professional_boundary():
    return {
        "human_review_required": True,
        "software_makes_compliance_claim": False,
        "software_makes_certification_claim": False,
        "software_makes_sealing_claim": False,
        "software_makes_approval_claim": False,
        "software_makes_authentication_claim": False,
    }


def ref(object_type, value):
    return {"object_type": object_type, "ref": value}


def quantity(value=1.0, unit="m", dimension="length"):
    return {
        "value": value,
        "unit": unit,
        "dimension": dimension,
        "provenance": provenance("quantity source"),
    }


def parameter(parameter_id="param:length", value=None):
    return {
        "parameter_id": parameter_id,
        "name": parameter_id,
        "value": quantity() if value is None else value,
        "value_kind": "quantity",
        "provenance": provenance("parameter source"),
    }


def design_record(record_id, record_kind="requirement", requirement_type="clearance"):
    if record_kind == "zone":
        return {
            "id": record_id,
            "record_kind": "zone",
            "name": record_id,
            "zone_type": "no_go_volume",
            "geometry": {
                "geometry_kind": "abstract_reference",
                "coordinate_refs": [],
                "boundary_refs": [],
                "quantities": [quantity()],
            },
            "source_notes": [],
            "assumptions": [],
            "provenance": provenance(f"{record_id} source"),
        }
    return {
        "id": record_id,
        "record_kind": "requirement",
        "name": record_id,
        "requirement_type": requirement_type,
        "target_refs": [ref("Component", "C-1")],
        "requirement_statement": "Invented fixture requirement.",
        "parameters": [parameter()],
        "source_notes": [],
        "assumptions": [],
        "provenance": provenance(f"{record_id} source"),
    }


def design_knowledge():
    records = [
        design_record("DK-clearance", requirement_type="clearance"),
        design_record("DK-slope", requirement_type="slope"),
        design_record("DK-access", requirement_type="access"),
        design_record("DK-zone", record_kind="zone"),
        {
            "id": "DK-equipment",
            "record_kind": "equipment_interface",
            "name": "equipment interface",
            "equipment_ref": ref("Component", "EQ-1"),
            "interface_role": "terminal_point",
            "location": {"x": quantity(), "y": quantity(), "z": quantity()},
            "interface_parameters": [parameter()],
            "source_notes": [],
            "assumptions": [],
            "provenance": provenance("equipment source"),
        },
    ]
    return {
        "schema_version": "0.1.0",
        "deliverable_id": "DEL-13-01",
        "package_id": "PKG-13",
        "scope_item": "SOW-067",
        "objectives": ["OBJ-014"],
        "data_boundary": {
            "public_examples_policy": "invented_or_cleared_data_only",
            "protected_source_policy": "no_bundled_protected_owner_or_standards_data",
            "private_data_policy": "user_controlled_private_paths",
            "unit_policy": "unit_bearing_values_require_explicit_unit_metadata",
            "professional_boundary": professional_boundary(),
        },
        "design_knowledge": {
            "knowledge_set_id": "DKS-1",
            "project_ref": ref("Project", "P-1"),
            "model_ref": ref("Model", "M-1"),
            "records": records,
            "diagnostics": [],
            "provenance": provenance("knowledge source"),
        },
    }


def constraint_record(kind, design_refs=None, parameters=None, targets=None, status="schema_validated"):
    return {
        "constraint_id": f"C-{kind}",
        "constraint_kind": kind,
        "name": f"{kind} constraint",
        "state": "active",
        "source_type": "user",
        "target_refs": targets if targets is not None else [ref("Component", f"target-{kind}")],
        "design_knowledge_refs": design_refs if design_refs is not None else [],
        "parameters": parameters if parameters is not None else [],
        "diagnostics": [],
        "assumptions": [],
        "validation_status": status,
        "provenance": provenance(f"{kind} source"),
        "professional_boundary": professional_boundary(),
    }


def constraint_envelope():
    constraints = [
        constraint_record(
            "connectivity",
            targets=[ref("Node", "N-1"), ref("Node", "N-2")],
            status="conflict_detected",
        ),
        constraint_record("clearance", design_refs=[ref("DesignKnowledgeRecord", "DK-clearance")], parameters=[parameter()]),
        constraint_record("no_go_volume", design_refs=[ref("DesignKnowledgeRecord", "DK-zone")]),
        constraint_record("support_zone", design_refs=[ref("DesignKnowledgeRecord", "DK-zone")]),
        constraint_record("route_conflict", design_refs=[ref("DesignKnowledgeRecord", "DK-zone")]),
        constraint_record("slope", design_refs=[ref("DesignKnowledgeRecord", "DK-slope")], parameters=[parameter()]),
        constraint_record("drain", design_refs=[ref("DesignKnowledgeRecord", "DK-slope")], parameters=[parameter()]),
        constraint_record("vent", design_refs=[ref("DesignKnowledgeRecord", "DK-slope")], parameters=[parameter()]),
        constraint_record("access", design_refs=[ref("DesignKnowledgeRecord", "DK-access")], parameters=[parameter()]),
        constraint_record("equipment_interface", design_refs=[ref("DesignKnowledgeRecord", "DK-equipment")]),
        constraint_record("missing_required_data", status="missing_data"),
    ]
    return {
        "schema_version": "0.1.0",
        "deliverable_id": "DEL-13-02",
        "package_id": "PKG-13",
        "scope_items": ["SOW-067", "SOW-068"],
        "objectives": ["OBJ-014", "OBJ-018"],
        "data_boundary": {
            "public_examples_policy": "invented_or_cleared_data_only",
            "protected_source_policy": "no_bundled_protected_owner_or_standards_data",
            "private_data_policy": "user_controlled_private_paths",
            "unit_policy": "unit_bearing_values_require_explicit_unit_metadata",
            "engineering_authority": "human_review_required_outside_software",
        },
        "constraint_set": {
            "constraint_set_id": "CS-1",
            "project_ref": ref("Project", "P-1"),
            "model_ref": ref("Model", "M-1"),
            "design_knowledge_refs": [ref("DesignKnowledgeRecord", "DK-clearance")],
            "constraints": constraints,
            "diagnostics": [],
            "provenance": provenance("constraint set source"),
            "professional_boundary": professional_boundary(),
        },
    }


def codes(result):
    return {item["code"] for item in diagnostic_dicts(result)}


def classes(result):
    return {item["class"] for item in diagnostic_dicts(result)}


def test_validation_is_deterministic_and_covers_represented_classes():
    constraints = constraint_envelope()
    knowledge = design_knowledge()

    first = diagnostic_dicts(validate_constraint_envelope(constraints, knowledge))
    second = diagnostic_dicts(validate_constraint_envelope(deepcopy(constraints), deepcopy(knowledge)))

    assert first == second
    assert {
        "CONNECTIVITY_CONFLICT",
        "CLEARANCE_CONFLICT",
        "ROUTE_CONFLICT",
        "SUPPORT_ZONE_CONFLICT",
        "SLOPE_DRAIN_VENT_CONFLICT",
        "CONSTRAINT_MISSING_DATA",
        "SCHEMA_VALIDATION",
    } <= {item["class"] for item in first}
    assert any(item["code"] == "CV-EQUIPMENT-INTERFACE-AVAILABLE" for item in first)
    assert any(item["code"] == "CV-ACCESS-AVAILABLE" for item in first)


def test_missing_data_and_unresolved_references_are_explicit_findings():
    constraints = constraint_envelope()
    broken = constraint_record(
        "clearance",
        design_refs=[ref("DesignKnowledgeRecord", "DK-missing")],
        parameters=[],
        targets=[],
    )
    broken.pop("provenance")
    constraints["constraint_set"]["constraints"] = [broken]

    result = validate_constraint_envelope(constraints, design_knowledge())

    assert {
        "CV-CONSTRAINT-MISSING-FIELD",
        "CV-CLEARANCE-TARGETS-MISSING",
        "CV-CLEARANCE-PARAMETERS-MISSING",
        "CV-DESIGN-REF-UNRESOLVED",
        "CV-CONSTRAINT-PROVENANCE-MISSING",
    } <= codes(result)
    assert result.has_blocking_findings


def test_unit_metadata_is_checked_without_conversion_or_tolerance():
    constraints = constraint_envelope()
    bad_quantity = {"value": 25.0, "unit": "", "dimension": "TBD", "provenance": provenance("bad quantity")}
    constraints["constraint_set"]["constraints"] = [
        constraint_record("slope", design_refs=[ref("DesignKnowledgeRecord", "DK-slope")], parameters=[parameter(value=bad_quantity)])
    ]

    result = validate_constraint_envelope(constraints, design_knowledge())
    unit_diagnostics = [item for item in diagnostic_dicts(result) if item["class"] == "UNIT_WARNING"]

    assert any(item["code"] == "CV-UNIT-METADATA-MISSING" for item in unit_diagnostics)
    joined = "\n".join(item["message"] + "\n" + item["remediation"] for item in unit_diagnostics).lower()
    assert "does not convert units" in joined
    assert "invent tolerances" in joined


def test_quantity_dimensions_are_checked_against_pkg02_vocabulary():
    constraints = constraint_envelope()
    accepted = constraint_record(
        "slope",
        design_refs=[ref("DesignKnowledgeRecord", "DK-slope")],
        parameters=[parameter(value=quantity(0.01, "m/m", "slope"))],
    )
    retired = constraint_record(
        "access",
        design_refs=[ref("DesignKnowledgeRecord", "DK-access")],
        parameters=[parameter(value=quantity(1.0, "N/m", "stiffness"))],
    )
    constraints["constraint_set"]["constraints"] = [accepted, retired]

    result = validate_constraint_envelope(constraints, design_knowledge())
    dimension_diagnostics = [
        item
        for item in diagnostic_dicts(result)
        if item["code"] == "CV-UNIT-DIMENSION-UNKNOWN"
    ]

    assert len(dimension_diagnostics) == 1
    assert "stiffness" in dimension_diagnostics[0]["message"]
    assert "slope" not in "\n".join(item["message"] for item in dimension_diagnostics)
    assert result.has_blocking_findings


def test_protected_or_private_provenance_is_preserved_as_boundary_diagnostic():
    constraints = constraint_envelope()
    constraints["constraint_set"]["constraints"][0]["provenance"]["redistribution_status"] = "protected_suspected"
    constraints["constraint_set"]["constraints"][0]["provenance"]["privacy_classification"] = "private_project_data"

    result = validate_constraint_envelope(constraints, design_knowledge())

    assert "IP_BOUNDARY_WARNING" in classes(result)
    protected = [
        item
        for item in diagnostic_dicts(result)
        if item["code"] in {"CV-CONSTRAINT-PROVENANCE-REDISTRIBUTION-REVIEW", "CV-CONSTRAINT-PROVENANCE-PRIVACY-REVIEW"}
    ]
    assert protected
    assert all(item["source_references"] for item in protected)


def test_output_text_does_not_make_prohibited_authority_claims():
    result = validate_constraint_envelope(constraint_envelope(), design_knowledge())
    text = "\n".join(
        f"{item['code']} {item['message']} {item['remediation']}"
        for item in diagnostic_dicts(result)
    ).lower()

    for forbidden in FORBIDDEN_CLAIMS:
        assert forbidden not in text


if __name__ == "__main__":
    test_validation_is_deterministic_and_covers_represented_classes()
    test_missing_data_and_unresolved_references_are_explicit_findings()
    test_unit_metadata_is_checked_without_conversion_or_tolerance()
    test_quantity_dimensions_are_checked_against_pkg02_vocabulary()
    test_protected_or_private_provenance_is_preserved_as_boundary_diagnostic()
    test_output_text_does_not_make_prohibited_authority_claims()
