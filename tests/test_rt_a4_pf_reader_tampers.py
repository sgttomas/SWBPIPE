"""RT-A4 review r1 P2-P3 (DEL-05-02 ``absolute-sum-srss-combinations.md`` sections 4-5, 10): preview-physics-2's
combination gates and unsigned rows are re-derived by both readers, and the runner's ``read preview-physics`` takes the
model.

Each envelope is solved by the headless preview runner from an invented model written here; each tamper is read through
the actual CLI (``openpipestress-runner read preview-physics`` via ``core/analysis_runs/rust_reader.py``) and by the
independent Python mirror (``tests/verifiers/reader_verdict.py``), with and without the model. Both name the same code.
Before the fix the CLI refused ``{source, model}`` as an unsupported member and both readers admitted every tamper.

Design section 10.3: preview-physics-2 publishes ``contract_evidence.combination_derivation`` (the model conditions and
each unsigned or nested combination's expression, with the combinations it reaches), so both readers verify such a
document without its model and refuse a tampered or missing derivation.
"""
from __future__ import annotations

import copy
import json
import os
import subprocess
from pathlib import Path

import pytest

from core.analysis_runs import rust_reader
from tests.verifiers import reader_verdict as py

PROV = "rt-a4 invented reader-tamper input; not library, component, material-catalog or code data"
CE_UNSIGNED = "CONSTANT_EFFORT_UNSIGNED_COMBINATION_OPERAND"
CE = "CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE"
UNVERIFIED_GATE = "SOURCE_PREVIEW_PHYSICS_COMBINATION_GATE_UNVERIFIED"
UNVERIFIED_ROW = "SOURCE_PREVIEW_PHYSICS_COMBINATION_UNSIGNED_UNVERIFIED"
DERIVATION = "SOURCE_PREVIEW_PHYSICS_COMBINATION_GATE_DERIVATION"
VALUE = "SOURCE_PREVIEW_PHYSICS_COMBINATION_UNSIGNED_VALUE"
SOURCES = "SOURCE_PREVIEW_PHYSICS_COMBINATION_UNSIGNED_SOURCES"
EVIDENCE = "SOURCE_PREVIEW_PHYSICS_COMBINATION_DERIVATION_EVIDENCE"
DERIVATION_MEMBER = "combination_derivation"


def qv(value, unit):
    return {"value": value, "unit": unit}


def terms(cid, basis, items):
    return {"id": cid, "basis": basis, "provenance": PROV,
            "terms": [{"load_case" if kind == "case" else "combination": operand, "factor": factor} for kind, operand, factor in items]}


def sub(cid, minuend, subtrahend):
    return {"id": cid, "basis": "result_state_subtraction", "minuend_id": minuend, "subtrahend_id": subtrahend, "provenance": PROV}


