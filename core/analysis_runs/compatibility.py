"""Versioned analysis-record compatibility and strict 0.2 construction."""
from __future__ import annotations

from copy import deepcopy
from pathlib import Path
import json
from typing import Any, Callable, Mapping

from core.serialization.canonical_json.adapter import canonical_json_checked_v1, canonical_sha256_checked_v1
from .source_blocks import (CONTRACT_ID as SOURCE_BLOCKS_CONTRACT_ID,
    CONTRACT_SHA256 as SOURCE_BLOCKS_CONTRACT_SHA256, CONTRACT_PATH as _SOURCE_BLOCKS_CONTRACT_PATH,
    validate_source_blocks, validate_receipt_shape, domain_hash, ordinary_case_legacy_semantics)

SCHEMA_VERSION = "0.2.0"
PROFILE = "openpipestress_jcs_ijson_v1"
SEMANTIC_CONTRACT_SHA256 = "4d6886d19e304db897e5e9f8f0054cbee91ba7795868f9698e2bbe070bde94da"
SEMANTIC_CONTRACT_ID = "openpipestress_result_semantics_v0_2"
_CONTRACT_PATH = Path(__file__).resolve().parents[2] / "fixtures" / "results" / "semantic_contract_v0_2.json"


def _semantic_rows(path: Path = _CONTRACT_PATH) -> list[dict[str, Any]]:
    payload = json.loads(path.read_text(encoding="utf-8"))
    return payload["rows"]


def _semantic(row: Mapping[str, Any], path: Path = _CONTRACT_PATH) -> tuple[dict[str, Any] | None, list[str]]:
    kind = str(row.get("kind", ""))
    unit = str(row.get("unit", ""))
    candidates = [entry for entry in _semantic_rows(path) if entry["kind"] == kind]
    if not candidates:
        return None, ["SOURCE_SIGNATURE_UNKNOWN"]
    units = [entry for entry in candidates if entry["unit"] == unit]
    if not units:
        raise ValueError(f"SOURCE_UNIT_CONTRADICTION: {kind} / {unit}")
    metadata = row.get("metadata") if isinstance(row.get("metadata"), Mapping) else {}
    component = metadata.get("component") if isinstance(metadata, Mapping) else None
    variants = [entry for entry in units if entry.get("component") == component]
    exact = [entry for entry in variants if "source_basis" not in entry or entry["source_basis"] == metadata.get("basis")]
    if variants and not exact:
        raise ValueError(f"SOURCE_BASIS_CONTRADICTION: {kind}")
    if exact:
        legacy_dimension = exact[0].get("legacy_declared_dimension")
        if row.get("dimension") and legacy_dimension and row.get("dimension") != legacy_dimension:
            raise ValueError(f"ANALYSIS-RUN-RESULT-DIMENSION-MISMATCH: {row.get('id')}")
        return exact[0], []
    generic_variants = [entry for entry in units if entry.get("component") is None]
    generic = [entry for entry in generic_variants if "source_basis" not in entry or entry["source_basis"] == metadata.get("basis")]
    if generic_variants and not generic:
        raise ValueError(f"SOURCE_BASIS_CONTRADICTION: {kind}")
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


