"""M3-T independent exact reference engine (standard-library Python only).

It imports and reads no product code. Its input is a public product request
(model document 0.8.0, or the historical 0.5.0 of the census corpora, under
`3.0.0/exact_pressure_v3`: the two spell these models identically but for the version); every binary64 number in
the request is taken as its exact value. Arithmetic is Python `decimal` at
PREC (default 110) significant digits; pi is computed by Machin's formula.
Rational element controls (SHEAR_REFERENCE S-A, S-E) run the same element
functions on `fractions.Fraction` (see `self_check`).

Mechanics (invented models; small displacement, linear elastic):

- straight two-node 3D frame members, 12x12 element in the local frame
  x = (x_j - x_i)/L, y = y_reference projected normal to x, z = x cross y;
- axial EA/L and torsion GJ/L on the nominal section (G = E/(2(1+nu)));
- two bending planes (v = u_y, theta = r_z with I_z; v = u_z, theta = -r_y
  with I_y), Euler-Bernoulli (phi = 0) or Timoshenko with
  phi = 12 E I/(K_s L^2), K_s = kappa_E G A_n and the energy-matched annular
  factor kappa_E of SHEAR_REFERENCE CONTRACT section 1 / section-bases Design
  section 3.1, from the nominal section of the authored OD, wall and nu:
      k_vv = 12EI/(L^3(1+phi)), k_vt = 6EI/(L^2(1+phi)),
      k_tt = (4+phi)EI/(L(1+phi)), k_tt' = (2-phi)EI/(L(1+phi));
- loads: nodal forces and moments; full-span uniform member loads
  (consistent vector [wL/2, wL^2/12, wL/2, -wL^2/12], identical for both
  theories, SHEAR_REFERENCE section 4); axial free strain (thermal alpha*dT
  or an explicit strain) as the end pair -/+ E A_n eps; pressure regions of
  collinear straight members: caps p*Ai outward at transferring terminals,
  kink forces p*Ai*(sum of outward member tangents) at interior region nodes
  and the Poisson free strain -2 nu p Ai/(E A_n) (T4-I8/T4-U4a wall
  formulation; section-bases Design section 1: Ai on the pressure bore, A_n
  the stiffness area);
- supports: anchors (any restrained DOF set, prescribed values from a
  load-reference boundary motion, else zero) and grounded linear springs;
- recovery: end actions f = K d - feq (node-on-element), the j-side section
  action at the stations 0, .25, .5, .75, 1 by statics from the i-end action
  plus the load integral, support-on-pipe reactions, and stresses on both D-5
  bases (section-bases Design sections 1 and 2.1).

D-5 section bases (section-bases Design section 1), for OD D, wall t, mill
tolerance c_m and allowance c_a, all binary64 inputs:
- nominal: wall t, r_i,n = D/2 - t (exact);
- reduced: wall t_r = fl(fl(t - c_m) - c_a) (the fixed binary64 order of the
  Design), r_i,r = D/2 - t_r (exact);
- pressure bore: r_i,r, Ai = pi r_i,r^2.
A, I, J = 2I, Z = I/r_o of each basis are exact functions of its wall.

Published units: translations mm, rotations rad, forces N, moments N*m,
element-local stresses MPa, pipe_* stresses Pa.
"""

from __future__ import annotations

import math
from decimal import Decimal, getcontext
from fractions import Fraction

PREC = 110
getcontext().prec = PREC
D = Decimal
ZERO = D(0)
ONE = D(1)
TWO = D(2)

SI_UNITS = {"m", "N", "N*m", "N/m", "Pa", "degC", "1/degC", "1", "rad", "N*m/rad"}
LOCATIONS = [("end_i", 0), ("quarter_1", 1), ("midspan", 2), ("quarter_3", 3), ("end_j", 4)]
BASES = ("nominal_section", "reduced_section")
DOFS = ["UX", "UY", "UZ", "RX", "RY", "RZ"]
DIRS = {"global_x": 0, "global_y": 1, "global_z": 2, "rotation_x": 3, "rotation_y": 4, "rotation_z": 5,
        "UX": 0, "UY": 1, "UZ": 2, "RX": 3, "RY": 4, "RZ": 5}


def exact(x) -> Decimal:
    """The exact value of a binary64 input (Decimal(float) is exact)."""
    return D(float(x))


