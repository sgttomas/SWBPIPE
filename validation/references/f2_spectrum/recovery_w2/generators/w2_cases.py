"""Invented models, spectra and requests for the W2 recovery references.

Every number is invented. Every input number is a binary64 value written
exactly in decimal (dyadic fractions and integers); the driver asserts this and
computes with the exact rational. The connector frame is derived exactly from
an integer guide (below); no rounded unit vector is an input.

Geometry (planar line in the global X-Y plane, all coordinates dyadic):

  A (0, 0, 0)            anchor
  B (1.5, 0, 0)          straight P1 A-B along +X
  C (1.875, 0.375, 0)    realized 90-degree arc BEND1 B-C, R = 0.375, centre (1.5, 0.375, 0)
  D (1.875, 1.375, 0)    face pipe P2 C-D along +Y
  E (1.875, 1.875, 0)    replaced span D-E: objective connector J1 (no pipe)
  F (1.875, 3.125, 0)    face pipe P3 E-F along +Y (1.25 m, unequal to P2 so no mirror symmetry); anchor
"""

SECTION = {
    "s3": {
        "outside_diameter_m": "0.171875",
        "wall_thickness_m": "0.0078125",
        "mill_tolerance_m": "0",
        "corrosion_allowance_m": "0",
        "contents_density_kg_per_m3": "1000",
        "insulation_thickness_m": None,
        "insulation_density_kg_per_m3": None,
    }
}
MATERIAL = {
    "steel": {
        "elastic_modulus_Pa": "200000000000",
        "shear_modulus_Pa": "78125000000",
        "density_kg_per_m3": "7850",
    }
}

NODES = {
    "A": ["0", "0", "0"],
    "B": ["1.5", "0", "0"],
    "C": ["1.875", "0.375", "0"],
    "D": ["1.875", "1.375", "0"],
    "E": ["1.875", "1.875", "0"],
    "F": ["1.875", "3.125", "0"],
}

PIPE_MEMBERS = {
    "P1": {"kind": "straight", "i": "A", "j": "B", "y_reference": ["0", "0", "1"]},
    "BEND1": {"kind": "arc", "i": "B", "j": "C", "radius_m": "0.375", "flexibility_factor": "4.25",
              "y_reference": ["1", "-1", "0"]},
    "P2": {"kind": "straight", "i": "C", "j": "D", "y_reference": ["0", "0", "1"]},
    "P3": {"kind": "straight", "i": "E", "j": "F", "y_reference": ["0", "0", "1"]},
}

# Objective connector J1 replacing the span D-E. Attachment offsets are zero (a_i = a_j = 0).
# Frame: x along r = x_E - x_D; y from the integer guide (4, 0, 3) made normal to x and
# normalized in exact rational arithmetic (here it is already normal: y = (4/5, 0, 3/5));
# z = x cross y. The work matrix H is diagonal and positive in the order
# (axial, lateral y, lateral z, torsion, bending y, bending z); translation scale Ls = 1 m,
# rotation scale 1 rad, so K = H / (d_a d_b) with d = (Ls, Ls, Ls, 1, 1, 1) has the same numerical entries as H here; H has work units N*m.
CONNECTOR = {
    "id": "J1",
    "i": "D",
    "j": "E",
    "offset_i_global_m": ["0", "0", "0"],
    "offset_j_global_m": ["0", "0", "0"],
    "y_guide_integer": [4, 0, 3],
    "translation_scale_m": "1",
    "H_diagonal": ["150000", "400000", "250000", "500000", "1500", "1000"],
    "H_units": ["N*m", "N*m", "N*m", "N*m", "N*m", "N*m"],
    "reference_load_q_ref": ["0", "0", "0", "0", "0", "0"],
}

# Two linear rods between the joint's ends, rigidly offset by +/- rho along the connector's y axis
# at both ends (so each rod is parallel to the connector x axis), reference extension 0.
ROD_OFFSET_M = "0.1875"
ROD_STIFFNESS_N_PER_M = "120000000"
RODS = [
    {"id": "R1", "i": "D", "j": "E", "offset_sign": 1},
    {"id": "R2", "i": "D", "j": "E", "offset_sign": -1},
]

