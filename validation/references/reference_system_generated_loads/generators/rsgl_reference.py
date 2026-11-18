#!/usr/bin/env python3
"""Reference system, per-load and warning references for RS-WIN, RS-EQX, RS-EQY, RS-EQZ (DEL-05-01, DEL-09-01).

Supplements validation/references/reference_system_pending_cases, which remains the reference for EQ-6: the
resultants derived here are checked equal to that package's, exactly.

Standard-library Python 3 only. This generator imports, reads and runs no product code and reads no product
output. Its default check and --write read only this package: this file, the README (for the key-value
quotation check) and staged/reference_system_model_members.json, the reference system model's members `nodes`,
`pipe_segments`, `components` and `pressure_contract` staged from the fixture at a named commit. The staged file
holds no `load_cases`, `combinations`, `analysis_status` or `diagnostics`. --check-live alone opens the live
fixture, keeps the same four members, and compares them with the staged copy leaf by leaf.

Everything else is transcribed below from the governing text, with its source: the case inputs of the
specification's section 8.1, the gravity of section 2, the bend corners of section 3.1, the member order of
section 3.2, and the frozen values of two sibling packages used as cross-checks.

What it derives (DEL-05-01 "Static-equivalent generation", "Mass basis", REQ-008, REQ-015, AC-007; spec §8.1,
§9.2 EQ-6), each value an exact sum of q * pi^k * sqrt(n) terms with q rational:

  wind      on each marked member, q = p * C_s * (D + 2 t_i) per length, along +X, whole span
  seismic   for the entered axis a with g-factor k: on each member that is neither the connector-replaced span
            nor the body of a qualifying rigid-family component, q = k * g * mu per length along +a; at each
            qualifying component's node the force F = k * W along +a and the couple r x F
  resultant force and moment about the global origin, each force taken at its point of action
  twin      the same seismic resultant with every non-replaced member carrying k * g * mu and no component force
  formation the binary64 bits of each component force and moment under the scope's rounding rule

Modes (from the package directory):
  python3 -I generators/rsgl_reference.py              check: rebuild every output in memory, compare byte for byte
  python3 -I generators/rsgl_reference.py --write      rewrite references.json and SHA256SUMS
  python3 -I generators/rsgl_reference.py --check-live compare the staged members with the live fixture's
                                                       (not run by the default check)
"""
import argparse
import hashlib
import json
import math
import struct
import sys
from decimal import Decimal, Context, ROUND_HALF_EVEN, getcontext, localcontext
from fractions import Fraction as Fr
from math import isqrt
from pathlib import Path

HERE = Path(__file__).resolve().parent
PKG = HERE.parent
ROOT = PKG.parents[2]
MODEL_REL = "fixtures/reference_system/reference_system.json"
LIVE_MODEL = ROOT / "fixtures" / "reference_system" / "reference_system.json"  # --check-live only
STAGED_REL = "staged/reference_system_model_members.json"
STAGED = PKG / STAGED_REL
REFERENCES = PKG / "references.json"
SUMS = PKG / "SHA256SUMS"
README = PKG / "README.md"
SELF_REL = "generators/rsgl_reference.py"

KEPT_MEMBERS = ("nodes", "pipe_segments", "components", "pressure_contract")
DROPPED_MEMBERS = ("load_cases", "combinations", "analysis_status", "diagnostics")

CROSS = []
STAGED_META = {}


def cross(name, compares, kind, agrees, detail=None):
    entry = {"check": name, "compares": compares, "kind": kind, "agrees": bool(agrees)}
    if detail is not None:
        entry["detail"] = detail
    CROSS.append(entry)


# ---------------------------------------------------------------------------------------------------------------
# Transcribed governing text
# ---------------------------------------------------------------------------------------------------------------

SPEC = {
    "gravity_weight": "§2: Gravity is entered on each weight case as `global_z`, value −9.80665 m/s²",
    "seismic": ("§8.1: Each static seismic case, RS-EQX, RS-EQY and RS-EQZ, enters a gravity acceleration of "
                "9.80665 m/s² as a positive magnitude and one factor on its own axis: `g_factor_x` 0.30, "
                "`g_factor_y` 0.30 and `g_factor_z` 0.20 respectively, each positive. The factor's sign is the "
                "direction's sign."),
    "wind": ("§8.1: RS-WIN | W + P + wind: 0.9 kPa, shape factor 0.7, `global_x`, exposed members RS-P070 … "
             "RS-P200 except the replaced span RS-P180"),
    "replaced": ("§5.1: The replaced span RS-P180 carries no load, member result or self-weight. It is excluded "
                 "from weight generation and from every primitive load"),
    "rigid": "§5: The valve, flanges and reducer are rigid-family components (`mechanics_geometry_only`).",
    "identifier": ("§8.1: `load:self-weight:<n>:<case id>:<m>:<pipe id>` for a member's line load, and "
                   "`load:self-weight:<n>:<case id>:component:<m>:<component id>:force:z` or `…:moment:<axis>` "
                   "for a component's weight, where `<n>` and `<m>` are the character counts of the id that "
                   "follows each."),
    "w_inventory": ("§8.1: W is generated self-weight of every member except RS-P180 (gravity as §2), with "
                    "component weights at their centres of gravity in place of their members' distributed "
                    "weight: 21 line loads on the straight and arc members other than RS-P020, RS-P030, RS-P040 "
                    "and RS-P150, and a force and a moment for each of RS-F1, RS-V1, RS-F2 and RS-R1, 29 "
                    "primitives per case."),
    "added_weight": "§8: no case carries condition `added_weight`.",
}
G_WEIGHT = Fr("9.80665")          # §2, magnitude; the weight cases enter it along -Z
G_SEISMIC = Fr("9.80665")         # §8.1, positive magnitude
SEISMIC_CASES = (("RS-EQX", "x", Fr("0.30")), ("RS-EQY", "y", Fr("0.30")), ("RS-EQZ", "z", Fr("0.20")))
WIND_PRESSURE_PA = Fr("0.9") * 1000
WIND_SHAPE = Fr("0.7")
WIND_AXIS = "x"
WIND_FIRST, WIND_LAST, WIND_EXCEPT = "pipe:RS-P070", "pipe:RS-P200", "pipe:RS-P180"
WIND_EXPECTED = tuple("pipe:RS-P%s" % s for s in (
    "070", "080", "085", "090", "100", "110", "120", "130", "140", "150", "160", "170", "190", "200"))
SPEC_MEMBER_ORDER = tuple("pipe:RS-P%s" % s for s in (
    "010", "020", "030", "040", "050", "060", "065", "070", "080", "085", "090", "100", "110", "120", "130",
    "140", "150", "160", "170", "180", "190", "200", "210", "220", "225", "230"))
SPEC_CORNERS = {"component:RS-B1": (6, 0, 1), "component:RS-B2": (6, 0, 7), "component:RS-B3": (6, 15, 2)}
SPEC_RIGID = ("component:RS-F1", "component:RS-V1", "component:RS-F2", "component:RS-R1")
RIGID_KINDS = ("valve", "flange", "reducer", "rigid", "specialty")   # DEL-03-05 "Rigid-family component"
MASS_BASIS_EX = "source_od_nominal_wall_pressure_bore_areas/v1"

