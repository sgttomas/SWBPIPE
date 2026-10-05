#!/usr/bin/env python3
"""Experiment 03 checks: per-reader class parity against producer verdicts, and a
byte/structural diff of each receipt against the experiment-02 receipt."""
import hashlib, json, sys, os
X, E2 = sys.argv[1], sys.argv[2]   # exp-03 out dir, exp-02 out dir
def classes(path, norm=False):
    rows = []
    for line in open(path).read().splitlines():
        if not line: continue
        rid, n, s, c = line.split('|', 3)
        if norm:
            c = {'NonQuantity':'non_quantity','RelativeVerified':'relative_verified','InputDerived':'input_derived'}.get(c, c)
            if c.startswith('AbsoluteVerified'): c = 'absolute_verified'
        rows.append((rid, n, s, c))
    return rows
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
for m in ['milestone_sparse_interactive', 'milestone_dense_scrutiny']:
    f3, f2 = os.path.join(X, m + '.json'), os.path.join(E2, m + '.json')
    b3, b2 = open(f3, 'rb').read(), open(f2, 'rb').read()
    h3, h2 = hashlib.sha256(b3).hexdigest(), hashlib.sha256(b2).hexdigest()
    print(f'== {m}\n  exp03 sha256={h3} bytes={len(b3)}\n  exp02 sha256={h2} bytes={len(b2)}\n  byte_identical={b3 == b2}')
    d3, d2 = json.loads(b3), json.loads(b2)
    for p, x, y in diff(d2, d3): print(f'  DIFF {p}: exp02={json.dumps(x)[:200]} exp03={json.dumps(y)[:200]}')
    rp3 = d3['source']['retained_precision']; rp2 = d2['source']['retained_precision']
    print(f'  receipt_sha256 equal={rp3["receipt_sha256"] == rp2["receipt_sha256"]} publication_sha256 equal={rp3["body"]["publication_sha256"] == rp2["body"]["publication_sha256"]}')
    print(f'  receipt_sha256={rp3["receipt_sha256"]}')
    b = rp3['body']; o = b['ordinary_attempts'][0]
    sbru = sum(1 for d in d3['source']['diagnostics'] if d['code'] == 'SOURCE_BLOCK_RECOVERY_UNAVAILABLE')
    print(f'  request schema {d3["invocation"]["request"]["model"]["schema_version"]} SBRU in envelope: {sbru} UNMAPPED {json.dumps(d3).count("UNMAPPED")}')
    print(f'  legacy_source {o["legacy_source"]} legacy_source_work {b["legacy_source_work"]}')
    print(f'  diagnostic_refs {o["diagnostic_refs"]}')
    py = classes(f3 + '.py_classes.txt'); rs = classes(f3 + '.rs_classes.txt', True); ts = classes(f3 + '.ts_classes.txt')
    ver = [l.split('|') for l in open(os.path.join(X, m + '.verdicts.txt')).read().splitlines() if l]
    vmap = {v[0]: ('non_quantity' if v[3] == 'none' else v[3], v[4]) for v in ver}
    agree = sum(1 for r in py if r[0] in vmap and vmap[r[0]][0] == r[3])
    counts = {}
    for r in py: counts[r[3]] = counts.get(r[3], 0) + 1
    print(f'  rows {len(py)} py==rs {py == rs} py==ts {py == ts} agree_with_producer_verdicts {agree} / {len(ver)} all_verdicts_passed {all(v[4] == "passed=true" for v in ver)} classes {counts}')
    for kind in ['py', 'rs', 'ts']:
        c3 = open(f3 + f'.{kind}_classes.txt', 'rb').read(); c2 = open(f2 + f'.{kind}_classes.txt', 'rb').read()
        print(f'  {kind}_classes identical to exp02: {c3 == c2}')
    v2 = open(os.path.join(E2, m + '.verdicts.txt'), 'rb').read(); v3 = open(os.path.join(X, m + '.verdicts.txt'), 'rb').read()
    print(f'  producer verdicts identical to exp02: {v2 == v3}')
    p2 = json.load(open(os.path.join(E2, m + '.provenance.json'))); p3 = json.load(open(os.path.join(X, m + '.provenance.json')))
    s2 = {tuple(x) for x in p2}; s3 = {tuple(x) for x in p3}
    for x in p2:
        if tuple(x) not in s3: print(f'  PROV - {x[0]}: {x[1][:160]}')
    for x in p3:
        if tuple(x) not in s2: print(f'  PROV + {x[0]}: {x[1][:160]}')
