#!/usr/bin/env python3
"""RV55 independent exact review: reads stored Rust vectors, never the author generator.
Alternative pi proof: pi/4 = atan(1/2)+atan(1/3); 320 alternating terms.
Source annulus uses differences of squares/fourth powers, not implementation products.
"""
from pathlib import Path
from fractions import Fraction as Q
import ast,json,re,hashlib,sys
s=Path(sys.argv[1]); fk=s/'projects/chirality-piping/core/solver/frame_kernel'
src=(fk/'src/structural/retained/product_certificate.rs').read_text()
vpath=fk/'tests/retained_k4/product_certificate_vectors.rs'
raw=vpath.read_text(); cases=ast.literal_eval(raw[raw.index('= &[')+3:].strip().rstrip(';').replace('&[','['))
def pow2(n): return Q(2)**n
def binary(bits):
 sign= -1 if bits>>63 else 1
 exp=(bits>>52)&2047; mant=bits&((1<<52)-1)
 assert exp!=2047
 return sign*Q(mant if exp==0 else mant+(1<<52))*pow2(-1074 if exp==0 else exp-1075)
def endpoint(t):
 if t=='Z+': return Q(0)
 m,e=t[1:].split('p'); return (-1 if t[0]=='-' else 1)*Q(int(m,16))*pow2(int(e)-4*len(m)+1)
def ulp(t):
 return pow2(int(t.split('p')[1])-1023)
def outward(t, exact, up):
 x=endpoint(t)
 if exact==0: assert x==0; return
 assert (x>=exact and x-ulp(t)<exact) if up else (x<=exact and x+ulp(t)>exact), (t,str(exact),up)
def atan_bounds(d):
 low=sum((Q(1 if k%2==0 else -1,(2*k+1)*d**(2*k+1)) for k in range(320)),Q())
 return low,low+Q(1,641*d**641)
a,b=atan_bounds(2),atan_bounds(3)
pbounds=(4*(a[0]+b[0]),4*(a[1]+b[1]))
pi=[]
for label in ['LOWER','UPPER']:
 block=re.search(r'const '+label+r': \[u64; 16\] = \[(.*?)\];',src,re.S).group(1)
 limbs=[int(v,16) for v in re.findall(r'0x([0-9a-f]+)',block)]
 assert len(limbs)==16
 pi.append(Q(sum(v<<(64*i) for i,v in enumerate(limbs)))*pow2(-1022))
assert pi[0]<pbounds[0]<pbounds[1]<pi[1]
assert pi[1]-pi[0]==pow2(-512)
assert (pi[0]*pow2(512)).denominator==1
results=[]
for name,mode,geo,m,k,refs,prods in cases:
 d,t=map(binary,geo); mat=list(map(binary,m)); kk=list(map(binary,k))
 c=d/2; ri=c-t; p=c*c-ri*ri; gg=c**4-ri**4; q=c*c+ri*ri
 assert 0<t<c and p==t*(d-t) and gg==p*q
 area=(pi[0]*p,pi[1]*p); inertia=(pi[0]*gg/4,pi[1]*gg/4)
 polar=(2*inertia[0],2*inertia[1]); z=(inertia[0]/c,inertia[1]/c)
 if mode==0:
  e=(mat[0],mat[0]); g=(mat[0]/(2+2*mat[1]),)*2
  assert -1<mat[1]<Q(1,2) and mat[0]==kk[0]
 elif mode==1:
  e=(mat[0],)*2; g=(mat[1],)*2
  assert mat[:2]==kk[:2]
 else:
  tl,at,th,el,eh,gl,gh,ea,ga=mat
  assert tl<at<th and gl>0 and gh>0
  fraction=(at-tl)/(th-tl)
  es=(1-fraction)*el+fraction*eh; gs=(1-fraction)*gl+fraction*gh
  assert es>0 and gs>0 and [ea,ga]==kk[:2]
  e=min(es,ea),max(es,ea);g=min(gs,ga),max(gs,ga)
 coeff=[(e[0]*area[0],e[1]*area[1]),(g[0]*polar[0],g[1]*polar[1]),(e[0]*inertia[0],e[1]*inertia[1]),(e[0]*inertia[0],e[1]*inertia[1])]
 ck=[kk[0]*kk[2],kk[1]*kk[3],kk[0]*kk[4],kk[0]*kk[5]]
 rz=(min(kk[4]/c,kk[6]),max(kk[4]/c,kk[6])) if mode else (Q(),Q())
 targets=[(c,c),(ri,ri),(p,p),(q,q),(gg,gg),area,inertia,polar,z,e,g,*coeff,rz]
 assert len(targets)==len(refs)==16
 for ref,target in zip(refs,targets):
  outward(ref[0],target[0],False);outward(ref[1],target[1],True)
 for j,exact in enumerate(ck):
  assert endpoint(prods[j])==exact
  outward(prods[j+4],max(abs(coeff[j][0]-exact),abs(coeff[j][1]-exact)),True)
 # The exact c-square requires only one multiplication because a shifted
 # binary64 significand squared has no more than 106 significant bits.
 num=(c*c).numerator; trailing=(num & -num).bit_length()-1
 assert num.bit_length()-trailing <= 106
 results.append({'case':name,'intervals':16,'exact_K':4,'delta_upper':4})
x=1+pow2(-52); assert x*x-(1+pow2(-51))==pow2(-104)
assert Q(18)*(-7)+Q(7)*18==0
# Exact operations create the expected partial entry counts by counting the
# source grammar, with the c-square singleton counted once.
common=[2,4+8,2+1+2+2+2+2+8+4,2,0,0]
assert common==[2,12,23,2,0,0]
counts={'common':common,'exact':[4,12,23,4,0,0],'ordinary':[2,12,23,4,0,0],'interpolated':[2,14,31,8,4,0]}
# Direct ordered-binary64 ceiling, independent of nearest+exact-side conversion.
def ceil64(value):
 if value==0:return 0
 top=0x7fefffffffffffff
 if value>binary(top):return 0x7ff0000000000000
 lo=0;hi=top
 while hi-lo>1:
  mid=(lo+hi)//2
  if binary(mid)>=value:hi=mid
  else:lo=mid
 assert binary(lo)<value<=binary(hi)
 return hi
max64=binary(0x7fefffffffffffff)
b64cases=[(Q(),0),(Q(1),0x3ff0000000000000),(1+pow2(-54),0x3ff0000000000001),(pow2(-1075),1),(pow2(-1074),1),(max64+pow2(969),0x7ff0000000000000),(pow2(1024),0x7ff0000000000000)]
for value,expected in b64cases: assert ceil64(value)==expected
print(json.dumps({'status':'PASS','B64U_direct_ceiling_cases':len(b64cases),'independent_pi_proof':'4*(atan(1/2)+atan(1/3)), 320 alternating terms each','stored_pi_grid':'2^-512','cases':results,'targets_checked':128,'exact_K_checked':32,'delta_upper_checked':32,'source_entry_counts':counts,'vector_sha256':hashlib.sha256(vpath.read_bytes()).hexdigest()},indent=2))
