#!/usr/bin/env python3
"""Focused tests for DEL-15-03 handoff export workflow."""

from __future__ import annotations

from copy import deepcopy
import json
from pathlib import Path
import sys
from unittest.mock import patch

from jsonschema import Draft202012Validator
import pytest


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.handoff.exporter import (  # noqa: E402
    build_handoff_export_workflow,
    canonical_json,
)
from core.handoff.target_mapping import build_target_mapping_contract  # noqa: E402
from core.security.redaction.route_control import control_route_export  # noqa: E402


FIXTURE_PATH = (
    ROOT
    / "execution"
    / "PKG-15_Handoff and External Prover Workflow"
    / "1_Working"
    / "DEL-15-03_Downstream modeling export workflow"
    / "fixtures"
    / "invented_target_fixture.json"
)
HANDOFF_SCHEMA_PATH = ROOT / "schemas" / "handoff_package.schema.json"
TARGET_MAPPING_SCHEMA_PATH = ROOT / "schemas" / "target_mapping.schema.json"

FORBIDDEN_CLAIMS = {
    "code " + "compliant",
    "cert" + "ified",
    "se" + "aled",
    "authenticated",
    "professional approval",
    "external validation",
    "engineering acceptance",
}


def ref(object_type: str, value: str) -> dict[str, str]:
    return {"object_type": object_type, "ref": value}


def load_schema(path: Path) -> dict[str, object]:
    with path.open(encoding="utf-8") as schema_file:
        return json.load(schema_file)


def validate_with_schema(payload: dict[str, object], path: Path) -> None:
    schema = load_schema(path)
    Draft202012Validator.check_schema(schema)
    validator = Draft202012Validator(schema)
    errors = sorted(validator.iter_errors(payload), key=lambda item: item.path)
    assert not errors, [error.message for error in errors]


def provenance(source_name: str = "Invented DEL-15-03 fixture") -> dict[str, str]:
    return {
        "source_name": source_name,
        "source_location": "execution/PKG-15_Handoff and External Prover Workflow/1_Working/DEL-15-03_Downstream modeling export workflow/fixtures/invented_target_fixture.json",
        "source_license": "project-invented-test-data",
        "contributor": "OpenPipeStress",
        "contributor_attestation": "invented non-engineering fixture",
        "redistribution_status": "invented_non_engineering_example",
        "review_classification": "machine_checked",
        "privacy_classification": "invented_public_example",
    }


def checksum(payload_ref: dict[str, str], scope: str, value: str) -> dict[str, object]:
    return {
        "algorithm": "sha256",
        "canonicalization": "deterministic_sorted_compact_json_payload_hash",
        "payload_ref": payload_ref,
        "payload_scope": scope,
        "value": value,
    }


def redacted_checksum() -> dict[str, object]:
    return {
        "algorithm": "[REDACTED]",
        "canonicalization": "[REDACTED]",
        "payload_ref": {"object_type": "[REDACTED]", "ref": "[REDACTED]"},
        "payload_scope": "[REDACTED]",
        "value": "[REDACTED]",
    }


MODEL_HASH = checksum(ref("Model", "model:invented-del-15-03"), "model_hash", "sha256:invented-model")
MISSING = object()


def test_supplied_checksum_uses_narrow_label_and_is_carried_without_jcs_claim():
    package = handoff_package()
    workflow = build_handoff_export_workflow(
        export_workflow_id="export:invented-del-15-03-label-check",
        handoff_package=package,
        target_fixture=target_fixture(),
        target_mapping_contract=target_mapping(package),
    )
    assert MODEL_HASH["canonicalization"] == (
        "deterministic_sorted_compact_json_payload_hash"
    )
    assert package["handoff_package_manifest"]["model_hash"] == MODEL_HASH
    assert workflow["export_payload"]["model_hash"] == redacted_checksum()
    checksum_decisions = [
        item
        for item in workflow.decisions
        if ".__route_key__model_hash." in item.path
    ]
    assert checksum_decisions
    assert all(item.action == "redact_value" for item in checksum_decisions)
    assert {
        item.privacy_classification for item in checksum_decisions
    } <= {"unknown", "private_project_data"}
    assert "JCS" not in canonical_json(workflow)


