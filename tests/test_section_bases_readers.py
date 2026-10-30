"""T4-U6/U8 (DEL-05-03 `section-bases-and-timoshenko.md` §1-§3.1, §5, §8): the test-only verifiers read the D-5
section bases and the D-6 beam theory, and refuse each reader mutant.

The documents are the regenerated committed pressure-2 fixtures, which are the producer's §2.5 output (both stress
families, basis tokens and id segments, the D-5 `pipe_sections` with each realized arc's marker, per-basis maximum
records and headlines): `pressure_connected_mechanics_dense` (coincident bases) and the arc corpus's envelopes (their
I7 sketches carry a mill tolerance, so their bases differ). Each is then given distinct bases (a nonzero corrosion
allowance) or a Timoshenko selection, then mutated, and (T4-U4b) the section-extreme and transverse shear rows formed
from the moved rows are re-formed by the verifier's fixed sequences (``_reform_u4b``). An arc's certified maximum cannot
be re-formed here, so arc documents with an allowance are the producer's own: the arc corpus's directional-SIF case
carries 0.0005 m on every pipe. No fixture is written. The Rust reader's twins of these mutations
are `core/reporting/result_export/tests/section_bases.rs`.

The arc-marker tests (§2.5's `member_kind = "realized_arc"` on an arc's case section) build their documents in memory
in the current D-5 shape and read no committed fixture; their Rust twins are
`core/reporting/result_export/src/arc_marker_tests.rs`.
"""
from __future__ import annotations

from copy import deepcopy
import json
import math
from pathlib import Path
import struct

import pytest

from tests.verifiers import physics_evidence as physical
from tests.verifiers import physics_source as ps
from tests.verifiers import pressure_2
from tests.verifiers import reader_verdict as rv
from tests.verifiers import retained_precision as rp
from tests.verifiers import section_basis as sb
from tests.verifiers import source_blocks
from tests.verifiers import u4b

ROOT = Path(__file__).resolve().parents[1]
BASE = ROOT / "fixtures/results/pressure_connected_mechanics_dense.json"
ARC = ROOT / "fixtures/results/pressure_v3_arc_reader_corpus.json"


def _unmarked(envelope: dict, pipe: str) -> dict:
    """``envelope`` with the realized-arc marker removed from ``pipe``'s case sections."""
    doc = deepcopy(envelope)
    for case in doc["contract_evidence"]["exact_cases"]:
        for s in case["pipe_sections"]:
            if s["pipe_id"] == pipe:
                assert s.pop(sb.MEMBER_KIND) == sb.REALIZED_ARC, pipe
    return doc


def _rows(doc):
    return {row["id"]: row for row in doc["results"]}


def _bore_bits(a: float, b: float) -> tuple[str, str]:
    hi = a - b
    b_virtual = a - hi
    a_virtual = hi + b_virtual
    lo = (a - a_virtual) + (b_virtual - b)
    return struct.pack(">d", hi).hex(), struct.pack(">d", 0.0 if lo == 0 else lo).hex()


def _reform_u4b(doc: dict) -> dict:
    """T4-U4b: re-forms each straight member's sigma+/- and transverse shear rows from the published rows of the same
    station and basis (and the basis annulus), by the verifier's fixed sequences, after a helper or a mutation moved
    their operands. Realized arcs are left as published (``distinct`` is for straight-only documents)."""
    evidence = doc["contract_evidence"]
    sections = {(case["load_case_id"], s["pipe_id"]): s for case in evidence["exact_cases"] for s in case["pipe_sections"]}
    nus = {(case["load_case_id"], m["pipe_id"]): m["nu"] for case in evidence["exact_cases"] for m in case["pipe_materials"]}
    index = {}
    for row in doc["results"]:
        metadata = row.get("metadata") or {}
        token = sb.split_token(metadata.get("basis"))
        basis = token[1] if token else None
        index[((row.get("basis_ref") or {}).get("ref_id"), row["entity_ref"], row["kind"], metadata.get("component"), metadata.get("location"), basis)] = row
    def value(key, kind, component, location, basis):
        row = index.get((*key, kind, component, location, basis))
        return None if row is None else row["value"]
    def force(key, kind, location):
        return next((r["value"] for k, r in index.items() if k[:3] == (*key, kind) and k[4] == location), None)
    for row in doc["results"]:
        if row["kind"] not in (u4b.EXTREME, u4b.SHEAR_MAX):
            continue
        key = (row["basis_ref"]["ref_id"], row["entity_ref"])
        section, location = sections[key], row["metadata"]["location"]
        if sb.MEMBER_KIND in section:
            continue
        basis = sb.split_token(row["metadata"]["basis"])[1]
        if row["kind"] == u4b.EXTREME:
            y = value(key, "element_local_bending_normal_stress_y", "bending_normal_stress_y", location, basis)
            z = value(key, "element_local_bending_normal_stress_z", "bending_normal_stress_z", location, basis)
            n = value(key, "element_local_axial_normal_stress", "axial_normal_stress", location, basis)
            if n is None:
                n = value(key, "pipe_axial_membrane_stress_v2", "axial_membrane_stress", location, basis) / 1e6
            plus, minus = u4b.extremes(n, y, z)
            row["value"] = plus if row["metadata"]["component"] == "sigma_plus" else minus
        else:
            factor = u4b.shear_factor(nus[key], section["ro_m"], section[basis]["ri_m"], section[basis]["I_m4"])
            vy = force(key, "element_local_shear_force_y", location)
            vz = force(key, "element_local_shear_force_z", location)
            row["value"] = u4b.shear_maximum(vy, vz, factor)
    return doc


def distinct(doc: dict, allowance: float = 0.001) -> dict:
    """A nonzero corrosion allowance on every pipe of a straight-only document: the reduced section, the pressure bore,
    the reduced stress rows (the same resultant on the reduced section), the reduced Lame rows (the reduced annulus),
    the bore bits and (T4-U4b) the re-formed section-extreme and shear rows."""
    doc = deepcopy(doc)
    evidence = doc["contract_evidence"]
    entries = [s for case in evidence["exact_cases"] for s in case["pipe_sections"]] + [g for region in evidence["pressure"] for g in region["geometry"]]
    assert not any(sb.MEMBER_KIND in s for s in entries), "an arc's certified maximum cannot be re-formed"
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
    return _reform_u4b(doc)


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
            if key in arcs or sb.MEMBER_KIND in s:
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
    return json.loads(BASE.read_text())


