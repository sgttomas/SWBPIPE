"""Strict precision-1 stress-neutral 0.3 package construction.

The 0.1 builder remains byte-stable in ``package.py``. This module decorates
its normalized data into the versioned nine-member checked-hash contract.
"""
from __future__ import annotations
from copy import deepcopy
import csv
from decimal import Decimal, InvalidOperation
import io
import json
import math
from pathlib import Path
import re
from typing import Any, Mapping

from core.serialization.canonical_json.adapter import canonical_json_checked_v1, canonical_sha256_checked_v1
from .package import build_stress_neutral_export_package, canonical_csv
from core.analysis_runs.compatibility import (
    _source_contract, _semantic, validate_analysis_run_v0_3, PRECISION_CONTRACT_ID,
    PRECISION_CONTRACT_SHA256,
)

VERSION = "0.3.0"
PROFILE = "openpipestress_jcs_ijson_v1"
EXPORT_PROFILE = "ops.stress_neutral.v3"
MEMBERS = ["manifest.json", "stress_neutral_results.csv", "result_rows.json", "unit_system_disclosure.json", "unit_preservation_witnesses.json", "stable_id_map.json", "loss_report.json", "validation_report.json", "diagnostics.json"]
SEMANTIC_CONTRACT_REF = {"object_type": "ExternalReference", "ref": "fixtures/results/semantic_contract_v0_3_precision_1.json"}
WITHHOLDING_CODES = {
    "diagnostic_work": "SN-UNIT-WITNESS-WITHHELD-DIAGNOSTIC-WORK",
    "unknown_semantic": "SN-UNIT-WITNESS-WITHHELD-UNKNOWN-SEMANTIC",
    "missing_semantic": "SN-UNIT-WITNESS-WITHHELD-MISSING-SEMANTIC",
    "contradiction": "SN-UNIT-WITNESS-WITHHELD-CONTRADICTION",
}
FAMILY_DIMENSIONS = {
    "displacement": "length", "rotation": "angle", "reaction": "force", "force": "force",
    "moment": "moment", "stress": "stress", "ratio": "ratio", "solver_mode": "dimensionless",
    "discrete": "dimensionless",
}
UNIT_DIMENSIONS = {
    "mm": "length", "m": "length", "rad": "angle", "N": "force", "N*m": "moment",
    "Pa": "stress", "MPa": "stress", "ratio": "ratio", "mode_code": "dimensionless",
    "count": "dimensionless", "boolean": "dimensionless", "state_code": "dimensionless",
    "unitless": "dimensionless", "record": "dimensionless", "N/m": "linear_stiffness",
    "N*m/rad": "rotational_stiffness",
}
CSV_COLUMNS = [
    "result_id", "canonical_ref", "row_kind", "result_family", "load_case_ref",
    "station_ref", "component_ref", "value", "unit", "dimension", "correlation_status",
]
JSON_NUMBER = re.compile(r"-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?\Z")
MAX_SAFE_INTEGER = (1 << 53) - 1


def _checked(payload: Any, filename: str, scope: str = "member_payload") -> dict[str, Any]:
    return {"algorithm": "sha256", "canonicalization": PROFILE, "payload_scope": scope, "payload_ref": {"object_type": "StressNeutralMember", "ref": filename}, "value": canonical_sha256_checked_v1(payload)}


def _witness_disposition(row: Mapping[str, Any]) -> tuple[str, str | None]:
    if row.get("row_kind") in {"diagnostic_work", "work"} or row.get("result_family") == "diagnostic_work":
        return "diagnostic_work", None
    family = row.get("result_family")
    if not isinstance(family, str):
        return "unknown_semantic", None
    unit_dimension = UNIT_DIMENSIONS.get(row.get("unit"))
    if unit_dimension is None:
        return "missing_semantic", None
    if family == "reaction":
        interpreted = {"N": "force", "N*m": "moment"}.get(row.get("unit"))
        if interpreted is None:
            return "contradiction", None
    elif family in FAMILY_DIMENSIONS:
        interpreted = FAMILY_DIMENSIONS[family]
    elif family == "other":
        interpreted = row.get("dimension")
        if not isinstance(interpreted, str) or interpreted in {"", "TBD"}:
            return "missing_semantic", None
    else:
        return "unknown_semantic", None
    if unit_dimension != interpreted or row.get("dimension") != interpreted or row.get("correlation_status") != "canonical_id_map":
        return "contradiction", None
    return "eligible", interpreted


def _csv_scalar(row: Mapping[str, Any], column: str) -> Any:
    if column == "value":
        return row.get(column)
    if column in {"canonical_ref", "load_case_ref", "station_ref", "component_ref"}:
        ref = row.get(column)
        return ref.get("ref", "") if isinstance(ref, Mapping) else ""
    return row.get(column, "")


