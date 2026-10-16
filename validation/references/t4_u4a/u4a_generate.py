"""T4-U4a frozen references: invented arc models, their requests and the oracle.

Standard-library Python 3 only; imports nothing from the product. The oracle
is computed by `u4a_engine.py` (unit-load method, 90-digit decimal). Run from
this directory:

    python3 -I u4a_generate.py            # verify requests.json and oracle.json
    python3 -I u4a_generate.py --write    # intentionally replace them

The check also recomputes every valued T4-I7 case (validation/references/
t4_i7/u2_reference_cases.json, read as data) and requires agreement of its
stations, tangent-frame end rows, chord-frame elastic end rows and support A
reactions to 1e-18 relative, so the frames, signs and pressure statics used
here are confirmed against an independently derived and reviewed reference.
"""

from __future__ import annotations

import hashlib
import json
import sys
from decimal import Decimal
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))  # the engine beside this file (python3 -I omits the script directory)
import u4a_engine as E  # noqa: E402

D = Decimal
I7_PATH = HERE.parent / "t4_i7" / "u2_reference_cases.json"
PROVENANCE = "invented T4-U4a arc-frame reference input; not library, component or code-rule data"
SCHEMA = "independent.arc_frames_and_signs/1.0.0"
SIG = 22  # significant digits written

# ------------------------------------------------------------------ models

# A proper rotation with rational entries (det +1): -(1/9)[[1,4,8],[4,7,-4],[8,-4,1]].
Q = [[D(-1) / 9, D(-4) / 9, D(-8) / 9], [D(-4) / 9, D(-7) / 9, D(4) / 9], [D(-8) / 9, D(4) / 9, D(-1) / 9]]


def rot(v):
    return [sum(Q[r][c] * D(str(v[c])) for c in range(3)) for r in range(3)]


def b64(x) -> float:
    return float(x)


PLANAR = {
    # Left turn: +X, a 90-degree arc (R = 0.375) to +Y.
    "L-LEFT": {
        "nodes": {"node:A": [0, 0, 0], "node:B": [2.5, 0, 0], "node:C": [2.875, 0.375, 0], "node:D": [2.875, 3.0, 0]},
        "order": ["node:A", "node:B", "node:C", "node:D"],
        "members": [("pipe:S1", "straight", "node:A", "node:B", [0, 0, 1]),
                    ("pipe:BEND", "arc", "node:B", "node:C", [1, -1, 0]),
                    ("pipe:S2", "straight", "node:C", "node:D", [0, 0, 1])],
    },
    # Right turn (mirror image): +X, the arc to -Y.
    "L-RIGHT": {
        "nodes": {"node:A": [0, 0, 0], "node:B": [2.5, 0, 0], "node:C": [2.875, -0.375, 0], "node:D": [2.875, -3.0, 0]},
        "order": ["node:A", "node:B", "node:C", "node:D"],
        "members": [("pipe:S1", "straight", "node:A", "node:B", [0, 0, 1]),
                    ("pipe:BEND", "arc", "node:B", "node:C", [1, 1, 0]),
                    ("pipe:S2", "straight", "node:C", "node:D", [0, 0, 1])],
    },
    # L-LEFT with the arc and the second straight authored against the traversal.
    "L-LEFT-REV": {
        "nodes": {"node:A": [0, 0, 0], "node:B": [2.5, 0, 0], "node:C": [2.875, 0.375, 0], "node:D": [2.875, 3.0, 0]},
        "order": ["node:A", "node:B", "node:C", "node:D"],
        "members": [("pipe:S1", "straight", "node:A", "node:B", [0, 0, 1]),
                    ("pipe:BEND", "arc", "node:C", "node:B", [1, -1, 0]),
                    ("pipe:S2", "straight", "node:D", "node:C", [0, 0, 1])],
    },
    # U: two left turns (R = 0.375).
    "U": {
        "nodes": {"node:A": [0, 0, 0], "node:B1": [2.0, 0, 0], "node:C1": [2.375, 0.375, 0], "node:B2": [2.375, 1.625, 0],
                  "node:C2": [2.0, 2.0, 0], "node:D": [0, 2.0, 0]},
        "order": ["node:A", "node:B1", "node:C1", "node:B2", "node:C2", "node:D"],
        "members": [("pipe:S1", "straight", "node:A", "node:B1", [0, 0, 1]),
                    ("pipe:BEND1", "arc", "node:B1", "node:C1", [1, -1, 0]),
                    ("pipe:S2", "straight", "node:C1", "node:B2", [0, 0, 1]),
                    ("pipe:BEND2", "arc", "node:B2", "node:C2", [1, 1, 0]),
                    ("pipe:S3", "straight", "node:C2", "node:D", [0, 0, 1])],
    },
    # Z: a left turn then a right turn.
    "Z": {
        "nodes": {"node:A": [0, 0, 0], "node:B1": [2.0, 0, 0], "node:C1": [2.375, 0.375, 0], "node:B2": [2.375, 1.625, 0],
                  "node:C2": [2.75, 2.0, 0], "node:D": [4.5, 2.0, 0]},
        "order": ["node:A", "node:B1", "node:C1", "node:B2", "node:C2", "node:D"],
        "members": [("pipe:S1", "straight", "node:A", "node:B1", [0, 0, 1]),
                    ("pipe:BEND1", "arc", "node:B1", "node:C1", [1, -1, 0]),
                    ("pipe:S2", "straight", "node:C1", "node:B2", [0, 0, 1]),
                    ("pipe:BEND2", "arc", "node:B2", "node:C2", [-1, 1, 0]),
                    ("pipe:S3", "straight", "node:C2", "node:D", [0, 0, 1])],
    },
    # Non-planar: a left turn in XY, then a turn up out of that plane.
    "3D": {
        "nodes": {"node:A": [0, 0, 0], "node:B1": [2.0, 0, 0], "node:C1": [2.375, 0.375, 0], "node:B2": [2.375, 1.625, 0],
                  "node:C2": [2.375, 2.0, 0.375], "node:D": [2.375, 2.0, 2.0]},
        "order": ["node:A", "node:B1", "node:C1", "node:B2", "node:C2", "node:D"],
        "members": [("pipe:S1", "straight", "node:A", "node:B1", [0, 0, 1]),
                    ("pipe:BEND1", "arc", "node:B1", "node:C1", [1, -1, 0]),
                    ("pipe:S2", "straight", "node:C1", "node:B2", [0, 0, 1]),
                    ("pipe:BEND2", "arc", "node:B2", "node:C2", [0, 1, -1]),
                    ("pipe:S3", "straight", "node:C2", "node:D", [1, 0, 0])],
    },
    # T0R's preview-physics arc model (product test `arc_model`), reproduced as authored.
    "T0R-B1": {
        "nodes": {"node:a": [0, 0, 0], "node:b": [1.0, 0, 0], "node:c": [1.2, 0.2, 0], "node:d": [1.2, 1.2, 0]},
        "order": ["node:a", "node:b", "node:c", "node:d"],
        "members": [("pipe:a-b", "straight", "node:a", "node:b", [0.0, 1.0, 0.0]),
                    ("pipe:b-c", "arc", "node:b", "node:c", [0.7071067811865476, -0.7071067811865476, 0.0]),
                    ("pipe:c-d", "straight", "node:c", "node:d", [1.0, 0.0, 0.0])],
    },
}


