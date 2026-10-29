"""PF M3-T independent exact reference engine (standard-library Python only).

It imports and reads no product code. Its input is a public product request for the
pressure-free (PF) route: model document 0.8.0 with no `pressure_contract`, the beam theory
selected by `model_settings.shear_model` (absent or `euler_bernoulli`: Euler-Bernoulli;
`timoshenko_annulus_energy_v1`: Timoshenko straight members). Every binary64 number in the
request is taken as its exact value. The element functions, the 110-digit `decimal` context,
Machin's pi, the dense solve and the recovery statics are imported unchanged from
../kd5_timoshenko/kd5t_engine.py (read-only; no bytecode is written there).

PF premises (pf-timoshenko.md sections 2 and 3; section-bases-and-timoshenko.md sections 1, 3):

- **Section.** PF's single stiffness section is the annulus r_o = OD/2, r_i = OD/2 - (t - c_m)
  ("PF's stiffness section, computed by PF's existing arithmetic (derive_pipe_section)"), used for
  every purpose (A, I, J = 2I, Z = I/r_o; stresses; the shear factor). The engine forms it exactly
  from the binary64 OD, wall and mill tolerance. PF's own section arithmetic is binary64; where
  fl(t - c_m) is not t - c_m exactly the engine refuses the model (the generator only authors exact
  differences), so the only difference left is the product's ulp-level area arithmetic.
- **Moduli.** E and G are authored independently; GJ uses the authored G. An authored
  `poisson_ratio` is not consumed.
- **Shear factor.** nu = E/(2G) - 1 exactly, and kappa_E is SHEAR_REFERENCE's energy-matched
  annular factor of that nu on PF's annulus, formed exactly in rationals in the moduli form of
  pf-timoshenko.md section 2 (t = 1 - 2G/E, kappa_E = 6a^2/((7 + t^2) b^2 + 48 m^2)); the
  self-check proves it equal to kd5t_engine.kappa_energy(m, nu), SHEAR_REFERENCE's form.
- **phi** = 12 E I/(kappa_E G A L^2) = 3 E (r_o^2 + r_i^2)/(kappa_E G L^2), an exact rational
  (I/A = (r_o^2 + r_i^2)/4), on every straight member of a Timoshenko document; 0 otherwise.
- **Loads.** Nodal forces and moments; full-span uniform `distributed_force` and `weight`
  primitives (consistent vector, both theories); `thermal` primitives as the held axial pair
  -/+ E A alpha dT; `imposed_displacement` primitives as prescribed values at a restrained DOF of
  the named support (W-16, partitioned elimination).
- **Supports.** `anchor` and `guide` restrain their listed DOFs (prescribed zero unless an imposed
  displacement names the DOF); `spring` adds a grounded stiffness on one DOF.
- **Recovery** (kd5t_engine): end rows f = K d - feq (node-on-element), the j-side section action at
  0, .25, .5, .75, 1 by statics, support-on-pipe reactions, stresses on the single section.
- **Derived rows** (published by PF, certified by nothing: DEL-04-01 `global-end-actions.md`,
  DEL-04-07 `u4b-u5b-implementation.md` sections 2.3 and 2.4): the global end actions F = R^T f,
  M = R^T m; the normal-stress extremes sigma+- = sigma_N +- hypot(sigma_y, sigma_z) of the station's
  stress rows; and the Saint-Venant transverse shear tau_v,max = |V| c, c = ((3 + 2nu) r_o^2 +
  (1 + 2nu) r_i^2)/(4 (1 + nu) I), with nu = E/(2G) - 1 exact (formed without 1 + nu: 1 + nu = E/(2G)),
  V the section shear at interior stations and the end-force shear at the ends. tau rows exist only
  where PF's binary64 nu_d lies in (-1, 1/2); otherwise the member has none and the case carries
  `TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN`.

Admission (design section 2, binary64, reported beside the rows): a Timoshenko document is
expected refused by name (`PF_TIMOSHENKO_SHEAR_FACTOR_UNAVAILABLE`) when nu_d = fl(fl(E/fl(2G)) - 1)
is not finite or not in (-1, 1/2), or when the moduli-form factor is not representable in
binary64 (t*t or D overflows, kappa_E or A_s = kappa_E*A not normal, G*A_s, (G*A_s)*L^2 or phi
not finite and normal), evaluated here in the design's stated operation order.
"""

from __future__ import annotations

import math
import os
import sys
from decimal import Decimal
from fractions import Fraction as Q

HERE = os.path.dirname(os.path.abspath(__file__))
sys.dont_write_bytecode = True
sys.path.insert(0, os.path.join(HERE, "..", "kd5_timoshenko"))
import kd5t_engine as K  # noqa: E402  (read-only reuse: element, solve, pi, decimal context)

D = Decimal
ZERO, ONE = K.ZERO, K.ONE
PI = K.PI
DOFS = K.DOFS
DIRS = K.DIRS
LOCATIONS = K.LOCATIONS
solve_dense = K.solve_dense
TIMOSHENKO = "timoshenko_annulus_energy_v1"
EULER = "euler_bernoulli"
REFUSAL_CODE = "PF_TIMOSHENKO_SHEAR_FACTOR_UNAVAILABLE"
SI = {"m", "N", "N*m", "N/m", "Pa", "degC", "1/degC", "1", "rad", "N*m/rad", "none"}


