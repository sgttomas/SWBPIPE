"""Immutable analysis-run records for DEL-14-02.

This module builds schema-shaped analysis-run records from already computed
preview mechanics results. It records provenance, diagnostics, result hashes,
and professional-boundary flags; it does not run solvers, persist project
files, evaluate code criteria, or make engineering acceptance claims.
"""

from __future__ import annotations

from copy import deepcopy
from hashlib import sha256
import json
from typing import Any, Mapping


SCHEMA_VERSION = "0.1.0"
DELIVERABLE_ID = "DEL-14-02"
PACKAGE_ID = "PKG-14"
SCOPE_ITEM = "SOW-072"
OBJECTIVES = ["OBJ-016"]

ENGINE_PROVENANCE = {
    "source_name": "OpenPipeStress DEL-14-02 analysis-run record builder",
    "source_location": "core/analysis_runs/records.py",
    "source_license": "project-governed",
    "contributor": "OpenPipeStress Type 2 worker",
    "contributor_certification": "implementation-only-no-professional-claim",
    "redistribution_status": "invented_non_engineering_example",
    "review_status": "pending",
    "privacy_classification": "invented_public_example",
}

PROFESSIONAL_BOUNDARY = {
    "human_review_required": True,
    "software_makes_compliance_claim": False,
    "software_makes_certification_claim": False,
    "software_makes_sealing_claim": False,
    "software_makes_approval_claim": False,
    "software_makes_authentication_claim": False,
}

IMMUTABILITY_POLICY = {
    "run_record_is_read_only": True,
    "mutation_policy": "changes_create_new_analysis_run",
    "new_run_required_for_change": True,
    "hash_invalidates_external_acceptance": True,
}

PHYSICAL_PROJECT_CONTAINER = {
    "profile": "sqlite_local_project_store",
    "decision_ref": "SCA-003",
    "storage_role": "local_store_index_projection",
    "canonical_truth": "sorted_compact_json_payload",
    "sql_public_contract": False,
    "direct_sql_access_allowed": False,
    "hosted_db_allowed": False,
    "network_required": False,
    "sidecars_rebuildable": True,
}

RUN_CONTRACT_STATUS = {
    "record_contract": "schema_first_analysis_run_records",
    "model_state_binding": "schemas/model_state.schema.json",
    "result_binding": "schemas/results.schema.yaml",
    "physical_project_container": deepcopy(PHYSICAL_PROJECT_CONTAINER),
    "external_validation_boundary": "reference_only_not_determined_by_software",
}


def build_preview_analysis_run_envelope(
    mechanics_result: Mapping[str, Any],
    *,
    model_state_ref: Mapping[str, str] | None = None,
    created_at: str = "2026-05-10T00:00:00Z",
    settings_ref: Mapping[str, str] | None = None,
    unit_system_ref: Mapping[str, str] | None = None,
    load_basis_refs: list[Mapping[str, str]] | None = None,
    environment_refs: list[Mapping[str, str]] | None = None,
    solver_name: str = "open_pipe_stress_product_physics",
    solver_version: str = "0.1.0",
    build_ref: Mapping[str, str] | None = None,
) -> dict[str, Any]:
    """Build a deterministic DEL-14-02 wrapper for a preview mechanics result."""

    result = deepcopy(dict(mechanics_result))
    run_id = str(result.get("run_id", "run:preview-linear-static-unknown"))
    result_envelope_ref = _ref("ResultEnvelope", f"result-envelope:{run_id}")

    result_refs = [_result_ref(item) for item in sorted(_results(result), key=_result_id)]
    diagnostics = [_diagnostic(item) for item in _diagnostics(result)]
    result_envelope_hash = _checksum(
        result_envelope_ref,
        "result_envelope",
        result,
    )

    analysis_run = {
        "run_id": run_id,
        "run_name": f"{run_id} preview mechanics run",
        "run_kind": "mechanics_solve",
        "created_at": created_at,
        "model_state_ref": dict(model_state_ref or _ref("ModelState", f"state:{result.get('model_ref', 'unknown')}")),
        "solver_version": {
            "solver_name": solver_name,
            "solver_version": solver_version,
            "build_ref": dict(build_ref or _ref("ExternalReference", "build:TBD")),
            "provenance": deepcopy(ENGINE_PROVENANCE),
        },
        "settings_ref": dict(settings_ref or _ref("SolverSettings", f"settings:{run_id}")),
        "unit_system_ref": dict(unit_system_ref or _ref("UnitSystem", "SI-preview-units")),
        "load_basis_refs": [dict(item) for item in (load_basis_refs or [_ref("LoadCase", "loadcase:preview")])],
        "diagnostics": diagnostics,
        "result_refs": result_refs,
        "rule_pack_refs": [],
        "library_refs": [],
        "hashes": [
            _checksum(_ref("AnalysisRun", run_id), "analysis_run_record", _run_hash_payload(run_id, result)),
            result_envelope_hash,
        ],
        "analysis_status": _analysis_status(result.get("status", {})),
        "reproducibility": {
            "input_manifest_refs": [result_envelope_ref],
            "environment_refs": [dict(item) for item in (environment_refs or [_ref("ExternalReference", "environment:local-preview")])],
            "determinism_notes": [
                "analysis run record was generated from an already computed invented preview mechanics result",
                "canonical hashes use stable JSON key ordering",
            ],
            "unresolved_tbd": [
                "physical project container",
                "release-grade solver build provenance",
            ],
        },
        "immutability_policy": deepcopy(IMMUTABILITY_POLICY),
        "professional_boundary": _professional_boundary(result.get("professional_boundary", {})),
        "provenance": deepcopy(ENGINE_PROVENANCE),
    }

    return {
        "schema_version": SCHEMA_VERSION,
        "deliverable_id": DELIVERABLE_ID,
        "package_id": PACKAGE_ID,
        "scope_item": SCOPE_ITEM,
        "objectives": list(OBJECTIVES),
        "run_contract_status": deepcopy(RUN_CONTRACT_STATUS),
        "analysis_run": analysis_run,
    }


