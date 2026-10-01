"""Supplemental sealed input/profile/launch/summary and dominance check.
No product, model, graph, parser, solver or author-calculator execution.
"""
from pathlib import Path
import hashlib,json,os,subprocess,sys
K,NUM,OUT=map(Path,sys.argv[1:]);raw=OUT/'_run_records';WT=K.parent
T='projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3';R=T+'/RESUME_2026-09-30';P=R+'/metric_design_10_vr_numbers'
def sha(b):return hashlib.sha256(b).hexdigest()
def load(path):return json.loads((K/path).read_text())
checks=[];records=[]
def ck(name,actual,expected=True):
 assert actual==expected,(name,actual,expected)
 checks.append(name)
bindings=load(P+'/_run_records/SOURCE_BINDING.json')
for b in bindings:
 origin=b['origin']
 if origin.startswith('<K6C>/'):
  p=K/origin.removeprefix('<K6C>/');ck('bound source '+origin,sha(p.read_bytes()),b['sha256']);records.append(b)
packets=['source_review_RV30/vr_nodes_17','source_review_RV30/public_layout_result_22','source_review_RV30/format_stream_18','source_review_RV30/static_pools_20','source_review_RV30/wrapped_errors_19','design_review_RV28/vr_prefix_correction_03','design_review_RV28/f64_parse_04','design_review_RV28/expected_list_parse_05','source_review_RV30/exact_11','source_review_RV30/sparse_13','source_review_RV30/sparse_correction_14']
seals=[]
for rel in packets:
 b=(K/R/rel/'SHA256SUMS').read_bytes();n=0
 for line in b.decode().splitlines():
  h,path=line.split(maxsplit=1);p=K/R/rel/path
  if not p.exists():p=K/path
  ck('prior '+rel+'/'+path,sha(p.read_bytes()),h);n+=1
 seals.append({'path':R+'/'+rel,'sha256':sha(b),'entries':n})
choice=load(P+'/_run_records/LAUNCH_CHOICE.json');cp=R+'/_run_records/final_numeric_launch_choice_01/CHOICE.json'
p=subprocess.run(['git','show','8fdfd8304ad3169168f6ae414babee74a2a19b50:'+cp],cwd=NUM,env=dict(os.environ,GIT_OPTIONAL_LOCKS='0'),capture_output=True)
ck('immutable launch read',p.returncode,0);ck('launch choice immutable equality',json.loads(p.stdout),choice)
launch=load(P+'/LAUNCHES.json');table=load(P+'/VR_CALLER_TABLE.json');calc=json.loads((raw/'RECOMPUTED.json').read_text());byid={x['id']:x for x in table['rows']}
ck('unique24',len(byid),24)
ext=0
for x in launch['rows']:
 for argvkey,lenkey,counts in [('normal_argv','argument_utf8_lengths',False),('counts_argv','counts_argument_utf8_lengths',True)]:
  argv=[a.replace('<WT>',str(WT)) for a in x[argvkey]];ck(x['id']+' '+argvkey+' utf8',list(map(lambda z:len(z.encode()),argv)),x[lenkey]);ck(x['id']+' binary',argv[0],choice['VR_binary']);ck(x['id']+' flags',argv[1:5],['--case',x['id'],'--heap-cap-bytes','536870912' if counts else '8053063680'])
  if '--model-file' in argv:
   at=argv.index('--model-file');ck(x['id']+' original model path',argv[at+1],str(WT/'scratch/i17/b_models'/str(x['id']+'.json')))
  ck(x['id']+' counts flag',('--counts-only' in argv),counts)
 ck(x['id']+' cwd',x['cwd'].replace('<WT>',str(WT)),choice['cwd']);ck(x['id']+' manifest',x['CARGO_MANIFEST_DIR'].replace('<WT>',str(WT)),choice['VR_CARGO_MANIFEST_DIR']);ext+=int(x['external_model_path_utf8_bytes']>0)
