#!/usr/bin/env python3
"""Stdlib checks for the headless runner contract."""

import json
from pathlib import Path
import subprocess

import pytest


ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "schemas" / "headless_runner.schema.yaml"
FINAL_RUNNER_PATH = ROOT / "core" / "runner" / "headless" / "src" / "bin" / "openpipestress-runner.rs"
COMPAT_RUNNER_PATH = ROOT / "core" / "runner" / "headless" / "src" / "bin" / "headless_preview_runner.rs"
HEADLESS_CRATE = ROOT / "core" / "runner" / "headless"
PREVIEW_FIXTURE = ROOT / "fixtures" / "product_preview" / "invented_preview_model.json"

REQUIRED_ROOT = {
    "schema_version",
    "deliverable_id",
    "package_id",
    "scope_items",
    "objectives",
    "runner_status",
    "tbd_decisions",
    "request",
    "result",
}

REQUIRED_DEFS = {
    "AnalysisStatus",
    "ChecksumRef",
    "Diagnostic",
    "HeadlessRunnerRequest",
    "HeadlessRunnerResult",
    "JobState",
    "PhysicalProjectContainer",
    "PrivacyContext",
    "ProfessionalBoundary",
    "Provenance",
    "Reference",
    "RequestedOutput",
    "ResultEnvelopeRef",
    "RunnerOperation",
    "RunnerStatus",
    "TbdDecisions",
}

REQUIRED_OPERATIONS = {
    "solve",
    "validate_input",
    "export_results",
    "run_benchmark",
    "run_regression",
    "TBD",
}

REQUIRED_DIAGNOSTIC_CLASSES = {
    "SOLVE_BLOCKING",
    "RULE_CHECK_BLOCKING",
    "PROVENANCE_WARNING",
    "ASSUMPTION_WARNING",
    "NONLINEAR_WARNING",
    "IP_BOUNDARY_WARNING",
    "UNIT_WARNING",
    "RUNNER_BLOCKING",
    "EXPORT_BLOCKING",
    "PRIVACY_WARNING",
}

FORBIDDEN_STATUS = {
    "HUMAN_APPROVED_FOR_PROJECT",
    "CODE_COMPLIANT",
    "CERTIFIED",
    "SEALED",
    "APPROVED",
}

REQUIRED_TBD = {
    "ci_provider",
    "release_matrix",
    "public_transport_protocol",
    "external_adapter_formats",
}

