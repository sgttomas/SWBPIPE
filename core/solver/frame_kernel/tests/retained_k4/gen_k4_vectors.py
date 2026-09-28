#!/usr/bin/env python3
"""Test vectors for T3 slice K4 (the W1a kernel method, `structural/retained/`).

Standard library only. None of the expected values comes from the code under
test. It imports, read-only, and records the sha256 of:
- K3's `gen_wide_k3_vectors.py` (the value model `W`, the Fraction rounding
  oracle and the stream operand rules, reused for the N5 streams);
- R1's `references.py` (the exact pi PI_Q and the full model of the RF-MECH
  case whose JSON model is abbreviated), with `references.json` hash-checked;
- D1's `floor_kinds.json` (the enumerated not-covered list of D1 §4.10),
and reads model inputs (binary64 bits) from `kd5_models.rs` (K-D5's D5C-1
controls) and `k2b_models.rs` (K2b's spring-carried case).

Oracles:
- every rounding is the exact value (integers or `fractions.Fraction`) rounded
  once to p bits, to nearest, ties to even; square roots with `math.isqrt`;
- the exact solutions of K4's own controls are exact rational direct-stiffness
  solves of the intended model built from the binary64 inputs;
- R1's cases carry R1's own expected values;
- the §4.1.6.1 formulas are reimplemented with Python floats (IEEE-exact
  + - * / sqrt and max).

Usage:  python3 gen_k4_vectors.py [--check]

With --check nothing is written; the files are regenerated in memory and
compared byte for byte with the committed ones.
"""
import hashlib
import importlib.util
import json
import math
import re
import struct
import sys
from fractions import Fraction as Fr
from pathlib import Path

# The imported inputs stay byte-identical: no __pycache__ beside them.
sys.dont_write_bytecode = True

HERE = Path(__file__).resolve().parent
FK = HERE.parent.parent
P_ROOT = FK.parent.parent.parent
T3 = (P_ROOT / "execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION"
      / "instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3")
K3_GEN = FK / "tests/retained_wide_k3/gen_wide_k3_vectors.py"
R1_PY = T3 / "REFERENCES/references.py"
R1_JSON = T3 / "REFERENCES/references.json"
FLOOR_JSON = T3 / "DESIGN_NUMERICS/_run_records/floor_kinds.json"
KD5_MODELS = P_ROOT / "core/solver/nonlinear_integration/src/structural_adapter/kd5_models.rs"
K2B_MODELS = P_ROOT / "core/solver/nonlinear_integration/src/structural_adapter/k2b_models.rs"
NI_FIXTURES = P_ROOT / "validation/benchmarks/numerical_integrity/fixtures.json"

PINNED = {
    K3_GEN: "ca48d1bd65540eb3da7f30acbbdd4fa553fe03fed432f2b18afd960e2ba4da29",
    R1_PY: "80d473a7351e92a2a903d233b9e633aac794aeff07f3ac41e40d25a0d94fc8a8",
    R1_JSON: "7b176dbbf2296be02d8bca19c698ee56d0d5753751150a9175e0d3ad4cf89cc9",
    FLOOR_JSON: "8326561530598e1f6b69d9f174b70946e52800373b5ecab48864ae174087ae78",
    KD5_MODELS: "17a4680c83ff3e940a7dcfd908ef334c8b5a53427b6ff388dafb5888477dca23",
    K2B_MODELS: "c23eedd273ebacd4d3d324bb24e72cbe766b33d2768c8808e5f9d51674c63090",
    NI_FIXTURES: "c061d73481d2ad137f7cef988475721681131789ec222e3e93c5ed3836b2910e",
}


