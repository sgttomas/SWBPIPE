"""Invented models, spectra and requests of the W1 recovery family.

Every number here is invented and is a binary64 value written exactly in
decimal (dyadic fractions and integers). The driver asserts this, publishes
each input as its shortest round-trip decimal, its hex form and its exact
rational, and computes with the exact rational.

The model form is f1_modal/mass_discrete's neutral form (read unchanged by
its Model, form_mass and form_stiffness), with additions that f1 ignores: `id`
on each support, and `poisson_ratio` on the material, from which the driver
derives the shear modulus f1 reads.

One model family, all on the EX basis:

  N1 (0, 0, 0) ............ anchor A1 (all six DOFs)
  N2 (2, 0, 0) ............ branch junction (tee legs as straights; no body)
  N3 (4, 0, 0) ............ upstream end of the reducer-like chord
  N4 = N3 + (3/16, 1/16, -3/64)   downstream end of the chord, |chord| = 13/64 m
  N5 = N4 + (3, 0, 0) ...... guide G1 (uy, uz)
  N6 = N2 + 2.5 (0, 3, 4)/5 = (2, 1.5, 2) .. rest R1 (uz); valve V1 node
  N7 = N6 + 0.625 (0, 3, 4)/5 = (2, 1.875, 2.5)
  N8 = N7 + 1.875 (0, 3, 4)/5 = (2, 3, 4) ... anchor A2 (all six DOFs)

  members H1 N1-N2, H2 N2-N3 (S1); RD N3-N4 (S1, the inclined chord);
          D1 N4-N5 (S2); B1 N2-N6, BV N6-N7 (valve body), B2 N7-N8 (S1)

The chord's transverse offset (0, 1/16, -3/64), of length 5/64 m, is the
reducer's geometric eccentricity: the positions of its nodes. It is not a
body offset. The valve V1 at N6 maps BV and, where eligible, replaces BV's
distributed mass with a point mass W/a at its centre of gravity N6 + r.
"""

A6 = ["ux", "uy", "uz", "rx", "ry", "rz"]
ZETA = 0.05

ACC = 9.8125  # weight-to-mass acceleration, an invented request input (m/s^2)

# State-selected modulus: an explicit input standing for the modulus the named
# state case resolves (no state solver here). Homogeneous isotropic: E and
# Poisson's ratio nu = 1/4 are the inputs; the driver derives
# G = E / (2 (1 + nu)) exactly and asserts it equals the binary64 value
# 76000000000 (STATED_SHEAR_MODULUS_PA). f1's Model reads the derived G.
MATERIALS = {"steel_state": {"elastic_modulus_Pa": 190000000000, "poisson_ratio": 0.25,
                             "density_kg_per_m3": 7850}}
STATED_SHEAR_MODULUS_PA = 76000000000

S1 = {"outside_diameter_m": 0.2734375, "wall_thickness_m": 0.009765625,
      "mill_tolerance_m": 0.0009765625, "corrosion_allowance_m": 0.0029296875,
      "contents_density_kg_per_m3": 875, "insulation_thickness_m": 0.0625,
      "insulation_density_kg_per_m3": 160}
S2 = {"outside_diameter_m": 0.16796875, "wall_thickness_m": 0.0068359375,
      "mill_tolerance_m": 0.0008544921875, "corrosion_allowance_m": 0.001953125,
      "contents_density_kg_per_m3": 875, "insulation_thickness_m": 0.05078125,
      "insulation_density_kg_per_m3": 160}

NODES_ECC = [("N1", [0, 0, 0]), ("N2", [2, 0, 0]), ("N3", [4, 0, 0]),
             ("N4", [4.1875, 0.0625, -0.046875]), ("N5", [7.1875, 0.0625, -0.046875]),
             ("N6", [2, 1.5, 2]), ("N7", [2, 1.875, 2.5]), ("N8", [2, 3, 4])]
