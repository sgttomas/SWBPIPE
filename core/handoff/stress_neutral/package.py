"""Native current stress-neutral 0.3 package construction for DEL-17-06.

Rust owns the source reader verdicts. This module normalizes the supplied
construction inputs and directly builds the nine-member checked-hash package.
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
import struct
from typing import Any, Mapping

from core.serialization.canonical_json.adapter import canonical_json_checked_v1, canonical_sha256_checked_v1
from core.handoff.external_prover.authority_boundary import contains_prohibited_authority_term
from core.analysis_runs.compatibility import (
    _source_contract, _semantic, validate_analysis_run_v0_3,
    _source_basis_reference, _source_reference_text,
    _validate_source_reference_fields, PRESSURE_CONTRACT_ID,
    PRESSURE_SOURCE_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID,
)

REQUIRED_SOURCE_BASIS_REFS = {
    ("Deliverable", "DEL-08-04"),
    ("Deliverable", "DEL-14-02"),
    ("Deliverable", "DEL-14-05"),
    ("Deliverable", "DEL-17-02"),
}


LOSS_CATEGORIES = {
    "exported",
    "omitted",
    "approximated",
    "delegated",
    "unsupported",
    "tbd",
}


ENGINE_PROVENANCE = {
    "source_name": "OpenPipeStress DEL-17-06 stress-neutral export package builder",
    "source_location": "core/handoff/stress_neutral/package.py",
    "source_license": "project-governed",
    "contributor": "OpenPipeStress Type 2 worker",
    "contributor_certification": "implementation-only-no-professional-claim",
    "redistribution_status": "public_permissive",
    "review_status": "machine_checked",
    "privacy_classification": "public_metadata",
}


PROFESSIONAL_BOUNDARY = {
    "human_review_required": True,
    "supports_review": True,
    "supports_regression_comparison_input": True,
    "supports_downstream_tooling": True,
    "software_makes_release_claim": False,
    "software_makes_external_compatibility_claim": False,
    "software_makes_solver_validation_claim": False,
    "software_makes_compliance_claim": False,
    "software_makes_certification_claim": False,
    "software_makes_sealing_claim": False,
    "software_makes_approval_claim": False,
    "software_creates_professional_reliance_record": False,
}


DEFAULT_PRIVACY = {
    "classification": "invented_public_example",
    "local_only": True,
    "telemetry_allowed": False,
    "private_payload_embedded": False,
    "protected_payload_embedded": False,
    "commercial_tool_payload_embedded": False,
    "redaction_refs": [],
}


DEFAULT_BOUNDARY_NOTES = [
    "Stress-neutral CSV/JSON is a project-owned review and regression artifact.",
    "This package is not a vendor format, solver input deck, or target compatibility claim.",
    "Comparison semantics are diagnostic/export-only; pass/fail tolerances remain outside this package.",
    "Package evidence does not assert solver validation, release readiness, code compliance, or professional reliance.",
]


VERSION = "0.3.0"
PROFILE = "openpipestress_jcs_ijson_v1"
EXPORT_PROFILE = "ops.stress_neutral.v3"
MEMBERS = ["manifest.json", "stress_neutral_results.csv", "result_rows.json", "unit_system_disclosure.json", "unit_preservation_witnesses.json", "stable_id_map.json", "loss_report.json", "validation_report.json", "diagnostics.json"]
# B7b (A4/A5): the current exact identities, pressure-2 and pressure-source-1, take over the retired v2 identities'
# packages (physics-1, physics-source-1, load-reference-1, load-reference-source-1, which the Rust reader refuses by
# name). A load/reference-state model publishes either with contract_evidence.load_reference_states.
SUPPORTED_METHODS = {PRESSURE_CONTRACT_ID, PRESSURE_SOURCE_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_ID,
                     PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID}
PHYSICAL_METHODS = {PRESSURE_CONTRACT_ID, PRESSURE_SOURCE_CONTRACT_ID}
RECEIPT_METHODS = {PRESSURE_SOURCE_CONTRACT_ID}
# preview-physics-2 retains its closed preview statement; no receipt namespace. The pressure-free Timoshenko
# identity (DEL-05-03 pf-timoshenko.md §4) carries the same statement under its own table.
PREVIEW_METHODS = {PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID}


def _method_namespaces(contract_id: str) -> set[str]:
    return ({"contract_evidence"} if contract_id in PHYSICAL_METHODS | PREVIEW_METHODS else set()) | ({"source_block_recovery"} if contract_id in RECEIPT_METHODS else set())


def _transport_contract(package: Mapping[str, Any], source_envelope: Mapping[str, Any] | None = None) -> tuple[str, str, Path]:
    """Validate retained transport statements; never reconstruct raw publication.

    Source validation below separately requires the actual supplied raw envelope.
    This metadata view deliberately has no results, diagnostics or fake status.
    pressure-2 has no transport reading (below), so its package is read through its source.
    """
    metadata = {"schema_version": "0.2.0", **{key: package.get(key) for key in ("producer", "numerical_quality", "formulation_basis")}}
    for key in ("contract_evidence", "source_block_recovery", "carrier_evidence"):
        if key in package:
            metadata[key] = package[key]
    producer = package.get("producer")
    if isinstance(producer, Mapping) and producer.get("semantic_contract_id") == PRESSURE_CONTRACT_ID and source_envelope is not None:
        method = _pressure_source_reading(source_envelope)
    else:
        # A pressure-2 package without its source takes Rust's transport verdict too: the reader
        # refuses it by name (SOURCE_PRESSURE_TRANSPORT_UNSUPPORTED).
        method = _source_contract(metadata, check_receipt=False)
    if method[0] not in SUPPORTED_METHODS:
        raise ValueError("SN-SOURCE-METHOD-UNSUPPORTED")
    expected = _method_namespaces(method[0])
    if {key for key in ("contract_evidence", "source_block_recovery") if key in package} != expected:
        raise ValueError("SN-SOURCE-NAMESPACE-MISMATCH")
    return method


def _pressure_source_reading(source_envelope: Mapping[str, Any]) -> tuple[str, str, Path]:
    """pressure-2's package statements, read through their source (B7b). Rust has no pressure-2
    transport reader (`transport("pressure")` is refused by name), so a pressure-2 package is read
    only with its supplied source envelope: the source takes Rust's raw reading (`read pressure`),
    and every member the package copies from it (producer, numerical quality, formulation basis,
    contract evidence, rows, annotations and the received checksum) is then bound to that source by
    the checks that follow."""
    method = _source_contract(source_envelope)
    if method[0] != PRESSURE_CONTRACT_ID:
        raise ValueError("SN-SOURCE-METHOD-BINDING-MISMATCH")
    return method


WITHHOLDING_CODES = {
    "diagnostic_work": "SN-UNIT-WITNESS-WITHHELD-DIAGNOSTIC-WORK",
    "unknown_semantic": "SN-UNIT-WITNESS-WITHHELD-UNKNOWN-SEMANTIC",
    "missing_semantic": "SN-UNIT-WITNESS-WITHHELD-MISSING-SEMANTIC",
    "contradiction": "SN-UNIT-WITNESS-WITHHELD-CONTRADICTION",
}
CSV_COLUMNS = [
    "result_id", "canonical_ref", "row_kind", "result_family", "load_case_ref",
    "station_ref", "component_ref", "value", "unit", "dimension", "correlation_status",
]
JSON_NUMBER = re.compile(r"-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?\Z")
MAX_SAFE_INTEGER = (1 << 53) - 1
UTF8_CSV_CANONICALIZATION = "utf8_csv_record_lf_v1"
UTF8_CSV_ROW_ORDER = "unicode_scalar_value_result_id"


def _csv_policy(package: Mapping[str, Any]) -> tuple[str, str]:
    """Method-bound CSV wire contract: UTF-8 records in scalar result-id order."""
    profile = package.get("export_profile", {})
    producer = package.get("producer", {})
    if not isinstance(profile, Mapping) or not isinstance(producer, Mapping):
        raise ValueError("SN-CSV-PROFILE-MISMATCH")
    method = producer.get("semantic_contract_id")
    if not isinstance(method, str) or method not in SUPPORTED_METHODS or profile.get("csv_encoding") != "utf-8" or profile.get("csv_row_order") != UTF8_CSV_ROW_ORDER:
        raise ValueError("SN-CSV-PROFILE-MISMATCH")
    return "utf-8", UTF8_CSV_CANONICALIZATION


def _utf8_csv_bytes(text: Any) -> bytes:
    """Strict UTF8 without BOM; record LF is checked without changing field data."""
    if not isinstance(text, str) or text.startswith("\ufeff") or not text.endswith("\n"):
        raise ValueError("SN-CSV-UTF8-INVALID")
    try:
        encoded = text.encode("utf-8", errors="strict")
    except UnicodeEncodeError as error:
        raise ValueError("SN-CSV-UTF8-INVALID") from error
    state = "start"
    index = 0
    while index < len(text):
        char = text[index]
        if state == "quoted":
            if char == '"':
                if index + 1 < len(text) and text[index + 1] == '"':
                    index += 1
                else:
                    state = "closed"
        elif char == "\r":
            raise ValueError("SN-CSV-RECORD-LF-REQUIRED")
        elif char in {",", "\n"}:
            state = "start"
        elif state == "closed":
            raise ValueError("SN-CSV-QUOTING-INVALID")
        elif char == '"':
            if state != "start":
                raise ValueError("SN-CSV-QUOTING-INVALID")
            state = "quoted"
        else:
            state = "plain"
        index += 1
    if state != "start":
        raise ValueError("SN-CSV-QUOTING-INVALID")
    return encoded


def _render_utf8_csv(rows: list[Mapping[str, Any]]) -> str:
    """Scalar-order CSV; no whole-text newline or Unicode normalization."""
    identities = [row.get("result_id") for row in rows]
    if any(not isinstance(identity, str) or not identity for identity in identities):
        raise ValueError("SN-CSV-ROW-BINDING-MISMATCH")
    def field(text: str) -> str:
        return '"' + text.replace('"', '""') + '"' if any(c in text for c in ',"\r\n') else text
    records = [",".join(CSV_COLUMNS)]
    for row in sorted(rows, key=lambda item: item["result_id"]):
        values = []
        for column in CSV_COLUMNS:
            value = _csv_scalar(row, column)
            if column == "value":
                value = str(int(value)) if isinstance(value, float) and value.is_integer() else "" if value is None else str(value)
            if not isinstance(value, str):
                raise ValueError("SN-CSV-ROW-BINDING-MISMATCH")
            values.append(field(value))
        records.append(",".join(values))
    text = "\n".join(records) + "\n"
    _utf8_csv_bytes(text)
    return text


def _checked(payload: Any, filename: str, scope: str = "member_payload") -> dict[str, Any]:
    return {"algorithm": "sha256", "canonicalization": PROFILE, "payload_scope": scope, "payload_ref": {"object_type": "StressNeutralMember", "ref": filename}, "value": canonical_sha256_checked_v1(payload)}


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
    _utf8_csv_bytes(csv_text)
    ids = [row.get("result_id") for row in rows]
    if any(not isinstance(identity, str) for identity in ids) or ids != sorted(ids):
        raise ValueError("SN-CSV-ROW-ORDER-MISMATCH")
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


def _source_witness_disposition(row: Mapping[str, Any], original: Mapping[str, Any] | None, table: Path) -> tuple[str, str | None]:
    """New-method interpretation from the exact validated source/table signature.

    A unit spelling alone cannot override the table's derivative target. In
    particular, a disclosed unitless diagnostic may have a ratio target without
    becoming a physical or governing/compliance ratio result.
    """
    if not isinstance(original, Mapping):
        return "unknown_semantic", None
    try:
        semantic, _ = _semantic(original, table)
    except ValueError:
        return "contradiction", None
    if semantic is None:
        return "unknown_semantic", None
    if semantic.get("category") == "diagnostic_work":
        return "diagnostic_work", None
    dimension = semantic.get("derivative_target_dimension")
    if not isinstance(dimension, str) or dimension in {"", "TBD"}:
        return "missing_semantic", None
    family = (semantic.get("family") or "other") if semantic.get("category") == "physical_quantity" else "other"
    if row.get("dimension") != dimension or row.get("unit") != original.get("unit") or row.get("result_family") != family or row.get("row_kind") != "result_value" or row.get("correlation_status") != "canonical_id_map":
        return "contradiction", None
    return "eligible", dimension


def _witnesses(rows: list[dict[str, Any]], provenance: Mapping[str, Any], source: Mapping[str, Any]) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    witnesses, findings = [], []
    source_rows = {item.get("id"): item for item in source.get("results", [])}
    _, _, table = _source_contract(source)
    for index, row in enumerate(rows):
        disposition, interpreted_dimension = _source_witness_disposition(row, source_rows.get(row["result_id"]), table)
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


def source_row_projection(source: Mapping[str, Any], row: Mapping[str, Any]) -> dict[str, Any]:
    """Source-qualified physical identity; never infer meaning from unit alone."""
    _, _, table = _source_contract(source)
    return _source_row_projection(source, row, table)


def _source_row_projection(source: Mapping[str, Any], row: Mapping[str, Any], table: Path) -> dict[str, Any]:
    semantic, _ = _semantic(row, table)
    family = (semantic.get("family") or "other") if semantic and semantic.get("category") == "physical_quantity" else "other"
    dimension = semantic.get("derivative_target_dimension") if semantic else None
    dimension = dimension or "TBD"
    _source_reference_text(row.get("id"))
    basis_ref = _source_basis_reference(row.get("basis_ref"))
    if basis_ref is None:
        basis_ref = {"object_type": "AnalysisRun", "ref": _source_reference_text(source.get("run_id"))}
    entity = row.get("entity_ref")
    if not isinstance(entity, str) or not entity:
        raise ValueError("SN-PRECISION-ROW-ENTITY-MISSING")
    entity_type = {"pipe": "PipeElement", "node": "Node", "support": "Support", "component": "Component", "material": "Material"}.get(entity.split(":", 1)[0] if ":" in entity else "", "CanonicalObject")
    if row.get("kind", "").startswith("support_reaction_") and row.get("kind", "").endswith("_v2"):
        entity_type = "Support"
    metadata = row.get("metadata") if isinstance(row.get("metadata"), Mapping) else {}
    location = metadata.get("location")
    if location is not None and (not isinstance(location, str) or not location):
        raise ValueError("SN-PRECISION-ROW-LOCATION-INVALID")
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
    contract_id, _, table = _source_contract(source)
    expected = {row.get("id"): _source_row_projection(source, row, table) for row in raw}
    for row in rows:
        projection = expected.get(row.get("result_id"))
        if projection is None or any(key not in row or row[key] != value for key, value in projection.items()):
            raise ValueError("SN-PRECISION-ROW-SOURCE-MISMATCH")


def _validate_stable_id_map(entries: Any, rows: list[Mapping[str, Any]]) -> None:
    """A bijection of exact source identities to caller-defined typed targets."""
    def reference(value: Any) -> tuple[str, str]:
        if not isinstance(value, Mapping) or set(value) != {"object_type", "ref"} or any(not isinstance(value[key], str) or not value[key] for key in ("object_type", "ref")):
            raise ValueError("SN-STABLE-ID-MAP-BINDING-MISMATCH")
        return value["object_type"], value["ref"]

    if not isinstance(entries, list) or len(entries) != len(rows):
        raise ValueError("SN-STABLE-ID-MAP-BINDING-MISMATCH")
    expected = {row["result_id"]: reference(row.get("canonical_ref")) for row in rows}
    seen_sources: set[str] = set()
    seen_targets: set[tuple[str, str]] = set()
    for entry in entries:
        if not isinstance(entry, Mapping):
            raise ValueError("SN-STABLE-ID-MAP-BINDING-MISMATCH")
        source = reference(entry.get("canonical_ref"))
        target = reference(entry.get("export_ref"))
        if expected.get(source[1]) != source or source[1] in seen_sources or target in seen_targets or entry.get("mapping_status") != "mapped" or entry.get("loss_category") != "exported":
            raise ValueError("SN-STABLE-ID-MAP-BINDING-MISMATCH")
        seen_sources.add(source[1])
        seen_targets.add(target)
    if seen_sources != set(expected):
        raise ValueError("SN-STABLE-ID-MAP-BINDING-MISMATCH")


def build_stress_neutral_export_package(*, source_envelope: Mapping[str, Any], analysis_record: Mapping[str, Any], expected_basis_refs: list[Mapping[str, str]] | None = None, **source: Any) -> dict[str, Any]:
    validate_analysis_run_v0_3(analysis_record, source_envelope, expected_basis_refs=expected_basis_refs)
    contract_id, contract_hash, contract_path = _source_contract(source_envelope)
    if contract_id not in SUPPORTED_METHODS:
        raise ValueError("SN-SOURCE-METHOD-UNSUPPORTED")
    semantic_contract_ref = {"object_type": "ExternalReference", "ref": f"fixtures/results/{contract_path.name}"}
    received = [item for item in analysis_record["analysis_run"]["hashes"] if item.get("payload_scope") == "received_result"]
    if len(received) != 1 or received[0] not in source.get("source_hashes", []) or source.get("source_result_ref") != received[0]["payload_ref"] or source.get("source_run_ref") != {"object_type":"AnalysisRun", "ref":source_envelope["run_id"]} or source.get("source_model_ref", {}).get("ref") != source_envelope["model_ref"]:
        raise ValueError("SN-PRECISION-SOURCE-BINDING-MISMATCH")
    _validate_source_rows(source_envelope, source.get("result_rows", []))
    _validate_stable_id_map(source.get("stable_id_map"), source.get("result_rows", []))
    supplied_profile = source.get("export_profile")
    if isinstance(supplied_profile, Mapping):
        if "profile_id" in supplied_profile and supplied_profile["profile_id"] != EXPORT_PROFILE:
            raise ValueError("SN-0.3-PROFILE-IDENTITY-MISMATCH")
        if "profile_version" in supplied_profile and supplied_profile["profile_version"] != VERSION:
            raise ValueError("SN-0.3-PROFILE-IDENTITY-MISMATCH")
    if isinstance(supplied_profile, Mapping):
        if (("csv_encoding" in supplied_profile and supplied_profile["csv_encoding"] != "utf-8") or ("csv_row_order" in supplied_profile and supplied_profile["csv_row_order"] != UTF8_CSV_ROW_ORDER)):
            raise ValueError("SN-CSV-PROFILE-MISMATCH")
    normalized = _normalized_inputs(**deepcopy(source))
    witnesses, witness_findings = _witnesses(normalized["result_rows"], normalized["provenance"], source_envelope)
    diagnostics = [deepcopy(item) for item in normalized["diagnostics"] if item.get("code") not in {"SN-DECLARED-DIMENSION-WITNESS-UNAVAILABLE", "SN-UNIT-DIMENSION-MISSING"}] + witness_findings
    blocking_count = sum(item.get("severity") == "blocking" for item in diagnostics)
    decision_basis_refs = deepcopy(normalized["unit_system_disclosure"]["decision_basis_refs"]) + [deepcopy(semantic_contract_ref), deepcopy(normalized["source_run_ref"])]
    boundary_notes = list(normalized["boundary_notes"]) + ["Result-row dimensions and witness eligibility are interpreted from the accepted semantic contract and bound analysis run; received numerical values, units, rows and source hashes remain unchanged."]
    export_profile = {**normalized["export_profile"], "profile_id": EXPORT_PROFILE, "profile_version": VERSION, "boundary_notes": boundary_notes, "source_basis_refs": deepcopy(normalized["export_profile"]["source_basis_refs"]) + [deepcopy(semantic_contract_ref), deepcopy(normalized["source_run_ref"])]}
    export_profile.update(csv_encoding="utf-8", csv_row_order=UTF8_CSV_ROW_ORDER)
    validation_report = {"validation_status": "blocked" if blocking_count else "passed", "checks": [{"check_id": "stress-neutral-boundary-diagnostics", "check_status": "blocking" if blocking_count else "passed", "diagnostic_count": len(diagnostics), "blocking_count": blocking_count, "provenance": deepcopy(normalized["provenance"])}], "human_review_required": True, "provenance": deepcopy(normalized["provenance"])}
    loss_report = deepcopy(normalized["loss_report"])
    withholding_count = len(witness_findings) - (1 if witness_findings else 0)
    for entry in loss_report:
        if entry.get("category") == "exported":
            entry["reason"] = f"All {len(normalized['result_rows'])} received numerical rows and units are retained unchanged; {len(witnesses)} rows have accepted semantic-contract dimension witnesses and {withholding_count} rows have explicit witness-withholding findings."
    package: dict[str, Any] = {
        "schema_version": VERSION, "deliverable_id": "DEL-17-06", "package_id": "PKG-17", "scope_items": ["SOW-046", "SOW-074"], "objectives": ["OBJ-007", "OBJ-017", "OBJ-018"],
        "export_id": normalized["export_id"], "package_status": "stress_neutral_export_package", "schema_conformant": True,
        "validation_ready": not any(item.get("severity") == "blocking" for item in diagnostics),
        "source_result_ref": normalized["source_result_ref"], "source_run_ref": normalized["source_run_ref"], "source_model_ref": normalized["source_model_ref"],
        "received_source_checksums": deepcopy(source.get("source_hashes", [])), "unresolved_assumption_refs": normalized["unresolved_assumption_refs"], "reproducibility_refs": normalized["reproducibility_refs"],
        "export_profile": export_profile, "manifest": {"manifest_id": normalized["manifest_id"], "export_profile_ref": {"object_type": "StressNeutralExportProfile", "ref": EXPORT_PROFILE}, "boundary_notes": boundary_notes, "package_members": [], "checksums": []},
        "csv_text": normalized["csv_text"], "result_rows": normalized["result_rows"], "unit_system_disclosure": {**normalized["unit_system_disclosure"], "decision_basis_refs": decision_basis_refs}, "unit_preservation_witnesses": witnesses,
        "stable_id_map": normalized["stable_id_map"], "loss_report": loss_report, "validation_report": validation_report, "diagnostics": diagnostics,
        "privacy": normalized["privacy"], "provenance": normalized["provenance"], "professional_boundary": normalized["professional_boundary"],
    }
    for key in ("producer", "numerical_quality", "formulation_basis"):
        package[key] = deepcopy(source_envelope[key])
    package["semantic_contract_ref"] = {"ref_type":"semantic_contract", "ref_id":contract_id}
    package["semantic_contract"] = {"id":contract_id, "sha256":contract_hash}
    for namespace in sorted(_method_namespaces(contract_id)):
        package[namespace] = deepcopy(source_envelope[namespace])
    package["source_annotations"] = _source_annotations(source_envelope)
    package["source_carrier_checksum"] = deepcopy(received[0])
    payloads = {"stress_neutral_results.csv": package["csv_text"], "result_rows.json": package["result_rows"], "unit_system_disclosure.json": package["unit_system_disclosure"], "unit_preservation_witnesses.json": package["unit_preservation_witnesses"], "stable_id_map.json": package["stable_id_map"], "loss_report.json": package["loss_report"], "validation_report.json": package["validation_report"], "diagnostics.json": package["diagnostics"]}
    checksums = []
    for filename, payload in payloads.items():
        if filename.endswith(".csv"):
            import hashlib
            checksum = {"algorithm": "sha256", "canonicalization": UTF8_CSV_CANONICALIZATION, "payload_scope": "member_bytes", "payload_ref": {"object_type": "StressNeutralMember", "ref": filename}, "value": hashlib.sha256(_utf8_csv_bytes(payload)).hexdigest()}
        else:
            checksum = _checked(payload, filename)
        checksums.append(checksum)
    seed_checksum = _checked(_manifest_seed(package, checksums), "manifest.json", "manifest_seed")
    package["manifest"]["checksums"] = checksums + [seed_checksum]
    package["manifest"]["package_members"] = [{"filename": filename, "checksum": next(item for item in package["manifest"]["checksums"] if item["payload_ref"]["ref"] == filename)} for filename in MEMBERS]
    package["package_checksum"] = {"algorithm": "sha256", "canonicalization": PROFILE, "payload_scope": "complete_package_excluding_self_checksum", "payload_ref": {"object_type": "StressNeutralExportPackage", "ref": package["export_id"]}, "value": canonical_sha256_checked_v1(package_projection(package))}
    validate_stress_neutral_export_package(package, source_envelope=source_envelope, analysis_record=analysis_record, expected_basis_refs=expected_basis_refs)
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


def materialized_members(package: Mapping[str, Any]) -> dict[str, bytes]:
    metadata = {key: deepcopy(value) for key, value in package.items()
                if key not in set(_MEMBER_FIELDS.values()) | {"manifest"}}
    transport = {"transport_version": VERSION, "manifest": deepcopy(package["manifest"]),
                 "package_metadata": metadata}
    values = {"manifest.json": transport, **{name: package[field] for name, field in _MEMBER_FIELDS.items()}}
    _csv_policy(package)
    return {name: (_utf8_csv_bytes(value) if name.endswith(".csv") else canonical_json_checked_v1(value).encode("utf-8")) for name, value in values.items()}


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


def reconstruct_materialized_members(members: Mapping[str, bytes], **validation: Any) -> dict[str, Any]:
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
    _csv_policy(package)
    for name, field in _MEMBER_FIELDS.items():
        if name.endswith(".csv"):
            try:
                package[field] = members[name].decode("utf-8", errors="strict")
            except UnicodeDecodeError as error:
                raise ValueError("SN-CSV-ENCODING-INVALID") from error
            _utf8_csv_bytes(package[field])
        else:
            package[field] = _transport_json(members[name])
    validate_stress_neutral_export_package(package, **validation)
    return package


def read_materialized_members(input_dir: str | Path, **validation: Any) -> dict[str, Any]:
    directory = Path(input_dir)
    if {entry.name for entry in directory.iterdir()} != set(MEMBERS) or any(not (directory / name).is_file() for name in MEMBERS):
        raise ValueError("SN-MEMBER-INVENTORY-MISMATCH")
    return reconstruct_materialized_members({name: (directory / name).read_bytes() for name in MEMBERS}, **validation)


def write_materialized_members(package: Mapping[str, Any], output_dir: str | Path, **validation: Any) -> list[Path]:
    """Validate and write the exact nine contracted member byte streams. `validation` takes the
    validator's optional source bindings; a pressure-2 package needs its source (no transport reader)."""
    validate_stress_neutral_export_package(package, **validation)
    destination = Path(output_dir)
    destination.mkdir(parents=True, exist_ok=True)
    written = []
    for filename, payload in materialized_members(package).items():
        target = destination / filename
        target.write_bytes(payload)
        written.append(target)
    return written


