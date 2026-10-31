#!/usr/bin/env python3
"""Generate the reference-system fixture (DEL-09-01 `Work/reference-system.md`, exit criterion 1).

Every value is invented for this fixture (spec §1-§8); none comes from a standard, a supplier, a
commercial example or a project. The specification's tables are held here as data, and the
generator writes three files beside itself:

- `reference_system.json`: the model document, tranche 1 (every family the §10 ledger marks
  "in tranche 1"), on the exact route EX;
- `authoring.json`: the authoring marks, the bend corners and the pending ledger (RT-E1's input);
- `key_results.json`: the named results and equilibrium references of §9, each expected value naming its
  frozen record under `validation/references/reference_system/` (EXPECTED).

W (spec §8.1) is the product's own self-weight generator's output
(`core/product_physics/src/self_weight.rs` through `core/loads/self_weight_wasm`), captured in
`self_weight_capture.json`; this generator copies the captured primitives and computes no weight.
The capture records a digest of every input it read (the weight basis); a generation whose basis
differs from the capture's refuses, and the capture is refreshed with

    python3 fixtures/reference_system/generate_reference_system.py --capture-self-weight <glue-dir>

where `<glue-dir>` holds the wasm-bindgen `--target nodejs` glue of `open_pipe_stress_self_weight_wasm`
built with `--features wasm` (`capture_self_weight.mjs` drives it), for example

    cargo build --locked --release --lib --target wasm32-unknown-unknown --features wasm \
        --manifest-path core/loads/self_weight_wasm/Cargo.toml --target-dir <target>
    wasm-bindgen --target nodejs --out-dir <glue-dir> \
        <target>/wasm32-unknown-unknown/release/open_pipe_stress_self_weight_wasm.wasm

The model-document version is the one constant MODEL_DOCUMENT_VERSION and is not part of the weight
basis, so a version ripple re-runs this generator without a new capture.

Usage:
    python3 fixtures/reference_system/generate_reference_system.py           # write
    python3 fixtures/reference_system/generate_reference_system.py --check   # compare only
"""
from __future__ import annotations

import argparse
from decimal import Decimal
import hashlib
import json
from pathlib import Path
import subprocess
import sys

MODEL_DOCUMENT_VERSION = "0.8.0"
HERE = Path(__file__).resolve().parent
SPEC = "execution/PKG-09/DEL-09-01/Work/reference-system.md"
MODEL_FILE = "reference_system.json"
AUTHORING_FILE = "authoring.json"
KEY_RESULTS_FILE = "key_results.json"
CAPTURE_FILE = "self_weight_capture.json"
CAPTURE_DRIVER = "capture_self_weight.mjs"

PROVENANCE = ("invented reference-system fixture input (DEL-09-01 Work/reference-system.md); "
              "not standard, supplier, commercial or project data")
