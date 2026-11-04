"""Operating-case slice 1b (DEL-05-02 Work ``operating-case.md`` section 1.2 and "As built (1b)"): the Python mirror of
the declared-difference readers over the shared corpus (``fixtures/results/preview_physics_difference_basis_corpus.json``,
written by PP ``op1b_difference_basis.rs``). Every producer document reads with its model, is refused without it under
the shipped switch (option C), and reads without it under option A; each forgery is refused with the Rust reader's code
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


def read(source: dict, model: dict | None = None, mode: str = "refuse", monkeypatch=None) -> str:
    monkeypatch.setattr(difference, "MODEL_FREE", mode)
    return code(reader_verdict.validate("preview-physics", source, model))


def bits(value: float) -> str:
    return f"{struct.unpack('>Q', struct.pack('>d', value))[0]:016x}"


def test_the_shipped_switch_fails_closed():
    assert difference.MODEL_FREE == "refuse"
    assert len(NAMES) >= 9


@pytest.mark.parametrize("name", NAMES)
def test_producer_documents_read_with_the_model_under_option_a_and_are_refused_under_option_c(name, monkeypatch):
    source, model = entry(name)
    assert read(source, model, monkeypatch=monkeypatch) == "ok"
    assert read(source, None, "verify", monkeypatch) == "ok"
    assert read(source, None, "refuse", monkeypatch) == difference.UNVERIFIED


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
def test_option_a_refuses_tampered_evidence_by_name(what, name, expected, edit, monkeypatch):
    source, _ = entry(name)
    edit(source)
    assert read(source, None, "verify", monkeypatch) == expected


def test_a_coherent_forgery_is_not_authenticated_without_the_model(monkeypatch):
    source, model = entry("v1_closing")
    source["contract_evidence"][MEMBER][0]["subtrahend"]["primitive_contents"] = []
    assert read(source, None, "verify", monkeypatch) == "ok"
    assert read(source, model, monkeypatch=monkeypatch) == difference.MODEL


def test_the_model_bound_reader_refuses_evidence_unlike_the_model_and_condition_use(monkeypatch):
    source, model = entry("v1_closing")
    _bump(source)
    assert read(source, model, monkeypatch=monkeypatch) == difference.MODEL
    source, model = entry("v1_closing")
    model["load_cases"][0]["condition_use"] = {"temperature": "OPE1"}
    assert read(source, model, monkeypatch=monkeypatch) == difference.UNVERIFIED
