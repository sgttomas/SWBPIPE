#!/usr/bin/env python3
"""RV53 independent fixture verification. Does not import/run author generator.
Sqrt endpoints are verified by exact squared inequalities/adjacency; huge
exponents stay symbolic. Binary64 operations use a distinct integer-in-h-units
rounder, then exact endpoint/predecessor inequalities.
"""
from pathlib import Path
import json,re,sys
P=1024
TOP=1<<1023
MAX=0x7fefffffffffffff
SIGN=1<<63

def token(t):
    h,e=t[1:].split('p')
    return (-1 if t[0]=='-' else 1)*int(h.ljust(256,'0'),16), int(e)-1023

def cmp(x,y):
    a,e=x;b,f=y
    assert a>=0 and b>=0
    if not a or not b: return (a>b)-(a<b)
    ax=a.bit_length()+e; bx=b.bit_length()+f
    if ax!=bx: return (ax>bx)-(ax<bx)
    d=e-f
    assert abs(d)<=5000
    if d>=0:a<<=d
    else:b<<=-d
    return (a>b)-(a<b)

def sq(x): return x[0]*x[0],2*x[1]
def add(x,y):
    m=min(x[1],y[1]); assert max(x[1],y[1])-m<=5000
    return (x[0]<<(x[1]-m))+(y[0]<<(y[1]-m)),m
def successor(x):
    m,e=x
    return (TOP,e+1) if m==(1<<1024)-1 else (m+1,e)

# Exact nonnegative finite binary64 value in units of h=2^-1074.
def units(bits):
    bits &= SIGN-1
    exp=bits>>52; frac=bits&((1<<52)-1)
    assert exp<2047
    return frac if exp==0 else ((1<<52)+frac)<<(exp-1)

# Exact positive integer-h-unit value -> binary64 encoding; no floating point.
def encode_int(n):
    if n<1<<52:return n
    shift=max(n.bit_length()-53,0)
    assert n % (1<<shift)==0
    return ((shift+1)<<52)+(n>>shift)-(1<<52)

# Round n / 2^denom_shift h-units to nearest even binary64.
# Derive output quantum from integer bit length, then perform one quotient/remainder.
def nearest(n,denom_shift=0):
    if n==0:return 0
    quantum=max(n.bit_length()-1-denom_shift-52,0)
    d=1<<(denom_shift+quantum)
    q,r=divmod(n,d)
    q+= (2*r>d or (2*r==d and q&1))
    result=encode_int(q<<quantum)
    # Independent Voronoi-cell check against exact predecessor/successor midpoints.
    v=units(result)
    if result>0:
        low=units(result-1)+v
        assert 2*n>=low*(1<<denom_shift)
        if 2*n==low*(1<<denom_shift): assert result%2==0
    if result<MAX:
        high=v+units(result+1)
        assert 2*n<=high*(1<<denom_shift)
        if 2*n==high*(1<<denom_shift): assert result%2==0
    return result

def component(bits,power):
    n=units(bits)
    near=nearest(n,power)
    back=nearest(units(near)<<power)
    return near+(units(back)<n)

def upward(n):
    # Generate direct integer quotient ceiling, not a floating bit search.
    shift=max(n.bit_length()-53,0)
    q=(n+(1<<shift)-1)>>shift
    return encode_int(q<<shift)

def check_sqrt(row):
    name,a,down,up,near=row
    aa,lo,hi,nn=map(token,(a,down,up,near))
    assert cmp(sq(lo),aa)<=0, name
    assert cmp(aa,sq(hi))<=0, name
    if cmp(lo,hi)==0:
        assert cmp(sq(lo),aa)==0 and nn==lo,name
    else:
        assert cmp(sq(lo),aa)<0 and cmp(aa,sq(hi))<0,name
        assert successor(lo)==hi,name
        mid=add(lo,hi);mid=(mid[0],mid[1]-1)
        side=cmp(aa,sq(mid))
        expected=hi if side>0 or (side==0 and lo[0]&1) else lo
        assert nn==expected,name
    return name

def check_small(row):
    name,*numbers=row
    value,scale,b,r,answer=map(lambda x:int(x,16),numbers)
    assert 0<scale<0x0230000000000000 and (value&(SIGN-1))<=MAX
    assert component(scale,64)==b,name
    assert component(value,53)==r,name
    exact=units(b)+units(r)+1
    assert upward(exact)==answer,name
    assert units(answer-1)<exact<=units(answer),name
    return dict(name=name,exact_h_units_bits=exact.bit_length(),
                final_is_exact=units(answer)==exact,
                final_midpoint=2*exact==units(answer-1)+units(answer))

src=Path(sys.argv[1])
text=src.read_text()
sqrt_text,small_text=text.split('pub const SMALL:')
sqrt_rows=re.findall(r'\("([^"]+)", "([^"]+)", "([^"]+)", "([^"]+)", "([^"]+)"\)',sqrt_text)
small_rows=re.findall(r'\(\s*"([^"]+)",\s*(0x[0-9a-f]+),\s*(0x[0-9a-f]+),\s*(0x[0-9a-f]+),\s*(0x[0-9a-f]+),\s*(0x[0-9a-f]+),\s*\)',small_text)
assert len(sqrt_rows)==14 and len(small_rows)==18,(len(sqrt_rows),len(small_rows))
sqrt_names=[check_sqrt(row) for row in sqrt_rows]
small_checks=[check_small(row) for row in small_rows]
# Mathematical binary-search bound over the entire admitted finite bit range:
# each midpoint leaves width <= ceil(old/2), therefore <=1 after 63.
width=MAX
widths=[width]
for _ in range(63):
    width=(width+1)//2
    widths.append(width)
assert width==1 and MAX<1<<63
# b0+h at scale=h and value=16 equals 2h = half of the p1024 ulp
# at r=2^-49. Nearest p1024 chooses exact r (even), but RU64 is r+1.
r=units(0x3ce0000000000000)
assert r==1<<1025 and component(1,64)==1
assert upward(r+2)==0x3ce0000000000001
# Nonzero p1024 sqrt exact midpoint cannot occur: midpoint significand is odd,
# and its squared significant integer has >1024 bits, so cannot be a p1024 input.
print(json.dumps(dict(sqrt_count=len(sqrt_names),sqrt=sqrt_names,
    small_count=len(small_checks),small=small_checks,
    search=dict(initial_width=MAX,bisections=63,final_width=width,total_comparisons_max=64),
    double_rounding='r=2^-49; tail=2^-1073; p1024 half-ulp lost to even; RU64 successor required',
    conclusion='All checked exact inequalities and stored bits agree'),indent=2))

