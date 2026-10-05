"""Abstract bit/exact controls only; no product import, builder, matrix or solve."""
from fractions import Fraction as F
import struct,json,math

def value(bits): return struct.unpack('>d',bits.to_bytes(8,'big'))[0]
def bits(x): return int.from_bytes(struct.pack('>d',x),'big')
MAX=0x7fefffffffffffff

def nearest(x,root=False):
    if not root and x<0: return -nearest(-x)
    if x==0: return 0.0
    maximum=F(value(MAX))
    if not root and x>maximum:
        return math.inf if x>=maximum+F(2)**970 else value(MAX)
    lo,hi=0,MAX
    while lo<hi:
        m=(lo+hi+1)//2; f=F(value(m)); q=f*f if root else f
        if q<=x: lo=m
        else: hi=m-1
    if lo==MAX:return value(lo)
    a,b=F(value(lo)),F(value(lo+1)); mid=(a+b)/2; boundary=mid*mid if root else mid
    return value(lo if x<boundary or (x==boundary and lo%2==0) else lo+1)

def rn_mul(a,b):return nearest(F(a)*F(b))
def rn_div(a,b):return nearest(F(a)/F(b))
def norm(v):
    p=[rn_mul(x,x) for x in v]
    s=nearest(F(p[0])+F(p[1])); s=nearest(F(s)+F(p[2]))
    return nearest(F(s),root=True)
def length(i,j):
    d=[nearest(F(b)-F(a)) for a,b in zip(i,j)]
    m=norm(d)
    if m<=1e-12:return {'refusal':'degenerate length','ops':9}
    inv=rn_div(1.,m); normalized=[rn_mul(x,inv) for x in d]
    result=norm(d)
    assert all(math.isfinite(x) for x in normalized)
    return {'L':result,'ops':19}
def normal(x):return math.isfinite(x) and abs(x)>=value(0x0010000000000000)
def coefficient(e,a,l):
    p=rn_mul(e,a)
    if not normal(p):return {'refusal':'product NumericalRange','ops':1,'intermediate':p.hex()}
    k=rn_div(p,l)
    if not normal(k):return {'refusal':'quotient NumericalRange','ops':2,'intermediate':k.hex()}
    return {'value':k,'ops':2}
# Exact reference validates the left-associated norm and exposes reassociation.
d=float.fromhex('0x1.6c00000000000p-27')
v=[1.,d,d]; L=length([0.,0.,0.],v)
ordinary=math.sqrt((1.+d*d)+d*d)
wrong=math.sqrt(1.+(d*d+d*d))
assert L['L']==ordinary and ordinary!=wrong and L['ops']==19
# Success matches two separately rounded operations; reassociation changes one bit.
e,a,l=map(float.fromhex,['0x1.9863000000000p+12','0x1.5085000000000p+11','0x1.ad85800000000p+9'])
k=coefficient(e,a,l); assert k['value']==(e*a)/l and k['value']!=e*(a/l)
# Positive coefficient range refusal is per operation, not final-only.
over=coefficient(value(MAX),2.,2.); under=coefficient(value(0x0010000000000000),.5,.5)
quot=coefficient(value(0x0010000000000000),1.,2.)
assert over['refusal'].startswith('product') and under['refusal'].startswith('product')
assert quot['refusal'].startswith('quotient')
assert length([0.,0.,0.],[1e-13,0.,0.])=={'refusal':'degenerate length','ops':9}
assert length([0.,0.,0.],[1.,0.,0.])=={'L':1.,'ops':19}
# Operational G5a uses normalized final rows and includes InputDerived values.
c=value(0x3ff0000000001000)
def lb(n,s,k):
    threshold=rn_mul(2.**-59,s)
    if n<=threshold:return 0.
    margin=rn_mul(2.**-60,s)
    return rn_mul(k,nearest(F(n)-F(margin)))
normalized=rn_div(1.,1000.); good=lb(normalized,normalized,1000.); wrong_raw=lb(1.,normalized,1000.)
assert good<=rn_mul(1.,c)<wrong_raw
assert lb(1.,1.,1.)>lb(0.,1.,1.) # omitted InputDerived changes the required lower bound
assert lb(0.,0.,1.)==0.
print(json.dumps({'scope':'abstract exact/bit controls; no product execution or historical-field capture',
 'result':'PASS','groups':5,
 'length':{'source_order':ordinary.hex(),'reassociated':wrong.hex(),'logical_source_operations':19},
 'coefficient':{'source_order':k['value'].hex(),'reassociated':(e*(a/l)).hex()},
 'range_controls':[over,under,quot],
 'G5a_normalized_LB':good.hex(),'G5a_wrong_raw_LB':wrong_raw.hex(),
 'successful_operand_sequence_operations':23},indent=2))
