"""Rehashed false input/evidence consistency probe; no real producer or native invocation."""
from pathlib import Path
import json,hashlib,sys
HERE=Path(__file__).resolve().parent
PROJECT=next(p for p in HERE.parents if (p/'core/analysis_runs/physics_source.py').is_file())
sys.path[:0]=[str(PROJECT),str(PROJECT/'tests')]
import test_physics_source_contract as t
source,invocation=t.actual('mixed_units')
material=invocation['request']['model']['materials'][0]
material['temperature_points'][0]['elastic_modulus']={'value':5e-324,'unit':'Pa'}
material['temperature_points'][1]['elastic_modulus']={'value':4000.0,'unit':'Pa'}
t.reseal_negative(source,invocation)
canonical=t.composite._canonical_inputs(invocation)
try:
    selection=t.composite._selected_material(material,invocation['request']['model']['load_cases'][1],canonical)
except Exception as error:selection=f'{type(error).__name__}: {error}'
try:outcome=t.composite.validate_physics_source(source,invocation)
except Exception as error:outcome=f'{type(error).__name__}: {error}'
record={'scope':'Rehashed deliberately false retained source/invocation copy, never actual producer or Current proof',
'reader_sha256':hashlib.sha256((PROJECT/'core/analysis_runs/physics_source.py').read_bytes()).hexdigest(),
'selected':selection,'lower_E_pa':5e-324,'lower_nu':.25,'lower_derived_G_pa':5e-324/(2*(1+.25)),
'public_reader_outcome':outcome}
destination=HERE/(sys.argv[1] if len(sys.argv)>1 else 'INTERPOLATION_PAIR_RANGE_REPLAY.json')
destination.write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record,indent=2))