def _supported_binary64(text: str, expected: Any) -> bool:
    if isinstance(expected, bool) or not isinstance(expected, (int, float)):
        return False
    if not JSON_NUMBER.fullmatch(text):
        return False
    try:
        decimal_value = Decimal(text)
        parsed = float(text)
    except (InvalidOperation, OverflowError, ValueError):
        return False
    if not decimal_value.is_finite() or not math.isfinite(parsed):
        return False
    if not decimal_value.is_zero() and parsed == 0.0:
        return False
    expected_float = float(expected)
    if not math.isfinite(expected_float):
        return False
    if parsed.is_integer() and abs(parsed) > MAX_SAFE_INTEGER:
        return False
    return parsed == expected_float


def _validate_received_csv(csv_text: Any, rows: list[Mapping[str, Any]]) -> None:
    if not isinstance(csv_text, str) or csv_text != canonical_csv(csv_text):
        raise ValueError("SN-CSV-ROW-BINDING-MISMATCH")
    try:
        csv_text.encode("ascii")
    except UnicodeEncodeError as error:
        raise ValueError("SN-CSV-ROW-BINDING-MISMATCH") from error
    try:
        records = list(csv.reader(io.StringIO(csv_text, newline=""), strict=True))
        if not records or records[0] != CSV_COLUMNS:
            raise ValueError("SN-CSV-ROW-BINDING-MISMATCH")
        if any(len(record) != len(CSV_COLUMNS) for record in records):
            raise ValueError("SN-CSV-ROW-BINDING-MISMATCH")
        received_rows = [dict(zip(CSV_COLUMNS, record)) for record in records[1:]]
    except (csv.Error, UnicodeError) as error:
        raise ValueError("SN-CSV-ROW-BINDING-MISMATCH") from error
    if len(received_rows) != len(rows):
        raise ValueError("SN-CSV-ROW-BINDING-MISMATCH")
    received_ids: set[str] = set()
    for received, expected in zip(received_rows, rows):
        result_id = received["result_id"]
        if not result_id or result_id in received_ids:
            raise ValueError("SN-CSV-ROW-BINDING-MISMATCH")
        received_ids.add(result_id)
        for column in CSV_COLUMNS:
            if column == "value":
                if not _supported_binary64(received[column], _csv_scalar(expected, column)):
                    raise ValueError("SN-CSV-ROW-BINDING-MISMATCH")
            elif received[column] != str(_csv_scalar(expected, column)):
                raise ValueError("SN-CSV-ROW-BINDING-MISMATCH")


def _witnesses(rows: list[dict[str, Any]], provenance: Mapping[str, Any], source: Mapping[str, Any] | None = None) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    witnesses, findings = [], []
    source_rows = {item.get("id"): item for item in source.get("results", [])} if source is not None else {}
    table = _source_contract(source)[2] if source is not None else None
    for index, row in enumerate(rows):
        disposition, interpreted_dimension = _witness_disposition(row)
        if source is not None:
            semantic, _ = _semantic(source_rows[row["result_id"]], table)
            if semantic is not None and semantic.get("category") == "diagnostic_work":
                disposition, interpreted_dimension = "diagnostic_work", None
            elif semantic is None:
                disposition, interpreted_dimension = "unknown_semantic", None
        if disposition != "eligible":
            diagnostic_work = disposition == "diagnostic_work"
            findings.append({"code": WITHHOLDING_CODES[disposition], "class": "unit_preservation_witness", "severity": "info" if diagnostic_work else "blocking", "source": {"object_type": "StressNeutralResultRow", "ref": str(row.get("result_id"))}, "affected_object": {"object_type": "StressNeutralUnitWitness", "ref": f"unit-witness:{index}"}, "message": "Diagnostic work evidence is retained as a row but has no physical unit-preservation witness." if diagnostic_work else f"{disposition.replace('_', ' ')} prevents a unit-preservation witness; the received row remains retained without a physical interpretation claim.", "remediation": "Review diagnostic work separately from physical quantity witnesses." if diagnostic_work else "Resolve the source family, unit, dimension and correlation evidence before downstream physical interpretation.", "provenance": deepcopy(dict(provenance))})
            continue
        quantity = {"value": row.get("value"), "unit": row.get("unit"), "dimension": interpreted_dimension}
        witnesses.append({"witness_id": f"unit-witness:{index}", "source_row_index": index, "result_id": row.get("result_id"), "source_quantity": quantity, "target_quantity": deepcopy(quantity), "conversion_performed": False, "policy": "preserve_received_value_and_unit"})
    if findings:
        findings.append({"code": "SN-DECLARED-DIMENSION-WITNESS-UNAVAILABLE", "class": "export_blocking", "severity": "blocking", "source": {"object_type": "ExportConsumer", "ref": "DEL-17-06"}, "affected_object": {"object_type": "StressNeutralResultRows", "ref": "stress-neutral:result-rows"}, "message": f"{len(findings)} retained rows are explicitly categorized as ineligible for unit-preservation witnesses; {len(witnesses)} rows have accepted semantic-contract interpretations and exact value/unit witnesses.", "remediation": "Review every explicit witness-withholding finding before downstream use.", "provenance": deepcopy(dict(provenance))})
    return witnesses, findings


