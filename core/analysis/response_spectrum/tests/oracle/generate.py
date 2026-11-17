"""Independent expectations for the S2 evaluation's accuracy test (tests/accuracy.rs).

It reads, imports and runs no product code. Python standard library only:
    python3 core/analysis/response_spectrum/tests/oracle/generate.py
rewrites tests/oracle/cases.rs deterministically.

For each case it states the represented binary64 inputs (bit patterns) and computes, from
their exact rational values, the exact interpolation of the represented table at the
represented query (period T = 1/f exactly) to 150 significant digits:
  linear ordinate  y = y0 + (y1 - y0) t, exactly rational under a linear abscissa;
  log ordinate     y = exp((1 - t) ln y0 + t ln y1);
  t = (X - x0)/(x1 - x0) or ln(X/x0)/ln(x1/x0), X = f or 1/f;
  S = g y (g-multiple tables), sS = s S.
The expected outcomes encode the present conservative refusal policy on this selected corpus,
not representational impossibility. Each exact y, S and sS is compared with the worst-case
half-subnormal-spacing allowance 2^-1075 and the upper range boundary. Exact subnormal
values can be representable with zero rounding error and still be refused by that policy.
Cases within 1 % of the subnormal threshold or within 1e-6 of 2^1024 are not generated:
computed blends at the upper edge can refuse even when the exact answer is representable.
These final-result comparisons are platform evidence for the recorded test configuration,
not a direct check or proof of the assumed ln, ln_1p and exp accuracy.
"""
import math
import random
import struct
from decimal import Decimal, localcontext
from fractions import Fraction as F
from pathlib import Path

PREC = 150
TWO_1024 = F(2) ** 1024
NORMAL_MIN = F(1, 2 ** 1022)
HALF_SUBNORMAL = F(1, 2 ** 1075)
ACCURACY = F(1, 10 ** 9)


def bits(x):
    return struct.unpack('<Q', struct.pack('<d', x))[0]


def dec(q):
    return Decimal(q.numerator) / Decimal(q.denominator)


def exact_values(axis, xr, yr, points, f, g, s):
    (x0, y0), (x1, y1) = [(F(a), F(b)) for a, b in points]
    X = 1 / F(f) if axis == 'period' else F(f)
    assert x0 <= X <= x1, 'queries inside the table only'
    with localcontext() as c:
        c.prec = PREC
        if X in (x0, x1):
            y = dec(y0 if X == x0 else y1)
        elif xr == 'linear' and yr == 'linear':
            y = dec(y0 + (y1 - y0) * (X - x0) / (x1 - x0))
        else:
            t = dec((X - x0) / (x1 - x0)) if xr == 'linear' else dec(X / x0).ln() / dec(x1 / x0).ln()
            if yr == 'linear':
                y = dec(y0) + (dec(y1) - dec(y0)) * t
            else:
                y = ((1 - t) * dec(y0).ln() + t * dec(y1).ln()).exp()
        S = y * dec(F(g)) if g is not None else y
        sS = S * dec(F(s))
        return [+y, +S, +sS]


def statable(v):
    """True, False, or None inside an excluded band."""
    q = F(v)  # exact rational of the 150-digit decimal
    if q >= TWO_1024 * (1 + F(1, 10 ** 6)):
        return False
    if q >= TWO_1024 * (1 - F(1, 10 ** 6)):
        return None
    if q >= NORMAL_MIN:
        return True
    ratio = HALF_SUBNORMAL / q / ACCURACY
    if ratio <= F(99, 100):
        return True
    if ratio >= F(101, 100):
        return False
    return None


def case(name, axis, xr, yr, points, f, g=None, s=1.0):
    values = exact_values(axis, xr, yr, points, f, g, s)
    verdicts = [statable(v) for v in values]
    if None in verdicts:
        return None
    return dict(name=name, axis=axis, xr=xr, yr=yr, points=points, f=f, g=g, s=s,
                expected=values if all(verdicts) else None)


def witnesses():
    """The first independent review's probes at 03c88215f, rebuilt from their stated inputs."""
    out = [
        case('linear_cancellation', 'frequency', 'linear', 'linear', [[1.0, 1e16], [2.0, 1.0]], math.nextafter(2.0, 0.0)),
        case('log_exp_overflow', 'frequency', 'linear', 'log', [[1.0, 1e-300], [2.0, 1e300]], 1.75),
        case('log_exp_subnormal', 'frequency', 'linear', 'log', [[1.0, 1e300], [2.0, 1e-300]], 1.52, s=1e30),
        case('log_exp_zero', 'frequency', 'linear', 'log', [[1.0, 1e300], [2.0, 1e-300]], 1.55, s=1e30),
        case('log_exp_precision', 'frequency', 'linear', 'log', [[1.0, 1e300], [2.0, 1e-300]], 1.53, s=1e18),
        case('linear_parameter_underflow', 'frequency', 'linear', 'linear', [[math.ulp(0.0), 1e-300], [1e308, 1e308]], 1e-20, s=1e20),
        case('g_underflow_then_scale', 'frequency', 'linear', 'linear', [[1.0, 1e-200], [2.0, 1e-200]], 1.0, g=1e-200, s=1e300),
        case('g_subnormal_then_scale', 'frequency', 'linear', 'linear', [[1.0, 1e-200], [2.0, 1e-200]], 1.0, g=1e-120, s=1e300),
    ]
    f = 1.5e308
    x = 1 / F(f)
    x0 = float(x)
    if F(x0) > x:
        x0 = math.nextafter(x0, 0.0)
    x1 = math.nextafter(x0, math.inf)
    out.append(case('period_subnormal_spacing', 'period', 'linear', 'linear', [[x0, 1.0], [x1, 2.0]], f))
    out.append(case('period_subnormal_spacing_logx', 'period', 'log', 'linear', [[x0, 1.0], [x1, 2.0]], f))
    return out


