"""RV101: validate RV101's successor stress-neutral packages under the unchanged schemas (own registry)."""
import json, sys
from pathlib import Path
from jsonschema import Draft202012Validator
from referencing import Registry, Resource
from referencing.jsonschema import DRAFT202012
P = Path(sys.argv[1]); PK = Path(sys.argv[2])
res = {}
for path in sorted((P / 'schemas').iterdir()):
    if '.schema.' not in path.name: continue
    try: doc = json.loads(path.read_text())
    except Exception: continue
    r = Resource.from_contents(doc, default_specification=DRAFT202012)
    for u in {path.resolve().as_uri(), 'https://openpipestress.org/schemas/' + path.name, doc.get('$id', path.name)}: res[u] = r
reg = Registry().with_resources(res.items())
def val(name):
    s = json.loads((P / 'schemas' / name).read_text()); Draft202012Validator.check_schema(s); return s, Draft202012Validator(s, registry=reg)
v3s, v3 = val('stress_neutral_export.v0.3.schema.json'); ds, dv = val('stress_neutral_export.schema.json')
for p in sorted(PK.glob('package_*.json')):
    doc = json.loads(p.read_text())
    errs = sorted(v3.iter_errors(doc), key=lambda e: list(e.path))
    def branch_ok(b):
        sch = dict(b); sch['$defs'] = v3s.get('$defs', {}); sch['$schema'] = v3s['$schema']
        if '$id' in v3s: sch['$id'] = v3s['$id']
        return Draft202012Validator(sch, registry=reg).is_valid(doc)
    branches = [i for i, b in enumerate(v3s.get('oneOf', [])) if branch_ok(b)] if 'oneOf' in v3s else ('allOf/anyOf?', list(v3s.keys()))
    print(p.name, 'matching oneOf branches:', branches, 'of', len(v3s.get('oneOf', [])))
    print(p.name, 'v0.3 valid:', not errs, 'dispatcher valid:', dv.is_valid(doc), 'first errors:', [e.message[:120] for e in errs[:2]])
    # negatives: each must be refused
    for label, edit in [('no receipt', lambda d: d.pop('retained_precision')), ('no annotations', lambda d: d.pop('source_annotations')), ('no contract_evidence', lambda d: d.pop('contract_evidence')), ('csv_encoding removed', lambda d: d['export_profile'].pop('csv_encoding')), ('receipt extra member', lambda d: d['retained_precision'].__setitem__('x', 1))]:
        d = json.loads(p.read_text())
        try: edit(d)
        except Exception as e: print('  ', label, 'n/a', e); continue
        print('  ', label, '-> v0.3 valid:', v3.is_valid(d), 'dispatcher valid:', dv.is_valid(d))
