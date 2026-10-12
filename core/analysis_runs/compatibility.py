"""Versioned analysis-record compatibility and strict 0.2 construction.

The analysis-run record builder and validator (DEL-14-02). Every reading of a result
statement here is the Rust product reader's verdict (b7-plan R1, R2), taken through
`rust_reader` (`openpipestress-runner read`): the header dispatch and each route's reader
(`_source_contract`), standing, classification summary and rule binding. Nothing here
re-derives a verdict; the record builder projects the table the reader bound.
The Python reading these functions held before R2 is now a test verifier only.
"""
from __future__ import annotations

from copy import deepcopy
from pathlib import Path
import json
from typing import Any, Callable, Mapping

from core.serialization.canonical_json.adapter import canonical_json_checked_v1, canonical_sha256_checked_v1
from . import rust_reader

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
SOURCE_BLOCKS_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/source-blocks-1"
SOURCE_BLOCKS_CONTRACT_SHA256 = "5f299065f15a157bbedf9467a598994ae684c4ecb3f851bbcb291981ec550a9f"
_SOURCE_BLOCKS_CONTRACT_PATH = _CONTRACT_PATH.with_name("semantic_contract_v0_3_source_blocks_1.json")
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



# The members a carrier keeps of a statement: what a transport reading reads.
_TRANSPORTED = ("schema_version", "producer", "numerical_quality", "formulation_basis", "contract_evidence", "source_block_recovery", "carrier_evidence")
# Each table a route binds, by identity: the reader's binding `{id, sha256}` must name one of these.
_TABLES = {PRECISION_CONTRACT_ID: (PRECISION_CONTRACT_SHA256, _PRECISION_CONTRACT_PATH),
           PHYSICS_CONTRACT_ID: (PHYSICS_CONTRACT_SHA256, _PHYSICS_CONTRACT_PATH),
           SOURCE_BLOCKS_CONTRACT_ID: (SOURCE_BLOCKS_CONTRACT_SHA256, _SOURCE_BLOCKS_CONTRACT_PATH),
           PHYSICS_SOURCE_CONTRACT_ID: (PHYSICS_SOURCE_CONTRACT_SHA256, _PHYSICS_SOURCE_CONTRACT_PATH),
           PREVIEW_PHYSICS_CONTRACT_ID: (PREVIEW_PHYSICS_CONTRACT_SHA256, _PREVIEW_PHYSICS_CONTRACT_PATH),
           LOAD_REFERENCE_CONTRACT_ID: (LOAD_REFERENCE_CONTRACT_SHA256, _LOAD_REFERENCE_CONTRACT_PATH),
           LOAD_REFERENCE_SOURCE_CONTRACT_ID: (LOAD_REFERENCE_SOURCE_CONTRACT_SHA256, _LOAD_REFERENCE_SOURCE_CONTRACT_PATH),
           **RETAINED_CONTRACTS}


def _has_retained_rows(source: Any) -> bool:
    """F-5 (C1 G6): some raw row carries the W1 method token. The legacy 0.2
    builder refuses such a source before any reading."""
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


def _refused(refusal: rust_reader.ReaderRefusal) -> ValueError:
    """The reader's refusal as product code raises it: its whole text (detail), otherwise its code."""
    return ValueError(refusal.detail or refusal.code)


def _retained_validation(source: Mapping[str, Any], invocation: Any = None) -> dict[str, Any]:
    """The Rust reader's G0-G8 on a successor (`retained_precision::validate`, through
    `openpipestress-runner read`). A refusal raises the Rust reader's detail when it
    carries one (a G7 base failure's text, in Rust's base validator's words), otherwise
    its code."""
    try:
        return rust_reader.retained_precision(source, invocation)
    except rust_reader.ReaderRefusal as refusal:
        raise _refused(refusal) from refusal


def _retained_transport(source: Mapping[str, Any]) -> dict[str, Any]:
    """F-U6b-2 (B6): the Rust reader's transport checks on a successor
    (`retained_precision::validate_transport_metadata`, through `openpipestress-runner
    read`: G0-G2, then the base header and the base's transport metadata on its
    projection); never eligible. A refusal raises the Rust reader's detail when it
    carries one (a G7 base failure's text, in Rust's base validator's words), otherwise
    its code."""
    try:
        return rust_reader.retained_precision_transport(source)
    except rust_reader.ReaderRefusal as refusal:
        raise _refused(refusal) from refusal


def _retained_contract(source: Mapping[str, Any], *, check_receipt: bool) -> tuple[str, str, Path]:
    if not check_receipt:
        # F-U6b-2 (B6): a transported successor (its receipt, with or without raw
        # rows) is checked by the reader's transport validator, never admitted unchecked.
        _retained_transport(source)
    else:
        _retained_validation(source)
    contract = source["producer"]["semantic_contract_id"]
    return (contract, *RETAINED_CONTRACTS[contract])


