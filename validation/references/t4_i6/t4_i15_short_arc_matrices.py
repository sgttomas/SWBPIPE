"""T4-I15 (T4-U1 phase B): reference matrices for CB's short-arc test
`short_arcs_at_r_0_3_form_at_ordinary_and_utm_coordinates`.

The intended element of T4-I6's frozen generator `_run_records/curved_ref.py`
(110 decimal digits) for R = 0.3 m arcs of nominal 0.06, 0.1, 0.5 and 5
degrees: d = grid(L (cos 30 deg, sin 30 deg, 0)) with y = (0, 1, 0) (IP) and
d = grid(L (1, 2, 2)/3) with y = (1, -1, 0.5) (SK), L = 2R sin(phi/2), on
the 2^-30 m grid (so x_i + d is exact at X = 5e6 m); E 2e11, G 8e10, K-D5's
section, k = 1. Prints, per arc, d, the actual included angle and the
global 12x12 K with each entry rounded once to binary64 (repr).

Usage, from this directory: python3 -I t4_i15_short_arc_matrices.py _run_records
Standard library only.
"""
import math
import sys

sys.path.insert(0, sys.argv[1])
from decimal import getcontext  # noqa: E402

import curved_ref as cr  # noqa: E402

getcontext().prec = 110
g = 2.0 ** -30
grid = lambda v: round(v / g) * g  # noqa: E731
R = 0.3
for degrees in (0.06, 0.1, 0.5, 5.0):
    phi = math.radians(degrees)
    length = 2.0 * R * math.sin(0.5 * phi)
    for plane, unit, y in (("IP", (math.cos(math.pi / 6.0), math.sin(math.pi / 6.0), 0.0), (0.0, 1.0, 0.0)),
                           ("SK", (1.0 / 3.0, 2.0 / 3.0, 2.0 / 3.0), (1.0, -1.0, 0.5))):
        d = [grid(length * u) for u in unit]
        e = cr.curved_element([0.0, 0.0, 0.0], d, R, list(y), 200e9, 80e9, 0.005969026041820614,
                              2.700984283923829e-05, 5.401968567847658e-05, 1.0, 1.0)
        print(f"arc {degrees} {plane} d {[repr(v) for v in d]} y {list(y)} phi {cr.sci(e['geo']['phi'])}")
        print("[")
        for row in e["K"]:
            print("    [" + ", ".join(repr(float(v)) for v in row) + "],")
        print("]")
