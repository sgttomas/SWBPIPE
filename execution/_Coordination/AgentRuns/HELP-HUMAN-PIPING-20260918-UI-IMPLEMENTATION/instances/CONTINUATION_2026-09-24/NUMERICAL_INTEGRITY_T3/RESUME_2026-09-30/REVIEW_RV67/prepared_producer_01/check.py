"""RV67 independent finite analytical check. No solver/product import or execution.
Read pinned actual capture and author's claimed results, derive targets independently,
and write only a caller-specified NEW output file. Python standard library only.
"""
from fractions import Fraction as Q
from pathlib import Path
import math, struct, hashlib, json, sys
ROOT=Path('/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3')
CAP=ROOT/'scratch/i50_first_publishing/runtime02/pp_named_dense_completion.log'
CLAIM=ROOT/'scratch/i51_replacement_producer/control_results.json'
OUT=Path(sys.argv[1])
assert not OUT.exists()
def power(k): return Q(2**k) if k>=0 else Q(1,2**-k)
def bits(x): return struct.pack('>d',x).hex()
def decode(s): return struct.unpack('>d',bytes.fromhex(s))[0]
def exp(q):
    q=abs(Q(q)); assert q
    e=q.numerator.bit_length()-q.denominator.bit_length()
    return e-1 if q<power(e) else e
def rn(q):
    q=Q(q)
    if not q:return 0.
    sg=-1 if q<0 else 1; q=abs(q); k=max(exp(q)-52,-1074)
    v=q/power(k); n,r=divmod(v.numerator,v.denominator)
    if 2*r>v.denominator or (2*r==v.denominator and n%2):n+=1
    x=math.ldexp(float(n),k)*sg
    assert math.isfinite(x)
    return x
def rnp(q,p):
    q=Q(q)
    if not q:return q
    k=exp(q)-p+1; v=q/power(k); n,r=divmod(v.numerator,v.denominator)
    if 2*r>v.denominator or (2*r==v.denominator and n%2):n+=1
    return n*power(k)
