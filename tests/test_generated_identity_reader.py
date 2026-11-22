"""The Python model-bound reader's generated-load identities (DEL-05-01 REQ-008, issue 452).

The shared fixture ``core/reporting/result_export/tests/fixtures/generated_identities.json`` states, for each case, the
identities of the loads the case generates, written from the generator's grammar; the product test
``core/product_physics/tests/w16_exact_equivalent_static.rs`` checks that the generator forms exactly those and that the
Rust reader rederives them. Here the Python reader (``tests/verifiers/reader_verdict.py``) must rederive the same, its
extent formatter must reproduce Rust's Display of each fixture fraction (checked against Rust by the same product test),
and its model checks must admit the identities and refuse a record that differs.
"""
from __future__ import annotations

from copy import deepcopy
import json
from pathlib import Path

import pytest

from tests.verifiers.reader_verdict import Refused, _expected_component_forces, expected_generated_load_ids, physics_model_checks, rust_f64_display

FIXTURE = json.loads((Path(__file__).resolve().parents[1] / "core/reporting/result_export/tests/fixtures/generated_identities.json").read_text())


def _case(item):
    model = FIXTURE["models"][item["model"]]
    return model, next(case for case in model["load_cases"] if case["id"] == item["load_case_id"])


@pytest.mark.parametrize("item", FIXTURE["cases"], ids=lambda item: f"{item['model']}-{item['load_case_id']}")
def test_the_reader_rederives_the_fixture_identities(item):
    model, case = _case(item)
    assert expected_generated_load_ids(model, case) == item["generated_load_ids"]


@pytest.mark.parametrize("fraction", FIXTURE["fractions"], ids=lambda fraction: fraction["display"][:24])
def test_the_extent_formatter_writes_rusts_display(fraction):
    assert rust_f64_display(fraction["value"]) == fraction["display"]


def test_a_negative_zero_is_written_as_rust_writes_it():
    assert rust_f64_display(-0.0) == "-0"


def _source(model, records):
    """A solved source naming every model case, with the members the model's connectors leave and the given records.
    A seismic case's record also carries the component entries the model forms, as the reader recomputes them
    (``_expected_component_forces``): their values are bound by tests/test_seismic_component_record_reader.py, and
    here only the identities vary. The model checks that read sections and the shear model are not reached: the
    model passed below carries neither."""
    replaced = {c["objective_connector"]["topology"]["span_ref"] for c in model.get("components", []) if "objective_connector" in c}
    members = [{"pipe_id": p["id"]} for p in model["pipe_segments"] if p["id"] not in replaced]
    cases = []
    for case in model["load_cases"]:
        entry = {"load_case_id": case["id"], "pipe_materials": members}
        if case["id"] in records:
            entry["equivalent_static"] = {"generated_load_ids": records[case["id"]]}
            generation = case.get("equivalent_static")
            seismic = generation.get("seismic") if isinstance(generation, dict) else None
            components = _expected_component_forces(model, case["id"], seismic) if seismic else []
            if components:
                entry["equivalent_static"]["component_forces"] = components
        cases.append(entry)
    return {"status": {"mechanics": "MECHANICS_SOLVED"}, "results": [],
            "contract_evidence": {"exact_cases": cases, "connector": [{"replaced_pipe_id": p} for p in sorted(replaced)]}}


@pytest.mark.parametrize("name", sorted(FIXTURE["models"]))
def test_the_model_checks_bind_the_record_to_the_model(name):
    model = deepcopy(FIXTURE["models"][name])
    model.pop("pressure_contract")
    records = {item["load_case_id"]: item["generated_load_ids"] for item in FIXTURE["cases"] if item["model"] == name}
    physics_model_checks(_source(model, records), model)
    for case_id, ids in records.items():
        for forged in (ids[1::-1] + ids[2:], ids[:-1]):
            with pytest.raises(Refused, match="SOURCE_PHYSICS_EQUIVALENT_STATIC_GENERATED_MODEL_MISMATCH"):
                physics_model_checks(_source(model, dict(records, **{case_id: forged})), model)
        missing = {key: value for key, value in records.items() if key != case_id}
        with pytest.raises(Refused, match="SOURCE_PHYSICS_MODEL_GENERATION_RECORD"):
            physics_model_checks(_source(model, missing), model)


