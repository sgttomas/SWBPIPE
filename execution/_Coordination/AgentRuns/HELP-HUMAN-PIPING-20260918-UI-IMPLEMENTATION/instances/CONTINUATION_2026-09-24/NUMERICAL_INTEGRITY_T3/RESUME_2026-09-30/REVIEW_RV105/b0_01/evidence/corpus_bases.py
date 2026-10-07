# RV105: enumerate CORPUS bases (read-only): per case status, solve_quality, initial kind, w2 kind; parity rows.
import json, hashlib, sys
path = sys.argv[1]
raw = open(path, 'rb').read()
print('# CORPUS sha256', hashlib.sha256(raw).hexdigest())
d = json.loads(raw)
print('# bases', len(d['cases']), 'mutations', len(d['mutations']), 'must_pass', len(d['must_pass']))
for c in d['cases']:
    src = c['source']; b = src['retained_precision']['body']
    sts = [(x['status'], src['numerical_quality']['cases'][i]['solve_quality'], b['ordinary_attempts'][i]['initial']['kind'], b['ordinary_attempts'][i]['w2']['kind']) for i, x in enumerate(b['cases'])]
    parity = sum(1 for r in src['results'] if r.get('kind') == 'sparse_live_path_dense_parity_relative_delta')
    print(c['id'], c['invocation']['solver_mode'], sts, 'parity_rows', parity)
nr = [m['id'] for m in d['must_pass'] + d['mutations'] if 'not_required' in json.dumps(m['edits'])]
print('# entries editing a case to not_required:', nr)
