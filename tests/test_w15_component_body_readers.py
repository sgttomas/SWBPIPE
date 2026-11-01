"""W-15 slice d (DEL-03-05 Design ``rigid-component-stiffness-scaling.md`` §5): each slice-d member is permitted on
one evidence object only, through the Python verifiers and the Rust readers, on both routes.

A stress-maximum coverage entry may carry ``component_body_pipe_ids`` and a stress extremum ``component_body``; the
other member on either object is a closed-shape refusal. Scrutiny of #1604 found both readers stripping both members
from both objects before the shape check, so a ``component_body`` on coverage and a ``component_body_pipe_ids`` on
every extremum (both exact basis twins included) were admitted. The sources are §2.4's indeterminate pair (the Rust
``core/product_physics/tests/w15_component_body.rs``), solved by the session runner; the inputs are invented.
"""
from copy import deepcopy
import json
import subprocess

import pytest
from core.analysis_runs import rust_reader
from tests.test_headless_runner_contract import final_runner_request
from tests.verifiers import component_body
from tests.verifiers.pressure_2 import pressure_table, validate_exact_pressure_evidence
from tests.verifiers.preview_physics_evidence import TABLE as PREVIEW_TABLE, validate_preview_physics_evidence

PROV = "invented W-15 slice-d test input; not library, catalog or code data"
BODY = "pipe:w15-body"
ROUTES = {
    "preview": ("preview_cases", "preview-physics", validate_preview_physics_evidence, lambda: PREVIEW_TABLE),
    "exact": ("exact_cases", "pressure", validate_exact_pressure_evidence, pressure_table),
}


def q(value, unit):
    return {"value": value, "unit": unit}


