#!/usr/bin/env python3
"""RT-G5 S2 independent references: condition-derived temperature differences
(E3) and closed-form mechanics of condition-driven load cases.

Standard-library Python 3 only. This generator imports, reads and runs no
product code and uses no product output as an expected value; it reads only its
own directory's committed files (to hash them) and writes `references.json` and
`SHA256SUMS`. Definitions: DEL-02-01 Work `model-document-0.9.0.md` sections
1.5, 1.5.1 and 1.8.

E3. For absolute temperatures T (unit u) and T0 (unit u0), each an authored
binary64:

  dT = RN(K(T) - K(T0))  in kelvin,

where K is the exact rational kelvin value of the operand's shortest round-trip
decimal under the unit's exact affine definition (K: x; degC: x + 273.15;
degF: (x + 459.67)*5/9; degR: x*5/9), and RN rounds once to binary64, to
nearest, ties to even; an exact zero is +0.0.

Two routes compute every row and must agree bit for bit:

  route A (the reference): fractions.Fraction(repr(v)) for the decimal (Python's
    repr is the shortest round-trip decimal, closest, ties to the even digit,
    which is the canonical decimal's rule) and float(Fraction), which CPython
    rounds correctly (integer true division);
  route B (the cross-check): a transcription of section 1.5.1's fixed-integer
    algorithm, decimal stage included, written from the design text; it asserts
    every intermediate of the decimal stage is below 2^192 and the rounding's
    shifted numerator below 2^136.

Mechanics. Closed-form textbook results (axial restraint and free expansion of
a straight member, Euler-Bernoulli cantilever under a uniform line load) for
invented inputs, in exact rational arithmetic with pi as a 60-digit rational,
converted to binary64 once. The product test compares its published rows with
these at a relative tolerance stated in the file.

Modes:
  python3 -I rt_g5_s2_reference.py           check: rebuild every output in memory and
                                              compare byte for byte with the committed
                                              files, SHA256SUMS included; when `node` is
                                              on PATH, also compare the decimal samples
                                              with ECMAScript Number#toString
  python3 -I rt_g5_s2_reference.py --write   write the outputs (intentional changes only)
"""
import argparse
import hashlib
import json
import random
import shutil
import struct
import subprocess
import sys
from decimal import Decimal
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE.parent
FIXED_FILES = ("README.md", "generators/rt_g5_s2_reference.py")

RANGE = "OPERATING_CONDITION_TEMPERATURE_RANGE_UNSUPPORTED"
INVALID = "OPERATING_CONDITION_QUANTITY_INVALID"

PI = F("3.14159265358979323846264338327950288419716939937510582097494")


# ----------------------------------------------------------------------------- binary64
def bits_of(value):
    return struct.unpack(">Q", struct.pack(">d", value))[0]


def of_bits(bits):
    return struct.unpack(">d", struct.pack(">Q", bits))[0]


def hex_bits(value):
    return f"{bits_of(value):016x}"


# ----------------------------------------------------------------------------- route A
AFFINE = {
    "K": (F(1), F(0)),
    "degC": (F(1), F(27315, 100)),
    "degF": (F(5, 9), F(45967, 100) * F(5, 9)),
    "degR": (F(5, 9), F(0)),
}


def decimal_exponent(text):
    """e of the decimal m*10^e with m not divisible by 10 (0 for zero)."""
    d = Decimal(text)
    if d == 0:
        return 0
    return d.normalize().as_tuple().exponent


def kelvin_a(value, unit):
    scale, offset = AFFINE[unit]
    return F(repr(value)) * scale + offset


def admit_a(value, unit):
    """None when admitted, else the refusal code (route A's own reading)."""
    scale, offset = AFFINE[unit]
    in_domain = value == 0 or (abs(value) <= 1e6 and decimal_exponent(repr(value)) >= -24)
    # Below zero kelvin is invalid whatever the magnitude; outside the domain the
    # binary value's sign decides, which is not in doubt there.
    exact = F(repr(value)) if in_domain else F(value)
    if exact * scale + offset < 0:
        return INVALID
    return None if in_domain else RANGE


def difference_a(t, tu, t0, t0u):
    return float(kelvin_a(t, tu) - kelvin_a(t0, t0u))


# ----------------------------------------------------------------------------- route B
B192 = 1 << 192
B136 = 1 << 136


class OutOfRange(Exception):
    pass


