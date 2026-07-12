"""Optional CAEPIPE external-run evidence package for DEL-17-05.

This module records deterministic evidence for skipped, parser-only, or
user-owned external CAEPIPE run attempts. It does not discover, install, bundle,
license, or invoke CAEPIPE by itself, and it does not treat parsed output as
compatibility, validation, code-compliance, or professional evidence.
"""

from __future__ import annotations

import csv
from copy import deepcopy
import hashlib
import io
import json
from pathlib import Path
from typing import Any, Mapping

from core.handoff.external_prover.authority_boundary import (
    contains_prohibited_authority_term,
)


CAEPIPE_EXTERNAL_RUN_VERSION = "0.1.0"

PACKAGE_STATUSES = {
    "skipped_no_executable",
    "parser_only_evidence",
    "external_run_evidence_blocked",
    "external_run_evidence_recorded",
}

ENGINE_PROVENANCE = {
    "source_name": "OpenPipeStress DEL-17-05 CAEPIPE external-run evidence builder",
    "source_location": "core/handoff/caepipe_external/run.py",
    "source_license": "project-governed",
    "contributor": "OpenPipeStress Type 2 worker",
    "contributor_certification": "implementation-only-no-professional-claim",
    "redistribution_status": "public_permissive",
    "review_status": "machine_checked",
    "privacy_classification": "public_metadata",
}

PROFESSIONAL_BOUNDARY = {
    "human_review_required": True,
    "supports_regression_evidence": True,
    "supports_handoff_review_evidence": True,
    "software_bundles_caepipe": False,
    "software_invokes_caepipe_without_user_configuration": False,
    "software_makes_release_claim": False,
    "software_makes_caepipe_compatibility_claim": False,
    "software_makes_solver_validation_claim": False,
    "software_makes_compliance_claim": False,
    "software_makes_certification_claim": False,
    "software_makes_sealing_claim": False,
    "software_makes_approval_claim": False,
    "software_creates_professional_reliance_record": False,
}

DEFAULT_PRIVACY = {
    "classification": "private_user_controlled",
    "local_only": True,
    "telemetry_allowed": False,
    "private_payload_embedded": False,
    "protected_payload_embedded": False,
    "commercial_tool_payload_embedded": False,
    "redaction_refs": [],
}

DEFAULT_BOUNDARY_NOTES = [
    "CAEPIPE external execution is optional and user-owned.",
    "Public tests use skipped or parser-only evidence and do not require a CAEPIPE executable.",
    "Parsed CSV output is regression and handoff evidence only.",
    "Evidence does not assert CAEPIPE compatibility, solver validation, release readiness, code compliance, or professional reliance.",
]

DEFAULT_PARSER_COVERAGE = [
    {
        "section_name": "NODE_DISPLACEMENTS",
        "basis": "invented_public_fixture",
        "support_status": "parser_only_supported",
        "expected_columns": ["stable_id", "target_id", "load_case", "ux", "uy", "uz", "unit"],
        "unmapped_row_policy": "diagnostic_warning",
        "diagnostic_severity": "warning",
        "fixture_provenance": "fixtures/caepipe_external/invented/caepipe_results.csv",
    },
    {
        "section_name": "ELEMENT_FORCES",
        "basis": "invented_public_fixture",
        "support_status": "parser_only_supported",
        "expected_columns": ["stable_id", "target_id", "load_case", "axial", "shear_y", "shear_z", "unit"],
        "unmapped_row_policy": "diagnostic_warning",
        "diagnostic_severity": "warning",
        "fixture_provenance": "fixtures/caepipe_external/invented/caepipe_results.csv",
    },
]

REQUIRED_MBF_REF = {"object_type": "CaePipeMbfExportPackage", "ref": "caepipe-mbf:invented-del-17-04"}