def model(exact):
    """§2.4's pair A(0) - B(1 m) - C(3 m), the body B-C a valve's mapped member at s = 4, a 3 kN force at B."""
    def pipe(pid, start, end):
        section = {"outside_diameter": q(0.12, "m"), "wall_thickness": q(0.01, "m")}
        if exact:
            section["corrosion_allowance"] = q(0.0, "m")
        return {"id": pid, "from": start, "to": end, "section": section, "material": "material:w15",
                "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0}, "provenance": PROV}

    def node(nid, x):
        return {"id": nid, "position": {"x": x, "y": 0.0, "z": 0.0}, "provenance": PROV}

    def anchor(nid):
        return {"id": f"support:{nid}", "node": nid, "family": "anchor",
                "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV}

    case = {"id": "case:w15", "primitive_loads": [{
        "id": "load:w15-fx", "category": "concentrated_force", "target": {"type": "node", "node": "node:b"},
        "direction": "global_x", "magnitude": q(3000.0, "N"), "dimension": "force", "provenance": PROV}],
        "provenance": PROV}
    out = {"schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
           "project": {"id": "project:w15-slice-d", "units": {"length": "m"}},
           "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
                               "rule_check": "not_performed_user_rule_inputs_missing",
                               "professional_acceptance": "not_provided"},
           "nodes": [node("node:a", 0.0), node("node:b", 1.0), node("node:c", 3.0)],
           "pipe_segments": [pipe("pipe:w15-pipe", "node:a", "node:b"), pipe(BODY, "node:b", "node:c")],
           "supports": [anchor("node:a"), anchor("node:c")],
           "components": [{"id": "component:w15-valve", "kind": "valve", "node": "node:b",
                           "geometry": {"rigid_pipe_ref": BODY},
                           "modifiers": {"stiffness_scaling_user_value": q(4.0, "1"),
                                         "source_reference": "invented W-15 scale"},
                           "mechanics_interface": {"solver_consumption": "mechanics_geometry_only"},
                           "provenance": PROV}],
           "combinations": []}
    if exact:
        out["pressure_contract"] = {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"}
        out["materials"] = [{"id": "material:w15", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
                             "elastic_modulus": q(200e9, "Pa"), "poisson_ratio": q(0.3, "1"), "provenance": PROV}]
        case["pressure_regions"] = []
    else:
        out["materials"] = [{"id": "material:w15", "elastic_modulus": q(200e9, "Pa"), "shear_modulus": q(80e9, "Pa"),
                             "thermal_expansion_coefficient": q(1.2e-5, "1/degC"), "provenance": PROV}]
    out["load_cases"] = [case]
    return out


@pytest.fixture(scope="module", params=sorted(ROUTES))
def solved(request):
    route = request.param
    wire = model(route == "exact")
    body = {"request": final_runner_request("solve"), "solve": {"preview_model": {"model": wire, "materials": []}}}
    completed = subprocess.run([str(rust_reader.runner_executable()), "solve", "--explicit-local-private-intent"],
                               input=json.dumps(body), capture_output=True, text=True, check=False)
    source = json.loads(completed.stdout)["payload"]["mechanics_envelope"]
    assert source["status"]["mechanics"] == "MECHANICS_SOLVED", source["diagnostics"]
    assert any("component_body" in row for row in source["results"]), "the body carries its record"
    return route, source, wire


def _cases(route, source):
    return source["contract_evidence"][ROUTES[route][0]]


def _coverage(case):
    entries = case["stress_maximum_coverage"]
    return entries if isinstance(entries, list) else [entries]


def _models(route, wire):
    """The Rust reader's verdicts: model-free, and model-bound where the runner's base reader takes a model (the
    pressure reader; the preview-physics one is model-free, and its model-bound verdict is the Rust test's)."""
    return [None, wire] if route == "exact" else [None]


def record_on_coverage(route, source):
    for case in _cases(route, source):
        for entry in _coverage(case):
            assert "component_body_pipe_ids" in entry
            entry["component_body"] = "garbage"


def body_list_on_every_extremum(route, source):
    extrema = [x for case in _cases(route, source) for x in case["pipe_stress_extrema"]]
    if route == "exact":
        assert {x["section_basis"] for x in extrema if x["pipe_id"] == BODY} == {"nominal_section", "reduced_section"}
    for extremum in extrema:
        extremum["component_body_pipe_ids"] = ["nonexistent"]


def _row(source, entity, kind):
    return next(row for row in source["results"] if row["entity_ref"] == entity and row["kind"] == kind)


def body_list_on_a_body_stress_row(route, source):
    row = _row(source, BODY, "element_local_axial_normal_stress")
    assert "component_body" in row
    row["component_body_pipe_ids"] = ["nonexistent"]


def body_list_on_the_body_maximum_row(route, source):
    row = _row(source, BODY, MAXIMUM)
    assert "component_body" in row
    row["component_body_pipe_ids"] = ["nonexistent"]


def body_list_on_a_pipe_stress_row(route, source):
    _row(source, "pipe:w15-pipe", "element_local_axial_normal_stress")["component_body_pipe_ids"] = ["nonexistent"]


MAXIMUM = "pipe_elastic_normal_stress_maximum_v2"
# (mutation, the Python verifier's refusal, the Rust reader's code): the closed-shape refusals of coverage and
# extrema ("maximum coverage shape"/"extrema shape" on preview, "maximum coverage" per section basis and "extrema
# shape" on exact), and the body list on a result row (round-2 scrutiny of #1604), which the source-bodies check
# refuses by name.
MUTATIONS = [
    (record_on_coverage, "maximum coverage", "SHAPE"),
    (body_list_on_every_extremum, "extrema shape", "SHAPE"),
    (body_list_on_a_body_stress_row, "misplaced body list", "SOURCE_COMPONENT_BODY_MISPLACED"),
    (body_list_on_the_body_maximum_row, "misplaced body list", "SOURCE_COMPONENT_BODY_MISPLACED"),
    (body_list_on_a_pipe_stress_row, "misplaced body list", "SOURCE_COMPONENT_BODY_MISPLACED"),
]
ROW_MUTATIONS = {body_list_on_a_body_stress_row, body_list_on_the_body_maximum_row, body_list_on_a_pipe_stress_row}


def test_w15_readers_admit_the_body_source(solved):
    route, source, wire = solved
    _, reader, verify, table = ROUTES[route]
    verify(deepcopy(source))
    component_body.model_checks(deepcopy(source), wire, table())
    for bound in _models(route, wire):
        assert rust_reader.validate(reader, deepcopy(source), model=bound)["ok"] is True
    assert rust_reader.dispatch(deepcopy(source), wire)["standing"]["eligible"] is True


@pytest.mark.parametrize("mutate,python,rust", MUTATIONS, ids=[m.__name__ for m, _, _ in MUTATIONS])
def test_w15_a_slice_d_member_on_another_object_is_refused(solved, mutate, python, rust):
    """Each slice-d member on an object it does not belong to is refused: by the Python verifier and, for a
    result row, its model-bound check; by the Rust reader model-free and model-bound (the pressure reader takes the
    model; the model-bound dispatch's standing is not eligible on either route)."""
    route, source, wire = solved
    _, reader, verify, table = ROUTES[route]
    mutated = deepcopy(source)
    mutate(route, mutated)
    with pytest.raises(ValueError, match=python):
        verify(deepcopy(mutated))
    if mutate in ROW_MUTATIONS:
        with pytest.raises(ValueError, match=python):
            component_body.model_checks(deepcopy(mutated), wire, table())
    for bound in _models(route, wire):
        with pytest.raises(rust_reader.ReaderRefusal) as refusal:
            rust_reader.validate(reader, deepcopy(mutated), model=bound)
        assert rust in refusal.value.code, (refusal.value.code, refusal.value.detail)
    standing = rust_reader.dispatch(deepcopy(mutated), wire)["standing"]
    assert standing["eligible"] is False and standing["token"] != "numerically_eligible", standing
