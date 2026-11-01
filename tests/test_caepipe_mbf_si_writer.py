#!/usr/bin/env python3
"""Focused tests for the DEL-17-04 SI CAEPIPE MBF pilot writer (invented models only)."""

from __future__ import annotations

from copy import deepcopy
from fractions import Fraction
import hashlib
import json
import math
from pathlib import Path
import sys

import pytest


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.handoff.caepipe_mbf import (  # noqa: E402
    MBF_SI_PROFILE_ID,
    STANDARD_GRAVITY,
    MbfSiResult,
    render_caepipe_mbf_si,
)
from core.handoff.caepipe_mbf.si_writer import (  # noqa: E402
    IMPORTED_PROJECT_UNITS, MODEL_SCHEMA_PATH, current_model_schema_version)


SIX = ["UX", "UY", "UZ", "RX", "RY", "RZ"]


def q(value, unit):
    return {"value": value, "unit": unit}


def model_shell(length_unit="m"):
    return {
        "schema_version": "0.9.0",
        "document_kind": "openpipestress.product_preview.model",
        "analysis_status": {"mechanics": "invented", "rule_check": "not_performed", "professional_acceptance": "not_provided"},
        "project": {"id": "invented:mbf-si", "units": {"length": length_unit, "force": "N", "temperature": "degC"}},
        "nodes": [],
        "pipe_segments": [],
        "materials": [
            {
                "id": "mat:CS",
                "elastic_modulus": q(2.0e11, "Pa"),
                "shear_modulus": q(8.0e10, "Pa"),
                "thermal_expansion_coefficient": q(1.2e-5, "1/K"),
            }
        ],
        "supports": [],
        "load_cases": [],
        "combinations": [],
    }


def node(node_id, x, y=0.0, z=0.0):
    return {"id": node_id, "position": {"x": x, "y": y, "z": z}}


def pipe(pipe_id, start, end, od, wall, unit):
    return {
        "id": pipe_id,
        "from": start,
        "to": end,
        "material": "mat:CS",
        "section": {"outside_diameter": q(od, unit), "wall_thickness": q(wall, unit)},
    }


def force(load_id, node_id, direction, value, unit="N"):
    return {"id": load_id, "category": "concentrated_force", "target": {"type": "node", "node": node_id},
            "direction": direction, "magnitude": q(value, unit), "dimension": "force"}


def cantilever(length_unit="m"):
    scale = 1000.0 if length_unit == "mm" else 1.0
    model = model_shell(length_unit)
    model["nodes"] = [node("N0", 0.0), node("N1", 3.0 * scale)]
    model["pipe_segments"] = [pipe("S1", "N0", "N1", 0.1683 * scale, 0.00711 * scale, length_unit)]
    model["supports"] = [{"id": "anchor:N0", "node": "N0", "restraints": list(SIX)}]
    model["load_cases"] = [{"id": "case:tip", "primitive_loads": [force("tip", "N1", "global_y", -6000.0)]}]
    return model


def branched():
    model = model_shell("mm")
    model["materials"][0] = {
        "id": "mat:CS",
        "elastic_modulus": q(200000.0, "MPa"),
        "poisson_ratio": q(0.3, "1"),
        "thermal_expansion_coefficient": q(1.2e-5, "1/degC"),
    }
    model["nodes"] = [node("N1", 0.0), node("N2", 3000.0), node("N3", 6000.0), node("N4", 3000.0, 2000.0)]
    model["pipe_segments"] = [
        pipe("P1", "N1", "N2", 168.3, 7.11, "mm"),
        pipe("P2", "N2", "N3", 168.3, 7.11, "mm"),
        pipe("P3", "N2", "N4", 168.3, 7.11, "mm"),
    ]
    model["supports"] = [
        {"id": "A1", "node": "N1", "restraints": list(SIX)},
        {"id": "A3", "node": "N3", "restraints": list(SIX)},
        {"id": "R4", "node": "N4", "restraints": ["UY"]},
    ]
    model["load_cases"] = [
        {"id": "case:sus", "primitive_loads": [
            force("f2", "N2", "global_y", -2500.0),
            {"id": "m4", "category": "concentrated_moment", "target": {"type": "node", "node": "N4"},
             "direction": "RZ", "magnitude": q(1.0, "kN*m"), "dimension": "moment"},
        ]},
        {"id": "case:th", "primitive_loads": [
            {"id": "t1", "category": "thermal", "target": {"type": "element", "pipe": "P1"},
             "direction": "global_x", "magnitude": q(80.0, "degC"), "dimension": "temperature_interval"},
            {"id": "t2", "category": "thermal", "target": {"type": "element", "pipe": "P2"},
             "direction": "global_x", "magnitude": q(80.0, "K"), "dimension": "temperature_interval"},
            {"id": "d3", "category": "imposed_displacement", "target": {"type": "support", "support": "A3", "dof": "UY"},
             "direction": "UY", "magnitude": q(0.005, "m"), "dimension": "displacement"},
            {"id": "r3", "category": "imposed_displacement", "target": {"type": "support", "support": "A3", "dof": "RZ"},
             "direction": "RZ", "magnitude": q(0.01, "rad"), "dimension": "rotation"},
        ]},
    ]
    model["combinations"] = [{"id": "combo:ope", "basis": "mechanics",
                              "terms": [{"load_case": "case:sus", "factor": 1.0}, {"load_case": "case:th", "factor": 1.0}]}]
    return model


def crlf(*lines):
    return "".join(line + "\r\n" for line in lines)


def codes(result: MbfSiResult, severity="error"):
    return {item["code"] for item in result.diagnostics if item["severity"] == severity}


def assert_refused(result: MbfSiResult, code: str, *fragments: str):
    assert result.status == "refused"
    assert result.text is None
    matching = [item for item in result.diagnostics if item["code"] == code and item["severity"] == "error"]
    assert matching, result.diagnostics
    text = " ".join(item["entity"] + " " + item["message"] for item in matching)
    for fragment in fragments:
        assert fragment in text
    if code not in {"MBF-SI-VALUE-OUT-OF-RANGE", "MBF-SI-INPUT-INVALID"}:
        assert any(item["code"] == code for item in result.loss_report)


def test_cantilever_matches_hand_written_mbf():
    result = render_caepipe_mbf_si(cantilever(), title="INVENTED CANTILEVER")
    assert result.status == "written", result.diagnostics
    assert result.text == crlf(
        "INVENTED CANTILEVER",
        "VERSION",
        "15.10",
        "OPTIONS",
        "HGRA,SI",
        "OPTEMP",
        "20",
        "MATERIAL",
        "M1,0,0.25,1,1,CS,mat:CS",
        "M1,20,200000,1.2e-05",
        "M1,120,200000,1.2e-05",
        "PIPE",
        "P1,168.3,7.11,0,0,0,0,0,0",
        "LOADS",
        "L1,20,0,0",
        "LAYOUT",
        "F10,KA,MM1,PP1,CL=L1",
        "T20,X3000,CFOR(0,-6000,0)",
    )
    # No piping code was chosen: none is written, and that is stated, not silent. The field sweep (Design §17.1)
    # lists what the MBF does not keep; nothing else is diagnosed.
    assert sorted((item["severity"], item["code"], item["entity"]) for item in result.diagnostics) == sorted([
        ("info", "MBF-SI-PIPING-CODE-NOT-WRITTEN", next(item["entity"] for item in result.diagnostics
                                                         if item["code"] == "MBF-SI-PIPING-CODE-NOT-WRITTEN")),
        ("warning", "MBF-SI-MEMBER-NOT-EXPORTED", "model.analysis_status"),
        ("warning", "MBF-SI-FIELD-NOT-EXPORTED", "model.project.id"),
        ("warning", "MBF-SI-FIELD-NOT-EXPORTED", "model.project.units"),
    ])
    assert result.node_map == {"N0": "10", "N1": "20"}
    assert result.element_map["S1"]["from"] == "10"
    assert result.element_map["S1"]["to"] == "20"
    assert result.element_map["S1"]["layout_order"] == 0
    assert result.support_map["anchor:N0"]["node"] == "10"
    assert result.support_map["anchor:N0"]["caepipe_kind"] == "anchor"
    assert result.load_case_map == {"case:tip": "Sustained (W+P)"}
    assert result.layout_row_count == 3
    assert result.text.isascii()
    as_dict = result.to_dict()
    assert as_dict["profile_id"] == MBF_SI_PROFILE_ID
    assert as_dict["status"] == "written"


_SWEEP_CODES = {"MBF-SI-MEMBER-NOT-EXPORTED", "MBF-SI-FIELD-NOT-EXPORTED"}


def test_a_chosen_b31_piping_code_is_written_with_b31j_off():
    default = render_caepipe_mbf_si(cantilever(), title="INVENTED CANTILEVER")
    result = render_caepipe_mbf_si(cantilever(), title="INVENTED CANTILEVER", piping_code="B311-80")
    assert result.status == "written", result.diagnostics
    lines = result.text.split("\r\n")
    assert lines[3:9] == ["OPTIONS", "HGRA,B311-80,SI", "OPCODE", "0,0,0", "OPTEMP", "20"]
    # Only the OPTIONS token and the OPCODE section differ from the text written without a code.
    assert result.text.replace("HGRA,B311-80,SI\r\nOPCODE\r\n0,0,0\r\n", "HGRA,SI\r\n") == default.text
    # Only the field sweep's omitted entries (Design §17.1) are diagnosed.
    assert {item["code"] for item in result.diagnostics} <= _SWEEP_CODES
    assert any(note.startswith("Piping code B311-80, chosen by the caller") and "B31J off" in note
               for note in result.assumptions)
    assert result.node_map == default.node_map and result.element_map == default.element_map


def test_without_a_piping_code_every_pilot_mbf_is_unchanged_and_a_code_changes_only_options_and_opcode():
    cases = sorted((ROOT / "validation" / "caepipe_correlation" / "cases").glob("*.json"))
    assert cases
    for path in cases:
        case = json.loads(path.read_text(encoding="utf-8"))
        arguments = {"title": case.get("title", case["case_id"]),
                     "reference_temperature_c": case.get("reference_temperature_c", 20.0)}
        default = render_caepipe_mbf_si(case["model"], **arguments)
        assert render_caepipe_mbf_si(case["model"], **arguments, piping_code=None).text == default.text
        assert "MBF-SI-PIPING-CODE-NOT-WRITTEN" in {item["code"] for item in default.diagnostics}
        assert "\r\nOPTIONS\r\nHGRA,SI\r\nOPTEMP\r\n" in default.text
        chosen = render_caepipe_mbf_si(case["model"], **arguments, piping_code="B311-80")
        assert chosen.text.replace("HGRA,B311-80,SI\r\nOPCODE\r\n0,0,0\r\n", "HGRA,SI\r\n") == default.text
        assert chosen.sidecar("x") == default.sidecar("x")


def test_a_non_b31_piping_code_is_written_without_opcode():
    result = render_caepipe_mbf_si(cantilever(), title="INVENTED CANTILEVER", piping_code="EUROPEAN")
    assert result.status == "written", result.diagnostics
    assert "\r\nOPTIONS\r\nHGRA,EUROPEAN,SI\r\nOPTEMP\r\n" in result.text
    assert "OPCODE" not in result.text


def test_a_code_that_locks_b31j_is_written_with_a_warning():
    result = render_caepipe_mbf_si(cantilever(), title="INVENTED CANTILEVER", piping_code="B311")
    assert result.status == "written", result.diagnostics
    assert "HGRA,B311,SI\r\nOPCODE\r\n0,0,0\r\n" in result.text
    assert codes(result, "warning") - _SWEEP_CODES == {"MBF-SI-PIPING-CODE-B31J-LOCKED"}


