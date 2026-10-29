#!/usr/bin/env python3
"""Correct rounding of the MBF engine's acos, asin, atan2, sin and tan (DEL-17-04, Rust/Python parity ruling (a)).

`core/handoff/caepipe_mbf/correctly_rounded.py` must return, for every binary64 input, the binary64 nearest the
exact value, ties to even, with `math`'s domain errors, special values and signed zeros. These tests compare it with
an independent oracle written here: Python `decimal` arithmetic (not the module's binary fixed point), pi by the
Gauss-Legendre AGM (not Machin's formula), arctangent by a 1/64 table and a short Taylor series (not argument
halving), the sine and cosine series after reduction by a multiple of 2 pi (not pi/2), and arcsine and arccosine by
their own identities. The oracle evaluates at two precisions and accepts a binary64 only when an interval 16 times
the two evaluations' difference rounds to it; otherwise it doubles the precision.

Hard-to-round inputs come from the CORE-MATH project's worst-case test inputs (`src/binary64/<function>/<function>.wc`,
https://gitlab.inria.fr/core-math/core-math, MIT licence), which collect V. Lefevre's testlibm worst cases
(https://www.vinc17.net/research/testlibm/) and BaCSeL searches (V. Lefevre, J.-M. Muller, "Worst cases for correct
rounding of the elementary functions in double precision", ARITH 15, 2001). A few inputs are quoted per function,
each checked here to lie within 2^-40 ulp of a rounding boundary. No table is reproduced.

Standard claim fence applies (claims taxonomy per DEC-081): these tests check the floating-point evaluation only.
"""

from __future__ import annotations

import decimal
from decimal import Decimal
from fractions import Fraction
import math
from pathlib import Path
import random
import struct
import sys
import types

import pytest


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))
TESTS = Path(__file__).resolve().parent
if str(TESTS) not in sys.path:
    sys.path.insert(0, str(TESTS))

from core.handoff.caepipe_mbf import correctly_rounded as cr  # noqa: E402


# ---------------------------------------------------------------------------------------------------------------
# The oracle
# ---------------------------------------------------------------------------------------------------------------

