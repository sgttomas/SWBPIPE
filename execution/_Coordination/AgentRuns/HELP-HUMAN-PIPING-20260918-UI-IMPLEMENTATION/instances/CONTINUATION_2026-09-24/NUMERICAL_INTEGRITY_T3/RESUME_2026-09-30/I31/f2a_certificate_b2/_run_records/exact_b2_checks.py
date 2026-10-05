#!/usr/bin/env python3
"""B2 finite exact abstract controls; no product import/model/solver execution.
sqrt_ref is an offline integer-rational oracle, not a proposed runtime format.
"""
from fractions import Fraction as F
from math import isqrt
import json

def p2(e): return F(1<<e) if e>=0 else F(1,1<<-e)
def sqrt_ref(q):
    assert q>=0
    if q==0: return F(0),F(0)
    e=q.numerator.bit_length()-q.denominator.bit_length()
    if q<p2(e): e-=1
    step=p2(e//2-128)
    scaled=q/(step*step)
    k=isqrt(scaled.numerator//scaled.denominator)
    lo=k*step; hi=lo if lo*lo==q else (k+1)*step
    assert lo*lo<=q<=hi*hi
    return lo,hi
def ab(I):
    lo,hi=I; assert lo<=hi
    return (F(0) if lo<=0<=hi else min(abs(lo),abs(hi))),max(abs(lo),abs(hi))
def hypot_box(Y,Z):
    ay,az=ab(Y),ab(Z)
    l2=ay[0]*ay[0]+az[0]*az[0]; u2=ay[1]*ay[1]+az[1]*az[1]
    return sqrt_ref(l2)[0],sqrt_ref(u2)[1]
def div_nonnegative(X,D):
    assert 0<=X[0]<=X[1] and 0<D[0]<=D[1]
    return X[0]/D[1],X[1]/D[0]
def add(X,Y): return X[0]+Y[0],X[1]+Y[1]
def sif(Y,Z,mod,i):
    assert i>0
    h=div_nonnegative(hypot_box(Y,Z),mod)
    return i*h[0],i*h[1]
def circular(N,Y,Z,A,mod):
    return add(div_nonnegative(ab(N),A),div_nonnegative(hypot_box(Y,Z),mod))
def open_sum(N,Y,Z,A,mod):
    return add(div_nonnegative(ab(N),A),div_nonnegative(add(ab(Y),ab(Z)),mod))
def maximum(X,Y): return max(X[0],Y[0]),max(X[1],Y[1])
def contains_norm(I,y,z,scale=F(1),offset=F(0)):
    assert scale>0 and I[0]<=I[1]
    q=y*y+z*z
    lower=(I[0]-offset)/scale; upper=(I[1]-offset)/scale
    assert upper>=0 and upper*upper>=q
    if lower>0: assert lower*lower<=q
def points(I): return [I[0],(I[0]+I[1])/2,I[1]]
records=[]
def record(name,**facts): records.append({"name":name,**facts})
def textq(q): return str(q.numerator)+"/"+str(q.denominator)

for I,want in [((F(-3),F(-1)),(F(1),F(3))),((F(2),F(5)),(F(2),F(5))),
               ((F(-2),F(3)),(F(0),F(3))),((F(0),F(0)),(F(0),F(0)))]:
    assert ab(I)==want
record("absolute_interval_sign_controls",checks=4)
roots=[F(0),F(1),F(2),F(25),F(1,9),p2(-2200),p2(2200),p2(-2148)]
for q in roots:
    l,u=sqrt_ref(q); assert l*l<=q<=u*u
assert sqrt_ref(25)==(5,5) and sqrt_ref(0)==(0,0)
assert sqrt_ref(p2(-2200))==(p2(-1100),p2(-1100))
record("directed_sqrt_reference",checks=len(roots),positive_tiny_preserved=True)

boxes=[((F(-2),F(3)),(F(-5),F(-4))),((F(-1),F(1)),(F(-1),F(1))),
       ((F(3),F(3)),(F(4),F(4))),((p2(-1100),p2(-1100)),(F(0),F(0)))]
npoints=0
for Y,Z in boxes:
    I=hypot_box(Y,Z)
    for y in points(Y):
        for z in points(Z): contains_norm(I,y,z); npoints+=1
assert hypot_box(*boxes[0])[0]==4
assert hypot_box(*boxes[2])==(5,5)
assert hypot_box(*boxes[3])==(p2(-1100),p2(-1100))
record("hypot_rectangle_enclosure",sample_points=npoints)

Y,Z=boxes[0]; mod=(F(2),F(3)); A=(F(4),F(5)); N=(F(-1),F(2)); i=F(7,3)
IS=sif(Y,Z,mod,i); IC=circular(N,Y,Z,A,mod); IO=open_sum(N,Y,Z,A,mod)
counts=[0,0,0]
for y in points(Y):
 for z in points(Z):
  for d in points(mod):
   contains_norm(IS,y,z,i/d); counts[0]+=1
   for n in points(N):
    for a in points(A):
     contains_norm(IC,y,z,F(1)/d,abs(n)/a); counts[1]+=1
     value=abs(n)/a+(abs(y)+abs(z))/d
     assert IO[0]<=value<=IO[1]; counts[2]+=1
record("sif_circular_open_endpoint_intervals",sif_points=counts[0],circular_points=counts[1],open_points=counts[2])

# Strict W1a source functionals: same signed N, affine moment pair at exact t.
# Endpoint norms are rational 0/5/13, so all comparisons below are rational squares.
cases=[((3,4),(-3,-4)),((0,0),(3,4)),((3,4),(5,12)),((-5,-12),(3,-4))]
count=0
for a,b in cases:
    r0=isqrt(a[0]*a[0]+a[1]*a[1]); r1=isqrt(b[0]*b[0]+b[1]*b[1])
    assert r0*r0==a[0]*a[0]+a[1]*a[1] and r1*r1==b[0]*b[0]+b[1]*b[1]
    n,A0,Z0=F(-2),F(3),F(5)
    circ_max=abs(n)/A0+F(max(r0,r1),1)/Z0
    open_max=abs(n)/A0+F(max(abs(a[0])+abs(a[1]),abs(b[0])+abs(b[1])),1)/Z0
    for ti in range(17):
        t=F(ti,16)
        y=(1-t)*a[0]+t*b[0]; z=(1-t)*a[1]+t*b[1]
        allowed=(circ_max-abs(n)/A0)*Z0
        assert y*y+z*z<=allowed*allowed
        assert abs(n)/A0+(abs(y)+abs(z))/Z0<=open_max
        # General convex chord bound for the norm, independent of max shortcut.
        chord=(1-t)*r0+t*r1
        assert y*y+z*z<=chord*chord
        count+=1
record("unloaded_affine_endpoint_maxima",interior_and_endpoint_points=count)

# Raw i-end moment has the opposite sign to its j-side cut.
Qi,Qj=F(3),F(-5); t=F(1,2)
proper=t*Qj+(t-1)*Qi
wrong=(1-t)*Qi+t*Qj
assert proper==-4 and wrong==-1
assert max(abs(-Qi),abs(Qj))==max(abs(Qi),abs(Qj))
record("end_sign_identity_independent_of_numeric_max",proper_midpoint=textq(proper),wrong_midpoint=textq(wrong),
       endpoint_absolute_max_cannot_detect_wrong_sign=True)

# Open producer identity: max(|a+b|,|a-b|)=|a|+b for b>=0.
count=0
for a in [F(-7),F(-1),F(0),F(2),F(9)]:
 for by in [F(-3),F(0),F(4)]:
  for bz in [F(-5),F(0),F(6)]:
   b=abs(by)+abs(bz)
   assert max(abs(a+b),abs(a-b))==abs(a)+abs(by)+abs(bz); count+=1
record("pressureless_open_formula_identity",triples=count)
assert max(abs(-2+2),abs(-2+2))!=abs(F(-2))+abs(F(2))
record("pressure_longitudinal_cancellation_is_outside_proof",comparison_preserved=True)

# A quadratic (member-loaded) moment invalidates endpoint reduction.
assert 4*F(0)*(1-F(0))==0 and 4*F(1)*(1-F(1))==0
assert 4*F(1,2)*(1-F(1,2))==1
record("quadratic_member_load_outside_domain",endpoint_max=0,interior_value=1)

# A certificate of supplied rounded controls does not certify source truth.
delta=p2(-53); source_max=F(0); supplied_bernstein_mid=delta/2
assert supplied_bernstein_mid>source_max
assert abs(supplied_bernstein_mid-source_max)>0 # fails unchanged b=0
record("supplied_coefficient_maximum_not_source_proof",source_max=0,
       supplied_polynomial_midpoint=textq(supplied_bernstein_mid),zero_bound_refuses=True)

I0=(F(9),F(11)); I1=(F(8),F(12)); IM=maximum(I0,I1)
assert IM==(9,12)
for q0 in points(I0):
 for q1 in points(I1): assert IM[0]<=max(q0,q1)<=IM[1]
record("maximum_of_endpoint_intervals",checks=9,lo=9,hi=12)
# Row A exact at 10; B centered 9 with radius2 has truth11. Published winner A.
a,b,truth_a,truth_b=F(10),F(9),F(10),F(11)
assert a>b and abs(a-truth_a)==0 and abs(b-truth_b)==2
assert abs(a-max(truth_a,truth_b))==1
record("row_alias_does_not_inherit_aggregate_radius",selected_row_radius=0,other_row_radius=2,
       aggregate_error=1)
rows=[("B",F(10),F(11)),("A",F(10),F(9))]
actual_winner=sorted(rows,key=lambda r:(-r[1],r[0]))[0]
truth_winner=max(rows,key=lambda r:r[2])
assert actual_winner[0]=="A" and truth_winner[0]=="B"
record("published_tie_is_not_unique_source_argmax",published_id="A",source_max_id="B")

# Zero input has exact nonlinear zero; absent or invalid data never gets this path.
assert hypot_box((F(0),F(0)),(F(0),F(0)))==(0,0)
assert circular((F(0),F(0)),(F(0),F(0)),(F(0),F(0)),A,mod)==(0,0)
assert open_sum((F(0),F(0)),(F(0),F(0)),(F(0),F(0)),A,mod)==(0,0)
bad=0
for fn,args in [(sqrt_ref,(F(-1),)),(div_nonnegative,((F(0),F(1)),(F(0),F(1)))),
                (sif,(Y,Z,mod,F(0)))]:
 try: fn(*args)
 except AssertionError: bad+=1
 else: raise AssertionError("invalid operand accepted")
assert bad==3
record("zero_and_invalid_operand_controls",exact_zero_checks=3,refusals=bad)

print(json.dumps({"scope":"finite exact abstract mathematics; no product reach or availability",
 "all_assertions_passed":True,"control_groups":len(records),"controls":records,
 "runtime_format_selected_here":False,
 "arithmetic_interface":"operation requirements only; I35 owns concrete format"},indent=2))

