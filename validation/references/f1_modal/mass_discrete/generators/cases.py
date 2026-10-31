"""Invented model data for the F1 mass (AC-001) and class (a) eigen (AC-003) references.

Every number is a binary64 value written exactly in decimal (dyadic
fractions and integers); model.exact_double asserts it. No value is a catalog,
code or supplier value. The acceleration 9.8125 m/s^2 is an invented request
input, not a physical constant.
"""

STEEL = {"elastic_modulus_Pa": 200000000000, "shear_modulus_Pa": 78125000000, "density_kg_per_m3": 7850}
S1 = {  # contents and insulation
    "outside_diameter_m": 0.21875, "wall_thickness_m": 0.0078125, "mill_tolerance_m": 0.0009765625,
    "corrosion_allowance_m": 0.001953125, "contents_density_kg_per_m3": 1000,
    "insulation_thickness_m": 0.0625, "insulation_density_kg_per_m3": 128,
}
S2 = {  # bare: no contents, no insulation, no mill tolerance or allowance
    "outside_diameter_m": 0.125, "wall_thickness_m": 0.0078125, "mill_tolerance_m": 0,
    "corrosion_allowance_m": 0, "contents_density_kg_per_m3": None,
    "insulation_thickness_m": None, "insulation_density_kg_per_m3": None,
}
A_REQ = 9.8125
ALL = ["ux", "uy", "uz", "rx", "ry", "rz"]


def base(nodes, members, supports, components=(), springs=(), constant_effort=(), acc=A_REQ, sections=None):
    return {
        "units": {"length": "m", "mass": "kg", "force": "N", "stress": "Pa", "time": "s"},
        "materials": {"steel": STEEL},
        "sections": sections or {"s1": S1},
        "nodes": [{"id": n, "xyz": list(x)} for n, x in nodes],
        "members": list(members),
        "components": list(components),
        "supports": list(supports),
        "constant_effort_supports": list(constant_effort),
        "springs": list(springs),
        "weight_to_mass_acceleration_m_per_s2": acc,
    }


def straight(mid, i, j, sec="s1", yref=(0, 0, 1)):
    return {"id": mid, "kind": "straight", "i": i, "j": j, "section": sec, "material": "steel",
            "y_reference": list(yref)}


def arc(mid, i, j, radius, yref, k=1, sec="s1"):
    return {"id": mid, "kind": "arc", "i": i, "j": j, "section": sec, "material": "steel",
            "radius_m": radius, "y_reference": list(yref), "flexibility_factor": k}


def valve(cid, node, member, weight, cog, scale, consumption="mechanics_geometry_only"):
    return {"id": cid, "kind": "valve", "node": node, "maps_member": member,
            "solver_consumption": consumption, "weight_N": weight,
            "center_of_gravity_m": list(cog) if cog is not None else None,
            "stiffness_scaling": scale, "stiffness_source_reference": "invented for this reference"}


def anchor(node):
    return {"node": node, "kind": "anchor", "restrained": list(ALL)}


def chain(prefix, start, step, n):
    pts = []
    for k in range(n + 1):
        pts.append((f"{prefix}{k}", tuple(start[c] + k * step[c] for c in range(3))))
    return pts


