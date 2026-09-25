"""Independent bounded repair controls; retained source, no new solver invocation."""
from pathlib import Path
from copy import deepcopy
import json,sys,struct,hashlib
HERE=Path(__file__).resolve().parent
PROJECT=next(p for p in HERE.parents if (p/'core/analysis_runs/physics_source.py').is_file())
sys.path[:0]=[str(PROJECT),str(PROJECT/'tests')]
import test_stress_neutral_physics_source as transport
import test_physics_source_contract as consumer
from core.handoff.stress_neutral import package_v0_3 as sn
bits=lambda n:struct.pack('>d',float(n)).hex()
out={'scope':'Existing actual retained source -> nine-member readback; isolated reader negatives, no new solve/native qualification','transport':[]}
for mode in ['sparse_interactive','dense_scrutiny']:
    raw,analysis,package=transport.captured(PROJECT/f'fixtures/product_preview/physics_source/mixed-{mode}.raw.json')
    original={r['id']:bits(r['value']) for r in raw['results']}
    members=sn.materialized_members_v0_3(package)
    reopened=sn.reconstruct_materialized_members_v0_3(members,source_envelope=raw,analysis_record=analysis)
    assert len(members)==9
    assert all(a['source_value_bits']==original[a['source_result_id']] for a in reopened['source_annotations'])
    zeros=[a for a in reopened['source_annotations'] if a['source_value_bits']=='8000000000000000']
    assert len(zeros)==31 if mode=='sparse_interactive' else bool(zeros)
    assert all(bits(a['source_row']['value'])=='0000000000000000' for a in zeros)
    forged=deepcopy(reopened)
    next(a for a in forged['source_annotations'] if a['source_value_bits']=='8000000000000000')['source_value_bits']='0000000000000000'
    unsealed_reject=False
    try:sn.validate_stress_neutral_export_package_v0_3(forged)
    except ValueError:unsealed_reject=True
    assert unsealed_reject
    transport.rehash_packet(forged)
    sn.validate_stress_neutral_export_package_v0_3(forged)
    bound_reject=False
    try:sn.validate_stress_neutral_export_package_v0_3(forged,source_envelope=raw,analysis_record=analysis)
    except ValueError:bound_reject=True
    assert bound_reject
    transport.validate_instance(json.loads(transport.SCHEMA.read_text()),reopened)
    assert {r['id']:bits(r['value']) for r in raw['results']}==original
    out['transport'].append({'mode':mode,'negative_zero_population':len(zeros),'all_received_row_value_bits_retained':True,'sidecar_unhashed_change_refused':True,'coherently_rehashed_sidecar_requires_original_binding':True,'schema_accepted':True,'member_hashes':{k:hashlib.sha256(v).hexdigest() for k,v in members.items()}})
transport.test_precision_package_and_all_nine_member_bytes_are_unchanged()
out['p1_packet_and_nine_member_goldens']='pass'
for field,unit in [('elastic_modulus','N'),('elastic_modulus','gpa'),('shear_modulus','K'),('poisson_ratio','none'),('thermal_expansion_coefficient','K')]:
    consumer.test_actual_units_authority_rejects_wrong_dimension_and_unregistered_spelling(field,unit)
for action in [0.0,-0.0,1e-200,-1e-200]:
    consumer.test_torsion_recipe_keeps_structural_zero_and_normal_intermediates(action)
consumer.test_material_selection_normalizes_named_points_and_strict_temperature_brackets()
consumer.test_selected_alpha_uses_same_normalized_material_basis()
out['selected_consequence_checks']={'invalid_dimension_or_spelling':5,'torsion_zero_or_normal':4,'named_point_strict_bracket':1,'normalized_alpha':1}
(HERE/'CR02_AND_CONSEQUENCES.json').write_text(json.dumps(out,indent=2)+'\n')
print(json.dumps({k:([{x:y for x,y in q.items() if x!='member_hashes'} for q in v] if k=='transport' else v) for k,v in out.items()},indent=2))

