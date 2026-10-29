"""Correctly rounded ``acos``, ``asin``, ``atan2``, ``sin`` and ``tan`` on binary64 for the MBF importer and writer.

CPython's ``math`` takes these five functions from the platform C library, whose last bit differs between platforms
(macOS's library misrounds some inputs of each, tan most often). The Rust MBF engine evaluates them
correctly rounded, so the Python twin does the same here: each function returns the binary64 nearest the exact
mathematical value of the exact binary64 input, ties to even. Correct rounding fixes the result by definition, so
any two correctly rounded implementations agree on every input and every platform, natively and in wasm.

Method (Ziv's strategy). Each function computes, at a working precision of ``w`` bits, integers ``lo <= hi`` with
``lo / 2**w <= f(x) <= hi / 2**w`` *rigorously*: every truncation in the binary fixed-point evaluation carries an
explicit error bound in units of ``2**-w``. Rounding is monotone, so when ``lo / 2**w`` and ``hi / 2**w`` round to
the same binary64 (Python's ``int / int`` is correctly rounded, subnormals included) that binary64 is the correctly
rounded result; otherwise the precision is doubled and the evaluation repeated. The exact values of these functions
at nonzero binary64 inputs are transcendental (Lindemann-Weierstrass), so they are never a rounding midpoint and the
loop ends; the inputs with exact results (``sin(0)``, ``acos(1)``, ...) are answered before it. The platform
library's result is read only to choose the first precision, never as a result.

The evaluation uses Python integers only: pi by Machin's formula, arctangent by argument halving
(``atan t = 2 atan(t / (1 + sqrt(1 + t^2)))``) and its Taylor series, sine and cosine by their Taylor series after an
reduction by a multiple of pi/2 (pi carried beyond the input's binade, so the reduced argument is within two units
of the working precision for every finite binary64), square roots by ``math.isqrt``. Domain errors, special values and signed zeros are those of ``math``:
inputs outside the finite nonzero domain (NaN, infinities, zeros, ``|x| > 1`` for acos and asin, a zero or
non-finite ``atan2`` argument) are answered by the ``math`` function itself, which raises the same ``ValueError``
or returns the same special value; CPython's ``math.atan2`` answers those cases itself, without the C library.

Standard claim fence applies (claims taxonomy per DEC-081): this fixes the floating-point evaluation of the five
functions; it is no evidence about the engineering meaning of any result computed with them.
"""

from __future__ import annotations

import math

__all__ = ["acos", "asin", "atan2", "sin", "tan"]

# Bits above the result's 53 in the first evaluation; a retry is needed only when the exact value lies within about
# 2^-(53 + _GUARD) relative of a rounding boundary, so retries are rare.
_GUARD = 24
# The precision at which the loop gives up. Unreachable for binary64 inputs (the hardest known cases need well
# under 200 bits beyond the result); it bounds the loop if an error bound were ever wrong.
_MAX_BITS = 1 << 17

_isqrt = math.isqrt


def _float(value: object) -> float:
    # math's own argument conversion (int, float subclasses, __float__, __index__) and its TypeError/OverflowError.
    return value if type(value) is float else math.ldexp(value, 0)


def _first_bits(hint: float, relative_extra: int = 0) -> int:
    """The first working precision: 53 + _GUARD bits of a result of magnitude ``hint`` (any finite estimate)."""
    if hint == 0.0 or not math.isfinite(hint):
        exponent = -1075
    else:
        exponent = math.frexp(hint)[1]
    return 53 + _GUARD + max(0, -exponent) + relative_extra


def _round(lo: int, hi: int, bits: int) -> float | None:
    """The binary64 every real in [lo, hi] / 2^bits rounds to, or None when they round apart."""
    scale = 1 << bits
    low = lo / scale
    high = hi / scale
    if low == high and (low != 0.0 or lo >= 0 or hi <= 0):
        return low
    return None


# ---------------------------------------------------------------------------------------------------------------
# pi
# ---------------------------------------------------------------------------------------------------------------

_PI_CACHE: list[int] = [0, 0]  # [bits, P] with |P - pi 2^bits| < 2, grown on demand
_PI_SPARE = 40


def _atan_inverse(n: int, bits: int) -> tuple[int, int]:
    """(A, E): |A - atan(1/n) 2^bits| <= E, for an integer n >= 2."""
    power = (1 << bits) // n  # error < 1
    total = power
    square = n * n
    k = 0
    while True:
        power //= square  # each step's error stays < 1 + 1/n^2 + ... < 1.07
        k += 1
        term = power // (2 * k + 1)  # error < 1.07 / 3 + 1 < 2
        if term == 0:
            break
        total += -term if k & 1 else term
    # k - 1 terms after the first, each within 2; the dropped alternating tail is below the first dropped term < 2
    return total, 2 * k + 2


