from review_io import *
import math,struct,re,collections,datetime
from fractions import Fraction as F
checks=collections.Counter();bad=[]
def eq(k,a,b):
 checks[k]+=1
 if a!=b:bad.append({'check':k,'actual':a,'expected':b})
def ok(k,v):eq(k,bool(v),True)
raw=archive(4);outcomes=json.loads((REC/'OUTCOMES.json').read_bytes());first=[o for o in outcomes if o['mode']=='w1a' and o['pass']==1];refsbytes=get(T3+'/REFERENCES/references.json');refs=json.loads(refsbytes)['cases'];inv=packet('ACTUAL_INVOCATION.json');r1inv=packet('R1_INVOCATION.json');r1done=packet('R1_COMPLETION.json');r1reported=packet('R1_RESULTS.json');reported={x['model']:x for x in r1reported['cases']}
eq('reference_hash',hashlib.sha256(refsbytes).hexdigest(),inv['reference_sha256']);cmp_path=inv['comparator_argv'][1].split('/k6c/',1)[1];cmp_bytes=get(cmp_path);eq('comparator_hash',hashlib.sha256(cmp_bytes).hexdigest(),inv['comparator_sha256']);eq('actual_comparator_argv',r1inv['argv'],inv['comparator_argv']);eq('one_comparator_launch',r1inv['launch_count'],1);eq('comparator_exit',r1done['exit_code'],0);eq('comparator_stderr',get(P+'/_run_records/R1.stderr.txt'),b'');eq('comparator_stdout_hash',hashlib.sha256(get(P+'/_run_records/R1.stdout.txt')).hexdigest(),r1done['stdout_sha256'])
selected=[o for o in first if o['outcome']['class']=='Selected'];unpublished=[o for o in first if o['outcome']['class']!='Selected'];expected_names=[o['run_id']+'.rows' for o in selected];eq('four_selected_cases',len(selected),4);eq('two_unpublished_cases',len(unpublished),2)
view=packet('R1_INPUT_VIEW.json');eq('complete_selected_input_roster',sorted(x['name'] for x in view['files']),sorted(expected_names));eq('input_directory_exact_roster',sorted(k.rsplit('/',1)[1] for k in raw if k.startswith('comparison_inputs/W1_T4_R1/')),sorted(expected_names));eq('unpublished_roster',sorted(view['unpublished_not_compared']),sorted(x['model'] for x in unpublished))
DOFS={'UX':0,'UY':1,'UZ':2,'RX':3,'RY':4,'RZ':5}
def node_id(label,model):
 if 'CHAIN' in model:assert label[0]=='N';return int(label[1:])
 assert 'CONT' in model
 return int(label[1:]) if label[0]=='S' else 5000+int(label[1:])
def member_id(label,model):
 if 'CHAIN' in model:assert label[0]=='M';return int(label[1:])
 j=int(label[1:]);return 2*j-1 if label[0]=='A' else 2*j
def requirements(label,model):
 kind,rest=label.split('.',1)
 if kind in ['u','th','R']:
  node,dof=rest.rsplit('.',1);return kind,['%s.%d.%d'%('R' if kind=='R' else 'u',node_id(node,model),DOFS[dof])]
 if kind=='Mb':
  mem,end=rest.rsplit('.',1);i=member_id(mem,model);return kind,(['st.%d.4'%i,'st.%d.5'%i] if end=='mid' else ['end.%d.%s.4'%(i,end),'end.%d.%s.5'%(i,end)])
 i=member_id(rest,model);return kind,['end.%d.j.%d'%(i,0 if kind in ['N','ext'] else 3)]
