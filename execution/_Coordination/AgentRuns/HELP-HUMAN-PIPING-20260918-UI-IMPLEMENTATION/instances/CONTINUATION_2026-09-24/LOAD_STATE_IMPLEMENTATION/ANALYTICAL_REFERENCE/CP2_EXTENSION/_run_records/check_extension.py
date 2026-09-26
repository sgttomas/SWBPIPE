#!/usr/bin/env python3
"""CP2_EXTENSION checker: preservation of every pre-existing fixture path, plus an
independent re-derivation of the added cases/variant.

Standard library only. It does not import the extension generator, the session-1
generator, the manager's checker or any product/kernel/test code. The inputs it
re-derives from are asserted equal to the values stated in the CP2_EXTENSION
brief. It uses different methods from the generator: segment antiderivatives
instead of trapezoids, bisect interpolation, expm1 at precision 150, and its own
pi from Machin's formula.

Usage: python3 check_extension.py <pre_edit.json> <post_edit.json>
"""
from bisect import bisect_right
from decimal import Decimal as D, localcontext
from fractions import Fraction as F
import copy
import difflib
import hashlib
import json
import re
import sys

ALLOWED_NEW = (
    "cases.multi_segment_free_length",
    "cases.temperature_unit_identity",
    "cases.thermal_datum_ratio.variants.verification_two_point",
)


# ------------------------------------------------------------------ strict JSON
def strict_load(raw):
    def pairs(items):
        keys = [k for k, _ in items]
        if len(keys) != len(set(keys)):
            raise ValueError(f"duplicate keys {keys}")
        return dict(items)

    def const(name):
        raise ValueError(f"non-finite constant {name}")

    return json.loads(raw, object_pairs_hook=pairs, parse_constant=const)


def flatten(x, path=""):
    """path -> (json type tag, scalar) for every leaf; containers tagged too."""
    out = {}
    if isinstance(x, dict):
        out[path] = ("object", None)
        for k, v in x.items():
            out.update(flatten(v, f"{path}.{k}" if path else k))
    elif isinstance(x, list):
        out[path] = ("array", None)
        for i, v in enumerate(x):
            out.update(flatten(v, f"{path}[{i}]"))
    else:
        tag = {bool: "bool", int: "int", float: "float", str: "str", type(None): "null"}[type(x)]
        out[path] = (tag, x)
    return out


def preservation(pre, post):
    """Every pre-existing path keeps identical type/value; key order kept; list lengths kept."""
    fails = []
    fp, fq = flatten(pre), flatten(post)
    for p, v in fp.items():
        if p not in fq:
            fails.append(f"missing pre-existing path {p!r}")
        elif fq[p] != v:
            fails.append(f"changed {p!r}: {v} -> {fq[p]}")

    def order(a, b, path=""):
        if isinstance(a, dict):
            kept = [k for k in b.keys() if k in a]
            if kept != list(a.keys()):
                fails.append(f"key order changed at {path!r}")
            for k in a:
                if k in b:
                    order(a[k], b[k], f"{path}.{k}" if path else k)
        elif isinstance(a, list):
            if len(a) != len(b):
                fails.append(f"array length changed at {path!r}: {len(a)} -> {len(b)}")
            for i, (u, w) in enumerate(zip(a, b)):
                order(u, w, f"{path}[{i}]")

    order(pre, post)
    added = sorted(p for p in fq if p not in fp)
    stray = [p for p in added if not any(p == a or p.startswith(a + ".") or p.startswith(a + "[") for a in ALLOWED_NEW)]
    if stray:
        fails.append(f"added paths outside the allowed new keys: {stray[:10]}")
    roots = sorted({a for a in ALLOWED_NEW for p in added if p == a})
    return fails, len(fp), added, roots


def textual_additive(pre_raw, post_raw):
    """Line diff: only insertions, except a trailing comma appended where a new sibling follows."""
    a, b = pre_raw.decode().splitlines(), post_raw.decode().splitlines()
    fails, inserted, comma = [], 0, 0
    for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes():
        if tag == "equal":
            continue
        if tag == "insert":
            inserted += j2 - j1
            continue
        if tag == "replace" and i2 - i1 == 1 and b[j1] == a[i1] + ",":
            comma += 1
            inserted += j2 - j1 - 1
            continue
        fails.append(f"non-additive {tag} pre[{i1}:{i2}] -> post[{j1}:{j2}]: {a[i1:i2][:3]} -> {b[j1:j2][:3]}")
    return fails, inserted, comma