def build_skipped_caepipe_external_run_package(
    *,
    run_id: str,
    mbf_package_ref: Mapping[str, Any],
    executable_config: Mapping[str, Any] | None = None,
    provenance: Mapping[str, Any] | None = None,
    privacy: Mapping[str, Any] | None = None,
    boundary_notes: list[str] | None = None,
) -> dict[str, Any]:
    """Build deterministic skip evidence when no user executable is configured."""

    return build_caepipe_external_run_package(
        run_id=run_id,
        package_status="skipped_no_executable",
        mbf_package_ref=mbf_package_ref,
        executable_config=executable_config
        or {
            "configuration_surface": "TBD-17-05-PH-001",
            "configured_path_state": "absent",
            "license_responsibility_acknowledged": False,
            "environment_responsibility_acknowledged": False,
            "path_record": "not_configured",
        },
        command_profile={
            "profile_id": "TBD-17-05-invocation-profile",
            "invocation_mode": "not_invoked",
            "command_shape": "not_invoked_without_user_executable",
            "profile_basis": "TBD-17-05-PH-001",
        },
        run_directory={
            "run_directory_ref": "not_created",
            "working_directory": "not_created",
            "input_mbf_path": "not_invoked",
            "expected_csv_path": "not_invoked",
            "observed_csv_path": None,
            "output_discovery_status": "not_attempted",
        },
        execution_result={
            "attempted": False,
            "exit_status": None,
            "stdout_capture": "not_available",
            "stderr_capture": "not_available",
            "skip_reason": "No user-owned CAEPIPE executable path was configured.",
        },
        parsed_csv=None,
        provenance=provenance,
        privacy=privacy,
        boundary_notes=boundary_notes,
    )


def build_caepipe_external_run_package(
    *,
    run_id: str,
    package_status: str,
    mbf_package_ref: Mapping[str, Any],
    executable_config: Mapping[str, Any],
    command_profile: Mapping[str, Any],
    run_directory: Mapping[str, Any],
    execution_result: Mapping[str, Any],
    parsed_csv: Mapping[str, Any] | None = None,
    parser_coverage: list[Mapping[str, Any]] | None = None,
    diagnostics: list[Mapping[str, Any]] | None = None,
    privacy: Mapping[str, Any] | None = None,
    provenance: Mapping[str, Any] | None = None,
    boundary_notes: list[str] | None = None,
) -> dict[str, Any]:
    """Build a deterministic CAEPIPE external-run evidence package."""

    package_ref = _ref("CaePipeExternalRunPackage", run_id)
    provenance_record = deepcopy(dict(provenance or ENGINE_PROVENANCE))
    privacy_record = _privacy(privacy)
    notes = list(boundary_notes or DEFAULT_BOUNDARY_NOTES)
    normalized_parsed = _parsed_csv(parsed_csv, provenance_record)
    coverage = _parser_coverage(parser_coverage, provenance_record)
    normalized = {
        "schema_version": CAEPIPE_EXTERNAL_RUN_VERSION,
        "deliverable_id": "DEL-17-05",
        "package_id": "PKG-17",
        "scope_items": ["SOW-030", "SOW-046", "SOW-075"],
        "objectives": ["OBJ-007", "OBJ-009", "OBJ-017", "OBJ-018"],
        "run_id": str(run_id),
        "package_status": str(package_status),
        "mbf_package_ref": deepcopy(dict(mbf_package_ref)),
        "executable_config": _executable_config(executable_config),
        "command_profile": _command_profile(command_profile),
        "run_directory": _run_directory(run_directory),
        "execution_result": _execution_result(execution_result),
        "parser_coverage": coverage,
        "parsed_csv": normalized_parsed,
        "boundary_notes": notes,
        "privacy": privacy_record,
        "provenance": provenance_record,
        "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
    }
    normalized["diagnostics"] = _stable(
        [deepcopy(dict(item)) for item in diagnostics or []]
        + _privacy_override_diagnostics(privacy, package_ref, provenance_record)
        + diagnostics_for_caepipe_external_run_package(normalized)
    )
    normalized["checksums"] = _checksums(run_id, normalized)
    return _sort(normalized)