def validate_analysis_run_envelope(envelope: Mapping[str, Any]) -> list[dict[str, Any]]:
    """Return blocking diagnostics for core DEL-14-02 invariant violations."""

    run = envelope.get("analysis_run", {}) if isinstance(envelope, Mapping) else {}
    diagnostics: list[dict[str, Any]] = []
    if not isinstance(run, Mapping):
        return [_validation_diagnostic("ANALYSIS_RUN_RECORD_MISSING", "analysis_run")]

    for field in (
        "run_id",
        "model_state_ref",
        "solver_version",
        "settings_ref",
        "unit_system_ref",
        "load_basis_refs",
        "result_refs",
        "hashes",
        "analysis_status",
        "immutability_policy",
        "professional_boundary",
    ):
        if not run.get(field):
            diagnostics.append(_validation_diagnostic("ANALYSIS_RUN_FIELD_MISSING", field))

    if "HUMAN_REVIEW_REQUIRED" not in set(run.get("analysis_status", [])):
        diagnostics.append(_validation_diagnostic("ANALYSIS_RUN_HUMAN_REVIEW_REQUIRED_MISSING", "analysis_status"))

    boundary = run.get("professional_boundary", {})
    if boundary != PROFESSIONAL_BOUNDARY:
        diagnostics.append(_validation_diagnostic("ANALYSIS_RUN_PROFESSIONAL_BOUNDARY_VIOLATION", "professional_boundary"))

    if run.get("immutability_policy", {}) != IMMUTABILITY_POLICY:
        diagnostics.append(_validation_diagnostic("ANALYSIS_RUN_IMMUTABILITY_POLICY_VIOLATION", "immutability_policy"))

    for result_ref in run.get("result_refs", []):
        if not result_ref.get("hash_refs"):
            diagnostics.append(_validation_diagnostic("ANALYSIS_RUN_RESULT_HASH_MISSING", _nested_ref(result_ref.get("result_ref"))))

    return sorted(diagnostics, key=canonical_json)


def canonical_json(value: Any) -> str:
    """Return the historical hash bytes: sorted keys, compact JSON, ASCII escapes.

    The public name is retained for compatibility. This serializer is not an
    RFC 8785 / JCS implementation and must not be labeled as one.
    """

    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def _results(mechanics_result: Mapping[str, Any]) -> list[Mapping[str, Any]]:
    values = mechanics_result.get("results", [])
    return [item for item in values if isinstance(item, Mapping)]


def _diagnostics(mechanics_result: Mapping[str, Any]) -> list[Mapping[str, Any]]:
    values = mechanics_result.get("diagnostics", [])
    return [item for item in values if isinstance(item, Mapping)]


def _result_ref(result: Mapping[str, Any]) -> dict[str, Any]:
    result_id = _result_id(result)
    return {
        "result_ref": _ref("Result", result_id),
        "result_family": _result_family(result),
        "hash_refs": [_checksum(_ref("Result", result_id), "result_value", result)],
        "privacy_classification": "invented_public_example",
        "provenance": deepcopy(ENGINE_PROVENANCE),
    }