# ------------------------------------------------------------------ numbers
def machin_pi(digits=110):
    with localcontext() as ctx:
        ctx.prec = digits + 10

        def arctan_inv(n):
            x = D(1) / n
            x2, term, total, k = x * x, x, x, 1
            while True:
                term *= -x2
                k += 2
                t = term / k
                if abs(t) < D(10) ** (-(digits + 5)):
                    break
                total += t
            return total

        return +(16 * arctan_inv(5) - 4 * arctan_inv(239))


PI150 = machin_pi(110)


def exact_of(q):
    e = q["exact"]
    if e["kind"] == "rational":
        return ("rational", F(e["rational"]))
    if e["kind"] == "rational_times_pi":
        return ("pi", F(e["rational"]))
    if e["kind"] == "symbolic":
        m = re.fullmatch(r"exp\((-?\d+(?:/\d+)?)\)-1", e["expression"])
        assert m, e
        return ("expm1", F(m.group(1)))
    raise ValueError(e)


def hi_value(kind, r):
    with localcontext() as ctx:
        ctx.prec = 150
        d = D(r.numerator) / D(r.denominator)
        if kind == "rational":
            return d
        if kind == "pi":
            return d * PI150
        return d.exp() - 1


class Checker:
    def __init__(self):
        self.checks, self.fails = 0, []

    def ok(self, cond, msg):
        self.checks += 1
        if not cond:
            self.fails.append(msg)

    def projection(self, name, q):
        """exact authoritative; decimal agrees; value is the correctly rounded binary64 projection."""
        self.ok(isinstance(q.get("unit"), str) and q["unit"] != "", f"{name}: unit missing")
        kind, r = exact_of(q)
        hv = hi_value(kind, r)
        with localcontext() as ctx:
            ctx.prec = 150
            dec = D(q["decimal"])
            if kind == "rational":
                self.ok(F(q["decimal"]) == r or abs(dec - hv) <= abs(hv) * D("1e-84"),
                        f"{name}: decimal {q['decimal']} does not match {r}")
                self.ok(q["value"] == float(r), f"{name}: value {q['value']!r} != float(exact) {float(r)!r}")
                with localcontext() as c85:
                    c85.prec = 85
                    conv = str(D(r.numerator) / D(r.denominator))
                self.ok(q["decimal"] == conv, f"{name}: decimal {q['decimal']} is not the prec-85 convention {conv}")
            else:
                tol = D("1e-83") if kind == "expm1" else abs(hv) * D("1e-83")
                self.ok(abs(dec - hv) <= tol, f"{name}: decimal off by {abs(dec - hv)}")
                self.ok(q["value"] == float(hv), f"{name}: value {q['value']!r} != correctly rounded {float(hv)!r}")
            if kind == "expm1":
                self.ok(q.get("evaluation") == "Decimal.exp, precision 85", f"{name}: evaluation label")
        return kind, r

    def exact(self, name, q, kind, r):
        self.checks += 1
        got = self.projection(name, q)
        if got != (kind, r):
            self.fails.append(f"{name}: fixture {got} != independent {(kind, r)}")


def walk_quantities(x, path=""):
    if isinstance(x, dict):
        if "exact" in x and "value" in x:
            yield path, x
            return
        for k, v in x.items():
            yield from walk_quantities(v, f"{path}.{k}")
    elif isinstance(x, list):
        for i, v in enumerate(x):
            yield from walk_quantities(v, f"{path}[{i}]")


def R(q):
    assert q["exact"]["kind"] == "rational", q
    return F(q["exact"]["rational"])


# ------------------------------------------------------------------ independent physics
def interp(points, T):
    ts = [t for t, _ in points]
    if not (ts[0] <= T <= ts[-1]):
        raise ValueError("outside coverage")
    k = min(bisect_right(ts, T), len(ts) - 1)
    (t0, y0), (t1, y1) = points[k - 1], points[k]
    return y0 + (y1 - y0) * (T - t0) / (t1 - t0)