def handoff_package() -> dict[str, object]:
    units_manifest = {
        "unit_system_ref": ref("UnitSystem", "units:invented-del-15-03"),
        "units_schema": "schemas/units.schema.yaml",
        "dimension_basis": "schemas/units.schema.yaml",
        "coordinate_unit": "m",
        "force_unit": "N",
        "moment_unit": "N-m",
        "displacement_unit": "m",
        "rotation_unit": "rad",
        "stress_unit": "Pa",
        "temperature_unit": "degC",
        "hash_refs": [
            checksum(ref("UnitSystem", "units:invented-del-15-03"), "units_manifest", "sha256:invented-units")
        ],
        "provenance": provenance("Invented units manifest"),
    }
    entity_ids = {
        "component_ids": ["component:pipe-1"],
        "node_ids": ["node:1", "node:2"],
        "element_ids": ["element:1"],
        "station_ids": ["station:1"],
        "load_case_ids": ["load:weight"],
        "result_ids": ["result:displacement"],
        "model_state_ids": ["state:handoff"],
        "analysis_run_ids": ["run:mechanics-reference"],
        "mapping_id_refs": [ref("TargetMapping", "mapping:pipe-diameter")],
    }
    warning = {
        "code": "INVENTED-HANDOFF-WARNING",
        "class": "LOCAL_HANDOFF_WARNING",
        "severity": "warning",
        "source": ref("HandoffPackage", "handoff:invented-del-15-03"),
        "affected_object": ref("Model", "model:invented-del-15-03"),
        "message": "Invented handoff warning for downstream review.",
        "remediation": "Review invented target fixture limitations.",
        "provenance": provenance("Invented warning"),
    }
    assumption = {
        "assumption_id": "assumption:invented-review",
        "statement": "Invented downstream target fixture requires review before use.",
        "status": "requires_human_review",
        "affected_refs": [ref("Model", "model:invented-del-15-03")],
        "provenance": provenance("Invented assumption"),
    }
    unsupported = {
        "flag_id": "unsupported:external-solver-not-invoked",
        "behavior_label": "external_solver_not_invoked",
        "status": "not_implemented",
        "target_ref": ref("ExternalReference", "target:invented-generic"),
        "affected_refs": [ref("AnalysisRun", "run:mechanics-reference")],
        "human_review_required": True,
        "provenance": provenance("Invented unsupported flag"),
    }
    return {
        "schema_version": "0.1.0",
        "deliverable_id": "DEL-15-01",
        "package_id": "PKG-15",
        "scope_item": "SOW-074",
        "objectives": ["OBJ-017"],
        "handoff_contract_status": {
            "record_contract": "schema_first_canonical_handoff_package_records",
            "manifest_contract": "schema_first_handoff_manifest_metadata",
            "model_schema_binding": "schemas/model.schema.yaml",
            "units_schema_binding": "schemas/units.schema.yaml",
            "model_state_binding": "schemas/model_state.schema.json",
            "analysis_run_binding": "schemas/analysis_run.schema.json",
            "result_export_binding": "schemas/results.schema.yaml",
            "local_fea_handoff_binding": "schemas/local_fea_handoff.schema.yaml",
            "audit_hash_binding": "hash_records_follow_audit_manifest_predecessor_semantics",
            "target_mapping_taxonomy": "reserved_for_DEL-15-02",
            "physical_package_container": "TBD",
            "external_prover_status": "not_declared_by_handoff_package",
            "professional_boundary": "downstream_modeling_and_review_support_only",
        },
        "handoff_package_manifest": {
            "package_identity": {
                "handoff_package_id": "handoff:invented-del-15-03",
                "manifest_id": "manifest:invented-del-15-03",
                "package_schema_version": "0.1.0",
                "created_at": "2026-05-06T00:00:00Z",
                "created_by": "OpenPipeStress DEL-15-03 fixture",
                "package_label": "Invented downstream modeling handoff",
                "package_kind": "canonical_handoff_manifest",
            },
            "model_basis": {
                "model_ref": ref("Model", "model:invented-del-15-03"),
                "model_schema": "schemas/model.schema.yaml",
                "model_kind": "handoff_basis",
                "basis_state_ref": ref("ModelState", "state:handoff"),
                "source_refs": [ref("Project", "project:invented-public")],
                "hash_refs": [MODEL_HASH],
                "provenance": provenance("Invented model basis"),
            },
            "model_hash": MODEL_HASH,
            "units_manifest": units_manifest,
            "entity_ids": entity_ids,
            "model_state_refs": [
                {
                    "model_state_ref": ref("ModelState", "state:handoff"),
                    "model_state_schema": "schemas/model_state.schema.json",
                    "state_kind": "handoff_basis",
                    "hash_refs": [checksum(ref("ModelState", "state:handoff"), "model_state_record", "sha256:invented-state")],
                    "privacy_classification": "invented_public_example",
                    "provenance": provenance("Invented model state"),
                }
            ],
            "analysis_run_refs": [],
            "result_export_refs": [],
            "library_refs": [
                {
                    "library_ref": ref("Library", "library:invented-public"),
                    "library_kind": "component",
                    "version": "0.1.0",
                    "checksum": checksum(ref("Library", "library:invented-public"), "library_reference_metadata", "sha256:invented-library"),
                    "source_notice": "Invented public fixture metadata only.",
                    "redistribution_status": "invented_non_engineering_example",
                    "review_classification": "machine_checked",
                    "privacy_classification": "invented_public_example",
                    "private_payload_redacted": True,
                    "provenance": provenance("Invented library ref"),
                }
            ],
            "rule_pack_refs": [
                {
                    "rule_pack_id": "rule-pack:invented-public",
                    "version": "0.1.0",
                    "checksum": checksum(ref("RulePack", "rule-pack:invented-public"), "rule_pack_reference_metadata", "sha256:invented-rule-pack"),
                    "source_notice": "Invented rule-pack metadata only.",
                    "redistribution_status": "invented_non_engineering_example",
                    "review_classification": "machine_checked",
                    "privacy_classification": "invented_public_example",
                    "private_payload_redacted": True,
                    "provenance": provenance("Invented rule pack ref"),
                }
            ],
            "checksums": [MODEL_HASH],
            "target_mapping_metadata": {
                "mapping_schema_status": "reserved_metadata_only",
                "target_system_kind": "generic_downstream_modeling",
                "target_mapping_refs": [ref("TargetMapping", "tm:invented-del-15-03")],
                "field_mapping_summary": "Invented provider-neutral field mappings.",
                "unsupported_behavior_refs": [ref("UnsupportedBehaviorFlag", "unsupported:external-solver-not-invoked")],
                "approximate_behavior_refs": [ref("UnsupportedBehaviorFlag", "approximate:target-field")],
                "detailed_taxonomy_owner": "DEL-15-02",
                "provenance": provenance("Invented target mapping metadata"),
            },
            "unsupported_behavior_flags": [unsupported],
            "unresolved_assumptions": [assumption],
            "warnings": [warning],
            "diagnostics": [],
            "privacy": {
                "classification": "invented_public_example",
                "local_only": True,
                "telemetry_allowed": False,
                "private_payload_embedded": False,
                "protected_payload_embedded": False,
                "commercial_tool_payload_embedded": False,
                "redaction_refs": [ref("ExternalReference", "redaction:invented-public")],
            },
            "redistribution_classification": "invented_non_engineering_example",
            "review_classification": "machine_checked",
            "provenance": provenance("Invented handoff manifest"),
            "professional_boundary": {
                "human_review_required": True,
                "supports_downstream_modeling": True,
                "supports_downstream_review": True,
                "software_makes_compliance_claim": False,
                "software_makes_certification_claim": False,
                "software_makes_sealing_claim": False,
                "software_makes_approval_claim": False,
                "software_makes_authentication_claim": False,
                "software_creates_professional_reliance_record": False,
            },
        },
    }