@pytest.mark.parametrize("code", ["NONE", "B31.1", "b311", "", 311])
def test_an_undocumented_piping_code_is_refused(code):
    assert_refused(render_caepipe_mbf_si(cantilever(), title="X", piping_code=code), "MBF-SI-PIPING-CODE-UNSUPPORTED")


def test_a_written_piping_code_survives_the_bound_reimport():
    from hashlib import sha256
    from core.handoff.caepipe_mbf.importer import import_caepipe_mbf

    result = render_caepipe_mbf_si(branched(), title="INVENTED BRANCH", piping_code="B313")
    assert result.status == "written", result.diagnostics
    data = result.text.encode("ascii")
    back = import_caepipe_mbf(data, sidecar=result.sidecar(sha256(data).hexdigest()), source_label="test",
                              material_provenance={"M1": {"origin": "project_invented", "source_name": "test"}})
    assert back.status == "imported", back.diagnostics
    assert back.target_options["OPTIONS.piping_code"] == ["B313"]
    assert back.target_options["OPCODE"] == {"fields": ["0", "0", "0"]}


def test_branched_model_with_restraint_moment_thermal_and_imposed_displacement():
    result = render_caepipe_mbf_si(branched(), title="INVENTED BRANCH")
    assert result.status == "written", result.diagnostics
    # 0.01 rad = 0.5729577951 deg to ten significant digits.
    assert result.text == crlf(
        "INVENTED BRANCH",
        "VERSION",
        "15.10",
        "OPTIONS",
        "HGRA,SI",
        "OPTEMP",
        "20",
        "MATERIAL",
        "M1,0,0.3,1,1,CS,mat:CS",
        "M1,20,200000,1.2e-05",
        "M1,120,200000,1.2e-05",
        "PIPE",
        "P1,168.3,7.11,0,0,0,0,0,0",
        "LOADS",
        "L1,100,0,0",
        "L2,20,0,0",
        "LAYOUT",
        "F10,KA,MM1,PP1,CL=L1",
        "T20,X3000,CFOR(0,-2500,0)",
        "T30,KA,X3000,CD(0,5,0),ROT(0,0,0.5729577951)",
        "F20,X3000,Y0,Z0",
        "T40,Y2000,CL=L2,FIXD(0,1,0),MOM(0,0,1000)",
    )
    assert result.node_map == {"N1": "10", "N2": "20", "N3": "30", "N4": "40"}
    assert result.element_map["P3"]["from"] == "20" and result.element_map["P3"]["to"] == "40"
    assert result.element_map["P3"]["layout_order"] == 2
    assert result.support_map["R4"]["caepipe_kind"] == "restraint"
    assert result.load_case_map == {"case:sus": "Sustained (W+P)", "case:th": "Expansion (T1)"}
    assert result.load_map["case:th/r3"]["mbf"] == "ROT"
    assert codes(result, "warning") == {"MBF-SI-COMBINATION-NOT-EXPORTED", "MBF-SI-MEMBER-NOT-EXPORTED",
                                        "MBF-SI-FIELD-NOT-EXPORTED"}
    assert [item["entity"] for item in result.loss_report] == ["combination:combo:ope", "model.analysis_status",
                                                               "model.project.id"]
    assert any("rad are written in degrees" in note for note in result.assumptions)


def test_weight_maps_to_material_weight_density():
    model = cantilever()
    od, wall = 0.1683, 0.00711
    area = math.pi / 4.0 * (od**2 - (od - 2 * wall) ** 2)
    weight = -area * 7850.0 * STANDARD_GRAVITY
    model["nodes"].append(node("N2", 6.0))
    model["pipe_segments"].append(pipe("S2", "N1", "N2", od, wall, "m"))
    model["load_cases"][0]["primitive_loads"].append(
        {"id": "w1", "category": "weight", "target": {"type": "element", "pipe": "S1"},
         "direction": "global_y", "magnitude": q(weight, "N/m"), "dimension": "force_per_length"}
    )
    result = render_caepipe_mbf_si(model, title="WEIGHT")
    assert result.status == "written", result.diagnostics
    density = result.material_map["M1"]["weight_density_kgf_per_m3"]
    assert abs(density - 7850.0) <= 1e-9 * 7850.0
    assert result.material_map["M2"]["weight_density_kgf_per_m3"] == 0.0
    assert result.element_map["S1"]["material"] == "M1"
    assert result.element_map["S2"]["material"] == "M2"
    lines = result.text.split("\r\n")
    assert "M1,7850,0.25,1,1,CS,mat:CS" in lines
    assert "M2,0,0.25,1,1,CS,mat:CS" in lines
    assert "T30,MM2,X3000" in lines
    assert any(str(STANDARD_GRAVITY) in note for note in result.assumptions)


def test_weight_along_other_than_minus_y_is_refused():
    model = cantilever()
    model["load_cases"][0]["primitive_loads"].append(
        {"id": "w1", "category": "weight", "target": {"type": "element", "pipe": "S1"},
         "direction": "global_z", "magnitude": q(-100.0, "N/m"), "dimension": "force_per_length"}
    )
    assert_refused(render_caepipe_mbf_si(model, title="W"), "MBF-SI-LOAD-UNSUPPORTED", "global_z")


def test_poisson_ratio_derived_from_e_and_g_and_range_checked():
    result = render_caepipe_mbf_si(cantilever(), title="NU")
    assert result.material_map["M1"]["poisson_ratio"] == 0.25
    assert any("E/(2G) - 1 = 0.25" in note for note in result.assumptions)

    model = cantilever()
    model["materials"][0]["shear_modulus"] = q(2.0e11, "Pa")  # nu = -0.5
    assert_refused(render_caepipe_mbf_si(model, title="NU"), "MBF-SI-VALUE-OUT-OF-RANGE", "outside (0, 0.5)")


def test_same_input_gives_identical_output():
    first = render_caepipe_mbf_si(branched(), title="DET").to_dict()
    second = render_caepipe_mbf_si(branched(), title="DET").to_dict()
    assert first == second


def test_mm_project_writes_the_same_text_as_metre_project():
    metres = render_caepipe_mbf_si(cantilever("m"), title="SAME")
    millimetres = render_caepipe_mbf_si(cantilever("mm"), title="SAME")
    assert metres.status == millimetres.status == "written"
    assert metres.text == millimetres.text
    assert metres.element_map == millimetres.element_map


def us_customary_cantilever():
    """Invented US-customary cantilever: 120 in along X, OD 6.625 in, t 0.28 in, E 29e6 psi, nu 0.3."""
    model = model_shell("in")
    model["project"]["units"].update(force="lbf", temperature="degF")
    model["materials"][0] = {"id": "mat:US", "elastic_modulus": q(29.0e6, "psi"), "poisson_ratio": q(0.3, "1"),
                             "thermal_expansion_coefficient": q(6.5e-6, "1/degF")}
    model["nodes"] = [node("N0", 0.0), node("N1", 120.0)]
    model["pipe_segments"] = [pipe("S1", "N0", "N1", 6.625, 0.28, "in")]
    model["pipe_segments"][0]["material"] = "mat:US"
    model["supports"] = [{"id": "anchor:N0", "node": "N0", "restraints": list(SIX)}]
    model["load_cases"] = [
        {"id": "case:sus", "primitive_loads": [
            force("tip", "N1", "global_y", -1000.0, "lbf"),
            {"id": "m", "category": "concentrated_moment", "target": {"type": "node", "node": "N1"}, "direction": "RZ",
             "magnitude": q(100.0, "lbf*ft"), "dimension": "moment"}]},
        {"id": "case:th", "primitive_loads": [
            {"id": "dt", "category": "thermal", "target": {"type": "element", "pipe": "S1"}, "direction": "global_x",
             "magnitude": q(180.0, "degF"), "dimension": "temperature_interval"}]},
    ]
    return model


def test_us_customary_model_is_normalized_by_exact_definitions():
    # Hand-derived (Design section 5.2, O-2): 120 in x 25.4 = 3048 mm; 6.625 in = 168.275 mm; 0.28 in = 7.112 mm;
    # 29e6 psi x 4.4482216152605 N / 645.16 mm^2 = 199947.9615 N/mm2; 6.5e-6 /degF x 9/5 = 1.17e-5 /degC;
    # 1000 lbf = 4448.2216152605 N; 100 lbf*ft = 100 x 4.4482216152605 x 0.3048 = 135.58179483314 N*m;
    # 180 degF interval x 5/9 = 100 C; reference 70 degF = (70 - 32) x 5/9 = 21.11111111 C.
    result = render_caepipe_mbf_si(us_customary_cantilever(), title="US", reference_temperature_f=70.0)
    assert result.status == "written", result.diagnostics
    assert result.text == crlf(
        "US", "VERSION", "15.10", "OPTIONS", "HGRA,SI", "OPTEMP", "21.11111111", "MATERIAL",
        "M1,0,0.3,1,1,CS,mat:US", "M1,21.11111111,199947.9615,1.17e-05", "M1,121.1111111,199947.9615,1.17e-05",
        "PIPE", "P1,168.275,7.112,0,0,0,0,0,0", "LOADS", "L1,121.1111111,0,0",
        "LAYOUT", "F10,KA,MM1,PP1,CL=L1", "T20,X3048,CFOR(0,-4448.221615,0),MOM(0,0,135.5817948)",
    )
    normalized = [item for item in result.diagnostics if item["code"] == "MBF-SI-UNITS-NORMALIZED"]
    assert len(normalized) == 1 and normalized[0]["severity"] == "info"
    for definition in ("1 in = 25.4 mm", "1 lbf = 0.45359237 kg x 9.80665 m/s^2", "1 psi = 1 lbf/in^2",
                       "1 lbf*ft = 4.4482216152605 N x 0.3048 m", "1/degF = 9/5 per degC",
                       "a degF interval = 5/9 degC", "T(degC) = (T(degF) - 32) x 5/9"):
        assert definition in normalized[0]["message"]
    assert any("exact definitions" in note for note in result.assumptions)


@pytest.mark.parametrize("unit, value, expected", [
    ("in", 10.0, Fraction(254)),
    ("lbf", 1.0, Fraction("4.4482216152605")),
    ("psi", 1.0, Fraction("4.4482216152605") / Fraction("645.16")),
    ("lbf*in", 1000.0, Fraction("4.4482216152605") * Fraction("0.0254") * 1000),
    ("lbf*ft", 1.0, Fraction("4.4482216152605") * Fraction("0.3048")),
    ("lbf/in", 1.0, Fraction("4.4482216152605") / Fraction("0.0254")),
    ("1/degF", 1.0, Fraction(9, 5)),
    ("degF", 9.0, Fraction(5)),
])
def test_each_normalization_factor_is_its_exact_definition(unit, value, expected):
    from core.handoff.caepipe_mbf.si_writer import NORMALIZATION
    factor, _ = NORMALIZATION[unit]
    assert Fraction(value) * factor == expected  # exact in rationals
    assert float(Fraction(value) * factor) == float(expected)  # rounded once


def test_us_customary_weight_per_length_is_normalized_before_the_weight_density():
    model = us_customary_cantilever()
    model["load_cases"][0]["primitive_loads"].append(
        {"id": "w", "category": "weight", "target": {"type": "element", "pipe": "S1"}, "direction": "global_y",
         "magnitude": q(-1.0, "lbf/in"), "dimension": "force_per_length"})
    result = render_caepipe_mbf_si(model, title="W")
    assert result.status == "written", result.diagnostics
    # 1 lbf/in = 175.1268352 N/m; A = pi/4 (0.168275^2 - 0.154051^2) m^2; rho = 175.1268352 / (A x 9.80665).
    area = math.pi / 4.0 * (0.168275**2 - (0.168275 - 2 * 0.007112) ** 2)
    expected = float(Fraction("4.4482216152605") / Fraction("0.0254")) / (area * STANDARD_GRAVITY)
    assert result.material_map["M1"]["weight_density_kgf_per_m3"] == pytest.approx(expected, rel=1e-15)