# Frozen sibling values, transcribed (the maintained test compares each with the committed file it names).
SIBLING = {
    "r1_ce": {
        "file": "validation/references/reference_system_r1_ce/references.json",
        "W_total_N": "(3600) + (66155686593423/5000000000)*pi + (124534361799687/800000000000)*pi^2",
        "weight_first_moment_N_m": [
            "(4605) + (240668557483367167/3200000000000)*pi + (2917059564163647/3200000000000)*pi^2",
            "(6060) + (689381386488375721/5000000000000)*pi + (253997473516297/800000000000)*pi^2",
            "(24705/4) + (1192422469821139207/16000000000000)*pi + (1884196126085903/3200000000000)*pi^2"],
        "weight_moment_about_origin_N_m": [
            "(-6060) + (-689381386488375721/5000000000000)*pi + (-253997473516297/800000000000)*pi^2",
            "(4605) + (240668557483367167/3200000000000)*pi + (2917059564163647/3200000000000)*pi^2",
            "0"],
    },
    "tranche1": {
        "file": "validation/references/reference_system/references.json",
        "W_total_N": ("(3300) + (66155686593423/5000000000)*pi + (1363190839087/160000000000)*pi*sqrt(257) + "
                      "(124534361799687/800000000000)*pi^2"),
    },
    "pending_control_before_2026_11_12": {
        "file": "validation/references/reference_system_pending_cases/references.json",
        "control": 'control: the rule before 2026-11-12: pipe mass on a mapped member, component weight left out',
        'RS-EQX': (['(1624542630897501/400000000000)*pi + (4089572517261/1600000000000)*pi*sqrt(257) + (373603085399061/8000000000000)*pi^2', '0', '0'],
                   ['0', '(3591989870525557221/160000000000000)*pi + (2286071037148899/128000000000000)*pi*sqrt(257) + (5652588378257709/32000000000000)*pi^2', '(-2068144159465127163/50000000000000)*pi + (-413046824243361/8000000000000)*pi*sqrt(257) + (-761992420548891/8000000000000)*pi^2']),
        'RS-EQY': (['0', '(1624542630897501/400000000000)*pi + (4089572517261/1600000000000)*pi*sqrt(257) + (373603085399061/8000000000000)*pi^2', '0'],
                   ['(-3591989870525557221/160000000000000)*pi + (-2286071037148899/128000000000000)*pi*sqrt(257) + (-5652588378257709/32000000000000)*pi^2', '0', '(724508490830665233/32000000000000)*pi + (12268717551783/800000000000)*pi*sqrt(257) + (8751178692490941/32000000000000)*pi^2']),
        'RS-EQZ': (['0', '0', '(541514210299167/200000000000)*pi + (1363190839087/800000000000)*pi*sqrt(257) + (124534361799687/4000000000000)*pi^2'],
                   ['(689381386488375721/25000000000000)*pi + (137682274747787/4000000000000)*pi*sqrt(257) + (253997473516297/4000000000000)*pi^2', '(-241502830276888411/16000000000000)*pi + (-4089572517261/400000000000)*pi*sqrt(257) + (-2917059564163647/16000000000000)*pi^2', '0']),
    },
    "pending_full": {
        "file": "validation/references/reference_system_pending_cases/references.json",
        "RS-WIN": (["(200529/25) + (2457/400)*sqrt(257) + (2457/50)*pi", "0", "0"],
                   ["0", "(52306317/1000) + (1373463/32000)*sqrt(257) + (81081/250)*pi",
                    "(-4882437/50) + (-248157/2000)*sqrt(257) + (-2457/125)*pi"]),
        "RS-EQX": (["(1080) + (198467059780269/50000000000)*pi + (373603085399061/8000000000000)*pi^2", "0", "0"],
                   ["0", "(14823/8) + (3577267409463417621/160000000000000)*pi + "
                         "(5652588378257709/32000000000000)*pi^2",
                    "(-1818) + (-2068144159465127163/50000000000000)*pi + (-761992420548891/8000000000000)*pi^2"]),
        "RS-EQY": (["0", "(1080) + (198467059780269/50000000000)*pi + (373603085399061/8000000000000)*pi^2", "0"],
                   ["(-14823/8) + (-3577267409463417621/160000000000000)*pi + "
                    "(-5652588378257709/32000000000000)*pi^2", "0",
                    "(2763/2) + (722005672450101501/32000000000000)*pi + (8751178692490941/32000000000000)*pi^2"]),
        "RS-EQZ": (["0", "0", "(720) + (66155686593423/25000000000)*pi + (124534361799687/4000000000000)*pi^2"],
                   ["(1212) + (689381386488375721/25000000000000)*pi + (253997473516297/4000000000000)*pi^2",
                    "(-921) + (-240668557483367167/16000000000000)*pi + (-2917059564163647/16000000000000)*pi^2",
                    "0"]),
    },
}

# ---------------------------------------------------------------------------------------------------------------
# Exact sums of q * pi^k * sqrt(n)
# ---------------------------------------------------------------------------------------------------------------


def _squarefree(n):
    """n = s^2 * m with m squarefree; returns (s, m)."""
    s, m, p = 1, n, 2
    while p * p <= m:
        while m % (p * p) == 0:
            m //= p * p
            s *= p
        p += 1
    return s, m


class X:
    """An exact sum of q * pi^k * sqrt(n), q rational, n squarefree."""
    __slots__ = ("t",)

    def __init__(self, t=None):
        self.t = {key: q for key, q in (t or {}).items() if q != 0}

    @staticmethod
    def c(q):
        return X({(0, 1): Fr(q)})

    @staticmethod
    def sqrt_of(q):
        q = Fr(q)
        assert q >= 0
        num = q.numerator * q.denominator
        s, m = _squarefree(num)
        return X({(0, m): Fr(s, q.denominator)})

    def __add__(self, o):
        o = o if isinstance(o, X) else X.c(o)
        t = dict(self.t)
        for key, q in o.t.items():
            t[key] = t.get(key, 0) + q
        return X(t)

    __radd__ = __add__

    def __neg__(self):
        return X({key: -q for key, q in self.t.items()})

    def __sub__(self, o):
        return self + (-(o if isinstance(o, X) else X.c(o)))

    def __mul__(self, o):
        if not isinstance(o, X):
            return X({key: q * Fr(o) for key, q in self.t.items()})
        t = {}
        for (k1, n1), q1 in self.t.items():
            for (k2, n2), q2 in o.t.items():
                s, m = _squarefree(n1 * n2)
                key = (k1 + k2, m)
                t[key] = t.get(key, 0) + q1 * q2 * s
        return X(t)

    __rmul__ = __mul__

    def is_zero(self):
        return not self.t

    def __eq__(self, o):
        o = o if isinstance(o, X) else X.c(o)
        return self.t == o.t

    def __hash__(self):
        return hash(tuple(sorted(self.t.items())))

    def text(self):
        if not self.t:
            return "0"
        parts = []
        for (k, n) in sorted(self.t):
            q = self.t[(k, n)]
            s = "(%s)" % q
            if n != 1:
                s += "*sqrt(%d)" % n
            if k == 1:
                s += "*pi"
            elif k > 1:
                s += "*pi^%d" % k
            parts.append(s)
        return " + ".join(parts)

    @staticmethod
    def parse(text):
        if text == "0":
            return X()
        t = {}
        for part in text.split(" + "):
            assert part.startswith("("), part
            close = part.index(")")
            q = Fr(part[1:close])
            rest = part[close + 1:]
            k, n = 0, 1
            for factor in [f for f in rest.split("*") if f]:
                if factor == "pi":
                    k += 1
                elif factor.startswith("pi^"):
                    k += int(factor[3:])
                elif factor.startswith("sqrt("):
                    n *= int(factor[5:-1])
                else:
                    raise ValueError(part)
            t[(k, n)] = t.get((k, n), 0) + q
        return X(t)


ZERO = X()

# Bounds: pi by Machin's formula, sqrt(n) by the integer square root, each to 10^-110.
_DIGITS = 110
_SCALE = 10 ** _DIGITS