# Variants: which nodes, members, connector, rods and anchors.
VARIANTS = {
    "V-ARC": {
        "nodes": ["A", "B", "C", "D"],
        "members": ["P1", "BEND1", "P2"],
        "connector": False,
        "rods": False,
        "anchors": ["A", "D"],
        "purpose": "arc only: the realized bend between massive straights, anchored at A and D",
    },
    "V-JOINT-0": {
        "nodes": ["C", "D", "E", "F"],
        "members": ["P2", "P3"],
        "connector": True,
        "rods": False,
        "anchors": ["C", "F"],
        "purpose": "joint only, rods absent: massive face pipes and the massless connector, anchored at C and F",
    },
    "V-JOINT-R": {
        "nodes": ["C", "D", "E", "F"],
        "members": ["P2", "P3"],
        "connector": True,
        "rods": True,
        "anchors": ["C", "F"],
        "purpose": "joint only, rods present",
    },
    "V-COMB-0": {
        "nodes": ["A", "B", "C", "D", "E", "F"],
        "members": ["P1", "BEND1", "P2", "P3"],
        "connector": True,
        "rods": False,
        "anchors": ["A", "F"],
        "purpose": "combined, rods absent",
    },
    "V-COMB-R": {
        "nodes": ["A", "B", "C", "D", "E", "F"],
        "members": ["P1", "BEND1", "P2", "P3"],
        "connector": True,
        "rods": True,
        "anchors": ["A", "F"],
        "purpose": "combined, rods present (the full W2 model)",
    },
}

# ---------------------------------------------------------------------------- spectra (invented)
# Pseudo-spectral accelerations in m/s^2. Every abscissa and ordinate is dyadic (exact binary64).
ZETA = "0.05"  # binary64 0.05 = 3602879701896397/72057594037927936, taken exactly
SPECTRA = {
    "S-W2-LL": {"abscissa": "frequency_Hz", "x_interp": "linear", "y_interp": "linear",
                "points": [["1", "2"], ["8", "6"], ["64", "9"], ["256", "5"], ["2048", "3"]]},
    "S-W2-GG": {"abscissa": "frequency_Hz", "x_interp": "log", "y_interp": "log",
                "points": [["1", "2"], ["8", "6"], ["64", "9"], ["256", "5"], ["2048", "3"]]},
    "S-W2-PLG": {"abscissa": "period_s", "x_interp": "linear", "y_interp": "log",
                 "points": [["0.00048828125", "3"], ["0.00390625", "5"], ["0.015625", "9"], ["0.125", "6"],
                            ["1", "2"]]},
}
DIRECTIONS = [["X", "S-W2-LL", "1"], ["Y", "S-W2-GG", "0.75"], ["Z", "S-W2-PLG", "0.5"]]

# Per variant: the extraction cut-off f_c (inclusive; it lies in a gap of the spectrum) and the rigid
# frequency f_r. Base rules: CQC, rigid SRSS, directional SRSS. The mutation grid covers every modal
# rule x {SRSS, ABSOLUTE_SUM, OFF} x {SRSS, 100-40-40, ABSOLUTE_SUM}.
REQUESTS = {
    "V-ARC": {"cutoff_Hz": "560", "rigid_frequency_Hz": "400"},
    "V-JOINT-0": {"cutoff_Hz": "768", "rigid_frequency_Hz": "500"},
    "V-JOINT-R": {"cutoff_Hz": "768", "rigid_frequency_Hz": "500"},
    "V-COMB-0": {"cutoff_Hz": "128", "rigid_frequency_Hz": "72"},
    "V-COMB-R": {"cutoff_Hz": "128", "rigid_frequency_Hz": "72"},
}
BASE_RULES = {"modal": "CQC", "rigid": "SRSS", "directional": "SRSS"}

# ---------------------------------------------------------------------------- controls (invented)
# Held static state for the fixed-state exclusion control: joint pressure p acting on the connector's
# effective area Ae (an axial thrust pair on the joint faces, a load), a nonzero connector reference
# load q_ref, and a held static displacement d0 of the joint's two end nodes. None of these is
# dynamic excitation or stiffness; the control shows their affine terms vanish in the increment.
HELD_STATE = {
    "pressure_Pa": "1500000",
    "effective_area_m2": "0.03125",
    "q_ref": ["-0.001953125", "0.0009765625", "0", "0", "0.0078125", "0"],
    "d0_joint_ends": ["0.00048828125", "-0.0009765625", "0.000244140625", "0.001953125", "0", "-0.00390625",
                      "-0.000732421875", "0.0029296875", "0", "0", "0.0009765625", "0.001953125"],
}
