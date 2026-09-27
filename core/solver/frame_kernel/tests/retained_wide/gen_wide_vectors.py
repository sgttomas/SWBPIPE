#!/usr/bin/env python3
"""Test vectors for `structural/retained/wide.rs` (T3 slice K3a, `Wide<2>`).

Standard library only. None of the expected values comes from the code under
test: every arithmetic result is the exact `fractions.Fraction` value (square
roots: `math.isqrt` with an exactness test) rounded once to p bits, to nearest,
ties to even. The arctangent reference is a 160-digit `decimal` value computed
two independent ways that must agree to 1e-140.

Usage:  python3 gen_wide_vectors.py [--check]

Writes, beside this script:
  targeted.txt             hard classes (ties, carry, cancellation, exact and
                           near-exact division and square root, sticky, TwoSum,
                           TwoProduct) at p in {53, 63, 64, 65, 127, 128}
  split.txt                the exact binary64 split, with the sub-2^-1074 cases
  atan.txt                 the included angle and the positive arctangent
  differential_sample.txt  the first records of both differential streams
  differential.txt         seeds, counts and sha256 digests of both streams
  SHA256SUMS               sha256 of the five files above
With --check, nothing is written; the files are regenerated in memory and
compared byte for byte with the committed ones.

Value tokens: `Z+` / `Z-` are +0 / -0; otherwise `<sign><32 hex digits>p<e>`,
the normalized 128-bit significand m (bit 127 set) and the exponent e of the
leading bit: value = sign * m * 2^(e - 127). Error tokens start with `E:`.

The differential streams are too large to commit (10^6 and 2*10^5 operations),
so the Rust test regenerates the operands from the recorded seed with the same
SplitMix64 generator and operand rules as `gen_operands` below, computes the
results with `Wide`, and compares the sha256 of its record stream (and of each
100 000-record chunk) with the digests recorded here.
"""
import hashlib
import math
import random
import sys
from decimal import Decimal, getcontext
from fractions import Fraction
from pathlib import Path

HERE = Path(__file__).resolve().parent
MASK64 = (1 << 64) - 1
TOP = 1 << 127
FULL = (1 << 128) - 1

DIFF_SEED_P128 = 0x4B33415F57494445  # "K3A_WIDE"
DIFF_SEED_MIXED = 0x4B33415F4D495844  # "K3A_MIXD"
DIFF_COUNT_P128 = 1_000_000
DIFF_COUNT_MIXED = 200_000
DIFF_CHUNK = 100_000
DIFF_SAMPLE = 1000
TARGET_SEED = 20260926
P_SET = (53, 63, 64, 65, 127, 128)


# ----------------------------------------------------------------------------
# The value model and the oracle
# ----------------------------------------------------------------------------
class W:
    """(-1)^neg * sig * 2^(exp - 127), sig normalized (bit 127) or zero."""

    __slots__ = ("neg", "exp", "sig")

    def __init__(self, neg, exp, sig):
        assert sig == 0 or TOP <= sig <= FULL, hex(sig)
        self.neg = bool(neg)
        self.exp = 0 if sig == 0 else exp
        self.sig = sig

    @staticmethod
    def zero(neg=False):
        return W(neg, 0, 0)

    @staticmethod
    def from_int(neg, m, lsb):
        """Exact value (-1)^neg * m * 2^lsb, m < 2^128."""
        if m == 0:
            return W.zero(neg)
        tz = (m & -m).bit_length() - 1
        if m.bit_length() > 128:
            m >>= tz
            lsb += tz
        bl = m.bit_length()
        assert bl <= 128
        return W(neg, lsb + bl - 1, m << (128 - bl))

    @staticmethod
    def from_f64_bits(bits):
        neg = bits >> 63
        be = (bits >> 52) & 0x7FF
        fr = bits & ((1 << 52) - 1)
        assert be != 0x7FF
        if be == 0:
            return W.from_int(neg, fr, -1074)
        return W.from_int(neg, fr | (1 << 52), be - 1075)

    def is_zero(self):
        return self.sig == 0

    def frac(self):
        v = Fraction(self.sig) * Fraction(2) ** (self.exp - 127)
        return -v if self.neg else v

    def negate(self):
        return W(not self.neg, self.exp, self.sig)

    def token(self):
        if self.sig == 0:
            return "Z-" if self.neg else "Z+"
        return "%s%032xp%d" % ("-" if self.neg else "+", self.sig, self.exp)

    def enc(self):
        return (
            bytes([1 if self.neg else 0])
            + self.exp.to_bytes(8, "little", signed=True)
            + (self.sig & MASK64).to_bytes(8, "little")
            + (self.sig >> 64).to_bytes(8, "little")
        )

    def fits(self, p):
        return self.sig & ((1 << (128 - p)) - 1) == 0


ONE = W(False, 0, TOP)


def floor_log2(a):
    n, d = a.numerator, a.denominator
    e = n.bit_length() - d.bit_length()
    if e >= 0:
        if n < (d << e):
            e -= 1
    elif (n << -e) < d:
        e -= 1
    return e


def rnd(x, p):
    """Round a nonzero Fraction once to p bits, to nearest, ties to even."""
    assert x != 0 and 2 <= p <= 128
    neg = x < 0
    a = -x if neg else x
    e = floor_log2(a)
    sh = p - 1 - e
    n, d = a.numerator, a.denominator
    if sh >= 0:
        num, den = n << sh, d
    else:
        num, den = n, d << (-sh)
    q, r = divmod(num, den)
    if 2 * r > den or (2 * r == den and q & 1):
        q += 1
    if q == 1 << p:
        q >>= 1
        e += 1
    return W(neg, e, q << (128 - p))


def round_int(neg, r, sticky, scale, p):
    """Round (r + f) * 2^scale, f in (0, 1) iff sticky, to p bits."""
    bl = r.bit_length()
    drop = bl - p
    if drop <= 0:
        assert not sticky
        return W.from_int(neg, r, scale)
    q = r >> drop
    rbit = (r >> (drop - 1)) & 1
    rest = sticky or (r & ((1 << (drop - 1)) - 1)) != 0
    if rbit and (rest or q & 1):
        q += 1
    e = scale + bl - 1
    if q == 1 << p:
        q >>= 1
        e += 1
    return W(neg, e, q << (128 - p))


