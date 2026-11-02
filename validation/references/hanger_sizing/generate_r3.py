#!/usr/bin/env python3
"""Independent reference generator for hanger-sizing arithmetic (DEL-04-11).

Covers R-3 (loads, fit tests, family policy, selection), R-3b (request refusal
for W <= 0) and the arithmetic part of R-4/R-4b (verification pass), as settled
in execution/PKG-04/DEL-04-11/Work/hanger-sizing.md sections 4.2, 6 and 7.

Independence: written from the design text and the task brief only. It reads,
searches and imports no product code and no other generator. Standard library
only. Every value is invented.

Usage:
    python3 generate_r3.py          # write r3_arithmetic.json
    python3 generate_r3.py --check  # exit 1 if the frozen JSON differs
"""

import argparse
import json
import sys
from fractions import Fraction
from pathlib import Path

SCHEMA = "swbpipe.hanger_sizing_reference.r3/1"
OUT = Path(__file__).resolve().parent / "r3_arithmetic.json"

VAR = "variable_spring_hanger"
CON = "constant_effort_support"
REFUSAL_CODE = "HANGER_SIZING_LOAD_NOT_POSITIVE"
VAR_TESTS = ("E1", "E2", "V0", "L", "T", "V")
CON_TESTS = ("E1", "L", "T")

BASIS = (
    "Independent reference for DEL-04-11 hanger-sizing arithmetic: R-3, R-3b and "
    "the arithmetic part of R-4/R-4b, from execution/PKG-04/DEL-04-11/Work/"
    "hanger-sizing.md sections 4.2, 6 and 7. SI units (N, m, N/m). Every position "
    "is measured from the installed position u = 0 along the candidate's positive "
    "DOF axis; a variable spring set to F_cold carries F(u) = F_cold - k*u. "
    "design_index is 0-based into u_operating_m. Inputs are the exact rational "
    "values of the decimals (or stated fractions) in the generator; every result "
    "is computed exactly with fractions.Fraction and emitted rounded to the "
    "nearest binary64. Fit tests, selection, tie-breaks and verification tests "
    "are decided on exact rational values; boundary-equality cases use inputs "
    "exactly representable in binary64, and the generator confirms that every "
    "discrete outcome is unchanged when each input is replaced by the exact value "
    "of its emitted binary64 number. Every value is invented; no supplier "
    "catalogue or standards data is used."
)


# ---------------------------------------------------------------- inputs


def q(x):
    """Exact rational from a decimal string, int or Fraction (floats refused)."""
    if x is None:
        return None
    if isinstance(x, Fraction):
        return x
    if isinstance(x, (str, int)) and not isinstance(x, bool):
        return Fraction(x)
    raise TypeError(f"inexact input {x!r}; use a decimal string or Fraction")


def method(load_design="hot", ref="operating_load", lam="0.25", a="0",
           rule="smallest_capacity"):
    return {"load_design": load_design, "variation_reference": ref,
            "variation_limit": q(lam), "travel_allowance_m": q(a),
            "selection_rule": rule}


def vs(hid, k, rng, travel, dof="UZ"):
    return {"hanger_id": hid, "hanger_type": VAR, "k": q(k),
            "dof": dof if k is not None else None,
            "range": None if rng is None else (q(rng[0]), q(rng[1])),
            "travel": q(travel)}


def ce(hid, rng, travel):
    return {"hanger_id": hid, "hanger_type": CON, "k": None, "dof": None,
            "range": None if rng is None else (q(rng[0]), q(rng[1])),
            "travel": q(travel)}


def cand(cid, w, uc, uops, d=0, policy="variable", dof="UZ", verified=None):
    return {"id": cid, "dof": dof, "policy": policy, "w": q(w), "uc": q(uc),
            "uops": [q(u) for u in uops], "d": d,
            "verified": None if verified is None else
            (q(verified[0]), [q(u) for u in verified[1]])}


# ---------------------------------------------------------------- equations


def span(uc, uops):
    pts = [Fraction(0), uc] + list(uops)
    return min(pts), max(pts)


def f_cold(m, c, k):
    if m["load_design"] == "hot":
        return c["w"] + k * c["uops"][c["d"]]
    return c["w"] + k * c["uc"]


def variations(m, w, k, uc, uops, fi):
    out = []
    for u, f in zip(uops, fi):
        r = f if m["variation_reference"] == "operating_load" else w
        if r <= 0:
            raise ValueError("variation reference not positive; ratio undefined")
        out.append(k * abs(u - uc) / r)
    return out


def envelope(fc0, k, smin, smax, a):
    return (fc0 - k * (smax + a), fc0 - k * (smin - a))