def source_context(package: dict[str, object]) -> dict[str, object]:
    manifest = package["handoff_package_manifest"]
    return {
        "model_hash": manifest["model_hash"],
        "units_manifest_ref": manifest["units_manifest"]["unit_system_ref"],
        "entity_id_refs": [ref("EntityIdManifest", "entity-ids:invented-del-15-03")],
        "library_refs": [item["library_ref"] for item in manifest["library_refs"]],
        "rule_pack_refs": [ref("RulePack", item["rule_pack_id"]) for item in manifest["rule_pack_refs"]],
        "unresolved_assumption_refs": [
            ref("Assumption", item["assumption_id"])
            for item in manifest["unresolved_assumptions"]
        ],
        "warning_refs": [ref("Diagnostic", item["code"]) for item in manifest["warnings"]],
        "privacy_context": manifest["privacy"],
    }


def target_mapping(package: dict[str, object]) -> dict[str, object]:
    return build_target_mapping_contract(
        mapping_contract_id="tm:invented-del-15-03",
        target_system_kind="generic_downstream_modeling",
        target_ref=ref("ExternalReference", "target:invented-generic"),
        source_context=source_context(package),
        mapping_records=[
            {
                "mapping_id": "mapping:pipe-diameter",
                "mapping_kind": "field",
                "source_ref": ref("Component", "component:pipe-1"),
                "target_ref": ref("ExternalReference", "target:pipe-diameter"),
                "value_kind": "quantity",
                "unit_metadata": {"unit": "m", "dimension": "length"},
                "mapping_status": "mapped",
                "assumption_refs": [],
                "warning_refs": [],
            }
        ],
        unsupported_behaviors=[
            {
                "flag_id": "unsupported:mesh",
                "behavior_label": "mesh_generation_not_performed",
                "status": "not_implemented",
                "target_ref": ref("ExternalReference", "target:mesh"),
                "affected_refs": [ref("Model", "model:invented-del-15-03")],
                "assumption_refs": [ref("Assumption", "assumption:invented-review")],
            }
        ],
        approximate_behaviors=[
            {
                "flag_id": "approximate:target-field",
                "behavior_label": "approximate_behavior_requires_review",
                "status": "approximate",
                "target_ref": ref("ExternalReference", "target:review-field"),
                "affected_refs": [ref("Component", "component:pipe-1")],
                "warning_refs": [ref("Diagnostic", "INVENTED-HANDOFF-WARNING")],
            }
        ],
    )


