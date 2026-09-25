#!/usr/bin/env python3
"""CP2_EXTENSION: additive extension of reference_cases.json.

Author: the CP2_EXTENSION Type 2 TASK. It is not the implementer of the
thermal/material kernels. Standard library only (fractions, decimal, json,
hashlib). It imports, reads and runs no product kernel, product test, fixture
generator or manager checker. Every expected value is derived here from the
stated invented inputs by exact closed forms (Fraction). The logarithmic form
uses Decimal.exp at precision 85, the same convention as the existing log
controls.

Usage: python3 extend_reference_cases.py <reference_cases.json>

The script refuses to run unless the input bytes have the reviewed pre-edit
sha256. It adds only:
  cases.multi_segment_free_length                        (new case)
  cases.thermal_datum_ratio.variants.verification_two_point (new variant)
  cases.temperature_unit_identity                        (new case)
It re-serialises with json.dumps(indent=2, allow_nan=False) + "\\n", under
which the pre-edit bytes round-trip byte-identically (asserted before writing).
"""
from decimal import Decimal as D, getcontext, localcontext
from fractions import Fraction as F
import hashlib
import json
import sys

PRE_SHA256 = "5478bba846bcc515187f38366eeda3ac9fb87b339039a5cf3ec2d56dced4b891"
getcontext().prec = 85


def dec(q):
    q = F(q)
    return D(q.numerator) / D(q.denominator)


def quantity(q, unit="1", pi=None):
    """Existing convention: exact authoritative, decimal at prec 85, binary64 value."""
    q = F(q)
    if pi is None:
        value = dec(q)
        exact = {"kind": "rational", "rational": str(q)}
        projected = float(q)  # correctly rounded from the exact rational
        assert projected == float(value), (q, projected, value)
    else:
        value = dec(q) * pi
        exact = {"kind": "rational_times_pi", "rational": str(q)}
        projected = float(value)
    return {"unit": unit, "exact": exact, "decimal": str(value), "value": projected}


def expm1_symbolic(x, unit="1"):
    """exp(x)-1 for exact rational x, evaluated as the existing log controls do."""
    x = F(x)
    with localcontext() as ctx:
        ctx.prec = 85
        value = dec(x).exp() - 1
    return {"unit": unit, "exact": {"kind": "symbolic", "expression": f"exp({x})-1"},
            "decimal": str(value), "value": float(value),
            "evaluation": "Decimal.exp, precision 85"}


# ---------------------------------------------------------------- piecewise-linear tables
def interpolate(points, T):
    """Linear interpolation inside coverage only; exact table points returned as-is."""
    for (t0, y0), (t1, y1) in zip(points, points[1:]):
        assert t0 < t1, "strictly increasing table required"
        if t0 <= T <= t1:
            return y0 + (y1 - y0) * (T - t0) / (t1 - t0)
    raise ValueError(f"{T} outside table coverage {points[0][0]}..{points[-1][0]}")


def segment_pieces(points, a, b):
    """Ordered pieces of [min(a,b), max(a,b)] per table segment: (i, j, lo, hi)."""
    lo, hi = min(a, b), max(a, b)
    interpolate(points, lo), interpolate(points, hi)  # coverage check, no extrapolation
    pieces = []
    for i, ((t0, _), (t1, _)) in enumerate(zip(points, points[1:])):
        s, e = max(lo, t0), min(hi, t1)
        if s < e:
            pieces.append((i, i + 1, s, e))
    return pieces


def trapezoid(points, s, e):
    """Exact integral of a linear coefficient over [s, e] inside one segment."""
    return (interpolate(points, s) + interpolate(points, e)) * (e - s) / 2


def integral(points, a, b):
    """I(a,b)=integral_a^b alpha dT, signed; sum of exact per-segment trapezoids."""
    total = sum((trapezoid(points, s, e) for _, _, s, e in segment_pieces(points, a, b)), F(0))
    return total if b >= a else -total


# ---------------------------------------------------------------- 1. multi_segment_free_length
K = "K"
Tm, Ti, To, Ts = F(300), F(350), F(550), F(450)
alpha_pts = [(F(300), F(1, 100000)), (F(400), F(2, 100000)), (F(500), F(1, 100000)), (F(600), F(3, 100000))]
dil_pts = [(F(300), F(0)), (F(400), F(1, 1000)), (F(500), F(15, 10000)), (F(600), F(35, 10000))]

