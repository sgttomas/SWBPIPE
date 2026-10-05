"""RV65 dense delta: immutable-file verification and recorded-log inspection only."""
from pathlib import Path
import datetime, hashlib, json, os, platform, struct, subprocess
NUM=Path('/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/numerics')
CODE=NUM.parent/'f2a'
R=Path('projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30')
OUT=Path(__file__).resolve().parent
PIN='d87eb0fb59214a561f37a9ce5dcae4ba4c4bcf75'
SOURCE='d0daa18717f8243a7232e898c9ef9b4f4d18d9e4'
env=dict(os.environ,GIT_OPTIONAL_LOCKS='0')
def sha(b): return hashlib.sha256(b).hexdigest()
def entry(p,scope):
 b=p.read_bytes(); return dict(location=str(p),bytes=len(b),sha256=sha(b),scope=scope)
def pinned(p,pin):
 b=subprocess.check_output(['git','show',pin+':'+str(p)],cwd=str(NUM),env=env)
 assert b==(NUM/p).read_bytes(), str(p)
 return b
manifest_path=R/'verification/i50_first_observed_02/FIRST_RUN.json'
raw=pinned(manifest_path,PIN)
assert sha(raw)=='1773eb7d47e432b8cb1440a95815938951591fd4352b2751909d56886ede7d0e'
assert raw==(CODE/R/'I50/first_publishing_component_02/FIRST_RUN.json').read_bytes()
manifest=json.loads(raw)
external=[]
for e in manifest['candidate_sources']+manifest['evidence']:
 p=Path(e['location']); b=p.read_bytes()
 assert len(b)==e['bytes'] and sha(b)==e['sha256']
 external.append(dict(e,verification='hash and size match'))
run=Path(next(e['location'] for e in manifest['evidence'] if e['location'].endswith('/pp_named_second.json')))
runjson=json.loads(run.read_text())
assert runjson['exit']==101 and runjson['source_stable']
assert runjson['before_digest']==runjson['after_digest']==manifest['source_digest']
log=Path(next(e['location'] for e in manifest['evidence'] if e['location'].endswith('/pp_named_second.log')))
records=[json.loads(line[11:]) for line in log.read_text().splitlines() if line.startswith('I50_RECORD ')]
assert len(records)==2
summary=[]
for x in records:
 rows=x['envelope']['results']; ancillary=[]
 for i,row in enumerate(rows):
  if row['kind'] in ('linear_solver_mode_basis','sparse_live_path_dense_parity_relative_delta'):
   ancillary.append(dict(ordinal=i,kind=row['kind'],value_bits=struct.pack('>d',row['value']).hex(),
    value=row['value'],unit=row['unit'],id=row['id'],entity_ref=row['entity_ref'],basis_ref=row['basis_ref'],
    metadata_basis_sha256=sha(row['metadata']['basis'].encode()),metadata_location=row['metadata']['location']))
 summary.append(dict(mode=x['mode'],native_Q=len(x['native']['rows']),final_rows=len(rows),verdicts=len(x['verdicts']),
    hooks=x['hooks'],error=x['error'],numeric_failure=x['numeric_failure'],numeric_pass=x['numeric_pass'],
    g5a=x['g5a'],observable=x['observable'],full_case=x['full_case'],ancillary=ancillary))
assert [(x['mode'],x['native_Q'],x['final_rows'],x['verdicts']) for x in summary]==[
 ('sparse_interactive',58,98,98),('dense_scrutiny',58,99,0)]
