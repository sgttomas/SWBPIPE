"""Independent caller14 check using this reviewer's earlier source-equation
calculator, extended for explicit generic springs and per-family reference inputs.
No author calculator or product/source algorithm is executed.
"""
from pathlib import Path
import json,hashlib,copy as deepcopy_module,sys
import reviewer_math as math
from reviewer_math import *
K,OUT=map(Path,sys.argv[1:]);raw=OUT/'_run_records';WT=K.parent
T='projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3';R=T+'/RESUME_2026-09-30';P=R+'/metric_design_14_reference_callers';VR='projects/chirality-piping/validation/benchmarks/numerical_robustness'
checks=[];mismatches=[];basis=[]
def sha(b):return hashlib.sha256(b).hexdigest()
def rd(path):
 b=(K/path).read_bytes();basis.append({'origin':'<K6C>','path':path,'sha256':sha(b),'bytes':len(b)});return b
def jo(path):return json.loads(rd(path))
def ck(n,a,b=True):
 checks.append(n)
 if a!=b:mismatches.append({'check':n,'actual':a,'expected':b})
seals=[]
for packet,expected_seal in [(P,'235c4a77a29789952587c9173bcf42e291540d87ccdde7719bc3dc5d56c2cbb2'),(R+'/design_review_RV28/api_contract_09','eac38b5e0e8cdcfed26d935050b88899ee84a6693b42ad29870a65899366ec43'),(R+'/design_review_RV28/vr_numbers_06','cd7e0f6108c88245d8bb729c381528644bcfba0cbb292aca225176a74fcda626'),(R+'/design_review_RV28/vr_correction_07','077f81000a0c4862e0dd84390897c4fd5baa2237140c9035439e52e6e7bf10b2'),(R+'/source_review_RV30/reference_reach_26','74b879c4013f108aad5ceff8ccd8a5791d2e422fa91fa8b8fb4f31fe05edd497'),(R+'/design_review_RV28/selected_numerators_01','82d1ae8902a5e5e9f9147ebe5177f16ec20aa050dfb61a3af4f38373bbe3c84a')]:
 s=rd(packet+'/SHA256SUMS');ck('seal '+packet,sha(s),expected_seal);num=0
 for line in s.decode().splitlines():
  h,f=line.split(maxsplit=1);p=K/packet/f
  if not p.exists():p=K/f
  ck('payload '+packet+'/'+f,sha(p.read_bytes()),h);num+=1
 seals.append({'packet':packet,'sha256':sha(s),'entries':num})
for p in ['METHOD_AND_JOIN.md','RETURN.md','SUMMARY.md','_run_records/TRIPLE_BINDING.json','_run_records/FINITE_BINDING.json']:
 rd(P+'/'+p)
for p in ['metric_design_13_api_contract/CONTRACT.md','design_review_RV28/api_contract_09/REVIEW.md','source_review_RV30/reference_reach_26/RETURN.md','design_review_RV28/selected_numerators_01/REVIEW.md']:
 rd(R+'/'+p)
math.schema=jo(R+'/I21/source_15/SCHEMAS.json')['schemas']
meta=jo(R+'/I21/source_09/FIXTURE_INPUTS.json');md={v['id']:v for v in meta['vr_rows']};roster=jo(R+'/metric_design_13_api_contract/_run_records/ROSTER_CONTEXT.json')['rows'];sto={v['id']:v for v in roster}
candidate=jo(P+'/REFERENCE193_CALLERS.json');proposed={v['id']:v for v in candidate['rows']};family_candidate={v['family']:v for v in jo(P+'/FAMILY10_PREFIXES.json')['families']}
launches=jo(P+'/REFERENCE_LAUNCHES.json');launches['rows']=sorted(launches['rows'],key=lambda v:next(i for i,r in enumerate(roster) if r['id']==v['id']))
groups={};models={};allcases={};casebounds={};familyraw={};files={};audit=[]
duplicates=[]
def pairs(p):
 d={}
 for k,v in p:
  if k in d:duplicates.append(k)
  d[k]=v
 return d
