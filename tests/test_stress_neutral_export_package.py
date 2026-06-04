#!/usr/bin/env python3
"""Focused tests for DEL-17-06 stress-neutral CSV/JSON export packages."""

from __future__ import annotations

from copy import deepcopy
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

from core.handoff.stress_neutral import (  # noqa: E402
    build_stress_neutral_export_package,
    canonical_json,
    render_stress_neutral_csv,
    write_stress_neutral_export_package,
)
from schema_validation import (  # noqa: E402
    JsonSchemaDependencyMissing,
    validate_instance,
    validate_schema_document,
)


SCHEMA_PATH = ROOT / "schemas" / "stress_neutral_export.schema.json"
FIXTURE_PATH = ROOT / "fixtures" / "stress_neutral" / "invented" / "stress_neutral_export_package.json"
CSV_FIXTURE_PATH = ROOT / "fixtures" / "stress_neutral" / "invented" / "stress_neutral_results.csv"
SOURCE_PAYLOAD_PATH = ROOT / "fixtures" / "stress_neutral" / "invented" / "source_result_payload.json"
SHA256_PATTERN = re.compile(r"^sha256:[a-f0-9]{64}$")

CSV_HEADER = (
    "result_id,canonical_ref,row_kind,result_family,load_case_ref,station_ref,"
    "component_ref,value,unit,dimension,correlation_status"
)

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


def source_payload() -> dict[str, object]:
    return {
        "export_id": "stress-neutral:invented-del-17-06",
        "source_result_ref": ref("ResultEnvelope", "result-envelope:invented-del-17-06"),
        "source_run_ref": ref("AnalysisRun", "run:invented-del-17-06"),
        "source_model_ref": ref("Model", "model:invented-del-17-06"),
        "source_hashes": [
            {
                "algorithm": "sha256",
                "canonicalization": "JCS_compatible_json_payload_hash",
                "payload_ref": ref("ResultEnvelope", "result-envelope:invented-del-17-06"),
                "payload_scope": "source_result_envelope",
                "value": "sha256:" + "2" * 64,
            }
        ],
        "result_rows": [
            {
                "result_id": "result:invented:force:P001:axial",
                "canonical_ref": ref("Result", "result:invented:force:P001:axial"),
                "row_kind": "result_value",
                "result_family": "force",
                "load_case_ref": ref("LoadCase", "load:invented:operating"),
                "station_ref": ref("Station", "station:invented:P001:end-i"),
                "component_ref": ref("PipeElement", "pipe:invented:001"),
                "value": 12.5,
                "unit": "N",
                "dimension": "force",
                "source_result_ref": ref("Result", "result:invented:force:P001:axial"),
                "correlation_status": "canonical_id_map",
            },
            {
                "result_id": "result:invented:stress:P001:sustained",
                "canonical_ref": ref("Result", "result:invented:stress:P001:sustained"),
                "row_kind": "result_value",
                "result_family": "stress",
                "load_case_ref": ref("LoadCase", "load:invented:sustained"),
                "station_ref": ref("Station", "station:invented:P001:midspan"),
                "component_ref": ref("PipeElement", "pipe:invented:001"),
                "value": 3450.0,
                "unit": "Pa",
                "dimension": "stress",
                "source_result_ref": ref("Result", "result:invented:stress:P001:sustained"),
                "correlation_status": "canonical_id_map",
            },
        ],
        "stable_id_map": [
            {
                "canonical_ref": ref("Result", "result:invented:force:P001:axial"),
                "export_ref": ref("StressNeutralRow", "result:invented:force:P001:axial"),
                "mapping_status": "mapped",
                "loss_category": "exported",
            },
            {
                "canonical_ref": ref("Result", "result:invented:stress:P001:sustained"),
                "export_ref": ref("StressNeutralRow", "result:invented:stress:P001:sustained"),
                "mapping_status": "mapped",
                "loss_category": "exported",
            },
        ],
        "loss_report": [
            {
                "loss_id": "loss:invented:stress-neutral-exported",
                "category": "exported",
                "severity": "info",
                "affected_refs": [ref("ResultEnvelope", "result-envelope:invented-del-17-06")],
                "target_artifact_ref": ref("StressNeutralExportPackage", "stress-neutral:invented-del-17-06"),
                "reason": "Invented result rows exported to stress-neutral CSV/JSON for deterministic package testing.",
                "source_basis_ref": ref("Deliverable", "DEL-17-02"),
                "downstream_implication": "Does not define comparison pass/fail or professional reliance semantics.",
            }
        ],
    }