ck('external12',ext,12)
dominance=[];group={}
for row in calc:
 id=row['id'];e=row['metric'];ks=row['kernel_addends'];prior=row['prelist_prior_failure_upper'];actual=byid[id][e]
 ck(id+e+' five interfaces',len(ks),5);ck(id+e+' no implicit unbound source',actual['unbound_source_addends'],{})
 f1=ks['expected_list_initialization_while_outcome_alive']+prior;f2=ks['nonselected_diagnostic_while_outcome_alive']+78
 shared=max(ks[k] for k in ks if k!='solve_max')
 ck(id+e+' F1 complete joined dominance',f1<=shared);ck(id+e+' F2 complete joined dominance',f2<=shared)
 dominance.append({'id':id,'metric':e,'F1_original_addend':ks['expected_list_initialization_while_outcome_alive'],'F1_preexisting_failures_upper':prior,'F1_corrected_candidate':f1,'F2_original_addend':ks['nonselected_diagnostic_while_outcome_alive'],'F2_LIST_retained':78,'F2_corrected_candidate':f2,'original_max_Kernel_outcome_addend':shared,'new_max_if_corrected':max(shared,f1,f2),'F1_margin':shared-f1,'F2_margin':shared-f2})
 parts=id.split('-');fam=parts[2];size=byid[id]['members'];g=group.setdefault((fam,size),{'requested':0,'moving':0,'dominants':set()});g[e]=max(g[e],actual['caller_only_max']);g['dominants'].add(actual['dominant_caller_phase'])
 if e=='moving':
  for key,v in actual['caller_only_phases'].items():ck(id+' move>=request '+key,v>=byid[id]['requested']['caller_only_phases'][key])
 ck(id+e+' integer bound range',all(0<=v<2**64 for v in list(row['phases'].values())+list(ks.values())))
summary=[]
for (fam,size),g in sorted(group.items()):summary.append({'family':fam,'members':size,**{k:v for k,v in g.items() if k!='dominants'},'dominants':sorted(g['dominants'])})
backstops=[]
for x in table['rows']:
 f=x['shape']['f'];v=4*f*(f+1);ck(x['id']+' triangular values',v,x['requested']['details']['sparse']['factor_values']);ck(x['id']+' backstop boolean',v>4026531840,x['coarse_factor_value_subterm_exceeds_backstop'])
 if x['members']==10000:backstops.append({'id':x['id'],'factor_values':v,'backstop':4026531840,'exceeds':v>4026531840})
ck('six10000',len(backstops),6);ck('six coarse deferrals',all(x['exceeds'] for x in backstops))
noop=load(P+'/NOOP_BRANCH.json');nv=[choice['VR_binary'],'--noop','--heap-cap-bytes','8053063680'];argc=24*len(nv)+sum(len(x.encode()) for x in nv);start=632+23+17+5+97 # base noop keys from SCHEMAS, overwritten below exact schema read
sch=load(R+'/I21/source_15/SCHEMAS.json')['schemas']['noop_start'];start=632+sch['key_bytes']+17+5+97
ck('noop bound',max(1700+argc,1700+start,1700+632+50),noop['requested_and_move_source_upper'])
(raw/'SUPPLEMENTAL_CHECKS.json').write_text(json.dumps({'status':'PASS','count':len(checks),'checks':checks,'preserved_seals':seals,'source_bindings':records,'launch_choice_read':{'argv':['git','show','8fdfd8304ad3169168f6ae414babee74a2a19b50:'+cp],'stdout_sha256':sha(p.stdout),'exit':p.returncode,'GIT_OPTIONAL_LOCKS':'0'}},indent=2)+'\n')
(raw/'DOMINANCE_AND_SUMMARY.json').write_text(json.dumps({'phase_findings':dominance,'summary':summary,'coarse_backstops':backstops,'no_whole_bound_or_admission':True},indent=2)+'\n')
print(json.dumps({'status':'PASS','checks':len(checks),'prior_seals':len(seals),'min_F1_joined_margin':min(x['F1_margin'] for x in dominance),'min_F2_joined_margin':min(x['F2_margin'] for x in dominance),'no_numeric_joined_max_change':all(x['new_max_if_corrected']==x['original_max_Kernel_outcome_addend'] for x in dominance)}))
