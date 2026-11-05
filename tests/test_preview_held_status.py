"""RT-A5 ``minuend_final`` (DEL-05-02 Work ``operating-case.md`` section 7.6): the Python mirror of the held-status
readers over the shared corpus (``fixtures/results/preview_physics_held_status_corpus.json``, written by PP
``op_mf_held_status.rs``). Every producer document reads with its model and without it; each forgery is refused with
the Rust reader's code (RE ``preview_held_status_tests.rs``)."""
from __future__ import annotations

import copy
import json
import struct
from pathlib import Path

import pytest

from tests.verifiers import preview_held_status as held
from tests.verifiers import reader_verdict

PROJECT = Path(__file__).resolve().parents[1]
CORPUS = json.loads((PROJECT / "fixtures/results/preview_physics_held_status_corpus.json").read_text(encoding="utf-8"))
NAMES = [e["name"] for e in CORPUS["documents"]]
MEMBER = held.MEMBER
OPERANDS_SHAPE = "SOURCE_PREVIEW_PHYSICS_COMBINATION_OPERANDS_SHAPE"
OPERANDS_ROWS = "SOURCE_PREVIEW_PHYSICS_COMBINATION_OPERANDS_ROWS"


def entry(name: str) -> tuple[dict, dict]:
    found = next(e for e in CORPUS["documents"] if e["name"] == name)
    return copy.deepcopy(found["document"]), copy.deepcopy(found["model"])


def code(outcome: dict) -> str:
    return "ok" if outcome["ok"] else outcome["error"]["code"]


def read(source: dict, model: dict | None = None) -> str:
    return code(reader_verdict.validate("preview-physics", source, model))


def hexbits(value: float) -> str:
    return f"{struct.unpack('>Q', struct.pack('>d', value))[0]:016x}"


def record(v: dict) -> dict:
    return v["contract_evidence"][MEMBER][0]


def first(v: dict) -> dict:
    return record(v)["held"][0]


def test_the_corpus_has_every_outcome():
    kinds = {e["document"]["contract_evidence"][MEMBER][0]["standing"]["kind"] for e in CORPUS["documents"]}
    assert kinds == {"established", "not_established", "gap_inspection", "withheld"}
    assert len(NAMES) >= 11


@pytest.mark.parametrize("name", NAMES)
def test_producer_documents_read_with_their_model_and_without_it(name):
    source, model = entry(name)
    assert read(source, model) == "ok"
    assert read(source) == "ok"


def _drop_held_rows(v):
    v["results"] = [r for r in v["results"] if ":held:" not in r["id"]]


def _first_row(v, predicate):
    return next(r for r in v["results"] if predicate(r))


def _minuend_code(v):
    _first_row(v, lambda r: r["kind"] == "nonlinear_support_active_set_state_code" and r["basis_ref"]["ref_id"] == "case:O")["value"] = 0.0


def _swap(v):
    r = record(v)
    r["source"]["load_case_id"], r["target"]["load_case_id"] = r["target"]["load_case_id"], r["source"]["load_case_id"]


def _extra_entry(v):
    extra = copy.deepcopy(first(v))
    extra["support_id"] = "support:u"
    record(v)["held"].append(extra)


