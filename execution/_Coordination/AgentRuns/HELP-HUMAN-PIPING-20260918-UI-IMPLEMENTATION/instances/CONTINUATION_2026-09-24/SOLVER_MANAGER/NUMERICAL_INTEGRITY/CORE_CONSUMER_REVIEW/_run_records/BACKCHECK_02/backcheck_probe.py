"""Source-only binding probe. Injected deterministic JSON hash is NOT production JCS.
It holds checksum generation and validation consistent while isolating relational
validation. No subprocess, build, test suite, or native code executes."""
import ast
from copy import deepcopy
import hashlib
import json
from pathlib import Path
import sys

ROOT=Path(__file__).resolve()
while not (ROOT/'projects/chirality-piping').is_dir():
    ROOT=ROOT.parent
P=ROOT/'projects/chirality-piping'
sys.path.insert(0,str(P))
from core.serialization.canonical_json import adapter
encode=lambda x: json.dumps(x,sort_keys=True,separators=(',',':'),allow_nan=False)
digest=lambda x: hashlib.sha256(encode(x).encode()).hexdigest()
adapter.canonical_json_checked_v1=encode
adapter.canonical_sha256_checked_v1=digest
from core.analysis_runs import compatibility as a
FROZEN=Path(__file__).resolve().parent/'frozen'
# Compile preserved exact-review bytes into module namespaces. Original __file__
# retains repository semantic-table paths; no production files are modified.
exec(compile((FROZEN/'compatibility.py').read_text(),a.__file__,'exec'),a.__dict__)
from core.handoff.stress_neutral import package_v0_3 as s
exec(compile((FROZEN/'package_v0_3.py').read_text(),s.__file__,'exec'),s.__dict__)
from core.handoff.stress_neutral.package import render_stress_neutral_csv
scope={'deepcopy':deepcopy,'PRECISION_CONTRACT_ID':a.PRECISION_CONTRACT_ID,
       'BASIS':{'ref_type':'load_case','ref_id':'load:actual'}}
for path,names in [
    (P/'tests/test_precision_consumer_contract.py',['source']),
    (P/'tests/test_stress_neutral_export_package.py',['ref','source_payload'])
]:
    tree=ast.parse(path.read_text())
    exec(compile(ast.Module(body=[node for node in tree.body if isinstance(node,ast.FunctionDef) and node.name in names],type_ignores=[]),str(path),'exec'),scope)
raw=scope['source']()
raw['results']=[{'id':f'result:{i}','kind':'displacement_magnitude','value':x,'unit':'mm','entity_ref':'node:actual','basis_ref':scope['BASIS']} for i,x in enumerate([1.2345678901234567e-7,-1.0000000000000002])]
record=a.build_analysis_run_v0_3(raw,input_manifest_ref={'object_type':'InputManifest','ref':'manifest:test'},input_manifest_hash='1'*64)
args=scope['source_payload']()
args.update(source_result_ref={'object_type':'ResultEnvelope','ref':f"result-envelope:{raw['run_id']}"},source_run_ref={'object_type':'AnalysisRun','ref':raw['run_id']},source_model_ref={'object_type':'Model','ref':raw['model_ref']},source_hashes=deepcopy(record['analysis_run']['hashes']))
args['reproducibility_refs']=[args['source_run_ref']]
for row,original,mapping in zip(args['result_rows'],raw['results'],args['stable_id_map']):
    row.update(result_id=original['id'],canonical_ref={'object_type':'Result','ref':original['id']},source_result_ref={'object_type':'Result','ref':original['id']},value=original['value'],unit='mm',dimension='length',result_family='displacement',load_case_ref={'object_type':'LoadCase','ref':'load:actual'},component_ref={'object_type':'Node','ref':'node:actual'})
    row.update(s.source_row_projection_v0_3(raw,original))
    mapping.update(canonical_ref=row['canonical_ref'],export_ref={'object_type':'StressNeutralRow','ref':original['id']})
base=s.build_stress_neutral_export_package_v0_3(source_envelope=raw,analysis_record=record,**args)
results=[]
def attempt(name,call):
    try:
        call(); results.append({'probe':name,'result':'ACCEPTED'})
    except Exception as exc:
        results.append({'probe':name,'result':'REJECTED','exception':type(exc).__name__,'message':str(exc)})
def rehash_packet(packet):
    packet['csv_text']=render_stress_neutral_csv(packet['result_rows'])
    members=s.materialized_members_v0_3(packet)
    checksums=packet['manifest']['checksums']
    non_manifest=[c for c in checksums if c['payload_ref']['ref']!='manifest.json']
    for claim in non_manifest:
        claim['value']=hashlib.sha256(members[claim['payload_ref']['ref']]).hexdigest()
    seed=s._checked(s._manifest_seed(packet,non_manifest),'manifest.json','manifest_seed')
    checksums[:]=non_manifest+[seed]
    packet['manifest']['package_members']=[{'filename':name,'checksum':deepcopy(next(c for c in checksums if c['payload_ref']['ref']==name))} for name in s.MEMBERS]
    packet['package_checksum']['value']=digest(s.package_projection(packet))
