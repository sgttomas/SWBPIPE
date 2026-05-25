#!/usr/bin/env python3
"""Focused tests for DEL-17-05 CAEPIPE external-run evidence packages."""

from __future__ import annotations

import json
from pathlib import Path
import re
import sys


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))
TESTS_DIR = ROOT / "tests"
if str(TESTS_DIR) not in sys.path:
    sys.path.insert(0, str(TESTS_DIR))

from core.handoff.caepipe_external import (  # noqa: E402
    build_caepipe_external_run_package,
    build_skipped_caepipe_external_run_package,
    canonical_json,
    parse_caepipe_csv_text,
    write_caepipe_external_run_package,
)
from schema_validation import (  # noqa: E402
    JsonSchemaDependencyMissing,
    validate_instance,
    validate_schema_document,
)


SCHEMA_PATH = ROOT / "schemas" / "caepipe_external_run.schema.json"
CSV_FIXTURE_PATH = ROOT / "fixtures" / "caepipe_external" / "invented" / "caepipe_results.csv"
SKIPPED_FIXTURE_PATH = ROOT / "fixtures" / "caepipe_external" / "invented" / "skipped_run_package.json"
PARSED_FIXTURE_PATH = ROOT / "fixtures" / "caepipe_external" / "invented" / "parser_only_run_package.json"
SHA256_PATTERN = re.compile(r"^sha256:[a-f0-9]{64}$")

FORBIDDEN_PAYLOAD_TEXT = {
    "real client",
    "asme table",
    "b31j",
    "cert" + "ified by openpipestress",
    "code " + "compliant",
    "professional " + "acceptance",
}


def ref(object_type: str, value: str) -> dict[str, str]:
    return {"object_type": object_type, "ref": value}


def load_json(path: Path) -> dict[str, object]:
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def stable_id_map() -> list[dict[str, object]]:
    return [
        {
            "canonical_ref": ref("Node", "node:invented:A"),
            "target_ref": ref("CaePipeMbfRecord", "N001"),
        },
        {
            "canonical_ref": ref("Node", "node:invented:B"),
            "target_ref": ref("CaePipeMbfRecord", "N002"),
        },
        {
            "canonical_ref": ref("PipeElement", "pipe:invented:001"),
            "target_ref": ref("CaePipeMbfRecord", "P001"),
        },
    ]


def mbf_package_ref() -> dict[str, str]:
    return ref("CaePipeMbfExportPackage", "caepipe-mbf:invented-del-17-04")


def parsed_csv() -> dict[str, object]:
    return parse_caepipe_csv_text(
        CSV_FIXTURE_PATH.read_text(encoding="ascii"),
        source_csv_ref=ref("CsvArtifact", "csv:invented-caepipe-results"),
        stable_id_map=stable_id_map(),
    )


def skipped_package() -> dict[str, object]:
    return build_skipped_caepipe_external_run_package(
        run_id="caepipe-run:invented-skipped",
        mbf_package_ref=mbf_package_ref(),
    )


def parser_only_package() -> dict[str, object]:
    return build_caepipe_external_run_package(
        run_id="caepipe-run:invented-parser-only",
        package_status="parser_only_evidence",
        mbf_package_ref=mbf_package_ref(),
        executable_config={
            "configuration_surface": "TBD-17-05-PH-001",
            "configured_path_state": "absent",
            "license_responsibility_acknowledged": False,
            "environment_responsibility_acknowledged": False,
            "path_record": "not_configured",
        },
        command_profile={
            "profile_id": "TBD-17-05-invocation-profile",
            "invocation_mode": "parser_only_not_invoked",
            "command_shape": "not_invoked_public_fixture_only",
            "profile_basis": "TBD-17-05-PH-001",
        },
        run_directory={
            "run_directory_ref": "fixture:caepipe-external-parser-only",
            "working_directory": "fixtures/caepipe_external/invented",
            "input_mbf_path": "fixtures/caepipe_mbf/invented/caepipe_mbf_export_package.json",
            "expected_csv_path": "fixtures/caepipe_external/invented/caepipe_results.csv",
            "observed_csv_path": "fixtures/caepipe_external/invented/caepipe_results.csv",
            "output_discovery_status": "fixture_observed",
        },
        execution_result={
            "attempted": False,
            "exit_status": None,
            "stdout_capture": "not_available",
            "stderr_capture": "not_available",
            "skip_reason": "Parser-only public fixture; external CAEPIPE execution was not attempted.",
        },
        parsed_csv=parsed_csv(),
    )


def walk_strings(value):
    if isinstance(value, str):
        yield value
    elif isinstance(value, dict):
        for item in value.values():
            yield from walk_strings(item)
    elif isinstance(value, list):
        for item in value:
            yield from walk_strings(item)


def check_jsonschema_validation():
    schema = load_json(SCHEMA_PATH)
    skipped = load_json(SKIPPED_FIXTURE_PATH)
    parser_only = load_json(PARSED_FIXTURE_PATH)
    try:
        assert validate_schema_document(schema, schema_label=str(SCHEMA_PATH))
        for label, instance in (
            ("skipped fixture", skipped),
            ("parser-only fixture", parser_only),
            ("skipped builder output", skipped_package()),
            ("parser-only builder output", parser_only_package()),
        ):
            assert validate_instance(
                schema,
                instance,
                schema_label=str(SCHEMA_PATH),
                instance_label=label,
            )
    except JsonSchemaDependencyMissing as exc:
        _skip_or_note_missing_jsonschema(exc)


