"""Focused tests of the pressure-elongation switch helper, the anchor's comparison and its reporting.

No solve runs here: the harness's solver is replaced by a stub. The full anchor is
`pressure_elongation_anchor.py check` (README.md in this directory).
"""
from __future__ import annotations

import copy
import json
import struct
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
    assert len(A.ADDED_LIMITATION_STATEMENTS) == 3


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


# ----------------------------------------------------------------------------- the receipt chain's commitments
def _sha(text: str) -> str:
    import hashlib
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def chained_envelope(invocation: str):
    """A retained-source envelope and its dump whose request hash is `invocation`, every commitment the
    hash of its preimage (the receipt's construction)."""
    evidence = {"exact_cases": [{"load_case_id": "case:a", "pressure_bits": "3ee4f8b588e368f1"}],
                "pressure": [{"load_case_id": "case:a", "region_id": "region:a", "p_pa": 2.0e6}]}
    payload = json.dumps({"payload": {"invocation_sha256": invocation, "terms": [0.5]}})
    plan = json.dumps({"payload": {"normalized_source_sha256": _sha(payload), "steps": 3}})
    case = {"basis_ref": {"ref_type": "load_case", "ref_id": "case:a"},
            "source": {"normalized_source_sha256": _sha(payload), "functional_plan_sha256": _sha(plan)},
            "physical_evidence_sha256": A._domain_hash(A.CASE_EVIDENCE_DOMAIN, {"exact_case": evidence["exact_cases"][0],
                                                                                 "pressure": evidence["pressure"]})}
    envelope = {"contract_evidence": evidence, "results": [{"id": "result:a", "value": 0.25}]}
    body = {"invocation": {"value": invocation}, "cases": [case], "publication_sha256": A._domain_hash(A.PUBLICATION_DOMAIN, envelope)}
    envelope["source_block_recovery"] = {"body": body, "receipt_sha256": A._domain_hash(A.RECEIPT_DOMAIN, body)}
    dump = {"records": [{"case_id": "case:a", "normalized_source_text": payload, "normalized_source_sha256": _sha(payload),
                         "functional_plan_text": plan, "functional_plan_sha256": _sha(plan)}]}
    return envelope, dump


def test_the_canonical_text_is_the_committed_vectors():
    cases = json.loads((A.ROOT / "fixtures/canonical_hash/cases.json").read_text(encoding="utf-8"))["cases"]
    for case in cases:
        try:
            value = A.loads(case["input_json"])
        except ValueError:
            continue
        if case.get("expected_canonical") is not None:
            assert A.canonical_json(value) == case["expected_canonical"], case["case_id"]


def test_a_zeroed_physical_evidence_commitment_fails_the_chain():
    reference, reference_dump = chained_envelope("1" * 64)
    actual, actual_dump = chained_envelope("2" * 64)
    assert A.receipt_chain(reference, actual, reference_dump, actual_dump) is None
    # Zeroed, with the receipt re-sealed over it so that only the case's commitment disagrees.
    recovery = actual["source_block_recovery"]
    recovery["body"]["cases"][0]["physical_evidence_sha256"] = "0" * 64
    recovery["receipt_sha256"] = A._domain_hash(A.RECEIPT_DOMAIN, recovery["body"])
    assert A.receipt_chain(reference, actual, reference_dump, actual_dump) == \
        "actual: case:a physical_evidence_sha256 is not the hash of its published preimage"
    # The publication and receipt commitments are recomputed too.
    actual, actual_dump = chained_envelope("2" * 64)
    actual["source_block_recovery"]["receipt_sha256"] = "0" * 64
    assert A.receipt_chain(reference, actual, reference_dump, actual_dump) == "actual: receipt_sha256 is not the hash of the receipt body"
    actual, actual_dump = chained_envelope("2" * 64)
    actual["results"][0]["value"] = 0.5
    assert A.receipt_chain(reference, actual, reference_dump, actual_dump) == \
        "actual: publication_sha256 is not the hash of the published envelope"


def test_profiles_the_default_unchanged_and_none_excludes_nothing():
    default = A.PROFILES[A.DEFAULT_PROFILE]
    assert A.DEFAULT_PROFILE == "pressure-elongation"
    assert (default["keys"], default["identity_substitutions"], default["limitations_dropped"], default["bend_clause_substitutions"]) == \
        (A.EXCLUDED_KEYS, A.IDENTITY_SUBSTITUTIONS, A.ADDED_LIMITATION_STATEMENTS, A.BEND_CLAUSE_SUBSTITUTIONS)
    assert list(A.differences(BASE, switched())) == list(A.differences(BASE, switched(), profile=default)) == []
    # Under `none` the identity, the added statement and the per-case label each differ.
    assert [p for p, _ in A.differences(BASE, switched(), profile=A.PROFILES["none"])] == [
        "/producer/semantic_contract_id", "/formulation_basis/limitations", "/formulation_basis/limitations/0",
        "/contract_evidence/exact_cases/0/pressure_elongation"]


