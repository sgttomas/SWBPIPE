#!/usr/bin/env python3
"""RV58 independent exact checks. No imports from author or production evaluator.
Input facts: emitted actual endpoints/centers/scales, immutable proposed native
law binary64 constants. Source mechanics: independent closed 6-DOF cantilever,
Machin pi interval and exact rational inverse. Review-only arbitrary precision.
"""
from pathlib import Path
from fractions import Fraction as Q
import json, re, sys, math, hashlib
ROOT=Path(__file__).resolve().parent
SOURCE=Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/f2a-arithmetic/projects/chirality-piping/core/solver/frame_kernel")
def pow2(e): return Q(2)**e
def b64(h):
    v=int(h,16) if isinstance(h,str) else h
    s=-1 if v>>63 else 1
    e=(v>>52)&2047; m=v&((1<<52)-1)
    assert e!=2047
    return s*Q(m if e==0 else (1<<52)|m)*pow2(-1074 if e==0 else e-1075)
def wide(v):
    s,e,m=v
    return (-1 if s else 1)*Q(int(m,16))*pow2(e-1023)
def floorlog(x):
    assert x>0
    e=x.numerator.bit_length()-x.denominator.bit_length()
    return e-(x<pow2(e))
def rn(x,p):
    if not x:return Q(0)
    step=pow2(floorlog(abs(x))-p+1); t=x/step
    n=t.numerator//t.denominator; rem=t-n
    return (n+(rem>Q(1,2) or rem==Q(1,2) and n%2))*step
def rn64(x):
    if not x:return Q(0)
    step=pow2(max(floorlog(abs(x))-52,-1074)); t=x/step
    n=t.numerator//t.denominator; rem=t-n
    return (n+(rem>Q(1,2) or rem==Q(1,2) and n%2))*step
def atan(q,n):
    s=sum((Q((-1)**k,(2*k+1)*q**(2*k+1)) for k in range(n)),Q(0))
    t=s+Q((-1)**n,(2*n+1)*q**(2*n+1))
    return min(s,t),max(s,t)
a,b=atan(5,400),atan(239,130)
PI=(16*a[0]-4*b[1],16*a[1]-4*b[0])
facts={}
vector_bytes=(SOURCE/"tests/retained_k4/source_residual_vectors.rs").read_bytes()
for name,value in re.findall(r"pub const (D|T|E|G|A|I|J|Z): u64 = 0x([0-9a-f]+);",vector_bytes.decode()):
    facts[name]=b64(value)
D,T,E,G,A,I,J=(facts[n] for n in ("D","T","E","G","A","I","J"))
inner=D-2*T
assert 0<inner<D and E>0 and G>0
area0=(D**2-inner**2)/4
inertia0=(D**4-inner**4)/64
torsion0=2*inertia0
assert A==rn64(area0*sum(PI)/2)
assert I==rn64(inertia0*sum(PI)/2)
assert J==rn64(torsion0*sum(PI)/2)
def dot(a,b):return sum((x*y for x,y in zip(a,b)),Q(0))
def mul(a,b):return [[dot(row,col) for col in zip(*b)] for row in a]
def transpose(a):return list(map(list,zip(*a)))
def inverse(a):
    n=len(a);v=[list(map(Q,row))+[Q(i==j) for j in range(n)] for i,row in enumerate(a)]
    for j in range(n):
        p=next(k for k in range(j,n) if v[k][j])
        v[p],v[j]=v[j],v[p]; d=v[j][j];v[j]=[x/d for x in v[j]]
        for i in range(n):
            if i!=j:
                c=v[i][j];v[i]=[x-c*y for x,y in zip(v[i],v[j])]
    return [row[n:] for row in v]
def norminf(m):return max(map(lambda r:sum(map(abs,r)),m),default=Q(0))
def local_matrix(ea,ei,gj,L):
    z=Q(0);m=[[z]*6 for _ in range(6)]
    for i,c in [(0,ea/L),(1,12*ei/L**3),(2,12*ei/L**3),(3,gj/L),(4,4*ei/L),(5,4*ei/L)]:m[i][i]=c
    m[1][5]=m[5][1]=-6*ei/L**2
    m[2][4]=m[4][2]=6*ei/L**2
    return m