def antiderivative_integral(points, a, b, keep=None):
    """integral_a^b of a piecewise-linear alpha via per-segment antiderivatives.
    keep(i, lo, hi, pieces) may drop pieces (used only to build wrong controls)."""
    lo, hi = min(a, b), max(a, b)
    interp(points, lo), interp(points, hi)
    pieces = []
    for i in range(len(points) - 1):
        (t0, a0), (t1, a1) = points[i], points[i + 1]
        s, e = max(lo, t0), min(hi, t1)
        if s < e:
            c1 = (a1 - a0) / (t1 - t0)
            c0 = a0 - c1 * t0
            A = lambda T: c0 * T + c1 * T * T / 2
            pieces.append((i, A(e) - A(s)))
    if keep is not None:
        pieces = keep(pieces)
    total = sum((v for _, v in pieces), F(0))
    return total if b >= a else -total


def check_multi_segment(c, doc):
    case = doc["cases"]["multi_segment_free_length"]
    c.ok(case["source"] == {**case["source"], "document": "VERIFICATION.md", "control": 5}, "ms source")
    v = case["variants"]["linear_coefficient_table"]
    i, e = v["inputs"], v["expected"]
    c.ok(i.get("provenance") == "invented", "ms coefficient inputs not labelled invented")
    c.ok(i["definitions"] == ["differential_per_datum_length", "logarithmic_per_current_length"], "ms definitions")
    for nm in ("datum_temperature", "installation_temperature", "operating_temperature", "split_temperature"):
        c.ok(i[nm]["unit"] == "K", f"ms {nm} unit")
    Tm, Ti, To, Ts = (R(i[n]) for n in ("datum_temperature", "installation_temperature", "operating_temperature", "split_temperature"))
    pts = [(R(p["temperature"]), R(p["coefficient"])) for p in i["linear_coefficient_points"]]
    # Inputs exactly as stated in the brief.
    c.ok((Tm, Ti, To, Ts) == (300, 350, 550, 450), "ms temperatures differ from brief")
    c.ok(pts == [(300, F("1e-5")), (400, F("2e-5")), (500, F("1e-5")), (600, F("3e-5"))], "ms coefficient table differs from brief")
    c.ok(all(p["coefficient"]["unit"] == "1/K" and p["temperature"]["unit"] == "K" for p in i["linear_coefficient_points"]), "ms table units")
    c.ok(all(Ti != t and To != t for t, _ in pts) and pts[0][0] < Ti < pts[1][0] and pts[2][0] < To < pts[3][0], "ms interior temperatures")

    I = lambda a, b: antiderivative_integral(pts, a, b)
    lam = lambda T: 1 + I(Tm, T)
    eps = lambda a, b: lam(b) / lam(a) - 1
    c.exact("ms.coef_install", e["coefficient_install"], "rational", interp(pts, Ti))
    c.exact("ms.coef_split", e["coefficient_split"], "rational", interp(pts, Ts))
    c.exact("ms.coef_operating", e["coefficient_operating"], "rational", interp(pts, To))
    c.exact("ms.I_m_i", e["integral_datum_to_install"], "rational", I(Tm, Ti))
    c.exact("ms.I_m_s", e["integral_datum_to_split"], "rational", I(Tm, Ts))
    c.exact("ms.I_m_o", e["integral_datum_to_operating"], "rational", I(Tm, To))
    c.exact("ms.I_i_o", e["integral_install_to_operating"], "rational", I(Ti, To))
    segs = e["segment_integrals_install_to_operating"]
    c.ok(len(segs) == 3 and [s["table_points"] for s in segs] == [[0, 1], [1, 2], [2, 3]], "ms must consume three segments")
    bounds = [Ti, pts[1][0], pts[2][0], To]
    for k, s in enumerate(segs):
        c.exact(f"ms.seg{k}.from", s["from"], "rational", bounds[k])
        c.exact(f"ms.seg{k}.to", s["to"], "rational", bounds[k + 1])
        c.exact(f"ms.seg{k}.integral", s["integral"], "rational", I(bounds[k], bounds[k + 1]))
    c.ok(sum(R(s["integral"]) for s in segs) == R(e["integral_install_to_operating"]), "ms segment sum")
    pairs = {"forward": (Ti, To), "reverse": (To, Ti), "install_to_split": (Ti, Ts), "split_to_operating": (Ts, To),
             "operating_to_split": (To, Ts), "split_to_install": (Ts, Ti)}
    for nm, (a, b) in pairs.items():
        c.exact(f"ms.datum.{nm}", e[f"datum_length_strain_{nm}"], "rational", eps(a, b))
        c.exact(f"ms.log.{nm}", e[f"current_length_strain_{nm}"], "expm1", I(a, b))
    x = {nm: R(e[f"datum_length_strain_{nm}"]) for nm in pairs}
    c.ok((1 + x["install_to_split"]) * (1 + x["split_to_operating"]) == 1 + x["forward"], "ms datum split composition")
    c.ok((1 + x["operating_to_split"]) * (1 + x["split_to_install"]) == 1 + x["reverse"], "ms datum reverse split composition")
    c.ok((1 + x["forward"]) * (1 + x["reverse"]) == 1, "ms datum reversal")
    g = {nm: exact_of(e[f"current_length_strain_{nm}"])[1] for nm in pairs}
    c.ok(g["install_to_split"] + g["split_to_operating"] == g["forward"] and g["forward"] == -g["reverse"], "ms log composition")
    with localcontext() as ctx:
        ctx.prec = 85
        lhs = (1 + D(e["current_length_strain_install_to_split"]["decimal"])) * (1 + D(e["current_length_strain_split_to_operating"]["decimal"]))
        c.ok(abs(lhs - (1 + D(e["current_length_strain_forward"]["decimal"]))) < D("1e-82"), "ms log decimal composition")

    # Dilation-table variant.
    v2 = doc["cases"]["multi_segment_free_length"]["variants"]["linear_dilation_table"]
    i2, e2 = v2["inputs"], v2["expected"]
    c.ok(i2.get("provenance") == "invented" and i2["definition"] == "engineering_dilation" and i2["interpolation"] == "linear_dilation", "ms dilation labels")
    dp = [(R(p["temperature"]), R(p["dilation"])) for p in i2["linear_dilation_points"]]
    c.ok(dp == [(300, 0), (400, F("1e-3")), (500, F("1.5e-3")), (600, F("3.5e-3"))], "ms dilation table differs from brief")
    c.ok(tuple(R(i2[n]) for n in ("datum_temperature", "installation_temperature", "operating_temperature", "split_temperature")) == (300, 350, 550, 450), "ms dilation temperatures")
    c.ok(interp(dp, Tm) == 0, "ms dilation datum rule d(T_m)=0")
    dl = lambda T: 1 + interp(dp, T)
    de = lambda a, b: dl(b) / dl(a) - 1
    c.exact("ms.dil.install", e2["dilation_install"], "rational", interp(dp, Ti))
    c.exact("ms.dil.split", e2["dilation_split"], "rational", interp(dp, Ts))
    c.exact("ms.dil.operating", e2["dilation_operating"], "rational", interp(dp, To))
    for nm, (a, b) in pairs.items():
        c.exact(f"ms.dil.{nm}", e2[f"dilation_strain_{nm}"], "rational", de(a, b))
    y = {nm: R(e2[f"dilation_strain_{nm}"]) for nm in pairs}
    c.ok((1 + y["install_to_split"]) * (1 + y["split_to_operating"]) == 1 + y["forward"], "ms dilation split composition")
    c.ok((1 + y["forward"]) * (1 + y["reverse"]) == 1, "ms dilation reversal")

    # Reviewer probe targets (after-the-fact cross-check; values quoted in the brief).
    c.ok(R(e["datum_length_strain_forward"]) == F(5, 1601), "probe 5/1601")
    c.ok(R(e["datum_length_strain_reverse"]) == F(-5, 1606), "probe -5/1606")
    c.ok(exact_of(e["current_length_strain_forward"]) == ("expm1", F("0.003125")), "probe expm1(0.003125)")
    c.ok(R(e2["dilation_strain_forward"]) == F(4, 2001), "probe 4/2001")

    # Wrong-result discriminators, rebuilt with the keep() hook.
    w = case["wrong_result_discriminators"]
    first = lambda ps: ps[:1]
    last = lambda ps: ps[-1:]
    li = lam(Ti)
    c.exact("ms.w.first.datum", w["first_segment_only"]["datum_length_strain_forward"], "rational", antiderivative_integral(pts, Ti, To, first) / li)
    c.exact("ms.w.first.log", w["first_segment_only"]["current_length_strain_forward"], "expm1", antiderivative_integral(pts, Ti, To, first))
    c.exact("ms.w.first.dil", w["first_segment_only"]["dilation_strain_forward"], "rational", (interp(dp, 400) - interp(dp, Ti)) / dl(Ti))
    # Datum-integral truncation to the first segment coincides here (T_install in segment 0).
    c.ok((1 + antiderivative_integral(pts, Tm, To, first)) / li - 1 == antiderivative_integral(pts, Ti, To, first) / li, "ms first-only forms coincide")
    wi = w["last_segment_only"]["interval_difference_form"]
    c.exact("ms.w.last_i.datum", wi["datum_length_strain_forward"], "rational", antiderivative_integral(pts, Ti, To, last) / li)
    c.exact("ms.w.last_i.log", wi["current_length_strain_forward"], "expm1", antiderivative_integral(pts, Ti, To, last))
    c.exact("ms.w.last_i.dil", wi["dilation_strain_forward"], "rational", (interp(dp, To) - interp(dp, 500)) / dl(Ti))
    wd = w["last_segment_only"]["datum_integral_form"]
    Io_last, Ii_last = antiderivative_integral(pts, Tm, To, last), antiderivative_integral(pts, Tm, Ti, last)
    c.ok(Ii_last == I(Tm, Ti), "ms install datum path is one segment")
    c.exact("ms.w.last_d.datum", wd["datum_length_strain_forward"], "rational", (1 + Io_last) / (1 + Ii_last) - 1)
    c.exact("ms.w.last_d.log", wd["current_length_strain_forward"], "expm1", Io_last - Ii_last)
    c.exact("ms.w.last_d.dil", wd["dilation_strain_forward"], "rational", (1 + interp(dp, To) - interp(dp, 500)) / dl(Ti) - 1)
    skip = (interp(pts, Ti) + interp(pts, To)) * (To - Ti) / 2
    c.exact("ms.w.skip.datum", w["interior_breakpoints_skipped"]["datum_length_strain_forward"], "rational", skip / li)
    c.exact("ms.w.skip.log", w["interior_breakpoints_skipped"]["current_length_strain_forward"], "expm1", skip)
    c.exact("ms.w.endpoint", w["endpoint_alpha_times_interval"]["strain"], "rational", interp(pts, To) * (To - Ti))
    # Every wrong control is separated from its correct value by far more than relative 1e-9.
    correct = {"datum_length_strain_forward": float(R(e["datum_length_strain_forward"])),
               "current_length_strain_forward": e["current_length_strain_forward"]["value"],
               "dilation_strain_forward": float(R(e2["dilation_strain_forward"])),
               "strain": float(R(e["datum_length_strain_forward"]))}
    for path, q in walk_quantities(w, "ms.w"):
        key = path.rsplit(".", 1)[1]
        rel = abs(q["value"] - correct[key]) / abs(correct[key])
        c.ok(rel > 1e-3, f"{path}: wrong control too close to correct ({rel})")


