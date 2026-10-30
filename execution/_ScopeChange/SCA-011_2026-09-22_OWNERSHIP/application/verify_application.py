#!/usr/bin/env python3
"""Check initial accepted application plus explicit repaired postimages; never apply/promote."""
import csv,hashlib,json,re,subprocess
from pathlib import Path
RUN=Path(__file__).resolve().parents[1]
ROOT=RUN
while not (ROOT/'tools/scope_of_work/validate_scope_of_work.py').exists(): ROOT=ROOT.parent
BASE='3e18334eca72509475684cc86786b3eeade83572'
def sha(b): return hashlib.sha256(b).hexdigest()
def blob(path): return subprocess.check_output(['git','show',BASE+':'+path],cwd=ROOT)
rows=list(csv.DictReader((RUN/'APPLY_MANIFEST.csv').open()))
checks=[];errors=[]
deviations=json.loads((RUN/'application/dependencies/POSTIMAGE_DEVIATIONS.json').read_text())
repairs={r['CanonicalTarget']:r for r in deviations['files']}
if len(repairs)!=21 or deviations['semantic_change'] or deviations['group3_executed']:errors.append('INVALID_REPAIR_BOUNDARY')
for r in deviations['changed_paths']:
 p=ROOT/r['path']
 if not p.exists()or sha(p.read_bytes())!=r['after_repair_sha256']:errors.append('REPAIR_POSTIMAGE_MISMATCH '+r['path'])
for row in rows:
 p=ROOT/row['CanonicalTarget'];observed=sha(p.read_bytes())if p.exists()else'ABSENT'
 repair=repairs.get(row['CanonicalTarget'])
 if repair and repair['AcceptedAppliedSHA256']!=row['AppliedSHA256']:errors.append('REPAIR_ACCEPTED_BASE_MISMATCH '+row['CanonicalTarget'])
 expected=repair['RepairedSHA256']if repair else row['AppliedSHA256']
 good=observed==expected
 checks.append({'target':row['CanonicalTarget'],'accepted_applied_sha256':row['AppliedSHA256'],'expected_sha256':expected,'explicit_repair':bool(repair),'observed_sha256':observed,'pass':good})
 if not good:errors.append('POSTIMAGE_MISMATCH '+row['CanonicalTarget'])
 if p.name=='_STATUS.md' and row['BeforeSHA256']!='ABSENT':
  memory=next((p.parent/n for n in ['_MEMORY.md','MEMORY.md'] if (p.parent/n).exists()),None)
  if memory: checks[-1]['paired_memory_path']=str(memory.relative_to(ROOT));checks[-1]['paired_memory_sha256']=sha(memory.read_bytes())
  old=blob(row['CanonicalTarget']).decode();new=p.read_text()
  a=re.search(r'\*\*Current State:\*\*\s*(\S+)',old);b=re.search(r'\*\*Current State:\*\*\s*(\S+)',new)
  if not a or not b or a.group(1)!=b.group(1): errors.append('LIFECYCLE_CHANGE '+row['CanonicalTarget'])
pointers=['projects/chirality-piping/execution/_ScopeChange/_LATEST.md','projects/chirality-piping/execution/_DAG/_LATEST.md']
for name in pointers:
 if (ROOT/name).read_bytes()!=blob(name):errors.append('POINTER_CHANGED '+name)
protected=['projects/chirality-piping/execution/_DAG/DAG-010','projects/chirality-piping/execution/_ScopeChange/SCA-009_2026-08-20_0000','projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS','projects/chirality-piping/core','projects/chirality-piping/apps','projects/chirality-piping/schemas','projects/chirality-piping/tests']
for name in protected:
 changed=subprocess.check_output(['git','diff','--name-only',BASE,'--',name],cwd=ROOT,text=True)
 if changed.strip():errors.append('PROTECTED_PATH_CHANGED '+name)
manifest=json.loads((RUN/'DEPENDENCY_SNAPSHOT_MANIFEST.json').read_text())
for row in manifest['files']:
 p=ROOT/row['proposed_target']
 repair=repairs.get(row['proposed_target'])
 if repair and repair['AcceptedAppliedSHA256']!=row['sha256']:errors.append('REPAIR_STAGED_BASE_MISMATCH '+str(p))
 expected=repair['RepairedSHA256']if repair else row['sha256']
 if not p.exists() or sha(p.read_bytes())!=expected:errors.append('STAGED_DAG_MISMATCH '+str(p))
report={'status':'PASS'if not errors else'FAIL','errors':errors,'accepted_package_commit':BASE,'actual_postimages_checked':len(rows),'unchanged_accepted_postimages':sum(not r['explicit_repair']for r in checks),'explicitly_repaired_postimages':sum(r['explicit_repair']for r in checks),'repair_manifest_sha256':sha((RUN/'application/dependencies/POSTIMAGE_DEVIATIONS.json').read_bytes()),'files':checks,'accepted_pointers_unchanged':not any(x.startswith('POINTER_CHANGED')for x in errors),'old_history_and_production_paths_unchanged':not any(x.startswith('PROTECTED_PATH_CHANGED')for x in errors),'group3_acceptance':False,'limit':'Accepted initial application plus explicit repaired postimage and preservation checks; no production conformance or original satisfaction recertification.'}
(RUN/'application/APPLICATION_VALIDATION.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({k:v for k,v in report.items()if k!='files'},indent=2))
raise SystemExit(bool(errors))
