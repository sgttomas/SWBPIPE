from pathlib import Path
from fractions import Fraction as F
import os,sys,subprocess,hashlib,json,struct,re,datetime
ROOT=Path(sys.argv[1]).resolve();WT=ROOT.parent;ENV=dict(os.environ,GIT_OPTIONAL_LOCKS='0',PYTHONDONTWRITEBYTECODE='1');T3=Path('projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3');R=T3/'RESUME_2026-09-30';RV=WT/'numerics'/T3/'REVIEW/RESUME_RECORDS_RV27';OUT=RV/'escalation_729e80b5';EVID='a6b40d2d036acac556e28f28f5b482a4adb39333';HEAD='729e80b5c2a33b278623a850d9c75c25372b688c';BASE='ea2aaee702624cd612eee23a7e1969fd3c670d3f';SOURCE='3bddc2b05f6106e969c7cf43373b230845c7cc66';cache={}
def git(*args):return subprocess.check_output(['git',*args],cwd=ROOT,env=ENV)
def blob(p,ref=EVID):
 key=(str(p),ref)
 if key not in cache:cache[key]=git('show',ref+':'+str(p))
 return cache[key]
def read(p):return blob(R/p)
def sha(b):return hashlib.sha256(b).hexdigest()
def file_sha(p):return sha(p.read_bytes())
def bits(s):return F.from_float(struct.unpack('>d',bytes.fromhex(s))[0])
def frac(j):return F(int(j['significand']))*F(2)**j['exponent2']
def manifest(p):
 data=read(p);fail=[];lines=data.decode().splitlines()
 for line in lines:
  h,rel=line.split(None,1);rel=rel.strip();b=read(str(Path(p).parent/rel))
  if sha(b)!=h:fail.append(rel)
 return {'origin':EVID+':'+str(R/p),'sha256':sha(data),'entries':len(lines),'failures':fail}
def local_seal(p):
 fail=[];lines=p.read_text().splitlines()
 for line in lines:
  h,r=line.split(None,1)
  if file_sha(p.parent/r.strip())!=h:fail.append(r.strip())
 return {'entries':len(lines),'sha256':file_sha(p),'failures':fail}
res={'candidate':HEAD,'actual_head':git('rev-parse','HEAD').decode().strip(),'base':BASE,'evidence_commit':EVID,'status':git('status','--porcelain=v1').decode(),'python':sys.version}
paths=git('diff','--name-only',BASE,HEAD).decode().splitlines();res['delta_paths']=paths
res['unchanged_scope_diff']=git('diff','--name-only',BASE,HEAD,'--','projects/chirality-piping/core','projects/chirality-piping/validation','tools','.github','AGENTS.md','agents','.agents',str(R/'_run_records'),str(T3/'AUDIT')).decode()
res['prior_seals']={x:local_seal(RV/x/'SHA256SUMS') for x in ['.','backcheck_de9f8d83','mergecheck_ea2aaee7']};res['old_preservation_seal']=local_seal(ROOT/R/'_run_records/aud_t3_04/SHA256SUMS')
res['witness_seals']=[manifest(p) for p in ['I22/build_01/SHA256SUMS','I22/b_01/SHA256SUMS','I22/c_01/SHA256SUMS']]
res['oracle_manifests']=[]
for p in ['oracle_fresh/FREEZE_MANIFEST.json','oracle_fresh/addendum_01/MANIFEST.json','oracle_fresh/addendum_02_C17/MANIFEST.json']:
 d=json.loads(read(p));fail=[]
 for row in d['files']:
  b=read('oracle_fresh/'+row['path'])
  if sha(b)!=row['sha256'] or ('bytes' in row and len(b)!=row['bytes']):fail.append(row['path'])
 res['oracle_manifests'].append({'path':p,'sha256':sha(read(p)),'entries':len(d['files']),'failures':fail})
