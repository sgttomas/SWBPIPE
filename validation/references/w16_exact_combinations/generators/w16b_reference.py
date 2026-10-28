#!/usr/bin/env python3
"""W-16 slice b independent references: load combinations on the exact route.

Standard-library Python 3 only. This generator imports, reads and runs no
product code and reads no file other than the ones it writes. Every expected
value is derived here in exact rational arithmetic (DEL-05-02 Design
`exact-combinations.md` section 8) for one invented two-member line:

  A(0) --m1-- B(L1) --m2-- C(L1 + L2), anchored at A, free at C, along global X.

Both members share one closed-ended section with a mill tolerance and a
corrosion allowance, so the D-5 bases differ: stiffness on the nominal wall t
(nominal area A_n), the pressure bore on the reduced wall t_r = (t - c_m) - c_a
(r_p = R - t_r, bore area A_i = pi r_p^2), and stresses on the nominal and the
reduced annulus. With P = p A_i, the closed forms of a closed-ended straight
member in a region of its own (caps transfer to the wall at both terminals) are

  N_w = N + P, S = N_w - P A_i / A_i = N  (S = N_w - p A_i),
  sigma_m,b = N_w / A_b,
  Lame on each basis b, C_b = p r_b^2 / (R^2 - r_b^2):
    sigma_theta = C_b (1 + R^2 / r^2), sigma_r = C_b (1 - R^2 / r^2),
  elongation = (N_w - 2 nu P) L / (E A_n),

with N the tension-positive mechanical axial force. Serial members add their
axial compliances. Three primitive cases:

  case:A  pressure p on m1 only;
  case:B  tip force F along X at C (both members), pressure p on m2 only;
  case:C  A's and B's loads together (both regions and F).

A member outside every region of an operand case takes its pressure-free
operand values (Design section 3): N_w = S = N, the end action is the
element end row, sigma_m,b = N / A_b, and the Lame values are zero. A
mechanics combination sum f_i X_i is formed with the represented binary64
factors f_i (Fraction(float)); a subtraction is X_minuend - X_subtrahend; a
range envelope selects per row over the operands by mode (ties to the
smallest case id). A combined magnitude is the norm of the combined
components.

Pi. Every value is q_1 pi + q_0 + q_-1 / pi with rational q_k (forces carry
pi through P; stresses divide by pi through A_b; motions are rational). Pi is
computed here by Machin's formula to about 95 digits, truncated to 90, and
used as the rational bounds [pi_90, pi_90 + 1e-90]; a binary64 value is the
nearest double to the exact value, decided by evaluating at both bounds
(refused if they disagree). Decimals print 40 significant digits.

Factor sets. The unit factor-sum rule of gate 3 (|sum f - 1| <= 2^-46, the
exact sum of the represented factors, compared exactly and inclusively) is
decided here in rationals for each factor set of the connector cases.

Modes:
  python3 -I w16b_reference.py           check: rebuild every output in memory and
                                          compare byte for byte with the committed
                                          files, SHA256SUMS included
  python3 -I w16b_reference.py --write   write the outputs (intentional changes only)
"""
import argparse
import hashlib
import json
import math
import struct
import sys
from decimal import Decimal, getcontext
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE.parent
FIXED_FILES = ("README.md", "generators/w16b_reference.py")

# ----------------------------------------------------------------------------- pi
getcontext().prec = 100


def _atan_inv(x):
    x = Decimal(x)
    total, term, n, sign = Decimal(0), 1 / x, 1, 1
    x2 = x * x
    eps = Decimal(10) ** -(getcontext().prec - 5)
    while True:
        delta = term / n
        if delta < eps:
            break
        total += sign * delta
        term /= x2
        n += 2
        sign = -sign
    return total


PI_DEC = 16 * _atan_inv(5) - 4 * _atan_inv(239)  # Machin, ~95 correct digits
_PI_90 = F(str(PI_DEC)[:92])  # "3." + 90 digits, truncated
PI_LO, PI_HI = _PI_90, _PI_90 + F(1, 10 ** 90)
assert PI_LO < F(314159265358979323846, 10 ** 20) + F(1, 10 ** 20) and PI_HI > F(314159265358979323846, 10 ** 20)


