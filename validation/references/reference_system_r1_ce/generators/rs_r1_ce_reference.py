#!/usr/bin/env python3
"""Reference system with RS-R1 and RS-CE1 adopted: EQ-1, EQ-2 and EQ-5 bookkeeping (DEL-09-01).

Standard-library Python 3 only. This generator imports, reads and runs no product code and reads no fixture
model file and no product output. Its inputs are the reference-system specification's own statements
(`fixtures/reference_system/SPECIFICATION.md` §3.1, §3.2, §4.2, §5 and §6), transcribed below. It reads two
other files, both frozen independent references, only to compare quantities it recomputes here with the same
quantities recorded there (consistency controls; nothing is taken from them):

  ../reference_system/references.json                tranche 1: RS-P150 a plain S270 member, no RS-R1
  ../reference_system_pending_cases/references.json  its RS-SUS "full system (RS-R1 in place)" context value

What it derives, every value an exact sum of q * pi^k * sqrt(n) terms with q rational:

  total weight     W = sum over weighted members of mu * g * length, plus each rigid-family component's weight
  first moment     S = sum of W_i * r_i (members at their centroids; components at node + centre of gravity)
  weight moment    sum of r x (0, 0, -W_i) = (-S_y, S_x, 0); EQ-2's reaction moment resultant is its negative
  RS-R1            its 300 N at the body midpoint replaces RS-P150's distributed weight (spec §5; W-15 §3);
                   its weight wrench at RS-160 is F = (0, 0, -300) N, M = r x F with r the entered CoG vector
  RS-CE1           its 9000 N constant force acts on the pipe along +Z at RS-100 in every case (spec §6;
                   DEC-049): one support action, counted once among all supports in EQ-1 and EQ-2; it adds no
                   weight, so the vertical reaction sum is still W and the other supports carry W - 9000 N
  EQ-5             no load differs between RS-SAM and RS-SUS, RS-CE1's force included: RS-dSAM is
                   self-equilibrated

Mass bases (EX's stated ones; the frozen tranche-1 package's README, "Mass bases"):

  pipe metal       rho_m * pi * (R^2 - (R - t)^2)                  nominal wall
  contents         rho_c * pi * (R - ((t - c_m) - c_a))^2          reduced (pressure) bore
  insulation       rho_i * pi * ((R + t_i)^2 - R^2)                OD to OD + 2 t_i
  realized arc     length R * pi/2; first moment R * [(pi/2) C + e1 + e2], C the arc centre, e1, e2 the
                   radius vectors to its tangent nodes
  replaced span    RS-P180 carries no weight (spec §5.1)

pi is bounded by Machin's formula in integer arithmetic and sqrt(n) by the integer square root, each to 100
decimal places; a printed decimal (40 significant digits) and a binary64 value are emitted only when both ends of
the bounding interval give the same one.

Modes:
  python3 -I rs_r1_ce_reference.py           check: rebuild every output in memory and compare byte for byte
                                              with the committed files, SHA256SUMS included
  python3 -I rs_r1_ce_reference.py --write   write references.json and SHA256SUMS (intentional changes only)
"""
import argparse
import hashlib
import json
import re
import sys
from decimal import Decimal, getcontext
from fractions import Fraction as F
from math import isqrt
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE.parent
REFS = OUT.parent
TRANCHE1 = REFS / "reference_system" / "references.json"
TRANCHE1_REL = "validation/references/reference_system/references.json"
PENDING = REFS / "reference_system_pending_cases" / "references.json"
PENDING_REL = "validation/references/reference_system_pending_cases/references.json"
WRITTEN = ("references.json", "SHA256SUMS")

# ============================================================================ controlled evaluation of pi and sqrt
DIGITS = 100
_GUARD = 10
_SCALE = 10 ** (DIGITS + _GUARD)