def parse_caepipe_csv_text(
    csv_text: str,
    *,
    source_csv_ref: Mapping[str, Any] | None = None,
    stable_id_map: list[Mapping[str, Any]] | None = None,
    provenance: Mapping[str, Any] | None = None,
) -> dict[str, Any]:
    """Parse the invented DEL-17-05 CSV evidence fixture into neutral rows."""

    provenance_record = deepcopy(dict(provenance or ENGINE_PROVENANCE))
    source_ref = deepcopy(dict(source_csv_ref or _ref("CsvArtifact", "csv:invented-caepipe-results")))
    canonical = canonical_csv(csv_text)
    stable_refs = _stable_refs(stable_id_map or [])
    rows: list[dict[str, Any]] = []
    diagnostics: list[dict[str, Any]] = []

    reader = csv.DictReader(io.StringIO(canonical))
    for index, row in enumerate(reader, start=2):
        section = str(row.get("section", "")).strip()
        stable_id = str(row.get("stable_id", "")).strip()
        target_id = str(row.get("target_id", "")).strip()
        load_case = str(row.get("load_case", "")).strip()
        unit = str(row.get("unit", "")).strip()
        values = {
            key: _csv_number(value)
            for key, value in sorted(row.items())
            if key not in {"section", "stable_id", "target_id", "load_case", "unit"} and value not in (None, "")
        }
        correlation = "canonical_id_map" if stable_id in stable_refs else "weak_or_unmapped"
        parsed_row = {
            "row_number": index,
            "section": section,
            "stable_id": stable_id,
            "target_id": target_id,
            "load_case": load_case,
            "unit": unit,
            "values": values,
            "correlation_status": correlation,
            "source_csv_ref": source_ref,
            "provenance": deepcopy(provenance_record),
        }
        rows.append(parsed_row)
        if section not in {"NODE_DISPLACEMENTS", "ELEMENT_FORCES"}:
            diagnostics.append(
                _diagnostic(
                    "CAEPIPE-CSV-UNKNOWN-SECTION",
                    "warning",
                    "PARSER_COVERAGE_WARNING",
                    f"CSV row {index} uses an unsupported or unconfirmed section.",
                    "Carry unknown sections as parser diagnostics until source or fixture coverage is confirmed.",
                    [_ref("CsvRow", str(index))],
                    provenance_record,
                )
            )
        if correlation != "canonical_id_map":
            diagnostics.append(
                _diagnostic(
                    "CAEPIPE-CSV-UNMAPPED-ROW",
                    "warning",
                    "TARGET_MAPPING_WARNING",
                    f"CSV row {index} is not bound to a known canonical stable ID.",
                    "Use DEL-17-04 sidecar stable-ID mapping before treating parsed rows as correlated evidence.",
                    [_ref("CsvRow", str(index))],
                    provenance_record,
                )
            )

    if not rows:
        diagnostics.append(
            _diagnostic(
                "CAEPIPE-CSV-NO-ROWS",
                "blocking",
                "PARSER_COVERAGE_WARNING",
                "CSV parser found no data rows.",
                "Provide an invented or rights-cleared CSV fixture with parser-covered rows.",
                [source_ref],
                provenance_record,
            )
        )

    return _sort(
        {
            "source_csv_ref": source_ref,
            "parser_status": "parsed_with_diagnostics"
            if diagnostics
            else "parsed_parser_only_fixture",
            "row_count": len(rows),
            "rows": rows,
            "diagnostics": _stable(diagnostics),
            "csv_checksum": _text_checksum(canonical, source_ref, "caepipe_csv_text"),
            "provenance": provenance_record,
        }
    )


