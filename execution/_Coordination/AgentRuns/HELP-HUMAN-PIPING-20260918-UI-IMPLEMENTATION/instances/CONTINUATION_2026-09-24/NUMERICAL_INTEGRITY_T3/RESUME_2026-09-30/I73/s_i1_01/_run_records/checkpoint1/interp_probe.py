import math
from fractions import Fraction as F
rows=[(-1e10,1e20),(1.0,8000.0),(2.0,8000.0)]
def point(x):
    for a,r in rows:
        if a==x: return r
    for (a0,r0),(a1,r1) in zip(rows,rows[1:]):
        if a0<x<a1:
            return r0+(r1-r0)*((x-a0)/(a1-a0))
def exact(x):
    x=F(x)
    for (a0,r0),(a1,r1) in zip(rows,rows[1:]):
        a0,r0,a1,r1=map(F,(a0,r0,a1,r1))
        if a0<=x<=a1: return r0+(r1-r0)*((x-a0)/(a1-a0))
lo,hi=0.5,2.0
x=math.nextafter(1.0,0)
print("point at pred(1):",point(x)," exact:",float(exact(x)))
print("point at lo:",point(lo),"exact lo",float(exact(lo)))
print("point at hi:",point(hi),"row r1",8000.0)
nd=lambda v: math.nextafter(v,-math.inf); nu=lambda v: math.nextafter(v,math.inf)
vals=[point(lo),point(hi),8000.0]
print("D2 literal hull:",nd(min(vals)),nu(max(vals)))
