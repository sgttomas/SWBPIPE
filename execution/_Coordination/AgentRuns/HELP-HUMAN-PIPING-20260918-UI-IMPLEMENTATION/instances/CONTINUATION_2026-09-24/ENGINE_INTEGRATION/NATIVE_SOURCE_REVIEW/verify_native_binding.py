"""Read-only native diff, lock, fixture/API and evidence binding checks."""
from pathlib import Path
import hashlib,json,re,subprocess,sys,tomllib

ROOT=Path('/private/tmp/piping-engine-integration-20260925')
HERE=Path(__file__).resolve().parent
manifest=json.loads((HERE/'MANIFEST.json').read_text())
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
patch=(HERE/'CANDIDATE.patch').read_text()
assert sha(HERE/'CANDIDATE.patch')==manifest['patch_sha256']
expected={p['path']:p['sha256'] for p in manifest['paths']}
base_bytes={}
paths=[]
for part in re.split(r'(?=^diff --git )',patch,flags=re.M)[1:]:
    lines=part.splitlines(keepends=True);path=re.match(r'diff --git a/(.*?) b/(.*?)\n',lines[0])[2];paths.append(path)
    old_bytes=subprocess.run(['git','-C',str(ROOT),'show',manifest['base']+':'+path],capture_output=True,check=True).stdout
    base_bytes[path]=old_bytes;old=old_bytes.decode().splitlines(keepends=True);out=[];cursor=0;i=0
    while i<len(lines):
        h=re.match(r'@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@',lines[i])
        if not h:i+=1;continue
        start=max(0,int(h[1])-1);out.extend(old[cursor:start]);cursor=start;i+=1
        while i<len(lines) and not lines[i].startswith('@@ '):
            line=lines[i]
            if line.startswith((' ','-')):assert old[cursor]==line[1:];cursor+=1
            if line.startswith((' ','+')):out.append(line[1:])
            i+=1
    out.extend(old[cursor:]);reconstructed=''.join(out).encode()
    assert reconstructed==(ROOT/path).read_bytes()
    assert hashlib.sha256(reconstructed).hexdigest()==expected[path]
assert set(paths)==set(expected)
scope=[sys.executable,str(ROOT/'tools/software_workflow/validate_change_scope.py'),str(ROOT)]
for path in paths:scope+=['--allowed',path,'--path',path]
(HERE/'SCOPE_CHECK.json').write_text(subprocess.run(scope,capture_output=True,text=True,check=True).stdout)

lock='projects/chirality-piping/apps/desktop/src-tauri/Cargo.lock'
old=tomllib.loads(base_bytes[lock].decode());new=tomllib.loads((ROOT/lock).read_text())
assert {k:v for k,v in old.items() if k!='package'}=={k:v for k,v in new.items() if k!='package'}
def key(p):return p['name'],p['version'],p.get('source')
before={key(p):p for p in old['package']};after={key(p):p for p in new['package']}
assert before.keys()==after.keys()
changes=[]
for k,v in before.items():
    now=after[k]
    assert {x:y for x,y in v.items() if x!='dependencies'}=={x:y for x,y in now.items() if x!='dependencies'}
    if v.get('dependencies',[])!=now.get('dependencies',[]):
        added=sorted(set(now['dependencies'])-set(v['dependencies']));removed=sorted(set(v['dependencies'])-set(now['dependencies']))
        assert not removed
        changes.append({'package':k[0],'added':added,'removed':removed})
assert changes==[
    {'package':'open_pipe_stress_product_physics','added':['open_pipe_stress_canonical_json','sha2'],'removed':[]},
    {'package':'open_pipe_stress_result_export','added':['open_pipe_stress_units'],'removed':[]},
]

fixture=ROOT/'projects/chirality-piping/fixtures/product_preview/source_blocks/ui/n05-sparse_interactive.request.json'
raw=json.loads(fixture.read_text());assert set(raw)=={'model','materials'} and raw['materials']==[]
assert raw['model']['schema_version']=='0.1.0'
authored=json.loads(fixture.read_text());authored['model']['schema_version']='0.2.0'
assert authored=={'model':authored['model'],'materials':[]}
assert json.loads(fixture.read_text())==raw
lib=(ROOT/'projects/chirality-piping/apps/desktop/src-tauri/src/lib.rs').read_text()
start=lib.index('fn solve_preview_mechanics_with_mode(');end=lib.index('\nfn resolve_solve_model_payload',start)
body=lib[start:end]
assert 'run_linear_static_preview_value_with_mode(request, solver_mode)' in body
assert 'serde_json::from_value' not in body and 'run_linear_static_preview_with_mode(' not in body
assert 'let request = json!({"model":model_payload,"materials":[]});' in body
assert 'source_block_invocation: Option<Value>' in lib
assert 'numerical_use_standing_with_context(envelope, &refs, invocation)' in lib
assert 'context.pointer("/request/model") != Some(model)' in lib
assert 'Some(value) => (value, source_block_invocation)' in lib

context=[
    'projects/chirality-piping/core/product_physics/src/lib.rs',
    'projects/chirality-piping/core/product_physics/src/source_receipt.rs',
    'projects/chirality-piping/core/reporting/result_export/src/semantic_contract.rs',
    'projects/chirality-piping/core/reporting/result_export/src/source_blocks.rs',
    'projects/chirality-piping/core/reporting/result_export/src/physics_source.rs',
    'projects/chirality-piping/apps/desktop/src/services/previewService.ts',
    'projects/chirality-piping/apps/desktop/src/services/ruleCheckService.ts',
    'projects/chirality-piping/apps/desktop/src-tauri/src/model_document_migration.rs',
    'projects/chirality-piping/apps/desktop/src-tauri/Cargo.toml',
    'projects/chirality-piping/core/product_physics/Cargo.toml',
    'projects/chirality-piping/core/reporting/result_export/Cargo.toml',
    str(fixture.relative_to(ROOT)),
    'projects/chirality-piping/fixtures/model_operations/exact_pressure_authoring_model.json',
]
report={'actor':'/root/physics_resume/joined_producer_review','parent':'/root/physics_resume','role':'TASK Type2','method':'Static source/diff/fixture checks and TOML lock comparison; no build/runtime','base':manifest['base'],'patch_sha256':manifest['patch_sha256'],'final_reviewed_hashes':expected,'lock_packages_before':len(before),'lock_packages_after':len(after),'package_version_source_checksum_metadata_unchanged':True,'exact_dependency_changes':changes,'fixture_original_schema':'0.1.0','native_test_authored_schema_before_invocation':'0.2.0','complete_fixture_request_matches_native_wrapper':True,'observed_context_hashes':{p:sha(ROOT/p) for p in context}}
(HERE/'VERIFICATION.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({k:v for k,v in report.items() if k not in ['observed_context_hashes','final_reviewed_hashes']}))
