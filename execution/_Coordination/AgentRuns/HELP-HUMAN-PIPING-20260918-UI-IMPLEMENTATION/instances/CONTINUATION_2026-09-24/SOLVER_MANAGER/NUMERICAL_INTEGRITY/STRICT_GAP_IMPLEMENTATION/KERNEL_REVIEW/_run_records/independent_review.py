"""Independent bounded exact-boundary/code-review and projection-design probes.
No production module, submitted generator, compiled solver, or external package imported.
The E/R arithmetic is a source-algorithm transcription verified at each operation
by exact Fraction arithmetic. The publication bridge is a proposal, NOT implemented Rust.
"""
from fractions import Fraction as F
from pathlib import Path
from itertools import product
import math,json,struct,re,sys
OUT=Path(__file__).parent
ROOT=next(p for p in OUT.parents if (p/"projects/chirality-piping/core/solver").is_dir())
NUM=OUT.parents[2]
REF=NUM/"STRICT_GAP_DESIGN/_run_records/REFERENCE_PROOF.json"
if not REF.exists():
    REF=next(ROOT.glob("projects/chirality-piping/execution/_Coordination/AgentRuns/*/instances/CONTINUATION_2026-09-24/SOLVER_MANAGER/NUMERICAL_INTEGRITY/STRICT_GAP_DESIGN/_run_records/REFERENCE_PROOF.json"))
ref=json.loads(REF.read_text())
ops=0
class E:
    def __init__(self,x=0):
        self.t=[] if not x else [float(x)]
    @property
    def q(self):return sum(map(F,self.t),F(0))
    def add(self,x):
        global ops
        exact=self.q+F(x);q=x;out=[]
        for e in self.t:
            s=q+e;z=s-q;lo=(q-(s-z))+(e-z)
            if not all(map(math.isfinite,[s,z,lo])):raise ArithmeticError
            if lo:out.append(lo)
            q=s;ops+=6
        if q:out.append(q)
        self.t=out
        assert self.q==exact
        assert ((self.t[-1]>0)-(self.t[-1]<0) if self.t else 0)==sgn(self.q)
        return self
    def plus(self,b,s=1):
        out=E();out.t=self.t[:]
        for x in b.t:out.add(s*x)
        return out
    def mul(self,b):
        global ops
        out=E()
        for x in self.t:
            for y in b.t:
                hi=x*y
                if not math.isfinite(hi):raise ArithmeticError
                exact=F(x)*F(y);lo=float(exact-F(hi))
                if F(lo)!=exact-F(hi):raise ArithmeticError
                out.add(lo);out.add(hi);ops+=2
        assert out.q==self.q*b.q
        return out
def sgn(x):return (x>0)-(x<0)
class R:
    def __init__(self,n=0,d=1):
        self.n=n if isinstance(n,E) else E(n)
        self.d=d if isinstance(d,E) else E(d)
        assert self.d.q>0
    @property
    def q(self):return self.n.q/self.d.q
    def plus(self,b):
        out=R(self.n.mul(b.d).plus(b.n.mul(self.d)),self.d.mul(b.d))
        assert out.q==self.q+b.q
        return out
K=[[100.,-100.,0.],[-100.,200.,-100.],[0.,-100.,100.]]
def response(load,gaps,state,s=1,root=0.,tail=0.):
    k=[[E(x) for x in row] for row in K]
    f=[E(),E(),E(s*load).add(s*tail)]
    fixed=[(0,root)]+[(j+1,s*gaps[j]) for j in range(2) if state[j]]
    free=[j+1 for j in range(2) if not state[j]]
    u=[R() for _ in range(3)]
    for i,v in fixed:u[i]=R(v)
    h=[]
    for i in free:
        hi=f[i]
        for j,v in fixed:hi=hi.plus(k[i][j].mul(E(v)),-1)
        h.append(hi)
    if len(free)==1:
        i=free[0];u[i]=R(h[0],k[i][i])
    elif len(free)==2:
        i,j=free
        d=k[i][i].mul(k[j][j]).plus(k[i][j].mul(k[i][j]),-1)
        assert d.q>0
        u[i]=R(k[j][j].mul(h[0]).plus(k[i][j].mul(h[1]),-1),d)
        u[j]=R(k[i][i].mul(h[1]).plus(k[i][j].mul(h[0]),-1),d)
    reactions=[]
    for i in range(3):
        r=R(E().plus(f[i],-1))
        for j in range(3):
            if k[i][j].t:r=r.plus(R(k[i][j].mul(u[j].n),u[j].d))
        reactions.append(r)
    assert all(reactions[i].q==0 for i in free)
    return u,reactions
