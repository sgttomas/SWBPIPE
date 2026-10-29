"""Operating-case slice 1a (DEL-05-02 Design ``pf-nonlinear-operand-standing.md`` §3): the Python mirror of the
preview-physics-2 ``combination_operands`` checks over the shared corpus
(``fixtures/results/preview_physics_nonlinear_operands_corpus.json``, written by PP ``op1a_operand_standing.rs``).
The producer documents are admitted with and without their models; every forgery is an in-memory copy and is refused
with the same code as the Rust reader (``tests/preview_combination_operands_contract.rs``)."""
from __future__ import annotations

import copy
import json
from pathlib import Path

import pytest

from tests.verifiers import reader_verdict
from tests.verifiers.preview_physics_evidence import validate_preview_physics_evidence

PROJECT = Path(__file__).resolve().parents[1]
CORPUS = json.loads((PROJECT / "fixtures/results/preview_physics_nonlinear_operands_corpus.json").read_text(encoding="utf-8"))
PREFIX = "SOURCE_PREVIEW_PHYSICS_COMBINATION_OPERANDS_"


def entry(name: str) -> tuple[dict, dict]:
    found = next(e for e in CORPUS["documents"] if e["name"] == name)
    return copy.deepcopy(found["document"]), copy.deepcopy(found["model"])


def refused(source: dict, code: str) -> None:
    with pytest.raises(ValueError) as error:
        validate_preview_physics_evidence(source)
    assert str(error.value) == PREFIX + code


def position(source: dict, code: str) -> int:
    return next(i for i, d in enumerate(source["diagnostics"]) if d["code"] == code)


def records(source: dict) -> list:
    return source["contract_evidence"]["combination_operands"]


@pytest.mark.parametrize("name", ["v1_gap_closes", "v3_friction", "slip_reversal", "v2_anchor_rise"])
def test_producer_documents_are_admitted_with_and_without_their_models(name):
    source, model = entry(name)
    validate_preview_physics_evidence(source)
    assert reader_verdict.validate("preview-physics", source, model) == {"ok": True}


def forged(name: str, edit) -> dict:
    source, _ = entry(name)
    edit(source)
    return source


def _set(path, value):
    def edit(source):
        target = source
        for key in path[:-1]:
            target = target[key]
        target[path[-1]] = value
    return edit


OPS = ("contract_evidence", "combination_operands")
FORGERIES = [
    ("v1_gap_closes", _set(OPS + (0, "operands", 0, "numerical_quality", "solve_quality"), "checks_passed"), "STANDING"),
    ("v3_friction", _set(OPS + (0, "operands", 0, "numerical_quality", "solve_quality"), "sensitive"), "STANDING"),
    ("v1_gap_closes", _set(OPS + (0, "operands", 0, "numerical_quality"), None), "STANDING"),
    ("v1_gap_closes", lambda s: s["contract_evidence"].pop("combination_operands"), "PRESENCE"),
    ("v1_gap_closes", _set(OPS, []), "PRESENCE"),
    ("v1_gap_closes", lambda s: records(s).pop(), "PRESENCE"),
    ("v1_gap_closes", lambda s: records(s).reverse(), "PRESENCE"),
    ("v3_friction", lambda s: s.__setitem__("results", [r for r in s["results"] if r["kind"] != "nonlinear_support_active_set_state_code"]), "PRESENCE"),
    ("v1_gap_closes", _set(OPS + (0, "certified"), True), "SHAPE"),
    ("v1_gap_closes", _set(OPS + (0, "standing"), "established"), "SHAPE"),
    ("v1_gap_closes", _set(OPS + (0, "basis"), "mechanics"), "SHAPE"),
    ("v1_gap_closes", lambda s: records(s)[0]["operands"].reverse(), "SHAPE"),
    ("v1_gap_closes", _set(OPS + (0, "operands", 0, "extra"), 1), "SHAPE"),
    ("v1_gap_closes", lambda s: (records(s)[0].__setitem__("expression", {"minuend_id": "case:S", "subtrahend_id": "case:O"}),
                                 records(s)[0]["operands"].reverse()), "ROWS"),
    ("v3_friction", _set(("status", "mechanics"), "MECHANICS_BLOCKED"), "PRESENCE"),
]