def diagnostics_for_caepipe_external_run_package(package: Mapping[str, Any]) -> list[dict[str, Any]]:
    """Return deterministic diagnostics for CAEPIPE external-run evidence."""

    provenance = deepcopy(dict(package.get("provenance", ENGINE_PROVENANCE)))
    package_ref = _ref("CaePipeExternalRunPackage", str(package.get("run_id", "unknown")))
    diagnostics: list[dict[str, Any]] = []
    status = package.get("package_status")
    if status not in PACKAGE_STATUSES:
        diagnostics.append(
            _diagnostic(
                "CAEPIPE-RUN-STATUS-UNSUPPORTED",
                "blocking",
                "RUN_CONTRACT_WARNING",
                "External-run package status is outside the DEL-17-05 contract.",
                "Use skipped_no_executable, parser_only_evidence, external_run_evidence_blocked, or external_run_evidence_recorded.",
                [package_ref],
                provenance,
            )
        )
    mbf_ref = package.get("mbf_package_ref")
    if not mbf_ref:
        diagnostics.append(
            _diagnostic(
                "CAEPIPE-RUN-MBF-REF-MISSING",
                "blocking",
                "EXPORT_BLOCKING",
                "External-run evidence has no DEL-17-04 MBF package reference.",
                "Bind run evidence to the accepted DEL-17-04 MBF package manifest and sidecars.",
                [package_ref],
                provenance,
            )
        )
    elif mbf_ref.get("object_type") != "CaePipeMbfExportPackage" or "del-17-04" not in str(
        mbf_ref.get("ref", "")
    ).lower():
        diagnostics.append(
            _diagnostic(
                "CAEPIPE-RUN-MBF-REF-UNSAFE",
                "blocking",
                "EXPORT_BLOCKING",
                "External-run evidence is not bound to a DEL-17-04 CAEPIPE MBF export package reference.",
                "Bind CAEPIPE external-run evidence to the accepted DEL-17-04 MBF package manifest and sidecars.",
                [package_ref],
                provenance,
            )
        )
    executable = package.get("executable_config", {})
    attempted = bool(package.get("execution_result", {}).get("attempted"))
    path_state = executable.get("configured_path_state")
    if attempted and path_state != "present":
        diagnostics.append(
            _diagnostic(
                "CAEPIPE-RUN-EXECUTABLE-NOT-PRESENT",
                "blocking",
                "EXECUTION_BOUNDARY_WARNING",
                "Execution is marked attempted without a present user-owned executable path.",
                "Do not attempt external execution unless the user supplied a local executable path.",
                [package_ref],
                provenance,
            )
        )
    if attempted and not executable.get("license_responsibility_acknowledged"):
        diagnostics.append(
            _diagnostic(
                "CAEPIPE-RUN-LICENSE-RESPONSIBILITY-MISSING",
                "blocking",
                "EXECUTION_BOUNDARY_WARNING",
                "External execution is attempted without an acknowledged user/license responsibility record.",
                "Require user-owned executable configuration and license responsibility acknowledgement before recording an attempted run.",
                [package_ref],
                provenance,
            )
        )
    if attempted and not executable.get("environment_responsibility_acknowledged"):
        diagnostics.append(
            _diagnostic(
                "CAEPIPE-RUN-ENVIRONMENT-RESPONSIBILITY-MISSING",
                "blocking",
                "EXECUTION_BOUNDARY_WARNING",
                "External execution is attempted without an acknowledged user-owned run environment record.",
                "Require the caller to acknowledge local environment responsibility before recording an attempted run.",
                [package_ref],
                provenance,
            )
        )
    if status == "skipped_no_executable" and attempted:
        diagnostics.append(
            _diagnostic(
                "CAEPIPE-RUN-SKIP-ATTEMPTED",
                "blocking",
                "EXECUTION_BOUNDARY_WARNING",
                "A skipped package cannot record an attempted external execution.",
                "Use not_attempted execution metadata for skipped public evidence.",
                [package_ref],
                provenance,
            )
        )
    if status != "skipped_no_executable" and not package.get("parsed_csv", {}).get("rows"):
        diagnostics.append(
            _diagnostic(
                "CAEPIPE-RUN-PARSED-ROWS-MISSING",
                "blocking",
                "PARSER_COVERAGE_WARNING",
                "Non-skipped external-run evidence has no parsed CSV rows.",
                "Provide parser-only rows or mark the package as skipped or blocked.",
                [package_ref],
                provenance,
            )
        )
    diagnostics.extend(deepcopy(_list(package.get("parsed_csv", {}).get("diagnostics"))))
    diagnostics.extend(_privacy_diagnostics(package.get("privacy"), package_ref, provenance))
    if contains_prohibited_authority_term(package.get("command_profile", {}).get("free_metadata", {})):
        diagnostics.append(
            _diagnostic(
                "CAEPIPE-RUN-AUTHORITY-TERM",
                "blocking",
                "IP_BOUNDARY_WARNING",
                "Command profile free metadata contains authority wording.",
                "Remove compatibility, validation, compliance, approval, or professional-reliance wording.",
                [package_ref],
                provenance,
            )
        )
    if contains_prohibited_authority_term(package.get("execution_result", {}).get("free_metadata", {})):
        diagnostics.append(
            _diagnostic(
                "CAEPIPE-RUN-RESULT-AUTHORITY-TERM",
                "blocking",
                "IP_BOUNDARY_WARNING",
                "Execution-result free metadata contains authority wording.",
                "Keep parsed output classified as operational regression and handoff evidence only.",
                [package_ref],
                provenance,
            )
        )
    return _stable(diagnostics)