def test_absolute_fahrenheit_reference_is_converted_by_definition():
    result = render_caepipe_mbf_si(cantilever(), title="T", reference_temperature_f=212.0)
    assert result.text.split("\r\n")[6] == "100"
    assert any(item["code"] == "MBF-SI-UNITS-NORMALIZED" and "T(degC)" in item["message"] for item in result.diagnostics)
    assert "MBF-SI-UNITS-NORMALIZED" not in {item["code"] for item in render_caepipe_mbf_si(cantilever(), title="T").diagnostics}


def test_units_without_an_exact_scaling_or_definition_are_refused():
    model = cantilever()
    model["project"]["units"]["length"] = "ft"
    assert_refused(render_caepipe_mbf_si(model, title="U"), "MBF-SI-PROJECT-UNIT-UNSUPPORTED", "project.units.length", "'ft'")
    model = cantilever()
    model["project"]["units"]["force"] = "kip"
    assert_refused(render_caepipe_mbf_si(model, title="U"), "MBF-SI-PROJECT-UNIT-UNSUPPORTED", "project.units.force", "'kip'")
    model = cantilever()
    model["materials"][0]["elastic_modulus"] = q(29.0e3, "ksi")
    assert_refused(render_caepipe_mbf_si(model, title="U"), "MBF-SI-UNIT-UNSUPPORTED",
                   "material:mat:CS", "elastic_modulus", "'ksi'")


def test_temperature_dependent_material_is_refused():
    model = cantilever()
    model["materials"][0]["temperature_points"] = [
        {"id": "hot", "temperature": q(200.0, "degC"), "elastic_modulus": q(1.9e11, "Pa")}
    ]
    assert_refused(render_caepipe_mbf_si(model, title="T"), "MBF-SI-TEMPERATURE-DEPENDENT-MATERIAL-UNSUPPORTED",
                   "material:mat:CS")


def test_hanger_support_is_refused():
    # Hangers stay refused (Design §12): a hanger's loads act by case, which no SWBPIPE member carries.
    model = cantilever()
    model["supports"].append({"id": "hanger:N1", "node": "N1", "family": "spring_hanger", "restraints": ["UY"],
                              "hanger": {"hanger_type": "spring_hanger",
                                         "stiffness": {"dof": "UY", "value": q(1.0e6, "N/m")}}})
    assert_refused(render_caepipe_mbf_si(model, title="S"), "MBF-SI-SUPPORT-UNSUPPORTED", "support:hanger:N1")


def spring(support_id, node_id, dof, value, unit="N/m", restraints=None):
    return {"id": support_id, "node": node_id, "family": "spring", "restraints": restraints or [dof],
            "stiffness": {"dof": dof, "value": q(value, unit)}}


def sprung_cantilever():
    """The invented cantilever with a spring on each translation at its tip: UY 250000 N/m, UX 1.5 lbf/in."""
    model = cantilever("mm")
    model["supports"] += [spring("spring:N1:y", "N1", "UY", 250000.0), spring("spring:N1:x", "N1", "UX", 1.5, "lbf/in")]
    return model


def test_a_spring_is_written_as_a_flexible_anchor():
    # Hand-derived: 250000 N/m = 250 N/mm (10^-3 exactly); 1.5 lbf/in = 1.5 x 4.4482216152605 N / 25.4 mm
    # = 0.262690252869714... N/mm, printed to 10 significant digits. UZ has no spring: 0, free; rotations free.
    result = render_caepipe_mbf_si(sprung_cantilever(), title="SPRING")
    assert result.status == "written", result.diagnostics
    assert "T20,KA,X3000,CTRAK(0.2626902529,250,0),ROTK(0,0,0),FOR(0,-6000,0)\r\n" in result.text
    assert result.support_map["spring:N1:y"] == {"node": "20", "caepipe_kind": "anchor", "restrained": ["UY"],
                                                 "family": "spring", "family_authored": True}
    assert "1 lbf/in" in next(item["message"] for item in result.diagnostics
                              if item["code"] == "MBF-SI-UNITS-NORMALIZED")
    assert any(note.startswith("Spring supports are written as flexible anchors") for note in result.assumptions)


@pytest.mark.parametrize("change, code, fragment", [
    (lambda m: m["supports"].append(spring("rx", "N1", "RX", 5.0, "N*m/rad")), "MBF-SI-SUPPORT-UNSUPPORTED",
     "rotational spring"),
    (lambda m: m["supports"].append(spring("y2", "N1", "UY", 5.0)), "MBF-SI-SUPPORT-UNSUPPORTED", "second spring"),
    (lambda m: m["supports"].append({"id": "r", "node": "N1", "restraints": ["UZ"]}), "MBF-SI-SUPPORT-UNSUPPORTED",
     "springs and rigid restraints"),
    (lambda m: m["supports"].append(spring("z0", "N1", "UZ", 0.0)), "MBF-SI-VALUE-OUT-OF-RANGE", "positive"),
    (lambda m: m["supports"].append(spring("zz", "N1", "UZ", 5.0, restraints=["UZ", "UX"])),
     "MBF-SI-SUPPORT-UNSUPPORTED", "same single DOF"),
    (lambda m: m["supports"].append(spring("zu", "N1", "UZ", 5.0, "N*m/rad")), "MBF-SI-UNIT-UNSUPPORTED", "stiffness"),
    (lambda m: m["supports"].append({"id": "zs", "node": "N1", "family": "spring", "restraints": ["UZ"]}),
     "MBF-SI-INPUT-INVALID", "needs a stiffness"),
    (lambda m: m["load_cases"].append({"id": "case:move", "primitive_loads": [
        {"id": "dy", "category": "imposed_displacement", "target": {"type": "support", "support": "spring:N1:y",
                                                                    "dof": "UY"},
         "direction": "UY", "magnitude": q(1.0, "mm"), "dimension": "displacement"}]}),
     "MBF-SI-IMPOSED-DISPLACEMENT-UNSUPPORTED", "anchor"),
])
def test_springs_the_writer_does_not_write_are_refused_by_name(change, code, fragment):
    model = sprung_cantilever()
    change(model)
    assert_refused(render_caepipe_mbf_si(model, title="S"), code, fragment)


def test_a_spring_export_is_checked_by_its_own_bound_reimport(monkeypatch):
    # The preflight runs for every spring export (Design §12). 1e306 N/mm prints as 1e+306, which the solve reads as
    # 1e309 N/m: the re-import refuses it (MBF-IMPORT-VALUE-UNREPRESENTABLE), so the export is refused by name.
    model = cantilever("mm")
    model["supports"].append(spring("huge", "N1", "UY", 1e306, "N/mm"))
    assert_refused(render_caepipe_mbf_si(model, title="S"), "MBF-SI-PREFLIGHT-REFUSED",
                   "MBF-IMPORT-VALUE-UNREPRESENTABLE")
    # 1e20 N/mm is finite in N/m, but the re-import refuses it as outside the solve's checked-JSON domain
    # (VER-002 r2): the preflight inherits that refusal.
    model["supports"][-1] = spring("huge", "N1", "UY", 1e20, "N/mm")
    assert_refused(render_caepipe_mbf_si(model, title="S"), "MBF-SI-PREFLIGHT-REFUSED",
                   "MBF-IMPORT-VALUE-UNREPRESENTABLE", "CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT")
    calls = []
    from core.handoff.caepipe_mbf import si_writer
    original = si_writer._Writer.survives_reimport
    monkeypatch.setattr(si_writer._Writer, "survives_reimport",
                        lambda self, result: calls.append(result.status) or original(self, result))
    assert render_caepipe_mbf_si(sprung_cantilever(), title="SPRING").status == "written"
    assert calls == ["written"]
    assert render_caepipe_mbf_si(cantilever(), title="PLAIN").status == "written"
    assert calls == ["written", "written"]  # the preflight is total (Design §14, VER-002 r3)


def test_the_preflight_is_total_and_refuses_what_printing_or_normalization_breaks(monkeypatch):
    # VER-002 r3: every export but the regeneration route's is checked by its own bound re-import, whatever it holds.
    from core.handoff.caepipe_mbf import si_writer
    from core.handoff.caepipe_mbf.importer import import_caepipe_mbf
    # A pressure-free, spring-free export passes it, and its text is unchanged by it.
    calls = []
    original = si_writer._Writer.survives_reimport
    monkeypatch.setattr(si_writer._Writer, "survives_reimport",
                        lambda self, result: calls.append(result.text) or original(self, result))
    plain = render_caepipe_mbf_si(cantilever(), title="PLAIN")
    assert plain.status == "written" and calls == [plain.text]
    # Printing crosses the checked-JSON boundary: 2^53 - 1 N imports, but prints to 10 significant digits as
    # 9.007199255e+15, an integral float beyond 2^53 - 1, which the re-import refuses.
    imported = import_caepipe_mbf(
        b"INVENTED BOUNDARY\r\nVERSION\r\n15.10\r\nOPTIONS\r\nHGRA,SI\r\nOPTEMP\r\n20\r\nMATERIAL\r\n"
        b"M1,0,0.3,1,1,CS,invented steel\r\nM1,20,200000,1.2e-05\r\nPIPE\r\nP1,168.3,7.11,0,0,0,0,0,0\r\nLOADS\r\n"
        b"L1,20,0,0\r\nLAYOUT\r\nF10,KA,MM1,PP1,CL=L1\r\nT20,X3000,CFOR(0,9007199254740991,0),FLC=1\r\n",
        material_provenance={"*": {"origin": "project_invented", "source_name": "invented writer test"}},
        source_label="boundary")
    assert imported.status == "imported", imported.diagnostics
    assert_refused(render_caepipe_mbf_si(imported.model, title="BOUNDARY"), "MBF-SI-PREFLIGHT-REFUSED",
                   "MBF-IMPORT-VALUE-UNREPRESENTABLE", "CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT")
    # Unit normalization crosses it: 1e14 kN, within the domain, is written as 1e17 N, beyond it.
    model = cantilever()
    model["load_cases"][0]["primitive_loads"] = [force("tip", "N1", "global_y", -1e14, "kN")]
    assert_refused(render_caepipe_mbf_si(model, title="KN"), "MBF-SI-PREFLIGHT-REFUSED",
                   "MBF-IMPORT-VALUE-UNREPRESENTABLE", "CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT")


def test_bend_component_is_refused():
    model = cantilever()
    model["components"] = [{"id": "bend:1", "kind": "bend", "node": "N1"}]
    assert_refused(render_caepipe_mbf_si(model, title="B"), "MBF-SI-COMPONENT-UNSUPPORTED", "component:bend:1", "'bend'")


def test_mixed_load_case_is_refused():
    model = cantilever()
    model["load_cases"][0]["primitive_loads"].append(
        {"id": "t1", "category": "thermal", "target": {"type": "element", "pipe": "S1"},
         "direction": "global_x", "magnitude": q(50.0, "degC"), "dimension": "temperature_interval"}
    )
    assert_refused(render_caepipe_mbf_si(model, title="M"), "MBF-SI-LOAD-CASE-MAPPING-UNSUPPORTED", "load_case:case:tip")


def test_third_load_case_is_refused():
    model = cantilever()
    extra = deepcopy(model["load_cases"][0])
    extra["id"] = "case:second"
    model["load_cases"].append(extra)
    assert_refused(render_caepipe_mbf_si(model, title="M"), "MBF-SI-LOAD-CASE-MAPPING-UNSUPPORTED", "load_case:case:second")


def test_non_finite_value_is_refused():
    model = cantilever()
    model["load_cases"][0]["primitive_loads"][0]["magnitude"]["value"] = float("nan")
    assert_refused(render_caepipe_mbf_si(model, title="N"), "MBF-SI-VALUE-OUT-OF-RANGE", "load:case:tip/tip")
    model = cantilever()
    model["nodes"][1]["position"]["x"] = float("inf")
    assert_refused(render_caepipe_mbf_si(model, title="N"), "MBF-SI-VALUE-OUT-OF-RANGE", "node:N1")


