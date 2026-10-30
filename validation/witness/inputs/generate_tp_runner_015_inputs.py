#!/usr/bin/env python3
"""Generate TP-RUNNER-015 CLI reproduction input fixtures.

The generated requests are intentionally invented/public metadata only. They
exercise the DEC-065 local CLI surface without adding protected standards data,
private project data, release packaging, network transport, or professional
reliance claims.
"""

from __future__ import annotations

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
OUT_DIR = Path(__file__).resolve().parent
PREVIEW_MODEL = ROOT / "fixtures" / "product_preview" / "invented_preview_model.json"


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


def provenance(source_name: str, source_location: str) -> dict[str, str]:
    return {
        "source_name": source_name,
        "source_location": source_location,
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


def request(operation: str, request_id: str, include_load_basis: bool = True) -> dict[str, object]:
    return {
        "request_id": request_id,
        "operation": operation,
        "operation_ref": reference("api_operation", operation),
        "project_ref": reference("project", "invented-project"),
        "model_ref": reference("model", "invented-model"),
        "unit_system_ref": reference("unit_system", "invented-si"),
        "load_basis_refs": [reference("load_case", "LC1")] if include_load_basis else [],
        "input_manifest_ref": reference("audit_manifest", f"manifest:{request_id}"),
        "requested_outputs": ["result_envelope", "audit_manifest", "diagnostics"],
        "privacy": privacy(),
        "provenance": provenance(
            "invented final CLI fixture",
            "validation/witness/inputs/generate_tp_runner_015_inputs.py",
        ),
        "professional_boundary": professional_boundary(),
        "tbd_decisions": SETTLED_TBD,
    }


def write_fixture(path: Path, payload: dict[str, object]) -> None:
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def main() -> None:
    preview_model = json.loads(PREVIEW_MODEL.read_text(encoding="utf-8"))
    write_fixture(
        OUT_DIR / "tp_runner_015_final_cli_solve_input.json",
        {
            "request": request("solve", "tp-runner-015-final-cli-solve"),
            "solve": {"preview_model": {"model": preview_model, "materials": []}},
        },
    )
    write_fixture(
        OUT_DIR / "tp_runner_015_final_cli_validation_blocking_input.json",
        {
            "request": request(
                "validate_input",
                "tp-runner-015-validation-blocking",
                include_load_basis=False,
            )
        },
    )
    write_fixture(
        OUT_DIR / "tp_runner_015_final_cli_benchmark_stub_input.json",
        {"request": request("run_benchmark", "tp-runner-015-benchmark-stub")},
    )


if __name__ == "__main__":
    main()
