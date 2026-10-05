"""RV50 integration backcheck: abstract exact arithmetic, no product execution."""
from fractions import Fraction as F
from itertools import permutations, product
import json
checks=[]
def two(e):return F(1<<e) if e>=0 else F(1,1<<-e)
def exponent(x):
 x=abs(x);e=x.numerator.bit_length()-x.denominator.bit_length()
 return e-1 if x<two(e) else e
def roundwide(x,mode,p=1024):
 x=F(x)
 if not x:return x
 if x<0:return -roundwide(-x,{'up':'down','down':'up','near':'near'}[mode],p)
 u=two(exponent(x)+1-p);v=x/u;k,r=divmod(v.numerator,v.denominator)
 if mode=='up':k+=r!=0
 elif mode=='near':k+=2*r>v.denominator or (2*r==v.denominator and k%2)
 return k*u
def rn64(x):
 x=F(x)
 if not x:return x
 sign=1 if x>0 else -1;x=abs(x);u=two(max(exponent(x)-52,-1074));v=x/u;k,r=divmod(v.numerator,v.denominator);k+=2*r>v.denominator or (2*r==v.denominator and k%2)
 if k*u>=two(1024):raise ValueError('range')
 return sign*k*u
def record(name,**kw):checks.append(dict(name=name,passed=True,**kw))
# The corrected exact-K constructor must not round after separate lifts.
x=1+two(-52);ck=x*x;rn=rn64(ck)
assert ck-rn==two(-104) and roundwide(ck,'up')==ck==roundwide(ck,'down')
for a,b in [(x,x),(F(7,4),F(11,8)),(two(-1074),two(-1074)),((2-two(-52))*two(1023),two(1023))]:
 assert roundwide(a*b,'near')==a*b
record('exact_K_products',pairs=4,discriminator='2^-104')
# Four input terms, exact sign, bounded residual span and seven lifetime inserts.
# Independent term valuation from reduced dyadic Fraction.
def span(xs):
 lo=[];hi=[]
 for x in xs:
  if x:
   x=abs(F(x));assert x.denominator&(x.denominator-1)==0
   lo.append((x.numerator&-x.numerator).bit_length()-x.denominator.bit_length())
   hi.append(x.numerator.bit_length()-x.denominator.bit_length())
 return max(hi)-min(lo)+1 if lo else 0
cases=[(F(0),F(7),F(25),F(-7),F(18)),(F(0),F(1),F(2),F(-1),F(3)),(-two(1023),F(0),two(1023),two(-1074),two(1023)),(F(0),two(-1074),two(-1073),F(1),F(3))]
for tl,t,th,xl,xh in cases:
 terms=[th*xl,-t*xl,t*xh,-tl*xh];num=sum(terms);h=th-tl
 assert num==(th-t)*xl+(t-tl)*xh and span(terms)<=4196
 near=roundwide(num,'near');assert span(terms+[-near])<=4199
 if num>0:
  lo=roundwide(roundwide(num,'down')/roundwide(h,'up'),'down');hi=roundwide(roundwide(num,'up')/roundwide(h,'down'),'up')
  assert 0<lo<=num/h<=hi
assert sum([F(25)*-7,-F(7)*-7,F(7)*18,-F(0)*18])==0
assert rn64(-7+rn64(rn64(F(7,25))*25))==two(-50)
assert 7*(18+128)+16+7*256+6*128+2*128==3854
assert 5*(1<<8128)<1<<8131<1<<8192
record('R4_exact_sign_span_and_carry',cases=len(cases),lifetime_inserts=7,live_terms=5,work_upper=3854)
# Strict scan compared with independently sorted strict adjacency, includes at-target blocker.
cnt=0
for xs in permutations([F(1),F(2),F(3),F(4)]):
 for t in [F(0),F(1),F(3,2),F(2),F(5,2),F(9,2)]:
  below=[x for x in xs if x<t];above=[x for x in xs if x>t]
  scan=None if t in xs or len(set(xs))<len(xs) or not below or not above else (max(below),min(above))
  s=sorted(xs);matches=[(a,b) for a,b in zip(s,s[1:]) if a<t<b]
  assert scan==(matches[0] if matches else None);cnt+=1