def scalarwalk(v,stats):
 if isinstance(v,str):stats['strings']+=1;stats['nonascii']+=not v.isascii()
 elif isinstance(v,list):
  for x in v:scalarwalk(x,stats)
 elif isinstance(v,dict):
  for k,x in v.items():scalarwalk(k,stats);scalarwalk(x,stats)
 elif type(v) is int:stats['ints']+=1;ck('integer scalar range',0<=v<2**64)
 elif isinstance(v,float):stats['floats']+=1
for f in meta['vr_files']:
 b=rd(f['path']);ck('family identity '+f['path'],sha(b),f['sha256']);cc=[json.loads(line,object_pairs_hook=pairs) for line in b.splitlines() if line];fam=cc[0]['family'];ck(fam+' uniform',all(c['family']==fam for c in cc));ck(fam+' unescaped',b'\\' not in b)
 stats={'strings':0,'nonascii':0,'ints':0,'floats':0};scalarwalk(cc,stats);ck(fam+' nonascii/float',(stats['nonascii'],stats['floats']),(0,0));audit.append({'family':fam,'bytes':len(b),**stats});groups[fam]=cc;familyraw[fam]=b;files[fam]=Path(f['path']).name
 for c in cc:allcases[c['id']]=c;casebounds[c['id']]=caseheap(c)
ck('no duplicate JSON object keys',duplicates,[]);ck('213 original IDs',len(allcases),213)
cases=[allcases[v['id']] for v in roster];ck('193 roster unique',len(cases)==len(set(sto))==len(proposed)==193)
ck('roster identity',set(sto),set(proposed));ck('ten family identity',set(groups),set(family_candidate))
descriptors={}
for c in cases:
 id=c['id'];m=c['model'];models[id]=m;N=len(m['nodes']);members=len(m['members']);s=sum(x[4] is not None for x in m['springs']);d=len(m['springs'])-s;r=len(m['constraints']);loads=len(m['loads']);station=len(m['stations']);ids=sum(len(x[3].encode()) for x in m['loads']);n=6*N;f=n-r
 ck(id+' primitive census',(N,members,s,d,r,loads,station),tuple(md[id][k] for k in ['N','m','s','d','r','l','t']));ck(id+' unique restraints',r,md[id]['r_unique']);ck(id+' ID bytes',ids,md[id]['source_id_bytes'])
 desc=dict(N=N,m=members,s=s,d=d,u=0,r=r,l=loads,t=station,n=n,f=f,z=sto[id]['committed_pattern_entries'],h_W1_unused_for_sparse=sto[id]['committed_profile_entries'],q=7*N+12*members+6*station+s+3*d+r,X=38+24*N+84*members+17*s+41*d+13*r+17*loads+ids+16*station,load_id_bytes=ids,B=N,b=f)
 for key,value in desc.items():ck(id+' shape '+key,value,proposed[id]['shape'][key])
 descriptors[id]=desc
manifest='/reference/projects/chirality-piping/validation/benchmarks/numerical_robustness';binary='/reference/target/release/examples/vk_scale';expected_path=manifest+'/cases/expected_unresolved.json';Family={}
for fam,cc in groups.items():
 Family[fam]={}
 for e,metric in [(0,'requested'),(1,'moving')]:
  prev=0;best=0;nf=len(cc);F=len(familyraw[fam]);path=manifest+'/cases/'+files[fam];paths=len(manifest)+arbitrary(1,len(manifest+'/cases'))+arbitrary(1,len(path))+e*len(path)
  for c in cc:
   r,tr,tm=casebounds[c['id']];jr,oldj=jtree(c);best=max(best,prev+max(jr+e*oldj,jr+(tm if e else tr),r+e*old(472,nf)));prev+=r
  value=paths+max(readfile(F,e),max(8,2*(F+32))+pushed(472,nf)+best);Family[fam][e]=value
  values={'family_load_upper':value,'path_construction':paths,'all_typed_case_retained':prev,'case_vec_backing':pushed(472,nf),'parse_typed_prefix_union':best,'raw_file_bytes':F,'scratch_string_retained':0}
  for key,v in values.items():ck(fam+' '+metric+' prefix '+key,v,family_candidate[fam]['metrics'][metric][key])
