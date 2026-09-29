#!/usr/bin/env python3
"""RV19 delta check: every published row (value, class, bound) and body scale
of the publications selected at 7d8fa9c0e against the same rows at
a5fa0eaf7 (both from RV19's dump probe). Standard library only."""
import sys, re
from collections import Counter
def rows(p):
    d = {}
    for line in open(p):
        for m in re.finditer(r'ROW (\S+) (\S+) (\S+) (\S+) (\S) ([0-9a-f]{16}) (\S+) ([0-9a-f]{16})', line):
            d[(m.group(1), m.group(2))] = m.groups()[2:]
        for m in re.finditer(r'SCALE (\S+) (\d+) (\S+) ([0-9a-f]{16})', line):
            d[('SCALE', m.group(1), m.group(2), m.group(3))] = m.group(4)
    return d
a, b = rows(sys.argv[1]), rows(sys.argv[2])
common = [k for k in a if k in b]
val, cls, bnd, scl = Counter(), Counter(), Counter(), Counter()
for k in common:
    if k[0] == 'SCALE':
        if a[k] != b[k]: scl[k[1]] += 1
        continue
    if a[k][2:4] != b[k][2:4]: val[k[0]] += 1
    if a[k][4] != b[k][4]: cls[k[0]] += 1
    elif a[k][5] != b[k][5]: bnd[k[0]] += 1
print('entries in both %d; only at 7d8fa9c0e %d; only at a5fa0eaf7 %d' % (
    len(common), len([k for k in a if k not in b]), len([k for k in b if k not in a])))
print('value changes', dict(val)); print('class changes', dict(cls))
print('bound changes', dict(bnd)); print('scale changes', dict(scl))