def geometry(name, skew=False, offset=(0, 0, 0)):
    g = PLANAR[name]
    nodes = {}
    for n, xyz in g["nodes"].items():
        p = rot(xyz) if skew else [D(str(v)) for v in xyz]
        nodes[n] = [b64(p[k] + D(str(offset[k]))) for k in range(3)]
    members = []
    for pid, kind, a, b, yref in g["members"]:
        y = rot(yref) if skew else [D(repr(float(v))) for v in yref]
        members.append({"pipe": pid, "kind": kind, "authored_from": a, "authored_to": b, "y_reference": [b64(v) for v in y]})
    return nodes, g["order"], members


def load_vector(v, skew):
    return [b64(x) for x in (rot(v) if skew else [D(str(c)) for c in v])]


PREVIEW_SECTION = {"outside_diameter": "0.1683", "wall_thickness": "0.00711"}
PREVIEW_MATERIAL = {"E": "2.0e11", "G": "7.9e10", "alpha": "1.2e-5"}
PRESSURE_SECTION = {"outside_diameter": "0.1143", "wall_thickness": "0.00602", "mill_tolerance": "0.0007"}
PRESSURE_MATERIAL = {"E": "1.95e11", "nu": "0.29", "alpha": "1.17e-5"}
T0R_SECTION = {"outside_diameter": "0.12", "wall_thickness": "0.01"}
T0R_MATERIAL = {"E": "2.0e11", "G": "7.6923076923e10", "alpha": "1.2e-5"}

TIP_FORCE = [1200, -800, 1500]
TIP_MOMENT = [300, -450, 600]
WEIGHT = [0, 0, -350]
ARC_SIDE_LOAD = [120, 60, 0]
INTERIOR_FORCE = [0, 0, -2000]


def case(cid, route, description, name, *, skew=False, offset=(0, 0, 0), k="2", radius="0.375", tip=False,
         anchor_last=False, delta_t="0", p="0", terminal_a="transfers_to_wall", terminal_d="transfers_to_wall",
         section=None, material=None, interior=None, arc_side=True, weight=True, t0r_force=None, tip_force=None, gap_last=None):
    nodes, order, members = geometry(name, skew, offset)
    for m in members:
        if m["kind"] == "arc":
            m["R"], m["k"] = radius, k
    uniform = {}
    for m in members:
        loads = []
        if weight:
            loads.append(load_vector(WEIGHT, skew))
        if arc_side and m["kind"] == "arc":
            loads.append(load_vector(ARC_SIDE_LOAD, skew))
        uniform[m["pipe"]] = loads
    nodal = {}
    if tip:
        nodal[order[-1]] = [load_vector(tip_force or TIP_FORCE, skew), load_vector(TIP_MOMENT, skew)]
    if interior:
        nodal[interior] = [load_vector(INTERIOR_FORCE, skew), [0.0, 0.0, 0.0]]
    if t0r_force is not None:
        nodal[order[-1]] = [[0.0, 0.0, t0r_force], [0.0, 0.0, 0.0]]
    return {
        "case_id": cid, "route": route, "description": description, "family": name, "skew": skew,
        "nodes": nodes, "traversal_order": order, "members": members,
        "section": section or (PRESSURE_SECTION if route == "pressure-1" else PREVIEW_SECTION),
        "material": material or (PRESSURE_MATERIAL if route == "pressure-1" else PREVIEW_MATERIAL),
        "loads": {"p": p, "terminal_A": terminal_a, "terminal_D": terminal_d, "delta_T_all": delta_t,
                  "uniform": {k2: [[str(D(x)) for x in w] for w in v] for k2, v in uniform.items()},
                  "nodal": {n: [[str(D(x)) for x in f], [str(D(x)) for x in mm]] for n, (f, mm) in nodal.items()}},
        "anchor_last": anchor_last,
        # A one-way vertical contact at the last node, closed at its gap (the product's nonlinear `gap` device).
        "gap_last": gap_last,
        "last_restraint": {"dofs": [2], "prescribed": [str(-D(gap_last))]} if gap_last else None,
    }


PP1 = "preview-physics-1"
PR1 = "pressure-1"
BIG = (5.0e6, 7.3e6, 1.2e6)
SKEW_OFFSET = (1.5, -2.25, 0.75)

