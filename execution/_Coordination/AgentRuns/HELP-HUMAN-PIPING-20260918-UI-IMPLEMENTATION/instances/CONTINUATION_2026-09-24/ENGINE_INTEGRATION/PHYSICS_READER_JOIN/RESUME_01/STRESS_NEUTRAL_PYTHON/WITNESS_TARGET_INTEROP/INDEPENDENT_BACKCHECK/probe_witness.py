"""Independent new-method witness checks; retained source, no TS or new solve."""
from pathlib import Path
from copy import deepcopy
import sys,json,struct,hashlib
HERE=Path(__file__).resolve().parent
PROJECT=next(p for p in HERE.parents if (p/'core/handoff/stress_neutral/package_v0_3.py').is_file())
sys.path[:0]=[str(PROJECT),str(PROJECT/'tests')]
import test_stress_neutral_physics_source as f
from core.handoff.stress_neutral import package_v0_3 as sn
bits=lambda v:struct.pack('>d',float(v)).hex()
source,analysis,packet=f.captured(f.COMPOSITE_DIR/'fields-dense_scrutiny.raw.json')
rid='result:sparse-live:dense-parity-relative-delta'
original=next(r for r in source['results'] if r['id']==rid)
row=next(r for r in packet['result_rows'] if r['result_id']==rid)
_,_,table=f._source_contract(source)
signature,_=sn._semantic(original,table)
assert signature['signature_id']=='supported-source-037' and signature['category']=='diagnostic_relative_ratio'
assert signature['derivative_target_dimension']=='ratio' and signature['source_physical_semantic_dimension']=='dimensionless'
assert not signature['governing_ratio_eligible']
assert sn._witness_disposition(row)[0]=='contradiction'
assert sn._source_witness_disposition(row,original,table)==('eligible','ratio')
w=next(w for w in packet['unit_preservation_witnesses'] if w['result_id']==rid)
assert w['source_quantity']==w['target_quantity']=={'value':original['value'],'unit':'unitless','dimension':'ratio'}
sn.validate_stress_neutral_export_package_v0_3(packet)
sn.validate_stress_neutral_export_package_v0_3(packet,source_envelope=source,analysis_record=analysis)
members=sn.materialized_members_v0_3(packet)
reopened=sn.reconstruct_materialized_members_v0_3(members,source_envelope=source,analysis_record=analysis)
byid={r['id']:r for r in source['results']}
assert all(a['source_value_bits']==bits(byid[a['source_result_id']]['value']) for a in reopened['source_annotations'])
refusals=[]
for name in ['witness_dimension','source_row_unit','source_row_kind','source_bits','semantic_hash','false_diagnostic_work']:
    bad=deepcopy(packet)
    witness=next(w for w in bad['unit_preservation_witnesses'] if w['result_id']==rid)
    annotation=next(a for a in bad['source_annotations'] if a['source_result_id']==rid)
    if name=='witness_dimension':witness['source_quantity']['dimension']='dimensionless'
    elif name in ['source_row_unit','source_row_kind']:
        annotation['source_row']['unit' if name.endswith('unit') else 'kind']='Pa' if name.endswith('unit') else 'invented_future_kind'
        annotation['source_row_sha256']=sn.canonical_sha256_checked_v1(annotation['source_row'])
    elif name=='source_bits':annotation['source_value_bits']='7ff0000000000000'
    elif name=='semantic_hash':bad['semantic_contract']['sha256']='0'*64
    else:
        bad['unit_preservation_witnesses'].remove(witness)
        bad['diagnostics'].append({'code':sn.WITHHOLDING_CODES['diagnostic_work'],'class':'unit_preservation_witness','severity':'info','source':{'object_type':'StressNeutralResultRow','ref':rid},'affected_object':{'object_type':'StressNeutralUnitWitness','ref':witness['witness_id']},'message':'Synthetic false category for refusal probe.','remediation':'Retain admitted table category.','provenance':deepcopy(bad['provenance'])})
    f.rehash_packet(bad)
    outcomes={}
    for mode,kwargs in [('standalone',{}),('supplied_source',{'source_envelope':source,'analysis_record':analysis})]:
        try:sn.validate_stress_neutral_export_package_v0_3(bad,**kwargs)
        except ValueError as error:outcomes[mode]=str(error)
        else:raise AssertionError((name,mode,'unexpected acceptance'))
    refusals.append({'case':name,**outcomes})
f.test_precision_package_and_all_nine_member_bytes_are_unchanged()
out={'scope':'Actual retained fields/dense source built by Python and reopened; coherent false-copy negatives, no TS14pair/new solve/native claim','signature':signature,'generic_previous_disposition':sn._witness_disposition(row),'new_disposition':sn._source_witness_disposition(row,original,table),'source_and_standalone_and_nine_member_readback':'pass','all_value_bits_retained':True,'p1_packet_and_nine_member_goldens':'pass','coherent_negative_refusals':refusals,'member_hashes':{k:hashlib.sha256(v).hexdigest() for k,v in members.items()}}
(HERE/'WITNESS_PROBES.json').write_text(json.dumps(out,indent=2)+'\n')
print(json.dumps({k:v for k,v in out.items() if k not in ['member_hashes','signature']},indent=2))