expected=rd(VR+'/cases/expected_unresolved.json');ck('expected list identity',sha(expected),'2e5d0975a90c69e5d198d61760cc7734d42615461710da1fe226f60b4a46f2c9');jexpect,oexpect=jtree(json.loads(expected))
external={};external_data={};results=[]
# This driver was derived from the reviewer's own previously sealed calculation,
# never from or by executing the author ARITHMETIC.py.
exec(compile((raw/'reviewer_driver.py').read_text(),'reviewer_driver.py','exec'),globals())
cli=jo(P+'/CLI24_CALLERS.json');basecli=jo(R+'/I21/vr_join_20/CORRECTED_CALLER_TABLE.json');ck('CLI24 ID roster',[r['id'] for r in cli['rows']],[r['id'] for r in basecli['rows']])
for new,prior in zip(cli['rows'],basecli['rows']):
 for metric in ['requested','moving']:
  stripped=deepcopy_module.deepcopy(new[metric]);stripped.pop('result5_components');ck(new['id']+metric+' CLI values unchanged',stripped,prior[metric])
  c=allcases[new['id']];ctrl=2*pushed(24,len(c['controls']))+sum(fmt(len(c['id'])+1+len(x[0])) for x in c['controls']);d=new[metric]['details'];X=new['shape']['X'];v=d['cut_survivors']+arbitrary(1,X)+78+ctrl+d['published_map'];r5=new[metric]['result5_components'];ck(new['id']+metric+' CLI result5',(r5['fixed_caller_retained_addend'],r5['model_retained']),(v,d['model_standalone']+d['published_map']))
ck('CLI launch unchanged',rd(P+'/CLI24_LAUNCHES.json'),rd(R+'/metric_design_10_vr_numbers/LAUNCHES.json'))
join=jo(P+'/KERNEL_JOIN_INPUTS.json');ck('reference join interface rows',len(join['rows']),193)
contexts={(r['context'],r['id']):r for r in join['rows']};ck('unique reference context/ID',len(contexts),193)
for ctx,rows in [('SingleCaseFamilyReferenceV1',candidate['rows']),('ActualRF_LARGE_CLI24',cli['rows'])]:
 if ctx not in {k[0] for k in contexts}:continue
 for row in rows:
  j=contexts[ctx,row['id']]
  for metric in ['requested','moving']:ck(row['id']+metric+' join addends',j[metric]['kernel_addends'],row[metric]['kernel_phase_addends'])
summary={}
for fam in groups:
 rows=[v for v in candidate['rows'] if allcases[v['id']]['family']==fam];summary[fam]={'cases':len(rows),'requested':max(v['requested']['caller_only_max'] for v in rows),'moving':max(v['moving']['caller_only_max'] for v in rows)}
(raw/'BASIS.json').write_text(json.dumps({'agent':'/root/rv28_a1_design','parent':'/root','role':'TASK','mechanism':'collaboration.followup_task','source_records':basis,'preserved_seals':seals,'equation_origin':'Reviewer-owned vr_numbers_06/_run_records/rederive.py, explicit generic adaptations preserved locally','scope':'No author calculator, production/source algorithm or kernel join'},indent=2)+'\n')
(raw/'CHECKS.json').write_text(json.dumps({'status':'PASS' if not mismatches else 'MISMATCH','checks':len(checks),'mismatch_count':len(mismatches),'mismatches':mismatches},indent=2)+'\n')
(raw/'RECOMPUTED.json').write_text(json.dumps(results,indent=2)+'\n');(raw/'SUMMARY.json').write_text(json.dumps({'family_summary':summary,'input_audit':audit,'reference_cases':193,'metrics':386,'CLI24_unchanged':True,'joined_kernel':False},indent=2)+'\n')
print(json.dumps({'checks':len(checks),'mismatch_count':len(mismatches),'first_mismatches':mismatches[:6],'reference_cases':193,'metric_rows':len(results),'caller_max_requested':max(r['requested']['caller_only_max'] for r in candidate['rows']),'caller_max_moving':max(r['moving']['caller_only_max'] for r in candidate['rows'])}))
