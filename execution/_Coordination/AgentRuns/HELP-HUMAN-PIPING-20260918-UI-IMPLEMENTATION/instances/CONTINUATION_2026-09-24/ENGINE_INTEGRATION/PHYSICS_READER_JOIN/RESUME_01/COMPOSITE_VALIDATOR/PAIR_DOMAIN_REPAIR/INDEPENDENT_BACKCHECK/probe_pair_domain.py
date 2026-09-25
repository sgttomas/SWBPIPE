"""Independent bounded CR01b domain/public-entry controls, no producer execution."""
from pathlib import Path
import sys,json,math,struct
HERE=Path(__file__).resolve().parent
PROJECT=next(p for p in HERE.parents if (p/'core/analysis_runs/physics_source.py').is_file())
sys.path[:0]=[str(PROJECT),str(PROJECT/'tests')]
import test_physics_source_contract as t
from core.analysis_runs.physics_source import _checked_material_pair, validate_physics_source
minimum=math.ulp(0.0);maximum=sys.float_info.max
cases=[
    ('original_underflow',minimum,.25,None),
    ('publication_tie',minimum,0.0,None),
    ('denominator_rounding_tie',minimum,-2**-54,None),
    ('adjacent_survives',minimum,math.nextafter(-2**-54,-math.inf),minimum),
    ('positive_subnormal',2*minimum,.25,minimum),
    ('identity_subnormal',minimum,-.5,minimum),
    ('top_identity',maximum,-.5,maximum),
    ('top_adjacent_overflow',maximum,math.nextafter(-.5,-math.inf),None),
    ('top_adjacent_rounds_identity',maximum,math.nextafter(-.5,math.inf),maximum),
    ('top_half',maximum,0.0,maximum*.5),
    ('overflow_exponent',maximum,math.nextafter(-1.,math.inf),None),
    ('ordinary_near_nu_limit',1.,math.nextafter(-1.,math.inf),2**52),
    ('smallest_near_upper_nu',minimum,math.nextafter(.5,-math.inf),None),
]
out={'scope':'Scalar domain and existing genuine input/received-source consistency probes; no new solver/native proof','boundaries':[]}
for name,e,nu,expected in cases:
    try:actual=_checked_material_pair(e,nu,'independent_boundary')
    except ValueError:actual=None
    assert (actual is None)==(expected is None),name
    if actual is not None:assert struct.pack('>d',actual)==struct.pack('>d',expected),name
    out['boundaries'].append({'name':name,'admitted':actual is not None,'output_bits':None if actual is None else struct.pack('>d',actual).hex()})
for stage in ['base','lower_bracket','upper_bracket','selected']:
    t.test_every_consumed_material_pair_checks_source_domain(stage)
out['consumed_stage_refusals']=['base','lower_bracket','upper_bracket','selected']
for endpoint in [0,1]:t.test_public_rehashed_copy_cannot_hide_invalid_bracket_pair_behind_finite_selection(endpoint)
out['public_false_copy_refusals']=['lower','upper']
out['unchanged_actual_positives']=[]
for name in ['n05_units','mixed_units']:
    for mode in ['sparse_interactive','dense_scrutiny']:
        source,invocation=t.actual(name,mode)
        assert validate_physics_source(source,invocation) is True
        assert validate_physics_source(source) is False
        out['unchanged_actual_positives'].append(name+'/'+mode)
(HERE/'INDEPENDENT_DOMAIN_CONTROLS.json').write_text(json.dumps(out,indent=2)+'\n')
print(json.dumps(out,indent=2))

