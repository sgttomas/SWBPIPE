#!/usr/bin/env python3
"""The stated bases of the benchmark comparison (DEL-17-05; tools/caepipe_correlation/stated_bases.py).

Each basis's realization is validated here on an independent ground: a closed form, or the product itself. No value of
the target program's output is used; the prints are invented. SWBPIPE's side is the headless runner's solve
(OPENPIPESTRESS_RUNNER_BIN, which tests/conftest.py builds or takes from a preset).
"""

from __future__ import annotations

import copy
import json
import math
import os
import subprocess
import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.handoff.caepipe_external.benchmark import (  # noqa: E402
    FINDING, UNMATCHED, WITHIN, build_benchmark_record, effective_axial_deltas, swbpipe_values,
)
from core.handoff.caepipe_external.print_csv import parse_caepipe_print_csv  # noqa: E402
from core.handoff.caepipe_mbf import render_caepipe_mbf_si  # noqa: E402
from tools.caepipe_correlation import benchmark_models, run_pilot, stated_bases as sb  # noqa: E402
from tools.caepipe_correlation.benchmark_tranche3 import K_ROTATION, K_TRANSLATION  # noqa: E402

MODELS = ROOT / "validation" / "caepipe_benchmark" / "models"
MANIFEST = ROOT / "validation" / "caepipe_benchmark" / "manifest.json"
SUSTAINED = "case:sustained"
TO_SI = {"m": 1.0, "mm": 1.0e-3, "rad": 1.0, "deg": math.pi / 180.0, "N": 1.0, "kN": 1.0e3, "N*m": 1.0,
         "kN*m": 1.0e3}


def tracked(model_id: str) -> dict:
    return json.loads((MODELS / f"{model_id}.json").read_text(encoding="utf-8"))


def run_of(model_id: str, series: str) -> dict:
    return benchmark_models.series_document(tracked(model_id), series)


def solve(model: dict) -> list[dict]:
    runner = os.environ.get("OPENPIPESTRESS_RUNNER_BIN")
    if not runner:
        pytest.skip("OPENPIPESTRESS_RUNNER_BIN is not set")
    body = {"request": run_pilot.runner_request("stated-basis-test"),
            "solve": {"preview_model": {"model": model, "materials": []}}}
    completed = subprocess.run([runner, "solve", "--explicit-local-private-intent", "--input", "-"],
                               input=json.dumps(body), capture_output=True, text=True, check=False)
    assert completed.returncode == 0, (completed.returncode, completed.stderr[-2000:], completed.stdout[-4000:])
    output = json.loads(completed.stdout)
    assert output["payload"]["mechanics_envelope"]["status"]["mechanics"] == "MECHANICS_SOLVED"
    return output["payload"]["mechanics_envelope"]["results"]


def value(rows: list[dict], kind: str, entity: str, case: str = SUSTAINED, *, component: str | None = None,
          location: str | None = None) -> float:
    """One row's value in SI, which must be unique."""
    found = [row for row in rows if row["kind"] == kind and row["entity_ref"] == entity
             and row["basis_ref"]["ref_id"] == case
             and (component is None or (row.get("metadata") or {}).get("component") == component)
             and (location is None or (row.get("metadata") or {}).get("location") == location)]
    assert len(found) == 1, (kind, entity, case, component, location, len(found))
    return found[0]["value"] * TO_SI[found[0]["unit"]]


def reaction(rows: list[dict], support_ids: set[str], component: str, case: str = SUSTAINED) -> float:
    """The sum of the named supports' reaction component (a realized anchor's springs, summed as at its node)."""
    return math.fsum(row["value"] * TO_SI[row["unit"]] for row in rows
                     if row["kind"] == "support_reaction_component_v2" and row["entity_ref"] in support_ids
                     and row["basis_ref"]["ref_id"] == case and row["metadata"]["component"] == component)


def close(a: float, b: float, rel: float, absolute: float = 0.0) -> bool:
    return abs(a - b) <= absolute + rel * max(abs(a), abs(b))


def section(document: dict, pipe: str = "pipe:P1") -> tuple[float, float, float, float]:
    """(OD, t, E, nu) of a member of a pressure document."""
    model = document["model"]
    segment = next(item for item in model["pipe_segments"] if item["id"] == pipe)
    material = next(item for item in model["materials"] if item["id"] == segment["material"])
    return (segment["section"]["outside_diameter"]["value"], segment["section"]["wall_thickness"]["value"],
            material["elastic_modulus"]["value"], material["poisson_ratio"]["value"])


def pressure_of(document: dict) -> float:
    return document["model"]["load_cases"][0]["pressure_regions"][0]["pressure"]["value"]


