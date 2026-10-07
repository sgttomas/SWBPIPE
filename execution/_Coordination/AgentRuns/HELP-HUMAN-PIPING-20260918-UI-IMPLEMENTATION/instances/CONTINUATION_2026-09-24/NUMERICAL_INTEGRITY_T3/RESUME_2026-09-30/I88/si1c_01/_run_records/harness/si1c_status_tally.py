#!/usr/bin/env python3
"""I88: status transitions of D-flagged checks on the point-mode runner lines (scratch tool)."""
import json, collections, sys
D = sys.argv[1]
def recs(flagfile, thread):
    return [l.rstrip('\n').split('\t') for l in open(flagfile) if l.split('\t', 1)[0] == thread]
def events(rec):
    out = collections.defaultdict(lambda: {'formula': None, 'n4': False})
    if len(rec) > 2 and rec[2]:
        for e in rec[2].split(';'):
            i, k, d = e.split('|', 2); i = int(i)
            if k == 'formula': out[i]['formula'] = d.split(',')
            elif k.startswith('n4'): out[i]['n4'] = True
    return out
def tally(dump, thread, split):
    r = recs(f'{D}/ibase/runner_crate.flags', thread)
    b = [l for l in open(f'{D}/base/{dump}').read().splitlines() if not l.startswith('#')]
    c = [l for l in open(f'{D}/cand/{dump}').read().splitlines() if not l.startswith('#')]
    t = collections.Counter()
    for rec, lb, lc in zip(r, b, c):
        mode, jb = split(lb); _, jc = split(lc)
        if mode not in ('p', 'plain'): continue
        ev = events(rec)
        if not any(e['formula'] and e['formula'][0] != '-' for e in ev.values()): continue
        db = json.loads(jb); dc = json.loads(jc)
        for i, e in ev.items():
            if e['formula'] and e['formula'][0] != '-' and not e['n4']:
                t[f"check {db['checks'][i]['status']} -> {dc['checks'][i]['status']}"] += 1
        t[f"aggregate {db['aggregate_status']} -> {dc['aggregate_status']}"] += 1
    return dict(sorted(t.items()))
sp_rv = lambda l: (l.split('\t')[1], (l.split('\t') + [None])[3])
sp_si = lambda l: (l.split('\t')[1], l.split('\t')[2])
sp_ix = lambda l: (l.split('\t')[4], '\t'.join(l.split('\t')[5:]))
out = {}
for name, thread, sp in (('rv104_run.txt', 'rv104_runner_dump', sp_rv), ('si1c_run.txt', 'si1c_runner_family', sp_si),
                         ('i79_run_extreme.tsv', 'i79_runner_extreme_dump', sp_ix)):
    out[name] = tally(name, thread, sp)
print(json.dumps(out, indent=1))
