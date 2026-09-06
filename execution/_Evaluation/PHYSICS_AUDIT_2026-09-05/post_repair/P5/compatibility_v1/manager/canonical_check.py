from pathlib import Path
import json,yaml,hashlib,collections
from jsonschema import Draft202012Validator
p=Path(__file__).resolve().parent
w=next(x for x in p.parents if x.name=='chirality-piping')
a=w/'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-PHYSICS-AUDIT/instances/P5/compatibility_v1/author'
s=yaml.safe_load((w/'schemas/results.schema.yaml').read_text())
v=Draft202012Validator(s); mv=Draft202012Validator(s['$defs']['ResultMetadata'])
out={}
for name in ('linear','nonlinear','zero_pressure'):
 f=a/(name+'_canonical_document.json'); doc=json.loads(f.read_text()); rows=[r for rs in doc['result_envelope']['result_sets'] for r in rs['values']]
 errors=[]; counts=collections.Counter(); invalidrows=0; stations=[]
 for r in rows:
  es=list(mv.iter_errors(r['metadata'])) if 'metadata' in r else []
  if es: invalidrows+=1
  for e in es:
   field='/'.join(map(str,e.path));counts[field]+=1;errors.append({'result_id':r['result_id'],'field':field,'value':e.instance,'message':e.message})
  if ':station-' in r['result_id'] or ':section:' in r['result_id']: stations.append(r['result_id'])
 out[name]={'document_sha256':hashlib.sha256(f.read_bytes()).hexdigest(),'actual_canonical_rows':len(rows),'metadata_rows':sum('metadata'in r for r in rows),'invalid_metadata_rows':invalidrows,'metadata_field_violations':len(errors),'fields':dict(counts),'violations':errors,'full_schema_errors':[{'path':'/'.join(map(str,e.absolute_path)),'message':e.message} for e in v.iter_errors(doc)]}
(p/'ACTUAL_CANONICAL_VALIDATION.json').write_text(json.dumps(out,indent=2)+'\n')
for k,d in out.items(): print(k,{a:b for a,b in d.items() if a not in ('violations','full_schema_errors')},'fullerrors',len(d['full_schema_errors']))
