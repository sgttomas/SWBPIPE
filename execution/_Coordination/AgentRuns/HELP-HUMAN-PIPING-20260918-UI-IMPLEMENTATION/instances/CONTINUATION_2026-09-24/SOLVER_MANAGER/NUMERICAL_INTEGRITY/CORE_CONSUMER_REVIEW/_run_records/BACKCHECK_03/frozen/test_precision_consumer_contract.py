"""Source dispatch and scientific-value transport; no engineering-accuracy claim."""
from copy import deepcopy
import hashlib
import json
from pathlib import Path
import struct
import pytest
from core.analysis_runs.compatibility import (
    build_analysis_run, build_analysis_run_v0_3, numerical_use_standing, verify_analysis_run_record,
    PRECISION_CONTRACT_ID, PRECISION_CONTRACT_SHA256,
)
from core.serialization.canonical_json.adapter import canonical_sha256_checked_v1
from schema_validation import validate_instance

PROJECT = Path(__file__).resolve().parents[1]
BASIS = {"ref_type": "load_case", "ref_id": "load:transport"}

def source(value=1.2345678901234567e-7):
    return {"schema_version":"0.2.0", "run_id":"precision-transport", "model_ref":"precision-model",
        "producer":{"component_name":"open_pipe_stress_product_physics","component_version":"0.2.0","semantic_contract_id":PRECISION_CONTRACT_ID},
        "numerical_quality":{"value_representation":"finite_binary64","publication_quantization":"none","integrity_policy":"M03-INTEGRITY-v1","status":"not_assessed","cases":[]},
        "formulation_basis":{"profile_id":"product_preview_mechanics_v1","limitations":["Transport fixture; physics unqualified"]},
        "status":{"mechanics":"MECHANICS_SOLVED"}, "diagnostics":[],
        "results":[{"id":"result:rotation","kind":"global_nodal_rotation_x","unit":"rad","entity_ref":"node:transport","value":value,"basis_ref":BASIS}]}

def build(raw):
    return build_analysis_run(raw,input_manifest_ref={"object_type":"InputManifest","ref":"manifest:transport"},input_manifest_hash="1"*64)

def test_precision_dispatch_and_schema_binding_preserve_source_hash_and_rule_revision():
    raw=source(); before=deepcopy(raw);record=build(raw)
    assert record['schema_version']=='0.3.0'
    assert raw==before
    assert verify_analysis_run_record(record)=="match"
    validate_instance(json.loads((PROJECT/'schemas/analysis_run.schema.json').read_text()),record,instance_label="precision analysis")
    semantic=record['analysis_run']['reproducibility']['semantic_contract']
    assert semantic=={'id':PRECISION_CONTRACT_ID,'sha256':PRECISION_CONTRACT_SHA256}
    hashes=record['analysis_run']['hashes'];received=next(x for x in hashes if x['payload_scope']=='received_result')
    assert received['value']==canonical_sha256_checked_v1(raw)
    revised=build_analysis_run_v0_3(raw,input_manifest_ref={"object_type":"InputManifest","ref":"manifest:transport"},input_manifest_hash="1"*64,rule_check_status="USER_RULE_CHECKED")
    assert next(x for x in revised['analysis_run']['hashes'] if x['payload_scope']=='received_result')==received
    assert verify_analysis_run_record(revised)=="match"
    for name in ['reactions','pressure','stress','future']:
        bad=deepcopy(raw);bad['producer']['semantic_contract_id']=f'openpipestress.result_semantics/0.3.0/{name}-1'
        with pytest.raises(ValueError,match='CONTRACT_UNSUPPORTED'):build(bad)

def test_history_never_becomes_current_from_source_or_input_authenticity():
    raw=source();assert numerical_use_standing(raw,[BASIS])=='needs_recompute'
    old=deepcopy(raw);old['schema_version']='0.1.0'
    for key in ['producer','numerical_quality','formulation_basis']:del old[key]
    assert numerical_use_standing(old,[BASIS])=='needs_recompute'
    assert build(old)['analysis_run']['reproducibility']['semantic_contract']['id']=='openpipestress_result_semantics_v0_2'
    quality=raw['numerical_quality'];quality['status']='checks_passed'
    assert numerical_use_standing(raw,[BASIS])=='needs_recompute'
    quality['cases']=[{'basis_ref':BASIS,'structural_status':'passive_model_basis','solve_quality':'checks_passed','model_matrix_fidelity':'represented_equations_retained','accuracy_evidence':'not_claimed','evidence_refs':['result:rotation']}]
    assert numerical_use_standing(raw,[BASIS])=='numerically_eligible'
    quality['cases'][0]['evidence_refs']=['missing:evidence']
    assert numerical_use_standing(raw,[BASIS])=='needs_recompute'
    assert numerical_use_standing(raw,[])=='needs_recompute'