class Pi:
    """An exact value q_1 pi + q_0 + q_-1 / pi."""

    def __init__(self, terms=None):
        self.t = {k: F(v) for k, v in (terms or {}).items() if F(v) != 0}

    @staticmethod
    def of(q, k=0):
        return Pi({k: q})

    def __add__(self, other):
        other = other if isinstance(other, Pi) else Pi.of(other)
        out = dict(self.t)
        for k, v in other.t.items():
            out[k] = out.get(k, F(0)) + v
        return Pi(out)

    def __neg__(self):
        return Pi({k: -v for k, v in self.t.items()})

    def __sub__(self, other):
        return self + (-(other if isinstance(other, Pi) else Pi.of(other)))

    def scale(self, q):
        return Pi({k: v * F(q) for k, v in self.t.items()})

    def times_pi(self, k):
        return Pi({j + k: v for j, v in self.t.items()})

    def bounds(self):
        lo = hi = F(0)
        for k, v in self.t.items():
            if k == 0:
                a = b = v
            elif k == 1:
                a, b = v * PI_LO, v * PI_HI
            elif k == -1:
                a, b = v / PI_HI, v / PI_LO
            else:
                raise SystemExit(f"pi power {k}")
            lo, hi = lo + min(a, b), hi + max(a, b)
        return lo, hi

    def binary64(self):
        lo, hi = self.bounds()
        if float(lo) != float(hi):
            raise SystemExit(f"binary64 rounding of {self.text()} is not decided by the pi bounds")
        return float(lo) + 0.0

    def sign(self):
        lo, hi = self.bounds()
        if lo > 0:
            return 1
        if hi < 0:
            return -1
        if not self.t:
            return 0
        raise SystemExit(f"sign of {self.text()} is not decided by the pi bounds")

    def is_zero(self):
        return not self.t

    def text(self):
        if not self.t:
            return "0"
        parts = []
        for k in sorted(self.t, reverse=True):
            v = self.t[k]
            parts.append(str(v) if k == 0 else f"({v})*pi" if k == 1 else f"({v})/pi")
        return " + ".join(parts)

    def decimal(self):
        if not self.t:
            return "0"
        total = Decimal(0)
        for k, v in self.t.items():
            d = Decimal(v.numerator) / Decimal(v.denominator)
            total += d * PI_DEC if k == 1 else d / PI_DEC if k == -1 else d
        return total.normalize().__format__(".39e")

    def __eq__(self, other):
        other = other if isinstance(other, Pi) else Pi.of(other)
        return (self - other).is_zero()

    def __lt__(self, other):
        return (self - other).sign() < 0


def record(value):
    return {"exact": value.text(), "decimal": value.decimal(), "value": value.binary64()}


def hex_bits(x):
    return struct.pack(">d", x).hex()


# ----------------------------------------------------------------------------- inputs (invented)
L1, L2 = F(2), F(3)  # m, member lengths along global X
D = F(1, 5)  # m, outside diameter
R = D / 2
T = F(1, 100)  # m, nominal wall
C_M = F(1, 800)  # m, mill tolerance
C_A = F(1, 500)  # m, corrosion allowance
E = F(200 * 10 ** 9)  # Pa
NU = F(3, 10)
P_PA = F(2 * 10 ** 6)  # Pa, internal differential pressure
F_TIP = F(-25000)  # N, tip force along global X at C (compression)
STATIONS = ("end_i", "end_j", "quarter_1", "midspan", "quarter_3")
BASES = ("nominal_section", "reduced_section")

T_R = (T - C_M) - C_A
R_N = R - T  # nominal inner radius
R_R = R - T_R  # reduced inner radius = pressure bore
A_N = Pi.of(R * R - R_N * R_N, 1)  # nominal wall area
A_R = Pi.of(R * R - R_R * R_R, 1)  # reduced wall area
A_I = Pi.of(R_R * R_R, 1)  # pressure-bore area
P_CAP = A_I.scale(P_PA)  # P = p A_i
AREA = {"nominal_section": A_N, "reduced_section": A_R}
INNER = {"nominal_section": R_N, "reduced_section": R_R}


def area_q(basis):
    """A_b / pi."""
    return AREA[basis].t[1]


def lame(basis, p):
    """(inner radial, outer radial, inner hoop, outer hoop) on the basis annulus at pressure p."""
    r = INNER[basis]
    c = p * r * r / (R * R - r * r)
    return {
        "lame_inner_radial_stress": c * (1 - R * R / (r * r)),
        "lame_outer_radial_stress": c * (1 - R * R / (R * R)),
        "lame_inner_hoop_stress": c * (1 + R * R / (r * r)),
        "lame_outer_hoop_stress": c * (1 + R * R / (R * R)),
    }


