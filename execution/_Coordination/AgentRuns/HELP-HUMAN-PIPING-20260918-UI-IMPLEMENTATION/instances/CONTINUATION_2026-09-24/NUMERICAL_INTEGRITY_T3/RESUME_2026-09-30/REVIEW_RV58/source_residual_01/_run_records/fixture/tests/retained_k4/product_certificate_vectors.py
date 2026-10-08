#!/usr/bin/env python3
"""I41 exact standard-library oracle. No product arithmetic imports or execution.
Bounded binary64 Fractions and 128-term Machin proof; no exponent-sized input.
Run with --write to regenerate only this new fixture, or default to verify it.
"""
from fractions import Fraction as F
from pathlib import Path
import json, struct, sys, hashlib

def bits(x): return struct.unpack('>Q',struct.pack('>d',x))[0]
def frac(x): return F.from_float(x)
def token(x, up=False):
    if x == 0: return 'Z+'
    sign=x<0; x=abs(x)
    e=x.numerator.bit_length()-x.denominator.bit_length()
    if x < (F(2)**e): e-=1
    step=F(2)**(e-1023)
    m=x//step
    if x != m*step and (up != sign): m+=1
    if m.bit_length()>1024: m//=2; e+=1
    return ('-' if sign else '+')+format(m,'0256x').rstrip('0')+'p'+str(e)

def atan(q):
    s=sum((F((-1)**k,(2*k+1)*q**(2*k+1)) for k in range(128)),F())
    return s,s+F(1,257*q**257)
a,b=atan(5),atan(239)
plo,phi=16*a[0]-4*b[1],16*a[1]-4*b[0]
scale=2**512
pn=plo*scale//1; un=-(-phi*scale//1)
assert un-pn==1
assert hex(pn)[2:]=='3243f6a8885a308d313198a2e03707344a4093822299f31d0082efa98ec4e6c89452821e638d01377be5466cf34e90c6cc0ac29b7c97c50dd3f84d5b5b5470917'
assert F(pn,scale)<plo<phi<F(un,scale)
assert phi-plo<F(1,2**600)
pi=(F(pn,scale),F(un,scale))
# mode, D, effective t, material operands [E,nu] / [E,G] /
# [Tlo,T,Thi,Elo,Ehi,Glo,Ghi,Ehat,Ghat]; K [E,G,A,J,Iz,Iy,Zhat].
x=float.fromhex('0x1.0000000000001p0'); h=float.fromhex('0x0.0000000000001p-1022')
cases=[
 ('exact_normal',0,4.,1.,[210e9,.3],[210e9,210e9/2.6,9.,30.,15.,15.,7.]),
 ('ordinary_exact_product',1,4.,1.,[x,x],[x,x,x,x,x,x,2.]),
 ('interpolated_negative_e',2,4.,1.,[0.,1.,2.,-1.,3.,2.,6.,1.,4.],[1.,4.,9.,30.,15.,15.,7.]),
 ('thin_cancellation',1,1.,2.**-100,[2.,3.],[2.,3.,2.**-98,2.**-102,2.**-103,2.**-103,2.**-102]),
 ('subnormal_geometry_material',1,4*h,h,[h,h],[h,h,h,h,h,h,h]),
 ('ratio_amplification',0,4.,1.,[1.,float.fromhex('-0x1.fffffffffffffp-1')],[1.,2.**52,1.,1.,1.,1.,1.]),
 ('interpolated_wide_span',2,4.,1.,[-float.fromhex('0x1.fffffffffffffp1023'),0.,float.fromhex('0x1.fffffffffffffp1023'),h,1.,h,1.,.5,.5],[.5,.5,1.,1.,1.,1.,1.]),
 ('interpolated_cancellation_span',2,4.,1.,[-2.**100,0.,2.**100,-1.,float.fromhex('0x1.0000000000001p0'),h,3*h,2.**-53,2*h],[2.**-53,2*h,1.,1.,1.,1.,1.]),
]
lines=['// Generated only by product_certificate_vectors.py; exact rational targets.', '#[rustfmt::skip]', 'pub(super) const CASES: &[(&str, u8, [u64; 2], [u64; 9], [u64; 7], &[(&str, &str)], &[&str])] = &[']
for name,mode,D,t,mat,k in cases:
    d,w=frac(D),frac(t); c=d/2; ri=c-w; p=w*(d-w); q=c*c+ri*ri; g=p*q
    assert 0<w<c and p==c*c-ri*ri and g==c**4-ri**4
    a=(pi[0]*p,pi[1]*p); i=(pi[0]*g/4,pi[1]*g/4); j=(2*i[0],2*i[1]); z=(i[0]/c,i[1]/c)
    m=list(map(frac,mat)); kk=list(map(frac,k))
    if mode==0: E=(m[0],m[0]); G=(m[0]/(2*(1+m[1])),)*2
    elif mode==1: E=(m[0],)*2; G=(m[1],)*2
    else:
        tl,T,th,el,eh,gl,gh,ea,ga=m
        assert tl<T<th
        def interp(lo,hi,actual):
            numerator=th*lo-T*lo+T*hi-tl*hi
            assert numerator==(th-T)*lo+(T-tl)*hi and numerator>0
            val=numerator/(th-tl)
            return min(val,actual),max(val,actual)
        E=interp(el,eh,ea); G=interp(gl,gh,ga)
    C=[(E[0]*a[0],E[1]*a[1]),(G[0]*j[0],G[1]*j[1]),(E[0]*i[0],E[1]*i[1]),(E[0]*i[0],E[1]*i[1])]
    CK=[kk[0]*kk[2],kk[1]*kk[3],kk[0]*kk[4],kk[0]*kk[5]]
    delta=[max(abs(lo-v),abs(hi-v)) for (lo,hi),v in zip(C,CK)]
    rz=(min(kk[4]/c,kk[6]),max(kk[4]/c,kk[6])) if mode else (F(),F())
    intervals=[(c,c),(ri,ri),(p,p),(q,q),(g,g),a,i,j,z,E,G,*C,rz]
    vals=mat+[0.]*(9-len(mat))
    lines += [f'    ("{name}", {mode}, [{bits(D)}, {bits(t)}],', '     ['+', '.join(str(bits(v)) for v in vals)+'],', '     ['+', '.join(str(bits(v)) for v in k)+'], &[']
    lines += [f'        ("{token(lo)}", "{token(hi,True)}"),' for lo,hi in intervals]
    lines += ['     ], &['+', '.join('"'+token(v,True)+'"' for v in CK+delta)+']),']
lines+= ['];','']
text='\n'.join(lines); path=Path(__file__).with_suffix('.rs')
if '--write' in sys.argv: path.write_text(text)
else: assert path.read_text()==text, 'frozen fixture differs'
assert frac(x)*frac(x)-frac(x*x)==F(1,2**104)
assert (F(25)*-7-F(7)*-7+F(7)*18)==0
print(json.dumps({'cases':len(cases),'pi_bracket':'128-term rational Machin, adjacent 2^-512 grid','exact_K_discriminator':'2^-104','fixture_sha256':hashlib.sha256(text.encode()).hexdigest()},indent=2))
