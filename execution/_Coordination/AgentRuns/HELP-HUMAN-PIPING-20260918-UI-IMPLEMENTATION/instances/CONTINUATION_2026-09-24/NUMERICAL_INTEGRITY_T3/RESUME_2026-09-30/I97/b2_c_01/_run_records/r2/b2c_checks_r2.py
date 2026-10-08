"""I97 B2-C revision 02: option (ii)'s reference implementation and its checks (records only; standard library only).

`rn64_norm3(x, y, z)` is the reference for DEF-C r2's `rows.displacement_magnitude`: RN64, ties to even, of the exact
3-norm sqrt(x^2 + y^2 + z^2) of three finite binary64 values (mm). Integers only: each component is an exact dyadic
n/2^k, the sum of squares an exact integer N over 2^(2k), and the root is rounded once from math.isqrt with a sticky
bit, at the binary64 quantum of its binade (the subnormal quantum below 2^-1022). A norm whose rounding is beyond
binary64's range is refused (None). An all-zero triple, signed zeros included, gives +0.

Checks (writes nothing but <out_json> and <vectors_json>):
  1. An independent oracle (Fraction midpoints: mid_lo^2 <= S <= mid_hi^2, ties to even at equality; the overflow
     threshold (MAX + ulp(MAX)/2)^2) agrees with every result.
  2. Bit-for-bit agreement with RV118's own `rn64_sqrt` (imported unchanged from its record) on every triple.
  3. The triples: RV118's 20,000 (its generator, seed 20261008); RV115's 9,000 published-component triples and its 3,000
     guard triples (its generator replayed from seed 115003); and curated cases (A-5's midpoint, the midpoint plus a
     tiny third component, signed zeros, subnormal, large and overflow, extreme spreads).
  4. G7's guard: |p - r| <= 64*eps*max(|p|, MIN_POSITIVE) for r = hypot(hypot(x,y),z) in this host's libm and in every
     faithful 1-ulp variant of each call; the largest |p - r| in ulps of p.
  5. A-6: the in-domain shapes listed, subtraction in both orders (19), with revision 01's T-6' rule and v0's.
  6. <vectors_json>: input and output bit patterns for B2-K's K-09 diagnose check and unit tests.

Usage: python b2c_checks_r2.py <r1 b2c_checks_r1.py> <RV118 rv118_a01_option_ii.py> <out_json> <vectors_json>
"""
import importlib.util
import itertools
import json
import math
import random
import struct
import sys
from fractions import Fraction as F
from pathlib import Path

EPS = 2.0 ** -52
MIN_POSITIVE = 2.0 ** -1022
MAX = sys.float_info.max


def load(path, name):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def bits(v):
    return "%016x" % struct.unpack("<Q", struct.pack("<d", v))[0]


# --------------------------------------------------------------------------------------- the reference implementation
def rn64_norm3(x, y, z):
    """(value or None when refused, tie) for RN64 ties-to-even of sqrt(x^2+y^2+z^2), exact; +0 for an all-zero triple."""
    parts = []
    for v in (x, y, z):
        if not math.isfinite(v):
            raise ValueError("finite components only")
        n, d = v.as_integer_ratio()  # v = n / 2^k exactly, d a power of two
        parts.append((abs(n), d.bit_length() - 1))
    k = max(kk for _, kk in parts)
    N = sum((n << (k - kk)) ** 2 for n, kk in parts)  # S = N / 2^(2k), exactly
    if N == 0:
        return 0.0, False
    E = math.isqrt(N).bit_length() - 1 - k  # floor(log2 sqrt(S))
    q = max(E - 52, -1074)  # the binary64 quantum's exponent in that binade
    h = -k - q  # 2*sqrt(S)/2^q = sqrt(4N) * 2^h
    if h >= 0:
        a = (4 * N) << (2 * h)
        r2 = math.isqrt(a)
        sticky = r2 * r2 != a
    else:
        a = 4 * N
        root = math.isqrt(a)
        r2 = root >> -h
        sticky = root * root != a or root & ((1 << -h) - 1) != 0
    m, half = r2 >> 1, r2 & 1  # sqrt(S)/2^q = m + half/2 + (sticky part)
    tie = bool(half) and not sticky
    if half and (sticky or m & 1):
        m += 1
    if m.bit_length() + q > 1024:  # m * 2^q >= 2^1024: beyond binary64's range
        return None, tie
    return math.ldexp(m, q), tie


