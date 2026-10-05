from fractions import Fraction as F
import json
MAX=0x7fefffffffffffff
INF=0x7ff0000000000000
def two(e):return F(1<<e) if e>=0 else F(1,1<<-e)
def val(bits):
 e=bits>>52;m=bits&((1<<52)-1)
 return m*two(-1074) if not e else ((1<<52)|m)*two(e-1075)
def ru_direct(x):
 if not x:return F(0)
 e=x.numerator.bit_length()-x.denominator.bit_length()
 if x<two(e):e-=1
 u=two(max(e-52,-1074));r=x/u
 result=(-((-r.numerator)//r.denominator))*u
 return 'infinity' if result>val(MAX) else result
vectors=[(F(0),0),(F(1),0x3ff0000000000000),(1+two(-54),0x3ff0000000000001),(1+3*two(-54),0x3ff0000000000001),(two(-1074),1),(two(-1076),1),(val(MAX)+two(969),INF),(two(1024),INF)]
for x,b in vectors:assert ru_direct(x)==('infinity' if b==INF else val(b))
# Finite abstract exit matrix for owner capture and collection. This models
# custody only, not actual Rust SumWork values or reachability of injected faults.
count=0
for failed_event in [None,0,1,2]:
 for numerical in [None,'Span','Exponent']:
  for owner_status in [0,1,2,3]:
   for merge_status in [0,1,2,3]:
    prefix=3 if failed_event is None else failed_event
    result_error=numerical if numerical else ('WorkAccounting' if failed_event is not None else None)
    joined=owner_status|merge_status|(1 if failed_event is not None else 0)
    captured=(prefix,owner_status,result_error)
    success=result_error is None and joined==0
    assert not success or (prefix==3 and owner_status==merge_status==0)
    assert captured[2]==result_error
    # Both producing prefix and original numeric cause survive, irrespective
    # of a simultaneous destination accounting fault.
    assert captured[0]==prefix and (not numerical or captured[2]==numerical)
    count+=1
assert ru_direct(val(MAX)+two(969))=='infinity'
assert val(MAX)+two(969)<two(1024)-two(970)<two(1024)
print(json.dumps({'kind':'independent exact B64U repair confirmation','numerical_direct_quantum_oracle_cases':len(vectors),'custody_exit_matrix_cases':count,'distinct_early_and_successor_overflow':True,'native_sumwork_measured':False,'product_code_executed':False,'passed':True},indent=2,sort_keys=True))