def dirq(q,p,up):
    q=Q(q)
    if not q:return q
    k=exp(q)-p+1; t=q/power(k)
    n=-((-t.numerator)//t.denominator) if up else t.numerator//t.denominator
    return n*power(k)
def point(q):return Q(q),Q(q)
def sqrti(q,p=320):
    q=Q(q); k=math.isqrt((q.numerator<<(2*p))//q.denominator)
    a=Q(k,1<<p); b=a if a*a==q else Q(k+1,1<<p)
    assert a*a<=q<=b*b
    return a,b
def hull(*v):return min(x[0] for x in v),max(x[1] for x in v)
def atan(q):
    a=sum((Q((-1)**i,(2*i+1)*q**(2*i+1)) for i in range(320)),Q(0))
    return a,a+Q(1,641*q**641)
a,b=atan(2),atan(3)
# tan(atan(1/2)+atan(1/3))=1, both angles positive, sum < 1 < pi/2.
pl,pu=4*(a[0]+b[0]),4*(a[1]+b[1])
assert pu-pl<power(-630)
pn=(pl*2**512).__floor__(); assert (pu*2**512).__ceil__()==pn+1
pi=Q(pn,2**512),Q(pn+1,2**512)
assert hex(pn)[2:]=='3243f6a8885a308d313198a2e03707344a4093822299f31d0082efa98ec4e6c89452821e638d01377be5466cf34e90c6cc0ac29b7c97c50dd3f84d5b5b5470917'
raw=CAP.read_bytes(); assert len(raw)==222045 and hashlib.sha256(raw).hexdigest()=='d762d4dcc137e0bff9e739fb0a829fa086055ea9492f5b55f765ac5801c6dde7'
caps=[json.loads(l.split('I50_RECORD ',1)[1]) for l in raw.decode().splitlines() if 'I50_RECORD ' in l]
claimed=CLAIM.read_bytes(); assert hashlib.sha256(claimed).hexdigest()=='574526effdd78a1902c1a28e96efbc8668a6ccc12e8d26e4989c517ca20ebccb'
claim=json.loads(claimed)
assert len(caps)==2 and caps[0]['request']==caps[1]['request'] and caps[0]['source']==caps[1]['source']
model=caps[0]['request']['model']; section=model['pipe_segments'][0]['section']; facts=caps[0]['facts'][0]; src=caps[0]['source']
assert len(model['nodes'])==2 and len(model['pipe_segments'])==1 and len(model['load_cases'])==1
assert model['load_cases'][0]['label']=='RF-SKEW-T-CANT-OFF-122-r1e-04'
assert src['nodes']==[[0,0,0],[1,2,2]] and src['members'][0]['nodes']==[0,1]
assert src['members'][0]['y_reference']==[1,0,0]
D,t=Q(section['outside_diameter']['value']),Q(section['wall_thickness']['value'])
assert D==Q(facts['D']) and t==Q(facts['t']); assert bits(float(D))=='3fc999999999999a' and bits(float(t))=='3f847ae147ae147b'
c=D/2; r=c-t; g=c**4-r**4; area_factor=c*c-r*r
assert 0<t<c and area_factor==t*(D-t)
geometry={key:tuple(x*factor for x in pi) for key,factor in [('A',area_factor),('I',g/4),('J',g/2),('Z',g/(4*c))]}
new={k:rn(v[0]) for k,v in geometry.items()}
for k,v in geometry.items(): assert bits(rn(v[0]))==bits(rn(v[1]))==claim['new_primitives'][k]
# Independently mimic only the proposed existing positive directed geometry schedule.
entries={'sub':0,'mul':0,'add':0,'div':0}
def op(a,b,name):
    entries[name]+=2
    if name=='sub': z=(a[0]-b[1],a[1]-b[0])
    elif name=='mul':z=(a[0]*b[0],a[1]*b[1])
    elif name=='add':z=(a[0]+b[0],a[1]+b[1])
    else:z=(a[0]/b[1],a[1]/b[0])
    return dirq(z[0],1024,False),dirq(z[1],1024,True)
rr=op(point(c),point(t),'sub'); dt=op(point(D),point(t),'sub'); p=op(point(t),dt,'mul')
entries['mul']+=1; cc=point(c*c); rr2=op(rr,rr,'mul'); qq=op(cc,rr2,'add'); gg=op(p,qq,'mul')
AA=op(pi,p,'mul'); II=tuple(z/4 for z in op(pi,gg,'mul')); JJ=tuple(z*2 for z in II); ZZ=op(II,point(c),'div')
for k,v in zip(('A','I','J','Z'),(AA,II,JJ,ZZ)):
 assert v[0]<=geometry[k][0]<=geometry[k][1]<=v[1]
 assert bits(rn(v[0]))==bits(rn(v[1]))==bits(new[k])
assert entries==dict(sub=4,mul=11,add=2,div=2)
E=Q(model['materials'][0]['elastic_modulus']['value']); G=Q(model['materials'][0]['shear_modulus']['value'])
assert E==200000000000 and G==80000000000
assert Q(src['members'][0]['E'])==E and Q(src['members'][0]['G'])==G
loads=model['load_cases'][0]['primitive_loads']; assert [l['direction'] for l in loads]==['RX','RY','RZ']
assert all(l['category']=='concentrated_moment' and l['target']=={'node':'N1','type':'node'} for l in loads)
m=[Q(l['magnitude']['value']) for l in loads]; assert m==[m[0],2*m[0],2*m[0]]
ks=[Q(s['stiffness']['value']['value']) for s in model['supports'][1:]]; assert ks==[144,1000000,1000000]
assert [s['k'] for s in src['springs']]==[144,1000000,1000000]
w=[a/b for a,b in zip(m,ks)]; d=[Q(1),Q(2),Q(2)]; L=Q(3); T=3*m[0]
u=[w[1]*d[2]-w[2]*d[1],w[2]*d[0]-w[0]*d[2],w[0]*d[1]-w[1]*d[0]]
# Equilibrium: root spring moments cancel tip moment. Compatible rigid rotation
# plus relative twist yields zero axial/bending strain and tip torque T e1.
assert all(ks[i]*w[i]==m[i] for i in range(3)); assert sum(x*x for x in d)==L*L
J=Q(new['J']); rotk=[point(w[i]+T*d[i]/(G*J)) for i in range(3)]
rots=[(w[i]+T*d[i]/(G*geometry['J'][1]),w[i]+T*d[i]/(G*geometry['J'][0])) for i in range(3)]
sk=point(T*c/J); ss=(T*c/geometry['J'][1],T*c/geometry['J'][0]); umag=sqrti(sum(x*x for x in u))
unit={'mm':Q(1,1000),'rad':Q(1),'N':Q(1),'N*m':Q(1),'Pa':Q(1),'MPa':Q(1000000)}
def target(row):
 k,e,z=row['kind'],row['entity_ref'],row.get('metadata') or {}; inp=False
 if k in ('linear_solver_mode_basis','sparse_live_path_dense_parity_relative_delta'):return None,'nonquantity',False
 if k.startswith('global_nodal_'):
  axis='xyz'.index(k[-1]); root=e=='N0'; assert e in ('N0','N1')
  if 'displacement' in k:return point(0 if root else u[axis]),'translation',root
  return point(w[axis]) if root else hull(rotk[axis],rots[axis]),'rotation',False
 if k=='displacement_magnitude':return point(0) if e=='N0' else umag,'translation',False
 if k.startswith('support_reaction_'):
  val=Q(0); comp=z['component']
  if e.startswith('spring:'):
   axis=int(e[-1]); val=(-m[axis] if comp=='M'+'xyz'[axis] else m[axis] if comp=='moment_magnitude' else Q(0))
  else:assert e=='rigid:N0'
  return point(val),'force' if row['unit']=='N' else 'moment',False
 if k=='element_local_torsional_moment':return point(-T if z['location']=='end_i' else T),'moment',False
 if k=='element_local_torsional_shear_stress':return hull(sk,ss),'stress',False
 if k=='pipe_elastic_normal_stress_maximum_v2':return point(0),'maximum',False
 assert k.startswith('element_local_')
 return point(0),'stress' if 'stress' in k else 'force' if row['unit']=='N' else 'moment',False

def b64add(a,b):return rn(Q(a)+Q(b))
def b64mul(a,b):return rn(Q(a)*Q(b))
def b64div(a,b):return rn(Q(a)/Q(b))
def allow(n,s):
 n=abs(Q(n)); s=Q(s); a=power(-64)*max(n,s)*(1+power(-21))+power(-53)*n+power(-1074)
 a0=rn(power(-64)*max(n,s)); a1=rn(Q(a0)*(1+power(-21)))
 u0=rn(power(-53)*n); u1=rn(Q(u0)+power(-1074)); af=Q(rn(Q(a1)+Q(u1)))
 return min(a,af)
summary=[]; allrows=[]
for cap,co in zip(caps,claim['captures']):
 assert cap['mode']==co['mode']; rows=[]
 for rawrow in cap['envelope']['results']:
  iv,kind,inp=target(rawrow)
  if iv is None:
   cr=next(r for r in co['rows'] if r['id']==rawrow['id'])
   assert cr['class']=='NonQuantity' and cr['raw_bits']==bits(rawrow['value'])
   continue
  factor=unit[rawrow['unit']]; y=rn(sum(iv)/(2*factor)); n=rn(Q(y)*factor)
  mid=rnp(sum(iv),1024)/2; projected=rn(rnp(mid/factor,1024))
  assert bits(projected)==bits(y)
  h=max(abs(Q(n)-q) for q in iv); hu=max(abs(Q(y)-q/factor) for q in iv)
  rows.append(dict(id=rawrow['id'],kind=kind,inp=inp,iv=iv,y=y,n=n,h=h,hu=hu,unit=factor))
 prim={k:max(abs(r['n']) for r in rows if r['kind']==k and not r['inp']) for k in ('translation','rotation','force','moment')}
 tr,ro,fo,mo=[prim[k] for k in ('translation','rotation','force','moment')]
 sc=dict(translation=max(tr,b64mul(3,ro)),rotation=max(ro,b64div(tr,3)),force=max(fo,b64div(mo,3)),moment=max(mo,b64mul(3,fo)))
 for key,k in [('stress',1.),('maximum',2*decode('3ff6a09e667f3bcd'))]:sc[key]=b64add(b64div(sc['force'],new['A']),b64mul(k,b64div(sc['moment'],new['Z'])))
 assert {k:bits(v) for k,v in sc.items()}==co['scale_bits']
 counts={}; margin=[]
 for row in rows:
  n,s=row['n'],sc[row['kind']]; h,hu=row['h'],row['hu']; inflate=power(-100)
  if row['inp']:cl='InputDerived'; assert row['iv']==point(n)
  elif s<2.**-988 or abs(n)<rn(power(-34)*Q(s)):
   cl='AbsoluteVerified'; bound=rn(power(-64)*Q(s))
   if Q(bound)<power(-64)*Q(s):bound=math.nextafter(bound,math.inf)
   assert h+inflate<=Q(bound)
  else:
   cl='RelativeVerified'; budget=allow(n,s)
   assert h+inflate<=budget and 10**9*(h+inflate)<=abs(Q(n)) and 10**9*(hu+inflate/row['unit'])<=abs(Q(row['y']))
   margin.append(float((budget-h)/budget))
  counts[cl]=counts.get(cl,0)+1
  cr=next(r for r in co['rows'] if r['id']==row['id'])
  assert cr['class']==cl and cr['raw_bits']==bits(row['y']) and cr['normalized_bits']==bits(n)
  allrows.append({'mode':cap['mode'],'id':row['id'],'class':cl,'raw_bits':bits(row['y']),'normalized_bits':bits(n),'scale_bits':bits(s),'max_SI_error':str(h),'max_raw_error':str(hu)})
 for support in model['supports']:
  sid=support['id']
  for family,prefix in [('force','F'),('moment','M')]:
   rawgroup=[r for r in cap['envelope']['results'] if r['entity_ref']==sid]
   def selected(comp):
    rid=next(r['id'] for r in rawgroup if r['metadata']['component']==comp)
    return next(r['y'] for r in rows if r['id']==rid)
   mag=selected(family+'_magnitude'); v=[selected(prefix+c) for c in 'xyz']
   assert sum(Q(x)**2 for x in v)==Q(mag)**2
 assert counts==co['class_counts'] and len(rows)==97
 assert len(cap['envelope']['results'])==co['count']
 def component(node,axis):return next(r['n'] for r in rows if r['id']=='result:disp:'+node+':'+axis)
 norms=[]
 for axes in [('ux','uy','uz'),('rx','ry','rz')]:
  endpoints=[b64add(b64add(abs(component(node,axes[0])),abs(component(node,axes[1]))),abs(component(node,axes[2]))) for node in ('N0','N1')]
  norms.append(b64add(*endpoints))
 op=[b64div(b64mul(E,new['A']),3),b64div(b64mul(G,new['J']),3)]
 demands=[]
 for norm,stiff,kind,scale_kind in zip(norms,op,('translation','rotation'),('force','moment')):
  lower=0. if norm<=b64mul(2.**-59,sc[kind]) else b64mul(stiff,rn(Q(norm)-Q(b64mul(2.**-60,sc[kind]))))
  demands.append(max(lower,sc[scale_kind]))
 assert demands==co['g5a_required_upper']
 summary.append({'g5a_required_upper_only':demands,'mode':cap['mode'],'mechanical':len(rows),'total':co['count'],'classes':counts,'scale_bits':co['scale_bits'],'min_fractional_margin':min(margin),'eight_support_norm_equalities':True})
S=Q(sc['stress']); old=T*c/Q(facts['J']); a=power(-64)*(1+power(-21)); b=power(-53); h=power(-1074)
N=(abs(old)+a*S+h)/(1-a-b); B=a*max(N,S)+b*N+h; gap=ss[0]-old-2*B
assert gap>0
# Rebuild canonical old source from the actual input/facts; no runtime source is created.
def source_encoding(props):
 b=bytearray(b'K4SRC\x01')
 def ui(n):b.extend(struct.pack('<I',n))
 def ff(v):b.extend(struct.pack('<d',v))
 def dof(n,c):ui(n);b.append(c)
 ui(2)
 for node in [[0.,0.,0.],[1.,2.,2.]]:
  for v in node:ff(v)
 ui(1);ui(0);ui(0);ui(1)
 for v in [E,G,props['A'],props['I'],props['I'],props['J'],1.,0.,0.]:ff(float(v))
 ui(3)
 for i,k in enumerate(ks):ui(i);dof(0,i+3);ff(float(k))
 ui(0);ui(3)
 for i in range(3):dof(0,i);ff(0.)
 ui(3)
 for i,load in enumerate(loads):
  dof(1,i+3);v=load['id'].encode();ui(len(v));b.extend(v);ff(load['magnitude']['value'])
 ui(3)
 for i,t in enumerate([.25,.5,.75]):ui(i);ui(0);ff(t)
 ui(4)
 for i in range(4):
  ui(i);ui(0);b.extend([1,1,1,0,0,0] if i==0 else [0]*6);ui(0 if i==0 else 1)
  if i:ui(i-1)
  ui(0)
 return bytes(b)
old_encoding=source_encoding(facts); hypothetical=source_encoding(new)
assert all(bytes(cp['native']['source_encoding'])==old_encoding for cp in caps)
assert old_encoding!=hypothetical
# The old D1 binary64 component recipe is a distinct process, demonstrably failing here.
y_old_recipe=b64div(b64div(b64mul(rn(T),rn(c)),new['J']),1e6)
n_old_recipe=b64mul(y_old_recipe,1e6); H_old_recipe=max(abs(Q(n_old_recipe)-v) for v in hull(sk,ss)); old_budget=allow(n_old_recipe,sc['stress'])
assert bits(y_old_recipe)=='3efbf3ab943069f5' and H_old_recipe>old_budget
# A fixed exact two-by-two residual-law control. An arbitrary, even poor center is
# allowed; the posterior residual, not correction quality, determines the bound.
for delta in [Q(0),Q(1,16)]:
 A=[[Q(2)+delta,Q(1)],[Q(1),Q(2)]]; f=[Q(1),Q(2)]; det=A[0][0]*A[1][1]-1
 exact=[(2*f[0]-f[1])/det,(A[0][0]*f[1]-f[0])/det]
 beta=Q(2); eta=delta; alpha=beta*eta; assert alpha<1
 for center in [[Q(0),Q(0)],[Q(50),Q(-7)]]:
  residual=[f[i]-sum(A[i][j]*center[j] for j in range(2)) for i in range(2)]
  radius=beta*max(map(abs,residual))/(1-alpha)
  assert max(abs(exact[i]-center[i]) for i in range(2))<=radius
  # Full changed recovery a=(2+delta,1), never reused admitted a=(2,1).
  source_q=sum(A[0][j]*exact[j] for j in range(2))
  center_q=sum(A[0][j]*center[j] for j in range(2))
  assert abs(source_q-center_q)<=sum(map(abs,A[0]))*radius
out={'canonical_old_source_sha256':hashlib.sha256(old_encoding).hexdigest(),'analytical_new_source_encoding_sha256_not_live':hashlib.sha256(hypothetical).hexdigest(),'precision_recipe_discriminator':{'old_recipe_raw_bits':bits(y_old_recipe),'old_recipe_normalized_bits':bits(n_old_recipe),'old_recipe_H':float(H_old_recipe),'old_recipe_allowance':float(old_budget),'old_recipe_fails':True},'closed_residual_2x2_controls':4,'status':'independent mathematical feasibility verified, not live producer/availability','pi_method':'320 terms each: 4(atan(1/2)+atan(1/3)); production 512-bit endpoints verified','new_bits':{k:bits(v) for k,v in new.items()},'geometry_directed_scalar_entries':entries,'new_K_source_stress_separation':str(hull(sk,ss)[1]-hull(sk,ss)[0]),'old_J_nonoverlap_lower':str(gap),'old_J_nonoverlap_approx':float(gap),'modes':summary,'rows':allrows,'limits':['actual p and residual widths unknown','no new-K G5a evidence','no API/lifetime/resource qualification','coefficient-zero analytical maximum only']}
OUT.write_text(json.dumps(out,indent=2)+'\n')
print(json.dumps({k:v for k,v in out.items() if k!='rows'},indent=2))
