"""Author arithmetic mirror checked against exact-Fraction continuous-box oracle.
No product output or rounded square root is used as the reference.
"""
import json, math, random
from fractions import Fraction as F
from pathlib import Path

def out(v, upper):
    if not math.isfinite(v) or v<0: raise ValueError('arithmetic')
    r=math.nextafter(v,math.inf if upper else 0)
    if not math.isfinite(r): raise ValueError('outward')
    return r

def corner(v,upper):
    m=max(v)
    if m==0: return 0.
    sq=[]
    for x in v:
        if x==0: sq.append(0.)
        elif x==m: sq.append(1.)
        else:
            r=out(x/m,upper);sq.append(out(r*r,upper))
    def add(a,b): return b if a==0 else a if b==0 else out(a+b,upper)
    s=add(add(sq[0],sq[1]),sq[2]);r=1. if s==1 else out(math.sqrt(s),upper)
    return m if r==1 else out(m*r,upper)

def evaluate(v,p):
    a=[(lo,hi) if lo>=0 else (-hi,-lo) if hi<=0 else (0.,max(-lo,hi)) for lo,hi in p]
    lo=corner([x[0] for x in a],False);hi=corner([x[1] for x in a],True)
    m=max(map(abs,v)); ratios=[0.,0.,0.] if m==0 else [x/m for x in v];q=0. if m==0 else m*math.sqrt(sum_order([x*x for x in ratios]))
    if not math.isfinite(q) or not lo<=q<=hi: raise ValueError('representative')
    e=0. if lo==q==hi else out(max(q-lo,hi-q),True)
    if hi==0:r=0.
    else:
        if min(lo,hi,q)<float.fromhex('0x1p-1022'): raise ValueError('publication')
        r=0. if e==0 else out(e/lo,True)
    if r>1e-9: raise ValueError('criterion')
    return q,lo,hi,e,r,a

def sum_order(s):return (s[0]+s[1])+s[2]

def check(v,p):
    q,lo,hi,e,r,a=evaluate(v,p)
    # Norm is monotone in absolute components; box extrema are exact squared sums.
    low2=sum(F(x[0])**2 for x in a);high2=sum(F(x[1])**2 for x in a)
    assert F(lo)**2<=low2 and F(hi)**2>=high2
    assert F(e)>=max(F(q)-F(lo),F(hi)-F(q))
    assert hi==0 or F(r)>=F(e)/F(lo)
    assert r<=1e-9
    return [q,lo,hi,e,r]

rng=random.Random(20260925);records=[];accepted=refused=0
controls=[[0.,0.,0.],[3.,4.,0.],[-3.,0.,-4.],[1e300,1e-300,0.],[1e-200,0.,0.],[math.nextafter(0.,1.),0.,0.],[1.7e308,1.7e308,0.]]
for _ in range(500):
    scale=10.**rng.uniform(-300,300)
    controls.append([scale*rng.uniform(-1,1) for _ in range(3)])
for i,v in enumerate(controls):
    width=0. if i<7 else rng.choice([0.,1e-13,1e-11,1e-7])
    p=[[x-abs(x)*width,x+abs(x)*width] for x in v]
    try: result=check(v,p);accepted+=1;records.append({'values':v,'intervals':p,'result':result})
    except ValueError as e:refused+=1;records.append({'values':v,'intervals':p,'refusal':str(e)})
output={'accepted':accepted,'refused':refused,'controls':records,'oracle':'exact Fraction squared continuous absolute-component boxes; no rounded sqrt reference','scope':'author mirror verification, not independent review or product Rust execution'}
Path(__file__).with_name('SUPPORT_NORM_REFERENCE.json').write_text(json.dumps(output,indent=2)+'\n')
print(json.dumps({k:output[k] for k in ['accepted','refused','oracle','scope']}))
