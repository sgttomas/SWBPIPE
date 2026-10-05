#!/usr/bin/env python3
"""RV49 independent exact abstract controls; no product imports or runtime claim.
Proof is in frozen SOURCE_DERIVATION. This script was written separately from
B2 controls. Root bounds use fixed rational bisection, not author's isqrt oracle.
"""
from fractions import Fraction as Q
from itertools import product
import json

def root_bracket(x):
    assert x >= 0
    if x == 0: return Q(0), Q(0)
    # Normalize by exact powers of four, then fixed 128 bisections on [0,2].
    k = (x.numerator.bit_length()-x.denominator.bit_length())//2
    power = Q(2)**k
    q = x/(power*power)
    if q >= 4: k += 1; power *= 2; q /= 4
    assert 0 < q < 4
    l,h = Q(0),Q(2)
    for _ in range(128):
        m=(l+h)/2
        if m*m <= q: l=m
        else: h=m
    assert l*l <= q <= h*h
    return l*power,h*power

def absolute(a):
    l,h=a; assert l<=h
    return (Q(0) if l<=0<=h else min(abs(l),abs(h)),max(abs(l),abs(h)))

def norm(a,b):
    al,ah=absolute(a);bl,bh=absolute(b)
    return root_bracket(al*al+bl*bl)[0],root_bracket(ah*ah+bh*bh)[1]

def recipe(n,y,z,a,d,i=Q(1),mode='circular'):
    assert a[0]>0 and d[0]>0 and i>0
    nl,nh=absolute(n);yl,yh=absolute(y);zl,zh=absolute(z)
    if mode=='open': bl,bh=yl+zl,yh+zh
    else: bl,bh=norm(y,z)
    if mode=='sif': return i*bl/d[1],i*bh/d[0]
    return nl/a[1]+bl/d[1],nh/a[0]+bh/d[0]

def norm_in_interval(interval,x,y,offset,scale):
    l,h=interval;lo=(l-offset)/scale;hi=(h-offset)/scale;v=x*x+y*y
    assert scale>0 and hi>=0 and hi*hi>=v
    assert lo<=0 or lo*lo<=v

records=[]
boxes=[(Q(-7),Q(-2)),(Q(-1),Q(5)),(Q(0),Q(0)),(Q(2,3),Q(9,4))]
roots=[Q(0),Q(1,7),Q(2),Q(25),Q(2)**-2200,Q(2)**2200]
for x in roots:
    l,h=root_bracket(x);assert l*l<=x<=h*h
records.append({'group':'independent_bisection_root_bounds','count':len(roots)})
count=0
for nb,yb,zb in product(boxes,repeat=3):
    a=(Q(5,3),Q(8,3));d=(Q(7,5),Q(11,5));i=Q(11,8)
    intervals={m:recipe(nb,yb,zb,a,d,i,m) for m in ['sif','circular','open']}
    for n,y,z in product(*[(x[0],(x[0]+x[1])/2,x[1]) for x in [nb,yb,zb]]):
        for av,dv in product(a,d):
            norm_in_interval(intervals['sif'],y,z,Q(0),i/dv)
            norm_in_interval(intervals['circular'],y,z,abs(n)/av,Q(1)/dv)
            val=abs(n)/av+(abs(y)+abs(z))/dv
            assert intervals['open'][0]<=val<=intervals['open'][1]
            count+=1
records.append({'group':'independent_nonlinear_rectangle_containment','realizations':count,'recipes_per_realization':3})
# Exact norm endpoint vectors; compare interior squared norm with convex chord.
vectors=[(Q(3),Q(4),Q(5)),(Q(-5),Q(12),Q(13)),(Q(0),Q(0),Q(0)),(Q(8,17),Q(-15,17),Q(1))]
count=0
for v,w in product(vectors,repeat=2):
    assert v[0]**2+v[1]**2==v[2]**2
    for j in range(33):
        t=Q(j,32);x=(1-t)*v[0]+t*w[0];y=(1-t)*v[1]+t*w[1]
        chord=(1-t)*v[2]+t*w[2]
        assert x*x+y*y<=chord*chord<=max(v[2],w[2])**2
        assert abs(x)+abs(y)<=max(abs(v[0])+abs(v[1]),abs(w[0])+abs(w[1]))
        count+=1