# -- B: the effective axial force binding --------------------------------------------------------------------------
def test_b_an_anchored_closed_straight_has_wall_force_p_ai_and_zero_effective_end_force():
    """Closed form: an anchored closed straight under pressure with elongation off carries N_w = p A_i and S = 0, so
    under the effective binding its end force along the axis is zero at both ends, where the wall binding gives the
    end action -N_w at end_i and +N_w at end_j."""
    document = run_of("cpb-p1-01", benchmark_models.SERIES_DEFAULT)
    od, t, _, _ = section(document)
    p_ai = pressure_of(document) * math.pi * (od - 2.0 * t) ** 2 / 4.0
    rows = solve(document["model"])
    axes, missing = sb.member_end_axes(document["model"])
    assert missing == {} and axes[("pipe:P1", "end_i")] == (1.0, 0.0, 0.0)
    pressurised = sb.pressurised_members(document["model"])
    for end, sign in (("end_i", -1.0), ("end_j", 1.0)):
        assert close(value(rows, "pipe_wall_axial_force_v2", "pipe:P1", location=end), p_ai, 1e-9)
        assert abs(value(rows, "pipe_effective_axial_force_v2", "pipe:P1", location=end)) <= 1e-9 * p_ai
        assert close(value(rows, "element_global_end_force_v1", "pipe:P1", component="Fx", location=end),
                     sign * p_ai, 1e-9)
    deltas, missing_rows = effective_axial_deltas(rows, [SUSTAINED], axes, pressurised)
    assert missing_rows == {}
    support_nodes = {"support:A": "10", "support:B": "20"}
    wall, _ = swbpipe_values(rows, [SUSTAINED], support_nodes)
    effective, invalid = swbpipe_values(rows, [SUSTAINED], support_nodes, end_force_deltas=(deltas, missing_rows))
    assert not invalid
    for end, sign in (("end_i", -1.0), ("end_j", 1.0)):
        assert close(wall[("element_force", "pipe:P1", end, "x")], sign * p_ai, 1e-9)
        assert abs(effective[("element_force", "pipe:P1", end, "x")]) <= 1e-9 * p_ai
        for axis in "yz":  # only the axial part changes
            assert effective[("element_force", "pipe:P1", end, axis)] == wall[("element_force", "pipe:P1", end, axis)]


def test_b_an_end_without_its_rows_or_axis_is_unmatched_under_the_basis_never_taken_as_the_wall_value():
    rows = [{"kind": "element_global_end_force_v1", "entity_ref": "pipe:P1", "basis_ref": {"ref_id": SUSTAINED},
             "metadata": {"component": "Fx", "location": "end_i"}, "unit": "N", "value": -5.0}]
    deltas, missing = effective_axial_deltas(rows, [SUSTAINED], {("pipe:P1", "end_i"): (1.0, 0.0, 0.0)},
                                             {SUSTAINED: {"pipe:P1"}})
    assert deltas == {} and "does not carry" in missing[("pipe:P1", "end_i")]
    values, invalid = swbpipe_values(rows, [SUSTAINED], {}, end_force_deltas=(deltas, missing))
    assert ("element_force", "pipe:P1", "end_i", "x") in invalid and ("element_force", "pipe:P1", "end_i", "x") \
        not in values


def test_b_a_bend_end_takes_the_axis_of_the_tangent_straight():
    model = tracked("cpb-p4-01")["model"]
    axes, missing = sb.member_end_axes(model)
    assert missing == {}
    assert axes[("pipe:BEND1", "end_i")] == pytest.approx((1.0, 0.0, 0.0), abs=1e-15)  # along P1, into the arc
    assert axes[("pipe:BEND1", "end_j")] == pytest.approx((0.0, 0.0, 1.0), abs=1e-15)  # along P2, out of it


# -- C: Timoshenko straight members --------------------------------------------------------------------------------
def cantilever(model_id: str = "cpb-basis-test", *, span: float = 1.5, od: float = 168.3, wall: float = 14.0,
               tip: float = -50000.0) -> dict:
    """An invented anchored cantilever along X with one tip force and no weight (pressure-free)."""
    b = benchmark_models.Builder(model_id, 0)
    a, t = b.node("A", (0.0, 0.0, 0.0)), b.node("B", (span, 0.0, 0.0))
    b.run("P", [a, t], od, wall)
    b.anchor("A", a)
    b.force("tip", t, "global_y", tip)
    return {"model_id": model_id, "family": "G1", "piping_code": "B311-80", "inexact_inputs": {}, "closed_form": [],
            "model": b.model()}


def kappa_energy(ri: float, ro: float, nu: float) -> float:
    """The annulus energy shear factor in its expanded textbook form (not the product's cancellation-free order)."""
    m2 = (ri / ro) ** 2
    return 6.0 * (1.0 + nu) ** 2 * (1.0 + m2) ** 2 / ((7.0 + 14.0 * nu + 8.0 * nu * nu) * (1.0 + m2) ** 2
                                                      + 4.0 * m2 * (5.0 + 10.0 * nu + 4.0 * nu * nu))