SOURCE = "invented user value, DEL-09-01 Work/reference-system.md §5"
ROUTE = {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"}
GRAVITY = {"value": -9.80665, "unit": "m/s^2", "axis": "global_z"}  # spec §2: standard gravity along -Z

# -- §3.1 nodes (m) -------------------------------------------------------------------------------------------------
NODES = [
    ("RS-010", "0", "0", "1.000", "Anchor A (nozzle A)"),
    ("RS-020", "0.400", "0", "1.000", "Flange F1 start"),
    ("RS-030", "0.550", "0", "1.000", "Valve V1 start"),
    ("RS-040", "1.150", "0", "1.000", "Flange F2 start"),
    ("RS-050", "1.300", "0", "1.000", "Flange F2 end"),
    ("RS-060", "3.500", "0", "1.000", "Line stop LS1; vertical support V060"),
    ("RS-070", "5.600", "0", "1.000", "Bend B1 near tangent"),
    ("RS-075", "6.000", "0", "1.400", "Bend B1 far tangent"),
    ("RS-080", "6.000", "0", "4.000", "Variable spring VS1 (riser)"),
    ("RS-090", "6.000", "0", "6.600", "Bend B2 near tangent"),
    ("RS-095", "6.000", "0.400", "7.000", "Bend B2 far tangent"),
    ("RS-100", "6.000", "3.000", "7.000", "Constant-effort support CE1"),
    ("RS-110", "6.000", "7.000", "7.000", "Lift-off support LO1"),
    ("RS-120", "6.000", "11.000", "7.000", "Resting support with friction FR1"),
    ("RS-130", "6.000", "15.000", "7.000", "Tee T1"),
    ("RS-140", "6.000", "18.000", "7.000", "Gap guide G1; vertical support V140"),
    ("RS-150", "6.000", "20.000", "7.000", "Reducer R1, large end"),
    ("RS-160", "6.000", "20.400", "6.975", "Reducer R1, small end (eccentric, flat on bottom)"),
    ("RS-170", "6.000", "23.000", "6.975", "Guide G2"),
    ("RS-180", "6.000", "25.000", "6.975", "Expansion joint EJ1, face i"),
    ("RS-190", "6.000", "25.600", "6.975", "Expansion joint EJ1, face j"),
    ("RS-200", "6.000", "28.000", "6.975", "Vertical support V200"),
    ("RS-210", "6.000", "30.000", "6.975", "Anchor B (nozzle B)"),
    ("RS-215", "6.000", "15.000", "4.500", "Branch guide G3"),
    ("RS-220", "6.000", "15.000", "2.250", "Bend B3 near tangent"),
    ("RS-225", "6.250", "15.000", "2.000", "Bend B3 far tangent"),
    ("RS-230", "8.000", "15.000", "2.000", "Anchor C (nozzle C; seismic anchor movement)"),
]

# -- §3.2 members -----------------------------------------------------------------------------------------------------
MEMBERS = [
    ("RS-P010", "RS-010", "RS-020", "S270", ""),
    ("RS-P020", "RS-020", "RS-030", "S270", "Flange F1"),
    ("RS-P030", "RS-030", "RS-040", "S270", "Valve V1"),
    ("RS-P040", "RS-040", "RS-050", "S270", "Flange F2"),
    ("RS-P050", "RS-050", "RS-060", "S270", ""),
    ("RS-P060", "RS-060", "RS-070", "S270", ""),
    ("RS-P065", "RS-070", "RS-075", "S270", "Bend B1 (arc)"),
    ("RS-P070", "RS-075", "RS-080", "S270", ""),
    ("RS-P080", "RS-080", "RS-090", "S270", ""),
    ("RS-P085", "RS-090", "RS-095", "S270", "Bend B2 (arc)"),
    ("RS-P090", "RS-095", "RS-100", "S270", ""),
    ("RS-P100", "RS-100", "RS-110", "S270", ""),
    ("RS-P110", "RS-110", "RS-120", "S270", ""),
    ("RS-P120", "RS-120", "RS-130", "S270", "Tee T1 header (in)"),
    ("RS-P130", "RS-130", "RS-140", "S270", "Tee T1 header (out)"),
    ("RS-P140", "RS-140", "RS-150", "S270", ""),
    ("RS-P150", "RS-150", "RS-160", "S270", "Reducer R1 (plain S270 member in tranche 1)"),
    ("RS-P160", "RS-160", "RS-170", "S220", ""),
    ("RS-P170", "RS-170", "RS-180", "S220", "EJ1 face pipe i"),
    ("RS-P180", "RS-180", "RS-190", "S220", "Replaced by EJ1 (carries no load or result)"),
    ("RS-P190", "RS-190", "RS-200", "S220", "EJ1 face pipe j"),
    ("RS-P200", "RS-200", "RS-210", "S220", ""),
    ("RS-P210", "RS-130", "RS-215", "S170", "Tee T1 branch"),
    ("RS-P220", "RS-215", "RS-220", "S170", ""),
    ("RS-P225", "RS-220", "RS-225", "S170", "Bend B3 (arc)"),
    ("RS-P230", "RS-225", "RS-230", "S170", ""),
]
REPLACED_SPAN = "RS-P180"

# -- §4 material and sections -----------------------------------------------------------------------------------------
MATERIAL = "RS-M1"
POISSON = 0.30
ALPHA = 1.2e-05  # 1/K, constant
INSTALLATION_C = 20
OPERATING_C = 210
TEMPERATURE_POINTS = [(20, 200), (100, 195), (200, 189), (300, 182)]  # (degC, GPa)
DENSITY = 7850  # kg/m^3, on each section
# (OD mm, wall mm, mill tolerance mm, corrosion allowance mm, insulation mm, insulation kg/m^3, contents kg/m^3)
SECTIONS = {
    "S270": (270, 9.0, 1.0, 1.5, 60, 140, 820),
    "S220": (220, 8.0, 0.9, 1.5, 60, 140, 820),
    "S170": (170, 7.0, 0.8, 1.5, 50, 140, 820),
}

# -- §5 components ----------------------------------------------------------------------------------------------------
# Realized bends: corner, radius (m), k, SIF in-plane, SIF out-of-plane, the incoming and outgoing straights, the
# tangent nodes and arc pipe the bend authoring creates. Insert bend at corner takes every new identity from its
# payload (`new_nodes`, `new_pipe`, `component`; bend_authoring.rs insert_at_corner), so the spec's §3 names are kept.
BENDS = [
    ("RS-B1", ("6", "0", "1"), "0.400", 4.2, 2.1, 1.8, "RS-P060", "RS-P070", ("RS-070", "RS-075"), "RS-P065"),
    ("RS-B2", ("6", "0", "7"), "0.400", 4.2, 2.1, 1.8, "RS-P080", "RS-P090", ("RS-090", "RS-095"), "RS-P085"),
    ("RS-B3", ("6", "15", "2"), "0.250", 3.6, 2.3, 1.9, "RS-P220", "RS-P230", ("RS-220", "RS-225"), "RS-P225"),
]
# Tee (branch connection record at its node): tag, node, header pipe, branch pipe, header SIF, branch SIF, connection
# angle (deg). Spec §5 gives no run or header size, connection type, reinforcement reference or flexibility factor, so
# the record carries none; with no mechanics_interface it is consumed as mechanics_geometry_only, the solver's default
# (preview_physics.rs solver_consumption), and the desktop's tee creation writes none either.
TEE = ("RS-T1", "RS-130", "RS-P120", "RS-P210", 2.4, 2.0, 90)
# §5: the tee's descriptive geometry, which tee creation requires (geometry only; EX joins the tee rigidly).
TEE_RUN_SIZE_MM = 170.0  # the branch outlet, RS-S170's outside diameter
TEE_HEADER_SIZE_MM = 270.0  # the header run, RS-S270's outside diameter
TEE_CONNECTION_TYPE = "invented unreinforced tee"
TEE_REINFORCEMENT_REFERENCE = "invented: no reinforcement"
# Rigid family: tag, kind, member, node, weight (N), CoG offset from the body midpoint (m, global), stiffness scale.
RIGID = [
    ("RS-F1", "flange", "RS-P020", "RS-030", 350, ("0", "0", "0"), 10.0),
    ("RS-V1", "valve", "RS-P030", "RS-040", 2600, ("0", "0", "0.30"), 10.0),
    ("RS-F2", "flange", "RS-P040", "RS-050", 350, ("0", "0", "0"), 10.0),
]
# §5.1 expansion joint: N/m for translations, N*m/rad for rotations (unit scales), in connector axes tx (pipe axis),
# ty, tz (lateral), rx (torsion), ry, rz (bending).
JOINT = {
    "tag": "RS-EJ1", "span": REPLACED_SPAN, "end_i": "RS-180", "end_j": "RS-190",
    "axes_columns": [(0, 1, 0), (-1, 0, 0), (0, 0, 1)],  # x = +Y, y = -X, z = +Z
    "diagonal": [150000.0, 400000.0, 400000.0, 500000.0, 1500.0, 1500.0],
    "effective_area_m2": 0.0490,
    "rods": [("RS-EJ1-RYP", "0.200"), ("RS-EJ1-RYN", "-0.200")],
    "rod_stiffness": 1.2e8,
}

# -- §6 supports (tranche 1) ------------------------------------------------------------------------------------------
SUPPORTS = [
    ("RS-A", "RS-010", "anchor", ["UX", "UY", "UZ", "RX", "RY", "RZ"], "Anchor A (nozzle A)"),
    ("RS-B", "RS-210", "anchor", ["UX", "UY", "UZ", "RX", "RY", "RZ"], "Anchor B (nozzle B)"),
    ("RS-C", "RS-230", "anchor", ["UX", "UY", "UZ", "RX", "RY", "RZ"], "Anchor C (nozzle C)"),
    ("RS-LS1", "RS-060", "line_stop", ["UX"], "Line stop LS1"),
    ("RS-V060", "RS-060", "vertical_support", ["UZ"], "Vertical support V060"),
    ("RS-VS1", "RS-080", "variable_spring_hanger", [], "Variable spring VS1 (user stiffness)"),
    ("RS-FR1V", "RS-120", "vertical_support", ["UZ"],
     "RS-FR1V interim linear vertical support (one_way target pending R-4 S7)"),
    ("RS-V140", "RS-140", "vertical_support", ["UZ"], "Vertical support V140"),
    ("RS-G2", "RS-170", "guide", ["UX", "UZ"], "Guide G2"),
    ("RS-V200", "RS-200", "vertical_support", ["UZ"], "Vertical support V200"),
    ("RS-G3", "RS-215", "guide", ["UX", "UY"], "Branch guide G3"),
]
SPRING_STIFFNESS = 180000.0  # N/m, UZ
# §6: the spring's entered loads and travel, review metadata only (never applied as preload on main).
SPRING_LOADS_N = {"installed_load": 8000.0, "cold_load": 8000.0, "hot_load": 7400.0}
SPRING_TRAVEL_RANGE_MM = 50.0
# Supports a pending ledger row keeps out of tranche 1 (their slots and ids stay).
PENDING_SUPPORTS: set[str] = set()

# -- §7 pressure regions ----------------------------------------------------------------------------------------------
REGIONS = [
    ("RG-1", ["RS-P010", "RS-P020", "RS-P030", "RS-P040", "RS-P050", "RS-P060", "RS-P065", "RS-P070", "RS-P080",
              "RS-P085", "RS-P090", "RS-P100", "RS-P110", "RS-P120"], [], ("RS-010", "RS-130")),
    ("RG-2", ["RS-P210", "RS-P220", "RS-P225", "RS-P230"], [], ("RS-130", "RS-230")),
    ("RG-3", ["RS-P130", "RS-P140"], [], ("RS-130", "RS-150")),
    ("RG-R", ["RS-P150"], [], ("RS-150", "RS-160")),
    ("RG-4", ["RS-P160", "RS-P170", "RS-P190", "RS-P200"], ["RS-EJ1"], ("RS-160", "RS-210")),
]
OPERATING_PRESSURE_KPA = 1600

# -- §8 tranche-1 load cases and combinations -------------------------------------------------------------------------
CASES = [
    ("RS-SUS", "RS-SUS sustained: W + P (operating pressure)", False, None),
    ("RS-OPE", "RS-OPE operating: W + P + T (210 degC, modulus at 210 degC)", True, None),
    ("RS-SAM", "RS-SAM seismic anchor movement: W + P + anchor RS-C moved UX +15 mm, UY -10 mm", False,
     ("RS-C", [("UX", 15), ("UY", -10)])),
]
COMBINATIONS = [
    ("RS-EXP", "RS-EXP expansion: RS-OPE - RS-SUS", "RS-OPE", "RS-SUS"),
    ("RS-dSAM", "RS-dSAM anchor movement term: RS-SAM - RS-SUS", "RS-SAM", "RS-SUS"),
]

# -- §10 pending-family ledger (family text, state, row / lane, expected until then), as in the spec ----------------
LEDGER = [
    ("Straight members, sections, temperature-dependent material", "in tranche 1", "—", "—"),
    ("Realized bends with user k and SIFs", "in tranche 1", "—", "—"),
    ("Valve and flanges, weight at CoG, stiffness scale", "in tranche 1", "—", "—"),
    ("Expansion joint with tie rods, pressurized", "in tranche 1", "—", "—"),
    ("Pressure (five regions, shared terminals `transfers_to_wall`)", "in tranche 1", "—", "—"),
    ("Anchors, guides, line stop, vertical supports", "in tranche 1", "—", "—"),
    ("Variable spring as a user-stiffness spring", "in tranche 1", "—", "—"),
    ("Tee RS-T1 with user SIFs", "in tranche 1", "—", "—"),
    ("Eccentric reducer RS-R1 in region RG-R",
     "pending; RS-P150 is in tranche 1 as a plain S270 member in its own region", "RT-A3 increment 2",
     "`EXACT_PRESSURE_REDUCER_IN_REGION`"),
    ("Constant-effort support RS-CE1", "pending", "RT-A3 increment 3", "`EXACT_PRESSURE_FAMILY_NOT_ADMITTED`"),
    ("Lift-off, friction and gap (RS-LO1, RS-FR1, RS-G1)", "pending",
     "RT-A3 increment 4; with the connector, increment 5", "`EXACT_PRESSURE_FAMILY_NOT_ADMITTED`"),
    ("RS-FR1 with RS-FR1V: lift-off friction shoe pending RT-A3 S7",
     "interim: RS-FR1V authored as a linear `vertical_support` UZ (Agent 0, 2026-10-31)",
     "RT-A3, R-4 stage S7, after increment 4 (DEL-04-07 `Work/route-unification-r4.md` §0a)",
     "a nonlinear friction normal source is refused on both routes"),
    ("RS-SUS, RS-OPE (thermal, modulus by temperature)",
     "in tranche 1 as linear cases (load/reference-state group)", "—", "—"),
    ("RS-EXP", "in tranche 1 as a subtraction; `difference_basis` pending",
     "RT-A5 slice 1b / load-cases lane (0.9.0)", "basis not recorded"),
    ("RS-WIN, RS-EQX, RS-EQY, RS-EQZ on the document that carries temperature", "pending",
     "#1678 / load-cases lane", "`EXACT_EQUIVALENT_STATIC_LOAD_STATE_UNSUPPORTED`"),
    ("RS-SAM (`boundary_motion` on rigid anchor RS-C)",
     "in tranche 1, on the load/reference-state document beside the temperature state; it shares #1678's "
     "conflict with wind and seismic", "— (plain EX documents: RT-A3)", "—"),
    ("RS-EQ-SRSS, RS-EQ-ABS, RS-OCC-SRSS, RS-OCC-ABS", "pending",
     "RT-A4 / load-cases lane (0.9.0 S1, then its slice)", "not authorable at 0.8.0"),
    ("RS-HYD", "pending", "RT-A6 / load-cases lane (0.9.0, then its slice)", "not authorable at 0.8.0"),
    ("Operating condition set", "pending", "RT-G5 / load-cases lane (0.9.0)", "authored per case at 0.8.0"),
    ("Modal (RS-OPE linearization)", "pending", "RT-A7 / dynamics lane (DEL-04-09, M5 in scope)",
     "no modal analysis"),
    ("Response spectrum, RS-SP1", "pending", "RT-A8 / dynamics lane (DEL-04-10, proposed)",
     "no spectrum analysis; `spectra[]` not authorable at 0.8.0"),
    ("Hanger sizing and table", "pending", "RT-A11 / DEL-04-11", "no sizing procedure"),
    ("Snubber RS-SN1 (RS-110, UX, rigid)", "pending",
     "RT-A8 / dynamics lane (DEL-04-10 slice: static solves ignore a snubber)",
     "`SNUBBER_DYNAMIC_ONLY_UNSUPPORTED` on every static solve; not authorable at 0.8.0"),
]
# The model entities each tranche-1 row covers, and the entities a pending row will add (ids of spec §2-§8).
LEDGER_ENTITIES = {
    "Tee RS-T1 with user SIFs": ["component:RS-T1"],
    "Eccentric reducer RS-R1 in region RG-R": ["component:RS-R1"],
    "Constant-effort support RS-CE1": ["support:RS-CE1"],
    "Lift-off, friction and gap (RS-LO1, RS-FR1, RS-G1)": ["support:RS-LO1", "support:RS-FR1", "support:RS-G1"],
    "RS-FR1 with RS-FR1V: lift-off friction shoe pending RT-A3 S7": ["support:RS-FR1V"],
    "RS-WIN, RS-EQX, RS-EQY, RS-EQZ on the document that carries temperature":
        ["load:RS-WIN", "load:RS-EQX", "load:RS-EQY", "load:RS-EQZ", "combination:RS-dWIN", "combination:RS-dEQX",
         "combination:RS-dEQY", "combination:RS-dEQZ"],
    "RS-EQ-SRSS, RS-EQ-ABS, RS-OCC-SRSS, RS-OCC-ABS":
        ["combination:RS-EQ-SRSS", "combination:RS-EQ-ABS", "combination:RS-OCC-SRSS", "combination:RS-OCC-ABS"],
    "RS-HYD": ["load:RS-HYD"],
    "Response spectrum, RS-SP1": ["spectrum:RS-SP1"],
    "Snubber RS-SN1 (RS-110, UX, rigid)": ["support:RS-SN1"],
}

# -- §9 key results -------------------------------------------------------------------------------------------------
KEY_RESULTS = [
    ("KR-01", "Reactions at anchors A, B and C (three forces, three moments)", ["every case and combination"],
     "verified_solve", "verified solve"),
    ("KR-02", "Largest translation and its node",
     ["RS-SUS", "RS-OPE", "RS-EXP", "RS-HYD", "RS-OCC-SRSS", "RS-OCC-ABS"], "verified_solve", "verified solve"),
    ("KR-03", "Spring RS-VS1 and constant-effort RS-CE1 loads", ["RS-SUS", "RS-OPE"], "verified_solve",
     "verified solve"),
    ("KR-04", "Nonlinear support status: RS-LO1 in contact or lifted, RS-G1 open or closed, RS-FR1 sticking or "
     "sliding, with RS-FR1's force", ["RS-OPE", "RS-SUS"], "verified_solve",
     "verified solve; status must match exactly"),
    ("KR-05", "Tee intensified stress rows at the header (RS-P120) and branch (RS-P210) ends at RS-130, on both "
     "section bases", ["RS-SUS", "RS-OPE", "RS-EXP"], "verified_solve",
     "presence: identity; values: verified solve"),
    ("KR-06", "RS-EJ1 tie-rod forces, joint pressure force pAe and effective axial force",
     ["RS-SUS", "RS-OPE", "RS-HYD"], "verified_solve", "verified solve"),
    ("KR-07", "Hydrotest marked as a test case, with RS-VS1 and RS-CE1 held", ["RS-HYD"], "identity",
     "identity (published flags)"),
    ("KR-08", "Frequencies up to 60 Hz, mass participation per direction", ["modal"], "verified_solve",
     "verified solve"),
    ("KR-09", "Spectrum reactions at A, B, C and displacements, per direction and combined", ["spectrum"],
     "verified_solve", "verified solve"),
    ("KR-10", "Hanger table: per hanger, the sized load, travel and selected invented record", ["hanger sizing"],
     "verified_solve", "verified solve"),
    ("EQ-1", "The sum of vertical reactions over all supports, the spring and the constant-effort support included, "
     "equals the total weight, in RS-SUS, RS-OPE and RS-SAM, and in RS-HYD (with the test contents) when it lands",
     ["RS-SUS", "RS-OPE", "RS-SAM", "RS-HYD"], "closed_form", "closed form: the total weight recomputed independently "
     "from the inputs (§4, §5) on EX's stated mass bases"),
    ("EQ-2", "In RS-SUS, RS-OPE and RS-SAM, and in RS-HYD when it lands, the horizontal reaction resultant is zero, and "
     "the reaction moment resultant about the origin equals minus the moment of the weight",
     ["RS-SUS", "RS-OPE", "RS-SAM", "RS-HYD"], "closed_form", "closed form from the inputs"),
    ("EQ-3", "On every published signed component row, RS-EXP equals RS-OPE - RS-SUS; magnitude rows are "
     "recomputed from the combined components; withheld rows are excluded", ["RS-EXP", "RS-OPE", "RS-SUS"],
     "identity", "exact identity"),
    ("EQ-4", "On the same signed component rows, RS-EQ-SRSS equals sqrt(sum dEQ^2) and RS-EQ-ABS equals "
     "sum |dEQ| over the operand rows, so SRSS <= ABS on each", ["RS-EQ-SRSS", "RS-EQ-ABS", "RS-dEQX", "RS-dEQY",
                                                                  "RS-dEQZ"], "identity", "exact identity"),
    ("EQ-5", "The RS-dSAM reactions are self-equilibrated (resultant force and moment zero)", ["RS-dSAM"], "identity",
     "exact identity: no load differs between RS-SAM and RS-SUS"),
    ("EQ-6", "The resultant of RS-dWIN reactions equals minus the generated wind resultant, and that of RS-dEQx "
     "equals minus the factor times g times the total mass on the generator's stated mass basis",
     ["RS-dWIN", "RS-dEQX", "RS-dEQY", "RS-dEQZ"], "closed_form", "closed form from the inputs"),
    ("EQ-7", "Spectrum directional combination recomputed from the published per-direction results", ["spectrum"],
     "identity", "exact identity"),
]


# Frozen expected values (spec §1): copied from the independent references in REFERENCES (EQ-1, EQ-2; the fixture
# test checks the copy against references.json) or naming the identity and its record (EQ-3, EQ-5). The tranche-1
# cases only: RS-HYD's weight (test contents) is not yet referenced.
REFERENCES = "validation/references/reference_system/references.json"
REFERENCES_README = "validation/references/reference_system/README.md"
EXPECTED = {
    "EQ-1": {
        "basis_record": REFERENCES, "key": "W_total_N", "unit": "N", "cases": ["RS-SUS", "RS-OPE", "RS-SAM"],
        "check": "the sum of the global Z reaction components over every support equals W_total",
        "exact": "(3300) + (66155686593423/5000000000)*pi + (1363190839087/160000000000)*pi*sqrt(257) + "
                 "(124534361799687/800000000000)*pi^2",
        "value": 46832.31956637544,
    },
    "EQ-2": {
        "basis_record": REFERENCES, "key": "weight_moment_about_origin_N_m", "unit": "N*m",
        "cases": ["RS-SUS", "RS-OPE", "RS-SAM"],
        "check": "the horizontal (X, Y) reaction resultant is zero, and the reaction moment resultant about the origin "
                 "(sum of r x F + M over every support) equals minus the weight's moment about the origin",
        "value": [-444952.3802374701, 250652.31673076993, 0.0],
    },
    "EQ-3": {
        "basis_record": REFERENCES_README, "cases": ["RS-EXP", "RS-OPE", "RS-SUS"],
        "identity": "on every published signed component row, RS-EXP = RS-OPE - RS-SUS; magnitude rows equal the norm "
                    "of the combined components; rows the combination withholds are excluded and listed",
    },
    "EQ-5": {
        "basis_record": REFERENCES_README, "cases": ["RS-dSAM"],
        "identity": "the RS-dSAM reaction resultant force and its moment about the origin are zero",
    },
}
# Checks that bind a key result's presence (an identity) while its values await a verified solve; the expected value
# stays null until then (spec §1, §9.1).
CHECKS = {
    "KR-05": {
        "kind": "presence", "test": "tests/test_reference_system_fixture.py",
        "check": "in RS-SUS and RS-OPE the solve publishes exactly four branch_intensified_bending_stress_v1 rows, at "
                 "the RS-130 ends of RS-P120 (header, component:RS-T1 branch_header) and RS-P210 (branch, "
                 "branch_branch), each on the nominal and the reduced section basis, with their intensified_measures "
                 "records; RS-EXP publishes none today (the combination withholds intensified rows by name, "
                 "COMBINATION_INTENSIFIED_STRESS_UNAVAILABLE), and the same four if it ever publishes them; presence "
                 "and basis only, no value",
    },
}


# -- helpers --------------------------------------------------------------------------------------------------------
def q(value, unit):
    return {"value": value, "unit": unit}


def num(text):
    """A decimal text as the binary64 nearest it (integers stay integers)."""
    value = Decimal(text)
    return int(value) if value == value.to_integral_value() and "." not in text else float(value)


def position(node):
    row = next(r for r in NODES if r[0] == node)
    return tuple(Decimal(v) for v in row[1:4])


def member(tag):
    return next(r for r in MEMBERS if r[0] == tag)


def length(tag):
    _, start, end, _, _ = member(tag)
    return sum((a - b) ** 2 for a, b in zip(position(start), position(end))).sqrt()


def straight_y_reference(tag):
    """Horizontal straights take global +Z; vertical ones global +X (the generator's recorded orientation rule)."""
    _, start, end, _, _ = member(tag)
    a, b = position(start), position(end)
    return {"x": 1.0, "y": 0.0, "z": 0.0} if a[0] == b[0] and a[1] == b[1] else {"x": 0.0, "y": 0.0, "z": 1.0}


def bend_of_arc(tag):
    return next((b for b in BENDS if b[9] == tag), None)


def arc_y_reference(bend):
    """Insert bend at corner's arc y_reference: u_in, the unit direction of its first incident pipe (model order)
    toward the corner (bend_authoring.rs insert_at_corner)."""
    far = position(member(bend[6])[1])  # the incoming straight runs from this node to the corner
    corner = tuple(Decimal(v) for v in bend[1])
    d = [c - f for c, f in zip(corner, far)]
    length = max(abs(x) for x in d)
    assert sum(1 for x in d if x != 0) == 1, "axis-aligned straights"
    return {axis: float(x / length) for axis, x in zip("xyz", d)}


def section_record(tag):
    od, wall, mill, ca, ins, ins_rho, contents = SECTIONS[tag]
    return {
        "outside_diameter": q(od, "mm"), "wall_thickness": q(wall, "mm"), "mill_tolerance": q(mill, "mm"),
        "corrosion_allowance": q(ca, "mm"), "material_density": q(DENSITY, "kg/m^3"),
        "contents_density": q(contents, "kg/m^3"), "insulation_thickness": q(ins, "mm"),
        "insulation_density": q(ins_rho, "kg/m^3"),
    }


def matrix_from_columns(columns):
    return [[float(columns[c][r]) for c in range(3)] for r in range(3)]


IDENTITY3 = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]


