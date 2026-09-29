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

D1 revision 5a.3 (R7 §5; ROOT's A3-0 rulings; this generator is the bit
oracle, DS1's emu7 the selection-level cross-check), checkpoint A3a:
- `directed.txt`: R7 7b's directed roundings (nearest, then one step when on
  the wrong side of the exact Fraction) and the least binary64 not below x;
- `models5a3.txt`: DS1's models4-6 controls used at A3a, K4's BLOCK-PRESC and
  R1's RF-LARGE frames at 10 members (through the adapter of section K);
- `scale.txt` (E-UNIT): K4 at each precision emulated in K4's own order
  (`emulate`: formation with g, assembly, RCM, the factor and its screens, the
  condition screen with est_c per block, the solve and the residual gate with
  its early stop on the 64-bit approximate ratio); g, the bounded operator Ā,
  E_q, E(body, kind), ê and Φ;
- `bounds.txt` (E-UC): at 256, 512 and 1024, factor()'s L and D digest, the
  blocks and their data flags, est_c, U_c, N_L,c, t_c, Uc_c, the forced shift
  (every block with est_c > 0) and ‖K̃_c⁻¹‖₁: exact (rational, blocks of at
  most 40 DOFs) or a certified upper bound (RF-LARGE: ‖X‖₁/(1 − ‖R‖₁) with X
  an approximate inverse at 3,072 bits and R = I − K̃X formed exactly);
- `profiles.txt`: V4's F2 family, DS1's low-precision stress (20,000 draws,
  every R7-M27 killer kept) and SD-G5's searched boundary profiles, with
  exact norms.

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
# Q10: the canonical encodings built independently (plan §9): K4SRC, K4STF
# and K4LED for a representative set, and combination ledgers
# ----------------------------------------------------------------------------
def u32b(v):
    return struct.pack("<I", v)


def dofb(node, c):
    return u32b(node) + bytes([c])


def f64b(x):
    return struct.pack("<d", x)


def source_bytes(m, stiffness=False):
    """source.rs `encoding` (K4SRC) or `stiffness_encoding` (K4STF), from the
    model's own lists in their canonical order."""
    out = b"K4STF\x01" if stiffness else b"K4SRC\x01"
    out += u32b(len(m["nodes"])) + b"".join(f64b(c) for pt in m["nodes"] for c in pt)
    members = sorted(m["members"], key=lambda x: x["id"])
    out += u32b(len(members))
    for mm in members:
        out += u32b(mm["id"]) + u32b(mm["i"]) + u32b(mm["j"])
        out += b"".join(f64b(mm[k]) for k in ("E", "G", "A", "Iy", "Iz", "J"))
        out += b"".join(f64b(c) for c in mm["y"])
    springs = sorted(m["springs"], key=lambda x: x["id"])
    out += u32b(len(springs))
    for sp in springs:
        out += u32b(sp["id"]) + dofb(sp["node"], sp["c"]) + f64b(sp["k"])
    dsprings = sorted(m["dsprings"], key=lambda x: x["id"])
    out += u32b(len(dsprings))
    for sp in dsprings:
        out += u32b(sp["id"]) + u32b(sp["node"]) + bytes([0 if sp["kind"] == "t" else 1])
        out += b"".join(f64b(c) for c in sp["n"]) + f64b(sp["k"])
    constraints = sorted(m["constraints"], key=lambda x: (x["node"], x["c"]))
    out += u32b(len(constraints))
    for c in constraints:
        out += dofb(c["node"], c["c"]) + (b"" if stiffness else f64b(c["v"]))
    if stiffness:
        return out
    loads = sorted(m["loads"], key=lambda l: (l["node"], l["c"], l["src"].encode(),
                                              struct.unpack("<Q", f64b(l["v"]))[0]))
    out += u32b(len(loads))
    for l in loads:
        out += dofb(l["node"], l["c"]) + u32b(len(l["src"].encode())) + l["src"].encode() + f64b(l["v"])
    stations = sorted(m["stations"], key=lambda x: x["id"])
    out += u32b(len(stations))
    for st in stations:
        out += u32b(st["id"]) + u32b(st["member"]) + f64b(st["t"])
    supports = sorted(m["supports"], key=lambda x: x["id"])
    out += u32b(len(supports))
    for g in supports:
        out += u32b(g["id"]) + u32b(g["node"]) + bytes(int(bool(x)) for x in g["r"])
        for ids in (sorted(set(g["springs"])), sorted(set(g["dsprings"]))):
            out += u32b(len(ids)) + b"".join(u32b(i) for i in ids)
    return out


