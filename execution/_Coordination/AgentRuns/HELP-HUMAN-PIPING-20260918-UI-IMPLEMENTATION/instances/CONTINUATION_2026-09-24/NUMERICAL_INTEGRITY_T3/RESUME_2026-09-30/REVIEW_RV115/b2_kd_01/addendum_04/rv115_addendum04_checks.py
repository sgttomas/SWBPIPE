#!/usr/bin/env python3
"""RV115 ADDENDUM_04: option (ii)'s numerics (standard library only).

argv: I97's b2c_checks_r2.py (only `rn64_norm3` is taken from it, as the thing under test),
      I97's exact_norm_vectors.json, DEF-C r1, DEF-C r2, DEF-O. Writes only to stdout.

My oracle is computed differently from I97's: a 1,300-digit Decimal square root of the
exact sum, rounded to binary64 by my own grid rounding, then corrected exactly against
Fraction midpoints of the true binary64 neighbours (from bit patterns), ties to even;
refusal iff S >= (MAX + ulp(MAX)/2)^2.

1. rn64_norm3 against my oracle: the SA3-1 study's triples (seed 115003, my generator),
   my guard triples, I97's 411 vectors, random bit patterns over the whole range,
   constructed exact midpoints (Pythagorean quadruples, scaled), near-midpoints, and
   curated edges.
2. A-5: the midpoint, the midpoint + 2^-1074, and what RN1024(S)-then-sqrt and nested
   hypot give.
3. Availability under (ii): the gate on the SA3-1 rows (point enclosure), v0 vs (ii).
4. G7's guard: |p - r| for r in my exact faithful nested-hypot set (RD/RU at each call).
5. B2-K's formation: a pipeline model (S rounded to 1,024 bits, a 1,024-bit sqrt, RN64,
   then the exact midpoint side) and two traps: the lower midpoint at 2^-1022, and MAX's
   upper midpoint (the overflow threshold).
6. DEF-C r2 against r1 (changed paths) and H, with DEF-O as the control.
"""
import decimal, hashlib, importlib.util, json, math, random, struct, sys
from fractions import Fraction as F

MAX = sys.float_info.max
TINY = 5e-324
MINP = 2.0 ** -1022

def b2f(h):
    return struct.unpack('>d', bytes.fromhex(h))[0]

def f2b(v):
    return struct.pack('>d', v).hex()

def succ(v):
    u = struct.unpack('<Q', struct.pack('<d', v))[0]
    return struct.unpack('<d', struct.pack('<Q', u + 1))[0]

def pred(v):
    u = struct.unpack('<Q', struct.pack('<d', v))[0]
    return struct.unpack('<d', struct.pack('<Q', u - 1))[0]

THRESH = F(MAX) + F(2) ** 970          # MAX + ulp(MAX)/2

def exact_S(x, y, z):
    return F(x) ** 2 + F(y) ** 2 + F(z) ** 2

def grid_round(r):
    """Nearest binary64 to a positive Fraction r on the binary64 grid (ties to even); inf beyond."""
    e = r.numerator.bit_length() - r.denominator.bit_length()
    if r < F(2) ** e:
        e -= 1
    q = max(e - 52, -1074)
    k = r / F(2) ** q
    m = k.numerator // k.denominator
    rem = k - m
    if rem > F(1, 2) or (rem == F(1, 2) and m % 2 == 1):
        m += 1
    if m.bit_length() + q > 1024:
        return math.inf
    return math.ldexp(m, q)

D = decimal.Context(prec=1300, Emin=-999999, Emax=999999)

def oracle(x, y, z):
    """(value or None if refused, tie)."""
    S = exact_S(x, y, z)
    if S == 0:
        return 0.0, False
    if S >= THRESH * THRESH:
        return None, S == THRESH * THRESH
    root = D.sqrt(D.divide(decimal.Decimal(S.numerator), decimal.Decimal(S.denominator)))
    c = grid_round(F(root))
    if c == math.inf:
        c = MAX
    for _ in range(4):
        lo, hi = pred(c) if c > 0 else 0.0, succ(c)
        mlo = (F(lo) + F(c)) / 2
        mhi = THRESH if c == MAX else (F(c) + F(hi)) / 2
        if S < mlo * mlo:
            c = lo
            continue
        if S > mhi * mhi:
            c = hi
            continue
        tie = S == mlo * mlo or S == mhi * mhi
        if tie:
            even = struct.unpack('<Q', struct.pack('<d', c))[0] % 2 == 0
            if not even:
                c = lo if S == mlo * mlo else hi
        return c, tie
    raise RuntimeError('oracle did not settle')