def canonical_json(value: Any) -> str:
    """Serialize values with stable JSON key ordering."""

    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def canonical_csv(value: str) -> str:
    """Normalize CSV text to ASCII-compatible LF records."""

    rows = [line.rstrip() for line in value.replace("\r\n", "\n").replace("\r", "\n").split("\n")]
    while rows and rows[-1] == "":
        rows.pop()
    return "\n".join(rows) + "\n"


def write_caepipe_external_run_package(directory: str | Path, package: Mapping[str, Any]) -> None:
    """Write run-evidence package members deterministically."""

    root = Path(directory)
    root.mkdir(parents=True, exist_ok=True)
    for key, filename in (
        ("run_metadata", "run_metadata.json"),
        ("parsed_csv", "parsed_csv.json"),
        ("diagnostics", "diagnostics.json"),
        ("checksums", "checksums.json"),
    ):
        if key == "run_metadata":
            value = {item_key: item_value for item_key, item_value in package.items() if item_key != "parsed_csv"}
        else:
            value = package[key]
        (root / filename).write_text(canonical_json(value) + "\n", encoding="utf-8")


def _executable_config(value: Mapping[str, Any]) -> dict[str, Any]:
    config = dict(value)
    return {
        "configuration_surface": str(config.get("configuration_surface", "TBD-17-05-PH-001")),
        "configured_path_state": str(config.get("configured_path_state", "absent")),
        "license_responsibility_acknowledged": bool(config.get("license_responsibility_acknowledged", False)),
        "environment_responsibility_acknowledged": bool(config.get("environment_responsibility_acknowledged", False)),
        "path_record": str(config.get("path_record", "not_configured")),
    }


def _command_profile(value: Mapping[str, Any]) -> dict[str, Any]:
    profile = dict(value)
    return {
        "profile_id": str(profile.get("profile_id", "TBD-17-05-invocation-profile")),
        "invocation_mode": str(profile.get("invocation_mode", "not_invoked")),
        "command_shape": str(profile.get("command_shape", "TBD")),
        "profile_basis": str(profile.get("profile_basis", "TBD-17-05-PH-001")),
        **({"free_metadata": deepcopy(profile["free_metadata"])} if "free_metadata" in profile else {}),
    }


def _run_directory(value: Mapping[str, Any]) -> dict[str, Any]:
    directory = dict(value)
    return {
        "run_directory_ref": str(directory.get("run_directory_ref", "TBD-17-05-PH-002")),
        "working_directory": str(directory.get("working_directory", "TBD")),
        "input_mbf_path": str(directory.get("input_mbf_path", "TBD")),
        "expected_csv_path": str(directory.get("expected_csv_path", "TBD")),
        "observed_csv_path": directory.get("observed_csv_path"),
        "output_discovery_status": str(directory.get("output_discovery_status", "not_attempted")),
    }