def _machin_pi() -> Decimal:
    getcontext().prec = PREC + 20

    def atan_inv(n):
        x = D(1) / n
        x2 = x * x
        total, term, k = ZERO, x, 1
        eps = D(10) ** (-(PREC + 25))
        while abs(term) > eps:
            total += term / k if (k // 2) % 2 == 0 else -term / k
            term *= x2
            k += 2
        return total

    pi = 16 * atan_inv(5) - 4 * atan_inv(239)
    getcontext().prec = PREC
    return +pi


PI = _machin_pi()


def b64_sub(a: float, b: float) -> float:
    """binary64 a - b (Python floats are IEEE binary64, round to nearest)."""
    return float(a) - float(b)


# ------------------------------------------------------------------ vectors
def vadd(a, b):
    return [a[0] + b[0], a[1] + b[1], a[2] + b[2]]


def vsub(a, b):
    return [a[0] - b[0], a[1] - b[1], a[2] - b[2]]


def vscale(a, s):
    return [a[0] * s, a[1] * s, a[2] * s]


def vdot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def vcross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def vnorm(a):
    return vdot(a, a).sqrt()


def vunit(a):
    n = vnorm(a)
    return [a[0] / n, a[1] / n, a[2] / n]


# ------------------------------------------------------------ section bases
def annulus(ro, wall):
    ri = ro - wall
    A = PI * (ro * ro - ri * ri)
    I = PI * (ro ** 4 - ri ** 4) / 4
    return {"wall": wall, "ri": ri, "A": A, "I": I, "J": 2 * I, "Z": I / ro}


def kappa_energy(m, nu):
    """SHEAR_REFERENCE's energy-matched annular factor, cancellation-free D (generic in the number type)."""
    m2 = m * m
    a = 1 + m2
    b = 1 - m2
    n1 = 1 + nu
    c = (8 * nu + 14) * nu + 7
    den = c * (b * b) + 48 * (m2 * (n1 * n1))
    return (6 * (n1 * n1)) * (a * a) / den


def kappa_cowper(m, nu):
    m2 = m * m
    return 6 * (1 + nu) * (1 + m2) ** 2 / ((7 + 6 * nu) * (1 + m2) ** 2 + (20 + 12 * nu) * m2)


def d5_sections(od: float, wall: float, mill: float, ca: float):
    ro = exact(od) / 2
    t = exact(wall)
    t_r = b64_sub(b64_sub(wall, mill), ca)          # the Design's fixed binary64 order
    nominal = annulus(ro, t)
    reduced = annulus(ro, exact(t_r))
    bore_ri = ro - exact(t_r)
    return {"ro": ro, "nominal_section": nominal, "reduced_section": reduced,
            "bore": {"ri": bore_ri, "Ai": PI * bore_ri * bore_ri}, "t_r_binary64": t_r}


# ----------------------------------------------------------- element blocks
def bending_block(EI, L, phi):
    """4x4 block for d = [v_i, theta_i, v_j, theta_j] (generic number type).

    phi = 12 EI/(Ks L^2) (0 for Euler-Bernoulli)."""
    c = EI / (L ** 3 * (1 + phi))
    return [[12 * c, 6 * L * c, -12 * c, 6 * L * c],
            [6 * L * c, (4 + phi) * L * L * c, -6 * L * c, (2 - phi) * L * L * c],
            [-12 * c, -6 * L * c, 12 * c, -6 * L * c],
            [6 * L * c, (2 - phi) * L * L * c, -6 * L * c, (4 + phi) * L * L * c]]


def local_stiffness(EA, GJ, EIy, EIz, L, phi_y, phi_z):
    """12x12 local stiffness, DOF order (u, v, w, rx, ry, rz) at i then j."""
    k = [[0 * EA for _ in range(12)] for _ in range(12)]
    a = EA / L
    for (p, q, s) in ((0, 0, 1), (0, 6, -1), (6, 0, -1), (6, 6, 1)):
        k[p][q] = s * a
    t = GJ / L
    for (p, q, s) in ((3, 3, 1), (3, 9, -1), (9, 3, -1), (9, 9, 1)):
        k[p][q] = s * t
    # x-y plane: v = u_y (1, 7), theta = r_z (5, 11)
    bz = bending_block(EIz, L, phi_z)
    idx = [1, 5, 7, 11]
    for r in range(4):
        for c in range(4):
            k[idx[r]][idx[c]] = bz[r][c]
    # x-z plane: v = u_z (2, 8), theta = -r_y (4, 10)
    by = bending_block(EIy, L, phi_y)
    idx = [2, 4, 8, 10]
    sgn = [1, -1, 1, -1]
    for r in range(4):
        for c in range(4):
            k[idx[r]][idx[c]] = sgn[r] * sgn[c] * by[r][c]
    return k


def uniform_feq_local(w, L):
    """Consistent full-span uniform load vector (both theories), local w = (wx, wy, wz)."""
    wx, wy, wz = w
    h = L / 2
    m = L * L / 12
    return [wx * h, wy * h, wz * h, 0 * wx, -wz * m, wy * m,
            wx * h, wy * h, wz * h, 0 * wx, wz * m, -wy * m]


def strain_feq_local(EA, eps):
    z = 0 * EA
    return [-EA * eps, z, z, z, z, z, EA * eps, z, z, z, z, z]


# ------------------------------------------------------------- linear solve
def solve_dense(a, b):
    """Gaussian elimination with partial pivoting (Decimal or Fraction)."""
    n = len(b)
    m = [row[:] + [b[i]] for i, row in enumerate(a)]
    for col in range(n):
        piv = max(range(col, n), key=lambda r: abs(m[r][col]))
        if m[piv][col] == 0:
            raise ZeroDivisionError("singular system")
        m[col], m[piv] = m[piv], m[col]
        pr = m[col]
        inv = 1 / pr[col]
        for r in range(col + 1, n):
            mr = m[r]
            if mr[col] != 0:
                f = mr[col] * inv
                for c in range(col, n + 1):
                    if pr[c] != 0:
                        mr[c] -= f * pr[c]
    x = [0 * b[0]] * n if n else []
    for i in range(n - 1, -1, -1):
        s = m[i][n]
        for c in range(i + 1, n):
            s -= m[i][c] * x[c]
        x[i] = s / m[i][i]
    return x


def matvec(a, x):
    return [sum((ai * xi for ai, xi in zip(row, x)), 0 * x[0]) for row in a]


# --------------------------------------------------------------- the model
def q_si(qty, allowed=None):
    if not isinstance(qty, dict) or "value" not in qty:
        raise ValueError(f"not a quantity: {qty!r}")
    unit = qty.get("unit")
    if unit not in SI_UNITS or (allowed and unit not in allowed):
        raise ValueError(f"unit {unit!r} is outside the engine's SI set")
    return float(qty["value"])


class Member:
    def __init__(self, spec, nodes, materials, theory, e_scale):
        self.id = spec["id"]
        self.i, self.j = spec["from"], spec["to"]
        mat = materials[spec["material"]]
        self.E = mat["E"] * e_scale
        self.nu = mat["nu"]
        self.G = self.E / (2 * (ONE + self.nu))
        self.alpha = mat["alpha"]
        sec = spec["section"]
        self.od = q_si(sec["outside_diameter"], {"m"})
        self.wall = q_si(sec["wall_thickness"], {"m"})
        self.mill = q_si(sec["mill_tolerance"], {"m"}) if "mill_tolerance" in sec else 0.0
        if "corrosion_allowance" not in sec:
            raise ValueError("exact-contract section without corrosion_allowance")
        self.ca = q_si(sec["corrosion_allowance"], {"m"})
        self.sec = d5_sections(self.od, self.wall, self.mill, self.ca)
        nom = self.sec["nominal_section"]
        xi, xj = nodes[self.i], nodes[self.j]
        d = vsub(xj, xi)
        self.L = vnorm(d)
        ex = [c / self.L for c in d]
        yr = spec["y_reference"]
        yref = [exact(yr["x"]), exact(yr["y"]), exact(yr["z"])]
        yp = vsub(yref, vscale(ex, vdot(yref, ex)))
        ey = vunit(yp)
        ez = vcross(ex, ey)
        self.R = [ex, ey, ez]
        self.EA = self.E * nom["A"]
        self.EI = self.E * nom["I"]
        self.GJ = self.G * nom["J"]
        m = nom["ri"] / self.sec["ro"]
        self.kappa = kappa_energy(m, self.nu)
        self.As_shear = self.kappa * nom["A"]
        self.Ks = self.G * self.As_shear
        self.phi_timoshenko = 12 * self.EI / (self.Ks * self.L * self.L)
        self.theory = theory
        self.phi = self.phi_timoshenko if theory == "timoshenko_annulus_energy_v1" else ZERO
        self.k_local = local_stiffness(self.EA, self.GJ, self.EI, self.EI, self.L, self.phi, self.phi)

    def to_local(self, v):
        return [vdot(self.R[0], v), vdot(self.R[1], v), vdot(self.R[2], v)]

    def to_global(self, v):
        ex, ey, ez = self.R
        return [ex[k] * v[0] + ey[k] * v[1] + ez[k] * v[2] for k in range(3)]

    def global_stiffness(self):
        kl = self.k_local
        R = self.R
        out = [[ZERO] * 12 for _ in range(12)]
        for bi in range(4):
            for bj in range(4):
                blk = [[kl[3 * bi + r][3 * bj + c] for c in range(3)] for r in range(3)]
                # R^T blk R
                tmp = [[sum((blk[r][s] * R[s][c] for s in range(3)), ZERO) for c in range(3)] for r in range(3)]
                for r in range(3):
                    for c in range(3):
                        out[3 * bi + r][3 * bj + c] = sum((R[s][r] * tmp[s][c] for s in range(3)), ZERO)
        return out


def parse_materials(model):
    out = {}
    for m in model["materials"]:
        if m.get("constitutive_basis") != "homogeneous_isotropic_E_nu_v1":
            raise ValueError("the engine admits homogeneous_isotropic_E_nu_v1 materials only")
        alpha = exact(q_si(m["thermal_expansion_coefficient"], {"1/degC"})) if "thermal_expansion_coefficient" in m else None
        out[m["id"]] = {"E": exact(q_si(m["elastic_modulus"], {"Pa"})), "nu": exact(q_si(m["poisson_ratio"], {"1"})), "alpha": alpha}
    return out


class Reference:
    """The exact reference for one request (all its load cases)."""

    def __init__(self, request, theory=None, e_scale=ONE):
        model = request["model"]
        if model.get("schema_version") not in ("0.8.0", "0.5.0"):
            raise ValueError("model document 0.8.0 (or the historical 0.5.0) expected")
        pc = model.get("pressure_contract")
        if not pc or pc.get("version") != "3.0.0" or pc.get("mode") != "exact_pressure_v3":
            raise ValueError("3.0.0/exact_pressure_v3 expected")
        self.theory = theory or pc["shear_model"]
        if self.theory not in ("euler_bernoulli", "timoshenko_annulus_energy_v1"):
            raise ValueError("unknown shear model")
        if model.get("components"):
            raise ValueError("components are outside the engine (straight members only)")
        self.nodes = {}
        self.node_order = []
        for n in model["nodes"]:
            p = n["position"]
            self.nodes[n["id"]] = [exact(p["x"]), exact(p["y"]), exact(p["z"])]
            self.node_order.append(n["id"])
        self.index = {n: k for k, n in enumerate(self.node_order)}
        self.materials = parse_materials(model)
        self.members = [Member(p, self.nodes, self.materials, self.theory, e_scale) for p in model["pipe_segments"]]
        self.member_by_id = {m.id: m for m in self.members}
        self.supports = model["supports"]
        self.cases = {}
        for case in model["load_cases"]:
            self.cases[case["id"]] = self._solve_case(case)

    # ---------------------------------------------------------------- case
    def _solve_case(self, case):
        n = 6 * len(self.node_order)
        state = case.get("analysis_state")
        factors = None
        strains = {m.id: ZERO for m in self.members}
        motion = {}
        if state is not None:
            factors = {s["source_ref"]: exact(s["factor"]) for s in state.get("load_sources", [])}
            for es in state.get("element_states", []):
                ts = es.get("thermal_state", {"kind": "unchanged_reference"})
                if es.get("material_selection", {}).get("kind") != "explicit_base_properties":
                    raise ValueError("engine admits explicit_base_properties material selections only")
                if ts["kind"] == "explicit_interval_strain":
                    strains[es["pipe_ref"]] += exact(q_si(ts["strain"], {"1"}))
                elif ts["kind"] != "unchanged_reference":
                    raise ValueError(f"thermal state {ts['kind']} is outside the engine")
            for ss in state.get("support_states", []):
                for bm in ss.get("boundary_motion", []):
                    if bm.get("meaning") != "absolute_reference_displacement":
                        raise ValueError("boundary motion meaning")
                    motion[(ss["support_ref"], DIRS[bm["dof"]])] = exact(q_si(bm["value"], {"m", "rad"}))
            for m in self.members:
                if not any(r.get("pipe_ref") == m.id for r in state.get("element_states", [])):
                    raise ValueError("load state without an element state for every pipe")
        nodal = [ZERO] * n
        member_w = {m.id: [ZERO, ZERO, ZERO] for m in self.members}
        for pl in case.get("primitive_loads", []):
            if factors is not None:
                if pl["id"] not in factors:
                    continue
                fac = factors[pl["id"]]
            else:
                fac = ONE
            cat = pl["category"]
            if cat == "concentrated_force":
                assert pl["target"]["type"] == "node"
                d = DIRS[pl["direction"]]
                assert d < 3
                nodal[6 * self.index[pl["target"]["node"]] + d] += exact(q_si(pl["magnitude"], {"N"})) * fac
            elif cat == "concentrated_moment":
                d = DIRS[pl["direction"]]
                assert d >= 3
                nodal[6 * self.index[pl["target"]["node"]] + d] += exact(q_si(pl["magnitude"], {"N*m"})) * fac
            elif cat in ("distributed_force", "weight"):
                assert pl["target"]["type"] == "element" and pl.get("dimension") == "force_per_length"
                if "extent" in pl or "span" in pl:
                    raise ValueError("partial-span loads are outside the exact route")
                d = DIRS[pl["direction"]]
                member_w[pl["target"]["pipe"]][d] += exact(q_si(pl["magnitude"], {"N/m"})) * fac
            elif cat == "thermal":
                if factors is not None:
                    raise ValueError("primitive thermal load in a load-state case is outside the engine")
                m = self.member_by_id[pl["target"]["pipe"]]
                if m.alpha is None:
                    raise ValueError("thermal load without alpha")
                strains[m.id] += m.alpha * exact(q_si(pl["magnitude"], {"degC"}))
            else:
                raise ValueError(f"load category {cat} is outside the engine")
        # pressure regions (collinear straight members)
        region_of = {}
        poisson = {m.id: ZERO for m in self.members}
        for reg in case.get("pressure_regions", []):
            if reg.get("pressure_basis") != "internal_differential_zero_external_v1":
                raise ValueError("pressure basis")
            if reg.get("member_connector_ids"):
                raise ValueError("connectors in a region are outside the engine")
            p = exact(q_si(reg["pressure"], {"Pa"}))
            pids = reg["member_pipe_ids"]
            ais = {self.member_by_id[pid].sec["bore"]["Ai"] for pid in pids}
            if len(ais) != 1:
                raise ValueError("unequal bores in one region")
            Ai = ais.pop()
            P = p * Ai
            closures = {t["node_ref"]: t["closure_transfer"] for t in reg["terminals"]}
            incidence = {}
            for pid in pids:
                m = self.member_by_id[pid]
                region_of[pid] = {"p": p, "Ai": Ai, "P": P}
                poisson[pid] += -2 * m.nu * P / (m.E * m.sec["nominal_section"]["A"])
                for a, b in ((m.i, m.j), (m.j, m.i)):
                    incidence.setdefault(a, []).append(vunit(vsub(self.nodes[a], self.nodes[b])))
            for node, outs in incidence.items():
                if len(outs) == 1:
                    if node not in closures:
                        raise ValueError("region end without a terminal")
                    if closures[node] != "transfers_to_wall":
                        continue
                elif node in closures:
                    raise ValueError("terminal at an interior region node")
                total = [ZERO, ZERO, ZERO]
                for u in outs:
                    total = vadd(total, u)
                for k in range(3):
                    nodal[6 * self.index[node] + k] += P * total[k]
        eps = {m.id: strains[m.id] + poisson[m.id] for m in self.members}
        # assembly
        K = [[ZERO] * n for _ in range(n)]
        f = nodal[:]
        feqs = {}
        for m in self.members:
            kg = m.global_stiffness()
            w_loc = m.to_local(member_w[m.id])
            feq = [a + b for a, b in zip(uniform_feq_local(w_loc, m.L), strain_feq_local(m.EA, eps[m.id]))]
            feqs[m.id] = (feq, w_loc)
            dofs = [6 * self.index[m.i] + k for k in range(6)] + [6 * self.index[m.j] + k for k in range(6)]
            fg = []
            for blk in range(4):
                fg += m.to_global(feq[3 * blk:3 * blk + 3])
            for r in range(12):
                f[dofs[r]] += fg[r]
                Kr = K[dofs[r]]
                kr = kg[r]
                for c in range(12):
                    Kr[dofs[c]] += kr[c]
        restrained = {}
        springs = []
        for s in self.supports:
            fam = s.get("family")
            if fam == "anchor":
                for dof in s["restraints"]:
                    g = 6 * self.index[s["node"]] + DIRS[dof]
                    if g in restrained:
                        raise ValueError("two restraints on one DOF")
                    restrained[g] = (s["id"], motion.get((s["id"], DIRS[dof]), ZERO))
            elif fam == "spring":
                st = s["stiffness"]
                dof = DIRS[st["dof"]]
                k = exact(q_si(st["value"], {"N/m"} if dof < 3 else {"N*m/rad"}))
                g = 6 * self.index[s["node"]] + dof
                K[g][g] += k
                springs.append((s["id"], g, k))
            else:
                raise ValueError(f"support family {fam} is outside the engine")
        for (sid, d), _ in motion.items():
            if not any(v[0] == sid for v in restrained.values()):
                raise ValueError("boundary motion on a non-anchor support")
        free = [g for g in range(n) if g not in restrained]
        uc = {g: v[1] for g, v in restrained.items()}
        rhs = [f[g] - sum((K[g][c] * uc[c] for c in uc), ZERO) for g in free]
        Kff = [[K[r][c] for c in free] for r in free]
        uf = solve_dense(Kff, rhs) if free else []
        u = [ZERO] * n
        for g, v in zip(free, uf):
            u[g] = v
        for g, v in uc.items():
            u[g] = v
        # recovery
        out = {"u": u, "free": free, "Kff": Kff, "members": {}, "reactions": {}, "region_of": region_of, "eps": eps}
        node_force = [ZERO] * n  # sum of node-on-element end actions (global)
        for m in self.members:
            feq, w_loc = feqs[m.id]
            dofs = [6 * self.index[m.i] + k for k in range(6)] + [6 * self.index[m.j] + k for k in range(6)]
            dg = [u[g] for g in dofs]
            dl = []
            for blk in range(4):
                dl += m.to_local(dg[3 * blk:3 * blk + 3])
            fe = [a - b for a, b in zip(matvec(m.k_local, dl), feq)]
            fg = []
            for blk in range(4):
                fg += m.to_global(fe[3 * blk:3 * blk + 3])
            for r in range(12):
                node_force[dofs[r]] += fg[r]
            out["members"][m.id] = {"f": fe, "w": w_loc, "d_local": dl}
        for s in self.supports:
            base = 6 * self.index[s["node"]]
            comps = [ZERO] * 6
            if s.get("family") == "anchor":
                for dof in s["restraints"]:
                    g = base + DIRS[dof]
                    comps[DIRS[dof]] = node_force[g] - nodal[g]
            else:
                st = s["stiffness"]
                g = base + DIRS[st["dof"]]
                k = [kk for (sid, gg, kk) in springs if sid == s["id"]][0]
                comps[DIRS[st["dof"]]] = -k * u[g]
            out["reactions"][s["id"]] = comps
        self._check_balance(out, nodal, feqs)
        return out

    def _check_balance(self, out, nodal, feqs):
        """Whole-body force and moment balance (about the first node) of reactions, nodal loads and member loads.

        Tolerances are relative to coupled scales SF = max(force terms, moment terms/L_b) and
        SM = max(moment terms, L_b*force terms), so couple-only cases are checked too; if every term
        is zero the sums must be exactly zero."""
        r0 = self.nodes[self.node_order[0]]
        F, M = [ZERO] * 3, [ZERO] * 3
        sf, sm = ZERO, ZERO

        def add(force, moment, at):
            nonlocal F, M, sf, sm
            arm = vsub(at, r0)
            mom = vadd(moment, vcross(arm, force))
            F, M = vadd(F, force), vadd(M, mom)
            sf = max([sf] + [abs(x) for x in force])
            sm = max([sm] + [abs(x) for x in moment] + [abs(x) for x in vcross(arm, force)])

        for s in self.supports:
            r = out["reactions"][s["id"]]
            add(r[:3], r[3:], self.nodes[s["node"]])
        for k, nid in enumerate(self.node_order):
            add(nodal[6 * k:6 * k + 3], nodal[6 * k + 3:6 * k + 6], self.nodes[nid])
        for m in self.members:
            w = m.to_global(feqs[m.id][1])
            mid = vscale(vadd(self.nodes[m.i], self.nodes[m.j]), D("0.5"))
            add(vscale(w, m.L), [ZERO] * 3, mid)
        pts = [self.nodes[n] for n in self.node_order]
        Lb = vnorm([max(p[a] for p in pts) - min(p[a] for p in pts) for a in range(3)])
        SF = max(sf, sm / Lb) if Lb else sf
        SM = max(sm, Lb * sf) if Lb else sm
        tol = D(10) ** (-(PREC - 25))
        if max(abs(x) for x in F) > SF * tol or max(abs(x) for x in M) > SM * tol:
            raise AssertionError(f"whole-body balance fails: F={F} M={M} (SF={SF}, SM={SM})")

    # ------------------------------------------------------------- recovery
    def cut(self, case_id, mid, frac):
        """j-side section action (local F, M) at fraction frac of member mid."""
        m = self.member_by_id[mid]
        rec = self.cases[case_id]["members"][mid]
        f, w = rec["f"], rec["w"]
        x = m.L * frac
        Fi, Mi = f[0:3], f[3:6]
        F = [-Fi[0] - w[0] * x, -Fi[1] - w[1] * x, -Fi[2] - w[2] * x]
        h = x * x / 2
        M = [-Mi[0], -Mi[1] - x * Fi[2] - h * w[2], -Mi[2] + x * Fi[1] + h * w[1]]
        return F, M

    def station_fraction(self, k):
        return [ZERO, D("0.25"), D("0.5"), D("0.75"), ONE][k]

    def max_circular(self, case_id, mid, basis, samples=96, iters=170):
        """Exact-enough continuous maximum of |N_w/A_b| + hypot(M_y, M_z)/Z_b over the member (Pa)."""
        m = self.member_by_id[mid]
        sec = m.sec[basis]

        def g(fr):
            F, M = self.cut(case_id, mid, fr)
            return abs(F[0]) / sec["A"] + (M[1] * M[1] + M[2] * M[2]).sqrt() / sec["Z"]

        xs = [D(k) / samples for k in range(samples + 1)]
        vals = [g(x) for x in xs]
        best = max(vals)
        order = sorted(range(len(xs)), key=lambda k: -vals[k])[:4]
        invphi = (D(5).sqrt() - 1) / 2
        for k in order:
            a = xs[max(k - 1, 0)]
            b = xs[min(k + 1, samples)]
            c = b - invphi * (b - a)
            d = a + invphi * (b - a)
            gc, gd = g(c), g(d)
            for _ in range(iters):
                if gc > gd:
                    b, d, gd = d, c, gc
                    c = b - invphi * (b - a)
                    gc = g(c)
                else:
                    a, c, gc = c, d, gd
                    d = a + invphi * (b - a)
                    gd = g(d)
            best = max(best, gc, gd)
        return best

    def rows(self, case_id):
        """Every expected published row: (kind, entity, location, component, basis) -> Decimal (published unit)."""
        case = self.cases[case_id]
        u = case["u"]
        out = {}
        mm = D(1000)
        for nid in self.node_order:
            b = 6 * self.index[nid]
            for k, ax in enumerate("xyz"):
                out[(f"global_nodal_displacement_{ax}", nid, "node", f"nodal_displacement_{ax}", None)] = u[b + k] * mm
                out[(f"global_nodal_rotation_{ax}", nid, "node", f"nodal_rotation_{ax}", None)] = u[b + 3 + k]
            out[("displacement_magnitude", nid, None, None, None)] = vnorm(u[b:b + 3]) * mm
        for s in self.supports:
            r = case["reactions"][s["id"]]
            for k, c in enumerate(["Fx", "Fy", "Fz", "Mx", "My", "Mz"]):
                out[("support_reaction_component_v2", s["id"], "node", c, None)] = r[k]
            out[("support_reaction_force_magnitude_v2", s["id"], "node", "force_magnitude", None)] = vnorm(r[:3])
            out[("support_reaction_moment_magnitude_v2", s["id"], "node", "moment_magnitude", None)] = vnorm(r[3:])
        names = [("element_local_axial_force", "axial_force", 0, "F"), ("element_local_shear_force_y", "shear_force_y", 1, "F"),
                 ("element_local_shear_force_z", "shear_force_z", 2, "F"), ("element_local_torsional_moment", "torsional_moment", 0, "M"),
                 ("element_local_bending_moment_y", "bending_moment_y", 1, "M"), ("element_local_bending_moment_z", "bending_moment_z", 2, "M")]
        for m in self.members:
            rec = case["members"][m.id]
            f = rec["f"]
            reg = case["region_of"].get(m.id)
            for loc, k in LOCATIONS:
                fr = self.station_fraction(k)
                F, M = self.cut(case_id, m.id, fr)
                if loc == "end_i":
                    act = (f[0:3], f[3:6])
                elif loc == "end_j":
                    act = (f[6:9], f[9:12])
                else:
                    act = (F, M)
                for kind, comp, idx, fam in names:
                    if kind == "element_local_axial_force" and reg is not None:
                        continue
                    out[(kind, m.id, loc, comp, None)] = (act[0] if fam == "F" else act[1])[idx]
                Nw = F[0]
                if reg is not None:
                    out[("pipe_wall_axial_force_v2", m.id, loc, "wall_axial_force", None)] = Nw
                    out[("pipe_effective_axial_force_v2", m.id, loc, "effective_axial_force", None)] = Nw - reg["P"]
                    if loc in ("end_i", "end_j"):
                        out[("pipe_wall_endpoint_action_v2", m.id, loc, "wall_axial_end_action", None)] = -Nw if loc == "end_i" else Nw
                for basis in BASES:
                    sec = m.sec[basis]
                    MPa = D(10) ** 6
                    if reg is None:
                        out[("element_local_axial_normal_stress", m.id, loc, "axial_normal_stress", basis)] = F[0] / sec["A"] / MPa
                    else:
                        p = reg["p"]
                        out[("pipe_axial_membrane_stress_v2", m.id, loc, "axial_membrane_stress", basis)] = Nw / sec["A"]
                        out[("pipe_lame_radial_stress_v2", m.id, loc, "lame_inner_radial_stress", basis)] = -p
                        out[("pipe_lame_radial_stress_v2", m.id, loc, "lame_outer_radial_stress", basis)] = ZERO
                        hoop_o = 2 * p * PI * sec["ri"] * sec["ri"] / sec["A"]
                        out[("pipe_lame_hoop_stress_v2", m.id, loc, "lame_outer_hoop_stress", basis)] = hoop_o
                        out[("pipe_lame_hoop_stress_v2", m.id, loc, "lame_inner_hoop_stress", basis)] = hoop_o + p
                    out[("element_local_bending_normal_stress_y", m.id, loc, "bending_normal_stress_y", basis)] = M[1] / sec["Z"] / MPa
                    out[("element_local_bending_normal_stress_z", m.id, loc, "bending_normal_stress_z", basis)] = M[2] / sec["Z"] / MPa
                    out[("element_local_torsional_shear_stress", m.id, loc, "torsional_shear_stress", basis)] = M[0] * m.sec["ro"] / sec["J"] / MPa
        return out

    def maxima(self, case_id):
        return {(m.id, b): self.max_circular(case_id, m.id, b) for m in self.members for b in BASES}

    def section_record(self):
        out = {}
        for m in self.members:
            rec = {"od": repr(m.od), "wall": repr(m.wall), "mill": repr(m.mill), "ca": repr(m.ca), "ro": dstr(m.sec["ro"]),
                   "t_r_binary64": repr(m.sec["t_r_binary64"]), "L": dstr(m.L), "E": dstr(m.E), "nu": dstr(m.nu), "G": dstr(m.G),
                   "kappa_E": dstr(m.kappa), "shear_area": dstr(m.As_shear), "phi_timoshenko": dstr(m.phi_timoshenko),
                   "phi_used": dstr(m.phi), "pressure_bore": {"ri": dstr(m.sec["bore"]["ri"]), "Ai": dstr(m.sec["bore"]["Ai"])}}
            for b in BASES:
                s = m.sec[b]
                rec[b] = {k: dstr(s[k]) for k in ("wall", "ri", "A", "I", "J", "Z")}
            out[m.id] = rec
        return out


def dstr(x: Decimal, digits: int = 45) -> str:
    """A decimal string of x to `digits` significant digits (exact zero as '0')."""
    if x == 0:
        return "0"
    return format(x, f".{digits - 1}e")


# ------------------------------------------------------------- self check
def self_check(verbose=True):
    """SHEAR_REFERENCE section 5 controls, run on the engine's own element functions."""
    F = Fraction
    checks = []

    def ok(name, cond):
        checks.append((name, bool(cond)))
        if not cond:
            raise AssertionError(name)

    # S-A: EI = 1, Ks = 12, L = 1, phi = 1 (abstract element)
    EI, Ks, L = F(1), F(12), F(1)
    phi = 12 * EI / (Ks * L * L)
    k = bending_block(EI, L, phi)
    ok("S-A stiffness symmetric", k == [list(r) for r in zip(*k)])
    ok("S-A rigid translation", matvec(k, [F(1), F(0), F(1), F(0)]) == [0] * 4)
    ok("S-A rigid rotation", matvec(k, [F(0), F(1), L, F(1)]) == [0] * 4)
    kff = [row[2:] for row in k[2:]]
    ok("S-A tip force: v = 5/12, theta = 1/2", solve_dense(kff, [F(1), F(0)]) == [F(5, 12), F(1, 2)])
    ok("S-A tip couple: v = 1/2, theta = 1", solve_dense(kff, [F(0), F(1)]) == [F(1, 2), F(1)])
    fu = uniform_feq_local([F(0), F(1), F(0)], L)
    ok("S-A full uniform: v = theta = 1/6", solve_dense(kff, [fu[7], fu[11]]) == [F(1, 6), F(1, 6)])
    # root actions of the tip-force cantilever: f = K d (no element load)
    d = [F(0), F(0)] + solve_dense(kff, [F(1), F(0)])
    fr = matvec(k, d)
    ok("S-A root Fy = -1, Mz = -1", fr[0] == -1 and fr[1] == -1)
    # S-E: interior ground spring at a = 1/2 with k = 12 on two exact elements, tip force 1
    for th, expect in (("timoshenko", [F(-7, 8), F(7, 96), F(37, 128), F(25, 64), F(-1, 8), F(-9, 16)]),
                       ("eb", [F(-5, 6), F(5, 72), F(71, 288), F(19, 48), F(-1, 6), F(-7, 12)])):
        h = F(1, 2)
        ph = 12 * EI / (Ks * h * h) if th == "timoshenko" else F(0)
        kb = bending_block(EI, h, ph)
        K = [[F(0)] * 6 for _ in range(6)]
        for e in range(2):
            for r in range(4):
                for c in range(4):
                    K[2 * e + r][2 * e + c] += kb[r][c]
        K[2][2] += 12
        u = solve_dense([row[2:] for row in K[2:]], [F(0), F(0), F(1), F(0)])
        R = -12 * u[0]
        root = matvec([row[2:] for row in K[:2]], u)
        got = [R, u[0], u[2], u[3], root[0], root[1]]
        ok(f"S-E {th} [R, u(a), u(L), theta(L), root F, root M]", got == expect)
    # S-C: kappa limits and Cowper equality at nu = 0
    for nu in (F(0), F(3, 10), F(-1, 5), F(49, 100)):
        ok(f"S-C m -> 0 at nu={nu}", kappa_energy(F(0), nu) == 6 * (1 + nu) ** 2 / (7 + 14 * nu + 8 * nu * nu))
        ok(f"S-C m -> 1 at nu={nu}", kappa_energy(F(1), nu) == F(1, 2))
        for m in (F(1, 4), F(9, 10)):
            closed = 6 * (1 + nu) ** 2 * (1 + m * m) ** 2 / ((7 + 14 * nu + 8 * nu * nu) * (1 + m * m) ** 2 + 4 * m * m * (5 + 10 * nu + 4 * nu * nu))
            ok(f"S-C cancellation-free form m={m} nu={nu}", kappa_energy(m, nu) == closed)
    ok("S-C nu = 0: Cowper equals energy", all(kappa_energy(m, F(0)) == kappa_cowper(m, F(0)) for m in (F(1, 4), F(9, 10))))
    ok("S-B kappa_E = 16609827/33068756", kappa_energy(F(9, 10), F(3, 10)) == F(16609827, 33068756))
    ok("S-B kappa_C = 1277679/2397284", kappa_cowper(F(9, 10), F(3, 10)) == F(1277679, 2397284))
    # S-B physical cantilevers through the full 3D engine (Decimal): ro = .1, ri = .09, E = 200 GPa, nu = .3, F = 100 N
    for Lm, vtip_exact_over_pi in ((1, F(125917189, 6068975250000)), (2, F(478867189, 3034487625000)), (10, F(11773267189, 606897525000))):
        req = _sb_request(float(Lm))
        ref = Reference(req)
        u = ref.cases["case:1"]["u"]
        vtip = u[6 + 1]
        expect = D(vtip_exact_over_pi.numerator) / D(vtip_exact_over_pi.denominator) / PI
        # the engine's section comes from binary64 OD 0.2 and wall 0.01 (exactly 0.2 - 0.18? not exact); compare relatively
        rel = abs(vtip - expect) / expect
        ok(f"S-B L={Lm} tip deflection within 1e-15 (binary64 section inputs)", rel < D("1e-15"))
        ok(f"S-B L={Lm} root reaction", abs(ref.cases["case:1"]["reactions"]["support:A"][1] + 100) < D("1e-90")
           and abs(ref.cases["case:1"]["reactions"]["support:A"][5] + 100 * Lm) < D("1e-88"))
    # whole-body balance on a couple-only case, and its failure when a reaction moment is perturbed
    req = _sb_request(2.0)
    req["model"]["load_cases"][0]["primitive_loads"] = [{"id": "c", "category": "concentrated_moment", "target": {"type": "node", "node": "node:B"},
                                                          "direction": "rotation_z", "magnitude": {"value": 250.0, "unit": "N*m"}, "dimension": "moment"}]
    ref = Reference(req)
    ok("couple-only case: root moment = -250 N*m and zero root force", abs(ref.cases["case:1"]["reactions"]["support:A"][5] + 250) < D("1e-90")
       and max(abs(x) for x in ref.cases["case:1"]["reactions"]["support:A"][:3]) < D("1e-90"))
    case = ref.cases["case:1"]
    case["reactions"]["support:A"][5] += D("1e-20")
    nodal = [ZERO] * 12
    nodal[6 + 5] = D(250)
    feqs = {"pipe:AB": ([ZERO] * 12, [ZERO, ZERO, ZERO])}
    try:
        ref._check_balance(case, nodal, feqs)
        raised = False
    except AssertionError:
        raised = True
    ok("couple-only case: a reaction moment off by 1e-20 fails the balance check", raised)
    # S-E physical companion through the full 3D engine: S-B section, L = 2 m, ground spring k = 1e6 N/m at a = 1 m,
    # tip force 100 N; SHEAR_REFERENCE's presentation decimals (12 significant digits)
    req = _sb_request(2.0)
    m = req["model"]
    m["nodes"].insert(1, {"id": "node:M", "position": {"x": 1.0, "y": 0.0, "z": 0.0}})
    seg = m["pipe_segments"][0]
    m["pipe_segments"] = [dict(seg, id="pipe:AM", to="node:M"), dict(seg, id="pipe:MB", **{"from": "node:M"})]
    m["supports"].append({"id": "support:k", "node": "node:M", "family": "spring", "restraints": ["UY"],
                          "stiffness": {"dof": "UY", "value": {"value": 1.0e6, "unit": "N/m"}}})
    ref = Reference(req)
    c = ref.cases["case:1"]
    u = c["u"]
    got = {"R": c["reactions"]["support:k"][1], "u(a)": u[6 + 1], "u(L)": u[12 + 1], "theta(L)": u[12 + 5],
           "root force": c["reactions"]["support:A"][1], "root moment": c["reactions"]["support:A"][5]}
    want = {"R": D("-14.8775370812"), "u(a)": D("1.48775370812e-5"), "u(L)": D("4.78723378122e-5"), "theta(L)": D("3.56464923927e-5"),
            "root force": D("-85.1224629188"), "root moment": D("-185.122462919")}
    for k in want:
        ok(f"S-E physical {k} = {want[k]} (12 digits)", abs(got[k] - want[k]) <= abs(want[k]) * D("1e-11"))
    # exactly six rigid modes of the 12x12 local element (both theories), and the statics of the cut
    for th in ("timoshenko_annulus_energy_v1", "euler_bernoulli"):
        req = _sb_request(1.0)
        ref = Reference(req, theory=th)
        m = ref.members[0]
        kl = m.k_local
        L = m.L
        modes = [[1, 0, 0, 0, 0, 0] * 2, [0, 1, 0, 0, 0, 0] * 2, [0, 0, 1, 0, 0, 0] * 2, [0, 0, 0, 1, 0, 0] * 2,
                 [0, 0, 0, 0, 0, 1, 0, L, 0, 0, 0, 1], [0, 0, 0, 0, 1, 0, 0, 0, -L, 0, 1, 0]]
        for mode in modes:
            v = matvec(kl, [D(x) for x in mode])
            ok(f"{th} rigid mode", max(abs(x) for x in v) < D("1e-90") * m.EA)
        F_, M_ = ref.cut("case:1", "pipe:AB", ONE)
        f = ref.cases["case:1"]["members"]["pipe:AB"]["f"]
        ok(f"{th} cut(L) equals the j-end action", max(abs(a - b) for a, b in zip(F_ + M_, f[6:12])) < D("1e-90"))
        # pure curvature d = [0,0,c L^2/2, c L] in the x-y plane gives V = 0 and M = EI c whatever K_s
        c = D("1e-3")
        dvec = [ZERO] * 12
        dvec[7], dvec[11] = c * L * L / 2, c * L
        fv = matvec(kl, dvec)
        ok(f"{th} pure curvature: no shear", abs(fv[7]) < D("1e-90") * m.EA and abs(fv[11] - m.EI * c) < D("1e-90") * m.EI)
    if verbose:
        for name, good in checks:
            print(("PASS " if good else "FAIL ") + name)
    return checks


def _sb_request(L):
    """SHEAR_REFERENCE S-B companion as a product request (Timoshenko, F = 100 N tip force)."""
    q = lambda v, u: {"value": v, "unit": u}
    prov = "invented M3-T engine self-check"
    return {"model": {"schema_version": "0.8.0", "document_kind": "openpipestress.product_preview.model",
                      "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "timoshenko_annulus_energy_v1"},
                      "nodes": [{"id": "node:A", "position": {"x": 0.0, "y": 0.0, "z": 0.0}}, {"id": "node:B", "position": {"x": L, "y": 0.0, "z": 0.0}}],
                      "pipe_segments": [{"id": "pipe:AB", "from": "node:A", "to": "node:B", "material": "material:m",
                                         "section": {"outside_diameter": q(0.2, "m"), "wall_thickness": q(0.01, "m"), "corrosion_allowance": q(0.0, "m")},
                                         "y_reference": {"x": 0.0, "y": 0.0, "z": 1.0}, "provenance": prov}],
                      "materials": [{"id": "material:m", "constitutive_basis": "homogeneous_isotropic_E_nu_v1", "elastic_modulus": q(2.0e11, "Pa"),
                                     "poisson_ratio": q(0.3, "1")}],
                      "supports": [{"id": "support:A", "node": "node:A", "family": "anchor", "restraints": DOFS}],
                      "components": [],
                      "load_cases": [{"id": "case:1", "primitive_loads": [{"id": "l", "category": "concentrated_force", "target": {"type": "node", "node": "node:B"},
                                                                            "direction": "global_y", "magnitude": q(100.0, "N"), "dimension": "force"}],
                                      "pressure_regions": []}]}, "materials": []}


if __name__ == "__main__":
    import sys
    if "--check" in sys.argv:
        n = len(self_check())
        print(f"engine self-check: {n} checks pass")