def q_si(qty, allowed=None):
    if not isinstance(qty, dict) or "value" not in qty:
        raise ValueError(f"not a quantity: {qty!r}")
    unit = qty.get("unit")
    if unit not in SI or (allowed and unit not in allowed):
        raise ValueError(f"unit {unit!r} is outside the engine's SI set")
    return float(qty["value"])


def qd(x: Q) -> Decimal:
    """A Fraction as a 110-digit Decimal."""
    return D(x.numerator) / D(x.denominator)


# ------------------------------------------------------------- the factor
def kappa_moduli(m: Q, E: Q, G: Q) -> Q:
    """pf-timoshenko.md section 2's moduli form, exact: x = 2G/E, t = 1 - x, kappa = 6a^2/((7+t^2)b^2+48m^2)."""
    m2 = m * m
    a, b = 1 + m2, 1 - m2
    t = 1 - 2 * G / E
    return 6 * (a * a) / ((7 + t * t) * (b * b) + 48 * m2)


def pf_section(od: float, wall: float, mill: float):
    """PF's single annulus, exact: r_o = OD/2, r_i = r_o - (t - c_m). Refuses inexact binary64 t - c_m."""
    weff = Q(wall) - Q(mill)
    if Q(wall - mill) != weff:
        raise ValueError("fl(t - c_m) differs from t - c_m: the PF section is then not determined here")
    ro = Q(od) / 2
    ri = ro - weff
    if not (0 < ri < ro):
        raise ValueError("wall outside (0, OD/2)")
    return ro, ri


def is_normal(x: float) -> bool:
    return math.isfinite(x) and abs(x) >= sys.float_info.min


def nu_derived(E: float, G: float) -> float:
    """PF's binary64 nu_d = RN(RN(E/RN(2G)) - 1) (pf-timoshenko.md section 2; u4b-u5b-implementation.md section 2.4)."""
    twoG = 2.0 * G
    return (E / twoG if twoG != 0 else math.inf) - 1.0


def nu_in_domain(E: float, G: float) -> bool:
    nu_d = nu_derived(E, G)
    return math.isfinite(nu_d) and -1.0 < nu_d < 0.5


def admission(od, wall, mill, E: float, G: float, L2: Q):
    """Design section 2's admission of a Timoshenko member, evaluated in binary64: (admitted, reasons)."""
    reasons = []
    twoG = 2.0 * G
    nu_d = nu_derived(E, G)
    if not nu_in_domain(E, G):
        reasons.append(f"nu_d={nu_d!r} outside (-1, 1/2)")
    # the moduli form, in the design's order (no fused multiply-add)
    ro = od / 2
    ri = ro - (wall - mill)
    m = ri / ro
    m2 = m * m
    a, b = 1 + m2, 1 - m2
    x = twoG / E
    t = 1 - x
    tt = t * t
    if not math.isfinite(tt):
        reasons.append("t*t overflows")
    Dv = (7 + tt) * (b * b) + 48 * m2
    if not is_normal(Dv):
        reasons.append("D not normal")
    kap = (6 * (a * a)) / Dv if math.isfinite(Dv) and Dv != 0 else 0.0
    if not is_normal(kap):
        reasons.append("kappa_E not normal")
    A = math.pi * (ro * ro - ri * ri)
    As = kap * A
    if not is_normal(As):
        reasons.append("A_s not normal")
    GAs = G * As
    L2f = float(L2)
    if not is_normal(GAs):
        reasons.append("G*A_s not normal")
    elif not is_normal(GAs * L2f):
        reasons.append("(G*A_s)*L^2 not normal")
    else:
        I = math.pi * (ro ** 4 - ri ** 4) / 4
        phi = 12 * E * I / (GAs * L2f)
        if not is_normal(phi):
            reasons.append("phi not normal")
    return (not reasons), reasons


