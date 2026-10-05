"""Independent integer-only reconstruction. No author program or numerical kernel executes."""
from pathlib import Path
from itertools import product
import hashlib,json,subprocess,os
root=Path.cwd()
r=Path('projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30')
out=r/'REVIEW_RV44/f2a_no_wrap_01/_run_records'
author=r/'I29/f2a_no_wrap_bound_03'
rev='4c2c1e9857'; source='49034a940f3f8cd3f3da4d4cbc839943b808063d'
env=dict(os.environ,GIT_OPTIONAL_LOCKS='0')
origins=[]
def at(path,revision):
    data=subprocess.check_output(['git','show',f'{revision}:{path}'],env=env)
    origins.append({'path':str(path),'revision':revision,'bytes':len(data),'sha256':hashlib.sha256(data).hexdigest()})
    return data
files=subprocess.check_output(['git','ls-tree','-r','--name-only',rev,'--',str(author)],env=env,text=True).splitlines()
inv=json.loads(at(author/'WRITE_INVENTORY.json',rev))
assert set(files)=={str(author/'WRITE_INVENTORY.json')}|{str(author/x['path']) for x in inv['files']}
blobs={}
for row in inv['files']:
    b=at(author/row['path'],rev); blobs[row['path']]=b
    assert len(b)==row['bytes'] and hashlib.sha256(b).hexdigest()==row['sha256']
assert len(files)==8 and len(blobs)==7
payload=json.loads(blobs['_run_records/ARITHMETIC.json'])
input_rows=json.loads(blobs['_run_records/ARITHMETIC_ORIGINS.json'])
inputs={}
for item in input_rows:
    data=at(item['path'],source)
    assert len(data)==item['bytes'] and hashlib.sha256(data).hexdigest()==item['sha256']
    inputs[item['path']]=data
# Independently account loop events and their prices.
def prices(l):
    return {'add':2*l,'mul':l*l,'div':(l+1)*(64*l+2),'round':2*l,'sqrt':(l+2)*(64*l+2),'twoprod':l*l+4*l}
def reconstructed(f,e,w,z,b,l=16):
    p=prices(l); q=e*max(w-1,0)//2
    facwide=q*p['mul']+q*p['add']+e*p['div']+e*p['mul']+2*e*p['add']
    fsum=3*f*(l+256)+3*f*256+128*f+6*z*(l+256)+6*z*256+128*z
    qrow=w*max(w-1,0)//2
    row=qrow*p['mul']+qrow*p['add']+w*p['div']+w*p['mul']+2*w*p['add']+3*(l+256)+3*256+128+6*z*(l+256)+6*z*256+128*z
    # Norm F rounds, six abs rounds, five dot rounds, two rounds/block/observer.
    rounding=f+6+5+2*b*11
    # Plain adds: six abs vectors + two observer vectors; dots have two wide terms.
    addterms=6*f+2*f*11+2*5*f
    cparts={'term':z*(l+256)+addterms*256,'shift':z*256+addterms*256,'net':rounding*256,'rounded':rounding*128}
    cwide=11*(2*e*p['mul']+2*e*p['add']+f*p['div'])+rounding*p['round']+5*f*p['twoprod']+(11*b+f+3)*p['div']+p['mul']
    if f==0:cparts=dict.fromkeys(cparts,0);cwide=0
    ctotal=cwide+sum(cparts.values())
    margin=f*sum(2*prices(width)['round']+prices(width)['div'] for width in [4,16])
    total=facwide+fsum+ctotal+margin
    return {'Q_upper':q,'factor_wide_LME':facwide,'factor_SumWork_upper':fsum,'factor_row_with_failure_upper':row,'condition_component_upper':cparts,'condition_wide_LME':cwide,'condition_total_upper':ctotal,'pivot_margin_charged_upper':margin,'covered_local_sum_upper':total,'strictly_below_u64_MAX':total<2**64-1,'global_admission_established':False}
