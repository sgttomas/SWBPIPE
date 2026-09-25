"""Independent exact represented-number decimal-gap reference.
No production import, generator import, solver, Cargo, or live nonlinear source.
"""
from fractions import Fraction as F
from itertools import product
from pathlib import Path
import json,struct,math
OUT=Path(__file__).parent
BASIS=OUT.parents[1]
def bits(x):return int.from_bytes(struct.pack(">d",x),"big")
def decode(x):
    b=bits(x);s=-1 if b>>63 else 1;exp=(b>>52)&2047;mant=b&((1<<52)-1)
    assert exp not in [0,2047]
    return s*F((1<<52)+mant)*F(2)**(exp-1023-52)
g=decode(.05);k=F(100);f=F(5)
assert g==F(3602879701896397,2**56)
assert g-f/k==F(1,5*2**56)
assert k*g-f==F(5,2**54)
# One-step trial in the unchanged Gap law. s*r<=0 bears; s*u>=g closes.
def scalar_trial(magnitude,s,state,gap):
    load=F(s)*F(magnitude)
    u=F(s)*gap if state=="Active" else load/k
    reaction=k*u-load
    nxt=("Active" if s*reaction<=0 else "Inactive") if state=="Active" else ("Active" if s*u>=gap else "Inactive")
    return u,reaction,nxt
def scalar_trace(magnitude,s,seed,gap):
    state=seed;trace=[]
    for i in range(1,9):
        u,r,nxt=scalar_trial(magnitude,s,state,gap)
        trace.append({"iteration":i,"prior":state,"u":str(u),"reaction":str(r),"normalized_reaction":str(s*r),"penetration":str(s*u-gap),"next":nxt,"changed":int(nxt!=state)})
        if nxt==state:return trace
        state=nxt
    raise AssertionError("unexpected exact nonconvergence")
rows=[]
for magnitude,s,seed,mode in product([4,5,6],[-1,1],["Active","Inactive"],["DenseScrutiny","SparseInteractive"]):
    trace=scalar_trace(magnitude,s,seed,g)
    final=trace[-1]
    expected="Active" if magnitude==6 else "Inactive"
    assert final["next"]==expected
    pu=float(F(final["u"]));pr=float(F(final["reaction"]))
    old_u=s*(.04 if magnitude<5 else .05)
    old_r=-s if magnitude>5 else 0.
    assert abs(pu-old_u)<1e-14
    assert abs(pr-old_r)<1e-14
    rows.append({"magnitude":magnitude,"sense":s,"seed":seed,"mode_reference_label_only":mode,
                 "expected":expected,"trace":trace,
                 "projected_u":pu,"projected_r":pr,
                 "unchanged_old_quantity_checks_pass":True})
assert len(rows)==24
# Check manager-derived packet against independent exact outputs, not as oracle.
submitted=json.loads((BASIS/"_run_records/EXACT_REFERENCE.json").read_text())
assert F(submitted["gap"])==g and F(submitted["gap_minus_free_u"])==g-f/k
assert F(submitted["prescribed_pulling_reaction"])==k*g-f
for case in submitted["cases"]:
    s=case["sense"];mag=case["force"]
    final=scalar_trace(mag,s,"Inactive",g)[-1]
    assert case["exact_state"]==final["next"]
    assert F(case["selected_displacement"])==F(final["u"])
    assert F(case["reaction"])==F(final["reaction"])
# A separate true binary64 tie; retain the decimal-spelled case, don't replace it.
dyadic_scalar=[]
for s,seed,mode in product([-1,1],["Active","Inactive"],["DenseScrutiny","SparseInteractive"]):
    trace=scalar_trace(F(25,2),s,seed,F(1,8))
    assert trace[-1]["next"]=="Active"
    assert F(trace[-1]["reaction"])==0
    dyadic_scalar.append({"sense":s,"seed":seed,"mode_reference_label_only":mode,"trace":trace})