def test_c_the_timoshenko_basis_solve_gives_the_closed_form_tip_deflection():
    entry = cantilever()
    assert sb.condition(entry, sb.SHEAR) == (True, "")
    document, aliases = sb.realize(entry, (sb.SHEAR,))
    assert aliases == {} and document["model"]["model_settings"] == {"shear_model": sb.TIMOSHENKO}
    assert "model_settings" not in entry["model"]  # the plain document is not changed
    _, e, nu, _, _ = benchmark_models.MATERIALS[0]
    span, ro, ri, p = 1.5, 0.1683 / 2.0, 0.1683 / 2.0 - 0.014, -50000.0
    area, inertia = math.pi * (ro ** 2 - ri ** 2), math.pi / 4.0 * (ro ** 4 - ri ** 4)
    g = e / (2.0 * (1.0 + nu))
    bending = p * span ** 3 / (3.0 * e * inertia)
    shear = p * span / (kappa_energy(ri, ro, nu) * g * area)
    euler = value(solve(entry["model"]), "global_nodal_displacement_y", "node:B")
    timoshenko = value(solve(document["model"]), "global_nodal_displacement_y", "node:B")
    assert close(euler, bending, 1e-9)
    assert close(timoshenko, bending + shear, 1e-9) and abs(shear) > 1e-3 * abs(bending)
    assert sb.shear_model_of(entry).startswith("euler_bernoulli") and sb.shear_model_of(document).startswith(
        sb.TIMOSHENKO)


def test_c_rests_on_the_run_declaration_and_on_what_the_document_selects():
    applies, why = sb.condition(tracked("cpb-p1-01"), sb.SHEAR)
    assert not applies and "pressure_contract" in why and sb.TIMOSHENKO in why
    # A tracked dynamics run (kind option_addition) is declared not screened: the basis rests on that declaration.
    dynamics = run_pilot.run_declarations(MANIFEST, "dynamics-comparable")["cpb-dyn-c-n08-flat"]
    assert dynamics["kind"] == "option_addition" and dynamics["screened"] is False
    applies, why = sb.condition(cantilever(), sb.SHEAR, dynamics)
    assert not applies and "declared not screened" in why and "headless modal" in why
    applies, why = sb.condition({**cantilever(), "capture_only": True}, sb.SHEAR)
    assert not applies and "capture-only" in why
    # A hand-entry run's declared screening waits for the window print route, which compares it.
    hand = run_pilot.run_declarations(MANIFEST, "hand-entry")["cpb-n-04"]
    assert hand["screened"] is False and sb.condition(cantilever(), sb.SHEAR, hand) == (True, "")
    # An explicit Euler-Bernoulli selection is still a plain Euler-Bernoulli solve: the basis applies.
    explicit = cantilever()
    explicit["model"]["model_settings"] = {"shear_model": sb.EULER_BERNOULLI}
    assert sb.condition(explicit, sb.SHEAR) == (True, "")
    assert sb.realize(explicit, (sb.SHEAR,))[0]["model"]["model_settings"] == {"shear_model": sb.TIMOSHENKO}
    windy = cantilever()
    windy["model"]["load_cases"][0]["equivalent_static"] = {"wind": {"exposed_spans": [{"pipe": "pipe:P1"}]}}
    applies, why = sb.condition(windy, sb.SHEAR)
    assert not applies and "wind exposed spans" in why


# -- D: anchors of the stated finite stiffness ---------------------------------------------------------------------
def test_d_an_anchor_on_springs_gives_the_closed_form_of_a_cantilever_on_a_rotational_spring():
    """Closed form: tip deflection P L^3/(3 E I) + P L^2/k_r + P/k_t, root rotation P L/k_r. Invented stiffnesses
    make the spring terms large enough to examine; the comparison uses the stated ones (next test)."""
    entry = cantilever()
    kt, kr = 2.0e8, 3.0e6
    document, aliases = sb.realize(entry, (sb.ANCHOR,), k_translation=kt, k_rotation=kr)
    springs = {spring for spring, anchor in aliases.items() if anchor == "support:A"}
    assert len(springs) == 6
    _, e, _, _, _ = benchmark_models.MATERIALS[0]
    span, ro, ri, p = 1.5, 0.1683 / 2.0, 0.1683 / 2.0 - 0.014, -50000.0
    inertia = math.pi / 4.0 * (ro ** 4 - ri ** 4)
    rows = solve(document["model"])
    expected_tip = p * span ** 3 / (3.0 * e * inertia) + p * span ** 2 / kr + p / kt
    assert close(value(rows, "global_nodal_displacement_y", "node:B"), expected_tip, 1e-9)
    assert close(value(rows, "global_nodal_rotation_z", "node:A"), p * span / kr, 1e-9)
    # The springs' reactions, summed as at the anchor's node, balance the tip force.
    assert close(reaction(rows, springs, "Fy"), -p, 1e-9)
    assert close(reaction(rows, springs, "Mz"), -p * span, 1e-9)


