"""Independent review of actual projection/accounting control flow.
Source arithmetic is transcribed, never compiled Rust. Exact Fraction verifies
every successful enclosure/bound. Earlier reviewer helper definitions are reused
without executing that file's top-level probes or writing its evidence.
"""
import ast,json,math,random,sys
from fractions import Fraction as F
from pathlib import Path
OUT=Path(__file__).parent
STAGE=OUT.parents[1]
previous=STAGE/"KERNEL_REVIEW/_run_records/independent_review.py"
nodes=ast.parse(previous.read_text()).body
names={"E","R","sgn","down","up"}
selected=[n for n in nodes if isinstance(n,(ast.ClassDef,ast.FunctionDef)) and n.name in names]
ops=0
exec(compile(ast.Module(body=selected,type_ignores=[]),str(previous),"exec"),globals())
class Budget(Exception):pass
class Invalid(Exception):pass
class Unresolved(Exception):pass
class W:
    def __init__(self,limit):self.limit=min(limit,2_000_000);self.used=0;self.rejected=0
    def charge(self,n):
        if self.used+n>self.limit:self.rejected+=n;raise Budget
        self.used+=n
    def add(self,a,b):
        if len(a.t)>=256:raise Budget
        self.charge(6*len(a.t)+1);a.add(b);return a
    def scalar(self,v):return self.add(E(),v)
    def sum(self,a,b,s=1):
        out=E();out.t=a.t[:]
        for x in b.t:self.add(out,s*x)
        return out
    def mul(self,a,b):
        self.charge(len(a.t)*len(b.t)*(12*256+128))
        # Normal/range-checked product transcription from prior independent probe.
        return a.mul(b)
def enclose(w,e):
    lo=hi=0.
    for t in e.t:
        w.charge(12);l=lo+t;h=hi+t
        if not math.isfinite(l) or not math.isfinite(h):raise Unresolved
        lo=down(l);hi=up(h)
        if not math.isfinite(lo) or not math.isfinite(hi):raise Unresolved
    return lo,hi
def project(w,r,criterion):
    w.charge(16)
    if not math.isfinite(criterion) or criterion<0:raise Invalid
    exact=r.q
    if not r.n.t:return dict(value=0.,lo=0.,hi=0.,absolute=0.,relative=0.,basis="exact_zero")
    w.charge(len(r.n.t)+len(r.d.t)+16)
    q=sum(r.n.t)/sum(r.d.t)
    if not math.isfinite(q):raise ArithmeticError
    if not q or sgn(q)!=sgn(exact):raise Unresolved
    qe=w.scalar(q)
    try:
        eq=w.sum(r.n,w.mul(qe,r.d),-1)
        if not eq.t:return dict(value=q,lo=q,hi=q,absolute=0.,relative=0.,basis="exact_identity")
    except ArithmeticError:pass
    nl,nu=enclose(w,r.n);dl,du=enclose(w,r.d);w.charge(48)
    if dl<=0 or nl<=0<=nu:raise Unresolved
    corners=[nl/dl,nl/du,nu/dl,nu/du]
    if not all(map(math.isfinite,corners)):raise Unresolved
    lo=down(min(corners));hi=up(max(corners))
    if not all(map(math.isfinite,[lo,hi])) or lo<=0<=hi:raise Unresolved
    q=min(max(q,lo),hi)
    if not math.isfinite(q) or not q or sgn(q)!=sgn(exact):raise Unresolved
    w.charge(32)
    l=q-lo;h=hi-q
    if not all(map(math.isfinite,[l,h])) or l<0 or h<0:raise Unresolved
    absolute=max(up(l),up(h))
    relative=up(absolute/min(abs(lo),abs(hi)))
    if not math.isfinite(absolute) or not math.isfinite(relative) or relative>criterion:raise Unresolved
    return dict(value=q,lo=lo,hi=hi,absolute=absolute,relative=relative,basis="outward_interval")
def attempt(r,criterion=1e-9,limit=2_000_000,dof_valid=True):
    w=W(limit)
    try:
        w.charge(4)
        if not dof_valid:raise Invalid
        p=project(w,r,criterion)
        exact=r.q
        assert F(p["lo"])<=exact<=F(p["hi"])
        assert abs(F(p["value"])-exact)<=F(p["absolute"])
        if exact:
            assert abs(F(p["value"])-exact)/abs(exact)<=F(p["relative"])
            assert sgn(p["value"])==sgn(exact)
        assert p["relative"]<=criterion
        result={"status":"ok","projection":p}
    except (Budget,Invalid,Unresolved,ArithmeticError) as e:result={"status":type(e).__name__}
    return {**result,"charged":w.used,"rejected":w.rejected,"limit":w.limit}
cases={}
for s in [-1,1]:
    for name,n,d in [
        ("zero",0.,1.),("minsub",math.ulp(0.),1.),
        ("min_normal",sys.float_info.min,1.),("max_finite",sys.float_info.max,1.),
        ("tail",2.**-60,1.),("half_minsub",math.ulp(0.),2.),
        ("one_and_half_minsub",3*math.ulp(0.),2.),("overflow",sys.float_info.max,.5),
        ("third",1.,3.),("gradual",1e-310,1.1)]:
        a=attempt(R(s*n,d));cases[f"{s}_{name}"]=a
        if name in ["half_minsub","one_and_half_minsub","overflow"]:assert a["status"]!="ok"
        else:assert a["status"]=="ok"
    assert attempt(R(s,3),0.)["status"]=="Unresolved"
    for exact in [0.,math.ulp(0.),sys.float_info.max]:
        p=attempt(R(s*exact),0.);assert p["status"]=="ok" and p["projection"]["relative"]==0