# ----------------------------------------------------------------------------- primitive cases
CASES = {
    # case: (pressurized members, tip force)
    "case:A": ({"pipe:m1"}, F(0)),
    "case:B": ({"pipe:m2"}, F_TIP),
    "case:C": ({"pipe:m1", "pipe:m2"}, F_TIP),
}
MEMBERS = ("pipe:m1", "pipe:m2")
LENGTH = {"pipe:m1": L1, "pipe:m2": L2}


def member_state(case, member):
    """(N mechanical, pressurized) of a member: the statically determinate line.

    The tip force F passes through both members. A member's own region adds
    its cap thrust P to its wall only (the cap at its far terminal is reacted
    by the near terminal's cap through the wall, closed-ended); a region on
    the other member loads this member with that region's caps, which cancel
    across the shared node B, so N is the tip force."""
    regions, force = CASES[case]
    return Pi.of(force), member in regions


def member_rows(case):
    """Rows of one case: closed forms where the member is pressurized, else the
    pressure-free operand values (N_w = S = N, Lame zero)."""
    rows = []
    for member in MEMBERS:
        n, pressurized = member_state(case, member)
        wall = n + P_CAP if pressurized else n
        effective = n
        for station in STATIONS:
            rows.append(("pipe_wall_axial_force_v2", "wall_axial_force", member, station, None, "N", wall))
            rows.append(("pipe_effective_axial_force_v2", "effective_axial_force", member, station, None, "N", effective))
            for basis in BASES:
                membrane = Pi.of(wall.t.get(0, F(0)) / area_q(basis), -1) + Pi.of(wall.t.get(1, F(0)) / area_q(basis), 0)
                rows.append(("pipe_axial_membrane_stress_v2", "axial_membrane_stress", member, station, basis, "Pa", membrane))
                for component, value in lame(basis, P_PA if pressurized else F(0)).items():
                    kind = "pipe_lame_radial_stress_v2" if "radial" in component else "pipe_lame_hoop_stress_v2"
                    rows.append((kind, component, member, station, basis, "Pa", Pi.of(value)))
        rows.append(("pipe_wall_endpoint_action_v2", "wall_axial_end_action", member, "end_i", None, "N", -wall))
        rows.append(("pipe_wall_endpoint_action_v2", "wall_axial_end_action", member, "end_j", None, "N", wall))
        # The global end actions of a member along X: Fx = -N_w at end_i, +N_w at end_j.
        rows.append(("element_global_end_force_v1", "Fx", member, "end_i", None, "N", -wall))
        rows.append(("element_global_end_force_v1", "Fx", member, "end_j", None, "N", wall))
    _, force = CASES[case]
    rows.append(("support_reaction_component_v2", "Fx", "support:a", "node", None, "N", Pi.of(-force)))
    # Elongations: (N_w - 2 nu P) L / (E A_n) on pressurized members, N L / (E A_n) otherwise.
    u = Pi()
    positions = {"node:a": Pi()}
    for member, node in (("pipe:m1", "node:b"), ("pipe:m2", "node:c")):
        n, pressurized = member_state(case, member)
        axial = n + P_CAP - P_CAP.scale(2 * NU) if pressurized else n
        # axial / (E A_n) * L: forces carry pi^1 and pi^0; A_n = a_n pi.
        a_n = A_N.t[1]
        u = u + Pi.of(axial.t.get(1, F(0)) * LENGTH[member] / (E * a_n), 0) + Pi.of(axial.t.get(0, F(0)) * LENGTH[member] / (E * a_n), -1)
        positions[node] = u
    for node in ("node:b", "node:c"):
        rows.append(("global_nodal_displacement_x", "nodal_displacement_x", node, "node", None, "m", positions[node]))
    return rows


def keyed(rows):
    return {(kind, component, entity, location, basis): (unit, value) for kind, component, entity, location, basis, unit, value in rows}


PRIMITIVE = {case: keyed(member_rows(case)) for case in CASES}