def check_packet(packet):
    s.validate_stress_neutral_export_package_v0_3(packet,source_envelope=raw,analysis_record=record)
attempt('baseline_packet_with_supplied_source_and_analysis',lambda:check_packet(base))
for field,value in [
    ('load_case_ref',{'object_type':'LoadCase','ref':'load:unrelated'}),
    ('component_ref',{'object_type':'Node','ref':'node:unrelated'}),
    ('result_family','other'),
]:
    changed=deepcopy(args);changed['result_rows'][0][field]=value
    attempt('builder_wrong_'+field,lambda changed=changed:s.build_stress_neutral_export_package_v0_3(source_envelope=raw,analysis_record=record,**changed))
for name,change in [
    ('source_model',lambda p:p['source_model_ref'].update(ref='model:unrelated')),
    ('source_result_ref',lambda p:(p['source_result_ref'].update(ref='result-envelope:unrelated'),p['source_carrier_checksum']['payload_ref'].update(ref='result-envelope:unrelated'))),
    ('formulation_metadata_negative_control',lambda p:p['formulation_basis'].update(limitations=['different'])),
]:
    altered=deepcopy(base);change(altered);rehash_packet(altered)
    attempt('rehashed_packet_'+name,lambda altered=altered:check_packet(altered))
def rehash_analysis(record):
    record['analysis_run']['hashes'][0]['value']=digest(a.analysis_record_projection(record))
for name,change in [
    ('load_basis_refs',lambda run:run.update(load_basis_refs=[{'object_type':'LoadCase','ref':'load:unrelated'}])),
    ('mechanics_status',lambda run:run.update(analysis_status=['MODEL_INCOMPLETE','HUMAN_REVIEW_REQUIRED'])),
    ('diagnostics',lambda run:run.update(diagnostics=[{'source_annotation':{'id':'forged','code':'FORGED'}}])),
    ('model_negative_control',lambda run:run['model_state_ref'].update(ref='state:unrelated:preview')),
    ('row_metadata_negative_control',lambda run:run['result_refs'][0]['source_annotation'].update(metadata={'component':'bad'})),
]:
    changed=deepcopy(record);change(changed['analysis_run']);rehash_analysis(changed)
    attempt('rehashed_analysis_'+name,lambda changed=changed:a.validate_analysis_run_v0_3(changed,raw))
altered=deepcopy(base)
old_ref=deepcopy(altered['source_result_ref'])
altered['source_result_ref']['ref']='result-envelope:unrelated'
altered['source_carrier_checksum']['payload_ref']['ref']='result-envelope:unrelated'
for claim in altered['received_source_checksums']:
    if claim['payload_scope']=='received_result':
        claim['payload_ref']['ref']='result-envelope:unrelated'
rehash_packet(altered)
attempt('rehashed_packet_source_result_ref_all_internal_links_updated',lambda:check_packet(altered))
emitted=s.materialized_members_v0_3(base)
probe_keys=['producer','numerical_quality','formulation_basis','semantic_contract_ref','source_carrier_checksum','package_checksum']
results.append({'probe':'materialized_precision_contract_fields','fields_found_in_member_json':{
    k:[name for name,b in emitted.items() if name.endswith('.json') and isinstance(json.loads(b),dict) and k in json.loads(b)]
    for k in probe_keys}})

reconstructed=s.reconstruct_materialized_members_v0_3(emitted,source_envelope=raw,analysis_record=record)
results.append({'probe':'transport_exact_reconstruction','exact':reconstructed==base,'nine_member_count':len(emitted),
 'precision_fields_equal':{key:reconstructed[key]==base[key] for key in probe_keys}})
for field in s.source_row_projection_v0_3(raw,raw['results'][0]):
    changed=deepcopy(args);del changed['result_rows'][0][field]
    attempt('missing_projected_'+field,lambda changed=changed:s.build_stress_neutral_export_package_v0_3(source_envelope=raw,analysis_record=record,**changed))
for rule in sorted(a.RULE_STATUSES):
    attempt('finite_rule_'+rule,lambda rule=rule:a.build_analysis_run_v0_3(raw,input_manifest_ref={'object_type':'InputManifest','ref':'m'},input_manifest_hash='1'*64,rule_check_status=rule))
witnessed=[{'object_type':'LoadCase','ref':'load:actual'},{'object_type':'LoadCase','ref':'additional:actual-model-case'}]
wider=a.build_analysis_run_v0_3(raw,input_manifest_ref={'object_type':'InputManifest','ref':'m'},input_manifest_hash='1'*64,expected_basis_refs=witnessed)
attempt('wider_basis_with_same_independent_witness',lambda:a.validate_analysis_run_v0_3(wider,raw,expected_basis_refs=witnessed))
attempt('wider_basis_with_no_independent_witness',lambda:a.validate_analysis_run_v0_3(wider,raw))
for bad_scope in [[],witnessed[1:],witnessed+witnessed[:1]]:
    attempt('invalid_independent_basis_'+str(bad_scope),lambda bad_scope=bad_scope:a.build_analysis_run_v0_3(raw,input_manifest_ref={'object_type':'InputManifest','ref':'m'},input_manifest_hash='1'*64,expected_basis_refs=bad_scope))
