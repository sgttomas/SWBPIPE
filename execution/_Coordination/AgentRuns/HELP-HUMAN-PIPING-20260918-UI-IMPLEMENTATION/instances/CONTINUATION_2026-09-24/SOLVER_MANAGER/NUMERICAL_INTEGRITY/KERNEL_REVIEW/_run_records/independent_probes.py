"""Small independent review probes; no production code imported or executed.

Exact Fraction substitution/rank/inverse checks are independent oracles.
The explicitly named *_transcription functions mirror selected source arithmetic
to expose counterexamples; their results are NOT Rust execution observations.
Run with Python stdlib only from any directory.
"""
import json
import math
import sys
from fractions import Fraction as F
from pathlib import Path

OUT = Path(__file__).parent
ROOT = next(p for p in OUT.parents if (p / "projects/chirality-piping/core/solver").is_dir())
FIXTURES = ROOT / "projects/chirality-piping/validation/benchmarks/numerical_integrity/fixtures.json"
fixtures = json.loads(FIXTURES.read_text())
EPS = 2.0 ** -52
MIN = sys.float_info.min

def rank(a):
    a = [[F(x) for x in row] for row in a]
    m, n = len(a), len(a[0]) if a else 0
    r = 0
    for j in range(n):
        pivot = next((i for i in range(r, m) if a[i][j]), None)
        if pivot is None:
            continue
        a[r], a[pivot] = a[pivot], a[r]
        d = a[r][j]
        a[r] = [x / d for x in a[r]]
        for i in range(m):
            if i != r:
                d = a[i][j]
                a[i] = [x - d*y for x, y in zip(a[i], a[r])]
        r += 1
    return r

def solve_exact(a, f):
    n = len(a)
    a = [[F(x) for x in row] + [F(f[i])] for i, row in enumerate(a)]
    for j in range(n):
        p = next(i for i in range(j, n) if a[i][j])
        a[j], a[p] = a[p], a[j]
        d = a[j][j]
        a[j] = [x/d for x in a[j]]
        for i in range(n):
            if i != j:
                d = a[i][j]
                a[i] = [x-d*y for x,y in zip(a[i],a[j])]
    return [row[-1] for row in a]

def motion_rows(p):
    x,y,z = p
    return [[1,0,0,0,z,-y], [0,1,0,-z,0,x], [0,0,1,y,-x,0],
            [0,0,0,1,0,0], [0,0,0,0,1,0], [0,0,0,0,0,1]]