I_i, I_s, I_o = integral(alpha_pts, Tm, Ti), integral(alpha_pts, Tm, Ts), integral(alpha_pts, Tm, To)
I_io = integral(alpha_pts, Ti, To)
assert I_io == I_o - I_i and integral(alpha_pts, To, Ti) == -I_io


def lam_datum(T):
    return 1 + integral(alpha_pts, Tm, T)


def eps_datum(a, b):
    return lam_datum(b) / lam_datum(a) - 1


def lam_dil(T):
    return 1 + interpolate(dil_pts, T)


def eps_dil(a, b):
    return lam_dil(b) / lam_dil(a) - 1


assert interpolate(dil_pts, Tm) == 0, "dilation table datum rule: d(T_m)=0"

# Composition and reversal identities of the derivation itself.
for eps in (eps_datum, eps_dil):
    assert (1 + eps(Ti, Ts)) * (1 + eps(Ts, To)) == 1 + eps(Ti, To)
    assert (1 + eps(To, Ts)) * (1 + eps(Ts, Ti)) == 1 + eps(To, Ti)
    assert (1 + eps(Ti, To)) * (1 + eps(To, Ti)) == 1
assert integral(alpha_pts, Ti, Ts) + integral(alpha_pts, Ts, To) == I_io

pieces_fwd = segment_pieces(alpha_pts, Ti, To)
assert len(pieces_fwd) == 3, "must consume three table segments"
assert all(Ti != t and To != t for t, _ in alpha_pts), "install/operate must be interior"

# Wrong-result discriminators (forward only; see definitions).
first_i, first_j, first_s, first_e = pieces_fwd[0]
last_i, last_j, last_s, last_e = pieces_fwd[-1]
first_piece = trapezoid(alpha_pts, first_s, first_e)
last_piece = trapezoid(alpha_pts, last_s, last_e)
lam_i = lam_datum(Ti)
# datum-integral form: I(T_m, T) truncated to its last segment.
datum_path_last = segment_pieces(alpha_pts, Tm, To)[-1]
I_o_last_only = trapezoid(alpha_pts, datum_path_last[2], datum_path_last[3])
datum_path_first = segment_pieces(alpha_pts, Tm, To)[0]
I_o_first_only = trapezoid(alpha_pts, datum_path_first[2], datum_path_first[3])
assert len(segment_pieces(alpha_pts, Tm, Ti)) == 1  # T_install lies in the first segment
# Dilation analogues.
d = lambda T: interpolate(dil_pts, T)
dil_first = d(alpha_pts[first_j][0]) - d(Ti)
dil_last = d(To) - d(alpha_pts[last_i][0])
d_o_last_only = d(To) - d(dil_pts[datum_path_last[0]][0])
d_o_first_only = d(dil_pts[datum_path_first[1]][0]) - d(Tm)
wrong_first_datum_form = (1 + I_o_first_only) / lam_i - 1
assert wrong_first_datum_form == first_piece / lam_i  # both first-only forms coincide here
assert (1 + d_o_first_only) / lam_dil(Ti) - 1 == dil_first / lam_dil(Ti)
skip_interior = (interpolate(alpha_pts, Ti) + interpolate(alpha_pts, To)) * (To - Ti) / 2
endpoint_alpha = interpolate(alpha_pts, To) * (To - Ti)


def kq(T):
    return quantity(T, K)


