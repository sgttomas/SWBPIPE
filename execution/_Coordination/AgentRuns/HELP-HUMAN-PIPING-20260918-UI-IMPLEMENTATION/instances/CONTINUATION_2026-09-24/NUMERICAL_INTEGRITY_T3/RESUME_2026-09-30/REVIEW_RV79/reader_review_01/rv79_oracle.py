"""RV79 independent exact-rational oracle for the Python retained-precision reader helpers.

The oracle uses only integers and fractions.Fraction (no reader function) to compute the
least binary64 upper bound RU64(q), correctly rounded nearest fl(q) and sqrt, and compares
them bit for bit with the reader's helpers on edge and random inputs. Run from P with the
reader importable: VENV/bin/python rv79_oracle.py
"""
import math, random, struct, sys
from fractions import Fraction as F

sys.path.insert(0, ".")
from core.analysis_runs import retained_precision as rp

MAXF = F((2**53 - 1) * 2**971)
TINY = F(1, 2**1074)

def exact_of_bits(w):
    e, m = (w >> 52) & 2047, w & ((1 << 52) - 1)
    assert e != 2047
    q = F(m | (1 << 52), 1) * F(2) ** (e - 1075) if e else F(m) * TINY
    return -q if w >> 63 else q

def word(x):
    return struct.unpack(">Q", struct.pack(">d", x))[0]

def exact(x):
    return exact_of_bits(word(x))

def from_dyadic(m, q):
    """binary64 of exactly m*2^q (assumed representable), built from integer fields."""
    if m == 0:
        return 0.0
    top = m.bit_length() - 1 + q
    if top < -1022:
        w = m << (q + 1074)
    else:
        w = ((top + 1023) << 52) | ((m << (52 - (m.bit_length() - 1))) - (1 << 52))
    return struct.unpack(">d", w.to_bytes(8, "big"))[0]

