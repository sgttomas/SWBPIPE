#!/usr/bin/env python3
"""RT-A4 independent references: the absolute-sum and SRSS kernel table (V-1).

Standard-library Python 3 only. This generator imports, reads and runs no
product code; it reads only its own directory's committed files (to hash them)
and writes `references.json` and `SHA256SUMS`. Definitions (DEL-05-02 Work
`absolute-sum-srss-combinations.md` section 3), for binary64 factors f_i and
values x_i:

  A = RN(sum |f_i x_i|)            the exact sum, rounded once;
  z_i = RN(f_i x_i),
  R = RN(sqrt(sum z_i^2))          the exact sum of squares, its root rounded once.

RN is rounding to nearest, ties to even, at the binary64 quantum of the result's
binade (2^-1074 in the subnormal binade). An exact zero is +0.0. A value that
rounds beyond the largest finite binary64 (an exact value at or above
MAX + 2^970) is refused as "non_representable", as is an R whose z_i does.

Every value is an exact fractions.Fraction. RN is decided on the exact rational
by integer rounding at the binade's quantum. The root is rounded without any
candidate or midpoint search: with Q the result's quantum and T = S / Q^2,
n = isqrt(floor(T)) is floor(sqrt(T)) and the remainder T - n^2 is compared
exactly with n + 1/4 (sqrt(T) against n + 1/2) to choose n or n + 1, ties to
even. Rows whose S must lie one accumulator quantum (2^-2148) from a midpoint
or from the overflow threshold are built by a greedy exact decomposition of
the target S into squares of binary64 values.

Modes:
  python3 -I rt_a4_reference.py           check: rebuild every output in memory and
                                           compare byte for byte with the committed
                                           files, SHA256SUMS included
  python3 -I rt_a4_reference.py --write   write the outputs (intentional changes only)
"""
import argparse
import hashlib
import json
import math
import sys
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE.parent
FIXED_FILES = ("README.md", "generators/rt_a4_reference.py")

REFUSED = "non_representable"
QUANTUM = F(1, 2 ** 2148)  # the quantum of an exact product of two binary64 values
TINY = F(1, 2 ** 1074)
MAX = F(2 ** 53 - 1) * F(2) ** 971
THRESHOLD = (MAX + F(2) ** 970) ** 2  # S at or above this: R is refused


# ----------------------------------------------------------------------------- binary64
def from_bits(bits):
    """The exact value of a finite binary64 bit pattern."""
    sign = -1 if bits >> 63 else 1
    biased = (bits >> 52) & 0x7FF
    fraction = bits & ((1 << 52) - 1)
    if biased == 0x7FF:
        raise ValueError("not finite")
    if biased == 0:
        return sign * F(fraction) * TINY
    return sign * F(fraction | (1 << 52)) * F(2) ** (biased - 1075)


def bits_of(value):
    """The bit pattern of a binary64 value (exactly representable; -0.0 is not used here)."""
    if value == 0:
        return 0
    sign = 1 << 63 if value < 0 else 0
    value = abs(value)
    e = floor_log2(value)
    if e >= -1022:
        m = value / F(2) ** (e - 52)
        assert m.denominator == 1 and 2 ** 52 <= m < 2 ** 53, value
        assert e <= 1023
        return sign | ((e + 1023) << 52) | (m.numerator - 2 ** 52)
    m = value / TINY
    assert m.denominator == 1 and m < 2 ** 52
    return sign | m.numerator


def double(x):
    """A Python float literal as its exact value (the literal's nearest binary64)."""
    return F(x)


def floor_log2(q):
    """floor(log2 q) for a rational q > 0."""
    e = q.numerator.bit_length() - q.denominator.bit_length()
    if q < F(2) ** e:
        e -= 1
    assert F(2) ** e <= q < F(2) ** (e + 1)
    return e


def quantum_exponent(q):
    """The binary64 quantum exponent of the binade holding q > 0."""
    return max(floor_log2(q) - 52, -1074)


def round_half_even(t):
    """t rounded to the nearest integer, ties to even (t a non-negative rational)."""
    n = t.numerator // t.denominator
    rest = t - n
    if rest > F(1, 2) or (rest == F(1, 2) and n % 2 == 1):
        n += 1
    return n


def finish(n, qe):
    """n * 2^qe as a binary64 value, or None beyond the finite range."""
    value = F(n) * F(2) ** qe
    if value >= F(2) ** 1024:
        return None
    return value