class Oracle:
    """High-precision decimal values of the five functions at a binary64 input, rounded to binary64."""

    def __init__(self) -> None:
        self._pi: dict[int, Decimal] = {}
        self._table: dict[tuple[int, int], Decimal] = {}

    @staticmethod
    def context(digits: int) -> decimal.Context:
        return decimal.Context(prec=digits, rounding=decimal.ROUND_HALF_EVEN, Emax=10 ** 6, Emin=-10 ** 6)

    def pi(self, digits: int) -> Decimal:
        value = self._pi.get(digits)
        if value is None:
            with decimal.localcontext(self.context(digits + 10)):
                a, b, t, p = Decimal(1), Decimal(1) / Decimal(2).sqrt(), Decimal(1) / 4, Decimal(1)
                while True:  # Gauss-Legendre: quadratic convergence
                    a_next = (a + b) / 2
                    b = (a * b).sqrt()
                    t -= p * (a - a_next) ** 2
                    p *= 2
                    if a_next == a:
                        break
                    a = a_next
                value = (a + b) ** 2 / (4 * t)
            self._pi[digits] = value
        return value

    @staticmethod
    def _series_sin_cos(r: Decimal, digits: int) -> tuple[Decimal, Decimal]:
        with decimal.localcontext(Oracle.context(digits + 10)):
            square = r * r
            sine = term = r
            k = 1
            while True:
                term = -term * square / ((2 * k) * (2 * k + 1))
                if term == 0 or abs(term) < abs(sine).scaleb(-digits - 8):
                    break
                sine += term
                k += 1
            cosine = term = Decimal(1)
            k = 1
            while True:
                term = -term * square / ((2 * k - 1) * (2 * k))
                if term == 0 or abs(term) < abs(cosine).scaleb(-digits - 8):
                    break
                cosine += term
                k += 1
        return sine, cosine

    def reduced(self, x: float, digits: int) -> Decimal:
        """x - k 2 pi in [-pi, pi], exact to about 10^-(digits + 10)."""
        value = Decimal(x)
        width = digits + max(0, value.adjusted()) + 20
        with decimal.localcontext(self.context(width)):
            two_pi = 2 * self.pi(width)
            turns = (value / two_pi).to_integral_value(rounding=decimal.ROUND_HALF_EVEN)
            return value - turns * two_pi

    def sin(self, x: float, digits: int) -> Decimal:
        return self._series_sin_cos(self.reduced(x, digits), digits)[0]

    def tan(self, x: float, digits: int) -> Decimal:
        sine, cosine = self._series_sin_cos(self.reduced(x, digits), digits)
        with decimal.localcontext(self.context(digits + 10)):
            return sine / cosine

    def _atan_table(self, k: int, digits: int) -> Decimal:
        """atan(k / 64) by Newton's iteration on sin(theta) - c cos(theta) = 0."""
        key = (k, digits)
        value = self._table.get(key)
        if value is None:
            with decimal.localcontext(self.context(digits + 10)):
                c = Decimal(k) / 64
                theta = Decimal(math.atan(k / 64))
                for _ in range(64):
                    sine, cosine = self._series_sin_cos(theta, digits)
                    step = (sine - c * cosine) / (cosine + c * sine)
                    theta -= step
                    if step == 0 or abs(step) < Decimal(1).scaleb(-digits - 8):
                        break
                value = theta
            self._table[key] = value
        return value

    def atan(self, t: Decimal, digits: int) -> Decimal:
        """atan(t) for t >= 0."""
        with decimal.localcontext(self.context(digits + 10)):
            if t > 1:
                return self.pi(digits) / 2 - self.atan(1 / t, digits)
            k = int((t * 64).to_integral_value(rounding=decimal.ROUND_HALF_EVEN))
            c = Decimal(k) / 64
            u = (t - c) / (1 + t * c)  # |u| <= 1/128
            square = u * u
            total = term = u
            n = 1
            while True:
                term = -term * square
                n += 2
                step = term / n
                if step == 0 or abs(step) < abs(total).scaleb(-digits - 8):
                    break
                total += step
            return self._atan_table(k, digits) + total if k else total

    def atan2(self, y: float, x: float, digits: int) -> Decimal:
        """atan2(|y|, x) magnitude for finite y != 0 and finite x."""
        with decimal.localcontext(self.context(digits + 10)):
            if x == 0:
                return self.pi(digits) / 2
            angle = self.atan(abs(Decimal(y)) / abs(Decimal(x)), digits)
            return self.pi(digits) - angle if x < 0 else angle

    def asin(self, a: float, digits: int) -> Decimal:
        """asin(|a|) magnitude for 0 < |a| <= 1, as atan(a / sqrt((1 - a)(1 + a)))."""
        with decimal.localcontext(self.context(digits + 10)):
            value = abs(Decimal(a))
            if value == 1:
                return self.pi(digits) / 2
            rest = ((1 - value) * (1 + value)).sqrt()
            return self.atan(value / rest, digits)

    def acos(self, c: float, digits: int) -> Decimal:
        """acos(c) for -1 <= c < 1, as atan(sqrt((1 - c)(1 + c)) / |c|) from the right quadrant."""
        with decimal.localcontext(self.context(digits + 10)):
            value = Decimal(c)
            if value == 0:
                return self.pi(digits) / 2
            rest = ((1 - value) * (1 + value)).sqrt()
            angle = self.atan(rest / abs(value), digits)
            return angle if value > 0 else self.pi(digits) - angle

    @staticmethod
    def to_binary64(value: Fraction) -> float:
        return value.numerator / value.denominator  # int / int is correctly rounded

    def rounded(self, name: str, *args: float) -> tuple[float, Fraction]:
        """(the correctly rounded binary64, the high-precision value) of `name` at `args`."""
        evaluate = getattr(self, name)
        digits = 40
        while digits <= 2000:
            first = Fraction(evaluate(*args, digits))
            second = Fraction(evaluate(*args, digits + 20))
            spread = max(abs(first - second), abs(second) / 10 ** (digits + 10))
            low = self.to_binary64(second - 16 * spread)
            high = self.to_binary64(second + 16 * spread)
            if low == high:
                return low, second
            digits *= 2
        raise AssertionError(f"oracle did not resolve {name}{args}")  # pragma: no cover


