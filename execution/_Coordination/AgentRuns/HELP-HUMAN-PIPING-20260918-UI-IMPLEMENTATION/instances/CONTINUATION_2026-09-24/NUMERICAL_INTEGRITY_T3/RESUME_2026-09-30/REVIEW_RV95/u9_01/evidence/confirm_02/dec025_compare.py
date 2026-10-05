"""RV95: per-manifest, per-test comparison of two DEC-025 NFF suite directories (read-only)."""
import os, re, sys, json, collections
def parse(path):
    out = {}; binary = '?'
    for line in open(path, errors='replace'):
        m = re.match(r'\s*(Running|Doc-tests)\s+(\S+)', line)
        if m:
            binary = m.group(2) if m.group(1) == 'Running' else 'doc-tests ' + m.group(2)
            binary = re.sub(r'-[0-9a-f]{16}\)?$', '', binary).split('/')[-1] if m.group(1) == 'Running' else binary
            continue
        m = re.match(r'test (.+?) \.\.\. (ok|FAILED|ignored)(?:, .*)?$', line.rstrip())
        if m:
            out[(binary, m.group(1))] = m.group(2)
    return out
def load(d):
    res = {}
    for f in sorted(os.listdir(d)):
        if f.endswith('.log'):
            res[f.split('_', 1)[1]] = parse(os.path.join(d, f))
    return res
a, b = load(sys.argv[1]), load(sys.argv[2])
report = {}
for man in sorted(set(a) | set(b)):
    x, y = a.get(man, {}), b.get(man, {})
    cnt = lambda t: collections.Counter(t.values())
    added = sorted(k for k in y if k not in x); removed = sorted(k for k in x if k not in y)
    changed = sorted(k for k in x if k in y and x[k] != y[k])
    if added or removed or changed or cnt(x) != cnt(y) or any(v == 'FAILED' for v in y.values()):
        report[man] = {'base': dict(cnt(x)), 'cand': dict(cnt(y)),
                       'added': [(k[0], k[1], y[k]) for k in added], 'removed': [(k[0], k[1], x[k]) for k in removed],
                       'changed': [(k[0], k[1], x[k], y[k]) for k in changed],
                       'cand_failed': sorted(k[1] for k, v in y.items() if v == 'FAILED')}
json.dump(report, open(sys.argv[3], 'w'), indent=1)
tot = lambda r: sum(sum(t.values()) for t in r.values())
print('manifests', len(a), len(b), 'tests', tot({k: collections.Counter(v.values()) for k, v in a.items()}), tot({k: collections.Counter(v.values()) for k, v in b.items()}))
for man, r in report.items():
    print(f"== {man}: base {r['base']} cand {r['cand']}; +{len(r['added'])} -{len(r['removed'])} changed {len(r['changed'])}; cand FAILED {r['cand_failed']}")
    for k in r['changed']: print('   CHANGED', k)
    adds_not_ok = [k for k in r['added'] if k[2] != 'ok']
    for k in adds_not_ok: print('   ADDED-not-ok', k)
    for k in r['removed']: print('   REMOVED', k)
