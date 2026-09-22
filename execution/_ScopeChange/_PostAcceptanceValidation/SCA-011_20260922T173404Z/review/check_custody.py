from pathlib import Path
import csv,json,hashlib,subprocess,datetime
ROOT=Path.cwd(); P=ROOT/'projects/chirality-piping'; POST=P/'execution/_ScopeChange/_PostAcceptanceValidation/SCA-011_20260922T173404Z'; OUT=POST/'review'; BASE='d6cc1482eee78ce860ff18658f11157f7efbd401'; checks=[]
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def check(name,ok,detail=None):checks.append({'check':name,'pass':bool(ok),'detail':detail})
def old(p):return subprocess.check_output(['git','show',BASE+':'+str(p.relative_to(ROOT))],cwd=ROOT)
def hashrefs(value):
 if isinstance(value,dict):
  if 'path' in value and 'sha256' in value: check('hash_binding:'+value['path'],sha(ROOT/value['path'])==value['sha256'])
  for v in value.values():hashrefs(v)
 elif isinstance(value,list):
  for v in value:hashrefs(v)
manifest=POST/'FINAL_REVIEW_MANIFEST.csv'; rows=list(csv.DictReader(manifest.open())); expected='223b75d751013855369d8ed26cf2f7579337b81335d0046f3d354525e672999c'
check('final_manifest_identity',sha(manifest)==expected,sha(manifest)); check('final_manifest_199_unique_entries',len(rows)==len({r['Path']for r in rows})==199)
check('final_manifest_bytes',all((ROOT/r['Path']).is_file() and sha(ROOT/r['Path'])==r['SHA256'] and (ROOT/r['Path']).stat().st_size==int(r['Size'])for r in rows))
paths=set()
for args in [['git','diff','--name-only','-z',BASE,'--','projects/chirality-piping'],['git','ls-files','--others','--exclude-standard','-z','--','projects/chirality-piping']]:paths.update(x.decode()for x in subprocess.check_output(args,cwd=ROOT).split(b'\0')if x)
paths.update(r['target']for r in json.loads((POST/'FINAL_TARGET_MANIFEST.json').read_text())['files'])
ex=[str(manifest.relative_to(ROOT)),str((POST/'INDEPENDENT_FINALIZATION_REVIEW.md').relative_to(ROOT)),str(OUT.relative_to(ROOT))+'/','__pycache__']; paths={x for x in paths if not any(e in x for e in ex)}
check('complete_delta_and_canonical_manifest_coverage',paths=={r['Path']for r in rows},{'missing':sorted(paths-{r['Path']for r in rows}),'extra':sorted({r['Path']for r in rows}-paths)})
dag=P/'execution/_DAG/DAG-011'
for n in ['DependencyEdges.csv','DeliverableNodes.csv']:check('graph_exact_reviewed_bytes:'+n,(dag/n).read_bytes()==old(dag/n))
for n in ['PROVENANCE.json']:hashrefs(json.loads((dag/n).read_text()))
hashrefs(json.loads((POST/'dependencies/ADOPTION_VERIFICATION.json').read_text()))
for line in (dag/'MANIFEST.sha256').read_text().splitlines():
 h,n=line.split(None,1);check('dag_manifest:'+n,sha(dag/n.strip().lstrip('*'))==h)
repair=json.loads((POST/'dependencies/POINTER_EVIDENCE_PACKAGING_REPAIR.json').read_text()); src=ROOT/repair['renamed_evidence']['path']; check('historical_pointer_preserved',src.read_bytes()==old(P/'execution/_DAG/_LATEST.md') and sha(src)==repair['prior_evidence']['sha256'] and not (ROOT/repair['prior_evidence']['path']).exists())
for n in ['CURRENT_SOURCE_BINDINGS.json','QUOTE_CHECKS.json','LOCAL_COMMANDS.json','StageGraph.json','StagedInterfaces.json','VALIDATION.json']:check('dependency_replay_identical:'+n,(OUT/'dependencies'/n).read_bytes()==(POST/'dependencies'/n).read_bytes())
active=json.loads((OUT/'active/raw_checks.json').read_text());check('independent_active_replay_56_pass',len(active['checks'])==56 and all(x['pass']for x in active['checks']))
status=[]; sources=json.loads((OUT/'active/SOURCES.json').read_text())['sources']
for name in sorted(sources):
 if ':' in name or not name.endswith('/_STATUS.md'):continue
 f=ROOT/name; siblings=[f.parent/n for n in ['_MEMORY.md','MEMORY.md'] if (f.parent/n).exists()]
 status.append({'status':name,'sha256':sha(f),'memory':[{'path':str(s.relative_to(ROOT)),'sha256':sha(s),'read_as':'non-authoritative operational context; no reliance to change authority'} for s in siblings]})
(OUT/'STATUS_MEMORY_READS.json').write_text(json.dumps(status,indent=2)+'\n')
origin=['AGENTS.md','agents/AGENT_TASK.md','projects/chirality-piping/AGENTS.md','projects/chirality-piping/loop/LOOP_INIT.md','workflows/scope-change/WORKFLOW.md','workflows/scope-change/resources/method.md']
origin += [str((POST/n).relative_to(ROOT))for n in ['REVIEW_BRIEF.md','OWNER_DECISION.md','CURRENT_STATE.md','PLAN.md','FINAL_REVIEW_MANIFEST.csv']]
(OUT/'CONTEXT_ORIGINS.json').write_text(json.dumps({'role':'TASK','task':'/root/piping_scope_manager/piping_activation_review','parent':'/root/piping_scope_manager','mechanism':'Codex delegated-harness-native descendant; no descendants','method':'bounded finalization review under bundled:chirality-root/scope-change final acceptance method','model_effort':'Inherited host; no override. Exact model/effort not exposed by executor.','enforcement':'Shared checkout, write-bound instructions; not an isolated filesystem sandbox.','sources':[{'path':n,'sha256':sha(ROOT/n)}for n in origin],'additional_reads':'review/active/SOURCES.json records replay source origins and historical commit-qualified reads. STATUS_MEMORY_READS.json records paired context; no memory creates authority.'},indent=2)+'\n')
report={'timestamp':datetime.datetime.now(datetime.timezone.utc).isoformat(),'status':'PASS'if all(c['pass']for c in checks)else'FAIL','base_commit':BASE,'final_manifest_sha256':sha(manifest),'checks':checks}
(OUT/'FINAL_CUSTODY.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'status':report['status'],'checks':len(checks),'failed':[c for c in checks if not c['pass']]},indent=2))
