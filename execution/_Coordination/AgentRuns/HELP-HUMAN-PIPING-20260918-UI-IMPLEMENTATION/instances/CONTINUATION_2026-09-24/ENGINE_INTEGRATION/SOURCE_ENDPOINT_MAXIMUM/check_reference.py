#!/usr/bin/env python3
"""Bounded candidate arithmetic check, not Rust/native execution.

The floating implementation below mirrors the proposed recipe. The oracle is
independent: Fraction.from_float represents supplied binary64 operands exactly;
comparison with sqrt is performed by exact rational squaring, without using
this implementation, product rows, hypot, or a rounded sqrt as the oracle.
"""
import itertools
import json
import math
import platform
import random
import sys
from fractions import Fraction as F
from pathlib import Path

CRITERION = 1e-9
MIN_NORMAL = sys.float_info.min


class Refusal(Exception):
    pass


def finite(x):
    if not math.isfinite(x):
        raise Refusal("ArithmeticRange")
    return x


def up(x):
    return finite(math.nextafter(x, math.inf))


def directed(x, upper):
    finite(x)
    return up(x) if upper else max(0.0, math.nextafter(x, -math.inf))


def div(x, d, upper):
    if x == 0.0 or d == 1.0:
        return x
    if x == d:
        return 1.0
    return directed(x / d, upper)


def mul(x, y, upper):
    if x == 0.0 or y == 0.0:
        return 0.0
    if x == 1.0:
        return y
    if y == 1.0:
        return x
    return directed(x * y, upper)


def add(x, y, upper):
    if x == 0.0:
        return y
    if y == 0.0:
        return x
    return directed(x + y, upper)


def norm(y, z):
    m = max(y, z)
    if m == 0.0:
        return 0.0
    a, b = y / m, z / m
    return finite(m * math.sqrt((a * a) + (b * b)))


def norm_bound(y, z, upper):
    if y == 0.0:
        return z
    if z == 0.0:
        return y
    m = max(y, z)
    a, b = div(y, m, upper), div(z, m, upper)
    root = directed(math.sqrt(add(mul(a, a, upper), mul(b, b, upper), upper)), upper)
    return mul(m, root, upper)


def p(value, lo=None, hi=None):
    lo = value if lo is None else lo
    hi = value if hi is None else hi
    if not all(map(math.isfinite, [value, lo, hi])) or not lo <= value <= hi:
        raise Refusal("InvalidProjection")
    return (value, lo, hi)


def endpoint(area, z, actions):
    n, my, mz = [abs(x[0]) for x in actions]
    value = finite(finite(n / area) + norm(finite(my / z), finite(mz / z)))
    abs_bounds = []
    for _, lo, hi in actions:
        abs_bounds.append((lo, hi) if lo >= 0 else ((-hi, -lo) if hi <= 0 else (0.0, max(-lo, hi))))
    bounds = []
    for side, upper in enumerate([False, True]):
        n, y, zz = [v[side] for v in abs_bounds]
        bounds.append(add(div(n, area, upper), norm_bound(div(y, z, upper), div(zz, z, upper), upper), upper))
    assert bounds[0] <= value <= bounds[1]
    return value, bounds


def overlap(a, b):
    return a[1] <= b[2] and b[1] <= a[2]


def exact_equal(a, b, magnitude=False):
    return (a[1] == a[2] and b[1] == b[2]
            and (abs(a[1]) == abs(b[1]) if magnitude else a[1] == b[1]))


def maximum(area, z, ends):
    if not all(map(math.isfinite, [area, z])) or area <= 0 or z <= 0:
        raise Refusal("InvalidSection")
    if not overlap(ends[0][0], ends[1][0]):
        raise Refusal("InconsistentAxialBounds")
    endpoints = [endpoint(area, z, e) for e in ends]
    value = max(e[0] for e in endpoints)
    lo, hi = [max(e[1][i] for e in endpoints) for i in range(2)]
    error = 0.0 if lo == value == hi else up(max(value - lo, hi - value))
    relative = 0.0
    if hi != 0.0:
        if value < MIN_NORMAL or hi < MIN_NORMAL or (lo != 0 and lo < MIN_NORMAL):
            raise Refusal("PublicationRange")
        if lo <= 0:
            raise Refusal("ProtectedCriterionUnestablished")
        relative = 0.0 if error == 0.0 else up(error / lo)
        if relative > CRITERION:
            raise Refusal("ProtectedCriterionUnestablished")
    if all(exact_equal(ends[0][c], ends[1][c]) for c in [1, 2]):
        locations = {"kind": "WholeSpanConstant"}
    elif endpoints[0][1][0] > endpoints[1][1][1]:
        locations = {"kind": "StrictEndpoint", "endpoint": "I"}
    elif endpoints[1][1][0] > endpoints[0][1][1]:
        locations = {"kind": "StrictEndpoint", "endpoint": "J"}
    else:
        locations = {"kind": "EndpointCandidates",
                     "exact_tie_proven": all(exact_equal(ends[0][c], ends[1][c], True) for c in [1, 2]),
                     "interior_equal_possible": all(overlap(ends[0][c], ends[1][c]) for c in [1, 2])}
    return {"value_pa": value, "interval_pa": [lo, hi], "absolute_error_bound_pa": error,
            "relative_error_bound": relative, "locations": locations, "endpoints": endpoints,
            "witness": "J" if endpoints[1][0] > endpoints[0][0] else "I"}


