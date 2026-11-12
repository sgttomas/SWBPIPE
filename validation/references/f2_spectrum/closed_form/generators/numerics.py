"""Shared numerics for the F2 closed-form spectrum references: standard library only.

Binary64 inputs are carried as exact rationals (`fractions`). Transcendental
work runs in `decimal` at WORK_PREC digits: pi by Machin's formula, sin and
cos by Taylor series after reduction modulo 2*pi, exp, ln and sqrt from
`decimal` itself (correctly rounded). No libm value enters a published
number; binary64 `math` is used only to scan for root brackets, and every
bracket is re-confirmed in `decimal` before bisection.
"""

import json
from decimal import Decimal, getcontext, localcontext
from fractions import Fraction

WORK_PREC = 90
getcontext().prec = WORK_PREC
SIG = 40  # significant digits of published reals


# --------------------------------------------------------------------------
# Conversions and JSON encodings


def dec(x):
    """Decimal at the current precision from Fraction, int, float, str or Decimal."""
    if isinstance(x, Fraction):
        return Decimal(x.numerator) / Decimal(x.denominator)
    if isinstance(x, float):
        return dec(Fraction(x))
    return +Decimal(x)


def fmt(d, sig=SIG):
    d = d if isinstance(d, Decimal) else dec(d)
    if d == 0:
        return "0"
    with localcontext() as c:
        c.prec = max(c.prec, sig + 5)
        return format(d, f".{sig - 1}e")


def rat_str(f):
    f = Fraction(f)
    return str(f.numerator) if f.denominator == 1 else f"{f.numerator}/{f.denominator}"


def inp(x):
    """A binary64 input: shortest round-trip decimal, hex and exact rational."""
    x = float(x)
    return {"binary64": repr(x), "hex": x.hex(), "exact": rat_str(Fraction(x))}


def val(d, sig=SIG):
    """A real: decimal string with `sig` significant digits and the nearest binary64."""
    d = d if isinstance(d, Decimal) else dec(d)
    if d == 0:
        d = Decimal(0)
    return {"value": fmt(d, sig), "binary64": repr(float(d))}


def rat(f, sig=SIG):
    """An exact rational with its decimal value and nearest binary64."""
    f = Fraction(f)
    return {"exact": rat_str(f), "value": fmt(dec(f), sig), "binary64": repr(float(f))}


def num(x, sig=SIG):
    """Fraction -> rat (exact); Decimal -> val."""
    return rat(x, sig) if isinstance(x, Fraction) else val(x, sig)


def sci(d, sig=3):
    """Short diagnostic number (residuals, differences)."""
    d = d if isinstance(d, Decimal) else dec(d)
    return "0" if d == 0 else format(d, f".{sig - 1}e")


def _enc(o, ind):
    """JSON with one-space indentation; a dict or list whose members are all scalars is kept on one line."""
    pad = " " * ind
    if isinstance(o, dict):
        if not o:
            return "{}"
        if all(not isinstance(v, (dict, list)) for v in o.values()):
            return "{" + ", ".join(f"{json.dumps(k, ensure_ascii=False)}: {json.dumps(v, ensure_ascii=False)}"
                                   for k, v in o.items()) + "}"
        items = [f"{pad} {json.dumps(k, ensure_ascii=False)}: {_enc(v, ind + 1)}" for k, v in o.items()]
        return "{\n" + ",\n".join(items) + "\n" + pad + "}"
    if isinstance(o, list):
        if not o:
            return "[]"
        if all(not isinstance(v, (dict, list)) for v in o):
            return "[" + ", ".join(json.dumps(v, ensure_ascii=False) for v in o) + "]"
        items = [f"{pad} {_enc(v, ind + 1)}" for v in o]
        return "[\n" + ",\n".join(items) + "\n" + pad + "]"
    return json.dumps(o, ensure_ascii=False)


def emit_json(doc):
    text = _enc(doc, 0) + "\n"
    assert json.loads(text) == doc
    return text.encode("utf-8")


def rel_diff(a, b):
    a, b = dec(a) if not isinstance(a, Decimal) else a, dec(b) if not isinstance(b, Decimal) else b
    if a == b:
        return Decimal(0)
    s = max(abs(a), abs(b))
    return abs(a - b) / s


# --------------------------------------------------------------------------
# Transcendentals in decimal

_PI = {}


def pi():
    p = getcontext().prec
    if p not in _PI:
        with localcontext() as c:
            c.prec = p + 15
            eps = Decimal(10) ** -(c.prec + 2)

            def atan_inv(n):
                n = Decimal(n)
                n2 = n * n
                power = 1 / n
                s = power
                k = 1
                while True:
                    power /= n2
                    t = power / (2 * k + 1)
                    if t < eps:
                        break
                    s += -t if k % 2 else t
                    k += 1
                return s

            v = 16 * atan_inv(5) - 4 * atan_inv(239)
        _PI[p] = +v
    return _PI[p]


def _reduce(x):
    with localcontext() as c:
        c.prec += 20 + max(0, x.adjusted())
        tp = 2 * pi()
        k = (x / tp).to_integral_value()
        return x - k * tp


def _taylor(r, first):
    with localcontext() as c:
        c.prec += 10
        eps = Decimal(10) ** -(c.prec + 2)
        r2 = r * r
        term = first
        s = first
        k = 0 if first == 1 else 1
        while abs(term) > eps:
            term = -term * r2 / ((k + 1) * (k + 2))
            s += term
            k += 2
    return +s


