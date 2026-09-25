"""Read-only evidence/data checks. Does not execute recipe, Cargo, or harness."""
from pathlib import Path
import datetime, difflib, hashlib, json

R=Path('/private/tmp/piping-numerical-corrections-20260924')
P=R/'projects/chirality-piping'
B=P/'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/SOLVER_MANAGER/ORDINARY_NUMERICAL_CUT'
H=B/'FIXTURE_GENERATION/RECIPE_REPAIR/REPAIR_01/_run_records/fault_controls'
D=B/'FIXTURE_GENERATION/RECIPE_REPLAY_01/_run_records'
E=B/'FIXTURE_ROUTE_REVIEW/RECIPE_REVIEW/RUNTIME_BACKCHECK/_run_records'
h=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
j=lambda p:json.loads(p.read_text())
prep=j(H/'PREPARATION_FREEZE.json')
result=j(D/'RESULT.json')
record=j(P/'fixtures/product_preview/precision_fixture_generation.json')
checks={
 'all 13 harness preparation identities match':all(h(H/x['path'])==x['sha256'] and (H/x['path']).stat().st_size==x['bytes'] for x in prep['files']),
 'harness and maintained recipe equal reviewed source':h(H/'recipe.mjs')==h(P/'tools/serialization/generate_product_preview_mechanics.mjs')=='4ebeca47a8040f8334ea38ea4c36bb5d50078b7ad2912a0981a9e86e3d10d399',
 'maintained package equals reviewed proposal':h(P/'package.json')=='ae2274d2078a378387a82d99af3fd49d58232d62aa4c383d2511cff02bff2a63',
 'real maintained replay successful recorded exit':result['exit_code']==0 and result['command']==['npm','run','generate:product-preview-mechanics'],
 'replay raw stdout and stderr hashes':h(D/'stdout.log')==result['stdout_sha256'] and h(D/'stderr.log')==result['stderr_sha256'],
 'all replay preimages verified':all(h(D/('before-'+name))==digest for name,digest in result['before'].items()),
 'all replay after hashes match maintained data':all(h(P/'fixtures/product_preview'/name)==digest for name,digest in result['after'].items()),
 'source inventories before and after identical':record['source_input_files']==record['source_input_files_after'],
 'all 128 source inventory identities match':all(h(P/name)==digest for name,digest in record['source_input_files'].items()),
 'record binds exact applied recipe':record['recipe']['sha256']==h(P/record['recipe']['path']),
 'inventory JSON hash recomputed':hashlib.sha256(json.dumps(record['source_input_files'],ensure_ascii=False,separators=(',',':')).encode()).hexdigest()==record['inventory_json_sha256'],
}
inventory=[]
for output in record['outputs']:
 raw=P/output['path'];data=j(raw);name=raw.name
 checks[output['mode']+' replay bytes equal original preserved preimage']=raw.read_bytes()==(D/('before-'+name)).read_bytes() and h(raw)==output['sha256']
 checks[output['mode']+' original producer capture equals replay']=raw.read_bytes()==(B/'FIXTURE_GENERATION/_run_records'/('sparse.stdout.json' if output['mode']=='sparse_interactive' else 'dense.stdout.json')).read_bytes()
 checks[output['mode']+' exact command and recorded successful exit']=output['command'][-2:]==['--',output['mode']] and output['exit_code']==0
 mode_rows=[r for r in data['results'] if r['kind']=='linear_solver_mode_basis']
 inventory.append({'mode':output['mode'],'sha256':h(raw),'rows':len(data['results']),'recorded_rows':output['row_count'],'numerical_status':data['numerical_quality']['status'],'mechanics':data['status']['mechanics'],'mode_rows':[{'case':r['basis_ref']['ref_id'],'value':r['value'],'basis':r['metadata']['basis']} for r in mode_rows]})
 checks[output['mode']+' output metadata agrees with bytes']=len(data['results'])==output['row_count'] and data['status']['mechanics']==output['mechanics_status'] and data['numerical_quality']['status']==output['numerical_status']
doc=P/'fixtures/product_preview/PRECISION_FIXTURES.md';before_doc=D/'PRECISION_FIXTURES.before.md'
(E/'PRECISION_FIXTURES.diff').write_text(''.join(difflib.unified_diff(before_doc.read_text().splitlines(True),doc.read_text().splitlines(True),fromfile='before/projects/chirality-piping/fixtures/product_preview/PRECISION_FIXTURES.md',tofile='after/projects/chirality-piping/fixtures/product_preview/PRECISION_FIXTURES.md')))
report={'time_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'checks':checks,'all_passed':all(checks.values()),'source_input_count':len(record['source_input_files']),'dependency_count':len(record['dependencies']),'observed_record_tools':record['tools'],'inventory':inventory,'doc_before_sha256':h(before_doc),'doc_after_sha256':h(doc),'limits':'Evidence backcheck of parent-run genuine replay; no reviewer execution of generator, Cargo, Node, harness or tests. Transaction controls separate; no Current/physics/full-cut claim.'}
(E/'REPLAY_BACKCHECK.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({'all_passed':report['all_passed'],'checks':len(checks),'failures':[n for n,v in checks.items() if not v],'source_inputs':report['source_input_count'],'doc_sha256':h(doc)},indent=2))
