#!/usr/bin/env python3
"""Stdlib checks for the headless runner contract."""

import json
import hashlib
import io
import sys
from pathlib import Path
import subprocess
import zipfile

import pytest

if str(Path(__file__).resolve().parent) not in sys.path:
    sys.path.insert(0, str(Path(__file__).resolve().parent))

from schema_validation import (  # noqa: E402
    enum_at,
    load_schema,
    required_at,
    walk_keys,
)


ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "schemas" / "headless_runner.schema.yaml"
FINAL_RUNNER_PATH = ROOT / "core" / "runner" / "headless" / "src" / "bin" / "openpipestress-runner.rs"
COMPAT_RUNNER_PATH = ROOT / "core" / "runner" / "headless" / "src" / "bin" / "headless_preview_runner.rs"
HEADLESS_CRATE = ROOT / "core" / "runner" / "headless"
PRODUCT_PHYSICS_CRATE = ROOT / "core" / "product_physics"
# Bundled legacy demo with nonzero legacy pressure loads. The current product
# refuses it with PRESSURE_MODEL_REAUTHOR_REQUIRED; it is retained only as the
# explicit refusal control below and is never used as a success-path input.
LEGACY_PRESSURE_PREVIEW_FIXTURE = (
    ROOT / "fixtures" / "product_preview" / "invented_preview_model.json"
)
# Synthetic, explicitly authored model (E/nu material, exact pressure region
# with closure transfers, six-component load case) that the current product
# solves. Success-path solve inputs use it unchanged.
AUTHORED_PREVIEW_MODEL = (
    ROOT / "fixtures" / "model_operations" / "exact_pressure_authoring_model.json"
)
# Solver identity recorded in the DEL-10-05 export-results witness inputs; it
# was the linked product-physics component version before the 0.2.0 change.
PRIOR_PRODUCT_PHYSICS_VERSION = "0.1.0"
EXPORT_SUCCESS_INPUT = (
    ROOT / "validation/witness/inputs/del1005_export_results_success_input.json"
)
EXPORT_MISSING_INPUT = (
    ROOT / "validation/witness/inputs/del1005_export_results_missing_payload_input.json"
)
EXPORT_MISMATCH_INPUT = (
    ROOT / "validation/witness/inputs/del1005_export_results_binding_mismatch_input.json"
)
EXPORT_BLOCKED_INPUT = (
    ROOT / "validation/witness/inputs/del1005_export_results_producer_blocked_input.json"
)

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


def main():
    schema = load_schema(SCHEMA_PATH)
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
    assert 'args.verb != "export-results" || code == 0' in final_source
    assert "&& !controlled.blocked" in final_source
    assert "args.explicit_local_private_intent" in final_source
    assert "HEADLESS_RUNNER_EXPORT_RESULTS_PAYLOAD_MISSING" in final_source
    assert "HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD" not in final_source

    assert compat_source.index("control_local_private(") < compat_source.index(
        'println!("{rendered}")'
    )
    assert "[--explicit-local-private-intent]" in compat_source
    assert "clean && !controlled.blocked" in compat_source


def cargo_target_debug_directory(crate):
    metadata = subprocess.run(
        ["cargo", "metadata", "--format-version", "1", "--no-deps"],
        cwd=crate,
        capture_output=True,
        text=True,
        check=False,
    )
    assert metadata.returncode == 0, metadata.stderr
    return Path(json.loads(metadata.stdout)["target_directory"]) / "debug"


def test_cargo_target_debug_directory_honors_cargo_target_dir(monkeypatch, tmp_path):
    redirected_target = tmp_path / "cargo-target"
    monkeypatch.setenv("CARGO_TARGET_DIR", str(redirected_target))

    assert cargo_target_debug_directory(HEADLESS_CRATE) == redirected_target / "debug"


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
    target = cargo_target_debug_directory(HEADLESS_CRATE)
    final = target / "openpipestress-runner"
    compat = target / "headless_preview_runner"
    assert final.is_file()
    assert compat.is_file()
    return final, compat