def check_two_point(c, doc):
    variants = doc["cases"]["thermal_datum_ratio"]["variants"]
    v = variants["verification_two_point"]
    i, e = v["inputs"], v["expected"]
    ck = lambda q: (c.ok(q["unit"] == "degC", "v2 unit"), R(q) + F("273.15"))[1]
    Tm, Ti, To = ck(i["datum_temperature"]), ck(i["installation_temperature"]), ck(i["operating_temperature"])
    pts = [(ck(p["temperature"]), R(p["alpha"])) for p in i["table_points"]]
    # VERIFICATION control 4 exact inputs.
    c.ok((Tm, Ti, To) == (F("293.15"), F("323.15"), F("423.15")), "v2 temperatures differ from VERIFICATION 4")
    c.ok(pts == [(F("323.15"), F("12e-6")), (F("423.15"), F("16e-6"))], "v2 table differs from VERIFICATION 4")
    c.ok(not any(t == Tm for t, _ in pts), "v2 table must not contain a datum point")
    c.ok(i["coefficient_definition"] == "engineering_secant" and i["interpolation"] == "linear_coefficient", "v2 labels")
    c.ok(R(i["alpha_install"]) == interp(pts, Ti) and R(i["alpha_operating"]) == interp(pts, To), "v2 alpha endpoints")
    lam = lambda T: 1 + interp(pts, T) * (T - Tm)
    strain = lam(To) / lam(Ti) - 1
    c.ok(strain == F(43, 25009), "v2 strain is not VERIFICATION 4's 43/25009")
    c.exact("v2.dil_i", e["dilation_install"], "rational", interp(pts, Ti) * (Ti - Tm))
    c.exact("v2.dil_o", e["dilation_operating"], "rational", interp(pts, To) * (To - Tm))
    c.exact("v2.stretch", e["thermal_stretch"], "rational", 1 + strain)
    c.exact("v2.strain", e["thermal_strain"], "rational", strain)
    c.exact("v2.Tm", e["datum_temperature_K"], "rational", Tm)
    c.exact("v2.Ti", e["installation_temperature_K"], "rational", Ti)
    c.exact("v2.To", e["operating_temperature_K"], "rational", To)
    lo, hi = min(Ti, To), max(Ti, To)
    c.exact("v2.cov_lo", e["required_coverage_low_K"], "rational", lo)
    c.exact("v2.cov_hi", e["required_coverage_high_K"], "rational", hi)
    c.ok(pts[0][0] <= lo and hi <= pts[-1][0], "v2 table covers the required interval")
    samples = [lo + (hi - lo) * k / 2000 for k in range(2001)]
    lmin = min(lam(t) for t in samples)
    c.ok(lmin > 0, "v2 stretch positive on the required interval")
    c.exact("v2.lam_min", e["minimum_datum_stretch_over_required_interval"], "rational", lmin)
    # Policy statement present and exact.
    pol = v["admissibility_policy"]
    c.ok(pol["datum_coverage_required"] is False and "[min(T_install,T), max(T_install,T)]" in pol["rule"]
         and "lambda(T_m)=1" in pol["rule"], "v2 policy text")
    # Annular companion: same values as the existing annular variant, and first principles.
    geom = doc["geometry"]
    od, t = R(geom["authored"]["outside_diameter"]), R(geom["authored"]["wall_thickness"])
    As = (od / 2) ** 2 - (od / 2 - t) ** 2
    L, E = R(i["L"]), R(i["E"])
    c.ok(i["geometry_ref"] == geom["id"] and (L, E) == (1, F("2e11")), "v2 companion inputs")
    c.exact("v2.tip", e["free_tip_UX"], "rational", L * strain)
    c.exact("v2.freeN", e["free_wall_N"], "rational", F(0))
    c.exact("v2.fixedN", e["fixed_wall_N"], "pi", -E * As * strain)
    c.exact("v2.root", e["fixed_root_Fx"], "pi", E * As * strain)
    c.exact("v2.far", e["fixed_far_Fx"], "pi", -E * As * strain)
    ann = variants["annular_companion"]["expected"]
    for k in ("thermal_strain", "thermal_stretch", "free_tip_UX", "free_wall_N", "fixed_wall_N", "fixed_root_Fx", "fixed_far_Fx",
              "dilation_install", "dilation_operating", "datum_temperature_K", "installation_temperature_K", "operating_temperature_K"):
        c.ok(e[k] == ann[k], f"v2.{k} differs from existing annular_companion")