# ------------------------------------------------------------- the model
class Member:
    def __init__(self, spec, nodes, materials, theory):
        self.id = spec["id"]
        self.i, self.j = spec["from"], spec["to"]
        mat = materials[spec["material"]]
        self.Ef, self.Gf = mat["E"], mat["G"]
        self.EQ, self.GQ = Q(mat["E"]), Q(mat["G"])
        self.E, self.G = qd(self.EQ), qd(self.GQ)
        self.alpha = mat["alpha"]
        sec = spec["section"]
        self.od = q_si(sec["outside_diameter"], {"m"})
        self.wall = q_si(sec["wall_thickness"], {"m"})
        self.mill = q_si(sec["mill_tolerance"], {"m"}) if "mill_tolerance" in sec else 0.0
        if "corrosion_allowance" in sec and q_si(sec["corrosion_allowance"], {"m"}) != 0.0:
            raise ValueError("a corrosion allowance is outside the PF engine")
        self.roQ, self.riQ = pf_section(self.od, self.wall, self.mill)
        ro, ri = qd(self.roQ), qd(self.riQ)
        A = PI * (ro * ro - ri * ri)
        I = PI * (ro ** 4 - ri ** 4) / 4
        self.sec = {"ro": ro, "ri": ri, "A": A, "I": I, "J": 2 * I, "Z": I / ro, "Ai": PI * ri * ri}
        xi, xj = nodes[self.i], nodes[self.j]
        dQ = [Q(b) - Q(a) for a, b in zip(xi["f"], xj["f"])]
        self.L2 = dQ[0] ** 2 + dQ[1] ** 2 + dQ[2] ** 2
        if self.L2 == 0:
            raise ValueError("zero-length member")
        d = [qd(c) for c in dQ]
        self.L = qd(self.L2).sqrt()
        ex = [c / self.L for c in d]
        yr = spec["y_reference"]
        yref = [K.exact(yr["x"]), K.exact(yr["y"]), K.exact(yr["z"])]
        yp = K.vsub(yref, K.vscale(ex, K.vdot(yref, ex)))
        ey = K.vunit(yp)
        ez = K.vcross(ex, ey)
        self.R = [ex, ey, ez]
        self.EA = self.E * A
        self.EI = self.E * I
        self.GJ = self.G * self.sec["J"]
        self.m = self.riQ / self.roQ
        self.nuQ = self.EQ / (2 * self.GQ) - 1
        self.kappaQ = kappa_moduli(self.m, self.EQ, self.GQ)
        self.kappa = qd(self.kappaQ)
        self.As_shear = self.kappa * A
        self.phiQ = 3 * self.EQ * (self.roQ ** 2 + self.riQ ** 2) / (self.kappaQ * self.GQ * self.L2)
        self.theory = theory
        self.phi_timoshenko = qd(self.phiQ)
        self.phi = self.phi_timoshenko if theory == TIMOSHENKO else ZERO
        self.k_local = K.local_stiffness(self.EA, self.GJ, self.EI, self.EI, self.L, self.phi, self.phi)
        self.admitted, self.refusal_reasons = admission(self.od, self.wall, self.mill, self.Ef, self.Gf, self.L2)
        self.tau_rows = nu_in_domain(self.Ef, self.Gf)
        xQ = self.EQ / (2 * self.GQ)                      # 1 + nu, exactly
        self.tau_cQ = ((1 + 2 * xQ) * self.roQ ** 2 + (2 * xQ - 1) * self.riQ ** 2) / (4 * xQ)  # c * I
        self.tau_c = qd(self.tau_cQ) / I

    to_local = K.Member.to_local
    to_global = K.Member.to_global
    global_stiffness = K.Member.global_stiffness


def parse_materials(model):
    out = {}
    for m in model["materials"]:
        alpha = K.exact(q_si(m["thermal_expansion_coefficient"], {"1/degC"})) if "thermal_expansion_coefficient" in m else None
        E = q_si(m["elastic_modulus"], {"Pa"})
        G = q_si(m["shear_modulus"], {"Pa"})
        if not (E > 0 and G > 0 and math.isfinite(E) and math.isfinite(G)):
            raise ValueError("moduli must be positive and finite")
        out[m["id"]] = {"E": E, "G": G, "alpha": alpha}
    return out


def theory_of(model):
    ms = model.get("model_settings") or {}
    if "shear_model" not in ms:
        return EULER
    th = ms["shear_model"]
    if th not in (EULER, TIMOSHENKO):
        raise ValueError("unknown shear model")
    return th