# Independent general elimination, separate from determinant/ratio transcription.
def gaussian(a,b):
    n=len(b);rows=[[F(v) for v in row]+[F(b[i])] for i,row in enumerate(a)]
    for j in range(n):
        p=next(i for i in range(j,n) if rows[i][j])
        rows[j],rows[p]=rows[p],rows[j]
        pivot=rows[j][j];rows[j]=[x/pivot for x in rows[j]]
        for i in range(n):
            if i!=j:
                scale=rows[i][j];rows[i]=[x-scale*y for x,y in zip(rows[i],rows[j])]
    return [row[-1] for row in rows]
def oracle(load,gaps,state,s=1,root=F(0)):
    fixed={0:root}
    for j,active in enumerate(state):
        if active:fixed[j+1]=F(s)*F(gaps[j])
    free=[i for i in range(3) if i not in fixed]
    f=[F(0),F(0),F(s)*F(load)]
    u=[fixed.get(i,F(0)) for i in range(3)]
    a=[[F(K[i][j]) for j in free] for i in free]
    h=[f[i]-sum(F(K[i][j])*v for j,v in fixed.items()) for i in free]
    for i,v in zip(free,gaussian(a,h)):u[i]=v
    r=[sum(F(K[i][j])*u[j] for j in range(3))-f[i] for i in range(3)]
    return u,r
def down(x):return math.nextafter(x,-math.inf)
def up(x):return math.nextafter(x,math.inf)
def interval(e):
    lo=hi=0.
    for t in e.t:
        lo=down(lo+t);hi=up(hi+t)
    return lo,hi
def proposed_projection(r):
    exact=r.q
    if not r.n.t:
        return {"status":"exact_zero","value":0.,"absolute_bound":0.,"relative_bound":0.}
    try:q=sum(r.n.t)/sum(r.d.t)
    except (OverflowError,ZeroDivisionError):return {"status":"unresolved"}
    if not math.isfinite(q) or q==0. or sgn(q)!=sgn(exact):
        return {"status":"unresolved"}
    # Proposed exact-identity shortcut. Rust would use checked expansions to test N-qD==0.
    if F(q)*r.d.q==r.n.q:
        return {"status":"exact_representable","value":q,"absolute_bound":0.,"relative_bound":0.}
    nl,nh=interval(r.n);dl,dh=interval(r.d)
    if not all(map(math.isfinite,[nl,nh,dl,dh])) or dl<=0 or nl<=0<=nh:
        return {"status":"unresolved"}
    corners=[nl/dl,nl/dh,nh/dl,nh/dh]
    if not all(map(math.isfinite,corners)):return {"status":"unresolved"}
    lo=down(min(corners));hi=up(max(corners))
    if not all(map(math.isfinite,[lo,hi])) or lo<=0<=hi:return {"status":"unresolved"}
    assert F(lo)<=exact<=F(hi)
    q=min(max(q,lo),hi)
    error=max(up(q-lo),up(hi-q))
    relative=up(error/min(abs(lo),abs(hi)))
    assert abs(F(q)-exact)<=F(error)
    assert abs(F(q)-exact)/abs(exact)<=F(relative)
    return {"status":"qualified" if relative<=1e-9 else "unresolved_accuracy",
            "value":q,"absolute_bound":error,"relative_bound":relative}
