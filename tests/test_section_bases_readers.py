"""T4-U6/U8 (DEL-05-03 `section-bases-and-timoshenko.md` §1-§3.1, §5, §8): the test-only verifiers read the D-5
section bases and the D-6 beam theory, and refuse each reader mutant.

The documents are synthetic: a committed pressure-1 fixture (or arc corpus envelope) is shaped in memory to §2.5's
interface (both stress families, basis tokens and id segments, the D-5 `pipe_sections`, per-basis maximum records
and headlines) with coincident bases, then given distinct bases (a nonzero corrosion allowance) or a Timoshenko
selection, then mutated. No committed fixture is read as current §2.5 output, and none is written. The Rust
reader's twins of these mutations are `core/reporting/result_export/tests/section_bases.rs`.
"""
from __future__ import annotations

from copy import deepcopy
import json
import math
from pathlib import Path
import struct

import pytest

from tests.verifiers import physics_source as ps
from tests.verifiers import pressure_1
from tests.verifiers import retained_precision as rp
from tests.verifiers import section_basis as sb

ROOT = Path(__file__).resolve().parents[1]
BASE = ROOT / "fixtures/results/pressure_connected_mechanics_dense.json"
ARC = ROOT / "fixtures/results/pressure_v3_arc_reader_corpus.json"
UNDERSCORE_IDS = ("result:pressure-exact:", "result:elastic-maximum:")


def _suffix(row_id: str, basis: str) -> str:
    return f"{row_id}:{basis}" if row_id.startswith(UNDERSCORE_IDS) else f"{row_id}:{sb.hyphenated(basis)}"


def _section(old: dict) -> dict:
    """The D-5 entry of a committed single-section entry, its effective wall taken as the nominal wall (c_m = c_a = 0)."""
    annulus = {"wall_m": old["effective_wall_thickness_m"], "ri_m": old["ri_m"], "As_m2": old["As_m2"], "I_m4": old["I_m4"], "J_m4": old["J_m4"], "Z_m3": old["Z_m3"]}
    return {"pipe_id": old["pipe_id"], "geometry_basis": sb.GEOMETRY_BASIS, "outside_diameter_m": old["outside_diameter_m"], "ro_m": old["ro_m"],
            "wall_thickness_m": old["effective_wall_thickness_m"], "mill_tolerance_m": 0.0, "corrosion_allowance_m": 0.0,
            "nominal_section": dict(annulus), "reduced_section": dict(annulus), "pressure_bore": {"ri_m": old["ri_m"], "Ai_m2": old["Ai_m2"]},
            "beam_theory": {"model": sb.EULER_BERNOULLI}}


def shape(envelope: dict) -> dict:
    """§2.5 with coincident bases: every stress row twice (the nominal row then its reduced twin), named by token and id."""
    doc = deepcopy(envelope)
    evidence = doc["contract_evidence"]
    renamed: dict[str, list[str]] = {}
    rows = []
    for row in doc["results"]:
        if row["kind"] not in sb.STRESS_KINDS:
            rows.append(row)
            continue
        twins = []
        for basis in sb.BASES:
            twin = deepcopy(row)
            twin["id"] = _suffix(row["id"], basis)
            twin["metadata"]["basis"] = f"{row['metadata']['basis']}; section_basis={basis}"
            twins.append(twin)
        renamed[row["id"]] = [t["id"] for t in twins]
        rows.extend(twins)
    doc["results"] = rows
    for case in evidence["exact_cases"]:
        case["pipe_sections"] = [_section(s) for s in case["pipe_sections"]]
        old = case["stress_maximum_coverage"]
        case["stress_maximum_coverage"] = [{"section_basis": b, "complete": old["complete"], "unavailable_pipe_ids": list(old["unavailable_pipe_ids"])} for b in sb.BASES]
        extrema = []
        for ex in case["pipe_stress_extrema"]:
            for basis in sb.BASES:
                extrema.append({"pipe_id": ex["pipe_id"], "section_basis": basis, **{k: v for k, v in ex.items() if k != "pipe_id"}, "result_id": _suffix(ex["result_id"], basis)})
        case["pipe_stress_extrema"] = extrema
    for region in evidence["pressure"]:
        geometry = []
        for g in region["geometry"]:
            arc = {k: g[k] for k in ("member_kind", "arc_end_tangents_global") if k in g}
            geometry.append({**_section(g), "traversal_forward": g["traversal_forward"], **arc})
        region["geometry"] = geometry
        region["result_ids"] = [new for rid in region["result_ids"] for new in renamed.get(rid, [rid])]
    summary = {}
    for key, value in doc["summary"].items():
        if key == "max_open_formula_stress":
            summary[key] = None
            for basis in sb.BASES:
                summary[f"{key}_{basis}"] = None if value is None else {**value, "result_ref": _suffix(value["result_ref"], basis)}
        else:
            summary[key] = value
    doc["summary"] = summary
    return doc