def test_pressure_load_is_refused():
    model = cantilever()
    model["load_cases"][0]["primitive_loads"].append(
        {"id": "p", "category": "pressure", "target": {"type": "element", "pipe": "S1"},
         "direction": "global_x", "magnitude": q(1.0e6, "Pa"), "dimension": "pressure"}
    )
    assert_refused(render_caepipe_mbf_si(model, title="P"), "MBF-SI-PRESSURE-UNSUPPORTED", "load:case:tip/p")


def straight_run(elements: int):
    model = model_shell("m")
    model["nodes"] = [node(f"N{index:02d}", float(index)) for index in range(elements + 1)]
    model["pipe_segments"] = [pipe(f"S{index:02d}", f"N{index:02d}", f"N{index + 1:02d}", 0.1683, 0.00711, "m")
                              for index in range(elements)]
    model["supports"] = [{"id": "A", "node": "N00", "restraints": list(SIX)}]
    return model


def test_layout_row_limit_warning_at_twenty_one_rows():
    at_limit = render_caepipe_mbf_si(straight_run(18), title="ROWS")
    assert at_limit.status == "written"
    assert at_limit.layout_row_count == 20
    assert "MBF-SI-LAYOUT-ROW-LIMIT" not in codes(at_limit, "warning")

    over = render_caepipe_mbf_si(straight_run(19), title="ROWS")
    assert over.status == "written"
    assert over.layout_row_count == 21
    assert "MBF-SI-LAYOUT-ROW-LIMIT" in codes(over, "warning")

    unlimited = render_caepipe_mbf_si(straight_run(19), title="ROWS", max_layout_rows=None)
    assert "MBF-SI-LAYOUT-ROW-LIMIT" not in codes(unlimited, "warning")


def test_title_is_sanitized_to_printable_ascii_without_commas():
    result = render_caepipe_mbf_si(cantilever(), title="Café, line " + "x" * 100)
    first = result.text.split("\r\n")[0]
    assert first.isascii() and "," not in first and len(first) <= 70


def test_partial_rotational_restraint_is_refused_for_the_target_build():
    model = branched()
    model["supports"][2]["restraints"] = ["UY", "RZ"]
    result = render_caepipe_mbf_si(model, title="FIXR")
    assert_refused(result, "MBF-SI-SUPPORT-UNSUPPORTED", "support:R4", "rotational restraints", "FIXR")


def test_full_six_dof_restraint_from_two_supports_is_an_anchor_not_a_rotational_restraint(monkeypatch):
    model = cantilever()
    model["supports"] = [{"id": "T", "node": "N0", "restraints": ["UX", "UY", "UZ"]},
                         {"id": "R", "node": "N0", "restraints": ["RX", "RY", "RZ"]}]
    # The solve reads the family-less R as a guide, which restrains no rotation (SUPPORT_INPUT_INVALID): the bound
    # re-import refuses its split, so the total preflight refuses the export by name (VER-002 r3).
    assert_refused(render_caepipe_mbf_si(model, title="SPLIT"), "MBF-SI-PREFLIGHT-REFUSED", "support_map[R]",
                   "SUPPORT_INPUT_INVALID")
    from core.handoff.caepipe_mbf import si_writer
    monkeypatch.setattr(si_writer._Writer, "survives_reimport", lambda self, result: True)
    result = render_caepipe_mbf_si(model, title="SPLIT")  # the rendering itself: one KA row, no FIXR
    assert result.status == "written", result.diagnostics
    assert result.text.split("\r\n")[16] == "F10,KA,MM1,PP1,CL=L1"


def bent_l(radius=0.25, factor=1.0, y_reference=(1.0, 0.0, -1.0)):
    """Invented horizontal L: straight 3.75 m along X, a 90 degree bend R = 0.25 m, straight 2.75 m along Z."""
    model = model_shell("m")
    model["nodes"] = [node("A", 0.0), node("T1", 4.0 - radius), node("T2", 4.0, 0.0, radius), node("B", 4.0, 0.0, 3.0)]
    model["pipe_segments"] = [
        pipe("S1", "A", "T1", 0.1683, 0.00711, "m"),
        pipe("BEND", "T1", "T2", 0.1683, 0.00711, "m"),
        pipe("S2", "T2", "B", 0.1683, 0.00711, "m"),
    ]
    for item in model["pipe_segments"]:
        item["y_reference"] = {"x": 0.0, "y": 1.0, "z": 0.0}
    model["pipe_segments"][1]["y_reference"] = dict(zip("xyz", y_reference))
    model["supports"] = [{"id": "SA", "node": "A", "restraints": list(SIX)}, {"id": "SB", "node": "B", "restraints": list(SIX)}]
    bend = {"id": "bend-1", "kind": "bend", "node": "T1",
            "geometry": {"bend_pipe_ref": "BEND", "bend_radius": q(radius * 1000.0, "mm"), "bend_angle": q(math.pi / 2, "rad")},
            "mechanics_interface": {"solver_consumption": "curved_bend_macro_element"}}
    if factor is not None:
        bend["modifiers"] = {"flexibility_factor_user_value": q(factor, "none")}
    model["components"] = [bend]
    model["load_cases"] = [{"id": "case:th", "primitive_loads": [
        {"id": f"t:{pipe_id}", "category": "thermal", "target": {"type": "element", "pipe": pipe_id},
         "direction": "global_x", "magnitude": q(100.0, "degC"), "dimension": "temperature_interval"}
        for pipe_id in ("S1", "BEND", "S2")]}]
    return model


def test_bend_is_written_at_its_tangent_intersection():
    result = render_caepipe_mbf_si(bent_l(), title="INVENTED L")
    assert result.status == "written", result.diagnostics
    lines = result.text.split("\r\n")
    assert lines[lines.index("LAYOUT") + 1:] == [
        "F10,KA,MM1,PP1,CL=L1",
        "T20,KI,B250,X4000,CFF=1",
        "T30,KA,Z3000",
        "",
    ]
    assert result.layout_row_count == 4
    assert result.node_map == {"A": "10", "T1": "20A", "T2": "20B", "B": "30"}
    assert {key: (value["from"], value["to"]) for key, value in result.element_map.items()} == {
        "S1": ("10", "20A"), "BEND": ("20A", "20B"), "S2": ("20B", "30")}
    assert [result.element_map[key]["layout_order"] for key in ("S1", "BEND", "S2")] == [0, 1, 2]
    bend = result.element_map["BEND"]["bend"]
    assert bend["radius_mm"] == 250.0 and bend["flexibility_factor"] == 1.0
    assert bend["tangent_intersection_mm"] == [4000.0, 0.0, 0.0]
    assert any("tangent intersection" in note for note in result.assumptions)
    omitted = [item for item in result.loss_report if item["code"] == "MBF-SI-ORIENTATION-NOT-EXPORTED"]
    assert omitted and "BEND" not in omitted[0]["reason"]


def bent_l_with_sifs(in_plane=2.5, out_of_plane=2.0, unit="none"):
    model = bent_l()
    model["components"][0]["modifiers"]["sif_in_plane_user_value"] = q(in_plane, unit)
    model["components"][0]["modifiers"]["sif_out_of_plane_user_value"] = q(out_of_plane, unit)
    return model


def test_bend_user_sifs_are_never_dropped_silently():
    """W-17's probe: a bend's directional SIF pair either changes the text or is listed by name in the loss report."""
    base = render_caepipe_mbf_si(bent_l(), title="INVENTED L")
    result = render_caepipe_mbf_si(bent_l_with_sifs(), title="INVENTED L")
    listed = [item for item in result.loss_report if item["entity"] == "component:bend-1"
              and "sif_in_plane_user_value" in item["reason"] and "sif_out_of_plane_user_value" in item["reason"]]
    assert result.text != base.text or listed


def test_bend_user_sifs_are_written_as_bsif():
    # Format of .MBF (users_manual/import_mbf.htm): BSIF(in-plane,out-plane) is a bend's SIF comment.
    result = render_caepipe_mbf_si(bent_l_with_sifs(), title="INVENTED L")
    assert result.status == "written", result.diagnostics
    lines = result.text.split("\r\n")
    assert lines[lines.index("LAYOUT") + 1:] == [
        "F10,KA,MM1,PP1,CL=L1",
        "T20,KI,B250,X4000,CFF=1,BSIF(2.5,2)",
        "T30,KA,Z3000",
        "",
    ]
    bend = result.element_map["BEND"]["bend"]
    assert (bend["sif_in_plane"], bend["sif_out_of_plane"]) == (2.5, 2.0)
    assert not [item for item in result.loss_report if "sif" in item["reason"].lower()]


@pytest.mark.parametrize("change, code, fragment", [
    (lambda m: m["components"][0]["modifiers"].pop("sif_out_of_plane_user_value"), "MBF-SI-BEND-SIF-INCOMPLETE",
     "only one of"),
    (lambda m: m["components"][0]["modifiers"].update(sif_in_plane_user_value=q(0.0, "none")),
     "MBF-SI-VALUE-OUT-OF-RANGE", "must be positive"),
    (lambda m: m["components"][0]["modifiers"].update(sif_in_plane_user_value=q(2.0, "mm")), "MBF-SI-UNIT-UNSUPPORTED",
     "sif_in_plane_user_value"),
])
def test_bend_sifs_the_writer_cannot_write_are_refused_by_name(change, code, fragment):
    model = bent_l_with_sifs()
    change(model)
    assert_refused(render_caepipe_mbf_si(model, title="B"), code, "component:bend-1", fragment)


def library_section(section_id, od=0.1683, wall=0.00711, unit="m"):
    return {"id": section_id, "name": section_id, "section_type": "pipe",
            "properties": {"outside_diameter": q(od, unit), "wall_thickness": q(wall, unit)},
            "provenance": "invented library section"}


def sectioned_bent_l():
    """bent_l continued by a straight S3 from B, every pipe of one OD and wall (Design §15, Format 4): S1 and the bend
    bind section:A, S2 binds section:B, S3 binds none, and section:unused is bound by no pipe."""
    model = bent_l()
    model["nodes"].append(node("C", 4.0, 0.0, 6.0))
    model["pipe_segments"].append(pipe("S3", "B", "C", 0.1683, 0.00711, "m"))
    model["sections"] = [library_section("section:A"), library_section("section:B"), library_section("section:unused")]
    for pipe_id, section_id in (("S1", "section:A"), ("BEND", "section:A"), ("S2", "section:B")):
        next(item for item in model["pipe_segments"] if item["id"] == pipe_id)["section_ref"] = section_id
    return model