source=(ROOT/"projects/chirality-piping/core/solver/frame_kernel/src/structural/exact_boundary.rs").read_text()
triples=re.findall(r'\(\s*(\d{19}),\s*(\d{19}),\s*(\d{19}),\s*\[(true|false),\s*(true|false)\],?\s*\)',source)
assert len(triples)==27
expected_source={}
for fb,g1,g2,a,b in triples:
    floats=[struct.unpack('>d',int(z).to_bytes(8,'big'))[0] for z in (fb,g1,g2)]
    expected_source[tuple(F(x) for x in floats)]=(a=="true",b=="true")
counts={};maxbound=0.;rowschecked=0;discrete=0;projections=0;casesout=[]
for case in ref["cases"]:
    load=float(F(case["F"]));gaps=[float(F(v)) for v in case["g"]]
    expected=tuple(bool(v) for v in case["expected"])
    assert expected_source[(F(load),*map(F,gaps))]==expected
    for s in [-1,1]:
        for seed in product([False,True],repeat=2):
            state=seed
            for it in range(1,4):
                u,r=response(load,gaps,state,s)
                ou,orr=oracle(load,gaps,state,s)
                assert [v.q for v in u]==ou
                assert [v.q for v in r]==orr
                rowschecked+=1
                for q in u+r:
                    p=proposed_projection(q);counts[p["status"]]=counts.get(p["status"],0)+1
                    assert p["status"] in ["exact_zero","exact_representable","qualified"]
                    maxbound=max(maxbound,p["relative_bound"]);projections+=1
                nxt=tuple(sgn(s*r[j+1].q)<=0 if state[j] else sgn(s*u[j+1].q-F(gaps[j]))>=0 for j in range(2))
                if nxt==state:break
                state=nxt
            else:raise AssertionError("cap")
            assert state==expected;discrete+=1
controls={}
for label,n,d in [
    ("zero",0.,1.),("minimum_subnormal_exact",math.ulp(0.),1.),
    ("half_minimum_subnormal",math.ulp(0.),2.),
    ("three_halves_minimum_subnormal",3*math.ulp(0.),2.),
    ("normal_minimum",sys.float_info.min,1.),
    ("small_subnormal_nonexact",1e-310,1.1),
    ("maximum_finite_exact",sys.float_info.max,1.),
    ("overflow",sys.float_info.max,.5),
    ("one_third",1.,3.),
    ("tiny_reaction_tail",-2.**-60,1.),
]:
    p=proposed_projection(R(n,d));controls[label]=p
assert controls["minimum_subnormal_exact"]["status"]=="exact_representable"
assert controls["maximum_finite_exact"]["status"]=="exact_representable"
assert controls["half_minimum_subnormal"]["status"]=="unresolved"
assert controls["overflow"]["status"]=="unresolved"
assert controls["three_halves_minimum_subnormal"]["status"].startswith("unresolved")
# Candidate nonzero original Kfc, lost-force tail, exact minor and unsupported graph proofs.
u,r=response(12.5,[.1875,.3125],[False,False],1,.0625)
assert [x.q for x in u]==[F(1,16),F(3,16),F(5,16)]
u,r=response(12.5,[.125,.25],[True,True],1,0.,2.**-60)
assert r[2].q==-F(1,2**60)
out={"kind":"Source review probes and separate PROPOSED publication bridge; not Rust execution",
 "frozen_neighbor_expectations_verified":len(triples),"discrete_traces_both_senses_all_seeds":discrete,
 "full_source_state_rows_checked":rowschecked,"all_ratios_match_independent_gaussian_oracle":True,
 "expansion_arithmetic_operations_checked":ops,
 "proposed_bridge":{"projection_count":projections,"status_counts":counts,
                   "maximum_relative_error_upper_bound":maxbound,"edge_controls":controls},
 "original_Kfc_and_force_tail_controls":True,
 "limits":"No implementation/pass claim for proposed projection; no native/backend execution or whole-product qualification."}
(OUT/"independent_review.json").write_text(json.dumps(out,indent=2)+"\n")
print(json.dumps(out,indent=2))

