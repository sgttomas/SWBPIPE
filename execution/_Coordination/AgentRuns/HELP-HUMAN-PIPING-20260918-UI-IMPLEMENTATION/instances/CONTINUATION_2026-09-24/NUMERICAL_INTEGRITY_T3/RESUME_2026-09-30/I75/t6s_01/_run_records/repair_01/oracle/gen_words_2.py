"""I75 REPAIR_01, second word lists (seeded; stdlib only), after the first-form probe found 16-digit ties.
ties16.txt: 30,000 words m*2^-k (k 1..14) whose mantissa's lowest set bit is t (0..6) and whose exact decimal
has at most 17 significant digits, ending in 5; both signs. These carry the 16-digit ties.
pow2.txt: every power of two in binary64 (2^-1074 .. 2^1023) and both neighbours of each, both signs."""
import math, random, struct, sys, pathlib
from fractions import Fraction
from decimal import Decimal, getcontext
getcontext().prec = 2000
out = pathlib.Path(sys.argv[1]); rng = random.Random(75016)
def word(x): return struct.pack(">d", x).hex()
def exact_digits(x):
    f = Fraction(abs(x))
    if f == 0: return ""
    return format(Decimal(f.numerator) / Decimal(f.denominator), "f").replace(".", "").lstrip("0").rstrip("0")
ties, seen = [], set()
while len(ties) < 30000:
    t = rng.randint(0, 6); k = rng.randint(1, 14)
    m = ((1 << 52) | rng.getrandbits(52)) >> t << t | (1 << t)
    if m >= 1 << 53: continue
    x = m / (1 << k)
    s = exact_digits(x)
    if len(s) <= 17 and s.endswith("5") and x != int(x):
        if rng.random() < 0.5: x = -x
        w = word(x)
        if w not in seen: seen.add(w); ties.append(w)
pw = []
for e in range(-1074, 1024):
    x = math.ldexp(1.0, e)
    for y in (math.nextafter(x, 0.0), x, math.nextafter(x, math.inf)):
        if y != 0 and math.isfinite(y):
            for z in (y, -y): pw.append(word(z))
pw = list(dict.fromkeys(pw))
(out / "ties16.txt").write_text("\n".join(ties) + "\n"); (out / "pow2.txt").write_text("\n".join(pw) + "\n")
print("ties16", len(ties), "pow2", len(pw))