def _manifest_seed(package: Mapping[str, Any], member_checksums: list[dict[str, Any]]) -> dict[str, Any]:
    return {
        "manifest_id": package["manifest"]["manifest_id"], "source_result_ref": package["source_result_ref"], "source_run_ref": package["source_run_ref"], "source_model_ref": package["source_model_ref"],
        "received_source_checksums": package["received_source_checksums"], "unresolved_assumption_refs": package["unresolved_assumption_refs"], "reproducibility_refs": package["reproducibility_refs"],
        "export_profile_ref": package["manifest"]["export_profile_ref"], "boundary_notes": package["manifest"]["boundary_notes"], "member_checksums": member_checksums, "diagnostics": package["diagnostics"],
    }


def package_projection(package: Mapping[str, Any]) -> dict[str, Any]:
    projection = deepcopy(dict(package))
    projection.pop("package_checksum", None)
    return projection


def source_row_projection_v0_3(source: Mapping[str, Any], row: Mapping[str, Any]) -> dict[str, Any]:
    """Source-qualified physical identity; never infer meaning from unit alone."""
    _, _, table = _source_contract(source)
    semantic, _ = _semantic(row, table)
    family = (semantic.get("family") or "other") if semantic and semantic.get("category") == "physical_quantity" else "other"
    dimension = semantic.get("derivative_target_dimension") if semantic else None
    dimension = dimension or "TBD"
    basis = row.get("basis_ref")
    basis_ref = {"object_type": "AnalysisRun", "ref": source.get("run_id")}
    if isinstance(basis, Mapping):
        basis_ref = {"object_type": {"load_case": "LoadCase", "combination": "Combination"}.get(basis.get("ref_type"), "ResultBasis"), "ref": basis.get("ref_id")}
    entity = row.get("entity_ref")
    if not isinstance(entity, str) or not entity:
        raise ValueError("SN-PRECISION-ROW-ENTITY-MISSING")
    entity_type = {"pipe": "PipeElement", "node": "Node", "support": "Support", "component": "Component", "material": "Material"}.get(entity.split(":", 1)[0] if ":" in entity else "", "CanonicalObject")
    metadata = row.get("metadata") if isinstance(row.get("metadata"), Mapping) else {}
    location = metadata.get("location")
    return {
        "result_id": row.get("id"), "canonical_ref": {"object_type": "Result", "ref": row.get("id")},
        "row_kind": "result_value", "result_family": family, "load_case_ref": basis_ref,
        "station_ref": {"object_type": "Station", "ref": "summary" if location is None else location},
        "component_ref": {"object_type": entity_type, "ref": entity}, "value": row.get("value"),
        "unit": row.get("unit"), "dimension": dimension,
        "source_result_ref": {"object_type": "Result", "ref": row.get("id")},
        "correlation_status": "canonical_id_map" if row.get("unit") and dimension != "TBD" else "unit_or_dimension_blocking_review_required",
    }


def _validate_source_rows(source: Mapping[str, Any], rows: list[Mapping[str, Any]]) -> None:
    raw = source.get("results", [])
    if len(rows) != len(raw) or len({row.get("id") for row in raw}) != len(raw) or len({row.get("result_id") for row in rows}) != len(rows):
        raise ValueError("SN-PRECISION-ROW-SOURCE-MISMATCH")
    expected = {row.get("id"): source_row_projection_v0_3(source, row) for row in raw}
    for row in rows:
        projection = expected.get(row.get("result_id"))
        if projection is None or any(key not in row or row[key] != value for key, value in projection.items()):
            raise ValueError("SN-PRECISION-ROW-SOURCE-MISMATCH")


