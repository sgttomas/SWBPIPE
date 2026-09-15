"""Versioned analysis-record compatibility and strict 0.2 construction."""
from __future__ import annotations

from copy import deepcopy
from pathlib import Path
import json
from typing import Any, Callable, Mapping

from core.serialization.canonical_json.adapter import canonical_sha256_checked_v1

SCHEMA_VERSION = "0.2.0"
PROFILE = "openpipestress_jcs_ijson_v1"
SEMANTIC_CONTRACT_SHA256 = "4d6886d19e304db897e5e9f8f0054cbee91ba7795868f9698e2bbe070bde94da"
SEMANTIC_CONTRACT_ID = "openpipestress_result_semantics_v0_2"
_CONTRACT_PATH = Path(__file__).resolve().parents[2] / "fixtures" / "results" / "semantic_contract_v0_2.json"


def _semantic_rows() -> list[dict[str, Any]]:
    payload = json.loads(_CONTRACT_PATH.read_text(encoding="utf-8"))
    return payload["rows"]


def _semantic(row: Mapping[str, Any]) -> tuple[dict[str, Any] | None, list[str]]:
    kind = str(row.get("kind", ""))
    unit = str(row.get("unit", ""))
    candidates = [entry for entry in _semantic_rows() if entry["kind"] == kind]
    if not candidates:
        return None, ["SOURCE_SIGNATURE_UNKNOWN"]
    units = [entry for entry in candidates if entry["unit"] == unit]
    if not units:
        raise ValueError(f"SOURCE_UNIT_CONTRADICTION: {kind} / {unit}")
    metadata = row.get("metadata") if isinstance(row.get("metadata"), Mapping) else {}
    component = metadata.get("component") if isinstance(metadata, Mapping) else None
    exact = [entry for entry in units if entry.get("component") == component]
    if exact:
        legacy_dimension = exact[0].get("legacy_declared_dimension")
        if row.get("dimension") and legacy_dimension and row.get("dimension") != legacy_dimension:
            raise ValueError(f"ANALYSIS-RUN-RESULT-DIMENSION-MISMATCH: {row.get('id')}")
        return exact[0], []
    generic = [entry for entry in units if entry.get("component") is None]
    if generic:
        legacy_dimension = generic[0].get("legacy_declared_dimension")
        if row.get("dimension") and legacy_dimension and row.get("dimension") != legacy_dimension:
            raise ValueError(f"ANALYSIS-RUN-RESULT-DIMENSION-MISMATCH: {row.get('id')}")
        return generic[0], ["OPTIONAL_SOURCE_METADATA_MISSING"] if component is None else []
    if component is not None:
        raise ValueError(f"SOURCE_COMPONENT_CONTRADICTION: {kind} / {component}")
    return None, ["SOURCE_COMPONENT_MISSING_SEMANTICS_UNAVAILABLE"]


def _checksum(scope: str, ref: dict[str, str], value: Any, hash_fn: Callable[[Any], str]) -> dict[str, Any]:
    return {"algorithm": "sha256", "canonicalization": PROFILE, "payload_ref": ref, "payload_scope": scope, "value": hash_fn(value)}


def analysis_record_projection(envelope: Mapping[str, Any]) -> dict[str, Any]:
    projected = deepcopy(dict(envelope))
    hashes = projected["analysis_run"]["hashes"]
    matches = [row for row in hashes if row.get("payload_scope") == "analysis_run_record"]
    if len(matches) > 1:
        raise ValueError("ANALYSIS-RUN-RECORD-CHECKSUM-DUPLICATE")
    projected["analysis_run"]["hashes"] = [row for row in hashes if row.get("payload_scope") != "analysis_run_record"]
    return projected


