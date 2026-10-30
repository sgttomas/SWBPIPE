"""J2 genuine publications and named negative statements, read unchanged by J4.

Storage expansion is the shared CI1282 DAG helper. Numerical/result verdicts
come solely from this independent test verifier; no producer code or process.
"""
import json
from pathlib import Path

import pytest

from tests.retained_precision_corpus import load_corpus
from tests.verifiers import physics_source as ps
from tests.verifiers import reader_verdict as rv
from tests.verifiers import source_blocks as blocks

ROOT = Path(__file__).resolve().parents[1]
CORPUS = load_corpus(("fixtures/results/pressure_source_reader_corpus.json",))
ENTRIES = CORPUS["entries"]


def test_pressure_source_corpus_has_24_genuine_twins_and_named_mutations():
    assert CORPUS["artifact"] == "pressure-source-reader-corpus"
    assert CORPUS["schema_version"] == "1.0.0"
    assert sum("raw_fixture" in entry for entry in ENTRIES) == 24
    assert len({entry["id"] for entry in ENTRIES}) == len(ENTRIES)
    # B7b: the v2 reader is gone, so MJ5 offers a pressure-source-1 statement to the pressure-2 reader and names the
    # retired v2 identity and captured v2 context refusals.
    for name in ("MJ2-wrong-domain", "MJ4-stale-receipt", "MJ4-publication-binding", "MJ5-pressure-source-offered-to-pressure-1",
                 "MJ5-retired-v2-identity", "MJ5-captured-v2-context", "MJ5-relabelled-shape", "MJ7-downgrade", "MJ3-selected-pressure-",
                 "RF-free-block-5-dofs", "RF-unknown-source-level", "RF-unsupported-block-order-3", "RF-unsupported-block-order-4"):
        assert any(name in entry["id"] for entry in ENTRIES), name


@pytest.mark.parametrize("entry", ENTRIES, ids=lambda entry: entry["id"])
def test_pressure_source_corpus_independent_verifier_matches_native(entry):
    source = entry["source"]
    if "raw_fixture" in entry:
        # Exact current producer serialization is also an unchanged consuming input.
        received = json.loads((ROOT / entry["raw_fixture"]).read_bytes())
        assert received == source
        source = received
    invocation = entry["invocation"]
    model = invocation["request"]["model"]
    actual = {
        "validation": rv.validate(entry["reader"], source, model, invocation),
        "unbound": rv.validate(entry["reader"], source),
        "transport": rv.transport(entry["reader"], entry["transport_source"]),
        "dispatch": rv.dispatch(source, model, invocation),
    }
    assert actual == entry["expected"]


def test_rf16_stored_unsupported_block_orders_3_and_4_are_refused_by_the_verifiers_order_bound():
    """RF source-method extension §7/§8 RF-16, in the independent verifier. The receipt schema admits only qualified
    cases, so J2's stored unsupported_block records are refused at the receipt shape before FAILURE_BLOCK_ORDER at
    every reader entry (the parametrized read above). The order bound is therefore checked here, directly on those
    records: orders 3 and 4 (which the current producer solves) are refused; order 5 without a source is admitted."""
    stored = [(entry, case) for entry in ENTRIES if ":RF-unsupported-block-order-" in entry["id"]
              for case in entry["source"]["source_block_recovery"]["body"]["cases"] if case["failure"] is not None]
    assert len(stored) == 8 and sorted({case["failure"]["block_order"] for _, case in stored}) == [3, 4]
    for entry, case in stored:
        assert entry["expected"]["validation"]["error"]["code"] == "PHYSICS_SOURCE_RECEIPT_SHAPE", entry["id"]
        assert case["failure"]["code"] == "unsupported_block" and case["source"] is None, entry["id"]
        assert not blocks._failure_block_order(case["failure"], case), entry["id"]
        for order, admitted in ((0, False), (2, False), (5, True), (89, True), (None, False)):
            assert blocks._failure_block_order({**case["failure"], "block_order": order}, case) is admitted, (entry["id"], order)
        assert not blocks._failure_block_order({**case["failure"], "block_order": 5}, {**case, "source": {}}), entry["id"]
        for code in ("unsupported_source_closure", "budget"):
            assert blocks._failure_block_order({**case["failure"], "code": code, "block_order": None}, case), (entry["id"], code)
            assert not blocks._failure_block_order({**case["failure"], "code": code}, case), (entry["id"], code)


def test_rf_block_length_and_level_are_the_receipt_schemas():
    """RF §7: block length and level are checked through the schema. No receipt is fabricated for a 4-DOF block or
    the congruence level: those read as valid only on a genuine producer output."""
    schema = ps._schema()
    source = schema["$defs"]["source"]["properties"]
    assert all(blocks._shape([list(range(n))], source["free_blocks"], schema) for n in (1, 2, 3, 4))
    assert not blocks._shape([[0, 1, 2, 3, 4]], source["free_blocks"], schema)
    assert not blocks._shape([[]], source["free_blocks"], schema)
    for level in ("complete_identified_represented_contributions", "complete_identified_declared_member_congruence"):
        assert blocks._shape(level, source["level"], schema), level
    for level in ("complete_identified_unknown_member_basis", "", None):
        assert not blocks._shape(level, source["level"], schema), level
