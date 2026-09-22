#!/usr/bin/env python3
"""Validate each draft and conditional Group-2 SoW without canonical writes."""
import json,subprocess,tempfile
from pathlib import Path
RUN=Path(__file__).resolve().parents[1]
ROOT=RUN
while not (ROOT/'tools/scope_of_work/validate_scope_of_work.py').exists(): ROOT=ROOT.parent
meta=json.loads((RUN/'evidence/SOURCES_DECOMPOSITION.json').read_text())
ops={x['target']:x for x in meta['application_metadata']['files']}
ops.update({x['target']:x for x in json.loads((RUN/'interfaces/INTERFACE_PROMOTION.json').read_text())['files']})
results=[]
with tempfile.TemporaryDirectory(prefix='sca011-sow-',dir=RUN/'evidence') as temporary:
 for p in sorted((RUN/'candidate').rglob('ScopeOfWork.md')):
  rel=str(p.relative_to(RUN/'candidate')); text=p.read_text()
  transformed=text
  for op in ops.get(rel,{}).get('operations',[]):
   assert transformed.count(op['old'])==op['expected_occurrences']
   transformed=transformed.replace(op['old'],op['new'])
  for phase,body in [('candidate',text),('group2_applied_preview',transformed)]:
   f=Path(temporary)/phase/Path(rel).parent.name/'ScopeOfWork.md'; f.parent.mkdir(parents=True,exist_ok=True);f.write_text(body)
   command=['python3',str(ROOT/'tools/scope_of_work/validate_scope_of_work.py'),str(f),'--json']
   result=subprocess.run(command,text=True,capture_output=True)
   results.append({'target':rel,'phase':phase,'tool':'python3 tools/scope_of_work/validate_scope_of_work.py <isolated ScopeOfWork.md> --json','exit_code':result.returncode,'report':json.loads(result.stdout) if result.stdout.strip().startswith('{') else result.stdout,'stderr':result.stderr})
report={'status':'PASS' if all(x['exit_code']==0 for x in results) else 'FAIL','candidate_count':len(results)//2,'validator_runs':len(results),'checks':results,'limit':'SoW format and local reference validity only; no implementation or semantic conformance claim'}
(RUN/'evidence/ALL_SOW_VALIDATION.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({k:v for k,v in report.items() if k!='checks'},indent=2))
raise SystemExit(0 if report['status']=='PASS' else 1)
