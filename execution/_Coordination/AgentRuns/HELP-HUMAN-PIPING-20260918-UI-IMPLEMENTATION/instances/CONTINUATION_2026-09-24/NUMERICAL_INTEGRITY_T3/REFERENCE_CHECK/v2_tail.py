"""V2: relative check of the decaying far-field values of RF-LARGE-CONT-n01000 (values down to ~1e-290 of the
class scale), which the 110-digit solve verifies only absolutely.  Re-solves at 400 digits.
Usage: python3 v2_tail.py <references.json>"""
import json
import sys
from decimal import Decimal, getcontext
import v2_frame as vf
from v2_models import load_cases

getcontext().prec = 400
pi = vf.chudnovsky_pi(420)
R = vf.Num('dec', pi=pi)
ref = json.load(open(sys.argv[1]))['cases']
cases = load_cases(sys.argv[1])
for cid in ('RF-LARGE-CONT-n01000-AX', 'RF-LARGE-CONT-n01000-ROT'):
    out = vf.solve(vf.load_model(cases[cid]['model']), R)
    exp = {r[0]: Decimal(r[1]) for r in ref[cid]['expected']}
    worst = (Decimal(0), None)
    for k, e in exp.items():
        if e == 0:
            continue
        rel = abs(out[k] - e) / abs(e)
        if rel > worst[0]:
            worst = (rel, k)
    small = sorted(((abs(e), k) for k, e in exp.items() if e != 0))[:4]
    print(cid, 'max relative difference over nonzero published values %s at %s' % (format(worst[0], '.3e'), worst[1]),
          'smallest |exp|:', [(k, format(v, '.3e'), format(abs(out[k] - exp[k]) / v, '.2e')) for v, k in small])
