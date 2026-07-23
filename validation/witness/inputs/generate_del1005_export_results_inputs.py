#!/usr/bin/env python3
"""Generate deterministic invented DEL-10-05 export-results witnesses."""

from __future__ import annotations

import copy
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
HERE = Path(__file__).resolve().parent


def reference(ref_type: str, ref_id: str) -> dict[str, str]:
    return {"ref_type": ref_type, "ref_id": ref_id}


def provenance() -> dict[str, object]:
    return {
        "source_name": "Invented DEL-10-05 export-results witness",
        "source_location": (
            "validation/witness/inputs/"
            "generate_del1005_export_results_inputs.py"
        ),
        "source_license": "project invented",
        "contributor": "OpenPipeStress",
        "contributor_certification": "invented non-engineering example",
        "redistribution_status": "invented_non_engineering_example",
        "review_status": "accepted",
    }


def boundary() -> dict[str, bool]:
    return {
        "human_review_required": True,
        "software_makes_compliance_claim": False,
        "software_makes_certification_claim": False,
        "software_makes_sealing_claim": False,
        "software_makes_approval_claim": False,
        "software_makes_authentication_claim": False,
    }


def runner_request() -> dict[str, object]:
    return {
        "request_id": "del1005-export-results-invented",
        "operation": "export_results",
        "operation_ref": reference("api_operation", "export_results"),
        "project_ref": reference("project", "invented-project"),
        "model_ref": reference("model", "invented-model"),
        "unit_system_ref": reference("unit_system", "invented-si"),
        "load_basis_refs": [reference("load_case", "LC1")],
        "input_manifest_ref": reference("input_manifest", "manifest-1"),
        "requested_outputs": [
            "result_envelope",
            "audit_manifest",
            "diagnostics",
        ],
        "privacy": {
            "local_only": True,
            "telemetry_allowed": False,
            "private_payload_redacted": True,
            "classification": "public_metadata",
        },
        "provenance": provenance(),
        "professional_boundary": boundary(),
        "tbd_decisions": {
            "final_cli_command_syntax": "SETTLED_DEC_065",
            "package_scripts": "SETTLED_DEC_065",
            "process_invocation": "SETTLED_DEC_065",
            "network_access": "SETTLED_DEC_065",
            "filesystem_mutation_policy": "SETTLED_DEC_065",
            "ci_provider": "TBD",
            "release_matrix": "TBD",
            "public_transport_protocol": "TBD",
            "external_adapter_formats": "TBD",
            "physical_project_container": "TBD",
        },
    }


def renderable_report() -> dict[str, object]:
    fixture = json.loads(
        (
            ROOT / "fixtures/reports/invented/calculation_report_fixture.json"
        ).read_text(encoding="utf-8")
    )
    section_provenance = {
        **provenance(),
        "privacy_classification": "invented_public_example",
    }
    return {
        "report_title": "Invented DEL-10-05 Export Results Report",
        "calculation_report": fixture["calculation_report"],
        "report_sections": {
            "report_section_id": "del1005-invented-report-sections",
            "model_ref": reference("model", "invented-model"),
            "run_ref": reference("analysis_run", "invented-run"),
            "diagnostics": [],
            "analysis_status_disclosures": [
                {
                    "status": "MECHANICS_SOLVED",
                    "source": reference("solver", "open_pipe_stress_product_physics"),
                    "affected_object": reference("model", "invented-model"),
                    "explanation": "Invented deterministic mechanics witness.",
                    "human_review_required": True,
                    "human_acceptance_ref": None,
                },
                {
                    "status": "HUMAN_REVIEW_REQUIRED",
                    "source": reference("report_package", "invented-package"),
                    "affected_object": reference("model", "invented-model"),
                    "explanation": "Human review is required before reliance.",
                    "human_review_required": True,
                    "human_acceptance_ref": None,
                },
            ],
            "provenance_notes": [section_provenance],
            "user_supplied_values": [],
            "assumptions": [],
            "limitations": [],
            "unresolved_tbds": [],
            "professional_boundary": boundary(),
        },
        "result_rows": [
            {
                "row_id": "invented-row-001",
                "label": "Invented stress",
                "case_ref": "load_case:LC1",
                "quantity_display": "12.5 Pa",
                "source_ref": "result_envelope:invented-result-envelope-001",
            }
        ],
    }


def checksum(payload_type: str, payload_id: str, value: str) -> dict[str, object]:
    return {
        "algorithm": "sha256",
        "canonicalization": "rfc8785_jcs",
        "payload_ref": reference(payload_type, payload_id),
        "value": value,
    }