@pytest.fixture(scope="module")
def arc():
    return json.loads(ARC.read_text())["cases"][0]["envelope"]


def test_the_fixtures_are_the_d5_interface_with_the_expected_bases(base, arc):
    """The regenerated fixtures carry §2.5's interface; the connected fixture's bases coincide (no mill tolerance or
    allowance), the arc's differ (its I7 sketch's mill tolerance)."""
    for doc, coincident in ((base, True), (arc, False)):
        assert doc["summary"]["max_open_formula_stress"] is None
        for case in doc["contract_evidence"]["exact_cases"]:
            assert len(case["stress_maximum_coverage"]) == 2
            for s in case["pipe_sections"]:
                assert s["geometry_basis"] == sb.GEOMETRY_BASIS
                assert (s["nominal_section"] == s["reduced_section"]) == coincident, s["pipe_id"]


def _refused(doc, detail):
    with pytest.raises(ValueError, match=detail):
        pressure_2.validate_pressure_evidence(doc)


def test_the_shaped_interface_is_read_on_coincident_and_distinct_bases_under_both_shear_models(base, arc):
    allowed = _arc_corpus_entry("U2-L-ANCH-PTW-K2 directional SIF")["envelope"]
    for doc in (base, distinct(base), timoshenko(base), timoshenko(distinct(base)), arc, timoshenko(arc), allowed, timoshenko(allowed)):
        pressure_2.validate_pressure_evidence(doc)
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
    _refused(_reform_u4b(doc), "stress basis section")
    # One basis for both Lame families: the reduced hoop copied onto its nominal twin.
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


def _case_sections_and_pressures(doc):
    evidence = doc["contract_evidence"]
    sections = {(case["load_case_id"], s["pipe_id"]): s for case in evidence["exact_cases"] for s in case["pipe_sections"]}
    pressures = {(region["load_case_id"], pid): region["p_pa"] for region in evidence["pressure"] for pid in region["member_pipe_ids"]}
    return sections, pressures


def test_pressure_1_reader_refuses_nominal_lame_at_the_pressure_bore(base):
    """§8, §2.1: the nominal-basis Lame rows evaluated at the pressure bore. On distinct bases every nominal hoop row is
    published as 2p*Ai/A_n at the outer surface and that plus p at the inner, with Ai = pi*r_i,r^2 the pressure bore's
    area and A_n the nominal area: neither the nominal annulus's own hoop nor the reduced twin's (Rust's twin is
    section_bases.rs's test of the same name)."""
    doc = distinct(base)
    sections, pressures = _case_sections_and_pressures(doc)
    rows = _rows(deepcopy(doc))
    published = 0
    for row in doc["results"]:
        if row["kind"] != "pipe_lame_hoop_stress_v2" or sb.id_basis(row["id"]) != sb.NOMINAL:
            continue
        key = (row["basis_ref"]["ref_id"], row["entity_ref"])
        s, p = sections[key], pressures[key]
        outer = 2.0 * (p * s["pressure_bore"]["Ai_m2"]) / s["nominal_section"]["As_m2"]
        value = outer if row["metadata"]["component"] == "lame_outer_hoop_stress" else outer + p
        assert value != row["value"] and value != rows[sb.twin_id(row["id"])]["value"], row["id"]
        row["value"] = value
        published += 1
    assert published
    _refused(doc, "stress basis section")


def _reduced_zero_rows(doc, kind):
    return [i for i, r in enumerate(doc["results"]) if r["kind"] == kind and sb.id_basis(r["id"]) == sb.REDUCED
            and type(r["value"]) is float and struct.pack(">d", r["value"]) == bytes(8)]


@pytest.mark.parametrize("value", [-0.0, 0])
def test_pressure_1_reader_compares_basis_twins_by_bits(base, value):
    """A2: a twin pair's numbers compare by their binary64 bits, as the Rust reader does. An exact positive zero's twin
    written -0.0 is refused; written ``0`` it is the same binary64 number and is accepted (Rust's twin is section_bases.rs's
    test of the same name)."""
    def read(doc):
        if type(value) is int:
            pressure_2.validate_pressure_evidence(doc)
        else:
            _refused(doc, "^SOURCE_PHYSICS_EVIDENCE_INVALID: stress basis section$")
    # Coincident bases: stress row twins (twin_values).
    for kind in ("element_local_bending_normal_stress_y", "element_local_torsional_shear_stress"):
        doc = deepcopy(base)
        doc["results"][_reduced_zero_rows(doc, kind)[0]]["value"] = value
        read(doc)
    # Distinct bases: the Lame radial twins at the outer surface.
    doc = distinct(base)
    doc["results"][_reduced_zero_rows(doc, "pipe_lame_radial_stress_v2")[0]]["value"] = value
    read(doc)
    # Coincident bases: the maximum records (extrema_twins).
    for key in ("station_fraction", "local_fraction"):
        doc = deepcopy(base)
        extremum = next(e for e in doc["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"] if e["section_basis"] == sb.REDUCED)
        assert struct.pack(">d", extremum[key]) == bytes(8), key
        extremum[key] = value
        read(doc)


def test_basis_headlines_bind_their_maximum_rows_by_bits():
    """A2: a headline binds its maximum row's value bit for bit. On a zero maximum, the headline written -0.0 is refused
    and written ``0`` accepted (Rust's twin: basis_headlines_bind_their_maximum_rows_by_bits)."""
    rows = {f"result:elastic-maximum:case:a:pipe:a:{b}": {"id": f"result:elastic-maximum:case:a:pipe:a:{b}", "kind": "pipe_elastic_normal_stress_maximum_v2",
            "value": 0.0, "unit": "Pa", "entity_ref": "pipe:a", "metadata": {"basis": f"recovered_from_open_mechanics_stress_components{sb.SEPARATOR}{b}"}}
            for b in sb.BASES}

    def summary(reduced):
        out = {"max_open_formula_stress": None}
        for b, rid in zip(sb.BASES, rows):
            out[f"max_open_formula_stress_{b}"] = {"value": 0.0, "unit": "Pa", "location_ref": "pipe:a", "result_ref": rid}
        out["max_open_formula_stress_reduced_section"]["value"] = reduced
        return out

    complete = {b: True for b in sb.BASES}
    for value in (0.0, 0):
        sb.headlines(summary(value), rows, complete)
    with pytest.raises(sb.BasisError, match="^stress maximum headline binding$"):
        sb.headlines(summary(-0.0), rows, complete)


