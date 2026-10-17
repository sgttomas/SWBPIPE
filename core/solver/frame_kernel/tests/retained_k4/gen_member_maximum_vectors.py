#!/usr/bin/env python3
"""Independent exact-polynomial / 2048-bit outward span-maximum reference.

No producer, Bernstein bounder, or test oracle is imported. Exact rational Sturm
sequences isolate every stationary or nonsmooth candidate to2080dyadic bits.
The objective is evaluated with2048-bit directed interval arithmetic. Squaring
the derivative equation may add candidates; it cannot remove a true maximum.
All inputs are exact binary64. This is test evidence, not a product equation
reference or admission law.
"""
from fractions import Fraction as F
from math import gcd, isqrt, lcm, sqrt
from pathlib import Path
import hashlib, struct, sys
HERE=Path(__file__).resolve().parent
PRECISION=2048
ROOT_BITS=2080

def trim(p):
    while p and p[-1]==0:p.pop()
    return p
def add(a,b):return trim([(a[i] if i<len(a) else 0)+(b[i] if i<len(b) else 0) for i in range(max(len(a),len(b)))])
def mul(a,b):
    c=[F(0)]*(len(a)+len(b)-1)
    for i,x in enumerate(a):
        for j,y in enumerate(b):c[i+j]+=x*y
    return trim(c)
def scale(a,b):return trim([x*b for x in a])
def derivative(a):return trim([i*a[i] for i in range(1,len(a))])
def divide(a,b):
    a=list(map(F,a));b=list(map(F,b));q=[F(0)]*max(0,len(a)-len(b)+1)
    while a and len(a)>=len(b):
        d=len(a)-len(b);v=a[-1]/b[-1];q[d]=v
        for i,x in enumerate(b):a[i+d]-=v*x
        trim(a)
    return trim(q),a