def _execution_result(value: Mapping[str, Any]) -> dict[str, Any]:
    result = dict(value)
    return {
        "attempted": bool(result.get("attempted", False)),
        "exit_status": result.get("exit_status"),
        "stdout_capture": str(result.get("stdout_capture", "not_available")),
        "stderr_capture": str(result.get("stderr_capture", "not_available")),
        "skip_reason": result.get("skip_reason"),
        **({"free_metadata": deepcopy(result["free_metadata"])} if "free_metadata" in result else {}),
    }


def _parsed_csv(value: Mapping[str, Any] | None, provenance: Mapping[str, Any]) -> dict[str, Any]:
    if not value:
        return {
            "source_csv_ref": _ref("CsvArtifact", "not_produced"),
            "parser_status": "not_run",
            "row_count": 0,
            "rows": [],
            "diagnostics": [],
            "csv_checksum": _text_checksum("", _ref("CsvArtifact", "not_produced"), "caepipe_csv_text"),
            "provenance": deepcopy(dict(provenance)),
        }
    parsed = dict(value)
    return {
        "source_csv_ref": deepcopy(dict(parsed.get("source_csv_ref", _ref("CsvArtifact", "csv:TBD")))),
        "parser_status": str(parsed.get("parser_status", "parsed_with_diagnostics")),
        "row_count": int(parsed.get("row_count", len(_list(parsed.get("rows"))))),
        "rows": deepcopy(_list(parsed.get("rows"))),
        "diagnostics": deepcopy(_list(parsed.get("diagnostics"))),
        "csv_checksum": deepcopy(
            dict(parsed.get("csv_checksum", _text_checksum("", _ref("CsvArtifact", "csv:TBD"), "caepipe_csv_text")))
        ),
        "provenance": deepcopy(dict(parsed.get("provenance", provenance))),
    }


def _parser_coverage(entries: list[Mapping[str, Any]] | None, provenance: Mapping[str, Any]) -> list[dict[str, Any]]:
    coverage = []
    for entry in entries or DEFAULT_PARSER_COVERAGE:
        coverage.append(
            {
                "section_name": str(entry.get("section_name", "TBD")),
                "basis": str(entry.get("basis", "TBD")),
                "support_status": str(entry.get("support_status", "TBD")),
                "expected_columns": [str(item) for item in _list(entry.get("expected_columns"))],
                "unmapped_row_policy": str(entry.get("unmapped_row_policy", "diagnostic_warning")),
                "diagnostic_severity": str(entry.get("diagnostic_severity", "warning")),
                "fixture_provenance": str(entry.get("fixture_provenance", "TBD")),
                "provenance": deepcopy(dict(entry.get("provenance", provenance))),
            }
        )
    return sorted(coverage, key=lambda item: item["section_name"])


def _stable_refs(entries: list[Mapping[str, Any]]) -> set[str]:
    refs = set()
    for entry in entries:
        canonical = entry.get("canonical_ref")
        if isinstance(canonical, Mapping):
            refs.add(str(canonical.get("ref", "")))
        if "stable_id" in entry:
            refs.add(str(entry["stable_id"]))
    return refs


def _checksums(run_id: str, package: Mapping[str, Any]) -> list[dict[str, Any]]:
    members = {
        "run_metadata": {key: value for key, value in package.items() if key not in {"checksums", "parsed_csv"}},
        "parsed_csv": package.get("parsed_csv", {}),
        "diagnostics": package.get("diagnostics", []),
    }
    return _stable(
        [
            _checksum(value, _ref("CaePipeExternalRunMember", f"{run_id}:{scope}"), f"caepipe_external_{scope}")
            for scope, value in members.items()
        ]
    )


def _checksum(value: Any, payload_ref: Mapping[str, Any], payload_scope: str) -> dict[str, Any]:
    return {
        "algorithm": "sha256",
        "canonicalization": "JCS_compatible_json_payload_hash",
        "payload_ref": deepcopy(dict(payload_ref)),
        "payload_scope": payload_scope,
        "value": "sha256:" + hashlib.sha256(canonical_json(value).encode("utf-8")).hexdigest(),
    }