def _pi_bits(bits: int) -> tuple[int, int]:
    """(P, 2): |P - pi 2^bits| <= 2, for bits >= 1."""
    cached_bits, cached = _PI_CACHE
    if cached_bits < bits + _PI_SPARE:
        work = max(2 * cached_bits, bits + _PI_SPARE, 1400)
        inner = work + _PI_SPARE
        a5, e5 = _atan_inverse(5, inner)
        a239, e239 = _atan_inverse(239, inner)
        value = 16 * a5 - 4 * a239
        error = 16 * e5 + 4 * e239
        if error >= 1 << _PI_SPARE:  # pragma: no cover - the bound is about 7 * inner
            raise ArithmeticError("pi error bound exceeded")
        _PI_CACHE[:] = [work, value >> _PI_SPARE]  # error < 1 + 1 = 2 units at `work` bits
        cached_bits, cached = _PI_CACHE
    # error <= 2 / 2^shift + 1 < 2 units at `bits` (floor shift adds < 1)
    return cached >> (cached_bits - bits), 2


# ---------------------------------------------------------------------------------------------------------------
# arctangent on [0, 1]
# ---------------------------------------------------------------------------------------------------------------

def _atan_fixed(t: int, bits: int) -> tuple[int, int]:
    """(V, E): |V - atan(t / 2^bits) 2^bits| <= E, for 0 <= t <= 2^bits."""
    if t == 0:
        return 0, 0
    one = 1 << bits
    halvings = 0
    error = 0  # |t - tau 2^bits| for the exact tau = tan(atan(t0) / 2^halvings)
    while t > one >> 4:
        # g(t) = t / (1 + sqrt(1 + t^2)) = tan(atan(t) / 2) has 0 < g' <= 1/2: the incoming error halves. The floor
        # square root and floor quotient add at most 1/4 + 1 units.
        root = _isqrt((one << bits) + t * t)
        t = (t << bits) // (one + root)
        error = (error + 1) // 2 + 2
        halvings += 1
    # Taylor series t - t^3/3 + t^5/5 - ... with t <= 1/16, so t^2 <= 2^-8.
    square = (t * t) >> bits  # error < 1
    power = t  # tracks t^(2k+1); error stays < 1/256 * e + 1/16 + 1 < 1.1
    total = t
    n = 1
    terms = 0
    while True:
        power = (power * square) >> bits
        n += 2
        term = power // n  # error < 1.1/3 + 1 < 2
        if term == 0:
            break
        terms += 1
        total += -term if terms & 1 else term
    # each term within 2, the dropped tail below the first dropped exact term < 2, atan' <= 1 carries `error`
    series_error = 2 * terms + 2 + error
    return total << halvings, series_error << halvings


def _atan_interval(t_lo: int, t_hi: int, bits: int) -> tuple[int, int]:
    """[lo, hi] / 2^bits contains atan(t) for every t in [t_lo, t_hi] / 2^bits, with 0 <= t_lo <= 2^bits."""
    value, error = _atan_fixed(t_lo, bits)
    return value - error, value + error + (t_hi - t_lo)  # atan is increasing and 1-Lipschitz


def _atan_ratio(numerator: int, denominator: int, bits: int) -> tuple[int, int]:
    """[lo, hi] / 2^bits containing atan(numerator / denominator) for positive integers."""
    if numerator <= denominator:
        t = (numerator << bits) // denominator
        return _atan_interval(t, t + 1, bits)
    low, high = _atan_ratio(denominator, numerator, bits)
    half_pi, pi_error = _pi_bits(bits - 1)
    return half_pi - pi_error - high, half_pi + pi_error - low


# ---------------------------------------------------------------------------------------------------------------
# sine and cosine
# ---------------------------------------------------------------------------------------------------------------

