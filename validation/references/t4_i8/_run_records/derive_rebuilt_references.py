"""T4-I8: independent references for T4-U2's rebuilt straight cases.

Standard library only (fractions, decimal). No product module is imported and no
product output is read. The only external input is the frozen independent
fixture SOURCE_ODWALL_EXPECTATIONS.json (case 4's twin), passed by path and
identified by sha256; it is compared, never used to form a case 1-3 value.

Usage:
    python -I derive_rebuilt_references.py <SOURCE_ODWALL_EXPECTATIONS.json> <out.json>

Every value is exact: a rational times pi**k (k in -1, 0, 1). Closed forms
(Lame, Hooke with the Poisson term, beam statics) are evaluated in Fractions and
cross-checked by a second method (exact direct-stiffness solve with consistent
loads and section recovery; the general Lame family; compliance identities).
"""

import hashlib
import json
import sys
from decimal import Decimal, getcontext
from fractions import Fraction as F

getcontext().prec = 130
CHECKS = []


def check(label, condition):
    CHECKS.append((label, bool(condition)))
    if not condition:
        print("FAIL", label)


# ---------------------------------------------------------------- pi, two ways
def pi_machin():
    getcontext().prec = 140

    def arctan_inv(n):
        x = Decimal(1) / n
        x2 = x * x
        term, total, k = x, x, 1
        while True:
            term *= -x2
            add = term / (2 * k + 1)
            if abs(add) < Decimal(10) ** -138:
                break
            total += add
            k += 1
        return total

    value = 16 * arctan_inv(5) - 4 * arctan_inv(239)
    getcontext().prec = 130
    return +value


def pi_gauss_legendre():
    getcontext().prec = 140
    a, b, t, p = Decimal(1), Decimal(1) / Decimal(2).sqrt(), Decimal(1) / 4, Decimal(1)
    for _ in range(9):
        an = (a + b) / 2
        b = (a * b).sqrt()
        t -= p * (a - an) ** 2
        a = an
        p *= 2
    value = (a + b) ** 2 / (4 * t)
    getcontext().prec = 130
    return +value


PI = pi_machin()
check("pi Machin == Gauss-Legendre to 1e-125", abs(PI - pi_gauss_legendre()) < Decimal(10) ** -125)
PI_STR_100 = format(+Decimal(PI).quantize(Decimal(10) ** -100), "f")


# ---------------------------------------------------------------- exact numbers
class Q:
    """rational * pi**k"""

    def __init__(self, r, k=0):
        self.r = F(r)
        self.k = k if self.r != 0 else 0

    def __mul__(self, o):
        o = o if isinstance(o, Q) else Q(o)
        return Q(self.r * o.r, self.k + o.k)

    __rmul__ = __mul__

    def __truediv__(self, o):
        o = o if isinstance(o, Q) else Q(o)
        return Q(self.r / o.r, self.k - o.k)

    def __add__(self, o):
        o = o if isinstance(o, Q) else Q(o)
        if self.r == 0:
            return o
        if o.r == 0:
            return self
        assert self.k == o.k, "mixed pi powers"
        return Q(self.r + o.r, self.k)

    def __sub__(self, o):
        o = o if isinstance(o, Q) else Q(o)
        return self + Q(-o.r, o.k)

    def __neg__(self):
        return Q(-self.r, self.k)

    def __eq__(self, o):
        o = o if isinstance(o, Q) else Q(o)
        return self.r == o.r and (self.k == o.k or self.r == 0)

    def dec(self):
        return Decimal(self.r.numerator) / Decimal(self.r.denominator) * PI ** self.k

    def f64(self):
        return float(self.dec())


KIND = {0: "rational", 1: "rational_times_pi", -1: "rational_over_pi"}


def rat_str(r):
    return str(r.numerator) if r.denominator == 1 else f"{r.numerator}/{r.denominator}"


def dec85(d):
    getcontext().prec = 85
    s = format(+d, "f")
    getcontext().prec = 130
    return s


def qty(q, unit):
    q = q if isinstance(q, Q) else Q(q)
    d = q.dec()
    return {"unit": unit, "exact": {"kind": KIND[q.k], "rational": rat_str(q.r)},
            "decimal": dec85(d) if q.r != 0 else "0", "value": q.f64()}


def sym_qty(text, dvalue, unit):
    return {"unit": unit, "exact": {"kind": "symbolic", "expression": text,
            "evaluation": "Decimal sqrt, precision 130"}, "decimal": dec85(dvalue), "value": float(dvalue)}


# ---------------------------------------------------------------- mechanics
def annulus(od, wall):
    od, wall = F(od), F(wall)
    ro = od / 2
    ri = ro - wall
    As = Q(wall * (od - wall), 1)              # pi t (OD - t)
    Ai = Q(ri * ri, 1)                         # pi ri^2
    I = As * Q((ro * ro + ri * ri) / 4)        # As (ro^2 + ri^2) / 4
    J = I * 2
    Z = I / Q(ro)
    check(f"As == pi(ro^2-ri^2) [{od},{wall}]", As == Q(ro * ro - ri * ri, 1))
    check(f"I == pi(ro^4-ri^4)/4 [{od},{wall}]", I == Q((ro ** 4 - ri ** 4) / 4, 1))
    return {"OD": od, "wall": wall, "ro": ro, "ri": ri, "As": As, "Ai": Ai, "I": I, "J": J, "Z": Z}


def lame(g, p):
    """Surface values from the general family sigma_r = C - D/r^2, sigma_h = C + D/r^2."""
    ro, ri, p = g["ro"], g["ri"], F(p)
    C = p * ri * ri / (ro * ro - ri * ri)
    D = p * ri * ri * ro * ro / (ro * ro - ri * ri)
    sr = lambda r: C - D / (r * r)
    sh = lambda r: C + D / (r * r)
    out = {"inner_radial": sr(ri), "outer_radial": sr(ro), "inner_hoop": sh(ri), "outer_hoop": sh(ro)}
    # closed forms as a second route
    check("Lame inner radial == -p", out["inner_radial"] == -p)
    check("Lame outer radial == 0", out["outer_radial"] == 0)
    check("Lame inner hoop closed form", out["inner_hoop"] == p * (ro * ro + ri * ri) / (ro * ro - ri * ri))
    check("Lame outer hoop closed form", out["outer_hoop"] == 2 * p * ri * ri / (ro * ro - ri * ri))
    P_over_As = (Q(p) * g["Ai"] / g["As"])
    check("sigma_r + sigma_h == 2 P/As (C = P/As)", P_over_As == Q(C) and sr(ri) + sh(ri) == 2 * C)
    return out


def axial_state(g, E, nu, p, eps_th, restrained, transfer=True):
    """Hooke with the Poisson term; returns Nw, S, sigma_z, strain."""
    E, nu, p, eps_th = F(E), F(nu), F(p), F(eps_th)
    P = Q(p) * g["Ai"]
    EA = Q(E) * g["As"]
    two_nu_P = Q(2 * nu) * P
    if restrained:
        strain = F(0)
        Nw = EA * Q(strain - eps_th) + two_nu_P
    else:
        Nw = P if transfer else Q(0)
        # strain from compliance: eps = (Nw/As - nu*2C)/E + eps_th
        strain = ((Nw / g["As"]) - Q(nu * 2) * (P / g["As"])).r / E + eps_th
    S = Nw - P
    sz = Nw / g["As"]
    # compliance identity check: eps_z - eps_th = (sigma_z - nu (sigma_r + sigma_h))/E
    C = (P / g["As"])
    check("axial compliance identity", Q(strain - eps_th) == (sz - Q(nu * 2) * C) / Q(E))
    return {"P": P, "EA": EA, "Nw": Nw, "S": S, "sigma_z": sz, "strain": strain}


def self_test_qualified_tables():
    """Reproduce the qualified rational table and the 6 m companion (PRESSURE_REFERENCE_QUALIFICATION
    section 2; STRESS_REFERENCE section 8) with this script's own functions."""
    g = {"ro": F(2), "ri": F(1), "As": Q(3, 1), "Ai": Q(1, 1)}
    L = lame(g, 3)
    check("qualified table radial [-3,0] hoop [5,2]",
          (L["inner_radial"], L["outer_radial"], L["inner_hoop"], L["outer_hoop"]) == (-3, 0, 5, 2))
    E, nu, p = 120, F(1, 4), 3
    rows = [(False, 0, F(1, 240), 3, 0, 1), (True, 0, 0, F(3, 2), F(-3, 2), F(1, 2)),
            (False, F(1, 1000), F(31, 6000), 3, 0, 1), (True, F(1, 1000), 0, F(57, 50), F(-93, 50), F(19, 50))]
    for restrained, th, strain, nw, s, sz in rows:
        st = axial_state(g, E, nu, p, th, restrained)
        check(f"qualified state restrained={restrained} th={th}",
              st["strain"] == strain and st["Nw"] == Q(nw, 1) and st["S"] == Q(s, 1) and st["sigma_z"] == Q(sz))
    st = axial_state(g, E, nu, p, 0, False, transfer=False)
    check("qualified state separate closures free wall", st["Nw"] == 0 and st["S"] == Q(-3, 1) and st["strain"] == F(-1, 240))
    g6 = annulus(F(12, 100), F(1, 100))
    st = axial_state(g6, 200 * 10 ** 9, F(3, 10), 2 * 10 ** 6, 0, True)
    check("6 m companion fixed wall 3000pi", st["Nw"] == Q(3000, 1))
    st = axial_state(g6, 200 * 10 ** 9, F(3, 10), 2 * 10 ** 6, F(12, 10000), True)
    check("6 m companion fixed thermal wall -261000pi, S -266000pi", st["Nw"] == Q(-261000, 1) and st["S"] == Q(-266000, 1))
    st = axial_state(g6, 200 * 10 ** 9, F(3, 10), 2 * 10 ** 6, 0, False)
    check("6 m companion free extension 3/55000 m", st["strain"] * 6 == F(3, 55000))
    L6 = lame(g6, 2 * 10 ** 6)
    check("6 m companion hoop 122e6/11, 100e6/11", L6["inner_hoop"] == F(122 * 10 ** 6, 11) and L6["outer_hoop"] == F(10 ** 8, 11))


# ---------------------------------------------------------------- exact frame solve (in-plane bending)
def solve_linear(A, b):
    n = len(A)
    M = [row[:] + [b[i]] for i, row in enumerate(A)]
    for c in range(n):
        piv = next(r for r in range(c, n) if M[r][c] != 0)
        M[c], M[piv] = M[piv], M[c]
        for r in range(n):
            if r != c and M[r][c] != 0:
                f = M[r][c] / M[c][c]
                M[r] = [x - f * y for x, y in zip(M[r], M[c])]
    return [M[i][n] / M[i][i] for i in range(n)]


def beam_fe(xs, loads, EI_r, fixed_node=0):
    """Exact Euler-Bernoulli direct stiffness in (v, theta_z); EI = EI_r * pi (pi kept outside).
    loads: {member index: q (N/m, +local y)}. Root node fixed in v and theta; others free.
    Returns nodal v*pi, theta*pi (i.e. multiply by 1/pi) and member end forces (node-on-element)."""
    n = len(xs)
    K = [[F(0)] * (2 * n) for _ in range(2 * n)]
    Fv = [F(0)] * (2 * n)
    mats = []
    for m in range(n - 1):
        L = xs[m + 1] - xs[m]
        c = EI_r / L ** 3
        k = [[12, 6 * L, -12, 6 * L], [6 * L, 4 * L * L, -6 * L, 2 * L * L],
             [-12, -6 * L, 12, -6 * L], [6 * L, 2 * L * L, -6 * L, 4 * L * L]]
        k = [[c * F(v) for v in row] for row in k]
        q = F(loads.get(m, 0))
        feq = [q * L / 2, q * L * L / 12, q * L / 2, -q * L * L / 12]
        dofs = [2 * m, 2 * m + 1, 2 * m + 2, 2 * m + 3]
        for a in range(4):
            Fv[dofs[a]] += feq[a]
            for b in range(4):
                K[dofs[a]][dofs[b]] += k[a][b]
        mats.append((k, feq, dofs, L, q))
    free = [d for d in range(2 * n) if d not in (2 * fixed_node, 2 * fixed_node + 1)]
    sol = solve_linear([[K[i][j] for j in free] for i in free], [Fv[i] for i in free])
    u = [F(0)] * (2 * n)  # u holds (displacement * pi): EI_r used instead of EI
    for d, v in zip(free, sol):
        u[d] = v
    ends = []
    for k, feq, dofs, L, q in mats:
        ue = [u[d] for d in dofs]
        f = [sum(k[a][b] * ue[b] for b in range(4)) - feq[a] for a in range(4)]
        ends.append((f, L, q))
    return u, ends