def dsin(x):
    x = x if isinstance(x, Decimal) else dec(x)
    with localcontext() as c:
        c.prec += 10
        r = _reduce(x)
        v = _taylor(r, r)
    return +v


def dcos(x):
    x = x if isinstance(x, Decimal) else dec(x)
    with localcontext() as c:
        c.prec += 10
        r = _reduce(x)
        v = _taylor(r, Decimal(1))
    return +v


def dexp(x):
    x = x if isinstance(x, Decimal) else dec(x)
    return x.exp()


def dln(x):
    x = x if isinstance(x, Decimal) else dec(x)
    return x.ln()


def dsqrt(x):
    x = x if isinstance(x, Decimal) else dec(x)
    return x.sqrt()


def dcosh(x):
    e = dexp(x)
    return (e + 1 / e) / 2


def dsinh(x):
    e = dexp(x)
    return (e - 1 / e) / 2


# --------------------------------------------------------------------------
# Roots


def bisect(f, lo, hi, rel_tol=None):
    """Bisection on a confirmed sign change of f over [lo, hi] (decimal)."""
    if rel_tol is None:
        rel_tol = Decimal(10) ** -(getcontext().prec - 25)
    flo, fhi = f(lo), f(hi)
    if flo == 0:
        return lo
    if fhi == 0:
        return hi
    if (flo < 0) == (fhi < 0):
        raise ValueError(f"no sign change on [{lo}, {hi}]")
    while hi - lo > rel_tol * abs(hi):
        mid = (lo + hi) / 2
        fm = f(mid)
        if fm == 0:
            return mid
        if (fm < 0) == (flo < 0):
            lo, flo = mid, fm
        else:
            hi = mid
    return (lo + hi) / 2


def scan_brackets(f_scan, start, stop, step, count):
    """First `count` sign changes of a binary64 scan function on a uniform grid."""
    out = []
    x0 = start
    f0 = f_scan(x0)
    k = 1
    while len(out) < count:
        x1 = start + k * step
        if x1 > stop:
            raise ValueError("scan range exhausted")
        f1 = f_scan(x1)
        if f0 == 0 or (f0 < 0) != (f1 < 0):
            out.append((x0, x1))
        x0, f0 = x1, f1
        k += 1
    return out


# --------------------------------------------------------------------------
# Gauss-Legendre quadrature in decimal (used only for cross-checks)

_GL = {}


def gauss_legendre(n):
    key = (n, getcontext().prec)
    if key in _GL:
        return _GL[key]
    import math
    nodes, weights = [], []
    eps = Decimal(10) ** -(getcontext().prec - 5)
    for i in range(1, n + 1):
        x = dec(math.cos(math.pi * (i - 0.25) / (n + 0.5)))  # starting guess only
        for _ in range(100):
            p0, p1 = Decimal(1), x
            for k in range(2, n + 1):
                p0, p1 = p1, ((2 * k - 1) * x * p1 - (k - 1) * p0) / k
            dp = n * (x * p1 - p0) / (x * x - 1)
            dx = p1 / dp
            x -= dx
            if abs(dx) < eps:
                break
        p0, p1 = Decimal(1), x
        for k in range(2, n + 1):
            p0, p1 = p1, ((2 * k - 1) * x * p1 - (k - 1) * p0) / k
        dp = n * (x * p1 - p0) / (x * x - 1)
        nodes.append(x)
        weights.append(2 / ((1 - x * x) * dp * dp))
    _GL[key] = (nodes, weights)
    return nodes, weights


def integrate(f, a, b, n_sub=12, n_gl=24):
    a, b = dec(a), dec(b)
    nodes, weights = gauss_legendre(n_gl)
    h = (b - a) / n_sub
    s = Decimal(0)
    for k in range(n_sub):
        lo = a + k * h
        mid = lo + h / 2
        for x, w in zip(nodes, weights):
            s += w * f(mid + h / 2 * x)
    return s * h / 2


# --------------------------------------------------------------------------
# Dense linear algebra in decimal / Fraction (small matrices)


def solve(A, b):
    """Gaussian elimination with partial pivoting; works for Fraction or Decimal."""
    n = len(A)
    M = [list(A[i]) + [b[i]] for i in range(n)]
    for k in range(n):
        p = max(range(k, n), key=lambda i: abs(M[i][k]))
        if M[p][k] == 0:
            raise ZeroDivisionError("singular")
        M[k], M[p] = M[p], M[k]
        for i in range(k + 1, n):
            l = M[i][k] / M[k][k]
            if l:
                for j in range(k, n + 1):
                    M[i][j] -= l * M[k][j]
    x = [None] * n
    for i in range(n - 1, -1, -1):
        s = M[i][n]
        for j in range(i + 1, n):
            s -= M[i][j] * x[j]
        x[i] = s / M[i][i]
    return x


def det(A):
    A = [row[:] for row in A]
    n = len(A)
    d = A[0][0] * 0 + 1
    for k in range(n):
        p = max(range(k, n), key=lambda i: abs(A[i][k]))
        if A[p][k] == 0:
            return A[0][0] * 0
        if p != k:
            A[k], A[p] = A[p], A[k]
            d = -d
        d = d * A[k][k]
        for i in range(k + 1, n):
            l = A[i][k] / A[k][k]
            for j in range(k, n):
                A[i][j] -= l * A[k][j]
    return d


def matvec(A, x):
    return [sum((a * b for a, b in zip(row, x)), x[0] * 0) for row in A]


def dot(x, y):
    return sum((a * b for a, b in zip(x, y)), x[0] * 0)