def test_d_the_comparison_uses_the_stiffness_the_public_page_states_and_refuses_an_anchor_that_moves():
    document, aliases = sb.realize(cantilever(), (sb.ANCHOR,))
    springs = [s for s in document["model"]["supports"] if s["id"] in aliases]
    assert {(s["restraints"][0], s["stiffness"]["value"]["value"], s["stiffness"]["value"]["unit"]) for s in springs} \
        == {("UX", K_TRANSLATION, "N/m"), ("UY", K_TRANSLATION, "N/m"), ("UZ", K_TRANSLATION, "N/m"),
            ("RX", K_ROTATION, "N*m/rad"), ("RY", K_ROTATION, "N*m/rad"), ("RZ", K_ROTATION, "N*m/rad")}
    # 1e12 lb/inch and 1e12 inch-lb/radian in SI.
    assert K_TRANSLATION == pytest.approx(1e12 * 4.4482216152605 / 0.0254, rel=1e-15)
    assert K_ROTATION == pytest.approx(1e12 * 4.4482216152605 * 0.0254, rel=1e-15)
    applies, why = sb.condition(tracked("cpb-g8-01"), sb.ANCHOR)
    assert not applies and "imposes a movement on anchor" in why
    rows = solve(document["model"])  # the stated stiffness solves
    assert abs(value(rows, "global_nodal_rotation_z", "node:A")) > 0.0


# -- E: the pressure bases -----------------------------------------------------------------------------------------
def thin_strain(document: dict) -> float:
    od, t, e, nu = section(document)
    return (1.0 - 2.0 * nu) * pressure_of(document) * (od - t) / (4.0 * t * e)


@pytest.mark.parametrize("model_id", ["cpb-p1-01", "cpb-p1-04"])
def test_e1_each_anchor_reaction_of_a_closed_straight_changes_by_the_thin_wall_force(model_id):
    """Closed form (P1): restrained, the thin-wall strain at the mean diameter carries E A_s eps = (1 - 2 nu) p pi
    D_m^2/4, since A_s = pi D_m t exactly."""
    on = run_of(model_id, benchmark_models.SERIES_ELONGATION_ON)
    off = run_of(model_id, benchmark_models.SERIES_DEFAULT)
    assert sb.condition(on, sb.THIN) == (True, "") and not sb.condition(off, sb.THIN)[0]
    document, _ = sb.realize(on, (sb.THIN,))
    assert "model_settings" not in document["model"]
    od, t, _, nu = section(on)
    change = (1.0 - 2.0 * nu) * pressure_of(on) * math.pi * (od - t) ** 2 / 4.0
    basis, plain_off = solve(document["model"]), solve(off["model"])
    for support, sign in (("support:A", 1.0), ("support:B", -1.0)):
        delta = reaction(basis, {support}, "Fx") - reaction(plain_off, {support}, "Fx")
        assert close(delta, sign * change, 1e-9), (support, delta, change)


@pytest.mark.parametrize("model_id", ["cpb-p2-01", "cpb-p2-03"])
def test_e1_a_free_end_moves_by_the_thin_wall_strain_times_the_length(model_id):
    on = run_of(model_id, benchmark_models.SERIES_ELONGATION_ON)
    off = run_of(model_id, benchmark_models.SERIES_DEFAULT)
    document, _ = sb.realize(on, (sb.THIN,))
    nodes = {node["id"]: node["position"] for node in on["model"]["nodes"]}
    pipe = on["model"]["pipe_segments"][0]
    a, b = nodes[pipe["from"]], nodes[pipe["to"]]
    length = math.dist([a[k] for k in "xyz"], [b[k] for k in "xyz"])
    axis = "xyz"[max(range(3), key=lambda k: abs(b["xyz"[k]] - a["xyz"[k]]))]
    held = pipe["from"] if any(s["node"] == pipe["from"] for s in on["model"]["supports"]) else pipe["to"]
    free = pipe["to"] if held == pipe["from"] else pipe["from"]
    outward = math.copysign(1.0, nodes[free][axis] - nodes[held][axis])  # away from the held end: elongation
    moved = value(solve(document["model"]), f"global_nodal_displacement_{axis}", free) \
        - value(solve(off["model"]), f"global_nodal_displacement_{axis}", free)
    assert close(moved, outward * thin_strain(on) * length, 1e-9), (moved, outward * thin_strain(on) * length)


