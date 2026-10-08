"""B3's Python reader controls: B3a's 0.3.0 legacy namespace (G8) and B3b's `<physics-retained>` branch.

Until lane P's m3l and m3x successors land, the bases are synthetic receipts derived from PP's pinned milestone
successor (D-U6-5) and resealed by the 07e format rule (preparation hashes, source identities, publication,
receipt). They are reader controls, not producer execution or native Current evidence.
- m3l: the milestone's request with schema 0.3.0 and {"version": "1.0.0", "mode": "legacy_pressure_v1"}, as I99's
  m3l input (RR "I99's B3-W verified; …", ruling 4); its receipt differs only in the invocation digest.
"""
from copy import deepcopy
import hashlib
import json
from pathlib import Path

import pytest

from core.analysis_runs import retained_precision as rp

ROOT = Path(__file__).resolve().parents[1]
PINNED = {"sparse_interactive": "ac6986b0680e0df9d88c33a5bf4635372fc3b83cbb9080da44e6d32dbdca59dc",
          "dense_scrutiny": "6cd1d249e5352aaffbd2b7d7349c74a1d0e0572df35500f66be49c3cad95c9b5"}
MODES = sorted(PINNED)
LEGACY = {"version": "1.0.0", "mode": "legacy_pressure_v1"}
INVOCATION = ("G8", "RETAINED_PRECISION_INVOCATION_MISMATCH")
PREPARATION = ("G8", "RETAINED_PRECISION_PREPARATION_MISMATCH")


def milestone(mode):
    """D-U6-5: PP's pinned milestone successor bytes; returns (source, invocation)."""
    raw = (ROOT / f"fixtures/results/retained_precision_milestone_successor_{mode}.json").read_bytes()
    assert hashlib.sha256(raw).hexdigest() == PINNED[mode]
    doc = json.loads(raw)
    return doc["source"], doc["invocation"]


def reseal(source, invocation):
    """The 07e format rule on a copy: the invocation digest, each complete preparation hash, the selected cases'
    source identities, then publication and receipt."""
    source = deepcopy(source)
    body = source["retained_precision"]["body"]
    body["invocation"]["value"] = rp._hash("source_blocks_invocation_v1", invocation)
    for s in body["sources"]:
        prep = s["preparation"]
        if prep is not None:
            attempt = body["product_attempts"][int(prep["attempt_ref"])]
            if all(m["result"]["kind"] == "prepared" for m in attempt["preparation"]["members"]):
                prep["sha256"] = rp._hash("retained_precision_preparation_v1", rp._preparation_payload(attempt))
    for case in body["cases"]:
        if case["status"] == "selected":
            case["source_identity_sha256"] = rp._source_hash(body["sources"][int(case["source_ref"])])
    body["publication_sha256"] = rp._hash("retained_precision_publication_mp_v2", {k: v for k, v in source.items() if k != "retained_precision"})
    source["retained_precision"]["receipt_sha256"] = rp._hash("retained_precision_receipt_mp_v2", body)
    return source


def outcome(source, invocation, transport=False):
    try:
        result = rp.validate_retained_precision_transport(deepcopy(source)) if transport else rp.validate_retained_precision(deepcopy(source), deepcopy(invocation))
        return ("pass", result["numerical_eligible"], result["standing"])
    except rp.RetainedPrecisionError as error:
        return (error.gate, error.code)


def edited_invocation(invocation, change):
    invocation = deepcopy(invocation)
    change(invocation["request"]["model"])
    return invocation


# ---------------------------------------------------------------------------------------------------------------
# B3a: G8 admits 0.3.0 with exactly the legacy_pressure_v1 contract (B3-D §6.3; REVISION_01 §3, N-4; B3D-10).


def m3l(mode):
    source, invocation = milestone(mode)
    invocation = edited_invocation(invocation, lambda m: m.update(schema_version="0.3.0", pressure_contract=dict(LEGACY)))
    return reseal(source, invocation), invocation