res['raw_hashes']={p:sha(read(p)) for p in ['I22/c_01/cases/C17/stdout.tsv','oracle_fresh/addendum_02_C17/stdout.tsv','oracle_fresh/TRUTH.json','oracle_fresh/exact_oracle.py','oracle_fresh/addendum_01/validate_compare.py']}
res['scratch_binary_sha256']=file_sha(WT/'scratch/i22/target/debug/a1_public_probe')
res['scratch_c17_raw_sha256']=file_sha(WT/'scratch/i22/c_01/C17/stdout.tsv')
res['build_scratch_manifest']={'entries':0,'failures':[]}
for line in read('I22/build_01/SCRATCH_SHA256SUMS').decode().splitlines():
 h,p=line.split(None,1);p=p.strip();res['build_scratch_manifest']['entries']+=1
 if file_sha(WT/'scratch/i22'/p)!=h:res['build_scratch_manifest']['failures'].append(p)
res['source_archive_against_git']={'entries':0,'failures':[]}
for line in read('I22/build_01/SOURCE_SHA256SUMS').decode().splitlines():
 h,p=line.split(None,1);p=p.strip();res['source_archive_against_git']['entries']+=1
 if sha(blob(p.removeprefix('source/'),SOURCE))!=h:res['source_archive_against_git']['failures'].append(p)
res['FK_trees']={ref:git('rev-parse',ref+':projects/chirality-piping/core/solver/frame_kernel').decode().strip() for ref in [SOURCE,'d01ad98a754698631f927709d08284c272de85e8','a38617d08bfbff6ea0a15b1cb129a16cbc82b2fc',EVID,HEAD]}
response='projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260930-T3-AUDIT-RESPONSE/instances/A1-DIAGNOSIS/'
res['probe_bindings']={p:sha(read('oracle_fresh/inputs/'+p))==sha(blob(response+'src/'+p,'520d7dfb790bcedabc03e92b9692884ce295be54'))==file_sha(WT/'scratch/i22/probe/src'/p) for p in ['main.rs','cases.rs']}
res['original_lock_matches']=file_sha(WT/'scratch/i22/probe/Cargo.lock')==sha(blob(response+'continuation_01/Cargo.lock','520d7dfb790bcedabc03e92b9692884ce295be54'))
res['features']={p:json.loads(read('I22/build_01/'+p)) for p in ['FEATURES_KERNEL.json','FEATURES_PROBE.json']}
source=json.loads(read('I22/c_01/C17_INPUT.json'))['primitive'];truth=next(c for c in json.loads(read('oracle_fresh/TRUTH.json'))['cases'] if c['id']=='C17')
loads={g:sum((bits(x['value_bits']) for x in source['loads'] if x['global_dof']==g),F(0)) for g in [0,6,9]};k=bits(source['springs'][0]['stiffness_bits']);length=bits(source['nodes_bits'][1][0]);member=source['members'][0];a=bits(member['elastic_modulus_bits'])*bits(member['area_bits'])/length;t=bits(member['shear_modulus_bits'])*bits(member['torsion_constant_bits'])/length
u0=(loads[0]+loads[6])/k;u6=u0+loads[6]/a;u9=loads[9]/t
assert loads[0]==a*(u0-u6)+k*u0 and loads[6]==a*(u6-u0) and loads[9]==t*u9
raw=[l.split('\t') for l in read('I22/c_01/cases/C17/stdout.tsv').decode().splitlines()];rows={x[1]:x for x in raw if x[0]=='ROW'};layout={x[1]:x for x in raw if x[0]=='LAYOUT'};headers={key:[x[1:] for x in raw if x[0]==key] for key in ['FORMAT','CASE','SOURCE_COMMIT','LIMITS','STATUS','SELECTED']}
res['independent_force_balance']={'sum_axial_loads_equals_9h':loads[0]+loads[6]==9*F(2)**-1074,'k_equals_2powminus33':k==F(2)**-33,'u0_equals_9_times_2powminus1041':u0==9*F(2)**-1041,'member_axial_equations_zero_residual':True,'other_node0_translations_constrained':all(g in [c['global_dof'] for c in source['constraints']] for g in [1,2]),'rows':[]}
for key in ['D:0','M:0']:
 row=rows[key];expect=u0 if key=='D:0' else abs(u0);bound=bits(row[6]);val=bits(row[4]);allowed=bound*(1+F(2)**-22);binary64_allowed=F.from_float(float(bound)*(1+2**-22));fr=next(x for x in truth['rows'] if x['key']==key)
 res['independent_force_balance']['rows'].append({'key':key,'raw':row,'layout':layout[key],'frozen_truth_matches':frac(fr['truth'])==expect,'zero_published':val==0,'bound_2powminus1038':bound==F(2)**-1038,'error_over_bound':str(abs(expect-val)/bound),'error_over_qualified_allowance':str(abs(expect-val)/allowed),'binary64_allowance_exact':binary64_allowed==allowed,'exact_claim_fails':abs(expect-val)>allowed,'binary64_claim_fails':abs(expect-val)>binary64_allowed})
