"""Independent exact references for operating-case slice 1b (DEL-05-02 Work
operating-case.md section 1.5, V-1 and V-2) and the G-1 containment verdicts of
section 1.2 / model document 0.9.0 section 2.2.

Standard library only; reads, imports and runs no product code. Values are exact
rationals (fractions.Fraction); each published expectation is written as the
binary64 nearest to it (every one here is a dyadic rational, so exactly
representable) in float.hex() form.

Run ``python3 -I generators/op1b_reference.py`` from this directory to compare the
outputs with the committed files; ``--write`` rewrites them.
"""
from __future__ import annotations

import hashlib
import json
import sys
from collections import Counter
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent.parent

# V-1: an axial bar along X anchored at x = 0, L = 4 m, k = EA/L = 2^20 N/m,
# alpha = 2^-16 /K, a tip gap on UX closing in +X at clearance c. The operating case
# adds dT = 16 K, so its free growth is d_T = alpha dT L = 2^-10 m.
K = F(2) ** 20
D_T = F(1, 2 ** 16) * 16 * 4


def bar(c: F, force: F, thermal: bool) -> dict:
    """Tip displacement u (m), axial force N (N, tension +) and the stop's Fx on the pipe."""
    growth = D_T if thermal else F(0)
    free = growth + force / K
    if free <= c:  # the gap stays open (the weak boundary u = c is open-equivalent: R = 0)
        return {"u": free, "N": force, "stop_fx": F(0)}
    n = K * (c - growth)
    return {"u": c, "N": n, "stop_fx": n - force}


def v1_cases() -> list[dict]:
    out = []
    for name, c, force in [("closing", F(1, 2 ** 9), F(1536)), ("both_open", F(1, 2 ** 8), F(1536)), ("both_active", F(1, 2 ** 9), F(2560))]:
        s, o = bar(c, force, False), bar(c, force, True)
        out.append({"name": name, "c": c, "F": force, "sustained": s, "operating": o,
                    "difference": {k: o[k] - s[k] for k in s}})
    return out


def v2_cases() -> list[dict]:
    """V-2: the W-16 A-1 propped cantilever (P at midspan, one-way prop at the tip),
    in units of P, PL and s* = 5PL^3/(48EI) = 2^-10 m: the declared difference of an
    anchor rise s and the sustained case (prop active, R = 5P/16, M_A = -3PL/16)."""
    s_star = F(1, 2 ** 10)
    return [
        {"name": "lift", "rise": 2 * s_star, "operating": {"u_tip": s_star, "R_over_P": F(0), "My_over_PL": F(-1, 2)},
         "difference": {"u_tip": s_star, "R_over_P": F(-5, 16), "My_over_PL": F(-5, 16)}},
        {"name": "active", "rise": s_star / 2, "operating": {"u_tip": F(0), "R_over_P": F(5, 32), "My_over_PL": F(-11, 32)},
         "difference": {"u_tip": F(0), "R_over_P": F(-5, 32), "My_over_PL": F(-5, 32)}},
    ]


# G-1 verdicts, by an independent multiset reading of section 2.2: a primitive's content
# is (category, target, direction, dimension, value bits, unit); ids are ignored; the
# subtrahend's count of each content must not exceed the minuend's.
FORCE = ("concentrated_force", ("node", "node:b"), "global_x", "force", F(1536), "N")
THERMAL = ("thermal", ("element", "pipe:a-b"), "global_x", "temperature_interval", F(16), "K")
WEIGHT = ("weight", ("node", "node:b"), "global_z", "force", F(-100), "N")


def next_up(value: F) -> F:
    """The next binary64 above a positive normal dyadic value (one ulp)."""
    exponent = value.numerator.bit_length() - value.denominator.bit_length()
    while F(2) ** exponent > value:
        exponent -= 1
    while F(2) ** (exponent + 1) <= value:
        exponent += 1
    return value + F(2) ** (exponent - 52)


def g1(minuend: list, subtrahend: list) -> bool:
    m, s = Counter(minuend), Counter(subtrahend)
    return all(m[key] >= count for key, count in s.items())


def g1_cases() -> list[dict]:
    ulp = (FORCE[0], FORCE[1], FORCE[2], FORCE[3], next_up(FORCE[4]), FORCE[5])
    rows = [
        ("contained", [FORCE, THERMAL], [FORCE], True),
        ("sustained_only_load", [FORCE, THERMAL], [FORCE, WEIGHT], False),
        ("one_ulp", [FORCE, THERMAL], [ulp], False),
        ("two_against_one", [FORCE, THERMAL], [FORCE, FORCE], False),
        ("two_against_two", [FORCE, FORCE, THERMAL], [FORCE, FORCE], True),
        ("swapped", [FORCE], [FORCE, THERMAL], False),
        ("empty_subtrahend", [FORCE], [], True),
    ]
    return [{"name": n, "minuend": len(m), "subtrahend": len(s), "contained": g1(m, s),
             "one_ulp_value": ulp[4] if n == "one_ulp" else None} for n, m, s, expect in rows if g1(m, s) == expect]


def hexed(value):
    if isinstance(value, F):
        f = float(value)
        assert F(f) == value, f"{value} is not a binary64"
        return f.hex()
    if isinstance(value, dict):
        return {k: hexed(v) for k, v in value.items()}
    if isinstance(value, list):
        return [hexed(v) for v in value]
    return value


def build() -> dict[str, bytes]:
    cases = g1_cases()
    assert len(cases) == 7, "every G-1 row's verdict is its stated one"
    references = {"v1": hexed(v1_cases()), "v2": hexed(v2_cases()), "g1": hexed(cases)}
    text = (json.dumps(references, indent=2, sort_keys=True) + "\n").encode()
    sums = "".join(f"{hashlib.sha256(data).hexdigest()}  {name}\n" for name, data in [
        ("generators/op1b_reference.py", (HERE / "generators" / "op1b_reference.py").read_bytes()),
        ("references.json", text)])
    return {"references.json": text, "SHA256SUMS": sums.encode()}


def main() -> int:
    outputs = build()
    if "--write" in sys.argv[1:]:
        for name, data in outputs.items():
            (HERE / name).write_bytes(data)
        return 0
    stale = [name for name, data in outputs.items() if not (HERE / name).exists() or (HERE / name).read_bytes() != data]
    print("stale: " + ", ".join(stale) if stale else "references are current")
    return 1 if stale else 0


if __name__ == "__main__":
    raise SystemExit(main())