def canonical_b(v, formatter=False):
    """Section 1.5.1 steps 1-5. Returns (negative, m, e). With formatter=True an
    exact tie between the two candidates takes the odd digit (the other candidate),
    which is the choice the design attributes to Rust's `{:e}`."""
    if v == 0:
        return (False, 0, 0)
    bits = bits_of(abs(v))
    biased = bits >> 52
    frac = bits & ((1 << 52) - 1)
    if biased == 0:
        raise OutOfRange("subnormal")
    B = frac | (1 << 52)
    E = biased - 1075
    # Step 1, the pre-gate 2^-80 <= |v| <= 2^20, on the exact value B*2^E.
    if not (F(1, 2 ** 80) <= F(B) * F(2) ** E <= F(2) ** 20):
        raise OutOfRange("pre-gate")
    assert -132 <= E <= -32
    # The admitted operand range: |v| <= 10^6 in its own unit (B*2^E <= 10^6).
    if B > 10 ** 6 * 2 ** (-E):
        raise OutOfRange("above 10^6")
    # Step 2: largest p with 10^p <= |v|, by integer comparison of
    # 10^max(0,p) * 2^-E against B * 10^max(0,-p).
    p = None
    for cand in range(6, -26, -1):
        left = 10 ** max(0, cand) * 2 ** (-E)
        right = B * 10 ** max(0, -cand)
        assert left < B192 and right < B192
        if left <= right:
            p = cand
            break
    assert p is not None and -25 <= p <= 6
    even_b = B % 2 == 0
    for n in range(1, 18):
        q = p - n + 1
        fine = 10 ** max(0, -q)
        if q <= 0:
            d0 = (B * 10 ** (-q)) // (2 ** (-E))
        else:
            d0 = B // (2 ** (-E) * 10 ** q)
        # Scale S = 2^(2-E) * 10^max(0,-q).
        vS = 4 * B * fine
        up = 2 * fine
        low = fine if B == 1 << 52 else 2 * fine
        lower_end, upper_end = vS - low, vS + up

        def xs(d):
            return d * 10 ** max(0, q) * 2 ** (2 - E)

        def trips(x):
            if lower_end < x < upper_end:
                return True
            return (x == lower_end or x == upper_end) and even_b

        cands = []
        for d in (d0, d0 + 1):
            x = xs(d)
            for value in (x, vS, lower_end, upper_end, d, 10 ** max(0, -q), 2 ** (2 - E)):
                assert 0 <= value < B192, "decimal stage bound"
            if trips(x):
                cands.append((d, x))
        if not cands:
            continue
        if len(cands) == 2:
            (da, xa), (db, xb) = cands
            ga, gb = vS - xa, xb - vS
            if ga < gb:
                d = da
            elif gb < ga:
                d = db
            else:
                tie_pick_even = da if da % 2 == 0 else db
                d = (db if tie_pick_even == da else da) if formatter else tie_pick_even
        else:
            d = cands[0][0]
        e = q
        while d % 10 == 0:
            d //= 10
            e += 1
        if e < -24:
            raise OutOfRange("exponent")
        return (v < 0, d, e)
    raise AssertionError("17 digits always round-trip")


D_DEN = 9 * 10 ** 24
OFFSET = 10 ** 22


def numerator_b(decimal, unit):
    negative, m, e = decimal
    M = m * 10 ** (e + 24)
    if negative:
        M = -M
    assert abs(M) <= 10 ** 30
    if unit == "K":
        return 9 * M
    if unit == "degC":
        return 9 * (M + 27315 * OFFSET)
    if unit == "degF":
        return 5 * (M + 45967 * OFFSET)
    if unit == "degR":
        return 5 * M
    raise ValueError(unit)


def round_b(N):
    """Section 1.5.1 Rounding, on integers; returns the binary64."""
    if N == 0:
        return 0.0
    a = abs(N)
    assert a < 1 << 105
    assert D_DEN.bit_length() == 83
    k = a.bit_length() - 83 - 53
    assert k < 0

    def divide(k):
        shifted = a << (-k)
        assert shifted < B136, "rounding bound"
        return divmod(shifted, D_DEN)

    q, r = divide(k)
    if q >= 1 << 53:
        k += 1
        q, r = divide(k)
    assert 2 * r < 1 << 84
    if 2 * r > D_DEN or (2 * r == D_DEN and q % 2 == 1):
        q += 1
    if q == 1 << 53:
        q = 1 << 52
        k += 1
    value = float(F(q) * F(2) ** k)  # q < 2^53 and a power-of-two scale: exact
    return -value if N < 0 else value


