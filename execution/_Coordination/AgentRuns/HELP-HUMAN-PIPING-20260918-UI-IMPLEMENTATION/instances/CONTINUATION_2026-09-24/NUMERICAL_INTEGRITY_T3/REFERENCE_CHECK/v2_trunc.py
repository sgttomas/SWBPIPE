"""V2: are the RF-RANGE / RF-CANCEL-UDL-W1e80 differences (<= 1.7e-27) fully explained by model inputs that
references.json prints rounded to 28 significant digits?  Replaces each such input by the exact value it
rounds (a short decimal times a power of two, or float(1e80)) and re-solves.
Usage: python3 v2_trunc.py <references.json>"""
import copy
import json
import re
import sys
from decimal import Decimal, getcontext
from fractions import Fraction as F
import v2_frame as vf
from v2_models import load_cases

D = Decimal


def exact_of(s):
    neg = s.startswith('-')
    x = vf.parse(s.lstrip('-'))
    if x == F(float(1e80)) or abs(x / F(float(1e80)) - 1) < F(1, 10 ** 26):
        v = F(float(1e80))
        return -v if neg else v
    for p in range(-1100, 1101):
        q = x / F(2) ** p
        if q <= 0:
            continue
        for sig in range(1, 16):
            e = D(q.numerator) / D(q.denominator)
            r = F(format(e, '.%de' % (sig - 1)))
            if r and abs(r / q - 1) < F(1, 10 ** 26):
                v = r * F(2) ** p
                return -v if neg else v
    raise ValueError(s)


def fix(x):
    if isinstance(x, str):
        m = re.fullmatch(r'-?(\d+)(\*2\^-?\d+)?', x)
        if m and len(m.group(1).rstrip('0')) >= 20:
            v = exact_of(x)
            return '%d/%d' % (v.numerator, v.denominator) if v.denominator != 1 else str(v.numerator)
        return x
    if isinstance(x, list):
        return [fix(y) for y in x]
    if isinstance(x, dict):
        return {k: fix(v) for k, v in x.items()}
    return x


ref = json.load(open(sys.argv[1]))['cases']
cases = load_cases(sys.argv[1])
R = vf.Num('dec')
for cid in ['RF-RANGE-CHAIN-LEF-large', 'RF-RANGE-CHAIN-SIM-a', 'RF-RANGE-CHAIN-SIM-b', 'RF-RANGE-SKEW-LEF-large',
            'RF-RANGE-SKEW-SIM-a', 'RF-RANGE-SKEW-SIM-b', 'RF-RANGE-CONT-LEF-large', 'RF-RANGE-CONT-SIM-b',
            'RF-CANCEL-UDL-W1e80']:
    mj = fix(copy.deepcopy(cases[cid]['model']))
    out = vf.solve(vf.load_model(mj), R)
    sc = {k: D(v['value']) for k, v in ref[cid]['scales'].items()}
    worst = max(abs(out[r[0]] - D(r[1])) / max(abs(D(r[1])), sc[r[2]]) for r in ref[cid]['expected'])
    print('%-28s max normalized difference with exact (untruncated) inputs: %s' % (cid, format(worst, '.3e')))
