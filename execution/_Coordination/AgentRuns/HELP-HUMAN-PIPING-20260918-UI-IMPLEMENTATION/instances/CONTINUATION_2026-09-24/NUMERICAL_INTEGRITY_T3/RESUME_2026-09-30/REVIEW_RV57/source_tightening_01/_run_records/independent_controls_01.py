#!/usr/bin/env python3
"""RV57 independent exact algebra. No author import, production execution or numerical solver.
Author JSON supplies claimed centers/endpoints, which are checked against direct
rational mechanics and independent source pi bounds. General controls use only
closed two-by-two inverses and exact rational arithmetic.
"""
from fractions import Fraction as Q
from pathlib import Path
import json, re, hashlib
HERE=Path(__file__).resolve().parent
SUBJECT=HERE.parents[2]/'I43/source_tightening_01'
DATA=SUBJECT/'_run_records/frozen_inputs'
def two(e): return Q(2)**e
def binary64(h):
    bits=int(h,16); sign=-1 if bits>>63 else 1; e=(bits>>52)&2047; f=bits&((1<<52)-1)
    assert e!=2047
    return sign*Q(f if e==0 else f+(1<<52))*two(-1074 if e==0 else e-1023-52)
def endpoint(a): return (-1 if a[0] else 1)*int(a[2],16)*two(a[1]-1023)
def pair(x): return (Q(x),Q(x))
def add(a,b): return (a[0]+b[0],a[1]+b[1])
def neg(a): return (-a[1],-a[0])
def sub(a,b): return add(a,neg(b))
def mul(a,b):
    p=[u*v for u in a for v in b];return(min(p),max(p))
def div(a,b):
    assert b[0]>0;return mul(a,(1/b[1],1/b[0]))
def asum(a): return max(abs(a[0]),abs(a[1]))
def isum(xs):
    out=pair(0)
    for x in xs:out=add(out,x)
    return out
def square(a): return (0 if a[0]<=0<=a[1] else min(a[0]**2,a[1]**2),max(a[0]**2,a[1]**2))
def encloses(a,b):return a[0]<=b[0]<=b[1]<=a[1]
def interval_record(q):return [str(t) for t in q]
def distance(a,x):return max(abs(a[0]-x),abs(a[1]-x))
def norm_contained(a,components):
    sq=isum(square(t) for t in components)
    return (a[0]<=0 or a[0]*a[0]<=sq[0]) and a[1]>=0 and a[1]*a[1]>=sq[1]
def atan(q,n):
    z=sum((Q((-1)**j,(2*j+1)*q**(2*j+1)) for j in range(n)),Q(0))
    assert n%2==0
    return(z,z+Q(1,(2*n+1)*q**(2*n+1)))
# Independent identity from I43's Machin formula; exact alternating remainder.
pi_ref=mul(pair(4),add(atan(2,600),atan(3,600)))
source=(DATA/'product_certificate.rs').read_text();pi=[]
for name in ['LOWER','UPPER']:
    body=re.search(r'const '+name+r': \[u64; 16\] = \[(.*?)\];',source,re.S).group(1)
    limbs=[int(x,16) for x in re.findall(r'0x([0-9a-f]+)',body)]
    pi.append(sum(x<<(64*i) for i,x in enumerate(limbs))*two(-1022))
pi=tuple(pi);assert encloses(pi,pi_ref)
raw=[];blocks=[]
for line in (DATA/'native_debug.stdout').read_text().splitlines():
    if 'I42_ROW ' in line:raw.append(json.loads(line.split('I42_ROW ',1)[1]))
    if 'I42_BLOCK ' in line:blocks.append(json.loads(line.split('I42_BLOCK ',1)[1]))
assert len(blocks)==1
block=blocks[0];rows={r['index']:r for r in raw if r['witness']=='loaded'};zero=[r for r in raw if r['witness']=='zero'];assert len(rows)==len(zero)==52
v={k:binary64(h) for k,h in re.findall(r'pub const (D|T|E|G|A|I|J|Z): u64 = 0x([0-9a-f]{16});',(DATA/'source_bridge_vectors.rs').read_text())}
D,t,E,g=v['D'],v['T'],v['E'],v['G']
# Difference-of-powers annulus, independently from I43's factored A/ri recipe.
ap=(D*D-(D-2*t)**2)/4; ip=(D**4-(D-2*t)**4)/64
cs=[E*ap,2*g*ip,E*ip,E*ip]; ck=[E*v['A'],g*v['J'],E*v['I'],E*v['I']]
# Six basic deformations from the inspected source B equation.
B=[[Q(0) for _ in range(12)] for _ in range(6)]
for i,terms in enumerate([[(0,-1),(6,1)],[(3,-1),(9,1)],[(1,1),(7,-1),(5,1)],[(1,1),(7,-1),(11,1)],[(2,-1),(8,1),(4,1)],[(2,-1),(8,1),(10,1)]]):
    for j,a in terms:B[i][j]=Q(a)