# -- the model --------------------------------------------------------------------------------------------------------
def material():
    points = [{"id": f"point:RS-M1-T{t:03d}", "temperature": q(t, "degC"), "elastic_modulus": q(e, "GPa"),
               "poisson_ratio": q(POISSON, "1"), "thermal_expansion_coefficient": q(ALPHA, "1/K"),
               "provenance": PROVENANCE} for t, e in TEMPERATURE_POINTS]
    return {
        "id": f"material:{MATERIAL}", "label": "RS-M1 invented carbon-steel-like material",
        "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
        "elastic_modulus": q(TEMPERATURE_POINTS[0][1], "GPa"), "poisson_ratio": q(POISSON, "1"),
        "thermal_expansion_coefficient": q(ALPHA, "1/K"), "temperature_points": points,
        "expansion_laws": [{"id": "law:RS-M1-SECANT", "definition": "engineering_secant",
                            "datum_temperature": q(INSTALLATION_C, "degC"),
                            "data": {"kind": "constant", "coefficient": q(ALPHA, "1/K")}, "provenance": PROVENANCE}],
        "provenance": PROVENANCE,
    }


def pipes():
    rows = []
    for tag, start, end, section, carries in MEMBERS:
        bend = bend_of_arc(tag)
        rows.append({
            "id": f"pipe:{tag}", "label": tag + (f" ({carries})" if carries else ""), "from": f"node:{start}",
            "to": f"node:{end}", "material": f"material:{MATERIAL}", "section_ref": f"section:RS-{section}",
            "section": section_record(section),
            "y_reference": arc_y_reference(bend) if bend else straight_y_reference(tag), "provenance": PROVENANCE,
        })
    return rows