# --------------------------------------------------------------------------------------- 1. the independent oracle
def oracle_ok(x, y, z, p):
    s = sum((F(v) * F(v) for v in (x, y, z)), F(0))
    over = (F(MAX) + F(math.ulp(MAX)) / 2) ** 2
    if p is None:
        return s >= over
    if p == 0.0:
        return s == 0 and math.copysign(1.0, p) > 0
    if s >= over or not (p > 0):
        return False
    lo = math.nextafter(p, 0.0)
    hi = math.nextafter(p, math.inf)
    mid_lo = (F(lo) + F(p)) / 2
    mid_hi = (F(p) + (F(hi) if math.isfinite(hi) else F(p) + F(math.ulp(p)))) / 2
    if not (mid_lo * mid_lo <= s <= mid_hi * mid_hi):
        return False
    even = int(bits(p), 16) & 1 == 0
    if s == mid_lo * mid_lo or s == mid_hi * mid_hi:
        return even
    return True


# --------------------------------------------------------------------------------------- 3. the triples
def rv118_triples():
    """RV118 ADDENDUM_01 `triples(20261008, 20000)`, verbatim."""
    rnd = random.Random(20261008)

    def draw():
        kind = rnd.random()
        if kind < 0.1:
            return rnd.choice([0.0, -0.0])
        if kind < 0.25:
            return rnd.choice([-1, 1]) * rnd.random() * 2.0 ** rnd.randint(-1074, -1022)
        return rnd.choice([-1, 1]) * rnd.random() * 2.0 ** rnd.randint(-1000, 1000)

    out = []
    for _ in range(20000):
        base = draw()
        mode = rnd.random()
        if mode < 0.3:
            out.append((base, base * (1 + rnd.random() * 1e-3), -base))
        elif mode < 0.5:
            out.append((base, base * 2.0 ** rnd.randint(-80, -20), base * 2.0 ** rnd.randint(-80, -20)))
        else:
            out.append((draw(), draw(), draw()))
    return [t for t in out if all(math.isfinite(v) for v in t)]


def rv115_triples():
    """RV115 ADDENDUM_03's generator replayed (seed 115003): `sample` for 3 x 3,000 trials, then the guard loop."""
    rng = random.Random(115003)

    def sample(kind):
        e = rng.randint(-10, 6)
        if kind == "comparable":
            c = [rng.uniform(-1, 1) for _ in range(3)]
        elif kind == "one_small":
            c = [rng.uniform(-1, 1), rng.uniform(-1, 1), rng.uniform(-1, 1) * 2.0 ** -rng.randint(4, 30)]
        else:
            c = [rng.uniform(-1, 1), rng.uniform(-1, 1), 0.0]
        q = []
        for v in c:
            if v == 0.0:
                q.append(F(0))
            else:
                q.append(F(v * 2.0 ** e) * (1 + F(rng.randint(-2 ** 40, 2 ** 40), 2 ** 95)))
        return tuple(float(v) for v in q)

    gate = [sample(kind) for kind in ("comparable", "one_small", "planar") for _ in range(3000)]
    guard = []
    for _ in range(3000):
        band = rng.choice([(-1074, -1023), (-1060, -1000), (-60, 60), (900, 1000)])
        guard.append(tuple(rng.choice([1, -1]) * rng.random() * 2.0 ** rng.randint(*band) for _ in range(3)))
    return gate, guard