# The unchanged RF source has 3m CHAIN and CONT half-span elements. The integer
# Q3 transform maps their axial delta (3,0,0) to (1,2,-2), also exactly length3.
# Section operations follow the frozen explicit-product rf_large_section recipe.
od=.2;inner=.18;od2=od*od;id2=inner*inner;area=math.pi*(od2-id2)/4.;second=math.pi*(od2*od2-id2*id2)/64.;J=2.*second;E=200000000000.;G=80000000000.;kt=(G*J)/3.;ka=(E*area)/3.
def magnitude(x,y):
 m=max(abs(x),abs(y))
 if m>2.**500 or (0<m<2.**-500):
  scale=2.**(-600 if m>1 else 600);a=x*scale;b=y*scale;return math.sqrt(a*a+b*b)/scale
 return math.sqrt(x*x+y*y)
case_results=[];value_results=[];custody=[]
for o in first:
 rid=o['run_id'];mid=o['model'];ot=o['outcome'];src='records/'+rid+'.rows'
 if ot['class']!='Selected':eq('unpublished_dump_absent',src in raw,False);eq('unpublished_no_R1_case',mid in reported,False);continue
 ok('Selected_dump_exists',src in raw);b=raw[src];copy=raw['comparison_inputs/W1_T4_R1/'+rid+'.rows'];eq('comparison_copy_exact',copy,b);md=next(x for x in view['files'] if x['name']==rid+'.rows');eq('dump_hash',hashlib.sha256(b).hexdigest(),md['sha256']);eq('dump_bytes',len(b),md['bytes'])
 case=refs[mid];requests=[(label,exp,kind,*requirements(label,mid)) for label,exp,kind in case['expected']];needed={key for row in requests for key in row[-1]};observed={};rows=0;ids=set();classes=collections.Counter();lines=b.decode().splitlines();eq('dump_header',lines[:2],['k6b-rows v1','model '+mid])
 for line in lines[2:]:
  f=line.split();rows+=1;ok('unique_published_key',f[0] not in ids);ids.add(f[0]);classes[f[3]]+=1
  if f[0] in needed:observed[f[0]]=None if f[5].startswith(('underflow','overflow')) else struct.unpack('>d',bytes.fromhex(f[5]))[0]
 eq('complete_dump_row_count',rows,ot['rows'])
 for cl in ['relative_verified','absolute_verified','input_derived','unpublishable']:eq('complete_dump_classes',classes[cl],ot['rows_'+cl])
 expected_section=next(iter(case['model']['sections'].values()));eq('R1_section',expected_section,{'E':'200000000000','G':'80000000000','OD':'0.2','ID':'0.18'})
 scales={k:F(v['value']) for k,v in case['scales'].items()};missing=failures=0;worst=(F(0),None)
 for label,exptext,classkind,q,keys in requests:
  vals=[observed.get(k) for k in keys]
  if any(v is None for v in vals):missing+=1;value_results.append({'model':mid,'label':label,'missing_keys':[k for k,v in zip(keys,vals) if v is None]});continue
  value=magnitude(*vals) if q=='Mb' else vals[0]/kt if q=='tw' else vals[0]/ka if q=='ext' else vals[0];exp=F(exptext);limit=F(1,10**9)*max(abs(exp),scales.get(classkind,F(0)));diff=abs(F(value)-exp);passed=diff<=limit;failures+=not passed;ratio=diff/limit if limit else F(0) if diff==0 else F(10**30)
  if ratio>worst[0]:worst=(ratio,label)
  value_results.append({'model':mid,'label':label,'class':classkind,'source_keys':keys,'observed_float_hex':value.hex(),'exact_expected':exptext,'exact_difference':str(diff),'exact_allowance':str(limit),'pass':passed})
 result={'model':mid,'compared':len(requests)-missing,'failures':failures,'missing':missing,'worst_ratio':float(worst[0]),'worst_ratio_display':'%.3e'%float(worst[0]),'worst_label':worst[1]};case_results.append(result)
 eq('R1_compared',result['compared'],reported[mid]['compared']);eq('R1_failures',failures,reported[mid]['failures']);eq('R1_missing',missing,reported[mid]['missing']);eq('R1_worst_label',worst[1],reported[mid]['worst_label']);eq('R1_worst_display',result['worst_ratio_display'],reported[mid]['reported_worst_allowance_ratio']);custody.append({'model':mid,'run_id':rid,'sha256':hashlib.sha256(b).hexdigest(),'rows':rows,'classes':dict(classes),'input_copy_identical':True})