# ============================================================================ mass
def mass_cases():
    out = []
    # --- straight members in several orientations, PF and EX, one bare section
    nodes, members, sup = [], [], []
    specs = [
        ("S-X", (0.5, 0.25, -0.125), (2.5, 0, 0), "s1", (0, 0, 1)),
        ("S-NEG-Y", (0, 1, 0), (0, -1.75, 0), "s1", (0, 0, 1)),
        ("S-Z", (1, 1, 1), (0, 0, 3), "s1", (1, 0, 0)),
        ("S-345", (2, 2, 0), (1.5, 2, 0), "s1", (0, 0, 1)),
        ("S-2M36", (-1, 0.5, 0.25), (0.5, -0.75, 1.5), "s1", (0, 0, 1)),
        ("S-X-BARE", (0, -2, 0), (1.25, 0, 0), "s2", (0, 0, 1)),
    ]
    for mid, xi, d, sec, yref in specs:
        ni, nj = f"{mid}-i", f"{mid}-j"
        nodes += [(ni, xi), (nj, tuple(xi[c] + d[c] for c in range(3)))]
        members.append(straight(mid, ni, nj, sec, yref))
        sup.append(anchor(ni))
    out.append({
        "id": "M-STRAIGHT-ORIENT",
        "purpose": "consistent straight mass in six orientations (axis X, -Y, Z, (3,4,0)/5, (2,-3,6)/7), "
                   "one bare section; each member anchored at its i node",
        "model": base(nodes, members, sup, acc=None, sections={"s1": S1, "s2": S2}),
    })
    # --- realized arcs
    nodes = [("A-XY-i", (2, 0, 0)), ("A-XY-j", (2.375, 0.375, 0)),
             ("A-SKEW-i", (1.5, 1.75, 2.5)), ("A-SKEW-j", (1.75, -0.5, 1.5)),
             ("A-PYTH-i", (1.75, -0.25, 0.125)), ("A-PYTH-j", (1.25, 0.75, 0.125))]
    members = [arc("A-XY", "A-XY-i", "A-XY-j", 0.375, (0, -1, 0)),
               arc("A-SKEW", "A-SKEW-i", "A-SKEW-j", 1.75, (5, -3, 8)),
               arc("A-PYTH", "A-PYTH-i", "A-PYTH-j", 1.25, (2, 1, 0))]
    out.append({
        "id": "M-ARC-SMALL",
        "purpose": "small-angle arc for stable evaluation: chord 1/4096 m, R = 625/512 m, so sin(alpha) = 1e-4 "
                   "exactly (half-angle alpha = asin(1e-4)); bend in the XZ plane; anchored at its i node",
        "model": base([("T-i", (2, 1, 0.5)), ("T-j", (2.000244140625, 1, 0.5))],
                      [arc("A-SMALL", "T-i", "T-j", 1.220703125, (0, 0, 1))], [anchor("T-i")], acc=None),
    })
    out.append({
        "id": "M-ARC",
        "purpose": "half-arc rigid lumps: a 90 degree arc in the XY plane, a 90 degree arc in a skew plane "
                   "(tangents (2,3,6)/7 and (3,-6,2)/7 directions), and a 53.13 degree arc (cos = 3/5); "
                   "each anchored at its i node",
        "model": base(nodes, members, [anchor("A-XY-i"), anchor("A-SKEW-i"), anchor("A-PYTH-i")], acc=None),
    })
    # --- eligible component, centre of gravity variants
    comp_nodes = [("A", (0, 0, 0)), ("B", (1.5, 0, 0)), ("C", (2, 0, 0))]
    comp_members = [straight("P1", "A", "B"), straight("P2", "B", "C")]
    for tag, cog in (("NODE", (0, 0, 0)), ("AXIAL", (0.25, 0, 0)), ("LATERAL", (0, 0.375, 0)),
                     ("GENERAL", (0.25, -0.375, 0.125))):
        out.append({
            "id": f"M-COMP-COG-{tag}",
            "purpose": "eligible valve body P2 (component V1 at C): pipe mass of P2 replaced by m = W/a at the "
                       f"centre of gravity r = {list(cog)} from C",
            "model": base(comp_nodes, comp_members, [anchor("A")],
                          components=[valve("V1", "C", "P2", 2560, cog, 8)]),
        })
    # --- ineligible components: keep pipe mass
    out.append({
        "id": "M-COMP-INELIGIBLE-CONSUMPTION",
        "purpose": "valve without solver_consumption: P2 keeps its pipe mass and the weight is not applied "
                   "(first unmet condition: consumption)",
        "model": base(comp_nodes, comp_members, [anchor("A")],
                      components=[valve("V1", "C", "P2", 2560, (0.25, 0, 0), None, consumption=None)]),
    })
    out.append({
        "id": "M-COMP-INELIGIBLE-NO-COG",
        "purpose": "valve with consumption, weight and scale but no centre of gravity: P2 keeps its pipe mass "
                   "(first unmet condition: centre of gravity); its stiffness scale still applies",
        "model": base(comp_nodes, comp_members, [anchor("A")],
                      components=[valve("V1", "C", "P2", 2560, None, 8)]),
    })
    # --- massless families
    nodes = [("A", (0, 0, 0)), ("B", (2, 0, 0)), ("C", (2.25, 0, 0)), ("D", (4.25, 0, 0))]
    members = [straight("P1", "A", "B"),
               {"id": "J1", "kind": "connector", "i": "B", "j": "C",
                "axial_stiffness_N_per_m": 1000000, "note": "objective connector; stiffness not used here"},
               {"id": "T1", "kind": "tie_rod", "i": "B", "j": "C",
                "axial_stiffness_N_per_m": 50000000, "note": "bilateral linear elastic; stiffness not used here"},
               straight("P2", "C", "D")]
    out.append({
        "id": "M-MASSLESS",
        "purpose": "an objective connector, a tie rod and a constant-effort support carry no mass; "
                   "the total equals the two pipes",
        "model": base(nodes, members, [anchor("A"), {"node": "D", "kind": "guide", "restrained": ["uy", "uz"]}],
                      constant_effort=[{"node": "C", "load_N": 1000, "note": "load only; no stiffness, no mass"}],
                      acc=None),
    })
    # --- assembled model
    nodes = [("A", (0, 0, 0)), ("B", (2, 0, 0)), ("C", (2.375, 0.375, 0)), ("D", (2.375, 1.875, 0)),
             ("E", (2.375, 2.375, 0)), ("F", (2.375, 2.375, 1.5))]
    members = [straight("P1", "A", "B"), arc("BEND1", "B", "C", 0.375, (0, -1, 0)),
               straight("P2", "C", "D"), straight("VB", "D", "E"),
               straight("P3", "E", "F", yref=(1, 0, 0))]
    out.append({
        "id": "M-MODEL",
        "purpose": "assembled model: straights in X, Y and Z, a 90 degree arc, a valve body with an offset "
                   "centre of gravity, a constant-effort support; anchor at A, F pinned",
        "model": base(nodes, members,
                      [anchor("A"), {"node": "F", "kind": "pinned", "restrained": ["ux", "uy", "uz"]}],
                      components=[valve("V1", "E", "VB", 1536, (0.125, 0.25, -0.375), 16)],
                      constant_effort=[{"node": "C", "load_N": 500, "note": "load only"}]),
    })
    return out


