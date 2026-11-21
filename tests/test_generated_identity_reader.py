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
            seismic = case["equivalent_static"].get("seismic")
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
