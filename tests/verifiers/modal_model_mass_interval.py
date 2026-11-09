"""Rigorous interval arithmetic for the independent model-derived modal mass reader
(`tests/verifiers/modal_model_mass.py`; DEL-04-09 Design, "Selected mass assurance:
next modal unit after CE").

Every interval [lo, hi] has exact rational endpoints (`fractions.Fraction`) and holds
the real value it stands for. After each operation the endpoints are rounded OUTWARD
to dyadic rationals of `PREC` significant bits, so the enclosure stays sound and the
operand sizes stay bounded whatever the input. Nothing here is a tolerance: widths
come only from that outward rounding (about 2^-PREC relative per operation) and from
the transcendental remainders bounded below.

Two kinds of widening are kept apart:

- `Iv` operations enclose the EXACT real result of an exact formula (the Design's).
- `binary64` / `scaled` / `dd` enclose what a PRODUCER computing the same formula
  with rounded arithmetic can return. Their premises (the rounding model) are stated
  where they are defined; a value outside a premise's range raises `OutsideModel`,
  which the caller reports as unresolved, never as passed.

Standard library only; it imports no product code and no other reader.
"""
from __future__ import annotations

from fractions import Fraction
import math

PREC = 192
# binary64 constants, exact.
U = Fraction(1, 1 << 53)                      # unit roundoff of round-to-nearest binary64
NORMAL_MIN = Fraction(1, 1 << 1022)           # least positive normal binary64
SUBNORMAL_HALF = Fraction(1, 1 << 1075)       # half the least positive subnormal
LARGEST = Fraction(float.fromhex("0x1.fffffffffffffp+1023"))
PROFILE_FLOOR = Fraction(1 << 53)             # magnitudes the checked profile cannot carry
# Double-double (≈106-bit) formation and accumulation, Design §3.4 check 1. The
# kernel's `dd.rs` operations are AccurateDWPlusDW (`add`, 3u²), DWPlusFP (`add_f64`,
# 2u²), DWTimesFP1 (`mul_f64`, 1.5u²) and DWTimesDW1 (`mul`, 7u²) (Joldes, Muller and
# Popescu, "Tight and rigorous error bounds for basic building blocks of double-word
# arithmetic", ACM TOMS 44(2), 2017); its division, square root and series carry
# the kernel's stated "small multiple of 2^-106". Premise: every such operation errs
# by less than 2^-100 of the magnitudes it combines, and one contribution's
# evaluation path (frame, local entry, series of at most 60 terms
# (`SERIES_MAX_TERMS`), rotation) is under 2^10 operations, so ONE contribution
# deviates by at most 2^-90 of its absolute scale. A member's scale carries 1 + κ,
# κ = |y|/|y⊥| (its frame is formed in double word from y). Each further double-word
# accumulation adds at most 2^-90 of the absolute term sum again, so a value
# accumulated from n contributions errs by at most 2^-90·(n + 1)·Σ scale
# (`dd_bound`). This is a stated premise about the producer's double-word path,
# far below one binary64 rounding; it decides only where terms cancel.
DD_RELATIVE = Fraction(1, 1 << 90)


# Below 2^-969 a double word's low word (about 2^-53 of it) is no longer normal, so the
# relative premise above fails (the kernel's own arc guard, `LOCAL_ENTRY_MIN`, is this
# floor). And once 2^-90·(n + 1)·(1 + κ) reaches one binary64 unit roundoff 2^-53 of a
# value's scale, the premise no longer sits below binary64 resolution. Either way the
# value is outside the rounding model: unresolved, never passed.
DD_LOW_WORD_FLOOR = Fraction(1, 1 << 969)
DD_RESOLUTION_LIMIT = 1 << 37


def below_floor(x: "Iv") -> bool:
    """Whether a nonzero double-word intermediate may be below the low-word floor."""
    return not (x.lo == 0 and x.hi == 0) and x.mig() < DD_LOW_WORD_FLOOR