def test_e2_the_straight_strain_realization_equals_swbpipes_own_elongation_on_a_closed_straight():
    """The product itself: on a straight-only model, SWBPIPE's thick-wall strain placed as a temperature interval on
    the straight members of the elongation-off document gives SWBPIPE's own elongation-on solve."""
    on = run_of("cpb-p1-01", benchmark_models.SERIES_ELONGATION_ON)
    model = copy.deepcopy(on["model"])
    members = sb._elongation_members(model)
    sb._without_elongation(model)
    sb._place_strains(model, members, thin_wall=False, straight_only=True)
    realized, own = solve(model), solve(on["model"])
    for support in ("support:A", "support:B"):
        assert close(reaction(realized, {support}, "Fx"), reaction(own, {support}, "Fx"), 1e-9, 1e-9)


@pytest.mark.parametrize("model_id", ["cpb-p4-01", "cpb-p5-01"])
def test_e2_on_a_bend_model_the_basis_is_elongation_off_plus_the_strain_on_straights_alone(model_id):
    """The product itself: the basis solve equals the elongation-off solve plus a solve of the same free strain alone
    on the straight members (linear superposition, the pressure and the strain in one case not interacting)."""
    on = run_of(model_id, benchmark_models.SERIES_ELONGATION_ON)
    off = run_of(model_id, benchmark_models.SERIES_DEFAULT)
    assert sb.condition(on, sb.STRAIGHT) == (True, "")
    document, _ = sb.realize(on, (sb.STRAIGHT,))
    # The reference strain loads are formed here, apart from the production placement: SWBPIPE's thick-wall strain
    # (1 - 2 nu) p A_i/(E A_s) as dT = eps/alpha on each straight member of a region of an elongation case.
    model = on["model"]
    arcs = {component["geometry"]["bend_pipe_ref"] for component in model["components"]
            if component["kind"] == "bend"}
    cases = set(model["model_settings"]["pressure_elongation"]["operating_load_cases"])
    alone = copy.deepcopy(off["model"])
    straights = 0
    for case in alone["load_cases"]:
        loads = []
        for region in (case.get("pressure_regions") or []) if case["id"] in cases else []:
            for pipe_id in region["member_pipe_ids"]:
                if pipe_id in arcs:
                    continue
                od, t, e, nu = section(on, pipe_id)
                segment = next(item for item in model["pipe_segments"] if item["id"] == pipe_id)
                material = next(item for item in model["materials"] if item["id"] == segment["material"])
                di = od - 2.0 * t
                eps = (1.0 - 2.0 * nu) * region["pressure"]["value"] * di ** 2 / (e * (od ** 2 - di ** 2))
                loads.append({"category": "thermal", "dimension": "temperature_interval", "direction": "global_x",
                              "id": f"load:reference-strain:{pipe_id}", "provenance": "invented test reference",
                              "magnitude": {"unit": "degC",
                                            "value": eps / material["thermal_expansion_coefficient"]["value"]},
                              "target": {"pipe": pipe_id, "type": "element"}})
                straights += 1
        case["primitive_loads"] = loads
        case.pop("pressure_regions", None)
    assert straights >= 2
    alone.pop("pressure_contract")
    for pipe in alone["pipe_segments"]:  # zero on the exact route; the pressure-free route has no such field
        assert pipe["section"].pop("corrosion_allowance")["value"] == 0.0
    for material in alone["materials"]:  # the pressure-free route's material basis: E and G
        nu = material.pop("poisson_ratio")["value"]
        material.pop("constitutive_basis")
        material["shear_modulus"] = {"unit": "Pa", "value": material["elastic_modulus"]["value"] / (2.0 * (1.0 + nu))}
    alone["model_settings"] = {"shear_model": sb.TIMOSHENKO}  # the exact route's straight members are Timoshenko
    basis, plain_off, strain = solve(document["model"]), solve(off["model"]), solve(alone)
    assert not any(load["target"]["pipe"] in arcs for case in document["model"]["load_cases"]
                   for load in case["primitive_loads"] if load["id"].startswith(sb.BASIS_LOAD_PREFIX))
    supports = {s["id"] for s in on["model"]["supports"]}
    scale = max(abs(reaction(basis, {s}, c)) for s in supports for c in ("Fx", "Fy", "Fz"))
    for support in supports:
        for component in ("Fx", "Fy", "Fz", "Mx", "My", "Mz"):
            expected = reaction(plain_off, {support}, component) + reaction(strain, {support}, component)
            assert close(reaction(basis, {support}, component), expected, 1e-7, 1e-9 * scale), (support, component)