def _rows(doc):
    return {row["id"]: row for row in doc["results"]}


def _bore_bits(a: float, b: float) -> tuple[str, str]:
    hi = a - b
    b_virtual = a - hi
    a_virtual = hi + b_virtual
    lo = (a - a_virtual) + (b_virtual - b)
    return struct.pack(">d", hi).hex(), struct.pack(">d", 0.0 if lo == 0 else lo).hex()


def distinct(doc: dict, allowance: float = 0.001) -> dict:
    """A nonzero corrosion allowance on every pipe: the reduced section, the pressure bore, the reduced stress rows
    (the same resultant on the reduced section), the reduced Lame rows (the reduced annulus) and the bore bits."""
    doc = deepcopy(doc)
    evidence = doc["contract_evidence"]
    entries = [s for case in evidence["exact_cases"] for s in case["pipe_sections"]] + [g for region in evidence["pressure"] for g in region["geometry"]]
    for s in entries:
        od, t = s["outside_diameter_m"], s["wall_thickness_m"]
        t_r = sb.reduced_wall(t, s["mill_tolerance_m"], allowance)
        ro, ri = od * 0.5, od * 0.5 - t_r
        area = (math.pi * t_r) * (od - t_r)
        second = (area * ((ro * ro) + (ri * ri))) * 0.25
        s["corrosion_allowance_m"] = allowance
        s["reduced_section"] = {"wall_m": t_r, "ri_m": ri, "As_m2": area, "I_m4": second, "J_m4": second * 2, "Z_m3": second / ro}
        s["pressure_bore"] = {"ri_m": ri, "Ai_m2": (math.pi * ri) * ri}
    sections = {(case["load_case_id"], s["pipe_id"]): s for case in evidence["exact_cases"] for s in case["pipe_sections"]}
    pressures = {(region["load_case_id"], pid): region["p_pa"] for region in evidence["pressure"] for pid in region["member_pipe_ids"]}
    rows = _rows(doc)
    for row in doc["results"]:
        if row["kind"] not in sb.STRESS_KINDS or not row["id"].endswith((":reduced_section", ":reduced-section")):
            continue
        s = sections[row["basis_ref"]["ref_id"], row["entity_ref"]]
        nominal = rows[sb.twin_id(row["id"])]
        component = row["metadata"]["component"]
        if row["kind"] in sb.DIVISOR:
            key = sb.DIVISOR[row["kind"]]
            row["value"] = nominal["value"] * s["nominal_section"][key] / s["reduced_section"][key]
        elif row["kind"] == "pipe_lame_hoop_stress_v2":
            p, annulus = pressures[row["basis_ref"]["ref_id"], row["entity_ref"]], s["reduced_section"]
            outer = 2.0 * (p * ((math.pi * annulus["ri_m"]) * annulus["ri_m"])) / annulus["As_m2"]
            row["value"] = outer if component == "lame_outer_hoop_stress" else outer + p
    for case in evidence["exact_cases"]:
        for group in case["pressure_rhs_assembly"]["groups"]:
            if "source_inner_radius_hi_bits" in group:
                s = sections[case["load_case_id"], group["terms"][0]["pipe_id"]]
                group["source_inner_radius_hi_bits"], group["source_inner_radius_lo_bits"] = _bore_bits(s["outside_diameter_m"] * 0.5, s["reduced_section"]["wall_m"])
    return doc


def timoshenko(doc: dict, *, radius: str = "nominal_section", area: str = "nominal_section", factor=None) -> dict:
    """Select timoshenko_annulus_energy_v1 for every straight member (arcs stay Euler-Bernoulli)."""
    doc = deepcopy(doc)
    evidence = doc["contract_evidence"]
    arcs = {(region["load_case_id"], g["pipe_id"]) for region in evidence["pressure"] for g in region["geometry"] if "member_kind" in g}
    nus = {(case["load_case_id"], m["pipe_id"]): m["nu"] for case in evidence["exact_cases"] for m in case["pipe_materials"]}
    beams = {}
    for case in evidence["exact_cases"]:
        for s in case["pipe_sections"]:
            key = (case["load_case_id"], s["pipe_id"])
            if key in arcs:
                continue
            kappa = sb.timoshenko_shear_factor(s[radius]["ri_m"], s["ro_m"], nus[key]) if factor is None else factor
            s["beam_theory"] = beams[key] = {"model": sb.TIMOSHENKO, "shear_factor": kappa, "shear_area_m2": kappa * s[area]["As_m2"]}
    for region in evidence["pressure"]:
        for g in region["geometry"]:
            if (region["load_case_id"], g["pipe_id"]) in beams:
                g["beam_theory"] = deepcopy(beams[region["load_case_id"], g["pipe_id"]])
    return doc


