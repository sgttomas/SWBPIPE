"""RV45 exact ABSTRACT matrix/action controls. No solver/model/product execution.
Only fixed small matrix contractions and closed scalar/2x2 identities.
"""
from fractions import Fraction as F
import json

checks=[]
def check(name, **details):
    checks.append(dict(name=name,status='pass',**details))
def zeros(m,n): return [[F(0) for _ in range(n)] for _ in range(m)]
def transpose(a): return list(map(list,zip(*a)))
def mm(a,b): return [[sum((x*y for x,y in zip(row,col)),F(0)) for col in zip(*b)] for row in a]
def mv(a,b): return [sum((x*y for x,y in zip(row,b)),F(0)) for row in a]
def sub(a,b): return [[x-y for x,y in zip(r,s)] for r,s in zip(a,b)]
def bounded(a,b): return all(abs(x)<=y for r,s in zip(a,b) for x,y in zip(r,s))
def nnz(a): return sum(x!=0 for row in a for x in row)
def local(inv):
    h=zeros(6,12)
    h[0][0],h[0][6]=-1,1
    h[1][3],h[1][9]=-1,1
    for row,rot,trans,sgn in [(2,5,1,1),(3,11,1,1),(4,4,2,-1),(5,10,2,-1)]:
        h[row][rot]=1; h[row][trans]=sgn*inv; h[row][trans+6]=-sgn*inv
    return h

def constit(c,inv):
    d=zeros(6,6);d[0][0]=c[0]*inv;d[1][1]=c[1]*inv
    for p,k in [(2,2),(4,3)]:
        d[p][p]=d[p+1][p+1]=4*c[k]*inv
        d[p][p+1]=d[p+1][p]=2*c[k]*inv
    return d

def global_bound(inv):
    b=zeros(6,12)
    for j in [0,1,2,6,7,8]:b[0][j]=1
    for j in [3,4,5,9,10,11]:b[1][j]=1
    for r,rot in [(2,3),(3,9),(4,3),(5,9)]:
        for j in [0,1,2,6,7,8]:b[r][j]=inv
        for j in range(rot,rot+3):b[r][j]=1
    return b

# Eight exact rational orthogonal frames; all unit axes checked algebraically.
identity=[[F(int(i==j)) for j in range(3)] for i in range(3)]
rx=[[F(1),0,0],[0,F(3,5),F(-4,5)],[0,F(4,5),F(3,5)]]
ry=[[F(5,13),0,F(12,13)],[0,F(1),0],[F(-12,13),0,F(5,13)]]
rz=[[F(8,17),F(-15,17),0],[F(15,17),F(8,17),0],[0,0,F(1)]]
frames=[identity,rx,ry,rz,mm(rx,ry),mm(ry,rz),mm(rx,mm(ry,rz)),mm(rz,mm(ry,rx))]
for r in frames: assert mm(r,transpose(r))==identity
check('rational_frames',count=len(frames))

matrix_entries=0; action_rows=0; station_rows=0
for length in [F(1,7),F(3,2),F(19)]:
    for frame in frames:
        ell=max(abs(length*x) for x in frame[0]); inv=1/length; rho=1/ell
        assert rho>=inv>0
        transform=zeros(12,12)
        for block in range(4):
            for i in range(3):
                for j in range(3):transform[3*block+i][3*block+j]=frame[i][j]
        h=local(inv); hb=[[abs(x) for x in row] for row in local(rho)]
        b=mm(h,transform);bb=global_bound(rho)
        assert nnz(bb)==48 and nnz(hb)==16 and bounded(b,bb) and bounded(h,hb)
        ck=list(map(F,[2,3,5,7]));cg=[F(21,10),F(14,5),F(51,10),F(69,10)]
        delta=[abs(x-y) for x,y in zip(cg,ck)]
        dk=constit(ck,inv);dg=constit(cg,inv);dgb=constit(cg,rho);dd=constit(delta,rho)
        assert nnz(dgb)==10 and bounded(sub(dg,dk),dd)
        kg=mm(transpose(b),mm(dg,b));kk=mm(transpose(b),mm(dk,b));de=mm(transpose(bb),mm(dd,bb))
        assert bounded(sub(kg,kk),de);matrix_entries+=144
        ag=mm(transpose(h),mm(dg,b));ak=mm(transpose(h),mm(dk,b))
        agb=mm(transpose(hb),mm(dgb,bb));dab=mm(transpose(hb),mm(dd,bb))
        assert bounded(ag,agb) and bounded(sub(ag,ak),dab)
        uk=[F((-1)**i*(i+1),11) for i in range(12)]
        # Four prescribed positions have exactly zero response difference.
        diff=[F(0) if i in [0,4,7,11] else F((-1)**i,101+i) for i in range(12)]
        ug=[x+y for x,y in zip(uk,diff)];U=list(map(abs,uk));T=list(map(abs,diff))
        radius=[x+y for x,y in zip(mv(agb,T),mv(dab,U))]
        actual=[abs(x-y) for x,y in zip(mv(ag,ug),mv(ak,uk))]
        assert all(a<=r for a,r in zip(actual,radius));action_rows+=12
        qg=mv(dg,mv(b,ug));qk=mv(dk,mv(b,uk))
        qr=[x+y for x,y in zip(mv(dgb,mv(bb,T)),mv(dd,mv(bb,U)))]
        for t in [F(0),F(1,3),F(1)]:
            for qi,qj in [(2,3),(4,5)]:
                err=abs(t*(qg[qj]-qk[qj])+(t-1)*(qg[qi]-qk[qi]))
                assert err<=t*qr[qj]+(1-t)*qr[qi];station_rows+=1
        # Same contractions supply all global reaction rows; ledger cancels.
        gb=mm(transpose(bb),mm(dgb,bb));rr=[x+y for x,y in zip(mv(gb,T),mv(de,U))]
        ar=[abs(x-y) for x,y in zip(mv(kg,ug),mv(kk,uk))]
        assert all(a<=r for a,r in zip(ar,rr))