# ----------------------------------------------------------------------------- combinations
COMBINATIONS = [
    {"id": "comb:K1", "basis": "mechanics", "terms": [("case:A", 1.0), ("case:B", 1.3)],
     "note": "A + 1.3B"},
    {"id": "comb:K2", "basis": "mechanics", "terms": [("case:A", -0.75), ("case:B", 2.5)],
     "note": "a negative factor"},
    {"id": "comb:K3", "basis": "mechanics", "terms": [("case:A", 0.0), ("case:B", 1.0)],
     "note": "a zero factor"},
    {"id": "comb:K4", "basis": "mechanics", "terms": [("case:C", 1.0), ("case:A", -1.0), ("case:B", -1.0)],
     "note": "cancelling factors: C - A - B is exactly zero"},
    {"id": "comb:S1", "basis": "result_state_subtraction", "minuend": "case:C", "subtrahend": "case:A",
     "note": "subtraction C - A"},
] + [
    {"id": f"comb:E-{mode}", "basis": "range_envelope", "operands": ["case:C", "case:A", "case:B"], "mode": mode,
     "note": f"range envelope over A, B and C, mode {mode}"}
    for mode in ("min", "max", "min_abs", "max_abs")
]


def operands_of(comb):
    if comb["basis"] == "mechanics":
        return [c for c, _ in comb["terms"]]
    if comb["basis"] == "result_state_subtraction":
        return [comb["minuend"], comb["subtrahend"]]
    return sorted(comb["operands"])


def evaluate(comb, values):
    if comb["basis"] == "mechanics":
        total = Pi()
        for (case, factor), value in zip(comb["terms"], values):
            total = total + value.scale(F(factor))  # the represented binary64 factor, exactly
        return total
    if comb["basis"] == "result_state_subtraction":
        return values[0] - values[1]
    mode = comb["mode"]
    best = values[0]
    for value in values[1:]:
        if mode == "min":
            better = value < best
        elif mode == "max":
            better = best < value
        elif mode == "min_abs":
            better = (value if value.sign() >= 0 else -value) < (best if best.sign() >= 0 else -best)
        else:
            better = (best if best.sign() >= 0 else -best) < (value if value.sign() >= 0 else -value)
        if better:
            best = value
    return best


def family(unit):
    return {"N": "force", "Pa": "stress", "m": "translation"}[unit]


def combination_reference(comb):
    operands = operands_of(comb)
    rows = []
    for key in PRIMITIVE[operands[0]]:
        unit = PRIMITIVE[operands[0]][key][0]
        values = [PRIMITIVE[case][key][1] for case in operands]
        value = evaluate(comb, values)
        kind, component, entity, location, basis = key
        rows.append({"kind": kind, "component": component, "entity": entity, "location": location,
                     "section_basis": basis, "unit": unit, **record(value)})
    # The tip's displacement magnitude: the norm of the combined components (y, z are zero),
    # or the selected operand magnitude in a range envelope.
    tip = [PRIMITIVE[case][("global_nodal_displacement_x", "nodal_displacement_x", "node:c", "node", None)][1] for case in operands]
    if comb["basis"] == "range_envelope":
        magnitudes = [v if v.sign() >= 0 else -v for v in tip]
        magnitude = evaluate(comb, magnitudes)
    else:
        combined = evaluate(comb, tip)
        magnitude = combined if combined.sign() >= 0 else -combined
    rows.append({"kind": "displacement_magnitude", "component": None, "entity": "node:c", "location": None,
                 "section_basis": None, "unit": "m", **record(magnitude)})
    scales = {}
    for case in operands:
        for (kind, component, entity, location, basis), (unit, value) in PRIMITIVE[case].items():
            fam = family(unit)
            v = abs(value.binary64())
            scales[fam] = max(scales.get(fam, 0.0), v)
    # Structure (Design section 3): a member pressurized in some operands but not all has its
    # pressure-family rows recovered in each other operand (62 witnesses each: 2 end actions,
    # 5 wall and 5 effective forces, 10 membranes, 20 Lame radial and 20 Lame hoop values) and
    # its element_local_axial_force (5 stations) and element_local_axial_normal_stress
    # (5 stations x 2 bases) rows missing in the operands that pressurize it (15 missing rows).
    witnesses = missing = 0
    for member in MEMBERS:
        pressurized = [member_state(case, member)[1] for case in operands]
        if any(pressurized) and not all(pressurized):
            witnesses += 62 * pressurized.count(False)
            missing += 15
    out = {"id": comb["id"], "basis": comb["basis"], "note": comb["note"], "operand_order": operands}
    if comb["basis"] == "mechanics":
        out["terms"] = [{"load_case": c, "factor_binary64_hex": hex_bits(f), "factor": f} for c, f in comb["terms"]]
    elif comb["basis"] == "result_state_subtraction":
        out["minuend"], out["subtrahend"] = comb["minuend"], comb["subtrahend"]
    else:
        out["operand_ids"], out["mode"] = comb["operands"], comb["mode"]
    out["expected_recovery_witnesses"] = witnesses
    out["expected_missing_sources"] = missing
    out["zero_scales"] = {k: scales[k] for k in sorted(scales)}
    out["rows"] = rows
    return out