def _text_checksum(value: str, payload_ref: Mapping[str, Any], payload_scope: str) -> dict[str, Any]:
    return {
        "algorithm": "sha256",
        "canonicalization": "normalized_ascii_lf_text",
        "payload_ref": deepcopy(dict(payload_ref)),
        "payload_scope": payload_scope,
        "value": "sha256:" + hashlib.sha256(value.encode("utf-8")).hexdigest(),
    }


def _privacy(value: Mapping[str, Any] | None) -> dict[str, Any]:
    privacy = deepcopy(DEFAULT_PRIVACY)
    privacy.update(
        {
            key: item
            for key, item in dict(value or {}).items()
            if key not in {"classification", "local_only", "telemetry_allowed"}
        }
    )
    privacy["redaction_refs"] = deepcopy(_list(privacy.get("redaction_refs")))
    return privacy


def _privacy_override_diagnostics(
    supplied: Mapping[str, Any] | None,
    package_ref: Mapping[str, Any],
    provenance: Mapping[str, Any],
) -> list[dict[str, Any]]:
    record = supplied or {}
    attempted = [
        field
        for field in ("classification", "local_only", "telemetry_allowed")
        if field in record and record.get(field) != DEFAULT_PRIVACY[field]
    ]
    return [
        _diagnostic(
            "CAEPIPE-RUN-PRIVACY-DEFAULT-OVERRIDE-BLOCKED",
            "blocking",
            "IP_BOUNDARY_WARNING",
            f"Caller attempted to override private-by-default CAEPIPE evidence field {field}.",
            "Keep user-provided CAEPIPE evidence private, local-only, and telemetry-disabled; no public-rights override path is defined.",
            [package_ref],
            provenance,
        )
        for field in attempted
    ]


def _privacy_diagnostics(
    privacy: Mapping[str, Any] | None,
    package_ref: Mapping[str, Any],
    provenance: Mapping[str, Any],
) -> list[dict[str, Any]]:
    diagnostics = []
    record = privacy or {}
    for field in (
        "private_payload_embedded",
        "protected_payload_embedded",
        "commercial_tool_payload_embedded",
        "telemetry_allowed",
    ):
        if record.get(field) is True:
            diagnostics.append(
                _diagnostic(
                    "CAEPIPE-RUN-PRIVACY-BOUNDARY-VIOLATION",
                    "blocking",
                    "IP_BOUNDARY_WARNING",
                    f"Privacy field {field} is not allowed for public CAEPIPE run evidence.",
                    "Reference private/protected/commercial content by governed metadata only.",
                    [package_ref],
                    provenance,
                )
            )
    return diagnostics


def _diagnostic(
    code: str,
    severity: str,
    diagnostic_class: str,
    message: str,
    remediation: str,
    affected_refs: list[Mapping[str, Any]],
    provenance: Mapping[str, Any],
) -> dict[str, Any]:
    return {
        "code": code,
        "class": diagnostic_class,
        "severity": severity,
        "source": _ref("Deliverable", "DEL-17-05"),
        "affected_object": deepcopy(dict(affected_refs[0] if affected_refs else _ref("Unknown", "unknown"))),
        "message": message,
        "remediation": remediation,
        "provenance": deepcopy(dict(provenance)),
    }


def _ref(object_type: str, value: str) -> dict[str, str]:
    return {"object_type": object_type, "ref": value}


def _list(value: Any) -> list[Any]:
    if value is None:
        return []
    if isinstance(value, list):
        return value
    return [value]


def _csv_number(value: Any) -> int | float | str:
    text = str(value)
    try:
        number = float(text)
    except ValueError:
        return text
    if number.is_integer():
        return int(number)
    return number


def _stable(values: list[Any]) -> list[Any]:
    return sorted(values, key=canonical_json)


def _sort(value: Any) -> Any:
    if isinstance(value, dict):
        return {key: _sort(value[key]) for key in sorted(value)}
    if isinstance(value, list):
        return [_sort(item) for item in value]
    return value
