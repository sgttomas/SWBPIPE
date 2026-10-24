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

_RESULTS_DIR = Path(__file__).resolve().parents[2] / "fixtures" / "results"
MECHANICS_RESULT_VERSION_RETIRED = "MECHANICS_RESULT_VERSION_RETIRED"
# B4/D1 (b): the retired non-composite source-block identity, refused by name.
RETIRED_SOURCE_BLOCKS_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/source-blocks-1"
SOURCE_BLOCKS_1_RETIRED = "SOURCE_BLOCKS_1_RETIRED"


# B1: precision-1 is no longer read; a stored precision-1 result is refused by name.
RETIRED_PRECISION_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/precision-1"
SOURCE_PRECISION_1_RETIRED = "SOURCE_PRECISION_1_RETIRED"
# Model 0.6.0 (DEL-04-07 Design u4b-u5b-implementation.md §1.2): the ordinary pressure-1 and preview-physics-1
# identities are retired and their tables deleted; a stored result under either is refused by name (re-run), as
# Rust's `current_header` refuses it.
RETIRED_PRESSURE_1_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/pressure-1"
SOURCE_PRESSURE_1_RETIRED = "SOURCE_PRESSURE_1_RETIRED"
RETIRED_PREVIEW_PHYSICS_1_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/preview-physics-1"
SOURCE_PREVIEW_PHYSICS_1_RETIRED = "SOURCE_PREVIEW_PHYSICS_1_RETIRED"
# B7b (A4/A5): the identities of the retired `2.0.0/exact_straight_pressure_v2` contract are refused by name at the
# header, as Rust's `current_header` refuses them (`SOURCE_EXACT_PRESSURE_V2_RETIRED`); none is read.
RETIRED_EXACT_V2_IDS = (
    "openpipestress.result_semantics/0.3.0/physics-1",
    "openpipestress.result_semantics/0.3.0/physics-source-1",
    "openpipestress.result_semantics/0.3.0/load-reference-1",
    "openpipestress.result_semantics/0.3.0/load-reference-source-1",
)
SOURCE_EXACT_PRESSURE_V2_RETIRED = "SOURCE_EXACT_PRESSURE_V2_RETIRED"
SOURCE_EXACT_PRESSURE_V2_RETIRED_TEXT = (f"{SOURCE_EXACT_PRESSURE_V2_RETIRED}: 2.0.0/exact_straight_pressure_v2 results are no longer read; "
                                         "re-author the model as 0.7.0 with the current exact pressure contract and re-run the analysis")
PRESSURE_SOURCE_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/pressure-source-1"
# T4-U6 (D-5, §2.4): the per-basis stress signatures.
PRESSURE_SOURCE_CONTRACT_SHA256 = "b5a4b0e585b9ec440d36144860baaf101b628498cfdab9a1de9248c2ed5069e7"
_PRESSURE_SOURCE_CONTRACT_PATH = _RESULTS_DIR / "semantic_contract_v0_3_pressure_source_1.json"
# pressure-2 (`3.0.0/exact_pressure_v3`): the ordinary exact route. Its raw statements are read by the independent
# `pressure_2` verifier; it is also the exact successor's G7 base (B7b D4 (a)).
PRESSURE_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/pressure-2"
PRESSURE_CONTRACT_SHA256 = "9183ef9b83f879620df45c03994f71808cc9c04c4a093c86b0bedfca2f96e59f"
_PRESSURE_CONTRACT_PATH = _RESULTS_DIR / "semantic_contract_v0_3_pressure_2.json"
PRESSURE_PROFILE = "exact_pressure_v3"
PREVIEW_PHYSICS_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/preview-physics-2"
PREVIEW_PHYSICS_CONTRACT_SHA256 = "ee8036d59d0aeb255ff132e4dee0d6e7a1e8da8b33e4697c7761c695c3f6e387"
_PREVIEW_PHYSICS_CONTRACT_PATH = _RESULTS_DIR / "semantic_contract_v0_3_preview_physics_2.json"
# U6b (D-U6-6; D2 4.7 S-1, 4.9.6): the F2a preview successor. Its statements are
# checked only by the accepted reader (`retained_precision`), and its standing
# comes only from that reader's verified receipt (D1 5 item 3).
PREVIEW_PHYSICS_RETAINED_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/preview-physics-retained-1"
PREVIEW_PHYSICS_RETAINED_CONTRACT_SHA256 = "4da08f964605a5080b38c0ea9793731b33720070db001a4c7f1c4f9fbcaeb76f"
_PREVIEW_PHYSICS_RETAINED_CONTRACT_PATH = _RESULTS_DIR / "semantic_contract_v0_3_preview_physics_retained_1.json"
# B3b (B3-D §2, §6.1): the exact successor, whose base is pressure-2 since B7b D4 (a). As the preview successor, its
# statements are checked only by the accepted reader, which dispatches on this identity, and its standing comes only
# from that reader.
PHYSICS_RETAINED_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/physics-retained-1"
PHYSICS_RETAINED_CONTRACT_SHA256 = "a0dac6244cfbb5b930e439d9d64a6eb45001d95271af6e8eb90939c7b3d323dd"
_PHYSICS_RETAINED_CONTRACT_PATH = _RESULTS_DIR / "semantic_contract_v0_3_physics_retained_1.json"
RETAINED_CONTRACTS = {PREVIEW_PHYSICS_RETAINED_CONTRACT_ID: (PREVIEW_PHYSICS_RETAINED_CONTRACT_SHA256, _PREVIEW_PHYSICS_RETAINED_CONTRACT_PATH),
                      PHYSICS_RETAINED_CONTRACT_ID: (PHYSICS_RETAINED_CONTRACT_SHA256, _PHYSICS_RETAINED_CONTRACT_PATH)}