def _build_analysis_run(
    mechanics_result: Mapping[str, Any], *, input_manifest_ref: Mapping[str, str], input_manifest_hash: str,
    input_manifest: Mapping[str, Any] | None = None,
    created_at: str | None = None, rule_check_status: str | None = None,
    hash_fn: Callable[[Any], str] = canonical_sha256_checked_v1,
    solver_name: str = "unavailable:not-supplied", solver_version: str = "unavailable:not-supplied",
    solver_build_ref: Mapping[str, str] | None = None, settings_ref: Mapping[str, str] | None = None,
    unit_system_ref: Mapping[str, str] | None = None,
    record_version: str = "0.2.0",
) -> dict[str, Any]:
    received = deepcopy(dict(mechanics_result))
    if record_version == "0.2.0":
        if any(key in received for key in ("producer", "numerical_quality", "formulation_basis", "contract_evidence", "source_block_recovery", "retained_precision")) or _has_retained_rows(received):
            raise ValueError("ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN")
        source_version = received.get("schema_version")
        if type(source_version) is not str or source_version not in ("0.1.0", "0.2.0"):
            raise ValueError("ANALYSIS_LEGACY_SOURCE_VERSION_UNSUPPORTED")
        contract_id, contract_hash, contract_path = SEMANTIC_CONTRACT_ID, SEMANTIC_CONTRACT_SHA256, _CONTRACT_PATH
    else:
        contract_id, contract_hash, contract_path = _source_contract(received)
        if contract_id not in CURRENT_RECORD_CONTRACT_IDS:
            raise ValueError("ANALYSIS_SOURCE_CONTRACT_VERSION_MISMATCH")
    run_id = str(received.get("run_id", "run:unknown"))
    rows = [deepcopy(dict(row)) for row in received.get("results", []) if isinstance(row, Mapping)]
    refs = []
    for row in rows:
        semantic, findings = _semantic(row, contract_path)
        row_id = str(row.get("id", "result:unknown"))
        refs.append({
            "result_ref": {"object_type": "Result", "ref": row_id},
            "source_row_index": len(refs),
            "category": semantic.get("category") if semantic else "unknown",
            "source_dimension": semantic.get("source_physical_semantic_dimension") if semantic else None,
            "result_family": semantic.get("family") if semantic else None,
            "semantic_contract": {"id": contract_id, "sha256": contract_hash, "signature_id": semantic.get("signature_id") if semantic else None},
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
    provenance = {"source_name": f"OpenPipeStress analysis record {record_version[:-2]}", "source_location": f"analysis_run.compatibility.v{record_version[:-2]}", "source_license": "project-governed", "review_status": "pending", "professional_claim": False}
    determinism_notes = (["created_at_unavailable"] if created_at is None else []) + (["model_state_ref_and_solver_settings_unit_basis_bound_by_input_manifest"] if manifest is not None else ["input_manifest_payload_unavailable_solver_settings_unit_basis_not_independently_verified"])
    for row in refs:
        row["provenance"] = deepcopy(provenance)
    envelope: dict[str, Any] = {
        "schema_version": record_version, "deliverable_id": "DEL-14-02", "package_id": "PKG-14", "scope_item": "SOW-072", "objectives": ["OBJ-016"],
        "run_contract_status": {"record_contract": "strict_analysis_run_v0_3" if record_version == "0.3.0" else "strict_analysis_run_v0_2", "result_binding": "received_mechanics_result", "external_validation_boundary": "reference_only_not_determined_by_software"},
        "analysis_run": {
            "run_id": run_id, "run_name": f"{run_id} analysis record", "run_kind": "mechanics_solve", "created_at": created_at,
            "model_state_ref": {"object_type": "ModelState", "ref": f"state:{received.get('model_ref', 'unknown')}:preview"},
            "solver_version": {"solver_name": solver_name, "solver_version": solver_version, "build_ref": deepcopy(dict(solver_build_ref or {"object_type": "ExternalReference", "ref": "unavailable:solver-build-not-supplied"}))},
            "settings_ref": deepcopy(dict(settings_ref or {"object_type": "SolverSettings", "ref": "unavailable:solver-settings-not-supplied"})), "unit_system_ref": deepcopy(dict(unit_system_ref or {"object_type": "UnitSystem", "ref": "unavailable:unit-system-not-supplied"})),
            "load_basis_refs": load_basis_refs, "diagnostics": [{"source_annotation": deepcopy(item)} for item in received.get("diagnostics", [])], "result_refs": refs, "rule_pack_refs": [], "library_refs": [],
            "hashes": [_checksum("received_result", received_ref, received, hash_fn)], "analysis_status": sorted(statuses),
            "reproducibility": {"input_manifest_refs": [deepcopy(dict(input_manifest_ref))], "input_manifest_hashes": [{"algorithm": "sha256", "canonicalization": "rfc8785_jcs", "payload_ref": deepcopy(dict(input_manifest_ref)), "payload_scope": "input_manifest", "value": input_manifest_hash}], "semantic_contract": {"id": contract_id, "sha256": contract_hash}, "determinism_notes": determinism_notes, "unresolved_tbd": []},
            "immutability_policy": {"run_record_is_read_only": True, "mutation_policy": "changes_create_new_immutable_record_revision", "new_mechanics_run_required_for_record_revision": False, "record_revision_identity": "analysis_run_record_sha256", "hash_invalidates_external_acceptance": True},
            "professional_boundary": {"human_review_required": True, "software_makes_compliance_claim": False, "software_makes_certification_claim": False, "software_makes_sealing_claim": False, "software_makes_approval_claim": False, "software_makes_authentication_claim": False},
            "provenance": provenance,
        },
    }
    if contract_id in {SOURCE_BLOCKS_CONTRACT_ID, PHYSICS_SOURCE_CONTRACT_ID, LOAD_REFERENCE_SOURCE_CONTRACT_ID}:
        envelope["analysis_run"]["source_block_recovery"] = deepcopy(received["source_block_recovery"])
    if contract_id in {PHYSICS_SOURCE_CONTRACT_ID, LOAD_REFERENCE_SOURCE_CONTRACT_ID}:
        envelope["analysis_run"]["contract_evidence"] = deepcopy(received["contract_evidence"])
    # C1:162; D2 4.9.6: the successor's AnalysisRun carries its complete receipt (B3b: either successor).
    if contract_id in RETAINED_CONTRACTS:
        envelope["analysis_run"]["retained_precision"] = deepcopy(received["retained_precision"])
    envelope["analysis_run"]["hashes"].insert(0, _checksum("analysis_run_record", run_ref, analysis_record_projection(envelope), hash_fn))
    return envelope


def verify_analysis_run_record(envelope: Mapping[str, Any], hash_fn: Callable[[Any], str] = canonical_sha256_checked_v1) -> str:
    version = envelope.get("schema_version")
    if version not in {SCHEMA_VERSION, "0.3.0"}:
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


PRECISION_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/precision-1"
PRECISION_CONTRACT_SHA256 = "d75aacee175e178dbdeb256d89a65f4b375265f7da077725ee635af33df51d7e"
_PRECISION_CONTRACT_PATH = _CONTRACT_PATH.with_name("semantic_contract_v0_3_precision_1.json")
PHYSICS_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/physics-1"
PHYSICS_CONTRACT_SHA256 = "9a2cf6268b57bd5265a1a115497c07450819dd4d03cd5ab618097bd9d19da8cc"
_PHYSICS_CONTRACT_PATH = _CONTRACT_PATH.with_name("semantic_contract_v0_3_physics_1.json")
PHYSICS_SOURCE_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/physics-source-1"
PHYSICS_SOURCE_CONTRACT_SHA256 = "ba13f2aefd7a38bd725e5f111e6ec30144bc8776aa957c6278ee7b1178298ba1"
_PHYSICS_SOURCE_CONTRACT_PATH = _CONTRACT_PATH.with_name("semantic_contract_v0_3_physics_source_1.json")
# Resolved load/reference-state method: one table, one profile; the AnalysisRun
# record mirrors physics-1 and carries no contract_evidence.
LOAD_REFERENCE_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/load-reference-1"
LOAD_REFERENCE_CONTRACT_SHA256 = "44bc41c06f589fab6ce931ac0eaa5344765ff64fd5f880cc2dd69ecb839c4f4d"
_LOAD_REFERENCE_CONTRACT_PATH = _CONTRACT_PATH.with_name("semantic_contract_v0_3_load_reference_1.json")
LOAD_REFERENCE_PROFILE = "resolved_straight_load_state_v1"
# Joined load/reference-state method (retained-source receipt required): one
# table, one profile, one receipt policy. The AnalysisRun record mirrors
# physics-source-1 and carries source_block_recovery and contract_evidence.
LOAD_REFERENCE_SOURCE_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/load-reference-source-1"
LOAD_REFERENCE_SOURCE_CONTRACT_SHA256 = "d1628194a7730f427843b00228dd233cf92b8e7d26f3bc31c660a3ea59e28337"
_LOAD_REFERENCE_SOURCE_CONTRACT_PATH = _CONTRACT_PATH.with_name("semantic_contract_v0_3_load_reference_source_1.json")
LOAD_REFERENCE_SOURCE_PROFILE = "resolved_straight_load_state_source_v1"
PREVIEW_PHYSICS_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/preview-physics-1"
PREVIEW_PHYSICS_CONTRACT_SHA256 = "ae55503d44a4750714a35c423623e38cf4132099134097193024d1635bfbc88a"
_PREVIEW_PHYSICS_CONTRACT_PATH = _CONTRACT_PATH.with_name("semantic_contract_v0_3_preview_physics_1.json")
# U6b (D-U6-6; D2 4.7 S-1, 4.9.6): the F2a preview successor. Its statements are
# checked only by the accepted reader (`retained_precision`), and its standing
# comes only from that reader's verified receipt (D1 5 item 3).
PREVIEW_PHYSICS_RETAINED_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/preview-physics-retained-1"
PREVIEW_PHYSICS_RETAINED_CONTRACT_SHA256 = "b2b4a54d610aa38c66f5d31921c2d8f3113313e33eb6933e45093ba6f1e3667c"
_PREVIEW_PHYSICS_RETAINED_CONTRACT_PATH = _CONTRACT_PATH.with_name("semantic_contract_v0_3_preview_physics_retained_1.json")
# B3b (B3-D §2, §6.1): the exact successor over physics-1. As the preview successor, its statements are checked
# only by the accepted reader, which dispatches on this identity, and its standing comes only from that reader.
PHYSICS_RETAINED_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/physics-retained-1"
PHYSICS_RETAINED_CONTRACT_SHA256 = "c4987e874889645ac315b5f55f58690082ad5e7745527f20e3e316efa3e70a3d"
_PHYSICS_RETAINED_CONTRACT_PATH = _CONTRACT_PATH.with_name("semantic_contract_v0_3_physics_retained_1.json")
RETAINED_CONTRACTS = {PREVIEW_PHYSICS_RETAINED_CONTRACT_ID: (PREVIEW_PHYSICS_RETAINED_CONTRACT_SHA256, _PREVIEW_PHYSICS_RETAINED_CONTRACT_PATH),
                      PHYSICS_RETAINED_CONTRACT_ID: (PHYSICS_RETAINED_CONTRACT_SHA256, _PHYSICS_RETAINED_CONTRACT_PATH)}
RETAINED_METHOD = "contribution_preserving_multiprecision_v1"
RETAINED_PRECISION_DOWNGRADE_FORBIDDEN = "RETAINED_PRECISION_DOWNGRADE_FORBIDDEN"
ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH = "ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH"
ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN = "ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN"
RULE_QUANTITY_BELOW_VERIFIED_FLOOR = "RULE_QUANTITY_BELOW_VERIFIED_FLOOR"
RULE_QUANTITY_NOT_COVERED = "RULE_QUANTITY_NOT_COVERED"
# Static fresh-publication identities (S1 §10); no route predicate. T1 added its
# load-reference identities here on activation.
FRESH_CONTRACT_IDS = frozenset({PREVIEW_PHYSICS_CONTRACT_ID, SOURCE_BLOCKS_CONTRACT_ID, PHYSICS_CONTRACT_ID, PHYSICS_SOURCE_CONTRACT_ID,
                                # T1 activation (DESIGN 10.3, SF-4): 0.4.0 exact-route identities only.
                                LOAD_REFERENCE_CONTRACT_ID, LOAD_REFERENCE_SOURCE_CONTRACT_ID,
                                # U6b (D-U6-6): membership is not standing.
                                PREVIEW_PHYSICS_RETAINED_CONTRACT_ID})
# Every current-record identity, which the 0.3 AnalysisRun builder and validator admit.
CURRENT_RECORD_CONTRACT_IDS = frozenset({PRECISION_CONTRACT_ID, PHYSICS_CONTRACT_ID, SOURCE_BLOCKS_CONTRACT_ID, PHYSICS_SOURCE_CONTRACT_ID,
                                         PREVIEW_PHYSICS_CONTRACT_ID, LOAD_REFERENCE_CONTRACT_ID, LOAD_REFERENCE_SOURCE_CONTRACT_ID,
                                         PREVIEW_PHYSICS_RETAINED_CONTRACT_ID, PHYSICS_RETAINED_CONTRACT_ID})
PRECISION_1_HISTORICAL_SEMANTICS = "PRECISION_1_HISTORICAL_SEMANTICS"
SOURCE_BLOCKS_ORDINARY_CASE_LEGACY_SEMANTICS = "SOURCE_BLOCKS_ORDINARY_CASE_LEGACY_SEMANTICS"
RULE_SOURCE_BLOCKS_SUMMARY_NOT_RELIABLE = "RULE_SOURCE_BLOCKS_SUMMARY_NOT_RELIABLE"



def _has_retained_rows(source: Any) -> bool:
    """F-5 (C1 G6): some raw row carries the W1 method token. Only the
    successor may; every other identity, legacy 0.1.0 included, is refused."""
    rows = source.get("results") if isinstance(source, Mapping) else None
    return isinstance(rows, list) and any(isinstance(row, Mapping) and row.get("recovery_method") == RETAINED_METHOD for row in rows)


def _same_canonical(a: Any, b: Any) -> bool:
    """RV92 S-1: equal as checked canonical JSON bytes, as TS's same() compares, so
    false is not 0 and true is not 1, while 0.0 and 0 are one JSON value. A value
    outside the checked profile equals nothing."""
    try:
        return canonical_json_checked_v1(a) == canonical_json_checked_v1(b)
    except ValueError:
        return False


def _is_retained(source: Any) -> bool:
    producer = source.get("producer") if isinstance(source, Mapping) else None
    contract = producer.get("semantic_contract_id") if isinstance(producer, Mapping) else None
    return isinstance(contract, str) and contract in RETAINED_CONTRACTS


def _retained_validation(source: Mapping[str, Any], invocation: Any = None) -> dict[str, Any]:
    """The accepted reader's G0-G8 on a successor; a G7 failure keeps the base
    validator's own text, every other gate its code."""
    from .retained_precision import RetainedPrecisionError, validate_retained_precision
    try:
        return validate_retained_precision(source, invocation)
    except RetainedPrecisionError as error:
        raise ValueError(error.detail or error.code) from error


def _retained_transport(source: Mapping[str, Any]) -> dict[str, Any]:
    """F-U6b-2 (B6): the accepted reader's transport checks on a successor (G0-G2, then
    the base header in Rust's order and the preview-physics transport metadata on its
    projection; RR "I4 made at `30f3d1b24a`; …", rulings 1 and 2), as Rust's
    for_source_metadata and TS's sourceContractTransport run theirs; never eligible.
    A G7 failure keeps the base validator's own text, every other gate its code."""
    from .retained_precision import RetainedPrecisionError, validate_retained_precision_transport
    try:
        return validate_retained_precision_transport(source)
    except RetainedPrecisionError as error:
        raise ValueError(error.detail or error.code) from error


def _retained_contract(source: Mapping[str, Any], *, check_receipt: bool) -> tuple[str, str, Path]:
    if not check_receipt:
        # F-U6b-2 (B6): a transported successor (its receipt, with or without raw
        # rows) is checked by the reader's transport validator, never admitted unchecked.
        _retained_transport(source)
    else:
        _retained_validation(source)
    contract = source["producer"]["semantic_contract_id"]
    return (contract, *RETAINED_CONTRACTS[contract])


def _not_required_cases_ordinarily_eligible(source: Mapping[str, Any], cases: list[Any]) -> bool:
    """Every case is selected, or not_required and ordinarily eligible by the
    base rules below; any other status is not eligible (D2 4.9.4; I66 F-7)."""
    quality = source.get("numerical_quality", {}).get("cases") if isinstance(source.get("numerical_quality"), Mapping) else None
    quality = quality if isinstance(quality, list) else []
    ids: set[str] = set()
    for key in ("results", "diagnostics"):
        items = source.get(key)
        for item in items if isinstance(items, list) else []:
            item_id = item.get("id") if isinstance(item, Mapping) else None
            if not isinstance(item_id, str) or not item_id or item_id in ids:
                return False
            ids.add(item_id)
    for index, case in enumerate(cases):
        status = case.get("status") if isinstance(case, Mapping) else None
        if status == "selected":
            continue
        if status != "not_required" or index >= len(quality) or not isinstance(quality[index], Mapping):
            return False
        q = quality[index]
        refs = q.get("evidence_refs")
        if not (q.get("basis_ref") == case.get("basis_ref") and q.get("solve_quality") == "checks_passed"
                and q.get("structural_status") == "passive_model_basis" and q.get("model_matrix_fidelity") == "represented_equations_retained"
                and q.get("accuracy_evidence") in {"not_claimed", "reference_verified"}
                and isinstance(refs, list) and refs and all(isinstance(ref, str) and ref in ids for ref in refs)):
            return False
    return True


def _retained_standing_from(validation: Mapping[str, Any], source: Mapping[str, Any], requested_basis_refs: list[Mapping[str, str]]) -> str:
    """D2 4.9.4: numerically_eligible needs an invocation-bound validation with
    eligibility set, requested refs equal to the receipt's case order,
    MECHANICS_SOLVED and the not_required conjunct. numerical_quality never
    contributes."""
    cases = source["retained_precision"]["body"]["cases"]
    expected = [case["basis_ref"] for case in cases]
    if (not validation["invocation_bound"] or not validation["numerical_eligible"] or list(requested_basis_refs) != expected
            or source["status"]["mechanics"] != "MECHANICS_SOLVED" or not _not_required_cases_ordinarily_eligible(source, cases)):
        return "needs_recompute"
    return "numerically_eligible"


def _class_binding_refusal(accuracy_class: Any) -> str | None:
    """The binding refusal of one validated class (D2 4.9.9, before S-I)."""
    return {"absolute_verified": RULE_QUANTITY_BELOW_VERIFIED_FLOOR, "not_covered": RULE_QUANTITY_NOT_COVERED}.get(accuracy_class)


def _retained_binding_refusal(envelope: Mapping[str, Any], row: Mapping[str, Any]) -> str | None:
    """A successor row binds only as its validated class allows; a headline binds
    the row its result_ref names. No validated class: every row is refused."""
    try:
        validation = _retained_validation(envelope)
    except (ValueError, KeyError, TypeError, AttributeError):
        return RULE_QUANTITY_NOT_COVERED
    row_id = row.get("id") if isinstance(row, Mapping) else None
    match = next((c for c in validation["classifications"] if c["result_id"] == row_id), None)
    return _class_binding_refusal(match["class"]) if match is not None else None


def classification_summary(source: Mapping[str, Any], invocation: Any = None,
                           requested_basis_refs: list[Mapping[str, str]] | None = None) -> list[dict[str, Any]]:
    """D2 4.9.9: per-case counts over a successor's validated G5c classes (rows of
    selected cases). `withheld` counts as Current only when the standing with the
    caller's requested refs is numerically_eligible (RV94 S-1, as TS's summary);
    otherwise, with no refs or other refs, it is the not-Current count. Any other
    identity, or a refused statement, returns nothing."""
    try:
        validation = _retained_validation(source, invocation)
    except (ValueError, KeyError, TypeError, AttributeError):
        return []
    return _classification_summary_from(validation, source, requested_basis_refs or [])


def _classification_summary_from(validation: Mapping[str, Any], source: Mapping[str, Any], requested_basis_refs: list[Mapping[str, str]]) -> list[dict[str, Any]]:
    # The invocation only binds the validation; its own cases are never substituted
    # for the caller's requested refs (RV94 N-4).
    current = _retained_standing_from(validation, source, requested_basis_refs) == "numerically_eligible"
    out = []
    for case in source["retained_precision"]["body"]["cases"]:
        case_id = case["basis_ref"]["ref_id"]
        n = {name: 0 for name in ("relative_verified", "absolute_verified", "not_covered", "input_derived", "non_quantity")}
        for c in validation["classifications"]:
            if c["basis_ref"]["ref_id"] == case_id:
                n[c["class"]] += 1
        withheld = n["absolute_verified"] + n["not_covered"] if current else n["relative_verified"] + n["absolute_verified"] + n["not_covered"] + n["input_derived"]
        out.append({"case_id": case_id, "relative_verified": n["relative_verified"], "absolute_verified": n["absolute_verified"],
                    "interval_bindable": 0, "not_covered": n["not_covered"], "input_derived": n["input_derived"],
                    "non_quantity": n["non_quantity"], "withheld": withheld})
    return out


def _source_contract(source: Mapping[str, Any], *, check_receipt: bool = True, rust_header_order: bool = False) -> tuple[str, str, Path]:
    """Interpretation dispatch does not authenticate a producer or qualify Current.

    `rust_header_order` is passed only by the retained reader's transport step, on its projection
    (RR "I4 made at `30f3d1b24a`; RV113's items for ROOT ruled; …", ruling 1). The header then takes
    Rust's `for_source_metadata` order and codes, as TS's does: it has no carrier branch, so a
    `carrier_evidence` member is left to the preview-physics transport metadata check (G7
    SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID), and `source_block_recovery` is checked before
    `contract_evidence`. Every other caller passes nothing and reads exactly as before."""
    if _is_retained(source):
        return _retained_contract(source, check_receipt=check_receipt)
    # F-5: no other identity may carry a receipt member.
    if isinstance(source, Mapping) and "retained_precision" in source:
        raise ValueError(RETAINED_PRECISION_DOWNGRADE_FORBIDDEN)
    version = source.get("schema_version")
    # A source-block receipt belongs only to its explicit method branch below;
    # the unrelated carrier namespace remains unsupported for every profile.
    if "carrier_evidence" in source and not rust_header_order:
        raise ValueError("SOURCE_PRODUCER_CONTRACT_UNSUPPORTED")
    if version == "0.1.0":
        if any(key in source for key in ("producer", "numerical_quality", "formulation_basis", "contract_evidence", "source_block_recovery")):
            raise ValueError("LEGACY_SOURCE_METADATA_CONTRADICTION")
        # F-5, as in Rust and TS: a legacy row with the W1 token is refused too.
        if check_receipt and _has_retained_rows(source):
            raise ValueError(RETAINED_PRECISION_DOWNGRADE_FORBIDDEN)
        return SEMANTIC_CONTRACT_ID, SEMANTIC_CONTRACT_SHA256, _CONTRACT_PATH
    if version != "0.2.0":
        raise ValueError("SOURCE_SCHEMA_VERSION_UNSUPPORTED")
    producer = source.get("producer")
    if not isinstance(producer, Mapping) or not isinstance(producer.get("semantic_contract_id"), str) or producer.get("semantic_contract_id") not in {PRECISION_CONTRACT_ID, PHYSICS_CONTRACT_ID, SOURCE_BLOCKS_CONTRACT_ID, PHYSICS_SOURCE_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_ID, LOAD_REFERENCE_CONTRACT_ID, LOAD_REFERENCE_SOURCE_CONTRACT_ID} or producer != {"component_name": "open_pipe_stress_product_physics", "component_version": "0.2.0", "semantic_contract_id": producer.get("semantic_contract_id")}:
        raise ValueError("SOURCE_PRODUCER_CONTRACT_UNSUPPORTED")
    composite = producer["semantic_contract_id"] == PHYSICS_SOURCE_CONTRACT_ID
    physics = producer["semantic_contract_id"] == PHYSICS_CONTRACT_ID or composite
    preview = producer["semantic_contract_id"] == PREVIEW_PHYSICS_CONTRACT_ID
    load_reference = producer["semantic_contract_id"] == LOAD_REFERENCE_CONTRACT_ID
    joined = producer["semantic_contract_id"] == LOAD_REFERENCE_SOURCE_CONTRACT_ID
    recovery_forbidden = producer["semantic_contract_id"] not in {SOURCE_BLOCKS_CONTRACT_ID, PHYSICS_SOURCE_CONTRACT_ID, LOAD_REFERENCE_SOURCE_CONTRACT_ID} and "source_block_recovery" in source
    if rust_header_order and recovery_forbidden:
        # Ruling 1: Rust's order, the recovery member before the evidence demand.
        raise ValueError("SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN")
    if not physics and not preview and not load_reference and not joined and source.get("contract_evidence") is not None:
        raise ValueError("SOURCE_PHYSICS_CONTRACT_MISMATCH")
    if preview and not isinstance(source.get("contract_evidence"), Mapping):
        raise ValueError("SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED")
    if recovery_forbidden:
        raise ValueError("SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN")
    quality = source.get("numerical_quality")
    if not isinstance(quality, Mapping) or set(quality) != {"value_representation", "publication_quantization", "integrity_policy", "status", "cases"} or quality.get("value_representation") != "finite_binary64" or quality.get("publication_quantization") != "none" or quality.get("integrity_policy") != "M03-INTEGRITY-v1" or not isinstance(quality.get("status"), str) or quality.get("status") not in {"not_assessed", "checks_passed", "sensitive", "unresolved", "failed"} or not isinstance(quality.get("cases"), list):
        # RV108 N1: each enum membership test refuses a non-string value first. A list- or
        # dict-valued enum is unhashable, so a bare `in` raised TypeError, which escaped callers
        # that catch ValueError only. A string is tested as before; no other value was ever a member.
        raise ValueError("SOURCE_NUMERICAL_QUALITY_INVALID")
    statuses = {"not_assessed", "checks_passed", "sensitive", "unresolved", "failed"}
    for case in quality["cases"]:
        if not isinstance(case, Mapping) or set(case) != {"basis_ref", "structural_status", "solve_quality", "model_matrix_fidelity", "accuracy_evidence", "evidence_refs"}:
            raise ValueError("SOURCE_NUMERICAL_CASE_INVALID")
        basis = case.get("basis_ref")
        if not isinstance(basis, Mapping) or set(basis) != {"ref_type", "ref_id"} or not all(isinstance(value, str) and value for value in basis.values()) or not isinstance(case.get("solve_quality"), str) or case.get("solve_quality") not in statuses or not isinstance(case.get("structural_status"), str) or case.get("structural_status") not in {"passive_model_basis", "physical_mechanism_witnessed", "negative_energy_witnessed", "numerically_unresolved"} or not isinstance(case.get("model_matrix_fidelity"), str) or case.get("model_matrix_fidelity") not in {"represented_equations_retained", "assembly_loss_detected", "assembly_uncertainty", "not_assessed"} or not isinstance(case.get("accuracy_evidence"), str) or case.get("accuracy_evidence") not in {"not_claimed", "reference_verified", "unresolved"} or not isinstance(case.get("evidence_refs"), list) or not all(isinstance(item, str) and item for item in case["evidence_refs"]):
            # RV108 N1: the same string guard on each case enum.
            raise ValueError("SOURCE_NUMERICAL_CASE_INVALID")
    formulation = source.get("formulation_basis")
    if not isinstance(formulation, Mapping) or set(formulation) != {"profile_id", "limitations"} or formulation.get("profile_id") != (LOAD_REFERENCE_PROFILE if load_reference else LOAD_REFERENCE_SOURCE_PROFILE if joined else "exact_straight_pressure_v2" if physics else "product_preview_mechanics_v1") or not isinstance(formulation.get("limitations"), list) or not formulation["limitations"] or not all(isinstance(item, str) and item for item in formulation["limitations"]):
        raise ValueError("SOURCE_FORMULATION_BASIS_UNSUPPORTED")
    # F-5 (C1 G6): no raw row outside the successor carries the W1 method token.
    if check_receipt and _has_retained_rows(source):
        raise ValueError(RETAINED_PRECISION_DOWNGRADE_FORBIDDEN)
    if physics and isinstance(source.get("contract_evidence"), Mapping) and "load_reference_states" in source["contract_evidence"]:
        # Already refused by the closed physics namespaces; the shared code names
        # the load-reference downgrade identically in the Rust reader.
        raise ValueError("SOURCE_LOAD_REFERENCE_EVIDENCE_FORBIDDEN")
    if load_reference:
        from .load_reference_evidence import load_reference_table, validate_load_reference_evidence, validate_load_reference_transport_metadata
        load_reference_table()
        if check_receipt:
            validate_load_reference_evidence(source)
        else:
            validate_load_reference_transport_metadata(source)
        return LOAD_REFERENCE_CONTRACT_ID, LOAD_REFERENCE_CONTRACT_SHA256, _LOAD_REFERENCE_CONTRACT_PATH
    if joined:
        from .load_reference_source import load_reference_source_table, validate_load_reference_source_evidence, validate_load_reference_source_transport_metadata
        load_reference_source_table()
        if check_receipt:
            validate_load_reference_source_evidence(source)
        else:
            validate_load_reference_source_transport_metadata(source)
        return LOAD_REFERENCE_SOURCE_CONTRACT_ID, LOAD_REFERENCE_SOURCE_CONTRACT_SHA256, _LOAD_REFERENCE_SOURCE_CONTRACT_PATH
    if composite:
        from .physics_source import validate_physics_source, validate_transport_metadata as validate_composite_transport_metadata
        if check_receipt:
            validate_physics_source(source)
        else:
            # Transport metadata validates its own retained closed receipt/hash;
            # it cannot replay raw publication or qualify a live invocation.
            validate_composite_transport_metadata(source)
        return PHYSICS_SOURCE_CONTRACT_ID, PHYSICS_SOURCE_CONTRACT_SHA256, _PHYSICS_SOURCE_CONTRACT_PATH
    if physics:
        if check_receipt:
            from .physics_evidence import validate_physics_evidence
            validate_physics_evidence(source)
        else:
            from .physics_evidence import validate_transport_metadata
            validate_transport_metadata(source)
        return PHYSICS_CONTRACT_ID, PHYSICS_CONTRACT_SHA256, _PHYSICS_CONTRACT_PATH
    if preview:
        from .preview_physics_evidence import validate_preview_physics_evidence, validate_transport_metadata as validate_preview_transport_metadata
        (validate_preview_physics_evidence if check_receipt else validate_preview_transport_metadata)(source)
        return PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_SHA256, _PREVIEW_PHYSICS_CONTRACT_PATH
    if producer["semantic_contract_id"] == SOURCE_BLOCKS_CONTRACT_ID:
        if check_receipt:
            validate_source_blocks(source)
        else:
            # Transport metadata cannot recompute the unavailable raw publication.
            # It validates its retained body, never numerical standing.
            receipt = source.get("source_block_recovery")
            validate_receipt_shape(receipt)
            if receipt["receipt_sha256"] != domain_hash("source_blocks_receipt_v1", receipt["body"]):
                raise ValueError("SOURCE_BLOCKS_RECEIPT_HASH")
        return SOURCE_BLOCKS_CONTRACT_ID, SOURCE_BLOCKS_CONTRACT_SHA256, _SOURCE_BLOCKS_CONTRACT_PATH
    return PRECISION_CONTRACT_ID, PRECISION_CONTRACT_SHA256, _PRECISION_CONTRACT_PATH


def numerical_use_standing(source: Mapping[str, Any], requested_basis_refs: list[Mapping[str, str]], source_block_context: Mapping[str, Any] | None = None) -> str:
    """Derived numerical eligibility only; never mutates an authentic historical record.

    A positive result still requires the caller's existing model/input/build/source
    authentication. Matching an input hash alone supplies no numerical evidence.
    """
    if _is_retained(source):
        # Validated once, with the invocation; the reader's G7 includes the base dispatch.
        try:
            validation = _retained_validation(source, source_block_context)
        except (ValueError, KeyError, TypeError, AttributeError):
            return "unsupported"
        return _retained_standing_from(validation, source, requested_basis_refs)
    try:
        contract, _, _ = _source_contract(source)
    except (ValueError, KeyError, TypeError, AttributeError):
        return "unsupported"
    # T0R (A2 item 10): validate first; then only static fresh identities without a
    # standing reason may continue.
    if contract not in FRESH_CONTRACT_IDS or _standing_reason(contract, source) is not None:
        return "needs_recompute"
    if contract == LOAD_REFERENCE_SOURCE_CONTRACT_ID:
        # T1 (declared standing edit): admitted joined evidence is never numerically
        # eligible here, because a 0.4.0 resolved case cannot be re-derived from a
        # captured request by a reader. Identical in outcome to the fall-through below.
        return "needs_recompute"
    if contract in {SOURCE_BLOCKS_CONTRACT_ID, PHYSICS_SOURCE_CONTRACT_ID}:
        if not requested_basis_refs or requested_basis_refs != [case["basis_ref"] for case in source["source_block_recovery"]["body"]["cases"]]:
            return "needs_recompute"
        try:
            if contract == PHYSICS_SOURCE_CONTRACT_ID:
                from .physics_source import validate_physics_source
                qualified = validate_physics_source(source, source_block_context)
            else:
                qualified = validate_source_blocks(source, source_block_context)
            return "numerically_eligible" if qualified else "needs_recompute"
        except ValueError:
            return "unsupported"
    if contract not in {PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_ID, LOAD_REFERENCE_CONTRACT_ID}:
        return "needs_recompute"
    quality = source["numerical_quality"]
    # Sensitive evidence remains inspectable but does not qualify source-answer accuracy.
    if quality["status"] != "checks_passed" or not requested_basis_refs:
        return "needs_recompute"
    cases = quality["cases"]
    if len(cases) != len(requested_basis_refs):
        return "needs_recompute"
    if not isinstance(source.get("results"), list) or not isinstance(source.get("diagnostics"), list):
        return "needs_recompute"
    evidence_ids: set[str] = set()
    for item in [*source["results"], *source["diagnostics"]]:
        item_id = item.get("id") if isinstance(item, Mapping) else None
        if not isinstance(item_id, str) or not item_id or item_id in evidence_ids:
            return "needs_recompute"
        evidence_ids.add(item_id)
    for index, basis in enumerate(requested_basis_refs):
        if basis in requested_basis_refs[:index]:
            return "needs_recompute"
        matched = [case for case in cases if isinstance(case, Mapping) and case.get("basis_ref") == basis]
        if len(matched) != 1:
            return "needs_recompute"
        case = matched[0]
        refs = case.get("evidence_refs")
        if case.get("solve_quality") != "checks_passed" or case.get("structural_status") != "passive_model_basis" or case.get("model_matrix_fidelity") != "represented_equations_retained" or case.get("accuracy_evidence") not in {"not_claimed", "reference_verified"} or not isinstance(refs, list) or not refs or not all(isinstance(ref, str) and ref in evidence_ids for ref in refs):
            return "needs_recompute"
    return "numerically_eligible"


def _standing_reason(contract: str, source: Mapping[str, Any]) -> str | None:
    if contract == PRECISION_CONTRACT_ID:
        return PRECISION_1_HISTORICAL_SEMANTICS
    if contract == SOURCE_BLOCKS_CONTRACT_ID and ordinary_case_legacy_semantics(source):
        return SOURCE_BLOCKS_ORDINARY_CASE_LEGACY_SEMANTICS
    return None


def standing_reason(source: Mapping[str, Any]) -> str | None:
    """Derived reason a readable source is not Current; None when none applies.

    Only the named historical semantics are reported. Numerical standing is
    still decided by ``numerical_use_standing``; an unreadable source has no
    reason here and is ``unsupported`` there.
    """
    try:
        contract, _, _ = _source_contract(source)
    except (ValueError, KeyError, TypeError, AttributeError):
        return None
    return _standing_reason(contract, source)


def is_fresh_contract_id(contract_id: Any) -> bool:
    return isinstance(contract_id, str) and contract_id in FRESH_CONTRACT_IDS


def rule_binding_refusal(envelope: Mapping[str, Any], row: Mapping[str, Any]) -> str | None:
    """Mirror of Rust ``semantic_contract::rule_binding_refusal`` (S1 §10).

    A non-composite source-blocks-1 summary stress is an absolute sum, not the
    circular-section maximum, so no rule may bind to it until T3.
    """
    if _is_retained(envelope):
        return _retained_binding_refusal(envelope, row)
    producer = envelope.get("producer") if isinstance(envelope, Mapping) else None
    if not isinstance(producer, Mapping) or producer.get("semantic_contract_id") != SOURCE_BLOCKS_CONTRACT_ID or not isinstance(row, Mapping):
        return None
    if row.get("kind") == "open_formula_stress_summary":
        return RULE_SOURCE_BLOCKS_SUMMARY_NOT_RELIABLE
    summary = envelope.get("summary")
    headline = summary.get("max_open_formula_stress") if isinstance(summary, Mapping) else None
    if isinstance(headline, Mapping) and isinstance(row.get("id"), str) and row.get("id") == headline.get("result_ref"):
        return RULE_SOURCE_BLOCKS_SUMMARY_NOT_RELIABLE
    return None


def build_analysis_run_v0_2(mechanics_result: Mapping[str, Any], **kwargs: Any) -> dict[str, Any]:
    """Explicit historical construction; includes original synthetic fixture meanings.

    Excluded from current source dispatch and refuses precision metadata.
    """
    return _build_analysis_run(mechanics_result, record_version="0.2.0", **kwargs)


RULE_STATUSES = {"RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED"}


def _source_reference_text(value: Any) -> str:
    if not isinstance(value, str) or not value:
        raise ValueError("ANALYSIS_SOURCE_REFERENCE_INVALID")
    return value


def _source_basis_reference(basis: Any) -> dict[str, str] | None:
    if basis is None:
        return None
    if not isinstance(basis, Mapping) or set(basis) != {"ref_type", "ref_id"}:
        raise ValueError("ANALYSIS_SOURCE_REFERENCE_INVALID")
    kind = _source_reference_text(basis.get("ref_type"))
    ref_id = _source_reference_text(basis.get("ref_id"))
    return {"object_type": {"load_case": "LoadCase", "combination": "Combination"}.get(kind, "ResultBasis"), "ref": ref_id}


def _validate_source_reference_fields(source: Mapping[str, Any]) -> None:
    for field in ("run_id", "model_ref"):
        _source_reference_text(source.get(field))
    for row in source.get("results", []):
        _source_reference_text(row.get("id"))
        _source_basis_reference(row.get("basis_ref"))


def _expected_load_basis(source: Mapping[str, Any], expected_basis_refs: list[Mapping[str, str]] | None) -> list[dict[str, str]]:
    """Available source bases are not evidence of requested-model completeness.

    An override is independent caller evidence and must be supplied again when
    validating; the record's own list is never used as its warrant.
    """
    if expected_basis_refs is not None:
        refs = deepcopy(expected_basis_refs)
        if not isinstance(refs, list) or any(not isinstance(ref, Mapping) or set(ref) != {"object_type", "ref"} or ref["object_type"] not in {"LoadCase", "Combination", "ResultBasis"} or not isinstance(ref["ref"], str) or not ref["ref"] for ref in refs) or any(ref in refs[:index] for index, ref in enumerate(refs)):
            raise ValueError("ANALYSIS_LOAD_BASIS_INVALID")
        for item in [*source.get("results", []), *source.get("numerical_quality", {}).get("cases", [])]:
            required = _source_basis_reference(item.get("basis_ref"))
            if required is not None:
                if required not in refs:
                    raise ValueError("ANALYSIS_LOAD_BASIS_SOURCE_SCOPE_MISMATCH")
        return refs
    refs = []
    for row in source.get("results", []):
        ref = _source_basis_reference(row.get("basis_ref"))
        if ref is not None and ref not in refs:
            refs.append(ref)
    return refs


def build_analysis_run_v0_3(mechanics_result: Mapping[str, Any], *, expected_basis_refs: list[Mapping[str, str]] | None = None, **kwargs: Any) -> dict[str, Any]:
    """Strict source-bound 0.3 record; optional basis is independent evidence."""
    _source_contract(mechanics_result)
    _validate_source_reference_fields(mechanics_result)
    producer = mechanics_result["producer"]
    kwargs.setdefault("solver_name", producer["component_name"])
    kwargs.setdefault("solver_version", producer["component_version"])
    source_rule = mechanics_result.get("status", {}).get("rule_check")
    if source_rule is not None and (not isinstance(source_rule, str) or source_rule not in RULE_STATUSES):
        raise ValueError("ANALYSIS_RULE_STATUS_INVALID")
    override = kwargs.get("rule_check_status")
    effective_rule = override if override is not None else (source_rule if source_rule is not None else "RULE_INPUTS_INCOMPLETE")
    if not isinstance(effective_rule, str) or effective_rule not in RULE_STATUSES:
        raise ValueError("ANALYSIS_RULE_STATUS_INVALID")
    kwargs["rule_check_status"] = effective_rule
    record = _build_analysis_run(mechanics_result, record_version="0.3.0", **kwargs)
    record["analysis_run"]["load_basis_refs"] = _expected_load_basis(mechanics_result, expected_basis_refs)
    hash_fn = kwargs.get("hash_fn", canonical_sha256_checked_v1)
    record["analysis_run"]["hashes"][0]["value"] = hash_fn(analysis_record_projection(record))
    validate_analysis_run_v0_3(record, mechanics_result, hash_fn=hash_fn, expected_basis_refs=expected_basis_refs)
    return record


def build_analysis_run(mechanics_result: Mapping[str, Any], **kwargs: Any) -> dict[str, Any]:
    contract, _, _ = _source_contract(mechanics_result)
    return build_analysis_run_v0_3(mechanics_result, **kwargs) if contract in CURRENT_RECORD_CONTRACT_IDS else build_analysis_run_v0_2(mechanics_result, **kwargs)


def validate_analysis_run_v0_3(envelope: Mapping[str, Any], source: Mapping[str, Any], *, hash_fn: Callable[[Any], str] = canonical_sha256_checked_v1, expected_basis_refs: list[Mapping[str, str]] | None = None) -> None:
    """Validate source interpretation/bindings separately from checksum verification.

    This accepts supplied evidence only; it does not authenticate a claimed build.
    """
    contract_id, contract_hash, path = _source_contract(source)
    _validate_source_reference_fields(source)
    if contract_id not in CURRENT_RECORD_CONTRACT_IDS or envelope.get("schema_version") != "0.3.0" or envelope.get("run_contract_status", {}).get("record_contract") != "strict_analysis_run_v0_3":
        raise ValueError("ANALYSIS_SOURCE_CONTRACT_VERSION_MISMATCH")
    run = envelope.get("analysis_run", {})
    if contract_id in {SOURCE_BLOCKS_CONTRACT_ID, PHYSICS_SOURCE_CONTRACT_ID, LOAD_REFERENCE_SOURCE_CONTRACT_ID}:
        if not _same_canonical(run.get("source_block_recovery"), source["source_block_recovery"]):
            raise ValueError("ANALYSIS_SOURCE_BLOCK_RECEIPT_MISMATCH")
    elif "source_block_recovery" in run:
        raise ValueError("SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN")
    if contract_id in {PHYSICS_SOURCE_CONTRACT_ID, LOAD_REFERENCE_SOURCE_CONTRACT_ID}:
        if not _same_canonical(run.get("contract_evidence"), source["contract_evidence"]):
            raise ValueError("ANALYSIS_PHYSICS_SOURCE_EVIDENCE_MISMATCH")
    elif "contract_evidence" in run:
        raise ValueError("ANALYSIS_PHYSICS_SOURCE_DOWNGRADE_FORBIDDEN")
    if contract_id in RETAINED_CONTRACTS:
        if "retained_precision" not in run or not _same_canonical(run["retained_precision"], source["retained_precision"]):
            raise ValueError(ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH)
    elif "retained_precision" in run:
        raise ValueError(ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN)
    if run.get("diagnostics") != [{"source_annotation": deepcopy(item)} for item in source.get("diagnostics", [])]:
        raise ValueError("ANALYSIS_SOURCE_DIAGNOSTICS_MISMATCH")
    statuses = run.get("analysis_status")
    source_rule = source.get("status", {}).get("rule_check")
    if source_rule is not None and (not isinstance(source_rule, str) or source_rule not in RULE_STATUSES):
        raise ValueError("ANALYSIS_RULE_STATUS_INVALID")
    mechanics = source.get("status", {}).get("mechanics")
    if mechanics not in {"MODEL_INCOMPLETE", "MECHANICS_SOLVED"}:
        raise ValueError("ANALYSIS_SOURCE_MECHANICS_STATUS_INVALID")
    if not isinstance(statuses, list) or any(not isinstance(item, str) for item in statuses) or len(statuses) != len(set(statuses)) or len(set(statuses) & RULE_STATUSES) != 1 or set(statuses) - RULE_STATUSES != {mechanics, "HUMAN_REVIEW_REQUIRED"}:
        raise ValueError("ANALYSIS_SOURCE_STATUS_MISMATCH")
    if run.get("load_basis_refs") != _expected_load_basis(source, expected_basis_refs):
        raise ValueError("ANALYSIS_SOURCE_LOAD_BASIS_MISMATCH")
    solver = run.get("solver_version", {})
    if solver.get("solver_name") != source["producer"]["component_name"] or solver.get("solver_version") != source["producer"]["component_version"]:
        raise ValueError("ANALYSIS_SOURCE_PRODUCER_MISMATCH")
    if run.get("model_state_ref") != {"object_type":"ModelState", "ref":f"state:{source.get('model_ref')}:preview"}:
        raise ValueError("ANALYSIS_SOURCE_MODEL_MISMATCH")
    if run.get("run_id") != source.get("run_id"):
        raise ValueError("ANALYSIS_SOURCE_RUN_MISMATCH")
    semantic = {"id": contract_id, "sha256": contract_hash}
    if run.get("reproducibility", {}).get("semantic_contract") != semantic:
        raise ValueError("ANALYSIS_SEMANTIC_CONTRACT_MISMATCH")
    hashes = [item for item in run.get("hashes", []) if item.get("payload_scope") == "received_result"]
    expected_hash = _checksum("received_result", {"object_type": "ResultEnvelope", "ref": f"result-envelope:{source.get('run_id')}"}, source, hash_fn)
    if hashes != [expected_hash]:
        raise ValueError("ANALYSIS_RECEIVED_SOURCE_MISMATCH")
    rows = source.get("results", [])
    refs = run.get("result_refs", [])
    if len(rows) != len(refs) or len({row.get("id") for row in rows}) != len(rows):
        raise ValueError("ANALYSIS_ROW_ACCOUNTING_MISMATCH")
    for index, (row, ref) in enumerate(zip(rows, refs)):
        interpretation, findings = _semantic(row, path)
        expected_semantic = {**semantic, "signature_id": interpretation.get("signature_id") if interpretation else None}
        expected_ref = {"object_type": "Result", "ref": str(row.get("id", "result:unknown"))}
        if ref.get("source_row_index") != index or ref.get("result_ref") != expected_ref or ref.get("semantic_contract") != expected_semantic or ref.get("hash_refs") != [_checksum("result_row", expected_ref, row, hash_fn)]:
            raise ValueError("ANALYSIS_ROW_SOURCE_BINDING_MISMATCH")
        if ref.get("category") != (interpretation.get("category") if interpretation else "unknown") or ref.get("source_dimension") != (interpretation.get("source_physical_semantic_dimension") if interpretation else None) or ref.get("result_family") != (interpretation.get("family") if interpretation else None) or ref.get("interpretation") != {"status": interpretation.get("canonical_disposition") if interpretation else "unavailable", "findings": findings} or ref.get("source_annotation") != {"kind": row.get("kind"), "unit": row.get("unit"), "metadata": row.get("metadata")}:
            raise ValueError("ANALYSIS_ROW_INTERPRETATION_MISMATCH")
    if verify_analysis_run_record(envelope, hash_fn) != "match":
        raise ValueError("ANALYSIS_RECORD_CHECKSUM_MISMATCH")
