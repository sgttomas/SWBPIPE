"""Connected precision carrier → analysis 0.3 → neutral 0.3 transport tests."""
from copy import deepcopy
import json
from pathlib import Path
import pytest
from core.analysis_runs.compatibility import build_analysis_run_v0_3, build_analysis_run_v0_2, validate_analysis_run_v0_3, verify_analysis_run_record
from core.handoff.stress_neutral.package_v0_3 import (
    build_stress_neutral_export_package_v0_3, validate_stress_neutral_export_package_v0_3,
    materialized_members_v0_3, package_projection, source_row_projection_v0_3,
    write_materialized_members_v0_3, read_materialized_members_v0_3, MEMBERS,
)
from core.handoff.stress_neutral.package_v0_2 import validate_stress_neutral_export_package_v0_2
from core.serialization.canonical_json.adapter import canonical_sha256_checked_v1
from test_precision_consumer_contract import source
from test_stress_neutral_export_package import source_payload
from schema_validation import validate_instance

PROJECT = Path(__file__).resolve().parents[1]

def prepare():
    raw=source();base=source_payload()
    raw['results']=[{'id':f'result:{index}', 'kind':'displacement_magnitude', 'value':value, 'unit':'mm', 'entity_ref':'node:transport'} for index,value in enumerate([1.2345678901234567e-7,-1.0000000000000002])]
    analysis=build_analysis_run_v0_3(raw,input_manifest_ref={'object_type':'InputManifest','ref':'manifest:transport'},input_manifest_hash='1'*64)
    base['source_result_ref']={'object_type':'ResultEnvelope','ref':f"result-envelope:{raw['run_id']}"}
    base['source_run_ref']={'object_type':'AnalysisRun','ref':raw['run_id']}
    base['source_model_ref']={'object_type':'Model','ref':raw['model_ref']}
    base['source_hashes']=deepcopy(analysis['analysis_run']['hashes'])
    base['reproducibility_refs']=[base['source_run_ref']]
    for row,original,mapping in zip(base['result_rows'],raw['results'],base['stable_id_map']):
        row.update(source_row_projection_v0_3(raw, original))
        mapping.update(canonical_ref=row['canonical_ref'],export_ref={'object_type':'StressNeutralRow','ref':original['id']})
    return raw,analysis,base

def test_new_source_analysis_and_neutral_share_exact_values_and_hashes(tmp_path):
    raw,analysis,args=prepare();packet=build_stress_neutral_export_package_v0_3(source_envelope=raw,analysis_record=analysis,**args)
    assert packet['schema_version']=='0.3.0'; assert packet['export_profile']['profile_id']=='ops.stress_neutral.v3'
    validate_instance(json.loads((PROJECT/'schemas/stress_neutral_export.schema.json').read_text()),packet,instance_label='precision neutral')
    for key in ['producer','numerical_quality','formulation_basis']:assert packet[key]==raw[key]
    assert [row['value'] for row in packet['result_rows']]==[row['value'] for row in raw['results']]
    reread=json.loads(json.dumps(packet));validate_stress_neutral_export_package_v0_3(reread,source_envelope=raw,analysis_record=analysis)
    assert packet['source_carrier_checksum']['value']==canonical_sha256_checked_v1(raw)
    for name,content in materialized_members_v0_3(packet).items():(tmp_path/name).write_bytes(content)
    written=write_materialized_members_v0_3(packet,tmp_path)
    assert [path.name for path in written]==MEMBERS
    reconstructed=read_materialized_members_v0_3(tmp_path,source_envelope=raw,analysis_record=analysis)
    assert reconstructed==packet
    assert canonical_sha256_checked_v1(package_projection(reconstructed))==packet['package_checksum']['value']
    validate_instance(json.loads((PROJECT/'schemas/stress_neutral_export.schema.json').read_text()),reconstructed,instance_label='reconstructed precision neutral')
    transport=json.loads((tmp_path/'manifest.json').read_text())
    assert transport['manifest']==packet['manifest']
    assert 'package_checksum' not in transport['manifest']
    for key in ['producer','numerical_quality','formulation_basis','semantic_contract_ref','source_carrier_checksum','package_checksum']:
        assert transport['package_metadata'][key]==packet[key]
    readrows=json.loads((tmp_path/'result_rows.json').read_text());assert [row['value'] for row in readrows]==[row['value'] for row in raw['results']]
    import csv, io
    assert [float(row['value']) for row in csv.DictReader(io.StringIO(packet['csv_text']))]==[row['value'] for row in raw['results']]
    with pytest.raises(ValueError,match='VERSION-UNSUPPORTED'):validate_stress_neutral_export_package_v0_2(packet)
    with pytest.raises(ValueError,match='DOWNGRADE'):build_analysis_run_v0_2(raw,input_manifest_ref={'object_type':'InputManifest','ref':'m'},input_manifest_hash='1'*64)

