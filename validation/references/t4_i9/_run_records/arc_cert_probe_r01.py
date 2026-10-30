"""T4-I9 revision 01: emulation probe of T4-U1b's arc load-vector certificate.

Standard library only (fractions, decimal, math). Run with `python -I`.
It is an emulation, not a product run: no Rust code is built or executed.

What changed from round 00 (DESIGN_R01.md section 9):
- A-2: every radius is computed in binary64 exactly as DESIGN_R01.md section 3.2
  specifies it: each operation rounds to nearest (IEEE binary64, which Python's
  float is) and then moves one step in the required direction, unconditionally
  (`up`/`dn`); lower bounds for denominators; |m|_up and |m|_dn from the exact
  binary64 split of the 128-bit midpoint (split_binary64's first term);
  constants 2^-128, next_up(1) and 22*2^-128. Round 00 rounded exact rational
  radii once. Midpoints are the Wide<2> p = 128 model (round to nearest, ties
  to even), as before.
- The arctangent midpoint is an emulation of K3a's `atan_positive` (reduction
  t <- t/(1 + sqrt(1 + t^2)) while t >= 1/20, the series with K3a's stop rule,
  summed smallest first), every operation rounded to 128 bits.
- A-6: L5 uses the sharpened derivative bound 1/(1 + (m_t - r_t)^2).
- c_h = sqrt(4R^2 - d.d)/(2R); 1 - cos(phi) = 2 s^2 and cos(2 phi) - 1 = -2 S^2
  (the stable forms; equal to round 00's expressions in exact arithmetic).
- New sections: small angles (radius share of the threshold), large k
  (T15d's natural catch, A-5), T15's body over k (T15c's window, A-4, with an
  emulation of FK `solve_dense`), near pi (A-6), and the directed-rounding
  primitives and lemma-level mutants (A-2, MU6).

Function names that RV129's tools import (formula, BallCtx, DecCtx, invert,
objective_binary64, section, Ball, rnd, U, up, b_add, b_mul, b_div, b_sqrt,
b_atan, ATAN_REL, atan_decimal, q_to_dec, dec_to_q, gauss_jordan) are kept.
"""
from decimal import Decimal, getcontext
from fractions import Fraction as Q
import math
import sys

P = 128
U = Q(1, 2**P)              # unit roundoff of the 128-bit midpoints (exact)
U64 = 2.0 ** -128           # the same, as a binary64 constant (exact)
ONE_UP = math.nextafter(1.0, math.inf)   # >= 1 + 2^-128, for (1 + u)
K_ATAN = 22.0 * 2.0 ** -128              # >= 21.54u/(1 - 21.54u), exact binary64
ATAN_REL = Q(2154, 100)     # K3a atan_positive: (1.54 + 4k) u <= 21.54 u (checked below)
TINY = math.ldexp(1.0, -1074)
CRITERION = math.nextafter(1e-9, 0.0)   # RD(1e-9), as formation_guard.rs


class CertificateFailure(ArithmeticError):
    """A failed precondition: the certificate returns Err, the terms stay
    CannotBound. `klass` is the DESIGN_R01 section 3.5 class number."""

    def __init__(self, klass, detail):
        super().__init__("class %d: %s" % (klass, detail))
        self.klass = klass

# ------------------------------------------------------------ 128-bit model


def rnd(x):
    """x rounded to P significant bits, to nearest, ties to even."""
    if x == 0:
        return Q(0)
    sign = -1 if x < 0 else 1
    a = abs(x)
    n, d = a.numerator, a.denominator
    e = n.bit_length() - d.bit_length()
    if Q(n, d) < Q(2) ** e:
        e -= 1
    shift = P - 1 - e
    if shift >= 0:
        num, den = n << shift, d
    else:
        num, den = n, d << (-shift)
    m, r = divmod(num, den)
    twice = 2 * r
    if twice > den or (twice == den and (m & 1)):
        m += 1
    return sign * Q(m) / (Q(2) ** shift)


def exponent(x):
    """Exponent of the leading bit of x != 0 (2^e <= |x| < 2^(e+1))."""
    a = abs(Q(x))
    n, d = a.numerator, a.denominator
    e = n.bit_length() - d.bit_length()
    if Q(n, d) < Q(2) ** e:
        e -= 1
    return e


