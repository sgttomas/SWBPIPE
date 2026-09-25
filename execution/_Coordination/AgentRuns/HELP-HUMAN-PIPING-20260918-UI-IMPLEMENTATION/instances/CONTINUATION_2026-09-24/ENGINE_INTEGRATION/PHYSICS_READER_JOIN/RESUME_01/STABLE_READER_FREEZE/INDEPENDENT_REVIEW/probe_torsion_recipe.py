"""Isolated current consumer recipe test; deliberately not a producer packet."""
from pathlib import Path
from fractions import Fraction
import hashlib,json,sys,math
HERE=Path(__file__).resolve().parent
PROJECT=next(p for p in HERE.parents if (p/'core/analysis_runs/physics_source.py').is_file())
sys.path.insert(0,str(PROJECT))
from core.analysis_runs.physics_source import validate_stress
source=json.loads((PROJECT/'fixtures/product_preview/physics_source/n05-sparse_interactive.raw.json').read_text())
case=source['source_block_recovery']['body']['cases'][0]
check=next(c for c in case['section_stress_checks'] if c['component']=='torsional_shear_stress')
row=next(r for r in source['results'] if r['id']==check['result_id'])
physical=next(c for c in source['contract_evidence']['exact_cases'] if c['load_case_id']==case['basis_ref']['ref_id'])
section=next(s for s in physical['pipe_sections'] if s['pipe_id']==row['entity_ref'])
action=1e-280;radius=1e-40;constant=1e-160
member=next(m for m in case['source']['section_functionals'] if m['pipe_id']==row['entity_ref'])
locations=['end_i','quarter_1','midspan','quarter_3','end_j'];station=member['stations'][locations.index(row['metadata']['location'])]
station['actions'][3]={'value':action,'interval':[action,action]};check['action']=station['actions'][3]
section.update(ro_m=radius,J_m4=constant)
check['parameters'].update(torsion_radius_m=radius,torsion_constant_m4=constant)
value=action*radius/constant/1e6;row['value']=value
reference=Fraction.from_float(action)*Fraction.from_float(radius)/Fraction.from_float(constant)/1000000
error=float(abs(Fraction.from_float(value)-reference)/abs(reference))
validate_stress(source,case,row,[])
assert 0<abs(action*radius)<sys.float_info.min and abs(value)>=sys.float_info.min and error>1e-9
out={'scope':'Only isolated existing Python consumer scalar recipe, not complete/resealed source admission, actual solve or native proof',
     'reader_sha256':hashlib.sha256((PROJECT/'core/analysis_runs/physics_source.py').read_bytes()).hexdigest(),
     'action':action,'radius':radius,'torsion_constant':constant,'intermediate':action*radius,'published_mpa':value,
     'relative_error_against_exact_represented_operands':error,'reader_recipe_outcome':'accepted',
     'producer_guard':'source_receipt/rows.rs rejects nonzero action*radius unless normal'}
(HERE/'TORSION_RECIPE_PROBE.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
