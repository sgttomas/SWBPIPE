"""Independent rational checks of recorded/helper-mirror outputs; no Rust execution.

The author's arithmetic mirror is a subject under test, never the oracle. The
oracle below bounds the complete continuous projection box algebraically using
exact binary64 Fractions; it does not sample points or use rounded sqrt/hypot.
"""
from pathlib import Path
from fractions import Fraction as F
import hashlib
import json
import math
import random
import sys

HERE = Path(__file__).resolve().parent
DRAFT = HERE.parent
EXPECTED = 'abc3b1766debcefc2bdb7e3d7f0cd4720642049a7d5b2735a3529a65269b66e6'
def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()
assert sha(DRAFT/'endpoint_maximum.rs') == EXPECTED
subject = {'__name__': 'arithmetic_subject'}
exec(compile((DRAFT/'check_reference.py').read_text(), str(DRAFT/'check_reference.py'), 'exec'), subject)
assertions = 0
def require(condition, label):
    global assertions
    assertions += 1
    assert condition, label

def abs_range(projection):
    _, low, high = map(F, projection)
    return (F(0) if low <= 0 <= high else min(abs(low), abs(high)), max(abs(low),abs(high)))
def exact_stress_components(actions, area, z, upper):
    index = int(upper)
    n,y,zz = [abs_range(action)[index] for action in actions]
    return n/F(area), (y*y+zz*zz)/(F(z)*F(z))
def lower_valid(bound, additive, radicand):
    difference=F(bound)-additive
    return difference<=0 or difference*difference<=radicand
def upper_valid(bound, additive, radicand):
    difference=F(bound)-additive
    return difference>=0 and difference*difference>=radicand

def audit(name, area, z, ends, result):
    # A lower corner minimizes |N| and both moment magnitudes independently;
    # an upper corner maximizes them. These algebraic tests cover every real
    # point in each continuous interval box, including interior sign crossings.
    for end, estimate in zip(ends,result['endpoints']):
        q,bounds=estimate
        require(lower_valid(bounds[0],*exact_stress_components(end,area,z,False)), name+' endpoint lower')
        require(upper_valid(bounds[1],*exact_stress_components(end,area,z,True)), name+' endpoint upper')
        require(bounds[0]<=q<=bounds[1],name+' representative in enclosure')
    lo,hi=map(F,result['interval_pa']);q=F(result['value_pa'])
    require(lo==max(F(e[1][0]) for e in result['endpoints']),name+' global lower')
    require(hi==max(F(e[1][1]) for e in result['endpoints']),name+' global upper')
    require(q==max(F(e[0]) for e in result['endpoints']),name+' representative maximum')
    error=F(result['absolute_error_bound_pa']); relative=F(result['relative_error_bound'])
    require(error>=max(q-lo,hi-q),name+' exact absolute error')
    if hi:
        require(lo>0 and relative>=error/lo,name+' exact composed relative error')
        require(relative<=F(1e-9),name+' protected criterion')
        require(all(float(x)>=sys.float_info.min for x in [q,lo,hi]),name+' normal publication')
    else:
        require(q==lo==error==relative==0,name+' proved exact zero convention')
    loc=result['locations'];kind=loc['kind']
    singleton=lambda a:a[1]==a[2]
    equal=lambda c:singleton(ends[0][c]) and singleton(ends[1][c]) and F(ends[0][c][1])==F(ends[1][c][1])
    if kind=='WholeSpanConstant':
        require(equal(1) and equal(2),name+' constant bending warrant')
    elif kind=='StrictEndpoint':
        winner=0 if loc['endpoint']=='I' else 1
        require(result['endpoints'][winner][1][0]>result['endpoints'][1-winner][1][1],name+' strict separation')
        require(result['witness']==loc['endpoint'],name+' strict witness correspondence')
    else:
        if loc['exact_tie_proven']:
            require(all(singleton(e[c]) for e in ends for c in [1,2]),name+' tie exactness')
            vectors=[[F(e[c][1]) for c in [1,2]] for e in ends]
            require(sum(x*x for x in vectors[0])==sum(x*x for x in vectors[1]) and vectors[0]!=vectors[1],name+' exact nonconstant equal squared norm')
        if not loc['interior_equal_possible']:
            require(any(F(ends[0][c][2])<F(ends[1][c][1]) or F(ends[1][c][2])<F(ends[0][c][1]) for c in [1,2]),name+' nonzero slope warrant')

record=json.loads((DRAFT/'REFERENCE_RESULTS.json').read_text())
recorded_pass=0;recorded_refusal=0
for case in record['cases']:
    inp=case['inputs']
    try:
        replayed=subject['maximum'](inp['area'],inp['section_modulus'],inp['endpoint_projections_value_lo_hi'])
    except subject['Refusal'] as error:
        require(str(error)==case.get('expected_refusal'),case['name']+' observed refusal')
        recorded_refusal+=1
        continue
    if 'result' in case:
        require(json.loads(json.dumps(replayed))==case['result'],case['name']+' retained mirror result')
        audit(case['name'],inp['area'],inp['section_modulus'],inp['endpoint_projections_value_lo_hi'],case['result']);recorded_pass+=1
    else:
        raise AssertionError(case['name']+' expected refusal was accepted')

