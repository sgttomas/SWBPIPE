"""Read/hash own physical files and read-only Git ignore metadata; write own inventory."""
from pathlib import Path
import json,hashlib,subprocess
out=Path(__file__).resolve().parent
root=out
while root.name!='chirality-3d-design-20260913':root=root.parent
files=sorted(p for p in out.rglob('*') if p.is_file() and p.name!='MANIFEST.json')
relative=[str(p.relative_to(root)) for p in files]
cmd=['git','check-ignore','--no-index','--stdin']
r=subprocess.run(cmd,input='\n'.join(relative)+'\n',text=True,capture_output=True,cwd=root)
ignored=set(r.stdout.splitlines())
record={'command':cmd,'cwd':str(root),'stdin':relative,'stdout':r.stdout,'stderr':r.stderr,'exitCode':r.returncode,'semantics':'read-only ignore inventory; exit 1 means no supplied paths ignored'}
(out/'_run_records/IGNORE_INVENTORY_COMMAND.json').write_text(json.dumps(record,indent=2)+'\n')
files=sorted(p for p in out.rglob('*') if p.is_file() and p.name!='MANIFEST.json')
rows=[{'path':str(p.relative_to(out)),'bytes':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'ignored':str(p.relative_to(root)) in ignored} for p in files]
manifest={'status':'CANDIDATE_SIX_CONFIGURATION_INTERACTION_SELF_CHECK_PASS_REVIEW_PENDING','sourceHEAD':'8f27fa3d8ec5e128e61fd3ac4076e74d7955f355','role':'TASK Type 2','parent':'/root/next_ui_design','derivativePackage':True,'acceptedUpstreamPlanSHA256':'74589c9e83f3cccbe5b0dc46a2d13cd412d5fd91ec4fea62026bdb7bd67e3536','closure':'NOT_CLOSED','files':rows,'directories':[str(p.relative_to(out)) for p in sorted(out.rglob('*')) if p.is_dir()],'totalEnumeratedBytes':sum(r['bytes'] for r in rows),'fileCountExcludingManifest':len(rows),'manifestSelfExcluded':'This file cannot recursively contain its own hash; caller computes final manifest hash','tested':['workspace.js syntax','capabilities.js syntax','witness.mjs syntax','witness-v2.mjs syntax','six-configuration three-journey actual UI self-check'],'untested':['independent source/interaction witness','full keyboard-only/a11y/usability acceptance','SVG render witness','split/unreferenced-transform application','support update/spring/nonlinear engineering form application'],'limitations':['all engineering and engine outcomes simulated','no production/Rust/schema/hash/numerical/acceptance proof']}
(out/'MANIFEST.json').write_text(json.dumps(manifest,indent=2)+'\n')