def decorated_bent_l():
    """bent_l with every member and field the MBF does not keep (Design §17.1), each non-empty."""
    model = bent_l_with_sifs()
    meta = {"label": "invented", "provenance": "invented", "tags": ["t"], "notes": "n"}
    record = {"contributor_certification": "invented", "redistribution_status": "public_permissive"}
    library = {"section_type": "pipe", "properties": {"outside_diameter": q(0.1683, "m"),
                                                      "wall_thickness": q(0.00711, "m")},
               **{key: value for key, value in meta.items() if key != "label"}}  # a Section has a name, no label
    model.update(data_boundary={"public_examples_policy": "invented"},
                 sections=[{"id": "sec", "name": "S", **library}, {"id": "sec:unused", "name": "U", **library}],
                 diagnostics=[{"code": "X", "severity": "info", "message": "m"}], lines=[{"id": "line"}],
                 unresolved_assumptions=[{"assumption_id": "a"}], rule_pack_refs=[{"rule_pack_id": "r"}],
                 constraints=[{"constraint_id": "c"}], equipment_interfaces=[{"id": "e"}])
    model["project"].update(name="P", description="D", tags=["t"], notes="n")
    model["project"]["units"].update(moment="N*m", angle="rad")
    model["materials"][0].update(meta, constitutive_basis="invented", provenance_record=record)
    for member in ("nodes", "pipe_segments", "supports", "load_cases"):
        for item in model[member]:
            item.update(meta)
    for item in model["pipe_segments"]:  # one library section on the bend row's pipes: still one PIPE record
        item["section_ref"] = "sec"
    model["supports"][0].update(boundary_association={"boundary_id": "b"}, properties={"linear_stiffness": q(1, "N/m")},
                                provenance_record=record)
    model["load_cases"][0].update(kind="thermal", status="draft")
    model["load_cases"][0]["primitive_loads"][0]["provenance"] = "invented"
    bend = model["components"][0]
    bend.update(meta, completeness={"status": "complete"}, provenance_record=record)
    bend["geometry"].update(bend_plane_orientation="+Y", bend_geometry_source_reference="invented")
    bend["modifiers"].update(sif_user_value=q(1.5, "none"), source_reference="invented")
    bend["mechanics_interface"]["rule_check_consumption"] = "invented"
    return model


SWEPT = {  # entity -> the model entities its reason names (Design §17.1)
    **{f"model.{name}": () for name in ("data_boundary", "analysis_status", "diagnostics", "lines",
                                        "unresolved_assumptions", "rule_pack_refs", "constraints",
                                        "equipment_interfaces")},
    **{f"model.project.{name}": ("project",) for name in ("id", "name", "description", "tags", "notes")},
    "model.project.units": ("length m", "angle rad"),
    **{f"model.materials.{name}": ("material:mat:CS",)
       for name in ("label", "provenance", "tags", "notes", "constitutive_basis", "provenance_record")},
    **{f"model.nodes.{name}": ("node:A", "node:T1", "node:T2", "node:B")
       for name in ("label", "provenance", "tags", "notes")},
    **{f"model.pipe_segments.{name}": ("pipe:S1", "pipe:BEND", "pipe:S2")
       for name in ("label", "provenance", "tags", "notes")},
    "model.sections": ("sec:unused",),
    **{f"model.sections.{name}": ("section:sec", "section:sec:unused") for name in ("name", "provenance", "tags", "notes")},
    **{f"model.supports.{name}": ("support:SA", "support:SB") for name in ("label", "provenance", "tags", "notes")},
    **{f"model.supports.{name}": ("support:SA",) for name in ("boundary_association", "properties",
                                                              "provenance_record")},
    **{f"model.components.{name}": ("component:bend-1",)
       for name in ("label", "provenance", "tags", "notes", "completeness", "provenance_record",
                    "geometry.bend_plane_orientation", "geometry.bend_geometry_source_reference",
                    "modifiers.sif_user_value", "modifiers.source_reference",
                    "mechanics_interface.rule_check_consumption")},
    **{f"model.load_cases.{name}": ("load_case:case:th",)
       for name in ("label", "provenance", "tags", "notes", "kind", "status")},
    "model.load_cases.primitive_loads.provenance": ("load:case:th/t:S1",),
}


def test_every_unwritten_member_and_field_is_one_omitted_loss_entry():
    plain = render_caepipe_mbf_si(bent_l_with_sifs(), title="INVENTED L")
    result = render_caepipe_mbf_si(decorated_bent_l(), title="INVENTED L")
    assert result.status == "written", result.diagnostics
    assert result.text == plain.text  # the sweep lists; it changes nothing written
    swept = {item["entity"]: item for item in result.loss_report
             if item["code"] in {"MBF-SI-MEMBER-NOT-EXPORTED", "MBF-SI-FIELD-NOT-EXPORTED"}}
    assert set(swept) == set(SWEPT)
    for entity, named in SWEPT.items():
        assert swept[entity]["category"] == "omitted"
        assert all(name in swept[entity]["reason"] for name in named), swept[entity]
    assert len(result.loss_report) == len({(item["code"], item["entity"]) for item in result.loss_report})


def test_empty_members_and_fields_are_not_listed():
    model = bent_l()
    model.update(diagnostics=[], lines=[], data_boundary={})
    model["nodes"][0].update(label="", tags=[], notes=None)
    swept = {item["entity"] for item in render_caepipe_mbf_si(model, title="L").loss_report}
    assert not swept & {"model.diagnostics", "model.lines", "model.data_boundary", "model.nodes.label",
                        "model.nodes.tags", "model.nodes.notes"}


def test_project_units_the_import_restores_are_not_listed():
    from core.handoff.caepipe_mbf.importer import import_caepipe_mbf  # deferred, as the writer's own preflight
    written = render_caepipe_mbf_si(cantilever("mm"), title="C")
    data = written.text.encode("ascii")
    back = import_caepipe_mbf(data, sidecar=written.sidecar(hashlib.sha256(data).hexdigest()), source_label="units",
                              material_provenance={"*": {"origin": "project_invented", "source_name": "invented"}})
    assert back.status == "imported", back.diagnostics
    assert back.model["project"]["units"] == IMPORTED_PROJECT_UNITS
    model = cantilever("mm")
    model["project"]["units"] = dict(IMPORTED_PROJECT_UNITS)
    assert "model.project.units" not in {item["entity"] for item in render_caepipe_mbf_si(model, title="C").loss_report}


def test_a_pressure_region_provenance_is_listed_as_omitted():
    result = render_caepipe_mbf_si(pressurized_l(), title="PRESSURE L")
    assert result.status == "written", result.diagnostics
    [entry] = [item for item in result.loss_report if item["entity"] == "model.load_cases.pressure_regions.provenance"]
    assert entry["category"] == "omitted" and "case:p/region:region:L" in entry["reason"], entry


def test_an_authored_shear_modulus_the_poisson_ratio_overrides_is_listed_as_omitted():
    model = cantilever()
    model["materials"][0]["poisson_ratio"] = q(0.25, "1")
    model["materials"][0]["shear_modulus"] = q(7.0e10, "Pa")  # E/(2(1+nu)) = 80 GPa
    result = render_caepipe_mbf_si(model, title="NU")
    assert result.status == "written", result.diagnostics
    [entry] = [item for item in result.loss_report if item["code"] == "MBF-SI-SHEAR-MODULUS-NOT-EXPORTED"]
    assert entry["category"] == "omitted" and entry["entity"] == "material:mat:CS", entry
    model["materials"][0]["shear_modulus"] = q(8.0e10, "Pa")  # consistent: nothing lost
    assert "MBF-SI-SHEAR-MODULUS-NOT-EXPORTED" not in {
        item["code"] for item in render_caepipe_mbf_si(model, title="NU").loss_report}


def test_a_material_no_pipe_segment_uses_is_listed_as_omitted():
    model = cantilever()
    model["materials"].append(dict(model["materials"][0], id="mat:unused"))
    result = render_caepipe_mbf_si(model, title="C")
    assert result.status == "written", result.diagnostics
    [entry] = [item for item in result.loss_report if item["entity"] == "model.materials"]
    assert entry["category"] == "omitted" and "mat:unused" in entry["reason"] and "mat:CS" not in entry["reason"]
    assert result.text == render_caepipe_mbf_si(cantilever(), title="C").text


def test_bend_traversed_from_the_far_side_swaps_a_and_b():
    model = bent_l()
    model["supports"] = [{"id": "SB", "node": "B", "restraints": list(SIX)}]
    result = render_caepipe_mbf_si(model, title="REVERSED L")
    assert result.status == "written", result.diagnostics
    lines = result.text.split("\r\n")
    assert lines[lines.index("LAYOUT") + 1:] == [
        "F10,KA,MM1,PP1,X4000,Y0,Z3000,CL=L1",
        "T20,KI,B250,Z-3000,CFF=1",
        "T30,X-4000",
        "",
    ]
    assert result.node_map == {"B": "10", "T2": "20A", "T1": "20B", "A": "30"}
    assert result.element_map["BEND"]["reversed"] is True


def test_bend_without_user_flexibility_factor_is_refused():
    assert_refused(render_caepipe_mbf_si(bent_l(factor=None), title="B"), "MBF-SI-BEND-FLEXIBILITY-MISSING",
                   "component:bend-1", "implicit or code-based")


def test_bend_geometry_inconsistent_with_radius_is_refused():
    model = bent_l()
    model["components"][0]["geometry"]["bend_radius"] = q(300.0, "mm")
    model["components"][0]["geometry"].pop("bend_angle")
    assert_refused(render_caepipe_mbf_si(model, title="B"), "MBF-SI-BEND-GEOMETRY-INCONSISTENT", "R tan(theta/2)")


def test_bend_angle_disagreeing_with_the_straights_is_refused():
    model = bent_l()
    model["components"][0]["geometry"]["bend_angle"] = q(80.0, "deg")
    assert_refused(render_caepipe_mbf_si(model, title="B"), "MBF-SI-BEND-GEOMETRY-INCONSISTENT", "bend_angle")


def test_bend_whose_straights_do_not_intersect_is_refused():
    model = bent_l()
    model["nodes"][3]["position"]["y"] = 0.5  # outgoing straight leaves the bend plane
    assert_refused(render_caepipe_mbf_si(model, title="B"), "MBF-SI-BEND-GEOMETRY-INCONSISTENT", "do not intersect")


def test_bend_y_reference_on_the_wrong_side_is_refused():
    assert_refused(render_caepipe_mbf_si(bent_l(y_reference=(-1.0, 0.0, 1.0)), title="B"),
                   "MBF-SI-BEND-GEOMETRY-INCONSISTENT", "y_reference")


def test_support_or_load_on_a_bend_tangent_point_is_refused():
    model = bent_l()
    model["supports"].append({"id": "G", "node": "T1", "restraints": ["UY"]})
    assert_refused(render_caepipe_mbf_si(model, title="B"), "MBF-SI-BEND-UNSUPPORTED", "tangent point 'T1'")
    model = bent_l()
    model["load_cases"].append({"id": "case:f", "primitive_loads": [force("f", "T2", "global_y", -10.0)]})
    assert_refused(render_caepipe_mbf_si(model, title="B"), "MBF-SI-BEND-UNSUPPORTED", "tangent point 'T2'")


def test_bend_at_a_run_end_is_refused():
    model = bent_l()
    model["nodes"] = [item for item in model["nodes"] if item["id"] != "B"]
    model["pipe_segments"] = model["pipe_segments"][:2]
    model["supports"] = model["supports"][:1]
    model["load_cases"][0]["primitive_loads"] = model["load_cases"][0]["primitive_loads"][:2]
    assert_refused(render_caepipe_mbf_si(model, title="B"), "MBF-SI-BEND-UNSUPPORTED", "ends a run")


def test_consecutive_bends_are_refused():
    model = bent_l()
    model["nodes"].append(node("T3", 4.25, 0.0, 0.5))
    model["pipe_segments"][2] = pipe("BEND2", "T2", "T3", 0.1683, 0.00711, "m")
    model["pipe_segments"][2]["y_reference"] = {"x": -1.0, "y": 0.0, "z": 1.0}
    model["pipe_segments"].append(pipe("S3", "T3", "B", 0.1683, 0.00711, "m"))
    model["load_cases"][0]["primitive_loads"] = model["load_cases"][0]["primitive_loads"][:2]
    model["components"].append({"id": "bend-2", "kind": "bend", "node": "T2",
                                "geometry": {"bend_pipe_ref": "BEND2", "bend_radius": q(250.0, "mm")},
                                "modifiers": {"flexibility_factor_user_value": q(1.0, "none")},
                                "mechanics_interface": {"solver_consumption": "curved_bend_macro_element"}})
    result = render_caepipe_mbf_si(model, title="B")
    assert_refused(result, "MBF-SI-BEND-UNSUPPORTED")
    assert any("consecutive bends" in item["message"] for item in result.diagnostics)


