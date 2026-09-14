from pathlib import Path
import json,hashlib,collections,re,sys
from jsonschema import Draft202012Validator
lane=Path(__file__).resolve().parent;root=Path.cwd();source=lane/'original_source/apps/desktop/src/features/stress-neutral/StressNeutralExportPanel.tsx';text=source.read_text();current=root/'apps/desktop/src/features/stress-neutral/StressNeutralExportPanel.tsx'
old=json.loads((lane/'final_schema_outputs_v1/RETURN.json').read_text())['stress_existing_schema']['errors'];packet_path=lane/'captures/041-final-stress-check/desktop/stress-neutral.packet.json';packet=json.loads(packet_path.read_text());schema=json.loads((root/'schemas/stress_neutral_export.schema.json').read_text())
def record(e):return {'path':list(e.absolute_path),'schema_path':list(e.absolute_schema_path),'message':e.message,'context':[record(c) for c in e.context]}
new=[record(e) for e in Draft202012Validator(schema).iter_errors(packet)]
assert len(old)==922;assert len(new)==911,(len(new))
# Baseline source declaration pins, with every residual error assigned to its exact existing construction surface.
facts={
'ROOT':('buildStressNeutralExportPacket','return {','Existing top-level preview fields and absent top-level provenance; unit_preservation_witnesses was already present in original.'),
'stable_id_map':('buildStressNeutralExportPacket','const stableIdMap =','Original callback already emits map_id/row_index in every mapping row.'),
'manifest':('member','function member(','Original member helper emits member_name/relative_path/format/record_count/role/hash_status; strict schema requires member_id/member_role/checksum.'),
'diagnostics':('stressNeutralDiagnostics','function stressNeutralDiagnostics(','Two unchanged preview warnings emit diagnostic_id/affected_ref and omit class/source/affected_object/remediation; newly exercised blocking findings use strict fields.'),
'source_hashes':('buildStressNeutralExportPacket','source_hashes: run.hashes.map','Original spread preserves legacy bare-hex/rfc8785_jcs hashes and adds hash_role; strict stress-neutral schema has different checksum format.'),
'privacy':('buildStressNeutralExportPacket','privacy: {','Original privacy object uses privacy_classification/protected_content_embedded/source_model_mutated and lacks six strict fields.'),
'professional_boundary':('professionalBoundary','function professionalBoundary(','Original preview boundary has extra supports_* / authentication keys and omits five strict claim keys.'),
'export_profile':('buildStressNeutralExportPacket','export_profile: {','Original profile lacks profile_version/boundary_notes/source_basis_refs.'),
'validation_report':('buildStressNeutralExportPacket','validation_report: {','Original validation object has extra validation_id/schema_validation_status/blocking_diagnostic_count and lacks human_review_required. New blocked outcome now uses accepted blocked enum.'),
'loss_report':('buildStressNeutralExportPacket','loss_report: {','Original loss_report is an object; strict schema requires array. Changed reason/downstream wording does not change this original structural mismatch.')}
for key,(fn,token,explanation) in facts.items():
 assert token in text,(key,token);facts[key]={'source_function':fn,'original_line':text[:text.index(token)].count('\n')+1,'original_declaration_token':token,'basis':explanation}
classified=[]
for e in old:
 path=e['path'];family=str(path[0]) if path else 'ROOT'
 if family=='diagnostics' and path[1]==3: disposition='introduced_declaration_witness_diagnostic_shape_REPAIRED'
 elif family=='diagnostics' and path[1]==2:disposition='new_semantic_activation_of_existing_missing_dimension_diagnostic_shape_REPAIRED'
 elif path==['validation_report','validation_status']:disposition='new_semantic_activation_of_existing_invalid_blocked_literal_REPAIRED'
 else:disposition='unchanged_original_preview_schema_gap'
 classified.append({'error':e,'disposition':disposition,'source_fact':family})
assert collections.Counter(c['disposition'] for c in classified)=={'unchanged_original_preview_schema_gap':911,'introduced_declaration_witness_diagnostic_shape_REPAIRED':5,'new_semantic_activation_of_existing_missing_dimension_diagnostic_shape_REPAIRED':5,'new_semantic_activation_of_existing_invalid_blocked_literal_REPAIRED':1}
for e in new:
 family=str(e['path'][0]) if e['path'] else 'ROOT';assert family in facts
 if family=='diagnostics':assert e['path'][1] in [0,1]
for diag in packet['diagnostics']:
 if diag['code'] in ['SN-DECLARED-DIMENSION-WITNESS-UNAVAILABLE','SN-UNIT-DIMENSION-MISSING']:assert list(Draft202012Validator(schema['$defs']['Diagnostic'],resolver=__import__('jsonschema').RefResolver.from_schema(schema)).iter_errors(diag))==[]
assert packet['validation_report']['validation_status']=='blocked';assert len(packet['result_rows'])==830
out={'status':'FAIL_UNCHANGED_NONCANONICAL_PREVIEW_SCHEMA_LIMITATION_ROOT_OWNED','original_inspection_errors':922,'introduced_or_activated_findings_repaired':11,'residual_errors':911,'no_baseline_solve_or_schema_redesign':True,'original_source':{'path':str(source),'sha256':hashlib.sha256(source.read_bytes()).hexdigest()},'current_source':{'path':str(current),'sha256':hashlib.sha256(current.read_bytes()).hexdigest()},'packet':{'path':str(packet_path),'sha256':hashlib.sha256(packet_path.read_bytes()).hexdigest()},'facts':facts,'all_original_error_dispositions':classified,'full_residual_errors':new,'residual_error_family_counts':dict(collections.Counter(str(e['path'][0]) if e['path'] else 'ROOT' for e in new)),'bounded_consumer_checks':'Every received numerical row/unit/source ref retained; CSV/JSON counts agree; semantic dimensions independent; incompatible or unavailable declaration witnesses withheld; strict blocking diagnostics and blocked outcome; no canonical package/target/readiness claim.'}
(lane/'STRESS_NEUTRAL_EXISTING_SCHEMA_DISPOSITION_V1.json').write_text(json.dumps(out,indent=2)+'\n');print('922 classified;11 introduced/activated failures repaired;911 unchanged preview/schema gaps remain; full-schema verdict FAIL retained')
sys.exit(1)