TO_K = {"K": lambda x: x, "degC": lambda x: x + F("273.15"),
        "degF": lambda x: (x + F("459.67")) * 5 / 9, "degR": lambda x: x * 5 / 9}


def check_identity(c, doc):
    case = doc["cases"]["temperature_unit_identity"]
    v = case["variants"]["exact_affine_identity"]
    i, e = v["inputs"], v["expected"]
    c.ok(i.get("provenance") == "invented", "tu provenance")
    want = {"minus_50_degC": [("-50", "degC"), ("223.15", "K")],
            "242_degC": [("242", "degC"), ("467.6", "degF"), ("515.15", "K")],
            "20_degC_rankine": [("20", "degC"), ("527.67", "degR")]}
    got = {g["id"]: [(q["decimal"], q["unit"]) for q in g["authored"]] for g in i["identity_groups"]}
    c.ok(got == want, f"tu groups differ from brief: {got}")
    exp_by_id = {g["id"]: g for g in e["identity_groups"]}
    for g in i["identity_groups"]:
        ks = []
        for k, q in enumerate(g["authored"]):
            c.ok(F(q["decimal"]) == R(q), f"tu {g['id']}[{k}] decimal/exact disagree")
            ks.append(TO_K[q["unit"]](F(q["decimal"])))
        c.ok(len(set(ks)) == 1, f"tu {g['id']} not identical: {ks}")
        x = exp_by_id[g["id"]]
        c.ok(x["compare_equal"] is True, f"tu {g['id']} compare_equal")
        c.exact(f"tu.{g['id']}.kelvin", x["kelvin"], "rational", ks[0])
        for k, q in enumerate(x["authored_kelvin"]):
            c.exact(f"tu.{g['id']}.authored_kelvin[{k}]", q, "rational", ks[k])
    ne = i["non_equal_control"]
    ks = [TO_K[q["unit"]](F(q["decimal"])) for q in ne["authored"]]
    c.ok(ks[0] != ks[1], "tu non-equal control compares equal")
    xe = e["non_equal_control"]
    c.ok(xe["compare_equal"] is False, "tu non-equal flag")
    c.exact("tu.ne.diff", xe["difference_K"], "rational", ks[0] - ks[1])
    for k, q in enumerate(xe["authored_kelvin"]):
        c.exact(f"tu.ne.authored_kelvin[{k}]", q, "rational", ks[k])
    c.ok(abs(ks[0] - ks[1]) / ks[1] > F(1, 10 ** 9), "tu control separated beyond relative 1e-9")
    b = case["wrong_result_discriminators"]["binary64_affine_conversion"]
    m50 = -50.0 + 273.15
    f = (467.6 + 459.67) * 5 / 9
    c.exact("tu.b64.m50", b["minus_50_degC"]["result"], "rational", F(m50))
    c.exact("tu.b64.223", b["minus_50_degC"]["binary64_223p15_K"], "rational", F(223.15))
    c.ok(b["minus_50_degC"]["doubles_equal"] is (m50 == 223.15), "tu b64 m50 flag")
    c.exact("tu.b64.f", b["467p6_degF"]["result"], "rational", F(f))
    c.exact("tu.b64.c242", b["467p6_degF"]["binary64_242_degC_via_plus_273p15"], "rational", F(242.0 + 273.15))
    c.exact("tu.b64.515", b["467p6_degF"]["binary64_515p15_K"], "rational", F(515.15))
    c.ok(b["467p6_degF"]["doubles_equal"] is (f == 242.0 + 273.15 or f == 515.15), "tu b64 f flag")


