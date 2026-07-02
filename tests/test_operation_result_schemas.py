#!/usr/bin/env python3
"""Stdlib checks for the DEL-10-03 published result schemas.

The two schemas mirror Rust result shapes field-by-field; the Rust sources
govern on any disagreement:

- schemas/operation_outcome.schema.json <-
  core/model_operations/operation_applier/src/lib.rs (OperationOutcome)
- schemas/rule_check_run_result.schema.json <-
  core/rules/rule_check_runner/src/lib.rs (RuleCheckRunResult)
"""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUTCOME_SCHEMA_PATH = ROOT / "schemas" / "operation_outcome.schema.json"
RULE_CHECK_SCHEMA_PATH = ROOT / "schemas" / "rule_check_run_result.schema.json"

OUTCOME_REQUIRED_ROOT = {
    "schema_version",
    "document_kind",
    "deliverable_refs",
    "mode",
    "application_route",
    "operation_id",
    "change_id",
    "operation_kind",
    "change_kind",
    "target_object_type",
    "target_ref",
    "validation",
    "diff_preview",
    "diagnostics",
    "model_basis",
    "input_model_unchanged",
    "applied_model",
    "applied_model_backend_hash",
    "acceptance",
    "audit_boundary",
    "professional_boundary",
}

OUTCOME_REQUIRED_DEFS = {
    "ValidationStates",
    "DiffPreviewRow",
    "OutcomeDiagnostic",
    "ModelBasisEvidence",
    "AcceptanceRecord",
}

RULE_CHECK_REQUIRED_ROOT = {
    "document_kind",
    "rule_pack_id",
    "grammar_version",
    "aggregate_status",
    "checks",
    "professional_boundary_notice",
}

RULE_CHECK_REQUIRED_DEFS = {
    "RuleCheckStatus",
    "ComputedQuantity",
    "BoundInput",
    "RunFinding",
    "CheckOutcome",
}

# Mirrors core/rules/rule_check_runner/src/lib.rs PROFESSIONAL_BOUNDARY_NOTICE.
RULE_CHECK_BOUNDARY_NOTICE = (
    "Software rule-check evidence only; not a professional, certification, "
    "sealing, authentication, approval, or code-compliance claim. Human "
    "review remains required."
)

PROFESSIONAL_BOUNDARY_FALSE_CLAIMS = {
    "software_makes_compliance_claim",
    "software_makes_certification_claim",
    "software_makes_sealing_claim",
    "software_makes_approval_claim",
    "software_makes_authentication_claim",
}


def load(path):
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def test_operation_outcome_schema_structure():
    schema = load(OUTCOME_SCHEMA_PATH)
    assert schema["$schema"] == "https://json-schema.org/draft/2020-12/schema"
    assert schema["additionalProperties"] is False
    assert set(schema["required"]) == OUTCOME_REQUIRED_ROOT
    assert set(schema["properties"]) == OUTCOME_REQUIRED_ROOT
    assert set(schema["$defs"]) == OUTCOME_REQUIRED_DEFS
    for name, definition in schema["$defs"].items():
        assert definition.get("additionalProperties") is False, name


def test_operation_outcome_closed_vocabularies():
    schema = load(OUTCOME_SCHEMA_PATH)
    props = schema["properties"]
    assert props["document_kind"]["const"] == (
        "openpipestress.desktop.operation_outcome"
    )
    assert props["mode"]["enum"] == ["validate_only", "apply"]
    assert props["application_route"]["enum"] == [
        "tauri_backend_apply",
        "local_wasm_engine",
    ]
    assert props["input_model_unchanged"]["const"] is True
    validation = schema["$defs"]["ValidationStates"]["properties"]
    assert validation["schema_validation"]["enum"] == ["passed", "blocked"]
    assert validation["before_state_validation"]["enum"] == [
        "not_run",
        "passed",
        "blocked_stale",
    ]
    assert validation["application_status"]["enum"] == [
        "not_applied",
        "applied_to_session_model",
        "blocked",
    ]
    basis = schema["$defs"]["ModelBasisEvidence"]["properties"]
    assert basis["backend_canonicalization"]["const"] == "rfc8785_jcs"
    assert basis["binding_status"]["enum"] == [
        "claimed_hash_echoed_cross_canonicalization_equality_not_evaluated",
        "no_claimed_hash_before_state_check_is_the_staleness_guard",
    ]
    acceptance = schema["$defs"]["AcceptanceRecord"]["properties"]
    assert acceptance["acceptance_is_professional_approval"]["const"] is False
    assert acceptance["acceptance_basis"]["enum"] == [
        "user_initiated_apply_in_local_session",
        "none_validation_only",
    ]


def test_operation_outcome_professional_boundary_flags():
    schema = load(OUTCOME_SCHEMA_PATH)
    boundary = schema["properties"]["professional_boundary"]
    assert boundary["additionalProperties"] is False
    flags = boundary["properties"]
    assert flags["human_review_required"]["const"] is True
    for claim in PROFESSIONAL_BOUNDARY_FALSE_CLAIMS:
        assert flags[claim]["const"] is False, claim
    audit = schema["properties"]["audit_boundary"]["properties"]
    assert audit["mutation_route"]["const"] == "structured_operations_only"
    assert audit["direct_model_mutation_allowed"]["const"] is False


def test_rule_check_run_result_schema_structure():
    schema = load(RULE_CHECK_SCHEMA_PATH)
    assert schema["$schema"] == "https://json-schema.org/draft/2020-12/schema"
    assert schema["additionalProperties"] is False
    assert set(schema["required"]) == RULE_CHECK_REQUIRED_ROOT
    assert set(schema["properties"]) == RULE_CHECK_REQUIRED_ROOT
    assert set(schema["$defs"]) == RULE_CHECK_REQUIRED_DEFS
    for name, definition in schema["$defs"].items():
        if name == "RuleCheckStatus":
            continue
        assert definition.get("additionalProperties") is False, name


def test_rule_check_run_result_closed_vocabularies():
    schema = load(RULE_CHECK_SCHEMA_PATH)
    props = schema["properties"]
    assert props["document_kind"]["const"] == "openpipestress.rule_check.run"
    assert props["professional_boundary_notice"]["const"] == (
        RULE_CHECK_BOUNDARY_NOTICE
    )
    assert schema["$defs"]["RuleCheckStatus"]["enum"] == [
        "RULE_INPUTS_INCOMPLETE",
        "USER_RULE_CHECKED",
        "USER_RULE_FAILED",
    ]
    outcome = schema["$defs"]["CheckOutcome"]
    assert outcome["properties"]["acceptability_relation"]["enum"] == [
        "less_than",
        "less_than_or_equal",
        "greater_than",
        "greater_than_or_equal",
        "formula_predicate",
        "none",
    ]
    # computed_value / limit_value are serde-skipped when absent — never required.
    assert "computed_value" not in outcome["required"]
    assert "limit_value" not in outcome["required"]
    bound = schema["$defs"]["BoundInput"]
    assert set(bound["required"]) == {"input_id", "source_kind", "supplied"}


if __name__ == "__main__":
    for name, func in sorted(globals().items()):
        if name.startswith("test_") and callable(func):
            func()
            print(f"PASS {name}")
