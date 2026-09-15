"""Strict stress-neutral 0.2 package construction.

The 0.1 builder remains byte-stable in ``package.py``. This module decorates
its normalized data into the versioned nine-member checked-hash contract.
"""
from __future__ import annotations
from copy import deepcopy
from typing import Any, Mapping

from core.serialization.canonical_json.adapter import canonical_json_checked_v1, canonical_sha256_checked_v1
from .package import build_stress_neutral_export_package

VERSION = "0.2.0"
PROFILE = "openpipestress_jcs_ijson_v1"
MEMBERS = ["manifest.json", "stress_neutral_results.csv", "result_rows.json", "unit_system_disclosure.json", "unit_preservation_witnesses.json", "stable_id_map.json", "loss_report.json", "validation_report.json", "diagnostics.json"]


def _checked(payload: Any, filename: str, scope: str = "member_payload") -> dict[str, Any]:
    return {"algorithm": "sha256", "canonicalization": PROFILE, "payload_scope": scope, "payload_ref": {"object_type": "StressNeutralMember", "ref": filename}, "value": canonical_sha256_checked_v1(payload)}


def _witnesses(rows: list[dict[str, Any]], provenance: Mapping[str, Any]) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    witnesses, findings = [], []
    for index, row in enumerate(rows):
        diagnostic_work = row.get("row_kind") in {"diagnostic_work", "work"} or row.get("result_family") == "diagnostic_work"
        if diagnostic_work:
            findings.append({"code": "SN-UNIT-WITNESS-WITHHELD-DIAGNOSTIC-WORK", "class": "unit_preservation_witness", "severity": "info", "source": {"object_type": "StressNeutralResultRow", "ref": str(row.get("result_id"))}, "affected_object": {"object_type": "StressNeutralUnitWitness", "ref": f"unit-witness:{index}"}, "message": "Diagnostic work evidence is retained as a row but has no physical unit-preservation witness.", "remediation": "Review diagnostic work separately from physical quantity witnesses.", "provenance": deepcopy(dict(provenance))})
            continue
        witnesses.append({"witness_id": f"unit-witness:{index}", "source_row_index": index, "result_id": row.get("result_id"), "source_quantity": {"value": row.get("value"), "unit": row.get("unit"), "dimension": row.get("dimension")}, "target_quantity": {"value": row.get("value"), "unit": row.get("unit"), "dimension": row.get("dimension")}, "conversion_performed": False, "policy": "preserve_received_value_and_unit"})
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


def build_stress_neutral_export_package_v0_2(**source: Any) -> dict[str, Any]:
    legacy = build_stress_neutral_export_package(**deepcopy(source))
    witnesses, witness_findings = _witnesses(legacy["result_rows"], legacy["provenance"])
    diagnostics = deepcopy(legacy["diagnostics"]) + witness_findings
    package: dict[str, Any] = {
        "schema_version": VERSION, "deliverable_id": "DEL-17-06", "package_id": "PKG-17", "scope_items": ["SOW-046", "SOW-074"], "objectives": ["OBJ-007", "OBJ-017", "OBJ-018"],
        "export_id": legacy["export_id"], "package_status": "stress_neutral_export_package", "schema_conformant": True,
        "validation_ready": not any(item.get("severity") == "blocking" for item in diagnostics),
        "source_result_ref": legacy["source_result_ref"], "source_run_ref": legacy["source_run_ref"], "source_model_ref": legacy["source_model_ref"],
        "received_source_checksums": deepcopy(source.get("source_hashes", [])), "unresolved_assumption_refs": legacy["unresolved_assumption_refs"], "reproducibility_refs": legacy["reproducibility_refs"],
        "export_profile": legacy["export_profile"], "manifest": {"manifest_id": legacy["manifest"]["manifest_id"], "export_profile_ref": legacy["manifest"]["export_profile_ref"], "boundary_notes": legacy["manifest"]["boundary_notes"], "package_members": [], "checksums": []},
        "csv_text": legacy["csv_text"], "result_rows": legacy["result_rows"], "unit_system_disclosure": legacy["unit_system_disclosure"], "unit_preservation_witnesses": witnesses,
        "stable_id_map": legacy["stable_id_map"], "loss_report": legacy["loss_report"], "validation_report": legacy["validation_report"], "diagnostics": diagnostics,
        "privacy": legacy["privacy"], "provenance": legacy["provenance"], "professional_boundary": legacy["professional_boundary"],
    }
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
    validate_stress_neutral_export_package_v0_2(package)
    return package


def materialized_members_v0_2(package: Mapping[str, Any]) -> dict[str, bytes]:
    values = {"manifest.json": package["manifest"], "stress_neutral_results.csv": package["csv_text"], "result_rows.json": package["result_rows"], "unit_system_disclosure.json": package["unit_system_disclosure"], "unit_preservation_witnesses.json": package["unit_preservation_witnesses"], "stable_id_map.json": package["stable_id_map"], "loss_report.json": package["loss_report"], "validation_report.json": package["validation_report"], "diagnostics.json": package["diagnostics"]}
    return {name: (value.encode("ascii") if name.endswith(".csv") else canonical_json_checked_v1(value).encode("utf-8")) for name, value in values.items()}


def validate_stress_neutral_export_package_v0_2(package: Mapping[str, Any]) -> None:
    if package.get("schema_version") != VERSION:
        raise ValueError("SN-VERSION-UNSUPPORTED")
    members = materialized_members_v0_2(package)
    if list(members) != MEMBERS:
        raise ValueError("SN-MEMBER-INVENTORY-MISMATCH")
    checksums = package["manifest"]["checksums"]
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
        if hashlib.sha256(members[filename]).hexdigest() != item["value"]:
            raise ValueError(f"SN-MEMBER-CHECKSUM-MISMATCH: {filename}")
    expected_package = canonical_sha256_checked_v1(package_projection(package))
    if package.get("package_checksum", {}).get("value") != expected_package:
        raise ValueError("SN-PACKAGE-CHECKSUM-MISMATCH")