def station_from_i_end(f, q, s):
    """Section-cut action on the +x face of the i-side segment at local distance s (the
    product's j-side cut convention): V = -f_iy - q s; M = -f_iM + s f_iy + q s^2/2."""
    return (-f[0] - q * s, -f[1] + s * f[0] + q * s * s / 2)


# ---------------------------------------------------------------- record builders
CRIT_REL = {"kind": "relative", "relative_tolerance": 1e-9, "absolute_tolerance": 0.0}
# Exact definitional factors from the reference unit to the published row unit (T1's convention:
# the reference is converted into the row unit; tolerances are in the row unit).
UNIT_FACTOR = {("m", "mm"): F(1000), ("Pa", "MPa"): F(1, 1000000)}


class Ledger:
    """Named expected quantities plus a row table for one load case (T1 selector shape)."""

    def __init__(self, case_id):
        self.case_id = case_id
        self.expected = {}
        self.scales = {}
        self.scale_defs = {}
        self.rows = []
        self._q = {}        # name -> Q or Decimal (symbolic)

    def put(self, name, q, unit):
        if name in self.expected:
            assert self._q[name] == q, name
            return name
        self.expected[name] = qty(q, unit)
        self._q[name] = q if isinstance(q, Q) else Q(q)
        return name

    def put_sym(self, name, expression, dvalue, unit, evaluation):
        self.expected[name] = {"unit": unit, "exact": {"kind": "symbolic", "expression": expression},
                               "evaluation": evaluation, "decimal": dec85(dvalue), "value": float(dvalue)}
        self._q[name] = dvalue
        return name

    def scale(self, name, q, unit, definition):
        self.scales[name] = qty(q, unit)
        self.scale_defs[name] = definition
        self._q[name] = q if isinstance(q, Q) else Q(q)

    def _zero(self, name):
        v = self._q[name]
        return v.r == 0 if isinstance(v, Q) else v == 0

    def _mag(self, name):
        v = self._q[name]
        return abs(v.dec()) if isinstance(v, Q) else abs(v)

    def row(self, kind, entity, component, location, unit, frame, name, q, scale=None, ref_unit=None, refreeze=None):
        ref_unit = ref_unit or unit
        if q is not None:
            self.put(name, q, ref_unit)
        assert self.expected[name]["unit"] == ref_unit, (name, ref_unit)
        factor = UNIT_FACTOR.get((ref_unit, unit), F(1))
        assert ref_unit == unit or (ref_unit, unit) in UNIT_FACTOR, (ref_unit, unit)
        origin = {"kind": "analytical", "pointer": name, "reference_unit": ref_unit, "transform": "identity"}
        if self._zero(name):
            assert scale is not None, (kind, entity, component, location)
            s_unit = self.scales[scale]["unit"]
            assert s_unit == ref_unit, (scale, s_unit, ref_unit)
            origin["zero_scale"] = {"tag": scale, "base": {"pointer": scale, "reference_unit": s_unit}, "scale_unit": s_unit}
            tol = Decimal("1e-9") * self._mag(scale) * Decimal(factor.numerator) / Decimal(factor.denominator)
            crit = {"kind": "zero_scale", "relative_tolerance": 0.0, "absolute_tolerance": float(tol)}
        else:
            crit = dict(CRIT_REL)
        entry = {"kind": kind, "entity_ref": entity, "component": component, "location": location,
                 "unit": unit, "coordinate_system": frame,
                 "basis_ref": {"ref_type": "load_case", "ref_id": self.case_id},
                 "expected": name, "reference_origin": origin, "criterion": crit}
        if refreeze:
            entry["refreeze"] = refreeze
        self.rows.append(entry)

    def to_json(self, base):
        rows = []
        for r in self.rows:
            r = json.loads(json.dumps(r))
            o = r["reference_origin"]
            o["pointer"] = f"{base}/expected/{o['pointer']}"
            if "zero_scale" in o:
                o["zero_scale"]["base"]["pointer"] = f"{base}/zero_scales/{o['zero_scale']['base']['pointer']}"
            rows.append(r)
        return {"expected": self.expected, "zero_scales": self.scales,
                "zero_scale_definitions": self.scale_defs, "rows": rows}


SIGN = {
    "wall": "tension-positive material wall section resultant Nw",
    "effective": "S = Nw - pAi; not material stress or a support reaction",
    "membrane": "tension-positive axial wall membrane stress Nw/As; no added longitudinal pressure scalar",
    "endpoint": "node-on-element wall action, positive along authored local x toward end j",
    "radial": "tension-positive radial stress at named surface",
    "hoop": "tension-positive circumferential stress at named surface",
    "end_rows": "node-on-element action at the member end, element-local DOF sense",
    "station_rows": "section-cut action on the +x face of the i-side segment (product: j-side cut)",
    "stress_rows": "bending normal stress = section M/Z and torsional shear = T ro/J, from the j-side section action at all five locations (ends included); published in MPa",
    "maximum": "per member: nonnegative max over the member of |Nw/As| + hypot(My,Mz)/Z; location governing_station; Pa",
    "support": "support-on-pipe; positive global force and right-hand couple about the attached node; magnitudes are the force and moment norms",
    "disp": "global cartesian; translations in mm, right-hand rotations in rad",
}

STATIONS = [("end_i", F(0)), ("quarter_1", F(1, 4)), ("midspan", F(1, 2)), ("quarter_3", F(3, 4)), ("end_j", F(1))]
TRANS = [("element_local_shear_force_y", "shear_force_y", "N", 0), ("element_local_shear_force_z", "shear_force_z", "N", None),
         ("element_local_torsional_moment", "torsional_moment", "N*m", None),
         ("element_local_bending_moment_y", "bending_moment_y", "N*m", None),
         ("element_local_bending_moment_z", "bending_moment_z", "N*m", 1)]
STRESS = [("element_local_bending_normal_stress_y", "bending_normal_stress_y", None),
          ("element_local_bending_normal_stress_z", "bending_normal_stress_z", 1),
          ("element_local_torsional_shear_stress", "torsional_shear_stress", None)]


def pressure_rows(led, pipe, st, lm, p, scale_force, scale_stress):
    led.put("Nw", st["Nw"], "N")
    for loc, _ in STATIONS:
        led.row("pipe_wall_axial_force_v2", pipe, "wall_axial_force", loc, "N", "element_local", "Nw", st["Nw"], scale_force)
        led.row("pipe_effective_axial_force_v2", pipe, "effective_axial_force", loc, "N", "element_local", "S", st["S"], scale_force)
        led.row("pipe_axial_membrane_stress_v2", pipe, "axial_membrane_stress", loc, "Pa", "pipe_section", "sigma_z", st["sigma_z"], scale_stress)
        led.row("pipe_lame_radial_stress_v2", pipe, "lame_inner_radial_stress", loc, "Pa", "pipe_section", "lame_inner_radial", lm["inner_radial"], scale_stress)
        led.row("pipe_lame_radial_stress_v2", pipe, "lame_outer_radial_stress", loc, "Pa", "pipe_section", "lame_outer_radial", lm["outer_radial"], scale_stress)
        led.row("pipe_lame_hoop_stress_v2", pipe, "lame_inner_hoop_stress", loc, "Pa", "pipe_section", "lame_inner_hoop", lm["inner_hoop"], scale_stress)
        led.row("pipe_lame_hoop_stress_v2", pipe, "lame_outer_hoop_stress", loc, "Pa", "pipe_section", "lame_outer_hoop", lm["outer_hoop"], scale_stress)
    led.row("pipe_wall_endpoint_action_v2", pipe, "wall_axial_end_action", "end_i", "N", "element_local", "wall_end_action_i", -st["Nw"], scale_force)
    led.row("pipe_wall_endpoint_action_v2", pipe, "wall_axial_end_action", "end_j", "N", "element_local", "wall_end_action_j", st["Nw"], scale_force)


def transverse_rows(led, pipe, tag, values, scale_force, scale_moment):
    """values[loc] = (Vy_row_value, Mz_row_value) already in row convention; others zero."""
    for loc, _ in STATIONS:
        vy, mz = values.get(loc, (Q(0), Q(0)))
        for kind, comp, unit, which in TRANS:
            v = vy if which == 0 else (mz if which == 1 else Q(0))
            sc = scale_force if unit == "N" else scale_moment
            name = f"{tag}_{loc}_{comp}" if v.r != 0 else "zero_" + ("force" if unit == "N" else "moment")
            led.row(kind, pipe, comp, loc, unit, "element_local", name, v, sc)


def stress_rows(led, pipe, tag, section_m, Z, scale_stress):
    """section_m[loc] = section-cut Mz (j-side) at loc; bending y and torsion are zero here."""
    for loc, _ in STATIONS:
        mz = section_m.get(loc, Q(0))
        for kind, comp, which in STRESS:
            v = (mz / Z) if (which == 1 and mz.r != 0) else Q(0)
            name = f"{tag}_{loc}_{comp}" if v.r != 0 else "zero_stress"
            led.row(kind, pipe, comp, loc, "MPa", "element_local", name, v, scale_stress, ref_unit="Pa")


def maximum_row(led, pipe, tag, membrane, m_abs_max, Z):
    """|Nw/As| + max|M|/Z for one member; symbolic when both parts are nonzero (mixed pi powers)."""
    a = Q(abs(membrane.r), membrane.k)
    if m_abs_max.r == 0:
        name = f"{tag}_maximum"
        led.row("pipe_elastic_normal_stress_maximum_v2", pipe, "maximum_absolute_normal_stress", "governing_station", "Pa", "pipe_section", name, a)
        return
    b = m_abs_max / Z
    assert a.k == 0 and b.k == -1
    name = f"{tag}_maximum"
    led.put_sym(name, f"{rat_str(a.r)} + ({rat_str(b.r)})/pi", a.dec() + b.dec(), "Pa", "Decimal, pi from numeric_representation, precision 130")
    led.row("pipe_elastic_normal_stress_maximum_v2", pipe, "maximum_absolute_normal_stress", "governing_station", "Pa", "pipe_section", name, None)


def node_rows(led, node, ux, uy, rz, scale_len, scale_rot, tag):
    for axis, val in (("x", ux), ("y", uy), ("z", Q(0))):
        name = f"{tag}_u{axis}" if val.r != 0 else "zero_length"
        led.row(f"global_nodal_displacement_{axis}", node, f"nodal_displacement_{axis}", "node", "mm", "global", name, val, scale_len, ref_unit="m")
    for axis, val in (("x", Q(0)), ("y", Q(0)), ("z", rz)):
        name = f"{tag}_r{axis}" if val.r != 0 else "zero_rotation"
        led.row(f"global_nodal_rotation_{axis}", node, f"nodal_rotation_{axis}", "node", "rad", "global", name, val, scale_rot)


def magnitude_rows(led, support, fvec, mvec, scale_force, scale_moment, tag):
    """Force and moment norms. fvec/mvec are lists of Q; a norm mixing pi powers is symbolic (sqrt)."""
    for kind, comp, vec, unit, sc in (("support_reaction_force_magnitude_v2", "force_magnitude", fvec, "N", scale_force),
                                      ("support_reaction_moment_magnitude_v2", "moment_magnitude", mvec, "N*m", scale_moment)):
        nz = [v for v in vec if v.r != 0]
        name = f"{tag}_{comp}"
        if not nz:
            led.row(kind, support, comp, "node", unit, "global", "zero_force" if unit == "N" else "zero_moment", Q(0), sc)
        elif len(nz) == 1:
            led.row(kind, support, comp, "node", unit, "global", name, Q(abs(nz[0].r), nz[0].k))
        else:
            assert len(nz) == 2 and {v.k for v in nz} == {0, 1}
            pk = [v for v in nz if v.k == 1][0]
            rk = [v for v in nz if v.k == 0][0]
            d = (pk.dec() ** 2 + rk.dec() ** 2).sqrt()
            led.put_sym(name, f"sqrt(({rat_str(pk.r)}*pi)^2 + ({rat_str(rk.r)})^2)", d, unit, "Decimal.sqrt, pi from numeric_representation, precision 130")
            led.row(kind, support, comp, "node", unit, "global", name, None)


