#!/usr/bin/env python3
"""Independent boundary/selection and source-versus-representation evidence.
No product import or candidate arithmetic is an expected-output oracle. Full
certificate predicates and G5a cannot be checked because binding never ran them.
"""
from pathlib import Path
import json,sys,hashlib,importlib.util
spec=importlib.util.spec_from_file_location('oracle',Path(__file__).with_name('independent_oracle.py'))
o=importlib.util.module_from_spec(spec);spec.loader.exec_module(o)
records=[];current=None;controls=[];refusals=[]
keys={'REQUEST':'request','INPUT':'input','ROWS':'rows','VERDICTS':'verdicts','G5A_DATA':'g5a','SELECTION':'selection','ANCILLARY':'ancillary','NATIVE_ROWS':'native_rows','NATIVE_IDENTITY':'native_identity','BOUNDARY':'boundary'}
for line in Path(sys.argv[1]).read_text().splitlines():
 if line.startswith('I47_CASE '):
  current={'label':line.split(' ',1)[1]};records.append(current)
 elif line.startswith('I47_CONTROL '):controls.append(json.loads(line.split(' ',1)[1]))
 elif line.startswith('I47_REFUSAL '):refusals.append(json.loads(line.split(' ',1)[1]))
 elif line.startswith('I47_'):
  tag,payload=line.split(' ',1)
  if tag[4:] in keys:current[keys[tag[4:]]]=json.loads(payload)
assert [x['label'] for x in records]==['point-zero','point-loaded','interpolated-zero','interpolated-loaded']
result=[]
for r in records:
 source=o.select_source(r)
 b=r['boundary'];rows=r['rows'];env=r['ancillary'];loaded=r['label'].endswith('loaded')
 assert b=={'calls':[1,1,1],'error':'Some(Association("row member"))','full_case':False,'g5a_entered':0,'observable_error':'None','verdict_count':0}
 assert r['verdicts']==[] and env['status']['mechanics']=='MECHANICS_SOLVED'
 assert len(rows)==75 and len(r['native_rows'])==52 and r['native_identity']['calls']==1
 ancillary=[(i,x) for i,x in enumerate(rows) if x['kind'] in ['linear_solver_mode_basis','modulus_basis_record']]
 assert [(i,x['kind']) for i,x in ancillary]==[(0,'modulus_basis_record'),(1,'linear_solver_mode_basis')]
 mr=ancillary[0][1];case=r['selection']['case']
 assert mr['entity_ref']==case and mr['basis_ref']=={'ref_type':'load_case','ref_id':case}
 assert mr['value']==1 and mr['unit']=='record'
 assert mr['metadata']['component']=='material_modulus_basis'
 assert mr['metadata']['location']==case
 assert all(p in mr['metadata']['basis'] for p in r['selection']['point_ids'] if p is not None)
 mechanical=[];node_ids=[x['entity_ref'] for x in rows if x['kind']=='displacement_magnitude']
 for i,row in enumerate(rows):
  if i<2:continue
  node=node_ids.index(row['entity_ref']) if row['entity_ref'] in node_ids else None
  loc=row.get('metadata',{}).get('location')
  if row['kind']=='support_reaction_component_v2':loc=row['metadata']['component']
  a=o.source_pair(r['input'],row['kind'],loc,node,loaded,True)
  k=o.source_pair(r['input'],row['kind'],loc,node,loaded,False)
  n=o.normalized(row)
  mechanical.append({'ordinal':i,'id':row['id'],'kind':row['kind'],'actual_raw_bits':o.bits(row['value']),'actual_normalized_bits':o.bits(n),'source_si':[str(x) for x in a],'represented_si':[str(x) for x in k],'source_raw':[str(x) for x in o.raw(a,row)],'represented_raw':[str(x) for x in o.raw(k,row)],'source_si_error':[str(x) for x in o.distance_bounds(n,a)],'represented_si_error':[str(x) for x in o.distance_bounds(n,k)],'candidate_predicates':'not executed; row binding refused','candidate_final_class_scale':'not produced'})
 assert len(mechanical)==73
 result.append({'case':r['label'],'selection':source,'boundary':b,'ancillary_roster':ancillary,'mechanical_rows':mechanical,'full_case':False,'g5a':'not executed; zero operation prefix','observable_validator':'not executed; None is not a pass'})
assert [x['name'] for x in controls]==['base-G-does-not-substitute','wrong-point-is-observably-distinct']
expected={'missing-alpha':'MODULUS_BASIS_INPUT_MISSING','missing-selected-G':'MODULUS_BASIS_INPUT_MISSING','unknown-point':'MODULUS_BASIS_UNRESOLVED','at-point-temperature':'MODULUS_BASIS_UNRESOLVED'}
assert set(x['name'] for x in refusals)==set(expected)
for r in refusals:
 assert r['calls']==[1,0,1]
 assert expected[r['name']] in [d['code'] for d in r['envelope']['diagnostics']]
 assert r['envelope']['status']['mechanics']!='MECHANICS_SOLVED'
output={'result':'source blocker independently identified; complete numerical witness unavailable','log_sha256':hashlib.sha256(Path(sys.argv[1]).read_bytes()).hexdigest(),'checks':result,'controls':[x['name'] for x in controls],'resolver_refusals':expected,'scope':'Captured source/represented closed-form comparisons only; no fabricated final verdict, class, scale, predicate or G5a success.'}
Path(sys.argv[2]).write_text(json.dumps(output,indent=2)+'\n')
print(json.dumps({'result':output['result'],'cases':[{'case':x['case'],'source':x['selection'],'mechanical_rows':len(x['mechanical_rows']),'ancillary_rows':len(x['ancillary_roster']),'boundary':x['boundary']} for x in result],'controls':output['controls'],'resolver_refusals':expected},indent=2))
