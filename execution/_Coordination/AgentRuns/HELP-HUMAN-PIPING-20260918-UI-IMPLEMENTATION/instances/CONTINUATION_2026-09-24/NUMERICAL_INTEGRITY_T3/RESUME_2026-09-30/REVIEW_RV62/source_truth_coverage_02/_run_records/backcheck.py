"""RV62 records-only full E/G source-cover backcheck; no producer/solver imports.
Reuse this reviewer's frozen exact algebra; add all independent material corners
and retain K with its two represented-Z meanings. Bulk result goes to scratch.
"""
import sys
sys.dont_write_bytecode=True
from pathlib import Path
from fractions import Fraction as Q
import importlib.util,json,hashlib,copy
HERE=Path(__file__).resolve().parent
manifest=json.loads((HERE/'INPUT_MANIFEST.json').read_text())
files={Path(x['absolute_location']).name:Path(x['absolute_location']) for x in manifest['inputs']}
for x in manifest['inputs']:
 p=Path(x['absolute_location']);assert hashlib.sha256(p.read_bytes()).hexdigest()==x['sha256'] and p.stat().st_size==x['bytes']
spec=importlib.util.spec_from_file_location('rv62_frozen',files['rv62_exact.py']);o=importlib.util.module_from_spec(spec);spec.loader.exec_module(o)
r=HERE.parents[2];author=r/'I47/source_truth_coverage_03/_run_records'
records=[];keys={'REQUEST','INPUT','ROWS','VERDICTS','G5A_DATA','SELECTION','ANCILLARY','NATIVE_ROWS','NATIVE_IDENTITY','BOUNDARY','BASIS_CAPTURE'}
for line in files['pp_debug.log'].read_text().splitlines():
 if line.startswith('I47_CASE '):records.append({'label':line.split(' ',1)[1]})
 elif line.startswith('I47_'):
  key,_,text=line.partition(' ')
  if key[4:] in keys:records[-1][key[4:]]=json.loads(text)
assert len(records)==4
# Author did not change captures: compare each shared field to independent old logs.
translate={'REQUEST':'request','INPUT':'input','ROWS':'rows','VERDICTS':'verdicts','G5A_DATA':'g5a','SELECTION':'selection','ANCILLARY':'ancillary','NATIVE_ROWS':'native_rows','NATIVE_IDENTITY':'native_identity','BOUNDARY':'boundary','BASIS_CAPTURE':'basis_capture'}
author_records=json.loads((author/'CAPTURES.json').read_text())
for a,b in zip(records,author_records):
 assert a['label']==b['label']
 for key,dest in translate.items():assert a[key]==b[dest],(a['label'],key,'changed capture')
old=json.loads(files['exact_final_debug.json'].read_text())['cases'];author_full=json.loads((author/'FULL_COVER_RESULTS.json').read_text())

def profile(q):
 q=Q(q);assert q.denominator & (q.denominator-1)==0;n=abs(q.numerator)
 significant=n.bit_length()-((n&-n).bit_length()-1) if n else 0
 return {'fraction':str(q),'significant_bits':significant,'binary_denominator_exponent':q.denominator.bit_length()-1}

def material_box(rec,b):
 Es,Gs=b['source'];Eh,Gh=map(Q,b['represented']);profiles={name:profile(x) for name,x in [('Es',Es),('Gs',Gs),('Eh',Eh),('Gh',Gh)]}
 selection=rec['SELECTION']['values']
 if selection['kind']=='interpolated':
  lo,t,hi=[Q(o.D(selection[k])) for k in ['t_lo','t','t_hi']];h=hi-lo;assert h>0
  profiles['h']=profile(h)
  for key,result in [('e',Es),('g',Gs)]:
   a,z=[Q(o.D(selection[key+k])) for k in ['_lo','_hi']]
   products=[hi*a,-t*a,t*z,-lo*z];running=Q(0)
   for j,value in enumerate(products):
    profiles[f'{key}_product_{j}']=profile(value);running+=value;profiles[f'{key}_sum_{j}']=profile(running)
   assert running>0 and running/h==result
   profiles[key+'_quotient']=profile(running/h)
 assert max(x['significant_bits'] for x in profiles.values())<=1024
 # Every actual material intermediate is dyadic within the 1024-bit format;
 # h and X endpoints therefore are exact here, not merely ideal endpoints.
 E=tuple(sorted([Es,Eh]));G=tuple(sorted([Gs,Gh]));assert E[0]>0 and G[0]>0
 return E,G,profiles