def build_from_source() -> dict[str, object]:
    return build_stress_neutral_export_package(**source_payload())


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
    fixture = load_json(FIXTURE_PATH)
    built = build_from_source()
    try:
        assert validate_schema_document(schema, schema_label=str(SCHEMA_PATH))
        assert validate_instance(schema, fixture, schema_label=str(SCHEMA_PATH), instance_label=str(FIXTURE_PATH))
        assert validate_instance(
            schema,
            built,
            schema_label=str(SCHEMA_PATH),
            instance_label="build_stress_neutral_export_package output",
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


def test_builder_is_deterministic_and_preserves_members():
    first = build_from_source()
    second = build_from_source()

    assert canonical_json(first) == canonical_json(second)
    assert first["deliverable_id"] == "DEL-17-06"
    assert first["package_status"] == "stress_neutral_export_package"
    assert first["export_profile"]["comparison_semantics"] == "diagnostic_export_only_no_pass_fail"
    assert first["export_profile"]["unit_policy"] == "unit_and_dimension_required_per_row"
    member_roles = {item["member_role"] for item in first["manifest"]["package_members"]}
    assert member_roles == {
        "manifest",
        "csv_text",
        "result_rows",
        "stable_id_map",
        "loss_report",
        "validation_report",
        "diagnostics",
    }
    assert all(SHA256_PATTERN.match(item["value"]) for item in first["manifest"]["checksums"])
    assert not [item for item in first["diagnostics"] if item["severity"] == "blocking"]


def test_csv_and_json_rows_are_synchronized_and_ascii_safe():
    package = build_from_source()
    text = package["csv_text"]

    assert text == render_stress_neutral_csv(package["result_rows"])
    assert text == CSV_FIXTURE_PATH.read_text(encoding="ascii")
    assert text.splitlines()[0] == CSV_HEADER
    text.encode("ascii")
    assert "result:invented:force:P001:axial" in text
    assert "result:invented:stress:P001:sustained" in text
    assert {row["unit"] for row in package["result_rows"]} == {"N", "Pa"}
    assert {row["dimension"] for row in package["result_rows"]} == {"force", "stress"}


def test_missing_units_stable_ids_and_loss_report_are_blocking():
    payload = source_payload()
    rows = deepcopy(payload["result_rows"])
    rows[0]["unit"] = ""

    package = build_stress_neutral_export_package(
        export_id="stress-neutral:blocked",
        source_result_ref=payload["source_result_ref"],
        source_run_ref=payload["source_run_ref"],
        source_model_ref=payload["source_model_ref"],
        source_hashes=payload["source_hashes"],
        result_rows=rows,
        stable_id_map=[],
        loss_report=[],
    )

    codes = {item["code"] for item in package["diagnostics"]}
    assert {"SN-UNIT-DIMENSION-MISSING", "SN-STABLE-ID-MAP-MISSING", "SN-LOSS-REPORT-MISSING"} <= codes
    assert any(item["severity"] == "blocking" for item in package["diagnostics"])
    assert package["validation_report"]["validation_status"] == "blocked"


def test_privacy_and_authority_boundary_diagnostics_are_blocking():
    payload = source_payload()
    package = build_stress_neutral_export_package(
        export_id="stress-neutral:privacy-boundary",
        source_result_ref=payload["source_result_ref"],
        source_run_ref=payload["source_run_ref"],
        source_model_ref=payload["source_model_ref"],
        source_hashes=payload["source_hashes"],
        result_rows=payload["result_rows"],
        stable_id_map=payload["stable_id_map"],
        loss_report=payload["loss_report"],
        export_profile={"free_metadata": {"unsafe_label": "cert" + "ified comparison export"}},
        privacy={"protected_payload_embedded": True},
    )

    codes = {item["code"] for item in package["diagnostics"]}
    assert {"SN-PRIVACY-BOUNDARY-VIOLATION", "SN-PROFILE-AUTHORITY-TERM"} <= codes
    assert any(item["severity"] == "blocking" for item in package["diagnostics"])
    assert package["professional_boundary"]["software_makes_compliance_claim"] is False
    assert package["professional_boundary"]["software_creates_professional_reliance_record"] is False


def test_source_basis_refs_cover_result_export_run_and_comparison_contracts():
    package = build_from_source()
    refs = {
        (item["object_type"], item["ref"])
        for item in package["export_profile"]["source_basis_refs"]
    }

    assert {
        ("Deliverable", "DEL-08-04"),
        ("Deliverable", "DEL-14-02"),
        ("Deliverable", "DEL-14-05"),
        ("Deliverable", "DEL-17-02"),
    } <= refs

    payload = source_payload()
    unsafe = build_stress_neutral_export_package(
        export_id="stress-neutral:missing-source-basis",
        source_result_ref=payload["source_result_ref"],
        source_run_ref=payload["source_run_ref"],
        source_model_ref=payload["source_model_ref"],
        source_hashes=payload["source_hashes"],
        result_rows=payload["result_rows"],
        stable_id_map=payload["stable_id_map"],
        loss_report=payload["loss_report"],
        export_profile={"source_basis_refs": [ref("Deliverable", "DEL-17-02")]},
    )

    assert "SN-SOURCE-BASIS-REFS-MISSING" in {item["code"] for item in unsafe["diagnostics"]}
    assert unsafe["validation_report"]["validation_status"] == "blocked"


def test_writer_outputs_csv_and_sidecars(tmp_path):
    package = build_from_source()

    write_stress_neutral_export_package(tmp_path, package)

    assert (tmp_path / "stress_neutral_results.csv").read_text(encoding="ascii") == package["csv_text"]
    assert load_json(tmp_path / "manifest.json") == package["manifest"]
    assert load_json(tmp_path / "result_rows.json") == package["result_rows"]
    assert load_json(tmp_path / "stable_id_map.json") == package["stable_id_map"]
    assert load_json(tmp_path / "loss_report.json") == package["loss_report"]
    assert load_json(tmp_path / "validation_report.json") == package["validation_report"]
    assert load_json(tmp_path / "diagnostics.json") == package["diagnostics"]


def test_fixtures_contain_no_private_or_protected_payload_text():
    text = "\n".join(
        walk_strings(
            {
                "package": load_json(FIXTURE_PATH),
                "source": load_json(SOURCE_PAYLOAD_PATH),
                "csv": CSV_FIXTURE_PATH.read_text(encoding="ascii"),
            }
        )
    ).lower()
    for forbidden in FORBIDDEN_PAYLOAD_TEXT:
        assert forbidden not in text


def main():
    check_jsonschema_validation()
    test_builder_is_deterministic_and_preserves_members()
    test_csv_and_json_rows_are_synchronized_and_ascii_safe()
    test_missing_units_stable_ids_and_loss_report_are_blocking()
    test_privacy_and_authority_boundary_diagnostics_are_blocking()
    test_source_basis_refs_cover_result_export_run_and_comparison_contracts()
    test_fixtures_contain_no_private_or_protected_payload_text()


if __name__ == "__main__":
    main()