hrecords=[json.loads(line) for line in inputs[next(p for p in inputs if p.endswith('counts.jsonl'))].splitlines()]
assert len(hrecords)==len(payload['H'])==33
h=[]
for raw,claimed in zip(hrecords,payload['H']):
    f,hc,w,z,b=(raw['w1_'+x] for x in ['free_dofs','profile_entries','half_bandwidth','pattern_entries','blocks'])
    assert raw['model']==claimed['id'] and raw['family']==claimed['family']
    expect=reconstructed(f,hc-f,w,z,b)
    assert expect==claimed['L16']
    h.append({'id':raw['model'],'family':raw['family'],'count_fields':{'F':f,'H':hc,'E':hc-f,'W':w,'Z':z,'B':b},'local_upper':expect['covered_local_sum_upper']})
v=[];missing=[];vclaims=iter(payload['VR_storage_only'])
for path in sorted(p for p in inputs if p.endswith('.json')):
    for row in json.loads(inputs[path]):
        arrays=[x['storage'] for x in row['attempts']]
        if not arrays:
            missing.append(row['id']);continue
        shapes={(x['pattern_entries'],x['profile_entries']) for x in arrays};assert len(shapes)==1
        z,hc=shapes.pop();c=next(vclaims)
        assert row['id']==c['id'] and row['family']==c['family'] and path==c['record']
        expect=reconstructed(hc,hc,hc,z,hc)
        assert expect==c['L16']
        v.append({'id':row['id'],'H':hc,'Z':z,'local_upper':expect['covered_local_sum_upper']})
assert next(vclaims,None) is None
assert len(v)==193 and len(missing)==8
assert missing==[x['id'] for x in payload['VR_without_counts']]
# Finite integer witnesses for the skyline inequality; not a solver experiment.
count=0
for n in range(0,8):
    for first in product(*[range(i+1) for i in range(n)]):
        e=sum(i-first[i] for i in range(n));w=max([0]+[i-first[i] for i in range(n)])
        q=sum(max(0,j-max(first[i],first[j])) for i in range(n) for j in range(first[i],i))
        assert q<=e*max(w-1,0)//2
        count+=1
assert 128*((2**64-1-1)//128)<2**64-1
assert ((2**32-2)+1)**2<2**64
maintained=subprocess.check_output(['git','diff','--name-only',source,rev,'--','projects/chirality-piping/core','projects/chirality-piping/apps','projects/chirality-piping/schemas','projects/chirality-piping/fixtures','projects/chirality-piping/tests','projects/chirality-piping/validation'],env=env,text=True)
assert maintained==''
stage0=(r/'REVIEW_RV44/f2a_no_wrap_01/SOURCE_DERIVATION.md').read_bytes()
assert hashlib.sha256(stage0).hexdigest()=='e431fc0bb02e724aa1315b366f80fe8df260b6579869d245c500fe69a21ca4b5'
# Count semantics source reviewed in addition to the Stage 0 origins.
for path in ['projects/chirality-piping/core/solver/performance_harness/src/k6/w1/counts.rs','projects/chirality-piping/core/solver/performance_harness/src/bin/k6_observe/main.rs']:
    at(path,source)
result={'author_revision':subprocess.check_output(['git','rev-parse',rev],env=env,text=True).strip(),'payloads_checked':7,'packet_files':8,'count_origins_checked':len(inputs),'H_rows':len(h),'H_max':max(x['local_upper'] for x in h),'H_max_rows':[x for x in h if x['local_upper']==max(y['local_upper'] for y in h)],'VR_rows':len(v),'VR_max':max(x['local_upper'] for x in v),'VR_max_rows':[x for x in v if x['local_upper']==max(y['local_upper'] for y in v)],'VR_missing_count_rows':missing,'all_author_arithmetic_fields_match':True,'skyline_integer_shapes_checked':count,'maintained_diff_from_source':[],'stage0_unchanged':True,'scope':'Finite source/count integer arithmetic only; no full admission proof.'}
(out/'STAGE1_CHECKS.json').write_text(json.dumps(result,indent=2)+'\n')
(out/'ORIGINS_STAGE1.json').write_text(json.dumps(origins,indent=2)+'\n')
(out/'INDEPENDENT_COUNT_ROWS.json').write_text(json.dumps({'H':h,'VR':v,'VR_missing':missing},indent=2)+'\n')
print(json.dumps(result,indent=2))