# ----------------------------------------------------------------------------- --baseline-only
def write_baseline(directory, envelope, entry_id="e", pointer="/cases/0/model", mode="sparse"):
    directory.mkdir(exist_ok=True)
    (directory / f"{A.baseline_key(entry_id, pointer, mode)}.json").write_text(json.dumps(envelope))
    return directory


def test_chained_is_error_reports_a_chained_pass_as_an_error(tmp_path, monkeypatch):
    monkeypatch.setattr(A, "ROOT", Path("/"))
    reference, reference_dump = chained_envelope("1" * 64)
    actual, actual_dump = chained_envelope("2" * 64)
    e = entry(tmp_path, reference)
    (tmp_path / "base").mkdir()
    write_baseline(tmp_path / "base", reference)
    (tmp_path / "base" / f"{A.baseline_key('e', '/cases/0/model', 'sparse')}.source-dump.json").write_text(json.dumps(reference_dump))

    class Chained(StubSolver):
        def dump(self, request, mode):
            return actual_dump
    solver = Chained(lambda r, m: ("envelope", actual, ""))
    chained = A.run_entry(e, "all", ["sparse"], solver, tmp_path / "base", "check", True)
    assert chained["status"] == "PASS_CHAINED", chained
    refused = A.run_entry(e, "all", ["sparse"], solver, tmp_path / "base", "check", True, chained_is_error=True)
    assert refused["status"] == "ERROR" and refused["detail"].startswith("PASS_CHAINED under --chained-is-error"), refused


def flip_last_bit(value: float) -> float:
    return struct.unpack(">d", (struct.unpack(">Q", struct.pack(">d", value))[0] ^ 1).to_bytes(8, "big"))[0]


def test_baseline_only_never_reads_the_committed_envelope(tmp_path, monkeypatch):
    monkeypatch.setattr(A, "ROOT", Path("/"))
    regenerated = switched()
    regenerated["results"][0]["value"] = 0.2  # the branch's committed envelope, moved with the solve
    e = entry(tmp_path, regenerated)
    solve = StubSolver(lambda r, m: ("envelope", regenerated, ""))
    assert A.run_entry(e, "all", ["sparse"], solve, None, "check")["status"] == "PASS"  # self-comparison
    baseline = write_baseline(tmp_path / "baseline", BASE)
    for only in (False, True):  # a baseline beside a committed envelope is used only with the option
        result = A.run_entry(e, "all", ["sparse"], solve, baseline, "check", baseline_only=only)
        assert result["status"] == ("FAIL" if only else "PASS")
    fail = A.run_entry(e, "all", ["sparse"], solve, baseline, "check", baseline_only=True)
    assert "/results/0/value" in fail["detail"] and "0.1 != 0.2" in fail["detail"]


def test_baseline_only_errors_on_a_missing_baseline_envelope(tmp_path, monkeypatch):
    monkeypatch.setattr(A, "ROOT", Path("/"))
    e = entry(tmp_path, BASE)
    solve = StubSolver(lambda r, m: ("envelope", switched(), ""))
    empty = tmp_path / "empty"
    empty.mkdir()
    for baseline in (empty, tmp_path / "absent", write_baseline(tmp_path / "other", BASE, pointer="/cases/1/model")):
        result = A.run_entry(e, "all", ["sparse"], solve, baseline, "check", baseline_only=True)
        assert result["status"] == "ERROR" and "no baseline envelope" in result["detail"]
    # nothing to compare (the entry's only mode is not requested) is an ERROR too, not UNCOMPARED
    nothing = A.run_entry(e, "all", ["dense"], solve, write_baseline(tmp_path / "b", BASE), "check", baseline_only=True)
    assert nothing["status"] == "ERROR" and "nothing compared" in nothing["detail"]


