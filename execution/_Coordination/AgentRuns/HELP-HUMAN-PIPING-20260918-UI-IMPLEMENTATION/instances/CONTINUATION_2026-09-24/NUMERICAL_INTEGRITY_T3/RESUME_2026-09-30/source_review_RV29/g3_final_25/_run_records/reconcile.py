from pathlib import Path
import subprocess,os,json,hashlib,collections,datetime
W=Path('/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3');A=W/'a1';R=Path('projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30');Q=A/R/'source_review_RV29/g3_final_25/_run_records';rev='9ef9508dea8205297631299b3c384a3cd5185bef';env=dict(os.environ,GIT_OPTIONAL_LOCKS='0');H=lambda b:hashlib.sha256(b).hexdigest()
def git(*a):return subprocess.check_output(['git','-C',str(A),*a],env=env)
def blob(p,r=rev):return git('show',r+':'+str(p))
ledger=json.loads(blob(R/'REVIEW_EVIDENCE_RELOCATION_2026-10-01.json'))
def mapped(p):
 s=str(p)
 for x in ledger['packets']:
  if s.startswith(x['old_prefix']):return x['new_prefix']+s[len(x['old_prefix']):]
 return s
# Check modes too: exact byte-preserving moves must retain Git mode.
def entries(r):
 data=git('ls-tree','-r','-z',r,str(R/'source_review_RV29')).split(b'\0');d={}
 for line in data:
  if not line:continue
  info,name=line.split(b'\t',1);mode,kind,oid=info.decode().split();d[name.decode()]=(mode,kind,oid)
 return d
old=entries(ledger['source_revision_before_relocation']);new=entries(rev)
for x in ledger['packets']:
 for n in list(x['payload_hashes'])+['SHA256SUMS']:assert old[x['old_prefix']+n]==new[x['new_prefix']+n]
manifest=json.loads(blob(R/'verification/g3_preparation_01/MANIFEST.json'));gs={g['runtime_id']:g for g in manifest['entries']};assert len(gs)==52
review_seals={x['new_prefix']:x['packet_seal_sha256'] for x in ledger['packets']};checks=[]
# Replay actual records rather than re-counting cumulative VERDICTS snapshots.
packets=['I22/protected_g3/runtime_01','I22/protected_g3/runtime_02','I22/protected_g3/runtime_03','I22/tail_runtime_01','I22/protected_g3/runtime_04','I22/protected_g3/runtime_05','I22/r33_named/runtime_01','I22/protected_g3/runtime_06'];allruns=[];unique_logs=set();allbins=set();counts=collections.Counter();packetrows=[]
for pp in packets:
 p=R/pp;c=json.loads(blob(p/'COMMANDS.json'));mb=blob(p/'SHA256SUMS');lines=mb.decode().splitlines()
 for l in lines:
  h,n=l.split(None,1);assert H(blob(p/Path(n.strip())))==h
 pc=collections.Counter()
 for run in c['runs']:
  assert run['log'] not in unique_logs,(pp,run['log']);unique_logs.add(run['log']);allbins.add(run['binary']['path']);kind=run['kind'];pc[kind]+=1;counts[kind]+=1;allruns.append({'packet':pp,**run})
 packetrows.append({'packet':str(p),'seal':H(mb),'payloads':len(lines),'commands':len(c['runs']),'kinds':dict(pc)})
