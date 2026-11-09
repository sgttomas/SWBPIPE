"""Focused tests of the pressure-elongation switch helper, the anchor's comparison and its reporting.

No solve runs here: the harness's solver is replaced by a stub. The full anchor is
`pressure_elongation_anchor.py check` (README.md in this directory).
"""
from __future__ import annotations

import copy
import json
import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[2]
if str(ROOT) not in sys.path:
    sys.path.append(str(ROOT))
from tools.validation import pressure_elongation as PE  # noqa: E402
from tools.validation import pressure_elongation_anchor as A  # noqa: E402
from tools.validation import pressure_elongation_inventory as INV  # noqa: E402

def region(pressure, rid="region:a", pipes=("pipe:a",)):
    out = {"id": rid, "member_pipe_ids": list(pipes), "pressure_basis": "internal_differential_zero_external_v1",
           "terminals": [], "provenance": "test"}
    if pressure is not None:
        out["pressure"] = {"value": pressure, "unit": "Pa"}
    return out


def document(**extra):
    doc = {"schema_version": A.MODEL_VERSION, "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3"},
           "pipe_segments": [{"id": "pipe:a", "section": {}, "condition_set_ref": "set:1"}],
           "condition_sets": [{"id": "set:1", "conditions": [{"key": "op", "pressure": {"value": 2.0e6, "unit": "Pa"}},
                                                            {"key": "cold", "pressure": {"value": 0.0, "unit": "Pa"}}]}],
           "load_cases": [{"id": "case:sus", "pressure_regions": [region(1.0e6)]},
                          {"id": "case:zero", "pressure_regions": [region(0.0)]},
                          {"id": "case:none", "pressure_regions": []},
                          {"id": "case:cond", "condition_use": {"pressure": "op"}, "pressure_regions": [region(None)]},
                          {"id": "case:cold", "condition_use": {"pressure": "cold"}, "pressure_regions": [region(None)]}]}
    doc.update(extra)
    return doc


# ----------------------------------------------------------------------------- the switch helper
def test_pressurised_cases_are_nonzero_regions_and_resolved_conditions_in_order():
    assert PE.pressurised_case_ids(document()) == ["case:sus", "case:cond"]
    assert PE.pressurised_case_ids({k: v for k, v in document().items() if k != "pressure_contract"}) == []


def test_all_names_every_pressurised_case_and_keeps_other_settings():
    doc = PE.apply(document(model_settings={"shear_model": "x"}), "all")
    assert doc["model_settings"] == {"shear_model": "x", "pressure_elongation": {"operating_load_cases": ["case:sus", "case:cond"]}}
    assert list(doc["model_settings"])[-1] == "pressure_elongation"


def test_absent_and_none_leave_todays_document_byte_identical():
    for mode in ("absent", "none"):
        before = json.dumps(document())
        assert json.dumps(PE.apply(document(), mode)) == before


def test_none_removes_the_member_and_a_record_that_only_held_it():
    doc = PE.apply(document(), "all")
    assert "model_settings" not in PE.apply(copy.deepcopy(doc), "none")
    kept = PE.apply(document(model_settings={"shear_model": "x"}), "all")
    assert PE.apply(kept, "none")["model_settings"] == {"shear_model": "x"}


def test_all_writes_nothing_without_a_pressurised_case():
    doc = document(load_cases=[{"id": "case:zero", "pressure_regions": [region(0.0)]}])
    assert "model_settings" not in PE.apply(doc, "all")


def test_switch_values_resolution_and_containers(monkeypatch):
    with pytest.raises(ValueError):
        PE.apply(document(), "some")
    monkeypatch.delenv(PE.ENV, raising=False)
    assert PE.argv_mode(["--write"]) == "absent"
    assert PE.argv_mode(["--pressure-elongation", "all"]) == "all"
    assert PE.argv_mode(["--pressure-elongation=none"]) == "none"
    monkeypatch.setenv(PE.ENV, "all")
    assert PE.resolve_mode(None) == "all" and PE.resolve_mode("absent") == "absent"
    container = {"cases": {"a": {"model": document()}, "b": {"model": document()}}, "note": "x"}
    PE.apply_to_container(container, "all")
    assert [p for p, _ in PE.documents(container)] == ["/cases/a/model", "/cases/b/model"]
    assert all("model_settings" in d for _, d in PE.documents(container))


# ----------------------------------------------------------------------------- the exclusion list
def test_the_exclusion_list_is_explicit_and_minimal():
    assert A.EXCLUDED_KEYS == frozenset({"pressure_elongation"})
    assert A.IDENTITY_SUBSTITUTIONS == (("pressure-3", "pressure-2"),)
    assert A.LIMITATIONS_KEY == "limitations"
    assert len(A.ADDED_LIMITATION_STATEMENTS) == 2


# ----------------------------------------------------------------------------- the comparison
BASE = {"producer": {"semantic_contract_id": "openpipestress.result_semantics/0.3.0/pressure-2"},
        "formulation_basis": {"limitations": ["3.0.0/exact_pressure_v3 under pressure-2 semantics."]},
        "contract_evidence": {"exact_cases": [{"load_case_id": "case:sus", "pressure_bits": "3ee4f8b588e368f1"}]},
        "results": [{"value": 0.1, "count": 3, "zero": 0.0, "flag": True, "none": None}]}


def switched():
    out = copy.deepcopy(BASE)
    out["producer"]["semantic_contract_id"] = "openpipestress.result_semantics/0.3.0/pressure-3"
    out["formulation_basis"]["limitations"] = ["3.0.0/exact_pressure_v3 under pressure-3 semantics.",
                                               sorted(A.ADDED_LIMITATION_STATEMENTS)[1]]
    out["contract_evidence"]["exact_cases"][0]["pressure_elongation"] = "included"
    return out