def _set_model_section(model, pipe, key, value):
    next(p for p in model["pipe_segments"] if p["id"] == pipe)["section"][key] = value


def test_pressure_1_model_checks_bind_each_section_input_to_the_model(base):
    """§1, §5 (FINAL-B): G2 binds each case section's captured D, t, c_m (0.0 when the model has none) and c_a to the
    model's section of that pipe, bit for bit after the unit catalog: each one ulp off is refused by name, as is a
    signed zero and a pipe's section swapped with another's (Rust's twin is section_bases.rs's test of the same name)."""
    code = "^SOURCE_PHYSICS_MODEL_SECTION_GEOMETRY$"
    request = json.loads((ROOT / "core/product_physics/tests/fixtures/exact_pressure_connected_request.json").read_text())
    connected = (deepcopy(base), request["model"])
    entry = _arc_corpus_entry("U2-L-ANCH-PTW-K2 0.3.0")
    milled = (entry["envelope"], entry["document"]["model"])
    # An allowance on the arc document (T4-U4b: the producer's, 0.0005 m on every pipe): its distinct bases and the model.
    entry = _arc_corpus_entry("U2-L-ANCH-PTW-K2 directional SIF")
    allowed = (entry["envelope"], entry["document"]["model"])
    assert {s["corrosion_allowance_m"] for s in allowed[0]["contract_evidence"]["exact_cases"][0]["pipe_sections"]} == {0.0005}
    leaves = (("outside_diameter", "outside_diameter_m"), ("wall_thickness", "wall_thickness_m"),
              ("mill_tolerance", "mill_tolerance_m"), ("corrosion_allowance", "corrosion_allowance_m"))
    for label, (doc, model) in (("connected", connected), ("milled arc", milled), ("allowance on the arc", allowed)):
        assert rv.validate("pressure", doc, model) == {"ok": True}, label
        section = doc["contract_evidence"]["exact_cases"][0]["pipe_sections"][0]
        for key, leaf in leaves:
            changed = deepcopy(model)
            _set_model_section(changed, section["pipe_id"], key, {"value": math.nextafter(section[leaf], math.inf), "unit": "m"})
            assert rv.validate("pressure", doc, changed) == {"ok": False, "error": {"code": "SOURCE_PHYSICS_MODEL_SECTION_GEOMETRY", "detail": None}}, (label, key)
            with pytest.raises(rv.Refused, match=code):
                rv.physics_model_checks(doc, changed)
            # The same quantity in metres, bit for bit, is bound.
            _set_model_section(changed, section["pipe_id"], key, {"value": section[leaf], "unit": "m"})
            rv.physics_model_checks(doc, changed)
    # The connected document has no mill tolerance and a zero allowance: an absent or null mill tolerance is 0.0; a
    # signed zero is not.
    doc, model = connected
    pipe = doc["contract_evidence"]["exact_cases"][0]["pipe_sections"][0]["pipe_id"]
    assert "mill_tolerance" not in model["pipe_segments"][0]["section"]
    for key, value, bound in (("mill_tolerance", None, True), ("mill_tolerance", {"value": 0, "unit": "mm"}, True),
                              ("mill_tolerance", {"value": -0.0, "unit": "m"}, False), ("corrosion_allowance", {"value": -0.0, "unit": "m"}, False),
                              ("corrosion_allowance", None, False), ("wall_thickness", {"value": 10.0, "unit": "mm"}, True),
                              ("wall_thickness", {"value": 10.0, "unit": "N"}, False)):
        changed = deepcopy(model)
        _set_model_section(changed, pipe, key, value)
        if bound:
            rv.physics_model_checks(doc, changed)
        else:
            with pytest.raises(rv.Refused, match=code):
                rv.physics_model_checks(doc, changed)
    # A pipe whose section is another pipe's: the arc document's members with their own walls, then two of them
    # swapped in the model.
    doc, model = deepcopy(milled[0]), deepcopy(milled[1])
    pipes = [s["pipe_id"] for s in doc["contract_evidence"]["exact_cases"][0]["pipe_sections"]]
    assert len(pipes) >= 2
    for i, pipe in enumerate(pipes):
        _set_model_section(model, pipe, "wall_thickness", {"value": 0.004 + 0.001 * i, "unit": "m"})
        for case in doc["contract_evidence"]["exact_cases"]:
            for s in case["pipe_sections"]:
                if s["pipe_id"] == pipe:
                    s["wall_thickness_m"] = 0.004 + 0.001 * i
    rv.physics_model_checks(doc, model)
    reversed_model = deepcopy(model)
    reversed_model["pipe_segments"].reverse()
    rv.physics_model_checks(doc, reversed_model)  # bound by pipe id, not by order
    swapped = deepcopy(model)
    sections = {p["id"]: p["section"] for p in model["pipe_segments"]}
    for segment in swapped["pipe_segments"]:
        if segment["id"] in pipes[:2]:
            segment["section"] = deepcopy(sections[pipes[1] if segment["id"] == pipes[0] else pipes[0]])
    with pytest.raises(rv.Refused, match=code):
        rv.physics_model_checks(doc, swapped)


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
    pressure_2.validate_pressure_evidence(good)
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


def _contract_model(doc, shear_model):
    """A model carrying only what G2's checks read here: the solved cases and the exact contract naming the shear model."""
    return {"load_cases": [{"id": case["load_case_id"]} for case in doc["contract_evidence"]["exact_cases"]],
            "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": shear_model}}


def _with_case_beam(doc, pipe, beam):
    doc = deepcopy(doc)
    for case in doc["contract_evidence"]["exact_cases"]:
        for s in case["pipe_sections"]:
            if s["pipe_id"] == pipe:
                s["beam_theory"] = deepcopy(beam)
    return doc


def _arc_corpus_entry(label):
    return next(c for c in json.loads(ARC.read_text())["cases"] if c["label"].startswith(label))