ORACLE = Oracle()


def expected(name: str, *args: float) -> float:
    """The correctly rounded result of the module function `name`, from the oracle."""
    if name == "atan2":
        y, x = args
        magnitude = ORACLE.rounded("atan2", y, x)[0]
        return -magnitude if y < 0 else magnitude
    if name == "asin":
        magnitude = ORACLE.rounded("asin", args[0])[0]
        return -magnitude if args[0] < 0 else magnitude
    return ORACLE.rounded(name, *args)[0]


def same(a: float, b: float) -> bool:
    """Bitwise equality (signed zeros and NaN payloads apart, NaN equals NaN)."""
    if math.isnan(a) or math.isnan(b):
        return math.isnan(a) and math.isnan(b)
    return a == b and math.copysign(1.0, a) == math.copysign(1.0, b)


def boundary_bits(name: str, *args: float) -> float:
    """-log2 of the exact value's distance to the nearest rounding boundary, in ulps of the result."""
    result, value = ORACLE.rounded(name, *args)
    magnitude = abs(result)
    above = math.nextafter(magnitude, math.inf)
    below = math.nextafter(magnitude, 0.0)
    exact = abs(value)
    distance = min(abs(exact - (Fraction(magnitude) + Fraction(above)) / 2),
                   abs(exact - (Fraction(magnitude) + Fraction(below)) / 2))
    return -math.log2(distance / Fraction(math.ulp(magnitude)))


# ---------------------------------------------------------------------------------------------------------------
# Hard-to-round inputs (CORE-MATH worst-case files, see the module docstring of this test)
# ---------------------------------------------------------------------------------------------------------------

# Each is quoted from the named section of the CORE-MATH file `src/binary64/<function>/<function>.wc`.
HARD_CASES = {
    "sin": [
        ("0x1.997d35866ce04p-14",),  # testlibm worst cases from 0 to pi
        ("0x1.4c377e1d8441fp-6",),  # testlibm worst cases from 0 to pi
        ("0x1.0380e8e041344p+1023",),  # BaCSeL worst cases in [2^1023, 2^1024)
        ("0x1.e3cf476542bd2p-25",),  # BaCSeL worst cases for 0 < x < 2^11
    ],
    "tan": [
        ("0x1.dffffffffff1fp-22",),  # testlibm worst cases from 0 to pi (about 79 bits after the round bit)
        ("0x1.bf9f47347b021p-8",),  # testlibm worst cases from 0 to pi
        ("0x1.da898d9a94d29p+4",),  # BaCSeL worst cases below 10.5 pi
        ("0x1.4e1f2d2cbbca8p+383",),  # hard-to-round cases for x >= 2^10
    ],
    "asin": [
        ("0x1.27225319ee943p-15",),  # testlibm and BaCSeL worst cases for 2^-26 <= x <= 1
        ("0x1.06800e629d667p-23",),
        ("0x1.255e30ce85775p-4",),
    ],
    "acos": [
        ("0x1.d18f3b5c96858p-3",),  # testlibm worst cases
        ("0x1.cf37d41c69889p-24",),  # testlibm hrcases-trig worst cases
        ("0x1.d0bfb34fbe039p-4",),  # BaCSeL worst cases
        ("-0x1.8b79e6cb1a3e8p-19",),  # BaCSeL worst cases for -1 <= x < 0
        ("-0x1.39676733ae8fep-52",),
    ],
    "atan2": [
        ("0xb.cd9b43114a29p+76", "0x1.47362dee55845p+52"),  # worst cases (about 93 bits after the round bit)
        ("0x1.291d62491a95bp+52", "0x1.9d70bcd22dbf4p+52"),
    ],
}

# Worst cases for the argument reduction, quoted from CORE-MATH `sin.wc`: in their binades these inputs come closest
# to a multiple of 2 pi (the first two) or of pi (the last two), so sin and tan are evaluated far from x's magnitude.
REDUCTION_CASES = ["0x1.0d09901565674p+614", "0x1.140e5e9156e6cp+621", "0x1.a71df2723e34cp+750",
                   "0x1.54a28326cfedep+268"]


