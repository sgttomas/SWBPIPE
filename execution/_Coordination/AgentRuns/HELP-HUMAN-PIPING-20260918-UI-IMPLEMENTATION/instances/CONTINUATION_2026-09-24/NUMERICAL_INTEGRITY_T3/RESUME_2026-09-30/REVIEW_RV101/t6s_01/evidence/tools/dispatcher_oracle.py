#!/usr/bin/env python3
"""RV101 dispatcher oracle (scratch). Builds its own local registry over a schemas folder (no network),
then compares the verdicts of the v0.3 version file (V), the candidate dispatcher (D) and the base
dispatcher (B) on:
  1. every committed result document at 0.3.0 found anywhere in the candidate tree (file list from
     `git grep -l result_envelope` over the candidate revision, read with `git show`), including
     execution records and desktop folders;
  2. RV101's own TS-derived documents (differential and successor oracle outputs);
  3. RV101's own mutations of a representative set, built to be refused by V.
Usage: dispatcher_oracle.py <cand P> <base P> <git worktree> <rev> <out dir> [extra doc dirs...]"""
import json, os, random, subprocess, sys, copy, time
from pathlib import Path
from urllib.parse import urljoin
from jsonschema import Draft202012Validator
from referencing import Registry, Resource
from referencing.jsonschema import DRAFT202012

CAND, BASE, GITWT, REV, OUT = (Path(sys.argv[1]), Path(sys.argv[2]), sys.argv[3], sys.argv[4], Path(sys.argv[5]))
EXTRA = [Path(p) for p in sys.argv[6:]]
OUT.mkdir(parents=True, exist_ok=True)

def registry(schema_dir: Path):
    resources = {}
    for path in sorted(schema_dir.iterdir()):
        if '.schema.' not in path.name:
            continue
        try:
            doc = json.loads(path.read_text(encoding='utf-8'))
        except Exception:
            continue
        res = Resource.from_contents(doc, default_specification=DRAFT202012)
        urls = {path.resolve().as_uri(), 'https://openpipestress.org/schemas/' + path.name}
        if '$id' in doc:
            urls.add(doc['$id'])
        for u in urls:
            resources[u] = res
    return Registry().with_resources(resources.items())

def validator(schema_dir: Path, name: str):
    schema = json.loads((schema_dir / name).read_text(encoding='utf-8'))
    Draft202012Validator.check_schema(schema)
    return Draft202012Validator(schema, registry=registry(schema_dir))

V = validator(CAND / 'schemas', 'results.v0.3.schema.yaml')
D = validator(CAND / 'schemas', 'results.schema.yaml')
B = validator(BASE / 'schemas', 'results.schema.yaml')
Vb = validator(BASE / 'schemas', 'results.v0.3.schema.yaml')

def verdicts(doc):
    return (V.is_valid(doc), D.is_valid(doc), B.is_valid(doc))

def find_docs(value, pointer=''):
    stack = [(value, pointer)]
    while stack:
        item, ptr = stack.pop()
        if isinstance(item, dict):
            if isinstance(item.get('schema_version'), str) and isinstance(item.get('result_envelope'), dict):
                yield ptr, item
            stack.extend((c, f'{ptr}/{k}') for k, c in item.items())
        elif isinstance(item, list):
            stack.extend((c, f'{ptr}/{i}') for i, c in enumerate(item))

rows = []
# 1. committed documents anywhere in the candidate revision.
env = dict(os.environ, GIT_OPTIONAL_LOCKS='0')
files = subprocess.run(['git', '-C', GITWT, 'grep', '-l', '-I', '"result_envelope"', REV, '--', 'projects/chirality-piping'], capture_output=True, text=True, env=env).stdout.split('\n')
files = [f.split(':', 1)[1] for f in files if f.strip()]
committed = []
for f in files:
    if not (f.endswith('.json') or f.endswith('.yaml') or f.endswith('.jsonl')):
        continue
    text = subprocess.run(['git', '-C', GITWT, 'show', f'{REV}:{f}'], capture_output=True, env=env).stdout.decode('utf-8', 'replace')
    values = []
    try:
        values = [json.loads(text)]
    except Exception:
        if f.endswith('.jsonl'):
            for line in text.splitlines():
                try:
                    values.append(json.loads(line))
                except Exception:
                    pass
    for value in values:
        for ptr, doc in find_docs(value):
            committed.append((f.replace('projects/chirality-piping/', 'P/') + '#' + ptr, doc))
