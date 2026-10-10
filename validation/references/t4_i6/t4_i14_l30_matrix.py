"""T4-I14 (T4-U1): frozen system matrices for FK's
`kd5_long_nearly_straight_bend_at_the_floor_does_not_demote`.

The intended element of T4-I6's frozen generator `_run_records/curved_ref.py`
(110 decimal digits) for RV131 `rv_o4_axial`'s configuration: d =
grid(30 (cos 30 deg, sin 30 deg, 0)) on the 2^-30 m grid, y = (0, 1, 0),
R = |d| / (2 sin(phi/2)) for phi = 1e-9 and 2e-9 rad, E 2e11, G 8e10,
K-D5's section, k = 1. Each entry is rounded once to binary64 (repr).

Usage, from this directory: python3 -I t4_i14_l30_matrix.py _run_records
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
d = [grid(30.0 * math.cos(math.pi / 6.0)), grid(30.0 * math.sin(math.pi / 6.0)), 0.0]
length = math.sqrt(d[0] * d[0] + d[1] * d[1])
for nominal in (1.0e-9, 2.0e-9):
    R = length / (2.0 * math.sin(0.5 * nominal))
    K = cr.curved_element([0.0, 0.0, 0.0], d, R, [0.0, 1.0, 0.0], 200e9, 80e9, 0.005969026041820614,
                          2.700984283923829e-05, 5.401968567847658e-05, 1.0, 1.0)["K"]
    print("nominal", nominal, "R", repr(R), "d", [repr(v) for v in d])
    print("[")
    for row in K:
        print("    [" + ", ".join(repr(float(v)) for v in row) + "],")
    print("]")
