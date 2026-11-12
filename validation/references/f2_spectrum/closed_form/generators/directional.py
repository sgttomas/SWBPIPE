"""Item 4: directional combination (DEL-04-10 REQ-003; AC-002 '100-40-40 and SRSS').

Given the directional magnitudes (R_x, R_y, R_z) of one response quantity:
  SRSS        sqrt(R_x^2 + R_y^2 + R_z^2)
  100-40-40   the largest of R_a + (2/5) R_b + (2/5) R_c over the three
              permutations (which direction carries the full value). Because
              R_a + (2/5)(R_b + R_c) = (3/5) R_a + (2/5)(R_x + R_y + R_z), the
              governing permutation is the one whose full direction has the
              largest magnitude; equal largest magnitudes tie exactly.
  ABSSUM      R_x + R_y + R_z (not in the scope today; for comparison only)
The 40 % factor is taken as the exact rational 2/5. A product that uses the
binary64 0.4 (= 3602879701896397/2^53, larger than 2/5 by 2.2e-17 relative)
differs from these values by less than 1e-16 relative.
"""

from decimal import Decimal
from fractions import Fraction

from numerics import dec, dsqrt, inp, num, rel_diff, sci, val

FORTY = Fraction(2, 5)
CASES = [
    ("D1", (1.25, 0.5, 0.3), "x governs"),
    ("D2", (0.5, 1.25, 0.3), "y governs"),
    ("D3", (0.3, 0.5, 1.25), "z governs"),
    ("D4", (1.0, 1.0, 0.25), "x and y tie"),
    ("D5", (0.75, 0.75, 0.75), "all three tie"),
    ("D6", (0.6, 0.9, 0.0), "one direction with zero magnitude; y governs"),
    ("D7", (0.8, 0.35, None), "two directions requested (z not requested; read as absent, equivalently 0)"),
]
NAMES = ["x", "y", "z"]


def build(cross):
    out = []
    worst = Decimal(0)
    tie_ok = True
    for cid, mags, note in CASES:
        R = [Fraction(m) if m is not None else None for m in mags]
        present = [i for i in range(3) if R[i] is not None]
        Rv = [R[i] if R[i] is not None else Fraction(0) for i in range(3)]
        perms = {}
        for a in present:
            others = [b for b in present if b != a]
            perms[f"full_{NAMES[a]}"] = Rv[a] + FORTY * sum((Rv[b] for b in others), Fraction(0))
        best = max(perms.values())
        gov = [k for k, v in perms.items() if v == best]
        # cross-check: the closed form (3/5) max + (2/5) sum
        alt = Fraction(3, 5) * max(Rv[i] for i in present) + FORTY * sum(Rv[i] for i in present)
        tie_ok &= alt == best
        sq = sum(Rv[i] ** 2 for i in present)
        srss = dsqrt(dec(sq))
        worst = max(worst, rel_diff(srss * srss, dec(sq)))
        out.append({
            "id": cid, "note": note,
            "directional_magnitudes": {NAMES[i]: (inp(mags[i]) if mags[i] is not None else "not requested")
                                       for i in range(3)},
            "SRSS": val(srss), "SRSS_squared_exact": num(sq),
            "100_40_40_permutations": {k: num(v) for k, v in perms.items()},
            "100_40_40": num(best), "governing_permutations": gov,
            "ABSSUM_not_in_scope": num(sum(Rv[i] for i in present)),
        })
    cross.append({"id": "DIR-100-40-40-closed-form",
                  "what": "max over permutations equals (3/5) max + (2/5) sum, exactly", "agrees": tie_ok})
    cross.append({"id": "DIR-SRSS-square", "what": "SRSS squared against the exact rational sum of squares",
                  "max_rel_diff": sci(worst), "agrees": worst < Decimal("1e-85")})
    return {"item": "4. Directional combination (SRSS and 100-40-40; absolute sum for comparison only)",
            "factor_40_percent": num(FORTY), "cases": out}