def transform(axes):
    return [[axes[i%3][j%3] if i//3==j//3 else Q(0) for j in range(6)] for i in range(6)]
def model(rotated):
    if rotated:
        axes=[[Q(1,3),Q(2,3),Q(2,3)],[Q(2,3),Q(-2,3),Q(1,3)],[Q(2,3),Q(1,3),Q(-2,3)]]
        L=Q(3);f=list(map(Q,[3,9,-6,-2,-1,11]))
    else:
        axes=[[Q(i==j) for j in range(3)] for i in range(3)]
        L=Q(1);f=list(map(Q,[1,1,0,1,0,0]))
    for i in range(3):
        for j in range(3):assert dot(axes[i],axes[j])==Q(i==j)
    TT=transform(axes)
    core=mul(transpose(TT),mul(local_matrix(E*area0,E*inertia0,G*torsion0,L),TT))
    k=mul(transpose(TT),mul(local_matrix(E*A,E*I,G*J,L),TT))
    source_solution0=[dot(row,f) for row in inverse(core)] # u=source_solution0/pi
    return axes,L,f,TT,core,k,source_solution0
components=["Ux","Uy","Uz","Rx","Ry","Rz"]
ids=[f"Displacement(Dof {{ node: {node}, component: {c} }})" for node in (0,1) for c in components]
ids += ["DisplacementMagnitude(0)","DisplacementMagnitude(1)"]
ids += [f"EndAction {{ member: 7, end: {end}, component: {c} }}" for end in ("I","J") for c in components]
ids += [f"StationAction {{ station: {st}, component: {c} }}" for st in (17,18,19) for c in components]
ids += [f"Reaction(Dof {{ node: 0, component: {c} }})" for c in components]
ids += ["SupportForceMagnitude(3)","SupportMomentMagnitude(3)"]
def range_div_pi(c):
    v=[c/p for p in PI];return min(v),max(v)
def sqrt_contains(lo,hi,square):
    assert hi>=0 and hi*hi>=square[1]
    assert lo<=0 or lo*lo<=square[0]
def source_truth(rows,rotated,loaded):
    axes,L,f,TT,core,k,u0=model(rotated)
    if not loaded:u0=[Q(0)]*6;f=[Q(0)]*6
    uloc0=[dot(r,u0) for r in TT]
    # Exact source reaction and end mechanics; pi cancels in load-controlled actions.
    ux,uy,uz,rx,ry,rz=uloc0
    ea,ei,gj=E*area0,E*inertia0,G*torsion0
    root=[-ea/L*ux,-12*ei/L**3*uy+6*ei/L**2*rz,-12*ei/L**3*uz-6*ei/L**2*ry,-gj/L*rx,6*ei/L**2*uz+2*ei/L*ry,-6*ei/L**2*uy+2*ei/L*rz]
    tip=[-root[0],-root[1],-root[2],-root[3],6*ei/L**2*uz+4*ei/L*ry,-6*ei/L**2*uy+4*ei/L*rz]
    reaction=[dot(row,root) for row in transpose(TT)]
    assert [dot(row,uloc0) for row in local_matrix(ea,ei,gj,L)]==[dot(row,f) for row in TT]
    exact=[(Q(0),Q(0))]*6+[range_div_pi(v) for v in u0]+[None,None]+[(v,v) for v in root+tip]
    for t in [Q(0),Q(1,4),Q(1)]:
        exact += [(v,v) for v in tip[:4]+[t*tip[c]+(t-1)*root[c] for c in (4,5)]]
    exact += [(v,v) for v in reaction]+[None,None]
    assert len(rows)==len(exact)==52
    for i,d in enumerate(rows):
        assert d["index"]==i and d["id"]==ids[i],("row identity",i)
        lo,hi=wide(d["lo"]),wide(d["hi"]);assert lo<=hi
        if exact[i] is not None:
            l,h=exact[i];assert lo<=l<=h<=hi,("source containment",i,d["id"])
        elif i==12:
            assert lo==hi==0
        elif i==13:
            square0=sum(v*v for v in u0[:3])
            sqrt_contains(lo,hi,(square0/PI[1]**2,square0/PI[0]**2))
        else:
            v=reaction[:3] if i==50 else reaction[3:]
            square=sum(x*x for x in v);sqrt_contains(lo,hi,(square,square))
    return core,k,u0,f
def read(path,marker):
    rows={};centers={};blocks={}
    for line in path.read_text().splitlines():
        for kind,target in [("ROW",rows),("CENTER",centers),("BLOCK",blocks)]:
            tag=marker+"_"+kind+" "
            if tag in line:
                d=json.loads(line.split(tag,1)[1]);label=d["witness"]
                if kind=="BLOCK":assert label not in target;target[label]=d
                else:target.setdefault(label,[]).append(d)
    for label in rows:rows[label].sort(key=lambda d:d["index"])
    for label in centers:centers[label].sort(key=lambda d:d["a"])
    return rows,centers,blocks
def check(path,marker):
    rows,centers,blocks=read(path,marker);summary={}
    for label,rr in rows.items():
        rotated=label in ("rotated","arbitrary");loaded=label!="zero"
        core,k,u0,f=source_truth(rr,rotated,loaded)
        cs=centers[label];bl=blocks[label]
        assert len(cs)==6 and [d["a"] for d in cs]==list(range(6))
        s=[pow2(d["s"]) for d in cs];y=[wide(d["center"]) for d in cs]
        alpha=wide(bl["alpha"]);eps=wide(bl["epsilon"])
        if loaded:
            beta=2*b64(bl["bound"])
            sks=[[s[i]*v*s[j] for j,v in enumerate(row)] for i,row in enumerate(k)]
            knorm=norminf(inverse(sks));assert beta>=knorm
            delta=[[max(abs(s[i]*(p*core[i][j]-k[i][j])*s[j]) for p in PI) for j in range(6)] for i in range(6)]
            assert alpha>=beta*norminf(delta) and 0<=alpha<1
            omega=max(max(abs(wide(d["rho_lo"])),abs(wide(d["rho_hi"]))) for d in cs)
            assert eps>=beta*omega/(1-alpha)
            for i in range(6):
                u=range_div_pi(u0[i]);assert y[i]-s[i]*eps<=u[0]<=u[1]<=y[i]+s[i]*eps,("displacement theorem",label,i)
        else:assert eps==0 and y==[0]*6
        residual_checks=0
        for name,u in [("rho",y)]+([("initial",[b64(d["x"]) for d in rr[6:12]])] if marker=="I44" else []):
            values=[[s[i]*((f[i] if loaded else 0)-p*dot(core[i],u)) for i in range(6)] for p in PI]
            for i in range(6):
                l,h=min(values[0][i],values[1][i]),max(values[0][i],values[1][i])
                assert wide(cs[i][name+"_lo"])<=l<=h<=wide(cs[i][name+"_hi"]),(label,name,i)
                residual_checks+=1
        if marker=="I44" and loaded:
            for i in range(6):
                mid=rn(wide(cs[i]["initial_lo"])+wide(cs[i]["initial_hi"]),1024)/2
                assert wide(cs[i]["rhs"])==rn(mid,bl["P"])
                assert y[i]==rn(b64(rr[6+i]["x"])+s[i]*wide(cs[i]["correction"]),1024)
        passed=None
        if marker=="I44":
            passed=0;rel=0
            for d in rr:
                x=b64(d["x"]);lo=wide(d["lo"]);hi=wide(d["hi"])
                h=max(abs(x-lo),abs(x-hi));bound=b64(d["bound"])
                if d["class"]=="relative":
                    rel+=1;scale=b64(d["scale"]);m=max(abs(x),scale)
                    exact=pow2(-64)*m+pow2(-85)*m+pow2(-53)*abs(x)+pow2(-1074)
                    a=rn64(rn64(pow2(-64)*m)*(1+pow2(-21)))
                    u=rn64(rn64(pow2(-53)*abs(x))+pow2(-1074))
                    assert bound==rn64(a+u),("binary64 ceiling",d["index"])
                    ok=h<=exact and h<=bound and 10**9*h<=abs(x)
                elif d["class"]=="input":ok=h==0
                else:assert d["class"]=="absolute";ok=h<=bound
                assert ok==d["pass"]
                passed+=ok
            assert passed==52
        summary[label]={"source_rows":len(rr),"source_residual_components":residual_checks,"native_predicates":passed,"source_containment":True,"inverse_perturbation_radius":"verified" if loaded else "zero free motion"}
    return summary
result={"pi_method":"Machin 16 atan(1/5) - 4 atan(1/239), 400/130 alternating terms","pi_width_log2_upper":floorlog(PI[1]-PI[0])+1,"native_input_vectors_sha256":hashlib.sha256(vector_bytes).hexdigest(),"results":{}}
for a in sys.argv[1:]:
    path=ROOT/a
    result["results"][a]=check(path,"RV58" if "fixture" in a else "I44")
# A K-only correction residual can vanish while the source residual is nonzero.
delta=Q(1,8);c=-delta;y=1+c
assert -delta-c==0 and 1-(1+delta)*y==Q(1,64)
result["K_only_counter_control"]="1/64 exact nonzero source residual"
print(json.dumps(result,indent=2))