@pytest.fixture(scope="module")
def linked_solver_identity():
    """Name and version the runner links as its product-physics component.

    The runner binds export-results to `solver_component_name()` and
    `solver_component_version()`, which are that crate's CARGO_PKG_NAME and
    CARGO_PKG_VERSION; read the same values from the crate's Cargo metadata.
    """
    metadata = subprocess.run(
        ["cargo", "metadata", "--format-version", "1", "--no-deps"],
        cwd=PRODUCT_PHYSICS_CRATE,
        capture_output=True,
        text=True,
        check=False,
    )
    assert metadata.returncode == 0, metadata.stderr
    (package,) = json.loads(metadata.stdout)["packages"]
    assert package["name"] == "open_pipe_stress_product_physics"
    return package["name"], package["version"]


def bind_solver_identity(body, name, version, *, audit=True, envelopes=True):
    """Set the export-results solver identity; the witness payload is otherwise unchanged."""
    export = body.get("export_results")
    if export is None:
        return body
    identity = {
        "solver_name": name,
        "solver_version": version,
        "solver_build_ref": f"{name}@{version}",
    }
    if audit:
        export["audit_manifest"]["solver_version"].update(identity)
    if envelopes:
        for envelope in export["result_envelopes"]:
            envelope.update(identity)
    return body


def export_input(path, linked_solver_identity):
    """Load a DEL-10-05 export-results witness bound to the linked solver identity.

    The witness files record the solver identity of the component they were
    generated against. The runner correctly refuses any other identity
    (REPORT-PACKAGE-SOLVER-IDENTITY-MISMATCH; see the identity control test),
    so export-results tests that protect other properties bind the payload to
    the component actually linked into the runner under test.
    """
    body = json.loads(path.read_text(encoding="utf-8"))
    return bind_solver_identity(body, *linked_solver_identity)


def authored_preview_model():
    return json.loads(AUTHORED_PREVIEW_MODEL.read_text(encoding="utf-8"))


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
    preview_model = authored_preview_model()
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


def test_final_runner_subprocess_covers_every_active_verb_and_stdout(
    runner_binaries, linked_solver_identity
):
    final, _ = runner_binaries
    for verb, operation, extra in active_final_runner_cases():
        body = {"request": final_runner_request(operation), **extra}
        completed = run_final(final, verb, body, "--explicit-local-private-intent")
        assert completed.returncode == 0, (verb, completed.stderr, completed.stdout)
        controlled = json.loads(completed.stdout)
        assert controlled["blocked"] is False
        assert controlled["payload"]["command"] == verb
        assert controlled["payload"]["operation"] == operation
        if verb == "solve":
            mechanics = controlled["payload"]["mechanics_envelope"]
            assert mechanics["status"]["mechanics"] == "MECHANICS_SOLVED"
            assert mechanics["results"]
            producer = mechanics["producer"]
            assert (
                producer["component_name"],
                producer["component_version"],
            ) == linked_solver_identity


def legacy_nonzero_pressure_loads(model):
    return {
        (case["id"], load["id"])
        for case in model["load_cases"]
        for load in case["primitive_loads"]
        if (load["category"] == "pressure" or load["dimension"] == "pressure")
        and load["magnitude"]["value"] != 0
    }


def assert_legacy_pressure_refusal(mechanics, runner_result, model):
    """The product refuses a fresh solve of a legacy nonzero-pressure model."""
    assert mechanics["status"]["mechanics"] == "MODEL_INCOMPLETE"
    assert mechanics["results"] == []
    refusals = [
        diagnostic
        for diagnostic in mechanics["diagnostics"]
        if diagnostic["code"] == "PRESSURE_MODEL_REAUTHOR_REQUIRED"
    ]
    assert all(diagnostic["severity"] == "blocking" for diagnostic in refusals)
    assert all(
        diagnostic["source"] == "core/product_physics/src/pressure_runtime.rs"
        for diagnostic in refusals
    )
    expected = legacy_nonzero_pressure_loads(model)
    assert expected, "the legacy control must carry nonzero legacy pressure"
    assert {tuple(diagnostic["affected_refs"]) for diagnostic in refusals} == expected
    assert "MODEL_INCOMPLETE" in runner_result["analysis_status"]
    assert "MECHANICS_SOLVED" not in runner_result["analysis_status"]
    assert runner_result["result_refs"] == []