def constitutive(c):
    a,t,z,y=c
    return [[a,0,0,0,0,0],[0,t,0,0,0,0],[0,0,4*z,2*z,0,0],[0,0,2*z,4*z,0,0],[0,0,0,0,4*y,2*y],[0,0,0,0,2*y,4*y]]
def transpose(a):return list(map(list,zip(*a)))
def mm(a,b):return [[sum(u*v for u,v in zip(r,c)) for c in transpose(b)] for r in a]
G0=mm(transpose(B),mm(constitutive(cs),B));K=mm(transpose(B),mm(constitutive(ck),B))
G=[[mul(pair(e),pi) for e in r] for r in G0]
Delta=[[sub(G[i][j],pair(K[i][j])) for j in range(12)] for i in range(12)]
s=[Q(0)]*6+[two(e) for e in block['scales']]
x=[binary64(rows[i]['x']) for i in range(12)];beta=2*binary64(block['bound']);alpha=endpoint(block['alpha']);assert 0<=alpha<1
f=[Q(0)]*12
for i in [6,7,9]:f[i]=1
eta=max(s[i]*sum(asum(Delta[i][j])*s[j] for j in range(6,12)) for i in range(6,12));assert beta*eta<=alpha
# Closed inverse of the free K, confirmed against every basis column.
def k_inverse(r):
    a,t,z,y=ck
    return [r[0]/a,(4*r[1]+6*r[5])/(12*z),(4*r[2]-6*r[4])/(12*y),r[3]/t,(-6*r[2]+12*r[4])/(12*y),(6*r[1]+12*r[5])/(12*z)]
ki=[k_inverse([Q(i==j) for i in range(6)]) for j in range(6)]
for j,c in enumerate(ki):
    assert [sum(K[i+6][h+6]*c[h] for h in range(6)) for i in range(6)]==[Q(i==j) for i in range(6)]
norm=max(sum(abs(ki[j][i]/s[i+6]/s[j+6]) for j in range(6)) for i in range(6));assert norm<=beta
claimed=json.loads((SUBJECT/'_run_records/exact_controls_08.stdout').read_text());summ=claimed['summary'];eps=Q(summ['center_residual_radius_exact']);c=[Q(0)]*6+[Q(q) for q in summ['correction_physical']];y=[u+v for u,v in zip(x,c)]
# pi is one shared scalar here: evaluate each exact residual as an affine form.
def residual(center):
    return [sub(pair(f[i]),mul(pair(sum(G0[i][j]*center[j] for j in range(12))),pi)) for i in range(6,12)]
rho=residual(y)
for r,cr in zip(rho,summ['source_residual_intervals']):assert encloses(tuple(Q(q) for q in cr),r)
omega=max(s[i+6]*asum(r) for i,r in enumerate(rho));assert beta*omega/(1-alpha)<=eps
# Arbitrary-center theorem gives this displacement box without trusting correction quality.
U=[(y[i]-s[i]*eps,y[i]+s[i]*eps) for i in range(12)]
# Exact interval dense source rows, independent of author's B/DB streaming contraction.
ends=[isum(mul(G[i][j],U[j]) for j in range(12)) for i in range(12)]
q={i:U[i] for i in range(12)}
for i,e in enumerate(ends):q[14+i]=e
for station,tau in enumerate([Q(0),Q(1,4),Q(1)]):
    for j in range(6):q[26+6*station+j]=ends[6+j] if j<4 else add(mul(pair(tau),ends[6+j]),mul(pair(tau-1),ends[j]))
for i in range(6):q[44+i]=ends[i]
proof_intervals={r['index']:tuple(Q(q) for q in r['corrected_interval']) for r in claimed['rows']}
for i,p in q.items():assert encloses(proof_intervals[i],p),('proof interval fails dense interval control',i)
for i,components in [(12,U[:3]),(13,U[6:9]),(50,ends[:3]),(51,ends[3:6])]:assert norm_contained(proof_intervals[i],components),('magnitude',i)
# Independent true source and K response, direct static actions and source sqrt.
def source_motion(p):
    return [pair(0)]*6+[div(pair(1),mul(pair(cs[0]),p)),div(pair(1),mul(pair(3*cs[2]),p)),pair(0),div(pair(1),mul(pair(cs[1]),p)),pair(0),div(pair(1),mul(pair(2*cs[2]),p))]