@pytest.mark.parametrize("mode", MODES)
def test_b3a_m3l_successor_is_admitted_like_the_milestone(mode):
    source, invocation = m3l(mode)
    assert outcome(source, invocation) == ("pass", True, "eligible")
    base_source, base_invocation = milestone(mode)
    got, want = rp.validate_retained_precision(source, invocation), rp.validate_retained_precision(base_source, base_invocation)
    assert got == want and got["numerical_eligible"] is True
    # B3a's successor differs from the milestone's only in the invocation digest and the receipt hash (ruling 3).
    assert source["retained_precision"]["body"]["publication_sha256"] == base_source["retained_precision"]["body"]["publication_sha256"]
    assert source["retained_precision"]["body"]["invocation"]["value"] != base_source["retained_precision"]["body"]["invocation"]["value"]


def _element_pressure(magnitude):
    return lambda m: m["load_cases"][0]["primitive_loads"].append(
        {"id": "load:p", "category": "pressure", "dimension": "pressure", "direction": "radial",
         "magnitude": {"unit": "Pa", "value": magnitude}, "target": {"type": "element", "pipe": "M1"}, "provenance": "b3a_reader_control"})


B3A_REFUSALS = {
    "contract mode changed": (lambda m: m["pressure_contract"].update(mode="exact_straight_pressure_v2"), INVOCATION),
    "contract version changed": (lambda m: m["pressure_contract"].update(version="1.0.1"), INVOCATION),
    "contract removed": (lambda m: m.pop("pressure_contract"), INVOCATION),
    "contract null": (lambda m: m.update(pressure_contract=None), INVOCATION),
    "contract with an extra key": (lambda m: m["pressure_contract"].update(extra="x"), INVOCATION),
    "contract the exact one": (lambda m: m.update(pressure_contract={"version": "2.0.0", "mode": "exact_straight_pressure_v2"}), INVOCATION),
    "contract version as a number": (lambda m: m["pressure_contract"].update(version=1.0), INVOCATION),
    "schema 0.2.0 keeping the contract": (lambda m: m.update(schema_version="0.2.0"), INVOCATION),
    "schema 0.4.0 keeping the contract": (lambda m: m.update(schema_version="0.4.0"), INVOCATION),
    "a zero-magnitude element pressure load": (_element_pressure(0.0), PREPARATION),
    "a non-zero element pressure load": (_element_pressure(1000.0), PREPARATION),
    "a non-zero node force with category pressure": (lambda m: m["load_cases"][0]["primitive_loads"].append(
        {"id": "load:p", "category": "pressure", "dimension": "force", "direction": "UX", "magnitude": {"unit": "N", "value": 1.0},
         "target": {"type": "node", "node": "N1"}, "provenance": "b3a_reader_control"}), PREPARATION),
}


@pytest.mark.parametrize("label", sorted(B3A_REFUSALS))
@pytest.mark.parametrize("mode", MODES)
def test_b3a_m3l_mutations(mode, label):
    change, expected = B3A_REFUSALS[label]
    source, invocation = m3l(mode)
    invocation = edited_invocation(invocation, change)
    assert outcome(reseal(source, invocation), invocation) == expected


@pytest.mark.parametrize("value", [{}, False, [], "", 0, LEGACY])
def test_b3a_branch_l_admits_only_an_absent_or_null_contract(value):
    """N-4: on 0.1.0 and 0.2.0 the contract is absent or JSON null, type-strictly; no falsy value stands for null,
    and the legacy contract belongs to 0.3.0 only."""
    source, invocation = milestone("sparse_interactive")
    for version in ("0.1.0", "0.2.0"):
        for contract in (None, "absent"):
            fine = edited_invocation(invocation, lambda m: (m.update(schema_version=version), m.pop("pressure_contract", None) if contract == "absent" else m.update(pressure_contract=None)))
            assert outcome(reseal(source, fine), fine)[0] == "pass"
        bad = edited_invocation(invocation, lambda m: m.update(schema_version=version, pressure_contract=deepcopy(value)))
        assert outcome(reseal(source, bad), bad) == INVOCATION, (version, value)


def test_b3a_tightening_0_3_0_needs_the_legacy_contract():
    """B3D-10, as ruled for all three readers: 0.3.0 without a contract (which the producer cannot emit) is refused."""
    source, invocation = milestone("sparse_interactive")
    bare = edited_invocation(invocation, lambda m: m.update(schema_version="0.3.0"))
    assert "pressure_contract" not in bare["request"]["model"]
    assert outcome(reseal(source, bare), bare) == INVOCATION
