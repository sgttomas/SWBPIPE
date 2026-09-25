"""Static full-diff reconstruction, origins, lock and negative-oracle checks."""
from pathlib import Path
from fractions import Fraction as F
import ast,hashlib,json,re,subprocess,sys,tomllib
ROOT=Path('/private/tmp/piping-engine-integration-20260925')
HERE=Path(__file__).resolve().parent
REVIEW=HERE.parent
JOIN=REVIEW.parent
P=ROOT/'projects/chirality-piping'
sha=lambda b:hashlib.sha256(b).hexdigest()
def blob(rev,path):return subprocess.run(['git','-C',str(ROOT),'show',rev+':'+path],capture_output=True,check=True).stdout
manifest=json.loads((REVIEW/'MANIFEST.json').read_text())
patch=(REVIEW/'CANDIDATE.patch').read_text()
assert sha(patch.encode())==manifest['patch_sha256']
audit='projects/chirality-piping/validation/benchmarks/physics_audit_regression/Cargo.lock'
repair=json.loads((REVIEW/'AUDIT_LOCK_REPAIR.json').read_text())
originals={};frozen={};paths=[];checks=0
for part in re.split(r'(?=^diff --git )',patch,flags=re.M)[1:]:
    lines=part.splitlines(keepends=True);path=re.match(r'diff --git a/(.*?) b/(.*?)\n',lines[0])[2];paths.append(path)
    b=b'' if 'new file mode 100644\n' in lines else blob(manifest['base'],path)
    originals[path]=b;old=b.decode().splitlines(keepends=True);out=[];cursor=0;i=0
    while i<len(lines):
        h=re.match(r'@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@',lines[i])
        if not h:i+=1;continue
        start=max(0,int(h[1])-1);out.extend(old[cursor:start]);cursor=start;i+=1
        while i<len(lines) and not lines[i].startswith('@@ '):
            line=lines[i];no_newline=i+1<len(lines) and lines[i+1].startswith('\\ No newline')
            value=line[1:].removesuffix('\n') if no_newline else line[1:]
            if line.startswith((' ','-')):assert old[cursor]==value;cursor+=1
            if line.startswith((' ','+')):out.append(value)
            i+=1
    out.extend(old[cursor:]);frozen[path]=''.join(out).encode()
    expected=next(x['sha256'] for x in manifest['paths'] if x['path']==path)
    assert sha(frozen[path])==expected;checks+=1
    if path!=audit:assert (ROOT/path).read_bytes()==frozen[path]
    else:assert sha((ROOT/path).read_bytes())==repair['after_sha256']
assert set(paths)=={x['path'] for x in manifest['paths']} and len(paths)==61
scope=[sys.executable,str(ROOT/'tools/software_workflow/validate_change_scope.py'),str(ROOT)]
for path in paths:scope+=['--allowed',path,'--path',path]
(HERE/'SCOPE_CHECK.json').write_text(subprocess.run(scope,capture_output=True,text=True,check=True).stdout)
dep='ed688b13f633ce8aa6aeede784c3562511c20966'
subset_path='projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/SOLVER_MANAGER/ORDINARY_NUMERICAL_CUT/SOURCE_RECOVERY_IMPLEMENTATION_01/REVIEWED_SUBSET.json'
subset_raw=blob(dep,subset_path);subset=json.loads(subset_raw)
origin=[]
for entry in subset['supporting_files']:
    path=entry['path']
    if path in frozen:
        data=blob(dep,path)
        assert data==frozen[path] and sha(data)==entry['sha256']
        origin.append({'path':path,'sha256':entry['sha256'],'scope_at_origin':'tested_support_not_independently_reviewed18'})
assert len(origin)==37

