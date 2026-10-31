"""Neutral model input, section bases and assembly for the F1 references.

A model is a plain dict (written verbatim into the references as `model`):

  nodes        [{id, xyz: [x, y, z]}]                         m
  materials    {name: {elastic_modulus_Pa, shear_modulus_Pa, density_kg_per_m3}}
  sections     {name: {outside_diameter_m, wall_thickness_m, mill_tolerance_m,
                       corrosion_allowance_m, contents_density_kg_per_m3 | null,
                       insulation_thickness_m | null, insulation_density_kg_per_m3 | null}}
  members      [{id, kind: straight | arc | connector | tie_rod, i, j,
                 section, material, y_reference: [..],
                 radius_m (arc), flexibility_factor (arc)}]
  components   [{id, kind, node, maps_member, solver_consumption | null,
                 weight_N | null, center_of_gravity_m: [..] | null,
                 stiffness_scaling | null}]
  supports     [{node, kind, restrained: [ux, uy, uz, rx, ry, rz subset]}]
  constant_effort_supports [{node}]                            (massless, no stiffness)
  springs      [{node, dof, stiffness}]                       N/m or N*m/rad, global
  weight_to_mass_acceleration_m_per_s2  a | null

Every input number is a binary64 value written exactly in decimal; the
generator asserts it and uses its exact rational value.
"""
from fractions import Fraction

import elements as el
from numeric import exact_double, QPi

DOFS = ["ux", "uy", "uz", "rx", "ry", "rz"]
RIGID_FAMILY = ("valve", "flange", "reducer", "rigid", "specialty")
GEOMETRY_ONLY = "mechanics_geometry_only"


def exact_sqrt(fr):
    """Exact square root of a non-negative Fraction when it is a perfect square, else None."""
    import math
    n, d = fr.numerator, fr.denominator
    rn, rd = math.isqrt(n), math.isqrt(d)
    if rn * rn == n and rd * rd == d:
        return Fraction(rn, rd)
    return None


# --------------------------------------------------------------------------
# Section bases (pi factored out: every value below is a rational coefficient of pi)
# --------------------------------------------------------------------------
def section_basis(sec, mat, basis):
    """Mass per length and rotary inertia per length on a mass basis, and the stiffness section.

    PF  source_od_effective_wall_areas/v1: metal and contents on the wall t - c_m.
    EX  source_od_nominal_wall_pressure_bore_areas/v1: metal on the nominal wall t,
        contents on the pressure bore r_p = OD/2 - t_r, t_r = (t - c_m) - c_a.
    Insulation on both: the annulus OD .. OD + 2 t_ins.
    j_bar = rho_m J_metal + rho_ins J_ins (contents excluded), J of an annulus
    = pi (D_o^4 - D_i^4) / 32 on the same metal annulus as the metal mass;
    i_bar = j_bar / 2.
    The stiffness section (A, I, J = 2I) is the metal annulus of the same basis
    (PF: t - c_m; EX: nominal t).
    """
    OD = exact_double(sec["outside_diameter_m"])
    t = exact_double(sec["wall_thickness_m"])
    cm = exact_double(sec["mill_tolerance_m"])
    ca = exact_double(sec["corrosion_allowance_m"])
    rho_m = exact_double(mat["density_kg_per_m3"])
    if basis == "PF":
        wall_metal = t - cm
        id_metal = OD - 2 * wall_metal
        id_bore = id_metal
        method = "source_od_effective_wall_areas/v1"
    elif basis == "EX":
        wall_metal = t
        id_metal = OD - 2 * t
        t_r = (t - cm) - ca
        id_bore = OD - 2 * t_r
        method = "source_od_nominal_wall_pressure_bore_areas/v1"
    else:
        raise ValueError(basis)
    a_metal = (OD ** 2 - id_metal ** 2) / 4
    i_metal = (OD ** 4 - id_metal ** 4) / 64
    j_metal = 2 * i_metal
    bore = id_bore ** 2 / 4
    rho_c = sec.get("contents_density_kg_per_m3")
    rho_c = exact_double(rho_c) if rho_c is not None else Fraction(0)
    t_ins = sec.get("insulation_thickness_m")
    rho_i = sec.get("insulation_density_kg_per_m3")
    assert (t_ins is None) == (rho_i is None), "insulation needs both thickness and density"
    if t_ins is not None:
        t_ins, rho_i = exact_double(t_ins), exact_double(rho_i)
        Do = OD + 2 * t_ins
        a_ins = (Do ** 2 - OD ** 2) / 4
        j_ins = (Do ** 4 - OD ** 4) / 32
    else:
        rho_i, a_ins, j_ins = Fraction(0), Fraction(0), Fraction(0)
    jbar = rho_m * j_metal + rho_i * j_ins
    return {
        "method": method,
        "metal": rho_m * a_metal,
        "contents": rho_c * bore,
        "insulation": rho_i * a_ins,
        "mbar": rho_m * a_metal + rho_c * bore + rho_i * a_ins,
        "jbar": jbar,
        "ibar": jbar / 2,
        "A": a_metal,
        "I": i_metal,
        "J": j_metal,
        "inner_diameter_metal": id_metal,
        "inner_diameter_bore": id_bore,
    }