def test_final_runner_refuses_legacy_pressure_demo_as_explicit_control(runner_binaries):
    final, _ = runner_binaries
    legacy_model = json.loads(LEGACY_PRESSURE_PREVIEW_FIXTURE.read_text(encoding="utf-8"))
    body = {
        "request": final_runner_request("solve"),
        "solve": {"preview_model": {"model": legacy_model, "materials": []}},
    }
    completed = run_final(final, "solve", body, "--explicit-local-private-intent")
    assert completed.returncode == 1, (completed.stderr, completed.stdout)
    controlled = json.loads(completed.stdout)
    # Explicit intent was given, so the refusal itself is visible, not withheld.
    assert controlled["blocked"] is False
    payload = controlled["payload"]
    assert payload["command"] == "solve"
    assert payload["operation"] == "solve"
    assert_legacy_pressure_refusal(
        payload["mechanics_envelope"], payload["runner_result"], legacy_model
    )
    # Exit 1 comes from result validation: a refused solve has no result rows.
    assert any(
        diagnostic["code"] == "HEADLESS_RUNNER_RESULT_REFS_MISSING"
        and diagnostic["severity"] == "blocking"
        for diagnostic in payload["result_validation"]["diagnostics"]
    )


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
    # A model the product solves, so exit 1 is attributable to the missing
    # local-private intent alone rather than to a refused solve.
    preview_model = authored_preview_model()
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


def test_final_runner_subprocess_export_results_success_is_deterministic_and_zip_exact(
    runner_binaries, linked_solver_identity
):
    final, _ = runner_binaries
    body = export_input(EXPORT_SUCCESS_INPUT, linked_solver_identity)
    first = run_final(
        final, "export-results", body, "--explicit-local-private-intent"
    )
    second = run_final(
        final, "export-results", body, "--explicit-local-private-intent"
    )
    assert first.returncode == second.returncode == 0, (first.stderr, second.stderr)
    assert first.stdout == second.stdout
    controlled = json.loads(first.stdout)
    assert controlled["blocked"] is False
    package = controlled["payload"]["report_package"]
    package_bytes = bytes(package["container_bytes"])
    assert hashlib.sha256(package_bytes).hexdigest() == package["container_sha256_hex"]
    assert len(
        [
            decision
            for decision in controlled["decisions"]
            if decision["path"] == "$.report_package"
        ]
    ) == 1
    assert len(
        [
            finding
            for finding in controlled["findings"]
            if finding["path"] == "$.report_package"
        ]
    ) == 1
    with zipfile.ZipFile(io.BytesIO(package_bytes)) as archive:
        assert archive.namelist() == [member["file_name"] for member in package["members"]]
        for member in package["members"]:
            member_bytes = archive.read(member["file_name"])
            assert len(member_bytes) == member["byte_length"]
            assert hashlib.sha256(member_bytes).hexdigest() == member["sha256_hex"]


def test_final_runner_export_results_output_is_only_named_json_file(
    runner_binaries, linked_solver_identity, tmp_path
):
    final, _ = runner_binaries
    body = export_input(EXPORT_SUCCESS_INPUT, linked_solver_identity)
    output = tmp_path / "runner-result.json"
    completed = run_final(
        final,
        "export-results",
        body,
        "--output",
        str(output),
        "--explicit-local-private-intent",
    )
    assert completed.returncode == 0, completed.stderr
    assert [path.name for path in tmp_path.iterdir()] == ["runner-result.json"]
    assert json.loads(output.read_text(encoding="utf-8")) == json.loads(completed.stdout)


