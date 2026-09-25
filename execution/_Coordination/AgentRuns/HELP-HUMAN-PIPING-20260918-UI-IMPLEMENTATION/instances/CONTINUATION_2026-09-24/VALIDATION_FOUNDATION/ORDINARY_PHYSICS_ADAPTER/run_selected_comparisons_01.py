"""Execute the four already-selected development comparisons; never edit targets."""
from pathlib import Path
import hashlib,json,subprocess,sys,time
E=Path(__file__).resolve().parent
P=next(p for p in E.parents if p.name=='chirality-piping');R=P.parents[1]
sys.path.insert(0,str(P))
from tools.validation import qualification_gate as gate
ENGINE=Path('/private/tmp/piping-engine-integration-20260925')
EXPECTED_REVIEW='f4143102f33b9131726c8949062c00eabeb05ccb3e477d95d3d0072f497dd5a7'
def sha(data):return hashlib.sha256(data).hexdigest()
def write(path,value):path.write_text(json.dumps(value,indent=2,allow_nan=False)+'\n')
review=E/'INDEPENDENT_REVIEW/REPAIR_BACKCHECK_02.md'
assert sha(review.read_bytes())==EXPECTED_REVIEW
build=json.loads((E/'HEADLESS_BUILD/BUILD_RESULT.json').read_text())
freeze=json.loads((E/'SOURCE_FREEZE_02.json').read_text())
source_inputs=json.loads((E/'HEADLESS_BUILD/SOURCE_BEFORE.json').read_text())
runner=Path(build['executable']['path'])
output=E/'COMPARISON_RUN_01';output.mkdir(exist_ok=False)

def observe():
 failures=[]
 for row in source_inputs:
  if sha(Path(row['path']).read_bytes())!=row['sha256']: failures.append(row['path'])
 for row in freeze['files']:
  if sha((R/row['path']).read_bytes())!=row['sha256']: failures.append(row['path'])
 commit=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ENGINE,text=True).strip()
 digest=sha(runner.read_bytes())
 status=subprocess.check_output(['git','status','--porcelain=v1'],cwd=ENGINE,text=True)
 return {'candidate_commit':commit,'executable_sha256':digest,'bound_build_inputs':len(source_inputs),
         'adapter_files':len(freeze['files']),'changed_bound_paths':failures,'source_status':status,
         'matches_selected_identity':not failures and commit==build['candidate_commit'] and digest==build['executable']['sha256']}

before=observe();write(output/'IDENTITY_BEFORE.json',before)
assert before['matches_selected_identity']
record={'artifact':'openpipestress.first_static_development_observation','version':'1.0.0',
 'programme_denominator':{'case_mode_executions':4,'scalar_obligations':292,'structural_obligations':40,'section_subchecks_nested':36},
 'build_result_sha256':sha((E/'HEADLESS_BUILD/BUILD_RESULT.json').read_bytes()),
 'adapter_freeze_sha256':sha((E/'SOURCE_FREEZE_02.json').read_bytes()),
 'independent_review_sha256':EXPECTED_REVIEW,'orchestrator_sha256':sha(Path(__file__).read_bytes()),
 'modes':[{'mode':mode,'state':'not_run'} for mode in ('sparse_interactive','dense_scrutiny')],
 'qualification':'bounded development comparison only; no full Q1, Current, native, professional acceptance or release',
 'missing_outputs':'FIRST_STATIC_BINDINGS/OUTPUT_OBLIGATIONS.json remains applicable; direct transverse-shear/signed circumferential coverage is not invented'}
write(output/'PROGRAMME.json',record)
for item in record['modes']:
 mode=item['mode'];selection=E/'EXECUTION_SELECTIONS'/mode/'selection.json'
 item['selection_sha256']=sha(selection.read_bytes());item['state']='running';write(output/'PROGRAMME.json',record)
 started=time.monotonic()
 try:
  ledger=gate.run_selection(selection,runner,ENGINE,output/mode)
  item.update(state=ledger['outcome'],summary=ledger['summary'],cases=[{'id':case['id'],'state':case['state'],'reason':case['reason']} for case in ledger['cases']],ledger_path=str((output/mode/'ledger.json').relative_to(E)),ledger_sha256=sha((output/mode/'ledger.json').read_bytes()))
 except Exception as exc:
  item.update(state='error',reason=type(exc).__name__+': '+str(exc))
 item['elapsed_seconds']=time.monotonic()-started;write(output/'PROGRAMME.json',record)
after=observe();write(output/'IDENTITY_AFTER.json',after)
record['identity_after_matches']=after['matches_selected_identity']
record['all_selected_obligations_matched']=after['matches_selected_identity'] and all(item['state']=='all_required_assertions_matched' for item in record['modes'])
record['actual_case_mode_executions']=sum(len(item.get('cases',[])) for item in record['modes'])
record['identity_before_sha256']=sha((output/'IDENTITY_BEFORE.json').read_bytes());record['identity_after_sha256']=sha((output/'IDENTITY_AFTER.json').read_bytes())
write(output/'PROGRAMME.json',record)
print(json.dumps(record,indent=2),flush=True)
