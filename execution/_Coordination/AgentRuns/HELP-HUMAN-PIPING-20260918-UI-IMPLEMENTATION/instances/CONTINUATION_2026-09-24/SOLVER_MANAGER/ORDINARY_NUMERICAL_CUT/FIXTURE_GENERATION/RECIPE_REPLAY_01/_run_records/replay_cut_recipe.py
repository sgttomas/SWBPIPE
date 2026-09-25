from pathlib import Path
import hashlib,json,os,subprocess,time
R=Path('/private/tmp/piping-numerical-corrections-20260924');P=R/'projects/chirality-piping';D=P/'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/SOLVER_MANAGER/ORDINARY_NUMERICAL_CUT/FIXTURE_GENERATION/RECIPE_REPLAY_01/_run_records';D.mkdir(parents=True,exist_ok=True)
F=P/'fixtures/product_preview'; names=['invented_mechanics_result_precision_1_sparse.json','invented_mechanics_result_precision_1_dense.json','precision_fixture_generation.json','invented_mechanics_result.json'];before={n:hashlib.sha256((F/n).read_bytes()).hexdigest() for n in names}
for n in names:(D/('before-'+n)).write_bytes((F/n).read_bytes())
env=os.environ.copy();env.update(CARGO_BUILD_JOBS='2',CARGO_NET_OFFLINE='true',CARGO_TARGET_DIR='/private/tmp/piping-numerical-corrections-product-target')
cmd=['npm','run','generate:product-preview-mechanics'];start=time.time()
with (D/'stdout.log').open('w') as out,(D/'stderr.log').open('w') as err: p=subprocess.run(cmd,cwd=P,env=env,stdout=out,stderr=err)
after={n:hashlib.sha256((F/n).read_bytes()).hexdigest() for n in names};row={'command':cmd,'cwd':str(P),'started_unix':start,'ended_unix':time.time(),'exit_code':p.returncode,'target':env['CARGO_TARGET_DIR'],'before':before,'after':after,'fixtures_unchanged':all(before[n]==after[n] for n in names if n!='precision_fixture_generation.json'),'stdout_sha256':hashlib.sha256((D/'stdout.log').read_bytes()).hexdigest(),'stderr_sha256':hashlib.sha256((D/'stderr.log').read_bytes()).hexdigest()}
(D/'RESULT.json').write_text(json.dumps(row,indent=2)+'\n');print(json.dumps(row),flush=True);print((D/'stderr.log').read_text()[-2000:],flush=True)
if p.returncode or not row['fixtures_unchanged']:raise SystemExit(p.returncode or 2)
record=json.loads((F/'precision_fixture_generation.json').read_text());assert record['source_input_files']==record['source_input_files_after']
for path,sha in record['source_input_files'].items():assert hashlib.sha256((P/path).read_bytes()).hexdigest()==sha,path
for output in record['outputs']:assert hashlib.sha256((P/output['path']).read_bytes()).hexdigest()==output['sha256']
(D/'VERIFIED.json').write_text(json.dumps({'source_input_count':len(record['source_input_files']),'dependencies_count':len(record['dependencies']),'outputs':record['outputs'],'source_and_output_hashes_verified':True},indent=2)+'\n');print('record/source/output verification PASS',flush=True)
