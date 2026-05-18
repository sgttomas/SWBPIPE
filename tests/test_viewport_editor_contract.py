#!/usr/bin/env python3
"""Stdlib checks for the viewport editor schema and invented fixture."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "schemas" / "viewport_editor.schema.yaml"
FIXTURE_PATH = ROOT / "fixtures" / "gui" / "invented" / "viewport_editor_session.json"

REQUIRED_ROOT = {
    "schema_version",
    "deliverable_id",
    "package_id",
    "scope_item",
    "objectives",
    "viewport_status",
    "viewport_session",
}

REQUIRED_DEFS = {
    "CameraState",
    "CommandIntent",
    "CommandType",
    "DiagnosticClass",
    "DiagnosticSeverity",
    "InteractionState",
    "Point3",
    "PrivacyClassification",
    "ProfessionalBoundary",
    "Provenance",
    "RedistributionStatus",
    "Reference",
    "ReviewStatus",
    "Vector3",
    "ViewGeometry",
    "ViewPrimitive",
    "ViewPrimitiveType",
    "ViewportDiagnostic",
    "ViewportDiagnosticCode",
    "ViewportSession",
    "ViewportStatus",
    "ViewportTool",
}

REQUIRED_PRIMITIVES = {
    "node",
    "pipe_run",
    "bend_arc",
    "branch_symbol",
    "valve_symbol",
    "flange_symbol",
    "reducer_symbol",
    "expansion_joint_symbol",
    "support_symbol",
}

REQUIRED_COMMANDS = {
    "create_node",
    "connect_pipe_run",
    "insert_bend",
    "insert_branch_symbol",
    "insert_component_symbol",
    "select_entities",
    "clear_selection",
}

REQUIRED_DIAGNOSTIC_CLASSES = {
    "SOLVE_BLOCKING",
    "RULE_CHECK_BLOCKING",
    "PROVENANCE_WARNING",
    "ASSUMPTION_WARNING",
    "NONLINEAR_WARNING",
    "IP_BOUNDARY_WARNING",
    "UNIT_WARNING",
    "GUI_STATE_WARNING",
}

FORBIDDEN_FIXTURE_TERMS = {
    "ASME",
    "B31",
    "B31J",
    "allowable stress table",
    "stress intensification factor table",
    "vendor catalog value",
    "real secret",
    "code compliant",
    "certified",
    "sealed",
}


def load_json(path):
    with path.open(encoding="utf-8") as json_file:
        return json.load(json_file)


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


def main():
    schema = load_json(SCHEMA_PATH)
    fixture = load_json(FIXTURE_PATH)
    defs = schema["$defs"]

    assert schema["$schema"] == "https://json-schema.org/draft/2020-12/schema"
    assert schema["additionalProperties"] is False
    assert "default" not in set(walk_keys(schema))
    assert REQUIRED_ROOT <= set(schema["required"])
    assert REQUIRED_DEFS <= set(defs)

    assert schema["properties"]["deliverable_id"]["const"] == "DEL-07-01"
    assert schema["properties"]["package_id"]["const"] == "PKG-07"
    assert schema["properties"]["scope_item"]["const"] == "SOW-020"
    assert schema["properties"]["objectives"]["contains"]["const"] == "OBJ-006"

    status = defs["ViewportStatus"]["properties"]
    assert status["frontend_app_shell"]["const"] == "TBD"
    assert status["viewport_renderer"]["const"] == "Three_js_runtime_integration_TBD"
    assert status["durable_state_mutation"]["const"] == (
        "application_service_command_intents_only"
    )
    assert status["transient_state_policy"]["const"] == (
        "camera_hover_selection_drag_snap_are_not_persisted_project_payload"
    )
    assert status["command_transport"]["const"] == "TBD"
    assert status["dependency_versions"]["const"] == "TBD"

    boundary_required = required_at(schema, "ProfessionalBoundary")
    assert {
        "human_review_required",
        "software_makes_compliance_claim",
        "software_makes_certification_claim",
        "software_makes_sealing_claim",
        "software_makes_approval_claim",
        "software_makes_authentication_claim",
    } <= boundary_required
    boundary = defs["ProfessionalBoundary"]["properties"]
    assert boundary["human_review_required"]["const"] is True
    for field in boundary_required - {"human_review_required"}:
        assert boundary[field]["const"] is False

    assert REQUIRED_PRIMITIVES <= enum_at(schema, "ViewPrimitiveType")
    assert REQUIRED_COMMANDS <= enum_at(schema, "CommandType")
    assert REQUIRED_DIAGNOSTIC_CLASSES <= enum_at(schema, "DiagnosticClass")
    assert {
        "VIEWPORT_UNIT_MISMATCH",
        "VIEWPORT_COMMAND_REQUIRES_SERVICE_VALIDATION",
        "VIEWPORT_COMPONENT_DATA_MISSING",
        "VIEWPORT_PROTECTED_CONTENT_SUSPECTED",
        "VIEWPORT_TRANSIENT_STATE_NOT_PERSISTED",
    } <= enum_at(schema, "ViewportDiagnosticCode")

    command_required = required_at(schema, "CommandIntent")
    assert {
        "intent_id",
        "command_type",
        "target_ref",
        "payload_refs",
        "unit_policy",
        "reversible",
        "validation_state",
        "diagnostic_refs",
        "provenance",
    } <= command_required
    assert (
        defs["CommandIntent"]["properties"]["unit_policy"]["const"]
        == "unit_aware_domain_validation_required"
    )

    assert fixture["deliverable_id"] == "DEL-07-01"
    assert fixture["viewport_status"]["frontend_app_shell"] == "TBD"
    assert (
        fixture["viewport_status"]["durable_state_mutation"]
        == "application_service_command_intents_only"
    )
    assert fixture["viewport_status"]["professional_boundary"][
        "software_makes_compliance_claim"
    ] is False
    assert fixture["viewport_session"]["interaction_state"]["active_tool"] == (
        "connect_pipe_run"
    )
    assert fixture["viewport_session"]["view_primitives"][0]["primitive_type"] == (
        "node"
    )
    assert fixture["viewport_session"]["view_primitives"][1]["primitive_type"] == (
        "pipe_run"
    )
    assert fixture["viewport_session"]["command_intents"][0]["validation_state"] == (
        "pending_service_validation"
    )
    assert fixture["viewport_session"]["command_intents"][0]["unit_policy"] == (
        "unit_aware_domain_validation_required"
    )
    assert fixture["viewport_session"]["diagnostics"][0]["class"] == "GUI_STATE_WARNING"
    assert fixture["viewport_session"]["provenance"]["redistribution_status"] == (
        "invented_non_engineering_example"
    )
    assert fixture["viewport_session"]["provenance"]["privacy_classification"] == (
        "invented_public_example"
    )

    fixture_text = FIXTURE_PATH.read_text(encoding="utf-8")
    for forbidden in FORBIDDEN_FIXTURE_TERMS:
        assert forbidden.lower() not in fixture_text.lower()


if __name__ == "__main__":
    main()
