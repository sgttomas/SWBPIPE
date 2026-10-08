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
    rz=(min(kk[4]/c,kk[6]),max(kk[4]/c,kk[6]))
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

# ---------------------------------------------------------------- B2-K (KD §6)
# The combination oracle (I94 KD §6, with RV115's SF-2, SF-3, N-7 and N-9). A new
# fixture, product_certificate_combination_vectors.rs; the CASES block above
# and its fixture are unchanged. Standard library only, no production import,
# no copy of kernel arithmetic: exact Fractions, the tight Machin bracket
# [plo, phi] above (width < 2^-600), the textbook 12x12 Euler-Bernoulli frame
# element solved exactly and reduced to the tip block, closed statics, and
# integer square roots.
import math
def b64(x): return F.from_float(x)
def rn64(x): return F(float(x))  # int/int true division: correctly rounded, ties to even
def bitsf(x): return bits(float(x))
L1,D2,T2,E2,G2=F(1),b64(.1),b64(.005),b64(210e9),b64(80e9)
C2=D2/2; RI2=C2-T2; P2=T2*(D2-T2); Q2=C2*C2+RI2*RI2; GG2=P2*Q2
AX=(plo*P2,phi*P2); IX=(plo*GG2/4,phi*GG2/4); JX=(2*IX[0],2*IX[1]); ZX=(IX[0]/C2,IX[1]/C2)
SEC=[]
for lo,hi in (AX,IX,JX,ZX):
    assert rn64(lo)==rn64(hi), 'unambiguous correctly rounded section'
    SEC.append(rn64(lo))
AK,IK,JK,ZK=SEC
def element(ea,gj,ei):
    """Local 12x12 Euler-Bernoulli frame stiffness, L = 1, Iy = Iz."""
    k=[[F(0)]*12 for _ in range(12)]
    def put(i,j,v): k[i][j]+=v
    for a,b,s in ((0,6,ea),(3,9,gj)):
        put(a,a,s);put(b,b,s);put(a,b,-s);put(b,a,-s)
    # bending in the x-y plane (uy, rz) and the x-z plane (uz, ry)
    for (u,r,sg) in ((1,5,1),(2,4,-1)):
        idx=[u,r,u+6,r+6]
        m=[[12*ei,6*ei*sg,-12*ei,6*ei*sg],[6*ei*sg,4*ei,-6*ei*sg,2*ei],
           [-12*ei,-6*ei*sg,12*ei,-6*ei*sg],[6*ei*sg,2*ei,-6*ei*sg,4*ei]]
        for x in range(4):
            for y in range(4): put(idx[x],idx[y],m[x][y])
    return k
def solve(a,b):
    n=len(b); m=[row[:]+[b[i]] for i,row in enumerate(a)]
    for c in range(n):
        p=next(r for r in range(c,n) if m[r][c]!=0); m[c],m[p]=m[p],m[c]
        for r in range(n):
            if r!=c and m[r][c]!=0:
                f=m[r][c]/m[c][c]; m[r]=[x-f*y for x,y in zip(m[r],m[c])]
    return [m[i][n]/m[i][i] for i in range(n)]
def tip_and_actions(coef,tip):
    k=element(*coef); kjj=[row[6:] for row in k[6:]]
    ut=solve(kjj,tip); u=[F(0)]*6+ut
    return ut,[sum(k[i][j]*u[j] for j in range(12)) for i in range(12)]
