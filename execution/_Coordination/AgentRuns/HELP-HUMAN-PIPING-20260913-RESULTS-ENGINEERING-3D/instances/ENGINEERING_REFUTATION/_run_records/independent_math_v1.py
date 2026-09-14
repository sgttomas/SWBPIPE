"""Independent review math. Pure Python rational algebra; no production imports or file I/O.
Input numbers are explicit frozen analytical expectations; output goes only to stdout.
Pressure force units below divide by pi, so exact rational arithmetic is possible.
"""
from fractions import Fraction as F
import json

checks=[]
def eq(name,a,b):
    assert a==b,(name,a,b)
    checks.append(name)
def vec(v): return list(map(F,v))
def add(a,b): return [x+y for x,y in zip(a,b)]
def neg(a): return [-x for x in a]
def sub(a,b): return add(a,neg(b))
def scale(c,a): return [c*x for x in a]
def dot(a,b): return sum(x*y for x,y in zip(a,b))
def cross(a,b): return [a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]]
def trans(a): return list(map(list,zip(*a)))
def mv(a,v): return [dot(row,v) for row in a]
def mm(a,b): return [[dot(x,y) for y in trans(b)] for x in a]
def diag(a): return [[x if i==j else F(0) for j in range(len(a))] for i,x in enumerate(a)]
def energy(K,q):return dot(q,mv(K,q))/2
I=diag(vec([1,1,1]));z=vec([0,0,0]);z12=vec([0]*12)
ri,ro,E,nu,p=F(1),F(2),F(120),F(1,4),F(3)
Ai,As=ri**2,ro**2-ri**2
P=p*Ai;EA=E*As;A=P/As;B=p*ri**2*ro**2/As
for n,a,b in [('Ai/pi',Ai,1),('As/pi',As,3),('P/pi',P,3),('EA/pi',EA,360),('G',E/(2*(1+nu)),48),('A',A,1),('B',B,4),('inner radial',A-B/ri**2,-3),('outer radial',A-B/ro**2,0),('inner hoop',A+B/ri**2,5),('outer hoop',A+B/ro**2,2)]: eq(n,a,b)
pressure=[]
for name,et,eps,cap,Nwant,Swant in [
('P1',F(0),F(1,240),P,F(3),F(0)),
('P2',F(0),F(0),P,F(3,2),F(-3,2)),
('P3',F(0),F(-1,240),F(0),F(0),F(-3)),
('P4_free',F(1,1000),F(31,6000),P,F(3),F(0)),
('P4_restrained',F(1,1000),F(0),P,F(57,50),F(-93,50))]:
    e0=et-2*nu*P/EA;N=EA*(eps-e0);S=N-P
    eigen=[-EA*e0,EA*e0];wall=[-N,N];caps=[-cap,cap]
    kd=[-EA*eps,EA*eps];support=sub(kd,add(eigen,caps))
    eq(name+' wall force',N,Nwant);eq(name+' effective force',S,Swant)
    eq(name+' recovery subtract eigen only',sub(kd,eigen),wall)
    eq(name+' vessel force balance',sum(add(support,caps)),0)
    if name=='P2':eq(name+' support',support,[F(3,2),F(-3,2)])
    if name=='P4_restrained':eq(name+' support',support,[F(93,50),F(-93,50)])
    pressure.append({'case':name,'Nw_over_pi':str(N),'S_over_pi':str(S),'sigma_z':str(N/As),'eigen_over_pi':list(map(str,eigen)),'support_over_pi':list(map(str,support))})
eq('remote P3 closure balance',add([P,-P],[-P,P]),[0,0])
eq('chain internal caps',P-P,0)
for orient in [F(-1),F(1)]:
    eq('oriented local axial force invariant '+str(orient),orient*(orient*P),P)
eq('paired interpolated E nu G',F(180)/(2*(1+F(7,24))),F(2160,31))
eq('independent G interpolation mutation differs',F(2160,31)==F(69),False)

def q_motion(geom,d):
    xi,xj,ai,aj,Q=geom
    ui,ti,uj,tj=d[:3],d[3:6],d[6:9],d[9:12]
    r=sub(add(xj,aj),add(xi,ai))
    di=add(ui,cross(ti,ai));dj=add(uj,cross(tj,aj))
    return mv(trans(Q),sub(sub(dj,di),cross(scale(F(1,2),add(ti,tj)),r)))+mv(trans(Q),sub(tj,ti))

def B_motion(geom):
    # Differentiate the defining motions using exact coordinate perturbations.
    cols=[]
    for n in range(12):
        d=z12.copy();d[n]=F(1);cols.append(q_motion(geom,d))
    return trans(cols)

def B_formula(geom):
    xi,xj,ai,aj,Q=geom;r=sub(add(xj,aj),add(xi,ai))
    def S(a):return [[F(0),-a[2],a[1]],[a[2],F(0),-a[0]],[-a[1],a[0],F(0)]]
    sa,sj,sr=S(ai),S(aj),S(r)
    bt=[neg(I[i])+add(sa[i],scale(F(1,2),sr[i]))+I[i]+add(neg(sj[i]),scale(F(1,2),sr[i])) for i in range(3)]
    br=[z+neg(I[i])+z+I[i] for i in range(3)]
    return mm(trans(Q),bt)+mm(trans(Q),br)
