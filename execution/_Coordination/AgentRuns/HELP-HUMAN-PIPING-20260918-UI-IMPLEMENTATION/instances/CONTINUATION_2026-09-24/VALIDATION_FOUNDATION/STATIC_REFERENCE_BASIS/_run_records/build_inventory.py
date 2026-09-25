"""Static acquisition inventory only. Never imports/executes acquired decks."""
from pathlib import Path
import json,hashlib,ast,subprocess
r=Path('/Users/ryan/.codex/worktrees/6614/chirality')
p=Path('/private/tmp/piping-static-reference-acquisition-20260925')
d=Path(__file__).parents[1]
records=[]
for fn in ['initial_acquisition.json','pinned_acquisition.json','dependency_acquisition.json','method_acquisition.json','assertion_semantics_acquisition.json']:
    records+=json.loads((p/fn).read_text())
meta=json.loads((p/'src_main.json').read_text());commit=meta['id']
for a in records:
    if a.get('status')==200:
        name=a['name']
        if not(p/name).exists():name += '.json' if name.endswith('main') else '.html'
        a['private_inspection_filename']=name
        assert hashlib.sha256((p/name).read_bytes()).hexdigest()==a['sha256']
exports=[]
for f in sorted(p.glob('*.export')):
    deps=[]
    for line in f.read_text().splitlines():
        words=line.split()
        if words and words[0]=='F':
            q=p/words[2];b=q.read_bytes()
            deps.append({'kind':words[1],'name':q.name,'mode':words[3],'logical_unit':words[4],'retrieved':True,'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest(),'container':'HDF5/MED signature' if b.startswith(b'\x89HDF\r\n\x1a\n') else 'text','content_decode':'command/dependency text inspected' if q.suffix in ['.comm','.com1','.export','.datg'] else 'input bytes available; full mesh/group interpretation not qualified'})
    exports.append({'test_id':f.stem,'export_sha256':hashlib.sha256(f.read_bytes()).hexdigest(),'dependencies':deps,'static_dependency_scope':'All F declarations acquired; command read/include scan found no extra external input beyond Code_Aster runtime and listed inputs. No deck run or runtime qualification.'})
(d/'ACQUISITION.json').write_text(json.dumps({'source_commit':commit,'source_commit_time':meta['committed_date'],'inspection_date':'2026-09-25','private_asset_root':'TEMP_ACQUISITION_ROOT; host path in run script only','source_assets_vendored':False,'retrievals':records,'declared_test_dependencies':exports,'errors_retained':'Web cache failures recovered via direct public HTTP; doc-project API404 and ssll101b404 remain unavailable endpoints. No source asset is qualified merely by download.'},indent=2)+'\n')
inventory=[]
for f in sorted(p.glob('ssll*.comm')):
    text=f.read_text();tree=ast.parse(text);counter={};specific=[]
    for node in ast.walk(tree):
        if not isinstance(node,ast.Call):continue
        kw={x.arg:x.value for x in node.keywords}
        if 'VALE_CALC' not in kw and 'VALE_REFE' not in kw:continue
        ref=ast.literal_eval(kw['REFERENCE']) if 'REFERENCE' in kw and isinstance(kw['REFERENCE'],ast.Constant) else 'not_explicitly_external_or_analytical'
        counter[ref]=counter.get(ref,0)+1
        if f.name=='ssll106a.comm' and node.lineno in range(175,252):
            specific.append({'line':node.lineno,'end_line':node.end_lineno,'reference_class':ref,'field':ast.literal_eval(kw['NOM_CHAM']) if 'NOM_CHAM' in kw else None,'component':ast.literal_eval(kw['NOM_CMP']) if 'NOM_CMP' in kw else None,'reference_expression':ast.get_source_segment(text,kw['VALE_REFE']) if 'VALE_REFE' in kw else None})
    inventory.append({'file':f.name,'sha256':hashlib.sha256(f.read_bytes()).hexdigest(),'AST_static_only':True,'assertion_blocks_by_explicit_reference':counter,'source_semantics':'VALE_CALC and VALE_REFE/REFERENCE are separate carrier fields; no source execution and no independent target admission','axial_assertion_locators':specific})
(d/'ASSERTION_LINEAGE.json').write_text(json.dumps(inventory,indent=2)+'\n')
files=['AGENTS.md','agents/AGENT_HELPS_HUMANS.md','projects/chirality-piping/AGENTS.md','projects/chirality-piping/loop/LOOP_INIT.md','projects/chirality-piping/docs/references/validation-programme/README.md','projects/chirality-piping/docs/references/validation-programme/piping_stress_solver_research_and_agent_plan.md','projects/chirality-piping/docs/references/validation-programme/piping_solver_benchmark_research_and_agent_plan.md']
basis=[]
for path in files:
    q=r/path;basis.append({'origin':'REPO_ROOT/'+path,'sha256':hashlib.sha256(q.read_bytes()).hexdigest(),'kind':'instructions or external-agent lead; not target authority'})
for path in ['projects/chirality-piping/core/product_physics/src/lib.rs','projects/chirality-piping/core/product_physics/src/pressure_runtime.rs','projects/chirality-piping/core/solver/frame_kernel/src/lib.rs']:
    b=subprocess.check_output(['git','show','22452ecd148e86486fe36a36e34a184c9a5a5943:'+path],cwd=r)
    basis.append({'origin':'git:22452ecd148e86486fe36a36e34a184c9a5a5943:'+path,'sha256':hashlib.sha256(b).hexdigest(),'kind':'fixed engine basis, bounded formulation and kernel read only'})
(d/'BASIS.json').write_text(json.dumps({'role':'HELPS_HUMANS','agent':'/root/load_state_design','actual_parent':'/root','mechanism':'delegated-harness-native continuation','assignment':'Bounded SSLL106/SSLL101 static reference applicability and acquisition','write_scope':'C/VALIDATION_FOUNDATION/STATIC_REFERENCE_BASIS','source_basis':basis,'instructions':'Owner resume supersedes planning README historical hold; no solver/campaign run','boundaries':['No product/test/graph/instruction/Git writes','No solver/deck execution, installations or vendored assets','No material/component/code-rule libraries','No repeated load-state/joint/shear derivation'],'primary_manual_versions':'v17 stable and v18 dev as source switcher labels; exact build commit not supplied by rendered docs','primary_source_commit':commit},indent=2)+'\n')
print(json.dumps({'retrieval_records':len(records),'test_export_files':len(exports),'all_declared_F_inputs_retrieved':True,'commands_parsed_without_execution':len(inventory)}))
