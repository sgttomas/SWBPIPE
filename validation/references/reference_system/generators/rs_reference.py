#!/usr/bin/env python3
"""Reference-system independent references: total weight and its moment (EQ-1, EQ-2).

Standard-library Python 3 only. This generator imports, reads and runs no product code. It reads one
file besides the ones it writes: the reference-system fixture's invented model inputs
(`fixtures/reference_system/reference_system.json`), as data. From that file it reads the nodes, the
pipe segments and their sections, and the components; it never reads `load_cases` (so none of the
captured self-weight primitives `load:self-weight:*`), and it never reads `self_weight_capture.json`.
The weight is recomputed here from the sections, the material density, the geometry and the
component weights, on EX's stated mass bases (README, "Mass bases"):

  pipe metal       rho_m * pi * (R^2 - r_n^2),        r_n = R - t           (nominal wall)
  contents         rho_c * pi * r_p^2,                r_p = R - ((t - c_m) - c_a)  (reduced bore)
  insulation       rho_i * pi * ((R + t_i)^2 - R^2)   (OD to OD + 2 t_i)
  member weight    (mass per length) * g * length,    g = 9.80665 m/s^2 along -Z
  realized arc     length R * theta (theta = pi/2 here)
  rigid family     the component's own weight W at node + centre-of-gravity vector, in place of its
                   mapped member's distributed weight
  replaced span    RS-P180 (the expansion joint's span) carries no weight

The fixture's numbers are read as exact decimals (json parse_float = Fraction), so every input is an
exact rational. Every result is then an exact sum of terms q * pi^k * sqrt(n) with q rational: k = 1
for straight members and k = 1 or 2 for arcs (the mass per length carries one pi, the arc length a
second); sqrt(257)/40 m is the length of the one inclined member RS-P150. The transcendental factors
are evaluated separately and under control: pi by Machin's formula to about 95 digits with explicit
rational lower and upper bounds, sqrt(n) by integer square root with bounds. Decimals are printed to 40
significant digits; binary64 values are the correctly rounded nearest doubles, checked by rounding
both ends of the bounding interval, which must agree.

Cross-checks (recorded in references.json): each mass per length in a second factored form; each arc
first moment against the classical centroid R sin(theta/2)/(theta/2) on the bisector (evaluated in
Decimal); each arc's corner against the specification's corner points; each component's centre of
gravity against the specification's "body midpoint (+ offset)"; the omitted members against the
specification; the total weight as a per-section sum of lengths.

Modes:
  python3 -I rs_reference.py           check: rebuild every output in memory and compare byte for byte
                                        with the committed files, SHA256SUMS included
  python3 -I rs_reference.py --write   write the outputs (intentional changes only)
"""
import argparse
import hashlib
import json
import sys
from decimal import Decimal, getcontext
from fractions import Fraction as F
from math import isqrt
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE.parent
ROOT = HERE.parents[3]
FIXTURE = ROOT / "fixtures" / "reference_system" / "reference_system.json"
FIXTURE_REL = "fixtures/reference_system/reference_system.json"
FIXED_FILES = ("README.md", "generators/rs_reference.py")

# ----------------------------------------------------------------------------- controlled evaluation
getcontext().prec = 110
DIGITS = 90  # rational bounds are this many decimal places wide


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


PI_DEC = 16 * _atan_inv(5) - 4 * _atan_inv(239)  # Machin, ~105 correct digits
_PI_T = F(str(PI_DEC)[:DIGITS + 2])  # "3." + 90 digits, truncated
PI_LO, PI_HI = _PI_T, _PI_T + F(1, 10 ** DIGITS)
assert PI_LO < F(314159265358979323847, 10 ** 20) and PI_HI > F(314159265358979323846, 10 ** 20)


def sqrt_bounds(n):
    s = isqrt(n * 10 ** (2 * DIGITS))
    assert s * s <= n * 10 ** (2 * DIGITS) < (s + 1) ** 2
    return F(s, 10 ** DIGITS), F(s + 1, 10 ** DIGITS)