def rn(q):
    """RN(q) for a rational q >= 0; None when it rounds beyond MAX."""
    if q == 0:
        return F(0)
    qe = quantum_exponent(q)
    return finish(round_half_even(q / F(2) ** qe), qe)


def rn_sqrt(s):
    """RN(sqrt(s)) for a rational s >= 0, by isqrt and a remainder test."""
    if s == 0:
        return F(0)
    qe = max(floor_log2(s) // 2 - 52, -1074)
    t = s / F(2) ** (2 * qe)
    n = math.isqrt(t.numerator // t.denominator)
    assert n * n <= t < (n + 1) * (n + 1)
    remainder = t - n * n  # sqrt(t) > n + 1/2  <=>  t > n^2 + n + 1/4
    half = n + F(1, 4)
    if remainder > half or (remainder == half and n % 2 == 1):
        n += 1
    return finish(n, qe)


# ----------------------------------------------------------------------------- kernel
def absolute_sum(terms):
    return rn(sum((abs(f * x) for f, x in terms), F(0)))


def srss(terms):
    s = F(0)
    for f, x in terms:
        z = rn(abs(f * x))
        if z is None:
            return None
        s += z * z
    return rn_sqrt(s)


def square_terms(target):
    """Binary64 values whose exact squares sum to `target` (a multiple of 2^-2148),
    greedily: each is the largest binary64 value whose square does not exceed what
    remains."""
    assert (target / QUANTUM).denominator == 1
    values, rest = [], target
    while rest > 0:
        qe = max(floor_log2(rest) // 2 - 52, -1074)
        t = rest / F(2) ** (2 * qe)
        n = math.isqrt(t.numerator // t.denominator)
        z = F(n) * F(2) ** qe
        assert n >= 1 and z * z <= rest < (z + F(2) ** qe) ** 2
        values.append(z)
        rest -= z * z
    return values


def unit(values):
    return [(F(1), v) for v in values]


# ----------------------------------------------------------------------------- cases
def p2(k):
    return F(2) ** k


def pythagorean(k, n):
    """k(2n+1, 2n(n+1), 2n^2+2n+1): an exact right triangle."""
    return k * (2 * n + 1), k * 2 * n * (n + 1), k * (2 * n * n + 2 * n + 1)


def cases():
    out = []

    def case(case_id, covers, terms):
        out.append((case_id, covers, terms))

    # Zeros and -0.0 (encoded by bits, so the sign survives).
    case("zero_positive", "a +0.0 value", [(F(1), "+0")])
    case("zero_negative", "a -0.0 value", [(F(1), "-0")])
    case("zero_negative_factor", "a -0.0 factor on a negative value", [("-0", double(-5.0))])
    case("zero_mixed_signs", "+0.0 and -0.0 values, opposite factors", [(F(1), "+0"), (F(-1), "-0")])
    # One term, across the range.
    case("one_term", "one positive term", [(F(1), F(3))])
    case("one_term_negative", "one negative value", [(F(1), F(-3))])
    case("one_term_negative_factor", "a negative dyadic factor", [(F(-2), F(5, 2))])
    case("one_term_tiny", "the smallest subnormal 2^-1074", [(F(1), TINY)])
    case("one_term_tiny_negative", "-2^-1074", [(F(1), -TINY)])
    case("one_term_2p1023", "2^1023", [(F(1), p2(1023))])
    case("one_term_max", "MAX", [(F(1), MAX)])
    case("one_term_max_negative", "-MAX with factor -1", [(F(-1), -MAX)])
    case("one_term_non_dyadic", "factor 1.3 (not dyadic)", [(double(1.3), F(7))])
    # Opposite signs.
    case("opposite_signs_3_4", "3 and -4: A = 7, R = 5", [(F(1), F(3)), (F(1), F(-4))])
    case("opposite_signs_cancel", "x and -x: A = 2|x|, R = sqrt(2)|x|",
         [(F(1), double(1e16)), (F(-1), double(1e16))])
    # Dyadic and non-dyadic factors.
    case("dyadic_factors", "factors 1/2 and 1/4", [(F(1, 2), F(3)), (F(1, 4), F(-4))])
    case("non_dyadic_factors", "1.3, 1.35, 0.7 on mixed magnitudes",
         [(double(1.3), double(4.1e7)), (double(1.35), double(-4.1e7)), (double(0.7), double(1.3))])
    case("non_dyadic_thirds", "fl(1/3), fl(2/3), 0.1",
         [(double(1 / 3), double(0.1)), (double(2 / 3), double(-0.2)), (double(0.1), double(0.3))])
    case("five_terms", "the five-term table permuted by the Rust tests (V-3)",
         [(double(1.3), double(7.25)), (double(-0.7), double(1e-3)), (F(2), F(-300)),
          (double(1 / 3), double(1e5)), (double(0.1), -p2(-30))])
    # Subnormal results.
    case("subnormal_pair", "2^-1074 twice: A = 2^-1073, R = RN(sqrt(2) 2^-1074)", unit([TINY, TINY]))
    case("subnormal_exact_root", "3 and 4 quanta: R = 5 quanta", unit([3 * TINY, 4 * TINY]))
    case("subnormal_2_quanta_pair", "2 quanta twice: R = RN(2 sqrt(2)) quanta", unit([2 * TINY, 2 * TINY]))
    case("subnormal_root_binade", "R in the subnormal binade from normal-adjacent terms",
         unit([p2(-1030), p2(-1031)]))
    case("subnormal_mixed_binades", "R in the subnormal binade, terms 2^-1060 and 7 quanta",
         unit([p2(-1060), 7 * TINY]))
    case("z_underflows_to_zero", "1/2 x 2^-1074: z ties to +0.0, A ties to +0.0", [(F(1, 2), TINY)])
    case("z_rounds_up_to_tiny", "3/4 x 2^-1074 rounds to 2^-1074", [(F(3, 4), TINY)])
    case("products_below_half_quantum", "2^-600 x 2^-600 = 2^-1200: A = +0, z = +0",
         [(p2(-600), p2(-600))])
    case("non_dyadic_subnormal", "fl(0.3) x 3 quanta", [(double(0.3), 3 * TINY)])
    case("span_tiny_to_2p1023", "2^-1074 and 2^1023", unit([TINY, p2(1023)]))
    # Overflow.
    case("z_overflows", "2 x MAX: z overflows (A overflows too)", [(F(2), MAX)])
    case("z_overflows_non_dyadic", "1e300 x 1e10", [(double(1e300), double(1e10))])
    case("a_overflows_r_finite", "0.6 MAX twice: A refused, R finite",
         unit([rn(MAX * F(3, 5)), rn(MAX * F(3, 5))]))
    case("max_and_one", "SRSS(MAX, 1) = MAX", unit([MAX, F(1)]))
    case("max_and_max", "SRSS(MAX, MAX) refused", unit([MAX, MAX]))
    case("threshold_minus_quantum", "S = (MAX + 2^970)^2 - 2^-2148: R = MAX",
         unit(square_terms(THRESHOLD - QUANTUM)))
    case("threshold", "S = (MAX + 2^970)^2: the tie goes to 2^1024, refused",
         unit(square_terms(THRESHOLD)))
    case("threshold_plus_quantum", "S = (MAX + 2^970)^2 + 2^-2148: refused",
         unit(square_terms(THRESHOLD + QUANTUM)))
    # Near-midpoint roots: S = m^2 and one quantum (2^-2148) either side.
    for label, r in (("even", F(1)), ("odd", F(1) + p2(-52))):
        m = r + p2(-53)
        r_text = "1" if label == "even" else "1 + 2^-52"
        for suffix, delta, s_text in (("minus_quantum", -QUANTUM, "m^2 - 2^-2148"), ("tie", F(0), "m^2"),
                                      ("plus_quantum", QUANTUM, "m^2 + 2^-2148")):
            case(f"midpoint_{label}_{suffix}", f"S = {s_text}, m the midpoint above the {label} r = {r_text}",
                 unit(square_terms(m * m + delta)))
    for k, n, label in ((1, (1 << 26) + 5, "down"), (3, 40_000_000, "up")):
        a, b, h = pythagorean(k, n)
        case(f"pythagorean_tie_{label}", f"a^2 + b^2 = h^2, h odd in [2^53, 2^54): the tie goes {label} to even",
             unit([F(a), F(b)]))
        case(f"pythagorean_tie_{label}_plus_quantum",
             "the same scaled by 2^-1074 with a third term 2^-1074: S = m^2 + 2^-2148, up",
             unit([a * TINY, b * TINY, TINY]))
    s = (1 << 26) + 3
    h = 2 * s * s + 1
    case("integer_midpoint_minus_one", "(h-1)^2 + (2s)^2 = h^2 - 1, h = 2s^2 + 1: below the midpoint h",
         unit([F(h - 1), F(2 * s)]))
    case("subnormal_near_midpoint_30", "S = 30 quanta^2: sqrt(30) = 5.48, R = 5 quanta",
         unit(square_terms(30 * QUANTUM)))
    case("subnormal_near_midpoint_31", "S = 31 quanta^2: sqrt(31) = 5.57, R = 6 quanta",
         unit(square_terms(31 * QUANTUM)))
    case("fold_loses_tail", "1, 2^-26, 2^-53, 2^-53: S = 1 + 2^-52 + 2^-105 is above the midpoint",
         unit([F(1), p2(-26), p2(-53), p2(-53)]))
    case("absolute_sum_fold_loses_tail", "1e16 + 1 + 1: exact 1e16 + 2",
         unit([double(1e16), F(1), F(1)]))
    case("absolute_sum_tie_even_down", "2^53 + 1: the tie goes to 2^53", unit([p2(53), F(1)]))
    case("absolute_sum_tie_even_up", "2^53 + 2 + 1: the tie goes to 2^53 + 4", unit([p2(53) + 2, F(1)]))
    # Exact squares.
    case("exact_square_2_3_6", "2, 3, 6: R = 7", unit([F(2), F(3), F(6)]))
    case("exact_square_four_terms", "1, 2, 2, 4: R = 5", unit([F(1), F(2), F(2), F(4)]))
    case("exact_square_large", "3 x 2^1000 and -4 x 2^1000: R = 5 x 2^1000",
         unit([3 * p2(1000), -4 * p2(1000)]))
    case("exact_square_scaled_terms", "2 x 1.5 and -4 x 1: z = 3, -4, R = 5",
         [(F(2), F(3, 2)), (F(-4), F(1))])
    # The smallest normal (r1 F-review): both gaps at 2^-1022 are 2^-1074.
    case("smallest_normal_gaps", "2^-1022 - 2^-1074 and 2^-1048: R = 2^-1022, no oscillation",
         unit([p2(-1022) - TINY, p2(-1048)]))
    return out


# ----------------------------------------------------------------------------- output
def hex_bits(bits):
    return f"0x{bits:016x}"


def encode(value):
    if value == "+0":
        return hex_bits(0)
    if value == "-0":
        return hex_bits(1 << 63)
    return hex_bits(bits_of(value))


def decode(value):
    return F(0) if value in ("+0", "-0") else value


def expected(value):
    return REFUSED if value is None else hex_bits(bits_of(value))


def case_record(case_id, covers, terms):
    exact = [(decode(f), decode(x)) for f, x in terms]
    for f, x in terms:
        for v in (f, x):
            if v not in ("+0", "-0"):
                assert from_bits(bits_of(v)) == v
    return {
        "id": case_id,
        "covers": covers,
        "terms": [[encode(f), encode(x)] for f, x in terms],
        "absolute_sum": expected(absolute_sum(exact)),
        "srss": expected(srss(exact)),
    }


def references():
    records = [case_record(*c) for c in cases()]
    ids = [r["id"] for r in records]
    assert len(set(ids)) == len(ids)
    return {
        "reference": "RT-A4 V-1: absolute-sum and SRSS kernel table (DEL-05-02 Work absolute-sum-srss-combinations.md sections 3 and 7)",
        "generator": "validation/references/rt_a4_combinations/generators/rt_a4_reference.py",
        "encoding": "each term is [factor, value] as binary64 bit patterns; absolute_sum and srss are the expected result bits, or non_representable",
        "criterion": "bit-for-bit equality",
        "cases": records,
    }


def render():
    text = json.dumps(references(), indent=1, ensure_ascii=True) + "\n"
    files = {"references.json": text.encode("ascii")}
    for name in FIXED_FILES:
        files[name] = (OUT / name).read_bytes()
    sums = "".join(f"{hashlib.sha256(files[name]).hexdigest()}  {name}\n" for name in sorted(files))
    files["SHA256SUMS"] = sums.encode("ascii")
    return files


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    files = render()
    if args.write:
        for name, data in files.items():
            if name not in FIXED_FILES:
                (OUT / name).write_bytes(data)
        print("written")
        return 0
    stale = [name for name, data in files.items() if not (OUT / name).exists() or (OUT / name).read_bytes() != data]
    if stale:
        print("stale: " + ", ".join(sorted(stale)))
        return 1
    print("current")
    return 0


if __name__ == "__main__":
    sys.exit(main())
