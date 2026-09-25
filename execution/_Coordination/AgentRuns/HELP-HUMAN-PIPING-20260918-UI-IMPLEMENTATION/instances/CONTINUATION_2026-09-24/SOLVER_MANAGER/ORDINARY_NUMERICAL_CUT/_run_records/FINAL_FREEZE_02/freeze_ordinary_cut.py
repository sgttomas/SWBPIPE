from pathlib import Path
import subprocess,json,hashlib
R=Path('/private/tmp/piping-numerical-corrections-20260924');P=R/'projects/chirality-piping';D=P/'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/SOLVER_MANAGER/ORDINARY_NUMERICAL_CUT';O=D/'_run_records/FINAL_FREEZE_02'
def git(*args):return subprocess.check_output(['git',*args],cwd=R)
tracked=set(git('diff','--name-only','HEAD').decode().splitlines());new=set(git('ls-files','--others','--exclude-standard').decode().splitlines())
allpaths=tracked|new
source=sorted(n for n in allpaths if '/execution/' not in n)
assert all(n.startswith('projects/chirality-piping/') for n in allpaths)
held=('extended_arithmetic','source_capture','core/analysis_runs/numerical_evidence','EXTENDED_','dashu','astro')
assert not [n for n in source if any(x in n for x in held)]
O.mkdir(parents=True,exist_ok=False)
patch=git('diff','--binary','HEAD','--',*[n for n in source if n in tracked])
for n in source:
 if n in new:
  p=subprocess.run(['git','diff','--no-index','--binary','--','/dev/null',n],cwd=R,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
  assert p.returncode in (0,1),p.stderr
  patch+=p.stdout
(O/'COMPLETE_SOURCE.patch').write_bytes(patch)
rows=[]
for n in source:
 p=R/n;rows.append({'path':n,'sha256':hashlib.sha256(p.read_bytes()).hexdigest() if p.exists() else None,'bytes':p.stat().st_size if p.exists() else 0,'git_state':'tracked_delta' if n in tracked else 'new'})
record={'status':'frozen for fresh complete-cut review; actual cut broad checks remain pending','base_head':git('rev-parse','HEAD').decode().strip(),'path_base':'repository_root','source_patch':'_run_records/FINAL_FREEZE_02/COMPLETE_SOURCE.patch','source_patch_sha256':hashlib.sha256(patch).hexdigest(),'source_files':rows}
(D/'REVISED_SOURCE_FREEZE_02.json').write_text(json.dumps(record,indent=2)+'\n')
evidence=[]
for n in sorted(allpaths):
 if '/execution/' in n and (R/n).is_file():evidence.append({'path':n,'sha256':hashlib.sha256((R/n).read_bytes()).hexdigest(),'bytes':(R/n).stat().st_size})
(O/'EVIDENCE_WHITELIST.json').write_text(json.dumps({'path_base':'repository_root','note':'Evidence inventory at source-freeze time; later review/check records are separately additive, no historical rewrites implied','files':evidence},indent=2)+'\n')
print(json.dumps({'source_files':len(rows),'patch_sha256':record['source_patch_sha256'],'evidence_files':len(evidence),'base':record['base_head']}))