def test_pressure_1_reader_refuses_beam_theory_contradicting_the_contract(base):
    """§5: a straight member's beam theory is the captured contract's shear model (G2, against the model); kappa and A_s
    stay the evidence reader's. Rust's twin is section_bases.rs's test of the same name."""
    eb, tim = sb.EULER_BERNOULLI, sb.TIMOSHENKO
    euler = distinct(base)
    timo = timoshenko(euler)
    rv.physics_model_checks(euler, _contract_model(euler, eb))
    rv.physics_model_checks(timo, _contract_model(timo, tim))
    # A contract naming no admitted model selects nothing.
    for doc, model in ((euler, tim), (timo, eb), (euler, "cowper")):
        with pytest.raises(rv.Refused, match="^SOURCE_PHYSICS_SHEAR_MODEL_CONTRACT$"):
            rv.physics_model_checks(doc, _contract_model(doc, model))
    # A region geometry record contradicting the contract, its case section agreeing.
    region = deepcopy(timo)
    region["contract_evidence"]["pressure"][0]["geometry"][0]["beam_theory"] = {"model": eb}
    with pytest.raises(rv.Refused, match="^SOURCE_PHYSICS_SHEAR_MODEL_CONTRACT$"):
        rv.physics_model_checks(region, _contract_model(region, tim))
    # The pressure route: the evidence reader, then G2 against the model.
    refusal = {"ok": False, "error": {"code": "SOURCE_PHYSICS_SHEAR_MODEL_CONTRACT", "detail": None}}
    for doc, model, expected in ((timo, eb, refusal), (timo, tim, {"ok": True}), (euler, tim, refusal), (euler, eb, {"ok": True})):
        assert rv.validate("pressure", doc, _contract_model(doc, model)) == expected


def test_pressure_1_model_checks_take_realized_arcs_from_the_model():
    """§3.6: a realized arc is Euler-Bernoulli under either contract. The model names the arcs, including one no pressure
    region marks (the L line with a connector has explicitly empty regions); §2.5: the marked case sections are exactly
    those arcs."""
    eb, tim = sb.EULER_BERNOULLI, sb.TIMOSHENKO
    entry = _arc_corpus_entry("L line with connector")
    model = deepcopy(entry["document"]["model"])
    assert sb.realized_arcs(model) == {"pipe:BEND"}
    # The producer marks the arc outside every region on its case sections.
    doc = deepcopy(entry["envelope"])
    assert all("member_kind" not in g for region in doc["contract_evidence"]["pressure"] for g in region["geometry"])
    model["pressure_contract"]["shear_model"] = tim
    # The arc's marked section stays Euler-Bernoulli; the straights take Timoshenko.
    stated = timoshenko(doc)
    rv.physics_model_checks(stated, model)
    # The arc claiming Timoshenko (its kappa and A_s consistent).
    unmarked = timoshenko(_unmarked(doc, "pipe:BEND"))
    claim = next(s["beam_theory"] for s in unmarked["contract_evidence"]["exact_cases"][0]["pipe_sections"] if s["pipe_id"] == "pipe:BEND")
    assert claim["model"] == tim
    with pytest.raises(rv.Refused, match="^SOURCE_PHYSICS_ARC_BEAM_THEORY$"):
        rv.physics_model_checks(_with_case_beam(stated, "pipe:BEND", claim), model)
    # The realized arc's section without its marker.
    unmarked_stated = _with_case_beam(unmarked, "pipe:BEND", {"model": eb})
    with pytest.raises(rv.Refused, match="^SOURCE_PHYSICS_ARC_MARKER$"):
        rv.physics_model_checks(unmarked_stated, model)
    # A bend the model does not realize as an arc is a straight member: its marker is refused, and unmarked it states
    # the contract's model.
    geometric = deepcopy(model)
    for component in geometric["components"]:
        if component["kind"] == "bend":
            component["mechanics_interface"]["solver_consumption"] = "mechanics_geometry_only"
    assert sb.realized_arcs(geometric) == set()
    with pytest.raises(rv.Refused, match="^SOURCE_PHYSICS_ARC_MARKER$"):
        rv.physics_model_checks(stated, geometric)
    with pytest.raises(rv.Refused, match="^SOURCE_PHYSICS_SHEAR_MODEL_CONTRACT$"):
        rv.physics_model_checks(unmarked_stated, geometric)
    rv.physics_model_checks(unmarked, geometric)
    # Under Euler-Bernoulli every member states Euler-Bernoulli.
    model["pressure_contract"]["shear_model"] = eb
    rv.physics_model_checks(doc, model)
    with pytest.raises(rv.Refused, match="^SOURCE_PHYSICS_SHEAR_MODEL_CONTRACT$"):
        rv.physics_model_checks(stated, model)
    # An arc a region marks: its marked section is the model's realized arc.
    entry = _arc_corpus_entry("U2-L-ANCH-PTW-K2 0.3.0")
    timo = timoshenko(entry["envelope"])
    model = deepcopy(entry["document"]["model"])
    model["pressure_contract"]["shear_model"] = tim
    rv.physics_model_checks(timo, model)
    bend = next(g["pipe_id"] for g in timo["contract_evidence"]["pressure"][0]["geometry"] if "member_kind" in g)
    straight = next(s["beam_theory"] for s in timo["contract_evidence"]["exact_cases"][0]["pipe_sections"] if s["pipe_id"] != bend)
    with pytest.raises(rv.Refused, match="^SOURCE_PHYSICS_ARC_BEAM_THEORY$"):
        rv.physics_model_checks(_with_case_beam(timo, bend, straight), model)
    # A model without components realizes no arc: the region's mark contradicts it.
    del model["components"]
    with pytest.raises(rv.Refused, match="^SOURCE_PHYSICS_ARC_MARKER$"):
        rv.physics_model_checks(timo, model)


def test_pressure_1_reader_binds_the_case_section_marker_to_the_region_arc(arc):
    """§2.5: the region marks exactly the members the case sections mark (Rust's twin is section_bases.rs's test of the
    same name)."""
    pressure_2.validate_pressure_evidence(arc)
    region = next(r for r in arc["contract_evidence"]["pressure"] if any("member_kind" in g for g in r["geometry"]))
    bend = next(g["pipe_id"] for g in region["geometry"] if "member_kind" in g)
    straight = next(g["pipe_id"] for g in region["geometry"] if "member_kind" not in g)

    def marking(pipe, marked):
        doc = deepcopy(arc)
        for case in doc["contract_evidence"]["exact_cases"]:
            for s in case["pipe_sections"]:
                if s["pipe_id"] == pipe:
                    if marked:
                        s[sb.MEMBER_KIND] = sb.REALIZED_ARC
                    else:
                        del s[sb.MEMBER_KIND]
        return doc

    # The region's arc without the marker on its case section; a region straight marked on its case section.
    _refused(marking(bend, False), "arc marker")
    _refused(marking(straight, True), "arc marker")