@pytest.mark.parametrize(
    ("input_path", "diagnostic_code"),
    [
        (EXPORT_MISSING_INPUT, "HEADLESS_RUNNER_EXPORT_RESULTS_PAYLOAD_MISSING"),
        (EXPORT_MISMATCH_INPUT, "HEADLESS_RUNNER_EXPORT_RESULTS_BINDING_MISMATCH"),
        (EXPORT_BLOCKED_INPUT, "HEADLESS_RUNNER_EXPORT_RESULTS_PACKAGE_BLOCKED"),
    ],
)
def test_final_runner_export_results_failures_have_no_payload_or_file(
    runner_binaries, linked_solver_identity, tmp_path, input_path, diagnostic_code
):
    final, _ = runner_binaries
    output = tmp_path / "must-not-exist.json"
    body = export_input(input_path, linked_solver_identity)
    completed = run_final(
        final,
        "export-results",
        body,
        "--output",
        str(output),
        "--explicit-local-private-intent",
    )
    assert completed.returncode == 1
    controlled = json.loads(completed.stdout)
    assert "report_package" not in controlled["payload"]
    assert any(
        diagnostic["code"] == diagnostic_code
        for diagnostic in controlled["payload"]["diagnostics"]
    )
    assert not output.exists()


def test_final_runner_export_results_requires_intent_once_and_writes_no_file(
    runner_binaries, linked_solver_identity, tmp_path
):
    final, _ = runner_binaries
    output = tmp_path / "must-not-exist.json"
    body = export_input(EXPORT_SUCCESS_INPUT, linked_solver_identity)
    completed = run_final(
        final, "export-results", body, "--output", str(output)
    )
    assert completed.returncode == 1
    controlled = json.loads(completed.stdout)
    assert controlled["blocked"] is True
    assert controlled["payload"] is None
    package_decisions = [
        decision
        for decision in controlled["decisions"]
        if decision["path"] == "$.report_package"
    ]
    package_findings = [
        finding
        for finding in controlled["findings"]
        if finding["path"] == "$.report_package"
    ]
    assert len(package_decisions) == len(package_findings) == 1
    assert package_decisions[0]["reason_code"] == "LOCAL_PRIVATE_INTENT_REQUIRED"
    assert not output.exists()


def test_final_runner_export_results_invalid_wire_payload_preserves_report_code(
    runner_binaries, linked_solver_identity
):
    final, _ = runner_binaries
    body = export_input(EXPORT_SUCCESS_INPUT, linked_solver_identity)
    body["export_results"]["audit_manifest"]["model_hash"]["value"] = "ABC"
    completed = run_final(
        final, "export-results", body, "--explicit-local-private-intent"
    )
    assert completed.returncode == 1
    controlled = json.loads(completed.stdout)
    assert "report_package" not in controlled["payload"]
    diagnostic = next(
        item
        for item in controlled["payload"]["diagnostics"]
        if item["code"] == "HEADLESS_RUNNER_EXPORT_RESULTS_PAYLOAD_INVALID"
    )
    assert "REPORT-PACKAGE-SHA256-INVALID" in diagnostic["message"]


@pytest.mark.parametrize(
    ("stale_audit", "stale_envelopes", "message_detail"),
    [
        (True, False, "request identity does not match"),
        (False, True, "result envelope identity does not match"),
    ],
)
def test_final_runner_export_results_refuses_unlinked_solver_identity(
    runner_binaries,
    linked_solver_identity,
    tmp_path,
    stale_audit,
    stale_envelopes,
    message_detail,
):
    final, _ = runner_binaries
    name, linked_version = linked_solver_identity
    assert linked_version != PRIOR_PRODUCT_PHYSICS_VERSION
    body = export_input(EXPORT_SUCCESS_INPUT, linked_solver_identity)
    bind_solver_identity(
        body,
        name,
        PRIOR_PRODUCT_PHYSICS_VERSION,
        audit=stale_audit,
        envelopes=stale_envelopes,
    )
    output = tmp_path / "must-not-exist.json"
    completed = run_final(
        final,
        "export-results",
        body,
        "--output",
        str(output),
        "--explicit-local-private-intent",
    )
    assert completed.returncode == 1
    controlled = json.loads(completed.stdout)
    assert controlled["blocked"] is False
    assert "report_package" not in controlled["payload"]
    diagnostic = next(
        item
        for item in controlled["payload"]["diagnostics"]
        if item["code"] == "HEADLESS_RUNNER_EXPORT_RESULTS_PAYLOAD_INVALID"
    )
    assert "REPORT-PACKAGE-SOLVER-IDENTITY-MISMATCH" in diagnostic["message"]
    assert message_detail in diagnostic["message"]
    assert not output.exists()


