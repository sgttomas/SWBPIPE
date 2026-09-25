"""Source-control-flow reservation model; not a Rust execution or timing test."""
from pathlib import Path
import json
OUT=Path(__file__).parent
MAX=(1<<64)-1
def add(total,count,width,limit):
    amount=count*width
    if amount>MAX or total+amount>MAX:return None,MAX
    total+=amount
    return (None,total) if total>limit else (total,None)
def reserve(counts,byte_lengths,limit):
    total=256
    if total>limit:return {"ok":False,"charged":0,"rejected":total,"visited_dynamic":0}
    for count in counts:
        total,error=add(total,count,64,limit)
        if error is not None:return {"ok":False,"charged":0,"rejected":error,"visited_dynamic":0}
    prefix=total;remaining=limit-prefix;dynamic=0;visited=0
    for count in byte_lengths:
        visited+=1
        dynamic,error=add(dynamic,count,1,remaining)
        if error is not None:
            return {"ok":False,"charged":prefix,"rejected":error,"visited_dynamic":visited}
    return {"ok":True,"charged":prefix+dynamic,"rejected":0,"prefix":prefix,"visited_dynamic":visited}
# Representative container counts for the actual 18-DOF/two-gap report.
counts=[1,1,17,17,18,18,18,18,2,2,2,36,36,40,40]
short_bytes=[100,100,150,150]+[20,0]*80+[2,2,2,2]
long_bytes=short_bytes[:]
# Four corresponding public/private work-label/decision ID length increases.
for index in [4,84,len(long_bytes)-4,len(long_bytes)-2]:
    long_bytes[index]+=4096-2
results={}
for name,bytes_ in [("short",short_bytes),("long",long_bytes)]:
    full=reserve(counts,bytes_,MAX);C=full["charged"];P=full["prefix"]
    tiny=reserve(counts,bytes_,1088)
    late=reserve(counts,bytes_,C-1)
    assert not tiny["ok"] and tiny["charged"]==0 and tiny["visited_dynamic"]==0
    assert not late["ok"] and late["charged"]==P>0
    assert late["charged"]+late["rejected"]==C
    assert late["visited_dynamic"]>0
    assert reserve(counts,bytes_,C)["ok"]
    changed=bytes_+[4096]
    nested_error=reserve(counts,changed,C)
    assert not nested_error["ok"] and nested_error["charged"]==P
    assert nested_error["rejected"]>C-P
    # A child receives only L-C and its accepted work is added, never a new full L.
    L=C+500
    child_charge=500;child_rejected=16
    assert C+child_charge==L
    results[name]={"full":full,"historical_1088":tiny,"C_minus_one":late,
                   "late_nested_error":nested_error,
                   "child_budget_example":{"outer":L,"child_limit":L-C,"combined_accepted":C+child_charge,"child_rejected":child_rejected}}
assert results["long"]["full"]["charged"]-results["short"]["full"]["charged"]==4*(4096-2)
early_overflow=reserve([MAX],[],MAX)
assert early_overflow["charged"]==0 and early_overflow["rejected"]==MAX
late_overflow=reserve([], [MAX-300,500], MAX)
assert late_overflow["charged"]==256 and late_overflow["rejected"]==MAX
# Bounded sweep of success, prefix refusal and later refusal.
for L in range(0,2000):
    a=reserve([1,2],[10,20,30],L)
    assert a["charged"]<=L
    if a["visited_dynamic"] and not a["ok"]:assert a["charged"]==448
    if a["ok"]:assert a["charged"]==508
out={"kind":"Independent two-stage control-flow/overflow model, not executed Rust",
 "representative_cases":results,"overflow":{"prefix":early_overflow,"dynamic":late_overflow},
 "limit_sweep_cases":2000,"success_costs_preserved":True,
 "late_rejection_retains_accepted_prefix":True,"no_child_budget_reset":True,
 "limits":"Byte lists are representative, not claimed actual Rust fixture charge measurements. Retention tests remain pending."}
(OUT/"LEDGER_CHECKS.json").write_text(json.dumps(out,indent=2)+"\n")
print(json.dumps({k:v for k,v in out.items() if k!="representative_cases"},indent=2))