qualified=[]
for i in range(1,53):
 id=f'R{i:02}';g=gs[id]
 if i<=8 or 11<=i<=14:review='g3_prefix_tail_17';selected=[r for r in allruns if r['id']==id and r['kind']=='mutant']
 elif i in [9,10]:
  review='g3_prefix_tail_17';fault='K4-M2' if i==9 else 'K4-M3';selected=[r for r in allruns if r['packet']=='I22/tail_runtime_01' and r['id']==fault and r['kind']=='mutant']
 elif i<=32:review='g3_mid_21';selected=[r for r in allruns if r['id']==id and r['kind']=='mutant']
 elif i==33:review='r33_runtime_24';selected=[r for r in allruns if r['packet']=='I22/r33_named/runtime_01' and r['kind']=='mutant']
 elif i<=45:review='g3_runtime05_23';selected=[r for r in allruns if r['id']==id and r['kind']=='mutant']
 elif i==46:review='g3_final_25';selected=[r for r in allruns if r['id']==id and r['kind']=='historical_mutant']
 else:review='g3_final_25';selected=[r for r in allruns if r['id']==id and r['kind']=='mutant']
 assert len(selected)==(2 if i==52 else 1),(id,len(selected));predicates=[]
 for run in selected:
  assert run['tool_completion']['exit_code']==101
  ctrlkind='historical_control' if i==46 else 'control';controls=[r for r in allruns if r['packet']==run['packet'] and r['id']==run['id'] and r['kind']==ctrlkind and r['filter']==run['filter'] and r['launch_utc']>run['completed_utc']];assert len(controls)==1 and controls[0]['tool_completion']['exit_code']==0
  predicates.append({'filter':run['filter'],'mutant_packet':str(R/run['packet']),'mutant_evidence':run['evidence'],'mutant_binary_sha256':run['binary']['sha256'],'control_evidence':controls[0]['evidence'],'control_binary_sha256':controls[0]['binary']['sha256']})
 rp=mapped(R/'source_review_RV29'/review/'')
 if review!='g3_final_25':
  # Path() removes trailing slash; map with explicit slash.
  rp=mapped(str(R/'source_review_RV29'/review)+'/');seal=H(blob(rp+'SHA256SUMS'));assert seal==review_seals[rp]
 else:rp=str(R/'source_review_RV29/g3_final_25')+'/';seal='SELF_MANIFEST'
 qualified.append({'id':id,'registration':g['group']+'::'+g['id'],'review_packet':rp,'review_seal':seal,'predicate_count':len(predicates),'predicates':predicates,'status':'QUALIFIED_G3_REGISTRATION','special_binding':'original tail vector isolated' if i in [9,10] else 'named N05/p128 isolation' if i==33 else 'actual historical classification-set19 mapping restored' if i==46 else 'both corrected M37 filters required' if i==52 else 'original frozen filter'})
assert len(qualified)==52 and sum(r['predicate_count'] for r in qualified)==53
excluded=[]
for id,pp in [('R09','I22/protected_g3/runtime_01'),('R10','I22/protected_g3/runtime_02'),('R33','I22/protected_g3/runtime_04'),('R46','I22/protected_g3/runtime_05')]:
 r=next(r for r in allruns if r['id']==id and r['packet']==pp and r['kind']=='mutant');excluded.append({'id':id,'packet':str(R/pp),'evidence':r['evidence'],'exit':r['tool_completion']['exit_code'],'credit':False,'reason':'focused-filter SURVIVOR / recovered mapping error' if id=='R46' else 'earlier wrong named witness'})
assert len(allruns)==238;testcalls=[r for r in allruns if r['kind'] not in ['build','list']];assert len(testcalls)==118;fails=sum(r['tool_completion']['exit_code']==101 for r in testcalls);passes=sum(r['tool_completion']['exit_code']==0 for r in testcalls);assert (fails,passes)==(56,62)
# 57 fault calls =53qualified failures+3wrong-witness failures+1survivor. Remaining61 are unmutated passes.
faultcalls=[r for r in testcalls if r['kind'] in ['mutant','historical_mutant']];assert len(faultcalls)==57;assert sum(r['tool_completion']['exit_code']==101 for r in faultcalls)==56
report={'candidate':rev,'all52registrations':qualified,'original_attempts_excluded':excluded,'packet_inventory':packetrows,'totals':{'registrations':52,'qualified_assertion_failures':53,'all_commands':238,'builds':counts['build'],'lists':counts['list'],'test_invocations':118,'fault_test_calls':57,'fault_failure_calls':56,'fault_survivors_uncredited':1,'wrong_witness_failures_uncredited':3,'unmutated_pass_calls':61,'all_pass_calls_including_survivor':62,'distinct_used_binary_paths':len(allbins)},'relocation_file_modes_and_blob_ids_unchanged':102,'replay_uses_prefix_ledger':True,'baseline_scope':'Prior full-FK456/0/1 unchanged560core bytes and33selected initial filters reused; initial full suite is not counted among238G3commands.','boundaries':'G3 selected52 registration closure only; historical derived/equivalent/retired/optional dispositions elsewhere unchanged; V-K and finalA1 remain outside scope.'};(Q/'G3_RECONCILIATION.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report['totals'],indent=2));print('ALL52CLOSED',len(qualified))