def components():
    rows = []
    for tag, _corner, radius, k, sif_in, sif_out, _incoming, _outgoing, tangents, arc in BENDS:
        rows.append({
            "id": f"component:{tag}", "label": f"Bend {tag[3:]}", "kind": "bend", "node": f"node:{tangents[0]}",
            "geometry": {"bend_pipe_ref": f"pipe:{arc}", "bend_radius": q(num(radius), "m"),
                         "bend_geometry_source_reference": SOURCE},
            "modifiers": {"flexibility_factor_user_value": q(k, "none"), "sif_in_plane_user_value": q(sif_in, "1"),
                          "sif_out_of_plane_user_value": q(sif_out, "1"), "source_reference": SOURCE},
            "mechanics_interface": {"solver_consumption": "curved_bend_macro_element"},
            "provenance": PROVENANCE,
        })
    tag, node, header, branch, sif_header, sif_branch, angle = TEE
    rows.append({
        "id": f"component:{tag}", "label": f"Tee {tag[3:]}", "kind": "tee", "node": f"node:{node}",
        "geometry": {"branch_header_pipe_ref": f"pipe:{header}", "branch_branch_pipe_ref": f"pipe:{branch}",
                     "branch_run_size": q(TEE_RUN_SIZE_MM, "mm"), "branch_header_size": q(TEE_HEADER_SIZE_MM, "mm"),
                     "branch_connection_angle": q(angle, "deg"), "branch_connection_type": TEE_CONNECTION_TYPE,
                     "branch_reinforcement_reference": TEE_REINFORCEMENT_REFERENCE,
                     "branch_geometry_source_reference": SOURCE},
        "modifiers": {"branch_header_sif_user_value": q(sif_header, "none"),
                      "branch_branch_sif_user_value": q(sif_branch, "none"), "source_reference": SOURCE},
        "provenance": PROVENANCE,
    })
    for tag, kind, pipe, node, weight, offset, scale in RIGID:
        _, start, end, _, _ = member(pipe)
        mid = [(a + b) / 2 for a, b in zip(position(start), position(end))]
        at = position(node)
        cog = [m + Decimal(o) - n for m, o, n in zip(mid, offset, at)]
        rows.append({
            "id": f"component:{tag}", "label": f"{kind.capitalize()} {tag[3:]}", "kind": kind, "node": f"node:{node}",
            "geometry": {"rigid_pipe_ref": f"pipe:{pipe}",
                         "rigid_body_length": q(float(length(pipe)), "m"),
                         "end_a_size": q(SECTIONS[member(pipe)[3]][0], "mm"),
                         "end_b_size": q(SECTIONS[member(pipe)[3]][0], "mm"),
                         "connection_end_a_reference": f"node:{start} (body end on {pipe})",
                         "connection_end_b_reference": f"node:{end} (body end on {pipe})",
                         "stiffness_behavior_reference": f"rigid body, user stiffness scale {scale} (DEC-045); "
                                                         f"{SOURCE}",
                         "weight": q(weight, "N"),
                         "center_of_gravity": {"x": float(cog[0]), "y": float(cog[1]), "z": float(cog[2]),
                                               "unit": "m"},
                         "rigid_component_source_reference": SOURCE},
            "modifiers": {"stiffness_scaling_user_value": q(scale, "none"), "source_reference": SOURCE},
            "mechanics_interface": {"solver_consumption": "mechanics_geometry_only"},
            "provenance": PROVENANCE,
        })
    j = JOINT
    upper = []
    for r in range(6):
        upper.extend(j["diagonal"][r] if c == r else 0.0 for c in range(r, 6))
    zero = {"x": 0.0, "y": 0.0, "z": 0.0}
    rows.append({
        "id": f"component:{j['tag']}", "label": "Expansion joint EJ1 (tied)", "kind": "expansion_joint",
        "node": f"node:{j['end_j']}",
        "objective_connector": {
            "version": "1.0.0", "motion_basis": "symmetric_midpoint_small_rotation_v1",
            "end_i": {"node_ref": f"node:{j['end_i']}", "initial_node_axes_global": IDENTITY3,
                      "offset_local": {**zero, "unit": "m"}},
            "end_j": {"node_ref": f"node:{j['end_j']}", "initial_node_axes_global": IDENTITY3,
                      "offset_local": {**zero, "unit": "m"}},
            "connector_axes_global": matrix_from_columns(j["axes_columns"]),
            "installed_reference_temperature": q(INSTALLATION_C, "degC"),
            "temperature_applicability": "fixed_installed_parameters_v1",
            "reference_state": "stress_free",
            "q_ref": {"translation": {**zero, "unit": "m"}, "rotation": {**zero, "unit": "rad"}},
            "stiffness": {
                "version": "1.0.0", "representation": "scaled_work_coefficients_v1",
                "translation_scale": q(1.0, "m"), "rotation_scale": q(1.0, "rad"), "coefficient_unit": "N*m",
                "upper_triangle": upper, "coordinate_order": ["tx", "ty", "tz", "rx", "ry", "rz"],
                "provenance": {"source_reference": SOURCE,
                               "measurement_restraints": "invented diagonal work matrix; no measurement",
                               "basis_transform_reference": "none (authored directly in the connector basis)"},
            },
            "topology": {"type": "replaces_span", "span_ref": f"pipe:{j['span']}"},
            "calibration": {"kind": "constant_structural_elasticity_v1", "includes_pressure_dependent_tangent": False,
                            "installed_geometry": "authored model geometry"},
            "hardware": {"kind": "explicit_axial_rods_v1", "rods": [
                {"id": f"rod:{rod}", "law": "bilateral_linear",
                 "offset_i": {"x": q(0.0, "m"), "y": q(num(y), "m"), "z": q(0.0, "m")},
                 "offset_j": {"x": q(0.0, "m"), "y": q(num(y), "m"), "z": q(0.0, "m")},
                 "stiffness": q(j["rod_stiffness"], "N/m"), "reference_extension": q(0.0, "m"),
                 "source_reference": SOURCE} for rod, y in j["rods"]]},
            "pressure_model": {"kind": "constant_effective_area_reference_v1",
                               "effective_area": q(j["effective_area_m2"], "m^2"),
                               "pressure_basis": "internal_differential_zero_external_v1", "source_reference": SOURCE},
            "provenance": {"source_reference": SOURCE, "measurement_restraints": "invented; no measurement",
                           "basis_transform_reference": "none",
                           "validity_statement": "invented test object; no applicability claim"},
        },
        "provenance": PROVENANCE,
    })
    return rows