def load(path, name):
    spec = importlib.util.spec_from_file_location(name, path)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod

i97 = load(sys.argv[1], 'i97_r2')
rn64_norm3 = i97.rn64_norm3
vectors = json.load(open(sys.argv[2]))['vectors']
out = {}

def same(a, b):
    (pa, ta), (pb, tb) = a, b
    if pa is None or pb is None:
        return pa is None and pb is None
    return f2b(pa) == f2b(pb) and ta == tb

# ---------------------------------------------------------------- triples
rng = random.Random(115003)
def sample(kind):
    e = rng.randint(-10, 6)
    if kind == 'comparable':
        c = [rng.uniform(-1, 1) for _ in range(3)]
    elif kind == 'one_small':
        c = [rng.uniform(-1, 1), rng.uniform(-1, 1), rng.uniform(-1, 1) * 2.0 ** -rng.randint(4, 30)]
    else:
        c = [rng.uniform(-1, 1), rng.uniform(-1, 1), 0.0]
    q = []
    for v in c:
        q.append(F(0) if v == 0.0 else F(v * 2.0 ** e) * (1 + F(rng.randint(-2 ** 40, 2 ** 40), 2 ** 95)))
    return q
sa3 = {k: [sample(k) for _ in range(3000)] for k in ('comparable', 'one_small', 'planar')}
guard_triples = []
for _ in range(3000):
    band = rng.choice([(-1074, -1023), (-1060, -1000), (-60, 60), (900, 1000)])
    guard_triples.append(tuple(rng.choice([1, -1]) * rng.random() * 2.0 ** rng.randint(*band) for _ in range(3)))
rb = random.Random(4)
def rand_bits():
    while True:
        v = struct.unpack('<d', struct.pack('<Q', rb.getrandbits(64)))[0]
        if math.isfinite(v):
            return v
random_bits = [(rand_bits(), rand_bits(), rand_bits()) for _ in range(3000)]
spread = []
for _ in range(2000):
    big = rb.choice([1, -1]) * rb.random() * 2.0 ** rb.randint(-1074, 1023)
    spread.append((big, big * 2.0 ** -rb.randint(0, 2000) if big else 0.0, rb.choice([0.0, -0.0, TINY, -TINY, MINP])))

# constructed exact midpoints: a^2+b^2+c^2=d^2 (Euler/Lebesgue), d odd 54-bit, scaled by 2^s
def quadruples(n):
    out_ = []
    r = random.Random(77)
    while len(out_) < n:
        m, nn, p, qq = (r.randint(2 ** 25, 2 ** 26) for _ in range(4))
        a = m * m + nn * nn - p * p - qq * qq
        b = 2 * (m * qq + nn * p)
        c = 2 * (nn * qq - m * p)
        d = m * m + nn * nn + p * p + qq * qq
        if d % 2 == 0 or d.bit_length() != 54:
            continue
        if any(abs(v) >= 2 ** 53 and v % 2 for v in (a, b, c)):
            continue
        if any(abs(v) >= 2 ** 54 for v in (a, b, c)):
            continue
        s = r.randint(-1000, 900)
        t = tuple(math.ldexp(v, s) for v in (a, b, c))
        if F(t[0]) ** 2 + F(t[1]) ** 2 + F(t[2]) ** 2 == F(d) ** 2 * F(2) ** (2 * s):
            out_.append(t)
    return out_
ties = quadruples(500)
near_ties = []
for t in ties[:250]:
    near_ties.append((succ(t[0]) if t[0] > 0 else pred(t[0]), t[1], t[2]))
    near_ties.append((t[0], t[1], TINY))

curated = [(0.0, 0.0, 0.0), (-0.0, -0.0, -0.0), (0.0, -0.0, 0.0), (TINY, 0.0, 0.0), (TINY, -TINY, TINY),
           (MINP - TINY,) * 3, (MINP, TINY, -TINY), (MAX, 0.0, -0.0), (MAX, MAX, 0.0), (MAX, TINY, 0.0),
           (math.ldexp(1, 1023), math.ldexp(1, 1023), 0.0), (1e308, 1e308, 1e308), (1e200, -1e200, 1e200),
           (1e-200, 1e-200, -1e-200), (1e300, 1e-300, TINY), (1.0, TINY, 0.0), (MAX, math.ldexp(1, 997), 0.0),
           (MAX, math.ldexp(1, 996), math.ldexp(1, 996))]