def fit_variable(m, c, rec):
    if rec["k"] is None or rec["range"] is None or rec["travel"] is None:
        return "E1", None
    if rec["dof"] != c["dof"]:
        return "E2", None
    k, a = rec["k"], m["travel_allowance_m"]
    fc0 = f_cold(m, c, k)
    fcc = fc0 - k * c["uc"]
    fi = [fc0 - k * u for u in c["uops"]]
    if not (fcc > 0 and all(f > 0 for f in fi)):
        return "V0", None
    smin, smax = span(c["uc"], c["uops"])
    req = smax - smin + 2 * a
    lo, hi = envelope(fc0, k, smin, smax, a)
    fmin, fmax = rec["range"]
    if not (lo >= fmin and hi <= fmax):
        return "L", None
    if not (req <= rec["travel"]):
        return "T", None
    v = variations(m, c["w"], k, c["uc"], c["uops"], fi)
    if not all(x <= m["variation_limit"] for x in v):
        return "V", None
    return None, {"f_cold": fc0, "f_cc": fcc, "f_op": fi, "v": v,
                  "envelope": (lo, hi), "maxv": max(v)}


def fit_constant(m, c, rec):
    if rec["range"] is None or rec["travel"] is None:
        return "E1", None
    a = m["travel_allowance_m"]
    smin, smax = span(c["uc"], c["uops"])
    req = smax - smin + 2 * a
    fmin, fmax = rec["range"]
    if not (fmin <= c["w"] <= fmax):
        return "L", None
    if not (req <= rec["travel"]):
        return "T", None
    return None, {"maxv": Fraction(0)}


def select(rule, fits):
    def key(item):
        rec, det = item
        fmax = rec["range"][1]
        hid = rec["hanger_id"].encode("utf-8")
        if rule == "smallest_capacity":
            return (fmax, det["maxv"], hid)
        if rule == "least_variation":
            return (det["maxv"], fmax, hid)
        raise ValueError(rule)
    return min(fits, key=key)


def size_candidate(m, c, records):
    counts = {}
    fits = []

    def run(family):
        hanger_type, tests, fn = ((VAR, VAR_TESTS, fit_variable) if family == "variable"
                                  else (CON, CON_TESTS, fit_constant))
        counts[family] = {t: 0 for t in tests}
        found = []
        for rec in records:
            if rec["hanger_type"] != hanger_type:
                continue
            reason, det = fn(m, c, rec)
            if reason is None:
                found.append((rec, det))
            else:
                counts[family][reason] += 1
        return found

    fallback = False
    if c["policy"] == "variable":
        fits = run("variable")
    elif c["policy"] == "constant":
        fits = run("constant")
    elif c["policy"] == "variable_else_constant":
        fits = run("variable")
        if not fits:
            fits = run("constant")
            fallback = True
    else:
        raise ValueError(c["policy"])

    a = m["travel_allowance_m"]
    smin, smax = span(c["uc"], c["uops"])
    row = {"id": c["id"], "outcome": "no_record_fits", "hanger_id": None,
           "f_cold_n": None, "f_cold_condition_n": None, "f_operating_n": None,
           "variation": None, "span_m": [smin, smax],
           "required_travel_m": smax - smin + 2 * a, "envelope_n": None,
           "failure_counts": counts, "verification": None}
    if not fits:
        return row, None, None
    rec, det = select(m["selection_rule"], fits)
    row["outcome"] = "selected_by_fallback" if fallback else "selected"
    row["hanger_id"] = rec["hanger_id"]
    if rec["hanger_type"] == VAR:
        row["f_cold_n"] = det["f_cold"]
        row["f_cold_condition_n"] = det["f_cc"]
        row["f_operating_n"] = det["f_op"]
        row["variation"] = det["v"]
        row["envelope_n"] = list(det["envelope"])
    else:
        row["f_cold_n"] = c["w"]
    return row, rec, det


def verify(m, c, rec, det):
    uc, uops = c["verified"]
    a, lam = m["travel_allowance_m"], m["variation_limit"]
    smin, smax = span(uc, uops)
    req = smax - smin + 2 * a
    t_pass = req <= rec["travel"]
    out = {"u_cold_m": uc, "u_operating_m": list(uops), "span_m": [smin, smax],
           "required_travel_m": req, "t_pass": t_pass,
           "f_cold_condition_n": None, "f_operating_n": None, "variation": None,
           "departure_n": None, "failed_tests": []}
    if rec["hanger_type"] == CON:
        out["failed_tests"] = [] if t_pass else ["T"]
        return out
    k, fc0, w = rec["k"], det["f_cold"], c["w"]
    fcc = fc0 - k * uc
    fi = [fc0 - k * u for u in uops]
    v0 = fcc > 0 and all(f > 0 for f in fi)
    v = variations(m, w, k, uc, uops, fi)
    lo, hi = envelope(fc0, k, smin, smax, a)
    fmin, fmax = rec["range"]
    l_pass = lo >= fmin and hi <= fmax
    v_pass = all(x <= lam for x in v)
    out["f_cold_condition_n"] = fcc
    out["f_operating_n"] = fi
    out["variation"] = v
    out["departure_n"] = (fi[c["d"]] - w) if m["load_design"] == "hot" else (fcc - w)
    out["failed_tests"] = [name for name, ok in
                           (("V0", v0), ("L", l_pass), ("T", t_pass), ("V", v_pass))
                           if not ok]
    return out