# Concentric control: the chord along X with the same length 13/64 m.
NODES_CONC = [("N1", [0, 0, 0]), ("N2", [2, 0, 0]), ("N3", [4, 0, 0]),
              ("N4", [4.203125, 0, 0]), ("N5", [7.203125, 0, 0]),
              ("N6", [2, 1.5, 2]), ("N7", [2, 1.875, 2.5]), ("N8", [2, 3, 4])]

# (id, i, j, section, y_reference). Every guide is chosen so that each member
# frame is rational: the chord RD, x = (12, 4, -3)/13, takes y_reference
# (0, 3, 4), already normal to x, so y = (0, 3, 4)/5 and z = (25, -48, 36)/65;
# the branch, x = (0, 3, 4)/5, takes (0, 0, 1), so y = (0, -4, 3)/5 and
# z = (1, 0, 0). Element matrices do not depend on the frame (circular
# section); the frame only defines the published member-axis components.
MEMBERS = [("H1", "N1", "N2", "S1", [0, 0, 1]), ("H2", "N2", "N3", "S1", [0, 0, 1]),
           ("RD", "N3", "N4", "S1", [0, 3, 4]), ("D1", "N4", "N5", "S2", [0, 0, 1]),
           ("B1", "N2", "N6", "S1", [0, 0, 1]), ("BV", "N6", "N7", "S1", [0, 0, 1]),
           ("B2", "N7", "N8", "S1", [0, 0, 1])]

SUPPORTS = [("A1", "N1", "anchor", A6), ("A2", "N8", "anchor", A6),
            ("G1", "N5", "guide", ["uy", "uz"]), ("R1", "N6", "rest", ["uz"])]

VALVE = {"id": "V1", "kind": "valve", "node": "N6", "maps_member": "BV",
         "solver_consumption": "mechanics_geometry_only", "weight_N": 6000,
         "center_of_gravity_m": [0.25, -0.125, 0.375], "stiffness_scaling": 4}

REDUCER_BODY = {"id": "RC1", "kind": "reducer", "node": "N3", "maps_member": "RD",
                "solver_consumption": "mechanics_geometry_only", "weight_N": 1200,
                "center_of_gravity_m": [0.0625, 0, 0.03125], "stiffness_scaling": None}


def model(nodes=NODES_ECC, s1=None, s2=None, components=None):
    return {
        "units": {"length": "m", "mass": "kg", "force": "N", "stress": "Pa", "time": "s"},
        "materials": MATERIALS,
        "sections": {"S1": dict(s1 or S1), "S2": dict(s2 or S2)},
        "nodes": [{"id": n, "xyz": list(x)} for n, x in nodes],
        "members": [{"id": m, "kind": "straight", "i": i, "j": j, "section": s, "material": "steel_state",
                     "y_reference": list(y)} for m, i, j, s, y in MEMBERS],
        "components": [dict(c) for c in (components if components is not None else [VALVE])],
        "supports": [{"id": s, "node": n, "kind": k, "restrained": list(r)} for s, n, k, r in SUPPORTS],
        "constant_effort_supports": [],
        "springs": [],
        "weight_to_mass_acceleration_m_per_s2": ACC,
    }


def spectrum(sid, abscissa, ax, ord_, points):
    return {"id": sid, "abscissa": abscissa, "interpolation": {"abscissa": ax, "ordinate": ord_},
            "ordinate": "pseudo-spectral acceleration, m/s^2", "damping_ratio": ZETA,
            "points": [list(p) for p in points]}


# Three invented tables, one per direction, each with a different rule.
SP_HX = spectrum("SP-HX", "frequency_Hz", "logarithmic", "logarithmic",
                 [(0.25, 1.5), (2, 4), (8, 9.5), (25, 9.5), (60, 5), (200, 3), (8192, 2.5)])
SP_HY = spectrum("SP-HY", "frequency_Hz", "linear", "linear",
                 [(0.25, 1.25), (3, 4.5), (10, 8.75), (18, 10.5), (45, 6.25), (150, 3.5), (8192, 2.75)])
SP_VZ = spectrum("SP-VZ", "period_s", "linear", "logarithmic",
                 [(0.0001220703125, 2), (0.009765625, 2.25), (0.03125, 5.5), (0.125, 6.75), (0.5, 3.5),
                  (4, 0.75)])
