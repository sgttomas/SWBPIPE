"""DEL-04-09, "Selected mass assurance: next modal unit after CE"
(`execution/PKG-04/DEL-04-09/Work/modal-analysis.md`): the independent model-derived
mass leaf (`tests/verifiers/modal_model_mass.py`) and its integration into the Python
modal reader's mass and participation standings.

Controls:
- the frozen, independently derived `f1_modal/mass_discrete` references (PF and EX,
  straights, arcs, bodies, ineligible fallbacks, a replaced span) against the leaf's
  exact enclosures, without changing a frozen value;
- every intact committed fixture with its request;
- coherent mutations: the authored request stays fixed while the document's formed M
  and every dependent mass and participation value are resealed to agree with a
  WRONG M (another model's correct M, or an edit invisible to a·M·R_k), so the
  payload relations still pass and only the model-derived check can reject it;
- missing inputs, refused domains, rounding and resource controls.

All models are invented.
"""
from __future__ import annotations

import ast
import copy
import json
from decimal import Decimal
from fractions import Fraction
import math
from pathlib import Path

import pytest

from tests.verifiers import canonical_json
from tests.verifiers import modal_evidence as me
from tests.verifiers import modal_model_mass as mm
from tests.verifiers import modal_model_mass_interval as iv

ROOT = Path(__file__).resolve().parents[1]
RESULTS = ROOT / "fixtures/results"
REFERENCES = json.loads((ROOT / "validation/references/f1_modal/mass_discrete/mass_references.json").read_text(encoding="utf-8"))
SOLVED = ("arc", "cantilever_below_cutoff", "cantilever_cluster_extension", "cantilever_sensitive",
          "cantilever_subnormal_cutoff", "cantilever_whole_spectrum", "component_body", "ex_n05",
          "state_friction_sliding", "state_one_way_pressed")


def fixture(name):
    doc = json.loads((RESULTS / f"modal_{name}.json").read_text(encoding="utf-8"))
    request = json.loads((RESULTS / f"modal_{name}.request.json").read_text(encoding="utf-8"))
    return doc, request


def relation(verdict, name):
    return verdict["relations"][name]


def model_texts(standing):
    return standing["scope"] + standing["unresolved"]


# --- interval primitives ------------------------------------------------------------------

def test_the_interval_constants_and_functions_enclose_their_values():
    pi = iv.pi()
    assert pi.hi - pi.lo < Fraction(1, 10 ** 50)
    # fl(π) is within half an ulp of π, and π lies between fl(π) and its successor.
    assert pi.lo > Fraction(math.pi) and pi.hi < Fraction(math.nextafter(math.pi, 4.0))
    quarter = iv.atan(iv.Iv(1))
    assert (quarter * iv.Iv(4)).meets(pi)
    angle, cosine = iv.asin_and_cos(iv.Iv(Fraction(1, 2)))
    assert (angle * iv.Iv(6)).meets(pi) and cosine.square().meets(iv.Iv(Fraction(3, 4)))
    root = iv.sqrt(iv.Iv(2))
    assert root.lo ** 2 <= 2 <= root.hi ** 2
    with pytest.raises(iv.Undecidable):
        iv.Iv(1) / iv.Iv(-1, 1)


def test_the_binary64_model_holds_every_rounding_and_detects_overflow():
    third = iv.binary64(iv.Iv(Fraction(1, 3)))
    assert third.contains(Fraction(1.0 / 3.0))
    assert iv.binary64(iv.Iv(Fraction(0.1))).is_point()  # an exact binary64 value rounds exactly
    tiny = iv.binary64(iv.Iv(Fraction(1, 1 << 1074) * Fraction(3, 2)))
    assert tiny.contains(Fraction(5e-324)) and tiny.contains(Fraction(1e-323))
    with pytest.raises(iv.OutsideModel):
        iv.binary64(iv.Iv(iv.LARGEST * 2))
    assert iv.published_image(iv.Iv(Fraction(1, 1 << 1030))).contains(0)


# --- the frozen f1_modal references ----------------------------------------------------------

def _q(value, unit):
    return {"value": value, "unit": unit}


