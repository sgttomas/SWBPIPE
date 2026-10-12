"""Test verifier of the base-route dispatch and standing (b7-plan R2): Python's reading of
`core/reporting/result_export/src/semantic_contract.rs` (`for_source`, `for_source_metadata`,
`numerical_use_standing_with_context`, `standing_reason`, `rule_binding_refusal`,
`classification_summary`).

Tests only. The product reader is Rust's: product Python (`core/analysis_runs/compatibility.py`) takes its
verdict through `core/analysis_runs/rust_reader.py` and keeps only the analysis-run record builder. This module
is the reader half that product file held before R2. It imports nothing from product code: retained successors
are read by `tests/verifiers/retained_precision.py`, hashes go through `tests/verifiers/canonical_json.py`
(`tests/test_verifier_independence.py`).
"""
from __future__ import annotations

from pathlib import Path
from typing import Any, Mapping

from .source_blocks import (CONTRACT_ID as SOURCE_BLOCKS_CONTRACT_ID,
    CONTRACT_SHA256 as SOURCE_BLOCKS_CONTRACT_SHA256, CONTRACT_PATH as _SOURCE_BLOCKS_CONTRACT_PATH,
    validate_source_blocks, validate_receipt_shape, domain_hash, ordinary_case_legacy_semantics)

SEMANTIC_CONTRACT_SHA256 = "4d6886d19e304db897e5e9f8f0054cbee91ba7795868f9698e2bbe070bde94da"
SEMANTIC_CONTRACT_ID = "openpipestress_result_semantics_v0_2"
_CONTRACT_PATH = Path(__file__).resolve().parents[2] / "fixtures" / "results" / "semantic_contract_v0_2.json"


PRECISION_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/precision-1"
PRECISION_CONTRACT_SHA256 = "d75aacee175e178dbdeb256d89a65f4b375265f7da077725ee635af33df51d7e"
_PRECISION_CONTRACT_PATH = _CONTRACT_PATH.with_name("semantic_contract_v0_3_precision_1.json")
PHYSICS_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/physics-1"
PHYSICS_CONTRACT_SHA256 = "9a2cf6268b57bd5265a1a115497c07450819dd4d03cd5ab618097bd9d19da8cc"
_PHYSICS_CONTRACT_PATH = _CONTRACT_PATH.with_name("semantic_contract_v0_3_physics_1.json")
PHYSICS_SOURCE_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/physics-source-1"
PHYSICS_SOURCE_CONTRACT_SHA256 = "ba13f2aefd7a38bd725e5f111e6ec30144bc8776aa957c6278ee7b1178298ba1"
_PHYSICS_SOURCE_CONTRACT_PATH = _CONTRACT_PATH.with_name("semantic_contract_v0_3_physics_source_1.json")
PRESSURE_SOURCE_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/pressure-source-1"
PRESSURE_SOURCE_CONTRACT_SHA256 = "3039f7ca7d36a82f9d34baea648159d7a780ec9623879ce5aa3c890b10b19a61"
_PRESSURE_SOURCE_CONTRACT_PATH = _CONTRACT_PATH.with_name("semantic_contract_v0_3_pressure_source_1.json")

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
PREVIEW_PHYSICS_CONTRACT_SHA256 = "ac522c1ee0c771ae1e3dc72facfe3e75a8f0b01009075c997658eae50e9bb60c"
_PREVIEW_PHYSICS_CONTRACT_PATH = _CONTRACT_PATH.with_name("semantic_contract_v0_3_preview_physics_1.json")
# U6b (D-U6-6; D2 4.7 S-1, 4.9.6): the F2a preview successor. Its statements are
# checked only by the accepted reader (`retained_precision`), and its standing
# comes only from that reader's verified receipt (D1 5 item 3).
PREVIEW_PHYSICS_RETAINED_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/preview-physics-retained-1"
PREVIEW_PHYSICS_RETAINED_CONTRACT_SHA256 = "ed1fc7fca562f26365157141f54c450cbfaf2ef65e689d37ff54086b70510357"
_PREVIEW_PHYSICS_RETAINED_CONTRACT_PATH = _CONTRACT_PATH.with_name("semantic_contract_v0_3_preview_physics_retained_1.json")
# B3b (B3-D §2, §6.1): the exact successor over physics-1. As the preview successor, its statements are checked
# only by the accepted reader, which dispatches on this identity, and its standing comes only from that reader.
PHYSICS_RETAINED_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/physics-retained-1"
PHYSICS_RETAINED_CONTRACT_SHA256 = "7f830a90e7a5f7e1ec279ae0c2bf0e622ff80f40ac283ae502096880d32b5eb9"
_PHYSICS_RETAINED_CONTRACT_PATH = _CONTRACT_PATH.with_name("semantic_contract_v0_3_physics_retained_1.json")
RETAINED_CONTRACTS = {PREVIEW_PHYSICS_RETAINED_CONTRACT_ID: (PREVIEW_PHYSICS_RETAINED_CONTRACT_SHA256, _PREVIEW_PHYSICS_RETAINED_CONTRACT_PATH),
                      PHYSICS_RETAINED_CONTRACT_ID: (PHYSICS_RETAINED_CONTRACT_SHA256, _PHYSICS_RETAINED_CONTRACT_PATH)}
