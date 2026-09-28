#!/usr/bin/env python3
"""Test vectors for `structural/retained/wide/multi.rs` (T3 slice K3).

Standard library only. None of the expected values comes from the code under
test:
- every arithmetic result is the exact `fractions.Fraction` value (square
  roots: `math.isqrt` with an exactness test) rounded once to p bits, to
  nearest, ties to even;
- every binary64 conversion is rounded once from the exact Fraction at the
  binary64 quantum, and cross-checked against CPython's correctly rounded
  `float(Fraction)` (a second, independent method); the subnormal relative
  precision 2^-1075/|value| is rounded upward with `math.nextafter`;
- TwoSum and TwoProduct results are s = fl_p(x op y) and e = (x op y) - s,
  checked to be exactly representable at p.

Usage:  python3 gen_wide_k3_vectors.py [--check]

Writes, beside this script:
  targeted_l4.txt, targeted_l8.txt, targeted_l16.txt
                           hard classes (ties at every limb boundary, carries
                           through every limb, cancellation, exact and
                           near-exact division and square root, sticky paths)
                           at every required precision of each width, and
                           (appended last, from their own generator) far
                           subtractions whose (1 - f) tail decides a tie
  conversion.txt           the binary64 conversion's boundary classes at
                           L = 2, 4, 8 and 16
  eft.txt                  TwoSum and TwoProduct, narrowing, and the integer
                           constructor (including 68-limb magnitudes at
                           quantum 2^-2148)
  differential_sample.txt  the first records of every stream
  differential.txt         seeds, counts and sha256 digests of every stream
  SHA256SUMS               sha256 of the files above
With --check, nothing is written; the files are regenerated in memory and
compared byte for byte with the committed ones.

Value tokens: `Z+` / `Z-` are +0 / -0; otherwise `<sign><hex>p<e>`: the
significand's hex digits from the top, left-aligned to 64L bits, with trailing
zero digits omitted, and the exponent e of the leading bit. At width L the
value is sign * m * 2^(e - (64L - 1)). Error tokens start with `E:`.
Conversion results: `N:<bits>`, `S:<bits>:<relative precision bits>`, `U+`,
`U-`, `O+`, `O-` (binary64 bits as 16 hex digits).

The streams are too large to commit, so the Rust test regenerates the
operands from the recorded seed with the same SplitMix64 generator and operand
rules as below, computes the results with `Wide`, and compares the sha256 of
its record stream (and of each 100 000-record chunk) with the digests here.
"""
import hashlib
import math
import random
import struct
import sys
from fractions import Fraction
from pathlib import Path

HERE = Path(__file__).resolve().parent
MASK64 = (1 << 64) - 1
WIDTHS = (4, 8, 16)
CONV_WIDTHS = (2, 4, 8, 16)
CLASS_PRECISIONS = (53, 128, 192, 256, 320, 512, 576, 1024)
TARGET_SEED = 20260928
DIFF_CHUNK = 100_000
DIFF_SAMPLE = 1000


def seed_of(tag):
    """Eight ASCII characters, read big-endian."""
    assert len(tag) == 8
    return int.from_bytes(tag.encode(), "big")


# (name, L, p or None for mixed, count, seed tag)
STREAMS = (
    ("p256", 4, 256, 1_000_000, "K3_P0256"),
    ("p512", 8, 512, 1_000_000, "K3_P0512"),
    ("p1024", 16, 1024, 1_000_000, "K3_P1024"),
    ("mixed4", 4, None, 200_000, "K3_MIX04"),
    ("mixed8", 8, None, 200_000, "K3_MIX08"),
    ("mixed16", 16, None, 200_000, "K3_MIX16"),
)
CONV_STREAMS = (
    ("conv2", 2, 1_000_000, "K3_CNV02"),
    ("conv4", 4, 1_000_000, "K3_CNV04"),
    ("conv8", 8, 1_000_000, "K3_CNV08"),
    ("conv16", 16, 1_000_000, "K3_CNV16"),
)


def precisions(L):
    """The targeted precisions of width L: the class precisions and every limb
    boundary 64k - 1, 64k, 64k + 1 (k = 1 ... L), within [2, 64L]."""
    nb = 64 * L
    ps = set(p for p in CLASS_PRECISIONS if p <= nb)
    for k in range(1, L + 1):
        for d in (-1, 0, 1):
            if 2 <= 64 * k + d <= nb:
                ps.add(64 * k + d)
    return sorted(ps)


def mixed_choices(L):
    """The weighted half of a mixed stream's precisions (mirrored in Rust)."""
    nb = 64 * L
    out = [p for p in CLASS_PRECISIONS if p <= nb]
    for k in range(1, L + 1):
        for d in (-1, 0, 1):
            if 2 <= 64 * k + d <= nb:
                out.append(64 * k + d)
    return out


