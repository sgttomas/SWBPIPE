"""Independent same-represented-input oracle for DEL-04-10 unit S4 (the combinations).

It computes, without reading or importing any product code, the exact value of each combination
rule (DEL-04-10 CLM-001) on the represented inputs: the binary64 contributions, rigid responses,
frequencies and damping ratio exactly as the product receives them. Cluster sums are exact
(Fractions of the binary64 values); rho, square roots and the 100-40-40 factor 2/5 are evaluated in
Decimal at a precision far above the 1e-9 comparison (120 digits, 450 for the strongly cancelling
witnesses), and each result is written with 30 significant digits.

It is a check of S4's arithmetic only. It does not replace the end-to-end comparison with the
frozen references (`f2_combinations.rs`), whose model values stay the analytic criterion.

Inputs: the frozen references under validation/references/f2_spectrum (read only, as data), and
the review witnesses defined below. Output: represented_input_oracle.json next to this file.

Run from the repository root:
    python3 core/analysis/response_spectrum/tests/oracle/represented_input_oracle.py
"""

import decimal
import json
import math
from decimal import Decimal
from fractions import Fraction
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[4]
DISCRETE = ROOT / "validation/references/f2_spectrum/discrete"
CLOSED = ROOT / "validation/references/f2_spectrum/closed_form"
OUT = HERE / "represented_input_oracle.json"

FROZEN_DIGITS = 120
WITNESS_DIGITS = 450


def dec(x):
    """A Fraction or float as a Decimal at the current precision."""
    if isinstance(x, float):
        x = Fraction(x)
    return Decimal(x.numerator) / Decimal(x.denominator)


def rho(fi, fj, zeta):
    """CLM-001: 8z^2(1+r)r^(3/2) / ((1-r^2)^2 + 4z^2 r (1+r)^2), r = fj/fi, on represented operands."""
    r = dec(Fraction(fj) / Fraction(fi))
    z = dec(Fraction(zeta))
    return 8 * z * z * (1 + r) * r * r.sqrt() / ((1 - r * r) ** 2 + 4 * z * z * r * (1 + r) ** 2)


def within_ten_percent(lower, upper):
    return Fraction(upper) - Fraction(lower) <= Fraction(lower) / 10


def modal(rule, sums, freqs, zeta):
    """One row, one direction: sums are exact Fractions per periodic cluster, freqs the clusters'
    lowest frequencies in ascending order."""
    if rule == "ABSOLUTE_SUM":
        return dec(sum((abs(s) for s in sums), Fraction(0)))
    squares = sum((s * s for s in sums), Fraction(0))
    if rule == "SRSS":
        return dec(squares).sqrt()
    if rule == "CQC":
        q = dec(squares)
        for i in range(len(sums)):
            for j in range(i + 1, len(sums)):
                q += 2 * rho(freqs[i], freqs[j], zeta) * dec(sums[i] * sums[j])
        if q < 0:
            raise ValueError("negative radicand")
        return q.sqrt()
    pairs = []
    if rule == "GROUPING":
        group, low = [], None
        groups = []
        for i, f in enumerate(freqs):
            if group and within_ten_percent(low, f):
                group.append(i)
            else:
                if group:
                    groups.append(group)
                group, low = [i], f
        if group:
            groups.append(group)
        pairs = [(g[a], g[b]) for g in groups for a in range(len(g)) for b in range(a + 1, len(g))]
    elif rule == "TEN_PERCENT":
        pairs = [(i, j) for i in range(len(freqs)) for j in range(i + 1, len(freqs)) if within_ten_percent(freqs[i], freqs[j])]
    else:
        raise ValueError(rule)
    total = squares + sum((2 * abs(sums[i] * sums[j]) for i, j in pairs), Fraction(0))
    return dec(total).sqrt()


def with_rigid(rule, periodic, rigid):
    if rule is None:
        return periodic
    r = dec(Fraction(rigid))
    return (periodic * periodic + r * r).sqrt() if rule == "SRSS" else periodic + abs(r)


def directional(rule, parts):
    if rule == "SRSS":
        return sum((p * p for p in parts), Decimal(0)).sqrt()
    if rule == "ABSOLUTE_SUM":
        return sum(parts, Decimal(0))
    if rule == "100-40-40":
        return (3 * max(parts) + 2 * sum(parts, Decimal(0))) / 5
    raise ValueError(rule)