def squarefree_split(q):
    """sqrt(q) = c * sqrt(n) with c rational and n a squarefree positive integer (q > 0 rational)."""
    num, den = q.numerator * q.denominator, q.denominator  # sqrt(q) = sqrt(num) / den
    c, n, p = 1, num, 2
    while p * p <= n:
        while n % (p * p) == 0:
            n //= p * p
            c *= p
        p += 1
    return F(c, den), n


class X:
    """An exact sum of q * pi^k * sqrt(n) terms, keyed by (k, n)."""

    def __init__(self, terms=None):
        self.t = {key: q for key, q in (terms or {}).items() if q != 0}

    @staticmethod
    def of(q, k=0, n=1):
        return X({(k, n): F(q)})

    def __add__(self, other):
        terms = dict(self.t)
        for key, q in other.t.items():
            terms[key] = terms.get(key, F(0)) + q
        return X(terms)

    def __neg__(self):
        return X({key: -q for key, q in self.t.items()})

    def __sub__(self, other):
        return self + (-other)

    def scale(self, q):
        return X({key: q * v for key, v in self.t.items()})

    def times_pi(self):
        return X({(k + 1, n): q for (k, n), q in self.t.items()})

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
            plo, phi = (PI_LO ** k, PI_HI ** k)
            slo, shi = sqrt_bounds(n) if n != 1 else (F(1), F(1))
            a, b = q * plo * slo, q * phi * shi
            lo, hi = lo + min(a, b), hi + max(a, b)
        return lo, hi

    def decimal(self):
        total = Decimal(0)
        for (k, n), q in self.t.items():
            total += Decimal(q.numerator) / Decimal(q.denominator) * PI_DEC ** k * Decimal(n).sqrt()
        return total

    def binary64(self):
        lo, hi = self.interval()
        if float(lo) != float(hi):
            raise SystemExit(f"binary64 rounding of {self.text()} is not decided by the bounds")
        return float(lo)

    def record(self):
        v = self.decimal()
        dec = "0" if not self.t else format(v, ".39e")
        return {"exact": self.text(), "decimal": dec, "value": self.binary64()}


def vrec(vector):
    return [x.record() for x in vector]


# ----------------------------------------------------------------------------- the specification's statements
# DEL-09-01 Work/reference-system.md, held here so the fixture's transcription is checked against the spec.
G = F("9.80665")  # m/s^2, standard gravity entered as global_z -9.80665 (spec §2)
SPEC_DENSITY = F(7850)  # kg/m^3 (spec §4.1, §4.2)
# §4.2: OD, wall, mill tolerance, corrosion allowance, insulation thickness (mm); insulation, contents (kg/m^3)
SPEC_SECTIONS = {
    "section:RS-S270": ("270", "9.0", "1.0", "1.5", "60", "140", "820"),
    "section:RS-S220": ("220", "8.0", "0.9", "1.5", "60", "140", "820"),
    "section:RS-S170": ("170", "7.0", "0.8", "1.5", "50", "140", "820"),
}
# §5: rigid family weight (N) and the centre of gravity as the spec states it: offset from the body midpoint
SPEC_RIGID = {
    "component:RS-F1": (F(350), (F(0), F(0), F(0))),
    "component:RS-V1": (F(2600), (F(0), F(0), F("0.30"))),
    "component:RS-F2": (F(350), (F(0), F(0), F(0))),
}
# §3.1 corner points of the realized bends, and §5's radius and angle (90 degrees)
SPEC_BENDS = {
    "component:RS-B1": ((F(6), F(0), F(1)), F("0.400")),
    "component:RS-B2": ((F(6), F(0), F(7)), F("0.400")),
    "component:RS-B3": ((F(6), F(15), F(2)), F("0.250")),
}
SPEC_OMITTED = {"pipe:RS-P020": "component:RS-F1", "pipe:RS-P030": "component:RS-V1",
                "pipe:RS-P040": "component:RS-F2", "pipe:RS-P180": "component:RS-EJ1"}
RIGID_KINDS = {"valve", "flange", "reducer", "rigid", "specialty"}

