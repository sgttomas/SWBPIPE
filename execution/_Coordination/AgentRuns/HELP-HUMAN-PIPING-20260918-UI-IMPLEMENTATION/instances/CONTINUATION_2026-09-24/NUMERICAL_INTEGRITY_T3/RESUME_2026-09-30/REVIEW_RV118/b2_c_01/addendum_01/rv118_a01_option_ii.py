"""RV118 addendum 01: what ROOT's option (ii) for S-4 (a) touches in the items RV118 confirms (read-only; standard library only).

Option (ii) (RR "RV115 confirms S-4 (a)'s soundness with SA3-1; ..."): a combination's displacement_magnitude is RN64 of
the exact 3-norm of its frozen published components (mm). This script does not re-derive RV115's availability study.
It checks only the consequences for the items RV118 confirms:
  1. G7's 64-epsilon guard (base combination_magnitudes, as REVISION_01 §1.2 and §10.1's G7 row rely on):
     |p - r| <= 64*eps*max(|p|, MIN_POSITIVE), with p = RN64(sqrt(x^2+y^2+z^2)) computed exactly here (integer square
     root with a sticky bit, ties to even, subnormal results included) and r = binary64 hypot(hypot(x,y),z) of the same
     components in the reader's library, each hypot call taken faithful (the correctly rounded value or one ulp away).
  2. W-CB1z: every all-zero triple, signed zeros included, publishes canonical +0 under (ii).
  3. m69: a published magnitude multiplied by (1 + 2^-40) leaves the guard for every normal p, under r1's nested hypot
     and under (ii) alike.
  4. A tie: the exact 3-norm of binary64 components can be a binary64 midpoint, so "RN64" needs its tie rule stated.

Usage: python rv118_a01_option_ii.py <out_json>
"""
import itertools
import json
import math
import random
import sys
from fractions import Fraction

EPS = 2.0 ** -52
MIN_POSITIVE = 2.0 ** -1022


def exact_square_sum(x, y, z):
    return sum((Fraction(v) * Fraction(v) for v in (x, y, z)), Fraction(0))