# Independent exact oracle. q <= a + sqrt(b) iff q-a <= 0 or (q-a)^2 <= b.
def stress_ge(bound, actions, area, z):
    n, y, zz = map(F, actions)
    q = F(bound) - abs(n) / F(area)
    return q <= 0 or (q * F(z)) ** 2 <= y * y + zz * zz


def stress_le(bound, actions, area, z):
    n, y, zz = map(F, actions)
    q = F(bound) - abs(n) / F(area)
    return q >= 0 and (q * F(z)) ** 2 >= y * y + zz * zz


def audit(name, area, z, ends, expect=None, refusal=None):
    inputs = {"area": area, "section_modulus": z, "endpoint_projections_value_lo_hi": ends}
    try:
        result = maximum(area, z, ends)
    except Refusal as e:
        assert refusal == str(e), (name, str(e), refusal)
        return {"name": name, "inputs": inputs, "expected_refusal": str(e)}
    assert refusal is None, name
    # Examine the corners and representative of each projected-action box.
    # This checks full component uncertainty, not only the rounded point value.
    checked = 0
    for end, estimate in zip(ends, result["endpoints"]):
        values = [sorted(set(action)) for action in end]
        for actions in itertools.product(*values):
            assert stress_ge(estimate[1][0], actions, area, z), (name, "endpoint lower", actions)
            assert stress_le(estimate[1][1], actions, area, z), (name, "endpoint upper", actions)
            checked += 1
    # Sample actual affine source fields: common constant N from the intersection
    # plus bending endpoint corners, with exact rational affine interpolation.
    common_n = [max(ends[0][0][1], ends[1][0][1]), min(ends[0][0][2], ends[1][0][2])]
    for n in set(common_n):
        for moments in itertools.product(*[sorted(set(a)) for e in ends for a in e[1:]]):
            actions0, actions1 = (n, *moments[:2]), (n, *moments[2:])
            assert stress_ge(result["interval_pa"][0], actions0, area, z) or stress_ge(result["interval_pa"][0], actions1, area, z)
            for t in [F(0), F(1, 8), F(1, 2), F(7, 8), F(1)]:
                actions = [F(n)] + [(1-t)*F(a)+t*F(b) for a, b in zip(moments[:2], moments[2:])]
                assert stress_le(result["interval_pa"][1], actions, area, z), (name, "span upper", t)
                checked += 1
    # The published absolute/relative promises are also compared in exact rationals.
    lo, hi, q = map(F, [*result["interval_pa"], result["value_pa"]])
    error, rel = map(F, [result["absolute_error_bound_pa"], result["relative_error_bound"]])
    assert error >= max(q-lo, hi-q)
    if lo > 0:
        assert rel >= error/lo
        assert rel <= F(CRITERION)
    if expect is not None:
        assert lo <= F(expect) <= hi, (name, expect, result)
    return {"name": name, "inputs": inputs, "result": result, "oracle_points": checked}


def exact(n, y, z):
    return [p(n), p(y), p(z)]