def supports():
    rows = []
    for tag, node, family, restraints, label in SUPPORTS:
        if tag in PENDING_SUPPORTS:
            continue
        row = {"id": f"support:{tag}", "label": label, "node": f"node:{node}", "family": family,
               "restraints": restraints, "provenance": PROVENANCE}
        if family == "variable_spring_hanger":
            row["hanger"] = {"hanger_type": "variable_spring_hanger",
                             "stiffness": {"dof": "UZ", "value": q(SPRING_STIFFNESS, "N/m")},
                             **{name: q(value, "N") for name, value in SPRING_LOADS_N.items()},
                             "travel_range": q(SPRING_TRAVEL_RANGE_MM, "mm"),
                             "source_reference": SOURCE,
                             "mechanics_consumption": "linear_spring_primitive_user_stiffness"}
        rows.append(row)
    return rows


def base_model():
    """Everything but the load cases and combinations: the self-weight generator's input."""
    return {
        "schema_version": MODEL_DOCUMENT_VERSION,
        "document_kind": "openpipestress.product_preview.model",
        "pressure_contract": dict(ROUTE),
        "project": {"id": "project:RS", "name": "Reference system (invented)",
                    "description": "Invented reference system for exit criterion 1 (DEL-09-01 "
                                   "Work/reference-system.md), tranche 1; a test object, not a design",
                    "units": {"length": "m", "force": "N", "moment": "N*m", "angle": "rad", "pressure": "Pa",
                              "stress": "Pa", "temperature": "degC"}},
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "professional_acceptance": "not_provided",
                            "rule_check": "not_performed_user_rule_inputs_missing"},
        "materials": [material()],
        "sections": [{"id": f"section:RS-{tag}", "name": f"RS-{tag}", "section_type": "pipe",
                      "properties": {"outside_diameter": q(od, "mm"), "wall_thickness": q(wall, "mm")},
                      "provenance": PROVENANCE} for tag, (od, wall, *_rest) in SECTIONS.items()],
        "nodes": [{"id": f"node:{tag}", "label": f"{tag} {role}", "position": {"x": num(x), "y": num(y), "z": num(z)},
                   "provenance": PROVENANCE} for tag, x, y, z, role in NODES],
        "pipe_segments": pipes(),
        "supports": supports(),
        "components": components(),
        "reference_configurations": [{
            "id": "reference:RS-INSTALLED", "label": "Installed reference (20 degC)",
            "geometry_ref": {"kind": "authored_model_geometry"},
            "member_references": [{"pipe_ref": f"pipe:{tag}",
                                   "basis": {"kind": "temperature_reference",
                                             "installation_temperature": q(INSTALLATION_C, "degC")},
                                   "fit": {"kind": "none"}, "provenance": PROVENANCE} for tag, *_ in MEMBERS],
            "provenance": PROVENANCE}],
        "load_cases": [],
        "combinations": [],
    }