def build_stress_neutral_export_package_v0_3(*, source_envelope: Mapping[str, Any], analysis_record: Mapping[str, Any], expected_basis_refs: list[Mapping[str, str]] | None = None, **source: Any) -> dict[str, Any]:
    validate_analysis_run_v0_3(analysis_record, source_envelope, expected_basis_refs=expected_basis_refs)
    received = [item for item in analysis_record["analysis_run"]["hashes"] if item.get("payload_scope") == "received_result"]
    if len(received) != 1 or received[0] not in source.get("source_hashes", []) or source.get("source_result_ref") != received[0]["payload_ref"] or source.get("source_run_ref") != {"object_type":"AnalysisRun", "ref":source_envelope["run_id"]} or source.get("source_model_ref", {}).get("ref") != source_envelope["model_ref"]:
        raise ValueError("SN-PRECISION-SOURCE-BINDING-MISMATCH")
    _validate_source_rows(source_envelope, source.get("result_rows", []))
    supplied_profile = source.get("export_profile")
    if isinstance(supplied_profile, Mapping):
        if "profile_id" in supplied_profile and supplied_profile["profile_id"] != EXPORT_PROFILE:
            raise ValueError("SN-0.3-PROFILE-IDENTITY-MISMATCH")
        if "profile_version" in supplied_profile and supplied_profile["profile_version"] != VERSION:
            raise ValueError("SN-0.3-PROFILE-IDENTITY-MISMATCH")
    legacy = build_stress_neutral_export_package(**deepcopy(source))
    witnesses, witness_findings = _witnesses(legacy["result_rows"], legacy["provenance"], source_envelope)
    diagnostics = [deepcopy(item) for item in legacy["diagnostics"] if item.get("code") not in {"SN-DECLARED-DIMENSION-WITNESS-UNAVAILABLE", "SN-UNIT-DIMENSION-MISSING"}] + witness_findings
    blocking_count = sum(item.get("severity") == "blocking" for item in diagnostics)
    decision_basis_refs = deepcopy(legacy["unit_system_disclosure"]["decision_basis_refs"]) + [deepcopy(SEMANTIC_CONTRACT_REF), deepcopy(legacy["source_run_ref"])]
    boundary_notes = list(legacy["manifest"]["boundary_notes"]) + ["Result-row dimensions and witness eligibility are interpreted from the accepted semantic contract and bound analysis run; received numerical values, units, rows and source hashes remain unchanged."]
    export_profile = {**legacy["export_profile"], "profile_id": EXPORT_PROFILE, "profile_version": VERSION, "boundary_notes": boundary_notes, "source_basis_refs": deepcopy(legacy["export_profile"]["source_basis_refs"]) + [deepcopy(SEMANTIC_CONTRACT_REF), deepcopy(legacy["source_run_ref"])]}
    validation_report = {"validation_status": "blocked" if blocking_count else "passed", "checks": [{"check_id": "stress-neutral-boundary-diagnostics", "check_status": "blocking" if blocking_count else "passed", "diagnostic_count": len(diagnostics), "blocking_count": blocking_count, "provenance": deepcopy(legacy["provenance"])}], "human_review_required": True, "provenance": deepcopy(legacy["provenance"])}
    loss_report = deepcopy(legacy["loss_report"])
    withholding_count = len(witness_findings) - (1 if witness_findings else 0)
    for entry in loss_report:
        if entry.get("category") == "exported":
            entry["reason"] = f"All {len(legacy['result_rows'])} received numerical rows and units are retained unchanged; {len(witnesses)} rows have accepted semantic-contract dimension witnesses and {withholding_count} rows have explicit witness-withholding findings."
    package: dict[str, Any] = {
        "schema_version": VERSION, "deliverable_id": "DEL-17-06", "package_id": "PKG-17", "scope_items": ["SOW-046", "SOW-074"], "objectives": ["OBJ-007", "OBJ-017", "OBJ-018"],
        "export_id": legacy["export_id"], "package_status": "stress_neutral_export_package", "schema_conformant": True,
        "validation_ready": not any(item.get("severity") == "blocking" for item in diagnostics),
        "source_result_ref": legacy["source_result_ref"], "source_run_ref": legacy["source_run_ref"], "source_model_ref": legacy["source_model_ref"],
        "received_source_checksums": deepcopy(source.get("source_hashes", [])), "unresolved_assumption_refs": legacy["unresolved_assumption_refs"], "reproducibility_refs": legacy["reproducibility_refs"],
        "export_profile": export_profile, "manifest": {"manifest_id": legacy["manifest"]["manifest_id"], "export_profile_ref": {"object_type": "StressNeutralExportProfile", "ref": EXPORT_PROFILE}, "boundary_notes": boundary_notes, "package_members": [], "checksums": []},
        "csv_text": legacy["csv_text"], "result_rows": legacy["result_rows"], "unit_system_disclosure": {**legacy["unit_system_disclosure"], "decision_basis_refs": decision_basis_refs}, "unit_preservation_witnesses": witnesses,
        "stable_id_map": legacy["stable_id_map"], "loss_report": loss_report, "validation_report": validation_report, "diagnostics": diagnostics,
        "privacy": legacy["privacy"], "provenance": legacy["provenance"], "professional_boundary": legacy["professional_boundary"],
    }
    for key in ("producer", "numerical_quality", "formulation_basis"):
        package[key] = deepcopy(source_envelope[key])
    package["semantic_contract_ref"] = {"ref_type":"semantic_contract", "ref_id":PRECISION_CONTRACT_ID}
    package["semantic_contract"] = {"id":PRECISION_CONTRACT_ID, "sha256":PRECISION_CONTRACT_SHA256}
    package["source_carrier_checksum"] = deepcopy(received[0])
    payloads = {"stress_neutral_results.csv": package["csv_text"], "result_rows.json": package["result_rows"], "unit_system_disclosure.json": package["unit_system_disclosure"], "unit_preservation_witnesses.json": package["unit_preservation_witnesses"], "stable_id_map.json": package["stable_id_map"], "loss_report.json": package["loss_report"], "validation_report.json": package["validation_report"], "diagnostics.json": package["diagnostics"]}
    checksums = []
    for filename, payload in payloads.items():
        if filename.endswith(".csv"):
            import hashlib
            checksum = {"algorithm": "sha256", "canonicalization": "normalized_ascii_lf_text", "payload_scope": "member_bytes", "payload_ref": {"object_type": "StressNeutralMember", "ref": filename}, "value": hashlib.sha256(payload.encode("ascii")).hexdigest()}
        else:
            checksum = _checked(payload, filename)
        checksums.append(checksum)
    seed_checksum = _checked(_manifest_seed(package, checksums), "manifest.json", "manifest_seed")
    package["manifest"]["checksums"] = checksums + [seed_checksum]
    package["manifest"]["package_members"] = [{"filename": filename, "checksum": next(item for item in package["manifest"]["checksums"] if item["payload_ref"]["ref"] == filename)} for filename in MEMBERS]
    package["package_checksum"] = {"algorithm": "sha256", "canonicalization": PROFILE, "payload_scope": "complete_package_excluding_self_checksum", "payload_ref": {"object_type": "StressNeutralExportPackage", "ref": package["export_id"]}, "value": canonical_sha256_checked_v1(package_projection(package))}
    validate_stress_neutral_export_package_v0_3(package, source_envelope=source_envelope, analysis_record=analysis_record, expected_basis_refs=expected_basis_refs)
    return package


