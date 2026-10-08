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
  an approximate inverse in fixed point at 2^-1024, and ‖X‖₁ and
  ‖R‖₁ = ‖I − K̃X‖₁ formed exactly as rationals, ‖R‖₁ < 1 asserted);
- `profiles.txt`: V4's F2 family, DS1's low-precision stress (20,000 draws,
  every R7-M27 killer kept) and SD-G5's searched boundary profiles, with
  exact norms.

Checkpoint A3b (the method):
- `models5a3.txt` gains DS1's remaining controls (LEVER2, TILT-LEVER, the
  SEEDED pair with `seed` lines, the probe set, F-2 to F-3, DEMOTION2 and the
  rest of plan §6), CEIL5A3 and F-1's ceiling control at 2^900 (CEIL-S), RF-LARGE
  at 100 members, and the 5a.3 combinations with their exact net expectations;
- `outcomes.txt`: the schedule (`schedule_em`: the hybrid gate, the
  verification pass and `decide` in R7's rejection order) of every model and
  combination: the selected precision and each attempt's outcome;
- `charge.txt` (E-CHARGE): at 256, 512 and 1024, R7 §4.1.6.3 items 1-12 in
  K4's order with R7's directed roundings: r̂, δ̂, Ŵ, ‖ā_q S‖₁, C and W⁺ (as
  digests), and per block and body B, θ, the norms, N_u and t₁ to t₃;
- `estimate.txt` (E-ESTIMATE): |R*(u_P) − q*| per force and moment row above
  2^-(P+20)·ê, u* the exact solution and R* a recovery at 4,096 bits;
- `r1_large.txt`: R1's RF-LARGE frames at 10 and 100 members as lane cases.
- after ROOT's A3b rulings: an ê that overflows while E encodes stops the
  verification as E's overflow does (EHAT-OVERFLOW), and
  `directional_span_exact.txt` gives DIRECTIONAL-SPAN's exact published
  quantities (5a.2's publication checked against them).

After ROOT's rulings on RV19's review:
- `outcomes.txt`: the stop rule's S* leaves out the rows whose candidate value
  has no binary64 value, as the classification does (O9 on the stop rule,
  RV19-1); no control moves, and RV19's controls join (OVF-ROT-928, withheld
  at every p; OVF-ROT-900; TINY-S-995 and TINY-S-900; GROUP-DIR and
  GROUP-DIR-X, support groups holding directional springs, RV19-4), with
  their `scale`, `bounds`, `charge` and `estimate` records;
- `classification.txt`: D1 revision 5a.3's amendment A1 (RV19-6): where
  0 < S* < 2^-988 each absolute row's bound is
  fl↑(fl↑(2^-64·S*) + fl↑(2^-53·|q|) + 2^-1074), with four targeted sets;
- `models.txt` and `models5a3.txt`: every `expect` line carries, after the
  binary64 of the exact value, its 128-bit token `x:±<m>p<e>` (the exact
  value rounded once to 128 bits), a range marker (`underflow`, `overflow`)
  for an exact value outside binary64's range, and, for a model with
  irrational lengths or axes (no exact rational solution: N03-RX, HH-FOOL,
  HH-SLENDER-m40, R115-SEED3, RF-LARGE's ROT frames at 10 and 100 members,
  OVF-ROT), the bound `err:<e>` (|x − q*| ≤ 2^e) of a decimal solve at 300
  digits checked against one at 240 (`solve_hp`). Every model and combination K4 selects now
  carries expectations; none do the mechanisms N02, N03-RZ and N04 and the
  three TILT-LEVER controls
  (withheld; a pivot of the decimal solve falls below its singular test). The
  binary64 fields of the existing lines are unchanged.
- after RV19's delta check (RV19-DN2): `models.txt`'s PRECISION-RULE carries
  the expectations of the combination as its own case (`combined_model`).

T3 KF3 (D1 revision 5a.3 amendment A2; ROOT's ruling on I19's plan):
- the Uc_c and S_c passes mirror `ExactWideSum`'s span limit (8,128 bits from
  the lowest set bit to the highest, over a directed sum's two terms and its
  nearest result): a refused bound is unavailable for its block only, B_c is
  the minimum over the bounds formed, and a block with data left with no bound
  after a refusal stops the verification (`failed:Span`). No earlier model
  refuses, so every earlier record is unchanged;
- `kf3.txt`: the constructed controls KF3-UC-SPAN and KF3-UC-SPAN-ZERO (a
  390-member chain whose comparison-matrix bound outgrows the span at 256),
  with their expectations (the two high-precision solves: the dense exact
  solve is out of reach at 2,340 DOFs), E-UNIT at 128 and 256, E-UC and
  E-CHARGE at 256, and their schedule.

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


# ---- high-precision solves and the expectation tokens (ROOT's rulings on
# RV19's review: RV19-2, RV19-6)
#
# A model whose member lengths or axes are irrational has no exact rational
# solution. Its intended model is solved in decimal arithmetic at HP_DIGITS[0]
# and again at HP_DIGITS[1] significant digits: every binary64 input is lifted
# exactly, each member is formed as T^T k T with the Gram-Schmidt axes, the
# free system is eliminated symmetrically in RCM order, and every published
# quantity is recovered as `solve_exact` recovers it. The expectation is the
# first solve; its error is bounded by 4|x1 - x2| (the second solve's error is
# about 10^60 times the first's), written with it as `err:<e>`
# (|x - q*| <= 2^e). A pivot below 10^-(digits/3) of the largest diagonal is
# a singular model: no expectations.
import decimal as _decimal

HP_DIGITS = (300, 240)
TOKEN_BITS = 128


def _dnum(x):
    if isinstance(x, Fr):
        return _decimal.Decimal(x.numerator) / _decimal.Decimal(x.denominator)
    return _decimal.Decimal(x)


def hp_member(nodes, m):
    """(ke global 12x12, e, L, axes) of a member, in the current decimal context."""
    D = _dnum
    xi = [D(c) for c in nodes[m["i"]]]
    xj = [D(c) for c in nodes[m["j"]]]
    d = [b - a for a, b in zip(xi, xj)]
    L = sum(c * c for c in d).sqrt()
    e = [c / L for c in d]
    yr = [D(c) for c in m["y"]]
    proj = sum(a * b for a, b in zip(yr, e))
    yc = [yr[k] - proj * e[k] for k in range(3)]
    ny = sum(c * c for c in yc).sqrt()
    ey = [c / ny for c in yc]
    ez = [e[1] * ey[2] - e[2] * ey[1], e[2] * ey[0] - e[0] * ey[2], e[0] * ey[1] - e[1] * ey[0]]
    R = [e, ey, ez]
    E = D(m["E"])
    EA, GJ = E * D(m["A"]) / L, D(m["G"]) * D(m["J"]) / L
    kl = [[D(0)] * 12 for _ in range(12)]

    def sym(a, b, v):
        kl[a][b] = v
        kl[b][a] = v
    kl[0][0] = kl[6][6] = EA
    sym(0, 6, -EA)
    kl[3][3] = kl[9][9] = GJ
    sym(3, 9, -GJ)
    for idx, I, sgn in (((1, 5, 7, 11), D(m["Iz"]), 1), ((2, 4, 8, 10), D(m["Iy"]), -1)):
        c = E * I / L ** 3
        pat = [[12, 6 * L * sgn, -12, 6 * L * sgn], [6 * L * sgn, 4 * L * L, -6 * L * sgn, 2 * L * L],
               [-12, -6 * L * sgn, 12, -6 * L * sgn], [6 * L * sgn, 2 * L * L, -6 * L * sgn, 4 * L * L]]
        for a in range(4):
            for b in range(4):
                kl[idx[a]][idx[b]] = c * pat[a][b]
    ke = [[D(0)] * 12 for _ in range(12)]
    for bi in range(4):
        for bj in range(4):
            blk = [[kl[3 * bi + r][3 * bj + c] for c in range(3)] for r in range(3)]
            tmp = [[sum(blk[r][k] * R[k][c] for k in range(3)) for c in range(3)] for r in range(3)]
            for r in range(3):
                for c in range(3):
                    ke[3 * bi + r][3 * bj + c] = sum(R[k][r] * tmp[k][c] for k in range(3))
    return ke, e, L, R


