"""Exact frozen-candidate semantic preservation and declared bitmap-work delta."""
from pathlib import Path
import hashlib,json,re,sys
COMMON=['REQUEST','CASE','INPUT','ROWS','VERDICTS','PASS','FULL_CASE','G5A','OPERATIONAL','G5A_WORK','TERM_MAP','G5A_DATA','NATIVE_ROWS','NATIVE_IDENTITY']
TAGS=['I45_'+x for x in COMMON]+['I47_'+x for x in COMMON+['BOUNDARY','SELECTION','BASIS_CAPTURE','ANCILLARY','CAPTURE']]
def read(path):
    data={k:[] for k in TAGS+['I45_ADAPTER','I47_ADAPTER','I50_RECORD']}
    for line in Path(path).read_text().splitlines():
        tag,_,value=line.partition(' ')
        if tag not in data:continue
        try:value=json.loads(value)
        except json.JSONDecodeError:pass
        data[tag].append(value)
    return data
def counts(value):
    assert 'fault: Cell { value: None }' in value
    m=re.search(r'counts: Cell \{ value: (\[[^\]]+\])',value)
    assert m
    values=json.loads(m.group(1));assert len(values)==10
    return values
def delta(old,new,writes):
    before,after=counts(old),counts(new)
    expected=[0,0,writes,3,0,0,0,0,0,0]
    assert [b-a for a,b in zip(before,after)]==expected,(before,after,expected)
    return expected
old,new=map(read,sys.argv[1:3])
for tag in TAGS:
    assert len(old[tag])==(2 if tag.startswith('I45_') else 4),tag
    assert old[tag]==new[tag],tag
work=[]
for tag in ['I45_ADAPTER','I47_ADAPTER']:
    assert len(old[tag])==len(new[tag])==(2 if tag.startswith('I45_') else 4)
    for i,(a,b) in enumerate(zip(old[tag],new[tag])):work.append({'tag':tag,'case':i,'delta':delta(a,b,13)})
assert len(old['I50_RECORD'])==len(new['I50_RECORD'])==2
for a,b in zip(old['I50_RECORD'],new['I50_RECORD']):
    assert set(a)==set(b)
    assert {k:v for k,v in a.items() if k!='adapter'}=={k:v for k,v in b.items() if k!='adapter'},a['mode']
    work.append({'tag':'I50_RECORD','mode':a['mode'],'delta':delta(a['adapter'],b['adapter'],19)})
result={'passed':True,'old_semantic_records':sum(len(old[k]) for k in TAGS),'old_cases':6,'named_modes':2,
        'named_comparison':'all fields equal except adapter; includes requests/envelopes/source/native/facts/verdicts/observations/numeric/G5a/observable outcomes and certificate work',
        'exact_adapter_deltas':work,'count_rule':'one added ValidationEntry per bitmap and one added MapWrite per filled slot; all other counters unchanged',
        'inputs':[{'location':str(Path(x).resolve()),'bytes':Path(x).stat().st_size,'sha256':hashlib.sha256(Path(x).read_bytes()).hexdigest()} for x in sys.argv[1:3]]}
Path(sys.argv[3]).write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({'passed':True,'old_semantic_records':result['old_semantic_records'],'named_modes':2,'checked_work_deltas':len(work)}))