def validate_stress_neutral_export_package(package: Mapping[str, Any], *, source_envelope: Mapping[str, Any] | None = None, analysis_record: Mapping[str, Any] | None = None, expected_basis_refs: list[Mapping[str, str]] | None = None) -> None:
    """Check packet claims/hash; supplied source enables additional binding checks.

    Neither mode authenticates a producer or qualifies Current numerical use.
    """
    if package.get("schema_version") != VERSION:
        raise ValueError("SN-VERSION-UNSUPPORTED")
    contract_id, contract_hash, contract_path = _transport_contract(package, source_envelope)
    csv_encoding, csv_canonicalization = _csv_policy(package)
    semantic_contract_ref = {"object_type": "ExternalReference", "ref": f"fixtures/results/{contract_path.name}"}
    if package.get("semantic_contract") != {"id":contract_id, "sha256":contract_hash} or package.get("semantic_contract_ref") != {"ref_type":"semantic_contract", "ref_id":contract_id}:
        raise ValueError("SN-PRECISION-SEMANTIC-CONTRACT-MISMATCH")
    _validate_source_annotations(package, contract_path)
    source_annotation_rows = {annotation["source_result_id"]: annotation["source_row"] for annotation in package["source_annotations"]}
    carrier = package.get("source_carrier_checksum", {})
    if carrier not in package.get("received_source_checksums", []) or carrier.get("payload_ref") != package.get("source_result_ref") or carrier.get("payload_scope") != "received_result" or carrier.get("algorithm") != "sha256" or carrier.get("canonicalization") != PROFILE:
        raise ValueError("SN-PRECISION-CARRIER-BINDING-MISMATCH")
    if source_envelope is not None:
        actual_method = _source_contract(source_envelope)
        if package.get("source_annotations") != _source_annotations(source_envelope):
            raise ValueError("SN-SOURCE-ANNOTATION-BINDING-MISMATCH")
        if actual_method != (contract_id, contract_hash, contract_path):
            raise ValueError("SN-SOURCE-METHOD-BINDING-MISMATCH")
        for namespace in _method_namespaces(contract_id):
            if package.get(namespace) != source_envelope.get(namespace):
                raise ValueError("SN-SOURCE-EVIDENCE-BINDING-MISMATCH")
        _validate_source_reference_fields(source_envelope)
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
    members = materialized_members(package)
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
            return ("sha256", csv_canonicalization, "member_bytes", "StressNeutralMember", filename)
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
    _validate_stable_id_map(package.get("stable_id_map"), rows)
    received = package.get("received_source_checksums", [])
    if not any(item.get("payload_ref") == package.get("source_result_ref") for item in received):
        raise ValueError("SN-SOURCE-RESULT-REF-UNBOUND")
    if package.get("source_run_ref") not in package.get("reproducibility_refs", []):
        raise ValueError("SN-SOURCE-RUN-REF-UNBOUND")
    seen_witnesses: set[str] = set()
    for witness in package.get("unit_preservation_witnesses", []):
        index = witness.get("source_row_index")
        row = rows[index] if isinstance(index, int) and 0 <= index < len(rows) else None
        if row:
            disposition, interpreted_dimension = _source_witness_disposition(row, source_annotation_rows.get(row.get("result_id")), contract_path)
        else:
            disposition, interpreted_dimension = "unknown_semantic", None
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
        expected_category, _ = _source_witness_disposition(row, source_annotation_rows.get(result_id), contract_path)
        if category != expected_category:
            raise ValueError("SN-WITNESS-CATEGORY-ACCOUNTING-MISMATCH")
        categorized[result_id] = category
    if set(categorized) & seen_witnesses or len(categorized) + len(seen_witnesses) != len(rows) or any(row_id not in categorized and row_id not in seen_witnesses for row_id in row_ids):
        raise ValueError("SN-WITNESS-CATEGORY-ACCOUNTING-MISMATCH")
    if semantic_contract_ref not in package.get("export_profile", {}).get("source_basis_refs", []) or semantic_contract_ref not in package.get("unit_system_disclosure", {}).get("decision_basis_refs", []):
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