def weighted_pipes():
    return [f"pipe:{tag}" for tag, *_ in MEMBERS if tag != REPLACED_SPAN]


def weight_requests():
    return [{"case_id": f"load:{case}", "label": label, "pipe_refs": weighted_pipes(), "gravity": dict(GRAVITY),
             "provenance": f"{PROVENANCE}; W of {case}: self-weight of every member except {REPLACED_SPAN}, "
                           "component weights at their centres of gravity (spec §8.1)"}
            for case, label, _thermal, _motion in CASES]


def weight_basis(model):
    """Every input the self-weight generator reads, version token excluded (sha256 of sorted-key JSON)."""
    nodes = {n["id"]: n["position"] for n in model["nodes"]}
    basis = {
        "requests": weight_requests(),
        "pipes": [{"id": p["id"], "from": nodes[p["from"]], "to": nodes[p["to"]], "section": p["section"],
                   "section_ref": p.get("section_ref"), "provenance": p["provenance"]}
                  for p in model["pipe_segments"]],
        "sections": model["sections"],
        "components": [{k: c.get(k) for k in ("id", "kind", "node", "geometry", "mechanics_interface", "provenance")}
                       for c in model["components"]],
        "pressure_contract": model["pressure_contract"],
        "project_units": model["project"]["units"],
    }
    return "sha256:" + hashlib.sha256(json.dumps(basis, sort_keys=True, separators=(",", ":")).encode()).hexdigest()