CASES = [
    case("U4A-PP1-L-LEFT-TIP", PP1, "planar left-turn L, anchored at A, free D: tip force and moment, weight, in-plane arc side load", "L-LEFT", tip=True),
    case("U4A-PP1-L-RIGHT-TIP", PP1, "mirror image of L-LEFT (right turn), same loads", "L-RIGHT", tip=True),
    case("U4A-PP1-L-LEFT-REV-TIP", PP1, "L-LEFT with the arc and S2 authored against the traversal", "L-LEFT-REV", tip=True),
    case("U4A-PP1-L-SKEW-TIP", PP1, "L-LEFT rigidly rotated (with its loads) and offset", "L-LEFT", skew=True, offset=SKEW_OFFSET, tip=True),
    case("U4A-PP1-L-SKEW-X5E6-TIP", PP1, "L-SKEW at large coordinates (5e6, 7.3e6, 1.2e6) m", "L-LEFT", skew=True, offset=BIG, tip=True),
    case("U4A-PP1-U-ANCH-TH", PP1, "planar U (two left turns), anchored at both ends: 80 degC, weight, interior force at C1", "U", anchor_last=True, delta_t="80", interior="node:C1"),
    case("U4A-PP1-U-SKEW-ANCH-TH", PP1, "U-ANCH rigidly rotated and offset", "U", skew=True, offset=SKEW_OFFSET, anchor_last=True, delta_t="80", interior="node:C1"),
    case("U4A-PP1-Z-ANCH-TH", PP1, "planar Z (left then right turn), anchored at both ends: 80 degC, weight, interior force at C1", "Z", anchor_last=True, delta_t="80", interior="node:C1"),
    case("U4A-PP1-3D-ANCH-TH", PP1, "non-planar line (second bend turns out of the first bend's plane), anchored at both ends", "3D", anchor_last=True, delta_t="80", interior="node:C1", k="3"),
    case("U4A-PP1-L-LEFT-GAP", PP1, "planar left-turn L with a one-way vertical contact at D that closes at its 0.01 m gap (nonlinear route; binary64 recovery)",
         "L-LEFT", tip=True, tip_force=[1200, -800, -1500], gap_last="0.01"),
    case("U4A-PP1-T0R-B1-K2", PP1, "T0R arc_model(k=2): the product preview test model, 1000 N along global z at node d", "T0R-B1", k="2", radius="0.2",
         section=T0R_SECTION, material=T0R_MATERIAL, arc_side=False, weight=False, t0r_force=1000.0),
    case("U4A-PP1-T0R-B1-K4", PP1, "T0R arc_model(k=4)", "T0R-B1", k="4", radius="0.2",
         section=T0R_SECTION, material=T0R_MATERIAL, arc_side=False, weight=False, t0r_force=1000.0),
    case("U4A-PR1-L-LEFT-TIP-P", PR1, "planar left-turn L under 4 MPa, closed transferring terminals, free D with tip loads and weight", "L-LEFT", tip=True, p="4.0e6"),
    case("U4A-PR1-L-RIGHT-TIP-P", PR1, "mirror image of PR1 L-LEFT", "L-RIGHT", tip=True, p="4.0e6"),
    case("U4A-PR1-L-LEFT-REV-TIP-P", PR1, "PR1 L-LEFT with the arc and S2 authored against the traversal", "L-LEFT-REV", tip=True, p="4.0e6"),
    case("U4A-PR1-L-SKEW-TIP-P", PR1, "PR1 L-LEFT rigidly rotated and offset", "L-LEFT", skew=True, offset=SKEW_OFFSET, tip=True, p="4.0e6"),
    case("U4A-PR1-U-ANCH-PT", PR1, "planar U under 4 MPa and 60 degC, anchored at both ends, weight, interior force; D closure separately supported",
         "U", anchor_last=True, delta_t="60", p="4.0e6", terminal_d="separately_supported_or_compensated", interior="node:C1"),
    case("U4A-PR1-U-SKEW-ANCH-PT", PR1, "PR1 U-ANCH rigidly rotated and offset", "U", skew=True, offset=SKEW_OFFSET, anchor_last=True, delta_t="60",
         p="4.0e6", terminal_d="separately_supported_or_compensated", interior="node:C1"),
    case("U4A-PR1-Z-ANCH-NEGP", PR1, "planar Z, anchored at both ends, signed pressure -0.6 MPa, 60 degC, weight", "Z", anchor_last=True, delta_t="60", p="-0.6e6"),
    case("U4A-PR1-3D-ANCH-PT", PR1, "non-planar line under 4 MPa and 60 degC, anchored at both ends", "3D", anchor_last=True, delta_t="60", p="4.0e6", k="3"),
]

# ---------------------------------------------------------------- requests