# ----------------------------------------------------------------------------
# The value model and the oracle
# ----------------------------------------------------------------------------
class W:
    """(-1)^neg * sig * 2^(exp - (64L - 1)), sig normalized or zero."""

    __slots__ = ("L", "neg", "exp", "sig")

    def __init__(self, L, neg, exp, sig):
        nb = 64 * L
        assert sig == 0 or (1 << (nb - 1)) <= sig < (1 << nb), hex(sig)
        self.L = L
        self.neg = bool(neg)
        self.exp = 0 if sig == 0 else exp
        self.sig = sig

    @staticmethod
    def zero(L, neg=False):
        return W(L, neg, 0, 0)

    @staticmethod
    def from_int(L, neg, m, lsb):
        """Exact value (-1)^neg * m * 2^lsb (trailing zeros of m dropped when
        m is wider than 64L bits)."""
        if m == 0:
            return W.zero(L, neg)
        nb = 64 * L
        if m.bit_length() > nb:
            tz = (m & -m).bit_length() - 1
            m >>= tz
            lsb += tz
        bl = m.bit_length()
        assert bl <= nb
        return W(L, neg, lsb + bl - 1, m << (nb - bl))

    def unit(self):
        return self.exp - (64 * self.L - 1)

    def frac(self):
        u = self.unit()
        v = Fraction(self.sig << u) if u >= 0 else Fraction(self.sig, 1 << -u)
        return -v if self.neg else v

    def negate(self):
        return W(self.L, not self.neg, self.exp, self.sig)

    def token(self):
        if self.sig == 0:
            return "Z-" if self.neg else "Z+"
        digits = ("%0*x" % (16 * self.L, self.sig)).rstrip("0")
        return "%s%sp%d" % ("-" if self.neg else "+", digits, self.exp)

    def enc(self):
        out = bytes([1 if self.neg else 0]) + self.exp.to_bytes(8, "little", signed=True)
        s = self.sig
        for _ in range(self.L):
            out += (s & MASK64).to_bytes(8, "little")
            s >>= 64
        return out

    def fits(self, p):
        nb = 64 * self.L
        return p >= nb or self.sig & ((1 << (nb - p)) - 1) == 0


def floor_log2(a):
    n, d = a.numerator, a.denominator
    e = n.bit_length() - d.bit_length()
    if e >= 0:
        if n < (d << e):
            e -= 1
    elif (n << -e) < d:
        e -= 1
    return e


def rnd(x, p, L):
    """Round a nonzero Fraction once to p bits at width L (nearest, ties to
    even)."""
    assert x != 0 and 2 <= p <= 64 * L
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
    return W(L, neg, e, q << (64 * L - p))


def round_int(L, neg, r, sticky, scale, p):
    """Round (r + f) * 2^scale, f in (0, 1) iff sticky, to p bits at width L."""
    bl = r.bit_length()
    drop = bl - p
    if drop <= 0:
        assert not sticky
        return W.from_int(L, neg, r, scale)
    q = r >> drop
    rbit = (r >> (drop - 1)) & 1
    rest = sticky or (r & ((1 << (drop - 1)) - 1)) != 0
    if rbit and (rest or q & 1):
        q += 1
    e = scale + bl - 1
    if q == 1 << p:
        q >>= 1
        e += 1
    return W(L, neg, e, q << (64 * L - p))


def o_add(a, b, p):
    L = a.L
    if a.sig == 0 and b.sig == 0:
        return W.zero(L, a.neg and b.neg)
    x = a.frac() + b.frac()
    if x == 0:
        return W.zero(L, False)
    return rnd(x, p, L)


def o_sub(a, b, p):
    return o_add(a, b.negate(), p)


def o_mul(a, b, p):
    if a.sig == 0 or b.sig == 0:
        return W.zero(a.L, a.neg != b.neg)
    return rnd(a.frac() * b.frac(), p, a.L)


def o_div(a, b, p):
    if b.sig == 0:
        return "E:div0"
    if a.sig == 0:
        return W.zero(a.L, a.neg != b.neg)
    return rnd(a.frac() / b.frac(), p, a.L)