eq('R1_total636',sum(x['compared'] for x in case_results),636);eq('R1_failures0',sum(x['failures']+x['missing'] for x in case_results),0);eq('total_new_published_rows',sum(x['rows'] for x in custody),1030052)
# Re-read original historical outputs. Unlike hashes remain independent identities.
kf=packet('KF3_SUPPORTED_FACTS.json');seed={x['model']:x for x in map(json.loads,get(H+'/observations/k6b/counts.jsonl',SOURCE).splitlines())};kfresults=[]
for mid,record in kf['models'].items():
 h=record['historical'];path=h['stdout_path'].split('/k6c/',1)[1];bb=get(path);eq('KF3_stdout_hash',hashlib.sha256(bb).hexdigest(),h['stdout_sha256']);oo=list(map(json.loads,bb.splitlines()));w=next(x for x in oo if x['kind']=='w1');co=next(x for x in oo if x['kind']=='counts');start=next(x for x in oo if x['kind']=='start');rec=next(x['record'] for x in oo if x['kind']=='record');current=[x for x in outcomes if x['model']==mid and x['mode']=='w1a']
 eq('KF3_model_SHA_identity',start['model_sha256'],h['canonical_model_sha256']);eq('KF3_source_SHA_identity',co['k4src_sha256'],h['source_k4src_sha256']);eq('KF3_case_identity',w['case'],mid)
 for pair in record['shared_primitive_count_correspondence']:eq('KF3_primitive_value',co[pair['KF3_field']],seed[mid][pair['H_field']]);eq('declared_primitive',co[pair['KF3_field']],pair['KF3_value'])
 for row in current:
  new=row['outcome']
  if w['outcome'].startswith('Selected at '):
   eq('KF3_Selected_class',new['class'],'Selected');eq('KF3_selected_precision',new['selected_precision'],w['selected_precision']);eq('KF3_verification_precision',new['verification_precision'],w['verification_precision']);eq('KF3_published_rows',new['rows'],w['published_rows']);eq('KF3_unpublishable',new['rows_unpublishable'],rec['unpublishable_rows'])
  else:
   eq('KF3_TREE_historical',w['outcome'],'Unresolved Ceiling [Restrained]');eq('KF3_unresolved',new['class'],'Unresolved');eq('KF3_Ceiling',new['reason'],'Ceiling');eq('KF3_absent_selected',new['selected_precision'],w['selected_precision']);eq('KF3_absent_verified',new['verification_precision'],w['verification_precision']);eq('KF3_no_fabricated_rows','rows' in new,False)
 kfresults.append({'model':mid,'historical_outcome':w['outcome'],'current_class':current[0]['outcome']['class'],'current_reason':current[0]['outcome']['reason'],'supported_fields_match':True,'geometry_suffix_unreported_by_H':'[Restrained]' if 'TREE' in mid else None,'unlike_hashes_not_equated':True,'full_historical_value_equality_established':False})
save('COMPARISON_CHECKS.json',{'check_counts':dict(checks),'failures':bad,'R1_cases':case_results,'total_compared':636,'total_new_rows':1030052,'unpublished_models':[x['model'] for x in unpublished],'unpublished_not_evaluated_not_passed':True,'original_comparator_not_run_by_reviewer':True,'model_not_generated':'Only requested published-key index mapping, fixed3m segment length and frozen section arithmetic used. No full nodes/members/coordinates constructed.','checked_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()});save('R1_EXACT_ARITHMETIC.json',value_results);save('PUBLICATION_CUSTODY.json',custody);save('KF3_CORRESPONDENCE.json',kfresults);print(json.dumps({'failures':bad,'R1_cases':case_results,'checks':sum(checks.values())},indent=2))