def difference_b(t, tu, t0, t0u, formatter=False):
    n = numerator_b(canonical_b(t, formatter), tu) - numerator_b(canonical_b(t0, formatter), t0u)
    return round_b(n), n


def shifted_numerator_b(N):
    a = abs(N)
    k = a.bit_length() - 136
    shifted = a << (-k)
    if shifted // D_DEN >= 1 << 53:
        k += 1
        shifted = a << (-k)
    return k, shifted


# ----------------------------------------------------------------------------- E3 rows
def decimal_text(decimal):
    negative, m, e = decimal
    return f"{'-' if negative else ''}{m}e{e}"


def operand(value, unit):
    return {"value": value, "bits": hex_bits(value), "unit": unit}


def row(rid, covers, t, tu, t0, t0u, **extra):
    refusal = admit_a(t, tu) or admit_a(t0, t0u)
    out = {"id": rid, "covers": covers, "temperature": operand(t, tu), "installation": operand(t0, t0u)}
    # Route B must agree on admission.
    try:
        refusal_b = None
        for v, u in ((t, tu), (t0, t0u)):
            n = numerator_b(canonical_b(v), u)
            if n < 0:
                refusal_b = refusal_b or INVALID
    except OutOfRange:
        below = any(F(v) * AFFINE[u][0] + AFFINE[u][1] < 0 for v, u in ((t, tu), (t0, t0u)))
        refusal_b = INVALID if below else RANGE
    if refusal is not None:
        assert refusal_b == refusal or (refusal == INVALID and refusal_b is not None), (rid, refusal, refusal_b)
        out["refused"] = refusal
        return out
    assert refusal_b is None, (rid, refusal_b)
    a = difference_a(t, tu, t0, t0u)
    b, n = difference_b(t, tu, t0, t0u)
    assert bits_of(a) == bits_of(b), (rid, a, b)
    out["numerators"] = {
        "temperature": str(numerator_b(canonical_b(t), tu)),
        "installation": str(numerator_b(canonical_b(t0), t0u)),
        "difference": str(n),
        "denominator": str(D_DEN),
    }
    out["canonical_decimals"] = [decimal_text(canonical_b(t)), decimal_text(canonical_b(t0))]
    out["expected"] = {"bits": hex_bits(a), "value": a}
    out.update(extra)
    return out


def nextafter_up(v):
    return of_bits(bits_of(v) + 1)


def near_halfway_rows():
    """Admitted pairs whose exact difference lies closest to a binary64 midpoint
    (a seeded search over typed-looking decimals); an exact tie cannot occur."""
    rng = random.Random(20261101)
    found = []
    units = ["K", "degC", "degF", "degR"]
    for _ in range(40000):
        tu, t0u = rng.choice(units), rng.choice(units)
        t = float(f"{rng.randint(0, 9_999_999)}e-{rng.randint(1, 5)}")
        t0 = float(f"{rng.randint(0, 9_999_999)}e-{rng.randint(1, 5)}")
        if admit_a(t, tu) or admit_a(t0, t0u):
            continue
        exact = kelvin_a(t, tu) - kelvin_a(t0, t0u)
        if exact == 0:
            continue
        r = float(exact)
        # distance of the exact value to the midpoint on its side, in ulps of r
        ulp = abs(F(nextafter_up(abs(r))) - F(abs(r)))
        below = F(of_bits(bits_of(abs(r)) - 1)) if abs(r) > 0 else F(0)
        mid_up = (F(abs(r)) + F(nextafter_up(abs(r)))) / 2
        mid_down = (F(abs(r)) + below) / 2
        distance = min(abs(abs(exact) - mid_up), abs(abs(exact) - mid_down)) / ulp
        found.append((distance, t, tu, t0, t0u))
    found.sort(key=lambda item: item[0])
    rows = []
    for i, (distance, t, tu, t0, t0u) in enumerate(found[:8]):
        rows.append(row(f"near_halfway_{i}", f"exact difference within {float(distance):.3e} ulp of a midpoint", t, tu, t0, t0u))
    return rows


