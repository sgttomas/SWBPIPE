"""the Python mirror of
the declared-difference readers over the shared corpus (``fixtures/results/preview_physics_difference_basis_corpus.json``,
written by PP ``op1b_difference_basis.rs``). Every producer document reads with its model and without it (option A,
re-derived from the published evidence); each forgery is refused with the Rust reader's code
(RE ``preview_difference_basis_tests.rs``)."""
from __future__ import annotations

import copy
import json
import struct
from pathlib import Path

import pytest

from tests.verifiers import preview_difference_basis as difference
from tests.verifiers import reader_verdict

PROJECT = Path(__file__).resolve().parents[1]
CORPUS = json.loads((PROJECT / "fixtures/results/preview_physics_difference_basis_corpus.json").read_text(encoding="utf-8"))
NAMES = [e["name"] for e in CORPUS["documents"]]
MEMBER = difference.MEMBER


def entry(name: str) -> tuple[dict, dict]:
    found = next(e for e in CORPUS["documents"] if e["name"] == name)
    return copy.deepcopy(found["document"]), copy.deepcopy(found["model"])


def code(outcome: dict) -> str:
    return "ok" if outcome["ok"] else outcome["error"]["code"]


def read(source: dict, model: dict | None = None) -> str:
    return code(reader_verdict.validate("preview-physics", source, model))


def bits(value: float) -> str:
    return f"{struct.unpack('>Q', struct.pack('>d', value))[0]:016x}"


def test_the_switch_is_gone():
    assert not hasattr(difference, "MODEL_FREE")
    assert len(NAMES) >= 9


@pytest.mark.parametrize("name", NAMES)
def test_producer_documents_read_with_their_model_and_without_it(name):
    source, model = entry(name)
    assert read(source, model) == "ok"
    assert read(source) == "ok"


def _sub(v):
    return v["contract_evidence"][MEMBER][0]["subtrahend"]["primitive_contents"]


def _gate(v, cid, value):
    for g in v["contract_evidence"]["combination_gates"]:
        if g["combination_id"] == cid:
            g.clear()
            g.update(value)


def _bump(v):
    e = _sub(v)[0]["magnitude"]
    e["value_bits"] = f"{int(e['value_bits'], 16) + 1:016x}"


def _swap(v):
    r = v["contract_evidence"][MEMBER][0]
    r["minuend"], r["subtrahend"] = r["subtrahend"], r["minuend"]


def _plain_record(v):
    extra = copy.deepcopy(v["contract_evidence"][MEMBER][0])
    extra["combination_id"] = "comb:plain"
    v["contract_evidence"][MEMBER].append(extra)


def _equal_bases(v):
    records = [r for r in v["results"] if r["kind"] == difference.MODULUS_RECORD and r["basis_ref"]["ref_id"] == "comb:o-s"]
    for r in records:
        r["metadata"]["basis"] = records[1]["metadata"]["basis"]


TAMPERS = [
    ("absent member", "v1_closing", difference.EVIDENCE, lambda v: v["contract_evidence"].pop(MEMBER)),
    ("empty member", "v1_closing", difference.EVIDENCE, lambda v: v["contract_evidence"].__setitem__(MEMBER, [])),
    ("missing operand", "v1_closing", difference.EVIDENCE, lambda v: v["contract_evidence"][MEMBER][0].pop("subtrahend")),
    ("another token", "v1_closing", difference.EVIDENCE, lambda v: v["contract_evidence"][MEMBER][0].__setitem__("difference_basis", "sustained")),
    ("evidence for a plain subtraction", "v1_closing", difference.ROW, _plain_record),
    ("unknown combination", "v1_closing", difference.BINDING, lambda v: v["contract_evidence"][MEMBER][0].__setitem__("combination_id", "comb:none")),
    ("unknown case", "v1_closing", difference.BINDING, lambda v: v["contract_evidence"][MEMBER][0]["subtrahend"].__setitem__("load_case_id", "case:none")),
    ("cases swapped", "v1_closing", difference.BINDING, _swap),
    ("a zero count", "v1_closing", difference.EVIDENCE, lambda v: _sub(v)[0].__setitem__("count", 0)),
    ("a float count", "v1_closing", difference.EVIDENCE, lambda v: _sub(v)[0].__setitem__("count", 1.0)),
    ("a boolean count", "v1_closing", difference.EVIDENCE, lambda v: _sub(v)[0].__setitem__("count", True)),
    ("a duplicated entry", "v1_closing", difference.EVIDENCE, lambda v: _sub(v).append(copy.deepcopy(_sub(v)[0]))),
    ("a non-finite value", "v1_closing", difference.EVIDENCE, lambda v: _sub(v)[0]["magnitude"].__setitem__("value_bits", bits(float("inf")))),
    ("upper-case bits", "v1_closing", difference.EVIDENCE, lambda v: _sub(v)[0]["magnitude"].__setitem__("value_bits", bits(0.1).upper())),
    ("one value bit", "v1_closing", difference.GATE_DERIVATION, _bump),
    ("a unit", "v1_closing", difference.GATE_DERIVATION, lambda v: _sub(v)[0]["magnitude"].__setitem__("unit", "kN")),
    ("a target", "v1_closing", difference.GATE_DERIVATION, lambda v: _sub(v)[0]["target"].__setitem__("node", "node:a")),
    ("a direction", "v1_closing", difference.GATE_DERIVATION, lambda v: _sub(v)[0].__setitem__("direction", "global_y")),
    ("a multiplicity", "v1_closing", difference.GATE_DERIVATION, lambda v: _sub(v)[0].__setitem__("count", 2)),
    ("G-1 restated as G-2", "v5_sustained_only_load", difference.GATE_DERIVATION,
     lambda v: _gate(v, "comb:o-s", {"combination_id": "comb:o-s", "withheld": True, "reason": difference.MODULUS_MIXED})),
    ("withheld evidence made contained", "v5_sustained_only_load", difference.GATE_DERIVATION,
     lambda v: v["contract_evidence"][MEMBER][0]["subtrahend"].__setitem__("primitive_contents", [e for e in _sub(v) if e["category"] != "weight"])),
    ("G-2's bases made equal", "v5_g2", difference.GATE_DERIVATION, _equal_bases),
    ("a G-1 reason without its record", "v5_sustained_only_load", difference.EVIDENCE, lambda v: v["contract_evidence"].pop(MEMBER)),
]