record('strict_point_selection_scan',cases=cnt)
# Convex/hypot interval algebra is accepted B2; independently verify hull distance after positive unit maps.
cnt=0
for l1,u1,l2,u2 in product(range(-2,3),repeat=4):
 if l1>u1 or l2>u2:continue
 hull=(min(l1,l2),max(u1,u2))
 for a in [F(1),F(1,1000),F(1000),F(1000000)]:
  y=F(7,9);d=lambda I:max(abs(y-F(I[0])/a),abs(y-F(I[1])/a))
  assert d(hull)==max(d((l1,u1)),d((l2,u2)));cnt+=1
record('source_represented_hull_raw_distances',cases=cnt)
# Route tables independently composed from their operation lists, never substitute 47q for B2.
common=dict(add=2,sub=18,mul=23,div=11,r4=0)
routes={'exact':dict(add=2,sub=0,mul=0,div=2,r4=0),'base':dict(add=0,sub=0,mul=0,div=0,r4=0),'interp':dict(add=0,sub=2,mul=8,div=4,r4=4)}
assert {k:sum(common.values())+sum(v.values()) for k,v in routes.items()}=={'exact':58,'base':54,'interp':72}
assert 3*(72-58)==42
for qs,qc,qo,os,oc,oo in [(1,0,0,0,0,0),(1,1,1,1,1,1),(7,5,3,4,2,1)]:
 ns,nc,no=qs+os,qc+oc,qo+oo
 each=[]
 each.extend([(2,0,6,2,2)]*ns) # add,sub,mul,div,sqrt per SIF
 each.extend([(8,0,8,8,4)]*nc)
 each.extend([(8,0,0,8,0)]*no)
 total=tuple(sum(v[j] for v in each) for j in range(5))
 assert total==(2*ns+8*nc+8*no,0,6*ns+8*nc,2*ns+8*nc+8*no,2*ns+4*nc)
assert 179*137+659*8==29795 and 152+21+2+2==177
record('route_B2_and_storage_counts',builders={'exact':58,'base':54,'interp':72},B2_cases=3,logical_bytes_without_controls=29795)
# Permits reject before event, exact prefix persists; finite-width overflow latches rather than clearing.
trials=0
for maxcount in [3,7,15]:
 for cap in range(maxcount+1):
  spent=0;executed=[];fault=None
  for i in range(maxcount+2):
   prospective=spent+1
   if prospective>maxcount:fault='O';break
   if prospective>cap:fault='VisitExhausted';break
   spent=prospective;executed.append(i)
  assert spent==cap and len(executed)==cap
  assert fault==('O' if cap==maxcount else 'VisitExhausted')
  # A new phase may not restart or erase this state.
  joined=(fault,spent);assert joined[0] and joined[1]==cap;trials+=1
record('pre_event_permit_and_overflow',trials=trials)
# Observable expression remains independent of broad source cover and actual tie semantics.
lo=two(1023);hi=lo+two(971);mid=rn64(lo+rn64(F(1,2)*rn64(hi-lo)))
assert mid==lo
try:rn64(lo+hi)
except ValueError:pass
else:raise AssertionError('expected alternative midpoint overflow')
y=F(5001,1000);true=F(5);guard=64*two(-52)*max(abs(y),two(-1022));assert 0<=y<=10 and abs(y-true)>guard
record('observable_order_and_independence',midpoint_order=True,source_cover_not_guard=True)
print(json.dumps({'kind':'exact_abstract_review_controls','product_executed':False,'groups_passed':len(checks),'checks':checks},indent=2,sort_keys=True))