multi_segment = {
    "source": {
        "document": "VERIFICATION.md",
        "control": 5,
        "extension": "Control 5 extended from one consumed table segment to three (CP1 independent review SF1). Inputs are invented by the CP2_EXTENSION TASK; they are not reviewed design values.",
    },
    "equations": [
        "alpha(T) and d(T) are piecewise linear between table points; no extrapolation",
        "I(a,b)=integral_a^b alpha dT = signed sum over consumed segments of (alpha(start)+alpha(end))*(end-start)/2",
        "differential_per_datum_length: lambda(T)=1+I(T_m,T)",
        "logarithmic_per_current_length: lambda(T)=exp(I(T_m,T)), so eps(a->b)=exp(I(a,b))-1",
        "engineering_dilation: lambda(T)=1+d(T), d(T_m)=0",
        "eps(a->b)=lambda(b)/lambda(a)-1",
        "(1+eps(a->s))*(1+eps(s->b))=1+eps(a->b)",
        "(1+eps(a->b))*(1+eps(b->a))=1",
    ],
    "variants": {
        "linear_coefficient_table": {
            "inputs": {
                "provenance": "invented",
                "definitions": ["differential_per_datum_length", "logarithmic_per_current_length"],
                "interpolation": "linear_coefficient",
                "datum_temperature": kq(Tm),
                "installation_temperature": kq(Ti),
                "operating_temperature": kq(To),
                "split_temperature": kq(Ts),
                "linear_coefficient_points": [
                    {"temperature": kq(t), "coefficient": quantity(a, "1/K")} for t, a in alpha_pts],
            },
            "expected": {
                "coefficient_install": quantity(interpolate(alpha_pts, Ti), "1/K"),
                "coefficient_split": quantity(interpolate(alpha_pts, Ts), "1/K"),
                "coefficient_operating": quantity(interpolate(alpha_pts, To), "1/K"),
                "integral_datum_to_install": quantity(I_i),
                "integral_datum_to_split": quantity(I_s),
                "integral_datum_to_operating": quantity(I_o),
                "integral_install_to_operating": quantity(I_io),
                "segment_integrals_install_to_operating": [
                    {"table_points": [i, j], "from": kq(s), "to": kq(e), "integral": quantity(trapezoid(alpha_pts, s, e))}
                    for i, j, s, e in pieces_fwd],
                "datum_length_strain_forward": quantity(eps_datum(Ti, To)),
                "datum_length_strain_reverse": quantity(eps_datum(To, Ti)),
                "datum_length_strain_install_to_split": quantity(eps_datum(Ti, Ts)),
                "datum_length_strain_split_to_operating": quantity(eps_datum(Ts, To)),
                "datum_length_strain_operating_to_split": quantity(eps_datum(To, Ts)),
                "datum_length_strain_split_to_install": quantity(eps_datum(Ts, Ti)),
                "current_length_strain_forward": expm1_symbolic(I_io),
                "current_length_strain_reverse": expm1_symbolic(-I_io),
                "current_length_strain_install_to_split": expm1_symbolic(integral(alpha_pts, Ti, Ts)),
                "current_length_strain_split_to_operating": expm1_symbolic(integral(alpha_pts, Ts, To)),
                "current_length_strain_operating_to_split": expm1_symbolic(integral(alpha_pts, To, Ts)),
                "current_length_strain_split_to_install": expm1_symbolic(integral(alpha_pts, Ts, Ti)),
            },
        },
        "linear_dilation_table": {
            "inputs": {
                "provenance": "invented",
                "definition": "engineering_dilation",
                "interpolation": "linear_dilation",
                "datum_temperature": kq(Tm),
                "installation_temperature": kq(Ti),
                "operating_temperature": kq(To),
                "split_temperature": kq(Ts),
                "linear_dilation_points": [
                    {"temperature": kq(t), "dilation": quantity(x)} for t, x in dil_pts],
            },
            "expected": {
                "dilation_install": quantity(d(Ti)),
                "dilation_split": quantity(d(Ts)),
                "dilation_operating": quantity(d(To)),
                "dilation_strain_forward": quantity(eps_dil(Ti, To)),
                "dilation_strain_reverse": quantity(eps_dil(To, Ti)),
                "dilation_strain_install_to_split": quantity(eps_dil(Ti, Ts)),
                "dilation_strain_split_to_operating": quantity(eps_dil(Ts, To)),
                "dilation_strain_operating_to_split": quantity(eps_dil(To, Ts)),
                "dilation_strain_split_to_install": quantity(eps_dil(Ts, Ti)),
            },
        },
    },
    "wrong_result_discriminators": {
        "first_segment_only": {
            "definition": "Only the first table segment traversed by [T_install, T] ([350 K, 400 K]) contributes to the integral or dilation difference; lambda(T_install) is correct. Truncating I(T_m,T) or d(T)-d(T_m) to its first segment ([300 K, 400 K]) gives the same values here because T_install lies in that segment. Forward direction only.",
            "datum_length_strain_forward": quantity(first_piece / lam_i),
            "current_length_strain_forward": expm1_symbolic(first_piece),
            "dilation_strain_forward": quantity(dil_first / lam_dil(Ti)),
        },
        "last_segment_only": {
            "interval_difference_form": {
                "definition": "Only the last table segment traversed by [T_install, T] ([500 K, 550 K]) contributes to the numerator integral or dilation difference; lambda(T_install) is correct. Forward direction only.",
                "datum_length_strain_forward": quantity(last_piece / lam_i),
                "current_length_strain_forward": expm1_symbolic(last_piece),
                "dilation_strain_forward": quantity(dil_last / lam_dil(Ti)),
            },
            "datum_integral_form": {
                "definition": "lambda(T) uses I(T_m,T) or d(T)-d(T_m) truncated to the last segment of [T_m, T] ([500 K, 550 K]); lambda(T_install) keeps its only segment [300 K, 350 K]. Forward direction only.",
                "datum_length_strain_forward": quantity((1 + I_o_last_only) / lam_i - 1),
                "current_length_strain_forward": expm1_symbolic(I_o_last_only - I_i),
                "dilation_strain_forward": quantity((1 + d_o_last_only) / lam_dil(Ti) - 1),
            },
        },
        "interior_breakpoints_skipped": {
            "definition": "One trapezoid from alpha(T_install) to alpha(T), ignoring the 400 K and 500 K table points. Coefficient definitions only; forward direction only.",
            "datum_length_strain_forward": quantity(skip_interior / lam_i),
            "current_length_strain_forward": expm1_symbolic(skip_interior),
        },
        "endpoint_alpha_times_interval": {
            "definition": "alpha(T)*(T-T_install), the control-5 wrong control applied to this table.",
            "strain": quantity(endpoint_alpha),
        },
    },
    "limits": [
        "All inputs are invented test quantities; they are not material-library, code-rule or vetted engineering data.",
        "T_install=350 K and T=550 K are interior to the first and last table segments; the 450 K split is interior to the middle segment.",
        "Free-length strains only. A 1 m free span has tip UX equal to the strain in metres; a fixed span has N=-E*A*eps. No mechanics companion values are allocated here.",
        "Wrong-result discriminators are given for the forward direction only; correct reverse values are pinned in expected.",
    ],
}