def _sin_cos_fixed(r: int, bits: int) -> tuple[int, int, int]:
    """(S, C, E): |S - sin(r / 2^bits) 2^bits| <= E and the same for C and cos, for |r| <= 0.8 2^bits."""
    magnitude = abs(r)
    one = 1 << bits
    square = (magnitude * magnitude) >> bits  # <= 0.64 2^bits, error < 1
    # Each term is the previous times r^2 / ((2k)(2k+1)) (or / ((2k-1)(2k))): with the previous error e <= 2, the new
    # error is below (0.64 e + 1 + 1) / 2 + 1 <= 2.7 for cosine's first divisor 2, and (0.64 e + 2) / 6 + 1 < 2
    # afterwards; terms decrease, so the dropped alternating tail is below the first dropped exact term (< 3).
    term = magnitude
    sine = magnitude
    k = 0
    while True:
        k += 1
        term = ((term * square) >> bits) // ((2 * k) * (2 * k + 1))
        if term == 0:
            break
        sine += -term if k & 1 else term
    sine_error = 3 * k + 3
    term = one
    cosine = one
    j = 0
    while True:
        j += 1
        term = ((term * square) >> bits) // ((2 * j - 1) * (2 * j))
        if term == 0:
            break
        cosine += -term if j & 1 else term
    cosine_error = 3 * j + 3
    return (sine if r >= 0 else -sine), cosine, max(sine_error, cosine_error)


def _reduce(x: float, bits: int) -> tuple[int, int, int]:
    """(q, R, E): x = q pi/2 + r with |r| <= 0.79 and |R - r 2^bits| <= E."""
    numerator, denominator = x.as_integer_ratio()
    shift = denominator.bit_length() - 1  # denominator = 2^shift
    if abs(x) < 0.78:
        if shift <= bits:
            return 0, numerator << (bits - shift), 0
        return 0, (numerator << bits) // denominator, 1
    # |x| >= 0.78: shift <= 53. Carry pi/2 to `wide` bits, beyond x's binade, so |q| E(pi) stays below one unit.
    exponent = math.frexp(x)[1]  # |x| < 2^exponent, so |q| < 2^exponent
    spare = max(exponent, 0) + 8
    wide = bits + spare
    half_pi, pi_error = _pi_bits(wide - 1)
    scaled = numerator << (wide - shift)  # exact x 2^wide
    q = (2 * scaled + half_pi) // (2 * half_pi)  # nearest multiple, to within the pi error
    residue = scaled - q * half_pi  # error <= |q| pi_error < 2^(exponent + 1)
    # floor shift by `spare` bits: error < 2^(exponent + 1 - spare) + 1 < 2
    return q, residue >> spare, 2


# ---------------------------------------------------------------------------------------------------------------
# public functions
# ---------------------------------------------------------------------------------------------------------------

def _exhausted(name: str) -> ArithmeticError:  # pragma: no cover - unreachable for binary64 inputs
    return ArithmeticError(f"correctly rounded {name} did not converge")


def sin(x: object) -> float:
    """``math.sin(x)`` correctly rounded to nearest-even."""
    x = _float(x)
    if x == 0.0 or not math.isfinite(x):
        return math.sin(x)  # +-0 -> +-0, nan -> nan, +-inf -> ValueError
    bits = _first_bits(math.sin(x))
    while bits <= _MAX_BITS:
        q, r, reduction_error = _reduce(x, bits)
        s, c, error = _sin_cos_fixed(r, bits)
        error += reduction_error  # sin and cos are 1-Lipschitz
        value = (s, c, -s, -c)[q & 3]
        result = _round(value - error, value + error, bits)
        if result is not None:
            return result
        bits *= 2
    raise _exhausted("sin")


def _quotient_bounds(a_lo: int, a_hi: int, b_lo: int, b_hi: int) -> tuple[float, float] | None:
    """(fl(min a/b), fl(max a/b)) over the boxes, or None if [b_lo, b_hi] contains 0 or the bounds overflow."""
    if b_lo <= 0 <= b_hi:
        return None
    try:
        candidates = (a_lo / b_lo, a_lo / b_hi, a_hi / b_lo, a_hi / b_hi)
    except OverflowError:
        return None
    return min(candidates), max(candidates)


def tan(x: object) -> float:
    """``math.tan(x)`` correctly rounded to nearest-even."""
    x = _float(x)
    if x == 0.0 or not math.isfinite(x):
        return math.tan(x)
    hint = math.tan(x)
    bits = _first_bits(hint, max(0, math.frexp(hint)[1]))
    while bits <= _MAX_BITS:
        q, r, reduction_error = _reduce(x, bits)
        s, c, error = _sin_cos_fixed(r, bits)
        error += reduction_error
        if q & 1:  # tan(q pi/2 + r) = -cos r / sin r
            bounds = _quotient_bounds(-c - error, -c + error, s - error, s + error)
        else:
            bounds = _quotient_bounds(s - error, s + error, c - error, c + error)
        # fl is monotone, so fl(min) == fl(max) fixes fl of every quotient in the box, the exact tangent included
        if bounds is not None and bounds[0] == bounds[1]:
            return bounds[0]
        bits *= 2
    raise _exhausted("tan")


