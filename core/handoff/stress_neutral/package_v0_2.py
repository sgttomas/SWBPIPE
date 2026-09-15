"""Strict stress-neutral 0.2 package construction.

The 0.1 builder remains byte-stable in ``package.py``. This module decorates
its normalized data into the versioned nine-member checked-hash contract.
"""
from __future__ import annotations
from copy import deepcopy
from pathlib import Path
from typing import Any, Mapping

from core.serialization.canonical_json.adapter import canonical_json_checked_v1, canonical_sha256_checked_v1
from .package import build_stress_neutral_export_package

VERSION = "0.2.0"
PROFILE = "openpipestress_jcs_ijson_v1"
EXPORT_PROFILE = "ops.stress_neutral.v2"
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
    supplied_profile = source.get("export_profile")
    if isinstance(supplied_profile, Mapping):
        if "profile_id" in supplied_profile and supplied_profile["profile_id"] != EXPORT_PROFILE:
            raise ValueError("SN-0.2-PROFILE-IDENTITY-MISMATCH")
        if "profile_version" in supplied_profile and supplied_profile["profile_version"] != VERSION:
            raise ValueError("SN-0.2-PROFILE-IDENTITY-MISMATCH")
    legacy = build_stress_neutral_export_package(**deepcopy(source))
    witnesses, witness_findings = _witnesses(legacy["result_rows"], legacy["provenance"])
    diagnostics = deepcopy(legacy["diagnostics"]) + witness_findings
    package: dict[str, Any] = {
        "schema_version": VERSION, "deliverable_id": "DEL-17-06", "package_id": "PKG-17", "scope_items": ["SOW-046", "SOW-074"], "objectives": ["OBJ-007", "OBJ-017", "OBJ-018"],
        "export_id": legacy["export_id"], "package_status": "stress_neutral_export_package", "schema_conformant": True,
        "validation_ready": not any(item.get("severity") == "blocking" for item in diagnostics),
        "source_result_ref": legacy["source_result_ref"], "source_run_ref": legacy["source_run_ref"], "source_model_ref": legacy["source_model_ref"],
        "received_source_checksums": deepcopy(source.get("source_hashes", [])), "unresolved_assumption_refs": legacy["unresolved_assumption_refs"], "reproducibility_refs": legacy["reproducibility_refs"],
        "export_profile": {**legacy["export_profile"], "profile_id": EXPORT_PROFILE, "profile_version": VERSION}, "manifest": {"manifest_id": legacy["manifest"]["manifest_id"], "export_profile_ref": {"object_type": "StressNeutralExportProfile", "ref": EXPORT_PROFILE}, "boundary_notes": legacy["manifest"]["boundary_notes"], "package_members": [], "checksums": []},
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


def write_materialized_members_v0_2(package: Mapping[str, Any], output_dir: str | Path) -> list[Path]:
    """Validate and write the exact nine contracted member byte streams."""
    validate_stress_neutral_export_package_v0_2(package)
    destination = Path(output_dir)
    destination.mkdir(parents=True, exist_ok=True)
    written = []
    for filename, payload in materialized_members_v0_2(package).items():
        target = destination / filename
        target.write_bytes(payload)
        written.append(target)
    return written


def validate_stress_neutral_export_package_v0_2(package: Mapping[str, Any]) -> None:
    if package.get("schema_version") != VERSION:
        raise ValueError("SN-VERSION-UNSUPPORTED")
    if package.get("schema_conformant") is not True:
        raise ValueError("SN-SCHEMA-CONFORMANCE-CLAIM-INVALID")
    if package.get("export_profile", {}).get("profile_id") != EXPORT_PROFILE or package.get("export_profile", {}).get("profile_version") != VERSION or package.get("manifest", {}).get("export_profile_ref") != {"object_type": "StressNeutralExportProfile", "ref": EXPORT_PROFILE}:
        raise ValueError("SN-PROFILE-IDENTITY-MISMATCH")
    members = materialized_members_v0_2(package)
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
    from .package import render_stress_neutral_csv
    if package.get("csv_text") != render_stress_neutral_csv(rows):
        raise ValueError("SN-CSV-ROW-BINDING-MISMATCH")
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
        quantity = {"value": row.get("value"), "unit": row.get("unit"), "dimension": row.get("dimension")} if row else None
        if row is None or witness.get("result_id") in seen_witnesses or witness.get("result_id") != row.get("result_id") or witness.get("source_quantity") != quantity or witness.get("target_quantity") != quantity or witness.get("conversion_performed") is not False or witness.get("policy") != "preserve_received_value_and_unit":
            raise ValueError("SN-UNIT-WITNESS-BINDING-MISMATCH")
        seen_witnesses.add(witness["result_id"])
    for entry in package.get("loss_report", []):
        if entry.get("target_artifact_ref", {}).get("ref") != package.get("export_id") or entry.get("human_review_required") is not True:
            raise ValueError("SN-LOSS-REPORT-BINDING-MISMATCH")
    diagnostic_work = {row.get("result_id") for row in rows if row.get("row_kind") in {"diagnostic_work", "work"} or row.get("result_family") == "diagnostic_work"}
    withheld = {item.get("source", {}).get("ref") for item in package.get("diagnostics", []) if item.get("code") == "SN-UNIT-WITNESS-WITHHELD-DIAGNOSTIC-WORK"}
    if diagnostic_work != withheld or diagnostic_work & seen_witnesses:
        raise ValueError("SN-DIAGNOSTIC-WORK-FINDING-MISMATCH")
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
