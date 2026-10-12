"""Independent numerical reference: exact-rational RN128 assembly and exact solve.

Invented K-D5 thermal-plus-rigid-rotation control; normalized leaves at e9296aae32.
This test/reference code is not imported by the product."""
import argparse,hashlib,json,math
from pathlib import Path
from fractions import Fraction as F
from math import isqrt
from decimal import Decimal,localcontext
P=lambda k:F(2)**k

def exponent(x):
    e=x.numerator.bit_length()-x.denominator.bit_length()
    return e if x>=P(e) else e-1

def rn(x):
    if not x:return x
    sign=-1 if x<0 else 1;x=abs(x);step=P(exponent(x)-127);y=x/step;n,r=divmod(y.numerator,y.denominator)
    if 2*r>y.denominator or 2*r==y.denominator and n%2:n+=1
    return sign*n*step

def sqrt(x):
    if not x:return x
    step=P(exponent(x)//2-127);y=x/(step*step);n=isqrt(y.numerator//y.denominator)
    cmp=4*y.numerator-y.denominator*(4*n*n+4*n+1)
    if cmp>0 or cmp==0 and n%2:n+=1
    return n*step
add=lambda a,b:rn(a+b)
mul=lambda a,b:rn(a*b)
div=lambda a,b:rn(a/b)
dot=lambda a,b:add(add(mul(a[0],b[0]),mul(a[1],b[1])),mul(a[2],b[2]))
cross=lambda a,b:[add(mul(a[(k+1)%3],b[(k+2)%3]),-mul(a[(k+2)%3],b[(k+1)%3])) for k in range(3)]
def axes(d,yr):
    L=sqrt(dot(d,d));x=[div(v,L) for v in d];proj=dot(yr,x);yc=[add(yr[k],-mul(proj,x[k])) for k in range(3)];ym=sqrt(dot(yc,yc));y=[div(v,ym) for v in yc]
    return [x,y,cross(x,y)],L

def frame(d,yr,A,I,J,E,G,eps):
    R,L=axes(d,yr);l2=mul(L,L);l3=mul(l2,L);ea=mul(E,A);axial=div(ea,L);torsion=div(mul(G,J),L);ei=mul(E,I)
    v12,v6,v4,v2=[div(mul(F(t),ei),den) for t,den in [(12,l3),(6,l2),(4,L),(2,L)]]
    k=[[F(0) for _ in range(12)] for _ in range(12)]
    for i,j,v in [(0,6,axial),(3,9,torsion)]:k[i][i]=k[j][j]=v;k[i][j]=k[j][i]=-v
    bz=[[v12,v6,-v12,v6],[v6,v4,-v6,v2],[-v12,-v6,v12,-v6],[v6,v2,-v6,v4]]
    by=[[v12,-v6,-v12,-v6],[-v6,v4,v6,v2],[-v12,v6,v12,v6],[-v6,v2,v6,v4]]
    for ids,B in [([1,5,7,11],bz),([2,4,8,10],by)]:
        for i in range(4):
            for j in range(4):k[ids[i]][ids[j]]=B[i][j]
    out=[[F(0) for _ in range(12)] for _ in range(12)]
    for bi in range(4):
        for bj in range(4):
            temp=[[F(0) for _ in range(3)] for _ in range(3)]
            for i in range(3):
                for j in range(3):
                    for h in range(3):temp[i][j]=add(temp[i][j],mul(k[3*bi+i][3*bj+h],R[h][j]))
            for i in range(3):
                for j in range(3):
                    for h in range(3):out[3*bi+i][3*bj+j]=add(out[3*bi+i][3*bj+j],mul(R[h][i],temp[h][j]))
    thrust=mul(ea,eps);loads=[F(0)]*12
    for i in range(3):loads[i]=-mul(thrust,R[0][i]);loads[6+i]=mul(thrust,R[0][i])
    return out,loads,R,L

def solve(A,b):
    aug=[list(row)+[rhs] for row,rhs in zip(A,b)];n=len(b)
    for col in range(n):
        pivot=max(range(col,n),key=lambda r:abs(aug[r][col]));aug[col],aug[pivot]=aug[pivot],aug[col]
        p=aug[col][col];assert p
        aug[col]=[x/p for x in aug[col]]
        for row in range(n):
            if row!=col and aug[row][col]:
                f=aug[row][col];aug[row]=[a-f*b for a,b in zip(aug[row],aug[col])]
    return [row[-1] for row in aug]
def display_norm_squared(x):
    with localcontext() as c:
        c.prec=100
        return (Decimal(x.numerator)/Decimal(x.denominator)).sqrt()

def dyadic_norm_upper(x):
    # Decimal supplies a starting approximation only. The exact rational
    # squared comparison establishes the bound, including upward correction.
    bound=float(display_norm_squared(x))
    while F.from_float(bound)**2 < x:
        bound=math.nextafter(bound,math.inf)
    assert F.from_float(bound)**2 >= x
    return bound
def sections_rows(v):
    return [v[:1],v[1:3],v[3:4],v[4:6]]

# Invented control; these are the actual normalized binary64 section leaves
# observed at e9296aae32. This mathematical basis is independent of model format.
model={
    'nodes':[{'position':dict(zip(['x','y','z'],v))} for v in
             [(5e6,7.3e6,1.2e6),(5e6+3,7.3e6+4,1.2e6+5),
              (5e6-1,7.3e6+7,1.2e6+5)]],
    'pipe_segments':[{'id':name,'y_reference':{'x':.3,'y':.1,'z':1.}}
                     for name in ['member','member2']],
}
normalized={name:{'As_m2':.005969026041820607,
                  'I_m4':2.7009842839238257e-5,
                  'J_m4':5.4019685678476514e-5}
            for name in ['member','member2']}
E=F.from_float(200e9);G=F.from_float(80e9);eps=F.from_float(.00012)
positions=[list(map(F.from_float,(float(n['position'][k]) for k in ['x','y','z']))) for n in model['nodes']]
phi=list(map(F.from_float,[.01,-.005,.007]));u_phys=[]
for r0 in positions:
    r=[x-y for x,y in zip(r0,positions[0])];rotation=[phi[(k+1)%3]*r[(k+2)%3]-phi[(k+2)%3]*r[(k+1)%3] for k in range(3)]
    u_phys.extend([eps*r[k]+rotation[k] for k in range(3)]+phi)
K=[[F(0) for _ in range(18)] for _ in range(18)];f=[F(0)]*18;members=[]
for ni,nj,pipe in [(0,1,model['pipe_segments'][0]),(1,2,model['pipe_segments'][1])]:
    s=normalized[pipe['id']];A,I,J=[F.from_float(s[k]) for k in ['As_m2','I_m4','J_m4']];yr=[F.from_float(float(pipe['y_reference'][k])) for k in ['x','y','z']];d=[b-a for a,b in zip(positions[ni],positions[nj])]
    ke,fe,R,L=frame(d,yr,A,I,J,E,G,eps);mp=list(range(6*ni,6*ni+6))+list(range(6*nj,6*nj+6))
    for i,gi in enumerate(mp):
        f[gi]+=fe[i]
        for j,gj in enumerate(mp):K[gi][gj]+=ke[i][j]
    members.append((ke,fe,R,L,mp))
uc=[F(0)]*3+phi
free=list(range(6,18));rhs=[f[i]-sum(K[i][j]*uc[j] for j in range(6)) for i in free]
uf=solve([[K[i][j] for j in free] for i in free],rhs);U=uc+uf
assert all(sum(K[i][j]*U[j] for j in range(18))==f[i] for i in free)
res=[sum(K[i][j]*u_phys[j] for j in range(18))-f[i] for i in range(18)]
print('basis=e9296aae normalized Section leaves='+str({k:{a:v[a] for a in ['As_m2','I_m4','J_m4']} for k,v in normalized.items()}))
print('physical zero identity residual in intended p system max='+str(float(max(map(abs,res)))))
force_squares=[];moment_squares=[]
for ke,fe,R,L,mp in members:
    vg=[sum(ke[r][c]*U[mp[c]] for c in range(12))-fe[r] for r in range(12)]
    vl=[sum(R[r%3][c]*vg[3*(r//3)+c] for c in range(3)) for r in range(12)]
    for rows in [vl[:6],vl[6:]]:
        a,s,t,m=sections_rows(rows);force_squares.extend([sum(x*x for x in a),sum(x*x for x in s)]);moment_squares.extend([sum(x*x for x in t),sum(x*x for x in m)])
    for frac in [F(0),F(1,4),F(1,2),F(3,4),F(1)]:
        x=L*frac;v=[-z for z in vl[:6]];v[4]-=x*vl[2];v[5]+=x*vl[1]
        a,s,t,m=sections_rows(v);force_squares.extend([sum(x*x for x in a),sum(x*x for x in s)]);moment_squares.extend([sum(x*x for x in t),sum(x*x for x in m)])
reaction=[sum(K[i][j]*U[j] for j in range(18))-f[i] for i in range(6)];force_squares.append(sum(x*x for x in reaction[:3]));moment_squares.append(sum(x*x for x in reaction[3:]))
F_squared=max(force_squares);M_squared=max(moment_squares)
print('exact intended-system reference max force group='+str(display_norm_squared(F_squared))+' N; moment group='+str(display_norm_squared(M_squared))+' Nm')

# Bind this independent calculation to the complete maintained invented control.
# The PP test consumes these bytes and checks the actual normalized leaves.
here=Path(__file__).resolve().parent
request_bytes=(here/'request.json').read_bytes()
request=json.loads(request_bytes)['model']
assert len(request['nodes'])==3 and len(request['pipe_segments'])==2
assert [n['id'] for n in request['nodes']]==['root','tip','free']
assert [n['position'] for n in request['nodes']]==[n['position'] for n in model['nodes']]
for pipe,(ni,nj,name) in zip(request['pipe_segments'],[('root','tip','member'),('tip','free','member2')]):
    assert pipe['id']==name and pipe['from']==ni and pipe['to']==nj
    assert pipe['y_reference']=={'x':.3,'y':.1,'z':1.}
    assert pipe['material']=='material'
    assert pipe['section']=={'outside_diameter':{'value':.2,'unit':'m'},'wall_thickness':{'value':.01,'unit':'m'}}
assert len(request['materials'])==1
material=request['materials'][0]
assert material['id']=='material' and material['constitutive_basis']=='homogeneous_isotropic_E_nu_v1'
assert material['elastic_modulus']=={'value':200e9,'unit':'Pa'}
assert material['poisson_ratio']=={'value':.25,'unit':'1'}
assert len(request['supports'])==1
support=request['supports'][0]
assert support['id']=='anchor' and support['node']=='root' and support['family']=='anchor'
assert set(support['restraints'])=={'UX','UY','UZ','RX','RY','RZ'}
assert request['components']==[] and request['combinations']==[]
assert len(request['reference_configurations'])==1 and len(request['load_cases'])==1
reference=request['reference_configurations'][0]
assert reference['geometry_ref']=={'kind':'authored_model_geometry'}
assert [(r['pipe_ref'],r['basis'],r['fit']) for r in reference['member_references']]==[(name,{'kind':'direct_strain_reference'},{'kind':'none'}) for name in ['member','member2']]
case=request['load_cases'][0]
assert case['primitive_loads']==[] and case['pressure_regions']==[]
state=case['analysis_state']
assert state['reference_configuration_ref']==reference['id'] and state['load_sources']==[]
assert state['history']=={'kind':'independent_equilibrium'}
assert len(state['element_states'])==2
for element,name in zip(state['element_states'],['member','member2']):
    assert element['pipe_ref']==name
    selection=element['material_selection'];assert selection['kind']=='explicit_base_properties' and selection['material_ref']=='material'
    thermal=element['thermal_state'];assert thermal['kind']=='explicit_interval_strain' and thermal['strain']=={'value':.00012,'unit':'1'}
assert len(state['support_states'])==1
ss=state['support_states'][0]
assert ss['support_ref']=='anchor' and ss['participation']=={'kind':'active_model_device'}
assert ss['boundary_motion']==[{'dof':dof,'value':{'value':value,'unit':'rad'},'meaning':'absolute_reference_displacement'} for dof,value in zip(('RX','RY','RZ'),(.01,-.005,.007))]
force_upper=dyadic_norm_upper(F_squared);moment_upper=dyadic_norm_upper(M_squared)
oracle={
    'control_request_sha256':hashlib.sha256(request_bytes).hexdigest(),
    'basis':'exact rational solve of independently formed RN128 stiffness and loads',
    'normalized_sections':normalized,
    'elastic_modulus_pa':float(E),'shear_modulus_pa':float(G),
    'interval_strain':float(eps),'prescribed_rotation_rad':list(map(float,phi)),
    'force_group_count':len(force_squares),'moment_group_count':len(moment_squares),
    'force_norm_upper_n':force_upper,'moment_norm_upper_nm':moment_upper,
    'force_squared_max':str(F_squared),'moment_squared_max':str(M_squared),
    'force_upper_hex':force_upper.hex(),'moment_upper_hex':moment_upper.hex(),
}
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--write-oracle',action='store_true',help='Replace the maintained oracle after checking the complete invented control.')
args=parser.parse_args()
encoded=json.dumps(oracle,indent=2)+'\n'
if args.write_oracle:
    (here/'oracle.json').write_text(encoded)
else:
    assert json.loads((here/'oracle.json').read_text())==oracle, 'maintained oracle differs from exact reference'
print('control_sha256='+oracle['control_request_sha256']+'; exact squared-norm oracle verified')