def build_analysis_run_v0_2(
    mechanics_result: Mapping[str, Any], *, input_manifest_ref: Mapping[str, str], input_manifest_hash: str,
    input_manifest: Mapping[str, Any] | None = None,
    created_at: str | None = None, rule_check_status: str | None = None,
    hash_fn: Callable[[Any], str] = canonical_sha256_checked_v1,
    solver_name: str = "unavailable:not-supplied", solver_version: str = "unavailable:not-supplied",
    solver_build_ref: Mapping[str, str] | None = None, settings_ref: Mapping[str, str] | None = None,
    unit_system_ref: Mapping[str, str] | None = None,
) -> dict[str, Any]:
    received = deepcopy(dict(mechanics_result))
    run_id = str(received.get("run_id", "run:unknown"))
    rows = [deepcopy(dict(row)) for row in received.get("results", []) if isinstance(row, Mapping)]
    refs = []
    for row in rows:
        semantic, findings = _semantic(row)
        row_id = str(row.get("id", "result:unknown"))
        refs.append({
            "result_ref": {"object_type": "Result", "ref": row_id},
            "source_row_index": len(refs),
            "category": semantic.get("category") if semantic else "unknown",
            "source_dimension": semantic.get("source_physical_semantic_dimension") if semantic else None,
            "result_family": semantic.get("family") if semantic else None,
            "semantic_contract": {"id": SEMANTIC_CONTRACT_ID, "sha256": SEMANTIC_CONTRACT_SHA256, "signature_id": semantic.get("signature_id") if semantic else None},
            "interpretation": {"status": semantic.get("canonical_disposition") if semantic else "unavailable", "findings": findings},
            "source_annotation": {"kind": row.get("kind"), "unit": row.get("unit"), "metadata": deepcopy(row.get("metadata"))},
            "hash_refs": [_checksum("result_row", {"object_type": "Result", "ref": row_id}, row, hash_fn)],
            "privacy_classification": "source_evidence",
        })
    statuses = {"HUMAN_REVIEW_REQUIRED"}
    status = received.get("status") if isinstance(received.get("status"), Mapping) else {}
    statuses.update(str(status[field]) for field in ("mechanics", "rule_check") if status.get(field))
    if rule_check_status:
        statuses.discard(str(status.get("rule_check", "")))
        statuses.add(rule_check_status)
    run_ref = {"object_type": "AnalysisRun", "ref": run_id}
    received_ref = {"object_type": "ResultEnvelope", "ref": f"result-envelope:{run_id}"}
    manifest = deepcopy(dict(input_manifest)) if input_manifest is not None else None
    if manifest is not None and manifest.get("model_basis", {}).get("model_ref") != received.get("model_ref"):
        raise ValueError("ANALYSIS-RUN-INPUT-MANIFEST-MODEL-MISMATCH")
    solver_basis = manifest.get("solver_basis", {}) if manifest is not None else {}
    solver_name = str(solver_basis.get("solver_name", solver_name))
    solver_version = str(solver_basis.get("solver_version", solver_version))
    if solver_build_ref is None and solver_basis.get("solver_build_ref"):
        solver_build_ref = {"object_type": "ExternalReference", "ref": str(solver_basis["solver_build_ref"])}
    manifest_identity = f"{input_manifest_ref['ref']}:{input_manifest_hash}"
    if settings_ref is None:
        settings_ref = {"object_type": "SolverSettings", "ref": f"solver-settings:{manifest_identity}"}
    if unit_system_ref is None:
        unit_system_ref = {"object_type": "UnitSystem", "ref": f"unit-system:{received.get('model_ref', 'unknown')}:{input_manifest_hash}"}
    load_basis_refs: list[dict[str, str]] = []
    seen_basis: set[str] = set()
    for row in rows:
        basis = row.get("basis_ref") if isinstance(row.get("basis_ref"), Mapping) else None
        if not basis:
            continue
        object_type = "Combination" if basis.get("ref_type") == "combination" else "LoadCase"
        key = f"{object_type}:{basis.get('ref_id')}"
        if key not in seen_basis:
            seen_basis.add(key)
            load_basis_refs.append({"object_type": object_type, "ref": str(basis.get("ref_id"))})
    provenance = {"source_name": "OpenPipeStress analysis record 0.2", "source_location": "analysis_run.compatibility.v0.2", "source_license": "project-governed", "review_status": "pending", "professional_claim": False}
    determinism_notes = (["created_at_unavailable"] if created_at is None else []) + (["model_state_ref_and_solver_settings_unit_basis_bound_by_input_manifest"] if manifest is not None else ["input_manifest_payload_unavailable_solver_settings_unit_basis_not_independently_verified"])
    for row in refs:
        row["provenance"] = deepcopy(provenance)
    envelope: dict[str, Any] = {
        "schema_version": SCHEMA_VERSION, "deliverable_id": "DEL-14-02", "package_id": "PKG-14", "scope_item": "SOW-072", "objectives": ["OBJ-016"],
        "run_contract_status": {"record_contract": "strict_analysis_run_v0_2", "result_binding": "received_mechanics_result", "external_validation_boundary": "reference_only_not_determined_by_software"},
        "analysis_run": {
            "run_id": run_id, "run_name": f"{run_id} analysis record", "run_kind": "mechanics_solve", "created_at": created_at,
            "model_state_ref": {"object_type": "ModelState", "ref": f"state:{received.get('model_ref', 'unknown')}:preview"},
            "solver_version": {"solver_name": solver_name, "solver_version": solver_version, "build_ref": deepcopy(dict(solver_build_ref or {"object_type": "ExternalReference", "ref": "unavailable:solver-build-not-supplied"}))},
            "settings_ref": deepcopy(dict(settings_ref or {"object_type": "SolverSettings", "ref": "unavailable:solver-settings-not-supplied"})), "unit_system_ref": deepcopy(dict(unit_system_ref or {"object_type": "UnitSystem", "ref": "unavailable:unit-system-not-supplied"})),
            "load_basis_refs": load_basis_refs, "diagnostics": [{"source_annotation": deepcopy(item)} for item in received.get("diagnostics", [])], "result_refs": refs, "rule_pack_refs": [], "library_refs": [],
            "hashes": [_checksum("received_result", received_ref, received, hash_fn)], "analysis_status": sorted(statuses),
            "reproducibility": {"input_manifest_refs": [deepcopy(dict(input_manifest_ref))], "input_manifest_hashes": [{"algorithm": "sha256", "canonicalization": "rfc8785_jcs", "payload_ref": deepcopy(dict(input_manifest_ref)), "payload_scope": "input_manifest", "value": input_manifest_hash}], "semantic_contract": {"id": SEMANTIC_CONTRACT_ID, "sha256": SEMANTIC_CONTRACT_SHA256}, "determinism_notes": determinism_notes, "unresolved_tbd": []},
            "immutability_policy": {"run_record_is_read_only": True, "mutation_policy": "changes_create_new_immutable_record_revision", "new_mechanics_run_required_for_record_revision": False, "record_revision_identity": "analysis_run_record_sha256", "hash_invalidates_external_acceptance": True},
            "professional_boundary": {"human_review_required": True, "software_makes_compliance_claim": False, "software_makes_certification_claim": False, "software_makes_sealing_claim": False, "software_makes_approval_claim": False, "software_makes_authentication_claim": False},
            "provenance": provenance,
        },
    }
    envelope["analysis_run"]["hashes"].insert(0, _checksum("analysis_run_record", run_ref, analysis_record_projection(envelope), hash_fn))
    return envelope


def verify_analysis_run_record(envelope: Mapping[str, Any], hash_fn: Callable[[Any], str] = canonical_sha256_checked_v1) -> str:
    version = envelope.get("schema_version")
    if version != SCHEMA_VERSION:
        if version == "0.1.0":
            return "unverifiable"
        raise ValueError(f"ANALYSIS-RUN-SCHEMA-VERSION-UNSUPPORTED: {version}")
    matches = [row for row in envelope["analysis_run"]["hashes"] if row.get("payload_scope") == "analysis_run_record"]
    if len(matches) != 1:
        return "unverifiable"
    expected_ref = {"object_type": "AnalysisRun", "ref": envelope["analysis_run"].get("run_id")}
    claim = matches[0]
    if claim.get("algorithm") != "sha256" or claim.get("canonicalization") != PROFILE or claim.get("payload_ref") != expected_ref:
        return "mismatch"
    return "match" if claim.get("value") == hash_fn(analysis_record_projection(envelope)) else "mismatch"
