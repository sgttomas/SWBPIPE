import hashlib, importlib.metadata, json, pathlib, shutil, sys
from collections import Counter
r = pathlib.Path.cwd(); w = r/'projects/chirality-piping'
o = w/'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260913-RESULTS-ENGINEERING-3D-PLAN/instances/RESULTS/raw'
sys.path.insert(0, str(w/'tests'))
from schema_validation import load_schema, schema_for_definition, validate_schema_document
from jsonschema import Draft202012Validator
h=lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
result_schema=load_schema(w/'schemas/results.schema.yaml'); analysis_schema=load_schema(w/'schemas/analysis_run.schema.json')
for s in [result_schema,analysis_schema]: validate_schema_document(s)
def errors(schema,value):
 return [{'instance_path':'/'.join(map(str,e.absolute_path)),'schema_path':'/'.join(map(str,e.absolute_schema_path)),'message':e.message} for e in sorted(Draft202012Validator(schema).iter_errors(value),key=lambda e:str(list(e.path)))]
report={'environment':{'python':sys.executable,'python_version':sys.version,'jsonschema':importlib.metadata.version('jsonschema')},'method':'existing repository schema_validation helpers + Draft202012Validator; no product execution','schema_sha256':{'results':h(w/'schemas/results.schema.yaml'),'analysis_run':h(w/'schemas/analysis_run.schema.json')},'historical_cases':[],'native_cases':[]}
p5=w/'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260909-VIEWPORT-ROUTING/instances/P5/schema_witness/outputs'
for p in sorted(p5.glob('*.json')):
 shutil.copyfile(p,o/p.name);d=json.loads(p.read_text());es=errors(result_schema,d);report['historical_cases'].append({'origin':str(p.relative_to(r)),'input_sha256':h(p),'scope':'FULL canonical wrapper; historical-produced, current schema','error_count':len(es),'errors':es,'status':'FAIL' if es else 'PASS'})
n=w/'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260913-WORKFLOW-CONTACT/instances/NATIVE_VERIFY/POST_DEC025_VIEWPORT/_run_records'
for name in ['initial-solved500.json','blocked-native.json','fresh-reopen-before-save.json']:
 p=n/name;shutil.copyfile(p,o/('native-'+name));d=json.loads(p.read_text());m=json.loads(d['mechanics_result_json']);a=json.loads(d['analysis_run_json']);es=errors(analysis_schema,a)
 rowes=[]
 for row in m['results']:
  if row.get('metadata'):rowes += [dict(e,row_id=row['id']) for e in errors(schema_for_definition(result_schema,'ResultMetadata'),row['metadata'])]
 report['native_cases'].append({'origin':str(p.relative_to(r)),'input_sha256':h(p),'mechanics_status':m['status'],'rows':len(m['results']),'dimensions_present':sum('dimension' in x for x in m['results']),'analysis_whole_schema_error_count':len(es),'analysis_whole_schema_errors':es,'metadata_definition_only_error_count':len(rowes),'metadata_definition_only_errors':rowes,'limit':'No canonical complete wrapper in native saved packet; row metadata check is a boundary mismatch probe, not a full-document pass. No dedicated MechanicsEnvelope schema discovered.'})
report['historical_total_errors']=sum(c['error_count'] for c in report['historical_cases']);report['historical_error_classes']=dict(Counter(e['schema_path'].split('/')[-2] for c in report['historical_cases'] for e in c['errors']))
(o/'VALIDATION.json').write_text(json.dumps(report,indent=2)+'\n')
for c in report['historical_cases']:print(pathlib.Path(c['origin']).name,c['status'],c['error_count'])
for c in report['native_cases']:print(pathlib.Path(c['origin']).name,'analysis-errors',c['analysis_whole_schema_error_count'],'metadata-only-errors',c['metadata_definition_only_error_count'],'rows',c['rows'],'dimensions',c['dimensions_present'])
print('historical total',report['historical_total_errors'],report['historical_error_classes'])
sys.exit(1 if report['historical_total_errors'] else 0)