def test_identity_setting_and_label_bytes_are_the_only_allowed_differences():
    assert A.compare(BASE, switched()) == (0, None)


@pytest.mark.parametrize("pointer,value,reason", [
    ("/results/0/value", 0.1 + 2 ** -56, "number"),                 # last binary64 place
    ("/results/0/zero", -0.0, "number"),                             # signed zero
    ("/results/0/count", 3.0, "type int != float"),
    ("/results/0/flag", False, "!="),
    ("/results/0/none", 0, "type null != int"),
    ("/contract_evidence/exact_cases/0/pressure_bits", "3ee4f8b588e368f2", "string"),
    ("/producer/semantic_contract_id", "openpipestress.result_semantics/0.3.0/pressure-4", "string"),
])
def test_every_other_leaf_is_compared_exactly(pointer, value, reason):
    actual = switched()
    parent = A.resolve(actual, A.parent_pointer(pointer))
    key = pointer.rsplit("/", 1)[1]
    parent[int(key) if isinstance(parent, list) else key] = value
    count, first = A.compare(BASE, actual)
    assert count == 1 and first.startswith(f"{pointer}: ") and reason in first


def test_structure_differences_report_the_first_path_in_document_order():
    actual = switched()
    actual["results"].append({"value": 1.0})
    del actual["contract_evidence"]["exact_cases"][0]["load_case_id"]
    actual["extra/key"] = 1
    found = list(A.differences(BASE, actual))
    assert found[0] == ("/contract_evidence/exact_cases/0/load_case_id", "missing in actual")
    assert ("/results", "array length 1 != 2") in found
    assert ("/extra~1key", "unexpected in actual") in found


def test_added_statements_are_dropped_only_from_limitations_lists():
    statement = sorted(A.ADDED_LIMITATION_STATEMENTS)[0]
    assert A.compare({"notes": []}, {"notes": [statement]})[0] == 1
    assert A.compare({"limitations": []}, {"limitations": [statement]})[0] == 0


# ----------------------------------------------------------------------------- re-authoring and requests
def test_reauthoring_twin_matches_current_model_rs():
    old = {"schema_version": "0.4.0", "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3"},
           "pipe_segments": [{"section": {}}, {"section": {"corrosion_allowance": {"value": 1.0, "unit": "mm"}}}]}
    A.reauthor_model(old)
    assert old["schema_version"] == A.MODEL_VERSION and old["pressure_contract"] == A.CURRENT_CONTRACT
    assert [p["section"]["corrosion_allowance"] for p in old["pipe_segments"]] == [
        {"value": 0.0, "unit": "m"}, {"value": 1.0, "unit": "mm"}]
    previous = {"schema_version": "0.5.0", "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3"}, "pipe_segments": [{"section": {}}]}
    assert A.reauthor_model(copy.deepcopy(previous)) == {**previous, "schema_version": A.MODEL_VERSION}
    current = document()
    assert A.reauthor_model(copy.deepcopy(current)) == current


def test_requests_keep_their_own_materials_and_take_the_switch():
    container = {"wrapped": {"model": document(), "materials": [{"id": "m"}]}, "bare": {"model": document(), "label": "x"}}
    wrapped = A.request_for(container, "/wrapped/model", "all")
    assert wrapped["materials"] == [{"id": "m"}] and "model_settings" in wrapped["model"]
    assert A.request_for(container, "/bare/model", "absent") == {"model": document(), "materials": []}
    assert "model_settings" not in container["wrapped"]["model"]  # the committed container is untouched


# ----------------------------------------------------------------------------- reporting
class StubSolver:
    def __init__(self, outcome):
        self.outcome = outcome

    def solve(self, request, mode, example):
        return self.outcome(request, mode)


def entry(tmp_path, envelope):
    path = tmp_path / "corpus.json"
    path.write_text(json.dumps({"cases": [{"model": document(), "envelope": envelope, "solver_mode": "sparse_interactive"}]}))
    return {"id": "e", "path": str(path), "anchor": {"kind": "inline", "case_depth": 1, "envelope": "envelope",
                                                       "mode_member": "solver_mode"},
            "documents": [{"pointer": "/cases/0/model"}]}


def test_run_entry_reports_pass_fail_and_the_refused_member(tmp_path, monkeypatch):
    monkeypatch.setattr(A, "ROOT", Path("/"))
    e = entry(tmp_path, BASE)
    ok = A.run_entry(e, "all", ["sparse", "dense"], StubSolver(lambda r, m: ("envelope", switched(), "")), None, "check")
    assert ok["status"] == "PASS" and ok["detail"] == "1 envelopes identical"
    moved = switched()
    moved["results"][0]["value"] = 0.2
    fail = A.run_entry(e, "all", ["sparse"], StubSolver(lambda r, m: ("envelope", moved, "")), None, "check")
    assert fail["status"] == "FAIL" and "/results/0/value" in fail["detail"]
    refused = A.run_entry(e, "all", ["sparse"], StubSolver(lambda r, m: (
        "refused_member", None, 'Error: "request: unknown field `pressure_elongation`"')), None, "check")
    assert refused["status"] == "REFUSED" and "unknown field" in refused["detail"]
    seen = []
    A.run_entry(e, "all", ["sparse"], StubSolver(lambda r, m: seen.append(r) or ("envelope", BASE, "")), None, "check")
    assert seen[0]["model"]["model_settings"]["pressure_elongation"]["operating_load_cases"] == ["case:sus", "case:cond"]


# ----------------------------------------------------------------------------- the inventory
def test_the_inventory_is_current():
    assert INV.OUTPUT.read_text(encoding="utf-8") == INV.dumps(INV.build()), \
        "run python3 tools/validation/pressure_elongation_inventory.py and classify any new pressure fixture"