SETTLED_DEC_065 = {
    "final_cli_command_syntax",
    "package_scripts",
    "process_invocation",
    "network_access",
    "filesystem_mutation_policy",
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


def main():
    schema = load_schema()
    defs = schema["$defs"]

    assert schema["$schema"] == "https://json-schema.org/draft/2020-12/schema"
    assert schema["additionalProperties"] is False
    assert "default" not in set(walk_keys(schema))
    assert REQUIRED_ROOT <= set(schema["required"])
    assert REQUIRED_DEFS <= set(defs)

    assert schema["properties"]["deliverable_id"]["const"] == "DEL-10-05"
    assert schema["properties"]["package_id"]["const"] == "PKG-10"
    assert {"SOW-054", "SOW-032"} <= set(
        schema["properties"]["scope_items"]["items"]["enum"]
    )
    assert {"OBJ-008", "OBJ-009", "OBJ-012"} <= set(
        schema["properties"]["objectives"]["items"]["enum"]
    )

    runner_status = defs["RunnerStatus"]["properties"]
    assert runner_status["interface_kind"]["const"] == (
        "schema_first_headless_runner_contract"
    )
    assert (
        runner_status["physical_project_container"]["$ref"]
        == "#/$defs/PhysicalProjectContainer"
    )
    assert runner_status["final_cli_command_syntax"]["const"].startswith(
        "openpipestress-runner <solve|validate-input|export-results|"
    )
    assert runner_status["final_cli_command_syntax"]["const"].endswith(
        "[--explicit-local-private-intent]"
    )
    assert runner_status["package_scripts"]["const"] == "dev_test_convenience_only"
    assert runner_status["process_invocation"]["const"] == (
        "single_foreground_local_process"
    )
    assert runner_status["network_access"]["const"] == "none"
    assert runner_status["filesystem_mutation_policy"]["const"] == (
        "stdout_default_explicit_output_path_only"
    )
    for key in [
        "public_transport_protocol",
        "ci_provider",
        "release_matrix",
        "external_adapter_formats",
        "local_fea_package_format",
    ]:
        assert runner_status[key]["const"] == "TBD"

    tbd = defs["TbdDecisions"]
    assert REQUIRED_TBD <= set(tbd["required"])
    for key in REQUIRED_TBD:
        assert tbd["properties"][key]["const"] == "TBD"
    assert SETTLED_DEC_065 <= set(tbd["required"])
    for key in SETTLED_DEC_065:
        assert tbd["properties"][key]["const"] == "SETTLED_DEC_065"
    assert "physical_project_container" not in set(tbd["required"])
    assert "physical_project_container" not in tbd["properties"]

    physical = defs["PhysicalProjectContainer"]["properties"]
    assert physical["profile"]["const"] == "sqlite_local_project_store"
    assert physical["decision_ref"]["const"] == "SCA-003"
    assert physical["storage_role"]["const"] == "local_store_index_projection"
    assert physical["canonical_truth"]["const"] == "sorted_compact_json_payload"
    assert physical["sql_public_contract"]["const"] is False
    assert physical["direct_sql_access_allowed"]["const"] is False
    assert physical["hosted_db_allowed"]["const"] is False
    assert physical["network_required"]["const"] is False
    assert physical["sidecars_rebuildable"]["const"] is True

    request_required = required_at(schema, "HeadlessRunnerRequest")
    assert {
        "request_id",
        "operation",
        "operation_ref",
        "project_ref",
        "model_ref",
        "unit_system_ref",
        "load_basis_refs",
        "input_manifest_ref",
        "requested_outputs",
        "privacy",
        "provenance",
        "professional_boundary",
    } <= request_required
    assert REQUIRED_OPERATIONS <= enum_at(schema, "RunnerOperation")
    assert {"result_envelope", "audit_manifest", "diagnostics"} <= enum_at(
        schema, "RequestedOutput"
    )

    result_required = required_at(schema, "HeadlessRunnerResult")
    assert {
        "run_id",
        "job",
        "analysis_status",
        "result_envelope_ref",
        "result_refs",
        "audit_manifest_ref",
        "checksums",
        "diagnostics",
        "privacy",
        "provenance",
        "professional_boundary",
    } <= result_required

    result_ref = defs["ResultEnvelopeRef"]["properties"]
    assert result_ref["schema_ref"]["const"] == "schemas/results.schema.yaml"
    assert result_ref["compatibility"]["const"] == "schema_first_json_result_envelope"
    assert (
        defs["HeadlessRunnerResult"]["properties"]["result_refs"]["items"]["$ref"]
        == "#/$defs/Reference"
    )

    status = enum_at(schema, "AnalysisStatus")
    assert {
        "MODEL_INCOMPLETE",
        "MECHANICS_SOLVED",
        "RULE_INPUTS_INCOMPLETE",
        "USER_RULE_CHECKED",
        "USER_RULE_FAILED",
        "HUMAN_REVIEW_REQUIRED",
    } <= status
    assert status.isdisjoint(FORBIDDEN_STATUS)
    assert (
        defs["HeadlessRunnerResult"]["properties"]["analysis_status"]["contains"][
            "const"
        ]
        == "HUMAN_REVIEW_REQUIRED"
    )

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
    assert REQUIRED_DIAGNOSTIC_CLASSES <= set(
        defs["Diagnostic"]["properties"]["class"]["enum"]
    )

    privacy = defs["PrivacyContext"]["properties"]
    assert privacy["local_only"]["const"] is True
    assert privacy["telemetry_allowed"]["const"] is False
    assert "protected_suspected" in privacy["classification"]["enum"]

    boundary = defs["ProfessionalBoundary"]["properties"]
    assert boundary["human_review_required"]["const"] is True
    assert boundary["software_makes_compliance_claim"]["const"] is False
    assert boundary["software_makes_certification_claim"]["const"] is False
    assert boundary["software_makes_sealing_claim"]["const"] is False
    assert boundary["software_makes_approval_claim"]["const"] is False
    assert boundary["software_makes_authentication_claim"]["const"] is False

    checksum = defs["ChecksumRef"]["properties"]
    assert set(checksum["algorithm"]["enum"]) == {"sha256", "sha512", "TBD"}
    assert set(checksum["canonicalization"]["enum"]) == {"rfc8785_jcs", "NONE", "TBD"}
    assert "rfc8785_jcs" in checksum["canonicalization"]["enum"]
    assert "NONE" in checksum["canonicalization"]["enum"]
    assert defs["HeadlessRunnerResult"]["properties"]["checksums"]["minItems"] == 1

    schema_upper = SCHEMA_PATH.read_text(encoding="utf-8").upper()
    for term in FORBIDDEN_STATUS:
        assert term not in schema_upper


def test_headless_runner_contract():
    main()


def test_both_runner_exposure_paths_are_controlled_before_stdout_or_file_write():
    final_source = FINAL_RUNNER_PATH.read_text(encoding="utf-8")
    compat_source = COMPAT_RUNNER_PATH.read_text(encoding="utf-8")

    assert final_source.index("control_local_private(") < final_source.index(
        'println!("{rendered}")'
    )
    assert final_source.index("control_local_private(") < final_source.index(
        "std::fs::write(output_path"
    )
    assert "if !controlled.blocked" in final_source
    assert "args.explicit_local_private_intent" in final_source
    assert "HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD" in final_source

    assert compat_source.index("control_local_private(") < compat_source.index(
        'println!("{rendered}")'
    )
    assert "[--explicit-local-private-intent]" in compat_source
    assert "clean && !controlled.blocked" in compat_source


@pytest.fixture(scope="module")
def runner_binaries():
    build = subprocess.run(
        ["cargo", "build", "--quiet", "--bins"],
        cwd=HEADLESS_CRATE,
        capture_output=True,
        text=True,
        check=False,
    )
    assert build.returncode == 0, build.stderr
    target = HEADLESS_CRATE / "target" / "debug"
    final = target / "openpipestress-runner"
    compat = target / "headless_preview_runner"
    assert final.is_file()
    assert compat.is_file()
    return final, compat


def final_runner_request(operation):
    return {
        "request_id": f"subprocess-{operation.replace('_', '-')}",
        "operation": operation,
        "operation_ref": {"ref_type": "api_operation", "ref_id": operation},
        "project_ref": {"ref_type": "project", "ref_id": "invented-project"},
        "model_ref": {"ref_type": "model", "ref_id": "invented-model"},
        "unit_system_ref": {"ref_type": "unit_system", "ref_id": "invented-si"},
        "load_basis_refs": [{"ref_type": "load_case", "ref_id": "LC1"}],
        "input_manifest_ref": {"ref_type": "audit_manifest", "ref_id": "manifest-1"},
        "requested_outputs": ["result_envelope", "audit_manifest", "diagnostics"],
        "privacy": {
            "local_only": True,
            "telemetry_allowed": False,
            "private_payload_redacted": True,
            "classification": "public_metadata",
        },
        "provenance": {
            "source_name": "invented subprocess fixture",
            "source_location": "tests/test_headless_runner_contract.py",
            "source_license": "project invented",
            "contributor": "OpenPipeStress",
            "contributor_certification": "invented non-engineering example",
            "redistribution_status": "invented_non_engineering_example",
            "review_status": "accepted",
        },
        "professional_boundary": {
            "human_review_required": True,
            "software_makes_compliance_claim": False,
            "software_makes_certification_claim": False,
            "software_makes_sealing_claim": False,
            "software_makes_approval_claim": False,
            "software_makes_authentication_claim": False,
        },
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


def run_final(binary, verb, body, *args):
    return subprocess.run(
        [str(binary), verb, *args],
        input=json.dumps(body),
        capture_output=True,
        text=True,
        check=False,
    )


def active_final_runner_cases():
    preview_model = json.loads(PREVIEW_FIXTURE.read_text(encoding="utf-8"))
    return [
        ("validate-input", "validate_input", {}),
        (
            "solve",
            "solve",
            {"solve": {"preview_model": {"model": preview_model, "materials": []}}},
        ),
        (
            "run-benchmark",
            "run_benchmark",
            {
                "benchmark": {
                    "suite": "mechanics",
                    "cases": ["MECH-TP-PHYS-004-LOAD-TO-RESULTANT"],
                }
            },
        ),
        (
            "run-regression",
            "run_regression",
            {
                "regression": {
                    "suite": "nonlinear",
                    "cases": ["NL-NONCONVERGENCE-LIMIT-ORIGINAL"],
                }
            },
        ),
    ]


def test_final_runner_subprocess_covers_every_active_verb_and_stdout(runner_binaries):
    final, _ = runner_binaries
    for verb, operation, extra in active_final_runner_cases():
        body = {"request": final_runner_request(operation), **extra}
        completed = run_final(final, verb, body, "--explicit-local-private-intent")
        assert completed.returncode == 0, (verb, completed.stderr, completed.stdout)
        controlled = json.loads(completed.stdout)
        assert controlled["blocked"] is False
        assert controlled["payload"]["command"] == verb
        assert controlled["payload"]["operation"] == operation


def test_final_runner_subprocess_output_file_matches_stdout(runner_binaries, tmp_path):
    final, _ = runner_binaries
    output_path = tmp_path / "controlled-output.json"
    body = {"request": final_runner_request("validate_input")}
    completed = run_final(
        final,
        "validate-input",
        body,
        "--output",
        str(output_path),
        "--explicit-local-private-intent",
    )
    assert completed.returncode == 0, completed.stderr
    assert output_path.is_file()
    assert json.loads(output_path.read_text(encoding="utf-8")) == json.loads(completed.stdout)


def test_final_runner_subprocess_blocking_exit_one_writes_no_file(runner_binaries, tmp_path):
    final, _ = runner_binaries
    output_path = tmp_path / "must-not-exist.json"
    preview_model = json.loads(PREVIEW_FIXTURE.read_text(encoding="utf-8"))
    body = {
        "request": final_runner_request("solve"),
        "solve": {"preview_model": {"model": preview_model, "materials": []}},
    }
    completed = run_final(final, "solve", body, "--output", str(output_path))
    assert completed.returncode == 1
    controlled = json.loads(completed.stdout)
    assert controlled["blocked"] is True
    assert controlled["payload"] is None
    assert not output_path.exists()


def test_final_runner_subprocess_preserves_export_results_stub(runner_binaries):
    final, _ = runner_binaries
    body = {"request": final_runner_request("export_results")}
    completed = run_final(final, "export-results", body, "--explicit-local-private-intent")
    assert completed.returncode == 1
    controlled = json.loads(completed.stdout)
    assert controlled["blocked"] is False
    assert any(
        diagnostic["code"] == "HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD"
        for diagnostic in controlled["payload"]["diagnostics"]
    )


def test_final_runner_subprocess_malformed_input_exits_two(runner_binaries):
    final, _ = runner_binaries
    completed = subprocess.run(
        [str(final), "validate-input", "--explicit-local-private-intent"],
        input="{",
        capture_output=True,
        text=True,
        check=False,
    )
    assert completed.returncode == 2
    controlled = json.loads(completed.stdout)
    assert controlled["payload"]["command"] == "validate-input"
    assert any(
        diagnostic["code"] == "HEADLESS_RUNNER_CLI_INPUT_JSON_INVALID"
        for diagnostic in controlled["payload"]["diagnostics"]
    )


def test_compatibility_runner_subprocess_controlled_stdout_and_exits(runner_binaries):
    _, compat = runner_binaries
    allowed = subprocess.run(
        [str(compat), str(PREVIEW_FIXTURE), "--explicit-local-private-intent"],
        capture_output=True,
        text=True,
        check=False,
    )
    assert allowed.returncode == 0, (allowed.stderr, allowed.stdout)
    allowed_controlled = json.loads(allowed.stdout)
    assert allowed_controlled["blocked"] is False
    assert "runner_result" in allowed_controlled["payload"]

    blocked = subprocess.run(
        [str(compat), str(PREVIEW_FIXTURE)],
        capture_output=True,
        text=True,
        check=False,
    )
    assert blocked.returncode == 1
    blocked_controlled = json.loads(blocked.stdout)
    assert blocked_controlled["blocked"] is True
    assert blocked_controlled["payload"] is None
    assert "runner_result" not in blocked_controlled

    usage = subprocess.run(
        [str(compat)],
        capture_output=True,
        text=True,
        check=False,
    )
    assert usage.returncode == 2
    assert usage.stdout == ""
    assert "usage (PROVISIONAL" in usage.stderr


if __name__ == "__main__":
    main()
