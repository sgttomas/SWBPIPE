"""Independent static/exact controls for frozen nonlinear join. No production execution."""
from pathlib import Path
from fractions import Fraction as F
import json,re,struct
OUT=Path(__file__).parent
STAGE=OUT.parents[1]
NUM=STAGE.parent
source=(OUT/"NGR01_FINAL_lib.rs").read_text()
segment=source.split("fn strict_gap_27_independent_neighbors_all_senses_seeds_and_backends()",1)[1].split("for (fb, g1b, g2b, expected, expected_u, expected_r) in cases",1)[0]
pattern=r'\(\s*(\d{19}),\s*(\d{19}),\s*(\d{19}),\s*\[(true|false),\s*(true|false)\],\s*\[([^\]]+)\],\s*\[([^\]]+)\],\s*\)'
submitted=re.findall(pattern,segment)
assert len(submitted)==27
reference=json.loads((NUM/"STRICT_GAP_DESIGN/_run_records/REFERENCE_PROOF.json").read_text())
def bits(x):return int.from_bytes(struct.pack(">d",x),"big")
lookup={}
for row in reference["cases"]:
    key=(bits(float(F(row["F"]))),*(bits(float(F(g))) for g in row["g"]))
    lookup[key]=row
checks=[]
for fb,g1,g2,b1,b2,uu,rr in submitted:
    key=tuple(map(int,[fb,g1,g2]));row=lookup[key]
    states=[b1=="true",b2=="true"]
    u=list(map(float,uu.split(",")));r=list(map(float,rr.split(",")))
    assert states==list(map(bool,row["expected"]))
    assert u==[float(F(x)) for x in row["u"]]
    assert r==[float(F(x)) for x in row["r"]]
    checks.append({"input_bits":key,"expected_states_match":True,"expected_displacement_bits_match":True,
                   "expected_reaction_bits_match":True,"nonzero_reaction_count":sum(x!=0 for x in r)})
# Nonconstant projected-u and exact-ratio reactions differ. Retain the actual exact basis.
g1=F(float.fromhex("0x1.fffffffffffffp-4")) # predecessor of 1/8
exact_u=[g1,g1+F(1,8)]
exact_reaction=200*exact_u[0]-100*exact_u[1]
published_u=list(map(float,exact_u))
action_of_published_u=200*F(published_u[0])-100*F(published_u[1])
binary64_action=200*published_u[0]-100*published_u[1]
assert exact_reaction==-F(25,2**54)
assert action_of_published_u==-F(25,2**53)
assert action_of_published_u!=exact_reaction
# Spring and imposed-root examples use original coefficient/action equations.
spring=[[F(300),F(-100)],[F(-100),F(100)]]
su=[F(1,16),F(3,16)]
assert [sum(spring[i][j]*su[j] for j in range(2)) for i in range(2)]==[0,F(25,2)]
K=[[100,-100,0],[-100,200,-100],[0,-100,100]]
up=[F(1,16),F(3,16),F(5,16)];force=[F(0),F(0),F(25,2)]
reaction=[sum(F(K[i][j])*up[j] for j in range(3))-force[i] for i in range(3)]
assert reaction==[-F(25,2),0,0]
# Per-call and undertaking reservation invariant for successful/unknown failed setup.
budget=[]
remaining=8_000_000
for outcome,charge in [("success",450_000),("known_failed_attempt",1_900_000),("unknown_setup_failure",None),
                       ("success",1_999_999),("unknown_setup_failure",None),("exhausted",None)]:
    cap=min(2_000_000,remaining)
    if not cap:
        debit=0
    else:
        debit=cap if charge is None else charge
        assert debit<=cap
    old=remaining;remaining-=debit
    assert 0<=remaining<=old
    budget.append({"outcome":outcome,"before":old,"iteration_cap":cap,"debit":debit,"after":remaining})
out={"kind":"Independent source-table/rational/accounting checks; not Rust/backend execution",
     "neighbor_rows_checked":checks,"case_count":27,"required_mode_sense_seed_solves":27*2*4*2,
     "expected_with_same_mode_repeats":27*2*4*2*2,
     "spring_source_control":True,"prescribed_root_original_Kfc_control":True,
     "recovery_method_distinction":{"gap1":str(g1),"exact_u":list(map(str,exact_u)),
       "published_u":published_u,"exact_ratio_reaction":str(exact_reaction),
       "exact_action_of_published_u":str(action_of_published_u),"binary64_action_of_published_u":binary64_action,
       "same_public_displacement_does_not_determine_correct_exact_reaction":True},
     "work_debits":budget,
     "limits":"No nonlinear compile/solve; no Current, full physics, general extended method or method-receipt qualification."}
(OUT/"CONTROL_CHECKS.json").write_text(json.dumps(out,indent=2)+"\n")
print(json.dumps({k:v for k,v in out.items() if k not in ["neighbor_rows_checked","work_debits"]},indent=2))