LAW_K=(E2*AK,G2*JK,E2*IK)
LAW_G=((E2*AX[0],G2*JX[0],E2*IX[0]),(E2*AX[1],G2*JX[1],E2*IX[1]))
# Operands: (node, component, value). Node 1 is the tip, node 0 the restrained root.
OPERANDS={
 'A':[(1,0,1.),(1,1,1.),(1,3,1.)],
 'B':[(1,2,3.),(1,4,-2.),(1,1,.5),(1,1,-.5)],
 'A2':[(1,0,1.),(1,1,1.),(1,3,1.)],
 'P':[(1,0,1.),(1,0,2.**-60)],
 'Q':[(1,0,-1.)],
 'H':[(1,1,2.**600),(1,1,2.**-600)],
 'R':[(0,1,5.),(0,2,4.)],
 'R2':[(0,2,-4.)],
}
OPERAND_ORDER=['A','B','A2','P','Q','H','R','R2']
# (name, [(factor, operand, prepared)])
COMBOS=[
 ('C1',[(1.,'A',0),(1.,'B',0),(-1.,'A2',0)]),
 ('C2',[(.1,'A',0),(3.,'B',0)]),
 ('C3',[(1.,'P',0),(1.,'Q',0)]),
 ('C4',[(2.,'B',1),(1.,'A',0)]),
 ('C5',[(2.**700,'A',0),(-2.**700,'A2',0),(2.**-700,'B',0)]),
 ('C6',[(1.,'H',0)]),
 ('C7',[(1.,'A',0),(1.,'R',0),(1.,'R2',0)]),
 ('C8',[(.1,'B',0),(1.,'A2',0)]),
 ('C9',[(2.**600,'A',0),(2.**-600,'A2',0)]),
]
def nets_of(terms):
    """Exact per-DOF nets of (factor, operand) terms: every DOF with a term."""
    nets={}
    for factor,name in terms:
        for node,comp,v in OPERANDS[name]:
            g=node*6+comp; nets[g]=nets.get(g,F(0))+b64(factor)*b64(v)
    return dict(sorted(nets.items()))
def canonical(x):
    if x==0: return (0,0,0)
    neg=int(x<0); x=abs(x); e=0
    while x.denominator!=1: x*=2; e-=1
    m=x.numerator
    while m%2==0: m//=2; e+=1
    return (neg,m,e)
