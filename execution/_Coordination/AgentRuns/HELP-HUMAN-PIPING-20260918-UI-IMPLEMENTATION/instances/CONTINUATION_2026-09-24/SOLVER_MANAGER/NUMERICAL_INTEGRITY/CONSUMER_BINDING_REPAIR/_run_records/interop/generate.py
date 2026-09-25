from pathlib import Path
import sys,json,hashlib,struct
from copy import deepcopy
out=Path(__file__).resolve().parent
root=out
while not (root/'projects/chirality-piping').is_dir():root=root.parent
P=root/'projects/chirality-piping';sys.path[:0]=[str(P),str(P/'tests')]
from core.serialization.canonical_json.adapter import canonical_json_checked_v1,canonical_sha256_checked_v1
from core.analysis_runs.compatibility import build_analysis_run_v0_3,validate_analysis_run_v0_3
from core.handoff.stress_neutral.package_v0_3 import build_stress_neutral_export_package_v0_3,validate_stress_neutral_export_package_v0_3,source_row_projection_v0_3
from schema_validation import validate_instance
source_path=Path('/private/tmp/piping-precision-headless-documents/curved-tip-weight-full.raw.json')
original=source_path.read_bytes();raw=json.loads(original)
assert raw['numerical_quality']['status']=='not_assessed'
fixture=P/'fixtures/results/invented/result_export_v0_2.json'
case=next(c for c in json.loads(fixture.read_text())['producer_cases'] if c['case_id']=='curved-tip-weight-full')
assert case['model']['project']['id']==raw['model_ref']
ref=lambda kind,value:{'object_type':kind,'ref':value}
# A new truthful input-custody manifest reconstructed from the maintained fixture,
# not a claimed captured native audit manifest or an eligibility attestation.
manifest={'manifest_kind':'reconstructed_interop_input_custody','model':case['model'],'materials':[],'fixture_ref':'fixtures/results/invented/result_export_v0_2.json#curved-tip-weight-full','fixture_sha256':hashlib.sha256(fixture.read_bytes()).hexdigest(),'origin':'Reconstructed from the unchanged maintained input fixture used by the actual retained headless test; original native input manifest was not captured here.'}
mref=ref('InputManifest','interop-custody:curved-tip-weight-full')
analysis=build_analysis_run_v0_3(raw,input_manifest_ref=mref,input_manifest_hash=canonical_sha256_checked_v1(manifest),settings_ref=ref('SolverSettings','unavailable:original-settings-record-not-captured'),unit_system_ref=ref('UnitSystem','unavailable:original-unit-system-record-not-captured'))
provenance={'source_name':'Cross-language transport verification of actual retained producer output','source_location':'SOLVER_MANAGER/NUMERICAL_INTEGRITY/CONSUMER_BINDING_REPAIR','source_license':'project-governed','contributor':'WORKING_ITEMS solver_manager','contributor_certification':'No engineering acceptance or Current qualification','redistribution_status':'not_determined_by_this_interop_check','review_status':'pending'}
rows=[{**source_row_projection_v0_3(raw,r),'provenance':deepcopy(provenance)} for r in reversed(raw['results'])]
maps=[{'canonical_ref':deepcopy(r['canonical_ref']),'export_ref':ref('ExternalInteropRow',f'interop-row:{i}'),'mapping_status':'mapped','loss_category':'exported'} for i,r in enumerate(rows)]
packet=build_stress_neutral_export_package_v0_3(source_envelope=raw,analysis_record=analysis,export_id='stress-neutral:actual-curved-tip-interop',source_result_ref=ref('ResultEnvelope','result-envelope:'+raw['run_id']),source_run_ref=ref('AnalysisRun',raw['run_id']),source_model_ref=ref('Model',raw['model_ref']),source_hashes=deepcopy(analysis['analysis_run']['hashes']),result_rows=rows,stable_id_map=list(reversed(maps)),loss_report=[],reproducibility_refs=[ref('AnalysisRun',raw['run_id']),mref],provenance=provenance)
validate_analysis_run_v0_3(analysis,raw);validate_stress_neutral_export_package_v0_3(packet,source_envelope=raw,analysis_record=analysis)
for name,obj,schema in [('source',raw,None),('analysis',analysis,'analysis_run.schema.json'),('packet',packet,'stress_neutral_export.schema.json'),('input-custody-manifest',manifest,None)]:
 text=canonical_json_checked_v1(obj);(out/(name+'.json')).write_text(text+'\n')
 if schema:validate_instance(json.loads((P/'schemas'/schema).read_text()),json.loads(text),instance_label='actual interop '+name)
copy=json.loads((out/'source.json').read_text());assert copy==raw
assert all(struct.pack('>d',float(a['value']))==struct.pack('>d',float(b['value'])) for a,b in zip(raw['results'],copy['results']))
(out/'source.original.json').write_bytes(original)
record={'actor':'/root/solver_manager','source_origin':str(source_path),'source_original_sha256':hashlib.sha256(original).hexdigest(),'rows':len(rows),'actual_source_status':raw['numerical_quality']['status'],'source_changed':False,'input_manifest_standing':'new reconstructed fixture input-custody document; not a captured original native audit manifest','purpose':'actual Python builder/canonical serializer to TypeScript reader/validator; no TSbuilder/Current/numerical/physics acceptance claim','features':['custom provenance','reversed source row order','custom typed export namespace','reversed stable-map order'],'hashes':{p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in out.glob('*.json')}}
(out/'GENERATION.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record,indent=2))
