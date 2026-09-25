"""Read-only lightweight replay of recorded product numerical standing and analytical comparison."""
from pathlib import Path
import sys, json, hashlib, re, platform
from decimal import Decimal as D, localcontext
sys.dont_write_bytecode = True
ROOT=Path.cwd()
P=ROOT/'projects/chirality-piping'
HERE=Path(__file__).resolve().parent
OBS=HERE.parent
sys.path.insert(0,str(P))
if "--frozen" in sys.argv:
 namespace={"__name__":"n05_frozen_compatibility","__file__":str(P/'core/analysis_runs/compatibility.py')}
 exec(compile((HERE/'compatibility.py.observed-source').read_text(),str(HERE/'compatibility.py.observed-source'),'exec'),namespace)
 numerical_use_standing=namespace['numerical_use_standing']
else:
 from core.analysis_runs.compatibility import numerical_use_standing
rows=json.loads((OBS/'_run_records/SELECTED.json').read_text())
reference=json.loads((P/'validation/benchmarks/numerical_integrity/fixtures.json').read_text())
raw=[json.loads(x) for x in (OBS/'_run_records/stdout.jsonl').read_text().splitlines() if x.strip()]
results=[]
with localcontext() as ctx:
 ctx.prec=100
 for r in rows:
  if r['case'] not in ('N05','N06'): continue
  assert r in raw, 'Selected product record differs from raw observation'
  o=r['observed']; n=reference['N'][r['case']]
  d={'case':r['case'],'mode':r['mode'],'status':o['status'],'numerical_quality':o['numerical_quality'],'python_numerical_use_standing':numerical_use_standing(o,[{'ref_type':'load_case','ref_id':x['id']} for x in r['input']['load_cases']]),'comparisons':[]}
  for node in ('root','tip'):
   v=next((x for x in o['results'] if x['id']==f'result:disp:{node}:rx'),None)
   if v:
    observed=D.from_float(v['value']);expected=D(n[f'theta_{node}_rad']);rel=(observed-expected)/expected
    d['comparisons'].append({'quantity':node+'_RX','observed_json':v['value'],'observed_binary64_exact':str(observed),'reference':str(expected),'absolute_error':str(observed-expected),'relative_error':str(rel),'protected_relative_tolerance':'1e-9','passes':abs(rel)<=D('1e-9')})
  if d['comparisons']:
   root=D(d['comparisons'][0]['observed_binary64_exact'])
   action=-D(n['spring_RX_Nm_per_rad'])*root;expected=D(n['spring_action_Nm'])
   d['derived_physical_spring_action']={'value':str(action),'reference':str(expected),'relative_error':str((action-expected)/expected),'basis':'Independently evaluated physical spring action -k*actual published root RX; this is NOT an emitted product moment row.'}
  for diagnostic in o['diagnostics']:
   if 'NUMERICAL_INTEGRITY' in diagnostic['code']:
    d['structural_diagnostic']={'code':diagnostic['code'],'severity':diagnostic['severity'],'metrics':{k:re.search(r'\b'+k+r': ([^,}]+)',diagnostic['message']).group(1) for k in ('reciprocal_condition_estimate','assembly_relative_perturbation_estimate','assembly_amplification_estimate','refinement_attempts') if re.search(r'\b'+k+r': ([^,}]+)',diagnostic['message'])}}
  results.append(d)
 np=reference['NP']['A'][0]
 output={'method':'100-digit Decimal comparison of exact decoded emitted binary64 values to unchanged frozen independent source-intended N05/N06 references; actual Python numerical standing invoked on unmodified saved product records. No solve rerun.','python':sys.version,'results':results,'np_a_n05_reference':{k:np[k] for k in ('a_binary64_hex','k_intended','k_stored_exact','k_relative_loss','stored_exact_root','root_relative_physical_error')},'np_a_observations':[r for r in rows if r['case'].startswith('NP-A')],'np_a_scope':'Generic dense/sparse matrix APIs only; records have no actual product carrier; no product admission evaluated or fabricated for NP-A.'}
output_name = sys.argv[sys.argv.index('--output')+1] if '--output' in sys.argv else ('PYTHON_FROZEN_REPLAY.json' if '--frozen' in sys.argv else 'PYTHON_COMPARISON_ADMISSION.json')
assert Path(output_name).name == output_name
(HERE/output_name).write_text(json.dumps(output,indent=2)+'\n')
print(json.dumps({'results':[{k:r[k] for k in ('case','mode','python_numerical_use_standing','comparisons','structural_diagnostic')} for r in results]},indent=2))