def compare(triples):
    agree, refused, tie_n, bad = 0, 0, 0, []
    for t in triples:
        t = tuple(float(v) for v in t)
        a, b = rn64_norm3(*t), oracle(*t)
        if same(a, b):
            agree += 1
        else:
            bad.append([f2b(v) for v in t])
        refused += b[0] is None
        tie_n += b[1]
    return {'triples': len(triples), 'agree': agree, 'refused': refused, 'ties': tie_n, 'disagreements': bad[:5]}

vec_ok, vec_bad = 0, []
for v in vectors:
    t = (b2f(v['x']), b2f(v['y']), b2f(v['z']))
    o = oracle(*t)
    exp_p = None if v['p'] in (None, 'refused') else b2f(v['p'])
    if (o[0] is None and exp_p is None) or (o[0] is not None and exp_p is not None and f2b(o[0]) == f2b(exp_p) and o[1] == v['tie']):
        vec_ok += 1
    else:
        vec_bad.append(v['label'])
out['1_rn64_norm3_vs_my_oracle'] = {
    'sa3_published_components': compare([t for k in sa3 for t in sa3[k]]),
    'guard_triples': compare(guard_triples),
    'random_bit_patterns': compare(random_bits),
    'extreme_spreads': compare(spread),
    'constructed_exact_midpoints': compare(ties),
    'near_midpoints': compare(near_ties),
    'curated': compare(curated),
    'I97_vectors': {'vectors': len(vectors), 'my_oracle_agrees': vec_ok, 'disagree': vec_bad},
}

# ---------------------------------------------------------------- 2. A-5
x5, y5 = math.ldexp(2 ** 27 + 1, -60), math.ldexp(2 ** 53 + 2 ** 27, -60)
def rn1024(r):
    e = r.numerator.bit_length() - r.denominator.bit_length()
    if r < F(2) ** e:
        e -= 1
    qq = F(2) ** (e - 1023)
    k = r / qq
    m = k.numerator // k.denominator
    rem = k - m
    if rem > F(1, 2) or (rem == F(1, 2) and m % 2):
        m += 1
    return m * qq
a5 = {}
for label, z in (('midpoint', 0.0), ('midpoint_plus_2^-1074', TINY)):
    S = exact_S(x5, y5, z)
    p_ok = oracle(x5, y5, z)
    S1024 = rn1024(S)
    via1024 = oracle_from = None
    r = D.sqrt(D.divide(decimal.Decimal(S1024.numerator), decimal.Decimal(S1024.denominator)))
    # nearest binary64 to sqrt(S1024): if S1024 is itself an exact midpoint square, ties to even
    c = grid_round(F(r))
    mlo = (F(pred(c)) + F(c)) / 2
    mhi = (F(c) + F(succ(c))) / 2
    if S1024 == mhi * mhi and struct.unpack('<Q', struct.pack('<d', c))[0] % 2:
        c = succ(c)
    if S1024 == mlo * mlo and struct.unpack('<Q', struct.pack('<d', c))[0] % 2:
        c = pred(c)
    a5[label] = {'exact': f2b(p_ok[0]), 'tie': p_ok[1], 'rn64_norm3': f2b(rn64_norm3(x5, y5, z)[0]),
                 'S_minus_m2_log2': (lambda d: None if d == 0 else (d.numerator.bit_length() - d.denominator.bit_length()))(S - ((F(y5) + F(math.ulp(y5)) / 2) ** 2)),
                 'S_rounded_to_1024_bits_equals_m2': S1024 == (F(y5) + F(math.ulp(y5)) / 2) ** 2,
                 'via_S_rounded_to_1024_bits': f2b(c),
                 'nested_math_hypot': f2b(math.hypot(math.hypot(x5, y5), z))}
out['2_A5'] = a5

# ---------------------------------------------------------------- 3. availability on the SA3-1 rows
def allowance_ok(n, truth, S_):
    m = max(abs(F(n)), F(S_))
    a_exact = m * F(2) ** -64 + m * F(2) ** -85 + abs(F(n)) * F(2) ** -53 + F(TINY)
    a0 = 2.0 ** -64 * max(abs(n), S_)
    a1 = a0 * struct.unpack('<d', struct.pack('<Q', 0x3ff0000080000000))[0]
    u1 = 2.0 ** -53 * abs(n) + TINY
    h = abs(F(n) - truth)
    return h <= a_exact and h <= F(a1 + u1)