def support_rows(led, support, comps, scale_force, scale_moment, tag):
    for comp, val in zip(["Fx", "Fy", "Fz", "Mx", "My", "Mz"], comps):
        unit = "N" if comp[0] == "F" else "N*m"
        sc = scale_force if unit == "N" else scale_moment
        name = f"{tag}_{comp}" if val.r != 0 else ("zero_force" if unit == "N" else "zero_moment")
        led.row("support_reaction_component_v2", support, comp, "node", unit, "global", name, val, sc)
    magnitude_rows(led, support, comps[:3], comps[3:], scale_force, scale_moment, tag)


# ---------------------------------------------------------------- document sketches
PROV = "invented T4-I8 rebuilt-case input; not library, component or code-rule data"
UNITS = {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}


def fnum(x):
    return float(x)


def doc_030(project, nodes, pipes, supports, materials, cases, contract=("2.0.0", "exact_straight_pressure_v2")):
    return {"model": {"schema_version": "0.3.0", "document_kind": "openpipestress.product_preview.model",
                      "pressure_contract": {"version": contract[0], "mode": contract[1]},
                      "project": {"id": project, "units": UNITS},
                      "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
                                          "rule_check": "not_performed_user_rule_inputs_missing",
                                          "professional_acceptance": "not_provided"},
                      "nodes": nodes, "pipe_segments": pipes, "supports": supports, "components": [],
                      "materials": materials, "load_cases": cases, "combinations": []}, "materials": []}


def node(id_, x, y=0.0, z=0.0):
    return {"id": id_, "position": {"x": fnum(x), "y": fnum(y), "z": fnum(z)}, "provenance": PROV}