CROSS = []


def cross_check(label, a, b):
    CROSS.append({"check": label, "agrees": a == b})


# ----------------------------------------------------------------------------- reading the fixture's inputs
def read_inputs():
    doc = json.loads(FIXTURE.read_text(encoding="utf-8"), parse_float=F, parse_int=F)
    # Only these members are read; load_cases (the captured self-weight primitives) are not.
    return {"nodes": doc["nodes"], "pipe_segments": doc["pipe_segments"], "components": doc["components"],
            "pressure_contract": doc["pressure_contract"]}


def length_m(quantity):
    unit = quantity["unit"]
    scale = {"m": F(1), "mm": F(1, 1000)}[unit]
    return F(quantity["value"]) * scale


def vec(a):
    return tuple(F(x) for x in a)


def sub(a, b):
    return tuple(x - y for x, y in zip(a, b))


def add(a, b):
    return tuple(x + y for x, y in zip(a, b))


def mul(q, a):
    return tuple(q * x for x in a)


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def section_mass(section):
    """Mass per length / pi (kg/m), on EX's stated bases."""
    def get(key, unit):
        q = section[key]
        assert q["unit"] == unit, (key, q["unit"])
        return F(q["value"])
    d = get("outside_diameter", "mm") / 1000
    t = get("wall_thickness", "mm") / 1000
    cm = get("mill_tolerance", "mm") / 1000
    ca = get("corrosion_allowance", "mm") / 1000
    ti = get("insulation_thickness", "mm") / 1000
    rho_m, rho_c, rho_i = (get("material_density", "kg/m^3"), get("contents_density", "kg/m^3"),
                           get("insulation_density", "kg/m^3"))
    r = d / 2
    rn = r - t  # nominal wall
    tr = (t - cm) - ca  # reduced wall
    rp = r - tr  # pressure (reduced) bore
    metal, contents, insulation = rho_m * (r * r - rn * rn), rho_c * rp * rp, rho_i * ((r + ti) ** 2 - r * r)
    # Second form: annulus areas as t(D - t), t_i(D + t_i); bore (D/2 - t + c_m + c_a)^2.
    alt = rho_m * t * (d - t) + rho_c * (d / 2 - t + cm + ca) ** 2 + rho_i * ti * (d + ti)
    return {"d": d, "t": t, "cm": cm, "ca": ca, "ti": ti, "rho_m": rho_m, "rho_c": rho_c, "rho_i": rho_i,
            "metal": metal, "contents": contents, "insulation": insulation,
            "total": metal + contents + insulation, "alt": alt}