def test_bend_with_other_properties_than_its_incoming_straight_is_refused():
    model = bent_l()
    model["load_cases"][0]["primitive_loads"][1]["magnitude"] = q(90.0, "degC")
    assert_refused(render_caepipe_mbf_si(model, title="B"), "MBF-SI-BEND-UNSUPPORTED", "one layout row")


# Model document 0.8.0 (DEL-02-01 Design model-document-0.8.0.md §3.1): the writer reads only the current model
# document. A retired one is refused by name with the solve's code and text, and nothing is written; any other
# non-current version is refused as MBF-SI-SCHEMA-VERSION.
def test_a_retired_model_document_is_refused_by_name_and_nothing_is_written():
    for version in ("0.8.0", "0.7.0", "0.1.0"):
        model = cantilever()
        model["schema_version"] = version
        result = render_caepipe_mbf_si(model, title="Invented retired cantilever")
        assert (result.status, result.text) == ("refused", None), version
        refusal = [d for d in result.diagnostics if d["code"] == "MODEL_SCHEMA_VERSION_RETIRED"]
        assert refusal == [{"code": "MODEL_SCHEMA_VERSION_RETIRED", "severity": "error", "entity": "model.schema_version",
                            "message": f"model document {version} is retired; re-author it as model document 0.9.0 (a pressure-free model omits pressure_contract). Nothing is migrated."}]


def test_a_newer_or_unversioned_model_document_is_refused():
    for version in ("0.10.0", None):
        model = cantilever()
        model["schema_version"] = version
        result = render_caepipe_mbf_si(model, title="Invented newer cantilever")
        assert (result.status, result.text) == ("refused", None), version
        assert [d["severity"] for d in result.diagnostics if d["code"] == "MBF-SI-SCHEMA-VERSION"] == ["error"]
    assert render_caepipe_mbf_si(cantilever(), title="Invented current cantilever").status == "written"


# Sidecar fields the MBF importer reads back (Design `caepipe-mbf-import.md` §7.4, §12, §15, §19).
def test_support_map_records_each_support_family():
    model = branched()
    model["supports"][0]["family"] = "anchor"
    result = render_caepipe_mbf_si(model, title="FAMILY")
    assert result.status == "written", result.diagnostics
    # The effective family (Design §15, format 2): the authored one, or the solve's inference, with the flag.
    assert (result.support_map["A1"]["family"], result.support_map["A1"]["family_authored"]) == ("anchor", True)
    assert (result.support_map["A3"]["family"], result.support_map["A3"]["family_authored"]) == ("anchor", False)
    restraints = result.support_map["R4"]["restrained"]
    assert len(restraints) < 6  # fewer than six DOFs and no family: the solve's guide
    assert (result.support_map["R4"]["family"], result.support_map["R4"]["family_authored"]) == ("guide", False)


def test_load_map_records_each_primitive_direction():
    result = render_caepipe_mbf_si(branched(), title="DIRECTION")
    assert result.status == "written", result.diagnostics
    assert {key: entry["direction"] for key, entry in result.load_map.items()} == {
        "case:sus/f2": "global_y", "case:sus/m4": "RZ", "case:th/t1": "global_x", "case:th/t2": "global_x",
        "case:th/d3": "UY", "case:th/r3": "RZ"}


def test_material_map_origin_only_from_affirmative_provenance():
    def origin(record):
        model = cantilever()
        if record is not None:
            model["materials"][0]["provenance_record"] = record
        result = render_caepipe_mbf_si(model, title="ORIGIN")
        assert result.status == "written", result.diagnostics
        found = result.material_map["M1"]["origin"]
        return None if found == "undeclared" else found  # format 2 records "undeclared", never null

    invented = {"source_name": "invented", "contributor_certification": "invented non-engineering example",
                "redistribution_status": "public_permissive"}
    assert origin(invented) == "project_invented"
    assert origin({**invented, "redistribution_status": "private_only"}) is None  # the disposition alone is not enough
    assert origin({"source_name": "x", "redistribution_status": "public_permissive"}) is None
    assert origin({"source_name": "own", "contributor_certification": "user-supplied own source"}) == "user_supplied"
    library = {"library_kind": "material", "library_id": "lib", "library_version": "sha256:0", "record_id": "r"}
    assert origin({**invented, "library_ref": library}) is None
    for status in ("public_permissive", "private_only", "unknown", "protected_suspected"):
        assert origin({"source_name": "x", "redistribution_status": status}) is None
    assert origin(None) is None  # a bare provenance string or none at all


def test_schema_version_check_reads_the_model_schema_enum():
    current = current_model_schema_version()
    schema = json.loads(MODEL_SCHEMA_PATH.read_text(encoding="utf-8"))
    assert schema["properties"]["schema_version"]["enum"] == [current]
    model = cantilever()
    model["schema_version"] = current
    assert "MBF-SI-SCHEMA-VERSION" not in codes(render_caepipe_mbf_si(model, title="V"))
    model["schema_version"] = "0.0.1"  # neither current nor retired: refused (r6 F4), no longer a warning
    result = render_caepipe_mbf_si(model, title="V")
    assert "MBF-SI-SCHEMA-VERSION" in codes(result, "error")
    assert any(f"is not {current}" in item["message"] for item in result.diagnostics)


def cancelling_cantilever():
    """Codex VER-002 r1 reproduction (invented): the US cantilever with an explicit G = 29e6/2.6 psi instead of nu,
    and three same-node Y forces 1e9, 0.001 and -1e9 lbf."""
    model = us_customary_cantilever()
    model["materials"][0].pop("poisson_ratio")
    model["materials"][0]["shear_modulus"] = q(29.0e6 / 2.6, "psi")
    model["load_cases"][0]["primitive_loads"] = [force(f"f{index}", "N1", "global_y", value, "lbf")
                                                 for index, value in enumerate((1e9, 0.001, -1e9))]
    return model


def test_same_target_loads_are_summed_exactly_before_one_rounding():
    # By hand: 1e9 + 0.001 - 1e9 = 0.001 lbf exactly; 0.001 x 4.4482216152605 = 0.0044482216152605 N, printed to ten
    # significant digits as 0.004448221615. A binary64 running sum of the converted forces gives 0.004447937012.
    result = render_caepipe_mbf_si(cancelling_cantilever(), title="CANCEL")
    assert result.status == "written", result.diagnostics
    assert "T20,X3048,CFOR(0,0.004448221615,0)" in result.text.split("\r\n")
    # The same holds for thermal changes summed on one element: 100 + 1e-12 - 100 C = 1e-12 C. At a 0 C reference,
    # T1 prints as 1e-12; a binary64 running sum would print 9.947598301e-13. (At 20 C, T1 = 20 + 1e-12 prints as 20:
    # the thermal case vanishes from the file but not from the sidecar, which the total preflight refuses.)
    model = cantilever()
    model["load_cases"] = [{"id": "case:th", "primitive_loads": [
        {"id": f"t{index}", "category": "thermal", "target": {"type": "element", "pipe": "S1"}, "direction": "global_x",
         "magnitude": q(value, "degC"), "dimension": "temperature_interval"} for index, value in enumerate((100.0, 1e-12, -100.0))]}]
    result = render_caepipe_mbf_si(model, title="DT", reference_temperature_c=0.0)
    assert result.status == "written", result.diagnostics
    assert "L1,1e-12,0,0" in result.text.split("\r\n")
    assert_refused(render_caepipe_mbf_si(model, title="DT"), "MBF-SI-PREFLIGHT-REFUSED", "load_case_map")


def test_a_normalized_value_that_overflows_or_underflows_is_refused_by_name():
    # By hand: 1e308 lbf x 4.4482216152605 N/lbf = 4.45e308 N, beyond the largest binary64 (1.80e308).
    model = us_customary_cantilever()
    model["load_cases"][0]["primitive_loads"][0]["magnitude"] = q(-1e308, "lbf")
    result = render_caepipe_mbf_si(model, title="O")
    assert result.status == "refused"
    assert any(item["code"] == "MBF-SI-VALUE-OUT-OF-RANGE" and "not representable" in item["message"]
               for item in result.diagnostics)
    # By hand: 5e-324 psi (the smallest subnormal) x 0.00689 N/mm2 per psi is below half the smallest subnormal: 0.
    model = us_customary_cantilever()
    model["materials"][0]["elastic_modulus"] = q(5e-324, "psi")
    result = render_caepipe_mbf_si(model, title="U")
    assert result.status == "refused" and "MBF-SI-VALUE-OUT-OF-RANGE" in codes(result)
    # By hand: 1e308 m x 1000 = 1e311 mm overflows.
    model = cantilever()
    model["nodes"][1]["position"]["x"] = 1e308
    assert "MBF-SI-VALUE-OUT-OF-RANGE" in codes(render_caepipe_mbf_si(model, title="L"))


# Derived values the writer forms itself (independent review VER-002 r2): each is checked before it is printed.
def test_overflowing_temperatures_are_refused_not_printed_as_inf():
    # Codex VER-002 r2: reference 1e308 C and a thermal change of 1e308 degC. By hand the material table's top
    # temperature is 1e308 + 1e308 = 2e308 C, beyond the largest binary64 (1.80e308); it printed "inf" before.
    model = cantilever()
    model["load_cases"] = [{"id": "case:th", "primitive_loads": [
        {"id": "dt", "category": "thermal", "target": {"type": "element", "pipe": "S1"}, "direction": "global_x",
         "magnitude": q(1e308, "degC"), "dimension": "temperature_interval"}]}]
    result = render_caepipe_mbf_si(model, title="HOT", reference_temperature_c=1e308)
    assert result.status == "refused" and result.text is None
    assert any(item["code"] == "MBF-SI-VALUE-OUT-OF-RANGE" and "temperature" in item["message"]
               for item in result.diagnostics)
    # By hand: -1e308 C reference and a -1e308 degC change give -2e308 C at the bottom of the table.
    model["load_cases"][0]["primitive_loads"][0]["magnitude"] = q(-1e308, "degC")
    assert "MBF-SI-VALUE-OUT-OF-RANGE" in codes(render_caepipe_mbf_si(model, title="COLD", reference_temperature_c=-1e308))


def test_an_overflowing_layout_offset_is_refused():
    # By hand: nodes at x = -1e308 mm and +1e308 mm give an offset of 2e308 mm.
    model = cantilever("mm")
    model["nodes"] = [node("N0", -1e308), node("N1", 1e308)]
    result = render_caepipe_mbf_si(model, title="FAR")
    assert result.status == "refused" and any(
        item["code"] == "MBF-SI-VALUE-OUT-OF-RANGE" and "layout offset" in item["message"] for item in result.diagnostics)


def test_an_overflowing_section_area_is_refused_not_raised():
    # By hand: OD 1e305 mm = 1e302 m; OD^2 = 1e604 overflows, so the bare-metal area and the weight density cannot
    # be formed. It raised OverflowError (float ** 2) before.
    model = cantilever("mm")
    model["pipe_segments"][0]["section"] = {"outside_diameter": q(1e305, "mm"), "wall_thickness": q(1e304, "mm")}
    model["load_cases"][0]["primitive_loads"].append(
        {"id": "w", "category": "weight", "target": {"type": "element", "pipe": "S1"}, "direction": "global_y",
         "magnitude": q(-1.0, "N/m"), "dimension": "force_per_length"})
    result = render_caepipe_mbf_si(model, title="BIG")
    assert result.status == "refused" and "MBF-SI-VALUE-OUT-OF-RANGE" in codes(result)