def target_fixture() -> dict[str, object]:
    with FIXTURE_PATH.open(encoding="utf-8") as handle:
        return json.load(handle)


def test_export_workflow_is_deterministic_and_preserves_required_context():
    package = handoff_package()
    mapping = target_mapping(package)
    fixture = target_fixture()

    validate_with_schema(package, HANDOFF_SCHEMA_PATH)
    validate_with_schema(mapping, TARGET_MAPPING_SCHEMA_PATH)
    assert fixture["provenance"]["redistribution_status"] == "invented_non_engineering_example"
    assert fixture["provenance"]["privacy_classification"] == "invented_public_example"
    assert "protected standards data" in " ".join(fixture["notes"])

    first = build_handoff_export_workflow(
        export_workflow_id="export:invented-del-15-03",
        handoff_package=package,
        target_mapping_contract=mapping,
        target_fixture=fixture,
    )
    second = build_handoff_export_workflow(
        export_workflow_id="export:invented-del-15-03",
        handoff_package=deepcopy(package),
        target_mapping_contract=deepcopy(mapping),
        target_fixture=target_fixture(),
    )

    assert canonical_json(first) == canonical_json(second)
    payload = first["export_payload"]
    source_manifest = package["handoff_package_manifest"]
    assert source_manifest["model_hash"] == MODEL_HASH
    assert payload["model_hash"] == redacted_checksum()
    assert payload["units_manifest"]["unit_system_ref"] == {
        "object_type": "[REDACTED]",
        "ref": "[REDACTED]",
    }
    assert payload["units_manifest"]["coordinate_unit"] == "[REDACTED]"
    assert payload["units_manifest"]["hash_refs"] == [redacted_checksum()]
    assert payload["units_manifest"]["provenance"] == source_manifest["units_manifest"]["provenance"]
    assert payload["entity_ids"]["component_ids"] == ["[REDACTED]"]
    assert source_manifest["entity_ids"]["component_ids"] == ["component:pipe-1"]
    assert payload["library_refs"][0]["library_kind"] == "component"
    assert payload["library_refs"][0]["library_ref"]["ref"] == "[REDACTED]"
    assert payload["library_refs"][0]["checksum"] == redacted_checksum()
    assert source_manifest["library_refs"][0]["library_ref"]["ref"] != "[REDACTED]"
    assert payload["rule_pack_refs"][0]["rule_pack_id"] == "rule-pack:invented-public"
    assert payload["rule_pack_refs"][0]["checksum"] == redacted_checksum()
    assert payload["target_mapping_metadata"]["target_system_kind"] == "[REDACTED]"
    assert payload["target_mapping_metadata"]["target_mapping_refs"][0]["ref"] == "[REDACTED]"
    assert payload["target_mapping_metadata"]["provenance"] == source_manifest["target_mapping_metadata"]["provenance"]
    assert payload["unresolved_assumptions"][0]["assumption_id"] == "[REDACTED]"
    assert payload["unresolved_assumptions"][0]["provenance"] == source_manifest["unresolved_assumptions"][0]["provenance"]
    assert payload["warnings"][0]["code"] == "[REDACTED]"
    assert payload["warnings"][0]["provenance"] == source_manifest["warnings"][0]["provenance"]
    assert first["diagnostics"] == []