def pipe(id_, a, b, od, wall, mat, mill=None):
    sec = {"outside_diameter": {"value": fnum(od), "unit": "m"}, "wall_thickness": {"value": fnum(wall), "unit": "m"}}
    if mill is not None:
        sec["mill_tolerance"] = {"value": fnum(mill), "unit": "m"}
    return {"id": id_, "from": a, "to": b, "section": sec, "material": mat,
            "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0}, "provenance": PROV}


def support(id_, node_, family, restraints):
    return {"id": id_, "node": node_, "family": family, "restraints": restraints, "provenance": PROV}


def material(id_, E, nu, alpha=None):
    m = {"id": id_, "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
         "elastic_modulus": {"value": fnum(E), "unit": "Pa"}, "poisson_ratio": {"value": fnum(nu), "unit": "1"},
         "provenance": PROV}
    if alpha is not None:
        m["thermal_expansion_coefficient"] = {"value": fnum(alpha), "unit": "1/degC"}
    return m


def region(id_, members, p, a, b):
    return {"id": id_, "member_pipe_ids": members, "pressure_basis": "internal_differential_zero_external_v1",
            "pressure": {"value": fnum(p), "unit": "Pa"},
            "terminals": [{"node_ref": a, "closure_transfer": "transfers_to_wall", "provenance": PROV},
                          {"node_ref": b, "closure_transfer": "transfers_to_wall", "provenance": PROV}],
            "provenance": PROV}


def to_040(doc030, case_states):
    """case_states[case_id] = {"thermal": {pipe: (alpha, dT)} , "sources": [ids]} ; thermal primitives are
    removed (0.4.0 owns thermal strain in element thermal_state)."""
    d = json.loads(json.dumps(doc030))
    m = d["model"]
    m["schema_version"] = "0.4.0"
    pipes = [p["id"] for p in m["pipe_segments"]]
    m["reference_configurations"] = [{"id": "reference:installed", "label": "Installed reference",
                                      "geometry_ref": {"kind": "authored_model_geometry"},
                                      "member_references": [{"pipe_ref": p, "basis": {"kind": "direct_strain_reference"},
                                                             "fit": {"kind": "none"}, "provenance": PROV} for p in pipes],
                                      "provenance": PROV}]
    for mat in m["materials"]:
        mat.pop("thermal_expansion_coefficient", None)
    for case in m["load_cases"]:
        cs = case_states.get(case["id"], {})
        thermal = cs.get("thermal", {})
        case["primitive_loads"] = [l for l in case["primitive_loads"] if l["category"] != "thermal"]
        elems = []
        for p in m["pipe_segments"]:
            if p["id"] in thermal:
                alpha, dT = thermal[p["id"]]
                ts = {"kind": "constant_alpha_interval", "coefficient": {"value": fnum(alpha), "unit": "1/degC"},
                      "temperature_change": {"value": fnum(dT), "unit": "degC"},
                      "coefficient_meaning": "engineering_interval", "provenance": PROV}
            else:
                ts = {"kind": "unchanged_reference", "provenance": PROV}
            elems.append({"pipe_ref": p["id"], "material_selection": {"kind": "explicit_base_properties",
                          "material_ref": p["material"], "applicability_reference": "invented analytical basis declared applicable (T4-I8)"},
                          "thermal_state": ts})
        case["analysis_state"] = {"contract": "openpipestress.load_reference_state/1.0.0",
                                  "reference_configuration_ref": "reference:installed", "element_states": elems,
                                  "support_states": [{"support_ref": s["id"], "participation": {"kind": "active_model_device"}} for s in m["supports"]],
                                  "load_sources": [{"source_ref": l["id"], "factor": 1.0} for l in case["primitive_loads"]],
                                  "history": {"kind": "independent_equilibrium"}, "provenance": PROV}
    return d


V3_PATCH = {"op": "replace", "path": "/model/pressure_contract", "value": {"version": "3.0.0", "mode": "exact_pressure_v3"},
            "status": "PROVISIONAL: H-1 names 3.0.0/exact_pressure_v3; T4-U2a fixes the wire (schema_version, region fields, any family tag). "
                      "Straight members are expected to need no field beyond v2's."}


def documents(doc030, states040):
    return {"v2_model_0.3.0": doc030, "v2_model_0.4.0": to_040(doc030, states040),
            "v3_patch_for_both": V3_PATCH,
            "sp1_pair": "/sp1_pair_scope",
            "note": "The v2 documents run on today's exact route (straight only). The v3 documents are the v2 documents with V3_PATCH applied. The pair is held to /sp1_pair_scope (all inputs here have p >= 0), separately for 0.3.0 and 0.4.0 and for each mode."}


# ================================================================= CASE 1: MILLTOL
def case_milltol():
    od, t_nom, ca, mill = F(1, 5), F(1, 100), F(1, 500), F(1, 800)
    wall_authored = t_nom - ca                      # corrosion allowance folded into the authored wall
    t_eff = wall_authored - mill                    # product section rule: wall - mill tolerance
    check("MILLTOL t_eff == 0.00675 (retired t_eff)", t_eff == F(27, 4000))
    g = annulus(od, t_eff)
    check("MILLTOL ri == 0.09325", g["ri"] == F(373, 4000))
    E, nu, p, L = 200 * 10 ** 9, F(3, 10), 2000, F(2)
    lm = lame(g, p)
    retired_hoop = F(p) * ((od - t_eff) / 2) / t_eff
    check("retired thin-wall hoop 28629.6296...", abs(float(retired_hoop) - 28629.62962962963) < 1e-9)
    out = {"case_id": "EXACT-PRESSURE-MILLTOL-LAME-MEMBRANE-001",
           "rebuilds": {"retired_case_id": "STRESS-TP-PMM-P3-MILLTOL-EFFECTIVE-WALL-STRESS",
                        "retired_values": ["pressure_hoop = p r_m / t_eff", "pressure_longitudinal = pressure_hoop / 2"],
                        "kept_in_stress_crate": "axial_normal, bending_normal_y, bending_normal_z, torsional_shear (unchanged; not part of this rebuild)",
                        "retired_by": "T3's U3 pressure retirement"},
           "refreeze": {"unit": "T4-U6", "condition": "re-freeze if D-5 changes any of: the cap-area (Ai) basis, the As basis of the Poisson eigenstrain, the stiffness basis, or the stress basis",
                        "basis_today": "one basis: OD and (authored wall - mill tolerance) for As, I, J, Z, the Lame radii and Ai (reduced bore)",
                        "fold": "the product has no corrosion-allowance input: authored wall_thickness = nominal 0.01 - allowance 0.002 = 0.008 m; mill_tolerance = 0.00125 m stays in its own slot; effective wall 0.00675 m equals the retired t_eff",
                        "after_D5_as_recommended": "wall 0.01 + a corrosion-allowance input 0.002 + mill 0.00125; stiffness and the eps_p area move to the nominal section; stresses on both sections; Ai stays on the reduced bore. Expected to move: displacements and any nominal-section stress row; expected to stay: Nw, S, and reduced-section sigma_z and Lame rows, provided the As of eps_p equals the stiffness As (RV1 N-5), as D-5 recommends (inference, not frozen)."},
           "inputs": {"outside_diameter": qty(od, "m"), "nominal_wall": qty(t_nom, "m"), "corrosion_allowance_retired_input": qty(ca, "m"),
                      "authored_wall_thickness": qty(wall_authored, "m"), "mill_tolerance": qty(mill, "m"),
                      "effective_wall": qty(t_eff, "m"), "pressure": qty(p, "Pa"), "E": qty(E, "Pa"), "nu": qty(nu, "1"),
                      "length": qty(L, "m")},
           "derived": {"ro": qty(g["ro"], "m"), "ri": qty(g["ri"], "m"), "As": qty(g["As"], "m^2"), "Ai": qty(g["Ai"], "m^2"),
                       "I": qty(g["I"], "m^4"), "J": qty(g["J"], "m^4"), "Z": qty(g["Z"], "m^3"),
                       "P": qty(Q(p) * g["Ai"], "N")},
           "equations": ["t_eff = (t_nom - c) - m", "ro = OD/2; ri = ro - t_eff; As = pi t_eff (OD - t_eff); Ai = pi ri^2; P = p Ai",
                         "sigma_r(ri) = -p; sigma_r(ro) = 0; sigma_h(ri) = p (ro^2+ri^2)/(ro^2-ri^2); sigma_h(ro) = 2 p ri^2/(ro^2-ri^2)",
                         "free, transferring closures: Nw = P; S = 0; sigma_z = P/As; u_tip = (1-2nu) P L/(E As)",
                         "axially restrained, transferring closures: Nw = 2 nu P; S = (2nu-1) P; root Fx = P - Nw = (1-2nu)P; far Fx = Nw - P"],
           "variants": {}}
    mat = [material("material:milltol", E, nu)]
    nodes = [node("node:A", 0), node("node:B", L)]
    pipes = [pipe("pipe:A-B", "node:A", "node:B", od, wall_authored, "material:milltol", mill)]
    for key, restrained in (("free_transferring", False), ("axially_restrained_transferring", True)):
        cid = "case:milltol-" + ("restrained" if restrained else "free")
        st = axial_state(g, E, nu, p, 0, restrained)
        led = Ledger(cid)
        P = st["P"]
        ext = Q(st["strain"] * L)
        free_ext = Q(axial_state(g, E, nu, p, 0, False)["strain"] * L)
        led.scale("scale_force", P, "N", "P = p Ai")
        led.scale("scale_moment", P * Q(L), "N*m", "P L")
        led.scale("scale_stress", P / g["As"], "Pa", "P/As")
        led.scale("scale_length", free_ext, "m", "free closed-tube extension (1-2nu) P L/(E As)")
        led.scale("scale_rotation", free_ext / Q(L), "rad", "free extension / L")
        pressure_rows(led, "pipe:A-B", st, lm, p, "scale_force", "scale_stress")
        transverse_rows(led, "pipe:A-B", "AB", {}, "scale_force", "scale_moment")
        stress_rows(led, "pipe:A-B", "AB", {}, g["Z"], "scale_stress")
        maximum_row(led, "pipe:A-B", "AB", st["sigma_z"], Q(0), g["Z"])
        node_rows(led, "node:A", Q(0), Q(0), Q(0), "scale_length", "scale_rotation", "A")
        node_rows(led, "node:B", ext, Q(0), Q(0), "scale_length", "scale_rotation", "B")
        support_rows(led, "support:A", [P - st["Nw"], Q(0), Q(0), Q(0), Q(0), Q(0)], "scale_force", "scale_moment", "A")
        sups = [support("support:A", "node:A", "anchor", ["UX", "UY", "UZ", "RX", "RY", "RZ"])]
        if restrained:
            support_rows(led, "support:B", [st["Nw"] - P, Q(0), Q(0), Q(0), Q(0), Q(0)], "scale_force", "scale_moment", "B")
            sups.append(support("support:B", "node:B", "anchor", ["UX", "UY", "UZ", "RX", "RY", "RZ"]))
        cases = [{"id": cid, "primitive_loads": [], "pressure_regions": [region("region:milltol", ["pipe:A-B"], p, "node:A", "node:B")], "provenance": PROV}]
        doc = doc_030("project:t4-i8-milltol-" + key, nodes, pipes, sups, mat, cases)
        out["variants"][key] = {"load_cases": {cid: led.to_json(f"/cases/milltol_lame_membrane/variants/{key}/load_cases/{cid}")}, "documents": documents(doc, {})}
    # wrong-result discriminators (values a correct product must not produce)
    def alt(wall_eff, bore_ri=None):
        ga = annulus(od, wall_eff)
        ri = ga["ri"] if bore_ri is None else bore_ri
        Pq = Q(p * ri * ri, 1)
        return ga, Pq
    disc = []
    disc.append({"id": "retired_thin_wall_hoop", "quantity": "lame_inner_hoop", "wrong": qty(retired_hoop, "Pa"), "why": "the retired p r_m/t_eff is not the Lame surface value"})
    disc.append({"id": "retired_thin_wall_longitudinal_free", "quantity": "sigma_z (free)", "wrong": qty(retired_hoop / 2, "Pa"), "why": "hoop/2 has no exact counterpart; free sigma_z = P/As"})
    disc.append({"id": "retired_thin_wall_longitudinal_restrained", "quantity": "sigma_z (restrained)", "wrong": qty(retired_hoop / 2, "Pa"), "why": "restrained sigma_z = 2 nu P/As"})
    ga, Pq = alt(t_nom - mill)
    disc.append({"id": "allowance_not_folded", "quantity": "lame_inner_hoop", "wrong": qty(lame(ga, p)["inner_hoop"], "Pa"), "why": "wall 0.01 - 0.00125: guards the document's authoring (the fold), not the product"})
    disc.append({"id": "allowance_not_folded_sigma_z_free", "quantity": "sigma_z (free)", "wrong": qty(Pq / ga["As"], "Pa"), "why": "guards the document's authoring (the fold), not the product"})
    ga, Pq = alt(wall_authored)
    disc.append({"id": "mill_tolerance_ignored", "quantity": "lame_inner_hoop", "wrong": qty(lame(ga, p)["inner_hoop"], "Pa"), "why": "wall 0.008 without the mill-tolerance reduction"})
    disc.append({"id": "mill_tolerance_ignored_sigma_z_free", "quantity": "sigma_z (free)", "wrong": qty(Pq / ga["As"], "Pa"), "why": "mill tolerance ignored"})
    Pnom = Q(p * F(9, 100) ** 2, 1)
    disc.append({"id": "cap_area_on_nominal_bore", "quantity": "sigma_z (free)", "wrong": qty(Pnom / g["As"], "Pa"), "producible_today": False, "active_from": "T4-U6",
                 "why": "kept as the T4-U6 control: Ai from the nominal bore (ri 0.09) with the reduced As. The folded document never carries the 0.01 wall, so today's product cannot produce it; once D-5 adds the nominal wall and an allowance input it becomes producible"})
    disc.append({"id": "longitudinal_pressure_added_free", "quantity": "sigma_z (free)", "wrong": qty(Q(2) * Q(p) * g["Ai"] / g["As"], "Pa"), "why": "Nw/As plus a separate longitudinal P/As counts the cap twice"})
    Pauth = Q(p * (od / 2 - wall_authored) ** 2, 1)
    s_auth = Pauth / g["As"]
    check("S-3: Ai on the authored bore gives sigma_z = 270848000/20871", s_auth == Q(F(270848000, 20871)))
    disc.append({"id": "poisson_sign_reversed_restrained", "quantity": "Nw (restrained)", "wrong": qty(Q(-2 * nu) * Q(p) * g["Ai"], "N"), "why": "pressure Poisson term as compression"})
    disc.append({"id": "cap_area_on_authored_bore", "quantity": "sigma_z (free)", "wrong": qty(s_auth, "Pa"), "producible_today": True,
                 "why": "the producible mixed basis: the mill reduction applied to As but Ai taken from the authored bore (ri 0.092); today's rule takes Ai from the reduced bore (ri 0.09325)"})
    out["wrong_result_discriminators"] = disc
    out["limits"] = "Long straight homogeneous isotropic annulus, zero external pressure increment, small strain; tiny invented p (2000 Pa) kept from the retired case. Not a corrosion or code-thickness rule."
    return out


# ================================================================= CASE 2: TP-PHYS-008/009 pressure halves
def case_tp_phys():
    od, wall = F(1, 5), F(1, 100)
    g = annulus(od, wall)
    E, nu, alpha, dT, p = 200 * 10 ** 9, F(3, 10), F(3, 250000), F(5), 2 * 10 ** 6
    eps_th = alpha * dT
    check("eps_th == 6e-5", eps_th == F(3, 50000))
    xs = [F(0), F(3, 2), F(9, 2), F(6)]
    L = xs[-1]
    q = F(-300)                        # N/m, global (= local) -Y, on x in [1.5, 4.5]
    a, b = xs[1], xs[2]
    lm = lame(g, p)
    EI = Q(E) * g["I"]
    nodes_id = ["node:A", "node:B", "node:C", "node:D"]
    pipes_id = ["pipe:A-B", "pipe:B-C", "pipe:C-D"]

    # --- closed-form transverse statics and deflection (cantilever, fixed at x=0, free transversely at x=L)
    def V(x):
        lo = max(x, a)
        return q * (b - lo) if lo < b else F(0)

    def M(x):
        if x <= a:
            return q * (b - a) * (a + b - 2 * x) / 2
        if x <= b:
            return q * (b - x) ** 2 / 2
        return F(0)

    def EItheta(x):   # integral of M from 0 (EI v'' = M; v(0) = v'(0) = 0)
        if x <= a:
            return q * (b - a) * ((a + b) * x - x * x) / 2
        t_a = EItheta(a)
        if x <= b:
            return t_a + q * ((b - a) ** 3 - (b - x) ** 3) / 6
        return EItheta(b)

    def EIv(x):
        if x <= a:
            return q * (b - a) * ((a + b) * x * x / 2 - x ** 3 / 3) / 2
        v_a, t_a = EIv(a), EItheta(a)
        if x <= b:
            # integral_a^x [t_a + q((b-a)^3 - (b-s)^3)/6] ds
            return v_a + t_a * (x - a) + q * ((b - a) ** 3 * (x - a) + ((b - x) ** 4 - (b - a) ** 4) / 4) / 6
        return EIv(b) + EItheta(b) * (x - b)

    # --- second method: exact direct stiffness (consistent loads), recovery from end forces
    EI_r = EI.r  # EI = EI_r * pi; beam_fe works with EI_r, so its u is pi * (true displacement)
    u, ends = beam_fe(xs, {1: q}, EI_r)
    for i, x in enumerate(xs):
        check(f"FE v == closed form EIv/EI at x={x}", u[2 * i] == EIv(x) / EI_r)
        check(f"FE theta == closed form EItheta/EI at x={x}", u[2 * i + 1] == EItheta(x) / EI_r)
    for m, (f, Lm, qm) in enumerate(ends):
        x0 = xs[m]
        check(f"member {m} end equilibrium (force)", f[0] + f[2] + qm * Lm == 0)
        for _, s in STATIONS:
            vv, mm = station_from_i_end(f, qm, s * Lm)
            check(f"member {m} station {s} V/M recovery == statics", vv == V(x0 + s * Lm) and mm == M(x0 + s * Lm))
        check(f"member {m} end_j rows equal +V,+M at x_j", f[2] == V(xs[m + 1]) and f[3] == M(xs[m + 1]))
        check(f"member {m} end_i rows equal -V,-M at x_i", f[0] == -V(x0) and f[1] == -M(x0))
    check("root reactions Fy = -V(0) = 900 N, Mz = -M(0) = 2700 N m", -V(F(0)) == 900 and -M(F(0)) == 2700)
    # convention cross-check against the retired 009 hand calculation's unchanged transverse half
    # (q = -2 N/m, EI = 2000: u_y1 = -0.070875 m, theta_z1 = -0.014625 rad; i-side stations V(1.5)=6, M(1.5)=9)
    s009 = F(-2) / q
    check("retired 009 transverse tip: EIv(L) * (-2/q) / 2000 == -0.070875", EIv(L) * s009 / 2000 == F(-70875, 1000000))
    check("retired 009 transverse tip rotation == -0.014625", EItheta(L) * s009 / 2000 == F(-14625, 1000000))
    check("retired 009 i-side station at 1.5 m (V 6, M 9) is the negated j-side cut", (-V(a) * s009, -M(a) * s009) == (6, 9))

    # --- axial: exact three-bar solve with Poisson-thermal eigenload and caps; ends fixed in UX
    def axial_fe(p_on, th_on):
        P = Q(p if p_on else 0) * g["Ai"]
        EA = Q(E) * g["As"]
        eps0 = Q(eps_th if th_on else 0) - (Q(2 * nu) * P / EA)  # alpha dT - 2 nu P/(E As)
        # interior nodes B, C free; A, D fixed. RHS_B = +EA eps0 (AB j) - EA eps0 (BC i) = 0; same at C.
        rhs_B = EA * eps0 - EA * eps0
        rhs_C = EA * eps0 - EA * eps0
        kB = [[(E * g["As"].r) * (1 / F(3, 2) + 1 / F(3)), -(E * g["As"].r) / F(3)],
              [-(E * g["As"].r) / F(3), (E * g["As"].r) * (1 / F(3) + 1 / F(3, 2))]]
        sol = solve_linear(kB, [rhs_B.r, rhs_C.r])
        check("interior axial displacements zero", sol == [0, 0])
        Nw = Q(0) - EA * eps0                    # EA((uj-ui)/L - eps0) with zero strain
        R_A = (EA * eps0) + P                    # -(applied at A) with applied = -EA eps0 - P
        R_D = Q(0) - (EA * eps0) - P
        check("R_A == P - Nw and R_D == Nw - P", R_A == P - Nw and R_D == Nw - P)
        check("global axial balance R_A + R_D + caps == 0", (R_A + R_D) == 0)
        return P, Nw, R_A, R_D

    out = {"case_id": "EXACT-PRESSURE-THERMAL-TRANSVERSE-MIXED-001",
           "rebuilds": {"retired_case_ids": ["MECH-TP-PHYS-008-THERMAL-PRESSURE-AXIAL-EFFECTS (pressure half)",
                                             "MECH-TP-PHYS-009-COMBINED-LOAD-AXIAL-EFFECTS (pressure half)"],
                        "retired_content": "F_pressure = p A_internal = 9 N added with the same sign as the thermal 3 N (total 12 N) on invented non-annular sections (A=4, Ai=0.1, Iy!=Iz)",
                        "kept": "the crate fixtures keep their ids and thermal/transverse halves (unchanged; not part of this rebuild)",
                        "what_changes": "pressure is wall tension 2 nu P (opposite in sign to restrained thermal compression); the caps are applied ledger loads; the section is a real annulus from OD and wall"},
           "partial_span_representation": "The product has no partial-extent primitive load (extents exist only for equivalent_static wind, refused on the exact route). The 009 loaded interval [0.25 L, 0.75 L] is represented exactly by a three-member collinear chain A-B-C-D with a uniform load on member B-C. Euler-Bernoulli members with consistent loads are nodally exact and the station recovery is exact, so the reference is the continuous cantilever closed form.",
           "inputs": {"outside_diameter": qty(od, "m"), "wall_thickness": qty(wall, "m"), "E": qty(E, "Pa"), "nu": qty(nu, "1"),
                      "alpha": qty(alpha, "1/degC"), "temperature_change": qty(dT, "degC"), "thermal_strain": qty(eps_th, "1"),
                      "pressure": qty(p, "Pa"), "q_global_y": qty(q, "N/m"), "x_nodes": [qty(x, "m") for x in xs],
                      "loaded_interval": [qty(a, "m"), qty(b, "m")]},
           "topology": {"nodes": {n: f"x = {rat_str(x)} m, y = z = 0" for n, x in zip(nodes_id, xs)},
                        "members": {"pipe:A-B": "A->B", "pipe:B-C": "B->C (carries q)", "pipe:C-D": "C->D"},
                        "supports": {"support:A": "anchor at A (UX UY UZ RX RY RZ)", "support:D": "line_stop at D (UX only)"},
                        "pressure_region": "one region, members A-B, B-C, C-D; terminals A and D transfers_to_wall (equal bore, collinear; interior caps cancel)",
                        "frame": "local x = global X (i->j), y_reference (0,1,0) -> local y = global Y, local z = global Z"},
           "derived": {"ro": qty(g["ro"], "m"), "ri": qty(g["ri"], "m"), "As": qty(g["As"], "m^2"), "Ai": qty(g["Ai"], "m^2"),
                       "I": qty(g["I"], "m^4"), "J": qty(g["J"], "m^4"), "Z": qty(g["Z"], "m^3"), "EI": qty(EI, "N*m^2"),
                       "E_As": qty(Q(E) * g["As"], "N"), "P": qty(Q(p) * g["Ai"], "N"), "two_nu_P": qty(Q(2 * nu * p) * g["Ai"], "N"),
                       "E_As_eps_th": qty(Q(E * eps_th) * g["As"], "N")},
           "equations": ["axial: both ends held in UX (anchor A, line stop D); total strain 0 in every member",
                         "Nw = E As (0 - eps_th) + 2 nu P  (pressure term +2nuP is tension; restrained thermal term is compression)",
                         "S = Nw - P; sigma_z = Nw/As; root Fx = P - Nw; line-stop Fx = Nw - P; interior UX = 0",
                         "transverse (combined case only): cantilever fixed at A, free in Y and RZ at D; q on [a,b] = [1.5, 4.5] m",
                         "V(x) = q (b - max(x,a)) for x < b else 0;  M(x) = q (b-a)(a+b-2x)/2 on [0,a], q (b-x)^2/2 on [a,b], 0 on [b,L]",
                         "EI v'' = M, v(0) = v'(0) = 0; uy = v; rz = v'",
                         "station rows (quarter_1, midspan, quarter_3) = V, M on the +x face of the i-side segment; end_i rows = -V(x_i), -M(x_i); end_j rows = +V(x_j), +M(x_j) (node-on-element)",
                         "anchor: Fy = -V(0), Mz = -M(0); Fz = Mx = My = 0; line stop: only Fx nonzero"],
           "load_cases": {}}

    for cid, th_on, tr_on in (("case:pressure-half", False, False), ("case:combined", True, True)):
        P, Nw, R_A, R_D = axial_fe(True, th_on)
        st = {"Nw": Nw, "S": Nw - P, "sigma_z": Nw / g["As"]}
        if not th_on:
            check("pressure half: Nw == +2 nu P (tension)", Nw == Q(2 * nu) * P and Nw.r > 0)
        else:
            check("combined: Nw == 2nuP - E As eps_th", Nw == Q(2 * nu) * P - Q(E * eps_th) * g["As"])
        led = Ledger(cid)
        free_ext = Q(eps_th if th_on else 0) * Q(L) + Q((1 - 2 * nu) * L) * P / (Q(E) * g["As"])
        led.scale("scale_force", P, "N", "P = p Ai (axial-family zeros)")
        led.scale("scale_stress", P / g["As"], "Pa", "P/As")
        led.scale("scale_axial_length", free_ext, "m", "free closed-line extension L (eps_th + (1-2nu) P/(E As))")
        if tr_on:
            led.scale("scale_transverse_force", Q(-q * (b - a)), "N", "|q| (b - a) = total transverse load")
            led.scale("scale_transverse_moment", Q(-M(F(0))), "N*m", "|M(0)| root moment")
            led.scale("scale_transverse_length", Q(-EIv(L)) / EI, "m", "|uy(D)|")
            led.scale("scale_rotation", Q(-EItheta(L)) / EI, "rad", "|rz(D)|")
            led.scale("scale_bending_stress", Q(-M(F(0))) / g["Z"], "Pa", "|M(0)|/Z root bending stress")
        else:
            led.scale("scale_transverse_force", P, "N", "P (no transverse load in this case)")
            led.scale("scale_transverse_moment", P * Q(L), "N*m", "P L")
            led.scale("scale_transverse_length", free_ext, "m", "free closed-line extension")
            led.scale("scale_rotation", free_ext / Q(L), "rad", "free extension / L")
            led.scale("scale_bending_stress", P / g["As"], "Pa", "P/As (no transverse load in this case)")
        led.put("zero_force", 0, "N")
        for m, pid in enumerate(pipes_id):
            pressure_rows(led, pid, st, lm, p, "scale_force", "scale_stress")
            vals = {}
            if tr_on:
                x0, x1 = xs[m], xs[m + 1]
                for loc, s in STATIONS:
                    x = x0 + s * (x1 - x0)
                    if loc == "end_i":
                        vals[loc] = (Q(-V(x)), Q(-M(x)))
                    elif loc == "end_j":
                        vals[loc] = (Q(V(x)), Q(M(x)))
                    else:
                        vals[loc] = (Q(V(x)), Q(M(x)))
            transverse_rows(led, pid, pid.replace("pipe:", ""), vals, "scale_transverse_force", "scale_transverse_moment")
            secm = {}
            mmax = Q(0)
            if tr_on:
                x0, x1 = xs[m], xs[m + 1]
                for loc, s in STATIONS:
                    secm[loc] = Q(M(x0 + s * (x1 - x0)))
                mmax = Q(max(abs(M(x0)), abs(M(x1))))   # |M| is monotone on each member here
                check(f"{pid}: |M| maximum at a member end", all(abs(M(x0 + F(k, 64) * (x1 - x0))) <= mmax.r for k in range(65)))
            stress_rows(led, pid, pid.replace("pipe:", ""), secm, g["Z"], "scale_bending_stress")
            maximum_row(led, pid, pid.replace("pipe:", ""), st["sigma_z"], mmax, g["Z"])
        for i, (nid, x) in enumerate(zip(nodes_id, xs)):
            uy = Q(EIv(x)) / EI if tr_on else Q(0)
            rz = Q(EItheta(x)) / EI if tr_on else Q(0)
            # zero UX: axial scale; transverse zeros: transverse scales
            for axis, val, sc in (("x", Q(0), "scale_axial_length"), ("y", uy, "scale_transverse_length"), ("z", Q(0), "scale_transverse_length")):
                name = f"{nid[5:]}_u{axis}" if val.r != 0 else "zero_length"
                led.row(f"global_nodal_displacement_{axis}", nid, f"nodal_displacement_{axis}", "node", "mm", "global", name, val, sc, ref_unit="m",
                        refreeze=["T4-U8: re-freeze if D-6 makes Timoshenko the exact-route default (Euler-Bernoulli deflection)"] if (axis == "y" and val.r != 0) else None)
            for axis, val in (("x", Q(0)), ("y", Q(0)), ("z", rz)):
                name = f"{nid[5:]}_r{axis}" if val.r != 0 else "zero_rotation"
                led.row(f"global_nodal_rotation_{axis}", nid, f"nodal_rotation_{axis}", "node", "rad", "global", name, val, "scale_rotation")
        fy = Q(-V(F(0))) if tr_on else Q(0)
        mz = Q(-M(F(0))) if tr_on else Q(0)
        for comp, val, sc in zip(["Fx", "Fy", "Fz", "Mx", "My", "Mz"], [R_A, fy, Q(0), Q(0), Q(0), mz],
                                 ["scale_force", "scale_transverse_force", "scale_transverse_force", "scale_transverse_moment", "scale_transverse_moment", "scale_transverse_moment"]):
            unit = "N" if comp[0] == "F" else "N*m"
            name = f"A_{comp}" if val.r != 0 else ("zero_force" if unit == "N" else "zero_moment")
            led.row("support_reaction_component_v2", "support:A", comp, "node", unit, "global", name, val, sc)
        magnitude_rows(led, "support:A", [R_A, fy, Q(0)], [Q(0), Q(0), mz], "scale_force", "scale_transverse_moment", "A")
        for comp, val, sc in zip(["Fx", "Fy", "Fz", "Mx", "My", "Mz"], [R_D, Q(0), Q(0), Q(0), Q(0), Q(0)],
                                 ["scale_force", "scale_transverse_force", "scale_transverse_force", "scale_transverse_moment", "scale_transverse_moment", "scale_transverse_moment"]):
            unit = "N" if comp[0] == "F" else "N*m"
            name = f"D_{comp}" if val.r != 0 else ("zero_force" if unit == "N" else "zero_moment")
            led.row("support_reaction_component_v2", "support:D", comp, "node", unit, "global", name, val, sc)
        magnitude_rows(led, "support:D", [R_D, Q(0), Q(0)], [Q(0), Q(0), Q(0)], "scale_force", "scale_transverse_moment", "D")
        # global balance: reactions + applied (caps cancel; transverse W) == 0
        check(f"{cid}: Fy balance", fy.r + (q * (b - a) if tr_on else 0) == 0)
        check(f"{cid}: Mz balance about A", mz.r + (q * (b * b - a * a) / 2 if tr_on else 0) == 0)
        # magnitudes
        jd = led.to_json(f"/cases/tp_phys_pressure_halves/load_cases/{cid}")
        out["load_cases"][cid] = jd
        out.setdefault("checks", {})[cid] = {"Nw": qty(Nw, "N"), "S": qty(Nw - P, "N"), "pressure_wall_tension_2nuP": qty(Q(2 * nu) * P, "N"),
                                             "thermal_wall_compression": qty(Q(-E * eps_th if th_on else 0) * g["As"], "N")}

    # documents
    nodes = [node(n, x) for n, x in zip(nodes_id, xs)]
    pipes = [pipe(pid, nodes_id[m], nodes_id[m + 1], od, wall, "material:tp") for m, pid in enumerate(pipes_id)]
    sups = [support("support:A", "node:A", "anchor", ["UX", "UY", "UZ", "RX", "RY", "RZ"]), support("support:D", "node:D", "line_stop", ["UX"])]
    mat = [material("material:tp", E, nu, alpha)]
    reg = region("region:tp", pipes_id, p, "node:A", "node:D")
    thermal = [{"id": f"load:thermal:{pid[5:]}", "category": "thermal", "target": {"type": "element", "pipe": pid}, "direction": "global_x",
                "magnitude": {"value": fnum(dT), "unit": "degC"}, "dimension": "temperature_interval", "provenance": PROV} for pid in pipes_id]
    transverse = {"id": "load:transverse:B-C", "category": "weight", "target": {"type": "element", "pipe": "pipe:B-C"}, "direction": "global_y",
                  "magnitude": {"value": fnum(q), "unit": "N/m"}, "dimension": "force_per_length", "provenance": PROV}
    cases = [{"id": "case:pressure-half", "primitive_loads": [], "pressure_regions": [reg], "provenance": PROV},
             {"id": "case:combined", "primitive_loads": thermal + [transverse], "pressure_regions": [reg], "provenance": PROV}]
    doc = doc_030("project:t4-i8-tp-phys", nodes, pipes, sups, mat, cases)
    out["documents"] = documents(doc, {"case:combined": {"thermal": {pid: (alpha, dT) for pid in pipes_id}}})

    P = Q(p) * g["Ai"]
    EAth = Q(E * eps_th) * g["As"]
    tnp = Q(2 * nu) * P
    Nw_c = tnp - EAth
    out["wrong_result_discriminators"] = [
        {"id": "legacy_same_sign_thrust", "case": "case:combined", "quantity": "Nw", "wrong": qty(Q(0) - EAth - P, "N"), "why": "retired premise: +pAi thrust with the sign of the thermal force"},
        {"id": "pressure_as_compression", "case": "case:combined", "quantity": "Nw", "wrong": qty(Q(0) - EAth - tnp, "N"), "why": "Poisson term with the thermal sign"},
        {"id": "poisson_term_omitted", "case": "case:combined", "quantity": "Nw", "wrong": qty(Q(0) - EAth, "N"), "why": "pressure eigenstrain missing"},
        {"id": "caps_subtracted_in_recovery", "case": "case:combined", "quantity": "Nw", "wrong": qty(Nw_c - P, "N"), "why": "S published as wall force"},
        {"id": "S_sign_reversed", "case": "case:combined", "quantity": "S", "wrong": qty(P - Nw_c, "N"), "why": "S = P - Nw"},
        {"id": "legacy_same_sign_reaction", "case": "case:combined", "quantity": "support:A Fx", "wrong": qty(EAth + P, "N"), "why": "retired premise: root reaction = -(thermal + pAi equivalent pair) = E As alpha dT + pAi"},
        {"id": "pressure_half_compression", "case": "case:pressure-half", "quantity": "Nw", "wrong": qty(Q(0) - tnp, "N"), "why": "2 nu P as compression"},
        {"id": "pressure_half_legacy_thrust", "case": "case:pressure-half", "quantity": "Nw", "wrong": qty(Q(0) - P, "N"), "why": "legacy -pAi end pair on a restrained line"},
        {"id": "pressure_half_caps_subtracted", "case": "case:pressure-half", "quantity": "Nw", "wrong": qty(tnp - P, "N"), "why": "S published as wall force"},
        {"id": "station_sign_i_side", "case": "case:combined", "quantity": "pipe:A-B midspan bending_moment_z", "wrong": qty(-M(F(3, 4)), "N*m"), "why": "i-side cut convention instead of the product's j-side cut"},
        {"id": "full_span_load", "case": "case:combined", "quantity": "node:D uy", "wrong": qty(Q((q * (b - a) / L) * L ** 4 / 8) / EI, "m"), "why": "the same total load spread over the whole span (same root Fy and Mz, different deflection)"},
    ]
    out["refreeze"] = {"unit": "T4-U8", "condition": "re-freeze if D-6 makes Timoshenko (energy-matched annular factor) the exact-route default",
                       "rows": "global_nodal_displacement_y at node:B, node:C, node:D of case:combined (rows tagged 'refreeze'): Euler-Bernoulli lateral deflections; a shear-deformable cantilever adds the integral of V/(kappa G A) (about 0.3 % at node:D)",
                       "stay": "statics (V, M, bending stress, maximum), reactions and their norms, every pressure row and all axial values; rz rows stay if the published rotation is the section rotation (theta' = M/EI with theta(0) = 0 is unchanged for this statically determinate cantilever), to be confirmed in T4-U8",
                       "sp1_note": "v2 stays byte-identical (Euler-Bernoulli); the v3 side of this case's SP-1 pair must then select Euler-Bernoulli explicitly, or SP-1's straight-v3 clause is scoped to Euler-Bernoulli selections (T4-U8 planning point, T4-RV4 S-2)"}
    out["limits"] = "Long straight annulus, linear statics, Euler-Bernoulli, small displacement; tip line stop restrains UX only. Interior caps cancel (equal bore). The straight elastic maximum is frozen here: it already contains the pressure-coupled |Nw/As| today (PP/src/lib.rs:5399-5446, 10340-10420@ed012c7ccf); T4-U4's maximum is the arc maximum."
    return out


# ================================================================= CASE 3: PRESSURE-MEMBRANE
def case_membrane():
    p, rm, t = 100, F(3), F(1, 2)
    od, wall = 2 * rm + t, t
    g = annulus(od, wall)
    E, nu, L = 200 * 10 ** 9, F(3, 10), F(65)
    lm = lame(g, p)
    st = axial_state(g, E, nu, p, 0, False)
    thin_h = F(p) * rm / t
    thin_l = thin_h / 2
    mean_h = F(p) * g["ri"] / t
    check("thin hoop 600, longitudinal 300 (retired)", thin_h == 600 and thin_l == 300)
    check("Lame inner hoop = thin + p t/(4 rm)", lm["inner_hoop"] == thin_h + F(p) * t / (4 * rm))
    check("Lame outer hoop = thin - p + p t/(4 rm)", lm["outer_hoop"] == thin_h - p + F(p) * t / (4 * rm))
    check("sigma_z = thin_l - p/2 + p t/(8 rm)", st["sigma_z"] == Q(thin_l - F(p, 2) + F(p) * t / (8 * rm)))
    check("through-thickness mean hoop = p ri/t (equilibrium)", mean_h == F(p) * g["ri"] / t)
    rel = lambda thin, ref: (F(thin) - ref) / ref
    comp = {"thin_hoop_p_rm_over_t": qty(thin_h, "Pa"), "thin_longitudinal_p_rm_over_2t": qty(thin_l, "Pa"),
            "relative_difference_thin_hoop_vs_lame_inner": qty(rel(thin_h, lm["inner_hoop"]), "1"),
            "relative_difference_thin_hoop_vs_lame_outer": qty(rel(thin_h, lm["outer_hoop"]), "1"),
            "relative_difference_thin_hoop_vs_mean_hoop_p_ri_over_t": qty(rel(thin_h, mean_h), "1"),
            "relative_difference_thin_longitudinal_vs_sigma_z_free": qty(rel(thin_l, st["sigma_z"].r), "1"),
            "limit_forms": ["sigma_h(ri) = p rm/t + p t/(4 rm)", "sigma_h(ro) = p rm/t - p + p t/(4 rm)",
                            "sigma_z(free, closed) = p rm/(2t) - p/2 + p t/(8 rm)",
                            "differences are O(p) while the thin-wall values are O(p rm/t): relative differences -> 0 as t/rm -> 0"],
            "reference_rule": "the Lame values are the reference; the thin-wall values are comparison data only (also wrong-result discriminators)"}
    led = Ledger("case:membrane-free")
    P = st["P"]
    ext = Q(st["strain"] * L)
    led.scale("scale_force", P, "N", "P = p Ai")
    led.scale("scale_moment", P * Q(L), "N*m", "P L")
    led.scale("scale_stress", P / g["As"], "Pa", "P/As")
    led.scale("scale_length", ext, "m", "free closed-tube extension")
    led.scale("scale_rotation", ext / Q(L), "rad", "free extension / L")
    pressure_rows(led, "pipe:A-B", st, lm, p, "scale_force", "scale_stress")
    transverse_rows(led, "pipe:A-B", "AB", {}, "scale_force", "scale_moment")
    stress_rows(led, "pipe:A-B", "AB", {}, g["Z"], "scale_stress")
    maximum_row(led, "pipe:A-B", "AB", st["sigma_z"], Q(0), g["Z"])
    node_rows(led, "node:A", Q(0), Q(0), Q(0), "scale_length", "scale_rotation", "A")
    node_rows(led, "node:B", ext, Q(0), Q(0), "scale_length", "scale_rotation", "B")
    support_rows(led, "support:A", [Q(0)] * 6, "scale_force", "scale_moment", "A")
    nodes = [node("node:A", 0), node("node:B", L)]
    pipes = [pipe("pipe:A-B", "node:A", "node:B", od, wall, "material:membrane")]
    sups = [support("support:A", "node:A", "anchor", ["UX", "UY", "UZ", "RX", "RY", "RZ"])]
    cases = [{"id": "case:membrane-free", "primitive_loads": [], "pressure_regions": [region("region:membrane", ["pipe:A-B"], p, "node:A", "node:B")], "provenance": PROV}]
    doc = doc_030("project:t4-i8-membrane", nodes, pipes, sups, [material("material:membrane", E, nu)], cases)
    return {"case_id": "EXACT-PRESSURE-LAME-THIN-WALL-LIMIT-001",
            "rebuilds": {"retired_case_id": "STRESS-PRESSURE-MEMBRANE-ORIGINAL", "retired_values": ["pressure_hoop = 600 Pa", "pressure_longitudinal = 300 Pa"],
                         "retired_inputs": "p = 100 Pa, membrane radius 3 m, wall 0.5 m", "retired_by": "T3's U3 (case removed entirely; I4 0.1)"},
            "inputs": {"pressure": qty(p, "Pa"), "membrane_radius_retired": qty(rm, "m"), "outside_diameter": qty(od, "m"),
                       "wall_thickness": qty(wall, "m"), "E": qty(E, "Pa"), "nu": qty(nu, "1"), "length": qty(L, "m")},
            "derived": {"ro": qty(g["ro"], "m"), "ri": qty(g["ri"], "m"), "As": qty(g["As"], "m^2"), "Ai": qty(g["Ai"], "m^2"),
                        "I": qty(g["I"], "m^4"), "J": qty(g["J"], "m^4"), "Z": qty(g["Z"], "m^3"), "P": qty(P, "N"),
                        "through_thickness_mean_hoop": qty(mean_h, "Pa")},
            "equations": ["OD = 2 rm + t; ro = OD/2 = 3.25; ri = ro - t = 2.75", "Lame surface values as case 1",
                          "free closed tube (anchor A, B free, both terminals transferring): Nw = P; S = 0; sigma_z = P/As; uB = (1-2nu) P L/(E As)"],
            "thin_wall_comparison": comp,
            "load_cases": {"case:membrane-free": led.to_json("/cases/pressure_membrane_thin_wall_limit/load_cases/case:membrane-free")},
            "documents": documents(doc, {}),
            "wrong_result_discriminators": [
                {"id": "thin_hoop", "quantity": "lame_inner_hoop and lame_outer_hoop", "wrong": qty(thin_h, "Pa"), "why": "retired thin-wall value"},
                {"id": "thin_longitudinal", "quantity": "sigma_z", "wrong": qty(thin_l, "Pa"), "why": "retired thin-wall value"},
                {"id": "mean_hoop_as_surface", "quantity": "lame_inner_hoop", "wrong": qty(mean_h, "Pa"), "why": "through-thickness mean is not a surface value"}],
            "limits": "Thick wall (t/rm = 1/6), long straight annulus (L/OD = 10); invented 6.5 m bore kept from the retired case for traceability."}


# ================================================================= CASE 4: v3 straight twin
def case_twin(path):
    raw = open(path, "rb").read()
    sha = hashlib.sha256(raw).hexdigest()
    fx = json.loads(raw)
    c0 = fx["cases"][0]
    check("twin fixture case[0] id is 'ordinary'", c0["id"] == "ordinary")
    inp = c0["inputs"]
    # exact binary64 inputs (the fixture's source semantics)
    od, wall, p, E, nu, L, Fy, Mx = (F(inp[k]) for k in ("OD_m", "wall_m", "p_Pa", "E_Pa", "nu", "L_m", "tip_Fy_N", "tip_Mx_Nm"))
    g = annulus(od, wall)
    P = Q(p) * g["Ai"]
    G = E / (2 * (1 + nu))
    vals = {"A_m2": g["As"], "Ai_m2": g["Ai"], "I_m4": g["I"], "J_m4": g["J"], "Z_m3": g["Z"], "P_N": P, "G_Pa": Q(G),
            "free_pressure_extension_m": Q((1 - 2 * nu) * L) * P / (Q(E) * g["As"]),
            "tip_bending_y_m": Q(Fy * L ** 3 / (3 * E)) / g["I"], "tip_rotation_z_rad": Q(Fy * L ** 2 / (2 * E)) / g["I"],
            "tip_rotation_x_rad": Q(Mx * L / G) / g["J"], "axial_membrane_Pa": P / g["As"],
            "lame_inner_radial_Pa": Q(-p), "lame_outer_radial_Pa": Q(0),
            "lame_inner_hoop_Pa": Q(lame(g, p)["inner_hoop"]), "lame_outer_hoop_Pa": Q(lame(g, p)["outer_hoop"]),
            "root_force_Fy_N": Q(-Fy), "root_moment_Mz_Nm": Q(-Fy * L), "root_moment_Mx_Nm": Q(-Mx)}
    worst = 0.0
    for k, q in vals.items():
        ref = c0[k]["f64"]
        mine = q.f64()
        rel = 0.0 if ref == mine == 0 else abs(mine - ref) / max(abs(ref), 1e-300)
        worst = max(worst, rel)
        check(f"twin re-derivation {k} within 1e-14 rel of fixture", rel <= 1e-14)
    mx = (P / g["As"]).dec() + (Q(Fy * L) / g["Z"]).dec()
    rel_mx = abs(float(mx) - c0["maximum_absolute_normal_stress_Pa"]["f64"]) / c0["maximum_absolute_normal_stress_Pa"]["f64"]
    check("twin re-derivation max |normal| = P/As + Fy L/Z within 1e-14", rel_mx <= 1e-14)
    tau = (Q(Mx) * Q(g["ro"]) / g["J"]).f64()
    check("twin torsional surface shear within 1e-14", abs(tau - c0["torsional_surface_shear_Pa"]["f64"]) / tau <= 1e-14)
    print(f"twin: fixture sha256 {sha}; {len(vals) + 2} values re-derived; worst relative difference {max(worst, rel_mx):.3e}")
    # documents (the consumer's fixture(inputs, fixed=false, pressurized=true), test-file constants)
    A, B, PIPE, REG, CASE = "node:section-a", "node:section-b", "pipe:source-section", "region:source-section", "case:source-section"
    loads = [{"id": "load:tip-y", "category": "concentrated_force", "target": {"type": "node", "node": B}, "direction": "global_y",
              "dimension": "force", "magnitude": {"value": inp["tip_Fy_N"], "unit": "N"}, "provenance": "independent_section_geometry_reference"},
             {"id": "load:tip-torque", "category": "concentrated_moment", "target": {"type": "node", "node": B}, "direction": "rotation_x",
              "dimension": "moment", "magnitude": {"value": inp["tip_Mx_Nm"], "unit": "N*m"}, "provenance": "independent_section_geometry_reference"}]
    reg = {"id": REG, "member_pipe_ids": [PIPE], "pressure_basis": "internal_differential_zero_external_v1",
           "pressure": {"value": inp["p_Pa"], "unit": "Pa"},
           "terminals": [{"node_ref": A, "closure_transfer": "transfers_to_wall", "provenance": "explicit_test_closure"},
                         {"node_ref": B, "closure_transfer": "transfers_to_wall", "provenance": "explicit_test_closure"}],
           "provenance": "independent_geometry_pressure_reference"}
    doc = {"model": {"schema_version": "0.3.0", "document_kind": "openpipestress.product_preview.model",
                     "pressure_contract": {"version": "2.0.0", "mode": "exact_straight_pressure_v2"},
                     "project": {"id": "project:section-oracle", "units": UNITS},
                     "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing",
                                         "professional_acceptance": "not_provided"},
                     "nodes": [{"id": A, "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": "synthetic"},
                               {"id": B, "position": {"x": inp["L_m"], "y": 0.0, "z": 0.0}, "provenance": "synthetic"}],
                     "pipe_segments": [{"id": PIPE, "from": A, "to": B, "section": {"outside_diameter": {"value": inp["OD_m"], "unit": "m"},
                                        "wall_thickness": {"value": inp["wall_m"], "unit": "m"}}, "material": "material:section",
                                        "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0}, "provenance": "arithmetic_geometry_control_not_manufactured_pipe"}],
                     "materials": [{"id": "material:section", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
                                    "elastic_modulus": {"value": inp["E_Pa"], "unit": "Pa"}, "poisson_ratio": {"value": inp["nu"], "unit": "1"},
                                    "provenance": "synthetic_isotropic_input"}],
                     "supports": [{"id": "support:section-a", "node": A, "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"],
                                   "provenance": "independent_section_geometry_control"}], "components": [],
                     "load_cases": [{"id": CASE, "primitive_loads": loads, "pressure_regions": [reg], "provenance": "independent_reference"}],
                     "combinations": []}, "materials": []}
    return {"case_id": "EXACT-PRESSURE-V3-STRAIGHT-TWIN-ODWALL-001",
            "twin_of": {"fixture": "PP/tests/fixtures/pressure_reference/SOURCE_ODWALL_EXPECTATIONS.json", "fixture_sha256_at_ed012c7ccf": sha,
                        "case_index": 0, "case_id": "ordinary",
                        "v2_consumer": "PP/tests/pressure_section_geometry.rs: fixture(inputs, fixed=false, pressurized=true); test ordinary_source_annulus_properties_and_all_responses; check_response and check_geometry",
                        "why_this_one": "one document exercises every v2 row family at once: extension, bending, torsion, Nw/S/membrane/Lame at five stations, endpoint wall actions, the elastic normal maximum, six-component reactions and the section evidence; its references are already independent and frozen"},
            "expectation": {"rule": "/sp1_pair_scope (all three clauses), for this document per schema version (0.3.0, 0.4.0) and per mode; p = 2 MPa >= 0",
                            "refreeze": {"unit": "T4-U8", "condition": "re-freeze if D-6 makes Timoshenko the exact-route default",
                                         "affected": "the fixture's tip_bending_y_m (Euler-Bernoulli F L^3/(3EI)) as an independent value for the v3 run; the maximum, reactions, rotations (section rotation), torsion and all pressure rows stay",
                                         "sp1_note": "v2 stays byte-identical (Euler-Bernoulli); the v3 twin must then select Euler-Bernoulli explicitly, or SP-1's straight-v3 clause is scoped to Euler-Bernoulli selections (T4-U8 planning point, T4-RV4 S-2)"},
                            "independent_values": f"the v3 run must also meet the fixture's frozen values at relative 1e-9 with the consumer test's zero scales (check_response: zero targets use the free extension, the membrane stress or P as scale); T4-I8 re-derived {len(vals) + 2} of them in exact arithmetic from the binary64 inputs; worst relative difference from the fixture's binary64 values {max(worst, rel_mx):.1e} (check threshold 1e-14)"},
            "documents": {"v2_model_0.3.0": doc, "v2_model_0.4.0": to_040(doc, {}), "v3_patch_for_both": V3_PATCH, "sp1_pair": "/sp1_pair_scope",
                          "note": "the 0.3.0 document is the consumer test's fixture() output for case 0, field for field; bit-equality compares v3 with v2 within one schema version and one mode"}}


# ================================================================= CASE 5: second SP-1 twin, separate closures
def case_twin_separate():
    """PP/tests/pressure_runtime.rs model(fixed=true, transfer=false, thermal=0.0), state 4 of
    six_si_pressure_states_through_both_public_solver_modes. No fixture under pressure_reference/ has
    separately supported closures; this existing v2 document and its frozen values serve."""
    od, wall = F(12, 100), F(1, 100)
    g = annulus(od, wall)
    E, nu, p, L = 200 * 10 ** 9, F(3, 10), 2 * 10 ** 6, F(6)
    lm = lame(g, p)
    P = Q(p) * g["Ai"]
    Nw = Q(2 * nu) * P                      # both ends held, no caps on the wall: Nw = E As (0 - 0) + 2 nu P
    check("separate-closure twin: Nw equals the restrained compliance state", Nw == axial_state(g, E, nu, p, 0, True)["Nw"])
    st = {"Nw": Nw, "S": Nw - P, "sigma_z": Nw / g["As"]}
    R_A, R_B = Q(0) - Nw, Nw                # node free bodies without cap loads
    # existing frozen oracle values (PP/tests/pressure_runtime.rs:438-446@ed012c7ccf): wall 3000pi, effective -2000pi, root reaction -3000pi
    check("separate-closure twin agrees with the frozen six-state values", Nw == Q(3000, 1) and st["S"] == Q(-2000, 1) and R_A == Q(-3000, 1))
    free_wall = Q(-2 * nu * L) * P / (Q(E) * g["As"])   # same document with the tip released: free wall contracts
    check("free-wall contraction -9/110000 m", free_wall == Q(F(-9, 110000)))
    cid = "case:pressure-oracle"
    led = Ledger(cid)
    led.scale("scale_force", P, "N", "P = p Ai")
    led.scale("scale_moment", P * Q(L), "N*m", "P L")
    led.scale("scale_stress", P / g["As"], "Pa", "P/As")
    led.scale("scale_length", Q(-free_wall.r), "m", "|free-wall contraction| 2 nu P L/(E As) of the same document with the tip released")
    led.scale("scale_rotation", Q(-free_wall.r) / Q(L), "rad", "that contraction / L")
    pressure_rows(led, "pipe:A-B", st, lm, p, "scale_force", "scale_stress")
    transverse_rows(led, "pipe:A-B", "AB", {}, "scale_force", "scale_moment")
    stress_rows(led, "pipe:A-B", "AB", {}, g["Z"], "scale_stress")
    maximum_row(led, "pipe:A-B", "AB", st["sigma_z"], Q(0), g["Z"])
    node_rows(led, "node:A", Q(0), Q(0), Q(0), "scale_length", "scale_rotation", "A")
    node_rows(led, "node:B", Q(0), Q(0), Q(0), "scale_length", "scale_rotation", "B")
    support_rows(led, "support:A", [R_A, Q(0), Q(0), Q(0), Q(0), Q(0)], "scale_force", "scale_moment", "A")
    support_rows(led, "support:B", [R_B, Q(0), Q(0), Q(0), Q(0), Q(0)], "scale_force", "scale_moment", "B")
    O = "independent_pressure_runtime_oracle"
    anchor_ = lambda i, n: {"id": i, "node": n, "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": O}
    term = lambda n: {"node_ref": n, "closure_transfer": "separately_supported_or_compensated", "provenance": "explicit_independent_pressure_boundary"}
    doc = {"model": {"schema_version": "0.3.0", "document_kind": "openpipestress.product_preview.model",
                     "pressure_contract": {"version": "2.0.0", "mode": "exact_straight_pressure_v2"},
                     "project": {"id": "project:pressure-oracle", "units": UNITS},
                     "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing",
                                         "professional_acceptance": "not_provided"},
                     "nodes": [{"id": "node:A", "position": {"x": 0.0, "y": 0.0, "z": 0.0}, "provenance": O},
                               {"id": "node:B", "position": {"x": 6.0, "y": 0.0, "z": 0.0}, "provenance": O}],
                     "pipe_segments": [{"id": "pipe:A-B", "from": "node:A", "to": "node:B",
                                        "section": {"outside_diameter": {"value": 0.12, "unit": "m"}, "wall_thickness": {"value": 0.01, "unit": "m"}},
                                        "material": "material:one", "y_reference": {"x": 0.0, "y": 0.0, "z": 1.0}, "provenance": "synthetic_straight_annulus"}],
                     "supports": [anchor_("support:A", "node:A"), anchor_("support:B", "node:B")], "components": [],
                     "materials": [{"id": "material:one", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
                                    "elastic_modulus": {"value": 200e9, "unit": "Pa"}, "poisson_ratio": {"value": 0.3, "unit": "1"},
                                    "thermal_expansion_coefficient": {"value": 0.000012, "unit": "1/degC"}, "temperature_points": [],
                                    "provenance": "synthetic_isotropic_reference_no_catalog"}],
                     "load_cases": [{"id": cid, "primitive_loads": [], "pressure_regions": [{
                         "id": "region:pressure-oracle", "member_pipe_ids": ["pipe:A-B"], "pressure_basis": "internal_differential_zero_external_v1",
                         "pressure": {"value": 2e6, "unit": "Pa"}, "terminals": [term("node:A"), term("node:B")],
                         "provenance": "independent_pressure_region"}], "provenance": "synthetic_load_case"}],
                     "combinations": []}, "materials": []}
    return {"case_id": "EXACT-PRESSURE-V3-STRAIGHT-TWIN-SEPARATE-CLOSURES-001",
            "twin_of": {"document": "PP/tests/pressure_runtime.rs model(fixed=true, transfer=false, thermal=0.0) at ed012c7ccf (state 4 of six_si_pressure_states_through_both_public_solver_modes, lines 432-477)",
                        "why": "SP-1 coverage of separately supported closures (T4-RV4 N-4); no fixture under pressure_reference/ has them",
                        "frozen_values": "the test's inline frozen values (wall 3000pi N, effective -2000pi N, root Fx -3000pi N, tip Fx +3000pi N, zero extension); independently: PRESSURE_REFERENCE_QUALIFICATION section 3 item 2 (restrained separate closures: Nw = 2 nu P, pipe reactions [-2 nu P, +2 nu P])"},
            "equations": ["no cap load on the wall (separately supported closures); both ends held: strain 0", "Nw = 2 nu P; S = Nw - P = (2 nu - 1) P; root Fx = -Nw; far Fx = +Nw",
                          "the remote closure supports carry +-P outside the pipe reaction ledger"],
            "frame": "y_reference (0,0,1) as in the test: local y = global Z, local z = -global Y (transverse rows are zero)",
            "load_cases": {cid: led.to_json("/cases/v3_straight_twin_separate_closures/load_cases/case:pressure-oracle")},
            "expectation": {"rule": "/sp1_pair_scope (all three clauses), per schema version and per mode; p = 2 MPa >= 0",
                            "independent_values": "the rows above at relative 1e-9 with their zero scales, in both modes"},
            "documents": {"v2_model_0.3.0": doc, "v2_model_0.4.0": to_040(doc, {}), "v3_patch_for_both": V3_PATCH, "sp1_pair": "/sp1_pair_scope",
                          "note": "the 0.3.0 document is the test's model(true, false, 0.0) output field for field"},
            "limits": "Straight annulus; separate closures must be supported elsewhere (their +-P is disclosed outside the pipe ledger)."}