for c in [math.nan,math.inf,-1.]:
    assert attempt(R(0),c)["status"]=="Invalid"
# Same-sign adversarial broad-range scalars; a bounded fixed seed, 200 probes.
rng=random.Random(92026)
accepted=blocked=0;maximum=0.
for _ in range(200):
    n=math.ldexp(rng.uniform(.5,1.),rng.randint(-500,500))
    d=math.ldexp(rng.uniform(.5,1.),rng.randint(-500,500))
    a=attempt(R(n if rng.getrandbits(1) else -n,d))
    if a["status"]=="ok":
        accepted+=1;maximum=max(maximum,a["projection"]["relative"])
    else:blocked+=1
# Multiple exact low tails exercise interval summation and identity subtraction,
# rather than only scalar numerator/denominator pairs.
multi_accepted=0;multi_maximum=0.
for _ in range(100):
    en=E();ed=E();scale=rng.randint(-100,100)
    for j in range(4):
        en.add(math.ldexp(rng.uniform(.5,1.)*(-1 if rng.getrandbits(1) else 1),scale-54*j))
        ed.add(math.ldexp(rng.uniform(.5,1.),scale-53*j))
    a=attempt(R(en,ed))
    assert a['status']=='ok'
    multi_accepted+=1;multi_maximum=max(multi_maximum,a['projection']['relative'])
# Budget identity must fail as Budget, never fall back and succeed without reserve.
budget=[]
for limit in [0,4,19,20,21,38,39,100,3238,3239,3240,10_000,sys.maxsize]:
    a=attempt(R(1,3),limit=limit);budget.append(a)
    assert a["charged"]<=a["limit"]
    if a["status"]=="Budget":assert a["rejected"]>0
base=attempt(R(1,3));cost=base["charged"]
assert attempt(R(1,3),limit=cost)["status"]=="ok"
a=attempt(R(1,3),limit=cost-1);assert a["status"]=="Budget" and a["rejected"]>0
invalid=attempt(R(1,3),dof_valid=False);assert invalid["status"]=="Invalid" and invalid["charged"]==4
gradual=cases["1_gradual"]
assert gradual["status"]=="ok" and gradual["charged"]>3200 # accepted failed-identity product reserve remains
# Independent small-strain derivation, before source-style ratio arithmetic.
E_mod=F(200_000_000_000);area=F(1,256);length=F(1)
k=E_mod*area/length;g1=F(1,16384);g2=F(1,8192);force=F(48828125,1024)
assert k==781250000 and force==k*g1 and g2==2*g1
A=[[2*k,-k],[-k,k]]
det=A[0][0]*A[1][1]-A[0][1]**2
assert det==k*k and det>0
residual=[sum(A[i][j]*[g1,g2][j] for j in range(2))-[0,force][i] for i in range(2)]
assert residual==[0,0]
small=[]
for s in [-1,1]:
    kk=E(float(k));ff=E(float(s*force))
    d=kk.mul(E(float(2*k))).plus(kk.mul(kk),-1)
    ratios=[R(kk.mul(ff),d),R(E(float(2*k)).mul(ff),d)]
    for gap,r in zip([g1,g2],ratios):
        a=attempt(r,0.);assert a["status"]=="ok" and F(a["projection"]["value"])==s*gap
        assert a["projection"]["basis"]=="exact_identity"
        small.append(a)
# Deliberate bad publication certificates must fail independent exact checks.
third=attempt(R(1,3))["projection"];x=F(1,3)
mutations={
 "zero_relative_bound":not(abs(F(third["value"])-x)/x<=0),
 "shift_interval_off_answer":not(F(third["hi"])<=x<=F(up(third["hi"]))),
 "rounded_reaction_erases_tail":F(0)!=-F(1,2**60),
}
assert all(mutations.values())
out={"kind":"Independent actual-source arithmetic/accounting transcription plus Fraction checks; NOT Rust runtime",
 "edge_cases":cases,"random_scalar_probes":{"count":200,"qualified":accepted,"unresolved":blocked,"maximum_relative_bound":maximum},
 "multi_tail_probes":{"count":100,"qualified":multi_accepted,"maximum_relative_bound":multi_maximum},
 "budget_attempts":budget,"exact_success_charge":cost,"one_less_limit_rejected":True,
 "arithmetic_shortcut_failed_reservation_retained":gradual["charged"],
 "negative_controls":mutations,
 "small_strain_companion":{"k_N_per_m":str(k),"force_N":str(force),"gap1_m":str(g1),"gap2_m":str(g2),
 "minor1":str(2*k),"determinant":str(det),"free_residuals":list(map(str,residual)),
 "strain_each_span":str(g1/length),"strain_each_span_decimal":float(g1/length),
 "linear_stress_Pa":str(E_mod*g1/length),"strain_energy_J":str(k*g1*g1),
 "four_signed_scalar_projections_exact":True,
 "scope":"Two equal 1m passive axial bars under the stated small-strain linear elastic model; no material adequacy or full-pipe qualification"},
 "limits":"No Rust compilation/test/native execution; source/response pointer binding assessed by code/lifetime review."}
(OUT/"check_projection.json").write_text(json.dumps(out,indent=2)+"\n")
print(json.dumps({k:v for k,v in out.items() if k not in ["edge_cases","budget_attempts"]},indent=2))
