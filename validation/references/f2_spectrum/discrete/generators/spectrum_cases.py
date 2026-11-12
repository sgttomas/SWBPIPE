"""Invented models, spectra and requests for the f2_spectrum discrete references.

Every number is invented. Every input number is a binary64 value; the
generator publishes each one as its shortest round-trip decimal, its hex form
and its exact rational, and computes with the exact rational.

Model form (a superset of the f1_modal mass_discrete neutral form, so that
f1's Model, form_mass and form_stiffness read it unchanged):

  nodes        [{id, xyz}]                                    m
  materials    {name: {elastic_modulus_Pa, shear_modulus_Pa, density_kg_per_m3}}
  sections     {name: {outside_diameter_m, wall_thickness_m, mill_tolerance_m,
                       corrosion_allowance_m, contents_density_kg_per_m3,
                       insulation_thickness_m, insulation_density_kg_per_m3}}
  members      [{id, kind: straight, i, j, section, material, y_reference}]
  supports     [{id, node, kind, restrained: [dof names]}]    rigid, axis-aligned
  springs      [{id, node, dof, stiffness}]                   linear, on a global DOF
  skew_springs [{id, node, direction: [dx, dy, dz], stiffness}]   k d d^T / (d^T d)
  snubbers     [{id, node, direction: dof name | [dx, dy, dz], stiffness: null | k}]
               null = rigid; a rigid skew snubber is the exact constraint d^T u = 0
  components [], constant_effort_supports [], weight_to_mass_acceleration_m_per_s2 null

Spectrum form:
  {id, abscissa: "frequency_Hz" | "period_s",
   interpolation: {abscissa: linear | logarithmic, ordinate: linear | logarithmic},
   ordinate: "pseudo-spectral acceleration, m/s^2", damping_ratio,
   points: [[abscissa, ordinate], ...]}   abscissae strictly increasing

Request form:
  {id, directions: [{direction: X|Y|Z, spectrum, scale_factor}],
   modal_combination: SRSS | CQC | GROUPING,
   directional_combination: SRSS | 100-40-40,
   missing_mass: {applied: bool, rigid_frequency_Hz, rigid_periodic_combination: SRSS | ABSOLUTE_SUM},
   extracted_modes: n (the lowest n modes; every extracted cluster complete)}
"""

A6 = ["ux", "uy", "uz", "rx", "ry", "rz"]
ZETA = 0.05

MATERIALS = {"steel": {"elastic_modulus_Pa": 200000000000, "shear_modulus_Pa": 78125000000,
                       "density_kg_per_m3": 7850}}
SECTIONS = {"s1": {"outside_diameter_m": 0.21875, "wall_thickness_m": 0.0078125,
                   "mill_tolerance_m": 0.0009765625, "corrosion_allowance_m": 0.001953125,
                   "contents_density_kg_per_m3": 1000, "insulation_thickness_m": None,
                   "insulation_density_kg_per_m3": None}}


def model(nodes, members, supports, springs=(), skew_springs=(), snubbers=(), sections=None, section="s1"):
    return {
        "units": {"length": "m", "mass": "kg", "force": "N", "stress": "Pa", "time": "s"},
        "materials": MATERIALS,
        "sections": sections or SECTIONS,
        "nodes": [{"id": n, "xyz": list(x)} for n, x in nodes],
        "members": [{"id": m, "kind": "straight", "i": i, "j": j, "section": section, "material": "steel",
                     "y_reference": list(y)} for m, i, j, y in members],
        "components": [],
        "supports": [{"id": s, "node": n, "kind": k, "restrained": list(r)} for s, n, k, r in supports],
        "constant_effort_supports": [],
        "springs": [{"id": s, "node": n, "dof": d, "stiffness": k} for s, n, d, k in springs],
        "skew_springs": [{"id": s, "node": n, "direction": list(d), "stiffness": k} for s, n, d, k in skew_springs],
        "snubbers": [{"id": s, "node": n, "direction": d if isinstance(d, str) else list(d), "stiffness": k}
                     for s, n, d, k in snubbers],
        "weight_to_mass_acceleration_m_per_s2": None,
    }