def test_a_tangent_intersection_beyond_binary64_is_refused():
    # By hand: incoming leg along +X ending at T1 x = 1.7e308 mm; outgoing direction (-0.5, 0, sqrt(3)/2), a 120
    # degree bend, with the far tangent node 1e307 mm from the TI. The TI lies at x = 1.7e308 + 1e307 = 1.8e308 mm,
    # beyond binary64, although every node is representable.
    t, s = 1e307, math.sqrt(3) / 2
    x1 = 1.7e308  # T1; T2 = TI + t (-0.5, 0, s) has x = x1 + 0.5 t; B = TI + 3t (-0.5, 0, s) has x = x1 - 0.5 t
    model = model_shell("mm")
    model["nodes"] = [node("A", 1.5e308), node("T1", x1), node("T2", x1 + 0.5 * t, 0.0, s * t),
                      node("B", x1 - 0.5 * t, 0.0, 3 * s * t)]
    model["pipe_segments"] = [pipe("S1", "A", "T1", 100.0, 5.0, "mm"), pipe("BEND", "T1", "T2", 100.0, 5.0, "mm"),
                              pipe("S2", "T2", "B", 100.0, 5.0, "mm")]
    model["pipe_segments"][1]["y_reference"] = {"x": 1.0, "y": 0.0, "z": 0.0}
    model["supports"] = [{"id": "SA", "node": "A", "restraints": list(SIX)}]
    model["components"] = [{"id": "bend", "kind": "bend", "node": "T1",
                            "geometry": {"bend_pipe_ref": "BEND", "bend_radius": q(t / math.tan(math.pi / 3), "mm")},
                            "modifiers": {"flexibility_factor_user_value": q(1.0, "none")},
                            "mechanics_interface": {"solver_consumption": "curved_bend_macro_element"}}]
    model["load_cases"] = [{"id": "c", "primitive_loads": [force("f", "B", "global_y", -1.0)]}]
    assert math.isinf(x1 + t)  # the TI is not representable; every node is
    result = render_caepipe_mbf_si(model, title="FAR BEND")
    assert result.status == "refused"
    assert any(item["code"] == "MBF-SI-VALUE-OUT-OF-RANGE" and "tangent intersection" in item["message"]
               for item in result.diagnostics)


