"""Number fields for the F1 mass and discrete-eigen references.

Standard library only. Three fields share one interface, so that every
element formula is written once and evaluated three ways:

  QPiField       exact a + b*pi with a, b rational (fractions.Fraction); used
                 for straight-member and component masses, whose entries are
                 rational multiples of pi or rational. A product of two
                 pi-carrying factors is refused, so nothing irrational leaks
                 into an "exact" string.
  PointField     decimal.Decimal at the working precision of the calling
                 thread's context (set by set_point_precision). Every
                 transcendental (pi, atan, sin, cos) is evaluated by series at
                 the working precision plus guard digits.
  IntervalField  outward-rounded decimal intervals [lo, hi] at a fixed
                 precision. Rationals enter by directed division; pi, atan
                 and sqrt are evaluated at extra precision and widened by one
                 unit in the last place on each side. A sign is known only
                 when the interval excludes zero. Used for the exact Sturm
                 counts (inertia of K - sigma*M), which are therefore
                 certified rather than estimated.

Interface of a field F:
  F.q(x)            rational (int or Fraction) -> number
  F.pi()            pi
  F.sqrt(x), F.atan(x)
  F.sign(x)         -1, 0, +1, or None when not decided (intervals only)
  F.is_zero(x)      exact structural zero
"""
import decimal
from decimal import Decimal, Context, ROUND_FLOOR, ROUND_CEILING, ROUND_HALF_EVEN
from fractions import Fraction

EXP = 999999


def make_context(prec, rounding=ROUND_HALF_EVEN):
    return Context(prec=prec, rounding=rounding, Emin=-EXP, Emax=EXP)


def set_point_precision(prec):
    decimal.setcontext(make_context(prec))


def to_decimal(x, ctx=None):
    """Exact-to-context conversion of int / Fraction / Decimal."""
    ctx = ctx or decimal.getcontext()
    if isinstance(x, Decimal):
        return ctx.plus(x)
    if isinstance(x, int):
        return ctx.plus(Decimal(x))
    x = Fraction(x)
    return ctx.divide(Decimal(x.numerator), Decimal(x.denominator))


# --------------------------------------------------------------------------
# Transcendentals in Decimal at a requested precision (series, guard digits)
# --------------------------------------------------------------------------
_PI_CACHE = {}


def _atan_inv_int(n, ctx):
    """atan(1/n) for integer n >= 2, Gregory series."""
    x = ctx.divide(Decimal(1), Decimal(n))
    x2 = ctx.multiply(x, x)
    eps = Decimal(10) ** (-(ctx.prec + 2))
    total, term, k, sign = x, x, 1, -1
    while True:
        term = ctx.multiply(term, x2)
        k += 2
        delta = ctx.divide(term, Decimal(k))
        if delta < eps:
            break
        total = ctx.add(total, delta) if sign > 0 else ctx.subtract(total, delta)
        sign = -sign
    return total


def pi_decimal(prec):
    """pi to `prec` significant digits (Machin, computed with 15 guard digits)."""
    if prec not in _PI_CACHE:
        ctx = make_context(prec + 15)
        p = ctx.subtract(ctx.multiply(Decimal(16), _atan_inv_int(5, ctx)),
                         ctx.multiply(Decimal(4), _atan_inv_int(239, ctx)))
        _PI_CACHE[prec] = make_context(prec).plus(p)
    return _PI_CACHE[prec]


def atan_decimal(x, prec):
    """atan(x) for Decimal x, to about `prec` digits."""
    ctx = make_context(prec + 20)
    x = ctx.plus(x)
    if x < 0:
        return -atan_decimal(-x, prec)
    if x == 0:
        return Decimal(0)
    invert = False
    if x > 1:
        x = ctx.divide(Decimal(1), x)
        invert = True
    halvings = 0
    while x > Decimal("0.01"):
        # atan(x) = 2 atan(x / (1 + sqrt(1 + x^2)))
        x = ctx.divide(x, ctx.add(Decimal(1), ctx.sqrt(ctx.add(Decimal(1), ctx.multiply(x, x)))))
        halvings += 1
    x2 = ctx.multiply(x, x)
    eps = Decimal(10) ** (-(prec + 18))
    total, term, k, sign = x, x, 1, -1
    while True:
        term = ctx.multiply(term, x2)
        k += 2
        delta = ctx.divide(term, Decimal(k))
        if delta < eps:
            break
        total = ctx.add(total, delta) if sign > 0 else ctx.subtract(total, delta)
        sign = -sign
    total = ctx.multiply(total, Decimal(2 ** halvings))
    if invert:
        total = ctx.subtract(ctx.divide(pi_decimal(prec + 20), Decimal(2)), total)
    return make_context(prec).plus(total)