# --------------------------------------------------------------------------
# Model
# --------------------------------------------------------------------------
class Model:
    def __init__(self, spec):
        self.spec = spec
        self.node_ids = [n["id"] for n in spec["nodes"]]
        self.idx = {nid: k for k, nid in enumerate(self.node_ids)}
        self.xyz = [[exact_double(v) for v in n["xyz"]] for n in spec["nodes"]]
        self.members = {m["id"]: m for m in spec["members"]}
        self.member_order = [m["id"] for m in spec["members"]]
        self.n = 6 * len(self.node_ids)
        a = spec.get("weight_to_mass_acceleration_m_per_s2")
        self.acc = exact_double(a) if a is not None else None
        self.restrained = set()
        for s in spec.get("supports", []):
            for d in s["restrained"]:
                self.restrained.add(6 * self.idx[s["node"]] + DOFS.index(d))
        self.free = [k for k in range(self.n) if k not in self.restrained]
        self._components()

    def _components(self):
        """Component-weight eligibility (self-weight's predicate) and stiffness scaling."""
        self.body_of = {}       # member id -> component (mass replaced)
        self.scale_of = {}      # member id -> stiffness scale
        self.component_status = []
        for c in self.spec.get("components", []):
            reason = None
            mem = self.members.get(c.get("maps_member"))
            if c["kind"] not in RIGID_FAMILY:
                reason = "not a rigid-family component"
            elif c.get("solver_consumption") != GEOMETRY_ONLY:
                reason = "consumption: solver_consumption is not mechanics_geometry_only"
            elif mem is None or c["node"] not in (mem["i"], mem["j"]):
                reason = "mapping: the mapped member does not end at the component node"
            elif c.get("weight_N") is None or exact_double(c["weight_N"]) <= 0:
                reason = "weight: missing or not positive"
            elif c.get("center_of_gravity_m") is None:
                reason = "center_of_gravity: missing"
            if reason is None:
                self.body_of[mem["id"]] = c
            scale_ok = (c["kind"] in RIGID_FAMILY and c.get("solver_consumption") == GEOMETRY_ONLY
                        and mem is not None and c["node"] in (mem["i"], mem["j"])
                        and c.get("stiffness_scaling") is not None)
            if scale_ok:
                self.scale_of[mem["id"]] = exact_double(c["stiffness_scaling"])
            self.component_status.append({
                "component": c["id"],
                "weight_applied_as_body_mass": reason is None,
                "first_unmet_condition": reason,
                "expected_warning": None if reason is None or c["kind"] not in RIGID_FAMILY
                else "SELF_WEIGHT_COMPONENT_WEIGHT_NOT_APPLIED",
                "stiffness_scale_applied": float(self.scale_of[mem["id"]]) if scale_ok else None,
            })

    # ---------------------------------------------------------------- geometry
    def x(self, F, nid):
        return [F.q(v) for v in self.xyz[self.idx[nid]]]

    def straight_axis(self, F, m):
        xi, xj = self.xyz[self.idx[m["i"]]], self.xyz[self.idx[m["j"]]]
        d = [xj[k] - xi[k] for k in range(3)]
        L2 = sum(v * v for v in d)
        Le = exact_sqrt(L2)
        if Le is not None:
            L = F.q(Le)
            return [F.q(v / Le) for v in d], L, Le
        L = F.sqrt(F.q(L2))
        return [F.q(v) / L for v in d], L, None

    def arc_geometry(self, F, m):
        return el.ArcGeometry(F, self.x(F, m["i"]), self.x(F, m["j"]),
                              F.q(exact_double(m["radius_m"])), [exact_double(v) for v in m["y_reference"]])

    def section(self, m, basis):
        return section_basis(self.spec["sections"][m["section"]], self.spec["materials"][m["material"]], basis)

    def dofs_of(self, nid):
        b = 6 * self.idx[nid]
        return list(range(b, b + 6))

    def bbox_diagonal_sq(self):
        lo = [min(p[k] for p in self.xyz) for k in range(3)]
        hi = [max(p[k] for p in self.xyz) for k in range(3)]
        return sum((hi[k] - lo[k]) ** 2 for k in range(3))

    def node_centroid(self):
        n = len(self.xyz)
        return [sum(p[k] for p in self.xyz) / n for k in range(3)]