def _model(nodes, pipe, supports, materials, cases, combinations, project):
    return {"schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
            "project": {"id": project, "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
            "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
            "nodes": nodes, "pipe_segments": [pipe], "supports": supports, "components": [], "materials": materials,
            "load_cases": cases, "combinations": combinations}


def _force(cid, direction, value):
    return {"id": cid, "provenance": PROV, "primitive_loads": [{"id": f"load:{cid}", "category": "concentrated_force", "target": {"type": "node", "node": "node:b"},
            "direction": direction, "magnitude": qv(value, "N"), "dimension": "force", "provenance": PROV}]}


def constant_effort_cantilever(combinations):
    """The 1 m cantilever of ``rt_a4_combinations.rs`` V-6 with a consuming constant-effort support at its tip."""
    nodes = [{"id": "node:a", "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": PROV}, {"id": "node:b", "position": {"x": 1.0, "y": 0.0, "z": 0.0}, "provenance": PROV}]
    pipe = {"id": "pipe:a-b", "from": "node:a", "to": "node:b", "section": {"outside_diameter": qv(0.12, "m"), "wall_thickness": qv(0.01, "m")},
            "material": "material:m", "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0}, "provenance": PROV}
    supports = [{"id": "support:a", "node": "node:a", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV},
                {"id": "support:ce", "node": "node:b", "family": "constant_effort_support", "restraints": ["UY"], "provenance": PROV,
                 "hanger": {"hanger_type": "constant_effort_support", "constant_load": qv(375.0, "N"), "travel_range": qv(0.05, "m"), "source_reference": "rt_a4_invented"}}]
    materials = [{"id": "material:m", "elastic_modulus": qv(200e9, "Pa"), "shear_modulus": qv(76.923076923e9, "Pa"), "provenance": PROV}]
    cases = [_force("W1", "global_y", -1000.0), _force("W2", "global_y", -500.0), _force("W3", "global_z", 250.0)]
    return _model(nodes, pipe, supports, materials, cases, combinations, "project:rt-a4-tamper-ce")


def gap_bar(combinations):
    """The op1a bar of ``rt_a4_combinations.rs`` V-9: a tip gap closing in UX (a nonlinear support)."""
    import math
    length, od, wall, k = 4.0, 0.12, 0.01, 1_048_576.0
    inner = od - 2.0 * wall
    e = 4.0 * k / (math.pi / 4.0 * (od * od - inner * inner))
    nodes = [{"id": "node:a", "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": PROV}, {"id": "node:b", "position": {"x": length, "y": 0.0, "z": 0.0}, "provenance": PROV}]
    pipe = {"id": "pipe:a-b", "from": "node:a", "to": "node:b", "section": {"outside_diameter": qv(od, "m"), "wall_thickness": qv(wall, "m")},
            "material": "material:m", "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0}, "provenance": PROV}
    supports = [{"id": "support:a", "node": "node:a", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV},
                {"id": "support:gap", "node": "node:b", "family": "nonlinear", "restraints": [], "provenance": PROV,
                 "nonlinear": {"behavior": "gap", "closes_when": "positive_displacement", "dof": "UX", "gap": qv(2.0 ** -9, "m"), "initial_state": "inactive"}}]
    materials = [{"id": "material:m", "elastic_modulus": qv(e, "Pa"), "shear_modulus": qv(e / 2.6, "Pa"), "thermal_expansion_coefficient": qv(1.0 / 65536.0, "1/degC"), "provenance": PROV}]
    force = lambda cid, f: {"id": cid, "category": "concentrated_force", "target": {"type": "node", "node": "node:b"}, "direction": "global_x",
                            "magnitude": qv(f, "N"), "dimension": "force", "provenance": PROV}
    thermal = {"id": "t-o", "category": "thermal", "target": {"type": "element", "pipe": "pipe:a-b"}, "direction": "global_x",
               "magnitude": qv(16.0, "degC"), "dimension": "temperature_interval", "provenance": PROV}
    cases = [{"id": "S", "primitive_loads": [force("f-s", 1536.0)], "provenance": PROV},
             {"id": "O", "primitive_loads": [force("f-o", 1536.0), thermal], "provenance": PROV}]
    return _model(nodes, pipe, supports, materials, cases, combinations, "project:rt-a4-tamper-gap")


@pytest.fixture(scope="module")
def solve(tmp_path_factory):
    runner = os.environ.get("OPENPIPESTRESS_PREVIEW_RUNNER_BIN")
    if not runner or not Path(runner).is_file():
        pytest.skip("OPENPIPESTRESS_PREVIEW_RUNNER_BIN is not supplied")
    directory = tmp_path_factory.mktemp("rt_a4_tamper")

    def run(model):
        path = directory / f"{model['project']['id'].split(':')[-1]}.json"
        path.write_text(json.dumps(model), encoding="utf-8")
        completed = subprocess.run([runner, str(path), "--explicit-local-private-intent"], capture_output=True, check=True)
        envelope = json.loads(completed.stdout)["payload"]["mechanics_envelope"]
        assert envelope["status"]["mechanics"] == "MECHANICS_SOLVED"
        return envelope
    return run


def cli(source, model=None):
    """The runner's verdict: None when it passes, else its refusal code."""
    payload = {"source": source} if model is None else {"source": source, "model": model}
    passed, outcome = rust_reader._run("preview-physics", payload)
    return None if passed else outcome["error"]["code"]


def mirror(source, model=None):
    verdict = py.validate("preview-physics", source, model)
    return None if verdict["ok"] else verdict["error"]["code"]


def both(source, model, expected):
    assert (cli(source, model), mirror(source, model)) == (expected, expected), f"model given: {model is not None}"


def set_gate(source, cid, reason):
    gate = next(g for g in source["contract_evidence"]["combination_gates"] if g["combination_id"] == cid)
    gate["withheld"], gate["reason"] = reason is not None, reason


def set_row(source, cid, kind, location, before, after):
    row = next(r for r in source["results"] if r.get("basis_ref") == {"ref_type": "combination", "ref_id": cid}
               and r["kind"] == kind and (r.get("metadata") or {}).get("location") == location)
    assert row["value"] == before
    row["value"] = after


def test_reason_swap_and_value_change_in_a_nonlinear_model(solve):
    model = gap_bar([sub("D", "O", "S"), terms("E", "srss", [("comb", "D", 1.0)]),
                     terms("occ", "mechanics", [("case", "S", 1.0), ("comb", "E", 1.0)]),
                     terms("flat", "mechanics", [("case", "S", 1.0), ("case", "O", 1.0)])])
    source = solve(model)
    for given in (None, model):
        both(source, given, None)
    swapped = copy.deepcopy(source)
    set_gate(swapped, "flat", CE_UNSIGNED)
    both(swapped, None, UNVERIFIED_GATE)
    both(swapped, model, DERIVATION)
    changed = copy.deepcopy(source)
    set_row(changed, "E", "element_local_shear_force_y", "midspan", 0.0, 123.0)
    both(changed, None, VALUE)  # the operand record states E's factor
    both(changed, model, VALUE)


def test_value_and_kappa_gate_changes_in_a_constant_effort_model(solve):
    model = constant_effort_cantilever([terms("srss-cases", "srss", [("case", "W1", 1.0), ("case", "W2", 1.0)]), sub("d1", "W1", "W2"),
                                        terms("abs-d", "absolute_sum", [("comb", "d1", 1.0)]), terms("srss-d", "srss", [("comb", "d1", 1.0)])])
    source = solve(model)
    both(source, model, None)
    both(source, None, None)  # the published derivation states every kappa input (design section 10.3)
    changed = copy.deepcopy(source)
    set_row(changed, "srss-d", "element_local_axial_normal_stress", "midspan", 0.0, 123.0)
    for given in (None, model):
        both(changed, given, VALUE)
    for cid, reason in (("srss-cases", CE), ("srss-cases", None), ("abs-d", CE_UNSIGNED)):
        gated = copy.deepcopy(source)
        set_gate(gated, cid, reason)
        if reason is not None:
            gated["results"] = [r for r in gated["results"] if r.get("basis_ref") != {"ref_type": "combination", "ref_id": cid}]
        for given in (None, model):
            both(gated, given, DERIVATION)


def linear_cantilever(combinations):
    """The cantilever of ``constant_effort_cantilever`` without its constant-effort support: a linear model."""
    model = constant_effort_cantilever(combinations)
    model["supports"] = [s for s in model["supports"] if s["id"] != "support:ce"]
    model["project"]["id"] = "project:rt-a4-tamper-linear"
    return model


LINEAR = [terms("srss-w", "srss", [("case", "W1", 1.0), ("case", "W3", -2.0)]), sub("d1", "W1", "W2"),
          terms("abs-d", "absolute_sum", [("comb", "d1", 0.5), ("case", "W3", 1.0)]),
          terms("sus+srss", "mechanics", [("case", "W2", 1.0), ("comb", "srss-w", 1.0)])]


def derivation_record(source, cid):
    return next(r for r in source["contract_evidence"][DERIVATION_MEMBER]["combinations"] if r["combination_id"] == cid)


def test_a_linear_unsigned_document_reads_without_its_model(solve):
    """Before section 10.3, both readers refused this document without its model (``..._UNSIGNED_UNVERIFIED``)."""
    model = linear_cantilever(LINEAR)
    source = solve(model)
    derivation = source["contract_evidence"][DERIVATION_MEMBER]
    assert (derivation["nonlinear_supports"], derivation["consuming_constant_effort"]) == (False, False)
    assert [r["combination_id"] for r in derivation["combinations"]] == ["srss-w", "d1", "abs-d", "sus+srss"]
    assert derivation_record(source, "abs-d") == {"combination_id": "abs-d", "basis": "absolute_sum", "expression": {
        "terms": [{"combination_id": "d1", "factor": 0.5}, {"load_case_id": "W3", "factor": 1.0}]}}
    assert derivation_record(source, "d1")["expression"] == {"minuend_id": "W1", "subtrahend_id": "W2"}
    for given in (model, None):
        both(source, given, None)


def test_tampered_or_missing_derivation_evidence_is_refused(solve):
    model = linear_cantilever(LINEAR)
    source = solve(model)
    # A published factor: srss-w's second term.
    factor = copy.deepcopy(source)
    derivation_record(factor, "srss-w")["expression"]["terms"][1]["factor"] = -3.0
    both(factor, None, VALUE)
    both(factor, model, EVIDENCE)
    # A published operand: srss-w's first term names W2 in place of W1.
    operand = copy.deepcopy(source)
    derivation_record(operand, "srss-w")["expression"]["terms"][0]["load_case_id"] = "W2"
    both(operand, None, SOURCES)
    both(operand, model, EVIDENCE)
    # A published row: a nonzero srss-w row restated.
    row = copy.deepcopy(source)
    target = next(r for r in row["results"] if r.get("basis_ref") == {"ref_type": "combination", "ref_id": "srss-w"}
                  and r["kind"] == "element_local_shear_force_y" and r["value"] != 0.0)
    target["value"] = target["value"] * 2.0
    for given in (None, model):
        both(row, given, VALUE)
    # The model condition: a consuming constant effort stated where there is none (srss-w over load cases would then be withheld).
    condition = copy.deepcopy(source)
    condition["contract_evidence"][DERIVATION_MEMBER]["consuming_constant_effort"] = True
    both(condition, None, DERIVATION)
    both(condition, model, EVIDENCE)
    # An operand combination's record removed: its dependant's kappa input is missing.
    closure = copy.deepcopy(source)
    closure["contract_evidence"][DERIVATION_MEMBER]["combinations"] = [r for r in closure["contract_evidence"][DERIVATION_MEMBER]["combinations"] if r["combination_id"] != "d1"]
    both(closure, None, EVIDENCE)
    both(closure, model, EVIDENCE)
    # The whole member removed.
    missing = copy.deepcopy(source)
    del missing["contract_evidence"][DERIVATION_MEMBER]
    both(missing, None, UNVERIFIED_ROW)
    both(missing, model, EVIDENCE)


def test_missing_derivation_evidence_leaves_a_kappa_gate_unverifiable(solve):
    model = constant_effort_cantilever([terms("srss-cases", "srss", [("case", "W1", 1.0), ("case", "W2", 1.0)])])
    source = solve(model)
    assert source["contract_evidence"]["combination_gates"] == [{"combination_id": "srss-cases", "withheld": True, "reason": CE_UNSIGNED}]
    for given in (None, model):
        both(source, given, None)
    missing = copy.deepcopy(source)
    del missing["contract_evidence"][DERIVATION_MEMBER]
    both(missing, None, UNVERIFIED_GATE)
    both(missing, model, EVIDENCE)


def test_a_flat_document_publishes_no_derivation(solve):
    model = constant_effort_cantilever([terms("flat", "mechanics", [("case", "W1", 1.0), ("case", "W2", 0.5)]), sub("d1", "W1", "W2"),
                                        {"id": "env", "basis": "range_envelope", "operand_ids": ["W1", "W2"], "mode": "max_abs", "provenance": PROV}])
    source = solve(model)
    assert set(source["contract_evidence"]) == {"preview_cases", "combination_gates"}
    for given in (None, model):
        both(source, given, None)
    stated = copy.deepcopy(source)
    stated["contract_evidence"][DERIVATION_MEMBER] = {"nonlinear_supports": False, "consuming_constant_effort": True, "combinations": [
        {"combination_id": "d1", "basis": "result_state_subtraction", "expression": {"minuend_id": "W1", "subtrahend_id": "W2"}}]}
    both(stated, None, EVIDENCE)  # no record is an unsigned or nested combination's
    both(stated, model, EVIDENCE)


# ----------------------------------------------------------------------------- nested rows (review r2 P2)
NESTED_ROW = "SOURCE_PREVIEW_PHYSICS_COMBINATION_NESTED_ROW"
NESTED_WITHHELD = "SOURCE_PREVIEW_PHYSICS_COMBINATION_NESTED_WITHHELD"


def envelope(cid, operands):
    return {"id": cid, "basis": "range_envelope", "operand_ids": operands, "mode": "max_abs", "provenance": PROV}


def _of(row, cid):
    return row.get("basis_ref") == {"ref_type": "combination", "ref_id": cid} and row["kind"] != "combination_modulus_basis_record"


def nonzero_row(source, cid, kind):
    return next(r for r in source["results"] if _of(r, cid) and r["kind"] == kind and r["value"] != 0.0)


def ulp_up(row):
    import struct
    row["value"] = struct.unpack("<d", struct.pack("<Q", struct.unpack("<Q", struct.pack("<d", float(row["value"])))[0] + 1))[0]


def test_nested_combination_rows_are_re_derived(solve):
    """Review r2 P2: both readers re-derive every row of a mechanics combination, subtraction or envelope with a
    combination operand from its operand rows, with the model's or the derivation record's factors, bit for bit. Before
    the repair both readers admitted every tamper below, with and without the model, except (b) with it."""
    model = linear_cantilever(LINEAR + [sub("dn", "sus+srss", "W2"), envelope("en", ["srss-w", "W1"])])
    source = solve(model)
    for given in (None, model):
        both(source, given, None)
    # (a) A mechanics row with a combination operand restated.
    a = copy.deepcopy(source)
    set_row(a, "sus+srss", "element_local_shear_force_y", "midspan", 500.0, 123456.0)
    for given in (None, model):
        both(a, given, NESTED_ROW)
    # (b) Its derivation record's factor on srss-w restated, the rows unchanged.
    b = copy.deepcopy(source)
    derivation_record(b, "sus+srss")["expression"]["terms"][1]["factor"] = 99.0
    both(b, None, NESTED_ROW)
    both(b, model, EVIDENCE)
    # (c) A subtraction's, an envelope's and recomputed magnitudes' rows restated by one ulp.
    for cid, kind in (("dn", "element_local_bending_moment_z"), ("en", "element_local_bending_moment_z"), ("dn", "displacement_magnitude"),
                      ("sus+srss", "support_reaction_force_magnitude_v2"), ("en", "support_reaction_component_v2")):
        c = copy.deepcopy(source)
        ulp_up(nonzero_row(c, cid, kind))
        for given in (None, model):
            both(c, given, NESTED_ROW)
    # (d) A row with no derivable counterpart; a derivable row and a support group's magnitude missing without a stated
    # withholding; a source reference moved.
    extra = copy.deepcopy(source)
    copied = copy.deepcopy(nonzero_row(extra, "dn", "element_local_shear_force_y"))
    copied["id"] = "result:combination:dn:invented"
    extra["results"].append(copied)
    missing = copy.deepcopy(source)
    gone = nonzero_row(missing, "en", "element_local_bending_moment_z")["id"]
    missing["results"] = [r for r in missing["results"] if r["id"] != gone]
    group = copy.deepcopy(source)
    group["results"] = [r for r in group["results"] if not (_of(r, "dn") and r["kind"] == "support_reaction_moment_magnitude_v2")]
    moved = copy.deepcopy(source)
    refs = nonzero_row(moved, "sus+srss", "element_local_bending_moment_z")["source_result_refs"]
    refs[0], refs[1] = refs[1], refs[0]
    for tampered in (extra, missing, group, moved):
        for given in (None, model):
            both(tampered, given, NESTED_ROW)
    # (e) A withholding stated where every row is derivable.
    stated = copy.deepcopy(source)
    stated["diagnostics"].append({"id": "diagnostic:preview-physics:combination-row-withheld:2:dn", "code": "LOAD_COMBINATION_ROW_WITHHELD",
                                  "severity": "warning", "message": "combination dn withholds 1 combined row(s) that could not be formed: invented",
                                  "source": "core/product_physics", "affected_refs": ["dn"]})
    for given in (None, model):
        both(stated, given, NESTED_WITHHELD)


def test_nested_withholdings_are_re_derived(solve):
    """An absolute sum with a large factor withholds its overflowing rows and the anchor's support group; a dependant
    mechanics combination omits the matching rows (LOAD_COMBINATION_SOURCE_RESULT_MISSING) and withholds the group, and
    an envelope over it omits them too. Both readers admit it, with and without the model; either withholding removed is
    refused."""
    model = linear_cantilever([terms("huge", "absolute_sum", [("case", "W1", 1.0e306)]),
                               terms("sus+huge", "mechanics", [("case", "W2", 1.0), ("comb", "huge", 1.0)]),
                               envelope("env-huge", ["huge", "W1"])])
    source = solve(model)
    for given in (None, model):
        both(source, given, None)

    def named(code, cid):
        return [d for d in source["diagnostics"] if d["code"] == code and d["affected_refs"][:1] == [cid]]
    assert len(named("LOAD_COMBINATION_ROW_WITHHELD", "huge")) == 1
    assert len(named("LOAD_COMBINATION_ROW_WITHHELD", "sus+huge")) == 1
    assert named("LOAD_COMBINATION_SOURCE_RESULT_MISSING", "sus+huge")
    assert named("LOAD_COMBINATION_SOURCE_RESULT_MISSING", "env-huge")
    for code, cid in (("LOAD_COMBINATION_ROW_WITHHELD", "sus+huge"), ("LOAD_COMBINATION_SOURCE_RESULT_MISSING", "sus+huge"),
                      ("LOAD_COMBINATION_SOURCE_RESULT_MISSING", "env-huge")):
        unstated = copy.deepcopy(source)
        first = named(code, cid)[0]["id"]
        unstated["diagnostics"] = [d for d in unstated["diagnostics"] if d["id"] != first]
        for given in (None, model):
            both(unstated, given, NESTED_WITHHELD)