def test_pressure_source_1_composite_refuses_arcs_by_name_and_reads_one_straight_shear_model(base, arc):
    """§3.6: the pressure-source-1 composite reading (``physics_evidence.py``) checks each section's beam theory as
    ``pressure_2.py`` and Rust's shared reader do, with one straight-member shear model per document. It never carries a
    realized arc (D7), so an arc region or a marked case section is refused by name (Rust
    ``SOURCE_PHYSICS_ARC_UNSUPPORTED``)."""
    def composite(doc):
        doc = deepcopy(doc)
        for case in doc["contract_evidence"]["exact_cases"]:
            case["recovery_method"] = source_blocks.MODES["dense_scrutiny"]
        return doc

    context = ps.SourceContract.PRESSURE_V3
    mixed = composite(timoshenko(base))
    mixed["contract_evidence"]["exact_cases"][1]["pipe_sections"][0]["beam_theory"] = {"model": sb.EULER_BERNOULLI}
    with pytest.raises(ValueError, match="one straight shear model per document"):
        physical._validate_physics_evidence(mixed, context=context)
    for doc in (arc, timoshenko(arc)):
        with pytest.raises(ValueError, match="realized arc unsupported"):
            physical._validate_physics_evidence(composite(doc), context=context)


def test_pressure_source_1_and_physics_retained_1_bind_the_section_beam_theory_to_the_invocation():
    """§5 and O4 (§3.4): each captured section states the beam theory its contract selects for a straight member
    (``physics_source._actual_materials``, also G8 step 5, whose exact namespace still admits euler_bernoulli only);
    a contract naming no admitted model fails closed. Rust's twin is physics_source.rs's
    ``actual_materials_bind_each_section_beam_theory_to_the_contract_selection``."""
    request = json.loads((ROOT / "fixtures/product_preview/pressure_source/physics_source/n05.request.json").read_text())
    invocation = {"request": request, "solver_mode": "sparse_interactive"}

    def check(invocation, beam):
        model = invocation["request"]["model"]
        pipe, authored, case = model["pipe_segments"][0], model["materials"][0], model["load_cases"][0]
        canonical = ps._canonical_inputs(invocation)
        e, nu, _ = ps._selected_material(authored, case, canonical)
        section = pipe["section"]
        exact = {"pipe_sections": [{"pipe_id": pipe["id"], "outside_diameter_m": canonical(section["outside_diameter"], "length"),
                                    "wall_thickness_m": canonical(section["wall_thickness"], "length"), "mill_tolerance_m": 0.0,
                                    "corrosion_allowance_m": canonical(section["corrosion_allowance"], "length"), "beam_theory": beam}],
                 "pipe_materials": [{"pipe_id": pipe["id"], "material_id": pipe["material"], "constitutive_basis": authored["constitutive_basis"],
                                     "E_pa": e, "nu": nu, "thermal_consumed": False, "alpha_per_kelvin": None}]}
        ps._actual_materials(invocation, case, exact, canonical)

    euler = {"model": sb.EULER_BERNOULLI}
    timoshenko_claim = {"model": sb.TIMOSHENKO, "shear_factor": 0.5, "shear_area_m2": 1e-3}
    check(invocation, euler)
    for beam in (timoshenko_claim, {"model": "cowper"}, None):
        with pytest.raises(ValueError, match="^PHYSICS_SOURCE_ACTUAL_SHEAR_MODEL$"):
            check(invocation, beam)
    # A Timoshenko contract selects Timoshenko for its straight members.
    other = deepcopy(invocation)
    other["request"]["model"]["pressure_contract"]["shear_model"] = sb.TIMOSHENKO
    check(other, timoshenko_claim)
    for beam in (euler, {"model": "cowper"}, None):
        with pytest.raises(ValueError, match="^PHYSICS_SOURCE_ACTUAL_SHEAR_MODEL$"):
            check(other, beam)
    # A contract naming no admitted model fails closed, whatever the section states.
    for shear in ("cowper", None, 5):
        unknown = deepcopy(invocation)
        unknown["request"]["model"]["pressure_contract"]["shear_model"] = shear
        for beam in (euler, timoshenko_claim):
            with pytest.raises(ValueError, match="^PHYSICS_SOURCE_ACTUAL_SHEAR_MODEL$"):
                check(unknown, beam)


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