def curated():
    a, b = 2 ** 26 + 1, 2 ** 26
    mx, my = math.ldexp(a * a - b * b, -60), math.ldexp(2 * a * b, -60)  # (2^27+1)2^-60, (2^53+2^27)2^-60
    tiny = math.ldexp(1, -1074)
    return {
        "A-5 midpoint, ties to even": (mx, my, 0.0),
        "A-5 midpoint plus 2^-1074: rounds up": (mx, my, tiny),
        "A-5 midpoint plus 2^-600: rounds up": (mx, my, math.ldexp(1, -600)),
        "A-5 midpoint, first component one ulp lower: rounds down": (math.nextafter(mx, 0.0), my, 0.0),
        "A-5 midpoint, signs flipped": (-mx, -my, -0.0),
        "3,4,12 -> 13 exactly": (-3.0, 4.0, 12.0),
        "1, 2^-1074, 0 -> 1": (1.0, tiny, 0.0),
        "1e300, 1e-300, 2^-1074": (1e300, 1e-300, tiny),
        "smallest subnormal": (tiny, 0.0, 0.0),
        "three smallest subnormals": (tiny, -tiny, tiny),
        "two smallest subnormals": (tiny, tiny, 0.0),
        "largest subnormal, three times": (MIN_POSITIVE - tiny,) * 3,
        "MIN_POSITIVE and a subnormal": (MIN_POSITIVE, tiny, -tiny),
        "MAX, 0, 0 -> MAX": (MAX, 0.0, -0.0),
        "MAX, MAX, 0 -> refused": (MAX, MAX, 0.0),
        "2^1023, 2^1023, 0 -> finite": (math.ldexp(1, 1023), math.ldexp(1, 1023), 0.0),
        "1e308 three times -> finite": (1e308, 1e308, 1e308),
        "1e200 (an unscaled hypot would overflow)": (1e200, -1e200, 1e200),
        "1e-200 (an unscaled hypot would underflow)": (1e-200, 1e-200, -1e-200),
        **{"zeros " + ",".join("-0" if math.copysign(1, v) < 0 else "+0" for v in t): t
           for t in itertools.product([0.0, -0.0], repeat=3)},
    }


# --------------------------------------------------------------------------------------- 4. the guard
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
    return out


def guarded(p, r):
    return abs(p - r) <= 64.0 * EPS * max(abs(p), MIN_POSITIVE)


def study(triples, rv118):
    st = {"triples": len(triples), "refused_overflow": 0, "oracle_agrees": 0, "rv118_rn64_sqrt_agrees": 0,
          "ties": 0, "guard_pairs": 0, "guard_failures": 0, "max_fraction_of_allowance": 0.0,
          "max_|p-r|_ulps_of_p": 0.0, "this_libm_nested_hypot_differs_from_p": 0}
    for x, y, z in triples:
        p, tie = rn64_norm3(x, y, z)
        st["oracle_agrees"] += oracle_ok(x, y, z, p)
        try:
            q, t2 = rv118.rn64_sqrt(rv118.exact_square_sum(x, y, z))
        except OverflowError:  # RV118's math.ldexp raises where the reference refuses
            q, t2 = math.inf, tie
        st["rv118_rn64_sqrt_agrees"] += (p is None and not math.isfinite(q)) or (
            p is not None and bits(p) == bits(q) and tie == t2)
        st["ties"] += tie
        if p is None:
            st["refused_overflow"] += 1
            continue
        h = math.hypot(math.hypot(x, y), z)
        st["this_libm_nested_hypot_differs_from_p"] += bits(h) != bits(p)
        for r in nested_variants(x, y, z):
            st["guard_pairs"] += 1
            st["guard_failures"] += not guarded(p, r)
            frac = abs(p - r) / (64.0 * EPS * max(abs(p), MIN_POSITIVE))
            st["max_fraction_of_allowance"] = max(st["max_fraction_of_allowance"], frac)
            if p > 0:
                st["max_|p-r|_ulps_of_p"] = max(st["max_|p-r|_ulps_of_p"], abs(p - r) / math.ulp(p))
    return st


# --------------------------------------------------------------------------------------- 5. A-6's shapes
def shapes(r1c):
    out = []
    for c in (1, 2):
        cases = "AB"[:c]
        kinds = [("mechanics, distinct cases", {"kind": "mechanics", "rows": True, "records": 0}),
                 ("mechanics, a repeated case (C-1, rowless)", {"kind": "mechanics", "rows": False, "records": 0})]
        kinds += [(f"range({','.join(ops)})", {"kind": "range", "rows": True, "records": len(ops)})
                  for n in range(1, c + 1) for ops in itertools.combinations(cases, n)]
        if c == 2:
            kinds += [("subtraction A-B", {"kind": "subtraction", "rows": True, "records": 2}),
                      ("subtraction B-A", {"kind": "subtraction", "rows": True, "records": 2})]
        for z in range(1, 4 - c):
            for seq in itertools.product(kinds, repeat=z):
                combos = [k for _, k in seq]
                rows = r1c.layout(c, combos)
                out.append({"c": c, "authored": [n for n, _ in seq], "r1_rule": r1c.t6_r1(rows, combos),
                            "v0_rule": r1c.t6_v0(rows, combos)})
    return out