# --------------------------------------------------------------------------
# Mass
# --------------------------------------------------------------------------
def add_block(M, dofs, blk):
    for a, ra in enumerate(dofs):
        row = M[ra]
        for b, rb in enumerate(dofs):
            row[rb] = row[rb] + blk[a][b]


def form_mass(model, F, basis):
    """Full (unreduced) mass, element contributions and category totals.

    F may be QPiField only for models without arcs.
    """
    pi = F.pi()
    M = el.zeros(F, model.n, model.n)
    contributions = []
    cats = {"metal": F.q(0), "contents": F.q(0), "insulation": F.q(0), "component_bodies": F.q(0)}
    for mid in model.member_order:
        m = model.members[mid]
        kind = m["kind"]
        if kind in ("connector", "tie_rod"):
            contributions.append({"id": mid, "kind": kind, "massless": True})
            continue
        if mid in model.body_of:
            contributions.append({"id": mid, "kind": kind, "replaced_by_component": model.body_of[mid]["id"]})
            continue
        sb = model.section(m, basis)
        dofs = model.dofs_of(m["i"]) + model.dofs_of(m["j"])
        if kind == "straight":
            e, L, _ = model.straight_axis(F, m)
            blk = el.straight_mass(F, e, L, F.q(sb["mbar"]) * pi, F.q(sb["ibar"]) * pi, F.q(sb["jbar"]) * pi)
            add_block(M, dofs, blk)
            length = L
            contributions.append({"id": mid, "kind": kind, "dofs": dofs, "matrix": blk, "length": L, "axis": e,
                                  "mbar": F.q(sb["mbar"]) * pi})
        elif kind == "arc":
            geo = model.arc_geometry(F, m)
            mbar = F.q(sb["mbar"]) * pi
            lumps = []
            section = el.half_arc_section_inertia(F, geo, F.q(sb["jbar"]) * pi, F.q(sb["ibar"]) * pi)
            for (side, mu, S, J), (_, Js) in zip(el.half_arc_unit_lumps(F, geo), section):
                Sm = [mbar * v for v in S]
                Jc = [[mbar * v for v in r] for r in J]
                # Design section 2.4: J_n = centreline second moment + section rotary inertia
                Jn = [[Jc[a][b] + Js[a][b] for b in range(3)] for a in range(3)]
                blk = el.rigid_lump(F, mbar * mu, Sm, Jn)
                nd = model.dofs_of(m[side])
                add_block(M, nd, blk)
                lumps.append({"node": m[side], "dofs": nd, "matrix": blk, "mass": mbar * mu, "first_moment": Sm,
                              "centreline_second_moment": Jc, "section_rotary_inertia": Js,
                              "matrix_centreline_only": el.rigid_lump(F, mbar * mu, Sm, Jc)})
            length = geo.R * geo.phi
            contributions.append({"id": mid, "kind": kind, "lumps": lumps, "geometry": geo, "length": length,
                                  "mbar": mbar, "jbar": F.q(sb["jbar"]) * pi, "ibar": F.q(sb["ibar"]) * pi})
        else:
            raise ValueError(kind)
        for cat in ("metal", "contents", "insulation"):
            cats[cat] = cats[cat] + F.q(sb[cat]) * pi * length
    for c in model.spec.get("components", []):
        mem_id = c.get("maps_member")
        if mem_id not in model.body_of or model.body_of[mem_id] is not c:
            continue
        mass = F.q(exact_double(c["weight_N"]) / model.acc)
        r = [F.q(exact_double(v)) for v in c["center_of_gravity_m"]]
        blk = el.point_mass_lump(F, mass, r)
        add_block(M, model.dofs_of(c["node"]), blk)
        contributions.append({"id": c["id"], "kind": "component_body", "node": c["node"],
                              "dofs": model.dofs_of(c["node"]), "matrix": blk, "mass": mass, "cog": r})
        cats["component_bodies"] = cats["component_bodies"] + mass
    return M, contributions, cats


