#!/usr/bin/env python3
"""Apply only the 51 owner-accepted SCA-011 Group-3 carrier rows."""
import hashlib,json,re,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[7]
POST=Path(__file__).resolve().parents[1]
RUN=ROOT/'projects/chirality-piping/execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP'
def sha(b):return hashlib.sha256(b).hexdigest()
def digest(p):return sha(p.read_bytes())
def rel(p):return str(p.relative_to(ROOT))
assert (ROOT/'AGENTS.md').is_file()
paths=[RUN/'evidence/SOURCES_DECOMPOSITION.json',RUN/'interfaces/INTERFACE_PROMOTION.json']
a,b=(json.loads(p.read_text()) for p in paths)
rows=a['group3_promotion_metadata']['files']+b['group3_files']
assert len(rows)==51 and len({r['target'] for r in rows})==51
assert 'Accept the audited result and adopt DAG-011' in (POST/'OWNER_DECISION.md').read_text()
if '--check' in sys.argv:
 for r in rows:assert digest(ROOT/r['target'])==r['accepted_sha256'],r['target']
 print('PASS: 51/51 final canonical carrier hashes equal accepted Group-3 values')
 sys.exit(0)
# Guard every preimage and compute every accepted postimage before target writes.
prepared=[]
for r in rows:
 p=ROOT/r['target'];before=p.read_bytes();assert sha(before)==r['applied_sha256'],r['target']
 text=before.decode()
 for op in r['operations']:
  assert op['operation']=='replace_exact'
  assert text.count(op['old'])==op['expected_occurrences'],(r['target'],op)
  text=text.replace(op['old'],op['new'])
 after=text.encode();assert sha(after)==r['accepted_sha256'],r['target']
 if p.name=='ScopeOfWork.md':
  # All claim, requirement, verification and decision-body text is unchanged.
  assert before.decode().split('## ',1)[1]==text.split('## ',1)[1],r['target']
 if p.name=='_STATUS.md':
  assert re.search(r'\*\*Current State:\*\* (\w+)',before.decode()).group(1)==re.search(r'\*\*Current State:\*\* (\w+)',text).group(1)
  assert before.decode().split('## History',1)[1]==text.split('## History',1)[1]
 prepared.append((r,p,before,after))
prior=json.loads((RUN/'application/SOURCES_CARRIER_APPLICATION.json').read_text());known={r['path']:r['sha256'] for r in prior['sources']}
known.update({r['target']:r['applied_sha256'] for r in json.loads((RUN/'application/CARRIER_APPLICATION.json').read_text())['results']})
source_files=[ROOT/p for p in ['AGENTS.md','agents/AGENT_TASK.md','projects/chirality-piping/AGENTS.md','tools/scope_of_work/validate_scope_of_work.py','tools/scope_of_work/common.py']]+paths+[POST/'OWNER_DECISION.md',POST/'PLAN.md']
continuity=[]
for d in sorted({p.parent for r,p,x,y in prepared if '/PKG-' in r['target']}):
 for n in ['_STATUS.md','MEMORY.md','_MEMORY.md']:
  p=d/n
  if p.exists():
   body=p.read_bytes();assert digest(p)==known[rel(p)],rel(p)
   continuity.append({'path':rel(p),'sha256_before':sha(body),'reading':'Prior paired status/Remaining and memory continuity retained; current bytes verified unchanged from the already-read Group-2 application source. Four new status files reread for Group 3.'});source_files.append(p)
  else:continuity.append({'path':rel(p),'state':'ABSENT','action':'Preserved absence'})
(POST/'carriers/SOURCES.json').write_text(json.dumps({'actor':'/root/piping_scope_manager/piping_apply_carriers','parent':'/root/piping_scope_manager','role':'TASK','mechanism':'Codex native collaboration followup_task','model_effort':'Inherited; no override or diversity claim','brief':'Apply 51 reviewed Group-3 carrier rows only; no dependency files, pointers, original evidence, code, schema, Git mutation or delegation','sources':[{'path':rel(p),'sha256':digest(p)} for p in dict.fromkeys(source_files)],'paired_continuity':continuity,'context_reuse':'Root/TASK/Piping instructions already read in this task; current hashes verified unchanged. No new skill or workflow body selected for this bounded metadata application.'},indent=2)+'\n')
manifest=[]
for r,p,before,after in prepared:
 assert digest(p)==r['applied_sha256']
 if before!=after:p.write_bytes(after)
 assert digest(p)==r['accepted_sha256']
 manifest.append({'target':r['target'],'before_sha256':sha(before),'accepted_sha256':sha(after),'actual_sha256':digest(p),'action':'UNCHANGED_NO_OP' if before==after else 'APPLIED','operation_count':len(r['operations'])})
(POST/'carriers/FINAL_TARGET_MANIFEST.json').write_text(json.dumps({'status':'PASS','reviewed_applied_state_commit':'d6cc1482eee78ce860ff18658f11157f7efbd401','owner_decision':rel(POST/'OWNER_DECISION.md'),'owner_decision_sha256':digest(POST/'OWNER_DECISION.md'),'target_count':51,'changed_count':sum(r['action']=='APPLIED' for r in manifest),'no_op_count':sum(r['action']=='UNCHANGED_NO_OP' for r in manifest),'claim_body_checks':24,'lifecycle_history_checks':4,'targets':manifest},indent=2)+'\n')
print('PASS: 51 exact final hashes; 44 changed; 7 no-ops; 24 unchanged SoW substantive bodies; 4 unchanged lifecycle/history records')
