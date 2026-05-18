#!/usr/bin/env python3
"""Stdlib checks for the report protected-content linter schema and fixtures."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "schemas" / "report_protected_content_linter.schema.yaml"
FIXTURE_PATH = ROOT / "fixtures" / "report_lint" / "invented" / "lint_run_fixture.json"
RISK_FIXTURE_PATH = (
    ROOT / "fixtures" / "report_lint" / "invented" / "synthetic_risk_template.txt"
)
SAFE_FIXTURE_PATH = (
    ROOT / "fixtures" / "report_lint" / "invented" / "safe_metadata_template.txt"
)

REQUIRED_ROOT = {
    "schema_version",
    "deliverable_id",
    "package_id",
    "scope_item",
    "objectives",
    "linter_status",
    "lint_run",
}

REQUIRED_DEFS = {
    "FindingClass",
    "FindingCode",
    "FindingSeverity",
    "LintConfiguration",
    "LintFinding",
    "LintRun",
    "LintSummary",
    "LintTarget",
    "LinterStatus",
    "PrivacyClassification",
    "Provenance",
    "RedistributionStatus",
    "Reference",
    "ReviewRoute",
    "ReviewStatus",
    "SourceLocation",
    "SurfaceKind",
}

REQUIRED_CODES = {
    "PROTECTED_CONTENT_SYNTHETIC_MARKER",
    "PRIVATE_DATA_SYNTHETIC_MARKER",
    "PROPRIETARY_SOURCE_SYNTHETIC_MARKER",
    "UNKNOWN_PROVENANCE_REVIEW_REQUIRED",
    "PROHIBITED_PROFESSIONAL_CLAIM",
    "SAFE_METADATA_ALLOWED",
}

REQUIRED_CLASSES = {
    "IP_BOUNDARY_WARNING",
    "PRIVATE_DATA_WARNING",
    "PROVENANCE_WARNING",
    "PROFESSIONAL_BOUNDARY_WARNING",
    "SAFE_METADATA",
}

FORBIDDEN_STATUS_TRUTH = {
    "legal_clearance",
    "security_sufficiency",
    "professional_approval",
}

FORBIDDEN_FIXTURE_TERMS = {
    "ASME",
    "B31",
    "B31J",
    "allowable stress table",
    "stress intensification factor table",
    "vendor catalog value",
    "real secret",
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

    assert schema["properties"]["deliverable_id"]["const"] == "DEL-08-05"
    assert schema["properties"]["package_id"]["const"] == "PKG-08"
    assert schema["properties"]["scope_item"]["const"] == "SOW-043"
    assert {"OBJ-002", "OBJ-007"} >= set(
        schema["properties"]["objectives"]["items"]["enum"]
    )

    linter_status = defs["LinterStatus"]["properties"]
    assert linter_status["baseline_linter"]["const"] == (
        "deterministic_public_surface_heuristic_linter"
    )
    assert linter_status["fixture_policy"]["const"] == "invented_synthetic_markers_only"
    assert linter_status["heuristic_only"]["const"] is True
    for field in FORBIDDEN_STATUS_TRUTH:
        assert linter_status[field]["const"] is False
    assert linter_status["ci_release_policy"]["const"] == "TBD"
    assert linter_status["redaction_export_controls"]["const"] == "TBD"
    assert (
        linter_status["educational_example_dependency"]["const"]
        == "DAG-001-E0621_RETAINED_CANDIDATE_NON_GATING"
    )

    config_required = required_at(schema, "LintConfiguration")
    assert {
        "public_surface_roots",
        "private_surface_default",
        "finding_order",
        "synthetic_marker_policy",
        "clean_scan_disclaimer",
        "candidate_edge_policy",
    } <= config_required
    assert (
        defs["LintConfiguration"]["properties"]["private_surface_default"]["const"]
        == "skip_unless_explicitly_authorized"
    )
    assert (
        defs["LintConfiguration"]["properties"]["candidate_edge_policy"]["const"]
        == "DAG-001-E0621_non_gating_no_DEL-11-04_dependency"
    )

    finding_required = required_at(schema, "LintFinding")
    assert {
        "finding_id",
        "code",
        "class",
        "severity",
        "target_ref",
        "source_location",
        "matched_policy",
        "excerpt",
        "message",
        "remediation",
        "review_route",
        "disposition",
        "provenance",
    } <= finding_required
    assert REQUIRED_CODES <= enum_at(schema, "FindingCode")
    assert REQUIRED_CLASSES <= enum_at(schema, "FindingClass")
    assert {"INFO", "WARNING", "BLOCKING"} <= enum_at(schema, "FindingSeverity")
    assert {
        "public_report_template",
        "public_report_example",
        "public_fixture",
        "private_user_template",
        "private_project_export",
    } <= enum_at(schema, "SurfaceKind")

    summary_required = required_at(schema, "LintSummary")
    assert {
        "target_count",
        "scanned_target_count",
        "skipped_private_target_count",
        "finding_count",
        "blocking_finding_count",
        "clean_scan_is_clearance",
    } <= summary_required
    assert (
        defs["LintSummary"]["properties"]["clean_scan_is_clearance"]["const"] is False
    )

    assert fixture["deliverable_id"] == "DEL-08-05"
    assert fixture["linter_status"]["heuristic_only"] is True
    assert fixture["linter_status"]["legal_clearance"] is False
    assert fixture["linter_status"]["professional_approval"] is False
    assert fixture["lint_run"]["configuration"]["private_surface_default"] == (
        "skip_unless_explicitly_authorized"
    )
    assert fixture["lint_run"]["configuration"]["candidate_edge_policy"] == (
        "DAG-001-E0621_non_gating_no_DEL-11-04_dependency"
    )

    findings = fixture["lint_run"]["findings"]
    assert [finding["finding_id"] for finding in findings] == sorted(
        finding["finding_id"] for finding in findings
    )
    assert {finding["code"] for finding in findings} == {
        "PROTECTED_CONTENT_SYNTHETIC_MARKER",
        "PROHIBITED_PROFESSIONAL_CLAIM",
    }
    assert all(finding["severity"] == "BLOCKING" for finding in findings)
    assert fixture["lint_run"]["summary"]["blocking_finding_count"] == 2
    assert fixture["lint_run"]["summary"]["clean_scan_is_clearance"] is False

    risk_fixture = RISK_FIXTURE_PATH.read_text(encoding="utf-8")
    safe_fixture = SAFE_FIXTURE_PATH.read_text(encoding="utf-8")
    assert "OPS_SYNTHETIC_PROTECTED_TABLE" in risk_fixture
    assert "human review required" in safe_fixture
    for forbidden in FORBIDDEN_FIXTURE_TERMS:
        assert forbidden not in risk_fixture
        assert forbidden not in safe_fixture

    serialized = json.dumps(schema) + json.dumps(fixture)
    assert "DEL-11-04_Invented educational example models" not in serialized
    assert "examples/models/invented" not in serialized


if __name__ == "__main__":
    main()
