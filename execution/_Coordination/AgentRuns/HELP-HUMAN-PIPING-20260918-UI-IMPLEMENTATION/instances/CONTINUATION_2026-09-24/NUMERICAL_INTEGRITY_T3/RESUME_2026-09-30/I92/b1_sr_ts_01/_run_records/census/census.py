"""I92 census comparison: base (I1) against the aligned reader, entry by entry, over 07m.
Usage: census.py <base.jsonl> <aligned.jsonl> <corpus.json>"""
import json, sys, hashlib
base = [json.loads(l) for l in open(sys.argv[1]) if l.strip()]
aligned = [json.loads(l) for l in open(sys.argv[2]) if l.strip()]
corpus_bytes = open(sys.argv[3], 'rb').read(); corpus = json.loads(corpus_bytes)
print('corpus sha256', hashlib.sha256(corpus_bytes).hexdigest())
key = lambda l: (l['kind'], l['id'])
B = {key(l): l for l in base}; A = {key(l): l for l in aligned}
assert len(B) == len(base) and len(A) == len(aligned)
ids = {'mutation': [m['id'] for m in corpus['mutations']], 'must_pass': [m['id'] for m in corpus['must_pass']], 'base': [c['id'] for c in corpus['cases']]}
changes = []
def strip(o):
    # detail text is informational; the census compares (gate, code) or the pass record
    return {k: v for k, v in o.items() if k != 'detail'} if isinstance(o, dict) else o
for kind, lst in ids.items():
    for i in lst:
        b, a = B.get((kind, i)), A.get((kind, i))
        if b is None or a is None: changes.append((kind, i, 'missing', b is None, a is None)); continue
        fields = ['observed'] if kind != 'base' else ['bound', 'unbound', 'transport']
        for f in fields:
            if strip(b[f]) != strip(a[f]): changes.append((kind, i, f, b[f], a[f]))
# expectation checks on each side
def expect_ok(side):
    bad = []
    for l in side:
        if l['kind'] == 'mutation':
            o = l['observed']
            if not ('gate' in o and {'gate': o['gate'], 'code': o['code']} == l['expected']): bad.append(l['id'])
        if l['kind'] == 'must_pass':
            o = l['observed'].get('pass')
            if not o or {k: o[k] for k in ('invocation_bound', 'numerical_eligible', 'standing')} != l['expected_eligibility'] or o['classifications_sha256'] != l['base_classifications_sha256']: bad.append(l['id'])
        if l['kind'] == 'base':
            o = l['bound'].get('pass')
            if not o or o['classifications_sha256'] != l['expected_classifications_sha256']: bad.append(l['id'])
    return bad
nb, na = expect_ok(base), expect_ok(aligned)
print(f"CENSUS mutations {len(ids['mutation'])} (base lines {sum(1 for l in base if l['kind']=='mutation')}, aligned lines {sum(1 for l in aligned if l['kind']=='mutation')}); "
      f"must_pass {len(ids['must_pass'])}; bases {len(ids['base'])}; expectation misses base {len(nb)} aligned {len(na)}; changes {len(changes)}")
for c in changes: print('CHANGE', json.dumps(c))
for i in nb: print('BASE_MISS', i)
for i in na: print('ALIGNED_MISS', i)
