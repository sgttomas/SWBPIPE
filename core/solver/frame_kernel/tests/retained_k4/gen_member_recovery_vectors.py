#!/usr/bin/env python3
"""Independent 4352-bit outward reference for actual compiled recovery tests.

The target is the unrounded frame/local-load recovery from the exact binary64
leaves and displacement. It imports no producer, retained emulator or test
oracle. The interval model retains higher-order terms by directed rounding.
"""
from fractions import Fraction as F
from math import isqrt, nextafter, inf, ulp, isfinite
import hashlib, json, random, struct

def two(e): return F(1 << e) if e >= 0 else F(1, 1 << -e)
def exponent(x):
    x=abs(x); e=x.numerator.bit_length()-x.denominator.bit_length()
    return e-int(x < two(e))
def rnd(x,p,mode=0):
    x=F(x)
    if not x: return F(0)
    h=two(exponent(x)-p+1); q,r=divmod(x/h,1); q=int(q)
    if mode==1: q+=int(bool(r))
    elif mode==0: q+=int(r>F(1,2) or (r==F(1,2) and q%2))
    return q*h
def sqrtr(x,p,mode=0):
    assert x>=0
    if not x:return F(0)
    h=two(exponent(x)//2-p+1); z=x/(h*h)
    q=isqrt(z.numerator//z.denominator)
    if mode==1: q+=int(F(q*q)!=z)
    elif mode==0:q+=int(4*z>(2*q+1)**2 or (4*z==(2*q+1)**2 and q%2))
    return q*h

class I:
    R=4352
    def __init__(self,a,b=None):self.a=F(a);self.b=F(a if b is None else b)
    def __add__(self,o):
        o=iv(o);return I(rnd(self.a+o.a,I.R,-1),rnd(self.b+o.b,I.R,1))
    __radd__=__add__
    def __neg__(self):return I(-self.b,-self.a)
    def __sub__(self,o):return self+-iv(o)
    def __rsub__(self,o):return iv(o)+-self
    def __mul__(self,o):
        o=iv(o);a=[x*y for x in (self.a,self.b) for y in(o.a,o.b)]
        return I(rnd(min(a),I.R,-1),rnd(max(a),I.R,1))
    __rmul__=__mul__
    def __truediv__(self,o):
        o=iv(o);assert not o.a<=0<=o.b
        a=[x/y for x in(self.a,self.b) for y in(o.a,o.b)]
        return I(rnd(min(a),I.R,-1),rnd(max(a),I.R,1))
    def sqrt(self):
        assert self.a>=0
        return I(sqrtr(self.a,I.R,-1),sqrtr(self.b,I.R,1))
def iv(x):return x if isinstance(x,I) else I(x)
def dot(a,b):return sum((x*y for x,y in zip(a,b)),0)
def cross(a,b):return [a[j]*b[k]-a[k]*b[j] for j,k in ((1,2),(2,0),(0,1))]

def target(xi,xj,yr,loads,U):
    def normalize(v):
        n=dot(v,v).sqrt();return [a/n for a in v],n
    x,L=normalize([I(b-a) for a,b in zip(xi,xj)]);v=[I(a)-dot(yr,x)*b for a,b in zip(yr,x)]
    y,_=normalize(v);z,_=normalize(cross(x,y));axes=[x,y,z];inv=I(1)/L
    D=[I(c)/L for c in(6,10,14,22)]
    d=[dot(axes[k%3],U[3*(k//3):3*(k//3)+3]) for k in range(12)]
    e=[d[6]-d[0],d[9]-d[3],d[5]+inv*d[1]-inv*d[7],d[11]+inv*d[1]-inv*d[7],
       d[4]-inv*d[2]+inv*d[8],d[10]-inv*d[2]+inv*d[8]]
    Q=[D[0]*e[0],D[1]*e[1],D[2]*(4*e[2]+2*e[3]),D[2]*(2*e[2]+4*e[3]),
       D[3]*(4*e[4]+2*e[5]),D[3]*(2*e[4]+4*e[5])]
    w=[sum((dot(a,b) for b in loads),I(0)) for a in axes]
    wl=[a*L for a in w];wll=[a*L for a in wl];h=[a/12 for a in wll]
    vb=[-inv*Q[2]-inv*Q[3],inv*Q[4]+inv*Q[5]]
    ends=[-Q[0]-wl[0]/2,-vb[0]-wl[1]/2,-vb[1]-wl[2]/2,-Q[1],Q[4]+h[2],Q[2]-h[1],
          Q[0]-wl[0]/2,vb[0]-wl[1]/2,vb[1]-wl[2]/2,Q[1],Q[5]-h[2],Q[3]+h[1]]
    return ends,Q,wl,wll,h,L

from pathlib import Path
import sys
HERE=Path(__file__).resolve().parent
TS=[F(0),F(1,4),F(float(1/3)),F(1)-two(-52),F(1)]
def bits(x):return "%016x"%struct.unpack("<Q",struct.pack("<d",float(x)))[0]
def dyad(x):
    if not x:return "+0@0"
    n=x.numerator;d=x.denominator;assert d&(d-1)==0
    e=-(d.bit_length()-1)
    while n%2==0:n//=2;e+=1
    return ("-" if n<0 else "+")+format(abs(n),"x")+"@"+str(e)
def rows(xi,xj,yr,loads,U):
    te,tq,tl,tll,th,tL=target(xi,xj,yr,loads,U);out=te[:];ti=I(1)/tL
    for t in TS:
        if t in(0,1):
            start=0 if t==0 else 6;sgn=-1 if t==0 else 1
            out.extend([sgn*x for x in te[start:start+6]]);continue
        tv=[tq[0],-ti*tq[2]-ti*tq[3],ti*tq[4]+ti*tq[5]]
        out.extend([tv[c]+tl[c]/2-tl[c]*t for c in range(3)]+[tq[1],
            t*tq[5]+t*tq[4]-tq[4]-th[2]+tll[2]*t/2-tll[2]*t*t/2,
            t*tq[3]+t*tq[2]-tq[2]+th[1]-tll[1]*t/2+tll[1]*t*t/2])
    return out

def build():
    lines=[]
    for p in(128,256,512,1024):
      for kind in("aligned","skew","near_parallel","g_boundary","g_above","fixed","cancel","zero","none","empty"):
        xi=[F(0)]*3;yr=[F(0),F(1),F(0)];xj=[F(1),F(0),F(0)]
        U=[F((i%5)-2,16) for i in range(12)];loads=[[F(1),F(-2),F(3)]]
        if kind=="skew":xj=[F(1),F(2),F(3)]
        if kind in("near_parallel","g_boundary","g_above","fixed","cancel","zero","none","empty"):
            k=80 if kind not in("g_boundary","g_above") else p-17+(kind=="g_above")
            xj=[F(1),F(1),two(-k)];yr=[F(1),F(1),F(0)];U=[F(0)]*12;loads=[[F(1),F(0),F(0)]]
        if kind=="cancel":loads=loads+[[-x for x in loads[0]]]
        if kind=="zero":loads=[[F(0),F(0),F(0)]]
        target_loads=[] if kind in("none","empty") else loads
        lines.append("case %s_%d %d %s"%(kind,p,p," ".join(bits(x) for x in xi+xj+yr+U)))
        for w in loads:lines.append("load "+" ".join(bits(x) for x in w))
        for i,row in enumerate(rows(xi,xj,yr,target_loads,U)):
            lines.append("row %d %s %s"%(i,dyad(row.a),dyad(row.b)))
        lines.append("end")
    return "\n".join(lines)+"\n"
if __name__=="__main__":
    value=build();path=HERE/"member_recovery.txt"
    sums=HERE/"SHA256SUMS"
    entries={line.split()[1]:line.split()[0] for line in sums.read_text().splitlines()}
    digest=hashlib.sha256(value.encode()).hexdigest()
    if "--check" in sys.argv:
        if not path.exists() or path.read_text()!=value:raise SystemExit("DIFF member_recovery.txt")
        if entries.get(path.name)!=digest:raise SystemExit("DIFF member_recovery.txt checksum")
        print("OK member_recovery.txt")
    else:
        path.write_text(value)
        entries[path.name]=digest
        sums.write_text("".join(f"{entries[name]}  {name}\n" for name in sorted(entries)))
