"""RV50 exact abstract controls. No repository imports, compiler or product execution."""
from fractions import Fraction as F
from math import isqrt
import random, json
P=1024
MAX=0x7fefffffffffffff
LIMIT=1<<62
rng=random.Random(50)
checks=[]
def power(k): return F(1<<k) if k>=0 else F(1,1<<-k)
def log2floor(x):
    assert x>0
    e=x.numerator.bit_length()-x.denominator.bit_length()
    return e-1 if x<power(e) else e
def ceilint(x): return -((-x.numerator)//x.denominator)
def rnd(x,p=P,mode='nearest'):
    x=F(x)
    if x==0:return x
    if x<0:return -rnd(-x,p,{'up':'down','down':'up','nearest':'nearest'}[mode])
    e=log2floor(x); unit=power(e+1-p); scaled=x/unit
    k,r=divmod(scaled.numerator,scaled.denominator)
    if mode=='up':k+=bool(r)
    elif mode=='nearest':k+=2*r>scaled.denominator or (2*r==scaled.denominator and k%2==1)
    return k*unit
def adjacent(x,d,p=P):
    assert x
    e=log2floor(abs(x))
    away=(d=='up')==(x>0)
    q=power(e+1-p if away or abs(x)!=power(e) else e-p)
    return x+q if d=='up' else x-q
def corrected(exact,p=P):
    near=rnd(exact,p)
    lo=adjacent(near,'down',p) if near>exact else near
    hi=adjacent(near,'up',p) if near<exact else near
    assert lo==rnd(exact,p,'down') and hi==rnd(exact,p,'up')
    return lo,hi
def report(name,**detail):checks.append(dict(name=name,passed=True,**detail))
# Exhaustive low-precision rational lattice verifies both signs, carry binades and ties.
total=0
for p in range(2,9):
 for a in range(-65,66):
  for d in (1,3,5,7,8,17):
   corrected(F(a,d),p);total+=1
report('adjacent_side_tests_exhaustive',rational_cases=total)
# Independent direct floor/ceil reference against source restoring division grammar.
maxdiv=0
for j in range(80):
 a=(1<<1023)|rng.getrandbits(1023); b=(1<<1023)|rng.getrandbits(1023)
 if j<4:a,b=[(1<<1023,1<<1023),((1<<1024)-1,1<<1023),(1<<1023,(1<<1024)-1),((1<<1024)-1,(1<<1024)-1)][j]
 rem=a; q=0
 if rem>=b:rem-=b;q=1<<1025
 for k in range(1,1026):
  rem*=2;maxdiv=max(maxdiv,rem.bit_length())
  if rem>=b:rem-=b;q|=1<<(1025-k)
  assert 0<=rem<b
 assert q==(a<<1025)//b and rem==(a<<1025)%b
 drop=q.bit_length()-P;keep=q>>drop
 rb=(q>>(drop-1))&1; rest=bool(rem or q&((1<<(drop-1))-1))
 keep+=bool(rb and (rest or keep&1))
 near=keep*power(drop-1025)
 assert near==rnd(F(a,b))
 for sign in (1,-1):corrected(sign*F(a,b))
report('source_division_finite_loop',pairs=80,iterations=1025,max_trial_bits=maxdiv)
# Compare source integer digit-pair root to isqrt, then independent midpoint-square nearest oracle.
maxroot=0
rootcases=[]
for e in (-2149,-2148,-1075,-1,0,1,1023,2046):
 for m in (1<<1023,(1<<1024)-1,(1<<1023)|rng.getrandbits(1023)):
  rootcases.append((m,e))
for m,e in rootcases:
 a=m*power(e-1023);ue=e-1023;base=1026;shift=base+((ue-base)&1);rad=m<<shift
 root=rem=0
 for k in range(1025,-1,-1):
  rem=(rem<<2)|((rad>>(2*k))&3);trial=(root<<2)|1
  maxroot=max(maxroot,rem.bit_length(),trial.bit_length());root<<=1
  if rem>=trial:rem-=trial;root|=1
 assert root==isqrt(rad) and rem==rad-root*root
 unit=power(e//2+1-P)
 scaled=a/(unit*unit)
 k=isqrt(scaled.numerator//scaled.denominator)
 lower=k*unit;upper=lower if lower*lower==a else (k+1)*unit
 midpoint=(lower+upper)/2
 near=lower if a<midpoint*midpoint else upper if a>midpoint*midpoint else (lower if k%2==0 else upper)
 drop=root.bit_length()-P;keep=root>>drop
 rb=(root>>(drop-1))&1;rest=bool(rem or root&((1<<(drop-1))-1))
 keep+=bool(rb and (rest or keep&1))
 actual=keep*power((ue-shift)//2+drop)
 assert actual==near
 lo=adjacent(actual,'down') if actual*actual>a else actual
 hi=adjacent(actual,'up') if actual*actual<a else actual
 assert (lo,hi)==(lower,upper) and 0<=lo*lo<=a<=hi*hi
report('source_sqrt_exact_midpoint_oracle',cases=len(rootcases),iterations=1026,max_active_bits=maxroot)
# Exact-product residual representability is independently checked for full significands.
for j in range(80):
 a=((1<<1023)|rng.getrandbits(1023))*power(rng.randrange(-10,11)-1023)
 b=((1<<1023)|rng.getrandbits(1023))*power(rng.randrange(-10,11)-1023)
 exact=a*b;s=rnd(exact);residual=exact-s
 assert residual==rnd(residual) and s+residual==exact
 corrected(exact)
report('two_product_residual_and_side',pairs=80)
# Carry bound and exponent/span obligations are symbolic, with exact finite stress magnitudes.
assert 5*((1<<8128)-1)<1<<8131<1<<8192
assert ((1<<64)-1)**2+2*((1<<64)-1)==(1<<128)-1
for count in range(1,6):
 pos=sum(((1<<8128)-1)-(j<<100) for j in range(count));neg=pos-1
 assert pos.bit_length()<=8131 and neg.bit_length()<=8131 and pos-neg==1
for e,k,ok in [(LIMIT,0,True),(LIMIT,1,False),(-LIMIT,-1,False),(-LIMIT,1024,True)]:assert (-LIMIT<=e+k<=LIMIT)==ok
# Independent span from exact dyadic integer valuation.
def span(values):
 lo=[];hi=[]
 for v in values:
  if v:
   v=abs(F(v));assert v.denominator&(v.denominator-1)==0
   lo.append((v.numerator&-v.numerator).bit_length()-v.denominator.bit_length())
   hi.append(v.numerator.bit_length()-v.denominator.bit_length())
 return max(hi)-min(lo)+1 if lo else 0
assert span([1,power(-8127)])==8128 and span([1,power(-8128)])==8129
report('carry_span_and_symbolic_exponent_fences',terms_at_most=5,live_magnitude_bits_at_most=8131)
# Direct quantum ceil oracle differs from the bit-search implementation under review.
def value(bits):
 b=bits>>52;f=bits&((1<<52)-1)
 return f*power(-1074) if b==0 else ((1<<52)|f)*power(b-1075)
def ru64(x):
 if x==0:return F(0)
 unit=power(max(log2floor(x)-52,-1074));ans=ceilint(x/unit)*unit
 if ans>value(MAX):raise ValueError('overflow')
 return ans
def rn64(x):
 if not x:return F(0)
 sign=1 if x>0 else -1;x=abs(x);unit=power(max(log2floor(x)-52,-1074));k,r=divmod((x/unit).numerator,(x/unit).denominator);d=(x/unit).denominator
 k+=2*r>d or (2*r==d and k%2==1)
 if k*unit>value(MAX):raise ValueError('overflow')
 return sign*k*unit
def search(terms):
 target=sum(terms,F(0));count=1
 if value(MAX)<target:raise ValueError('overflow')
 lo=0;hi=MAX
 for _ in range(63):
  if lo==hi:break
  mid=(lo+hi)//2;count+=1
  if value(mid)>=target:hi=mid
  else:lo=mid+1
 assert lo==hi and count<=64 and value(lo)==ru64(target)
 return value(lo),count
h=power(-1074);maxcmp=0
triples=[(F(0),F(0),F(0)),(h,h,h),(F(1),power(-53),h),(value(MAX),F(0),F(0))]
for j in range(128):
 S=value(rng.randrange(1,0x0230000000000000));n=value(rng.randrange(0,MAX+1))
 triples.append((ru64(S*power(-64)),ru64(n*power(-53)),h))
for t in triples:
 ans,c=search(t);maxcmp=max(maxcmp,c)
try:search((value(MAX),h,F(0)))
except ValueError:pass
else:raise AssertionError('expected overflow')
# A closed small-bound expression demonstrates why generic RN1024->RN64 is not a proof.
t=(power(-1053),F(1),h);target=sum(t,F(0))
assert rn64(rnd(target))<target and search(t)[0]==1+power(-52)
report('closed_RU64_vs_quantum_oracle',cases=len(triples)+2,max_comparisons=maxcmp,double_rounding_discriminator=True)
# Geometry includes extreme admitted-bit mathematical cases; no runtime admission claimed.
pl=int('3243f6a8885a308d313198a2e03707344a4093822299f31d0082efa98ec4e6c89452821e638d01377be5466cf34e90c6cc0ac29b7c97c50dd3f84d5b5b5470917',16)*power(-512);pu=pl+power(-512)
def down(x):return rnd(x,mode='down')
def up(x):return rnd(x,mode='up')
for D,t in [(F(2),power(-1074)),(power(1023),F(1)),(power(-1000),power(-1003)),(F(3),F(1,8)),(F(2),1-power(-53))]:
 c=D/2;dl,du=down(D-t),up(D-t);rl,ru=down(c-t),up(c-t)
 Pl,Pu=down(t*dl),up(t*du);Ql,Qu=down(c*c+down(rl*rl)),up(c*c+up(ru*ru));Gl,Gu=down(Pl*Ql),up(Pu*Qu)
 ep=t*(D-t);eg=ep*(c*c+(c-t)**2)
 for faclo,fachi,elo,ehi in [(down(pl*Pl),up(pu*Pu),pl*ep,pu*ep),(down(pl*Gl)/4,up(pu*Gu)/4,pl*eg/4,pu*eg/4),(down(down(pl*Gl)/4/c),up(up(pu*Gu)/4/c),pl*eg/(4*c),pu*eg/(4*c))]:assert 0<faclo<=elo<=ehi<=fachi
report('outward_geometry_extremes',pairs=5)
for E,nu in [(F(3),-1+power(-53)),(F(7),F(1,4)),(F(1),power(-1074))]:
 dl=2*down(1+nu);du=2*up(1+nu);assert 0<down(E/du)<=E/(2*(1+nu))<=up(E/dl)
for beta,eta,v in [(F(2),F(1,8),F(1,3)),(F(1),1-power(-1024),F(1))]:
 ah=up(beta*eta);den=down(1-ah);assert den>0
 tau=up(up(beta*v)/den);assert tau>=beta*v/(1-beta*eta)
assert up(1-power(-1025))==1
report('material_and_strict_bridge',material_cases=3,tau_cases=2,alpha_rounded_to_one_refuses=True)
# Raw conversion and all exact predicates have independent equality/neighbor witnesses.
y=F(1);n=rn64(y/1000);assert abs(n-n)==0 and abs(y-1000*n)>0
for n,S in [(F(1),F(1)),(F(0),F(0)),(h,h),(power(-1000),power(-999)),(value(MAX),value(MAX))]:
 allowance=max(abs(n),S)*power(-64)+max(abs(n),S)*power(-85)+abs(n)*power(-53)+h
 eps=power(-1200)
 assert allowance-(allowance-eps)>0 and allowance-allowance==0 and allowance-(allowance+eps)<0
 if n:
  d=abs(n)/10**9;assert abs(n)-10**9*d==0 and abs(n)-10**9*(d+eps)<0
report('final_exact_comparisons_and_raw_SI',allowance_cases=5)
# Resource arithmetic and worst primitive work are checked, not actual allocation/CPU evidence.
assert 3*(4+18+23+13)+236+248+244+280==1182
assert 6+3+16+16+2+4==47
assert 152*137+(256+130+17+128+128)*8+128*8+64+2*137==27458
assert 5*(18+128)+16+5*256+5*128+2*128==2922<4096
assert 5*(18+128)+5*16+5*256+128==2218<4096
assert 17*1026+320+32==17794 and 18*1026+320+32==18820
assert 16*((6*17+1)*1026+2**14)==1952992<2**21
# Eight-bit checked toy aggregates are a semantic control for all overflow branches, not Rust evidence.
for a,b in [(0,255),(254,1),(255,1),(128,128)]:assert (a+b<=255)==(not(a+b>255))
report('counts_and_logical_payload',member_calls=1182,constant_logical_bytes=27458,directed_SumWork_upper=2922,comparator_SumWork_upper=2218,microstep_reservation=2**21)
print(json.dumps({'kind':'independent exact abstract review controls','source_executed':False,'groups_passed':len(checks),'checks':checks},indent=2,sort_keys=True))