@pytest.mark.parametrize("form", ["not-an-object", [], True], ids=["text", "list", "boolean"])
def test_a_generation_in_another_form_than_an_object_is_refused(form):
    """Round 1 review of pull request 479: a model case whose generation is neither absent, null nor an object is no
    generation the product solves; a record bound to it is refused, never left unchecked."""
    model = deepcopy(FIXTURE["models"]["frame"])
    model.pop("pressure_contract")
    records = {item["load_case_id"]: item["generated_load_ids"] for item in FIXTURE["cases"] if item["model"] == "frame"}
    source = _source(model, records)
    physics_model_checks(source, model)
    model["load_cases"][0]["equivalent_static"] = form
    with pytest.raises(Refused, match="SOURCE_PHYSICS_EQUIVALENT_STATIC_GENERATED_MODEL_MISMATCH"):
        physics_model_checks(source, model)


def test_a_member_two_components_map_is_refused():
    """Round 1 review of pull request 479: two rigid-family components mapping one member is a conflict the product
    refuses (RIGID_COMPONENT_MAPPING_CONFLICT), so no solved record of the model exists; the reader refuses it rather
    than read the member as one no component replaces."""
    model = deepcopy(FIXTURE["models"]["components"])
    model.pop("pressure_contract")
    records = {item["load_case_id"]: item["generated_load_ids"] for item in FIXTURE["cases"] if item["model"] == "components"}
    source = _source(model, records)
    physics_model_checks(source, model)
    second = deepcopy(model["components"][1])
    second["id"] = "component:second-on-b-c"
    second["node"] = model["components"][0]["node"]
    second["geometry"]["rigid_pipe_ref"] = model["components"][0]["geometry"]["rigid_pipe_ref"]
    model["components"].append(second)
    with pytest.raises(Refused, match="SOURCE_PHYSICS_EQUIVALENT_STATIC_COMPONENT_MODEL_MISMATCH"):
        physics_model_checks(source, model)


def _valve(component_id, model, pipe_ref):
    pipe = next(p for p in model["pipe_segments"] if p["id"] == pipe_ref)
    return {"id": component_id, "kind": "valve", "node": pipe["to"],
            "geometry": {"rigid_pipe_ref": pipe_ref, "weight": {"value": 2350.0, "unit": "N"},
                         "center_of_gravity": {"x": 0.0, "y": 0.0, "z": 0.0, "unit": "m"}},
            "mechanics_interface": {"solver_consumption": "mechanics_geometry_only"}, "provenance": "invented"}


@pytest.mark.parametrize("name,cases", [("frame", ["WIND-TIE"]), ("connector", None)], ids=["wind-only", "connector-replaced"])
def test_a_member_two_components_map_is_refused_on_every_document(name, cases):
    """Round 2 review of pull request 479: the product refuses a member that two rigid-family components map on every
    document (RIGID_COMPONENT_MAPPING_CONFLICT), whatever its cases generate and whether a connector replaces the
    member, so the reader refuses it at document level: on a wind-only document, and on the member the expansion
    joint replaces."""
    model = deepcopy(FIXTURE["models"][name])
    model.pop("pressure_contract")
    if cases is not None:
        model["load_cases"] = [case for case in model["load_cases"] if case["id"] in cases]
    records = {item["load_case_id"]: item["generated_load_ids"] for item in FIXTURE["cases"]
               if item["model"] == name and any(case["id"] == item["load_case_id"] for case in model["load_cases"])}
    source = _source(model, records)
    physics_model_checks(source, model)
    shared = deepcopy(model)
    shared["components"] = list(shared.get("components", [])) + [_valve("component:first-on-b-c", model, "pipe:b-c"),
                                                                   _valve("component:second-on-b-c", model, "pipe:b-c")]
    with pytest.raises(Refused, match="SOURCE_PHYSICS_EQUIVALENT_STATIC_COMPONENT_MODEL_MISMATCH"):
        physics_model_checks(source, shared)


def test_a_record_on_a_case_that_does_not_generate_is_refused_by_its_presence():
    """Round 1 review of pull request 479: a forged generation record without component entries on a case whose
    model case does not generate is refused by its presence (issue 452 decision 6), before its contents are read."""
    model = deepcopy(FIXTURE["models"]["frame"])
    model.pop("pressure_contract")
    model["load_cases"].append({"id": "PLAIN", "primitive_loads": []})
    records = {item["load_case_id"]: item["generated_load_ids"] for item in FIXTURE["cases"] if item["model"] == "frame"}
    physics_model_checks(_source(model, records), model)
    forged = dict(records, PLAIN=["PLAIN:generated:seismic:global-x:element-0"])
    with pytest.raises(Refused, match="SOURCE_PHYSICS_MODEL_GENERATION_RECORD"):
        physics_model_checks(_source(model, forged), model)