RETAINED_METHOD = "contribution_preserving_multiprecision_v1"
RETAINED_PRECISION_DOWNGRADE_FORBIDDEN = "RETAINED_PRECISION_DOWNGRADE_FORBIDDEN"
RULE_QUANTITY_BELOW_VERIFIED_FLOOR = "RULE_QUANTITY_BELOW_VERIFIED_FLOOR"
RULE_QUANTITY_NOT_COVERED = "RULE_QUANTITY_NOT_COVERED"
# pressure-2 has no transport (metadata-only) reading outside the exact successor's G7 step (Rust `reader_verdict`).
PRESSURE_TRANSPORT_UNSUPPORTED = "SOURCE_PRESSURE_TRANSPORT_UNSUPPORTED"
# Static fresh-publication identities (S1 §10; Rust `FRESH_IDENTITIES`); no route predicate.
FRESH_CONTRACT_IDS = frozenset({PREVIEW_PHYSICS_CONTRACT_ID, PRESSURE_SOURCE_CONTRACT_ID,
                                # T4-U2: pressure-2 (`3.0.0/exact_pressure_v3`).
                                PRESSURE_CONTRACT_ID,
                                # U6b (D-U6-6): membership is not standing.
                                PREVIEW_PHYSICS_RETAINED_CONTRACT_ID})


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
    `carrier_evidence` member is left to the base transport metadata check (G7), and `source_block_recovery` is
    checked before `contract_evidence`. On that step a pressure-2 projection (the exact successor's base since
    B7b D4 (a)) takes the exact-retained transport check (`physics_evidence.validate_transport_metadata`), as
    Rust's retained transport does; pressure-2 has no other transport reading (Rust's `transport("pressure")`
    refuses it). Every other caller passes nothing and reads exactly as before."""
    if _is_retained(source):
        return _retained_contract(source, check_receipt=check_receipt)
    # F-5: no other identity may carry a receipt member.
    if isinstance(source, Mapping) and "retained_precision" in source:
        raise ValueError(RETAINED_PRECISION_DOWNGRADE_FORBIDDEN)
    version = source.get("schema_version")
    # A source-block receipt belongs only to its explicit method branch below;
    # the unrelated carrier namespace remains unsupported for every profile. On the exact identities (pressure-2,
    # pressure-source-1) Rust's header (`current_header`) has one carrier check, restored in B7b as the retired
    # load-reference routes' guard: after the identity allowlist, a statement whose `contract_evidence` carries the
    # load/reference-state group (`load_reference_states`) and a `carrier_evidence` member is refused
    # SOURCE_PRODUCER_CONTRACT_UNSUPPORTED (mirrored below, in both header orders). Any other carrier on those
    # identities passes the header; their readers refuse the foreign namespace themselves, as here.
    named = source.get("producer").get("semantic_contract_id") if isinstance(source.get("producer"), Mapping) else None
    if "carrier_evidence" in source and not rust_header_order and named not in {PRESSURE_CONTRACT_ID, PRESSURE_SOURCE_CONTRACT_ID}:
        raise ValueError("SOURCE_PRODUCER_CONTRACT_UNSUPPORTED")
    # b7-plan E2: the producer-less 0.1.0 carrier is retired and refused by name.
    if version == "0.1.0":
        raise ValueError(MECHANICS_RESULT_VERSION_RETIRED)
    if version != "0.2.0":
        raise ValueError("SOURCE_SCHEMA_VERSION_UNSUPPORTED")
    producer = source.get("producer")
    # The joined receipt cannot be downgraded to ordinary pressure-2, including
    # transport statements. This is a source-qualified exception only.
    if producer == {"component_name":"open_pipe_stress_product_physics", "component_version":"0.2.0", "semantic_contract_id":PRESSURE_CONTRACT_ID} and "source_block_recovery" in source:
        raise ValueError("SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN")
    if producer == {"component_name": "open_pipe_stress_product_physics", "component_version": "0.2.0", "semantic_contract_id": RETIRED_PRECISION_CONTRACT_ID}:
        raise ValueError(f"{SOURCE_PRECISION_1_RETIRED}: precision-1 results are no longer read; re-run the analysis")
    if producer == {"component_name": "open_pipe_stress_product_physics", "component_version": "0.2.0", "semantic_contract_id": RETIRED_PRESSURE_1_CONTRACT_ID}:
        raise ValueError(f"{SOURCE_PRESSURE_1_RETIRED}: pressure-1 results are no longer read; re-run the analysis")
    if producer == {"component_name": "open_pipe_stress_product_physics", "component_version": "0.2.0", "semantic_contract_id": RETIRED_PREVIEW_PHYSICS_1_CONTRACT_ID}:
        raise ValueError(f"{SOURCE_PREVIEW_PHYSICS_1_RETIRED}: preview-physics-1 results are no longer read; re-run the analysis")
    if producer == {"component_name": "open_pipe_stress_product_physics", "component_version": "0.2.0", "semantic_contract_id": RETIRED_SOURCE_BLOCKS_CONTRACT_ID}:
        raise ValueError(f"{SOURCE_BLOCKS_1_RETIRED}: source-blocks-1 results are no longer read; re-run the analysis")
    # B7b (A4/A5): a stored v2 result is refused by name, never read or reinterpreted.
    if isinstance(producer, Mapping) and producer.get("semantic_contract_id") in RETIRED_EXACT_V2_IDS and producer == {"component_name": "open_pipe_stress_product_physics", "component_version": "0.2.0", "semantic_contract_id": producer.get("semantic_contract_id")}:
        raise ValueError(SOURCE_EXACT_PRESSURE_V2_RETIRED_TEXT)
    if not isinstance(producer, Mapping) or not isinstance(producer.get("semantic_contract_id"), str) or producer.get("semantic_contract_id") not in {PRESSURE_CONTRACT_ID, PRESSURE_SOURCE_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_ID} or producer != {"component_name": "open_pipe_stress_product_physics", "component_version": "0.2.0", "semantic_contract_id": producer.get("semantic_contract_id")}:
        raise ValueError("SOURCE_PRODUCER_CONTRACT_UNSUPPORTED")
    # Rust `current_header`, after the identity allowlist and before the recovery downgrade check: the
    # load/reference-state group owns no carrier namespace (the retired load-reference routes' guard, kept on
    # the current group), in both header orders.
    if isinstance(source.get("contract_evidence"), Mapping) and "load_reference_states" in source["contract_evidence"] and "carrier_evidence" in source:
        raise ValueError("SOURCE_PRODUCER_CONTRACT_UNSUPPORTED")
    pressure_source = producer["semantic_contract_id"] == PRESSURE_SOURCE_CONTRACT_ID
    pressure = producer["semantic_contract_id"] == PRESSURE_CONTRACT_ID
    preview = producer["semantic_contract_id"] == PREVIEW_PHYSICS_CONTRACT_ID
    if pressure_source:
        from .physics_source import SourceContract
        SourceContract.PRESSURE_V3.check_received(source)
    recovery_forbidden = not pressure_source and "source_block_recovery" in source
    if rust_header_order and recovery_forbidden:
        # Ruling 1: Rust's order, the recovery member before the evidence demand.
        raise ValueError("SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN")
    if preview and not isinstance(source.get("contract_evidence"), Mapping):
        raise ValueError("SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED")
    if recovery_forbidden:
        raise ValueError("SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN")
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
    if not isinstance(formulation, Mapping) or set(formulation) != {"profile_id", "limitations"} or formulation.get("profile_id") != (PRESSURE_PROFILE if pressure_source or pressure else "product_preview_mechanics_v1") or not isinstance(formulation.get("limitations"), list) or not formulation["limitations"] or not all(isinstance(item, str) and item for item in formulation["limitations"]):
        raise ValueError("SOURCE_FORMULATION_BASIS_UNSUPPORTED")
    # F-5 (C1 G6): no raw row outside the successor carries the W1 method token.
    if check_receipt and _has_retained_rows(source):
        raise ValueError(RETAINED_PRECISION_DOWNGRADE_FORBIDDEN)
    if pressure_source:
        from .physics_source import SourceContract, pressure_source_table, validate_physics_source, validate_transport_metadata
        pressure_source_table()  # The reviewed, independently resident current source table.
        if check_receipt: validate_physics_source(source, source_contract=SourceContract.PRESSURE_V3)
        else: validate_transport_metadata(source, source_contract=SourceContract.PRESSURE_V3)
        return PRESSURE_SOURCE_CONTRACT_ID, PRESSURE_SOURCE_CONTRACT_SHA256, _PRESSURE_SOURCE_CONTRACT_PATH
    if pressure:
        from .pressure_2 import pressure_table, validate_pressure_evidence
        pressure_table()  # The reviewed ordinary pressure table.
        if check_receipt:
            validate_pressure_evidence(source)
        elif rust_header_order:
            # The exact successor's G7 transport step on its pressure-2 projection (B7b D4 (a)).
            from .physics_evidence import validate_transport_metadata as exact_retained_transport
            exact_retained_transport(source)
        else:
            raise ValueError(PRESSURE_TRANSPORT_UNSUPPORTED)
        return PRESSURE_CONTRACT_ID, PRESSURE_CONTRACT_SHA256, _PRESSURE_CONTRACT_PATH
    if preview:
        from .preview_physics_evidence import validate_preview_physics_evidence, validate_transport_metadata as validate_preview_transport_metadata
        (validate_preview_physics_evidence if check_receipt else validate_preview_transport_metadata)(source)
        return PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_SHA256, _PREVIEW_PHYSICS_CONTRACT_PATH
    raise ValueError("SOURCE_PRODUCER_CONTRACT_UNSUPPORTED")


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
    if contract == PRESSURE_SOURCE_CONTRACT_ID:
        if not requested_basis_refs or requested_basis_refs != [case["basis_ref"] for case in source["source_block_recovery"]["body"]["cases"]]:
            return "needs_recompute"
        try:
            from .physics_source import SourceContract, validate_physics_source
            # A load/reference-state reading validates but is never eligible (T1; join standing ruling).
            qualified = validate_physics_source(source, source_block_context, source_contract=SourceContract.PRESSURE_V3)
            return "numerically_eligible" if qualified else "needs_recompute"
        except ValueError:
            return "unsupported"
    if contract not in {PREVIEW_PHYSICS_CONTRACT_ID, PRESSURE_CONTRACT_ID}:
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
    # B1 and B4/D1 (b) refuse precision-1 and source-blocks-1 by name, so no
    # readable source has a standing reason.
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

    A successor row is refused by its validated class. B4/D1 (b) retired the
    non-composite source-blocks-1 summary refusal with that identity.
    W-15 slice d: a component body's row (``component_body``) is refused as
    ``COMPONENT_BODY_REFERENCE_SECTION``.
    """
    if isinstance(row, Mapping) and "component_body" in row:
        return "COMPONENT_BODY_REFERENCE_SECTION"
    if _is_retained(envelope):
        return _retained_binding_refusal(envelope, row)
    return None