def test_rehashed_metadata_and_source_mismatch_are_not_authenticated():
    raw,analysis,args=prepare();packet=build_stress_neutral_export_package_v0_3(source_envelope=raw,analysis_record=analysis,**args)
    bad=deepcopy(packet);bad['formulation_basis']['limitations']=['forged but well-shaped limitation']
    bad['package_checksum']['value']=canonical_sha256_checked_v1(package_projection(bad))
    # Standalone checks only packet consistency; actual source binding is separate.
    validate_stress_neutral_export_package_v0_3(bad)
    with pytest.raises(ValueError,match='SOURCE-BINDING'):validate_stress_neutral_export_package_v0_3(bad,source_envelope=raw)
    wrong=deepcopy(raw);wrong['results'][0]['value']*=2
    with pytest.raises(ValueError,match='SOURCE'):validate_analysis_run_v0_3(analysis,wrong)
    old=deepcopy(raw)
    for key in ['producer','numerical_quality','formulation_basis']:del old[key]
    with pytest.raises(ValueError,match='CONTRACT_UNSUPPORTED'):build_analysis_run_v0_3(old,input_manifest_ref={'object_type':'InputManifest','ref':'m'},input_manifest_hash='1'*64)


def rehash_packet(packet):
    """Rehash every changed member/reference using the real checked JCS CLI."""
    import hashlib
    from core.handoff.stress_neutral import package_v0_3 as sn
    from core.handoff.stress_neutral.package import render_stress_neutral_csv
    packet['csv_text'] = render_stress_neutral_csv(packet['result_rows'])
    members = sn.materialized_members_v0_3(packet)
    checks = [item for item in packet['manifest']['checksums'] if item['payload_ref']['ref'] != 'manifest.json']
    for claim in checks:
        claim['value'] = hashlib.sha256(members[claim['payload_ref']['ref']]).hexdigest()
    checks.append(sn._checked(sn._manifest_seed(packet, checks), 'manifest.json', 'manifest_seed'))
    packet['manifest']['checksums'] = checks
    packet['manifest']['package_members'] = [{'filename': name, 'checksum': deepcopy(next(c for c in checks if c['payload_ref']['ref'] == name))} for name in MEMBERS]
    packet['package_checksum']['value'] = canonical_sha256_checked_v1(package_projection(packet))


@pytest.mark.parametrize('field,value', [
    ('load_case_ref', {'object_type':'LoadCase','ref':'load:unrelated'}),
    ('component_ref', {'object_type':'Node','ref':'node:unrelated'}),
    ('station_ref', {'object_type':'Station','ref':'end_j'}),
    ('result_family', 'other'), ('row_kind', 'diagnostic_work'),
    ('dimension', 'force'), ('correlation_status', 'unit_or_dimension_blocking_review_required'),
    ('source_result_ref', {'object_type':'Result','ref':'unrelated'}),
    ('canonical_ref', {'object_type':'Component','ref':'result:0'}),
    ('station_ref', None),
])
def test_full_row_projection_rejects_wrong_identity_even_after_rehash(field,value):
    raw,analysis,args=prepare()
    packet=build_stress_neutral_export_package_v0_3(source_envelope=raw,analysis_record=analysis,**args)
    wrong=deepcopy(args);wrong['result_rows'][0][field]=value
    with pytest.raises(ValueError,match='ROW-SOURCE'):
        build_stress_neutral_export_package_v0_3(source_envelope=raw,analysis_record=analysis,**wrong)
    packet['result_rows'][0][field]=value
    rehash_packet(packet)
    with pytest.raises(ValueError,match='ROW-SOURCE'):
        validate_stress_neutral_export_package_v0_3(packet,source_envelope=raw,analysis_record=analysis)


def test_projection_requires_presence_and_preserves_unknown_withholding():
    raw,analysis,args=prepare()
    for field in source_row_projection_v0_3(raw,raw['results'][0]):
        wrong=deepcopy(args);del wrong['result_rows'][0][field]
        with pytest.raises(ValueError,match='ROW-SOURCE'):
            build_stress_neutral_export_package_v0_3(source_envelope=raw,analysis_record=analysis,**wrong)
    raw['results'][0]['kind']='future_unknown_kind'
    raw['results'][0]['metadata']={'location':None}
    projected=source_row_projection_v0_3(raw,raw['results'][0])
    assert projected['result_family']=='other' and projected['dimension']=='TBD'
    assert projected['station_ref']=={'object_type':'Station','ref':'summary'}
    assert projected['correlation_status']=='unit_or_dimension_blocking_review_required'
    args['result_rows'][0].update(projected)
    analysis=build_analysis_run_v0_3(raw,input_manifest_ref={'object_type':'InputManifest','ref':'m'},input_manifest_hash='1'*64)
    args['source_hashes']=deepcopy(analysis['analysis_run']['hashes'])
    packet=build_stress_neutral_export_package_v0_3(source_envelope=raw,analysis_record=analysis,**args)
    assert not any(w['result_id']=='result:0' for w in packet['unit_preservation_witnesses'])