@pytest.mark.parametrize("name,edit,code", FORGERIES)
def test_forged_records_are_refused(name, edit, code):
    refused(forged(name, edit), code)


def test_forged_rows_are_refused():
    source, _ = entry("v1_gap_closes")
    row = next(r for r in source["results"] if r["basis_ref"]["ref_id"] == "comb:o-s" and r["kind"] == "global_nodal_displacement_x")
    row["source_result_refs"].reverse()
    refused(source, "ROWS")


@pytest.mark.parametrize("code", ["COMBINATION_OPERAND_SUPPORT_STATES_DIFFER", "COMBINATION_OPERAND_CHECKS_NOT_PASSED"])
def test_forged_missing_and_extra_notices_are_refused(code):
    v1, _ = entry("v1_gap_closes")
    i = position(v1, code)
    missing = copy.deepcopy(v1)
    del missing["diagnostics"][i]
    refused(missing, "NOTICE")
    for field in ("id", "severity", "message", "source"):
        altered = copy.deepcopy(v1)
        altered["diagnostics"][i][field] = "forged"
        refused(altered, "NOTICE")
    refs = copy.deepcopy(v1)
    refs["diagnostics"][i]["affected_refs"].pop()
    refused(refs, "NOTICE")
    duplicate = copy.deepcopy(v1)
    duplicate["diagnostics"].append(copy.deepcopy(v1["diagnostics"][i]))
    refused(duplicate, "NOTICE")
    extra, _ = entry("v3_friction")
    extra["diagnostics"].append(copy.deepcopy(v1["diagnostics"][i]))
    refused(extra, "NOTICE")


def test_forged_support_states_are_refused():
    equal, _ = entry("v1_gap_closes")
    for row in equal["results"]:
        if row["kind"] == "nonlinear_support_active_set_state_code" and row["basis_ref"]["ref_id"] == "case:O":
            row["value"] = 0.0
    refused(equal, "NOTICE")
    outside, _ = entry("v1_gap_closes")
    for row in outside["results"]:
        if row["kind"] == "nonlinear_support_active_set_state_code":
            row["value"] = 7.0
    refused(outside, "STATE_CODE")


def test_the_model_binding_refuses_forged_expressions_and_selections():
    v1, model = entry("v1_gap_closes")

    def refused_with(source, model):
        verdict = reader_verdict.validate("preview-physics", source, model)
        assert verdict["ok"] is False and "COMBINATION_OPERANDS_MODEL" in json.dumps(verdict), verdict

    m = copy.deepcopy(model)
    m["combinations"][1]["mode"] = "min"
    refused_with(v1, m)
    selection = copy.deepcopy(v1)
    records(selection)[0]["operands"][0]["modulus_basis_ref"] = "point:hot"
    validate_preview_physics_evidence(selection)
    refused_with(selection, model)
    mode = copy.deepcopy(v1)
    records(mode)[1]["expression"]["mode"] = "min"
    validate_preview_physics_evidence(mode)
    refused_with(mode, model)
    m = copy.deepcopy(model)
    m["supports"] = [s for s in m["supports"] if "nonlinear" not in s]
    refused_with(v1, m)
    m = copy.deepcopy(model)
    del m["combinations"][1]
    refused_with(v1, m)