def test_unsupported_and_approximate_target_behavior_is_explicit():
    package = handoff_package()
    export = build_handoff_export_workflow(
        export_workflow_id="export:invented-del-15-03",
        handoff_package=package,
        target_mapping_contract=target_mapping(package),
        target_fixture=target_fixture(),
    )
    records = export["export_payload"]["unsupported_target_records"]
    assert len(records) == 4
    assert {item["record_id"] for item in records} == {"[REDACTED]"}
    assert all(item["human_review_required"] == "[REDACTED]" for item in records)
    assert all(item["affected_refs"] for item in records)
    assert all(
        affected["ref"] == "[REDACTED]"
        for item in records
        for affected in item["affected_refs"]
    )
    controlled_text = canonical_json(export)
    for raw_record_id in (
        "unsupported:external-solver-not-invoked",
        "unsupported:mesh",
        "approximate:target-field",
        "capability:external-solver",
    ):
        assert raw_record_id not in controlled_text


def test_unit_bearing_mapping_without_unit_metadata_is_blocked():
    package = handoff_package()
    mapping = target_mapping(package)
    mapping["mapping_records"][0]["unit_metadata"] = None

    with patch(
        "core.security.redaction.route_control._materialize",
        side_effect=AssertionError(
            "blocked source diagnostics must not reach materialization"
        ),
    ) as materialize:
        export = build_handoff_export_workflow(
            export_workflow_id="export:bad-unit",
            handoff_package=package,
            target_mapping_contract=mapping,
            target_fixture=target_fixture(),
        )

    assert export.blocked is True
    assert export.payload is None
    materialize.assert_not_called()
    assert export.summary["source_finding_count"] == 1
    assert export.summary["source_blocking_count"] == 1
    assert export.summary["redaction_blocking_count"] == 0
    assert export.summary["exposure_blocking_count"] == 1
    assert export.decisions
    assert export.findings
    assert any(
        ".__route_key__diagnostics[0].__route_key__code" in item.path
        and item.action == "redact_value"
        for item in export.decisions
    )
    assert any(
        ".__route_key__diagnostics[0].__route_key__severity" in item.path
        and item.action == "redact_value"
        for item in export.decisions
    )
    assert "EXP-UNIT-METADATA-MISSING" not in canonical_json(export)