def k4led(nets):
    out=bytearray(b'K4LED\x01')+struct.pack('<I',len(nets))
    for g,v in nets.items():
        neg,m,e=canonical(v); limbs=[]
        while m: limbs.append(m&(2**64-1)); m>>=64
        out+=struct.pack('<IBBqI',g//6,g%6,neg,e,len(limbs))
        for l in limbs: out+=struct.pack('<Q',l)
    return bytes(out)
def floorlog2(x):
    e=x.numerator.bit_length()-x.denominator.bit_length()
    if x<F(2)**e: e-=1
    return e
def rd1024(x,up=False):
    """x rounded to 1024 significant bits toward -inf (or +inf)."""
    if x==0: return F(0)
    step=F(2)**(floorlog2(abs(x))-1023); q=x/step; n=q.numerator//q.denominator
    if up and n*q.denominator!=q.numerator: n+=1
    return n*step
def rn1024(x):
    if x==0: return F(0)
    step=F(2)**(floorlog2(abs(x))-1023); q=x/step; n=q.numerator//q.denominator; r=q-n
    if r>F(1,2) or (r==F(1,2) and n%2): n+=1
    return n*step
def sqrt_bounds(x):
    if x==0: return (F(0),F(0))
    e=floorlog2(x)//2-2100; s=x/F(4)**e; n=math.isqrt(s.numerator//s.denominator)
    up=n if n*n*s.denominator==s.numerator else n+1
    return (n*F(2)**e,up*F(2)**e)
def hull(*xs): return (min(xs),max(xs))
def interval_div(a,b):  # a a point, b a positive interval
    return hull(a/b[0],a/b[1])
def mag_bounds(vs):  # vs: [(lo,hi)]*3
    lo=sum((F(0) if l<=0<=h else min(abs(l),abs(h)))**2 for l,h in vs)
    hi=sum(max(abs(l),abs(h))**2 for l,h in vs)
    return (sqrt_bounds(lo)[0],sqrt_bounds(hi)[1])
STATIONS=((17,F(1,4)),(18,F(1,2)),(19,F(3,4)))
def truths(terms):
    """Every native row and stress row of the one-member specimen, per lane."""
    nets=nets_of(terms)
    tip=[nets.get(6+c,F(0)) for c in range(6)]; root=[nets.get(c,F(0)) for c in range(6)]
    uk,ak=tip_and_actions(LAW_K,tip)
    ug=[tip_and_actions(l,tip) for l in LAW_G]
    for _,a in ug: assert a==ak, 'statics are law-independent'
    # closed statics: end J = tip loads; end I = -(tip loads moved to the root)
    assert ak[6:]==tip
    assert ak[:6]==[-tip[0],-tip[1],-tip[2],-tip[3],-(tip[4]-L1*tip[2]),-(tip[5]+L1*tip[1])]
    rows={}
    def both(key,k,g): rows[key]=(k,g)
    for c in range(6):
        both(f'u0.{c}',(F(0),F(0)),(F(0),F(0)))
        both(f'u1.{c}',(uk[c],uk[c]),hull(ug[0][0][c],ug[1][0][c]))
    both('m0',(F(0),F(0)),(F(0),F(0)))
    both('m1',mag_bounds([(uk[c],uk[c]) for c in range(3)]),mag_bounds([rows[f'u1.{c}'][1] for c in range(3)]))
    for c in range(6):
        both(f'e7.I.{c}',(ak[c],ak[c]),(ak[c],ak[c])); both(f'e7.J.{c}',(ak[6+c],ak[6+c]),(ak[6+c],ak[6+c]))
    site={}
    for sid,t in STATIONS:
        s=[ak[6+c] if c<4 else t*ak[6+c]+(t-1)*ak[c] for c in range(6)]
        assert s==[tip[0],tip[1],tip[2],tip[3],tip[4]-(1-t)*tip[2],tip[5]+(1-t)*tip[1]]
        site[str(sid)]=s
        for c in range(6): both(f's{sid}.{c}',(s[c],s[c]),(s[c],s[c]))
    react=[ak[c]-root[c] for c in range(6)]
    for c in range(6): both(f'r0.{c}',(react[c],react[c]),(react[c],react[c]))
    sf=sqrt_bounds(sum(v*v for v in react[:3])); sm=sqrt_bounds(sum(v*v for v in react[3:]))
    both('sf3',sf,sf); both('sm3',sm,sm)
    site['I']=[-v for v in ak[:6]]; site['J']=ak[6:]
    zk=hull(IK/C2,ZK)
    for sname in ['I','J','17','18','19']:
        x=site[sname]
        n,t,my,mz=x[0],x[3],x[4],x[5]
        both(f'z7.{sname}.0',(n/AK,n/AK),interval_div(n,AX))
        for st,m in ((1,my),(2,mz)):
            both(f'z7.{sname}.{st}',interval_div(m,zk),hull(m*C2/IX[0],m*C2/IX[1]))
        both(f'z7.{sname}.3',(t*C2/JK,t*C2/JK),hull(t*C2/JX[0],t*C2/JX[1]))
    return nets,rows
def b2k_fixture():
    out=['// Generated only by product_certificate_vectors.py (B2-K section); exact rational targets.',
         '// Truths are outward 1024-bit tokens [RD, RU] of exact intervals; SI units.',
         '#[rustfmt::skip]',
         f'pub(super) const SECTION: [u64; 5] = [0x{bitsf(AK):016x}, 0x{bitsf(IK):016x}, 0x{bitsf(JK):016x}, 0x{bitsf(ZK):016x}, 0x{bitsf(C2):016x}];',
         '#[rustfmt::skip]',
         'pub(super) const OPERANDS: &[(&str, &[(u32, u8, u64)])] = &[']
    for name in OPERAND_ORDER:
        out.append(f'    ("{name}", &['+', '.join(f'({n}, {c}, 0x{bits(v):016x})' for n,c,v in OPERANDS[name])+']),')
    out+=['];','#[rustfmt::skip]',
          'pub(super) const COMBINATIONS: &[(&str, &[(u64, &str, bool)], &str, &[(u32, &str, &str, &str)], &[(&str, (&str, &str), (&str, &str))])] = &[']
    summary={}
    for name,terms in COMBOS:
        nets,rows=truths([(f,o) for f,o,_ in terms])
        led=k4led(nets); summary[name]={'k4led_sha256':hashlib.sha256(led).hexdigest(),
            'nets':{str(g):str(v) for g,v in nets.items()}}
        out.append(f'    ("{name}", &['+', '.join(f'(0x{bits(f):016x}, "{o}", {str(bool(p)).lower()})' for f,o,p in terms)+'],')
        out.append(f'     "{led.hex()}",')
        netl=[(g,token(rd1024(v)),token(rd1024(v,True),True),token(rn1024(v))) for g,v in nets.items() if v!=0]
        out.append('     &['+', '.join(f'({g}, "{a}", "{b}", "{c}")' for g,a,b,c in netl)+'],')
        out.append('     &[')
        for key,(k,g) in rows.items():
            out.append(f'        ("{key}", ("{token(k[0])}", "{token(k[1],True)}"), ("{token(g[0])}", "{token(g[1],True)}")),')
        out.append('     ]),')
    out+=['];','']
    # Discriminators. C3: the binary64 sum of its operand rows is exactly 0,
    # while its truth is 2^-60/EA.
    p_row=rn64((1+F(2)**-60)/LAW_K[0]); q_row=rn64(-1/LAW_K[0])
    assert p_row+q_row==0 and truths([(1.,'P'),(1.,'Q')])[1]['u1.0'][0][0]==F(2)**-60/LAW_K[0]
    # C2/C8 (SF-3): 0.1*3 needs 54 bits, so fl(0.1*3) is not the net.
    exact=b64(.1)*3; assert exact!=b64(.1*3) and exact.numerator.bit_length()==54
    # C6: the nearest 1024-bit rounding of 2^600 + 2^-600 is 2^600, which excludes the net.
    n6=F(2)**600+F(2)**-600; assert rn1024(n6)==F(2)**600<n6<rd1024(n6,True)
    # Representative-source discriminator (KD §6 item 4 (ii)): the K-law tip
    # motions under operand 0's loads alone, for C1, C2, C4 and C7.
    out.append('#[rustfmt::skip]')
    out.append('pub(super) const REPRESENTATIVE_TIP_K: &[(&str, [(&str, &str); 6])] = &[')
    for name,terms in COMBOS:
        if name not in ('C1','C2','C4','C7'): continue
        f,o,_=terms[0]; rows=truths([(1.,o)])[1]
        out.append(f'    ("{name}", ['+', '.join(f'("{token(rows[f"u1.{c}"][0][0])}", "{token(rows[f"u1.{c}"][0][1],True)}")' for c in range(6))+']),')
    out+=['];','']
    return '\n'.join(out),summary
# (ii) (DEF-C r2 rows.displacement_magnitude): RN64, ties to even, of the
# exact sqrt(x^2+y^2+z^2), refused beyond binary64; an integer square root at
# the result's own quantum, independent of production.
MAXF=b64(float.fromhex('0x1.fffffffffffffp1023'))
def f64b(b): return struct.unpack('>d',struct.pack('>Q',b))[0]
def norm3(bx,by,bz):
    S=sum(b64(f64b(b))**2 for b in (bx,by,bz))
    if S==0: return 0
    E=floorlog2(S)//2; q=max(E-52,-1074); T=S/F(4)**q
    k=math.isqrt(T.numerator//T.denominator); c=4*T-(2*k+1)**2
    if c>0 or (c==0 and k%2==1): k+=1
    v=k*F(2)**q
    return None if v>MAXF else bits(float(v))
def norm3_vectors():
    import random
    rng=random.Random(20261008); out=[]
    MIN=0x0010000000000000; MAXB=0x7fefffffffffffff
    curated=[(0x000fffffffffffff,0x0000000004000001,0),(0x000fffffffffffff,0x0000000004000000,0),
        (0x000fffffffffffff,0x0000000003ffffff,0x0000000000002d42),(0x000fffffffffffff,0x0000000003ffffff,0x0000000000002d41),
        (MAXB,0,0),(MAXB,MAXB,0),(MAXB,bits(2.**997),0),(MAXB,bits(2.**998),0),(bits(2.**1023),bits(2.**1023),0),
        (bits(1e308),bits(1e308),bits(1e308)),(1,0,0),(1,1,1),(0x000fffffffffffff,)*3,(MIN,1,0),
        (bits(1.),1,0),(bits(1e300),bits(1e-300),1),(bits(1e200),bits(1e200),0),(bits(1e-200),bits(1e-200),0),
        (bits(-3.),bits(4.),bits(12.))]
    for sx in (0,1):
        for sy in (0,1):
            for sz in (0,1): curated.append((sx<<63,sy<<63,sz<<63))
    out+=curated
    # constructed exact midpoints m = d*r (54 bits, odd) with (a,b,c,d) a Pythagorean
    # quadruple: x,y,z = a*r,b*r,c*r are binary64; then one ulp-of-r perturbations.
    for a,b,c,d in ((1,2,2,3),(2,3,6,7),(1,4,8,9),(4,4,7,9),(2,6,9,11),(6,6,7,11)):
        for _ in range(12):
            lo=-(-2**53//d); r=rng.randrange(lo,min(2**54//d,2**53)) | 1
            if (d*r).bit_length()!=54: continue
            e=rng.randrange(-1000,900)
            t=[bits(float(F(v*r)*F(2)**e)) for v in (a,b,c)]
            out.append(tuple(t))
            out.append((t[0],t[1],t[2]+1)); out.append((t[0],t[1],t[2]-1))
    for _ in range(120):
        e=rng.choice([rng.randrange(-1074,-1000),rng.randrange(-200,200),rng.randrange(900,1023)])
        out.append(tuple(bits(float(F(rng.getrandbits(53))*F(2)**(e-52)))|(rng.getrandbits(1)<<63) for _ in range(3)))
    for _ in range(60):
        out.append(tuple(rng.getrandbits(63)|(rng.getrandbits(1)<<63) for _ in range(3)))
    return [v for v in out if all(math.isfinite(f64b(x)) for x in v)]
NORM3=[(x,y,z,norm3(x,y,z)) for x,y,z in norm3_vectors()]
assert sum(1 for v in NORM3[:4] if v[3]==0x0010000000000000)==3, 'SA4-1 vectors at MIN_POSITIVE'
b2k_text,b2k_summary=b2k_fixture()
b2k_text+='#[rustfmt::skip]\npub(super) const NORM3: &[(u64, u64, u64, Option<u64>)] = &[\n'+''.join(
    f'    (0x{x:016x}, 0x{y:016x}, 0x{z:016x}, {"None" if r is None else f"Some(0x{r:016x})"}),\n' for x,y,z,r in NORM3)+'];\n'

b2k_path=Path(__file__).with_name('product_certificate_combination_vectors.rs')
if '--write' in sys.argv: b2k_path.write_text(b2k_text)
else: assert b2k_path.read_text()==b2k_text, 'frozen combination fixture differs'
def tokval(t):
    if t=='Z+': return F(0)
    h,e=t[1:].split('p'); v=int(h.ljust(256,'0'),16)*F(2)**(int(e)-1023)
    return -v if t[0]=='-' else v
def rnd(x,up): return rd1024(x,up)
def diagnose(path,vectors=None):
    """K-09 (KD §6, RV115 N-7): from the kernel's printed B2K rows, check
    independently (a) both lanes contain the exact truth, (b) S* from the
    published rows, the classes, bounds and the four relative predicates
    (reproducing the binary64 operations of A_f64 and the exact A_exact and
    decimal tests on the kernel's own enclosures), (c) every kernel verdict,
    (d) each K4LED and its sha256, (e) the (ii) displacement magnitudes."""
    rows=[];ledgers={}
    for line in Path(path).read_text().splitlines():
        if 'B2K_LEDGER ' in line: d=json.loads(line.split('B2K_LEDGER ',1)[1]); ledgers[d['combination']]=d
        elif 'B2K_ROW ' in line: rows.append(json.loads(line.split('B2K_ROW ',1)[1]))
    assert set(ledgers)=={c[0] for c in COMBOS}, sorted(ledgers)
    two64=18446744073709551616.0; small=2.0**-988
    report={}
    for name,terms in COMBOS:
        nets,truth=truths([(f,o) for f,o,_ in terms])
        led=k4led(nets); assert ledgers[name]['k4led']==led.hex(), name
        assert ledgers[name]['precision']!=512, 'no p512 floor in this specimen'
        mine=[r for r in rows if r['combination']==name]
        val=lambda r:f64b(int(r['raw'],16))
        def norm(r):
            y=val(r)
            return y/1000.0 if r['unit']=='mm' else (y*1e6 if r['unit']=='MPa' else y)
        # (b) S*: per kind, the largest |n| over non-input native rows and support components; then couple at L = 1.
        sl=[0.0]*4
        def slot_of(k):  # translation, rotation, force, moment
            if k[0]=='u': return 0 if int(k.split('.')[1])<3 else 1
            if k[0]=='m': return 0
            if k in ('sf3','sm3'): return 2 if k=='sf3' else 3
            return 2 if int(k.split('.')[-1])<3 else 3
        for r in mine:
            k=r['key']
            if k.startswith('z') or r['class']=='input': continue
            sl[slot_of(k)]=max(sl[slot_of(k)],abs(norm(r)))
        sc=[max(sl[0],1.0*sl[1]),max(sl[1],sl[0]/1.0),max(sl[2],sl[3]/1.0),max(sl[3],1.0*sl[2])]
        A,Z=float(AK),float(ZK)
        agree=passed=0
        for r in mine:
            k=r['key']; n=norm(r); assert int(r['n'],16)==bits(n), (name,k,'normalized')
            scale=(sc[2]/A)+(1.0*(sc[3]/Z)) if k[0]=='z' else sc[slot_of(k)]
            assert int(r['scale'],16)==bits(scale), (name,k,'scale',r['scale'],scale.hex())
            klo,khi,glo,ghi=(tokval(t) for t in r['k']+r['g'])
            tk,tg=truth[k]
            assert klo<=tk[0] and tk[1]<=khi and glo<=tg[0] and tg[1]<=ghi, (name,k,'truth outside a lane')
            lo,hi=min(klo,glo),max(khi,ghi); x=b64(n)
            h=max(rnd(x-lo,True),rnd(hi-x,True))
            inp=k.startswith('u0.')
            relative=not inp and scale>=small and not (abs(n)<2.0**-34*scale)
            cls='input' if inp else ('relative' if relative else 'absolute')
            assert cls==r['class'], (name,k,cls,r['class'])
            if cls=='input': ok=h==0; preds=[ok,None,None,None]
            elif cls=='absolute':
                assert not (0<scale<small), 'small-scale bound not reached here'
                near=scale/two64; b=math.nextafter(near,math.inf) if near*two64<scale else near
                assert int(r['bound'],16)==bits(b), (name,k,'bound'); ok=h<=b64(b); preds=[ok,None,None,None]
            else:
                m=max(b64(abs(n)),b64(scale))
                se=m*F(2)**-64+m*F(2)**-85+b64(abs(n))*F(2)**-53+F(2)**-1074-h>=0
                a0=2.0**-64*max(abs(n),scale); a1=a0*(1+2.0**-21); u1=2.0**-53*abs(n)+2.0**-1074
                sb=h<=b64(a1+u1)
                dsi=b64(abs(n))-10**9*h>=0
                y=b64(val(r)); c={'mm':F(1000),'MPa':F(1,10**6)}.get(r['unit'],F(1))
                rlo,rhi=(rnd(lo*c,False),rnd(hi*c,True)) if c!=1 else (lo,hi)
                hu=max(rnd(y-rlo,True),rnd(rhi-y,True)); draw=abs(y)-10**9*hu>=0
                preds=[se,sb,dsi,draw]; ok=all(preds)
            kernel=[None if p is None else p for p in r['predicates']]
            assert kernel==preds, (name,k,kernel,preds)
            assert ok==r['pass'], (name,k)
            agree+=1; passed+=ok
            if k=='m1':
                comps=[int(c,16) for c in r['components']]
                mine_u=[next(x for x in mine if x['key']==f'u1.{j}') for j in range(3)]
                assert comps==[int(u['raw'],16) for u in mine_u]
                assert norm3(*comps)==int(r['raw'],16), (name,'(ii) magnitude')
        report[name]={'rows':len(mine),'verdicts_agree':agree,'passed':passed,'k4led_sha256':hashlib.sha256(led).hexdigest()}
    out={'b2k_diagnose':report,'rows':len(rows)}
    if vectors:
        d=json.load(open(vectors)); bad=[]
        for v in d['vectors']:
            want=None if v['p']=='refused' else int(v['p'],16)
            if norm3(int(v['x'],16),int(v['y'],16),int(v['z'],16))!=want: bad.append(v['label'])
        out['exact_norm_vectors']={'count':len(d['vectors']),'disagree':bad}
        assert not bad, bad
    print(json.dumps(out,indent=2))
print(json.dumps({'b2k_combinations':len(COMBOS),'section_bits':[hex(bitsf(v)) for v in SEC+[C2]],
    'combination_fixture_sha256':hashlib.sha256(b2k_text.encode()).hexdigest(),
    'k4led_sha256':{k:v['k4led_sha256'] for k,v in b2k_summary.items()}},indent=2))
if '--diagnose' in sys.argv:
    i=sys.argv.index('--diagnose'); diagnose(sys.argv[i+1],sys.argv[i+2] if len(sys.argv)>i+2 else None)