def _arctan_inv(x):
    """(floor-truncated sum, number of terms) of the series for arctan(1/x) * _SCALE."""
    total, n, sign, power = 0, 1, 1, _SCALE // x
    terms = 0
    x2 = x * x
    while power:
        total += sign * (power // n)
        terms += 1
        power //= x2
        n += 2
        sign = -sign
    return total, terms


def _pi_bounds():
    a5, n5 = _arctan_inv(5)
    a239, n239 = _arctan_inv(239)
    scaled = 16 * a5 - 4 * a239
    # Each term carries two floor truncations (power, then the division) of less than 1 each, and the first
    # omitted term is below 1: per series an error below 2 * terms + 1 units, times its multiplier.
    err = 16 * (2 * n5 + 1) + 4 * (2 * n239 + 1)
    return F(scaled - err, _SCALE), F(scaled + err, _SCALE)


PI_LO, PI_HI = _pi_bounds()
assert PI_LO < F(314159265358979323847, 10 ** 20) and PI_HI > F(314159265358979323846, 10 ** 20)
assert PI_HI - PI_LO < F(1, 10 ** DIGITS)


def sqrt_bounds(n):
    s = isqrt(n * 10 ** (2 * DIGITS))
    assert s * s <= n * 10 ** (2 * DIGITS) < (s + 1) ** 2
    return F(s, 10 ** DIGITS), F(s + 1, 10 ** DIGITS)


def squarefree(q):
    """sqrt(q) = c * sqrt(n), c rational, n a squarefree positive integer (q a positive rational)."""
    num, den = q.numerator * q.denominator, q.denominator
    c, n, p = 1, num, 2
    while p * p <= n:
        while n % (p * p) == 0:
            n //= p * p
            c *= p
        p += 1
    return F(c, den), n


class X:
    """An exact sum of q * pi^k * sqrt(n), keyed by (k, n)."""

    def __init__(self, terms=None):
        self.t = {key: F(q) for key, q in (terms or {}).items() if q != 0}

    @staticmethod
    def q(value, k=0, n=1):
        return X({(k, n): F(value)})

    def __add__(self, other):
        terms = dict(self.t)
        for key, value in other.t.items():
            terms[key] = terms.get(key, F(0)) + value
        return X(terms)

    def __neg__(self):
        return X({key: -v for key, v in self.t.items()})

    def __sub__(self, other):
        return self + (-other)

    def __mul__(self, other):
        """Products whose sqrt factors combine to one squarefree factor (here at most one is not 1)."""
        terms = {}
        for (k1, n1), a in self.t.items():
            for (k2, n2), b in other.t.items():
                assert n1 == 1 or n2 == 1, "a product of two surds does not arise here"
                key = (k1 + k2, n1 * n2)
                terms[key] = terms.get(key, F(0)) + a * b
        return X(terms)

    def scale(self, value):
        return X({key: F(value) * v for key, v in self.t.items()})

    def __eq__(self, other):
        return self.t == other.t

    def text(self):
        if not self.t:
            return "0"
        parts = []
        for (k, n), value in sorted(self.t.items()):
            factor = (f"*pi^{k}" if k > 1 else "*pi" if k == 1 else "") + (f"*sqrt({n})" if n != 1 else "")
            parts.append(f"({value}){factor}")
        return " + ".join(parts)

    def interval(self):
        lo = hi = F(0)
        for (k, n), value in self.t.items():
            slo, shi = sqrt_bounds(n) if n != 1 else (F(1), F(1))
            a, b = value * PI_LO ** k * slo, value * PI_HI ** k * shi
            lo, hi = lo + min(a, b), hi + max(a, b)
        return lo, hi

    def record(self):
        if not self.t:
            return {"exact": "0", "decimal": "0", "value": 0.0}
        lo, hi = self.interval()
        getcontext().prec = 60
        dlo = format(Decimal(lo.numerator) / Decimal(lo.denominator), ".39e")
        dhi = format(Decimal(hi.numerator) / Decimal(hi.denominator), ".39e")
        if dlo != dhi or float(lo) != float(hi):
            raise SystemExit(f"the bounds of {self.text()} do not decide its printed values")
        return {"exact": self.text(), "decimal": dlo, "value": float(lo)}


TERM = re.compile(r"\((-?\d+(?:/\d+)?)\)(?:\*pi(?:\^(\d+))?)?(?:\*sqrt\((\d+)\))?")


def parse_exact(text):
    """The X a frozen record's `exact` text states (the format both frozen packages print)."""
    if text == "0":
        return X()
    out = X()
    for part in text.split(" + "):
        m = TERM.fullmatch(part)
        assert m, part
        k = 0 if "*pi" not in part else int(m.group(2) or 1)
        out = out + X.q(F(m.group(1)), k, int(m.group(3) or 1))
    return out


def vrec(vector):
    return [x.record() for x in vector]


# ============================================================================ the specification's statements
G = F("9.80665")  # m/s^2, spec §2: gravity entered on each weight case as global_z -9.80665
NODES = {  # §3.1, m
    "RS-010": ("0", "0", "1.000"), "RS-020": ("0.400", "0", "1.000"), "RS-030": ("0.550", "0", "1.000"),
    "RS-040": ("1.150", "0", "1.000"), "RS-050": ("1.300", "0", "1.000"), "RS-060": ("3.500", "0", "1.000"),
    "RS-070": ("5.600", "0", "1.000"), "RS-075": ("6.000", "0", "1.400"), "RS-080": ("6.000", "0", "4.000"),
    "RS-090": ("6.000", "0", "6.600"), "RS-095": ("6.000", "0.400", "7.000"), "RS-100": ("6.000", "3.000", "7.000"),
    "RS-110": ("6.000", "7.000", "7.000"), "RS-120": ("6.000", "11.000", "7.000"),
    "RS-130": ("6.000", "15.000", "7.000"), "RS-140": ("6.000", "18.000", "7.000"),
    "RS-150": ("6.000", "20.000", "7.000"), "RS-160": ("6.000", "20.400", "6.975"),
    "RS-170": ("6.000", "23.000", "6.975"), "RS-180": ("6.000", "25.000", "6.975"),
    "RS-190": ("6.000", "25.600", "6.975"), "RS-200": ("6.000", "28.000", "6.975"),
    "RS-210": ("6.000", "30.000", "6.975"), "RS-215": ("6.000", "15.000", "4.500"),
    "RS-220": ("6.000", "15.000", "2.250"), "RS-225": ("6.250", "15.000", "2.000"),
    "RS-230": ("8.000", "15.000", "2.000"),
}
MEMBERS = [  # §3.2: member, from, to, section
    ("RS-P010", "RS-010", "RS-020", "S270"), ("RS-P020", "RS-020", "RS-030", "S270"),
    ("RS-P030", "RS-030", "RS-040", "S270"), ("RS-P040", "RS-040", "RS-050", "S270"),
    ("RS-P050", "RS-050", "RS-060", "S270"), ("RS-P060", "RS-060", "RS-070", "S270"),
    ("RS-P065", "RS-070", "RS-075", "S270"), ("RS-P070", "RS-075", "RS-080", "S270"),
    ("RS-P080", "RS-080", "RS-090", "S270"), ("RS-P085", "RS-090", "RS-095", "S270"),
    ("RS-P090", "RS-095", "RS-100", "S270"), ("RS-P100", "RS-100", "RS-110", "S270"),
    ("RS-P110", "RS-110", "RS-120", "S270"), ("RS-P120", "RS-120", "RS-130", "S270"),
    ("RS-P130", "RS-130", "RS-140", "S270"), ("RS-P140", "RS-140", "RS-150", "S270"),
    ("RS-P150", "RS-150", "RS-160", "S270"), ("RS-P160", "RS-160", "RS-170", "S220"),
    ("RS-P170", "RS-170", "RS-180", "S220"), ("RS-P180", "RS-180", "RS-190", "S220"),
    ("RS-P190", "RS-190", "RS-200", "S220"), ("RS-P200", "RS-200", "RS-210", "S220"),
    ("RS-P210", "RS-130", "RS-215", "S170"), ("RS-P220", "RS-215", "RS-220", "S170"),
    ("RS-P225", "RS-220", "RS-225", "S170"), ("RS-P230", "RS-225", "RS-230", "S170"),
]
# §4.2: OD, wall, mill tolerance, corrosion allowance, insulation thickness (mm); insulation, contents (kg/m^3)
SECTIONS = {
    "S270": ("270", "9.0", "1.0", "1.5", "60", "140", "820"),
    "S220": ("220", "8.0", "0.9", "1.5", "60", "140", "820"),
    "S170": ("170", "7.0", "0.8", "1.5", "50", "140", "820"),
}
RHO_M = F(7850)  # §4.1, §4.2
ARCS = {"RS-P065": ("RS-B1", ("6", "0", "1"), "0.400"), "RS-P085": ("RS-B2", ("6", "0", "7"), "0.400"),
        "RS-P225": ("RS-B3", ("6", "15", "2"), "0.250")}  # §3.1 corners, §5 radii, 90 degrees
# §5 rigid family: component, mapped member, node, weight (N), CoG offset from the body midpoint (m, global)
RIGID = {
    "RS-F1": ("RS-P020", "RS-030", "350", ("0", "0", "0")),
    "RS-V1": ("RS-P030", "RS-040", "2600", ("0", "0", "0.30")),
    "RS-F2": ("RS-P040", "RS-050", "350", ("0", "0", "0")),
    "RS-R1": ("RS-P150", "RS-160", "300", ("0", "0", "0")),
}
# §5 RS-R1: end sizes (mm), eccentric_offset (mm, from the centreline at RS-160 to the one at RS-150), scale.
R1_ENDS_MM = (F(270), F(220))
R1_OFFSET_MM = (F(0), F(0), F(25))
R1_SCALE = F(2)
REPLACED_SPAN = "RS-P180"  # §5.1
# §6 RS-CE1: node, its one translational restraint, the entered constant load (N), acting along +UZ.
CE1 = {"support": "support:RS-CE1", "node": "RS-100", "dof": "UZ", "constant_load_N": F(9000),
       "direction": (F(0), F(0), F(1))}
CASES = ("load:RS-SUS", "load:RS-OPE", "load:RS-SAM")
SYSTEMS = {"tranche_1": ("RS-F1", "RS-V1", "RS-F2"), "adopted": ("RS-F1", "RS-V1", "RS-F2", "RS-R1")}

CROSS = []


def cross_check(label, agrees):
    CROSS.append({"check": label, "agrees": bool(agrees)})


# ============================================================================ geometry and mass
def P(node):
    return tuple(F(v) for v in NODES[node])


def vsub(a, b):
    return tuple(x - y for x, y in zip(a, b))


def vadd(a, b):
    return tuple(x + y for x, y in zip(a, b))


def vmul(q, a):
    return tuple(q * x for x in a)


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def cross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


MEMBER = {m[0]: m for m in MEMBERS}


def section_mass(sid):
    """Mass per length over pi (kg/m), with a second factored form."""
    od, wall, mill, ca, ti, rho_i, rho_c = (F(v) for v in SECTIONS[sid])
    d, t, cm, ca, ti = od / 1000, wall / 1000, mill / 1000, ca / 1000, ti / 1000
    r = d / 2
    parts = {"metal": RHO_M * (r * r - (r - t) ** 2), "contents": rho_c * (r - ((t - cm) - ca)) ** 2,
             "insulation": rho_i * ((r + ti) ** 2 - r * r)}
    alt = RHO_M * t * (d - t) + rho_c * (d / 2 - t + cm + ca) ** 2 + rho_i * ti * (d + ti)
    return parts, sum(parts.values()), alt


def member_weight(pid):
    """(weight, first moment vector, length text, kind) of a weighted member, as X values."""
    _, a_id, b_id, sid = MEMBER[pid]
    a, b = P(a_id), P(b_id)
    q = section_mass(sid)[1] * G  # N/m over pi
    if pid in ARCS:
        _, corner, radius = ARCS[pid]
        c, R = tuple(F(v) for v in corner), F(radius)
        centre = vsub(vadd(a, b), c)
        e1, e2 = vsub(a, centre), vsub(b, centre)
        weight = X.q(q * R / 2, 2)  # q * pi times R * pi / 2
        moment = [X.q(q * R * centre[i] / 2, 2) + X.q(q * R * (e1[i] + e2[i]), 1) for i in range(3)]
        return weight, moment, f"({R})*pi/2", "arc"
    l2 = dot(vsub(b, a), vsub(b, a))
    c, n = squarefree(l2)
    weight = X.q(q * c, 1, n)
    mid = vmul(F(1, 2), vadd(a, b))
    return weight, [weight.scale(mid[i]) for i in range(3)], str(c) if n == 1 else f"({c})*sqrt({n})", "straight"


def component_position(cid):
    pipe, node, _, offset = RIGID[cid]
    _, a, b, _ = MEMBER[pipe]
    midpoint = vmul(F(1, 2), vadd(P(a), P(b)))
    return vadd(midpoint, tuple(F(v) for v in offset)), node


def system_weight(system):
    """(W, S, member rows, component rows) of one system."""
    components = SYSTEMS[system]
    mapped = {RIGID[c][0]: c for c in components}
    total, first, members, comps = X(), [X(), X(), X()], [], []
    for pid, *_ in MEMBERS:
        if pid == REPLACED_SPAN:
            members.append({"member": pid, "carries": "no distributed weight",
                            "reason": "replaced span of RS-EJ1 (spec §5.1)"})
            continue
        if pid in mapped:
            members.append({"member": pid, "carries": "no distributed weight", "by": f"component:{mapped[pid]}",
                            "reason": "rigid-family component weight replaces the member's distributed weight"})
            continue
        weight, moment, length_text, kind = member_weight(pid)
        total = total + weight
        first = [first[i] + moment[i] for i in range(3)]
        members.append({"member": pid, "section": f"section:RS-{MEMBER[pid][3]}", "kind": kind,
                        "length_m": length_text, "weight_N": weight.record(), "first_moment_N_m": vrec(moment)})
    for cid in components:
        position, node = component_position(cid)
        w = F(RIGID[cid][2])
        weight = X.q(w)
        total = total + weight
        moment = [X.q(w * position[i]) for i in range(3)]
        first = [first[i] + moment[i] for i in range(3)]
        cog = vsub(position, P(node))
        force = (F(0), F(0), -w)
        wrench = cross(cog, force)
        comps.append({"component": f"component:{cid}", "node": f"node:{node}",
                      "replaces_member": f"pipe:{RIGID[cid][0]}", "weight_N": weight.record(),
                      "position_m": [str(v) for v in position],
                      "center_of_gravity_from_node_m": [str(v) for v in cog],
                      "weight_wrench_at_node": {"force_N": [str(v) for v in force],
                                                "moment_N_m": [str(v) for v in wrench]},
                      "first_moment_N_m": vrec(moment)})
    return total, first, members, comps


def weight_moment(first):
    """sum of r x (0, 0, -W_i) = (-S_y, S_x, 0)."""
    return [-first[1], first[0], X()]


# ============================================================================ the derivation
def build():
    CROSS.clear()
    # Sections.
    section_rows = []
    for sid in SECTIONS:
        parts, total, alt = section_mass(sid)
        cross_check(f"RS-{sid}: mass per length over pi in a second factored form", total == alt)
        section_rows.append({"section": f"section:RS-{sid}", "mass_per_length_kg_m": {
            **{k: X.q(v, 1).record() for k, v in parts.items()}, "total": X.q(total, 1).record()},
            "weight_per_length_N_m": X.q(total * G, 1).record()})

    # The realized bends (§3.1 corners, §5 radii): 90-degree arcs between their tangent nodes.
    for pid, (bend, corner, radius) in ARCS.items():
        a, b = P(MEMBER[pid][1]), P(MEMBER[pid][2])
        c, R = tuple(F(v) for v in corner), F(radius)
        centre = vsub(vadd(a, b), c)
        e1, e2 = vsub(a, centre), vsub(b, centre)
        ok = (dot(vsub(a, c), vsub(a, c)) == R * R and dot(vsub(b, c), vsub(b, c)) == R * R
              and dot(vsub(a, c), vsub(b, c)) == 0 and dot(e1, e1) == R * R and dot(e2, e2) == R * R
              and dot(e1, e2) == 0)
        cross_check(f"{bend} ({pid}): tangent nodes R from the specified corner on perpendicular legs, the arc "
                    "centre R from both", ok)

    # RS-R1 geometry (§3.1, §5): chord, drop, offset and centre of gravity.
    a, b = P("RS-150"), P("RS-160")
    chord = vsub(b, a)
    c, n = squarefree(dot(chord, chord))
    cross_check("RS-P150 chord length is sqrt(257)/40 m (0.400 m along +Y, 0.025 m drop)", (c, n) == (F(1, 40), 257))
    cross_check("RS-160's drop equals half the end-size difference (270 - 220)/2 mm (§3.1, §5)",
                a[2] - b[2] == (R1_ENDS_MM[0] - R1_ENDS_MM[1]) / 2000)
    run = (F(0), F(1), F(0))
    for upstream, downstream in (("RS-140", "RS-150"), ("RS-160", "RS-170")):
        d = vsub(P(downstream), P(upstream))
        cross_check(f"{upstream} -> {downstream} runs along +Y (the run axis both neighbours share)",
                    d[0] == 0 and d[2] == 0 and d[1] > 0)
    separation = vsub(a, b)  # from the centreline at RS-160 to the one at RS-150
    transverse = vsub(separation, vmul(dot(separation, run), run))
    cross_check("RS-R1's eccentric_offset (0, 0, +25) mm is the transverse separation from RS-160 to RS-150",
                transverse == tuple(v / 1000 for v in R1_OFFSET_MM))
    r1_pos, r1_node = component_position("RS-R1")
    r1_cog = vsub(r1_pos, P(r1_node))
    cross_check("RS-R1's centre of gravity from RS-160 is the body midpoint less the node, (0, -0.2, 0.0125) m",
                r1_cog == (F(0), F("-0.2"), F("0.0125")))
    r1_wrench = cross(r1_cog, (F(0), F(0), F(-300)))
    cross_check("RS-R1's weight wrench at RS-160: M = r x F = (60, 0, 0) N*m", r1_wrench == (F(60), F(0), F(0)))

    # The two systems.
    systems = {}
    for system in SYSTEMS:
        w, s, members, comps = system_weight(system)
        systems[system] = {"W": w, "S": s, "members": members, "components": comps}
    t1, ad = systems["tranche_1"], systems["adopted"]
    p150_w, p150_s, _, _ = member_weight("RS-P150")
    r1_w = X.q(300)
    r1_s = [X.q(300 * r1_pos[i]) for i in range(3)]

    # Replacement route: tranche 1 less RS-P150's distributed weight plus RS-R1's weight at its CoG.
    cross_check("adopted W equals tranche-1 W less RS-P150's distributed weight plus RS-R1's 300 N",
                ad["W"] == t1["W"] - p150_w + r1_w)
    cross_check("adopted first moment equals tranche-1 first moment less RS-P150's plus RS-R1's at its CoG",
                all(ad["S"][i] == t1["S"][i] - p150_s[i] + r1_s[i] for i in range(3)))
    # A second route for the adopted total: per-section length sums of the weighted members, plus components.
    alt = X()
    weighted = {m["member"] for m in ad["members"] if "weight_N" in m}
    for sid in SECTIONS:
        lengths = X()
        for pid, a_id, b_id, s in MEMBERS:
            if s == sid and pid in weighted:
                if pid in ARCS:
                    lengths = lengths + X.q(F(ARCS[pid][2]) / 2, 1)  # R * pi / 2
                else:
                    d = vsub(P(b_id), P(a_id))
                    c, n = squarefree(dot(d, d))
                    lengths = lengths + X.q(c, 0, n)
        alt = alt + lengths * X.q(section_mass(sid)[1] * G, 1)
    alt = alt + X.q(sum(F(RIGID[c][2]) for c in SYSTEMS["adopted"]))
    cross_check("adopted W again as per-section length sums times weight per length, plus component weights",
                alt == ad["W"])
    cross_check("the adopted W carries no sqrt(257) term: RS-P150, the one inclined member, carries no distributed "
                "weight", all(n == 1 for (_, n) in ad["W"].t))

    # Controls against the frozen independent references (consistency only).
    frozen = json.loads(TRANCHE1.read_text(encoding="utf-8"))
    fk = frozen["key_values"]
    fp150 = next(m for m in frozen["members"] if m["member"] == "pipe:RS-P150")
    cross_check(f"tranche-1 W recomputed here equals {TRANCHE1_REL} W_total_N exactly",
                parse_exact(fk["W_total_N"]["exact"]) == t1["W"] and fk["W_total_N"]["value"] == t1["W"].record()["value"])
    cross_check(f"tranche-1 first moment recomputed here equals {TRANCHE1_REL} exactly",
                all(parse_exact(fk["weight_first_moment_N_m"][i]["exact"]) == t1["S"][i] for i in range(3)))
    cross_check(f"RS-P150's distributed weight and first moment equal {TRANCHE1_REL}'s member record exactly",
                parse_exact(fp150["weight_N"]["exact"]) == p150_w
                and all(parse_exact(fp150["first_moment_N_m"][i]["exact"]) == p150_s[i] for i in range(3)))
    pending = json.loads(PENDING.read_text(encoding="utf-8"))["context"]
    cross_check(f"adopted W equals {PENDING_REL}'s RS-SUS full-system (RS-R1 in place) W_total exactly",
                parse_exact(pending["RS-SUS_full_W_total_N"]["exact"]) == ad["W"])
    cross_check(f"adopted weight moment about the origin equals {PENDING_REL}'s full-system value exactly",
                all(parse_exact(pending["RS-SUS_full_weight_moment_about_origin_N_m"][i]["exact"]) == x
                    for i, x in enumerate(weight_moment(ad["S"]))))

    # RS-CE1's action and the support bookkeeping.
    ce_force = vmul(CE1["constant_load_N"], CE1["direction"])
    ce_moment = cross(P(CE1["node"]), ce_force)
    m_w = weight_moment(ad["S"])
    eq2_moment = [-x for x in m_w]
    others_moment = [eq2_moment[i] - X.q(ce_moment[i]) for i in range(3)]
    cross_check("RS-CE1's moment about the origin: r x F = (27000, -54000, 0) N*m for 9000 N +Z at (6, 3, 7) m",
                ce_moment == (F(27000), F(-54000), F(0)))
    # Negative controls: the bookkeeping errors the threat model names, each a definite value a correct solve misses.
    negative = {
        "R1_added_and_RS-P150_kept": {"what": "RS-R1's 300 N added without removing RS-P150's distributed weight "
                                              "(double-counted body weight)",
                                      "W_N": (ad["W"] + p150_w).record()},
        "R1_omitted": {"what": "the tranche-1 total: RS-P150 a plain member and no RS-R1", "W_N": t1["W"].record()},
        "CE1_counted_twice": {"what": "RS-CE1's 9000 N counted both as its support action and again as an applied "
                                      "load in the vertical sum", "sum_Fz_N": (ad["W"] + X.q(9000)).record()},
        "CE1_omitted_from_the_sum": {"what": "the vertical sum over every support but RS-CE1 taken as EQ-1's",
                                     "sum_Fz_N": (ad["W"] - X.q(9000)).record()},
    }
    for label, value in negative.items():
        v = value.get("W_N") or value["sum_Fz_N"]
        cross_check(f"negative control {label} differs from W by more than 1e-6 relative",
                    abs(v["value"] - ad["W"].record()["value"]) > 1e-6 * ad["W"].record()["value"])

    key = {
        "W_total_N": ad["W"].record(),
        "weight_resultant_N": vrec([X(), X(), -ad["W"]]),
        "weight_first_moment_N_m": vrec(ad["S"]),
        "weight_moment_about_origin_N_m": vrec(m_w),
        "reaction_moment_about_origin_N_m": vrec(eq2_moment),
    }
    replacement = {
        "RS-P150_distributed_weight_N": p150_w.record(),
        "RS-P150_first_moment_N_m": vrec(p150_s),
        "RS-R1_weight_N": r1_w.record(),
        "RS-R1_position_m": [str(v) for v in r1_pos],
        "RS-R1_first_moment_N_m": vrec(r1_s),
        "delta_W_N": (r1_w - p150_w).record(),
        "delta_first_moment_N_m": vrec([r1_s[i] - p150_s[i] for i in range(3)]),
    }
    r1 = {
        "component": "component:RS-R1", "kind": "reducer", "node": "node:RS-160", "body": "pipe:RS-P150",
        "body_chord_m": "(1/40)*sqrt(257)", "end_sizes_mm": [str(v) for v in R1_ENDS_MM],
        "eccentric_offset_mm": [str(v) for v in R1_OFFSET_MM], "stiffness_scale": str(R1_SCALE),
        "center_of_gravity_from_node_m": [str(v) for v in r1_cog],
        "weight_wrench_at_node": {"force_N": ["0", "0", "-300"], "moment_N_m": [str(v) for v in r1_wrench]},
        "body_distributed_weight": "none: the component's weight replaces it",
    }
    weighted_lines = sorted(m["member"] for m in ad["members"] if "weight_N" in m)
    moments = sum(1 for comp in ad["components"] for v in comp["weight_wrench_at_node"]["moment_N_m"] if F(v) != 0)
    inventory = {
        "per_case": {"line_loads": len(weighted_lines), "component_forces": len(ad["components"]),
                     "component_moments": moments,
                     "total": len(weighted_lines) + len(ad["components"]) + moments},
        "weighted_members": [f"pipe:{m}" for m in weighted_lines],
        "rule": "one line load per weighted member; per rigid-family component one force along -Z and one moment per "
                "nonzero component of r x F (r the entered CoG vector from its node)",
        "tranche_1_per_case_total": sum(1 for m in t1["members"] if "weight_N" in m) + len(t1["components"])
        + sum(1 for comp in t1["components"] for v in comp["weight_wrench_at_node"]["moment_N_m"] if F(v) != 0),
    }
    ce = {
        "support": CE1["support"], "node": f"node:{CE1['node']}", "dof": CE1["dof"],
        "constant_load_N": str(CE1["constant_load_N"]),
        "cases": list(CASES),
        "applied_load_N": str(CE1["constant_load_N"]),
        "support_action_on_pipe_N": [str(v) for v in ce_force],
        "support_action_moment_about_node_N_m": ["0", "0", "0"],
        "support_action_moment_about_origin_N_m": [str(v) for v in ce_moment],
        "other_supports_sum_Fz_N": (ad["W"] - X.q(CE1["constant_load_N"])).record(),
        "other_supports_reaction_moment_about_origin_N_m": vrec(others_moment),
        "rule": "RS-CE1 is consumed (DEC-049): its applied-load row and its published support action Fz each state the "
                "entered 9000 N, and its other five components are zero, in every case; in EQ-1 and EQ-2 it is one "
                "support among all, counted once; it adds no weight, so the vertical sum over every support is W",
    }
    return {"key": key, "replacement": replacement, "r1": r1, "ce": ce, "inventory": inventory,
            "negative": negative, "sections": section_rows,
            "tranche_1": {"W_total_N": t1["W"].record(), "weight_moment_about_origin_N_m": vrec(weight_moment(t1["S"])),
                          "agrees_exactly_with": TRANCHE1_REL},
            "members": ad["members"], "components": ad["components"]}


def document(result):
    return {
        "document": "swbpipe.reference_system_r1_ce_references",
        "status": "candidate frozen references; derived and cross-checked by the author TASK before the adopted fixture "
                  "was solved against them, not yet independently refuted",
        "specification": "DEL-09-01 (EQ-1, EQ-2, EQ-5; KR-03's RS-CE1 identity) on the fixture with RS-R1 and RS-CE1 "
                         "adopted",
        "inputs": "the specification's statements (§3.1, §3.2, §4.2, §5, §6), transcribed in the generator; no fixture "
                  "model file and no product output is read",
        "controls_read": [TRANCHE1_REL, PENDING_REL],
        "units": {"force": "N", "moment": "N*m", "length": "m", "mass_per_length": "kg/m"},
        "gravity": {"value": str(G), "unit": "m/s^2", "direction": "-Z"},
        "cases": list(CASES),
        "checks": {
            "EQ-1": "in RS-SUS, RS-OPE and RS-SAM, the sum of the global Z support action components over every "
                    "support, RS-VS1 and RS-CE1 included, equals W_total",
            "EQ-2": "in the same cases, the horizontal (X, Y) resultant is zero and the support action moment "
                    "resultant about the origin (sum of r x F + M) equals reaction_moment_about_origin",
            "EQ-5": "the RS-dSAM support action resultant force and its moment about the origin are zero; RS-CE1's "
                    "constant force is the same in RS-SAM and RS-SUS",
            "KR-03 (RS-CE1)": "RS-CE1's applied-load row and its published support action Fz equal its entered "
                              "constant_load, 9000 N, with Fx, Fy, Mx, My and Mz zero, in every case",
        },
        "criterion": "|observed - expected| <= 1e-9 * |expected|; an exactly zero expectation uses 1e-9 * zero_scale "
                     "of its family (largest nonzero magnitude of that family in the case); the RS-CE1 identities "
                     "are exact",
        "crosscheck": list(CROSS),
        "key_values": result["key"],
        "replacement": result["replacement"],
        "reducer": result["r1"],
        "constant_effort": result["ce"],
        "self_weight_inventory": result["inventory"],
        "negative_controls": result["negative"],
        "tranche_1": result["tranche_1"],
        "sections": result["sections"],
        "members": result["members"],
        "components": result["components"],
    }


def dumps(obj):
    def inline(o):
        return json.dumps(o, ensure_ascii=False, separators=(", ", ": "))

    def emit(o, level):
        pad = " " * level
        if isinstance(o, dict) and ("exact" in o or "check" in o):
            return inline(o)
        if isinstance(o, dict):
            items = [f'{pad} {json.dumps(k)}: {emit(v, level + 1)}' for k, v in o.items()]
            return "{\n" + ",\n".join(items) + "\n" + pad + "}" if items else "{}"
        if isinstance(o, list) and o and all(isinstance(x, dict) for x in o):
            items = [f"{pad} {emit(v, level + 1)}" for v in o]
            return "[\n" + ",\n".join(items) + "\n" + pad + "]"
        return inline(o)
    return emit(obj, 0) + "\n"


def package_files():
    """Every committed file of the package but the two this generator writes."""
    return {str(p.relative_to(OUT)): p.read_bytes() for p in sorted(OUT.rglob("*"))
            if p.is_file() and "__pycache__" not in p.parts and str(p.relative_to(OUT)) not in WRITTEN}


def build_files():
    result = build()
    text = dumps(document(result))
    assert json.loads(text) == json.loads(json.dumps(document(result)))
    files = {"references.json": text.encode(), **package_files()}
    lines = [f"{hashlib.sha256(data).hexdigest()}  {path}" for path, data in sorted(files.items())]
    files["SHA256SUMS"] = ("\n".join(lines) + "\n").encode()
    return files, result


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    files, result = build_files()
    bad = [c for c in CROSS if not c["agrees"]]
    print(f"cross-checks: {len(CROSS)} recorded, {len(bad)} disagreeing")
    for c in bad:
        print("  DISAGREES:", c["check"])
    key = result["key"]
    print(f"W_total {key['W_total_N']['value']!r} N")
    print("reaction moment about origin", [v["value"] for v in key["reaction_moment_about_origin_N_m"]], "N*m")
    if args.write:
        if bad:
            print("not written: a cross-check disagrees")
            return 1
        for path in WRITTEN:
            (OUT / path).write_bytes(files[path])
        print(f"wrote {', '.join(WRITTEN)}")
        return 0
    failures = [p for p, d in files.items() if not (OUT / p).is_file() or (OUT / p).read_bytes() != d]
    if failures or bad:
        print("MISMATCH: " + ", ".join(failures))
        return 1
    print(f"verified {len(files)} files byte for byte (SHA256SUMS included)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
