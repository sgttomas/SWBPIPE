#!/usr/bin/env python3
"""G-i corpus: validate every translated value against its schema $def (jsonschema, Draft 2020-12)."""
import json, sys, collections
import jsonschema
schema = json.load(open(sys.argv[1])); corpus = json.load(open(sys.argv[2]))
defs = schema['$defs']
inline = {'BlockRefusal': defs['Physical']['properties']['bound_refusals']['items']}
bad = 0; per = collections.Counter(); fails = []
for x in corpus:
    per[x['def']] += 1
    if x['failures']:
        fails.append((x['label'], [ (f['check'], f['path']) for f in x['failures']]))
        continue
    sub = inline.get(x['def'], {'$ref': '#/$defs/' + x['def']})
    v = jsonschema.Draft202012Validator({**sub, '$defs': defs})
    errs = list(v.iter_errors(x['value']))
    if errs:
        bad += 1; print('INVALID', x['def'], x['label'], json.dumps(x['value'])[:200], errs[0].message[:200])
print('entries', len(corpus), 'by def', dict(per))
print('schema-invalid', bad)
print('entries with typed failures (no wire form):')
for l, f in fails: print('  ', l, f)