def dd_bound(count: int, scale: Fraction) -> Fraction:
    """The double-word error allowance of a value accumulated from `count`
    contributions whose absolute (κ-weighted) scales sum to `scale`."""
    return DD_RELATIVE * (count + 1) * scale


class OutsideModel(Exception):
    """A value leaves the range in which the stated rounding model holds (overflow,
    or an operand the model does not cover). The comparison it feeds is unresolved."""


class Undecidable(Exception):
    """An interval operation is undefined on its enclosure (a divisor or a square-root
    operand that may be zero or negative)."""


def _floor(x: Fraction) -> Fraction:
    if x == 0:
        return x
    n, d = x.numerator, x.denominator
    shift = PREC - (abs(n).bit_length() - d.bit_length())
    if shift >= 0:
        return Fraction((n << shift) // d, 1 << shift)
    return Fraction((n // (d << -shift)) << -shift)


def _ceil(x: Fraction) -> Fraction:
    return -_floor(-x)


def exact(value) -> Fraction:
    """The exact rational value of a finite binary64 number (or an int/Fraction)."""
    if isinstance(value, Fraction):
        return value
    if isinstance(value, bool):
        raise TypeError("a boolean is not a number")
    if isinstance(value, float):
        if not math.isfinite(value):
            raise ValueError("a non-finite binary64 value has no exact value")
        return Fraction(value)
    if isinstance(value, int):
        return Fraction(value)
    raise TypeError(f"not a number: {value!r}")


class Iv:
    """A closed interval of reals with exact rational ends."""

    __slots__ = ("lo", "hi")

    def __init__(self, lo, hi=None):
        lo = exact(lo)
        hi = lo if hi is None else exact(hi)
        if lo > hi:
            raise ValueError("an interval's lower end exceeds its upper end")
        self.lo, self.hi = lo, hi

    @staticmethod
    def point(value) -> "Iv":
        return Iv(value)

    def is_point(self) -> bool:
        return self.lo == self.hi

    def mag(self) -> Fraction:
        return max(abs(self.lo), abs(self.hi))

    def mig(self) -> Fraction:
        """The least magnitude in the interval."""
        if self.lo <= 0 <= self.hi:
            return Fraction(0)
        return min(abs(self.lo), abs(self.hi))

    def contains(self, x) -> bool:
        x = exact(x)
        return self.lo <= x <= self.hi

    def contains_interval(self, other: "Iv") -> bool:
        return self.lo <= other.lo and other.hi <= self.hi

    def meets(self, other: "Iv") -> bool:
        return max(self.lo, other.lo) <= min(self.hi, other.hi)

    def excludes_zero(self) -> bool:
        return self.lo > 0 or self.hi < 0

    def __add__(self, other):
        other = _iv(other)
        return Iv(_floor(self.lo + other.lo), _ceil(self.hi + other.hi))

    __radd__ = __add__

    def __neg__(self):
        return Iv(-self.hi, -self.lo)

    def __sub__(self, other):
        return self + (-_iv(other))

    def __rsub__(self, other):
        return _iv(other) + (-self)

    def __mul__(self, other):
        other = _iv(other)
        if self.is_point() and other.is_point():
            v = self.lo * other.lo
            return Iv(_floor(v), _ceil(v))
        ends = (self.lo * other.lo, self.lo * other.hi, self.hi * other.lo, self.hi * other.hi)
        return Iv(_floor(min(ends)), _ceil(max(ends)))

    __rmul__ = __mul__

    def __truediv__(self, other):
        other = _iv(other)
        if not other.excludes_zero():
            raise Undecidable("a divisor's enclosure contains zero")
        inverse_ends = (1 / other.lo, 1 / other.hi)
        reciprocal = Iv(min(inverse_ends), max(inverse_ends))
        ends = (self.lo * reciprocal.lo, self.lo * reciprocal.hi, self.hi * reciprocal.lo, self.hi * reciprocal.hi)
        return Iv(_floor(min(ends)), _ceil(max(ends)))

    def __rtruediv__(self, other):
        return _iv(other) / self

    def square(self) -> "Iv":
        if self.lo >= 0:
            return Iv(_floor(self.lo * self.lo), _ceil(self.hi * self.hi))
        if self.hi <= 0:
            return Iv(_floor(self.hi * self.hi), _ceil(self.lo * self.lo))
        return Iv(0, _ceil(max(self.lo * self.lo, self.hi * self.hi)))

    def abs(self) -> "Iv":
        if self.lo >= 0:
            return self
        if self.hi <= 0:
            return -self
        return Iv(0, self.mag())

    def hull(self, other: "Iv") -> "Iv":
        return Iv(min(self.lo, other.lo), max(self.hi, other.hi))

    def widen(self, absolute) -> "Iv":
        absolute = exact(absolute)
        return Iv(self.lo - absolute, self.hi + absolute)

    def __repr__(self):
        return f"Iv({float(self.lo)!r}, {float(self.hi)!r})"


def _iv(value) -> Iv:
    return value if isinstance(value, Iv) else Iv(value)


ZERO = Iv(0)
ONE = Iv(1)


# --- square root ------------------------------------------------------------------------

def _sqrt_floor(x: Fraction) -> Fraction:
    """A dyadic lower bound of √x (x ≥ 0) with about PREC significant bits."""
    if x <= 0:
        return Fraction(0)
    # Choose k so that x·4^k has about 2·PREC + 2 integer bits.
    bits = x.numerator.bit_length() - x.denominator.bit_length()
    k = max(0, (2 * PREC + 4 - bits) // 2 + 1)
    scaled = (x.numerator << (2 * k)) // x.denominator
    return Fraction(math.isqrt(scaled), 1 << k)


def _sqrt_ceil(x: Fraction) -> Fraction:
    if x <= 0:
        return Fraction(0)
    bits = x.numerator.bit_length() - x.denominator.bit_length()
    k = max(0, (2 * PREC + 4 - bits) // 2 + 1)
    scaled = -((-(x.numerator << (2 * k))) // x.denominator)  # ceil
    root = math.isqrt(scaled)
    if root * root < scaled:
        root += 1
    return Fraction(root, 1 << k)


def sqrt(x: Iv) -> Iv:
    if x.lo < 0:
        raise Undecidable("a square-root operand's enclosure reaches below zero")
    return Iv(_floor(_sqrt_floor(x.lo)), _ceil(_sqrt_ceil(x.hi)))


# --- arctangent, π and arcsine -------------------------------------------------------------

def _atan_small(x: Iv) -> Iv:
    """atan on |x| ≤ 1/16: the alternating Taylor series, with its remainder bounded by
    the first omitted term (the terms decrease in magnitude for |x| < 1)."""
    if x.mag() > Fraction(1, 16):
        raise ValueError("argument outside the series range")
    total = ZERO
    power = x
    square = x.square()
    n = 0
    bound = Fraction(1, 1 << (PREC + 8))
    while True:
        term = power / Iv(2 * n + 1)
        total = total + term if n % 2 == 0 else total - term
        power = power * square
        n += 1
        if power.mag() / (2 * n + 1) < bound:
            return total.widen(power.mag() / (2 * n + 1))


def _atan_point(x: Fraction) -> Iv:
    """An enclosure of atan(x) for a point x ≥ 0."""
    if x < 0:
        raise ValueError("negative argument")
    if x > 1:
        # atan(x) = π/2 − atan(1/x), with 1/x < 1.
        return _half_pi() - _atan_point(1 / x)
    y = Iv(x)
    halvings = 0
    # atan(y) = 2·atan(y / (1 + √(1 + y²))), monotone, until |y| ≤ 1/16.
    while y.mag() > Fraction(1, 16):
        y = y / (ONE + sqrt(ONE + y.square()))
        halvings += 1
    return _atan_small(y) * Iv(1 << halvings)


_PI_CACHE: list[Iv] = []


def pi() -> Iv:
    """π by Machin's formula, π = 16·atan(1/5) − 4·atan(1/239), each atan enclosed."""
    if not _PI_CACHE:
        value = Iv(16) * _atan_point(Fraction(1, 5)) - Iv(4) * _atan_point(Fraction(1, 239))
        _PI_CACHE.append(value)
    return _PI_CACHE[0]


def _half_pi() -> Iv:
    return pi() / Iv(2)


def atan(x: Iv) -> Iv:
    """atan on an interval of non-negative reals (monotone, so each end separately)."""
    if x.lo < 0:
        raise Undecidable("atan is enclosed here only for non-negative arguments")
    return Iv(_atan_point(x.lo).lo, _atan_point(x.hi).hi)


def asin_and_cos(s: Iv) -> tuple[Iv, Iv]:
    """(asin s, cos(asin s) = √(1 − s²)) for 0 < s < 1, each enclosed. asin s =
    atan(s/√(1 − s²)) is monotone in s, so each end is enclosed separately."""
    if not (s.lo > 0 and s.hi < 1):
        raise Undecidable("asin is enclosed here only on (0, 1)")

    def at(point: Fraction) -> Iv:
        p = Iv(point)
        return atan(p / sqrt(ONE - p.square()))

    angle = Iv(at(s.lo).lo, at(s.hi).hi)
    cosine = sqrt(ONE - s.square())
    return angle, cosine


# --- the producer's rounding (a superset of what it can return) ------------------------------

def _range_check(x: Iv, what: str) -> None:
    if x.hi > LARGEST or x.lo < -LARGEST:
        raise OutsideModel(f"{what}: may overflow the binary64 range")


def binary64(x: Iv, what: str = "a binary64 operation") -> Iv:
    """What one round-to-nearest binary64 operation can return for a real result in x:
    |fl(r) − r| ≤ u·|r| in the normal range and ≤ 2^-1075 below it. A result that is
    already a binary64 point is returned unchanged (its rounding is exact)."""
    _range_check(x, what)
    if x.is_point():
        try:
            if Fraction(float(x.lo)) == x.lo:
                return x
        except OverflowError:
            raise OutsideModel(f"{what}: may overflow the binary64 range") from None
    slack_lo = U * abs(x.lo)
    slack_hi = U * abs(x.hi)
    if x.mig() < NORMAL_MIN:
        slack_lo += SUBNORMAL_HALF
        slack_hi += SUBNORMAL_HALF
    return Iv(_floor(x.lo - slack_lo), _ceil(x.hi + slack_hi))


def scaled(x: Iv, operands_magnitude: Fraction = Fraction(0), what: str = "a scaled operation") -> Iv:
    """What one operation of a binary64 mantissa with a separate binary exponent can
    return (the product's `Scaled` area arithmetic): its mantissa rounds once (≤ u
    relative); an addend aligned below the result's exponent can lose at most 2^-1074
    of the larger operand. No exponent range applies until the final conversion
    (`binary64`)."""
    slack_lo = U * abs(x.lo) + operands_magnitude * Fraction(1, 1 << 1072)
    slack_hi = U * abs(x.hi) + operands_magnitude * Fraction(1, 1 << 1072)
    return Iv(_floor(x.lo - slack_lo), _ceil(x.hi + slack_hi))


def relative(x: Iv, bound: Fraction) -> Iv:
    """x with every value perturbed by a relative amount of at most `bound`."""
    return Iv(_floor(x.lo - bound * abs(x.lo)), _ceil(x.hi + bound * abs(x.hi)))


def published_image(x: Iv) -> Iv:
    """What a binary64 publication of a real value in x can read: x rounded to nearest,
    widened by u relative and 2^-1075 absolute; a value below the normal range may also
    be published as a flagged 0, so 0 is included there."""
    out = binary64(x, "a published value")
    if x.mig() < NORMAL_MIN:
        out = out.hull(ZERO)
    return out