def e3_rows():
    rows = []
    add = rows.append
    # Each unit, and mixed units.
    add(row("kelvin", "K - K", 300.0, "K", 293.15, "K"))
    add(row("celsius_typed_decimals", "20.3 degC - 20.1 degC: the binary64 nearest 0.2",
            20.3, "degC", 20.1, "degC",
            binary64_affine_route=((20.3 + 273.15) - (20.1 + 273.15))))
    add(row("celsius_operating", "150 degC - 20 degC", 150.0, "degC", 20.0, "degC"))
    add(row("fahrenheit_212_70", "212 degF - 70 degF (section 1.8 example)", 212.0, "degF", 70.0, "degF",
            binary64_affine_route=((212.0 + 459.67) * (5.0 / 9.0) - (70.0 + 459.67) * (5.0 / 9.0))))
    add(row("fahrenheit_77_70", "77 degF - 70 degF: the degF/degR disclosure row", 77.0, "degF", 70.0, "degF",
            typed_interval={"value": 7.0, "unit": "degF", "bits": hex_bits(7.0 * (5.0 / 9.0)),
                            "basis": "a typed 7 degF interval: value x fl(5/9), the unchanged typed route"}))
    add(row("rankine", "degR - degR", 671.67, "degR", 529.67, "degR"))
    add(row("mixed_celsius_fahrenheit", "150 degC - 68 degF", 150.0, "degC", 68.0, "degF"))
    add(row("mixed_kelvin_celsius", "373.15 K - 20 degC", 373.15, "K", 20.0, "degC"))
    add(row("mixed_rankine_celsius", "500 degR - 0 degC", 500.0, "degR", 0.0, "degC"))
    add(row("mixed_fahrenheit_kelvin", "350.1 degF - 255.3 K", 350.1, "degF", 255.3, "K"))
    # Equal temperatures, and negative differences.
    add(row("equal_same_unit", "20 degC - 20 degC: +0.0", 20.0, "degC", 20.0, "degC"))
    add(row("equal_mixed_units", "293.15 K - 20 degC: exactly equal, +0.0", 293.15, "K", 20.0, "degC"))
    add(row("equal_fahrenheit_celsius", "68 degF - 20 degC: exactly equal, +0.0", 68.0, "degF", 20.0, "degC"))
    add(row("negative_zero_operand", "-0.0 K - 0 K: +0.0", -0.0, "K", 0.0, "K"))
    add(row("negative_difference", "10 degC - 20 degC", 10.0, "degC", 20.0, "degC"))
    add(row("negative_mixed", "-40 degF - 20 degC", -40.0, "degF", 20.0, "degC"))
    # Range edges (admitted).
    add(row("edge_upper_kelvin", "|v| = 10^6 K", 1e6, "K", 0.0, "K"))
    add(row("edge_upper_celsius_both", "10^6 degC - (-273.15 degC)", 1e6, "degC", -273.15, "degC"))
    add(row("edge_zero_kelvin_fahrenheit", "-459.67 degF is exactly 0 K", 100.0, "K", -459.67, "degF"))
    add(row("width_regression", "1e-24 K - 0 K: N = 9, shifted numerator 9*2^132 above i128", 1e-24, "K", 0.0, "K"))
    add(row("smallest_negative", "1e-24 K - 2e-24 K", 1e-24, "K", 2e-24, "K"))
    add(row("edge_exponent_with_upper", "1e6 K - 1e-24 K", 1e6, "K", 1e-24, "K"))
    # The decimal tie regression.
    tie = of_bits(0x3F80010000000000)
    formatter_value, _ = difference_b(tie, "K", 0.007, "K", formatter=True)
    add(row("decimal_tie_regression", "binary64 0x3f80010000000000 K - 0.007 K: canonical decimal 0.007814407348632812",
            tie, "K", 0.007, "K",
            formatter_decimal=decimal_text(canonical_b(tie, formatter=True)),
            formatter_route_bits=hex_bits(formatter_value)))
    # Refusals.
    add(row("refused_above_upper", "next binary64 above 10^6 K", nextafter_up(1e6), "K", 0.0, "K"))
    add(row("refused_above_upper_decimal", "1.000001e6 K", 1.000001e6, "K", 0.0, "K"))
    add(row("refused_exponent", "1.5e-24 K: e = -25", 1.5e-24, "K", 0.0, "K"))
    add(row("refused_subnormal", "1e-310 K", 1e-310, "K", 0.0, "K"))
    add(row("refused_installation_range", "an out-of-range installation temperature", 20.0, "degC", 2.0e6, "degC"))
    add(row("refused_below_zero_kelvin", "-273.16 degC is below zero kelvin", -273.16, "degC", 20.0, "degC"))
    add(row("refused_below_zero_out_of_range", "-2e6 degC: invalid whatever the magnitude", -2.0e6, "degC", 20.0, "degC"))
    add(row("refused_below_zero_rankine", "-1 degR", -1.0, "degR", 20.0, "degC"))
    rows.extend(near_halfway_rows())
    return rows