native=tomllib.loads((P/'apps/desktop/src-tauri/Cargo.lock').read_text())
def key(p):return p['name'],p['version'],p.get('source')
registry={key(p):p for p in native['package'] if 'source' in p}
dependency_lock=tomllib.loads(blob(dep,'projects/chirality-piping/core/product_physics/Cargo.lock').decode())
registry.update({key(p):p for p in dependency_lock['package'] if 'source' in p})
lock_results=[]
for path in paths:
    if not path.endswith('Cargo.lock'):continue
    before=tomllib.loads(originals[path].decode());after=tomllib.loads((ROOT/path).read_text())
    bm={key(x):x for x in before['package']};am={key(x):x for x in after['package']}
    assert bm.keys()<=am.keys()
    changed=[]
    for k,v in bm.items():
        now=am[k]
        assert {a:b for a,b in v.items() if a!='dependencies'}=={a:b for a,b in now.items() if a!='dependencies'}
        if v.get('dependencies',[])!=now.get('dependencies',[]):
            removed=set(v.get('dependencies',[]))-set(now.get('dependencies',[]));assert not removed
            changed.append({'package':k[0],'added_dependencies':sorted(set(now.get('dependencies',[]))-set(v.get('dependencies',[])))})
    for k in am.keys()-bm.keys():
        v=am[k]
        if 'source' in v:assert registry[k]==v
        else:assert v['name'].startswith('open_pipe_stress_')
    pp=next(v for v in am.values() if v['name']=='open_pipe_stress_product_physics')
    required=set(tomllib.loads((P/'core/product_physics/Cargo.toml').read_text())['dependencies'])
    assert set(pp['dependencies'])==required
    lock_results.append({'path':path,'before_packages':len(bm),'after_packages':len(am),'added_packages':[dict(name=k[0],version=k[1],source=k[2]) for k in sorted(am.keys()-bm.keys())],'existing_dependency_additions':changed,'existing_package_metadata_unchanged':True,'product_dependency_graph_complete':True})
# The supplementary repair must alter only the two missing edges.
assert sha(frozen[audit])==repair['before_sha256']
assert (REVIEW/'audit-lock-before.txt').read_bytes()==frozen[audit]
before=tomllib.loads(frozen[audit].decode());after=tomllib.loads((ROOT/audit).read_text())
bm={key(x):x for x in before['package']};am={key(x):x for x in after['package']};assert bm.keys()==am.keys()
different=[k for k in bm if bm[k]!=am[k]];assert len(different)==1 and different[0][0]=='open_pipe_stress_product_physics'
assert set(am[different[0]]['dependencies'])-set(bm[different[0]]['dependencies'])=={'open_pipe_stress_canonical_json','sha2'}
assert not(set(bm[different[0]]['dependencies'])-set(am[different[0]]['dependencies']))

# Read the retained wrong-output oracle exactly; preserve it as negative evidence.
oracle=json.loads((P/'fixtures/product_preview/source_blocks/rejected_stress_range/ORACLE.json').read_text())
for record in oracle['records']:
    expected=F(record['source_force_represented'])/F(record['area_represented'])/1000000
    for sample in record['samples']:
        relative=abs(F(sample['value'])/expected-1)
        assert relative>F(1e-9)
        assert abs(float(relative)-sample['relative_error_against_represented_F_A_MPa'])<1e-20
# Supporting test inputs use maintained fixtures, not dated evidence roots.
for path in ['projects/chirality-piping/core/product_physics/src/source_budget_tests.rs','projects/chirality-piping/core/product_physics/tests/source_block_recovery.rs']:
    text=(ROOT/path).read_text();assert 'AgentRuns/' not in text and '/private/tmp/' not in text
    for name in re.findall(r'include_str!\("([^"]+)"\)',text):assert ((ROOT/path).parent/name).is_file()
refs='projects/chirality-piping/validation/benchmarks/numerical_integrity/fixtures.json'
assert (ROOT/refs).read_bytes()==blob(manifest['base'],refs)
ast.parse((P/'core/analysis_runs/__init__.py').read_text())
final=[{'path':path,'sha256':sha((ROOT/path).read_bytes()),'matches_original_freeze':sha((ROOT/path).read_bytes())==sha(frozen[path])} for path in paths]
report={'actor':'/root/physics_resume/joined_producer_review','parent':'/root/physics_resume','role':'TASK Type2','execution':'Read-only Git blob inspection, Python/Node static file/lock/hash/arithmetic checks only; no builds or product execution','manifest_sha256':sha((REVIEW/'MANIFEST.json').read_bytes()),'patch_sha256':manifest['patch_sha256'],'reconstructed_files':checks,'NUM_origin_commit':dep,'NUM_subset_manifest_sha256':sha(subset_raw),'inherited_supporting_files':origin,'lock_comparisons':lock_results,'audit_lock_repair_backchecked':True,'original_numerical_reference_bytes_unchanged':True,'negative_oracle_exceeds_protected_criterion':True,'final_source_hashes':final}
(HERE/'VERIFICATION.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({'reconstructed_files':checks,'inherited_supporting_files':len(origin),'locks_checked':len(lock_results),'repaired_audit_lock_sha256':sha((ROOT/audit).read_bytes()),'unchanged_frozen_paths':sum(x['matches_original_freeze'] for x in final)}))
