#!/usr/bin/env python3
"""Apply only accepted SCA-011 Group-2 carrier postimages; no Group-3 operation."""
import csv,hashlib,json,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[7]
RUN=Path(__file__).resolve().parents[2]
def sha(b):return hashlib.sha256(b).hexdigest()
def digest(p):return sha(p.read_bytes()) if p.is_file() else 'ABSENT'
def rel(p):return str(p.relative_to(ROOT))
assert (ROOT/'AGENTS.md').is_file(),ROOT
snapshot=RUN.parent/'checkpoint_snapshots/SCA-011_GROUP-2_2026-09-22'
for a in csv.DictReader((snapshot/'ACCEPTED_MANIFEST.csv').open()):
 assert digest(ROOT/a['Path'])==a['SHA256'],a['Path']
rows=[r for r in csv.DictReader((RUN/'APPLY_MANIFEST.csv').open()) if r['ApplicationLane']=='GROUP2_APPLICATION' and '/PKG-' in r['CanonicalTarget']]
assert len(rows)==58
meta=json.loads((RUN/'evidence/SOURCES_DECOMPOSITION.json').read_text())
operations={r['target']:r for r in meta['application_metadata']['files']}
operations.update({r['target']:r for r in json.loads((RUN/'interfaces/INTERFACE_PROMOTION.json').read_text())['files']})
prepared=[]
for r in rows:
 p=ROOT/r['CanonicalTarget'];c=ROOT/r['CandidatePath'];body=c.read_bytes()
 assert digest(c)==r['CandidateSHA256'],str(c)
 text=body.decode()
 for op in operations.get(r['CanonicalTarget'],{}).get('operations',[]):
  assert op['operation']=='replace_exact'
  assert text.count(op['old'])==op['expected_occurrences'],str(p)
  text=text.replace(op['old'],op['new'])
 body=text.encode();assert sha(body)==r['AppliedSHA256'],str(p)
 prepared.append((r,p,body))
if '--check' in sys.argv:
 for r,p,b in prepared:assert digest(p)==r['AppliedSHA256'],str(p)
 print('PASS: all 58 canonical carrier hashes equal accepted Group-2 applied hashes')
 sys.exit(0)
# Verify every preimage before any target write. Existing empty files are never new.
for r,p,b in prepared:assert digest(p)==r['BeforeSHA256'],str(p)
source_paths=[ROOT/x for x in ['AGENTS.md','agents/AGENT_TASK.md','projects/chirality-piping/AGENTS.md','.agents/skills/preparation/SKILL.md','.agents/skills/preparation/references/scaffold-contract.md','workflows/project-setup/execution.json','workflows/project-setup/WORKFLOW.md','workflows/project-setup/resources/contract.md','workflows/project-setup/resources/method.md','tools/scaffolding/scaffold_deliverable.sh','tools/validation/check_min_viable_fileset.sh','tools/scope_of_work/validate_scope_of_work.py','tools/scope_of_work/common.py']]
source_paths += [RUN/'APPLY_MANIFEST.csv',RUN/'DECISION_PACKAGE.md',RUN/'evidence/SOURCES_DECOMPOSITION.json',RUN/'interfaces/INTERFACE_PROMOTION.json',snapshot/'DECISION.md',snapshot/'ACCEPTED_MANIFEST.csv',ROOT/'projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md']
paired=[]
for d in sorted({p.parent for r,p,b in prepared if r['BeforeSHA256']!='ABSENT'}):
 for n in ['_STATUS.md','MEMORY.md','_MEMORY.md']:
  q=d/n
  if q.exists():source_paths.append(q);paired.append({'path':rel(q),'sha256_before':digest(q),'reading':'Current status/Remaining and paired continuity excerpts; immutable full-file identity retained, not an assertion of full semantic reread'})
source_paths += [ROOT/r['CandidatePath'] for r in rows]
inventory=[{'path':r['CanonicalTarget'],'exists_before':p.exists(),'sha256_before':digest(p),'parent_exists_before':p.parent.exists(),'candidate_sha256':r['CandidateSHA256'],'expected_applied_sha256':r['AppliedSHA256']} for r,p,b in prepared]
(RUN/'application/carriers/INVENTORY_BEFORE.json').write_text(json.dumps(inventory,indent=2)+'\n')
(RUN/'application/SOURCES_CARRIER_APPLICATION.json').write_text(json.dumps({'selected_workflow':'bundled:chirality-root/project-setup','selected_skill':'.agents/skills/preparation/SKILL.md','selection_scope':'Bounded materialization executor under WORKING_ITEMS parent, not independent orchestration','sources':[{'path':rel(q),'sha256':digest(q)} for q in dict.fromkeys(source_paths)],'paired_continuity':paired},indent=2)+'\n')
created=[];updated=[]
for r,p,b in prepared:
 p.parent.mkdir(parents=True,exist_ok=True)
 if r['BeforeSHA256']=='ABSENT':
  with p.open('xb') as f:f.write(b)
  created.append(rel(p))
 else:
  assert digest(p)==r['BeforeSHA256'],str(p)
  p.write_bytes(b);updated.append(rel(p))
 assert digest(p)==r['AppliedSHA256'],str(p)
report={'status':'PASS','actor':'/root/piping_scope_manager/piping_apply_carriers','parent':'/root/piping_scope_manager','role':'TASK','mechanism':'Codex native collaboration spawn','model_effort':'Inherited; no override or diversity claim','enforcement':'Actual host plus narrower task brief; no Git, delegation, production code or Group-3 writes','group2_targets':58,'created_paths':created,'updated_paths':updated,'skipped_paths':[], 'helper_choice':'Standard scaffold_deliverable.sh unconditionally creates _DEPENDENCIES.md, owned by the concurrent dependency executor. Used pre-inventoried exclusive file creation for only the exact accepted new targets. No placeholder outside the accepted target list. This script is a bounded application tool, not a revised reusable workflow.', 'commands':['python3 '+rel(Path(__file__)),'python3 '+rel(Path(__file__))+' --check'],'results':[{'target':r['CanonicalTarget'],'before_sha256':r['BeforeSHA256'],'candidate_sha256':r['CandidateSHA256'],'applied_sha256':digest(p)} for r,p,b in prepared],'validation':'SoW and combined minimum-fileset checks to follow in carriers/VALIDATION.json','limitations':['Group 3 remains unexecuted','No product, native, engineering, release, schema-publication or lifecycle-advancement claim']}
(RUN/'application/CARRIER_APPLICATION.json').write_text(json.dumps(report,indent=2)+'\n')
print('PASS:',len(created),'new targets;',len(updated),'existing targets; 58 exact postimage hashes')