@pytest.mark.parametrize("name, args", [(name, args) for name, cases in HARD_CASES.items() for args in cases])
def test_hard_to_round_cases_from_the_literature(name, args):
    inputs = tuple(float.fromhex(value) for value in args)
    assert boundary_bits(name, *inputs) >= 40  # each case is genuinely hard to round
    assert same(getattr(cr, name)(*inputs), expected(name, *inputs)), inputs


@pytest.mark.parametrize("name", ["sin", "tan"])
@pytest.mark.parametrize("text", REDUCTION_CASES)
def test_argument_reduction_worst_cases(name, text):
    x = float.fromhex(text)
    assert same(getattr(cr, name)(x), expected(name, x))
    assert same(getattr(cr, name)(-x), -expected(name, x))  # odd functions


# ---------------------------------------------------------------------------------------------------------------
# Sampled inputs: 10^5 per function across its domain, against the oracle
# ---------------------------------------------------------------------------------------------------------------

SAMPLES = 100_000


def _any_finite(rng: random.Random) -> float:
    """A uniformly random finite nonzero bit pattern: every binade, subnormals included, equally likely."""
    while True:
        value = struct.unpack("<d", struct.pack("<Q", rng.getrandbits(64)))[0]
        if math.isfinite(value) and value != 0.0:
            return value


def _log_uniform(rng: random.Random, low: int, high: int) -> float:
    """A random sign and mantissa in a random binade 2^[low, high)."""
    return math.ldexp(1.0 + rng.getrandbits(52) / 2.0 ** 52, rng.randrange(low, high)) * rng.choice((1.0, -1.0))


def _unit(rng: random.Random) -> float:
    """An input of acos and asin: uniform, any binade below 1, or near -1 and 1."""
    kind = rng.randrange(5)
    if kind < 2:
        return rng.uniform(-1.0, 1.0)
    if kind < 4:
        while True:  # nonzero: the zeros are special values, tested below
            value = math.ldexp(rng.getrandbits(53) / 2.0 ** 53, -rng.randrange(0, 1075)) * rng.choice((1.0, -1.0))
            if value != 0.0:
                return value
    return (1.0 - rng.randrange(1, 1 << 20) * 2.0 ** -53) * rng.choice((1.0, -1.0))


def _angle(rng: random.Random) -> float:
    """An input of sin and tan: any binade, a few turns, moderate magnitudes or up to 2^20."""
    kind = rng.randrange(4)
    if kind == 0:
        return _any_finite(rng)
    if kind == 1:
        return rng.uniform(-8.0, 8.0)
    if kind == 2:
        return _log_uniform(rng, -30, 30)
    return rng.uniform(-2.0 ** 20, 2.0 ** 20)


def _pair(rng: random.Random) -> tuple[float, float]:
    """An input of atan2: any binades, model-coordinate magnitudes, or nearby binades of either sign."""
    kind = rng.randrange(3)
    if kind == 0:
        return _any_finite(rng), _any_finite(rng)
    if kind == 1:
        return rng.uniform(-1e4, 1e4), rng.uniform(-1e4, 1e4)
    return _log_uniform(rng, -60, 60), _log_uniform(rng, -60, 60)


def _check_samples(name: str, inputs) -> None:
    function = getattr(cr, name)
    wrong = []
    for args in inputs:
        if not same(function(*args), expected(name, *args)):
            wrong.append(args)
    assert not wrong, [tuple(value.hex() for value in args) for args in wrong[:10]]


# One class per function, so the parallel runner's `--dist loadscope` spreads them over workers.
class TestSampledSin:
    def test_sampled_inputs(self):
        rng = random.Random(0x5111)
        _check_samples("sin", [(_angle(rng),) for _ in range(SAMPLES)])


class TestSampledTan:
    def test_sampled_inputs(self):
        rng = random.Random(0x7A11)
        _check_samples("tan", [(_angle(rng),) for _ in range(SAMPLES)])


class TestSampledAsin:
    def test_sampled_inputs(self):
        rng = random.Random(0xA511)
        _check_samples("asin", [(_unit(rng),) for _ in range(SAMPLES)])


class TestSampledAcos:
    def test_sampled_inputs(self):
        rng = random.Random(0xAC05)
        _check_samples("acos", [(_unit(rng),) for _ in range(SAMPLES)])