RETAINED_METHOD = "contribution_preserving_multiprecision_v1"
RETAINED_PRECISION_DOWNGRADE_FORBIDDEN = "RETAINED_PRECISION_DOWNGRADE_FORBIDDEN"
RULE_QUANTITY_BELOW_VERIFIED_FLOOR = "RULE_QUANTITY_BELOW_VERIFIED_FLOOR"
RULE_QUANTITY_NOT_COVERED = "RULE_QUANTITY_NOT_COVERED"
# Static fresh-publication identities (S1 §10); no route predicate. T1 added its
# load-reference identities here on activation.
FRESH_CONTRACT_IDS = frozenset({PREVIEW_PHYSICS_CONTRACT_ID, SOURCE_BLOCKS_CONTRACT_ID, PHYSICS_CONTRACT_ID, PHYSICS_SOURCE_CONTRACT_ID, PRESSURE_SOURCE_CONTRACT_ID,
                                # T1 activation (DESIGN 10.3, SF-4): 0.4.0 exact-route identities only.
                                LOAD_REFERENCE_CONTRACT_ID, LOAD_REFERENCE_SOURCE_CONTRACT_ID,
                                # U6b (D-U6-6): membership is not standing.
                                PREVIEW_PHYSICS_RETAINED_CONTRACT_ID})
PRECISION_1_HISTORICAL_SEMANTICS = "PRECISION_1_HISTORICAL_SEMANTICS"
SOURCE_BLOCKS_ORDINARY_CASE_LEGACY_SEMANTICS = "SOURCE_BLOCKS_ORDINARY_CASE_LEGACY_SEMANTICS"
RULE_SOURCE_BLOCKS_SUMMARY_NOT_RELIABLE = "RULE_SOURCE_BLOCKS_SUMMARY_NOT_RELIABLE"


def _has_retained_rows(source: Any) -> bool:
    """F-5 (C1 G6): some raw row carries the W1 method token. Only the
    successor may; every other identity, legacy 0.1.0 included, is refused."""
    rows = source.get("results") if isinstance(source, Mapping) else None
    return isinstance(rows, list) and any(isinstance(row, Mapping) and row.get("recovery_method") == RETAINED_METHOD for row in rows)


def _is_retained(source: Any) -> bool:
    producer = source.get("producer") if isinstance(source, Mapping) else None
    contract = producer.get("semantic_contract_id") if isinstance(producer, Mapping) else None
    return isinstance(contract, str) and contract in RETAINED_CONTRACTS


def _retained_validation(source: Mapping[str, Any], invocation: Any = None) -> dict[str, Any]:
    """The retained verifier's G0-G8 on a successor (`tests/verifiers/retained_precision.py`). A refusal raises
    its detail when it carries one, otherwise its code."""
    from .retained_precision import RetainedPrecisionError, validate_retained_precision
    try:
        return validate_retained_precision(source, invocation)
    except RetainedPrecisionError as refusal:
        raise ValueError(refusal.detail or refusal.code) from refusal