# Test exact nine-member inverse and rejected ambiguity independently of native I/O/JCS.
for attack in ['duplicate_nested','bad_version','metadata_collision','missing_member','extra_member','source_metadata_tamper']:
    altered_members=deepcopy(emitted)
    if attack=='duplicate_nested':
        altered_members['manifest.json']=altered_members['manifest.json'].replace(b'"component_version":"0.2.0"',b'"component_version":"0.2.0","component_version":"0.2.0"',1)
    elif attack=='missing_member':del altered_members['result_rows.json']
    elif attack=='extra_member':altered_members['extra.json']=b'{}'
    else:
        carrier=json.loads(altered_members['manifest.json'])
        if attack=='bad_version':carrier['transport_version']='0.2.0'
        elif attack=='metadata_collision':carrier['package_metadata']['result_rows']=[]
        else:carrier['package_metadata']['formulation_basis']['limitations']=['modified']
        altered_members['manifest.json']=encode(carrier).encode()
    attempt('transport_'+attack,lambda altered_members=altered_members:s.reconstruct_materialized_members_v0_3(altered_members,source_envelope=raw,analysis_record=record))


for location in ['',7,[],{},False,None,'end_i']:
    def full_location_probe(location=location):
        changed_source=deepcopy(raw)
        changed_source['results'][0]['metadata']={'location':location}
        before=deepcopy(changed_source)
        changed_record=a.build_analysis_run_v0_3(changed_source,input_manifest_ref={'object_type':'InputManifest','ref':'m'},input_manifest_hash='1'*64)
        changed_args=deepcopy(args)
        changed_args['source_hashes']=deepcopy(changed_record['analysis_run']['hashes'])
        # Use caller supplied invalid/valid station to exercise complete builder,
        # rather than have the standalone projection reject first.
        changed_args['result_rows'][0]['station_ref']={'object_type':'Station','ref':'summary' if location is None else location}
        try:
            output=s.build_stress_neutral_export_package_v0_3(source_envelope=changed_source,analysis_record=changed_record,**changed_args)
            assert isinstance(output['result_rows'][0]['station_ref']['ref'],str) and output['result_rows'][0]['station_ref']['ref']
        finally:
            assert changed_source==before
    attempt('complete_builder_location_'+repr(location),full_location_probe)
for bad_basis in [{},{'ref_type':'load_case','ref_id':''},{'ref_type':'load_case','ref_id':7},{'ref_type':'','ref_id':'x'},[],{'ref_type':'load_case','ref_id':'x','extra':True},None,{'ref_type':'custom','ref_id':'generic:scope'}]:
    changed_source=deepcopy(raw);changed_source['results'][0]['basis_ref']=bad_basis
    attempt('default_source_basis_'+repr(bad_basis),lambda changed_source=changed_source:a.build_analysis_run_v0_3(changed_source,input_manifest_ref={'object_type':'InputManifest','ref':'m'},input_manifest_hash='1'*64))
for field,value in [('run_id',''),('run_id',7),('model_ref',[]),('model_ref',''),('result_id',''),('result_id',7)]:
    changed_source=deepcopy(raw)
    if field=='result_id':changed_source['results'][0]['id']=value
    else:changed_source[field]=value
    attempt('invalid_identity_'+field+'_'+repr(value),lambda changed_source=changed_source:a.build_analysis_run_v0_3(changed_source,input_manifest_ref={'object_type':'InputManifest','ref':'m'},input_manifest_hash='1'*64))
# Execute the exact maintained fixture helper, not a duplicated repair.
tree=ast.parse((FROZEN/'test_stress_neutral_precision.py').read_text())
helpers={'deepcopy':deepcopy,'canonical_sha256_checked_v1':digest,'MEMBERS':s.MEMBERS,'package_projection':s.package_projection}
exec(compile(ast.Module(body=[x for x in tree.body if isinstance(x,ast.FunctionDef) and x.name=='rehash_packet'],type_ignores=[]),'frozen/test_stress_neutral_precision.py','exec'),helpers)
bad=deepcopy(base);bad['result_rows'][0]['station_ref']=None
helpers['rehash_packet'](bad)
results.append({'probe':'exact_maintained_null_reference_rehash_helper','completed_without_exception':True,'invalid_json_ref_retained':bad['result_rows'][0]['station_ref'] is None})
attempt('rehashed_null_reference_reaches_source_validator',lambda:s.validate_stress_neutral_export_package_v0_3(bad,source_envelope=raw,analysis_record=record))
print(json.dumps({'hash_seam':'injected JSON hash; exact frozen modules and test helper; NOT production JCS/native/file-I/O proof','probes':results},indent=2))
