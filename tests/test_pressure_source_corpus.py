"""J2 genuine publications and named negative statements, read unchanged by J4.

Storage expansion is the shared CI1282 DAG helper. Numerical/result verdicts
come solely from this independent test verifier; no producer code or process.
"""
import json
from pathlib import Path

import pytest

from tests.retained_precision_corpus import load_corpus
from tests.verifiers import reader_verdict as rv

ROOT = Path(__file__).resolve().parents[1]
CORPUS = load_corpus(("fixtures/results/pressure_source_reader_corpus.json",))
ENTRIES = CORPUS["entries"]


def test_pressure_source_corpus_has_24_genuine_twins_and_named_mutations():
    assert CORPUS["artifact"] == "pressure-source-reader-corpus"
    assert CORPUS["schema_version"] == "1.0.0"
    assert sum("raw_fixture" in entry for entry in ENTRIES) == 24
    assert len({entry["id"] for entry in ENTRIES}) == len(ENTRIES)
    # B7b: the v2 reader is gone, so MJ5 offers a pressure-source-1 statement to the pressure-1 reader and names the
    # retired v2 identity and captured v2 context refusals.
    for name in ("MJ2-wrong-domain", "MJ4-stale-receipt", "MJ4-publication-binding", "MJ5-pressure-source-offered-to-pressure-1",
                 "MJ5-retired-v2-identity", "MJ5-captured-v2-context", "MJ5-relabelled-shape", "MJ7-downgrade", "MJ3-selected-pressure-"):
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
