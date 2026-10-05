"""RV81 independent exact-rational oracle for the TypeScript reader's exported arithmetic.
Uses Python Fraction only (no binary search shared with the reader's tests, no reader code).
RU(x) = least binary64 >= x (None if x > MAX finite). Inputs nonnegative finite unless noted."""
import json, random, struct, math
from fractions import Fraction as F
MAXW = 0x7fefffffffffffff
def w2f(w): return struct.unpack('>d', struct.pack('>Q', w))[0]
def f2w(x): return struct.unpack('>Q', struct.pack('>d', x))[0]
def exact(w):  # word -> Fraction (sign ignored only for nonneg words)
    e = (w >> 52) & 0x7ff; m = w & ((1 << 52) - 1)
    v = F(m, 1 << 1074) if e == 0 else F(m | (1 << 52)) * (F(2) ** (e - 1075))
    return -v if w >> 63 else v
MAXF = exact(MAXW)
def RU(x):
    """Least binary64 >= x >= 0 via exponent/significand construction (no search)."""
    assert x >= 0
    if x == 0: return 0
    if x > MAXF: return None
    # find e with 2^e <= x < 2^(e+1)
    e = x.numerator.bit_length() - x.denominator.bit_length()
    if F(2) ** e > x: e -= 1
    q = max(e - 52, -1074)  # quantum exponent
    n = x / (F(2) ** q); c = -((-n.numerator) // n.denominator)  # ceil
    v = F(c) * F(2) ** q
    if v > MAXF: return None
    return f2w(float(v)) if v == F(float(v)) else (_ for _ in ()).throw(AssertionError('not representable'))
def bits(w): return format(w, '016x')
rnd = random.Random(0x5281)
def sample_word():
    k = rnd.random()
    if k < 0.15: return rnd.randrange(1, 1 << 52)                     # subnormal
    if k < 0.25: return rnd.choice([0, 1, 2, 3, (1<<52)-1, 1<<52, (1<<52)+1, MAXW, MAXW-1, 0x3ff0000000000000, 0x3ff0000000000001, 0x3fefffffffffffff])
    if k < 0.40: return (rnd.randrange(1, 60) << 52) | rnd.randrange(0, 1 << 52)   # tiny normals
    if k < 0.50: return (rnd.randrange(0x7a0, 0x7ff) << 52) | rnd.randrange(0, 1 << 52)  # huge
    return rnd.randrange(1 << 52, MAXW + 1)
out = {"products": [], "small_sums": [], "bounds": [], "phi512": [], "ehat": []}
for _ in range(3000):
    a, b = sample_word(), sample_word()
    r = RU(exact(a) * exact(b)); out["products"].append([bits(a), bits(b), None if r is None else bits(r)])
for _ in range(2000):
    a, b = sample_word(), sample_word()
    r = RU(exact(a) + exact(b) + F(1, 1 << 1074)); out["small_sums"].append([bits(a), bits(b), None if r is None else bits(r)])
# absoluteBound(value, S) per C1:158 (value may be negative).
SMALL = F(2) ** -988
for i in range(3000):
    v = sample_word() | (rnd.getrandbits(1) << 63); S = sample_word()
    if i % 3 == 0: S = rnd.randrange(0, 1 << 52) if i % 2 else (rnd.randrange(1, 66) << 52) | rnd.randrange(0, 1 << 52)  # force the small branch often
    s = exact(S)
    if s == 0: b = 0
    elif s < SMALL:
        b0 = exact(RU(s / F(2) ** 64)); r = exact(RU(abs(exact(v)) / F(2) ** 53)); b = RU(b0 + r + F(1, 1 << 1074))
    else: b = RU(s / F(2) ** 64)
    out["bounds"].append([bits(v), bits(S), bits(b)])
for _ in range(3000):
    h = sample_word(); r = RU(exact(h) / F(2) ** 438); out["phi512"].append([bits(h), bits(r)])
# e_hat with round-to-nearest (Python float ops are IEEE binary64 nearest-even).
for _ in range(2000):
    f, m, L = (w2f(sample_word()) for _ in range(3))
    if rnd.random() < 0.1: L = 0.0
    if L == 0: r = [f, m]
    else:
        try: r = [max(f, m / L), max(m, L * f)]
        except OverflowError: continue
        if any(math.isinf(x) for x in r): continue
    out["ehat"].append([bits(f2w(f)), bits(f2w(m)), bits(f2w(L)), bits(f2w(r[0])), bits(f2w(r[1]))])
# stopFeasible: native generator per I57 s2 with D = false, enumerated independently.
feas = []
for present in range(16):
    for nonin in range(16):
        if nonin & ~present: continue  # non-input presence implies presence
        for L in (0, 1):
            for floor in (None, (0, 0), (0, 1), (1, 0), (1, 1)):
                gen = set()
                for A in range(16):
                    if A & ~nonin: continue
                    pos = [(A >> k) & 1 for k in range(4)]
                    if L: pos = [pos[0] | pos[1]] * 2 + [pos[2] | pos[3]] * 2
                    if floor: pos[2] |= floor[0]; pos[3] |= floor[1]
                    gen.add(tuple(((present >> k) & 1) & (pos[k] | ((A >> k) & 1)) for k in range(4)))
                for s in range(16):
                    st = tuple((s >> k) & 1 for k in range(4))
                    feas.append([present, nonin, L, list(floor) if floor else None, s, st in gen])
out["stop_feasible"] = feas
import os
json.dump(out, open(os.environ['WT'] + '/scratch/rv81_reader_review/oracle_vectors.json', 'w'))
print({k: len(v) for k, v in out.items()}, sum(1 for x in out['products'] if x[2] is None), 'overflow products')