def test_u6_joint_face_area_is_the_d5_pressure_bore():
    """§1, §2.3, §2.5: a joint face's bore area is the region geometry's ``pressure_bore.Ai_m2``, on distinct bases (a
    nonzero corrosion allowance puts the pressure bore off the nominal bore). Rust's twin is connector_evidence.rs
    ``u6_joint_face_area_is_the_d5_pressure_bore``."""
    p, ae, od, wall, allowance = 2.0e6, 0.0125, 0.1143, 0.006, 0.0015
    reduced = (wall - 0.0) - allowance

    def annulus(w):
        ro, ri = od * 0.5, od * 0.5 - w
        area = (math.pi * w) * (od - w)
        second = (area * ((ro * ro) + (ri * ri))) * 0.25
        return {"wall_m": w, "ri_m": ri, "As_m2": area, "I_m4": second, "J_m4": second * 2, "Z_m3": second / ro}

    def bore(w):
        ri = od * 0.5 - w
        return (math.pi * ri) * ri

    def geometry(pipe):
        return {"pipe_id": pipe, "geometry_basis": sb.GEOMETRY_BASIS, "outside_diameter_m": od, "ro_m": od * 0.5, "wall_thickness_m": wall,
                "mill_tolerance_m": 0.0, "corrosion_allowance_m": allowance, "nominal_section": annulus(wall), "reduced_section": annulus(reduced),
                "pressure_bore": {"ri_m": od * 0.5 - reduced, "Ai_m2": bore(reduced)}, "beam_theory": {"model": sb.EULER_BERNOULLI}, "traversal_forward": True}

    def region(entries, face):
        pe = p * ae
        difference = pe - p * face
        return [{"load_case_id": "case:A", "p_pa": p, "pressure_basis": "internal_differential_zero_external_v1", "member_pipe_ids": ["pipe:A", "pipe:B"],
                 "member_connector_ids": ["connector:J"], "terminals": [{"node_ref": "node:0"}, {"node_ref": "node:3"}], "geometry": entries,
                 "joint_edges": [{"connector_id": "connector:J", "node_i_ref": "node:1", "node_j_ref": "node:2", "face_i_pipe_id": "pipe:A",
                                  "face_j_pipe_id": "pipe:B", "p_pa": p, "effective_area_m2": ae, "face_bore_area_m2": [face, face],
                                  "joint_pressure_force_n": pe, "area_difference_force_n": [difference, difference],
                                  "attachment_pair_global_n": [[-difference, 0.0, 0.0], [difference, 0.0, 0.0]],
                                  "representation": "B: p(Ae - Ai) at the attachments; joint_effective_area + joint_face_cap"}]}]

    record = {"component_id": "connector:J", "pressure_model": "constant_effective_area_reference_v1", "effective_area_m2": ae, "node_i": "node:1",
              "node_j": "node:2", "pressure_basis": "internal_differential_zero_external_v1", "end_i_offset_local_m": [0.0, 0.0, 0.0],
              "end_j_offset_local_m": [0.0, 0.0, 0.0], "connector_axes_global": [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]}
    check = lambda regions: pressure_2._joint_pressure_bindings([record], regions, {}, set(), False)
    assert bore(reduced) != bore(wall)
    # The D-5 region geometry is read: the face is the pressure bore.
    check(region([geometry("pipe:A"), geometry("pipe:B")], bore(reduced)))
    # A face on the nominal bore contradicts the pressure bore.
    with pytest.raises(ValueError, match="joint face area"):
        check(region([geometry("pipe:A"), geometry("pipe:B")], bore(wall)))
    # A stray single-section Ai_m2 beside the D-5 entry, set to the nominal bore, does not bind the face.
    stray = [{**geometry(pipe), "Ai_m2": bore(wall)} for pipe in ("pipe:A", "pipe:B")]
    with pytest.raises(ValueError, match="joint face area"):
        check(region(stray, bore(wall)))
    # An entry without the pressure bore states no face area.
    bare = [{k: v for k, v in geometry(pipe).items() if k != "pressure_bore"} for pipe in ("pipe:A", "pipe:B")]
    with pytest.raises(ValueError, match="joint face area"):
        check(region(bare, bore(reduced)))


def test_pressure_source_1_admits_mill_tolerance_and_physics_retained_1_refuses_it():
    """O4 (§3.4): PHYSICS_SOURCE_ACTUAL_MILL_TOLERANCE is retired. pressure-source-1's invocation check admits a mill
    tolerance within the section sum (it binds by value through ACTUAL_SOURCE_GEOMETRY and its range through the D-5
    rederivation); one that is not a finite length >= 0, or that reaches the wall, leaves the section sum
    unestablished and is refused with the allowance. Rust's twin is section_bases.rs's
    ``pressure_source_1_admits_mill_tolerance_within_the_section_sum``. O4 (§4.7): physics-retained-1's G8 admits the
    same mill tolerances within the section sum; its named decline is retired."""
    request = json.loads((ROOT / "fixtures/product_preview/pressure_source/physics_source/n05.request.json").read_text())
    invocation = {"request": request, "solver_mode": "sparse_interactive"}
    contract = ps.SourceContract.PRESSURE_V3
    contract.check_invocation(invocation)
    changed = deepcopy(invocation)
    changed["request"]["model"]["pipe_segments"][0]["section"]["mill_tolerance"] = {"value": 0.5, "unit": "mm"}
    contract.check_invocation(changed)
    for value in ({"value": 0, "unit": "mm"}, None, {"value": 9.999, "unit": "mm"}):
        admitted = deepcopy(invocation)
        admitted["request"]["model"]["pipe_segments"][0]["section"]["mill_tolerance"] = value
        contract.check_invocation(admitted)
    for bad in ({"value": 1, "unit": "N"}, {"value": "0", "unit": "mm"}, {"value": 10, "unit": "mm"}, {"value": -0.5, "unit": "mm"}):
        broken = deepcopy(invocation)
        broken["request"]["model"]["pipe_segments"][0]["section"]["mill_tolerance"] = bad
        with pytest.raises(ValueError, match="^PHYSICS_SOURCE_ACTUAL_CORROSION_ALLOWANCE$"):
            contract.check_invocation(broken)
    # O4 (§4.7): physics-retained-1's G8 namespace admits the nonzero mill tolerance within the section sum, and
    # refuses one that is not a finite length >= 0 or that reaches the wall; the named decline is retired.
    assert not hasattr(rp, "MILL_TOLERANCE_DECLINE") and not hasattr(rp, "_exact_mill_tolerances")
    assert rp._exact_namespace(changed["request"]["model"]) and rp._exact_namespace(invocation["request"]["model"])
    for bad in ({"value": 1, "unit": "N"}, {"value": 10, "unit": "mm"}, {"value": -0.5, "unit": "mm"}):
        broken = deepcopy(invocation)
        broken["request"]["model"]["pipe_segments"][0]["section"]["mill_tolerance"] = bad
        assert not rp._exact_namespace(broken["request"]["model"]), bad


# ----------------------------------------------------------------------------------------------------------------------
# §2.5 arc markers, on documents built in memory in the current D-5 shape: two exact cases, a straight member and a
# realized arc with explicitly empty pressure regions, distinct section bases (a nonzero corrosion allowance), a bending
# row pair per member and per case, and the straight member's maximum on both bases (the arc's is withheld).
# ----------------------------------------------------------------------------------------------------------------------

CASES = ("case:A", "case:B")
STRAIGHT, BEND = "pipe:A", "pipe:BEND"
OD, WALL, ALLOWANCE, E, NU, MAXIMUM = 0.1683, 0.0071, 0.0015, 2.0e11, 0.3, 30.0e6
MAX_SIGN = ("nonnegative circumferential maximum |Nw/As|+hypot(My,Mz)/Z; bounded over all straight statics intervals; torsional "
            "shear remains separate; no code stress or equivalent stress claim")
BENDING_SIGN = ("positive value follows the j-side section action in the element-local frame (local x toward end j); resultants "
                "come from section equilibrium over assembled end actions")