# --------------------------------------------------------------------------
# Stiffness (PF linear static K: no load-dependent terms)
# --------------------------------------------------------------------------
def form_stiffness(model, F, basis="PF"):
    pi = F.pi()
    K = el.zeros(F, model.n, model.n)
    for mid in model.member_order:
        m = model.members[mid]
        kind = m["kind"]
        mat = model.spec["materials"][m["material"]]
        E = F.q(exact_double(mat["elastic_modulus_Pa"]))
        G = F.q(exact_double(mat["shear_modulus_Pa"]))
        sb = model.section(m, basis)
        A, I, J = F.q(sb["A"]) * pi, F.q(sb["I"]) * pi, F.q(sb["J"]) * pi
        s = model.scale_of.get(mid)
        if s is not None:
            E, G = E * F.q(s), G * F.q(s)
        dofs = model.dofs_of(m["i"]) + model.dofs_of(m["j"])
        if kind == "straight":
            e, L, _ = model.straight_axis(F, m)
            add_block(K, dofs, el.straight_stiffness(F, e, L, E * A, E * I, G * J))
        elif kind == "arc":
            geo = model.arc_geometry(F, m)
            k = F.q(exact_double(m["flexibility_factor"]))
            add_block(K, dofs, el.arc_stiffness(F, geo, E * A, E * I, G * J, k))
        else:
            raise ValueError(f"no stiffness formulation in these references for {kind}")
    for sp in model.spec.get("springs", []):
        d = 6 * model.idx[sp["node"]] + DOFS.index(sp["dof"])
        K[d][d] = K[d][d] + F.q(exact_double(sp["stiffness"]))
    return K


def reduce(A, free):
    return [[A[i][j] for j in free] for i in free]


def rigid_translation(model, F, k):
    r = [F.q(0)] * model.n
    for node in range(len(model.node_ids)):
        r[6 * node + k] = F.q(1)
    return r


def exact_entry(x):
    """QPi -> exact string; used for straight and component matrices."""
    return QPi._c(x).exact_str()