# --- pressure (Design §9.4, tranche 2) -----------------------------------------------------------------------------
EXACT = {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"}


def region(region_id, members, value, unit, ends, closures=("transfers_to_wall", "transfers_to_wall")):
    return {"id": region_id, "member_pipe_ids": list(members), "pressure_basis": "internal_differential_zero_external_v1",
            "pressure": q(value, unit), "provenance": "invented pressure for this test",
            "terminals": [{"node_ref": node_id, "closure_transfer": closure, "provenance": "invented closure for this test"}
                          for node_id, closure in zip(ends, closures)]}


def exact(model):
    """`model` under the exact pressure contract: an E/nu material basis, explicit zero corrosion allowances and an
    explicit (empty) region list on every case."""
    model = deepcopy(model)
    model["pressure_contract"] = dict(EXACT)
    for material in model["materials"]:
        material["constitutive_basis"] = "homogeneous_isotropic_E_nu_v1"
        if material.get("poisson_ratio") is None:
            material["poisson_ratio"] = q(0.25, "1")  # E / (2G) - 1 for the shell material
        material.pop("shear_modulus", None)
    for item in model["pipe_segments"]:
        item["section"]["corrosion_allowance"] = q(0.0, item["section"]["wall_thickness"]["unit"])
    for case in model["load_cases"]:
        case.setdefault("pressure_regions", [])
    return model


def pressurized_l(value=2.5e6, unit="Pa", shear="euler_bernoulli"):
    """Invented: the bent L with one region over S1, the bend and S2 in a pressure-only case."""
    model = exact(bent_l())
    model["pressure_contract"]["shear_model"] = shear
    model["load_cases"].insert(0, {"id": "case:p", "primitive_loads": [], "pressure_regions": [
        region("region:L", ("S1", "BEND", "S2"), value, unit, ("A", "B"))]})
    return model


def pressurized_tee():
    """Invented tee in mm with three anchored legs, one region per leg at 10 bar, and a separately supported end."""
    model = exact(branched())
    model["supports"] = [{"id": f"A{n}", "node": f"N{n}", "restraints": list(SIX)} for n in (1, 3, 4)]
    model["load_cases"] = [
        {"id": "case:sus", "primitive_loads": [force("f2", "N2", "global_y", -2500.0)], "pressure_regions": [
            region("region:west", ("P1",), 10.0, "bar", ("N1", "N2")),
            region("region:east", ("P2",), 10.0, "bar", ("N2", "N3")),
            region("region:branch", ("P3",), 10.0, "bar", ("N2", "N4"),
                   ("transfers_to_wall", "separately_supported_or_compensated"))]},
        {"id": "case:th", "primitive_loads": [
            {"id": "t1", "category": "thermal", "target": {"type": "element", "pipe": "P1"}, "direction": "global_x",
             "magnitude": q(80.0, "degC"), "dimension": "temperature_interval"}], "pressure_regions": []}]
    model["combinations"] = []
    return model


def pressurized_us_cantilever():
    """Invented US-customary cantilever with 150 psi over S1, capped at its free end."""
    model = exact(us_customary_cantilever())
    model["load_cases"][0]["pressure_regions"] = [region("region:S1", ("S1",), 150.0, "psi", ("N0", "N1"))]
    return model


def test_a_pressure_region_is_written_as_pressure_1_in_bar():
    result = render_caepipe_mbf_si(pressurized_l(), title="PRESSURE L")
    assert result.status == "written", result.diagnostics
    lines = result.text.split("\r\n")
    assert lines[lines.index("OPPRES") + 1] == "0,0,0,1,0" and lines.index("OPPRES") == lines.index("OPTEMP") + 2
    assert lines[lines.index("LOADS") + 1] == "L1,120,25,0"  # 2.5e6 Pa = 25 bar; T1 = 20 + 100
    assert result.load_case_map == {"case:p": "Sustained (W+P)", "case:th": "Expansion (T1)"}
    assert result.pressure_map == {"contract": {"version": "3.0.0", "mode": "exact_pressure_v3"},
                                   "shear_model": EXACT["shear_model"], "regions": [
        {"id": "region:L", "members": ["S1", "BEND", "S2"], "pressure_bar": 25.0,
         "provenance": "invented pressure for this test",
         "terminals": [{"node_ref": node_id, "closure_transfer": "transfers_to_wall",
                        "provenance": "invented closure for this test"} for node_id in ("A", "B")]}]}
    assert codes(result, "warning") >= {"MBF-SI-SHEAR-MODEL-NOT-EXPORTED"}
    notes = " ".join(result.assumptions)
    assert "1 bar = 1e5 Pa exactly" in notes and "pressure_map" in notes and "OPPRES is written 0,0,0,1,0" in notes
    assert result.to_dict()["pressure_map"] == result.pressure_map


@pytest.mark.parametrize("value, unit, printed", [
    (2.5e6, "Pa", "25"), (1.5, "MPa", "15"), (300.0, "kPa", "3"), (12.5, "bar", "12.5"),
    # 100 psi = 100 x 4.4482216152605 N / 645.16 mm^2 = 444.82216152605 / 64.516 bar = 6.8947572931683...bar.
    (100.0, "psi", "6.894757293"),
])
def test_each_pressure_unit_becomes_bar_by_its_definition(value, unit, printed):
    result = render_caepipe_mbf_si(pressurized_l(value, unit), title="P")
    assert result.text.split("\r\n")[result.text.split("\r\n").index("LOADS") + 1] == f"L1,120,{printed},0"
    exact_bar = Fraction(value) * {"Pa": Fraction(1, 10**5), "MPa": 10, "kPa": Fraction(1, 100), "bar": 1,
                                   "psi": Fraction("4.4482216152605") / Fraction("0.00064516") / 10**5}[unit]
    assert result.pressure_map["regions"][0]["pressure_bar"] == float(exact_bar)
    if unit == "psi":
        assert exact_bar == Fraction("444.82216152605") / Fraction("64.516")
        assert "MBF-SI-UNITS-NORMALIZED" in codes(result, "info")


def test_unpressurized_elements_and_models_write_pressure_0_and_no_oppres():
    tee = render_caepipe_mbf_si(pressurized_tee(), title="TEE")
    assert tee.status == "written", tee.diagnostics
    lines = tee.text.split("\r\n")
    assert lines[lines.index("LOADS") + 1:lines.index("LAYOUT")] == ["L1,100,10,0", "L2,20,10,0"]
    plain = render_caepipe_mbf_si(cantilever(), title="PLAIN")
    assert "OPPRES" not in plain.text.split("\r\n")
    assert plain.pressure_map == {"contract": "none", "shear_model": "euler_bernoulli", "regions": []}
    no_regions = exact(cantilever())  # the contract alone: Pressure 1 = 0 everywhere, the contract in the sidecar
    result = render_caepipe_mbf_si(no_regions, title="CONTRACT ONLY")
    assert "OPPRES" not in result.text.split("\r\n")
    assert result.pressure_map == {"contract": {"version": "3.0.0", "mode": "exact_pressure_v3"},
                                   "shear_model": EXACT["shear_model"], "regions": []}


def test_pressure_refusals_and_omissions():
    thermal = pressurized_l()
    regions = thermal["load_cases"].pop(0)["pressure_regions"]
    thermal["load_cases"][0]["pressure_regions"] = regions
    assert_refused(render_caepipe_mbf_si(thermal, title="P"), "MBF-SI-PRESSURE-CASE-UNSUPPORTED", "Expansion (T1)")
    assert_refused(render_caepipe_mbf_si(pressurized_l(-1.0e5), title="P"), "MBF-SI-PRESSURE-UNSUPPORTED", "negative")
    zero = render_caepipe_mbf_si(pressurized_l(0.0), title="P")
    assert "MBF-SI-PRESSURE-REGION-ZERO" in codes(zero, "warning") and "OPPRES" not in zero.text.split("\r\n")
    no_contract = pressurized_l()
    del no_contract["pressure_contract"]
    assert_refused(render_caepipe_mbf_si(no_contract, title="P"), "MBF-SI-PRESSURE-UNSUPPORTED",
                   "without the exact pressure contract")
    old = pressurized_l()
    old["pressure_contract"]["version"] = "2.0.0"
    assert_refused(render_caepipe_mbf_si(old, title="P"), "MBF-SI-PRESSURE-UNSUPPORTED", "3.0.0/exact_pressure_v3")
    joint = pressurized_l()
    joint["load_cases"][0]["pressure_regions"][0]["member_connector_ids"] = ["joint:1"]
    assert_refused(render_caepipe_mbf_si(joint, title="P"), "MBF-SI-PRESSURE-UNSUPPORTED", "connectors")
    split = pressurized_l()
    split["load_cases"][0]["pressure_regions"][0]["member_pipe_ids"] = ["BEND", "S2"]
    assert_refused(render_caepipe_mbf_si(split, title="P"), "MBF-SI-BEND-UNSUPPORTED", "temperature or pressure")
    unclosed = pressurized_l()
    unclosed["load_cases"][0]["pressure_regions"][0]["terminals"][1]["closure_transfer"] = None
    assert_refused(render_caepipe_mbf_si(unclosed, title="P"), "MBF-SI-INPUT-INVALID", "closure transfer")


def test_an_exact_pressure_section_without_an_allowance_is_refused_not_written_as_zero():
    # VER-002 r2 item 2: the solve refuses such a model (EXACT_PRESSURE_CORROSION_ALLOWANCE_REQUIRED); writing the
    # MBF's 0 and importing it back would author an allowance and solve.
    for pipe_id in ("S2", "S1"):
        model = pressurized_l()
        del next(item for item in model["pipe_segments"] if item["id"] == pipe_id)["section"]["corrosion_allowance"]
        assert_refused(render_caepipe_mbf_si(model, title="P"), "MBF-SI-CORROSION-ALLOWANCE-MISSING", f"pipe:{pipe_id}")
    contract_only = exact(cantilever())  # the contract without regions needs it too
    del contract_only["pipe_segments"][0]["section"]["corrosion_allowance"]
    assert_refused(render_caepipe_mbf_si(contract_only, title="P"), "MBF-SI-CORROSION-ALLOWANCE-MISSING")
    plain = cantilever()  # no contract: no allowance needed
    assert render_caepipe_mbf_si(plain, title="P").status == "written"


# Model document 0.9.0 (DEL-02-01 Design model-document-0.9.0.md §2.1, §2.2, §2.4, §3.5): the writer refuses each member
# it cannot express by name, never dropping it; an exactly axis-aligned direction is the record on that axis.
def current(model):
    model["schema_version"] = current_model_schema_version()
    return model


@pytest.mark.parametrize("change, code, fragment", [
    (lambda m: m["load_cases"][0].update(condition_use={"temperature": "OPE1"}), "MBF-SI-CONDITION-USE-UNSUPPORTED",
     "condition_use"),
    (lambda m: m["load_cases"][0].update(test_condition={"kind": "hydrostatic", "held_supports": "none"}),
     "MBF-SI-TEST-CONDITION-UNSUPPORTED", "test_condition"),
    (lambda m: m["supports"].append({"id": "skew", "node": "N1", "family": "line_stop", "restraints": ["UY"],
                                     "direction": {"x": 0.0, "y": 3.0, "z": 4.0}}),
     "MBF-SI-SUPPORT-DIRECTION-UNSUPPORTED", "support:skew"),
    (lambda m: m["supports"].append({"id": "snub", "node": "N1", "family": "snubber", "restraints": ["UY"]}),
     "MBF-SI-SNUBBER-UNSUPPORTED", "support:snub"),
    (lambda m: m.update(spectra=[{"id": "sp"}]), "MBF-SI-SPECTRUM-UNSUPPORTED", "spectra"),
    (lambda m: m["combinations"].append({"id": "c", "basis": "result_state_subtraction", "minuend_id": "case:tip",
                                         "subtrahend_id": "case:tip", "difference_basis": "operating_minus_sustained"}),
     "MBF-SI-DIFFERENCE-BASIS-UNSUPPORTED", "combination:c"),
    (lambda m: m["combinations"].append({"id": "c", "basis": "result_state_subtraction", "minuend_id": "case:tip",
                                         "subtrahend_id": "case:tip", "subtrahend_support_state": "own"}),
     "MBF-SI-SUBTRAHEND-SUPPORT-STATE-UNSUPPORTED", "combination:c"),
    (lambda m: m["combinations"].append({"id": "c", "basis": "srss", "terms": [{"load_case": "case:tip", "factor": 1.0}]}),
     "MBF-SI-COMBINATION-BASIS-UNSUPPORTED", "srss"),
    (lambda m: m["combinations"].extend([{"id": "c0", "basis": "mechanics", "terms": [{"load_case": "case:tip", "factor": 1.0}]},
                                         {"id": "c", "basis": "mechanics", "terms": [{"combination": "c0", "factor": 1.0}]}]),
     "MBF-SI-COMBINATION-NESTED-UNSUPPORTED", "combination:c"),
    (lambda m: m["combinations"].extend([{"id": "c0", "basis": "mechanics", "terms": [{"load_case": "case:tip", "factor": 1.0}]},
                                         {"id": "c", "basis": "range_envelope", "operand_ids": ["case:tip", "c0"], "mode": "max"}]),
     "MBF-SI-COMBINATION-NESTED-UNSUPPORTED", "combination:c"),
])
def test_model_document_0_9_0_members_are_refused_by_name(change, code, fragment):
    model = current(cantilever())
    assert render_caepipe_mbf_si(model, title="S").status == "written"
    change(model)
    assert_refused(render_caepipe_mbf_si(model, title="S"), code, fragment)


def test_an_axis_aligned_direction_writes_the_record_on_its_axis_and_unused_sets_are_listed_as_omitted():
    reference = current(sprung_cantilever())
    reference["supports"].append({"id": "stop", "node": "N1", "family": "line_stop", "restraints": ["UZ"]})
    directed = deepcopy(reference)
    directed["supports"][-1] = {"id": "stop", "node": "N1", "family": "line_stop", "restraints": ["UX"],
                                "direction": {"x": 0.0, "y": 0.0, "z": -2.0}}
    directed["supports"][1]["direction"] = {"x": 0.0, "y": 5.0, "z": 0.0}
    # Springs and a rigid restraint at one node are refused alike; compare the outcomes, which name the same records.
    written, expected = render_caepipe_mbf_si(directed, title="D"), render_caepipe_mbf_si(reference, title="D")
    assert (written.status, written.text, written.diagnostics) == (expected.status, expected.text, expected.diagnostics)
    plain = current(sprung_cantilever())
    plain["supports"][1]["direction"] = {"x": 0.0, "y": -1.0, "z": 0.0}
    plain_reference = current(sprung_cantilever())
    assert render_caepipe_mbf_si(plain, title="D").text == render_caepipe_mbf_si(plain_reference, title="D").text
    assert render_caepipe_mbf_si(plain, title="D").status == "written"
    # A UX spring with d = (0, 0, 7) is written as the UZ spring.
    rewritten, twin = current(cantilever("mm")), current(cantilever("mm"))
    rewritten["supports"].append({**spring("s", "N1", "UX", 250000.0), "direction": {"x": 0.0, "y": 0.0, "z": 7.0}})
    twin["supports"].append(spring("s", "N1", "UZ", 250000.0))
    result = render_caepipe_mbf_si(rewritten, title="D")
    assert result.status == "written", result.diagnostics
    assert result.text == render_caepipe_mbf_si(twin, title="D").text
    assert "CTRAK(0,0,250)" in result.text
    # Condition sets and assignments that no case uses are not exported and are listed as omitted, never dropped.
    annotated = current(cantilever())
    annotated["condition_sets"] = [{"id": "cs", "provenance": "invented"}]
    annotated["pipe_segments"][0]["condition_set_ref"] = "cs"
    result = render_caepipe_mbf_si(annotated, title="S")
    assert result.status == "written"
    assert result.text == render_caepipe_mbf_si(current(cantilever()), title="S").text
    omitted = {item["entity"] for item in result.loss_report}
    assert {"model.condition_sets", "model.pipe_segments.condition_set_ref"} <= omitted, result.loss_report


def test_a_spring_at_a_node_no_pipe_reaches_is_refused_by_name():
    # Regression: the writer raised KeyError for a spring alone at a node no pipe segment reaches. It is refused like
    # a rigid support there, by name.
    model = sprung_cantilever()
    model["nodes"].append(node("N9", 0.0, 5.0))
    model["supports"].append(spring("spring:N9", "N9", "UY", 1000.0, "N/mm"))
    assert_refused(render_caepipe_mbf_si(model, title="SPRING"), "MBF-SI-TOPOLOGY-UNSUPPORTED",
                   "node:N9", "carries a support or load but no pipe segment")


# --- sidecar format 4: section identities (Design §15, Format 4) -----------------------------------------------------
def test_library_sections_get_their_own_pipe_records_and_section_map_entries():
    # Three pipes of one OD and wall: two library sections and no section are three PIPE records, numbered in layout
    # order; the sidecar's section_map gives each record's section id, or null. The unbound section is listed.
    result = render_caepipe_mbf_si(sectioned_bent_l(), title="SECTIONS")
    assert result.status == "written", result.diagnostics
    assert "PIPE\r\nP1,168.3,7.11,0,0,0,0,0,0\r\nP2,168.3,7.11,0,0,0,0,0,0\r\nP3,168.3,7.11,0,0,0,0,0,0\r\n" in result.text
    assert {pipe_id: entry["section"] for pipe_id, entry in result.element_map.items()} == {
        "S1": "P1", "BEND": "P1", "S2": "P2", "S3": "P3"}
    sidecar = result.sidecar("0" * 64)
    assert sidecar["sidecar_format"] == 4
    assert sidecar["section_map"] == {"P1": {"swbpipe_section": "section:A"}, "P2": {"swbpipe_section": "section:B"},
                                      "P3": {"swbpipe_section": None}}
    entities = {item["entity"]: item for item in result.loss_report}
    assert "section:unused" in entities["model.sections"]["reason"]
    assert entities["model.sections.name"]["category"] == "omitted"
    assert "model.pipe_segments.section_ref" not in entities
    # Without library sections the records are keyed by OD and wall alone, as before: one record.
    plain = render_caepipe_mbf_si(bent_l(), title="SECTIONS")
    assert "PIPE\r\nP1,168.3,7.11,0,0,0,0,0,0\r\nLOADS" in plain.text
    assert plain.sidecar("0" * 64)["section_map"] == {"P1": {"swbpipe_section": None}}


def test_a_bend_whose_pipes_differ_only_in_section_ref_is_refused():
    # Keying records by section_ref can refuse a bend row whose straight and bend pipe differ in section_ref alone
    # (bend authoring does not produce one: OP-BEND-CORNER-SECTION-MISMATCH).
    model = sectioned_bent_l()
    next(item for item in model["pipe_segments"] if item["id"] == "S1").pop("section_ref")
    assert_refused(render_caepipe_mbf_si(model, title="SECTIONS"), "MBF-SI-BEND-UNSUPPORTED",
                   "pipe:BEND", "section_ref alone")


def _with_section(change):
    model = sectioned_bent_l()
    change(model)
    return render_caepipe_mbf_si(model, title="SECTIONS")


def _section(model, section_id):
    return next(item for item in model["sections"] if item["id"] == section_id)


@pytest.mark.parametrize("code, change", [
    ("MBF-SI-REFERENCE-UNRESOLVED", lambda m: m["pipe_segments"][3].update(section_ref="section:none")),
    ("MBF-SI-REFERENCE-UNRESOLVED", lambda m: m["pipe_segments"][3].update(section_ref=" 　")),
    ("MBF-SI-REFERENCE-UNRESOLVED", lambda m: m["pipe_segments"][3].update(section_ref=7)),
    ("MBF-SI-REFERENCE-UNRESOLVED", lambda m: m["sections"].append(library_section("section:B"))),
    ("MBF-SI-SECTION-BINDING-UNSUPPORTED", lambda m: _section(m, "section:B").update(section_type="beam")),
    ("MBF-SI-SECTION-BINDING-UNSUPPORTED",
     lambda m: _section(m, "section:B")["properties"].update(mill_tolerance=q(0.5, "mm"))),
    ("MBF-SI-SECTION-BINDING-UNSUPPORTED", lambda m: _section(m, "section:B")["properties"].pop("wall_thickness")),
    ("MBF-SI-SECTION-BINDING-UNSUPPORTED",
     lambda m: _section(m, "section:B")["properties"]["wall_thickness"].update(value=True)),
    ("MBF-SI-SECTION-REFERENCE-STALE",
     lambda m: _section(m, "section:B")["properties"]["wall_thickness"].update(value=0.008)),
    ("MBF-SI-SECTION-REFERENCE-STALE",
     lambda m: _section(m, "section:B")["properties"].update(outside_diameter=q(168.3, "mm"))),
])
def test_a_section_ref_the_solve_would_not_bind_is_refused_by_name(code, change):
    assert_refused(_with_section(change), code, "pipe:S")


def test_an_integer_library_value_equal_to_the_inline_float_binds():
    # The solve compares the values as numbers (270 and 270.0 are one value), and the unit as text.
    def change(model):
        for item in model["pipe_segments"]:
            item["section"]["outside_diameter"] = q(168.0, "mm")
            item["section"]["wall_thickness"] = q(7.0, "mm")
        for section in model["sections"]:
            section["properties"] = {"outside_diameter": q(168, "mm"), "wall_thickness": q(7, "mm")}
    assert _with_section(change).status == "written"