SPECTRA = [SP_HX, SP_HY, SP_VZ]

DIRECTIONS = [("X", "SP-HX", 1), ("Y", "SP-HY", 1), ("Z", "SP-VZ", 0.75)]

# The rigid frequency and the extraction count are chosen from this family's
# own frequencies (README, "Request"): f_r lies in a wide gap so that a
# truncated set of modes is periodic and the extracted modes above f_r are
# rigid; the extracted set reaches f_r (DEL-04-10 REQ-001, extraction premise).
RIGID_FREQUENCY_HZ = 50
EXTRACTED_MODES = 10


def request(rid, modal="CQC", directional="SRSS", mm=True, rule="SRSS", fr=RIGID_FREQUENCY_HZ,
            n_extract=EXTRACTED_MODES):
    return {"id": rid,
            "directions": [{"direction": d, "spectrum": s, "scale_factor": f} for d, s, f in DIRECTIONS],
            "modal_combination": modal, "directional_combination": directional,
            "missing_mass": ({"applied": True, "rigid_frequency_Hz": fr, "rigid_periodic_combination": rule}
                             if mm else {"applied": False, "rigid_frequency_Hz": None,
                                         "rigid_periodic_combination": None}),
            "extracted_modes": n_extract}


def _sec(base, **kw):
    out = dict(base)
    out.update(kw)
    return out


VARIANTS = [
    {"id": "W1-BASE", "basis": "EX", "model": model(),
     "purpose": "Branched EX frame: inclined eccentric chord, eligible valve body with an offset centre of "
                "gravity next to a restrained DOF, contents and insulation, positive mill and corrosion, "
                "state-selected modulus. Full method grid."},
    {"id": "W1-MILL", "basis": "EX", "model": model(s1=_sec(S1, mill_tolerance_m=0.001953125)),
     "purpose": "S1 mill tolerance raised from 1/1024 m to 2/1024 m, corrosion unchanged."},
    {"id": "W1-CORR", "basis": "EX", "model": model(s1=_sec(S1, corrosion_allowance_m=0.00390625)),
     "purpose": "S1 corrosion allowance raised from 3/1024 m to 4/1024 m, mill unchanged. On EX only "
                "c_m + c_a enters (the pressure bore), so this equals W1-MILL exactly."},
    {"id": "W1-BODY-INELIGIBLE", "basis": "EX",
     "model": model(components=[_sec(VALVE, center_of_gravity_m=None)]),
     "purpose": "Valve without a centre of gravity: not eligible, BV keeps its pipe mass, the stiffness "
                "scale still applies, so only the mass differs from W1-BASE."},
    {"id": "W1-BODY-COG0", "basis": "EX",
     "model": model(components=[_sec(VALVE, center_of_gravity_m=[0, 0, 0])]),
     "purpose": "Eligible valve with its centre of gravity at its node: no translation-rotation coupling."},
    {"id": "W1-CONCENTRIC", "basis": "EX", "model": model(nodes=NODES_CONC),
     "purpose": "Geometric-offset control: the chord along X, same length 13/64 m; the valve body and its "
                "offset are unchanged."},
    {"id": "W1-REDUCER-BODY", "basis": "EX", "model": model(components=[VALVE, REDUCER_BODY]),
     "purpose": "An eligible reducer body on RD whose centre of gravity (1/16, 0, 1/32) is not on the chord: "
                "RD's distributed mass is absent, its stiffness unscaled; the CoG offset is not the "
                "eccentricity."},
]

# Every modal rule x every rigid treatment x every directional rule, on W1-BASE.
MODAL_RULES = ["SRSS", "CQC", "GROUPING", "TEN_PERCENT", "ABSOLUTE_SUM"]
RIGID_RULES = ["SRSS", "ABSOLUTE_SUM", "OFF"]
DIRECTIONAL_RULES = ["SRSS", "100-40-40", "ABSOLUTE_SUM"]