res['headers']=headers;res['matching_source_encoding']=next(x[1] for x in raw if x[0]=='SOURCE_ENCODING')==next(x[1] for x in raw if x[0]=='SOURCE_ENCODING_SELECTED');res['layout_rows']=len(layout);res['published_rows']=len(rows)
case=json.loads(read('I22/c_01/cases/C17/CASE_RESULT.json'));freeze=json.loads(read('oracle_fresh/FREEZE_MANIFEST.json'));add=json.loads(read('oracle_fresh/addendum_02_C17/MANIFEST.json'))
res['time_order']={'frozen_utc':freeze['frozen_utc'],'case_launch':case['launch_utc'],'confirmation_sealed_utc':add['sealed_utc'],'freeze_before_launch':datetime.datetime.fromisoformat(freeze['frozen_utc'])<datetime.datetime.fromisoformat(case['launch_utc'].replace('Z','+00:00')),'frozen_claim_outputs_read_before_freeze':freeze['actual_solver_outputs_read_before_freeze'],'limit':'Preserved read-order record and timestamp consistency, not independent access to the original oracle conversation.'}
res['run_scope']={'run_exit':case['run']['exit_code'],'comparison_exit':case['compare']['exit_code'],'run_argv':case['run']['cmd'],'guard_before_exit':case['guard_before']['exit_code'],'guard_after_exit':case['guard_after']['exit_code'],'committed_c_case_directories':sorted(set(p.split('/cases/')[1].split('/')[0] for p in git('ls-tree','-r','--name-only',EVID,'--',str(R/'I22/c_01/cases')).decode().splitlines())),'scratch_c_directories':sorted(p.name for p in (WT/'scratch/i22/c_01').iterdir() if p.is_dir())}
res['evidence_scope_changed_paths']=git('diff','--name-only','d01ad98a754698631f927709d08284c272de85e8',EVID).decode().splitlines()
res['ruling_preserves_prior_bytes']=(ROOT/T3/'ROOT_RULINGS_V1.md').read_bytes().startswith(blob(T3/'ROOT_RULINGS_V1.md',BASE))
res['graph_history_preserved']=(ROOT/'projects/chirality-piping/execution/_Coordination/WorkGraphs/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/WORK_GRAPH.md').read_text().split('**Historical progression:**',1)[1].split('\n',1)[0]==blob('projects/chirality-piping/execution/_Coordination/WorkGraphs/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/WORK_GRAPH.md',BASE).decode().split('**Historical progression:**',1)[1].split('\n',1)[0]
res['design_brief_hash']=file_sha(ROOT/R/'BRIEFS/A1_DESIGN.md');res['design_dispatch']=json.loads((ROOT/R/'DISPATCH_UPDATES.jsonl').read_text().splitlines()[-1]);res['status_after']=git('status','--porcelain=v1').decode()
(OUT/'evidence/CHECKS.json').write_text(json.dumps(res,indent=2)+'\n');(OUT/'evidence/delta.patch').write_bytes(git('diff','--no-ext-diff',BASE,HEAD));(OUT/'evidence/CONSULTED_BLOBS.json').write_text(json.dumps([{'origin':ref+':'+p,'sha256':sha(b),'bytes':len(b)} for (p,ref),b in sorted(cache.items())],indent=2)+'\n')
print(json.dumps({k:v for k,v in res.items() if k not in ['features','evidence_scope_changed_paths','design_dispatch','FK_trees']},indent=2));print('evidence paths',len(res['evidence_scope_changed_paths']),'unexpected',[p for p in res['evidence_scope_changed_paths'] if not p.startswith(str(R)+'/')])