def text(v):
    return format(v, ".29e") if v != 0 else "0"


def combine(choice, inputs):
    """inputs: directions, units (list of (cluster, [modes], lowest f)), contributions[(d, mode)] ->
    values per row, rigid[d] -> values per row or None, zeta, rows."""
    modal_rule, rigid_rule, directional_rule = choice
    out = []
    for r in range(inputs["rows"]):
        parts = []
        for d in inputs["directions"]:
            sums = [sum((Fraction(inputs["contributions"][(d, m)][r]) for m in modes), Fraction(0))
                    for (_, modes, _) in inputs["units"](rigid_rule)]
            freqs = [f for (_, _, f) in inputs["units"](rigid_rule)]
            p = modal(modal_rule, sums, freqs, inputs["zeta"])
            parts.append(with_rigid(rigid_rule, p, inputs["rigid"][d][r] if rigid_rule else 0.0))
        out.append(directional(directional_rule, parts))
    return out


def discrete_inputs(case, analysis):
    request = analysis["request"]
    spectra = {s["id"]: s for s in case["spectra"]}
    zeta = spectra[request["directions"][0]["spectrum"]]["damping_ratio"]
    directions = [d["direction"] for d in request["directions"]]
    scope = case["modal_basis"]["scope_clusters"]
    cluster_of = {m: i + 1 for i, c in enumerate(scope) for m in c["modes"]}
    modes = [(m["mode"], m["frequency_Hz"][1], m["status"]) for m in analysis["mode_status"]]
    keys = None
    contributions = {}
    for d, entries in analysis["contributions_signed"].items():
        for e in entries:
            if e.get("values") is None:
                continue
            if keys is None:
                keys = list(e["values"].keys())
            contributions[(d, e["mode"])] = [e["values"][k][1] for k in keys]
    rigid = {d: [v[k][1] for k in keys] for d, v in (analysis.get("rigid_response_signed") or {}).items()}

    def units(rigid_rule):
        periodic = [m for m in modes if (rigid_rule is None or m[2] == "periodic")]
        grouped = {}
        for mode, f, _ in periodic:
            grouped.setdefault(cluster_of[mode], []).append((mode, f))
        return [(c, [m for m, _ in ms], min(f for _, f in ms)) for c, ms in sorted(grouped.items())]

    return keys, {"directions": directions, "units": units, "contributions": contributions, "rigid": rigid,
                  "zeta": zeta, "rows": len(keys)}


def rigid_choice(name):
    return None if name in (None, "OFF") else name


def frozen():
    decimal.getcontext().prec = FROZEN_DIGITS
    cases = []
    for name, analyses in [("c4_close_pair.json", None), ("c10_cluster_witnesses.json", None), ("c9_pipe_diagonal.json", ["C9-CHAIN-Y"])]:
        doc = json.loads((DISCRETE / name).read_text())
        for case in doc["cases"]:
            for analysis in case["analyses"]:
                rid = analysis["request"]["id"]
                if analyses and rid not in analyses:
                    continue
                keys, inputs = discrete_inputs(case, analysis)
                request = analysis["request"]
                choices = [(request["modal_combination"], rigid_choice(request["missing_mass"]["rigid_periodic_combination"]),
                            request["directional_combination"])]
                for m in analysis.get("mutations", []):
                    choices.append((m["modal_combination"], rigid_choice(m["missing_mass"]), m["directional_combination"]))
                results = []
                for choice in choices:
                    values = combine(choice, inputs)
                    results.append({"modal_combination": choice[0], "missing_mass": choice[1] or "OFF",
                                    "directional_combination": choice[2], "combined": dict(zip(keys, map(text, values)))})
                cases.append({"source": f"validation/references/f2_spectrum/discrete/{name}", "analysis": rid, "results": results})
    return cases


def binomial(n, zeta, scale=1.0):
    freqs = [1.0 + i * 2.0 ** -19 for i in range(n)]
    values = [(-1) ** k * math.comb(n - 1, k) * scale for k in range(n)]
    return freqs, values, zeta


