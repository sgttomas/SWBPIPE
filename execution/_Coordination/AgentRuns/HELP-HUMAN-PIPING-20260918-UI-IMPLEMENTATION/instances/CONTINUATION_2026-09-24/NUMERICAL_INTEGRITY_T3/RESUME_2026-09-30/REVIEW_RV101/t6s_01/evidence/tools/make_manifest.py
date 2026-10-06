#!/usr/bin/env python3
"""RV101: enumerate every committed MechanicsResult-shaped JSON (any depth <= 2) under P and pair it
with a model. Usage: make_manifest.py <P-root> <out.json>. Paths in the output are P-relative."""
import json, os, sys

P, out = sys.argv[1], sys.argv[2]
ROOTS = ['fixtures', 'apps/desktop/src', 'examples', 'validation', 'tests', 'core', 'api', 'docs', 'tools']

def is_mr(x):
    return isinstance(x, dict) and isinstance(x.get('results'), list) and 'run_id' in x and 'model_ref' in x and 'status' in x

def is_model(x):
    return isinstance(x, dict) and isinstance(x.get('project'), dict) and 'id' in x['project'] and 'load_cases' in x and ('pipe_segments' in x or 'nodes' in x)

results, models = [], {}
for root in ROOTS:
    for dp, dn, fn in os.walk(os.path.join(P, root)):
        dn[:] = [d for d in dn if d not in ('node_modules', 'target', '.venv', '__pycache__')]
        for f in sorted(fn):
            if not f.endswith('.json'):
                continue
            p = os.path.join(dp, f)
            rel = os.path.relpath(p, P)
            try:
                if os.path.getsize(p) > 30_000_000:
                    continue
                d = json.load(open(p, encoding='utf-8'))
            except Exception:
                continue
            if is_mr(d):
                results.append((rel, []))
            if is_model(d):
                models.setdefault(d['project']['id'], []).append((rel, []))
            if isinstance(d, dict):
                for k, v in d.items():
                    if is_mr(v):
                        results.append((rel, [k]))
                    if is_model(v):
                        models.setdefault(v['project']['id'], []).append((rel, [k]))
                    if isinstance(v, dict):
                        for k2, v2 in v.items():
                            if is_mr(v2):
                                results.append((rel, [k, k2]))
                            if is_model(v2):
                                models.setdefault(v2['project']['id'], []).append((rel, [k, k2]))

def get(rel, keys):
    d = json.load(open(os.path.join(P, rel), encoding='utf-8'))
    for k in keys:
        d = d[k]
    return d

entries = []
for rel, keys in sorted(results):
    r = get(rel, keys)
    sid = (r.get('producer') or {}).get('semantic_contract_id')
    mode = 'dense_scrutiny' if 'dense' in os.path.basename(rel) else 'sparse_interactive'
    model = None
    d = os.path.dirname(rel)
    base = os.path.basename(rel)
    # 1) a sibling request (mode-specific, then shared stem)
    if base.endswith('.raw.json'):
        stem = base[:-len('.raw.json')]
        cands = [stem + '.request.json']
        if '-' in stem:
            cands.append(stem.split('-')[0] + '.request.json')
        for c in cands:
            cp = os.path.join(d, c)
            if os.path.exists(os.path.join(P, cp)):
                m = get(cp, ['model'])
                if m['project']['id'] == r['model_ref']:
                    model = (cp, ['model'])
                    break
    # 2) a model in the same document (e.g. invocation.request.model or context.request.model)
    if model is None and keys:
        top = get(rel, [])
        for path in (['invocation', 'request', 'model'], ['context', 'request', 'model']):
            try:
                m = top
                for k in path:
                    m = m[k]
                if is_model(m) and m['project']['id'] == r['model_ref']:
                    model = (rel, path)
                    break
            except Exception:
                pass
    # 3) by model_ref
    if model is None and r['model_ref'] in models:
        prefer = sorted(models[r['model_ref']], key=lambda x: (len(x[1]), not x[0].startswith('fixtures/'), x[0]))
        model = prefer[0]
    entries.append({
        'id': rel + ('#' + '/'.join(keys) if keys else ''),
        'result_path': rel, 'result_keys': keys,
        'model_path': model[0] if model else None, 'model_keys': model[1] if model else None,
        'mode': mode, 'semantic_contract_id': sid, 'schema_version': r.get('schema_version'),
        'successor': sid == 'openpipestress.result_semantics/0.3.0/preview-physics-retained-1',
    })
json.dump(entries, open(out, 'w'), indent=1)
print(len(entries), 'entries;', sum(e['successor'] for e in entries), 'successor;', sum(e['model_path'] is None for e in entries), 'without model')