def _retained_transport(source: Mapping[str, Any]) -> dict[str, Any]:
    """F-U6b-2 (B6): the retained verifier's transport checks on a successor; never eligible."""
    from .retained_precision import RetainedPrecisionError, validate_retained_precision_transport
    try:
        return validate_retained_precision_transport(source)
    except RetainedPrecisionError as refusal:
        raise ValueError(refusal.detail or refusal.code) from refusal


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

    `rust_header_order` is passed only by the retained-precision test verifier's transport step, on its projection
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
    # The joined receipt cannot be downgraded to ordinary pressure-1, including
    # transport statements. This is a source-qualified exception only.
    if producer == {"component_name":"open_pipe_stress_product_physics", "component_version":"0.2.0", "semantic_contract_id":"openpipestress.result_semantics/0.3.0/pressure-1"} and "source_block_recovery" in source:
        raise ValueError("SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN")
    if not isinstance(producer, Mapping) or not isinstance(producer.get("semantic_contract_id"), str) or producer.get("semantic_contract_id") not in {PRECISION_CONTRACT_ID, PHYSICS_CONTRACT_ID, SOURCE_BLOCKS_CONTRACT_ID, PHYSICS_SOURCE_CONTRACT_ID, PRESSURE_SOURCE_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_ID, LOAD_REFERENCE_CONTRACT_ID, LOAD_REFERENCE_SOURCE_CONTRACT_ID} or producer != {"component_name": "open_pipe_stress_product_physics", "component_version": "0.2.0", "semantic_contract_id": producer.get("semantic_contract_id")}:
        raise ValueError("SOURCE_PRODUCER_CONTRACT_UNSUPPORTED")
    pressure_source = producer["semantic_contract_id"] == PRESSURE_SOURCE_CONTRACT_ID
    composite = producer["semantic_contract_id"] in {PHYSICS_SOURCE_CONTRACT_ID, PRESSURE_SOURCE_CONTRACT_ID}
    if composite:
        from .physics_source import SourceContract
        (SourceContract.PRESSURE_V3 if pressure_source else SourceContract.PHYSICS_V2).check_received(source)
    physics = producer["semantic_contract_id"] == PHYSICS_CONTRACT_ID or composite
    preview = producer["semantic_contract_id"] == PREVIEW_PHYSICS_CONTRACT_ID
    load_reference = producer["semantic_contract_id"] == LOAD_REFERENCE_CONTRACT_ID
    joined = producer["semantic_contract_id"] == LOAD_REFERENCE_SOURCE_CONTRACT_ID
    recovery_forbidden = producer["semantic_contract_id"] not in {SOURCE_BLOCKS_CONTRACT_ID, PHYSICS_SOURCE_CONTRACT_ID, PRESSURE_SOURCE_CONTRACT_ID, LOAD_REFERENCE_SOURCE_CONTRACT_ID} and "source_block_recovery" in source
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
    if not isinstance(formulation, Mapping) or set(formulation) != {"profile_id", "limitations"} or formulation.get("profile_id") != ("exact_pressure_v3" if pressure_source else LOAD_REFERENCE_PROFILE if load_reference else LOAD_REFERENCE_SOURCE_PROFILE if joined else "exact_straight_pressure_v2" if physics else "product_preview_mechanics_v1") or not isinstance(formulation.get("limitations"), list) or not formulation["limitations"] or not all(isinstance(item, str) and item for item in formulation["limitations"]):
        raise ValueError("SOURCE_FORMULATION_BASIS_UNSUPPORTED")
    # F-5 (C1 G6): no raw row outside the successor carries the W1 method token.
    if check_receipt and _has_retained_rows(source):
        raise ValueError(RETAINED_PRECISION_DOWNGRADE_FORBIDDEN)
    if physics and not pressure_source and isinstance(source.get("contract_evidence"), Mapping) and "load_reference_states" in source["contract_evidence"]:
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
    if pressure_source:
        from .physics_source import SourceContract, pressure_source_table, validate_physics_source, validate_transport_metadata
        pressure_source_table()  # Reviewed binding; only the shared TABLE alias is retained.
        if check_receipt: validate_physics_source(source, source_contract=SourceContract.PRESSURE_V3)
        else: validate_transport_metadata(source, source_contract=SourceContract.PRESSURE_V3)
        return PRESSURE_SOURCE_CONTRACT_ID, PRESSURE_SOURCE_CONTRACT_SHA256, _PRESSURE_SOURCE_CONTRACT_PATH
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
    if contract in {SOURCE_BLOCKS_CONTRACT_ID, PHYSICS_SOURCE_CONTRACT_ID, PRESSURE_SOURCE_CONTRACT_ID}:
        if not requested_basis_refs or requested_basis_refs != [case["basis_ref"] for case in source["source_block_recovery"]["body"]["cases"]]:
            return "needs_recompute"
        try:
            if contract in {PHYSICS_SOURCE_CONTRACT_ID, PRESSURE_SOURCE_CONTRACT_ID}:
                from .physics_source import SourceContract, validate_physics_source
                context = SourceContract.PRESSURE_V3 if contract == PRESSURE_SOURCE_CONTRACT_ID else SourceContract.PHYSICS_V2
                qualified = validate_physics_source(source, source_block_context, source_contract=context)
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


