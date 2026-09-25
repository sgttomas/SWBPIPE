"""Independent small repair probes; stdlib only, not Rust execution.
Expansion/profile transcriptions are checked against exact Fraction oracles.
"""
from fractions import Fraction as F
import json, math, random, sys
from pathlib import Path
OUT=Path(__file__).parent
ROOT=next(p for p in OUT.parents if (p/"projects/chirality-piping/core/solver").is_dir())
rng=random.Random(5192401)
checks={}
def grow(terms,value):
    q=value;next=[]
    for e in terms:
        s=q+e;z=s-q;lo=(q-(s-z))+(e-z)
        if not all(map(math.isfinite,[s,z,lo])):raise ValueError("range")
        if lo:next.append(lo)
        q=s
    if q:next.append(q)
    return next
def radix(value,exponent):
    if not math.isfinite(value):raise ValueError("range")
    while exponent and value:
        step=max(-512,min(512,exponent))
        try:nxt=math.ldexp(value,step)
        except OverflowError:raise ValueError("range")
        if not math.isfinite(nxt) or not nxt or math.ldexp(nxt,-step)!=value:raise ValueError("range")
        value=nxt;exponent-=step
    return value
def add_product(terms,a,b,e=0):
    if not a or not b:return terms
    ea=math.frexp(abs(a))[1]-1;eb=math.frexp(abs(b))[1]-1
    ma=radix(a,-ea);mb=radix(b,-eb)
    hi=ma*mb
    # Exact rational FMA oracle instead of importing any production FMA/helper.
    lo=float(F(ma)*F(mb)-F(hi))
    terms=grow(terms,radix(lo,ea+eb+e))
    return grow(terms,radix(hi,ea+eb+e))
sequences=[[1e16,1.,1e-16,-1.,-1e16],
 [1.,2.**-53,-1.], [sys.float_info.min,math.ulp(0.),-sys.float_info.min],
 [1e200,1e-200,-1e200], [1e16,-9999999999999998.]]
for _ in range(160):
    sequence=[]
    for _ in range(10):
        x=math.ldexp(rng.uniform(-1,1),rng.randint(-1000,1000))
        sequence.append(x)
    # Deliberate large cancellation followed by small residual.
    sequence += [-sequence[0],-sequence[1]]
    sequences.append(sequence)
for seq in sequences:
    terms=[];exact=F(0)
    for value in seq:
        terms=grow(terms,value);exact+=F(value)
        assert sum(map(F,terms))==exact
checks["expansion_sequences"]={"count":len(sequences),"all_intermediate_sums_exact":True}
products=[(1e200,1e-200,0),(1.,math.ulp(0.),0),
          (math.ulp(0.),.5,0),(1e200,1e200,0),(1e-200,1e-200,0)]
for _ in range(160):
    products.append((math.ldexp(rng.uniform(-1,1),rng.randint(-500,500)),
                     math.ldexp(rng.uniform(-1,1),rng.randint(-500,500)),rng.randint(-10,10)))
ok=blocked=0
for a,b,e in products:
    exact=F(a)*F(b)*F(2)**e
    try:terms=add_product([],a,b,e)
    except ValueError:blocked+=1;continue
    assert sum(map(F,terms))==exact
    ok+=1
checks["expansion_products"]={"count":len(products),"exact_accepted":ok,"explicit_range_rejections":blocked}
# Original-coordinate witness arithmetic, across the author's radix-unit cases.
def actions(coords,candidate,length):
    out=[]
    for p in coords:
        delta=[grow(grow([],p[i]),-coords[0][i]) for i in range(3)]
        m=[add_product([],length,candidate[i]) for i in range(3)]
        for i,pa,pw,na,nw in [(0,2,4,1,5),(1,0,5,2,3),(2,1,3,0,4)]:
            for t in delta[pa]:m[i]=add_product(m[i],t,candidate[pw])
            for t in delta[na]:m[i]=add_product(m[i],-t,candidate[nw])
        out.append([sum(map(F,t)) for t in m])
    return out