assert summary[0]['error']==summary[0]['numeric_failure']==summary[0]['g5a']==summary[0]['observable']=='None'
assert not summary[0]['numeric_pass'] and not summary[0]['full_case']
assert summary[1]['error']=='Some(Association("ordinary sparse mode record"))'
assert summary[1]['ancillary'][1]['value_bits']=='3e2c76280d1333f1'
assert [x['ancillary'][0]['value_bits'] for x in summary]==['3ff0000000000000','4000000000000000']
origins=[]
paths=[
 ('AGENTS.md','inherited active instructions, not reloaded'),('agents/AGENT_TASK.md','inherited active role, not reloaded'),
 ('projects/chirality-piping/AGENTS.md','inherited project instructions, not reloaded'),
 ('.agents/skills/software-code-review/SKILL.md','continued selected skill, not reloaded'),
 (str(R/'BRIEFS/RV65_DENSE_OBSERVATION_DELTA.md'),'full active delta brief'),
 (str(R.parent/'DESIGN_STANDING/DESIGN.md'),'introduction and closed table 4.9.10'),
 (str(R.parent/'DESIGN_NUMERICS/DESIGN.md'),'targeted closed classification table 4.1.6.1'),
 (str(R.parent/'SELECTION_PACKAGE.md'),'1-42 design basis and proposal/selection distinction'),
 (str(R.parent/'ROOT_SELECTION_DESIGNS.md'),'full authoritative design selection'),
 (str(R.parent/'ROOT_RULINGS_V1.md'),'targeted classification/adoption search only'),
 (str(R.parent/'ROOT_RULINGS_V2.md'),'targeted selection search only'),
 (str(manifest_path),'complete preserved first-run manifest'),
 (str(R/'verification/i50_first_observed_02/CHECKS.json'),'full ROOT preservation check'),
 (str(R/'REVIEW_RV65/named_support_component_01/SEAL.json'),'prior seal verification; no overwrite'),
 (str(R/'REVIEW_RV65/named_support_component_01/ORIGINS.json'),'continued instruction origins')]
for p,scope in paths:
 p=Path(p)
 if p.name not in ('ROOT_RULINGS_V1.md','ROOT_RULINGS_V2.md'): pinned(p,PIN)
 b=subprocess.check_output(['git','show',PIN+':'+str(p)],cwd=str(NUM),env=env)
 origins.append(dict(location=str(NUM/p),revision=PIN,bytes=len(b),sha256=sha(b),scope=scope))
for rel,scope in [
 ('projects/chirality-piping/core/product_physics/src/lib.rs','actual mode enum, observer hooks, case block 3838-3918, producer 5496-5682, final qualification 2590-2635'),
 ('projects/chirality-piping/core/product_physics/src/source_receipt/rows.rs','existing observation classifier 704-738')]:
 b=pinned(Path(rel),SOURCE); origins.append(dict(location=str(NUM/rel),revision=SOURCE,bytes=len(b),sha256=sha(b),scope=scope))
old=NUM/R/'REVIEW_RV65/named_support_component_01'
s=json.loads((old/'SEAL.json').read_text())
for e in s['files']:
 b=(old/e['path']).read_bytes(); assert len(b)==e['bytes'] and sha(b)==e['sha256']
assert sha((old/'SEAL.json').read_bytes())=='32cb3179a9431534e3bcc87232c46730c9f690628a8cdd740dd2ebfb76fad402'
out=dict(kind='static immutable evidence and selected-source check; no runtime',checked_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
 host=platform.node(),manifest_sha256=sha(raw),all12_external_files_verified=True,first_run_exit=runjson['exit'],
 first_run_source_digest=manifest['source_digest'],frozen_observations=summary,external_manifest=external,
 external_source_scope='first_candidate_0.rs binding/metadata/work; first_candidate_1.rs actual test/helper; first_candidate_3.rs typed coverage/verdict; other snapshots hash-only',
 oracle_reviewed_or_executed=False,old_seal_unchanged=True,origins=origins,
 census_correction='sparse98; observed dense99; same97 quantity rows and Q58; parity presence is actual completed producer fact, not inferred solely from mode',
 assertions_passed=True)
(OUT/'CHECK.json').write_text(json.dumps(out,indent=2)+'\n')
print(json.dumps(dict(assertions_passed=True,manifest_files=len(external),summary=summary,old_seal_unchanged=True),indent=2))
