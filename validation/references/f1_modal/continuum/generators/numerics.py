"""Shared numerics for the F1 continuum references: standard library only.

Exact rationals (`fractions`) carry binary64 inputs and every matrix whose
inertia is published. Transcendental work runs in `decimal` at WORK_PREC
digits: pi by Machin's formula, sin/cos by Taylor series after reduction
modulo 2*pi, exp/sqrt/ln from `decimal` itself (correctly rounded). No libm
result enters a frozen value; binary64 `math` is used only to scan for root
brackets, and every bracket is re-confirmed in `decimal`.
"""

import json
from decimal import Decimal, getcontext, localcontext
from fractions import Fraction

WORK_PREC = 90
getcontext().prec = WORK_PREC
SIG = 40  # significant digits published for continuum quantities


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
    """Scientific decimal string with `sig` significant digits."""
    d = dec(d) if not isinstance(d, Decimal) else d
    if d == 0:
        return "0"
    return format(d, f".{sig - 1}e")


def inp(x):
    """A binary64 input: shortest repr, hex and exact rational."""
    x = float(x)
    return {"binary64": repr(x), "hex": x.hex(), "exact": rat_str(Fraction(x))}


def rat_str(f):
    f = Fraction(f)
    return str(f.numerator) if f.denominator == 1 else f"{f.numerator}/{f.denominator}"


def rat(f, sig=SIG):
    """An exact rational quantity with its decimal value."""
    return {"exact": rat_str(f), "value": fmt(dec(Fraction(f)), sig)}


def val(d, sig=SIG):
    """A real quantity: decimal string and the nearest binary64."""
    d = dec(d) if not isinstance(d, Decimal) else d
    return {"value": fmt(d, sig), "binary64": repr(float(d))}


def emit_json(doc):
    text = json.dumps(doc, indent=1, ensure_ascii=False) + "\n"
    assert json.loads(text) == doc
    return text.encode("utf-8")


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
    """x - 2*pi*k in [-pi, pi], carried with guard digits."""
    with localcontext() as c:
        c.prec += 20 + max(0, x.adjusted())
        tp = 2 * pi()
        k = (x / tp).to_integral_value()
        return x - k * tp


def _taylor(r, first, start_k):
    with localcontext() as c:
        c.prec += 10
        eps = Decimal(10) ** -(c.prec + 2)
        r2 = r * r
        term = first
        s = first
        k = start_k
        while abs(term) > eps:
            term = -term * r2 / ((k + 1) * (k + 2))
            s += term
            k += 2
    return +s


def dsin(x):
    x = dec(x) if not isinstance(x, Decimal) else x
    with localcontext() as c:
        c.prec += 10
        r = _reduce(x)
        v = _taylor(r, r, 1)
    return +v


def dcos(x):
    x = dec(x) if not isinstance(x, Decimal) else x
    with localcontext() as c:
        c.prec += 10
        r = _reduce(x)
        v = _taylor(r, Decimal(1), 0)
    return +v


def dcosh(x):
    e = x.exp()
    return (e + 1 / e) / 2


def dsinh(x):
    e = x.exp()
    return (e - 1 / e) / 2


def dlog2(x):
    return x.ln() / Decimal(2).ln()


# --------------------------------------------------------------------------
# Root bracketing and bisection


def bisect(f, lo, hi, rel_tol=None):
    """Bisection on a confirmed sign change of f over [lo, hi] (decimal)."""
    if rel_tol is None:
        rel_tol = Decimal(10) ** -(getcontext().prec - 30)
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
    """First `count` sign changes of a (cheap) scan function on a uniform grid."""
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
# Symmetric matrices: exact inertia, unpivoted LDL^T counts, eigenvalues


def shifted(K, M, s):
    n = len(K)
    return [[K[i][j] - s * M[i][j] for j in range(n)] for i in range(n)]


def inertia_exact(A):
    """(negative, zero, positive) of a symmetric rational matrix.

    Symmetric elimination with a nonzero 1x1 diagonal pivot where one exists,
    otherwise a 2x2 pivot [[0, a], [a, 0]] (one negative, one positive
    eigenvalue). Every step is a congruence, so by Sylvester's law of inertia
    the counts are exact for any symmetric rational matrix.
    """
    A = [[Fraction(v) for v in row] for row in A]
    idx = list(range(len(A)))
    neg = zero = pos = 0
    while idx:
        p = next((i for i in idx if A[i][i] != 0), None)
        if p is not None:
            d = A[p][p]
            neg += d < 0
            pos += d > 0
            rest = [i for i in idx if i != p]
            for i in rest:
                if A[i][p] != 0:
                    l = A[i][p] / d
                    for j in rest:
                        A[i][j] -= l * A[p][j]
            idx = rest
            continue
        pair = next(((i, j) for i in idx for j in idx if i < j and A[i][j] != 0), None)
        if pair is None:
            zero += len(idx)
            break
        i0, j0 = pair
        a = A[i0][j0]
        neg += 1
        pos += 1
        rest = [i for i in idx if i not in pair]
        for i in rest:
            for j in rest:
                A[i][j] -= (A[i][i0] * A[j0][j] + A[i][j0] * A[i0][j]) / a
        idx = rest
    return neg, zero, pos