@pytest.mark.parametrize(
    ("mutate", "serde_detail"),
    [
        (
            lambda payload: payload["export_results"].pop("package_id"),
            "missing field `package_id`",
        ),
        (
            lambda payload: payload["export_results"].__setitem__(
                "source_basis_refs", "not-an-array"
            ),
            "invalid type: string",
        ),
    ],
)
def test_final_runner_export_results_wire_deserialization_failures_exit_one(
    runner_binaries, linked_solver_identity, tmp_path, mutate, serde_detail
):
    final, _ = runner_binaries
    body = export_input(EXPORT_SUCCESS_INPUT, linked_solver_identity)
    mutate(body)
    output = tmp_path / "must-not-exist.json"
    completed = run_final(
        final,
        "export-results",
        body,
        "--output",
        str(output),
        "--explicit-local-private-intent",
    )
    assert completed.returncode == 1
    controlled = json.loads(completed.stdout)
    assert "report_package" not in controlled["payload"]
    diagnostic = next(
        item
        for item in controlled["payload"]["diagnostics"]
        if item["code"] == "HEADLESS_RUNNER_EXPORT_RESULTS_PAYLOAD_INVALID"
    )
    assert diagnostic["message"].startswith("REPORT-PACKAGE-WIRE-INCOMPLETE: ")
    assert serde_detail in diagnostic["message"]
    assert not output.exists()


def test_final_runner_export_results_native_size_is_exact_with_constant_cardinality(
    runner_binaries, linked_solver_identity
):
    final, _ = runner_binaries
    body = export_input(EXPORT_SUCCESS_INPUT, linked_solver_identity)
    body["export_results"]["state_comparison_handoff_records"][0][
        "invented_padding"
    ] = "x" * 3_200_000
    completed = run_final(
        final, "export-results", body, "--explicit-local-private-intent"
    )
    assert completed.returncode == 0, completed.stderr
    controlled = json.loads(completed.stdout)
    package = controlled["payload"]["report_package"]
    package_bytes = bytes(package["container_bytes"])
    assert len(package_bytes) >= 3_189_621
    assert hashlib.sha256(package_bytes).hexdigest() == package["container_sha256_hex"]
    assert (
        sum(
            decision["path"] == "$.report_package"
            for decision in controlled["decisions"]
        )
        == 1
    )
    assert (
        sum(
            finding["path"] == "$.report_package"
            for finding in controlled["findings"]
        )
        == 1
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
        [str(compat), str(AUTHORED_PREVIEW_MODEL), "--explicit-local-private-intent"],
        capture_output=True,
        text=True,
        check=False,
    )
    assert allowed.returncode == 0, (allowed.stderr, allowed.stdout)
    allowed_controlled = json.loads(allowed.stdout)
    assert allowed_controlled["blocked"] is False
    assert "runner_result" in allowed_controlled["payload"]
    assert (
        allowed_controlled["payload"]["mechanics_envelope"]["status"]["mechanics"]
        == "MECHANICS_SOLVED"
    )

    # The same solvable model, so exit 1 is attributable to the missing
    # local-private intent alone rather than to a refused solve.
    blocked = subprocess.run(
        [str(compat), str(AUTHORED_PREVIEW_MODEL)],
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


def test_compatibility_runner_refuses_legacy_pressure_demo_as_explicit_control(
    runner_binaries,
):
    _, compat = runner_binaries
    legacy_model = json.loads(LEGACY_PRESSURE_PREVIEW_FIXTURE.read_text(encoding="utf-8"))
    refused = subprocess.run(
        [
            str(compat),
            str(LEGACY_PRESSURE_PREVIEW_FIXTURE),
            "--explicit-local-private-intent",
        ],
        capture_output=True,
        text=True,
        check=False,
    )
    assert refused.returncode == 1, (refused.stderr, refused.stdout)
    controlled = json.loads(refused.stdout)
    # Explicit intent was given, so the refusal itself is visible, not withheld.
    assert controlled["blocked"] is False
    payload = controlled["payload"]
    assert_legacy_pressure_refusal(
        payload["mechanics_envelope"], payload["runner_result"], legacy_model
    )


if __name__ == "__main__":
    main()
