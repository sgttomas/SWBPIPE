"""Model document 0.9.0's members in the schema (DEL-02-01 Design `model-document-0.9.0.md` §3.4).

A document carrying every member the 0.9.0 step adds (condition sets, the pipe's set reference, a case's
condition use and test condition, a support's direction and the snubber family, a combination's
difference basis and subtrahend support state, the absolute-sum and SRSS bases, a combination operand
and spectra) validates against the format root, and each record stays closed. The document is the
bundled demo at its own (current) version, so the version step carries it.
"""
from __future__ import annotations

import copy
import json
import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "tests"))

from schema_validation import JsonSchemaDependencyMissing, _skip_or_note_missing_jsonschema, validate_instance  # noqa: E402

SCHEMA_PATH = ROOT / "schemas" / "model.schema.yaml"
DEMO = ROOT / "fixtures" / "product_preview" / "invented_demo_model.json"


def schema():
    return json.loads(SCHEMA_PATH.read_text(encoding="utf-8"))


def every_member_document():
    document = json.loads(DEMO.read_text(encoding="utf-8"))
    assert document["schema_version"] in schema()["properties"]["schema_version"]["enum"]
    document["condition_sets"] = [{
        "id": "cs:line-100", "label": "Line 100", "provenance": "invented_example_user_input",
        "conditions": [
            {"key": "OPE1", "label": "Operating", "temperature": {"value": 150.0, "unit": "degC"},
             "pressure": {"value": 1.5, "unit": "MPa"}, "contents_density": {"value": 998.0, "unit": "kg/m^3"}},
            {"key": "HYDRO", "temperature": {"value": 20.0, "unit": "degC"}},
        ],
        "installation_temperature": {"value": 20.0, "unit": "degC"},
        "design": {"temperature": {"value": 180.0, "unit": "degC"}, "pressure": {"value": 2.0, "unit": "MPa"}},
        "peak": {"temperature": {"value": 200.0, "unit": "degC"}},
        "added_weight": {"magnitude": {"value": -50.0, "unit": "N/m"}, "direction": "global_z"},
        "exposure": {"wind": True, "snow_ice": False, "wave": False},
        "tags": ["line-100"], "notes": "invented",
    }]
    document["spectra"] = [
        {"id": "spectrum:g", "label": "Invented g spectrum", "abscissa": "frequency", "ordinate": "g_multiple",
         "units": {"abscissa": "Hz"}, "g_value": {"value": 9.80665, "unit": "m/s^2"}, "damping_ratio": 0.05,
         "interpolation": {"abscissa": "log", "ordinate": "log"}, "points": [[1.0, 0.2], [10.0, 0.5]],
         "tags": ["dynamics"], "notes": "invented"},
        {"id": "spectrum:a", "abscissa": "period", "ordinate": "acceleration",
         "units": {"abscissa": "s", "ordinate": "m/s^2"}, "damping_ratio": 0.0,
         "interpolation": {"abscissa": "linear", "ordinate": "linear"}, "points": [[0.1, 1.0], [1.0, 2.0]]},
    ]
    document["pipe_segments"][0]["condition_set_ref"] = "cs:line-100"
    case = document["load_cases"][0]
    case["condition_use"] = {"temperature": "OPE1", "added_weight": True, "wind_exposure": True}
    document["load_cases"][1]["test_condition"] = {"kind": "hydrostatic", "held_supports": "springs_and_constant_effort"}
    document["supports"].append({"id": "support:SN-1", "label": "Snubber", "node": document["nodes"][-1]["id"],
                                 "family": "snubber", "restraints": ["UY"], "direction": {"x": 0.0, "y": 3.0, "z": 4.0},
                                 "stiffness": {"dof": "UY", "value": {"value": 1.0e7, "unit": "N/m"}},
                                 "provenance": "invented_example"})
    first, second = (case["id"] for case in document["load_cases"])
    document["combinations"] += [
        {"id": "combination:EXP", "basis": "result_state_subtraction", "minuend_id": first, "subtrahend_id": second,
         "difference_basis": "operating_minus_sustained", "subtrahend_support_state": "minuend_final"},
        {"id": "combination:ABS", "basis": "absolute_sum",
         "terms": [{"load_case": first, "factor": 1.0}, {"combination": "combination:EXP", "factor": 1.0}]},
        {"id": "combination:SRSS", "basis": "srss", "terms": [{"combination": "combination:ABS", "factor": 1.0}]},
    ]
    return document