def count_exact(K, M, s):
    """Exact number of eigenvalues of (K, M) below s, and the number equal to s.

    Unpivoted banded LDL^T on rationals; on an exact zero pivot (a singular
    leading minor), falls back to the pivoted inertia above.
    """
    s = Fraction(s)
    A = shifted(K, M, s)
    n = len(A)
    bw = max((j - i for i in range(n) for j in range(i, n) if A[i][j] != 0), default=0)
    W = [row[:] for row in A]
    neg = 0
    for k in range(n):
        d = W[k][k]
        if d == 0:
            neg_, zero_, _ = inertia_exact(A)
            return neg_, zero_
        neg += d < 0
        hi = min(n, k + bw + 1)
        for i in range(k + 1, hi):
            if W[i][k] != 0:
                l = W[i][k] / d
                for j in range(k + 1, hi):
                    W[i][j] -= l * W[k][j]
    return neg, 0


def band_of(A, bw):
    """Upper band rows: B[i][d] = A[i][i + d], d = 0..bw (zero past the end)."""
    n = len(A)
    return [[A[i][i + d] if i + d < n else 0 for d in range(bw + 1)] for i in range(n)]


def count_decimal(Kb, Mb, s):
    """Unpivoted banded LDL^T negative-pivot count in decimal (bisection only)."""
    n = len(Kb)
    bw = len(Kb[0]) - 1 if n else 0
    W = [[k - s * m for k, m in zip(kr, mr)] for kr, mr in zip(Kb, Mb)]
    tiny = Decimal(10) ** -(getcontext().prec + 10)
    neg = 0
    for k in range(n):
        wk = W[k]
        d = wk[0]
        if d == 0:
            d = tiny
        neg += d < 0
        for i in range(1, bw + 1):
            if k + i >= n:
                break
            u = wk[i]
            if u == 0:
                continue
            l = u / d
            wi = W[k + i]
            for j in range(i, bw + 1):
                wi[j - i] -= l * wk[j]
    return neg


def bandwidth(A):
    n = len(A)
    return max((j - i for i in range(n) for j in range(i, n) if A[i][j] != 0), default=0)


def pencil_eigenvalues(Kf, Mf, count, rel_tol_digits, upper_hint=None):
    """Lowest `count` eigenvalues of a rational SPD-K pencil by decimal bisection.

    Returns decimals; the caller verifies each with exact counts.
    """
    bw = max(bandwidth(Kf), bandwidth(Mf))
    Kb = [[dec(v) for v in row] for row in band_of(Kf, bw)]
    Mb = [[dec(v) for v in row] for row in band_of(Mf, bw)]
    tol = Decimal(10) ** -rel_tol_digits
    hi = dec(upper_hint) if upper_hint else Decimal(1)
    while count_decimal(Kb, Mb, hi) < count:
        hi *= 2
    out = []
    for k in range(1, count + 1):
        lo = out[-1] if out else Decimal(0)
        h = hi
        while h - lo > tol * h:
            mid = (lo + h) / 2
            if count_decimal(Kb, Mb, mid) >= k:
                h = mid
            else:
                lo = mid
        out.append((lo + h) / 2)
    return out


def verify_simple_eigenvalues(Kf, Mf, lams, offset_digits):
    """Exact counts at decimal shifts just below and above each eigenvalue.

    Returns a list of {shift_below, count_below, shift_above, count_above}
    and asserts the counts are k-1 and k (simple eigenvalues).
    """
    rows = []
    off = Decimal(10) ** -offset_digits
    for k, lam in enumerate(lams, start=1):
        s_lo = fmt(lam * (1 - off), offset_digits + 8)
        s_hi = fmt(lam * (1 + off), offset_digits + 8)
        cl, zl = count_exact(Kf, Mf, Fraction(Decimal(s_lo)))
        ch, zh = count_exact(Kf, Mf, Fraction(Decimal(s_hi)))
        assert zl == 0 and zh == 0
        assert cl == k - 1 and ch == k, (k, cl, ch)
        rows.append({"mode": k, "shift_below": s_lo, "count_below": cl,
                     "shift_above": s_hi, "count_above": ch})
    return rows
