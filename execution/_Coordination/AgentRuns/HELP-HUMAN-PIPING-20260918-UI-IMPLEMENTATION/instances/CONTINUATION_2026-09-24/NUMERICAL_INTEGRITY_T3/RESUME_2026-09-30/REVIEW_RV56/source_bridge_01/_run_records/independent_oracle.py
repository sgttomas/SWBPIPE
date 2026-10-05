from fractions import Fraction as F
from pathlib import Path
import struct,math,json,re
OUT=Path(__file__).parent

def bitreal(h): return F(struct.unpack('>d',bytes.fromhex(h))[0])
def wide(w): return (-1 if w[0] else 1)*F(int(w[2],16))*F(2)**(w[1]-1023)
def atan_bounds(q,n=192):
 s=sum(((-1)**k*F(1,(2*k+1)*q**(2*k+1)) for k in range(n)),F(0))
 return s,s+F(1,(2*n+1)*q**(2*n+1))
a,b=atan_bounds(5),atan_bounds(239)
pi=(16*a[0]-4*b[1],16*a[1]-4*b[0])
assert pi[0]<pi[1]
D,T,E,G=F(0.1),F(0.005),F(210e9),F(80e9)
# Direct annular integral: A=pi*(Ro^2-Ri^2), I=pi*(Ro^4-Ri^4)/4.
ro,ri=D/2,D/2-T
area=tuple(q*(ro**2-ri**2) for q in pi)
inertia=tuple(q*(ro**4-ri**4)/4 for q in pi)
j=tuple(2*v for v in inertia)
# Independently derive deterministic K bits by rounding the tight analytic bracket.
A,I,J=map(lambda v:F(float(sum(v)/2)),[area,inertia,j])
def reciprocal(pair,scale):return (1/(scale*pair[1]),1/(scale*pair[0]))
gmotion=[reciprocal(area,E),reciprocal(inertia,3*E),(F(0),F(0)),reciprocal(j,G),(F(0),F(0)),reciprocal(inertia,2*E)]
kmotion=[(F(1)/(E*A),)*2,(F(1)/(3*E*I),)*2,(F(0),)*2,(F(1)/(G*J),)*2,(F(0),)*2,(F(1)/(2*E*I),)*2]
def log2(x):
 e=x.numerator.bit_length()-x.denominator.bit_length()
 return e-(x<F(2)**e)
def sqrt_i(pair):
 def sq(x,up):
  if not x:return F(0)
  e=log2(x)//2-640;v=x/F(2)**(2*e);m=math.isqrt(v.numerator//v.denominator)
  if up and F(m*m)<v:m+=1
  return m*F(2)**e
 return sq(pair[0],False),sq(pair[1],True)
def mag(m):return sqrt_i(tuple(sum(v[k]**2 for v in m[:3]) for k in range(2)))
components=['Ux','Uy','Uz','Rx','Ry','Rz']
def truth(i,loaded,geometric):
 if not loaded:return (F(0),)*2
 m=gmotion if geometric else kmotion
 if i<6:return (F(0),)*2
 if i<12:return m[i-6]
 if i==12:return (F(0),)*2
 if i==13:return mag(m)
 if i<26:
  at=i-14
  v=([-1,-1,0,-1,0,-1]+[1,1,0,1,0,0])[at]
  return (F(v),)*2
 if i<44:
  at=i-26;station,c=divmod(at,6);t=[F(0),F(1,4),F(1)][station]
  return ((F(1) if c in [0,1,3] else 1-t if c==5 else F(0)),)*2
 if i<50:return (F([-1,-1,0,-1,0,-1][i-44]),)*2
 return sqrt_i((F(2),F(2)))
counts={}
for line in (OUT/'candidate_bridge.stdout').read_text().splitlines():
 if 'I42_ROW ' not in line:continue
 d=json.loads(line.split('I42_ROW ',1)[1]);idx=d['index'];loaded=d['witness']=='loaded';x=bitreal(d['x']);lo,hi=wide(d['lo']),wide(d['hi']);g=truth(idx,loaded,True);k=truth(idx,loaded,False)
 assert lo<=g[0]<=g[1]<=hi,(idx,'source containment')
 assert wide(d['glo'])<=g[0]<=g[1]<=wide(d['ghi']),(idx,'independent G vs author vectors')
 assert wide(d['klo'])<=k[0]<=k[1]<=wide(d['khi']),(idx,'independent K vs author vectors')
 if d['radius']!='absent':
  r=bitreal(d['radius']);assert x-r<=k[0]<=k[1]<=x+r,(idx,'K private radius')
 err=max(abs(x-g[0]),abs(x-g[1]));h=max(abs(x-lo),abs(x-hi))
 if d['class']=='input': allowance=F(0)
 elif d['class']=='absolute':allowance=bitreal(d['bound'])
 else:
  scale=bitreal(d['scale']);exact=F(2)**-64*max(abs(x),scale)*(1+F(2)**-21)+F(2)**-53*abs(x)+F(2)**-1074
  allowance=min(exact,bitreal(d['sharper']),abs(x)/10**9)
 assert err<=allowance,(idx,'independent exact-source predicate')
 c=counts.setdefault(d['witness'],{'rows':0,'source_contains':0,'exact_pass':0,'enclosure_pass':0})
 c['rows']+=1;c['source_contains']+=1;c['exact_pass']+=1;c['enclosure_pass']+=h<=allowance
result={'derivation':'Independent Machin 16 atan(1/5)-4 atan(1/239); exact annular integrals and closed cantilever/spring equations; no evaluator/generator imports','machin_terms_per_atan':192,'native':counts}
# New reviewer control, L=2 along global +Y, tip Fy=1, spring k=100000.
# U=1/(EA/L+k), axial member force=1-k*U, spring=-k*U.
stiffness=F(100000);u=tuple(1/(E*v/2+stiffness) for v in area[::-1]);force=(1-stiffness*u[1],1-stiffness*u[0]);spring=(-stiffness*u[1],-stiffness*u[0])
def endpoint(x,up):
 if not x:return '(false, 0, [0;16])'
 sign=x<0;quantum=F(2)**(log2(abs(x))-767);z=x/quantum;m=z.numerator//z.denominator
 if up and F(m)!=z:m+=1
 v=m*quantum;sign=v<0;v=abs(v);e=log2(v);sig=v/F(2)**(e-1023);assert sig.denominator==1
 limbs=[(sig.numerator>>(64*k))&((1<<64)-1) for k in range(16)]
 return '('+str(sign).lower()+','+str(e)+',['+','.join(hex(k) for k in limbs)+'])'
def pair(p):return '('+endpoint(p[0],False)+','+endpoint(p[1],True)+')'
code='// RV56 exact independent rotated L=2 axial+spring witness.\n'
for name,v in [('U',u),('MEMBER',force),('SPRING',spring)]:code+='pub const '+name+': ((bool,i64,[u64;16]),(bool,i64,[u64;16])) = '+pair(v)+';\n'
(OUT.parent/'imported/tests/retained_k4/review_vectors.rs').write_text(code)
(OUT/'independent_oracle.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result,indent=2))