def _source_contract(source: Mapping[str, Any], *, check_receipt: bool = True) -> tuple[str, str, Path]:
    """The Rust reader's reading of a statement: its table `(id, sha256, path)`, or its refusal raised.

    A successor goes to the retained reader (R1). Every other statement takes the Rust header
    dispatch (`reader_verdict::dispatch`, b7-plan R2 G5), then its route's reader: the raw
    validation (`check_receipt`), or the transport checks of a statement without raw rows.
    Interpretation dispatch does not authenticate a producer or qualify Current."""
    if _is_retained(source):
        return _retained_contract(source, check_receipt=check_receipt)
    # A transport reading reads the transported members only (as Rust's header-only `for_source_metadata`
    # and the carriers that keep them), never raw rows the statement may still hold.
    verdict = rust_reader.dispatch(source if check_receipt or not isinstance(source, Mapping)
                                   else {key: source[key] for key in _TRANSPORTED if key in source})
    if not verdict["ok"]:
        error = verdict["error"]
        raise ValueError(error["detail"] or error["code"])
    route = verdict["route"]
    if route in {"retained_preview_physics", "retained_physics"}:
        # Only a successor identity routes here, and it went to the retained reader above.
        raise RuntimeError(f"RUST-READER-RESPONSE-INVALID: {rust_reader.DISPATCH}")
    reader = rust_reader.ROUTE_READERS.get(route)
    if reader is not None:
        try:
            if check_receipt:
                rust_reader.validate(reader, source)
            else:
                rust_reader.transport(reader, source)
        except rust_reader.ReaderRefusal as refusal:
            raise _refused(refusal) from refusal
    if route == "legacy":
        return SEMANTIC_CONTRACT_ID, SEMANTIC_CONTRACT_SHA256, _CONTRACT_PATH
    binding = verdict["binding"]
    table = _TABLES.get(binding["id"])
    if table is None or table[0] != binding["sha256"]:
        raise RuntimeError(f"RUST-READER-BINDING-UNKNOWN: {binding['id']}")
    return binding["id"], *table


def numerical_use_standing(source: Mapping[str, Any], requested_basis_refs: list[Mapping[str, str]], source_block_context: Mapping[str, Any] | None = None) -> str:
    """Derived numerical eligibility only; never mutates an authentic historical record.

    The Rust reader's `numerical_use_standing_with_context`. A positive result still
    requires the caller's existing model/input/build/source authentication. Matching
    an input hash alone supplies no numerical evidence. A statement that is not JSON
    is unsupported.
    """
    try:
        return rust_reader.standing(source, requested_basis_refs, source_block_context)["token"]
    except (ValueError, TypeError):
        return "unsupported"


def standing_reason(source: Mapping[str, Any]) -> str | None:
    """Derived reason a readable source is not Current; None when none applies.

    The Rust reader's `standing_reason` of a statement it reads (`_source_contract`).
    Only the named historical semantics are reported. Numerical standing is still
    decided by ``numerical_use_standing``; an unreadable source has no reason here
    and is ``unsupported`` there.
    """
    try:
        _source_contract(source)
    except (ValueError, KeyError, TypeError, AttributeError):
        return None
    return rust_reader.standing(source, [])["reason"]


def classification_summary(source: Mapping[str, Any], invocation: Any = None,
                           requested_basis_refs: list[Mapping[str, str]] | None = None) -> list[dict[str, Any]]:
    """D2 4.9.9: the Rust reader's per-case counts over a successor's validated G5c classes
    (`semantic_contract::classification_summary`). Any other identity, or a refused
    statement (one that is not JSON included), returns nothing."""
    try:
        return rust_reader.classification_summary(source, invocation, requested_basis_refs or [])
    except (ValueError, TypeError):
        return []


def is_fresh_contract_id(contract_id: Any) -> bool:
    return isinstance(contract_id, str) and contract_id in FRESH_CONTRACT_IDS


def rule_binding_refusal(envelope: Mapping[str, Any], row: Mapping[str, Any]) -> str | None:
    """The Rust reader's ``semantic_contract::rule_binding_refusal`` (S1 §10; D2 4.9.9).

    A non-composite source-blocks-1 summary stress is an absolute sum, not the
    circular-section maximum, so no rule may bind to it until T3; a successor row
    binds only as its validated class allows. A statement that is not JSON is refused
    (ValueError or TypeError): no reader reads it, so no row of it is bound.
    """
    return rust_reader.rule_binding(envelope, row)


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