def test_e2_does_not_apply_without_a_bend_and_e1_e2_only_on_the_elongation_on_run():
    applies, why = sb.condition(run_of("cpb-p1-01", benchmark_models.SERIES_ELONGATION_ON), sb.STRAIGHT)
    assert not applies and "no realized bend" in why
    for name in (sb.THIN, sb.STRAIGHT):
        applies, why = sb.condition(run_of("cpb-p4-01", benchmark_models.SERIES_DEFAULT), name)
        assert not applies and "pressure-elongation-on" in why


def test_e3_a_compensated_terminal_transferring_to_the_wall_leaves_no_axial_reaction():
    """Closed form: a closed cantilever under pressure with elongation off carries no axial reaction at its anchor;
    with the stated terminal it carries the unbalanced thrust p A_i."""
    p3 = [document for document in (tracked(f"cpb-p3-{i:02d}") for i in range(1, 5))]
    for document in p3:
        off = benchmark_models.series_document(document, benchmark_models.SERIES_DEFAULT)
        assert sb.condition(off, sb.TERMINAL) == (True, "")
        realized, _ = sb.realize(off, (sb.TERMINAL,))
        od, t, _, _ = section(off)
        p_ai = pressure_of(off) * math.pi * (od - 2.0 * t) ** 2 / 4.0
        anchor = next(s for s in off["model"]["supports"] if s.get("family") == "anchor")
        pipe = off["model"]["pipe_segments"][0]
        nodes = {node["id"]: node["position"] for node in off["model"]["nodes"]}
        axis = max("xyz", key=lambda k: abs(nodes[pipe["to"]][k] - nodes[pipe["from"]][k])).upper()
        [terminals] = [region["terminals"] for case in off["model"]["load_cases"]
                       for region in case.get("pressure_regions") or []][:1]
        [wall] = [t["node_ref"] for t in terminals if t["closure_transfer"] == sb.TRANSFERS]
        [other] = [t["node_ref"] for t in terminals if t["closure_transfer"] != sb.TRANSFERS]
        outward = math.copysign(1.0, nodes[wall][axis.lower()] - nodes[other][axis.lower()])
        anchor = anchor["id"]
        plain, basis = solve(off["model"]), solve(realized["model"])
        # The closure that transfers to the wall takes the unbalanced thrust p A_i, outward from the pipe's inside at
        # its end; the anchor holds the pipe against it: its action on the pipe is -p A_i along that outward axis.
        assert close(reaction(plain, {anchor}, f"F{axis.lower()}"), -outward * p_ai, 1e-9), document["model_id"]
        assert abs(reaction(basis, {anchor}, f"F{axis.lower()}")) <= 1e-9 * p_ai, document["model_id"]
    applies, why = sb.condition(tracked("cpb-p1-01"), sb.TERMINAL)
    assert not applies and "transfers to the wall" in why


# -- the record: the plain comparison is kept, each basis beside it ------------------------------------------------
def mirror_print(document: dict, rows: list[dict]) -> tuple[object, str]:
    from tests.test_caepipe_benchmark import mirror_csv

    return mirror_csv(document, {"mechanics_envelope": {"results": rows}})