def isqrt_rounded(x):
    """sqrt(x) for x > 0 rounded to P bits, to nearest (exact decision)."""
    n, d = x.numerator, x.denominator
    e = (n.bit_length() - d.bit_length()) // 2
    k = P + 8 - e
    if k >= 0:
        num, den = n << (2 * k), d
    else:
        num, den = n, d << (-2 * k)
    target = Q(num, den)
    s = math.isqrt(num // den)
    sticky = Q(s * s) != target
    bits = s.bit_length()
    drop = bits - P
    if drop <= 0:
        return Q(s, 1) / (Q(2) ** k) if not sticky else rnd(Q(2 * s + 1, 2) / (Q(2) ** k))
    m, rem = s >> drop, s & ((1 << drop) - 1)
    half = 1 << (drop - 1)
    if rem > half or (rem == half and (sticky or (m & 1))):
        m += 1
    return Q(m) * (Q(2) ** drop) / (Q(2) ** k)

# ---------------------------------------------- split_binary64 (wide.rs:409)


def split_binary64(m):
    """(terms, truncated) exactly as `Wide::split_binary64`: the 128-bit
    significand cut into bits 127..75, 74..22 and 21..0, low bits below
    2^-1074 dropped (truncation toward zero). Raises class 9 on overflow."""
    if m == 0:
        return [], False
    e = exponent(m)
    if e > 1023:
        raise CertificateFailure(9, "SplitOverflow")
    neg = m < 0
    sig = abs(m) * (Q(2) ** (127 - e))
    assert sig.denominator == 1 and sig.numerator < 2**128, "midpoint is not a 128-bit value"
    sig = sig.numerator
    terms, truncated = [], False
    for high, width in ((127, 53), (74, 53), (21, 22)):
        shift = high + 1 - width
        chunk = (sig >> shift) & ((1 << width) - 1)
        lsb = e - 127 + shift
        if chunk == 0:
            continue
        if lsb < -1074:
            drop = -1074 - lsb
            if drop >= 64:
                truncated = True
                continue
            if chunk & ((1 << drop) - 1):
                truncated = True
            chunk >>= drop
            lsb = -1074
            if chunk == 0:
                continue
        v = math.ldexp(float(chunk), lsb)
        assert Q(v) == Q(chunk) * Q(2) ** lsb
        terms.append(-v if neg else v)
    return terms, truncated


def abs_dn(m):
    """|t0| <= |m| (split_binary64 truncates toward zero)."""
    terms, _ = split_binary64(m)
    return abs(terms[0]) if terms else 0.0


def abs_up(m):
    """next_up(|t0|) >= |m| for m != 0; 0 for m = 0."""
    if m == 0:
        return 0.0
    terms, _ = split_binary64(m)
    return math.nextafter(abs(terms[0]) if terms else 0.0, math.inf)

# --------------------------- binary64 directed primitives (DESIGN_R01 3.2)


def up(x):
    """The smallest binary64 value >= x (x >= 0): RV129's tools use it."""
    x = Q(x)
    f = float(x)
    if Q(f) < x:
        f = math.nextafter(f, math.inf)
    return f


def nu(x):
    return math.nextafter(x, math.inf)


def nd(x):
    return math.nextafter(x, -math.inf)


def add_up(a, b):
    """Upper bound of a + b (a, b >= 0): nearest, then one step up."""
    if a == 0.0:
        return b
    if b == 0.0:
        return a
    return nu(a + b)


def add_dn(a, b):
    if a == 0.0:
        return b
    if b == 0.0:
        return a
    return nd(a + b)


def sub_dn(a, b):
    """Lower bound of a - b (a, b >= 0); may be <= 0 (the caller fails)."""
    if b == 0.0:
        return a
    return nd(a - b)


def mul_up(a, b):
    if a == 0.0 or b == 0.0:
        return 0.0
    return nu(a * b)


def mul_dn(a, b):
    if a == 0.0 or b == 0.0:
        return 0.0
    return max(nd(a * b), 0.0)


def div_up(a, b):
    """Upper bound of a / b (a >= 0, b > 0)."""
    if a == 0.0:
        return 0.0
    return nu(a / b)


def finite(r, site):
    if not math.isfinite(r):
        raise CertificateFailure(8, "radius not finite at %s" % site)
    return r

# --------------------------------------------- K3a atan_positive at p = 128


def atan_positive_k3a(t):
    """Emulation of `WideArith::atan_positive` (wide.rs:940-1000) at p = 128."""
    if t <= 0:
        raise CertificateFailure(7, "AngleDomain")
    one = Q(1)
    k = 0
    while t >= Q(1, 20):
        if k == 5:
            raise CertificateFailure(7, "ArctangentLimit")
        t2 = rnd(t * t)
        b = rnd(one + t2)
        r = isqrt_rounded(b)
        d = rnd(one + r)
        t = rnd(t / d)
        k += 1
    t2 = rnd(t * t)
    threshold = exponent(t) - P - 1
    terms = [t]
    power = t
    for n in range(1, 16):
        power = -rnd(power * t2)
        term = rnd(power / (2 * n + 1))
        if term == 0 or exponent(term) < threshold:
            break
        if n == 15:
            raise CertificateFailure(7, "ArctangentLimit")
        terms.append(term)
    tail = Q(0)
    for term in reversed(terms[1:]):
        tail = rnd(tail + term)
    return rnd(t + tail) * (Q(2) ** k)

# -------------------------------------------------------------------- balls


class Ball:
    __slots__ = ("m", "r")

    def __init__(self, m, r=0.0):
        self.m = Q(m)
        self.r = float(r)

    @staticmethod
    def lift(x):
        if not math.isfinite(x):
            raise CertificateFailure(7, "NonFinite lift")
        return Ball(Q(x), 0.0)

    def __neg__(self):
        return Ball(-self.m, self.r)


def u_term(m):
    """u |m|, rounded up."""
    return mul_up(U64, abs_up(m))


def b_add(a, b):                                    # L1
    m = rnd(a.m + b.m)
    r = add_up(add_up(a.r, b.r), u_term(m))
    return Ball(m, finite(r, "L1"))


def b_sub(a, b):
    return b_add(a, -b)


def b_mul(a, b):                                    # L2
    m = rnd(a.m * b.m)
    r = add_up(mul_up(abs_up(a.m), b.r), mul_up(abs_up(b.m), a.r))
    r = add_up(r, mul_up(a.r, b.r))
    r = add_up(r, u_term(m))
    return Ball(m, finite(r, "L2"))


def b_div(a, b):                                    # L3
    den = sub_dn(abs_dn(b.m), b.r) if b.m != 0 else 0.0
    if not den > 0.0:
        raise CertificateFailure(2, "divisor ball reaches zero")
    m = rnd(a.m / b.m)
    q_up = mul_up(abs_up(m), ONE_UP)              # >= |m_a/m_b|
    num = add_up(a.r, mul_up(q_up, b.r))
    r = add_up(div_up(num, den), u_term(m))
    return Ball(m, finite(r, "L3"))


def b_sqrt(a):                                      # L4
    if not (a.m > 0 and sub_dn(abs_dn(a.m), a.r) > 0.0):
        raise CertificateFailure(3, "sqrt argument ball reaches zero")
    m = isqrt_rounded(a.m)
    den = abs_dn(m)
    if not den > 0.0:
        raise CertificateFailure(3, "sqrt result below the binary64 range")
    r = add_up(div_up(mul_up(a.r, ONE_UP), den), u_term(m))
    return Ball(m, finite(r, "L4"))


def b_atan(t):                                      # L5, sharpened (A-6)
    lo = sub_dn(abs_dn(t.m), t.r) if t.m > 0 else 0.0
    if not lo > 0.0:
        raise CertificateFailure(4, "atan argument ball not positive")
    m = atan_positive_k3a(t.m)
    den = add_dn(1.0, mul_dn(lo, lo))
    r = add_up(div_up(t.r, den), mul_up(K_ATAN, abs_up(m)))
    return Ball(m, finite(r, "L5"))


def b_pow2(a, k):                                   # L6
    return Ball(a.m * (Q(2) ** k), finite(mul_up(a.r, 2.0 ** k), "L6"))


# Legacy names RV129's near-pi tool reads.
def atan_decimal(x):
    """atan(x) for Decimal x > 0, by halving reductions and the series."""
    getcontext().prec = 90
    k = 0
    while x > Decimal("0.05"):
        x = x / (1 + (1 + x * x).sqrt())
        k += 1
    x2 = x * x
    term, total, n = x, x, 1
    eps = Decimal(10) ** -85
    while abs(term) > eps:
        term = -term * x2
        total += term / (2 * n + 1)
        n += 1
    return total * (2 ** k)


def q_to_dec(x):
    getcontext().prec = 90
    return Decimal(x.numerator) / Decimal(x.denominator)


def dec_to_q(x):
    return Q(x)

# --------------------------------------------------- arithmetic "contexts"


class BallCtx:
    name = "ball"
    def lift(self, x): return Ball.lift(x)
    def add(self, a, b): return b_add(a, b)
    def sub(self, a, b): return b_sub(a, b)
    def mul(self, a, b): return b_mul(a, b)
    def div(self, a, b): return b_div(a, b)
    def sqrt(self, a): return b_sqrt(a)
    def neg(self, a): return -a
    def half(self, a): return b_pow2(a, -1)
    def zero(self): return Ball(Q(0))
    def one(self): return Ball(Q(1))

    def half_angle(self, dd, length, R):
        """s_h = L/(2R), c_h = sqrt(4R^2 - d.d)/(2R) (F1)."""
        two_r = b_pow2(self.lift(R), 1)
        s_h = self.div(self.half(length), R if isinstance(R, Ball) else self.lift(R))
        q = self.sub(self.mul(two_r, two_r), dd)
        c_h = self.div(self.sqrt(q), two_r)
        return s_h, c_h

    def trig(self, s_h, c_h):
        """phi, S, C, S2, C2, 1 - C, C2 - 1 (F2), stable forms."""
        one = self.one()
        phi = b_pow2(b_atan(self.div(s_h, c_h)), 1)
        sin = b_pow2(self.mul(s_h, c_h), 1)
        one_c = b_pow2(self.mul(s_h, s_h), 1)          # 1 - cos = 2 s^2
        cos = self.sub(one, one_c)
        sin2 = b_pow2(self.mul(sin, cos), 1)
        c2m1 = -b_pow2(self.mul(sin, sin), 1)          # cos 2phi - 1 = -2 S^2
        cos2 = self.add(one, c2m1)
        return phi, sin, cos, sin2, cos2, one_c, c2m1


class DecCtx:
    name = "decimal"
    def __init__(self):
        getcontext().prec = 70
    def lift(self, x): return Decimal(x)
    def add(self, a, b): return a + b
    def sub(self, a, b): return a - b
    def mul(self, a, b): return a * b
    def div(self, a, b): return a / b
    def sqrt(self, a): return a.sqrt()
    def neg(self, a): return -a
    def half(self, a): return a / 2
    def zero(self): return Decimal(0)
    def one(self): return Decimal(1)

    @staticmethod
    def sin_cos(x):
        getcontext().prec = 80
        s, c = Decimal(0), Decimal(0)
        term = x
        n = 1
        while abs(term) > Decimal(10) ** -78:
            s += term
            term = -term * x * x / ((n + 1) * (n + 2))
            n += 2
        term = Decimal(1)
        n = 0
        while abs(term) > Decimal(10) ** -78:
            c += term
            term = -term * x * x / ((n + 1) * (n + 2))
            n += 2
        getcontext().prec = 70
        return +s, +c

    def half_angle(self, dd, length, R):
        s_h = length / 2 / Decimal(R)
        return s_h, (1 - s_h * s_h).sqrt()        # an independent route

    def trig(self, s_h, c_h):
        getcontext().prec = 90
        phi = 2 * atan_decimal(s_h / c_h)
        getcontext().prec = 70
        phi = +phi
        sin, cos = self.sin_cos(phi)
        sin2, cos2 = self.sin_cos(2 * phi)
        return phi, sin, cos, sin2, cos2, 1 - cos, cos2 - 1

# ------------------------------------------------ the formula (T4-U1/U1b)


def formula(ctx, xi, xj, radius, yref, em, gm, area, inertia, torsion, kin, kout, w):
    """Consistent uniform-load vector (global, [node i; node j]) of the
    objective arc (U1_REFERENCE B1), written once for any arithmetic context.
    For BallCtx the operation order is the specification (DESIGN_R01 3.3)."""
    L = ctx.lift
    d = [ctx.sub(L(xj[k]), L(xi[k])) for k in range(3)]
    dd = ctx.add(ctx.add(ctx.mul(d[0], d[0]), ctx.mul(d[1], d[1])), ctx.mul(d[2], d[2]))
    length = ctx.sqrt(dd)
    R = L(radius)
    s_h, c_h = ctx.half_angle(dd, length, radius)
    phi, s, c, s2, c2, one_c, c2m1 = ctx.trig(s_h, c_h)
    dh = [ctx.div(d[k], length) for k in range(3)]
    y = [L(v) for v in yref]
    proj = ctx.add(ctx.add(ctx.mul(y[0], dh[0]), ctx.mul(y[1], dh[1])), ctx.mul(y[2], dh[2]))
    nr = [ctx.sub(y[k], ctx.mul(proj, dh[k])) for k in range(3)]
    nn = ctx.sqrt(ctx.add(ctx.add(ctx.mul(nr[0], nr[0]), ctx.mul(nr[1], nr[1])), ctx.mul(nr[2], nr[2])))
    nh = [ctx.div(nr[k], nn) for k in range(3)]
    ex = [ctx.add(ctx.neg(ctx.mul(s_h, dh[k])), ctx.mul(c_h, nh[k])) for k in range(3)]

    def cross(a, b):
        return [ctx.sub(ctx.mul(a[1], b[2]), ctx.mul(a[2], b[1])),
                ctx.sub(ctx.mul(a[2], b[0]), ctx.mul(a[0], b[2])),
                ctx.sub(ctx.mul(a[0], b[1]), ctx.mul(a[1], b[0]))]

    def dot(a, b):
        return ctx.add(ctx.add(ctx.mul(a[0], b[0]), ctx.mul(a[1], b[1])), ctx.mul(a[2], b[2]))

    ez = cross(nh, dh)
    ey = cross(ez, ex)
    axes = [ex, ey, ez]
    wl = [dot(axes[a], [L(v) for v in w]) for a in range(3)]
    chord = [dot(axes[a], d) for a in range(3)]          # actual chord, local

    one, zero = ctx.one(), ctx.zero()
    Rs, Rc = ctx.mul(R, s), ctx.mul(R, c)
    z3 = [zero, zero, zero]
    cases = [
        ([ctx.neg(Rs), zero, R], z3, z3, [zero, zero, ctx.neg(one)]),
        ([Rc, ctx.neg(R), zero], z3, z3, [zero, one, zero]),
        (z3, [zero, Rs, ctx.neg(Rc)], [R, ctx.neg(Rc), ctx.neg(Rs)], z3),
        (z3, [zero, one, zero], [zero, zero, ctx.neg(one)], z3),
        (z3, [zero, zero, one], [zero, one, zero], z3),
        ([one, zero, zero], z3, z3, z3),
    ]
    half_phi = ctx.half(phi)
    q_s2 = ctx.half(ctx.half(s2))
    half_ss = ctx.half(ctx.mul(s, s))
    gram = [[phi, s, one_c],
            [s, ctx.add(half_phi, q_s2), half_ss],
            [one_c, half_ss, ctx.sub(half_phi, q_s2)]]
    phi_s = ctx.mul(phi, s)
    phi_c = ctx.mul(phi, c)
    i_tc = ctx.sub(phi_s, one_c)                     # phi S + C - 1
    i_ts = ctx.sub(s, phi_c)
    phi2_4 = ctx.half(ctx.half(ctx.mul(phi, phi)))
    phi_s2_4 = ctx.half(ctx.half(ctx.mul(phi, s2)))
    c2m1_8 = ctx.half(ctx.half(ctx.half(c2m1)))
    i_tcc = ctx.add(ctx.add(phi2_4, phi_s2_4), c2m1_8)
    i_tss = ctx.sub(ctx.sub(phi2_4, phi_s2_4), c2m1_8)
    i_tsc = ctx.sub(ctx.half(ctx.half(ctx.half(s2))), ctx.half(ctx.half(ctx.mul(phi, c2))))
    egram = [[phi, s, one_c, ctx.half(ctx.mul(phi, phi)), i_tc, i_ts],
             [s, ctx.add(half_phi, q_s2), half_ss, i_tc, i_tcc, i_tsc],
             [one_c, half_ss, ctx.sub(half_phi, q_s2), i_ts, i_tsc, i_tss]]
    R2 = ctx.mul(R, R)
    wx, wy, wz = wl
    a_x = [s, ctx.neg(phi), ctx.neg(one), zero, one, zero]
    a_y = [ctx.neg(c), one, ctx.neg(phi), zero, zero, one]
    ld_ip = [ctx.mul(R2, ctx.sub(ctx.mul(wy, a_x[k]), ctx.mul(wx, a_y[k]))) for k in range(6)]
    R2wz = ctx.mul(R2, wz)
    ld_op = [R2wz, ctx.neg(ctx.mul(R2wz, c)), ctx.neg(ctx.mul(R2wz, s)), zero, zero, zero]
    ld_t = [ctx.mul(R2wz, phi), ctx.neg(ctx.mul(R2wz, s)), ctx.mul(R2wz, c), ctx.neg(R2wz), zero, zero]
    Rwy, Rwx = ctx.mul(R, wy), ctx.mul(R, wx)
    ld_ax = [zero, ctx.mul(Rwy, phi), ctx.neg(ctx.mul(Rwx, phi)), zero, ctx.neg(Rwy), Rwx]

    def quad(g, left, right, ncol):
        total = zero
        for r in range(3):
            for k in range(ncol):
                total = ctx.add(total, ctx.mul(ctx.mul(left[r], g[r][k]), right[k]))
        return total

    EI = ctx.mul(L(em), L(inertia))
    GJ = ctx.mul(L(gm), L(torsion))
    EA = ctx.mul(L(em), L(area))
    fin, fout = L(kin), L(kout)

    def energy(qi, qo, qt, qa):
        t = ctx.add(ctx.div(ctx.mul(fin, qi), EI), ctx.div(ctx.mul(fout, qo), EI))
        t = ctx.add(ctx.add(t, ctx.div(qt, GJ)), ctx.div(qa, EA))
        return ctx.mul(R, t)

    F = [[None] * 6 for _ in range(6)]
    for r in range(6):
        for k in range(r, 6):
            v = energy(*(quad(gram, cases[r][j], cases[k][j], 3) for j in range(4)))
            F[r][k] = v
            F[k][r] = v
    delta = [energy(quad(egram, cases[r][0], ld_ip, 6), quad(egram, cases[r][1], ld_op, 6),
                    quad(egram, cases[r][2], ld_t, 6), quad(egram, cases[r][3], ld_ax, 6))
             for r in range(6)]
    K = invert(ctx, F)
    X = [zero] * 6
    for r in range(6):
        for k in range(6):
            X[r] = ctx.sub(X[r], ctx.mul(K[r][k], delta[k]))
    arm = [ctx.mul(R2, ctx.sub(s, phi)), ctx.mul(R2, one_c), zero]
    mom = cross(arm, wl)
    Rphi = ctx.mul(R, phi)
    W = [ctx.mul(Rphi, wl[a]) for a in range(3)] + mom
    H = [[one if r == k else zero for k in range(6)] for r in range(6)]
    H[3][1], H[3][2] = ctx.neg(chord[2]), chord[1]
    H[4][0], H[4][2] = chord[2], ctx.neg(chord[0])
    H[5][0], H[5][1] = ctx.neg(chord[1]), chord[0]
    local = [zero] * 12
    for r in range(6):
        t = zero
        for k in range(6):
            t = ctx.add(t, ctx.mul(H[r][k], X[k]))
        local[r] = ctx.add(t, W[r])
        local[6 + r] = ctx.neg(X[r])
    out = []
    for blk in range(4):
        loc = local[3 * blk:3 * blk + 3]
        for comp in range(3):
            g = zero
            for a in range(3):
                g = ctx.add(g, ctx.mul(axes[a][comp], loc[a]))
            out.append(g)
    if ctx.name == "ball":
        out = [finish_component(b) for b in out]
    return out, F


def finish_component(b):
    """Output split (section 3.3 step 9): a truncated split adds 2^-1074."""
    terms, truncated = split_binary64(b.m)
    r = add_up(b.r, TINY) if truncated else b.r
    return Ball(b.m, finite(r, "output"))


def invert(ctx, F):
    if ctx.name == "decimal":
        return gauss_jordan(F, lambda a, b: a + b, lambda a, b: a - b,
                            lambda a, b: a * b, lambda a, b: a / b, Decimal(0), Decimal(1))
    # L7: point inverse of the midpoints at 128 bits, then the residual test.
    mid = [[F[r][k].m for k in range(6)] for r in range(6)]
    Xt = gauss_jordan(mid, lambda a, b: rnd(a + b), lambda a, b: rnd(a - b),
                      lambda a, b: rnd(a * b), lambda a, b: rnd(a / b), Q(0), Q(1))
    rho = 0.0
    for r in range(6):
        row = 0.0
        for k in range(6):
            acc = Ball(Q(1) if r == k else Q(0))
            for j in range(6):
                acc = b_sub(acc, b_mul(F[r][j], Ball(Xt[j][k])))
            row = add_up(row, add_up(abs_up(acc.m), acc.r))
        rho = max(rho, row)
    invert.last_rho = rho
    if not rho < 0.5:
        raise CertificateFailure(5, "flexibility inverse not certified: rho=%.3g" % rho)
    norm_x = 0.0
    for r in range(6):
        row = 0.0
        for k in range(6):
            row = add_up(row, abs_up(Xt[r][k]))
        norm_x = max(norm_x, row)
    rad = finite(div_up(mul_up(norm_x, rho), sub_dn(1.0, rho)), "L7")
    return [[Ball(Xt[r][k], rad) for k in range(6)] for r in range(6)]


invert.last_rho = float("nan")


def gauss_jordan(Fm, add, sub, mul, div, zero, one):
    n = 6
    m = [row[:] for row in Fm]
    inv = [[one if r == k else zero for k in range(n)] for r in range(n)]
    for col in range(n):
        piv = max(range(col, n), key=lambda r: abs(m[r][col]))
        m[col], m[piv] = m[piv], m[col]
        inv[col], inv[piv] = inv[piv], inv[col]
        p = m[col][col]
        if p == 0:
            raise CertificateFailure(6, "Gauss-Jordan pivot is zero (DivisionByZero)")
        m[col] = [div(v, p) for v in m[col]]
        inv[col] = [div(v, p) for v in inv[col]]
        for r in range(n):
            if r == col or m[r][col] == 0:
                continue
            f = m[r][col]
            m[r] = [sub(m[r][k], mul(f, m[col][k])) for k in range(n)]
            inv[r] = [sub(inv[r][k], mul(f, inv[col][k])) for k in range(n)]
    return inv


def certify(*args):
    """(balls, rho) or raises CertificateFailure."""
    invert.last_rho = float("nan")
    balls, _ = formula(BallCtx(), *args)
    return balls, invert.last_rho

# --------------------------------------------- binary64 emulations of CB


class SingularSystem(Exception):
    pass


DENSE_SOLVE_ZERO_PIVOT_GUARD = 1.0e-12


def solve_dense_fk(matrix, force, guard=True):
    """FK `solve_dense` (lib.rs:1632-1695 at 10b70036ef), operation order kept.
    guard=False drops only the absolute pivot guard (round 00's stand-in)."""
    size = len(matrix)
    a = [row[:] for row in matrix]
    rhs = list(force)
    for pivot in range(size):
        pivot_row, pivot_value = pivot, abs(a[pivot][pivot])
        for row in range(pivot + 1, size):
            if abs(a[row][pivot]) > pivot_value:
                pivot_row, pivot_value = row, abs(a[row][pivot])
        if guard and pivot_value <= DENSE_SOLVE_ZERO_PIVOT_GUARD:
            raise SingularSystem("pivot %d" % pivot)
        if pivot_row != pivot:
            a[pivot], a[pivot_row] = a[pivot_row], a[pivot]
            rhs[pivot], rhs[pivot_row] = rhs[pivot_row], rhs[pivot]
        diag = a[pivot][pivot]
        tail = a[pivot][pivot + 1:]
        for r in range(pivot + 1, size):
            factor = a[r][pivot] / diag
            a[r][pivot] = 0.0
            for k, pe in zip(range(pivot + 1, size), tail):
                a[r][k] -= factor * pe
            rhs[r] -= factor * rhs[pivot]
            if not all(math.isfinite(v) for v in a[r]) or not math.isfinite(rhs[r]):
                raise SingularSystem("non-finite elimination")
    sol = [0.0] * size
    for row in reversed(range(size)):
        s = rhs[row]
        for col in range(row + 1, size):
            s -= a[row][col] * sol[col]
        if guard and abs(a[row][row]) <= DENSE_SOLVE_ZERO_PIVOT_GUARD:
            raise SingularSystem("back substitution %d" % row)
        sol[row] = s / a[row][row]
        if not math.isfinite(sol[row]):
            raise SingularSystem("non-finite solution")
    return sol


def cb_vector(axes, radius, phi, s, c, s2, c2, chord, em, gm, area, inertia, torsion, kin, kout, w,
              strict=True):
    """CB's consistent_uniform_nodal_loads in binary64 (CB's operation order;
    FK `solve_dense` emulated, then CB's symmetric average)."""
    dot = lambda a, b: a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
    wl = [dot(axes[0], w), dot(axes[1], w), dot(axes[2], w)]
    R = radius
    cases = [
        ([-R * s, 0.0, R], [0.0] * 3, [0.0] * 3, [0.0, 0.0, -1.0]),
        ([R * c, -R, 0.0], [0.0] * 3, [0.0] * 3, [0.0, 1.0, 0.0]),
        ([0.0] * 3, [0.0, R * s, -R * c], [R, -R * c, -R * s], [0.0] * 3),
        ([0.0] * 3, [0.0, 1.0, 0.0], [0.0, 0.0, -1.0], [0.0] * 3),
        ([0.0] * 3, [0.0, 0.0, 1.0], [0.0, 1.0, 0.0], [0.0] * 3),
        ([1.0, 0.0, 0.0], [0.0] * 3, [0.0] * 3, [0.0] * 3),
    ]
    gram = [[phi, s, 1.0 - c], [s, 0.5 * phi + 0.25 * s2, 0.5 * s * s], [1.0 - c, 0.5 * s * s, 0.5 * phi - 0.25 * s2]]
    itc = phi * s + c - 1.0
    its = s - phi * c
    itcc = 0.25 * phi * phi + 0.25 * phi * s2 + 0.125 * (c2 - 1.0)
    itss = 0.25 * phi * phi - 0.25 * phi * s2 - 0.125 * (c2 - 1.0)
    itsc = 0.125 * s2 - 0.25 * phi * c2
    eg = [[phi, s, 1.0 - c, 0.5 * phi * phi, itc, its],
          [s, 0.5 * phi + 0.25 * s2, 0.5 * s * s, itc, itcc, itsc],
          [1.0 - c, 0.5 * s * s, 0.5 * phi - 0.25 * s2, its, itsc, itss]]
    wx, wy, wz = wl
    R2 = R * R
    ax_ = [s, -phi, -1.0, 0.0, 1.0, 0.0]
    ay_ = [-c, 1.0, -phi, 0.0, 0.0, 1.0]
    ld = ([R2 * (wy * ax_[k] - wx * ay_[k]) for k in range(6)],
          [R2 * wz, -R2 * wz * c, -R2 * wz * s, 0.0, 0.0, 0.0],
          [R2 * wz * phi, -R2 * wz * s, R2 * wz * c, -R2 * wz, 0.0, 0.0],
          [0.0, R * wy * phi, -R * wx * phi, 0.0, -R * wy, R * wx])

    def quad(g, l, r, n):
        t = 0.0
        for i in range(3):
            for k in range(n):
                t += l[i] * g[i][k] * r[k]
        return t

    EI, GJ, EA = em * inertia, gm * torsion, em * area

    def energy(qi, qo, qt, qa):
        return R * (kin * qi / EI + kout * qo / EI + qt / GJ + qa / EA)

    F = [[0.0] * 6 for _ in range(6)]
    for r in range(6):
        for k in range(r, 6):
            v = energy(*(quad(gram, cases[r][j], cases[k][j], 3) for j in range(4)))
            F[r][k] = F[k][r] = v
    delta = [energy(*(quad(eg, cases[r][j], ld[j], 6) for j in range(4))) for r in range(6)]
    K = [[0.0] * 6 for _ in range(6)]
    for col in range(6):
        e = [0.0] * 6
        e[col] = 1.0
        x = solve_dense_fk(F, e, guard=strict)
        for r in range(6):
            K[r][col] = x[r]
    for r in range(6):
        for k in range(r + 1, 6):
            a = 0.5 * (K[r][k] + K[k][r])
            K[r][k] = K[k][r] = a
    X = [0.0] * 6
    for r in range(6):
        for k in range(6):
            X[r] -= K[r][k] * delta[k]
    arm = [R2 * (s - phi), R2 * (1.0 - c), 0.0]
    mom = [arm[1] * wl[2] - arm[2] * wl[1], arm[2] * wl[0] - arm[0] * wl[2], arm[0] * wl[1] - arm[1] * wl[0]]
    W = [R * phi * wl[0], R * phi * wl[1], R * phi * wl[2]] + mom
    H = [[1.0 if r == k else 0.0 for k in range(6)] for r in range(6)]
    H[3][1], H[3][2] = -chord[2], chord[1]
    H[4][0], H[4][2] = chord[2], -chord[0]
    H[5][0], H[5][1] = -chord[1], chord[0]
    local = [0.0] * 12
    for r in range(6):
        t = 0.0
        for k in range(6):
            t += H[r][k] * X[k]
        local[r] = t + W[r]
        local[6 + r] = -X[r]
    out = []
    for blk in range(4):
        loc = local[3 * blk:3 * blk + 3]
        g = [0.0, 0.0, 0.0]
        for a in range(3):
            for comp in range(3):
                g[comp] += axes[a][comp] * loc[a]
        out.extend(g)
    return out


def norm(v):
    return math.sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2])