def _annulus(wall):
    ro, ri = OD * 0.5, OD * 0.5 - wall
    area = (math.pi * wall) * (OD - wall)
    second = (area * ((ro * ro) + (ri * ri))) * 0.25
    return {"wall_m": wall, "ri_m": ri, "As_m2": area, "I_m4": second, "J_m4": second * 2, "Z_m3": second / ro}


def _reduced_wall():
    return sb.reduced_wall(WALL, 0.0, ALLOWANCE)


def _timoshenko_beam():
    kappa = sb.timoshenko_shear_factor(OD * 0.5 - WALL, OD * 0.5, NU)
    return {"model": sb.TIMOSHENKO, "shear_factor": kappa, "shear_area_m2": kappa * _annulus(WALL)["As_m2"]}


def _entry(pipe, beam, marked):
    ri = OD * 0.5 - _reduced_wall()
    entry = {"pipe_id": pipe, "geometry_basis": sb.GEOMETRY_BASIS, "outside_diameter_m": OD, "ro_m": OD * 0.5, "wall_thickness_m": WALL,
             "mill_tolerance_m": 0.0, "corrosion_allowance_m": ALLOWANCE, "nominal_section": _annulus(WALL), "reduced_section": _annulus(_reduced_wall()),
             "pressure_bore": {"ri_m": ri, "Ai_m2": (math.pi * ri) * ri}, "beam_theory": beam}
    return {**entry, sb.MEMBER_KIND: sb.REALIZED_ARC} if marked else entry


def _marker_document(shear_model, arc=True):
    """A pressure-2 document under ``shear_model`` with explicitly empty pressure regions (Rust ``arc_marker_tests::document``)."""
    straight_beam = _timoshenko_beam() if shear_model == sb.TIMOSHENKO else {"model": sb.EULER_BERNOULLI}
    scale = _annulus(WALL)["Z_m3"] / _annulus(_reduced_wall())["Z_m3"]
    members = [(STRAIGHT, 12.5), (BEND, 7.25)] if arc else [(STRAIGHT, 12.5)]
    results, cases, headlines = [], [], {}
    zeros = [0.0] * 12
    for case in CASES:
        for pipe, value in members:
            for basis, v in ((sb.NOMINAL, value), (sb.REDUCED, value * scale)):
                results.append({"id": f"result:loadcase:{case}:stress:{pipe}:end-i:bending-normal-y:{sb.hyphenated(basis)}",
                                "kind": "element_local_bending_normal_stress_y", "value": v, "unit": "MPa", "entity_ref": pipe,
                                "basis_ref": {"ref_type": "load_case", "ref_id": case},
                                "metadata": {"component": "bending_normal_stress_y", "coordinate_system": "element_local", "location": "end_i",
                                             "basis": f"recovered_from_local_element_stiffness; section_basis={basis}", "sign_convention": BENDING_SIGN}})
        extrema = []
        for basis, v in ((sb.NOMINAL, MAXIMUM), (sb.REDUCED, MAXIMUM * scale)):
            rid = f"result:elastic-maximum:{len(case)}:{case}:{len(STRAIGHT)}:{STRAIGHT}:{basis}"
            results.append({"id": rid, "kind": "pipe_elastic_normal_stress_maximum_v2", "value": v, "unit": "Pa", "entity_ref": STRAIGHT,
                            "basis_ref": {"ref_type": "load_case", "ref_id": case},
                            "metadata": {"component": "maximum_absolute_normal_stress", "coordinate_system": "pipe_section", "location": "governing_station",
                                         "basis": f"recovered_from_open_mechanics_stress_components; section_basis={basis}", "sign_convention": MAX_SIGN}})
            extrema.append({"pipe_id": STRAIGHT, "section_basis": basis, "result_id": rid, "station_fraction": 0.0, "span_index": 0, "local_fraction": 0.0,
                            "value_lower_pa": v, "value_upper_pa": v, "global_upper_bound_pa": v, "certified_gap_pa": 0.0, "subdivisions": 0,
                            "approximation": "piecewise_quadratic_straight_section_statics", "coefficient_basis": "j_side_section_equilibrium_binary64_enclosures",
                            "enclosure_scope": "supplied_binary64_polynomial_coefficient_enclosures; solution error is separate"})
            headlines.setdefault(f"max_open_formula_stress_{basis}", None if arc else {"value": v, "unit": "Pa", "location_ref": STRAIGHT, "result_ref": rid})
        cases.append({"load_case_id": case, "profile_mode": "exact_pressure_v3", "material_basis": "base_material_common_E_nu",
                      "pipe_materials": [{"pipe_id": pipe, "material_id": "material:steel", "E_pa": E, "nu": NU, "G_pa": E / (2.0 * (1.0 + NU)),
                                          "constitutive_basis": "homogeneous_isotropic_E_nu_v1", "thermal_consumed": False, "alpha_per_kelvin": None,
                                          "provenance": "authored"} for pipe, _ in members],
                      "pipe_sections": [_entry(pipe, {"model": sb.EULER_BERNOULLI}, True) if pipe == BEND else _entry(pipe, deepcopy(straight_beam), False) for pipe, _ in members],
                      "pipe_stress_extrema": extrema,
                      "stress_maximum_coverage": [{"section_basis": b, "complete": not arc, "unavailable_pipe_ids": [BEND] if arc else []} for b in sb.BASES],
                      "pressure_rhs_assembly": {"method": "source_factor_grouped_pressure_rhs_v1", "load_case_id": case, "node_order": ["node:1", "node:2"],
                                                "dof_order": ["Fx", "Fy", "Fz", "Mx", "My", "Mz"], "dof_units": ["N", "N", "N", "N*m", "N*m", "N*m"],
                                                "assembled_pressure_rhs_global": list(zeros), "groups": [], "rounded_cap_rhs_global": list(zeros),
                                                "rounded_poisson_rhs_global": list(zeros), "rounded_cap_and_eigen_ledgers_are_observational": True,
                                                "cancellation_screen": 0.0, "screen_limit": 1e-9, "screen_roundoff_multiplier": 32,
                                                "screen_is_not_numerical_qualification": True}})
    return {"status": {"mechanics": "MECHANICS_SOLVED"},
            "numerical_quality": {"cases": [{"basis_ref": {"ref_type": "load_case", "ref_id": c}, "evidence_refs": []} for c in CASES]},
            "results": results, "diagnostics": [], "summary": {"max_open_formula_stress": None, **headlines},
            "contract_evidence": {"pressure": [], "connector": [], "exact_cases": cases}}


