"""RV59 abstract exact controls only. No product import or model/solver execution."""
from fractions import Fraction as F
import json

def hull(a,b): return min(a[0],b[0]),max(a[1],b[1])
def distance(y,a): return max(abs(y-a[0]),abs(y-a[1]))
def quotient(a,d):
    assert 0<d[0]<=d[1]
    v=[x/y for x in a for y in d]
    return min(v),max(v)
def torsion(a,c,j):
    assert 0<c[0]<=c[1] and 0<j[0]<=j[1]
    v=[x*y/z for x in a for y in c for z in j]
    return min(v),max(v)
def serial(x):
    if isinstance(x,F): return str(x)
    if isinstance(x,(tuple,list)): return [serial(v) for v in x]
    if isinstance(x,dict): return {k:serial(v) for k,v in x.items()}
    return x
# A changed center has no containment implication, even if each branch is exact.
k=(F(1),F(1)); g=(F(2),F(2)); y=F(2)
assert distance(y,g)==0 and distance(y,hull(k,g))==1
# The old symmetric bridge was wider by construction; this is the precise lost premise.
x=F(3); rk=F(1,8); e=F(1,4)
old_k=(x-rk,x+rk); old_g=(x-rk-e,x+rk+e)
assert old_g[0]<=old_k[0]<=old_k[1]<=old_g[1]
# Completed ordinary recipes cover both meanings without cross-mixing operands.
represented=quotient((F(2),F(2)),(F(2),F(2)))
source=quotient((F(6),F(6)),(F(3),F(3)))
completed=hull(represented,source)
mixed=quotient((F(2),F(6)),(F(2),F(3)))
assert completed==(F(1),F(2)) and mixed==(F(2,3),F(3))
assert mixed[0]<completed[0] and completed[1]<mixed[1]
# Positive raw conversion commutes with hull. Its endpoint error is the maximum branch error.
intervals=[(F(a,4),F(b,4)) for a in range(-5,6) for b in range(a,6)]
scales=[F(1),F(1,1000),F(1000),F(1000000)]
checked=0
for a in intervals:
    for b in intervals:
        both=hull(a,b)
        for value in [F(-3),F(0),F(2,7)]:
            assert distance(value,both)==max(distance(value,a),distance(value,b))
            for scale in scales:
                convert=lambda z:(z[0]/scale,z[1]/scale)
                assert convert(both)==hull(convert(a),convert(b))
                assert distance(value,convert(both))==max(distance(value,convert(a)),distance(value,convert(b)))
                checked+=1
# Signed quotient and torsion enclosures retain negative and zero-crossing cases.
assert quotient((F(-3),F(2)),(F(2),F(4)))==(F(-3,2),F(1))
assert torsion((F(-3),F(2)),(F(1),F(2)),(F(2),F(4)))==(F(-3),F(2))
# Abstract exact-profile source-only contract is distinct: adding a K branch changes its predicate.
assert distance(F(2),g)==0 and distance(F(2),hull(k,g))>0
print(json.dumps(serial({
 'scope':'abstract Fraction controls; no product admission, execution, availability or implementation claim',
 'groups':6,'raw_hull_distance_cases':checked,
 'lost_inclusion':{'K':k,'source':g,'source_only_error':distance(y,g),'dual_error':distance(y,hull(k,g))},
 'separate_recipes':{'represented':represented,'source':source,'completed_hull':completed,'mixed_operand_hull':mixed},
 'result':'PASS'}),indent=2))