# ----------------------------------------------------------------------------- decimal samples
def decimal_samples():
    """Seeded random admitted binary64 values with Python repr, and the tie rows
    (both candidates round-trip and are equally close) where the formatter's
    choice differs from the canonical decimal."""
    rng = random.Random(1501)
    samples = []
    while len(samples) < 400:
        biased = rng.randint(943, 1042)  # 2^-80 .. below 2^20
        bits = (biased << 52) | rng.getrandbits(52)
        if rng.random() < 0.5:
            bits |= 1 << 63
        v = of_bits(bits)
        try:
            dec = canonical_b(v)
        except OutOfRange:
            continue
        if abs(v) > 1e6:
            continue
        r = repr(v)
        assert Decimal(r) == Decimal(f"{'-' if dec[0] else ''}{dec[1]}E{dec[2]}"), (hex_bits(v), r, dec)
        samples.append({"bits": hex_bits(v), "repr": r})
    ties = []
    for bits in (0x3F80010000000000, 0x3F50640000000000, 0x3E60000000000000):
        v = of_bits(bits)
        ties.append({"bits": f"{bits:016x}", "canonical": decimal_text(canonical_b(v)), "repr": repr(v),
                     "formatter": decimal_text(canonical_b(v, formatter=True))})
        assert Decimal(repr(v)) == Decimal(decimal_text(canonical_b(v)).replace("e", "E"))
    return samples, ties


def node_check(samples, ties):
    node = shutil.which("node")
    if node is None:
        return "node not on PATH: Number#toString comparison not run"
    values = [s["bits"] for s in samples] + [t["bits"] for t in ties]
    script = (
        "const v=JSON.parse(require('fs').readFileSync(0,'utf8'));"
        "const b=new DataView(new ArrayBuffer(8));"
        "console.log(JSON.stringify(v.map(h=>{b.setBigUint64(0,BigInt('0x'+h));return b.getFloat64(0).toString();})));"
    )
    out = subprocess.run([node, "-e", script], input=json.dumps(values), capture_output=True, text=True, check=True)
    texts = json.loads(out.stdout)
    for h, text in zip(values, texts):
        assert Decimal(text) == Decimal(repr(of_bits(int(h, 16)))), (h, text)
    return f"Number#toString agreed with repr on {len(values)} values"