def _edit_sections(doc, pipe, edit, cases=CASES):
    doc = deepcopy(doc)
    for case in doc["contract_evidence"]["exact_cases"]:
        if case["load_case_id"] in cases:
            for s in case["pipe_sections"]:
                if s["pipe_id"] == pipe:
                    edit(s)
    return doc


def _unmark(s):
    del s[sb.MEMBER_KIND]


def _mark(s):
    s[sb.MEMBER_KIND] = sb.REALIZED_ARC


def _marker_model(shear_model, realized=True):
    return {"load_cases": [{"id": c} for c in CASES],
            "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": shear_model},
            "components": [{"id": "component:bend", "kind": "bend", "geometry": {"bend_pipe_ref": BEND},
                            "mechanics_interface": {"solver_consumption": "curved_bend_macro_element" if realized else "mechanics_geometry_only"}}]}


def test_an_arc_outside_every_pressure_region_is_known_by_its_marker():
    eb, tim = sb.EULER_BERNOULLI, sb.TIMOSHENKO
    # The arc outside every region in a Timoshenko document is admitted, as under Euler-Bernoulli.
    for shear_model in (eb, tim):
        pressure_2.validate_pressure_evidence(_marker_document(shear_model))
        pressure_2.validate_pressure_evidence(_marker_document(shear_model, arc=False))
    timo = _marker_document(tim)
    # Without its marker the arc's Euler-Bernoulli entry reads as a second straight model.
    _refused(_edit_sections(timo, BEND, _unmark), "one straight shear model per document")
    # The arc stating Timoshenko (its kappa and A_s consistent) is refused.
    _refused(_edit_sections(timo, BEND, lambda s: s.__setitem__("beam_theory", _timoshenko_beam())), "arc beam theory")
    # Any other member_kind, on the arc or on a straight member, is refused by name.
    for pipe in (BEND, STRAIGHT):
        _refused(_edit_sections(timo, pipe, lambda s: s.__setitem__(sb.MEMBER_KIND, "straight_pipe")), "member kind")
    # The marker on a straight member: an arc stating Timoshenko, or (stating Euler-Bernoulli) an arc whose rows keep
    # the straight sign conventions (T4-U4b: arcs publish their maximum, under the arc's own convention).
    _refused(_edit_sections(timo, STRAIGHT, _mark), "arc beam theory")
    _refused(_edit_sections(_marker_document(eb), STRAIGHT, _mark), "physical sign convention")
    # Realized arcs are a model fact: every case marks the same members.
    _refused(_edit_sections(_marker_document(eb), BEND, _unmark, cases=CASES[1:]), "arc marker")


def test_the_marked_entries_are_the_models_realized_arcs():
    eb, tim = sb.EULER_BERNOULLI, sb.TIMOSHENKO

    def refused(doc, model, code):
        with pytest.raises(rv.Refused, match=f"^{code}$"):
            rv.physics_model_checks(doc, model)

    for shear_model in (eb, tim):
        doc = _marker_document(shear_model)
        rv.physics_model_checks(doc, _marker_model(shear_model))
        assert rv.validate("pressure", doc, _marker_model(shear_model)) == {"ok": True}
        # A model that realizes no arc: the marked entry is a straight member's.
        refused(doc, _marker_model(shear_model, realized=False), "SOURCE_PHYSICS_ARC_MARKER")
    # The marker missing on the arc: an Euler-Bernoulli document the evidence reader admits (the arc reads as a straight
    # member), refused against the model.
    unmarked = _edit_sections(_marker_document(eb), BEND, _unmark)
    pressure_2.validate_pressure_evidence(unmarked)
    refused(unmarked, _marker_model(eb), "SOURCE_PHYSICS_ARC_MARKER")
    assert rv.validate("pressure", unmarked, _marker_model(eb))["error"]["code"] == "SOURCE_PHYSICS_ARC_MARKER"
    # The marker on a straight member; any other member_kind.
    refused(_edit_sections(_marker_document(eb), STRAIGHT, _mark), _marker_model(eb), "SOURCE_PHYSICS_ARC_MARKER")
    refused(_edit_sections(_marker_document(eb), BEND, lambda s: s.__setitem__(sb.MEMBER_KIND, "straight_pipe")), _marker_model(eb), "SOURCE_PHYSICS_MEMBER_KIND")
    # bind_shear_model keeps its checks: the arc stating Timoshenko, and a straight member stating a model other than
    # the contract's.
    refused(_edit_sections(_marker_document(tim), BEND, lambda s: s.__setitem__("beam_theory", _timoshenko_beam())), _marker_model(tim), "SOURCE_PHYSICS_ARC_BEAM_THEORY")
    refused(_marker_document(eb), _marker_model(tim), "SOURCE_PHYSICS_SHEAR_MODEL_CONTRACT")
    refused(_marker_document(tim), _marker_model(eb), "SOURCE_PHYSICS_SHEAR_MODEL_CONTRACT")


def test_the_pressure_source_1_composite_refuses_a_realized_arc_by_name():
    context = ps.SourceContract.PRESSURE_V3

    def composite(doc):
        doc = deepcopy(doc)
        for case in doc["contract_evidence"]["exact_cases"]:
            case["recovery_method"] = source_blocks.MODES["dense_scrutiny"]
        return physical._validate_physics_evidence(doc, context=context)

    straight = _marker_document(sb.EULER_BERNOULLI, arc=False)
    composite(straight)
    # A marked arc section, outside every pressure region.
    with pytest.raises(ValueError, match="realized arc unsupported"):
        composite(_marker_document(sb.EULER_BERNOULLI))
    # An arc region record, refused before anything else is read from it; the pressure-2 reading of the same evidence
    # does not refuse it by that name.
    region = deepcopy(straight)
    geometry = {**_entry(BEND, {"model": sb.EULER_BERNOULLI}, True), "traversal_forward": True, "arc_end_tangents_global": [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0]]}
    region["contract_evidence"]["pressure"] = [{"load_case_id": CASES[0], "region_id": "region:arc", "geometry": [geometry]}]
    with pytest.raises(ValueError, match="realized arc unsupported"):
        composite(region)
    with pytest.raises(ValueError) as ordinary:
        pressure_2.validate_pressure_evidence(region)
    assert "realized arc unsupported" not in str(ordinary.value)
