"""I75 REPAIR_01: deterministic binary64 word lists for the {:e} tie check (seeded; stdlib only).
random.txt: 60,000 words over every binade (30,000 normal, 28,000 subnormal, the zeros, and every corpus/milestone bound).
ties.txt: 21,000 exact-tie words (exact decimal of 18 significant digits ending in 5), both signs:
binades 2^44..2^53 with random 52-bit mantissas, and m*2^-k with k <= 25."""
import json, random, struct, sys, pathlib
from fractions import Fraction
from decimal import Decimal, getcontext
getcontext().prec = 2000
P = pathlib.Path(sys.argv[1]); out = pathlib.Path(sys.argv[2])
rng = random.Random(75006)
def word(x): return struct.pack(">d", x).hex()
def val(w): return struct.unpack(">d", bytes.fromhex(w))[0]
def exact_digits(x):
    f = Fraction(abs(x))
    if f == 0: return ""
    d = Decimal(f.numerator) / Decimal(f.denominator)
    s = format(d, "f").replace(".", "").lstrip("0").rstrip("0")
    return s
words = set()
while len(words) < 30000:
    w = rng.getrandbits(64)
    if (w >> 52) & 0x7ff in (0, 0x7ff): continue
    words.add(f"{w:016x}")
while len(words) < 58000:
    w = (rng.getrandbits(1) << 63) | rng.getrandbits(52)
    if w & ((1 << 52) - 1) == 0: continue
    words.add(f"{w:016x}")
words |= {"0000000000000000", "8000000000000000"}
bounds = set()
for path in ["fixtures/results/retained_precision_cases.json", "fixtures/results/retained_precision_milestone_successor_sparse_interactive.json", "fixtures/results/retained_precision_milestone_successor_dense_scrutiny.json"]:
    def walk(v):
        if isinstance(v, dict):
            for k, x in v.items():
                if k == "absolute_verified" and isinstance(x, list):
                    for e in x:
                        if isinstance(e, dict) and isinstance(e.get("bound"), str): bounds.add(e["bound"])
                walk(x)
        elif isinstance(v, list):
            for x in v: walk(x)
    walk(json.loads((P / path).read_text()))
words |= bounds
while len(words) < 60000:
    w = rng.getrandbits(64)
    if (w >> 52) & 0x7ff == 0x7ff: continue
    words.add(f"{w:016x}")
ties = []
seen = set()
while len(ties) < 21000:
    if rng.random() < 0.5:
        e = rng.randint(44, 52); m = (1 << 52) | rng.getrandbits(52); x = float(Fraction(m, 1 << 52) * (1 << e))
    else:
        k = rng.randint(1, 25); m = rng.getrandbits(53) or 1; x = m / (1 << k)
    if rng.random() < 0.5: x = -x
    s = exact_digits(x)
    if len(s) == 18 and s.endswith("5"):
        w = word(x)
        if w not in seen: seen.add(w); ties.append(w)
(out / "random.txt").write_text("\n".join(sorted(words)) + "\n")
(out / "ties.txt").write_text("\n".join(ties) + "\n")
print("random", len(words), "bounds", len(bounds), "ties", len(ties))