def sub3(a, b):
    return [a[0] - b[0], a[1] - b[1], a[2] - b[2]]


def cross3(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def objective_binary64(xi, xj, R, yref, mat, w, libm_trig=False, strict=False):
    """A naive binary64 closed form of the objective element (not T4-U1's
    stable form, which is not yet written): half-angle trig, no libm except
    atan2 for phi; H from the actual chord."""
    d = sub3(xj, xi)
    L = norm(d)
    s_h = (0.5 * L) / R
    c_h = math.sqrt(1.0 - s_h * s_h)
    phi = 2.0 * math.atan2(s_h, c_h)
    if libm_trig:
        s, c, s2, c2 = math.sin(phi), math.cos(phi), math.sin(2 * phi), math.cos(2 * phi)
    else:
        s, c = 2.0 * s_h * c_h, 1.0 - 2.0 * s_h * s_h
        s2, c2 = 2.0 * s * c, 1.0 - 2.0 * s * s
    dh = [v / L for v in d]
    pr = yref[0] * dh[0] + yref[1] * dh[1] + yref[2] * dh[2]
    nr = [yref[k] - pr * dh[k] for k in range(3)]
    nm = norm(nr)
    nh = [v / nm for v in nr]
    ex = [-s_h * dh[k] + c_h * nh[k] for k in range(3)]
    ez = cross3(nh, dh)
    ey = cross3(ez, ex)
    axes = [ex, ey, ez]
    chord = [sum(axes[a][k] * d[k] for k in range(3)) for a in range(3)]
    return cb_vector(axes, R, phi, s, c, s2, c2, chord, *mat, w, strict=strict)

# ------------------------------------------------------ the L line forecast


def straight_terms(p, q, w):
    d = sub3(q, p)
    L = norm(d)
    x = [v / L for v in d]
    f = [Q(wk) * Q(L) / 2 for wk in w]
    mx = cross3(x, w)
    m = [Q(mk) * Q(L) * Q(L) / 12 for mk in mx]
    return f + m, f + [-v for v in m]


def section(od, wall):
    ro, ri = od / 2, od / 2 - wall
    area = math.pi * (ro * ro - ri * ri)
    inertia = math.pi * (ro ** 4 - ri ** 4) / 4
    return area, inertia, 2 * inertia


def l_line(phi, offset, l1=3.0, l2=4.0, R=0.2286):
    loc = {"A": [0.0, 0.0, 0.0], "B": [l1, 0.0, 0.0],
           "C": [l1 + R * math.sin(phi), R * (1.0 - math.cos(phi)), 0.0]}
    tj = [math.cos(phi), math.sin(phi), 0.0]
    loc["D"] = [loc["C"][i] + l2 * tj[i] for i in range(3)]
    return {n: [loc[n][i] + offset[i] for i in range(3)] for n in loc}


def run_case(phi_rad, offset, w, k, label, l1=3.0, l2=4.0, controls=True, binary64=True, R=0.2286):
    X = l_line(phi_rad, offset, l1, l2, R)
    yref = [0.0, -1.0, 0.0]
    em = 2.03e11
    gm = em / (2.0 * 1.3)
    area, inertia, torsion = section(0.1683, 0.00711)
    mat = (em, gm, area, inertia, torsion, k, k)
    xi, xj = X["B"], X["C"]
    args = (xi, xj, R, yref, em, gm, area, inertia, torsion, k, k, w)
    try:
        ball, rho = certify(*args)
    except CertificateFailure as error:
        return {"label": label, "certificate": "CannotBound (%s)" % error, "rho": invert.last_rho}
    ref, _ = formula(DecCtx(), *args)
    enclosed = all(abs(Q(ref[i]) - ball[i].m) <= Q(ball[i].r) for i in range(12))
    scale = max(abs(float(b.m)) for b in ball)
    rad_rel = max(b.r for b in ball) / scale
    ab_i, ab_j = straight_terms(X["A"], X["B"], w)
    cd_i, cd_j = straight_terms(X["C"], X["D"], w)
    extent = math.sqrt(sum((max(X[n][a] for n in X) - min(X[n][a] for n in X)) ** 2 for a in range(3)))
    out = {"label": label, "rho": rho, "enclosed": enclosed, "radius_rel": rad_rel,
           "radius_share": guard_ratio(None, ball, ab_j, cd_i, extent)}
    if not binary64:
        return out
    try:
        v = objective_binary64(xi, xj, R, yref, mat, w, strict=True)
    except SingularSystem as error:
        out["refusal"] = "FK solve_dense refuses (%s); unguarded elimination:" % error
        v = objective_binary64(xi, xj, R, yref, mat, w, strict=False)
    out["objective_defect_rel"] = float(max(abs(Q(v[i]) - ball[i].m) for i in range(12)) / Q(scale))
    out["objective"] = guard_ratio(v, ball, ab_j, cd_i, extent)
    if controls:
        i_max = max(range(12), key=lambda i: abs(ball[i].m))
        t0 = guard_rows(ball, ab_j, cd_i, extent)[i_max]
        fired = []
        for factor in (Q(2), Q(1, 2)):
            vp = list(v)
            vp[i_max] = float(Q(v[i_max]) + factor * t0)
            fired.append(guard_ratio(vp, ball, ab_j, cd_i, extent) >= 1.0)
        out["perturbed"] = "+2T0 fires=%s, +T0/2 fires=%s" % tuple(fired)
    return out


def guard_rows(ball, sB, sC, extent):
    rows = []
    for sterms, offs in ((sB, 0), (sC, 6)):
        for dof in range(6):
            rows.append((dof, sterms[dof] + ball[offs + dof].m))
    F = max(abs(r[1]) for r in rows if r[0] < 3)
    M = max(abs(r[1]) for r in rows if r[0] >= 3)
    fo, mo = max(F, M / Q(extent)), max(M, Q(extent) * F)
    return [Q(CRITERION) * max(abs(n), mo if dof >= 3 else fo) for dof, n in rows]


def guard_ratio(v, ball, sB, sC, extent):
    """Max S11-G load-row statistic over the 12 free rows (nodes B, C).
    v = None gives the radius-only share (|A_net| = 0)."""
    t0s = guard_rows(ball, sB, sC, extent)
    worst = 0.0
    for i in range(12):
        defect = Q(0) if v is None else Q(v[i]) - ball[i].m
        worst = max(worst, float((abs(defect) + Q(ball[i].r)) / t0s[i]))
    return worst

# ------------------------------------------------- T15's body (curved_body)


def t15_body_case(k, w=(0.0, 0.0, 0.3)):
    """s11g_tests.rs `curved_body` at origin 0: chord (2, 0, 0), R = sqrt 2,
    y_reference (0, 1, 0), OD 0.168 m, wall 0.007 m, E 200 GPa, G 80 GPa,
    k_in = k_out = k, a uniform global_z load (T15's 0.3 N/m)."""
    xi, xj = [0.0, 0.0, 0.0], [2.0, 0.0, 0.0]
    R = math.sqrt(2.0)
    yref = [0.0, 1.0, 0.0]
    area, inertia, torsion = section(0.168, 0.007)
    em, gm = 200e9, 80e9
    args = (xi, xj, R, yref, em, gm, area, inertia, torsion, k, k, list(w))
    out = {"k": k}
    try:
        ball, rho = certify(*args)
        out["certificate"] = "certified, rho=%.3g, rad/max|m|=%.1e" % (
            rho, max(b.r for b in ball) / max(abs(float(b.m)) for b in ball))
        scale = max(abs(float(b.m)) for b in ball)
    except CertificateFailure as error:
        out["certificate"] = "Err: %s" % error
        ball, scale = None, None
    mat = (em, gm, area, inertia, torsion, k, k)
    try:
        v = objective_binary64(xi, xj, R, yref, mat, list(w), strict=True)
        out["binary64"] = "solve_dense ok, max|v|=%.2e" % max(abs(x) for x in v)
        if ball is None:
            ref, _ = formula(DecCtx(), *args)
            rs = max(abs(x) for x in ref)
            out["binary64"] += ", defect/max|f|=%.1e (70-digit reference)" % (
                max(abs(Decimal(v[i]) - ref[i]) for i in range(12)) / rs)
        else:
            out["binary64"] += ", defect/max|m|=%.1e" % float(
                max(abs(Q(v[i]) - ball[i].m) for i in range(12)) / Q(scale))
    except SingularSystem as error:
        out["binary64"] = "solve_dense refuses (%s): LOAD_INPUT_INVALID / build refusal" % error
    return out

# ----------------------------------------------------------------- near pi


def near_pi_case(target, seed):
    """RV129's construction (rv129_near_pi.py): a chord of length 0.4572 m
    with binary64 components, R the smallest binary64 value with 2R > L."""
    import random
    rng = random.Random(seed)
    best = None
    for _ in range(200000):
        a = rng.uniform(0.2, 1.3)
        d = [0.4572 * math.cos(a), 0.4572 * math.sin(a), 0.0]
        L2 = Q(d[0]) ** 2 + Q(d[1]) ** 2
        R = math.sqrt(float(L2)) / 2
        for _ in range(4):
            R = math.nextafter(R, 0.0)
        while Q(R) ** 2 * 4 <= L2:
            R = math.nextafter(R, math.inf)
        oms2 = 1 - L2 / (4 * Q(R) ** 2)
        score = abs(math.log10(float(oms2)) - math.log10(target))
        if best is None or score < best[0]:
            best = (score, d, R, oms2)
            if score < 0.3:
                break
    _, d, R, oms2 = best
    return [0.0, 0.0, 0.0], [d[0], d[1], 0.0], R, oms2

# ------------------------------------------------------------------- main


def fmt_case(r, names=("objective",)):
    if "certificate" in r:
        return "%s | %s" % (r["label"], r["certificate"])
    parts = [r["label"], "rho=%.2e" % r["rho"], "enclosed=%s" % r["enclosed"],
             "rad=%.1e" % r["radius_rel"], "rad-share=%.1e" % r["radius_share"]]
    for name in names:
        if name not in r:
            continue
        if isinstance(r[name], str):
            parts.append("%s: %s" % (name, r[name]))
        else:
            parts.append("%s: defect=%.1e guard=%.1e" % (name, r[name + "_defect_rel"], r[name]))
    if "perturbed" in r:
        parts.append("perturbed: %s" % r["perturbed"])
    if "refusal" in r:
        parts.insert(5, r["refusal"])
    return " | ".join(parts)


def main():
    getcontext().prec = 70
    print("T4-I9 r01 arc certificate probe (emulation; python %s)" % sys.version.split()[0])
    print("Ball = 128-bit midpoint (Wide<2> model) + binary64 radius, directed as DESIGN_R01 3.2.")
    print("rho = ||I - F X||_inf (certified < 1/2); enclosed = 70-digit reference inside every ball;")
    print("rad = max radius / max |m|; rad-share = max over the 12 free rows of radius / T0 (radius alone);")
    print("defect = max |v - m| / max |m| (naive binary64 objective closed form);")
    print("guard = max S11-G load-row statistic / threshold over the 12 free rows (>= 1 fires).")
    totals = {"certified": 0, "enclosed": 0, "fires2": 0, "silent_half": 0, "half_applicable": 0,
              "refused": 0}

    def tally(r):
        if "certificate" in r:
            totals["refused"] += 1
            return
        totals["certified"] += 1
        totals["enclosed"] += r["enclosed"]
        if "perturbed" in r:
            totals["fires2"] += "+2T0 fires=True" in r["perturbed"]
            if r["objective"] < 0.5:
                totals["half_applicable"] += 1
                totals["silent_half"] += "+T0/2 fires=False" in r["perturbed"]

    print()
    print("[1] L line: straight 3 m, bend R = 0.2286 m, straight 4 m; OD 168.3 mm, t 7.11 mm;")
    print("    E = 2.03e11 Pa, nu = 0.3; both ends anchored; uniform load on every member.")
    loads = {"self-weight -z": [0.0, 0.0, -450.0], "in-plane -y": [0.0, -450.0, 0.0]}
    offsets = {"origin": [0.0, 0.0, 0.0], "UTM 5e6/3.5e6": [5.0e6, 3.5e6, 0.0], "UTM 7.3e6": [7.3e6, 0.0, 0.0]}
    for lname, w in loads.items():
        for oname, off in offsets.items():
            for phi_deg in (90.0, 45.0, 15.0, 5.0, 1.0, 0.1):
                for k in (1.0, 2.0):
                    label = "%s | %s | phi=%g deg | k=%g" % (lname, oname, phi_deg, k)
                    r = run_case(math.radians(phi_deg), off, w, k, label)
                    tally(r)
                    print(fmt_case(r))
    print()
    print("[2] Bend-dominated body: straights 0.3 m and 0.3 m (self-weight -z, k = 1)")
    for oname, off in offsets.items():
        for phi_deg in (45.0, 15.0, 5.0, 1.0):
            label = "short | %s | phi=%g deg" % (oname, phi_deg)
            r = run_case(math.radians(phi_deg), off, loads["self-weight -z"], 1.0, label, 0.3, 0.3)
            tally(r)
            print(fmt_case(r))
    print()
    print("[3] Small angles (L line, self-weight, k = 1): the certificate's own share of the threshold")
    for oname in ("origin", "UTM 7.3e6"):
        for phi in (1e-4, 1e-5, 1e-6, 3e-7, 1e-7, 3e-8):
            label = "small | %s | phi=%g rad" % (oname, phi)
            r = run_case(phi, offsets[oname], loads["self-weight -z"], 1.0, label,
                         controls=False, binary64=False)
            tally(r)
            print(fmt_case(r))
    print()
    print("[3b] Small angles on a fixed 0.3 m chord (R = 0.15/sin(phi/2)), L line, self-weight, k = 1")
    for oname in ("origin", "UTM 7.3e6"):
        for phi in (1e-4, 1e-6, 1e-7, 1e-8, 3e-9):
            R = 0.15 / math.sin(phi / 2)
            label = "chord 0.3 m | %s | phi=%g rad | R=%.3g" % (oname, phi, R)
            r = run_case(phi, offsets[oname], loads["self-weight -z"], 1.0, label,
                         controls=False, binary64=False, R=R)
            tally(r)
            print(fmt_case(r))
    print()
    print("[4] Large k (L line, 90 deg, self-weight, origin): T15d's natural catch (A-5)")
    for k in (1e4, 1e6, 1e8, 1e9, 1e10, 1e11, 1e12, 1e15, 1e20):
        label = "large-k | phi=90 deg | k=%g" % k
        r = run_case(math.radians(90.0), offsets["origin"], loads["self-weight -z"], k, label, controls=True)
        tally(r)
        print(fmt_case(r))
    print()
    print("[5] T15's body (s11g_tests.rs curved_body) over k: T15c's natural window (A-4)")
    for k in (2.0, 1e8, 1e9, 1e10, 1e11, 1e12, 1e30, 1e34, 1e35, 2e35, 3e35, 4e35, 5e35, 6e35, 8e35,
              1e36, 1.5e36, 1e37, 1e38, 1e39, 1e40):
        r = t15_body_case(k)
        print("T15 body | k=%g | certificate: %s | %s" % (k, r["certificate"], r["binary64"]))
    print()
    print("[6] Near pi (RV129's skew-chord construction; self-weight, k = 1)")
    area, inertia, torsion = section(0.1683, 0.00711)
    em = 2.03e11
    gm = em / 2.6
    for phi in (math.pi - 1e-4, math.pi - 1e-6):
        label = "near pi | L line | phi=pi-%g" % (math.pi - phi)
        r = run_case(phi, offsets["origin"], loads["self-weight -z"], 1.0, label, controls=True)
        tally(r)
        print(fmt_case(r))
    for target, seed in ((1e-15, 1292), (1e-17, 1293), (1e-19, 1294)):
        xi, xj, R, oms2 = near_pi_case(target, seed)
        d = [xj[k] - xi[k] for k in range(3)]
        yref = [-d[1], d[0], 0.0]
        args = (xi, xj, R, yref, em, gm, area, inertia, torsion, 1.0, 1.0, [0.0, 0.0, -450.0])
        try:
            ball, rho = certify(*args)
        except CertificateFailure as error:
            print("near pi | 1-s^2=%.2e | %s" % (float(oms2), error))
            continue
        ref, _ = formula(DecCtx(), *args)
        getcontext().prec = 70
        enclosed = all(abs(Q(ref[i]) - ball[i].m) <= Q(ball[i].r) for i in range(12))
        scale = max(abs(float(b.m)) for b in ball)
        print("near pi | skew chord | 1-s^2=%.2e (pi-phi ~ %.1e) | rho=%.1e | rad/max|m|=%.1e | enclosed=%s"
              % (float(oms2), 2 * math.sqrt(float(oms2)), rho, max(b.r for b in ball) / scale, enclosed))
        totals["certified"] += 1
        totals["enclosed"] += enclosed
    print()
    print("[7] K3a atan_positive emulation against a 90-digit atan: max error / (u atan t)")
    worst = 0.0
    import random
    rng = random.Random(129)
    samples = [Q(1, 20), Q(1, 21), Q(1), Q(10) ** 9, Q(10) ** -9]
    samples += [Q(10 ** rng.uniform(-12, 12)) for _ in range(200)]
    for t in samples:
        t = rnd(t)
        m = atan_positive_k3a(t)
        getcontext().prec = 90
        exact = Q(atan_decimal(q_to_dec(t)))
        worst = max(worst, float(abs(m - exact) / (U * exact)))
    print("atan_positive: %d inputs, worst |m - atan t| = %.2f u atan(t) (K3a bound 21.54; L5 uses 22)"
          % (len(samples), worst))
    print()
    print("Totals: certified %d (enclosed %d); refused %d; +2T0 fires %d; +T0/2 silent %d of %d applicable"
          % (totals["certified"], totals["enclosed"], totals["refused"], totals["fires2"],
             totals["silent_half"], totals["half_applicable"]))
    print("done")


if __name__ == "__main__":
    main()