class TestSampledAtan2:
    def test_sampled_inputs(self):
        rng = random.Random(0xA7A2)
        _check_samples("atan2", [_pair(rng) for _ in range(SAMPLES)])


# ---------------------------------------------------------------------------------------------------------------
# Special values, domain errors, signed zeros and argument conversion: those of `math`
# ---------------------------------------------------------------------------------------------------------------

TINY = 5e-324
HUGE = sys.float_info.max
SPECIAL = [0.0, -0.0, math.inf, -math.inf, math.nan, 1.0, -1.0, 2.0, -2.0, TINY, -TINY, HUGE, -HUGE,
           math.nextafter(1.0, 2.0), math.nextafter(-1.0, -2.0), math.nextafter(1.0, 0.0), math.nextafter(-1.0, 0.0),
           sys.float_info.min, 2.0 ** -1022 - TINY]


def _outcome(function, *args):
    try:
        return ("value", function(*args))
    except (ValueError, OverflowError, TypeError) as error:
        return (type(error), str(error))


@pytest.mark.parametrize("name", ["sin", "tan", "asin", "acos"])
@pytest.mark.parametrize("x", SPECIAL)
def test_special_values_and_domain_errors_match_math(name, x):
    ours = _outcome(getattr(cr, name), x)
    theirs = _outcome(getattr(math, name), x)
    if ours[0] != "value" or theirs[0] != "value":
        assert ours == theirs  # the same exception type and message
    elif math.isnan(theirs[1]) or math.isinf(x) or x == 0.0 or (name == "acos" and x == 1.0):
        assert same(ours[1], theirs[1])  # NaN, signed zero, exact results
    else:
        assert same(ours[1], expected(name, x))


def test_the_exact_special_results():
    assert same(cr.sin(-0.0), -0.0) and same(cr.tan(-0.0), -0.0) and same(cr.asin(-0.0), -0.0)
    assert same(cr.acos(1.0), 0.0)
    assert cr.acos(-1.0) == math.pi and cr.acos(0.0) == math.pi / 2 and cr.acos(-0.0) == math.pi / 2
    assert cr.asin(1.0) == math.pi / 2 and cr.asin(-1.0) == -math.pi / 2
    assert same(cr.sin(TINY), TINY) and same(cr.tan(-TINY), -TINY) and same(cr.asin(TINY), TINY)
    for name in ("sin", "tan"):
        for x in (math.inf, -math.inf):
            with pytest.raises(ValueError, match="math domain error"):
                getattr(cr, name)(x)
    for name in ("asin", "acos"):
        for x in (math.nextafter(1.0, 2.0), -2.0, math.inf):
            with pytest.raises(ValueError, match="math domain error"):
                getattr(cr, name)(x)
        assert math.isnan(getattr(cr, name)(math.nan))


ATAN2_SPECIAL = [0.0, -0.0, math.inf, -math.inf, math.nan, 1.0, -1.0, TINY, -TINY, HUGE, -HUGE]


@pytest.mark.parametrize("y", ATAN2_SPECIAL)
@pytest.mark.parametrize("x", ATAN2_SPECIAL)
def test_atan2_special_values_match_math(y, x):
    ours = cr.atan2(y, x)
    if y == 0.0 or not (math.isfinite(x) and math.isfinite(y)):
        assert same(ours, math.atan2(y, x))  # CPython's own special cases, not the C library
    elif x == 0.0:
        assert same(ours, math.copysign(math.pi / 2, y))
    else:
        assert same(ours, expected("atan2", y, x))


def test_atan2_results_below_the_least_subnormal_round_to_signed_zero():
    assert same(cr.atan2(TINY, HUGE), 0.0) and same(cr.atan2(-TINY, HUGE), -0.0)
    assert same(cr.atan2(1e-300, 1e300), 0.0) and same(cr.atan2(-1e-300, 1e300), -0.0)
    assert same(cr.atan2(-1e-300, -1e300), -math.pi)
    # atan2(y, x) = (y / x)(1 - (y / x)^2 / 3 + ...): with y / x = 2^-1075 exactly the value lies just below the
    # midpoint 2^-1075 and rounds to 0; one ulp less x puts it just above, and it rounds to the least subnormal.
    assert same(cr.atan2(TINY, 2.0), 0.0) and same(cr.atan2(-TINY, 2.0), -0.0)
    assert same(cr.atan2(TINY, math.nextafter(2.0, 0.0)), TINY)
    assert same(expected("atan2", TINY, math.nextafter(2.0, 0.0)), TINY) and same(expected("atan2", TINY, 2.0), 0.0)


