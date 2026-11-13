"""The rule for tranche 3's designed values (DEL-17-05; the CAEPIPE benchmark generator's S1, S2 and S3 series and its
part-two hand-entry cases).

What the rule states, and the tests check:

- Every designed outside diameter is a whole multiple of 10 mm.
- Each wall thickness is that outside diameter divided by a stated diameter-to-thickness ratio, exactly.
- The material constants are the stated round values of DESIGNED_MATERIAL.

Lengths, temperature changes and forces are round values stated in each series' text (a span of 1.2 m, a temperature
change of 150 K, a tip force of 200 kN); a weight is computed from the section and the material (-rho A g).
"""

from __future__ import annotations

# (id, E Pa, Poisson ratio, expansion per K, density kg/m3): an invented material with stated round values.
DESIGNED_MATERIAL = ("material:bench-t3", 210.0e9, 0.29, 11.0e-6, 7800.0)


def designed_od(od_mm: float) -> float:
    """`od_mm` if it is a whole multiple of 10 mm, else ValueError."""
    if od_mm <= 0 or od_mm % 10 != 0:
        raise ValueError(f"{od_mm} mm is not a whole multiple of 10 mm")
    return float(od_mm)


def designed_wall(od_mm: float, dt: float) -> float:
    """The wall thickness of a designed section: the outside diameter over its diameter-to-thickness ratio."""
    return designed_od(od_mm) / dt
