from pathlib import Path
import hashlib,json,os,subprocess,sys,time,runpy
R=Path('/private/tmp/piping-numerical-corrections-20260924');P=R/'projects/chirality-piping';E=P/'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/SOLVER_MANAGER/ORDINARY_NUMERICAL_CUT/FIXTURE_GENERATION/_run_records';E.mkdir(parents=True,exist_ok=True)
a=sys.argv;sys.argv=['snapshot'];snap=runpy.run_path('/private/tmp/solver_ready_checks.py')['snapshot'];sys.argv=a;snap.__globals__['R']=R
m=P/'core/product_physics/Cargo.toml';before=snap(m);model=P/'fixtures/product_preview/invented_preview_model.json';legacy=P/'fixtures/product_preview/invented_mechanics_result.json';h=lambda b:hashlib.sha256(b).hexdigest();legacyhash=h(legacy.read_bytes());sourceid=h(json.dumps(before,sort_keys=True,separators=(',',':')).encode());records=[]
(E/'BUILD_INPUTS.json').write_text(json.dumps({'snapshot':before,'input_model_sha256':h(model.read_bytes()),'legacy_fixture_sha256':legacyhash,'source_snapshot_json_sha256':sourceid,'started_unix':time.time()},indent=2)+'\n')
for mode,suffix in [('sparse_interactive','sparse'),('dense_scrutiny','dense')]:
 cmd=['cargo','run','--offline','--locked','-j2','--manifest-path',str(m.relative_to(R)),'--example','preview_result','--',mode];env=os.environ.copy();env['CARGO_BUILD_JOBS']='2';env['CARGO_TARGET_DIR']='/private/tmp/piping-numerical-corrections-product-target'
 with (E/(suffix+'.stdout.json')).open('wb') as out,(E/(suffix+'.stderr.log')).open('wb') as err:result=subprocess.run(cmd,cwd=R,env=env,stdout=out,stderr=err)
 record={'requested_mode':mode,'command':cmd,'cwd':str(R),'target':env['CARGO_TARGET_DIR'],'exit_code':result.returncode,'stdout_sha256':h((E/(suffix+'.stdout.json')).read_bytes()),'ended_unix':time.time()}
 if result.returncode:
  records.append(record);(E/'GENERATION.json').write_text(json.dumps(records,indent=2)+'\n');print(json.dumps(record),flush=True);print((E/(suffix+'.stderr.log')).read_text()[-4000:]);sys.exit(result.returncode)
 data=json.loads((E/(suffix+'.stdout.json')).read_text());modeldata=json.loads(model.read_text());expected=1.0 if mode=='sparse_interactive' else 2.0
 rows=[x for x in data['results'] if x['kind']=='linear_solver_mode_basis'];expectedids=[x['id'] for x in modeldata['load_cases']]
 assert data['schema_version']=='0.2.0' and data['producer']['component_name']=='open_pipe_stress_product_physics' and data['producer']['semantic_contract_id']=='openpipestress.result_semantics/0.3.0/precision-1'
 assert data['model_ref']==modeldata['project']['id']
 if data['status']['mechanics']=='MECHANICS_SOLVED':
  assert len(rows)==len(expectedids),(mode,rows)
  for cid in expectedids:
   matched=[x for x in rows if x.get('basis_ref')=={'ref_type':'load_case','ref_id':cid}];assert len(matched)==1 and matched[0]['value']==expected,(cid,matched)
   assert f'solver_mode={mode}' in matched[0]['metadata']['basis']
 assert snap(m)==before and h(legacy.read_bytes())==legacyhash
 target=P/f'fixtures/product_preview/invented_mechanics_result_precision_1_{suffix}.json';target.write_bytes((E/(suffix+'.stdout.json')).read_bytes())
 record.update({'output_path':str(target.relative_to(R)),'mechanics_status':data['status']['mechanics'],'numerical_status':data['numerical_quality']['status'],'result_count':len(data['results']),'mode_rows':rows,'all_source_inputs_unchanged':True});records.append(record)
 (E/'GENERATION.json').write_text(json.dumps(records,indent=2)+'\n');print(json.dumps({'mode':mode,'exit':0,'mechanics':record['mechanics_status'],'numerical':record['numerical_status'],'rows':len(data['results']),'mode_rows':rows,'sha256':record['stdout_sha256']}),flush=True)
(E/'AFTER.json').write_text(json.dumps({'sources_and_locks_unchanged':snap(m)==before,'legacy_fixture_unchanged':h(legacy.read_bytes())==legacyhash,'ended_unix':time.time()},indent=2)+'\n')