def build():
    CROSS.clear()
    inp = read_inputs()
    assert inp["pressure_contract"] == {"version": "3.0.0", "mode": "exact_pressure_v3",
                                        "shear_model": "euler_bernoulli"}
    nodes = {n["id"]: (F(n["position"]["x"]), F(n["position"]["y"]), F(n["position"]["z"])) for n in inp["nodes"]}
    pipes = {p["id"]: p for p in inp["pipe_segments"]}
    components = {c["id"]: c for c in inp["components"]}

    # Sections: per pipe, checked against the spec's table; one mass per section id.
    sections = {}
    for pipe in pipes.values():
        sid, sec = pipe["section_ref"], pipe["section"]
        m = section_mass(sec)
        od, wall, mill, ca, ins, ins_rho, contents = (F(x) for x in SPEC_SECTIONS[sid])
        assert (m["d"], m["t"], m["cm"], m["ca"], m["ti"]) == (od / 1000, wall / 1000, mill / 1000, ca / 1000,
                                                              ins / 1000), sid
        assert (m["rho_m"], m["rho_i"], m["rho_c"]) == (SPEC_DENSITY, ins_rho, contents), sid
        if sid in sections:
            assert sections[sid] == m, sid
        sections[sid] = m
    for sid, m in sorted(sections.items()):
        cross_check(f"{sid}: mass per length, second factored form", m["total"], m["alt"])

    # Members replaced by a component's weight, and the span an objective connector replaces.
    omitted = {}
    rigid = []
    for cid, comp in sorted(components.items()):
        connector = comp.get("objective_connector")
        if connector and connector["topology"]["type"] == "replaces_span":
            omitted[connector["topology"]["span_ref"]] = (cid, "replaced span: carries no weight (spec §5.1)")
        if comp["kind"] in RIGID_KINDS:
            geo = comp["geometry"]
            consumption = comp.get("mechanics_interface", {}).get("solver_consumption")
            assert consumption == "mechanics_geometry_only" and "weight" in geo and "center_of_gravity" in geo, cid
            assert geo["weight"]["unit"] == "N" and geo["center_of_gravity"]["unit"] == "m", cid
            omitted[geo["rigid_pipe_ref"]] = (cid, "rigid-family component weight replaces the member's distributed "
                                                   "weight (W-15 §3)")
            rigid.append(cid)
    cross_check("omitted members are the specification's (RS-P020, RS-P030, RS-P040 by component weight; RS-P180 "
                "replaced span)", {k: v[0] for k, v in omitted.items()}, SPEC_OMITTED)

    arcs = {}
    for cid, comp in components.items():
        if comp["kind"] == "bend":
            assert comp["mechanics_interface"]["solver_consumption"] == "curved_bend_macro_element", cid
            arcs[comp["geometry"]["bend_pipe_ref"]] = (cid, length_m(comp["geometry"]["bend_radius"]))

    def neighbour_far_node(node, exclude):
        """The other end of the one member at `node` besides `exclude`."""
        found = [p for pid, p in pipes.items() if pid != exclude and node in (p["from"], p["to"])]
        assert len(found) == 1, (node, [p["id"] for p in found])
        p = found[0]
        return p["to"] if p["from"] == node else p["from"]

    members, total = [], X()
    first = [X(), X(), X()]  # sum of weight * position, N*m
    by_section = {}
    for pid, pipe in pipes.items():
        a, b = nodes[pipe["from"]], nodes[pipe["to"]]
        sid = pipe["section_ref"]
        mu = sections[sid]["total"]  # kg/m / pi
        if pid in omitted:
            members.append({"member": pid, "section": sid, "carries": "no distributed weight",
                            "by": omitted[pid][0], "reason": omitted[pid][1]})
            continue
        q = mu * G  # weight per length / pi, N/m
        if pid in arcs:
            cid, radius = arcs[pid]
            corner_spec, radius_spec = SPEC_BENDS[cid]
            assert radius == radius_spec, cid
            far_in = nodes[neighbour_far_node(pipe["from"], pid)]
            far_out = nodes[neighbour_far_node(pipe["to"], pid)]
            d1 = sub(a, far_in)
            d2 = sub(far_out, b)
            # Axis-aligned tangents: unit vectors by scaling with the one nonzero component.
            d1 = mul(F(1) / max(abs(x) for x in d1), d1)
            d2 = mul(F(1) / max(abs(x) for x in d2), d2)
            assert dot(d1, d1) == 1 and dot(d2, d2) == 1 and dot(d1, d2) == 0, f"{cid}: a 90-degree bend"
            corner = add(a, mul(radius, d1))
            cross_check(f"{cid}: corner from tangent node and radius equals the specification's corner",
                        corner, corner_spec)
            cross_check(f"{cid}: far tangent node lies at R along the outgoing tangent from the corner",
                        add(corner, mul(radius, d2)), b)
            centre = add(a, mul(radius, d2))
            e1, e2 = sub(a, centre), sub(b, centre)  # radius vectors to the tangent points (length R)
            assert dot(e1, e1) == radius * radius and dot(e2, e2) == radius * radius and dot(e1, e2) == 0
            # Arc length R*theta, theta = pi/2: weight = q*pi * R*pi/2.
            weight = X.of(q * radius / 2, 2)
            # Integral of position over the arc: R * [theta*C + (sin theta) e1 + (1 - cos theta) e2],
            # with e1, e2 the radius vectors above (length R): R * [(pi/2) C + e1 + e2].
            moment = [X.of(q * radius * centre[i] / 2, 2) + X.of(q * radius * (e1[i] + e2[i]), 1) for i in range(3)]
            # Cross-check: weight times the classical centroid C + (2R sin(theta/2)/theta) * bisector.
            half = Decimal(2).sqrt() / 2  # sin(pi/4)
            bis = [Decimal((e1[i] + e2[i]).numerator) / Decimal((e1[i] + e2[i]).denominator) for i in range(3)]
            norm = sum(x * x for x in bis).sqrt()
            r_dec = Decimal(radius.numerator) / Decimal(radius.denominator)
            dist = 2 * r_dec * half / (PI_DEC / 2)
            ok = True
            for i in range(3):
                c_i = Decimal(centre[i].numerator) / Decimal(centre[i].denominator)
                classical = weight.decimal() * (c_i + dist * bis[i] / norm)
                ok = ok and abs(classical - moment[i].decimal()) <= Decimal(10) ** -80 * (1 + abs(classical))
            cross_check(f"{cid}: arc first moment equals weight times the classical arc centroid (Decimal, 1e-80)",
                        ok, True)
            length_text, kind, centroid = f"({radius})*pi/2", "arc", None
        else:
            l2 = dot(sub(b, a), sub(b, a))
            c, n = squarefree_split(l2)
            weight = X.of(q * c, 1, n)
            mid = mul(F(1, 2), add(a, b))
            moment = [weight.scale(mid[i]) for i in range(3)]
            length_text = str(c) if n == 1 else f"({c})*sqrt({n})"
            kind, centroid = "straight", [str(x) for x in mid]
        total = total + weight
        first = [first[i] + moment[i] for i in range(3)]
        acc = by_section.setdefault(sid, {"straight": X(), "arc": X()})
        acc[kind] = acc[kind] + (X.of(c, 0, n) if kind == "straight" else X.of(arcs[pid][1] / 2, 1))
        row = {"member": pid, "section": sid, "kind": kind, "length_m": length_text, "weight_N": weight.record(),
               "first_moment_N_m": vrec(moment)}
        if centroid:
            row["centroid_m"] = centroid
        members.append(row)

    comps = []
    for cid in rigid:
        geo = components[cid]["geometry"]
        w = F(geo["weight"]["value"])
        cog = vec((geo["center_of_gravity"]["x"], geo["center_of_gravity"]["y"], geo["center_of_gravity"]["z"]))
        node = components[cid]["node"]
        pipe = pipes[geo["rigid_pipe_ref"]]
        assert node in (pipe["from"], pipe["to"]), cid
        spec_w, spec_offset = SPEC_RIGID[cid]
        midpoint = mul(F(1, 2), add(nodes[pipe["from"]], nodes[pipe["to"]]))
        cross_check(f"{cid}: weight is the specification's", w, spec_w)
        cross_check(f"{cid}: centre-of-gravity vector from the node equals the specification's body midpoint "
                    f"plus offset, less the node", cog, sub(add(midpoint, spec_offset), nodes[node]))
        position = add(nodes[node], cog)
        weight = X.of(w)
        total = total + weight
        moment = [X.of(w * position[i]) for i in range(3)]
        first = [first[i] + moment[i] for i in range(3)]
        comps.append({"component": cid, "kind": components[cid]["kind"], "node": node,
                      "replaces_member": geo["rigid_pipe_ref"], "weight_N": weight.record(),
                      "center_of_gravity_m": [str(x) for x in cog], "position_m": [str(x) for x in position],
                      "first_moment_N_m": vrec(moment)})

    # Second route for the total: per section, (straight lengths + arc lengths) * mass per length * g, + components.
    alt = X()
    for sid, acc in by_section.items():
        alt = alt + (acc["straight"] + acc["arc"]).scale(sections[sid]["total"] * G).times_pi()
    for cid in rigid:
        alt = alt + X.of(F(components[cid]["geometry"]["weight"]["value"]))
    cross_check("total weight: per-section length sums times mass per length, plus component weights", alt, total)

    # The weight is F = (0, 0, -w) at r: its moment about the origin r x F = (-w*y, w*x, 0).
    weight_moment = [-first[1], first[0], X()]
    section_rows = [{"section": sid, "mass_per_length_kg_m": {
        "metal": X.of(m["metal"], 1).record(), "contents": X.of(m["contents"], 1).record(),
        "insulation": X.of(m["insulation"], 1).record(), "total": X.of(m["total"], 1).record()},
        "weight_per_length_N_m": X.of(m["total"] * G, 1).record()} for sid, m in sorted(sections.items())]
    key = {"W_total_N": total.record(),
           "weight_resultant_N": vrec([X(), X(), -total]),
           "weight_first_moment_N_m": vrec(first),
           "weight_moment_about_origin_N_m": vrec(weight_moment)}
    return {"sections": section_rows, "members": members, "components": comps, "key_values": key}


