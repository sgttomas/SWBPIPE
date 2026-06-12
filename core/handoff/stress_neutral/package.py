"""Stress-neutral CSV/JSON export package builder for DEL-17-06.

The package is a project-owned review and regression artifact. It preserves
result identity, units, source refs, stable IDs, loss reporting, diagnostics,
provenance, privacy, and professional-boundary notices without becoming a
vendor format, solver input deck, validation record, code-compliance record, or
professional reliance record.
"""

from __future__ import annotations

from copy import deepcopy
import csv
import hashlib
import io
import json
from pathlib import Path
from typing import Any, Mapping

from core.handoff.external_prover.authority_boundary import (
    contains_prohibited_authority_term,
)


STRESS_NEUTRAL_EXPORT_VERSION = "0.1.0"
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

CSV_COLUMNS = [
    "result_id",
    "canonical_ref",
    "row_kind",
    "result_family",
    "load_case_ref",
    "station_ref",
    "component_ref",
    "value",
    "unit",
    "dimension",
    "correlation_status",
]

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


def build_stress_neutral_export_package(
    *,
    export_id: str,
    source_result_ref: Mapping[str, Any],
    source_run_ref: Mapping[str, Any],
    source_model_ref: Mapping[str, Any],
    source_hashes: list[Mapping[str, Any]],
    result_rows: list[Mapping[str, Any]],
    stable_id_map: list[Mapping[str, Any]],
    loss_report: list[Mapping[str, Any]],
    unit_system_disclosure: Mapping[str, Any] | None = None,
    export_profile: Mapping[str, Any] | None = None,
    validation_checks: list[Mapping[str, Any]] | None = None,
    diagnostics: list[Mapping[str, Any]] | None = None,
    privacy: Mapping[str, Any] | None = None,
    provenance: Mapping[str, Any] | None = None,
    boundary_notes: list[str] | None = None,
) -> dict[str, Any]:
    """Build a deterministic stress-neutral CSV/JSON export package."""

    package_ref = _ref("StressNeutralExportPackage", export_id)
    provenance_record = deepcopy(dict(provenance or ENGINE_PROVENANCE))
    privacy_record = _privacy(privacy)
    notes = list(boundary_notes or DEFAULT_BOUNDARY_NOTES)
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
    csv_text = render_stress_neutral_csv(rows)
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
        + diagnostics_for_stress_neutral_export_package(
            result_rows=rows,
            stable_id_map=normalized_stable_id_map,
            loss_report=normalized_loss_report,
            export_profile=profile,
            privacy=privacy_record,
            package_ref=package_ref,
        )
    )
    validation_report = _validation_report(validation_checks, normalized_diagnostics, provenance_record)
    checksums = {
        "csv_text": _text_checksum(csv_text, _ref("StressNeutralMember", f"{export_id}:csv_text")),
        "result_rows": _checksum(rows, _ref("StressNeutralMember", f"{export_id}:result_rows"), "stress_neutral_result_rows"),
        "unit_system_disclosure": _checksum(
            unit_record,
            _ref("StressNeutralMember", f"{export_id}:unit_system_disclosure"),
            "stress_neutral_unit_system_disclosure",
        ),
        "stable_id_map": _checksum(
            normalized_stable_id_map,
            _ref("StressNeutralMember", f"{export_id}:stable_id_map"),
            "stress_neutral_stable_id_map",
        ),
        "loss_report": _checksum(
            normalized_loss_report,
            _ref("StressNeutralMember", f"{export_id}:loss_report"),
            "stress_neutral_loss_report",
        ),
        "validation_report": _checksum(
            validation_report,
            _ref("StressNeutralMember", f"{export_id}:validation_report"),
            "stress_neutral_validation_report",
        ),
        "diagnostics": _checksum(
            normalized_diagnostics,
            _ref("StressNeutralMember", f"{export_id}:diagnostics"),
            "stress_neutral_diagnostics",
        ),
    }
    manifest_seed = {
        "manifest_id": f"{export_id}:manifest",
        "source_result_ref": deepcopy(dict(source_result_ref)),
        "source_run_ref": deepcopy(dict(source_run_ref)),
        "source_model_ref": deepcopy(dict(source_model_ref)),
        "source_hashes": deepcopy(list(source_hashes)),
        "export_profile_ref": _ref("ExportProfile", profile["profile_id"]),
        "boundary_notes": notes,
        "member_hashes": checksums,
        "diagnostics": normalized_diagnostics,
    }
    checksums["manifest"] = _checksum(
        manifest_seed,
        _ref("StressNeutralMember", f"{export_id}:manifest"),
        "stress_neutral_manifest",
    )
    manifest = {
        "manifest_id": f"{export_id}:manifest",
        "source_result_ref": deepcopy(dict(source_result_ref)),
        "source_run_ref": deepcopy(dict(source_run_ref)),
        "source_model_ref": deepcopy(dict(source_model_ref)),
        "source_hashes": deepcopy(list(source_hashes)),
        "export_profile_ref": _ref("ExportProfile", profile["profile_id"]),
        "package_members": _package_members(export_id, checksums),
        "checksums": _stable(list(checksums.values()) + deepcopy(list(source_hashes))),
        "boundary_notes": notes,
        "diagnostics": normalized_diagnostics,
    }
    package = {
        "schema_version": STRESS_NEUTRAL_EXPORT_VERSION,
        "deliverable_id": "DEL-17-06",
        "package_id": "PKG-17",
        "scope_items": ["SOW-046", "SOW-074"],
        "objectives": ["OBJ-007", "OBJ-017", "OBJ-018"],
        "export_id": export_id,
        "package_status": "stress_neutral_export_package",
        "source_result_ref": deepcopy(dict(source_result_ref)),
        "source_run_ref": deepcopy(dict(source_run_ref)),
        "source_model_ref": deepcopy(dict(source_model_ref)),
        "source_hashes": deepcopy(list(source_hashes)),
        "export_profile": profile,
        "manifest": manifest,
        "unit_system_disclosure": unit_record,
        "result_rows": rows,
        "csv_text": csv_text,
        "stable_id_map": normalized_stable_id_map,
        "loss_report": normalized_loss_report,
        "validation_report": validation_report,
        "diagnostics": normalized_diagnostics,
        "privacy": privacy_record,
        "provenance": provenance_record,
        "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
    }
    return _sort(package)


