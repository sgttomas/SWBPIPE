"""RV116 (RV-D) independent read-only checks for I96's B3-D (standard library only).

Written fresh by RV116; it imports nothing from I96's scripts.

1. H: an independent JCS-style canonicalizer (keys sorted by UTF-16 code units,
   JCS string escaping, integers only) for H(domain, payload) =
   sha256(canonical({"domain":d,"payload":p})). Controls: DEF-O's pinned
   a7ed7ca0...0349 from the committed DEF-O; then DEF-E (the regenerated draft).
2. Sections: the exact route's SourceAnnulus in plain binary64 (normal range,
   where PP's Scaled steps equal binary64 steps) against the committed
   physics-source raw outputs; the correctly rounded annulus with my own
   pi enclosure (Machin with integer fixed point and explicit error bounds,
   width < 2^-1300); preview's derive_pipe_section (powi as repeated squaring);
   and a random-section sweep with a different generator and seed.
3. G_hat: an exact-rational model of PP's Scaled path for E/(2(1+nu))
   (RN64 at 1+nu, exact doubling, quotient rounded to 53 bits with unbounded
   exponent, then a final binary64 conversion), compared with the binary64
   expression e/(2.0*(1.0+nu)); plus targeted edges (subnormal E, nu near -1,
   nu at 1/2 - ulp, tiny nu, E at max).

Usage: python rv116_checks.py <P> <draft_definition.json> <out.json>
"""
import hashlib
import json
import math
import random
import struct
import sys
from fractions import Fraction as F
from pathlib import Path


def bits(x):
    return struct.unpack(">Q", struct.pack(">d", x))[0]


def hx(x):
    return format(bits(x), "016x")


# ---------- 1. canonical JSON and H ----------
def jcs_string(s):
    out = ['"']
    for ch in s:
        o = ord(ch)
        if ch == '"':
            out.append('\\"')
        elif ch == "\\":
            out.append("\\\\")
        elif ch == "\b":
            out.append("\\b")
        elif ch == "\f":
            out.append("\\f")
        elif ch == "\n":
            out.append("\\n")
        elif ch == "\r":
            out.append("\\r")
        elif ch == "\t":
            out.append("\\t")
        elif o < 0x20:
            out.append("\\u%04x" % o)
        else:
            out.append(ch)
    out.append('"')
    return "".join(out)


def utf16_key(s):
    return s.encode("utf-16-be")


def jcs(v):
    if v is None:
        return "null"
    if v is True:
        return "true"
    if v is False:
        return "false"
    if isinstance(v, int):
        assert abs(v) < 2**53
        return str(v)
    if isinstance(v, float):
        raise ValueError("float in definition")
    if isinstance(v, str):
        return jcs_string(v)
    if isinstance(v, list):
        return "[" + ",".join(jcs(x) for x in v) + "]"
    if isinstance(v, dict):
        keys = sorted(v, key=utf16_key)
        return "{" + ",".join(jcs_string(k) + ":" + jcs(v[k]) for k in keys) + "}"
    raise TypeError(type(v))


def H(domain, payload):
    return hashlib.sha256(jcs({"domain": domain, "payload": payload}).encode("utf-8")).hexdigest()


# ---------- 2. pi, annulus, SourceAnnulus, derive_pipe_section ----------
def atan_inv_fixed(x, prec):
    """atan(1/x) * 2^prec as integer bounds (lo, hi), alternating series with truncation."""
    one = 1 << prec
    total = 0
    term = one // x  # floor(2^prec / x)
    n = 1
    k = 0
    x2 = x * x
    count = 0
    while term:
        t = term // n
        total += -t if k % 2 else t
        term //= x2
        n += 2
        k += 1
        count += 1
    # each floor division loses < 1 per term in two places; bound generously
    err = 4 * (count + 2)
    return total - err, total + err


def pi_bounds(prec=1400):
    a_lo, a_hi = atan_inv_fixed(5, prec)
    b_lo, b_hi = atan_inv_fixed(239, prec)
    lo = 16 * a_lo - 4 * b_hi
    hi = 16 * a_hi - 4 * b_lo
    return F(lo, 1 << prec), F(hi, 1 << prec)


PI_LO, PI_HI = pi_bounds()
assert PI_LO < F(math.pi) * (1 + F(1, 2**50)) and PI_HI > F(math.pi) * (1 - F(1, 2**50))
assert PI_HI - PI_LO < F(1, 2**1300)
assert F(314159265358979323846264338327950, 10**32) < PI_LO < PI_HI < F(314159265358979323846264338327951, 10**32)