@pytest.fixture(scope="module")
def base():
    return shape(json.loads(BASE.read_text()))


@pytest.fixture(scope="module")
def arc():
    return shape(json.loads(ARC.read_text())["cases"][0]["envelope"])


def _refused(doc, detail):
    with pytest.raises(ValueError, match=detail):
        pressure_1.validate_pressure_evidence(doc)


def test_the_shaped_interface_is_read_on_coincident_and_distinct_bases_under_both_shear_models(base, arc):
    for doc in (base, distinct(base), timoshenko(base), timoshenko(distinct(base)), arc, timoshenko(arc), distinct(arc)):
        pressure_1.validate_pressure_evidence(doc)
    # Coincident bases: every twin pair is bit-equal; distinct bases move every reduced linear stress.
    rows = _rows(distinct(base))
    moved = [r for r in rows.values() if r["kind"] == "element_local_bending_normal_stress_y" and r["id"].endswith("reduced-section") and r["value"] != 0]
    assert moved and all(r["value"] != rows[sb.twin_id(r["id"])]["value"] for r in moved)


def test_pressure_1_reader_refuses_cross_basis_stress(base):
    # §8: one basis for both families (reduced rows copied from nominal) on distinct bases.
    doc = distinct(base)
    rows = _rows(doc)
    for row in doc["results"]:
        if row["kind"] == "element_local_bending_normal_stress_y" and row["id"].endswith(":reduced-section") and row["value"] != 0:
            row["value"] = rows[sb.twin_id(row["id"])]["value"]
            break
    _refused(doc, "stress basis section")
    # The nominal-basis Lame hoop evaluated at the pressure bore (§8).
    doc = distinct(base)
    for row in doc["results"]:
        if row["kind"] == "pipe_lame_hoop_stress_v2" and row["id"].endswith(":nominal_section"):
            row["value"] = _rows(doc)[sb.twin_id(row["id"])]["value"]
            break
    _refused(doc, "stress basis section")
    # A reduced token on a nominal id.
    doc = deepcopy(base)
    row = next(r for r in doc["results"] if r["kind"] == "pipe_axial_membrane_stress_v2" and r["id"].endswith(":nominal_section"))
    row["metadata"]["basis"] = row["metadata"]["basis"].replace("nominal_section", "reduced_section")
    _refused(doc, "stress row section basis|pressure station coverage")
    doc = deepcopy(base)
    row = next(r for r in doc["results"] if r["kind"] == "element_local_torsional_shear_stress" and r["id"].endswith(":nominal-section"))
    row["metadata"]["basis"] = row["metadata"]["basis"].replace("nominal_section", "reduced_section")
    _refused(doc, "stress row section basis")


def test_pressure_1_reader_refuses_cross_basis_headline(base):
    doc = deepcopy(base)
    reduced = doc["summary"]["max_open_formula_stress_reduced_section"]
    reduced["result_ref"] = reduced["result_ref"].replace(":reduced_section", ":nominal_section")
    _refused(doc, "stress maximum headline binding")
    doc = deepcopy(base)
    doc["summary"]["max_open_formula_stress"] = deepcopy(doc["summary"]["max_open_formula_stress_nominal_section"])
    _refused(doc, "stress maximum headline bases")
    doc = deepcopy(base)
    del doc["summary"]["max_open_formula_stress_reduced_section"]
    _refused(doc, "stress maximum headline bases")