class Reference:
    """The exact reference for one PF request (all its load cases)."""

    def __init__(self, request, theory=None):
        model = request["model"]
        if model.get("schema_version") != "0.8.0":
            raise ValueError("model document 0.8.0 expected")
        if "pressure_contract" in model:
            raise ValueError("a PF document has no pressure_contract")
        if model.get("components"):
            raise ValueError("components are outside the engine (straight members only)")
        if model.get("combinations"):
            raise ValueError("combinations are outside the judged set")
        self.theory = theory or theory_of(model)
        self.nodes, self.node_order = {}, []
        for n in model["nodes"]:
            p = n["position"]
            f = [float(p["x"]), float(p["y"]), float(p["z"])]
            self.nodes[n["id"]] = {"f": f, "d": [K.exact(c) for c in f]}
            self.node_order.append(n["id"])
        self.index = {n: k for k, n in enumerate(self.node_order)}
        self.materials = parse_materials(model)
        self.members = [Member(p, self.nodes, self.materials, self.theory) for p in model["pipe_segments"]]
        self.member_by_id = {m.id: m for m in self.members}
        self.supports = model["supports"]
        self.admitted = self.theory != TIMOSHENKO or all(m.admitted for m in self.members)
        self.cases = {}
        for case in model["load_cases"]:
            self.cases[case["id"]] = self._solve_case(case)

    def point(self, nid):
        return self.nodes[nid]["d"]

    def _solve_case(self, case):
        n = 6 * len(self.node_order)
        nodal = [ZERO] * n
        member_w = {m.id: [ZERO, ZERO, ZERO] for m in self.members}
        strains = {m.id: ZERO for m in self.members}
        motion = {}
        sup_by_id = {s["id"]: s for s in self.supports}
        for pl in case.get("primitive_loads", []):
            cat = pl["category"]
            if cat == "concentrated_force":
                d = DIRS[pl["direction"]]
                assert pl["target"]["type"] == "node" and d < 3
                nodal[6 * self.index[pl["target"]["node"]] + d] += K.exact(q_si(pl["magnitude"], {"N"}))
            elif cat == "concentrated_moment":
                d = DIRS[pl["direction"]]
                assert pl["target"]["type"] == "node" and d >= 3
                nodal[6 * self.index[pl["target"]["node"]] + d] += K.exact(q_si(pl["magnitude"], {"N*m"}))
            elif cat in ("distributed_force", "weight"):
                assert pl["target"]["type"] == "element" and pl.get("dimension") == "force_per_length"
                if "extent" in pl or "span" in pl:
                    raise ValueError("partial-span loads are refused on a PF Timoshenko member")
                d = DIRS[pl["direction"]]
                assert d < 3
                member_w[pl["target"]["pipe"]][d] += K.exact(q_si(pl["magnitude"], {"N/m"}))
            elif cat == "thermal":
                m = self.member_by_id[pl["target"]["pipe"]]
                if m.alpha is None:
                    raise ValueError("thermal load without alpha")
                strains[m.id] += m.alpha * K.exact(q_si(pl["magnitude"], {"degC"}))
            elif cat == "imposed_displacement":
                t = pl["target"]
                assert t["type"] == "support" and t["dof"] == pl["direction"]
                d = DIRS[t["dof"]]
                s = sup_by_id[t["support"]]
                if s["family"] not in ("anchor", "guide") or t["dof"] not in s["restraints"]:
                    raise ValueError("imposed displacement on a DOF the support does not restrain")
                key = (s["id"], d)
                if key in motion:
                    raise ValueError("two imposed displacements on one DOF")
                motion[key] = K.exact(q_si(pl["magnitude"], {"m"} if d < 3 else {"rad"}))
            else:
                raise ValueError(f"load category {cat} is outside the engine")
        K_ = [[ZERO] * n for _ in range(n)]
        f = nodal[:]
        feqs = {}
        for m in self.members:
            kg = m.global_stiffness()
            w_loc = m.to_local(member_w[m.id])
            feq = [a + b for a, b in zip(K.uniform_feq_local(w_loc, m.L), K.strain_feq_local(m.EA, strains[m.id]))]
            feqs[m.id] = (feq, w_loc)
            dofs = self._dofs(m)
            fg = []
            for blk in range(4):
                fg += m.to_global(feq[3 * blk:3 * blk + 3])
            for r in range(12):
                f[dofs[r]] += fg[r]
                Kr, kr = K_[dofs[r]], kg[r]
                for c in range(12):
                    Kr[dofs[c]] += kr[c]
        restrained, springs = {}, []
        for s in self.supports:
            fam = s.get("family")
            if fam in ("anchor", "guide"):
                for dof in s["restraints"]:
                    g = 6 * self.index[s["node"]] + DIRS[dof]
                    if g in restrained:
                        raise ValueError("two restraints on one DOF")
                    restrained[g] = (s["id"], motion.get((s["id"], DIRS[dof]), ZERO))
            elif fam == "spring":
                st = s["stiffness"]
                dof = DIRS[st["dof"]]
                k = K.exact(q_si(st["value"], {"N/m"} if dof < 3 else {"N*m/rad"}))
                g = 6 * self.index[s["node"]] + dof
                K_[g][g] += k
                springs.append((s["id"], g, k))
            else:
                raise ValueError(f"support family {fam} is outside the engine")
        free = [g for g in range(n) if g not in restrained]
        uc = {g: v[1] for g, v in restrained.items()}
        rhs = [f[g] - sum((K_[g][c] * uc[c] for c in uc), ZERO) for g in free]
        Kff = [[K_[r][c] for c in free] for r in free]
        uf = K.solve_dense(Kff, rhs) if free else []
        u = [ZERO] * n
        for g, v in zip(free, uf):
            u[g] = v
        for g, v in uc.items():
            u[g] = v
        out = {"u": u, "free": free, "Kff": Kff, "members": {}, "reactions": {}, "region_of": {}}
        node_force = [ZERO] * n
        for m in self.members:
            feq, w_loc = feqs[m.id]
            dofs = self._dofs(m)
            dl = []
            for blk in range(4):
                dl += m.to_local([u[g] for g in dofs[3 * blk:3 * blk + 3]])
            fe = [a - b for a, b in zip(K.matvec(m.k_local, dl), feq)]
            fg = []
            for blk in range(4):
                fg += m.to_global(fe[3 * blk:3 * blk + 3])
            for r in range(12):
                node_force[dofs[r]] += fg[r]
            out["members"][m.id] = {"f": fe, "w": w_loc, "d_local": dl}
        for s in self.supports:
            base = 6 * self.index[s["node"]]
            comps = [ZERO] * 6
            if s["family"] in ("anchor", "guide"):
                for dof in s["restraints"]:
                    g = base + DIRS[dof]
                    comps[DIRS[dof]] = node_force[g] - nodal[g]
            else:
                g = base + DIRS[s["stiffness"]["dof"]]
                k = [kk for (sid, gg, kk) in springs if sid == s["id"]][0]
                comps[DIRS[s["stiffness"]["dof"]]] = -k * u[g]
            out["reactions"][s["id"]] = comps
        self._check_balance(out, nodal, feqs)
        return out

    def _dofs(self, m):
        return [6 * self.index[m.i] + k for k in range(6)] + [6 * self.index[m.j] + k for k in range(6)]

    def _check_balance(self, out, nodal, feqs):
        """kd5t_engine's whole-body balance on the exact node coordinates."""
        shim = _BalanceShim(self)
        K.Reference._check_balance(shim, out, nodal, feqs)

    # ------------------------------------------------------------- recovery
    cut = K.Reference.cut
    station_fraction = K.Reference.station_fraction

    def max_circular(self, case_id, mid, samples=96, iters=170):
        """Continuous maximum of |N/A| + hypot(M_y, M_z)/Z over the member (Pa), PF's single section."""
        return K.Reference.max_circular(_MaxShim(self, self.member_by_id[mid]), case_id, mid, "single", samples, iters)

    def rows(self, case_id):
        """Every expected published row: (kind, entity, location, component, None) -> Decimal (published unit)."""
        case = self.cases[case_id]
        u = case["u"]
        out = {}
        mm = D(1000)
        MPa = D(10) ** 6
        for nid in self.node_order:
            b = 6 * self.index[nid]
            for k, ax in enumerate("xyz"):
                out[(f"global_nodal_displacement_{ax}", nid, "node", f"nodal_displacement_{ax}", None)] = u[b + k] * mm
                out[(f"global_nodal_rotation_{ax}", nid, "node", f"nodal_rotation_{ax}", None)] = u[b + 3 + k]
            out[("displacement_magnitude", nid, None, None, None)] = K.vnorm(u[b:b + 3]) * mm
        for s in self.supports:
            r = case["reactions"][s["id"]]
            for k, c in enumerate(["Fx", "Fy", "Fz", "Mx", "My", "Mz"]):
                out[("support_reaction_component_v2", s["id"], "node", c, None)] = r[k]
            out[("support_reaction_force_magnitude_v2", s["id"], "node", "force_magnitude", None)] = K.vnorm(r[:3])
            out[("support_reaction_moment_magnitude_v2", s["id"], "node", "moment_magnitude", None)] = K.vnorm(r[3:])
        names = [("element_local_axial_force", "axial_force", 0, "F"), ("element_local_shear_force_y", "shear_force_y", 1, "F"),
                 ("element_local_shear_force_z", "shear_force_z", 2, "F"), ("element_local_torsional_moment", "torsional_moment", 0, "M"),
                 ("element_local_bending_moment_y", "bending_moment_y", 1, "M"), ("element_local_bending_moment_z", "bending_moment_z", 2, "M")]
        for m in self.members:
            f = case["members"][m.id]["f"]
            sec = m.sec
            for loc, k in LOCATIONS:
                F, M = self.cut(case_id, m.id, self.station_fraction(k))
                act = (f[0:3], f[3:6]) if loc == "end_i" else (f[6:9], f[9:12]) if loc == "end_j" else (F, M)
                for kind, comp, idx, fam in names:
                    out[(kind, m.id, loc, comp, None)] = (act[0] if fam == "F" else act[1])[idx]
                out[("element_local_axial_normal_stress", m.id, loc, "axial_normal_stress", None)] = F[0] / sec["A"] / MPa
                out[("element_local_bending_normal_stress_y", m.id, loc, "bending_normal_stress_y", None)] = M[1] / sec["Z"] / MPa
                out[("element_local_bending_normal_stress_z", m.id, loc, "bending_normal_stress_z", None)] = M[2] / sec["Z"] / MPa
                out[("element_local_torsional_shear_stress", m.id, loc, "torsional_shear_stress", None)] = M[0] * sec["ro"] / sec["J"] / MPa
                # derived rows: sigma+- from the station's stress rows, tau_v,max from its shear
                sN, h = F[0] / sec["A"] / MPa, (M[1] * M[1] + M[2] * M[2]).sqrt() / sec["Z"] / MPa
                out[("element_local_normal_stress_extreme", m.id, loc, "sigma_plus", None)] = sN + h
                out[("element_local_normal_stress_extreme", m.id, loc, "sigma_minus", None)] = sN - h
                if m.tau_rows:
                    V = act[0]
                    out[("element_local_transverse_shear_stress_maximum", m.id, loc, "tau_v_max", None)] = (V[1] * V[1] + V[2] * V[2]).sqrt() * m.tau_c / MPa
            for loc, a in (("end_i", 0), ("end_j", 6)):
                Fg, Mg = m.to_global(f[a:a + 3]), m.to_global(f[a + 3:a + 6])
                for k, c in enumerate("xyz"):
                    out[("element_global_end_force_v1", m.id, loc, "F" + c, None)] = Fg[k]
                    out[("element_global_end_moment_v1", m.id, loc, "M" + c, None)] = Mg[k]
        return out

    def expected_warnings(self):
        """Warnings the design names for this document (the tau domain rule)."""
        return sorted({"TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN"} if any(not m.tau_rows for m in self.members) else set())

    def maxima(self, case_id):
        return {m.id: self.max_circular(case_id, m.id) for m in self.members}

    def section_record(self):
        out = {}
        for m in self.members:
            out[m.id] = {"od": repr(m.od), "wall": repr(m.wall), "mill": repr(m.mill), "E": repr(m.Ef), "G": repr(m.Gf),
                         "ro": K.dstr(m.sec["ro"]), "ri": K.dstr(m.sec["ri"]), "A": K.dstr(m.sec["A"]), "I": K.dstr(m.sec["I"]),
                         "J": K.dstr(m.sec["J"]), "Z": K.dstr(m.sec["Z"]), "L": K.dstr(m.L),
                         "nu": qtext(m.nuQ), "E_over_2G": qtext(m.EQ / (2 * m.GQ)), "kappa_E": qtext(m.kappaQ),
                         "shear_area": K.dstr(m.As_shear), "tau_c": K.dstr(m.tau_c), "phi_timoshenko": qtext(m.phiQ), "phi_used": K.dstr(m.phi),
                         "exact_nu_in_domain": -1 < m.nuQ < Q(1, 2), "admitted": m.admitted, "refusal_reasons": m.refusal_reasons}
        return out