def _source_value_bits(value: Any) -> str:
    """Finite received binary64 identity, outside JCS's zero normalization."""
    if type(value) not in (int, float):
        raise ValueError("SN-SOURCE-ANNOTATION-VALUE-BITS")
    try:
        converted = float(value)
        if not math.isfinite(converted):
            raise ValueError("SN-SOURCE-ANNOTATION-VALUE-BITS")
        return struct.pack(">d", converted).hex()
    except (OverflowError, struct.error) as error:
        raise ValueError("SN-SOURCE-ANNOTATION-VALUE-BITS") from error


def _source_annotations(source: Mapping[str, Any]) -> list[dict[str, Any]]:
    """Retain row fields plus actual value bits under the unchanged JCS profile.

    source_row.value is the canonical numeric value (+0 for either zero sign).
    source_value_bits captures the received IEEE754 value before serialization;
    it is required because JCS intentionally erases negative zero. Other row
    properties, absent/null distinctions and the checked row hash are retained.
    No producer custody follows from these transported statements.
    """
    annotations = []
    for index, row in enumerate(source["results"]):
        value_bits = _source_value_bits(row["value"])
        canonical_row = deepcopy(row)
        if canonical_row["value"] == 0:
            canonical_row["value"] = 0.0
        annotations.append({"source_row_index": index, "source_result_id": row["id"],
            "source_row": canonical_row, "source_row_sha256": canonical_sha256_checked_v1(row),
            "source_value_bits": value_bits})
    return annotations


