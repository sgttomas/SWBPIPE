from pathlib import Path
import json,hashlib,subprocess,collections,re
O=Path(__file__).parent;D=O.parent;R=next(p for p in D.parents if (p/'agents/AGENT_TASK.md').is_file());C=D.parents[1].relative_to(R);E=C/'ENGINE_INTEGRATION';V=C/'VALIDATION_FOUNDATION';N=C/'_run_records/JOINED_ENGINE_NATIVE';A=V/'ORDINARY_PHYSICS_ADAPTER'
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def git(*args):return subprocess.check_output(['git',*args],cwd=R)
def blob(rev,path):return git('show',rev+':'+str(path))
b=read(D/'BASIS.json');cust=read(D/'PRIMARY_BASIS_CUSTODY.json');head=git('rev-parse','HEAD').decode().strip();assert head=='c278f64ba122eb8b848b9f14e58a0e533e94439a';assert git('show','-s','--format=%P',head).decode().strip().split()==[b['engine_parent'],b['validation_parent']]
paths=git('diff','--name-only',b['engine_parent'],head).decode().splitlines();incoming=b['incoming_paths'];maint=b['incoming_maintained_paths'];assert len(incoming)==414 and len(maint)==32
for path in incoming:assert blob(head,path)==blob(b['validation_parent'],path)
assert set(maint)=={p for p in paths if '/execution/' not in p};assert all(p.startswith(('projects/chirality-piping/tests/','projects/chirality-piping/tools/validation/','projects/chirality-piping/validation/qualification/')) for p in maint)
assert len(cust['files'])==10
for r in cust['files']:assert hashlib.sha256(blob(head,r['path'])).hexdigest()==r['sha256'] and blob(head,r['path'])==blob(cust['primary_commit'],r['path'])
f=read(R/A/'SOURCE_FREEZE_03.json');assert len(f['files'])==26
for row in f['files']:assert sha(R/row['path'])==row['sha256']==hashlib.sha256(blob(head,row['path'])).hexdigest()
# Cover the6 remaining maintained validation paths through their existing reviews.
remaining=set(maint)-{r['path'] for r in f['files']};mapping=read(R/V/'SOURCE_FREEZE_02.json');delta=read(R/V/'GATE_REVIEW/MAPPING_DELTA_BACKCHECK.json');mapped={x['path']:x['sha256'] for x in mapping['files']};mapped.update({x['path']:x['current_sha256'] for x in delta['results']})
for p,h in mapped.items():assert sha(R/p)==h
extra_dependencies={'projects/chirality-piping/tools/validation/qualification_process.py':'bf1ed365763440261f21a0f699ecf8729c745d259880d3851649d0292c1282c6','projects/chirality-piping/tests/test_qualification_gate.py':'5447262a437532b043b210b0fc397739ec5ee3330238644c342018cc2f89b64a'}
assert remaining==set(mapped)|set(extra_dependencies)
for p,h in extra_dependencies.items():assert sha(R/p)==h
# Native build input identity is unchanged despite added Python harness/tests/evidence.
native_inputs=read(R/N/'_run_records/inputs-pre.json');assert len(native_inputs)==1092 and native_inputs==read(R/N/'_run_records/inputs-post.json')
for row in native_inputs:assert sha(R/row['path'])==row['sha256']
artifact=read(R/N/'_run_records/artifact-identity.json');assert sha(Path(artifact['executable']))==artifact['executable_sha256']=='f68dc4de906eb5e8c96bb72c8736b7a5a9afec09ba533ada7ac676223fd36dcc'
nc=read(R/N/'CUSTODY.json');assert len(nc['public_files'])==60
for row in nc['public_files']:assert sha(R/N/row['path'])==row['sha256'] and (R/N/row['path']).stat().st_size==row['bytes']
review=read(R/N/'INDEPENDENT_REVIEW/VERIFICATION.json');assert review['primary_comparisons']==348 and review['repeated_comparisons']==129 and review['maintained_build_inputs_verified']==1092
# Validate new-import scope of every extra record without importing private payloads.
extras=set(paths)-set(incoming);policy={x['path'] for x in cust['files']};native={p for p in extras if p.startswith(str(N)+'/')};eng={p for p in extras if p.startswith(str(E)+'/')};assert extras==policy|native|eng and len(native)==67 and len(eng)==27
assert all(not p.lower().endswith(('.med','.pdf','.zip','.dmg','.exe','.app','.wasm')) for p in paths)
# Selected run evidence remains historical8b982 source/build, not relabelled as a newc278 run.
programme=read(R/A/'COMPARISON_RUN_01/PROGRAMME.json');assert programme['actual_case_mode_executions']==4 and programme['all_selected_obligations_matched'];assert programme['programme_denominator']=={'case_mode_executions':4,'scalar_obligations':292,'structural_obligations':40,'section_subchecks_nested':36}
assert sha(R/A/'SOURCE_FREEZE_02.json')==programme['adapter_freeze_sha256'];run_artifacts=0
for mode in programme['modes']:
 lp=R/A/mode['ledger_path'];assert sha(lp)==mode['ledger_sha256'];ledger=read(lp);assert ledger['execution_basis']['source_commit']==b['engine_parent'];assert ledger['execution_basis']['harness_sha256']==sha(R/'projects/chirality-piping/tools/validation/qualification_gate.py')
 for case in ledger['cases']:
  for item in case['retained_artifacts']:
   p=lp.parent/item['path'];assert sha(p)==item['sha256'];run_artifacts+=1