FORGERIES = [
    ("empty member", "h2_released", OPERANDS_SHAPE, lambda v: v["contract_evidence"].__setitem__(MEMBER, [])),
    ("extra record member", "h2_released", held.EVIDENCE, lambda v: record(v).__setitem__("note", "x")),
    ("extra entry member", "h2_released", held.EVIDENCE, lambda v: first(v).__setitem__("note", "x")),
    ("missing input", "h4_kept_explicit", held.EVIDENCE, lambda v: first(v)["inputs"].pop("coefficient_bits")),
    ("an unknown state", "h2_released", held.EVIDENCE, lambda v: first(v).__setitem__("held_state", "closed")),
    ("a withheld record with held entries", "source_unavailable", held.EVIDENCE, lambda v: record(v).__setitem__("held", [{"support_id": "x"}])),
    ("unknown combination", "h2_released", OPERANDS_SHAPE, lambda v: record(v).__setitem__("combination_id", "comb:none")),
    ("source and target swapped", "h2_released", held.BINDING, _swap),
    ("another mode", "h2_released", held.BINDING, lambda v: record(v)["source"].__setitem__("solver_mode", "dense_scrutiny")),
    ("an outcome the gate does not state", "h2_released", OPERANDS_SHAPE, lambda v: record(v).__setitem__("outcome", "HELD_STATUS_BINDING")),
    ("the minuend's published code against the held state", "h2_released", held.BINDING, _minuend_code),
    ("a held entry missing", "h2_released", held.EVIDENCE, lambda v: record(v).__setitem__("held", [])),
    ("an extra held entry", "h2_released", held.BINDING, _extra_entry),
    ("a DOF forged", "h2_released", held.BINDING, lambda v: first(v).__setitem__("dof", "UY")),
    ("no held-operand rows", "h2_released", OPERANDS_ROWS, _drop_held_rows),
    ("classified state forged", "h2_released", held.DERIVATION, lambda v: first(v).__setitem__("classified_state", "active")),
    ("a sense forged", "h2_released", held.DERIVATION, lambda v: first(v).__setitem__("sense", "negative_reaction")),
    ("released forged", "h2_released", held.DERIVATION, lambda v: first(v).__setitem__("released", "none")),
    ("a stick decision without its inputs", "h4_kept_explicit", held.DERIVATION, lambda v: first(v)["inputs"].__setitem__("reaction_bits", hexbits(1.0e6))),
    ("a branch verdict forged", "h13_branch", held.DERIVATION, lambda v: first(v)["inputs"].__setitem__("branch_admissible", True)),
    ("a zero limit read by mu > 0 (H-15c)", "h15c_zero_limit", held.DERIVATION, lambda v: first(v).__setitem__("classified_state", "sticking")),
    ("a strict-gap decision against its signs", "h7g_gap", held.DERIVATION, lambda v: first(v)["strict_gap"].__setitem__("state", "active")),
    ("a strict-gap prior forged", "h7g_gap", held.EVIDENCE, lambda v: first(v)["strict_gap"].__setitem__("prior", "inactive")),
    ("a release promoted", "h2_released", OPERANDS_SHAPE, lambda v: record(v).__setitem__("standing", {"kind": "established", "reason": held.ESTABLISHED})),
    ("a gap standing forged", "h7g_gap", OPERANDS_SHAPE, lambda v: record(v)["standing"].__setitem__("kind", "not_established")),
    ("E without its conditions", "h4_kept_explicit", held.DERIVATION, lambda v: record(v).pop("conditions")),
    ("a condition's class forged", "h16a_input_decision", held.DERIVATION, lambda v: record(v)["conditions"][0].__setitem__("class", "strict_ball")),
    ("E's sentence edited", "h4_kept_explicit", OPERANDS_SHAPE, lambda v: record(v)["standing"].__setitem__("reason", "Established.")),
    ("the release notice removed", "h2_released", held.DERIVATION, lambda v: v.__setitem__("diagnostics", [d for d in v["diagnostics"] if d["code"] != held.RELEASED])),
    ("an unconverged held result claimed kept", "h4_kept_explicit", held.DERIVATION, lambda v: record(v)["held_solve"].__setitem__("converged", False)),
    # The derived normal's source (review P2-1), against the minuend's derived-normal row.
    ("a derived normal's source support forged", "h6b_kept_derived", held.BINDING, lambda v: first(v)["inputs"]["normal_source_ref"].__setitem__("support", "support:a")),
    ("a derived normal's source DOF forged", "h6b_kept_derived", held.BINDING, lambda v: first(v)["inputs"]["normal_source_ref"].__setitem__("dof", "UX")),
    ("a derived normal's source removed", "h6b_kept_derived", held.EVIDENCE, lambda v: first(v)["inputs"].pop("normal_source_ref")),
    ("a derived normal's source opened", "h6b_kept_derived", held.EVIDENCE, lambda v: first(v)["inputs"]["normal_source_ref"].__setitem__("note", "x")),
    ("an explicit normal given a source", "h4_kept_explicit", held.EVIDENCE,
     lambda v: first(v)["inputs"].__setitem__("normal_source_ref", {"support": "support:z", "node": "node:b", "dof": "UZ"})),
    # The source's provenance and the target's modulus key (review P2-2).
    ("an unqualified source admitted", "h4_kept_explicit", held.BINDING, lambda v: record(v)["source"].__setitem__("qualification", "inspectable_unqualified_gap")),
    ("an admitted source's standing removed", "h4_kept_explicit", held.BINDING, lambda v: record(v)["source"].__setitem__("standing", None)),
    ("a confirmed source stated not established", "h4_kept_explicit", held.BINDING, lambda v: record(v)["source"].__setitem__("standing", "not_established")),
    ("an unconfirmed source stated confirmed", "h15c_zero_limit", held.BINDING, lambda v: record(v)["source"].__setitem__("standing", "confirmed")),
    ("an absent source stated qualified", "source_unavailable", held.BINDING,
     lambda v: record(v)["source"].update({"qualification": "qualified", "standing": "not_established"})),
    ("a target modulus key forged", "h4_kept_explicit", held.BINDING, lambda v: record(v)["target"].__setitem__("modulus_basis_key", "bogus")),
    ("a target modulus key stated exact", "h4_kept_explicit", held.BINDING, lambda v: record(v)["target"].__setitem__("modulus_basis_key", "exact:point-a")),
    # The held solve's convergence against its decisions (review P2-3).
    ("a released held result claimed converged", "h2_released", held.DERIVATION,
     lambda v: record(v)["held_solve"].update({"converged": True, "exit_diagnostic": None})),
    ("a released held result's convergence alone forged", "h2_released", held.DERIVATION, lambda v: record(v)["held_solve"].__setitem__("converged", True)),
    ("a non-converged exit diagnostic removed", "h2_released", held.DERIVATION, lambda v: record(v)["held_solve"].__setitem__("exit_diagnostic", None)),
    ("a non-converged exit diagnostic renamed", "h2_released", held.DERIVATION,
     lambda v: record(v)["held_solve"]["exit_diagnostic"].__setitem__("code", "NONLINEAR_SUPPORT_STATE_REVIEW")),
    ("a non-converged exit diagnostic emptied", "h2_released", held.DERIVATION, lambda v: record(v)["held_solve"]["exit_diagnostic"].__setitem__("message", " ")),
    ("a converged held result given an exit diagnostic", "h4_kept_explicit", held.DERIVATION,
     lambda v: record(v)["held_solve"].__setitem__("exit_diagnostic", {"code": "NONLINEAR_SUPPORT_NONCONVERGENCE", "message": "x"})),
    ("the held sign on the minuend's case row", "h2_released", held.ROW,
     lambda v: _first_row(v, lambda r: r.get("basis_ref", {}).get("ref_type") == "load_case" and r.get("metadata") is not None)["metadata"].__setitem__("sign_convention", held.SIGN)),
    ("the typed basis on a difference row", "h2_released", OPERANDS_ROWS,
     lambda v: _first_row(v, lambda r: r.get("basis_ref", {}).get("ref_type") == "combination" and ":held:" not in r["id"] and r.get("metadata") is not None
                          and r["kind"] != "combination_modulus_basis_record")["metadata"].__setitem__("basis", held.OPERAND_BASIS)),
    ("a held row's clause removed", "h2_released", held.ROW,
     lambda v: _first_row(v, lambda r: ":held:" in r["id"])["metadata"].__setitem__("sign_convention", "positive along global X")),
]