def sha256_file(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


for _path, _digest in PINNED.items():
    assert sha256_file(_path) == _digest, "pinned input changed: %s" % _path


def load_module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


k3 = load_module("gen_wide_k3_vectors", K3_GEN)
W = k3.W
MASK64 = (1 << 64) - 1

# The K4 precisions and their widths (K3 ruling 8).
K4_PRECISIONS = ((128, 4), (192, 4), (256, 4), (320, 8), (512, 8), (576, 16), (1024, 16))


def seed_of(tag):
    assert len(tag) == 8
    return int.from_bytes(tag.encode(), "big")


def b64(x):
    return struct.unpack("<Q", struct.pack("<d", float(x)))[0]


def f64_of(bits):
    return struct.unpack("<d", struct.pack("<Q", bits))[0]


def hexf(x):
    return "%016x" % b64(x)


# ----------------------------------------------------------------------------
# Exact values as (m, e): m * 2^e, m a signed integer
# ----------------------------------------------------------------------------
def w_exact(w):
    """(m, e) of a W."""
    if w.sig == 0:
        return (0, 0)
    return (-w.sig if w.neg else w.sig, w.unit())


def f_exact(x):
    """(m, e) of a finite binary64."""
    bits = b64(x)
    neg = bits >> 63
    biased = (bits >> 52) & 0x7FF
    frac = bits & ((1 << 52) - 1)
    if biased == 0:
        m, e = frac, -1074
    else:
        m, e = frac | (1 << 52), biased - 1075
    return (-m if neg else m, e)


def exact_sum(terms):
    """Exact sum of (m, e) terms, as (m, e)."""
    nz = [(m, e) for (m, e) in terms if m != 0]
    if not nz:
        return (0, 0)
    e0 = min(e for _, e in nz)
    return (sum(m << (e - e0) for m, e in nz), e0)


def round_exact(me, p, L):
    """(m, e) rounded once to p bits at width L; +0 for an exact zero."""
    m, e = me
    if m == 0:
        return W.zero(L, False)
    return k3.round_int(L, m < 0, abs(m), False, e, p)


def frac_of(me):
    m, e = me
    return Fr(m) * (Fr(2) ** e)


def tok_int(neg, mag, e):
    return "%s%x@%d" % ("-" if neg else "+", mag, e)


# ----------------------------------------------------------------------------
# B: the multi-term sum
# ----------------------------------------------------------------------------
def w_value(L, neg, e, mag):
    """W of (-1)^neg * mag * 2^(e - bits + 1) with leading-bit exponent e."""
    return W.from_int(L, neg, mag, e - mag.bit_length() + 1)


def pbits(p, top_exp, L, mantissa):
    """A W with exactly the given p-bit mantissa (top bit set) and leading exponent."""
    assert mantissa.bit_length() == p
    return w_value(L, False, top_exp, mantissa)


def term_line(kind, *parts):
    return kind + ":" + "*".join(parts)


def sum_targeted():
    lines = []

    def emit(p, L, terms, expect):
        # terms: list of (token, (m, e)); expect: W or "refuse:<why>"
        toks = " ".join(t for t, _ in terms)
        res = expect if isinstance(expect, str) else expect.token()
        lines.append("sum %d %d %d %s = %s" % (L, p, len(terms), toks, res))

    def W_(L, neg, m, lsb):
        return W.from_int(L, neg, m, lsb)

    def wt(w, negate=False):
        return (("~" if negate else "") + "w:" + w.token(),
                (lambda me: (-me[0], me[1]) if negate else me)(w_exact(w)))

    def ft(x, negate=False):
        me = f_exact(x)
        return (("~" if negate else "") + "f:" + hexf(x), (-me[0], me[1]) if negate else me)

    def xt(a, b, negate=False):
        ma, ea = w_exact(a)
        mb, eb = w_exact(b)
        me = (ma * mb, ea + eb)
        return (("~" if negate else "") + "x:" + a.token() + "*" + b.token(),
                (-me[0], me[1]) if negate else me)

    def it(neg, mag, e, negate=False):
        me = (-mag if neg else mag, e)
        return (("~" if negate else "") + "i:" + tok_int(neg, mag, e),
                (-me[0], me[1]) if negate else me)

    def expect(terms, p, L):
        return round_exact(exact_sum([me for _, me in terms]), p, L)

    precisions = list(K4_PRECISIONS) + [(255, 4), (511, 8), (1023, 16)]
    # RV12's counterexample (p = 128): 1 + 2^-127, 2^-128, -2^-400 -> 1 + 2^-127.
    L = 4
    t = [wt(W_(L, False, (1 << 127) + 1, -127)), wt(W_(L, False, 1, -128)), wt(W_(L, True, 1, -400))]
    e = expect(t, 128, L)
    assert e.frac() == 1 + Fr(1, 1 << 127)
    emit(128, L, t, e)
    for p, L in precisions:
        one = W_(L, False, 1, 0)
        ulp = Fr(1, 1 << (p - 1))
        # Ties decided by a far tail: big (even or odd last bit) + half ulp +- far.
        for odd in (0, 1):
            big = W_(L, False, (1 << (p - 1)) + odd, -(p - 1))
            half = W_(L, False, 1, -p)
            for far_exp in (-p - 5, -p - 64, -p - 300, -p - 2000, -64 * L - 1, -64 * L - 2):
                for fneg in (False, True):
                    far = W_(L, fneg, 1, far_exp)
                    t = [wt(big), wt(half), wt(far)]
                    emit(p, L, t, expect(t, p, L))
            # the exact tie (no tail): ties to even
            t = [wt(big), wt(half)]
            emit(p, L, t, expect(t, p, L))
            # the tail as a sum of many small terms that cancel to the sign
            t = [wt(big), wt(half), wt(W_(L, False, 3, -p - 900)), wt(W_(L, True, 1, -p - 898))]
            emit(p, L, t, expect(t, p, L))
        # Total cancellation -> Z+.
        x = W_(L, True, (1 << p) - 1, -3)
        y = W_(L, False, (1 << (p - 1)) + 5, 40)
        for t in ([wt(x), wt(x, True)], [wt(x), wt(y), wt(x, True), wt(y, True)],
                  [wt(y), ft(-2.5), wt(y, True), ft(2.5)]):
            r = expect(t, p, L)
            assert r.sig == 0 and not r.neg
            emit(p, L, t, r)
        # Mixed signs, gaps beyond the width.
        t = [wt(W_(L, False, 1, 3000)), wt(one), wt(W_(L, True, 1, 3000))]
        emit(p, L, t, expect(t, p, L))
        t = [wt(W_(L, False, (1 << p) - 1, 200)), wt(W_(L, True, 1, 200 - 64 * L - 7)),
             wt(W_(L, False, 1, -900))]
        emit(p, L, t, expect(t, p, L))
        # Carries through every limb: 2^(p)-1 ulps + 1 ulp.
        t = [wt(W_(L, False, (1 << p) - 1, 0)), wt(W_(L, False, 1, 0)), wt(W_(L, True, 1, -3000))]
        emit(p, L, t, expect(t, p, L))
        # Exact products as terms (their low halves decide).
        a = W_(L, False, (1 << p) - 1, 0)
        b = W_(L, False, (1 << p) - 3, -p)
        t = [xt(a, b), xt(a, b, True), wt(W_(L, False, 1, -1500))]
        emit(p, L, t, expect(t, p, L))
        t = [xt(a, b), wt(W_(L, True, 1, 0)), xt(b, b)]
        emit(p, L, t, expect(t, p, L))
        # Binary64 terms and a ledger-like 68-limb integer net.
        net = (1 << 4200) + (1 << 2) + 1
        t = [it(False, net, -2148), ft(1e-300), ft(-1e300), it(True, 3, -2148)]
        emit(p, L, t, expect(t, p, L))
        t = [ft(1e80), ft(1e-8), ft(-1e80)]
        r = expect(t, p, L)
        assert r.frac() == frac_of(f_exact(1e-8))
        emit(p, L, t, r)
        # Scaled terms (a Wide times a small integer and a power of two).
        t = [("s:" + y.token() + "*3@-7", (w_exact(y)[0] * 3, w_exact(y)[1] - 7)), wt(x)]
        emit(p, L, t, expect(t, p, L))
    # The span limit: 8,128 bits accepted, 8,129 refused (never truncated).
    L = 4
    t = [wt(W_(L, False, 1, 0)), wt(W_(L, False, 1, -8127))]
    emit(256, L, t, expect(t, 256, L))
    t = [wt(W_(L, False, 1, 0)), wt(W_(L, False, 1, -8128))]
    emit(256, L, t, "refuse:span")
    t = [wt(W_(L, False, 1, -8128)), wt(W_(L, False, 1, 0))]
    emit(256, L, t, "refuse:span")
    # A 256-bit term (leading bit 2^0, lowest 2^-255): the span runs from its
    # leading bit, not its lowest.
    t = [wt(W_(L, False, (1 << 256) - 1, -255)), wt(W_(L, False, 1, -8127))]
    emit(256, L, t, expect(t, 256, L))
    t = [wt(W_(L, False, (1 << 256) - 1, -255)), wt(W_(L, False, 1, -8128))]
    emit(256, L, t, "refuse:span")
    # The exponent limit: a product whose exponent leaves the Wide range.
    big = W(4, False, (1 << 62) - 10, 1 << 255)
    t = [("x:" + big.token() + "*" + big.token(), None)]
    lines.append("sum 4 128 1 %s = refuse:exponent" % t[0][0])
    return lines


def rand_mantissa(rng, p):
    """A p-bit mantissa (top bit set), with a few structured shapes."""
    L = (p + 63) // 64
    m = k3.rand_bits(rng, L) >> (64 * L - p)
    m |= 1 << (p - 1)
    sel = rng.next()
    kind = sel % 6
    if kind == 0:  # short
        nb = 1 + (sel >> 8) % p
        m = (m >> (p - nb)) << (p - nb)
    elif kind == 1:  # all ones
        m = (1 << p) - 1
    elif kind == 2:  # top and one low bit
        m = (1 << (p - 1)) | (1 << ((sel >> 8) % (p - 1)))
    return m


def gen_sum(rng, L, p):
    """One differential sum: a list of (token, (m, e)) (mirrored in Rust)."""
    r = rng.next()
    n = 2 + r % 63
    base = (r >> 8) % 801 - 400
    terms = []
    for _ in range(n):
        s = rng.next()
        kind = s % 8
        neg = bool((s >> 3) & 1)
        if kind == 4 and terms:
            tk, me = terms[(s >> 4) % len(terms)]
            if tk.startswith("~"):
                terms.append((tk[1:], (-me[0], me[1])))
            else:
                terms.append(("~" + tk, (-me[0], me[1])))
            continue
        if kind <= 4:
            e = base + (s >> 4) % 257 - 128
            if (s >> 16) % 16 == 0:
                e = base + (s >> 20) % 4001 - 2000
            m = rand_mantissa(rng, p)
            w = w_value(L, neg, e, m)
            terms.append(("w:" + w.token(), w_exact(w)))
        elif kind == 5:
            biased = min(2046, max(1, base + 1023 + (s >> 4) % 101 - 50))
            frac = rng.next() & ((1 << 52) - 1)
            bits = (int(neg) << 63) | (biased << 52) | frac
            x = f64_of(bits)
            terms.append(("f:%016x" % bits, f_exact(x)))
        elif kind == 6:
            ea = base // 2 + (s >> 4) % 101 - 50
            eb = base - base // 2 + (s >> 12) % 101 - 50
            a = w_value(L, neg, ea, rand_mantissa(rng, p))
            b = w_value(L, False, eb, rand_mantissa(rng, p))
            (ma, xa), (mb, xb) = w_exact(a), w_exact(b)
            terms.append(("x:" + a.token() + "*" + b.token(), (ma * mb, xa + xb)))
        else:
            limbs = 1 + (s >> 4) % 8
            mag = k3.rand_bits(rng, limbs) | 1
            e = base - 64 * limbs + (s >> 12) % 129 - 64
            terms.append(("i:" + tok_int(neg, mag, e), (-mag if neg else mag, e)))
    return terms


SUM_STREAMS = tuple(("sum%d" % p, L, p, 100_000, "K4S%05d" % p) for p, L in K4_PRECISIONS)
SUM_CHUNK = 10_000
SAMPLE = 1000


def run_sum_stream(name, L, p, seed, count, sample_out):
    rng = k3.SplitMix64(seed)
    total = hashlib.sha256()
    chunk = hashlib.sha256()
    chunks = []
    for i in range(count):
        terms = gen_sum(rng, L, p)
        res = round_exact(exact_sum([me for _, me in terms]), p, L)
        rec = bytes([0]) + res.enc()
        total.update(rec)
        chunk.update(rec)
        if i < SAMPLE:
            # The term list as its sha256 (the full lists would be about 23 MB).
            listing = hashlib.sha256(" ".join(t for t, _ in terms).encode()).hexdigest()
            sample_out.append("%s %d %d %s = %s" % (name, i, len(terms), listing, res.token()))
        if (i + 1) % SUM_CHUNK == 0:
            chunks.append(chunk.hexdigest())
            chunk = hashlib.sha256()
    return total.hexdigest(), chunks


# ----------------------------------------------------------------------------
# C: the ledger and the accessor
# ----------------------------------------------------------------------------
LEDGER_TARGETS = ((53, 4),) + K4_PRECISIONS


def ledger_lines():
    rng = k3.SplitMix64(seed_of("K4LEDGER"))
    cases = []
    # Named: RF-CANCEL's authored orders and V1's check L.
    for g in (1e5, 1e6, 1e7, 1e8):
        for order in ((g, 0.3, -g), (g, -g, 0.3), (0.3, g, -g)):
            cases.append(list(order))
    cases.append([1e80, 1e-8, -1e80])
    cases.append([1e80, -1e80, 1e-8])
    cases.append([1e-8, 1e80, -1e80])
    cases.append([2e-8])
    cases.append([1.0, -1.0])  # exact zero
    cases.append([])
    # Ties at every K4 precision decided by a tail.
    for p, _ in LEDGER_TARGETS:
        cases.append([1.0, 2.0 ** -p, 2.0 ** -(p + 60)])
        cases.append([1.0, 2.0 ** -p, -(2.0 ** -(p + 60))])
        cases.append([1.0 + 2.0 ** -52, 2.0 ** -p])
    # Seeded ledgers (1..12 terms, with cancellation).
    for _ in range(400):
        n = 1 + rng.next() % 12
        values = []
        for _k in range(n):
            s = rng.next()
            if values and s % 4 == 0:
                values.append(-values[(s >> 2) % len(values)])
                continue
            biased = 1 + (s >> 8) % 2046
            if s % 5 == 1:
                biased = 1023 + (s >> 8) % 60 - 30
            bits = ((s >> 1) & 1) << 63 | (biased << 52) | (rng.next() & ((1 << 52) - 1))
            values.append(f64_of(bits))
        cases.append(values)
    lines = []
    for values in cases:
        me = exact_sum([f_exact(v) for v in values])
        m, e = me
        # The accessor's form: magnitude at quantum 2^-2148.
        if m == 0:
            net = "+0@0"
        else:
            mag, ex = abs(m), e
            while mag % 2 == 0:
                mag //= 2
                ex += 1
            net = tok_int(m < 0, mag, ex)
        parts = ["ledger", str(len(values))] + [hexf(v) for v in values] + ["net", net]
        for p, L in LEDGER_TARGETS:
            parts += ["p%d" % p, round_exact(me, p, L).token()]
        # round(): the accumulator's binary64 rounding (nearest even), +0 for zero,
        # "R" when non-representable.
        if m == 0:
            rd = "0000000000000000"
        else:
            try:
                f = float(frac_of(me))
                rd = "%016x" % b64(abs(f) if f == 0 else f)
                if f == 0:
                    rd = "0000000000000000"
            except OverflowError:
                rd = "R"
        parts += ["round", rd]
        lines.append(" ".join(parts))
    return lines


# ----------------------------------------------------------------------------
# D: the N5 streams (K3's operand rules and oracle at K4's working precisions)
# ----------------------------------------------------------------------------
STREAMS = (
    ("p128", 4, 128, 1_000_000, "K4_P0128"),
    ("p192", 4, 192, 1_000_000, "K4_P0192"),
    ("p320", 8, 320, 1_000_000, "K4_P0320"),
    ("p576", 16, 576, 1_000_000, "K4_P0576"),
)


# ----------------------------------------------------------------------------
# E: K4's formation, emulated bit for bit (assemble.rs), and exact rational
# solves of the intended models (the controls' expectations)
# ----------------------------------------------------------------------------
from math import isqrt

r1 = load_module("references", R1_PY)
PI_Q = r1.PI_Q


def rp(x, p):
    """Fraction x rounded once to p bits (nearest, ties to even); 0 -> 0."""
    if x == 0:
        return Fr(0)
    neg = x < 0
    a = -x if neg else x
    e = k3.floor_log2(a)
    sh = p - 1 - e
    n, d = a.numerator, a.denominator
    if sh >= 0:
        num, den = n << sh, d
    else:
        num, den = n, d << (-sh)
    q, r = divmod(num, den)
    if 2 * r > den or (2 * r == den and q & 1):
        q += 1
    v = Fr(q, 1 << sh) if sh >= 0 else Fr(q << (-sh))
    return -v if neg else v


def sqrt_p(x, p):
    """The correctly rounded square root of a Fraction x > 0 at p bits."""
    assert x > 0
    n, d = x.numerator, x.denominator
    k = (p + 4) - (n.bit_length() - d.bit_length()) // 2
    num = n << (2 * k) if k >= 0 else n
    den = d if k >= 0 else d << (-2 * k)
    t = num // den
    s = isqrt(t)
    exact = s * s == t and t * den == num
    val = Fr(2 * s + (0 if exact else 1), 2)
    val = val / (Fr(2) ** k)
    return rp(val, p)


def W_of(x, L):
    """The W holding the Fraction x (which must fit 64L bits exactly)."""
    if x == 0:
        return W.zero(L, False)
    neg = x < 0
    a = -x if neg else x
    n, d = a.numerator, a.denominator
    assert d & (d - 1) == 0
    e = -(d.bit_length() - 1)
    while n % 2 == 0 and n:
        n //= 2
        e += 1
    return W.from_int(L, neg, n, e)


def width_of(p):
    return 4 if p <= 256 else (8 if p <= 512 else 16)


UPPER = [(a, b) for a in range(12) for b in range(a, 12)]


def form_member_em(nodes, m, p):
    """assemble.rs `form_member` at precision p (every rounding mirrored)."""
    F = Fr
    xi, xj = nodes[m["i"]], nodes[m["j"]]

    def dot(a, b):
        return rp(sum((x * y for x, y in zip(a, b)), Fr(0)), p)

    def normalize(v):
        n = sqrt_p(dot(v, v), p)
        return [rp(c / n, p) for c in v], n

    d = [rp(F(xj[k]) - F(xi[k]), p) for k in range(3)]
    ex, L = normalize(d)
    yr = [F(c) for c in m["y"]]
    proj = dot(yr, ex)
    yc = [rp(yr[k] - proj * ex[k], p) for k in range(3)]
    ey, _ = normalize(yc)
    zc = [rp(ex[a] * ey[b] - ex[b] * ey[a], p) for a, b in ((1, 2), (2, 0), (0, 1))]
    ez, _ = normalize(zc)
    inv = rp(Fr(1) / L, p)
    axial = rp(rp(F(m["E"]) * F(m["A"]), p) / L, p)
    torsion = rp(rp(F(m["G"]) * F(m["J"]), p) / L, p)
    bz = rp(rp(F(m["E"]) * F(m["Iz"]), p) / L, p)
    by = rp(rp(F(m["E"]) * F(m["Iy"]), p) / L, p)
    B = [[Fr(0)] * 12 for _ in range(6)]
    for k in range(3):
        iy = rp(inv * ey[k], p)
        iz = rp(inv * ez[k], p)
        B[0][k], B[0][6 + k] = -ex[k], ex[k]
        B[1][3 + k], B[1][9 + k] = -ex[k], ex[k]
        for row, rot in ((2, 3), (3, 9)):
            B[row][k], B[row][6 + k], B[row][rot + k] = iy, -iy, ez[k]
        for row, rot in ((4, 3), (5, 9)):
            B[row][k], B[row][6 + k], B[row][rot + k] = -iz, iz, ey[k]
    drows = [[(0, axial)], [(1, torsion)], [(2, 4 * bz), (3, 2 * bz)], [(2, 2 * bz), (3, 4 * bz)],
             [(4, 4 * by), (5, 2 * by)], [(4, 2 * by), (5, 4 * by)]]
    DB = [[rp(sum((c * B[s][col] for s, c in drows[r]), Fr(0)), p) for col in range(12)] for r in range(6)]
    ke = {}
    for a, b in UPPER:
        ke[(a, b)] = rp(sum((B[r][a] * DB[r][b] for r in range(6)), Fr(0)), p)
    return dict(axes=[ex, ey, ez], L=L, inv=inv, axial=axial, torsion=torsion, bz=bz, by=by, B=B, ke=ke)


def directional_em(s, p):
    n = [Fr(c) for c in s["n"]]
    norm2 = rp(sum((c * c for c in n), Fr(0)), p)
    k = Fr(s["k"])
    out = [[Fr(0)] * 3 for _ in range(3)]
    for a in range(3):
        for b in range(a, 3):
            m = rp(n[a] * n[b], p)
            v = rp(rp(k * m, p) / norm2, p)
            out[a][b] = out[b][a] = v
    return out


def kind_offset(kind):
    return 0 if kind == "t" else 3


def assemble_em(model, p):
    """The assembled K at p: {(r, c) upper: value} (one exact sum per entry)."""
    nodes = model["nodes"]
    contrib = {}

    def add(r, c, v):
        if r <= c:
            contrib.setdefault((r, c), []).append(v)

    members = [form_member_em(nodes, m, p) for m in model["members"]]
    for m, op in zip(model["members"], members):
        dofs = [6 * m["i"] + k for k in range(6)] + [6 * m["j"] + k for k in range(6)]
        for a in range(12):
            for b in range(12):
                r, c = dofs[a], dofs[b]
                if r <= c:
                    contrib.setdefault((r, c), []).append(op["ke"][(min(a, b), max(a, b))])
    for s in model["springs"]:
        d = 6 * s["node"] + s["c"]
        contrib.setdefault((d, d), []).append(Fr(s["k"]))
    for s in model["dsprings"]:
        blk = directional_em(s, p)
        base = 6 * s["node"] + kind_offset(s["kind"])
        for a in range(3):
            for b in range(a, 3):
                contrib.setdefault((base + a, base + b), []).append(blk[a][b])
    return {rc: rp(sum(vs, Fr(0)), p) for rc, vs in contrib.items()}, members


# ---- exact rational solves (the controls' expectations)

def qsqrt_or_none(q):
    n, d = q.numerator, q.denominator
    rn, rd = isqrt(n), isqrt(d)
    if rn * rn == n and rd * rd == d:
        return Fr(rn, rd)
    return None


def skew(e):
    return [[Fr(0), -e[2], e[1]], [e[2], Fr(0), -e[0]], [-e[1], e[0], Fr(0)]]


def local_matrix(m, L):
    F = Fr
    EA, GJ = F(m["E"]) * F(m["A"]) / L, F(m["G"]) * F(m["J"]) / L
    k = [[Fr(0)] * 12 for _ in range(12)]

    def sym(a, b, v):
        k[a][b] = v
        k[b][a] = v
    k[0][0] = k[6][6] = EA
    sym(0, 6, -EA)
    k[3][3] = k[9][9] = GJ
    sym(3, 9, -GJ)
    for idx, I, sgn in (((1, 5, 7, 11), F(m["Iz"]), 1), ((2, 4, 8, 10), F(m["Iy"]), -1)):
        c = F(m["E"]) * I / L ** 3
        pat = [[12, 6 * L * sgn, -12, 6 * L * sgn], [6 * L * sgn, 4 * L * L, -6 * L * sgn, 2 * L * L],
               [-12, -6 * L * sgn, 12, -6 * L * sgn], [6 * L * sgn, 2 * L * L, -6 * L * sgn, 4 * L * L]]
        for a in range(4):
            for b in range(4):
                k[idx[a]][idx[b]] = c * pat[a][b]
    return k


def exact_member(nodes, m):
    """(ke global 12x12, e, L, axes or None) of a member, exact."""
    xi = [Fr(c) for c in nodes[m["i"]]]
    xj = [Fr(c) for c in nodes[m["j"]]]
    d = [b - a for a, b in zip(xi, xj)]
    L = qsqrt_or_none(sum(c * c for c in d))
    assert L is not None, "irrational member length: no exact control"
    e = [c / L for c in d]
    yr = [Fr(c) for c in m["y"]]
    proj = sum(a * b for a, b in zip(yr, e))
    yc = [yr[k] - proj * e[k] for k in range(3)]
    ny = qsqrt_or_none(sum(c * c for c in yc))
    axes = None
    if ny is not None:
        ey = [c / ny for c in yc]
        ez = [e[1] * ey[2] - e[2] * ey[1], e[2] * ey[0] - e[0] * ey[2], e[0] * ey[1] - e[1] * ey[0]]
        axes = [e, ey, ez]
        kl = local_matrix(m, L)
        T = [[Fr(0)] * 12 for _ in range(12)]
        for blk in range(4):
            for r in range(3):
                for c in range(3):
                    T[3 * blk + r][3 * blk + c] = axes[r][c]
        KT = [[sum(kl[i][k] * T[k][j] for k in range(12)) for j in range(12)] for i in range(12)]
        ke = [[sum(T[k][i] * KT[k][j] for k in range(12)) for j in range(12)] for i in range(12)]
    else:
        assert m["Iy"] == m["Iz"], "irrational axes need an isotropic section"
    if ny is None or m["Iy"] == m["Iz"]:
        EA, GJ = Fr(m["E"]) * Fr(m["A"]) / L, Fr(m["G"]) * Fr(m["J"]) / L
        EI = Fr(m["E"]) * Fr(m["Iz"])
        a, b, f4, f2 = 12 * EI / L ** 3, 6 * EI / L ** 2, 4 * EI / L, 2 * EI / L
        P = [[(1 if r == c else 0) - e[r] * e[c] for c in range(3)] for r in range(3)]
        E3 = [[e[r] * e[c] for c in range(3)] for r in range(3)]
        S = skew(e)
        blocks = {}
        Kt = [[EA * E3[r][c] + a * P[r][c] for c in range(3)] for r in range(3)]
        Kri = [[GJ * E3[r][c] + f4 * P[r][c] for c in range(3)] for r in range(3)]
        Krj = [[-GJ * E3[r][c] + f2 * P[r][c] for c in range(3)] for r in range(3)]
        neg = lambda M: [[-x for x in row] for row in M]
        tr = lambda M: [[M[c][r] for c in range(3)] for r in range(3)]
        mS = [[-b * S[r][c] for c in range(3)] for r in range(3)]
        pS = [[b * S[r][c] for c in range(3)] for r in range(3)]
        blocks[(0, 0)], blocks[(0, 2)], blocks[(2, 2)] = Kt, neg(Kt), Kt
        blocks[(1, 1)], blocks[(1, 3)], blocks[(3, 3)] = Kri, Krj, Kri
        blocks[(0, 1)], blocks[(0, 3)], blocks[(2, 1)], blocks[(2, 3)] = mS, mS, pS, pS
        full = [[Fr(0)] * 12 for _ in range(12)]
        for (bi, bj), M in list(blocks.items()):
            for r in range(3):
                for c in range(3):
                    full[3 * bi + r][3 * bj + c] = M[r][c]
                    full[3 * bj + c][3 * bi + r] = M[r][c]
        if ny is not None:
            assert full == ke, "projector form disagrees with T^T K T"
        ke = full
    return ke, e, L, axes


def solve_exact(model):
    """Exact published quantities of a model (keys as in models.txt)."""
    nodes = model["nodes"]
    nn = len(nodes)
    n = 6 * nn
    K = {}
    mem = []
    for m in model["members"]:
        ke, e, L, axes = exact_member(nodes, m)
        dofs = [6 * m["i"] + k for k in range(6)] + [6 * m["j"] + k for k in range(6)]
        for a in range(12):
            for b in range(12):
                if ke[a][b] != 0:
                    K[(dofs[a], dofs[b])] = K.get((dofs[a], dofs[b]), Fr(0)) + ke[a][b]
        mem.append((m, ke, e, L, axes, dofs))
    for s in model["springs"]:
        d = 6 * s["node"] + s["c"]
        K[(d, d)] = K.get((d, d), Fr(0)) + Fr(s["k"])
    for s in model["dsprings"]:
        nv = [Fr(c) for c in s["n"]]
        n2 = sum(c * c for c in nv)
        base = 6 * s["node"] + kind_offset(s["kind"])
        for a in range(3):
            for b in range(3):
                v = Fr(s["k"]) * nv[a] * nv[b] / n2
                if v != 0:
                    K[(base + a, base + b)] = K.get((base + a, base + b), Fr(0)) + v
    f = [Fr(0)] * n
    for l in model["loads"]:
        f[6 * l["node"] + l["c"]] += Fr(l["v"])
    fixed = {6 * c["node"] + c["c"]: Fr(c["v"]) for c in model["constraints"]}
    free = [g for g in range(n) if g not in fixed]
    pos = {g: a for a, g in enumerate(free)}
    nf = len(free)
    A = [[Fr(0)] * nf for _ in range(nf)]
    rhs = [f[g] for g in free]
    for (r, c), v in K.items():
        if r in pos and c in pos:
            A[pos[r]][pos[c]] += v
        elif r in pos and c in fixed:
            rhs[pos[r]] -= v * fixed[c]
    # Gaussian elimination (exact).
    M = [row[:] + [rhs[i]] for i, row in enumerate(A)]
    for col in range(nf):
        piv = next(r for r in range(col, nf) if M[r][col] != 0)
        M[col], M[piv] = M[piv], M[col]
        for r in range(nf):
            if r != col and M[r][col] != 0:
                fac = M[r][col] / M[col][col]
                M[r] = [x - fac * y for x, y in zip(M[r], M[col])]
    u = [Fr(0)] * n
    for g, v in fixed.items():
        u[g] = v
    for a, g in enumerate(free):
        u[g] = M[a][nf] / M[a][a]
    out = {}
    for g in range(n):
        out["u.%d.%d" % (g // 6, g % 6)] = u[g]
    for node in range(nn):
        out["mag.%d" % node] = ("sqrt", sum(u[6 * node + k] ** 2 for k in range(3)))
    for (m, ke, e, L, axes, dofs) in mem:
        ue = [u[d] for d in dofs]
        Fe = [sum(ke[a][b] * ue[b] for b in range(12)) for a in range(12)]
        Fi, Mi, Fj, Mj = Fe[0:3], Fe[3:6], Fe[6:9], Fe[9:12]
        dot3 = lambda a, b: sum(x * y for x, y in zip(a, b))
        perp2 = lambda v: dot3(v, v) - dot3(v, e) ** 2
        mid = m["id"]
        out["N.%d" % mid] = dot3(Fj, e)
        out["T.%d" % mid] = dot3(Mj, e)
        out["Mb.%d.i" % mid] = ("sqrt", perp2(Mi))
        out["Mb.%d.j" % mid] = ("sqrt", perp2(Mj))
        if axes is not None:
            for end, (Fv, Mv) in (("i", (Fi, Mi)), ("j", (Fj, Mj))):
                for c in range(3):
                    out["end.%d.%s.%d" % (mid, end, c)] = dot3(axes[c], Fv)
                    out["end.%d.%s.%d" % (mid, end, 3 + c)] = dot3(axes[c], Mv)
        for st in model["stations"]:
            if st["member"] != mid:
                continue
            t = Fr(st["t"])
            xi = [Fr(c) for c in nodes[m["i"]]]
            xj = [Fr(c) for c in nodes[m["j"]]]
            arm = [t * (b - a) for a, b in zip(xi, xj)]
            cross = [arm[1] * Fi[2] - arm[2] * Fi[1], arm[2] * Fi[0] - arm[0] * Fi[2], arm[0] * Fi[1] - arm[1] * Fi[0]]
            Mx = [-Mi[k] + cross[k] for k in range(3)]
            Fx = [-c for c in Fi]
            out["Mbs.%d" % st["id"]] = ("sqrt", perp2(Mx))
            if axes is not None:
                for c in range(3):
                    out["st.%d.%d" % (st["id"], c)] = dot3(axes[c], Fx)
                    out["st.%d.%d" % (st["id"], 3 + c)] = dot3(axes[c], Mx)
    spring_action = {}
    for s in model["springs"]:
        v = -Fr(s["k"]) * u[6 * s["node"] + s["c"]]
        spring_action[s["id"]] = {s["c"]: v}
        out["spr.%d.%d" % (s["id"], s["c"])] = v
    for s in model["dsprings"]:
        nv = [Fr(c) for c in s["n"]]
        n2 = sum(c * c for c in nv)
        base = 6 * s["node"] + kind_offset(s["kind"])
        proj = sum(nv[b] * u[base + b] for b in range(3))
        acts = {}
        for a in range(3):
            v = -Fr(s["k"]) * proj * nv[a] / n2
            acts[kind_offset(s["kind"]) + a] = v
            out["dspr.%d.%d" % (s["id"], kind_offset(s["kind"]) + a)] = v
        spring_action[s["id"]] = acts
    reaction = {}
    for g in sorted(fixed):
        r = sum(v * u[c] for (rr, c), v in K.items() if rr == g) - f[g]
        reaction[g] = r
        out["R.%d.%d" % (g // 6, g % 6)] = r
    for grp in model["supports"]:
        comp = [Fr(0)] * 6
        for c in range(6):
            g = 6 * grp["node"] + c
            if grp["r"][c]:
                comp[c] += reaction[g]
            for sid in grp["springs"] + grp["dsprings"]:
                comp[c] += spring_action[sid].get(c, Fr(0))
        out["sf.%d" % grp["id"]] = ("sqrt", sum(x * x for x in comp[:3]))
        out["sm.%d" % grp["id"]] = ("sqrt", sum(x * x for x in comp[3:]))
    return out


def to_f64_bits(v):
    if isinstance(v, tuple):
        x = v[1]
        if x == 0:
            return 0
        return b64(float(sqrt_p(x, 90)))
    if v == 0:
        return 0
    return b64(float(v))


# ---- model definitions (invented inputs; D1's probe models and K4's own controls)

def n_section():
    """The N-series section on the intended basis: fl() of R1's exact values."""
    E, G = 200e9, 80e9
    od, idd = Fr("0.2"), Fr("0.18")
    A = PI_Q * (od * od - idd * idd) / 4
    I = PI_Q * (od ** 4 - idd ** 4) / 64
    return dict(E=E, G=G, A=float(A), Iy=float(I), Iz=float(I), J=float(2 * I))


def new_model(name, nodes):
    return dict(name=name, nodes=[tuple(float(c) for c in p) for p in nodes], members=[], springs=[],
                dsprings=[], constraints=[], loads=[], stations=[], supports=[])


def add_member(model, mid, i, j, y=(0.0, 0.0, 1.0), section=None, **over):
    s = dict(section or n_section())
    s.update(over)
    model["members"].append(dict(id=mid, i=i, j=j, y=tuple(float(c) for c in y), **s))


def fix(model, node, comps, value=0.0):
    for c in comps:
        model["constraints"].append(dict(node=node, c=c, v=float(value)))


def spring(model, sid, node, c, k):
    model["springs"].append(dict(id=sid, node=node, c=c, k=float(k)))


def load(model, node, c, v, src=None):
    model["loads"].append(dict(node=node, c=c, v=float(v), src=src or "l%d" % len(model["loads"])))


def pin_case(name, direction, k, moment, members=1, y=(0.0, 0.0, 1.0)):
    """D1's probe `pin_case`: root translations fixed, root rotations on three
    global springs k, a run of members along `direction`, a tip moment."""
    nodes = [tuple(c * s for c in direction) for s in range(members + 1)]
    m = new_model(name, nodes)
    for s in range(members):
        add_member(m, s + 1, s, s + 1, y)
    fix(m, 0, (0, 1, 2))
    for c in (3, 4, 5):
        spring(m, c, 0, c, k)
    for c in range(3):
        if moment[c] != 0:
            load(m, members, 3 + c, moment[c])
    return m


def models():
    out = []
    sec = n_section()
    # N01, N08 (four torques), N09 (bending and torsion).
    m = new_model("N01", [(0, 0, 0), (2, 0, 0)])
    add_member(m, 1, 0, 1)
    fix(m, 0, range(6))
    load(m, 1, 1, 1000.0)
    m["stations"].append(dict(id=1, member=1, t=0.25))
    m["stations"].append(dict(id=2, member=1, t=0.5))
    out.append(m)
    for idx, T in enumerate((1.0, -1.0, 0.1, -0.1)):
        m = new_model("N08-%d" % idx, [(0, 0, 0), (2, 0, 0)])
        add_member(m, 1, 0, 1)
        fix(m, 0, range(6))
        load(m, 1, 3, T)
        out.append(m)
    m = new_model("N09-B", [(0, 0, 0), (10, 0, 0)])
    add_member(m, 1, 0, 1)
    fix(m, 0, range(6))
    load(m, 1, 1, 100.0)
    out.append(m)
    m = new_model("N09-T", [(0, 0, 0), (10, 0, 0)])
    add_member(m, 1, 0, 1)
    fix(m, 0, range(6))
    load(m, 1, 3, 0.1)
    out.append(m)
    # N05, N06, and N05 with a transverse tip force (T3's N05 item).
    for name, k, T, fy in (("N05", 1e-4, 1e-8, None), ("N06", 1e-12, 1e-16, None), ("N05-TRANSVERSE", 1e-4, 1e-8, 1.0)):
        m = new_model(name, [(0, 0, 0), (2, 0, 0)])
        add_member(m, 1, 0, 1)
        fix(m, 0, (0, 1, 2, 4, 5))
        spring(m, 1, 0, 3, k)
        load(m, 1, 3, T)
        if fy is not None:
            load(m, 1, 1, fy)
        m["stations"].append(dict(id=1, member=1, t=0.5))
        m["supports"].append(dict(id=1, node=0, r=[1, 1, 1, 0, 1, 1], springs=[1], dsprings=[]))
        out.append(m)
    # D1's probes (§3.1, §3.2).
    skew = (3.0, 4.0, 0.0)
    mom = lambda k: (1e-8 * k / 1e-4, 2e-8 * k / 1e-4, 0.0)
    out.append(pin_case("SKEW-K1E-28", skew, 1e-28, mom(1e-28)))
    out.append(pin_case("SKEW6-K1E-12", skew, 1e-12, mom(1e-12), members=6))
    out.append(pin_case("SKEW-K1E-12", skew, 1e-12, mom(1e-12)))
    out.append(pin_case("SKEW-K1E-4", skew, 1e-4, mom(1e-4)))
    out.append(pin_case("AXIS-K1E-4", (5.0, 0.0, 0.0), 1e-4, mom(1e-4)))
    out.append(pin_case("OBLIQUE-K1E-4", (2.0, 3.0, 6.0), 1e-4, mom(1e-4), y=(6.0, 2.0, -3.0)))
    m = pin_case("B1-L", skew, 1e-4, (0.0, 0.0, 0.0))
    for v in (1e80, 1e-8, -1e80):
        load(m, 1, 3, v)
    load(m, 1, 4, 2e-8)
    out.append(m)
    for name, loads in (("B1-C-A", [1e80]), ("B1-C-B", [1e-8]), ("B1-C-A2", [1e80]),
                        ("B1-E-A", [1e-8, 1e-53]), ("B1-E-B", [1e-8]), ("B1-E-NET", [1e-53]),
                        ("B1-C-NET", [1e-8])):
        m = pin_case(name, skew, 1e-4, (0.0, 0.0, 0.0))
        for v in loads:
            load(m, 1, 3, v)
        out.append(m)
    for s in (1e-10, 1e-14):
        m = new_model("S8-W-%g" % s, [(0, 0, 0), (3, 4, 0), (6, 8, 0)])
        add_member(m, 1, 0, 1)
        add_member(m, 2, 1, 2, E=sec["E"] * s, G=sec["G"] * s)
        fix(m, 0, range(6))
        for c in range(6):
            spring(m, 1 + c, 2, c, 1e12)
        load(m, 1, 1, 1000.0)
        load(m, 1, 5, 300.0)
        out.append(m)
    # §7.3-16's duplicate-operand control: two identical collinear members meet
    # at node 1 (their node-1 translation-rotation couplings cancel exactly) and
    # a third member, 2^-300 as stiff, leaves node 1 along (3,4,0), so it adds
    # to the same entries. The weak member's id sorts between the two (the
    # source's canonical member order), the order in which a sequential fold
    # at p <= 256 loses it.
    m = new_model("DUPLICATE", [(0, 0, 0), (2, 0, 0), (4, 0, 0), (5, 4, 0)])
    add_member(m, 1, 0, 1)
    add_member(m, 2, 1, 3, E=sec["E"] * 2.0 ** -300, G=sec["G"] * 2.0 ** -300)
    add_member(m, 3, 1, 2)
    fix(m, 0, range(6))
    fix(m, 2, range(6))
    fix(m, 3, range(6))
    load(m, 1, 5, 1.0)
    load(m, 1, 1, 1.0)
    out.append(m)
    # A prescribed-motion control (KREV-02 analogue): a two-member cantilever
    # whose root rotates by a prescribed 1e-3 rad about z and whose far end
    # settles by 2e-4 m; no load.
    m = new_model("PRESCRIBED", [(0, 0, 0), (2, 0, 0), (4, 0, 0)])
    add_member(m, 1, 0, 1)
    add_member(m, 2, 1, 2)
    fix(m, 0, (0, 1, 2, 3, 4))
    fix(m, 0, (5,), 1e-3)
    fix(m, 2, (1,), -2e-4)
    fix(m, 2, (0, 2))
    out.append(m)
    # The pivot-escalation control: N05 with k = 2^-120·(GJ/L) (fails at 128,
    # passes at 256).
    a = float(Fr(sec["G"]) * Fr(sec["J"]) / 2)
    m = new_model("PIVOT", [(0, 0, 0), (2, 0, 0)])
    add_member(m, 1, 0, 1)
    fix(m, 0, (0, 1, 2, 4, 5))
    spring(m, 1, 0, 3, a * 2.0 ** -120)
    load(m, 1, 3, a * 2.0 ** -120 * 1e-4)
    out.append(m)
    # The reactions-only control (K4-M16): a stiff member whose two nodes are
    # prescribed the same unit translation (a rigid motion: its end actions are
    # exact zeros, its reactions carry K_e's rigid-mode leakage of about
    # 2^-p·EA/L), and, in the same body, a flexible axis-aligned cantilever
    # (formed exactly) carrying a tiny tip load F = 2^-45, which sets S*(force).
    # At 128 only reactions disagree (2^-101 > 2^-64·S*); at 256 they agree.
    m = new_model("REACTIONS-ONLY", [(0, 0, 0), (3, 4, 0), (3, 4, 2)])
    add_member(m, 1, 0, 1)
    fix(m, 0, (0,), 1.0)
    fix(m, 1, (0,), 1.0)
    fix(m, 0, (1, 2, 3, 4, 5))
    fix(m, 1, (1, 2, 3, 4, 5))
    add_member(m, 2, 1, 2, y=(1.0, 0.0, 0.0), Iy=2.0 ** -26, Iz=2.0 ** -26)
    load(m, 2, 0, 2.0 ** -45)
    out.append(m)
    # A finding (reported at A1): an unloaded body moved rigidly by prescribed
    # values has exact-zero forces and moments, but K_e's rigid-mode leakage
    # makes the computed ones nonzero at every p, so the stop rule's S*(force)
    # is the leakage itself and the case is unresolved at the ceiling.
    m = new_model("RIGID-UNLOADED", [(0, 0, 0), (3, 4, 0), (0, 0, 5), (0, 0, 7)])
    add_member(m, 1, 0, 1)
    fix(m, 0, (0,), 1.0)
    fix(m, 1, (0,), 1.0)
    fix(m, 0, (1, 2, 3, 4, 5))
    fix(m, 1, (1, 2, 3, 4, 5))
    add_member(m, 2, 2, 3, y=(1.0, 0.0, 0.0))
    fix(m, 2, range(6))
    load(m, 3, 0, 1e-30)
    out.append(m)
    # K4-M11's control: two collinear spans of lengths 1 and 1 + 2^-40 between
    # fixed ends, a moment at the shared node.
    m = new_model("TWO-SPAN", [(0, 0, 0), (1, 0, 0), (2 + 2.0 ** -40, 0, 0)])
    add_member(m, 1, 0, 1)
    add_member(m, 2, 1, 2)
    fix(m, 0, range(6))
    fix(m, 2, range(6))
    load(m, 1, 5, 1.0)
    out.append(m)
    # A structural-zero kind: a (3,4,0) cantilever under a torque along its axis
    # (every force and bending moment is an exact zero).
    m = new_model("ZERO-TORSION-345", [(0, 0, 0), (3, 4, 0)])
    add_member(m, 1, 0, 1)
    fix(m, 0, range(6))
    load(m, 1, 3, 3.0)
    load(m, 1, 4, 4.0)
    out.append(m)
    # An all-zero body beside a loaded one.
    m = new_model("ALL-ZERO-BODY", [(0, 0, 0), (2, 0, 0), (0, 5, 0), (2, 5, 0)])
    add_member(m, 1, 0, 1)
    add_member(m, 2, 2, 3)
    fix(m, 0, range(6))
    fix(m, 2, range(6))
    load(m, 1, 2, 10.0)
    out.append(m)
    # Directional springs spanning R³ exactly although a binary64 determinant
    # is zero (K4-M26): a cantilever root held by three directional springs.
    eps = 2.0 ** -52
    m = new_model("DIRECTIONAL-SPAN", [(0, 0, 0), (0, 0, 3)])
    add_member(m, 1, 0, 1, y=(1.0, 0.0, 0.0))
    fix(m, 0, (0, 1, 2))
    for sid, n in ((1, (1.0, 1.0, 1.0)), (2, (1.0, 1.0 + eps, 1.0)), (3, (1.0, 1.0, 1.0 + eps))):
        m["dsprings"].append(dict(id=sid, node=0, kind="r", n=n, k=1e6))
    load(m, 1, 3, 1.0)
    out.append(m)
    # Geometry controls from the frozen N-series (N02-N04): refused or solved.
    m = new_model("N02", [(0, 0, 0), (1.2, 1.6, 0)])
    add_member(m, 1, 0, 1)
    fix(m, 0, (0, 1, 2))
    fix(m, 1, (0, 1, 2))
    load(m, 1, 3, 0.6)
    load(m, 1, 4, 0.8)
    out.append(m)
    m = new_model("N03-RX", [(0, 0, 0), (1.2, 1.6, 0)])
    add_member(m, 1, 0, 1)
    fix(m, 0, (0, 1, 2, 3))
    fix(m, 1, (0, 1, 2))
    load(m, 1, 3, 0.6)
    load(m, 1, 4, 0.8)
    out.append(m)
    m = new_model("N03-RZ", [(0, 0, 0), (1.2, 1.6, 0)])
    add_member(m, 1, 0, 1)
    fix(m, 0, (0, 1, 2, 5))
    fix(m, 1, (0, 1, 2))
    load(m, 1, 3, 0.6)
    out.append(m)
    m = new_model("N04", [(0, 0, 0), (2, 0, 0), (0, 3, 0), (2, 3, 0)])
    add_member(m, 1, 0, 1)
    add_member(m, 2, 2, 3)
    fix(m, 0, range(6))
    load(m, 1, 1, 1000.0)
    out.append(m)
    # Deep soft modes: accepted at 512 (verified at the ceiling), and never.
    out.append(pin_case("SKEW-K1E-60", skew, 1e-60, mom(1e-60)))
    out.append(pin_case("SKEW-K1E-300", skew, 1e-300, mom(1e-300)))
    # A combination-precision control: the k = 1e-28 geometry under a load that
    # does not excite its soft mode (selected at 128), and one that does.
    m = pin_case("SKEW-K1E-28-AXIAL", skew, 1e-28, (0.0, 0.0, 0.0))
    load(m, 1, 0, 3.0)
    load(m, 1, 1, 4.0)
    out.append(m)
    # The ceiling-combination finding: operands P + ε and P with ε/P = 2^-1060.
    for name, loads in (("CEIL-A", [2.0 ** 1000, 2.0 ** -60]), ("CEIL-B", [2.0 ** 1000]), ("CEIL-NET", [2.0 ** -60])):
        m = pin_case(name, skew, 1e-4, (0.0, 0.0, 0.0))
        for v in loads:
            load(m, 1, 3, v)
        out.append(m)
    return out


def routed_models():
    """K-D5's D5C-1 controls (PROBE_D, PROBE_C, BENDING_SOFT) and K2b's
    spring-carried case, from their committed binary64 inputs."""
    out = []
    text = KD5_MODELS.read_text()
    num = r"(-?[0-9.e+-]+)"
    for name in ("PROBE_D", "PROBE_C", "BENDING_SOFT"):
        block = text[text.index('name: "%s"' % name):]
        block = block[:block.index("};")]
        sec = re.search(r"e: %s, g: %s, a: %s, i: %s, j: %s" % ((num,) * 5), block).groups()
        E, G, A, I, J = (float(x) for x in sec)
        nodes = [tuple(float(x) for x in t) for t in re.findall(r"\[%s, %s, %s\]" % ((num,) * 3),
                                                                block[block.index("nodes:"):block.index("members:")])]
        m = new_model(name, nodes)
        y = re.search(r"y_reference: \[%s, %s, %s\]" % ((num,) * 3), block).groups()
        add_member(m, 1, 0, 1, y=tuple(float(x) for x in y), section=dict(E=E, G=G, A=A, Iy=I, Iz=I, J=J))
        rigid = re.search(r"rigid: &\[([0-9, ]*)\]", block).group(1)
        for d in (int(x) for x in rigid.split(",") if x.strip()):
            fix(m, d // 6, (d % 6,))
        springs = re.search(r"springs: &\[(.*?)\],\n", block).group(1)
        for k, (d, v) in enumerate(re.findall(r"\(([0-9]+), %s\)" % num, springs)):
            spring(m, k + 1, int(d) // 6, int(d) % 6, float(v))
        loads = re.search(r"loads: &\[(.*?)\],\n", block).group(1)
        for d, v in re.findall(r"\(([0-9]+), %s\)" % num, loads):
            load(m, int(d) // 6, int(d) % 6, float(v))
        out.append(m)
    text = K2B_MODELS.read_text()
    block = text[text.index('id: "spring-carried"'):]
    block = block[:block.index("},\n    ReachCase")]
    bits = lambda field: f64_of(int(re.search(field + r": f64::from_bits\(0x([0-9a-f]+)\)", block).group(1), 16))
    length, E, G = bits("length"), bits("e"), bits("g")
    A, I, J = bits("area"), bits(r"\bi"), bits(r"\bj")
    m = new_model("SPRING-CARRIED", [(0, 0, 0), (length, 0, 0)])
    add_member(m, 1, 0, 1, y=(0.0, 1.0, 0.0), section=dict(E=E, G=G, A=A, Iy=I, Iz=I, J=J))
    fix(m, 0, range(6))
    fix(m, 1, (0, 1, 2, 4, 5))
    spring(m, 1, 1, 3, 1.0)
    load(m, 1, 3, 1.0)
    out.append(m)
    return out


COMBOS = (
    ("B1-C", ((1.0, "B1-C-A"), (1.0, "B1-C-B"), (-1.0, "B1-C-A2")), "B1-C-NET"),
    ("B1-E", ((1.0, "B1-E-A"), (-1.0, "B1-E-B")), "B1-E-NET"),
    ("PRECISION-RULE", ((1.0, "SKEW-K1E-28-AXIAL"), (1.0, "SKEW-K1E-28")), None),
    ("CEILING", ((1.0, "CEIL-A"), (-1.0, "CEIL-B")), "CEIL-NET"),
)


def model_lines(m, expectations=True):
    lines = ["model %s" % m["name"]]
    for p in m["nodes"]:
        lines.append("node %s %s %s" % tuple(hexf(c) for c in p))
    for mm in m["members"]:
        lines.append("member %d %d %d %s %s %s %s %s %s %s %s %s" % (
            mm["id"], mm["i"], mm["j"], hexf(mm["E"]), hexf(mm["G"]), hexf(mm["A"]), hexf(mm["Iy"]),
            hexf(mm["Iz"]), hexf(mm["J"]), hexf(mm["y"][0]), hexf(mm["y"][1]), hexf(mm["y"][2])))
    for s in m["springs"]:
        lines.append("spring %d %d %d %s" % (s["id"], s["node"], s["c"], hexf(s["k"])))
    for s in m["dsprings"]:
        lines.append("dspring %d %d %s %s %s %s %s" % (s["id"], s["node"], s["kind"], hexf(s["n"][0]),
                                                     hexf(s["n"][1]), hexf(s["n"][2]), hexf(s["k"])))
    for c in m["constraints"]:
        lines.append("constraint %d %d %s" % (c["node"], c["c"], hexf(c["v"])))
    for l in m["loads"]:
        lines.append("load %d %d %s %s" % (l["node"], l["c"], hexf(l["v"]), l["src"]))
    for s in m["stations"]:
        lines.append("station %d %d %s" % (s["id"], s["member"], hexf(s["t"])))
    for g in m["supports"]:
        lines.append("support %d %d %s %s %s" % (g["id"], g["node"], "".join(str(int(x)) for x in g["r"]),
                                                ",".join(map(str, g["springs"])) or "-",
                                                ",".join(map(str, g["dsprings"])) or "-"))
    if expectations:
        try:
            ex = solve_exact(m)
        except (AssertionError, StopIteration):
            ex = None  # an irrational length, or a singular (mechanism) model
        if ex is not None:
            for key in sorted(ex):
                lines.append("expect %s %016x" % (key, to_f64_bits(ex[key])))
    lines.append("end")
    return lines


FORMATION_MODELS = ("M-AX", "M-345", "M-236", "M-122", "M-OFF", "DUPLICATE", "DIRECTIONAL-SPAN")


def formation_models():
    sec = n_section()
    out = {}
    for name, xj, y in (("M-AX", (2, 0, 0), (0.0, 1.0, 0.0)), ("M-345", (3, 4, 0), (0.0, 0.0, 1.0)),
                        ("M-236", (2, 3, 6), (6.0, -2.0 + 4.0, -3.0)), ("M-122", (1, 2, 2), (1.0, 0.0, 0.0))):
        m = new_model(name, [(0, 0, 0), xj])
        add_member(m, 1, 0, 1, y=y)
        out[name] = m
    m = new_model("M-OFF", [(1.1, -0.7, 2.3), (4.9, 3.3, -1.7)])
    add_member(m, 1, 0, 1, y=(0.3, 1.0, 0.2), Iy=sec["Iy"] * 1.5)
    out["M-OFF"] = m
    for mm in models():
        if mm["name"] in ("DUPLICATE", "DIRECTIONAL-SPAN"):
            out[mm["name"]] = mm
    return out


def formation_lines():
    lines = []
    fm = formation_models()
    for name in FORMATION_MODELS:
        m = fm[name]
        lines += model_lines(m, expectations=False)
        for p, L in K4_PRECISIONS + ((53, 4),):
            K, _ = assemble_em(m, p)
            for (r, c) in sorted(K):
                lines.append("k %s %d %d %d %d %s" % (name, p, L, r, c, W_of(K[(r, c)], L).token()))
    return lines


# ----------------------------------------------------------------------------
# Assembly of the files
# ----------------------------------------------------------------------------
# ----------------------------------------------------------------------------
# O8: the method emulated bit for bit (ROOT's ruling O8): N05 and N06 at 128
# and 256, and one skew member at 128; the retained state's sha256
# ----------------------------------------------------------------------------
def rcm_em(adjacency):
    """factor.rs `reverse_cuthill_mckee` (sparse_direct's rules)."""
    n = len(adjacency)
    neighbors = [[] for _ in range(n)]
    for node, raw in enumerate(adjacency):
        for other in raw:
            if other != node:
                neighbors[node].append(other)
                neighbors[other].append(node)
    neighbors = [sorted(set(l)) for l in neighbors]
    degrees = [len(l) for l in neighbors]
    neighbors = [sorted(l, key=lambda x: (degrees[x], x)) for l in neighbors]

    def reachable(seed):
        marked = {seed}
        out, queue = [seed], [seed]
        while queue:
            node = queue.pop(0)
            for nxt in neighbors[node]:
                if nxt not in marked:
                    marked.add(nxt)
                    out.append(nxt)
                    queue.append(nxt)
        return out

    def eccentricity(start):
        marked = {start}
        level, depth = [start], 0
        while True:
            nxt_level = []
            for node in level:
                for nxt in neighbors[node]:
                    if nxt not in marked:
                        marked.add(nxt)
                        nxt_level.append(nxt)
            if not nxt_level:
                return depth, level
            depth += 1
            level = nxt_level

    def peripheral(seed):
        comp = reachable(seed)
        cand = min(comp, key=lambda x: (degrees[x], x))
        ecc, last = eccentricity(cand)
        while last:
            nxt = min(last, key=lambda x: (degrees[x], x))
            e2, l2 = eccentricity(nxt)
            if e2 > ecc:
                cand, ecc, last = nxt, e2, l2
            else:
                break
        return cand

    visited = [False] * n
    order = []
    for seed in range(n):
        if visited[seed]:
            continue
        start = peripheral(seed)
        visited[start] = True
        queue = [start]
        while queue:
            node = queue.pop(0)
            order.append(node)
            for nxt in neighbors[node]:
                if not visited[nxt]:
                    visited[nxt] = True
                    queue.append(nxt)
    order.reverse()
    return order


def emulate_state(model, p):
    """solve_case_at at p (its shared stages included): (u, Q per member,
    corrections), every rounding mirrored."""
    q = p + 64
    rnd = lambda x: rp(x, p)
    K, members = assemble_em(model, p)
    Kq, _ = assemble_em(model, q)
    n = 6 * len(model["nodes"])
    prescribed = {6 * c["node"] + c["c"]: Fr(c["v"]) for c in model["constraints"]}
    free = [g for g in range(n) if g not in prescribed]
    position = {g: a for a, g in enumerate(free)}
    pattern = {g: set() for g in range(n)}
    for (r, c) in K:
        pattern[r].add(c)
        pattern[c].add(r)
    get_k = lambda M, r, c: M.get((min(r, c), max(r, c)))
    adjacency = [[position[c] for c in sorted(pattern[g]) if c in position and c != g] for g in free]
    order = rcm_em(adjacency)
    nf = len(free)
    rank = [0] * nf
    for k, a in enumerate(order):
        rank[a] = k
    first = [min([rank[b] for b in adjacency[a]] + [i]) for i, a in enumerate(order)]
    scale = [-(k3.floor_log2(get_k(K, g, g)) // 2) for g in free]
    rows = []
    for i in range(nf):
        a = order[i]
        row = {}
        for j in range(first[i], i + 1):
            b = order[j]
            v = get_k(K, free[a], free[b])
            row[j] = (v if v is not None else Fr(0)) * Fr(2) ** (scale[a] + scale[b])
        rows.append(row)
    get = lambda i, j: rows[i][j] if j >= first[i] else Fr(0)
    work = [Fr(0)] * nf
    for i in range(nf):
        for j in range(first[i], i):
            s_ = get(i, j)
            for kk in range(max(first[i], first[j]), j):
                s_ = rnd(s_ - rnd(work[kk] * get(j, kk)))
            work[j] = s_
            rows[i][j] = rnd(s_ / get(j, j))
        pivot = get(i, i)
        canc = abs(pivot)
        for kk in range(first[i], i):
            term = rnd(work[kk] * get(i, kk))
            pivot = rnd(pivot - term)
            canc = rnd(canc + abs(term))
        m = 2 * (i - first[i]) + 2
        assert pivot * (2 ** p - m) - 64 * m * canc > 0, "pivot screen"
        rows[i][i] = pivot

    def solve(b):
        x = [b[order[i]] * Fr(2) ** scale[order[i]] for i in range(nf)]
        for i in range(nf):
            for j in range(first[i], i):
                x[i] = rnd(x[i] - rnd(get(i, j) * x[j]))
        for i in range(nf):
            x[i] = rnd(x[i] / get(i, i))
        for i in reversed(range(nf)):
            v = x[i]
            for j in range(first[i], i):
                x[j] = rnd(x[j] - rnd(get(i, j) * v))
        out = [Fr(0)] * nf
        for i in range(nf):
            out[order[i]] = x[i]
        return [out[a] * Fr(2) ** scale[a] for a in range(nf)]

    ledger = {}
    for l in model["loads"]:
        g = 6 * l["node"] + l["c"]
        ledger[g] = ledger.get(g, Fr(0)) + Fr(l["v"])
    rhs = []
    for g in free:
        v = ledger.get(g, Fr(0))
        for c in pattern[g]:
            if c in prescribed:
                v -= get_k(K, g, c) * prescribed[c]
        rhs.append(rnd(v))
    u_free = solve(rhs)
    u = [prescribed.get(g, Fr(0)) for g in range(n)]
    corrections = 0
    while True:
        for a, g in enumerate(free):
            u[g] = u_free[a]
        passes, residuals = True, []
        for g in free:
            r = ledger.get(g, Fr(0))
            d = abs(r)
            count = 0
            for c in sorted(pattern[g]):
                kv = get_k(Kq, g, c)
                if kv == 0 or u[c] == 0:
                    continue
                count += 1
                r -= kv * u[c]
                d += abs(kv * u[c])
            m = 2 * count + 2
            passes = passes and abs(r) * (2 ** p - m) <= 64 * m * d
            residuals.append(rnd(r))
        if passes:
            break
        assert corrections < 3, "the emulated cases need no more than three corrections"
        delta = solve(residuals)
        u_free = [rnd(x + y) for x, y in zip(u_free, delta)]
        corrections += 1
    Q = []
    for mm, op in zip(model["members"], members):
        dofs = [6 * mm["i"] + k for k in range(6)] + [6 * mm["j"] + k for k in range(6)]
        axes, inv = op["axes"], op["inv"]
        d = [rnd(sum((axes[r][c] * u[dofs[3 * blk + c]] for c in range(3)), Fr(0)))
             for blk in range(4) for r in range(3)]
        e = [rnd(d[6] - d[0]), rnd(d[9] - d[3])]
        for rot, tr, sign in ((5, 1, True), (11, 1, True), (4, 2, False), (10, 2, False)):
            e.append(rnd(d[rot] + (inv * d[tr] - inv * d[tr + 6]) * (1 if sign else -1)))
        bz, by = op["bz"], op["by"]
        Q.append([rnd(op["axial"] * e[0]), rnd(op["torsion"] * e[1]),
                  rnd(4 * bz * e[2] + 2 * bz * e[3]), rnd(2 * bz * e[2] + 4 * bz * e[3]),
                  rnd(4 * by * e[4] + 2 * by * e[5]), rnd(2 * by * e[4] + 4 * by * e[5])])
    return u, Q, corrections


def state_sha(model, p):
    L = width_of(p)
    u, Q, corrections = emulate_state(model, p)
    out = b"K4RST\x01" + struct.pack("<II", p, L) + struct.pack("<I", len(u))
    for v in u:
        out += W_of(v, L).enc()
    out += struct.pack("<I", len(Q))
    for qm in Q:
        for v in qm:
            out += W_of(v, L).enc()
    return hashlib.sha256(out).hexdigest(), corrections


def o8_lines():
    by_name = {m["name"]: m for m in models()}
    lines = []
    for name, p in (("N05", 128), ("N05", 256), ("N06", 128), ("N06", 256), ("SKEW-K1E-4", 128)):
        sha, corrections = state_sha(by_name[name], p)
        lines.append("state %s %d %s %d" % (name, p, sha, corrections))
    return lines


# ----------------------------------------------------------------------------
# J: S* and the classification (D1 §4.1.6 item 1, §4.1.6.1), a binary64
# reimplementation (Python floats are IEEE binary64, round to nearest)
# ----------------------------------------------------------------------------
R_FLOOR = f64_of(0x3DD0000000000000)  # 2^-34
SMALL_S = f64_of(0x0230000000000000)  # 2^-988
K_SQRT2 = f64_of(0x3FF6A09E667F3BCD)
K_TWO_SQRT2 = f64_of(0x4006A09E667F3BCD)
TWO64 = 18446744073709551616.0


def next_up_f(x):
    return math.nextafter(x, math.inf)


def bound_up(s):
    nearest = s / TWO64
    return next_up_f(nearest) if nearest * TWO64 < s else nearest


def coupled(s, extent):
    if extent == 0.0:
        return list(s)
    tr, ro, fo, mo = s
    return [max(tr, extent * ro), max(ro, tr / extent), max(fo, mo / extent), max(mo, extent * fo)]


def classify_set(bodies, extents, rows):
    S = [[0.0] * 4 for _ in range(bodies)]
    for kind, body, derived, out in rows:
        if not derived and out[0] in "NS":
            S[body][kind] = max(S[body][kind], abs(out[1]))
    scales = [coupled(S[b], extents[b]) for b in range(bodies)]
    classes = []
    for kind, body, derived, out in rows:
        if derived:
            classes.append("I")
        elif out[0] in "NS":
            s = scales[body][kind]
            if s < SMALL_S or abs(out[1]) < R_FLOOR * s:
                classes.append("A:%s" % hexf(bound_up(s)))
            else:
                classes.append("R")
        else:
            classes.append("U")
    return scales, classes


def out_token(out):
    if out[0] in "NS":
        return "%s:%s" % (out[0], hexf(out[1]))
    return out[0] + ("-" if out[1] else "+")


def rand_value(rng):
    r = rng.next() % 64
    neg = rng.next() & 1 == 1
    if r == 0:
        return ("N", 0.0)
    if r == 1:
        return ("U", neg)
    if r == 2:
        return ("O", neg)
    if r == 3:
        v = f64_of(rng.next() % (1 << 52) or 1)
        return ("S", -v if neg else v)
    e = (rng.next() % 1600) - 1000
    v = math.ldexp(1.0 + (rng.next() % (1 << 52)) / 2.0 ** 52, max(min(e, 1000), -1020))
    return ("N", -v if neg else v)


def classification_lines():
    rng = k3.SplitMix64(seed_of("K4CLASS1"))
    sets = []
    for _ in range(400):
        bodies = 1 + rng.next() % 3
        extents = [0.0 if rng.next() % 8 == 0 else math.ldexp(1.0 + (rng.next() % 1000) / 1000.0,
                                                               (rng.next() % 30) - 10) for _ in range(bodies)]
        rows = [(rng.next() % 4, rng.next() % bodies, rng.next() % 8 == 0, rand_value(rng))
                for _ in range(1 + rng.next() % 24)]
        sets.append((bodies, extents, rows))
    # Targeted: one ulp either side of t = fl(R·S*) (§7.3-20); S* < 2^-988;
    # 0 < S* < 2^-1011 (b is at least the least subnormal); S* = 0 (b = 0).
    t = R_FLOOR * 1.0
    sets.append((1, [0.0], [(0, 0, False, ("N", 1.0)), (0, 0, False, ("N", t)),
                            (0, 0, False, ("N", math.nextafter(t, 0.0))), (0, 0, False, ("N", next_up_f(t))),
                            (0, 0, False, ("N", -math.nextafter(t, 0.0)))]))
    sets.append((1, [2.0], [(1, 0, False, ("N", 3.0)), (1, 0, False, ("N", R_FLOOR * 3.0)),
                            (1, 0, False, ("N", math.nextafter(R_FLOOR * 3.0, 0.0)))]))
    tiny = math.ldexp(1.0, -990)
    sets.append((1, [0.0], [(2, 0, False, ("N", tiny)), (2, 0, False, ("N", tiny / 3.0))]))
    sets.append((1, [0.0], [(3, 0, False, ("S", math.ldexp(1.0, -1015))), (3, 0, False, ("N", 0.0))]))
    sets.append((1, [0.0], [(0, 0, False, ("N", 0.0)), (1, 0, False, ("N", 0.0)), (0, 0, True, ("N", 5.0))]))
    lines = []
    for k, (bodies, extents, rows) in enumerate(sets):
        scales, classes = classify_set(bodies, extents, rows)
        lines.append("set %d %d %s" % (k, bodies, " ".join(hexf(e) for e in extents)))
        for (kind, body, derived, out), cls in zip(rows, classes):
            lines.append("row %d %d %d %s %s" % (kind, body, int(derived), out_token(out), cls))
        for b in range(bodies):
            lines.append("scales %d %s" % (b, " ".join(hexf(x) for x in scales[b])))
        lines.append("end")
    # Item 7: per-member stress scales and k_i = fl↑(k√2·i).
    for _ in range(300):
        i = 1.0 + (rng.next() % (1 << 40)) / 2.0 ** 38
        exact = Fr(K_SQRT2) * Fr(i)
        nearest = K_SQRT2 * i
        ki = next_up_f(nearest) if Fr(nearest) < exact else nearest
        lines.append("kint %s %s" % (hexf(i), hexf(ki)))
        fo = math.ldexp(1.0 + (rng.next() % 1000) / 999.0, (rng.next() % 60) - 30)
        mo = math.ldexp(1.0 + (rng.next() % 1000) / 999.0, (rng.next() % 60) - 30)
        area = math.ldexp(1.0 + (rng.next() % 1000) / 999.0, (rng.next() % 20) - 15)
        modulus = math.ldexp(1.0 + (rng.next() % 1000) / 999.0, (rng.next() % 20) - 20)
        for kk in (1.0, K_SQRT2, K_TWO_SQRT2, 4.0, ki):
            lines.append("stress %s %s %s %s %s %s" % tuple(hexf(x) for x in
                                                             (fo, mo, area, modulus, kk, fo / area + kk * (mo / modulus))))
    return lines


# ----------------------------------------------------------------------------
# K: R1's frozen references through the adapter (plan §12.1)
# ----------------------------------------------------------------------------
R1_FAMILIES = ("RF-CHAIN", "RF-SKEW", "RF-WEAK", "RF-FINITE", "RF-MECH", "RF-CANCEL")
AXIS_DOF = {"UX": 0, "UY": 1, "UZ": 2, "RX": 3, "RY": 4, "RZ": 5}


def r1_cases():
    """The K4 cases of R1, in references.json order (RF-CANCEL's UDL cases are W1b's)."""
    ref = json.loads(R1_JSON.read_text())["cases"]
    out = []
    for cid, c in ref.items():
        if c["family"] not in R1_FAMILIES or "member_uniform_loads_N_per_m_global" in c["model"]:
            continue
        model = c["model"]
        if "generator" in model:
            r1.build_mech()
            case = next(x for x in r1.CASES if x["id"] == cid)
            model = r1.model_json(case["defn"], full=True)
        out.append((cid, c, model))
    return out


def r1_adapt(cid, c, model):
    """The kernel's binary64 model of an R1 case (plan §12.1) and its key maps."""
    basis = c.get("basis") or "intended"
    num = r1.parse_input
    dec = (lambda x: num(x)) if basis == "intended" else (lambda x: r1.rep(num(x)))
    names = list(model["nodes_m"])
    index = {n: k for k, n in enumerate(names)}
    m = new_model(cid, [tuple(float(num(x)) for x in model["nodes_m"][n]) for n in names])
    sections = {}
    for sid, sec in model["sections"].items():
        od, idd = dec(sec["OD"]), dec(sec["ID"])
        A = PI_Q * (od * od - idd * idd) / 4
        I = PI_Q * (od ** 4 - idd ** 4) / 64
        sections[sid] = dict(E=float(num(sec["E"])), G=float(num(sec["G"])), A=float(A), Iy=float(I),
                             Iz=float(I), J=2.0 * float(I))
    member_id = {}
    kmem = []
    for k, (mid, ni, nj, sid) in enumerate(model["members"]):
        xi, xj = m["nodes"][index[ni]], m["nodes"][index[nj]]
        d = [xj[0] - xi[0], xj[1] - xi[1], xj[2] - xi[2]]
        y = (1.0, 0.0, 0.0) if d[0] == 0.0 and d[1] == 0.0 else (0.0, 0.0, 1.0)
        add_member(m, k + 1, index[ni], index[nj], y=y, section=sections[sid])
        m["stations"].append(dict(id=k + 1, member=k + 1, t=0.5))
        member_id[mid] = k + 1
        length = math.sqrt(d[0] * d[0] + d[1] * d[1] + d[2] * d[2])
        sec = sections[sid]
        kmem.append((k + 1, (sec["G"] * sec["J"]) / length, (sec["E"] * sec["A"]) / length))
    spring_key = {}
    next_id = 1
    for node, sup in model.get("supports", {}).items():
        n = index[node]
        for dof in sup.get("rigid", []):
            fix(m, n, (AXIS_DOF[dof],))
        springs = sup.get("springs", [])
        for kind in ("translation", "rotation"):
            mine = [(i, sp) for i, sp in enumerate(springs) if sp["kind"] == kind]
            dirs = [[num(x) for x in sp["direction"]] for _, sp in mine]
            axis_only = all(sum(1 for x in dv if x != 0) == 1 for dv in dirs)
            off = 0 if kind == "translation" else 3
            for (i, sp), dv in zip(mine, dirs):
                k = float(dec(sp["k"]))
                if axis_only:
                    comp = next(a for a in range(3) if dv[a] != 0)
                    m["springs"].append(dict(id=next_id, node=n, c=off + comp, k=k))
                    spring_key[(node, i)] = ("spr", next_id, off, comp)
                else:
                    m["dsprings"].append(dict(id=next_id, node=n, kind="t" if kind == "translation" else "r",
                                              n=tuple(float(x) for x in dv), k=k))
                    spring_key[(node, i)] = ("dspr", next_id, off, None)
                next_id += 1
    for node, ld in model.get("loads", {}).items():
        for key, off in (("F", 0), ("M", 3)):
            for a, x in enumerate(ld.get(key, ())):
                v = float(dec(x))
                if v != 0.0:
                    load(m, index[node], off + a, v)
    for k, (node, dof, x) in enumerate(model.get("load_contributions_in_authored_order", [])):
        load(m, index[node], AXIS_DOF[dof], float(dec(x)), "c%d" % k)

    def key_of(rk):
        """Our key for an R1 key, or None (a component the kernel does not publish)."""
        f = rk.split(".")
        if f[0] in ("u", "th", "R"):
            return "%s.%d.%d" % ("R" if f[0] == "R" else "u", index[f[1]], AXIS_DOF[f[2]])
        if f[0] in ("N", "T", "tw", "ext"):
            return "%s.%d" % (f[0], member_id[f[1]])
        if f[0] == "Mb":
            mid = member_id[f[1]]
            return "Mbs.%d" % mid if f[2] == "mid" else "Mb.%d.%s" % (mid, f[2])
        if f[0] == "S":
            kind_, sid, off, comp = spring_key[(f[1], int(f[2]))]
            a = "XYZ".index(f[3][1])
            if kind_ == "dspr":
                return "dspr.%d.%d" % (sid, off + a)
            return "spr.%d.%d" % (sid, off + a) if a == comp else None
        raise ValueError(rk)
    return m, kmem, key_of


def r1_lines():
    floors = json.loads(FLOOR_JSON.read_text())["cases"]
    lines = []
    for cid, c, model in r1_cases():
        m, kmem, key_of = r1_adapt(cid, c, model)
        lines += model_lines(m, expectations=False)[:-1]
        outcome = "refuse" if c["family"] == "RF-MECH" and "expected" not in c else "solve"
        lines.append("case %s %s %s" % (c["family"], c.get("basis") or "-", outcome))
        for mid, kt, ka in kmem:
            lines.append("kmem %d %s %s" % (mid, hexf(kt), hexf(ka)))
        rows = c.get("expected_represented") if c.get("basis") == "represented" else c.get("expected")
        scales = c.get("scales", {})
        for row in rows or []:
            rk, e, cls = row[0], r1.parse_input(row[1]), row[2]
            scale = r1.parse_input(row[3]) if c["family"] == "RF-CANCEL" else r1.parse_input(scales[cls]["value"])
            ours = key_of(rk)
            if ours is None:
                assert e == 0, (cid, rk)
                lines.append("zero %s" % rk)
                continue
            lines.append("ref %s %s %s %s %s" % (ours, hexf(float(e)), hexf(float(scale)), cls.split("@")[0], rk))
        for nc in c.get("negative_controls", []):
            if "defective_outcome" in nc:
                # An outcome control: the defect is an outcome, not values.
                lines.append("nco %s %d %s" % (nc["id"], int(bool(nc.get("discriminates"))),
                                               nc["defective_outcome"].replace(" ", "_")))
                continue
            vals = []
            for rk, v in sorted((nc.get("values") or {}).items()):
                ours = key_of(rk)
                if ours is not None:
                    vals.append("%s=%s" % (ours, hexf(float(r1.parse_input(v)))))
            lines.append("nc %s %d %s" % (nc["id"], int(bool(nc.get("discriminates"))), " ".join(vals) or "-"))
        fl = floors.get(cid, {})
        for rk, _cls, _ratio in fl.get("F_rec_scale_below" if c["family"] == "RF-CANCEL" else "F_scale_below", []):
            lines.append("floor %s" % key_of(rk))
        lines.append("end")
    return lines


def build(parts=None):
    files = {}
    summary = {}
    want = (lambda k: parts is None or k in parts)
    if want("sums"):
        t = sum_targeted()
        files["sum_targeted.txt"] = "\n".join(t) + "\n"
        sample, manifest = [], []
        for name, L, p, count, tag in SUM_STREAMS:
            seed = seed_of(tag)
            h, chunks = run_sum_stream(name, L, p, seed, count, sample)
            manifest.append("stream %s width %d precision %d seed %016x count %d chunk %d sha256 %s"
                            % (name, L, p, seed, count, SUM_CHUNK, h))
            manifest += ["chunk %s %d %s" % (name, i, d) for i, d in enumerate(chunks)]
            summary["sum_%s" % name] = h
        files["sum_differential.txt"] = "\n".join(manifest) + "\n"
        files["sum_differential_sample.txt"] = "\n".join(sample) + "\n"
    if want("ledger"):
        files["ledger.txt"] = "\n".join(ledger_lines()) + "\n"
    if want("streams"):
        sample, manifest = [], []
        for name, L, p, count, tag in STREAMS:
            seed = seed_of(tag)
            h, chunks = k3.run_stream(name, L, p, seed, count, sample)
            manifest.append("stream %s width %d precision %d seed %016x count %d chunk %d sha256 %s"
                            % (name, L, p, seed, count, k3.DIFF_CHUNK, h))
            manifest += ["chunk %s %d %s" % (name, i, d) for i, d in enumerate(chunks)]
            summary["stream_%s" % name] = h
        files["streams.txt"] = "\n".join(manifest) + "\n"
        files["streams_sample.txt"] = "\n".join(sample) + "\n"
    if want("formation"):
        files["formation.txt"] = "\n".join(formation_lines()) + "\n"
    if want("o8"):
        files["o8_states.txt"] = "\n".join(o8_lines()) + "\n"
    if want("classification"):
        files["classification.txt"] = "\n".join(classification_lines()) + "\n"
    if want("r1"):
        files["r1_cases.txt"] = "\n".join(r1_lines()) + "\n"
    if want("models"):
        lines = []
        by_name = {}
        for m in models() + routed_models():
            by_name[m["name"]] = m
            lines += model_lines(m)
        for name, operands, net in COMBOS:
            lines.append("combo %s %s" % (name, " ".join("%s:%s" % (hexf(c), mname) for c, mname in operands)))
            if net is not None:
                ex = solve_exact(by_name[net])
                for key in sorted(ex):
                    lines.append("expect %s %016x" % (key, to_f64_bits(ex[key])))
            lines.append("end")
        # NP-A's represented (stored binary64) answers, which W1 must not give.
        fixtures = json.loads(NI_FIXTURES.read_text())
        for row in fixtures["NP"]["A"]:
            if row["id"] in ("N05", "N06") and "stored_exact_root" in row:
                lines.append("npa %s root %s tip %s" % (row["id"], hexf(Fr(row["stored_exact_root"])),
                                                        hexf(Fr(row["stored_exact_tip"]))))
            elif row["id"] in ("N05", "N06"):
                lines.append("npa %s status %s" % (row["id"], json.dumps(row["stored_status"]).replace(" ", "_")))
        files["models.txt"] = "\n".join(lines) + "\n"
    return files, summary


def main():
    args = sys.argv[1:]
    check = "--check" in args
    only = [a[len("--only="):] for a in args if a.startswith("--only=")]
    files, summary = build(set(only[0].split(",")) if only else None)
    status = 0
    if not only:
        names = sorted(files)
        files["SHA256SUMS"] = "".join("%s  %s\n" % (hashlib.sha256(files[n].encode()).hexdigest(), n)
                                      for n in names)
    for name, text in files.items():
        path = HERE / name
        if check:
            same = path.exists() and path.read_text() == text
            print("%s %s" % ("OK  " if same else "DIFF", name))
            status |= 0 if same else 1
        else:
            path.write_text(text)
    if only and not check:
        # A partial run refreshes SHA256SUMS from the files on disk.
        sums = HERE / "SHA256SUMS"
        names = sorted({line.split()[1] for line in sums.read_text().splitlines()} | set(files))
        sums.write_text("".join("%s  %s\n" % (sha256_file(HERE / n), n) for n in names))
    summary["inputs"] = {str(p.relative_to(P_ROOT)): d for p, d in PINNED.items()}
    print(json.dumps(summary, indent=1, sort_keys=True))
    return status


if __name__ == "__main__":
    sys.exit(main())
