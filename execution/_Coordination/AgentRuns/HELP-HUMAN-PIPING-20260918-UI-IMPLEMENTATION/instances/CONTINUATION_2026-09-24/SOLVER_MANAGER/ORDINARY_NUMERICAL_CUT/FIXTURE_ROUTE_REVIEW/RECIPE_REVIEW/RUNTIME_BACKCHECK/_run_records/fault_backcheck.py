"""Inspect retained fault-control evidence/sandboxes; never executes the harness."""
from pathlib import Path
import datetime,hashlib,json
R=Path('/private/tmp/piping-numerical-corrections-20260924')
P=R/'projects/chirality-piping'
B=P/'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/SOLVER_MANAGER/ORDINARY_NUMERICAL_CUT'
H=B/'FIXTURE_GENERATION/RECIPE_REPAIR/REPAIR_01/_run_records/fault_controls'
X=H/'EXECUTION_01'
E=B/'FIXTURE_ROUTE_REVIEW/RECIPE_REVIEW/RUNTIME_BACKCHECK/_run_records'
h=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
j=lambda p:json.loads(p.read_text())
summary=j(X/'SUMMARY.json');invocation=j(X/'INVOCATION.json')
checks={'harness invocation completed successfully':invocation['exit_code']==0 and invocation['command']==['node','run.mjs','./EXECUTION_01'],
 'invocation binds prepared freeze':invocation['source_freeze_sha256']==h(H/'PREPARATION_FREEZE.json'),
 'executed source/input inventory matches current prepared bytes':all(h(H/name)==digest for name,digest in summary['inputs'].items()),
 'exact four requested controls': [c['scenario'] for c in summary['cases']]==['install2','install3','install3-rollback1','symlink-parent']}
seed_names={'invented_mechanics_result_precision_1_sparse.json':'sparse.preimage.json','invented_mechanics_result_precision_1_dense.json':'dense.preimage.json','precision_fixture_generation.json':'record.preimage.json','invented_mechanics_result.json':'legacy.preimage.json','invented_preview_model.json':'model.json'}
sandbox_origins=[]
for brief in summary['cases']:
 name=brief['scenario'];case=j(X/(name+'.json'));container=Path(case['sandbox']);root=container/'project';fd=root/'fixtures/product_preview'
 assert container.parent==Path('/private/tmp') and container.name.startswith('precision-transaction-fault-')
 observed=container/'outside-project-fixtures' if name=='symlink-parent' else fd
 actual={n:h(observed/n) for n in seed_names}
 events_file=root/'fault-events.ndjson'
 events=[json.loads(line) for line in events_file.read_text().splitlines()] if events_file.exists() else []
 stages=sorted(p.name for p in observed.iterdir() if p.name.startswith('.precision-generation-'))
 stderr=(X/(name+'.stderr.log')).read_text()
 checks[name+' expected failure and assertion pass']=case['status']==1 and case['assertions']==brief['assertions']=='passed'
 checks[name+' actual sandbox recipe/stub bytes']=h(root/'tools/serialization/generate_product_preview_mechanics.mjs')==case['recipe_sha256']==h(H/'recipe.mjs') and all(h(root/'bin'/n)==h(H/(n+'-stub.mjs')) for n in ['cargo','rustc'])
 checks[name+' independent before and actual after binding']=case['before']=={n:h(H/'inputs'/seed) for n,seed in seed_names.items()} and actual==case['after']
 checks[name+' raw event trace agrees with record']=events==case['events'] and stages==sorted(case['recovery_directories'])
 checks[name+' input and legacy retained']=all(actual[n]==case['before'][n] for n in ['invented_mechanics_result.json','invented_preview_model.json'])
 if name in ['install2','install3']:
  checks[name+' all preimages restored no stage']=actual==case['before'] and not stages and 'INJECTED_INSTALL_'+name[-1] in stderr
  checks[name+' correct install/restore counts']=len([x for x in events if x['operation']=='install'])==int(name[-1]) and len([x for x in events if x['operation']=='restore'])==int(name[-1])-1
 elif name=='install3-rollback1':
  recovery=observed/stages[0]
  checks[name+' exact retained backup bytes']=len(stages)==1 and h(recovery/'1.old')==case['before']['invented_mechanics_result_precision_1_dense.json'] and h(recovery/'2.old')==case['before']['precision_fixture_generation.json']
  checks[name+' continued sparse restore and intact record']=actual['invented_mechanics_result_precision_1_sparse.json']==case['before']['invented_mechanics_result_precision_1_sparse.json'] and actual['precision_fixture_generation.json']==case['before']['precision_fixture_generation.json'] and len([x for x in events if x['operation']=='restore'])==2
  checks[name+' unrecovered dense equals only stub raw']=actual['invented_mechanics_result_precision_1_dense.json']==h(H/'inputs/dense.stdout.json')
  checks[name+' recovery map and both errors disclosed']=len(j(recovery/'recovery.json')['destinations'])==3 and all(s in stderr for s in ['INJECTED_INSTALL_3','INJECTED_RESTORE_1','rollback incomplete',str(recovery)])
 else:
  checks[name+' linked external parent remained untouched']=fd.is_symlink() and fd.resolve()==observed and actual==case['before'] and not stages and not events
  checks[name+' refusal before Cargo']=not (root/'stub-commands.ndjson').exists() and 'redirected project path: fixtures/product_preview' in stderr
 if name!='symlink-parent':
  commands=[json.loads(line) for line in (root/'stub-commands.ndjson').read_text().splitlines()]
  checks[name+' stub commands include exact two mode captures']=[c['args'][-1] for c in commands if c['args'][0]=='run']==['sparse_interactive','dense_scrutiny']
 sandbox_origins.extend({'path':str(p),'sha256':h(p)} for p in sorted(container.rglob('*')) if p.is_file() and not p.is_symlink())