def test_one_flipped_bit_in_a_baseline_float_fails_the_entry(tmp_path, monkeypatch):
    monkeypatch.setattr(A, "ROOT", Path("/"))
    e = entry(tmp_path, BASE)
    solve = StubSolver(lambda r, m: ("envelope", switched(), ""))
    assert A.run_entry(e, "all", ["sparse"], solve, write_baseline(tmp_path / "same", BASE), "check",
                       baseline_only=True)["status"] == "PASS"
    flipped = copy.deepcopy(BASE)
    flipped["results"][0]["value"] = flip_last_bit(0.1)
    assert flipped["results"][0]["value"] != 0.1
    result = A.run_entry(e, "all", ["sparse"], solve, write_baseline(tmp_path / "flipped", flipped), "check",
                         baseline_only=True)
    assert result["status"] == "FAIL" and result["detail"].startswith("/cases/0/model [sparse]: 1 differences")
    assert "/results/0/value" in result["detail"]


def test_baseline_only_exit_status_is_nonzero_on_a_missing_entry(tmp_path, monkeypatch, capsys):
    monkeypatch.setattr(A, "ROOT", Path("/"))
    e = entry(tmp_path, BASE)

    class Built(StubSolver):
        def __init__(self, examples_dir):
            super().__init__(lambda r, m: ("envelope", switched(), ""))

        def ensure(self):
            pass

    monkeypatch.setattr(A, "Solver", Built)
    monkeypatch.setattr(A, "load_inventory", lambda: {"entries": [e]})
    argv = ["check", "--switch", "all", "--modes", "sparse", "--baseline-only", "--json", str(tmp_path / "r.json")]
    assert A.main(argv + ["--baseline", str(write_baseline(tmp_path / "ok", BASE))]) == 0
    empty = tmp_path / "empty"
    empty.mkdir()
    assert A.main(argv + ["--baseline", str(empty)]) == 1
    report = json.loads((tmp_path / "r.json").read_text())
    assert report["reference"] == "baseline only" and report["totals"] == {"ERROR": 1}
    with pytest.raises(SystemExit, match="needs --baseline"):
        A.main(argv)


def test_baseline_only_compares_every_mode_the_baseline_holds(tmp_path, monkeypatch):
    monkeypatch.setattr(A, "ROOT", Path("/"))
    e = entry(tmp_path, BASE)  # the committed envelope holds sparse only
    seen = []
    solve = StubSolver(lambda r, m: seen.append(m) or ("envelope", switched(), ""))
    baseline = write_baseline(write_baseline(tmp_path / "both", BASE), BASE, mode="dense")
    both = A.run_entry(e, "all", ["sparse", "dense"], solve, baseline, "check", baseline_only=True)
    assert both["status"] == "PASS" and both["detail"] == "2 envelopes identical" and seen == ["sparse", "dense"]
    # by default the committed envelope's modes still decide
    seen.clear()
    assert A.run_entry(e, "all", ["sparse", "dense"], solve, baseline, "check")["detail"] == "1 envelopes identical"
    assert seen == ["sparse"]
    # a held mode the run did not compare is an ERROR, never a silently smaller comparison
    dropped = A.run_entry(e, "all", ["sparse"], solve, baseline, "check", baseline_only=True)
    assert dropped["status"] == "ERROR" and "not compared: /cases/0/model [dense]" in dropped["detail"]


def test_entry_baseline_takes_one_entrys_references_from_another_directory(tmp_path, monkeypatch, capsys):
    monkeypatch.setattr(A, "ROOT", Path("/"))
    e = entry(tmp_path, BASE)

    class Built(StubSolver):
        def __init__(self, examples_dir):
            super().__init__(lambda r, m: ("envelope", switched(), ""))

        def ensure(self):
            pass

    monkeypatch.setattr(A, "Solver", Built)
    monkeypatch.setattr(A, "load_inventory", lambda: {"entries": [e]})
    empty = tmp_path / "empty"
    empty.mkdir()
    argv = ["check", "--switch", "all", "--modes", "sparse", "--baseline-only", "--baseline", str(empty),
            "--json", str(tmp_path / "r.json")]
    assert A.main(argv) == 1
    own = write_baseline(tmp_path / "own", BASE)
    assert A.main(argv + ["--entry-baseline", f"e={own}"]) == 0
    report = json.loads((tmp_path / "r.json").read_text())
    assert report["entry_baselines"] == {"e": str(own.resolve())} and report["totals"] == {"PASS": 1}
    with pytest.raises(SystemExit, match="not run"):
        A.main(argv + ["--entry-baseline", f"other={own}"])
    with pytest.raises(SystemExit, match="ID=DIR"):
        A.main(argv + ["--entry-baseline", str(own)])


# ----------------------------------------------------------------------------- the inventory
def test_the_inventory_is_current():
    assert INV.OUTPUT.read_text(encoding="utf-8") == INV.dumps(INV.build()), \
        "run python3 tools/validation/pressure_elongation_inventory.py and classify any new pressure fixture"
