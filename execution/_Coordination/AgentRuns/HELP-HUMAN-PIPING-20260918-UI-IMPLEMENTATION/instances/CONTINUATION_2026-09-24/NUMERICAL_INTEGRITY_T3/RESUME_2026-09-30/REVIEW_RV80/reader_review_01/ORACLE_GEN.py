#!/usr/bin/env python3
"""RV80 independent exact-rational oracle for the Rust retained-precision helpers.

Every expected value is computed from fractions.Fraction (exact rationals) and an
explicit directed rounding to binary64; no reader function is consulted.
Writes oracle_vectors.json next to this file.
"""
import json, math, random, struct, sys
from fractions import Fraction as F
from pathlib import Path

MAXF = F(struct.unpack('<d', struct.pack('<Q', 0x7fefffffffffffff))[0])
TINY = F(1, 2**1074)


def b2f(b):
    return struct.unpack('<d', struct.pack('<Q', b))[0]


def f2b(x):
    return struct.unpack('<Q', struct.pack('<d', x))[0]


def rn(fr):
    """Round-to-nearest-even of an exact rational (CPython int/int is correctly rounded)."""
    if fr == 0:
        return 0.0
    try:
        return fr.numerator / fr.denominator
    except OverflowError:
        return math.inf if fr > 0 else -math.inf


def ru(fr):
    """Smallest binary64 >= fr, or None if above the largest finite."""
    if fr > MAXF:
        return None
    x = rn(fr)
    if math.isinf(x):
        return None
    if F(x) < fr:
        x = math.nextafter(x, math.inf)
    if math.isinf(x):
        return None
    return x + 0.0  # canonical +0


def hexb(x):
    return '%016x' % f2b(x)


rng = random.Random(80)


def rand_double(kind):
    if kind == 'normal':
        e = rng.randint(1, 2046)
    elif kind == 'small':
        e = rng.randint(1, 120)
    elif kind == 'sub':
        e = 0
    elif kind == 'big':
        e = rng.randint(1900, 2046)
    else:
        e = rng.randint(1000, 1100)
    m = rng.getrandbits(52)
    if e == 0 and m == 0:
        m = 1
    return b2f((e << 52) | m)


vectors = []
specials = [0.0, b2f(1), b2f(2), b2f(0x000fffffffffffff), b2f(0x0010000000000000),
            1.0, 3.0, 0.1, 2.0 ** -537, 2.0 ** -538, 2.0 ** 511, 2.0 ** 512,
            b2f(0x7fefffffffffffff), 1.0 + 2.0 ** -52, 2.0 ** -988, b2f(0x0230000000000000 - 1),
            2.0 ** -1022 * 1.5, 2.0 ** 1023]
kinds = ['normal', 'small', 'sub', 'big', 'mid']

# upward_product(a, b) = RU(a*b) for finite nonnegative operands, error on overflow.
pairs = [(a, b) for a in specials for b in specials]
for _ in range(1500):
    pairs.append((rand_double(rng.choice(kinds)), rand_double(rng.choice(kinds))))
for a, b in pairs:
    r = ru(F(a) * F(b))
    vectors.append({'fn': 'upward_product', 'args': [hexb(a), hexb(b)],
                    'expected': None if r is None else hexb(r)})
# negative / non-finite operands must be refused
for a, b in [(-1.0, 1.0), (1.0, -2.0), (math.inf, 1.0), (1.0, math.nan)]:
    vectors.append({'fn': 'upward_product', 'args': [hexb(a), hexb(b)], 'expected': None})

# upward_small_sum(b0, r) = RU(b0 + r + 2^-1074)
sums = [(a, b) for a in specials for b in specials]
for _ in range(1500):
    sums.append((rand_double(rng.choice(kinds)), rand_double(rng.choice(kinds))))
for a, b in sums:
    r = ru(F(a) + F(b) + TINY)
    vectors.append({'fn': 'upward_small_sum', 'args': [hexb(a), hexb(b)],
                    'expected': None if r is None else hexb(r)})


# absolute_bound(value, S): C1:158 / adaptive.rs row_bound
def bound(value, s):
    if s == 0:
        return 0.0
    b0 = ru(F(s) / 2 ** 64)
    if s < 2.0 ** -988:
        return ru(F(b0) + F(ru(abs(F(value)) / 2 ** 53)) + TINY)
    return b0


bounds = []
for s in [0.0, b2f(1), 2.0 ** -988, b2f(0x0230000000000000 - 1), 2.0 ** -1000, 2.0 ** -1060, 1.0, b2f(0x7fefffffffffffff)]:
    for v in [0.0, -0.0, b2f(1), -b2f(5), 2.0 ** -1000, -(2.0 ** -990), 1.0, -3.5, 2.0 ** -1060]:
        bounds.append((v, s))
for _ in range(1500):
    s = rand_double(rng.choice(['sub', 'small', 'normal']))
    v = rand_double(rng.choice(['sub', 'small', 'normal'])) * rng.choice([1, -1])
    bounds.append((v, s))
for v, s in bounds:
    r = bound(v, s)
    vectors.append({'fn': 'absolute_bound', 'args': [hexb(v), hexb(s)],
                    'expected': None if r is None else hexb(r)})
for v, s in [(1.0, -1.0), (math.inf, 1.0), (1.0, math.inf), (math.nan, 1.0)]:
    vectors.append({'fn': 'absolute_bound', 'args': [hexb(v), hexb(s)], 'expected': None})

# phi_512(e) = RU(2^-438 e), for e >= 0 finite
phis = [0.0, b2f(1), 2.0 ** -600, 2.0 ** -636, 2.0 ** -637, 3 * 2.0 ** -1000, 1.0, 2.0 ** -584, 2.0 ** -585 * 3, b2f(0x7fefffffffffffff)]
for _ in range(1500):
    phis.append(rand_double(rng.choice(kinds)))
for e in phis:
    r = ru(F(e) / 2 ** 438)
    vectors.append({'fn': 'phi_512', 'args': [hexb(e)], 'expected': hexb(r)})


# e_hat([fo, mo], L) = L==0 ? E : [max(fo, RN(mo/L)), max(mo, RN(L*fo))]
def ehat(fo, mo, L):
    if L == 0:
        return fo, mo
    a = rn(F(mo) / F(L))
    b = rn(F(L) * F(fo))
    return max(fo, a), max(mo, b)


for _ in range(1500):
    fo = rand_double(rng.choice(kinds))
    mo = rand_double(rng.choice(kinds))
    L = rng.choice([0.0, rand_double(rng.choice(['normal', 'mid', 'small']))])
    a, b = ehat(fo, mo, L)
    vectors.append({'fn': 'e_hat', 'args': [hexb(fo), hexb(mo), hexb(L)],
                    'expected': [hexb(a), hexb(b)]})

out = Path(__file__).with_name('oracle_vectors.json')
out.write_text(json.dumps({'generator': 'RV80 oracle_gen.py (fractions.Fraction, seed 80)', 'vectors': vectors}, indent=0))
from collections import Counter
print(len(vectors), Counter(v['fn'] for v in vectors))
