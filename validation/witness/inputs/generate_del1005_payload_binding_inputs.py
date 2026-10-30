#!/usr/bin/env python3
"""Generate DEL-10-05 benchmark/regression payload-binding input fixtures.

Witness inputs for the CB-2026-07-19-DEL-10-05-RUNNER-PAYLOADS-001 tranche
that bound the `run-benchmark` and `run-regression` downstream payloads to the
existing suite crates (DEL-09-01 mechanics, DEL-09-02 stress, DEL-09-03
nonlinear). The generated requests are intentionally invented/public metadata
only. They exercise the DEC-065 local CLI surface without adding protected
standards data, private project data, release packaging, network transport, or
professional reliance claims. Per-case match/fail exercised through these
fixtures is regression evidence for current solver behavior only; release
thresholds, final tolerance policy, CI gate policy, and professional reliance
remain TBD pending human approval.

The frozen TP-RUNNER-015 inputs and their generator are intentionally not
touched by this script; it writes only new `del1005_payload_binding_*` files.
"""

from __future__ import annotations

import json
from pathlib import Path


OUT_DIR = Path(__file__).resolve().parent


SETTLED_TBD = {
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
}


def reference(ref_type: str, ref_id: str) -> dict[str, str]:
    return {"ref_type": ref_type, "ref_id": ref_id}


def privacy() -> dict[str, object]:
    return {
        "local_only": True,
        "telemetry_allowed": False,
        "private_payload_redacted": True,
        "classification": "public_metadata",
    }


def provenance() -> dict[str, str]:
    return {
        "source_name": "invented DEL-10-05 payload-binding CLI fixture",
        "source_location": (
            "validation/witness/inputs/generate_del1005_payload_binding_inputs.py"
        ),
        "source_license": "project invented",
        "contributor": "OpenPipeStress",
        "contributor_certification": "invented non-engineering example",
        "redistribution_status": "invented_non_engineering_example",
        "review_status": "accepted",
    }


def professional_boundary() -> dict[str, bool]:
    return {
        "human_review_required": True,
        "software_makes_compliance_claim": False,
        "software_makes_certification_claim": False,
        "software_makes_sealing_claim": False,
        "software_makes_approval_claim": False,
        "software_makes_authentication_claim": False,
    }


def request(operation: str, request_id: str) -> dict[str, object]:
    return {
        "request_id": request_id,
        "operation": operation,
        "operation_ref": reference("api_operation", operation),
        "project_ref": reference("project", "invented-project"),
        "model_ref": reference("model", "invented-model"),
        "unit_system_ref": reference("unit_system", "invented-si"),
        "load_basis_refs": [reference("load_case", "LC1")],
        "input_manifest_ref": reference("audit_manifest", f"manifest:{request_id}"),
        "requested_outputs": ["result_envelope", "audit_manifest", "diagnostics"],
        "privacy": privacy(),
        "provenance": provenance(),
        "professional_boundary": professional_boundary(),
        "tbd_decisions": SETTLED_TBD,
    }


def write_fixture(path: Path, payload: dict[str, object]) -> None:
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def main() -> None:
    write_fixture(
        OUT_DIR / "del1005_payload_binding_benchmark_single_case_input.json",
        {
            "request": request(
                "run_benchmark", "del1005-payload-binding-benchmark-single-case"
            ),
            "benchmark": {
                "suite": "mechanics",
                "cases": ["MECH-TP-PHYS-004-LOAD-TO-RESULTANT"],
            },
        },
    )
    write_fixture(
        OUT_DIR / "del1005_payload_binding_benchmark_multi_case_input.json",
        {
            "request": request(
                "run_benchmark", "del1005-payload-binding-benchmark-multi-case"
            ),
            "benchmark": {
                "suite": "stress",
                "cases": [
                    "STRESS-AXIAL-NORMAL-ORIGINAL",
                    "STRESS-RANGE-MECHANICS-ORIGINAL",
                    "STRESS-TP-PMM-P3-MILLTOL-EFFECTIVE-WALL-STRESS",
                ],
            },
        },
    )
    write_fixture(
        OUT_DIR / "del1005_payload_binding_regression_full_suite_input.json",
        {
            "request": request(
                "run_regression", "del1005-payload-binding-regression-full-suite"
            ),
            "regression": {"suite": "nonlinear"},
        },
    )
    write_fixture(
        OUT_DIR / "del1005_payload_binding_benchmark_payload_missing_input.json",
        {
            "request": request(
                "run_benchmark", "del1005-payload-binding-benchmark-payload-missing"
            )
        },
    )
    write_fixture(
        OUT_DIR / "del1005_payload_binding_regression_payload_missing_input.json",
        {
            "request": request(
                "run_regression", "del1005-payload-binding-regression-payload-missing"
            )
        },
    )


if __name__ == "__main__":
    main()