# ---------------------------------------------------------------- 2. verification_two_point
C2K = F(27315, 100)
tm_c, ti_c, to_c = F(20), F(50), F(150)
a_i, a_o = F(12, 1000000), F(16, 1000000)
pts_c = [(ti_c, a_i), (to_c, a_o)]
tm_k, ti_k, to_k = tm_c + C2K, ti_c + C2K, to_c + C2K
secant_pts_k = [(t + C2K, a) for t, a in pts_c]  # coefficient per K: no offset
assert interpolate(secant_pts_k, ti_k) == a_i and interpolate(secant_pts_k, to_k) == a_o
assert not any(t == tm_k for t, _ in secant_pts_k), "no coefficient point at the datum"


def lam_secant(Tk):
    return 1 + interpolate(secant_pts_k, Tk) * (Tk - tm_k)


dil_i, dil_o = a_i * (ti_k - tm_k), a_o * (to_k - tm_k)
eps_sec = lam_secant(to_k) / lam_secant(ti_k) - 1
assert eps_sec == F(43, 25009)
# Positivity over the required interval [min(Ti,T), max(Ti,T)]: lambda is quadratic
# in T on the one segment; check both ends and any interior stationary point.
lo_k, hi_k = min(ti_k, to_k), max(ti_k, to_k)
(t0, y0), (t1, y1) = secant_pts_k
slope = (y1 - y0) / (t1 - t0)
# lambda(T)=1+(y0+slope*(T-t0))*(T-tm): d/dT = slope*(T-tm)+y0+slope*(T-t0) = 0
stationary = (slope * (tm_k + t0) - y0) / (2 * slope)
candidates = [lo_k, hi_k] + ([stationary] if lo_k < stationary < hi_k else [])
lam_min = min(lam_secant(t) for t in candidates)
assert lam_min > 0 and lam_min == lam_secant(lo_k)
L, E = F(1), F(200000000000)


def load_fixture(path):
    raw = open(path, "rb").read()
    got = hashlib.sha256(raw).hexdigest()
    if got != PRE_SHA256:
        sys.exit(f"refusing: {path} sha256 {got} != reviewed pre-edit {PRE_SHA256}")
    doc = json.loads(raw)
    assert (json.dumps(doc, indent=2, allow_nan=False) + "\n").encode() == raw, "pre-edit bytes must round-trip"
    return raw, doc