def spectrum(sid, abscissa, ax, ord_, points):
    return {"id": sid, "abscissa": abscissa, "interpolation": {"abscissa": ax, "ordinate": ord_},
            "ordinate": "pseudo-spectral acceleration, m/s^2", "damping_ratio": ZETA,
            "points": [list(p) for p in points]}


def request(rid, dirs, modal, directional, mm, n_extract):
    return {"id": rid,
            "directions": [{"direction": d, "spectrum": s, "scale_factor": f} for d, s, f in dirs],
            "modal_combination": modal, "directional_combination": directional,
            "missing_mass": mm, "extracted_modes": n_extract}


def mm_on(fr, rule):
    return {"applied": True, "rigid_frequency_Hz": fr, "rigid_periodic_combination": rule}


MM_OFF = {"applied": False, "rigid_frequency_Hz": None, "rigid_periodic_combination": None}

# ------------------------------------------------------------------ spectra
# Four interpolation rules: lin-lin (S-LL), log-log (S-GG), period abscissa with
# linear abscissa and logarithmic ordinate (S-PLG), log abscissa and linear
# ordinate (S-GL).
S_LL = spectrum("S-LL", "frequency_Hz", "linear", "linear",
                [(0.5, 1.25), (2, 3.5), (6, 8.75), (12, 11.5), (30, 7.25), (80, 4.5), (1000, 3)])
S_GG = spectrum("S-GG", "frequency_Hz", "logarithmic", "logarithmic",
                [(0.5, 1), (3, 4.5), (9, 10.25), (20, 9), (50, 5.5), (160, 3.75), (1000, 3.25)])
S_PLG = spectrum("S-PLG", "period_s", "linear", "logarithmic",
                 [(0.001, 2.5), (0.0125, 2.75), (0.03125, 4.25), (0.0625, 6.5), (0.125, 7.75), (0.25, 6),
                  (1, 2.25), (2, 1.125)])
S_GL = spectrum("S-GL", "frequency_Hz", "logarithmic", "linear",
                [(0.5, 0.75), (4, 5), (10, 9.5), (13, 6.25), (25, 4), (100, 2.5), (2000, 2)])
S_FLAT = spectrum("S-FLAT", "frequency_Hz", "linear", "linear",
                  [(1, 2), (20, 6), (50, 4.5), (100, 4.5), (20000, 4.5)])

# ------------------------------------------------------------------ models
ONE_FREE = model([("N1", [0, 0, 0]), ("N2", [4, 0, 0])], [("P1", "N1", "N2", [0, 0, 1])],
                 [("A1", "N1", "anchor", A6)])
ONE_GUIDED = model([("N1", [0, 0, 0]), ("N2", [4, 0, 0])], [("P1", "N1", "N2", [0, 0, 1])],
                   [("A1", "N1", "anchor", A6), ("G1", "N2", "guide", ["uz", "ry"])])
TWO_SPRING = model([("N1", [0, 0, 0]), ("N2", [2, 0, 0]), ("N3", [4, 0, 0])],
                   [("P1", "N1", "N2", [0, 0, 1]), ("P2", "N2", "N3", [0, 0, 1])],
                   [("A1", "N1", "anchor", A6)], springs=[("S1", "N3", "uy", 262144)])
TWO_PLAIN = model([("N1", [0, 0, 0]), ("N2", [2, 0, 0]), ("N3", [4, 0, 0])],
                  [("P1", "N1", "N2", [0, 0, 1]), ("P2", "N2", "N3", [0, 0, 1])],
                  [("A1", "N1", "anchor", A6)])