def _valid(document, label):
    validate_instance(schema(), document, schema_label=str(SCHEMA_PATH), instance_label=label)


def _invalid(document, label):
    with pytest.raises(AssertionError, match="failed JSON Schema validation"):
        _valid(document, label)


def test_a_document_carrying_every_0_9_0_member_validates():
    try:
        _valid(every_member_document(), "every 0.9.0 member")
    except JsonSchemaDependencyMissing as exc:
        _skip_or_note_missing_jsonschema(exc)


def test_the_new_records_are_closed_and_an_authored_null_is_not_absence():
    defs = schema()["$defs"]
    for name in ("ConditionSet", "OperatingCondition", "ConditionValues", "AddedWeight", "ConditionExposure",
                 "ConditionUse", "TestCondition", "Spectrum"):
        assert defs[name]["additionalProperties"] is False, name
    assert defs["Spectrum"]["properties"]["units"]["additionalProperties"] is False
    assert defs["Spectrum"]["properties"]["interpolation"]["additionalProperties"] is False
    assert "snubber" in defs["SupportFamily"]["enum"]
    assert {"absolute_sum", "srss"} <= set(defs["Combination"]["properties"]["basis"]["enum"])
    try:
        mutations = [
            ("extra condition-set member", lambda d: d["condition_sets"][0].update(extra=1)),
            ("null condition_set_ref", lambda d: d["pipe_segments"][0].update(condition_set_ref=None)),
            ("null direction", lambda d: d["supports"][-1].update(direction=None)),
            ("direction member", lambda d: d["supports"][-1]["direction"].update(w=1)),
            ("null difference_basis", lambda d: d["combinations"][1].update(difference_basis=None)),
            ("unknown difference_basis", lambda d: d["combinations"][1].update(difference_basis="sustained")),
            ("unknown subtrahend_support_state", lambda d: d["combinations"][1].update(subtrahend_support_state="final")),
            ("empty condition_use", lambda d: d["load_cases"][0].update(condition_use={})),
            ("false added_weight", lambda d: d["load_cases"][0]["condition_use"].update(added_weight=False)),
            ("term with both operands", lambda d: d["combinations"][2]["terms"][0].update(combination="combination:EXP")),
            ("term with neither operand", lambda d: d["combinations"][2]["terms"][0].pop("load_case")),
            ("test condition missing held_supports", lambda d: d["load_cases"][1]["test_condition"].pop("held_supports")),
            ("g_multiple without g_value", lambda d: d["spectra"][0].pop("g_value")),
            ("g_value on acceleration", lambda d: d["spectra"][1].update(g_value={"value": 9.8, "unit": "m/s^2"})),
            ("acceleration without units.ordinate", lambda d: d["spectra"][1]["units"].pop("ordinate")),
            ("units.ordinate on g_multiple", lambda d: d["spectra"][0]["units"].update(ordinate="m/s^2")),
            ("one point", lambda d: d["spectra"][0].update(points=[[1.0, 0.2]])),
            ("a point of three", lambda d: d["spectra"][0]["points"][0].append(1.0)),
            ("null spectrum label", lambda d: d["spectra"][0].update(label=None)),
        ]
        for label, mutate in mutations:
            document = every_member_document()
            mutate(document)
            _invalid(document, label)
    except JsonSchemaDependencyMissing as exc:
        _skip_or_note_missing_jsonschema(exc)


def test_a_region_omits_pressure_only_where_the_case_uses_condition_pressure():
    contract = json.loads((ROOT / "schemas" / "model.schema.yaml").read_text(encoding="utf-8"))["$defs"]["LoadCase"]
    assert "if" in contract and "then" in contract
    document = every_member_document()
    region = {"id": "region:1", "member_pipe_ids": [document["pipe_segments"][0]["id"]],
              "pressure_basis": "internal_differential_zero_external_v1", "terminals": [], "provenance": "invented"}
    try:
        case = document["load_cases"][0]
        case["pressure_regions"] = [copy.deepcopy(region)]
        _invalid(document, "region without pressure, no condition pressure")
        case["condition_use"]["pressure"] = "OPE1"
        _valid(document, "region without pressure under condition_use.pressure")
        case["pressure_regions"][0]["pressure"] = {"value": 1.0, "unit": "MPa"}
        _valid(document, "region with pressure (the exclusivity is the solve's refusal)")
    except JsonSchemaDependencyMissing as exc:
        _skip_or_note_missing_jsonschema(exc)