@pytest.mark.parametrize(
    ("input_name", "expected_code"),
    (
        ("target_mapping_contract", "EXP-TARGET-MAPPING-MISSING"),
        ("target_fixture", "EXP-TARGET-FIXTURE-MISSING"),
        ("units_manifest", "EXP-HANDOFF-MANIFEST-FIELD-MISSING"),
    ),
)
@pytest.mark.parametrize(
    "invalid_value",
    (MISSING, None, []),
    ids=("missing", "null", "non-mapping"),
)
def test_invalid_mapping_inputs_are_controlled_source_blocks(
    input_name, expected_code, invalid_value
):
    package = handoff_package()
    kwargs = {
        "export_workflow_id": f"export:invalid-{input_name}",
        "handoff_package": package,
        "target_mapping_contract": target_mapping(package),
        "target_fixture": target_fixture(),
    }
    if input_name == "units_manifest":
        manifest = package["handoff_package_manifest"]
        if invalid_value is MISSING:
            manifest.pop("units_manifest")
        else:
            manifest["units_manifest"] = invalid_value
    elif invalid_value is MISSING:
        kwargs.pop(input_name)
    else:
        kwargs[input_name] = invalid_value

    source_findings = []

    def capture_source_findings(*args, **control_kwargs):
        source_findings.extend(deepcopy(control_kwargs["source_findings"]))
        return control_route_export(*args, **control_kwargs)

    with patch(
        "core.handoff.exporter.workflow.control_route_export",
        side_effect=capture_source_findings,
    ), patch(
        "core.security.redaction.route_control._materialize",
        side_effect=AssertionError(
            "invalid source inputs must not reach materialization"
        ),
    ) as materialize:
        export = build_handoff_export_workflow(**kwargs)

    assert [
        item["code"]
        for item in source_findings
        if item["severity"].casefold() == "blocking"
    ] == [expected_code]
    assert export.blocked is True
    assert export.payload is None
    materialize.assert_not_called()
    assert export.summary["source_finding_count"] == 1
    assert export.summary["source_blocking_count"] == 1
    assert export.summary["redaction_blocking_count"] == 0
    assert export.summary["lossless_blocking_count"] == 0
    assert export.summary["exposure_blocking_count"] == 1
    assert export.decisions
    assert export.findings
    assert expected_code not in canonical_json(export)


def test_empty_units_manifest_with_matching_null_source_ref_is_blocked():
    package = handoff_package()
    mapping = target_mapping(package)
    package["handoff_package_manifest"]["units_manifest"] = {}
    mapping["source_context"]["units_manifest_ref"] = None
    source_findings = []

    def capture_source_findings(*args, **control_kwargs):
        source_findings.extend(deepcopy(control_kwargs["source_findings"]))
        return control_route_export(*args, **control_kwargs)

    with patch(
        "core.handoff.exporter.workflow.control_route_export",
        side_effect=capture_source_findings,
    ), patch(
        "core.security.redaction.route_control._materialize",
        side_effect=AssertionError(
            "an empty units manifest must not reach materialization"
        ),
    ) as materialize:
        export = build_handoff_export_workflow(
            export_workflow_id="export:empty-units-manifest",
            handoff_package=package,
            target_mapping_contract=mapping,
            target_fixture=target_fixture(),
        )

    assert [
        item["code"]
        for item in source_findings
        if item["severity"].casefold() == "blocking"
    ] == ["EXP-HANDOFF-MANIFEST-FIELD-MISSING"]
    assert export.blocked is True
    assert export.payload is None
    materialize.assert_not_called()
    assert export.summary["source_finding_count"] == 1
    assert export.summary["source_blocking_count"] == 1
    assert export.summary["redaction_blocking_count"] == 0
    assert export.summary["lossless_blocking_count"] == 0
    assert export.summary["exposure_blocking_count"] == 1
    assert export.decisions
    assert export.findings
    assert "EXP-HANDOFF-MANIFEST-FIELD-MISSING" not in canonical_json(export)


