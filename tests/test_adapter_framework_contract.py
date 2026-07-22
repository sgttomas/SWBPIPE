#!/usr/bin/env python3
"""Stdlib checks for the adapter framework contract."""

import copy
import json
from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))

SCHEMA_PATH = ROOT / "schemas" / "adapter_framework.schema.yaml"
FIXTURE_PATH = ROOT / "fixtures" / "adapters" / "invented" / "invented_adapter_framework.json"

from core.adapters.framework import (  # noqa: E402
    build_result,
    gate_adapter_runtime_dispatch,
    validate_adapter_declaration,
)


REQUIRED_ROOT = {
    "schema_version",
    "deliverable_id",
    "package_id",
    "scope_item",
    "objective",
    "framework_status",
    "tbd_decisions",
    "adapter_declaration",
    "validation_plan",
    "operation_result",
}

REQUIRED_DEFS = {
    "AdapterCapability",
    "AdapterDeclaration",
    "AdapterOperationResult",
    "ChecksumRef",
    "Diagnostic",
    "FrameworkStatus",
    "NoBypassControls",
    "OperationBoundary",
    "PrivacyContext",
    "ProfessionalBoundary",
    "Provenance",
    "Reference",
    "ResultEnvelopeRef",
    "TbdDecisions",
    "ValidationPlan",
}

REQUIRED_TBD = {
    "external_format_list",
    "public_transport_protocol",
    "endpoint_syntax",
    "adapter_execution_model",
    "plugin_runtime",
    "permission_grant_persistence",
    "package_scripts",
    "ci_provider",
    "release_matrix",
    "physical_project_container",
    "local_fea_package_format",
    "redaction_workflow",
}

REQUIRED_NO_BYPASS = {
    "must_use_public_api_boundary",
    "must_use_unit_validation",
    "must_preserve_provenance",
    "must_preserve_redistribution_review",
    "must_preserve_privacy_classification",
    "must_screen_protected_content",
    "must_preserve_diagnostics",
    "must_preserve_rule_pack_sandbox",
    "must_preserve_persistence_hash_controls",
    "must_route_persistence_through_application_services",
    "must_not_expose_sql_or_raw_sqlite",
    "must_not_expose_table_names",
    "must_not_mutate_project_store_directly",
    "must_preserve_report_controls",
    "must_preserve_human_acceptance_boundary",
    "must_not_execute_arbitrary_code",
    "must_not_access_network",
    "must_not_choose_filesystem_roots",
    "must_not_claim_code_compliance",
    "must_not_transmit_private_data_by_default",
}

REQUIRED_VALIDATION = {
    "schema_validation",
    "unit_validation",
    "dimension_validation",
    "provenance_validation",
    "redistribution_review",
    "privacy_classification",
    "protected_content_screening",
}

FORBIDDEN_TERMS = {
    "CODE_COMPLIANT",
    "CERTIFIED",
    "SEALED",
    "APPROVED_FOR_PROFESSIONAL_RELIANCE",
    "SECURITY_CERTIFIED",
    "COMPLIANCE_ATTESTED",
}


def load_json(path):
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def current_authority_fixture():
    fixture = load_json(FIXTURE_PATH)
    fixture["adapter_declaration"]["no_bypass_controls"].update(
        {key: True for key in REQUIRED_NO_BYPASS}
    )
    return fixture


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


def codes(result):
    return {finding.code for finding in result.findings}


def test_schema_contract_shape_and_traceability():
    schema = load_json(SCHEMA_PATH)
    defs = schema["$defs"]

    assert schema["$schema"] == "https://json-schema.org/draft/2020-12/schema"
    assert schema["additionalProperties"] is False
    assert "default" not in set(walk_keys(schema))
    assert REQUIRED_ROOT <= set(schema["required"])
    assert REQUIRED_DEFS <= set(defs)

    assert schema["properties"]["deliverable_id"]["const"] == "DEL-10-02"
    assert schema["properties"]["package_id"]["const"] == "PKG-10"
    assert schema["properties"]["scope_item"]["const"] == "SOW-030"
    assert schema["properties"]["objective"]["const"] == "OBJ-009"