def _validate_source_value_bits(annotation: Mapping[str, Any]) -> None:
    bits = annotation.get("source_value_bits")
    if not isinstance(bits, str) or re.fullmatch(r"[0-9a-f]{16}", bits) is None:
        raise ValueError("SN-SOURCE-ANNOTATION-VALUE-BITS")
    value = annotation["source_row"].get("value")
    canonical_bits = _source_value_bits(value)
    decoded = struct.unpack(">d", bytes.fromhex(bits))[0]
    if not math.isfinite(decoded) or decoded != float(value):
        raise ValueError("SN-SOURCE-ANNOTATION-VALUE-BITS")
    if value == 0:
        if canonical_bits != "0000000000000000" or bits not in {"0000000000000000", "8000000000000000"}:
            raise ValueError("SN-SOURCE-ANNOTATION-VALUE-BITS")
    elif bits != canonical_bits:
        raise ValueError("SN-SOURCE-ANNOTATION-VALUE-BITS")


def _validate_source_annotations(package: Mapping[str, Any], table: Path) -> None:
    annotations = package.get("source_annotations")
    rows = package.get("result_rows")
    if not isinstance(rows, list) or not isinstance(annotations, list) or len(rows) != len(annotations):
        raise ValueError("SN-SOURCE-ANNOTATION-COVERAGE")
    by_id = {row.get("result_id"): row for row in rows}
    if len(by_id) != len(rows):
        raise ValueError("SN-SOURCE-ANNOTATION-COVERAGE")
    seen: set[str] = set()
    # Only the fallback run identity is needed for row projection. This is not
    # passed to any raw-source validator or treated as a mechanics publication.
    projection_identity = {"run_id": package.get("source_run_ref", {}).get("ref")}
    for index, annotation in enumerate(annotations):
        if not isinstance(annotation, Mapping) or set(annotation) != {"source_row_index", "source_result_id", "source_row", "source_row_sha256", "source_value_bits"}:
            raise ValueError("SN-SOURCE-ANNOTATION-SHAPE")
        original = annotation["source_row"]
        identity = annotation["source_result_id"]
        if not isinstance(original, Mapping) or not isinstance(identity, str) or not identity or identity in seen or original.get("id") != identity or type(annotation["source_row_index"]) is not int or annotation["source_row_index"] != index or annotation["source_row_sha256"] != canonical_sha256_checked_v1(original):
            raise ValueError("SN-SOURCE-ANNOTATION-IDENTITY-OR-HASH")
        _validate_source_value_bits(annotation)
        seen.add(identity)
        row = by_id.get(identity)
        projection = _source_row_projection(projection_identity, original, table)
        if row is None or any(key not in row or row[key] != value for key, value in projection.items()):
            raise ValueError("SN-SOURCE-ANNOTATION-ROW-BINDING")
    if seen != set(by_id):
        raise ValueError("SN-SOURCE-ANNOTATION-COVERAGE")