def witnesses():
    decimal.getcontext().prec = WITNESS_DIGITS
    out = []

    def single(wid, rule, freqs, clusters, values, zeta, note):
        units = {}
        for f, c, v in zip(freqs, clusters, values):
            units.setdefault(c, []).append((f, v))
        sums = [sum((Fraction(v) for _, v in u), Fraction(0)) for _, u in sorted(units.items())]
        lows = [min(f for f, _ in u) for _, u in sorted(units.items())]
        value = modal(rule, sums, lows, zeta)
        out.append({"id": wid, "note": note, "modal_combination": rule, "damping_ratio": zeta, "frequencies_hz": freqs,
                    "clusters": clusters, "contributions": values, "combined": text(value)})

    for rule in ["SRSS", "CQC", "GROUPING", "TEN_PERCENT", "ABSOLUTE_SUM"]:
        single(f"cluster_cancellation_{rule}", rule, [1.0, 1.0, 1.0], [1, 1, 1], [1e16, 1.0, -1e16], 0.05,
               "review finding 2: one cluster, contributions 1e16, 1, -1e16; exact sum 1")
        single(f"cluster_intermediate_overflow_{rule}", rule, [1.0, 1.0, 1.0], [1, 1, 1], [1e308, 1e308, -1e308], 0.05,
               "an exact cluster sum whose running binary64 sum would overflow")
    for zeta in [0.01, 0.05, 0.2]:
        for step in [5e-7, 1e-6, 2e-6, 1e-5, 1e-4, 1e-3, 1e-2]:
            freqs = [1.0, 1.0 + step, 1.0 + 2 * step]
            single(f"cqc_second_difference_z{zeta}_step{step}", "CQC", freqs, [1, 2, 3], [1.0, -2.0, 1.0], zeta,
                   "review: CQC second difference")
    single("cqc_scaled_second_difference", "CQC", [1.0, 1.00001, 1.00002], [1, 2, 3], [2.0 ** 30, -(2.0 ** 31), 2.0 ** 30], 0.2,
           "review finding 1: exact 3.32127331762412677799")
    for step in [5e-7, 1e-6, 1e-5]:
        single(f"cqc_pair_step{step}", "CQC", [1.0, 1.0 + step], [1, 2], [1.0, -1.0], 0.2, "review: an opposite-sign close pair")
    for wid, n, zeta, scale in [("R1_n4_z0.05", 4, 0.05, 1.0), ("R1_n3_z0.2", 3, 0.2, 1.0), ("R2_n7_z0.2", 7, 0.2, 1.0),
                                ("R2_n8_z0.05", 8, 0.05, 1.0), ("R2_n7_z0.2_scaled_2^90", 7, 0.2, 2.0 ** 90),
                                ("demanding_n20_z0.2", 20, 0.2, 1.0)]:
        freqs, values, z = binomial(n, zeta, scale)
        single(wid, "CQC", freqs, list(range(1, n + 1)), values, z,
               "evenly spaced alternating binomial: f_i = 1 + i*2^-19 Hz, R_k = (-1)^k C(n-1,k) N times the scale")
    for wid, parts in [("hundred_forty_forty_normal_small", [1e-200, 2e-200, 3e-200]),
                       ("hundred_forty_forty_normal_large", [1e200, 2e200, 3e200])]:
        value = directional("100-40-40", [dec(Fraction(p)) for p in parts])
        out.append({"id": wid, "note": "100-40-40 with the exact 2/5 over X, Y, Z magnitudes", "directional_magnitudes": parts,
                    "combined": text(value)})
    return out


def main():
    doc = {
        "document": "openpipestress.response_spectrum.s4_represented_input_oracle",
        "statement": "Exact combinations of the represented binary64 inputs, computed independently of product code "
                     "(see the generator's docstring). A check of S4 arithmetic; not a model reference.",
        "generator": "core/analysis/response_spectrum/tests/oracle/represented_input_oracle.py",
        "digits": {"frozen": FROZEN_DIGITS, "witnesses": WITNESS_DIGITS},
        "frozen": frozen(),
        "witnesses": witnesses(),
    }
    OUT.write_text(json.dumps(doc, indent=1, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