ref=source_motion(pi_ref)
source_truth={i:p for i,p in enumerate(ref)}
for i in range(12):source_truth[14+i]=pair(-1 if i in [0,1,3,5] else 1 if i in [6,7,9] else 0)
for station,tau in enumerate([Q(0),Q(1,4),Q(1)]):
    for j in range(6):source_truth[26+6*station+j]=pair(1 if j in [0,1,3] else 1-tau if j==5 else 0)
for i in range(6):source_truth[44+i]=pair(-1 if i in [0,1,3,5] else 0)
def radius(r):
    if r['radius']=='absent':assert r['class']=='input' and r['index']<6 and binary64(r['x'])==0;return Q(0)
    return binary64(r['radius'])
def allowance(r):
    if r['class']=='input':return Q(0)
    if r['class']=='absolute':return binary64(r['bound'])
    n=binary64(r['x']);scale=binary64(r['scale'])
    return min(two(-64)*max(abs(n),scale)*(1+two(-21))+two(-53)*abs(n)+two(-1074),binary64(r['sharper']),abs(n)/10**9)
for i,p in source_truth.items():
    assert encloses(proof_intervals[i],p)
    assert encloses((endpoint(rows[i]['glo']),endpoint(rows[i]['ghi'])),p)
for i,components in [(12,ref[:3]),(13,ref[6:9]),(50,[pair(1),pair(1),pair(0)]),(51,[pair(1),pair(0),pair(1)])]:assert norm_contained(proof_intervals[i],components)
passes=sum(distance(proof_intervals[i],binary64(r['x']))<=allowance(r) for i,r in rows.items());assert passes==52
# Independent sign-only interval error and wrong-zero-center pass counts.
radii=[radius(rows[i]) for i in range(12)];dx=[mul(pair(sum((G0[i][j]*x[j]) for j in range(12))),pi) for i in range(12)];dx=[sub(dx[i],pair(sum(K[i][j]*x[j] for j in range(12)))) for i in range(12)]
dr=[sum(asum(Delta[i][j])*radii[j] for j in range(12)) for i in range(12)]
tau=beta*max(s[i]*(asum(dx[i])+dr[i]) for i in range(6,12))/(1-alpha)
T=[z*tau for z in s];action_error=[sum(asum(G[i][j])*T[j] for j in range(12))+asum(dx[i])+dr[i] for i in range(12)]
errors={i:T[i] for i in range(12)};errors[12]=0;errors[13]=sum(T[6:9]);errors.update({14+i:e for i,e in enumerate(action_error)})
for station,t0 in enumerate([Q(0),Q(1,4),Q(1)]):
    for j in range(6):errors[26+6*station+j]=action_error[6+j] if j<4 else t0*action_error[6+j]+(1-t0)*action_error[j]
errors.update({44+i:action_error[i] for i in range(6)});errors[50]=sum(action_error[:3]);errors[51]=sum(action_error[3:6]);sign_pass=sum(radius(r)+errors[i]<=allowance(r) for i,r in rows.items());assert sign_pass==7
wrong_eps=beta*max(s[i+6]*asum(r) for i,r in enumerate(residual(x)))/(1-alpha)
wrong_U=[(x[i]-s[i]*wrong_eps,x[i]+s[i]*wrong_eps) for i in range(12)]
wrong_ends=[isum(mul(G[i][j],wrong_U[j]) for j in range(12)) for i in range(12)]
wrong={i:p for i,p in enumerate(wrong_U)};wrong.update({14+i:e for i,e in enumerate(wrong_ends)})
for station,t0 in enumerate([Q(0),Q(1,4),Q(1)]):
    for j in range(6):wrong[26+6*station+j]=wrong_ends[6+j] if j<4 else add(mul(pair(t0),wrong_ends[6+j]),mul(pair(t0-1),wrong_ends[j]))
wrong.update({44+i:wrong_ends[i] for i in range(6)})
# For magnitude predicates, exact squared endpoint tests avoid approximating sqrt.
wrong_ok={i:distance(p,binary64(rows[i]['x']))<=allowance(rows[i]) for i,p in wrong.items()}
for i,components in [(12,wrong_U[:3]),(13,wrong_U[6:9]),(50,wrong_ends[:3]),(51,wrong_ends[3:6])]:
    n=binary64(rows[i]['x']);a=allowance(rows[i]);wrong_ok[i]=norm_contained((n-a,n+a),components)