def test_pressure_1_reader_refuses_a_non_maximal_basis_headline(base):
    doc = deepcopy(base)
    rows = _rows(doc)
    maxima = sorted((r for r in rows.values() if r["kind"] == "pipe_elastic_normal_stress_maximum_v2" and r["id"].endswith(":reduced_section")), key=lambda r: r["value"])
    assert len(maxima) == 2 and maxima[0]["value"] < maxima[1]["value"]
    doc["summary"]["max_open_formula_stress_reduced_section"] = {"value": maxima[0]["value"], "unit": "Pa", "location_ref": maxima[0]["entity_ref"], "result_ref": maxima[0]["id"]}
    _refused(doc, "stress maximum headline binding")
    # Incomplete coverage on one basis withholds that basis's headline.
    doc = deepcopy(base)
    case = doc["contract_evidence"]["exact_cases"][1]
    pipe = case["pipe_sections"][0]["pipe_id"]
    case["stress_maximum_coverage"][1] = {"section_basis": sb.REDUCED, "complete": False, "unavailable_pipe_ids": [pipe]}
    case["pipe_stress_extrema"] = [ex for ex in case["pipe_stress_extrema"] if ex["section_basis"] == sb.NOMINAL]
    _refused(doc, "incomplete stress maximum headline|unbound extrema row")


def test_pressure_1_reader_refuses_a_missing_reduced_twin(base):
    doc = deepcopy(base)
    gone = next(r["id"] for r in doc["results"] if r["kind"] == "element_local_torsional_shear_stress" and r["id"].endswith(":reduced-section"))
    doc["results"] = [r for r in doc["results"] if r["id"] != gone]
    _refused(doc, "stress basis twin")
    doc = deepcopy(base)
    gone = next(r["id"] for r in doc["results"] if r["kind"] == "pipe_axial_membrane_stress_v2" and r["id"].endswith(":reduced_section"))
    doc["results"] = [r for r in doc["results"] if r["id"] != gone]
    for region in doc["contract_evidence"]["pressure"]:
        region["result_ids"] = [i for i in region["result_ids"] if i != gone]
    _refused(doc, "pressure station coverage|stress basis twin")


def test_pressure_1_reader_refuses_a_wrong_kappa_a_cowper_kappa_or_kappa_on_the_reduced_section(base):
    good = timoshenko(distinct(base))
    pressure_1.validate_pressure_evidence(good)
    section = good["contract_evidence"]["exact_cases"][0]["pipe_sections"][0]
    nu = good["contract_evidence"]["exact_cases"][0]["pipe_materials"][0]["nu"]
    m = section["nominal_section"]["ri_m"] / section["ro_m"]
    cowper = 6 * (1 + nu) * (1 + m * m) ** 2 / ((7 + 6 * nu) * (1 + m * m) ** 2 + (20 + 12 * nu) * m * m)
    for doc in (timoshenko(distinct(base), factor=math.nextafter(section["beam_theory"]["shear_factor"], 1.0)),
                timoshenko(distinct(base), factor=cowper), timoshenko(distinct(base), factor=5 / 6),
                timoshenko(distinct(base), radius="reduced_section"), timoshenko(distinct(base), area="reduced_section")):
        _refused(doc, "timoshenko shear factor")


def test_pressure_1_reader_refuses_an_arc_claiming_timoshenko(arc):
    doc = timoshenko(arc)
    region = next(r for r in doc["contract_evidence"]["pressure"] if any("member_kind" in g for g in r["geometry"]))
    bend = next(g for g in region["geometry"] if "member_kind" in g)
    nu = next(m["nu"] for c in doc["contract_evidence"]["exact_cases"] for m in c["pipe_materials"] if m["pipe_id"] == bend["pipe_id"])
    kappa = sb.timoshenko_shear_factor(bend["nominal_section"]["ri_m"], bend["ro_m"], nu)
    claim = {"model": sb.TIMOSHENKO, "shear_factor": kappa, "shear_area_m2": kappa * bend["nominal_section"]["As_m2"]}
    bend["beam_theory"] = claim
    _refused(doc, "arc beam theory")
    doc = timoshenko(arc)
    for case in doc["contract_evidence"]["exact_cases"]:
        for s in case["pipe_sections"]:
            if s["pipe_id"] == bend["pipe_id"]:
                s["beam_theory"] = deepcopy(claim)
    _refused(doc, "arc beam theory|geometry disagreement")


def test_pressure_1_reader_refuses_a_basis_less_thin_wall_row_on_an_exact_envelope(base):
    doc = deepcopy(base)
    case = doc["contract_evidence"]["exact_cases"][0]
    doc["results"].append({"id": "result:thin-wall:hoop", "kind": "pipe_section_pressure_hoop_stress", "value": 1.0, "unit": "MPa",
                           "entity_ref": case["pipe_sections"][0]["pipe_id"], "basis_ref": {"ref_type": "load_case", "ref_id": case["load_case_id"]},
                           "metadata": {"component": "pressure_hoop_stress", "coordinate_system": "pipe_section", "location": "midspan",
                                        "basis": "recovered_from_open_mechanics_stress_components", "sign_convention": "thin wall"}})
    _refused(doc, "thin-wall row names no section basis")
    # A bare recovery token on a stress row names no basis either.
    doc = deepcopy(base)
    row = next(r for r in doc["results"] if r["kind"] == "element_local_bending_normal_stress_z")
    row["metadata"]["basis"] = row["metadata"]["basis"].split("; section_basis=")[0]
    _refused(doc, "stress row section basis")