records.append({'group':'affine_span_convexity','fraction_checks':count})
# The section i-end is negative Qi: same endpoint norms fail to detect wrong sign.
qi,qj,t=Q(7),Q(3),Q(1,4)
proper=t*qj+(t-1)*qi;wrong=(1-t)*qi+t*qj
assert proper==-Q(9,2) and wrong==6
assert max(abs(-qi),abs(qj))==max(abs(qi),abs(qj))
# Loaded quadratic is excluded even when endpoints vanish.
assert 4*t*(1-t)>0 and 4*Q(0)*(1-Q(0))==4*Q(1)*(1-Q(1))==0
records.append({'group':'sign_and_loaded_domain_discriminators','checks':2})
# Source interval maxima retain both lower and upper bounds, with unequal ends.
count=0
for a,b in product(boxes,repeat=2):
    l,h=max(a[0],b[0]),max(a[1],b[1])
    for x,y in product(a,b):assert l<=max(x,y)<=h;count+=1
records.append({'group':'max_interval_containment','checks':count})
# Raw MPa normalization can hide nonzero raw-unit discrepancy.
y=Q(3602879701896397,2**55);a=Q(10**6);n=Q(100000);truth=n
hn=abs(n-truth);hu=abs(y-truth/a)
assert hn==0 and hu>0 and abs(n-a*y)>0
assert 10**9*hu<abs(y)
records.append({'group':'raw_and_normalized_coordinates_are_distinct','normalized_error':str(hn),'raw_error':str(hu)})
# D2 selected-row alias does not acquire an aggregate radius or source argmax.
rows=[('a',Q(4),Q(4),Q(0)),('b',Q(3),Q(5),Q(2))]
assert rows[0][1]>rows[1][1] and abs(rows[0][1]-rows[0][2])==rows[0][3]
assert abs(rows[0][1]-max(r[2] for r in rows))>rows[0][3]
assert max(r[2]-r[3] for r in rows)<=max(r[2] for r in rows)<=max(r[2]+r[3] for r in rows)
records.append({'group':'headline_alias_aggregate_boundary','selected_radius':0,'aggregate_error':1})
# Product witness midpoint is not midpoint of the global-maximum certificate.
l,u,glob=Q(10),Q(12),Q(13)
assert l+(u-l)/2==11 and l+(glob-l)/2==Q(23,2)
# Rounded Bernstein controls can have interior curvature despite affine source.
c=(Q(0),Q(1,8),Q(0));t=Q(1,2)
poly=c[0]*(1-t)**2+2*c[1]*t*(1-t)+c[2]*t*t
assert poly==Q(1,16)>max(c[0],c[2])
records.append({'group':'coefficient_and_witness_limits','checks':2})
# Conditional schedule counts derive by explicit endpoint recipe aggregation.
s,mc,mo=7,11,13
counts={'squares':s*4+mc*2*4,'roots':s*2+mc*2*2,'products':s*2,
'divisions':s*2+mc*2*4+mo*2*4,'adds':s*2+mc*2*4+mo*2*4,
'max_comparisons':mc*2+mo*2,'abs_ranges':s*2+mc*2*3+mo*2*3}
assert counts==dict(squares=4*s+8*mc,roots=2*s+4*mc,products=2*s,
 divisions=2*s+8*mc+8*mo,adds=2*s+8*mc+8*mo,max_comparisons=2*mc+2*mo,abs_ranges=2*s+6*mc+6*mo)
records.append({'group':'conditional_recipe_operation_counts','counts':counts,'format_or_M1_qualification':False})
print(json.dumps({'all_assertions_passed':True,'scope':'exact abstract controls only','groups':records},indent=2))
