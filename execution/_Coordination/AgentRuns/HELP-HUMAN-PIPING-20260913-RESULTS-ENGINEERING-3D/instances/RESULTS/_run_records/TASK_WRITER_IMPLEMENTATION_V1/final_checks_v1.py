from pathlib import Path
import json,hashlib,subprocess,sys,importlib.util
from jsonschema import Draft202012Validator
lane=Path(__file__).resolve().parent;root=Path.cwd();records=lane.parent;destination=lane/'final_schema_outputs_v1';destination.mkdir(exist_ok=False)
commands=[[sys.executable,'tests/test_results_schema.py'],[sys.executable,'tests/test_result_export_v0_2.py'],[sys.executable,'tests/test_results_viewer_contract.py']]
command_results=[]
for i,cmd in enumerate(commands):
 print('EXECUTE',cmd,flush=True);run=subprocess.run(cmd,capture_output=True,text=True);(destination/f'{i}.stdout.txt').write_text(run.stdout);(destination/f'{i}.stderr.txt').write_text(run.stderr);command_results.append({'argv':cmd,'exit':run.returncode});print('EXIT',run.returncode,run.stdout[-1500:],run.stderr[-1500:],flush=True)
spec=importlib.util.spec_from_file_location('version_checks',root/'tests/test_result_export_v0_2.py');module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
folders=[lane/'fresh_producer_outputs_v1',lane/'captures/027-rust-export/rust',lane/'captures/034-current-schema-repair/desktop'];schema_results=[]
for folder in folders:
 for path in sorted(folder.glob('*.document.json')):
  errors=module.errors(json.loads(path.read_text()));schema_results.append({'path':str(path),'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'errors':errors,'error_count':len(errors)})
print('FULL_DOCUMENTS',len(schema_results),'ERRORS',sum(r['error_count'] for r in schema_results),flush=True)
# Compare against independently frozen source meanings, not adapter-derived expected values.
oracle_path=records/'MANAGER_COMPLETE_SUPPORTED_SOURCE_AND_LEGACY_ADMISSION_V2.json';oracle=json.loads(oracle_path.read_text());assert len(oracle['rows'])==60
semantic_results=[]
for folder in folders:
 for path in sorted(folder.glob('*.document.json')):
  source_path=path.with_name(path.name.replace('.document.json','.received.json'));doc=json.loads(path.read_text());source=json.loads(source_path.read_text());e=doc['result_envelope'];assert len(e['row_accounting'])==len(source['results']);seen=set()
  for i,(a,row,ann) in enumerate(zip(e['row_accounting'],source['results'],e['source_annotations'])):
   assert a['source_row_index']==i and a['source_result_id']==row['id'];assert a['target_field_path'] not in seen;seen.add(a['target_field_path']);target=doc
   for part in a['target_field_path'].strip('/').split('/'):target=target[int(part)] if isinstance(target,list) else target[part]
   assert target['source_value' if a['disposition']=='disclosed' else 'magnitude']==row['value'];assert target['source_unit' if a['disposition']=='disclosed' else 'unit']==row['unit'];assert ann['metadata']==row.get('metadata');assert ann['basis_ref']==row.get('basis_ref');assert ann['source_result_refs']==row.get('source_result_refs',[]);assert ann['observed_carrier_dimension']=={'present':'dimension' in row,'value':row.get('dimension')}
   component=row.get('metadata',{}).get('component') if isinstance(row.get('metadata'),dict) else None
   candidates=[s for s in oracle['rows'] if s['kind']==row['kind'] and s['unit']==row['unit'] and (s['component'] is None or s['component']==component)]
   if not candidates and not component:candidates=[s for s in oracle['rows'] if s['kind']==row['kind'] and s['unit']==row['unit']]
   assert candidates,(path.name,row['kind'],row['unit'],component)
   expected=candidates[0];assert ann['source_physical_semantic_dimension']==expected['source_physical_semantic_dimension'];assert ann['derivative_target_dimension']==expected['derivative_target_dimension']
   assert target['declared_semantic_dimension' if a['disposition']=='disclosed' else 'dimension']==expected['derivative_target_dimension']
   if a['disposition']=='exported_quantity':assert expected['category']=='physical_quantity' and target['family']==expected['family']
  semantic_results.append({'document':str(path),'received_source':str(source_path),'received_sha256':hashlib.sha256(source_path.read_bytes()).hexdigest(),'rows':len(source['results']),'all_value_unit_observed_dimension_annotations_and_independent_semantic_expectations_equal':True})
# Portable fixture projection must equal selected full pinned packets without origin records.
selected=json.loads((records/'MANAGER_CANDIDATE_AMENDMENT_V3/CONCRETE_METADATA_EXPECTATIONS_V3.json').read_text());portable=json.loads((root/'fixtures/results/invented/result_export_v0_2.json').read_text());assert len(portable['fixtures'])==60
for full,local in zip(selected['fixtures'],portable['fixtures']):
 for key in local:assert local[key]==full[key],key
selected_ann=json.loads((records/'MANAGER_CANDIDATE_ANNOTATION_REPAIR_V4/PARTIAL_EMPTY_UNKNOWN_ANNOTATION_EXPECTATIONS_V4.json').read_text());portable_ann=json.loads((root/'fixtures/results/invented/result_export_v0_2_rejections.json').read_text());assert len(portable_ann['cases'])==28
for full,local in zip(selected_ann['cases'],portable_ann['cases']):
 for key in local:assert local[key]==full[key],key
for p in [root/'fixtures/results/semantic_contract_v0_2.json',root/'fixtures/results/invented/result_export_v0_2.json',root/'fixtures/results/invented/result_export_v0_2_rejections.json']:assert 'AgentRuns' not in p.read_text() and '/Users/ryan' not in p.read_text()
# All seven frozen functions are preserved by exact unchanged complete source file;
# Historical reader/hash authority bytes remain independent.
freeze=json.loads((records/'IMMUTABLE_LEGACY_BINDER_SOURCE_FREEZE_V1.json').read_text());guards=[]
for original in freeze['full_file_hashes']:
 p=Path(original['origin']);p=root/'schemas/results.v0.1.schema.yaml' if p.name=='results.schema.yaml' else p;actual=hashlib.sha256(p.read_bytes()).hexdigest();assert actual==original['sha256'];guards.append({'path':str(p),'sha256':actual,'matches_original':True})
# Existing stress-neutral wire schema has no added semantic fields.
stress_path=lane/'captures/026-desktop-consumers/desktop/stress-neutral.packet.json';stress=json.loads(stress_path.read_text());stress_schema=json.loads((root/'schemas/stress_neutral_export.schema.json').read_text());stress_errors=[module.error_record(e) for e in Draft202012Validator(stress_schema).iter_errors(stress)]
print('STRESS_EXISTING_SCHEMA_ERRORS',len(stress_errors),flush=True)
result={'commands':command_results,'full_document_validation':schema_results,'source_semantic_equality':semantic_results,'independent_oracle':{'path':str(oracle_path),'sha256':hashlib.sha256(oracle_path.read_bytes()).hexdigest()},'portable_fixtures_selected_projection_equal':True,'immutable_legacy_full_file_guards':guards,'stress_existing_schema':{'path':str(stress_path),'errors':stress_errors,'error_count':len(stress_errors)}}
(destination/'RETURN.json').write_text(json.dumps(result,indent=2,ensure_ascii=False)+'\n')
if any(c['exit'] for c in command_results) or any(d['error_count'] for d in schema_results) or stress_errors:sys.exit(1)
print('PASS final focused schema/source/fixture/legacy checks')