def export_results(package_id: str = "del1005-invented-package") -> dict[str, object]:
    hex_a = "a" * 64
    hex_b = "b" * 64
    return {
        "package_id": package_id,
        "export_profile_id": "runner_local_private_report_package_1",
        "source_model_ref": reference("model", "invented-model"),
        "source_basis_refs": [
            reference("analysis_run", "invented-run"),
            reference("load_case", "LC1"),
        ],
        "report": renderable_report(),
        "audit_manifest": {
            "manifest_id": "manifest-1",
            "model_hash": {
                "algorithm": "sha256",
                "canonicalization": "project_local_deterministic_json",
                "payload_kind": "model_json",
                "payload_ref": "invented-model",
                "value": hex_a,
            },
            "input_manifest_hash": {
                "algorithm": "sha256",
                "canonicalization": "project_local_deterministic_json",
                "payload_kind": "input_manifest_json",
                "payload_ref": "manifest-1",
                "value": hex_b,
            },
            "solver_version": {
                "solver_name": "open_pipe_stress_product_physics",
                "solver_version": "0.1.0",
                "solver_build_ref": "open_pipe_stress_product_physics@0.1.0",
            },
            "unit_system_ref": "invented-si",
            "rule_pack_refs": [],
            "assets": [],
            "professional_boundary": boundary(),
        },
        "result_envelopes": [
            {
                "envelope_id": "invented-result-envelope-001",
                "schema_version": "0.1.0",
                "model_ref": reference("model", "invented-model"),
                "run_ref": reference("analysis_run", "invented-run"),
                "solver_name": "open_pipe_stress_product_physics",
                "solver_version": "0.1.0",
                "solver_build_ref": "open_pipe_stress_product_physics@0.1.0",
                "unit_system_ref": reference("unit_system", "invented-si"),
                "load_basis_refs": [reference("load_case", "LC1")],
                "result_sets": [
                    {
                        "set_id": "invented-set-1",
                        "set_type": "mechanics",
                        "basis_ref": reference("load_case", "LC1"),
                        "values": [
                            {
                                "result_id": "invented-stress-1",
                                "family": "stress",
                                "object_ref": reference("node", "N1"),
                                "basis_ref": reference("load_case", "LC1"),
                                "station_ref": None,
                                "magnitude": 12.5,
                                "unit": "Pa",
                                "dimension": "stress",
                                "metadata": None,
                                "diagnostics": [],
                                "trace_chain": [],
                                "provenance": provenance(),
                            }
                        ],
                    }
                ],
                "diagnostics": [],
                "provenance": provenance(),
                "reproducibility": {
                    "model_hash": checksum("model", "invented-model", hex_a),
                    "run_hashes": [
                        checksum("analysis_run", "invented-run", hex_b)
                    ],
                    "audit_manifest_ref": reference("audit_manifest", "manifest-1"),
                    "deterministic_ordering": True,
                },
                "analysis_status": [
                    "MECHANICS_SOLVED",
                    "HUMAN_REVIEW_REQUIRED",
                ],
                "rule_pack_refs": [],
                "professional_boundary": boundary(),
            }
        ],
        "state_comparison_handoff_records": [
            {
                "schema_version": "0.1.0",
                "deliverable_id": "DEL-08-06",
                "package_id": "PKG-08",
                "scope_item": "SOW-024",
                "section_set_id": "invented-section-set-001",
                "section_contract_status": "backend_report_section_records_only",
                "sections": {
                    "state_run_sections": [],
                    "comparison_sections": [],
                    "handoff_sections": [],
                },
                "diagnostics": [],
                "professional_boundary": {
                    **boundary(),
                    "software_creates_professional_reliance_record": False,
                    "software_creates_external_validation_record": False,
                },
            }
        ],
        "rule_check_aggregate": None,
        "solve_rule_check_status": "RULE_INPUTS_INCOMPLETE",
    }


def write(name: str, value: dict[str, object]) -> None:
    (HERE / name).write_text(
        json.dumps(value, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )


def main() -> None:
    success = {
        "request": runner_request(),
        "export_results": export_results(),
    }
    write("del1005_export_results_success_input.json", success)
    write(
        "del1005_export_results_missing_payload_input.json",
        {"request": runner_request()},
    )
    mismatch = copy.deepcopy(success)
    mismatch["request"]["model_ref"]["ref_id"] = "invented-mismatch"
    write("del1005_export_results_binding_mismatch_input.json", mismatch)
    blocked = copy.deepcopy(success)
    blocked["export_results"]["package_id"] = ""
    write("del1005_export_results_producer_blocked_input.json", blocked)


if __name__ == "__main__":
    main()
