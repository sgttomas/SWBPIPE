"""Bounded independent table/schema/source checks; does not execute a solver."""
from pathlib import Path
import sys, json, hashlib
HERE = Path(__file__).resolve().parent
PROJECT = next(p for p in HERE.parents if (p/'core/analysis_runs/compatibility.py').is_file())
sys.path[:0] = [str(PROJECT), str(PROJECT/'tests')]
from core.analysis_runs.physics_evidence import validate_physics_evidence
from core.analysis_runs.compatibility import build_analysis_run, validate_analysis_run_v0_3, verify_analysis_run_record, numerical_use_standing
from core.serialization.canonical_json.adapter import canonical_sha256_checked_v1
from schema_validation import validate_instance
import jsonschema
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
reader = HERE.parents[1]/'PHYSICS_READER_JOIN'
p1 = PROJECT/'fixtures/results/semantic_contract_v0_3_precision_1.json'
physics = PROJECT/'fixtures/results/semantic_contract_v0_3_physics_1.json'
a, b = json.loads(p1.read_text()), json.loads(physics.read_text())
schema = json.loads((PROJECT/'schemas/results.v0.3.schema.yaml').read_text())
es = jsonschema.Draft202012Validator({'$ref':'#/$defs/PhysicsContractEvidence','$defs':schema['$defs']})
report = {
    'scope':'Retained actual source/derivative checks only; no new solve, native witness or physical oracle inference.',
    'p1_sha256':sha(p1), 'physics_sha256':sha(physics),
    'unchanged_60_rows':b['rows'][:len(a['rows'])] == a['rows'],
    'rows':len(b['rows']), 'kinds':len(set(r['kind'] for r in b['rows'])),
    'signature_ids_unique':len(set(r['signature_id'] for r in b['rows'])) == len(b['rows']),
    'signature_keys_unique':len(set((r['kind'],r['unit'],r['component']) for r in b['rows'])) == len(b['rows']),
    'sources':[], 'headless':[], 'blocked':[],
}
for path in sorted((PROJECT/'fixtures/results').glob('physics_connected*mechanics*.json')):
    source = json.loads(path.read_text())
    validate_physics_evidence(source); es.validate(source['contract_evidence'])
    physical = [r for r in source['results'] if r['kind'].endswith('_v2')]
    signatures = [next((t for t in b['rows'] if (t['kind'],t['unit'],t['component']) == (r['kind'],r['unit'],r['metadata']['component'])), None) for r in physical]
    report['sources'].append({'path':str(path.relative_to(PROJECT)), 'sha256':sha(path), 'rows':len(source['results']), 'physical_rows':len(physical), 'all_physical_signatures_found':all(signatures), 'schema_and_consumer':'pass'})
for path in sorted((reader/'HEADLESS_ACTUAL').glob('*.raw.json')):
    source = json.loads(path.read_text())
    document_path = path.with_name(path.name.replace('.raw.','.document.'))
    document = json.loads(document_path.read_text())
    validate_instance(schema, document, instance_label=path.name)
    envelope = document['result_envelope']; digest = canonical_sha256_checked_v1(source)
    assert envelope['reproducibility']['source_origin_bindings'][0]['received_carrier_checksum']['value'] == digest
    assert all(envelope[k] == source[k] for k in ('producer','numerical_quality','formulation_basis','contract_evidence'))
    assert len(envelope['row_accounting']) == len(source['results'])
    report['headless'].append({'path':str(path.relative_to(PROJECT)), 'raw_sha256':sha(path), 'document_sha256':sha(document_path), 'current_canonical_hash':digest, 'source_binding':'pass', 'schema':'pass'})
for path in sorted((reader/'RULE_SOURCE_IMMUTABILITY/BLOCKED_ACTUAL').glob('*.raw.json')):
    source = json.loads(path.read_text()); validate_physics_evidence(source); es.validate(source['contract_evidence'])
    record = build_analysis_run(source, input_manifest_ref={'object_type':'InputManifest','ref':'manifest:review-blocked'}, input_manifest_hash='1'*64)
    validate_analysis_run_v0_3(record, source)
    report['blocked'].append({'path':str(path.relative_to(PROJECT)), 'sha256':sha(path), 'mechanics':source['status']['mechanics'], 'rows':len(source['results']), 'empty_evidence':source['contract_evidence'] == {'pressure':[],'connector':[],'exact_cases':[]}, 'analysis_checksum':verify_analysis_run_record(record), 'numerical_standing':numerical_use_standing(source, [c['basis_ref'] for c in source['numerical_quality']['cases']])})
full_path = PROJECT/'fixtures/model_operations/exact_pressure_authoring_model.json'
core_path = PROJECT/'core/product_physics/tests/fixtures/exact_pressure_connected_request.json'
full, core = json.loads(full_path.read_text()), json.loads(core_path.read_text())['model']
report['input_scope'] = {'core_request_sha256':sha(core_path), 'full_ui_model_sha256':sha(full_path), 'same_project_id':core['project']['id'] == full['project']['id'], 'equal_models':core == full, 'core_case_fields':[sorted(c) for c in core['load_cases']], 'ui_case_fields':[sorted(c) for c in full['load_cases']]}
(HERE/'POSITIVE_SCOPE_CHECKS.json').write_text(json.dumps(report, indent=2)+'\n')
print(json.dumps(report, indent=2))
