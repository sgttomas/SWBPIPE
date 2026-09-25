"""Independent SJ-R1 backcheck; bounded probes, no build or full suite."""
from pathlib import Path
import hashlib, json, sys
from unittest.mock import patch

HERE = Path(__file__).resolve().parent
PROJECT = next(p for p in HERE.parents if (p/'core/serialization/canonical_json/adapter.py').is_file())
sys.path.insert(0, str(PROJECT))
from core.serialization.canonical_json import adapter

original_dumps = json.dumps
calls = []
def observe(value, *args, **kwargs):
    assert not isinstance(value, list), 'Oversized snapshot reached serialization'
    assert not (isinstance(value,dict) and value.get('items')), 'Oversized request reached serialization'
    output = original_dumps(value,*args,**kwargs)
    calls.append({'kind':type(value).__name__,'characters':len(output)})
    return output
outcomes=[]
with patch.object(adapter.json,'dumps',observe), patch.object(adapter.subprocess,'run',side_effect=AssertionError('Unexpected process')):
    shared='x'*(1024*1024)
    try:
        adapter.canonical_json_binary64_v1([shared]*9)
        raise AssertionError('Alias expansion admitted')
    except ValueError as error:
        assert 'BYTE-LIMIT' in str(error)
        outcomes.append({'case':'original one-MiB alias repeated nine times','result':str(error),'full_snapshot_serialization_performed':False})
    with patch.object(adapter,'_BINARY64_MAX_REQUEST_BYTES',512):
        for name,call in [('raw_batch',lambda:adapter.canonicalize_binary64_text_batch([(str(i),'"'+'a'*150+'"') for i in range(4)])),
                          ('snapshot_batch',lambda:adapter.canonicalize_binary64_batch([(str(i),'a'*150) for i in range(4)]))]:
            try:
                call();raise AssertionError('Cumulative request admitted')
            except ValueError as error:
                assert 'BYTE-LIMIT' in str(error)
                outcomes.append({'case':name,'test_request_budget':512,'result':str(error),'full_request_serialization_performed':False})
assert adapter._BINARY64_MAX_REQUEST_BYTES == 64*1024*1024
accounting=[]
for index,value in enumerate([None,True,False,[],{},[1,2],{'a':[None,True,False]},'é😀\x00\n"\\',
                              {'é/😀':[-5e-324,1e160,0.1]},['alias'*10]*20]):
    state=[0,0];frozen=adapter._freeze_binary64(value,set(),0,state)
    serialized=original_dumps(frozen,ensure_ascii=False,separators=(',',':'),allow_nan=False)
    assert state[1] == len(serialized.encode('utf8'))
    accounting.append({'case':index,'preflight_bytes':state[1],'actual_bytes':len(serialized.encode('utf8'))})
report={'adapter_sha256':hashlib.sha256((PROJECT/'core/serialization/canonical_json/adapter.py').read_bytes()).hexdigest(),
        'scope':'New-profile resource backcheck only. Small local cumulative budget uses unchanged production path; no actual 64MiB/OOM experiment.',
        'outcomes':outcomes,'admitted_small_materializations':calls,'byte_accounting':accounting,'production_request_limit':adapter._BINARY64_MAX_REQUEST_BYTES}
(HERE/'RESOURCE_REPAIR_PROBES.json').write_text(original_dumps(report,indent=2)+'\n')
print(original_dumps(report,indent=2))
