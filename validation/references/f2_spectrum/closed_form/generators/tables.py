"""Item 1: spectrum table evaluation (DEL-04-10 REQ-001, REQ-002; AC-001, AC-002).

Two invented tables, each under the four interpolation rules:
  T1  abscissa frequency (Hz), ordinate acceleration (m/s^2)
  T2  abscissa period (s), ordinate multiple of a user-entered g
Queries are frequencies (binary64). Each is evaluated at the tabulated
points, strictly inside every segment, at both range ends, and outside the
range (expected outcome: refused, no extrapolation). The scale factor s
multiplies the evaluated ordinate.
"""

from decimal import Decimal
from fractions import Fraction

from numerics import dec, inp, num, rel_diff, sci
import rules

RULES = [("linear", "linear"), ("log", "log"), ("linear", "log"), ("log", "linear")]
SCALE = 1.5

T1 = {"id": "T1", "abscissa": "frequency", "ordinate": "acceleration",
      "abscissae": [0.3, 1.5, 6.25, 25.0, 80.0], "ordinates": [0.9, 3.6, 9.81, 7.5, 2.2],
      "damping": 0.05}
T2 = {"id": "T2", "abscissa": "period", "ordinate": "g", "g": 9.80665,
      "abscissae": [0.03125, 0.125, 0.5, 2.0, 4.0], "ordinates": [0.35, 1.1, 0.8, 0.25, 0.1],
      "damping": 0.05}

QUERIES = {
    "T1": {"tabulated": [0.3, 1.5, 6.25, 25.0, 80.0], "inside": [0.7, 3.0, 10.0, 47.5],
           "outside": [0.29, 80.5]},
    # The tabulated periods are powers of two, so 1/T is exact in binary64 and
    # the frequencies 32, 8, 2, 0.5, 0.25 land exactly on the tabulated points.
    "T2": {"tabulated": [32.0, 8.0, 2.0, 0.5, 0.25], "inside": [20.0, 5.0, 1.2, 0.3],
           "outside": [33.0, 0.2]},
}


def variant(table, rule):
    t = dict(table)
    t["rule"] = rule
    return t


def rule_name(rule):
    return f"{rule[0]}-abscissa/{rule[1]}-ordinate"


def build(cross):
    out_tables = []
    worst_alt = Decimal(0)
    worst_exact = Decimal(0)
    n_eval = 0
    for base in (T1, T2):
        tdoc = rules.table_doc(dict(base, rule=("(per rule)", "(per rule)")))
        del tdoc["interpolation"]
        rows_by_rule = []
        for rule in RULES:
            tab = variant(base, rule)
            readings = ["tabulated"]
            if base["abscissa"] == "period" and rule[0] == "linear":
                readings.append("frequency")
            for reading in readings:
                rows = []
                for where in ("tabulated", "inside", "outside"):
                    for q in QUERIES[base["id"]][where]:
                        f = Fraction(q)
                        S = rules.evaluate(tab, f, reading)
                        row = {"query_frequency_hz": inp(q), "where": where}
                        if base["abscissa"] == "period":
                            row["query_period_s"] = num(1 / f)
                        if S is None:
                            assert where == "outside"
                            row["expected"] = "refused, no extrapolation (outside the tabulated range)"
                            rows.append(row)
                            continue
                        assert where != "outside"
                        n_eval += 1
                        Salt = rules.evaluate(tab, f, reading, alt=True)
                        worst_alt = max(worst_alt, rel_diff(dec(S) if isinstance(S, Fraction) else S,
                                                            dec(Salt) if isinstance(Salt, Fraction) else Salt))
                        if isinstance(S, Fraction):
                            assert S == Salt
                        if base["ordinate"] == "g":
                            row["S_g_multiple"] = num(S / Fraction(base["g"]) if isinstance(S, Fraction) else S / dec(base["g"]))
                        row["S_m_per_s2"] = num(S)
                        sS = S * Fraction(SCALE) if isinstance(S, Fraction) else S * dec(SCALE)
                        row["sS_m_per_s2"] = num(sS)
                        row["exact"] = isinstance(S, Fraction)
                        rows.append(row)
                entry = {"rule": rule_name(rule), "abscissa_rule": rule[0], "ordinate_rule": rule[1],
                         "rows": rows}
                if base["abscissa"] == "period":
                    entry["reading"] = {
                        "tabulated": "interpolated on the tabulated axis, period T = 1/f (primary reading)",
                        "frequency": ("tabulated periods converted to frequencies 1/T, then interpolated "
                                      "linearly in f (alternative reading; see README, ambiguity A1)")}[reading]
                    if rule[0] == "log":
                        entry["reading"] += "; a logarithmic abscissa gives the same value on either axis"
                rows_by_rule.append(entry)
        tdoc["evaluations"] = rows_by_rule
        out_tables.append(tdoc)
        # g-multiple versus converted-first: identical under every rule (scaling commutes)
        if base["ordinate"] == "g":
            worst = Decimal(0)
            for rule in RULES:
                for q in QUERIES["T2"]["inside"]:
                    f = Fraction(q)
                    a = rules.evaluate(variant(base, rule), f)
                    # convert ordinates first, exactly, then interpolate
                    tab2 = {"abscissa": "period", "ordinate": "acceleration", "rule": rule,
                            "abscissae": base["abscissae"],
                            "ordinates": [Fraction(v) * Fraction(base["g"]) for v in base["ordinates"]]}
                    b = rules.evaluate(tab2, f)
                    worst = max(worst, rel_diff(dec(a) if isinstance(a, Fraction) else a,
                                                dec(b) if isinstance(b, Fraction) else b))
            cross.append({"id": "T2-g-scaling-commutes",
                          "what": "g multiple interpolated then times g, against ordinates converted to m/s^2 first",
                          "max_rel_diff": sci(worst), "agrees": worst < Decimal("1e-80")})
    cross.append({"id": "TABLE-other-end",
                  "what": f"each of {n_eval} evaluations recomputed from the segment's other end",
                  "max_rel_diff": sci(worst_alt), "agrees": worst_alt < Decimal("1e-80")})
    # Rule identity at tabulated points and monotone segment bounds
    bad = 0
    for base in (T1, T2):
        for rule in RULES:
            for q in QUERIES[base["id"]]["inside"]:
                f = Fraction(q)
                S = rules.evaluate(variant(base, rule), f)
                S = dec(S) if isinstance(S, Fraction) else S
                xs, ys, _ = rules.table_points(variant(base, rule))
                gf = dec(Fraction(base.get("g", 1.0)) if base["ordinate"] == "g" else 1)
                x = rules._to_x(f, base["abscissa"])
                i = rules._segment(xs, x)
                lo, hi = sorted((dec(ys[i]) * gf, dec(ys[i + 1]) * gf))
                bad += not (lo < S < hi)
    cross.append({"id": "TABLE-inside-bounds",
                  "what": "every strictly interior value lies strictly between its segment's ordinates",
                  "violations": bad, "agrees": bad == 0})
    return {
        "item": "1. Spectrum table evaluation (DEL-04-10 REQ-001, REQ-002; AC-002 'each interpolation rule')",
        "scale_factor_s": inp(SCALE),
        "units": "frequency Hz, period s, acceleration m/s^2",
        "tables": out_tables,
    }