def test_independent_same_unit_bit_vectors_reach_checked_hash_and_analysis():
    for bits in [1,2,0x0010000000000000,0x3e9fffffffffffff,0x3ea0000000000000,0x3ea0000000000001,0x3fb9999999999999,0x3fb999999999999a,0x3ff0000000000001]:
        for sign in [0,1<<63]:
            value=struct.unpack('>d',(bits|sign).to_bytes(8,'big'))[0];raw=source(value)
            read=json.loads(json.dumps(raw));assert struct.pack('>d',read['results'][0]['value'])==struct.pack('>d',value)
            record=build(read);assert verify_analysis_run_record(record)=='match'
            assert canonical_sha256_checked_v1(read)==canonical_sha256_checked_v1(raw)
    with pytest.raises(ValueError):build(source(float('inf')))
    with pytest.raises(ValueError):build(source(9007199254740992.0))

def test_immutable_tables_and_historical_schema_bytes():
    table=PROJECT/'fixtures/results/semantic_contract_v0_3_precision_1.json'
    assert hashlib.sha256(table.read_bytes()).hexdigest()==PRECISION_CONTRACT_SHA256
    old=json.loads((PROJECT/'fixtures/results/semantic_contract_v0_2.json').read_text())
    assert json.loads(table.read_text())['rows']==old['rows']
    assert hashlib.sha256((PROJECT/'fixtures/results/semantic_contract_v0_2.json').read_bytes()).hexdigest()=='4d6886d19e304db897e5e9f8f0054cbee91ba7795868f9698e2bbe070bde94da'


def test_actual_headless_source_to_analysis_and_derivative_hash_binding():
    import os
    from core.analysis_runs.compatibility import validate_analysis_run_v0_3
    folder = os.environ.get("HEADLESS_PRECISION_OUTPUT_DIR")
    if not folder:
        pytest.skip("Requires actual headless product artifacts from the focused Rust test lane")
    paths = list(Path(folder).glob("*.raw.json"))
    assert paths, "No actual headless producer artifacts; a synthetic substitute is not accepted"
    for path in paths:
        raw = json.loads(path.read_text())
        derivative = json.loads(path.with_name(path.name.replace(".raw.json", ".document.json")).read_text())
        record = build(raw)
        validate_analysis_run_v0_3(record, raw)
        carrier = derivative["result_envelope"]["reproducibility"]["source_origin_bindings"][0]["received_carrier_checksum"]
        received = next(item for item in record["analysis_run"]["hashes"] if item["payload_scope"] == "received_result")
        assert carrier["value"] == received["value"] == canonical_sha256_checked_v1(raw)
        assert derivative["schema_version"] == record["schema_version"] == "0.3.0"
        for key in ["producer", "numerical_quality", "formulation_basis"]:
            assert derivative["result_envelope"][key] == raw[key]


@pytest.mark.parametrize('field,value', [
    ('diagnostics',[{'source_annotation':{'id':'forged','code':'FORGED'}}]),
    ('load_basis_refs',[{'object_type':'LoadCase','ref':'unrelated'}]),
    ('analysis_status',['MODEL_INCOMPLETE','HUMAN_REVIEW_REQUIRED','USER_RULE_CHECKED']),
    ('analysis_status',['MECHANICS_SOLVED','USER_RULE_CHECKED']),
    ('analysis_status',['MECHANICS_SOLVED','HUMAN_REVIEW_REQUIRED','UNKNOWN_RULE']),
    ('analysis_status',['MECHANICS_SOLVED','HUMAN_REVIEW_REQUIRED','USER_RULE_CHECKED','USER_RULE_FAILED']),
])
def test_rehashed_analysis_source_claims_rejected(field,value):
    from core.analysis_runs.compatibility import validate_analysis_run_v0_3, analysis_record_projection
    raw=source();record=build(raw);record['analysis_run'][field]=value
    record['analysis_run']['hashes'][0]['value']=canonical_sha256_checked_v1(analysis_record_projection(record))
    with pytest.raises(ValueError,match='SOURCE_'):
        validate_analysis_run_v0_3(record,raw)