def test_lossless_only_withholding_counts_its_exposure_blocker():
    export = control_route_export(
        {"value": "invented private payload"},
        route_id="REXC-LOSSLESS-COUNT-TEST",
        export_context="downstream_tool",
        require_lossless_materialization=True,
    )

    assert export.blocked is True
    assert export.payload is None
    assert export.summary["source_blocking_count"] == 0
    assert export.summary["redaction_blocking_count"] == 0
    assert export.summary["lossless_blocking_count"] == 1
    assert export.summary["exposure_blocking_count"] == 1
    assert export.summary["materialization_withheld"] is True
    assert export.decisions
    assert export.findings


def test_mapping_hash_and_units_mismatch_are_blocked_not_defaulted():
    package = handoff_package()
    mapping = target_mapping(package)
    mapping["source_context"]["model_hash"] = checksum(ref("Model", "model:other"), "model_hash", "sha256:other")
    mapping["source_context"]["units_manifest_ref"] = ref("UnitSystem", "units:other")

    export = build_handoff_export_workflow(
        export_workflow_id="export:mismatch",
        handoff_package=package,
        target_mapping_contract=mapping,
        target_fixture=target_fixture(),
    )

    assert export.blocked is True
    assert export.payload is None
    assert export.summary["source_finding_count"] == 2
    assert export.summary["source_blocking_count"] == 2
    assert export.summary["exposure_blocking_count"] == 2
    controlled_text = canonical_json(export)
    assert "EXP-MODEL-HASH-MISMATCH" not in controlled_text
    assert "EXP-UNITS-MANIFEST-MISMATCH" not in controlled_text


def test_target_fixture_authority_metadata_is_blocking_boundary_diagnostic():
    package = handoff_package()
    fixture = target_fixture()
    fixture["notes"] = ["Target fixture marked as " + "approved" + " by downstream reviewer."]
    fixture["free_metadata"] = {"review_label": "cert" + "ified target handoff"}
    fixture["unsupported_capabilities"][0]["behavior_label"] = "engineering_" + "acceptance"

    export = build_handoff_export_workflow(
        export_workflow_id="export:fixture-authority",
        handoff_package=package,
        target_mapping_contract=target_mapping(package),
        target_fixture=fixture,
    )

    assert export.blocked is True
    assert export.payload is None
    assert export.summary["source_finding_count"] == 3
    assert export.summary["source_blocking_count"] == 3
    assert export.summary["exposure_blocking_count"] == 3
    assert "EXP-PROHIBITED-AUTHORITY-TERM" not in canonical_json(export)


def test_export_output_contains_no_prohibited_authority_claims():
    package = handoff_package()
    export = build_handoff_export_workflow(
        export_workflow_id="export:boundary",
        handoff_package=package,
        target_mapping_contract=target_mapping(package),
        target_fixture=target_fixture(),
    )
    text = canonical_json(export).lower()

    for forbidden in FORBIDDEN_CLAIMS:
        assert forbidden not in text
    assert set(export["professional_boundary"].values()) == {"[REDACTED]"}
    assert export["professional_boundary"]["software_makes_compliance_claim"] == "[REDACTED]"


def main():
    test_export_workflow_is_deterministic_and_preserves_required_context()
    test_unsupported_and_approximate_target_behavior_is_explicit()
    test_unit_bearing_mapping_without_unit_metadata_is_blocked()
    test_mapping_hash_and_units_mismatch_are_blocked_not_defaulted()
    test_target_fixture_authority_metadata_is_blocking_boundary_diagnostic()
    test_export_output_contains_no_prohibited_authority_claims()


if __name__ == "__main__":
    main()