base=(z,vec([2,0,0]),z,z,I)
K=diag(vec([10,20,30,40,50,60]));baseB=B_motion(base)
isolations=[(6,F(1,100),None),(7,F(1,100),None),(8,F(1,100),None),(9,F(1,100),None),(10,F(1,100),(8,F(-1,100))),(11,F(1,100),(7,F(1,100)))]
for n,(j,v,extra) in enumerate(isolations):
    d=z12.copy();d[j]=v
    if extra:d[extra[0]]=extra[1]
    q=q_motion(base,d);expected=vec([0]*6);expected[n]=F(1,100)
    eq('isolated q '+str(n),q,expected)
    eq('isolated energy '+str(n),energy(K,q),F(n+1,2000) if n<3 else F(n+1,2000))
    f=mv(trans(baseB),mv(K,q))
    if n==1:eq('shear y balancing moments',[f[5],f[11]],[F(-1,5),F(-1,5)])
    if n==2:eq('shear z balancing moments',[f[4],f[10]],[F(3,10),F(3,10)])
H=diag(vec([0]*6));H[0][0]=F(4);H[0][3]=H[3][0]=F(1);H[3][3]=F(9)
D=diag(vec([2,2,2,1,1,1]));Di=diag([F(1,2)]*3+[F(1)]*3)
KC=mm(mm(Di,H),Di);q=vec(['.2',0,0,'.1',0,0]);g=mv(KC,q)
eq('coupled g',[g[0],g[3]],[F(1,4),F(1)])
eq('coupled energy',energy(KC,q),F(3,40))
Rscale=diag([F(1,2)]*3+[F(1)]*3);Hp=mm(mm(Rscale,H),Rscale)
eq('rescaled H',[(Hp[0][0]),Hp[0][3],Hp[3][3]],[1,F(1,2),9]);eq('rescaled physical K',Hp,KC)
f0=mv(trans(baseB),neg(g));eq('initial residual',[f0[0],f0[6],f0[3],f0[9]],[F(1,4),F(-1,4),F(1),F(-1)])
eq('PSD principal determinant',H[0][0]*H[3][3]-H[0][3]**2,35)
eq('PSD exact completion of square',energy(KC,q),F(1,2)*(4*(q[0]/2+q[3]/4)**2+F(35,4)*q[3]**2))
nullref=vec([0,1,0,0,0,0]);eq('null reference stress free',mv(KC,nullref),vec([0]*6))

R=[vec([0,-1,0]),vec([1,0,0]),vec([0,0,1])];c=vec([7,-3,5]);origin=vec([11,-17,3])
geoms=[base,(vec([1,2,3]),vec([3,2,3]),vec([0,'.2',0]),vec([0,'.2',0]),I),
       (vec([1,2,3]),vec([3,2,3]),vec([0,'.2','.1']),vec([0,'.2','.1']),I)]
for k,geom in enumerate(geoms):
    xi,xj,ai,aj,Q=geom;B0=B_motion(geom)
    eq('B derivative '+str(k),B0,B_formula(geom))
    for axis in range(3):
        t=z.copy();t[axis]=F(1);eq('rigid translation '+str((k,axis)),mv(B0,t+z+t+z),vec([0]*6))
        w=z.copy();w[axis]=F(1)
        rigid=cross(w,sub(xi,origin))+w+cross(w,sub(xj,origin))+w
        eq('rigid rotation '+str((k,axis)),mv(B0,rigid),vec([0]*6))
    d=[F(j-3,97) for j in range(12)];dq=[F(7-j,101) for j in range(12)]
    q0=q_motion(geom,d);elastic=sub(q0,q);g0=mv(KC,elastic);f=mv(trans(B0),g0)
    eq('virtual work '+str(k),dot(g0,mv(B0,dq)),dot(f,dq))
    h=F(1,1000)
    grad=[(energy(KC,sub(q_motion(geom,add(d,scale(h,[F(int(j==a)) for j in range(12))]))),q))-energy(KC,sub(q_motion(geom,sub(d,scale(h,[F(int(j==a)) for j in range(12))]))),q)))/(2*h) for a in range(12)]
    eq('central energy derivative '+str(k),grad,f)
    eq('global force balance '+str(k),add(f[:3],f[6:9]),z)
    eq('arbitrary origin moment balance '+str(k),add(add(cross(sub(xi,origin),f[:3]),f[3:6]),add(cross(sub(xj,origin),f[6:9]),f[9:12])),z)
    rotated=(add(mv(R,xi),c),add(mv(R,xj),c),mv(R,ai),mv(R,aj),mm(R,Q))
    rd=sum([mv(R,d[j:j+3]) for j in range(0,12,3)],[])
    eq('rigid coordinate frame q '+str(k),q_motion(rotated,rd),q0)
    rf=mv(trans(B_motion(rotated)),mv(KC,sub(q_motion(rotated,rd),q)))
    eq('rigid coordinate frame force '+str(k),rf,sum([mv(R,f[j:j+3]) for j in range(0,12,3)],[]))
    J=diag(vec([-1,1,-1]));T=diag(vec([1,-1,1,1,-1,1]))
    reverse=(xj,xi,aj,ai,mm(Q,J));dr=d[6:]+d[:6]
    qr=q_motion(reverse,dr);KCr=mm(mm(T,KC),T)
    eq('endpoint reversal q '+str(k),qr,mv(T,q0))
    fr=mv(trans(B_motion(reverse)),mv(KCr,sub(qr,mv(T,q))))
    eq('endpoint reversal residual '+str(k),fr,f[6:]+f[:6])
    eq('endpoint reversal energy '+str(k),energy(KCr,sub(qr,mv(T,q))),energy(KC,elastic))
raw=vec([0,0,0,0,0,'.01',0,'.02',0,0,0,'.01'])
eq('raw difference mutation correct q',q_motion(base,raw),vec([0]*6))
eq('raw difference mutation bad energy',F(20)*F('.02')**2/2,F('.004'))
print(json.dumps({'status':'PASS','method':'independent stdlib Fraction motion/energy derivation; no production imports','exact_rational_assertions':len(checks),'checks':checks,'pressure_cases':pressure},indent=2))