def _boundary_diagnostics(
    *,
    result_rows: list[Mapping[str, Any]],
    stable_id_map: list[Mapping[str, Any]],
    loss_report: list[Mapping[str, Any]],
    export_profile: Mapping[str, Any],
    privacy: Mapping[str, Any],
    package_ref: Mapping[str, Any],
) -> list[dict[str, Any]]:
    """Return deterministic diagnostics for stress-neutral package boundaries."""

    diagnostics: list[dict[str, Any]] = []
    if not result_rows:
        diagnostics.append(
            _diagnostic(
                "SN-RESULT-ROWS-MISSING",
                "blocking",
                "EXPORT_BLOCKING",
                "Stress-neutral package has no result rows.",
                "Provide unit-bearing invented or governed result rows.",
                [package_ref],
            )
        )
    if not stable_id_map:
        diagnostics.append(
            _diagnostic(
                "SN-STABLE-ID-MAP-MISSING",
                "blocking",
                "TARGET_MAPPING_WARNING",
                "Stress-neutral package has no stable ID map entries.",
                "Map exported result identities to canonical refs explicitly.",
                [package_ref],
            )
        )
    if not loss_report:
        diagnostics.append(
            _diagnostic(
                "SN-LOSS-REPORT-MISSING",
                "blocking",
                "UNSUPPORTED_BEHAVIOR_WARNING",
                "Stress-neutral package has no loss report entries.",
                "Record exported, omitted, approximated, delegated, unsupported, or tbd behavior explicitly.",
                [package_ref],
            )
        )
    for row in result_rows:
        if not row.get("unit") or not row.get("dimension"):
            diagnostics.append(
                _diagnostic(
                    "SN-UNIT-DIMENSION-MISSING",
                    "blocking",
                    "UNIT_WARNING",
                    "Stress-neutral result row is missing unit or dimension metadata.",
                    "Carry explicit unit and dimension metadata for every exported row.",
                    [_ref("Result", str(row.get("result_id", "unknown")))],
                )
            )
    for entry in loss_report:
        if entry.get("category") not in LOSS_CATEGORIES:
            diagnostics.append(
                _diagnostic(
                    "SN-LOSS-CATEGORY-UNSUPPORTED",
                    "blocking",
                    "UNSUPPORTED_BEHAVIOR_WARNING",
                    "Loss report entry uses a category outside the DEL-17-02 taxonomy.",
                    "Use exported, omitted, approximated, delegated, unsupported, or tbd.",
                    [_ref("LossReportEntry", str(entry.get("loss_id", "unknown")))],
                )
            )
    if export_profile.get("comparison_semantics") != "diagnostic_export_only_no_pass_fail":
        diagnostics.append(
            _diagnostic(
                "SN-COMPARISON-SEMANTICS-UNSAFE",
                "blocking",
                "ASSUMPTION_WARNING",
                "Stress-neutral profile does not preserve diagnostic-only comparison semantics.",
                "Keep comparison pass/fail tolerances outside DEL-17-06.",
                [_ref("ExportProfile", str(export_profile.get("profile_id", "unknown")))],
            )
        )
    source_basis = _ref_pairs(export_profile.get("source_basis_refs", []))
    missing_source_basis = sorted(REQUIRED_SOURCE_BASIS_REFS - source_basis)
    if missing_source_basis:
        diagnostics.append(
            _diagnostic(
                "SN-SOURCE-BASIS-REFS-MISSING",
                "blocking",
                "PROVENANCE_WARNING",
                "Stress-neutral profile omits required result/export/run/comparison source-basis references.",
                "Carry DEL-08-04, DEL-14-02, DEL-14-05, and DEL-17-02 as source-basis refs.",
                [_ref("ExportProfile", str(export_profile.get("profile_id", "unknown")))],
            )
        )
    if contains_prohibited_authority_term(export_profile.get("free_metadata", {})):
        diagnostics.append(
            _diagnostic(
                "SN-PROFILE-AUTHORITY-TERM",
                "blocking",
                "IP_BOUNDARY_WARNING",
                "Export profile free metadata contains authority wording.",
                "Remove validation, compatibility, compliance, approval, or professional-reliance wording.",
                    [_ref("ExportProfile", str(export_profile.get("profile_id", "unknown")))],
            )
        )
    for field in (
        "private_payload_embedded",
        "protected_payload_embedded",
        "commercial_tool_payload_embedded",
        "telemetry_allowed",
    ):
        if privacy.get(field) is True:
            diagnostics.append(
                _diagnostic(
                    "SN-PRIVACY-BOUNDARY-VIOLATION",
                    "blocking",
                    "IP_BOUNDARY_WARNING",
                    f"Privacy field {field} is not allowed for public stress-neutral fixtures.",
                    "Reference private/protected/commercial content by governed metadata only.",
                    [package_ref],
                )
            )
    return _stable(diagnostics)