CANT3D = model([("N1", [0, 0, 0]), ("N2", [2, 0, 0]), ("N3", [2, 1.5, 0]), ("N4", [2, 1.5, 1.25]),
                ("N5", [3, 1.5, 1.25])],
               [("P1", "N1", "N2", [0, 0, 1]), ("P2", "N2", "N3", [0, 0, 1]), ("P3", "N3", "N4", [1, 0, 0]),
                ("P4", "N4", "N5", [0, 0, 1])],
               [("A1", "N1", "anchor", A6)])
PAIR = model([("N1", [0, 0, 0]), ("N2", [0, 0, 1.5]), ("N3", [0, 0, 3])],
             [("P1", "N1", "N2", [1, 0, 0]), ("P2", "N2", "N3", [1, 0, 0])],
             [("A1", "N1", "anchor", A6)],
             skew_springs=[("K1", "N3", [3, 4, 0], 131072), ("K2", "N3", [-4, 3, 0], 196608)])
STIFF = model([("N1", [0, 0, 0]), ("N2", [0, 0, 0.25]), ("N3", [0, 0, 0.5])],
              [("P1", "N1", "N2", [1, 0, 0]), ("P2", "N2", "N3", [1, 0, 0])],
              [("A1", "N1", "anchor", A6)])
L_NODES = [("N1", [0, 0, 0]), ("N2", [2, 0, 0]), ("N3", [4, 0, 0]), ("N4", [4, 1.5, 0]), ("N5", [4, 3, 0])]
L_MEMBERS = [("P1", "N1", "N2", [0, 0, 1]), ("P2", "N2", "N3", [0, 0, 1]), ("P3", "N3", "N4", [0, 0, 1]),
             ("P4", "N4", "N5", [0, 0, 1])]
L_SUPPORTS = [("A1", "N1", "anchor", A6), ("A2", "N5", "anchor", A6)]
L_MODEL = model(L_NODES, L_MEMBERS, L_SUPPORTS)
L_SNUB_RIGID = model(L_NODES, L_MEMBERS, L_SUPPORTS,
                     snubbers=[("SN1", "N2", "uz", None), ("SN2", "N4", [3, 4, 0], None)])
L_SNUB_FINITE = model(L_NODES, L_MEMBERS, L_SUPPORTS,
                      snubbers=[("SN1", "N2", "uz", 1048576), ("SN2", "N4", [3, 4, 0], 524288)])

# ------------------------------------------------------------------ benchmark-shaped pipes (C8, C9)
# Slender straight pipe, 15 m, metal only: OD 60.3 mm, wall 3.91 mm (binary64 values of 0.0603 and 0.00391).
SECTIONS_S2 = {"s2": {"outside_diameter_m": 0.0603, "wall_thickness_m": 0.00391, "mill_tolerance_m": 0,
                      "corrosion_allowance_m": 0, "contents_density_kg_per_m3": None,
                      "insulation_thickness_m": None, "insulation_density_kg_per_m3": None}}


def straight_pipe(n, coords, supports, springs=(), yref=(0, 0, 1)):
    """n equal members from A to B; intermediate nodes M1 .. M(n-1)."""
    ids = ["A"] + [f"M{k}" for k in range(1, n)] + ["B"]
    return model([(ids[k], coords(k)) for k in range(n + 1)],
                 [(f"P{k + 1}", ids[k], ids[k + 1], yref) for k in range(n)],
                 supports, springs=springs, sections=SECTIONS_S2, section="s2")


C8_SUPPORTS = [("SA", "A", "anchor", A6), ("SB", "B", "directional restraint", ["uz"])]


def c8_model(n):
    return straight_pipe(n, lambda k: [15 * k / n, 0, 0], C8_SUPPORTS)


def diagonal_coordinate(k, n=8):
    """The binary64 nearest to 15 k / (n sqrt 3), correctly rounded through 60-digit decimal arithmetic."""
    from decimal import Decimal, localcontext
    with localcontext() as ctx:
        ctx.prec = 60
        return float(Decimal(15 * k) / (n * Decimal(3).sqrt()))