def test_schema_keeps_runtime_and_format_decisions_tbd():
    schema = load_json(SCHEMA_PATH)

    status = schema["$defs"]["FrameworkStatus"]["properties"]
    assert status["interface_kind"]["const"] == (
        "schema_first_format_neutral_adapter_framework"
    )
    for key in REQUIRED_TBD - {"package_scripts"}:
        assert status[key]["const"] == "TBD"

    tbd = schema["$defs"]["TbdDecisions"]
    assert REQUIRED_TBD <= set(tbd["required"])
    for key in REQUIRED_TBD:
        assert tbd["properties"][key]["const"] == "TBD"


def test_schema_requires_no_bypass_and_validation_hooks():
    schema = load_json(SCHEMA_PATH)

    assert REQUIRED_NO_BYPASS <= required_at(schema, "NoBypassControls")
    for key in REQUIRED_NO_BYPASS:
        assert schema["$defs"]["NoBypassControls"]["properties"][key]["const"] is True

    assert REQUIRED_VALIDATION <= required_at(schema, "ValidationPlan")
    plan = schema["$defs"]["ValidationPlan"]["properties"]
    for key in REQUIRED_VALIDATION:
        assert plan[key]["const"] == "required"
    assert plan["human_review_required"]["const"] is True

    assert {
        "import_model",
        "export_model",
        "import_library",
        "export_library",
        "validate_payload",
        "contribution_review",
    } <= enum_at(schema, "AdapterCapability")


def test_schema_preserves_diagnostics_privacy_result_and_authority_boundaries():
    schema = load_json(SCHEMA_PATH)
    defs = schema["$defs"]

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

    diagnostic_classes = set(defs["Diagnostic"]["properties"]["class"]["enum"])
    assert {
        "PROVENANCE_WARNING",
        "IP_BOUNDARY_WARNING",
        "UNIT_WARNING",
        "PRIVACY_WARNING",
        "EXPORT_BLOCKING",
        "ADAPTER_BLOCKING",
        "RULE_CHECK_BLOCKING",
    } <= diagnostic_classes

    privacy = defs["PrivacyContext"]["properties"]
    assert privacy["local_first"]["const"] is True
    assert privacy["telemetry_allowed"]["const"] is False
    assert "protected_suspected" in privacy["classification"]["enum"]
    assert "export_review_required" in privacy["classification"]["enum"]

    result_ref = defs["ResultEnvelopeRef"]["properties"]
    assert result_ref["schema_ref"]["const"] == "schemas/results.schema.yaml"
    assert result_ref["compatibility"]["const"] == "schema_first_json_result_envelope"

    boundary = defs["ProfessionalBoundary"]["properties"]
    assert boundary["human_review_required"]["const"] is True
    assert boundary["mechanics_solve_distinct"]["const"] is True
    assert boundary["user_rule_check_distinct"]["const"] is True
    assert boundary["software_makes_compliance_claim"]["const"] is False
    assert boundary["software_makes_certification_claim"]["const"] is False
    assert boundary["software_makes_sealing_claim"]["const"] is False
    assert boundary["software_makes_approval_claim"]["const"] is False
    assert boundary["software_makes_security_certification_claim"]["const"] is False


def test_invented_fixture_is_format_neutral_and_accepted():
    fixture = load_json(FIXTURE_PATH)
    result = validate_adapter_declaration(fixture)

    assert result.accepted is True
    assert result.outcome == "ACCEPTED_FORMAT_NEUTRAL_DECLARATION"
    assert result.findings == ()
    assert fixture["adapter_declaration"]["format_status"] == "TBD"
    assert fixture["operation_result"]["parse_status"] == "not_parsed_by_framework"
    assert fixture["adapter_declaration"]["privacy"]["local_first"] is True
    assert fixture["adapter_declaration"]["privacy"]["telemetry_allowed"] is False


