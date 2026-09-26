#!/usr/bin/env python3
"""CP2 independent review: re-derive the additive fixture sections from first
principles (standard library only) and check additivity against the CP1 bytes.

It does not import or read the extension generator, its checker, the manager's
checker or any product code. Pi comes from Machin's formula.
Usage: review2_fixture_rederive.py <current reference_cases.json> <cp1 reference_cases.json>
"""
import json
import sys
from decimal import Decimal, getcontext
from fractions import Fraction as F

getcontext().prec = 110
checks = 0
failures = []


def machin_pi():
    getcontext().prec += 10

    def arctan_inv(x):
        x = Decimal(x)
        total, term, n, sign = Decimal(0), 1 / x, 1, 1
        x2 = x * x
        while term != 0:
            total += sign * term / n
            term /= x2
            n += 2
            sign = -sign
        return total

    pi = 4 * (4 * arctan_inv(5) - arctan_inv(239))
    getcontext().prec -= 10
    return +pi


PI = machin_pi()


def ok(cond, what):
    global checks
    checks += 1
    if not cond:
        failures.append(what)


def exact_of(q):
    e = q["exact"]
    if e["kind"] == "rational":
        return ("r", F(e["rational"]))
    if e["kind"] == "rational_times_pi":
        return ("pi", F(e["rational"]))
    if e["kind"] == "symbolic":
        return ("sym", e["expression"])
    raise ValueError(e)


def dec_of_exact(kind, val):
    if kind == "r":
        return Decimal(val.numerator) / Decimal(val.denominator)
    if kind == "pi":
        return Decimal(val.numerator) / Decimal(val.denominator) * PI
    raise ValueError


def check_quantity(q, expected, unit, what, kind="r"):
    """expected: Fraction (kind r/pi) or Decimal (kind sym). Checks exact, decimal and value."""
    ok(q["unit"] == unit, f"{what}: unit {q['unit']} != {unit}")
    k, v = exact_of(q)
    if kind in ("r", "pi"):
        ok(k == kind and v == expected, f"{what}: exact {q['exact']} != {kind} {expected}")
        d = dec_of_exact(kind, expected)
    else:
        ok(k == "sym", f"{what}: exact kind {q['exact']}")
        d = expected
    # decimal agrees to >= 80 significant digits
    authored = Decimal(q["decimal"])
    if d == 0:
        ok(authored == 0, f"{what}: decimal {authored} != 0")
    else:
        rel = abs((authored - d) / d)
        ok(rel < Decimal("1e-80"), f"{what}: decimal rel err {rel}")
    # value is the correctly rounded binary64
    ok(q["value"] == float(d) if kind != "r" else q["value"] == float(expected) if expected.denominator < 10**300 else True,
       f"{what}: value {q['value']!r} != {float(d)!r}")


def dexp(x):
    getcontext().prec = 110
    return Decimal(x.numerator) / Decimal(x.denominator) if isinstance(x, F) else x


def exp_minus_1(fr):
    x = Decimal(fr.numerator) / Decimal(fr.denominator)
    return x.exp() - 1


def lerp(points, t):
    for (t0, v0), (t1, v1) in zip(points, points[1:]):
        if t0 <= t <= t1:
            return v0 + (v1 - v0) * (t - t0) / (t1 - t0)
    raise ValueError("outside")


def integral(points, a, b):
    """exact signed integral of piecewise-linear points from a to b"""
    sign = 1
    if b < a:
        a, b, sign = b, a, -1
    total = F(0)
    for (t0, v0), (t1, v1) in zip(points, points[1:]):
        lo, hi = max(a, t0), min(b, t1)
        if lo < hi:
            total += (lerp(points, lo) + lerp(points, hi)) * (hi - lo) / 2
    return sign * total


