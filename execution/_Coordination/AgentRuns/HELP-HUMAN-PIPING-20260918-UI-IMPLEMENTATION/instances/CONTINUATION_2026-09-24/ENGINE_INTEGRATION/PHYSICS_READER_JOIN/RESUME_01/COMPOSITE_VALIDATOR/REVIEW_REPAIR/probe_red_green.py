"""Read supplied actual unit records and an isolated reviewer torsion negative."""
from pathlib import Path
from fractions import Fraction
import hashlib,json,sys
HERE=Path(__file__).resolve().parent
PROJECT=next(p for p in HERE.parents if (p/'core/analysis_runs/physics_source.py').is_file())
sys.path.insert(0,str(PROJECT))
from core.analysis_runs.physics_source import validate_physics_source,validate_stress
fixtures=PROJECT/'fixtures/product_preview/physics_source'
out={'scope':'CR01 actual full producer outputs; CR03 isolated synthetic recipe only, never full source/current proof','reader_sha256':hashlib.sha256((PROJECT/'core/analysis_runs/physics_source.py').read_bytes()).hexdigest(),'units':[]}
for name in ['n05_units','mixed_units']:
 for mode in ['sparse_interactive','dense_scrutiny']:
  s=json.loads((fixtures/f'{name}-{mode}.raw.json').read_text());i={'request':json.loads((fixtures/f'{name}.request.json').read_text()),'solver_mode':mode}
  try:result=validate_physics_source(s,i)
  except Exception as error:result=f'{type(error).__name__}: {error}'
  out['units'].append({'name':name,'mode':mode,'outcome':result})
source=json.loads((fixtures/'n05-sparse_interactive.raw.json').read_text())
case=source['source_block_recovery']['body']['cases'][0]
check=next(c for c in case['section_stress_checks'] if c['component']=='torsional_shear_stress')
row=next(r for r in source['results'] if r['id']==check['result_id'])
physical=next(c for c in source['contract_evidence']['exact_cases'] if c['load_case_id']==case['basis_ref']['ref_id'])
section=next(s for s in physical['pipe_sections'] if s['pipe_id']==row['entity_ref'])
action,radius,constant=1e-280,1e-40,1e-160
member=next(m for m in case['source']['section_functionals'] if m['pipe_id']==row['entity_ref'])
station=member['stations'][['end_i','quarter_1','midspan','quarter_3','end_j'].index(row['metadata']['location'])]
station['actions'][3]={'value':action,'interval':[action,action]};check['action']=station['actions'][3]
section.update(ro_m=radius,J_m4=constant);check['parameters'].update(torsion_radius_m=radius,torsion_constant_m4=constant)
value=action*radius/constant/1e6;row['value']=value
reference=Fraction.from_float(action)*Fraction.from_float(radius)/Fraction.from_float(constant)/1000000
try:validate_stress(source,case,row,[]);result='accepted'
except Exception as error:result=f'{type(error).__name__}: {error}'
out['torsion']={'action':action,'radius':radius,'torsion_constant':constant,'intermediate':action*radius,'published_mpa':value,'relative_error_against_exact_operands':float(abs(Fraction.from_float(value)-reference)/abs(reference)),'outcome':result}
(HERE/sys.argv[1]).write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