SP1_PAIR_SCOPE = {
    "applies_to": "every v2/v3 document pair in this file (cases 1-3 and both twins); recommended mechanically for every existing v2 exact document in PP's tests with p >= 0, which also covers reversed member and terminal order, unequal-wall chains and mixed closures (T4-RV4 N-4)",
    "inputs": "v2-admissible inputs only: p >= 0 in every region. v3 admits signed pressure and v2 refuses p < 0 after T4-U0, so a p < 0 v3 document has no v2 twin",
    "clause_1_v2_anchor": "the candidate's serialized v2 envelope is byte-identical to the base revision's for the same document and mode (base = the PR's merge base on main); T4-U0's declared p < 0 refusal lies outside the p >= 0 scope",
    "clause_2_v3_equals_v2": "every numeric JSON leaf of the candidate's v3 envelope equals the candidate's v2 leaf at the same path as an f64 bit pattern (sign of zero included); arrays are compared in order; every non-numeric leaf is equal too, except the closed exclusion list",
    "closed_exclusion_list": [
        "E1: producer.semantic_contract_id (string)",
        "E2: formulation_basis.profile_id and formulation_basis.limitations (strings: approximation and limitation text)",
        "E3: string leaves of contract_evidence that state the pressure contract version and mode, the result-semantics identity or the admitted families; T4-U2a enumerates their paths in the PR that introduces them, and no existing v2 key changes value",
        "E4: diagnostics[].message text where it names the contract (id, code, severity and affected_refs are compared)",
        "E5: result-row kind strings, only through a declared one-to-one kind correspondence if T4-U2a renames kinds under pressure-1 (identity otherwise)"],
    "no_numeric_exclusion": "no numeric leaf is excluded: results values, summary, numerical_quality, source_block_recovery, diagnostics numerics and every numeric contract_evidence leaf, including material E/nu/G/alpha, eigenload_pair_local_n, cap pairs, terminal cap forces, assembly-term coefficients and assembled forces (PP/src/pressure_runtime.rs:642-647, 686-689, 880-883), the exact section evidence, pipe_stress_extrema (PP/src/lib.rs:5569) and 0.4.0 load_reference_states evidence (PP/src/lib.rs:2820), all at ed012c7ccf",
    "clause_3_scope": "per mode separately and per schema version separately; cross-mode and cross-schema equality are not claimed",
    "standing": "the same per-case standing and formation-guard verdicts follow from clause 2 (they are leaves of the envelope)",
    "basis": "plan section 2 SP-1; H-1; T4-RV4 S-4",
}