def _ordering_key(value: Any) -> str:
    """Deterministic ordering key for normalized records; never a checksum profile."""

    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def _export_profile(profile: Mapping[str, Any] | None, boundary_notes: list[str]) -> dict[str, Any]:
    profile = dict(profile or {})
    return {
        "profile_id": str(profile.get("profile_id", EXPORT_PROFILE)),
        "profile_version": str(profile.get("profile_version", VERSION)),
        "target_family": "stress_neutral_csv_json",
        "csv_columns": deepcopy(list(profile.get("csv_columns", CSV_COLUMNS))),
        "identity_policy": "canonical_ref_per_row_plus_stable_id_map",
        "unit_policy": "unit_and_dimension_required_per_row",
        "loss_report_policy": "mandatory_for_every_package",
        "comparison_semantics": "diagnostic_export_only_no_pass_fail",
        "boundary_notes": list(profile.get("boundary_notes", boundary_notes)),
        "source_basis_refs": deepcopy(
            list(
                profile.get(
                    "source_basis_refs",
                    [
                        _ref("Deliverable", "DEL-08-04"),
                        _ref("Deliverable", "DEL-14-02"),
                        _ref("Deliverable", "DEL-14-05"),
                        _ref("Deliverable", "DEL-17-02"),
                    ],
                )
            )
        ),
        **({"free_metadata": deepcopy(profile["free_metadata"])} if "free_metadata" in profile else {}),
    }