# ----------------------------------------------------------------------------- factor sets
P46 = 2.0 ** -46
FACTOR_SETS = [
    ("unit", [1.0]),
    ("tenths", [0.3, 0.7]),
    ("below_ulp", [1.0, 2.0 ** -54]),
    ("upper_boundary", [1.0, P46]),
    ("lower_boundary", [1.0, -P46]),
    ("two", [2.0]),
    ("zero", [0.0]),
    ("cancelling_2p53", [2.0 ** 53, -(2.0 ** 53)]),
    ("just_above", [1.0, P46, 2.0 ** -100]),
    ("just_below", [1.0, -P46, -(2.0 ** -100)]),
]


def factor_set(name, factors):
    total = sum((F(f) for f in factors), F(0))
    offset = total - 1
    return {"id": name, "factors": factors, "factors_binary64_hex": [hex_bits(f) for f in factors],
            "exact_sum_minus_one": str(offset), "bound": str(F(P46)),
            "unit_sum_admitted": abs(offset) <= F(P46),
            "rounded_sum_minus_one": str(F(math.fsum(factors)) - 1)}


# ----------------------------------------------------------------------------- cross-checks
def crosschecks():
    out = []

    def check(name, ok, detail):
        out.append({"check": name, "agrees": bool(ok), "detail": detail})

    for case in CASES:
        rows = PRIMITIVE[case]
        _, force = CASES[case]
        # Equilibrium: the support reaction balances the applied tip force (caps cancel).
        reaction = rows[("support_reaction_component_v2", "Fx", "support:a", "node", None)][1]
        check(f"{case} reaction balances the applied load", reaction + Pi.of(force) == Pi(), reaction.text())
        for member in MEMBERS:
            wall = rows[("pipe_wall_axial_force_v2", "wall_axial_force", member, "midspan", None)][1]
            eff = rows[("pipe_effective_axial_force_v2", "effective_axial_force", member, "midspan", None)][1]
            pressurized = member_state(case, member)[1]
            # S = N_w - p A_i where pressurized, S = N_w otherwise.
            check(f"{case} {member} S = N_w - p A_i", eff == wall - (P_CAP if pressurized else Pi()), (wall - eff).text())
            for basis in BASES:
                mem = rows[("pipe_axial_membrane_stress_v2", "axial_membrane_stress", member, "midspan", basis)][1]
                # sigma_m A_b = N_w, by multiplication rather than the division used above.
                check(f"{case} {member} {basis} sigma_m A_b = N_w", mem.times_pi(1).scale(area_q(basis)) == wall, mem.text())
                inner_r = rows[("pipe_lame_radial_stress_v2", "lame_inner_radial_stress", member, "midspan", basis)][1]
                outer_h = rows[("pipe_lame_hoop_stress_v2", "lame_outer_hoop_stress", member, "midspan", basis)][1]
                inner_h = rows[("pipe_lame_hoop_stress_v2", "lame_inner_hoop_stress", member, "midspan", basis)][1]
                p = P_PA if pressurized else F(0)
                # Lame: the inner radial stress is -p; the hoop difference is C (R^2/r^2 - 1) = p.
                check(f"{case} {member} {basis} Lame inner radial = -p", inner_r == Pi.of(-p), inner_r.text())
                check(f"{case} {member} {basis} Lame hoop difference = p", inner_h - outer_h == Pi.of(p), (inner_h - outer_h).text())
                # The hoop resultant over the wall balances p times the bore diameter (Barlow, thick wall).
                r = INNER[basis]
                c = p * r * r / (R * R - r * r)
                resultant = c * (R - r) + c * R * R * (1 / r - 1 / R)
                check(f"{case} {member} {basis} hoop resultant = p r_b", resultant == p * r, str(resultant))
    # The elongation of a pressurized member by the integrated axial strain of its wall,
    # (N_w - 2 nu P) / (E A_n), equals the product's eigenload form N_m L / (E A_n) with
    # N_m = N_w - 2 nu p A_i (Design section 2's straight recovery N_w = N_m + 2 nu p A_i).
    for case in CASES:
        u = PRIMITIVE[case][("global_nodal_displacement_x", "nodal_displacement_x", "node:c", "node", None)][1]
        total = Pi()
        for member in MEMBERS:
            wall = PRIMITIVE[case][("pipe_wall_axial_force_v2", "wall_axial_force", member, "midspan", None)][1]
            pressurized = member_state(case, member)[1]
            nm = wall - P_CAP.scale(2 * NU) if pressurized else wall
            total = total + Pi.of(nm.t.get(1, F(0)) * LENGTH[member] / (E * A_N.t[1]), 0) + Pi.of(nm.t.get(0, F(0)) * LENGTH[member] / (E * A_N.t[1]), -1)
        check(f"{case} tip displacement = sum of member elongations", u == total, u.text())
    # Superposition: C = A + B row by row (linearity in the resolved loads, Design section 2),
    # with the pressure-free operand values standing in for the unpressurized members.
    differing = [key for key in PRIMITIVE["case:A"]
                 if not (PRIMITIVE["case:A"][key][1] + PRIMITIVE["case:B"][key][1] == PRIMITIVE["case:C"][key][1])]
    check("superposition C = A + B on every row", not differing, f"{len(PRIMITIVE['case:A'])} rows; differing {differing[:3]}")
    return out