def expected(case):
    m, records, cands = case["method"], case["records"], case["candidates"]
    bad = [c["id"] for c in cands if c["w"] <= 0]
    if bad:
        return {"refused": {"code": REFUSAL_CODE, "candidates": bad}}
    rows = []
    any_no_fit = False
    for c in cands:
        row, rec, det = size_candidate(m, c, records)
        if rec is None:
            any_no_fit = True
        elif c["verified"] is not None:
            row["verification"] = verify(m, c, rec, det)
        rows.append(row)
    return {"candidates": rows,
            "verification_standing": "conditional" if any_no_fit else "unconditional"}


# ---------------------------------------------------------------- self-checks


def binarized(case):
    """The case with every input replaced by the exact value of its binary64."""
    b = lambda x: None if x is None else Fraction(float(x))
    m = dict(case["method"])
    m["variation_limit"] = b(m["variation_limit"])
    m["travel_allowance_m"] = b(m["travel_allowance_m"])
    recs = []
    for r in case["records"]:
        r = dict(r)
        r["k"] = b(r["k"])
        r["travel"] = b(r["travel"])
        r["range"] = None if r["range"] is None else (b(r["range"][0]), b(r["range"][1]))
        recs.append(r)
    cands = []
    for c in case["candidates"]:
        c = dict(c)
        c["w"], c["uc"] = b(c["w"]), b(c["uc"])
        c["uops"] = [b(u) for u in c["uops"]]
        if c["verified"] is not None:
            c["verified"] = (b(c["verified"][0]), [b(u) for u in c["verified"][1]])
        cands.append(c)
    return {"method": m, "records": recs, "candidates": cands}


def discrete(exp):
    if "refused" in exp:
        return exp
    return ([(r["id"], r["outcome"], r["hanger_id"], r["failure_counts"],
              None if r["verification"] is None else
              (r["verification"]["t_pass"], r["verification"]["failed_tests"]))
             for r in exp["candidates"]], exp["verification_standing"])


def check_intent(case, exp):
    intent = case["intent"]
    if intent == "refused":
        assert "refused" in exp, case["id"]
        return
    got = {r["id"]: (r["outcome"], r["hanger_id"]) for r in exp["candidates"]}
    assert got == intent, (case["id"], got, intent)


def F(*x):
    return Fraction(*x)


def check_stated_values(cases_by_id, exps):
    """Values stated in the task brief, recomputed here exactly."""
    def row(cid, i=0):
        return exps[cid]["candidates"][i]

    r = row("R3-01")
    assert r["f_cold_n"] == 1120 and r["f_cold_condition_n"] == 1100
    assert r["f_operating_n"] == [1000, 1200]
    assert r["envelope_n"] == [990, 1210] and r["required_travel_m"] == F("0.022")
    r = row("R3-02")
    assert r["f_cold_n"] == 1020 and r["f_cold_condition_n"] == 1000
    assert r["f_operating_n"] == [900, 1100] and r["envelope_n"] == [890, 1110]
    r = row("R3-05")
    assert r["f_cold_n"] == 880 and r["f_cold_condition_n"] == 900
    assert r["f_operating_n"] == [1000] and r["required_travel_m"] == F("0.012")
    r = row("R3-06")
    assert r["required_travel_m"] == F("0.014") and r["envelope_n"] == [870, 1010]
    a, b = row("R4B-01", 0), row("R4B-01", 1)
    assert a["f_cold_n"] == 120
    assert a["verification"]["variation"] == [F(1, 15)]
    assert a["verification"]["f_cold_condition_n"] == F(320, 3)
    assert a["verification"]["departure_n"] == 0
    assert a["verification"]["failed_tests"] == []
    assert b["verification"]["required_travel_m"] == F(26, 3000)
    assert b["verification"]["failed_tests"] == ["T"]


# ---------------------------------------------------------------- cases