def neutral_request(case, basis):
    """A product request for a reference's neutral model (README "Neutral input form").
    A connector becomes its replaced span (an expansion joint whose objective connector
    names it, on the exact contract); a tie rod is connector hardware and carries no
    member; a constant-effort support is a support of that family."""
    m = case["model"]
    materials = m["materials"]
    model = {"schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
             "project": {"id": "p", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa",
                                              "temperature": "degC"}},
             "analysis_status": {}, "nodes": [{"id": n["id"], "position": dict(zip("xyz", n["xyz"]))} for n in m["nodes"]],
             "pipe_segments": [], "supports": [], "components": [], "load_cases": [], "combinations": [], "materials": []}
    if basis == "EX":
        model["pressure_contract"] = {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"}
    for member in m["members"]:
        if member["kind"] == "tie_rod":
            continue
        if member["kind"] == "connector":
            if basis != "EX":
                return None
            model["pipe_segments"].append({"id": member["id"], "from": member["i"], "to": member["j"], "material": "steel",
                                           "section": {"outside_diameter": _q(0.25, "m"), "wall_thickness": _q(0.01, "m"),
                                                       "corrosion_allowance": _q(0, "m"),
                                                       "material_density": _q(7850, "kg/m^3")},
                                           "y_reference": {"x": 0, "y": 0, "z": 1}})
            model["components"].append({"id": "joint:" + member["id"], "kind": "expansion_joint", "node": member["i"],
                                        "objective_connector": {"topology": {"type": "replaces_span", "span_ref": member["id"]}}})
            continue
        s = m["sections"][member["section"]]
        section = {"outside_diameter": _q(s["outside_diameter_m"], "m"), "wall_thickness": _q(s["wall_thickness_m"], "m"),
                   "mill_tolerance": _q(s["mill_tolerance_m"], "m"), "corrosion_allowance": _q(s["corrosion_allowance_m"], "m"),
                   "material_density": _q(materials[member["material"]]["density_kg_per_m3"], "kg/m^3")}
        if s["contents_density_kg_per_m3"] is not None:
            section["contents_density"] = _q(s["contents_density_kg_per_m3"], "kg/m^3")
        if s["insulation_thickness_m"] is not None:
            section["insulation_thickness"] = _q(s["insulation_thickness_m"], "m")
            section["insulation_density"] = _q(s["insulation_density_kg_per_m3"], "kg/m^3")
        model["pipe_segments"].append({"id": member["id"], "from": member["i"], "to": member["j"], "material": member["material"],
                                       "section": section, "y_reference": dict(zip("xyz", member["y_reference"]))})
        if member["kind"] == "arc":
            model["components"].append({"id": "bend:" + member["id"], "kind": "bend", "node": member["j"],
                                        "geometry": {"bend_pipe_ref": member["id"], "bend_radius": _q(member["radius_m"], "m")},
                                        "mechanics_interface": {"solver_consumption": "curved_bend_macro_element"},
                                        "modifiers": {"flexibility_factor_user_value": _q(member["flexibility_factor"], "none")}})
    for c in m["components"]:
        geometry = {"rigid_pipe_ref": c["maps_member"], "weight": _q(c["weight_N"], "N")}
        if c["center_of_gravity_m"] is not None:
            geometry["center_of_gravity"] = {**dict(zip("xyz", c["center_of_gravity_m"])), "unit": "m"}
        model["components"].append({"id": c["id"], "kind": c["kind"], "node": c["node"], "geometry": geometry,
                                    "mechanics_interface": {"solver_consumption": c["solver_consumption"]}})
    for i, s in enumerate(m["supports"]):
        model["supports"].append({"id": f"s{i}", "node": s["node"], "family": "anchor" if len(s["restrained"]) == 6 else "guide",
                                  "restraints": [r.upper() for r in s["restrained"]]})
    for i, s in enumerate(m.get("constant_effort_supports") or []):
        model["supports"].append({"id": f"ce{i}", "node": s["node"], "family": "constant_effort_support", "restraints": []})
    request = {"model": model, "materials": [], "mode_count": 1}
    if m["weight_to_mass_acceleration_m_per_s2"] is not None:
        request["weight_to_mass_acceleration"] = _q(m["weight_to_mass_acceleration_m_per_s2"], "m/s^2")
    return request


def _decimal(text):
    """A 40-significant-digit reference decimal as the interval it rounds."""
    d = Decimal(text)
    if d == 0:
        return iv.Iv(0)
    half = Fraction(5) * Fraction(10) ** (d.adjusted() - 40)
    return iv.Iv(Fraction(d) - half, Fraction(d) + half)


REFERENCE_CASES = [(case["id"], basis) for case in REFERENCES["cases"] for basis in case["bases"]]


@pytest.mark.parametrize("case_id,basis", REFERENCE_CASES)
def test_the_model_mass_reproduces_the_frozen_f1_modal_references(case_id, basis):
    case = next(c for c in REFERENCES["cases"] if c["id"] == case_id)
    request = neutral_request(case, basis)
    if request is None:
        pytest.skip("a connector exists only on the exact contract")
    derived = mm.derive(request)
    reference = case["bases"][basis]
    position = [6 * derived.node_ids.index(node) + mm.COMPONENTS.index(c) for node, c in reference["assembled_dof_order"]]
    expected = {}
    for row in reference["assembled_full_mass_upper_nonzero"]:
        a, b = position[row[0]], position[row[1]]
        expected[(min(a, b), max(a, b))] = _decimal(row[-1])
    # Every reference entry, and every structural zero the leaf forms, agrees to the
    # reference's 40 digits; the nearest binary64 lies in the published envelope.
    for key in set(expected) | set(derived.entries):
        want = expected.get(key, iv.Iv(0))
        assert derived.exact_entry(key).meets(want), (case_id, basis, key)
        if key in derived.entries:
            assert derived.rounded(key).contains(Fraction(float((want.lo + want.hi) / 2))), (case_id, basis, key)
    for category, value in reference["category_totals_kg"].items():
        assert derived.categories[category].meets(_decimal(value["decimal"])), (case_id, category)
    status = case["component_status"]
    assert len(derived.bodies) == sum(1 for s in status if s["weight_applied_as_body_mass"])
    assert len(derived.fallbacks) == sum(1 for s in status if s["expected_warning"])


# --- coherent documents ----------------------------------------------------------------------

UNIT_BY_CLASS = {"translation_translation": "unit:kilogram", "translation_rotation": "unit:kilogram_meter_per_radian",
                 "rotation_rotation": "unit:kilogram_square_meter_per_square_radian"}
NORMAL = Fraction(1, 1 << 1022)


def formed(request):
    """The correctly rounded M a request's model implies: a producer-like formed M."""
    derived = mm.derive(request)
    return {key: float((held[0].lo + held[0].hi) / 2) for key, held in derived.entries.items()}


def _published(x: Fraction):
    """The producer's `to_published`: (value, below_normal)."""
    if x != 0 and abs(x) < NORMAL:
        return 0.0, True
    return float(x), False


def _set(vo, value, flagged=False):
    vo["value"], vo["below_normal"], vo["undefined_reason"] = value, flagged, None


def _reseal(doc):
    for table in doc["tables"].values():
        if table.get("status") != "published":
            continue
        checksums = []
        for row in table["rows"]:
            row["checksum"] = canonical_json.canonical_sha256({k: v for k, v in row.items() if k != "checksum"})
            checksums.append(row["checksum"])
        table["row_count"] = len(checksums)
        table["digest"] = canonical_json.canonical_sha256(checksums)


def coherent(doc, mass, free=None):
    """`doc` with its formed M replaced by `mass` ({(a, b): binary64}, a ≤ b) and every
    value that depends on M re-formed the way the producer forms it (exact sums, one
    rounding), resealed: T_k, the free-DOF masses, the centre, m_i, Γ, m_eff, the
    fractions and their cumulative sums. `free` optionally replaces the free set."""
    doc = copy.deepcopy(doc)
    p = me._closure(doc)
    if free is not None:
        index = 0
        for i, row in enumerate(doc["tables"]["dofs"]["rows"]):
            row["free_index"] = index if free[i] else None
            index += 1 if free[i] else 0
        doc["guard"]["free_dimension"] = index
    else:
        free = [f is not None for f in p.free]
    n = len(p.free)
    rows = []
    for i, (a, b) in enumerate(sorted(k for k, v in mass.items() if v != 0.0)):
        ta, tb = a % 6 < 3, b % 6 < 3
        pair = "translation_translation" if ta and tb else ("rotation_rotation" if not ta and not tb else "translation_rotation")
        rows.append({"row_id": f"mass:{p.dof_ids[a]}|{p.dof_ids[b]}", "ordinal": i, "row_dof_ref": p.dof_ids[a],
                     "col_dof_ref": p.dof_ids[b], "pair_class": pair,
                     "value": {"kind": "mass_matrix_entry", "value": mass[(a, b)], "unit": UNIT_BY_CLASS[pair],
                               "unit_status": "catalog", "below_normal": False, "undefined_reason": None}})
    doc["tables"]["mass_matrix"] = {"status": "published", "row_count": 0, "digest": "", "rows": rows}
    exact = {k: Fraction(v) for k, v in mass.items() if v != 0.0}
    product = [[Fraction(0)] * n for _ in range(3)]
    for (a, b), v in exact.items():
        if b % 6 < 3:
            product[b % 6][a] += v
        if a != b and a % 6 < 3:
            product[a % 6][b] += v
    totals = []
    for k, row in enumerate(doc["tables"]["mass_totals"]["rows"]):
        total = sum((v * (1 if a == b else 2) for (a, b), v in exact.items() if a % 6 == k and b % 6 == k), Fraction(0))
        free_total = sum((v * (1 if a == b else 2) for (a, b), v in exact.items()
                          if a % 6 == k and b % 6 == k and free[a] and free[b]), Fraction(0))
        _set(row["total_mass"], float(total))
        _set(row["free_dof_mass"], float(free_total))
        totals.append(float(total))
    nodes = [[Fraction(c) for c in node] for node in p.nodes]
    point = [sum((c[k] for c in nodes), Fraction(0)) / len(nodes) for k in range(3)]

    def moment(k):
        out = [Fraction(0)] * 3
        for i, c in enumerate(nodes):
            d = [c[a] - point[a] for a in range(3)]
            f = [product[k][6 * i + a] for a in range(3)]
            m = (d[1] * f[2] - d[2] * f[1], d[2] * f[0] - d[0] * f[2], d[0] * f[1] - d[1] * f[0])
            for a in range(3):
                out[a] += m[a] + product[k][6 * i + 3 + a]
        return out

    if doc["basis"]["mass_center"] is not None:
        mz, mx = moment(2), moment(0)
        centre = (point[0] - mz[1] / Fraction(totals[2]), point[1] + mz[0] / Fraction(totals[2]), point[2] + mx[1] / Fraction(totals[0]))
        for k, axis in enumerate("xyz"):
            _set(doc["basis"]["mass_center"][axis], float(centre[k]))
    cumulative = [Fraction(0)] * 3
    for m_index, mode_row in enumerate(doc["tables"]["modes"]["rows"]):
        phi = [Fraction(vo.value) for vo in p.shapes[m_index]]
        generalized = sum((phi[a] * v * phi[b] * (1 if a == b else 2) for (a, b), v in exact.items() if free[a] and free[b]),
                          Fraction(0))
        _set(mode_row["generalized_mass"], *_published(generalized))
        for k in range(3):
            q = sum((phi[a] * Fraction(float(product[k][a])) for a in range(n) if free[a]), Fraction(0))
            row = doc["tables"]["participation"]["rows"][3 * m_index + k]
            _set(row["participation_factor"], *_published(q / generalized))
            _set(row["effective_mass"], *_published(q * q / generalized))
            value, flagged = _published(q * q / (generalized * Fraction(totals[k])))
            _set(row["effective_mass_fraction"], value, flagged)
            cumulative[k] += Fraction(value)
            _set(row["cumulative_effective_mass_fraction"], float(cumulative[k]))
    _reseal(doc)
    return doc


def rebind(doc, request):
    """`doc` bound to `request` (hashes, the state case's object and the run id), for a
    request that differs only in what the model-derived mass reads."""
    doc = copy.deepcopy(doc)
    b = doc["binding"]
    b["request_sha256"] = canonical_json.canonical_sha256(request)
    b["model_sha256"] = canonical_json.canonical_sha256(request["model"])
    b["materials_sha256"] = canonical_json.canonical_sha256(request.get("materials", []))
    if b["state_case"] is not None:
        case = request["model"]["load_cases"][b["state_case"]["case_index"]]
        b["state_case"]["case_sha256"] = canonical_json.canonical_sha256(case)
    run = canonical_json.canonical_sha256({"request_sha256": b["request_sha256"],
                                           "integrity_parameters_sha256": b["integrity_parameters_sha256"]})
    doc["run"]["run_id"] = f"modal-run:{run[:16]}"
    return doc


def assert_payload_consistent(doc):
    """The payload relations alone (no request) admit the document."""
    verdict = me.read(doc)
    assert verdict["closure"]["status"] == "passed", verdict["closure"]
    assert all(r["status"] != "failed" for r in verdict["relations"].values()), {
        k: r["difference"] for k, r in verdict["relations"].items()}
    return verdict


def assert_model_rejects(doc, request, relation_name="mass", phrase=""):
    verdict = me.read(doc, request)
    standing = relation(verdict, relation_name)
    assert standing["status"] == "failed", (relation_name, standing)
    assert "model-derived" in standing["difference"] and phrase in standing["difference"], standing["difference"]
    assert verdict["decision"] == "rejected" and verdict["validation"]["status"] == "not_fully_validated"
    return verdict


# --- intact controls ----------------------------------------------------------------------------

@pytest.mark.parametrize("name", SOLVED)
def test_every_intact_fixture_agrees_with_its_models_mass(name):
    doc, request = fixture(name)
    verdict = me.read(doc, request)
    assert verdict["binding"]["status"] == "passed"
    for name_ in ("mass", "participation"):
        standing = relation(verdict, name_)
        assert standing["status"] != "failed", standing
        assert not any("model" in text for text in standing["unresolved"]), standing
    derived = mm.derive(request)
    assert derived.route == doc["basis"]["route"]


def test_the_resealing_harness_is_itself_coherent():
    """An intact control for every mutation below: the request's own M, re-formed and
    resealed, is admitted by both the payload relations and the model."""
    for name in ("arc", "component_body", "cantilever_whole_spectrum", "ex_n05"):
        doc, request = fixture(name)
        resealed = coherent(doc, formed(request))
        assert_payload_consistent(resealed)
        verdict = me.read(resealed, request)
        for name_ in ("mass", "participation"):
            assert relation(verdict, name_)["status"] != "failed", relation(verdict, name_)


def test_the_state_case_partition_is_checked_but_stays_conditional():
    for name in ("state_friction_sliding", "state_one_way_pressed"):
        doc, request = fixture(name)
        verdict = me.read(doc, request)
        for name_ in ("mass", "participation"):
            standing = relation(verdict, name_)
            assert standing["status"] == "scoped_unsupported", standing
        assert any("REPORTED nonlinear support states" in text for text in relation(verdict, "mass")["scope"])
        assert any("conditional" in text for text in relation(verdict, "participation")["scope"])
        assert verdict["validation"]["status"] == "not_fully_validated"
        # A reported state whose treatment contradicts the rule fails.
        tampered = copy.deepcopy(doc)
        row = tampered["tables"]["supports"]["rows"][0]
        row["treatment"] = "free" if row["treatment"] != "free" else "restrained"
        _reseal(tampered)
        assert relation(me.read(tampered, request), "mass")["status"] == "failed"


def test_alternate_units_and_a_shared_section_bind_to_the_same_mass():
    doc, request = fixture("component_body")
    alternate = copy.deepcopy(request)
    model = alternate["model"]
    model["project"]["units"]["length"] = "mm"
    for node in model["nodes"]:
        for axis in "xyz":
            node["position"][axis] = node["position"][axis] * 1000
    for pipe in model["pipe_segments"]:
        pipe["section"]["outside_diameter"] = _q(200, "mm")
        pipe["section"]["wall_thickness"] = _q(10, "mm")
    model["components"][0]["geometry"]["center_of_gravity"] = {"x": -500, "y": 250, "z": 0, "unit": "mm"}
    model["components"][0]["geometry"]["weight"] = _q(0.5, "kN")
    model["sections"] = [{"id": "section:s", "name": "s", "section_type": "pipe", "provenance": "invented",
                          "properties": {"outside_diameter": _q(200, "mm"), "wall_thickness": _q(10, "mm")}}]
    model["pipe_segments"][0]["section_ref"] = "section:s"
    # The conversions are exact here, so the canonical inputs are the original's bit for bit.
    assert mm.derive(alternate).coordinates == mm.derive(request).coordinates
    verdict = me.read(rebind(doc, alternate), alternate)
    assert verdict["binding"]["status"] == "passed"
    for name_ in ("mass", "participation"):
        assert relation(verdict, name_)["status"] == "passed", relation(verdict, name_)
    # A stale inline cache of the shared section is a refused domain.
    stale = copy.deepcopy(alternate)
    stale["model"]["pipe_segments"][0]["section"]["wall_thickness"] = _q(0.01, "m")
    standing = relation(me.read(rebind(doc, stale), stale), "mass")
    assert standing["status"] == "failed" and "SECTION_REFERENCE_CACHE_STALE" in standing["difference"]


# --- coherent mutations (the threat model) -----------------------------------------------------

def _mutate(request, edit):
    out = copy.deepcopy(request)
    edit(out["model"])
    return out


def _section_edit(key, value):
    def edit(model):
        for pipe in model["pipe_segments"]:
            if value is None:
                pipe["section"].pop(key, None)
            else:
                pipe["section"][key] = value
    return edit


def _component_edit(path, value):
    def edit(model):
        target = model["components"][0]
        for part in path[:-1]:
            target = target[part]
        target[path[-1]] = value
    return edit


def _bend_removed(model):
    model["components"] = [c for c in model["components"] if c["kind"] != "bend"]


def _bow_flipped(model):
    for pipe in model["pipe_segments"]:
        if pipe["id"] == "pipe:b-c":
            pipe["y_reference"] = {axis: -value for axis, value in pipe["y_reference"].items()}


WRONG_MODELS = [
    ("cantilever_whole_spectrum", "wrong wall", _section_edit("wall_thickness", _q(0.0085, "m"))),
    ("cantilever_whole_spectrum", "wrong metal density", _section_edit("material_density", _q(7800.0, "kg/m^3"))),
    ("cantilever_whole_spectrum", "omitted contents", _section_edit("contents_density", None)),
    ("cantilever_whole_spectrum", "added insulation", lambda m: [_section_edit("insulation_thickness", _q(0.05, "m"))(m),
                                                                  _section_edit("insulation_density", _q(100.0, "kg/m^3"))(m)]),
    ("cantilever_whole_spectrum", "a mill deduction on the bore", _section_edit("mill_tolerance", _q(0.001, "m"))),
    ("ex_n05", "a corrosion allowance on the exact bore", _section_edit("corrosion_allowance", _q(0.002, "m"))),
    ("arc", "the arc's chord mass in place of its half-arcs", _bend_removed),
    ("arc", "the arc bowing the other way", _bow_flipped),
    ("component_body", "the body omitted (pipe mass kept)", _component_edit(("mechanics_interface", "solver_consumption"), "not_solver_consumed")),
    ("component_body", "the body at the other end node", _component_edit(("node",), "b")),
    ("component_body", "a wrong centre of gravity", _component_edit(("geometry", "center_of_gravity"), {"x": 0.5, "y": 0.25, "z": 0, "unit": "m"})),
    ("component_body", "a wrong body weight", _component_edit(("geometry", "weight"), _q(505.0, "N"))),
]


@pytest.mark.parametrize("name,label,edit", WRONG_MODELS, ids=[f"{n}:{label}" for n, label, _ in WRONG_MODELS])
def test_a_coherently_wrong_formed_mass_is_rejected_against_the_fixed_request(name, label, edit):
    doc, request = fixture(name)
    wrong = coherent(doc, formed(_mutate(request, edit)))
    assert_payload_consistent(wrong)
    assert_model_rejects(wrong, request)


def test_a_double_counted_body_is_rejected():
    doc, request = fixture("component_body")
    original = formed(request)
    pipe_instead = formed(_mutate(request, _component_edit(("mechanics_interface", "solver_consumption"), "not_solver_consumed")))

    def without_member(model):
        model["pipe_segments"] = [p for p in model["pipe_segments"] if p["id"] != "body:v"]
        model["components"] = []

    rest = formed(_mutate(request, without_member))
    keys = set(original) | set(pipe_instead) | set(rest)
    double = {k: original.get(k, 0.0) + pipe_instead.get(k, 0.0) - rest.get(k, 0.0) for k in keys}
    wrong = coherent(doc, double)
    assert_payload_consistent(wrong)
    assert_model_rejects(wrong, request)


def _free_node_dofs(doc, components):
    p = me._closure(doc)
    return [i for i, f in enumerate(p.free) if f is not None and i % 6 in components]


def test_changes_invisible_to_a_m_r_k_are_rejected():
    """Rotary, torsional and Rayleigh-shaped edits that leave M·R_k, and so every T_k and
    every resultant, unchanged: only an entrywise comparison of M can see them."""
    doc, request = fixture("cantilever_whole_spectrum")
    base = formed(request)
    rotations = _free_node_dofs(doc, (3, 4, 5))
    rotary = dict(base)
    for dof in rotations:
        rotary[(dof, dof)] = rotary[(dof, dof)] * 1.5
    torsion = dict(base)
    for dof in (d for d in rotations if d % 6 == 3):
        torsion[(dof, dof)] = torsion[(dof, dof)] * 0.5
    # A Rayleigh-shaped edit: +δ on two translational diagonals, −δ on their coupling, so
    # each row of M·R_k and every T_k is unchanged. δ = 1 kg keeps every edited entry in
    # its binade, so each edit is exact in binary64 and the totals stay bit-identical.
    rayleigh = dict(base)
    a, b = 6 * 1 + 1, 6 * 2 + 1  # uy of the two free nodes
    delta = 1.0
    assert all(math.frexp(rayleigh[k])[1] == math.frexp(rayleigh[k] + s * delta)[1]
               for k, s in (((a, a), 1), ((b, b), 1), ((a, b), -1)))
    rayleigh[(a, a)] += delta
    rayleigh[(b, b)] += delta
    rayleigh[(a, b)] = rayleigh.get((a, b), 0.0) - delta
    for variant in (rotary, torsion, rayleigh):
        wrong = coherent(doc, variant)
        assert_payload_consistent(wrong)
        totals = [row["total_mass"]["value"] for row in wrong["tables"]["mass_totals"]["rows"]]
        assert totals == [row["total_mass"]["value"] for row in doc["tables"]["mass_totals"]["rows"]]
        assert_model_rejects(wrong, request, phrase="mass entry")


def test_a_dropped_restrained_coupling_is_rejected():
    """D-v: the coupling M_fc to restrained DOFs enters p_k; dropping it changes Γ and
    m_eff coherently, which the payload relations then accept."""
    doc, request = fixture("component_body")
    free = [f is not None for f in me._closure(doc).free]
    dropped = {k: (0.0 if free[k[0]] != free[k[1]] else v) for k, v in formed(request).items()}
    wrong = coherent(doc, dropped)
    assert_payload_consistent(wrong)
    assert_model_rejects(wrong, request, phrase="mass entry")


def test_a_wrong_restraint_partition_is_rejected():
    doc, request = fixture("cantilever_whole_spectrum")
    p = me._closure(doc)
    free = [f is not None for f in p.free]
    free[0] = True  # release the anchor's UX
    wrong = coherent(doc, formed(request), free=free)
    assert_payload_consistent(wrong)
    assert_model_rejects(wrong, request, phrase="partition")


def test_a_published_entry_one_ulp_off_is_inside_its_rounding_envelope():
    doc, request = fixture("arc")
    nudged = formed(request)
    key = max(nudged, key=lambda k: abs(nudged[k]))
    nudged[key] = math.nextafter(nudged[key], math.inf)
    verdict = me.read(coherent(doc, nudged), request)
    assert relation(verdict, "mass")["status"] == "passed"


# --- missing inputs, refused domains, rounding and resources -----------------------------------

def test_without_a_request_the_model_mass_is_scoped_never_passed():
    doc, _ = fixture("component_body")
    verdict = me.read(doc)
    for name_ in ("mass", "participation"):
        standing = relation(verdict, name_)
        assert standing["status"] == "scoped_unsupported"
        assert any("no request is supplied" in text for text in standing["scope"])
    assert verdict["validation"]["status"] == "not_fully_validated"
    assert {"mass", "participation"} <= set(verdict["validation"]["unmet"])


def test_a_failed_binding_never_feeds_the_model_mass():
    doc, request = fixture("component_body")
    other = _mutate(request, _component_edit(("geometry", "weight"), _q(505.0, "N")))
    verdict = me.read(doc, other)
    assert verdict["binding"]["status"] == "failed"
    assert any("binding did not pass" in text for text in relation(verdict, "mass")["scope"])


def test_a_withheld_payload_scopes_only_what_needs_it():
    doc, request = fixture("component_body")
    withheld = copy.deepcopy(doc)
    withheld["tables"]["mass_matrix"] = {"status": "withheld", "reason": "test"}
    verdict = me.read(withheld, request)
    standing = relation(verdict, "mass")
    assert standing["status"] == "scoped_unsupported"
    assert any("formed entries are not compared" in text for text in standing["scope"])
    # The totals are still held to the model: a resealed wrong M with the payload withheld fails.
    wrong = coherent(doc, formed(_mutate(request, _section_edit("contents_density", None))))
    wrong["tables"]["mass_matrix"] = {"status": "withheld", "reason": "test"}
    assert_model_rejects(wrong, request, phrase="T x")
    shapes = copy.deepcopy(doc)
    shapes["tables"]["shapes"] = {"status": "withheld", "reason": "test"}
    assert any("shapes withheld" in text for text in relation(me.read(shapes, request), "participation")["scope"])


def _refused(edit, name="component_body", phrase=""):
    doc, request = fixture(name)
    changed = _mutate(request, edit)
    standing = relation(me.read(rebind(doc, changed), changed), "mass")
    assert standing["status"] == "failed", standing
    assert "producer refuses" in standing["difference"] and phrase in standing["difference"], standing["difference"]


def test_a_solved_document_in_a_refused_domain_fails():
    _refused(lambda m: m["supports"].append({"id": "sk", "node": "c", "family": "line_stop", "restraints": ["UY"],
                                             "direction": {"x": 1, "y": 1, "z": 0}}), phrase="skew")
    _refused(lambda m: m["supports"].append({"id": "sn", "node": "c", "family": "snubber", "restraints": ["UY"]}), phrase="snubber")
    _refused(lambda m: m.__setitem__("model_settings", {"shear_model": "timoshenko_annulus_energy_v1"}), phrase="Timoshenko")
    _refused(_section_edit("material_density", None), phrase="material density")
    _refused(_section_edit("insulation_thickness", _q(0.05, "m")), phrase="insulation")
    _refused(lambda m: m["components"].append({"id": "v2", "kind": "flange", "node": "c",
                                               "geometry": {"rigid_pipe_ref": "body:v"}}), phrase="more than one")
    _refused(lambda m: m["supports"].append({"id": "g", "node": "c", "family": "nonlinear", "restraints": [],
                                             "nonlinear": {"behavior": "gap", "dof": "UZ"}}), phrase="gap")


def test_condition_contents_on_the_state_case_are_refused_never_replaced():
    doc, request = fixture("state_one_way_pressed")
    changed = copy.deepcopy(request)
    changed["model"]["load_cases"][0]["condition_use"] = {"contents_density": "cond:water"}
    standing = relation(me.read(rebind(doc, changed), changed), "mass")
    assert standing["status"] == "failed" and "condition contents" in standing["difference"]


def test_a_missing_acceleration_or_a_disagreeing_applied_record():
    doc, request = fixture("component_body")
    changed = copy.deepcopy(request)
    changed["model"]["load_cases"][0]["primitive_loads"].append({
        "id": "w", "category": "weight", "dimension": "force", "direction": "global_z", "target": {"type": "node", "node": "c"},
        "magnitude": _q(-500.0, "N"),
        "provenance": json.dumps({"method": mm.COMPONENT_WEIGHT_METHOD, "normalized_acceleration_m_per_s2": -9.81})})
    standing = relation(me.read(rebind(doc, changed), changed), "mass")
    assert standing["status"] == "scoped_unsupported" and any("not decided here" in t for t in standing["scope"])
    agreeing = copy.deepcopy(changed)
    record = json.loads(agreeing["model"]["load_cases"][0]["primitive_loads"][-1]["provenance"])
    record["normalized_acceleration_m_per_s2"] = -9.8125
    agreeing["model"]["load_cases"][0]["primitive_loads"][-1]["provenance"] = json.dumps(record)
    assert relation(me.read(rebind(doc, agreeing), agreeing), "mass")["status"] == "passed"


def test_a_comparison_outside_the_rounding_model_is_unresolved_never_passed():
    """Requests inside the canonical profile (|x| < 2^53) cannot overflow M, so the guard
    is exercised directly: an overflow or an undecidable interval operation leaves that
    one comparison unresolved, the other comparisons still run, and a contradiction
    propagates."""
    items = []

    def overflow():
        iv.binary64(iv.Iv(iv.LARGEST * 2))

    mm._guarded(items, "mass entry a", overflow)
    mm._guarded(items, "Γ", lambda: (iv.Iv(1) / iv.Iv(-1, 1), None)[1])
    mm._guarded(items, "T x", lambda: mm._judge("T x", _Vo(1.5), iv.Iv(1, 2)))
    assert len(items) == 2 and all(text.startswith(mm.UNRESOLVED) for text in items)
    with pytest.raises(mm.Contradiction):
        mm._guarded(items, "T y", lambda: mm._judge("T y", _Vo(3.0), iv.Iv(1, 2)))


class _Vo:
    def __init__(self, value=None, below=False, undefined=None):
        self.value, self.below, self.undefined = value, below, undefined


def test_the_published_value_domains_against_an_envelope():
    near_zero = iv.Iv(0, Fraction(1, 1 << 1000))
    unit = iv.Iv(1, 2)
    assert mm._judge("x", _Vo(1.5), unit) is None
    with pytest.raises(mm.Contradiction):
        mm._judge("x", _Vo(3.0), unit)
    assert mm._judge("x", _Vo(0.0, below=True), near_zero).startswith(mm.UNRESOLVED)
    with pytest.raises(mm.Contradiction):
        mm._judge("x", _Vo(0.0, below=True), unit)
    with pytest.raises(mm.Contradiction):
        mm._judge("x", _Vo(undefined="outside_canonical_profile"), unit)
    assert mm._judge("x", _Vo(undefined="outside_canonical_profile"), iv.Iv(1, iv.LARGEST)).startswith(mm.UNRESOLVED)
    with pytest.raises(mm.Contradiction):
        mm._judge("x", _Vo(undefined="not_finite"), unit)
    assert mm._judge("x", _Vo(undefined="total_mass_zero"), unit) is None  # a claim: its payload relation decides


def test_a_supplied_resource_budget_scopes_before_any_work():
    doc, request = fixture("component_body")
    verdict = me.read(doc, request, model_mass_budget=10)
    standing = relation(verdict, "mass")
    assert standing["status"] == "scoped_unsupported" and any(text.startswith("resources:") for text in standing["scope"])
    assert relation(me.read(doc, request, model_mass_budget=10 ** 9), "mass")["status"] == "passed"


def test_the_leaf_imports_only_the_standard_library_and_its_unit_and_interval_siblings():
    allowed = {"__future__", "fractions", "json", "math", "typing", "tests.verifiers", "tests.verifiers.modal_model_mass_interval"}
    for name in ("modal_model_mass.py", "modal_model_mass_interval.py"):
        tree = ast.parse((ROOT / "tests/verifiers" / name).read_text(encoding="utf-8"))
        imported = set()
        for node in ast.walk(tree):
            if isinstance(node, ast.Import):
                imported |= {alias.name for alias in node.names}
            elif isinstance(node, ast.ImportFrom):
                imported.add(node.module or "")
                if node.module == "tests.verifiers":
                    assert {alias.name for alias in node.names} <= {"units"}, name
        assert imported <= allowed, (name, imported)


def test_the_double_word_allowance_scales_with_its_contributions_and_frame_conditioning():
    """The stated premise (`modal_model_mass_interval`): 2^-90 of a contribution's scale per
    contribution plus one, so a long accumulation or an ill-conditioned member frame widens
    its allowance rather than leaving a correct producer value outside it."""
    scale = Fraction(3)
    assert iv.dd_bound(0, scale) == Fraction(3, 1 << 90)
    assert iv.dd_bound(9, scale) == 10 * iv.dd_bound(0, scale)
    # 1 + |y|/|y⊥|: 2 for a y reference normal to the chord, about 1 + 10^6 nearly parallel.
    normal = mm._frame_conditioning((0.0, 0.0, 0.0), (2.0, 0.0, 0.0), (0.0, 1.0, 0.0), "normal")
    assert Fraction(2) <= normal < Fraction(2) + Fraction(1, 1 << 40)
    steep = mm._frame_conditioning((0.0, 0.0, 0.0), (2.0, 0.0, 0.0), (1.0, 1e-6, 0.0), "steep")
    assert Fraction(10 ** 6) < steep < Fraction(10 ** 6 + 2)
    with pytest.raises(mm.RefusedDomain):
        mm._frame_conditioning((0.0, 0.0, 0.0), (2.0, 0.0, 0.0), (3.0, 0.0, 0.0), "parallel")


def test_a_frame_rounded_entry_that_cancels_exactly_is_inside_its_envelope():
    """Review of ea6165a2d, finding 1: the producer turns each member's local mass to global
    axes through a double-word frame from y (`rotate12`), so an entry that is exactly 0 in
    the frame-free form can be published as rounding residue. With y = (0, 1, 1.1) on the
    whole-spectrum cantilever, replaying the producer's operations gives
    M[n0.uy, n0.uz] = 3.944304526105059e-31. Its allowance scales with the rotation's
    absolute terms, not the cancelled entry."""
    doc, request = fixture("cantilever_whole_spectrum")

    def tilted(model):
        for pipe in model["pipe_segments"]:
            pipe["y_reference"] = {"x": 0, "y": 1, "z": 1.1}

    changed = _mutate(request, tilted)
    mass = formed(changed)
    assert mass.get((1, 2), 0.0) == 0.0
    mass[(1, 2)] = 3.944304526105059e-31
    verdict = me.read(coherent(rebind(doc, changed), mass), changed)
    assert relation(verdict, "mass")["status"] != "failed", relation(verdict, "mass")


def test_an_axis_aligned_direction_is_lowered_as_the_producer_lowers_it():
    """Review of ea6165a2d, finding 2 (`lower_axis_aligned_directions`): a record with an
    exactly axis-aligned direction has each DOF token on another axis set to that axis's
    translation. The anchor split into five restraints and a line stop authored UY along
    +x restores the complete anchor, so the unchanged document still passes. A direction
    record the producer refuses before lowering (`supports_findings`) fails instead."""
    doc, request = fixture("component_body")

    def split(**line_stop):
        def edit(model):
            model["supports"][0]["restraints"] = ["UY", "UZ", "RX", "RY", "RZ"]
            model["supports"].append({"id": "ls", "node": "a", "family": "line_stop", "restraints": ["UY"],
                                      "direction": {"x": 1, "y": 0, "z": 0}, **line_stop})
        return edit

    changed = _mutate(request, split())
    verdict = me.read(rebind(doc, changed), changed)
    assert relation(verdict, "mass")["status"] == "passed", relation(verdict, "mass")
    for variant, phrase in (({"family": "anchor"}, "SUPPORT_DIRECTION_RECORD_UNSUPPORTED"),
                            ({"restraints": ["RY"]}, "SUPPORT_DIRECTION_ROTATIONAL_UNSUPPORTED"),
                            ({"direction": {"x": 0, "y": 0, "z": 0}}, "SUPPORT_DIRECTION_ZERO"),
                            ({"direction": None}, "SUPPORT_DIRECTION_INVALID"),
                            ({"stiffness": {"dof": "UZ", "value": {"unit": "N/m", "value": 1e6}}},
                             "SUPPORT_DIRECTION_RECORD_UNSUPPORTED")):
        _refused(split(**variant), phrase=phrase)

    def null_on_anchor(model):  # review of 2f254a66b, finding 3: a present null is refused
        model["supports"][0]["direction"] = None

    _refused(null_on_anchor, phrase="SUPPORT_DIRECTION_INVALID")


def test_a_body_whose_spatial_product_underflows_is_refused():
    """Review of ea6165a2d, finding 3: the producer's range premise (`component_range`)
    refuses a nonzero m·r_i·r_j that rounds to zero; W = 500, a = 10 and a centre of gravity
    (1e-200, 0, 0) do that, so a solved document bound to it is a contradiction."""
    _refused(_component_edit(["geometry", "center_of_gravity"], {"unit": "m", "x": 1e-200, "y": 0, "z": 0}),
             phrase="MODAL_COMPONENT_MASS_OUT_OF_RANGE")


def test_a_body_whose_double_word_low_word_may_underflow_is_unresolved():
    """Review of 3599a206b, finding 1: W = 2.3195656276910255e-308, a = 1 and a centre of
    gravity (250575.49268495973, 2.116592064317826, 0) pass the producer's range premise,
    but its double-word m·r_a has a subnormal low word, so `component_mass` publishes
    M[rx, ry] = -1.2302189740973076e-302, outside the relative premise's enclosure. Below
    2^-969 the body's entries are outside the rounding model: unresolved, never failed.
    Every other comparison is still made, so a wrong pipe entry beside them fails."""
    doc, request = fixture("component_body")
    changed = _mutate(request, _component_edit(["geometry", "weight"], {"unit": "N", "value": 2.3195656276910255e-308}))
    changed = _mutate(changed, _component_edit(["geometry", "center_of_gravity"],
                                               {"unit": "m", "x": 250575.49268495973, "y": 2.116592064317826, "z": 0}))
    changed["weight_to_mass_acceleration"] = {"unit": "m/s^2", "value": 1.0}
    bound = rebind(doc, changed)
    bound["basis"]["weight_to_mass_acceleration"]["value"] = 1.0
    bound["binding"]["options"]["weight_to_mass_acceleration"]["value"] = 1.0
    produced = formed(changed)
    rx_ry = (15, 16)  # node c, the body's node
    assert produced[rx_ry] != -1.2302189740973076e-302
    produced[rx_ry] = -1.2302189740973076e-302  # what the producer publishes there
    published = coherent(bound, produced)
    assert_payload_consistent(published)
    verdict = me.read(published, changed)
    assert verdict["binding"]["status"] == "passed", verdict["binding"]
    mass = relation(verdict, "mass")
    assert mass["status"] == "unresolved", mass
    assert any("(15, 16)" in item and "2^-969" in item for item in model_texts(mass)), model_texts(mass)

    wrong = dict(produced)
    rotary = (3, 3)  # node a's rx diagonal, a pipe entry outside the body's block
    wrong[rotary] *= 1.25
    assert_model_rejects(coherent(bound, wrong), changed)


def test_a_mode_that_does_not_use_an_undecidable_block_is_still_compared():
    """Review of 2f254a66b, finding 2: beside the tiny body's undecidable block at node c,
    a mode shaped only at node b (φ[b:uy] = 1, φ[b:rx] = 0.5) uses none of it, so its m
    is still compared: M withheld and M[b:rx, b:rx] ×1.25 re-formed coherently fails."""
    doc, request = fixture("component_body")
    changed = _mutate(request, _component_edit(["geometry", "weight"], {"unit": "N", "value": 2.3195656276910255e-308}))
    changed = _mutate(changed, _component_edit(["geometry", "center_of_gravity"],
                                               {"unit": "m", "x": 250575.49268495973, "y": 2.116592064317826, "z": 0}))
    changed["weight_to_mass_acceleration"] = {"unit": "m/s^2", "value": 1.0}
    bound = rebind(doc, changed)
    bound["basis"]["weight_to_mass_acceleration"]["value"] = 1.0
    bound["binding"]["options"]["weight_to_mass_acceleration"]["value"] = 1.0
    shaped = {"dof:b:uy": 1.0, "dof:b:rx": 0.5}
    for row in bound["tables"]["shapes"]["rows"]:
        if row["mode_ref"] == "mode:1":
            row["value"]["value"] = shaped.get(row["dof_ref"], 0.0)
    _reseal(bound)
    produced = formed(changed)
    produced[(15, 16)] = -1.2302189740973076e-302
    b_rx = (9, 9)

    def read_withheld(mass):
        published = coherent(bound, mass)
        published["tables"]["mass_matrix"] = {"status": "withheld", "reason": "corpus: payload withheld"}
        _reseal(published)
        return me.read(published, changed)

    intact = read_withheld(produced)
    assert relation(intact, "participation")["status"] != "failed", relation(intact, "participation")
    wrong = dict(produced)
    wrong[b_rx] *= 1.25
    verdict = read_withheld(wrong)
    standing = relation(verdict, "participation")
    assert standing["status"] == "failed" and "mode 1: m" in standing["difference"], standing


def test_a_centre_whose_moment_products_underflow_is_inside_its_envelope():
    """Review of 2f254a66b, finding 1: point bodies of 2^-200 and 2^-199 kg at nodes a and
    c (zero offsets, a = 1) replace both spans, and c sits at (2, 2^-900, 0). The arm ×
    mass products underflow in the producer's plain double word (`nodal_resultant`), so it
    publishes the centre y as 3.943507287222582e-272 where the exact centre is about twice
    that. The underflow term (`DD_ABSOLUTE`) holds it; a value far outside still fails."""
    _, request = fixture("component_body")

    def edit(model):
        model["nodes"][2]["position"]["y"] = 2.0 ** -900
        valve = model["components"][0]
        valve["geometry"]["weight"] = {"unit": "N", "value": 2.0 ** -199}
        valve["geometry"]["center_of_gravity"] = {"unit": "m", "x": 0, "y": 0, "z": 0}
        twin = copy.deepcopy(valve)
        twin.update(id="component:twin", node="a")
        twin["geometry"].update(rigid_pipe_ref="pipe:p1", weight={"unit": "N", "value": 2.0 ** -200})
        model["components"].append(twin)

    changed = _mutate(request, edit)
    changed["weight_to_mass_acceleration"] = {"unit": "m/s^2", "value": 1.0}
    model = mm.derive(changed)
    assert sorted(model.bodies) == ["body:v", "pipe:p1"]
    rounded, outside = mm._rounded(model)
    assert not outside

    class View:
        def __init__(self, y):
            self.mass_center = [_Vo(4 / 3), _Vo(y), _Vo(0.0)]

    assert mm._centre(model, View(3.943507287222582e-272), rounded, 18, []) is None
    with pytest.raises(mm.Contradiction):
        mm._centre(model, View(1e-200), rounded, 18, [])


def test_a_frame_conditioning_at_binary64_resolution_is_unresolved_never_passed():
    """Review of 3599a206b, finding 2: y = (9e15, 1e-7, 0) on the whole-spectrum cantilever
    gives κ ≈ 9e22, where the premise's allowance 2^-90·(n + 1)·(1 + κ) reaches binary64
    resolution, so it would pass M scaled by 1.0001 (the producer's block for that y is
    identical to y = (0, 1, 0)'s). Such an entry is outside the rounding model."""
    doc, request = fixture("cantilever_whole_spectrum")

    def tilted(model):
        for pipe in model["pipe_segments"]:
            pipe["y_reference"] = {"x": 9e15, "y": 1e-7, "z": 0}

    changed = _mutate(request, tilted)
    wrong = coherent(rebind(doc, changed), {k: v * 1.0001 for k, v in formed(changed).items()})
    assert_payload_consistent(wrong)
    verdict = me.read(wrong, changed)
    for name in ("mass", "participation"):
        assert relation(verdict, name)["status"] == "unresolved", relation(verdict, name)