def sin_cos_decimal(x, prec):
    """(sin x, cos x) by Taylor series, for |x| <= 4 (used only by cross-checks)."""
    ctx = make_context(prec + 20)
    x = ctx.plus(x)
    assert abs(x) <= 4
    x2 = ctx.multiply(x, x)
    eps = Decimal(10) ** (-(prec + 18))
    s, term, k = x, x, 1
    while True:
        term = ctx.divide(ctx.multiply(-term, x2), Decimal((k + 1) * (k + 2)))
        k += 2
        if abs(term) < eps:
            break
        s = ctx.add(s, term)
    c, term, k = Decimal(1), Decimal(1), 0
    while True:
        term = ctx.divide(ctx.multiply(-term, x2), Decimal((k + 1) * (k + 2)))
        k += 2
        if abs(term) < eps:
            break
        c = ctx.add(c, term)
    out = make_context(prec)
    return out.plus(s), out.plus(c)


# --------------------------------------------------------------------------
# Exact a + b*pi
# --------------------------------------------------------------------------
class QPi:
    __slots__ = ("a", "b")

    def __init__(self, a=0, b=0):
        self.a = Fraction(a)
        self.b = Fraction(b)

    @staticmethod
    def _c(o):
        return o if isinstance(o, QPi) else QPi(o, 0)

    def __add__(self, o):
        o = QPi._c(o)
        return QPi(self.a + o.a, self.b + o.b)

    __radd__ = __add__

    def __sub__(self, o):
        o = QPi._c(o)
        return QPi(self.a - o.a, self.b - o.b)

    def __rsub__(self, o):
        return QPi._c(o) - self

    def __neg__(self):
        return QPi(-self.a, -self.b)

    def __mul__(self, o):
        o = QPi._c(o)
        if self.b != 0 and o.b != 0:
            raise ArithmeticError("pi**2 term: not representable as a + b*pi")
        return QPi(self.a * o.a, self.a * o.b + self.b * o.a)

    __rmul__ = __mul__

    def __truediv__(self, o):
        o = QPi._c(o)
        if o.b != 0:
            raise ArithmeticError("division by a pi-carrying number")
        return QPi(self.a / o.a, self.b / o.a)

    def __eq__(self, o):
        o = QPi._c(o)
        return self.a == o.a and self.b == o.b

    def __hash__(self):
        return hash((self.a, self.b))

    def is_zero(self):
        return self.a == 0 and self.b == 0

    def to_decimal(self, prec):
        ctx = make_context(prec + 10)
        v = ctx.add(to_decimal(self.a, ctx), ctx.multiply(to_decimal(self.b, ctx), pi_decimal(prec + 10)))
        return make_context(prec).plus(v)

    def exact_str(self):
        parts = []
        if self.a != 0:
            parts.append(str(self.a))
        if self.b != 0:
            parts.append(f"{self.b}*pi")
        if not parts:
            return "0"
        return " + ".join(parts).replace("+ -", "- ")


class QPiField:
    name = "exact"

    def q(self, x):
        return QPi(Fraction(x), 0)

    def pi(self):
        return QPi(0, 1)

    def sqrt(self, x):
        raise ArithmeticError("sqrt not available in the exact field")

    def atan(self, x):
        raise ArithmeticError("atan not available in the exact field")

    def is_zero(self, x):
        return QPi._c(x).is_zero()

    def sign(self, x):
        raise ArithmeticError("sign of a + b*pi not needed")


class PointField:
    name = "point"

    def q(self, x):
        return to_decimal(x)

    def pi(self):
        return pi_decimal(decimal.getcontext().prec)

    def sqrt(self, x):
        return x.sqrt()

    def atan(self, x):
        return atan_decimal(x, decimal.getcontext().prec)

    def is_zero(self, x):
        return x == 0

    def sign(self, x):
        if x > 0:
            return 1
        if x < 0:
            return -1
        return 0