def ledger_bytes(terms):
    """ledger.rs `encoding` (K4LED): {global DOF: exact net}."""
    out = b"K4LED\x01" + u32b(len(terms))
    for dof in sorted(terms):
        net = terms[dof]
        out += dofb(dof // 6, dof % 6)
        if net == 0:
            out += bytes([0]) + struct.pack("<q", 0) + u32b(0)
            continue
        a = abs(net)
        num, den = a.numerator, a.denominator
        assert den & (den - 1) == 0
        e = -(den.bit_length() - 1)
        while num % 2 == 0:
            num //= 2
            e += 1
        limbs = []
        while num:
            limbs.append(num & ((1 << 64) - 1))
            num >>= 64
        out += bytes([int(net < 0)]) + struct.pack("<q", e) + u32b(len(limbs))
        out += b"".join(struct.pack("<Q", l) for l in limbs)
    return out


def ledger_terms(operands):
    """The exact nets of Σ c·(case loads), per loaded DOF."""
    terms = {}
    for factor, m in operands:
        for l in m["loads"]:
            g = 6 * l["node"] + l["c"]
            terms[g] = terms.get(g, Fr(0)) + Fr(factor) * Fr(l["v"])
    return terms


ENCODED_MODELS = ("N01", "N05", "N06", "SKEW6-K1E-12", "PRESCRIBED", "DIRECTIONAL-SPAN", "B1-C-A", "B1-L")


def encoding_lines():
    by_name = {m["name"]: m for m in models()}
    lines = []

    def emit(kind, name, data):
        lines.append("enc %s %s %s %s" % (kind, name, hashlib.sha256(data).hexdigest(), data.hex()))
    for name in ENCODED_MODELS:
        m = by_name[name]
        emit("src", name, source_bytes(m))
        emit("stf", name, source_bytes(m, stiffness=True))
        emit("led", name, ledger_bytes(ledger_terms([(1.0, m)])))
    for name, operands, _ in COMBOS:
        if name == "PRECISION-RULE":
            continue
        emit("cled", name, ledger_bytes(ledger_terms([(f, by_name[n]) for f, n in operands])))
    return lines


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


# ----------------------------------------------------------------------------
# D1 revision 5a.3 (R7 §5), checkpoint A3a: directed rounding, g, the bounded
# operator Ā, the formation scale E, the free-free blocks, est_c, Uc, the
# shifted factorization and exact block norms. Every rounding mirrors K4's
# order (ROOT's A3-0 ruling Q1: this generator is the bit oracle; DS1's emu7
# is the selection-level cross-check).
# ----------------------------------------------------------------------------
def next_p(r, p, up):
    """The adjacent p-bit value of a nonzero p-bit Fraction r."""
    e = k3.floor_log2(abs(r))
    away = Fr(2) ** (e - p + 1)
    toward = Fr(2) ** (e - p) if abs(r) == Fr(2) ** e else away
    if r > 0:
        return r + away if up else r - toward
    return r + toward if up else r - away


def ru(x, p):
    """x rounded upward at p bits: nearest, then one step when below x."""
    r = rp(x, p)
    return next_p(r, p, True) if r < x else r


def rd(x, p):
    """x rounded downward at p bits: nearest, then one step when above x."""
    r = rp(x, p)
    return next_p(r, p, False) if r > x else r


def fl_up(x):
    """The least binary64 not below a Fraction x >= 0 (+inf beyond range)."""
    try:
        y = float(x)
    except OverflowError:
        return math.inf
    if y != math.inf and Fr(y) < x:
        y = math.nextafter(y, math.inf)
    return y


def rand_pbits(rng, p, lo, hi, signed=True):
    m = rng.getrandbits(p) | (1 << (p - 1))
    e = rng.randint(lo, hi)
    v = Fr(m) * Fr(2) ** (e - p + 1)
    return -v if signed and rng.random() < 0.5 else v


DIRECTED_P = (3, 10, 53, 128, 256, 512, 1024)


def directed_lines():
    """R7 7b's directed roundings: add, sub, mul, div (b > 0) and binary64_up."""
    rng = __import__("random").Random(seed_of("K4DIR5A3"))
    lines = []

    def emit(op, p, a, b, x):
        L = width_of(p)
        for name, f in (("up", ru), ("down", rd)):
            lines.append("dir %s %d %d %s %s %s %s" % (op, p, L, name, W_of(a, L).token(), W_of(b, L).token(),
                                                        W_of(f(x, p), L).token()))

    for p in DIRECTED_P:
        for _ in range(24):
            a = rand_pbits(rng, p, -40, 40)
            b = rand_pbits(rng, p, -40, 40)
            emit("add", p, a, b, a + b)
            emit("sub", p, a, b, a - b)
            emit("mul", p, a, b, a * b)
            b = abs(b)
            emit("div", p, a, b, a / b)
        # Targeted: exact results, and values one bit either side of a power of two.
        for k in (-3, 0, 5):
            two = Fr(2) ** k
            tiny = Fr(2) ** (k - p - 3)
            emit("add", p, two, tiny, two + tiny)
            emit("sub", p, two, tiny, two - tiny)
            emit("sub", p, tiny, two, tiny - two)
            emit("add", p, two, two, 2 * two)
            emit("mul", p, two, Fr(3) if p >= 2 else two, two * (3 if p >= 2 else 1))
            emit("div", p, two, Fr(2) ** 3, two / 8)
            if p >= 2:
                emit("div", p, Fr(1), Fr(3), Fr(1, 3))
                emit("div", p, -Fr(1), Fr(3), -Fr(1, 3))
    # binary64_up of nonnegative wide values.
    for _ in range(60):
        p = rng.choice((128, 256, 512, 1024))
        x = abs(rand_pbits(rng, p, -1100, 1100))
        L = width_of(p)
        y = fl_up(x)
        lines.append("up64 %d %d %s %016x" % (p, L, W_of(x, L).token(), b64(y)))
    for x, p in ((Fr(0), 128), (Fr(2) ** -1074, 128), (Fr(2) ** -1075, 128), (Fr(3) * Fr(2) ** -1076, 256),
                 (Fr(2) ** 1024, 256), (Fr(2) ** 1023 * (2 - Fr(2) ** -52), 128),
                 (Fr(2) ** 1023 * (2 - Fr(2) ** -60), 128), (Fr(1) + Fr(2) ** -60, 128)):
        L = width_of(p)
        lines.append("up64 %d %d %s %016x" % (p, L, W_of(x, L).token(), b64(fl_up(x))))
    return lines


# ---- the 5a.3 models (DS1's models4-6 and K4's own), in models.txt's format

UNIT_SECTION = dict(E=1.0, G=1.0, A=1.0, Iy=1.0, Iz=1.0, J=1.0)


def hh_fool(mexp, loaded, name=None):
    """V4's HH-FOOL frame (DS1's models5.py, ported)."""
    H = [(0, 0, 0), (1, -1, 0), (2, -2, 0), (3, -3, 0)]
    nv = 8
    V = [(10 + k, 5, 0) for k in range(nv)]
    nodes = [H[0], V[0], H[1], V[1], H[2], V[2], H[3]] + V[3:]
    hn = [0, 2, 4, 6]
    vn = [1, 3, 5] + list(range(7, 7 + nv - 3))
    md = new_model(name or ("HH-FOOL-m%d%s" % (mexp, "-LOADED" if loaded else "")), nodes)
    sec_h = dict(UNIT_SECTION, Iy=2.0 ** -mexp, Iz=2.0 ** -mexp)
    mid = 1
    for a, b in ((0, 1), (2, 3), (0, 2), (1, 3)):
        add_member(md, mid, hn[a], hn[b], y=(0, 0, 1), section=sec_h)
        mid += 1
    for k in range(nv - 1):
        add_member(md, mid, vn[k], vn[k + 1], y=(0, 0, 1), section=dict(UNIT_SECTION, A=(1.0 if k % 2 == 0 else 3.0)))
        mid += 1
    comp = {hn[0]: 0, hn[1]: 1, hn[2]: 1, hn[3]: 0}
    for nd in range(len(nodes)):
        keep = comp.get(nd, 0)
        fix(md, nd, [c for c in range(6) if c != keep])
    spring(md, 1, vn[0], 0, 0.25)
    spring(md, 2, vn[-1], 0, 0.25)
    if loaded:
        load(md, vn[nv // 2], 0, 1.0)
        load(md, hn[0], 0, 1.0)
    return md


def models5a3():
    """The 5a.3 models of checkpoint A3a (DS1's models4/5/6 ported, and K4's
    BLOCK-PRESC); A3b adds the rest of R7 §7's controls."""
    out = []
    base = {m["name"]: m for m in models()}
    m = new_model("CHARGE-SLENDER", [(0, 0, 0), (1, 0, 0)])
    I = 2.0 ** -180
    add_member(m, 1, 0, 1, section=dict(UNIT_SECTION, Iy=I, Iz=I))
    fix(m, 0, range(6))
    load(m, 1, 1, 1.0)
    out.append(m)
    for mexp, loaded in ((40, False), (40, True), (100, False), (100, True)):
        out.append(hh_fool(mexp, loaded))
    # HH-SLENDER-m40: HH-FOOL-m40 unloaded plus CHARGE-SLENDER as a third body.
    m = hh_fool(40, False, name="HH-SLENDER-m40")
    a = len(m["nodes"])
    m["nodes"] += [(100.0, 0.0, 0.0), (101.0, 0.0, 0.0)]
    add_member(m, 50, a, a + 1, section=dict(UNIT_SECTION, Iy=2.0 ** -180, Iz=2.0 ** -180))
    fix(m, a, range(6))
    load(m, a + 1, 1, 1.0)
    out.append(m)
    # THETA-ZERO-BODY and G-FIXED-MEMBER: N05 plus an extra body (V4-U2).
    for name in ("THETA-ZERO-BODY", "G-FIXED-MEMBER"):
        m = json.loads(json.dumps(base["N05"]))
        m["name"] = name
        m["nodes"] = [tuple(p) for p in m["nodes"]]
        for mm in m["members"]:
            mm["y"] = tuple(mm["y"])
        a = len(m["nodes"])
        if name == "THETA-ZERO-BODY":
            m["nodes"] += [(50.0, 0.0, 0.0), (51.0, 0.0, 0.0)]
            add_member(m, 900, a, a + 1, section=dict(UNIT_SECTION, Iy=2.0 ** -400, Iz=2.0 ** -400))
            fix(m, a, range(6))
        else:
            m["nodes"] += [(60.0, 0.0, 0.0), (61.0, 0.0, 0.0)]
            add_member(m, 901, a, a + 1, y=(1.0, 2.0 ** -300, 0.0), section=UNIT_SECTION)
            fix(m, a, range(6))
            fix(m, a + 1, range(6))
        out.append(m)
    # THETA-STUB (DS1, R5) and THETA-STUB-COUPLED (DS1, R6).
    m = new_model("THETA-STUB", [(0, 0, 0), (1, 0, 0), (0, 1, 0)])
    add_member(m, 1, 0, 1, section=UNIT_SECTION)
    add_member(m, 2, 0, 2, y=(0.0, 0.0, 1.0), section=dict(UNIT_SECTION, A=2.0 ** 50, Iy=2.0 ** -204, Iz=2.0 ** -204))
    fix(m, 0, range(6))
    fix(m, 2, (0, 1, 3, 4, 5))
    load(m, 1, 1, 1.0)
    out.append(m)
    m = new_model("THETA-STUB-COUPLED", [(0, 0, 0), (1, 0, 0), (2, 1, 0), (2, 0, 0)])
    add_member(m, 1, 0, 1, section=UNIT_SECTION)
    add_member(m, 3, 1, 3, section=UNIT_SECTION)
    add_member(m, 2, 3, 2, y=(0.0, 0.0, 1.0), section=dict(UNIT_SECTION, A=2.0 ** 50, Iy=2.0 ** -204, Iz=2.0 ** -204))
    fix(m, 0, range(6))
    fix(m, 3, (0, 1, 2))
    fix(m, 2, (0, 1, 3, 4, 5))
    load(m, 1, 1, 1.0)
    out.append(m)
    # G-PRESC-MEMBER (DS1, R5): g = 2^245 on a member with a nonzero prescription.
    m = new_model("G-PRESC-MEMBER", [(0, 0, 0), (1, 0, 0), (0, 0, 1)])
    add_member(m, 1, 0, 1, section=UNIT_SECTION)
    add_member(m, 2, 0, 2, y=(2.0 ** -244, 0.0, 1.0), section=UNIT_SECTION)
    fix(m, 0, range(6))
    fix(m, 2, (1, 2, 3, 4, 5))
    fix(m, 2, (0,), 2.0 ** -300)
    load(m, 1, 1, 1.0)
    out.append(m)
    # BLOCK-PRESC (K4, SD-G5): a loaded unit cantilever, and a second body whose
    # only free DOF uy(3) is coupled by the pattern (not numerically: the member
    # is along x) to the prescribed ux(2) = 1e-3, so its state is exactly zero
    # and its block carries data only through 7a's prescribed-coupling rule.
    m = new_model("BLOCK-PRESC", [(0, 0, 0), (1, 0, 0), (0, 5, 0), (1, 5, 0)])
    add_member(m, 1, 0, 1, section=UNIT_SECTION)
    add_member(m, 2, 2, 3, section=UNIT_SECTION)
    fix(m, 0, range(6))
    load(m, 1, 1, 1.0)
    fix(m, 2, (0,), 1e-3)
    fix(m, 2, (1, 2, 3, 4, 5))
    fix(m, 3, (0, 2, 3, 4, 5))
    out.append(m)
    return out


LARGE_A3A = ("RF-LARGE-CHAIN-n00010-AX", "RF-LARGE-CHAIN-n00010-ROT", "RF-LARGE-TREE-n00010-AX",
             "RF-LARGE-TREE-n00010-ROT", "RF-LARGE-CONT-n00010-AX", "RF-LARGE-CONT-n00010-ROT")


def large_models(ids):
    """R1's RF-LARGE frames through K4's adapter (plan §12.1)."""
    ref = json.loads(R1_JSON.read_text())["cases"]
    out = []
    for cid in ids:
        c = ref[cid]
        m, _, _ = r1_adapt(cid, c, c["model"])
        out.append(m)
    return out


# ---- the emulation of K4 at one precision (shared stages, condition, solve)

def canonical(model):
    """K4's canonical lists (source.rs): by id, constraints by DOF."""
    m = dict(model)
    m["members"] = sorted(model["members"], key=lambda x: x["id"])
    m["springs"] = sorted(model["springs"], key=lambda x: x["id"])
    m["dsprings"] = sorted(model["dsprings"], key=lambda x: x["id"])
    m["stations"] = sorted(model["stations"], key=lambda x: x["id"])
    m["supports"] = sorted(model["supports"], key=lambda x: x["id"])
    m["constraints"] = sorted(model["constraints"], key=lambda c: 6 * c["node"] + c["c"])
    return m


def g_exp_em(yr, yc):
    yy = sum((c * c for c in yr), Fr(0))
    cc = sum((c * c for c in yc), Fr(0))
    k = 0
    while Fr(4) ** k * cc < yy:
        k += 1
    return k


def form_member_g(nodes, m, p):
    """form_member_em plus g (R7 §4.1.6.2 item 2), from the same yc."""
    op = form_member_em(nodes, m, p)
    F = Fr
    xi, xj = nodes[m["i"]], nodes[m["j"]]

    def dot(a, b):
        return rp(sum((x * y for x, y in zip(a, b)), Fr(0)), p)
    d = [rp(F(xj[k]) - F(xi[k]), p) for k in range(3)]
    n = sqrt_p(dot(d, d), p)
    ex = [rp(c / n, p) for c in d]
    yr = [F(c) for c in m["y"]]
    proj = dot(yr, ex)
    yc = [rp(yr[k] - proj * ex[k], p) for k in range(3)]
    assert ex == op["axes"][0]
    op["g"] = g_exp_em(yr, yc)
    return op


def bounded_block_em(op, p):
    """assemble.rs `bounded_block` (emu7's `element_bounded`)."""
    inv = abs(op["inv"])
    Bb = [[Fr(0)] * 12 for _ in range(6)]
    for k in range(3):
        Bb[0][k] = Bb[0][6 + k] = Fr(1)
        Bb[1][3 + k] = Bb[1][9 + k] = Fr(1)
        for row, rot in ((2, 3), (3, 9), (4, 3), (5, 9)):
            Bb[row][k] = Bb[row][6 + k] = inv
            Bb[row][rot + k] = Fr(1)
    z, y = abs(op["bz"]), abs(op["by"])
    drows = [[(0, abs(op["axial"]))], [(1, abs(op["torsion"]))], [(2, 4 * z), (3, 2 * z)], [(2, 2 * z), (3, 4 * z)],
             [(4, 4 * y), (5, 2 * y)], [(4, 2 * y), (5, 4 * y)]]
    DB = [[rp(sum((c * Bb[s][col] for s, c in drows[r]), Fr(0)), p) for col in range(12)] for r in range(6)]
    g = Fr(2) ** op["g"]
    return [[rp(sum((Bb[r][a] * DB[r][b] for r in range(6)), Fr(0)), p) * g for b in range(12)] for a in range(12)]


def member_dofs_em(m):
    return [6 * m["i"] + k for k in range(6)] + [6 * m["j"] + k for k in range(6)]


def emulate(model, p, state=True):
    """K4 at p: formation (with g), assembly, the ordering, the factor and its
    screens, the condition screen with est_c per block, and (state=True) the
    solve with the residual gate. Returns a dict; `stop` names a failed stage."""
    model = canonical(model)
    q = 1024 if p == 1024 else p + 64
    rnd = lambda x: rp(x, p)
    nodes = model["nodes"]
    members = [form_member_g(nodes, m, p) for m in model["members"]]
    contrib = {}
    for m, op in zip(model["members"], members):
        dofs = member_dofs_em(m)
        for a in range(12):
            for b in range(12):
                if dofs[a] <= dofs[b]:
                    contrib.setdefault((dofs[a], dofs[b]), []).append(op["ke"][(min(a, b), max(a, b))])
    for s in model["springs"]:
        d = 6 * s["node"] + s["c"]
        contrib.setdefault((d, d), []).append(Fr(s["k"]))
    dblocks = []
    for s in model["dsprings"]:
        blk = directional_em(s, p)
        base = 6 * s["node"] + kind_offset(s["kind"])
        dblocks.append((base, blk))
        for a in range(3):
            for b in range(a, 3):
                contrib.setdefault((base + a, base + b), []).append(blk[a][b])
    K = {rc: rnd(sum(vs, Fr(0))) for rc, vs in contrib.items()}
    n = 6 * len(nodes)
    prescribed = {6 * c["node"] + c["c"]: Fr(c["v"]) for c in model["constraints"]}
    free = [g for g in range(n) if g not in prescribed]
    position = {g: a for a, g in enumerate(free)}
    pattern = {g: set() for g in range(n)}
    for (r, c) in K:
        pattern[r].add(c)
        pattern[c].add(r)
    get_k = lambda M, r, c: M.get((min(r, c), max(r, c)))
    out = dict(model=model, K=K, members=members, dblocks=dblocks, free=free, position=position, pattern=pattern,
               prescribed=prescribed, n=n, p=p)
    nf = len(free)
    adjacency = [[position[c] for c in sorted(pattern[g]) if c in position and c != g] for g in free]
    # Blocks (7a): components of the free-free pattern, numbered by least position.
    blk = [-1] * nf
    blocks = []
    for s0 in range(nf):
        if blk[s0] >= 0:
            continue
        bid = len(blocks)
        blk[s0] = bid
        stack, comp = [s0], []
        while stack:
            v = stack.pop()
            comp.append(v)
            for w in adjacency[v]:
                if blk[w] < 0:
                    blk[w] = bid
                    stack.append(w)
        blocks.append(sorted(comp))
    out.update(blocks=blocks, blk=blk)
    for g in free:
        d = get_k(K, g, g)
        if d is None or d == 0:
            out["stop"] = "ZeroDiagonal"
            return out
        if d < 0:
            out["stop"] = "NegativeEnergy"
            return out
    order = rcm_em(adjacency)
    rank = [0] * nf
    for kk, a in enumerate(order):
        rank[a] = kk
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
    scaled = [dict(r) for r in rows]
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
        mm = 2 * (i - first[i]) + 2
        if not (pivot * (2 ** p - mm) - 64 * mm * canc > 0):
            out["stop"] = "Pivot"
            return out
        rows[i][i] = pivot
    out.update(order=order, rank=rank, first=first, scale=scale, rows=rows, scaled=scaled, get=get)

    def solve_scaled(b):
        x = [b[order[i]] for i in range(nf)]
        for i in range(nf):
            for j in range(first[i], i):
                x[i] = rnd(x[i] - rnd(get(i, j) * x[j]))
        for i in range(nf):
            x[i] = rnd(x[i] / get(i, i))
        for i in reversed(range(nf)):
            v = x[i]
            for j in range(first[i], i):
                x[j] = rnd(x[j] - rnd(get(i, j) * v))
        res = [Fr(0)] * nf
        for i in range(nf):
            res[order[i]] = x[i]
        return res

    def solve(bv):
        y = solve_scaled([bv[a] * Fr(2) ** scale[a] for a in range(nf)])
        return [y[a] * Fr(2) ** scale[a] for a in range(nf)]
    out["solve"] = solve
    # The condition screen, with est_c per block observed at p >= 256.
    est_blk = [Fr(0)] * len(blocks)

    def observe(xv, yv):
        for b, pl in enumerate(blocks):
            xs = rnd(sum((abs(xv[a]) for a in pl), Fr(0)))
            if xs == 0:
                continue
            ys = rnd(sum((abs(yv[a]) for a in pl), Fr(0)))
            est_blk[b] = max(est_blk[b], rnd(ys / xs))
    if nf > 0:
        norm = Fr(0)
        for a, g in enumerate(free):
            col = rnd(sum((abs(get_k(K, g, c)) * Fr(2) ** (scale[a] + scale[position[c]])
                           for c in pattern[g] if c in position), Fr(0)))
            norm = max(norm, col)
        one = Fr(1)
        x = [rnd(Fr(1, nf))] * nf
        est = Fr(0)
        prev = nf
        for _ in range(5):
            y = solve_scaled(x)
            observe(x, y)
            cur = rnd(sum((abs(v) for v in y), Fr(0)))
            if cur <= est and est != 0:
                break
            est = cur
            signs = [one if v >= 0 else -one for v in y]
            z = solve_scaled(signs)
            observe(signs, z)
            j = 0
            for i in range(1, nf):
                if abs(z[i]) >= abs(z[j]):
                    j = i
            dot = rnd(sum((z[i] * x[i] for i in range(nf)), Fr(0)))
            if abs(z[j]) <= dot or j == prev:
                break
            prev = j
            x = [Fr(0)] * nf
            x[j] = one
        alt = []
        for i in range(nf):
            mag = one if nf == 1 else rnd(Fr(nf - 1 + i, nf - 1))
            alt.append(mag if i % 2 == 0 else -mag)
        y = solve_scaled(alt)
        observe(alt, y)
        total = rnd(sum((abs(v) for v in y), Fr(0))) * 2
        alternative = rnd(total / (3 * nf))
        est = max(est, alternative)
        product = rnd(norm * est)
        if product == 0 or product >= Fr(2) ** (p - 1):
            out["stop"] = "Condition"
            return out
    out["est_blk"] = est_blk
    if not state:
        return out
    # The solve and the residual gate (the coalesced test, the early stop on
    # K4's 64-bit approximate worst ratio).
    # The residual system re-formed at q from the primitives (K at the ceiling).
    Kq = K if q == p else assemble_em(model, q)[0]
    ledger = {}
    nonzero = set()
    for l in model["loads"]:
        g = 6 * l["node"] + l["c"]
        ledger[g] = ledger.get(g, Fr(0)) + Fr(l["v"])
        if l["v"] != 0:
            nonzero.add(g)
    out.update(ledger=ledger, nonzero_terms=nonzero)
    rhs = []
    for g in free:
        v = ledger.get(g, Fr(0))
        for c in pattern[g]:
            if c in prescribed:
                v -= get_k(K, g, c) * prescribed[c]
        rhs.append(rnd(v))
    u_free = solve(rhs) if nf else []
    u = [prescribed.get(g, Fr(0)) for g in range(n)]
    corrections = 0
    prior = math.inf
    while True:
        for a, g in enumerate(free):
            u[g] = u_free[a]
        passes, residuals, worst = True, [], 0.0
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
            mm = 2 * count + 2
            num = abs(r) * (2 ** p - mm)
            den = 64 * mm * d
            passes = passes and num <= den
            nv = rp(num, 64)
            if nv == 0:
                ratio = 0.0
            else:
                dv = rp(den, 64)
                ratio = math.inf if dv == 0 else float(rp(nv / dv, 64))
            worst = max(worst, ratio)
            residuals.append(rnd(r))
        if passes:
            break
        if corrections == 3 or worst >= prior:
            out["stop"] = "ResidualGate"
            return out
        prior = worst
        delta = solve(residuals)
        u_free = [rnd(xv + dv) for xv, dv in zip(u_free, delta)]
        corrections += 1
    out.update(u=list(u), corrections=corrections)
    return out


def bodies_em(model):
    """K4's body numbering (source.rs): components of the member graph,
    numbered by their least node."""
    nn = len(model["nodes"])
    parent = list(range(nn))

    def root(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x
    for m in model["members"]:
        a, b = root(m["i"]), root(m["j"])
        if a != b:
            lo, hi = min(a, b), max(a, b)
            parent[hi] = lo
    body_of_root, out, count = {}, [], 0
    for node in range(nn):
        r = root(node)
        if r not in body_of_root:
            body_of_root[r] = count
            count += 1
        out.append(body_of_root[r])
    return out, count


def body_extent_em(coords):
    if not coords:
        return 0.0
    d = []
    for a in range(3):
        vals = [c[a] for c in coords]
        d.append(max(vals) - min(vals))
    return math.sqrt((d[0] * d[0] + d[1] * d[1]) + d[2] * d[2])


def abar_em(em):
    """Ā at p: {(r, c): value}, both triangles, one exact sum per entry."""
    p, model = em["p"], em["model"]
    blocks = [bounded_block_em(op, p) for op in em["members"]]
    contrib = {}
    for m, A in zip(model["members"], blocks):
        dofs = member_dofs_em(m)
        for a in range(12):
            for b in range(12):
                contrib.setdefault((dofs[a], dofs[b]), []).append(A[a][b])
    for s in model["springs"]:
        d = 6 * s["node"] + s["c"]
        contrib.setdefault((d, d), []).append(abs(Fr(s["k"])))
    for base, blk in em["dblocks"]:
        for a in range(3):
            for b in range(3):
                contrib.setdefault((base + a, base + b), []).append(abs(blk[a][b]))
    return {rc: rp(sum(vs, Fr(0)), p) for rc, vs in contrib.items()}


def formation_scale_em(em, abar, w, ledger, stage):
    """verify.rs `formation_scale`: E_q per layout row (None for the others),
    each stage one exact sum rounded by `stage`."""
    model = em["model"]
    nn = len(model["nodes"])
    out = [None] * (6 * nn + nn)
    qs, acts_all = [], []
    for m, op in zip(model["members"], em["members"]):
        dofs = member_dofs_em(m)
        blockv = [stage(sum((w[dofs[3 * b + c]] for c in range(3)), Fr(0))) for b in range(4)]
        dd = lambda idx: blockv[idx // 3]
        inv = abs(op["inv"])
        e = [stage(dd(6) + dd(0)), stage(dd(9) + dd(3))]
        for rot, tr in ((5, 1), (11, 1), (4, 2), (10, 2)):
            e.append(stage(dd(rot) + inv * dd(tr) + inv * dd(tr + 6)))
        z, y = abs(op["bz"]), abs(op["by"])
        coeffs = [[(abs(op["axial"]), 0)], [(abs(op["torsion"]), 1)], [(4 * z, 2), (2 * z, 3)], [(2 * z, 2), (4 * z, 3)],
                  [(4 * y, 4), (2 * y, 5)], [(2 * y, 4), (4 * y, 5)]]
        Q = [stage(sum((c * e[at] for c, at in t), Fr(0))) for t in coeffs]
        vy = stage(inv * Q[2] + inv * Q[3])
        vz = stage(inv * Q[4] + inv * Q[5])
        g = Fr(2) ** op["g"]
        acts = [x * g for x in (Q[0], vy, vz, Q[1], Q[4], Q[2], Q[0], vy, vz, Q[1], Q[5], Q[3])]
        qs.append(Q)
        acts_all.append(acts)
    for acts in acts_all:
        out.extend(acts)
    ids = [mm["id"] for mm in model["members"]]
    for st in model["stations"]:
        k = ids.index(st["member"])
        Q, acts = qs[k], acts_all[k]
        g = Fr(2) ** em["members"][k]["g"]
        t = abs(Fr(st["t"]))
        out.extend([acts[6], acts[7], acts[8], acts[9], stage(t * Q[5] + t * Q[4] + Q[4]) * g,
                    stage(t * Q[3] + t * Q[2] + Q[2]) * g])
    spring_e = {}
    for s in model["springs"]:
        v = stage(abs(Fr(s["k"])) * w[6 * s["node"] + s["c"]])
        spring_e[s["id"]] = (s["c"], v)
        out.append(v)
    dir_e = {}
    for s, (base, blk) in zip(model["dsprings"], em["dblocks"]):
        comps = [stage(sum((abs(blk[a][b]) * w[base + b] for b in range(3)), Fr(0))) for a in range(3)]
        dir_e[s["id"]] = (kind_offset(s["kind"]), comps)
        out.extend(comps)
    reaction_e = {}
    for c in model["constraints"]:
        g = 6 * c["node"] + c["c"]
        f = abs(ledger.get(g, Fr(0))) if ledger is not None else Fr(0)
        v = stage(f + sum((abar.get((g, j), Fr(0)) * w[j] for j in em["pattern"][g]), Fr(0)))
        reaction_e[g] = v
        out.append(v)
    for grp in model["supports"]:
        for rng_ in (range(0, 3), range(3, 6)):
            acc = Fr(0)
            for c in rng_:
                g = 6 * grp["node"] + c
                if grp["r"][c]:
                    acc += reaction_e.get(g, Fr(0))
                for sid in grp["springs"]:
                    comp, v = spring_e[sid]
                    if comp == c:
                        acc += v
                for sid in grp["dsprings"]:
                    off, comps = dir_e[sid]
                    if off <= c < off + 3:
                        acc += comps[c - off]
            out.append(stage(acc))
    return out


def layout_kinds(model):
    """K4's layout (recover.rs), as (kind, body, input_derived) per row;
    kind 0..3 = tr, ro, fo, mo."""
    body_of, _ = bodies_em(model)
    nn = len(model["nodes"])
    fixed = {6 * c["node"] + c["c"] for c in model["constraints"]}
    out = []
    for g in range(6 * nn):
        out.append((0 if g % 6 < 3 else 1, body_of[g // 6], g in fixed))
    for node in range(nn):
        out.append((0, body_of[node], False))
    for m in model["members"]:
        for _end in range(2):
            for c in range(6):
                out.append((2 if c < 3 else 3, body_of[m["i"]], False))
    for st in model["stations"]:
        mm = next(x for x in model["members"] if x["id"] == st["member"])
        for c in range(6):
            out.append((2 if c < 3 else 3, body_of[mm["i"]], False))
    for s in model["springs"]:
        out.append((2 if s["c"] < 3 else 3, body_of[s["node"]], False))
    for s in model["dsprings"]:
        for k in range(3):
            c = kind_offset(s["kind"]) + k
            out.append((2 if c < 3 else 3, body_of[s["node"]], False))
    for c in model["constraints"]:
        out.append((2 if c["c"] < 3 else 3, body_of[c["node"]], False))
    for grp in model["supports"]:
        out.append((2, body_of[grp["node"]], False))
        out.append((3, body_of[grp["node"]], False))
    return out


def e_hat_em(e, extent):
    if extent == 0.0:
        return list(e)
    fo, mo = e
    return [max(fo, mo / extent), max(mo, extent * fo)]


def phi_em(eh):
    nearest = eh * f64_of(0x2490000000000000)
    return math.nextafter(nearest, math.inf) if nearest * f64_of(0x5B50000000000000) < eh else nearest


def digest(lines):
    return hashlib.sha256(("\n".join(lines) + "\n").encode()).hexdigest()


def scale_record(name, model, p):
    """E-UNIT's record of a model at p: g, Ā, E_q, E(body, kind), ê and Φ."""
    em = emulate(model, p)
    if "stop" in em:
        return ["scale %s %d stop %s" % (name, p, em["stop"])]
    L = width_of(p)
    mdl = em["model"]
    abar = abar_em(em)
    w = [abs(v) for v in em["u"]]
    E = formation_scale_em(em, abar, w, em["ledger"], lambda x: rp(x, p))
    kinds = layout_kinds(mdl)
    assert len(E) == len(kinds)
    lines = ["scale %s %d state %d" % (name, p, em["corrections"])]
    lines.append("g %s %d %s" % (name, p, ",".join(str(op["g"]) for op in em["members"]) or "-"))
    lines.append("abar %s %d %s" % (name, p, digest(["%d %d %s" % (r, c, W_of(abar[(r, c)], L).token())
                                                     for (r, c) in sorted(abar)])))
    erows = ["%d %s" % (i, W_of(v, L).token()) for i, (v, (k, _b, _i)) in enumerate(zip(E, kinds))
             if k >= 2 and v is not None]
    lines.append("erows %s %d %s" % (name, p, digest(erows)))
    body_of, nb = bodies_em(mdl)
    top = [[Fr(0), Fr(0)] for _ in range(nb)]
    for v, (k, b, _i) in zip(E, kinds):
        if k >= 2 and v is not None:
            top[b][k - 2] = max(top[b][k - 2], v)
    for b in range(nb):
        e = [fl_up(top[b][0]), fl_up(top[b][1])]
        coords = [mdl["nodes"][nd] for nd in range(len(mdl["nodes"])) if body_of[nd] == b]
        eh = e_hat_em(e, body_extent_em(coords))
        ph = [phi_em(x) for x in eh]
        lines.append("ebody %s %d %d %s" % (name, p, b, " ".join("%016x" % b64(x) for x in e + eh + ph)))
    if name in ("N05", "TWO-SPAN", "CHARGE-SLENDER", "DIRECTIONAL-SPAN", "PRESCRIBED"):
        lines += ["erow %s %d %s" % (name, p, r) for r in erows]
    return lines


SCALE_PRECISIONS = (128, 256, 512, 1024)


def scale_lines():
    lines = []
    for m in models() + routed_models() + models5a3() + large_models(LARGE_A3A):
        for p in SCALE_PRECISIONS:
            lines += scale_record(m["name"], m, p)
    return lines


# ---- the bounds (R7 7b-7d), at the verification precisions

def u_pass_em(get, first, nf, p):
    a_ = [Fr(1)] * nf
    for i in range(nf):
        acc = Fr(1)
        for j in range(first[i], i):
            l = abs(get(i, j))
            if l:
                acc = ru(acc + ru(l * a_[j], p), p)
        a_[i] = acc
    c_ = [ru(a_[i] / get(i, i), p) for i in range(nf)]
    for i in reversed(range(nf)):
        for j in range(first[i], i):
            l = abs(get(i, j))
            if l:
                c_[j] = ru(c_[j] + ru(l * c_[i], p), p)
    return c_


def nl_pass_em(get, first, nf, p):
    at = [Fr(1)] * nf
    for i in range(nf):
        for j in range(first[i], i):
            l = abs(get(i, j))
            if l:
                at[j] = ru(at[j] + l, p)
    bt = [ru(get(i, i) * at[i], p) for i in range(nf)]
    ct = list(bt)
    for i in range(nf):
        for j in range(first[i], i):
            l = abs(get(i, j))
            if l:
                ct[i] = ru(ct[i] + ru(l * bt[j], p), p)
    return ct


def gamma_em(nf, p):
    m = 2 * nf + 2
    return ru(Fr(m) / (Fr(2) ** p - m), p)


def uc_from_em(U, NL, gam, p):
    t = ru(ru(U * gam, p) * NL, p)
    uc = ru(U / rd(1 - t, p), p) if t < 1 else None
    return t, uc


def shifted_factor_em(scaled, first, block_of_row, sigma, p):
    """bound.rs `shifted_factor`: (get, failed blocks, shifted diagonal)."""
    nf = len(first)
    rnd = lambda x: rp(x, p)
    sr = [dict(r) for r in scaled]
    dsh = [None] * nf
    for i in range(nf):
        s = sigma.get(block_of_row[i])
        if s is not None:
            dsh[i] = rnd(sr[i][i] - s)
            sr[i][i] = dsh[i]
    get = lambda i, j: sr[i].get(j, Fr(0)) if j >= first[i] else Fr(0)
    failed = set()
    wk = [Fr(0)] * nf
    for i in range(nf):
        for j in range(first[i], i):
            s_ = get(i, j)
            for kk in range(max(first[i], first[j]), j):
                s_ = rnd(s_ - rnd(wk[kk] * get(j, kk)))
            wk[j] = s_
            sr[i][j] = rnd(s_ / get(j, j))
        piv = get(i, i)
        for kk in range(first[i], i):
            piv = rnd(piv - rnd(wk[kk] * get(i, kk)))
        if piv <= 0:
            failed.add(block_of_row[i])
            piv = Fr(1)
        sr[i][i] = piv
    return get, failed, dsh


def ceil_sqrt_em(k):
    r = isqrt(k)
    return r if r * r == k else r + 1


def shift_schedule_em(scaled, first, block_of_row, gam, start, p):
    """bound.rs `shift_schedule`: start = [(block, sigma, n_c)]."""
    nf = len(first)
    res = {b: dict(sigma=s, tries=0, nl=None, delta=None, sp=None, S=None) for b, s, _ in start}
    cur = list(start)
    count = 0
    while cur and count < 3:
        sig = {b: s for b, s, _ in cur}
        get, failed, dsh = shifted_factor_em(scaled, first, block_of_row, sig, p)
        count += 1
        ct = nl_pass_em(get, first, nf, p)
        nxt = []
        for b, s, n_c in cur:
            r = res[b]
            r["tries"] += 1
            r["sigma"] = s
            if b in failed:
                nxt.append((b, s / 2, n_c))
                continue
            rows_b = [i for i in range(nf) if block_of_row[i] == b]
            NLp = max(ct[i] for i in rows_b)
            delta = Fr(2) ** (1 - p) * max(abs(dsh[i]) for i in rows_b)
            e = ru(ru(gam * NLp, p) + delta, p)
            sp = rd(s - e, p)
            r.update(nl=NLp, delta=delta, sp=sp)
            if sp > 0:
                r["S"] = ru(Fr(ceil_sqrt_em(n_c)) / sp, p)
        cur = nxt
    return res, count


def inv_norm1_exact(A):
    """‖A⁻¹‖₁ of a symmetric positive definite Fraction matrix (dense list),
    by exact LDLᵀ on its profile (no pivoting) and one solve per column."""
    n = len(A)
    first = [min(j for j in range(i + 1) if A[i][j] != 0 or j == i) for i in range(n)]
    Lm = [dict() for _ in range(n)]
    D = [Fr(0)] * n
    for i in range(n):
        for j in range(first[i], i + 1):
            s = A[i][j]
            for k in range(max(first[i], first[j]), j):
                lik, ljk = Lm[i].get(k), Lm[j].get(k)
                if lik and ljk:
                    s -= lik * ljk * D[k]
            if j == i:
                D[i] = s
            elif s != 0:
                Lm[i][j] = s / D[j]
        assert D[i] != 0, "singular"
    best = Fr(0)
    for col in range(n):
        x = [Fr(int(i == col)) for i in range(n)]
        for i in range(n):
            for j, l in Lm[i].items():
                x[i] -= l * x[j]
        for i in range(n):
            x[i] /= D[i]
        for i in reversed(range(n)):
            for j, l in Lm[i].items():
                x[j] -= l * x[i]
        best = max(best, sum((abs(v) for v in x), Fr(0)))
    return best


def inv_norm1_upper(A, bits=3072):
    """A certified upper bound on ‖A⁻¹‖₁ for a large symmetric positive
    definite dyadic matrix (RF-LARGE; the exact rational inverse is too slow
    for Python at 60 DOFs and 256+ bits): X ≈ A⁻¹ by LDLᵀ and column solves
    rounded at `bits`, the residual R = I − A·X formed exactly, and
    ‖A⁻¹‖₁ ≤ ‖X‖₁/(1 − ‖R‖₁) (A⁻¹ = X(I − R)⁻¹), valid when ‖R‖₁ < 1."""
    n = len(A)
    rnd = lambda x: rp(x, bits)
    first = [min(j for j in range(i + 1) if A[i][j] != 0 or j == i) for i in range(n)]
    Lm = [dict() for _ in range(n)]
    D = [Fr(0)] * n
    for i in range(n):
        for j in range(first[i], i + 1):
            s_ = A[i][j]
            for k in range(max(first[i], first[j]), j):
                lik, ljk = Lm[i].get(k), Lm[j].get(k)
                if lik and ljk:
                    s_ = rnd(s_ - rnd(rnd(lik * ljk) * D[k]))
            if j == i:
                D[i] = s_
            elif s_ != 0:
                Lm[i][j] = rnd(s_ / D[j])
        assert D[i] > 0
    cols = []
    for col in range(n):
        x = [Fr(int(i == col)) for i in range(n)]
        for i in range(n):
            for j, l in Lm[i].items():
                x[i] = rnd(x[i] - rnd(l * x[j]))
        for i in range(n):
            x[i] = rnd(x[i] / D[i])
        for i in reversed(range(n)):
            for j, l in Lm[i].items():
                x[j] = rnd(x[j] - rnd(l * x[i]))
        cols.append(x)
    nx = max(sum((abs(v) for v in x), Fr(0)) for x in cols)
    nz = [[(j, A[i][j]) for j in range(n) if A[i][j] != 0] for i in range(n)]
    nr = Fr(0)
    for col, x in enumerate(cols):
        r = Fr(0)
        for i in range(n):
            ax = sum((a * x[j] for j, a in nz[i]), Fr(0))
            r += abs(Fr(int(i == col)) - ax)
        nr = max(nr, r)
    assert nr < Fr(1, 2 ** 100), "the approximate inverse is not accurate"
    return nx / (1 - nr)


def frac_hex(x):
    return "%x %x" % (x.numerator, x.denominator)


def factor_digest(em, L):
    nf = len(em["free"])
    get = em["get"]
    return digest(["%d %d %s" % (i, j, W_of(get(i, j), L).token()) for i in range(nf)
                   for j in range(em["first"][i], i + 1)])


def bounds_record(name, model, p, exact=True):
    """E-UC's record at a verification precision p: the factor's digest (Q4's
    pin), per block est_c, U, N_L, t, Uc, the data flag at the state, the
    forced shift (every block with est_c > 0) and the exact ‖K̃_c⁻¹‖₁."""
    em = emulate(model, p)
    if "order" not in em or "est_blk" not in em:
        return ["bnd %s %d stop %s" % (name, p, em.get("stop"))]
    L = width_of(p)
    nf = len(em["free"])
    get, first, order, blk = em["get"], em["first"], em["order"], em["blk"]
    block_of_row = [blk[order[i]] for i in range(nf)]
    gam = gamma_em(nf, p)
    lines = ["bnd %s %d %d %d %s %s" % (name, p, nf, len(em["blocks"]), W_of(gam, L).token(), factor_digest(em, L))]
    c_ = u_pass_em(get, first, nf, p)
    ct = nl_pass_em(get, first, nf, p)
    # Data flags at the state (7a), when the state exists.
    data = None
    if "u" in em:
        data = []
        prescribed_nonzero = {g for g, v in em["prescribed"].items() if v != 0}
        for b, pl in enumerate(em["blocks"]):
            flag = False
            for a in pl:
                g = em["free"][a]
                if g in em["nonzero_terms"] or em["u"][g] != 0:
                    flag = True
                if any(c in prescribed_nonzero for c in em["pattern"][g]):
                    flag = True
            data.append(flag)
    start = []
    for b, pl in enumerate(em["blocks"]):
        rows_b = [i for i in range(nf) if block_of_row[i] == b]
        U = max(c_[i] for i in rows_b)
        NL = max(ct[i] for i in rows_b)
        t, uc = uc_from_em(U, NL, gam, p)
        est = em["est_blk"][b]
        tok = lambda v: W_of(v, L).token() if v is not None else "-"
        norm = "- - -"
        if exact:
            sub = [[em["scaled"][max(i, j)].get(min(i, j), Fr(0)) for j in rows_b] for i in rows_b]
            if len(rows_b) <= 40:
                norm = "exact " + frac_hex(inv_norm1_exact(sub))
            else:
                norm = "upper " + frac_hex(inv_norm1_upper(sub))
        need = None
        if est > 0:
            need = uc is None or uc > 2 * ceil_sqrt_em(len(pl)) * est
            start.append((b, rd(Fr(1) / (2 * est), p), len(pl)))
        lines.append("blk %s %d %d %d %s %s %s %s %s %s %s %s" % (
            name, p, b, len(pl), "-" if data is None else int(data[b]), tok(est), tok(U), tok(NL), tok(t), tok(uc),
            "-" if need is None else int(need), norm))
    res, count = shift_schedule_em(em["scaled"], first, block_of_row, gam, start, p)
    for b in sorted(res):
        r = res[b]
        tok = lambda v: W_of(v, L).token() if v is not None else "-"
        lines.append("shf %s %d %d %s %d %s %s %s %s" % (name, p, b, tok(r["sigma"]), r["tries"], tok(r["nl"]),
                                                        tok(r["delta"]), tok(r["sp"]), tok(r["S"])))
    lines.append("shiftcount %s %d %d" % (name, p, count))
    return lines


BOUND_PRECISIONS = (256, 512, 1024)


def bounds_lines():
    lines = []
    for m in models() + routed_models() + models5a3() + large_models(LARGE_A3A):
        for p in BOUND_PRECISIONS:
            lines += bounds_record(m["name"], m, p)
    return lines


# ---- raw profiles: V4's F2 family, the low-precision stress, SD-G5's searches

def f2_matrix(m, nv=12):
    """V4's F2 family (DS1's stress6.py `f2`, from V4's r4_hh)."""
    n = nv + 4
    hpos = [0, 2, 4, 6]
    vpos = [i for i in range(n) if i not in hpos]
    K = [[Fr(0)] * n for _ in range(n)]
    ks = [Fr(1) if t % 2 == 0 else Fr(3) for t in range(nv - 1)]
    for t, k in enumerate(ks):
        a, b = vpos[t], vpos[t + 1]
        K[a][a] += k
        K[b][b] += k
        K[a][b] -= k
        K[b][a] -= k
    K[vpos[0]][vpos[0]] += Fr(1, 4)
    K[vpos[-1]][vpos[-1]] += Fr(1, 4)
    w = [Fr(1), Fr(-1), Fr(-1), Fr(1)]
    d = Fr(1, 2 ** m)
    for a in range(4):
        for b in range(4):
            K[hpos[a]][hpos[b]] = Fr(int(a == b)) - w[a] * w[b] / 4 + d * w[a] * w[b] / 4
    return K


def stress_matrix(rng, kind, n):
    """DS1's stress6.py `gen`."""
    if kind == "gram":
        B = [[Fr(rng.randint(-8, 8), 8) for _ in range(n)] for _ in range(n)]
        A = [[sum(B[k][i] * B[k][j] for k in range(n)) for j in range(n)] for i in range(n)]
        for i in range(n):
            A[i][i] += Fr(1, 2 ** rng.randint(2, 12))
    elif kind == "laplacian":
        A = [[Fr(0)] * n for _ in range(n)]
        for i in range(n):
            for j in range(i + 1, n):
                if rng.random() < 0.6:
                    w = Fr(rng.randint(1, 16), 4)
                    A[i][i] += w
                    A[j][j] += w
                    A[i][j] -= w
                    A[j][i] -= w
        for i in range(n):
            A[i][i] += Fr(1, 2 ** rng.randint(0, 14))
    elif kind == "lever":
        A = [[Fr(0)] * n for _ in range(n)]
        for i in range(n - 1):
            k = Fr(2) ** rng.randint(-6, 6)
            s = rng.choice((1, -1))
            A[i][i] += k
            A[i + 1][i + 1] += k
            A[i][i + 1] += s * k
            A[i + 1][i] += s * k
            A[i][i] += k * Fr(rng.randint(1, 4), 64)
        A[n - 1][n - 1] += Fr(1, 2 ** rng.randint(0, 10))
    else:
        B = [[Fr(rng.randint(-8, 8), 8) for _ in range(n)] for _ in range(n - 1)]
        row = [sum(B[k][j] * rng.randint(-2, 2) for k in range(n - 1)) for j in range(n)]
        eps = Fr(1, 2 ** rng.randint(4, 20))
        B.append([x + eps * rng.randint(-4, 4) for x in row])
        A = [[sum(B[k][i] * B[k][j] for k in range(n)) for j in range(n)] for i in range(n)]
        for i in range(n):
            A[i][i] += Fr(1, 2 ** rng.randint(10, 30))
    return A


def equilibrate_round(A, P):
    """Entries rounded to P bits, then K4's radix scaling (symmetric)."""
    n = len(A)
    s = [-(k3.floor_log2(A[i][i]) // 2) for i in range(n)]
    K = [[rp(A[i][j], P) * Fr(2) ** (s[i] + s[j]) for j in range(n)] for i in range(n)]
    for i in range(n):
        for j in range(i):
            K[i][j] = K[j][i]
    return K


def profile_record(tag, K, P, sigma_list):
    """A raw profile (natural order, one block): its rows, the unshifted
    factor's Uc (when every pivot passes), and the shift schedule from each
    given σ. Emits `prof`, `puc` and `pshf` lines."""
    n = len(K)
    L = width_of(P)
    first = [min(j for j in range(i + 1) if K[i][j] != 0 or j == i) for i in range(n)]
    scaled = [{j: K[i][j] for j in range(first[i], i + 1)} for i in range(n)]
    tok = lambda v: W_of(v, L).token() if v is not None else "-"
    ex = inv_norm1_exact(K)
    lines = ["prof %s %d %d %s %s" % (tag, P, n, " ".join(str(f) for f in first), frac_hex(ex))]
    for i in range(n):
        lines.append("prow %s %s" % (tag, " ".join(tok(scaled[i][j]) for j in range(first[i], i + 1))))
    block_of_row = [0] * n
    gam = gamma_em(n, P)
    get, failed, _ = shifted_factor_em(scaled, first, block_of_row, {}, P)
    if failed:
        lines.append("puc %s fail" % tag)
    else:
        U = max(u_pass_em(get, first, n, P))
        NL = max(nl_pass_em(get, first, n, P))
        t, uc = uc_from_em(U, NL, gam, P)
        lines.append("puc %s ok %s %s %s %s %d" % (tag, tok(U), tok(NL), tok(t), tok(uc), int(U < ex)))
    for sig in sigma_list:
        res, count = shift_schedule_em(scaled, first, block_of_row, gam, [(0, sig, n)], P)
        r = res[0]
        m27 = None
        if r["sp"] is not None:
            m27 = ru(Fr(ceil_sqrt_em(n)) / r["sigma"], P) < ex
        lines.append("pshf %s %s %d %d %s %s %s %s %s" % (tag, tok(sig), count, r["tries"], tok(r["nl"]), tok(r["delta"]),
                                                        tok(r["sp"]), tok(r["S"]), "-" if m27 is None else int(m27)))
    return lines, ex


def profile_lines():
    lines = []
    # V4's F2 family at 128, 256 and 512.
    for P in (128, 256, 512):
        for mexp in (20, 60, 100, 110, 120, 200, 230):
            K = equilibrate_round(f2_matrix(mexp), P)
            try:
                ex = inv_norm1_exact(K)
            except AssertionError:
                continue  # the hidden mode is lost at P: K̃ is singular
            sigmas = [rp(f / ex, P) for f in (Fr(1, 2), Fr(9, 10), Fr(11, 10), Fr(2))]
            ls, _ = profile_record("F2-m%d-p%d" % (mexp, P), K, P, sigmas)
            lines += ls
    # The low-precision stress (Lemma E; R7-M27's kill at low precision).
    rng = __import__("random").Random(seed_of("K4STR5A3"))
    kept_killers, kept_other = 0, 0
    for k in range(STRESS_COUNT):
        kind = ("gram", "laplacian", "lever", "near")[k % 4]
        n = rng.randint(3, 10)
        P = rng.choice((10, 12, 16, 20, 24, 32))
        A = stress_matrix(rng, kind, n)
        if any(A[i][i] <= 0 for i in range(n)):
            continue
        K = equilibrate_round(A, P)
        try:
            ex = inv_norm1_exact(K)
        except AssertionError:
            continue
        f = Fr(rng.randint(30, 1000), 100)
        sig = rp(f / ex, P)
        if sig <= 0:
            continue
        ls, _ = profile_record("ST%05d" % k, K, P, [sig])
        killer = ls[-1].split()[-1] == "1"
        if killer or kept_other < STRESS_KEEP:
            lines += ls
            kept_killers += killer
            kept_other += not killer
    lines.append("stresscount %d %d %d" % (STRESS_COUNT, kept_killers, kept_other))
    return lines + sdg5_profile_lines()


STRESS_COUNT = 20000
STRESS_KEEP = 400


def uc_of(K, P):
    """(t, Uc) of a raw profile's unshifted factor, or None when a pivot fails."""
    n = len(K)
    first = [min(j for j in range(i + 1) if K[i][j] != 0 or j == i) for i in range(n)]
    scaled = [{j: K[i][j] for j in range(first[i], i + 1)} for i in range(n)]
    get, failed, _ = shifted_factor_em(scaled, first, [0] * n, {}, P)
    if failed:
        return None
    U = max(u_pass_em(get, first, n, P))
    NL = max(nl_pass_em(get, first, n, P))
    return uc_from_em(U, NL, gamma_em(n, P), P)


def schedule_of(K, P, sig):
    n = len(K)
    first = [min(j for j in range(i + 1) if K[i][j] != 0 or j == i) for i in range(n)]
    scaled = [{j: K[i][j] for j in range(first[i], i + 1)} for i in range(n)]
    res, count = shift_schedule_em(scaled, first, [0] * n, gamma_em(n, P), [(0, sig, n)], P)
    return res[0], count


def sdg5_profile_lines():
    """SD-G5's searched profiles (R7 §6.2): t just below 1 and at 1; a shift
    that fails once and succeeds at σ/2; three failures with Uc (B = Uc) and
    without (`uc`); passed pivots with σ′ ≤ 0. Deterministic searches."""
    lines = []
    found = {}
    # t at 1 and just below: 2x2 profiles [[a, b], [b, c]] at small P.
    for P in range(3, 9):
        vals = sorted({rp(Fr(k, 2 ** (P - 1)), P) for k in range(2 ** (P - 1), 2 ** (P + 1))})
        offs = sorted({rp(Fr(k, 2 ** (P - 1)), P) for k in range(-(2 ** P) + 1, 2 ** P) if k != 0})
        for a in vals:
            for c in vals:
                for b in offs:
                    if b * b >= a * c:
                        continue
                    K = [[a, b], [b, c]]
                    r = uc_of(K, P)
                    if r is None:
                        continue
                    t, uc = r
                    if t == 1 and "SD-T1" not in found:
                        found["SD-T1"] = (K, P, [])
                    if t == 1 - Fr(2) ** -P and "SD-TBELOW" not in found:
                        found["SD-TBELOW"] = (K, P, [])
            if "SD-T1" in found and "SD-TBELOW" in found:
                break
        if "SD-T1" in found and "SD-TBELOW" in found:
            break
    rng = __import__("random").Random(seed_of("K4SDG5A3"))
    for k in range(20000):
        if all(t in found for t in ("SD-HALF", "SD-FAIL3-UC", "SD-FAIL3-NONE", "SD-SPNEG")):
            break
        kind = ("gram", "laplacian", "lever", "near")[k % 4]
        n = rng.randint(2, 6)
        P = rng.choice((10, 12, 16))
        A = stress_matrix(rng, kind, n)
        if any(A[i][i] <= 0 for i in range(n)):
            continue
        K = equilibrate_round(A, P)
        try:
            ex = inv_norm1_exact(K)
        except AssertionError:
            continue
        r = uc_of(K, P)
        if r is None:
            continue
        _t, uc = r
        for f in (Fr(1), Fr(3, 2), Fr(40), Fr(3, 10)):
            sig = rp(f / ex, P)
            if sig <= 0:
                continue
            res, count = schedule_of(K, P, sig)
            if "SD-HALF" not in found and res["tries"] == 2 and res["S"] is not None:
                found["SD-HALF"] = (K, P, [sig])
            if res["tries"] == 3 and res["sp"] is None:
                tag = "SD-FAIL3-UC" if uc is not None else "SD-FAIL3-NONE"
                if tag not in found:
                    found[tag] = (K, P, [sig])
            if "SD-SPNEG" not in found and res["sp"] is not None and res["sp"] <= 0:
                found["SD-SPNEG"] = (K, P, [sig])
    for tag in ("SD-T1", "SD-TBELOW", "SD-HALF", "SD-FAIL3-UC", "SD-FAIL3-NONE", "SD-SPNEG"):
        if tag in found:
            K, P, sigmas = found[tag]
            ls, _ = profile_record(tag, K, P, sigmas)
            lines += ls
        else:
            lines.append("notfound %s" % tag)
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
    if want("encodings"):
        files["encodings.txt"] = "\n".join(encoding_lines()) + "\n"
    if want("o8"):
        files["o8_states.txt"] = "\n".join(o8_lines()) + "\n"
    if want("classification"):
        files["classification.txt"] = "\n".join(classification_lines()) + "\n"
    if want("r1"):
        files["r1_cases.txt"] = "\n".join(r1_lines()) + "\n"
    if want("directed"):
        files["directed.txt"] = "\n".join(directed_lines()) + "\n"
    if want("models5a3"):
        lines = []
        for m in models5a3():
            lines += model_lines(m)
        for m in large_models(LARGE_A3A):
            lines += model_lines(m, expectations=False)
        files["models5a3.txt"] = "\n".join(lines) + "\n"
    if want("scale"):
        files["scale.txt"] = "\n".join(scale_lines()) + "\n"
    if want("bounds"):
        files["bounds.txt"] = "\n".join(bounds_lines()) + "\n"
    if want("profiles"):
        files["profiles.txt"] = "\n".join(profile_lines()) + "\n"
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