def test_independent_model_basis_override_and_finite_rule_revisions():
    from core.analysis_runs.compatibility import validate_analysis_run_v0_3
    raw=source();raw['diagnostics']=[{'id':'diagnostic:source','code':'SOURCE','detail':None}]
    model_basis=[{'object_type':'LoadCase','ref':BASIS['ref_id']},{'object_type':'LoadCase','ref':'blocked:without-row'}]
    kwargs=dict(input_manifest_ref={'object_type':'InputManifest','ref':'m'},input_manifest_hash='1'*64)
    for rule in ['RULE_INPUTS_INCOMPLETE','USER_RULE_CHECKED','USER_RULE_FAILED']:
        record=build_analysis_run_v0_3(raw,expected_basis_refs=model_basis,rule_check_status=rule,**kwargs)
        assert record['analysis_run']['diagnostics']==[{'source_annotation':raw['diagnostics'][0]}]
        validate_analysis_run_v0_3(record,raw,expected_basis_refs=model_basis)
        with pytest.raises(ValueError,match='LOAD_BASIS'):
            validate_analysis_run_v0_3(record,raw)
    for scope in [[],[model_basis[1]],model_basis+model_basis[:1]]:
        with pytest.raises(ValueError,match='LOAD_BASIS'):
            build_analysis_run_v0_3(raw,expected_basis_refs=scope,**kwargs)
    with pytest.raises(ValueError,match='RULE_STATUS'):
        build_analysis_run_v0_3(raw,rule_check_status='arbitrary',**kwargs)
    assert 'RULE_INPUTS_INCOMPLETE' in build(raw)['analysis_run']['analysis_status']


@pytest.mark.parametrize('basis', [{},{'ref_type':'load_case','ref_id':''},{'ref_type':'load_case','ref_id':7},{'ref_type':'','ref_id':'load:a'},{'ref_type':7,'ref_id':'load:a'},[],{'ref_type':'load_case','ref_id':'load:a','extra':True}])
def test_malformed_source_basis_rejected_without_reference_coercion(basis):
    from core.analysis_runs.compatibility import validate_analysis_run_v0_3
    raw=source();record=build(raw);raw['results'][0]['basis_ref']=basis;before=deepcopy(raw)
    with pytest.raises(ValueError,match='SOURCE_REFERENCE_INVALID'):build(raw)
    with pytest.raises(ValueError,match='SOURCE_REFERENCE_INVALID'):validate_analysis_run_v0_3(record,raw)
    assert raw==before


@pytest.mark.parametrize('field,value', [('run_id',''),('run_id',7),('model_ref',[]),('model_ref',''),('result_id',7),('result_id','')])
def test_source_reference_identifiers_are_nonempty_strings(field,value):
    raw=source()
    if field=='result_id':raw['results'][0]['id']=value
    else:raw[field]=value
    with pytest.raises(ValueError,match='SOURCE_REFERENCE_INVALID'):build(raw)


@pytest.mark.parametrize('basis,expected', [(None,[]),({'ref_type':'custom_basis','ref_id':'basis:actual'},[{'object_type':'ResultBasis','ref':'basis:actual'}])])
def test_optional_and_generic_source_basis_keeps_conforming_explicit_meaning(basis,expected):
    raw=source();raw['results'][0]['basis_ref']=basis;record=build(raw)
    assert record['analysis_run']['load_basis_refs']==expected
    validate_instance(json.loads((PROJECT/'schemas/analysis_run.schema.json').read_text()),record,instance_label='optional or generic basis')


@pytest.mark.parametrize('invalid', ['',0,False,[],{},'UNKNOWN'])
def test_present_invalid_rule_status_never_becomes_default(invalid):
    from core.analysis_runs.compatibility import validate_analysis_run_v0_3
    raw=source();record=build(raw)
    kwargs=dict(input_manifest_ref={'object_type':'InputManifest','ref':'m'},input_manifest_hash='1'*64)
    with pytest.raises(ValueError,match='RULE_STATUS_INVALID'):
        build_analysis_run_v0_3(raw,rule_check_status=invalid,**kwargs)
    raw['status']['rule_check']=invalid
    with pytest.raises(ValueError,match='RULE_STATUS_INVALID'):build(raw)
    with pytest.raises(ValueError,match='RULE_STATUS_INVALID'):
        build_analysis_run_v0_3(raw,rule_check_status='USER_RULE_CHECKED',**kwargs)
    with pytest.raises(ValueError,match='RULE_STATUS_INVALID'):validate_analysis_run_v0_3(record,raw)


def test_null_or_absent_rule_status_and_override_have_explicit_default():
    kwargs=dict(input_manifest_ref={'object_type':'InputManifest','ref':'m'},input_manifest_hash='1'*64)
    for source_rule in [None,'USER_RULE_FAILED']:
        raw=source();raw['status']['rule_check']=source_rule
        record=build_analysis_run_v0_3(raw,rule_check_status=None,**kwargs)
        assert (source_rule or 'RULE_INPUTS_INCOMPLETE') in record['analysis_run']['analysis_status']
    assert 'RULE_INPUTS_INCOMPLETE' in build(source())['analysis_run']['analysis_status']