def document(result):
    return {
        "document": "swbpipe.reference_system_references",
        "status": "candidate frozen references; derived and cross-checked by the author TASK, not yet independently "
                  "refuted",
        "specification": "execution/PKG-09/DEL-09-01/Work/reference-system.md §9.2 (EQ-1, EQ-2)",
        "inputs": f"{FIXTURE_REL}: nodes, pipe_segments (sections), components; load_cases and the self-weight "
                  "capture are not read",
        "units": {"force": "N", "moment": "N*m", "length": "m", "mass_per_length": "kg/m"},
        "gravity": {"value": str(G), "unit": "m/s^2", "direction": "-Z"},
        "mass_bases": {
            "pipe_metal": "rho_m * pi * (R^2 - (R - t)^2), the nominal wall",
            "contents": "rho_c * pi * (R - ((t - c_m) - c_a))^2, the reduced (pressure) bore",
            "insulation": "rho_i * pi * ((R + t_i)^2 - R^2), OD to OD + 2 t_i",
            "realized_arc": "length R * theta, theta = pi/2",
            "rigid_family": "the component's weight at node + centre-of-gravity vector (global axes), in place of "
                            "its mapped member's distributed weight",
            "replaced_span": "RS-P180 carries no weight",
        },
        "conventions": {
            "weight_moment_about_origin": "sum of r x F over the weight, F = (0, 0, -W) at r; equals "
                                          "(-S_y, S_x, 0) with S the weight first moment sum(W * r)",
            "exact": "a sum of (q)*pi^k*sqrt(n) terms with q rational",
            "pi": "Machin's formula; rational bounds 90 decimal places apart",
            "sqrt": "integer square root; rational bounds 90 decimal places apart",
        },
        "checks": {
            "EQ-1": "in RS-SUS, RS-OPE and RS-SAM, the sum of the vertical (global Z) reaction components over "
                    "every support equals W_total",
            "EQ-2": "in RS-SUS, RS-OPE and RS-SAM, the horizontal reaction resultant (X, Y) is zero, and the "
                    "reaction moment resultant about the origin equals minus weight_moment_about_origin",
        },
        "criterion": "|observed - expected| <= 1e-9 * |expected|; an exactly zero expectation uses 1e-9 * zero_scale "
                     "of its family (largest nonzero magnitude of that family in the case)",
        "crosscheck": list(CROSS),
        "key_values": result["key_values"],
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


def sha256sums(files):
    lines = [f"{hashlib.sha256(data).hexdigest()}  {path}" for path, data in sorted(files.items())]
    return ("\n".join(lines) + "\n").encode()


def build_files():
    result = build()
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
    key = result["key_values"]
    print(f"W_total {key['W_total_N']['value']!r} N")
    print("weight moment about origin", [v["value"] for v in key["weight_moment_about_origin_N_m"]], "N*m")
    if args.write:
        if bad:
            print("not written: a cross-check disagrees")
            return 1
        for path, data in files.items():
            if path in FIXED_FILES:
                continue
            target = OUT / path
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(data)
        print(f"wrote {len(files) - len([f for f in FIXED_FILES if f in files])} files")
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