def _result_rows(rows: list[Mapping[str, Any]], provenance: Mapping[str, Any]) -> list[dict[str, Any]]:
    normalized = []
    for row in rows:
        normalized.append(
            {
                "result_id": str(row.get("result_id", "result:TBD")),
                "canonical_ref": deepcopy(dict(row.get("canonical_ref", _ref("Result", str(row.get("result_id", "result:TBD")))))),
                "row_kind": str(row.get("row_kind", "result_value")),
                "result_family": str(row.get("result_family", "TBD")),
                "load_case_ref": deepcopy(dict(row.get("load_case_ref", _ref("LoadCase", "TBD")))),
                "station_ref": deepcopy(dict(row.get("station_ref", _ref("Station", "TBD")))),
                "component_ref": deepcopy(dict(row.get("component_ref", _ref("Component", "TBD")))),
                "value": row.get("value"),
                "unit": str(row.get("unit", "")),
                "dimension": str(row.get("dimension", "")),
                "source_result_ref": deepcopy(dict(row.get("source_result_ref", _ref("Result", str(row.get("result_id", "result:TBD")))))),
                "correlation_status": str(row.get("correlation_status", "canonical_id_map")),
                "provenance": deepcopy(dict(row.get("provenance", provenance))),
            }
        )
    return sorted(normalized, key=lambda item: item["result_id"])


def _stable_id_map(entries: list[Mapping[str, Any]], provenance: Mapping[str, Any]) -> list[dict[str, Any]]:
    normalized = []
    for entry in entries:
        normalized.append(
            {
                "canonical_ref": deepcopy(dict(entry.get("canonical_ref", _ref("Result", "missing")))),
                "export_ref": deepcopy(dict(entry.get("export_ref", _ref("StressNeutralRow", "missing")))),
                "mapping_status": str(entry.get("mapping_status", "mapped")),
                "loss_category": str(entry.get("loss_category", "exported")),
                "provenance": deepcopy(dict(entry.get("provenance", provenance))),
            }
        )
    return sorted(normalized, key=lambda item: _ordering_key(item["canonical_ref"]))


def _loss_report(
    entries: list[Mapping[str, Any]],
    package_ref: Mapping[str, Any],
    provenance: Mapping[str, Any],
) -> list[dict[str, Any]]:
    if not entries:
        entries = [
            {
                "loss_id": "loss:TBD",
                "category": "tbd",
                "severity": "blocking",
                "affected_refs": [package_ref],
                "target_artifact_ref": package_ref,
                "reason": "Loss report entries were not supplied.",
                "source_basis_ref": _ref("Deliverable", "DEL-17-02"),
                "downstream_implication": "Stress-neutral package cannot be treated as complete until losses are classified.",
            }
        ]
    normalized = []
    for entry in entries:
        normalized.append(
            {
                "loss_id": str(entry.get("loss_id", "loss:TBD")),
                "category": str(entry.get("category", "tbd")),
                "severity": str(entry.get("severity", "warning")),
                "affected_refs": deepcopy(list(entry.get("affected_refs", [package_ref]))),
                "target_artifact_ref": deepcopy(dict(entry.get("target_artifact_ref", package_ref))),
                "reason": str(entry.get("reason", "TBD")),
                "source_basis_ref": deepcopy(dict(entry.get("source_basis_ref", _ref("Deliverable", "DEL-17-02")))),
                "downstream_implication": str(entry.get("downstream_implication", "Requires downstream review.")),
                "human_review_required": True,
                "provenance": deepcopy(dict(entry.get("provenance", provenance))),
            }
        )
    return sorted(normalized, key=lambda item: item["loss_id"])