def test_the_record_keeps_the_plain_comparison_and_lists_every_basis_with_its_condition():
    entry = run_of("cpb-p4-01", benchmark_models.SERIES_ELONGATION_ON)
    rows = solve(entry["model"])
    payload = {"mechanics_envelope": {"results": rows}}
    mbf, text = mirror_print(entry, rows)
    parsed = parse_caepipe_print_csv(text)
    plain = build_benchmark_record(entry, swbpipe_result_payload=payload, mbf_result=mbf, mbf_text=mbf.text,
                                   parsed_caepipe=parsed)
    before = json.dumps(plain, sort_keys=True)
    solved, documents = [], {}

    def basis_solve(document, names):
        solved.append(names)
        documents[names] = document
        return {"mechanics_envelope": {"results": solve(document["model"])}}, "", {"stated_bases": list(names)}
    bases = sb.compare_bases(entry, plain_payload=payload, plain_record=plain, mbf_result=mbf, mbf_text=mbf.text,
                             parsed_caepipe=parsed, solve=basis_solve)
    assert json.dumps(plain, sort_keys=True) == before  # the plain comparison is not changed
    by_name = {block["name"]: block for block in bases["bases"]}
    assert list(by_name) == [sb.EFFECTIVE_AXIAL, *sb.SOLVE_BASES, sb.TOGETHER]
    for block in by_name.values():
        assert block["condition"] and block["reason"] and block["ground"] and block["realization"]
        assert block["outcome"] in sb.OUTCOMES and (block["applied"] or block["not_applied"])
    assert not by_name[sb.SHEAR]["applied"] and not by_name[sb.TERMINAL]["applied"]
    assert by_name[sb.ANCHOR]["applied"] and by_name[sb.THIN]["applied"] and by_name[sb.STRAIGHT]["applied"]
    assert by_name[sb.TOGETHER]["bases_combined"] == [sb.ANCHOR, sb.THIN, sb.STRAIGHT]
    assert solved == [(sb.ANCHOR,), (sb.THIN,), (sb.STRAIGHT,), (sb.ANCHOR, sb.THIN, sb.STRAIGHT)]
    assert bases["plain"]["shear_model"].startswith(sb.TIMOSHENKO)

    # Together: every change of each basis is in the one document, and the thin-wall strain is on straights only.
    together = documents[(sb.ANCHOR, sb.THIN, sb.STRAIGHT)]["model"]
    anchors = [s for s in entry["model"]["supports"] if s.get("family") == "anchor"]
    assert anchors and not [s for s in together["supports"] if s.get("family") == "anchor"]
    assert len([s for s in together["supports"] if s.get("family") == "spring"]) == 6 * len(anchors)
    assert "pressure_elongation" not in (together.get("model_settings") or {})
    arcs = {c["geometry"]["bend_pipe_ref"] for c in entry["model"]["components"] if c["kind"] == "bend"}
    placed = {load["target"]["pipe"]: load["magnitude"]["value"] for case in together["load_cases"]
              for load in case.get("primitive_loads") or [] if load["id"].startswith(sb.BASIS_LOAD_PREFIX)}
    straights = {pipe for case in entry["model"]["load_cases"] for region in case.get("pressure_regions") or []
                 for pipe in region["member_pipe_ids"]} - arcs
    assert set(placed) == straights and straights
    for pipe_id, interval in placed.items():
        od, t, e, nu = section(entry, pipe_id)
        segment = next(item for item in entry["model"]["pipe_segments"] if item["id"] == pipe_id)
        material = next(item for item in entry["model"]["materials"] if item["id"] == segment["material"])
        thin = (1.0 - 2.0 * nu) * pressure_of(entry) * (od - t) / (4.0 * t * e)
        assert close(interval, thin / material["thermal_expansion_coefficient"]["value"], 1e-12), pipe_id

    # The anchor basis: the springs' reactions are bound to the anchor's node (support_aliases); without that mapping
    # each anchor's rows would have CAEPIPE's value alone and be unmatched.
    anchor_rows = [row for row in by_name[sb.ANCHOR]["results"] if row.get("kind") in ("support_force",
                                                                                         "support_moment")]
    assert anchor_rows and all("screen" in row and row["values"]["swbpipe"] is not None for row in anchor_rows)

    # The effective axial binding, on the bend: each axis at each end is the wall value plus s (S - N_w) t, with t the
    # tangent straight's axis (+X into the arc at end_i, +Z out of it at end_j) and s = -1 at end_i, +1 at end_j; only
    # cases where the member is pressurised (sustained, and the operating sum) carry rows.
    axial = by_name[sb.EFFECTIVE_AXIAL]
    assert axial["applied"] and axial["members"] == ["pipe:BEND1", "pipe:P1", "pipe:P2"]
    assert {row["case"] for row in axial["results"]} == {SUSTAINED, "operating"}
    assert axial["counts"][FINDING] > 0 and axial["counts"][UNMATCHED] == 0
    tangent = {"end_i": (1.0, 0.0, 0.0), "end_j": (0.0, 0.0, 1.0)}
    checked = 0
    for row in axial["results"]:
        if row["entity"] != "pipe:BEND1" or row["case"] != SUSTAINED:
            continue
        end = row["end"]
        change = value(rows, "pipe_effective_axial_force_v2", "pipe:BEND1", location=end) - value(
            rows, "pipe_wall_axial_force_v2", "pipe:BEND1", location=end)
        expected = row["wall_binding"]["swbpipe"] + {"end_i": -1.0, "end_j": 1.0}[end] * change * tangent[end][
            "xyz".index(row["component"])]
        assert close(row["values"]["swbpipe"], expected, 1e-12, 1e-9), (end, row["component"])
        checked += 1
    assert checked == 6 and abs(change) > 1.0


def test_a_basis_the_product_refuses_is_not_applied_and_lists_its_refusal():
    entry = cantilever()
    mbf = render_caepipe_mbf_si(entry["model"], title="INVENTED", piping_code=entry["piping_code"])
    plain = {"results": []}
    bases = sb.compare_bases(entry, plain_payload={"mechanics_envelope": {"results": []}}, plain_record=plain,
                             mbf_result=mbf, mbf_text=mbf.text, parsed_caepipe=None,
                             solve=lambda document, names: (None, "runner exit 1; not a clean solve", {}))
    by_name = {block["name"]: block for block in bases["bases"]}
    for name in (sb.SHEAR, sb.ANCHOR, sb.TOGETHER):
        assert by_name[name]["applied"] is False and by_name[name]["outcome"] == sb.REFUSED_BY_PRODUCT
        assert by_name[name]["refusal"] == "runner exit 1; not a clean solve" and "results" not in by_name[name]


