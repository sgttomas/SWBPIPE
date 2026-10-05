#!/usr/bin/env python3
"""I43 exact algebra controls. No source imports, numerical solver, or native run.
Reads frozen I42 specimen. Dyadic interval endpoints round out to 1024 bits.
The optional center is constructed by CLOSED rational 1x1/2x2 formulas and a
256-bit rounding. This is NOT execution of the retained factor or its output.
"""
from pathlib import Path
from fractions import Fraction as F
import json, re, struct, hashlib, sys, math
BASIS=Path(__file__).parent/'frozen_inputs'
INPUT_FILES={'projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I42/source_bridge_01/_run_records/debug_final.stdout': 'native_debug.stdout', 'projects/chirality-piping/core/solver/frame_kernel/tests/retained_k4/source_bridge_vectors.rs': 'source_bridge_vectors.rs', 'projects/chirality-piping/core/solver/frame_kernel/tests/retained_k4/source_bridge_tests.rs': 'source_bridge_tests.rs', 'projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/product_certificate.rs': 'product_certificate.rs'}
EXPECTED_INPUT_HASHES={'projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I42/source_bridge_01/_run_records/debug_final.stdout': '07725ec8180d3ad8f326b42a471375ecaa667681e3939e7dbeb627e013db7599', 'projects/chirality-piping/core/solver/frame_kernel/tests/retained_k4/source_bridge_vectors.rs': 'fc7dc63470bc78500b32430a5e48c6dd17542fc04bbc88ef5c2f2ab0f412af9c', 'projects/chirality-piping/core/solver/frame_kernel/tests/retained_k4/source_bridge_tests.rs': '12bb7ec09844892532de15dd6a051d0aa5c87ceb031f9d6f093e999720681313', 'projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/product_certificate.rs': '6bdcc659913e6caa6687190b5f12a36d1e53e272a9880ee3c8cd12bddf300bba'}
for rel,expected in EXPECTED_INPUT_HASHES.items():
    assert hashlib.sha256((BASIS/INPUT_FILES[rel]).read_bytes()).hexdigest()==expected, ('frozen input changed',rel)
R=Path('projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30')
FK=Path('projects/chirality-piping/core/solver/frame_kernel')
LOG=BASIS/'native_debug.stdout'
def real(x):return F(struct.unpack('>d',bytes.fromhex(x))[0])
def wide(x):return (-1 if x[0] else 1)*int(x[2],16)*F(2)**(x[1]-1023)
def flog(x):
    e=x.numerator.bit_length()-x.denominator.bit_length()
    return e-(x<F(2)**e)
def rnd(x,side,p=1024):
    x=F(x)
    if x==0:return F(0)
    h=F(2)**(flog(abs(x))-p+1); a=x/h; n=a.numerator//a.denominator
    if side=='up':n+=a!=n
    elif side=='near':
        d=a-n
        n+=d>F(1,2) or (d==F(1,2) and n%2!=0)
    return n*h