def build_cases():
    cases = []

    def add(cid, desc, m, records, cands, intent):
        cases.append({"id": cid, "description": desc, "method": m,
                      "records": records, "candidates": cands, "intent": intent})

    # Shared simple candidate: W 1000 N, u_C 0, one operating case at +10 mm.
    # With rate k under hot design: F_cold = 1000 + 0.01k, F_D = 1000,
    # v = k/100000 (operating_load), envelope [1000, 1000 + 0.01k] (a = 0).
    def base(cid="H-1", policy="variable", w="1000", verified=None):
        return cand(cid, w, "0", ["0.010"], policy=policy, verified=verified)

    ok = vs("VS-OK", "10000", ("800", "1400"), "0.05")

    # Nonzero u_C, the brief's stated values.
    pos = dict(w="1000", uc="0.002", uops=["0.012", "-0.008"])
    rec10k = [vs("VS-10K", "10000", ("850", "1500"), "0.05")]
    add("R3-01", "Hot design, nonzero u_C = 2 mm, operating-load variation, allowance 1 mm. "
        "F_cold = W + k*u_D = 1120, F_C = 1100, F_D = 1000, F_2 = 1200, "
        "span [-8, 12] mm, required travel 22 mm, envelope [990, 1210] N.",
        method("hot", "operating_load", "0.25", "0.001"), rec10k,
        [cand("H-1", **pos)], {"H-1": ("selected", "VS-10K")})
    add("R3-02", "Cold design, nonzero u_C = 2 mm, design-load variation, allowance 1 mm. "
        "F_cold = W + k*u_C = 1020, F_C = 1000, F_D = 900, F_2 = 1100, envelope [890, 1110] N.",
        method("cold", "design_load", "0.25", "0.001"), rec10k,
        [cand("H-1", **pos)], {"H-1": ("selected", "VS-10K")})
    add("R3-03", "Hot design, nonzero u_C, design-load variation: both cases vary by "
        "k*10 mm / W = 0.1.",
        method("hot", "design_load", "0.25", "0.001"), rec10k,
        [cand("H-1", **pos)], {"H-1": ("selected", "VS-10K")})
    add("R3-04", "Cold design, nonzero u_C, operating-load variation: v = 100/900 and 100/1100.",
        method("cold", "operating_load", "0.25", "0.001"), rec10k,
        [cand("H-1", **pos)], {"H-1": ("selected", "VS-10K")})

    down = [vs("VS-10K", "10000", ("800", "1200"), "0.05")]
    add("R3-05", "Entirely downward travel, hot design, zero allowance: u_C = -2 mm, "
        "u_D = -12 mm. F_cold = 880, F_C = 900, F_D = 1000, span [-12, 0] mm, "
        "required travel 12 mm, envelope [880, 1000] N.",
        method("hot", "operating_load", "0.25", "0"), down,
        [cand("H-1", "1000", "-0.002", ["-0.012"])], {"H-1": ("selected", "VS-10K")})
    add("R3-06", "Entirely downward travel, hot design, allowance 1 mm: required travel "
        "14 mm, envelope [870, 1010] N.",
        method("hot", "operating_load", "0.25", "0.001"), down,
        [cand("H-1", "1000", "-0.002", ["-0.012"])], {"H-1": ("selected", "VS-10K")})
    add("R3-07", "Linear model (u_C = 0) under cold design, zero allowance: F_cold = W = 1500, "
        "F_1 = 1200, operating-load variation 0.25 below a limit of 0.3.",
        method("cold", "operating_load", "0.3", "0"),
        [vs("VS-15K", "15000", ("1000", "2000"), "0.03")],
        [cand("H-1", "1500", "0", ["0.020"])], {"H-1": ("selected", "VS-15K")})

    add("R3-08", "Multi-case governing, hot design on case 0 (+10 mm); case 1 (+25 mm) "
        "governs variation and the envelope's low end, case 2 (-5 mm) widens the span. "
        "VS-STIFF fails V on case 1, VS-NARROW fails L on case 1's end (1830 < 1900), "
        "VS-SHORT fails T (34 mm > 32 mm) only because of case 2; VS-FIT fits.",
        method("hot", "operating_load", "0.25", "0.002"),
        [vs("VS-STIFF", "20000", ("1000", "3000"), "0.05"),
         vs("VS-NARROW", "10000", ("1900", "2600"), "0.05"),
         vs("VS-SHORT", "10000", ("1800", "2400"), "0.032"),
         vs("VS-FIT", "10000", ("1800", "2400"), "0.05")],
        [cand("H-1", "2000", "0", ["0.010", "0.025", "-0.005"])],
        {"H-1": ("selected", "VS-FIT")})

    # Each test failing first.
    add("R3-09", "E1 first: a record with no stiffness, a library-1.0.0-style record with no "
        "working load range, and a record with no travel range all fail E1 (count 3); "
        "VS-OK fits.",
        method(),
        [vs("VS-NOK", None, ("800", "1400"), "0.05"),
         vs("VS-NORANGE", "10000", None, "0.05"),
         vs("VS-NOTRAVEL", "10000", ("800", "1400"), None), ok],
        [base()], {"H-1": ("selected", "VS-OK")})
    add("R3-10", "E2 first: VS-UX would otherwise fit but its stiffness DOF is UX, not the "
        "candidate's UZ; VS-OK fits.",
        method(), [vs("VS-UX", "10000", ("800", "1400"), "0.05", dof="UX"), ok],
        [base()], {"H-1": ("selected", "VS-OK")})
    add("R3-11", "V0 first through F_C <= 0: hot design, u_D = 0, u_C = +1/64 m. VS-6400 gives "
        "F_C = 100 - 6400/64 = 0 exactly (it would also fail L); VS-3200 gives F_C = 50 and fits.",
        method("hot", "operating_load", "0.6", "0"),
        [vs("VS-6400", "6400", ("10", "200"), "0.05"),
         vs("VS-3200", "3200", ("10", "200"), "0.05")],
        [cand("H-1", "100", "0.015625", ["0"])], {"H-1": ("selected", "VS-3200")})
    add("R3-12", "V0 first through F_i <= 0: cold design, u_C = 0, cases at +1/128 m (design) "
        "and +1/32 m. VS-6400 gives F_2 = 100 - 200 = -100; VS-1600 gives F_2 = 50 and fits "
        "with design-load variation 0.5.",
        method("cold", "design_load", "0.6", "0"),
        [vs("VS-6400", "6400", ("10", "200"), "0.05"),
         vs("VS-1600", "1600", ("10", "200"), "0.05")],
        [cand("H-1", "100", "0", ["0.0078125", "0.03125"])], {"H-1": ("selected", "VS-1600")})
    add("R3-13", "L first, low side: envelope [1000, 1100] N, VS-LLOW's F_min is 1001.",
        method(), [vs("VS-LLOW", "10000", ("1001", "1400"), "0.05"), ok],
        [base()], {"H-1": ("selected", "VS-OK")})
    add("R3-14", "L first, high side: envelope [1000, 1100] N, VS-LHIGH's F_max is 1099.",
        method(), [vs("VS-LHIGH", "10000", ("800", "1099"), "0.05"), ok],
        [base()], {"H-1": ("selected", "VS-OK")})
    add("R3-15", "T first: required travel 10 mm, VS-T's travel range is 9 mm.",
        method(), [vs("VS-T", "10000", ("800", "1400"), "0.009"), ok],
        [base()], {"H-1": ("selected", "VS-OK")})
    add("R3-16", "V first: VS-V (k = 30000) passes L and T but varies by 0.3 > 0.25.",
        method(), [vs("VS-V", "30000", ("800", "1400"), "0.05"), ok],
        [base()], {"H-1": ("selected", "VS-OK")})
    add("R3-17", "No fit under the variable policy: one record failing each variable test "
        "first (counts 1 each). Cases +10 mm (design) and +20 mm. The constant record is not "
        "tested and is not counted.",
        method(),
        [vs("VS-E1", "10000", None, "0.05"),
         vs("VS-E2", "10000", ("800", "1400"), "0.05", dof="UY"),
         vs("VS-V0", "200000", ("1", "1000000"), "1"),
         vs("VS-L", "10000", ("950", "1400"), "0.05"),
         vs("VS-T", "10000", ("800", "1400"), "0.015"),
         vs("VS-V", "20000", ("700", "1300"), "0.05"),
         ce("CE-1", ("500", "1500"), "0.05")],
        [cand("H-1", "1000", "0", ["0.010", "0.020"])],
        {"H-1": ("no_record_fits", None)})

    # Exact boundary equality (all binary64-exact).
    bnd = dict(w="1024", uc="0", uops=["0.0625"])
    add("R3-18", "L boundary passes: W = 1024, k = 8192, u_D = 1/16 m, a = 1/128 m, hot. "
        "Envelope [960, 1600] N equals VS-LEQ's range exactly; VS-LOVER (F_max 1599.5) fails L.",
        method("hot", "design_load", "0.75", "0.0078125"),
        [vs("VS-LOVER", "8192", ("960", "1599.5"), "0.125"),
         vs("VS-LEQ", "8192", ("960", "1600"), "0.125")],
        [cand("H-1", **bnd)], {"H-1": ("selected", "VS-LEQ")})
    add("R3-19", "T boundary passes: required travel 1/16 + 2/128 = 0.078125 m equals VS-TEQ's "
        "travel range; VS-TSHORT (0.078 m) fails T.",
        method("hot", "design_load", "0.75", "0.0078125"),
        [vs("VS-TSHORT", "8192", ("900", "1700"), "0.078"),
         vs("VS-TEQ", "8192", ("900", "1700"), "0.078125")],
        [cand("H-1", **bnd)], {"H-1": ("selected", "VS-TEQ")})
    add("R3-20", "V boundary passes: operating-load variation 512/1024 = 0.5 equals the limit; "
        "VS-8200 varies by 512.5/1024 and fails V.",
        method("hot", "operating_load", "0.5", "0.0078125"),
        [vs("VS-8200", "8200", ("900", "1700"), "0.125"),
         vs("VS-VEQ", "8192", ("900", "1700"), "0.125")],
        [cand("H-1", **bnd)], {"H-1": ("selected", "VS-VEQ")})
    add("R3-21", "Constant boundaries pass: W = 1024 equals CE-MIN's F_min (whose travel range "
        "equals the required 0.078125 m) and CE-MAX's F_max; both fit and smallest capacity "
        "picks CE-MAX. CE-OVER fails L, CE-SHORT fails T.",
        method("hot", "operating_load", "0.25", "0.0078125"),
        [ce("CE-MIN", ("1024", "2048"), "0.078125"),
         ce("CE-MAX", ("512", "1024"), "0.125"),
         ce("CE-OVER", ("1024.5", "2048"), "0.125"),
         ce("CE-SHORT", ("512", "2048"), "0.078")],
        [cand("H-1", policy="constant", **bnd)], {"H-1": ("selected", "CE-MAX")})

    # Selection rules and ties.
    add("R3-22", "smallest_capacity decided by F_max: VS-A (F_max 1300, v 0.2) beats "
        "VS-B (1500, v 0.05) and VS-C (2000).",
        method(rule="smallest_capacity"),
        [vs("VS-B", "5000", ("900", "1500"), "0.05"),
         vs("VS-A", "20000", ("900", "1300"), "0.05"),
         vs("VS-C", "10000", ("900", "2000"), "0.05")],
        [base()], {"H-1": ("selected", "VS-A")})
    add("R3-23", "smallest_capacity tie on F_max (1500) broken by max variation: VS-B "
        "(v 0.05) beats VS-A (0.2); VS-C has F_max 1600.",
        method(rule="smallest_capacity"),
        [vs("VS-A", "20000", ("900", "1500"), "0.05"),
         vs("VS-B", "5000", ("950", "1500"), "0.05"),
         vs("VS-C", "10000", ("800", "1600"), "0.05")],
        [base()], {"H-1": ("selected", "VS-B")})
    add("R3-24", "smallest_capacity tie on F_max and variation broken by hanger_id byte order: "
        "'VS-10' < 'VS-9' < 'vs-1'.",
        method(rule="smallest_capacity"),
        [vs("VS-9", "10000", ("900", "1500"), "0.05"),
         vs("vs-1", "10000", ("900", "1500"), "0.05"),
         vs("VS-10", "10000", ("950", "1500"), "0.04"),
         vs("VS-0", "10000", ("900", "1600"), "0.05")],
        [base()], {"H-1": ("selected", "VS-10")})
    add("R3-25", "least_variation decided by variation: VS-B (v 0.05) beats VS-C (smaller "
        "F_max 1200, v 0.1) and VS-A (v 0.2).",
        method(rule="least_variation"),
        [vs("VS-A", "20000", ("900", "1300"), "0.05"),
         vs("VS-B", "5000", ("900", "1500"), "0.05"),
         vs("VS-C", "10000", ("900", "1200"), "0.05")],
        [base()], {"H-1": ("selected", "VS-B")})
    add("R3-26", "least_variation tie on variation (k = 5000) broken by F_max: VS-B (1400) "
        "beats VS-A (1600).",
        method(rule="least_variation"),
        [vs("VS-A", "5000", ("900", "1600"), "0.05"),
         vs("VS-B", "5000", ("900", "1400"), "0.05"),
         vs("VS-C", "10000", ("900", "1200"), "0.05")],
        [base()], {"H-1": ("selected", "VS-B")})
    add("R3-27", "least_variation tie on variation and F_max broken by hanger_id byte order: "
        "'VS-B10' < 'VS-B2'.",
        method(rule="least_variation"),
        [vs("VS-B2", "5000", ("900", "1400"), "0.05"),
         vs("VS-B10", "5000", ("950", "1400"), "0.03"),
         vs("VS-C", "10000", ("900", "1200"), "0.05")],
        [base()], {"H-1": ("selected", "VS-B10")})
    add("R3-28", "Constant policy under least_variation: constant variation is 0, so the rule "
        "reduces to F_max then hanger_id; 'CE-10' < 'CE-2' at F_max 1500. The variable record "
        "is not tested.",
        method(rule="least_variation"),
        [ce("CE-2", ("500", "1500"), "0.05"),
         ce("CE-10", ("600", "1500"), "0.03"),
         ce("CE-3", ("500", "2000"), "0.05"),
         ok],
        [base(policy="constant")], {"H-1": ("selected", "CE-10")})

    # Family policy.
    add("R3-29", "Fallback taken: variable_else_constant, no variable record fits (T, V), so "
        "constants are tested; CE-SMALL fails L and CE-1 is selected by fallback.",
        method(),
        [vs("VS-T", "10000", ("800", "1400"), "0.009"),
         vs("VS-V", "30000", ("800", "1400"), "0.05"),
         ce("CE-SMALL", ("200", "900"), "0.05"),
         ce("CE-1", ("500", "1500"), "0.05")],
        [base(policy="variable_else_constant")],
        {"H-1": ("selected_by_fallback", "CE-1")})
    add("R3-30", "Fallback not needed: variable_else_constant, VS-OK fits, so constants are "
        "not tested and only variable counts are reported.",
        method(),
        [vs("VS-T", "10000", ("800", "1400"), "0.009"), ok,
         ce("CE-1", ("500", "1500"), "0.05")],
        [base(policy="variable_else_constant")], {"H-1": ("selected", "VS-OK")})
    add("R3-31", "No fit under the constant policy: CE-NORANGE fails E1, CE-SMALL fails L, "
        "CE-SHORT fails T.",
        method(),
        [ce("CE-NORANGE", None, "0.05"),
         ce("CE-SMALL", ("200", "900"), "0.05"),
         ce("CE-SHORT", ("500", "1500"), "0.005"), ok],
        [base(policy="constant")], {"H-1": ("no_record_fits", None)})
    add("R3-32", "No fit under variable_else_constant: VS-V fails V, the fallback is tried and "
        "CE-SMALL fails L; both families' counts are reported.",
        method(),
        [vs("VS-V", "30000", ("800", "1400"), "0.05"),
         ce("CE-SMALL", ("200", "900"), "0.05")],
        [base(policy="variable_else_constant")], {"H-1": ("no_record_fits", None)})

    add("R3B-01", "R-3b: two of three candidates have W <= 0 (H-3 at -250 N, H-2 at 0 N). "
        "The request is refused, naming them in input order; no candidate is sized.",
        method(), [ok],
        [cand("H-3", "-250", "0", ["0.010"]),
         cand("H-1", "1000", "0", ["0.010"]),
         cand("H-2", "0", "0", ["0.010"])], "refused")

    # Verification (R-4 / R-4b arithmetic).
    add("R4-01", "Verification of a variable selection, passing: cold design, design-load "
        "variation, allowance 1 mm. Sized F_cold = 2010; verified u_C^v = 0.5 mm, "
        "u^v = (11.5, 6.5) mm give F_C^v = 2005, departure +5 N, v^v = (0.055, 0.03).",
        method("cold", "design_load", "0.1", "0.001"),
        [vs("VS-R4", "10000", ("1800", "2200"), "0.03")],
        [cand("H-1", "2000", "0.001", ["0.011", "0.006"],
              verified=("0.0005", ["0.0115", "0.0065"]))],
        {"H-1": ("selected", "VS-R4")})
    add("R4-02", "Verification of a variable selection failing L and V: hot design, sized "
        "F_cold = 1100. Verified u_C^v = -2 mm, u_D^v = 16 mm give F_D^v = 940 (envelope low "
        "end 940 < 950) and v^v = 180/940 > 0.15; departure -60 N.",
        method("hot", "operating_load", "0.15", "0"),
        [vs("VS-R4F", "10000", ("950", "1150"), "0.05")],
        [cand("H-1", "1000", "0", ["0.010"], verified=("-0.002", ["0.016"]))],
        {"H-1": ("selected", "VS-R4F")})
    add("R4-03", "Verification of a variable selection failing V0, L and V with the "
        "design-load reference (so the variation stays defined): verified u_D^v = 120 mm "
        "gives F_D^v = -100 N; T passes.",
        method("hot", "design_load", "0.5", "0"),
        [vs("VS-R4Z", "10000", ("500", "1500"), "0.2")],
        [cand("H-1", "1000", "0", ["0.010"], verified=("0", ["0.120"]))],
        {"H-1": ("selected", "VS-R4Z")})
    add("R4B-01", "R-4b mixed verification in SI. Stiffness [[2, -1], [-1, 1]] N/mm. A is "
        "variable (k = 2 N/mm = 2000 N/m, hot setting 120 N, W = 100 N); B is constant "
        "(W = 100 N, 5 mm travel range). Sizing positions u_C = (0, 0), u_D = (10, 0) mm; "
        "verified u_C^v = (20/3, 20/3) mm, u_D^v = (10, 0) mm; a = 1 mm. A: F_C^v = 320/3 N, "
        "v^v = 1/15 (6.667 %), departure 0, passes. B: verified required travel 26/3 mm "
        "> 5 mm, fails T.",
        method("hot", "operating_load", "0.25", "0.001"),
        [vs("VS-A2", "2000", ("50", "200"), "0.03"),
         ce("CE-B5", ("50", "200"), "0.005")],
        [cand("A", "100", "0", ["0.010"], verified=(Fraction(1, 150), ["0.010"])),
         cand("B", "100", "0", ["0"], policy="constant",
              verified=(Fraction(1, 150), ["0"]))],
        {"A": ("selected", "VS-A2"), "B": ("selected", "CE-B5")})
    add("R4-04", "Verification standing conditional: H-HEAVY (W = 5000 N) has no fitting "
        "record and gets no verification block; H-OK is verified and passes.",
        method(),
        [ok],
        [base("H-OK", verified=("0.0001", ["0.0105"])),
         base("H-HEAVY", w="5000", verified=("0.0002", ["0.011"]))],
        {"H-OK": ("selected", "VS-OK"), "H-HEAVY": ("no_record_fits", None)})
    return cases