FORBIDDEN = re.compile(r"\b(ASME|B31(\.\d+)?|ASTM|EN\s?13480|API|ISO|A106|A312|A335|SA-\d+|allowable stress|carbon steel|stainless|Inconel)\b", re.I)


def check_all(pre, post, pre_raw, post_raw):
    c = Checker()
    fails, n_pre, added, roots = preservation(pre, post)
    for f in fails:
        c.ok(False, f)
    c.ok(set(roots) == set(ALLOWED_NEW), f"expected new roots {ALLOWED_NEW}, got {roots}")
    tfails, inserted, comma = ([], 0, 0) if pre_raw is None else textual_additive(pre_raw, post_raw)
    for f in tfails:
        c.ok(False, f)
    if post_raw is not None:
        c.ok((json.dumps(post, indent=2, allow_nan=False) + "\n").encode() == post_raw, "post is not in the file's serialisation convention")
    check_multi_segment(c, post)
    check_two_point(c, post)
    check_identity(c, post)
    n_q = 0
    for root in ALLOWED_NEW:
        node = post
        for part in root.split("."):
            node = node[part]
        for path, q in walk_quantities(node, root):
            n_q += 1
            c.projection(path, q)
        text = json.dumps(node)
        m = FORBIDDEN.search(text)
        c.ok(m is None, f"{root}: library/code term {m.group(0) if m else ''}")
    return c, {"pre_paths_preserved": n_pre, "added_paths": len(added), "inserted_lines": inserted,
               "comma_only_line_changes": comma, "new_quantities_projected": n_q}