def verification_two_point(doc):
    PI = D(doc["numeric_representation"]["pi_decimal"])
    geom = doc["geometry"]
    od = F(geom["authored"]["outside_diameter"]["exact"]["rational"])
    t = F(geom["authored"]["wall_thickness"]["exact"]["rational"])
    As = t * (od - t)  # times pi
    assert As == (od / 2) ** 2 - (od / 2 - t) ** 2
    cq = lambda x: quantity(x, "degC")
    return {
        "inputs": {
            "coefficient_definition": "engineering_secant",
            "datum_temperature": cq(tm_c),
            "installation_temperature": cq(ti_c),
            "operating_temperature": cq(to_c),
            "alpha_install": quantity(a_i, "1/K"),
            "alpha_operating": quantity(a_o, "1/K"),
            "interpolation": "linear_coefficient",
            "table_points": [{"temperature": cq(tc), "alpha": quantity(a, "1/K")} for tc, a in pts_c],
            "geometry_ref": geom["id"],
            "L": quantity(L, "m"),
            "E": quantity(E, "Pa"),
            "provenance": "VERIFICATION.md control 4 exact inputs; the table has no coefficient point at the datum temperature.",
        },
        "admissibility_policy": {
            "selected_by": "ROOT disposition of CP1 independent review SF2 (CP2_EXTENSION brief)",
            "rule": "For engineering_secant tables, table coverage and positive free stretch lambda are required only over [min(T_install,T), max(T_install,T)]. No coefficient point at T_m is required, because lambda(T_m)=1 by definition and the strain consumes alpha_sec only at T_install and T.",
            "datum_coverage_required": False,
            "negative_control_scope": "negative_contract_controls.datum_outside_thermal_coverage is unchanged. It still applies to differential_per_datum_length, logarithmic_per_current_length and engineering_dilation, whose lambda(T) is referred to T_m through table data; it does not apply to engineering_secant.",
        },
        "expected": {
            "dilation_install": quantity(dil_i),
            "dilation_operating": quantity(dil_o),
            "thermal_stretch": quantity(1 + eps_sec),
            "thermal_strain": quantity(eps_sec),
            "datum_temperature_K": quantity(tm_k, "K"),
            "installation_temperature_K": quantity(ti_k, "K"),
            "operating_temperature_K": quantity(to_k, "K"),
            "required_coverage_low_K": quantity(lo_k, "K"),
            "required_coverage_high_K": quantity(hi_k, "K"),
            "minimum_datum_stretch_over_required_interval": quantity(lam_min),
            "free_tip_UX": quantity(L * eps_sec, "m"),
            "free_wall_N": quantity(0, "N"),
            "fixed_wall_N": quantity(-E * As * eps_sec, "N", pi=PI),
            "fixed_root_Fx": quantity(E * As * eps_sec, "N", pi=PI),
            "fixed_far_Fx": quantity(-E * As * eps_sec, "N", pi=PI),
        },
    }


# ---------------------------------------------------------------- 3. temperature_unit_identity
TO_K = {
    "K": lambda x: x,
    "degC": lambda x: x + F(27315, 100),
    "degF": lambda x: (x + F(45967, 100)) * F(5, 9),
    "degR": lambda x: x * F(5, 9),
}


def authored(text, unit):
    q = quantity(F(text), unit)
    assert q["decimal"] == text, (text, q["decimal"])  # authored decimal is exact
    return q


groups = [
    ("minus_50_degC", [("-50", "degC"), ("223.15", "K")]),
    ("242_degC", [("242", "degC"), ("467.6", "degF"), ("515.15", "K")]),
    ("20_degC_rankine", [("20", "degC"), ("527.67", "degR")]),
]
control = ("minus_49p999999_degC_vs_223p15_K", [("-49.999999", "degC"), ("223.15", "K")])
group_expected = []
for gid, items in groups:
    ks = [TO_K[u](F(x)) for x, u in items]
    assert len(set(ks)) == 1, (gid, ks)
    group_expected.append({"id": gid, "kelvin": quantity(ks[0], "K"),
                           "authored_kelvin": [quantity(k, "K") for k in ks], "compare_equal": True})
ck = [TO_K[u](F(x)) for x, u in control[1]]
assert ck[0] != ck[1] and ck[0] - ck[1] == F(1, 1000000)


def binary64_dyadic(x, unit):
    return quantity(F(x), unit)  # F(float) is the exact dyadic value of the double


b64_m50 = -50.0 + 273.15
b64_223 = 223.15
b64_f = (467.6 + 459.67) * 5 / 9
b64_c242 = 242.0 + 273.15
b64_515 = 515.15
assert b64_m50 != b64_223 and b64_f != b64_c242 and b64_f != b64_515

