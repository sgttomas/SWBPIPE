"""Revision-1 preservation check: every pre-existing (case, quantity) value is unchanged."""
import json, sys
old = json.load(open(sys.argv[1]))
new = json.load(open(sys.argv[2]))
oc = {c['id']: c for c in old['cases']}
nc = {c['id']: c for c in new['cases']}
assert set(oc) <= set(nc), 'a case was removed'
added = sorted(set(nc) - set(oc))
nvals = nrep = 0
field_changes = []
nc_changes = []
for cid, o in oc.items():
    n = nc[cid]
    for k in o:
        if k in ('negative_controls',):
            continue
        if o[k] != n.get(k):
            field_changes.append((cid, k, sorted(set(n[k]) - set(o[k])) if isinstance(o[k], dict) else None))
    assert set(n) - set(o) == set(), (cid, set(n) - set(o))
    # values: rows identical, row by row
    assert o['expected'] == n['expected'], cid
    nvals += len(o['expected'])
    assert o.get('expected_represented') == n.get('expected_represented'), cid
    nrep += len(o.get('expected_represented') or [])
    on = {x['id']: x for x in o['negative_controls']}
    nn = {x['id']: x for x in n['negative_controls']}
    for i, x in on.items():
        if i not in nn:
            nc_changes.append((cid, i, 'retired'))
            continue
        y = nn[i]
        rest_o = {k: v for k, v in x.items() if k not in ('defect', 'label')}
        rest_n = {k: v for k, v in y.items() if k not in ('defect', 'label')}
        assert rest_o == rest_n, (cid, i, 'result changed')
        ch = [k for k in ('defect', 'label') if x.get(k) != y.get(k)]
        if ch:
            nc_changes.append((cid, i, '+'.join(ch)))
    assert set(nn) <= set(on), (cid, 'control added to an existing case')
print('pre-existing cases: %d; expected rows identical: %d; represented rows identical: %d' % (len(oc), nvals, nrep))
print('added cases: %s' % added)
print('non-control field changes: %s' % field_changes)
print('control changes (%d):' % len(nc_changes))
for c in nc_changes:
    print('  %s %s: %s' % c)