def _unit_system_disclosure(
    value: Mapping[str, Any] | None,
    *,
    source_model_ref: Mapping[str, Any],
    result_rows: list[Mapping[str, Any]],
    target_export_units: Mapping[str, Any],
    conversion_policy: str,
    conversion_performed: bool,
    conversion_scope: list[str],
    provenance: Mapping[str, Any],
) -> dict[str, Any]:
    record = dict(value or {})
    return {
        "unit_system_ref": deepcopy(dict(record.get("unit_system_ref", _ref("UnitSystem", "unit-system:dec-018-si-dual-display")))),
        "source_model_ref": deepcopy(dict(record.get("source_model_ref", source_model_ref))),
        "storage_convention": str(record.get("storage_convention", "entered_units_preserved")),
        "model_units": _string_record(record.get("model_units", {})),
        "result_units": sorted({str(row.get("unit", "")) for row in result_rows if row.get("unit")}),
        "target_export_units": _string_record(record.get("target_export_units", target_export_units)),
        "conversion_policy": str(record.get("conversion_policy", conversion_policy)),
        "conversion_performed": bool(record.get("conversion_performed", conversion_performed)),
        "conversion_scope": sorted(str(item) for item in _list(record.get("conversion_scope", conversion_scope))),
        "decision_basis_refs": deepcopy(
            list(record.get("decision_basis_refs", [_ref("Decision", "DEC-018"), _ref("Deliverable", "DEL-02-02")]))
        ),
        "protected_content_included": False,
        "private_payload_included": False,
        "provenance": deepcopy(dict(record.get("provenance", provenance))),
    }


def _privacy(value: Mapping[str, Any] | None) -> dict[str, Any]:
    privacy = deepcopy(DEFAULT_PRIVACY)
    privacy.update(dict(value or {}))
    privacy["redaction_refs"] = deepcopy(_list(privacy.get("redaction_refs")))
    return privacy


def _diagnostic(
    code: str,
    severity: str,
    diagnostic_class: str,
    message: str,
    remediation: str,
    affected_refs: list[Mapping[str, Any]],
) -> dict[str, Any]:
    return {
        "code": code,
        "class": diagnostic_class,
        "severity": severity,
        "source": _ref("Deliverable", "DEL-17-06"),
        "affected_object": deepcopy(dict(affected_refs[0] if affected_refs else _ref("Unknown", "unknown"))),
        "message": message,
        "remediation": remediation,
        "provenance": deepcopy(ENGINE_PROVENANCE),
    }


def _ref(object_type: str, value: str) -> dict[str, str]:
    return {"object_type": object_type, "ref": value}


def _list(value: Any) -> list[Any]:
    if value is None:
        return []
    if isinstance(value, list):
        return value
    return [value]


def _string_record(value: Any) -> dict[str, str]:
    if not isinstance(value, Mapping):
        return {}
    return {str(key): str(value[key]) for key in sorted(value)}


def _ref_pairs(refs: Any) -> set[tuple[str, str]]:
    pairs = set()
    for item in _list(refs):
        if isinstance(item, Mapping):
            pairs.add((str(item.get("object_type", "")), str(item.get("ref", ""))))
    return pairs


def _stable(values: list[Any]) -> list[Any]:
    return sorted(values, key=_ordering_key)


def _sort(value: Any) -> Any:
    if isinstance(value, dict):
        return {key: _sort(value[key]) for key in sorted(value)}
    if isinstance(value, list):
        return [_sort(item) for item in value]
    return value


def _normalized_inputs(
    *,
    export_id: str,
    source_result_ref: Mapping[str, Any],
    source_run_ref: Mapping[str, Any],
    source_model_ref: Mapping[str, Any],
    source_hashes: list[Mapping[str, Any]],
    result_rows: list[Mapping[str, Any]],
    stable_id_map: list[Mapping[str, Any]],
    loss_report: list[Mapping[str, Any]],
    unresolved_assumption_refs: list[Mapping[str, Any]] | None = None,
    reproducibility_refs: list[Mapping[str, Any]] | None = None,
    unit_system_disclosure: Mapping[str, Any] | None = None,
    export_profile: Mapping[str, Any] | None = None,
    validation_checks: list[Mapping[str, Any]] | None = None,
    diagnostics: list[Mapping[str, Any]] | None = None,
    privacy: Mapping[str, Any] | None = None,
    provenance: Mapping[str, Any] | None = None,
    boundary_notes: list[str] | None = None,
) -> dict[str, Any]:
    """Normalize current construction inputs without creating a package or checksum."""

    package_ref = _ref("StressNeutralExportPackage", export_id)
    provenance_record = deepcopy(dict(provenance or ENGINE_PROVENANCE))
    privacy_record = _privacy(privacy)
    notes = list(boundary_notes or DEFAULT_BOUNDARY_NOTES)
    assumption_refs = deepcopy(list(unresolved_assumption_refs or []))
    reproduction_refs = deepcopy(list(reproducibility_refs or []))
    profile = _export_profile(export_profile, notes)
    rows = _result_rows(result_rows, provenance_record)
    unit_record = _unit_system_disclosure(
        unit_system_disclosure,
        source_model_ref=source_model_ref,
        result_rows=rows,
        target_export_units={},
        conversion_policy="result_row_units_preserved_no_export_time_conversion",
        conversion_performed=False,
        conversion_scope=[],
        provenance=provenance_record,
    )
    csv_text = _render_utf8_csv(rows)
    normalized_stable_id_map = _stable_id_map(stable_id_map, provenance_record)
    normalized_loss_report = _loss_report(loss_report, package_ref, provenance_record)
    normalized_diagnostics = _stable(
        [deepcopy(dict(item)) for item in diagnostics or []]
        + (
            [
                _diagnostic(
                    "SN-LOSS-REPORT-MISSING",
                    "blocking",
                    "UNSUPPORTED_BEHAVIOR_WARNING",
                    "Stress-neutral package has no supplied loss report entries.",
                    "Supply explicit exported, omitted, approximated, delegated, unsupported, or tbd loss records.",
                    [package_ref],
                )
            ]
            if not loss_report
            else []
        )
        + _boundary_diagnostics(
            result_rows=rows,
            stable_id_map=normalized_stable_id_map,
            loss_report=normalized_loss_report,
            export_profile=profile,
            privacy=privacy_record,
            package_ref=package_ref,
        )
    )
    return _sort({
        "export_id": export_id, "manifest_id": f"{export_id}:manifest", "boundary_notes": notes,
        "source_result_ref": deepcopy(dict(source_result_ref)), "source_run_ref": deepcopy(dict(source_run_ref)), "source_model_ref": deepcopy(dict(source_model_ref)),
        "unresolved_assumption_refs": assumption_refs, "reproducibility_refs": reproduction_refs,
        "export_profile": profile, "unit_system_disclosure": unit_record, "result_rows": rows, "csv_text": csv_text,
        "stable_id_map": normalized_stable_id_map, "loss_report": normalized_loss_report, "diagnostics": normalized_diagnostics,
        "privacy": privacy_record, "provenance": provenance_record, "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
    })