# ================================================================= main
def main():
    fixture_path, out_path = sys.argv[1], sys.argv[2]
    self_test_qualified_tables()
    cases = {"milltol_lame_membrane": case_milltol(), "tp_phys_pressure_halves": case_tp_phys(),
             "pressure_membrane_thin_wall_limit": case_membrane(), "v3_straight_twin": case_twin(fixture_path),
             "v3_straight_twin_separate_closures": case_twin_separate()}
    doc = {
        "schema_version": "independent.exact_pressure_rebuilt_reference_cases/1.0.0",
        "purpose": "Independent analytical references for T4-U2's rebuilt straight cases (VP-STATIC exact_pressure_1). Frozen before any implementation; not product request DTOs or observed solver outputs.",
        "status": "frozen_by_T4-I8_awaiting_independent_refutation",
        "author": {"task": "T4-I8", "role": "TASK (Type 2)", "brief": "R4/BRIEFS/T4-I8_U2_STRAIGHT_REBUILT_REFERENCES.md",
                   "brief_sha256": "1f294de53fa8dfbfc0bb63aa9300b4567332cbc884c6cb37353d5c315a7aa134", "records_revision": "0c17c8d352"},
        "conventions_basis": {"code_revision": "ed012c7ccf", "use": "read for conventions only (section rule, row kinds, stations, frames, signs); no value taken from the product"},
        "numbers": "Invented test quantities; decimal inputs are exact rational targets; no material-library, component or code-rule data.",
        "numeric_representation": {"exact_is_authoritative": True, "kinds": ["rational", "rational_times_pi", "rational_over_pi",
                                   "symbolic: 'sqrt((a*pi)^2 + (b)^2)' (one support force norm) and 'a + (b)/pi' (two elastic maxima); T1's generator needs these two forms added to its SYMBOLIC pattern, or the three rows left out"],
                                   "pi_decimal": PI_STR_100, "pi_method": "Machin, checked against Gauss-Legendre to 1e-125",
                                   "decimal_precision": 85, "value": "binary64 rounding of the exact value; a convenience, never a product observation",
                                   "input_rounding": "authored decimals (0.2, 0.3, 0.00125, ...) are generally not binary64-exact; the 1e-9 criterion absorbs the operand rounding"},
        "sign_conventions": SIGN,
        "row_kinds": {
            "pipe_wall_axial_force_v2/wall_axial_force": "N, element_local, five stations; tension-positive Nw (PP/src/lib.rs:11557@ed012c7ccf)",
            "pipe_effective_axial_force_v2/effective_axial_force": "N, element_local, five stations; S = Nw - pAi",
            "pipe_axial_membrane_stress_v2/axial_membrane_stress": "Pa, pipe_section, five stations; Nw/As",
            "pipe_lame_radial_stress_v2/lame_{inner,outer}_radial_stress": "Pa, pipe_section, five stations",
            "pipe_lame_hoop_stress_v2/lame_{inner,outer}_hoop_stress": "Pa, pipe_section, five stations (PP/src/lib.rs:11574-11584@ed012c7ccf)",
            "pipe_wall_endpoint_action_v2/wall_axial_end_action": "N, element_local, end_i = -Nw(0), end_j = +Nw(L); node-on-element (PP/src/lib.rs:11512-11517@ed012c7ccf)",
            "element_local_{shear_force_y,shear_force_z,torsional_moment,bending_moment_y,bending_moment_z}": "end_i/end_j: node-on-element end forces (PP/src/lib.rs:11100-11180@ed012c7ccf); quarter_1/midspan/quarter_3: section-cut action on the +x face of the i-side segment, i.e. the negated i-side helper sum (PP/src/lib.rs:10302-10331, 5077-5078@ed012c7ccf)",
            "global_nodal_displacement_{x,y,z}": "mm, global (PP/src/lib.rs:11182-11260@ed012c7ccf)",
            "global_nodal_rotation_{x,y,z}": "rad, global, right-hand",
            "support_reaction_component_v2/{Fx,Fy,Fz,Mx,My,Mz}": "N and N*m, global, location node, support-on-pipe; all six published for every support, unrestrained DOFs exactly 0 (PP/src/lib.rs:4806-4835, 11400-11422@ed012c7ccf)",
            "support_reaction_{force,moment}_magnitude_v2/{force,moment}_magnitude": "N and N*m, global, node: the norms of the force and moment components (PP/src/lib.rs:11423-11435@ed012c7ccf)",
            "element_local_{bending_normal_stress_y,bending_normal_stress_z,torsional_shear_stress}": "MPa (reference in Pa, exact factor 1/1000000), element_local, five locations: M/Z and T ro/J of the j-side section action, the ends included (PP/src/lib.rs:5179-5199, 5243-5345, 12555-12735; stress_recovery/src/lib.rs:431-460@ed012c7ccf)",
            "pipe_elastic_normal_stress_maximum_v2/maximum_absolute_normal_stress": "Pa, pipe_section, governing_station, one per member: max over the member of |Nw/As| + hypot(My,Mz)/Z, pressure-coupled through Nw today (PP/src/lib.rs:5399-5446, 10340-10420@ed012c7ccf); the arc maximum is T4-U4's",
            "stations": "end_i 0, quarter_1 0.25, midspan 0.5, quarter_3 0.75, end_j 1 of each member (PP/src/lib.rs:11518-11524@ed012c7ccf)",
            "absent_on_pressurized_members": "element_local_axial_force, element_local_axial_normal_stress, pipe_section_pressure_hoop_stress, pipe_section_pressure_longitudinal_stress (PP/src/lib.rs:11462-11471@ed012c7ccf)"},
        "section_rule": "effective wall = wall_thickness - mill_tolerance (absent slot = no reduction); ro = OD/2; ri = ro - effective wall; As = pi t (OD - t); Ai = pi ri^2 (reduced bore); I = As (ro^2+ri^2)/4; J = 2I; Z = I/ro (PP/src/lib.rs:10213-10230, 7244-7258; annulus_geometry.rs:34-45; pressure_exact/source_geometry.rs:28-46@ed012c7ccf)",
        "criteria": {"modes": ["sparse_interactive", "dense_scrutiny"], "same_reference_both_modes": True,
                     "nonzero": "|observed - expected| <= 1e-9 |expected|",
                     "zero": "|observed| <= 1e-9 * |zero_scale| converted into the row unit; each row's reference_origin.zero_scale is T1's {tag, base: {pointer, reference_unit}, scale_unit}, and criterion.absolute_tolerance is precomputed in the row unit",
                     "unit_convention": "T1's: every row's reference_origin is {kind: analytical, pointer, reference_unit, transform: identity}; the reference value is converted into the row unit by an exact definitional factor and the comparison is made in the row unit",
                     "unit_factors": {"m->mm": "1000 (displacement rows; in T1's UNIT_FACTORS)", "Pa->MPa": "1/1000000 (element_local bending and torsional stress rows, published in MPa; T1's UNIT_FACTORS needs this one exact factor added)"},
                     "zero_scale_definitions": "each load case's zero_scales entries are maintained quantities {unit, exact, decimal, value} only; the definitions are in the sibling map zero_scale_definitions",
                     "negative_assertions": "wrong_result_discriminators: |observed - wrong| > max(abs, 1e-9 max(|observed|,|wrong|))",
                     "no_new_threshold": "the existing protected 1e-9 relative criterion only"},
        "structural_expectations": ["status MECHANICS_SOLVED with no blocking or failure diagnostic, both modes",
                                    "no element_local_axial_force, element_local_axial_normal_stress, pipe_section_pressure_hoop_stress or pipe_section_pressure_longitudinal_stress row on a pressurized member",
                                    "every pressure row is listed in its region's contract_evidence.pressure[].result_ids exactly once",
                                    "pressure rows carry basis_ref {load_case, <case id>}; forces element_local, stresses pipe_section"],
        "sp1_pair_scope": SP1_PAIR_SCOPE,
        "cases": cases,
    }
    ok = all(c for _, c in CHECKS)
    with open(out_path, "w") as fh:
        json.dump(doc, fh, indent=1, ensure_ascii=True)
        fh.write("\n")
    rows = sum(len(lc["rows"]) for c in cases.values() for v in ([c] + list(c.get("variants", {}).values()))
               for lc in v.get("load_cases", {}).values())
    print(f"checks: {sum(1 for _, c in CHECKS if c)} passed, {sum(1 for _, c in CHECKS if not c)} failed")
    print(f"row assertions written: {rows}")
    for key, c in cases.items():
        print(f"{key}: {c['case_id']}")
    # headline values
    tp = cases["tp_phys_pressure_halves"]["load_cases"]
    for cid in tp:
        e = tp[cid]["expected"]
        print(f"  {cid}: Nw={e['Nw']['value']!r} S={e['S']['value']!r} sigma_z={e['sigma_z']['value']!r} A_Fx={e['A_Fx']['value']!r}")
    e = tp["case:combined"]["expected"]
    print(f"  combined: D_uy={e['D_uy']['value']!r} m D_rz={e['D_rz']['value']!r} rad A_Mz={e['A_Mz']['value']!r} A_Fy={e['A_Fy']['value']!r}")
    for key in ("free_transferring", "axially_restrained_transferring"):
        lc = list(cases["milltol_lame_membrane"]["variants"][key]["load_cases"].values())[0]["expected"]
        print(f"  milltol {key}: sigma_z={lc['sigma_z']['value']!r} hoop_in={lc['lame_inner_hoop']['value']!r} hoop_out={lc['lame_outer_hoop']['value']!r} Nw={lc['Nw']['value']!r}")
    lc = cases["pressure_membrane_thin_wall_limit"]["load_cases"]["case:membrane-free"]["expected"]
    print(f"  membrane: sigma_z={lc['sigma_z']['value']!r} hoop_in={lc['lame_inner_hoop']['value']!r} hoop_out={lc['lame_outer_hoop']['value']!r}")
    comp = cases["pressure_membrane_thin_wall_limit"]["thin_wall_comparison"]
    for k in [k for k in comp if k.startswith("relative")]:
        print(f"  {k} = {comp[k]['exact']['rational']} = {comp[k]['value']!r}")
    print("RESULT:", "PASS" if ok else "FAIL")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