UNITS = {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}
STATUS = {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"}
AXES = ["global_x", "global_y", "global_z"]
ROT_AXES = ["rotation_x", "rotation_y", "rotation_z"]


def q(value, unit):
    return {"value": value, "unit": unit}


def primitive_loads(c):
    out = []
    for pid, loads in c["loads"]["uniform"].items():
        for n, w in enumerate(loads):
            for axis, value in enumerate(w):
                if float(value) != 0.0:
                    out.append({"id": f"load:line:{pid}:{n}:{AXES[axis]}", "category": "distributed_force",
                                "target": {"type": "element", "pipe": pid}, "direction": AXES[axis],
                                "magnitude": q(float(value), "N/m"), "dimension": "force_per_length", "provenance": PROVENANCE})
    for node, (f, mm) in c["loads"]["nodal"].items():
        for axis, value in enumerate(f):
            if float(value) != 0.0:
                out.append({"id": f"load:force:{node}:{AXES[axis]}", "category": "concentrated_force",
                            "target": {"type": "node", "node": node}, "direction": AXES[axis],
                            "magnitude": q(float(value), "N"), "dimension": "force", "provenance": PROVENANCE})
        for axis, value in enumerate(mm):
            if float(value) != 0.0:
                out.append({"id": f"load:moment:{node}:{ROT_AXES[axis]}", "category": "concentrated_moment",
                            "target": {"type": "node", "node": node}, "direction": ROT_AXES[axis],
                            "magnitude": q(float(value), "N*m"), "dimension": "moment", "provenance": PROVENANCE})
    return out


def bend_components(c, line):
    out = []
    for n, m in enumerate(line.members):
        if m.kind != "arc":
            continue
        spec = next(s for s in c["members"] if s["pipe"] == m.id)
        out.append({"id": f"component:bend-{n}", "kind": "bend", "node": spec["authored_from"],
                    "geometry": {"bend_pipe_ref": m.id, "bend_radius": q(float(D(spec["R"])), "m"),
                                 "bend_angle": q(float(m.phi), "rad"),
                                 "bend_plane_orientation": "plane of the span chord and the pipe y_reference; the arc bows toward +y_reference"},
                    "modifiers": {"flexibility_factor_user_value": q(float(D(spec["k"])), "none"), "source_reference": "invented test value"},
                    "mechanics_interface": {"solver_consumption": "curved_bend_macro_element"},
                    "provenance": PROVENANCE + "; user flexibility factor k is an invented test value"})
    return out


def section_json(sec):
    out = {"outside_diameter": q(float(D(sec["outside_diameter"])), "m"), "wall_thickness": q(float(D(sec["wall_thickness"])), "m")}
    if "mill_tolerance" in sec:
        out["mill_tolerance"] = q(float(D(sec["mill_tolerance"])), "m")
    return out


def request_document(c, line):
    nodes = [{"id": n, "position": dict(zip("xyz", xyz)), "provenance": PROVENANCE} for n, xyz in c["nodes"].items()]
    pipes = [{"id": m["pipe"], "from": m["authored_from"], "to": m["authored_to"], "section": section_json(c["section"]),
              "material": "material:steel", "y_reference": dict(zip("xyz", m["y_reference"])), "provenance": PROVENANCE}
             for m in c["members"]]
    supports = [{"id": "support:A", "node": c["traversal_order"][0], "family": "anchor",
                 "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROVENANCE}]
    if c["anchor_last"]:
        supports.append({"id": "support:D", "node": c["traversal_order"][-1], "family": "anchor",
                         "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROVENANCE})
    if c["gap_last"]:
        supports.append({"id": "support:D", "node": c["traversal_order"][-1], "family": "nonlinear", "restraints": [], "provenance": PROVENANCE,
                         "nonlinear": {"behavior": "gap", "dof": "UZ", "initial_state": "inactive", "closes_when": "negative_displacement",
                                       "gap": q(float(D(c["gap_last"])), "m")}})
    loads = primitive_loads(c)
    project = {"id": "project:" + c["case_id"].lower(), "units": UNITS}
    if c["route"] == PP1:
        mat = c["material"]
        if D(c["loads"]["delta_T_all"]) != 0:
            for m in c["members"]:
                loads.append({"id": f"load:thermal:{m['pipe']}", "category": "thermal", "target": {"type": "element", "pipe": m["pipe"]},
                              "direction": "global_x", "magnitude": q(float(D(c["loads"]["delta_T_all"])), "degC"),
                              "dimension": "temperature_interval", "provenance": PROVENANCE})
        model = {"schema_version": "0.2.0", "document_kind": "openpipestress.product_preview.model", "project": project,
                 "analysis_status": STATUS, "nodes": nodes, "pipe_segments": pipes, "supports": supports,
                 "components": bend_components(c, line),
                 "materials": [{"id": "material:steel", "elastic_modulus": q(float(D(mat["E"])), "Pa"),
                                "shear_modulus": q(float(D(mat["G"])), "Pa"),
                                "thermal_expansion_coefficient": q(float(D(mat["alpha"])), "1/degC"), "provenance": PROVENANCE}],
                 "load_cases": [{"id": "case:u4a", "primitive_loads": loads, "provenance": PROVENANCE}], "combinations": []}
        return {"model": model, "materials": []}
    mat = c["material"]
    states = []
    for m in c["members"]:
        state = {"pipe_ref": m["pipe"], "material_selection": {"kind": "explicit_base_properties", "material_ref": "material:steel",
                                                                "applicability_reference": "invented analytical basis declared applicable"}}
        if D(c["loads"]["delta_T_all"]) != 0:
            state["thermal_state"] = {"kind": "constant_alpha_interval", "coefficient": q(float(D(mat["alpha"])), "1/degC"),
                                      "temperature_change": q(float(D(c["loads"]["delta_T_all"])), "degC"),
                                      "coefficient_meaning": "engineering_interval", "provenance": PROVENANCE}
        else:
            state["thermal_state"] = {"kind": "unchanged_reference", "provenance": PROVENANCE}
        states.append(state)
    model = {
        "schema_version": "0.4.0", "document_kind": "openpipestress.product_preview.model",
        "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3"},
        "project": project, "analysis_status": STATUS, "nodes": nodes, "pipe_segments": pipes, "supports": supports,
        "components": bend_components(c, line),
        "materials": [{"id": "material:steel", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
                       "elastic_modulus": q(float(D(mat["E"])), "Pa"), "poisson_ratio": q(float(D(mat["nu"])), "1"), "provenance": PROVENANCE}],
        "reference_configurations": [{"id": "reference:installed", "label": "Installed reference", "geometry_ref": {"kind": "authored_model_geometry"},
                                      "member_references": [{"pipe_ref": m["pipe"], "basis": {"kind": "direct_strain_reference"}, "fit": {"kind": "none"},
                                                             "provenance": PROVENANCE} for m in c["members"]],
                                      "provenance": PROVENANCE}],
        "load_cases": [{
            "id": "case:u4a",
            "pressure_regions": [{"id": "region:u4a", "member_pipe_ids": [m["pipe"] for m in c["members"]],
                                  "pressure_basis": "internal_differential_zero_external_v1", "pressure": q(float(D(c["loads"]["p"])), "Pa"),
                                  "terminals": [{"node_ref": c["traversal_order"][0], "closure_transfer": c["loads"]["terminal_A"], "provenance": PROVENANCE},
                                                {"node_ref": c["traversal_order"][-1], "closure_transfer": c["loads"]["terminal_D"], "provenance": PROVENANCE}],
                                  "provenance": PROVENANCE}],
            "primitive_loads": loads,
            "analysis_state": {"contract": "openpipestress.load_reference_state/1.0.0", "reference_configuration_ref": "reference:installed",
                               "element_states": states,
                               "support_states": [{"support_ref": s["id"], "participation": {"kind": "active_model_device"}} for s in supports],
                               "load_sources": [{"source_ref": load["id"], "factor": 1.0} for load in loads],
                               "history": {"kind": "independent_equilibrium"}, "provenance": PROVENANCE},
            "provenance": PROVENANCE}],
        "combinations": [],
    }
    return {"model": model, "materials": []}


# ------------------------------------------------------------------ oracle

SNAP = D("1e-60")  # far below any physical value here (forces ~1e3 N, rotations ~1e-5 rad); 90-digit noise only


def fmt(x: Decimal) -> str:
    if abs(x) < SNAP:
        return "0"
    return f"{x:.{SIG - 1}e}"


def vec(v):
    return [fmt(c) for c in v]


KEYS = ["F_x", "F_y", "F_z", "M_x", "M_y", "M_z"]
CUT_KEYS = ["N", "V_y", "V_z", "T", "M_y", "M_z"]


def engine_model(c):
    members = [dict(m, y_reference=m["y_reference"]) for m in c["members"]]
    loads = dict(c["loads"])
    model = {"nodes": c["nodes"], "traversal_order": c["traversal_order"], "members": members, "section": c["section"],
             "material": c["material"], "loads": loads, "anchor_last": c["anchor_last"], "last_restraint": c["last_restraint"]}
    if c["route"] == PR1:
        model["section_bases"] = "d5"  # D-5 (T4-U6): the exact route's section bases; preview-physics-1 keeps one section
    return model


def rotation(rows_axes, chord_axes):
    return [[E.dot(a, b) for b in chord_axes] for a in rows_axes]


def mutant_frames(m, end):
    """Wrong frames for the discriminators (rows: axes as global vectors)."""
    dh, nh, ez = m.chord_axes
    s, c = m.half_sin, m.half_cos
    sign = D(1) if end == "end_i" else D(-1)
    sp, cp = m.sin_phi, m.cos_phi
    return {
        # Rotation by +-phi/2 about the chord z only (no pi flip about x): y away from the centre, z = chord z.
        "inplane_rotation_without_flip": (E.add(E.scale(dh, c), E.scale(nh, sign * s)), E.add(E.scale(dh, -sign * s), E.scale(nh, c)), ez),
        # The full bend angle phi instead of phi/2.
        "full_angle_instead_of_half": (E.add(E.scale(dh, cp), E.scale(nh, sign * sp)), E.add(E.scale(dh, sign * sp), E.scale(nh, -cp)), E.scale(ez, D(-1))),
    }


def oracle_case(c):
    line = E.Line(engine_model(c)).solve()
    pressure = c["route"] == PR1
    if line.bases is None:
        derived = {"A_s": fmt(line.As), "A_i": fmt(line.Ai), "I": fmt(line.I), "J": fmt(line.J), "Z": fmt(line.Z), "r_o": fmt(line.ro),
                   "E": fmt(line.E), "G": fmt(line.G), "p_Ai": fmt(line.p_ai),
                   "eps0": {m.id: fmt(line.eps0[m.id]) for m in line.members}}
    else:
        derived = {"stiffness_section": "nominal_section",
                   "section_bases": {b: {"wall": fmt(s["wall"]), "r_i": fmt(s["ri"]), "A_s": fmt(s["As"]), "I": fmt(s["I"]),
                                         "J": fmt(s["J"]), "Z": fmt(s["Z"])} for b, s in line.bases.items()},
                   "pressure_bore": {"r_i": fmt(line.ri_bore), "A_i": fmt(line.Ai)}, "r_o": fmt(line.ro),
                   "E": fmt(line.E), "G": fmt(line.G), "p_Ai": fmt(line.p_ai),
                   "eps0": {m.id: fmt(line.eps0[m.id]) for m in line.members}}
    out = {"route": c["route"], "description": c["description"], "family": c["family"], "skew": c["skew"],
           "derived": derived, "members": {}}
    if line.last_dofs:
        out["redundant_support_D_on_pipe"] = {"force": vec(line.redundant[0]), "moment_about_D": vec(line.redundant[1])}
    fa, ma = line.support_a()
    out["support_A_on_pipe"] = {"force": vec(fa), "moment_about_A": vec(ma)}
    checks = {"rotation_formula_max_abs_error": D(0), "end_cut_identity_max_abs_error": D(0)}
    raw = {}
    for k, m in enumerate(line.members):
        rec = {"kind": m.kind, "authored_from": m.authored_from, "authored_to": m.authored_to, "traversal_matches_authored": m.forward}
        cuts, cut_rows = {}, {}
        for loc, u in E.FRACTIONS:
            F, M = line.cut(k, u)
            rows = E.components(F, M, m.axes(u))
            entry = dict(zip(CUT_KEYS, rows))
            if pressure:
                entry = {"N_w": rows[0], "S": rows[0] - line.p_ai, **dict(zip(CUT_KEYS[1:], rows[1:]))}
            cut_rows[loc] = (F, M, entry)
        # elastic end actions (node-on-element), global
        ends = {}
        for loc, u, sign in [("end_i", D(0), D(-1)), ("end_j", D(1), D(1))]:
            F, M, _ = cut_rows[loc]
            t = m.axes(u)[0]
            Fe = E.scale(E.sub(F, E.scale(t, line.p_ai)), sign)  # S along the tangent: subtract p Ai t from the wall cut
            Me = E.scale(M, sign)
            ends[loc] = (Fe, Me, u)
        rec["frames"] = {"chord": {"x": vec(m.chord_axes[0]), "y": vec(m.chord_axes[1]), "z": vec(m.chord_axes[2])}}
        if m.kind == "arc":
            rec["arc"] = {"R": fmt(m.radius), "included_angle": fmt(m.phi), "half_angle_sin_s": fmt(m.half_sin), "half_angle_cos_c": fmt(m.half_cos),
                          "centre": vec(m.centre), "bend_plane_normal": vec(m.normal), "k": fmt(m.k)}
            rec["frames"]["tangent_end_i"] = dict(zip("xyz", (vec(a) for a in m.axes(D(0)))))
            rec["frames"]["tangent_end_j"] = dict(zip("xyz", (vec(a) for a in m.axes(D(1)))))
            # For the enclosure control of the binary64-tangent mutant: 45 significant digits, and whether
            # binary64 rounding of the end tangent axes is inexact (then a radius-zero lift of a binary64
            # tangent cannot enclose the exact axis).
            rec["tangent_axes_45_digits"] = {loc: {name: [f"{v:.44e}" if abs(v) >= SNAP else "0" for v in axis]
                                                   for name, axis in zip("xyz", m.axes(u))} for loc, u in (("end_i", D(0)), ("end_j", D(1)))}
            rec["binary64_tangent_axes_inexact"] = any(abs(D(float(v)) - v) > D("1e-60") for u in (D(0), D(1)) for axis in m.axes(u) for v in axis)
            s, cc = m.half_sin, m.half_cos
            formula = {"end_i": [[cc, s, D(0)], [s, -cc, D(0)], [D(0), D(0), D(-1)]],
                       "end_j": [[cc, -s, D(0)], [-s, -cc, D(0)], [D(0), D(0), D(-1)]]}
            rec["chord_to_tangent_rotation"] = {}
            for loc, u in [("end_i", D(0)), ("end_j", D(1))]:
                r = rotation(m.axes(u), m.chord_axes)
                for a in range(3):
                    for b in range(3):
                        checks["rotation_formula_max_abs_error"] = max(checks["rotation_formula_max_abs_error"], abs(r[a][b] - formula[loc][a][b]))
                rec["chord_to_tangent_rotation"][loc] = [[fmt(v) for v in row] for row in r]
        rec["cuts_tangent_frame_j_side"] = {loc: {key: fmt(v) for key, v in cut_rows[loc][2].items()} for loc, _ in E.FRACTIONS}
        end_rows = {}
        for loc, (Fe, Me, u) in ends.items():
            tangent = E.components(Fe, Me, m.axes(u))
            entry = {"tangent_frame": dict(zip(KEYS, (fmt(v) for v in tangent)))}
            if m.kind == "arc":
                entry["chord_frame"] = dict(zip(KEYS, (fmt(v) for v in E.components(Fe, Me, m.chord_axes))))
            end_rows[loc] = entry
            # identity: end_i = -cut(0), end_j = +cut(1) (elastic), in the frame at that end
            cut = cut_rows[loc][2]
            elastic_cut = [cut["S"] if pressure else cut["N"]] + [cut[key] for key in CUT_KEYS[1:]]
            sign = D(-1) if loc == "end_i" else D(1)
            for a in range(6):
                checks["end_cut_identity_max_abs_error"] = max(checks["end_cut_identity_max_abs_error"], abs(tangent[a] - sign * elastic_cut[a]))
        rec["end_rows_node_on_element_elastic"] = end_rows
        if pressure:
            rec["wall_end_action_along_end_tangent"] = {"end_i": fmt(-cut_rows["end_i"][2]["N_w"]), "end_j": fmt(cut_rows["end_j"][2]["N_w"])}
        if line.bases is None:
            stresses = {}
            for loc, _ in E.FRACTIONS:
                cut = cut_rows[loc][2]
                axial = cut["N_w"] if pressure else cut["N"]
                stresses[loc] = {("sigma_m_wall_membrane" if pressure else "sigma_axial"): fmt(axial / line.As),
                                 "sigma_b_y": fmt(cut["M_y"] / line.Z), "sigma_b_z": fmt(cut["M_z"] / line.Z), "tau_t": fmt(cut["T"] * line.ro / line.J)}
            rec["nominal_stresses_SI"] = stresses
        else:  # D-5: the one wall force and section actions on each basis (DEL-05-03 Design section 2.1)
            by_basis = {}
            for b, s in line.bases.items():
                by_basis[b] = {}
                for loc, _ in E.FRACTIONS:
                    cut = cut_rows[loc][2]
                    by_basis[b][loc] = {"sigma_m_wall_membrane": fmt(cut["N_w"] / s["As"]), "sigma_b_y": fmt(cut["M_y"] / s["Z"]),
                                        "sigma_b_z": fmt(cut["M_z"] / s["Z"]), "tau_t": fmt(cut["T"] * line.ro / s["J"])}
            rec["stresses_SI_by_section_basis"] = by_basis
        raw[m.id] = (m, cut_rows, ends)
        out["members"][m.id] = rec
    # node displacements (unchanged by U4a; for validating the request against the product)
    out["node_displacements"] = {}
    for index, node in enumerate(line.order):
        if index == 0 or (line.anchor_d and index == len(line.order) - 1):
            values = [D(0)] * 6
        else:
            values = line.virtual_work(index)
            if index == len(line.order) - 1:
                for dof, value in zip(line.last_dofs, line.last_prescribed):
                    assert abs(values[dof] - value) < D("1e-70"), (dof, values[dof], value)
        out["node_displacements"][node] = dict(zip(["ux", "uy", "uz", "rx", "ry", "rz"], (fmt(v) for v in values)))
    # member-relative zero scales and tolerances
    case_force = max([abs(v) for _, (m, cr, en) in raw.items() for loc in cr for key, v in cr[loc][2].items() if key in ("N", "N_w", "S", "V_y", "V_z")] + [D(0)])
    case_moment = max([abs(v) for _, (m, cr, en) in raw.items() for loc in cr for key, v in cr[loc][2].items() if key in ("T", "M_y", "M_z")] + [D(0)])
    for pid, (m, cr, en) in raw.items():
        force = max([abs(v) for loc in cr for key, v in cr[loc][2].items() if key in ("N", "S", "V_y", "V_z")] + [D(0)]) or case_force
        moment = max([abs(v) for loc in cr for key, v in cr[loc][2].items() if key in ("T", "M_y", "M_z")] + [D(0)]) or case_moment
        out["members"][pid]["zero_scale"] = {"force": fmt(force), "moment": fmt(moment)}
    out["discriminators"] = discriminators(raw, pressure, out)
    out["internal_checks"] = {key: fmt(v) for key, v in checks.items()}
    assert checks["rotation_formula_max_abs_error"] < D("1e-80"), checks
    assert checks["end_cut_identity_max_abs_error"] < D("1e-70"), checks
    return out, line


PUBLISHED = {PP1: ["F_x", "F_y", "F_z", "M_x", "M_y", "M_z"], PR1: ["F_y", "F_z", "M_x", "M_y", "M_z"]}


def discriminators(raw, pressure, out):
    """Each wrong frame or sign, with its distance from the oracle in tolerances on the published arc end rows."""
    route = PR1 if pressure else PP1
    keys = PUBLISHED[route]
    results = {}

    def tol(pid, key, expected):
        zs = out["members"][pid]["zero_scale"]
        scale = D(zs["moment" if key[0] == "M" else "force"])
        return D("1e-9") * max(abs(expected), scale)

    def mutant_rows(name, m, ends, loc):
        Fe, Me, u = ends[loc]
        other = "end_j" if loc == "end_i" else "end_i"
        if name == "chord_frame_unrotated":  # today's rows; also a rotation applied twice (R is an involution)
            return E.components(Fe, Me, m.chord_axes)
        if name == "frame_swap_between_ends":
            return E.components(Fe, Me, m.axes(ends[other][2]))
        if name == "cut_sign_at_end_i":  # +cut(0) published as the end_i row
            return [-v for v in E.components(Fe, Me, m.axes(u))] if loc == "end_i" else E.components(Fe, Me, m.axes(u))
        if name == "sign_flip_at_end_j":
            return [-v for v in E.components(Fe, Me, m.axes(u))] if loc == "end_j" else E.components(Fe, Me, m.axes(u))
        if name == "end_rows_swapped":
            F2, M2, u2 = ends[other]
            return E.components(F2, M2, m.axes(u2))
        if name in ("inplane_rotation_without_flip", "full_angle_instead_of_half"):
            return E.components(Fe, Me, mutant_frames(m, loc)[name])
        if name == "binary64_tangent_axes":
            axes = tuple(tuple(D(float(v)) for v in a) for a in m.axes(u))
            return E.components(Fe, Me, axes)
        raise KeyError(name)

    names = ["chord_frame_unrotated", "frame_swap_between_ends", "cut_sign_at_end_i", "sign_flip_at_end_j", "end_rows_swapped",
             "inplane_rotation_without_flip", "full_angle_instead_of_half", "binary64_tangent_axes"]
    for name in names:
        rows = []
        for pid, (m, cr, ends) in raw.items():
            if m.kind != "arc":
                continue
            for loc in ("end_i", "end_j"):
                Fe, Me, u = ends[loc]
                expected = E.components(Fe, Me, m.axes(u))
                got = mutant_rows(name, m, ends, loc)
                for a, key in enumerate(KEYS):
                    if key not in keys:
                        continue
                    d = abs(got[a] - expected[a]) / tol(pid, key, expected[a])
                    rows.append((d, pid, loc, key, got[a]))
        rows.sort(key=lambda r: (-r[0], r[1], r[2], r[3]))
        results[name] = {"max_distance_in_tolerances": fmt(rows[0][0]) if rows else "0",
                         "listed_rows": [{"pipe": r[1], "location": r[2], "row": r[3], "mutant_value": fmt(r[4]),
                                          "distance_in_tolerances": fmt(r[0])} for r in rows[:6] if r[0] >= 1000]}
    return results


# ---------------------------------------------------------------- T4-I7 cross-check

def i7_model(case):
    i = case["inputs"]; g = i["geometry"]
    members = []
    for m in g["members"]:
        spec = {"pipe": m["pipe"], "kind": m["kind"], "authored_from": m["authored_from"], "authored_to": m["authored_to"],
                "y_reference": m["y_reference_binary64_exact"]}
        if m["kind"] == "arc":
            spec.update(R=m["bend_radius"], k=m["flexibility_factor_k"])
        members.append(spec)
    w = D(i["self_weight"]["w_N_per_m"])
    return {"nodes": g["node_coordinates_binary64_exact"], "traversal_order": g["traversal_order"], "members": members,
            "section": i["section"], "material": {"E": i["material"]["E"], "nu": i["material"]["nu"], "alpha": i["material"]["alpha_per_degC"]},
            "loads": {"p": i["pressure"]["p_Pa"], "terminal_A": i["pressure"]["terminal_A"], "terminal_D": i["pressure"]["terminal_D"],
                      "delta_T_all": i["thermal"]["delta_T_degC"], "uniform_all": [] if w == 0 else [["0", "0", str(-w)]]},
            "anchor_last": any(s.endswith("at " + g["traversal_order"][-1]) for s in i["supports"]),
            "section_bases": "d5"}  # T4-I7 is exact-route (D-5); its allowance is 0


def i7_cross_check():
    data = I7_PATH.read_bytes()
    i7 = json.loads(data)
    worst, count, compared = D(0), 0, 0
    where = None
    for cid, case in i7["cases"].items():
        if not isinstance(case.get("expected"), dict) or "members" not in case["expected"]:
            continue
        line = E.Line(i7_model(case)).solve()
        zs = case["zero_scale"]
        count += 1

        def cmp(ptr, got, exp, scale_key):
            nonlocal worst, compared, where
            e = D(exp)
            scale = D(zs[scale_key]["zero_scale"]) if scale_key in zs else D(0)
            denom = max(abs(e), scale)
            d = abs(got - e) / denom if denom else abs(got - e)
            compared += 1
            if d > worst:
                worst, where = d, f"{cid}:{ptr}"
        for k, m in enumerate(line.members):
            mem = case["expected"]["members"][m.id]
            for loc, u in E.FRACTIONS:
                F, M = line.cut(k, u)
                rows = E.components(F, M, m.axes(u))
                st = mem["stations"][loc]
                cmp(f"{m.id}/{loc}/N_w", rows[0], st["N_w"], "wall_axial_force")
                for key, idx, group in [("V_y", 1, "shear_force"), ("V_z", 2, "shear_force"), ("T", 3, "section_moment"), ("M_y", 4, "section_moment"), ("M_z", 5, "section_moment")]:
                    cmp(f"{m.id}/{loc}/{key}", rows[idx], st[key], group)
            if m.kind != "arc":
                continue
            for loc, u, sign in [("end_i", D(0), D(-1)), ("end_j", D(1), D(1))]:
                F, M = line.cut(k, u)
                Fw, Mw = E.scale(F, sign), E.scale(M, sign)
                tangent = E.components(Fw, Mw, m.axes(u))
                for idx, key in enumerate(KEYS):
                    cmp(f"{m.id}/{loc}/tangent_frame/{key}", tangent[idx], mem["end_rows"][loc]["tangent_frame"][key],
                        "section_moment" if key[0] == "M" else "wall_axial_force")
                Fe = E.add(Fw, E.scale(m.axes(u)[0], -sign * line.p_ai))
                chord = E.components(Fe, Mw, m.chord_axes)
                for idx, key in enumerate(KEYS):
                    cmp(f"{m.id}/{loc}/chord_frame_elastic/{key}", chord[idx], mem["end_rows"][loc]["chord_frame_elastic"][key],
                        "section_moment" if key[0] == "M" else "elastic_end_force_chord_frame")
        fa, ma = line.support_a()
        sup = case["expected"]["supports"]
        a = next(s for s in sup if s.endswith(":A"))
        for key, v in zip(["Fx", "Fy", "Fz", "Mx", "My", "Mz"], list(fa) + list(ma)):
            cmp(f"{a}/{key}", v, sup[a][key], f"support_{'force' if key[0] == 'F' else 'moment'}@{a}")
    assert worst < D("1e-18"), (worst, where)
    return {"file": "validation/references/t4_i7/u2_reference_cases.json", "sha256": hashlib.sha256(data).hexdigest(),
            "valued_cases": count, "values_compared": compared,
            "compared": "every station (N_w, V_y, V_z, T, M_y, M_z), every arc's tangent_frame and chord_frame_elastic end rows, support A",
            "criterion": "|engine - T4-I7| <= 1e-18 * max(|T4-I7|, the case's own zero_scale for the group)",
            "max_relative_difference": f"{worst:.3e}"}


# ------------------------------------------------------------ objectivity

PAIRS = [("U4A-PP1-L-LEFT-TIP", "U4A-PP1-L-SKEW-TIP"), ("U4A-PP1-L-LEFT-TIP", "U4A-PP1-L-SKEW-X5E6-TIP"),
         ("U4A-PP1-U-ANCH-TH", "U4A-PP1-U-SKEW-ANCH-TH"), ("U4A-PR1-L-LEFT-TIP-P", "U4A-PR1-L-SKEW-TIP-P"),
         ("U4A-PR1-U-ANCH-PT", "U4A-PR1-U-SKEW-ANCH-PT")]


def objectivity(cases):
    """A rigidly rotated model with rotated loads has the same frame-local rows (to binary64 input rounding)."""
    out = []
    for a, b in PAIRS:
        worst = D(0)
        for pid, ma in cases[a]["members"].items():
            mb = cases[b]["members"][pid]
            scale = {"F": D(ma["zero_scale"]["force"]), "M": D(ma["zero_scale"]["moment"])}
            for loc, rows in ma["end_rows_node_on_element_elastic"].items():
                for key, v in rows["tangent_frame"].items():
                    w = D(mb["end_rows_node_on_element_elastic"][loc]["tangent_frame"][key])
                    worst = max(worst, abs(D(v) - w) / max(abs(D(v)), scale[key[0]]))
            for loc, rows in ma["cuts_tangent_frame_j_side"].items():
                for key, v in rows.items():
                    w = D(mb["cuts_tangent_frame_j_side"][loc][key])
                    worst = max(worst, abs(D(v) - w) / max(abs(D(v)), scale["M" if key[0] in "TM" else "F"]))
        # The rotated nodes are rounded to binary64: about 1e-16 relative at moderate coordinates, but one
        # ulp of 5e6 m (9.3e-10 m) against R = 0.375 m perturbs the X5E6 arc's geometry by about 1e-9.
        bound = D("1e-8") if "X5E6" in b else D("1e-12")
        assert worst < bound, (a, b, worst)
        out.append({"planar": a, "rotated": b, "max_relative_difference_tangent_end_rows_and_cuts": f"{worst:.3e}",
                    "bound": f"{bound:.0e}", "note": "differences are the binary64 rounding of the rotated inputs, which the oracle takes as exact"})
    return out


# ------------------------------------------------------------------- main

def precision_check():
    """The 48-point quadrature against 64 points on two indeterminate arc cases."""
    worst = D(0)
    for cid in ("U4A-PP1-3D-ANCH-TH", "U4A-PR1-U-SKEW-ANCH-PT"):
        c = next(x for x in CASES if x["case_id"] == cid)
        a = E.Line(engine_model(c)).solve(48)
        b = E.Line(engine_model(c)).solve(64)
        for v, w in zip(list(a.redundant[0]) + list(a.redundant[1]), list(b.redundant[0]) + list(b.redundant[1])):
            worst = max(worst, abs(v - w) / max(abs(w), D(1)))
    assert worst < D("1e-40"), worst
    return f"{worst:.3e}"


def build():
    requests = {"schema_version": "u4a.requests/1.0.0",
                "purpose": "Invented T4-U4a arc models as product documents: preview-physics-1 cases are 0.2.0 pressure-free documents; pressure-1 cases are 0.4.0 documents under 3.0.0/exact_pressure_v3.",
                "cases": {}}
    cases = {}
    for c in CASES:
        oracle, line = oracle_case(c)
        requests["cases"][c["case_id"]] = request_document(c, line)
        cases[c["case_id"]] = oracle
    req_bytes = (json.dumps(requests, indent=1, ensure_ascii=False) + "\n").encode()
    oracle = {
        "schema_version": SCHEMA,
        "purpose": "Frozen independent expectations for T4-U4a (DEL-04-07 Design/arc-frames-and-signs.md): the end rows of realized arcs in the tangent frame at each end, node-on-element, on preview-physics-1 and pressure-1; the unchanged cut rows; and wrong-frame and wrong-sign discriminators.",
        "requests_file": "requests.json",
        "requests_sha256": hashlib.sha256(req_bytes).hexdigest(),
        "basis": "u4a_engine.py: unit-load (flexibility) method, 90-digit decimal, closed-form statics, 48-point Gauss-Legendre virtual-work integrals; imports no product code",
        "numbers": "Invented test quantities. Not material-library, component-library or code-rule data.",
        "inputs": "node coordinates, y_reference components and load components are the binary64 values written in requests.json and are used as exact inputs; section, material, radius, k, pressure and temperature are exact decimal targets",
        "conventions": {
            "chord_frame": "x = (x_j - x_i)/L, y = y_reference projected normal to x and normalised (the bow direction), z = x cross y",
            "tangent_frame": "at arc angle psi from authored node i: x = arc tangent toward authored j, y = toward the arc centre, z = bend-plane normal (x_i - c) cross (x_j - c), normalised; on a straight, the element-local frame",
            "cuts": "j-side section action at the station in the frame at that station (positive N is tension); pressure-1 gives N_w (wall) and S = N_w - p Ai",
            "end_rows": "node-on-element elastic action: end_i = -(elastic cut at fraction 0), end_j = +(elastic cut at fraction 1), in the tangent frame at that end; chord_frame is the same vector in the chord frame (today's arc end rows)",
            "chord_to_tangent_rotation": "rows are the tangent axes at that end in chord components: end_i [[c, s, 0], [s, -c, 0], [0, 0, -1]], end_j [[c, -s, 0], [-s, -c, 0], [0, 0, -1]], s = L/(2R), c = sqrt(1 - s^2)",
            "wall_end_action": "pressure-1: node-on-element wall action along the end tangent, end_i = -N_w(0), end_j = +N_w(1)",
            "stresses": "SI (Pa): sigma_axial = N/As (preview-physics-1) or sigma_m_wall_membrane = N_w/As (pressure-1); sigma_b_y = M_y/Z; sigma_b_z = M_z/Z; tau_t = T r_o/J",
            "section_bases": "pressure-1 (D-5, T4-U6; DEL-05-03 Design sections 1 and 2.1): stiffness (EA, EI, GJ and the As of the Poisson strain) on the nominal wall, p Ai on the pressure bore OD/2 - ((wall - mill) - allowance) with allowance 0 (the documents carry none; the current-format re-authoring writes 0), and the stresses on each basis in stresses_SI_by_section_basis.{nominal_section, reduced_section} (A_b, Z_b, J_b of that basis); preview-physics-1 keeps its one section and nominal_stresses_SI",
            "supports": "support-on-pipe, global; moment about the attached node",
            "displacements": "global translations (m) and right-hand rotations (rad)",
        },
        "criterion": {
            "rule": "|observed - expected| <= 1e-9 * max(|expected|, zero_scale(member, family)) per row, in both solver modes",
            "zero_scale": "per member: force = max |N|, |S|, |V_y|, |V_z| over its five cuts; moment = max |T|, |M_y|, |M_z| over its five cuts (the case maximum if the member's is zero); displacements and supports use 1e-9 * max(|expected|, the case's maximum of that family)",
            "unit_transforms": "the oracle is SI; the product publishes translations in mm and element stresses in MPa: compare after exact conversion",
            "discriminators": "each wrong frame or sign must be at least 1e3 tolerances from the oracle on at least one published arc end row of the case; listed_rows give up to six such rows",
        },
        "objectivity_controls": objectivity(cases),
        "t4_i7_cross_check": i7_cross_check(),
        "quadrature_precision_check": {"cases": ["U4A-PP1-3D-ANCH-TH", "U4A-PR1-U-SKEW-ANCH-PT"], "redundants_48_vs_64_points_max_relative": precision_check()},
        "cases": cases,
    }
    oracle_bytes = (json.dumps(oracle, indent=1, ensure_ascii=False) + "\n").encode()
    return req_bytes, oracle_bytes


def main():
    req, orc = build()
    targets = [(HERE / "requests.json", req), (HERE / "oracle.json", orc)]
    if "--write" in sys.argv:
        for path, data in targets:
            path.write_bytes(data)
        print("wrote", ", ".join(p.name for p, _ in targets))
        return 0
    bad = [p.name for p, data in targets if not p.exists() or p.read_bytes() != data]
    if bad:
        print("MISMATCH:", ", ".join(bad))
        return 1
    print("requests.json and oracle.json reproduce byte for byte")
    return 0


if __name__ == "__main__":
    sys.exit(main())