# ---------------------------------------------------------------- emission


def jnum(x):
    if isinstance(x, bool) or x is None or isinstance(x, (str, int)):
        return x
    if isinstance(x, Fraction):
        return float(x)  # int/int true division: correctly rounded binary64
    if isinstance(x, (list, tuple)):
        return [jnum(v) for v in x]
    if isinstance(x, dict):
        return {k: jnum(v) for k, v in x.items()}
    raise TypeError(type(x))


def emit_case(case, exp):
    m = case["method"]
    return {
        "id": case["id"],
        "description": case["description"],
        "method": {
            "load_design": m["load_design"],
            "variation_reference": m["variation_reference"],
            "variation_limit": float(m["variation_limit"]),
            "travel_allowance_m": float(m["travel_allowance_m"]),
            "selection_rule": m["selection_rule"],
        },
        "records": [{
            "hanger_id": r["hanger_id"],
            "hanger_type": r["hanger_type"],
            "stiffness_n_per_m": jnum(r["k"]),
            "stiffness_dof": r["dof"],
            "working_load_range_n": jnum(r["range"]),
            "travel_range_m": jnum(r["travel"]),
        } for r in case["records"]],
        "candidates": [{
            "id": c["id"],
            "dof": c["dof"],
            "family_policy": c["policy"],
            "w_n": float(c["w"]),
            "u_cold_m": float(c["uc"]),
            "u_operating_m": jnum(c["uops"]),
            "design_index": c["d"],
            "verified": None if c["verified"] is None else {
                "u_cold_m": float(c["verified"][0]),
                "u_operating_m": jnum(c["verified"][1]),
            },
        } for c in case["candidates"]],
        "expected": jnum(exp),
    }