# --------------------------------------------------------------------------
# Outward-rounded intervals
# --------------------------------------------------------------------------
class IntervalField:
    name = "interval"

    def __init__(self, prec):
        self.prec = prec
        self.D = make_context(prec, ROUND_FLOOR)
        self.U = make_context(prec, ROUND_CEILING)
        self._pi = None

    def q(self, x):
        if isinstance(x, Iv):
            return x
        if isinstance(x, Decimal):
            return Iv(self, self.D.plus(x), self.U.plus(x))
        x = Fraction(x)
        n, d = Decimal(x.numerator), Decimal(x.denominator)
        return Iv(self, self.D.divide(n, d), self.U.divide(n, d))

    def _enclose(self, v):
        """Enclose a value v known to within a few units at prec + 15 digits."""
        return Iv(self, self.D.next_minus(self.D.plus(v)), self.U.next_plus(self.U.plus(v)))

    def pi(self):
        if self._pi is None:
            self._pi = self._enclose(pi_decimal(self.prec + 15))
        return self._pi

    def sqrt(self, x):
        assert x.lo >= 0, "sqrt of an interval reaching below zero"
        hi_ctx = make_context(self.prec + 15)
        lo = self.D.next_minus(self.D.plus(hi_ctx.sqrt(x.lo))) if x.lo > 0 else Decimal(0)
        hi = self.U.next_plus(self.U.plus(hi_ctx.sqrt(x.hi)))
        return Iv(self, lo, hi)

    def atan(self, x):
        p = self.prec + 15
        lo = self.D.next_minus(self.D.plus(atan_decimal(x.lo, p)))
        hi = self.U.next_plus(self.U.plus(atan_decimal(x.hi, p)))
        return Iv(self, lo, hi)

    def is_zero(self, x):
        return x.lo == 0 and x.hi == 0

    def sign(self, x):
        if x.lo > 0:
            return 1
        if x.hi < 0:
            return -1
        if x.lo == 0 and x.hi == 0:
            return 0
        return None


class Iv:
    __slots__ = ("f", "lo", "hi")

    def __init__(self, f, lo, hi):
        self.f = f
        self.lo = lo
        self.hi = hi

    def _c(self, o):
        if isinstance(o, Iv):
            return o
        return self.f.q(o)

    def __add__(self, o):
        o = self._c(o)
        return Iv(self.f, self.f.D.add(self.lo, o.lo), self.f.U.add(self.hi, o.hi))

    __radd__ = __add__

    def __sub__(self, o):
        o = self._c(o)
        return Iv(self.f, self.f.D.subtract(self.lo, o.hi), self.f.U.subtract(self.hi, o.lo))

    def __rsub__(self, o):
        return self._c(o) - self

    def __neg__(self):
        return Iv(self.f, -self.hi, -self.lo)

    def __mul__(self, o):
        o = self._c(o)
        D, U = self.f.D, self.f.U
        a, b, c, d = self.lo, self.hi, o.lo, o.hi
        if a >= 0 and c >= 0:
            return Iv(self.f, D.multiply(a, c), U.multiply(b, d))
        if b <= 0 and d <= 0:
            return Iv(self.f, D.multiply(b, d), U.multiply(a, c))
        if a >= 0 and d <= 0:
            return Iv(self.f, D.multiply(b, c), U.multiply(a, d))
        if b <= 0 and c >= 0:
            return Iv(self.f, D.multiply(a, d), U.multiply(b, c))
        lo = min(D.multiply(a, c), D.multiply(a, d), D.multiply(b, c), D.multiply(b, d))
        hi = max(U.multiply(a, c), U.multiply(a, d), U.multiply(b, c), U.multiply(b, d))
        return Iv(self.f, lo, hi)

    __rmul__ = __mul__

    def __truediv__(self, o):
        o = self._c(o)
        if o.lo <= 0 <= o.hi:
            raise ZeroDivisionError("interval divisor contains zero")
        D, U = self.f.D, self.f.U
        a, b, c, d = self.lo, self.hi, o.lo, o.hi
        lo = min(D.divide(a, c), D.divide(a, d), D.divide(b, c), D.divide(b, d))
        hi = max(U.divide(a, c), U.divide(a, d), U.divide(b, c), U.divide(b, d))
        return Iv(self.f, lo, hi)

    def __rtruediv__(self, o):
        return self._c(o) / self

    def width_rel(self):
        m = max(abs(self.lo), abs(self.hi))
        return (self.hi - self.lo) / m if m else Decimal(0)


# --------------------------------------------------------------------------
# Formatting
# --------------------------------------------------------------------------
def fmt(x, digits):
    """Decimal -> string with `digits` significant digits (scientific)."""
    x = Decimal(x)
    if x == 0:
        return "0"
    return f"{x:.{digits - 1}e}"


def nearest_double(x):
    """Nearest binary64 to a Decimal (Python's float() of a Decimal is correctly rounded)."""
    return float(Decimal(x))


def exact_double(s):
    """Fraction of a decimal input string, asserting it is a binary64 number."""
    fr = Fraction(s)
    assert Fraction(float(fr)) == fr, f"input {s} is not exactly representable in binary64"
    return fr
