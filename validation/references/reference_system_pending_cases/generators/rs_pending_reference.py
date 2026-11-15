#!/usr/bin/env python3
"""Reference-system pending cases: wind and static seismic resultants (EQ-6) and the hydrotest weight (EQ-1, EQ-2).

Standard-library Python 3 only. This generator imports, reads and runs no product code and reads no fixture
model file. Its inputs are the reference-system specification's own statements
(`fixtures/reference_system/SPECIFICATION.md`), transcribed below and checked against that file's tables and
sentences on every run, so that an edit to the specification's inputs fails the check until the references are
regenerated. It reads one other file, the existing package's `references.json`, only to compare its RS-SUS total
weight and moment with the same quantities recomputed here (a consistency check; nothing is taken from it).

Bases (README, "Bases"):

  pipe metal       rho_m * pi * (R^2 - (R - t)^2)                  nominal wall
  contents         rho_c * pi * (R - ((t - c_m) - c_a))^2          reduced (pressure) bore
  insulation       rho_i * pi * ((R + t_i)^2 - R^2)                OD to OD + 2 t_i
  realized arc     length R * pi/2; first moment R * [(pi/2) C + e1 + e2]
  wind             p * C_s * (D + 2 t_i) per length, along +X, on the marked members
  seismic          the g-factor times the self-weight plan evaluated at the seismic case's gravity, with each
                   entered component force W held fixed (decided 2026-11-12): a * g * mu per length on every member except the replaced span and the members whose component weight the
                   weight case applies, and a * W at each such component's centre of gravity; positive axis
  hydrotest weight mu with rho_c replaced by the test contents density, times g along -Z; rigid-family component
                   weights at their centres of gravity in place of their members' distributed weight

Every result is an exact sum of terms q * pi^k * sqrt(n) with q rational. pi is bounded by Machin's formula in
integer arithmetic and sqrt(n) by the integer square root, each to 100 decimal places; a printed decimal (40
significant digits) and a binary64 value are emitted only when both ends of the bounding interval give the same.

Modes:
  python3 -I rs_pending_reference.py           check: rebuild every output in memory and compare byte for byte
                                                with the committed files, SHA256SUMS included
  python3 -I rs_pending_reference.py --write   write references.json and SHA256SUMS (intentional changes only)
"""
import argparse
import hashlib
import json
import math
import re
import struct
import sys
from fractions import Fraction as F
from math import isqrt
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE.parent
ROOT = HERE.parents[3]
SPEC = ROOT / "fixtures" / "reference_system" / "SPECIFICATION.md"
SPEC_REL = "fixtures/reference_system/SPECIFICATION.md"
EXISTING = ROOT / "validation" / "references" / "reference_system" / "references.json"
EXISTING_REL = "validation/references/reference_system/references.json"
FIXED_FILES = ("README.md", "generators/rs_pending_reference.py")

# ============================================================================ controlled evaluation of pi and sqrt
DIGITS = 100
_GUARD = 12
_SCALE = 10 ** (DIGITS + _GUARD)