# ----------------------------------------------------------------------------- output
def inputs():
    return {
        "geometry": {"L1_m": str(L1), "L2_m": str(L2), "axis": "global X; A(0) anchored, C(L1 + L2) free"},
        "section": {"outside_diameter_m": str(D), "wall_thickness_m": str(T), "mill_tolerance_m": str(C_M),
                    "corrosion_allowance_m": str(C_A)},
        "material": {"elastic_modulus_Pa": str(E), "poisson_ratio": str(NU)},
        "loads": {"pressure_Pa": str(P_PA), "tip_force_x_N": str(F_TIP)},
        "cases": {case: {"pressurized_members": sorted(regions), "tip_force_x_N": str(force)} for case, (regions, force) in CASES.items()},
    }


def derived():
    return {
        "reduced_wall_m": str(T_R),
        "nominal_inner_radius_m": str(R_N),
        "pressure_bore_radius_m": str(R_R),
        "nominal_area_m2": record(A_N),
        "reduced_area_m2": record(A_R),
        "pressure_bore_area_m2": record(A_I),
        "cap_thrust_P_N": record(P_CAP),
    }


def primitive_cases():
    out = []
    for case in CASES:
        rows = []
        for (kind, component, entity, location, basis), (unit, value) in PRIMITIVE[case].items():
            rows.append({"kind": kind, "component": component, "entity": entity, "location": location,
                         "section_basis": basis, "unit": unit, **record(value)})
        out.append({"id": case, "rows": rows})
    return out


def references():
    return {
        "reference": "W-16 slice b: load combinations on the exact pressure route (DEL-05-02 Design exact-combinations.md section 8)",
        "generator": "validation/references/w16_exact_combinations/generators/w16b_reference.py",
        "units": {"N": "newton", "Pa": "pascal", "m": "metre (the product publishes displacements in mm)"},
        "criterion": "|obs - exp| <= 1e-9 |exp|; an exact zero uses 1e-9 times the combination's zero scale of its family",
        "inputs": inputs(),
        "derived": derived(),
        "cases": primitive_cases(),
        "combinations": [combination_reference(c) for c in COMBINATIONS],
        "factor_sets": [factor_set(n, f) for n, f in FACTOR_SETS],
        "crosscheck": crosschecks(),
    }


def render():
    text = json.dumps(references(), indent=1, ensure_ascii=True) + "\n"
    files = {"references.json": text.encode("ascii")}
    for name in FIXED_FILES:
        files[name] = (OUT / name).read_bytes()
    sums = "".join(f"{hashlib.sha256(files[name]).hexdigest()}  {name}\n" for name in sorted(files))
    files["SHA256SUMS"] = sums.encode("ascii")
    return files


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    files = render()
    if args.write:
        for name, data in files.items():
            if name not in FIXED_FILES:
                (OUT / name).write_bytes(data)
        print("written")
        return 0
    stale = [name for name, data in files.items() if not (OUT / name).exists() or (OUT / name).read_bytes() != data]
    if stale:
        print("stale: " + ", ".join(sorted(stale)))
        return 1
    print("current")
    return 0


if __name__ == "__main__":
    sys.exit(main())