def _diagnostic(diagnostic: Mapping[str, Any]) -> dict[str, Any]:
    diagnostic_id = str(diagnostic.get("id", "diagnostic:unknown"))
    severity = str(diagnostic.get("severity", "warning"))
    return {
        "code": str(diagnostic.get("code", "UNKNOWN_DIAGNOSTIC")),
        "class": _diagnostic_class(diagnostic),
        "severity": severity if severity in {"info", "warning", "blocking"} else "warning",
        "source": _ref("ExternalReference", str(diagnostic.get("source", "unknown"))),
        "affected_object": _ref("Diagnostic", diagnostic_id),
        "message": str(diagnostic.get("message", "No diagnostic message supplied.")),
        "remediation": "Review the computed preview diagnostic before relying on this analysis run record.",
        "provenance": deepcopy(ENGINE_PROVENANCE),
    }


def _diagnostic_class(diagnostic: Mapping[str, Any]) -> str:
    code = str(diagnostic.get("code", ""))
    if "RULE" in code:
        return "RULE_CHECK_BLOCKING" if diagnostic.get("severity") == "blocking" else "ASSUMPTION_WARNING"
    if "UNIT" in code:
        return "UNIT_WARNING"
    if "PROVENANCE" in code:
        return "PROVENANCE_WARNING"
    if diagnostic.get("severity") == "blocking":
        return "SOLVE_BLOCKING"
    return "RUN_REPRODUCIBILITY_WARNING"


def _analysis_status(status: Mapping[str, Any]) -> list[str]:
    values = {"HUMAN_REVIEW_REQUIRED"}
    mechanics = str(status.get("mechanics", ""))
    rule_check = str(status.get("rule_check", ""))
    if mechanics:
        values.add(mechanics)
    if rule_check:
        values.add(rule_check)
    return sorted(values)


def _professional_boundary(boundary: Mapping[str, Any]) -> dict[str, bool]:
    normalized = deepcopy(PROFESSIONAL_BOUNDARY)
    for key in normalized:
        if key in boundary:
            normalized[key] = bool(boundary[key])
    normalized["software_makes_authentication_claim"] = False
    return normalized


def _result_family(result: Mapping[str, Any]) -> str:
    kind = str(result.get("kind", ""))
    result_id = _result_id(result)
    if "disp" in result_id or "displacement" in kind:
        return "displacement"
    if "reaction" in result_id or "reaction" in kind:
        return "reaction"
    if "force" in result_id or "force" in kind:
        return "force"
    if "moment" in result_id or "moment" in kind:
        return "moment"
    if "stress" in result_id or "stress" in kind:
        return "stress"
    if "ratio" in result_id or "ratio" in kind:
        return "ratio"
    return "TBD"


def _result_id(result: Mapping[str, Any]) -> str:
    return str(result.get("id", "result:unknown"))


def _checksum(payload_ref: Mapping[str, str], payload_scope: str, payload: Any) -> dict[str, Any]:
    return {
        "algorithm": "sha256",
        "canonicalization": "SORTED_COMPACT_JSON",
        "payload_ref": dict(payload_ref),
        "payload_scope": payload_scope,
        "value": sha256(canonical_json(payload).encode("utf-8")).hexdigest(),
    }


def _run_hash_payload(run_id: str, mechanics_result: Mapping[str, Any]) -> dict[str, Any]:
    return {
        "run_id": run_id,
        "model_ref": mechanics_result.get("model_ref"),
        "status": mechanics_result.get("status"),
        "result_ids": [_result_id(item) for item in sorted(_results(mechanics_result), key=_result_id)],
        "diagnostic_ids": [str(item.get("id", "diagnostic:unknown")) for item in sorted(_diagnostics(mechanics_result), key=lambda item: str(item.get("id", "")))],
    }


def _validation_diagnostic(code: str, affected_ref: str) -> dict[str, Any]:
    return {
        "code": code,
        "class": "RUN_REPRODUCIBILITY_WARNING",
        "severity": "blocking",
        "source": _ref("ExternalReference", "core/analysis_runs/records.py"),
        "affected_object": _ref("AnalysisRun", affected_ref),
        "message": f"Analysis run invariant failed: {affected_ref}.",
        "remediation": "Regenerate the analysis run record from explicit model, result, and hash inputs.",
        "provenance": deepcopy(ENGINE_PROVENANCE),
    }


def _ref(object_type: str, ref: str) -> dict[str, str]:
    return {"object_type": object_type, "ref": ref}


def _nested_ref(reference: Any) -> str:
    if isinstance(reference, Mapping):
        return str(reference.get("ref", "unknown"))
    return "unknown"
