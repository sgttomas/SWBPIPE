#!/usr/bin/env python3
import csv,hashlib,json,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[7];RUN=Path(__file__).resolve().parents[2]
regpath=ROOT/'projects/chirality-piping/docs/_Registers/Deliverables.csv';decomp=ROOT/'projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md'
reg={r['DeliverableID']:r for r in csv.DictReader(regpath.open())};meta=json.loads((RUN/'evidence/SOURCES_DECOMPOSITION.json').read_text())
results=[]
for row in meta['new_deliverables']:
 id=row[0];d=next((ROOT/'projects/chirality-piping/execution').glob('PKG-*/1_Working/'+id+'_*'));c=(d/'_CONTEXT.md').read_text();status=(d/'_STATUS.md').read_text();sem=(d/'_SEMANTIC.md').read_text()
 for f in ['DeliverableID','PackageID','Name','Description','Type']:
  assert reg[id][f] in c,(id,f)
 assert reg[id]['Description']==row[7]
 for f in ['AnticipatedArtifacts','CoversScopeItems','SupportsObjectives']:
  sep=';' if f=='AnticipatedArtifacts' else ','
  assert all(v.strip() in c for v in reg[id][f].split(sep)),(id,f)
 assert f'|{id}|{reg[id]["Name"]}|{reg[id]["Type"]}|' in decomp.read_text(),id
 assert '**Current State:** OPEN' in status,id
 assert 'PLACEHOLDER' in sem,id
 command=['zsh','tools/validation/check_min_viable_fileset.sh',str(d.relative_to(ROOT))]
 check=subprocess.run(command,cwd=ROOT,text=True,capture_output=True)
 results.append({'deliverable_id':id,'metadata_parity':'PASS','state':'OPEN','semantic':'PLACEHOLDER','command':command,'exit_code':check.returncode,'stdout':check.stdout,'stderr':check.stderr,'files':[{'path':str((d/n).relative_to(ROOT)),'sha256':hashlib.sha256((d/n).read_bytes()).hexdigest()} for n in ['_STATUS.md','_CONTEXT.md','_REFERENCES.md','_SEMANTIC.md','_DEPENDENCIES.md']]})
report={'status':'PASS' if all(r['exit_code']==0 for r in results) else 'FAIL','new_deliverable_count':4,'sources':[{'path':str(p.relative_to(ROOT)),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for p in [regpath,decomp]],'results':results,'prior_probe':{'result':'INVALID_CHECK_ASSUMPTION','detail':'An exploratory assertion expected full Description prose in SOFTWARE_DECOMP.md and stopped at DEL-04-07. The main decomposition table contains identity, type, scope and objective columns; accepted full descriptions reside in Deliverables.csv. Corrected metadata parity verifies each field against its actual source without changing any canonical or candidate bytes.'},'limitations':'Structural minimum-fileset and accepted metadata parity only; not product or semantic readiness.'}
(RUN/'application/carriers/CONTROL_VALIDATION.json').write_text(json.dumps(report,indent=2)+'\n');print(report['status'],'4/4 actual control sets and source metadata parity')
raise SystemExit(0 if report['status']=='PASS' else 1)
