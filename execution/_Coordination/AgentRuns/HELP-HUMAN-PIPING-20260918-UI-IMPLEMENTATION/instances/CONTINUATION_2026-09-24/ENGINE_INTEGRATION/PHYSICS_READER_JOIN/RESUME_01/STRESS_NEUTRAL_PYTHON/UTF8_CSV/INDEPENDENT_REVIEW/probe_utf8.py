"""Independent UTF8 wire controls and retained Unicode producer transport, not a solve."""
from pathlib import Path
from copy import deepcopy
import json,sys,csv,io,hashlib,struct
HERE=Path(__file__).resolve().parent
PROJECT=next(p for p in HERE.parents if (p/'core/handoff/stress_neutral/package_v0_3.py').is_file())
sys.path[:0]=[str(PROJECT),str(PROJECT/'tests')]
import test_stress_neutral_physics_source as fixture
from core.handoff.stress_neutral import package_v0_3 as sn
from core.analysis_runs.source_blocks import domain_hash
bits=lambda n:struct.pack('>d',float(n)).hex()
ids=['result:😀','result:\ue000','result:e\u0301','result:é','\ufeffauthored','id,"q"\r\n tail ']
rows=[fixture.csv_transport_row(s) for s in ids]
wire=sn._utf8_csv_bytes(sn._render_utf8_csv(rows))
records=list(csv.DictReader(io.StringIO(wire.decode('utf-8'),newline=''),strict=True))
assert [r['result_id'] for r in records]==sorted(ids)
assert records[0]['load_case_ref']=='case:温度,"a"\r\nline'
assert records[0]['station_ref']=='端点,"i"\nnext'
assert records[0]['component_ref']=='pipe:é\rline'
sn._validate_received_csv(wire.decode('utf-8'),sorted(rows,key=lambda r:r['result_id']),utf8=True)
assert 'result:e\u0301' in [r['result_id'] for r in records] and 'result:é' in [r['result_id'] for r in records]
out={'scope':'Pure wire vectors and unchanged actual source-to-nine-member projections; no new producer/native invocation','pure_wire_sha256':hashlib.sha256(wire).hexdigest(),'pure_exact_ids_and_quoted_CR_LF':True,'actual':[]}
for mode in ['sparse_interactive','dense_scrutiny']:
    path=PROJECT/f'fixtures/product_preview/physics_source/n05_unicode-{mode}.raw.json'
    raw,analysis,packet=fixture.captured(path)
    request=json.loads((path.parent/'n05_unicode.request.json').read_text())
    assert raw['source_block_recovery']['body']['invocation']['value']==domain_hash('source_blocks_invocation_v1',{'request':request,'solver_mode':mode})
    members=sn.materialized_members_v0_3(packet)
    reopened=sn.reconstruct_materialized_members_v0_3(members,source_envelope=raw,analysis_record=analysis)
    fixture.validate_instance(json.loads(fixture.SCHEMA.read_text()),reopened)
    csvbytes=members['stress_neutral_results.csv']
    check=next(c for c in reopened['manifest']['checksums'] if c['payload_ref']['ref']=='stress_neutral_results.csv')
    assert check['canonicalization']=='utf8_csv_record_lf_v1' and check['value']==hashlib.sha256(csvbytes).hexdigest()
    actual_csv=list(csv.DictReader(io.StringIO(csvbytes.decode('utf-8'),newline=''),strict=True))
    assert [r['result_id'] for r in actual_csv]==sorted(r['id'] for r in raw['results'])
    by_id={r['id']:r for r in raw['results']}
    assert all(a['source_value_bits']==bits(by_id[a['source_result_id']]['value']) and a['source_row'].get('metadata')==by_id[a['source_result_id']].get('metadata') for a in reopened['source_annotations'])
    refused=[]
    for label,bad in [('bad_utf8',b'\xff'+csvbytes),('overlong',b'\xc0\x80'+csvbytes),('surrogate',b'\xed\xa0\x80'+csvbytes),('BOM',b'\xef\xbb\xbf'+csvbytes),('record_CRLF',csvbytes.replace(b'\n',b'\r\n',1))]:
        changed=dict(members);changed['stress_neutral_results.csv']=bad
        try:sn.reconstruct_materialized_members_v0_3(changed,source_envelope=raw,analysis_record=analysis)
        except ValueError:refused.append(label)
    assert len(refused)==5
    out['actual'].append({'mode':mode,'source_sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'rows':len(actual_csv),'sidecars_and_metadata_exact':True,'nine_members':len(members),'refused_wires':refused,'members':{k:hashlib.sha256(v).hexdigest() for k,v in members.items()}})
fixture.test_precision_package_and_all_nine_member_bytes_are_unchanged()
out['p1_packet_and_nine_members']='unchanged golden pass'
(HERE/'UTF8_PROBES.json').write_text(json.dumps(out,indent=2,ensure_ascii=True)+'\n')
print(json.dumps({**out,'actual':[{k:v for k,v in r.items() if k!='members'} for r in out['actual']]},indent=2))