@pytest.mark.parametrize("name", ["sin", "tan", "asin", "acos"])
def test_arguments_are_converted_as_math_converts_them(name):
    function, reference = getattr(cr, name), getattr(math, name)
    assert same(function(0), reference(0)) and same(function(True), function(1.0))
    assert same(function(Fraction(1, 3)), function(1 / 3)) and same(function(Decimal("0.25")), function(0.25))
    for bad in ("0.5", None, [0.5]):
        assert _outcome(function, bad) == _outcome(reference, bad)
    assert _outcome(function, 10 ** 400) == _outcome(reference, 10 ** 400)  # OverflowError
    assert _outcome(cr.atan2, "1", 1.0) == _outcome(math.atan2, "1", 1.0)
    assert same(cr.atan2(1, 2), cr.atan2(1.0, 2.0))


# ---------------------------------------------------------------------------------------------------------------
# The node 20B repro of the Rust port's review
# ---------------------------------------------------------------------------------------------------------------

REPRO_LAYOUT = ("F10,KA,MM1,PP1", "T20,KI,X10000,B100", "T30,X-1975,Y8724")
# The two calls the importer makes for this bend (theta = acos(cosine), tangent length = R tan(theta / 2)), with
# their correctly rounded results. macOS's C library returns 1.793430522422298 and 1.2516923300464655 here.
REPRO_CALLS = [
    ("acos", ("-0x1.c4329053dc30ap-3",), "0x1.cb1e434173b40p+0"),
    ("tan", ("0x1.cb1e434173b40p-1",), "0x1.406ee896341fbp+0"),
]


@pytest.mark.parametrize("name, args, result", REPRO_CALLS)
def test_the_trig_values_that_feed_node_20b(name, args, result):
    inputs = tuple(float.fromhex(value) for value in args)
    assert same(getattr(cr, name)(*inputs), float.fromhex(result))
    assert same(expected(name, *inputs), float.fromhex(result))


def test_node_20b_is_the_rust_value(monkeypatch):
    import test_caepipe_mbf_importer as importer_tests
    from core.handoff.caepipe_mbf import importer as importer_module
    from core.handoff.caepipe_mbf import si_writer as si_writer_module

    data = importer_tests.mbf(*REPRO_LAYOUT, loads=None)
    result = importer_tests.run(data)
    assert result.status == "imported", result.diagnostics
    nodes = {node["id"]: node["position"] for node in result.model["nodes"]}
    assert nodes["node:20B"]["y"] == 122.07995677558971  # the Rust port's value, on every platform

    # The importer and the SI writer take every acos, asin, atan2, sin and tan from the module, none from `math`.
    assert importer_module.correctly_rounded is cr and si_writer_module.correctly_rounded is cr
    calls = []
    proxy = types.SimpleNamespace()
    for name in ("acos", "asin", "atan2", "sin", "tan"):
        def recorded(*args, _name=name):
            value = getattr(cr, _name)(*args)
            calls.append((_name, tuple(arg.hex() for arg in args), value.hex()))
            return value
        setattr(proxy, name, recorded)
    monkeypatch.setattr(importer_module, "correctly_rounded", proxy)
    trigless = types.SimpleNamespace(**{key: getattr(math, key) for key in dir(math)
                                        if not key.startswith("_") and key not in ("acos", "asin", "atan2", "sin", "tan")})
    monkeypatch.setattr(importer_module, "math", trigless)  # a remaining math trig call would fail
    again = importer_tests.run(data)
    assert again.model_bytes() == result.model_bytes()
    assert calls == [(name, tuple(float.fromhex(a).hex() for a in args), float.fromhex(value).hex())
                     for name, args, value in REPRO_CALLS]
