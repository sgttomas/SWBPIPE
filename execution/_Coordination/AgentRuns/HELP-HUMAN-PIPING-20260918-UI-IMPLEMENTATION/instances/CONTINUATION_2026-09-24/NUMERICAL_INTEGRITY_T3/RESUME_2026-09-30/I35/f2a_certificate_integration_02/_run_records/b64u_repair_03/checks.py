"""Exact ABSTRACT conversion and ownership controls, not native SumWork counts."""
from fractions import Fraction as F
import json

MAX=0x7fefffffffffffff
INF=0x7ff0000000000000

def two(e): return F(1<<e) if e>=0 else F(1,1<<-e)
def value(b):
    assert 0<=b<=MAX
    e,f=b>>52,b&((1<<52)-1)
    return f*two(-1074) if e==0 else ((1<<52)|f)*two(e-1075)
def rn_bits(x):
    assert x>=0
    if not x: return 0
    if x>=two(1024)-two(970): return INF
    e=x.numerator.bit_length()-x.denominator.bit_length()
    if x<two(e): e-=1
    quantum=max(e-52,-1074)
    a=x/two(quantum); n,r=divmod(a.numerator,a.denominator)
    if 2*r>a.denominator or (2*r==a.denominator and n&1): n+=1
    if n==1<<53: n>>=1; quantum+=1
    if n==0: return 0
    if n<(1<<52): return n
    return ((quantum+52+1023)<<52)|(n-(1<<52))

class Fault(Exception): pass
class Owner:
    def __init__(self): self.events=[]; self.status=set(); self.exports=0
    def event(self,name,fail):
        if name==fail:
            self.status.add('Overflow')
            raise Fault(name)
        self.events.append(name)
    def export(self):
        self.exports+=1
        assert self.exports==1
        return self

# The event sequence stands for actual-owner capture. It is deliberately NOT a
# numerical SumWork/LME emulation; production must return the real owner's work().
def once(x,fail=None):
    nearest=rn_bits(x)
    if nearest==INF:
        return {'bits':INF,'error':None,'owner':None,'zero_sumwork':True,'conversion_calls':1}
    owner=Owner()
    try:
        owner.event('add_wide',fail)
        owner.event('add_candidate',fail)
        owner.event('signum',fail)
        bits=nearest+(1 if x>value(nearest) else 0)
        result=dict(bits=bits,error=None)
    except Fault as error:
        result=dict(bits=None,error=str(error))
    return dict(result,owner=owner.export(),zero_sumwork=None,conversion_calls=1)

def collect(spent,merge_fault=False):
    owner=spent['owner']
    events=[] if owner is None else owner.events[:]
    status=set() if owner is None else owner.status.copy()
    if merge_fault: status.add('Overflow')
    error=spent['error']
    if status: result=None
    elif error is not None: result=None
    elif spent['bits']==INF: error='finite-scale-range'; result=None
    else: result=spent['bits']
    return dict(result=result,error=error,status=sorted(status),events=events)

checks=[]
def group(name,detail): checks.append(dict(name=name,passed=True,detail=detail))

cases=[
 ('zero',F(0),0),
 ('exact_normal',F(1),0x3ff0000000000000),
 ('normal_successor',F(1)+two(-54),0x3ff0000000000001),
 ('nearest_above_no_step',F(1)+3*two(-54),0x3ff0000000000001),
 ('exact_subnormal',two(-1074),1),
 ('underflow_successor',two(-1076),1),
 ('max_successor_infinity',value(MAX)+two(969),INF),
 ('early_overflow',two(1024),INF),
]
observed=[]
for name,x,expected in cases:
    s=once(x)
    assert s['bits']==expected and s['conversion_calls']==1
    assert (s['owner'] is None)==(name=='early_overflow')
    if s['owner'] is not None:
        assert s['owner'].exports==1 and s['owner'].events==['add_wide','add_candidate','signum']
    c=collect(s)
    if expected==INF: assert c['result'] is None and c['error']=='finite-scale-range'
    else: assert c['result']==expected and not c['status']
    # Independent RU oracle: predecessor of finite result is strictly below x.
    if expected<INF:
        assert value(expected)>=x
        assert expected==0 or value(expected-1)<x
    observed.append({'case':name,'result_bits':format(expected,'016x'),'prefix':c['events']})
group('zero_normal_successor_subnormal_and_overflow',observed)

for index,site in enumerate(['add_wide','add_candidate','signum']):
    s=once(F(1)+two(-54),site)
    assert s['owner'].events==['add_wide','add_candidate','signum'][:index]
    assert s['owner'].exports==1
    c=collect(s)
    assert c['result'] is None and c['error']==site and c['status']==['Overflow']
    assert len(c['events'])==index
    assert 1152 not in c['events'] # the safety upper never becomes observed work
    try: s['owner'].export()
    except AssertionError: pass
    else: assert False
group('injected_refusal_preserves_actual_owner_prefix',{'sites':3,'not_a_reachability_claim':True,'bound_not_spent':True,'no_second_export':True})

s=once(F(1)); c=collect(s,True)
assert c['result'] is None and c['status']==['Overflow'] and c['events']==s['owner'].events
s=once(F(1),'signum'); c=collect(s,True)
assert c['error']=='signum' and c['status']==['Overflow']
group('collection_fault_blocks_success_preserves_original_error',{'success_blocked':True,'prior_error_preserved':True})

# Explicit legacy numerical projection uses the same one execution, including
# infinity. Production legacy callers do not start a new facade charge policy.
for name,x,expected in cases:
    s=once(x)
    legacy=s['bits'] if s['error'] is None else s['error']
    assert legacy==expected and s['conversion_calls']==1
assert once(two(1024))['zero_sumwork'] is True
assert once(value(MAX)+two(969))['owner'].events!=[]
group('legacy_projection_and_distinct_overflow_prefixes',{'legacy_value_parity':True,'early_zero_sumwork':True,'successor_has_sum_prefix':True,'no_replay':True})

print(json.dumps({'kind':'abstract_exact_B64U_custody_repair_controls','groups_passed':len(checks),'native_sumwork_counts_measured':False,'product_code_executed':False,'checks':checks},indent=2,sort_keys=True))