def c9_model(k_tip):
    return straight_pipe(8, lambda k: [diagonal_coordinate(k)] * 3, [("SA", "A", "anchor", A6)],
                         springs=[("S1", "B", "ux", k_tip)])


S_C8_FLAT = spectrum("S-C8-FLAT", "frequency_Hz", "linear", "linear", [(0.125, 2.5), (1000, 2.5)])
S_C8_TWO_LL = spectrum("S-C8-TWO-LL", "frequency_Hz", "linear", "linear", [(0.125, 1.5), (2, 4.5), (16, 2.25)])
S_C8_TWO_GG = spectrum("S-C8-TWO-GG", "frequency_Hz", "logarithmic", "logarithmic",
                       [(0.125, 1.5), (2, 4.5), (16, 2.25)])
S_C8_TRUNC = spectrum("S-C8-TRUNC", "frequency_Hz", "linear", "linear", [(0.125, 1.25), (0.75, 3.5), (1000, 3.5)])
S_C9_FLAT = spectrum("S-C9-FLAT", "frequency_Hz", "linear", "linear", [(0.125, 2.5), (1000, 2.5)])
ALL_MODAL = ["SRSS", "CQC", "GROUPING", "ABSOLUTE_SUM", "PAIRWISE_TEN_PERCENT"]

# W2 and W3: the C7 model with one weak skewed tip spring, which splits each repeated pair into two distinct
# modes whose relative eigenvalue gap (about 2^-22) lies inside delta_c = 2^-20.
K_W = 0.0625
CLOSE = model([("N1", [0, 0, 0]), ("N2", [2, 0, 0]), ("N3", [4, 0, 0])],
              [("P1", "N1", "N2", [0, 0, 1]), ("P2", "N2", "N3", [0, 0, 1])],
              [("A1", "N1", "anchor", A6)], skew_springs=[("K1", "N3", [0, 3, 4], K_W)])
FR_W3 = 9.5395413

XYZ3 = [("X", "S-LL", 1), ("Y", "S-GG", 1), ("Z", "S-PLG", 0.75)]


