#!/usr/bin/env python3
"""Focused tests for DEL-17-06 stress-neutral CSV/JSON export packages."""

from __future__ import annotations

from copy import deepcopy
import hashlib
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
    CANONICALIZATION_LABEL,
    build_stress_neutral_export_package,
    canonical_json,
    render_stress_neutral_csv,
    write_stress_neutral_export_package,
    build_stress_neutral_export_package_v0_2,
    materialized_members_v0_2,
    validate_stress_neutral_export_package_v0_2,
)
from schema_validation import (  # noqa: E402
    JsonSchemaDependencyMissing,
    _skip_or_note_missing_jsonschema,
    validate_instance,
    validate_schema_document,
    walk_strings,
)


SCHEMA_PATH = ROOT / "schemas" / "stress_neutral_export.schema.json"
FIXTURE_PATH = ROOT / "fixtures" / "stress_neutral" / "invented" / "stress_neutral_export_package.json"
CSV_FIXTURE_PATH = ROOT / "fixtures" / "stress_neutral" / "invented" / "stress_neutral_results.csv"
SOURCE_PAYLOAD_PATH = ROOT / "fixtures" / "stress_neutral" / "invented" / "source_result_payload.json"
V02_FIXTURE_PATH = ROOT / "fixtures" / "stress_neutral" / "invented" / "stress_neutral_export_package_v0_2.json"
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
                "canonicalization": "deterministic_sorted_compact_json_payload_hash",
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
        "unresolved_assumption_refs": [
            ref("Assumption", "assumption:invented-result-review"),
        ],
        "reproducibility_refs": [
            ref("AnalysisRun", "run:invented-del-17-06"),
            ref("AuditManifest", "audit-manifest:invented-del-17-06"),
        ],
    }


def build_from_source() -> dict[str, object]:
    return build_stress_neutral_export_package(**source_payload())


def source_payload_830() -> dict[str, object]:
    payload = source_payload()
    template = payload["result_rows"][0]
    rows, stable = [], []
    for index in range(830):
        row = deepcopy(template)
        row["result_id"] = f"result:stress-neutral:{index:03d}"
        row["canonical_ref"] = ref("Result", row["result_id"])
        row["source_result_ref"] = ref("Result", row["result_id"])
        row["value"] = float(index)
        if index >= 828:
            row["row_kind"] = "diagnostic_work"
            row["result_family"] = "diagnostic_work"
        rows.append(row)
        stable.append({"canonical_ref": ref("Result", row["result_id"]), "export_ref": ref("StressNeutralRow", row["result_id"]), "mapping_status": "mapped", "loss_category": "exported"})
    payload["result_rows"] = rows
    payload["stable_id_map"] = stable
    return payload


def test_v02_preserves_830_rows_and_materializes_exact_nine_members():
    source = source_payload_830()
    received = deepcopy(source["source_hashes"])
    package = build_stress_neutral_export_package_v0_2(**source)
    assert package["schema_version"] == "0.2.0"
    assert len(package["result_rows"]) == 830
    assert len(package["stable_id_map"]) == 830
    assert len(package["unit_preservation_witnesses"]) == 828
    assert sum(item["code"] == "SN-UNIT-WITNESS-WITHHELD-DIAGNOSTIC-WORK" for item in package["diagnostics"]) == 2
    assert package["received_source_checksums"] == received
    members = materialized_members_v0_2(package)
    assert list(members) == ["manifest.json", "stress_neutral_results.csv", "result_rows.json", "unit_system_disclosure.json", "unit_preservation_witnesses.json", "stable_id_map.json", "loss_report.json", "validation_report.json", "diagnostics.json"]
    assert all(not data.endswith(b"\n") for name, data in members.items() if name.endswith(".json"))
    assert next(item for item in package["manifest"]["checksums"] if item["payload_ref"]["ref"] == "manifest.json")["payload_scope"] == "manifest_seed"
    validate_stress_neutral_export_package_v0_2(package)
    validate_instance(load_json(SCHEMA_PATH), package, schema_label=str(SCHEMA_PATH), instance_label="strict stress-neutral 0.2 package")


def test_v02_rejects_member_and_package_tamper():
    package = build_stress_neutral_export_package_v0_2(**source_payload())
    package["result_rows"][0]["value"] = 999
    try:
        validate_stress_neutral_export_package_v0_2(package)
    except ValueError as error:
        assert "MEMBER-CHECKSUM-MISMATCH" in str(error)
    else:
        raise AssertionError("member tamper must fail")


def test_v02_fixture_matches_builder_and_dispatch_schema():
    fixture = load_json(V02_FIXTURE_PATH)
    assert fixture == build_stress_neutral_export_package_v0_2(**source_payload())
    validate_instance(load_json(SCHEMA_PATH), fixture, schema_label=str(SCHEMA_PATH), instance_label=str(V02_FIXTURE_PATH))


