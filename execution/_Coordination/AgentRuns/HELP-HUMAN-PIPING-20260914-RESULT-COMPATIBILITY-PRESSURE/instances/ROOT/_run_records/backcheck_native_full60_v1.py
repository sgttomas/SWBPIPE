from pathlib import Path
import hashlib,json
base=Path.cwd()
run=base/"projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260914-RESULT-COMPATIBILITY-PRESSURE"
native=run/'instances/NATIVE/FINAL_NATIVE_V4'
contract_path=base/'projects/chirality-piping/fixtures/results/semantic_contract_v0_2.json'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
contract=json.loads(contract_path.read_text())
lookup={(r['kind'],r['unit'],r['component']):r['signature_id'] for r in contract['rows']}
assert len(lookup)==60
sources=[]
def classify(label,rows,path):
 ids=set()
 for row in rows:
  signature=(row['kind'],row['unit'],(row.get('metadata') or {}).get('component'))
  assert signature in lookup,(label,signature)
  ids.add(lookup[signature])
 sources.append({'label':label,'path':str(path.relative_to(base)),'sha256':sha(path),'row_count':len(rows),'signatures':sorted(ids)})
 return ids
case_map_path=native/'_run_records/execution/consumer/case_map_v4_v2.json'
case_map=json.loads(case_map_path.read_text())
producer=set()
pair_hashes=[]
for name,c in sorted(case_map['cases'].items()):
 req=Path(c['request_path']);out=Path(c['output_path'])
 assert sha(req)==c['request_sha256'] and sha(out)==c['output_sha256']
 response=json.loads(out.read_text())
 assert response['blocked'] is False
 mechanics=response['payload']['mechanics_envelope']
 producer |= classify(name,mechanics['results'],out)
 pair_hashes.append({'case':name,'request_path':str(req.relative_to(base)),'request_sha256':sha(req),'output_path':str(out.relative_to(base)),'output_sha256':sha(out)})
assert len(producer)==52 and 'supported-source-059' in producer
store_path=native/'_run_records/store/after_first_save_v2.row.json'
stored=json.loads(store_path.read_text())
raw=json.loads(stored['mechanics_result_json'])
sparse=classify('native_sparse_saved_received_carrier',raw['results'],store_path)
dense_path=next((native/'delivery/dense').glob('openpipestress-preview-results-*.json'))
assert sha(dense_path)=='7804559e26690f643e68e70734a2933dbba17edb84cdb1e4c348d4920d880903'
dense_doc=json.loads(dense_path.read_text());envelope=dense_doc['result_envelope'];rows=[]
def pointer(document,path):
 for key in path.lstrip('/').split('/'):
  key=key.replace('~1','/').replace('~0','~')
  document=document[int(key)] if isinstance(document,list) else document[key]
 return document
assert len(envelope['row_accounting'])==len(envelope['source_annotations'])
for i,account in enumerate(envelope['row_accounting']):
 annotation=envelope['source_annotations'][i]
 assert account['source_row_index']==annotation['source_row_index']==i
 assert account['source_result_id']==annotation['source_result_id']
 target=pointer(dense_doc,account['target_field_path'])
 assert target['source_row_index']==i
 rows.append({'kind':account['source_kind'],'unit':target['source_unit'] if account['disposition']=='disclosed' else target['unit'],'metadata':annotation['metadata']})
dense=classify('native_dense_delivered_derivative_signature_projection',rows,dense_path)
union=producer|sparse|dense
expected={r['signature_id'] for r in contract['rows']}
assert union==expected and (sparse|dense)-producer=={f'supported-source-{i:03d}' for i in [37,38,39,40,41,42,43,55]}
owning_path=native/'_run_records/execution/FULL60_UNION_V4.json'
owning=json.loads(owning_path.read_text())
assert set(owning['complete_union'])==union and set(owning['authentic_producer_union'])==producer
result={'schema':'root-native-full60-backcheck-v1','status':'PASS','candidate':'8ad37207cf088025623aa1e777a97a6fcb802f48','contract':{'path':str(contract_path.relative_to(base)),'sha256':sha(contract_path)},'owning_union':{'path':str(owning_path.relative_to(base)),'sha256':sha(owning_path)},'case_map':{'path':str(case_map_path.relative_to(base)),'sha256':sha(case_map_path)},'verified_pairs':pair_hashes,'source_projections':sources,'producer_signature_count':len(producer),'complete_signature_count':len(union),'missing':sorted(expected-union),'extra':sorted(union-expected),'attribution':'Root independently reads and classifies captured exact-file evidence; no solver, canonicalizer, product build, GUI or native app execution. Dense signatures are reconstructed from the actual delivered derivative fields, not independently recovered original producer bytes. Final native lifecycle acceptance remains separate.'}
out=run/'instances/ROOT/_run_records/NATIVE_FULL60_BACKCHECK_V1.json'
assert not out.exists()
out.write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({'status':'PASS','path':str(out.relative_to(base)),'sha256':sha(out),'complete_signature_count':len(union)}))