def test_concrete_format_selection_is_rejected():
    fixture = current_authority_fixture()
    fixture["adapter_declaration"]["format_status"] = "real_format_name"

    result = validate_adapter_declaration(fixture)

    assert result.accepted is False
    assert result.outcome == "REJECTED"
    assert "ADAPTER_FORMAT_SELECTED" in codes(result)


def test_direct_persistence_access_is_rejected():
    fixture = current_authority_fixture()
    fixture["adapter_declaration"]["persistence_escape_hatch"] = {
        "raw_sqlite_handle": "sqlite://project.db",
        "table_names": ["projects", "model_objects"],
        "direct_store_mutation": True,
    }

    result = validate_adapter_declaration(fixture)

    assert result.accepted is False
    assert result.outcome == "REJECTED"
    assert "ADAPTER_FORBIDDEN_PERSISTENCE_ACCESS" in codes(result)


def test_missing_provenance_blocks_adapter_declaration():
    fixture = current_authority_fixture()
    del fixture["adapter_declaration"]["provenance"]["source_license"]

    result = validate_adapter_declaration(fixture)

    assert result.accepted is False
    assert result.outcome == "REJECTED"
    assert "ADAPTER_PROVENANCE_INCOMPLETE" in codes(result)


def test_protected_suspected_fixture_quarantines():
    fixture = current_authority_fixture()
    fixture["adapter_declaration"]["provenance"][
        "redistribution_status"
    ] = "protected_suspected"

    result = validate_adapter_declaration(fixture)

    assert result.accepted is False
    assert result.outcome == "QUARANTINE"
    assert "ADAPTER_PROTECTED_CONTENT_SUSPECTED" in codes(result)


def test_no_bypass_controls_are_enforced():
    fixture = current_authority_fixture()
    fixture["adapter_declaration"]["no_bypass_controls"][
        "must_use_unit_validation"
    ] = False

    result = validate_adapter_declaration(fixture)

    assert result.accepted is False
    assert "ADAPTER_NO_BYPASS_CONTROL_DISABLED" in codes(result)


def test_selected_adapter_runtime_gate_has_no_dispatch_bypass():
    guarded_controls = {
        "must_use_unit_validation",
        "must_preserve_provenance",
        "must_preserve_privacy_classification",
        "must_screen_protected_content",
        "must_preserve_rule_pack_sandbox",
        "must_preserve_persistence_hash_controls",
        "must_preserve_report_controls",
        "must_not_transmit_private_data_by_default",
    }
    for control in guarded_controls:
        fixture = current_authority_fixture()
        fixture["adapter_declaration"]["no_bypass_controls"][control] = False

        gated = gate_adapter_runtime_dispatch(fixture)

        assert gated.outcome == "REJECTED"
        assert gated.declaration_accepted is False
        assert gated.runtime_dispatched is False
        assert "ADAPTER_NO_BYPASS_CONTROL_DISABLED" in {
            finding.code for finding in gated.findings
        }


def test_selected_adapter_runtime_gate_quarantines_protected_suspected_content():
    fixture = current_authority_fixture()
    fixture["adapter_declaration"]["provenance"][
        "redistribution_status"
    ] = "protected_suspected"

    gated = gate_adapter_runtime_dispatch(fixture)

    assert gated.outcome == "QUARANTINE"
    assert gated.declaration_accepted is False
    assert gated.runtime_dispatched is False
    assert "ADAPTER_PROTECTED_CONTENT_SUSPECTED" in {
        finding.code for finding in gated.findings
    }


def test_valid_declaration_still_cannot_dispatch_without_selected_runtime():
    gated = gate_adapter_runtime_dispatch(current_authority_fixture())

    assert gated.outcome == "BLOCKED_RUNTIME_NOT_SELECTED"
    assert gated.declaration_accepted is True
    assert gated.runtime_dispatched is False
    assert [finding.code for finding in gated.findings] == [
        "ADAPTER_RUNTIME_NOT_SELECTED"
    ]


