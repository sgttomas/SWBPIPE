"""RV101 ADDENDUM_01 word lists (stdlib only; seeded). Usage: gen_rv101_words.py <out dir>
rv101_ties17.txt  exact 18-digit decimals ending in 5 (17-digit tie candidates): random 52-bit mantissas in binades
                  2^44..2^53, and m*2^-k (k<=25); both signs (the generator of RV101's review, seed 25).
rv101_ties16.txt  RV101's own 16-digit tie construction: m odd, k in 1..24, digits(m*5^k) == 17 (so the exact
                  decimal has 17 significant digits ending in 5), m < 2^53; both signs; seed 101016.
rv101_small.txt   the small-magnitude edge: the same constructions at k = 23, 24, 25, 26 (both signs), where a tie
                  first becomes possible (|x| >= 2^-25).
rv101_pow2.txt    every power of two 2^-1074..2^1023, its neighbours one ulp either side, and two ulps above; both signs.
vectors.txt       the 69 words of the repaired test, parsed from the test file (argv[2])."""
import math, random, re, struct, sys, pathlib
from decimal import Decimal
from fractions import Fraction
out = pathlib.Path(sys.argv[1]); out.mkdir(parents=True, exist_ok=True)
w = lambda x: struct.pack('>d', x).hex()
def exact_sig(x):
    f = Fraction(abs(x)); s = format(Decimal(f.numerator) / Decimal(f.denominator), 'f') if f.denominator != 1 else str(f.numerator)
    return s.replace('.', '').lstrip('0').rstrip('0')
# 17-digit tie candidates (as in the review)
random.seed(25); cands = []
def tie18(x): s = exact_sig(x); return len(s) == 18 and s[-1] == '5'
for e in range(44, 54):
    for _ in range(4000):
        x = struct.unpack('>d', struct.pack('>Q', ((1023 + e) << 52) | random.getrandbits(52)))[0]
        if tie18(x): cands.append(x)
for k in range(1, 26):
    for _ in range(3000):
        x = (random.randrange(1, 2 ** 53) | 1) * 2.0 ** -k
        if tie18(x): cands.append(x)
t17 = sorted({w(v) for x in cands for v in (x, -x)})
# 16-digit construction
rng = random.Random(101016); t16 = set()
for k in range(1, 25):
    lo = -(-10 ** 16 // 5 ** k); hi = min((10 ** 17 - 1) // 5 ** k, 2 ** 53 - 1)
    if lo > hi: continue
    for _ in range(1500):
        m = rng.randint(lo, hi) | 1
        if m > hi: continue
        x = m / 2 ** k
        assert Fraction(x) == Fraction(m, 2 ** k) and len(str(m * 5 ** k)) == 17
        t16.add(w(x)); t16.add(w(-x))
# small-magnitude edge
small = set()
for k in (23, 24, 25, 26):
    for digits in (17, 18):
        lo = -(-10 ** (digits - 1) // 5 ** k); hi = min((10 ** digits - 1) // 5 ** k, 2 ** 53 - 1)
        for m in range(max(lo, 1), min(hi, max(lo, 1) + 400) + 1):
            if m % 2 == 0: continue
            x = m / 2 ** k; small.add(w(x)); small.add(w(-x))
# powers of two
p2 = []
for e in range(-1074, 1024):
    x = math.ldexp(1.0, e)
    for y in (math.nextafter(x, 0.0), x, math.nextafter(x, math.inf), math.nextafter(math.nextafter(x, math.inf), math.inf)):
        if y != 0 and math.isfinite(y):
            p2 += [w(y), w(-y)]
p2 = list(dict.fromkeys(p2))
vec = re.findall(r'\["([0-9a-f]{16})", "([^"]+)"\]', pathlib.Path(sys.argv[2]).read_text())
(out / 'rv101_ties17.txt').write_text('\n'.join(t17) + '\n'); (out / 'rv101_ties16.txt').write_text('\n'.join(sorted(t16)) + '\n')
(out / 'rv101_small.txt').write_text('\n'.join(sorted(small)) + '\n'); (out / 'rv101_pow2.txt').write_text('\n'.join(p2) + '\n')
(out / 'vectors.txt').write_text('\n'.join(a for a, _ in vec) + '\n'); (out / 'vectors_expected.tsv').write_text(''.join(f'{a}\t{b}\n' for a, b in vec))
print('ties17', len(t17), 'ties16', len(t16), 'small', len(small), 'pow2', len(p2), 'vectors', len(vec))