def solve_hp(model, digits):
    """The published quantities of a model's intended system, at `digits`
    significant decimal digits (keys as in `solve_exact`; magnitudes as their
    square roots)."""
    with _decimal.localcontext() as ctx:
        ctx.prec = digits
        D = _dnum
        nodes = model["nodes"]
        nn = len(nodes)
        n = 6 * nn
        K = {}
        mem = []
        for m in model["members"]:
            ke, e, L, axes = hp_member(nodes, m)
            dofs = [6 * m["i"] + k for k in range(6)] + [6 * m["j"] + k for k in range(6)]
            for a in range(12):
                for b in range(12):
                    if ke[a][b] != 0:
                        K[(dofs[a], dofs[b])] = K.get((dofs[a], dofs[b]), D(0)) + ke[a][b]
            mem.append((m, ke, e, L, axes, dofs))
        for s in model["springs"]:
            d = 6 * s["node"] + s["c"]
            K[(d, d)] = K.get((d, d), D(0)) + D(s["k"])
        for s in model["dsprings"]:
            nv = [D(c) for c in s["n"]]
            n2 = sum(c * c for c in nv)
            base = 6 * s["node"] + kind_offset(s["kind"])
            for a in range(3):
                for b in range(3):
                    v = D(s["k"]) * nv[a] * nv[b] / n2
                    if v != 0:
                        K[(base + a, base + b)] = K.get((base + a, base + b), D(0)) + v
        f = [D(0)] * n
        for l in model["loads"]:
            f[6 * l["node"] + l["c"]] += D(l["v"])
        fixed = {}
        for c in model["constraints"]:
            fixed[6 * c["node"] + c["c"]] = D(c["v"])
        free = [g for g in range(n) if g not in fixed]
        pos = {g: a for a, g in enumerate(free)}
        nf = len(free)
        A = [dict() for _ in range(nf)]
        rhs = [f[g] for g in free]
        for (r, c), v in K.items():
            if r in pos and c in pos:
                A[pos[r]][pos[c]] = A[pos[r]].get(pos[c], D(0)) + v
            elif r in pos and c in fixed:
                rhs[pos[r]] -= v * fixed[c]
        if nf:
            top = max(abs(A[i].get(i, D(0))) for i in range(nf))
            tol = top * D(10) ** (-(digits // 3))
            order = rcm_em([sorted(row) for row in A])
            done = [False] * nf
            for k in order:
                piv = A[k].get(k, D(0))
                if abs(piv) <= tol:
                    raise ZeroDivisionError("singular")
                row = [(j, v) for j, v in A[k].items() if j != k and not done[j]]
                for i, aki in row:
                    lik = aki / piv
                    Ai = A[i]
                    for j, v in row:
                        Ai[j] = Ai.get(j, D(0)) - lik * v
                    rhs[i] -= lik * rhs[k]
                done[k] = True
            rank = {v: t for t, v in enumerate(order)}
            x = [None] * nf
            for k in reversed(order):
                acc = rhs[k]
                for j, v in A[k].items():
                    if rank[j] > rank[k]:
                        acc -= v * x[j]
                x[k] = acc / A[k][k]
        u = [D(0)] * n
        for g, v in fixed.items():
            u[g] = v
        for a, g in enumerate(free):
            u[g] = x[a]
        out = {}
        for g in range(n):
            out["u.%d.%d" % (g // 6, g % 6)] = u[g]
        for node in range(nn):
            out["mag.%d" % node] = sum(u[6 * node + k] ** 2 for k in range(3)).sqrt()
        dot3 = lambda a, b: sum(p * q for p, q in zip(a, b))
        for (m, ke, e, L, axes, dofs) in mem:
            ue = [u[d] for d in dofs]
            Fe = [sum(ke[a][b] * ue[b] for b in range(12)) for a in range(12)]
            Fi, Mi, Fj, Mj = Fe[0:3], Fe[3:6], Fe[6:9], Fe[9:12]
            perp2 = lambda v: max(dot3(v, v) - dot3(v, e) ** 2, D(0))
            mid = m["id"]
            out["N.%d" % mid] = dot3(Fj, e)
            out["T.%d" % mid] = dot3(Mj, e)
            out["Mb.%d.i" % mid] = perp2(Mi).sqrt()
            out["Mb.%d.j" % mid] = perp2(Mj).sqrt()
            for end, (Fv, Mv) in (("i", (Fi, Mi)), ("j", (Fj, Mj))):
                for c in range(3):
                    out["end.%d.%s.%d" % (mid, end, c)] = dot3(axes[c], Fv)
                    out["end.%d.%s.%d" % (mid, end, 3 + c)] = dot3(axes[c], Mv)
            for st in model["stations"]:
                if st["member"] != mid:
                    continue
                t = D(st["t"])
                xi = [D(c) for c in nodes[m["i"]]]
                xj = [D(c) for c in nodes[m["j"]]]
                arm = [t * (b - a) for a, b in zip(xi, xj)]
                cross = [arm[1] * Fi[2] - arm[2] * Fi[1], arm[2] * Fi[0] - arm[0] * Fi[2],
                         arm[0] * Fi[1] - arm[1] * Fi[0]]
                Mx = [-Mi[k] + cross[k] for k in range(3)]
                Fx = [-c for c in Fi]
                out["Mbs.%d" % st["id"]] = perp2(Mx).sqrt()
                for c in range(3):
                    out["st.%d.%d" % (st["id"], c)] = dot3(axes[c], Fx)
                    out["st.%d.%d" % (st["id"], 3 + c)] = dot3(axes[c], Mx)
        spring_action = {}
        for s in model["springs"]:
            v = -D(s["k"]) * u[6 * s["node"] + s["c"]]
            spring_action[s["id"]] = {s["c"]: v}
            out["spr.%d.%d" % (s["id"], s["c"])] = v
        for s in model["dsprings"]:
            nv = [D(c) for c in s["n"]]
            n2 = sum(c * c for c in nv)
            base = 6 * s["node"] + kind_offset(s["kind"])
            proj = sum(nv[b] * u[base + b] for b in range(3))
            acts = {}
            for a in range(3):
                v = -D(s["k"]) * proj * nv[a] / n2
                acts[kind_offset(s["kind"]) + a] = v
                out["dspr.%d.%d" % (s["id"], kind_offset(s["kind"]) + a)] = v
            spring_action[s["id"]] = acts
        reaction = {}
        rows_of = {}
        for (rr, c), v in K.items():
            rows_of.setdefault(rr, []).append((c, v))
        for g in sorted(fixed):
            r = sum((v * u[c] for c, v in rows_of.get(g, [])), D(0)) - f[g]
            reaction[g] = r
            out["R.%d.%d" % (g // 6, g % 6)] = r
        for grp in model["supports"]:
            comp = [D(0)] * 6
            for c in range(6):
                g = 6 * grp["node"] + c
                if grp["r"][c]:
                    comp[c] += reaction[g]
                for sid in grp["springs"] + grp["dsprings"]:
                    comp[c] += spring_action[sid].get(c, D(0))
            out["sf.%d" % grp["id"]] = sum(x * x for x in comp[:3]).sqrt()
            out["sm.%d" % grp["id"]] = sum(x * x for x in comp[3:]).sqrt()
        return {k: Fr(v) for k, v in out.items()}


def ceil_log2(x):
    """The least e with 2^e >= x, for a Fraction x > 0."""
    e = k3.floor_log2(x)
    return e if Fr(2) ** e == x else e + 1


def x_token(v):
    """A value rounded once to TOKEN_BITS bits (nearest, ties to even), as
    x:<sign><32 hex digits>p<exponent> (the significand m has 2^127 <= m <
    2^128), or x:0."""
    if v == 0:
        return "x:0"
    r = rp(v, TOKEN_BITS)
    neg = r < 0
    a = -r if neg else r
    e = k3.floor_log2(a) - (TOKEN_BITS - 1)
    mant = a / Fr(2) ** e
    assert mant.denominator == 1 and (1 << (TOKEN_BITS - 1)) <= mant.numerator < (1 << TOKEN_BITS)
    return "x:%s%032xp%d" % ("-" if neg else "+", mant.numerator, e)


def range_marker(v):
    """`underflow` for a nonzero value that rounds to zero in binary64,
    `overflow` for one beyond its range (the sign is x's)."""
    if v == 0:
        return None
    try:
        y = float(v)
    except OverflowError:
        return "overflow"
    return "underflow" if y == 0.0 else None


def f64_hex_of(v):
    """The binary64 of a value (correctly rounded; +-inf beyond the range)."""
    try:
        return "%016x" % b64(float(v))
    except OverflowError:
        return "fff0000000000000" if v < 0 else "7ff0000000000000"


def full_axes(model, ex):
    return all("end.%d.j.0" % m["id"] in ex for m in model["members"])


def expectation_lines(model):
    """`expect` lines for a model: the binary64 of the exact value (as before),
    its 128-bit token, `err:<e>` for a high-precision solve, and a range
    marker. Exact rational solves wherever the geometry allows them;
    otherwise the two high-precision solves; none for a singular model."""
    exact = None
    try:
        # T3 KF3: a model too large for the dense exact solve takes the two
        # high-precision solves (RCM-ordered, sparse).
        exact = None if model.get("hp_only") else solve_exact(model)
        if exact is not None and not full_axes(model, exact):
            exact = None
    except (AssertionError, StopIteration, ZeroDivisionError):
        exact = None
    lines = []
    if exact is not None:
        for key in sorted(exact):
            v = exact[key]
            if isinstance(v, tuple):
                hexv = "%016x" % to_f64_bits(v)
                v = sqrt_p(v[1], 4 * TOKEN_BITS) if v[1] != 0 else Fr(0)
            else:
                hexv = "%016x" % to_f64_bits(v) if range_marker(v) != "overflow" else f64_hex_of(v)
            toks = ["expect", key, hexv, x_token(v)]
            mk = range_marker(v)
            if mk:
                toks.append(mk)
            lines.append(" ".join(toks))
        return lines
    try:
        first = solve_hp(model, HP_DIGITS[0])
        second = solve_hp(model, HP_DIGITS[1])
    except (ZeroDivisionError, _decimal.DivisionByZero, _decimal.InvalidOperation):
        return lines
    for key in sorted(first):
        v = first[key]
        toks = ["expect", key, f64_hex_of(v), x_token(v)]
        err = 4 * abs(v - second[key])
        if err != 0:
            toks.append("err:%d" % ceil_log2(err))
        mk = range_marker(v)
        if mk:
            toks.append(mk)
        lines.append(" ".join(toks))
    return lines


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
        lines += expectation_lines(m)
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


TWO_M53 = Fr(1, 2 ** 53)
TINY_F = Fr(1, 2 ** 1074)


def row_bound(q, s):
    """b for an `absolute_verified` row: fl↑(2^-64·S*), and where
    0 < S* < 2^-988 (D1 revision 5a.3 amendment A1, ROOT's ruling on RV19-6)
    b_row = fl↑(fl↑(2^-64·S*) + fl↑(2^-53·|q|) + 2^-1074)."""
    b = bound_up(s)
    if s == 0.0 or s >= SMALL_S:
        return b
    return fl_up(Fr(b) + Fr(fl_up(Fr(abs(q)) * TWO_M53)) + TINY_F)


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
                classes.append("A:%s" % hexf(row_bound(out[1], s)))
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
    # Amendment A1 (RV19-6): 0 < S* < 2^-988 gives each row its own bound
    # (TINY-S-995's scale; a subnormal and a zero row; S* just below 2^-988,
    # where 2^-53·|q| is normal, and at 2^-988, where the plain rule holds).
    s995 = math.ldexp(1.0, -995)
    sets.append((1, [0.0], [(2, 0, False, ("N", s995)), (2, 0, False, ("N", -s995 * 0.75)),
                            (2, 0, False, ("N", math.nextafter(s995, 0.0))),
                            (2, 0, False, ("S", math.ldexp(3.0, -1070))), (2, 0, False, ("N", 0.0))]))
    below = math.nextafter(SMALL_S, 0.0)
    sets.append((1, [0.0], [(3, 0, False, ("N", below)), (3, 0, False, ("N", below * 0.5)),
                            (3, 0, False, ("N", math.ldexp(1.0, -1030)))]))
    sets.append((1, [0.0], [(0, 0, False, ("N", SMALL_S)), (0, 0, False, ("N", SMALL_S * 0.5))]))
    sets.append((1, [5.0], [(1, 0, False, ("N", math.ldexp(1.0, -1000))), (0, 0, False, ("S", math.ldexp(1.0, -1060)))]))
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
        lines += r1_case_lines(cid, c, model, floors)
    return lines


def r1_large_lines():
    """R1's RF-LARGE frames at 10 and 100 members as R1 lane cases (plan §6:
    honest against R1's references), in `r1_cases.txt`'s format."""
    floors = json.loads(FLOOR_JSON.read_text())["cases"]
    ref = json.loads(R1_JSON.read_text())["cases"]
    lines = []
    for cid in LARGE_A3A + LARGE_A3B:
        lines += r1_case_lines(cid, ref[cid], ref[cid]["model"], floors)
    return lines


def r1_case_lines(cid, c, model, floors):
    lines = []
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


def emulate(model, p, state=True, seeds=()):
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
    # The prescribed values at p: each exact value rounded once (a case's
    # binary64 values exactly; a combination's exact Σ c·v rounded once).
    pres_p = {g: rnd(v) for g, v in prescribed.items()}
    rhs = []
    for g in free:
        v = ledger.get(g, Fr(0))
        for c in pattern[g]:
            if c in prescribed:
                v -= get_k(K, g, c) * pres_p[c]
        rhs.append(rnd(v))
    u_free = solve(rhs) if nf else []
    u = [pres_p.get(g, Fr(0)) for g in range(n)]
    corrections = 0
    prior = math.inf
    evaluated = []
    gate = ("coalesced",)
    while True:
        for a, g in enumerate(free):
            u[g] = u_free[a]
        evaluated.append(list(u_free))
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
            # The bounded test on the best evaluated state (R7 §4.1.4 step 3).
            pick = bounded_fallback_em(model, p, q, Kq, pattern, ledger, free, u, evaluated)
            if pick is None:
                out["stop"] = "ResidualGate"
                return out
            u_free = evaluated[pick]
            for a, g in enumerate(free):
                u[g] = u_free[a]
            gate = ("bounded", pick, len(evaluated))
            break
        prior = worst
        delta = solve(residuals)
        u_free = [rnd(xv + dv) for xv, dv in zip(u_free, delta)]
        corrections += 1
    # A test-only seed of the final state (R7 §7), after the gate.
    for g, v in seeds:
        u[g] = rnd(u[g] + Fr(v))
    out.update(u=list(u), corrections=corrections, gate=gate)
    return out


def abar_at(model, q):
    """Ā at precision q (both triangles), from members formed at q with g."""
    nodes = model["nodes"]
    contrib = {}
    for m in model["members"]:
        op = form_member_g(nodes, m, q)
        A = bounded_block_em(op, q)
        dofs = member_dofs_em(m)
        for a in range(12):
            for b in range(12):
                contrib.setdefault((dofs[a], dofs[b]), []).append(A[a][b])
    for s_ in model["springs"]:
        d = 6 * s_["node"] + s_["c"]
        contrib.setdefault((d, d), []).append(abs(Fr(s_["k"])))
    for s_ in model["dsprings"]:
        blk = directional_em(s_, q)
        base = 6 * s_["node"] + kind_offset(s_["kind"])
        for a in range(3):
            for b in range(3):
                contrib.setdefault((base + a, base + b), []).append(abs(blk[a][b]))
    return {rc: rp(sum(vs, Fr(0)), q) for rc, vs in contrib.items()}


def bounded_fallback_em(model, p, q, Kq, pattern, ledger, free, u_base, evaluated):
    """adaptive.rs `bounded_fallback`: the best evaluated state under the
    bounded denominator, if every row of it passes."""
    abq = abar_at(model, q)
    get_k = lambda r, c: Kq.get((min(r, c), max(r, c)))
    best = None
    for k, uf in enumerate(evaluated):
        u = list(u_base)
        for a, g in enumerate(free):
            u[g] = uf[a]
        eligible, all_pass, worst = True, True, Fr(0)
        for g in free:
            r = ledger.get(g, Fr(0))
            d = abs(r)
            count = 0
            for c in sorted(pattern[g]):
                if u[c] == 0:
                    continue
                kv = get_k(g, c)
                if kv:
                    count += 1
                    r -= kv * u[c]
                av = abq.get((g, c), Fr(0))
                if av:
                    d += av * abs(u[c])
            mm = 2 * count + 2
            num = abs(r) * (2 ** p - mm)
            den = 64 * mm * d
            all_pass = all_pass and num <= den
            if num == 0:
                continue
            if den == 0:
                eligible = False
                break
            worst = max(worst, num / den)
        if not eligible:
            continue
        if best is None or worst < best[1]:
            best = (k, worst, all_pass)
    if best is not None and best[2]:
        return best[0]
    return None


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
    for m in models() + routed_models() + all5a3_models():
        for p in SCALE_PRECISIONS:
            lines += scale_record(m["name"], m, p)
    return lines


# ---- the bounds (R7 7b-7d), at the verification precisions

# T3 KF3 (D1 revision 5a.3 amendment A2): `ExactWideSum` refuses a sum whose
# terms span more than 8,128 bits, from the lowest set bit to the highest
# (`wide_sum.rs`: 128 limbs less 64 bits of carry headroom). In a directed
# addition x + y (`directed.rs` `add_toward`, `sub_toward`) the terms are x, y
# and the nearest result; a product's or a quotient's terms span at most
# 2P + 2 bits, never refused at K4's precisions (asserted). A refusal in the
# formation of Uc_c or S_c makes that bound unavailable for its block only.
SPAN_LIMIT_BITS = 8128


class Refused(Exception):
    """An exact sum the accumulator refuses (amendment A2)."""


def _hi_bit(x):
    x = abs(x)
    return x.numerator.bit_length() - x.denominator.bit_length()


def _lo_bit(x):
    x = abs(x)
    n = x.numerator
    return (n & -n).bit_length() - x.denominator.bit_length()


def sum_span(terms):
    """The bits an exact sum of dyadic terms spans (0 when all are zero)."""
    t = [x for x in terms if x != 0]
    return (max(_hi_bit(x) for x in t) - min(_lo_bit(x) for x in t) + 1) if t else 0


def add_dir(x, y, p, up=True):
    """x + y rounded upward (or downward) at p, refused as `add_toward` is."""
    s = x + y
    if sum_span([x, y, rp(s, p)]) > SPAN_LIMIT_BITS:
        raise Refused("span")
    return ru(s, p) if up else rd(s, p)


def mul_up(x, y, p):
    """x·y rounded upward at p (`mul_toward`): never refused at p <= 1024."""
    s = rp(x * y, p)
    assert sum_span([s, x * y - s, s]) <= SPAN_LIMIT_BITS
    return ru(x * y, p)


def u_pass_em(get, first, nf, p, blk=None, refused=None):
    """bound.rs `u_pass`. With `blk` (the block of each row) and `refused` (a
    dict block -> (kind, pass, row)), amendment A2's refusals: a block's first
    refusal is kept, and its later rows are skipped."""
    if blk is None:
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
    a_ = [Fr(1)] * nf
    for i in range(nf):
        b = blk[i]
        if b in refused:
            continue
        acc = Fr(1)
        try:
            for j in range(first[i], i):
                l = abs(get(i, j))
                if l:
                    acc = add_dir(acc, mul_up(l, a_[j], p), p)
            a_[i] = acc
        except Refused:
            refused[b] = ("span", "forward", i)
    c_ = [ru(a_[i] / get(i, i), p) if blk[i] not in refused else Fr(0) for i in range(nf)]
    for i in reversed(range(nf)):
        b = blk[i]
        if b in refused:
            continue
        for j in range(first[i], i):
            l = abs(get(i, j))
            if l:
                try:
                    c_[j] = add_dir(c_[j], mul_up(l, c_[i], p), p)
                except Refused:
                    refused[b] = ("span", "backward", j)
                    break
    return c_


def nl_pass_em(get, first, nf, p, blk=None, refused=None):
    """bound.rs `nl_pass`; refusals as in `u_pass_em`."""
    if blk is None:
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
    at = [Fr(1)] * nf
    for i in range(nf):
        b = blk[i]
        if b in refused:
            continue
        for j in range(first[i], i):
            l = abs(get(i, j))
            if l:
                try:
                    at[j] = add_dir(at[j], l, p)
                except Refused:
                    refused[b] = ("span", "nl_column", j)
                    break
    bt = [mul_up(get(i, i), at[i], p) if blk[i] not in refused else Fr(0) for i in range(nf)]
    ct = list(bt)
    for i in range(nf):
        b = blk[i]
        if b in refused:
            continue
        for j in range(first[i], i):
            l = abs(get(i, j))
            if l:
                try:
                    ct[i] = add_dir(ct[i], mul_up(l, bt[j], p), p)
                except Refused:
                    refused[b] = ("span", "nl_row", i)
                    break
    return ct


def gamma_em(nf, p):
    m = 2 * nf + 2
    return ru(Fr(m) / (Fr(2) ** p - m), p)


def uc_from_em(U, NL, gam, p):
    t = ru(ru(U * gam, p) * NL, p)
    uc = ru(U / rd(1 - t, p), p) if t < 1 else None
    return t, uc


def uc_from_em_a2(U, NL, gam, p):
    """`bounds_from` with A2: 1 - t is a directed sum (refused as `sub_toward`)."""
    t = mul_up(mul_up(U, gam, p), NL, p)
    uc = ru(U / add_dir(Fr(1), -t, p, up=False), p) if t < 1 else None
    return t, uc


def refusal_token(r):
    """A block's refusal as K4's evidence names it: refused:<kind>:<pass>:<row>."""
    kind, pas, row = r
    return "refused:%s:%s:%s" % (kind, pas, "-" if row is None else row)


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


def shift_schedule_em(scaled, first, block_of_row, gam, start, p, a2=False):
    """bound.rs `shift_schedule`: start = [(block, sigma, n_c)]. With `a2`,
    amendment A2's refusals (N'_L's passes, and e and sigma' as directed sums):
    a refused block has `refused` set and is not retried."""
    nf = len(first)
    res = {b: dict(sigma=s, tries=0, nl=None, delta=None, sp=None, S=None) for b, s, _ in start}
    cur = list(start)
    count = 0
    while cur and count < 3:
        sig = {b: s for b, s, _ in cur}
        get, failed, dsh = shifted_factor_em(scaled, first, block_of_row, sig, p)
        count += 1
        refused = {}
        if a2:
            ct = nl_pass_em(get, first, nf, p, block_of_row, refused)
        else:
            ct = nl_pass_em(get, first, nf, p)
        nxt = []
        for b, s, n_c in cur:
            r = res[b]
            r["tries"] += 1
            r["sigma"] = s
            if b in refused:
                r["refused"] = refused[b]
                continue
            if b in failed:
                nxt.append((b, s / 2, n_c))
                continue
            rows_b = [i for i in range(nf) if block_of_row[i] == b]
            NLp = max(ct[i] for i in rows_b)
            delta = Fr(2) ** (1 - p) * max(abs(dsh[i]) for i in rows_b)
            if a2:
                try:
                    e = add_dir(mul_up(gam, NLp, p), delta, p)
                    sp = add_dir(s, -e, p, up=False)
                except Refused:
                    r["refused"] = ("span", "shift_form", None)
                    continue
            else:
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


def inv_norm1_upper(A, F=1024):
    """A certified upper bound on ‖A⁻¹‖₁ for a symmetric positive definite
    dyadic matrix too large for the exact rational inverse (RF-LARGE; ROOT's
    ruling on A3a): X ≈ A⁻¹ by LDLᵀ and one solve per column in fixed point
    (integers scaled by 2^F, truncating divisions), then, exactly in integers,
    ‖X‖₁ and the residual R = I − A·X, and ‖A⁻¹‖₁ ≤ ‖X‖₁/(1 − ‖R‖₁)
    (A⁻¹ = X(I − R)⁻¹), which needs ‖R‖₁ < 1: asserted explicitly (and far
    below it)."""
    n = len(A)
    s = 0
    for row in A:
        for v in row:
            if v != 0:
                d = v.denominator
                assert d & (d - 1) == 0, "dyadic entries"
                s = max(s, d.bit_length() - 1)
    M = [{j: int(v * (1 << s)) for j, v in enumerate(row) if v != 0} for row in A]
    one = 1 << F
    Ai = [{j: (v << F) >> s if s <= F else v // (1 << (s - F)) for j, v in r.items()} for r in M]
    first = [min(M[i]) for i in range(n)]
    Lm = [dict() for _ in range(n)]
    D = [0] * n
    for i in range(n):
        for j in range(first[i], i + 1):
            acc = Ai[i].get(j, 0) * one
            for k in range(max(first[i], first[j]), j):
                lik, ljk = Lm[i].get(k), Lm[j].get(k)
                if lik and ljk:
                    acc -= (lik * ljk >> F) * D[k]
            acc >>= F
            if j == i:
                D[i] = acc
            elif acc != 0:
                Lm[i][j] = (acc << F) // D[j]
        assert D[i] > 0
    xs = []
    for col in range(n):
        x = [one if i == col else 0 for i in range(n)]
        for i in range(n):
            for j, l in Lm[i].items():
                x[i] -= (l * x[j]) >> F
        for i in range(n):
            x[i] = (x[i] << F) // D[i]
        for i in reversed(range(n)):
            for j, l in Lm[i].items():
                x[j] -= (l * x[i]) >> F
        xs.append(x)
    nx = max(sum(abs(v) for v in x) for x in xs)
    # R = I − A·X exactly: (2^(s+F)·δ − M·X)/2^(s+F), column by column.
    scale = 1 << (s + F)
    nr = 0
    for col, x in enumerate(xs):
        total = 0
        for i in range(n):
            ax = sum(v * x[j] for j, v in M[i].items())
            total += abs((scale if i == col else 0) - ax)
        nr = max(nr, total)
    norm_r = Fr(nr, scale)
    assert norm_r < 1, "‖R‖₁ < 1 is required"
    assert norm_r < Fr(1, 2 ** 100), "the approximate inverse is not accurate"
    return Fr(nx, one) / (1 - norm_r)


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
    # Amendment A2 (T3 KF3): the passes refuse per block as K4's do.
    refused = {}
    c_ = u_pass_em(get, first, nf, p, block_of_row, refused)
    ct = nl_pass_em(get, first, nf, p, block_of_row, refused)
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
        tok = lambda v: W_of(v, L).token() if v is not None else "-"
        if b not in refused:
            U = max(c_[i] for i in rows_b)
            NL = max(ct[i] for i in rows_b)
            try:
                t, uc = uc_from_em_a2(U, NL, gam, p)
            except Refused:
                refused[b] = ("span", "form", None)
        if b in refused:
            U, NL, t, uc = refusal_token(refused[b]), None, None, None
            u_tok = U
        else:
            u_tok = tok(U)
        est = em["est_blk"][b]
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
            name, p, b, len(pl), "-" if data is None else int(data[b]), tok(est), u_tok, tok(NL), tok(t), tok(uc),
            "-" if need is None else int(need), norm))
    res, count = shift_schedule_em(em["scaled"], first, block_of_row, gam, start, p, a2=True)
    for b in sorted(res):
        r = res[b]
        tok = lambda v: W_of(v, L).token() if v is not None else "-"
        line = "shf %s %d %d %s %d %s %s %s %s" % (name, p, b, tok(r["sigma"]), r["tries"], tok(r["nl"]),
                                                  tok(r["delta"]), tok(r["sp"]), tok(r["S"]))
        if "refused" in r:
            line += " " + refusal_token(r["refused"])
        lines.append(line)
    lines.append("shiftcount %s %d %d" % (name, p, count))
    return lines


BOUND_PRECISIONS = (256, 512, 1024)


def bounds_lines():
    lines = []
    for m in models() + routed_models() + all5a3_models():
        precisions = (256,) if "n00100" in m["name"] else BOUND_PRECISIONS
        for p in precisions:
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


# ----------------------------------------------------------------------------
# D1 revision 5a.3, checkpoint A3b: R7 §7's remaining controls, and K4's
# verification pass (R7 §4.1.6.3 items 1-12), its decision (§5.1 (a)-(d), in
# emu7's order: ROOT's A3-0 ruling Q7) and its schedule, emulated in K4's
# order with R7's directed roundings.
# ----------------------------------------------------------------------------
def unit_section(E=1.0, **over):
    s = dict(E=E, G=1.0, A=1.0, Iy=1.0, Iz=1.0, J=1.0)
    s.update(over)
    return s


def f2_model(name="F-2", E=1024.0, load_value=2.0 ** -300):
    m = new_model(name, [(0, 0, 0), (2, 0, 0)])
    add_member(m, 1, 0, 1, y=(0.0, 1.0, 0.0), section=unit_section(E))
    fix(m, 0, (0,), 1.0)
    fix(m, 0, (1, 2, 3, 4, 5))
    fix(m, 1, (1, 2, 3, 4, 5))
    load(m, 1, 0, load_value)
    return m


def demotion(name, E_big=2.0 ** 480, I_soft=1.0, y1=(0.0, 1.0, 0.0)):
    m = new_model(name, [(0, 0, 0), (2, 0, 0), (2, 2, 0)])
    add_member(m, 1, 0, 1, y=y1, section=unit_section(E_big))
    add_member(m, 2, 1, 2, y=(1.0, 0.0, 0.0), section=unit_section(1.0, Iy=I_soft, Iz=I_soft))
    fix(m, 0, (0,), 1.0)
    fix(m, 0, (1, 2, 3, 4, 5))
    fix(m, 1, (0,), 1.0)
    fix(m, 1, (1, 2, 3, 4, 5))
    fix(m, 2, (0,), 1.0)
    fix(m, 2, (2, 3, 4, 5))
    load(m, 2, 1, 1.0)
    return m


def run345(name, y, rigid=None, tip=None, section=None, members=3):
    m = new_model(name, [(3.0 * s, 4.0 * s, 0.0) for s in range(members + 1)])
    for s in range(members):
        add_member(m, s + 1, s, s + 1, y=y, section=section)
    if rigid is None:
        fix(m, 0, range(6))
    elif rigid == "t":
        fix(m, 0, (0,), 1.0)
        fix(m, 0, (1, 2, 3, 4, 5))
    else:
        fix(m, 0, (0, 1, 2, 3, 4))
        fix(m, 0, (5,), 1e-3)
    if tip:
        load(m, members, 1, tip)
    return m


def lever2(k, s, P=None, kt=None):
    """V4's LEVER2 (DS1's lever3.py `lever2`)."""
    L = 2.0 ** k
    m = new_model("LEVER2-k%d-s%d" % (k, s), [(L, 0, 0), (-1, 0, 0), (0, 0, 0), (-1, -1, 0), (0, 0, 1)])
    C, B, A, G, D = 0, 1, 2, 3, 4
    add_member(m, 1, B, A, section=UNIT_SECTION)
    add_member(m, 2, A, C, section=dict(UNIT_SECTION, E=2.0 ** (s + 3 * k), A=2.0 ** -(s + 3 * k)))
    add_member(m, 3, G, B, y=(1, 0, 0), section=dict(UNIT_SECTION, E=4.0, Iy=1.0 / 64, Iz=1.0 / 64))
    Pb = P if P else 1.0
    add_member(m, 4, A, D, y=(1, 0, 0), section=dict(UNIT_SECTION, E=Pb, Iy=1.0 / Pb, Iz=1.0 / Pb, J=Pb))
    for nd in range(4):
        fix(m, nd, (2, 3, 4))
    fix(m, D, (3, 4))
    fix(m, A, (0,), 0.0)
    fix(m, A, (1,), 1.0)
    fix(m, G, (0,), 0.0)
    fix(m, G, (1,), 1.0)
    fix(m, G, (5,), 0.0)
    fix(m, B, (0,), 0.0)
    fix(m, C, (0,), 0.0)
    fix(m, D, (0,), 0.0)
    fix(m, D, (1,), 1.0)
    fix(m, D, (5,), 0.0)
    if kt is not None:
        spring(m, 1, C, 1, kt)
    if P is not None:
        load(m, D, 2, P)
    return m


# DS1's LEVER2 parameters (lever3.stdout.json, built in emu3; GEN reproduces
# kt from the exact 4096-bit tip diagonal and P from ê at 512).
LEVER2_PARAMS = {(90, 40): (2.2227587494850775e-162, 4.9569176510071274e-119),
                 (100, 40): (2.2227587494850775e-162, 4.9569176510071274e-119),
                 (110, 20): (2.1197879309511924e-168, 4.727285052306297e-125)}


def tilt_lever(k, s, P=None, w_exp=-10):
    """V4's TILT-LEVER (DS1's models4.py `tilt_lever`)."""
    L = 2.0 ** k
    t = 2.0 ** (k - 290)
    om = 2.0 ** w_exp
    m = new_model("TILT-LEVER-k%d-s%d" % (k, s), [(L, -t, 0), (-1, 0, 0), (0, 0, 0), (-1, -1, 0), (0, 0, 1)])
    C, B, A, G, D = 0, 1, 2, 3, 4
    add_member(m, 1, B, A, section=UNIT_SECTION)
    add_member(m, 2, A, C, section=dict(UNIT_SECTION, E=2.0 ** (s + 3 * k), A=2.0 ** (6 - 2 * k)))
    add_member(m, 3, G, B, y=(1, 0, 0), section=dict(UNIT_SECTION, E=4.0, Iy=1.0 / 64, Iz=1.0 / 64))
    pb = P if P else 1.0
    add_member(m, 4, A, D, y=(1, 0, 0), section=dict(UNIT_SECTION, E=pb, Iy=1.0 / pb, Iz=1.0 / pb, J=pb))
    for nd in range(4):
        fix(m, nd, (2, 3, 4))
    fix(m, D, (3, 4))
    fix(m, A, (0,), 0.0)
    fix(m, A, (1,), om)
    fix(m, G, (0,), om)
    fix(m, G, (1,), 0.0)
    fix(m, G, (5,), om)
    fix(m, B, (0,), 0.0)
    fix(m, D, (0,), 0.0)
    fix(m, D, (1,), om)
    fix(m, D, (5,), om)
    if P is not None:
        load(m, D, 2, P)
    return m


def e_hat_fo_at(model, p):
    """ê_fo of body 0 at p (K4's E and item 6a), for the lever builders."""
    em = emulate(model, p)
    abar = abar_em(em)
    E = formation_scale_em(em, abar, [abs(v) for v in em["u"]], em["ledger"], lambda x: rp(x, p))
    kinds = layout_kinds(em["model"])
    body_of, nb = bodies_em(em["model"])
    top = [Fr(0), Fr(0)]
    for v, (kk, b, _i) in zip(E, kinds):
        if kk >= 2 and v is not None and b == 0:
            top[kk - 2] = max(top[kk - 2], v)
    e = [fl_up(top[0]), fl_up(top[1])]
    coords = [em["model"]["nodes"][nd] for nd in range(len(em["model"]["nodes"])) if body_of[nd] == 0]
    return e_hat_em(e, body_extent_em(coords))[0]


def lever2_built(k, s):
    m0 = lever2(k, s)
    K, _ = assemble_em(m0, 4096)
    kt = 2.0 ** (k3.floor_log2(K[(1, 1)]) - 580)
    efo = e_hat_fo_at(lever2(k, s, kt=kt), 512)
    P = 2.0 ** (math.floor(math.log2(efo)) - 438)
    assert (kt, P) == LEVER2_PARAMS[(k, s)], ("LEVER2 parameters differ from DS1's", k, s, kt, P)
    return lever2(k, s, P=P, kt=kt)


def tilt_built(k, s):
    efo = e_hat_fo_at(tilt_lever(k, s), 512)
    P = 2.0 ** (math.floor(math.log2(efo)) - 438)
    return tilt_lever(k, s, P=P)


def sweep_frame(seed, index):
    """DS1's candidate sweep generator (`sweep.py` `gen`), frame `index` of `seed`."""
    rng = __import__("random").Random(seed)

    def rigid(t, th, x):
        return (t[0] + th[1] * x[2] - th[2] * x[1], t[1] + th[2] * x[0] - th[0] * x[2],
                t[2] + th[0] * x[1] - th[1] * x[0], th[0], th[1], th[2])
    for idx in range(index + 1):
        nn = rng.choice((2, 3, 3, 4))
        pts = []
        while len(pts) < nn:
            if not pts:
                p = (0.0, 0.0, 0.0)
            else:
                base = pts[rng.randrange(len(pts))]
                d = rng.choice([(3, 4, 0), (4, 0, 3), (0, 3, 4), (2, 3, 6), (2, 0, 0), (0, 0, 5), (1, 2, 2), (6, 2, 3)])
                sgn = [rng.choice((1, -1)) for _ in range(3)]
                p = tuple(float(base[k] + sgn[k] * d[k]) for k in range(3))
            if p not in pts:
                pts.append(p)
        m = new_model("R115-SEED3" if idx == index else "R%d" % idx, pts)
        sec = n_section()
        edges = [(k, k + 1) for k in range(nn - 1)]
        if nn >= 3 and rng.random() < 0.3:
            edges.append((0, nn - 1))
        for mid, (i, j) in enumerate(edges, start=1):
            d = [pts[j][k] - pts[i][k] for k in range(3)]
            if d[0] == 0 and d[1] == 0:
                y = (1.0, 0.0, 0.0)
            else:
                y = rng.choice([(0.0, 0.0, 1.0), (0.0, 0.0, 1.0), (1.0, 1.0, 1.0), (0.0, 1.0, 0.0)])
                cr = (d[1] * y[2] - d[2] * y[1], d[2] * y[0] - d[0] * y[2], d[0] * y[1] - d[1] * y[0])
                if cr == (0.0, 0.0, 0.0):
                    y = (0.0, 0.0, 1.0) if d[2] == 0 else (1.0, 0.0, 0.0)
            f = rng.choice([1.0, 1.0, 2.0 ** 40, 2.0 ** -40, 2.0 ** 100, 2.0 ** -100])
            add_member(m, mid, i, j, y=y, E=sec["E"] * f, G=sec["G"] * f)
        mode = rng.choice(("zero", "rigid", "rigid", "settle"))
        t = [rng.choice((0.0, 1.0, -1e-3, 1e3)) for _ in range(3)]
        th = [rng.choice((0.0, 0.0, 1e-3, -2e-3)) for _ in range(3)]
        if rng.random() < 0.7:
            v = rigid(t, th, pts[0]) if mode != "zero" else (0.0,) * 6
            for c in range(6):
                fix(m, 0, (c,), v[c])
        else:
            v = rigid(t, th, pts[0]) if mode != "zero" else (0.0,) * 6
            for c in range(3):
                fix(m, 0, (c,), v[c])
            for c in range(3, 6):
                spring(m, c, 0, c, rng.choice((1e-4, 1e-12, 1e-20, 1e3)))
        sid = 10
        for n in range(1, nn):
            r = rng.random()
            if r < 0.25:
                v = rigid(t, th, pts[n]) if mode == "rigid" else ((0.0,) * 6 if mode == "zero" else
                                                                 rigid([x * 1.5 for x in t], th, pts[n]))
                comps = rng.sample(range(6), rng.choice((1, 3, 6)))
                for c in comps:
                    fix(m, n, (c,), v[c])
            elif r < 0.45:
                c = rng.randrange(6)
                spring(m, sid, n, c, rng.choice((1e-8, 1e2, 1e12)))
                sid += 1
        for _ in range(rng.choice((0, 1, 1, 2))):
            n = rng.randrange(nn)
            c = rng.randrange(6)
            load(m, n, c, rng.choice((1.0, 2.0 ** -60, 2.0 ** -200, 1e-30, 1e10, -3.0)))
    return m


PROBE_LOADS = (("general", (1000.0, -500.0, 2000.0, 100.0, 200.0, 300.0)),
               ("inplane", (-400.0, 300.0, 0.0, 0.0, 0.0, 0.0)),
               ("outofplane", (0.0, 0.0, 1000.0, 0.0, 0.0, 0.0)),
               ("axial", (600.0, 800.0, 0.0, 0.0, 0.0, 0.0)),
               ("torque", (0.0, 0.0, 0.0, 60.0, 80.0, 0.0)))


def probe_models():
    """K4's V4-S3 probe set (adaptive_tests.rs `run_345`): a (3,4,0) run of 1 or
    3 N-section members, root fixed, five tip loads, y_ref (3,4,5) or (0,0,1)."""
    out = []
    for label, y in (("y345", (3.0, 4.0, 5.0)), ("y001", (0.0, 0.0, 1.0))):
        for members in (1, 3):
            for load_label, tip in PROBE_LOADS:
                m = new_model("PROBE-%s-m%d-%s" % (label, members, load_label),
                              [(3.0 * s, 4.0 * s, 0.0) for s in range(members + 1)])
                for s in range(members):
                    add_member(m, s + 1, s, s + 1, y=y)
                fix(m, 0, range(6))
                for c, v in enumerate(tip):
                    if v != 0.0:
                        load(m, members, c, v, src="tip%d" % c)
                out.append(m)
    return out


SEEDS5A3 = {"SEEDED-COMMON": [(6 * 1 + 1, 2.0 ** -110)], "SEEDED-SOFT": [(6 * 2 + 0, 2.0 ** 40)]}


def models5a3_b():
    """A3b's controls (R7 §7), in addition to `models5a3`."""
    out = []
    base = {m["name"]: m for m in models()}
    out.append(f2_model())
    out.append(f2_model("F-2-CEIL", E=2.0 ** 41, load_value=2.0 ** -1000))
    m = new_model("F-3-FREE", [(0, 0, 0), (3, 4, 0), (6, 8, 0), (9, 12, 0)])
    for s_ in range(3):
        add_member(m, s_ + 1, s_, s_ + 1)
    fix(m, 0, (0,), 1.0)
    fix(m, 0, (1, 2, 3, 4, 5))
    out.append(m)
    m = new_model("F-3-ROT", [(0, 0, 0), (3, 4, 0), (6, 8, 0)])
    for s_ in range(2):
        add_member(m, s_ + 1, s_, s_ + 1)
    fix(m, 0, (0, 1, 2, 3, 4))
    fix(m, 0, (5,), 1e-3)
    out.append(m)
    m = new_model("F-2-SPOS", [(0, 0, 0), (2, 0, 0), (2, 2, 0)])
    add_member(m, 1, 0, 1, y=(0.0, 1.0, 0.0), section=unit_section(2.0 ** 200))
    add_member(m, 2, 1, 2, y=(1.0, 0.0, 0.0), section=unit_section(1.0))
    fix(m, 0, (0,), 1.0)
    fix(m, 0, (1, 2, 3, 4, 5))
    fix(m, 1, (1, 2, 3, 4, 5))
    fix(m, 2, (2, 3, 4))
    load(m, 1, 0, 2.0 ** -60)
    load(m, 2, 1, 1.0)
    out.append(m)
    # PRESCRIBED-TAIL in combination form (R7 §7): 1·A + 2^-100·B, A prescribing
    # ux = 1 at both nodes, B ux(1) = 2^-1000; the net prescription at node 1 is
    # 1 + 2^-1100. PRESCRIBED-TAIL-FREE likewise at node 0 of a two-member run.
    for name, vals in (("PT-A", ((0, 1.0), (1, 1.0))), ("PT-B", ((0, 0.0), (1, 2.0 ** -1000)))):
        m = new_model(name, [(0, 0, 0), (2, 0, 0)])
        add_member(m, 1, 0, 1, y=(0.0, 1.0, 0.0), section=unit_section(1024.0))
        for node, v in vals:
            fix(m, node, (0,), v)
            fix(m, node, (1, 2, 3, 4, 5))
        out.append(m)
    for name, vals in (("PTF-A", ((0, 1.0), (2, 1.0))), ("PTF-B", ((0, 2.0 ** -1000), (2, 0.0)))):
        m = new_model(name, [(0, 0, 0), (2, 0, 0), (4, 0, 0)])
        add_member(m, 1, 0, 1, y=(0.0, 1.0, 0.0), section=unit_section(1024.0))
        add_member(m, 2, 1, 2, y=(0.0, 1.0, 0.0), section=unit_section(1024.0))
        for node, v in vals:
            fix(m, node, (0,), v)
            fix(m, node, (1, 2, 3, 4, 5))
        fix(m, 1, (1, 2, 3, 4, 5))
        out.append(m)
    for name, tip in (("MIXED-2^-200", 2.0 ** -200),):
        m = new_model(name, [(0, 0, 0), (3, 4, 0), (3, 4, 2)])
        add_member(m, 1, 0, 1)
        fix(m, 0, (0,), 1.0)
        fix(m, 1, (0,), 1.0)
        fix(m, 0, (1, 2, 3, 4, 5))
        fix(m, 1, (1, 2, 3, 4, 5))
        add_member(m, 2, 1, 2, y=(1.0, 0.0, 0.0), Iy=2.0 ** -26, Iz=2.0 ** -26)
        load(m, 2, 0, tip)
        out.append(m)
    out.append(demotion("DEMOTION2", I_soft=2.0 ** 400))
    out.append(demotion("ASSEMBLY-SAT", I_soft=1.0))
    out.append(run345("LOADONLY-y345", (3.0, 4.0, 5.0), tip=1.0))
    out.append(run345("LOADONLY-y001", (0.0, 0.0, 1.0), tip=1.0))
    out.append(run345("GS-TRANS-y345", (3.0, 4.0, 5.0), rigid="t"))
    out.append(run345("GS-ROT-y345", (3.0, 4.0, 5.0), rigid="r"))
    out.append(run345("GS-ROT-y345-LOADED", (3.0, 4.0, 5.0), rigid="r", tip=1.0))
    m = pin_case("LEDGER-AT-RESTRAINT", (3.0, 4.0, 0.0), 1e-4, (0.0, 0.0, 0.0))
    for v in (1e80, 1e-8, -1e80):
        load(m, 0, 0, v)
    load(m, 1, 4, 2e-8)
    out.append(m)
    m = json.loads(json.dumps(base["N05"]))
    m["name"] = "SEEDED-COMMON"
    m["nodes"] = [tuple(p) for p in m["nodes"]]
    for mm in m["members"]:
        mm["y"] = tuple(mm["y"])
    m["supports"] = []
    out.append(m)
    m = new_model("M7-GS1", [(0, 0, 0), (3, 4, 0)])
    add_member(m, 1, 0, 1, y=(3.0, 4.0, 5.0))
    fix(m, 0, (0,), 1.0)
    fix(m, 0, (1, 2, 3, 4, 5))
    fix(m, 1, (0,), 1.0)
    fix(m, 1, (1, 2, 3, 4, 5))
    out.append(m)
    sec = n_section()
    sec["Iy"] = sec["Iz"] * 1024.0
    m = new_model("M10-ANISO", [(0, 0, 0), (3, 4, 0), (6, 8, 0)])
    for s_ in range(2):
        add_member(m, s_ + 1, s_, s_ + 1, y=(3.0, 4.0, 5.0 * 2.0 ** -12), section=sec)
    fix(m, 0, (0, 1, 2, 3, 4))
    fix(m, 0, (5,), 1e-3)
    out.append(m)
    m = demotion("M10-G", E_big=2.0 ** 173, I_soft=2.0 ** 100, y1=(1.0, 2.0 ** -12, 0.0))
    out.append(m)
    m = new_model("EXACT-RIGID", [(0, 0, 0), (2, 0, 0)])
    add_member(m, 1, 0, 1, y=(0.0, 1.0, 0.0), section=unit_section(1.0))
    fix(m, 0, (0,), 1.0)
    fix(m, 0, (1, 2, 3, 4, 5))
    fix(m, 1, (0,), 1.0)
    fix(m, 1, (1, 2, 3, 4, 5))
    out.append(m)
    m = new_model("SEEDED-SOFT", [(0, 0, 0), (1, 0, 0), (2, 0, 0)])
    add_member(m, 1, 0, 1, section=UNIT_SECTION)
    tiny = 2.0 ** -300
    add_member(m, 2, 1, 2, section=dict(UNIT_SECTION, A=tiny, Iy=tiny, Iz=tiny, J=tiny))
    fix(m, 0, range(6))
    for nd in (1, 2):
        fix(m, nd, (1, 2, 3, 4, 5))
    load(m, 1, 0, 1.0)
    load(m, 2, 0, 2.0 ** -240)
    out.append(m)
    out.append(sweep_frame(3, 115))
    for k, s_ in ((90, 40), (100, 40), (110, 20)):
        out.append(lever2_built(k, s_))
    for k, s_ in ((90, 40), (100, 40), (110, 20)):
        out.append(tilt_built(k, s_))
    # CEIL5A3 (K4-M24's proposed kill, ROOT's ruling Q14): A and B are
    # LEVER2-k90 with a unit load Q added at D's ux, plus and minus; their
    # half-sum is LEVER2-k90's own case.
    lev = lever2_built(90, 40)
    for name, sign in (("CEIL5A3-A", 1.0), ("CEIL5A3-B", -1.0)):
        m = json.loads(json.dumps(lev))
        m["name"] = name
        m["nodes"] = [tuple(p) for p in m["nodes"]]
        for mm in m["members"]:
            mm["y"] = tuple(mm["y"])
        load(m, 4, 0, sign * 1.0, src="q")
        out.append(m)
    # DIRECTIONAL-SPAN's springs span R³ only through 2^-52 (its condition is
    # about 2^104) and 5a.3 withholds it; the recovery of directional springs
    # keeps a selected control here: well-spread rotational springs and a
    # translational one.
    m = new_model("DIRECTIONAL-WELL", [(0, 0, 0), (0, 0, 3)])
    add_member(m, 1, 0, 1, y=(1.0, 0.0, 0.0))
    fix(m, 0, (0, 1, 2))
    for sid, n in ((1, (1.0, 1.0, 1.0)), (2, (1.0, -1.0, 0.0)), (3, (1.0, 1.0, -2.0))):
        m["dsprings"].append(dict(id=sid, node=0, kind="r", n=n, k=1e6))
    m["dsprings"].append(dict(id=4, node=1, kind="t", n=(0.0, 1.0, 1.0), k=1e3))
    load(m, 1, 3, 1.0)
    load(m, 1, 1, 10.0)
    out.append(m)
    # E finite at the verification while ê_mo = fl(L_b·E_fo) overflows
    # (ROOT's A3b ruling: the same terminal case as E's own overflow).
    m = pin_case("EHAT-OVERFLOW", (3.0, 4.0, 0.0), 1e-4, (0.0, 0.0, 0.0))
    load(m, 1, 3, 2.0 ** 980)
    out.append(m)
    # F-1's ceiling control at P = 2^900 (A3b): CEIL-A and CEIL-B's E does not
    # encode under 5a.3 (a 2^1013-rad rigid rotation), so the control keeps its
    # ratio ε/P = 2^-1060 with ε = 2^-160.
    for name, loads in (("CEIL-S-A", [2.0 ** 900, 2.0 ** -160]), ("CEIL-S-B", [2.0 ** 900]),
                        ("CEIL-S-NET", [2.0 ** -160])):
        m = pin_case(name, (3.0, 4.0, 0.0), 1e-4, (0.0, 0.0, 0.0))
        for v in loads:
            load(m, 1, 3, v)
        out.append(m)
    return out


COMBOS5A3 = (
    ("PRESCRIBED-TAIL", ((1.0, "PT-A"), (2.0 ** -100, "PT-B"))),
    ("PRESCRIBED-TAIL-FREE", ((1.0, "PTF-A"), (2.0 ** -100, "PTF-B"))),
    ("CEIL5A3", ((0.5, "CEIL5A3-A"), (0.5, "CEIL5A3-B"))),
    ("CEILING-S", ((1.0, "CEIL-S-A"), (-1.0, "CEIL-S-B"))),
)
LARGE_A3B = tuple(i.replace("n00010", "n00100") for i in LARGE_A3A)


def combined_model(name, operands, by_name):
    """A combination as its own case (ROOT's F-1 ruling): the first operand's
    stiffness, every operand's load term times its factor (exact Fractions),
    and each prescribed value the exact Σ c·v."""
    first = by_name[operands[0][1]]
    m = dict(first)
    m["name"] = name
    m["loads"] = []
    pres = {}
    for c, opn in operands:
        op = by_name[opn]
        for l in op["loads"]:
            m["loads"].append(dict(node=l["node"], c=l["c"], v=Fr(c) * Fr(l["v"]), src=l["src"]))
        for cc in op["constraints"]:
            g = (cc["node"], cc["c"])
            pres[g] = pres.get(g, Fr(0)) + Fr(c) * Fr(cc["v"])
    m["constraints"] = [dict(node=n_, c=c_, v=v) for (n_, c_), v in pres.items()]
    return m


# ---- K4's recovery, emulated in K4's layout (recover.rs; emu7's `recover`
# plus support groups)

def recover_em(em, u, ledger):
    """recover.rs at p on the state u (all DOFs); ledger None omits it."""
    p, model = em["p"], em["model"]
    rnd = lambda x: rp(x, p)
    nn = len(model["nodes"])
    vals = list(u)
    for node in range(nn):
        sq = rnd(sum((u[6 * node + k] ** 2 for k in range(3)), Fr(0)))
        vals.append(sqrt_p(sq, p) if sq else Fr(0))
    Qs, ends = [], []
    for mm, op in zip(model["members"], em["members"]):
        dofs = member_dofs_em(mm)
        d = [rnd(sum((op["axes"][r][c] * u[dofs[3 * blk + c]] for c in range(3)), Fr(0)))
             for blk in range(4) for r in range(3)]
        inv = op["inv"]
        e = [rnd(d[6] - d[0]), rnd(d[9] - d[3])]
        for rot, tr, sign in ((5, 1, True), (11, 1, True), (4, 2, False), (10, 2, False)):
            e.append(rnd(d[rot] + (inv * d[tr] - inv * d[tr + 6]) * (1 if sign else -1)))
        bz, by = op["bz"], op["by"]
        Q = [rnd(op["axial"] * e[0]), rnd(op["torsion"] * e[1]), rnd(4 * bz * e[2] + 2 * bz * e[3]),
             rnd(2 * bz * e[2] + 4 * bz * e[3]), rnd(4 * by * e[4] + 2 * by * e[5]), rnd(2 * by * e[4] + 4 * by * e[5])]
        vy = rnd(inv * Q[2] + inv * Q[3])
        vz = rnd(-inv * Q[4] - inv * Q[5])
        ends.append([-Q[0], vy, vz, -Q[1], Q[4], Q[2], Q[0], -vy, -vz, Q[1], Q[5], Q[3]])
        Qs.append(Q)
    for acts in ends:
        vals.extend(acts)
    ids = [mm["id"] for mm in model["members"]]
    for st in model["stations"]:
        k = ids.index(st["member"])
        Q, acts = Qs[k], ends[k]
        t = Fr(st["t"])
        vals.extend([acts[6], acts[7], acts[8], acts[9], rnd(t * Q[5] + t * Q[4] - Q[4]),
                     rnd(t * Q[3] + t * Q[2] - Q[2])])
    spring_action = {}
    for s in model["springs"]:
        v = rnd(-Fr(s["k"]) * u[6 * s["node"] + s["c"]])
        spring_action[s["id"]] = (s["c"], v)
        vals.append(v)
    dir_action = {}
    for s, (base, blk) in zip(model["dsprings"], em["dblocks"]):
        comps = [rnd(-sum((blk[a][b] * u[base + b] for b in range(3)), Fr(0))) for a in range(3)]
        dir_action[s["id"]] = (kind_offset(s["kind"]), comps)
        vals.extend(comps)
    K = em["K"]
    get_k = lambda r, c: K.get((min(r, c), max(r, c)), Fr(0))
    reaction = {}
    for c in model["constraints"]:
        g = 6 * c["node"] + c["c"]
        s = sum((get_k(g, j) * u[j] for j in em["pattern"][g]), Fr(0))
        if ledger is not None:
            s -= ledger.get(g, Fr(0))
        reaction[g] = rnd(s)
        vals.append(reaction[g])
    for grp in model["supports"]:
        comp = []
        for c in range(6):
            g = 6 * grp["node"] + c
            s = Fr(0)
            if grp["r"][c]:
                s += reaction.get(g, Fr(0))
            for sid in grp["springs"]:
                cc, v = spring_action[sid]
                if cc == c:
                    s += v
            for sid in grp["dsprings"]:
                off, comps = dir_action[sid]
                if off <= c < off + 3:
                    s += comps[c - off]
            comp.append(rnd(s))
        for part in (comp[:3], comp[3:]):
            sq = rnd(sum((x * x for x in part), Fr(0)))
            vals.append(sqrt_p(sq, p) if sq else Fr(0))
    return vals


def layout_meta(model):
    """(kind 0..3, body, input_derived, id kind 'u'|'mag'|'row') per K4 row."""
    kinds = layout_kinds(model)
    nn = len(model["nodes"])
    out = []
    for i, (k, b, inp) in enumerate(kinds):
        idk = "u" if i < 6 * nn else ("mag" if i < 7 * nn else "row")
        out.append((k, b, inp, idk))
    return out


def verify_em(em):
    """R7 §4.1.6.2 item 4 and §4.1.6.3 items 1-12 at a verification state
    (verify.rs `verify_state`). Returns a dict, or dict(stop=...)."""
    p, model = em["p"], em["model"]
    rnd = lambda x: rp(x, p)
    up = lambda x: ru(x, p)
    qw = min(3 * (p // 2) + 64, 1024)
    nodes = model["nodes"]
    if qw == p:
        mem_w, dblk_w = em["members"], [blk for _, blk in em["dblocks"]]
    else:
        mem_w = [form_member_em(nodes, m, qw) for m in model["members"]]
        dblk_w = [directional_em(s, qw) for s in model["dsprings"]]
    Kc = {}
    for m, op in zip(model["members"], mem_w):
        dofs = member_dofs_em(m)
        for a in range(12):
            for b in range(12):
                key = (dofs[a], dofs[b])
                Kc[key] = Kc.get(key, Fr(0)) + op["ke"][(min(a, b), max(a, b))]
    for s in model["springs"]:
        d = 6 * s["node"] + s["c"]
        Kc[(d, d)] = Kc.get((d, d), Fr(0)) + Fr(s["k"])
    for s, blk in zip(model["dsprings"], dblk_w):
        base = 6 * s["node"] + kind_offset(s["kind"])
        for a in range(3):
            for b in range(3):
                key = (base + a, base + b)
                Kc[key] = Kc.get(key, Fr(0)) + blk[a][b]
    abar = abar_em(em)
    u = em["u"]
    free, position, pattern = em["free"], em["position"], em["pattern"]
    ledger = em["ledger"]
    pres = em["prescribed"]
    u0 = list(u)
    for g, v in pres.items():
        u0[g] = v
    E = formation_scale_em(em, abar, [abs(x) for x in u], ledger, rnd)
    meta = layout_meta(model)
    body_of, nb = bodies_em(model)
    top = [[Fr(0), Fr(0)] for _ in range(nb)]
    for v, (k, b, _i, _t) in zip(E, meta):
        if k >= 2 and v is not None:
            top[b][k - 2] = max(top[b][k - 2], v)
    resolution = [[fl_up(t[0]), fl_up(t[1])] for t in top]
    for b, rr in enumerate(resolution):
        for kind, v in (("fo", rr[0]), ("mo", rr[1])):
            if v == math.inf:
                return dict(stop="ResolutionScale", body=b, kind=kind)
    # ê overflowing binary64 while E encodes is the same terminal case (ROOT's
    # A3b ruling; verify.rs `resolution_hats`): E first for every body, then ê.
    for b in range(nb):
        coords = [model["nodes"][nd] for nd in range(len(model["nodes"])) if body_of[nd] == b]
        h = e_hat_em(resolution[b], body_extent_em(coords))
        for kind, v in (("fo", h[0]), ("mo", h[1])):
            if v == math.inf:
                return dict(stop="ResolutionScale", body=b, kind=kind)
    scale = em["scale"]
    s_of = [Fr(2) ** sc for sc in scale]
    nf = len(free)
    r_exact, r_hat, sr = [], [], []
    for a, g in enumerate(free):
        r = ledger.get(g, Fr(0)) - sum((Kc.get((g, j), Fr(0)) * u0[j] for j in pattern[g]), Fr(0))
        r_exact.append(r)
        r_hat.append(rnd(r))
        sr.append(up(abs(r)) * s_of[a])
    delta = em["solve"](r_hat) if nf else []
    dfull = [Fr(0)] * em["n"]
    for a, g in enumerate(free):
        dfull[g] = delta[a]
    rec = recover_em(em, dfull, None)
    W = [abs(v) if k >= 2 else None for v, (k, _b, _i, _t) in zip(rec, meta)]
    sr2 = []
    for a, g in enumerate(free):
        r2 = r_exact[a] - sum((Kc.get((g, j), Fr(0)) * dfull[j] for j in pattern[g] if j in position), Fr(0))
        sr2.append(up(abs(r2)) * s_of[a])
    inf_row, one_col, sau_row = [], [], []
    for a, g in enumerate(free):
        inf_row.append(up(sum((abar.get((g, j), Fr(0)) * s_of[a] * s_of[position[j]]
                               for j in pattern[g] if j in position), Fr(0))))
        one_col.append(up(sum((abar.get((j, g), Fr(0)) * s_of[a] * s_of[position[j]]
                               for j in pattern[g] if j in position), Fr(0))))
        sau_row.append(up(sum((abar.get((g, j), Fr(0)) * abs(u0[j]) for j in pattern[g]), Fr(0))) * s_of[a])
    w_s = [Fr(0)] * em["n"]
    for a, g in enumerate(free):
        w_s[g] = s_of[a]
    a_s = formation_scale_em(em, abar, w_s, None, up)
    # Blocks, data flags, Uc, the shift, B.
    blocks, blk = em["blocks"], em["blk"]
    prescribed_nonzero = {g for g, v in pres.items() if v != 0}
    data = []
    for pl in blocks:
        flag = False
        for a in pl:
            g = free[a]
            if g in em["nonzero_terms"] or u[g] != 0 or any(c in prescribed_nonzero for c in pattern[g]):
                flag = True
        data.append(flag)
    get, first, order = em["get"], em["first"], em["order"]
    block_of_row = [blk[order[i]] for i in range(nf)]
    gam = gamma_em(nf, p)
    # Amendment A2 (T3 KF3): a refused Uc_c (or S_c) is unavailable for its
    # block only (verify.rs `verify_state`, bound.rs `certificates`).
    refused = {}
    c_ = u_pass_em(get, first, nf, p, block_of_row, refused) if nf else []
    ct = nl_pass_em(get, first, nf, p, block_of_row, refused) if nf else []
    ucs = []
    for b in range(len(blocks)):
        if b in refused:
            ucs.append(None)
            continue
        rows_b = [i for i in range(nf) if block_of_row[i] == b]
        U = max(c_[i] for i in rows_b)
        NL = max(ct[i] for i in rows_b)
        try:
            ucs.append(uc_from_em_a2(U, NL, gam, p)[1])
        except Refused:
            refused[b] = ("span", "form", None)
            ucs.append(None)
    start = []
    for b, pl in enumerate(blocks):
        est = em["est_blk"][b]
        if data[b] and est > 0 and (ucs[b] is None or ucs[b] > 2 * ceil_sqrt_em(len(pl)) * est):
            start.append((b, rd(Fr(1) / (2 * est), p), len(pl)))
    shifts, count = shift_schedule_em(em["scaled"], first, block_of_row, gam, start, p, a2=True) if start else ({}, 0)
    B = []
    uc_missing = None
    stop = None
    for b in range(len(blocks)):
        cands = [x for x in (ucs[b], shifts.get(b, {}).get("S")) if x is not None]
        bb = min(cands) if (data[b] and cands) else None
        if data[b] and bb is None:
            why = refused.get(b) or shifts.get(b, {}).get("refused")
            if why is not None and stop is None:
                stop = why
            elif why is None and uc_missing is None:
                uc_missing = b
        B.append(bb)
    if stop is not None:
        # 7d with A2: a block with data and no bound after a refusal stops the
        # attempt with that refusal, before any `uc` rejection.
        return dict(stop="Span" if stop[0] == "span" else "Exponent")
    norms = []
    for pl in blocks:
        n_ = dict(sas_one=max(one_col[a] for a in pl), sas_inf=max(inf_row[a] for a in pl),
                  sau=max(sau_row[a] for a in pl), sr=max(sr[a] for a in pl), sr2=max(sr2[a] for a in pl),
                  sid=max(abs(delta[a]) / s_of[a] for a in pl))
        n_["sas"] = max(n_["sas_one"], n_["sas_inf"])
        norms.append(n_)
    theta = [up(B[b] * norms[b]["sas"]) * Fr(2) ** (7 - p) if B[b] is not None else None for b in range(len(blocks))]
    g_max, g_violation = 0, None
    for m, op in zip(model["members"], em["members"]):
        dofs = member_dofs_em(m)
        in_scope = any((d in position and data[blk[position[d]]]) or (d not in position and d in prescribed_nonzero)
                       for d in dofs)
        if in_scope:
            g_max = max(g_max, op["g"])
            if op["g"] > p - 16 and g_violation is None:
                g_violation = m["id"]
    block_body = [body_of[free[pl[0]] // 6] for pl in blocks]
    bodies = []
    for body in range(nb):
        bb, th = None, Fr(0)
        agg = dict(sas=Fr(0), sau=Fr(0), sr=Fr(0), sr2=Fr(0), sid=Fr(0))
        for b in range(len(blocks)):
            if block_body[b] != body or not data[b]:
                continue
            if B[b] is not None:
                bb = B[b] if bb is None else max(bb, B[b])
            if theta[b] is not None:
                th = max(th, theta[b])
            for key in agg:
                agg[key] = max(agg[key], norms[b][key])
        bv = bb if bb is not None else Fr(0)
        n_u = up(agg["sau"] + up(up(2 * bv * agg["sas"]) * agg["sr"]))
        t1 = up(bv * n_u) * Fr(2) ** (7 - qw)
        t2 = up(70 * agg["sid"]) * Fr(2) ** (-p)
        t3 = up(up(3 * bv) * agg["sr2"])
        bodies.append(dict(b=bb, theta=th, n_u=n_u, t1=t1, t2=t2, t3=t3, **agg))
    C = [None] * len(meta)
    Wp = [None] * len(meta)
    if uc_missing is None:
        nn = len(model["nodes"])
        for idx, (k, body, inp, idk) in enumerate(meta):
            bd = bodies[body]
            if k >= 2:
                if a_s[idx] is not None:
                    C[idx] = up(a_s[idx] * bd["t1"] + a_s[idx] * bd["t2"] + a_s[idx] * bd["t3"])
            elif idk == "u" and not inp:
                a = position[idx]
                Wp[idx] = up(abs(delta[a]) + s_of[a] * bd["t1"] + s_of[a] * bd["t3"])
            elif idk == "mag":
                node = idx - 6 * nn
                acc = Fr(0)
                for c in range(3):
                    g = 6 * node + c
                    if g in position:
                        a = position[g]
                        acc += abs(delta[a]) + s_of[a] * bd["t1"] + s_of[a] * bd["t3"]
                    else:
                        acc += abs(u[g] - pres[g])
                Wp[idx] = up(acc)
    return dict(p=p, qw=qw, resolution=resolution, E=E, W=W, a_s=a_s, C=C, Wp=Wp, r_hat=r_hat, delta=delta,
                data=data, ucs=ucs, shifts=shifts, count=count, B=B, norms=norms, theta=theta, bodies=bodies,
                g_max=g_max, g_violation=g_violation, uc_missing=uc_missing, meta=meta)


def unpublishable_em(v):
    """A nonzero value with no binary64 value (it underflows to zero or
    overflows)."""
    return range_marker(v) is not None


def scales_at_em(model, meta, values, P, candidate=None):
    """adaptive.rs `scales_at`: S* per body and kind at 2p from the rows that
    are not input-derived, leaving out (ROOT's ruling on RV19-1, O9 on the
    stop rule) every row whose candidate value has no binary64 value."""
    body_of, nb = bodies_em(model)
    s = [[Fr(0)] * 4 for _ in range(nb)]
    for idx, ((k, b, inp, _t), v) in enumerate(zip(meta, values)):
        if candidate is not None and unpublishable_em(candidate[idx]):
            continue
        if not inp:
            s[b][k] = max(s[b][k], abs(v))
    out = []
    for b in range(nb):
        coords = [model["nodes"][nd] for nd in range(len(model["nodes"])) if body_of[nd] == b]
        L = body_extent_em(coords)
        tr, ro, fo, mo = s[b]
        if L == 0.0:
            out.append([tr, ro, fo, mo])
            continue
        Lb = Fr(L)
        out.append([max(tr, rp(Lb * ro, P)), max(ro, rp(tr / Lb, P)), max(fo, rp(mo / Lb, P)), max(mo, rp(Lb * fo, P))])
    return out


def decide_em(model, cand, ver, rep):
    """adaptive.rs `decide`: (a) with V, the floor at 512; (b); uc; θ; g; (d).
    Returns None (accepted) or (reason, detail)."""
    P = rep["p"]
    meta = rep["meta"]
    body_of, nb = bodies_em(model)
    scales = scales_at_em(model, meta, ver, P, candidate=cand)
    hats = []
    for b in range(nb):
        coords = [model["nodes"][nd] for nd in range(len(model["nodes"])) if body_of[nd] == b]
        hats.append(e_hat_em(rep["resolution"][b], body_extent_em(coords)))
    if P == 1024:
        for b in range(nb):
            for k, h in ((2, hats[b][0]), (3, hats[b][1])):
                scales[b][k] = max(scales[b][k], Fr(phi_em(h)))
    hat = lambda b, k: hats[b][0] if k == 2 else hats[b][1]
    Ms = []
    for idx, (k, b, inp, idk) in enumerate(meta):
        q2 = ver[idx]
        M = max(abs(q2), scales[b][k])
        Ms.append(M)
        lhs = abs(cand[idx] - q2)
        if k >= 2:
            lhs += Fr(hat(b, k)) * Fr(2) ** (8 - P)
        elif not inp:
            if rep["Wp"][idx] is not None:
                lhs += rep["Wp"][idx]
            if idk == "mag":
                lhs += abs(q2) * Fr(2) ** (1 - P)
        if lhs > M * Fr(2) ** -64:
            return ("stop_rule", idx)
    for idx, (k, b, inp, idk) in enumerate(meta):
        if rep["W"][idx] is None:
            continue
        if rep["W"][idx] > Fr(hat(b, k)) * Fr(2) ** (6 - P):
            return ("verification_estimate", idx)
    if rep["uc_missing"] is not None:
        return ("uc", rep["uc_missing"])
    for b, t in enumerate(rep["theta"]):
        if t is not None and t > Fr(1, 2):
            return ("theta", b)
    if rep["g_violation"] is not None:
        return ("g_validity", rep["g_violation"])
    for idx, (k, b, inp, idk) in enumerate(meta):
        c = rep["C"][idx]
        if c is None:
            continue
        allow = 60 * Fr(hat(b, k)) * Fr(2) ** (-P) if P < 1024 else Fr(2) ** -86 * Ms[idx]
        if c > allow:
            return ("charge", idx)
    return None


def schedule_em(model, seeds=()):
    """adaptive.rs `run_schedule` with the verification pass and `decide`:
    (selected precision or None, attempts [(p, outcome)])."""
    cache = {}

    def state(p):
        if p not in cache:
            em = emulate(model, p, seeds=seeds)
            if "stop" in em:
                cache[p] = (em["stop"], None, None)
            else:
                cache[p] = (None, em, recover_em(em, em["u"], em["ledger"]))
        return cache[p]
    attempts = []
    ladder = [128, 256, 512, 1024]
    c = 0
    pending = None
    while c < 3:
        p = ladder[c]
        if pending is not None:
            cand = pending
            pending = None
        else:
            cand = state(p)
            if cand[0] is not None:
                attempts.append((p, "failed:" + cand[0]))
                c += 1
                continue
        ver = state(ladder[c + 1])
        if ver[0] is not None:
            attempts.append((p, "rejected:verification_failed"))
            attempts.append((ladder[c + 1], "failed:" + ver[0]))
            c += 2
            continue
        rep = verify_em(ver[1])
        if "stop" in rep:
            attempts.append((p, "rejected:verification_failed"))
            attempts.append((ladder[c + 1], "failed:%s" % rep["stop"]))
            return None, attempts
        why = decide_em(ver[1]["model"], cand[2], ver[2], rep)
        if why is None:
            attempts.append((p, "accepted"))
            attempts.append((ladder[c + 1], "verified"))
            return p, attempts
        attempts.append((p, "rejected:%s:%s" % why))
        pending = ver
        c += 1
    return None, attempts


def models_rv19():
    """The controls of ROOT's rulings on RV19's review:
    - OVF-ROT-928 (RV19-1): an axial load along a (1,2,0) member of section
      2^-100, whose displacements overflow binary64 while its rotation is 0;
      OVF-ROT-900 beside it, where nothing overflows;
    - TINY-S-995 (RV19-6, amendment A1): a unit (3,4,0) member with a tip load
      2^-995, so every row of the body is `absolute_verified` below
      S* = 2^-988; TINY-S-900 beside it;
    - GROUP-DIR and GROUP-DIR-X (RV19-4): DIRECTIONAL-WELL with support groups
      holding its directional springs and restraints (X adds loads at the
      restrained node's rotations)."""
    out = []
    for name, e in (("OVF-ROT-928", 928), ("OVF-ROT-900", 900)):
        m = new_model(name, [(0, 0, 0), (1, 2, 0)])
        tiny = 2.0 ** -100
        add_member(m, 1, 0, 1, section=unit_section(1.0, A=tiny, Iy=tiny, Iz=tiny, J=tiny))
        fix(m, 0, range(6))
        load(m, 1, 0, 2.0 ** e, src="l0")
        load(m, 1, 1, 2.0 ** (e + 1), src="l1")
        out.append(m)
    for name, e in (("TINY-S-995", -995), ("TINY-S-900", -900)):
        m = new_model(name, [(0, 0, 0), (3, 4, 0)])
        add_member(m, 1, 0, 1, section=unit_section(1.0))
        fix(m, 0, range(6))
        load(m, 1, 0, 2.0 ** e, src="l0")
        out.append(m)
    for name, extra in (("GROUP-DIR", ()), ("GROUP-DIR-X", ((0, 3, 2.0), (0, 4, -3.0)))):
        m = new_model(name, [(0, 0, 0), (0, 0, 3)])
        add_member(m, 1, 0, 1, y=(1.0, 0.0, 0.0))
        fix(m, 0, (0, 1, 2))
        for sid, n in ((1, (1.0, 1.0, 1.0)), (2, (1.0, -1.0, 0.0)), (3, (1.0, 1.0, -2.0))):
            m["dsprings"].append(dict(id=sid, node=0, kind="r", n=n, k=1e6))
        m["dsprings"].append(dict(id=4, node=1, kind="t", n=(0.0, 1.0, 1.0), k=1e3))
        load(m, 1, 3, 1.0, src="l0")
        load(m, 1, 1, 10.0, src="l1")
        for k, (nd, c, v) in enumerate(extra):
            load(m, nd, c, v, src="l%d" % (k + 2))
        m["supports"].append(dict(id=1, node=0, r=[1, 1, 1, 0, 0, 0], springs=[], dsprings=[1, 2, 3]))
        m["supports"].append(dict(id=2, node=1, r=[0, 0, 0, 0, 0, 0], springs=[], dsprings=[4]))
        out.append(m)
    return out


def all5a3_models():
    """Every model of models5a3.txt, in order."""
    return (models5a3() + models5a3_b() + probe_models() + large_models(LARGE_A3A) + large_models(LARGE_A3B)
            + models_rv19())


def models5a3_lines():
    lines = []
    for m in all5a3_models():
        body = model_lines(m)
        for g, v in SEEDS5A3.get(m["name"], ()):
            body.insert(-1, "seed %d %s" % (g, hexf(v)))
        lines += body
    by_name = {m["name"]: m for m in all5a3_models()}
    for name, operands in COMBOS5A3:
        lines.append("combo %s %s" % (name, " ".join("%s:%s" % (hexf(c), n_) for c, n_ in operands)))
        lines += expectation_lines(combined_model(name, operands, by_name))
        lines.append("end")
    return lines


def outcome_token(model, rep_block_body, item):
    p, what = item
    return "%d:%s" % (p, what)


def outcome_lines():
    """GEN's schedule (`schedule_em`) for every K4 model and 5a.3 control and
    combination: the selected precision (or -) and each attempt's outcome; a
    rejection names its layout row (stop_rule, verification_estimate, charge),
    the block's body (uc, theta) or the member id (g_validity)."""
    lines = []
    everything = models() + routed_models() + all5a3_models()
    by_name = {m["name"]: m for m in everything}
    items = [(m["name"], m) for m in everything]
    items += [(name, combined_model(name, ops, by_name)) for name, ops in COMBOS5A3]
    for name, m in items:
        sel, attempts = schedule_em(m, seeds=SEEDS5A3.get(name, ()))
        mdl = canonical(m)
        body_of, _ = bodies_em(mdl)
        toks = []
        for p, what in attempts:
            f = what.split(":")
            if f[0] == "rejected" and f[1] in ("uc", "theta"):
                # the block's body
                em = emulate(m, 2 * p, seeds=SEEDS5A3.get(name, ()))
                block = int(f[2])
                what = "rejected:%s:%d" % (f[1], body_of[em["free"][em["blocks"][block][0]] // 6])
            toks.append("%d:%s" % (p, what))
        lines.append("outcome %s %s %s" % (name, "-" if sel is None else sel, " ".join(toks)))
    return lines


def charge_record(name, model, P, seeds=()):
    """E-CHARGE's record of a verification state at P (R7 §4.1.6.3 items
    1-12, with R7's directed roundings), as digests and tokens."""
    em = emulate(model, P, seeds=seeds)
    if "stop" in em:
        return ["chg %s %d stop %s" % (name, P, em["stop"])]
    rep = verify_em(em)
    if "stop" in rep:
        return ["chg %s %d stop %s" % (name, P, rep["stop"])]
    L = width_of(P)
    tok = lambda v: W_of(v, L).token() if v is not None else "-"
    rows = lambda vals: digest(["%d %s" % (i, tok(v)) for i, v in enumerate(vals) if v is not None])
    lines = ["chg %s %d ok %d %s %s" % (name, P, rep["qw"], "-" if rep["uc_missing"] is None else rep["uc_missing"],
                                         "-" if rep["g_violation"] is None else rep["g_violation"])]
    lines.append("chgrows %s %d %s %s %s %s %s %s" % (
        name, P, rows(rep["r_hat"]), rows(rep["delta"]), rows(rep["W"]), rows(rep["a_s"]), rows(rep["C"]),
        rows(rep["Wp"])))
    for b, n_ in enumerate(rep["norms"]):
        lines.append("chgblk %s %d %d %d %s %s %s %s %s %s %s %s %s" % (
            name, P, b, int(rep["data"][b]), tok(rep["B"][b]), tok(rep["theta"][b]), tok(n_["sas_one"]),
            tok(n_["sas_inf"]), tok(n_["sau"]), tok(n_["sr"]), tok(n_["sr2"]), tok(n_["sid"]),
            rep["shifts"].get(b, {}).get("tries", 0)))
    for b, bd in enumerate(rep["bodies"]):
        lines.append("chgbody %s %d %d %s %s %s %s %s %s" % (name, P, b, tok(bd["b"]), tok(bd["theta"]),
                                                           tok(bd["n_u"]), tok(bd["t1"]), tok(bd["t2"]), tok(bd["t3"])))
    lines.append("chgcount %s %d %d %d" % (name, P, rep["count"], rep["g_max"]))
    return lines


def charge_lines():
    lines = []
    large = set(LARGE_A3A) | set(LARGE_A3B)
    for m in models() + routed_models() + all5a3_models():
        if m["name"] in large and "n00100" in m["name"]:
            precisions = (256,)
        else:
            precisions = (256, 512, 1024)
        for P in precisions:
            lines += charge_record(m["name"], m, P, seeds=SEEDS5A3.get(m["name"], ()))
    return lines


HP_BITS = 4096


def hp_em(model):
    """A recovery context at HP_BITS (E-ESTIMATE's reference): the model's
    operators, directional blocks and K rounded at 4,096 bits, far below any
    difference the test resolves."""
    model = canonical(model)
    nodes = model["nodes"]
    members = [form_member_g(nodes, m, HP_BITS) for m in model["members"]]
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
        blk = directional_em(s, HP_BITS)
        base = 6 * s["node"] + kind_offset(s["kind"])
        dblocks.append((base, blk))
        for a in range(3):
            for b in range(a, 3):
                contrib.setdefault((base + a, base + b), []).append(blk[a][b])
    K = {rc: rp(sum(vs, Fr(0)), HP_BITS) for rc, vs in contrib.items()}
    pattern = {g: set() for g in range(6 * len(nodes))}
    for (r, c) in K:
        pattern[r].add(c)
        pattern[c].add(r)
    return dict(model=model, K=K, members=members, dblocks=dblocks, pattern=pattern, p=HP_BITS)


def estimate_lines():
    """E-ESTIMATE's reference (plan §5.1; ROOT's A3-0 ruling Q10): for every
    control with an exact solution u* (RF-LARGE excepted) and P in (256, 512),
    K4's P state u_P as `emulate` reproduces it, and on every force and moment
    row the error |R*(u_P) − q*| = |R*(u_P − u*)|, with R* the recovery at
    4,096 bits (linear: the ledger cancels). A row is listed when its error
    exceeds 2^-(P+20)·ê, with ê from the P state's own E; the token is the
    error rounded to P bits."""
    lines = []
    large = set(LARGE_A3A) | set(LARGE_A3B)
    for m in models() + routed_models() + all5a3_models():
        name = m["name"]
        if name in large:
            continue
        try:
            ex = solve_exact(m)
        except (AssertionError, StopIteration, ZeroDivisionError):
            continue
        mdl = canonical(m)
        n = 6 * len(mdl["nodes"])
        ustar = [ex["u.%d.%d" % (g // 6, g % 6)] for g in range(n)]
        hp = None
        body_of, nb = bodies_em(mdl)
        for P in (256, 512):
            em = emulate(m, P, seeds=SEEDS5A3.get(name, ()))
            if "stop" in em:
                lines.append("estcount %s %d stop" % (name, P))
                continue
            rep = verify_em(em)
            if "stop" in rep:
                lines.append("estcount %s %d stop" % (name, P))
                continue
            if hp is None:
                hp = hp_em(m)
            u0 = list(em["u"])
            for g, v in em["prescribed"].items():
                u0[g] = v
            err = recover_em(hp, [a - b for a, b in zip(u0, ustar)], None)
            hats = []
            for b in range(nb):
                coords = [mdl["nodes"][nd] for nd in range(len(mdl["nodes"])) if body_of[nd] == b]
                hats.append(e_hat_em(rep["resolution"][b], body_extent_em(coords)))
            L = width_of(P)
            count = 0
            for idx, (k, b, _inp, _t) in enumerate(rep["meta"]):
                if k < 2:
                    continue
                e = abs(err[idx])
                h = Fr(hats[b][k - 2])
                if e > h * Fr(2) ** (-(P + 20)):
                    lines.append("est %s %d %d %s" % (name, P, idx, W_of(rp(e, P), L).token()))
                    count += 1
            lines.append("estcount %s %d %d" % (name, P, count))
    return lines


def directional_span_lines():
    """DIRECTIONAL-SPAN's exact published quantities (ROOT's A3b ruling: was
    5a.2's publication within its claim?): q* = R*(u*), u* the exact solution
    and R* the recovery at 4,096 bits with the exact ledger, per layout row,
    rounded to 512 bits."""
    m = next(x for x in models() if x["name"] == "DIRECTIONAL-SPAN")
    mdl = canonical(m)
    ex = solve_exact(m)
    ustar = [ex["u.%d.%d" % (g // 6, g % 6)] for g in range(6 * len(mdl["nodes"]))]
    ledger = emulate(m, 256)["ledger"]
    qs = recover_em(hp_em(m), ustar, ledger)
    return ["qstar DIRECTIONAL-SPAN %d %s" % (i, W_of(rp(q, 512), 8).token()) for i, q in enumerate(qs)]


# ----------------------------------------------------------------------------
# T3 KF3: D1 revision 5a.3 amendment A2 (ROOT's ruling on I19's plan): a
# certified bound whose formation is refused is unavailable for its block, and
# B_c is the minimum over the bounds formed.
# ----------------------------------------------------------------------------
KF3_MEMBERS = 390
# R1's RF-LARGE section (E, G, A, I, I, J as R1 states them), scaled here by
# exact powers of two: invented inputs.
KF3_SECTION = dict(E=2e11, G=8e10, A=0.005969026041820607 * 2.0 ** -26,
                   Iy=2.7009842839238247e-05 * 2.0 ** 16, Iz=2.7009842839238247e-05 * 2.0 ** 28,
                   J=5.4019685678476494e-05 * 2.0 ** 10)


def kf3_models():
    """KF3's constructed controls: a straight chain of 390 members along
    (-1, 12, -12) (each 17 long), y_ref (-12, -9, -8) (orthogonal to the axis,
    norm 17: the local axes are rational), fixed at node 0, with KF3_SECTION.
    In K4's elimination order the comparison-matrix bound M(L)^-1 grows about
    20.8 bits per member, so at the 256 verification the Uc_c passes refuse
    (a backward sum spans more than 8,128 bits), while the true inverse norm
    is about 2^56 and S_c exists (KF3's plan and RETURN).
    - KF3-UC-SPAN: tip loads; before A2 the case ends Unresolved(ExactSumSpan),
      under A2 its block's B_c = S_c.
    - KF3-UC-SPAN-ZERO: no loads; its only block carries no data, so it needs
      no bound, and the refusal is only recorded.
    Too large for the dense exact solve: its expectations are the two
    high-precision solves (`hp_only`)."""
    out = []
    for name, loaded in (("KF3-UC-SPAN", True), ("KF3-UC-SPAN-ZERO", False)):
        nodes = [(-1.0 * k, 12.0 * k, -12.0 * k) for k in range(KF3_MEMBERS + 1)]
        m = new_model(name, nodes)
        for k in range(KF3_MEMBERS):
            add_member(m, k + 1, k, k + 1, y=(-12.0, -9.0, -8.0), section=KF3_SECTION)
        fix(m, 0, range(6))
        if loaded:
            load(m, KF3_MEMBERS, 0, 0.09375, src="l0")
            load(m, KF3_MEMBERS, 1, -0.046875, src="l1")
            load(m, KF3_MEMBERS, 5, 0.25, src="l2")
        m["hp_only"] = True
        out.append(m)
    return out


def kf3_lines():
    """`kf3.txt`: the KF3 models (with their high-precision expectations),
    their E-UNIT records at 128 and 256, their E-UC record at 256 (the norm
    column `-`: no certified norm at this size), their E-CHARGE record at 256
    and their schedule, in the formats of `models5a3.txt`, `scale.txt`,
    `bounds.txt`, `charge.txt` and `outcomes.txt`."""
    lines = []
    ms = kf3_models()
    for m in ms:
        lines += model_lines(m)
    for m in ms:
        for p in (128, 256):
            lines += scale_record(m["name"], m, p)
    for m in ms:
        lines += bounds_record(m["name"], m, 256, exact=False)
    for m in ms:
        lines += charge_record(m["name"], m, 256)
    for m in ms:
        sel, attempts = schedule_em(m)
        toks = ["%d:%s" % (p, what) for p, what in attempts]
        assert not any(t.split(":")[1:2] == ["rejected"] and t.split(":")[2] in ("uc", "theta") for t in toks)
        lines.append("outcome %s %s %s" % (m["name"], "-" if sel is None else sel, " ".join(toks)))
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
    if want("r1large"):
        files["r1_large.txt"] = "\n".join(r1_large_lines()) + "\n"
    if want("directed"):
        files["directed.txt"] = "\n".join(directed_lines()) + "\n"
    if want("models5a3"):
        files["models5a3.txt"] = "\n".join(models5a3_lines()) + "\n"
    if want("outcomes"):
        files["outcomes.txt"] = "\n".join(outcome_lines()) + "\n"
    if want("charge"):
        files["charge.txt"] = "\n".join(charge_lines()) + "\n"
    if want("estimate"):
        files["estimate.txt"] = "\n".join(estimate_lines()) + "\n"
    if want("ds52"):
        files["directional_span_exact.txt"] = "\n".join(directional_span_lines()) + "\n"
    if want("scale"):
        files["scale.txt"] = "\n".join(scale_lines()) + "\n"
    if want("bounds"):
        files["bounds.txt"] = "\n".join(bounds_lines()) + "\n"
    if want("profiles"):
        files["profiles.txt"] = "\n".join(profile_lines()) + "\n"
    if want("kf3"):
        files["kf3.txt"] = "\n".join(kf3_lines()) + "\n"
    if want("models"):
        lines = []
        by_name = {}
        for m in models() + routed_models():
            by_name[m["name"]] = m
            lines += model_lines(m)
        for name, operands, net in COMBOS:
            lines.append("combo %s %s" % (name, " ".join("%s:%s" % (hexf(c), mname) for c, mname in operands)))
            if net is not None:
                lines += expectation_lines(by_name[net])
            else:
                # PRECISION-RULE (RV19-DN2): the combination as its own case.
                lines += expectation_lines(combined_model(name, operands, by_name))
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