def rn(q):
    """Correctly rounded binary64 nearest-even of a positive rational (CPython int/int true division)."""
    return q.numerator / q.denominator


def prepared(D, t):
    d, w = F(D), F(t)
    c = d / 2
    ri = c - w
    p = w * (d - w)
    q = c * c + ri * ri
    g = p * q
    out = {}
    amb = []
    for name, lo, hi in [
        ("A", PI_LO * p, PI_HI * p),
        ("I", PI_LO * g / 4, PI_HI * g / 4),
        ("J", PI_LO * g / 2, PI_HI * g / 2),
        ("Z", PI_LO * g / 4 / c, PI_HI * g / 4 / c),
    ]:
        a, b = rn(lo), rn(hi)
        if a != b:
            amb.append(name)
        out[name] = a
    out["c"] = rn(c)
    return out, amb


def source_annulus(D, t):
    pi = math.pi
    A = (pi * t) * (D - t)
    ro = D * 0.5
    ri = ro - t
    I = (A * (ro * ro + ri * ri)) * 0.25
    J = I * 2.0
    Z = I / ro
    return {"A": A, "I": I, "J": J, "Z": Z}


def derive_pipe_section(D, t):
    pi = math.pi
    idd = D - 2.0 * t
    sq = lambda x: x * x
    A = pi * (sq(D) - sq(idd)) / 4.0
    I = pi * (sq(sq(D)) - sq(sq(idd))) / 64.0
    J = 2.0 * I
    Z = I / (D / 2.0)
    return {"A": A, "I": I, "J": J, "Z": Z}


# ---------- 3. G_hat ----------
def rn_rel53(q):
    """Round a nonzero positive rational to 53 significant bits, nearest-even, unbounded exponent."""
    assert q > 0
    e = q.numerator.bit_length() - q.denominator.bit_length()
    if q < F(2) ** e:
        e -= 1
    # q in [2^e, 2^(e+1)); quantum 2^(e-52)
    scaled = q / F(2) ** (e - 52)
    m = scaled.numerator // scaled.denominator
    rem = scaled - m
    if rem > F(1, 2) or (rem == F(1, 2) and m % 2 == 1):
        m += 1
    return F(m) * F(2) ** (e - 52)


def g_scaled_model(E, nu):
    """PP IsotropicENu::new: d = 2*RN64(1+nu) (exact doubling); G = to_f64(RN53(E/d))."""
    one_plus = 1.0 + nu  # binary64 add (Scaled add equals it: result near 1, normal)
    d = F(2) * F(one_plus)
    q = rn_rel53(F(E) / d)
    # to_f64: exact if normal; report None (refused/non-normal) otherwise
    if q >= F(2) ** 1024:
        return None
    if q < F(2) ** -1022:
        return None
    return float(q)