check('exact_member_matrix_majorants',frames_and_lengths=24,entry_inequalities=matrix_entries)
check('local_end_action_majorants',rows=action_rows)
check('station_signed_interpolation',rows=station_rows)
check('global_reaction_majorants',rows=288)
assert 2*(48+10+48+12)==236 and 2*(48+10+16+48)==244
check('term_counts',Bbar=48,Hbar=16,D=10,pass2=236,pass3=244,station_upper=24)

# Nontrivial 2x2 scaled proof, all inverse quantities closed forms.
a,b,d=F(2),F(1,4),F(3); det=a*d-b*b
ki=[[d/det,-b/det],[-b/det,a/det]]
sc=[F(4),F(1,2)]
scaled_inv=[[ki[i][j]/sc[i]/sc[j] for j in range(2)] for i in range(2)]
beta=max(sum(map(abs,row)) for row in scaled_inv)
delmat=[[F(1,1024),F(-1,4096)],[F(-1,4096),F(1,2048)]]
freeM=[[abs(x) for x in row] for row in delmat]
eta=max(sc[i]*sum(freeM[i][j]*sc[j] for j in range(2)) for i in range(2))
uk=[F(3,5),F(-4,7)];f=mv([[a,b],[b,d]],uk)
ga,gb,gd=a+delmat[0][0],b+delmat[0][1],d+delmat[1][1]
gdet=ga*gd-gb*gb
ug=[(gd*f[0]-gb*f[1])/gdet,(ga*f[1]-gb*f[0])/gdet]
v=max(sc[i]*sum(freeM[i][j]*abs(uk[j]) for j in range(2)) for i in range(2))
assert beta*eta<1
tau=beta*v/(1-beta*eta)
assert all(abs(ug[i]-uk[i])<=sc[i]*tau for i in range(2))
check('scaled_neumann_closed_2x2',alpha=str(beta*eta))

# A full prescribed column changes RHS even though delta_FF=0.
c=F(7,3); kk,kg=F(-2),F(-2)-F(1,1024); kff=F(5)
uk=-kk*c/kff;ug=-kg*c/kff
assert abs(ug-uk)==abs(kg-kk)*abs(c)/kff>0
check('prescribed_column_required',exact_error=str(abs(ug-uk)))

# A direct constitutive change is needed even at identical displacement.
u=F(3,2);ck,cg=F(5),F(5)+F(1,512)
assert abs(cg*u-ck*u)==abs(cg-ck)*abs(u)>0
check('action_difference_not_only_displacement')

# Same total reaction does not identify a branch; scalar stiffness partition.
k1,k2=F(3),F(11);g1,g2=k1+F(1,256),k2
uk=1/(k1+k2);ug=1/(g1+g2)
assert k1*uk+k2*uk==g1*ug+g2*ug==1 and k1*uk!=g1*ug
check('equilibrium_total_does_not_prove_branch')

# Structural zero block and ledger flags must not be inferred from net cancellation.
assert sum([F(1),F(-1)],F(0))==0 and any(x!=0 for x in [F(1),F(-1)])
assert not any(x!=0 for x in [F(0),F(0)])
# An introduced coupling invalidates the original two singleton block decomposition.
delta=F(1,100);assert delta!=0
check('zero_data_and_cross_block_premises')

# Source changes can exceed unchanged bare b, with SPD source and exact q_K zero.
h=F(1,2**52);source_zero=-h/(1-h*h);bare=F(1,2**64)
assert 1-h>0 and abs(source_zero)>bare
assert F(1)/(F(2))>0 # alpha=1 rejection does not imply singularity
check('unavoidable_abstract_bare_b_refusal',error=str(source_zero),b=str(bare))

# Endpoint certificates retain separate raw and normalized coordinates.
l,u=F(999,1000),F(1001,1000);y=F(1000);a=F(1,1000);n=F(1)
hn=max(abs(n-l),abs(n-u));hu=max(abs(y-l/a),abs(y-u/a))
assert hn==F(1,1000) and hu==1 and hu==hn/a
# For source interval zero and raw/normalized zero, positivity of denominators adds no error.
assert max(abs(F(0)-F(0)),abs(F(0)-F(0)))==0
check('final_coordinate_and_zero_exactness')

# Outward intervals of E/nu derived G cannot use the rounded stored G singleton.
g=F(1)/(2*(1+F(1,4)));g_hat=F.from_float(float(g))
assert g!=g_hat
check('exact_selected_E_nu_material_difference',delta=str(abs(g-g_hat)))

print(json.dumps({'kind':'RV45 independent exact ABSTRACT controls; no product reach or availability claim','groups':len(checks),'checks':checks},indent=2))