def test_slip_sense_is_part_of_the_reported_state():
    """Codex r1 finding 1: a sliding support's slip sense; the relative row decides it when present."""
    dropped, _ = entry("slip_reversal")
    dropped["diagnostics"] = [d for d in dropped["diagnostics"] if d["code"] != "COMBINATION_OPERAND_SUPPORT_STATES_DIFFER"]
    refused(dropped, "NOTICE")
    same, _ = entry("slip_reversal")
    for row in same["results"]:
        if row["kind"] == "nonlinear_support_final_displacement" and row["basis_ref"]["ref_id"] == "case:O":
            row["value"] = 0.9765625
    refused(same, "NOTICE")
    relative, _ = entry("slip_reversal")
    row = copy.deepcopy(next(r for r in relative["results"] if r["kind"] == "nonlinear_support_final_displacement" and r["basis_ref"]["ref_id"] == "case:O"))
    row.update(id="result:nonlinear-support:support-shoe:UX-relative-displacement:forged", kind="nonlinear_support_final_relative_displacement", value=0.5)
    row["metadata"]["component"] = "nonlinear_support_final_relative_displacement"
    relative["results"].append(row)
    refused(relative, "NOTICE")


def _tag(source):
    row = next(r for r in source["results"] if r["basis_ref"]["ref_id"] == "comb:o-s" and r["kind"] == "global_nodal_rotation_x")
    row["metadata"]["basis"] = "explicit_user_linear_combination"


def test_admitted_mechanics_rows_never_exempt_a_record():
    """Codex r1 finding 2."""
    stripped, _ = entry("v1_gap_closes")
    _tag(stripped)
    records(stripped).pop(0)
    stripped["diagnostics"] = [d for d in stripped["diagnostics"]
                               if not (d["code"].startswith("COMBINATION_OPERAND_") and d["affected_refs"][0] == "comb:o-s")]
    refused(stripped, "MECHANICS_ADMITTED")
    tagged, _ = entry("v1_gap_closes")
    _tag(tagged)
    refused(tagged, "MECHANICS_ADMITTED")


def _index(source, cid, kind):
    return next(i for i, r in enumerate(source["results"]) if r["basis_ref"]["ref_id"] == cid and r["kind"] == kind)


def test_row_references_follow_kind_and_basis():
    """Codex r1 finding 3."""
    cleared, _ = entry("v1_gap_closes")
    records(cleared)[0]["expression"] = {"minuend_id": "case:S", "subtrahend_id": "case:O"}
    records(cleared)[0]["operands"].reverse()
    for row in cleared["results"]:
        if row["basis_ref"]["ref_id"] == "comb:o-s":
            row["source_result_refs"] = []
    refused(cleared, "ROWS")
    v1, _ = entry("v1_gap_closes")
    i = _index(v1, "comb:o-s", "global_nodal_displacement_x")
    y = _index(v1, "comb:o-s", "global_nodal_displacement_y")
    m = _index(v1, "comb:o-s", "support_reaction_force_magnitude_v2")
    c = _index(v1, "comb:o-s", "support_reaction_component_v2")
    edits = [
        lambda s: s["results"][i]["source_result_refs"].pop(),
        lambda s: s["results"][i].pop("source_result_refs"),
        lambda s: s["results"][i]["source_result_refs"].__setitem__(1, s["results"][y]["id"]),
        lambda s: s["results"][i]["source_result_refs"].__setitem__(1, s["results"][y]["source_result_refs"][1]),
        lambda s: s["results"][m]["source_result_refs"].reverse(),
        lambda s: s["results"][m].__setitem__("source_result_refs", list(s["results"][c]["source_result_refs"])),
        lambda s: s["results"][i]["metadata"].__setitem__("basis", "explicit_user_range_envelope"),
    ]
    for edit in edits:
        source = copy.deepcopy(v1)
        edit(source)
        refused(source, "ROWS")


@pytest.mark.parametrize("mode", ["invalid", "", None, "MAX"])
def test_range_modes_are_closed(mode):
    """Codex r1 finding 4."""
    source, _ = entry("v1_gap_closes")
    records(source)[1]["expression"]["mode"] = mode
    refused(source, "SHAPE")
