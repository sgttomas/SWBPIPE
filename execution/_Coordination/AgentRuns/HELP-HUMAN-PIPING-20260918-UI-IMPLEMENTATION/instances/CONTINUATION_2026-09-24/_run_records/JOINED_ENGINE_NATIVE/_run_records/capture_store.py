from pathlib import Path
import sqlite3,json,hashlib,sys,datetime
out=Path(__file__).parent
pid=sys.argv[1];label=sys.argv[2]
assert pid.startswith('project:blank-local-20260925') and label.isascii() and label.replace('-','').isalnum()
store=Path('/Users/ryan/Library/Application Support/com.chirality.swbpipe/openpipestress-projects.sqlite3')
con=sqlite3.connect(store.as_uri()+'?mode=ro',uri=True)
keys=['project_id','model_json','editor_intents_json','mechanics_result_json','analysis_run_json','model_hash_json','project_envelope_hash_json']
row=con.execute('SELECT '+','.join(keys)+' FROM local_projects WHERE project_id=?',(pid,)).fetchone();assert row
record=dict(zip(keys,row));model=json.loads(record['model_json']);assert model['project']['id']==pid
capture={'store_read_only':str(store),'project_id':pid,'captured_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'row':record}
private=out/'private-rows';private.mkdir(exist_ok=True);target=private/(label+'.json');assert not target.exists();target.write_text(json.dumps(capture,indent=2)+'\n')
result=json.loads(record['mechanics_result_json']) if record['mechanics_result_json'] else None
summary={'project_id':pid,'private_capture_sha256':hashlib.sha256(target.read_bytes()).hexdigest(),'model_json_sha256':hashlib.sha256(record['model_json'].encode()).hexdigest(),'model_hash':json.loads(record['model_hash_json']) if record['model_hash_json'] else None,'schema_version':model['schema_version'],'counts':{k:len(model[k]) for k in ['nodes','pipe_segments','materials','supports','load_cases']},'material_density':model['pipe_segments'][0].get('section',{}).get('material_density') if model['pipe_segments'] else None,'pressure_contract':model.get('pressure_contract'),'constitutive_basis':model['materials'][0].get('constitutive_basis') if model['materials'] else None,'poisson_ratio':model['materials'][0].get('poisson_ratio') if model['materials'] else None,'result_status':result.get('status') if isinstance(result,dict) else None,'result_schema':result.get('schema_version') if isinstance(result,dict) else None,'result_rows':len(result.get('results',[])) if isinstance(result,dict) else 0}
audit=out/'row-audits';audit.mkdir(exist_ok=True);(audit/(label+'.json')).write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps(summary,indent=2))