def main():
    P = Path(sys.argv[1])
    draft = Path(sys.argv[2])
    out_path = Path(sys.argv[3])
    res = P / "fixtures/results"
    report = {}

    defo = json.loads((res / "retained_precision_prepared_ordinary_v1.json").read_bytes())
    defe_raw = draft.read_bytes()
    defe = json.loads(defe_raw)
    report["H"] = {
        "DEF_O_H": H("retained_precision_formation_v1", defo),
        "DEF_O_pinned": "a7ed7ca0bf0bba6e8b821ca4befa00a0fa9541a83694be8b28ac63e39b1d0349",
        "DEF_E_H": H("retained_precision_formation_v1", defe),
        "DEF_E_claimed": "9b66492e56a25a10939973a9ceb060a538769b0cd8f0da52962d5ec90f68693d",
        "DEF_E_raw_sha256": hashlib.sha256(defe_raw).hexdigest(),
        "DEF_E_raw_is_canonical": jcs(defe).encode("utf-8") == defe_raw,
        "DEF_E_preparation_equals_DEF_O": defe["preparation"] == defo["preparation"],
        "DEF_E_top_keys_minus_DEF_O": sorted(set(defe) - set(defo)),
        "DEF_O_top_keys_minus_DEF_E": sorted(set(defo) - set(defe)),
    }
    report["H"]["DEF_O_matches_pin"] = report["H"]["DEF_O_H"] == report["H"]["DEF_O_pinned"]
    report["H"]["DEF_E_matches_claim"] = report["H"]["DEF_E_H"] == report["H"]["DEF_E_claimed"]

    # Sections against the committed exact-route outputs
    pub = []
    for f in sorted((P / "fixtures/product_preview/physics_source").glob("*.raw.json")):
        d = json.loads(f.read_bytes())
        for case in (d.get("contract_evidence") or {}).get("exact_cases", []):
            for s in case["pipe_sections"]:
                pub.append((f.name, s))
    emul_ok = 0
    distinct = {}
    for name, s in pub:
        D, t = s["outside_diameter_m"], s["effective_wall_thickness_m"]
        sa = source_annulus(D, t)
        same = all(bits(sa[k]) == bits(s[key]) for k, key in [("A", "As_m2"), ("I", "I_m4"), ("J", "J_m4"), ("Z", "Z_m3")])
        emul_ok += same
        distinct[(D, t)] = s
    rows = []
    for (D, t) in distinct:
        sa = source_annulus(D, t)
        pr, amb = prepared(D, t)
        dp = derive_pipe_section(D, t)
        rows.append({
            "D": D, "t": t,
            "source_annulus": {k: hx(v) for k, v in sa.items()},
            "prepared": {k: hx(v) for k, v in pr.items()},
            "derive_pipe_section": {k: hx(v) for k, v in dp.items()},
            "ulp_prepared_minus_source": {k: bits(pr[k]) - bits(sa[k]) for k in "AIJZ"},
            "ambiguous": amb,
        })
    report["sections"] = {"published": len(pub), "emulation_matches": emul_ok, "distinct": rows}

    rng = random.Random(116)
    n = 3000
    differ_any = 0
    per = {k: 0 for k in "AIJZ"}
    ambiguous = 0
    maxulp = {k: 0 for k in "AIJZ"}
    for _ in range(n):
        D = math.exp(rng.uniform(math.log(0.01), math.log(3.0)))
        t = D * rng.uniform(0.005, 0.25)
        sa = source_annulus(D, t)
        pr, amb = prepared(D, t)
        if amb:
            ambiguous += 1
            continue
        hit = False
        for k in "AIJZ":
            du = abs(bits(pr[k]) - bits(sa[k]))
            maxulp[k] = max(maxulp[k], du)
            if du:
                per[k] += 1
                hit = True
        differ_any += hit
    report["random_sections"] = {"n": n, "seed": 116, "D_range_m": [0.01, 3.0], "t_over_D": [0.005, 0.25],
                                 "ambiguous": ambiguous, "any_differs": differ_any, "per_property": per,
                                 "max_ulp_distance": maxulp}

    # G_hat
    rng = random.Random(1160)
    cases = []
    for _ in range(200000):
        E = 10 ** rng.uniform(-300, 307)
        nu = rng.uniform(-0.999999, 0.4999999)
        cases.append((E, nu))
    for _ in range(50000):
        E = 10 ** rng.uniform(5, 12)
        nu = struct.unpack(">d", struct.pack(">Q", rng.getrandbits(62) & 0x3FDFFFFFFFFFFFFF))[0] * rng.choice([1, -1])
        cases.append((E, nu))
    m1 = math.nextafter(-1.0, 0.0)
    half_m = math.nextafter(0.5, 0.0)
    edges = [(2e11, 0.3), (2e11, 0.25), (210e9, 0.3), (1.0, m1), (1e-300, m1), (5e-324, m1), (1e300, half_m),
             (1.7976931348623157e308, half_m), (1.7976931348623157e308, 0.0), (2e11, 5e-324), (2e11, -5e-324),
             (2e11, 2.0**-53), (2e11, -2.0**-54), (2e11, -2.0**-53), (2.2250738585072014e-308, 0.0),
             (4.9e-324 * 2**60, 0.3), (1.0, 0.3125)]
    cases.extend(edges)
    eq = diff = refused_both = refused_model_only = refused_expr_only = 0
    firsts = []
    for E, nu in cases:
        model = g_scaled_model(E, nu)
        with_expr = E / (2.0 * (1.0 + nu)) if math.isfinite(E / (2.0 * (1.0 + nu))) else None
        expr_normal = with_expr if (with_expr is not None and with_expr >= 2.2250738585072014e-308) else None
        if model is None and expr_normal is None:
            refused_both += 1
        elif model is None:
            refused_model_only += 1
        elif expr_normal is None:
            refused_expr_only += 1
        elif bits(model) == bits(expr_normal):
            eq += 1
        else:
            diff += 1
            if len(firsts) < 5:
                firsts.append([E, nu, hx(model), hx(expr_normal)])
    report["g_hat"] = {"cases": len(cases), "equal_normal": eq, "differ": diff, "refused_both_nonnormal": refused_both,
                       "model_only_nonnormal": refused_model_only, "expr_only_nonnormal": refused_expr_only,
                       "first_differences": firsts,
                       "nu_0_3125_E_1": hx(g_scaled_model(1.0, 0.3125))}
    out_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    print(json.dumps(report, indent=1, sort_keys=True))


if __name__ == "__main__":
    main()