def primitive(a):
    if not a:return []
    den=lcm(*(F(x).denominator for x in a));p=[int(x*den) for x in a]
    g=gcd(*p);return [x//g for x in p]
def squarefree(p):
    a=p;b=derivative(p)
    while b:a,b=b,divide(a,b)[1]
    return primitive(divide(p,a)[0])
def sturm(p):
    seq=[primitive(p),primitive(derivative(p))]
    while seq[-1]:
        r=divide(seq[-2],seq[-1])[1]
        if not r:break
        seq.append(primitive(scale(r,-1)))
    return seq

def sign_at(p,n,k):
    den=1<<k;v=p[-1];power=den
    for x in reversed(p[:-1]):v=v*n+x*power;power*=den
    return (v>0)-(v<0)
def variation(seq,n,k):
    signs=[sign_at(p,n,k) for p in seq];signs=[s for s in signs if s]
    return sum(a!=b for a,b in zip(signs,signs[1:]))
class ExactRoot(Exception):
    def __init__(self,x):self.x=x

def roots(p):
    if len(p)<=1:return []
    p=squarefree(p);exact=[]
    # Remove endpoint roots explicitly: Sturm's open/closed convention then
    # has no endpoint ambiguity, including roots hit by a dyadic split.
    while True:
        for x in (F(0),F(1)):
            if sum(a*x**i for i,a in enumerate(p))==0:
                exact.append((x,x));p=primitive(divide(p,[-x,F(1)])[0]);break
        else:break
        if len(p)<=1:return exact
    while True:
        seq=sturm(p);out=[];pending=[(0,0,variation(seq,0,0)-variation(seq,1,0))]
        try:
            while pending:
                n,k,count=pending.pop()
                if count==0:continue
                if k>=ROOT_BITS:
                    assert count==1
                    out.append((F(n,1<<k),F(n+1,1<<k)));continue
                mid=2*n+1
                if sign_at(p,mid,k+1)==0:raise ExactRoot(F(mid,1<<(k+1)))
                left=variation(seq,n,k)-variation(seq,mid,k+1)
                pending.append((mid,k+1,count-left));pending.append((2*n,k+1,left))
            return exact+out
        except ExactRoot as hit:
            exact.append((hit.x,hit.x));p=primitive(divide(p,[-hit.x,F(1)])[0])
            if len(p)<=1:return exact

def two(e):return F(1<<e) if e>=0 else F(1,1<<-e)
def exponent(x):
    x=abs(x);e=x.numerator.bit_length()-x.denominator.bit_length()
    return e-int(x<two(e))
def directed(x,up):
    if not x:return F(0)
    h=two(exponent(x)-PRECISION+1);q,r=divmod(x/h,1)
    return (q+int(up and bool(r)))*h
def sqrt_directed(x,up):
    if not x:return F(0)
    h=two(exponent(x)//2-PRECISION+1);v=x/(h*h);q=isqrt(v.numerator//v.denominator)
    return (q+int(up and q*q!=v))*h
class I:
    def __init__(self,a,b=None):self.a=F(a);self.b=F(a if b is None else b)
    def __add__(self,o):
        if not isinstance(o,I):o=I(o)
        return I(directed(self.a+o.a,False),directed(self.b+o.b,True))
    __radd__=__add__
    def __mul__(self,o):
        if not isinstance(o,I):o=I(o)
        products=[x*y for x in (self.a,self.b) for y in(o.a,o.b)]
        return I(directed(min(products),False),directed(max(products),True))
    __rmul__=__mul__
    def square(self):
        lo=0 if self.a<=0<=self.b else min(self.a*self.a,self.b*self.b)
        return I(directed(lo,False),directed(max(self.a*self.a,self.b*self.b),True))
    def absolute(self):return I(0 if self.a<=0<=self.b else min(abs(self.a),abs(self.b)),max(abs(self.a),abs(self.b)))
    def sqrt(self):return I(sqrt_directed(self.a,False),sqrt_directed(self.b,True))
def evaluate(p,x):
    v=I(0)
    for a in reversed(p):v=v*x+a
    return v

def reference(ends,q):
    n=[ends[0][0],ends[1][0]-ends[0][0]]
    moments=[[ends[0][c],ends[1][c]-ends[0][c]+q[c-1],-q[c-1]] for c in(1,2)]
    norm2=add(mul(moments[0],moments[0]),mul(moments[1],moments[1]))
    d=derivative(norm2);stationary=add(mul(d,d),scale(norm2,-4*n[1]*n[1]))
    candidates=[(F(0),F(0)),(F(1),F(1))]+roots(stationary)+roots(norm2)
    if n[1] and 0< -n[0]/n[1]<1:candidates.append((-n[0]/n[1],-n[0]/n[1]))
    values=[]
    for a,b in candidates:
        x=I(a,b);axial=evaluate(n,x).absolute();my=evaluate(moments[0],x);mz=evaluate(moments[1],x)
        values.append(axial+(my.square()+mz.square()).sqrt())
    return max(v.a for v in values),max(v.b for v in values)
def dyad(x):
    if not x:return '+0@0'
    n=x.numerator;e=-(x.denominator.bit_length()-1);assert x.denominator==1<<-e
    while n%2==0:n//=2;e+=1
    return ('-' if n<0 else '+')+format(abs(n),'x')+'@'+str(e)
def bits(x):return '%016x'%struct.unpack('<Q',struct.pack('<d',float(x)))[0]
def reference_controls():
    assert roots([F(1,4),F(-1),F(1)])==[(F(1,2),F(1,2))]
    lo,hi=reference([[F(0)]*3,[F(0)]*3],[F(8),F(0)])
    assert lo==hi==2
    lo,hi=reference([[F(1),F(0),F(0)],[F(2),F(0),F(0)]],[F(7),F(0)])
    assert lo<=F(23,7)<=hi
    lo,hi=reference([[F(0),F(1),F(0)],[F(0),F(0),F(1)]],[F(1),F(1)])
    assert lo*lo<=F(9,8)<=hi*hi

def build():
    reference_controls()
    lines=['# independent exact Sturm roots2080 / outward evaluation2048; A=Z=1']
    seed=110080783
    def next_value():
        nonlocal seed
        seed=(6364136223846793005*seed+1)&((1<<64)-1)
        return F(((seed>>32)%41)-20,4)
    for case in range(200):
        while True:
            ends=[[next_value() for _ in range(3)] for _ in range(2)];q=[next_value(),next_value()]
            # All three bending coefficient vectors must span two dimensions.
            vectors=[ends[0][1:],ends[1][1:],q]
            if any(a[0]*b[1]!=a[1]*b[0] for a in vectors for b in vectors):break
        lo,hi=reference(ends,q)
        lines.append('case %03d %s %s %s'%(case,' '.join(bits(x) for row in ends for x in row)+' '+' '.join(bits(x) for x in q),dyad(lo),dyad(hi)))
    return '\n'.join(lines)+'\n'
def capacity():
    e=1e-5;t=0.4137;r=2500.;w=sqrt(8.*e);k=r*w*w*(1.+e)/2.
    ends=[[F(0),F(r-k*t*t),F(-r*w*t)],[F(0),F(r-k*(1.-t)*(1.-t)),F(r*w*(1.-t))]]
    q=[F(k),F(0)];lo,hi=reference(ends,q)
    return 'capacity e_1e-5 %s %s %s\n'%(' '.join(bits(x) for row in ends for x in row)+' '+' '.join(bits(x) for x in q),dyad(lo),dyad(hi))

if __name__=='__main__':
    reference_controls()
    outputs={'member_maximum_capacity.txt':capacity()}
    if '--only=capacity' not in sys.argv:outputs['member_maximum.txt']=build()
    sums=HERE/'SHA256SUMS';entries={line.split()[1]:line.split()[0] for line in sums.read_text().splitlines()}
    for name,value in outputs.items():
        path=HERE/name;digest=hashlib.sha256(value.encode()).hexdigest()
        if '--check' in sys.argv:
            if not path.exists() or path.read_text()!=value:raise SystemExit('DIFF '+name)
            if entries.get(name)!=digest:raise SystemExit('DIFF '+name+' checksum')
            print('OK '+name)
        else:
            path.write_text(value);entries[name]=digest;print('WROTE '+name)
    if '--check' not in sys.argv:sums.write_text(''.join(f'{entries[name]}  {name}\n' for name in sorted(entries)))
