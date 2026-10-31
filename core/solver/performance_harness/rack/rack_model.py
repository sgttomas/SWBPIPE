#!/usr/bin/env python3
"""Deterministic generator of the RT-U4 reference racks (DEL-04-05, release target criterion 9).

A reference rack is an invented pipe-rack model document of an exact node count, used to time the linear static
and the nonlinear operating solve. Every value in it (geometry, sections, material, components, supports, loads) is
invented for this performance measurement; none is library, supplier, standard or project data, and the rack is a
test object, not a design.

Layout (global Z up, the rack along +x). A header of section S270 starts at an anchor, runs through `b` identical
bays and ends in a tail run and a second anchor. One bay, in header order:

- 16 straight header nodes at 1.5 m spacing (s1 .. s16). s4, s12 and s16 are patterned rests; s8 is the stop
  station (a vertical support with one line stop, UX). Patterned rests are numbered along the whole header, the loop
  top and the tail (index i from 1); every third (i % 3 == 0) is a guide (UZ + UY) instead of a vertical support;
- one horizontal expansion loop of four realized 90-degree bends (radius 0.405 m), out along +y and back: two legs of
  three 1.5 m spans and a top run of two, 13 nodes; the top run's middle node is a patterned rest;
- one S170 branch at s2, along +y in even bays and -y in odd bays: a 0.5 m spool, a flanged valve (flange, valve,
  flange: rigid components with invented weights), then eight 1.5 m spans; vertical supports at the first and fifth
  run node and an anchor (the equipment nozzle) at the eighth.

A bay has 41 nodes. The tail has 4 + r nodes, where r absorbs the remainder so that the node count is exact; tail
rests sit at every fourth tail node short of the end anchor. The rack is a tree, so members = nodes - 1.

Profiles: `linear-ex` (criterion 9's linear static, exact route EX), `nonlinear-ex` (criterion 9's nonlinear
operating, EX) and `nonlinear-pf` (the nonlinear twin on the pressure-free route). Only what the product admits is
emitted; each target family or case form that it refuses is recorded in LEDGER with its interim form, the refusal
observed and what it waits for.

The model document version is read at run time from core/product_physics/src/model_format.rs; it is never written
here. When the version moves, tests/test_performance_rack_model.py fails and names the re-check.

Usage:
    python3 core/solver/performance_harness/rack/rack_model.py --profile linear-ex --nodes 2000 [-o rack.json]
    python3 core/solver/performance_harness/rack/rack_model.py --list-profiles
    python3 core/solver/performance_harness/rack/rack_model.py --ledger
    python3 core/solver/performance_harness/rack/rack_model.py --hashes > core/solver/performance_harness/observations/rack/models_sha256.txt

The document (canonical JSON: sorted keys, no whitespace) goes to -o or stdout; a one-line JSON stats object goes to
stderr.
"""
from __future__ import annotations

import argparse
from collections import Counter
import hashlib
import json
import math
from pathlib import Path
import re
import sys

MODEL_FORMAT = Path(__file__).resolve().parents[3] / "product_physics" / "src" / "model_format.rs"


def read_model_version(path: Path = MODEL_FORMAT) -> str:
    """The current model document version, `pub const MODEL_SCHEMA_VERSION: &str = "...";` in model_format.rs."""
    match = re.search(r'pub const MODEL_SCHEMA_VERSION: &str = "([^"]+)";', path.read_text(encoding="utf-8"))
    if match is None:
        raise SystemExit(f"rack_model: MODEL_SCHEMA_VERSION not found in {path}")
    return match.group(1)


MODEL_VERSION = read_model_version()