def element_states(thermal):
    """RS-OPE: every member selects its modulus at the operating temperature, RS-P180 included (the MBF lane's
    coverage rule); every member but RS-P180 takes the free-length state at it, and RS-P180 keeps zero eigenstrain
    (spec §5.1). Other cases: the installed point and no thermal state."""
    rows = []
    for tag, *_ in MEMBERS:
        hot = thermal and tag != REPLACED_SPAN
        row = {"pipe_ref": f"pipe:{tag}"}
        if hot:
            row["operating_temperature"] = q(OPERATING_C, "degC")
        if thermal:
            row["material_selection"] = {"kind": "temperature_interpolation", "material_ref": f"material:{MATERIAL}",
                                         "temperature": q(OPERATING_C, "degC"), "interpolation": "piecewise_linear",
                                         "extrapolation": "forbidden"}
        else:
            row["material_selection"] = {"kind": "exact_point", "material_ref": f"material:{MATERIAL}",
                                         "point_ref": f"point:RS-M1-T{INSTALLATION_C:03d}"}
        if hot:
            row["thermal_state"] = {"kind": "free_length_state", "expansion_law_ref": "law:RS-M1-SECANT"}
        else:
            reason = ("replaced span RS-P180: zero eigenstrain (spec §5.1)" if tag == REPLACED_SPAN
                      else "no temperature in this case")
            row["thermal_state"] = {"kind": "unchanged_reference", "provenance": f"{PROVENANCE}; {reason}"}
        rows.append(row)
    return rows


def load_case(case, label, thermal, motion, captured):
    record = dict(captured["case_record"])
    primitives = captured["primitive_loads"]
    record["primitive_loads"] = primitives
    record["pressure_regions"] = [{
        "id": f"region:{case}-{region}", "member_pipe_ids": [f"pipe:{p}" for p in members],
        **({"member_connector_ids": [f"component:{c}" for c in connectors]} if connectors else {}),
        "pressure_basis": "internal_differential_zero_external_v1", "pressure": q(OPERATING_PRESSURE_KPA, "kPa"),
        "terminals": [{"node_ref": f"node:{n}", "closure_transfer": "transfers_to_wall", "provenance": PROVENANCE}
                      for n in terminals],
        "provenance": PROVENANCE} for region, members, connectors, terminals in REGIONS]
    states = []
    for tag, *_ in SUPPORTS:
        if tag in PENDING_SUPPORTS:
            continue
        state = {"support_ref": f"support:{tag}", "participation": {"kind": "active_model_device"}}
        if motion and motion[0] == tag:
            state["boundary_motion"] = [{"dof": dof, "value": q(mm, "mm"), "meaning": "absolute_reference_displacement"}
                                        for dof, mm in motion[1]]
        states.append(state)
    record["analysis_state"] = {
        "contract": "openpipestress.load_reference_state/1.0.0",
        "reference_configuration_ref": "reference:RS-INSTALLED",
        "element_states": element_states(thermal),
        "support_states": states,
        "load_sources": [{"source_ref": p["id"], "factor": 1.0} for p in primitives],
        "history": {"kind": "independent_equilibrium"},
        "provenance": PROVENANCE,
    }
    return record


def read_capture(model):
    path = HERE / CAPTURE_FILE
    if not path.is_file():
        raise SystemExit(f"{CAPTURE_FILE} is missing: run --capture-self-weight <glue-dir>")
    capture = json.loads(path.read_text(encoding="utf-8"))
    basis = weight_basis(model)
    if capture.get("weight_basis") != basis:
        raise SystemExit(f"{CAPTURE_FILE} is stale: its weight basis {capture.get('weight_basis')} is not the "
                         f"current {basis}; re-run --capture-self-weight <glue-dir>")
    by_case = {entry["request"]["case_id"]: entry for entry in capture["cases"]}
    if sorted(by_case) != sorted(f"load:{case}" for case, *_ in CASES):
        raise SystemExit(f"{CAPTURE_FILE} does not hold exactly the tranche-1 cases")
    return by_case


def model_document():
    model = base_model()
    captured = read_capture(model)
    model["load_cases"] = [load_case(case, label, thermal, motion, captured[f"load:{case}"])
                           for case, label, thermal, motion in CASES]
    model["combinations"] = [{"id": f"combination:{tag}", "label": label, "basis": "result_state_subtraction",
                              "minuend_id": f"load:{minuend}", "subtrahend_id": f"load:{subtrahend}",
                              "provenance": PROVENANCE} for tag, label, minuend, subtrahend in COMBINATIONS]
    return model


# -- authoring.json ---------------------------------------------------------------------------------------------------
PERSON_FIELDS = {
    "bend": {"modifiers.flexibility_factor_user_value", "modifiers.sif_in_plane_user_value",
             "modifiers.sif_out_of_plane_user_value", "modifiers.source_reference"},
    "tee": {"modifiers.branch_header_sif_user_value", "modifiers.branch_branch_sif_user_value",
            "modifiers.source_reference"},
}
LEAF_KEYS = ({"value", "unit"}, {"x", "y", "z"}, {"x", "y", "z", "unit"})


def leaf_paths(value, prefix=""):
    if isinstance(value, dict) and set(value) not in LEAF_KEYS:
        for key, item in value.items():
            yield from leaf_paths(item, f"{prefix}.{key}" if prefix else key)
    else:
        yield prefix