@pytest.mark.parametrize('field,object_type', [('source_model_ref','Model'),('source_run_ref','AnalysisRun'),('source_result_ref','ResultEnvelope')])
def test_consistently_rehashed_source_references_require_independent_source(field,object_type):
    raw,analysis,args=prepare();packet=build_stress_neutral_export_package_v0_3(source_envelope=raw,analysis_record=analysis,**args)
    old=deepcopy(packet[field]);new={'object_type':object_type,'ref':'unrelated'}
    def relabel(value):
        if isinstance(value,dict):
            if value==old: value.clear();value.update(new)
            else:
                for item in value.values(): relabel(item)
        elif isinstance(value,list):
            for item in value: relabel(item)
    relabel(packet);rehash_packet(packet)
    with pytest.raises(ValueError,match='SOURCE-BINDING'):
        validate_stress_neutral_export_package_v0_3(packet,source_envelope=raw,analysis_record=analysis)


def test_transport_tamper_and_metadata_collision_rejected(tmp_path):
    raw,analysis,args=prepare();packet=build_stress_neutral_export_package_v0_3(source_envelope=raw,analysis_record=analysis,**args)
    write_materialized_members_v0_3(packet,tmp_path)
    manifest=tmp_path/'manifest.json'; transport=json.loads(manifest.read_text())
    transport['package_metadata']['formulation_basis']['limitations']=['tamper']
    manifest.write_text(json.dumps(transport))
    with pytest.raises(ValueError,match='SOURCE-BINDING'):
        read_materialized_members_v0_3(tmp_path,source_envelope=raw)
    transport['package_metadata']['result_rows']=[];manifest.write_text(json.dumps(transport))
    with pytest.raises(ValueError,match='COLLISION'):read_materialized_members_v0_3(tmp_path)


def test_source_location_and_basis_null_projection_and_witness_identity():
    raw,analysis,args=prepare()
    for source_location,expected in [(None,'summary'),('end_i','end_i'),('','')]:
        row=deepcopy(raw['results'][0]);row['metadata']={'location':source_location}
        row['basis_ref']=None
        projected=source_row_projection_v0_3(raw,row)
        assert projected['station_ref']=={'object_type':'Station','ref':expected}
        assert projected['load_case_ref']=={'object_type':'AnalysisRun','ref':raw['run_id']}
    packet=build_stress_neutral_export_package_v0_3(source_envelope=raw,analysis_record=analysis,**args)
    packet['unit_preservation_witnesses'][1]['witness_id']=packet['unit_preservation_witnesses'][0]['witness_id']
    rehash_packet(packet)
    with pytest.raises(ValueError,match='WITNESS-BINDING'):
        validate_stress_neutral_export_package_v0_3(packet,source_envelope=raw)


def test_diagnostic_work_and_missing_component_remain_withheld():
    raw,_,args=prepare()
    raw['results'][0].update(kind='nonlinear_support_free_dof_work_residual',unit='N*m',metadata={'component':'free_dof_work_residual'})
    raw['results'][1].update(kind='nonlinear_support_free_dof_work_residual',unit='N*m')
    for row,original in zip(args['result_rows'],raw['results']):row.update(source_row_projection_v0_3(raw,original))
    analysis=build_analysis_run_v0_3(raw,input_manifest_ref={'object_type':'InputManifest','ref':'m'},input_manifest_hash='1'*64)
    args['source_hashes']=deepcopy(analysis['analysis_run']['hashes'])
    packet=build_stress_neutral_export_package_v0_3(source_envelope=raw,analysis_record=analysis,**args)
    assert packet['unit_preservation_witnesses']==[]
    findings={item['source']['ref']:item['code'] for item in packet['diagnostics'] if item['code'].startswith('SN-UNIT-WITNESS-WITHHELD')}
    assert findings['result:0']=='SN-UNIT-WITNESS-WITHHELD-DIAGNOSTIC-WORK'
    assert findings['result:1']=='SN-UNIT-WITNESS-WITHHELD-UNKNOWN-SEMANTIC'


@pytest.mark.parametrize('change', ['missing','extra','duplicate_key','version','manifest_shape'])
def test_transport_rejects_ambiguous_shape_and_file_inventory(tmp_path,change):
    raw,analysis,args=prepare();packet=build_stress_neutral_export_package_v0_3(source_envelope=raw,analysis_record=analysis,**args)
    write_materialized_members_v0_3(packet,tmp_path)
    manifest=tmp_path/'manifest.json'
    if change=='missing':(tmp_path/'result_rows.json').unlink()
    elif change=='extra':(tmp_path/'extra.json').write_text('{}')
    elif change=='duplicate_key':manifest.write_text('{"transport_version":"0.3.0",'+manifest.read_text()[1:])
    else:
        transport=json.loads(manifest.read_text())
        if change=='version':transport['transport_version']='0.2.0'
        else:transport['manifest']=[]
        manifest.write_text(json.dumps(transport))
    with pytest.raises(ValueError,match='SN-(MEMBER|TRANSPORT)'):
        read_materialized_members_v0_3(tmp_path,source_envelope=raw,analysis_record=analysis)