def test_application_service_persistence_no_bypass_controls_are_enforced():
    fixture = current_authority_fixture()
    fixture["adapter_declaration"]["no_bypass_controls"][
        "must_route_persistence_through_application_services"
    ] = False

    result = validate_adapter_declaration(fixture)

    assert result.accepted is False
    assert "ADAPTER_NO_BYPASS_CONTROL_DISABLED" in codes(result)


def test_operation_result_builder_preserves_boundaries():
    finding_result = validate_adapter_declaration({})
    built = build_result(
        operation_id="ops.adapter.test.validation",
        operation_class="validate",
        diagnostics=finding_result.findings[:1],
    )

    assert built["parse_status"] == "not_parsed_by_framework"
    assert built["privacy"]["local_first"] is True
    assert built["privacy"]["telemetry_allowed"] is False
    assert built["result_envelope_ref"]["schema_ref"] == "schemas/results.schema.yaml"
    assert built["professional_boundary"]["software_makes_compliance_claim"] is False


def test_schema_and_fixture_do_not_contain_forbidden_status_terms():
    combined = f"{SCHEMA_PATH.read_text(encoding='utf-8')}\n{FIXTURE_PATH.read_text(encoding='utf-8')}"
    combined_upper = combined.upper()
    for term in FORBIDDEN_TERMS:
        assert term not in combined_upper


def test_premature_tbd_resolution_is_rejected():
    fixture = copy.deepcopy(current_authority_fixture())
    fixture["tbd_decisions"]["public_transport_protocol"] = "http"

    result = validate_adapter_declaration(fixture)

    assert result.accepted is False
    assert "ADAPTER_DECISION_PREMATURE" in codes(result)


def test_protected_content_lint_cli_remains_release_tool_only():
    """REXC-LINT-001 stays a governed diagnostic, not a product export."""

    cli = (
        ROOT
        / "core"
        / "reporting"
        / "protected_content_linter"
        / "src"
        / "bin"
        / "protected_content_lint_cli.rs"
    )
    release_consumers = {
        ROOT / "tools" / "release" / "run_release_candidate_scan.py",
        ROOT / "tools" / "release" / "export_public_openpipestress.py",
    }
    assert cli.is_file()
    assert all(path.is_file() for path in release_consumers)
    assert all(
        "protected_content_lint_cli" in path.read_text(encoding="utf-8")
        for path in release_consumers
    )

    unexpected = []
    for root in (ROOT / "apps", ROOT / "core", ROOT / "tools"):
        for path in root.rglob("*"):
            if not path.is_file() or path.suffix not in {".py", ".rs", ".ts", ".tsx"}:
                continue
            if "target" in path.parts or path == cli or path in release_consumers:
                continue
            if "protected_content_lint_cli" in path.read_text(
                encoding="utf-8", errors="ignore"
            ):
                unexpected.append(path.relative_to(ROOT).as_posix())
    assert unexpected == []


if __name__ == "__main__":
    test_schema_contract_shape_and_traceability()
    test_schema_keeps_runtime_and_format_decisions_tbd()
    test_schema_requires_no_bypass_and_validation_hooks()
    test_schema_preserves_diagnostics_privacy_result_and_authority_boundaries()
    test_invented_fixture_is_format_neutral_and_accepted()
    test_concrete_format_selection_is_rejected()
    test_direct_persistence_access_is_rejected()
    test_missing_provenance_blocks_adapter_declaration()
    test_protected_suspected_fixture_quarantines()
    test_no_bypass_controls_are_enforced()
    test_application_service_persistence_no_bypass_controls_are_enforced()
    test_operation_result_builder_preserves_boundaries()
    test_schema_and_fixture_do_not_contain_forbidden_status_terms()
    test_premature_tbd_resolution_is_rejected()
    test_protected_content_lint_cli_remains_release_tool_only()