def k_readouts(rec,b,row):
 # Distinct represented bending meanings, not a relabelled source-geometry law.
 both=o.law(rec,b,row,False);kind=row['kind'];inp=rec['INPUT']
 if kind not in ['element_local_bending_normal_stress_z','pipe_elastic_normal_stress_maximum_v2']:
  return [('K_Z_hat',both),('K_I_over_exact_c',both)]
 zhat=o.point(inp['Z']);zc=o.over(o.point(inp['I']),o.point(Q(inp['D'])/2));fx=b['loads']['UX'];fy=b['loads']['UY']
 if kind=='element_local_bending_normal_stress_z':
  x={'end_i':Q(0),'end_j':Q(1),'quarter_1':Q(1,4),'midspan':Q(1,2),'quarter_3':Q(3,4)}[row['metadata']['location']]
  numerator=o.point(fy*(1-x));out=[o.over(numerator,z) for z in [zhat,zc]]
 else:out=[o.plus(o.over(o.point(abs(fx)),o.point(inp['A'])),o.over(o.point(abs(fy)),z)) for z in [zhat,zc]]
 assert o.join(*out)==both
 return list(zip(['K_Z_hat','K_I_over_exact_c'],out))

results=[];changes=[];counts={'rows':0,'predicates':0,'candidate_pass_predicates':0,'uncertain':0,'false_candidate_pass':0,'conservative_predicates':0};decisive={}
for index,rec in enumerate(records):
 previous=o.verify(rec);assert previous==old[index],('old comparisons changed',rec['label'])
 b=o.inputs(rec);m=rec['REQUEST']['model'];assert m.get('components',[])==[] and m.get('combinations',[])==[]
 assert b['support']['family']=='anchor' and all(value in [Q(0),Q(1)] for value in b['loads'].values())
 assert len(set(b['loads'].values()))==1 and 0<Q(rec['INPUT']['t'])<Q(rec['INPUT']['D'])/2
 E,G,profiles=material_box(rec,b);corners=[(f'source_E_{i}_G_{j}',e,g) for i,e in enumerate(E) for j,g in enumerate(G)]
 output=[]
 for prior in previous['rows']:
  ordinal=prior['row'];row=rec['ROWS'][ordinal];v=rec['VERDICTS'][ordinal];n=o.si(row)
  laws=[]
  for name,e,g in corners:
   operands={**b,'source':[e,g]};laws.append((name,o.law(rec,operands,row,True)))
  laws+=k_readouts(rec,b,row)
  tests=[]
  for slot,p in enumerate(prior['proofs']):
   raw=p['name']=='DecimalRaw';y=row['value'] if raw else n;A=Q(p['allowance']);candidate=p['candidate'];bounds=[]
   for name,si in laws:
    truth=o.raw(si,row) if raw else si;lower,upper=o.err(y,truth);bounds.append((name,lower,upper))
   lower=max(x[1] for x in bounds);upper=max(x[2] for x in bounds)
   truth_pass=upper<=A;truth_fail=lower>A
   assert truth_pass or truth_fail,(rec['label'],ordinal,p['name'],'UNCERTAIN')
   if candidate:assert truth_pass,(rec['label'],ordinal,p['name'],'FALSE CANDIDATE PASS')
   counts['predicates']+=1;counts['candidate_pass_predicates']+=candidate;counts['conservative_predicates']+=truth_pass and not candidate
   tests.append({'name':p['name'],'allowance':str(A),'candidate_pass':candidate,'old_truth_pass':p['truth_pass'],'full_truth_pass':truth_pass,'error_lower':str(lower),'error_upper':str(upper),'laws':[{'name':name,'error_lower':str(lo),'error_upper':str(hi)} for name,lo,hi in bounds]})
  full_pass=all(p['full_truth_pass'] for p in tests);old_pass=all(p['truth_pass'] for p in prior['proofs'])
  changed=[p['name'] for p in tests if p['full_truth_pass']!=p['old_truth_pass']]
  item={'row':ordinal,'id':row['id'],'kind':row['kind'],'class':v['class'],'scale_bits':v['scale_bits'],'bound_bits':v['bound_bits'],'normalized_bits':v['normalized_bits'],'candidate_pass':v['passed'],'old_truth_pass':old_pass,'full_truth_pass':full_pass,'predicates':tests,'laws':[{'name':name,'si':list(map(str,si)),'raw':list(map(str,o.raw(si,row)))} for name,si in laws]}
  if changed:changes.append({'case':rec['label'],'row':ordinal,'id':row['id'],'changed_predicates':changed,'old_truth_pass':old_pass,'full_truth_pass':full_pass})
  if rec['label']=='interpolated-loaded' and row['id'].endswith(':ux') and row['entity_ref']==b['nodes'][1]['id']:
   direct=next(si for name,si in laws if name=='source_E_0_G_0');Es=b['source'][0];Eh=Q(b['represented'][0]);exact=o.law(rec,b,row,True)
   assert direct==tuple(q*Es/Eh for q in exact)
   lo,hi=o.err(n,direct);A=Q(tests[0]['allowance']);assert lo>A
   decisive['ux']={'row':ordinal,'id':row['id'],'normalized_bits':v['normalized_bits'],'direct_source_resolved_E':list(map(str,direct)),'error_lower':str(lo),'error_upper':str(hi),'allowance':str(A),'ratio_lower':str(lo/A),'error_lower_display':float(lo),'allowance_display':float(A),'ratio_lower_display':float(lo/A),'rescaling_equal':True,'candidate_predicates':v['predicates']}
  if rec['label']=='interpolated-loaded' and row['id'].endswith(':uz') and row['entity_ref']==b['nodes'][1]['id']:
   assert all(si==(Q(0),Q(0)) for _,si in laws) and full_pass and not v['passed']
   decisive['uz']={'row':ordinal,'id':row['id'],'full_error':'0','absolute_bound':tests[0]['allowance'],'bound_bits':v['bound_bits'],'candidate_predicates':v['predicates'],'full_truth_pass':True}
  output.append(item);counts['rows']+=1
 summary={'case':rec['label'],'candidate_numeric_passes':sum(x['candidate_pass'] for x in output),'old_misses':sum(not x['old_truth_pass'] for x in output),'new_misses':sum(not x['full_truth_pass'] for x in output),'old_conservative':sum(x['old_truth_pass'] and not x['candidate_pass'] for x in output),'new_conservative':sum(x['full_truth_pass'] and not x['candidate_pass'] for x in output)}
 a=author_full['cases'][index];assert len(output)==len(a['rows'])==73
 for ours,theirs in zip(output,a['rows']):
  assert (ours['row'],ours['id'],ours['full_truth_pass'])==(theirs['row'],theirs['id'],theirs['full_cover_truth_passed'])
  for op,tp in zip(ours['predicates'],theirs['predicates']):assert(op['name'],op['allowance'],op['full_truth_pass'],op['candidate_pass'])==(tp['name'],tp['allowance'],tp['full_cover_truth_passed'],tp['candidate_passed'])
 results.append({'summary':summary,'E_interval':list(map(str,E)),'G_interval':list(map(str,G)),'material_exactness':profiles,'rows':output})