def sqrt_hi(s, bits=400):
    if s == 0:
        return F(0)
    k = max(bits - (s.numerator.bit_length() - s.denominator.bit_length()) // 2, 0)
    return F(math.isqrt(s.numerator * (1 << (2 * k)) // s.denominator), 1 << k)
avail = {}
for kind, rows in sa3.items():
    st = {'v0_fail': 0, 'ii_fail': 0}
    for q in rows:
        xs = [float(v) for v in q]
        T = sqrt_hi(sum(v * v for v in q)) / 1000
        S_true = sum(v * v for v in q)
        y_v0 = grid_round(F(D.sqrt(D.divide(decimal.Decimal(S_true.numerator), decimal.Decimal(S_true.denominator)))))
        n_v0 = float(F(y_v0) / 1000)
        p_ii = rn64_norm3(*xs)[0]
        n_ii = float(F(p_ii) / 1000)
        st['v0_fail'] += not allowance_ok(n_v0, T, n_v0)
        st['ii_fail'] += not allowance_ok(n_ii, T, n_ii)
    avail[kind] = st
out['3_availability_point_enclosure_S_eq_n'] = avail

# ---------------------------------------------------------------- 4. the guard against exact faithful nested hypot
def dir_sqrt(S):
    """(RD, RU) binary64 of sqrt(S), exact."""
    if S == 0:
        return 0.0, 0.0
    r = D.sqrt(D.divide(decimal.Decimal(S.numerator), decimal.Decimal(S.denominator)))
    c = grid_round(F(r))
    while F(c) * F(c) > S:
        c = pred(c)
    while F(succ(c)) * F(succ(c)) <= S:
        c = succ(c)
    return (c, c) if F(c) * F(c) == S else (c, succ(c))
worst_ulps, worst_frac, pairs, fails = 0.0, 0.0, 0, 0
for t in [tuple(float(v) for v in tt) for k in sa3 for tt in sa3[k]][:3000] + guard_triples[:1500] + ties[:200]:
    p, _ = rn64_norm3(*t)
    if p is None:
        continue
    rs = set()
    for inner in dir_sqrt(F(t[0]) ** 2 + F(t[1]) ** 2):
        if not math.isfinite(inner):
            continue
        rs |= set(dir_sqrt(F(inner) ** 2 + F(t[2]) ** 2))
    for r_ in rs:
        if not math.isfinite(r_):
            continue
        pairs += 1
        allow = F(64) * F(2.0 ** -52) * max(abs(F(p)), F(MINP))
        d = abs(F(p) - F(r_))
        fails += d > allow
        worst_frac = max(worst_frac, float(d / allow))
        if p > 0:
            worst_ulps = max(worst_ulps, float(d / F(math.ulp(p))))
out['4_guard'] = {'pairs': pairs, 'failures': fails, 'max_|p-r|_ulps_of_p': worst_ulps, 'max_fraction_of_allowance': worst_frac}

# ---------------------------------------------------------------- 5. B2-K's pipeline model and its traps
def pipeline(x, y, z, lower_mid_rule='neighbours'):
    """S rounded to 1024 bits, a correctly rounded 1024-bit sqrt, RN64, then the exact midpoint side (one step)."""
    S = exact_S(x, y, z)
    if S == 0:
        return 0.0
    S1 = rn1024(S)
    est = rn1024(sqrt_hi(S1, 1200))           # a 1024-bit sqrt (rounding of a 1200-bit value; enough here)
    y0 = grid_round(est)
    if y0 == math.inf:
        # condition F-3: an overflowing estimate is decided exactly against MAX's upper midpoint
        return 'refused' if S >= THRESH * THRESH else MAX
    if y0 == 0:
        return 'zero_estimate'
    def mids(c):
        if lower_mid_rule == 'neighbours':
            lo = (F(pred(c)) + F(c)) / 2
        else:   # "half as far at a binade's first value" applied at every power of two
            frac = F(c) / F(2) ** (F(c).numerator.bit_length() - F(c).denominator.bit_length())
            is_pow2 = F(c).numerator & (F(c).numerator - 1) == 0 and F(c).denominator & (F(c).denominator - 1) == 0
            q = F(math.ulp(c))
            lo = F(c) - (q / 4 if is_pow2 and c >= MINP else q / 2)
        hi = THRESH if c == MAX else (F(c) + F(succ(c))) / 2
        return lo, hi
    lo, hi = mids(y0)
    if S < lo * lo:
        y0 = pred(y0)
    elif S > hi * hi:
        if y0 == MAX:
            return 'refused'
        y0 = succ(y0)
    elif S == lo * lo or S == hi * hi:
        if struct.unpack('<Q', struct.pack('<d', y0))[0] % 2:
            y0 = pred(y0) if S == lo * lo else succ(y0)
    return y0
pipe_ok, pipe_bad = 0, []
for t in [tuple(float(v) for v in tt) for k in sa3 for tt in sa3[k]][:1500] + ties[:300] + near_ties[:300] + curated + random_bits[:800]:
    want = oracle(*t)[0]
    got = pipeline(*t)
    if (want is None and got == 'refused') or (isinstance(got, float) and want is not None and f2b(got) == f2b(want)):
        pipe_ok += 1
    else:
        pipe_bad.append([[f2b(v) for v in t], str(got), None if want is None else f2b(want)])
# trap 1: sqrt(S) just above the true lower midpoint of 2^-1022 (between L + 2^-1075 and L + 1.5*2^-1075)
L = MINP - TINY
trap1 = None
for k in range(int(1.0 * 2 ** 26) + 1, int(1.25 * 2 ** 26), 997):
    yv = math.ldexp(k, -1074)
    S = F(L) ** 2 + F(yv) ** 2
    lo_true = F(L) + F(2) ** -1075
    lo_wrong = F(MINP) - F(2) ** -1076
    if lo_true * lo_true < S < lo_wrong * lo_wrong:
        trap1 = {'x': f2b(L), 'y': f2b(yv), 'z': f2b(0.0), 'correct': f2b(oracle(L, yv, 0.0)[0]),
                 'rn64_norm3': f2b(rn64_norm3(L, yv, 0.0)[0]),
                 'pipeline_with_true_neighbour_midpoints': f2b(pipeline(L, yv, 0.0)),
                 'pipeline_with_half_as_far_below_2^-1022': f2b(pipeline(L, yv, 0.0, 'binade_rule'))}
        break
out['5_formation'] = {'pipeline_agrees_with_oracle': pipe_ok, 'pipeline_cases': pipe_ok + len(pipe_bad),
                      'pipeline_disagreements': pipe_bad[:5], 'trap_lower_midpoint_at_2^-1022': trap1,
                      'MAX_upper_midpoint': {'(MAX,0,0)': pipeline(MAX, 0.0, 0.0) == MAX,
                                             '(MAX,MAX,0) refused': pipeline(MAX, MAX, 0.0) == 'refused' or pipeline(MAX, MAX, 0.0) == 'overflow_estimate'}}

# ---------------------------------------------------------------- 6. DEF-C r2 against r1, and H
r1, r2, defo = (json.load(open(p)) for p in sys.argv[3:6])
raw_r2 = open(sys.argv[4], 'rb').read()
def leaves(v, path=''):
    if isinstance(v, dict):
        o = {}
        for k in v:
            o.update(leaves(v[k], f'{path}/{k}'))
        return o
    return {path: json.dumps(v, sort_keys=True)}
a, b = leaves(r1), leaves(r2)
def jcs(v):
    return json.dumps(v, ensure_ascii=False, separators=(',', ':'), sort_keys=True, allow_nan=False)
def H(d, v):
    return hashlib.sha256(jcs({'domain': d, 'payload': v}).encode('utf-8')).hexdigest()
out['6_defc'] = {'changed_leaf_paths_r1_to_r2': sorted(p for p in set(a) | set(b) if a.get(p) != b.get(p)),
                 'r2_texts': {p: json.loads(b[p]) for p in sorted(p for p in set(a) | set(b) if a.get(p) != b.get(p)) if p in b},
                 'DEF_O_H_control': H('retained_precision_formation_v1', defo),
                 'DEF_C_r2_raw_sha256': hashlib.sha256(raw_r2).hexdigest(),
                 'DEF_C_r2_raw_is_canonical': jcs(r2).encode('utf-8') == raw_r2,
                 'DEF_C_r2_H': H('retained_precision_formation_v1', r2),
                 'operand_definition_is_H_DEF_O': r2['inherits']['operand_definition']['sha256'] == H('retained_precision_formation_v1', defo),
                 'support_magnitude_equals_DEF_O': r2['rows']['support_magnitude'] == defo['rows']['support_magnitude']}
print(json.dumps(out, indent=1, sort_keys=True, ensure_ascii=True))