bad=[
 ([[0.,0.,0.],[1e200,0.,0.],[0.,1e-200,0.]],[0.,0.,0.,1.,0.,0.]),
 ([[1e16,1e16,0.],[-1e16,-1e16,0.],[-1e16,-9999999999999998.,0.]],[0.,0.,0.,-1.,-1.,0.])]
for coords,candidate in bad:
    for units in [.5,1.,2.]:
        pts=[[x*units for x in p] for p in coords]
        assert any(any(row) for row in actions(pts,candidate,1.))
for units in [.5,1.,2.]:
    for shift in [0.,8.]:
        coords=[[shift,shift,0.],[shift+3*units,shift+4*units,0.]]
        assert not any(any(row) for row in actions(coords,[0.,0.,0.,3.,4.,0.],5*units))
checks["geometry_original_action"]={"false_modes_refuted":6,"valid_oblique_controls":6}
# Perturbations in constrained coupling must reach reduced RHS and intended action.
coupling=[]
for intended in [-1.,-1.125,-2.]:
    difference=F(intended)-F(-1.)
    delta_rhs=-difference*F(1.)
    ratio=abs(delta_rhs)/F(1.)
    stored_u=F(1,2)
    intended_action=F(intended)+2*stored_u
    coupling.append({"coupling":intended,"relative_rhs_change":float(ratio),
                     "intended_free_residual":float(intended_action),
                     "residual_is_zero":intended_action==0})
assert coupling[0]["residual_is_zero"] and not coupling[1]["residual_is_zero"]
checks["prescribed_coupling"]=coupling
# Changed skyline algorithm: exact reference entries, nonconstant manufactured u,
# three permutations, factor/solve transcription checked against independent f=A*u.
fixtures=json.loads((ROOT/"projects/chirality-piping/validation/benchmarks/numerical_integrity/fixtures.json").read_text())
row=next(x for x in fixtures["NP"]["B"] if x["n"]==8 and x["coefficient"]=="0.75")
n=8;a=[[F(0)]*n for _ in range(n)]
for i,j,v in row["K_entries"]:a[i][j]=F(v)
expected=[F(i+1 if i%2==0 else -i-1) for i in range(n)]
rhs=[sum(a[i][j]*expected[j] for j in range(n)) for i in range(n)]
profile_results={}
for name,order in row["orderings"].items():
    matrix=[[float(a[i][j]) for j in order] for i in order]
    first=[next((j for j in range(i) if matrix[i][j]),i) for i in range(n)]
    rows=[[matrix[i][j] for j in range(first[i],i+1)] for i in range(n)]
    def get(i,j):return 0. if j<first[i] else rows[i][j-first[i]]
    def put(i,j,v):rows[i][j-first[i]]=v
    work=[0.]*n;pivots=[]
    for i in range(n):
        for j in range(first[i],i):
            v=get(i,j)
            for k in range(max(first[i],first[j]),j):v-=work[k]*get(j,k)
            work[j]=v;put(i,j,v/get(j,j))
        pivot=get(i,i)
        for k in range(first[i],i):pivot-=work[k]*get(i,k)
        assert pivot>0
        put(i,i,pivot);pivots.append(pivot)
    x=[float(rhs[i]) for i in order]
    for i in range(n):
        for j in range(first[i],i):x[i]-=get(i,j)*x[j]
    for i in range(n):x[i]/=get(i,i)
    for i in reversed(range(n)):
        value=x[i]
        for j in range(first[i],i):x[j]-=get(i,j)*value
    out=[0.]*n
    for i,j in enumerate(order):out[j]=x[i]
    error=max(abs(x-float(y)) for x,y in zip(out,expected))
    assert error<1e-12
    profile_results[name]={"max_abs_error":error,"positive_pivots":all(p>0 for p in pivots)}
checks["moved_profile_factor"]=profile_results
checks["limitations"]=["Python transcriptions, not Rust execution.",
 "Only bounded module repair consequences, not whole numerical qualification.",
 "No protected tolerance modified; candidate CI/native/full N/R/NP integration remains separate."]
(OUT/"independent_backcheck.json").write_text(json.dumps(checks,indent=2)+"\n")
print(json.dumps(checks,indent=2))