@pytest.mark.parametrize("what,base,expected,edit", FORGERIES, ids=[f[0] for f in FORGERIES])
def test_both_readings_refuse_forged_evidence_by_name(what, base, expected, edit):
    source, model = entry(base)
    edit(source)
    assert read(source) == expected
    assert read(source, model) != "ok"


MODEL_FORGERIES = [
    ("a coefficient forged", "h4_kept_explicit", lambda v: first(v)["inputs"].__setitem__("coefficient_bits", hexbits(0.5))),
    ("an explicit normal forged consistently", "h4_kept_explicit",
     lambda v: (first(v)["inputs"].__setitem__("normal_signed_bits", hexbits(8192.0)), first(v)["inputs"].__setitem__("normal_trial_bits", hexbits(8192.0)))),
    ("a node forged", "h2_released", lambda v: first(v).__setitem__("node", "node:a")),
    ("the trial's ground forged", "h17c_moved_ground", lambda v: first(v)["inputs"].__setitem__("ground_bits", hexbits(1.0 / 1024.0))),
    ("a derived normal's source node forged", "h6b_kept_derived", lambda v: first(v)["inputs"]["normal_source_ref"].__setitem__("node", "node:a")),
]


@pytest.mark.parametrize("what,base,edit", MODEL_FORGERIES, ids=[f[0] for f in MODEL_FORGERIES])
def test_the_model_bound_reading_refuses_inputs_the_model_contradicts(what, base, edit):
    source, model = entry(base)
    edit(source)
    assert read(source, model) == held.MODEL


def test_the_sliding_ground_withholding_and_conditions_with_the_model():
    source, model = entry("sliding_ground_differs")
    for row in source["results"]:
        if row["kind"] == "nonlinear_support_active_set_state_code":
            row["value"] = 2.0
    assert read(source, model) == held.MODEL
    source, model = entry("h2_released")
    model["load_cases"][0]["condition_use"] = {"condition_set": "x"}
    assert read(source, model) == held.UNVERIFIED


def _model_check(source: dict, model: dict) -> str:
    try:
        held._model_check(source, model, held.read(source))
    except ValueError as exc:
        return str(exc)
    return "ok"


def test_the_model_bound_reading_refuses_a_derived_normal_source_the_model_moves():
    """Review P2-1: the document unchanged, the model's derived-normal source moved to another support and DOF."""
    source, model = entry("h6b_kept_derived")
    support = next(s for s in model["supports"] if s["id"] == "support:t")
    support["nonlinear"]["normal_reaction_source"] = {"support_ref": "support:a", "dof": "UX"}
    assert read(source, model) != "ok"
    assert read(source) == "ok"
    assert _model_check(source, model) == held.MODEL


def test_the_model_bound_reading_refuses_a_target_modulus_key_forged_coherently():
    """Review P2-2: a key and its modulus record label forged together read without the model, never with it."""
    source, model = entry("h4_kept_explicit")
    record(source)["target"]["modulus_basis_key"] = "exact:point-a"
    label = _first_row(source, lambda r: r["kind"] == held.MODULUS_RECORD and r["entity_ref"] == "case:S" and r["basis_ref"]["ref_type"] == "combination")
    label["metadata"]["basis"] = "temperature_point:point-a; selection=exact_user_entered_point; interpolation=not_performed; x"
    held.read(source)
    assert _model_check(source, model) == held.MODEL