def qtext(x: Q, digits=30):
    """A rational as a decimal string (digits significant)."""
    return K.dstr(qd(x), digits)


class _BalanceShim:
    """Presents the PF reference to kd5t_engine's balance check (Decimal node points)."""

    def __init__(self, ref):
        self.nodes = {n: ref.nodes[n]["d"] for n in ref.node_order}
        self.node_order = ref.node_order
        self.supports = ref.supports
        self.members = [_MemberShim(m) for m in ref.members]


class _MemberShim:
    def __init__(self, m):
        self.i, self.j, self.L, self.R, self.id = m.i, m.j, m.L, m.R, m.id

    to_global = K.Member.to_global


class _MaxShim:
    """Presents one PF member's single section to kd5t_engine.max_circular."""

    def __init__(self, ref, m):
        self.member_by_id = {m.id: _SecShim(m)}
        self.cases = ref.cases
        self._ref = ref

    def cut(self, case_id, mid, frac):
        return self._ref.cut(case_id, mid, frac)


class _SecShim:
    def __init__(self, m):
        self.sec = {"single": m.sec}
        self.L = m.L


# ------------------------------------------------------------- self check
def self_check(verbose=True):
    """The kd5t_engine self-check (element controls, unchanged), plus the PF-specific controls."""
    checks = []

    def ok(name, cond):
        checks.append((name, bool(cond)))
        if not cond:
            raise AssertionError(name)

    n0 = len(K.self_check(verbose=False))
    ok(f"kd5t_engine self-check ({n0} checks: SHEAR_REFERENCE S-A, S-B, S-C, S-E, rigid modes, balance)", n0 > 0)
    # the moduli form equals SHEAR_REFERENCE's (E, nu) form for every m and E/(2G) on a grid
    grid_x = [Q(3, 2) - Q(1, 10 ** 12), Q(1), Q(1, 2), Q(1, 3), Q(1, 2 ** 54) + Q(1, 2 ** 60), Q(1, 10 ** 17), Q(1, 10 ** 160), Q(5, 4), Q(4, 9)]
    for m in (Q(1, 3), Q(9, 10), Q(3, 5), Q(91, 100), Q(1, 100), Q(999, 1000)):
        for x in grid_x:
            E, G = Q(1), 1 / (2 * x)
            nu = E / (2 * G) - 1
            ok(f"moduli form = kappa_energy at m={m}, E/(2G)={float(x):.3g}", kappa_moduli(m, E, G) == K.kappa_energy(m, nu))
            closed = 6 * (1 + nu) ** 2 * (1 + m * m) ** 2 / ((7 + 14 * nu + 8 * nu * nu) * (1 + m * m) ** 2 + 4 * m * m * (5 + 10 * nu + 4 * nu * nu))
            ok(f"moduli form = SHEAR_REFERENCE closed form at m={m}, E/(2G)={float(x):.3g}", kappa_moduli(m, E, G) == closed)
    # t4_u8's frozen kappa_E(9/10, 3/10) = 16609827/33068756, from the moduli G = E/(2(1 + 3/10))
    ok("S-B kappa_E(9/10, nu = 3/10) = 16609827/33068756 from E and G = E/2.6", kappa_moduli(Q(9, 10), Q(1), Q(1) / Q(26, 10)) == Q(16609827, 33068756))
    # pf_timoshenko's table: OD 0.2, t 0.01, E 200 GPa, G 77 GPa: nu = 23/77 exactly for the binary64 inputs,
    # kappa_E of the exact binary64 annulus within 1e-15 of 1965660000/3913460969 (the decimal-input annulus)
    req = probe_request("cant", TIMOSHENKO)
    ref = Reference(req)
    mbr = ref.members[0]
    ok("PF S-B: nu = 23/77 exactly from E = 200 GPa, G = 77 GPa", mbr.nuQ == Q(23, 77))
    ok("PF S-B: kappa_E within 1e-15 of pf_timoshenko's 1965660000/3913460969", abs(mbr.kappaQ / Q(1965660000, 3913460969) - 1) < Q(1, 10 ** 15))
    # the auxetic phi = 2 control (pf_timoshenko): OD 9/32, t 3/32, L 7/32, E 8 GPa, G 9 GPa: nu = -5/9, kappa_E = 30/49, phi = 2
    req = probe_request("auxetic", TIMOSHENKO)
    ref = Reference(req)
    mbr = ref.members[0]
    ok("auxetic control: nu = -5/9, kappa_E = 30/49, phi = 2 exactly", mbr.nuQ == Q(-5, 9) and mbr.kappaQ == Q(30, 49) and mbr.phiQ == 2)
    # Codex's case: tip restrained except UY; UY = F/(12 E I/(L^3 (1 + phi))) exactly; Codex's energy value 8.31392979802745e-12 m
    req = probe_request("codex", TIMOSHENKO)
    ref = Reference(req)
    mbr = ref.members[0]
    uy = ref.cases["case:1"]["u"][6 + 1]
    closed = K.exact(1e-18) * mbr.L ** 3 * (1 + mbr.phi) / (12 * mbr.EI)
    energy = K.exact(1e-18) * (mbr.L ** 3 / (12 * mbr.EI) + mbr.L / (mbr.G * mbr.As_shear))
    ok("Codex: UY equals the guided-cantilever closed form F L^3 (1 + phi)/(12 E I)", abs(uy / closed - 1) < D("1e-90"))
    ok("Codex: UY equals the energy form F (L^3/(12 E I) + L/(G A_s))", abs(uy / energy - 1) < D("1e-90"))
    ok(f"Codex: UY = {K.dstr(uy, 17)} m rounds to Codex's 8.31392979802745e-12 m (15 digits)", f"{float(uy):.14e}" == "8.31392979802745e-12")
    ok("Codex: admitted (E/(2G) = 1/(3 2^52) > 2^-54)", ref.admitted and mbr.EQ / (2 * mbr.GQ) == Q(1, 3 * 2 ** 52))
    # admission edges: E/(2G) = 2^-54 refused, the next binary64 G below admitted; t*t overflow named
    for G, want in ((2.0 ** 53, False), (math.nextafter(2.0 ** 53, 0.0), True)):
        adm, why = admission(0.2, 0.01, 0.0, 1.0, G, Q(1))
        ok(f"admission at E = 1, G = {G!r}: {'admitted' if want else 'refused (nu rounds to -1)'}", adm == want)
    adm, why = admission(0.2, 0.01, 0.0, 1.0, 1e300, Q(1))
    ok("admission at E/(2G) = 5e-301: refused, t*t overflow named", not adm and "t*t overflows" in why)
    for E, want in ((3.0, False), (math.nextafter(3.0, 0.0), True)):
        adm, _ = admission(0.2, 0.01, 0.0, E, 1.0, Q(1))
        ok(f"admission at E/(2G) = {E / 2!r}: {'admitted' if want else 'refused (nu = 1/2)'}", adm == want)
    # Euler-Bernoulli twin: the same document without the member gives phi = 0 and the EB tip deflection
    req = probe_request("cant", None)
    ref = Reference(req)
    mbr = ref.members[0]
    uy = ref.cases["case:1"]["u"][6 + 1]
    ok("EB twin (member absent): tip UY = F L^3/(3 E I)", mbr.phi == 0 and abs(uy / (D(100) * mbr.L ** 3 / (3 * mbr.EI)) - 1) < D("1e-90"))
    ref = Reference(probe_request("cant", TIMOSHENKO))
    mbr = ref.members[0]
    uy = ref.cases["case:1"]["u"][6 + 1]
    ok("Timoshenko cantilever: tip UY = F L^3/(3 E I) + F L/(kappa_E G A)",
       abs(uy / (D(100) * mbr.L ** 3 / (3 * mbr.EI) + D(100) * mbr.L / (mbr.G * mbr.As_shear)) - 1) < D("1e-90"))
    # mill tolerance: the single PF section is t - c_m (wall 3/256, c_m 1/512: t - c_m = 5/512 exactly in binary64);
    # a pair whose binary64 difference is inexact (0.0125 - 0.0025) is refused
    req = probe_request("cant", TIMOSHENKO)
    sec = req["model"]["pipe_segments"][0]["section"]
    sec["wall_thickness"]["value"], sec["mill_tolerance"] = 3 / 256, {"value": 1 / 512, "unit": "m"}
    ref = Reference(req)
    ok("PF section: wall 3/256 with c_m 1/512 is the annulus r_i = OD/2 - 5/512", ref.members[0].riQ == Q(0.2) / 2 - Q(5, 512))
    sec["wall_thickness"]["value"], sec["mill_tolerance"] = 0.0125, {"value": 0.0025, "unit": "m"}
    try:
        Reference(req)
        refused = False
    except ValueError:
        refused = True
    ok("PF section: an inexact binary64 t - c_m (0.0125 - 0.0025) is refused by the engine", refused)
    # tau_v,max: c formed without 1 + nu equals the (E, nu) form; the global end rows rotate the local ones back
    mbr = Reference(probe_request("cant", TIMOSHENKO)).members[0]
    nu = mbr.nuQ
    ok("tau constant: ((1+2x) r_o^2 + (2x-1) r_i^2)/(4x) = ((3+2nu) r_o^2 + (1+2nu) r_i^2)/(4(1+nu))",
       mbr.tau_cQ == ((3 + 2 * nu) * mbr.roQ ** 2 + (1 + 2 * nu) * mbr.riQ ** 2) / (4 * (1 + nu)))
    ok("tau rows exist for nu_d in (-1, 1/2) only", mbr.tau_rows and not nu_in_domain(1.0, 2.0 ** 53) and nu_in_domain(1.0, math.nextafter(2.0 ** 53, 0.0)))
    ref = Reference(probe_request("cant", None))
    rows = ref.rows("case:1")
    ok("global end rows at end_i of an x-aligned member equal the local end rows (y_reference +Y)",
       all(rows[("element_global_end_force_v1", "pipe:AB", "end_i", "F" + c, None)] == rows[(k, "pipe:AB", "end_i", comp, None)]
           for c, k, comp in (("x", "element_local_axial_force", "axial_force"), ("y", "element_local_shear_force_y", "shear_force_y"),
                              ("z", "element_local_shear_force_z", "shear_force_z"))))
    # imposed displacement on a guide DOF: the reaction balances, the DOF takes the value
    req = probe_request("imposed", None)
    ref = Reference(req)
    ok("imposed displacement: the prescribed DOF takes its value", ref.cases["case:1"]["u"][6 + 1] == K.exact(0.002))
    if verbose:
        for name, good in checks:
            print(("PASS " if good else "FAIL ") + name)
    return checks