def render_stress_neutral_csv(result_rows: list[Mapping[str, Any]]) -> str:
    """Render deterministic stress-neutral CSV text from normalized rows."""

    output = io.StringIO()
    writer = csv.DictWriter(output, fieldnames=CSV_COLUMNS, lineterminator="\n", extrasaction="ignore")
    writer.writeheader()
    for row in sorted(result_rows, key=lambda item: str(item.get("result_id", ""))):
        writer.writerow(
            {
                "result_id": row.get("result_id", ""),
                "canonical_ref": row.get("canonical_ref", {}).get("ref", ""),
                "row_kind": row.get("row_kind", ""),
                "result_family": row.get("result_family", ""),
                "load_case_ref": row.get("load_case_ref", {}).get("ref", ""),
                "station_ref": row.get("station_ref", {}).get("ref", ""),
                "component_ref": row.get("component_ref", {}).get("ref", ""),
                "value": _csv_value(row.get("value")),
                "unit": row.get("unit", ""),
                "dimension": row.get("dimension", ""),
                "correlation_status": row.get("correlation_status", ""),
            }
        )
    return canonical_csv(output.getvalue())


def diagnostics_for_stress_neutral_export_package(
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


def canonical_json(value: Any) -> str:
    """Serialize package values with deterministic JSON key ordering."""

    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def canonical_csv(value: str) -> str:
    """Normalize CSV text for deterministic hashing and writing."""

    lines = [line.rstrip() for line in value.replace("\r\n", "\n").replace("\r", "\n").split("\n")]
    while lines and lines[-1] == "":
        lines.pop()
    return "\n".join(lines) + "\n"


def write_stress_neutral_export_package(directory: str | Path, package: Mapping[str, Any]) -> None:
    """Write CSV plus JSON sidecars using deterministic encodings."""

    root = Path(directory)
    root.mkdir(parents=True, exist_ok=True)
    (root / "stress_neutral_results.csv").write_text(str(package["csv_text"]), encoding="ascii")
    for key, filename in (
        ("manifest", "manifest.json"),
        ("result_rows", "result_rows.json"),
        ("stable_id_map", "stable_id_map.json"),
        ("loss_report", "loss_report.json"),
        ("validation_report", "validation_report.json"),
        ("diagnostics", "diagnostics.json"),
    ):
        (root / filename).write_text(canonical_json(package[key]) + "\n", encoding="utf-8")


def _export_profile(profile: Mapping[str, Any] | None, boundary_notes: list[str]) -> dict[str, Any]:
    profile = dict(profile or {})
    return {
        "profile_id": str(profile.get("profile_id", "ops.stress_neutral.v1")),
        "profile_version": str(profile.get("profile_version", STRESS_NEUTRAL_EXPORT_VERSION)),
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
    return sorted(normalized, key=lambda item: canonical_json(item["canonical_ref"]))


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


def _validation_report(
    validation_checks: list[Mapping[str, Any]] | None,
    diagnostics: list[Mapping[str, Any]],
    provenance: Mapping[str, Any],
) -> dict[str, Any]:
    checks = deepcopy(list(validation_checks or []))
    blocking = [item for item in diagnostics if item.get("severity") == "blocking"]
    checks.append(
        {
            "check_id": "stress-neutral-boundary-diagnostics",
            "check_status": "blocked" if blocking else "passed",
            "diagnostic_count": len(diagnostics),
            "blocking_count": len(blocking),
            "provenance": deepcopy(dict(provenance)),
        }
    )
    return {
        "validation_status": "blocked" if blocking else "passed",
        "checks": checks,
        "human_review_required": True,
        "provenance": deepcopy(dict(provenance)),
    }


def _package_members(export_id: str, checksums: Mapping[str, Mapping[str, Any]]) -> list[dict[str, Any]]:
    filenames = {
        "manifest": "manifest.json",
        "csv_text": "stress_neutral_results.csv",
        "result_rows": "result_rows.json",
        "unit_system_disclosure": "unit_system_disclosure.json",
        "stable_id_map": "stable_id_map.json",
        "loss_report": "loss_report.json",
        "validation_report": "validation_report.json",
        "diagnostics": "diagnostics.json",
    }
    return [
        {
            "member_id": f"{export_id}:{role}",
            "member_role": role,
            "path": filenames[role],
            "checksum": deepcopy(dict(checksums[role])),
        }
        for role in sorted(filenames)
    ]


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


def _checksum(value: Any, payload_ref: Mapping[str, Any], payload_scope: str) -> dict[str, Any]:
    return {
        "algorithm": "sha256",
        "canonicalization": "JCS_compatible_json_payload_hash",
        "payload_ref": deepcopy(dict(payload_ref)),
        "payload_scope": payload_scope,
        "value": "sha256:" + hashlib.sha256(canonical_json(value).encode("utf-8")).hexdigest(),
    }


def _text_checksum(value: str, payload_ref: Mapping[str, Any]) -> dict[str, Any]:
    return {
        "algorithm": "sha256",
        "canonicalization": "normalized_ascii_lf_text",
        "payload_ref": deepcopy(dict(payload_ref)),
        "payload_scope": "stress_neutral_csv_text",
        "value": "sha256:" + hashlib.sha256(value.encode("utf-8")).hexdigest(),
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


def _csv_value(value: Any) -> str:
    if isinstance(value, float) and value.is_integer():
        return str(int(value))
    return "" if value is None else str(value)


def _stable(values: list[Any]) -> list[Any]:
    return sorted(values, key=canonical_json)


def _sort(value: Any) -> Any:
    if isinstance(value, dict):
        return {key: _sort(value[key]) for key in sorted(value)}
    if isinstance(value, list):
        return [_sort(item) for item in value]
    return value
