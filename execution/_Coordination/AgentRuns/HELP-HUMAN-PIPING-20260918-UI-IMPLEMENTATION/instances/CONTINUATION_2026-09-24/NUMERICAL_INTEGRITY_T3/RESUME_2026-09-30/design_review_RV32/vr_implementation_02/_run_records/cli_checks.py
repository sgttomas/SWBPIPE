from pathlib import Path
import subprocess,json,hashlib,shutil
out=Path(__file__).resolve().parent
vr=out/'clean/projects/chirality-piping/validation/benchmarks/numerical_robustness'
binary=out/'target/debug/examples/vk_scale'
r=out.parents[2]
model_source=r/'I24/vr_integration_01/_run_records/changed_model_path_for_context_capture_and_duplicate_arguments/RF-LARGE-CHAIN-n01000-AX.json'
model_dir=out/'external_path_with_explicit_rebound_context'
model_dir.mkdir(exist_ok=True)
model=model_dir/model_source.name
shutil.copyfile(model_source,model)
commands=[]
def run(name,args,expected):
 argv=[str(binary)]+args
 with (out/(name+'.stdout.jsonl')).open('wb') as stdout,(out/(name+'.stderr.txt')).open('wb') as stderr:
  proc=subprocess.run(argv,cwd=vr,stdout=stdout,stderr=stderr,timeout=90)
 rows=[json.loads(x) for x in (out/(name+'.stdout.jsonl')).read_text().splitlines() if x.strip()]
 commands.append({'name':name,'argv':argv,'cwd':str(vr),'returncode':proc.returncode,'expected':expected,'match':proc.returncode==expected})
 assert proc.returncode==expected,(name,proc.returncode)
 return rows
id='RF-LARGE-CHAIN-n00010-AX'
counts=run('ind_counts',['--case',id,'--heap-cap-bytes','536870912','--counts-only'],0)
normal=run('ind_normal',['--case',id,'--heap-cap-bytes','536870912'],0)
refused=run('ind_half_cap',['--case',id,'--heap-cap-bytes','16777216'],3)
external=run('ind_external1000',['--case','overwritten','--case','RF-LARGE-CHAIN-n01000-AX','--model-file','unused-overwritten-path','--model-file',str(model),'--heap-cap-bytes','0000536870912','--counts-only'],0)
cf=next(x for x in counts if x['kind']=='counts');nf=next(x for x in normal if x['kind']=='counts');ef=next(x for x in external if x['kind']=='counts')
assert cf['estimate_max_bytes']>=nf['estimate_max_bytes']
assert cf['estimate_sel128_bytes']>=nf['estimate_sel128_bytes']
assert any(x.get('reason')=='estimate_exceeds_half_cap' for x in refused)
assert not any(x.get('kind')=='w1' for x in refused)
assert any(x.get('kind')=='w1' and x.get('counts_match_storage') for x in normal)
assert not any(x.get('kind')=='w1' for x in external)
# Independent closed caller/join reference: the retained path contributes once to the dominant post-cut external1000 alternatives.
accepted=json.loads((r/'metric_design_15_result5_join/RESULT5_CLI24.json').read_text())['rows']
reference=next(x for x in accepted if x['id']==ef['case'])
launch=next(x for x in json.loads((r/'metric_design_14_reference_callers/CLI24_LAUNCHES.json').read_text())['rows'] if x['id']==ef['case'])
path_delta=len(str(model).encode())-launch['external_model_path_utf8_bytes']
checks={}
for f in ['max','sel128','fixed']:
 checks[f]={'runtime':ef['estimate_'+f+'_bytes'],'accepted_plus_retained_path_delta':reference['metrics']['moving']['fields'][f]+path_delta}
 assert checks[f]['runtime']==checks[f]['accepted_plus_retained_path_delta'],checks[f]
for f in ['model','decide']:
 assert ef['estimate_'+f+'_bytes']==reference['metrics']['moving']['fields'][f]
(out/'CLI_CHECKS.json').write_text(json.dumps({'binary_sha256':hashlib.sha256(binary.read_bytes()).hexdigest(),'commands':commands,'counts_continuation_dominates_actual_normal':True,'external_raw_sha256':hashlib.sha256(model.read_bytes()).hexdigest(),'external_case_raw_hash':next(x for x in external if x['kind']=='start')['committed_model_sha256'],'external_retained_path_delta':path_delta,'external_reference_checks':checks,'claim':'functional/conditional arithmetic checks only; no timing/RSS or current-executable global-bound qualification'},indent=2)+'\n')
print('Four bounded CLI checks passed; external1000 exact retained-path delta verified.')