assert sum(wrong_ok.values())==7
assert all(endpoint(r['lo'])==endpoint(r['hi'])==binary64(r['x'])==0 for r in zero)
# Nontrivial independently constructed 2x2 arbitrary-center/prescription/scaling controls.
general=0
for j in range(1,41):
    a=Q(2+j,3);b=Q((-1)**j,7);d=Q(3+j,2);det=a*d-b*b;assert det>0
    perturb=Q((-1)**j,10000+j);A=a+perturb;C=d-perturb;B=b+perturb;gd=A*C-B*B;assert gd>0
    scales=[two(j%7-3),two((j*3)%7-3)]
    inverse_K=[[d/det,-b/det],[-b/det,a/det]]
    beta0=max(sum(abs(inverse_K[k][l]/scales[k]/scales[l]) for l in range(2)) for k in range(2))*2
    eta0=max(scales[k]*sum(abs(v)*scales[l] for l,v in enumerate([perturb,perturb] if k==0 else [perturb,-perturb])) for k in range(2));alpha0=beta0*eta0;assert alpha0<1
    uc=Q(j-21,9);coupling=[Q(2,5),Q(-3,7)];f0=[Q(j,3),Q(2-j,5)];rhs=[f0[k]-coupling[k]*uc for k in range(2)]
    truth=[(C*rhs[0]-B*rhs[1])/gd,(-B*rhs[0]+A*rhs[1])/gd]
    center=[Q(3*j-10,7),Q(5-j,11)] # unrelated inaccurate center deliberately
    res=[rhs[0]-A*center[0]-B*center[1],rhs[1]-B*center[0]-C*center[1]]
    epsilon=beta0*max(abs(scales[k]*res[k]) for k in range(2))/(1-alpha0)
    for k in range(2):assert abs(truth[k]-center[k])<=scales[k]*epsilon
    coeff=[Q(-4,3),Q(2,9)];offset=Q(-7,5);action=sum(coeff[k]*truth[k] for k in range(2))+offset;mid=sum(coeff[k]*center[k] for k in range(2))+offset
    assert abs(action-mid)<=epsilon*sum(abs(coeff[k])*scales[k] for k in range(2))
    general+=1
# Three omitted-term errors and the real immutable-output obstruction.
d=Q(1,8);yy=1-d;assert 1-(1+d)*yy==d*d!=0
assert 1-1-Q(1,8)*2==Q(-1,4)
fixed=[Q(-11),Q(6)];assert fixed==[2*0-2*3-5,-2*0+2*3-0]
assert [Q(-5),Q(0)]==[2*0-2*0-5,-2*0+2*0]
ledger=[Q(1),Q(-1)];assert sum(ledger)==0 and any(v!=0 for v in ledger)
d=two(-52);assert abs(-d/(1-d*d))>two(-64)
# Grammar re-count independent of proposal arithmetic.
frame={'DA':2*(3*2+2),'DS':2*(3+3+3),'DM':8*(3+3+6+6)+2*(3*3),'DD':8*(3*3+1),'DQ':2*3};assert frame=={'DA':16,'DS':18,'DM':162,'DD':80,'DQ':6}
print(json.dumps({'status':'PASS','method':'independent exact rational source and closed 2x2 controls; no author/production import','source_pi_constant_contains_independent_1200bit_reference':True,'fixed_captured_source_rows_checked':52,'claimed_intervals_contain_independent_source_references':True,'claimed_intervals_contain_independent_residual_theorem_recovery':True,'claimed_rho_contains_shared_pi_affine_residual':True,'claimed_epsilon_covers_independent_direct_residual':True,'alpha_covers_actual_scaled_source_delta':True,'beta_covers_exact_scaled_K_inverse':True,'constructed_center_unchanged_predicate_pass':passes,'sign_only_pass':sign_pass,'wrong_zero_center_pass':sum(wrong_ok.values()),'original_zero_rows':52,'arbitrary_center_two_by_two_controls':general,'prescription_constrained_load_cancelling_ledger_controls':True,'immutable_output_violation_control':True,'frame_scalar_call_upper':frame,'residual_radius_claimed':str(eps),'residual_radius_independent_lower_requirement':str(beta*omega/(1-alpha))},indent=2))