def ru64(q):
    """Least binary64 >= q >= 0, or None on overflow. Pure integer arithmetic."""
    assert q >= 0
    if q == 0:
        return 0.0
    # exponent e with 2^e <= q < 2^(e+1)
    e = q.numerator.bit_length() - q.denominator.bit_length()
    if F(2) ** e > q: e -= 1
    if F(2) ** (e + 1) <= q: e += 1
    quantum = max(e - 52, -1074)
    scaled = q / F(2) ** quantum
    m = -((-scaled.numerator) // scaled.denominator)
    if m == 1 << 53:
        m, quantum = 1 << 52, quantum + 1
    if F(m) * F(2) ** quantum > MAXF:
        return None
    return from_dyadic(m, quantum)

def fl(q):
    """Correctly rounded nearest-even binary64 (Fraction.__float__ is correctly rounded)."""
    return float(q)

def check_sqrt(x, r):
    """r is the correctly rounded sqrt of x (x >= 0) iff x lies within r's rounding interval."""
    if x == 0: return r == 0
    lo = (exact(math.nextafter(r, 0)) + exact(r)) / 2
    hi = (exact(r) + exact(math.nextafter(r, math.inf))) / 2
    X = exact(x)
    ok_lo = lo * lo <= X if (word(r) & 1) == 0 else lo * lo < X
    ok_hi = X <= hi * hi if (word(r) & 1) == 0 else X < hi * hi
    return ok_lo and ok_hi

def rand_float(rng):
    k = rng.random()
    if k < 0.15: return struct.unpack(">d", rng.getrandbits(52).to_bytes(8, "big"))[0]  # subnormal
    if k < 0.25: return float(rng.choice([0.0, 5e-324, 2.0 ** -1022, 2.0 ** -988, 2.0 ** -1022 - 5e-324, 1.0, 2.0 ** 1023, 1.7976931348623157e308]))
    e = rng.randint(-1074, 1023) if k < 0.6 else rng.randint(-60, 60)
    return abs(math.ldexp(rng.random() + 0.5, e)) if math.ldexp(1, min(e, 1023)) != math.inf else 1.0

def main(n=20000, seed=79):
    rng = random.Random(seed)
    stats = {}
    def rec(name, ok, detail=None):
        s = stats.setdefault(name, [0, 0, []])
        s[0] += 1
        if not ok:
            s[1] += 1
            if len(s[2]) < 5: s[2].append(detail)
    for _ in range(n):
        a, b = rand_float(rng), rand_float(rng)
        if not (math.isfinite(a) and math.isfinite(b)): continue
        # upward_product
        want = ru64(exact(a) * exact(b))
        try:
            got = rp.upward_product(a, b)
        except ValueError:
            got = None
        rec("upward_product", (want is None and got is None) or (want is not None and got is not None and word(got) == word(want)), (a.hex(), b.hex()))
        # upward_small_sum (operands as used by absolute_bound: both small)
        b0, r = math.ldexp(a, -60) if a < 1e300 else a, math.ldexp(b, -60)
        want = ru64(exact(b0) + exact(r) + TINY)
        try: got = rp.upward_small_sum(b0, r)
        except ValueError: got = None
        rec("upward_small_sum", (want is None and got is None) or (got is not None and want is not None and word(got) == word(want)), (b0.hex(), r.hex()))
        # _scaled_component
        for p in (53, 64):
            want = ru64(exact(a) / F(2) ** p)
            rec("_scaled_component", word(rp._scaled_component(a, p)) == word(want), (a.hex(), p))
        # absolute_bound per C1:158
        for nval, S in ((a, b), (a, math.ldexp(b, -1000)), (-a, b)):
            if S == 0:
                want = 0.0
            else:
                b0w = ru64(exact(S) / F(2) ** 64)
                want = ru64(exact(b0w) + exact(ru64(abs(exact(nval)) / F(2) ** 53)) + TINY) if S < 2.0 ** -988 else b0w
            rec("absolute_bound", word(rp.absolute_bound(nval, S)) == word(want), (nval.hex(), S.hex()))
        # phi_512 = RU64(2^-438 * e_hat)
        want = ru64(exact(a) / F(2) ** 438)
        rec("_phi_512", word(rp._phi_512(a)) == word(want), a.hex())
        # e_hat (nearest, verify.rs:323-334) with L>0 and L==0
        L = abs(math.ldexp(rng.random() + 0.5, rng.randint(-20, 20)))
        fo, mo = a, b
        q1, q2 = exact(mo) / exact(L), exact(L) * exact(fo)
        if q1 <= MAXF and q2 <= MAXF:
            want = [max(fo, fl(q1)), max(mo, fl(q2))]
            rec("_e_hat", [word(x) for x in rp._e_hat([fo, mo], L)] == [word(x) for x in want], (fo.hex(), mo.hex(), L.hex()))
        rec("_e_hat_L0", [word(x) for x in rp._e_hat([fo, mo], 0.0)] == [word(fo), word(mo)], (fo.hex(), mo.hex()))
        # _coupled (adaptive.rs:341-353)
        s4 = [abs(math.ldexp(rng.random() + 0.5, rng.randint(-40, 40))) for _ in range(4)]
        tr, ro, fo4, mo4 = s4
        want = [max(tr, fl(exact(L) * exact(ro))), max(ro, fl(exact(tr) / exact(L))), max(fo4, fl(exact(mo4) / exact(L))), max(mo4, fl(exact(L) * exact(fo4)))]
        rec("_coupled", [word(x) for x in rp._coupled(s4, L)] == [word(x) for x in want], s4)
        # _extent (adaptive.rs:321-339): fl(sqrt(fl(fl(fl(dx^2)+fl(dy^2))+fl(dz^2))))
        pts = [[math.ldexp(rng.random() - 0.5, rng.randint(-5, 8)) for _ in range(3)] for _ in range(rng.randint(1, 4))]
        d = [fl(exact(max(p[j] for p in pts)) - exact(min(p[j] for p in pts))) for j in range(3)]
        sq = [fl(exact(x) * exact(x)) for x in d]
        inner = fl(exact(fl(exact(sq[0]) + exact(sq[1]))) + exact(sq[2]))
        got = rp._extent(pts)
        rec("_extent", check_sqrt(inner, got), pts)
    # targeted edges
    edges = [
        ("ceil_carry", rp.upward_product(math.nextafter(1.0, 2), math.nextafter(1.0, 2)), ru64(exact(math.nextafter(1.0, 2)) ** 2)),
        ("subnormal_product", rp.upward_product(5e-324, 0.5), ru64(F(1, 2**1075))),
        ("normal_boundary", rp.upward_product(2.0 ** -1022, math.nextafter(1.0, 0)), ru64(exact(2.0 ** -1022) * exact(math.nextafter(1.0, 0)))),
        ("phi_zero", rp._phi_512(0.0), 0.0),
        ("phi_min", rp._phi_512(5e-324), ru64(TINY / F(2) ** 438)),
        ("phi_max", rp._phi_512(1.7976931348623157e308), ru64(MAXF / F(2) ** 438)),
        ("abs_bound_S_min", rp.absolute_bound(1.0, 5e-324), ru64(exact(ru64(TINY / F(2) ** 64)) + exact(ru64(F(1) / F(2) ** 53)) + TINY)),
        ("abs_bound_S_2^-988", rp.absolute_bound(1e300, 2.0 ** -988), ru64(exact(2.0 ** -988) / F(2) ** 64)),
        ("abs_bound_S_below_2^-988", rp.absolute_bound(1e300, math.nextafter(2.0 ** -988, 0)), ru64(exact(ru64(exact(math.nextafter(2.0 ** -988, 0)) / F(2) ** 64)) + exact(ru64(exact(1e300) / F(2) ** 53)) + TINY)),
    ]
    for name, got, want in edges:
        rec("edge:" + name, word(got) == word(want), (got, want))
    try:
        rp.upward_product(1.7976931348623157e308, 1.0000000000000002)
        rec("edge:overflow_refused", False, "no error")
    except ValueError:
        rec("edge:overflow_refused", True)
    rec("edge:max_times_one", word(rp.upward_product(1.7976931348623157e308, 1.0)) == word(1.7976931348623157e308))
    for k, (total, bad, ex) in sorted(stats.items()):
        print(f"{k}: {total} checked, {bad} mismatches" + (f" e.g. {ex}" if bad else ""))
    return sum(v[1] for v in stats.values())

if __name__ == "__main__":
    sys.exit(1 if main() else 0)