def test_v02_schema_rejects_unknown_and_missing_fields_across_repaired_families():
    schema = load_json(SCHEMA_PATH)
    valid = build_stress_neutral_export_package_v0_2(**source_payload())
    diagnostic_source = source_payload()
    diagnostic_source["result_rows"][0]["row_kind"] = "diagnostic_work"
    diagnostic_source["result_rows"][0]["result_family"] = "diagnostic_work"
    with_diagnostic = build_stress_neutral_export_package_v0_2(**diagnostic_source)
    cases = []
    item = deepcopy(valid); item["export_profile"]["unknown"] = True; cases.append(item)
    item = deepcopy(valid); del item["result_rows"][0]["unit"]; cases.append(item)
    item = deepcopy(valid); item["unit_system_disclosure"]["unknown"] = True; cases.append(item)
    item = deepcopy(valid); del item["stable_id_map"][0]["mapping_status"]; cases.append(item)
    item = deepcopy(valid); del item["loss_report"][0]["reason"]; cases.append(item)
    item = deepcopy(valid); item["validation_report"]["unknown"] = True; cases.append(item)
    item = deepcopy(with_diagnostic); item["diagnostics"][0]["unknown"] = True; cases.append(item)
    item = deepcopy(valid); item["provenance"]["unknown"] = True; cases.append(item)
    item = deepcopy(valid); item["received_source_checksums"][0]["unknown"] = True; cases.append(item)
    item = deepcopy(valid); item["privacy"]["unknown"] = True; cases.append(item)
    item = deepcopy(valid); item["professional_boundary"]["unknown"] = True; cases.append(item)
    for case in cases:
        try:
            validate_instance(schema, case, schema_label=str(SCHEMA_PATH), instance_label="negative strict stress-neutral package")
        except AssertionError:
            continue
        raise AssertionError("strict stress-neutral schema accepted an unknown or missing family field")


def walk_mappings(value):
    if isinstance(value, dict):
        yield value
        for item in value.values():
            yield from walk_mappings(item)
    elif isinstance(value, list):
        for item in value:
            yield from walk_mappings(item)


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
        "unit_system_disclosure",
        "stable_id_map",
        "loss_report",
        "validation_report",
        "diagnostics",
    }
    assert first["unit_system_disclosure"]["unit_system_ref"]["ref"] == "unit-system:dec-018-si-dual-display"
    assert first["unit_system_disclosure"]["result_units"] == ["N", "Pa"]
    assert first["unit_system_disclosure"]["conversion_performed"] is False
    assert first["unit_system_disclosure"]["protected_content_included"] is False
    assert all(SHA256_PATTERN.match(item["value"]) for item in first["manifest"]["checksums"])
    assert not [item for item in first["diagnostics"] if item["severity"] == "blocking"]


def test_json_hash_contract_is_exact_deterministic_and_not_jcs():
    sample = {"z": "café", "a": [1, {"b": True}]}
    expected = b'{"a":[1,{"b":true}],"z":"caf\\u00e9"}'
    assert canonical_json(sample).encode("ascii") == expected
    assert hashlib.sha256(expected).hexdigest() == (
        "9fe3dcafeafda5780fb9062a482c657fdd139cb98c1d4e4356e7123b64665753"
    )

    package = build_from_source()
    json_checksums = [
        item
        for item in walk_mappings(package)
        if item.get("algorithm") == "sha256"
        and item.get("canonicalization") != "normalized_ascii_lf_text"
    ]
    assert json_checksums
    assert {item["canonicalization"] for item in json_checksums} == {
        CANONICALIZATION_LABEL
    }
    assert "JCS" not in canonical_json(package)

    mutated = source_payload()
    mutated["result_rows"][0]["value"] = 13.5
    changed = build_stress_neutral_export_package(**mutated)
    original_hashes = {item["value"] for item in package["manifest"]["checksums"]}
    changed_hashes = {item["value"] for item in changed["manifest"]["checksums"]}
    assert changed_hashes != original_hashes


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


def test_assumption_and_reproducibility_refs_pass_through_to_package_and_manifest():
    payload = source_payload()
    package = build_stress_neutral_export_package(**payload)

    for key in ("unresolved_assumption_refs", "reproducibility_refs"):
        assert package[key] == payload[key]
        assert package["manifest"][key] == payload[key]

    payload["unresolved_assumption_refs"][0]["ref"] = "mutated-after-build"
    assert package["unresolved_assumption_refs"][0]["ref"] == (
        "assumption:invented-result-review"
    )
    assert package["professional_boundary"]["software_creates_professional_reliance_record"] is False


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

    controlled = write_stress_neutral_export_package(tmp_path, package)

    assert controlled.blocked is True
    assert controlled.summary["materialization_withheld"] is True
    assert list(tmp_path.iterdir()) == []


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
