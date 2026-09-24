import json,pathlib,hashlib,collections,re,sys,datetime
p=pathlib.Path(sys.argv[1]); root=pathlib.Path('/Users/ryan/.codex/worktrees/6614/chirality')
load=lambda f:json.loads(f.read_text())
sha=lambda f:hashlib.sha256(f.read_bytes()).hexdigest()
run=load(p/'run.json'); jobs=load(p/'jobs.json')['jobs']; arts=load(p/'artifacts.json')['artifacts']
assert run['status']=='completed' and run['conclusion']=='success'
expected='a34c9d819d62b54b74d7f07edde87738b97b87cf'
assert run['head_sha']==expected
plan=load(p/'extracted/piping-e2e-selection/piping-e2e-plan.json'); assert plan['mode']=='full' and plan['head']==expected
key=lambda r:(r['id'],r['project'],r['file'],tuple(r['title_path']))
def rows(report):
 out=[]
 def visit(s,titles):
  for sp in s.get('specs',[]):
   for t in sp['tests']: out.append(dict(id=sp['id'],project=t['projectName'],file='e2e/'+sp['file'],title_path=titles+[sp['title']]))
  for c in s.get('suites',[]):visit(c,titles+[c['title']] if c.get('line',0) else titles)
 assert not report.get('errors');visit(report,[]);return out
union=[]; base=None; summaries=[]
for f in sorted((p/'extracted').glob('*/collection.json')):
 c=load(f);assert c['head']==expected and c['status']=='validated' and c['mode']=='full' and not c['omitted']
 source=collections.Counter(map(key,rows(load(f.parent/'source.stdout.json'))));assert source and all(v==1 for v in source.values())
 assert collections.Counter(map(key,c['selected']))==source
 if base is None:base=source
 assert source==base
 partition=collections.Counter(key(t) for v in c['partition'].values() for t in v);assert partition==base
 stage='barrier' if c['stage']=='barrier' else f"shard-{c['shard']}"
 assert collections.Counter(map(key,c['execution_tests']))==collections.Counter(map(key,c['partition'][stage]))
 j=next(j for j in jobs if j['name']==('Accessibility barrier and selected coverage' if stage=='barrier' else f"Source remainder ({c['shard']}/4)"))
 step=next(s for s in j['steps'] if s['name']==('Run accessibility first, then any reduced selection' if stage=='barrier' else 'Run isolated source remainder shard'))
 assert j['head_sha']==expected and j['conclusion']=='success' and step['conclusion']=='success'
 log=(p/f"job-{j['id']}.log").read_text(); running=[int(x) for x in re.findall(r'Running (\d+) tests? using',log)]; passed=[int(x) for x in re.findall(r'Z\s+(\d+) passed \(',log)]; skipped=[int(x) for x in re.findall(r'Z\s+(\d+) skipped\s*$',log,re.M)]
 assert sum(running)==len(c['execution_tests']) and sum(passed)+sum(skipped)==sum(running)
 union+=c['execution_tests']; summaries.append(dict(stage=stage,job_id=j['id'],job_url=j['html_url'],step=step,identities=len(c['execution_tests']),passed=sum(passed),skipped=sum(skipped),collection=str(f.relative_to(p)),collection_sha256=sha(f)))
assert len(summaries)==5 and collections.Counter(map(key,union))==base
report=dict(head=expected,run_id=run['id'],run_url=run['html_url'],run_attempt=run['run_attempt'],canonical_identity=['id','project','file','title_path'],discovered=len(base),union=len(union),missing=0,duplicates=0,unexpected=0,profiles=sorted({t['project'] for t in union}),passed=sum(s['passed'] for s in summaries),skipped=sum(s['skipped'] for s in summaries),jobs=summaries,limitations='Collection-only artifacts establish identities, not execution. Successful hosted execution steps and raw logs independently establish actual execution. Skipped cases are not claimed passed. No engineering correctness or native-host claims.')
(p/'IDENTITY_EXECUTION_CHECK.json').write_text(json.dumps(report,indent=2)+'\n')
binding=dict(workflow_path='.github/workflows/piping-desktop-e2e.yml',run_id=run['id'],run_attempt=run['run_attempt'],head_sha=expected,conclusion='success',registered_e2e_specs_executed=True,viewport_projects=['chromium-desktop','chromium-compact'])
(p/'surface4-ci-binding.json').write_text(json.dumps(binding,indent=2)+'\n')
print(json.dumps({k:report[k] for k in ['head','run_id','discovered','passed','skipped','missing','duplicates','unexpected']}))