class I:
    def __init__(self,l=0,u=None):self.l=F(l);self.u=F(l if u is None else u);assert self.l<=self.u
    def __add__(self,b):
        b=iv(b);return I(rnd(self.l+b.l,'down'),rnd(self.u+b.u,'up'))
    __radd__=__add__
    def __neg__(self):return I(-self.u,-self.l)
    def __sub__(self,b):return self+-iv(b)
    def __rsub__(self,b):return iv(b)+-self
    def __mul__(self,b):
        b=iv(b);p=[self.l*b.l,self.l*b.u,self.u*b.l,self.u*b.u];return I(rnd(min(p),'down'),rnd(max(p),'up'))
    __rmul__=__mul__
    def __truediv__(self,b):
        b=iv(b);assert b.l>0
        return self*I(rnd(1/b.u,'down'),rnd(1/b.l,'up'))
    def ab(self):return max(abs(self.l),abs(self.u))
    def sq(self):
        lo=0 if self.l<=0<=self.u else min(self.l*self.l,self.u*self.u)
        return I(rnd(lo,'down'),rnd(max(self.l*self.l,self.u*self.u),'up'))
    def sqrt(self):
        assert self.l>=0
        def rt(x,up):
            if not x:return F(0)
            h=F(2)**(flog(x)//2-1023);a=x/(h*h);n=math.isqrt(a.numerator//a.denominator)
            return (n+(up and n*n*a.denominator<a.numerator))*h
        return I(rt(self.l,False),rt(self.u,True))
    def contains(self,b):b=iv(b);return self.l<=b.l<=b.u<=self.u
    def ser(self):return [str(self.l),str(self.u)]
def iv(a):return a if isinstance(a,I) else I(a)
def dot(a,b):return sum((iv(x)*y for x,y in zip(a,b)),I())
def mv(a,b):return [dot(r,b) for r in a]
def tr(a):return list(map(list,zip(*a)))
def mat(a,b):return [[dot(r,c) for c in tr(b)] for r in a]
def mx(a):return [[iv(x).ab() for x in r] for r in a]
def pt(a):return [I(v) for v in a]
def matrix(c):
    a,t,z,y=c
    return [[a,0,0,0,0,0],[0,t,0,0,0,0],[0,0,4*z,2*z,0,0],[0,0,2*z,4*z,0,0],[0,0,0,0,4*y,2*y],[0,0,0,0,2*y,4*y]]
raw=[];block=None
for line in LOG.read_text().splitlines():
    if 'I42_BLOCK ' in line:block=json.loads(line.split('I42_BLOCK ',1)[1])
    if 'I42_ROW ' in line:raw.append(json.loads(line.split('I42_ROW ',1)[1]))
rows={w:{r['index']:r for r in raw if r['witness']==w} for w in ['loaded','zero']}
assert len(rows['loaded'])==len(rows['zero'])==52
vector=(BASIS/'source_bridge_vectors.rs').read_text()
v={k:real(h) for k,h in re.findall(r'pub const (D|T|E|G|A|I|J|Z): u64 = 0x([0-9a-f]{16});',vector)}
D,T,E,G,A,Jk,Ik=[v[k] for k in ['D','T','E','G','A','J','I']]
source=(BASIS/'product_certificate.rs').read_text()
pi=[]
for name in ['LOWER','UPPER']:
    body=re.search(r'const '+name+r': \[u64; 16\] = \[(.*?)\];',source,re.S).group(1)
    limbs=[int(h,16) for h in re.findall(r'0x([0-9a-f]+)',body)]
    assert len(limbs)==16
    pi.append(sum(q<<(64*i) for i,q in enumerate(limbs))*F(2)**-1022)
PI=I(*pi)
# Independent reference uses a different identity from I42: Machin 16 atan1/5 - 4 atan1/239.
def atan(q,n=160):
    a=sum((F((-1)**j,(2*j+1)*q**(2*j+1)) for j in range(n)),F(0))
    return I(a,a+F(1,(2*n+1)*q**(2*n+1)))
PIR=16*atan(5)-4*atan(239)
assert PI.contains(PIR)
# Common source annulus formula; independent reference below uses powers.
ri=I(D/2)-T; area=PI*(I(T)*(D-T)); inertia=area*((I(D/2).sq()+ri.sq())/4)
CG=[E*area,G*2*inertia,E*inertia,E*inertia]
CK=[E*A,G*Jk,E*Ik,E*Ik]; DC=[c-k for c,k in zip(CG,CK)]
# Captured nodes (0,0,0)->(1,0,0), yref=(0,1,0) prove exact axes and L=1.
B=[[F(0) for _ in range(12)] for _ in range(6)]
for r,terms in enumerate([{0:-1,6:1},{3:-1,9:1},{1:1,7:-1,5:1},{1:1,7:-1,11:1},{2:-1,8:1,4:1},{2:-1,8:1,10:1}]):
    for j,a in terms.items():B[r][j]=F(a)
DG,DD=matrix(CG),matrix(DC)
Ag=mat(tr(B),mat(DG,B)); Delta=mat(tr(B),mat(DD,B)); Ae=mx(Ag); De=mx(Delta)
s=[F(0)]*6+[F(2)**e for e in block['scales']]
beta=2*real(block['bound']); oldalpha=wide(block['alpha']); oldtau=wide(block['tau'])
x=[real(rows['loaded'][i]['x']) for i in range(12)]
def row_radius(r):
    if r['radius']=='absent':
        assert r['class']=='input' and r['index']<6 and real(r['x'])==0
        return F(0) # captured exact zero prescriptions, not absent-radius fallback
    return real(r['radius'])
radius=[row_radius(rows['loaded'][i]) for i in range(12)]
def upper_sum(xs):return rnd(sum(xs,F(0)),'up')
# Signed point contraction before symmetric radius; full dense reference is independent of streaming B implementation.
dx=mv(tr(B),mv(DD,mv(B,pt(x))))
rerr=[upper_sum(De[i][j]*radius[j] for j in range(12)) for i in range(12)]
v_signed=max(rnd(s[i]*(dx[i].ab()+rerr[i]),'up') for i in range(6,12))
eta_actual=max(rnd(s[i]*sum(De[i][j]*s[j] for j in range(6,12)),'up') for i in range(6,12))
alpha_actual=rnd(beta*eta_actual,'up')
assert alpha_actual<=oldalpha<1
denom=rnd(1-oldalpha,'down');assert denom>0
def neumann(v):return rnd(rnd(beta*v,'up')/denom,'up')
tau_signed=neumann(v_signed) # retain OLD alpha deliberately
Tcorr=[rnd(q*tau_signed,'up') for q in s]
no_solve_action=[upper_sum(Ae[i][j]*Tcorr[j] for j in range(12))+dx[i].ab()+rerr[i] for i in range(12)]
# Exact row functionals on a point/interval displacement, common for both controls.
def recover(U):
    ends=mv(tr(B),mv(DG,mv(B,U)))
    out={i:U[i] for i in range(12)}
    for n in [0,1]:out[12+n]=sum((U[n*6+j].sq() for j in range(3)),I()).sqrt()
    for i,q in enumerate(ends):out[14+i]=q
    for si,t in enumerate([F(0),F(1,4),F(1)]):
        for c in range(6):out[26+6*si+c]=ends[6+c] if c<4 else t*ends[6+c]-(1-t)*ends[c]
    for i in range(6):out[44+i]=ends[i] # no constrained loads on captured specimen
    out[50]=sum((ends[i].sq() for i in range(3)),I()).sqrt()
    out[51]=sum((ends[i].sq() for i in range(3,6)),I()).sqrt()
    return out
err={i:Tcorr[i] for i in range(12)}
err[12]=0;err[13]=sum(Tcorr[6:9])
for i in range(12):err[14+i]=no_solve_action[i]
for si,t in enumerate([F(0),F(1,4),F(1)]):
    for c in range(6):err[26+6*si+c]=no_solve_action[6+c] if c<4 else t*no_solve_action[6+c]+(1-t)*no_solve_action[c]
for i in range(6):err[44+i]=no_solve_action[i]
err[50]=sum(err[44+i] for i in range(3));err[51]=sum(err[44+i] for i in range(3,6))
no_solve={i:I(real(r['x']))+I(-1,1)*(row_radius(r)+err[i]) for i,r in rows['loaded'].items()}
# Preferred certificate: arbitrary center followed by SOURCE residual, no solve accuracy assumption.
f=[F(0)]*12
for i in [6,7,9]:f[i]=1
res=[I(f[i])-q for i,q in enumerate(mv(tr(B),mv(DG,mv(B,pt(x)))))]
# Hypothetical retained factor input: scaled interval midpoint rounded to P=256.
rscaled=[rnd(s[i]*(res[i].l+res[i].u)/2,'near',256) for i in range(6,12)]
rphys=[rscaled[j]/s[6+j] for j in range(6)]
# CLOSED independent exact inverse algebra of this captured 6x6 cantilever block.
def closed_correction(r):
    a,t,z,y=CK
    return [r[0]/a,(4*r[1]+6*r[5])/(12*z),(4*r[2]-6*r[4])/(12*y),r[3]/t,(-6*r[2]+12*r[4])/(12*y),(6*r[1]+12*r[5])/(12*z)]
# Exact scaled inverse row sum used for diagnosis only, not the proposed certificate.
kinv_columns=[closed_correction([F(i==j) for i in range(6)]) for j in range(6)]
exact_scaled_inverse_norm=max(sum(abs(kinv_columns[j][i]/(s[6+i]*s[6+j])) for j in range(6)) for i in range(6))
assert exact_scaled_inverse_norm<=beta
ce=closed_correction(rphys)
z0=[rnd(ce[j]/s[6+j],'near',256) for j in range(6)]
c=[F(0)]*6+[s[6+j]*z0[j] for j in range(6)]
y=[x[i]+c[i] for i in range(12)] # exact dyadic addition in this control
assert all(rnd(q,'near')==q for q in y)
rho=[I(f[i])-q for i,q in enumerate(mv(tr(B),mv(DG,mv(B,pt(y)))))]
omega=max(rnd(s[i]*rho[i].ab(),'up') for i in range(6,12))
eps=neumann(omega)
U=[I(rnd(y[i]-s[i]*eps,'down'),rnd(y[i]+s[i]*eps,'up')) if i>=6 else I(y[i]) for i in range(12)]
corrected=recover(U)
# Different source formulas and closed response: no center fit to expected outputs.
AGref=PIR*(D*D-(D-2*T)**2)/4;IGref=PIR*(D**4-(D-2*T)**4)/64
refU=[I()]*6+[I(1)/(E*AGref),I(1)/(3*E*IGref),I(),I(1)/(2*G*IGref),I(),I(1)/(2*E*IGref)]
refs={i:q for i,q in enumerate(refU)}
refs[12]=I();refs[13]=(refU[6].sq()+refU[7].sq()).sqrt()
for i in range(12):refs[14+i]=I(-1 if i in [0,1,3,5] else 1 if i in [6,7,9] else 0)
for si,t in enumerate([F(0),F(1,4),F(1)]):
    for k in range(6):refs[26+6*si+k]=I(1 if k in [0,1,3] else 1-t if k==5 else 0)
for i in range(6):refs[44+i]=I(-1 if i in [0,1,3,5] else 0)
refs[50]=refs[51]=I(2).sqrt()
krefs=dict(refs)
for j,vv in enumerate([I(1)/CK[0],I(1)/(3*CK[2]),I(),I(1)/CK[1],I(),I(1)/(2*CK[2])]):krefs[6+j]=vv
krefs[13]=(krefs[6].sq()+krefs[7].sq()).sqrt()
def allowance(r):
    x=real(r['x'])
    if r['class']=='input':return F(0)
    if r['class']=='absolute':return real(r['bound'])
    sc=real(r['scale']);return min(F(2)**-64*max(abs(x),sc)*(1+F(2)**-21)+F(2)**-53*abs(x)+F(2)**-1074,real(r['sharper']),abs(x)/10**9)
def H(q,x):return max(abs(q.l-x),abs(q.u-x))
report=[]
for i,r in rows['loaded'].items():
    published=real(r['x']);baseline=I(wide(r['lo']),wide(r['hi']))
    assert baseline.contains(refs[i])
    assert no_solve[i].contains(refs[i]),('no solve containment',i)
    assert corrected[i].contains(refs[i]),('corrected containment',i)
    # New control references also overlap the frozen independent references.
    assert I(wide(r['glo']),wide(r['ghi'])).contains(refs[i])
    assert I(published-row_radius(r),published+row_radius(r)).contains(krefs[i]),('independent native radius',i)
    assert H(no_solve[i],published)<=H(baseline,published),('smaller no-solve',i)
    assert H(corrected[i],published)<=H(baseline,published),('smaller corrected',i)
    a=allowance(r)
    report.append({'index':i,'id':r['id'],'baseline_H':float(H(baseline,published)),'no_solve_H':float(H(no_solve[i],published)),'corrected_H':float(H(corrected[i],published)),'allowance':float(a),'baseline_pass':H(baseline,published)<=a,'no_solve_pass':H(no_solve[i],published)<=a,'corrected_pass':H(corrected[i],published)<=a,'reference_pass':H(refs[i],published)<=a,'corrected_interval':corrected[i].ser()})
# Counter-controls prove residual necessity and coefficient-change soundness.
# Deliberately wrong zero correction is allowed mathematically but must fail many predicates.
rho0=[res[i] for i in range(12)]
eps0=neumann(max(rnd(s[i]*rho0[i].ab(),'up') for i in range(6,12)))
wrong=recover([I(x[i]-s[i]*eps0,x[i]+s[i]*eps0) for i in range(12)])
wrong_pass=sum(H(wrong[i],real(r['x']))<=allowance(r) for i,r in rows['loaded'].items())
assert wrong_pass<52
# Small rational scalar: K=1,G=1+d,f=1,x=1,c=-d. A K-only residual gives false zero.
d=F(1,8);cc=-d;yy=1+cc;rr=1-(1+d)*yy
assert rr==d*d and rr!=0
exact=1/(1+d);rad=abs(rr)/(1-d)
assert yy-rad<=exact<=yy+rad and exact!=yy
# Prescribed-column control: K_FF=G_FF=1, K_FC=0, G_FC=1/8, u_C=2.
# Common complete potential pattern may contain a K entry canceled to exact zero.
prescribed_rho=F(1)-F(1)*1-F(1,8)*2
assert prescribed_rho==F(-1,4) and (F(1)-F(1)*1)==0
assert F(1)+prescribed_rho==F(3,4)
# Inverse norm loses direction even with exact frame/signs: K=I,G diagonal, RHS on first row.
assert (1/(1+d)-1)!=0 # a nonzero scalar tau cannot prove second component exactly zero
# General source violation is not repaired by the center: I33 SPD two-mode counter-control.
d=F(2)**-52;source0=-d/(1-d*d);bound=F(2)**-64
assert abs(source0)>bound
# Fully fixed axial member: no free equation or B, yet source recovery is nonzero.
fixed_G=[[F(2),F(-2)],[F(-2),F(2)]]
fixed_u=[F(0),F(3)]; fixed_f=[F(5),F(0)]
fixed_reaction=[sum(a*b for a,b in zip(row,fixed_u))-f for row,f in zip(fixed_G,fixed_f)]
assert fixed_reaction==[F(-11),F(6)]
assert [sum(a*b for a,b in zip(row,[F(0),F(0)]))-f for row,f in zip(fixed_G,fixed_f)]==[F(-5),F(0)]
# zero fixture: no B required; exact source-zero + anchored uniqueness is original theorem.
assert all(wide(r['lo'])==wide(r['hi'])==real(r['x'])==0 for r in rows['zero'].values())
summary={'control_kind':'exact algebra of captured specimen; constructed P256 center is NOT an observed retained-factor result','rows':52,'baseline_pass':sum(r['baseline_pass'] for r in report),'no_solve_pass':sum(r['no_solve_pass'] for r in report),'constructed_center_pass':sum(r['corrected_pass'] for r in report),'reference_pass':sum(r['reference_pass'] for r in report),'zero_pass_unchanged':52,'wrong_zero_center_pass':wrong_pass,'beta':float(beta),'old_alpha':float(oldalpha),'actual_pattern_alpha':float(alpha_actual),'exact_scaled_K_inverse_norm_control_only':float(exact_scaled_inverse_norm),'beta_over_exact_inverse_norm':float(beta/exact_scaled_inverse_norm),'old_scaled_rhs_majorant':float(oldtau*(1-oldalpha)/beta),'signed_scaled_rhs_majorant':float(v_signed),'old_tau':float(oldtau),'signed_tau_using_old_alpha':float(tau_signed),'old_to_signed_tau_ratio':float(oldtau/tau_signed),'source_residual_radius_scaled':float(eps),'source_residual_scaled_norm':float(omega),'correction_physical':[str(t) for t in c[6:]],'correction_scaled':[str(t) for t in z0],'source_residual_intervals':[q.ser() for q in rho[6:]],'center_residual_radius_exact':str(eps),'counter_controls':{'wrong_center_is_not_accepted_without_residual':True,'K_only_residual_misses_delta_c':True,'omitted_prescribed_column_falsely_zeroes_residual':True,'unchanged_output_source_violation_remains':True,'no_data_requires_original_zero_and_uniqueness_warrant':True,'no_free_data_does_not_erase_constrained_recovery':True}}
print(json.dumps({'summary':summary,'rows':report},indent=2))