def _arctan_inv(x, scale):
    total, term, n, sign = 0, scale // x, 1, 1
    x2 = x * x
    while term:
        total += sign * (term // n)
        term //= x2
        n += 2
        sign = -sign
    return total


def _pi_bounds():
    guard = 10 ** 10
    scale = _SCALE * guard
    approx = 4 * (4 * _arctan_inv(5, scale) - _arctan_inv(239, scale))
    return Fr(approx - 1000, scale), Fr(approx + 1000, scale)


PI_LO, PI_HI = _pi_bounds()


def _sqrt_bounds(n):
    r = isqrt(n * _SCALE * _SCALE)
    return Fr(r, _SCALE), Fr(r + 1, _SCALE)


def interval(x):
    lo = hi = Fr(0)
    for (k, n), q in x.t.items():
        sl, sh = _sqrt_bounds(n) if n != 1 else (Fr(1), Fr(1))
        ml, mh = PI_LO ** k * sl, PI_HI ** k * sh
        if q >= 0:
            lo += q * ml
            hi += q * mh
        else:
            lo += q * mh
            hi += q * ml
    return lo, hi


def rn(q):
    """Round a rational to the nearest binary64, ties to even (written here; not float())."""
    q = Fr(q)
    if q == 0:
        return 0.0
    sign = -1.0 if q < 0 else 1.0
    a = abs(q)
    e = a.numerator.bit_length() - a.denominator.bit_length()
    if Fr(2) ** e > a:
        e -= 1
    while Fr(2) ** (e + 1) <= a:
        e += 1
    e = max(e, -1022)
    m = a * Fr(2) ** (52 - e)
    n = m.numerator // m.denominator
    rem = m - n
    if rem > Fr(1, 2) or (rem == Fr(1, 2) and n % 2 == 1):
        n += 1
    if n == 2 ** 53:
        n //= 2
        e += 1
    if e > 1023:
        return sign * math.inf
    return sign * math.ldexp(n, e - 52)


def dec40(q):
    ctx = Context(prec=40, rounding=ROUND_HALF_EVEN)
    return ctx.divide(Decimal(q.numerator), Decimal(q.denominator))


def fmt_dec(d):
    return format(d, ".39e") if d != 0 else "0"


def bits(v):
    return struct.pack(">d", v).hex()


def num(x):
    """{exact, decimal, value} of an exact sum; decimal and value only when both interval ends agree."""
    x = x if isinstance(x, X) else X.c(x)
    if x.is_zero():
        return {"exact": "0", "decimal": "0", "value": 0.0}
    lo, hi = interval(x)
    dl, dh = dec40(lo), dec40(hi)
    vl, vh = rn(lo), rn(hi)
    assert dl == dh and vl == vh, x.text()
    return {"exact": x.text(), "decimal": fmt_dec(dl), "value": vl}


def value(x):
    return num(x)["value"]


def vec(v):
    return [num(c) for c in v]


def vadd(a, b):
    return [a[i] + b[i] for i in range(3)]


def vneg(a):
    return [-c for c in a]


def vscale(a, s):
    return [c * s for c in a]


def cross3(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


AXES = ("x", "y", "z")


def unit(axis):
    return [X.c(1) if a == axis else ZERO for a in AXES]


# ---------------------------------------------------------------------------------------------------------------
# The model members read
# ---------------------------------------------------------------------------------------------------------------


def read_staged():
    staged = json.loads(STAGED.read_text(encoding="utf-8"), parse_float=Fr, parse_int=Fr)
    assert staged["source"] == MODEL_REL
    model = staged["members"]
    assert set(model) == set(KEPT_MEMBERS), sorted(model)
    return staged, model


def read_model():
    return read_staged()[1]


def read_live_members():
    """The live fixture's four kept members; the others are dropped as the top-level object is built."""
    def keep(pairs):
        d = dict(pairs)
        if all(k in d for k in KEPT_MEMBERS) and "schema_version" in d:
            return {k: d[k] for k in KEPT_MEMBERS}
        return d
    model = json.loads(LIVE_MODEL.read_text(encoding="utf-8"), parse_float=Fr, parse_int=Fr, object_pairs_hook=keep)
    assert set(model) == set(KEPT_MEMBERS), sorted(model)
    return model


def leaves(obj, path=""):
    """Every leaf keyed by its path, records in a list keyed by their id; a number as ('number', exact),
    a boolean, string or null by its type and value."""
    out = {}
    if isinstance(obj, dict):
        for k, v in obj.items():
            out.update(leaves(v, f"{path}.{k}" if path else k))
    elif isinstance(obj, list):
        ids = [x.get("id") if isinstance(x, dict) else None for x in obj]
        keyed = all(ids) and len(set(ids)) == len(ids)
        for i, v in enumerate(obj):
            out.update(leaves(v, f"{path}[{ids[i] if keyed else i}]"))
        out[path + ".<length>"] = ("length", len(obj))
    elif isinstance(obj, Fr):
        out[path] = ("number", obj)
    else:
        out[path] = (type(obj).__name__, obj)
    return out


def check_live():
    staged = leaves(read_model())
    now = leaves(read_live_members())
    differ = sorted(k for k in set(staged) | set(now) if staged.get(k, "<absent>") != now.get(k, "<absent>"))
    for k in differ:
        print(f"DIFFERS: {MODEL_REL} {k}: staged {staged.get(k, '<absent>')!r}, live {now.get(k, '<absent>')!r}")
    print(f"live fixture {MODEL_REL}: {len(staged)} staged leaves of {', '.join(KEPT_MEMBERS)} compared, "
          f"{len(differ)} differ")
    if differ:
        print("SOURCE MOVED ON: the live fixture's model members no longer match the staged input in the leaves named "
              "above; a decision is needed on a successor package or a regeneration. The committed output stands for "
              "its staged input.")
    return 1 if differ else 0


def qty(q, unit_text):
    assert q["unit"] == unit_text, (q, unit_text)
    return Fr(q["value"])


def build_inputs(model):
    nodes = {}
    for n in model["nodes"]:
        p = n["position"]
        nodes[n["id"]] = tuple(Fr(p[a]) for a in AXES)
    comps = {c["id"]: c for c in model["components"]}
    arcs = {}
    replaced = []
    for c in model["components"]:
        if c["kind"] == "bend":
            arcs[c["geometry"]["bend_pipe_ref"]] = (c["id"], qty(c["geometry"]["bend_radius"], "m"))
        oc = c.get("objective_connector")
        if oc and oc["topology"]["type"] == "replaces_span":
            replaced.append(oc["topology"]["span_ref"])
    members = []
    for p in model["pipe_segments"]:
        s = p["section"]
        sec = {
            "D": qty(s["outside_diameter"], "mm") / 1000, "t": qty(s["wall_thickness"], "mm") / 1000,
            "c_m": qty(s["mill_tolerance"], "mm") / 1000, "c_a": qty(s["corrosion_allowance"], "mm") / 1000,
            "t_i": qty(s["insulation_thickness"], "mm") / 1000,
            "rho_m": qty(s["material_density"], "kg/m^3"), "rho_c": qty(s["contents_density"], "kg/m^3"),
            "rho_i": qty(s["insulation_density"], "kg/m^3"),
        }
        members.append({"id": p["id"], "from": p["from"], "to": p["to"], "section_ref": p["section_ref"],
                        "section": sec, "arc": arcs.get(p["id"])})
    return nodes, members, comps, replaced


# ---------------------------------------------------------------------------------------------------------------
# Route A: closed forms
# ---------------------------------------------------------------------------------------------------------------


def mu_parts(s):
    """Mass per length on the exact route's basis, radius form; each part's rational coefficient of pi."""
    R = s["D"] / 2
    metal = s["rho_m"] * (R ** 2 - (R - s["t"]) ** 2)
    contents = s["rho_c"] * (R - ((s["t"] - s["c_m"]) - s["c_a"])) ** 2
    insulation = s["rho_i"] * ((R + s["t_i"]) ** 2 - R ** 2)
    return metal, contents, insulation


def mu_diameter_form(s):
    """The same mass per length in diameter form (pi/4 times differences of squared diameters)."""
    D, t = s["D"], s["t"]
    d_bore = D - 2 * ((t - s["c_m"]) - s["c_a"])
    return (s["rho_m"] * (D ** 2 - (D - 2 * t) ** 2) + s["rho_c"] * d_bore ** 2
            + s["rho_i"] * ((D + 2 * s["t_i"]) ** 2 - D ** 2)) / 4


def member_geometry(m, nodes):
    """Length and first moment (integral of r ds) of a member, exact."""
    a, b = nodes[m["from"]], nodes[m["to"]]
    if m["arc"] is None:
        L = X.sqrt_of(sum((b[i] - a[i]) ** 2 for i in range(3)))
        first = [L * ((a[i] + b[i]) / 2) for i in range(3)]
        return {"kind": "straight", "length": L, "first_moment": first}
    comp_id, R = m["arc"]
    corner = tuple(Fr(c) for c in SPEC_CORNERS[comp_id])
    C = tuple(a[i] + b[i] - corner[i] for i in range(3))
    e1 = tuple(a[i] - C[i] for i in range(3))
    e2 = tuple(b[i] - C[i] for i in range(3))
    assert sum(e1[i] ** 2 for i in range(3)) == R ** 2 == sum(e2[i] ** 2 for i in range(3))
    assert sum(e1[i] * e2[i] for i in range(3)) == 0
    half_pi = X({(1, 1): Fr(1, 2)})
    L = half_pi * R
    first = [(half_pi * C[i] + (e1[i] + e2[i])) * R for i in range(3)]
    return {"kind": "arc", "length": L, "first_moment": first, "radius": R, "centre": C, "e1": e1, "e2": e2,
            "corner": corner}


# ---------------------------------------------------------------------------------------------------------------
# Component qualification (REQ-015's ordered rules as the seismic definition uses them)
# ---------------------------------------------------------------------------------------------------------------


def qualify(comps, members, replaced):
    by_id = {m["id"]: m for m in members}
    mapped = {}
    for c in comps.values():
        ref = c.get("geometry", {}).get("rigid_pipe_ref")
        if c["kind"] in RIGID_KINDS and ref is not None:
            mapped.setdefault(ref, []).append(c["id"])
    out = []
    for c in comps.values():
        if c["kind"] not in RIGID_KINDS:
            continue
        g = c["geometry"]
        ref = g.get("rigid_pipe_ref")
        steps = []
        resolved = ref in by_id and ref not in replaced
        steps.append(["member resolves to a member that is not connector-replaced", resolved])
        conflict = len(mapped.get(ref, [])) > 1
        steps.append(["no rigid-family mapping conflict on the member", not conflict])
        consumption = c.get("mechanics_interface", {}).get("solver_consumption")
        steps.append(["not declared not_solver_consumed", consumption != "not_solver_consumed"])
        cog = g["center_of_gravity"]
        assert cog["unit"] == "m"
        r = tuple(Fr(cog[a]) for a in AXES)
        no_vanishing = all(rn(v) != 0 for v in r if v != 0)
        steps.append(["no nonzero centre-of-gravity coordinate that normalizes to zero", no_vanishing])
        steps.append(["declares mechanics_geometry_only", consumption == "mechanics_geometry_only"])
        m = by_id.get(ref)
        ends = m is not None and c["node"] in (m["from"], m["to"])
        steps.append(["its member ends at the component's node", ends])
        W = qty(g["weight"], "N")
        steps.append(["W normalizes to a finite positive force", math.isfinite(rn(W)) and rn(W) > 0])
        steps.append(["the centre of gravity normalizes to a finite global length vector",
                      all(math.isfinite(rn(v)) for v in r)])
        out.append({"component": c["id"], "kind": c["kind"], "node": c["node"], "member": ref, "W": W, "r": r,
                    "steps": steps, "replaces": all(s[1] for s in steps)})
    return out


# ---------------------------------------------------------------------------------------------------------------
# Binary64 formation of a component force and moment (DEL-05-01 "Static-equivalent generation")
# ---------------------------------------------------------------------------------------------------------------

# For F along axis a, r x F has the two transverse components (axis, sign, coordinate of r):
TRANSVERSE = {
    "x": (("y", 1, "z"), ("z", -1, "y")),
    "y": (("x", -1, "z"), ("z", 1, "x")),
    "z": (("x", 1, "y"), ("y", -1, "x")),
}


def formation(k, W, r, axis):
    kb, Wb = rn(k), rn(W)
    rb = {a: rn(v) for a, v in zip(AXES, r)}
    F = rn(Fr(kb) * Fr(Wb))
    F_float = kb * Wb
    agree = [F == F_float and bits(F) == bits(F_float)]
    force = {"axis": axis, "required_nonzero": k != 0, "exact_on_entered_decimals": num(k * W),
             "binary64": {"value": F, "bits": bits(F), "exact_of_binary64": str(Fr(F))}}
    moments = []
    for m_axis, sign, coord in TRANSVERSE[axis]:
        c = rb[coord]
        M = rn(Fr(sign) * Fr(c) * Fr(F))
        M_float = (c * F) if sign > 0 else -(c * F)
        agree.append(M == M_float and (M == 0 or bits(M) == bits(M_float)))
        exact = Fr(sign) * dict(zip(AXES, r))[coord] * k * W
        moments.append({"axis": m_axis, "formed_as": "%sRN(r_%s * F)" % ("" if sign > 0 else "-", coord),
                        "required_nonzero": k != 0 and dict(zip(AXES, r))[coord] != 0,
                        "exact_on_entered_decimals": num(exact),
                        "binary64": {"value": M, "bits": bits(M), "exact_of_binary64": str(Fr(M))}})
        if M == 0:
            moments[-1]["binary64"]["sign_of_zero"] = "not fixed by the scope; +0 and -0 both satisfy it"
    in_range = all(math.isfinite(v["binary64"]["value"]) and
                   ((v["binary64"]["value"] != 0) == v["required_nonzero"]) for v in [force] + moments)
    return force, moments, in_range, all(agree), {"k": [bits(kb), str(Fr(kb))], "W": [bits(Wb), str(Fr(Wb))],
                                                   "r": {a: [bits(rb[a]), str(Fr(rb[a]))] for a in AXES}}


# ---------------------------------------------------------------------------------------------------------------
# Route B: quadrature in Decimal, a different pi, a different arc construction, diameter-form masses
# ---------------------------------------------------------------------------------------------------------------

PREC_B = 70


def pi_agm():
    with localcontext() as ctx:
        ctx.prec = PREC_B + 10
        a, b, t, p = Decimal(1), Decimal(1) / Decimal(2).sqrt(), Decimal(1) / 4, Decimal(1)
        for _ in range(12):
            an = (a + b) / 2
            b = (a * b).sqrt()
            t -= p * (a - an) ** 2
            a, p = an, 2 * p
        return (a + b) ** 2 / (4 * t)


def dsin_cos(x):
    with localcontext() as ctx:
        ctx.prec = PREC_B + 10
        s, c = Decimal(0), Decimal(0)
        term, n = Decimal(1), 0
        while True:
            if n % 4 == 0:
                c += term
            elif n % 4 == 1:
                s += term
            elif n % 4 == 2:
                c -= term
            else:
                s -= term
            n += 1
            term = term * x / n
            if abs(term) < Decimal(10) ** -(PREC_B + 8):
                break
        return s, c


def romberg(f, a, b, levels=12):
    with localcontext() as ctx:
        ctx.prec = PREC_B + 10
        R = [[(b - a) / 2 * (f(a) + f(b))]]
        n = 1
        for i in range(1, levels):
            n *= 2
            h = (b - a) / n
            s = sum(f(a + (2 * j - 1) * h) for j in range(1, n // 2 + 1))
            row = [R[i - 1][0] / 2 + h * s]
            for j in range(1, i + 1):
                row.append(row[j - 1] + (row[j - 1] - R[i - 1][j - 1]) / (4 ** j - 1))
            R.append(row)
        return R[-1][-1]


def D(q):
    q = Fr(q)
    with localcontext() as ctx:
        ctx.prec = PREC_B + 10
        return Decimal(q.numerator) / Decimal(q.denominator)


def route_b_members(members, nodes):
    """Length and first moment of every member by quadrature. Arc centre from the far tangent node and the
    incoming direction (C = B - R t_in); arcs parametrised r(th) = C + e1 cos th + e2 sin th, th in [0, pi/2]."""
    pi = pi_agm()
    into = {m["to"]: m for m in members}
    out = {}
    with localcontext() as ctx:
        ctx.prec = PREC_B + 10
        for m in members:
            a = [D(v) for v in nodes[m["from"]]]
            b = [D(v) for v in nodes[m["to"]]]
            if m["arc"] is None:
                L = sum((b[i] - a[i]) ** 2 for i in range(3)).sqrt()
                mid = [(a[i] + b[i]) / 2 for i in range(3)]
                first = [L * (a[i] + 4 * mid[i] + b[i]) / 6 for i in range(3)]   # Simpson
                out[m["id"]] = (L, first)
                continue
            R = D(m["arc"][1])
            prev = into[m["from"]]
            pa = [D(v) for v in nodes[prev["from"]]]
            d = [a[i] - pa[i] for i in range(3)]
            n = sum(x * x for x in d).sqrt()
            t_in = [x / n for x in d]
            C = [b[i] - R * t_in[i] for i in range(3)]
            e1 = [a[i] - C[i] for i in range(3)]
            e2 = [b[i] - C[i] for i in range(3)]
            L = romberg(lambda th: R, Decimal(0), pi / 2)
            first = []
            for i in range(3):
                def f(th, i=i):
                    s, c = dsin_cos(th)
                    return R * (C[i] + e1[i] * c + e2[i] * s)
                first.append(romberg(f, Decimal(0), pi / 2))
            out[m["id"]] = (L, first)
    return out, pi


def dcross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def to_dec(x):
    lo, hi = interval(x)
    with localcontext() as ctx:
        ctx.prec = PREC_B + 10
        return D((lo + hi) / 2)


# ---------------------------------------------------------------------------------------------------------------
# Build
# ---------------------------------------------------------------------------------------------------------------


def self_weight_ids(case_id, line_members, rigid_records):
    cid = "load:" + case_id
    ids = []
    for pid in line_members:
        ids.append("load:self-weight:%d:%s:%d:%s" % (len(cid), cid, len(pid), pid))
    for rec in rigid_records:
        comp = rec["component"]
        ids.append("load:self-weight:%d:%s:component:%d:%s:force:z" % (len(cid), cid, len(comp), comp))
        ids.append("load:self-weight:%d:%s:component:%d:%s:moment:%s"
                   % (len(cid), cid, len(comp), comp, rec["weight_moment_axis"]))
    return ids


def floors(force, moment):
    out = {}
    for fam, v in (("force_N", force), ("moment_N_m", moment)):
        scale = max(abs(value(c)) for c in v)
        out[fam] = {"zero_scale": scale, "floor": 1e-9 * scale,
                    "exact_zero_components": [AXES[i] for i, c in enumerate(v) if c.is_zero()]}
    return out


def build():
    CROSS.clear()
    getcontext().prec = PREC_B + 10
    staged, model = read_staged()
    STAGED_META.update({k: staged[k] for k in ("staged_at_commit", "source_git_blob")})
    assert model["pressure_contract"]["mode"] == "exact_pressure_v3"
    nodes, members, comps, replaced = build_inputs(model)
    by_id = {m["id"]: m for m in members}

    # --- transcription and set checks
    order = tuple(m["id"] for m in members)
    cross("member order", "the model's pipe_segments order against spec §3.2's table order", "transcription",
          order == SPEC_MEMBER_ORDER)
    cross("replaced span", "the objective connector's replaces_span span_ref against spec §5.1 (RS-P180)",
          "transcription", replaced == [WIND_EXCEPT])
    i0, i1 = SPEC_MEMBER_ORDER.index(WIND_FIRST), SPEC_MEMBER_ORDER.index(WIND_LAST)
    exposed = [p for p in SPEC_MEMBER_ORDER[i0:i1 + 1] if p not in replaced]
    cross("exposed set", "RS-P070 … RS-P200 in §3.2 order less the connector's replaced span, against the "
          "14 members listed here from spec §8.1", "transcription", tuple(exposed) == WIND_EXPECTED,
          {"exposed": exposed})

    # --- sections and geometry
    sections = {}
    for m in members:
        sid = m["section_ref"]
        metal, contents, insulation = mu_parts(m["section"])
        mu_coeff = metal + contents + insulation
        prev = sections.get(sid)
        if prev is not None:
            assert prev["coeff"] == mu_coeff, sid
            continue
        s = m["section"]
        cross("mu " + sid, "radius-form mass per length against the diameter form", "distinct calculation",
              mu_diameter_form(s) == mu_coeff)
        sections[sid] = {"coeff": mu_coeff, "parts": (metal, contents, insulation),
                         "projected_diameter": s["D"] + 2 * s["t_i"], "section": s}
    geo = {m["id"]: member_geometry(m, nodes) for m in members}
    in_plane = all(nodes[by_id[p][e]][0] == 6 for p in exposed for e in ("from", "to")) and \
        all(geo[p]["centre"][0] == 6 and geo[p]["e1"][0] == 0 == geo[p]["e2"][0]
            for p in exposed if geo[p]["kind"] == "arc")
    cross("exposed members normal to the wind", "every exposed member's end nodes have X = 6 m and each exposed "
          "arc's centre has X = 6 m with radius vectors of zero X component, so every exposed member lies in "
          "the plane X = 6 m, normal to +X", "distinct calculation", in_plane)

    # --- components
    quals = qualify(comps, members, replaced)
    replacing = {q["member"]: q for q in quals if q["replaces"]}
    cross("rigid-family components that replace their member", "the components the ordered rules admit "
          "against spec §5's statement (RS-F1, RS-V1, RS-F2, RS-R1)", "transcription",
          sorted(q["component"] for q in quals if q["replaces"]) == sorted(SPEC_RIGID))
    for q in quals:
        F = [ZERO, ZERO, X.c(-q["W"])]
        M = cross3([X.c(v) for v in q["r"]], F)
        nz = [AXES[i] for i in range(3) if not M[i].is_zero()]
        assert len(nz) == 1
        q["weight_moment_axis"] = nz[0]

    line_members = [m["id"] for m in members if m["id"] not in replaced and m["id"] not in replacing]
    cross("self-weight inventory", "21 line loads and a force and a moment for each of four components (29) "
          "against spec §8.1", "transcription",
          len(line_members) == 21 and len(replacing) == 4 and len(line_members) + 2 * len(replacing) == 29)

    # --- route B geometry
    rb_geo, pi_b = route_b_members(members, nodes)
    cross("pi", "Machin bounds against the Gauss-Legendre AGM value", "distinct calculation",
          D(PI_LO) <= pi_b + Decimal(10) ** -60 and pi_b - Decimal(10) ** -60 <= D(PI_HI))

    def mu_b(m):
        return D(mu_diameter_form(m["section"])) * pi_b

    cases = {}
    route_b_results = {}

    # --- wind
    w_loads, w_force, w_moment = [], [ZERO] * 3, [ZERO] * 3
    bF, bM = [Decimal(0)] * 3, [Decimal(0)] * 3
    e = unit(WIND_AXIS)
    for pid in exposed:
        sec = sections[by_id[pid]["section_ref"]]
        q = WIND_PRESSURE_PA * WIND_SHAPE * sec["projected_diameter"]
        g = geo[pid]
        w_force = vadd(w_force, vscale(e, g["length"] * q))
        w_moment = vadd(w_moment, cross3(g["first_moment"], vscale(e, q)))
        w_loads.append({"target": pid, "kind": "distributed_on_member", "direction": "+X", "span_fraction": [0, 1],
                        "intensity_N_per_m": num(q),
                        "inputs": {"pressure_Pa": str(WIND_PRESSURE_PA), "shape_factor": str(WIND_SHAPE),
                                   "projected_diameter_m": str(sec["projected_diameter"])}})
        L, first = rb_geo[pid]
        qd = D(q)
        bF = [bF[i] + L * qd * D(e[i].t.get((0, 1), 0)) for i in range(3)]
        bM = [bM[i] + dcross(first, [qd if a == WIND_AXIS else Decimal(0) for a in AXES])[i] for i in range(3)]
    cases["RS-WIN"] = {"loads": w_loads, "force": w_force, "moment": w_moment}
    route_b_results["RS-WIN"] = (bF, bM)

    # --- seismic
    formations_agree = True
    for case_id, axis, k in SEISMIC_CASES:
        e = unit(axis)
        loads, force, moment = [], [ZERO] * 3, [ZERO] * 3
        t_force, t_moment = [ZERO] * 3, [ZERO] * 3
        bF, bM = [Decimal(0)] * 3, [Decimal(0)] * 3
        tbF, tbM = [Decimal(0)] * 3, [Decimal(0)] * 3
        ed = [Decimal(1) if a == axis else Decimal(0) for a in AXES]
        for m in members:
            pid = m["id"]
            if pid in replaced:
                continue
            sec = sections[m["section_ref"]]
            q = X({(1, 1): k * G_SEISMIC * sec["coeff"]})
            g = geo[pid]
            df = vscale(e, g["length"] * q)
            dm = cross3(g["first_moment"], vscale(e, q))
            t_force, t_moment = vadd(t_force, df), vadd(t_moment, dm)
            L, first = rb_geo[pid]
            qd = D(k * G_SEISMIC) * mu_b(m)
            tb_f = [L * qd * ed[i] for i in range(3)]
            tb_m = dcross(first, [qd * ed[i] for i in range(3)])
            tbF = [tbF[i] + tb_f[i] for i in range(3)]
            tbM = [tbM[i] + tb_m[i] for i in range(3)]
            if pid in replacing:
                continue
            force, moment = vadd(force, df), vadd(moment, dm)
            bF = [bF[i] + tb_f[i] for i in range(3)]
            bM = [bM[i] + tb_m[i] for i in range(3)]
            loads.append({"target": pid, "kind": "distributed_on_member", "direction": "+" + axis.upper(),
                          "span_fraction": [0, 1], "intensity_N_per_m": num(q),
                          "inputs": {"g_factor": str(k), "gravity_m_per_s2": str(G_SEISMIC),
                                     "mass_per_length_kg_per_m": num(X({(1, 1): sec["coeff"]})),
                                     "mass_basis": MASS_BASIS_EX}})
        for qrec in quals:
            if not qrec["replaces"]:
                continue
            node = nodes[qrec["node"]]
            Fv = vscale(e, X.c(k * qrec["W"]))
            r = [X.c(v) for v in qrec["r"]]
            couple = cross3(r, Fv)
            force = vadd(force, Fv)
            moment = vadd(moment, vadd(cross3([X.c(v) for v in node], Fv), couple))
            fo, mo, in_range, agree, normalized = formation(k, qrec["W"], qrec["r"], axis)
            formations_agree = formations_agree and agree
            loads.append({"target": qrec["node"], "component": qrec["component"], "kind": "component_force",
                          "direction": "+" + axis.upper(), "value_N": fo["exact_on_entered_decimals"],
                          "binary64_by_formation": fo["binary64"], "required_nonzero": fo["required_nonzero"],
                          "inputs": {"g_factor": str(k), "W_N": str(qrec["W"]),
                                     "center_of_gravity_m": [str(v) for v in qrec["r"]],
                                     "normalized_binary64": normalized}})
            for mrec in mo:
                loads.append({"target": qrec["node"], "component": qrec["component"], "kind": "component_moment",
                              "axis": mrec["axis"], "formed_as": mrec["formed_as"],
                              "value_N_m": mrec["exact_on_entered_decimals"],
                              "binary64_by_formation": mrec["binary64"],
                              "required_nonzero": mrec["required_nonzero"],
                              "record_emitted_when_zero": None if not mrec["required_nonzero"] else True})
            assert in_range
            # route B: the force at its point of action, node + r
            p = [D(node[i] + qrec["r"][i]) for i in range(3)]
            fd = [D(k * qrec["W"]) * ed[i] for i in range(3)]
            bF = [bF[i] + fd[i] for i in range(3)]
            bM = [bM[i] + dcross(p, fd)[i] for i in range(3)]
        cases[case_id] = {"loads": loads, "force": force, "moment": moment, "axis": axis, "k": k,
                          "twin_force": t_force, "twin_moment": t_moment}
        route_b_results[case_id] = (bF, bM)
        route_b_results[case_id + " twin"] = (tbF, tbM)
    cross("binary64 formation", "each component force and moment rounded here from the exact rational product "
          "(ties to even) against the hardware binary64 product of the same binary64 operands, bits",
          "distinct calculation", formations_agree)

    # --- route A against route B
    worst = Decimal(0)
    for name, (bF, bM) in route_b_results.items():
        cid = name.split(" ")[0]
        c = cases[cid]
        fA, mA = (c["twin_force"], c["twin_moment"]) if name.endswith("twin") else (c["force"], c["moment"])
        for a_vec, b_vec in ((fA, bF), (mA, bM)):
            scale = max(abs(to_dec(x)) for x in a_vec)
            for xa, xb in zip(a_vec, b_vec):
                with localcontext() as ctx:
                    ctx.prec = PREC_B
                    rel = abs(to_dec(xa) - xb) / scale
                worst = max(worst, rel)
    cross("resultants by quadrature", "every resultant force and moment (wind, three seismic cases, three twins) "
          "against direct quadrature along each member and arc: a different pi (AGM), arc centre from the "
          "incoming tangent, trigonometric parametrisation with Romberg integration, diameter-form masses, "
          "component forces at node + r; agreement within 1e-50 of each family's largest magnitude",
          "distinct calculation", worst < Decimal(10) ** -50, {"largest_difference_relative": "%.3e" % worst})

    # --- seismic Z, X and Y against the r1_ce weight (exact mapping)
    sib = SIBLING["r1_ce"]
    W_tot = X.parse(sib["W_total_N"])
    S = [X.parse(s) for s in sib["weight_first_moment_N_m"]]
    Mw = [X.parse(s) for s in sib["weight_moment_about_origin_N_m"]]
    for case_id, axis, k in SEISMIC_CASES:
        c = cases[case_id]
        e = unit(axis)
        okF = c["force"] == vscale(e, W_tot * k)
        okM = c["moment"] == vscale(cross3(S, e), k)
        detail = None
        if axis == "z":
            okM = okM and c["moment"] == vscale(Mw, -k)
        cross("%s against reference_system_r1_ce" % case_id,
              "force k·W_total·e and moment k·(S × e) from the frozen weight total W_total and first moment S of "
              "validation/references/reference_system_r1_ce (for Z also −k times its weight moment), exactly. "
              "Exact because the seismic gravity equals the weight gravity's magnitude (9.80665 m/s², §2, §8.1), "
              "the same members carry distributed load on the same basis, and each component's W is held fixed "
              "at the same centre of gravity",
              "distinct calculation by another author", okF and okM, detail)

    # --- seismic Z against the tranche-1 package: exact only after RS-R1's substitution
    W1 = X.parse(SIBLING["tranche1"]["W_total_N"])
    p150 = by_id["pipe:RS-P150"]
    sub = X.c(replacing["pipe:RS-P150"]["W"]) - X({(1, 1): G_WEIGHT * sections[p150["section_ref"]]["coeff"]}) * \
        geo["pipe:RS-P150"]["length"]
    cross("RS-EQZ against reference_system (tranche 1)",
          "the force k·(W_total(tranche 1) + 300 N − g·μ_S270·√257/40) against RS-EQZ's force, exactly. Not exact "
          "without the correction: tranche 1 has no RS-R1, so RS-P150 carries its distributed weight there",
          "distinct calculation by another author",
          cases["RS-EQZ"]["force"][2] == (W1 + sub) * Fr("0.20"))

    # --- wind and seismic against the pending-cases package's full system (same rule, gravity then assumed)
    pend = SIBLING["pending_full"]
    for case_id, (fs, ms) in ((k, v) for k, v in pend.items() if k != "file"):
        c = cases[case_id]
        ok = c["force"] == [X.parse(s) for s in fs] and c["moment"] == [X.parse(s) for s in ms]
        cross("%s against reference_system_pending_cases (full)" % case_id,
              "generated force and moment about the origin, exactly; that package assumed the seismic gravity "
              "9.80665 m/s² before spec §8.1 stated it, under the rule decided on 2026-11-12, the rule the "
              "scope now states", "distinct calculation by another author", ok)

    ctl = SIBLING["pending_control_before_2026_11_12"]
    for case_id, axis, k in SEISMIC_CASES:
        fs, ms = ctl[case_id]
        c = cases[case_id]
        ok = c["twin_force"] == [X.parse(t) for t in fs] and c["twin_moment"] == [X.parse(t) for t in ms]
        cross("%s twin against reference_system_pending_cases control" % case_id,
              "the pipes-only twin's force and moment against that package's full-system control 'the rule "
              "before 2026-11-12: pipe mass on a mapped member, component weight left out', exactly",
              "distinct calculation by another author", ok)

    # --- assemble
    out_cases = {}
    for case_id in ("RS-WIN",) + tuple(c[0] for c in SEISMIC_CASES):
        c = cases[case_id]
        term = "RS-d" + case_id[3:]
        rec = {
            "generated_loads": c["loads"],
            "generated_load_counts": {},
            "resultant": {"force_N": vec(c["force"]), "moment_about_origin_N_m": vec(c["moment"])},
            "EQ-6": {"reference_of_record": "validation/references/reference_system_pending_cases key_values.full."
                                            + case_id + " (expected_" + term + "_…); checked equal here",
                     "term": term, "expected_reaction_force_resultant_N": vec(vneg(c["force"])),
                     "expected_reaction_moment_resultant_about_origin_N_m": vec(vneg(c["moment"]))},
            "floors": floors(c["force"], c["moment"]),
        }
        counts = {}
        for ld in c["loads"]:
            key = ld["kind"] + ("" if ld["kind"] != "component_moment" else
                                (" required nonzero" if ld["required_nonzero"] else " zero"))
            counts[key] = counts.get(key, 0) + 1
        rec["generated_load_counts"] = counts
        if case_id != "RS-WIN":
            diff_f = [c["force"][i] - c["twin_force"][i] for i in range(3)]
            diff_m = [c["moment"][i] - c["twin_moment"][i] for i in range(3)]
            disc = []
            for fam, exp, d in (("force", c["force"], diff_f), ("moment", c["moment"], diff_m)):
                for i in range(3):
                    if exp[i].is_zero():
                        disc.append({"component": fam + "_" + AXES[i], "expected_exact_zero": True,
                                     "difference_exact_zero": d[i].is_zero(), "discriminates": False,
                                     "why": "exact zero under both the rule and the twin"})
                    else:
                        ratio = abs(value(d[i])) / (1e-9 * abs(value(exp[i])))
                        disc.append({"component": fam + "_" + AXES[i], "expected_exact_zero": False,
                                     "difference_over_tolerance": float("%.6g" % ratio),
                                     "discriminates": ratio > 1})
            rec["pipes_only_twin"] = {
                "what": "every non-replaced member keeps k·g·μ, no component force",
                "force_N": vec(c["twin_force"]), "moment_about_origin_N_m": vec(c["twin_moment"]),
                "rule_minus_twin_force_N": vec(diff_f), "rule_minus_twin_moment_N_m": vec(diff_m),
                "discrimination_at_the_comparison_rule": disc,
            }
            rec["seismic_model_check"] = warning_record(case_id, line_members, quals)
        else:
            rec["seismic_model_check"] = {"expected_findings": "none",
                                          "why": "RS-WIN is not a seismic case; AC-007: none of those findings "
                                                 "otherwise"}
        out_cases[case_id] = rec

    doc = {
        "document": "validation/references/reference_system_generated_loads/references.json",
        "status": "candidate per-load and warning references, derived and cross-checked by their author, not yet "
                  "independently refuted",
        "supplements": {"package": "validation/references/reference_system_pending_cases",
                        "relation": "that package remains the reference for EQ-6; the four resultants and three "
                                    "twins here are values this generator checks equal to its frozen ones, exactly"},
        "governs": {"scope": "execution/PKG-05/DEL-05-01/ScopeOfWork.md at aebd3442f: Static-equivalent "
                             "generation, Mass basis, Self-weight plan, REQ-008, REQ-015, AC-007, VER-005",
                    "specification": "fixtures/reference_system/SPECIFICATION.md §2, §3, §4.2, §5, §5.1, §8.1, "
                                     "§8.2, §9.2 (EQ-4, EQ-6 as restated at e494a5925)",
                    "quoted": SPEC},
        "reads": {"staged_input": STAGED_REL, "staged_from": MODEL_REL,
                  "staged_at_commit": STAGED_META["staged_at_commit"],
                  "source_git_blob": STAGED_META["source_git_blob"],
                  "members_staged": list(KEPT_MEMBERS), "members_not_staged": list(DROPPED_MEMBERS)},
        "units": {"length": "m", "force": "N", "moment": "N*m", "distributed": "N/m", "mass_per_length": "kg/m"},
        "conventions": {
            "origin": "global (0, 0, 0)",
            "moment": "sum of r × F over every force at its point of action, plus the component couples",
            "sign": "a positive g-factor and global_x wind load along the positive global axis (spec §8.1: the "
                    "factor's sign is the direction's sign)",
            "member": "straight: r(s) from its from-node to its to-node; arc: centre C, e1 and e2 the radius "
                      "vectors to its from- and to-node, length R·π/2, ∫ r ds = R[(π/2)C + e1 + e2]",
            "exact": "a sum of (q)*sqrt(n)*pi^k terms, q rational",
            "decimal": "40 significant digits, round half even; value: the nearest binary64; each emitted only "
                       "when both ends of a bounding interval give the same",
        },
        "inputs": inputs_record(nodes, members, quals, exposed, replaced),
        "mass_basis": {"name": MASS_BASIS_EX, "route": "exact (spec §2 pressure_contract exact_pressure_v3)",
                       "formula": "μ = π[ρ_m(R² − (R − t)²) + ρ_c(R − ((t − c_m) − c_a))² + ρ_i((R + t_i)² − R²)]"},
        "sections": {sid: {"metal_kg_per_m": num(X({(1, 1): s["parts"][0]})),
                           "contents_kg_per_m": num(X({(1, 1): s["parts"][1]})),
                           "insulation_kg_per_m": num(X({(1, 1): s["parts"][2]})),
                           "mu_kg_per_m": num(X({(1, 1): s["coeff"]})),
                           "projected_diameter_m": str(s["projected_diameter"])}
                     for sid, s in sections.items()},
        "members": [{"id": m["id"], "from": m["from"], "to": m["to"], "section": m["section_ref"],
                     "kind": geo[m["id"]]["kind"], "length_m": num(geo[m["id"]]["length"]),
                     "first_moment_m2": vec(geo[m["id"]]["first_moment"]),
                     "connector_replaced": m["id"] in replaced, "wind_exposed": m["id"] in exposed,
                     "seismic": ("none" if m["id"] in replaced else
                                 "component force of " + replacing[m["id"]]["component"]
                                 if m["id"] in replacing else "distributed")}
                    for m in members],
        "arcs": {m["id"]: {"radius_m": str(geo[m["id"]]["radius"]), "corner": [str(v) for v in geo[m["id"]]["corner"]],
                           "centre": [str(v) for v in geo[m["id"]]["centre"]],
                           "e1": [str(v) for v in geo[m["id"]]["e1"]], "e2": [str(v) for v in geo[m["id"]]["e2"]]}
                 for m in members if m["arc"]},
        "component_qualification": [{"component": q["component"], "kind": q["kind"], "node": q["node"],
                                     "member": q["member"], "W_N": str(q["W"]),
                                     "center_of_gravity_m": [str(v) for v in q["r"]],
                                     "ordered_conditions": q["steps"], "replaces_member_load": q["replaces"]}
                                    for q in quals],
        "cases": out_cases,
        "EQ-4": {"frozen_values": None,
                 "why": "an identity on published rows (spec §9.2 EQ-4)",
                 "recompute": "on each signed component row of RS-EQ-SRSS and RS-EQ-ABS, with x_X, x_Y, x_Z the "
                              "matching rows of RS-dEQX, RS-dEQY, RS-dEQZ as published and factors 1: "
                              "ABS = RN(|x_X| + |x_Y| + |x_Z|), the exact sum rounded once; SRSS = RN(√(x_X² + x_Y² "
                              "+ x_Z²)), the exact square root of the exact sum, rounded once (DEL-05-02 "
                              "'Absolute sum', 'SRSS'); then SRSS ≤ ABS. Magnitude rows follow the combination's "
                              "own recomputation rule; withheld rows are excluded"},
        "comparison_rule": {
            "nonzero": "|observed − expected| ≤ 1e-9·|expected| (relative alone)",
            "exact_zero": "|observed| ≤ 1e-9·zero_scale, zero_scale the largest magnitude of its family in the "
                          "item (each item's floors record it)",
            "discrete": "compared exactly",
            "component_bits": "a component force or moment the scope's formation rule fixes is also given as "
                              "binary64 bits; a test may compare those exactly",
            "source": "DEL-05-01 AC-001 (1e-9 relative)",
            "relative": 1e-9,
        },
        "seismic_warning_decisions": DECISIONS_WARNING,
        "seismic_warning_decisions_scope_text": DECISION_SCOPE_TEXT,
        "warning_readings_considered_and_excluded": CONSIDERED_AND_EXCLUDED,
        "open_items": OPEN_ITEMS,
        "crosscheck": CROSS,
        "crosscheck_count": len(CROSS),
    }
    assert all(c["agrees"] for c in CROSS), [c for c in CROSS if not c["agrees"]]
    return doc


def inputs_record(nodes, members, quals, exposed, replaced):
    return {
        "nodes_m": {k: [str(v) for v in p] for k, p in nodes.items()},
        "members": {m["id"]: {"from": m["from"], "to": m["to"], "section_ref": m["section_ref"],
                              "section_m_and_kg_per_m3": {k: str(v) for k, v in m["section"].items()},
                              "arc": None if m["arc"] is None else {"bend": m["arc"][0], "radius_m": str(m["arc"][1])}}
                    for m in members},
        "wind": {"pressure_Pa": str(WIND_PRESSURE_PA), "shape_factor": str(WIND_SHAPE), "direction": "global_x",
                 "exposed": exposed, "excluded": replaced},
        "seismic": {c: {"axis": a, "g_factor": str(k), "gravity_m_per_s2": str(G_SEISMIC)}
                    for c, a, k in SEISMIC_CASES},
        "weight_gravity": {"axis": "global_z", "value_m_per_s2": "-" + str(G_WEIGHT)},
        "bend_corners_spec_3_1": {k: [str(v) for v in c] for k, c in SPEC_CORNERS.items()},
    }


DECISIONS_WARNING = [
    {"decided_by": "SWBPIPE's Agent 0", "date": "2026-11-18",
     "decision": "the seismic model-check warning names the two kinds of weight REQ-008 states, loads in the weight "
                 "category and, on the pressure-free route, the added weight a case takes from an operating condition; "
                 "it does not name applied generated self-weight",
     "reason": "the warning tells the engineer which weights seismic generation did not scale; a pipe's applied "
               "self-weight and a component's applied weight are the ones it did, since generation takes the pipe's "
               "mass and the component's W directly"},
    {"decided_by": "SWBPIPE's Agent 0", "date": "2026-11-18",
     "decision": "a manual override of generated self-weight is named in the warning"},
    {"decided_by": "SWBPIPE's Agent 0", "date": "2026-11-18",
     "decision": "the gravity comparison reads fresh applied component-weight primitives only; an overridden component "
                 "contributes no gravity to it"},
    {"decided_by": "SWBPIPE's Agent 0", "date": "2026-11-18",
     "decision": "the weight category is the solver's parsed category, an entered distributed member load included; "
                 "the text says 'load in the weight category'"},
]
DECISION_SCOPE_TEXT = "the scope's sentences are to be clarified in a later change; no scope wording is quoted for them"


def warning_record(case_id, line_members, quals):
    rigid = [q for q in quals if q["replaces"]]
    ids = self_weight_ids(case_id, line_members, rigid)
    cid = "load:" + case_id
    baseline = ["load:self-weight:%d:%s:component:%d:%s:force:z" % (len(cid), cid, len(q["component"]),
                                                                    q["component"]) for q in rigid]
    return {
        "weight_finding_required": False,
        "gravity_finding_required": G_WEIGHT != G_SEISMIC,
        "any_seismic_model_check_finding_required": False,
        "ground": "the four decisions in seismic_warning_decisions (SWBPIPE's Agent 0, 2026-11-18)",
        "condition": "the case's 29 self-weight primitives are fresh (the self-weight plan's output, matching the "
                     "model, as spec §8.1 states them: none overridden), and the case holds no other load in the "
                     "weight category; the expectation holds only under this condition",
        "self_weight_primitives_of_the_case": {
            "count": len(ids), "ids": ids,
            "id_reading": "ids from spec §8.1's rule with <case id> = '%s', <pipe id> and <component id> the full "
                          "ids (spec §2 Identities), the component moment's <axis> the one nonzero component of "
                          "r × (0, 0, −W); listed in spec §3.2 member order, then component by component" % cid,
        },
        "other_loads_in_the_weight_category": [],
        "added_weight_named": [],
        "added_weight_why": "the case is on the exact route, and no case carries condition added_weight (spec §8)",
        "gravity_comparison": {
            "baseline_loads": baseline,
            "baseline_why": "the fresh applied component-weight primitives of the case for the components whose "
                            "forces it generates (RS-F1, RS-V1, RS-F2, RS-R1); none is overridden",
            "applied_gravity_magnitude_m_per_s2": str(G_WEIGHT),
            "applied_source": "spec §2 (−9.80665 m/s² along global_z on each weight case) and §8.1 (W: gravity "
                              "as §2)",
            "seismic_gravity_magnitude_m_per_s2": str(G_SEISMIC),
            "seismic_source": "spec §8.1",
            "equal_after_normalization": G_WEIGHT == G_SEISMIC,
            "gravity_finding_required": G_WEIGHT != G_SEISMIC,
        },
        "code": None, "message": None, "findings_per_load_or_per_case": None, "order_of_named_loads": None,
        "open_fields_matter_for": "only a test that asserts the absence of these findings",
    }


OPEN_ITEMS = [
    {"id": "OI-1", "item": "the seismic warning's code and message words", "frozen": None,
     "unsettled": "REQ-008 states what the findings name, not their code or wording; no finding is required on "
     "these cases, so this matters only for a test that asserts absence"},
    {"id": "OI-2", "item": "one finding per named load or one per case, and their order", "frozen": None,
     "unsettled": "REQ-008 says 'findings' and fixes neither their number nor their order"},
    {"id": "OI-3", "item": "whether an exactly zero transverse moment component becomes a load record",
     "frozen": None, "unsettled": "the scope fixes when a component is required nonzero, not whether a zero one "
     "is emitted; the values and the required-nonzero set are frozen, the record count is not"},
    {"id": "OI-4", "item": "generated-load identities and their order", "frozen": None,
     "unsettled": "no identifier rule is given for generated wind or seismic loads; loads are keyed here by "
     "target, kind and axis"},
]

CONSIDERED_AND_EXCLUDED = [
    {"reading": "the warning names the seismic case's own applied self-weight primitives (29 per case)",
     "excluded_by": "SWBPIPE's Agent 0's decision of 2026-11-18 that applied generated self-weight is not named"},
    {"reading": "the warning names every weight-category load in the document",
     "excluded_by": "REQ-008 'on each seismic case' (the assigner's reading of it as that case's loads)"},
]


# ---------------------------------------------------------------------------------------------------------------
# Output
# ---------------------------------------------------------------------------------------------------------------


def render(doc):
    return json.dumps(doc, indent=1, ensure_ascii=False) + "\n"


def readme_quotes(doc):
    text = README.read_text(encoding="utf-8") if README.exists() else ""
    missing = []
    for case_id, c in doc["cases"].items():
        for v in c["resultant"]["force_N"] + c["resultant"]["moment_about_origin_N_m"]:
            if v["value"] != 0 and repr(v["value"]) not in text:
                missing.append(case_id + " " + repr(v["value"]))
    return missing


def sums(files):
    return "".join("%s  %s\n" % (hashlib.sha256(b).hexdigest(), name) for name, b in sorted(files.items()))


def main():
    ap = argparse.ArgumentParser()
    mode = ap.add_mutually_exclusive_group()
    mode.add_argument("--write", action="store_true")
    mode.add_argument("--check-live", action="store_true")
    args = ap.parse_args()
    if args.check_live:
        return check_live()
    doc = build()
    missing = readme_quotes(doc)
    doc["crosscheck"].append({"check": "README quotes", "compares": "the README quotes every nonzero resultant "
                              "value", "kind": "consistency", "agrees": not missing})
    doc["crosscheck_count"] = len(doc["crosscheck"])
    refs = render(doc).encode("utf-8")
    files = {"README.md": README.read_bytes() if README.exists() else b"",
             SELF_REL: Path(__file__).read_bytes(), STAGED_REL: STAGED.read_bytes(), "references.json": refs}
    sums_bytes = sums(files).encode("utf-8")
    if args.write:
        REFERENCES.write_bytes(refs)
        SUMS.write_bytes(sums_bytes)
        print("wrote references.json and SHA256SUMS; README quotes missing: %d" % len(missing))
        return 0 if not missing else 1
    problems = []
    if missing:
        problems.append("README does not quote: " + ", ".join(missing))
    if not REFERENCES.exists() or REFERENCES.read_bytes() != refs:
        problems.append("references.json differs from the rebuilt bytes")
    if not SUMS.exists() or SUMS.read_bytes() != sums_bytes:
        problems.append("SHA256SUMS differs from the rebuilt bytes")
    for p in problems:
        print(p)
    if not problems:
        print("references.json and SHA256SUMS reproduce byte for byte; %d cross-checks agree"
              % doc["crosscheck_count"])
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
