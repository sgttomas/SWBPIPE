#!/usr/bin/env python3
"""U1 control 3: Python/TypeScript class files against the certificate verdicts (experiment-03 verdict lines; same certificate)."""
import sys, os
S, E = sys.argv[1], sys.argv[2]
for m in ['sparse_interactive', 'dense_scrutiny']:
    f = os.path.join(S, f'u1_milestone_{m}.json')
    py = open(f + '.py_classes.txt').read().splitlines(); ts = open(f + '.ts_classes.txt').read().splitlines()
    ver = [l.split('|') for l in open(os.path.join(E, f'milestone_{m}.verdicts.txt')).read().splitlines() if l]
    vmap = {v[0]: ('non_quantity' if v[3] == 'none' else v[3]) for v in ver}
    rows = [l.split('|') for l in py]
    agree = sum(1 for r in rows if vmap.get(r[0]) == r[3])
    same_e3 = open(os.path.join(E, f'milestone_{m}.json.py_classes.txt')).read().splitlines() == py
    print(f'{m}: rows {len(py)} py==ts {py == ts} agree_with_verdicts {agree}/{len(ver)} py_classes_equal_exp03 {same_e3}')