def _arctan_inv_scaled(x):
    """floor-truncated series for arctan(1/x) * _SCALE; each of its terms is truncated by less than 1."""
    total, power, n, sign = 0, _SCALE // x, 1, 1
    x2 = x * x
    while power:
        total += sign * (power // n)
        power //= x2
        n += 2
        sign = -sign
    return total


_PI_SCALED = 16 * _arctan_inv_scaled(5) - 4 * _arctan_inv_scaled(239)
# Fewer than 200 truncated terms in all, each off by less than 2 units (the power and the quotient), times 16 at
# most: the error is below 10^4 units of 10^-(DIGITS + _GUARD).
PI_LO, PI_HI = F(_PI_SCALED - 10 ** 4, _SCALE), F(_PI_SCALED + 10 ** 4, _SCALE)
assert PI_LO < F(314159265358979323847, 10 ** 20) and PI_HI > F(314159265358979323846, 10 ** 20)
assert PI_HI - PI_LO < F(1, 10 ** DIGITS)


def sqrt_bounds(n):
    s = isqrt(n * 10 ** (2 * DIGITS))
    assert s * s <= n * 10 ** (2 * DIGITS) < (s + 1) ** 2
    return F(s, 10 ** DIGITS), F(s + 1, 10 ** DIGITS)


def squarefree(n):
    """sqrt(n) = c * sqrt(m) with m squarefree (n a positive integer)."""
    c, m, p = 1, n, 2
    while p * p <= m:
        while m % (p * p) == 0:
            m //= p * p
            c *= p
        p += 1
    return c, m


def sig(fr, digits=40):
    """A rational printed with `digits` significant digits, round half to even, as Decimal's '.39e' prints."""
    if fr == 0:
        return "0"
    sign, a = ("-" if fr < 0 else ""), abs(fr)
    e = 0
    while a >= F(10) ** (e + 1):
        e += 1
    while a < F(10) ** e:
        e -= 1
    m = round(a * F(10) ** (digits - 1 - e))
    if m == 10 ** digits:
        m, e = m // 10, e + 1
    s = str(m)
    return f"{sign}{s[0]}.{s[1:]}e{'+' if e >= 0 else '-'}{abs(e)}"


class X:
    """An exact sum of q * pi^k * sqrt(n) terms (q rational, n squarefree), keyed by (k, n)."""

    def __init__(self, terms=None):
        self.t = {key: F(q) for key, q in (terms or {}).items() if q != 0}

    @staticmethod
    def q(value, k=0, n=1):
        return X({(k, n): F(value)})

    def __add__(self, other):
        out = dict(self.t)
        for key, q in other.t.items():
            out[key] = out.get(key, F(0)) + q
        return X(out)

    def __neg__(self):
        return X({key: -q for key, q in self.t.items()})

    def __sub__(self, other):
        return self + (-other)

    def __mul__(self, other):
        if not isinstance(other, X):
            other = X.q(other)
        out = {}
        for (k1, n1), a in self.t.items():
            for (k2, n2), b in other.t.items():
                c, m = squarefree(n1 * n2)
                out[(k1 + k2, m)] = out.get((k1 + k2, m), F(0)) + a * b * c
        return X(out)

    __rmul__ = __mul__

    def __eq__(self, other):
        return self.t == other.t

    def text(self):
        if not self.t:
            return "0"
        parts = []
        for (k, n), q in sorted(self.t.items()):
            factor = "".join(([f"*pi^{k}" if k > 1 else "*pi"] if k else []) + ([f"*sqrt({n})"] if n != 1 else []))
            parts.append(f"({q}){factor}" if factor else f"({q})")
        return " + ".join(parts)

    def interval(self):
        lo = hi = F(0)
        for (k, n), q in self.t.items():
            slo, shi = sqrt_bounds(n) if n != 1 else (F(1), F(1))
            a, b = q * PI_LO ** k * slo, q * PI_HI ** k * shi
            lo, hi = lo + min(a, b), hi + max(a, b)
        return lo, hi

    def record(self):
        if not self.t:
            return {"exact": "0", "decimal": "0", "value": 0.0}
        lo, hi = self.interval()
        if float(lo) != float(hi) or sig(lo) != sig(hi):
            raise SystemExit(f"rounding of {self.text()} is not decided by its bounds")
        return {"exact": self.text(), "decimal": sig(lo), "value": float(lo)}

    def approx(self):
        lo, hi = self.interval()
        return float((lo + hi) / 2)


def parse_exact(text):
    """The inverse of X.text, for the existing package's exact strings."""
    if text == "0":
        return X()
    out = {}
    for term in text.split(" + "):
        m = re.fullmatch(r"\((-?\d+(?:/\d+)?)\)(?:\*pi(?:\^(\d+))?)?(?:\*sqrt\((\d+)\))?", term)
        assert m, term
        k = 0 if "*pi" not in term else int(m.group(2) or 1)
        out[(k, int(m.group(3) or 1))] = F(m.group(1))
    return X(out)


def vrec(vector):
    return [x.record() for x in vector]


# ============================================================================ the specification's inputs (transcribed)
# Each is checked against SPECIFICATION.md by check_transcription(); the section it comes from is named.
G = F("9.80665")  # §2: gravity on the weight case, global_z -9.80665 m/s^2
NODES = {  # §3.1, m
    "010": ("0", "0", "1.000"), "020": ("0.400", "0", "1.000"), "030": ("0.550", "0", "1.000"),
    "040": ("1.150", "0", "1.000"), "050": ("1.300", "0", "1.000"), "060": ("3.500", "0", "1.000"),
    "070": ("5.600", "0", "1.000"), "075": ("6.000", "0", "1.400"), "080": ("6.000", "0", "4.000"),
    "090": ("6.000", "0", "6.600"), "095": ("6.000", "0.400", "7.000"), "100": ("6.000", "3.000", "7.000"),
    "110": ("6.000", "7.000", "7.000"), "120": ("6.000", "11.000", "7.000"), "130": ("6.000", "15.000", "7.000"),
    "140": ("6.000", "18.000", "7.000"), "150": ("6.000", "20.000", "7.000"), "160": ("6.000", "20.400", "6.975"),
    "170": ("6.000", "23.000", "6.975"), "180": ("6.000", "25.000", "6.975"), "190": ("6.000", "25.600", "6.975"),
    "200": ("6.000", "28.000", "6.975"), "210": ("6.000", "30.000", "6.975"), "215": ("6.000", "15.000", "4.500"),
    "220": ("6.000", "15.000", "2.250"), "225": ("6.250", "15.000", "2.000"), "230": ("8.000", "15.000", "2.000"),
}
MEMBERS = [  # §3.2: member, from, to, section
    ("RS-P010", "010", "020", "S270"), ("RS-P020", "020", "030", "S270"), ("RS-P030", "030", "040", "S270"),
    ("RS-P040", "040", "050", "S270"), ("RS-P050", "050", "060", "S270"), ("RS-P060", "060", "070", "S270"),
    ("RS-P065", "070", "075", "S270"), ("RS-P070", "075", "080", "S270"), ("RS-P080", "080", "090", "S270"),
    ("RS-P085", "090", "095", "S270"), ("RS-P090", "095", "100", "S270"), ("RS-P100", "100", "110", "S270"),
    ("RS-P110", "110", "120", "S270"), ("RS-P120", "120", "130", "S270"), ("RS-P130", "130", "140", "S270"),
    ("RS-P140", "140", "150", "S270"), ("RS-P150", "150", "160", "S270"), ("RS-P160", "160", "170", "S220"),
    ("RS-P170", "170", "180", "S220"), ("RS-P180", "180", "190", "S220"), ("RS-P190", "190", "200", "S220"),
    ("RS-P200", "200", "210", "S220"), ("RS-P210", "130", "215", "S170"), ("RS-P220", "215", "220", "S170"),
    ("RS-P225", "220", "225", "S170"), ("RS-P230", "225", "230", "S170"),
]
# §4.2: OD, wall, mill tolerance, corrosion allowance (mm), insulation thickness (mm) @ density, contents (kg/m^3)
SECTIONS = {
    "S270": ("270", "9.0", "1.0", "1.5", "60", "140", "820"),
    "S220": ("220", "8.0", "0.9", "1.5", "60", "140", "820"),
    "S170": ("170", "7.0", "0.8", "1.5", "50", "140", "820"),
}
RHO_M = F(7850)  # §4.1, §4.2
RHO_TEST = F(1000)  # §4.2, §8.3: test contents density
# §3.1 corners and §5 radius (90 degree realized bends)
ARCS = {"RS-P065": ("RS-B1", ("6", "0", "1"), "0.400"), "RS-P085": ("RS-B2", ("6", "0", "7"), "0.400"),
        "RS-P225": ("RS-B3", ("6", "15", "2"), "0.250")}
# §5 rigid family: component, mapped member, node, weight (N), CoG offset from the body midpoint (m)
RIGID = {
    "RS-F1": ("RS-P020", "030", "350", ("0", "0", "0")),
    "RS-V1": ("RS-P030", "040", "2600", ("0", "0", "0.30")),
    "RS-F2": ("RS-P040", "050", "350", ("0", "0", "0")),
    "RS-R1": ("RS-P150", "160", "300", ("0", "0", "0")),
}
REPLACED_SPAN = "RS-P180"  # §5.1
WIND = {"pressure_Pa": F(900), "shape_factor": F("0.7"), "direction": (1, 0, 0),  # §8.1 RS-WIN
        "exposed_first": "RS-P070", "exposed_last": "RS-P200", "excluded": REPLACED_SPAN}
SEISMIC = {"RS-EQX": (F("0.30"), 0), "RS-EQY": (F("0.30"), 1), "RS-EQZ": (F("0.20"), 2)}  # §8.1

# The specification's sentences each transcription rests on; each must appear in SPECIFICATION.md verbatim.
SPEC_SENTENCES = [
    "Gravity is entered on each weight case as\n  `global_z`, value −9.80665 m/s²",
    "density 7850 kg/m³ (on each member's section as `material_density`)",
    "The hydrotest's test contents are\n1000 kg/m³",
    "contents mass and pressure end-cap areas use the reduced bore (nominal wall\nless mill tolerance and "
    "corrosion allowance)",
    "| RS-B1 | `bend`, realized (`curved_bend_macro_element`), at RS-070 | arc RS-P065 | R 0.400 m, 90°;",
    "| RS-B2 | `bend`, realized, at RS-090 | arc RS-P085 | R 0.400 m, 90°;",
    "| RS-B3 | `bend`, realized, at RS-220 | arc RS-P225 | R 0.250 m, 90°;",
    "Corner points: B1 (6, 0, 1), B2 (6, 0, 7), B3 (6, 15, 2).",
    "| RS-R1 | `reducer`, eccentric | RS-P150, node RS-160 |",
    "weight 300 N, CoG at the body midpoint",
    "| RS-F1 | `flange` | RS-P020, node RS-030 | weight 350 N; CoG at the body midpoint;",
    "| RS-V1 | `valve` | RS-P030, node RS-040 | weight 2600 N; CoG offset (0, 0, +0.30) m from the body midpoint",
    "| RS-F2 | `flange` | RS-P040, node RS-050 | weight 350 N; CoG at the body midpoint;",
    "| RS-EJ1 | `expansion_joint`, objective connector 1.0.0 | replaces RS-P180,",
    "It is excluded\n  from weight generation and from every primitive load",
    "| RS-WIN | W + P + wind: 0.9 kPa, shape factor 0.7, `global_x`, exposed members RS-P070 … RS-P200 except the "
    "replaced span RS-P180 |",
    "| RS-EQX | W + P + seismic, `g_factor_x` 0.30 |",
    "| RS-EQY | W + P + seismic, `g_factor_y` 0.30 |",
    "| RS-EQZ | W + P + seismic, `g_factor_z` 0.20 |",
    "| Eccentric reducer RS-R1 in region RG-R | pending; RS-P150 is in tranche 1 as a plain S270 member in its own "
    "region RG-R |",
]

CROSS = []


def cross_check(label, agrees):
    CROSS.append({"check": label, "agrees": bool(agrees)})


def check_transcription():
    text = SPEC.read_text(encoding="utf-8")
    nodes = {m.group(1): tuple(s.strip() for s in m.group(2).split(","))
             for m in re.finditer(r"^\| RS-(\d{3}) \| ([-0-9., ]+) \|", text, re.M)}
    cross_check("§3.1 node table equals the transcribed nodes", nodes == NODES)
    members = [(m.group(1), m.group(2), m.group(3), m.group(4))
               for m in re.finditer(r"^\| (RS-P\d{3}) \| (\d{3}) → (\d{3}) \| (S\d{3}) \|", text, re.M)]
    cross_check("§3.2 member table equals the transcribed members, in order", members == MEMBERS)
    sections = {m.group(1): m.groups()[1:] for m in re.finditer(
        r"^\| RS-(S\d{3}) \| (\S+) × (\S+) \| (\S+) \| (\S+) \| (\S+) @ (\S+) \| (\S+) \|", text, re.M)}
    cross_check("§4.2 section table equals the transcribed sections", sections == SECTIONS)
    missing = [s for s in SPEC_SENTENCES if s not in text]
    cross_check(f"{len(SPEC_SENTENCES)} statements of the specification quoted in the generator appear verbatim",
                not missing)
    for s in missing:
        print("  specification statement not found:", s.splitlines()[0])


# ============================================================================ geometry
def P(node):
    return tuple(F(x) for x in NODES[node])


def vsub(a, b):
    return tuple(x - y for x, y in zip(a, b))


def vadd(a, b):
    return tuple(x + y for x, y in zip(a, b))


def vmul(q, a):
    return tuple(q * x for x in a)


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def length(a, b):
    """|b - a| as an exact X (rational times sqrt of a squarefree integer)."""
    q = dot(vsub(b, a), vsub(b, a))
    c, m = squarefree(q.numerator * q.denominator)
    return X.q(F(c, q.denominator), 0, m)


def straight(a, b):
    """(length, integral of position over the member) for the straight member a -> b."""
    L = length(a, b)
    mid = vmul(F(1, 2), vadd(a, b))
    return [(L, [L * mid[i] for i in range(3)])]


def arc_geometry(pid):
    _, corner, radius = ARCS[pid]
    a, b = P(MEMBER[pid][1]), P(MEMBER[pid][2])
    c, R = tuple(F(x) for x in corner), F(radius)
    centre = vsub(vadd(a, b), c)
    e1, e2 = vsub(a, centre), vsub(b, centre)
    return a, b, c, R, centre, e1, e2


def segments(pid, arc_mode):
    """The member's geometry as (length, first moment of length) segments.

    arc_mode "arc": a realized bend is its quarter circle (length R pi/2), as §3 and §5 specify. "legs": a control
    that departs from the specification, the two straight tangent legs from the tangent nodes to the corner
    (length 2R)."""
    _, frm, to, _ = MEMBER[pid]
    if pid not in ARCS:
        return straight(P(frm), P(to))
    a, b, c, R, centre, e1, e2 = arc_geometry(pid)
    if arc_mode == "legs":
        return straight(a, c) + straight(c, b)
    L = X.q(R / 2, 1)
    return [(L, [X.q(R * centre[i] / 2, 1) + X.q(R * (e1[i] + e2[i])) for i in range(3)])]


MEMBER = {m[0]: m for m in MEMBERS}


def check_geometry():
    for pid, (bend, corner, radius) in sorted(ARCS.items()):
        a, b, c, R, centre, e1, e2 = arc_geometry(pid)
        ok = (dot(vsub(a, c), vsub(a, c)) == R * R and dot(vsub(b, c), vsub(b, c)) == R * R
              and dot(vsub(a, c), vsub(b, c)) == 0 and dot(e1, e1) == R * R and dot(e2, e2) == R * R
              and dot(e1, e2) == 0 and dot(e1, vsub(c, a)) == 0 and dot(e2, vsub(c, b)) == 0)
        cross_check(f"{bend} ({pid}): tangent nodes lie R from the corner on perpendicular legs, and the arc centre "
                    f"is R from both, tangent to both legs", ok)
        # The first moment of the arc against a composite Simpson quadrature of the parametrised quarter circle.
        (L, fm), = segments(pid, "arc")
        n = 2000
        h = (math.pi / 2) / n
        ok = True
        for i in range(3):
            def r(th):
                return float(centre[i]) + float(e1[i]) * math.cos(th) + float(e2[i]) * math.sin(th)
            s = r(0) + r(math.pi / 2) + sum((4 if j % 2 else 2) * r(j * h) for j in range(1, n))
            quad = float(R) * s * h / 3
            ok = ok and abs(quad - fm[i].approx()) <= 1e-11 * (1 + abs(quad))
        cross_check(f"{bend} ({pid}): arc first moment R[(pi/2)C + e1 + e2] agrees with Simpson quadrature (1e-11)",
                    ok)
    L = length(P("150"), P("160"))
    cross_check("RS-P150 chord length is sqrt(257)/40 m (§3.1: 0.400 m along Y, 0.025 m drop)",
                L == X.q(F(1, 40), 0, 257))
    # The reducer's eccentric drop is half the OD difference (§3.1).
    cross_check("RS-160 drop equals half the S270 / S220 outside-diameter difference (§3.1, §5 RS-R1)",
                P("150")[2] - P("160")[2] == (F(SECTIONS["S270"][0]) - F(SECTIONS["S220"][0])) / 2000)


# ============================================================================ sections
def section(sid, rho_c=None, insulation=True):
    od, wall, mill, ca, ti, rho_i, rho_c0 = (F(x) for x in SECTIONS[sid])
    R, t, cm, ca, ti = od / 2000, wall / 1000, mill / 1000, ca / 1000, ti / 1000
    if not insulation:
        ti = F(0)
    rho_c = rho_c0 if rho_c is None else rho_c
    rn = R - t
    rp = R - ((t - cm) - ca)
    parts = {"metal": RHO_M * (R * R - rn * rn), "contents": rho_c * rp * rp,
             "insulation": rho_i * ((R + ti) ** 2 - R * R)}
    total = sum(parts.values())
    # Second form: annuli as t (D - t) and t_i (D + t_i) over 4 ... times 4 / 4; bore as (D/2 - t + c_m + c_a)^2.
    d = 2 * R
    alt = RHO_M * t * (d - t) + rho_c * (d / 2 - t + cm + ca) ** 2 + rho_i * ti * (d + ti)
    return {"R": R, "t": t, "cm": cm, "ca": ca, "ti": ti, "rho_i": rho_i, "rho_c": rho_c, "parts": parts,
            "total": total, "alt": alt, "bore": rp * rp, "projected_diameter": d + 2 * ti}


def check_sections():
    for sid in SECTIONS:
        for rho_c in (None, RHO_TEST):
            s = section(sid, rho_c)
            cross_check(f"RS-{sid}: mass per length / pi in a second factored form (contents "
                        f"{s['rho_c']} kg/m^3)", s["total"] == s["alt"])


# ============================================================================ systems and readings
def mapped(system):
    """Members whose distributed weight a rigid-family component replaces, in this system."""
    comps = {"RS-F1", "RS-V1", "RS-F2"} | ({"RS-R1"} if system == "full" else set())
    return {RIGID[c][0]: c for c in sorted(comps)}


def component_position(cid):
    pid, node, w, offset = RIGID[cid]
    _, frm, to, _ = MEMBER[pid]
    assert node in (frm, to)
    mid = vmul(F(1, 2), vadd(P(frm), P(to)))
    return vadd(mid, tuple(F(x) for x in offset))


def line_items(pid, per_length, arc_mode):
    """(total, first moment) of a quantity per length (an X) over the member."""
    total, fm = X(), [X(), X(), X()]
    for L, m in segments(pid, arc_mode):
        total = total + per_length * L
        fm = [fm[i] + per_length * m[i] for i in range(3)]
    return total, fm


def accumulate(items):
    total, fm = X(), [X(), X(), X()]
    for t, m in items:
        total = total + t
        fm = [fm[i] + m[i] for i in range(3)]
    return total, fm


def seismic_weight(system, mapped_mode="component", replaced_span=False, arc_mode="arc", insulation=True):
    """The weight (N) the seismic g-factors multiply, and its first moment (N m): the seismic force along an axis
    is a times it, and the moment about the origin a times (first moment x axis).

    mapped_mode "component" (the decided rule, SWBPIPE Agent 0, 2026-11-12, in DEL-05-01): every member carries
    g * mu per length, except a member whose component weight the weight case applies, which carries instead the
    component's weight W at its centre of gravity (the force a * W at the node with its moments about the node from
    the centre-of-gravity vector; a * W is a force and no gravity divides it). So the seismic weight is the self-weight
    plan evaluated at the seismic case's gravity, with each entered component force W held fixed; with the seismic
    gravity taken as the weight case's (the stated assumption), it equals the weight case's weight here. Controls: "member", the rule before 2026-11-12 (pipe mass on a mapped member,
    component weight left out); "both", the double count (the member's pipe weight and the component's weight)."""
    comp_of = mapped(system)
    items, rows = [], {}
    for pid, _, _, sid in MEMBERS:
        if pid == REPLACED_SPAN and not replaced_span:
            continue
        if pid in comp_of and mapped_mode == "component":
            continue
        q = X.q(section(sid, insulation=insulation)["total"] * G, 1)
        item = line_items(pid, q, arc_mode)
        items.append(item)
        rows[pid] = item
    if mapped_mode in ("component", "both"):
        for pid, cid in comp_of.items():
            w = F(RIGID[cid][2])
            pos = component_position(cid)
            item = (X.q(w), [X.q(w * pos[i]) for i in range(3)])
            items.append(item)
            rows[cid] = item
    return accumulate(items), rows


def test_weight(system, rho_c=RHO_TEST, mapped_contents=None, arc_mode="arc", insulation=True):
    """Total weight (N) and its first moment (N m) with contents density rho_c on every member that carries
    distributed weight; rigid-family weights at their centres of gravity in place of their members'; the replaced
    span carries none.

    mapped_contents None: component weights unchanged and nothing added for their bodies' contents (primary).
    "full": the contents rho_c of the mapped member's bore added along it. "increment": only (rho_c - section's
    own contents density) of that bore added along it."""
    comp_of = mapped(system)
    items, rows = [], {}
    for pid, _, _, sid in MEMBERS:
        if pid == REPLACED_SPAN:
            continue
        s = section(sid, rho_c, insulation)
        if pid in comp_of:
            cid = comp_of[pid]
            w = F(RIGID[cid][2])
            pos = component_position(cid)
            item = (X.q(w), [X.q(w * pos[i]) for i in range(3)])
            items.append(item)
            rows[cid] = item
            if mapped_contents:
                dens = rho_c if mapped_contents == "full" else rho_c - F(SECTIONS[sid][6])
                extra = line_items(pid, X.q(dens * s["bore"] * G, 1), arc_mode)
                items.append(extra)
                rows[pid + " contents"] = extra
            continue
        item = line_items(pid, X.q(s["total"] * G, 1), arc_mode)
        items.append(item)
        rows[pid] = item
    return accumulate(items), rows


def exposed_members(include_arc=True):
    ids = [m[0] for m in MEMBERS]
    span = ids[ids.index(WIND["exposed_first"]):ids.index(WIND["exposed_last"]) + 1]
    return [p for p in span if p != WIND["excluded"] and (include_arc or p not in ARCS)]


def wind_load(arc_mode="arc", include_arc=True, insulation=True):
    """Total wind force (N, along +X) and its first moment (N m)."""
    items, rows = [], {}
    for pid in exposed_members(include_arc):
        sid = MEMBER[pid][3]
        q = WIND["pressure_Pa"] * WIND["shape_factor"] * section(sid, insulation=insulation)["projected_diameter"]
        item = line_items(pid, X.q(q), arc_mode)
        items.append(item)
        rows[pid] = item
    return accumulate(items), rows


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def resultant_along(magnitude, first_moment, axis):
    """Force and moment about the origin of a distributed load of total `magnitude` along +axis whose first
    moment (integral of position times intensity) is `first_moment`: F = magnitude e, M = first_moment x e."""
    e = [X.q(1 if i == axis else 0) for i in range(3)]
    force = [magnitude * e[i] for i in range(3)]
    return force, cross(first_moment, e)


def weight_resultant(total, fm):
    """Weight along -Z: force (0, 0, -W), moment about the origin S x (0, 0, -1) = (-S_y, S_x, 0)."""
    return [X(), X(), -total], [-fm[1], fm[0], X()]


# ============================================================================ the cases
def case_values(system, readings=None):
    r = dict(readings or {})
    out = {}
    (wind_total, wind_fm), _ = wind_load(r.get("arc", "arc"), r.get("wind_arc_exposed", True),
                                         r.get("insulation", True))
    f, m = resultant_along(wind_total, wind_fm, 0)
    out["RS-WIN"] = (f, m)
    (sw, swfm), _ = seismic_weight(system, r.get("mapped", "component"), r.get("replaced_span", False),
                                   r.get("arc", "arc"), r.get("insulation", True))
    for case, (a, axis) in SEISMIC.items():
        f, m = resultant_along(sw * a, [x * a for x in swfm], axis)
        out[case] = (f, m)
    mass = sw * (1 / G)  # reported only: the total mass, the seismic weight over g
    (w, wfm), _ = test_weight(system, RHO_TEST, r.get("mapped_contents"), r.get("arc", "arc"),
                              r.get("insulation", True))
    out["RS-HYD"] = weight_resultant(w, wfm)
    out["_mass"] = mass
    out["_hyd_W"] = w
    return out


def case_records(values):
    rec = {}
    for case in ("RS-WIN", "RS-EQX", "RS-EQY", "RS-EQZ"):
        f, m = values[case]
        diff = "RS-d" + case[3:]
        rec[case] = {"generated_force_N": vrec(f), "generated_moment_about_origin_N_m": vrec(m),
                     f"expected_{diff}_reaction_force_resultant_N": vrec([-x for x in f]),
                     f"expected_{diff}_reaction_moment_resultant_about_origin_N_m": vrec([-x for x in m])}
    f, m = values["RS-HYD"]
    rec["RS-HYD"] = {"W_total_N": (-f[2]).record(), "weight_moment_about_origin_N_m": vrec(m),
                     "expected_reaction_force_resultant_N": vrec([-x for x in f]),
                     "expected_reaction_moment_resultant_about_origin_N_m": vrec([-x for x in m])}
    return rec


SYSTEMS = {
    "full": "the full specified system (§3-§5): rigid-family components RS-F1, RS-V1, RS-F2 and the eccentric "
            "reducer RS-R1 on RS-P150 (300 N at the body midpoint)",
    "fixture_today": "the system as the fixture carries it today (§10): RS-R1 pending, RS-P150 a plain S270 member "
                     "carrying its own distributed weight; RS-F1, RS-V1, RS-F2 as specified",
}

SEISMIC_RULE = ("decided by SWBPIPE Agent 0 on 2026-11-12, in DEL-05-01's wording: for each global axis, the g-factor "
                "times the weight the weight case gives the model: on every member a load per length a * g * mu; for "
                "a member whose component weight the weight case applies, in place of that load, the force a * W at "
                "the component's node with its moments about the node from the centre-of-gravity vector; a member "
                "whose component weight the weight case does not apply keeps its distributed load. Which component "
                "weights the weight case applies is the specification's (§5, §8.1: component weights at their "
                "centres of gravity in place of their members' distributed weight): RS-F1, RS-V1, RS-F2 in both "
                "systems and RS-R1 in the full system; RS-R1 is pending in the fixture today (§10), where RS-P150 "
                "keeps its distributed weight and so its distributed seismic load. Convention (SWBPIPE Agent 0): the "
                "self-weight plan evaluated at the seismic case's gravity, with each entered component force W held "
                "fixed; it names an exception and does not make the two mass models equivalent in general. This "
                "closes the former open reading OR-1.")

SEISMIC_CONTROLS = [
    ("control: the rule before 2026-11-12: pipe mass on a mapped member, component weight left out",
     {"mapped": "member"}),
    ("control: double count: the mapped member's pipe mass and the component's weight both", {"mapped": "both"}),
]

SPECIFICATION_DEPARTURE_CONTROLS = [
    ("SD-2", "seismic mass given to the replaced span RS-P180", "departure from the specification",
     "§5.1: 'The replaced span RS-P180 carries no load, member result or self-weight. It is excluded from weight "
     "generation and from every primitive load'; the references give it no seismic mass and no wind",
     [("control, departing from §5.1: RS-P180's S220 mass per length over its 0.600 m added to the seismic weight",
       {"replaced_span": True})],
     ("RS-EQX", "RS-EQY", "RS-EQZ")),
    ("SD-3", "realized bends replaced, or a bend's wind exposure omitted", "departure from the specification",
     "§3: 'It has 27 nodes (corner points are not nodes; each realized bend has two tangent nodes) and 26 members: 22 "
     "straight, three arcs and the span the expansion joint replaces.'; §3.2 lists RS-P085 between RS-P080 and RS-P090; "
     "§8.1: 'exposed members RS-P070 … RS-P200 except the replaced span RS-P180'. The references take each arc as its "
     "quarter circle, length R pi/2 (its projected area normal to X is exactly arc length times projected diameter), "
     "and RS-P085 as exposed",
     [("control, departing from §3 (three arcs): the two tangent legs to the corner, length 2R, as straight members",
       {"arc": "legs"}),
      ("control, departing from §8.1's exposed range: the arc RS-P085 not exposed to wind; mass unchanged",
       {"wind_arc_exposed": False})],
     ("RS-WIN", "RS-EQX", "RS-EQY", "RS-EQZ", "RS-HYD")),
    ("SD-4", "insulation removed", "departure from the specification",
     "§4.2's section table gives every section its insulation (60 @ 140, 60 @ 140, 50 @ 140 mm @ kg/m^3), and "
     "DEL-05-01 takes the wind's projected diameter as 'outside diameter plus twice the insulation thickness'; the "
     "references carry that insulation on every member, mapped members included",
     [("control, departing from §4.2's insulation: bare pipe, projected diameter OD, no insulation mass",
       {"insulation": False})],
     ("RS-WIN", "RS-EQX", "RS-EQY", "RS-EQZ", "RS-HYD")),
    ("SD-5", "contents added inside the rigid-family components' bodies in RS-HYD",
     "hypothetical additions the specification does not require",
     "§8.1: 'Component weights are rigid-component `weight` at a centre of gravity, not added weight.' and W is "
     "generated 'with component weights at their centres of gravity in place of their members' distributed weight'; "
     "§8.3 changes only the contents density. The references replace the contents of every member that carries "
     "distributed weight, once, and leave each component's weight unchanged",
     [("hypothetical addition, not required by §8.1 or §8.3: 1000 kg/m^3 over each mapped member's reduced bore, "
       "along the member", {"mapped_contents": "full"}),
      ("hypothetical addition, not required by §8.1 or §8.3: only the increment (1000 - 820) kg/m^3 over each "
       "mapped member's reduced bore", {"mapped_contents": "increment"})],
     ("RS-HYD",)),
]

OPEN_READINGS = [
    {"id": "OR-G", "subject": "the seismic case's gravity magnitude",
     "status": "open: §2 says the seismic case enters gravity_acceleration explicitly but gives no value, and the "
               "lane that holds the specification is adding it to §8.1. Every seismic value here is conditional on the "
               "stated assumption 9.80665 m/s^2 (the weight case's gravity). The distributed part of the seismic weight "
               "scales with g and the component forces W do not, so another value would move each seismic value, but "
               "not in simple proportion. No alternative values are frozen. Nothing else in the specification is left "
               "open for these references."},
]

BASES = {
    "seismic": "the decided rule (seismic_rule), along the positive global axis of the g-factor: a * g * mu on every member except RS-P180 and the members "
               "whose component weight the weight case applies, and a * W at each such component's centre of "
               "gravity; mu on the exact route's bases (README). Conditional on the stated assumption g = 9.80665 "
               "m/s^2 for the seismic case's gravity_acceleration (open reading OR-G)",
    "wind": "900 Pa * 0.7 * (OD + 2 t_i) per length along +X ('global_x') on RS-P070, RS-P080, RS-P085, RS-P090, "
            "RS-P100, RS-P110, RS-P120, RS-P130, RS-P140, RS-P150, RS-P160, RS-P170, RS-P190, RS-P200; every exposed "
            "member lies in the plane X = 6 m, so its full length is normal to the wind; RS-P150 takes its S270 "
            "section's diameter in both systems",
    "hydrotest": "W of every member except RS-P180 with each section's contents density replaced by 1000 kg/m^3, "
                 "once; rigid-family component weights unchanged at their centres of gravity in place of their "
                 "members' distributed weight (§8.1, §8.3)",
    "arcs": "quarter-circle arcs of length R pi/2 carry mass and wind per length like any member (§3, §5)",
}


SPEC_OBSERVATIONS = [
    "sense of a positive g-factor: taken along the positive global axis; the opposite sense (for Z, along gravity) "
    "reverses every seismic force and moment",
    "RS-P150 carries section S270 (§3.2) under the reducer RS-R1 (270 to 220 mm): its wind diameter, and its "
    "distributed seismic load in the fixture today, use S270",
]

def build():
    CROSS.clear()
    check_transcription()
    check_geometry()
    check_sections()

    # Consistency with the existing package: RS-SUS on today's fixture with the operating contents.
    (w, wfm), _ = test_weight("fixture_today", rho_c=None)
    f, m = weight_resultant(w, wfm)
    existing = json.loads(EXISTING.read_text(encoding="utf-8"))["key_values"]
    ok_w = parse_exact(existing["W_total_N"]["exact"]) == w and existing["W_total_N"]["value"] == w.record()["value"]
    ok_m = all(parse_exact(e["exact"]) == x and e["value"] == x.record()["value"]
               for e, x in zip(existing["weight_moment_about_origin_N_m"], m))
    cross_check(f"RS-SUS (fixture today, operating contents): W_total equals {EXISTING_REL} exactly", ok_w)
    cross_check(f"RS-SUS (fixture today, operating contents): weight moment about the origin equals {EXISTING_REL} "
                "exactly", ok_m)
    consistency = {"RS-SUS_fixture_today_W_total_N": w.record(), "RS-SUS_fixture_today_weight_moment_N_m": vrec(m),
                   "agrees_exactly_with": EXISTING_REL if ok_w and ok_m else None}
    (wf, wffm), _ = test_weight("full", rho_c=None)
    context = {"RS-SUS_full_W_total_N": wf.record(),
               "RS-SUS_full_weight_moment_about_origin_N_m": vrec(weight_resultant(wf, wffm)[1]),
               "note": "operating contents, full system (RS-R1 in place): not a pending-case reference; the weight "
                       "the existing package's EQ-1 and EQ-2 move to when RS-R1 lands"}

    # Second routes for the totals.
    for system in SYSTEMS:
        (sw, swfm), rows = seismic_weight(system)
        comp_of = mapped(system)
        alt = X()
        for sid in SECTIONS:
            Ls = X()
            for pid, _, _, s in MEMBERS:
                if s == sid and pid != REPLACED_SPAN and pid not in comp_of:
                    for L, _ in segments(pid, "arc"):
                        Ls = Ls + L
            alt = alt + Ls * X.q(section(sid)["total"] * G, 1)
        alt = alt + X.q(sum(F(RIGID[c][2]) for c in comp_of.values()))
        cross_check(f"{system}: seismic weight equals per-section length sums of unmapped members times weight per "
                    f"length, plus component weights", alt == sw)
        # EQ-6 as decided: at the assumed seismic gravity (the weight case's), the seismic weight equals the weight the
        # weight case gives the model (operating contents), computed by the separate weight routine, total and first
        # moment.
        (wop, wopfm), _ = test_weight(system, rho_c=None)
        cross_check(f"{system}: seismic weight and its first moment equal the weight case's (operating contents)",
                    sw == wop and swfm == wopfm)
        (w, _), _ = test_weight(system)
        alt = X()
        for sid in SECTIONS:
            Ls = X()
            for pid, _, _, s in MEMBERS:
                if s == sid and pid != REPLACED_SPAN and pid not in comp_of:
                    for L, _ in segments(pid, "arc"):
                        Ls = Ls + L
            alt = alt + Ls * X.q(section(sid, RHO_TEST)["total"] * G, 1)
        alt = alt + X.q(sum(F(RIGID[c][2]) for c in comp_of.values()))
        cross_check(f"{system}: RS-HYD weight equals per-section length sums times test weight per length, plus "
                    f"component weights", alt == w)
    (wt, _), rows = wind_load()
    alt = X()
    for sid in ("S270", "S220"):
        Ls = X()
        for pid in exposed_members():
            if MEMBER[pid][3] == sid:
                for L, _ in segments(pid, "arc"):
                    Ls = Ls + L
        alt = alt + Ls * X.q(WIND["pressure_Pa"] * WIND["shape_factor"] * section(sid)["projected_diameter"])
    cross_check("wind force equals per-section exposed length sums times p * C_s * (OD + 2 t_i)", alt == wt)
    # Every exposed member lies in the plane X = 6 m, so the wind (along X) is normal to its whole length.
    cross_check("every exposed member lies in the plane X = 6 m (normal to the wind along its whole length)",
                all(P(MEMBER[p][1])[0] == 6 and P(MEMBER[p][2])[0] == 6 for p in exposed_members()))
    # Seismic moment identity: the Z-case moment is the weight-type moment of the same mass, scaled.
    values = {s: case_values(s) for s in SYSTEMS}
    for s in SYSTEMS:
        _, mz = values[s]["RS-EQZ"]
        _, mx = values[s]["RS-EQX"]
        cross_check(f"{s}: RS-EQZ moment has no Z component and RS-EQX moment no X component",
                    mz[2] == X() and mx[0] == X())
    # Wind does not see RS-R1: the two systems agree.
    cross_check("RS-WIN: full and fixture-today systems agree",
                values["full"]["RS-WIN"] == values["fixture_today"]["RS-WIN"])
    # Seismic differs between the systems by the reducer alone: the full system replaces RS-P150's distributed
    # load by RS-R1's weight at its centre of gravity.
    q150, fm150 = line_items("RS-P150", X.q(section("S270")["total"] * G, 1), "arc")
    w_r1, pos_r1 = F(RIGID["RS-R1"][2]), component_position("RS-R1")
    d_w, d_fm = X.q(w_r1) - q150, [X.q(w_r1 * pos_r1[i]) - fm150[i] for i in range(3)]
    ok = True
    for case, (a, axis) in SEISMIC.items():
        f, m = resultant_along(d_w * a, [x * a for x in d_fm], axis)
        ff, mf = values["full"][case]
        ft, mt = values["fixture_today"][case]
        ok = ok and all(ff[i] - ft[i] == f[i] and mf[i] - mt[i] == m[i] for i in range(3))
    cross_check("RS-EQX, RS-EQY, RS-EQZ: full minus fixture-today equals a times (RS-R1's weight at its centre of "
                "gravity less RS-P150's distributed weight), force and moment", ok)
    check_srss_rounding()

    key = {s: case_records(values[s]) for s in SYSTEMS}
    for s in SYSTEMS:
        key[s]["total_seismic_mass_kg"] = values[s]["_mass"].record()

    controls = []
    for text, change in SEISMIC_CONTROLS:
        per_system = {}
        for s in SYSTEMS:
            v = case_values(s, change)
            rec = case_records(v)
            per_system[s] = {c: {k: rec[c][k] for k in rec[c] if not k.startswith("expected")}
                             for c in ("RS-EQX", "RS-EQY", "RS-EQZ")}
            per_system[s]["total_seismic_mass_kg"] = v["_mass"].record()
        controls.append({"control": text, "values": per_system})

    departures = []
    for oid, subject, kind, specified, alts, cases in SPECIFICATION_DEPARTURE_CONTROLS:
        entry = {"id": oid, "subject": subject, "kind": kind, "specified": specified, "controls": []}
        for text, change in alts:
            per_system = {}
            for s in SYSTEMS:
                v = case_values(s, change)
                rec = case_records(v)
                changed = [c for c in cases if v[c] != values[s][c]]  # only the cases the reading moves
                per_system[s] = {c: {k: rec[c][k] for k in rec[c] if not k.startswith("expected")} for c in changed}
                if v["_mass"] != values[s]["_mass"]:
                    per_system[s]["total_seismic_mass_kg"] = v["_mass"].record()
            entry["controls"].append({"control": text, "values": per_system})
        departures.append(entry)

    return {"key": key, "departures": departures, "controls": controls, "consistency": consistency,
            "context": context, "tables": tables()}


def tables():
    sections = []
    for sid in SECTIONS:
        op, te = section(sid), section(sid, RHO_TEST)
        sections.append({
            "section": f"RS-{sid}",
            "mass_per_length_kg_m": {"metal": X.q(op["parts"]["metal"], 1).record(),
                                     "contents_operating_820": X.q(op["parts"]["contents"], 1).record(),
                                     "contents_test_1000": X.q(te["parts"]["contents"], 1).record(),
                                     "insulation": X.q(op["parts"]["insulation"], 1).record(),
                                     "total_operating": X.q(op["total"], 1).record(),
                                     "total_test": X.q(te["total"], 1).record()},
            "projected_diameter_m": X.q(op["projected_diameter"]).record(),
            "wind_load_per_length_N_m": X.q(WIND["pressure_Pa"] * WIND["shape_factor"]
                                            * op["projected_diameter"]).record()})
    exposed = set(exposed_members())
    full_map, today_map = mapped("full"), mapped("fixture_today")
    members = []
    for pid, frm, to, sid in MEMBERS:
        L = sum((seg[0] for seg in segments(pid, "arc")), X())
        row = {"member": pid, "from": f"RS-{frm}", "to": f"RS-{to}", "section": f"RS-{sid}",
               "kind": "arc" if pid in ARCS else "straight", "length_m": L.record(),
               "wind_exposed": pid in exposed,
               "seismic": ("none: replaced span" if pid == REPLACED_SPAN else
                           f"replaced by a * W of {full_map[pid]} at its centre of gravity (both systems)"
                           if pid in today_map else
                           f"replaced by a * W of {full_map[pid]} at its centre of gravity (full system); a * g * mu "
                           f"(fixture today)" if pid in full_map else "a * g * mu"),
               "hydrotest_weight": ("none: replaced span" if pid == REPLACED_SPAN else
                                    f"replaced by {full_map[pid]} (both systems)" if pid in today_map else
                                    f"replaced by {full_map[pid]} (full system); distributed (fixture today)"
                                    if pid in full_map else "distributed, test contents")}
        if pid in ARCS:
            row["realizes"] = ARCS[pid][0]
        members.append(row)
    comps = []
    for cid, (pid, node, w, offset) in RIGID.items():
        comps.append({"component": cid, "maps": pid, "node": f"RS-{node}", "weight_N": w,
                      "centre_of_gravity_m": [str(x) for x in component_position(cid)],
                      "systems": "full" if cid == "RS-R1" else "full, fixture_today"})
    return {"sections": sections, "members": members, "components": comps}


# ============================================================================ EQ-4: DEL-05-02's rounding, as a helper
def rn(q):
    """Round an exact rational to the nearest binary64, ties to even (Python's int/int division is correctly
    rounded)."""
    q = F(q)
    return q.numerator / q.denominator


def rn_sqrt(s):
    """The binary64 nearest sqrt(s) for an exact non-negative rational s, ties to even."""
    s = F(s)
    if s == 0:
        return 0.0
    # A start within a few units in the last place, from the integer square root at about 70 bits.
    a, b = s.numerator, s.denominator
    k = 70 - (a.bit_length() - b.bit_length()) // 2
    scaled = a * 4 ** k // b if k >= 0 else a // (b * 4 ** (-k))
    d = float(F(isqrt(scaled)) / F(2) ** k)
    while True:
        lo_mid = (F(math.nextafter(d, 0.0)) + F(d)) / 2
        hi_mid = (F(d) + F(math.nextafter(d, math.inf))) / 2
        if lo_mid * lo_mid > s:
            d = math.nextafter(d, 0.0)
        elif hi_mid * hi_mid < s:
            d = math.nextafter(d, math.inf)
        else:
            if lo_mid * lo_mid == s or hi_mid * hi_mid == s:  # a tie: take the neighbour with an even significand
                other = math.nextafter(d, 0.0) if lo_mid * lo_mid == s else math.nextafter(d, math.inf)
                if struct.unpack(">Q", struct.pack(">d", d))[0] & 1:
                    d = other
            return d


def absolute_sum(operands, factors=None):
    """DEL-05-02: A = RN(sum |f_i x_i|), the exact sum rounded once."""
    factors = factors or [1] * len(operands)
    return rn(sum(abs(F(f) * F(x)) for f, x in zip(factors, operands)))


def srss(operands, factors=None):
    """DEL-05-02: z_i = RN(f_i x_i), R = RN(sqrt(sum z_i^2)), the square root of the exact sum, correctly rounded."""
    factors = factors or [1] * len(operands)
    z = [F(rn(F(f) * F(x))) for f, x in zip(factors, operands)]
    return rn_sqrt(sum(v * v for v in z))


def check_srss_rounding():
    cases = [(3.0, 4.0, 0.0), (1.0, 1.0, 1.0), (-0.1, 0.2, -0.3), (1e-300, 1e-300, 0.0), (1e300, 1e300, 1e300),
             (12345.678, -9876.54321, 0.000123)]
    ok = True
    for c in cases:
        r, a = srss(c), absolute_sum(c)
        s = sum(F(x) * F(x) for x in c)
        # Correct rounding: no binary64 neighbour of r is nearer to sqrt(s).
        for nb in (math.nextafter(r, 0.0), math.nextafter(r, math.inf)):
            # |nb - sqrt s| < |r - sqrt s|  <=>  (nb + r)/2 lies strictly between ... compare squares of midpoints
            mid = (F(nb) + F(r)) / 2
            ok = ok and ((nb < r and mid * mid <= s) or (nb > r and mid * mid >= s))
        ok = ok and r <= a and a == rn(sum(abs(F(x)) for x in c))
    ok = ok and srss((3.0, 4.0, 0.0)) == 5.0 and srss((1.0, 1.0, 1.0)) == math.sqrt(3.0)
    cross_check("EQ-4 helper: SRSS rounds the exact square root once and correctly, absolute sum the exact sum once, "
                "and SRSS <= ABS, on six invented operand triples", ok)


# ============================================================================ output
def document(result):
    return {
        "document": "swbpipe.reference_system_pending_case_references",
        "status": "candidate frozen references; derived and cross-checked by the author TASK, not yet independently "
                  "refuted; repaired after one independent refutation (the decided seismic rule made the expected "
                  "path); after a second refutation the former open readings OR-2 to OR-5 are relabelled as controls for "
                  "departures from the specification (README); the one open reading is the seismic gravity, and the "
                  "seismic values are "
                  "conditional on the stated assumption g = 9.80665 m/s^2",
        "specification": "DEL-09-01 reference-system specification §9.2: EQ-6 for RS-WIN, RS-EQX, RS-EQY, RS-EQZ; "
                         "EQ-1 and EQ-2 for RS-HYD; EQ-4 is an identity stated in the README",
        "inputs": f"{SPEC_REL}, transcribed in the generator and checked against the file on every run; no product "
                  f"file and no fixture model file is read; {EXISTING_REL} is read only for the consistency check",
        "units": {"force": "N", "moment": "N*m", "length": "m", "mass": "kg", "mass_per_length": "kg/m"},
        "gravity": {"value": str(G), "unit": "m/s^2", "weight_direction": "-Z"},
        "origin": "global (0, 0, 0) m; moments are sum of r x F about it",
        "systems": SYSTEMS,
        "bases": BASES,
        "seismic_rule": SEISMIC_RULE,
        "specification_observations": SPEC_OBSERVATIONS,
        "checks": {
            "EQ-6": "the reaction force resultant of RS-dWIN and its moment about the origin (sum over supports of "
                    "r_node x F + M) equal minus the generated wind force and moment; for each of RS-dEQX, RS-dEQY, "
                    "RS-dEQZ, the reaction resultant and its moment about the origin equal minus the g-factor times "
                    "the weight the same gravity gives the model, component weights included (key_values: the "
                    "expected_RS-d* entries)",
            "EQ-1 (RS-HYD)": "the sum of the vertical (Z) reaction components over every support, held supports "
                             "included, equals W_total",
            "EQ-2 (RS-HYD)": "the horizontal (X, Y) reaction resultant is zero and the reaction moment resultant "
                             "about the origin equals minus weight_moment_about_origin",
        },
        "tolerance": "not set here: the consumer applies the criterion it adopts",
        "crosscheck": list(CROSS),
        "key_values": result["key"],
        "seismic_controls": result["controls"],
        "open_readings": OPEN_READINGS,
        "specification_departure_controls": result["departures"],
        "consistency": result["consistency"],
        "context": result["context"],
        "sections": result["tables"]["sections"],
        "members": result["tables"]["members"],
        "components": result["tables"]["components"],
    }


def dumps(obj):
    def inline(o):
        return json.dumps(o, ensure_ascii=False, separators=(", ", ": "))

    def emit(o, level):
        pad = " " * level
        if isinstance(o, dict) and ("exact" in o or "check" in o):
            return inline(o)
        if isinstance(o, dict):
            items = [f"{pad} {json.dumps(k, ensure_ascii=False)}: {emit(v, level + 1)}" for k, v in o.items()]
            return "{\n" + ",\n".join(items) + "\n" + pad + "}" if items else "{}"
        if isinstance(o, list) and o and all(isinstance(x, dict) for x in o):
            items = [f"{pad} {emit(v, level + 1)}" for v in o]
            return "[\n" + ",\n".join(items) + "\n" + pad + "]"
        return inline(o)
    return emit(obj, 0) + "\n"


def sha256sums(files):
    return ("".join(f"{hashlib.sha256(d).hexdigest()}  {p}\n" for p, d in sorted(files.items()))).encode()


def readme_quotes(result):
    """Every frozen key value's binary64 appears in the README (so its tables cannot drift from the file)."""
    readme = OUT / "README.md"
    if not readme.exists():
        return []
    text = readme.read_text(encoding="utf-8")
    missing = []
    for s, cases in result["key"].items():
        for case, rec in cases.items():
            if case == "total_seismic_mass_kg":
                values = [rec["value"]]
            elif case == "RS-HYD":
                values = [rec["W_total_N"]["value"]] + [v["value"] for v in rec["weight_moment_about_origin_N_m"]]
            else:
                values = [v["value"] for k in ("generated_force_N", "generated_moment_about_origin_N_m")
                          for v in rec[k]]
            missing += [f"{s} {case} {v!r}" for v in values if v != 0 and repr(v) not in text]
    return missing


def build_files():
    result = build()
    missing = readme_quotes(result)
    cross_check("README quotes every nonzero frozen key value's binary64", not missing)
    for m in missing:
        print("  README does not quote", m)
    doc = document(result)
    text = dumps(doc)
    assert json.loads(text) == json.loads(json.dumps(doc))
    files = {"references.json": text.encode()}
    for fixed in FIXED_FILES:
        p = OUT / fixed
        if p.exists():
            files[fixed] = p.read_bytes()
    files["SHA256SUMS"] = sha256sums({k: v for k, v in files.items() if k != "SHA256SUMS"})
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
    for s, cases in result["key"].items():
        for case in ("RS-WIN", "RS-EQX", "RS-EQY", "RS-EQZ"):
            rec = cases[case]
            print(f"{s} {case} F {[v['value'] for v in rec['generated_force_N']]} "
                  f"M {[v['value'] for v in rec['generated_moment_about_origin_N_m']]}")
        h = cases["RS-HYD"]
        print(f"{s} RS-HYD W {h['W_total_N']['value']!r} M {[v['value'] for v in h['weight_moment_about_origin_N_m']]}")
    if args.write:
        if any(not c["agrees"] and not c["check"].startswith("README") for c in CROSS):
            print("not written: a cross-check disagrees")
            return 1
        for path, data in files.items():
            if path in FIXED_FILES:
                continue
            (OUT / path).write_bytes(data)
        print("wrote references.json and SHA256SUMS")
        return 0
    failures = [p for p, d in files.items() if not (OUT / p).exists() or (OUT / p).read_bytes() != d]
    committed = {str(p.relative_to(OUT)) for p in OUT.rglob("*") if p.is_file() and "__pycache__" not in p.parts}
    extra = sorted(committed - set(files))
    if failures or extra or bad:
        print("MISMATCH: " + ", ".join(failures + [f"unexpected {e}" for e in extra]))
        return 1
    print(f"verified {len(files)} files byte for byte (SHA256SUMS included)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