def main():
    r1_checks, rv118_path, out_json, vectors_json = (Path(a) for a in sys.argv[1:5])
    r1c = load(r1_checks, "b2c_checks_r1")
    rv118 = load(rv118_path, "rv118_a01_option_ii")
    gate, guard = rv115_triples()
    cur = curated()
    report = {"curated": {}}
    for label, (x, y, z) in cur.items():
        p, tie = rn64_norm3(x, y, z)
        report["curated"][label] = {
            "x": bits(x), "y": bits(y), "z": bits(z), "p": "refused" if p is None else bits(p),
            "p_hex": "refused" if p is None else p.hex(), "tie": tie, "oracle_agrees": oracle_ok(x, y, z, p),
            "this_libm_nested_hypot": math.hypot(math.hypot(x, y), z).hex()}
    report["studies"] = {
        "RV118 triples (seed 20261008)": study(rv118_triples(), rv118),
        "RV115 gate triples (seed 115003, published components)": study(gate, rv118),
        "RV115 guard triples (seed 115003, finite)": study([t for t in guard if all(map(math.isfinite, t))], rv118),
        "curated": study(list(cur.values()), rv118),
    }
    report["rv115_worst_guard_triple_regenerated"] = (
        ("0x0.0000000000002p-1022", "-0x0.0000000000004p-1022", "-0x0.0000000000002p-1022")
        in {tuple(v.hex() for v in t) for t in guard})
    ex = rv118.tie_example()
    mine = report["curated"]["A-5 midpoint, ties to even"]
    report["a5_matches_rv118_tie_example"] = (ex["rn64_ties_to_even"] == mine["p_hex"] and ex["tie_detected"] and
                                              mine["tie"])
    sh = shapes(r1c)
    report["a6_shapes"] = {"count": len(sh), "r1_rule_holds_for_all": all(s["r1_rule"] for s in sh),
                           "v0_rule_fails": [f"c={s['c']}: " + "; ".join(s["authored"]) for s in sh if not s["v0_rule"]],
                           "list": [f"c={s['c']}: " + "; ".join(s["authored"]) for s in sh]}
    report["all_agree"] = all(v["oracle_agrees"] == v["triples"] and v["rv118_rn64_sqrt_agrees"] == v["triples"]
                              and v["guard_failures"] == 0 for v in report["studies"].values()) and all(
        c["oracle_agrees"] for c in report["curated"].values())
    Path(out_json).write_text(json.dumps(report, indent=1, sort_keys=True, ensure_ascii=True) + "\n")

    # 6. Vectors for B2-K: every curated case, then the first 256 RV118 and 128 RV115 gate triples.
    vec = [(label, t) for label, t in cur.items()]
    vec += [("RV118 triple %d" % i, t) for i, t in enumerate(rv118_triples()[:256])]
    vec += [("RV115 gate triple %d" % i, t) for i, t in enumerate(gate[:128])]
    rows = []
    for label, (x, y, z) in vec:
        p, tie = rn64_norm3(x, y, z)
        rows.append({"label": label, "x": bits(x), "y": bits(y), "z": bits(z),
                     "p": "refused" if p is None else bits(p), "tie": tie})
    Path(vectors_json).write_text(json.dumps({
        "recipe": "DEF-C r2 rows.displacement_magnitude: RN64, ties to even, of sqrt(x^2+y^2+z^2), exact; refused "
                  "beyond binary64's range; +0 for an all-zero triple",
        "encoding": "IEEE 754 binary64 bit patterns, 16 lowercase hex digits", "vectors": rows},
        indent=1, ensure_ascii=True) + "\n")
    print(json.dumps({"all_agree": report["all_agree"],
                      "studies": report["studies"], "a6": {k: v for k, v in report["a6_shapes"].items() if k != "list"},
                      "rv115_worst_guard_triple_regenerated": report["rv115_worst_guard_triple_regenerated"],
                      "a5_matches_rv118": report["a5_matches_rv118_tie_example"], "vectors": len(rows)},
                     indent=1, sort_keys=True))


if __name__ == "__main__":
    main()