def rn64_sqrt(q):
    """Round-to-nearest, ties-to-even binary64 of sqrt(q) for a nonnegative dyadic rational q. Returns (value, tie)."""
    if q == 0:
        return 0.0, False
    num, den = q.numerator, q.denominator  # den is a power of two
    k = den.bit_length() - 1
    if k % 2:
        num, k = num * 2, k + 1
    j = k // 2  # sqrt(q) = sqrt(num) / 2^j
    u = max(0, 64 - num.bit_length() // 2)  # at least 64 significant bits in the integer root
    t = num << (2 * u)
    s = math.isqrt(t)
    sticky = s * s != t  # sqrt(q) = (s + delta) / 2^(j+u), 0 <= delta < 1, delta > 0 iff sticky
    scale = j + u
    e = s.bit_length() - 1 - scale  # floor(log2 sqrt(q))
    qe = max(e - 52, -1074)  # the binary64 quantum's exponent at that binade
    shift = qe + scale
    if shift <= 0:
        return math.ldexp(s << -shift, qe), False
    kept, rem, half = s >> shift, s & ((1 << shift) - 1), 1 << (shift - 1)
    tie = rem == half and not sticky
    if rem > half or (rem == half and (sticky or kept & 1)):
        kept += 1
    value = math.ldexp(kept, qe)
    return value, tie


def faithful(v):
    return [v, math.nextafter(v, math.inf), math.nextafter(v, -math.inf)] if v != 0.0 else [0.0, math.nextafter(0.0, 1.0)]


def nested_variants(x, y, z):
    out = set()
    for inner in faithful(math.hypot(x, y)):
        if inner < 0 or not math.isfinite(inner):
            continue
        for outer in faithful(math.hypot(inner, z)):
            if outer >= 0 and math.isfinite(outer):
                out.add(outer)
    return sorted(out)


def allowance(p):
    return 64 * EPS * max(abs(p), MIN_POSITIVE)


def triples(seed, count):
    rnd = random.Random(seed)

    def draw():
        kind = rnd.random()
        if kind < 0.1:
            return rnd.choice([0.0, -0.0])
        if kind < 0.25:
            return rnd.choice([-1, 1]) * rnd.random() * 2.0 ** rnd.randint(-1074, -1022)
        return rnd.choice([-1, 1]) * rnd.random() * 2.0 ** rnd.randint(-1000, 1000)

    out = []
    for _ in range(count):
        base = draw()
        mode = rnd.random()
        if mode < 0.3:
            out.append((base, base * (1 + rnd.random() * 1e-3), -base))
        elif mode < 0.5:
            out.append((base, base * 2.0 ** rnd.randint(-80, -20), base * 2.0 ** rnd.randint(-80, -20)))
        else:
            out.append((draw(), draw(), draw()))
    return [t for t in out if all(math.isfinite(v) for v in t)]


def guard_under_ii(cases):
    worst, pairs, failures, ulps_worst, crosscheck = 0.0, 0, [], 0.0, 0
    for x, y, z in cases:
        p, _ = rn64_sqrt(exact_square_sum(x, y, z))
        if not math.isfinite(p):
            continue
        # cross-check the exact rounding against two directed neighbours: p's neighbours are farther from the norm
        q = exact_square_sum(x, y, z)
        if p > 0:
            lo, hi = Fraction(math.nextafter(p, 0.0)), Fraction(math.nextafter(p, math.inf))
            mid_lo, mid_hi = (lo + Fraction(p)) / 2, (hi + Fraction(p)) / 2
            assert mid_lo * mid_lo <= q <= mid_hi * mid_hi, (x, y, z)
            crosscheck += 1
        for r in nested_variants(x, y, z):
            pairs += 1
            ratio = abs(p - r) / allowance(p)
            worst = max(worst, ratio)
            if p > 0:
                ulps_worst = max(ulps_worst, abs(p - r) / (math.ulp(p)))
            if ratio > 1:
                failures.append([x.hex(), y.hex(), z.hex(), p.hex(), r.hex()])
    return {'triples': len(cases), 'pairs': pairs, 'exact_rounding_crosschecked': crosscheck,
            'largest_fraction_of_allowance': worst, 'largest_|p-r|_in_ulps_of_p': ulps_worst,
            'failure_count': len(failures), 'failures': failures[:5]}


def zeros():
    out = {}
    for t in itertools.product([0.0, -0.0], repeat=3):
        p, _ = rn64_sqrt(exact_square_sum(*t))
        out[','.join(v.hex() for v in t)] = {'p': p.hex(), 'canonical_plus_zero': p == 0.0 and math.copysign(1.0, p) > 0}
    return out


def m69(cases):
    worst = math.inf
    counted = 0
    for x, y, z in cases:
        p_ii, _ = rn64_sqrt(exact_square_sum(x, y, z))
        p_r1 = math.hypot(math.hypot(x, y), z)
        for p in (p_ii, p_r1):
            if not (math.isfinite(p) and p >= MIN_POSITIVE):
                continue
            edited = p * (1 + 2.0 ** -40)
            if not math.isfinite(edited):
                continue
            counted += 1
            for r in nested_variants(x, y, z):
                worst = min(worst, abs(edited - r) / allowance(edited))
    return {'normal_magnitudes_edited': counted, 'smallest_|edited-r|_over_allowance': worst}


def tie_example():
    # a = 2^26 + 1, b = 2^26: (a^2 - b^2, 2ab, 0) has the exact norm a^2 + b^2 = 2^53 + 2^27 + 1, an odd integer in
    # [2^53, 2^54), i.e. a binary64 midpoint. Scaled by 2^-60 (exact) to a plausible size in mm.
    a, b = 2 ** 26 + 1, 2 ** 26
    x, y, z = math.ldexp(a * a - b * b, -60), math.ldexp(2 * a * b, -60), 0.0
    assert Fraction(x) == Fraction(a * a - b * b, 2 ** 60) and Fraction(y) == Fraction(2 * a * b, 2 ** 60)
    q = exact_square_sum(x, y, z)
    norm = Fraction(a * a + b * b, 2 ** 60)
    assert norm * norm == q
    p, tie = rn64_sqrt(q)
    down, up = math.ldexp(2 ** 53 + 2 ** 27, -60), math.ldexp(2 ** 53 + 2 ** 27 + 2, -60)
    return {'components_mm': [x.hex(), y.hex(), z.hex()], 'exact_norm_is_a_binary64_midpoint':
            Fraction(down) < norm < Fraction(up) and norm - Fraction(down) == Fraction(up) - norm,
            'tie_detected': tie, 'rn64_ties_to_even': p.hex(), 'equals_lower_even_neighbour': p == down,
            'this_libm_nested_hypot': math.hypot(math.hypot(x, y), z).hex(),
            'guard_holds_for_this_libm': abs(p - math.hypot(math.hypot(x, y), z)) <= allowance(p)}


def main():
    out_path = sys.argv[1]
    cases = triples(20261008, 20000)
    result = {
        'guard_under_option_ii': guard_under_ii(cases),
        'w_cb1z_zero_triples': zeros(),
        'm69_edit_under_r1_and_option_ii': m69(cases),
        'tie_example': tie_example(),
    }
    text = json.dumps(result, indent=1, sort_keys=True) + '\n'
    with open(out_path, 'w', encoding='ascii') as f:
        f.write(text)
    print(json.dumps({k: (v if k != 'w_cb1z_zero_triples' else 'see out') for k, v in result.items()}, indent=1))


if __name__ == '__main__':
    main()