# ----------------------------------------------------------------------------- mechanics
def mechanics():
    """Closed forms for invented inputs. SI units (m, N, Pa, K)."""
    L = F(5)
    od, wall = F(2, 10), F(1, 100)
    E = F(200) * 10 ** 9
    alpha = F(12, 10 ** 6)  # 1/K (entered as 1/degC: an interval factor of 1)
    d_in = od - 2 * wall
    A = PI / 4 * (od ** 2 - d_in ** 2)
    I = PI / 64 * (od ** 4 - d_in ** 4)
    common = {
        "length_m": "5", "outside_diameter_m": "0.2", "wall_thickness_m": "0.01",
        "elastic_modulus_pa": "2e11", "shear_modulus_pa": "8e10", "thermal_expansion_per_degc": "1.2e-5",
    }

    def f(x):
        return float(x)

    cases = []
    # M1: restrained thermal, anchors at both ends, temperature from conditions.
    dT1 = kelvin_a(150.0, "degC") - kelvin_a(20.0, "degC")
    force = E * A * alpha * dT1
    cases.append({
        "id": "restrained_thermal",
        "covers": "a straight pipe between two anchors; 150 degC operating, 20 degC installation from a condition set",
        "conditions": {"temperature": {"value": 150.0, "unit": "degC"}, "installation": {"value": 20.0, "unit": "degC"}},
        "expected": {
            "temperature_change_k": f(dT1),
            "axial_force_n_magnitude": f(force),
            "axial_stress_pa_magnitude": f(E * alpha * dT1),
            "reaction_x_at_start_n": f(force),
            "reaction_x_at_end_n": f(-force),
            "midspan_displacement_x_m": 0.0,
        },
        "formulas": "N = E*A*alpha*dT (compressive); sigma = E*alpha*dT; anchors push the pipe inward: R_start = +N, R_end = -N",
    })
    # M2: free expansion of a cantilever, temperatures in degF.
    dT2 = kelvin_a(212.0, "degF") - kelvin_a(70.0, "degF")
    cases.append({
        "id": "free_expansion",
        "covers": "a cantilever anchored at its start; 212 degF operating, 70 degF installation",
        "conditions": {"temperature": {"value": 212.0, "unit": "degF"}, "installation": {"value": 70.0, "unit": "degF"}},
        "expected": {
            "temperature_change_k": f(dT2),
            "tip_displacement_x_m": f(alpha * dT2 * L),
            "reaction_x_at_start_n": 0.0,
        },
        "formulas": "delta = alpha*dT*L; no restraint, no force",
    })
    # M3: added weight from the set on a cantilever (Euler-Bernoulli).
    w = F(-1500)  # N/m along global_z, entered by the user
    cases.append({
        "id": "added_weight_cantilever",
        "covers": "a cantilever under its set's added weight -1500 N/m along global_z",
        "conditions": {"added_weight": {"magnitude": {"value": -1500.0, "unit": "N/m"}, "direction": "global_z"}},
        "expected": {
            "tip_displacement_z_m": f(w * L ** 4 / (8 * E * I)),
            "tip_rotation_y_rad": f(-w * L ** 3 / (6 * E * I)),
            "reaction_z_at_start_n": f(-w * L),
        },
        "formulas": "v_tip = w*L^4/(8EI); theta_y,tip = -w*L^3/(6EI) (right-handed, z up, x along the pipe); R_z = -w*L",
    })
    # M4: wind exposure from the set on a cantilever.
    p, C = F(1200), F(7, 10)
    q = p * C * od
    cases.append({
        "id": "wind_cantilever",
        "covers": "a cantilever whose set is wind-exposed; 1200 Pa, shape factor 0.7, global_y, no insulation",
        "conditions": {"wind": {"pressure_pa": 1200.0, "shape_factor": 0.7, "direction": "global_y"}},
        "expected": {
            "line_load_n_per_m": f(q),
            "tip_displacement_y_m": f(q * L ** 4 / (8 * E * I)),
            "reaction_y_at_start_n": f(-q * L),
        },
        "formulas": "q = p*C*OD; v_tip = q*L^4/(8EI); R_y = -q*L",
    })
    return {
        "inputs": common,
        "derived": {"area_m2": f(A), "second_moment_m4": f(I)},
        "relative_tolerance": 1e-9,
        "absolute_floor": {"displacement_m": 1e-15, "force_n": 1e-6},
        "cases": cases,
    }


# ----------------------------------------------------------------------------- outputs
def build():
    samples, ties = decimal_samples()
    references = {
        "schema": "rt_g5_s2_condition_references/1",
        "basis": "DEL-02-01 Work model-document-0.9.0.md sections 1.5, 1.5.1, 1.8; generated by generators/rt_g5_s2_reference.py",
        "status": "candidate references; not independently refuted",
        "temperature_differences": e3_rows(),
        "decimal_samples": samples,
        "decimal_ties": ties,
        "mechanics": mechanics(),
    }
    files = {name: (OUT / name).read_bytes() for name in FIXED_FILES}
    files["references.json"] = (json.dumps(references, indent=1, sort_keys=False) + "\n").encode("utf-8")
    sums = "".join(f"{hashlib.sha256(files[name]).hexdigest()}  {name}\n" for name in sorted(files))
    files["SHA256SUMS"] = sums.encode("ascii")
    return files, samples, ties


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    files, samples, ties = build()
    print(node_check(samples, ties))
    if args.write:
        for name in ("references.json", "SHA256SUMS"):
            (OUT / name).write_bytes(files[name])
        print("wrote references.json and SHA256SUMS")
        return 0
    stale = [name for name, data in files.items() if not (OUT / name).exists() or (OUT / name).read_bytes() != data]
    if stale:
        print("stale or missing: " + ", ".join(stale))
        return 1
    print("references.json and SHA256SUMS are current")
    return 0


if __name__ == "__main__":
    sys.exit(main())