@pytest.mark.parametrize("what,name,expected,edit", TAMPERS, ids=[t[0] for t in TAMPERS])
def test_the_model_free_reading_refuses_tampered_evidence_by_name(what, name, expected, edit):
    source, _ = entry(name)
    edit(source)
    assert read(source) == expected


def test_a_coherent_forgery_is_not_authenticated_without_the_model():
    source, model = entry("v1_closing")
    source["contract_evidence"][MEMBER][0]["subtrahend"]["primitive_contents"] = []
    assert read(source) == "ok"
    assert read(source, model) == difference.MODEL


def test_the_model_bound_reader_refuses_evidence_unlike_the_model_and_condition_use():
    source, model = entry("v1_closing")
    _bump(source)
    assert read(source, model) == difference.MODEL
    source, model = entry("v1_closing")
    model["load_cases"][0]["condition_use"] = {"temperature": "OPE1"}
    assert read(source, model) == difference.UNVERIFIED


# ------------------------------------------------------------------ review r1
MAX_COUNT = 2**53 - 1


def _declare_plain_twin(v):
    twin = copy.deepcopy(v["contract_evidence"][MEMBER][0])
    twin["combination_id"] = "comb:plain"
    v["contract_evidence"][MEMBER].append(twin)
    for row in v["results"]:
        if (row.get("basis_ref") or {}).get("ref_id") == "comb:plain" and row["kind"] != difference.MODULUS_RECORD \
                and isinstance((row.get("metadata") or {}).get("sign_convention"), str):
            row["metadata"]["sign_convention"] = difference.SIGN


def _model_declaring_plain_twin(model):
    model = copy.deepcopy(model)
    for c in model["combinations"]:
        if c["id"] == "comb:plain":
            c["difference_basis"] = "operating_minus_sustained"
    return model


def test_a_case_recurring_across_records_carries_identical_contents():
    """Review r1 P1: one producer resolution gives one multiset per case, in both readings."""
    source, model = entry("v1_closing")
    model = _model_declaring_plain_twin(model)
    _declare_plain_twin(source)
    assert read(source) == "ok"
    assert read(source, model) == "ok"
    contradictory = copy.deepcopy(source)
    contradictory["contract_evidence"][MEMBER][1]["subtrahend"]["primitive_contents"] = []
    assert read(contradictory) == difference.BINDING
    assert read(contradictory, model) == difference.BINDING
    minuend = copy.deepcopy(source)
    minuend["contract_evidence"][MEMBER][1]["minuend"]["primitive_contents"][0]["count"] = 2
    assert read(minuend) == difference.BINDING
    assert read(minuend, model) == difference.BINDING


BAD_COUNTS = [("zero", 0), ("negative", -1), ("max + 1", MAX_COUNT + 1), ("2^64", 2**64), ("a float", 1.0), ("a boolean", True), ("a string", "1")]


@pytest.mark.parametrize("what,count", BAD_COUNTS, ids=[b[0] for b in BAD_COUNTS])
def test_the_count_domain_and_shape_are_checked_before_the_model_comparison(what, count):
    """Review r1 P2/P2: an integer from 1 to 2^53 - 1 (the checked JSON profile's safe integers), checked by the one
    shape validator both readings share, before any comparison with the model's evidence."""
    source, model = entry("v1_closing")
    source["contract_evidence"][MEMBER][0]["subtrahend"]["primitive_contents"][0]["count"] = count
    assert read(source) == difference.EVIDENCE
    assert read(source, model) == difference.EVIDENCE


def test_the_maximum_count_is_in_the_domain():
    source, model = entry("v1_closing")
    record = source["contract_evidence"][MEMBER][0]
    record["subtrahend"]["primitive_contents"][0]["count"] = MAX_COUNT
    assert read(source) == difference.GATE_DERIVATION, "in the domain, and not contained"
    record["minuend"]["primitive_contents"][0]["count"] = MAX_COUNT
    assert read(source) == "ok"
    assert read(source, model) == difference.MODEL


def test_member_order_alone_is_not_a_contradiction():
    source, _ = entry("v1_closing")
    _declare_plain_twin(source)
    entry_ = source["contract_evidence"][MEMBER][1]["subtrahend"]["primitive_contents"][0]
    source["contract_evidence"][MEMBER][1]["subtrahend"]["primitive_contents"][0] = dict(reversed(list(entry_.items())))
    assert read(source) == "ok"
