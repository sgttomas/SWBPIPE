"""Closed admission of the joined load-reference-source-1 evidence.

Mirror of ``core/reporting/result_export/src/load_reference_source.rs``: the
same checks run in the same order and raise the same error strings. Change both
together. Contract ``openpipestress.result_semantics/0.3.0/load-reference-source-1``,
profile ``resolved_straight_load_state_source_v1``, receipt policy
``LOAD-REFERENCE-SOURCE-1`` (CP2_WIRE_ADDENDUM_2 section 5, CP4 corrections).

1. The load-reference pre-pass (S1-S13) in its joined form: a selected case
   publishes ``retained_source_blocks_exact_v1`` and carries
   ``SOURCE_BLOCK_RECOVERY_SELECTED`` instead of NOT_JOINED.
2. The receipt on the received bytes: policy, closed shape, receipt hash,
   publication hash (raw only), case order, per-case method and requested mode,
   and the per-case physical-evidence hash over domain
   ``load_reference_source_case_evidence_v1`` of
   ``{exact_case, pressure, load_reference_state}``, as the producer hashes it.
3. The unchanged physics-source-1 validator on a projected copy. The projection
   removes or neutralizes only what steps 1-2 bound, then re-derives the
   physics-source-1 hashes of the projected bytes; every received hash was
   verified first.

Numerical eligibility is never granted: a reader cannot re-derive a 0.4.0
resolved case from a captured request, so a valid envelope is
``needs_recompute``. This establishes internal source consistency, never
producer origin, solver accuracy or model freshness.
"""
from __future__ import annotations

from collections.abc import Mapping
from copy import deepcopy
from functools import lru_cache
import hashlib
import json
from pathlib import Path
from typing import Any

from . import load_reference_evidence as lr

CONTRACT_ID = "openpipestress.result_semantics/0.3.0/load-reference-source-1"
PROFILE = "resolved_straight_load_state_source_v1"
TABLE_SHA256 = "d1628194a7730f427843b00228dd233cf92b8e7d26f3bc31c660a3ea59e28337"
TABLE_PATH = Path(__file__).resolve().parents[2] / "fixtures" / "results" / "semantic_contract_v0_3_load_reference_source_1.json"
POLICY = "LOAD-REFERENCE-SOURCE-1"
CASE_EVIDENCE_DOMAIN = "load_reference_source_case_evidence_v1"
PHYSICS_SOURCE_POLICY = "PHYSICS-SOURCE-1"
PHYSICS_SOURCE_CASE_DOMAIN = "physics_source_case_evidence_v1"

LoadReferenceSourceError = lr.LoadReferenceError


def _require(ok: bool, name: str) -> None:
    lr._require(ok, name)


def verify_table(data: bytes) -> dict[str, Any]:
    """Pinned table bytes: identity, profile, policy and sha256 are checked, never inferred."""
    if hashlib.sha256(data).hexdigest() != TABLE_SHA256:
        raise LoadReferenceSourceError("SOURCE_LOAD_REFERENCE_SOURCE_TABLE_HASH")
    table = json.loads(data)
    if not isinstance(table, Mapping) or table.get("semantic_contract_id") != CONTRACT_ID or table.get("formulation_profile_id") != PROFILE or table.get("source_block_policy") != POLICY:
        raise LoadReferenceSourceError("SOURCE_LOAD_REFERENCE_SOURCE_TABLE_IDENTITY")
    return table


@lru_cache(maxsize=1)
def load_reference_source_table() -> dict[str, Any]:
    return verify_table(TABLE_PATH.read_bytes())


def validate_load_reference_source_evidence(source: Mapping[str, Any]) -> bool:
    """Raw joined publication. Returns False: admitted, never numerically eligible."""
    _guarded(source, raw=True)
    return False


def validate_load_reference_source_transport_metadata(source: Mapping[str, Any]) -> None:
    """Retained joined statements without raw rows: pre-pass, retained receipt, physics-source-1 transport."""
    _guarded(source, raw=False)


def _guarded(source: Mapping[str, Any], *, raw: bool) -> None:
    try:
        _validate(source, raw)
    except (TypeError, KeyError, AttributeError, OverflowError, IndexError) as error:
        raise LoadReferenceSourceError("SOURCE_LOAD_REFERENCE_MALFORMED") from error