t0 = time.time()
for name, doc in committed:
    v = verdicts(doc)
    rows.append({'set': 'committed', 'name': name, 'schema_version': doc.get('schema_version'), 'identity': ((doc.get('result_envelope') or {}).get('producer') or {}).get('semantic_contract_id'), 'V': v[0], 'D': v[1], 'B': v[2]})
print('committed docs', len(committed), 'in', round(time.time() - t0, 1), 's', flush=True)

# 2. RV101's own derived documents.
own = []
for d in EXTRA:
    for p in sorted(d.rglob('*.json')):
        try:
            value = json.loads(p.read_text(encoding='utf-8'))
        except Exception:
            continue
        if isinstance(value, dict) and isinstance(value.get('result_envelope'), dict) and isinstance(value.get('schema_version'), str):
            own.append((str(p.relative_to(d.parent)), value))
seen = set()
for name, doc in own:
    key = __import__('hashlib').sha256(json.dumps(doc, sort_keys=True).encode()).hexdigest()
    if key in seen:
        continue
    seen.add(key)
    v = verdicts(doc)
    rows.append({'set': 'rv101_derived', 'name': name, 'schema_version': doc.get('schema_version'), 'identity': ((doc.get('result_envelope') or {}).get('producer') or {}).get('semantic_contract_id'), 'V': v[0], 'D': v[1], 'B': v[2]})
print('own docs', len(seen), flush=True)

# 3. Mutations, built to be refused by V, on one representative per identity at 0.3.0.
reps = {}
for name, doc in committed + own:
    if doc.get('schema_version') != '0.3.0' or not V.is_valid(doc):
        continue
    ident = ((doc.get('result_envelope') or {}).get('producer') or {}).get('semantic_contract_id')
    reps.setdefault(ident, (name, doc))
golden = next((d for n, d in committed if 'retained_precision_successor_derivative_sparse' in n), None)
rng = random.Random(101)

def leaves(v, path=()):
    if isinstance(v, dict):
        for k, c in v.items():
            yield from leaves(c, path + (k,))
    elif isinstance(v, list):
        for i, c in enumerate(v):
            yield from leaves(c, path + (i,))
    else:
        yield path

def get(v, path):
    for k in path:
        v = v[k]
    return v

def setp(v, path, value):
    for k in path[:-1]:
        v = v[k]
    v[path[-1]] = value

