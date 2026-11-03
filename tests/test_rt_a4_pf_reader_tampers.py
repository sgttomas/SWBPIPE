"""RT-A4 review r1 P2-P3 (DEL-05-02 ``absolute-sum-srss-combinations.md`` sections 4-5, 10): preview-physics-2's
combination gates and unsigned rows are re-derived by both readers, and the runner's ``read preview-physics`` takes the
model.

Each envelope is solved by the headless preview runner from an invented model written here; each tamper is read through
the actual CLI (``openpipestress-runner read preview-physics`` via ``core/analysis_runs/rust_reader.py``) and by the
independent Python mirror (``tests/verifiers/reader_verdict.py``), with and without the model. Both name the same code.
Before the fix the CLI refused ``{source, model}`` as an unsupported member and both readers admitted every tamper.
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
    both(source, None, UNVERIFIED_GATE)  # the kappa withholding of srss-cases cannot be verified without the model
    changed = copy.deepcopy(source)
    set_row(changed, "srss-d", "element_local_axial_normal_stress", "midspan", 0.0, 123.0)
    both(changed, model, VALUE)
    for cid, reason in (("srss-cases", CE), ("srss-cases", None), ("abs-d", CE_UNSIGNED)):
        gated = copy.deepcopy(source)
        set_gate(gated, cid, reason)
        if reason is not None:
            gated["results"] = [r for r in gated["results"] if r.get("basis_ref") != {"ref_type": "combination", "ref_id": cid}]
        both(gated, model, DERIVATION)


def test_a_linear_unsigned_document_without_its_model_is_unverifiable(solve):
    model = constant_effort_cantilever([sub("d1", "W1", "W2"), terms("srss-d", "srss", [("comb", "d1", 1.0)])])
    source = solve(model)
    both(source, model, None)
    both(source, None, UNVERIFIED_ROW)
