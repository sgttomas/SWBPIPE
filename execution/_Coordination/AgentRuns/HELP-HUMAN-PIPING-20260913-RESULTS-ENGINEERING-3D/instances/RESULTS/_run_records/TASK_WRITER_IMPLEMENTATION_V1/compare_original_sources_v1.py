from pathlib import Path
import json,hashlib
lane=Path(__file__).resolve().parent;base=lane.parent/'TASK_WRITER/baseline_outputs_v1';fresh=lane/'fresh_producer_outputs_v1'
records=[];total=0
for old in sorted(base.glob('*.mechanics.json')):
 case=old.name.removesuffix('.mechanics.json'); new=fresh/(case+'.received.json'); a=json.loads(old.read_text());b=json.loads(new.read_text());assert a==b,case;total+=len(a['results']);
 model=base/(case+'.model.json');request=base/(case+'.request.json');assert model.read_bytes()==(fresh/(case+'.model.json')).read_bytes() or json.loads(model.read_text())==json.loads((fresh/(case+'.model.json')).read_text());assert json.loads(request.read_text())==json.loads((fresh/(case+'.request.json')).read_text());records.append({'case':case,'original_source':str(old),'original_sha256':hashlib.sha256(old.read_bytes()).hexdigest(),'fresh_source':str(new),'fresh_sha256':hashlib.sha256(new.read_bytes()).hexdigest(),'complete_decoded_carrier_equal':True,'rows':len(a['results']),'exact_value_unit_raw_dimension_presence_and_annotations_equal':True,'original_model_sha256':hashlib.sha256(model.read_bytes()).hexdigest(),'original_request_sha256':hashlib.sha256(request.read_bytes()).hexdigest()})
assert len(records)==12;assert total==2991
out={'original_cases':len(records),'original_rows':total,'records':records,'gap_additional_cases':'separate new actual runtime, not part of original2991','claim':'exact decoded original mechanics carriers, including numerical values/units/source annotations and unchanged source dimension absence'}
(lane/'ORIGINAL_SOURCE_PARITY_V1.json').write_text(json.dumps(out,indent=2)+'\n');print('PASS',len(records),total)