custody=j(X/'RECOVERY_CUSTODY.json')
checks['all nine recovery custody copies equal retained originals']=len(custody['files'])==9 and all(h(Path(x['original']))==h(X/x['custody'])==x['sha256'] for x in custody['files'])
report={'time_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'checks':checks,'all_passed':all(checks.values()),'sandbox_files_inspected':sandbox_origins,'boundary':'Observed parent-run four fault controls on the exact recipe, with stub Cargo/rustc and invented unresolved transaction inputs. Reviewer ran no Node, Cargo, harness or tests. No producer/physics/Current result claim.'}
(E/'FAULT_BACKCHECK.json').write_text(json.dumps(report,indent=2)+'\n')
prior=j(B/'FIXTURE_ROUTE_REVIEW/_run_records/ORIGINS.json')
inputs=[p for p in sorted(H.rglob('*')) if p.is_file()]+[p for p in sorted((B/'FIXTURE_GENERATION/RECIPE_REPLAY_01').rglob('*')) if p.is_file()]+[P/'fixtures/product_preview/PRECISION_FIXTURES.md',P/'fixtures/product_preview/precision_fixture_generation.json',P/'tools/serialization/generate_product_preview_mechanics.mjs',P/'package.json']
origins={'agent':'/root/solver_manager/fixture_route_review','parent':'/root/solver_manager','role':'TASK Type 2, same independent reviewer','mechanism':'delegated-harness-native followup, no descendants','instruction_origins':prior['instructions_fully_read'],'instruction_hashes_unchanged':all(h(R/x['path'])==x['sha256'] for x in prior['instructions_fully_read']),'scope':'Fault-harness source review, parent-run real maintained replay, doc delta and parent-run transaction controls; App checks excluded','review_inputs':[{'path':str(p.relative_to(R)),'sha256':h(p)} for p in inputs],'sandbox_evidence':'FAULT_BACKCHECK.json contains actual retained sandbox file hashes','limits':report['boundary']}
(E/'ORIGINS.json').write_text(json.dumps(origins,indent=2)+'\n')
print(json.dumps({'all_passed':report['all_passed'],'checks':len(checks),'failures':[n for n,v in checks.items() if not v],'sandbox_file_count':len(sandbox_origins)},indent=2))