def run():
    tiny = math.ulp(0.0)
    cases = [
        ("axial_constant", 2.0, 1.0, [exact(-10.,0.,0.)]*2, 5., None),
        ("pure_bending", 1.0, 2.0, [exact(0.,3.,0.)]*2, 1.5, None),
        ("biaxial_plus_axial", 2.0, 2.0, [exact(10.,3.,4.)]*2, 7.5, None),
        ("opposite_equal_end_bending",1.0,1.0,[exact(0.,3.,4.),exact(0.,-3.,-4.)],5.,None),
        ("strict_unequal_ends",1.0,1.0,[exact(0.,1.,0.),exact(0.,2.,0.)],2.,None),
        ("uncertain_order",1.0,1.0,[[p(1.),p(3.,3.-1e-10,3.+1e-10),p(4.)]]*2,6.,None),
        ("cancelled_source_action",2.0,1.0,[[p(1.,1.-1e-12,1.+1e-12),p(0.),p(0.)]]*2,0.5,None),
        ("large_normal",1.0,1.0,[exact(1e300,1e300,1e300)]*2,None,None),
        ("small_normal",1.0,1.0,[exact(1e-300,1e-300,1e-300)]*2,None,None),
        ("subnormal_projection_normal_stress",tiny,1.0,[exact(tiny,0.,0.)]*2,1.,None),
        ("subnormal_norm_ratio",1.0,1.0,[exact(0.,1.,tiny)]*2,None,None),
        ("zero",2.0,1.0,[exact(0.,0.,0.)]*2,0.,None),
        ("projection_budget_exceeded",1.0,1.0,[[p(1.,.9,1.1),p(3.),p(4.)]]*2,None,"ProtectedCriterionUnestablished"),
        ("subnormal_publication",1.0,1.0,[exact(tiny,0.,0.)]*2,None,"PublicationRange"),
        ("zero_from_underflow",2.0,1.0,[exact(tiny,0.,0.)]*2,None,"PublicationRange"),
        ("overflow",1.0,1.0,[exact(1e308,1e308,1e308)]*2,None,"ArithmeticRange"),
        ("inconsistent_axial",1.0,1.0,[exact(1.,0.,0.),exact(2.,0.,0.)],None,"InconsistentAxialBounds"),
    ]
    # Each primitive enclosure independently meets 1e-9, yet its remaining
    # margin cannot pay for this derived arithmetic. The derived recipe refuses.
    near_limit = []
    for value in [3.0, 4.0]:
        lo, hi = value*(1-CRITERION), value*(1+CRITERION)
        while max(F(value)-F(lo), F(hi)-F(value))/F(lo) > F(CRITERION):
            lo, hi = math.nextafter(lo, value), math.nextafter(hi, value)
        near_limit.append(p(value,lo,hi))
    cases.append(("primitive_pass_derived_budget_refused",1.,1.,[[p(0.),*near_limit]]*2,None,"ProtectedCriterionUnestablished"))
    cases.append(("rounded_equal_nonconstant_candidates",1.,1.,
                  [[p(0.),p(3.,3.-1e-10,3.+1e-10),p(4.)],
                   [p(0.),p(-3.,-3.-1e-10,-3.+1e-10),p(-4.)]],5.,None))
    # Independent exact cancellation and tied/interior analytical controls.
    assert (F(2**53)+1)-F(2**53) == 1
    assert (float(2**53)+1)-float(2**53) == 0.0
    assert stress_ge(F(5), (0,3,4),1,1) and stress_le(F(5),(0,3,4),1,1)
    assert stress_ge(F(0), (0,0,0),1,1) and stress_le(F(0),(0,0,0),1,1)
    results = [audit(*case) for case in cases]
    assert results[0]["result"]["locations"]["kind"] == "WholeSpanConstant"
    assert results[3]["result"]["locations"] == {"kind":"EndpointCandidates","exact_tie_proven":True,"interior_equal_possible":False}
    assert results[4]["result"]["locations"] == {"kind":"StrictEndpoint","endpoint":"J"}
    assert results[5]["result"]["locations"] == {"kind":"EndpointCandidates","exact_tie_proven":False,"interior_equal_possible":True}
    assert results[-1]["result"]["locations"] == {"kind":"EndpointCandidates","exact_tie_proven":False,"interior_equal_possible":False}
    # Bounded reproducible spread, independent of product fixtures or outputs.
    rng = random.Random(25092026)
    sweep = []
    for i in range(48):
        scale = math.ldexp(1.0, rng.randint(-900,900))
        n = rng.uniform(-3,3)*scale
        ends = [exact(n,rng.uniform(-3,3)*scale,rng.uniform(-3,3)*scale) for _ in range(2)]
        sweep.append(audit(f"dyadic_scale_{i}",1.0,1.0,ends))
    report = {
        "status":"PASS", "execution":"Python arithmetic mirror + exact Fraction oracle; Rust tests NOT executed",
        "python":sys.version, "platform":platform.platform(), "criterion":CRITERION,
        "cases":results, "spread_controls":len(sweep), "spread_seed":25092026,
        "oracle_points":sum(r.get("oracle_points",0) for r in results+sweep),
    }
    out = Path(__file__).with_name("REFERENCE_RESULTS.json")
    out.write_text(json.dumps(report,indent=2,allow_nan=False)+"\n")
    print(json.dumps({k:v for k,v in report.items() if k != "cases"},indent=2))


if __name__ == "__main__":
    run()
