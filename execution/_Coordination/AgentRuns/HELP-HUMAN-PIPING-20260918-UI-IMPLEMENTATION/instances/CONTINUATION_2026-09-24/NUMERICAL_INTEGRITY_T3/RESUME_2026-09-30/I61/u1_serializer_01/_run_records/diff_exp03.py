#!/usr/bin/env python3
"""U1 control 2: structural diff of the serializer's successor against the experiment-03 receipts."""
import json, sys, os
U, E = sys.argv[1], sys.argv[2]
def diff(a, b, path='$', out=None):
    out = [] if out is None else out
    if type(a) != type(b): out.append((path, a, b)); return out
    if isinstance(a, dict):
        for k in sorted(set(a) | set(b)):
            if k not in a or k not in b: out.append((f'{path}.{k}', a.get(k, '<absent>'), b.get(k, '<absent>')))
            else: diff(a[k], b[k], f'{path}.{k}', out)
    elif isinstance(a, list):
        if len(a) != len(b): out.append((path + '.length', len(a), len(b)))
        for i, (x, y) in enumerate(zip(a, b)): diff(x, y, f'{path}[{i}]', out)
    elif a != b: out.append((path, a, b))
    return out
for m in ['sparse_interactive', 'dense_scrutiny']:
    u = json.load(open(os.path.join(U, f'u1_milestone_{m}.json')))
    e = json.load(open(os.path.join(E, f'milestone_{m}.json')))
    print(f'== {m}: invocation equal={u["invocation"] == e["invocation"]}')
    for p, x, y in diff(e['source'], u['source']):
        print(f'  DIFF {p}: exp03={json.dumps(x)[:160]} u1={json.dumps(y)[:160]}')