# ============================================================================ eigen
def eigen_cases():
    out = []
    for n in (1, 2, 4, 8):
        nodes = chain("N", (0, 0, 0), (4 / n, 0, 0), n)
        members = [straight(f"P{k + 1}", f"N{k}", f"N{k + 1}") for k in range(n)]
        out.append({"id": f"E-CANT-{n}", "purpose": f"cantilever along X, L = 4 m, {n} element(s); exact "
                    "bending pairs (Y, Z); axial modes; pure torsion modes (normalization case b)",
                    "model": base(nodes, members, [anchor("N0")], acc=None)})
    nodes = chain("N", (0, 0, 0), (0.25, 0.375, 0.75), 4)
    out.append({"id": "E-CANT-INCLINED-4", "purpose": "cantilever along (2,3,6)/7, L = 3.5 m, 4 elements",
                "model": base(nodes, [straight(f"P{k + 1}", f"N{k}", f"N{k + 1}") for k in range(4)],
                              [anchor("N0")], acc=None)})
    for n in (1, 2, 4, 8):
        nodes = chain("N", (0, 0, 0), (4 / n, 0, 0), n)
        sup = [{"node": "N0", "kind": "pinned", "restrained": ["ux", "uy", "uz", "rx"]},
               {"node": f"N{n}", "kind": "roller", "restrained": ["uy", "uz"]}]
        out.append({"id": f"E-SS-{n}", "purpose": f"simply supported span along X, L = 4 m, {n} element(s)",
                    "model": base(nodes, [straight(f"P{k + 1}", f"N{k}", f"N{k + 1}") for k in range(n)],
                                  sup, acc=None)})
    for n in (2, 4, 8):
        nodes = chain("N", (0, 0, 0), (4 / n, 0, 0), n)
        out.append({"id": f"E-FF-{n}", "purpose": f"fixed-fixed span along X, L = 4 m, {n} elements",
                    "model": base(nodes, [straight(f"P{k + 1}", f"N{k}", f"N{k + 1}") for k in range(n)],
                                  [anchor("N0"), anchor(f"N{n}")], acc=None)})
    for tag, cog in (("NODE", (0, 0, 0)), ("AXIAL", (0.25, 0, 0)), ("LATERAL", (0, 0.375, 0)),
                     ("GENERAL", (0.25, -0.375, 0.125))):
        nodes = chain("N", (0, 0, 0), (1, 0, 0), 4)
        members = [straight(f"P{k + 1}", f"N{k}", f"N{k + 1}") for k in range(4)]
        out.append({"id": f"E-POINTMASS-{tag}",
                    "purpose": "cantilever along X, 4 x 1 m; the tip member P4 is a valve body (V1 at N4, s = 8) "
                               f"whose mass is W/a = 2560/9.8125 kg at r = {list(cog)} from N4",
                    "model": base(nodes, members, [anchor("N0")],
                                  components=[valve("V1", "N4", "P4", 2560, cog, 8)])})
    for k, tag in ((1, "K1"), (3, "K3")):
        nodes = [("A", (0, 0, 0)), ("M1", (1.5, 0, 0)), ("B", (3, 0, 0)), ("C", (3.375, 0.375, 0)),
                 ("M2", (3.375, 2.375, 0)), ("D", (3.375, 4.375, 0))]
        members = [straight("P1", "A", "M1"), straight("P2", "M1", "B"), arc("BEND1", "B", "C", 0.375, (0, -1, 0), k),
                   straight("P3", "C", "M2"), straight("P4", "M2", "D")]
        out.append({"id": f"E-L-LINE-{tag}", "purpose": f"anchored L-line in the XY plane (legs 3 m and 4 m), 90 degree "
                    f"arc R = 0.375 m, flexibility factor {k}", "model": base(nodes, members, [anchor("A"), anchor("D")], acc=None)})
    for k, tag in ((1, "K1"), (2.5, "K2.5")):
        nodes = [("A", (0, 0, 0)), ("A1", (1, 0, 0)), ("B", (2, 0, 0)), ("C", (2.375, 0.375, 0)),
                 ("C1", (2.375, 1.375, 0)), ("D", (2.375, 2.375, 0)), ("E", (2.75, 2.75, 0)),
                 ("E1", (3.75, 2.75, 0)), ("F", (4.75, 2.75, 0))]
        members = [straight("P1", "A", "A1"), straight("P2", "A1", "B"), arc("BEND1", "B", "C", 0.375, (0, -1, 0), k),
                   straight("P3", "C", "C1"), straight("P4", "C1", "D"), arc("BEND2", "D", "E", 0.375, (0, 1, 0), k),
                   straight("P5", "E", "E1"), straight("P6", "E1", "F")]
        out.append({"id": f"E-Z-BEND-{tag}", "purpose": f"anchored Z-bend in the XY plane, two 90 degree arcs "
                    f"(left then right turn), flexibility factor {k}",
                    "model": base(nodes, members, [anchor("A"), anchor("F")], acc=None)})
    for s in (1000, 10000, 100000, 1000000):
        nodes = [("A", (0, 0, 0)), ("B", (1.5, 0, 0)), ("C", (2, 0, 0)), ("D", (3.5, 0, 0)), ("E", (5, 0, 0))]
        members = [straight("P1", "A", "B"), straight("VB", "B", "C"), straight("P3", "C", "D"),
                   straight("P4", "D", "E")]
        out.append({"id": f"E-RIGID-SCALE-{s:.0e}".replace("+0", ""),
                    "purpose": f"valve body B-C with stiffness scale s = {s} (component V1 at C, W = 4096 N, "
                               "r = (0, 0.25, 0)); anchor at A, E pinned",
                    "model": base(nodes, members,
                                  [anchor("A"), {"node": "E", "kind": "pinned", "restrained": ["ux", "uy", "uz"]}],
                                  components=[valve("V1", "C", "VB", 4096, (0, 0.25, 0), s)])})
    out.append({"id": "E-AXIAL-WITNESS", "witness": "axial_flat_spectrum",
                "purpose": "one straight element along X, L = 2 m, node N0 anchored, N1 free in ux only: one axial "
                           "mode; M_ff = m/3, M_fc = m/6, p = m/2, effective mass 3m/4 of T = m; u/A = m/(2k)",
                "model": base([("N0", (0, 0, 0)), ("N1", (2, 0, 0))], [straight("P1", "N0", "N1")],
                              [anchor("N0"), {"node": "N1", "kind": "axial_only",
                                              "restrained": ["uy", "uz", "rx", "ry", "rz"]}], acc=None)})
    nodes = chain("N", (0, 0, 0), (1.25, 0, 0), 4)
    springs = [{"node": "N2", "dof": "uy", "stiffness": 250000}, {"node": "N2", "dof": "uz", "stiffness": 400000},
               {"node": "N4", "dof": "ux", "stiffness": 1000000}, {"node": "N4", "dof": "uy", "stiffness": 125000},
               {"node": "N4", "dof": "rz", "stiffness": 50000}]
    out.append({"id": "E-SPRINGS", "purpose": "cantilever along X, 4 x 1.25 m, translational and rotational "
                "springs (N/m, N*m/rad) at N2 and N4",
                "model": base(nodes, [straight(f"P{k + 1}", f"N{k}", f"N{k + 1}") for k in range(4)],
                              [anchor("N0")], springs=springs, acc=None)})
    return out