def probe_request(kind, theory):
    """Small invented PF requests for the self-check (and, Euler-Bernoulli only, the format probes)."""
    q = lambda v, u: {"value": v, "unit": u}
    prov = "invented PF M3-T engine self-check"
    L, od, wall, E, G, F = 1.0, 0.2, 0.01, 2.0e11, 7.7e10, 100.0
    tip_restraints = None
    if kind == "auxetic":
        L, od, wall, E, G = 7 / 32, 9 / 32, 3 / 32, 8.0e9, 9.0e9
    elif kind == "codex":
        E, G, F = 1.0e9, 6755399441055744.0e9, 1e-18
        tip_restraints = ["UX", "UZ", "RX", "RY", "RZ"]
    model = {"schema_version": "0.8.0", "document_kind": "openpipestress.product_preview.model",
             "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing",
                                 "professional_acceptance": "not_provided"},
             "project": {"id": f"invented:pf-m3t:selfcheck-{kind}",
                         "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
             "nodes": [{"id": "node:A", "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": prov},
                       {"id": "node:B", "position": {"x": L, "y": 0.0, "z": 0.0}, "provenance": prov}],
             "pipe_segments": [{"id": "pipe:AB", "from": "node:A", "to": "node:B", "material": "mat:A", "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0},
                                "section": {"outside_diameter": q(od, "m"), "wall_thickness": q(wall, "m")}, "provenance": prov}],
             "materials": [{"id": "mat:A", "elastic_modulus": q(E, "Pa"), "shear_modulus": q(G, "Pa"), "provenance": prov}],
             "supports": [{"id": "support:A", "node": "node:A", "family": "anchor", "restraints": DOFS[:], "provenance": prov}],
             "components": [],
             "load_cases": [{"id": "case:1", "kind": "primitive_user_load", "primitive_loads": [
                 {"id": "load:1", "category": "concentrated_force", "target": {"type": "node", "node": "node:B"}, "direction": "global_y",
                  "magnitude": q(F, "N"), "dimension": "force", "provenance": prov}], "provenance": prov}],
             "combinations": []}
    if theory is not None:
        model["model_settings"] = {"shear_model": theory}
    if tip_restraints:
        model["supports"].append({"id": "support:B", "node": "node:B", "family": "anchor", "restraints": tip_restraints, "provenance": prov})
    if kind == "imposed":
        model["supports"].append({"id": "support:B", "node": "node:B", "family": "guide", "restraints": ["UY"], "provenance": prov})
        model["load_cases"][0]["primitive_loads"] = [
            {"id": "load:1", "category": "imposed_displacement", "target": {"type": "support", "support": "support:B", "dof": "UY"},
             "direction": "UY", "magnitude": q(0.002, "m"), "dimension": "displacement", "provenance": prov}]
    return {"model": model, "materials": []}


if __name__ == "__main__":
    if "--check" in sys.argv:
        n = len(self_check())
        print(f"PF engine self-check: {n} checks pass")