PROVENANCE = "invented_rt_u4_perf_rack_not_library_or_project_data"
SOURCE = "invented user value for the RT-U4 performance rack (DEL-04-05); not library, supplier, standard or project data"
EX_CONTRACT = {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"}
LEDGER_BASIS = ("observed on main at 7e061cf2e (model document schema of that revision) through the headless runner's "
                "solve, sparse_interactive")

# -- invented inputs ----------------------------------------------------------------------------------------------
SPACING = 1.5  # m, header and branch node spacing
ELEVATION = 6.0  # m, rack elevation (Z)
STRAIGHT = 16  # straight header nodes per bay
BRANCH_AT = 2  # the branch leaves the header at s2
STOP_AT = 8  # the stop station (vertical support + line stop) at s8
REST_AT = (4, 12, 16)  # patterned rests on the straight header
BEND_RADIUS = 0.405  # m
LEG_SPANS, TOP_SPANS = 3, 2  # 1.5 m spans of each loop leg and of the loop top
LOOP_LEG = BEND_RADIUS * 2 + LEG_SPANS * SPACING  # corner-to-corner loop height (m)
LOOP_WIDTH = BEND_RADIUS * 2 + TOP_SPANS * SPACING  # corner-to-corner loop width (m)
BEND_K, BEND_SIF_IN, BEND_SIF_OUT = 5.0, 2.2, 1.8
# Branch: spool, flange A, valve, flange B (m), then RUN spans of 1.5 m.
SPOOL, FLANGE_LENGTH, VALVE_LENGTH = 0.5, 0.15, 0.6
BRANCH_RUN = 8
BRANCH_RESTS = (1, 5)  # run nodes with a vertical support; the last run node is the anchor
FLANGE_WEIGHT, VALVE_WEIGHT, RIGID_SCALE = 220.0, 1800.0, 10.0  # N, N, DEC-045 stiffness scale
VALVE_COG_RISE = 0.25  # m, valve CoG above the body midpoint (actuator)
TAIL_MIN = 4
SCALING_SIZES = (250, 500, 1000, 1500, 2000)  # criterion 9's 2,000 nodes and the scaling series
BAY_NODES = STRAIGHT + (8 + 2 * (LEG_SPANS - 1) + (TOP_SPANS - 1)) + (4 + BRANCH_RUN)  # 16 + 13 + 12 = 41
BASE_NODES = 1 + TAIL_MIN

# Material RK-M1 (invented, carbon-steel-like): (degC, GPa) points, nu, alpha (1/K), density (kg/m^3).
TEMPERATURE_POINTS = [(20, 200.0), (100, 195.0), (200, 189.0), (300, 182.0)]
POISSON, ALPHA, STEEL_DENSITY = 0.30, 1.2e-05, 7850.0
# Sections (invented, non-standard): OD mm, wall mm, mill tolerance mm, corrosion allowance mm, insulation mm and
# kg/m^3, contents kg/m^3.
SECTIONS = {"S270": (270.0, 9.0, 1.0, 1.5, 60.0, 140.0, 820.0), "S170": (170.0, 7.0, 0.8, 1.5, 50.0, 140.0, 820.0)}
GRAVITY = 9.80665  # m/s^2, standard gravity (a definitional constant), along -Z
INSTALLED_C, OPERATING_C = 20.0, 180.0
OPERATING_KPA, TEST_KPA, TEST_CONTENTS = 1500.0, 2250.0, 1000.0
WIND_KPA, WIND_SHAPE, WIND_DIRECTION = 0.75, 0.65, "global_y"
SEISMIC = {"EQX": ("g_factor_x", 0.25), "EQY": ("g_factor_y", 0.25), "EQZ": ("g_factor_z", 0.15)}
FRICTION_MU, GAP_M = 0.30, 0.005

# -- profiles -------------------------------------------------------------------------------------------------------
PROFILES = {
    "linear-ex": {
        "route": "EX",
        "nonlinear": False,
        "min_nodes": BASE_NODES + BAY_NODES,
        "description": "criterion 9 linear static on the exact route: pressure regions, the static case set "
                       "(SUS, OPE, HYD interim, WIN, EQX, EQY, EQZ) and its subtractions",
        "timed": "every case and combination of the document",
    },
    "nonlinear-ex": {
        "route": "EX",
        "nonlinear": True,
        "min_nodes": BASE_NODES + BAY_NODES + 5,
        "description": "criterion 9 nonlinear operating on the exact route: OPE (W + P + T); lift-off, friction and "
                       "gap supports in their interim linear forms until EX admits nonlinear supports",
        "timed": "OPE",
    },
    "nonlinear-pf": {
        "route": "PF",
        "nonlinear": True,
        "min_nodes": BASE_NODES + BAY_NODES + 5,
        "description": "nonlinear-ex's interim twin on the pressure-free route: OPE (W + T) with lift-off, friction "
                       "and gap supports, no pressure contract, regions or pressure",
        "timed": "OPE",
    },
}

# -- pending-family ledger ------------------------------------------------------------------------------------------
NONLINEAR_DENSE_GUARD_NODES = 1775  # the first nonlinear-pf node count the dense-loop memory guard refuses
TWIN_NOTE = ("admitted on the pressure-free route; omitted from nonlinear-pf so that the twin keeps nonlinear-ex's "
             "topology")
LEDGER = [
    {"family": "tee at each branch junction",
     "target_form": "component kind tee at the junction header node with user branch SIFs (header and branch pipe "
                    "refs, connection angle 90 degrees)",
     "interim_form": "plain three-pipe junction with no tee component",
     "refusal_code": "EXACT_PRESSURE_FAMILY_NOT_ADMITTED",
     "waits_for": "RT-A3 increment 1 (tee)",
     "profiles": ["linear-ex", "nonlinear-ex", "nonlinear-pf"],
     "note": f"nonlinear-pf: {TWIN_NOTE}"},
    {"family": "reducer at the branch connection (S270 header to S170 branch)",
     "target_form": "reducer component on the branch's first member, inside the branch's pressure region",
     "interim_form": "omitted: the branch is S170 from the junction",
     "refusal_code": "EXACT_PRESSURE_REDUCER_IN_REGION",
     "waits_for": "RT-A3 increment 2 (reducer in region)",
     "profiles": ["linear-ex", "nonlinear-ex", "nonlinear-pf"],
     "note": f"nonlinear-pf: {TWIN_NOTE}"},
    {"family": "nonlinear supports on the exact route (lift-off, friction shoe, gap)",
     "target_form": "nonlinear lift_off UZ at rests i % 3 == 1; friction UX (mu 0.30, normal from the rest) at rests "
                    "i % 12 in {2, 5, 8}; gap UY (5 mm, closes on positive displacement) replacing the guide's UY at "
                    "guides i % 6 == 0 (nonlinear-pf's supports)",
     "interim_form": "the linear forms: lift-off rests stay vertical supports, friction shoes are omitted, gapped "
                     "guides keep their linear UY; the OPE solve is linear",
     "refusal_code": "EXACT_PRESSURE_FAMILY_NOT_ADMITTED",
     "waits_for": "RT-A3 increment 4 (nonlinear on EX)",
     "profiles": ["nonlinear-ex"]},
    {"family": "lift-off friction shoe (friction normal from a one-way rest)",
     "target_form": "the friction shoe's normal_reaction_source names a one_way UZ rest at the same node",
     "interim_form": "normal_reaction_source names a linear vertical support UZ at the same node",
     "refusal_code": "NONLINEAR_FRICTION_NORMAL_SOURCE_INVALID",
     "waits_for": "RT-A3 S7 (lift-off friction shoe)",
     "profiles": ["nonlinear-ex", "nonlinear-pf"]},
    {"family": "pressure on the nonlinear twin",
     "target_form": "nonlinear-ex's pressure contract, regions and operating pressure",
     "interim_form": "omitted: nonlinear-pf is pressure-free (no pressure_contract, regions, mill tolerance or "
                     "corrosion allowance; E and G material)",
     "refusal_code": "EXACT_PRESSURE_FAMILY_NOT_ADMITTED (nonlinear supports on EX)",
     "waits_for": "RT-A3 increment 4 (nonlinear on EX)",
     "profiles": ["nonlinear-pf"]},
    {"family": "nonlinear operating solve above about 1,760 nodes",
     "target_form": "the nonlinear OPE solve of the rack",
     "interim_form": "document emitted unchanged; the product refuses the solve before any case",
     "refusal_code": "NONLINEAR_DENSE_MEMORY_LIMIT_EXCEEDED",
     "waits_for": "RT-A3 stage P (nonlinear above ~1,760 nodes)",
     "profiles": ["nonlinear-pf"],
     "min_nodes": NONLINEAR_DENSE_GUARD_NODES},
    {"family": "SAM: anchor movement on the end anchor, with dSAM",
     "target_form": "analysis_state.support_states[].boundary_motion (absolute_reference_displacement) on the end "
                    "anchor, in the document that carries OPE, WIN and EQX/EQY/EQZ",
     "interim_form": "omitted",
     "refusal_code": "EXACT_EQUIVALENT_STATIC_LOAD_STATE_UNSUPPORTED (with the load/reference-state group every "
                     "case's wind and seismic generation is refused); EXACT_IMPOSED_DISPLACEMENT_PRIMITIVE_UNSUPPORTED "
                     "(a support-targeted primitive on a plain EX document)",
     "waits_for": "#1678 / load-cases lane",
     "profiles": ["linear-ex"]},
    {"family": "HYD hydrotest",
     "target_form": "test_condition {kind: hydrostatic, held_supports} with condition_use.contents_density",
     "interim_form": "plain case: weight with the test contents density (1000 kg/m^3) and the test pressure, no "
                     "test_condition, no held supports",
     "refusal_code": "HEADLESS_RUNNER_SOLVE_PAYLOAD_INVALID (unknown field test_condition)",
     "waits_for": "model document 0.9.0 S1, then RT-A6",
     "profiles": ["linear-ex"]},
    {"family": "EQ-SRSS, EQ-ABS, OCC-SRSS, OCC-ABS",
     "target_form": "srss and absolute_sum over dEQX, dEQY, dEQZ; mechanics combinations with combination operands",
     "interim_form": "omitted",
     "refusal_code": "LOAD_COMBINATION_BASIS_UNSUPPORTED",
     "waits_for": "model document 0.9.0 S1, then RT-A4",
     "profiles": ["linear-ex"]},
    {"family": "EXP difference_basis",
     "target_form": "result_state_subtraction OPE - SUS with its difference_basis recorded",
     "interim_form": "result_state_subtraction without a basis",
     "refusal_code": "not authorable at the current model document",
     "waits_for": "model document 0.9.0 S1, then RT-A5 slice 1b",
     "profiles": ["linear-ex"]},
    {"family": "operating condition set",
     "target_form": "one condition set (temperature, pressure, contents) applied to every member",
     "interim_form": "authored per case (thermal primitives, pressure regions, weight line loads)",
     "refusal_code": "not authorable at the current model document",
     "waits_for": "model document 0.9.0 S1 (RT-G5)",
     "profiles": ["linear-ex", "nonlinear-ex", "nonlinear-pf"]},
]


def ledger_for(profile: str, nodes: int | None = None) -> list[dict]:
    """The ledger entries that apply to a profile (and, for size-bound entries, to a node count)."""
    return [entry for entry in LEDGER if profile in entry["profiles"]
            and (nodes is None or nodes >= entry.get("min_nodes", 0))]


# -- helpers --------------------------------------------------------------------------------------------------------
def q(value, unit):
    return {"value": value, "unit": unit}


def r6(value: float) -> float:
    return round(value, 6) + 0.0  # + 0.0 turns -0.0 into 0.0


def mass_per_length(section: str, exact: bool, contents: float) -> float:
    """kg/m on the product's self-weight mass basis: pipe metal on the nominal wall, contents on the bore (on EX the
    reduced bore: wall less mill tolerance and corrosion allowance), insulation outside the OD."""
    od, wall, mill, ca, ins, ins_rho, _ = SECTIONS[section]
    od, wall, mill, ca, ins = (v / 1000.0 for v in (od, wall, mill, ca, ins))
    area = lambda d: math.pi / 4.0 * d * d  # noqa: E731
    bore = od - 2.0 * (wall - mill - ca) if exact else od - 2.0 * wall
    return STEEL_DENSITY * (area(od) - area(od - 2.0 * wall)) + contents * area(bore) + ins_rho * (
        area(od + 2.0 * ins) - area(od))


def section_record(section: str, exact: bool) -> dict:
    od, wall, mill, ca, ins, ins_rho, contents = SECTIONS[section]
    record = {"outside_diameter": q(od, "mm"), "wall_thickness": q(wall, "mm"),
              "material_density": q(STEEL_DENSITY, "kg/m^3"), "contents_density": q(contents, "kg/m^3"),
              "insulation_thickness": q(ins, "mm"), "insulation_density": q(ins_rho, "kg/m^3")}
    if exact:
        record.update({"mill_tolerance": q(mill, "mm"), "corrosion_allowance": q(ca, "mm")})
    return record


# -- layout ---------------------------------------------------------------------------------------------------------
def bays_and_tail(nodes: int) -> tuple[int, int]:
    bays = (nodes - BASE_NODES) // BAY_NODES
    return bays, TAIL_MIN + (nodes - BASE_NODES - bays * BAY_NODES)


class Rack:
    """The geometry and support pattern of one rack, independent of route and load cases."""

    def __init__(self, nodes: int):
        self.bays, self.tail = bays_and_tail(nodes)
        self.nodes: list[dict] = []
        self.pipes: list[dict] = []  # {id, from, to, section, kind: straight|arc|rigid, y_ref}
        self.bends: list[dict] = []
        self.rigid: list[dict] = []
        self.rests: list[str] = []  # patterned rest nodes in header order
        self.stops: list[str] = []
        self.branch_rests: list[str] = []
        self.anchors: list[str] = []
        self.junctions: list[str] = []
        self.branches: list[dict] = []  # {sign, nodes: [junction, ...], pipes: [...]}
        self.header_path: list[str] = []
        self.header_pipes: list[str] = []
        self._header(nodes)

    def _node(self, nid, x, y, z=ELEVATION):
        self.nodes.append({"id": nid, "position": {"x": r6(x), "y": r6(y), "z": r6(z)}})
        return nid

    def _pipe(self, pid, a, b, section, kind, y_ref):
        self.pipes.append({"id": pid, "from": a, "to": b, "section": section, "kind": kind, "y_ref": y_ref})
        return pid

    def _header_node(self, x, y):
        index = len(self.header_path)
        nid = self._node(f"node:RK-H{index:04d}", x, y)
        if self.header_path:
            self.header_pipes.append(self._pipe(f"pipe:RK-H{index:04d}", self.header_path[-1], nid, "S270",
                                                "straight", (0.0, 0.0, 1.0)))
        self.header_path.append(nid)
        return nid

    def _arc(self, near, x, y, incoming):
        """The far tangent node of a realized bend whose near tangent is `near` (the last header node)."""
        index = len(self.header_path)
        nid = self._node(f"node:RK-H{index:04d}", x, y)
        pid = self._pipe(f"pipe:RK-H{index:04d}", near, nid, "S270", "arc", incoming)
        self.header_pipes.append(pid)
        self.header_path.append(nid)
        self.bends.append({"id": f"component:RK-BD{len(self.bends) // 4:03d}-{len(self.bends) % 4 + 1}",
                           "node": near, "pipe": pid})
        return nid

    def _header(self, nodes):
        x = 0.0
        self.anchors.append(self._header_node(x, 0.0))
        for bay in range(self.bays):
            for j in range(1, STRAIGHT + 1):
                x += SPACING
                nid = self._header_node(x, 0.0)
                if j == BRANCH_AT:
                    self.junctions.append(nid)
                    self._branch(bay, nid, x)
                elif j == STOP_AT:
                    self.stops.append(nid)
                elif j in REST_AT:
                    self.rests.append(nid)
            # Expansion loop: corners (c, 0), (c, LEG), (c + WIDTH, LEG), (c + WIDTH, 0).
            x += SPACING
            c, r = x + BEND_RADIUS, BEND_RADIUS
            near = self._header_node(x, 0.0)
            self._arc(near, c, r, (1.0, 0.0, 0.0))
            for k in range(1, LEG_SPANS):
                self._header_node(c, r + k * SPACING)
            near = self._header_node(c, LOOP_LEG - r)
            self._arc(near, c + r, LOOP_LEG, (0.0, 1.0, 0.0))
            for k in range(1, TOP_SPANS):
                nid = self._header_node(c + r + k * SPACING, LOOP_LEG)
                if k == TOP_SPANS // 2:
                    self.rests.append(nid)
            near = self._header_node(c + LOOP_WIDTH - r, LOOP_LEG)
            self._arc(near, c + LOOP_WIDTH, LOOP_LEG - r, (1.0, 0.0, 0.0))
            for k in range(1, LEG_SPANS):
                self._header_node(c + LOOP_WIDTH, LOOP_LEG - r - k * SPACING)
            near = self._header_node(c + LOOP_WIDTH, r)
            self._arc(near, c + LOOP_WIDTH + r, 0.0, (0.0, -1.0, 0.0))
            x = c + LOOP_WIDTH + r
        for j in range(1, self.tail + 1):
            x += SPACING
            nid = self._header_node(x, 0.0)
            if j == self.tail:
                self.anchors.append(nid)
            elif j % 4 == 0:
                self.rests.append(nid)
        assert len(self.nodes) == nodes, (len(self.nodes), nodes)

    def _branch(self, bay, junction, x):
        sign = 1.0 if bay % 2 == 0 else -1.0
        offsets = [SPOOL, SPOOL + FLANGE_LENGTH, SPOOL + FLANGE_LENGTH + VALVE_LENGTH,
                   SPOOL + 2 * FLANGE_LENGTH + VALVE_LENGTH]
        offsets += [offsets[-1] + k * SPACING for k in range(1, BRANCH_RUN + 1)]
        ids, pipes = [junction], []
        for j, offset in enumerate(offsets, start=1):
            nid = self._node(f"node:RK-B{bay:03d}-{j:02d}", x, sign * offset)
            kind = "rigid" if j in (2, 3, 4) else "straight"
            pipes.append(self._pipe(f"pipe:RK-B{bay:03d}-{j:02d}", ids[-1], nid, "S170", kind, (0.0, 0.0, 1.0)))
            ids.append(nid)
        for j, (tag, kind, weight, rise) in zip((2, 3, 4), (("FA", "flange", FLANGE_WEIGHT, 0.0),
                                                            ("V", "valve", VALVE_WEIGHT, VALVE_COG_RISE),
                                                            ("FB", "flange", FLANGE_WEIGHT, 0.0))):
            length = offsets[j - 1] - offsets[j - 2]
            # CoG from the component node (the mapped member's far end) to the body midpoint, plus the rise.
            self.rigid.append({"id": f"component:RK-{tag}{bay:03d}", "kind": kind, "node": ids[j],
                               "pipe": pipes[j - 1], "length": length, "weight": weight,
                               "cog": (0.0, r6(-sign * length / 2.0), rise)})
        for run in BRANCH_RESTS:
            self.branch_rests.append(ids[4 + run])
        self.anchors.append(ids[-1])
        self.branches.append({"sign": sign, "nodes": ids, "pipes": pipes})


# -- the document ---------------------------------------------------------------------------------------------------
def material(exact: bool) -> dict:
    """EX: the E/nu basis with temperature points (OPE selects the modulus at the operating temperature). PF: the
    pressure-free route's material, E and G = E / (2 (1 + nu)) at the installed point."""
    e = TEMPERATURE_POINTS[0][1]
    record = {"id": "material:RK-M1", "label": "RK-M1 invented carbon-steel-like material",
              "elastic_modulus": q(e, "GPa"), "thermal_expansion_coefficient": q(ALPHA, "1/K"),
              "provenance": PROVENANCE}
    if not exact:
        record["shear_modulus"] = q(r6(e / (2.0 * (1.0 + POISSON))), "GPa")
        return record
    record.update({"constitutive_basis": "homogeneous_isotropic_E_nu_v1", "poisson_ratio": q(POISSON, "1"),
                   "temperature_points": [
                       {"id": f"point:RK-M1-T{t:03d}", "temperature": q(t, "degC"), "elastic_modulus": q(gpa, "GPa"),
                        "poisson_ratio": q(POISSON, "1"), "thermal_expansion_coefficient": q(ALPHA, "1/K"),
                        "provenance": PROVENANCE} for t, gpa in TEMPERATURE_POINTS]})
    return record


def components(rack: Rack) -> list[dict]:
    rows = []
    for bend in rack.bends:
        rows.append({
            "id": bend["id"], "kind": "bend", "node": bend["node"],
            "geometry": {"bend_pipe_ref": bend["pipe"], "bend_radius": q(BEND_RADIUS * 1000.0, "mm"),
                         "bend_geometry_source_reference": SOURCE},
            "modifiers": {"flexibility_factor_user_value": q(BEND_K, "none"),
                          "sif_in_plane_user_value": q(BEND_SIF_IN, "1"),
                          "sif_out_of_plane_user_value": q(BEND_SIF_OUT, "1"), "source_reference": SOURCE},
            "mechanics_interface": {"solver_consumption": "curved_bend_macro_element"},
            "provenance": PROVENANCE})
    pipes = {p["id"]: p for p in rack.pipes}
    for rigid in rack.rigid:
        pipe = pipes[rigid["pipe"]]
        od = SECTIONS[pipe["section"]][0]
        rows.append({
            "id": rigid["id"], "kind": rigid["kind"], "node": rigid["node"],
            "geometry": {"rigid_pipe_ref": rigid["pipe"], "rigid_body_length": q(r6(rigid["length"]), "m"),
                         "end_a_size": q(od, "mm"), "end_b_size": q(od, "mm"),
                         "connection_end_a_reference": f"{pipe['from']} (body end on {rigid['pipe']})",
                         "connection_end_b_reference": f"{pipe['to']} (body end on {rigid['pipe']})",
                         "stiffness_behavior_reference": f"rigid body, user stiffness scale {RIGID_SCALE} (DEC-045); "
                                                         f"{SOURCE}",
                         "weight": q(rigid["weight"], "N"),
                         "center_of_gravity": {"x": rigid["cog"][0], "y": rigid["cog"][1], "z": rigid["cog"][2],
                                               "unit": "m"},
                         "rigid_component_source_reference": SOURCE},
            "modifiers": {"stiffness_scaling_user_value": q(RIGID_SCALE, "none"), "source_reference": SOURCE},
            "mechanics_interface": {"solver_consumption": "mechanics_geometry_only"},
            "provenance": PROVENANCE})
    return rows


def support(sid, node, family, restraints, nonlinear=None):
    row = {"id": sid, "node": node, "family": family, "restraints": restraints, "provenance": PROVENANCE}
    if nonlinear is not None:
        row["nonlinear"] = nonlinear
    return row


def supports(rack: Rack, nonlinear: bool) -> list[dict]:
    """Anchors, stop stations and branch rests are linear in every profile. Patterned rest i (from 1): a guide when
    i % 3 == 0, else a vertical support; with nonlinear supports, rest i % 3 == 1 lifts off, rest i % 12 in {2, 5, 8}
    carries a friction shoe whose normal comes from its own vertical support, and guide i % 6 == 0 replaces its
    lateral UY by a 5 mm gap."""
    rows = []
    tag = lambda node: node.split(":", 1)[1][3:]  # noqa: E731  ("node:RK-H0012" -> "H0012")
    for node in rack.anchors:
        rows.append(support(f"support:RK-A-{tag(node)}", node, "anchor", ["UX", "UY", "UZ", "RX", "RY", "RZ"]))
    for node in rack.stops:
        rows.append(support(f"support:RK-V-{tag(node)}", node, "vertical_support", ["UZ"]))
        rows.append(support(f"support:RK-LS-{tag(node)}", node, "line_stop", ["UX"]))
    for node in rack.branch_rests:
        rows.append(support(f"support:RK-V-{tag(node)}", node, "vertical_support", ["UZ"]))
    for i, node in enumerate(rack.rests, start=1):
        if i % 3 == 0:
            if nonlinear and i % 6 == 0:
                rows.append(support(f"support:RK-V-{tag(node)}", node, "vertical_support", ["UZ"]))
                rows.append(support(f"support:RK-GP-{tag(node)}", node, "nonlinear", [], {
                    "behavior": "gap", "dof": "UY", "initial_state": "inactive",
                    "closes_when": "positive_displacement", "gap": q(GAP_M, "m")}))
            else:
                rows.append(support(f"support:RK-G-{tag(node)}", node, "guide", ["UZ", "UY"]))
        elif nonlinear and i % 3 == 1:
            rows.append(support(f"support:RK-LO-{tag(node)}", node, "nonlinear", [], {
                "behavior": "lift_off", "dof": "UZ", "initial_state": "active", "contact_when": "positive_reaction"}))
        else:
            rest = f"support:RK-V-{tag(node)}"
            rows.append(support(rest, node, "vertical_support", ["UZ"]))
            if nonlinear and i % 12 in (2, 5, 8):
                rows.append(support(f"support:RK-FR-{tag(node)}", node, "nonlinear", [], {
                    "behavior": "friction", "dof": "UX", "initial_state": "sticking",
                    "friction_coefficient": q(FRICTION_MU, "none"),
                    "normal_reaction_source": {"support_ref": rest, "dof": "UZ"}}))
    return rows


def weight_loads(rack: Rack, case: str, exact: bool, contents: float) -> list[dict]:
    """W: a weight line load on every member but the rigid bodies, and each rigid body's weight at its centre of
    gravity (a nodal force and the moment of its offset) in place of its member's distributed weight."""
    loads = []
    w = {section: -GRAVITY * mass_per_length(section, exact, contents) for section in SECTIONS}
    for pipe in rack.pipes:
        if pipe["kind"] == "rigid":
            continue
        loads.append({"id": f"load:{case}-W-{pipe['id'].split(':', 1)[1][3:]}", "category": "weight",
                      "dimension": "force_per_length", "direction": "global_z",
                      "magnitude": q(w[pipe["section"]], "N/m"), "target": {"type": "element", "pipe": pipe["id"]},
                      "provenance": PROVENANCE})
    for rigid in rack.rigid:
        weight = rigid["weight"]
        rx, ry, _ = rigid["cog"]
        tag = rigid["id"].split(":", 1)[1][3:]
        loads.append({"id": f"load:{case}-W-{tag}-FZ", "category": "weight", "dimension": "force",
                      "direction": "global_z", "magnitude": q(-weight, "N"),
                      "target": {"type": "node", "node": rigid["node"]}, "provenance": PROVENANCE})
        # r x (0, 0, -W) = (-W ry, W rx, 0)
        for axis, moment in (("x", -weight * ry), ("y", weight * rx)):
            if moment != 0.0:
                loads.append({"id": f"load:{case}-W-{tag}-M{axis.upper()}", "category": "weight",
                              "dimension": "moment", "direction": f"rotation_{axis}", "magnitude": q(r6(moment), "N*m"),
                              "target": {"type": "node", "node": rigid["node"]}, "provenance": PROVENANCE})
    return loads


def thermal_loads(rack: Rack, case: str) -> list[dict]:
    return [{"id": f"load:{case}-T-{pipe['id'].split(':', 1)[1][3:]}", "category": "thermal",
             "dimension": "temperature_interval", "direction": "global_x",
             "magnitude": q(OPERATING_C - INSTALLED_C, "degC"), "target": {"type": "element", "pipe": pipe["id"]},
             "provenance": PROVENANCE} for pipe in rack.pipes]


def regions(rack: Rack, case: str, kpa: float) -> list[dict]:
    """The header split at every branch junction and each branch its own region; every terminal transfers to the
    wall. Valves, flanges and bends lie inside their regions."""
    ends = [rack.header_path[0], *rack.junctions, rack.header_path[-1]]
    position = {nid: i for i, nid in enumerate(rack.header_path)}
    spans = [(f"HR{k:03d}", rack.header_pipes[position[a]:position[b]], (a, b))
             for k, (a, b) in enumerate(zip(ends, ends[1:]))]
    spans += [(f"BR{k:03d}", branch["pipes"], (branch["nodes"][0], branch["nodes"][-1]))
              for k, branch in enumerate(rack.branches)]
    return [{"id": f"region:{case}-{tag}", "member_pipe_ids": list(members),
             "pressure_basis": "internal_differential_zero_external_v1", "pressure": q(kpa, "kPa"),
             "terminals": [{"node_ref": node, "closure_transfer": "transfers_to_wall", "provenance": PROVENANCE}
                           for node in terminals],
             "provenance": PROVENANCE} for tag, members, terminals in spans]


def load_cases(rack: Rack, profile: str) -> tuple[list[dict], list[dict]]:
    exact = PROFILES[profile]["route"] == "EX"
    op_contents = SECTIONS["S270"][6]

    def case(tag, label, *, contents=op_contents, kpa=OPERATING_KPA, thermal=False, extra=None):
        cid = f"load:RK-{tag}"
        loads = weight_loads(rack, f"RK-{tag}", exact, contents)
        if thermal:
            loads += thermal_loads(rack, f"RK-{tag}")
        record = {"id": cid, "label": label, "primitive_loads": loads, "provenance": PROVENANCE}
        if exact:
            record["pressure_regions"] = regions(rack, f"RK-{tag}", kpa)
        if thermal and exact:
            record["modulus_basis_temperature"] = q(OPERATING_C, "degC")
        if extra:
            record["equivalent_static"] = {**extra, "provenance": PROVENANCE}
        return record

    if profile != "linear-ex":
        label = "OPE operating: W + P + T" if exact else "OPE operating: W + T (pressure-free twin)"
        return [case("OPE", label, thermal=True)], []
    every_pipe = [pipe["id"] for pipe in rack.pipes]
    cases = [
        case("SUS", "SUS sustained: W + P"),
        case("OPE", "OPE operating: W + P + T", thermal=True),
        case("HYD", "HYD interim: W with test contents + test pressure", contents=TEST_CONTENTS, kpa=TEST_KPA),
        case("WIN", "WIN: W + P + wind", extra={"wind": {
            "pressure": q(WIND_KPA, "kPa"), "shape_factor": q(WIND_SHAPE, "1"), "direction": WIND_DIRECTION,
            "exposed_pipe_refs": every_pipe}}),
    ]
    for tag, (factor, value) in SEISMIC.items():
        cases.append(case(tag, f"{tag}: W + P + seismic {factor} {value}", extra={"seismic": {
            "gravity_acceleration": q(GRAVITY, "m/s^2"), factor: q(value, "1")}}))
    combinations = [{"id": f"combination:RK-{tag}", "label": f"{tag}: {minuend} - SUS",
                     "basis": "result_state_subtraction", "minuend_id": f"load:RK-{minuend}",
                     "subtrahend_id": "load:RK-SUS", "provenance": PROVENANCE}
                    for tag, minuend in (("EXP", "OPE"), ("dWIN", "WIN"), ("dEQX", "EQX"), ("dEQY", "EQY"),
                                         ("dEQZ", "EQZ"))]
    return cases, combinations


def build(profile: str, nodes: int) -> tuple[dict, dict]:
    """The model document of `profile` with exactly `nodes` nodes, and its stats."""
    if profile not in PROFILES:
        raise ValueError(f"unknown profile {profile!r}; profiles: {', '.join(PROFILES)}")
    spec = PROFILES[profile]
    if not isinstance(nodes, int) or nodes < spec["min_nodes"]:
        raise ValueError(f"profile {profile} needs at least {spec['min_nodes']} nodes (one {BAY_NODES}-node bay, the "
                         f"two end anchors' runs and every support family); got {nodes}")
    exact = spec["route"] == "EX"
    rack = Rack(nodes)
    # On EX the nonlinear supports wait for RT-A3 increment 4: their linear interim forms are emitted.
    nonlinear = spec["nonlinear"] and not exact
    cases, combinations = load_cases(rack, profile)
    pipes = [{"id": p["id"], "from": p["from"], "to": p["to"], "material": "material:RK-M1",
              "section_ref": f"section:RK-{p['section']}", "section": section_record(p["section"], exact),
              "y_reference": dict(zip("xyz", p["y_ref"])), "provenance": PROVENANCE} for p in rack.pipes]
    document = {
        "schema_version": MODEL_VERSION,
        "document_kind": "openpipestress.product_preview.model",
        "project": {"id": f"project:RK-{profile}-{nodes}", "name": f"RT-U4 reference rack {profile} {nodes} (invented)",
                    "description": "Invented pipe rack for the DEL-04-05 performance harness (release target "
                                   "criterion 9); a test object, not a design",
                    "units": {"length": "m", "force": "N", "moment": "N*m", "angle": "rad", "pressure": "Pa",
                              "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "professional_acceptance": "not_provided",
                            "rule_check": "not_performed_user_rule_inputs_missing"},
        "materials": [material(exact)],
        "sections": [{"id": f"section:RK-{tag}", "name": f"RK-{tag}", "section_type": "pipe",
                      "properties": {"outside_diameter": q(od, "mm"), "wall_thickness": q(wall, "mm")},
                      "provenance": PROVENANCE} for tag, (od, wall, *_rest) in SECTIONS.items()],
        "nodes": [{**node, "provenance": PROVENANCE} for node in rack.nodes],
        "pipe_segments": pipes,
        "supports": supports(rack, nonlinear),
        "components": components(rack),
        "load_cases": cases,
        "combinations": combinations,
    }
    if exact:
        document["pressure_contract"] = dict(EX_CONTRACT)
    return document, stats(document, profile, nodes)


def stats(document: dict, profile: str, nodes: int) -> dict:
    supports_by = Counter(s["nonlinear"]["behavior"] if s["family"] == "nonlinear" else s["family"]
                          for s in document["supports"])
    components_by = Counter(c["kind"] for c in document["components"])
    region_counts = sorted({len(c.get("pressure_regions") or []) for c in document["load_cases"]})
    bays, tail = bays_and_tail(nodes)
    return {
        "profile": profile, "route": PROFILES[profile]["route"], "nodes": len(document["nodes"]),
        "members": len(document["pipe_segments"]), "bays": bays, "tail_nodes": tail,
        "bends": components_by.get("bend", 0), "components": dict(sorted(components_by.items())),
        "supports": dict(sorted(supports_by.items())), "cases": [c["id"] for c in document["load_cases"]],
        "combinations": [c["id"] for c in document["combinations"]],
        "regions_per_case": region_counts[0] if len(region_counts) == 1 else region_counts,
        "primitive_loads": sum(len(c["primitive_loads"]) for c in document["load_cases"]),
        "timed": PROFILES[profile]["timed"],
        "pending_families_omitted": [entry["family"] for entry in ledger_for(profile, nodes)],
        "model_version": document["schema_version"],
    }


def canonical(document: dict) -> bytes:
    return json.dumps(document, sort_keys=True, separators=(",", ":"), ensure_ascii=False,
                      allow_nan=False).encode("utf-8")


def hashes_text() -> str:
    """The text of observations/rack/models_sha256.txt: the sha256 of each profile's canonical bytes at each size of
    the scaling series."""
    lines = ["# sha256 of the RT-U4 reference racks' canonical bytes (rack/rack_model.py), model document "
             f"{MODEL_VERSION}", "# regenerate: python3 core/solver/performance_harness/rack/rack_model.py --hashes"]
    for profile in PROFILES:
        for nodes in SCALING_SIZES:
            digest = hashlib.sha256(canonical(build(profile, nodes)[0])).hexdigest()
            lines.append(f"{digest}  {profile}  {nodes}")
    return "\n".join(lines) + "\n"


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--profile", choices=sorted(PROFILES))
    parser.add_argument("--nodes", type=int)
    parser.add_argument("-o", "--output", type=Path)
    parser.add_argument("--list-profiles", action="store_true", help="print the profiles as JSON")
    parser.add_argument("--ledger", action="store_true", help="print the pending-family ledger as JSON")
    parser.add_argument("--hashes", action="store_true",
                        help="print observations/rack/models_sha256.txt (every profile at every scaling size)")
    args = parser.parse_args(argv)
    if args.list_profiles:
        print(json.dumps(PROFILES, indent=2, sort_keys=True))
        return 0
    if args.ledger:
        print(json.dumps({"basis": LEDGER_BASIS, "entries": LEDGER}, indent=2, sort_keys=True))
        return 0
    if args.hashes:
        sys.stdout.write(hashes_text())
        return 0
    if args.profile is None or args.nodes is None:
        parser.error("--profile and --nodes are required (or --list-profiles / --ledger)")
    try:
        document, summary = build(args.profile, args.nodes)
    except ValueError as error:
        print(f"rack_model: {error}", file=sys.stderr)
        return 2
    data = canonical(document)
    if args.output:
        args.output.write_bytes(data)
    else:
        sys.stdout.buffer.write(data)
    print(json.dumps(summary, sort_keys=True), file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main())