def build_document():
    cases = build_cases()
    ids = [c["id"] for c in cases]
    assert len(ids) == len(set(ids)), "duplicate case id"
    exps = {}
    for case in cases:
        exp = expected(case)
        if discrete(exp) != discrete(expected(binarized(case))):
            raise AssertionError(f"{case['id']}: discrete outcome depends on input rounding")
        check_intent(case, exp)
        exps[case["id"]] = exp
    check_stated_values({c["id"]: c for c in cases}, exps)
    return {"schema": SCHEMA, "basis": BASIS,
            "cases": [emit_case(c, exps[c["id"]]) for c in cases]}


def render():
    return json.dumps(build_document(), indent=2) + "\n"


def main(argv=None):
    p = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    p.add_argument("--check", action="store_true",
                   help="exit 1 if the frozen JSON differs from the generated output")
    args = p.parse_args(argv)
    text = render()
    if args.check:
        try:
            frozen = OUT.read_text(encoding="utf-8")
        except FileNotFoundError:
            print(f"missing: {OUT}", file=sys.stderr)
            return 1
        if frozen != text:
            print(f"differs: {OUT}", file=sys.stderr)
            return 1
        print(f"ok: {OUT.name} matches ({len(json.loads(text)['cases'])} cases)")
        return 0
    OUT.write_text(text, encoding="utf-8")
    print(f"wrote {OUT}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