def geometry_transcription(coords, ground):
    # Source rigid_body.rs 44-86 and 88-201. Fraction evaluates the final
    # exact-product annihilation condition, independently of submitted EFT.
    origin = coords[0]
    relative = [[p[j]-origin[j] for j in range(3)] for p in coords]
    length = max(math.hypot(math.hypot(*p[:2]),p[2]) for p in relative) or 1.
    relative = [[x/length for x in p] for p in relative]
    actual = [motion_rows(relative[d//6])[d%6] for d in ground]
    b = [[x/math.sqrt(sum(t*t for t in row)) for x in row] for row in actual]
    v = [[float(i==j) for j in range(6)] for i in range(6)]
    converged = False
    for sweep in range(64):
        changed = False
        for p in range(6):
            for q in range(p+1,6):
                alpha = sum(r[p]*r[p] for r in b)
                beta = sum(r[q]*r[q] for r in b)
                cross = sum(r[p]*r[q] for r in b)
                if abs(cross) <= 8*EPS*math.sqrt(alpha)*math.sqrt(beta):
                    continue
                zeta = (beta-alpha)/(2*cross)
                t = (math.copysign(1.,zeta)/(abs(zeta)+math.hypot(zeta,1.))
                     if zeta != 0 else 1.)
                c = 1/math.sqrt(1+t*t); s=c*t
                for row in b+v:
                    a,d=row[p],row[q]
                    row[p],row[q]=c*a-s*d,s*a+c*d
                changed = True
        if not changed:
            converged = True
            break
    singular = [math.sqrt(sum(r[j]*r[j] for r in b)) for j in range(6)]
    screen = 64*gamma(max(len(ground),6))*max(singular)
    # Rust min_by returns first minimal element for ordinary equal numbers.
    index = min(range(6),key=lambda j: singular[j])
    if converged and singular[index]>screen:
        return {"status":"Restrained","singular_values":singular,"screen":screen}
    candidate = [row[index] for row in v]
    def exact_zero(c):
        return all(sum(F(x)*F(y) for x,y in zip(row,c))==0 for row in actual)
    action = max((abs(sum(x*y for x,y in zip(row,candidate))) for row in actual), default=0.)
    if action != 0:
        candidates = [[float(i==j) for j in range(6)] for i in range(6)]
        candidates += [[0.,0.,0.]+r for r in relative if any(r)]
        for c in candidates:
            if all(sum(x*y for x,y in zip(row,c))==0 for row in actual) and exact_zero(c):
                candidate=c
                break
    result = {"status":"MechanismWitnessed" if exact_zero(candidate) else "NumericallyUnresolved",
              "normalized_relative":relative,"candidate":candidate,"singular_values":singular,
              "screen":screen, "iterations":sweep+1}
    # Independent physical constraints: exact subtraction of represented input coordinates.
    exact_relative = [[F(p[j])-F(origin[j]) for j in range(3)] for p in coords]
    exact_rows = [motion_rows(exact_relative[d//6])[d%6] for d in ground]
    physical_candidate = [F(x)*F(length) for x in candidate[:3]]+[F(x) for x in candidate[3:]]
    actions = [sum(F(x)*y for x,y in zip(row,physical_candidate)) for row in exact_rows]
    result.update(actual_exact_rank=rank(exact_rows), normalized_exact_rank=rank(actual),
                  actual_action_nonzero=any(actions),
                  maximum_actual_action=float(max(map(abs,actions),default=F(0))))
    return result

def gamma(m):
    x = m*(EPS/2)
    return x/(1-x)

def residual_transcription(k, f, u, free):
    results = []
    for i in free:
        exponents = [math.frexp(abs(f[i]))[1]-1] if f[i] else []
        exponents += [math.frexp(abs(a))[1]+math.frexp(abs(b))[1]-1
                      for a,b in zip(k[i],u) if a and b]
        e=max(exponents,default=0)
        r=-math.ldexp(f[i],-e); d=abs(r); count=0
        for a,b in zip(k[i],u):
            if not a:
                continue
            count+=1
            if b:
                ma,ea=math.frexp(a);mb,eb=math.frexp(b)
                p=math.ldexp((ma*2)*(mb*2),ea+eb-2-e)
            else: p=0.
            r+=p;d+=abs(p)
        g=gamma(2*count+2)
        guarded=(abs(r)/d+g/(1-g))*(1+g)*(1+4*EPS) if d else 0.
        results.append({"row":i,"ratio":guarded,"target":64*g,"passed":guarded<=64*g,
                        "normal_residual":r,"physical_residual":math.ldexp(r,e),
                        "operations":2*count+2})
    return results

def hager_transcription(a, solve):
    n=len(a)
    norm=max(sum(abs(row[j]) for row in a) for j in range(n))
    x=[1./n]*n; est=0.;previous=n
    for _ in range(5):
        y=solve(x); current=sum(map(abs,y))
        if current<=est and est!=0:break
        est=current
        z=solve([1. if t>=0 else -1. for t in y])
        j=max(range(n),key=lambda j:(abs(z[j]),j))
        dot=sum(p*q for p,q in zip(z,x))
        if abs(z[j])<=dot or j==previous:break
        previous=j;x=[float(i==j) for i in range(n)]
    alt=[(1. if i%2==0 else -1.)*(1+i/(n-1)) if n>1 else 1. for i in range(n)]
    est=max(est,sum(map(abs,solve(alt)))*2/(3*n))
    return min(1.,1/(norm*est))

def two_sum(a,b):
    s=a+b;z=s-a
    return s,(a-(s-z))+(b-z)

results={}
ground=[0,1,2,6,7,8,12,13,14]
geometries={
 "normalization_underflow":[[0.,0.,0.],[1e200,0.,0.],[0.,1e-200,0.]],
 "centering_rounds_noncollinearity":[[1e16,1e16,0.],[-1e16,-1e16,0.],[-1e16,-9999999999999998.,0.]],
 "near_collinear_retained":[[0.,0.,0.],[1.,0.,0.],[1.,1e-16,0.]],
 "noncollinear_positive":[[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],
}
results["geometry"]={name:geometry_transcription(coords,ground) for name,coords in geometries.items()}
for name in ["normalization_underflow","centering_rounds_noncollinearity"]:
    r=results["geometry"][name]
    assert r["status"]=="MechanismWitnessed" and r["actual_exact_rank"]==6
    assert r["normalized_exact_rank"]==5 and r["actual_action_nonzero"]
assert results["geometry"]["near_collinear_retained"]["status"]=="NumericallyUnresolved"
assert results["geometry"]["noncollinear_positive"]["status"]=="Restrained"
results["geometry"]["N02"] = geometry_transcription([[0.,0.,0.],[1.2,1.6,0.]],ground[:6])
results["geometry"]["N03_RX"] = geometry_transcription([[0.,0.,0.],[1.2,1.6,0.]],ground[:6]+[3])
results["geometry"]["N03_RZ"] = geometry_transcription([[0.,0.,0.],[1.2,1.6,0.]],ground[:6]+[5])
assert results["geometry"]["N02"]["status"]=="MechanismWitnessed"
assert results["geometry"]["N03_RX"]["status"]=="Restrained"
assert results["geometry"]["N03_RZ"]["status"]=="MechanismWitnessed"

results["residual_controls"]={}
for name in ["R01","R02","R03","R04","R05"]:
    f=fixtures["R"][name]
    rows=residual_transcription([[float(x) for x in row] for row in f["K_original"]],
                               list(map(float,f["f_original"])),list(map(float,f["candidate"])),f["free"])
    assert any(not r["passed"] for r in rows)
    results["residual_controls"][name]=rows
f=fixtures["R"]["R04"]
valid=residual_transcription([[float(x) for x in row] for row in f["K_original"]],
                            list(map(float,f["f_original"])),list(map(float,f["valid_u"])),f["free"])
assert all(r["passed"] for r in valid)
results["residual_controls"]["R04_valid"]=valid
results["residual_controls"]["soft_cancellation"]=residual_transcription([[2e-300]],[4e-300],[1.9999999999999996],[0])
assert results["residual_controls"]["soft_cancellation"][0]["passed"]
results["residual_controls"]["all_fixed"]=residual_transcription([[1.]],[7.],[0.],[])

n07=fixtures["N"]["N07"];v=list(map(F,n07["negative_vector"]));k=n07["K"]
energy=sum(v[i]*F(k[i][j])*v[j] for i in range(2) for j in range(2))
assert energy==-2
results["invalid_energy"]={"exact_energy":str(energy),"zero_load_null_rank":rank([[1,-1],[-1,1]])}
def n07_lu(rhs):
    return [(-rhs[0]+2*rhs[1])/3, (2*rhs[0]-rhs[1])/3]
completion_rcond=hager_transcription([[1.,2.],[2.,1.]],n07_lu)
completion_residual=residual_transcription([[1.,2.],[2.,1.]],[1.,1.],n07_lu([1.,1.]),[0,1])
assert completion_rcond>EPS and all(r['passed'] for r in completion_residual)
results['public_completion_bypass']={'empty_pivots':[], 'rcond':completion_rcond,
 'residual_rows':completion_residual,'negative_energy':-2,
 'source_result':'finish_structural returns Passed; top-level wrappers correctly reject'}

# Counterexample to free/free-only assembly fidelity.
stored=[[2,-1],[-1,2]];intended=[[2,-2],[-2,2]]
u=solve_exact([[stored[1][1]]],[-stored[1][0]])
physical=solve_exact([[intended[1][1]]],[-intended[1][0]])
assert u==[F(1,2)] and physical==[1]
results["prescribed_contribution_loss"]={
 "stored_u":[1.,float(u[0])],"intended_u":[1.,float(physical[0])],
 "reported_free_free_perturbation":0., "reported_amplification":0.,
 "original_stored_residual":residual_transcription(stored,[0.,0.],[1.,float(u[0])],[1]),
 "intended_free_residual":str(F(intended[1][0])+F(intended[1][1])*u[0])}

# Low-word accumulation can itself lose a term; complete directed off-diagonal contributions.
terms=[1e16,1.,1e-16,-1.,-1e16];hi=lo=0.
for t in terms:
    hi,e=two_sum(hi,t);lo+=e
exact=sum(map(F,terms))
assert hi==lo==0. and exact!=0
results["low_word_loss"]={"terms":terms,"accumulated_high":hi,"accumulated_low":lo,
                          "exact_total":str(exact),"exact_total_float":float(exact)}

# Exact 8x8 NP-B oracle. No large-matrix solve; O(n) comparison recurrence through order 80.
npb=next(x for x in fixtures["NP"]["B"] if x["n"]==8 and x["coefficient"]=="0.75")
a=[[F(0) for _ in range(8)] for _ in range(8)]
for i,j,x in npb["K_entries"]:a[i][j]=F(x)
f=list(map(F,npb["f_exact"]))
assert solve_exact(a,f)==[F(1)]*8
npb_orderings={}
for name,p in npb["orderings"].items():
    ap=[[a[i][j] for j in p] for i in p]
    assert solve_exact(ap,[f[i] for i in p])==[F(1)]*8
    npb_orderings[name]=True
inv=[solve_exact(a,[int(i==j) for i in range(8)]) for j in range(8)]
norm=max(sum(abs(a[i][j]) for i in range(8)) for j in range(8))
invnorm=max(sum(map(abs,col)) for col in inv)
def exact_inverse_action(x):
    return [float(sum(inv[j][i]*F(x[j]) for j in range(8))) for i in range(8)]
est=hager_transcription([[float(x) for x in row] for row in a],exact_inverse_action)
z=[]
for i in range(80):z.append(F(1)+F(3,4)*sum(z[max(0,i-2):i]))
results["NP_B"]={"n8_orderings":npb_orderings,"exact_rcond1":float(1/(norm*invnorm)),
 "hager_rcond_transcription":est,"n80_comparison_squared_u":float(z[-1]**2/F(2**53)),
 "analytical_uniform_condition2_bound":"<429 for dyadic family, independent of order"}
assert est>EPS and results["NP_B"]["n80_comparison_squared_u"]>1
results["limitations"]=["Source transcriptions are not Rust execution.",
 "Parent frame 51/51 and sparse 24/24 logs are separate observed runs.",
 "No full N01-N09 product, sparse/fallback mutation, native, DEC050/053, or closure claim.",
 "N05/N06 still require contribution-preserving physical-accuracy repair."]
(OUT/"independent_probes.json").write_text(json.dumps(results,indent=2)+"\n")
print(json.dumps(results,indent=2))
