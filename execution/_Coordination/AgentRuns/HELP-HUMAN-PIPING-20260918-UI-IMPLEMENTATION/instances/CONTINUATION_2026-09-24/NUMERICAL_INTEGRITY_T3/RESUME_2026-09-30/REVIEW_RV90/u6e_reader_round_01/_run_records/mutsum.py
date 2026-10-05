import json, sys
S = sys.argv[1]
for f in ['mut_rs.out', 'mut_ts.out', 'mut_py.out', 'mut_corpus.out']:
    try:
        for l in open(S + '/logs/' + f):
            if not l.startswith('{'): print(f, l.strip()[:200]); continue
            r = json.loads(l); print(r['lang'], r['id'], 'KILLED' if r['killed'] else 'SURVIVED', r['failing_count'], r['seconds'], 'compile' if r['compile_or_collect_error'] else '', [x[:90] for x in r['failing'][:3]])
    except FileNotFoundError: pass
