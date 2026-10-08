"""I96 B3-D: two read-only numeric checks (standard library only).

1. G_hat: the exact route's derived shear modulus, as PP computes it
   (pressure_exact.rs `IsotropicENu::new`, `Scaled` arithmetic), against the
   plain binary64 expression e / (2.0 * (1.0 + nu)) that a reader can recompute.
2. Section bits: the exact route's ordinary section (pressure_exact
   `SourceAnnulus::from_od_wall`, `Scaled` arithmetic with binary64 pi) against
   the correctly rounded annulus that DEF-O's RP-PREPARED-ANNULUS-v1 produces,
   on the committed exact-route fixtures. The SourceAnnulus emulation is first
   checked bit for bit against the fixtures' published `pipe_sections`.

Usage: python b3d_numeric_checks.py <P> <out.json>
<P> is the piping project root (placeholder P in the records).
"""
import json
import math
import random
import struct
import sys
from fractions import Fraction as F
from pathlib import Path


def bits(x):
    return struct.unpack(">Q", struct.pack(">d", x))[0]


def from_bits(b):
    return struct.unpack(">d", struct.pack(">Q", b))[0]


def power_of_two(e):
    assert -1074 <= e <= 1023
    if e >= -1022:
        return from_bits((e + 1023) << 52)
    return from_bits(1 << (e + 1074))


class Scaled:
    """Line-for-line transcription of pressure_exact.rs `Scaled`."""

    def __init__(self, m, e):
        self.m, self.e = m, e

    @staticmethod
    def from_f64(v):
        assert math.isfinite(v)
        if v == 0.0:
            return Scaled(v, 0)
        b = bits(v)
        sign = b & (1 << 63)
        biased = (b >> 52) & 0x7FF
        fraction = b & ((1 << 52) - 1)
        if biased != 0:
            return Scaled(from_bits(sign | (1022 << 52) | fraction), biased - 1022)
        highest = fraction.bit_length() - 1
        magnitude = fraction / power_of_two(highest + 1) if highest + 1 <= 1023 else None
        return Scaled(math.copysign(magnitude, v), highest + 1 - 1074)

    @staticmethod
    def from_parts(m, e):
        n = Scaled.from_f64(m)
        if n.m == 0.0:
            return n
        return Scaled(n.m, n.e + e)

    def neg(self):
        return Scaled(-self.m, self.e)

    def add(self, o):
        if self.m == 0.0:
            return o
        if o.m == 0.0:
            return self
        e = max(self.e, o.e)
        return Scaled.from_parts(align(self.m, self.e - e) + align(o.m, o.e - e), e)

    def sub(self, o):
        return self.add(o.neg())

    def mul(self, o):
        if self.m == 0.0 or o.m == 0.0:
            return Scaled.from_f64(self.m * o.m)
        return Scaled.from_parts(self.m * o.m, self.e + o.e)

    def div(self, o):
        if o.m == 0.0:
            return None
        if self.m == 0.0:
            return Scaled.from_f64(self.m / o.m)
        return Scaled.from_parts(self.m / o.m, self.e - o.e)

    def to_f64(self):
        if self.m == 0.0:
            return self.m
        if self.e > 1024:
            return math.copysign(math.inf, self.m)
        if self.e == 1024:
            return (self.m * power_of_two(1023)) * 2.0
        if self.e < -1074:
            return math.copysign(0.0, self.m)
        return self.m * power_of_two(self.e)


def align(m, shift):
    assert shift <= 0
    if shift < -1074:
        return math.copysign(0.0, m)
    return m * power_of_two(shift)


def g_hat_pp(e, nu):
    den = Scaled.from_f64(2.0).mul(Scaled.from_f64(1.0).add(Scaled.from_f64(nu)))
    return Scaled.from_f64(e).div(den).to_f64()


def g_reader(e, nu):
    return e / (2.0 * (1.0 + nu))