# The transport envelope is not the in-memory manifest and is never included in
# package_projection. Keeping the original manifest separate avoids self-reference.
_MEMBER_FIELDS = {
    "stress_neutral_results.csv": "csv_text", "result_rows.json": "result_rows",
    "unit_system_disclosure.json": "unit_system_disclosure",
    "unit_preservation_witnesses.json": "unit_preservation_witnesses",
    "stable_id_map.json": "stable_id_map", "loss_report.json": "loss_report",
    "validation_report.json": "validation_report", "diagnostics.json": "diagnostics",
}


def materialized_members_v0_3(package: Mapping[str, Any]) -> dict[str, bytes]:
    metadata = {key: deepcopy(value) for key, value in package.items()
                if key not in set(_MEMBER_FIELDS.values()) | {"manifest"}}
    transport = {"transport_version": VERSION, "manifest": deepcopy(package["manifest"]),
                 "package_metadata": metadata}
    values = {"manifest.json": transport, **{name: package[field] for name, field in _MEMBER_FIELDS.items()}}
    return {name: (value.encode("ascii") if name.endswith(".csv") else canonical_json_checked_v1(value).encode("utf-8")) for name, value in values.items()}


def _transport_json(payload: bytes) -> Any:
    def unique_object(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
        result = {}
        for key, value in pairs:
            if key in result:
                raise ValueError("SN-TRANSPORT-JSON-DUPLICATE-KEY")
            result[key] = value
        return result

    def reject_constant(value: str) -> None:
        raise ValueError("SN-TRANSPORT-JSON-NONFINITE")

    return json.loads(payload, object_pairs_hook=unique_object, parse_constant=reject_constant)


def reconstruct_materialized_members_v0_3(members: Mapping[str, bytes], **validation: Any) -> dict[str, Any]:
    """Reconstruct the exact in-memory package before checksum/source validation."""
    if set(members) != set(MEMBERS):
        raise ValueError("SN-MEMBER-INVENTORY-MISMATCH")
    transport = _transport_json(members["manifest.json"])
    if not isinstance(transport, dict) or set(transport) != {"transport_version", "manifest", "package_metadata"} or transport["transport_version"] != VERSION or not isinstance(transport["manifest"], dict) or not isinstance(transport["package_metadata"], dict):
        raise ValueError("SN-TRANSPORT-MANIFEST-INVALID")
    package = deepcopy(transport["package_metadata"])
    if set(package) & (set(_MEMBER_FIELDS.values()) | {"manifest"}):
        raise ValueError("SN-TRANSPORT-METADATA-COLLISION")
    package["manifest"] = transport["manifest"]
    for name, field in _MEMBER_FIELDS.items():
        package[field] = members[name].decode("ascii") if name.endswith(".csv") else _transport_json(members[name])
    validate_stress_neutral_export_package_v0_3(package, **validation)
    return package


def read_materialized_members_v0_3(input_dir: str | Path, **validation: Any) -> dict[str, Any]:
    directory = Path(input_dir)
    if {entry.name for entry in directory.iterdir()} != set(MEMBERS) or any(not (directory / name).is_file() for name in MEMBERS):
        raise ValueError("SN-MEMBER-INVENTORY-MISMATCH")
    return reconstruct_materialized_members_v0_3({name: (directory / name).read_bytes() for name in MEMBERS}, **validation)


def write_materialized_members_v0_3(package: Mapping[str, Any], output_dir: str | Path) -> list[Path]:
    """Validate and write the exact nine contracted member byte streams."""
    validate_stress_neutral_export_package_v0_3(package)
    destination = Path(output_dir)
    destination.mkdir(parents=True, exist_ok=True)
    written = []
    for filename, payload in materialized_members_v0_3(package).items():
        target = destination / filename
        target.write_bytes(payload)
        written.append(target)
    return written


def validate_stress_neutral_export_package_v0_3(package: Mapping[str, Any], *, source_envelope: Mapping[str, Any] | None = None, analysis_record: Mapping[str, Any] | None = None, expected_basis_refs: list[Mapping[str, str]] | None = None) -> None:
    """Check packet claims/hash; supplied source enables additional binding checks.

    Neither mode authenticates a producer or qualifies Current numerical use.
    """
    metadata_source = {"schema_version":"0.2.0", **{key: package.get(key) for key in ("producer", "numerical_quality", "formulation_basis")}}
    _source_contract(metadata_source)
    if package.get("semantic_contract") != {"id":PRECISION_CONTRACT_ID, "sha256":PRECISION_CONTRACT_SHA256} or package.get("semantic_contract_ref") != {"ref_type":"semantic_contract", "ref_id":PRECISION_CONTRACT_ID}:
        raise ValueError("SN-PRECISION-SEMANTIC-CONTRACT-MISMATCH")
    carrier = package.get("source_carrier_checksum", {})
    if carrier not in package.get("received_source_checksums", []) or carrier.get("payload_ref") != package.get("source_result_ref") or carrier.get("payload_scope") != "received_result" or carrier.get("algorithm") != "sha256" or carrier.get("canonicalization") != PROFILE:
        raise ValueError("SN-PRECISION-CARRIER-BINDING-MISMATCH")
    if source_envelope is not None:
        _source_contract(source_envelope)
        if any(package.get(key) != source_envelope.get(key) for key in ("producer", "numerical_quality", "formulation_basis")) or carrier.get("value") != canonical_sha256_checked_v1(source_envelope):
            raise ValueError("SN-PRECISION-SOURCE-BINDING-MISMATCH")
        expected_result_ref = {"object_type": "ResultEnvelope", "ref": f"result-envelope:{source_envelope.get('run_id')}"}
        expected_carrier = {"algorithm": "sha256", "canonicalization": PROFILE, "payload_scope": "received_result", "payload_ref": expected_result_ref, "value": canonical_sha256_checked_v1(source_envelope)}
        if package.get("source_result_ref") != expected_result_ref or package.get("source_run_ref") != {"object_type": "AnalysisRun", "ref": source_envelope.get("run_id")} or package.get("source_model_ref") != {"object_type": "Model", "ref": source_envelope.get("model_ref")} or carrier != expected_carrier or [item for item in package.get("received_source_checksums", []) if item.get("payload_scope") == "received_result"] != [expected_carrier]:
            raise ValueError("SN-PRECISION-SOURCE-BINDING-MISMATCH")
        raw_rows = source_envelope.get("results", [])
        rows = package.get("result_rows", [])
        _validate_source_rows(source_envelope, rows)
        expected_witnesses, expected_findings = _witnesses(rows, package.get("provenance", {}), source_envelope)
        witness_codes = set(WITHHOLDING_CODES.values()) | {"SN-DECLARED-DIMENSION-WITNESS-UNAVAILABLE"}
        if package.get("unit_preservation_witnesses") != expected_witnesses or [item for item in package.get("diagnostics", []) if item.get("code") in witness_codes] != expected_findings:
            raise ValueError("SN-PRECISION-WITNESS-BINDING-MISMATCH")
        _, _, table_path = _source_contract(source_envelope)
        source_by_id = {row.get("id"): row for row in raw_rows}
        for witness in package.get("unit_preservation_witnesses", []):
            original = source_by_id.get(witness.get("result_id"))
            semantic, _ = _semantic(original, table_path) if original else (None, [])
            if semantic is None or semantic.get("category") == "diagnostic_work" or semantic.get("derivative_target_dimension") != witness.get("source_quantity", {}).get("dimension"):
                raise ValueError("SN-PRECISION-WITNESS-SEMANTICS-MISMATCH")
    if analysis_record is not None:
        if source_envelope is None:
            raise ValueError("SN-PRECISION-ANALYSIS-SOURCE-REQUIRED")
        validate_analysis_run_v0_3(analysis_record, source_envelope, expected_basis_refs=expected_basis_refs)
        if package.get("source_run_ref") != {"object_type":"AnalysisRun", "ref":analysis_record["analysis_run"]["run_id"]}:
            raise ValueError("SN-PRECISION-ANALYSIS-BINDING-MISMATCH")
    if package.get("schema_version") != VERSION:
        raise ValueError("SN-VERSION-UNSUPPORTED")
    if package.get("schema_conformant") is not True:
        raise ValueError("SN-SCHEMA-CONFORMANCE-CLAIM-INVALID")
    if package.get("export_profile", {}).get("profile_id") != EXPORT_PROFILE or package.get("export_profile", {}).get("profile_version") != VERSION or package.get("manifest", {}).get("export_profile_ref") != {"object_type": "StressNeutralExportProfile", "ref": EXPORT_PROFILE}:
        raise ValueError("SN-PROFILE-IDENTITY-MISMATCH")
    members = materialized_members_v0_3(package)
    if list(members) != MEMBERS:
        raise ValueError("SN-MEMBER-INVENTORY-MISMATCH")
    manifest = package.get("manifest")
    if not isinstance(manifest, Mapping) or not isinstance(manifest.get("checksums"), list) or not isinstance(manifest.get("package_members"), list):
        raise ValueError("SN-STRICT-INVENTORY-MISMATCH")
    checksums = manifest["checksums"]
    package_members = manifest["package_members"]
    if len(checksums) != len(MEMBERS) or len(package_members) != len(MEMBERS):
        raise ValueError("SN-STRICT-INVENTORY-MISMATCH")
    member_names = [item.get("filename") if isinstance(item, Mapping) else None for item in package_members]
    checksum_names = [item.get("payload_ref", {}).get("ref") if isinstance(item, Mapping) else None for item in checksums]
    if member_names != MEMBERS:
        raise ValueError("SN-MEMBER-ORDER-MISMATCH")
    if len(set(checksum_names)) != len(MEMBERS) or set(checksum_names) != set(MEMBERS):
        raise ValueError("SN-MEMBER-CHECKSUM-BIJECTION-MISMATCH")
    def expected_metadata(filename: str) -> tuple[str, str, str, str, str]:
        if filename == "manifest.json":
            return ("sha256", PROFILE, "manifest_seed", "StressNeutralMember", filename)
        if filename.endswith(".csv"):
            return ("sha256", "normalized_ascii_lf_text", "member_bytes", "StressNeutralMember", filename)
        return ("sha256", PROFILE, "member_payload", "StressNeutralMember", filename)
    for index, filename in enumerate(MEMBERS):
        claim = next((item for item in checksums if item.get("payload_ref", {}).get("ref") == filename), None)
        actual_metadata = (claim.get("algorithm"), claim.get("canonicalization"), claim.get("payload_scope"), claim.get("payload_ref", {}).get("object_type"), claim.get("payload_ref", {}).get("ref")) if isinstance(claim, Mapping) else None
        if actual_metadata != expected_metadata(filename):
            raise ValueError(f"SN-CHECKSUM-METADATA-MISMATCH: {filename}")
        if package_members[index].get("filename") != filename or package_members[index].get("checksum") != claim:
            raise ValueError(f"SN-MEMBER-CHECKSUM-BINDING-MISMATCH: {filename}")
    package_checksum = package.get("package_checksum", {})
    if (package_checksum.get("algorithm"), package_checksum.get("canonicalization"), package_checksum.get("payload_scope"), package_checksum.get("payload_ref", {}).get("object_type"), package_checksum.get("payload_ref", {}).get("ref")) != ("sha256", PROFILE, "complete_package_excluding_self_checksum", "StressNeutralExportPackage", package.get("export_id")):
        raise ValueError("SN-PACKAGE-CHECKSUM-METADATA-MISMATCH")
    if any(item in checksums for item in package.get("received_source_checksums", [])):
        raise ValueError("SN-RECEIVED-CHECKSUM-RELABEL")
    non_manifest = [item for item in checksums if item["payload_ref"]["ref"] != "manifest.json"]
    expected_seed = _checked(_manifest_seed(package, non_manifest), "manifest.json", "manifest_seed")
    actual_seed = next((item for item in checksums if item["payload_ref"]["ref"] == "manifest.json"), None)
    if actual_seed != expected_seed:
        raise ValueError("SN-MANIFEST-SEED-CHECKSUM-MISMATCH")
    import hashlib
    for item in non_manifest:
        filename = item["payload_ref"]["ref"]
        if filename not in members:
            raise ValueError("SN-MEMBER-INVENTORY-MISMATCH")
        if hashlib.sha256(members[filename]).hexdigest() != item["value"]:
            raise ValueError(f"SN-MEMBER-CHECKSUM-MISMATCH: {filename}")
    rows = package.get("result_rows", [])
    row_ids = [row.get("result_id") for row in rows]
    if len(set(row_ids)) != len(rows) or any(not isinstance(row_id, str) or row.get("canonical_ref", {}).get("ref") != row_id or row.get("source_result_ref", {}).get("ref") != row_id for row_id, row in zip(row_ids, rows)):
        raise ValueError("SN-RESULT-ROW-IDENTITY-MISMATCH")
    _validate_received_csv(package.get("csv_text"), rows)
    stable = package.get("stable_id_map", [])
    canonical_ids = [item.get("canonical_ref", {}).get("ref") for item in stable]
    export_ids = [item.get("export_ref", {}).get("ref") for item in stable]
    if len(stable) != len(rows) or len(set(canonical_ids)) != len(rows) or len(set(export_ids)) != len(rows) or set(canonical_ids) != set(row_ids) or any(item.get("mapping_status") != "mapped" or item.get("loss_category") != "exported" for item in stable):
        raise ValueError("SN-STABLE-ID-MAP-BINDING-MISMATCH")
    received = package.get("received_source_checksums", [])
    if not any(item.get("payload_ref") == package.get("source_result_ref") for item in received):
        raise ValueError("SN-SOURCE-RESULT-REF-UNBOUND")
    if package.get("source_run_ref") not in package.get("reproducibility_refs", []):
        raise ValueError("SN-SOURCE-RUN-REF-UNBOUND")
    seen_witnesses: set[str] = set()
    for witness in package.get("unit_preservation_witnesses", []):
        index = witness.get("source_row_index")
        row = rows[index] if isinstance(index, int) and 0 <= index < len(rows) else None
        disposition, interpreted_dimension = _witness_disposition(row) if row else ("unknown_semantic", None)
        quantity = {"value": row.get("value"), "unit": row.get("unit"), "dimension": interpreted_dimension} if row and disposition == "eligible" else None
        if row is None or witness.get("witness_id") != f"unit-witness:{index}" or witness.get("result_id") in seen_witnesses or witness.get("result_id") != row.get("result_id") or witness.get("source_quantity") != quantity or witness.get("target_quantity") != quantity or witness.get("conversion_performed") is not False or witness.get("policy") != "preserve_received_value_and_unit":
            raise ValueError("SN-UNIT-WITNESS-BINDING-MISMATCH")
        seen_witnesses.add(witness["result_id"])
    for entry in package.get("loss_report", []):
        if entry.get("target_artifact_ref", {}).get("ref") != package.get("export_id") or entry.get("human_review_required") is not True:
            raise ValueError("SN-LOSS-REPORT-BINDING-MISMATCH")
    categories_by_code = {code: category for category, code in WITHHOLDING_CODES.items()}
    categorized: dict[str, str] = {}
    for item in package.get("diagnostics", []):
        category = categories_by_code.get(item.get("code"))
        if category is None:
            continue
        result_id = item.get("source", {}).get("ref")
        expected_severity = "info" if category == "diagnostic_work" else "blocking"
        if not isinstance(result_id, str) or result_id not in row_ids or result_id in categorized or item.get("severity") != expected_severity:
            raise ValueError("SN-WITNESS-CATEGORY-ACCOUNTING-MISMATCH")
        row = rows[row_ids.index(result_id)]
        expected_category, _ = _witness_disposition(row)
        if category == "diagnostic_work":
            explicit_work_row = row.get("row_kind") in {"diagnostic_work", "work"} or row.get("result_family") == "diagnostic_work"
            explicit_withheld_semantics = row.get("dimension") in {None, "", "TBD"} and row.get("correlation_status") != "canonical_id_map"
            if expected_category == "eligible" or not (explicit_work_row or explicit_withheld_semantics):
                raise ValueError("SN-WITNESS-CATEGORY-ACCOUNTING-MISMATCH")
        else:
            if expected_category == "diagnostic_work" or (expected_category == "contradiction" and category != "contradiction"):
                raise ValueError("SN-WITNESS-CATEGORY-ACCOUNTING-MISMATCH")
        categorized[result_id] = category
    if set(categorized) & seen_witnesses or len(categorized) + len(seen_witnesses) != len(rows) or any(row_id not in categorized and row_id not in seen_witnesses for row_id in row_ids):
        raise ValueError("SN-WITNESS-CATEGORY-ACCOUNTING-MISMATCH")
    if SEMANTIC_CONTRACT_REF not in package.get("export_profile", {}).get("source_basis_refs", []) or SEMANTIC_CONTRACT_REF not in package.get("unit_system_disclosure", {}).get("decision_basis_refs", []):
        raise ValueError("SN-SEMANTIC-CONTRACT-BINDING-MISSING")
    blocking_diagnostics = sum(item.get("severity") == "blocking" for item in package.get("diagnostics", []))
    checks = package.get("validation_report", {}).get("checks", [])
    checks_consistent = isinstance(checks, list) and all(isinstance(item.get("blocking_count"), int) and item["blocking_count"] >= 0 and ((item.get("check_status") == "blocking" and item["blocking_count"] > 0) or (item.get("check_status") == "passed" and item["blocking_count"] == 0)) for item in checks)
    if not checks_consistent or package.get("validation_report", {}).get("validation_status") != ("blocked" if blocking_diagnostics else "passed") or any(item.get("check_status") == "blocking" for item in checks) != bool(blocking_diagnostics) or package.get("validation_ready") != (blocking_diagnostics == 0):
        raise ValueError("SN-VALIDATION-STATUS-BINDING-MISMATCH")
    privacy = package.get("privacy", {})
    if privacy.get("local_only") is not True or any(privacy.get(key) is not False for key in ("commercial_tool_payload_embedded", "private_payload_embedded", "protected_payload_embedded", "telemetry_allowed")):
        raise ValueError("SN-PRIVACY-BOUNDARY-VIOLATION")
    boundary = package.get("professional_boundary", {})
    false_claims = ("software_makes_release_claim", "software_makes_external_compatibility_claim", "software_makes_solver_validation_claim", "software_makes_compliance_claim", "software_makes_certification_claim", "software_makes_sealing_claim", "software_makes_approval_claim", "software_creates_professional_reliance_record")
    if boundary.get("human_review_required") is not True or any(boundary.get(key) is not False for key in false_claims):
        raise ValueError("SN-PROFESSIONAL-BOUNDARY-VIOLATION")
    expected_package = canonical_sha256_checked_v1(package_projection(package))
    if package_checksum.get("value") != expected_package:
        raise ValueError("SN-PACKAGE-CHECKSUM-MISMATCH")