def o_sqrt(a, p):
    if a.sig == 0:
        return a
    if a.neg:
        return "E:neg_sqrt"
    # a = sig * 2^e0. N = sig * 2^t with t >= 0, t + e0 even and N wide
    # enough that isqrt(N) has at least p + 2 bits.
    L = a.L
    e0 = a.unit()
    t = max(0, 2 * p + 4 - 64 * L)
    if (t + e0) % 2:
        t += 1
    n = a.sig << t
    r = math.isqrt(n)
    return round_int(L, False, r, r * r != n, (e0 - t) // 2, p)


OPS = {"add": o_add, "sub": o_sub, "mul": o_mul, "div": o_div}
OP_NAMES = ("add", "sub", "mul", "div", "sqrt")


def apply(op, a, b, p):
    if op == "sqrt":
        return o_sqrt(a, p)
    return OPS[op](a, b, p)


def tok(v):
    return v if isinstance(v, str) else v.token()


# ----------------------------------------------------------------------------
# The binary64 conversion oracle
# ----------------------------------------------------------------------------
def b64_bits(f):
    return struct.unpack("<Q", struct.pack("<d", f))[0]


def round_up_b64(fr):
    """The smallest binary64 value >= the positive Fraction fr."""
    f = float(fr)
    if Fraction(f) < fr:
        f = math.nextafter(f, math.inf)
    return b64_bits(f)


def conv_oracle(w):
    """(code, bits, rel): code 0 normal, 1 subnormal, 2 underflow, 3 overflow."""
    sign = (1 << 63) if w.neg else 0
    if w.sig == 0:
        return (0, sign, 0)
    if w.exp > 1023:
        return (3, sign, 0)
    if w.exp < -1076:  # below 2^-1075: rounds to zero
        return (2, sign, 0)
    x = abs(w.frac())
    e = w.exp
    q = max(e - 52, -1074)
    num, den = x.numerator, x.denominator
    if q >= 0:
        den <<= q
    else:
        num <<= -q
    k, r = divmod(num, den)
    if 2 * r > den or (2 * r == den and k & 1):
        k += 1
    if k == 1 << 53:
        k = 1 << 52
        q += 1
    if k == 0:
        res = (2, sign, 0)
    elif k >= 1 << 52:
        lead = q + 52
        if lead > 1023:
            res = (3, sign, 0)
        else:
            res = (0, sign | ((lead + 1023) << 52) | (k - (1 << 52)), 0)
    else:
        assert q == -1074
        res = (1, sign | k, round_up_b64(Fraction(1, 2 * k)))
    # Independent cross-check: CPython's correctly rounded int / int.
    try:
        f = float(w.frac())
    except OverflowError:
        assert res[0] == 3, (w.token(), res)
        return res
    if res[0] == 2:
        assert f == 0.0 and math.copysign(1.0, f) == (-1.0 if w.neg else 1.0)
    else:
        assert res[0] in (0, 1) and b64_bits(f) == res[1], (w.token(), res, f)
    return res


def conv_token(res):
    code, bits, rel = res
    neg = bits >> 63
    if code == 0:
        return "N:%016x" % bits
    if code == 1:
        return "S:%016x:%016x" % (bits, rel)
    return ("U" if code == 2 else "O") + ("-" if neg else "+")


def conv_enc(res):
    code, bits, rel = res
    return bytes([code]) + bits.to_bytes(8, "little") + rel.to_bytes(8, "little")


# ----------------------------------------------------------------------------
# SplitMix64 and the stream operand rules (mirrored in the Rust test)
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


def rand_bits(rng, L):
    """64L random bits, the first draw the top limb."""
    v = 0
    for _ in range(L):
        v = (v << 64) | rng.next()
    return v


def rand_sig(rng, L):
    nb = 64 * L
    top = 1 << (nb - 1)
    sig = rand_bits(rng, L) | top
    sel = rng.next()
    kind = sel % 8
    if kind == 0:  # short significand
        nbits = 1 + (sel >> 8) % nb
        sig = (sig >> (nb - nbits)) << (nb - nbits)
    elif kind == 1:  # leading run of ones (carry-out)
        w = 1 + (sel >> 8) % nb
        sig |= ((1 << w) - 1) << (nb - w)
    elif kind == 2:  # short prefix and one far low bit (sticky)
        nbits = 1 + (sel >> 8) % (nb - 1)
        low = (sel >> 24) % (nb - nbits)
        sig = ((sig >> (nb - nbits)) << (nb - nbits)) | (1 << low)
    elif kind == 3:  # ones down to a limb boundary, then zeros
        k = 1 + (sel >> 8) % L
        w = min(nb, max(1, 64 * k + (sel >> 16) % 3 - 1))
        sig = ((1 << w) - 1) << (nb - w)
    return sig


def gen_operands(rng, L):
    nb = 64 * L
    top = 1 << (nb - 1)
    full = (1 << nb) - 1
    r = rng.next()
    op = r % 5
    mode = (r >> 3) % 8
    a_neg = (r >> 6) & 1
    b_neg = (r >> 7) & 1
    ea = (r >> 8) % 4001 - 2000
    zero_a = (r >> 20) % 64 == 0
    zero_b = (r >> 26) % 64 == 0
    a_sig = rand_sig(rng, L)
    if mode <= 3:  # independent, exponents within 150
        eb = ea + rng.next() % 301 - 150
        b_sig = rand_sig(rng, L)
    elif mode == 4:  # within 15 units of the last place
        delta = rng.next() % 16
        b_sig = a_sig + delta if a_sig + delta <= full else a_sig - delta
        eb = ea
    elif mode == 5:  # the low w bits differ
        w = rng.next() % (nb + 1)
        x = rand_bits(rng, L)
        b_sig = (a_sig ^ (x & ((1 << w) - 1))) | top
        eb = ea
    elif mode == 6:  # straddling a power of two
        w = rng.next() % nb
        x = rand_bits(rng, L)
        a_sig = full ^ (x & ((1 << w) - 1))
        w2 = rng.next() % nb
        y = rand_bits(rng, L)
        b_sig = top | (y & ((1 << w2) - 1))
        eb = ea + 1
    else:  # exponent gap up to beyond the 2L-limb window
        span = nb + 400
        eb = ea + rng.next() % (2 * span + 1) - span
        b_sig = rand_sig(rng, L)
    if op == 4:
        a_neg = 0
    a = W.zero(L, a_neg) if zero_a else W(L, a_neg, ea, a_sig)
    b = W.zero(L, b_neg) if zero_b else W(L, b_neg, eb, b_sig)
    return op, a, b


def mixed_p(rng, L):
    r = rng.next()
    if r % 2 == 0:
        return 2 + (r >> 1) % (64 * L - 1)
    choices = mixed_choices(L)
    return choices[(r >> 1) % len(choices)]


def run_stream(name, L, fixed_p, seed, count, sample_out):
    rng = SplitMix64(seed)
    total = hashlib.sha256()
    chunk = hashlib.sha256()
    chunks = []
    body_len = 9 + 8 * L
    for i in range(count):
        p = mixed_p(rng, L) if fixed_p is None else fixed_p
        op, a, b = gen_operands(rng, L)
        res = apply(OP_NAMES[op], a, b, p)
        if isinstance(res, str):
            assert res == "E:div0"
            status, body = 1, bytes(body_len)
        else:
            status, body = 0, res.enc()
        head = bytes([op]) + (p.to_bytes(2, "little") if fixed_p is None else b"") + bytes([status])
        rec = head + body
        total.update(rec)
        chunk.update(rec)
        if i < DIFF_SAMPLE:
            sample_out.append("%s %d %s %d %s %s %s" % (name, i, OP_NAMES[op], p, a.token(), b.token(), tok(res)))
        if (i + 1) % DIFF_CHUNK == 0:
            chunks.append(chunk.hexdigest())
            chunk = hashlib.sha256()
    assert count % DIFF_CHUNK == 0
    return total.hexdigest(), chunks


def conv_sig(rng, L, e):
    """A significand shaped around the binary64 rounding position of a value
    whose leading bit has exponent e."""
    nb = 64 * L
    top = 1 << (nb - 1)
    sig = rand_sig(rng, L)
    sel = rng.next()
    kind = sel % 6
    kept = min(53, e + 1075)  # bits kept by the conversion
    rb = nb - 1 - kept  # index of the round bit in the significand
    if 0 <= rb <= nb - 1:
        if kind == 0:  # exact tie
            sig = ((sig >> (rb + 1)) << (rb + 1)) | (1 << rb)
        elif kind == 1 and rb > 0:  # tie plus one far sticky bit
            sig = ((sig >> (rb + 1)) << (rb + 1)) | (1 << rb) | (1 << ((sel >> 8) % rb))
        elif kind == 2 and rb < nb - 1:  # just below a tie
            sig = ((sig >> (rb + 1)) << (rb + 1)) | ((1 << rb) - 1)
        elif kind == 3:  # all ones: the rounding carries
            sig = (1 << nb) - 1
    assert sig & top
    return sig


def gen_conv_value(rng, L):
    nb = 64 * L
    r = rng.next()
    region = r % 16
    neg = (r >> 4) & 1
    zero = (r >> 5) % 128 == 0
    x = rng.next()
    if region < 5:  # the subnormal band and its edges
        e = -1076 + x % 57  # [-1076, -1020]
    elif region < 8:  # the overflow edge
        e = 1018 + x % 7  # [1018, 1024]
    elif region < 10:  # far below the subnormal range
        e = -1076 - x % (nb + 64)
    elif region == 10:  # near the exponent limits
        e = (1 << 62) - x % 1000 if x & 1 else -(1 << 62) + (x >> 1) % 1000
    else:  # the general range
        e = -1100 + x % 2201
    sig = conv_sig(rng, L, e)
    if zero:
        return W.zero(L, neg)
    return W(L, neg, e, sig)


def run_conv_stream(name, L, seed, count, sample_out):
    rng = SplitMix64(seed)
    total = hashlib.sha256()
    chunk = hashlib.sha256()
    chunks = []
    codes = [0, 0, 0, 0]
    for i in range(count):
        w = gen_conv_value(rng, L)
        res = conv_oracle(w)
        codes[res[0]] += 1
        rec = conv_enc(res)
        total.update(rec)
        chunk.update(rec)
        if i < DIFF_SAMPLE:
            sample_out.append("%s %d %s %s" % (name, i, w.token(), conv_token(res)))
        if (i + 1) % DIFF_CHUNK == 0:
            chunks.append(chunk.hexdigest())
            chunk = hashlib.sha256()
    assert count % DIFF_CHUNK == 0
    return total.hexdigest(), chunks, codes


# ----------------------------------------------------------------------------
# Targeted hard classes (per width)
# ----------------------------------------------------------------------------
def rand_odd(R, bits):
    if bits == 1:
        return 1
    return (1 << (bits - 1)) | R.getrandbits(bits - 1) | 1


def targeted(L):
    R = random.Random(TARGET_SEED * 100 + L)
    nb = 64 * L
    top = 1 << (nb - 1)
    full = (1 << nb) - 1
    lines = []

    def Wi(neg, m, lsb):
        return W.from_int(L, neg, m, lsb)

    def emit(tag, op, p, a, b):
        res = apply(op, a, b, p)
        lines.append("%s %s %d %s %s %s" % (tag, op, p, a.token(), "-" if b is None else b.token(), tok(res)))

    for p in precisions(L):
        # --- exact ties to even at p (the round bit at a limb boundary when
        # p = 64k), with both parities of the kept part
        for _ in range(3):
            x = rand_odd(R, p + 1)  # p + 1 bits ending in 1: a tie at p
            y = rand_odd(R, R.randint(2, min(60, p)))
            s = R.randint(-60, 60)
            neg = R.random() < 0.5
            emit("tie", "add", p, Wi(neg, x - y, s), Wi(neg, y, s))
            xy = x + y
            if (xy >> ((xy & -xy).bit_length() - 1)).bit_length() <= nb:
                emit("tie", "sub", p, Wi(neg, xy, s), Wi(neg, y, s))
        found = 0
        while found < 3:
            l1 = R.randint(max(2, p + 1 - (nb - 1)), min(nb - 1, p))
            l2 = p + 1 - l1 + R.randint(0, 1)
            if l2 < 1 or l2 > nb:
                continue
            f1, f2 = rand_odd(R, l1), rand_odd(R, l2)
            if (f1 * f2).bit_length() != p + 1:
                continue
            emit("tie", "mul", p, Wi(R.random() < 0.5, f1, R.randint(-40, 40)), Wi(R.random() < 0.5, f2, R.randint(-40, 40)))
            found += 1
        if p + 1 < nb:
            for _ in range(2):
                x = rand_odd(R, p + 1)
                bb = rand_odd(R, R.randint(1, nb - (p + 1)))
                emit("tie", "div", p, Wi(False, bb * x, R.randint(-30, 30)), Wi(R.random() < 0.5, bb, R.randint(-30, 30)))
        if 2 * (p + 1) <= nb:
            for _ in range(2):
                x = rand_odd(R, p + 1)
                emit("tie", "sqrt", p, Wi(False, x * x, 2 * R.randint(-30, 30)), None)

        # --- carry-out and renormalization, including a carry through every
        # limb: all ones plus one unit (exact) and all ones at p plus half an
        # ulp (a tie on an odd kept part)
        s = R.randint(-50, 50)
        emit("carry", "add", p, Wi(False, full, s), Wi(False, 1, s))
        emit("carry", "add", p, Wi(False, (1 << p) - 1, s), Wi(False, 1, s - 1))
        emit("carry", "add", p, Wi(True, (1 << p) - 1, s), Wi(True, 3, s - 2))
        emit("carry", "add", p, Wi(False, full, s), Wi(False, full, s))
        m = top | R.getrandbits(nb - 1)
        emit("carry", "add", p, W(L, False, s, m), W(L, False, s, m))
        emit("carry", "sub", p, Wi(False, 1, s + nb), Wi(False, 1, s))
        emit("carry", "sub", p, Wi(False, 1, s + nb + 1), Wi(False, 3, s))
        emit("carry", "mul", p, Wi(False, full, 3), Wi(True, full, -9))
        emit("carry", "mul", p, Wi(False, (1 << 64) - 1, 0), Wi(False, (1 << 64) + 1, 0))
        if p < nb:
            emit("carry", "mul", p, Wi(False, (1 << p) - 1, 0), Wi(False, (1 << p) + 1, 0))
        emit("carry", "div", p, Wi(False, full, 0), Wi(False, 1, nb))
        emit("carry", "div", p, Wi(True, full, 7), Wi(False, full - 1, 7))
        emit("carry", "sqrt", p, Wi(False, full, 0), None)
        emit("carry", "sqrt", p, Wi(False, full, 1), None)

        # --- massive cancellation
        m = top | R.getrandbits(nb - 1)
        if m == full:
            m -= 1
        e = R.randint(-300, 300)
        emit("cancel", "sub", p, W(L, False, e, m), W(L, False, e, m + 1))
        emit("cancel", "add", p, W(L, True, e, m), W(L, False, e, m + 1))
        emit("cancel", "sub", p, W(L, False, e, m), W(L, False, e, m))
        mp = (m >> (nb - p)) << (nb - p)
        if mp + (1 << (nb - p)) <= full:
            emit("cancel", "sub", p, W(L, False, e, mp), W(L, False, e, mp + (1 << (nb - p))))
        pp = R.choice([q for q in (53, p - 1, p, nb - 1, nb) if 2 <= q <= nb])
        emit("cancel", "sub", p, W(L, False, e, top), Wi(False, (1 << pp) - 1, e - pp))
        emit("cancel", "mul", p, W(L, False, e, m), W(L, False, -e, full))
        emit("cancel", "div", p, W(L, False, e, m), W(L, False, e, m + 1))
        emit("cancel", "sqrt", p, W(L, False, 1, top | 1), None)

        # --- exact and near-exact division and square root
        for _ in range(2):
            lb = R.randint(1, nb - 2)
            bb = rand_odd(R, lb)
            q = rand_odd(R, R.randint(1, nb - lb))
            a = bb * q
            emit("exactdiv", "div", p, Wi(False, a, R.randint(-20, 20)), Wi(True, bb, R.randint(-20, 20)))
            for d in (-1, 1):
                if 0 < a + d and (a + d).bit_length() <= nb:
                    emit("nearexactdiv", "div", p, Wi(False, a + d, 0), Wi(False, bb, 0))
            x = R.getrandbits(nb // 2) | (1 << (nb // 2 - 1)) | 1
            sq = x * x
            e2 = 2 * R.randint(-40, 40)
            emit("perfectsq", "sqrt", p, Wi(False, sq, e2), None)
            w = Wi(False, sq, e2)
            emit("nearsq", "sqrt", p, W(L, False, w.exp, w.sig + 1) if w.sig < full else w, None)
            emit("nearsq", "sqrt", p, W(L, False, w.exp, w.sig - 1) if w.sig > top else W(L, False, w.exp - 1, full), None)
            wp = W(L, False, w.exp, (w.sig >> (nb - p)) << (nb - p))
            if wp.sig + (1 << (nb - p)) <= full:
                emit("nearsq", "sqrt", p, W(L, False, wp.exp, wp.sig + (1 << (nb - p))), None)

        # --- sticky-bit paths: a tie whose direction only a far bit decides
        if p < nb:
            x = rand_odd(R, p + 1)  # a tie at p on its own
            s = R.randint(-50, 50)
            for far in sorted(set([p + 70, nb + 100])):
                for sign in (False, True):
                    emit("sticky", "add", p, Wi(False, x, s), Wi(sign, 1, s - far))
                    emit("sticky", "sub", p, Wi(True, x, s), Wi(sign, 1, s - far))
        # half an ulp plus a far tail inside the second operand (gap > 64L)
        m = (top | R.getrandbits(nb - 1)) >> (nb - p) << (nb - p)
        e = R.randint(-50, 50)
        tail = Wi(False, top | 1, e - p - (nb - 1))
        emit("sticky", "add", p, W(L, False, e, m), tail)
        emit("sticky", "sub", p, W(L, False, e, m), tail)
        # a single operand whose only tail bit is in limb 0, far below the
        # round bit: rounded through multiplication by one
        if p < nb - 1:
            x = rand_odd(R, p + 1) & ~2  # a tie with an even kept part
            sh = nb - (p + 1)
            base = x << sh
            one = W(L, False, 0, top)
            emit("sticky", "mul", p, W(L, False, 3, base), one)
            emit("sticky", "mul", p, W(L, False, 3, base | 1), one)
        # multiplication: sparse factors put the round bit and a lone low bit
        found = 0
        tries = 0
        while found < 3 and tries < 200000:
            tries += 1
            l1 = R.randint(2, nb)
            l2 = R.randint(2, nb)
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
                emit("sticky", "mul", p, Wi(False, f1, 0), Wi(False, f2, 0))
                found += 1
        assert found == 3, (L, p, found)
        # division: the (64L + 2)-bit quotient has its round bit set and a zero
        # tail, and the remainder is nonzero: the sticky bit alone rounds up
        found = 0
        while found < 2:
            bb = top | R.getrandbits(nb - 1) | 1
            Q = (1 << (nb + 1)) | (R.getrandbits(p - 1) << (nb + 2 - p)) | (1 << (nb + 1 - p))
            Q &= ~(1 << (nb + 2 - p))
            num = bb * Q
            rem = (-num) % (1 << (nb + 1))
            if rem == 0 or rem >= bb:
                continue
            a = (num + rem) >> (nb + 1)
            if not top <= a <= full:
                continue
            emit("sticky", "div", p, Wi(False, a, 0), Wi(False, bb, 0))
            found += 1
        # square root: x^2 + 1 with x of p + 1 bits (a tie without the sticky
        # bit), else the square of a tie-shaped root cut to 64L bits
        for _ in range(2):
            x = rand_odd(R, p + 1) & ~2
            sq = x * x
            if sq.bit_length() < nb:
                emit("sticky", "sqrt", p, Wi(False, sq + 1, 0), None)
            else:
                bl = sq.bit_length()
                m2 = (sq >> (bl - nb)) + 1
                if m2 <= full:
                    emit("sticky", "sqrt", p, Wi(False, m2, bl - nb), None)

    # --- far subtractions whose (1 - f) tail decides a tie (the independent
    # review RV12's N1): an odd full-width big minus an all-ones small at gap
    # 64L + 1 lies just above the midpoint below big, so at p = 64L only the
    # tail left after the borrow keeps the result at big. Gap 64L + 2 and
    # p = 64L - 1 are neighbours. Their own generator, appended last, so every
    # vector above is unchanged.
    T = random.Random(TARGET_SEED * 100 + L + 50)
    for p in (nb, nb - 1):
        for _ in range(4):
            n = T.getrandbits(nb) | top | 1
            e = T.randint(-100, 100)
            neg = T.random() < 0.5
            big = W(L, neg, e, n)
            for gap in (nb + 1, nb + 2):
                small = W(L, not neg, e - gap, full)
                emit("taildecides", "add", p, big, small)
                emit("taildecides", "sub", p, big, small.negate())
    return lines


# ----------------------------------------------------------------------------
# The conversion's boundary classes
# ----------------------------------------------------------------------------
def conversion_lines():
    R = random.Random(TARGET_SEED + 7)
    lines = []
    for L in CONV_WIDTHS:
        nb = 64 * L
        top = 1 << (nb - 1)
        full = (1 << nb) - 1

        def emit(tag, w):
            lines.append("conv %d %s %s %s" % (L, tag, w.token(), conv_token(conv_oracle(w))))

        def Wi(neg, m, lsb):
            return W.from_int(L, neg, m, lsb)

        for neg in (False, True):
            emit("zero", W.zero(L, neg))
            emit("min_subnormal", Wi(neg, 1, -1074))
            emit("max_subnormal", Wi(neg, (1 << 52) - 1, -1074))
            emit("min_normal", Wi(neg, 1, -1022))
            emit("max", Wi(neg, (1 << 53) - 1, 971))
            # [2^-1022 - 2^-1075, 2^-1022): rounds up into the normal range
            emit("up_to_normal", Wi(neg, (1 << 53) - 1, -1075))  # the tie
            emit("up_to_normal", Wi(neg, ((1 << 53) - 1) * 2 + 1, -1076))
            emit("up_to_normal", Wi(neg, (((1 << 53) - 1) << (nb - 54)) | 1, -1075 - (nb - 54)))
            emit("below_up_to_normal", Wi(neg, (((1 << 53) - 1) << (nb - 54)) - 1, -1075 - (nb - 54)))
            # exactly 2^-1075: a tie that rounds to zero, so it underflows
            emit("half_min_subnormal", Wi(neg, 1, -1075))
            # 2^-1075 plus the smallest tail this width carries
            emit("half_min_plus_tail", Wi(neg, top | 1, -1075 - (nb - 1)))
            emit("half_min_minus_tail", Wi(neg, full, -1076 - (nb - 1)))
            # subnormal ties with a sticky bit far below (hundreds of binades
            # at L = 16), on even and odd kept parts: never double-rounded
            for k in (2, 3, (1 << 40) + 6, (1 << 51) + 1):
                emit("subnormal_tie", Wi(neg, 2 * k + 1, -1075))
                far = nb - 60
                if far > 0:
                    emit("subnormal_tie_far_sticky", Wi(neg, ((2 * k + 1) << far) | 1, -1075 - far))
                    emit("subnormal_tie_far_below", Wi(neg, ((2 * k + 1) << far) - 1, -1075 - far))
            # a value that 53-bit rounding would turn into a subnormal tie
            if nb > 120:
                emit("double_rounding_trap", Wi(neg, (1 << 60) | 1, -1075 - 60))
                k = (1 << 40) + 6  # even: 53-bit rounding first would tie down
                emit("double_rounding_trap", Wi(neg, ((2 * k + 1) << 70) | 1, -1075 - 70))
            # the overflow edge: MAX, just below the midpoint, the midpoint
            mid = ((1 << 54) - 1)  # (2^54 - 1) * 2^970 = 2^1024 - 2^970
            emit("below_midpoint", Wi(neg, (mid << (nb - 54)) - 1, 970 - (nb - 54)))
            emit("midpoint", Wi(neg, mid, 970))
            emit("above_midpoint", Wi(neg, (mid << (nb - 54)) | 1, 970 - (nb - 54)))
            emit("overflow", Wi(neg, 1, 1024))
            # exponents near the limits: overflow or underflow, never a wrap
            limit = 1 << 62
            emit("exponent_limit", W(L, neg, limit, full))
            emit("exponent_limit", W(L, neg, -limit, top))
            emit("exponent_limit", W(L, neg, limit - 1, top | 1))
            emit("exponent_limit", W(L, neg, -limit + 1, full))
            # normal values: ties, carries, far sticky bits
            for _ in range(4):
                e = R.randint(-1022, 1022)
                m = top | R.getrandbits(nb - 1)
                emit("normal", W(L, neg, e, m))
                tie = ((m >> (nb - 54)) | 1) << (nb - 54)
                emit("normal_tie", W(L, neg, e, tie))
                if nb > 54:
                    emit("normal_tie_far_sticky", W(L, neg, e, tie | 1))
                emit("normal_carry", W(L, neg, e, full))
    return lines


# ----------------------------------------------------------------------------
# TwoSum, TwoProduct, narrowing and the integer constructor
# ----------------------------------------------------------------------------
def eft_lines():
    R = random.Random(TARGET_SEED + 11)
    lines = []
    for L in WIDTHS:
        nb = 64 * L
        ps = sorted(set([q for q in CLASS_PRECISIONS if q <= nb] + [nb]))
        for p in ps:

            def at_p(e_lo, e_hi):
                m = R.getrandbits(nb) | 1
                return rnd(Fraction(m) * Fraction(2) ** R.randint(e_lo, e_hi), p, L)

            pairs = []
            for _ in range(8):
                a, b = at_p(-400, 400), at_p(-400, 400)
                if R.random() < 0.5:
                    b = b.negate()
                pairs.append((a, b))
            a = at_p(-10, 10)
            pairs.append((a, a))  # exact doubling: e = 0
            pairs.append((a, a.negate()))  # exact cancellation
            pairs.append((a, rnd(a.frac() * Fraction(1, 1 << (p + 40)), p, L)))  # far below
            b = rnd(a.frac() * (1 + Fraction(1, 1 << (p - 1))), p, L)
            pairs.append((a, b.negate()))  # neighbours: exact difference
            for x, y in pairs:
                for u, v in ((x, y), (y, x)):
                    s = o_add(u, v, p)
                    exact = u.frac() + v.frac() - s.frac()
                    e = W.zero(L) if exact == 0 else rnd(exact, p, L)
                    assert e.frac() == exact
                    lines.append("twosum %d %d %s %s %s %s" % (L, p, u.token(), v.token(), s.token(), e.token()))
            for x, y in pairs:
                s = o_mul(x, y, p)
                exact = x.frac() * y.frac() - s.frac()
                e = W.zero(L) if exact == 0 else rnd(exact, p, L)
                assert e.frac() == exact
                lines.append("twoprod %d %d %s %s %s %s" % (L, p, x.token(), y.token(), s.token(), e.token()))
    # narrowing (and re-rounding, and widening with rounding): a value of
    # width Ls rounded to (Ld, p)
    for ls in CONV_WIDTHS:
        nbs = 64 * ls
        tops = 1 << (nbs - 1)
        for ld in WIDTHS:
            ps = sorted(set([q for q in CLASS_PRECISIONS if q <= 64 * ld] + [64 * ld]))
            for p in ps:
                for kind in range(6):
                    e = R.randint(-500, 500)
                    m = tops | R.getrandbits(nbs - 1)
                    if p < nbs:
                        rb = nbs - 1 - p
                        if kind == 0:  # exact tie
                            m = ((m >> (rb + 1)) << (rb + 1)) | (1 << rb)
                        elif kind == 1 and rb > 0:  # tie and a far sticky bit
                            m = ((m >> (rb + 1)) << (rb + 1)) | (1 << rb) | 1
                        elif kind == 2:  # all ones: carry
                            m = (1 << nbs) - 1
                    x = W(ls, R.random() < 0.5, e, m)
                    res = rnd(x.frac(), p, ld)
                    lines.append("narrow %d %d %d %s %s" % (ls, ld, p, x.token(), res.token()))
    # the integer constructor: +-magnitude * 2^exponent rounded once
    for L in WIDTHS:
        nb = 64 * L
        ps = sorted(set([q for q in CLASS_PRECISIONS if q <= nb] + [nb]))
        for p in ps:
            cases = []
            # ExactAccumulator's shape: 68 limbs, quantum 2^-2148
            for kind in range(6):
                hb = R.randint(nb + 10, 68 * 64 - 1)  # leading bit index
                m = (1 << hb) | R.getrandbits(hb)
                rb = hb - p  # round bit index
                if kind == 0:  # exact tie
                    m = ((m >> (rb + 1)) << (rb + 1)) | (1 << rb)
                elif kind == 1:  # tie plus a sticky bit in limb 0
                    m = ((m >> (rb + 1)) << (rb + 1)) | (1 << rb) | 1
                elif kind == 2:  # just below the tie: all ones below the round bit
                    m = ((m >> (rb + 1)) << (rb + 1)) | ((1 << rb) - 1)
                elif kind == 3:  # all ones: carry
                    m = (1 << (hb + 1)) - 1
                cases.append((R.random() < 0.5, m, -2148, 68))
            cases.append((False, R.getrandbits(40) | 1, -2148, 68))  # exact, short
            cases.append((True, 0, -2148, 68))  # zero: +0 whatever the flag
            cases.append((False, R.getrandbits(3 * 64) | (1 << 191), R.randint(-300, 300), 3))
            for neg, m, ex, limbs in cases:
                if m == 0:
                    res = W.zero(L, False)
                else:
                    v = Fraction(m) * Fraction(2) ** ex
                    res = rnd(-v if neg else v, p, L)
                lines.append("int %d %d %d %d %d %x %s" % (L, p, 1 if neg else 0, ex, limbs, m, res.token()))
    return lines


# ----------------------------------------------------------------------------
def build():
    summary = {}
    files = {}
    for L in WIDTHS:
        t = targeted(L)
        files["targeted_l%d.txt" % L] = "\n".join(t) + "\n"
        summary["targeted_l%d_lines" % L] = len(t)
    c = conversion_lines()
    files["conversion.txt"] = "\n".join(c) + "\n"
    summary["conversion_lines"] = len(c)
    e = eft_lines()
    files["eft.txt"] = "\n".join(e) + "\n"
    summary["eft_lines"] = len(e)
    sample = []
    manifest = []
    for name, L, p, count, tag in STREAMS:
        seed = seed_of(tag)
        h, chunks = run_stream(name, L, p, seed, count, sample)
        manifest.append("stream %s width %d precision %s seed %016x count %d chunk %d sha256 %s" % (name, L, "mixed" if p is None else p, seed, count, DIFF_CHUNK, h))
        manifest += ["chunk %s %d %s" % (name, i, d) for i, d in enumerate(chunks)]
        summary["stream_%s_sha256" % name] = h
    for name, L, count, tag in CONV_STREAMS:
        seed = seed_of(tag)
        h, chunks, codes = run_conv_stream(name, L, seed, count, sample)
        manifest.append("conversion %s width %d seed %016x count %d chunk %d sha256 %s" % (name, L, seed, count, DIFF_CHUNK, h))
        manifest += ["chunk %s %d %s" % (name, i, d) for i, d in enumerate(chunks)]
        summary["conversion_%s_sha256" % name] = h
        summary["conversion_%s_outcomes_normal_subnormal_underflow_overflow" % name] = codes
    files["differential_sample.txt"] = "\n".join(sample) + "\n"
    files["differential.txt"] = "\n".join(manifest) + "\n"
    names = ["targeted_l%d.txt" % L for L in WIDTHS] + ["conversion.txt", "eft.txt", "differential_sample.txt", "differential.txt"]
    files["SHA256SUMS"] = "".join("%s  %s\n" % (hashlib.sha256(files[n].encode()).hexdigest(), n) for n in names)
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