def exact(n,y,z):return [(x,x,x) for x in [n,y,z]]
tiny=math.ulp(0.0)
special=[
    ('same rounded norm unequal exact norm',1.,1.,[exact(0.,1.,0.),exact(0.,1.,tiny)]),
    ('equal norms unequal component magnitudes',1.,1.,[exact(0.,3.,4.),exact(0.,0.,5.)]),
    ('same ray affine nonconstant',1.,1.,[exact(2.,3.,4.),exact(2.,6.,8.)]),
    ('dominating axial hides norm difference',1.,1.,[exact(1e300,1.,0.),exact(1e300,1.,tiny)]),
    ('uncertain sign-crossing small axial',1.,1.,[[(0.,-tiny,tiny),(3.,3.,3.),(4.,4.,4.)]]*2),
    ('subnormal bending scales normal',1.,tiny,[exact(0.,tiny,tiny)]*2),
    ('maximum identities',sys.float_info.max,sys.float_info.max,[exact(sys.float_info.max,sys.float_info.max,0.)]*2),
    ('normal boundary exact identity',1.,1.,[exact(sys.float_info.min,0.,0.)]*2),
    ('normal boundary outward conservative refusal',1.5,1.,[exact(sys.float_info.min*1.5,0.,0.)]*2),
]
results=[]
def observe(name,area,z,ends):
    try:
        result=subject['maximum'](area,z,ends)
    except subject['Refusal'] as error:
        results.append({'name':name,'outcome':'refused','reason':str(error)})
        return None
    audit(name,area,z,ends,result)
    results.append({'name':name,'outcome':'bounded','value':result['value_pa'],'interval':result['interval_pa'],'locations':result['locations']})
    return result
for case in special:
    result=observe(*case)
    if case[0] in ['same rounded norm unequal exact norm','dominating axial hides norm difference']:
        require(result is not None and result['locations']=={'kind':'EndpointCandidates','exact_tie_proven':False,'interior_equal_possible':False},case[0]+' never infer rounded tie')
    if case[0]=='equal norms unequal component magnitudes':
        require(result is not None and result['locations']['kind']=='EndpointCandidates' and not result['locations']['exact_tie_proven'],case[0]+' conservative tie completeness')

rng=random.Random(202609250701)
for index in range(320):
    exponent=rng.randint(-1000,1000)
    scale=math.ldexp(1.,exponent)
    area=math.ldexp(rng.uniform(1.,2.),rng.randint(-1022,1022))
    z=math.ldexp(rng.uniform(1.,2.),rng.randint(-1022,1022))
    n=rng.uniform(-2.,2.)*scale
    ends=[]
    for end in range(2):
        actions=[]
        for value in [n,rng.uniform(-2.,2.)*scale,rng.uniform(-2.,2.)*scale]:
            delta=abs(value)*[0.,1e-13,3e-10,2e-9][index%4]
            actions.append((value,value-delta,value+delta))
        ends.append(actions)
    observe('spread '+str(index),area,z,ends)

final=[]
for item in json.loads((DRAFT/'FINAL_HASHES.json').read_text())['files']:
    digest=sha(DRAFT/item['path']);require(digest==item['sha256'],'frozen author '+item['path']);final.append({'path':item['path'],'sha256':digest})
origin_comparison=json.loads((HERE/'ORIGIN_COMPARISON.json').read_text())
for item in origin_comparison:
    require(sha(item['path'])==item['current_sha256'],'origin unchanged '+item['path'])
report={'actor':'/root/physics_resume/joined_producer_review','parent':'/root/physics_resume','mechanism':'delegated-harness-native TASK Type2','candidate_sha256':EXPECTED,'oracle':'Exact Fraction full continuous component-box bounds and exact squared-norm/location/error comparisons; no floating sqrt oracle','subject':'Read-only author Python mirror checked against inspected Rust statements; this is not independent Rust execution','recorded_successful_cases_checked':recorded_pass,'recorded_expected_refusals':recorded_refusal,'additional_cases':len(results),'additional_successes':sum(x['outcome']=='bounded' for x in results),'additional_refusals':sum(x['outcome']=='refused' for x in results),'random_seed':202609250701,'assertions':assertions,'results':results,'final_frozen_hashes':final}
(HERE/'VERIFICATION.json').write_text(json.dumps(report,indent=2,allow_nan=False)+'\n')
print(json.dumps({k:v for k,v in report.items() if k not in ['results','final_frozen_hashes']}))