def no_solve(document, names):
    raise AssertionError("no basis is solved here")


def bases_of(entry: dict, solve=no_solve, declaration=None) -> dict:
    mbf = render_caepipe_mbf_si(entry["model"], title="INVENTED", piping_code=entry["piping_code"])
    return {block["name"]: block for block in sb.compare_bases(
        entry, plain_payload={"mechanics_envelope": {"results": []}}, plain_record={"results": []}, mbf_result=mbf,
        mbf_text=mbf.text, parsed_caepipe=None, solve=solve, declaration=declaration)["bases"]}


@pytest.mark.parametrize("alpha, words", [
    ({"unit": "1/degC", "value": 0.0}, "zero or not finite"), (None, "absent or not in 1/degC or 1/K"),
    ({"unit": "1/F", "value": 1e-5}, "absent or not in 1/degC or 1/K")])
def test_a_pressure_basis_that_cannot_be_placed_is_not_representable_and_never_raises(alpha, words):
    """The tracked P4 elongation-on run with an expansion coefficient the placement cannot use (in memory): the
    pressure bases end as not_representable with the reason, never an exception; the other bases are unaffected."""
    entry = run_of("cpb-p4-01", benchmark_models.SERIES_ELONGATION_ON)
    for material in entry["model"]["materials"]:
        if alpha is None:
            material.pop("thermal_expansion_coefficient")
        else:
            material["thermal_expansion_coefficient"] = alpha
    by_name = bases_of(entry, solve=lambda document, names: (None, "not solved here", {}))
    for name in (sb.THIN, sb.STRAIGHT):
        assert by_name[name]["applied"] is False and by_name[name]["outcome"] == sb.NOT_REPRESENTABLE
        assert words in by_name[name]["not_applied"]
    assert by_name[sb.ANCHOR]["outcome"] == sb.REFUSED_BY_PRODUCT  # unaffected: solved (here refused) on its own
    assert by_name[sb.TOGETHER]["outcome"] == sb.NOT_APPLICABLE  # one usable solve basis: nothing to combine
    with pytest.raises(sb.NotRepresentable):
        sb.realize(entry, (sb.THIN,))


def test_a_kelvin_coefficient_and_a_zero_mill_tolerance_are_represented():
    entry = run_of("cpb-p4-01", benchmark_models.SERIES_ELONGATION_ON)
    reference = run_of("cpb-p4-01", benchmark_models.SERIES_ELONGATION_ON)
    for material in entry["model"]["materials"]:
        material["thermal_expansion_coefficient"]["unit"] = "1/K"
    for pipe in entry["model"]["pipe_segments"]:
        pipe["section"]["mill_tolerance"] = {"unit": "mm", "value": 0}
    assert sb.representation(entry, sb.THIN) == "" and sb.representation(entry, sb.STRAIGHT) == ""
    loads = lambda e: [load["magnitude"] for case in sb.realize(e, (sb.THIN,))[0]["model"]["load_cases"]
                       for load in case.get("primitive_loads") or [] if load["id"].startswith(sb.BASIS_LOAD_PREFIX)]
    assert loads(entry) == loads(reference) and loads(entry)
    entry["model"]["pipe_segments"][0]["section"]["mill_tolerance"] = {"unit": "mm", "value": 0.5}
    assert "mill tolerance" in sb.representation(entry, sb.THIN)


def test_a_run_declared_not_screened_compares_no_basis():
    dynamics = run_pilot.run_declarations(MANIFEST, "dynamics-comparable")["cpb-dyn-c-n08-flat"]
    by_name = bases_of(cantilever(), declaration=dynamics)
    assert set(by_name) == {sb.EFFECTIVE_AXIAL, *sb.SOLVE_BASES, sb.TOGETHER}
    assert all(block["outcome"] == sb.NOT_COMPARED and not block["applied"] for block in by_name.values())


def test_the_effective_binding_is_not_applied_where_no_pressurised_end_force_is_bound():
    """A pressurised member in a case the print does not have: no row is bound, so the binding is not applied."""
    entry = run_of("cpb-p1-01", benchmark_models.SERIES_DEFAULT)
    by_name = bases_of(entry, solve=lambda document, names: (None, "not solved here", {}))
    axial = by_name[sb.EFFECTIVE_AXIAL]
    assert axial["applied"] is False and axial["outcome"] == sb.NOT_APPLICABLE and axial["results"] == []