def mutate(post, fn):
    m = copy.deepcopy(post)
    fn(m)
    return m


def set_rational(q, r):
    q["exact"]["rational"] = str(r)
    with localcontext() as ctx:
        ctx.prec = 85
        q["decimal"] = str(D(r.numerator) / D(r.denominator))
    q["value"] = float(r)


def mutations(pre, post):
    ms = post["cases"]["multi_segment_free_length"]
    ce = lambda m: m["cases"]["multi_segment_free_length"]["variants"]["linear_coefficient_table"]["expected"]
    de = lambda m: m["cases"]["multi_segment_free_length"]["variants"]["linear_dilation_table"]["expected"]
    v2 = lambda m: m["cases"]["thermal_datum_ratio"]["variants"]["verification_two_point"]
    tu = lambda m: m["cases"]["temperature_unit_identity"]["variants"]["exact_affine_identity"]

    def log_to_last_only(m):
        q = ce(m)["current_length_strain_forward"]
        q["exact"]["expression"] = "exp(3/4000)-1"
        with localcontext() as ctx:
            ctx.prec = 85
            q["decimal"] = str(D("0.00075").exp() - 1)
        q["value"] = float(D(q["decimal"]))

    def add_datum_point(m):
        pts = v2(m)["inputs"]["table_points"]
        pts.insert(0, copy.deepcopy(post["cases"]["thermal_datum_ratio"]["variants"]["generic_reviewed"]["inputs"]["table_points"][0]))

    def one_ulp(m):
        q = ce(m)["datum_length_strain_forward"]
        q["value"] = q["value"] + 2 ** -60

    table = {
        "M1_consistent_last_segment_datum_strain": lambda m: set_rational(ce(m)["datum_length_strain_forward"], F(6, 8005)),
        "M2_value_projection_off": one_ulp,
        "M3_consistent_log_last_only": log_to_last_only,
        "M4_dilation_reverse_by_negation": lambda m: set_rational(de(m)["dilation_strain_reverse"], F(-4, 2001)),
        "M5_two_point_gains_20C_point": add_datum_point,
        "M6_identity_kelvin_binary64": lambda m: set_rational(tu(m)["expected"]["identity_groups"][0]["kelvin"], F(-50.0 + 273.15)),
        "M7_preexisting_value_changed": lambda m: m["cases"]["thermal_datum_ratio"]["limits"].__setitem__(0, "edited"),
        "M8_preexisting_key_removed": lambda m: m["cases"]["coefficient_definition"]["wrong_result_discriminators"].pop("reverse_by_negation"),
        "M9_split_composition_broken": lambda m: set_rational(ce(m)["datum_length_strain_split_to_operating"], F(11, 8005)),
        "M10_non_equal_control_made_equal": lambda m: tu(m)["inputs"]["non_equal_control"]["authored"][0].update(
            {"exact": {"kind": "rational", "rational": "-50"}, "decimal": "-50", "value": -50.0}),
    }
    out = {}
    for name, fn in table.items():
        m = mutate(post, fn)
        try:
            c, _ = check_all(pre, m, None, None)
            out[name] = {"detected": bool(c.fails), "first_failure": c.fails[0] if c.fails else None}
        except Exception as exc:  # a crash on malformed data also counts as detection
            out[name] = {"detected": True, "first_failure": f"exception {type(exc).__name__}: {exc}"}
    return out


def main():
    pre_path, post_path = sys.argv[1], sys.argv[2]
    pre_raw, post_raw = open(pre_path, "rb").read(), open(post_path, "rb").read()
    pre, post = strict_load(pre_raw), strict_load(post_raw)
    c, stats = check_all(pre, post, pre_raw, post_raw)
    muts = mutations(pre, post)
    report = {
        "pre": {"path_arg": pre_path, "sha256": hashlib.sha256(pre_raw).hexdigest(), "bytes": len(pre_raw)},
        "post": {"path_arg": post_path, "sha256": hashlib.sha256(post_raw).hexdigest(), "bytes": len(post_raw)},
        "valid_json_strict": True,
        "machin_pi_matches_fixture_pi_string": str(PI150)[:102] == post["numeric_representation"]["pi_decimal"][:102],
        "stats": stats,
        "checks": c.checks,
        "failures": c.fails,
        "mutations": muts,
        "all_mutations_detected": all(x["detected"] for x in muts.values()),
    }
    print(json.dumps(report, indent=1))
    ok = not c.fails and report["all_mutations_detected"] and report["machin_pi_matches_fixture_pi_string"]
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
