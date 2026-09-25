"""Independent exact retention references. No production code or Rust execution."""
from fractions import Fraction as F
from pathlib import Path
import json,copy
OUT=Path(__file__).parent
gap=F(float.fromhex("0x1.fffffffffffffp-4"))
K=[[F(100),F(-100),F(0)],[F(-100),F(200),F(-100)],[F(0),F(-100),F(100)]]
force=[F(0),F(0),F(25,2)]
prescribed={0:F(0),1:gap}
free=[2]
h=force[2]-sum(K[2][j]*v for j,v in prescribed.items())
u=[F(0),gap,h/K[2][2]]
r=[sum(K[i][j]*u[j] for j in range(3))-force[i] for i in range(3)]
assert h==100*gap+F(25,2)
assert u[2]==F(1,4)-F(1,2**56)
assert r[1]==-F(25,2**54) and r[2]==0
published=[F(float(x)) for x in u]
wrong=sum(K[1][j]*published[j] for j in range(3))-force[1]
assert wrong==-F(25,2**53)
assert float(u[2])==.25
assert u[2]-F(1,4)<0 and r[1]<0
a,b,c=F(200),F(-100),F(100)
det=a*c-b*b
assert a>0 and det==10000
# The source-changing-but-internally-consistent control is distinguishable.
changed_force=[0,0,F(13)]
changed_u=[F(0),F(13,100),F(13,50)]
assert [sum(K[i][j]*changed_u[j] for j in range(3))-F(changed_force[i]) for i in [1,2]]==[0,0]
assert changed_force!=force and changed_u!=[0,F(1,8),F(1,4)]
tail=F(1,2**60)
assert float(F(1)+tail)==1.
assert -(F(1)+tail)!=-F(1)
# Shared-cap invariant, including adapter prefix and failed reservations.
ledger=[]
for total,prefix,costs in [(100,20,[30,40,10]),(99,20,[30,40,10]),(20,21,[])]:
    accepted=rejected=0
    if prefix>total:rejected=prefix
    else:
        accepted=prefix
        child_limit=total-prefix
        child_used=0
        for n in costs:
            if child_used+n>child_limit:
                rejected=n
                break
            child_used+=n
        accepted+=child_used
    assert accepted<=total
    ledger.append({"limit":total,"accepted":accepted,"rejected_unexecuted":rejected})
assert ledger[0]["accepted"]==100 and ledger[1]["rejected_unexecuted"]>0
out={"kind":"Independent Fraction/source-retention reference checks, not executed Rust retention/replay",
 "adjacent_source":{"K":[list(map(str,row)) for row in K],"force":list(map(str,force)),
  "free":free,"prescribed":{str(k):str(v) for k,v in prescribed.items()}},
 "singleton_witness":{"a":str(K[2][2]),"positive":True},
 "two_dof_witness":{"a":str(a),"b":str(b),"c":str(c),"determinant":str(det)},
 "exact_u":list(map(str,u)),"exact_reactions":list(map(str,r)),
 "public_u":list(map(float,u)),"reaction_from_exact_ratios":str(r[1]),
 "exact_action_on_public_u":str(wrong),"factor_two_difference_verified":True,
 "independent_original_source_vs_other_consistent_source":True,
 "identified_force_tail_preserved_required":str(tail),"tail_reaction":str(-1-tail),
 "one_cap_accounting_model":ledger,
 "limits":"Actual Rust constructor/privacy/comparison/order/shape paths are reviewed in source; tests remain pending. Wire/import/persistence/currentness are outside this delta."}
(OUT/"REFERENCE_CHECKS.json").write_text(json.dumps(out,indent=2)+"\n")
print(json.dumps(out,indent=2))