def edges():
    """Published values either side of the subnormal threshold and of 2^1024, and the period
    near-knot numerator 1 - f x0 = 2^-104 that the review cited."""
    out = []
    pts = [[1.0, 1.0], [4.0, 1.0]]
    mid = 2.0
    for name, g in [('S_subnormal_accepted', 3e-315), ('S_subnormal_refused', 2e-315),
                    ('S_deep_subnormal_refused', 1e-320), ('S_normal_min', 2.2250738585072014e-308)]:
        out.append(case(name, 'frequency', 'linear', 'linear', pts, mid, g=g))
    for name, s in [('sS_below_overflow', 1.6e308), ('sS_overflow_refused', 1.7e308)]:
        out.append(case(name, 'frequency', 'linear', 'linear', [[1.0, 1.1], [4.0, 1.1]], mid, s=s))
    out.append(case('sS_overflow_refused_by_scale', 'frequency', 'linear', 'linear', [[1.0, 1e308], [2.0, 1.5e308]], 1.5, s=2.0))
    f = 1 + 2.0 ** -52
    x0 = 1 - 2.0 ** -52
    for xr in ('linear', 'log'):
        for yr in ('linear', 'log'):
            out.append(case(f'period_near_knot_{xr}_{yr}', 'period', xr, yr, [[x0, 3.0], [1e300, 1e-300]], f, s=7.0))
    return out


def adversarial(rng, count):
    def rpos():
        x = math.ldexp(1.0 + rng.random(), rng.randint(-1074, 1023))
        return x if 0 < x < math.inf else 1.0
    out = []
    while len(out) < count:
        axis, xr, yr = rng.choice(['frequency', 'period']), rng.choice(['linear', 'log']), rng.choice(['linear', 'log'])
        x0 = rpos()
        x1 = rpos() if rng.random() < 0.5 else x0 * (1 + rng.choice([2.0 ** -52, 2.0 ** -40, 1e-6, 0.5, 3.0]))
        if not (math.isfinite(x1) and x1 > x0):
            continue
        if rng.random() < 0.7:
            X = x0 if rng.random() < 0.5 else x1
            f = 1 / X if axis == 'period' else X
            if not (0 < f < math.inf):
                continue
            for _ in range(rng.randint(1, 5)):
                f = math.nextafter(f, math.inf if rng.random() < 0.5 else 0.0)
        else:
            X = math.exp(rng.uniform(math.log(x0), math.log(x1)))
            f = 1 / X if axis == 'period' else X
        if not (0 < f < math.inf):
            continue
        Xq = 1 / F(f) if axis == 'period' else F(f)
        if not (F(x0) < Xq < F(x1)):
            continue
        y0, y1 = rpos(), rpos()
        if rng.random() < 0.3:
            y1 = y0 * rng.choice([1 + 2.0 ** -50, 1e16, 1e-16])
            if not (0 < y1 < math.inf):
                continue
        g = rpos() if rng.random() < 0.3 else None
        s = math.ldexp(1.0 + rng.random(), rng.randint(-60, 60))
        c = case(f'adversarial_{len(out):03d}', axis, xr, yr, [[x0, y0], [x1, y1]], f, g, s)
        if c is not None:
            out.append(c)
    return out


def rust(c):
    def f64(x):
        return f'f64::from_bits(0x{bits(x):016x})'
    g = 'None' if c['g'] is None else f'Some({f64(c["g"])})'
    expected = 'None' if c['expected'] is None else 'Some([' + ', '.join(f'"{v:.40E}"' for v in c['expected']) + '])'
    (x0, y0), (x1, y1) = c['points']
    return (f'    Case {{ name: "{c["name"]}", period: {str(c["axis"] == "period").lower()}, log_x: {str(c["xr"] == "log").lower()}, '
            f'log_y: {str(c["yr"] == "log").lower()},\n        points: [[{f64(x0)}, {f64(y0)}], [{f64(x1)}, {f64(y1)}]],\n'
            f'        f: {f64(c["f"])}, g: {g}, s: {f64(c["s"])},\n        expected: {expected} }},\n')


def main():
    rng = random.Random(20261115)
    cases = [c for c in witnesses() + edges() if c is not None] + adversarial(rng, 240)
    assert len([c for c in cases if c['name'].startswith(('linear_', 'log_', 'period_subnormal', 'g_'))]) == 10, 'every witness is decided'
    text = ('// Generated by tests/oracle/generate.py from exact rationals at 150 digits; do not edit.\n'
            '// Expected values are y, S and s·S; `None` means the evaluation is refused for accuracy.\n'
            'use super::Case;\n\npub const CASES: &[Case] = &[\n' + ''.join(rust(c) for c in cases) + '];\n')
    Path(__file__).with_name('cases.rs').write_text(text)
    print(len(cases), 'cases;', sum(c['expected'] is None for c in cases), 'refused')


if __name__ == '__main__':
    main()