def _skip_or_note_missing_jsonschema(exc):
    if "pytest" in sys.modules:
        import pytest

        pytest.skip(str(exc))
    print(f"SKIP: {exc}")


def test_fixture_and_builder_validate_against_schema():
    check_jsonschema_validation()


def test_skipped_without_executable_is_nonblocking_and_deterministic():
    first = skipped_package()
    second = skipped_package()

    assert canonical_json(first) == canonical_json(second)
    assert first["package_status"] == "skipped_no_executable"
    assert first["execution_result"]["attempted"] is False
    assert first["executable_config"]["configured_path_state"] == "absent"
    assert not [item for item in first["diagnostics"] if item["severity"] == "blocking"]
    assert all(SHA256_PATTERN.match(item["value"]) for item in first["checksums"])
    assert first["professional_boundary"]["software_bundles_caepipe"] is False
    assert first["professional_boundary"]["software_invokes_caepipe_without_user_configuration"] is False


def test_parser_only_package_binds_rows_to_sidecar_stable_ids():
    package = parser_only_package()

    assert package["package_status"] == "parser_only_evidence"
    assert package["parsed_csv"]["row_count"] == 3
    assert {row["correlation_status"] for row in package["parsed_csv"]["rows"]} == {"canonical_id_map"}
    assert {row["section"] for row in package["parsed_csv"]["rows"]} == {
        "ELEMENT_FORCES",
        "NODE_DISPLACEMENTS",
    }
    assert not [item for item in package["diagnostics"] if item["severity"] == "blocking"]
    assert package["professional_boundary"]["software_makes_caepipe_compatibility_claim"] is False
    assert package["professional_boundary"]["software_creates_professional_reliance_record"] is False


def test_parser_records_unknown_sections_and_unmapped_rows_as_warnings():
    text = (
        CSV_FIXTURE_PATH.read_text(encoding="ascii")
        + "UNKNOWN_SECTION,node:invented:missing,X999,L001,0,0,0,,,,m\n"
    )
    parsed = parse_caepipe_csv_text(
        text,
        source_csv_ref=ref("CsvArtifact", "csv:invented-unknown-row"),
        stable_id_map=stable_id_map(),
    )

    codes = {item["code"] for item in parsed["diagnostics"]}
    assert {"CAEPIPE-CSV-UNKNOWN-SECTION", "CAEPIPE-CSV-UNMAPPED-ROW"} <= codes
    assert parsed["parser_status"] == "parsed_with_diagnostics"
    assert any(row["correlation_status"] == "weak_or_unmapped" for row in parsed["rows"])


def test_privacy_and_authority_boundary_diagnostics_are_blocking():
    package = build_caepipe_external_run_package(
        run_id="caepipe-run:privacy-boundary",
        package_status="parser_only_evidence",
        mbf_package_ref=mbf_package_ref(),
        executable_config={
            "configured_path_state": "absent",
            "path_record": "not_configured",
        },
        command_profile={
            "command_shape": "not_invoked",
            "free_metadata": {"unsafe_label": "cert" + "ified target run"},
        },
        run_directory={
            "run_directory_ref": "fixture:privacy",
            "working_directory": "fixtures",
            "input_mbf_path": "model.mbf",
            "expected_csv_path": "results.csv",
            "observed_csv_path": "results.csv",
            "output_discovery_status": "fixture_observed",
        },
        execution_result={"attempted": False},
        parsed_csv=parsed_csv(),
        privacy={"protected_payload_embedded": True},
    )

    codes = {item["code"] for item in package["diagnostics"]}
    assert {"CAEPIPE-RUN-PRIVACY-BOUNDARY-VIOLATION", "CAEPIPE-RUN-AUTHORITY-TERM"} <= codes
    assert any(item["severity"] == "blocking" for item in package["diagnostics"])


def test_writer_outputs_run_metadata_and_sidecars(tmp_path):
    package = parser_only_package()

    write_caepipe_external_run_package(tmp_path, package)

    metadata = load_json(tmp_path / "run_metadata.json")
    assert metadata["run_id"] == package["run_id"]
    assert "parsed_csv" not in metadata
    assert load_json(tmp_path / "parsed_csv.json") == package["parsed_csv"]
    assert load_json(tmp_path / "diagnostics.json") == package["diagnostics"]
    assert load_json(tmp_path / "checksums.json") == package["checksums"]


def test_fixtures_contain_no_private_or_protected_payload_text():
    text = "\n".join(
        walk_strings(
            {
                "skipped": load_json(SKIPPED_FIXTURE_PATH),
                "parser_only": load_json(PARSED_FIXTURE_PATH),
                "csv": CSV_FIXTURE_PATH.read_text(encoding="ascii"),
            }
        )
    ).lower()
    for forbidden in FORBIDDEN_PAYLOAD_TEXT:
        assert forbidden not in text


def main():
    check_jsonschema_validation()
    test_skipped_without_executable_is_nonblocking_and_deterministic()
    test_parser_only_package_binds_rows_to_sidecar_stable_ids()
    test_parser_records_unknown_sections_and_unmapped_rows_as_warnings()
    test_privacy_and_authority_boundary_diagnostics_are_blocking()
    test_fixtures_contain_no_private_or_protected_payload_text()


if __name__ == "__main__":
    main()
