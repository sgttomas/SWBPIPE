"""Compare the newly executed old specimens with their frozen accepted captures.
Equality transfers the existing complete truth-cover checks; work is separate.
"""
from pathlib import Path
import hashlib,json,sys
COMMON=['REQUEST','CASE','INPUT','ROWS','VERDICTS','PASS','FULL_CASE','G5A','OPERATIONAL','G5A_WORK','TERM_MAP','G5A_DATA','NATIVE_ROWS','NATIVE_IDENTITY']
TAGS=['I45_'+x for x in COMMON]+['I47_'+x for x in COMMON+['BOUNDARY','SELECTION','BASIS_CAPTURE','ANCILLARY','CAPTURE']]
def read(path):
    data={k:[] for k in TAGS}
    for line in Path(path).read_text().splitlines():
        tag,_,value=line.partition(' ')
        if tag not in data:continue
        try:value=json.loads(value)
        except json.JSONDecodeError:pass
        data[tag].append(value)
    return data
old,new=map(read,sys.argv[1:3]);checked=[]
for tag in TAGS:
    assert len(old[tag])==(2 if tag.startswith('I45_') else 4),(tag,'missing frozen capture')
    if tag=='I47_CAPTURE':
        # This Debug tuple also carries the intentionally extended capacity
        # summary. Verify its exact new three entries, then compare every
        # pre-existing identity/property/capacity/source-correction field.
        extension=', ("supports", 1), ("spring_map", 0), ("support_fixed", 1)'
        assert all(x.count(extension)==1 for x in new[tag])
        new[tag]=[x.replace(extension,'') for x in new[tag]]
    assert old[tag]==new[tag],(tag,'semantic capture changed')
    checked.append({'tag':tag,'records':len(old[tag]),'identical':True})
answer={'checked':checked,'semantic_records':sum(x['records'] for x in checked),'case_count':6,
        'unchanged':'actual requests/source identities/rows/values/classes/scales/bounds/predicates/G5a/observables/complete outcomes',
        'not_claimed_unchanged':'new adapter and certificate work/capacity ledgers',
        'inputs':[{'location':str(Path(p).resolve()),'bytes':Path(p).stat().st_size,'sha256':hashlib.sha256(Path(p).read_bytes()).hexdigest()} for p in sys.argv[1:3]]}
Path(sys.argv[3]).write_text(json.dumps(answer,indent=2)+'\n')
print(json.dumps({'passed':True,'cases':6,'semantic_records':answer['semantic_records']}))