def _validate(source: Mapping[str, Any], raw: bool) -> None:
    # J0 the joined identity and profile. The projection below replaces both,
    # so the direct validator checks them itself.
    _require(lr._eq(lr._get(lr._get(source, "producer"), "semantic_contract_id"), CONTRACT_ID) and lr._eq(lr._get(lr._get(source, "formulation_basis"), "profile_id"), PROFILE), "JOIN_IDENTITY")
    # J1 joined pre-pass (S1-S13).
    lr._prepass(source, raw, lr.JOINED)
    # J2 receipt on the received bytes.
    _receipt(source, raw)
    # J3 physics-source-1 on the projected copy.
    projected = _project(source, raw)
    from .physics_source import validate_physics_source, validate_transport_metadata
    try:
        if raw:
            validate_physics_source(projected)
        else:
            validate_transport_metadata(projected)
    except ValueError as error:
        raise LoadReferenceSourceError(f"{lr._code('JOIN_PHYSICS_SOURCE')}: {error}") from error


def _case_pressure(pressure: list[Any], case_id: Any) -> list[Any]:
    return [p for p in pressure if isinstance(p, Mapping) and p.get("load_case_id") == case_id]


def _receipt(source: Mapping[str, Any], raw: bool) -> None:
    from .physics_source import validate_receipt_shape
    from .source_blocks import domain_hash
    receipt = source["source_block_recovery"]
    body = lr._get(receipt, "body")
    # R1 policy, then R2 the closed physics-source-1 receipt shape with only the
    # policy constant substituted.
    _require(lr._eq(lr._get(body, "policy"), POLICY), "JOIN_RECEIPT_POLICY")
    shaped = deepcopy(receipt)
    shaped["body"]["policy"] = PHYSICS_SOURCE_POLICY
    try:
        validate_receipt_shape(shaped)
    except ValueError:
        raise lr._fail("JOIN_RECEIPT_SHAPE") from None
    # R3-R4 receipt and publication hashes.
    _require(receipt["receipt_sha256"] == domain_hash("source_blocks_receipt_v1", body), "JOIN_RECEIPT_HASH")
    if raw:
        publication = {key: value for key, value in source.items() if key != "source_block_recovery"}
        _require(body["publication_sha256"] == domain_hash("source_blocks_publication_v1", publication), "JOIN_PUBLICATION_HASH")
    # R5 one receipt case, exact case and record per case, in case order.
    evidence = source["contract_evidence"]
    cases, exact, records, pressure = body["cases"], evidence["exact_cases"], evidence["load_reference_states"], evidence["pressure"]
    exact_ids = [case["load_case_id"] for case in exact]
    _require([case["basis_ref"]["ref_id"] for case in cases] == exact_ids and [record["load_case_id"] for record in records] == exact_ids, "JOIN_CASE_ORDER")
    # R6-R7 per case: selected method, requested mode and physical-evidence hash.
    for case, exact_case, record in zip(cases, exact, records):
        _require(case["selected_method"] == exact_case["recovery_method"], "JOIN_RECOVERY_METHOD")
        _require(case["requested_mode"] == record["solve"]["requested_mode"], "JOIN_REQUESTED_MODE")
        proof = {"exact_case": exact_case, "pressure": _case_pressure(pressure, exact_case["load_case_id"]), "load_reference_state": record}
        _require(case["physical_evidence_sha256"] == domain_hash(CASE_EVIDENCE_DOMAIN, proof), "JOIN_PHYSICAL_CASE_HASH")


def _project(source: Mapping[str, Any], raw: bool) -> dict[str, Any]:
    """The physics-source-1 form of verified joined bytes (after steps 1-2)."""
    from .physics_source import CONTRACT_ID as PHYSICS_SOURCE_ID, PROFILE as PHYSICS_SOURCE_PROFILE
    from .source_blocks import domain_hash
    projected = lr._project(source)
    projected["producer"]["semantic_contract_id"] = PHYSICS_SOURCE_ID
    projected["formulation_basis"]["profile_id"] = PHYSICS_SOURCE_PROFILE
    evidence = projected["contract_evidence"]
    body = projected["source_block_recovery"]["body"]
    body["policy"] = PHYSICS_SOURCE_POLICY
    for case, exact_case in zip(body["cases"], evidence["exact_cases"]):
        case["physical_evidence_sha256"] = domain_hash(PHYSICS_SOURCE_CASE_DOMAIN, {"exact_case": exact_case, "pressure": _case_pressure(evidence["pressure"], exact_case["load_case_id"])})
    if raw:
        body["publication_sha256"] = domain_hash("source_blocks_publication_v1", {key: value for key, value in projected.items() if key != "source_block_recovery"})
    projected["source_block_recovery"]["receipt_sha256"] = domain_hash("source_blocks_receipt_v1", body)
    return projected
