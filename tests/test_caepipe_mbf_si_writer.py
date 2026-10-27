#!/usr/bin/env python3
"""Focused tests for the DEL-17-04 SI CAEPIPE MBF pilot writer (invented models only)."""

from __future__ import annotations

from copy import deepcopy
from fractions import Fraction
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
from core.handoff.caepipe_mbf.si_writer import MODEL_SCHEMA_PATH, current_model_schema_version  # noqa: E402


SIX = ["UX", "UY", "UZ", "RX", "RY", "RZ"]


def q(value, unit):
    return {"value": value, "unit": unit}


def model_shell(length_unit="m"):
    return {
        "schema_version": "0.8.0",
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
    assert result.diagnostics == []
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
    assert codes(result, "warning") == {"MBF-SI-COMBINATION-NOT-EXPORTED"}
    assert [item["entity"] for item in result.loss_report] == ["combination:combo:ope"]
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


def test_spring_support_is_refused():
    model = cantilever()
    model["supports"].append({"id": "spring:N1", "node": "N1", "family": "spring", "restraints": ["UY"],
                              "stiffness": {"dof": "UY", "value": q(1.0e6, "N/m")}})
    assert_refused(render_caepipe_mbf_si(model, title="S"), "MBF-SI-SUPPORT-UNSUPPORTED", "support:spring:N1")


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


def test_full_six_dof_restraint_from_two_supports_is_an_anchor_not_a_rotational_restraint():
    model = cantilever()
    model["supports"] = [{"id": "T", "node": "N0", "restraints": ["UX", "UY", "UZ"]},
                         {"id": "R", "node": "N0", "restraints": ["RX", "RY", "RZ"]}]
    result = render_caepipe_mbf_si(model, title="SPLIT")
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
    for version in ("0.7.0", "0.1.0"):
        model = cantilever()
        model["schema_version"] = version
        result = render_caepipe_mbf_si(model, title="Invented retired cantilever")
        assert (result.status, result.text) == ("refused", None), version
        refusal = [d for d in result.diagnostics if d["code"] == "MODEL_SCHEMA_VERSION_RETIRED"]
        assert refusal == [{"code": "MODEL_SCHEMA_VERSION_RETIRED", "severity": "error", "entity": "model.schema_version",
                            "message": f"model document {version} is retired; re-author it as model document 0.8.0 (a pressure-free model omits pressure_contract). Nothing is migrated."}]


def test_a_newer_or_unversioned_model_document_is_refused():
    for version in ("0.9.0", None):
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
    assert result.support_map["A1"]["family"] == "anchor"
    assert result.support_map["A3"]["family"] is None  # authored without a family
    assert result.support_map["R4"]["family"] is None


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
        return result.material_map["M1"].get("origin")

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
    # The same holds for thermal changes summed on one element: 100 + 1e-12 - 100 C = 1e-12 C.
    model = cantilever()
    model["load_cases"] = [{"id": "case:th", "primitive_loads": [
        {"id": f"t{index}", "category": "thermal", "target": {"type": "element", "pipe": "S1"}, "direction": "global_x",
         "magnitude": q(value, "degC"), "dimension": "temperature_interval"} for index, value in enumerate((100.0, 1e-12, -100.0))]}]
    result = render_caepipe_mbf_si(model, title="DT")
    assert result.status == "written", result.diagnostics
    assert "L1,20,0,0" in result.text.split("\r\n")  # T1 = 20 + 1e-12 prints as 20


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
