#!/usr/bin/env python3
"""I61 U7 slice L: U5 on the U7 head against U5's committed report and log. The only expected
difference is the reader's eligibility (U7, D-U7-5): numerical_eligible false -> true and the
reader label needs_recompute -> eligible, per mode. Substituting U5's two values back must give
U5's bytes exactly. Usage: u5_delta.py U5_REPORT U5_LOG NEW_REPORT NEW_LOG"""
import sys
from pathlib import Path
u5r, u5l, newr, newl = (Path(p).read_text() for p in sys.argv[1:5])
SWAP = [('"numerical_eligible": true, "standing": "eligible"', '"numerical_eligible": false, "standing": "needs_recompute"'),
        ('"numerical_eligible": true,\n     "standing": "eligible"', '"numerical_eligible": false,\n     "standing": "needs_recompute"')]
for name, old, new in (("report", u5r, newr), ("log", u5l, newl)):
    back = new
    for a, b in SWAP: back = back.replace(a, b)
    n = sum(new.count(a) for a, _ in SWAP)
    print(f"{name}: identical={old == new}; eligibility occurrences={n}; identical after substituting U5's eligibility={old == back}")