temperature_identity = {
    "source": {
        "document": "VERIFICATION.md",
        "control": 4,
        "clause": "Equivalent degC/K absolute and temperature-interval inputs give identical normalized mechanics.",
        "motivation": "CP1 independent review SF3; identity values set by the CP2_EXTENSION brief.",
    },
    "equations": [
        "K = degC + 273.15",
        "K = (degF + 459.67)*5/9",
        "K = degR*5/9",
        "identity: read the authored decimal as an exact rational, apply the exact affine definition, compare exactly in K",
    ],
    "variants": {
        "exact_affine_identity": {
            "inputs": {
                "provenance": "invented",
                "identity_groups": [{"id": gid, "authored": [authored(x, u) for x, u in items]} for gid, items in groups],
                "non_equal_control": {"id": control[0], "authored": [authored(x, u) for x, u in control[1]]},
            },
            "expected": {
                "identity_groups": group_expected,
                "non_equal_control": {
                    "id": control[0],
                    "authored_kelvin": [quantity(k, "K") for k in ck],
                    "difference_K": quantity(ck[0] - ck[1], "K"),
                    "compare_equal": False,
                },
            },
        },
    },
    "wrong_result_discriminators": {
        "binary64_affine_conversion": {
            "definition": "IEEE-754 binary64 round-to-nearest-even evaluation of the stated expression, then equality of doubles. Exact values are the dyadic rationals of the doubles. The results depend on operation order; they show that binary64 equality after conversion is not temperature identity.",
            "minus_50_degC": {
                "expression": "-50.0 + 273.15",
                "result": binary64_dyadic(b64_m50, "K"),
                "binary64_223p15_K": binary64_dyadic(b64_223, "K"),
                "doubles_equal": False,
            },
            "467p6_degF": {
                "expression": "(467.6 + 459.67) * 5 / 9",
                "result": binary64_dyadic(b64_f, "K"),
                "binary64_242_degC_via_plus_273p15": binary64_dyadic(b64_c242, "K"),
                "binary64_515p15_K": binary64_dyadic(b64_515, "K"),
                "doubles_equal": False,
            },
        },
    },
    "limits": [
        "Absolute temperatures only. Temperature intervals and inverse-temperature coefficients convert without offset (1 degC interval = 1 K; 1 degF or degR interval = 5/9 K; alpha[1/K] = alpha[1/degF]*9/5).",
        "The authored decimals are invented test quantities. The identities are exact consequences of the unit definitions; no product unit-catalog behaviour is asserted here.",
        "The non-equal control differs by exactly 1e-6 K (relative about 4.5e-9). It must not be matched to 223.15 K by identity, bracketing or exact-point selection.",
    ],
}


def main():
    path = sys.argv[1]
    raw, doc = load_fixture(path)
    cases = doc["cases"]
    for key in ("multi_segment_free_length", "temperature_unit_identity"):
        assert key not in cases, key
    assert "verification_two_point" not in cases["thermal_datum_ratio"]["variants"]
    cases["thermal_datum_ratio"]["variants"]["verification_two_point"] = verification_two_point(doc)
    cases["multi_segment_free_length"] = multi_segment
    cases["temperature_unit_identity"] = temperature_identity
    out = (json.dumps(doc, indent=2, allow_nan=False) + "\n").encode()
    json.loads(out)
    with open(path, "wb") as fh:
        fh.write(out)
    print(json.dumps({
        "input_sha256": hashlib.sha256(raw).hexdigest(),
        "output_sha256": hashlib.sha256(out).hexdigest(),
        "bytes_before": len(raw), "bytes_after": len(out),
        "added": ["cases.thermal_datum_ratio.variants.verification_two_point",
                  "cases.multi_segment_free_length", "cases.temperature_unit_identity"],
        "headline": {
            "datum_length_strain_forward": str(eps_datum(Ti, To)),
            "datum_length_strain_reverse": str(eps_datum(To, Ti)),
            "current_length_strain_forward": f"exp({I_io})-1",
            "dilation_strain_forward": str(eps_dil(Ti, To)),
            "dilation_strain_reverse": str(eps_dil(To, Ti)),
            "verification_two_point_strain": str(eps_sec),
        },
    }, indent=1))


if __name__ == "__main__":
    main()