# Independent general 2x2 rational constrained solve for captured coupled dyadic control.
A=[[F(200),F(-100)],[F(-100),F(100)]]
def solve(a,b):
    n=len(b);x=[row[:]+[b[i]] for i,row in enumerate(a)]
    for j in range(n):
        pivot=next(i for i in range(j,n) if x[i][j])
        x[j],x[pivot]=x[pivot],x[j]
        d=x[j][j];x[j]=[v/d for v in x[j]]
        for i in range(n):
            if i!=j:
                d=x[i][j];x[i]=[v-d*w for v,w in zip(x[i],x[j])]
    return [row[-1] for row in x]
coupled=[]
for s,seed,mode in product([-1,1],list(product([False,True],repeat=2)),["DenseScrutiny","SparseInteractive"]):
    state=seed;history=[]
    for it in range(1,9):
        fixed={i:F(s)*[F(1,8),F(1,4)][i] for i in range(2) if state[i]}
        free=[i for i in range(2) if i not in fixed]
        load=[F(0),F(s)*F(25,2)]
        rhs=[load[i]-sum(A[i][j]*v for j,v in fixed.items()) for i in free]
        uu=[fixed.get(i,F(0)) for i in range(2)]
        for i,v in zip(free,solve([[A[i][j] for j in free] for i in free],rhs)):uu[i]=v
        rr=[sum(A[i][j]*uu[j] for j in range(2))-load[i] for i in range(2)]
        nxt=tuple(s*rr[i]<=0 if state[i] else s*uu[i]>=[F(1,8),F(1,4)][i] for i in range(2))
        history.append({"iteration":it,"prior":state,"u":list(map(str,uu)),"r":list(map(str,rr)),"next":nxt})
        if nxt==state:break
        state=nxt
    assert state==(True,True) and rr==[0,0]
    assert uu==[F(s,8),F(s,4)]
    coupled.append({"sense":s,"seed":seed,"mode_reference_label_only":mode,"trace":history})
# Exact source proof versus binary64 rounded trial observations.
floating={"free_quotient_equals_gap_bits":bits(5./100.)==bits(.05),
          "rounded_active_reaction":100.*.05-5.,
          "decimal_text_as_exact_rational_gap_would_differ":F("0.05")!=g}
assert all([floating["free_quotient_equals_gap_bits"],floating["rounded_active_reaction"]==0.,floating["decimal_text_as_exact_rational_gap_would_differ"]])
out={"basis":"Exact decoded binary64 source numbers and captured unchanged Gap law; mode labels are expected references only.",
 "gap_bits":hex(bits(.05)),"gap_hex":(.05).hex(),"gap_exact":str(g),
 "free_u_at_F5":str(f/k),"gap_minus_free_u":str(g-f/k),"pulling_reaction_if_active":str(k*g-f),
 "scalar_cases":rows,"scalar_case_count":len(rows),"F5_state_conflicts_with_captured_assertion":sum(r["magnitude"]==5 for r in rows),
 "max_scalar_reference_iterations":max(len(r["trace"]) for r in rows),
 "dyadic_scalar_cases":dyadic_scalar,"dyadic_coupled_cases":coupled,
 "dyadic_scalar_case_count":len(dyadic_scalar),"dyadic_coupled_case_count":len(coupled),
 "max_dyadic_reference_iterations":max(len(x["trace"]) for x in dyadic_scalar+coupled),
 "manager_packet_independently_confirmed":True,
 "floating_explanation_not_exact_oracle":floating,
 "limits":"No production test/source edit, no Rust execution, no live half-diff qualification. Original 1e-14 quantity assertions and contact equality convention unchanged."}
(OUT/"EXACT_REVIEW.json").write_text(json.dumps(out,indent=2)+"\n")
print(json.dumps({k:v for k,v in out.items() if k not in ["scalar_cases","dyadic_scalar_cases","dyadic_coupled_cases"]},indent=2))