def cases():
    return [
        {"id": "C1-ONE-ELEMENT",
         "serves": "AC-002: the one-element consistent-mass and restrained-mass witnesses (REQ-002, REQ-004)",
         "purpose": "one straight member fixed at N1, tip N2 free: (a) axial response to motion along the "
                    "member, with and without the correction; (b) the restrained-mass witness, every mode "
                    "rigid, in X, Y and Z",
         "model": ONE_FREE, "spectra": [S_LL],
         "requests": [
             request("C1A-AXIAL", [("X", "S-LL", 1)], "SRSS", "SRSS", MM_OFF, 6),
             request("C1A-AXIAL-MM-ALL-PERIODIC", [("X", "S-LL", 1)], "SRSS", "SRSS", mm_on(320, "SRSS"), 6),
             request("C1B-RESTRAINED-MASS", [("X", "S-LL", 1), ("Y", "S-LL", 0.5), ("Z", "S-LL", 1.5)],
                     "SRSS", "SRSS", mm_on(4, "ABSOLUTE_SUM"), 6),
         ]},
        {"id": "C1C-ONE-ELEMENT-TRANSVERSE",
         "serves": "AC-002: the one-element witness in transverse motion with its bending modes; an extracted "
                   "rigid mode entering the static solve (REQ-002, REQ-004)",
         "purpose": "the member of C1 with its tip guided against Z translation and rotation about Y, so that "
                    "the bending modes are simple; motion along Y",
         "model": ONE_GUIDED, "spectra": [S_LL],
         "requests": [request("C1C-TRANSVERSE", [("Y", "S-LL", 1)], "SRSS", "SRSS", mm_on(40, "SRSS"), 4)]},
        {"id": "C2-GROUND-SPRING",
         "serves": "AC-002: a ground-spring reaction beside a rigid-end reaction, in equilibrium (REQ-002, REQ-004)",
         "purpose": "two members fixed at N1, a linear spring to ground at N3 along Y",
         "model": TWO_SPRING, "spectra": [S_GG],
         "requests": [request("C2-SPRING", [("Y", "S-GG", 1)], "SRSS", "SRSS", mm_on(33, "SRSS"), 4)]},
        {"id": "C3-CANTILEVER-3D",
         "serves": "AC-002: a multi-degree-of-freedom model's modal responses, each modal combination, the 100-40-40 "
                   "and SRSS directional combinations, the rigid response with extracted rigid modes, the "
                   "correction off with its disclosure; mutation data (REQ-002 to REQ-004)",
         "purpose": "four members along X, Y, Z and X, anchored at N1; every mode simple",
         "model": CANT3D, "spectra": [S_LL, S_GG, S_PLG],
         "mutations": True,
         "requests": [request("C3-BASE", XYZ3, "CQC", "SRSS", mm_on(70, "SRSS"), 8)]},
        {"id": "C4-CLOSE-PAIR",
         "serves": "AC-002: CQC on a closely spaced pair with opposite-sign contributions; mutation data (REQ-003)",
         "purpose": "vertical cantilever with two skewed springs of unequal stiffness at its tip, along (3,4,0) "
                    "and (-4,3,0): the first bending pair splits by a few percent and both modes are excited "
                    "by X and by Y",
         "model": PAIR, "spectra": [S_GL],
         "mutations": True,
         "requests": [request("C4-BASE", [("X", "S-GL", 1), ("Y", "S-GL", 0.75)], "CQC", "100-40-40",
                              mm_on(60, "ABSOLUTE_SUM"), 4)]},
        {"id": "C5A-STIFF",
         "serves": "AC-002: a stiff model whose spectrum response equals the static response to its rigid "
                   "acceleration (REQ-004)",
         "purpose": "short vertical cantilever whose lowest mode lies far above the rigid frequency",
         "model": STIFF, "spectra": [S_FLAT],
         "requests": [request("C5A-RIGID", [("X", "S-FLAT", 1), ("Y", "S-FLAT", 1), ("Z", "S-FLAT", 1)],
                              "SRSS", "SRSS", mm_on(100, "SRSS"), 2)]},
        {"id": "C5B-L-LINE",
         "serves": "AC-002: a piping-shaped L line anchored at both ends, in-plane and out-of-plane (REQ-002 to "
                   "REQ-004); the unsnubbered basis of C6",
         "purpose": "two legs of 4 m along X and 3 m along Y in the XY plane, members only, anchors at both ends",
         "model": L_MODEL, "spectra": [S_LL, S_GG, S_PLG],
         "requests": [request("C5B-L", XYZ3, "CQC", "100-40-40", mm_on(120, "ABSOLUTE_SUM"), 6)]},
        {"id": "C6A-SNUBBERS-RIGID",
         "serves": "AC-003: snubbers on a global axis and on a skew direction, rigid (REQ-005)",
         "purpose": "the L line of C5B with a rigid snubber on uz at N2 and a rigid snubber along (3,4,0) at N4, "
                    "the latter as the exact constraint d^T u = 0",
         "model": L_SNUB_RIGID, "spectra": [S_LL, S_GG, S_PLG], "without_snubbers": "C5B-L-LINE",
         "requests": [request("C6A-L-RIGID", XYZ3, "CQC", "100-40-40", mm_on(120, "ABSOLUTE_SUM"), 6)]},
        {"id": "C6B-SNUBBERS-FINITE",
         "serves": "AC-003: snubbers on a global axis and on a skew direction, finite stiffness (REQ-005)",
         "purpose": "the L line of C5B with a snubber of 1048576 N/m on uz at N2 and one of 524288 N/m along "
                    "(3,4,0) at N4, k d d^T / (d^T d)",
         "model": L_SNUB_FINITE, "spectra": [S_LL, S_GG, S_PLG], "without_snubbers": "C5B-L-LINE",
         "requests": [request("C6B-L-FINITE", XYZ3, "CQC", "100-40-40", mm_on(120, "ABSOLUTE_SUM"), 6)]},
        {"id": "C8-PIPE-X",
         "serves": "AC-002 on a benchmark-shaped model: a slender straight pipe, cantilever in the XY plane and "
                   "propped cantilever in the XZ plane; every modal and directional rule; truncation with the "
                   "missing-mass correction; a mesh series",
         "purpose": "15 m pipe along X, 8 equal members, anchored at A, only uz restrained at the tip B",
         "model": c8_model(8), "spectra": [S_C8_FLAT, S_C8_TWO_LL, S_C8_TWO_GG, S_C8_TRUNC],
         "benchmark": {"planes": True, "single_direction_check": True},
         "grids": {
             "C8-I-FLAT": {"modal": ALL_MODAL, "rigid": ["OFF"], "directional": ["SRSS", "ABSOLUTE_SUM"]},
             "C8-II-TWO-LL": {"modal": ALL_MODAL, "rigid": ["OFF"], "directional": ["SRSS", "ABSOLUTE_SUM"]},
             "C8-II-TWO-GG": {"modal": ALL_MODAL, "rigid": ["OFF"], "directional": ["SRSS", "ABSOLUTE_SUM"]},
             "C8-III-TRUNCATION": {"modal": ALL_MODAL, "rigid": ["SRSS", "ABSOLUTE_SUM"],
                                   "directional": ["SRSS", "ABSOLUTE_SUM"]}},
         "requests": [
             request("C8-I-FLAT", [("Y", "S-C8-FLAT", 1), ("Z", "S-C8-FLAT", 1)], "CQC", "SRSS", MM_OFF, 6),
             request("C8-II-TWO-LL", [("Y", "S-C8-TWO-LL", 1), ("Z", "S-C8-TWO-LL", 1)], "CQC", "SRSS", MM_OFF, 6),
             request("C8-II-TWO-GG", [("Y", "S-C8-TWO-GG", 1), ("Z", "S-C8-TWO-GG", 1)], "CQC", "SRSS", MM_OFF, 6),
             request("C8-III-TRUNCATION", [("Y", "S-C8-TRUNC", 1), ("Z", "S-C8-TRUNC", 1)], "CQC", "SRSS",
                     mm_on(1.25, "SRSS"), 3),
         ],
         "mesh_series": {"members": [2, 4, 16], "request": "C8-I-FLAT",
                         "quantities": ["B.uy", "B.ry", "B.rz", "SA.Fy", "SA.Fz", "SA.My", "SA.Mz", "SB.Fz"],
                         "models": {2: c8_model(2), 4: c8_model(4), 16: c8_model(16)}}},
        {"id": "C9-PIPE-DIAGONAL",
         "serves": "AC-002 on a benchmark-shaped model: CQC on a close pair (about 5 percent) with same- and "
                   "opposite-sign contributions, every modal rule",
         "purpose": "15 m pipe along the space diagonal toward (1,1,1), 8 equal members, anchored at A, a linear "
                    "spring of 8 N/m along global X at the tip B, which splits the first bending pair",
         "model": c9_model(8), "spectra": [S_C9_FLAT],
         "member_axes": True, "scope_cluster_exception": "unextracted",
         "benchmark": {"planes": False, "close_pair": [1, 2]},
         "grids": {"C9-Y": {"modal": ALL_MODAL, "rigid": ["OFF"], "directional": ["SRSS", "ABSOLUTE_SUM"]}},
         "requests": [request("C9-Y", [("Y", "S-C9-FLAT", 1)], "CQC", "SRSS", MM_OFF, 6)]},
        {"id": "C9-CHAIN",
         "serves": "mutation data separating GROUPING from the pairwise ten-percent rule (a three-mode chain)",
         "purpose": "the pipe of C9 with a stiff tip spring of 1048576 N/m along global X instead of 8 N/m: modes "
                    "21, 22 and 23 form a chain (f22 within 10 percent of f21, f23 within 10 percent of f22, not "
                    "of f21)",
         "model": c9_model(1048576), "spectra": [S_C9_FLAT],
         "member_axes": True,
         "benchmark": {"planes": False, "chain": [21, 22, 23]},
         "grids": {"C9-CHAIN-Y": {"modal": ALL_MODAL, "rigid": ["OFF"], "directional": ["SRSS"]}},
         "publish_quantities": ["B.", "SA.", "S1."],
         "requests": [request("C9-CHAIN-Y", [("Y", "S-C9-FLAT", 1)], "CQC", "SRSS", MM_OFF, 23)]},
        {"id": "C7-REPEATED-PAIR",
         "serves": "the scope's open question on exactly repeated modes (REQ-003): basis-invariant quantities only",
         "purpose": "two members along X fixed at N1, tip free: the bending modes are exactly repeated pairs "
                    "(circular section), so any basis of a pair is a valid modal basis",
         "model": TWO_PLAIN, "spectra": [S_LL],
         "witness": {"basis": {"request": "C7-PAIRS", "modes": [1, 2], "tip": "N3", "quantities": ["N3.", "A1."]}},
         "requests": [request("C7-PAIRS", [("Y", "S-LL", 1), ("Z", "S-LL", 0.5)], "SRSS", "SRSS", MM_OFF, 4)]},
        {"id": "C10-W2-CLOSE-DISTINCT",
         "serves": "the adopted repeated-mode rule (REQ-003): a cluster of two genuinely distinct modes inside "
                   "delta_c, each with its own ordinate and contribution, summed before every modal rule",
         "purpose": "the C7 model with a skewed spring of 0.0625 N/m along (0,3,4) at the tip N3: each bending "
                    "pair splits by a relative eigenvalue gap far below delta_c; the first pair lies on a sloped "
                    "segment of S-LL",
         "model": CLOSE, "spectra": [S_LL], "scope_cluster_exception": "distinct modes inside clusters",
         "benchmark": {"planes": False},
         "witness": {"cluster_sums": True, "mode_by_mode": True, "quantities": ["N3.", "A1.", "K1."]},
         "grids": {"C10-W2": {"modal": ALL_MODAL, "rigid": ["OFF"], "directional": ["SRSS"]}},
         "requests": [request("C10-W2", [("Y", "S-LL", 1), ("Z", "S-LL", 0.5)], "SRSS", "SRSS", MM_OFF, 4)]},
        {"id": "C10-W3-STRADDLING-CLUSTER",
         "serves": "the adopted rule's periodic/rigid classification: a cluster whose two frequencies straddle "
                   "f_r is periodic as a whole (REQ-003, REQ-004)",
         "purpose": "the model of C10-W2 with f_r strictly between the two frequencies of its first cluster",
         "model": CLOSE, "spectra": [S_LL], "scope_cluster_exception": "distinct modes inside clusters",
         "benchmark": {"planes": False},
         "witness": {"cluster_sums": True, "rigid_load": True, "quantities": ["N3.", "A1.", "K1."]},
         "grids": {"C10-W3": {"modal": ALL_MODAL, "rigid": ["SRSS", "ABSOLUTE_SUM"], "directional": ["SRSS"]}},
         "requests": [request("C10-W3", [("Y", "S-LL", 1), ("Z", "S-LL", 0.5)], "SRSS", "SRSS",
                              mm_on(FR_W3, "SRSS"), 4)]},
    ]