def check_g_hat(samples, seed):
    rng = random.Random(seed)
    cases = [(210e9, 0.3), (200e9, 0.25), (2000.0, 0.25), (1.0, 0.0),
             (210e9, -0.9999999999999999), (210e9, 0.49999999999999994),
             (210e9, 5e-324), (210e9, -5e-324), (210e9, 1e-300), (1e-300, 0.3),
             (1.7976931348623157e308, 0.4999999999999999)]
    for _ in range(samples):
        e = from_bits(rng.randrange(bits(1e-200), bits(1e300)))
        u = rng.random()
        nu = -1.0 + 1.5 * u
        if not (-1.0 < nu < 0.5):
            continue
        cases.append((e, nu))
    for _ in range(samples // 4):
        # nu as an arbitrary binary64 in (-1, 0.5), drawn by bits on each side of zero
        if rng.random() < 0.5:
            nu = from_bits(rng.randrange(1, bits(0.5)))
        else:
            nu = -from_bits(rng.randrange(1, bits(1.0)))
        e = from_bits(rng.randrange(bits(1.0), bits(1e15)))
        cases.append((e, nu))
    equal = differ = skipped = 0
    first = []
    for e, nu in cases:
        g = g_hat_pp(e, nu)
        if not (math.isfinite(g) and g >= 2.2250738585072014e-308):
            skipped += 1  # PP refuses non-finite or nonpositive G; subnormal G is outside the claim
            continue
        r = g_reader(e, nu)
        if bits(g) == bits(r):
            equal += 1
        else:
            differ += 1
            if len(first) < 5:
                first.append({"E_bits": f"{bits(e):016x}", "nu_bits": f"{bits(nu):016x}",
                              "pp": f"{bits(g):016x}", "reader": f"{bits(r):016x}"})
    return {"cases": len(cases), "normal_G_equal": equal, "normal_G_differ": differ,
            "skipped_nonnormal_G": skipped, "first_differences": first}


def pi_enclosure(terms=400):
    # Machin: pi = 16 atan(1/5) - 4 atan(1/239), alternating series with explicit tails.
    def atan_inv(x, n):
        s = F(0)
        for k in range(n):
            s += F((-1) ** k, (2 * k + 1) * x ** (2 * k + 1))
        tail = F(1, (2 * n + 1) * x ** (2 * n + 1))
        return s, tail
    a, ta = atan_inv(5, terms)
    b, tb = atan_inv(239, terms)
    mid = 16 * a - 4 * b
    err = 16 * ta + 4 * tb
    return mid - err, mid + err


def rn(q):
    return q.numerator / q.denominator  # Python int true division is correctly rounded


def prepared(d, t, pi):
    D, T = F(d), F(t)
    c = D / 2
    ri = c - T
    P = T * (D - T)
    Q = c * c + ri * ri
    G = P * Q
    out = {}
    for name, f in (("A", lambda p: p * P), ("I", lambda p: p * G / 4),
                    ("J", lambda p: p * G / 2), ("Z", lambda p: p * G / 4 / c)):
        lo, hi = rn(f(pi[0])), rn(f(pi[1]))
        out[name] = f"{bits(lo):016x}" if lo == hi else None
    out["c"] = f"{bits(rn(c)):016x}"
    return out


def source_annulus(od, wall):
    t = Scaled.from_f64(wall)
    a_s = Scaled.from_f64(math.pi).mul(t).mul(Scaled.from_f64(od).sub(t))
    ro = Scaled.from_f64(od).mul(Scaled.from_f64(0.5))
    ri = ro.sub(Scaled.from_f64(wall))
    i = a_s.mul(ro.mul(ro).add(ri.mul(ri))).mul(Scaled.from_f64(0.25))
    j = i.mul(Scaled.from_f64(2.0))
    z = i.div(ro)
    return {"A": f"{bits(a_s.to_f64()):016x}", "I": f"{bits(i.to_f64()):016x}",
            "J": f"{bits(j.to_f64()):016x}", "Z": f"{bits(z.to_f64()):016x}"}


def main():
    p = Path(sys.argv[1])
    out = {"g_hat": check_g_hat(200000, 96)}
    pi = pi_enclosure()
    assert pi[0] < F(math.pi) * 2 and pi[1] - pi[0] < F(1, 2 ** 1100)
    sections = []
    seen = set()
    for raw in sorted((p / "fixtures/product_preview/physics_source").glob("*.raw.json")):
        doc = json.loads(raw.read_text())
        for case in doc["contract_evidence"]["exact_cases"]:
            for s in case["pipe_sections"]:
                od, wall = s["outside_diameter_m"], s["effective_wall_thickness_m"]
                published = {k: f"{bits(s[v]):016x}" for k, v in
                             (("A", "As_m2"), ("I", "I_m4"), ("J", "J_m4"), ("Z", "Z_m3"))}
                emulated = source_annulus(od, wall)
                key = (raw.name, case["load_case_id"], s["pipe_id"])
                seen.add((od, wall))
                sections.append({"fixture": raw.name, "case": case["load_case_id"], "pipe": s["pipe_id"],
                                 "od": od, "wall": wall, "published": published,
                                 "emulation_matches_published": emulated == published})
    comparisons = []
    for od, wall in sorted(seen):
        src = source_annulus(od, wall)
        prep = prepared(od, wall, pi)
        comparisons.append({"od": od, "wall": wall, "source_annulus": src, "prepared": prep,
                            "differing_properties": [k for k in "AIJZ" if src[k] != prep[k]]})
    # A few further normalized sections, to show the difference is not a fixture accident.
    rng = random.Random(7)
    extra = {"sections": 0, "any_property_differs": 0, "per_property_differs": {k: 0 for k in "AIJZ"},
             "ambiguous_rounding": 0}
    for _ in range(2000):
        od = rng.uniform(0.02, 1.5)
        wall = od * rng.uniform(0.01, 0.2)
        src = source_annulus(od, wall)
        prep = prepared(od, wall, pi)
        extra["sections"] += 1
        if any(prep[k] is None for k in "AIJZ"):
            extra["ambiguous_rounding"] += 1
            continue
        d = [k for k in "AIJZ" if src[k] != prep[k]]
        extra["any_property_differs"] += bool(d)
        for k in d:
            extra["per_property_differs"][k] += 1
    out["sections"] = {"published_checked": len(sections),
                       "emulation_matches_all_published": all(s["emulation_matches_published"] for s in sections),
                       "distinct_inputs": comparisons, "random_sections": extra}
    Path(sys.argv[2]).write_text(json.dumps(out, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"g_hat": {k: v for k, v in out["g_hat"].items() if k != "first_differences"},
                      "emulation_ok": out["sections"]["emulation_matches_all_published"],
                      "fixture_comparison": comparisons, "random": extra}, indent=1))


if __name__ == "__main__":
    main()