def o_add(a, b, p):
    if a.is_zero() and b.is_zero():
        return W.zero(a.neg and b.neg)
    x = a.frac() + b.frac()
    if x == 0:
        return W.zero(False)
    return rnd(x, p)


def o_sub(a, b, p):
    return o_add(a, b.negate(), p)


def o_mul(a, b, p):
    if a.is_zero() or b.is_zero():
        return W.zero(a.neg != b.neg)
    return rnd(a.frac() * b.frac(), p)


def o_div(a, b, p):
    if b.is_zero():
        return "E:div0"
    if a.is_zero():
        return W.zero(a.neg != b.neg)
    return rnd(a.frac() / b.frac(), p)


def o_sqrt(a, p):
    if a.is_zero():
        return a
    if a.neg:
        return "E:neg_sqrt"
    # a = sig * 2^(exp-127). N = sig * 2^t with t >= 0, t + exp - 127 even and
    # N wide enough that isqrt(N) has at least p + 2 bits.
    e0 = a.exp - 127
    t = max(0, 2 * p + 4 - 128)
    if (t + e0) % 2:
        t += 1
    n = a.sig << t
    r = math.isqrt(n)
    return round_int(False, r, r * r != n, (e0 - t) // 2, p)


OPS = {"add": o_add, "sub": o_sub, "mul": o_mul, "div": o_div}


def apply(op, a, b, p):
    if op == "sqrt":
        return o_sqrt(a, p)
    return OPS[op](a, b, p)


def tok(v):
    return v if isinstance(v, str) else v.token()


# ----------------------------------------------------------------------------
# The positive arctangent and the included angle (the algorithm under test,
# emulated operation by operation; see wide.rs for the proof of its bound)
# ----------------------------------------------------------------------------
MAX_REDUCTIONS = 5
MAX_TERMS = 15


def atan_core(t, p, audit=None):
    k = 0
    while t.frac() >= Fraction(1, 20):
        if k == MAX_REDUCTIONS:
            return "E:reduction_bound"
        t2 = o_mul(t, t, p)
        b = o_add(ONE, t2, p)
        r = o_sqrt(b, p)
        d = o_add(ONE, r, p)
        tn = o_div(t, d, p)
        if audit is not None:
            # relative error of this step against the exact half-angle map
            tf = t.frac()
            exact = tf / (1 + Decimal_sqrt_frac(1 + tf * tf))
            audit.append(("reduction", abs(tn.frac() - exact) / exact))
        t = tn
        k += 1
    t2 = o_mul(t, t, p)
    thr = t.exp - p - 1
    adds = [t]
    term = t
    for n in range(1, MAX_TERMS + 1):
        term = o_mul(term, t2, p).negate()
        add = o_div(term, W.from_int(False, 2 * n + 1, 0), p)
        if add.is_zero() or add.exp < thr:
            break
        if n == MAX_TERMS:
            return "E:series_bound"
        adds.append(add)
    tail = W.zero(False)
    for a in reversed(adds[1:]):
        tail = o_add(tail, a, p)
    acc = o_add(t, tail, p)
    if audit is not None:
        exact = ref_atan(t.frac())
        audit.append(("series", abs(acc.frac() - exact) / exact))
        audit.append(("terms", len(adds)))
        audit.append(("k", k))
    return W(acc.neg, acc.exp + k, acc.sig)


def included_angle(s, c, p, audit=None):
    if s.is_zero() or s.neg:
        return "E:angle_domain"
    d = o_add(ONE, c, p)
    if d.is_zero() or d.neg:
        return "E:angle_domain"
    t = o_div(s, d, p)
    if audit is not None:
        exact = s.frac() / (1 + c.frac())
        audit.append(("initial", abs(t.frac() - exact) / exact))
    h = atan_core(t, p, audit)
    if isinstance(h, str):
        return h
    return W(h.neg, h.exp + 1, h.sig)


def atan_positive(t, p):
    if t.is_zero() or t.neg:
        return "E:angle_domain"
    return atan_core(t, p)


def v1_variant(s, c, p):
    """V1's emulated variant (forward summation, stop on ulp(acc)/4), for the
    comparison printed in the summary only."""
    d = o_add(ONE, c, p)
    t = o_div(s, d, p)
    k = 0
    while t.frac() >= Fraction(1, 20):
        t = o_div(t, o_add(ONE, o_sqrt(o_add(ONE, o_mul(t, t, p), p), p), p), p)
        k += 1
    t2 = o_mul(t, t, p)
    term, n, acc = t, 1, W.zero()
    while True:
        add = o_div(term, W.from_int(False, n, 0), p)
        if not acc.is_zero() and abs(add.frac()) < Fraction(2) ** (acc.exp - p + 1) / 4:
            break
        acc = o_add(acc, add, p)
        term = o_mul(term, t2, p).negate()
        n += 2
    return W(acc.neg, acc.exp + k + 1, acc.sig)


# 160-digit references ---------------------------------------------------------
DPREC = 160


def dec(fr):
    return Decimal(fr.numerator) / Decimal(fr.denominator)


def Decimal_sqrt_frac(fr):
    getcontext().prec = DPREC
    x = dec(fr).sqrt()
    return Fraction(x)


def ref_atan_halving(t):
    """Method 1: halve the angle until t < 1e-15, Taylor series, scale back."""
    getcontext().prec = DPREC
    k = 0
    while t > Decimal("1e-15"):
        t = t / (1 + (1 + t * t).sqrt())
        k += 1
    s, term, n = Decimal(0), t, 1
    t2 = t * t
    while True:
        add = term / n
        if s != 0 and abs(add) < abs(s) * Decimal(10) ** -(DPREC + 2):
            break
        s += add
        term = -term * t2
        n += 2
    return s * (2 ** k)


def euler_atan(x):
    """Euler's series, all terms positive: atan x = sum 2^2n (n!)^2/(2n+1)!
    * x^(2n+1)/(1+x^2)^(n+1). Used for 0 < x <= 1."""
    getcontext().prec = DPREC
    y = x * x / (1 + x * x)
    term = x / (1 + x * x)
    s = Decimal(0)
    n = 0
    while True:
        if s != 0 and term < s * Decimal(10) ** -(DPREC + 2):
            break
        s += term
        n += 1
        term = term * y * (2 * n) / (2 * n + 1)
    return s


_PI = None


def pi_machin():
    global _PI
    if _PI is None:
        getcontext().prec = DPREC
        _PI = 16 * euler_atan(Decimal(1) / 5) - 4 * euler_atan(Decimal(1) / 239)
    return _PI


def ref_atan_euler(t):
    """Method 2: Euler's series directly, with atan t = pi/2 - atan(1/t) above 1
    (pi by Machin's formula)."""
    getcontext().prec = DPREC
    if t <= 1:
        return euler_atan(t)
    return pi_machin() / 2 - euler_atan(1 / t)


def ref_atan(tf):
    getcontext().prec = DPREC
    t = dec(tf)
    r1 = ref_atan_halving(t)
    r2 = ref_atan_euler(t)
    assert abs(r1 - r2) <= abs(r1) * Decimal(10) ** -140, (tf, r1, r2)
    return Fraction(r1)


def ref_pair(ref):
    hi = rnd(ref, 128)
    rem = ref - hi.frac()
    lo = W.zero() if rem == 0 else rnd(rem, 128)
    return hi, lo


def ulps(got, ref, p):
    u = Fraction(2) ** (got.exp - p + 1)
    return abs(got.frac() - ref) / u


def dec_sin_cos(phi):
    """sin and cos of a Decimal angle (Taylor series; angles above 1e-10 are
    reduced by 2^20 and doubled back). Only the choice of test inputs depends
    on it: every reference is computed from the rounded inputs themselves."""
    getcontext().prec = DPREC + 20
    red = 20 if phi > Decimal("1e-10") else 0
    x = phi / 2 ** red
    eps = Decimal(10) ** -(DPREC + 20)
    s, term, n = Decimal(0), x, 1
    while term != 0 and abs(term) > abs(x) * eps:
        s += term
        term = -term * x * x / ((n + 1) * (n + 2))
        n += 2
    c, term, n = Decimal(0), Decimal(1), 0
    while term != 0 and abs(term) > eps:
        c += term
        term = -term * x * x / ((n + 1) * (n + 2))
        n += 2
    for _ in range(red):
        s, c = 2 * s * c, c * c - s * s
    getcontext().prec = DPREC
    return +s, +c


def round_fr_to_p(fr, p):
    return W.zero() if fr == 0 else rnd(fr, p)


# ----------------------------------------------------------------------------
# SplitMix64 and the differential operand rules (mirrored in the Rust test)
# ----------------------------------------------------------------------------
class SplitMix64:
    def __init__(self, seed):
        self.s = seed & MASK64

    def next(self):
        self.s = (self.s + 0x9E3779B97F4A7C15) & MASK64
        z = self.s
        z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & MASK64
        z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & MASK64
        return z ^ (z >> 31)


def rand_sig(rng):
    hi = rng.next()
    lo = rng.next()
    sel = rng.next()
    sig = ((hi << 64) | lo) | TOP
    kind = sel % 8
    if kind == 0:  # short significand
        nbits = 1 + (sel >> 8) % 128
        sig = (sig >> (128 - nbits)) << (128 - nbits)
    elif kind == 1:  # leading run of ones (carry-out)
        w = 1 + (sel >> 8) % 128
        sig |= ((1 << w) - 1) << (128 - w)
    elif kind == 2:  # short prefix and one far low bit (sticky)
        nbits = 1 + (sel >> 8) % 127
        low = (sel >> 16) % (128 - nbits)
        sig = ((sig >> (128 - nbits)) << (128 - nbits)) | (1 << low)
    return sig


def gen_operands(rng):
    r = rng.next()
    op = r % 5
    mode = (r >> 3) % 8
    a_neg = (r >> 6) & 1
    b_neg = (r >> 7) & 1
    ea = (r >> 8) % 4001 - 2000
    zero_a = (r >> 20) % 64 == 0
    zero_b = (r >> 26) % 64 == 0
    a_sig = rand_sig(rng)
    if mode <= 3:  # independent, exponents within 150
        eb = ea + rng.next() % 301 - 150
        b_sig = rand_sig(rng)
    elif mode == 4:  # within 15 units of the last place
        delta = rng.next() % 16
        b_sig = a_sig + delta if a_sig + delta <= FULL else a_sig - delta
        eb = ea
    elif mode == 5:  # low w bits differ
        w = rng.next() % 129
        x_hi = rng.next()
        x_lo = rng.next()
        b_sig = (a_sig ^ (((x_hi << 64) | x_lo) & ((1 << w) - 1))) | TOP
        eb = ea
    elif mode == 6:  # straddling a power of two
        w = rng.next() % 128
        x = rng.next()
        a_sig = FULL ^ (x & ((1 << w) - 1))
        w2 = rng.next() % 128
        y = rng.next()
        b_sig = TOP | (y & ((1 << w2) - 1))
        eb = ea + 1
    else:  # large exponent gap
        eb = ea + rng.next() % 3001 - 1500
        b_sig = rand_sig(rng)
    if op == 4:
        a_neg = 0
    a = W.zero(a_neg) if zero_a else W(a_neg, ea, a_sig)
    b = W.zero(b_neg) if zero_b else W(b_neg, eb, b_sig)
    return op, a, b


OP_NAMES = ("add", "sub", "mul", "div", "sqrt")


def run_stream(seed, count, mixed, sample_out):
    rng = SplitMix64(seed)
    total = hashlib.sha256()
    chunk = hashlib.sha256()
    chunks = []
    for i in range(count):
        p = 2 + rng.next() % 127 if mixed else 128
        op, a, b = gen_operands(rng)
        res = apply(OP_NAMES[op], a, b, p)
        if isinstance(res, str):
            assert res == "E:div0"
            status, body = 1, bytes(25)
        else:
            status, body = 0, res.enc()
        rec = (bytes([op, p, status]) if mixed else bytes([op, status])) + body
        total.update(rec)
        chunk.update(rec)
        if i < DIFF_SAMPLE:
            sample_out.append(
                "%s %d %s %d %s %s %s"
                % ("mixed" if mixed else "p128", i, OP_NAMES[op], p, a.token(), b.token(), tok(res))
            )
        if (i + 1) % DIFF_CHUNK == 0:
            chunks.append(chunk.hexdigest())
            chunk = hashlib.sha256()
    assert count % DIFF_CHUNK == 0
    return total.hexdigest(), chunks


# ----------------------------------------------------------------------------
# Targeted hard classes
# ----------------------------------------------------------------------------
def rand_odd(R, bits):
    return (1 << (bits - 1)) | R.getrandbits(bits - 1) | 1


def targeted():
    R = random.Random(TARGET_SEED)
    lines = []

    def emit(tag, op, p, a, b):
        res = apply(op, a, b, p)
        lines.append("%s %s %d %s %s %s" % (tag, op, p, a.token(), "-" if b is None else b.token(), tok(res)))
        return res

    for p in P_SET:
        # --- exact ties to even (round bit at the limb boundary for p = 64, 128)
        for _ in range(12):
            x = rand_odd(R, p + 1)  # p + 1 bits, last bit 1: a tie at p
            y = rand_odd(R, 20)
            s = R.randint(-60, 60)
            neg = R.random() < 0.5
            # x - y and x + y are even, so they carry at most p (or p + 1) bits
            emit("tie", "add", p, W.from_int(neg, x - y, s), W.from_int(neg, y, s))
            xy = x + y
            if (xy >> ((xy & -xy).bit_length() - 1)).bit_length() <= 128:
                emit("tie", "sub", p, W.from_int(neg, xy, s), W.from_int(neg, y, s))
        # multiplication ties: odd factors whose product has exactly p + 1 bits
        found = 0
        while found < 12:
            l1 = R.randint(max(2, p + 1 - 127), min(127, p))
            l2 = p + 1 - l1 + R.randint(0, 1)
            if l2 < 1 or l2 > 128:
                continue
            f1, f2 = rand_odd(R, l1), rand_odd(R, l2)
            if (f1 * f2).bit_length() != p + 1:
                continue
            emit("tie", "mul", p, W.from_int(R.random() < 0.5, f1, R.randint(-40, 40)), W.from_int(R.random() < 0.5, f2, R.randint(-40, 40)))
            found += 1
        # division ties: a = b * x with x of p + 1 bits (only when a fits 128 bits)
        for _ in range(8):
            if p + 1 <= 128:
                x = rand_odd(R, p + 1)
                bb = rand_odd(R, R.randint(1, 128 - (p + 1))) if p + 1 < 128 else 1
                emit("tie", "div", p, W.from_int(False, bb * x, R.randint(-30, 30)), W.from_int(R.random() < 0.5, bb, R.randint(-30, 30)))
        # square-root ties: a = x^2 with x of p + 1 bits (only p <= 63)
        if 2 * (p + 1) <= 128:
            for _ in range(8):
                x = rand_odd(R, p + 1)
                emit("tie", "sqrt", p, W.from_int(False, x * x, 2 * R.randint(-30, 30)), None)

        # --- carry-out and renormalization
        s = R.randint(-50, 50)
        emit("carry", "add", p, W.from_int(False, (1 << p) - 1, s), W.from_int(False, 1, s - 1))
        emit("carry", "add", p, W.from_int(True, (1 << p) - 1, s), W.from_int(True, 3, s - 2))
        emit("carry", "add", p, W.from_int(False, FULL, s), W.from_int(False, 1, s))
        for _ in range(6):
            m = TOP | R.getrandbits(127)
            emit("carry", "add", p, W.from_int(False, m, s), W.from_int(False, m, s))
            m2 = TOP | R.getrandbits(127)
            emit("carry", "add", p, W.from_int(True, m, s), W.from_int(True, m2, s))
        emit("carry", "mul", p, W.from_int(False, (1 << 64) - 1, 0), W.from_int(False, (1 << 64) + 1, 0))
        emit("carry", "mul", p, W.from_int(False, FULL, 3), W.from_int(True, FULL, -9))
        emit("carry", "mul", p, W.from_int(False, (1 << p) - 1, 0), W.from_int(False, (1 << p) + 1, 0) if p < 128 else W.from_int(False, 1, 0))
        emit("carry", "div", p, W.from_int(False, FULL, 0), W.from_int(False, 1, 128))
        emit("carry", "div", p, W.from_int(True, FULL, 7), W.from_int(False, FULL - 1, 7))
        emit("carry", "sqrt", p, W.from_int(False, FULL, 0), None)
        emit("carry", "sqrt", p, W.from_int(False, FULL, 1), None)
        emit("carry", "sub", p, W.from_int(False, 1, s + 200), W.from_int(False, 1, s))
        emit("carry", "sub", p, W.from_int(False, 1, s + 128), W.from_int(False, 1, s))
        emit("carry", "sub", p, W.from_int(False, 1, s + 129), W.from_int(False, 3, s))

        # --- massive cancellation
        for _ in range(8):
            m = TOP | R.getrandbits(127)
            if m == FULL:
                m -= 1
            e = R.randint(-300, 300)
            emit("cancel1ulp", "sub", p, W(False, e, m), W(False, e, m + 1))
            emit("cancel1ulp", "add", p, W(True, e, m), W(False, e, m + 1))
            # within one ulp of p bits
            mp = (m >> (128 - p)) << (128 - p)
            if mp + (1 << (128 - p)) <= FULL:
                emit("cancel1ulp", "sub", p, W(False, e, mp), W(False, e, mp + (1 << (128 - p))))
            emit("cancel1ulp", "sub", p, W(False, e, m), W(False, e, m))
            emit("cancel1ulp", "div", p, W(False, e, m), W(False, e, m + 1))
            emit("cancel1ulp", "mul", p, W(False, e, m), W(False, -e, FULL))
        emit("cancel1ulp", "sqrt", p, W(False, 0, FULL), None)
        emit("cancel1ulp", "sqrt", p, W(False, 1, TOP | 1), None)
        for pp in (53, 60, 100, 120, 127, 128):
            e = R.randint(-100, 100)
            m = TOP | R.getrandbits(127)
            delta = 1 << (128 - pp)
            if m + delta <= FULL:
                emit("cancelpprime", "sub", p, W(False, e, m + delta), W(False, e, m))
            # across a binade: 2^e and 2^e (1 - 2^-pp)
            emit("cancelpprime", "sub", p, W(False, e, TOP), W.from_int(False, (1 << pp) - 1, e - pp))
            emit("cancelpprime", "add", p, W(True, e, TOP), W.from_int(False, (1 << pp) - 1, e - pp))
            if pp < 128:
                emit("cancelpprime", "sub", p, W.from_int(False, (1 << pp) + 1, e - pp), W.from_int(False, (1 << pp) - 1, e - pp))

        # --- exact and near-exact division and square root
        for _ in range(10):
            lb = R.randint(1, 100)
            bb = rand_odd(R, lb)
            q = rand_odd(R, R.randint(1, 128 - lb))
            a = bb * q
            emit("exactdiv", "div", p, W.from_int(False, a, R.randint(-20, 20)), W.from_int(True, bb, R.randint(-20, 20)))
            for d in (-1, 1):
                if 0 < a + d and (a + d).bit_length() <= 128:
                    emit("nearexactdiv", "div", p, W.from_int(False, a + d, 0), W.from_int(False, bb, 0))
            x = R.getrandbits(64) | (1 << 63) | 1 if R.random() < 0.5 else R.getrandbits(R.randint(1, 64)) | 1
            sq = x * x
            e2 = 2 * R.randint(-40, 40)
            emit("perfectsq", "sqrt", p, W.from_int(False, sq, e2), None)
            w = W.from_int(False, sq, e2)
            # one ulp (of 128 bits) above and below the perfect square
            emit("nearsq", "sqrt", p, W(False, w.exp, w.sig + 1) if w.sig < FULL else w, None)
            emit("nearsq", "sqrt", p, W(False, w.exp, w.sig - 1) if w.sig > TOP else W(False, w.exp - 1, FULL), None)
            # one ulp of p bits away
            wp = W(False, w.exp, (w.sig >> (128 - p)) << (128 - p))
            if wp.sig + (1 << (128 - p)) <= FULL:
                emit("nearsq", "sqrt", p, W(False, wp.exp, wp.sig + (1 << (128 - p))), None)

        # --- sticky-bit paths
        for _ in range(6):
            # a has p+1 bits ending in 1 (a tie on its own) and b is far below:
            # the sticky bit alone decides the direction.
            if p <= 127:
                x = rand_odd(R, p + 1)
                s = R.randint(-50, 50)
                for sign in (False, True):
                    emit("sticky", "add", p, W.from_int(False, x, s), W.from_int(sign, 1, s - 200))
                    emit("sticky", "sub", p, W.from_int(True, x, s), W.from_int(sign, 1, s - 300))
            # half-ulp plus a far tail inside one operand (exponent gap > 128)
            m = (TOP | R.getrandbits(127)) >> (128 - p) << (128 - p)
            e = R.randint(-50, 50)
            emit("sticky", "add", p, W(False, e, m), W.from_int(False, (1 << 127) | 1, e - p - 127))
            emit("sticky", "add", p, W(False, e, m), W.from_int(True, (1 << 127) | 1, e - p - 127))
            emit("sticky", "sub", p, W(True, e, m), W.from_int(False, (1 << 127) | 1, e - p - 127))
            emit("sticky", "sub", p, W(False, e, m), W.from_int(False, (1 << 127) | 1, e - p - 127))
        # multiplication: sparse factors put the round bit and a lone low bit
        found = 0
        tries = 0
        while found < 10 and tries < 200000:
            tries += 1
            l1 = R.randint(2, 128)
            l2 = R.randint(2, 128)
            f1 = (1 << (l1 - 1)) | (1 << R.randint(0, l1 - 2)) | (1 if R.random() < 0.7 else 0)
            f2 = (1 << (l2 - 1)) | (1 << R.randint(0, l2 - 2)) | 1
            prod = f1 * f2
            bl = prod.bit_length()
            if bl < p + 3:
                continue
            drop = bl - p
            rbit = (prod >> (drop - 1)) & 1
            rest = prod & ((1 << (drop - 1)) - 1)
            keep = prod >> drop
            if rbit and rest and keep % 2 == 0:
                emit("sticky", "mul", p, W.from_int(False, f1, 0), W.from_int(False, f2, 0))
                found += 1
        assert found == 10, (p, found)
        # division: choose the 130-bit quotient Q (round bit set, tail zero) and
        # the divisor, then a with a nonzero remainder: sticky alone rounds up.
        found = 0
        while found < 10:
            bb = TOP | R.getrandbits(127) | 1
            Q = (1 << 129) | (R.getrandbits(p - 1) << (130 - p)) | (1 << (129 - p))
            Q &= ~(1 << (130 - p))  # keep's last bit even: a tie would round down
            num = bb * Q
            rem = (-num) % (1 << 129)
            if rem == 0 or rem >= bb:
                continue
            a = (num + rem) >> 129
            if not (TOP <= a <= FULL):
                continue
            emit("sticky", "div", p, W.from_int(False, a, 0), W.from_int(False, bb, 0))
            found += 1
        # square root: x^2 + 1 with x of p + 1 bits and an even kept part (a
        # tie without the sticky bit; only p <= 63), else the square of a
        # tie-shaped root rounded up to 128 bits (a near tie)
        for _ in range(8):
            x = rand_odd(R, p + 1) & ~2
            sq = x * x
            if sq.bit_length() < 128:
                emit("sticky", "sqrt", p, W.from_int(False, sq + 1, 0), None)
            else:
                bl = sq.bit_length()
                m = (sq >> (bl - 128)) + 1
                if m <= FULL:
                    emit("neartie", "sqrt", p, W.from_int(False, m, bl - 128), None)

    # --- TwoSum and TwoProduct (error-free transformations)
    for p in (53, 64, 128):
        for _ in range(40):
            a = rnd(Fraction(R.getrandbits(128) | 1) * Fraction(2) ** R.randint(-200, 200), p)
            b = rnd(Fraction(R.getrandbits(128) | 1) * Fraction(2) ** R.randint(-200, 200), p)
            if R.random() < 0.5:
                b = b.negate()
            for x, y in ((a, b), (b, a)):
                s = o_add(x, y, p)
                bv = o_sub(s, x, p)
                e = o_add(o_sub(x, o_sub(s, bv, p), p), o_sub(y, bv, p), p)
                assert s.frac() + e.frac() == x.frac() + y.frac()
                lines.append("eft twosum %d %s %s %s %s" % (p, x.token(), y.token(), s.token(), e.token()))
            # Dekker's TwoProduct with Veltkamp's split, splitter 2^ceil(p/2)+1
            C = W.from_int(False, (1 << ((p + 1) // 2)) + 1, 0)

            def split(v):
                c = o_mul(C, v, p)
                hi = o_sub(c, o_sub(c, v, p), p)
                return hi, o_sub(v, hi, p)

            x = o_mul(a, b, p)
            ah, al = split(a)
            bh, bl = split(b)
            e1 = o_sub(x, o_mul(ah, bh, p), p)
            e2 = o_sub(e1, o_mul(al, bh, p), p)
            e3 = o_sub(e2, o_mul(ah, bl, p), p)
            y = o_sub(o_mul(al, bl, p), e3, p)
            assert x.frac() + y.frac() == a.frac() * b.frac(), p
            lines.append("eft twoprod %d %s %s %s %s" % (p, a.token(), b.token(), x.token(), y.token()))
    return lines


# ----------------------------------------------------------------------------
# The exact split into at most three binary64 terms
# ----------------------------------------------------------------------------
def f64_bits_exact(neg, c, lsb):
    """Bits of the binary64 value (-1)^neg * c * 2^lsb (exact by construction)."""
    fr = Fraction(c) * Fraction(2) ** lsb
    top = c.bit_length() - 1
    e = lsb + top
    assert c > 0 and lsb >= -1074 and e <= 1023 and c < (1 << 53)
    if e >= -1022:
        m = c << (52 - top)
        bits = ((e + 1023) << 52) | (m & ((1 << 52) - 1))
    else:
        bits = c << (lsb + 1074)
        assert bits < (1 << 52)
    out = bits | ((1 << 63) if neg else 0)
    assert W.from_f64_bits(out).frac() == (-fr if neg else fr)
    return out


def split_value(w):
    if w.is_zero():
        return [], False
    if w.exp > 1023:
        return "E:split_overflow", None
    terms = []
    truncated = False
    for hi_bit, width in ((127, 53), (74, 53), (21, 22)):
        chunk = (w.sig >> (hi_bit - width + 1)) & ((1 << width) - 1)
        lsb = w.exp - 127 + hi_bit - width + 1
        if chunk == 0:
            continue
        if lsb < -1074:
            sh = -1074 - lsb
            if sh >= 64:
                truncated = True
                continue
            if chunk & ((1 << sh) - 1):
                truncated = True
            chunk >>= sh
            lsb = -1074
            if chunk == 0:
                continue
        terms.append(f64_bits_exact(w.neg, chunk, lsb))
    # independent check: exact remainder below 2^-1074, same sign, zero iff exact
    rem = w.frac() - sum((W.from_f64_bits(t).frac() for t in terms), Fraction(0))
    assert abs(rem) < Fraction(2) ** -1074
    assert (rem != 0) == truncated
    assert rem == 0 or (rem < 0) == w.neg
    return terms, truncated


def split_lines():
    R = random.Random(TARGET_SEED + 1)
    vals = []
    sig_patterns = [TOP, FULL, TOP | 1, TOP | (1 << 75), TOP | (1 << 74), TOP | (1 << 22), TOP | (1 << 21),
                    (FULL >> 75) << 75, TOP | ((1 << 22) - 1), TOP | ((1 << 75) - 1)]
    exps = [1024, 1023, 1022, 0, -1, 52, -52, -1022, -1023, -1021, -970, -969, -1000, -1021 - 52,
            -1022 - 53, -1074 + 105, -1074 + 106, -1074 + 127, -1074 + 128, -1074 + 52, -1074 + 53, -1074, -1075, -1076, -1200, -5000]
    for e in exps:
        for m in sig_patterns:
            for neg in (False, True):
                vals.append(W(neg, e, m))
    for _ in range(300):
        e = R.choice([R.randint(-1200, 1100), R.randint(-1110, -1000), R.randint(1000, 1030)])
        vals.append(W(R.random() < 0.5, e, TOP | R.getrandbits(127)))
    vals.append(W.zero(False))
    vals.append(W.zero(True))
    out = []
    for w in vals:
        res, tr = split_value(w)
        if isinstance(res, str):
            out.append("split %s %s" % (w.token(), res))
        else:
            out.append("split %s %d %s %d" % (w.token(), len(res), " ".join("%016x" % t for t in res) if res else "-", 1 if tr else 0))
    return out


# ----------------------------------------------------------------------------
# Arctangent vectors
# ----------------------------------------------------------------------------
def parse_token(tok):
    if tok in ("Z+", "Z-"):
        return W.zero(tok == "Z-")
    hexpart, e = tok[1:].split("p")
    return W(tok[0] == "-", int(e), int(hexpart, 16))


RV2_R2_INPUTS = [
    ("angle", 128, ("+afe8cd16013c9f292103f35b822d7f35p-1", "+b9fd2bf4af032d764c3c967ef87dbbc0p-1")),
    ("angle", 128, ("+d0c689d451a552797c397657df7a910bp-1", "+9426b410e5342962497e78ef8c48c6dep-1")),
    ("angle", 128, ("+ffc52bb2a14511d872043a2c9e4e3eb5p-1", "-ad839131250967eb3338de0703f6e20cp-5")),
    ("angle", 128, ("+f33bd063fe3d35c15b3711a27b8b32bbp-2", "+e1446c8d3c36a1a4804a721c0ba007dep-1")),
    ("angle", 128, ("+ffccc6f6acdd6a7de4d7f808ef5ec424p-1", "+a1e9bb35b0b2ae9c91ba8ae3f28e1d3ep-5")),
    ("angle", 128, ("+ff7329cf39ddda657d4da133705043e9p-1", "+86315fa38308fb13ffa389c27d14508cp-4")),
    ("angle", 128, ("+e7b6cc90e2471d7e7bc85930bd6a754fp-1", "-d9aa6eb88b0551b8c19f37e5c5b8fc35p-2")),
    ("angle", 128, ("+fb12ee04c4494a91c1848bcb9570f2d8p-1", "-c7ea3ad3ec179c47dd9319039a66461ap-3")),
    ("angle", 128, ("+ca99bd44f6b2a99e7250c8a1ffa14e86p-2", "-eb1ad27bde6ca02be88bd4da7bcd5db8p-1")),
    ("angle", 65, ("+e06546eac61ed94c25f86b273f5e47aap-1", "+f66f7a2836536e8fa4785f0fb4bcabdcp-2")),
    ("atanpos", 128, ("+9ec0f1308a9fd4ca0cc1cb4f4f6c9bb3p60",)),
    ("atanpos", 128, ("+d6b886a36169f14c184c1ba2e170bdedp106",)),
    ("atanpos", 128, ("+f0d6599fafe8a0e94d0cff86f4e59ba9p24",)),
]


def atan_lines(summary):
    getcontext().prec = DPREC
    R = random.Random(TARGET_SEED + 2)
    pi = Fraction(pi_machin())
    out = []
    worst = {}
    v1_worst = [0.0, None]
    audit_max = {"initial": Fraction(0), "reduction": Fraction(0), "series": Fraction(0)}
    hist = {"le1": 0, "le2": 0, "le2.69": 0, "le4": 0, "le6": 0, "gt6": 0}
    stats = {"max_k": 0, "max_terms": 0, "count": 0}

    def angle_case(tag, phi_dec, p_in, p):
        """s, c: sin and cos of phi, each rounded to p_in bits."""
        s_d, c_d = dec_sin_cos(phi_dec)
        s = round_fr_to_p(Fraction(s_d), p_in)
        c = round_fr_to_p(Fraction(c_d), p_in)
        pair_case(tag, s, c, p)

    def pair_case(tag, s, c, p):
        audit = [] if p == 128 else None
        got = included_angle(s, c, p, audit)
        if isinstance(got, str):
            out.append("angle %s %d %s %s %s - -" % (tag, p, s.token(), c.token(), got))
            return
        tf = s.frac() / (1 + c.frac())
        ref = 2 * ref_atan(tf)
        hi, lo = ref_pair(ref)
        err = ulps(got, ref, p)
        record(tag, p, err, audit)
        if p == 128:
            v = v1_variant(s, c, p)
            ev = float(ulps(v, ref, p))
            if ev > v1_worst[0]:
                v1_worst[0], v1_worst[1] = ev, tag
        out.append("angle %s %d %s %s %s %s %s" % (tag, p, s.token(), c.token(), got.token(), hi.token(), lo.token()))

    def t_case(tag, t, p):
        got = atan_positive(t, p)
        if isinstance(got, str):
            out.append("atanpos %s %d %s %s - -" % (tag, p, t.token(), got))
            return
        ref = ref_atan(t.frac())
        hi, lo = ref_pair(ref)
        record(tag, p, ulps(got, ref, p), None)
        out.append("atanpos %s %d %s %s %s %s" % (tag, p, t.token(), got.token(), hi.token(), lo.token()))

    def record(tag, p, err, audit):
        stats["count"] += 1
        if p == 128:
            for lim, key in ((1, "le1"), (2, "le2"), (Fraction(269, 100), "le2.69"), (4, "le4"), (6, "le6")):
                if err <= lim:
                    hist[key] += 1
                    break
            else:
                hist["gt6"] += 1
        key = p
        if key not in worst or err > worst[key][0]:
            worst[key] = (err, tag)
        if audit:
            for kind, val in audit:
                if kind in audit_max:
                    audit_max[kind] = max(audit_max[kind], val)
                elif kind == "k":
                    stats["max_k"] = max(stats["max_k"], val)
                elif kind == "terms":
                    stats["max_terms"] = max(stats["max_terms"], val)

    pid = Decimal(pi.numerator) / Decimal(pi.denominator)
    # V1's 13 angles, binary64 inputs
    for phi in ("1e-12", "1e-6", "1e-3", "0.1", "0.5", "1.0", "PI/2", "2.0", "3.0", "3.1", "PI-1e-6", "PI-1e-7", "1e-300"):
        if phi == "PI/2":
            ph = pid / 2
        elif phi.startswith("PI-"):
            ph = pid - Decimal(phi[3:])
        else:
            ph = Decimal(phi)
        angle_case("v1:" + phi, ph, 53, 128)
    # decades toward 0: binary64 inputs to 1e-300, 128-bit inputs to 1e-40
    for k in range(1, 301):
        angle_case("zero53:1e-%d" % k, Decimal(10) ** -k, 53, 128)
    for k in range(1, 41):
        angle_case("zero128:1e-%d" % k, Decimal(10) ** -k, 128, 128)
    # decades toward pi: binary64 inputs to pi-1e-8 (then c = -1, refused),
    # 128-bit inputs to pi-1e-19 (then c = -1, refused)
    for k in range(1, 11):
        angle_case("pi53:1e-%d" % k, pid - Decimal(10) ** -k, 53, 128)
    for k in range(1, 23):
        angle_case("pi128:1e-%d" % k, pid - Decimal(10) ** -k, 128, 128)
    # pi/2 and landmarks
    for num, den in ((1, 2), (1, 3), (2, 3), (1, 4), (3, 4), (1, 6), (5, 6), (1, 64), (63, 64), (31, 32)):
        for p_in in (53, 128):
            angle_case("frac:%d/%dpi:%d" % (num, den, p_in), pid * num / den, p_in, 128)
    # uniform random angles
    for i in range(1500):
        u = Decimal(R.randint(1, 10 ** 30 - 1)) / Decimal(10 ** 30)
        angle_case("rand128:%d" % i, pid * u, 128, 128)
    for i in range(300):
        u = Decimal(R.randint(1, 10 ** 30 - 1)) / Decimal(10 ** 30)
        angle_case("rand53:%d" % i, pid * u, 53, 128)
    # log-uniform small and near-pi angles
    for i in range(200):
        ex = R.uniform(-300, 0)
        angle_case("logsmall:%d" % i, Decimal(10) ** Decimal(repr(ex)), 128, 128)
    for i in range(200):
        ex = R.uniform(-19, 0)
        angle_case("lognearpi:%d" % i, pid - Decimal(10) ** Decimal(repr(ex)), 128, 128)
    # inconsistent pairs: s, c independent (the function is 2 atan(s/(1+c)))
    for i in range(200):
        s = rnd(Fraction(R.getrandbits(128) | 1, 1 << 128) * Fraction(2) ** R.randint(-100, 100), 128)
        c = rnd(Fraction(R.getrandbits(128) | 1, 1 << 128) * 3 - 1, 128)
        pair_case("pair:%d" % i, s, c, 128)
    # far toward pi with a large quotient: c = -1 + 2^-128 and huge s
    for j in (0, 1, 64, 127, 1000, 5000):
        pair_case("farpi:%d" % j, W.from_int(False, 1, j), W(True, -1, FULL), 128)
    # other precisions (the tolerance is in ulps of p)
    for p in (53, 64, 100, 127):
        for i in range(60):
            u = Decimal(R.randint(1, 10 ** 30 - 1)) / Decimal(10 ** 30)
            angle_case("p%d:%d" % (p, i), pid * u, 128, p)
    # independent review inputs (RV2, K3A_REVIEW.md): the S1 input (6.08 ulp at
    # p = 128) and the 13 inputs that separate the smallest-first tail summation
    # from largest first (S2, r2_distinguishing_inputs.txt)
    pair_case("rv2s1:0", parse_token("+ff4014cc2258bc73504f91ab8019dddcp-1"),
              parse_token("-9c9e9003feb63e97c537ba9a6156981bp-4"), 128)
    for i, (kind, p_r, args) in enumerate(RV2_R2_INPUTS):
        if kind == "angle":
            pair_case("rv2r2:%d" % i, parse_token(args[0]), parse_token(args[1]), p_r)
        else:
            t_case("rv2r2:%d" % i, parse_token(args[0]), p_r)
    # domain refusals
    pair_case("refuse:s0", W.zero(False), W.zero(False), 128)
    pair_case("refuse:sneg", W.from_int(True, 1, -3), W.zero(False), 128)
    pair_case("refuse:cm1", W.from_int(False, 1, -3), W(True, 0, TOP), 128)
    pair_case("refuse:clt", W.from_int(False, 1, -3), W(True, 1, TOP), 128)
    # positive arctangent over t: decades 1e-300 .. 1e300 and powers of two
    for k in range(-300, 301, 3):
        t_case("t10:%d" % k, rnd(Fraction(10) ** k, 128), 128)
    for j in list(range(-1100, 1101, 100)) + [-100000, 100000]:
        t_case("t2:%d" % j, W.from_int(False, 1, j), 128)
    # around the reduction threshold 1/20 and the step boundaries
    for base in (Fraction(1, 20),):
        h = rnd(base, 128)
        for d in range(-3, 4):
            t_case("thr:%d" % d, W(False, h.exp, h.sig + d), 128)
    for kk in range(1, 6):
        # tan(pi/2^(kk+1)) * 20: the input whose k-th reduction lands at 1/20
        getcontext().prec = DPREC
        th = Fraction(ref_atan(Fraction(1, 20))) * 2 ** kk
        s_d, c_d = dec_sin_cos(dec(th))
        tb = rnd(Fraction(s_d) / Fraction(c_d), 128)
        for d in (-2, 0, 2):
            t_case("step%d:%d" % (kk, d), W(False, tb.exp, tb.sig + d), 128)
    t_case("refuse:t0", W.zero(False), 128)
    t_case("refuse:tneg", W.from_int(True, 3, 0), 128)
    summary["atan_worst_ulps"] = {str(k): [float(v[0]), v[1]] for k, v in worst.items()}
    summary["atan_v1_variant_worst_p128"] = v1_worst
    u = Fraction(1, 1 << 128)
    summary["atan_audit_p128"] = {
        "max_initial_rel_err_over_u128": float(audit_max["initial"] / u),
        "max_reduction_rel_err_over_u128": float(audit_max["reduction"] / u),
        "max_series_rel_err_over_u128": float(audit_max["series"] / u),
        "p128_error_histogram_ulps": hist,
        "max_reductions": stats["max_k"],
        "max_series_terms": stats["max_terms"],
        "vectors_with_reference": stats["count"],
    }
    return out


# ----------------------------------------------------------------------------
def build():
    summary = {}
    files = {}
    t = targeted()
    files["targeted.txt"] = "\n".join(t) + "\n"
    files["split.txt"] = "\n".join(split_lines()) + "\n"
    files["atan.txt"] = "\n".join(atan_lines(summary)) + "\n"
    sample = []
    h128, c128 = run_stream(DIFF_SEED_P128, DIFF_COUNT_P128, False, sample)
    hmx, cmx = run_stream(DIFF_SEED_MIXED, DIFF_COUNT_MIXED, True, sample)
    files["differential_sample.txt"] = "\n".join(sample) + "\n"
    man = [
        "stream p128 seed %016x count %d chunk %d sha256 %s" % (DIFF_SEED_P128, DIFF_COUNT_P128, DIFF_CHUNK, h128),
    ]
    man += ["chunk p128 %d %s" % (i, h) for i, h in enumerate(c128)]
    man.append("stream mixed seed %016x count %d chunk %d sha256 %s" % (DIFF_SEED_MIXED, DIFF_COUNT_MIXED, DIFF_CHUNK, hmx))
    man += ["chunk mixed %d %s" % (i, h) for i, h in enumerate(cmx)]
    files["differential.txt"] = "\n".join(man) + "\n"
    sums = "".join(
        "%s  %s\n" % (hashlib.sha256(files[n].encode()).hexdigest(), n)
        for n in ("targeted.txt", "split.txt", "atan.txt", "differential_sample.txt", "differential.txt")
    )
    files["SHA256SUMS"] = sums
    summary["targeted_lines"] = len(t)
    summary["diff_p128_sha256"] = h128
    summary["diff_mixed_sha256"] = hmx
    return files, summary


def main():
    import json

    check = "--check" in sys.argv[1:]
    files, summary = build()
    status = 0
    for name, text in files.items():
        path = HERE / name
        if check:
            same = path.exists() and path.read_text() == text
            print("%s %s" % ("OK  " if same else "DIFF", name))
            status |= 0 if same else 1
        else:
            path.write_text(text)
    print(json.dumps(summary, indent=1, sort_keys=True))
    return status


if __name__ == "__main__":
    sys.exit(main())