def authoring(model):
    inputs = []
    collections = ["materials", "sections", "nodes", "pipe_segments", "components", "supports",
                   "reference_configurations", "load_cases", "combinations"]
    for collection in collections:
        for entity in model[collection]:
            person = PERSON_FIELDS.get(entity.get("kind"), set()) if collection == "components" else set()
            if collection == "load_cases":
                fields = [key for key in entity if key != "id"]
            else:
                fields = [path for path in leaf_paths({k: v for k, v in entity.items() if k != "id"})]
            inputs.append({"collection": collection, "id": entity["id"],
                           "fields": {path: ("person" if path in person else "agent_authorable") for path in fields}})
    for key in ("schema_version", "pressure_contract", "project", "analysis_status"):
        inputs.append({"collection": "document", "id": key, "fields": {key: "agent_authorable"}})
    bends = []
    for tag, corner, radius, k, sif_in, sif_out, incoming, outgoing, tangents, arc in BENDS:
        inc, out = member(incoming), member(outgoing)
        bends.append({
            "component": f"component:{tag}",
            "corner": {"position": {"x": num(corner[0]), "y": num(corner[1]), "z": num(corner[2])},
                       "node_id_while_drafting": f"node:{tag}-CORNER",
                       "note": "the corner node exists only before the bend is inserted; Insert bend at corner "
                               "removes it (bend_authoring.rs insert_at_corner)"},
            "straight_runs": [{"pipe": f"pipe:{incoming}", "before": [f"node:{inc[1]}", f"node:{tag}-CORNER"],
                               "after": [f"node:{inc[1]}", f"node:{tangents[0]}"]},
                              {"pipe": f"pipe:{outgoing}", "before": [f"node:{tag}-CORNER", f"node:{out[2]}"],
                               "after": [f"node:{tangents[1]}", f"node:{out[2]}"]}],
            "radius": q(num(radius), "m"),
            "creates": {"tangent_nodes": [f"node:{tangents[0]}", f"node:{tangents[1]}"], "arc_pipe": f"pipe:{arc}",
                        "component": f"component:{tag}"},
            "arc_y_reference": arc_y_reference(next(b for b in BENDS if b[0] == tag)),
            "person_inputs": {"flexibility_factor": q(k, "none"), "flexibility_source_reference": SOURCE,
                              "sif_in_plane_user_value": q(sif_in, "1"), "sif_out_of_plane_user_value": q(sif_out, "1"),
                              "sif_source_reference": SOURCE},
            "order": "insert before any support, load, region or reference names the corner or either straight "
                     "(OP-BEND-CORNER-ATTACHMENT-UNSUPPORTED)",
        })
    pending = [{"family": family, "in_tranche_1": state.startswith("in tranche 1"), "state": state, "row_lane": lane,
                "expected_until_then": expected, "entities": LEDGER_ENTITIES.get(family, [])}
               for family, state, lane, expected in LEDGER]
    return {
        "document_kind": "swbpipe.reference_system.authoring",
        "specification": SPEC,
        "model_document": MODEL_FILE,
        "model_document_version": MODEL_DOCUMENT_VERSION,
        "marks": {"person": "entered by the person, with its source: the bend k, the bend SIFs, the tee SIFs and "
                            "their sources (spec §11.3; OP-BEND-FLEXIBILITY-USER-ENTRY-REQUIRED)",
                  "agent_authorable": "every other authored input"},
        "inputs": inputs,
        "pending_inputs": [],
        "self_weight": {"cases": [f"load:{case}" for case, *_ in CASES], "pipe_refs": weighted_pipes(),
                        "gravity": dict(GRAVITY), "generator": "self-weight generator (self_weight.rs), captured in "
                                                              f"{CAPTURE_FILE}", "mark": "agent_authorable"},
        "bends": bends,
        "pending": pending,
    }


# -- key_results.json -------------------------------------------------------------------------------------------------
def key_results():
    return {
        "document_kind": "swbpipe.reference_system.key_results",
        "specification": SPEC,
        "rule": "an expected value enters only from a closed-form reference with an independent generator, an exact "
                "identity or a verified solve, frozen in validation/references/reference_system/ with its basis "
                "recorded (spec §1); each expected value here names that record",
        "results": [{"id": rid, "description": description, "cases": cases, "basis": basis, "spec_basis": spec_basis,
                     "expected": EXPECTED.get(rid), **({"check": CHECKS[rid]} if rid in CHECKS else {})}
                    for rid, description, cases, basis, spec_basis in KEY_RESULTS],
    }


# -- writing and checking ---------------------------------------------------------------------------------------------
def dumps(value):
    return json.dumps(value, indent=2, sort_keys=True, ensure_ascii=False, allow_nan=False) + "\n"


def generate():
    model = model_document()
    return {MODEL_FILE: dumps(model), AUTHORING_FILE: dumps(authoring(model)), KEY_RESULTS_FILE: dumps(key_results())}


def capture_self_weight(glue_dir: Path):
    model = base_model()
    payload = {"glue": str(glue_dir.resolve()), "model": model, "requests": weight_requests()}
    completed = subprocess.run(["node", str(HERE / CAPTURE_DRIVER)], input=json.dumps(payload), capture_output=True,
                               text=True, check=False)
    if completed.returncode != 0:
        raise SystemExit(f"capture failed: {completed.stderr.strip()}")
    outcomes = json.loads(completed.stdout)
    cases = []
    for request, outcome in zip(weight_requests(), outcomes):
        if outcome.get("status") != "ready" or outcome.get("diagnostics"):
            raise SystemExit(f"self-weight generation blocked for {request['case_id']}: {outcome.get('diagnostics')}")
        plan = outcome["plan"]
        if plan.get("warnings"):  # e.g. a component weight not applied: W would not be the spec's (§8.1)
            raise SystemExit(f"self-weight plan for {request['case_id']} has warnings: {plan['warnings']}")
        changes = plan["changes"]
        if changes[0]["change_kind"] != "create_load_case" or any(c["change_kind"] != "create_primitive_load"
                                                                   for c in changes[1:]):
            raise SystemExit("unexpected plan shape")
        cases.append({"request": {**request, "source_model_hash": plan["source_model_hash"]},
                      "scope_label": plan["scope_label"],
                      "case_record": json.loads(changes[0]["after"]),
                      "primitive_loads": [json.loads(c["after"]) for c in changes[1:]]})
    capture = {"document_kind": "swbpipe.reference_system.self_weight_capture",
               "engine": "core/loads/self_weight_wasm generate_self_weight_plan_json (self_weight.rs), wasm32 "
                         "--features wasm, wasm-bindgen --target nodejs",
               "driver": f"fixtures/reference_system/{CAPTURE_DRIVER}",
               "weight_basis": weight_basis(model), "cases": cases}
    (HERE / CAPTURE_FILE).write_text(dumps(capture), encoding="utf-8")


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--check", action="store_true", help="compare with the committed files; write nothing")
    parser.add_argument("--capture-self-weight", type=Path, metavar="GLUE_DIR",
                        help="refresh the self-weight capture from the product engine, then write")
    args = parser.parse_args(argv)
    if args.capture_self_weight and args.check:
        parser.error("--capture-self-weight writes; it cannot be combined with --check")
    if args.capture_self_weight:
        capture_self_weight(args.capture_self_weight)
    outputs = generate()
    if args.check:
        differing = [name for name, text in sorted(outputs.items())
                     if not (HERE / name).is_file() or (HERE / name).read_text(encoding="utf-8") != text]
        for name in differing:
            print(f"differs: {name}")
        return 1 if differing else 0
    for name, text in outputs.items():
        (HERE / name).write_text(text, encoding="utf-8")
    return 0


if __name__ == "__main__":
    sys.exit(main())
