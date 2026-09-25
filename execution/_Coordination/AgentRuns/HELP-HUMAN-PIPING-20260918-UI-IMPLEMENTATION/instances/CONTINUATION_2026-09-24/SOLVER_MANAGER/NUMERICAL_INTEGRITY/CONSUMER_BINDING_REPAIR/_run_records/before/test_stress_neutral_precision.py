"""Connected precision carrier → analysis 0.3 → neutral 0.3 transport tests."""
from copy import deepcopy
import json
from pathlib import Path
import pytest
from core.analysis_runs.compatibility import build_analysis_run_v0_3, build_analysis_run_v0_2, validate_analysis_run_v0_3, verify_analysis_run_record
from core.handoff.stress_neutral.package_v0_3 import (
    build_stress_neutral_export_package_v0_3, validate_stress_neutral_export_package_v0_3,
    materialized_members_v0_3, package_projection,
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
        row.update(result_id=original['id'],canonical_ref={'object_type':'Result','ref':original['id']},source_result_ref={'object_type':'Result','ref':original['id']},value=original['value'],unit='mm',dimension='length',result_family='displacement')
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