root=json.loads(files['ROOT_CHECK.json'].read_text());assert Q(root['allowance_exact_m'])==Q(decisive['ux']['allowance'])
q1=list(map(Q,root['q_interval']));q2=list(map(Q,decisive['ux']['direct_source_resolved_E']));assert max(q1[0],q2[0])<=min(q1[1],q2[1])
scratch=Path(manifest['scratch']);bulk=scratch/'FULL_COVER_RESULTS.json';bulk.write_text(json.dumps({'cases':results,'changes':changes,'counts':counts,'decisive':decisive},indent=2)+'\n')
summary_decisive={'ux':{k:v for k,v in decisive['ux'].items() if k in ['row','id','normalized_bits','rescaling_equal','candidate_predicates'] or k.endswith('display')},'uz':decisive['uz']}
summary={'counts':counts,'cases':[r['summary'] for r in results],'changed_rows':changes,'decisive':summary_decisive,'max_material_significant_bits':max(p['significant_bits'] for r in results for p in r['material_exactness'].values()),'all_author_full_cover_decisions_match':True,'old_reviewer_results_exactly_reproduced':True,'author_captures_equal_old_reviewer_log':True,'root_bracket_overlaps':True,'bulk':{'absolute_location':str(bulk),'sha256':hashlib.sha256(bulk.read_bytes()).hexdigest(),'bytes':bulk.stat().st_size}}
(HERE/'RESULT.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps({k:v for k,v in summary.items() if k!='decisive'},indent=2));print('decisive UX',json.dumps({k:v for k,v in decisive['ux'].items() if k.endswith('display')}))