# Runtime helper dependency/review origins remain available in the union.
binding=read(R/A/'REVIEWED_READER_BINDING.json')
for item in binding['files']+binding['review_basis']:assert sha(R/'projects/chirality-piping'/item['path'])==item['sha256']
for name in ['qualification_fixture_support.py','test_first_static_selection.py','test_qualification_physics_integration.py','test_qualification_physics_structure.py']:
 text=(R/'projects/chirality-piping/tests'/name).read_text();assert 'execution/_Coordination/AgentRuns' not in text
helper=(R/'projects/chirality-piping/tests/qualification_fixture_support.py').read_text();assert "row['path']" in helper and "row['origin_path']" not in helper
addendum=read(R/E/'FINAL_REVIEW_COVERAGE_ADDENDUM.json')
for item in addendum['new_completed_reviews']:assert sha(R/item['path'])==item['sha256']
result={'status':'CLEAR bounded final integration review','candidate':head,'parents':[b['engine_parent'],b['validation_parent']],'all_incoming414_equal_validation_parent':True,'actual_delta_paths':len(paths),'maintained32':[{'path':p,'sha256':sha(R/p)} for p in maint],'validation26_freeze03_match':True,'other6_match_named_mapping_gate_reviews':True,'ten_primary_policy_records_equal':cust,'native_build_inputs_unchanged':1092,'native_executable_sha256':artifact['executable_sha256'],'native_public_custody_files':60,'native_observation_review_reused':{'primary':348,'repeated':129,'total':477,'independent_UI_witness_by_this_reviewer':False},'record_scope_groups':{'validation_records':382,'validation_maintained':32,'native_records':67,'engine_review_records':27,'primary_policy_records':10},'all_actual_delta_accounted':True,'no_external_binary_or_engineering_source_assets_added':True,'real_development_runs':programme['programme_denominator'],'historical8b_identity_preserved':True,'ledger_artifacts_rechecked':run_artifacts,'reader_dependency_review_hashes_match':True,'no_permanent_dated_test_dependency':True,'limits':'Source integration/custody review only, not actual-head CI/sweep/merge acceptance. No rerun, native/build/network or private model/store reads. Original receipt authorship separate; unchanged producer reviews reused.'}
(O/'CHECKS.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({k:v for k,v in result.items() if k not in ['maintained32','ten_primary_policy_records_equal']},indent=2))