def test_the_d5_geometry_is_rederived_in_its_fixed_order(base):
    for path, value, detail in (
        (("reduced_section", "wall_m"), lambda s: s["reduced_section"]["wall_m"] * (1 + 2 ** -50), "section source binding"),
        (("nominal_section", "ri_m"), lambda s: s["pressure_bore"]["ri_m"], "section source binding"),
        (("pressure_bore", "ri_m"), lambda s: s["nominal_section"]["ri_m"], "section bases"),
        (("corrosion_allowance_m",), lambda s: s["wall_thickness_m"], "section allowance"),
        (("corrosion_allowance_m",), lambda s: -0.001, "section range"),
    ):
        doc = distinct(base)
        for case in doc["contract_evidence"]["exact_cases"]:
            for s in case["pipe_sections"]:
                target = s
                for key in path[:-1]:
                    target = target[key]
                target[path[-1]] = value(s)
        _refused(doc, detail)
    # The ruled order (t - c_m) - c_a: where it and OD/2 - t + c_m + c_a round apart, the evidence radius is the former.
    t, cm, ca, od = 0.0071, 0.000889, 0.0005, 0.1683
    ruled = od * 0.5 - sb.reduced_wall(t, cm, ca)
    other = od * 0.5 - t + cm + ca
    assert ruled == od * 0.5 - ((t - cm) - ca)
    if ruled != other:
        doc = distinct(base)
        s = doc["contract_evidence"]["exact_cases"][0]["pipe_sections"][0]
        s["reduced_section"]["ri_m"] = s["outside_diameter_m"] * 0.5 - s["wall_thickness_m"] + s["corrosion_allowance_m"]
        if s["reduced_section"]["ri_m"] != s["outside_diameter_m"] * 0.5 - s["reduced_section"]["wall_m"]:
            _refused(doc, "section source binding")
    # Coincident walls with different leaves are one section stated twice.
    doc = deepcopy(base)
    s = doc["contract_evidence"]["exact_cases"][0]["pipe_sections"][0]
    s["reduced_section"]["As_m2"] = math.nextafter(s["reduced_section"]["As_m2"], 1.0)
    _refused(doc, "section bases")


def test_pressure_source_1_and_physics_retained_1_refuse_nonzero_mill_tolerance():
    request = json.loads((ROOT / "fixtures/product_preview/pressure_source/physics_source/n05.request.json").read_text())
    invocation = {"request": request, "solver_mode": "sparse_interactive"}
    contract = ps.SourceContract.PRESSURE_V3
    contract.check_invocation(invocation)
    changed = deepcopy(invocation)
    changed["request"]["model"]["pipe_segments"][0]["section"]["mill_tolerance"] = {"value": 0.5, "unit": "mm"}
    with pytest.raises(ValueError, match="PHYSICS_SOURCE_ACTUAL_MILL_TOLERANCE"):
        contract.check_invocation(changed)
    for value in ({"value": 0, "unit": "mm"}, None):
        zero = deepcopy(invocation)
        zero["request"]["model"]["pipe_segments"][0]["section"]["mill_tolerance"] = value
        contract.check_invocation(zero)
    for bad in ({"value": 1, "unit": "N"}, {"value": "0", "unit": "mm"}):
        broken = deepcopy(invocation)
        broken["request"]["model"]["pipe_segments"][0]["section"]["mill_tolerance"] = bad
        with pytest.raises(ValueError, match="PHYSICS_SOURCE_ACTUAL_MILL_TOLERANCE"):
            contract.check_invocation(broken)
    # physics-retained-1's G8 names the nonzero mill tolerance on the exact route.
    with pytest.raises(rp.RetainedPrecisionError) as refused:
        rp._exact_mill_tolerances(changed["request"]["model"])
    assert (refused.value.gate, refused.value.code, refused.value.detail) == ("G8", "RETAINED_PRECISION_INVOCATION_MISMATCH", rp.MILL_TOLERANCE_DECLINE)
    rp._exact_mill_tolerances(invocation["request"]["model"])