def mutations(doc):
    out = []
    def m(label, f):
        d = copy.deepcopy(doc)
        try:
            f(d)
        except Exception as e:
            return
        out.append((label, d))
    for k in list(doc.keys()):
        m(f'drop_top:{k}', lambda d, k=k: d.pop(k))
    m('add_top:x', lambda d: d.__setitem__('x', 1))
    m('envelope_version_0.2.0', lambda d: d['result_envelope'].__setitem__('schema_version', '0.2.0'))
    m('envelope_version_0.4.0', lambda d: d['result_envelope'].__setitem__('schema_version', '0.4.0'))
    m('objectives_one', lambda d: d.__setitem__('objectives', ['OBJ-007']))
    m('objectives_other', lambda d: d.__setitem__('objectives', ['OBJ-007', 'OBJ-008']))
    m('deliverable_wrong', lambda d: d.__setitem__('deliverable_id', 'DEL-00-00'))
    m('efs_extra', lambda d: d['export_format_status'].__setitem__('x', 'TBD'))
    m('efs_value', lambda d: d['export_format_status'].__setitem__('additional_formats', 'csv'))
    for k in list(doc['result_envelope'].keys()):
        m(f'drop_env:{k}', lambda d, k=k: d['result_envelope'].pop(k))
    m('add_env:x', lambda d: d['result_envelope'].__setitem__('x', None))
    ident = (doc['result_envelope'].get('producer') or {}).get('semantic_contract_id')
    if golden is not None and ident != golden['result_envelope']['producer']['semantic_contract_id']:
        m('add_env:retained_precision_from_golden', lambda d: d['result_envelope'].__setitem__('retained_precision', copy.deepcopy(golden['result_envelope']['retained_precision'])))
        m('identity_relabelled_to_successor', lambda d: d['result_envelope']['producer'].__setitem__('semantic_contract_id', golden['result_envelope']['producer']['semantic_contract_id']))
    if 'retained_precision' in doc['result_envelope']:
        m('receipt_sha_not_hex', lambda d: d['result_envelope']['retained_precision'].__setitem__('receipt_sha256', 'x'))
        m('receipt_extra_member', lambda d: d['result_envelope']['retained_precision'].__setitem__('x', 1))
        m('receipt_body_dropped', lambda d: d['result_envelope']['retained_precision'].pop('body'))
        m('identity_relabelled_to_preview', lambda d: d['result_envelope']['producer'].__setitem__('semantic_contract_id', 'openpipestress.result_semantics/0.3.0/preview-physics-1'))
    if doc['result_envelope'].get('row_disclosures'):
        m('disclosure_extra_member', lambda d: d['result_envelope']['row_disclosures'][0].__setitem__('x', 1))
        m('disclosure_reason_bogus', lambda d: d['result_envelope']['row_disclosures'][0].__setitem__('reason_code', 'bogus_reason'))
        m('disclosure_message_number', lambda d: d['result_envelope']['row_disclosures'][0].__setitem__('message', 7))
    paths = list(leaves(doc))
    for i in range(40):
        p = rng.choice(paths)
        old = get(doc, p)
        new = 'rv101' if not isinstance(old, str) else 12345
        m(f'retype:{"/".join(map(str, p))}', lambda d, p=p, new=new: setp(d, p, new))
    return out

mrows = []
for ident, (name, doc) in sorted(reps.items(), key=lambda x: str(x[0])):
    for label, d in mutations(doc):
        v = verdicts(d)
        mrows.append({'set': 'mutation', 'base': name, 'identity': ident, 'mutation': label, 'schema_version': d.get('schema_version'), 'V': v[0], 'D': v[1], 'B': v[2]})
print('mutations', len(mrows), flush=True)
rows += mrows
with open(OUT / 'dispatcher_oracle.jsonl', 'w') as fh:
    for r in rows:
        fh.write(json.dumps(r) + '\n')
# Summaries.
def summary(sel):
    import collections
    c = collections.Counter((r['set'], r['schema_version'], r['V'], r['D'], r['B']) for r in sel)
    return {str(k): v for k, v in sorted(c.items(), key=str)}
mism = [r for r in rows if r.get('schema_version') == '0.3.0' and r['V'] != r['D']]
refused_by_v = [r for r in mrows if r['schema_version'] == '0.3.0' and not r['V']]
res = {'summary': summary(rows), 'v03_V_vs_D_mismatches': mism, 'mutations_refused_by_V': len(refused_by_v), 'of_which_admitted_by_D': [r for r in refused_by_v if r['D']], 'representatives': {str(k): n for k, (n, _) in reps.items()}, 'version_file_unchanged_base_vs_cand': (CAND / 'schemas/results.v0.3.schema.yaml').read_bytes() == (BASE / 'schemas/results.v0.3.schema.yaml').read_bytes()}
(OUT / 'dispatcher_oracle_summary.json').write_text(json.dumps(res, indent=1))
print(json.dumps({k: (v if k != 'summary' else v) for k, v in res.items() if k != 'of_which_admitted_by_D'}, indent=1)[:6000])
print('admitted_by_D_but_refused_by_V:', len(res['of_which_admitted_by_D']))