def atan2(y: object, x: object) -> float:
    """``math.atan2(y, x)`` correctly rounded to nearest-even."""
    y = _float(y)
    x = _float(x)
    if y == 0.0 or not (math.isfinite(x) and math.isfinite(y)):
        return math.atan2(y, x)  # CPython's special cases, answered without the C library
    bits = _first_bits(math.atan2(y, x))
    y_num, y_den = abs(y).as_integer_ratio()
    if x == 0.0:
        numerator = denominator = 0
    else:
        x_num, x_den = abs(x).as_integer_ratio()
        numerator, denominator = y_num * x_den, y_den * x_num  # |y| / |x| exactly
    while bits <= _MAX_BITS:
        if x == 0.0:
            half_pi, pi_error = _pi_bits(bits - 1)
            lo, hi = half_pi - pi_error, half_pi + pi_error
        else:
            lo, hi = _atan_ratio(numerator, denominator, bits)
            if x < 0.0:
                pi, pi_error = _pi_bits(bits)
                lo, hi = pi - pi_error - hi, pi + pi_error - lo
        result = _round(max(lo, 0), hi, bits)  # the magnitude is positive; it may round to +0
        if result is not None:
            return -result if y < 0.0 else result
        bits *= 2
    raise _exhausted("atan2")


def _arcsine_parts(a: float, bits: int) -> tuple[int, int, bool]:
    """For 0 <= a <= 1 with s = sqrt(1 - a^2): ([lo, hi] / 2^bits around phi, small) where phi = atan(a / s) when
    a <= s (small) and phi = atan(s / a) otherwise, so asin a = phi or pi/2 - phi."""
    numerator, denominator = a.as_integer_ratio()
    square_den = denominator * denominator
    rest = square_den - numerator * numerator  # (1 - a^2) denominator^2, exact
    root = _isqrt((rest << (2 * bits)) // square_den)  # s 2^bits in [root, root + 1)
    if 2 * numerator * numerator <= square_den:  # a <= 1/sqrt 2 <= s: t = a / s <= 1, s >= 1/sqrt 2 so root > 0
        top = numerator << (2 * bits)
        t_lo = top // (denominator * (root + 1))
        t_hi = -(-top // (denominator * root))
        lo, hi = _atan_interval(t_lo, t_hi, bits)
        return lo, hi, True
    # a > s: t = s / a < 1
    t_lo = (root * denominator) // numerator
    t_hi = -(-((root + 1) * denominator) // numerator)
    lo, hi = _atan_interval(t_lo, t_hi, bits)
    return lo, hi, False


def asin(x: object) -> float:
    """``math.asin(x)`` correctly rounded to nearest-even."""
    x = _float(x)
    if x == 0.0 or not abs(x) <= 1.0:
        return math.asin(x)  # +-0 -> +-0, nan -> nan, |x| > 1 -> ValueError
    a = abs(x)
    bits = _first_bits(math.asin(a))
    while bits <= _MAX_BITS:
        lo, hi, small = _arcsine_parts(a, bits)
        if not small:
            half_pi, pi_error = _pi_bits(bits - 1)
            lo, hi = half_pi - pi_error - hi, half_pi + pi_error - lo
        result = _round(lo, hi, bits)
        if result is not None:
            return -result if x < 0.0 else result
        bits *= 2
    raise _exhausted("asin")


def acos(x: object) -> float:
    """``math.acos(x)`` correctly rounded to nearest-even."""
    x = _float(x)
    if x == 1.0 or not abs(x) <= 1.0:
        return math.acos(x)  # 1 -> +0, nan -> nan, |x| > 1 -> ValueError
    a = abs(x)
    bits = _first_bits(math.acos(x))
    while bits <= _MAX_BITS:
        lo, hi, small = _arcsine_parts(a, bits)
        if small:  # acos a = pi/2 - asin a = pi/2 - phi
            half_pi, pi_error = _pi_bits(bits - 1)
            lo, hi = half_pi - pi_error - hi, half_pi + pi_error - lo
        # else acos a = pi/2 - (pi/2 - phi) = phi
        if x < 0.0:  # acos(-a) = pi - acos a
            pi, pi_error = _pi_bits(bits)
            lo, hi = pi - pi_error - hi, pi + pi_error - lo
        result = _round(lo, hi, bits)
        if result is not None:
            return result
        bits *= 2
    raise _exhausted("acos")