def main(current_path, cp1_path):
    cur = json.load(open(current_path))
    old = json.load(open(cp1_path))

    # ---- additivity: every CP1 path/value is unchanged -------------------
    def walk(o, prefix=()):
        if isinstance(o, dict):
            for k, v in o.items():
                yield from walk(v, prefix + (k,))
        elif isinstance(o, list):
            for i, v in enumerate(o):
                yield from walk(v, prefix + (i,))
        else:
            yield prefix, o

    def get(o, path):
        for p in path:
            o = o[p]
        return o

    old_paths = list(walk(old))
    for path, value in old_paths:
        try:
            now = get(cur, path)
        except (KeyError, IndexError, TypeError):
            ok(False, f"additivity: path {path} removed")
            continue
        ok(now == value and type(now) == type(value), f"additivity: {path} changed {value!r} -> {now!r}")
    new_paths = [p for p, _ in walk(cur)]
    print(f"cp1 leaf paths={len(old_paths)} current leaf paths={len(new_paths)}")
    added_top = sorted(set(cur["cases"]) - set(old["cases"]))
    print("added cases:", added_top)
    print("added thermal_datum_ratio variants:",
          sorted(set(cur["cases"]["thermal_datum_ratio"]["variants"]) - set(old["cases"]["thermal_datum_ratio"]["variants"])))

    # ---- multi_segment_free_length --------------------------------------
    ms = cur["cases"]["multi_segment_free_length"]["variants"]
    lc = ms["linear_coefficient_table"]
    inp = lc["inputs"]
    T = lambda q: F(exact_of(q)[1])
    Tm, Ti, To, Ts = (T(inp[k]) for k in ("datum_temperature", "installation_temperature", "operating_temperature", "split_temperature"))
    pts = [(T(p["temperature"]), T(p["coefficient"])) for p in inp["linear_coefficient_points"]]
    # the inputs themselves: invented; recheck their exact/decimal/value representations
    for i, p in enumerate(inp["linear_coefficient_points"]):
        check_quantity(p["temperature"], pts[i][0], "K", f"ms.coef.points[{i}].T")
        check_quantity(p["coefficient"], pts[i][1], "1/K", f"ms.coef.points[{i}].alpha")
    ex = lc["expected"]
    check_quantity(ex["coefficient_install"], lerp(pts, Ti), "1/K", "ms.coefficient_install")
    check_quantity(ex["coefficient_split"], lerp(pts, Ts), "1/K", "ms.coefficient_split")
    check_quantity(ex["coefficient_operating"], lerp(pts, To), "1/K", "ms.coefficient_operating")
    check_quantity(ex["integral_datum_to_install"], integral(pts, Tm, Ti), "1", "ms.I(Tm,Ti)")
    check_quantity(ex["integral_datum_to_split"], integral(pts, Tm, Ts), "1", "ms.I(Tm,Ts)")
    check_quantity(ex["integral_datum_to_operating"], integral(pts, Tm, To), "1", "ms.I(Tm,To)")
    check_quantity(ex["integral_install_to_operating"], integral(pts, Ti, To), "1", "ms.I(Ti,To)")
    breaks = sorted({Ti, To} | {t for t, _ in pts if Ti < t < To})
    segs = ex["segment_integrals_install_to_operating"]
    ok(len(segs) == len(breaks) - 1, "ms: segment count")
    for n, (a, b) in enumerate(zip(breaks, breaks[1:])):
        check_quantity(segs[n]["from"], a, "K", f"ms.seg[{n}].from")
        check_quantity(segs[n]["to"], b, "K", f"ms.seg[{n}].to")
        check_quantity(segs[n]["integral"], integral(pts, a, b), "1", f"ms.seg[{n}].integral")
        lower = max(i for i, (t, _) in enumerate(pts) if t <= a)
        ok(segs[n]["table_points"] == [lower, lower + 1], f"ms.seg[{n}].table_points")
    lam = lambda t: 1 + integral(pts, Tm, t)
    pairs = {"forward": (Ti, To), "reverse": (To, Ti), "install_to_split": (Ti, Ts),
             "split_to_operating": (Ts, To), "operating_to_split": (To, Ts), "split_to_install": (Ts, Ti)}
    for name, (a, b) in pairs.items():
        check_quantity(ex[f"datum_length_strain_{name}"], lam(b) / lam(a) - 1, "1", f"ms.datum_{name}")
        check_quantity(ex[f"current_length_strain_{name}"], exp_minus_1(integral(pts, a, b)), "1",
                       f"ms.log_{name}", kind="sym")
        ok(ex[f"current_length_strain_{name}"]["exact"]["expression"] == f"exp({integral(pts, a, b)})-1",
           f"ms.log_{name}: expression {ex[f'current_length_strain_{name}']['exact']['expression']}")
    # composition identities (exact)
    ok((lam(Ts) / lam(Ti)) * (lam(To) / lam(Ts)) == lam(To) / lam(Ti), "ms: split composition")
    # discriminators (forward only)
    wr = cur["cases"]["multi_segment_free_length"]["wrong_result_discriminators"]
    first = [p for p in pts if p[0] <= 400]
    # first-segment-only numerator: integral over [Ti, 400] only
    I_first = integral(pts, Ti, F(400))
    check_quantity(wr["first_segment_only"]["datum_length_strain_forward"], I_first / lam(Ti), "1", "wr.first.datum")
    check_quantity(wr["first_segment_only"]["current_length_strain_forward"], exp_minus_1(I_first), "1", "wr.first.log", kind="sym")
    I_last = integral(pts, F(500), To)
    check_quantity(wr["last_segment_only"]["interval_difference_form"]["datum_length_strain_forward"], I_last / lam(Ti), "1", "wr.last.diff.datum")
    check_quantity(wr["last_segment_only"]["interval_difference_form"]["current_length_strain_forward"], exp_minus_1(I_last), "1", "wr.last.diff.log", kind="sym")
    lam_last = 1 + integral(pts, F(500), To)
    check_quantity(wr["last_segment_only"]["datum_integral_form"]["datum_length_strain_forward"], lam_last / lam(Ti) - 1, "1", "wr.last.int.datum")
    check_quantity(wr["last_segment_only"]["datum_integral_form"]["current_length_strain_forward"],
                   exp_minus_1(integral(pts, F(500), To) - integral(pts, Tm, Ti)), "1", "wr.last.int.log", kind="sym")
    one_trap = (lerp(pts, Ti) + lerp(pts, To)) * (To - Ti) / 2
    check_quantity(wr["interior_breakpoints_skipped"]["datum_length_strain_forward"], one_trap / lam(Ti), "1", "wr.skip.datum")
    check_quantity(wr["interior_breakpoints_skipped"]["current_length_strain_forward"], exp_minus_1(one_trap), "1", "wr.skip.log", kind="sym")
    check_quantity(wr["endpoint_alpha_times_interval"]["strain"], lerp(pts, To) * (To - Ti), "1", "wr.endpoint_alpha")
    # each discriminator must differ from the correct value by far more than 1e-9 relative
    correct = lam(To) / lam(Ti) - 1
    for w in (I_first / lam(Ti), I_last / lam(Ti), lam_last / lam(Ti) - 1, one_trap / lam(Ti), lerp(pts, To) * (To - Ti)):
        ok(abs(w - correct) > correct * F(1, 10**6), "ms: discriminator not separated")

    dl = ms["linear_dilation_table"]
    inp = dl["inputs"]
    Tm, Ti, To, Ts = (T(inp[k]) for k in ("datum_temperature", "installation_temperature", "operating_temperature", "split_temperature"))
    dp = [(T(p["temperature"]), T(p["dilation"])) for p in inp["linear_dilation_points"]]
    ok(lerp(dp, Tm) == 0, "dilation zero at datum")
    ex = dl["expected"]
    check_quantity(ex["dilation_install"], lerp(dp, Ti), "1", "dl.d(Ti)")
    check_quantity(ex["dilation_split"], lerp(dp, Ts), "1", "dl.d(Ts)")
    check_quantity(ex["dilation_operating"], lerp(dp, To), "1", "dl.d(To)")
    lamd = lambda t: 1 + lerp(dp, t)
    for name, (a, b) in {"forward": (Ti, To), "reverse": (To, Ti), "install_to_split": (Ti, Ts),
                         "split_to_operating": (Ts, To), "operating_to_split": (To, Ts), "split_to_install": (Ts, Ti)}.items():
        check_quantity(ex[f"dilation_strain_{name}"], lamd(b) / lamd(a) - 1, "1", f"dl.{name}")
    check_quantity(wr["first_segment_only"]["dilation_strain_forward"], (lerp(dp, F(400)) - lerp(dp, Ti)) / lamd(Ti), "1", "wr.first.dil")
    check_quantity(wr["last_segment_only"]["interval_difference_form"]["dilation_strain_forward"], (lerp(dp, To) - lerp(dp, F(500))) / lamd(Ti), "1", "wr.last.diff.dil")
    check_quantity(wr["last_segment_only"]["datum_integral_form"]["dilation_strain_forward"], (1 + lerp(dp, To) - lerp(dp, F(500))) / lamd(Ti) - 1, "1", "wr.last.int.dil")

    # ---- thermal_datum_ratio.verification_two_point (VERIFICATION control 4) --
    v = cur["cases"]["thermal_datum_ratio"]["variants"]["verification_two_point"]
    inp = v["inputs"]
    c2k = lambda c: c + F(27315, 100)
    tm, ti, to = (c2k(T(inp[k])) for k in ("datum_temperature", "installation_temperature", "operating_temperature"))
    # VERIFICATION.md control 4 inputs, typed here from the design text, not from the fixture
    ok(T(inp["datum_temperature"]) == 20 and T(inp["installation_temperature"]) == 50 and T(inp["operating_temperature"]) == 150,
       "cp4 temperatures")
    ai, ao = F(12, 10**6), F(16, 10**6)
    ok(T(inp["alpha_install"]) == ai and T(inp["alpha_operating"]) == ao, "cp4 alphas")
    tp = inp["table_points"]
    ok(len(tp) == 2 and T(tp[0]["temperature"]) == 50 and T(tp[1]["temperature"]) == 150 and T(tp[0]["alpha"]) == ai
       and T(tp[1]["alpha"]) == ao, "cp4 table is exactly the two reviewed points")
    ex = v["expected"]
    di, do = ai * (ti - tm), ao * (to - tm)
    check_quantity(ex["dilation_install"], di, "1", "cp4.d_i")
    check_quantity(ex["dilation_operating"], do, "1", "cp4.d_o")
    check_quantity(ex["thermal_stretch"], (1 + do) / (1 + di), "1", "cp4.stretch")
    eps = (1 + do) / (1 + di) - 1
    ok(eps == F(43, 25009), "cp4 = 43/25009 (VERIFICATION text)")
    check_quantity(ex["thermal_strain"], eps, "1", "cp4.strain")
    check_quantity(ex["datum_temperature_K"], tm, "K", "cp4.Tm")
    check_quantity(ex["installation_temperature_K"], ti, "K", "cp4.Ti")
    check_quantity(ex["operating_temperature_K"], to, "K", "cp4.To")
    check_quantity(ex["required_coverage_low_K"], min(ti, to), "K", "cp4.cov_lo")
    check_quantity(ex["required_coverage_high_K"], max(ti, to), "K", "cp4.cov_hi")
    # secant lambda(t)=1+alpha(t)(t-Tm), alpha linear on [Ti,To]; minimum over the interval
    def lam_sec(t):
        a = ai + (ao - ai) * (t - ti) / (to - ti)
        return 1 + a * (t - tm)
    # quadratic: stationary point
    slope = (ao - ai) / (to - ti)
    # lambda = 1 + (ai + slope (t-ti))(t-tm); derivative: slope (t-tm) + ai + slope (t - ti) = 0
    tstar = (slope * (tm + ti) - ai) / (2 * slope)
    cands = [ti, to] + ([tstar] if ti < tstar < to else [])
    check_quantity(ex["minimum_datum_stretch_over_required_interval"], min(lam_sec(t) for t in cands), "1", "cp4.min_stretch")
    # mechanics: annulus OD 0.2 wall 0.01, L=1, E=200 GPa
    ro, ri = F(1, 10), F(9, 100)
    As_over_pi = ro * ro - ri * ri
    E = F(200 * 10**9)
    ok(T(inp["E"]) == E and T(inp["L"]) == 1, "cp4 E/L")
    check_quantity(ex["free_tip_UX"], eps * 1, "m", "cp4.tip")
    check_quantity(ex["free_wall_N"], F(0), "N", "cp4.freeN")
    check_quantity(ex["fixed_wall_N"], -E * As_over_pi * eps, "N", "cp4.fixedN", kind="pi")
    check_quantity(ex["fixed_root_Fx"], E * As_over_pi * eps, "N", "cp4.rootFx", kind="pi")
    check_quantity(ex["fixed_far_Fx"], -E * As_over_pi * eps, "N", "cp4.farFx", kind="pi")

    # ---- temperature_unit_identity --------------------------------------
    tu = cur["cases"]["temperature_unit_identity"]
    ev = tu["variants"]["exact_affine_identity"]
    def to_k(unit, value):
        return {"K": value, "degC": value + F(27315, 100), "degF": (value + F(45967, 100)) * F(5, 9), "degR": value * F(5, 9)}[unit]
    for g_in, g_ex in zip(ev["inputs"]["identity_groups"], ev["expected"]["identity_groups"]):
        ks = [to_k(q["unit"], T(q)) for q in g_in["authored"]]
        # authored decimals: exact rational equals the shortest decimal of the binary64 value
        for q in g_in["authored"]:
            ok(F(repr(q["value"])) == T(q), f"tu.{g_in['id']}: authored exact != shortest decimal of value")
        ok(len(set(ks)) == 1, f"tu.{g_in['id']}: not one exact identity {ks}")
        check_quantity(g_ex["kelvin"], ks[0], "K", f"tu.{g_in['id']}.kelvin")
        for n, q in enumerate(g_ex["authored_kelvin"]):
            check_quantity(q, ks[n], "K", f"tu.{g_in['id']}.authored_kelvin[{n}]")
        ok(g_ex["compare_equal"] is True, f"tu.{g_in['id']}.compare_equal")
    ne_in, ne_ex = ev["inputs"]["non_equal_control"], ev["expected"]["non_equal_control"]
    ks = [to_k(q["unit"], T(q)) for q in ne_in["authored"]]
    for n, q in enumerate(ne_ex["authored_kelvin"]):
        check_quantity(q, ks[n], "K", f"tu.nonequal.authored_kelvin[{n}]")
    check_quantity(ne_ex["difference_K"], ks[0] - ks[1], "K", "tu.nonequal.diff")
    ok(ne_ex["compare_equal"] is False and ks[0] != ks[1], "tu.nonequal.compare")
    b = tu["wrong_result_discriminators"]["binary64_affine_conversion"]
    m50 = -50.0 + 273.15
    check_quantity(b["minus_50_degC"]["result"], F(m50), "K", "tu.b64.-50")
    check_quantity(b["minus_50_degC"]["binary64_223p15_K"], F(223.15), "K", "tu.b64.223.15")
    ok(b["minus_50_degC"]["doubles_equal"] == (m50 == 223.15), "tu.b64.-50 equality flag")
    f4676 = (467.6 + 459.67) * 5 / 9
    check_quantity(b["467p6_degF"]["result"], F(f4676), "K", "tu.b64.467.6F")
    check_quantity(b["467p6_degF"]["binary64_242_degC_via_plus_273p15"], F(242.0 + 273.15), "K", "tu.b64.242C")
    check_quantity(b["467p6_degF"]["binary64_515p15_K"], F(515.15), "K", "tu.b64.515.15")
    ok(b["467p6_degF"]["doubles_equal"] == (f4676 == 515.15), "tu.b64.467.6 equality flag")

    print(f"checks={checks} failures={len(failures)}")
    for f in failures:
        print("FAIL", f)
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1], sys.argv[2]))
